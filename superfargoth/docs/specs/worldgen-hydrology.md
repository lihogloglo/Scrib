# Worldgen hydrology: the Magna Urbis water chain as stage 5 (WG-W1)

What: rivers and lakes of a generated world as a proper stage of the default chain, anchor-aware, audited, cached.
Repos and crates: `scrib-worldgen` (AGPL-3.0-or-later) `crates/water` (bin `worldgen-water`) and its vendored Magna
Urbis (MU) `vendor/magna-urbis/rust/crates/terrain`; Scrib (GPL) reads the products through files only:
`sf_worldgen::water` (reader), `mw_environment_bevy` (bodies, FL-5a), `bevyrrowind/tools` (chain), adapter data in
`bevyrrowind/worldgen/`. Research: `../research/worldgen-suite-2026-11-26.md` §5 row 2, §6, §8 WG-W1;
`scrib-worldgen/docs/research/magna-urbis-terrain-2026-11-24.md` §2, §4. Ledger `docs/ledger/worldgen.md` (WR-1, WR-2,
HC-0/1, S-2). Status: **specced 2026-11-26**. Parent spec: `worldgen.md` stage 5.

## Goal

Rivers are the 10x land's skeleton: roads route along and across them (WR-2), towns found on them (Balmora must
straddle its river, owner 2026-11-19), climate and flora read their wetness (WG-E1). Today the chain is a prototype arm
run by hand (WR-1). After WG-W1 it is a step of the default chain: anchor-aware (kept places never carved, flooded or
cut through), audited, cached by content hash, its products the contract of the water-bodies lane (FL-3, FL-4, FL-6).

## Today (verified 2026-11-26 in code; V = read in source, U = from a summary)

| What | Where |
|---|---|
| `worldgen-water --guide --recipe --out`: guide folder in, carved guide copy + `water.json` out (V) | `scrib-worldgen/crates/water/src/main.rs:6-19`, `lib.rs:1-12` |
| Guide padded with sea to a square, sea moved to 0, MU `condition_and_water` on it (V) | `run.rs:217-246` |
| The height change `delta` is added to **every** sample, stamps or not; only sea cells are kept when downsampled (V) | `run.rs:247-261` |
| Stamps are read as a conservative `w > 0` mask only (weights, not heights) (V) | `guide.rs:87-150` |
| Lakes touching a stamp are dropped; a river is cut where it enters a stamp or a dropped lake, the downstream part dropped (`clipped`) (V) | `run.rs:272-281, 726-763` |
| A dry closed basin crossed at its spill level becomes a lake; one touching a stamp ends the course at its rim (V) | `run.rs:584-679, 700-706, 833-849` |
| River levels fitted on the carved heights: non-increasing, at most `ground + fill·depth`, never under the sea (V) | `run.rs:544-579` |
| Provenance `water_from` = tool, guide path, MU rev, recipe blake3, input heights blake3; **no stamps hash** (V) | `run.rs:297-314` |
| Output byte-identical across runs; tests: island end-to-end (no river in a stamp, one reaches the sea, two runs equal), downsampled, basin (V) | `run.rs:75`; `tests/island.rs:258-341`, `tests/basin.rs:162` |
| Recipe lives in the tool repo: `conditioning.mode = "none"`, `downsample = 1` (WR-1) (V) | `scrib-worldgen/recipes/water-x10s.toml:10-13, 146-149` |
| MU used unmodified, pinned `21e2217` (V) | `lib.rs:12`, `run.rs:23` |
| MU chain: `delta = result.height - input`; conditioning `none` skips hollows and lake bowls; incision cuts beds and valleys after routing (V) | MU `vector_water/mod.rs:423-471` |
| MU keeps every height <= 0 (sea) unchanged and floors land at `min(h, 0.1)`; valley cuts never take land under the sea (V) | MU `conditioning.rs:797-802`, `incision.rs:236-240` |
| A MU course ends only at a confluence, a drawn lake, the sea or the map edge; `outlet` = the MU cell index of its end (V) | MU `mod.rs:227`, `rivers/network.rs:1843` |
| MU has no keep-out input (V, research grep) | `magna-urbis-terrain-2026-11-24.md:50` |
| Runtime stamp blend: `w >= 1` returns the stamp height exactly; else `h + (h_s - h)·w`, detail × `(1 - w)` (V) | `sf_worldgen/src/tiles.rs:415-432` |
| Stamp weight: 1 on the footprint, `1 - smootherstep(d / band)` across the band; band `clamp(0.5 R, 30, 150)` m (V) | `mw_worldgen/src/anchor.rs:83-85, 160-165, 600-601` |
| `worldgen-x10` writes the guide and its stamps; it never runs or expects water (V) | `mw_worldgen/src/x10.rs:1-8, 488-545` |
| Chain run by hand: x10 (HC-1 curve) -> volcano -> anchors -> derive (Balmora 2.0) -> water -> roads -> settle; no chain script (grep `worldgen-water`) (V) | adapter spec `worldgen-adapter.md:141-144`; ledger HC-1, S-2 |
| Reader keeps rivers `id, kind, points`, lakes `id, level_m, outline, holes`; drops `joins`, `outlet`, `clipped`, lake `kind` (V) | `sf_worldgen/src/water.rs:5-9, 22-44` |
| The game reads `water.json` and appends guide lakes and rivers to the water query (FL-5a) (V) | `mw_environment_bevy/src/water.rs:273`, `worldgen_water.rs:304` |
| Roads read `heights.f32` and `water.json`, never the stamps (grep `stamps` in `crates/roads/src`: none) (V) | `scrib-worldgen/crates/roads/src/raster.rs:72-118` |

## Canonical technique

Flow routing on a depression-handled DEM (priority flood, Barnes et al. 2014), channels where the drainage area
passes a threshold, width by hydraulic geometry (`w ~ Q^0.5`, Leopold & Maddock 1953), lakes by water budget, a
meandering planform (bank-migration models, Howard & Knutson 1984; Paris et al. 2023), then the bed and valley carved
into the relief. Authored keep-out zones enter the routing surface only, as **walls** (Arc Hydro DEM reconditioning
"Build Walls"; U, from memory), and authored rivers as **stream burning** (Arc Hydro AGREE; U). MU implements the
first part whole (`condition_and_water`, V); the walls are what we add.

## Reuse check

- **Bevy 0.20:** not applicable, offline tool. At runtime the products go through existing systems: `sf_water`
  bodies (FL-5a) and the terrain stamp blend (`tiles.rs`).
- **Crates:** MU's `terrain` crate (AGPL-3.0-only, rev 21e2217, vendored; `serde`, `rayon`) is the adopted tool (owner
  2026-11-24: research is how to use it, never whether). WhiteboxTools (Rust, MIT, maintained; U on version) has D8,
  breach and fill but no river network, meanders or lakes: not needed. No other Rust crate builds river networks
  (research §4; `water-bodies.md` reuse table).
- **Ours:** `crates/water` (wrapper: frame, lake tracing, level fit, basin lakes), `sf_worldgen::stamps` (weights),
  `sf_worldgen::water` (reader). `sf_worldgen::stream` routes but makes no rivers or lakes (research
  `magna-urbis-terrain` §4 row 5).

## Industry comparison

| Technique | Who ships it | Ours | Match |
|---|---|---|---|
| Priority-flood routing, area threshold | Houdini HeightField Flow, Gaea, World Machine (U) | MU flood + `riverArea` | match |
| Meanders, braids, oxbows as a planform stage | Paris 2023; Houdini river tools (U) | MU `rivers/meander.rs`, `sinuosity` 1.8, `terrainInfluence` | match: drops our planned meander port |
| Water-budget lakes, hydraulic-geometry widths | Turc-Pike; Leopold & Maddock | MU `balance`, `3·A^0.45` | match |
| Authored zones kept, rivers routed around | Wildlands, Far Cry 5: authored first, procedural between (U); Arc Hydro walls | MU routing barrier (W1a) + carve fade (W1b) | match after W1a/b; gap today (cut, not rerouted) |
| Authored rivers forced | Arc Hydro stream burning; UE5 river splines carve via Landmass | none | gap: Later |
| Sub-grid channel carve from the spline | UE5 Water river spline + Landmass brush | none (bed is a grid effect at 18 m) | gap: Later (ROADMAP [p] "generated rivers refactor") |
| Tiled hydrology | Gaea Tiled Builds (U) | one global pass | differ by design: upstream discharge cannot be tiled (MU doc, research `magna-urbis-terrain` §5.5); 4.2 s whole island |

## Design

### Chain order (files only between repos)

The default 10x chain, one product folder per step, each step a pure function of its input files, its recipe and the
seed. Stamps are made in step 3 and applied at runtime (`tiles.rs:415-432`): that runtime blend is "stage 4 restores
the anchors bit-exact" for every later step.

| # | Command (repo) | Product folder | Adds or changes |
|---|---|---|---|
| 1 | `bevyrrowind worldgen-x10 --height-curve bevyrrowind/worldgen/x10-heights.toml` (Scrib) | `x10h` | `heights.f32`, `meta.json`, regions, markers |
| 2 | `scrib-worldgen stamp-volcano --params bevyrrowind/worldgen/x10-volcano.toml` (Scrib `sf_worldgen` CLI) | `x10hv/s55` | `heights.f32` |
| 3 | `bevyrrowind worldgen-x10 --heights x10hv/s55` (Scrib) | `x10hs` | `stamps.json/.f32`, `anchors.json`, census |
| 4 | `bevyrrowind worldgen-layers` (Scrib) | `x10hs` | `layers.json/.u16` (source LTEX, reads no heights, `layers.rs:1-12`) |
| 5 | `bevyrrowind worldgen-derive` (Scrib; adapter's town choices, e.g. Balmora 2.0) | `x10hs-b2` | stamps dropped or appended |
| 6 | **`worldgen-water --recipe bevyrrowind/worldgen/water-x10.toml`** (scrib-worldgen) | `x10hs-b2w` | `heights.f32` carved, `water.json`, `flow_area.f32`, `meta.json` `water_from` |
| 7 | `worldgen-roads` (scrib-worldgen) | `x10hs-b2w` | `roads.json` |
| 8 | `settle` (scrib-worldgen) | `mods/<town>` | town |

Stage 6 (material fields, climate WG-E1) runs after step 6 on its products (`worldgen.md:164-168`: masks rebuilt from
the current field). Flags of steps 1-5 and 7-8 are the tools' own (U beyond the adapter spec); W1g writes them once.

**Cache.** A step's key is blake3 over its command line, tool version and the bytes of every input file it reads (for
water: `heights.f32`, `stamps.json`, `stamps.f32`, `meta.json`, the recipe, MU rev), written to `<out>/.chain-key`; a
matching key skips the step (`cached`), a changed one reruns it and every later step. `water_from` gains
`stamps_blake3` and `key` (W1f).

**Determinism.** MU writes its rows in parallel, each row its own slice (V, research `magna-urbis-terrain` §1); the
wrapper sorts every output. The same key gives byte-identical folders on any thread count (test W1f). The "tile alone
equals the whole run" rule does not apply: the stage is one global pass by necessity (table above).

### Anchor masking and rerouting

Let `h` be the guide's generated heights, `(h_s, w)` the stamp height and weight at a sample (bilinear, largest
weight wins, `sf_worldgen/src/stamps.rs:14-17`), and `G = h + (h_s - h)·w` the ground the runtime shows (`tiles.rs:431`
without detail; `G = h_s` where `w >= 1`).

1. **Input:** MU runs on `G` (sea at 0), so courses see the real ground around kept places, lift included.
2. **Walls (W1a, MU patch):** a generic `barrier` mask in `condition_and_water`: barrier cells never receive flow and
   no course is traced through them; their input heights are not raised (the routing surface is separate): MU's valley
   cut targets an absolute section (`incision.rs:226-240`), so raised walls in the input would turn into huge
   negative `delta` in the band. Arc Hydro's walls are likewise a routing DEM only. `None` gives byte-identical output. The wrapper sets the barrier on every
   sample with `w > 0` (the same conservative mask as today, `guide.rs:87-95`) **and** `G > sea`. Courses then spill
   around a kept place by MU's own flood, or end in the kept place's own water where it is at sea level (below).
3. **Output mask (W1b):** `heights.f32` gets `h' = h + delta·(1 - w)`; `anchor_w = w`, the stamp weight. Where
   `w = 1` (the footprint) `h'` equals `h` bitwise. The runtime then gives `R = mix(h', h_s, w) = G + (1 - w)²·delta`:
   kept places return `h_s` exactly (`tiles.rs:420-421`); a valley side that reaches into a band (MU valley reach up to
   600 m, `incision.rs:52`) fades to zero, C1, at the footprint, so no step at the anchor edge (the open risk of
   research `magna-urbis-terrain` §2).
4. **Water fit:** lakes and courses stay off `w > 0`, so `R = h'` wherever water is drawn and the existing fit
   (`run.rs:544-579`) is exact. The drop and clip rules stay as a guard; on the fixture they must not fire (W1b test),
   on the real guide their counts are reported per anchor.
5. **Impounded courses:** a course whose only way on is through a kept place pools behind the wall. The pool becomes
   a basin lake when it stays below the footprint's lowest rim ground and off `w > 0` (`run.rs:584-679`), else the
   course ends at its rim (`courses_ended_stamp`), named per anchor in the report. Owner question 1.

**Inland sea-level channels.** Source rivers that sit at sea level (the Odai through Balmora) are under the sea level
at 10x (HC-1: bed -5..-30 m). MU keeps every height <= 0 unchanged (`conditioning.rs:797-798`), counts it as sea and
ends courses there (`mod.rs:227`); the wrapper's basin flood and lake push-out stop at it (`run.rs:460, 653`); roads
never bridge it (WR-2). WG-W1 keeps that: the stage makes no body for them, never carves them, and a course reaching
one ends as a mouth (`ends_in: sea`). Their water is the ocean plane at sea level. This is the kept town's own water:
Balmora's Odai stays the town's river, rivers coming down to it end in it. Owner question 2 (what it looks like).

### Conditioning arm (decided)

WR-1 (2026-11-24) measured `both` (MU default): 3.2 M of 3.3 M land samples lowered, Red Mountain crater silted
+515 m; `breach`: 103 k samples raised > 5 m; `none` /3: coarse trenches; **`none` /1: 4.2 s, 170 lakes, 80 rivers
(127 km), 97 % of land bit-exact, 0 raised**. `none` /1 is the default (our stage 2 already eroded the relief; MU only
carves courses and fills lakes); `both` and `breach` stay recipe arms (`conditioning.mode`, recipe line 146). The crater
must not silt: a basin probe (W1c) asserts its depth stays within 5 m of the input. Owner look pending (TN-2).

### Wet/dry audit (W1c)

Hydrology is the only stage that creates water (`worldgen.md:164`), so its audit checks that all new water is its own
product, on `R`: (a) **new wet**: samples dry in the input `G` (above sea) that are under sea level in `R` and not
under a lake outline or a river ribbon: 0; (b) **new dry**: input sea samples above sea level in `R`: 0 (MU keeps them,
V); (c) **water over kept places**: lake or ribbon over a sample of `w = 1`: 0; (d) **probe basins** from the recipe
(`[[audit.basin]] name, centre_m, radius_m, max_depth_change_m`): depth = max minus min of `R` in the disc, compared
with the same on `G`. `--strict` fails the run on any breach; the chain always passes it. Counts go to `stats.audit`.

### Meanders (decided)

`worldgen.md` stage 5 planned a separate centreline stage (Paris 2023) before carving. MU's chain already meanders,
braids, cuts oxbows and carves along the meandered planform (`mod.rs:464-466`; `sinuosity`, `terrainInfluence`,
`braiding`, `avulsions` in the recipe). No separate stage, no later slice: the planned port is dropped (research
`magna-urbis-terrain` §4, row "5 Meander centreline"). Oxbow lakes stay off (`oxbows = 0`) until a look asks for them.

### Quay kind (out of this stage)

"Quay kind undiagnosed" (research §5 row 2) is the HC-1 ledger note "quays still one landing (deck now +0.3 m)": MU's
town planner gives Balmora 2.0's river front one quay kind. That is the settlement lane's harbour slice (MU C-8, gentle
bank rule `macroplan/harbour.rs:234`, research §6), not hydrology; this stage owes it the carved heights and river lines.

### Contract owed to the water-bodies lane (`water-bodies.md` FL-3, FL-4, FL-6)

`water.json` stays format 1 (new keys are additive; the reader ignores unknown keys, `water.rs:7-8`). W1d guarantees:

- Rivers: `points [x, y, width_m, level_m]` upstream to downstream, level non-increasing and >= sea; off lakes
  `level - R <= fill·bed_depth(width)` (`points_floating = 0`). `mu_id` unique per written course; `joins` names the
  `mu_id` of a written river or is absent (dangling ones removed when the receiver was dropped).
- **`ends_in`** per river (new): `sea`, `lake:<id>`, `river:<mu_id>`, `sink` (ended at a basin rim), `edge`, `stamp`;
  **`starts_in`**: `source` or `lake:<id>` (a lake outflow). FL-4a's mouth, inlet and tributary cases read these
  instead of re-testing polygons; FL-6a reads the end-to-receiver step from `level` and the receiver's level.
- Lakes: `level_m`, outline counter-clockwise, holes clockwise, pushed `lake_overlap_m` under the bank, `kind`
  open/closed, `from_course` for basin lakes.
- Bed profile: FL-1's analytic cross profile; the stage owes the centre level and width per point and the terrain `R`
  under them (FL-3 obstacles). `outlet` stays a pass-through MU cell index (`network.rs:1843`): no reader uses it.

## Adapter data (Bevyrrowind)

- `bevyrrowind/worldgen/water-x10.toml`: the recipe moved from `scrib-worldgen/recipes/water-x10s.toml` (game data
  leaves the tool repo; the tool keeps a generic fixture recipe), plus the probe: `[[audit.basin]] name = "red-mountain
  crater"`, `centre_m = [2974.5, 9852.2]` (crater floor, `x10-volcano.toml:15-17`, guide frame U), `radius_m = 700`
  (`x10-volcano.toml:42`), `max_depth_change_m = 5`.
- Climate stays two scalars (11 °C, 800 mm, Guess) until WG-E1 feeds per-region values (Later).
- No Morrowind logic: the Odai is sea because its source heights are at sea level, not because of a name.

## Slices

- **WG-W1a** MU routing barrier: generic `barrier: Option<&[u8]>` (cells that never receive flow and carry no
  course) through a new `condition_and_water_masked`; the old entry calls it with `None`. Files:
  `vendor/magna-urbis/rust/crates/terrain/src/vector_water/mod.rs` and its flood and trace call sites (U on which);
  `VENDOR.md` "Patches" entry "vector_water routing barrier (generic; None = byte-identical)".
- **WG-W1b** `crates/water`: `guide.rs` reads stamp heights and weights; `run.rs` builds `G`, the barrier, the mask.
- **WG-W1c** audit and probes (`stats.audit`, `--strict`, recipe `[[audit.basin]]`).
- **WG-W1d** `ends_in`, `starts_in`, dangling `joins` removed.
- **WG-W1e** `flow_area.f32` (MU accumulation, m², guide grid, sea 0) for WG-E1b moisture and scatter wetness.
- **WG-W1f** `water_from.stamps_blake3` and `key`, thread-count determinism test.
- **WG-W1g** `bevyrrowind/tools/worldgen_x10_chain.sh` with the `.chain-key` skip, reading the adapter recipes.

## Build rows

| Slice | Deliverable | Repo and crates (claims) | Depends | Test | Verify |
|---|---|---|---|---|---|
| WG-W1a | MU routing barrier | scrib-worldgen `vendor/magna-urbis` terrain `vector_water` + `VENDOR.md` | - | MU's own tests byte-identical with `None`; fixture: barrier across a valley, no course cell in it, flow reaches the sea | `cargo test -p magna-urbis-terrain` from `vendor/magna-urbis/rust` (nested workspace, `VENDOR.md:27-31`) |
| WG-W1b | anchor-aware run | scrib-worldgen `crates/water` (`guide.rs`, `run.rs`) | W1a | island fixture, stamp across the valley: `heights.f32` bitwise = input on `w = 1`; a course goes round and reaches the sea, `rivers_clipped = 0`, no point on `w > 0`; guide without stamps: output byte-identical to before | `cargo test -p water`; real guide summary line |
| WG-W1c | wet/dry audit + probes | `crates/water` (`run.rs`, `recipe.rs`) | W1b | fixture: new wet 0, new dry 0, water over `w = 1` 0; crater fixture: depth change <= 5 m (mode none: 0 raised); a planted breach fails `--strict` | real guide: `audit` line, Red Mountain probe printed |
| WG-W1d | end records + joins | `crates/water` (`run.rs`) | W1b | Y-confluence + lake + mouth fixture: `ends_in` sea, lake and river each found where built, every `joins` resolves, a tributary ends within one sample of its receiver | counts on x10hs-b2w printed |
| WG-W1e | `flow_area.f32` | `crates/water` (`run.rs`) | W1b | cone fixture: area at the outlet = land area within 1 cell; monotone downstream along a course | file size = guide samples × 4 |
| WG-W1f | provenance key | `crates/water` (`run.rs`, tests) | W1b | same key and byte-identical folder twice and on 1 vs 4 rayon threads; one changed byte in `stamps.f32` changes the key | `cargo test -p water` |
| WG-W1g | default chain | Scrib `bevyrrowind/tools/worldgen_x10_chain.sh`, `bevyrrowind/worldgen/water-x10.toml` | W1c, W1f | second run logs `cached` for every step and leaves every product's blake3 unchanged | owner check TN-2 round 4 |

Order: W1a, W1b, then W1c, W1d, W1e, W1f in any order, W1g last. Co-op: the products are offline files, the same on
every peer; nothing runs during play.

## Tunables

Recipe values use the tool's `Tunable {value, unit, source}` (`recipe.rs:13-20`). New ones:

| Name | Unit | Range | Default | Source | Slice |
|---|---|---|---|---|---|
| `audit.basin[].radius_m` | m | 10-5000 | 700 (crater) | `x10-volcano.toml:42` | W1c |
| `audit.basin[].max_depth_change_m` | m | 0-100 | 5 | WG-W1 brief (crater must not silt) | W1c |

Kept as they are (`water-x10s.toml`): `downsample` 1 and `conditioning.mode` none (measured, WR-1), `widthScale` 1.0
(hydraulic geometry), `river_fill` 0.6 (MU FILL), `lake_overlap_m` 6, `lake_simplify_m` 2, `max_basin_lake_km2` 5,
climate 11 °C / 800 mm, seed (Guess). The barrier threshold is the format's `w > 0` (no new value); the wrapper's
tolerances (`run.rs:384-387, 582`) are float noise, consts.

## Performance

Bake: today `none` /1 4.2 s (WR-1, measured); W1a +0.1 s, W1b +0.3 s, W1c +0.5 s, W1d-f +0.2 s (all E); the whole
chain W1g under 2 min (E: anchors 3.6 s WG-X2, roads 0.8-8.8 s WR-2/S-2, other steps U). Runtime: 0 ms for every
slice; the bodies' query cost is FL-5a's (0.844 µs per sample measured).

## Tests

Exact, fixtures built in the tests (table above): audit zero, anchor interiors bitwise, crater within 5 m, same hash
twice and on two thread counts, a river reaching the sea (existing `island_end_to_end`, kept), no river through or cut
by a footprint. `#[ignore = "measurement: x10 water stage"]` prints the real guide's summary, audit and ended courses
per anchor. Scrib side: none new (the reader is FL-4a's).

## Verification (owner checks)

TN-2 round 4 (rows file): the default chain's `x10hs-b2w`, after W1g, as 4 yes/no questions (rivers in their beds,
gorges read as valleys, crater still a bowl, no river stopping at a town edge). WA-13 (flow look) stays the
water-bodies lane's.

## Open questions (owner)

1. A river whose valley is closed by a kept place: let it pool into a lake beside the town (or end at the basin rim
   when the lake would flood the town), or move water through the town? **Recommended: the lake beside the town**; the
   town is never flooded or cut, and each case is logged per anchor.
2. The Odai and other sea-level channels inland: keep them as calm arms of the sea (ocean plane, sea waves reduced by
   the FL lane), or make them flowing rivers with a current? **Recommended: arms of the sea in WG-W1**; a flowing
   estuary is a later FL slice if the look asks for it.

## Later

- Named rivers forced as channels (stream burning of the guide's strokes; `worldgen.md` stage 2, adapter WG-A2).
- Sub-grid bed carve under the ribbon at runtime (ROADMAP [p] "generated rivers refactor"; research
  `magna-urbis-terrain` §5.3).
- Per-region climate into MU's water budget once WG-E1 exists (MU takes two scalars: a vendored patch for rasters).
- Oxbow lakes written; wetness raster beyond `flow_area.f32`; tiled water for worlds over 8192 cells a side.
