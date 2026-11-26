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
(one anchor per cluster, transition bands between them). **Settlements on geology** (owner 2026-11-18): Vivec sits in
a bay, Balmora on both banks of the Odai, Seyda Neen and Ebonheart on the shore: their anchors carry coastlines and
rivers that the generated land must continue (the bay as a coast constraint in the layout, the river as a forced
channel), not just a ground patch. A plain stamp is accepted for the first screenshot.

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

## Same map at 10× (WG-X, owner 2026-11-19)

Owner: players will choose classic Morrowind, **the same map at 10×**, and later the generated "realistic island" (the
WG-S/VO/TD chain, kept for later). The same map is the easy one and comes first: Morrowind's own heights ×10 in x, y
and z (`target/worldgen/x10`, 3329 × 3073 @ 18.288 m, summit 2.7 km), runtime detail from the G2 filter, then
**Morrowind's own ground textures, flora and settlements at their own places**. Settlements stay 1:1 and will feel
small (owner: expected; a settlement-growth system comes later). Red Mountain gets higher and more prominent with a
gentle apron (owner 2026-11-19, the 2026-11-18 summit ~5.5 km decision).

Pipeline (each step a deterministic product in the guide folder; positions never change after step 1):

1. **Scale** (`worldgen-x10`): the dump's heights ×S, spacing ×S, origin ×S (source frame metres, x east, y north).
2. **Volcano uplift** (`scrib-worldgen stamp-volcano`, additive mode): `h + apron(d_coast) + edifice(r)`, the base
   relief kept (Morrowind's crater, foyadas and ridges ride on top), calibrated to `summit_m`; zero on and near water,
   so no lake drains (wet/dry audit 0).
3. **Anchors** (`worldgen-x10 --heights <guide>`): references classified human-made / nature ("Anchors: what counts as
   human-made"); human-made ones clustered (single linkage, radius a tunable); each cluster an anchor that moves
   rigidly: world = S·c + (p − c) in x, y, and + L in height. **Lift L**: the median over the anchor's band ring of
   (final 10× height − source height) on dry samples, so the stamp meets the land around it (uniform ×S keeps slopes,
   so the mismatch is only curvature); L = 0 for anchors with a member standing on sea-connected water (docks,
   wrecks, Vivec), so they keep the sea. Census per anchor: members, L, ring mismatch p95, wet/dry changes.
4. **Stamps** (framework, generic: `sf_worldgen::stamps`, format below): each anchor's source heights at 1:1 + L
   over footprint + band, weight 1 inside the footprint and a C1 smootherstep to 0 across the band. Final height
   `mix(h_gen, h_stamp, w)`, runtime detail amplitude × (1 − w), on the GPU fill and the CPU twin alike.
5. **Ground textures** (WG-X3): the source layer grid (LTEX per 7.3 m texel) read at p/S outside anchors and at the
   anchor's source point inside them (w > 0.5), into the provider's `LayerMap`s; layer arrays from the exterior bake.
6. **Statics** (WG-X4): anchor members through the anchor transform, nature references dropped, into the 10× world's
   own static index / scene / far field. **Flora** (WG-X5): per region, the source's own flora models and their
   density per m² of land, scattered on the 10× land (slope, height, clumping noise, seed), counts raised in steps.
   Scatter v2 (WG-E2, `superfargoth/docs/specs/worldgen-scatter.md` "Adapter data"): species rows in
   `bevyrrowind/worldgen/scatter_species.toml`; learned densities become a region prior, not a replay target.

Known limits accepted for the first look: the Odai and other source rivers at sea level are arms of the sea at 10x
(HC-1); generated rivers route round kept towns (`../../../superfargoth/docs/specs/worldgen-hydrology.md`, WG-W1;
recipe `bevyrrowind/worldgen/water-x10.toml`); roads exist only inside footprints; door destinations, travel and scripts are not remapped yet
(WG-A3b); Solstheim is scaled too.

**Owner verdict at the end of 2026-11-19** (x10s, flora up to full density 7.2 M): still empty and 26 fps; no proper
forest (trees snug together in stands), rocks all one size, assets "sprayed like little bits on a donut"; relief still
smooth and blobby (**the next big thing**); some settlements misplaced (Balmora must straddle its river: anchors need
the geology constraints of "Anchors: what counts as human-made", not a plain stamp). Next session starts with research:
**how the Chlorophyll engine does vegetation at scale** (owner), forest structure (stands, clearings, size
distributions, undergrowth), and the sharp-relief plan. Base heights: `target/worldgen/x10v/s55` (regenerate:
`bevyrrowind worldgen-x10 --out target/worldgen/x10` then `scrib-worldgen stamp-volcano --base target/worldgen/x10
--params bevyrrowind/worldgen/x10-volcano.toml --out target/worldgen/x10v`, then `worldgen-x10 --heights
target/worldgen/x10v/s55 --out target/worldgen/x10s` and `worldgen-layers --guide target/worldgen/x10s`).

**Next (owner 2026-11-19, after the first look: "alright", textures super repeated, world extremely empty):**
- **X6 the 10× world as its own baked worldspace** (index, static scene, precombine, far field) instead of the load-time
  scaffold `WorldgenPlacementStatics` (one entity per placement, scene/precombine/far field off): fixes the 890-pipeline
  warm-up and the entity count, and lets flora counts rise (statics and flora far away need the far field).
- **X7 textures:** **X7a** anti-tiling (Mikkelsen 2022 Practical Real-Time Hex-Tiling, live A/B tunable `terrain.hex_tiling` in `sf_terrain_bevy`, built 2026-11-20); **X7b** slope-driven cliff rock + crevice darkening + VCLR tint as a guide product (the 10× macro is the layer mean only).
- Wet species (kelp, lilypads, menhirs on wet ground: 38.6k source references never placed), creatures in the wilds
  (6,399 levelled spawns left out of anchors), a flora/rocks share tunable (rocks take ~45 % of the budget).
- Sharp landforms (cliffs, scarps, block fields): `../../../docs/research/terrain-sharp-features-2026-11-19.md` (§7
  addendum 2026-11-20); owner 2026-11-20: aggressive dramatic cliffs and escarpments are welcome, keep generation fast
  for quick A/B iteration; also harvest dropped `terrain_rock_*` footprints as a cliff/outcrop steering mask.
- **Wilderness sites** (`../../../superfargoth/docs/specs/sites.md`): rows in `bevyrrowind/worldgen/sites.toml`,
  Morrowind templates and the remote block in `bevyrrowind/worldgen/mu-packs/sites.json`; kits from `worldgen-kit`
  on named anchors; classes from WG-E1b, else region ids.

**Stamp format v1** (guide folder, optional): `stamps.json` = `{ "format": 1, "stamps": [{ "name", "origin_m": [x, y]
(sample (0, 0), guide frame, row 0 = north), "dims": [w, h], "spacing_m", "offset": <f32 index into stamps.f32> }] }`;
`stamps.f32` = per stamp `w·h` heights (final metres, lift included) then `w·h` weights (0..1), little-endian,
row-major, north up. Stamps never overlap in weight > 0 (the adapter merges clusters whose bands touch); samples
are bilinear (Morrowind's terrain is bilinear-ish per quad; statics sit on it). `anchors.json` (adapter-side,
format 1): per anchor `name`, `stamp` index, `source_centre_m`, `world_centre_m`, `lift_m`, member reference keys.

**Reuse check / industry:** Bevy has no terrain stamping; no crate stamps authored patches into a heightfield with
a CPU/GPU twin. Industry standard: authored-content-first with procedural land between (Ghost Recon Wildlands, Far
Cry 5; framework spec "Industry comparison"); height stamps with falloff are UE5's Landmass / Landscape Patch brushes,
Houdini HeightField Copy/blend and World Machine layouts: match (stamp + smootherstep band, detail suppressed inside).

## Landforms (LF-A, 2026-11-22)

Framework: `../../../superfargoth/docs/specs/landforms.md` (operators, geology map, runtime detail). This section is
the Morrowind data: the base, rock units, region → operator table and sources. Base = `target/worldgen/x10s` (owner
2026-11-22: the reference; every arm is a delta on it). Data files: `bevyrrowind/worldgen/geology.toml` (LF-A1, built) and
`x10-landforms.toml` (with LF-2's recipe format); `x10-cliffs.toml` stays reproducible until LF-4 deletes its periodic passes.

**Rock units** (stand-ins for Vvardenfell's volcanic geology, S: game lore + Etna/Iceland analogues; hardness and bed
values `Guess` until an owner arm settles them; layer ids picked from the LTEX/PBR census at LF-A1, known names
`tx_ac_rock_01`, `tx_ashland_rock_03`, `tx_bc_rock_01`):

| Unit | Hardness | Beds | Dip | Outcrop | Regions |
|---|---|---|---|---|---|
| `basalt_flow` | 0.85 | 2-8 m flow units | `Surface` (radial from the summit) | yes | Red Mountain, Molag Amur, Ashlands highs |
| `tuff` | 0.55 | 1-4 m | `Surface` | yes | Ashlands, West Gash |
| `tephra` | 0.15 | none | n/a | no | Ashlands plains, Red Mountain apron, Molag Amur ash fields |
| `capped_sediment` | 0.75 cap / 0.3 beds | 3-12 m | `Fixed` 2-5° | yes | West Gash highlands, Grazelands plateau |
| `coastal_basalt` | 0.9 | 5-15 m (columnar) | `Fixed` ~0° | yes | Azura's Coast, Sheogorad |
| `alluvium` | 0.05 | none | n/a | no | Bitter Coast, Ascadian Isles lowlands |

**Region → operators** (weight and face budget per region; low budgets keep most land untouched):

| Region | Operators | Face budget |
|---|---|---|
| Red Mountain | `caldera` at the existing crater (several breaches, owner 2026-11-22: where foyadas and the existing paths meet the rim), `lava_lobe` from flank vents, `fan`; foyadas stay channels | 0.06 |
| Ashlands | low `fault_scarp` on unit boundaries, `lava_lobe`, `fan`; tephra plains calm | 0.04 |
| Molag Amur | `lava_lobe` fronts, fissure `fault_scarp`, `fan` | 0.05 |
| West Gash | `mesa`/`butte`, `canyon` on the larger rivers, `fault_scarp`, `terraces` (the rockiest region) | 0.08 |
| Grazelands | plateau-edge `fault_scarp` toward the coast, few `mesa`, `fan` | 0.04 |
| Azura's Coast, Sheogorad | `sea_cliff` on headlands, `butte` islets; bays stay beaches | 0.05 |
| Ascadian Isles | river `terraces` only | 0.01 |
| Bitter Coast | none (swamp), small `sea_cliff` on rocky headlands | 0.005 |

Calm-base weights (framework step 1, strength 0 until the owner A/Bs): 1 everywhere except Red Mountain (0: the
s55 apron is the accepted shape). Solstheim: out of scope (later recipe).

**Sources:**

- **Intent mask:** `terrain_rock_*` footprints → `cliff_mask.f32` (exists, `mw_worldgen::x10` `write_cliff_mask`):
  raises the score of every operator inside it, so landforms land where Bethesda faked rock. LF-A2 adds direction:
  clusters of those references (single linkage, as anchors) → principal axis + top height of the bounds → an
  oriented `fault_scarp` / `mesa` intent stroke per cluster. The stem's region code (`terrain_rock_ac_01`, ...) picks
  the unit where the RockMap disagrees; the full code list comes from the LF-A1 census.
- **Hero strokes:** the three `x10-cliffs.toml` strokes (`red_mountain_sw_scarp`, `west_gash_escarpment`,
  `molag_amur_rim`) become manual `fault_scarp`s with tapered tips and relay ramps.
- **Census names** (WG-L2a names list): the Red Mountain crater and named craters feed `caldera`; named valleys and
  foyadas are kept as channels (no landform crosses them).
- **Anchors:** stamp weights fold into the human mask; no landform enters an anchor band; the Ghostfence path ("Anchors")
  is kept clear of landforms too, so the ring stays a continuous walkable path.

Slices: **LF-A1** units + region table + layer ids (test: every REGN on the island has a row, every layer id
resolves). **LF-A2** oriented intent strokes from `terrain_rock_*` clusters (test: a fixture cluster yields a lip
along its principal axis, deterministic).

**As built LF-A1** (2026-11-22): `bevyrrowind/worldgen/geology.toml` = the framework `GeologyTable` (6 units as the
table above; `k`, soft-bed shares, repose and the share mixes `Guess`, sourced per value in comments) + one adapter
section `[layer_names.<unit>]` (LTEX ids, the framework ignores it). One row per legend region, Solstheim's 6 as
stand-ins (basalt under till). `capped_sediment` dips 3.5° toward 255° (Guess); `coastal_basalt` flat. Glue
`mw_worldgen::geology` (`AdapterGeology`: parse + checks, `resolve_layers` against a guide's `layers.json` palette,
`resolved_toml` for `scrib-worldgen geology --table`, `region_coverage`). On `x10s` every land sample lies in one of
the 15 rows (none outside a region); resolved table `target/worldgen/lf-a1/geology.resolved.toml`.

## Build plan

Framework slices and the order across both specs: framework spec "Build plan".

- [~] **WG-X** same map at 10× (section above), 2026-11-19: built X1 additive volcano (s55 accepted), X2 anchors +
  `worldgen-x10`, X2s stamps in the tiles (CPU + GPU), X3 ground textures, X4 statics (load-time scaffold), X5 flora;
  owner check WG-X seen (empty, 26 fps, sprayed flora, blobby relief). Next: research (Chlorophyll, forests, sharp
  relief), X6 own baked worldspace, X7 textures, settlement geology.

- [~] **WG-R2** measure the sources: terrain half done 2026-11-18 ("Source numbers"); left: the census of exterior
  references by class (with WG-A1).
- [x] **WG-R3** Procedural Tamriel mined (2026-11-18): `../../../docs/research/procedural-tamriel-2026-11-18.md`
  (§11 Take / Adapt / Avoid by slice); folded into this spec and the framework pipeline.
- [x] **WG-S1** height dump (2026-11-18): `bevyrrowind worldgen-dump --out <dir> [--bin-m 100]`, crate `mw_worldgen`
  (`assemble`, `region_map`, `dump`; `stats.rs`, `output.rs`), 5 tests. Writes `heights.f32` (raw LE f32, north-up),
  `heights.png` (16-bit), `mask.png`, `regions.png` (indexed, REGN `map_color`) + `regions.json`, `meta.json` (dims,
  spacing, origin, stats, blake3 hash). 1.4 s on the owner's install; output `target/worldgen/vvardenfell/` (ignored).
  Later: a `--cells` crop (the box includes Solstheim when Bloodmoon loads).
- [x] **WG-V2** markers (2026-11-18): the dump also writes `markers.json` (`mw_worldgen::markers`, format 1): one
  marker per 8-connected group of same-named exterior cells (case-insensitive; no settlement flag exists, so forts,
  ruins and shrines count; `cells` count lets a reader filter), centroid and cell bounds in **source metres** (units x
  1.8288/128, x east, y north, origin = Morrowind's), plus the summit (highest `LAND` sample). A 10x arm puts a
  marker at 10 x centroid; its cells keep their 117 m. Owner's install: 96 markers, 21 with 2+ cells (Vivec's
  cantons are separate names), summit (422.5, 1055.2) m. Read by `worldgen-view --markers`.
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
- **WG-LU3** rural land use data: `worldgen/landuse-x10.toml` (recipe: class -> MU biome, crop surplus per culture),
  `worldgen/mu-packs/landuse.json` (enclosures of Morrowind cultures), `worldgen/landuse.toml` (kind -> LTEX layer, crop
  and orchard species, boundary kits); spec `superfargoth/docs/specs/worldgen-landuse.md`.
- **WG-G2** Tamriel Rebuilt's ESMs through the same adapter: new regions, new recipes.

## Open questions (owner)

1. ~~Procedural Tamriel licence~~: answered 2026-11-18 (ideas yes, reimplemented in Rust).
2. Travel at 10×: how long a walk between towns should feel, and what carries the player (striders, boats, mounts,
   propylons, Intervention). After WG-S3.
3. Ambiguous anchor classes: ruled on from the WG-A1 census.
