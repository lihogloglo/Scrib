# Rural land use: fields, pastures, orchards, field ways (`WG-LU*`)

Fields, pastures, orchards, gardens, field ways and field boundaries round every farming place of a generated world,
made by the vendored Magna Urbis (MU) land-use module through its own data, read by Scrib as one product file.

- **Crates.** `scrib-worldgen` (AGPL, files only): `vendor/magna-urbis` (patches 06, 07), new `crates/landuse`
  (`worldgen-landuse`), reusing `crates/roads` (frame, sites, water readers) and `crates/settle` (wall-run filler,
  `placements.json`). Scrib (GPL): new `sf_worldgen::landuse` (Bevy-free reader + queries); game `mw_worldgen`
  (`layers.rs`, `flora.rs`, `town.rs`); data `bevyrrowind/worldgen/landuse.toml`, `landuse-x10.toml`,
  `mu-packs/landuse.json`.
- **Research:** the MU source at rev 21e2217 (`vendor/magna-urbis/VENDOR.md:8-9`); capability row 11 and plan C-9 of
  `scrib-worldgen/docs/research/magna-urbis-capabilities-2026-11-24.md:65`, `:182-183` (verified below, not copied).
- **Status:** specced 2026-11-26 (WG-LU0). Framework stage 10 of `worldgen.md` (pointer in the rows file).

Paths: `L/` = `vendor/magna-urbis/rust/crates/core/src/landuse/`, `S/` = `.../core/src/settlements/`, `U/` =
`.../core/src/urban/`. V = read in source, U = unverified.

## Goal

The 10x world has towns, hamlets and farms (`sites.md`) on empty ground (owner 2026-11-19: "world still empty").
Real farmland is the largest human mark on a rural landscape. Give every farming place its worked land: fields cut
in strips or closes, pastures, orchards and gardens, field ways linking them to the roads, and walls, hedges or
fences where the culture encloses. All of it from MU's land-use rules (owner 2026-11-26: "use as much Magna Urbis as we
can"), shown through the existing terrain layers, groundcover, scatter, roads and static paths. Same inputs and seed,
same bytes.

## Today (verified in code)

**What `landuse` is (V).**
- Shared by MU's native and WASM bridges (`L/mod.rs:1`). Host contract: `wire::execute(header, planes)`, JSON
  header + LE planes in, binary reply out, operation `rural` among others (`L/wire.rs:10-91`); MU's local server calls
  exactly this (`local/src/landuse.rs:13`); `wire::decode_reply` gives JSON back (`L/wire.rs:340-357`).
- `Input` (`L/mod.rs:83-134`): `seed`, `w`, `h`, `cell` (m), `mapSize`, `biome`, `seaLevel`, `options`, `geometry`,
  `plan`; planes `height`, `slope`, `water` (u8), `cost`, `hab`, `dWater`, `dSea`, `naturalForest`, `soil`, or
  `travelFrom` + `pass` (0 blocked, 1 land, 2 brook, 3 bridged) and MU computes the cost (`L/wire.rs:66-72`). `rural`
  needs a **square** raster and `w*h <= 16 777 216` (`L/mod.rs:136-164`). Frame: cell centres, origin upper left, y
  down (`L/rural.rs:251-252`; as `crates/roads/src/raster.rs:4-6`).
- Output (`L/mod.rs:167-229`): `areas[]` (`kind`, `poly`, `holes`, `stripAngle`, `strips[]`, `fieldLayout`,
  `cultivation`, `enclosed`, `enclosure`), `farmsteads[]`, `reserve`, `ways`, `headlands`, `stats`. Kinds: field,
  meadow, pasture, forest, orchard, garden, marsh, commons (`L/mod.rs:249-251`).
- `generate_rural` (`L/rural.rs:132`) works **one place's map**: the main settlement (`plan.footprint`,
  `plan.population`, `L/rural.rs:212-229`, `:950-956`) plus secondary settlements of `geometry.settlements`
  (`:182-185`, footprint from `plan.settlementFootprints[key]`, a 20 m disc for `detail: "farmstead"`,
  `:293-301`). Drawn water is the one water authority; no class on a water cell, every region cut by it
  (`:170-180`, `:1235`). Roads raster an exclusion band `width/2 + 0.75 cell + 3 m` (`:152-168`).
- Rules (`L/rural.rs:926-936`, V): cultivated land is worked within `working_reach` of a place (0.8 km farmstead ...
  2.5 km town, `:1714-1724`) or 800 m of a farm (`FARM_REACH`, `:938`); its area is `people x 0.7 ha x (1 +
  cropSurplus)` (`:939-959`, `cropSurplus` default 0.2, clamped 0-3) and at most one `holding_ha` (12 ha, 6 cold, 4
  Underdark, `:1727-1735`) per farming household (`rural_households`, `:1749`). Surplus becomes pasture. Fields are
  partitioned into furlongs and closes (`fields::partition`, `:1426-1445`), furlongs cut into strips, closes enclosed
  with `plan.fieldMaterial` unless hedge (`:1571-1611`); `fieldEnclosure` 0..1 sets the bocage share (`:653-664`).
- Field ways: `fields::connect` links them to roads and farm drives, `fields::prune` drops the unlinked
  (`L/rural.rs:1449-1509`); in plan mode woods and open land are cleared along them (`:1612-1652`).
- Farms inside land use (`L/rural_farms.rs`, V): (a) a farmstead laid out at each `detail: "farmstead"` secondary
  (`:483-570`), (b) farms along the roads by town size (`:583-602`), both off when `plan.farmsteads == false`
  (`:481`); (c) **demand farms** for suited land no farmer reaches (`add_for_demand`, `:404-461`, called at
  `L/rural.rs:1062-1077`), **not gated by any data key**. Their drives are routed by `farm_access.rs:1-10`.
- Farm types are a weighted draw from `catalogue.json` `farmstead.cultures.<c>.types` (`L/farms.rs:118-125`,
  `:190-215`); a data-only type is laid out by `steading` (`:1488`), so new types are a pack.
- Determinism: one seed, named forks (`L/rural.rs:142-144`); `par::map` is an order-preserving rayon map
  (`L/par.rs:1-12`): no thread-count dependence. Cost: `stats` `ms.lu.*` (`L/rural.rs:434-435`, `:1362`, `:1653`),
  laps under `URBAN_PROFILE` (`L/timing.rs`). No production timing is recorded; the one fixture is 20 x 20 cells of
  32 m (`core/tests/fixtures/landuse-ts.json`). `horticulture-ts.json` is test-only (`L/tests.rs:325`).

**MU's other farm paths (V).**
- `Terrain::place` puts dispersed farms (`S/countryside.rs:107-183`: ~0.45/km2 x density, arable potential >= 0.2,
  within 1 km + 3 radii of the parent, 300 m apart) as `ActivitySite`s with buildings from `U/activities.rs`
  `farm_yard` (`:170`), the town farmsteads' generator (`U/farmsteads.rs:1-7`). `sites.md` takes these as `source =
  farm` (`sites.md:121`).
- MU roads lay **field tracks** from each place into its farmed belt (`S/roads/mod.rs:536-575`), sized by the
  land-use rules (`farmed_reach`, `:1068-1076`, calls `landuse::rural_households`, `holding_ha`, `working_reach`).
  `roads.json` already carries them (ledger WR-2: 232 km of track).
- Enclosure per culture: `enclosures.json` `field`, `fieldMaterial` (`hedge | wall | fence`), groups, `village` level
  (`U/enclosures.json:3-4`); `enclosure::resolve` (`U/enclosure.rs:50-67`) is in a private module
  (`U/mod.rs:24`); fields are open unless `countryside_enclosures` (`U/enclosure.rs:60-61`).

**Neighbours (V).** `site::choose_site` returns `dWater`, `dSea`, `hab` (height above water level), `cost`
(`core/src/site.rs:692-703`, `:806`, `:1547`) at a fixed position (`:1383`; exact seat U). `Terrain::resources()`
gives `arable` (`S/mod.rs:454`, `resources/potentials.rs:63`). Patches 04, 05 are `sites.md`'s (`:136`, `:177`).
SW calls no land use today (grep: comments only). Scrib: `sf_worldgen::layers` (`layers.rs:1-44`) is written by
`mw_worldgen/src/layers.rs`; scatter's `excluded` = stamp weight > 0 (`flora.rs:335-337`); groundcover maps layer
weights to species (`groundcover.md:87`).

## Magna Urbis modules and what we use

SW = scrib-worldgen. "Used" = called by a SW crate today (grep of `crates/*/src` and `Cargo.toml:16-20`).

| Module | What it does (its own head doc) | Used today (by) | Planned by | V/U |
|---|---|---|---|---|
| crate `core` | generation core: resources, settlements, urban; re-exports terrain (`lib.rs:1-3`) | yes (settle, roads) | all | V |
| crate `geom` | planar geometry and polygon ops in metres (`geom/src/lib.rs:1`) | yes (settle) | LU1d clip | V |
| crate `graph` | planar half-edge subdivisions (`graph/src/lib.rs:1`) | no (inside core only) | none | V |
| crate `houses` | architectural recipes, local building generation (`houses/src/lib.rs:1`) | no | buildings later (`VENDOR.md:46`) | V |
| crate `local` | loopback service for the browser builds (`local/src/main.rs:1`) | no; never (we call core) | none | V |
| crate `random` | the TS sfc32 stream and forks (`random/src/lib.rs:1`) | via core | via core | V |
| crate `terrain` | V2 terrain and water chain (`terrain/src/lib.rs:1`) | yes (water: `vector_water`) | hydrology | V |
| `core/landuse` `generate_rural` via `wire::execute` | rural classes, fields, ways (`L/mod.rs:1`) | no | this spec | V |
| `landuse` `generate_natural_cover`, ground ops, `tools`, `farm_geometry` | wild cover, renderer ground masks, diagnostics, one farm plan | no | none (scatter owns natural cover; sites own farms) | V |
| `core/settlements` `place` (placement, countryside, activities, remote, nodes) | regional points, farms, sites (`S/mod.rs:1-4`) | no | `sites.md` WG-P1b | V |
| `settlements/roads` | regional network, field tracks, activity access | yes (roads: `Terrain::roads`, `run.rs:234-235`) | `sites.md` P1c | V |
| `core/urban` `generate_quarters`, `generate_parcels` | the town engine (`U/mod.rs:1-2`) | yes (settle `run.rs:174-178`) | settlements | V |
| `urban/` access, network, quarters, macroplan, footprint, parcels, programme, forms, fortification, harbours, funerary, growth | town pipeline stages (each head doc) | inside settle's two calls (not traced) | settlements | U |
| `urban/enclosure` | lot and field enclosure per culture (`U/enclosure.rs:1-4`) | via `partition.enclosure` (`U/pipeline/parcels.rs:153`) | LU1b (re-export) | V |
| `urban/farmsteads`, `activities` | farm yards and activity data (`U/farmsteads.rs:1-7`) | no | `sites.md` (farms) | V |
| `urban/edits` | City Bench inspector edits (`U/edits/mod.rs:1-2`) | no | none (UI) | V |
| `urban/neighbours`, `culture` | claims; culture mappings | yes (settle `claim.rs`, packs) | settlements | V |
| `core/resources` | resource fields on a block grid (`resources/mod.rs:1-4`) | no | sites (via `place`), LU1c soil | V |
| `core/render_geometry` | render-side map geometry (`render_geometry/mod.rs:1-2`) | no | none (Scrib renders) | V |
| `core/site.rs` | main-town site choice, planes (`site.rs:1-8`) | no | LU1c planes | V |
| `core/data.rs` | data documents + `install_packs` (`VENDOR.md:82`) | yes (settle `culture_packs.rs`) | LU3 pack | V |
| `core/threads.rs` | process thread policy (`threads.rs:1-5`) | no | none | V |

## Canonical technique

**Land classes by rule, then parcel cutting.** Land is classed from travel cost to the farmers, soil, slope and
water; holdings are sized by households; land is cut into furlongs along contour or road, then strips (open field)
or closes (enclosure, bocage); parcel cutting as Vanegas et al. 2012 (Eurographics, U). MU does this chain: access rings, budget,
furlongs, strips, closes, headlands, ways (`L/rural.rs:926-1611`).

## Reuse check

- **Bevy 0.20:** not applicable, offline tool. At runtime land use is ground layers, groundcover, scatter instances,
  road ribbons and static placements through existing systems: 0 new runtime systems.
- **Crates:** MU is the owner's choice. Scrib's reader needs only `serde`/`serde_json` (already in `sf_worldgen`).
  No new dependency on either side.
- **Ours:** `crates/roads` (`raster.rs:21-52` frame, sites and water readers), `crates/settle/src/filler.rs` (kit
  pieces end to end along a line, gates), `placements.json` + `worldgen-town`, `sf_worldgen::roads` ribbons, the
  groundcover rule table.

## Industry comparison

| Technique | Who ships it | Ours | Match or gap |
|---|---|---|---|
| Rule-based land classes + parcel subdivision | Vanegas 2012 (U); CityEngine lot subdivision (U) | MU `generate_rural` | match |
| Open-field strips, closes, bocage by culture | historical field systems; MU `fieldEnclosure`, `fieldMaterial` | MU data per culture | match |
| Procedural farmland round villages in a Houdini world pipeline | Ghost Recon Wildlands (GDC 2017, U), Far Cry 5 (GDC 2018: biomes, water, cliffs, vegetation procedural, map largely hand-made; fields U) | offline tool, one product | match |
| Hand-authored fields | KCD, Witcher 3 (U) | kept anchors stay authored | match |
| Crops as layer-driven grass | UE5 Landscape Grass Type, Skyrim LTEX/GRAS (`groundcover.md:61-65`) | field layer -> crops | match |
| Walls and hedges as kit along edges | UE spline meshes, Houdini copy-to-curve (U) | settle filler on rings | match |
| Woods compete with clearance | MU `naturalForest` claims dense wood (`L/rural.rs:819-860`) | not fed: fields are clearings, scatter owns woods | gap by choice (Later) |
| Field size and shape by culture | MU code (`fields.rs`), not data | MU's sizes | gap: patch only on owner look |

## Design

**Owners.** Farm points, yards, buildings: `sites.md` (MU `countryside.rs`). Field tracks from places into their
belt: MU roads (`S/roads/mod.rs:536`). Tracks to farms: MU roads `activity_access` (`sites.md` P1c). Everything else
on rural land (classes, fields, strips, closes, pastures, orchards, gardens, ways, headlands, enclosure): MU land use,
this spec. Land use places **no farm**: `plan.farmsteads: false` turns off (a) and (b) (`L/rural_farms.rs:481`);
**patch 06** adds `plan.demandFarms` (default `true` = upstream) for (c), whose count `free_households -
farmsteads` (`L/rural.rs:1015-1016`) no data reaches. Unreached suited land then goes to pasture, as MU does for
unbuilt demand farms (`:1078-1085`). A test asserts `farmsteads` is empty in every reply.

**Chain position.** After the final ground (stamps, `water.json`), `sites.valid.json` (WG-V1), pads (WG-P2) and
`roads.json` (WG-P1c); before the layer writer, scatter and groundcover. It creates no water and writes no heights
(stage rules `worldgen.md:175-177`; audit: heights hash unchanged).

**Parishes (the call unit).** MU farms one place's map per call (`L/rural.rs:132`; capability plan C-9 "per town").
A parish = one farming place P (kept town, generated hamlet or village) with the farms whose `parent` is P.
1. **Parish map** on a `landuse.cell_m` grid over the guide: a labelled distance transform from every place footprint
   and farm footprint, labelled by parish (ties: lowest key). Each cell has one owner.
2. **Window** per parish: the square holding P's footprint, its farms' footprints grown by 800 m (MU's
   `FARM_REACH`) and P's `working_reach`, plus `landuse.window_margin_m`. Square because `rural` demands it.
3. **Request** (`wire::execute`, operation `rural`, as MU's local server): planes `height`, `slope` from the final
   ground; `water` (1 sea, 2 lake, 3 river: `resources::Surface` codes); `pass` (bridges and fords = 3) with
   `travelFrom` = P's centre; `hab`, `dWater`, `dSea` from `site::choose_site` at P's fixed position; `soil` from the
   resource model's `arable` on its block lattice. No `naturalForest` (fields are clearings; scatter owns woods).
   `geometry`: `roads.json` segments in the window (class as `kind`), `water.json` rivers and lakes, settlements = P
   (`main`), its farms (`detail: "farmstead"`, `farm_population`), and every other place, farm and **anchor
   footprint** in the window with population 0 (blocked land, no budget). `plan`: `footprint`, `population`,
   `settlementFootprints`, `farmsteads: false`, `demandFarms: false`, `fieldEnclosure` + `fieldMaterial` from MU's own
   `enclosure::resolve` (**patch 07** re-exports it, no behaviour change), `cropSurplus` per culture. `options`:
   `mapSize` = window side, `culture`, `size`, `relief`, `climate`; `biome` from the class at P. MU's web host glue
   for `hab`, `soil`, `plan` (upstream `web/src/gen/landuse`, not vendored, U) is ported from upstream rev 21e2217
   in LU1c, AGPL into AGPL; where it differs from the above, upstream wins.
4. **Clip** each reply's areas, strips, ways and headlands to P's parish cells; drop pieces under
   `landuse.min_piece_m2`. Parishes run in parallel, collected in key order: same bytes on any thread count, and a
   parish run alone equals the same parish in a whole run (its inputs are files, never another parish's output).

**Seeds.** `<world seed>:landuse:<parish key>`; MU forks inside (`L/rural.rs:142-144`).

**Kept towns and anchors.** Anchor footprints (with their band) enter every request as blocked footprints, and the
clip removes any piece where stamp weight > 0: land use never touches an anchor interior (stage rule). Authored
fields and plantations inside anchors are anchors: they stay as authored. Generated fields start beyond the band.

**Product `landuse.json` format 1** (guide frame, x east, y north, metres; JSON like `roads.json`): header `format`,
`seed`, `mu_rev`, blake3 of guide heights, `water.json`, `sites.valid.json`, `roads.json`, recipe, packs; per parish
`key`, `culture`, `biome`, `stats` (MU's, `ms.*` zeroed for the hash); `areas[]` = `id` (`<parish>:<n>`), `kind`,
`ring`, `holes`, `strip_angle_deg`, `strips[]` (rings), `layout`, `cultivation`, `enclosed`, `enclosure`; `ways[]` and
`headlands[]` (polylines). Coordinates rounded to 1 cm, fixed key order. Readers check `format` first and reject
unknown fields (settle convention). Co-op: an offline product, the same file on every peer.

**Boundaries.** For every enclosed area, `worldgen-landuse` lays the culture's wall, hedge or fence kit along its
ring with `settle`'s filler (whole pieces, a gate piece where a way, headland or road meets the ring), skips edges
shared with a road ribbon or an anchor band, and writes `landuse/placements.json` (format 1, `instances` only, one
group key per parish). `worldgen-town` turns it into a native mod, one `sf.placement_group` per parish.

**Scrib consumers (generic reader `sf_worldgen::landuse`, names no game).**
- `LandUse::read(dir)`; `kind_at(x, y)` (bucket grid, pure point query); `ways()` as `roads::Segment`s of class
  `track`, merged at read (`roads.json` stays the router's product).
- Layers: the adapter's layer writer paints each kind into the global layer grid by its table, headlands as pasture;
  groundcover then grows crops on the field layers by its rule table.
- Scatter: `excluded` = stamps OR clearings OR a kind marked `clear` (field, garden, orchard ground); pasture and
  commons scale tree intensity by `tree_scale`. Orchards: `rows(area, spacing, row_gap, jitter)` along `strip_angle`,
  a scatter species rank-cut by the scatter budget. Field ways: the existing road ribbon path. Map: Later.

## Adapter data (Bevyrrowind)

- `bevyrrowind/worldgen/landuse-x10.toml` (recipe for `worldgen-landuse`): `cell_m`, margins, `farm_population`, per
  MU culture `cropSurplus` (`Guess`), climate class -> MU `biome` (Ascadian Isles, West Gash: `temperate`; Bitter
  Coast: `tropical`; Grazelands: `steppe`; Ashlands, Molag Amur, Red Mountain: `desert`; Sheogorad, Azura's Coast:
  `temperate`; all `Guess`), region names until WG-E1b classes exist.
- `bevyrrowind/worldgen/mu-packs/landuse.json` (MU pack): `enclosures.cultures.hlaalu = {field 0.5, fieldMaterial
  "fence"}`, `imperial` `wall`, `redoran` `wall`; farmstead types reuse `hlaalu.toml:118-119`. Ashlander places farm
  nothing (`cropSurplus` 0, population counted as herders: pasture only). All `Guess`.
- `bevyrrowind/worldgen/landuse.toml` (Scrib adapter): kind (+ cultivation, class) -> layer record id (resolved
  against the guide palette like `geology.rs:85`; unknown = error); groundcover crop rows (saltrice on wet fields,
  wickwheat, ash yam on the Ashlands fringe, marshmerrow on the Bitter Coast: names checked against the pack's records
  at bake); orchard species; kit style per boundary material (`worldgen-kit` on plantation fences, Imperial walls).

## Slices

One deliverable plus its test each. Order: LU1a, LU1b -> LU1c -> LU1d -> LU1e -> LU2a -> (LU2b, LU2c, LU2d, LU2e beside
each other) -> LU2f -> LU3 -> owner check WG-LU-look. Depends on `sites.md` WG-P1c (roads with farms), WG-V1, WG-P2.

## Build rows

| Slice | Deliverable | Repo and crates (claims) | Depends | Test | Verify |
|---|---|---|---|---|---|
| WG-LU1a | MU patch 06: `plan.demandFarms` (default true) | SW `vendor/magna-urbis/rust/crates/core` (`landuse/rural.rs`), `VENDOR.md` | V-1 | false: no farm added, unreached suited land is pasture; absent = byte-identical fixture reply; core tests pass | settle gate unchanged |
| WG-LU1b | MU patch 07: `pub use enclosure::{resolve, Resolved}` | SW vendor (`urban/mod.rs`), `VENDOR.md` | V-1 | `resolve("mediterranean-modern", ..)` equals `enclosures.json` values; no output changes | core tests pass |
| WG-LU1c | parish map + request builder (port of upstream host glue) | SW new `crates/landuse` (`parish.rs`, `request.rs`) | LU1a, LU1b, P1c, V1 | parish cells exact on a 3-place fixture; frame round trip exact; request passes `Input::validate(true)`; anchor footprint blocked | `cargo test -p worldgen-landuse` |
| WG-LU1d | `worldgen-landuse` run, clip, `landuse.json` v1 | SW `crates/landuse` (`run.rs`, `product.rs`, `main.rs`) | LU1c | same bytes twice and at 1 vs N threads; parish alone == whole run; no area on water, anchor band or road band; `farmsteads` empty; every field within reach (MU invariant); every way end on a road, way or drive | census on `x10s` |
| WG-LU1e | boundary runs -> `landuse/placements.json` | SW `crates/landuse` (`bounds.rs`), `crates/settle` (`filler.rs` made line-generic) | LU1d, kits | every enclosed ring gets pieces or a counted skip; gates at every way crossing; settle gate byte-identical | instance census |
| WG-LU2a | `sf_worldgen::landuse` reader + `kind_at` + `ways()` | Scrib `sf_worldgen` (`landuse.rs`) | spec | fixture round trip; wrong format and unknown field rejected; `kind_at` exact on a hole; tile alone == whole | `test -p sf_worldgen` |
| WG-LU2b | land-use kinds in the layer writer | `mw_worldgen` (`layers.rs`), `landuse.toml` | LU2a | every field texel = its table layer; stamp grids untouched; unknown layer id is an error | layer census |
| WG-LU2c | scatter exclusion and tree scale by kind | `sf_worldgen` (`scatter.rs` hook), `mw_worldgen` (`flora.rs`) | LU2a, E2g | no tree or rock in a `clear` kind; tile alone == whole | flora census delta |
| WG-LU2d | orchard rows as a scatter species | `sf_worldgen` (`landuse.rs` `rows`), `mw_worldgen` (`flora.rs`) | LU2a | rows parallel to `strip_angle`, spacing exact on a rectangle; all inside the area | instance count |
| WG-LU2e | field ways drawn as track ribbons | `mw_worldgen` (road drawing input) | LU2a | segment count = ways; ribbons draped (no NaN); none on water | log line |
| WG-LU2f | boundary mod: one placement group per parish | `mw_worldgen` (`town.rs`), app `main.rs` | LU1e | groups == parishes with walls; instance count exact; same bytes twice | owner check |
| WG-LU3 | adapter data: recipe, MU pack, `landuse.toml`, groundcover crop rows | `bevyrrowind/worldgen/*` | LU1d, LU2b | every table entry resolves; census per culture printed | owner check WG-LU-look |

## Tunables

| Name | Unit | Range | Default | Source |
|---|---|---|---|---|
| `landuse.cell_m` | m | 4-32 | 10 | Guess (MU fixture 32 m; strips are vector) |
| `landuse.window_margin_m` | m | 0-3000 | 600 | Guess |
| `landuse.farm_population` | people | 1-15 | 6 | MU `HOUSEHOLD` 5.5 (`L/rural.rs:1711`), rounded |
| `landuse.min_piece_m2` | m2 | 0-2000 | 200 | Guess |
| `landuse.crop_surplus.<culture>` | ratio | 0-3 | 0.2 | MU default (`L/rural.rs:940`) |
| `landuse.biome.<class>` | enum | MU biomes | table above | Guess |
| `landuse.soil_scale`, `soil_offset` | 1 | -2..2 | 2, -1 (arable 0..1 -> -1..1) | Guess until the LU1c port |
| `landuse.<kind>.tree_scale` | ratio | 0-1 | field 0, pasture 0.15, commons 0.3 | Guess |
| `landuse.orchard.spacing_m`, `row_gap_m`, `jitter_m` | m | 2-15, 3-15, 0-2 | 6, 7, 0.5 | Guess |
| `landuse.way_width_m` | m | 1.5-5 | 3 | MU clears 3 m of open land (`L/rural.rs:1622`; half or full width U) |
| `landuse.boundary.gate_gap_m` | m | 2-8 | 4 | Guess |

MU's own constants (`L/rural.rs:938-939`, `:1711-1735`) stay upstream code; we steer through `plan` data. Changing
them is a patch, only after an owner look asks for it.

## Performance

| Slice | Bake (s) | Runtime ms |
|---|---|---|
| LU1d, ~130 parishes, windows ~6-8 km at 10 m (0.4-0.6 M cells), 16 threads | 30-90 E (no MU production timing recorded) | 0 |
| LU1e boundaries | under 10 E | 0 |
| LU2b-d (in the layer and scatter bakes) | under 5 E | 0 new; orchard trees and boundary pieces count against the scatter and static instance budgets |

The census of LU1d prints `ms.lu.*` per parish and the total: bake time is a measurement row for the worldgen lane.

## Tests

Exact assertions are listed per slice in Build rows, plus: heights hash unchanged (the wet/dry audit). Measurement
(`#[ignore = "measurement: ..."]`, prints): ha per kind, parish and culture; MU `field.unreachedHa`, `surplusHa`,
`orphanHa`; wall metres; bake seconds.

## Verification (owner checks)

WG-LU-look (entry text in the rows file): one generated hamlet with farms on the 10x island, and the surroundings of
one kept town. Questions are yes/no on fields reading as worked land, strips and closes, walls with gates, crops, and
woods kept out of fields. No automated look check.

## Open questions (owner)

1. **Are field crops harvestable?** Recommended: not in v1 (look only, groundcover); later a share of crop plants
   near farms becomes harvestable like vanilla flora, with ownership (taking them is theft).
2. **May generated fields reach right up to kept towns and plantations?** Recommended: yes, beyond the anchor band,
   so kept places sit in farmland rather than in a ring of wild land.
3. **Do field walls and fences block the player?** Recommended: yes, they collide, but stay low enough to jump
   (kit choice), with a gate at every way.

## Later

- MU's own land-use farms (`plan.farmsteads`, `plan.demandFarms` on: farms along roads and on good land no farmer
  reaches, with drives from `farm_access.rs`): more of MU, but each such farm must then pass site validation and get a
  pad (`sites.md`), so it needs a second pass. Decided after WG-LU-look; both switches are data.
- Feed scatter's canopy suitability as `naturalForest`, if the owner wants dense woods to win over fields.
- Field size and shape per culture (today MU code in `fields.rs`): a generic data patch only after an owner look.
- Map colouring of land use (map.md), seasonal crop looks, Underdark fungal beds (MU `cultivation: fungal`).
