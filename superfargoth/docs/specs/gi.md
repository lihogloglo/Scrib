# Global illumination (raster path)

> **Agent summary (2026-11-01).** Status: spec written, spike queued (ROADMAP 4 "GI"). Decision (owner 2026-11-01):
> dynamic ray-query probe GI is the High path on RT hardware; no bake; Solari path tracing stays Ultra. Research:
> `../research/gi-2026-11-01.md` (baked probes, rejected), `../research/gi-dynamic-2026-11-01.md` (this design).
> Open: everything in "Open questions"; the spike replaces every estimated number here.

## Goal

Indirect diffuse light and sky occlusion that follow the scene as it is *now*: a torch carried through a room, a
lamp a script toggles, the sun moving through the day, overcast weather dimming a courtyard. Interiors and exteriors,
on the raster path, inside the High target (1440p + DLSS Quality <= 11 ms on an RTX 4060 laptop). Nothing bakes.
Specular stays the sky environment map plus specular occlusion from the same probes (reflection probes are out of
scope here). Ambient occlusion (GTAO) keeps acting on indirect light only; this system is what gives it something to
act on.

## Canonical technique

**Cascaded probe volumes updated by hardware ray queries**, the pattern Assassin's Creed Shadows (Anvil, SIGGRAPH
2025) and idTech 8 (Doom: The Dark Ages, Indiana Jones, "Fast as Hell", SIGGRAPH 2025) ship: DDGI-style probes
(Majercik et al. 2019) storing octahedral irradiance and visibility (mean and mean-square distance), a budgeted
number of probes re-traced per frame with a few dozen rays each, hits shaded with the scene's lights, misses reading
the sky, previous-frame probes supplying the second and later bounces. Published costs: idTech 8 1.4 ms async / 1.9 ms
serial on Series X at 1440p; AC Shadows ~1 ms for 512 probes per frame on an RTX 4070. Our estimate: 0.6-1.1 ms on the
4060 at the DLSS Quality internal size (the spike measures it).

Reuse order (CONTRIBUTING 2):
- **Bevy 0.20 gives:** `bevy_solari::scene::RaytracingScenePlugin` (BLAS/TLAS per `RaytracingMesh3d`, retained; a
  bind group with vertex/index/material arrays, `trace_ray`, `resolve_ray_hit_full`, the sky environment map and
  directional lights: `scene/bindings.wesl`, `sampling.wesl`); `IrradianceVolume` (ambient cubes sampled per pixel,
  `light_probe/irradiance_volume.wesl`, 8 per view, `LightProbe` transform); `RecordDiagnostics::pass_span` for GPU
  timings. `SolariLighting` (ReSTIR DI/GI, forced deferred, no point lights in 0.20) is **not** used: it is the Ultra
  tier (`sky-lighting.md`).
- **Crates:** none usable for Bevy 0.20 (research doc 1 section 4: 2D radiance cascades only, `bevy-hikari` dead,
  one deferred-only SSGI WIP). RTXGI's SDK is not MIT: written from the paper.
- **Ours:** the probe update compute pass, a world-space light list (point and spot lights are missing from Solari's
  scene), the probe sampling (visibility-weighted, in a vendored `bevy_pbr` hunk next to the irradiance-volume
  path), cascade placement and streaming, and the per-preset budget.

Why not the alternatives (research doc 2 section 2): baked irradiance volumes cannot express a carried torch and add
a per-cell bake product; Lumen-style software RT and 3D radiance cascades cost several ms on stronger hardware; voxel
cone tracing (CryEngine SVOGI, Clockwork's voxel bounce) is the no-RT fallback family, kept as a possible *tracer
swap* under the same probe atlas (Godot SDFGI / AMD Brixelizer, both MIT), not built now.

## Tiers

| Preset | GI | Notes |
|---|---|---|
| Ultra (`--rt`) | Solari ReSTIR GI, unchanged | this system off (`RenderPath::RayTraced`) |
| High, RT hardware | Tier A: probe volumes, full budget | 0.6-1.1 ms est. |
| Medium, RT hardware | Tier A at half the probe update rate, fewer exterior cascades | 0.3-0.6 ms est. |
| Low, or no RT hardware | off: cell ambient + sky env map + GTAO (today's look) | Clockwork's "light return" fill values are a cheap option here (research `clockwork-code-2026-10-25.md` row 12) |

"RT hardware" = the device has `SolariPlugins::required_wgpu_features()` (`EXPERIMENTAL_RAY_QUERY` and the binding-array
features; Vulkan/DX12/Metal). Without them the system is absent, not degraded: no plugin systems run, no resources
exist, the shader hunk is compiled out (`GI_PROBES` define).

## Design

### Probe volumes
- **Probe:** world position (grid point + relocation offset), octahedral irradiance 6x6 texels (RGB, e.g.
  `Rgb9e5Ufloat` or `Rgba16Float`), octahedral visibility 14x14 texels (`Rg16Float`: mean distance, mean square),
  one-texel border each for bilinear wrap, as DDGI. Per-probe state: relocation offset, backface-hit fraction,
  "inactive" flag (inside geometry), update priority.
- **Interiors (one worldspace each):** one grid fitted to the cell's bounds at ~1 m spacing, clamped to a probe budget
  (e.g. 32x16x32 = 16k; huge interiors get camera-following cascades like exteriors). Rebuilt when the worldspace
  changes (door teleport): the new volume starts from a flat estimate (cell ambient) and converges over a few frames
  behind the door fade (`portals.md`).
  *As built (2026-11-01, placement slices):* `sf_gi::fit_volume(min, max, VolumeBudget)` pads the box by one spacing
  per side; spacing = `ceil(max(extent / (max_dims - 2)) / min_spacing) * min_spacing` (default budget 32x16x32 at
  >= 1 m), dims = `ceil(extent / spacing) + 2`. The bounds are the world index's cell content AABBs
  (`CellDirectory`, known before placements stream). The game's `mw_exterior_bevy` `gi_volume.rs` follows
  `ActiveWorldspace`: on a change (the fade's cut) the old volume is despawned, a `Partition::Single` worldspace gets a
  fitted volume (`gi: volume AxBxC probes at S m (worldspace <id>; bounds ...)`), a streamed one none (`gi: no volume
  (exterior)`); volumes are immutable (a re-fit respawns). While the fade holds, it keeps the screen black until the new
  volume's first full trace shows (`FadeTransition::hold`, capped by the fade's 1.5 s gate). Live switch:
  `sf_sky_bevy::GiEnabled` (console `render.gi`, `on|off|flip gi`): off drops the volumes' GPU state (not traced or
  shown), keeps the entities and the ray-traced scene; on re-traces from scratch. The glue could move into `sf_gi_bevy`
  (generic: directory + active worldspace) once it depends on `sf_world_bevy`. Not yet: no flat cell-ambient start
  (probes start black until traced; hidden behind the fade), exterior starts have no GI plugin (so no interior GI after
  entering from outside).
- **Exteriors:** 3-5 camera-following cascades (e.g. 16x8x16 probes each at 2, 4, 8, 16, 32 m; AC Shadows uses 5
  cascades at 2-32 m). Cascades scroll in whole probe steps (toroidal addressing, no reconvergence on scroll).
- **Lookup:** finest cascade containing the point wins, blended into the next at the edge; outside all cascades the sky
  env map alone (today's path).

### Update pass (compute, one dispatch per frame)
1. **Select probes:** a budget per frame (High 512-1024, Medium half), round-robin plus priority for probes near the
   camera and near lights that changed this frame (a torch moved, a lamp toggled).
2. **Trace:** 32-64 rays per probe, spherical Fibonacci directions rotated by a per-frame random rotation, through
   Solari's TLAS (`trace_ray`), with the probe's relocation offset as origin.
3. **Shade hits:** albedo and emissive from the hit (`resolve_ray_hit_full`), direct light from **our light list**
   (sun with a shadow ray, point/spot lights with a visibility ray and the same falloff as the raster shader, a cap on
   lights per hit by distance), plus indirect from last frame's probes at the hit (the multi-bounce); back-face hits
   count toward relocation and are shaded black.
4. **Misses:** the sky environment map (`sample_environment_map_light`), which already carries time of day and weather.
5. **Blend:** hysteresis per probe (0.9-0.97; lower for probes whose lights changed, so a torch lags frames, not
   seconds); irradiance and visibility atlases updated in place; borders copied.
6. **Relocate / classify:** as DDGI (move probes out of walls along the shortest back-face ray; mark probes with a
   high back-face fraction inactive).

### Sampling (per pixel, vendored `bevy_pbr` hunk)
Eight surrounding probes of the finest cascade, weights: trilinear x smooth back-face (normal vs direction to probe)
x Chebyshev visibility from the visibility atlas, with the usual bias along normal + view direction. Result replaces the
irradiance-volume and env-map diffuse term when a GI volume covers the point (it keeps Bevy's env-map specular and multiplies in
`diffuse_occlusion` from GTAO). Specular occlusion: the probe's mean visibility along the reflection direction scales
the env-map specular (cheap cone occlusion). The hunk lives beside `irradiance_volume.wesl` under a `GI_PROBES` define,
one new bind group entry (the atlases + probe buffer + per-cascade uniforms); no render-world injection (CONTRIBUTING 4).

**As built (slice 3, 2026-11-01, gi-sampling).** Atlases (`sf_gi::ddgi`, from the paper, Majercik et al. 2019;
not RTXGI's SDK): per probe an octahedral irradiance tile 6x6 and a visibility tile 14x14 (mean and mean squared
hit distance, clamped to `1.5 x sqrt(3) x spacing`), each with DDGI's one-texel border (edges mirrored, corners from
the opposite corner); tiles at column `x + dims.x * y`, row `z` (16x8x16: 1024x128 and 2048x256 texels). Both
`Rgba16Float` render-world textures owned by `sf_gi_bevy` (`Rg16Float` and `Rgb9e5Ufloat` are not core
storage formats); history in storage buffers (interior texels, f32). Update: irradiance texel = cosine-weighted mean
radiance (irradiance / pi, Bevy's env-map units), visibility texel = moments under a `cos^50` lobe, both blended
with the probe's hysteresis; the second bounce reads the history with the same 8-probe DDGI weights (nearest
texel, normal bias only). Sampling (vendored `bevy_pbr`, `superfargoth patch (own, GI)`, `render/gi_probes.{rs,wesl}`):
a view with a `GiProbeVolumeView` render-world component (inserted by `sf_gi_bevy` on the render-path cameras'
views once every probe was traced) gets `MeshPipelineKey::GI_PROBES` / `MeshPipelineViewLayoutKey::GI_PROBES`
and mesh-view bindings 39-42 (irradiance atlas, visibility atlas, linear clamp sampler, `GiProbeVolumeUniform`);
the bindings travel in the component, so a view's layout and bind group always agree. In `apply_pbr_lighting`,
inside the volume's box: `gi_probe_irradiance(P, N, V) x diffuse_color x diffuse_occlusion` replaces the
irradiance volume and env-map diffuse (env-map specular kept); 8 probes, trilinear x `((dot(dir, N) + 1) / 2)^2 + 0.2`
x Chebyshev`^3`, weights under 0.2 crushed (paper), self-shadow bias `(0.2 N + 0.8 V) x 0.75 x spacing x 0.3` (the
paper's value, ~4x smaller than a full spacing), square-root perceptual blend (the paper's; RTXGI's
`pow(x, 1/5)` not needed). Not built: specular occlusion from the visibility tile (needs a reflection-direction
moment fetch per probe and a cone model: next slice), probe relocation/classification, edge blend at the box
boundary (hard switch to the env map).
Built (2026-11-02, gi-ambient): inside the box the flat `AmbientLight` term (diffuse and its specular) is scaled by
`GiVolume::ambient_scale` (uniform `bias_intensity.w`, default 0: the probes replace it, as DDGI/Lumen integrations
drop the flat term under probe coverage); outside the box and on lightmapped meshes it is untouched. The game uses 0.
Built (2026-11-02, gi-tune): `intensity` and `ambient_scale` are live (UE's "Indirect Lighting Intensity"): `prepare`
rewrites the 48-byte volume uniform in place when they change (pure `uniform_update`), keeping atlases and history;
the dev console's `render.gi_intensity` / `render.gi_ambient` write every `GiVolume`, re-fits included.

### Scene and lights
- **RT scene:** `RaytracingScenePlugin` alone (not `SolariPlugins`) on the raster path; `RaytracingMesh3d` mirrored
  onto visible static meshes of the active worldspace (today `path.rs` does this only on the RT path: it becomes "RT
  path or GI on"). Skinned characters stay out (Solari gap; AC Shadows excludes them too). Terrain: the RT proxies the
  terrain spec plans for Ultra (coarse clipmap meshes). BLAS build is per mesh asset, retained; TLAS per frame.
- **Ray-tracing mesh variants (slice 1b):** Solari takes only uncompressed position/normal/uv0/tangent meshes with
  u32 indices, so streamed models need a second copy. `sf_world_bevy::RayTracingVariants` (resource: which
  worldspaces, `sf_world::RayTracedWorldspaces::{None, Single, All}`, and the LOD level) makes streaming load a
  worldspace's cooked models with `SfMeshLoaderSettings::ray_tracing_lod`; `spawn_template_ranges` puts
  `RayTracingMesh` on the traced level's parts and `NoRayTracing` on the others (both components now live in
  `sf_model_bevy`, re-exported by `sf_sky_bevy`). The game sets `Single` (interiors: `Partition::Single`) at Startup
  when `GiSupport` is true or the ray-traced path is active, level `mw_assets::cooked::RAY_TRACING_LOD` (0).
  Exteriors stay without (BLAS memory); adding them is `All` plus a reduced level. Caveat: Bevy keeps one asset per
  path and ignores settings on a path already loading, so a model first loaded for an untraced worldspace has no
  variant until all its users drop it (`StreamStats::ray_tracing_misses`).
- **Precombined buckets:** cell precombines (`.sfnear`, only for worldspaces with a far field) are merged at bake time
  and loaded with default loader settings. Ray-traced worldspaces skip them (`start_fetch`) and trace the per-model
  meshes: correct with no new loader path, and free today since interiors are never precombined. Exteriors under
  RT would rather build bucket variants (fewer, larger BLAS/TLAS instances) in the `CellPrecombine` loader.
- **Active worldspace only:** `mirror_meshes` traces a mesh only if its `RenderLayers` (none: layer 0) intersect the
  render-path cameras' layers. Each worldspace draws in its own layer, so an exterior kept loaded behind an interior
  stays out of the BLAS/TLAS, and meshes leave when the camera changes worldspace.
- **Light list (slice 2, as built):** `sf_gi_bevy` `render.rs` `extract` reads the main world each frame: `PointLight`
  and `SpotLight` with `InheritedVisibility` true whose `RenderLayers` (none: layer 0) intersect the
  `RenderPathCamera`s' (the `mirror_meshes` filter), plus the brightest such `DirectionalLight` as the sun (linear
  colour x illuminance; no atmosphere transmittance tint yet). Not `ViewVisibility`: Bevy culls lights whose range
  misses the frustum, so a world-space volume would lose lights behind the camera. `sf_gi::select_nearest` keeps
  lights whose range reaches the volume box, nearest its centre first, `GiSettings::max_lights` (64). Units are
  Bevy's GPU light (`bevy_pbr` `render/light.rs`): colour x lumens / 4pi (candela), `1/range^2`, spot
  `scale/offset` from the cone angles; shading at a hit (`probe_update.wesl` `list_irradiance`, mirrored by
  `sf_gi::GiLight::irradiance`) is albedo/pi x NdotL x `getDistanceAttenuation` x Filament spot cone, times one
  visibility ray (`RAY_FLAG_TERMINATE_ON_FIRST_HIT`) that stops the light's `shadow_map_near_z` short of it (a
  lantern's cage doesn't shadow its own flame). The sun gets a shadow ray to `RAY_T_MAX`; Solari's
  `directional_lights` are not read (dropped). Storage buffer at group 1 binding 4 (`GiLight`, 64 bytes; a dummy
  record when empty), count in the uniforms. **Hysteresis:** `sf_gi::LightTracker` (render world, by main-world
  entity) flags a light new, moved > 5 cm or power changed > 10% since last frame, and lights gone since last frame
  stay one frame as dark changed records; probes within a changed light's range blend with
  `GiSettings::changed_hysteresis` (0.8) instead of 0.95 (`near_changed_light`, GPU). Cost is O(lights in range x
  rays) per hit: no per-hit light cap yet (open: importance-sample one or a few lights per hit, as Solari's ReSTIR
  DI, if N grows outdoors). Carried torches are ordinary point lights on the character.
- **Materials at hits:** Solari's binder reads `StandardMaterial` assets (`scene/binder/assets.rs`): base colour,
  emissive, normal, metallic-roughness textures. Our NIF meshes use `StandardMaterial` (`mw_assets`); terrain and
  other custom materials need a proxy albedo (spike check 2). Alpha-tested foliage: opaque at hits in the spike;
  later, `RAY_FLAG` any-hit alpha test or AC's per-mesh baked opacity.
- **Lamp bodies (L-B, as built 2026-11-05):** a mesh with `GiSkipEmissive` (`sf_model_bevy`, re-exported by
  `sf_sky_bevy`/`sf_gi_bevy`) gives probe hits its albedo but no emission, since its glow is already a light in the
  list (openmw-rtx `LampBody`). On the raster path `mirror_meshes` puts its `RaytracingMesh3d` on a child proxy (no
  `Mesh3d`, never rasterized) whose material is an emission-free copy (one per source material): no shader or Solari
  patch, raster look unchanged. On the Solari path the mesh keeps its own material (emissive may be its only light);
  a path switch or a new flag re-mirrors. The game flags emissive meshes under a model with a placed light
  (`mw_assets` `mark_lamp_bodies`). Interior `ambient_scale` floor 0.1 (`gi_volume.rs`).

### Per-preset budget (replaced by spike numbers)
| | Probes resident | Updated / frame | Rays / probe | Atlas VRAM |
|---|---|---|---|---|
| High | interiors <= 16k; exteriors 5 x 2k | 512-1024 | 64 | ~40 MB |
| Medium | interiors <= 8k; exteriors 3 x 2k | 256-512 | 32 | ~20 MB |

VRAM for BLAS/TLAS is the larger cost (AC: ~320 MB in a city); the 4060 laptop has 8 GB; measure on the Balmora walk.

## Interfaces

- New crates: `sf_gi` (Bevy-free: cascade placement, toroidal addressing, octahedral mapping, Fibonacci directions,
  probe budgeting, hysteresis; exact tests) and `sf_gi_bevy` (thin plugin: extract lights, prepare atlases and
  buffers, the compute node, the `IrradianceVolume` spike bridge, GPU timing span `gi/probe_update`). Both in
  `superfargoth/crates`, `sf_gi` in hakari's final-excludes.
- `GiPlugin { settings: GiSettings }`; `GiSettings` per preset (probe budgets, rays, cascades, hysteresis) chosen by
  the game (`exterior.rs` quality table), `--off gi` A/B part, console vars `gi.rays`, `gi.budget`, `gi.hysteresis`
  (dev builds). `GiSupport(bool)` resource from the device features (as `RayTracingSupport`). `GiVolume` component on
  the volume entity (interior grid or exterior cascade set), `GiLightList` written in `Render` extract.
- Game side (`bevyrrowind`): nothing Morrowind-specific; the cell bounds for interior grids come from the streamed
  worldspace's bounds (`sf_world`), lights from the existing light entities.
- Net-ready: presentation only, no gameplay reads GI.

## Verification

- `sf_gi` unit tests: cascade scroll keeps probe identity (toroidal), octahedral encode/decode round trip, Fibonacci
  directions unit length and well distributed (exact assertions).
- Sanity (look and feel rule): atlases finite and bounded (no NaN, irradiance <= sky + lights), probe relocation
  converges, a worldspace change leaves no stale probes; printed by `#[ignore = "measurement: ..."]` tests and the
  `gi/probe_update` GPU span (ms per pass per preset, probe count, rays).
- Owner visual check (never automated, CONTRIBUTING "Tests and visual checks"): register scenes in
  `bevyrrowind/docs/visual-checks/gi.md`: Census office with a carried torch (bounce follows the torch, no leaks
  through the floor), a lamp toggled by console, Balmora street at 10:00 / 18:30 / overcast (under-eave darkening,
  sky-lit shadows), Vivec canton interior (large interior cascades), door teleport (no stale lighting). A/B `--off gi`
  and `--rt` (Solari) in the same room.

## Spike (first slice, ROADMAP 4 "GI")

One interior (Census office), High, DLSS Quality: `RaytracingScenePlugin` on the raster path, one hand-written
ray-query compute pass, 16x8x16 probes at 1 m, 64 rays each, 512 probes per frame, the sun and one torch from the light
list, hysteresis 0.95, output as ambient cubes through stock `IrradianceVolume`. Deliverable: the pass runs, the
`gi/probe_update` span prints ms, console `--off gi` A/B, and answers to spike checks 1-5 written into this spec:
1. The forward renderer runs with `RaytracingScenePlugin` but without `SolariLightingPlugin` (no forced deferred).
2. Which of our materials give albedo at hits (`StandardMaterial` yes; terrain, LOD, water, foliage cutouts?).
3. Cost per pass at 512 probes x 64 rays, and VRAM for BLAS/TLAS + atlases in the office and on the Balmora walk.
4. Behaviour without RT features: clean "absent", no panic, no bind-group mismatch.
5. Torch lag at hysteresis 0.95 vs 0.8 (frames to converge, printed).

Slices 1, 1b and 1c as built, with checks 1-3 and the slice 1c root causes: [gi-history.md](gi-history.md).

- Check 5: **pending.** Slice 2 (gi-lights) built the light list and the per-light hysteresis (0.8 near a changed
  light); torch lag frames at 0.95 vs 0.8 are not printed (needs a probe readback and a moving torch in view).
- **Slice 2 run (2026-11-01, gi-lights, dev build, 1280x720 window):** `gi: probe_update 0.048 ms, 512 probes, 32768
  rays; ray-traced meshes 833 mirrored, 682 accepted (BLAS), rejected 151 (not enabled 0, attributes 151, topology 0,
  not u32 indices 0), unclassified 0; Solari lights 0; lights 21 (0 changed), sun no` (0.045-0.077 ms typical, one
  0.150 spike, over 30 s; 25 lights in the office's layer, 21 reach the volume). No panic, no wgpu validation error.
  `--off gi` run (~25 s): no `gi:` line. The first slice-2 run showed `lights 1`: the volume was centred on the
  `view` overview eye, ~15 m above the room (the box missed it entirely); `mw_exterior_bevy` now centres it on the
  start's look target (the placements' centre), still static.
- **Slice 3 run (2026-11-01, gi-sampling, dev build, 1280x720 window, ~15 s, then the owner's no-run rule):** volume
  15x7x21 at 2 m (gi-placement's fit); `gi: volume traced, sampled by the PBR shader (DDGI probes)` ~0.15 s after the
  first dispatch; verbatim: `gi: probe_update 0.081 ms, 512 probes, 32768 rays, atlas 105x21 tiles; ray-traced meshes
  833 mirrored, 682 accepted (BLAS), rejected 151 (not enabled 0, attributes 151, topology 0, not u32 indices 0),
  unclassified 0; Solari lights 0; lights 26 (0 changed), sun no` (0.078-0.098 ms typical, 0.18-0.28 ms for a few
  seconds, one 0.232 ms start). No `naga`/`wgsl`/validation error or panic in the log (the GI_PROBES PBR pipelines
  compiled). `--off gi` run not done (owner: no game runs by agents); command for the owner in GI-2.
- Check 4 (no RT features): by construction: `GiSupport` false skips `render::build` (no pipelines, no Core3d
  system), no texture, `GiActive` stays false; `RaytracingScenePlugin` warns and
  stays empty. `--off gi` adds nothing. Neither run yet on a GPU without ray queries.
- Interiors have no sun since the 2026-11-01 shadow audit (SH-16): there the spike lights only from emissive and
  misses until the light list (slice 2) brings point lights (built, see above).

## Industry comparison

| Engine / title | Technique | Dynamic lights | Cost (published) | Ours |
|---|---|---|---|---|
| AC Shadows (Anvil, 2025) | 5 cascaded DDGI volumes + 1 ray/px diffuse, screen-space march first | full | ~1 ms probes + 2.65 ms per-pixel (4070) | same probe design; per-pixel pass is Tier B, later |
| idTech 8 (2025) | 6 cascaded irradiance volumes + world radiance cache, RT as visibility | full | 1.4-1.9 ms (Series X 1440p) | same; cache = Tier B |
| Metro Exodus EE, Snowdrop | RT probe grids / caches | full | n/a | same family |
| CryEngine SVOGI (KCD2), Clockwork | voxel cone tracing, no RT | full, noisy small lights | 2-3 ms "average PC" | fallback tracer candidate only |
| UE5 Lumen | SDF/HWRT + surface cache | full | 4-8 ms console | too heavy |
| Unity APV, UE volumetric lightmaps, Skyrim ambient cube | baked probes (+ sky occlusion) | static only | ~0 | rejected: carried torch |
| Bevy Solari | ReSTIR DI/GI path tracing | directional + emissive only (0.20) | needs DLSS RR | Ultra tier |

## Open questions

1. Can `RaytracingScenePlugin` run on the forward path in 0.20 without Solari's deferred default (spike 1)?
2. Proxy albedo for custom materials at hits; alpha-tested foliage (spike 2).
3. BLAS memory and build hitches while streaming exterior cells; TLAS rebuild cost per frame.
4. Thin walls (tents, modular kits thinner than probe spacing): visibility + relocation + GTAO reduce leaks; Ubisoft
   reports they never vanish. Decide the acceptable level with the owner.
5. Torch flicker/lag vs hysteresis; whether a near-torch mini-cascade or Tier B is needed.
6. wgpu ray queries are `EXPERIMENTAL_RAY_QUERY`: API may shift on the next crate update.
7. Whether Medium should get GI at all on the 4060 (cost vs the 11 ms target is High's; Medium's own target TBD).
