# Big-world rendering: as-built history

Moved out of [big-world-rendering.md](../big-world-rendering.md) on 2026-11-26 (doc budget). Append-only; grep it.
The spec keeps a few lines per slice under "As built".

### BW-1: `sf_bigworld` core (2026-11-26; 49b7ae15, 11b0cf8f, 1d134b3d)

Bevy-free, one dependency (`sf_tunables`). Frame: metres, x and z on the ground, y up. 27 exact tests plus one
measurement test.

- **Tiles** (`tile.rs`): `TileKey { x: i32, z: i32 }`, `containing`, `min_corner`, `distance_to` (0 inside the square).
- **Instance format** (`instance.rs`): `PackedInstance` = four little-endian u32 words.
  - `w0` = x | z << 16, each `round(v / tile_m × 65535)`.
  - `w1` = y | yaw << 16 | scale << 24. Height is quantised over the tile's `[y_min, y_max]`, kept in `TileHeader`, so a
    tile's step is its relief / 65535. Yaw has 256 steps. Scale is logarithmic over [0.25, 4] (about 1.1 % steps).
  - `w2` = species | seed << 16.
  - `w3` = flags | up << 16: the up axis, octahedral with +y as the pole (Cigolle et al. 2014), 2 × u8, about 1° of
    error; upright is exact.
  - `PackedTile::pack` / `unpack` / `world_position` / `write_le_bytes`. Packing an unpacked tile again gives the same
    bits. A point on a shared tile edge can decode to exactly `tile_m`.
- **Sources** (`source.rs`): `InstanceSource::tile(key, tile_m, out)` (`Send + Sync`, pure). `SyntheticForest` is a
  jittered grid hashed per tile with SplitMix64 (`hash.rs`), with optional sine relief: for tests, BW-2 and example
  scenes.
- **Tier bands** (`bands.rs`): `ViewScale` (pixels per radian), `SpeciesDims` (height, crown), `Bands::new` gives
  `mesh_end_m = height · ppr / mesh.switch_px` and `impostor_end_m = crown · ppr / aggregate.switch_px` (never below the
  mesh end). `Bands::fades(dist, scale)` returns `TierFades { mesh, impostor }` with `m` and `i` the two ends times the
  instance's scale and `b` = `fade_band`:
  - mesh falls from 1 to 0 over `[m(1−b), m]`; the impostor rises over the same band, so the two always sum to 1 there;
  - the impostor falls from 1 to 0 over `[i(1−b), i]`. The T2 tile must be fully drawn from `aggregate_lead_m = i(1−b)`
    (BW-8 owns that); when a species' two bands touch, this is what keeps it from showing a hole;
  - `b = 0` is a hard switch. Bad inputs (NaN, zero or negative sizes) never give NaN.
  The maths is f32 saturate-style, written to port to the producer's WGSL one-to-one (BW-3).
- **Ring planner** (`ring.rs`): `Ring::update(viewer, &RingSettings) -> RingDelta`. A tile enters when the viewer is
  within `radius_m` of its square and leaves past `radius_m + hysteresis_tiles × tile_m`; optional world `bounds`.
  `enter` is sorted nearest first (the upload order). `clear` is for teleports and for a changed `tile_m`. The renderer
  keeps two rings, from `ring_radii(bands, max_scale)`: the impostor radius (the GPU pool) and the mesh radius
  (`NearInstances`).
- **Tunables** (`tunables.rs`, `Group::Foliage`): `bigworld.tile_m`, `bigworld.ring.hysteresis_tiles`,
  `bigworld.mesh.switch_px`, `bigworld.aggregate.switch_px`, `bigworld.fade_band`. No plugin registers them until
  `sf_bigworld_bevy` exists, so `docs/tunables-index.md` does not list them yet.
- **Measured** (`measure_ring_counts`, release, one thread, ppr 831, one 12 m species with an 8 m crown; ledger
  2026-11-26). The ring counts match the cost model within 5 %:

  | Preset | Density | Tiles | Instances resident (pool) | In the mesh tier | In the impostor tier |
  |---|---|---|---|---|---|
  | High | 60/ha | 276 | 110 k (1.7 MB) | 298 | 95 k |
  | High | 400/ha | 276 | 718 k (11.0 MB) | 1,962 | 618 k |
  | Ultra | 400/ha | 568 | 1.48 M (22.5 MB) | 5,018 | 1.39 M |
  | Low | 400/ha | 80 | 208 k (3.2 MB) | 318 | 155 k |

  The synthetic source generates and packs a tile in 0.02 ms at 60/ha and 0.12 ms at 400/ha. A real scatter with
  terrain queries will cost more; BW-6 measures it.

### BW-2: ceiling bench `forest_ceiling` (2026-11-26; e1b8c973, 0c9b0fab, 62d29ecc)

`sf_bigworld_bevy` exists with an empty library: its one target is the example
`superfargoth/crates/sf_bigworld_bevy/examples/forest_ceiling/` (24 unit tests in the example harness, `test = true`).
The plugin arrives with BW-3. No new Bevy feature set (group [1] of `tools/bevy_variants.sh`).

- **Scene.** A flat `SyntheticForest` at 400 trees/ha around the origin. Camera: the game's DLSS-path camera (`Hdr`,
  no MSAA, depth and motion-vector prepass, `OcclusionCulling`) in a 1707×960 window, fov 60°, 60 m up and pitched 10°
  down, so nothing hides the forest (the hard case). Sun at 45° with 4 cascades to 500 m, 2048 px. One ground plane.
- **Meshes** (`meshes.rs`): a 12 m tree with an 8 m crown, 1,464 triangles (trunk plus 700 alpha-masked leaf quads); a
  2-triangle 8 × 12 m card turned to the camera on the CPU (a stand-in for the impostor billboard); one procedural mask
  texture, one `StandardMaterial` with `AlphaMode::Mask`.
- **Slots** (`fill.rs`, `upload.rs`): 64 B `GpuMeshInstance` slots built once on the CPU and written with
  `RenderQueue::write_buffer` in `GpuInstanceBatchSystems::Publish` (the groundcover writer's pattern, runs split at
  16 MB). Inactive slots are zeroed.
- **Arms** (`--arms`, default all eight, one launch):
  - `empty`: the bare scene, the baseline to subtract;
  - `card:<n>` and `tree:<n>` at 10⁵, 3·10⁵ and 10⁶: the n trees nearest the viewer in one batch of capacity n, all
    active (10⁶ = a 2.8 km disc);
  - `stand`: the cost model's worst case as the producer will write it, from `sf_bigworld::Bands` at the High defaults
    (mesh end 124.7 m, impostor end 2,217 m, horizontal distance): 1,973 trees in a 4,096-slot batch, and 179,901 cards
    in a 250,000-slot batch (every tree with an impostor fade that is inside the 500 m shadow range or the camera
    frustum). The model's counts are 1,960 and about 180 k.
- **Measurement** (`bench.rs`, `stats.rs`): per arm Settle (old batches gone, next arm built) → Warm (4 s) → Measure
  (12 s and 120 frames, capped at 60 s), a 9 min watchdog. It prints `bw2 ceiling | ...` lines: frame median and p99,
  main and render thread ms (`FramePhasePlugin`), the GPU total over top-level spans, every GPU span's median and p95,
  frames without window focus, and a warning when a batch's per-view buffer would pass a wgpu limit.
- **Run it:** `target/debug/examples/forest_ceiling.exe` with no arguments (dev profile: dependencies are optimised),
  built by `cargo build -p sf_bigworld_bevy --example forest_ceiling`. Only the lane's orchestrator launches it.
- **Adapter facts** (smoke launch): RTX 4060 Laptop on Vulkan reports `max_storage_buffer_binding_size` 2 GB and
  `max_buffer_size` 4 GB, so a 10⁶-slot batch (about 176 MB per view) needs no raised limit there.

**Measured (one run, 2026-11-26 17:33, 157 s; lines in `bevyrrowind/benchmarks/runs/bw2_ceiling/run.log`).** GPU ms,
medians of the timestamp spans, each arm minus `empty` (empty: frame 3.47 ms, p99 4.74, GPU 0.69 ms). Preprocess =
`early_` + `late_mesh_preprocessing` + `range_unpacking` + `bin_unpacking`; prepasses = early + late; shadows = the
four `per_view_shadow_pass` spans (cascade 0 holds nothing from 60 m up).

| GPU ms minus empty | card 10⁵ | card 3·10⁵ | card 10⁶ | tree 10⁵ | tree 3·10⁵ | tree 10⁶ | **stand** | model, stand |
|---|---|---|---|---|---|---|---|---|
| materialize | 0.136 | 0.380 | 1.218 | 0.141 | 0.383 | 1.224 | **0.328** | 0.3 |
| preprocess | 0.190 | 1.278 | 3.873 | 0.189 | 1.295 | 4.313 | **1.042** | 0.25 |
| prepasses | 0.651 | 0.882 | 1.234 | 13.47 | 31.39 | 97.55 | **1.849** | |
| shadows | 1.401 | 1.633 | 1.637 | 32.29 | 39.12 | 39.67 | **5.921** | |
| main opaque | 1.844 | 2.520 | 3.460 | 30.97 | 50.98 | 115.59 | **4.600** | |
| raster + fragment (the three rows above) | 3.896 | 5.035 | 6.331 | 76.72 | 121.48 | 252.80 | **12.370** | 1.2 |
| slot path total | 4.222 | 6.693 | 11.422 | 77.05 | 123.16 | 258.34 | **13.740** | 1.75 |
| whole-frame GPU | 4.47 | 6.77 | 11.67 | 77.37 | 123.39 | 258.65 | **13.97** | stop at 3.5 |
| frame median / p99 ms | 5.83 / 7.73 | 16.5 / 19.4 † | 16.7 / 22.3 † | 81.6 / 88.6 † | 127 / 135 † | 263 / 269 † | 18.0 / 19.1 † | |

- **Verdict: the stop rule trips** (13.97 ms against 3.5 ms). `stand`, whole frame: median 17.96 ms, p99 19.15 †,
  main 1.07 ms (empty 1.21), render busy 3.41 ms (empty 2.62), render thread 17.5 ms (it waits on the GPU).
- **Materialize is linear and as modelled:** 1.36, 1.27 and 1.22 ns per slot of capacity at 10⁵, 3·10⁵ and 10⁶
  (1.29 at stand).
- **Preprocess costs capacity × views, and is not linear below 3·10⁵:** 0.38 ns per slot per view at 10⁵, then 0.85
  and 0.77-0.86 at 3·10⁵ and 10⁶ (5 views; 0.82 at stand). 6.7× the cost for 3× the slots between the first two
  sizes; the same in the card and the tree arms, so it comes from the slot count, not the mesh. Cause not looked for.
- **No VRAM fault at 10⁶:** materialize and preprocess grow 3.2× and 3.0-3.3× for 3.3× the slots (the groundcover
  ledger's GC-D1 jump of 3-8× did not happen), and no buffer-limit warning was printed.
- **Raster is fill-bound near the camera, vertex-bound far away.** Trees: 0.52, 0.28 and 0.17 ms per million
  triangles in slots (all five views together) at 10⁵, 3·10⁵ and 10⁶; each further ring adds 0.13-0.15 ms per million
  triangles in the ring (~4 Gtri/s per camera pass if a quarter of the ring is in the frustum as the model assumes;
  the model used 2.5). The first 10⁵ trees cost 76.7 ms. Cards: 3.9 ms for the nearest 10⁵, then +1.1 ms for the
  next 2·10⁵ and +1.3 ms for the next 7·10⁵ (1.9 ns per far card).
- **Both halves of `stand` break the 1.2 ms row on their own.** Its cards within 1.5 km are a subset of
  `card:300000`, so they cost at most ~5.0 ms; that leaves at least ~7.3 ms for the 1,973 trees (2.9 M triangles,
  700 alpha-masked leaf quads each), of which 2.5 ms in cascade 1 alone. The bench has no arm that separates them
  further.
- **Caveats.** The camera is the hard case (60 m up, nothing hidden). Cards are plain alpha-masked quads with the PBR
  `StandardMaterial`, not BW-4's impostor shader, and in the sweep arms cards and trees sit right at the camera: only
  `stand` compares with the model. No DLSS: the window renders 1707×960 directly. † The window lost focus during
  `card:300000` and never got it back (`focus=lost`, 2,967 frames), and other lanes' builds ran from 17:33:51 to the
  end: frame, main and render ms of every arm after `card:100000` are dirty (the card and stand frames sit at the
  60 Hz pacing of an unfocused window). The GPU spans do not show it: `tree:100000` (unfocused) and `card:100000`
  (focused) agree on materialize and preprocess within 4 %. Main and render ms come from a dev-profile exe (our
  crates opt-level 1, dependencies 3): the render-busy rise of 0.5 to 2.3 ms with the slot count (model: ≤ 0.2 ms)
  needs a release run before it counts.

### BW-2b: the standard fixes, priced (2026-11-26)

The `stand` arm takes per-batch variants (`fill.rs`, `main.rs`): `stand[/trees[.mod..]][/cards[.mod..]]`, only the
named batches are spawned; `.unlit` = `StandardMaterial::unlit`, `.opaque` = `AlphaMode::Opaque` in place of
`Mask`, `.noshadow` = `NotShadowCaster` on the batch. No arguments still runs the BW-2 list. One run of eleven arms
(17:48, 177 s, **focus kept, no build running**: other lanes' builds were held by a build freeze); lines in
`bevyrrowind/benchmarks/runs/bw2_ceiling/run2b.log`. GPU ms, span medians minus `empty` (0.70 ms); bookkeeping =
materialize + preprocess. Industry sources: `superfargoth/docs/research/far-vegetation-fill-2026-11-26.md`.

| Arm | whole frame | bookkeeping | prepasses | shadows | main opaque | Stands for |
|---|---|---|---|---|---|---|
| `stand` | **14.96** | 1.25 | 1.73 | 5.67 | 5.61 | BW-2's worst case, this time with focus (BW-2 said 13.97) |
| `stand/trees` | 7.58 | 0.02 | 0.78 | 4.33 | 2.15 | the 1,973 mesh trees alone |
| `stand/trees.opaque` | 3.44 | 0.02 | 0.29 | 2.74 | 0.10 | geometry leaves, no alpha test (Chlorophyll: trees fully triangles) |
| `stand/trees.noshadow` | 3.07 | 0.01 | 0.78 | 0 | 2.17 | upper bound of what a shadow LOD or proxy for near trees can save |
| `stand/cards` | 6.25 | 1.20 | 0.98 | 1.24 | 2.73 | the 179,901 cards alone |
| `stand/cards.unlit` | 5.08 | 1.20 | 1.00 | 1.29 | 1.29 | a cheap fragment shader (the cheap end of an impostor shader) |
| `stand/cards.noshadow` | 4.62 | 0.83 | 1.00 | 0 | 2.79 | far LODs and impostors cast into no cascade (UE per-LOD cast shadow off; far shadows from another system) |
| `stand/cards.unlit.noshadow` | 3.46 | 0.83 | 1.00 | 0 | 1.42 | both |
| `stand/cards.opaque` | 1.80 | 1.20 | 0.23 | 0.29 | 0.16 | no discard: the depth-only pipeline and early Z (what a mask-only prepass plus a depth-equal main pass, or an opaque shadow proxy, aims at) |
| `stand/trees.opaque/cards.unlit.noshadow` | 6.92 | 0.86 | 1.25 | 2.75 | 1.46 | a combined set; equals the sum of its two halves within 0.02 ms |

- **The alpha test is the cost, not the slot path.** Bevy 0.20 runs a `Mask` material's fragment shader in the depth
  prepass, in every cascade and again in the main pass, which compares `GreaterEqual` and discards again; only
  materials that never discard get the depth-only pipeline (research doc, part 2, with `path:line`). Without the
  discard the cards' fill falls from 4.95 to 0.67 ms and the trees' prepass + main from 2.93 to 0.40 ms.
- **The slot path meets its model once the content has no alpha test and casts nothing:** trees opaque without
  shadows 0.69 ms + cards opaque without shadows 1.15 ms = **1.84 ms** (model 1.75). Derived from measured spans
  (shadow spans drop to zero with `noshadow`; the preprocess drops 0.37 ms), not run as one arm.
- **Sets under the 3.5 ms limit** (sums of measured halves; the halves add up, see the last row): opaque trees
  without shadows + opaque cards with shadows = 2.49; + opaque cards without shadows = 1.84. **Not under:** any set
  that keeps the trees' shadows as full meshes (opaque trees alone 3.44, of which 2.74 is three cascades of 2.9 M
  triangles), and any set that keeps masked cards as they are (best 3.46 for the cards alone).
- **Bookkeeping:** 1.25 ms for 254 k slots with five views, 0.83-0.86 ms when the cards are in the camera view only
  (model 0.55): the camera view costs ~0.5 ms, each cascade ~0.09 ms.
- **Not priced (no arm):** a trimmed card hull (Persson, Avalanche: a 4- to 8-vertex hull keeps 52-60 % of the
  quad's area), a mask-only prepass with a depth-equal main pass that does not discard (UE
  `r.EarlyZPassOnlyMaterialMasking`; needs a material that specialises its main pipeline, or a patch to the vendored
  `bevy_pbr`: check which before BW-4), a shadow LOD for near trees (between 0 and 2.74 ms), the real impostor
  shader (frame blend and parallax cost more per pixel than the flat card). Hashed alpha and alpha to coverage fix
  quality, not fill, and alpha to coverage needs MSAA (off on the DLSS path).
- **Expected for the design below, not measured:** T0 with geometry leaves and a shadow LOD ~0.7 ms + its shadows;
  T1 impostors out of the cascades with a masked prepass and a depth-equal main pass ~2.0 ms (0.83 bookkeeping, 1.0
  prepass, ~0.2 main), ~1.6 ms with a trimmed hull; an opaque shadow proxy for T1 inside the shadow range ~0.3 ms
  (the opaque cards' shadow spans). About 2.5-3.3 ms at stand density: under the stop limit, over the 2.0 ms budget.

## BW-2c: depth-equal main pass for today's impostor (2026-11-26, bigworld-6)

- **Pipeline** (`sf_lod_bevy/src/depth_equal.rs`, `depth_equal_main_pass`, called from `ImpostorMaterial::specialize`
  when the material key's `depth_equal` is set): a descriptor that is not a prepass one (no `PREPASS_PIPELINE`
  vertex def and not labelled `prepass_pipeline`), whose mesh key has `DEPTH_PREPASS` and whose stock state writes
  depth, gets `depth_compare = Equal`, `depth_write_enabled = false` and the fragment def `DEPTH_EQUAL_MAIN`.
  Prepass and shadow descriptors, views without a depth prepass and blended passes stay as Bevy built them.
- **Shader:** `impostor.wesl` keeps its alpha test and its range dither only without `DEPTH_EQUAL_MAIN`. The def is
  our own, set with the state, so the shader can never drop the discard on a pipeline that still tests
  `GreaterEqual` (the spec's first wording, `@if(!DEPTH_PREPASS)`, would do that with the switch off).
- **Switch:** tunable `far.impostor.depth_equal` (`sf_lod_bevy::IMPOSTOR_DEPTH_EQUAL`, default on) ->
  `FarFieldSettings::impostor_depth_equal` -> `ImpostorMaterial::depth_equal` (pipeline key `ImpostorKey`), written
  by `update_materials` when it changes, so it is live. The game binds it in `quality_presets::add_gates`.
- **The open question, traced** (vendored `bevy_pbr/src/render/gpu_preprocess.rs`, `mesh_preprocess.wesl`,
  `build_indirect_params.wesl`): the early and late preprocess passes loop over every render phase's own work
  items with one shader, one view uniform and one depth pyramid per pass; a phase's main draw uses its own
  `early_instance_count + late_instance_count`. The main alpha-mask phase and the prepass alpha-mask phase therefore
  keep the same instances. They differ only through the CPU queue: `queue_material_meshes` and
  `queue_prepass_material_meshes` read separate specialization caches, and a draw whose pipeline is still compiling
  is skipped.
- **Counter:** `count_unprepassed` (render world, `RenderSystems::Cleanup`) reads the public
  `SpecializedMaterialPipelineCache`, `SpecializedPrepassMaterialPipelineCache`, `ViewKeyCache` and `PipelineCache`
  for the extracted impostor parts (one entity per far node with impostors) in every camera view whose key has
  `DEPTH_PREPASS`. It counts parts with a ready main-pass pipeline and no ready prepass pipeline, on screen or not,
  so it is an upper bound of missing draws. Gauge `far/impostors/no_prepass` when the diag counters exist; one
  warn line when an episode starts and one when it ends (peak, frames).
- **Tests** (`test -p sf_lod_bevy`, 11): descriptor tests (main with a depth prepass: `Equal`, no write, the def,
  nothing else changed; main without: stock; prepass and shadow: unchanged; blended: unchanged); a tripwire that
  the vendored `bevy_pbr` still writes the marks the hook reads; `shader_tests.rs` (new dev-dependencies `naga`,
  `wesl`): 20 def sets of both stages link and validate, the main pass has no `discard` under the def and has one
  without, both vertex stages place the card with the same `@invariant` expression and the camera prepass writes
  no `frag_depth`; the counter's rule.
- **Not done:** no ms in game (one `perf ab` on the live switch prices it); the bench arm is BW-2d.
