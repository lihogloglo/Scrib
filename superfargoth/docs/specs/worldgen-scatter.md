# Worldgen scatter v2: forest and flora structure (WG-E2; rocks WG-E3)

Stage 8 of [worldgen.md](worldgen.md) ("Nature scatter"), rewritten so vegetation reads as forest. Crates:
`sf_worldgen` (generic, Bevy-free) and `mw_worldgen` (adapter data), both Scrib GPL; nothing in `scrib-worldgen`.
Research: `../research/scatter-ecology-2026-11-26.md` (WG-R; its "Verified" section wins). Status: specced 2026-11-26.
Marks: V read in source, U from a summary, D derived by us.

## Goal

Owner 2026-11-19 on v1: "no proper forest (trees not snug together), rocks all one size, assets sprayed like bits on
a donut" (ledger WG-X5). v2 shows the three signatures of a real forest: clumps at stand scale, regular spacing at
crown scale, many small and few big trees, plus layers and clearings. A tile alone still equals the same tile of a
whole run, bitwise, on any thread count.

## Today (V)

- `Scatter::tile`: `floor(lambda keep A + u)` independent uniform points per (tile, species), one Bernoulli draw each
  against `density x occupancy x clump / lambda` (`scatter.rs:575-588`); clump `1 + 0.6 noise` at 48 m (`66-68`);
  scale uniform in the learned range (`597`). No point sees another: complete spatial randomness, hence the spray.
- Occupancy = slope class x height band (`scatter.rs:131-150`). Exclusion = `Terrain::excluded` (`345-346`); water =
  height under sea + margin (`593-595`), so inland lakes and rivers are not excluded.
- Budget `keep = min(1, t size^p)`, one `t` per group (`scatter.rs:436-468`); runs nest. Hash: splitmix64 `mix`,
  `stream`, `draw` (`472-495`). Tests: tile alone (`tests_scatter.rs:104`), threads and order (`125`), exclusion
  (`151`), budget (`173`, `189`).
- Caller: `mw_worldgen::flora::scatter_world` learns per region from the source's nature references
  (`flora.rs:204-283`) and runs `Scatter` (`flora.rs:416`) **at world load** (`view world:<dir>` ->
  `apps/bevyrrowind/src/world.rs:289` -> `flora::extra`, `flora.rs:475-497`). Two groups, flora and rocks
  (`flora.rs:56-60`). Wet-ground references skipped (`flora.rs:230-235`; 38,572, ledger).
- Climate (WG-E1, ledger 2026-11-26): `climate::{temperature, precipitation, moisture}`, `climate_classes` (12
  Whittaker classes), `climate_override`, `ClimateFile` are built. WG-E1c3 (rasters in the guide product) is not: the
  guide holds heights and regions only (`guide.rs:33-43`); drainage area is a function (`mfd.rs:81`), not a product.
- Clearings: `sites.md` WG-V1b specs `Clearings::contains(x, y)` (site footprint + `clearing_m`, road +
  `clearing.road_m`), answered through the game's `excluded` (`sites.md:166-168`). Not built.

## Canonical technique

**Rule and density-map placement with a marked cluster process and competition** (WG-R §1, §3): suitability from
fields, a stand mosaic, a Neyman-Scott (Thomas) parent process with a size mark per tree, a zone-of-influence
competition where the bigger crown wins, then lower layers by the light the canopy leaves. Time-stepped ecosystem
simulation is rejected (Makowski 2019, V: global state over years breaks tile-alone). Cordonnier 2017 (read in part) is
a cell model with no per-tree positions: it supports this choice and gives the hat viability combined by minimum.

## Reuse check

- **Bevy 0.20:** not applicable, offline CPU stage. Output reaches the screen through `sf_bigworld` (species the
  adapter flags) or placements (`big-world-rendering.md` "Hooks").
- **Crates** (crates.io 2026-11-26, WG-R §6, V): `fast_poisson` 1.0.2, `faster_poisson` 1.3.0, `bluenoise` 0.2.2
  (single-region Poisson disk, own RNG, not tile-hash-stable), `poisson` 0.10.1 (2019), `map_scatter` 0.4.1 (not
  tile-deterministic, U), `rstar` 0.14.0, `kiddo` 6.3.2 (indices). None does tile-deterministic cluster plus
  competition; none taken.
- **Ours:** keep the counter hash and the `Terrain` trait; add a uniform bucket grid and a 40-line test g(r).

## Industry comparison

| Technique | Who ships it | Ours | Match or gap |
|---|---|---|---|
| Density maps per layer | Horizon (V), UE PCG (U), FC5 (U) | suitability from fields | match |
| Mosaic: sparse, edge, inner, clearing | Horizon "Forest inverted gives Clearing" (V) | stand pass | match |
| Spacing by footprint, bigger wins | Horizon dither (V); UE Collision Radius, Overlap Priority (V) | `R_ij = k (r_i + r_j)`, size priority | match |
| Seeding around parents, few steps | UE Num Steps 3-4, Spread Distance (V) | Thomas offspring | match (no ageing) |
| Shade layer | UE Shade Radius, Can Grow in Shade (V); Makowski `stol` (V) | understorey by light | match |
| Cascade big to small | Horizon layers (V), Wildlands (U) | rocks, canopy, understorey, ground | match |
| Debris below cliffs, sized | Houdini scree workflows (U); no game talk found (WG-R §4.7) | talus fall process, Pareto sizes | match (our design) |
| Rocks prune foliage | UE PCG Difference, Self Pruning (U) | rocks first, footprints as obstacles | match |
| Steady-state species competition | Ecormier-Nocca 2021 (V) | suitability min only | gap by choice (Later) |
| GPU placement per frame | Horizon, ~250 us (V) | CPU, once, cached file | gap by choice (co-op safe) |

## Design

Each pass is a pure function of (seed, fields, species table, tile). Layer order per tile: rocks (WG-E3, reserved
first), canopy, understorey, ground flora. "Shared" = the rocks slice reuses it as is.

1. **Fields** (shared). Trait `Fields`: temperature C, precipitation mm/yr, moisture 0..1, class id, slope, aspect,
   altitude, curvature, distance to water. Providers: the guide product's rasters (after WG-E1c3), bilinear; neutral
   constants for tests.
2. **Suitability** (vegetation). Per species and driver a hat `[e_min, i_min, i_max, e_max]` (0 outside, 1 inside,
   linear between; Gain 2017 via Cordonnier), combined by minimum (Liebig: Nocca V, Cordonnier V). Optional class list.
   Species mix along gradients by overlapping hats, no extra rule.
3. **Stand mosaic** (vegetation). `D(x)` = value noise at `stand_wavelength_m` x the best canopy suitability, decoded by
   thresholds into clearing, sparse, edge, inner. Forced clearings = `Terrain::excluded` (stamps OR WG-V1b). Distance to
   the nearest clearing, on a tile-plus-halo grid at `field_step_m`, capped at `edge_band_m`, gives the edge band: the
   regrowth fringe round every clearing.
4. **Candidates** (shared hashing). Truncated Thomas. Parents per parent tile: `n = floor(kappa_max A + u)` from
   `hash(seed, species, parent tile, j)`, thinned at the parent by `kappa / kappa_max` (Lewis-Shedler), `kappa =
   parent_per_ha x suitability x D x region_prior`. Offspring: Poisson(`offspring_mean x suitability`) by inversion,
   Gaussian offset `cluster_sigma_m` by Box-Muller from the parent's stream, rejected beyond 3 sigma (exact reach). Plus
   a uniform background stream per tile at `background_fraction`.
5. **Size mark** (shared form). Per candidate, before competition: reverse-J (truncated exponential in height,
   `size_min_m..size_max_m`) or Weibull `size_shape`, by inversion; in the edge band the quantile is scaled by
   `edge_size_ratio` (big trees in the core). Crown radius `r = crown_a_m + crown_b h` (crown width linear in DBH for
   open-grown trees, Brack and Wood, snippet V).
6. **Competition** (shared). Conflict when `dist < R_ij = crown_k (r_i + r_j)`; priority (size mark, hash). Fixed-
   priority parallel maximal independent set for `competition_rounds` rounds (Luby 1986, U): round 1 keeps a candidate
   with no higher-priority conflict (Matern II); round r re-admits a loser free of earlier winners and of
   higher-priority eligible round-r contenders. Winners never conflict. Never "until converged".
7. **Lower layers** (vegetation). Light `L = exp(-sum_i a_i)`, `a_i = shade_per_crown` inside canopy winner i's crown
   (WG-R §3; Makowski `Q_eff = lerp(stol, 1, Q)`, V). Intensity x `lerp(stol, 1, L)`, x `(1 + edge_boost)` in the edge
   band. Same passes 4-6 inside the layer; earlier layers' trunk and rock footprints are hard obstacles.
8. **Exclusion** (shared). Dropped where `excluded`, on the sea (v1 rule), inside a lake or within a river half width
   plus `water_margin_m` (`water::GuideWater`, V), outside the slope hat, or outside a region the species lives in.
   Shore species use the `water_distance_m` hat.
9. **Pose** (shared form). Uniform yaw; scale = size mark / model height; up and jitter by the v1 rule
   (`scatter.rs:598`); height at the point plus `offset_m x scale`.
10. **Rank and budget** (shared). Each kept instance has a hashed rank `u`; it is kept when `u < min(1, t_g size^p)`,
    one `t_g` per group by the v1 bisection: budgets nest exactly, smallest cut first, gaps left open (LOD-style; no
    re-competition). Groups: canopy, understorey, ground, rocks (WG-E3).

**Determinism across tiles.** Tile T rebuilds every parent within `H` of its square from the parent's own stream, runs
the passes on the grown square and keeps instances inside T (v1 membership). State after round r depends on points
within `(2r - 1) R_max` (D), so `H = 3 sigma_max + (2k - 1) R_max` plus the crown reach for lower layers (the research's
`k R_max` is too short). Output order is the integer key (layer, species, parent tile, parent, offspring). Membership is
decided by integer counts and hash comparisons only.

**Product.** Hash-cached like every stage: the kept set is written once to the guide folder, keyed by a blake3 of the
species file, declared values, seed and field rasters (`hash_f32`, `run.rs:664`, V), read at load. The budget cut stays
at load (`--set scatter.budget=N` needs no rebuild).

**Counts per biome class** (canopy stems/ha, class mean with its mosaic; Guess from `N = 10,000 f / (pi r^2)`, D):
desert, cold desert, tundra 0-5; grassland 5; shrubland 10; savanna 20; woodland 60; taiga 250; temperate and
seasonal forest 150; temperate rainforest 250; rainforest 300 (per km2: x 100). Inner stands are capped at
`canopy_stems_per_ha_max` 400 per tile in rank order, the "stand" case of `big-world-rendering.md` "Cost model" (V).
BW-2 measured that case over budget (13.97 ms GPU, "Tripped"): the cap and `scatter.budget` are the levers.

**Groundcover border** ([groundcover.md](groundcover.md)): a scatter instance blocks, collides, casts far shadow, is
interactive (harvestables) or is seen beyond groundcover's `Mid` range; blades and small near-camera flora are
groundcover. Groundcover avoids trunks through `BigworldFootprints`.

**Stage rules** (`worldgen.md:172`): reads heights and water, writes neither: wet/dry audit zero, anchors bit-exact.
**Co-op:** the product is an offline file, the same on every peer.

## Adapter data (Bevyrrowind)

- `bevyrrowind/worldgen/scatter_species.toml`: one row per species plus a `[columns]` table (unit, range, source per
  column; the WG-E1c2 `[override_columns]` pattern). **One canopy tree species** to start (owner 2026-11-18); the
  source's shrubs and flora as understorey and ground rows. Species are data (owner 2026-11-11: generic, not pack-tuned).
- **Learned** (`mw_worldgen::flora`): species and model, size bounds (scale range x extent), pose (`align`,
  `tilt_jitter_deg`, `offset_m`), slope hat `e_max` (v1 `max_slope_deg`), and a **region prior** (learned density over
  its class mean, clamped to `region_prior_max`) on parent intensity. The mean density is no longer replayed.
- **Authored:** layer, allometry, cluster sigma and offspring, shade tolerance, hats, classes.
- **Decided, owner 2026-11-26 (via meta-5):** dense forest may block sight lines in regions like the Bitter Coast and
  Ascadian Isles, with guaranteed clearings at roads and sites; `canopy_stems_per_ha_max` and WG-V1b's `clearing_m`,
  `clearing.road_m` are live tunables. Their WG-E1c2 overrides put them in forest classes.

## Rocks (WG-E3)

Rocks come from sources and obey transport (WG-R §4); the layer runs first, reusing passes 4, 6, 8, 9, 10.
**Today (V):** v1 learns only `terrain_rock*` and rocks of 14 m or more (`classify.rs:118-131`); scale is uniform in
that narrow range (`scatter.rs:597`): "all one size". The RG-2 library is not wired in (`rockgen.md:262-265`).

**Inputs (V).** Guide heights; `geology/` products `unit.u8` and `cliff.u8` (`geology.rs:28`, written by placement,
`placed/mod.rs:987-996`); `RockUnit::outcrop` (`geology.rs:164`). Scarp cell = `cliff.u8 > 0` OR slope >=
`scarp_slope_deg` (fault, stroke and eroded cliffs alike). No geology product: the region's default unit (adapter).

**Three contexts**, candidate streams of pass 4 with a size mark `s` (longest axis, `rockgen.md:116`):
1. **Talus.** Parents on scarp cells at `talus_parent_per_ha` x the unit's `rockfall`. An offspring draws `s`, then
   run-out `x ~ exp(-x / ell)`, `ell = fall_ell_m (s / size_min_m)^sort_beta`, cut at `reach_max_m`, along the
   parent's downslope +- `talus_spread_deg`. Kept if below the shadow-angle line, `z_parent - z_land >= x
   tan(shadow_angle_deg)` (Evans and Hungr 1993 minimum 27.5 deg, abstract V), and on slope <= `loose_slope_max_deg`.
   **Large blocks go furthest, to the talus toe** (Statham 1976, Evans and Hungr, U), as a tendency: a 2021 cone
   survey found sorting weak or absent (V), `sort_beta` 0 turns it off. Density falls away from the scarp.
2. **Outcrop (parent block and debris).** In units with `outcrop`, Thomas parents at `outcrop_parent_per_ha`;
   the parent is a rock from the top of the law (`s >= outcrop_parent_min_m`), its children
   (Poisson `child_mean`) from the law cut at `child_size_ratio x s_parent`, Gaussian offset `cluster_sigma_m`.
3. **Field stone.** Per unit, `field_stone_per_ha` in small Thomas clumps, `field` size range, x the slope hat.

**Size law.** Truncated Pareto, `s = s_min (1 - u (1 - (s_min / s_max)^D))^(-1/D)` (WG-R §4.2), `N(>s) ~ s^-D`
(Turcotte 1986, U; Ord 2021 prefers other laws: `size_law` is a column). Below ~0.3 m groundcover takes over.

**Competition and exclusion (shared).** Pass 6 with footprint `r = s / 2`, `R_ij = rock_k (r_i + r_j)`, `rock_k`
0.9 (rocks may touch), priority (size, hash): the big block keeps its spot, debris fills round it. Pass
8 as is (roads, sites, anchors, forced clearings, sea, lakes, rivers). **Rock over tree:** rocks are reserved first;
their footprints are hard obstacles for later layers (pass 7; UE PCG Difference and Self Pruning, U). **Budget:** its
own group and `t_g` (pass 10; WG-R §9.3), debris cut first; share by the v1 rule (`scatter.rs:436-468`), ~45 % in WG-X5.

**Tile independence.** As vegetation, with `H_rock = max(reach_max_m, 3 cluster_sigma_m) + (2k - 1) rock_k s_max`
(D); offspring come from the parent's stream, scarp cells and units are pure functions of guide position. Bake
seconds: measured by E3m; runtime 0 ms from the stage (same product file).

**Mesh pick (recipe data).** Per rock unit and context, weighted entries of two kinds: a pack's own rock models stay
in the scatter next to the library (owner 2026-11-26, question 3). A **library class** (`sf_rockgen`
`classes.rs:44-157`, 9 classes) fits any `s`: variant `hash(instance, 2) mod N` (`rockgen.md:119-120`), scale `s /
class size` (`rockgen.md:364`). A **pack model** (a model the adapter supplies with its longest axis and allowed
scale range) is eligible only when `s` lies in `axis x [scale_min, scale_max]`, drawn at scale `s / axis`, with its
own collision. Entry by `hash(instance, 1)` over the eligible weights, pack models x `rock.pack_model_share` and
classes x its complement (0 = library only, 1 = a pack model wherever one fits: the A/B arms). No stretch:
`aspect_spread` varies shape, hulls stay exact (`sf_physics` statics). One size law and one pose rule for both kinds.

**Pose.** Uniform yaw; up = v1 `align` blend + up to `tilt_jitter_deg`. Sink = `lerp(sink_small, sink_large, t)` of
the short axis, `t = ln(s / s_min) / ln(s_max / s_min)` (WG-R 10-40 %, `rockgen.md:156` 20-40 %): small stones sit
deep, big blocks high. Loose rocks above `loose_slope_max_deg` drop; outcrop parents are exempt.

**Adapter data (Bevyrrowind).** Rock rows and `[rock_units]` in `scatter_species.toml` for the six units of
`worldgen/geology.toml`. **Learned** from the 33,017 rock references (`flora.rs:86-113`): a field-stone region prior,
pose `align`, `offset_m`. **Authored:** classes per unit and context, from the
typology table (`rockgen.md` "(e) Typology", RG-9; talus / outcrop / field): basalt_flow `talus`, `jagged` /
`columnar`, `block` / `jagged`, `crust_plate`; tuff `soft_block`, `block` / `soft_block` / `soft_block`, `cobble`;
tephra `scoria` / none / `scoria`, `bomb`; capped_sediment `slab`, `platy` / `slab` / `platy`; coastal_basalt
`column_segment`, `talus` / `columnar` / `beach_column`, `cobble`; alluvium `river_cobble` / none / `river_cobble`,
`cobble` (classes that need RG-9c-g join as those rows land). Per weathering regime (frost, arid, semi-arid,
temperate, wet; RG-9j) the entry lists and `field_stone_per_ha` may be overridden. `rockfall`, size ranges. **The source's rock models stay** (owner 2026-11-26):
each learned model (`terrain_rock*`, rocks and menhirs of 14 m or more, `classify.rs:118-131`) is a pack-model entry
with its bounds axis, learned scale range and learned count per region as weight; the library fills the sizes they
do not cover. The mix is looked at again once the owner has seen generated rocks (WG-E3 check).

## Slices

One deliverable plus its test each. E2a-E2h need no climate product (neutral `Fields`). E2i and the owner look wait
for WG-E1c3 and WG-V1b. Order a to h, then i, j, m. Rocks: E3a-E3d after E2e (they reuse E2d-E2e), E3e after E2i.

## Build rows

| Slice | Deliverable | Repo and crates (claims) | Depends | Test | Verify |
|---|---|---|---|---|---|
| WG-E2a | species file `SpeciesFile::parse_toml`, `[columns]` check, declared values | Scrib `sf_worldgen` (`scatter_species.rs`) | none | round trip; bad range or column rejected; census passes | `test -p sf_worldgen` |
| WG-E2b | `Fields` trait, neutral and grid providers, hat suitability | Scrib `sf_worldgen` (`scatter_fields.rs`) | E2a | exact hats on fixture points; share monotone on a ramp | same |
| WG-E2c | stand mosaic and edge band with forced clearings | Scrib `sf_worldgen` (`scatter_stand.rs`) | E2b | classes partition; band width exact on a disc; tile alone | same |
| WG-E2d | Thomas candidates across the halo, size marks | Scrib `sf_worldgen` (`scatter.rs`) | E2c | tile alone bitwise; g(r) > 1.5 at clump scale; size slope within 10 % | same |
| WG-E2e | bounded-round competition, bucket grid | Scrib `sf_worldgen` (`scatter.rs`) | E2d | no winners closer than `R_ij`; g(r) ~0 below 0.8 R; tile alone; threads | same |
| WG-E2f | lower layers by light, footprint obstacles | Scrib `sf_worldgen` (`scatter.rs`) | E2e | none in a trunk footprint; mean L higher than at uniform points | same |
| WG-E2g | inland water exclusion, shore hat | Scrib `sf_worldgen` (`scatter.rs`) | E2f | nothing in lake, river, footprint, excluded cell | same |
| WG-E2h | rank budget by group (v1 deletion moved to E2i) | Scrib `sf_worldgen` (`scatter.rs`) | E2g | budgets nest exactly; group shares; v1 tests ported | same + clippy dependents |
| WG-E2i | adapter: species TOML, learned priors, four groups, product `Fields`; the v1 Bernoulli path deleted | Scrib `mw_worldgen` (`flora.rs`), `bevyrrowind/worldgen/scatter_species.toml` | E2h, WG-E1c3, WG-V1b | `tests_flora` tile alone; region prior exact | `test -p mw_worldgen` |
| WG-E2j | hash-cached product in the guide folder, read at load | Scrib `sf_worldgen` (`scatter_product.rs`), `mw_worldgen` | E2i | same hash twice; seed change rebuilds; read == computed | same |
| WG-E2m | measurement: stems/ha and cover per class, counts, seconds | Scrib `mw_worldgen` (test only) | E2j | `#[ignore = "measurement: scatter v2 counts and seconds"]` | ledger line |
| WG-E3a | rock columns, `[rock_units]` table (entries: library class or pack model) | Scrib `sf_worldgen` (`scatter_species.rs`) | E2a | round trip; bad weight or unknown entry kind rejected | `test -p sf_worldgen` |
| WG-E3b | talus candidates, Pareto sizes | Scrib `sf_worldgen` (`scatter_rocks.rs`) | E2e, E3a | size slope within 10 % of D; size-distance Spearman > 0.08 (beta 0.4; was 0.3, unreachable with the exponential run-out, see As built), rising with beta, ~0 (beta 0); none above the shadow line; tile alone | same |
| WG-E3c | outcrop clusters, field stones | same file | E3b | children <= ratio x parent; g(r) > 1.5 at sigma; tile alone | same |
| WG-E3d | rock layer first in the passes | Scrib `sf_worldgen` (`scatter.rs`) | E3c | no pair under `R_ij`; none on water, roads, footprints; no tree in a rock | same |
| WG-E3e | mesh pick (both entry kinds) and pose | `scatter_rocks.rs` | E3d | entry shares within 3 sigma; no pack model outside its scale range; share 0 = library only; sink exact | same + clippy dependents |
| WG-E3f | adapter rock rows, pack-model entries from the learned rock models, field-stone prior | Scrib `mw_worldgen` (`flora.rs`), `scatter_species.toml` | E3e, E2i, RG-2b | `tests_flora` tile alone; prior exact; every learned rock model is an entry with axis and scale range | `test -p mw_worldgen` |
| WG-E3m | measurement: rocks/ha, size slope, share, seconds | `mw_worldgen` (test only) | E3f, E2j | `#[ignore = "measurement: rocks"]` | ledger line |

## As built

- **WG-E2a** (2026-11-26, `scatter_species.rs`): `SpeciesFile::parse_toml` / `to_toml` (exact round trip); `[params]`
  = 12 declared `scatter.` values (a name left out keeps its default; stand thresholds strictly ordered, whole round
  count), `[columns]` = 14 numeric columns (range inside the declared one, `species_declared`) + 6 `hat_*` columns,
  `[classes."<name>"] stems_per_ha` over the 12 shipped class defaults, `[[species]]` rows (a value only in a declared
  column; a left-out value takes the declared default, checked against the file's column range). 6 tests.
- **WG-E2b** (2026-11-27, `scatter_fields.rs`): trait `Fields` (`sample(x, y) -> FieldSample`, `class_names`);
  `NeutralFields` (one sample) and `GridFields` (f32 rasters in the guide frame, row 0 north, `check()`; bilinear, the
  class by nearest sample, the aspect blended as a unit vector across north). `hat` (NaN -> 0, step hats exact);
  `Suitability::new(species, class_names)` = minimum of the hats, 0 outside the class list. 6 tests.
- **WG-E2c** (2026-11-27, `scatter_stand.rs`): `StandRules` (`density` = value noise mapped to [0, 1] x the best
  canopy suitability, 0 with none; `class_of` by the three thresholds) and `StandMap::build(rules, fields, excluded,
  rect)`: clearing nodes (mosaic or forced) on a world-aligned `field_step_m` grid grown by the band + one step,
  `measure::distance_transform`, capped at `edge_band_m`, bilinear; `at` -> `StandPoint` (D, class, forced, distance,
  edge band). 3 tests (partition on 1 km2; disc band: nodes exact vs brute force, width within one step on 64 rays; tile alone bitwise).
- **WG-E2d** (2026-11-27, `scatter.rs` v2 section): `CandidateRules::candidates(fields, stand, rect)` per species and
  parent tile (`scatter.tile_m`): `floor(kappa_max A + u)` parents thinned by suitability x stand x prior (canopy stand
  = D, 0 in a clearing; lower layers 1 until E2f), Poisson offspring by inversion, Box-Muller offsets cut at 3 sigma,
  background share `b` (suitability squared); `size_mark` (reverse J of rate `size_shape / range`; Weibull of scale
  the range, truncated), canopy edge quantile x `edge_size_ratio`; `tile_rect` = tile + `(2k-1) R_max`, `halo_m`. 4 tests.
- **WG-E2e** (2026-11-27, `scatter.rs` v2 competition): `compete(file, cands, rounds)` per layer; conflict when
  `dist < R_ij = k_i r_i + k_j r_j` (`crown_k (r_i + r_j)` for one species, never past `R_max`); priority size, hash,
  key; each round a rayon map (eligible and no higher eligible conflict wins; a winner's neighbours are out); CSR
  bucket grid of cell `max(R_max, sqrt(area / n))`. `CandidateRules::tile_winners` = candidates over `tile_rect`,
  compete, membership. 4 tests (grid == brute force; no pair under `R_ij`; rounds only add; g(<0.8 R) = 0; tile alone; threads).
- **WG-E2f** (2026-11-27, `scatter.rs` v2 lower layers): `square_winners` / `tile_winners` run the layers in pass
  order (candidates per layer over the square grown by `layer_grows`: `grow_l = max_m (grow_m + max(foot_l + foot_m,
  crown_l)) + (2k - 1) R_max(l)`), each layer's winners added to a `Cover` (canopy crowns: `L = exp(-shade_per_crown
  n)`; every stem `footprint_ratio x size`, new species column, Guess 0.01). Lower candidates: dominating intensity x
  `(1 + edge_boost)`, kept at the point with `lerp(stol, 1, L) x (1 + edge_boost in band) / (1 + edge_boost)`; dropped
  when their footprint overlaps an earlier stem. One CSR `Buckets` grid shared with the competition. 3 tests.
- **WG-E2g** (2026-11-27, `scatter.rs` v2 water): `WaterMask::new(&GuideWater, water_margin_m)` / `sea` / dry
  `Default`; `CandidateRules` gains `water` and `height`. Each instance is dropped at its own point in a forced clearing
  (now every layer), on the sea, in a lake (holes are dry) or within the margin of a lake shore or river bank (the
  river the swept discs of its interpolated half width, exact); offspring also by region prior 0 and the slope and shore
  hats at the point. The shore hat reads `min(fields, exact inland distance)`. The margin also rings lakes. 2 tests.

- **WG-E2h** (2026-11-27, `scatter.rs` v2 budget): `budget_rank(seed, key)` (its own hash stream) and
  `budget_cut(seed, instances, budget, size_priority)`: per layer the `round(budget x n_l / n)` instances of smallest
  `u / max(size mark, 0.1)^p`, ties by key (`select_nth`), all when `budget >= n`. v1 is kept until E2i: deleting it
  breaks `mw_worldgen::flora` (it calls `Scatter`, `Learner`, `budget_keep`), which only E2i may rewrite. 4 tests
  (v1's tile-alone, order and threads, slope, region and sea tests ported to four halo tiles; nest, shares, order).
- **WG-E3a** (2026-11-27, `scatter_species.rs`): `[rocks.talus|outcrop|field]` rows (`size_min_m`, `size_max_m`
  required, in the species columns; `sink_small`, `sink_large`, `align`) and `[rock_units."<unit>"]` (`field_stone_per_ha`,
  `rockfall`; per context a list of `RockEntry`: `kind = "class"` (library class by name, resolved at E3e) or `"model"`
  (`axis_m`, `scale_min..scale_max`); weight finite > 0, no repeat). Four rock columns in `rock_declared`. The scatter-wide
  `rock.*` values are declared by the rows that use them (E3b-E3e). 2 tests (round trip; 23 refusals).
- **WG-E3b** (2026-11-27, `scatter_rocks.rs`): `RockValues` (10 `scatter.rock.*` values in the file's `[params]`, new
  `talus_child_mean` 4), trait `RockGround` (height, slope + aspect, cliff, unit name), `pareto_size`, `RockRules::
  talus_candidates(ground, rect)`: parents per tile thinned to scarp cells x unit `rockfall`, Poisson blocks, run-out,
  shadow line + loose slope; `RockKey` order, `source` kept. On a 60 m 80 deg scarp, 23k blocks: slope of N(>s) 1.64 /
  2.06 / 3.03 for D 1.6 / 2 / 3; Spearman 0.014 / 0.120 / 0.214 at beta 0 / 0.4 / 1. The row's > 0.3 is not reachable
  with exponential run-out (var ln E = pi^2/6 swamps beta^2 var ln s; simulated 0.12-0.24 over D 1-2), so the bar is
  0.08 and rising with beta (field sorting is weak, V). 4 tests.
- **WG-E3c** (2026-11-27, `scatter_rocks.rs`): `outcrop_candidates`, `field_candidates` (shared `for_parents`, Gaussian
  offset truncated at 3 sigma). "Units with `outcrop`" = units whose `[rock_units]` table lists `outcrop` entries (the
  adapter maps the geology flag). Parent = offspring 0, no loose-slope drop; debris cut at `ratio x s_parent` (none if
  below `size_min_m`). Field: three new values (sigma 2 m, 5 stones a clump, slope hat full to 15 deg, 0 at the loose
  limit). Plain fixture: 1334 parents, 7955 children all <= 0.4 x parent; g(0.5-1 sigma) outcrop 3.06, field 9.83;
  6420 stones on 6400 expected. 3 tests.
- **WG-E3d** (2026-11-27, `scatter.rs`): `CandidateRules.rocks: Option<RockLayer { rules, ground }>`; `square_layers` /
  `tile_layers` -> `LayerWinners { rocks, plants }` (`square_winners` = the plants). Rocks run first over the square grown by
  `rock_grow_m` = max over plant layers (`grow_l + s_max / 2 + foot_l`) + `(2k - 1) rock_k s_max`; dropped at their point in a
  forced clearing or on water, one competition over the contexts (`scatter.rock.rock_k`, declared here), footprints `s / 2`
  into `Cover` as obstacles. The rock budget group is not wired (`budget_cut` takes plants). 2 tests.
- **WG-E2i.1** (2026-11-27, `climate_map.rs`, `scatter_fields.rs`): `ClimateMap::read(dir)` loads `dir/climate/`,
  refusing a wrong format, size or blake3 (per file and product) and out-of-range class ids; `climate.json` gains
  `budget` + `mfd_stuck` (older products read 0). `GridFields::from_products(&Guide, &ClimateMap)`: climate rasters as
  is; `terrain_shape` = Horn slope/aspect + Florinsky plan curvature (1/m, convex +); sea distance by exact EDT
  (`NO_WATER_M` without sea; inland water stays E2g's exact distance). 3 tests.
- **WG-E2i.2** (2026-11-27, `bevyrrowind/worldgen/scatter_species.toml`, `mw_worldgen::flora::SPECIES_FILE`): one canopy
  `tree` (reverse-J 3-20 m, every class with canopy stems), `shrub` (understorey, stol 0.5) and `flora` (ground,
  Weibull 0.2-1 m, stol 0.8); no model named (filled from learned refs at E2i.3); the 12 class stems stated over the
  shipped Whittaker names; `[params]` on defaults. 1 test (parses; 1 canopy row; every named class in the table).
- **WG-E2i.3** (2026-11-27, `mw_worldgen::flora`, `scatter.rs`): flora base records become models (`Slot`) of the row
  nearest their size (log distance to the range's geometric centre); learned size bounds, count-weighted pose, slope
  `e_max` replace the authored columns, rows without a model dropped; `region_prior` = `D(r) / M(c)`, `M` the
  area-weighted class mean on the guide (each class keeps its authored intensity; rocks counted, placed from E3f).
  Fields from the guide's `climate/` (none: the pass logs the `scrib-worldgen climate` command and places nothing).
  New in `scatter.rs`: `CandidateRules::tiles` / `run_tiles` (tiles in parallel) and `budget_cut_layers` (four
  groups, rocks on their own rank stream). Model pick: slots whose size range holds the mark, weighted by region refs.
  Fixture: 1379 trees + 14707 ground plants; with a town and road, 864 fewer stood in them yet the total rose
  (16784: cleared canopy lets the lower layers in). 6 tests + 1 sf_worldgen budget test.
- **WG-E2i.4** (2026-11-27, `scatter.rs`): the v1 path deleted (tag `dead/scatter-v1`): `Learner`, `Table`, `Entry`,
  `Scatter`, `Terrain`, `area_census`, `expected`, `budget_keep`, `aligned_up`, `tilt` and `tests_scatter.rs` (7 v1
  tests; its tile-alone, threads, slope and sea tests live on in E2h's halo tests). `Params` keeps the seven run values
  the adapter reads (`tile_m`, `seed`, `align_min_tilt_deg`, `slope_margin_deg`, `water_margin_m`, `budget`,
  `size_priority`) plus `sea_level_m`; ten v1 values left `declared()`. scatter.rs 1898 -> 1358 lines.

## Tunables

Prefix `scatter.`; source Guess unless noted. Species columns go in the file's `[columns]` table.

| Name | Unit | Range | Default | Source |
|---|---|---|---|---|
| `stand_wavelength_m` | m | 50-1000 | 250 | Guess (WG-R §8) |
| `stand_clearing_threshold`, `_sparse_`, `_inner_` | ratio | 0-1 | 0.35, 0.5, 0.7 | Guess |
| `edge_band_m` / `edge_size_ratio` / `edge_boost` | m / ratio / ratio | 5-60 / 0.2-1 / 0-4 | 25 / 0.6 / 1.0 | Guess |
| `field_step_m` | m | 1-16 | 4 | Guess (WG-R: 4-8 m) |
| `competition_rounds` | count | 1-6 | 3 | Guess (D) |
| `shade_per_crown` | ratio | 0.2-3 | 1.0 | Makowski form (V), value Guess |
| `canopy_stems_per_ha_max` | 1/ha | 50-2000 | 400 | big-world-rendering.md "stand" (V) |
| `region_prior_max` | ratio | 1-10 | 3 | Guess |
| `class.<name>.stems_per_ha` | 1/ha | 0-2000 | table above | Guess |
| `tile_m`, `seed`, `water_margin_m`, `budget`, `size_priority` | | | v1 | kept (`scatter.rs:49-73`) |
| species `parent_per_ha` | 1/ha | 0-500 | 4 | Guess |
| species `cluster_sigma_m` / `offspring_mean` | m / count | 2-40 / 1-60 | 12 / 12 | Guess (Thomas, U) |
| species `background_fraction` | ratio | 0-0.5 | 0.2 | Guess |
| species `size_law`, `size_shape` | enum, ratio | 0.5-6 | reverse-J, 1.2 | Guess (Weibull, de Liocourt U) |
| species `crown_a_m`, `crown_b` | m, ratio | 0-5, 0-0.5 | 0.75, 0.2 | Brack and Wood form (V snippet) |
| species `crown_k` | ratio | 0.3-1.2 | 0.8 | Guess (zone of influence, U) |
| species `shade_tolerance` | ratio | 0-1 | 0.3 | Makowski `stol` form (V) |
| species `footprint_ratio` | ratio | 0-0.5 | 0.01 | Guess (stem radius per metre of size; slenderness H/D 25-100, U) |
| species hats (T, P, moisture, slope, altitude, water distance) | per driver | 4 values | open | Guess |
| `rock.size_dim` D / `scarp_slope_deg` / `loose_slope_max_deg` | ratio / deg / deg | 1-3.5 / 40-75 / 30-60 | 2.0 / 50 / 45 | Turcotte, WG-R §4 (U) |
| `rock.shadow_angle_deg` | deg | 15-35 | 27.5 | Evans and Hungr abstract (V) |
| `rock.reach_max_m` / `fall_ell_m` / `sort_beta` | m / m / ratio | 20-400 / 2-100 / 0-1 | 150 / 15 / 0.4 | Guess |
| `rock.talus_parent_per_ha` / `talus_spread_deg` / `talus_child_mean` | 1/ha / deg / count | 0-500 / 0-60 / 0-40 | 40 / 20 / 4 | Guess (child mean added at E3b: the process needs it) |
| `rock.outcrop_parent_per_ha`, `_parent_min_m`, `cluster_sigma_m`, `child_mean`, `child_size_ratio` | | 0-50, 1-40, 2-30, 0-40, 0.05-1 | 2, 4, 6, 8, 0.4 | Guess (WG-R sigma 3-15) |
| `rock.field_cluster_sigma_m` / `field_child_mean` / `field_slope_full_deg` | m / count / deg | 0.5-20 / 1-40 / 0-45 | 2 / 5 / 15 | Guess (added at E3c: the clumps and the slope hat need them) |
| `rock.rock_k` / `tilt_jitter_deg` | ratio / deg | 0.5-1.2 / 0-30 | 0.9 / 15 | WG-R (U) |
| `rock.pack_model_share` | ratio | 0-1 | 0.5 | Guess; owner check WG-E3 (0 and 1 are the A/B arms) |
| row `size_min_m`, `size_max_m`; `sink_small`, `sink_large` | m; ratio | 0.2-40; 0-0.6 | talus 0.4-12, outcrop 1-20, field 0.3-2; 0.35, 0.15 | Guess |
| unit `field_stone_per_ha` / `rockfall` | 1/ha / ratio | 0-500 / 0-4 | 20 / 1 | Guess |

Removed with the v1 path (E2i.4): `clump_wavelength_m`, `clump_strength`, `occupancy_prior`, `occupancy_max`, `census_step_m`,
slope classes (`slope_class_deg`, `slope_classes`), bands (`band_base_m`, `band_ratio`, `bands`).

## Performance

- Bake: measured by WG-E2m. Only v1 is measured (WG-X5, at load): 200k 0.9 s, 1 M 1.4 s, 7.21 M 6.3 s. The halo
  multiplies candidates by `((256 + 2H)/256)^2` (D). Over a few minutes for the whole world fails the bake rule: the
  E2m ledger line says so before the owner look.
- Runtime: 0 ms from the stage (a cached file read at load). The counts' render cost is `big-world-rendering.md`'s.

## Tests

Per build row. Statistics on a flat fixture of at least 1 km2, one species, neutral fields; g(r) with border edge
correction; bands are sanity bands, not look limits. v1 tests (`tests_scatter.rs:104-205`) are ported, with halo tiles.
Rocks: a fixture with one straight 60 m scarp of one unit; size slopes fitted on log N(>s) above `s_min`.

## Verification (owner checks)

**WG-E2** after E2j on x10s (entry in the rows file): snug stands, clearings at roads and sites, big trees in the core,
understorey in gaps, open classes open. Yes/no only; amounts are live tunables.
**WG-E3** after E3f on x10s at placed scarps (rows file): size spread, big blocks at the toe, debris round blocks,
and whether the pack's rock models and the library's sit together (`rock.pack_model_share` live).

## Unverified dependences

All are `Guess` tunables; the spec leans on their form, not their numbers. (1) Thomas g(r) formula and sigma 5-20 m,
offspring 3-30. (2) Stand densities 300-700 /ha temperate, 1000-3000 boreal, and the class table built on them.
(3) SDI cap 700-1100 (unused; the cap is the renderer's 400). (4) Weibull shape, de Liocourt q. (5) Crown allometry
per species (one open-grown study). (6) Ripley K or g(r) of real stands (test bands). (7) Random sequential adsorption
near 55 % (cover after 3 rounds: measured by E2m). (8) Luby 1986 round structure (U); the `(2r - 1) R` radius is ours
(D), proven by the tile-alone test. Rocks: (9) talus 15-40 and scarp 45-60 deg. (10) shadow angle (abstract only).
(11) D 1.6-3 (one quote of 1.6). (12) sorting beta 0.3-0.7 (ours). (13) sink 10-40 %.

## Open questions (owner)

None open.
1. (Answered, owner via meta-6, 2026-11-26: yes.) Harvestable plants follow the same ecology: gathering means finding
   the habitat (clumps in their biome, at edges).
2. (Answered, owner via meta-6, 2026-11-26: yes.) Water plants (kelp, lily pads: 38,572 source references) come back
   once generated water exists (WG-W1), in a later slice, not E2.
3. (Answered, owner via meta-6, 2026-11-26: no, "Let's keep them for now".) The source's rock models (`terrain_rock*`,
   menhirs) stay in the scatter next to the rockgen library, as pack data ("Mesh pick"); the recommendation was
   library only. Looked at again at the WG-E3 check.

## Later

Water plants in water (after WG-W1). A coarse Nocca-style competition raster for emergent mixing. Ageing, dead wood,
fallen trees. Felled trees as an override list (`big-world-rendering.md` "Hooks").
