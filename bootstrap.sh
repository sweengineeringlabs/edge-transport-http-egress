#!/usr/bin/env bash
set -euo pipefail
# Delegates to the canonical bootstrap script — scm/ is the actual crate
# root; this file exists only to satisfy the workspace-root onboarding
# entry point arch expects.
exec "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/scm/bootstrap.sh" "$@"
