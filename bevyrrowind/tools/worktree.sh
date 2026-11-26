#!/usr/bin/env bash
# Git worktrees for parallel sessions (build-speed BS2b, 2026-11-26; CONTRIBUTING.md "Parallel sessions").
# A worktree is a second checkout on its own branch, so one session's half-done edit never breaks another's build.
#
#   bash bevyrrowind/tools/worktree.sh new <name> [base]   # D:\Gamedev\scrib-wt\<name> on branch wt/<name> (base: HEAD)
#   bash bevyrrowind/tools/worktree.sh init [dir]          # give an existing worktree (e.g. an Agent-tool one) its target
#   bash bevyrrowind/tools/worktree.sh list
#   bash bevyrrowind/tools/worktree.sh rm <name|dir>       # removes the checkout and its build folder; keeps the branch
#
# Each worktree needs its OWN target folder. Cargo names a path crate's artifacts by its path relative to the
# workspace root and checks freshness by mtime, so two checkouts sharing one target would reuse each other's
# builds of the same crate. The build folder is C:\cargo-target\scrib-wt\<name> (D: is nearly full), reached
# through `target` junctions like the main checkout's. Its first build compiles all of Bevy (~9 min, in the queue)
# and it grows to tens of GB: use a worktree for long or risky slices, `rm` it when merged.
# Builds still go through tools/cargo_turn.sh, which uses the main checkout's queue from any worktree.
set -e
main="$(dirname "$(git rev-parse --path-format=absolute --git-common-dir)")"
wt_root="${SCRIB_WT_ROOT:-D:/Gamedev/scrib-wt}"
tgt_root="${SCRIB_WT_TARGETS:-C:/cargo-target/scrib-wt}"

junction() { # <link> <target>: a directory junction (no admin needed), never a copy
    [ -e "$1" ] && return 0
    MSYS2_ARG_CONV_EXCL="*" cmd /c mklink /J "$(cygpath -w "$1")" "$(cygpath -w "$2")" > /dev/null
}
unjunction() { # remove only the link, never what it points to (CLAUDE.local.md: never rm -rf a junction)
    [ -e "$1" ] || [ -L "$1" ] || return 0
    MSYS2_ARG_CONV_EXCL="*" cmd /c rmdir "$(cygpath -w "$1")"
}

init() {
    local dir name t
    dir="$(cd "${1:-.}" && git rev-parse --show-toplevel)"
    [ "$(cd "$dir" && pwd -W 2>/dev/null || pwd)" != "$(cd "$main" && pwd -W 2>/dev/null || pwd)" ] \
        || { echo "worktree: $dir is the main checkout" >&2; exit 1; }
    name="$(basename "$dir")"
    t="$tgt_root/$name"
    mkdir -p "$t"
    for link in target bevyrrowind/target superfargoth/target; do junction "$dir/$link" "$t"; done
    # Machine notes are untracked; the worktree's sessions need them too.
    [ -f "$main/CLAUDE.local.md" ] && [ ! -f "$dir/CLAUDE.local.md" ] && cp "$main/CLAUDE.local.md" "$dir/"
    echo "worktree: $dir builds into $t"
}

case "$1" in
    new)
        [ -n "$2" ] || { echo "usage: worktree.sh new <name> [base]" >&2; exit 2; }
        dir="$wt_root/$2"
        mkdir -p "$wt_root"
        git -C "$main" worktree add -b "wt/$2" "$dir" "${3:-HEAD}"
        init "$dir"
        echo "worktree: branch wt/$2. Work and build there; merge into main from the main checkout when done."
        ;;
    init) init "$2" ;;
    list) git -C "$main" worktree list ;;
    rm)
        [ -n "$2" ] || { echo "usage: worktree.sh rm <name|dir>" >&2; exit 2; }
        dir="$2"; [ -d "$dir" ] || dir="$wt_root/$2"
        dir="$(cd "$dir" && pwd)"; name="$(basename "$dir")"
        for link in target bevyrrowind/target superfargoth/target; do unjunction "$dir/$link"; done
        git -C "$main" worktree remove "$dir"
        rm -rf "${tgt_root:?}/$name"
        echo "worktree: removed $dir and its build folder; the branch is kept (git branch -d wt/$name when merged)"
        ;;
    *) sed -n '2,15p' "$0"; exit 2 ;;
esac
