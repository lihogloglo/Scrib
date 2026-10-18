# Starts the dev exe (`--features dev`, Bevy dynamically linked, BRP on) without
# `cargo run`, so no cargo process lingers while the game is open: other
# sessions' "is a build running?" checks look for cargo/rustc (CLAUDE.md).
# Build first: `cargo build -p bevyrrowind --features dev`.
#
#   powershell -File tools/run_dev.ps1 view "Seyda Neen, Census and Excise Office"
#   powershell -File tools/run_dev.ps1 scene wk-balmora
#
# Prints the process id (from bash: `> target/run_dev.pid`, not `$(...)`: the game
# inherits the pipe and the capture would wait for it to exit); its stdout/stderr
# go to target/run_dev.{out,err}.log.
# Wait for BRP with `bevyrrowind/status` until `ready` (CLAUDE.md "Live inspection (BRP)").

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$exe = Join-Path $root 'target\debug\bevyrrowind.exe'
if (-not (Test-Path $exe)) { throw "no dev build at $exe" }
# The DLLs `cargo run` would put on PATH: Rust std (sysroot bin) and bevy_dylib (deps).
$sysroot = (& rustc --print sysroot).Trim()
$env:PATH = "$sysroot\bin;$root\target\debug\deps;$root\target\debug;$env:PATH"
$quoted = $args | ForEach-Object { if ($_ -match '\s') { '"' + $_ + '"' } else { $_ } }
$p = Start-Process -FilePath $exe -ArgumentList $quoted -WorkingDirectory $root -PassThru `
    -RedirectStandardOutput (Join-Path $root 'target\run_dev.out.log') `
    -RedirectStandardError (Join-Path $root 'target\run_dev.err.log')
$p.Id
