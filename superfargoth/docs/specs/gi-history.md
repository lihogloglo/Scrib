# Global illumination: as-built history

Archived from [gi.md](gi.md) "Spike (first slice, ROADMAP 4 \"GI\")" (2026-11-02, gi-ambient): the slice 1 / 1b / 1c
notes and spike checks 1-3, verbatim. Current state lives in `gi.md`.

## Spike: slices 1, 1b and 1c (2026-11-01)

**Slice 1 as built (2026-11-01, gi-spike; verification incomplete, see below).** `sf_gi` (grid indexing, Fibonacci
directions, per-frame Shoemake rotation, round-robin window, ambient cube from samples + hysteresis blend, Bevy atlas
texel; 10 exact tests) and `sf_gi_bevy` (`GiPlugin`, `GiSupport`, `GiVolume`; `render.rs` + `probe_update.wesl`: Solari
scene at group 0, ours at group 1, one 64-lane workgroup per probe, sun with a shadow ray, emissive, nearest-probe
second bounce, sky on miss; history in a storage buffer, `Rgba16Float` 3D texture through stock `IrradianceVolume`;
span `gi/probe_update`, log `gi: probe_update ...` and `gi: <status>`). Note: Bevy 0.20's `irradiance_volume.wesl`
reads the negative side at `t + Ry`, the opposite of its module docs; `sf_gi::atlas_texel` follows the shader.
Mirroring: `sf_sky_bevy::GiActive` (set while a volume exists) extends `path.rs` `mirror_meshes` to the raster path,
so `sf_gi_bevy` turns on `sf_sky_bevy/solari` (every game build now compiles `bevy_solari`; since slice 1b the
Solari plugins stay out of headless apps without a render app, so `sf_sky_bevy`'s unit tests run under both
configurations). Game:
`--off gi`, High/Ultra raster, interiors only, one static 16x8x16 volume at the start camera; `GiPlugin` goes before
`SkyPlugin` so the sky does not add `SolariLightingPlugin`; `RenderDiagnosticsPlugin` added when GI is on. The
`IrradianceVolume` is attached only once every probe was traced (`GiTraced`): an untraced volume never darkens a room.
- Check 1 (forward survives): yes so far. Census office ran ~25 s with `RaytracingScenePlugin` alone on the raster
  path: no panic, no wgpu validation error, the GI pipeline compiled (`gi: waiting: Solari scene empty`).
- Check 2 (albedo at hits): **blocked**. 8254 meshes mirrored, 0 Solari-compatible among those still readable, 8006
  render-world-only: the runtime's static meshes (cooked, compressed, extra attributes) have no `RayTracingMesh`
  variants (only the `view.rs` viewer path builds them), so Solari has no BLAS, no bind group, and the pass never
  dispatches. Slice 1b (built 2026-11-01, gi-meshes; **game run pending**) gives streamed interiors ray-tracing
  variants and scopes mirroring to the active worldspace ("Scene and lights"). Slice 1c run: 682 of 833 mirrored
  meshes have a BLAS (check 3); whether hits give the right albedo is the owner's visual check (GI-1).
- Check 3 (cost): **slice 1c run (2026-11-01, gi-dispatch, dev build, 1280x720 window, RTX 4060 laptop):**
  `gi/probe_update` 0.024-0.025 ms typical (512 probes x 64 rays = 32768 rays/frame, no sun indoors so no shadow
  rays; single-frame spikes 0.1-0.47 ms over 40 s). Verbatim: `gi: probe_update 0.102 ms, 512 probes, 32768 rays;
  ray-traced meshes 833 mirrored, 682 accepted (BLAS), rejected 151 (not enabled 0, attributes 151, topology 0, not
  u32 indices 0), unclassified 0; Solari lights 0` and `gi: volume traced, shown as an irradiance volume` (~1.7 s
  after start). `--off gi` run: no `gi:` line. VRAM (BLAS/TLAS, variants, atlas): not in the log, not measured.
- **Slice 1c (gi-dispatch) root causes.** (a) Solari 0.20 builds its scene bind group only with at least one light
  (`prepare_raytracing_scene_bind_group`), and interiors have none: the sun/moon get `RenderLayers::none()` indoors
  (`mw_exterior_bevy` `light_layers`, SH-16), so no camera sees them, their `ViewVisibility` is false and Bevy does not
  extract them; Solari 0.20 has no point lights. Fix: own `bevy_solari` patch (`superfargoth/vendor/README.md`): the bind
  group exists whenever the TLAS has instances; Solari's lighting/path tracer read `lit_bind_group()` and still skip
  lightless scenes. Our shader reads no light list; a missing environment map binds Solari's fallback cube at
  intensity 0 (`sample_environment_map_light` returns 0). (b) The status counter read main-world `Assets<Mesh>`
  (render-world-only data showed as "unknown"); it now classifies the extracted meshes in the render world with
  Solari's predicate (`MeshVerdicts`) and reads `RaytracingSceneBindings::{instance_count, light_count}`. Slice 1b's
  variants do arrive (682 accepted); the 151 rejections (attribute set) are meshes mirrored without a variant, growing
  as actors load (likely NPC/creature parts: skinned, outside the streamed-static path; not identified).
