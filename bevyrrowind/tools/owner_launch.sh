#!/usr/bin/env bash
# Launches a scene that needs the owner's eyes (CONTRIBUTING.md "Tests and visual checks", owner 2026-11-11):
# waits until the machine is free, copies the current dev build into a private folder (so rebuilds can't
# replace or lock it, and no agent closes it), then starts it fullscreen, detached.
#
#   bash bevyrrowind/tools/owner_launch.sh scene level-flat-slopes
#   bash bevyrrowind/tools/owner_launch.sh view "Seyda Neen, Census and Excise Office" --hour 12
#
# Free means all of:
#   - no build from any project (no rustc.exe; no cargo.exe other than `cargo run`/`cargo dev`, as cargo_turn.sh);
#   - no game window already open (bevyrrowind.exe from any folder): one owner window at a time;
#   - the owner isn't in something critical: no fullscreen window in the foreground (game, video, slides),
#     no call or recording app running a session (Zoom meeting, OBS) and none of Teams/Webex/Skype focused.
# Prints what it waits for, once per reason. `OWNER_LAUNCH_NO_WAIT=1` checks once and exits 3 if busy.
# The dev build must exist (`cargo build -p bevyrrowind --features dev`); this script never builds.
set -u
TARGET=/c/cargo-target/scrib/debug
ROOT="$(cd "$(dirname "$0")/.." && pwd)"   # bevyrrowind/, the game's working dir

build_busy() {
    tasklist 2>/dev/null | grep -qiE '^rustc\.exe' && return 0
    tasklist 2>/dev/null | grep -qiE '^cargo\.exe' || return 1
    powershell -NoProfile -Command "Get-CimInstance Win32_Process -Filter \"Name='cargo.exe'\" | ForEach-Object { \$_.CommandLine }" 2>/dev/null \
        | tr -d '\r' | sed -E 's/^("[^"]*"|[^ ]*)//' \
        | awk '{ s = ""; for (i = 1; i <= NF; i++) if ($i !~ /^[-+]/) { s = $i; break } print s }' \
        | grep -qvxE 'run|dev'
}
# A game window = a bevyrrowind.exe that isn't a headless subcommand (an agent's `bake`, `info`, ...).
game_open() {
    tasklist 2>/dev/null | grep -qiE '^bevyrrowind\.exe' || return 1
    powershell -NoProfile -Command "Get-CimInstance Win32_Process -Filter \"Name='bevyrrowind.exe'\" | ForEach-Object { \$_.CommandLine }" 2>/dev/null \
        | tr -d '\r' | grep -qvE '[[:space:]](bake|info|mods|census|clip-preview --export)([[:space:]]|$)'
}
# Prints a reason when the owner looks busy, nothing otherwise.
owner_busy() {
    powershell -NoProfile -Command '
Add-Type @"
using System; using System.Runtime.InteropServices;
public struct R { public int L, T, Rt, B; }
public static class W {
  [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out R r);
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint p);
}
"@
Add-Type -AssemblyName System.Windows.Forms
$h = [W]::GetForegroundWindow(); $r = New-Object R; [void][W]::GetWindowRect($h, [ref]$r)
$p = 0; [void][W]::GetWindowThreadProcessId($h, [ref]$p); $name = (Get-Process -Id $p -ErrorAction SilentlyContinue).ProcessName
$s = [System.Windows.Forms.Screen]::FromHandle($h).Bounds
$shell = @("explorer", "ShellExperienceHost", "SearchHost", "StartMenuExperienceHost", "LockApp")
if ($r.L -le $s.Left -and $r.T -le $s.Top -and $r.Rt -ge $s.Right -and $r.B -ge $s.Bottom -and $shell -notcontains $name) { "fullscreen $name in front"; exit }
if (Get-Process -Name CptHost, obs64, obs32 -ErrorAction SilentlyContinue) { "a meeting or recording is running"; exit }
if (@("Teams", "ms-teams", "Zoom", "CiscoCollabHost", "webexmta", "Skype") -contains $name) { "$name in front" }
' 2>/dev/null | tr -d '\r'
}

# Owner away (owner 2026-11-11): while <repo>/.sessions/OWNER-AWAY exists, nothing launches; park the visual check
# ("awaiting owner" entry in docs/visual-checks) instead. Exit 5.
away="$ROOT/../.sessions/OWNER-AWAY"
if [ -f "$away" ]; then
    echo "owner_launch: owner away, not launching. Park the check: $(head -1 "$away")"
    exit 5
fi

said=""
note() { [ "$said" = "$1" ] || { echo "owner_launch: waiting: $1"; said="$1"; }; }
while :; do
    if build_busy; then why="a build is running"
    elif game_open; then why="a game window is already open"
    else why="$(owner_busy)"; fi
    [ -z "$why" ] && break
    [ "${OWNER_LAUNCH_NO_WAIT:-}" = 1 ] && { echo "owner_launch: busy: $why"; exit 3; }
    note "$why"; sleep 15
done

[ -f "$TARGET/bevyrrowind.exe" ] || { echo "owner_launch: no dev build at $TARGET"; exit 2; }
dir="$TEMP/bw-owner-$(date +%Y-%m-%d-%H%M)"
mkdir -p "$dir"
cp "$TARGET/bevyrrowind.exe" "$dir/" || exit 2
dylib="$(grep -ao 'bevy_dylib-[0-9a-f]\{16\}' "$TARGET/bevyrrowind.exe" | head -1)"
[ -n "$dylib" ] && cp "$TARGET/deps/$dylib.dll" "$dir/"
cp "$(rustc --print sysroot)"/bin/std-*.dll "$dir/" 2>/dev/null
[ -d "$TARGET/dlss" ] && cp -r "$TARGET/dlss" "$dir/"
log="$dir/last.log"
# Bake first, with the very exe the owner will run (2026-11-11: a scene showed only capsules because the mesh cache
# on disk was an older .sfmesh format than the build). A no-op when the store is current (~2 s); a stale store is
# rebaked, which the scene needs anyway. A failed bake means no launch. The caller's env passes through (e.g. a
# temporary BEVYRROWIND_BAKE_SKIP_NAV).
echo "owner_launch: checking the bake (no-op when current)"
if ! ( cd "$ROOT" && "$dir/bevyrrowind.exe" bake ) > "$dir/bake.log" 2>&1; then
    echo "owner_launch: bake failed, not launching (log $(cygpath -w "$dir/bake.log"))"
    tail -5 "$dir/bake.log"
    exit 4
fi
# The command the owner can rerun, then the launch itself, detached so this script returns.
args=""; for a in "$@"; do args="$args \"$a\""; done
printf '@echo off\r\ncd /d "%s"\r\n"%%~dp0bevyrrowind.exe"%s > "%%~dp0last.log" 2>&1\r\n' "$(cygpath -w "$ROOT")" "$args" > "$dir/run.cmd"
cmd //c start "" //min "$(cygpath -w "$dir/run.cmd")"
echo "owner_launch: started $* from $(cygpath -w "$dir") (log $(cygpath -w "$log"))"
