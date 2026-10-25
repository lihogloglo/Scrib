# World adapter (Morrowind → sf_world, sf_terrain)

Status: implemented (2026-10-11): records, `WorldSource`, `TerrainSource`; baked by `bevyrrowind bake` and streamed by `view <exterior>` (2026-10-11)
Crates: `mw_world` (adapter), `mw_records` (per-file LTEX index slots)
Framework specs: `../superfargoth/docs/specs/streaming.md`, `terrain.md`, `world-coordinates.md`

## Goal

Express the loaded Morrowind content in the framework's world model:
`Worldspace` and `DoorLink` records, placements through `sf_world::WorldSource`,
and heightfield tiles through `sf_terrain::TerrainSource`, with identities
that survive load-order changes and edits, so bake products and saves stay
valid. Non-goals: spawning (that's `sf_world_bevy` plus the content pack's
`PlacementSpawner`s), leveled-list resolution (M7), portal geometry (M8).

## Canonical technique

The framework side is UE5-style world partition (streaming.md). The adapter
side follows the original engine's data semantics, taken from OpenMW (GPL
reuse is fine in this repo): `components/esm3/loadcell.cpp` and
`apps/openmw/mwworld/store.cpp` (references, moved references, land
texture scoping), `components/esm3/loadland.cpp`, `components/esmterrain`,
and `components/terrain` (heights, texture grid, triangulation). No crate
covers this mapping; `tes3` parses the records.

## Data model

### Identity (`mw_world::ids`)

| Thing | Id |
|---|---|
| Exterior worldspace | `@exterior` (`@` never starts an interior name, same rule as `mw_records::keys`) |
| Interior worldspace | the interior cell's name (case-insensitive, = its cell record id) |
| Region | `RegionId::from_key(cell record id)`: `@x,y` for exterior squares, the name for interiors |
| Placement | `PlacementId::from_key("mw.ref", lowercase(file) ‖ 0 ‖ refr_index_le)` |
| Door link | `mw.door/<placement id of end a>` |
| Source tile | `SourceTileId(x as u32 << 32 | y as u32)` of the exterior grid square |

**Reference keys.** A `FRMR` is a file-local master index plus a 24-bit
reference number. The original engine and OpenMW (`RefNum` = content file +
index; saves map content-file indices through file *names*) identify a
reference by the content file that first placed it plus that number.
`mw_records` already re-keys every reference to `(SourceId of the defining
file, refr_index)`; the adapter turns the `SourceId` into the file name.
Consequences: a plugin that edits, moves, or deletes a master's reference
addresses the same id; references a plugin adds carry the plugin's name;
reordering plugins changes nothing; renaming a content file changes its
references' ids (as it breaks saves in the original engine). Measured on
Morrowind + Tribunal + Bloodmoon: 357,692 placements, 0 duplicate ids.

### Worldspaces (`mw_world::worldspaces`)

- Exterior: `Worldspace { name: "Morrowind", partition: Grid { shift: -1 }
  (128 m), has_terrain: true, sea_level: Some(0.0), environment: None }`.
  Sky and weather come from `sf_sky`, not a per-worldspace preset.
- One per interior cell: `partition: Single`, `sea_level` = `WHGT` in meters
  when the cell has water, `environment` → an `mw.cell_environment`
  record (same id as the cell) holding `AMBI` (ambient, sunlight, fog color
  as sRGB 0..1, fog density), "behaves like exterior" (sky), water level.
  `CellEnvironment` is an adapter kind until the framework defines an
  environment/lighting preset kind (sky-lighting work); then it maps onto it.
- `WorldRecords::define_into(db, source)` registers the `sf_world` kinds
  and defines everything under an adapter source applied after the content
  files, so mods can patch them like any record.

### Door links

A door reference with `DODT` (+ optional `DNAM`) is a door end: no `DNAM`
means the exterior. Pairing: door j pairs with door i when j stands in i's
destination worldspace, i in j's, j is within 8 m of i's arrival point and
i within 8 m of j's. Candidates are accepted greedily by the sum of both
distances (then by id), so the result is order-independent. Paired doors
become one `DoorLink { two_way: true }`: each end's `door` is its own
placement, `arrival` is the *other* door's destination pose, and `frame`
sits at the door with +Z along the arriving traveler's facing (point links
for M2; M8 derives real portal frames from door geometry). Unpaired doors
become one-way links whose far end is the arrival pose with no door.

Threshold from the vanilla data: the best back-pointing candidate's
distance sum is ≤ 8 m for 3,369 of 3,610 doors, ≤ 16 m for 3,584, then a
thin tail. Measured (Morrowind + Tribunal + Bloodmoon): 3,619 door
references, **1,791 two-way links (3,582 doors), 37 one-way, 0 dangling**.
The one-way ones are mostly real one-ways (Vivec puzzle canal exits, jail
doors, Mortrag Glacier's sequence, a test cell), plus a few sloppy pairs
(Seyda Neen census office back door: 7.3 m / 13.9 m) and second doors
that point at an already paired door (Druscashti, Telvanni vault).

### Portal frames: feasibility (`mw_world::portal`, M8 prep, 2026-10-12)

Morrowind has no portal data: doors are point links. `mw_world::portal`
derives `LinkEnd { frame, extent }` from door geometry, Bevy-free, without
changing the baked `DoorLink` yet. Report:
`cargo run --release -p mw_world --example portal_report`; test
`door_links_have_portal_frames` (`tests/real_install.rs`).

**Derivation (adapter knowledge, stays here).** Slab = the door model's
visible bounds (`mw_nif::visible_bounds`, engine space, unit scale) times
the reference scale. Thinnest axis = door normal; of the other two, the
more upright (in world space after the reference rotation) is the height,
the other the width. A normal with |y| > 0.7 is a horizontal door
(trapdoor, hatch). A slab thicker than 0.35 × width is `Thick` (cave
mouths, sewer grates in frames, dwemer hatches): its normal is the
horizontal axis the arrival lies along, since the thin axis lies. The
normal's sign comes from the arrival marker that lands at this door (the
partner's destination): travelers appear on the front, so +Z points into
the door's own worldspace, as `LinkEnd::frame` requires. Model-axis
conventions don't work for this: 31 door models are placed facing both
ways (in_velothismall_ndoor_01 263/256), so one-way doors (no landing
marker) have an unknown front. **Frame origin = the opening's bottom
center (sill)** on the slab's mid-plane, X along the width, Y up; the
opening spans x ∈ ±w/2, y ∈ 0..h. Sill rather than center so floors line
up across a size mismatch (proposed for streaming.md: "extent rises from
the frame origin"). Hinge = model origin along X as a fraction of the
width: 956 of 3,000 slabs have it at an edge (swinging doors), load doors
mostly centered, so the hinge is not needed for portals.

**Install numbers** (Morrowind + Tribunal + Bloodmoon; 3,619 doors with a
destination, 118 door models, all with bounds): 1,791 two-way links,
37 one-way, **252 ambiguous doors** (2+ back-pointing candidates within
8 m; pairing takes the closest, e.g. Balmora shop doors next to each
other). Shapes: 3,000 slab, 400 thick, 219 horizontal, 0 missing.

| Class | Links | Rule | Examples |
|---|---|---|---|
| portal-ready | **1,361** | two-way, both upright slabs, openings within 15% per dimension | Balmora, Milie Hastien ↔ (-4,-2); Pelagiad, Mebestien Ence ↔ (0,-7); Vivec waistworks pairs |
| portal-with-adjustment | **251** | 117 size mismatch 15%-100% (63 of them are the *same model at a different reference scale*: interiors scale their doors up, e.g. Census Office ×1.27); 134 with a thick end (caves: (4,5) ↔ Halit Mine; tomb and Kogoruhn inner doors) | Seyda Neen Census and Excise Warehouse ↔ (-2,-9) ×1.70; Tel Mora upper tower ×1.38 |
| fade-only | **216** | 37 one-way (prison markers, Vivec canal exits, test cells), 174 with a horizontal end (ship trapdoors, Pelagiad cellar trapdoors, dwemer down-hatches), 5 openings > 2× apart | Arkngthand ↔ (0,-2) ×2.83; Solstheim, Solvjord ×3.95 |

Size ratio of two-way pairs (≤1.05, 1.15, 1.3, 1.6, 2.0, more):
1,233 / 287 / 76 / 47 / 75 / 73. "Adjust" means: portal extent = the
smaller opening per dimension, sill-aligned; the bigger side keeps its
door slab drawn around the opening (or the slab swings open, revealing a
rectangle no larger than the partner's). A rigid link can't make a
1.27× door match without a scaled (non-rigid) portal transform, which
`DoorLink::cross` deliberately doesn't do (the player must not resize).

**Arrival markers.** Forward = `rotation · (engine −Z)` (Morrowind +Y,
clockwise-positive yaw, as `mw_nif::coords::reference_rotation`): with
it, 3,307 of 3,363 arrivals at upright doors face straight out of their
door (within 60°), 54 sideways, 2 back at the door; 0 lie behind their
door; distance in front: 1-4 m for 3,234, > 4 m for 89. Only 8 markers
carry pitch/roll. **Markers float**: the marker sits 0.75-1.75 m above
the door's sill for 2,890 of 3,363 (median about 1.2 m), so it is not a
foot position. The original engine (and `OpenMW`) let the actor fall to
the floor; an engine that puts the camera at marker + eye height starts
about 1.2 m too high, and must drop the arrival onto the ground (a down
probe against static collision) before placing the eye. Suspicious
markers (sideways at upright doors, from the report's list): 8 ancestral
tombs and the Redoran vaults/prison cells with `in_velothismall_ndoor_01`
(about 90°, authored that way, likely facing along the entry corridor),
Balmora (-3,-2) and Ebonheart/(2,-13) street doors (90°), Caldera,
Surane Leoriane's house (101°), (15,5)/(18,3) Telvanni doors, Seyda Neen
lighthouse, ship cabin doors at (18,-1)/(15,11) (173°, facing back), Tel
Vos northeastern tower trapdoor (174°). "Facing into a wall" needs a ray
against the interior's collision; not done here (next step for the
camera job).

**Census Office (spike target).** The office has four two-way doors, all
`ex_nord_door_01`, all *adjust* (interior doors scaled 1.20-1.27×). The
street exit is **`CharGen Exit Door`** (interior) ↔ **`chargendoorjournal`**
(exterior (-2,-9)):

| End | Sill frame (engine m, absolute) | +Z (into its worldspace) | Opening w × h, thickness |
|---|---|---|---|
| interior `CharGen Exit Door` | (16.730, 2.278, 10.741) | (0, 0, −1), yaw 180° | 2.077 × 3.569, 0.258 |
| exterior `chargendoorjournal` | (−147.068, 2.096, 1023.539) | (0.707, 0, −0.707), yaw 135° | 1.636 × 2.810, 0.203 |

Size ratio 1.27 (same model, scale 1.27 vs 1.0). Crossing interior →
exterior is `b.frame · R_y(π) · a.frame⁻¹`: rotation +135° about Y,
interior sill ↦ exterior sill. Authored arrivals: exterior 2.01 m in
front, 1.22 m above the sill, facing out (cos 1.0); interior 2.02 m in
front, 1.61 m above, facing out. Stepping 0.5 m through the interior
door lands 0.5 m in front of the exterior door at sill height, facing the
same way as the authored arrival (0.0° apart; 1.94 m from it, which is the
marker's 2 m stand-off and float). The other three pairs ((-2,-9)
`chargen door captain`, `chargen customs door`, `ex_nord_door_01` ↔
`chargen door exit`) print in the report.

**Next step (wiring, not done: the app is mid-build on the current
format).** `mw_bake::world` already builds `lookup` from `model.info`
bounds (`model_bounds`, line ~187) right before `WorldRecords::build(db)`.
Add `WorldRecords::build_with(db, Some(&lookup))` that, after
`link_doors`, runs `portal::analyze` (with `portal::door_models(db)`)
and, for each two-way link that is not fade-only, sets both ends'
`frame` to the derived sill frames and `extent` to the per-dimension
minimum of the two openings (same on both ends); fade-only links keep
today's arrival frame and `extent = 0`. `arrival` stays the authored
marker (grounded at runtime). Check first that `model.info` bounds equal
`mw_nif::visible_bounds` for door models (collision excluded). No
`sf_world` type change; one streaming.md line: the extent rises from the
frame origin. If the bake caches link records by input fingerprint,
include the door model bounds in it so links rebuild when a door model
changes.

### Placements (`mw_world::source`)

`MwWorldSource::build(db, options)` snapshots every cell: one region per
cell record (and per exterior square that only receives moved references),
placements sorted by key (file, refr_index), fingerprint = blake3 over the
canonical bytes of the region's resolved placements, so a changed base
record (mesh, class) changes the fingerprint too, not only reference edits.

| Base kind | Class | Notes |
|---|---|---|
| `STAT` | `Static` | editor markers (`EDITOR_MARKERS`) → `Marker` + `EDITOR_ONLY` |
| `DOOR` | `Door` | |
| `LIGH` | `Light` | carried model if any; light parameters come from `mw_world::LightInfo` |
| `CONT` | `Custom(1)` (`source::CONTAINER`) | includes harvestable flora |
| `ACTI` | `Custom(2)` (`source::ACTIVATOR`) | |
| `NPC_`, `CREA`, `LEVC` | `Actor` | leveled lists have no model until resolved |
| items, `LEVI` | `Dynamic` | |

`base` = `RecordRef { kind: <four-letter tag>, id }`; `model` = `meshes/...`
VFS path. Transforms convert once (`mw_world::convert`): Morrowind
`(x, y, z)` units → engine `(x, z, −y) · 0.9144/64` m computed in f64 and
stored straight into `WorldPos` (no f32 world coordinate is ever formed;
the only rounding is the source's own f32 and the f32 cell offset, ≤ 7.6 µm;
`WorldPos` round-trips measured exact). Rotation: `mw_nif::coords::
reference_rotation` (verified visually in M1). Scale: uniform `XSCL`.

**`GROUNDED`** (exterior only; statics, doors, containers, activators, and
lights with a model): with model bounds, when the placement's world AABB
straddles the source terrain surface under its pivot (bottom ≤ ground +
0.5 m and top ≥ ground − 0.5 m); without bounds, when the pivot is within
1 m of the surface. Ground height uses the source triangulation. Measured
with NIF bounds: 134,661 grounded (Static 111,462 of 119,122 exterior,
containers 21,409 of 22,677, doors 600 of 1,211); the pivot rule finds only
65,698, so the bake should pass `model.info` bounds.

**Moved references (MVRF).** A plugin that moves a master's exterior
reference to another grid square writes, in the *original* cell's record,
`MVRF` (the reference) + `CNDT` (destination grid) followed by the full
`FRMR` with its new data (OpenMW's `Store<Cell>::load` leases it to the
target cell, and the source cell skips it). `tes3` attaches the `CNDT` grid
to the following `FRMR` (`moved_cell`), `mw_records`' cell merge keeps it,
and the adapter delivers the reference from the **destination's region**
(creating that region if no `CELL` record exists there). Positions are
absolute in exteriors, so nothing else changes; the index assigns cells by
position anyway. `MVRF` in interiors is meaningless and ignored (counted).
Vanilla Morrowind + Tribunal + Bloodmoon contain **no** `MVRF` (plugins use
it); the synthetic test covers it. Known limit: a later plugin that edits a
moved reference again without repeating `MVRF` returns it to the original
cell's region; harmless for streaming (position decides the cell), it only
affects which region's fingerprint changes.

### Terrain (`mw_world::terrain`)

`MwTerrainSource` exposes one source tile per exterior `LAND`:

- **Heights**: 65 × 65 vertices, 128 units (1.8288 m) apart. `VHGT` decodes
  as OpenMW does: float offset, then int8 deltas; each row starts from the
  previous row's first vertex, each vertex from its west neighbor; × 8
  units. A `LAND` without the heights flag is flat at −2048 units (OpenMW's
  `DEFAULT_HEIGHT`, also `terrain::fill_height()` for squares without
  `LAND`). Engine tiles start at the cell's north-west vertex, rows step +Z
  (south), so Morrowind rows are flipped. Borders are shared with the
  neighbors (measured: all 1,478 east-west shared edges agree exactly).
- **Triangulation**: Morrowind splits quad (i, j) along (i, j)–(i+1, j+1)
  when i + j is even, along the other diagonal when odd (OpenMW's
  "diamond" index buffer; its Bullet heightfields mirror it). The row flip
  turns this into `Triangulation::Alternating(Diagonal::Main)` in engine
  tile terms; a test checks the engine mosaic reproduces the Morrowind
  surface within 1 mm everywhere.
- **Tint**: `VCLR` RGB per vertex on the height lattice (absent without the
  flag: no tint).
- **Layers**: `VTEX` 16 × 16, stored as 16 blocks of 4 × 4 (transposed on
  load, OpenMW's `transposeTextureData`). 0 = default texture
  (`textures/_land_default.tga`, layer `@default`); n = the `LTEX` with
  index n − 1 **in the file that defined the winning `LAND`**: indices are
  per plugin (OpenMW keeps `(plugin, index) → id`, then `id → texture`,
  latest definition wins). `mw_records` therefore stores one
  `mw.ltex_slot` record per (file, index) (`keys::ltex_slot`), first
  definition in a file wins, deleted LTEX add none. Placement quirk:
  OpenMW samples the texel one to the west and nudges its blend map a
  quarter texel "to look like vanilla", which puts texel (sx, sy) over
  x ∈ [sx + ¼, sx + 1¼), y ∈ [sy − ¼, sy + ¾) texels (512 units) from the
  cell's south-west corner: one vertex east and one vertex south of the
  naive grid. The `SourceLayers` origin carries that offset; every cell has
  the same one, so tiles still share one lattice. Checked 2026-10-11
  against OpenMW's current source: `gridsampling.hpp` (`minRow … - 1`: the
  column to the west, x only), `buffercache.cpp` (chunk UV u = east,
  v = north), `material.cpp` (blend-map nudge `(+¼, −¼)` texel after the
  `16/17` scale that puts blend texel k at k/16). Seen on screen
  (top-down Pelagiad at 70 m, against the naive grid and a 3-vertex-south
  variant): paving follows Pelagiad's and Balmora's streets. The bake
  blends with a ¾-texel tent (bake-adapter.md); the renderer's height
  blend keeps boundaries at texel midpoints.
- **`TerrainLayer`s**: one per live `LTEX` (id = LTEX id) plus `@default`;
  `albedo_height` = the texture through the VFS (`.dds` preferred when a VFS
  is given), `tiling_m` = 7.3152 (one repeat per texel, as vanilla),
  no normal/ARM yet, `surface` = `sf.surface:<name>` from `SURFACE_RULES`
  (keyword table on id + file name: snow, ice, ash, sand, mud, stone, grass,
  rock, dirt). Scaffold `TerrainLayerDef`: `sf_terrain::bake::TerrainLayer`
  (kind `terrain.layer`) is still in progress behind `sf_terrain`'s `bake`
  feature (which doesn't build from this workspace yet: it needs
  `sf_records`' serde feature); the bake host converts field by field until
  `terrain_layers` can return it directly.
- **Fingerprint**: blake3 of the produced tile (heights, palette, texels,
  tint), stable per cell; bake inputs are `terrain.source_tile` records
  keyed by tile id.

## Interfaces

```rust
// mw_world
WorldRecords::build(&RecordDb) -> WorldRecords        // worldspaces, environments, door links, DoorStats
WorldRecords::define_into(&mut RecordDb, SourceId)
MwWorldSource::build(&RecordDb, &SourceOptions) -> MwWorldSource   // impl sf_world::WorldSource
SourceOptions { model_bounds: Option<&dyn Fn(&str) -> Option<([f32; 3], [f32; 3])>>, .. }
MwTerrainSource::new(D: Deref<Target = RecordDb>)    // impl sf_terrain::TerrainSource
terrain::terrain_layers(&RecordDb, Option<&Vfs>) -> Vec<(RecordId, TerrainLayerDef)>
terrain::fill_height() -> f32
```

The bake (`mw_bake`) hands `MwWorldSource` to the `world.*` steps and
`MwTerrainSource` to the terrain steps directly (streaming.md: no terrain
hook on `WorldSource`).

## Verification

- `cargo test -p mw_world -p mw_records`: synthetic master + plugin
  (classes, markers, grounding, MVRF re-homing, stable ids across load sets,
  fingerprints change only where content does, per-file LTEX resolution,
  door pairing/one-way/dangling, `VHGT` accumulation, `VTEX` transposition,
  engine tiles reproduce the source surface, texel offset) and
  `tests/real_install.rs` (skips without an install).
- `cargo run --release -p mw_world --example world_report`: the numbers
  below.

Measured 2026-10-11, release, Morrowind + Tribunal + Bloodmoon (GOTY,
Steam), shared machine:

| Step | Result | Time |
|---|---|---|
| Load 3 ESMs | ~62k records | 0.27 s |
| `WorldRecords::build` | 1,329 worldspaces, 3,619 doors → 1,791 two-way + 37 one-way | 14 ms |
| NIF bounds pass (`mw_nif::visible_bounds`, rayon) | 4,758 of 4,768 models | 0.22 s |
| `MwWorldSource::build` (with grounding) | 2,887 regions, 357,692 placements | 0.5–0.6 s |
| Exterior index (`build_index`, 128 m cells) | 156,778 placements, 1,692 cells, 25 pages | 0.12 s |
| Encode pages + directory | 10.3 MB (65.8 B/placement) + 66 KB | 0.05 s |
| All 1,328 interiors indexed (`Single`) | | 0.12 s |
| `MwTerrainSource::new` (decode + fingerprint every LAND) | 1,540 tiles, 131 layers used, 0 unresolved | 0.07 s |
| Read all tiles + one `SourceMosaic` | ok | 0.05 s |

Baked and on screen (2026-10-11; `bevyrrowind bake`, then
`cargo dev view "Seyda Neen"`, `"Balmora"`, `"Pelagiad"`, `"2,7"`; any
exterior cell name or grid streams the whole exterior):

- `cargo run --release -p mw_bake --example ground_check`: the baked
  level-0 terrain against Morrowind's own surface (`HeightSampler`: raw
  `LAND` heights on the alternating triangulation, Morrowind axes), at
  every exterior placement pivot. **Grounded (134,661): median 0.4 mm,
  p99 3.2 cm, mean +0.3 mm** (95 over 10 cm, max 41 cm: pivots at a
  footprint's feathered edge). Not grounded (22,111): median 2.7 mm, p99
  9.4 cm, max 84 cm (the clamped Catmull-Rom upsample). This checks the
  axis conversion, the tile origin and row flip, the triangulation mapping,
  and `constrain` together: any of them wrong shows as decimeters to meters.
- Screenshots: objects sit on the terrain at Seyda Neen's docks, Balmora,
  Pelagiad; streets' paving follows the streets (texture offset above);
  terrain LOD colors + wireframe over Balmora show no cracks.

Placements by class: Static 224,634; Custom(1) containers 42,044; Dynamic
40,010; Light 21,758; Actor 12,276; Custom(2) activators 10,913; Door 4,739;
Marker 1,318. Largest exterior cells: 661 placements ([-18, 1, -21]),
569 ([-2, 1, 2]). Largest interiors: Old Mournhold: Forgotten Sewer 316 m
across, Mortrag Glacier: Huntsman's Hall 249 m, Mournhold Temple
Courtyard 232 m.

## Industry comparison

J9 (superfargoth `industry-audit.md`), 2026-10-12, docs only (`.sessions/j9rest.md`).
This adapter maps data; the techniques are compared in superfargoth
`terrain.md`, `streaming.md` and `far-field.md`.

| Topic | Original engine / OpenMW | Ours | Verdict |
|---|---|---|---|
| Terrain heights | vanilla and OpenMW render the 65×65 `LAND` grid (1.83 m) directly; Skyrim-era Creation uses 33×33 per cell at the same spacing plus offline terrain LOD meshes | decoded exactly as OpenMW, upsampled to 0.5 m by clamped Catmull-Rom, `constrain` under placements | **differ** (finer, smooth; within 1 mm at source vertices) |
| Terrain layers | vanilla vertex-blended 16×16 texels; OpenMW blend maps per layer + composite maps for distance | same texel placement as OpenMW, height-blended splats + macro albedo | **match** placement; vanilla textures carry no height channel, so the height blend degrades to a weight blend until PBR/height generation (M7) |
| Distant objects | vanilla none; OpenMW object paging (runtime merge); MGE XE distant statics (offline-generated) | baked HLOD L1–L3 | **match** MGE/UE-style offline HLOD |
| Interiors | separate cells behind load doors (all Bethesda, OpenMW) | separate worldspaces, prefetch + fade; M8 portals | **match**, portals beyond |
| Runtime enable/disable of refs | per-ref `enable`/`disable` from scripts | adapter maps to the framework's overlay; groups are superfargoth streaming.md open question 2 | see there |

Sources: cited in "Terrain" above (OpenMW source files) and in the
superfargoth specs.

## Open questions

1. Large interiors as `Single`: six exceed 190 m. Fine for M2 (interiors
   are small in bytes); a grid partition for interiors over ~256 m is a
   one-line change in `WorldRecords::build` if streaming tests ask for it.
2. Texture-grid offset: settled (OpenMW source + screenshots, above).
3. `CellEnvironment` and `sf.surface` kinds await their framework owners.
4. Grounding of lights/activators on posts and walls: the AABB rule grounds
   anything touching the surface; the constrain step only keeps terrain
   under footprints at the source surface, so over-grounding is harmless.
