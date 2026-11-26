#!/usr/bin/env bash
# Runs the pre-PR gate in CI for your slice (build-speed BS3, 2026-11-26; CONTRIBUTING.md "Building").
# Pushes a snapshot, HEAD plus the given paths as they are on disk, to branch ci/<name>, and touches nothing
# else: not the working tree, not the shared index, not main. Other sessions' changes are left out.
#
#   bash bevyrrowind/tools/ci_push.sh <name> <path>...   # then: gh run watch, gh run view --log-failed
#   bash bevyrrowind/tools/ci_push.sh --done <name>      # delete the ci/<name> branch once merged or dropped
#
# From a worktree (tools/worktree.sh) you can instead commit there and push wt/<name>: CI runs on wt/** too.
set -e
if [ "$1" = --done ]; then git push origin --delete "ci/$2"; exit; fi
[ -n "$2" ] || { sed -n '5,6p' "$0"; exit 2; }
name="$1"; shift
idx="$(mktemp)"
trap 'rm -f "$idx"' EXIT
GIT_INDEX_FILE="$idx" git read-tree HEAD
GIT_INDEX_FILE="$idx" git add -A -- "$@"
tree="$(GIT_INDEX_FILE="$idx" git write-tree)"
[ "$tree" != "$(git rev-parse 'HEAD^{tree}')" ] || echo "ci_push: no changes in those paths; testing HEAD"
commit="$(git commit-tree "$tree" -p HEAD -m "ci: $name (snapshot of HEAD + $*)")"
git push --force origin "$commit:refs/heads/ci/$name"
echo "ci_push: ci/$name = $commit. Follow: gh run watch \$(gh run list -b ci/$name -L1 --json databaseId -q '.[0].databaseId')"
