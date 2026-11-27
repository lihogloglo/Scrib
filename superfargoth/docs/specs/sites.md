# Wilderness sites: placement, reachability, clearings (`WG-P*`, `WG-V1*`, `WG-NM*`)

Generated places between the kept anchors: ruins, camps, shrines, tombs (fronts), mines (surface works), standing
stones, farms, hamlets, waystations. Placement is Magna Urbis (MU) driven by a site table (data); Scrib reads the
result, validates reachability on the final ground, clears flora around sites and turns each site into static
content like an anchor.

- **Crates.** `scrib-worldgen` (AGPL, files only): `vendor/magna-urbis` (patch 04), new `crates/sites`
  (`worldgen-sites`), `crates/roads`, `crates/settle`. Scrib (GPL): `sf_worldgen::sites`, `sf_worldgen::reach`
  (Bevy-free); game `mw_worldgen` (`town.rs`, flora exclusion), data `bevyrrowind/worldgen/sites.toml`.
- **Research:** `../research/worldgen-suite-2026-11-26.md` §4-§6, §8-§9; MU capabilities
  `scrib-worldgen/docs/research/magna-urbis-capabilities-2026-11-24.md` rows 11-14, C-10, C-11, C-14.
- **Status:** specced 2026-11-26 (WG-P0). Framework stage 9 of `worldgen.md` (pointer). Caves and dungeon entrances:
  `rockgen.md` RG-7 (cave mouths), later.

## Goal

The 10x world has 631 authored anchors over ~2500 km2 and nothing between 31 towns (owner look 2026-11-19: "world
still empty"; owner 2026-11-25: gameplay and interactivity next). Fill the land between anchors with places that
belong to their ground (a tomb in a gorge, a camp on the ash plain, a shrine at a crossroads), that the player can
walk to, that stand in a clearing, and that ride the existing content paths. Same seed and inputs, same sites, same
bytes.

**Decisions (do not reopen):**
- Exterior-only at first: ruins, camps, shrines, surface mine works, standing stones, tomb fronts with a sealed
  door; no new interiors. Assigning existing interior templates comes later (meta 2026-11-26, research §9 q2).
- Generated place names: yes, shown on the map only once discovered (meta 2026-11-26, §9 q3).
- Sites and hamlets come from MU via data, never a bespoke placer (owner 2026-11-24). A missing behaviour goes into
  our vendored copy, generic and data-driven, with a `VENDOR.md` patch-log entry.
- Climate classes come from WG-E1 (`worldgen.md:139-152`); the site table may read the class raster and moisture.

## Today (verified in code: V; from a summary: U)

- MU `Config.mode` is `auto | counts | list` (`settlements/mod.rs:57-58`, V). In `list` mode the requests are the
  list only (`settlements/placement.rs:797-798`, V): **no generated places are added**. A list entry with a
  `position` keeps it (`placement.rs:783`, `:854`, V); an entry without one is placed by MU's scoring. So "kept towns
  + generated hamlets" is data: kept towns with `position`, hamlets as positionless list entries. No patch needed.
- `Terrain::place` (`settlements/mod.rs:488-505`, V): placement, activity sites on resource anchors
  (`activities.rs:134`), rural worship, remote places (`remote.rs:504`), farms (`countryside.rs:111`; 1 = ~0.45/km2).
- `ActivitySite` (`activities.rs:27-62`, V): `id` `activity:<parent>:<anchor>` (stable, also the road key), `template`,
  `mode`, `parent`, `x, y, ground`, `access`, `access_class` (`track | trail | none`), `buildings[]` (archetype,
  look, ring), `installations[]`, `situation`, `approach`. Draws are keyed by parent key and anchor id, not by
  iteration order (`activities.rs:4-5`, V).
- Remote count: `rate * sqrt(land_km2 / 100) * frequency`, capped at `most` (`remote.rs:511-512`, V); data
  `activities.json:513-` has `rate` 1.2, `most` 12, `spacing` 2500 m (V). At 2500 km2 that is at most 12 remote
  places: far too few for 10x. Situations (summit, crag, cliff-ledge, cliff-foot, gorge, islet, marsh-island,
  deep-forest) and accesses (none, track, cliff-stair, rock-steps, tunnel, causeway, rope-bridge) are data (V).
  Templates `lost_ruins`, `gorge_tomb`, `forgotten_stones` (megalith or summit cairn) exist (`activities.json:509-511`, V).
- `remote_places > 10` is rejected (`mod.rs:597`); activity reach and budgets are code (`activities.rs:65-90`, V).
- MU data overlays: patch 02 `data::install_packs` merges JSON packs (`$from`, `$append`) before the first read
  (`VENDOR.md:82-118`, V). New templates, cultures and the remote block are packs, not vendored-file edits.
  Next free patch number: 04 (`VENDOR.md:119` is 03, V).
- `scrib-worldgen` roads: `sites.json` format 1 = `key, name, population, culture, centre_m, radius_m, laid_out`
  (`crates/roads/src/sites.rs:7-33`, V); `activity_access: false`, `network_nodes: false` ("no farms or activity
  sites yet", `crates/roads/src/run.rs:147-159`, V). `crates/settle` uses no MU activity or remote data (grep, V). No
  tool writes `sites.json` today; the S-2 runs used a hand-made file (U).
- Scrib: `sf_worldgen::roads` reads `roads.json` v1 (`roads.rs:1-11`, V); `sf_worldgen::measure::walk_slope_deg`
  and `walkability` (`measure.rs:778`, `:800`, V); scatter asks `Terrain::excluded` (`scatter.rs:346`, V), which the
  game answers with "stamp weight > 0" only (`mw_worldgen/src/flora.rs:335-337`, V); anchor footprints have
  `distance` (`anchors.rs:44`, V). Walk limit `MoverParams::max_slope_deg` 46 (`sf_controller/src/mover.rs:91`, V).
- Content path: `worldgen-town` turns a `placements.json` into a native mod with one `sf.placement_group`
  (`mw_worldgen/src/town.rs:1-3`, `apps/bevyrrowind/src/main.rs:162`, V); `worldgen-kit` builds `kit.json` from
  named authored anchors in one style (`kit.rs:1-3`, V). Map places: `MapPlace { discover: DiscoverRule::Near }`
  (`superfargoth/docs/specs/map.md:92-96`, specced).

## Canonical technique

**Constraint-based placement of authored kits from a site table, then a reachability pass.** Each row of a table
names an archetype, the ground it accepts (biome or climate class, slope, altitude), distance rules (from roads,
towns, water, its own kind), a density, and the kit it builds; candidates are scored, accepted with spacing, then
validated by a cost-distance flood from the travel network (GIS cost-distance, Tobler 1993 hiking function).
Flora and rocks keep out of each site's footprint plus a clearing; the ground under a building is flattened with a
falloff pad.

## Reuse check

- **Bevy 0.20:** not applicable, offline tools. At runtime sites are ordinary static content (`sf.placement_group`
  native mod, static scene, far field), map places (`sf_map`), and navmesh tiles baked once by `sf_bake` (missing
  or changed tiles rebuilt off the frame and cached, CONTRIBUTING 2026-11-26). No new runtime system.
- **Crates:** no maintained crate places POIs on terrain (research §3). `ghx_proc_gen` 0.10.0 (MIT/Apache, Bevy
  0.20) is WFC for kit and tile layouts: not needed while MU lays out site buildings; kept for ruin footprints later.
  `petgraph` 0.8.3 and `geo` 0.33.1 are not needed: the flood is a grid Dijkstra on `std` `BinaryHeap`,
  footprints are discs and convex rings.
- **Ours / MU:** MU already scores land, keeps fixed places, picks remote situations and accesses, ties activity
  sites to resource anchors and routes their access tracks (`Output::road_sites`, `settlements/mod.rs:204-224`).
  We adapt it through data plus one generic patch. Scrib keeps the readers and the validator (GPL, our own).

## Industry comparison

| Technique | Who ships it | Ours | Match |
|---|---|---|---|
| Location table: biome, altitude, min distance from similar, count, prefab | Valheim `ZoneLocation` (fan sources, U); Minecraft structure sets (spacing, separation, biome tags, U memory) | site table rows compiled to MU rules (patch 04) | match |
| Rule graphs with density, exclusion and difference | UE5 PCG (Epic docs, V search, research §4); Horizon placement (GDC 2017, U) | MU scoring + row predicates; clearing mask in scatter | match |
| Authored content first, prefab kits between | Wildlands, Far Cry 5; Bethesda pack-ins, UE Level Instances (U) | anchors kept; `kit.json` from anchors fitted to MU rings by `settle` | match |
| Same tile kit everywhere, no identity | Skyrim radiant dungeons (anti-example, memory) | rows keyed to situation and climate, names per culture | avoided |
| Reachability by cost-distance | GIS cost-distance; Tobler 1993 | `sf_worldgen::reach`, Tobler time, 46 deg wall | match |
| Flatten ground under a POI with falloff | Valheim location flatten (U), UE Landscape Patch | pad stamps in stamp format v1 | match |
| Local stability: a retune far away keeps ids | Horizon tooling goals (research §7, V slides) | ids from MU keys; MU placement is global | gap: pinned sites via override layer WG-T2 |

## Design

**Flow (each step a file in the guide folder, pure function of inputs + seed, blake3 of inputs in each header):**

1. `worldgen-sites` (SW `crates/sites`): guide heights + water (+ WG-E1 class raster when present, else the region
   map) + anchors + kept towns + `sites.toml` + MU packs → MU `Terrain::place` → `sites.json` format 2.
2. `worldgen-roads` (SW) reads format 2, routes towns, hamlets and activity accesses (`activity_access: true`).
3. `scrib-worldgen reach` (Scrib): final ground + `roads.json` + `sites.json` → `sites.valid.json` (status per site)
   and a census.
4. `scrib-worldgen site-pads` (Scrib): pads for kept sites appended to `stamps.json`.
5. Clearing mask: a pure query over `sites.valid.json` that scatter and rocks read through `Terrain::excluded`.
6. Content: hamlets ride the town path (`worldgen-site --guide` → `settle` → `placements.json` → `worldgen-town`,
   ledger S-2/SC-1); activity and remote sites go through `settle --activity` (kit fit of MU's building rings and
   installations) and one native mod with one `sf.placement_group` per site.
7. Names (WG-NM1) are written into `sites.json`; map places (WG-NM2) are records in the site mod.

**Site table (`sites.toml`, game data, generic schema).** One `[[row]]` per archetype:

| Key | Meaning |
|---|---|
| `id` | archetype id, unique (`ruin.crag`, `camp.nomad`) |
| `source` | `hamlet` (positionless list entries), `farm` (MU countryside), `activity` (MU templates on resource anchors), `remote` (MU remote templates), `node` (road network nodes) |
| `templates`, `culture` | MU template ids (from `activities.json` or a pack); `culture` for hamlets, else the parent's |
| `classes` | accepted climate classes or region names (empty = any) |
| `slope_max_deg`, `altitude_m` | over the footprint on MU's grid; `[min, max]` above sea level |
| `distance_m` | `town`, `road`, `water`: `[min, max]`; `same`, `site`, `anchor`: minimum gaps |
| `density_per_km2` | target over the land of the accepted classes; also the cap |
| `kit`, `footprint_m` | kit style id (`kit.json`) and footprint radius when MU gives no ring |
| `access` | `road | track | trail | none` (MU `access_class`) |
| `reach` | `foot | swim | any` (what V1 demands) |
| `clearing_m`, `pad` | flora-free margin beyond the footprint; flatten pad on or off |
| `name_key` | grammar key for WG-NM1 (`dunmer.ruin`) |

Unknown keys are an error. Placement-time rules (classes, slope, altitude, town, same, site, anchor, density) go to
MU. Post-road rules (road distance, reach) are V1's.

**MU patch 04, site rules (generic, data-driven).** Files: `settlements/mod.rs` (`Config.site_rules`,
`Terrain::set_class_plane(n, Vec<u16>, names)`), `settlements/activities.rs`, `remote.rs`, `countryside.rs`,
`placement.rs` (one candidate predicate), data key `remote.count_law: "sqrt" | "linear"` (default `sqrt` =
upstream). A `SiteRule` per template or list key: allowed class ids, `slope_max`, altitude range, minimum gaps,
`frequency`, `max_per_km2`. Class 0 means "no site" (our tool rasterises anchor footprints + bands + `anchor` gap
into it). Empty `site_rules` and no class plane: output byte-identical to today (the 59-test suite and the settle
gate stay equal). Patch-log entry 04 in `VENDOR.md`, its own `mu-patch 04:` commit.

**`sites.json` format 2** (guide frame metres, x east, y north; a superset of format 1, whose fields keep their
meaning): header `format, seed, guide_blake3, table_blake3, packs_blake3`; per site `key` (kept towns: their key;
generated: `site:<row>:<MU id>`, stable while MU's inputs are), `name`, `name_key`, `population`, `culture`,
`centre_m`, `radius_m`, `laid_out`, `kind` (`town | hamlet | farm | activity | remote | node`), `kept`, `row`,
`template`, `mode`, `parent`, `ground_m`, `yaw_deg`, `footprint` (ring), `access` [x, y], `access_class`,
`approach` (MU's access kind string, `cliff-stair`, `causeway`..; its line is an installation, `activities.rs:57`),
`situation`, `kit`, `clearing_m`, `pad`, `reach`, `buildings[]` and `installations[]` (MU serde, passed through),
plus format 1's `anchor` and header `producer`. V1 adds `status` (`placed | reseated | dropped`), `moved_m`,
`travel_s`, `via` (`foot | swim`), `reason` (why dropped). Readers check `format` first and reject unknown fields (settle convention). `sf_worldgen::sites`
is the generic Scrib reader; it names no game.

**WG-V1 reachability.** Grid at `reach.cell_m` over the final ground (guide heights, stamps, the runtime filter's CPU
twin), slope and water rebuilt from that field (stage rule: no stale masks). Sources: every town and hamlet
footprint cell and every road ribbon cell (`sf_worldgen::roads`). Cost per step = distance / Tobler speed,
`v = 6 exp(-3.5 |tan(theta) + 0.05|)` km/h; a step steeper than `MoverParams::max_slope_deg` (46) is a wall; water
cells cost `reach.swim_factor` times flat walking and count as `swim`. Multi-source Dijkstra, costs in integer
microseconds, ties broken by cell index: same result on any thread count. Per site: the best cell inside its
footprint gives `travel_s` and `via`; a site whose `reach` demand fails is re-seated to the cheapest reachable cell
within `reach.reseat_radius_m` that still meets its row (slope over the footprint, water gap, anchor gap); else
`dropped`, with the reason. Post-road rules (`distance_m.road`) are checked here too. `access: none` sites pass when
reachable as `reach` demands (no climbing model: a wall is a wall). Navmesh: nothing here; the bake's nav tiles see
sites as static geometry like anchors.

**WG-V1b clearings.** A pure query `Clearings::contains(x, y)`: inside any kept site's footprint + `clearing_m`, or
within `clearing.road_m` of a road ribbon edge. The game's scatter terrains answer `excluded` with stamps OR
clearings (`flora.rs:335`, `flora_towns.rs:231`); rock placement asks the same query. Bucket grid for speed; a tile
evaluated alone equals the whole run (pure point query).

**WG-P2 pads.** For `pad = true` sites: a stamp (format v1) of the footprint's median ground over the footprint,
smootherstep to zero across `pad.blend_m`. Refused (site re-checked, else dropped by V1's rules) when the cut or fill
exceeds `pad.max_cut_m`, when any pad sample lies below water level + `pad.water_margin_m` (wet/dry audit: no new
or removed water), or when its weight overlaps an anchor stamp's weight > 0 (anchor interiors stay bit-exact).

**WG-NM1 names (short design).** Port MU's TS name grammar and pack (51 grammars, 549 k names, capabilities row 26)
into the vendored Rust as data with a loader (patch 05), plus a Dunmer/Ashlander/Imperial grammar pack shipped by the
game. Each site draws from its `name_key` and culture, keyed by its `key` (stable), no duplicates in a world.
Names land in `sites.json` `name`. WG-NM2 writes one `sf.map.place` per kept site (`discover: Near`, kind from the
row) so names appear on the map only once discovered (map.md MAP1).

**Co-op:** an offline product, the same files on every peer; discovery stays per player in `MapReveal` (host).

## Adapter data (Bevyrrowind)

`bevyrrowind/worldgen/sites.toml` ships the rows; `bevyrrowind/worldgen/mu-packs/sites.json` the MU pack (Morrowind
templates, remote block `count_law = "linear"`, `most` raised). Classes: WG-E1b names when built, else region ids
(Ashlands, Bitter Coast, West Gash, Grazelands, Ascadian Isles, Molag Amur, Azura's Coast, Sheogorad, Red Mountain).
First rows (densities `Guess`): `camp.ashlander` (nomad camp, Ashlands/Grazelands/Molag Amur, slope <= 12, town >= 3
km, 0.02/km2, kit from Urshilaku camp); `ruin.daedric` (remote: crag, islet, marsh-island, 0.01/km2, access any);
`tomb.ancestral` (gorge or cliff-foot, sealed door, town <= 6 km, 0.03/km2); `mine.egg` (activity on `ore`
anchors, surface works, 0.01/km2); `shrine.wayside` (road <= 30 m, 0.02/km2); `stones.standing`
(`forgotten_stones`, 0.005/km2); `farm.hlaalu` (MU farms, 0.3/km2 of arable class); `hamlet.fishing` (coasts,
water <= 200 m, 0.004/km2); `node.waystation` (WG-P1e). Kits: `worldgen-kit` on named anchors per style; MU
cultures from the settlements lane (C-5).

## Slices

One deliverable plus its test each. Order: WG-P1a → WG-P1b (WG-P1d beside it) → WG-P1c → WG-V1 → WG-P2 → WG-V1b →
WG-P3a → WG-P3b → owner check WG-P-look → WG-P1e, WG-NM1 → WG-NM2. WG-E1b (classes) is optional input: P1b falls
back to the region map.

## Build rows

| Slice | Deliverable | Repo and crates (claims) | Depends | Test | Verify |
|---|---|---|---|---|---|
| WG-P1a | MU patch 04: class plane + `site_rules` + `remote.count_law` | SW `vendor/magna-urbis/rust/crates/core` (settlements/*), `VENDOR.md` | V-1 | a forbidden class gets no site; slope, altitude, gaps, cap respected; empty rules = byte-identical; 59 tests pass | settle gate unchanged |
| WG-P1b | `worldgen-sites`: table + packs → `Terrain::place` list mode → `sites.json` v2 | SW new `crates/sites` | P1a | same bytes twice and at 1 vs N threads; kept towns bit-exact; all sites dry, slope <= row max | census on `x10s` |
| WG-P1c | roads read v2, route activity access | SW `crates/roads` (`sites.rs`, `run.rs`) | P1b | v1 file gives byte-identical `roads.json` on the S-2 gate; v2 access tracks end at `access` | `roads.json` counts |
| WG-P1d | `sf_worldgen::sites` reader v2 | Scrib `sf_worldgen` (`sites.rs`) | spec | fixture round-trip; wrong format and unknown field rejected | `test -p sf_worldgen` |
| WG-V1 | `sf_worldgen::reach` + `scrib-worldgen reach` | Scrib `sf_worldgen` (`reach.rs`, bin) | P1c, P1d | island + bridge fixture: exact reached/dropped sets; 47 deg ridge is a wall; swim rule; reseat exact; same result 1 vs N threads | census print |
| WG-P2 | site pads into `stamps.json` | Scrib `sf_worldgen` (`sites.rs` pads, bin) | V1 | anchor interiors bit-exact; wet/dry audit 0; band slope <= 46 | stamp count |
| WG-V1b | clearing query + game scatter exclusion | Scrib `sf_worldgen` (`sites.rs`), `mw_worldgen` (`flora.rs`, `flora_towns.rs`) | V1 | no flora or rock inside a clearing; tile alone == whole run | flora census delta |
| WG-P3a | `settle --activity`: kit fit of MU rings + installations | SW `crates/settle` | P1c, kits | every building ring gets an instance or a counted skip; same bytes twice | stats per row |
| WG-P3b | site mod: one `sf.placement_group` per site | `mw_worldgen` (`town.rs`), app `main.rs` | P3a, P2 | groups == kept sites; instance count exact; same bytes twice | owner check WG-P-look |
| WG-P1e | road network nodes (waystations) as sites | SW `crates/roads`, `crates/sites` | P1c | nodes every 15-25 km of trunk, deterministic | census |
| WG-NM1 | names: MU grammar + pack loader (patch 05) + game grammar pack | SW vendor + `crates/sites`; data pack | P1b | deterministic; no duplicate in 1000 draws or in a world | names in `sites.json` |
| WG-NM2 | `sf.map.place` per kept site, `discover: Near` | `mw_worldgen` (site mod) | NM1, P3b, MAP1 | one place per kept site; rule and kind exact | map check later |

## As built

- **WG-P1d (2026-11-27)** `sf_worldgen::sites` (`sites.rs`, tests `tests_sites.rs`): `SitesFile::read/from_reader`
  checks `format` first (1 or 2), `deny_unknown_fields` on every object (a format-2 field in a format-1 file is
  unknown), duplicate keys refused; format 1 reads as kept `town`s, reach `any`, no access class; `to_json/write`
  always write format 2, empty fields skipped, so read(write(x)) == x. Required in format 2: header `seed` + three
  hashes, per site `key, centre_m, kind, kept`.
- **WG-V1.1 (2026-11-27)** `sf_worldgen::reach` (`reach.rs`, tests `tests_reach.rs`): `ReachGround::sample` (pure
  per-cell map; height = water surface over water), `ReachMap::flood` runs two serial Dijkstra floods, `foot` (water
  never entered) and `swim`; µs costs, lowest `(cost, swam)` kept, heap pops ties by cell index. Wall = the ground
  gradient under the step (2 x 2 cells for a diagonal, rise + cross gradient for a straight step) above
  `reach.max_walk_deg`, not the step's own rise: a diagonal across a 47 deg face would otherwise read 37 deg and leak.
  `best(cell, Reach)` gives `(travel_us, Via)`. `reach.*` values declared (`reach::declared`).

## Tunables

| Name | Unit | Range | Default | Source |
|---|---|---|---|---|
| `sites.<row>.density_per_km2` | 1/km2 | 0-1 | per row above | Guess |
| `sites.<row>.slope_max_deg` | deg | 0-46 | per row (12-30) | Guess |
| `sites.<row>.distance_m.*` | m | 0-20000 | per row | Guess |
| `sites.anchor_gap_m` | m | 0-1000 | 150 | Guess |
| `sites.<row>.clearing_m` | m | 0-60 | 12 | Guess (owner Q1) |
| `clearing.road_m` | m | 0-30 | 6 | Guess (owner Q1) |
| `reach.cell_m` | m | 2-60 | guide spacing (18.288) | quality/cost, Guess |
| `reach.max_walk_deg` | deg | read | 46 | `MoverParams::max_slope_deg` (`mover.rs:91`) |
| `reach.swim_factor` | x | 1-10 | 3 | Guess |
| `reach.reseat_radius_m` | m | 0-200 | 60 | one MU cell at 1024 over the 10x world, Guess |
| `pad.blend_m` | m | 2-30 | 8 | Guess |
| `pad.max_cut_m` | m | 0-10 | 3 | Guess |
| `pad.water_margin_m` | m | 0-5 | 1 | Guess |

MU's own numbers (`ATTACHED_M`, `REMOTE_M`, budgets, `remote.rate`, `most`, `spacing`) stay MU's; the pack sets the
data ones. Tobler's constants are maths.

## Performance

| Slice | Bake (s) | Runtime ms |
|---|---|---|
| P1b MU place (1024 raster, ~1500 sites) | 5-20 E (roads 8.8 s, ledger WR-2) | 0 |
| P1c roads with activity access | 10-30 E | 0 |
| V1 flood, 10.2 M cells | 3-8 E | 0 |
| P2 pads, V1b mask | < 1 E | 0 (mask is bake-time) |
| P3a settle per activity site | 30-120 E total | 0 |
| P3b, NM1, NM2 | < 2 E | 0 logic; ~1500 sites x ~20 = ~30 k statics (vs 7.2 M flora), counted at P3b |

Whole chain under a few minutes (E); measurements go in `docs/ledger/worldgen.md`.

## Tests

Exact assertions per row above. Fixtures: a 4 km island, one town, a ridge over 46 deg, a river with one bridge, an
islet. Same bytes on two runs and 1 vs N threads (P1b, V1, P2, P3b). Measurement (`#[ignore = "measurement: sites
census"]`): per row placed, re-seated, dropped, travel-time p50/p95 on `x10s`. No test judges the look.

## Verification (owner checks)

**WG-P-look** (after P3b): `bash bevyrrowind/tools/owner_launch.sh view "world:target/worldgen/x10s"` with the site
mod, camera at three sites from the census (a crag ruin, an ash-plain camp, a roadside shrine), noon, clear weather.
Questions: (1) Do sites sit on ground that suits them (ruins on crags and islets, camps on open plains)? (2) Is
anything floating, buried or cut by a pad edge? (3) Is there enough to find between two towns on foot? (4) Do the
clearings read as natural, not as stamped discs? (5) Should these sites get contents (creatures, a container) as the
next slice? Each "no" to 1-4 goes to a tunable or a row.

## Open questions (owner)

1. (Answered, owner via meta-5, 2026-11-26: yes.) Dense forest may block sight lines (Bitter Coast, Ascadian Isles),
   with guaranteed clearings at roads and sites; here it sets `clearing_m` and `clearing.road_m` (live tunables).
2. (Answered, owner via meta-6, 2026-11-26: yes, for remote rows: "many places won't have a road of course".)
   May some remote sites have no path at all (MU access `none`: reached across rough ground or by swimming, never by
   a track)? Recommend yes, for remote rows only.
3. (Not answered, owner via meta-6, 2026-11-26: cannot say before seeing sites.) Sites start as scenery for
   WG-P-look (the recommendation stands; it blocks nothing) and the owner decides on contents (levelled creatures, a
   container) at that first look, as its question 5. Not to be asked again before it.

## Later

Site contents (levelled spawns, loot) through the game's levelled lists; assigning existing interior templates to
tomb and mine doors; caves and dungeon entrances (`rockgen.md` RG-7); WFC ruin footprints (`ghx_proc_gen`); MU
decay and lore (C-11); pinned or forbidden sites from the override layer (WG-T2) for local stability.
