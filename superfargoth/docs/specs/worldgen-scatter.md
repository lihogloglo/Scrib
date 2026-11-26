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

Specced in the next slice.

## Slices

One deliverable plus its test each. E2a-E2h need no climate product (neutral `Fields`). E2i and the owner look wait
for WG-E1c3 and WG-V1b. Order a to h, then i, j, m.

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
| WG-E2h | rank budget by group; v1 Bernoulli path deleted | Scrib `sf_worldgen` (`scatter.rs`) | E2g | budgets nest exactly; group shares; v1 tests ported | same + clippy dependents |
| WG-E2i | adapter: species TOML, learned priors, four groups, product `Fields` | Scrib `mw_worldgen` (`flora.rs`), `bevyrrowind/worldgen/scatter_species.toml` | E2h, WG-E1c3, WG-V1b | `tests_flora` tile alone; region prior exact | `test -p mw_worldgen` |
| WG-E2j | hash-cached product in the guide folder, read at load | Scrib `sf_worldgen` (`scatter_product.rs`), `mw_worldgen` | E2i | same hash twice; seed change rebuilds; read == computed | same |
| WG-E2m | measurement: stems/ha and cover per class, counts, seconds | Scrib `mw_worldgen` (test only) | E2j | `#[ignore = "measurement: scatter v2 counts and seconds"]` | ledger line |

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
| species hats (T, P, moisture, slope, altitude, water distance) | per driver | 4 values | open | Guess |

Removed in E2h: `clump_wavelength_m`, `clump_strength`, `occupancy_prior`, `occupancy_max`, slope classes, bands.

## Performance

- Bake: measured by WG-E2m. Only v1 is measured (WG-X5, at load): 200k 0.9 s, 1 M 1.4 s, 7.21 M 6.3 s. The halo
  multiplies candidates by `((256 + 2H)/256)^2` (D). Over a few minutes for the whole world fails the bake rule: the
  E2m ledger line says so before the owner look.
- Runtime: 0 ms from the stage (a cached file read at load). The counts' render cost is `big-world-rendering.md`'s.

## Tests

Per build row. Statistics on a flat fixture of at least 1 km2, one species, neutral fields; g(r) with border edge
correction; bands are sanity bands, not look limits. v1 tests (`tests_scatter.rs:104-205`) are ported, with halo tiles.

## Verification (owner checks)

**WG-E2** after E2j on x10s (entry in the rows file): snug stands, clearings at roads and sites, big trees in the core,
understorey in gaps, open classes open. Yes/no only; amounts are live tunables.

## Unverified dependences

All are `Guess` tunables; the spec leans on their form, not their numbers. (1) Thomas g(r) formula and sigma 5-20 m,
offspring 3-30. (2) Stand densities 300-700 /ha temperate, 1000-3000 boreal, and the class table built on them.
(3) SDI cap 700-1100 (unused; the cap is the renderer's 400). (4) Weibull shape, de Liocourt q. (5) Crown allometry
per species (one open-grown study). (6) Ripley K or g(r) of real stands (test bands). (7) Random sequential adsorption
near 55 % (cover after 3 rounds: measured by E2m). (8) Luby 1986 round structure (U); the `(2r - 1) R` radius is ours
(D), proven by the tile-alone test.

## Open questions (owner)

1. Do harvestable plants follow the same ecology, so gathering means finding the habitat (clumps in their biome, at
   edges)? Recommended: yes.
2. Do water plants (kelp, lily pads: 38,572 source references) come back once generated water exists (WG-W1)?
   Recommended: yes, in a later slice, not E2.

## Later

Water plants in water (after WG-W1). A coarse Nocca-style competition raster for emergent mixing. Ageing, dead wood,
fallen trees. Felled trees as an override list (`big-world-rendering.md` "Hooks").
