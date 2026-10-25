# Bake adapter: NIF importer, inputs, cooked models at runtime

Status: implemented (2026-10-11); world index and terrain products added (2026-10-11)
Crates: mw_bake (Bevy-free: importer, inputs, model bake, exterior bake,
texture decoding, report), mw_assets (cooked models as entities),
apps/bevyrrowind (`bake`, `view`)
Framework spec: `../../superfargoth/docs/specs/bake.md`

## Goal

Morrowind content goes through the framework bake: every model a record
names is imported from NIF into `sf_model::Model`, and static ones are
cooked into `.sfmesh` with a LOD chain. The viewer, and later streaming,
spawns cooked models and keeps the NIF path only for what cooking can't
represent yet (animation, particles, skinning, attached lights).
Non-goals: animated or skinned models in `sf_model` (until the framework
covers controllers), texture baking (M7).

## Canonical technique

The framework's (bake.md): constructive-trace build steps over a
content-addressed store, quadric simplification with meshoptimizer. This
adapter adds no technique of its own. It maps data, as glTF importers do in
any engine, and decides what is static. OpenMW does no offline cooking
(it builds scene graphs from NIF at load and caches them in memory), so
there was nothing to reuse there beyond the NIF semantics `mw_nif` already
covers.

## Importer mapping (`mw_bake::NifImporter`, version 1)

Input is `mw_nif::Model`: engine space (Y up, meters), properties
inherited, root transforms dropped (as M1 renders).

| NIF | `sf_model` |
|---|---|
| node (name, parent, TRS; uniform scale) | `Node` (scale copied to xyz) |
| geometry on a visible node | `MeshData`: positions, normals (area-weighted smooth normals when the NIF has none), first UV set, triangles |
| vertex colors | `colors`, only when the material's vertex color mode isn't `Ignore` (the cook fills white elsewhere: same result) |
| collision subtrees, hidden nodes (app-culled, `Tri EditorMarker`, inactive switch children), marker models (`MRK`) | node kept, no mesh |
| diffuse, emissive (rendered as sRGB since M1) | `base_color`, `emissive`, converted to linear |
| material alpha | `base_color[3]` |
| base texture | `base_color_texture` (VFS path as named; the runtime resolves `.dds`/`.ktx2` alternates) |
| alpha test / blend | `Mask(cutoff)` / `Blend` |
| additive blend | `Blend` + extra `nif.blend = additive` |
| stencil draw-both | `double_sided` |
| glow map | extra `nif.glow_texture` |
| vertex color mode, depth test/write off | extras `nif.vertex_colors`, `nif.depth_test`, `nif.depth_write` |

Only used materials are kept (re-indexed).

**Animated nodes.** A node is `animated` if a keyframe or visibility
controller targets it, if it is a billboard, switch/LOD, or particle node,
or if it draws a material that an alpha, material color, or UV
controller animates. Skinned NIFs mark every mesh `skinned`. Either one
makes `mesh.cook` skip the whole model. Keyframes that only gameplay plays
(no auto-play) count too, because scripts may play them.

**LOD policy.** Alpha-blended parts are `Keep` (few triangles, and
simplification shows on them). Parts whose bounding radius is under 2 % of
the model's are `Drop`: at LOD1's switch distance (about 10× the model
radius on the reference screen) they cover about 2 px. Everything else is
`Simplify`.

## Inputs (`mw_bake::VfsInputs`)

- Files: the winning VFS source. Loose files are keyed by their own stat,
  and archive members by the archive's stat plus the member path (`tes3`
  doesn't expose member offsets; the path is unique inside an archive). The
  memo lives in `<cache>/filehash.memo`.
- Records: hash of the resolved value's `Debug` form. This is a scaffold
  (`DebugRecordFingerprint`): it is deterministic for every TES3 kind except
  `CELL` (its references sit in a `HashMap`), so `CELL` is refused. Removal
  path: hash the winning raw subrecords plus patches once `mw_records`
  keeps them. No M2 step reads records yet.
- Extra records (`VfsInputs::with_extra_records`): kinds the TES3 database
  doesn't hold, fingerprinted by the host: `terrain.source_tile` (each
  `LAND`'s tile fingerprint, `MwTerrainSource`) and `terrain.layer` (the
  converted layer's `Debug` form).
- Profile: `mw-<hash of archive and content file names in load order>`.
- Cache root: `%LOCALAPPDATA%/bevyrrowind/bake` (`dirs::data_local_dir`),
  overridable with `--cache`.

## Exterior products (`mw_bake::world`)

`bevyrrowind bake` runs one `Baker` session: the model steps first, then
`bake_exterior` registers the framework's `world.*` and `terrain.*` steps
on the same baker, so one manifest names every product.

1. **Model bounds** from this session's `model.info` products (read in
   parallel: thousands of small store reads are latency-bound).
2. **`MwWorldSource::build`** with those bounds (world-adapter.md:
   `GROUNDED` from bounds, 134,661 grounded placements).
3. **World index**: `WorldBake` over the source with `IndexParams::default()`
   (128 m grid), exterior only. Interiors are not indexed yet:
   `WorldBake` takes one partition for every worldspace and interiors are
   `Single`; needs a per-worldspace partition in `sf_world` (open).
4. **Terrain**: `TerrainBake` with `MwTerrainSource`, `PlacementFootprints`
   of the grounded placements (`constrain` keeps them on the source
   surface), and `MwLayers` (every live `LTEX` + `@default`). Config:
   fill height = the sea floor (−2048 units), default layer `@default`,
   blend radius ¾ texel (5.49 m: the middle half of a texel is one
   texture, the rest ramps, like the original engine's blend maps; one
   texel washed paving out). A layer whose texture is missing (vanilla
   Tribunal's `tx_ma_sandstone02.tga`) draws the default texture.
   Layer arrays at 256² (vanilla land textures' size).
5. **Texture decoding** (`mw_bake::textures`) for the layer arrays. Reuse
   check: Bevy decodes DDS only for upload (BCn stays compressed); the bake
   needs RGBA8 on the CPU. `ddsfile` (MIT, already in the tree through
   `bevy_image`) parses the container, `texpresso` 2 (MIT, pure Rust)
   decodes BC1–BC3, `image` (in the tree) reads TGA/BMP/PNG. Uncompressed
   DDS formats (A8R8G8B8, X8R8G8B8, R8G8B8, ...) are swizzled by hand.

## Runtime (`mw_assets::cooked`, `view`)

- `view` uses a model cooked when the profile manifest has its
  `mesh.cook` product and `model.info` says it is neither animated nor
  skinned. Placements that carry a light keep the NIF path, because the
  light attaches to the `AttachLight` node and cooking flattens nodes away.
  `--no-baked` forces NIF everywhere.
- `CookedModelRoot` → once loaded, one child per LOD level and material
  with `VisibilityRange` bands from `sf_model::lod_ranges` (error × placement
  scale, reference screen 1440p/70°/1 px, last level ends at 2 km). A model
  that turns out skipped or fails to load falls back to its NIF.
- **Rest pose for the far field** (superfargoth weather/distant-shadows.md bug B2,
  2026-10-12): `model_baker` also registers `mesh.rest` (`MeshRestStep`,
  `sf_model::cook_rest`): the NIF importer's model with its `animated`
  flags ignored, i.e. the rest transforms; particle nodes have no mesh,
  inactive switch children stay hidden, skinned models stay `Skipped`.
  The far steps request it only for exterior far-eligible models whose
  `mesh.cook` is `Skipped`: 21 models, all lamps/fires/lava sparks/mist
  (`light_tikitorch00` 1,457 placements, 4,037 over all cells). `bake`
  prints the count and the top models. The runtime still spawns these
  models from the NIF; `NifSpawner`/`LightSpawner` put the placement's
  range on the NIF (`SubtreeRange`) and preload it before the cell
  reveals (`PlacementSpawner::prepare`). The Ghostfence segments are
  `ACTI` placements (not far-eligible) made only of blended parts, so they
  get no proxy yet (weather/distant-shadows.md B5).
- Materials go through the same `Surface → StandardMaterial` mapping as the
  NIF loader, so both paths look the same. `--lod-tint` colors levels
  (green, yellow, orange, red, magenta).
- Cooked meshes load with `sf_model_bevy`'s defaults: Bevy vertex
  compression, GPU-only. Positions of GPU-only meshes stay uncompressed,
  because Bevy 0.20's `Mesh::take_gpu_data` loses the AABB of compressed
  positions (found here; worked around in `sf_model_bevy`).

## Static collision (`mw_bake::collision`)

`model_baker` also registers `collision.cook` with the Morrowind rules
(`RootCollisionNode` geometry, `NC`/`MRK` markers, rest pose for animated
models, else the cooked LOD1). Rules, counts and timing:
`actors-adapter.md`, "Static collision as built". Report:
`cargo run --release -p mw_bake --example collision_report`.

## Animation (`mw_bake::anim`, as built 2026-10-12)

Framework steps (superfargoth animation.md "Bake steps"), Morrowind rules:

- **Sets** (`actor_sets`): `mw_actors` recipes over every `NPC_` and `CREA`
  (id order) give each skeleton, its graphs and its first-person view model.
  Stock humanoid skeletons play their `mw_graphs::HUMANOID_SETS` files; a
  custom NPC or `.1st` skeleton layers `x<model>.kf` over the race/gender set;
  creatures use `mw_graphs::creature_animations` (biped: `xbase_anim.kf`
  under their own). Vanilla: 96 skeletons, 182 files, 96 (graph, skeleton)
  pairs, 6,478 actors, 58 ms; one conflict (`anim_dancinggirl.nif` used by
  female and male NPCs: the first in id order is kept).
- **Phase 1**: `skeleton.cook` per skeleton (`mw_anim::NifAnimImporter`:
  `rig::skeleton_source`, so the product equals `rig::skeleton`) and
  `mw.anim.groups` per file (`AnimGroupsStep`: group name, duration, looping,
  loop range, sections from the text keys).
- **Phase 2** with `MwAnimRules` built from those products: `anim.cook`
  `<skeleton>|<file>|<Group[@section|@tail]>` for every layered group (the
  first file of the set that has it wins), sections and tails cooked when a
  graph names them; `rig.profile` hints are the humanoid rig map's profile
  (NPCs, first person, biped creatures) or `rig::creature_hints`;
  `animgraph.compile` `<graph>|<skeleton>` resolves references through the
  same layering, watches every file of the set plus missing `x<model>`
  candidates, and gets the humanoid bone → joint names for `JointRef::Rig`.
- **Builtin graphs**: `mw_graphs::builtin_source()` is a `MemSource` mounted
  first (lowest priority), so `ActorConfig` paths resolve through the VFS
  and a mod's file at the same path wins.
- **Loading** (`AnimLibrary::new(store, manifest)`, Bevy-free): `load(skeleton,
  graph)` / `load_recipe(&CharacterRecipe)` return `ActorAnimation` (skeleton,
  rig profile, compiled graph, clips in slot order); `groups(skeleton)` and
  `group_clip` serve PlayGroup/LoopGroup for any group of the set.

Numbers (real install, `tests/anim_real_install.rs`, 16 threads): cold
13.5 s (phase 1 0.45–0.6 s), 3,987 clips (3,307 whole groups + sections),
96 graphs, 3,024 clip slots; warm (sets + no-op) 0.31 s, nothing rebuilt;
all 96 graphs load in 0.8 s. Clips by rate: 30 Hz 2,220, 60 Hz 741, 120 Hz
1,004; 1,150 above the 0.1 mm bound (worst 3.6 cm). The `anim.cook` ≤ 5 s
gate is missed (see animation.md "Measured"). The 2,050 groups in the
182 files actors use (93 of the 162 KFs) match `mw_anim` file by file
(mw_anim's 2,330 counts every KF in the install).

## Verification

- `cargo test -p mw_bake` (importer mapping, dynamic parts, markers,
  normals, stat-keyed hashing, record fingerprints).
- `bevyrrowind bake` on the real install prints per-step hits, built,
  failures and timings, cooked and skipped counts with the top skip
  reasons, and LOD statistics. First and warm bake times are recorded in
  the roadmap session log.
- `cargo dev view "Seyda Neen, Census and Excise Office" --screenshot a.png
  --camera "1.0,4.2,5.0,1.0,3.6,-6.0"`, with and without `--no-baked`: same
  image. `--lod-tint` from far away shows the chain switching.
- `--bench benchmarks/census_office.json` compares both paths' frame times.

Measured (2026-10-11, Morrowind + Tribunal + Bloodmoon, 16 threads, release,
`mesh.cook` v3):

- 6,619 referenced models: 5,873 cooked, 736 skipped, 10 missing in
  vanilla. 94 % of placed references use a cooked model. Skip reasons by
  model count: skinned 410, gameplay keyframes 307, particles 224,
  auto-play keyframes 57, UV 42, material color/alpha 39, visibility 23,
  billboard 19.
- First bake 22 s (fresh cache; I/O-bound on about 26k store files), warm
  re-run 2.2 s (0.3 s of it validating traces).
- With the exterior (fresh cache, shared machine): **32 s total** (load
  0.3 s, models 18.4 s, model bounds 0.07 s, `MwWorldSource` 0.9 s, world
  index 2.7 s, terrain 7.8 s); warm re-run **2.4 s**. 16,471 products:
  1,559 `world.region`, 25 `world.page`, 1 `world.dir`; 376 level-0 terrain
  tiles, 179 pyramid tiles (levels 1–9), 555 macro and RT-proxy tiles, the
  layer table and 256² arrays. Step CPU (summed over threads): pyramid 32 s,
  tile 16 s, macro 13 s, RT proxy 9.5 s, pages 3.6 s.
- Dev-loop health check (2026-10-12, idle machine, release, Ryzen 7 7435HS
  16 threads, store on C:, NIF importer v2, collision and far field
  included; 26,208 products). **Cold** (fresh `--cache` dir): **47.7 s**
  (gate ≤ 2 min): load 0.27 s, models 22.4 s (CPU: mesh.cook 27.8 s,
  collision.cook 2.8 s, model.info 1.6 s), summary 0.46 s, `MwWorldSource`
  0.58 s, index 2.4 s, terrain 6.2 s (CPU: pyramid 24.4, tile 11.7, macro
  10.7, RT proxy 8.7 s), far field 15.2 s (CPU: lod.policy 12.5 s,
  lod.impostor 5.5 s). **Warm** (everything hits): **1.75 s** (gate ≤ 2.5 s;
  was 2.41 s: the summary decoded all 6.6k cooked models one by one, 1.03 s,
  now in parallel, 0.35 s): load 0.27 s, models 0.22-0.27 s, summary 0.35 s,
  `MwWorldSource` 0.55 s, index 0.03 s, terrain 0.06 s, far 0.06 s.
  The nifpop 71 s bake was a partial rebuild on a busy machine (rest-pose
  change re-ran the far field), not a cache regression: an importer version
  bump re-cooks every model (38 s here, all 6,609 `mesh.cook`/`model.info`).
  The store for one profile is 793 MB in 48k files; the live
  `%LOCALAPPDATA%/bevyrrowind/bake` had grown to 2.5 GB (old products; no
  `bake gc` subcommand yet). Parallel efficiency is low: models use ~32 s
  of step CPU in 22 s wall, the far field ~18 s CPU in 15 s wall on 16
  threads (time outside `run`: fingerprinting, store writes, per-level
  barriers in `FarBake::bake_worldspaces`); the first lever if the cold
  bake must get faster. `bake` now prints every step's stats.
- 3,971 of the 4,079 models with LOD0 ≥ 128 triangles have a chain. LOD1
  keeps 61 % of LOD0's triangles, LOD2 37 %, LOD3 21 %, LOD4 11 %.
- Census Office (`benchmarks/census_office.json`, RTX 4060 Laptop, 720p):
  cooked and NIF paths render the same (PSNR 50 to 52 dB, against 57 dB
  run-to-run noise from light flicker). Frame time is the same too (about
  3.3 to 3.5 ms median, CPU-bound), with about 13 % fewer opaque vertex
  shader invocations on the cooked path. Load-to-ready time is 0.7 to
  1.3 s on both paths.

## Industry comparison

J9 (superfargoth `industry-audit.md`), 2026-10-12, docs only (`.sessions/j9rest.md`).
The pipeline itself is compared in superfargoth `bake.md` ("Industry
comparison"). Adapter-level: legacy content runs through an importer that
flattens and cooks statics, like UE's Interchange/Datasmith or Unity's
model importer (bake once, load cooked data); OpenMW parses NIF at load and
caches scene graphs in memory (no offline cooking); the community's Skyrim
SE "NIF optimizer" and xLODGen/DynDOLOD are the Bethesda-world precedent for
an offline pass over shipped content. **Match** (offline cook), no gap.

## Open questions

- Lights on cooked models: carry named attachment points (`AttachLight`)
  through `sf_model`/`.sfmesh`, so lamps can cook too.
- Gameplay-only keyframes block cooking for 307 models (2.2k placements).
  They could cook in their rest pose, with the NIF path used only once a
  script plays them.
