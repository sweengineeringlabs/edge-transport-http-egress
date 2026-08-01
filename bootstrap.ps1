# Delegates to the canonical bootstrap script — scm/ is the actual crate
# root; this file exists only to satisfy the workspace-root onboarding
# entry point arch expects.
$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
& (Join-Path $repoRoot 'scm\bootstrap.ps1')
