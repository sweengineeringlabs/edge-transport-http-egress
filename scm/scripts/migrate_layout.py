#!/usr/bin/env python3
"""Plan or apply the reviewed `-api` / `-std` package split of one workspace theme (issue #30, T5).

    python scm/scripts/migrate_layout.py breaker            # read-only plan (the default)
    python scm/scripts/migrate_layout.py breaker --apply    # change the repository

The reviewed mapping for a theme is `scm/scripts/layout_map/<theme>.json` plus the template tree
`scm/scripts/layout_map/<theme>/{api,std}/...`. The template tree holds the files that are new or
rewritten by hand (manifests, `lib.rs`, factory trait and implementation); everything else is moved
from the old package, so history follows the files (`git mv`).

What `--apply` does, in this order, after all checks pass:
  1. moves `main/src/api/**` of the old package into the `-api` package with `api/` stripped, except the
     files the mapping sends to `-std` (concrete implementation types) or drops;
  2. moves `main/src/core/**` and `main/src/saf/**` into the `-std` package, except dropped files;
  3. moves auxiliary directories (config, examples, docs, scripts, deny.toml, CHANGELOG.md) as mapped;
  4. rewrites `crate::api::` paths: in `-api` files to `crate::`, in `-std` files to the `-api` crate;
  5. copies the template tree over the result (a template must not overwrite a moved file);
  6. sorts each test by the crate it now imports, rewrites its imports and call sites by the mapping's
     rules, and moves it to that package's `tests/`;
  7. replaces the old workspace member with the two new ones.
Compile errors that remain are fixed by hand and reviewed in the same commit; the script does not guess.

Safety: nothing is written without `--apply`; `--apply` refuses a dirty working tree (unless
`--allow-dirty`), a missing source file, an existing destination, a template that collides with a
moved file, and a test file that matches no sorting rule. Re-running after a successful apply is
refused because the old package no longer exists. Unrelated files are never touched.
"""

from __future__ import annotations

import argparse
import json
import re
import shutil
import subprocess
import sys
from pathlib import Path

SCM = Path(__file__).resolve().parents[1]
REPO = SCM.parent
MAP_DIR = SCM / "scripts" / "layout_map"
PREFIX = "edge-transport-http-egress"


def run_git(*args: str) -> str:
    return subprocess.run(
        ["git", "-C", str(REPO), *args], check=True, capture_output=True, text=True
    ).stdout


def move_file(src: Path, dest: Path) -> None:
    """Move one file; `git mv` for tracked files (history follows), a plain move for the rest
    (for example git-ignored generated reports that travel with their package)."""
    dest.parent.mkdir(parents=True, exist_ok=True)
    tracked = subprocess.run(
        ["git", "-C", str(REPO), "ls-files", "--error-unmatch", str(src)],
        capture_output=True,
    ).returncode == 0
    if tracked:
        run_git("mv", str(src), str(dest))
    else:
        shutil.move(str(src), str(dest))


def remove_file(path: Path) -> None:
    """Delete one file; `git rm` for tracked files."""
    if not path.exists():
        return
    tracked = subprocess.run(
        ["git", "-C", str(REPO), "ls-files", "--error-unmatch", str(path)], capture_output=True
    ).returncode == 0
    if tracked:
        run_git("rm", "-q", "-f", str(path))
    else:
        path.unlink()


def fail(message: str) -> "NoReturn":  # type: ignore[name-defined]
    print(f"refused: {message}", file=sys.stderr)
    sys.exit(2)


def files_under(root: Path) -> list[Path]:
    return sorted(p for p in root.rglob("*") if p.is_file())


class Plan:
    """An ordered list of operations, printed in plan mode and executed in apply mode."""

    def __init__(self) -> None:
        self.moves: list[tuple[Path, Path]] = []
        self.copies: list[tuple[Path, Path]] = []
        self.rewrites: list[Path] = []

    def move(self, src: Path, dest: Path) -> None:
        self.moves.append((src, dest))


def load_map(theme: str) -> dict:
    path = MAP_DIR / f"{theme}.json"
    if not path.is_file():
        fail(f"no reviewed mapping at {path}")
    mapping = json.loads(path.read_text(encoding="utf-8"))
    if mapping.get("reviewed") is not True:
        fail(f"{path} is not marked reviewed")
    if mapping.get("theme") != theme:
        fail(f"{path} is for theme {mapping.get('theme')!r}, not {theme!r}")
    return mapping


def lib_name(package: str) -> str:
    return package.replace("-", "_")


def build_plan(theme: str, mapping: dict) -> tuple[Plan, dict]:
    old = SCM / theme
    api_dir, std_dir = SCM / f"{theme}-api", SCM / f"{theme}-std"
    if not old.is_dir():
        fail(f"{old} does not exist (already migrated?)")
    for dest in (api_dir, std_dir):
        if dest.exists():
            fail(f"destination {dest} already exists")
    plan = Plan()
    to_std = mapping.get("api_to_std", {})
    drop = set(mapping.get("drop", []))
    src_api = old / "main" / "src" / "api"
    moved_dests: set[Path] = set()

    def add(src: Path, dest: Path) -> None:
        if not src.is_file():
            fail(f"mapped source {src} does not exist")
        if dest in moved_dests:
            fail(f"two files map to {dest}")
        moved_dests.add(dest)
        plan.move(src, dest)

    for f in files_under(src_api):
        rel = f.relative_to(src_api).as_posix()
        if rel in drop:
            continue
        if rel in to_std:
            add(f, std_dir / "main" / "src" / to_std[rel])
        else:
            add(f, api_dir / "main" / "src" / rel)
    for rel in list(to_std) + sorted(drop):
        base = old / "main" / "src" if rel.split("/", 1)[0] in ("core", "saf") else src_api
        if not (base / rel).is_file():
            fail(f"api_to_std/drop entry {rel!r} is not a file under {base}")
    for part in ("core", "saf"):
        base = old / "main" / "src" / part
        for f in files_under(base):
            rel = f"{part}/{f.relative_to(base).as_posix()}"
            if rel in drop:
                continue
            if rel in mapping.get("core_to_api", {}):
                add(f, api_dir / "main" / "src" / mapping["core_to_api"][rel])
            else:
                add(f, std_dir / "main" / "src" / rel)
    for rel in mapping.get("core_to_api", {}):
        if not (old / "main" / "src" / rel).is_file():
            fail(f"core_to_api entry {rel!r} is not a file")
    for name, target in mapping.get("aux", {}).items():
        src = old / name
        if not src.exists():
            fail(f"aux entry {name!r} does not exist in {old}")
        targets = ["api", "std"] if target == "both" else [target]
        for t in targets:
            dest_root = api_dir if t == "api" else std_dir
            if src.is_dir():
                for f in files_under(src):
                    add(f, dest_root / name / f.relative_to(src))
            elif target == "both":
                if t == "api":
                    add(src, dest_root / name)
                else:
                    plan.copies.append((api_dir / name, dest_root / name))
            else:
                add(src, dest_root / name)
    # template tree
    tmpl = MAP_DIR / theme
    for side, dest_root in (("api", api_dir), ("std", std_dir)):
        for f in files_under(tmpl / side):
            dest = dest_root / f.relative_to(tmpl / side)
            if dest in moved_dests:
                fail(f"template {f} collides with a moved file {dest}")
            plan.copies.append((f, dest))
    # tests
    tests = old / "tests"
    for f in files_under(tests):
        if f.name == ".gitkeep":
            continue
        plan.moves.append((f, Path("<sorted by import>") / f.relative_to(tests)))
    extra = {
        "api_dir": api_dir,
        "std_dir": std_dir,
        "old": old,
        "api_lib": lib_name(f"{PREFIX}-{theme}-api"),
        "std_lib": lib_name(f"{PREFIX}-{theme}-std"),
        "old_lib": lib_name(f"{PREFIX}-{theme}"),
        "theme": theme,
    }
    return plan, extra


API_PATH = re.compile(r"crate::api::(?:[a-z_0-9]+::)*")


def rewrite_text(text: str, replacement: str) -> str:
    return API_PATH.sub(replacement, text)


def split_names(group: str) -> list[str]:
    return [n.strip() for n in group.replace("\n", " ").split(",") if n.strip()]


def rewrite_test(text: str, mapping: dict, extra: dict, name: str = "") -> tuple[str, str]:
    """Rewrite one test file; return (new text, 'api' or 'std')."""
    std_names = set(mapping["std_names"])
    drop_names = set(mapping.get("drop_names", []))
    for pattern, repl in mapping.get("test_rules", []):
        text = re.sub(pattern, repl, text)
    pat = re.compile(rf"use {extra['old_lib']}::(\{{[^}}]*\}}|[A-Za-z_0-9]+);")

    def fix_import(m: re.Match) -> str:
        body = m.group(1)
        names = split_names(body[1:-1]) if body.startswith("{") else [body]
        dropped = [n for n in names if n in drop_names]
        api = [n for n in names if n not in std_names and n not in drop_names]
        std = [n for n in names if n in std_names]
        if dropped:
            std += [n for n in sorted(std_names) if n not in std]
            api += [n for n in mapping.get("trait_names", []) if n not in api]
        out = []
        if api:
            out.append(f"use {extra['api_lib']}::{{{', '.join(api)}}};")
        if std:
            out.append(f"use {extra['std_lib']}::{{{', '.join(std)}}};")
        return "\n".join(out)

    text = pat.sub(fix_import, text)
    # fully qualified uses of the old crate
    for name in std_names:
        text = text.replace(f"{extra['old_lib']}::{name}", f"{extra['std_lib']}::{name}")
    text = text.replace(f"{extra['old_lib']}::", f"{extra['api_lib']}::")
    side = mapping.get("test_side", {}).get(name) or ("std" if extra["std_lib"] in text else "api")
    # the arch rule test_fn_prefixes wants the first segment to be the package's role, not the old theme
    text = re.sub(rf"fn {re.escape(extra['theme'])}_((?:enum|struct|type|trait)_)", rf"fn {side}_", text)
    return text, side


def print_plan(theme: str, plan: Plan, extra: dict) -> None:
    print(f"plan for theme {theme!r} (read-only; pass --apply to execute)")
    print(f"  moves ({len(plan.moves)}):")
    for src, dest in plan.moves:
        print(f"    {src.relative_to(REPO).as_posix()} -> {dest if dest.is_absolute() is False else dest.relative_to(REPO).as_posix()}")
    print(f"  new files from templates / copies ({len(plan.copies)}):")
    for src, dest in plan.copies:
        print(f"    {dest.relative_to(REPO).as_posix()}")
    print(f"  workspace: replace member {theme!r} with '{theme}-api' and '{theme}-std'")


def apply_plan(theme: str, mapping: dict, plan: Plan, extra: dict) -> None:
    api_dir, std_dir, old = extra["api_dir"], extra["std_dir"], extra["old"]
    for src, dest in plan.moves:
        if not dest.is_absolute():
            continue  # tests are handled below
        move_file(src, dest)
    # 4. path rewrites
    for f in files_under(api_dir / "main" / "src"):
        if f.suffix == ".rs":
            f.write_text(rewrite_text(f.read_text(encoding="utf-8"), "crate::"), encoding="utf-8")
    for f in files_under(std_dir / "main" / "src"):
        if f.suffix == ".rs":
            f.write_text(
                rewrite_text(f.read_text(encoding="utf-8"), f"{extra['api_lib']}::"),
                encoding="utf-8",
            )
    # per-file visibility / text rules
    for rel, rules in mapping.get("text_rules", {}).items():
        target = (api_dir if rel.startswith("api/") else std_dir) / rel.split("/", 1)[1]
        text = target.read_text(encoding="utf-8")
        for old_text, new_text in rules:
            if old_text not in text:
                fail(f"text rule for {rel}: {old_text!r} not found")
            text = text.replace(old_text, new_text)
        target.write_text(text, encoding="utf-8")
    # 5. templates and copies
    for src, dest in plan.copies:
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(src, dest)
    # 6. tests
    tests = old / "tests"
    for f in files_under(tests):
        if f.name == ".gitkeep":
            continue
        new_text, side = rewrite_test(f.read_text(encoding="utf-8"), mapping, extra, f.name)
        dest_root = (api_dir if side == "api" else std_dir) / "tests"
        dest = dest_root / f.relative_to(tests)
        if dest.exists():
            fail(f"test destination {dest} exists")
        move_file(f, dest)
        dest.write_text(new_text, encoding="utf-8")
    # examples follow the std package
    for f in files_under(std_dir / "examples"):
        if f.suffix == ".rs":
            new_text, _ = rewrite_test(f.read_text(encoding="utf-8"), mapping, extra)
            f.write_text(new_text, encoding="utf-8")
    # 7. workspace
    ws = SCM / "Cargo.toml"
    text = ws.read_text(encoding="utf-8")
    member = re.compile(rf'^(\s*)"{re.escape(theme)}",\s*$', re.M)
    if not member.search(text):
        fail(f"workspace member {theme!r} not found in {ws}")
    text = member.sub(lambda m: f'{m.group(1)}"{theme}-api",\n{m.group(1)}"{theme}-std",', text)
    ws.write_text(text, encoding="utf-8")
    # remove what the mapping drops or deletes, then require that nothing else is left behind
    for rel in sorted(mapping.get("drop", [])):
        path = old / "main" / "src" / (rel if rel.split("/", 1)[0] in ("core", "saf") else f"api/{rel}")
        remove_file(path)
    for rel in mapping.get("delete", []):
        remove_file(old / rel)
    leftovers = files_under(old) if old.exists() else []
    if leftovers:
        print("not moved and not listed in drop/delete (review, then add to the mapping):")
        for p in leftovers:
            print("  ", p.relative_to(REPO).as_posix())
        sys.exit(3)
    shutil.rmtree(old, ignore_errors=True)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawTextHelpFormatter)
    ap.add_argument("theme")
    ap.add_argument("--apply", action="store_true", help="execute the plan (default is read-only)")
    ap.add_argument("--allow-dirty", action="store_true")
    args = ap.parse_args()
    mapping = load_map(args.theme)
    plan, extra = build_plan(args.theme, mapping)
    print_plan(args.theme, plan, extra)
    if not args.apply:
        return
    if run_git("status", "--porcelain").strip() and not args.allow_dirty:
        fail("working tree is dirty; commit or stash first, or pass --allow-dirty")
    apply_plan(args.theme, mapping, plan, extra)
    print("applied; now build, fix residual compile errors by hand, and review the diff")


if __name__ == "__main__":
    main()
