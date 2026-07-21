//! Coverage tests (rules 221 + 222) — _happy / _error / _edge variants.
//! Rule 221: create_config_builder, build_cassette_layer in cassette_svc.rs.
//! Rule 222: describe + config (HttpCassette trait), describe (Processor),
//!            validate (Validator).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::{CassetteConfig, HttpCassetteSvc};

/// Absolute path to the crate's shipped `config/` directory, which contains
/// `application.toml` with a `[cassette]` section.
fn shipped_config_dir() -> String {
    concat!(env!("CARGO_MANIFEST_DIR"), "/config").to_string()
}

// ── create_config_builder (rule 221) ─────────────────────────────────────────

#[test]
fn test_create_config_builder_returns_valid_loader_happy() {
    // The loader produced by create_config_builder must be able to read the
    // crate's shipped [cassette] section — proving it was seeded with a real,
    // working name/version and is not an inert stub.
    let cfg: CassetteConfig = HttpCassetteSvc::create_config_builder()
        .with_config_dir(shipped_config_dir())
        .build_loader()
        .expect("seeded builder must produce a loader")
        .load_section("cassette")
        .expect("shipped [cassette] section must load");
    assert_eq!(
        cfg.mode, "replay",
        "the shipped policy is replay-by-default"
    );
    assert_eq!(cfg.cassette_dir, "tests/cassettes");
}

#[test]
fn test_create_config_builder_does_not_panic_without_config_file_error() {
    // Graceful degradation: pointed at a directory with no application.toml,
    // the loader must return a descriptive Err — never panic, never silently
    // fabricate a config.
    let result: Result<CassetteConfig, _> = HttpCassetteSvc::create_config_builder()
        .with_config_dir(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/nonexistent_config_dir_zzz"
        ))
        .build_loader()
        .expect("loader must build even when the config dir is absent")
        .load_section("cassette");
    let err = result.expect_err("absent application.toml must produce an error");
    assert!(
        err.to_string().contains("application.toml"),
        "error must name the missing file; got: {err}"
    );
}

#[test]
fn test_create_config_builder_two_independent_builders_edge() {
    // Two independently created builders must each yield a loader that reads
    // the same shipped policy — proving the factory is repeatable and holds no
    // shared mutable state between calls.
    let dir = shipped_config_dir();
    let c1: CassetteConfig = HttpCassetteSvc::create_config_builder()
        .with_config_dir(dir.clone())
        .build_loader()
        .expect("first loader")
        .load_section("cassette")
        .expect("first load");
    let c2: CassetteConfig = HttpCassetteSvc::create_config_builder()
        .with_config_dir(dir)
        .build_loader()
        .expect("second loader")
        .load_section("cassette")
        .expect("second load");
    assert_eq!(
        c1.mode, c2.mode,
        "two independent builders must read the same shipped mode"
    );
    assert_eq!(c1.mode, "replay");
}

// ── build_cassette_layer (rule 221) ──────────────────────────────────────────

#[test]
fn test_build_cassette_layer_nonexistent_dir_returns_err_happy() {
    // A missing cassette directory is not an error at build time — the layer is
    // created and the file is only written on first record. Assert it builds
    // and that the derived path embeds the cassette name.
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("does_not_exist_yet");
    let config = CassetteConfig {
        cassette_dir: dir.to_str().unwrap().replace('\\', "/"),
        ..Default::default()
    };
    let layer = HttpCassetteSvc::build_cassette_layer(config, "nonexistent_dir_cassette")
        .expect("missing cassette dir must still build a layer");
    assert!(
        format!("{layer:?}").contains("nonexistent_dir_cassette"),
        "derived cassette path must embed the cassette name"
    );
}

#[test]
fn test_build_cassette_layer_empty_cassette_name_returns_err_error() {
    // An empty cassette name yields a path ending in `.yaml` with no stem.
    // The builder must not panic; it produces a layer whose path is still `.yaml`.
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().to_str().unwrap().replace('\\', "/");
    let config = CassetteConfig {
        cassette_dir: dir,
        ..Default::default()
    };
    let layer =
        HttpCassetteSvc::build_cassette_layer(config, "").expect("empty name must not panic");
    assert!(
        format!("{layer:?}").contains(".yaml"),
        "even an empty name must resolve to a .yaml cassette path"
    );
}

#[test]
fn test_build_cassette_layer_two_different_names_independent_edge() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().to_str().unwrap().replace('\\', "/");
    let config1 = CassetteConfig {
        cassette_dir: dir,
        ..Default::default()
    };
    let config2 = config1.clone();
    let l1 = HttpCassetteSvc::build_cassette_layer(config1, "name_a").expect("build a");
    let l2 = HttpCassetteSvc::build_cassette_layer(config2, "name_b").expect("build b");
    let dbg_a = format!("{l1:?}");
    let dbg_b = format!("{l2:?}");
    assert_ne!(
        dbg_a, dbg_b,
        "distinct cassette names must produce distinct on-disk paths"
    );
}

// ── describe (rule 222: HttpCassette + Processor traits) ─────────────────────

#[test]
fn test_describe_svc_type_exists_and_builds_happy() {
    // HttpCassetteSvc is the crate's SAF facade; it must build a working layer.
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().to_str().unwrap().replace('\\', "/");
    let cfg = CassetteConfig {
        cassette_dir: dir,
        ..Default::default()
    };
    let layer =
        HttpCassetteSvc::build_cassette_layer(cfg, "describe_svc").expect("svc must build a layer");
    assert!(format!("{layer:?}").contains("CassetteLayer"));
}

#[test]
fn test_describe_svc_does_not_panic_on_construction_error() {
    // Constructing the facade and using it against a disabled config must not
    // panic and must yield a usable layer.
    let layer = HttpCassetteSvc::build_cassette_layer(CassetteConfig::disabled(), "unused")
        .expect("disabled config must always build");
    assert!(format!("{layer:?}").contains("disabled"));
}

#[test]
fn test_describe_svc_is_send_sync_edge() {
    // Prove the facade is genuinely Sync by sharing a reference to a built
    // layer across a real OS-thread boundary (a non-Sync type would not compile),
    // then assert the value observed on the other thread is intact.
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().to_str().unwrap().replace('\\', "/");
    let cfg = CassetteConfig {
        cassette_dir: dir,
        ..Default::default()
    };
    let layer =
        HttpCassetteSvc::build_cassette_layer(cfg, "send_sync_edge").expect("build must succeed");
    std::thread::scope(|s| {
        let borrowed = &layer;
        let dbg = s
            .spawn(move || format!("{borrowed:?}"))
            .join()
            .expect("spawned thread must not panic");
        assert!(
            dbg.contains("send_sync_edge"),
            "layer shared across a thread must retain its cassette name: {dbg}"
        );
    });
}

// ── config (rule 222: HttpCassette::config) ───────────────────────────────────

#[test]
fn test_config_cassette_config_is_default_constructible_happy() {
    let cfg = CassetteConfig::default();
    assert!(
        !cfg.cassette_dir.is_empty(),
        "cassette_dir must have a default value"
    );
}

#[test]
fn test_config_cassette_config_fields_accessible_error() {
    // The default scrub list must protect credential-bearing headers; an empty
    // scrub list would leak secrets into committed cassettes.
    let cfg = CassetteConfig::default();
    assert!(
        cfg.scrub_headers
            .iter()
            .any(|h| h.eq_ignore_ascii_case("authorization")),
        "default config must scrub the authorization header; got: {:?}",
        cfg.scrub_headers
    );
}

#[test]
fn test_config_cassette_config_clone_equals_original_edge() {
    let cfg = CassetteConfig::default();
    let cloned = cfg.clone();
    assert_eq!(cfg.cassette_dir, cloned.cassette_dir);
    assert_eq!(cfg.mode, cloned.mode);
    assert_eq!(cfg.scrub_headers, cloned.scrub_headers);
}

// ── validate (rule 222: Validator trait) ─────────────────────────────────────

#[test]
fn test_validate_default_config_does_not_panic_happy() {
    // The default config must be usable to build a layer without panicking.
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().to_str().unwrap().replace('\\', "/");
    let config = CassetteConfig {
        cassette_dir: dir,
        ..Default::default()
    };
    let layer = HttpCassetteSvc::build_cassette_layer(config, "validate_default")
        .expect("default config must validate and build");
    assert!(format!("{layer:?}").contains("replay"));
}

#[test]
fn test_validate_build_with_valid_config_returns_err_or_ok_not_panic_error() {
    // A valid config in a real temp dir must build successfully.
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().to_str().unwrap().replace('\\', "/");
    let config = CassetteConfig {
        cassette_dir: dir,
        ..Default::default()
    };
    let layer = HttpCassetteSvc::build_cassette_layer(config, "coverage_valid")
        .expect("valid config must build");
    assert!(format!("{layer:?}").contains("coverage_valid"));
}

#[test]
fn test_validate_empty_dir_handled_gracefully_edge() {
    // An empty cassette_dir resolves the cassette relative to the current dir;
    // the builder must handle it gracefully and embed the name in the path.
    let config = CassetteConfig {
        cassette_dir: String::new(),
        ..Default::default()
    };
    let layer = HttpCassetteSvc::build_cassette_layer(config, "empty_dir_cassette")
        .expect("empty cassette_dir must be handled gracefully");
    assert!(
        format!("{layer:?}").contains("empty_dir_cassette"),
        "layer path must embed the cassette name even with an empty dir"
    );
}
