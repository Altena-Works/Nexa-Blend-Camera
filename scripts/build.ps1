param([switch]$Test)
$ErrorActionPreference = 'Stop'
$project = Split-Path -Parent $PSScriptRoot
Push-Location -LiteralPath $project
try {
    if ($Test) {
        & rustup run 1.95.0 cargo test --offline --locked --workspace -- --test-threads=1
        if ($LASTEXITCODE -ne 0) { throw "Tests failed ($LASTEXITCODE)" }
    }
    & rustup run 1.95.0 cargo build --offline --locked --release --workspace
    if ($LASTEXITCODE -ne 0) { throw "Build failed ($LASTEXITCODE)" }
} finally {
    Pop-Location
}
