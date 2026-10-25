# industry-audit: archive

Moved out of [industry-audit.md](../industry-audit.md) on 2026-10-26 to keep the spec under the doc budget: history, as-built logs, research and comparisons. industry-audit.md and the code win where they differ.

### J1 as built and measured (2026-10-12, `.sessions/sizecull.md`)

Built: `LodScreen::{min_pixels, min_draw_distance, focal_px, draw_distance}`
(`sf_model`), `size_culled_end` + `StreamStats::size_culled` (`sf_world_bevy`,
applied to every spawner through `SpawnCx::max_distance`/`end_band`, so the
runtime-NIF `SubtreeRange` path follows too), `CellProxies::kept_radius`
(far field reports the L1 keep rule per cell), `PlacementFlags::NO_SIZE_CULL`,
app `--quality low|medium|high|ultra` and `--off sizecull`. Bevy 0.20 checked
from source: `VisibilityRange` is tested against the mesh origin (`use_aabb`
false) from `lod_view_world_position`, i.e. the camera or `ShadowLodOrigin`
in shadow views, on CPU (`check_visibility_ranges`) and GPU
(`mesh_preprocess.wesl`), and the dither (`get_visibility_range_dither_level`)
runs in the main and prepass/shadow shaders; so a size-culled object fades
out of its shadow at the same distance. The design held; changes: the
L1-contributor clamp is real (see above), `d_floor` is absolute, radius 0
(unknown bounds) is exempt.

**Histogram** (`cargo run --release -p mw_bake --example size_histogram`,
147,897 exterior placements with a model; radius p5 0.66 m, p10 1.2 m,
p50 3.6 m): placements size-culled before their ring/handoff end: Low 11,318
(8 %), Medium 8,737 (6 %), High 4,773 (3 %), Ultra 491 (0.3 %), almost all
between 128 m and the handoff. Eligible ones the L1 clamp may keep: 1937 /
1254 / 528 / 126. **Vanilla exteriors have few small objects**: the clutter
the audit pictured (cups, plates, books) is interior content; there are only
432 item placements outside.

**A/B** (`perf suite`, 2 interleaved rounds on/off, release `solari,dlss`,
DLSS off, RTX 4060 Laptop at 2573–2584 MHz, 66–82 °C; `target/perf/sc-*`):
Balmora at High size-culls 503 of 13,357 resident placements; entities
48,419 either way (culled chains are single-level, no level skipped).
Medians on/off: Balmora 1440p 11.95, 12.02 / 12.02, 12.12 ms (main opaque
6.29, 6.39 / 6.31, 6.43); **Balmora CPU floor (960x600) 6.03, 6.01 / 6.32,
6.32 ms**, main busy 4.76, 4.67 / 5.05, 5.01 (the only consistent gain,
~0.3 ms: fewer entities pass frustum tests, sorting and extraction); run
1440p 10.54, 10.46 / 10.49, 10.54, floor 7.70, 7.52 / 7.81, 7.60; flight 1440p
10.90, 11.03 / 10.99, 11.02, floor 6.65, 6.68 / 6.77, 6.75. Stutter gate
(borderless 1080p, uncapped): flight 20.1, 21.0 / 21.8, 21.8 a minute, run
3.0, 1.5 / 3.0, 3.0; it fails either way. Draw counts were not measured
(no counter; Nsight). Verdict: correct and cheap, keep on (High default),
but no measurable GPU win on vanilla exteriors. This lowers the expected value of
J3 (size-tiered spawning): at most ~3–8 % of exterior placements, so J2
(`NoCpuCulling`) and J6 (precombine) are the bigger entity/CPU levers.


## Sources

Checked this session (2026-10-12):
- UE, Cull Distance Volumes: https://dev.epicgames.com/documentation/en-us/unreal-engine/cull-distance-volumes-in-unreal-engine
- UE, Visibility and occlusion culling reference: https://dev.epicgames.com/documentation/unreal-engine/visibility-and-occlusion-culling-reference-in-unreal-engine
- UE, `UInstancedStaticMeshComponent` (per-instance start/end cull distance): https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Runtime/Engine/UInstancedStaticMeshComponent
- UE3 visibility culling (MaxDrawDistance, volumes use the lower value): https://docs.unrealengine.com/udk/Three/VisibilityCulling.html
- UE, World Partition HLOD (layers with their own runtime grid cell size and loading range): https://dev.epicgames.com/documentation/en-us/unreal-engine/world-partition---hierarchical-level-of-detail-in-unreal-engine
- UE 4.22 mesh drawing pipeline refactor (automatic instancing, cached static draw commands): https://www.unrealengine.com/events/unreal-fest-europe-2019/refactoring-the-mesh-drawing-pipeline-for-unreal-engine-4-22
- Unity, GPU Resident Drawer: https://docs.unity3d.com/6000.3/Documentation/Manual/urp/gpu-resident-drawer.html
- Unity, LOD Group: https://docs.unity3d.com/6000.4/Documentation/Manual/class-LODGroup.html
- OpenMW camera settings (small feature culling, pixel size 2.0): https://openmw.readthedocs.io/en/openmw-0.47.0_a/reference/modding/settings/camera.html
- OpenMW terrain settings (object paging, min size 0.01, active grid paging): https://openmw-trav.readthedocs.io/en/latest/reference/modding/settings/terrain.html ; defaults: https://git.cijber.net/mirror/openmw/src/tag/stable/files/settings-default.cfg
- Fallout 4 precombines and previs (Bethesda staff quoted secondhand): https://forums.nexusmods.com/topic/5522717-fallout-4-optimization-and-performance-systems-explained/
- Skyrim SE fade multipliers (objects 5, items 3): https://stepmodifications.org/wiki/SkyrimSE:SkyrimPrefs_INI/LOD
- RAGE (GTA V) entity `lodDist`, `childLodDist`, priority level: https://docs.sollumz.org/documentation/maps/entities ; https://docs.fivem.net/docs/assets-manual/beginner-series/part-6/
- Horizon Zero Dawn GPU procedural placement (van Muijden, GDC 2017): https://www.guerrilla-games.com/read/gpu-based-procedural-placement-in-horizon-zero-dawn

- Haar, Aaltonen, *GPU-Driven Rendering Pipelines*, SIGGRAPH 2015 Advances (Ubisoft, AC Unity; GPU
  decides visibility per viewport, motivated by long draw distances, many small objects and shadow-map
  draw counts): https://history.siggraph.org/?p=60117 (course page http://advances.realtimerendering.com/s2015/)
- Wihlidal, *Optimizing the Graphics Pipeline with Compute*, GDC 2016 (Frostbite): https://gdcvault.com/play/1023109/Optimizing-the-Graphics-Pipeline-With
- UE shadow-caster screen-size cull: `r.Shadow.RadiusThreshold` and project setting "Min Screen Radius for
  Cascaded Shadow Maps" (both default 0.03 per forum reports; higher at Medium scalability and below):
  https://issues.unrealengine.com/issue/UE-372125 and the visibility culling reference above

**Unverified** (known references, cited from memory, not fetched this session; check before quoting
details):
- Collin, *Culling the Battlefield*, GDC 2011 (Frostbite software occlusion)
- Karis et al., *Nanite: A Deep Dive*, SIGGRAPH 2021 Advances
- Chen, *Adaptive Virtual Texture Rendering in Far Cry 4*, GDC 2015
- Wohllaib, *Procedural Grass in Ghost of Tsushima*, GDC 2021
- Ruskin, *Marvel's Spider-Man: A Technical Postmortem*, GDC 2019 (cell streaming budgets)
- UE PSO precaching (5.1+) and texture streaming docs on dev.epicgames.com
- Courrèges, *GTA V Graphics Study*: https://adriancourreges.com/blog/2015/11/02/gta-v-graphics-study-part-2

Bevy 0.20 sources read: `bevy_camera/src/visibility/{mod,range}.rs`,
`bevy_light/src/lib.rs` (`check_dir_light_mesh_visibility`),
`bevy_render/src/view/visibility/{mod,range}.rs`,
`bevy_render/src/render_phase/mod.rs`, `bevy_render/src/batching/gpu_preprocessing.rs`,
`bevy_pbr/src/render/{mesh.rs, mesh_preprocess.wesl, gpu_preprocess.rs}`.

