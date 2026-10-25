#!/usr/bin/env bash
# Builds the sf_dlss plugin and assembles the separately distributed DLSS package:
#
#   <out>/dlss/sf_dlss.dll             our plugin (MIT OR Apache-2.0 source; links NVIDIA's NGX SDK)
#   <out>/dlss/nvngx_dlss.dll          NVIDIA DLSS Super Resolution runtime
#   <out>/dlss/nvngx_dlssd.dll         NVIDIA DLSS Ray Reconstruction runtime
#   <out>/dlss/NVIDIA-DLSS-LICENSE.txt NVIDIA's licence (sdk/dlss/LICENSE.txt)
#   <out>/dlss/README.txt              what this is, where it goes
#
# Players drop the `dlss/` folder next to the game exe. Never part of a game release.
#
#   bash build-package.sh [out-dir]     # default: superfargoth/target/sf_dlss/package
#
# Runs cargo through bevyrrowind/tools/cargo_turn.sh (machine-wide build slot, BelowNormal).
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../../.." && pwd)"          # Bevyrrowind workspace folder
sdk="${DLSS_SDK:-$root/sdk/dlss}"
out="${1:-$root/superfargoth/target/sf_dlss/package}"

(cd "$here" && bash "$root/bevyrrowind/tools/cargo_turn.sh" build --release -j "${JOBS:-4}")

dll="$root/superfargoth/target/sf_dlss/release/sf_dlss.dll"
runtime="$sdk/lib/Windows_x86_64/rel"
for f in "$dll" "$runtime/nvngx_dlss.dll" "$runtime/nvngx_dlssd.dll" "$sdk/LICENSE.txt"; do
    [ -f "$f" ] || { echo "missing $f (bash $root/sdk/fetch.sh?)" >&2; exit 1; }
done

mkdir -p "$out/dlss"
cp "$dll" "$runtime/nvngx_dlss.dll" "$runtime/nvngx_dlssd.dll" "$out/dlss/"
cp "$sdk/LICENSE.txt" "$out/dlss/NVIDIA-DLSS-LICENSE.txt"
cat > "$out/dlss/README.txt" <<'EOF'
Optional NVIDIA DLSS support package.

Copy this "dlss" folder next to the game executable. Without it the game runs normally
and reports DLSS as unavailable.

sf_dlss.dll is built from superfargoth/plugins/sf_dlss (source: MIT OR Apache-2.0) and
statically links NVIDIA's NGX SDK. nvngx_dlss.dll and nvngx_dlssd.dll are NVIDIA's DLSS
runtimes. This package is distributed separately from the game, under NVIDIA's terms:
see NVIDIA-DLSS-LICENSE.txt.
EOF
echo "DLSS package: $out/dlss"
ls -l "$out/dlss"
