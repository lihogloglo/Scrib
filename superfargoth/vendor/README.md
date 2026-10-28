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
| `bevy_core_pipeline`, `bevy_pbr` | same | bevyengine/bevy [#26235](https://github.com/bevyengine/bevy/pull/26235), [#26236](https://github.com/bevyengine/bevy/pull/26236), [#26237](https://github.com/bevyengine/bevy/pull/26237) (LT-4 DLSS cluster lookup, transmission UV, and OIT indexing with `MainPassResolutionOverride`) | upstream Bevy release |
| `bevy_render`, `bevy_pbr` | same | bevyengine/bevy [#26321](https://github.com/bevyengine/bevy/pull/26321) (`ExtendedMaterial` bindless fallback when extension does not support bindless) | upstream Bevy release |
| `bevy_render`, `bevy_pbr` | same | bevyengine/bevy [#26095](https://github.com/bevyengine/bevy/pull/26095) (clear out stale bin unpacking & uniform allocation jobs and bind groups) | upstream Bevy release |
| `bevy_pbr` (own) | same | `superfargoth patch (own, SH-20)`: `ShadowMoverRefresh` (redraw mover/animated shadow faces per frame outside LRU budget, `--off movershadows` A/B) | upstream gets dynamic caster refresh |
| `bevy_pbr` (own) | same | `superfargoth patch (own, H2)`: World-Unit Directional PCSS (`shadow_sampling.wesl`, `shadows.wesl`: world-meter penumbra scale and analytic receiver slope across cascades) | upstream gets cascade-invariant world-unit PCSS |

## The Bevy shadow patch (#26149, #26161)

- **#26149** (`Core3dShadows`): point and spot shadow views run a slim schedule (GPU mesh
  preprocessing + the two shadow passes) instead of the whole `Core3d` camera schedule
  per cube face (bevy issue #25576).
- **#26161** (`SkipRootViewRendering`): per point/spot light, a hash of its visible casters,
  its view/projection and the queued shadow-phase entity count; a shadow view whose inputs
  are unchanged (no caster moved, deforms, or changed mesh; no pipeline still compiling)
  is skipped by the camera driver and keeps last frame's map. Directional lights are not
  covered. `bevy_render` only gains `BinnedRenderPhase::entity_count`.

## Upstream PR fixes

- **#26235, #26236, #26237** (`LT-4` DLSS viewport indexing): when `MainPassResolutionOverride` (DLSS)
  is active, fragment shading passes rasterize at `main_pass_viewport` rather than full `viewport`.
  Without these patches, clustered forward light assignment (`clustered_forward.wesl`), transmissive
  refraction sampling (`transmission.wesl`), and order-independent transparency (`oit/draw.wesl`,
  `oit_resolve.wesl`) sample clusters and buffers scaled to the output resolution, producing blocky light
  dropout and misaligned screen-space lookups.
- **#26321** (`ExtendedMaterial` bindless fallback): combines `AsBindGroup::bindless_supported` across base
  and extension materials. If either material does not support bindless, the combined material forces non-bindless
  layout entries and bind group construction.
- **#26095** (GPU preprocessing unpacking job clear): clears `SceneUnpackingBuffers` unpacking jobs and
  `BinUnpackingBindGroups` / `UniformAllocationBindGroups` every frame so that stale metadata from prior frames
  or dead views does not cause dangling compute dispatches.

## Our own Bevy patches

### Shadow reset B2 (early-out & lamp filter)
Diffed against the PR-patched sources in `patches/superfargoth-shadow-early-out-lamp-filter.diff`:
- **Shadow early-out** (`pbr_functions.wesl`): point and spot loops evaluate the light first and fetch the
  shadow (and lamp contact shadows) only if `any(light_contrib != 0)`. Range falloff, N·L and the spot cone
  saturate to exactly 0, so skipped fragments are the ones the shadow would have been multiplied by zero: exact.
- **Lamp shadow filter**: camera marker `PointSpotShadowsHardware2x2` (`render/mesh.rs`, extracted like other
  camera settings) sets `MeshPipelineKey::POINT_SPOT_SHADOWS_HARDWARE_2X2` → shader def
  `POINT_SPOT_SHADOW_FILTER_HARDWARE_2X2`: point cubes (`sample_shadow_cubemap`) and spot maps
  (`fetch_spot_shadow`) take Bevy's 1-tap 2x2 hardware PCF; directional keeps the camera's
  `ShadowFilteringMethod`. PCSS lights (`soft_shadow_size`) unchanged. Forward/prepass only: the deferred
  lighting pipeline and meshlets don't read the key. Volumetric fog's point/spot lookups share the functions.

### SH-20 Mover shadow refresh (`render/light.rs`)
- Animated characters moving under lamp lights stepped under the B3 rule (which quantized skinned roots on a
  ≥ 20 cm grid and capped refreshes to 2 dirty faces per frame).
- `ShadowMoverRefresh`: split face hashing into `(members, motion)`. When casters are unchanged meshes but
  their light-relative translation or joint poses / morph weights changed (`motion`), the face redraws the same
  frame outside the LRU budget. Set changes (streaming, LOD swaps) remain budgeted. Main-world resource
  `ShadowMoverRefresh { per_frame }` (default true; false = B3 rule, toggled via `--off movershadows`).

### H2 World-Unit Directional PCSS (`render/shadow_sampling.wesl`, `shadows.wesl`)
- Stock Bevy directional PCSS computes blocker depth and penumbra filter radius in orthographic cascade NDC,
  making penumbra width depend on each cascade's bounding box and receiver NDC depth (jumping visibly across
  cascade splits).
- Adapted from OpenMW Clockwork `files/shaders/lib/light/sunsoft.glsl` @ `c5e5a9a6` (GPL-3.0, Thomas Portal),
  plus rotated blocker kernel from `bevyengine/bevy#22272`.
- Converts blocker height into world meters via `clip_from_world`'s Z row (`per_z = length(row_z)`), computes
  analytic receiver-plane depth slope to avoid derivatives across non-uniform cascade branches, and applies
  an angular penumbra spread in world meters across all cascade splits.

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
