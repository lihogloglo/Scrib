# Depth-only opaque casters without the material bind group (research, 2026-11-17)

Labels: **code read** = lines read; **grep only** = matched by grep, not read around. Paths are under
`superfargoth/vendor/bevy_pbr/src/` unless full. Bevy 0.20 vendored copy.

## 1. Verdict

- **The machinery already exists in our vendored copy (stock Bevy 0.20) and is switched off for StandardMaterial.** Nothing needs to be built; one predicate needs to change. (code read)
- Stock has: key bit `PREPASS_READS_MATERIAL`, a second draw function `DrawDepthOnlyPrepass` (empty group 3), an empty layout at group 3, and `material_bind_group_index: None` in the batch-set keys. All in place for shadows and `Opaque3dPrepass`.
- Why off: `StandardMaterial::prepass_fragment_shader()` returns a shader (`pbr_material.rs:1464`), and `prepass_reads_material()` (`material.rs:1903-1912`) is true whenever a prepass vertex or fragment shader override exists. So every StandardMaterial draw takes `DrawPrepass` and binds the slab.
- Size: about 15-30 lines in 2-3 files (`material.rs`, optionally `pbr_material.rs`, one test). No shader change, no key-struct change. Biggest unknown: where to store the per-material flag, because `MaterialProperties` lives in the unvendored `bevy_material` crate.
- Gain is shadow views and any prepass whose key is exactly `DEPTH_PREPASS`. A main prepass with `NormalPrepass` (the ledger says SSAO needs it) stays on the normal path.

## 2. Bound today

**Shadow view, opaque StandardMaterial** (code read)
- Specialised in `render/light.rs:3644-3690` (`specialize_shadows`). `is_depth_only_opaque` = no `MAY_DISCARD` and no `PREPASS_READS_MATERIAL` and not emulated unclipped depth (`:3660-3663`). If true the draw function is `ShadowsDepthOnlyDrawFunction`, else `ShadowsDrawFunction` (`:3664-3670`).
- The key bit comes from `precache::shadow_mesh_key` (`precache.rs:263-275`): `MAY_DISCARD` for Mask/Blend/Premult/Add/AlphaToCoverage, and `PREPASS_READS_MATERIAL` when `material.prepass_reads_material()`. For StandardMaterial that is always true, so the depth-only branch is never taken.
- Today therefore: `DrawPrepass` = SetItemPipeline, `SetPrepassViewBindGroup<0>`, `SetPrepassViewEmptyBindGroup<1>`, `SetMeshBindGroup<2>`, `SetMaterialBindGroup<3>`, DrawMesh (`prepass/mod.rs:1642-1649`). Registered for `Shadow` at `material.rs:408`.
- The pipeline layout (`prepass/mod.rs:433-470, 629`): group 0 prepass view layout, group 1 empty, group 2 mesh/skin/morph layout (inserted at index 2), group 3 either `material_layout` (the slab) or `empty_layout`. The empty branch needs `is_depth_only_opaque_prepass(key) && !emulate_unclipped_depth && !prepass_reads_material()` (`:457-467`). Today the last term is false.

**Main camera depth prepass** (code read)
- `Opaque3dPrepass` item draw function chosen at `prepass/mod.rs:1270-1290`: deferred, else depth-only when `is_depth_only_opaque_prepass(key)`, else `PrepassOpaqueDrawFunction`. `is_depth_only_opaque_prepass` (`:413-417`) requires the prepass bits to equal exactly `DEPTH_PREPASS` and neither `MAY_DISCARD` nor `PREPASS_READS_MATERIAL`. `ALL_PREPASS_BITS` includes NORMAL, DEFERRED, MOTION_VECTOR, MAY_DISCARD, READS_MATERIAL (`render/mesh.rs:3436-3442`). So a view with `NormalPrepass` or `MotionVectorPrepass` never goes depth-only (correct: those fragments read flags, the normal map, UV transform).
- Alpha-masked: `AlphaMask3dPrepass` keeps `DrawPrepass` (`prepass/mod.rs:1285-1290`, the queue at `:1505-1535` always uses `Some(group)`).

**Does the stage read group 3?**
- Vertex stage (`prepass/prepass.wesl`): grep for `material`/`MATERIAL` finds only a comment at line 201; the parts read (lines 1-150) use mesh bindings, morph, skin, view only. No material read. (code read for 1-150, grep only for the rest.)
- Fragment stage `render/pbr_prepass.wesl`: reads `flags` and `uv_transform` from the material unconditionally at the top (bindless: `mesh[...].material_and_lightmap_bind_group_slot`, `material_indices[slot]`, `material_array[...]`), then `prepass_sample_color_and_alpha_discard` (guarded `MAY_DISCARD`, `render/pbr_prepass_functions.wesl:45-80`, reads base_color, flags, alpha_cutoff, uv_transform, base colour texture via the bindless arrays), then the normal-map sample (guarded `NORMAL_PREPASS` and `STANDARD_MATERIAL_NORMAL_MAP`, lines about 60-120). Parallax: not read (matches the existing depth-key patch comment, `pbr_material.rs:1636-1640`). (code read, first ~120 lines)
- **A depth-only shadow pipeline has no fragment stage for opaque**: `targets` are cleared when all are None (`prepass/mod.rs:640-646`), and `fragment_required` is true only for targets, emulated unclipped depth, or `MAY_DISCARD` with a material prepass fragment shader (`:648-653`). Shadow keys have only `DEPTH_PREPASS`, so no fragment unless `emulate_unclipped_depth` (needs `!depth_clip_control_supported`, `light.rs:3652-3663`). (code read)

## 3. Keys and bins

- Shadow batch-set key `ShadowBatchSetKey { pipeline, draw_function, material_bind_group_index: Option<u32>, slabs }` (`render/light.rs:3845-3870`). Bin key `ShadowBinKey { asset_id }` (`:3818-3823`). The queue sets `material_bind_group_index = None` when `is_depth_only_opaque`, otherwise looks up the material and uses its group (`:3787-3807`); the draw function comes from the per-view specialisation cache (`:3759-3763`). So with the flag on, shadow items for all materials with the same pipeline and mesh slab collapse into one batch set. (code read)
- Prepass: `OpaqueNoLightmap3dBatchSetKey { draw_function, pipeline, material_bind_group_index, slabs }`, bin key `asset_id` (`prepass/mod.rs:1461-1500`). For `Opaque3dPrepass` the index is `None` iff the draw function equals `PrepassOpaqueDepthOnlyDrawFunction` (`:1481-1492`); alpha-mask and both deferred phases always `Some(group)` (`:1469, 1513, 1529`). (code read)
- Depth-only draw function: `SetPrepassEmptyMaterialBindGroup<3>` sets the empty bind group at index 3 (`prepass/mod.rs:1622-1640, 1651-1658`). Registered for `Opaque3dPrepass` (`prepass/mod.rs:168`) and `Shadow` (`material.rs:409`); ids placed in each material's draw function list (`material.rs:1762-1800`). (code read)

## 4. Patch list

The change needed is only to make `prepass_reads_material()` false for StandardMaterial when the fragment is not needed. Safe because every use is already guarded by an exact-key test (`DEPTH_PREPASS` only, no `MAY_DISCARD`).

| file:line | function | change |
|---|---|---|
| `material.rs:1903-1912` | `MaterialPropertiesExt::prepass_reads_material` | return false when the material is marked "default pbr prepass, material-free in depth-only passes" |
| `material.rs:340-380` | `initialize_material_shaders::<M>` | push a marker entry (new label, e.g. `PrepassDepthOnlyIgnoresMaterial`) into the `shaders` list when `M::prepass_depth_only_ignores_material()` (new trait method, default false) |
| `material.rs:~230` | `Material` trait | add that defaulted method (next to `prepass_fragment_shader`) |
| `pbr_material.rs:~1464` | `impl Material for StandardMaterial` | override to `true` |
| `sf_world_bevy` tests | next to `depth_only_pipeline_key_ignores_normal_map_and_relief` | assert key bit absent for opaque StandardMaterial, present for Mask |

Alternative to the marker: vendor `bevy_material` and add a `bool` field to `MaterialProperties` (cleaner, but a new vendored crate; `MaterialProperties` is `bevy_material-0.20.0/src/lib.rs:53`, unvendored; `get_shader` not read). Alternative 2: compare the stored `PrepassFragmentShader` handle path with the embedded `pbr_prepass.wesl` (smaller, fragile). No change is needed in `light.rs`, `prepass/mod.rs`, the batch-set keys, the layout code or the draw functions. They all key off `prepass_reads_material()`. The precache functions (`precache.rs:229, 273`) pick the change up automatically. (code read)

## 5. Must stay on the normal path

- Alpha mask, Blend, Premultiplied, Add, A2C-off-MSAA: `MAY_DISCARD` (`precache.rs:263-272`, `material.rs:715-725`). Excluded by `is_depth_only_opaque*`. (code read)
- Normal prepass, motion-vector prepass, deferred prepass: excluded by the exact-equality in `is_depth_only_opaque_prepass` (`prepass/mod.rs:413-417`) and the deferred branch first at `:1272`. (code read)
- A2C with MSAA on has `BLEND_ALPHA_TO_COVERAGE`, not `MAY_DISCARD`, in the prepass key (`material.rs:722-724`): the prepass would go depth-only and skip the alpha test. Stock Bevy behaves the same for materials without prepass shader overrides. We run MSAA off with DLSS; note it only. (code read)
- `ExtendedMaterial` and any material overriding prepass vertex/fragment shaders: default trait method false means unchanged behaviour (still true through the shader check). Our own non-Standard materials: unaffected unless they opt in. ExtendedMaterial's `prepass_*_shader` forwarding was not read. (grep only)
- Non-bindless fallback: no difference in this path; the depth-only draw never touches the layout. (code read by inference, not tested)
- Unclipped-depth emulation (`!depth_clip_control_supported`): normal path (fragment writes depth). (code read)

## 6. Risks, unknowns, tests

1. **Group 3 is declared, not omitted.** The layout keeps four groups with an empty layout at index 3 and the draw sets an empty bind group there (`prepass/mod.rs:463, 1656`). No gap question; wgpu needs a bind group set for every declared layout, and the empty one is set. The slab is declared nowhere in that pipeline, so wgpu-core does not walk its ~2,700 views. (code read; wgpu internals not read)
2. **Never exercised for StandardMaterial in our tree.** The path has been dead for our main material since vendoring; shadows for it have not run in this form. Test: sanity-only log check (no validation errors, shadow draw count unchanged), then owner visual check for missing or extra shadows, plus untraced interleaved A/B per CONTRIBUTING (expected gain not measured; the slab walk per shadow encoder is the cost named in the brief).
3. **Pipeline catalog / precache.** The key bit disappears from StandardMaterial keys (`precache.rs:229, 273`), so baked pipeline catalog entries (W4) built with the bit set will miss once and respecialise; a re-bake is likely needed. How the catalog stores keys: not read.
4. **Depth-only batching expectations.** Shadow batch sets should drop (material group no longer splits them, `light.rs:3787`). Pipelines stay split by vertex layout (tangent vs plain, ledger "fix B2") and by depth-bias/cull bits that `prepass_bind_group_data` keeps (`pbr_material.rs:1638-1648`). (code read)
5. **Shadow-view skip patch** (`light.rs:1618`, `skip_unchanged_shadow_views`): not read beyond its doc; it removes views, does not touch items, so no interaction expected. Not verified.
6. **SURFACE_WEATHER prepass bits** (`prepass/mod.rs:429-446`, key bit `render/mesh.rs:3434`) add bindings in group 0, not 3. A key with SURFACE_WEATHER and DEPTH_PREPASS only: the `PREPASS_READS_MATERIAL`-style test compares `ALL_PREPASS_BITS` only, so it stays depth-only; whether the surface-weather fragment path (`SURFACE_WEATHER_PREPASS`) needs a fragment: not read. If it does, `fragment_required` would be false and it would be skipped silently: check.
7. **Opaque + "unlit" or double-sided flags:** depth-only ignores material `flags`; culling comes from the pipeline key, which keeps cull bits (`pbr_material.rs:1646-1648`). Double-sided materials are specialised by cull bit, not by the shader. (code read)

## 7. What binds the slab per exterior frame (grep only unless noted)

`SetMaterialBindGroup` exists only in `DrawMaterial` (`material.rs:610-618`) and `DrawPrepass` (`prepass/mod.rs:1642-1649`), plus the generic impl (`material.rs:624`). (code read)

| pass | draw type | binds slab today | after patch |
|---|---|---|---|
| main Opaque3d, AlphaMask3d, Transmissive3d, Transparent3d | `DrawMaterial` | yes | yes (needs textures) |
| Opaque3dPrepass, key exactly depth | `DrawPrepass` | yes | no |
| Opaque3dPrepass with Normal or MotionVector | `DrawPrepass` | yes | yes |
| AlphaMask3dPrepass | `DrawPrepass` | yes | yes |
| deferred opaque/alpha-mask | `DrawPrepass` | yes | yes (deferred not used in our exterior: not read) |
| Shadow, 3 sun cascades | `DrawPrepass` | yes | no for opaque, yes for masked |
| Shadow, lamp cube faces (up to 42) | `DrawPrepass` | yes | no for opaque, yes for masked |
| spot lights | `DrawPrepass` | yes | no for opaque |

Per-frame counts of passes: from the brief (3 + up to 42 shadow views), not re-derived. The share removed is every shadow view (the bulk of encoders) plus the depth-only main prepass if it runs without Normal/MotionVector.
