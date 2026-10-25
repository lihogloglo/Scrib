# Vendored third-party crates

Patched copies of crates.io crates, wired with `[patch.crates-io]` in **both**
`superfargoth/Cargo.toml` and `bevyrrowind/Cargo.toml`, excluded from the superfargoth
workspace (not ours to lint or test). Every changed hunk carries a `superfargoth patch`
comment; `grep -rn "superfargoth patch" vendor/<crate>` lists them.

| Crate(s) | Base | Patch | Drop when |
|---|---|---|---|
| `big_space` | crates.io release | dirty-tree pruning in low-precision propagation (`docs/research/perf/README.md` item 2) | upstream has it |
| `bevy_core_pipeline`, `bevy_pbr`, `bevy_render` | 0.20.0 (crates.io) | bevyengine/bevy [#26149](https://github.com/bevyengine/bevy/pull/26149) + [#26161](https://github.com/bevyengine/bevy/pull/26161) (`patches/bevy-26149-26161.diff`, both PRs' stacked diff at head `4e718df0`) | the Bevy release that ships both PRs |
| `dlss_wgpu` | 6.0.0 (crates.io) | NVIDIA-free facade: same API, NGX calls moved to the runtime-loaded `plugins/sf_dlss` DLL through `src/abi.rs` (GPL game exe holds no NVIDIA code) | never while the game is GPL; re-port on each dlss_wgpu bump that Bevy needs |
| `bevy_pbr` (own, on top of the row above) | same | `superfargoth patch (own: shadow early-out)` + `(own: lamp shadow filter)` (`patches/superfargoth-shadow-early-out-lamp-filter.diff`, 44+/9- lines, shadow reset B2) | early-out: Bevy skips the point/spot shadow fetch where the light contributes nothing; filter: Bevy gets a per-light-type (or per-light) shadow filter |

## The Bevy shadow patch (#26149, #26161)

- **#26149** (`Core3dShadows`): point and spot shadow views run a slim schedule (GPU mesh
  preprocessing + the two shadow passes) instead of the whole `Core3d` camera schedule
  per cube face (bevy issue #25576).
- **#26161** (`SkipRootViewRendering`): per point/spot light, a hash of its visible casters,
  its view/projection and the queued shadow-phase entity count; a shadow view whose inputs
  are unchanged (no caster moved, deforms, or changed mesh; no pipeline still compiling)
  is skipped by the camera driver and keeps last frame's map. Directional lights are not
  covered. `bevy_render` only gains `BinnedRenderPhase::entity_count`.

The diff applied cleanly to the 0.20.0 sources (`patch -p1` from `vendor/`, two hunks
offset by 9 lines).

## Our own Bevy patches (shadow reset B2)

Not upstream PRs; ours, marked `superfargoth patch (own: …)`, diffed against the PR-patched sources in
`patches/superfargoth-shadow-early-out-lamp-filter.diff` (`patch -p1` from `vendor/`, after the PR diff).

- **Shadow early-out** (`pbr_functions.wesl`): the point and spot loops evaluate the light first and fetch the
  shadow (and lamp contact shadows) only if `any(light_contrib != 0)`. Range falloff, N·L and the spot cone
  saturate to exactly 0, so skipped fragments are the ones the shadow would have been multiplied by zero: exact.
  Transmission (`STANDARD_MATERIAL_DIFFUSE_TRANSMISSION`) still fetches unconditionally.
- **Lamp shadow filter**: camera marker `PointSpotShadowsHardware2x2` (`render/mesh.rs`, extracted like other
  camera settings) sets `MeshPipelineKey::POINT_SPOT_SHADOWS_HARDWARE_2X2` → shader def
  `POINT_SPOT_SHADOW_FILTER_HARDWARE_2X2`: point cubes (`sample_shadow_cubemap`) and spot maps
  (`fetch_spot_shadow`) take Bevy's 1-tap 2x2 hardware PCF; directional keeps the camera's
  `ShadowFilteringMethod`. PCSS lights (`soft_shadow_size`) unchanged. Forward/prepass only: the deferred
  lighting pipeline and meshlets don't read the key. Volumetric fog's point/spot lookups share the functions.
- On a Bevy upgrade, re-apply this diff after the PR diff (or drop each hunk per the "Drop when" column).

## Rebase or drop on a Bevy upgrade

1. Check both PRs: merged and released in the new version? Delete the three
   `vendor/bevy_*` folders and their `[patch.crates-io]` lines in both workspaces. Done.
2. Otherwise copy the new version's crates from the cargo registry
   (`~/.cargo/registry/src/index.crates.io-*/bevy_{core_pipeline,pbr,render}-<ver>`) over
   the folders (remove `Cargo.lock`, `.cargo-ok`), fetch the PRs' current diff
   (`gh pr diff 26161 -R bevyengine/bevy`, it includes #26149), strip the `crates/` prefix
   (`sed 's#\(a\|b\)/crates/#\1/#g'`), `patch -p1` it from `vendor/`, re-add the
   `superfargoth patch (#…)` comments, refresh `patches/`, and bump the versions above.
3. Every patched crate must match the version the rest of Bevy resolves to, or cargo
   silently ignores the patch (`cargo tree -i bevy_pbr` shows the path source when it took).
