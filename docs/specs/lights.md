# Local lights: culling, budget, cost (ROADMAP 1.4, lights first)

Status: **researched and measured (lights1, 2026-10-20): no light budget needed now**; the cost was the lamp shadow. Shadows of local lights
are in [shadows.md](shadows.md); calibration (lumens per radius², range scale, exposure) in bevyrrowind
`mw_assets::lights::LightCalibration` and [sky-lighting.md](sky-lighting.md).

Problem: the Balmora walk spawns **~290 point lights** (Balmora standing: 210), every light of the near ring, each
with `range = 4 × record radius`. The feature ladder (bevyrrowind perf.md) puts them at **+2.0 ms frame, +2.0 ms
render thread, +1.7 ms GPU** at 1440p. Target: High 1440p ≤ 11 ms, and the CPU frame must fit too.

## What Bevy 0.20 does (read in `bevy_light-0.20.0`, `bevy_pbr-0.20.0`, `bevy_camera-0.20.0`)

- **GPU clustering is the default on desktop** (new in 0.20, PR #23036): `GlobalClusterSettings::gpu_clustering`
  is `Some` when the adapter has compute shaders and storage buffers. `assign_objects_to_clusters` then skips the
  CPU light assignment entirely; per view, five GPU steps (Z-slice compute → count raster → local prefix sum →
  global prefix sum → populate raster) build the cluster lists. The index list grows on overflow
  (`GPU_CLUSTERING_INITIAL_INDEX_LIST_CAPACITY` 65536) with a few wrong frames while it resizes. CPU clustering
  stays as the fallback (WebGL2: uniform buffers, 256 lights max).
- **Which lights reach the GPU:** a light's `ViewVisibility` comes from `check_visibility` like any entity
  (`VisibilityClass` via `ClusterVisibilityClass`): `Visibility`, `RenderLayers`, frustum test of its range
  sphere, **and `VisibilityRange`** (any entity with one is culled outside it: a hard cut, the dither is
  mesh-only). Lights out of the frustum but whose range overlaps it stay in.
- **Extraction is change-driven:** `extract_lights` only visits lights with a changed `PointLight`,
  `GlobalTransform`, `ViewVisibility`, `RenderLayers`, `CubemapFrusta`, `CubemapVisibleEntities` or
  `VolumetricLight`. **An animated light (flicker: `intensity` written every frame) re-extracts every frame.**
  A hidden light's `ExtractedPointLight` is removed.
- **`prepare_lights` is O(lights) per frame:** collects every extracted light, sorts point lights
  (shadowed and volumetric first, then by entity), fills the `GpuClusterableObjects` storage buffer, then per
  shadowed light builds 6 cube-face views. Unshadowed lights skip `update_point_light_frusta` and
  `check_point_light_mesh_visibility` on the main world.
- **Shadow views:** each shadowed point light = 6 views (frusta, visible-entity lists, specialize, queue, batch,
  preprocess, encode), cleared and redrawn every frame. Measured ~0.35 ms render thread per face view even with
  nothing in it (shadows.md "Slice 2 as built"), ~1.9 ms per shadowed light at 1024². One global
  `PointLightShadowMap::size`; max cubes = `max_texture_array_layers / 6` (42 at wgpu's 256).
- **Falloff:** inverse-square with Bevy's range window (smooth to 0 at `range`); `radius` is the emitter size
  (specular, PCSS), not the reach. The per-fragment cost is the cluster's light count, so `range` drives the GPU
  side: 4× radius covers 16× the area of 1× radius.
- **0.19:** contact shadows (`ContactShadows` camera component + `contact_shadows_enabled` per light);
  `shadow_pass` split into `per_view_shadow_pass` and `shared_shadow_pass`. **0.20:** GPU clustering; Solari
  retained scene (RT only).

## Crates (search 2026-10-20)

None found for light budgeting, light LOD or light culling on Bevy 0.20 (web search). Bevy itself gives the culling primitive
(`VisibilityRange` on a light) and GPU clustering; the fade and the budget are game policy.

## Industry practice

- **Per-light draw distance with a fade**, separate for the light and its shadow: UE light components
  `MaxDrawDistance` + `MaxDistanceFadeRange`; Godot 4 `Light3D.distance_fade_begin/length/shadow`; Unity HDRP
  light "Fade Distance" and "Shadow Fade Distance". OpenMW: `maximum light distance` 8192 units (~117 m), fading
  from 0.85 of it ("lights fade out within one cell").
- **Reach tied to the visible falloff, not a long tail.** OpenMW's GL path lights to 1.75 × the record radius;
  its Vulkan "Clockwork" renderer (OursCodeur `clockwork-alpha`, GPL, behaviour only from its settings docs:
  `shaders.rst` "light radius multiplier") to **1.0 × radius**, noting that within ¾ of the radius surfaces are
  lit the same whatever the multiplier, so only where the pool ends moves. Carried lights get a separate
  multiplier (default 8 there, fading more slowly).
- **Clustered forward, all lights through clusters** (Doom 2016, Clockwork: "the Vulkan entries light through
  clusters only"), with caps on lights visible per frame (Unity URP Forward+, HDRP "max punctual lights on
  screen") and a separate, much smaller cap on shadow-casting lights (HDRP max shadows on screen; shadows.md).
- **Far lights are sprites:** past the light's draw distance, glow cards/emissive (we have this: far-field light
  sprites, `sf_lod_bevy::FarLightsNight`); UE plugins swap far lights for decals.
- **Flicker is intensity only** (no re-culling, no re-selection); a flickering light should not cost more on the
  CPU than a steady one.

## Measured (lights1, 2026-10-20; bevyrrowind perf.md "Lights")

- **Unshadowed lights are cheap now:** ~290 lights on the walk cost nothing measurable on the frame; standing in
  Balmora (210 lights, GPU bound) they cost ~0.5 ms GPU (`--off lights` vs `--point-shadows 0`). GPU clustering
  works. The ladder's "+2.0 ms lights" was the **shadowed lamp** that comes with them.
- **The lamp shadow** (High: 1 light, 1024² cube) cost ~3.5 ms frame on the walk, ~2 ms of it a Bevy 0.20 leak of
  cube-face views (shadows.md "Stale cube-face views", fixed on our side). Left: ~1.5 ms frame, 1.4 ms render
  thread for one lamp: the shadows lane, not this one.
- In the trace, `prepare_lights`, `extract_lights` and the GPU cluster passes are each under 0.4 ms; the
  light-related render-thread time is the shadow views (`Core3d` per face, specialize/queue/batch).

## Design (decided by the measurements)

1. **No light budget system now.** It would buy at most ~0.5 ms GPU standing in a town. Keep Bevy's GPU
   clustering and frustum culling as they are.
2. **Later, cheap and standard (P2, when a scene needs it):** a light draw distance with a fade per preset
   (UE, Godot, HDRP; OpenMW ~117 m fading from 0.85), built on Bevy's `VisibilityRange` on the light plus our
   intensity ramp so nothing pops; and the range scale (4× today vs Clockwork's 1×) as a quality knob the owner
   judges by eye, since it moves where pools end.
3. **Flicker** stays per frame: re-extraction of animated lights didn't show in the trace.
4. Shadowed lights: budget and cost in shadows.md.
