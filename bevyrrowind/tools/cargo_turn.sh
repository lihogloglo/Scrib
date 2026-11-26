#!/usr/bin/env bash
# Runs cargo in turn: a first-come, first-served queue shared by every session, then cargo at BelowNormal
# priority (CONTRIBUTING.md "Building"). The queue is `<repo>/.sessions/build-queue/`, one ticket per waiting
# or running command; the oldest ticket builds. Builds outside the queue (other projects, a bare cargo) still
# hold everyone: the slot is also busy while any rustc.exe runs, or any cargo.exe other than one that only runs a
# finished program (`cargo run` / `cargo dev`, e.g. a game left open; owner 2026-10-26).
#
#   bash tools/cargo_turn.sh build -p bevyrrowind --features dev
#   bash ../bevyrrowind/tools/cargo_turn.sh test -p sf_ui_css      # from superfargoth
#   bash tools/cargo_turn.sh --queue                               # show the queue
#   bash tools/cargo_turn.sh --stats [days]                        # sum the build log (default: last 7 days)
#
# Pipe the output through a filter if it's long (e.g. `2>&1 | grep -E "^(error|warning)" -A5`).
#
# Build log: every command appends one line to `<repo>/.sessions/build-log.tsv` (untracked, this machine only)
# when it ends: how long it waited and why (behind other tickets / a build outside the queue / a build freeze), how
# long cargo ran, its exit code, and how many units cargo compiled (Compiling/Checking lines on stderr), of which
# Bevy crates. Set SCRIB_SESSION=<name> to tag your lines. Passive: it measures the real workday, no benchmarks.
root="$(cd "$(dirname "$0")/../.." && pwd)"
# From a git worktree (tools/worktree.sh) the queue, freeze and log stay the main checkout's: one machine, one queue.
common="$(git -C "$root" rev-parse --path-format=absolute --git-common-dir 2>/dev/null)"
[ -n "$common" ] && [ -d "$(dirname "$common")/.sessions" ] && root="$(dirname "$common")"
queue="$root/.sessions/build-queue"
freeze="$root/.sessions/BUILD-FREEZE"
log="$root/.sessions/build-log.tsv"
mkdir -p "$queue"

ms() { local t; t=$(date +%s%N); echo $((t / 1000000)); }
secs() { printf '%d.%d' $(($1 / 1000)) $(($1 % 1000 / 100)); }

if [ "$1" = "--stats" ]; then
    [ -f "$log" ] || { echo "no build log yet ($log)"; exit 0; }
    days="${2:-7}"
    since="$(date -d "-$days days" '+%F %T')"
    awk -F'\t' -v since="$since" -v days="$days" '
        NR == 1 || $1 < since { next }
        {
            s = $3; n[s]++; N++
            wait[s] += $4; W += $4; wq += $5; wb += $6; wf += $7
            if ($8 != "-") { run[s] += $8; R += $8; r[s, ++nr[s]] = $8; if ($8 > mx[s]) mx[s] = $8 }
            if ($11 != "-" && $11 > 0) { bev[s]++; B++ }
            if ($9 != 0 && $9 != "exec") fail[s]++
            if ($8 != "-") slow[N] = sprintf("%8.1f s  %-6s %4s units %4s bevy  %s  %s", $8, s, $10, $11, $1, $13)
            slowkey[N] = ($8 == "-") ? -1 : $8
        }
        function median(s,   a, i, k) {
            if (!nr[s]) return 0
            for (i = 1; i <= nr[s]; i++) a[i] = r[s, i]
            k = asort(a); return (k % 2) ? a[(k + 1) / 2] : (a[k / 2] + a[k / 2 + 1]) / 2
        }
        END {
            if (!N) { print "no commands logged since " since; exit }
            printf "build log since %s (%s days): %d commands\n\n", since, days, N
            printf "%-10s %5s %10s %10s %11s %9s %13s %7s\n", "command", "n", "wait min", "run min", "median s", "max s", "compiled Bevy", "failed"
            for (s in n) printf "%-10s %5d %10.1f %10.1f %11.1f %9.1f %13d %7d\n", s, n[s], wait[s] / 60, run[s] / 60, median(s), mx[s], bev[s], fail[s]
            printf "%-10s %5d %10.1f %10.1f %11s %9s %13d\n\n", "total", N, W / 60, R / 60, "", "", B
            printf "waiting: %.1f min behind other tickets, %.1f min for a build outside the queue, %.1f min in a freeze,\n", wq / 60, wb / 60, wf / 60
            printf "         %.1f min in the queue script own checks (process scans)\n", (W - wq - wb - wf) / 60
            printf "failed = non-zero exit (compile errors, failing tests, Ctrl+C = 130); run/dev log their wait only\n\n"
            print "slowest runs:"
            k = asorti(slowkey, idx, "@val_num_desc")
            for (i = 1; i <= k && i <= 10; i++) if (slowkey[idx[i]] >= 0) print "  " slow[idx[i]]
        }' "$log"
    exit 0
fi

# Every Windows process's exe path, one per line. `ps -W` takes ~0.15 s where each tasklist took ~0.7 s and a
# PowerShell CIM query ~0.9 s (idle-1 L1, 2026-11-26: the scans were ~2 s of every command's wait). PowerShell runs
# only when a command line is needed (a cargo.exe or bevyrrowind.exe is present). Falls back to tasklist names.
procs() {
    local p
    p="$(ps -W 2>/dev/null | awk 'NR > 1 { sub(/^ *([^ ]+ +){7}/, ""); print }' | tr -d '\r')"
    if [ -n "$p" ]; then printf '%s\n' "$p"; else tasklist 2>/dev/null | awk '{ print "\\" $1 }' | tr -d '\r'; fi
}
has_exe() { grep -qiE "[\\\\/]($1)(\\.exe)?\$" <<< "$2"; } # <name regex> <procs output>

busy() {
    local p; p="$(procs)"
    # A headless game subcommand (a bake, a census) is real work even when started by `cargo run`, and a `--bench`
    # run must not share the CPU: both hold the slot. Same headless list as owner_launch.sh game_open(); a game
    # window doesn't hold it.
    if has_exe bevyrrowind "$p"; then
        powershell -NoProfile -Command "Get-CimInstance Win32_Process -Filter \"Name='bevyrrowind.exe'\" | ForEach-Object { \$_.CommandLine }" 2>/dev/null \
            | tr -d '\r' | grep -qE '[[:space:]](bake|info|mods|census|clip-preview --export|--bench)([[:space:]]|$)' && return 0
    fi
    # rustdoc.exe compiles and links doctests; a test run that has to recompile shows up as rustc.exe.
    has_exe 'rustc|rustdoc' "$p" && return 0
    has_exe cargo "$p" || return 1
    # Each cargo.exe's subcommand: the first word after the exe path that isn't a flag or +toolchain. `test-run` is
    # this script's second phase of a `cargo test` (its binaries running, already built; see the end of the file).
    powershell -NoProfile -Command "Get-CimInstance Win32_Process -Filter \"Name='cargo.exe'\" | ForEach-Object { \$_.CommandLine }" 2>/dev/null \
        | tr -d '\r' \
        | sed -E 's/^("[^"]*"|[^ ]*)//' \
        | awk '{ sub_ = ""; for (i = 1; i <= NF; i++) if ($i !~ /^[-+]/) { sub_ = $i; break } print sub_ }' \
        | grep -qvxE 'run|dev|test-run'
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

# Log state: times in ms; t_run stays empty until cargo starts (a command cancelled while waiting logs run "-").
queued_at="$(date '+%F %T')"; all_args="$*"
t_q=$(ms); last=$t_q; reason=""; w_queue=0; w_busy=0; w_freeze=0; t_run=""; errlog=""; capped=0; quiet=0
sub=""
for a in "$@"; do case "$a" in -*|+*) ;; *) sub="$a"; break ;; esac; done
for a in "$@"; do case "$a" in --) break ;; -q|--quiet) quiet=1 ;; esac; done

write_log() {
    local rc="$1" end wait run units="-" bevy="-"
    end=$(ms)
    [ -n "$t_run" ] || { [ "$reason" = queue ] && w_queue=$((w_queue + end - last)); [ "$reason" = busy ] && w_busy=$((w_busy + end - last)); [ "$reason" = freeze ] && w_freeze=$((w_freeze + end - last)); }
    # `run`/`dev` log as they hand over to cargo (exec): their run time is the program's, not a build's.
    run="-"
    if [ -z "$t_run" ] || [ "$t_run" = exec ]; then
        wait=$((end - t_q))
    else
        wait=$((t_run - t_q)); run=$(secs $((end - t_run)))
    fi
    if [ -n "$errlog" ] && [ -f "$errlog" ] && [ $quiet = 0 ]; then
        units=$(sed 's/\x1b\[[0-9;]*[mK]//g' "$errlog" | grep -cE '^ *(Compiling|Checking|Documenting) ')
        bevy=$(sed 's/\x1b\[[0-9;]*[mK]//g' "$errlog" | grep -cE '^ *(Compiling|Checking) bevy')
    fi
    [ -f "$log" ] || printf 'queued\twho\tcmd\twait_s\tqueue_s\tbusy_s\tfreeze_s\trun_s\texit\tunits\tbevy\tjobs_capped\targs\n' > "$log"
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$queued_at" "${SCRIB_SESSION:--}" "${sub:--}" \
        "$(secs $wait)" "$(secs $w_queue)" "$(secs $w_busy)" "$(secs $w_freeze)" "$run" "$rc" "$units" "$bevy" "$capped" \
        "$(printf '%s' "$all_args" | tr '\t' ' ')" >> "$log"
}
on_exit() {
    local rc=$?
    rm -f "$ticket"
    write_log "$rc" 2>/dev/null
    [ -n "$errlog" ] && rm -f "$errlog"
}
trap on_exit EXIT
trap 'exit 130' INT TERM

shown=""
while :; do
    now=$(ms)
    case "$reason" in
        queue) w_queue=$((w_queue + now - last)) ;;
        busy) w_busy=$((w_busy + now - last)) ;;
        freeze) w_freeze=$((w_freeze + now - last)) ;;
    esac
    last=$now
    first="$(live_tickets | head -1)"
    if [ "$first" = "${ticket##*/}" ]; then
        if [ -f "$freeze" ] && [ -z "$SCRIB_BUILD_FREEZE_OK" ]; then
            msg="build freeze ($(head -1 "$freeze"))"; reason=freeze
        elif busy; then
            msg="next in line, waiting for the running build"; reason=busy
        else
            break
        fi
    else
        pos=$(live_tickets | grep -nxF "${ticket##*/}" | cut -d: -f1)
        msg="queued #$pos behind: $(head -1 "$queue/$first" 2>/dev/null)"; reason=queue
    fi
    if [ "$msg" != "$shown" ]; then echo "cargo_turn: $msg" >&2; shown="$msg"; fi
    # Handoff latency: a waiter sees the ticket ahead go within ~1 s (was 3 s). Waiting on a build outside the queue
    # polls every 2 s, since each poll may cost a PowerShell query.
    if [ "$reason" = busy ]; then sleep 2; else sleep 1; fi
done
echo "running $(date +%H:%M:%S)" >> "$ticket"
# Build freeze: while .sessions/BUILD-FREEZE exists, only commands run with SCRIB_BUILD_FREEZE_OK=1 leave the
# queue (handled above); everyone else waits in line. The file says why.

# Thread cap only while the owner plays (owner 2026-11-11): a `-j N` / `--jobs N` before `--` is kept only when a
# private owner copy (%TEMP%\bw-owner-*) is running; otherwise cargo uses every core (BelowNormal keeps the desktop smooth).
owner_playing() {
    # Either separator: a process started from bash (owner_launch.sh) is listed with its MSYS path.
    procs | grep -qiE '[\\/]bw-owner-'
}
if owner_playing; then
    capped=1
else
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
if [ "$sub" = run ] || [ "$sub" = dev ]; then
    rm -f "$ticket"
    t_run=exec
    trap - EXIT
    write_log exec 2>/dev/null
    exec nice -n 10 cargo "$@"
fi
# `nice` gives cargo (and the rustc it starts, which inherits the class) BelowNormal priority. The ticket stays
# until cargo exits, so the next in line starts right after. Cargo's stderr is copied to a temp file for the log's
# unit counts (it still reaches the caller unchanged; colour kept when the caller is a terminal).
[ -t 2 ] && [ -z "${CARGO_TERM_COLOR:-}" ] && export CARGO_TERM_COLOR=always
errlog="$(mktemp 2>/dev/null)" || errlog=""
run_cargo() {
    if [ -n "$errlog" ]; then
        nice -n 10 cargo "$@" 2> >(tee "$errlog" >&2)
        rc=$?
        wait $! 2>/dev/null
    else
        nice -n 10 cargo "$@"
        rc=$?
    fi
}

# `cargo test` in two phases (build-speed BS2, 2026-11-26): the ticket covers only the build (`--no-run`); the
# binaries then run outside the queue as `cargo test-run` (an alias of `test` set through CARGO_ALIAS_TEST_RUN, so it
# works for any project, and busy() can tell it apart), and the next in line builds meanwhile. Running took 290 s of a no-compile `test --workspace`.
# The second cargo only checks freshness (a file changed in between recompiles, and its rustc holds the others
# through busy()). Not split: `--no-run` itself, `--doc` (cargo refuses `--doc --no-run`), SCRIB_TEST_ONE_PHASE=1.
two_phase=0
if [ "$sub" = test ] && [ -z "${SCRIB_TEST_ONE_PHASE:-}" ]; then
    two_phase=1
    for a in "$@"; do case "$a" in --) break ;; --no-run|--doc) two_phase=0 ;; esac; done
fi
if [ $two_phase = 0 ]; then
    t_run=$(ms)
    run_cargo "$@"
    exit $rc
fi

build_args=(); run_args=(); seen_sub=0; dd=0
for a in "$@"; do
    if [ $dd = 0 ] && [ "$a" = -- ]; then dd=1; build_args+=(--no-run); fi
    if [ $dd = 0 ]; then build_args+=("$a"); fi
    if [ $seen_sub = 0 ] && [ "$a" = test ]; then seen_sub=1; run_args+=(test-run); else run_args+=("$a"); fi
done
[ $dd = 0 ] && build_args+=(--no-run)

t_run=$(ms)
run_cargo "${build_args[@]}"
[ $rc = 0 ] || exit $rc
# Build done: log it as `test`, give up the ticket, and log the run as its own `test-run` line.
rm -f "$ticket"
write_log 0 2>/dev/null
[ -n "$errlog" ] && : > "$errlog"
sub=test-run; queued_at="$(date '+%F %T')"; all_args="${run_args[*]}"
t_q=$(ms); last=$t_q; reason=""; w_queue=0; w_busy=0; w_freeze=0
t_run=$t_q
CARGO_ALIAS_TEST_RUN=test run_cargo "${run_args[@]}"
exit $rc
