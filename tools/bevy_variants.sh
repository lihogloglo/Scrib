#!/usr/bin/env bash
# bevy_variants.sh: how many Bevy builds do these cargo selections need? (build-perf.md "Feature unification")
#
#   bash tools/bevy_variants.sh                       # the default selections below
#   bash tools/bevy_variants.sh "-p sf_ui_bevy" "-p bevyrrowind" ...
#
# Cargo resolves features per command, so two selections that turn on different features of a crate in Bevy's
# dependency closure compile that crate (and all of Bevy above it) twice. For each selection this takes
# `cargo tree -f '{p}|{f}'`, keeps the crates of `bevy_internal`'s closure (name + version), and groups
# selections that agree on every such crate they both build. Run it after adding a crate, a dependency or a
# feature: the plain selections must stay in one group. Default selections: `cargo test -p` of a framework and a
# game plugin crate, the app build, the workspace, and `cargo dev` (its own group by design, build-perf.md).
# `cargo tree` only reads manifests and the lock file (no compile), so it doesn't take a cargo_turn.sh slot.
set -u
cd "$(dirname "$0")/.."
if [ $# -eq 0 ]; then
    set -- "-p sf_ui_bevy" "-p mw_exterior_bevy" "-p bevyrrowind" "--workspace" \
           "-p bevyrrowind --features dev"
fi
out="$(mktemp -d)"
trap 'rm -rf "$out"' EXIT

# "name version|features" per line, without cargo tree's markers and path suffixes.
norm() { sed -E 's/ \(\*\)$//; s/ \(proc-macro\)//; s/ \([A-Za-z]:[^)]*\)//; s/ \(\/[^)]*\)//' | sort -u; }

cargo tree -q -p bevy_internal -e normal --prefix none -f '{p}' 2>/dev/null | norm > "$out/closure"
[ -s "$out/closure" ] || { echo "bevy_variants: cargo tree failed (run from the repo)" >&2; exit 2; }

i=0
for sel in "$@"; do
    i=$((i + 1))
    # shellcheck disable=SC2086 # $sel is a list of cargo args on purpose
    cargo tree -q -e normal,dev $sel --prefix none -f '{p}|{f}' 2>"$out/err$i" | norm \
        | awk -F'|' 'NR == FNR { keep[$0] = 1; next } $1 in keep' "$out/closure" - > "$out/sel$i"
    [ -s "$out/sel$i" ] || { echo "bevy_variants: '$sel' failed: $(head -3 "$out/err$i")" >&2; exit 2; }
done

# Lines where a and b both build a crate with different features ("<" = a, ">" = b).
conflicts() {
    awk -F'|' -v side="$3" 'NR == FNR { a[$1] = $2; next } ($1 in a) && a[$1] != $2 { print "      " $1 ": [" side "] " a[$1] "  vs  " $2 }' "$1" "$2"
}

groups=()   # representative selection index per group
for j in $(seq 1 $#); do
    g=0
    for k in "${!groups[@]}"; do
        [ -z "$(conflicts "$out/sel${groups[$k]}" "$out/sel$j" x)" ] && { g=$((k + 1)); break; }
    done
    if [ $g = 0 ]; then groups+=("$j"); g=${#groups[@]}; fi
    echo "$g $j" >> "$out/member"
done

echo "bevy_variants: ${#groups[@]} Bevy build(s) for $# selection(s)"
for k in "${!groups[@]}"; do
    n=$((k + 1))
    for j in $(awk -v g="$n" '$1 == g {print $2}' "$out/member"); do
        eval "sel=\${$j}"
        echo "  [$n] $sel"
    done
    if [ $n -gt 1 ]; then
        echo "      vs [1] ($(conflicts "$out/sel${groups[0]}" "$out/sel${groups[$k]}" 1 | wc -l) crates differ; first 8):"
        conflicts "$out/sel${groups[0]}" "$out/sel${groups[$k]}" 1 | cut -c1-200 | head -8
    fi
done
