# far-field: archive

Moved out of [far-field.md](../far-field.md) on 2026-10-26 to keep the spec under the doc budget: history, as-built logs, research and comparisons. far-field.md and the code win where they differ.

## Research summary (2026-10-11)

Reuse order (CLAUDE.md rule 8): Bevy, then crates, then our own.

- **UE5 World Partition HLOD.** HLOD actors stand in for *unloaded*
  cells and stream on their own runtime grid. Layers: *Instancing*
  (lowest-LOD instanced meshes, "ideal for impostor meshes such as trees
  and foliage"), *Merged*, *Simplified* (merged then simplified to a
  screen size), *Approximated* (remeshed with baked materials), *Custom*.
  Layers chain through a parent layer, so each level switches at its own
  distance; the top layer is usually "always loaded". UE's HLOD swap is a
  hard switch unless the material opts into dithered LOD transitions.
  This is the shape we adopt.
- **OGC 3D Tiles** is the open standard for exactly this problem: a tree
  of baked proxies, each with a `geometricError` in meters, REPLACE
  refinement (children replace the parent), and screen-space-error
  traversal. **We adopt its semantics, not its format**: every node stores
  its geometric error, refinement is REPLACE, and the switch distance is
  the bake.md screen-error formula (= 3D Tiles' SSE test). The format
  (`tileset.json` + glTF/b3dm content) buys nothing here: our products
  live in the `sf_bake` store and are read through `baked://`.
- **`bevy_3d_tiles` 0.5.1** (MIT/Apache, active): a 3D Tiles renderer for
  Bevy. **Rejected**: it targets Bevy 0.19; it loads glTF/b3dm tile
  content (we have `.sfmesh` and need instanced impostors, which it
  doesn't support: no `i3dm`); it despawns tiles outside the render cut
  (entity churn, the cost we're minimizing); its traversal is per-frame
  SSE over a tileset, while our residency is the streaming scheduler's.
  Useful data point from its docs: its maintainers built a dithered LOD
  crossfade, measured it, and removed it for a plain swap. Its
  "keep the parent until children arrive" rule is our no-hole invariant.
- **Bevy 0.20, engine features used first:**
  - `VisibilityRange` does LOD switching with a *complementary* ordered
    dither: an entity leaving through its `end_margin` and one entering
    through an equal `start_margin` draw disjoint pixels (verified in
    `mesh_functions.wesl` / `pbr_functions.wesl`: the outgoing entity
    keeps threshold < 16 − k, the incoming keeps the rest). The dither
    distance is measured on the GPU from the mesh's *origin*
    (`world_from_local[3]`); `use_aabb` only changes the CPU culling
    check, so it is left off (on, CPU and GPU would disagree).
  - Async loading through the `AssetServer` (`baked://`), upload
    throttling by `RenderAssetBytesPerFrame`, GPU frustum and two-phase
    occlusion culling, multi-draw indirect, bindless `StandardMaterial`
    with vertex colors. Proxies are plain `Mesh3d` + `StandardMaterial`
    entities, so they batch with everything else.
  - Missing from Bevy: world partition, HLOD baking, impostors, light
    sprites. Those are ours.
- **Bevy virtual geometry (`MeshletPlugin`): not used.** Vulkan/Metal
  only, needs 64-bit texture atomics, position/normal/UV only (content
  relies on vertex colors; alpha-tested parts are common), no MSAA, no
  mixing forward and deferred (our raster path is forward + prepass;
  Solari is deferred). Its per-cluster LOD also doesn't remove
  per-instance entities, which is our real cost. Revisit for hero assets
  when it drops those limits.
- **Godotwind (the owner's earlier Godot attempt): an anecdote, not a
  reference.** Its costs came from Godot specifics, so none of its tiers
  or constants carry over. The general point, which UE's offline HLOD
  build already makes: far representations are baked and simplified
  offline, never merged at runtime. Distances here are starting values,
  calibrated by our own Bevy measurements.
- **Crates.** No Bevy or Rust crate does octahedral impostors or HLOD
  baking (crates.io searched 2026-10-11 for impostor, hlod, lod,
  streaming; GitHub: Godot and three.js impostor ports only). Reused:
  `meshopt` 0.6.2 (merge, weld, simplify, optimize), the `sf_bake` store,
  `sf_model`'s `.sfmesh` container and `sf_model_bevy`'s mesh decoding.
  - Impostor capture: a CPU rasterizer. `euc` 0.5.3 (Apache-2.0 AND MIT)
    was last released in 2021 and is a generic programmable pipeline; an
    orthographic capture with a depth buffer and barycentric attribute
    interpolation is ~150 lines, deterministic, and needs no GPU in the
    bake. Ours. GPU capture only if CPU misses the bake budget.
  - Texture decode/encode (for textured L1 and impostor atlases of
    textured models): `image` 0.25 (already in the tree through
    `bevy_image`; decodes PNG/TGA/BMP/DXT DDS) for decode; BC7 through
    `ctt` 0.5 (MIT OR Apache-2.0 OR Zlib, the backend of Bevy 0.20's
    `CompressedImageSaver`, C++ encoders built with `cc` like meshopt)
    when far textures land. Phase 1 ships vertex-color proxies and RGBA8
    impostor atlases; compression follows the textured L1 (below).


## Measured on Morrowind (2026-10-11)

Release build, RTX 4060 Laptop + Ryzen 7 7435HS, on AC, idle machine (no
cargo/rustc), `benchmarks/exterior_*.json` (run/flight with
`--bench-ground`). "Off" = `--off farfield` (streaming's old far ring).

| scenario | far field | off |
|---|---|---|
| Balmora standing, 720p | 7.84 ms median, p99 9.6 | 20.86 ms |
| Balmora standing, 1440p | 11.87 ms (GPU-bound: 8.6 ms timed passes) | 22.2 ms |
| run 7 m/s, 720p / 1440p | 10.0 / 11.3 ms, worst 17.8 / 18.4 | 23.4 ms, worst 53 |
| flight 60 m/s, 720p / 1440p | 9.6 / 10.9 ms, worst 19.4 / 20.9 | 27.6 ms, worst 66 |
| entities, Balmora | 48.4k (far 2,317) | 353k |

- Gates: far-field CPU (`far/select/ms`) 0.14–0.17 ms mean, 0.5 ms max
  (≤ 0.3 ✓); far-field entities 2.3–3.5k (≤ 5,000 ✓); far-field GPU
  ≈ 1.2 ms at 1440p (main pass with near objects off minus terrain only,
  on a GPU below the 3070 reference: ✓); no frame over 33 ms ✓; worst
  frame vs median 1.6× (run) and 1.9× (flight) at 1440p ✗: the remaining
  spikes are streaming frames (spawn/despawn 1–3 ms, big_space 2–3 ms,
  terrain updates), not the far field. Origin shifts now cost 2–3 ms
  (big_space rewrites ~50–75k transforms instead of ~350k).
- Lead-band double draw: 0.26 ms GPU (above). Depth prepass: +0.4 ms and
  broken crossfades (pitfall 2), removed.
- **`d1` calibration** (runtime `screen.bias`, Balmora 1440p): d1 300 m →
  11.55 ms, 30.4k entities, main pass 6.54 ms (proxies shown at ~1.3 px);
  400 m → 11.87 ms, 48.4k, 6.26 ms; 600 m → 15.05 ms, 97.4k, 7.88 ms.
  **400 m stays the default** ("high"); 300 m is the "low" candidate. The
  GPU frame is led by terrain (2.9 ms of main pass in that view), then
  near objects (~2.2 ms), then the far field (~1.2 ms).
- Screenshots: `D:/Gamedev/Bevyrrowind/shots/farfield/` (`balmora_a*`,
  `redmtn_view*` (the mountain's see-through gaps with the far field off),
  `summit_vivec*` incl. night skyline lights, `ebonheart_night*`,
  `handoff_forest*` (impostor handoff), `evidence_prepass_dither_holes.png`).

## Industry comparison

J9 (industry-audit.md), 2026-10-12, docs research only (`.sessions/j9rest.md`).
"Research summary" above covers UE HLOD, 3D Tiles and Bevy; this table adds
the other engines and verdicts. "Unverified" marks claims from memory not
re-read this session.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Hierarchical baked proxies over a grid | UE World Partition HLOD layers (Instancing, Merged, Simplified, Approximated); Bethesda object LOD per 4/8/16/32-cell block (generated offline by the Creation Kit, extended by the community's DynDOLOD, unverified detail); RAGE SLOD1–4 + per-entity `lodDist` (GTA V); OpenMW object paging (objects merged per chunk by size); 3D Tiles REPLACE | L1–L3 baked clusters, 3D Tiles semantics | **match** |
| Precombined near statics | Fallout 4/76 precombines; OpenMW active-grid paging | none | known gap (audit row 10, J6) |
| Foliage far representation | octahedral/hemi-octahedral impostors (Brucks 2018, Amplify, UE Instancing HLOD layer); SpeedTree billboards; Bethesda flat 2-view tree LOD billboards (Skyrim; DynDOLOD adds 3D tree LOD, unverified); **UE 5.7 Nanite Foliage (experimental): assemblies of instanced parts, bone-skinned wind, distant trees as Nanite voxels** | hemi-octahedral 8×8, albedo + normal/depth, 3-frame blend, lit by Bevy PBR | **match** for raster impostors. Nanite voxel foliage is n/a (Bevy meshlets, below); it's the direction AAA foliage is moving, worth re-checking when Bevy's virtual geometry grows |
| Distant light points | GTA V "LOD lights" (city light sprites to the horizon); DynDOLOD glow windows (unverified); OpenMW none | light sprites, night factor | **match** |
| LOD transition | UE dithered LOD transition (opt-in per material), Unity LODGroup crossfade (dither), RAGE fades; 3D Tiles renderers often hard swap | `VisibilityRange` complementary dither + lead bands | **match** |
| Virtualized geometry | UE5 Nanite (Karis 2021): cluster DAG, per-cluster LOD and culling, software raster for small triangles, Nanite landscape and foliage | Bevy 0.20 `MeshletPlugin` re-checked in source 2026-10-12: still Vulkan/Metal only, needs `TEXTURE_INT64_ATOMIC`, no MSAA, no forward/deferred mixing, position/normal/UV materials; has instance BVH + cluster culling | **n/a** (unchanged reasons above; audit row 32, P2 revisit) |
| Shadows of far proxies | UE far shadow cascades + distance field shadows; Bethesda LOD casts no dynamic shadow (unverified) | L1 in cascades, terrain far-shadow map beyond | **match** |
| Size culling of small objects in proxies | UE HLOD "minimum screen size" per layer; OpenMW min size | 2 px at the level's near end | **match** |

No new gap: the open differences are audit rows 10, 12, 16 and 32.

**Sources** (checked 2026-10-12, beyond "Research summary"):
- UE, Nanite Foliage (5.7, experimental): https://dev.epicgames.com/documentation/en-us/unreal-engine/nanite-foliage
- UE 5.7 feature coverage (assemblies, skinning, voxels): https://80.lv/articles/check-out-new-features-arriving-with-unreal-engine-5-7 and https://tomlooman.com/unreal-engine-5-7-performance-highlights/
- Bevy 0.20 `bevy_pbr/src/meshlet/mod.rs` (`MeshletPlugin` docs), local registry.
- OpenMW object paging: settings docs, cited in industry-audit.md.

