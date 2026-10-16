# TASK-0018 — verificación local Rust + gobernanza documental
$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

Write-Host "== sanctumlm verify (TASK-0018) =="

# rustfmt: solo con toolchain nightly explícita; no fallar si falta
$fmtResult = "not-run"
if (Get-Command rustup -ErrorAction SilentlyContinue) {
    $nightly = rustup toolchain list 2>$null | Select-String -Pattern "^nightly"
    if ($nightly) {
        if (Get-Command cargo -ErrorAction SilentlyContinue) {
            Write-Host ">> cargo +nightly fmt --check"
            cargo +nightly fmt --check
            if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
            $fmtResult = "pass"
        }
    } else {
        Write-Host ">> cargo fmt --check: not-run (no nightly toolchain)"
    }
} else {
    Write-Host ">> cargo fmt --check: not-run (rustup not found)"
}

Write-Host ">> cargo test --workspace"
cargo test --workspace
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host ">> python scripts/validate_docs.py"
python scripts/validate_docs.py
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host ">> fmt summary: $fmtResult"
Write-Host "== verify OK =="
