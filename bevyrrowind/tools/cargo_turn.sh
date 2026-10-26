#!/usr/bin/env bash
# Waits for the build slot, then runs cargo at BelowNormal priority (CLAUDE.md "Parallel sessions").
# The slot is free when nothing builds, from any project (the owner builds others): no rustc.exe, and
# no cargo.exe other than one that only runs a finished program (`cargo run` / `cargo dev`, e.g. a game
# left open; owner 2026-10-26). A `cargo run` still compiling has rustc children, so it still counts.
#
#   bash tools/cargo_turn.sh build -p bevyrrowind --features dev -j 6
#   bash ../bevyrrowind/tools/cargo_turn.sh test -p sf_ui_css      # from superfargoth
#
# Pipe the output through a filter if it's long (e.g. `2>&1 | grep -E "^(error|warning)" -A5`).
busy() {
    tasklist 2>/dev/null | grep -qiE '^rustc\.exe' && return 0
    tasklist 2>/dev/null | grep -qiE '^cargo\.exe' || return 1
    # Each cargo.exe's subcommand: the first word after the exe path that isn't a flag or +toolchain.
    powershell -NoProfile -Command "Get-CimInstance Win32_Process -Filter \"Name='cargo.exe'\" | ForEach-Object { \$_.CommandLine }" 2>/dev/null \
        | tr -d '\r' \
        | sed -E 's/^("[^"]*"|[^ ]*)//' \
        | awk '{ sub_ = ""; for (i = 1; i <= NF; i++) if ($i !~ /^[-+]/) { sub_ = $i; break } print sub_ }' \
        | grep -qvxE 'run|dev'
}
while busy; do sleep 5; done
# A short settle so two sessions that saw the slot free at once don't both start.
sleep $((RANDOM % 4 + 1))
while busy; do sleep 5; done
exec cmd //c start "" //belownormal //b //wait cargo "$@"
