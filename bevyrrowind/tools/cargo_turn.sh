#!/usr/bin/env bash
# Runs cargo in turn: a first-come, first-served queue shared by every session, then cargo at BelowNormal
# priority (CLAUDE.md "Parallel sessions"). The queue is `<repo>/.sessions/build-queue/`, one ticket per waiting
# or running command; the oldest ticket builds. Builds outside the queue (other projects, a bare cargo) still
# hold everyone: the slot is also busy while any rustc.exe runs, or any cargo.exe other than one that only runs a
# finished program (`cargo run` / `cargo dev`, e.g. a game left open; owner 2026-10-26).
#
#   bash tools/cargo_turn.sh build -p bevyrrowind --features dev
#   bash ../bevyrrowind/tools/cargo_turn.sh test -p sf_ui_css      # from superfargoth
#   bash tools/cargo_turn.sh --queue                               # show the queue
#
# Pipe the output through a filter if it's long (e.g. `2>&1 | grep -E "^(error|warning)" -A5`).
root="$(cd "$(dirname "$0")/../.." && pwd)"
queue="$root/.sessions/build-queue"
freeze="$root/.sessions/BUILD-FREEZE"
mkdir -p "$queue"

busy() {
    # A headless game subcommand (a bake, a census) is real work even when started by `cargo run`, and a `--bench`
    # run must not share the CPU: both hold the slot. Same headless list as owner_launch.sh game_open(); a game
    # window doesn't hold it.
    if tasklist 2>/dev/null | grep -qiE '^bevyrrowind\.exe'; then
        powershell -NoProfile -Command "Get-CimInstance Win32_Process -Filter \"Name='bevyrrowind.exe'\" | ForEach-Object { \$_.CommandLine }" 2>/dev/null \
            | tr -d '\r' | grep -qE '[[:space:]](bake|info|mods|census|clip-preview --export|--bench)([[:space:]]|$)' && return 0
    fi
    tasklist 2>/dev/null | grep -qiE '^rustc\.exe' && return 0
    tasklist 2>/dev/null | grep -qiE '^cargo\.exe' || return 1
    # Each cargo.exe's subcommand: the first word after the exe path that isn't a flag or +toolchain.
    powershell -NoProfile -Command "Get-CimInstance Win32_Process -Filter \"Name='cargo.exe'\" | ForEach-Object { \$_.CommandLine }" 2>/dev/null \
        | tr -d '\r' \
        | sed -E 's/^("[^"]*"|[^ ]*)//' \
        | awk '{ sub_ = ""; for (i = 1; i <= NF; i++) if ($i !~ /^[-+]/) { sub_ = $i; break } print sub_ }' \
        | grep -qvxE 'run|dev'
}

# Drop tickets whose shell is gone (killed sessions), then print the live ones oldest first.
live_tickets() {
    local t pid
    for t in "$queue"/*; do
        [ -f "$t" ] || continue
        pid="${t##*-}"
        if kill -0 "$pid" 2>/dev/null; then echo "${t##*/}"; else rm -f "$t"; fi
    done | sort
}

if [ "$1" = "--queue" ]; then
    now=$(date +%s)
    n=0
    for t in $(live_tickets); do
        n=$((n + 1))
        since=$(( now - ${t%%.*} ))
        state="waiting"; grep -q '^running ' "$queue/$t" && state="RUNNING since $(grep '^running ' "$queue/$t" | cut -d' ' -f2)"
        printf '%2d  %4ds  %-22s %s\n' "$n" "$since" "$state" "$(head -1 "$queue/$t")"
    done
    [ $n = 0 ] && echo "queue empty"
    if busy && ! grep -qs '^running ' "$queue"/* ; then echo "(a build outside the queue is running)"; fi
    exit 0
fi

# Ticket: <epoch seconds>.<nanoseconds>-<shell pid>, holding the command and where it came from.
ticket="$queue/$(date +%s.%N)-$$"
printf '%s  [%s]\n' "cargo $*" "$PWD" > "$ticket"
trap 'rm -f "$ticket"' EXIT
trap 'exit 130' INT TERM

shown=""
while :; do
    first="$(live_tickets | head -1)"
    if [ "$first" = "${ticket##*/}" ]; then
        if [ -f "$freeze" ] && [ -z "$SCRIB_BUILD_FREEZE_OK" ]; then
            msg="build freeze ($(head -1 "$freeze"))"
        elif busy; then
            msg="next in line, waiting for the running build"
        else
            break
        fi
    else
        pos=$(live_tickets | grep -nxF "${ticket##*/}" | cut -d: -f1)
        msg="queued #$pos behind: $(head -1 "$queue/$first" 2>/dev/null)"
    fi
    if [ "$msg" != "$shown" ]; then echo "cargo_turn: $msg" >&2; shown="$msg"; fi
    sleep 3
done
echo "running $(date +%H:%M:%S)" >> "$ticket"
# Build freeze: while .sessions/BUILD-FREEZE exists, only commands run with SCRIB_BUILD_FREEZE_OK=1 leave the
# queue (handled above); everyone else waits in line. The file says why.

# Thread cap only while the owner plays (owner 2026-11-11): a `-j N` / `--jobs N` before `--` is kept only when a
# private owner copy (%TEMP%\bw-owner-*) is running; otherwise cargo uses every core (BelowNormal keeps the desktop smooth).
owner_playing() {
    powershell -NoProfile -Command "Get-CimInstance Win32_Process | Where-Object { \$_.ExecutablePath -like '*\\bw-owner-*' } | Select-Object -First 1 -ExpandProperty ProcessId" 2>/dev/null | grep -q '[0-9]'
}
if ! owner_playing; then
    args=(); skip=0; dd=0
    for a in "$@"; do
        if [ $dd = 1 ]; then args+=("$a"); continue; fi
        if [ $skip = 1 ]; then skip=0; continue; fi
        case "$a" in
            --) dd=1; args+=("$a") ;;
            -j|--jobs) skip=1 ;;
            -j[0-9]*|--jobs=*) ;;
            *) args+=("$a") ;;
        esac
    done
    set -- "${args[@]}"
fi

# `cargo run` / `cargo dev` keep running while the program runs (a game window, a bake): leave the queue as they
# start, so a game left open doesn't hold everyone; while they still compile, their rustc keeps the slot busy.
sub=""
for a in "$@"; do case "$a" in -*|+*) ;; *) sub="$a"; break ;; esac; done
if [ "$sub" = run ] || [ "$sub" = dev ]; then
    rm -f "$ticket"
    trap - EXIT
    exec nice -n 10 cargo "$@"
fi
# `nice` gives cargo (and the rustc it starts, which inherits the class) BelowNormal priority and keeps cargo's
# exit code. The ticket stays until cargo exits, so the next in line starts right after.
nice -n 10 cargo "$@"
