# Worldgen adapter (Morrowind → sf_worldgen)

Status: **plan** (owner 2026-11-18). No code. Framework spec: `../../../superfargoth/docs/specs/worldgen.md` (goal,
pipeline, framework slices, order). Crate (planned): `mw_worldgen` (Bevy-free); touches `mw_world` (anchor references
at new positions), `mw_script` (scripted coordinates), `mw_bake` (dump, `TerrainSource` wiring). Related:
`world-adapter.md` ("Terrain"), `water-adapter.md`.

## Goal

Owner (2026-11-18): **Vvardenfell at 10×** first, then Tamriel Rebuilt's data (other environments) through the same
adapter. The human-made content (settlements, shipwrecks, ancestral tombs, mines, shrines, camps, ...) stays as
it is; the nature between it is generated. Red Mountain becomes a real mountain. Settlement growth along roads is
later (or mods).

## Sources

Any ESM/ESP load order through `mw_records` (Morrowind, Tribunal, Bloodmoon, Tamriel Rebuilt, Project Tamriel
data): `LAND` heights (65² per 117 m cell, ~1.8 m spacing) as the intent guide, `REGN` for the region map, `CELL`
water levels, `LTEX` for the layer mapping, exterior `REFR`s for anchors and the placement statistics that drive
nature scatter. Nothing derived from them is shipped; the recipe runs in the player's bake (framework spec
"Constraints").

## Source numbers (WG-R2 terrain half, 2026-11-18, `worldgen-dump` on the owner's install)

Morrowind + Tribunal + Bloodmoon: mosaic 3329 × 3073 samples at 1.8288 m, cells (-28,-19)..(23,28) (52 × 48 =
6.1 × 5.6 km, Solstheim included), 1,540 LAND cells (21.1 km²), 104 LAND without heights, 0 disagreeing shared edges.
Heights -30.7..270.8 m; empty cells filled at -29.26 m. Vvardenfell **above sea: 10.9 km², 5.6 × 5.0 km**.
**Summit 270.8 m** (cell (3, 9), Red Mountain Region). At S = 10 with vertical ×10: ~56 × 50 km of land and a 2.7 km
Red Mountain (Etna 3.3 km), so a uniform ×10 is already plausible.

| Region | Cells | km² | Min | Median | p95 | Max (m) |
|---|---|---|---|---|---|---|
| Red Mountain | 37 | 0.5 | 15.7 | 154.2 | 242.1 | 270.8 |
| Ashlands | 162 | 2.2 | -10.1 | 32.7 | 113.7 | 198.0 |
| Molag Mar (Molag Amur) | 123 | 1.7 | -20.5 | 26.9 | 69.4 | 112.5 |
| West Gash | 193 | 2.6 | -30.7 | 7.8 | 45.0 | 102.1 |
| Grazelands | 57 | 0.8 | -13.6 | 16.3 | 42.9 | 83.0 |
| Bitter Coast | 166 | 2.3 | -29.4 | -15.2 | 17.7 | 80.8 |
| Ascadian Isles | 116 | 1.6 | -30.6 | 0.2 | 27.3 | 71.6 |
| Azura's Coast | 379 | 5.2 | -29.3 | -9.3 | 22.3 | 67.3 |
| Sheogorad | 170 | 2.3 | -29.3 | -10.5 | 23.0 | 62.6 |
| Solstheim (6 regions) | 146 | - | - | - | - | 68.6 |
| (no region: sea) | 947 | 13.0 | -29.3 | -29.3 | -29.3 | -25.1 |

## Regions → recipes (draft; the WG-S2 look test settles it)

| REGN | Landform recipe (draft) | Real-world reference |
|---|---|---|
| Red Mountain Region | stratovolcano, summit caldera, radial lava channels (foyadas) | Etna, Teide, Fuji |
| Ashlands | basalt flows, tephra plains, lava tubes | Iceland interior |
| Molag Amur | young lava fields, fissures, spatter cones | Hawaii, Lanzarote |
| West Gash | eroded highlands, river canyons | Scottish Highlands (TBD) |
| Bitter Coast | swamp delta, braided channels | river deltas (TBD) |
| Ascadian Isles | lake lowlands, river islands | (TBD) |
| Grazelands | plateau grassland, escarpments | (TBD) |
| Azura's Coast | drowned coast, sea stacks, islands | (TBD) |
| Sheogorad | archipelago of rocky islets | (TBD) |
| Solstheim (Bloodmoon) | glacial valleys, fjords | Norway, later |

Foyadas exist in the source as valleys; their channels go in as river/lava strokes so erosion keeps them.

## Anchors: what counts as human-made

A reference is human-made if any of these holds: a building or structure mesh (by mesh path class), a door, a
container, a book or note, an NPC or creature placed by hand, a light, an activator, a script, or a quest or
dialogue reference to its id. Human-made references cluster into anchors (radius a tunable). Flora (`flora_*`)
and terrain rocks (`terrain_rock_*`, the large cliff-faking meshes) are nature: dropped and regenerated. WG-A1
prints a census by class and the ambiguous classes (ruin fragments, lone bones, signposts, roadside rocks) for the
owner to rule on.

Hard cases: the **Ghostfence** (its pylons ring Red Mountain; at 10× the ring is regenerated along a path, not
stamped), **roads** (signposts are anchors; roads are re-routed later, until then the road LTEX stays only inside
footprints), **anchors on slopes** (terracing or retaining in the stamp), **Dwemer ruins** spread over large areas
(one anchor per cluster, transition bands between them).

## Coordinates through the anchor transform (WG-A3b)

The game data stays unmodified; at load the adapter maps every exterior source coordinate through the anchor
transform: door destinations from interiors to exteriors, `PositionCell`/`Position`/`SetPos` and AI package
targets (`AiTravel`, `AiEscort`, `AiFollow` with coordinates), travel service destinations, Divine/Almsivi
Intervention markers, Mark/Recall, and saved positions (saves store world positions plus the recipe hash).

## Prior art

- **Procedural Tamriel** (`procgen/Liam Mello/procedural_tamriel_bundle_2026-10-19.zip`, untracked, Python,
  2026-08 to 10): fills the gaps between Project Tamriel's LAND (TR, Skyrim HotN, Cyrodiil) at vanilla scale and
  writes masterless ESMs. Terrain corpus, seam continuation, multifractal + exemplar pull-back, physical erosion over
  230 M vertices (the signed-F field bake: 1,738 s), region assignment, climate rock palettes, scatter, roads (its
  grade-first router was rejected at scale), cityforge and interiors. The closest prior work. **Its author shared it
  with the owner (2026-11-18): ideas and techniques may be taken; we reimplement in Rust, never port the Python.**
  Its docs mark their own unverified claims; keep that distinction.
  Lessons (WG-R3): their full-world stream-power + droplet erosion (m 0.5, n 1.0, 16-24 cycles, ~0.3 droplets per
  land vertex, erodibility by class from mountain 1.0 to plateau 0.2) took 1,561 s on 90.7 M land vertices, mostly
  serial, and was **rejected visually**. It also touched finished hand-made terrain. Their CPU and CUDA erosion
  differed (p99 7 GU), which supports the fixed-point rule. Their parameters are our starting `Guess` tunables only.
- **Distant Landscapes of Tamriel** (nexusmods.com/morrowind/mods/58155): not read (Nexus blocks fetching; owner
  or a browser).

## Build plan

Framework slices and the order across both specs: framework spec "Build plan".

- [~] **WG-R2** measure the sources: terrain half done 2026-11-18 ("Source numbers"); left: the census of exterior
  references by class (with WG-A1).
- [x] **WG-R3** Procedural Tamriel mined (2026-11-18): `../../../docs/research/procedural-tamriel-2026-11-18.md`
  (§11 Take / Adapt / Avoid by slice); folded into this spec and the framework pipeline.
- [x] **WG-S1** height dump (2026-11-18): `bevyrrowind worldgen-dump --out <dir> [--bin-m 100]`, crate `mw_worldgen`
  (`assemble`, `region_map`, `dump`; `stats.rs`, `output.rs`), 5 tests. Writes `heights.f32` (raw LE f32, north-up),
  `heights.png` (16-bit), `mask.png`, `regions.png` (indexed, REGN `map_color`) + `regions.json`, `meta.json` (dims,
  spacing, origin, stats, blake3 hash). 1.4 s on the owner's install; output `target/worldgen/vvardenfell/` (ignored).
  Later: a `--cells` crop (the box includes Solstheim when Bloodmoon loads).
- [~] **WG-S2** look test, built 2026-11-18, **awaiting owner** (visual check WG-S2): first `sf_worldgen` crate
  (guide v1 reader, resample, priority-flood, implicit stream power, thermal, hillshade, `scrib-worldgen run`), recipe
  `bevyrrowind/worldgen/vvardenfell-s2.toml` (41 values, 33 `Guess`), 9 tests; 4 arms at 20 m in 113 s; numbers in
  `docs/ledger/worldgen.md`. The verdict picks the macro technique and the vertical targets.
- **WG-S3** in-game walk: a ~10 × 10 km crop around Red Mountain and Ghostgate through a raw-heightmap
  `TerrainSource`, no anchors. The owner walks it: scale, view distance, travel feel.
- **WG-A1** anchor extraction and census (rules above). Classify by mesh folder, then filename hints, then
  bounding box (≥ ~14 m terrain rocks are cliff giants), unknowns to `other`; then **verify against the source
  plugins**. In TR, path class alone misfires: its `x\` folder mixes structures and landscape rocks (Procedural
  Tamriel left whole regions rockless that way, then retracted its "complete" claim).
- **WG-A2** guides: intent heights, the REGN → recipe table, sea level, named rivers (Odai, Samsi, ...), foyada
  strokes.
- **WG-A3** placement: anchor references at their new positions in `mw_world` streaming (stable record ids).
  **WG-A3b** the coordinate list above through the anchor transform.
- **WG-A4** first Vvardenfell 10× world in the game (`--world vvardenfell-10x`); owner check WG-A4.
- **WG-A5** rivers and lakes from hydrology as water bodies (`water-adapter.md`).
- **WG-G2** Tamriel Rebuilt's ESMs through the same adapter: new regions, new recipes.

## Open questions (owner)

1. ~~Procedural Tamriel licence~~: answered 2026-11-18 (ideas yes, reimplemented in Rust).
2. Travel at 10×: how long a walk between towns should feel, and what carries the player (striders, boats, mounts,
   propylons, Intervention). After WG-S3.
3. Ambiguous anchor classes: ruled on from the WG-A1 census.
