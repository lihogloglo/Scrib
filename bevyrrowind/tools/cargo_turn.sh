#!/usr/bin/env bash
# Waits for the build slot, then runs cargo at BelowNormal priority (CLAUDE.md "Parallel sessions").
# The slot is free when no cargo.exe or rustc.exe runs, from any project (the owner builds others).
#
#   bash tools/cargo_turn.sh build -p bevyrrowind --features dev -j 6
#   bash ../bevyrrowind/tools/cargo_turn.sh test -p sf_ui_css      # from superfargoth
#
# Pipe the output through a filter if it's long (e.g. `2>&1 | grep -E "^(error|warning)" -A5`).
busy() { tasklist 2>/dev/null | grep -qiE '^(cargo|rustc)\.exe'; }
while busy; do sleep 5; done
# A short settle so two sessions that saw the slot free at once don't both start.
sleep $((RANDOM % 4 + 1))
while busy; do sleep 5; done
exec cmd //c start "" //belownormal //b //wait cargo "$@"
