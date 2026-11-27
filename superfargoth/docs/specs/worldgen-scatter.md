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
| Density measured over a sample of the whole class | forest inventory (FIA ratio-of-means over mapped plots, U); UE PCG / Foliage Spawner: artist tunes on a view (U) | calibration on a hash-ordered tile sample (E2o.e) | match (inventory form) |
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
   regrowth fringe round every clearing. `D` keeps the suitability (E2o: clearings follow poor ground, the thresholds
   keep their meaning); it is the one place canopy suitability enters the intensity (Design 4).
4. **Candidates** (shared hashing). Truncated Thomas. Parents per parent tile: `n = floor(kappa_max A + u)` from
   `hash(seed, species, parent tile, j)`, thinned at the parent by `kappa / kappa_max` (Lewis-Shedler). Offspring:
   Poisson(`offspring_mean`) by inversion, Gaussian offset `cluster_sigma_m` by Box-Muller from the parent's stream,
   rejected beyond 3 sigma (exact reach). Plus a uniform background stream per tile at `background_fraction`, thinned
   by the same weight. **The weight enters once** (E2o; the standard inhomogeneous Neyman-Scott form: a covariate
   thins parents or offspring, never both, research `scatter-target-density-2026-11-27.md` Finding 2; E2n measured
   `E[su^2 D]` from three entries). Per layer:
   - **Canopy: from the class target.** `w_s(x) = D(x) x share_s(x) x p_s(x)`, with `share_s = su_s / sum_c su_c` over
     the canopy species (1 with one species; the species split the class target by suitability), `p_s` the region
     prior clamped to `[0, region_prior_max]`, and `w = 0` in a mosaic clearing. Intensity before competition, stems
     per ha: `lambda_s(x) = T(c) x w_s(x) / m(c) x g(c)`, `c = class(x)`, `T` = `class.<name>.stems_per_ha`, `m(c)` the
     class mean of `sum_s w_s` (the census below), `g(c)` the competition scale (the calibration below). Parent
     intensity `(1 - background_fraction) lambda_s / offspring_mean`, background `background_fraction x lambda_s`.
     A point with no class, or a class with `T = 0` or `m = 0`, gets no canopy (bake log line per class with land).
     The canopy rows' `parent_per_ha` is not read (lower layers only). **Class multiplier** `s(c) = g(c) / m(c)`,
     clamped to `intensity_scale_max` (REVIEW: `T / m` alone has no bound, `m -> 0` on a class of poor ground; a class
     whose `s` clamps gets fewer stems than `T`, bake warning `class <name>: weight mean m too low`). Lewis-Shedler
     bound, one per species for the whole world (exact for any class pattern, `D`, `share` <= 1):
     `kappa_max = max_c T(c) s(c) x region_prior_max x (1 - b) / offspring_mean`, background `b max_c T s x
     region_prior_max`. Cost (D: `T` 300, `s` 16, prior bound 3, `b` 0.2, offspring 12 gives 960 parent draws/ha,
     ~105 M on x10s's ~1,090 km2 before halo overlap, each one field sample and a hash; today's canopy bound is
     `0.8 x 4 x 3` = 9.6/ha, so the canopy's parent draws rise ~100x, most rejected outside the dense classes; E2o.m
     measures the bake against E2m's 57.8 s); a bound per species from the prior raster's max and one per parent tile
     from the classes it holds are the later optimisations if the bake leaves "a few minutes".
   - **Lower layers: per species.** `kappa = parent_per_ha x su x p` at the parent, plain `offspring_mean`, light and
     edge thinning as in Design 7. They have no class targets: their density is a cover under canopy light, the
     per-layer, artist-set density of every shipped tool (UE PCG per graph, HZD per layer; research Finding 1). The
     su-once form changes their counts (E2o.m measures them; their per-species values stay Guess).
   - **Class census** (bake, deterministic). A grid at `census_step_m` over the guide's bounds, at cell centres: the
     class, land only (not sea, lake, river or `Terrain::excluded`: a town or road cut neither counts as forest land
     nor densifies the woods round it); a mosaic clearing counts as land with `w = 0` (the target is "class mean with
     its mosaic"). Per class: area `A(c) = N(c) x census_step_m^2` and `m(c) = sum w / N(c)`, summed in row order
     (bitwise across thread counts). Thomas offspring land up to 3 sigma from the parent, so the class mean is kept
     inside a class and smeared over ~sigma at a class border (D).
   - **Calibration** of `g(c)` (bake; research Finding 3: no closed form covers clustered + random radii + size
     priority + k rounds, so the bake does the "artist iterates" loop of UE PCG and the Foliage Spawner). Per class
     with `T > 0`, a **sample of its tiles** (E2o.e): one world order of the census's parent tiles by
     `hash(seed, tile key)` (ties by tile key), shared by every class; class `c` takes, in that order, the tiles
     holding `c` land until their `c` land reaches `calib_area_ha` (all of them when the class has less). The rock
     layer (when present: canopy candidates avoid its footprints) and the canopy layer run there with the full halo
     (exact winners). `W` = winners at class-`c` points / `c` land on those tiles, per ha, with no rescale: the
     forest-inventory ratio-of-means over mapped plots (a plot straddling conditions counts each condition's trees
     against that condition's area: Zarnoch and Bechtold 2000, Bechtold and Patterson 2005, U), consistent for the
     class-wide winners / land because a hash order is a simple random sample of tiles. The census mean of `w` over
     those cells, `m_t`, is reported, not applied. Why (E2o.d, x10s): the purest tiles (the v1 choice, 31 tiles at
     100 % share for 203 ha) had `m_t` 0.12-0.96 against `m` 0.20-0.38, and `W` is not linear in weight (crowns pack),
     so the old `x m / m_t` misread the class: reached vs world woodland 35 vs 53, temperate forest 30 vs 65,
     rainforest 21 vs 62 per ha, while raw `W` equalled the world run on the same tiles exactly (count, halo and class
     attribution are not the gap). The inventory standard says the same: assigning a whole plot to its predominant
     condition shifted FIA volume by 3.9 % (Gartner and Reams 2005, U). The shared order (permanent random numbers,
     the coordinated-sample form of statistics offices, Ohlsson 1995, U) makes one class's sample a prefix of the
     same list as the others', so a measurement run covers the union of the active classes' samples = the prefix
     up to the farthest one, and a mixed tile serves every class it holds. First guess the Matern II
     closed form `g0 = -ln(1 - x) / x`, `x = T pi R^2 / 10^4`, `R = crown_k x 2 x (crown_a_m + crown_b x E[h])` over
     the class's species mix (`g0` = the upper end below when `x >= 0.9`), clamped like every iterate to `[1, intensity_scale_max x m(c)]` (`s` <= its bound; when that upper
     end is below 1, `g` is it and the class carries the low-weight warning); then `g := clamp(g x T / W, ...)`
     (fixed point: `W(g) / g` falls as intensity rises, so a step from below undershoots and one from above stays
     above, monotone either way; E2o.2: from the second step on the secant in log-log, `g x (T / W)^(1/b)`, `b` the
     slope of `ln W` over `ln g` through the last two measurements clamped to `[calib_saturation, 1]`, since `W` flattens
     as crowns pack and the plain fixed point crawled, 601 -> 465 per ha in 4 steps for T 400), at most `calib_iters` steps, stop when `|W - T| <= calib_tol x T`; `W = 0` sets `g`
     to the upper end. **Ceiling:** a raise of `g` by a fraction f that raises `W` by less than
     `calib_saturation x f`, or `g` at the upper end with `W < (1 - calib_tol) T`, means the target is not reachable
     (packed crowns: above what the class's sizes allow, research Finding 4; or the multiplier clamp); stop, keep
     the iterate with the largest `W` (ties: the smaller `g`), bake warning `class <name>: target T above the crown ceiling, reached W`. Same inputs, same `g`.
     **Joint check** (E2o.e): a class's `W` on mixed tiles depends on its neighbours' `g` (competition and
     offspring cross class borders; E2o.d: grassland, 130 tiles at 20 % share, converged at 5.1 while its
     neighbours' `g` still moved, 6.6 once they settled). When no class is active, one run over the union of every
     calibrated class's sample at every class's kept `g`; each class's `W` from it is pushed as a measurement at its
     kept `g`. A converged class now outside `calib_tol` is reopened: it keeps its history and step count and steps
     from that measurement (secant as above); ceiling and out-of-steps classes are not reopened. Repeat until a joint
     check reopens nothing, at most `calib_iters` joint checks. The file reports, per class, the last joint check's
     `g` and `W` (one consistent snapshot: `reached` is what the world run gives on those tiles at those `g`), and the
     ceiling flag of the last decision. **Cost** (D): per class about `calib_area_ha / (A(c) / N(c))` tiles, `N(c)`
     the parent tiles holding `c` (mixed tiles: more tiles per 200 ha, grassland at 20 % share needed 130 where the
     pure classes needed ~31 at x10s's ~6.5 ha tiles); a run covers the prefix up to the farthest active class, at
     most the whole land (= one world canopy pass) when a class is rare; at most `calib_iters + 1` runs per class
     plus the joint checks. E2o.f prints tiles per class and the calibration seconds on x10s; if they stop fitting
     the bake's "a few minutes", the lever is `calib_area_ha` (data). Sampling error (D): ratio-of-means CV ~ the
     between-tile CV of winners/ha over `sqrt(n)` tiles, so more mixed tiles at the same area lower it; E2o.f compares
     `reached` with the world over census land per class.
   - **Result file.** `A`, `m`, `g`, `W`, the sample's tiles, land and `m_t`, steps, joint checks and the ceiling flag per class go to `<guide>/scatter/calibration.toml`
     beside the product (diagnostics and tests read it). They are functions of hashed inputs (the species file with
     its class targets, values, seed, fields), so the product hash needs no extra part; the six new bake tunables
     (`census_step_m`, `calib_*`, `intensity_scale_max`) are species-file declared values, hashed with the others
     (`flora.rs` `product_inputs` hashes the whole `SpeciesFile`, V). Load-only values stay out of that file (Design 10).
5. **Size mark** (shared form). Per candidate, before competition: reverse-J (truncated exponential in height,
   `size_min_m..size_max_m`) or Weibull `size_shape`, by inversion; in the edge band the quantile is scaled by
   `edge_size_ratio` (big trees in the core). Crown radius `r = crown_a_m + crown_b h` (crown width linear in DBH for
   open-grown trees, Brack and Wood, snippet V). Canopy in a class with `class.<name>.size_max_m` set draws its
   size up to `min(size_max_m, class cap)` (the ceiling rule, "Counts per biome class").
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
    re-competition). Groups: canopy, understorey, ground, rocks (WG-E3). All at load, none in the product hash: the
    load values are declared beside `scatter.budget` in `scatter.rs`'s v1 `Params` (not hashed, V `product_inputs`)
    and set by `--set` the way the app reads `scatter.budget` (`world.rs:96-100`). `canopy_stems_per_ha_max` is today
    a species-file value (`scatter_species.rs:104`, hashed, never read, E2n): E2q moves it to `Params` (REVIEW).
    - **Stand cap** (E2q, applied first, inside the full radius too): per product tile (`tile_m` 256 m, 6.55 ha, by
      position), canopy instances above `round(canopy_stems_per_ha_max x tile area)` are dropped in rank order
      (largest `u` first: a uniform thinning, sizes kept unbiased; the survivor set is budget-independent, so the
      budgets below still nest). It is the render cost lever of `big-world-rendering.md`'s "stand" case and stays a
      live tunable (owner 2026-11-26); the bake warns for a class target above it. Dense classes' inner stands sit
      above their class mean (`lambda` scales with `w / m`), so the cap can cut a 250-300/ha class under its target;
      E2o.m reports winners/ha before the cap (the product) and the cap's cut apart.
    - **Full radius** (E2p, a named scaffold): with a focus point (the game's start position: the app's eye, `world.rs:89`, known before
      the content load; scatter plane `(x, y) = (eye.x, -eye.z)`, the inverse of `flora::placements`' map, V) and
      `full_radius_m > 0`, every kept instance within `full_radius_m` of the focus is placed whatever its rank; the rest share one global remainder
      `max(0, budget - sum_g inside_g)`, split by `budget_cut_layers`' group shares of the outside set (REVIEW: a
      per-group `budget_g - inside_g` lets the total pass the budget whenever one group's inside exceeds its share,
      e.g. canopy: 5 % of E2m's kept set is a 10 k share against ~25 k inside). Total = `max(budget, inside)`
      capped by the set; budgets nest (fixed focus, the remainder grows with the budget). No focus or radius 0 =
      the whole-world cut above. Industry: UE5 PCG runtime generation (full generation of grid cells within a generation radius of a source, U), Decima's tiles round the player (U),
      research Finding 5. One radius for every group now; a radius per group (UE: per grid size) comes with the
      removal path, `big-world-rendering.md` BW-3's `InstanceSource` per ring tile round the moving camera. Budget
      arithmetic (D, E2m and E2n rates on x10s): 1 km round the focus holds ~25 k canopy at the 81/ha mean (~78 k
      in a 250/ha class), ~18 k understorey, ~32 k ground: inside the 200 k budget. The lower-layer rates are E2m's
      `su^2` counts; with `su` once they rise by `E[su] / E[su^2]` (E2o.m measures; at ~2x, a 250/ha class's ~178 k
      inside still fits, a denser start does not and the total becomes `inside`). The load logs `inside_g` per group.

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
seasonal forest 150; temperate rainforest 250; rainforest 300 (per km2: x 100). They are winners after competition,
met by Design 4's canopy form plus calibration (E2o; before it they were parsed and never read, E2n). Inner stands
are capped at `canopy_stems_per_ha_max` 400 per tile in rank order at load (Design 10), the "stand" case of
`big-world-rendering.md` "Cost model" (V). BW-2 measured that case over budget (13.97 ms GPU, "Tripped"): the cap,
`full_radius_m` and `scatter.budget` are the levers.
**Ceiling.** Crowns bound the density: with today's canopy row (`crown_a_m` 0.75, `crown_b` 0.2, `crown_k` 0.8) two
trees of height h conflict at `R = 1.2 + 0.32 h` m, a Matern II ceiling `1/(pi R^2)` of 164/ha at 10 m and 55/ha at
20 m, at most ~2.2x that after many rounds (RSA jamming 0.547 over Matern II's 0.25 packing; 3 rounds reach less;
research Finding 4, D). Dense stands are stands of small
trees (Reineke's self-thinning rule `N = k d^-1.605`, U). Rule: the calibration detects a target above the ceiling
and reports it (Design 4); the fix is data, an optional `class.<name>.size_max_m` (m, 1-40, unset = the species'
size range) that caps the size mark of canopy in that class, set by the adapter only for a class E2o.m reports off by
more than 25 %. The framework never shrinks trees by itself.

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
for WG-E1c3 and WG-V1b. Order a to h, then i, j, m; then the density fix o.1, o.2, o.3, o.m, then p, q (E2n diagnosis, E2o research). Rocks: E3a-E3d after E2e (they reuse E2d-E2e), E3e after E2i.

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
| WG-E2o.1 | Design 4 weight-once form: canopy `lambda = T w / m x g` from a `CanopyTargets` input (per class id: `T`, `m`, `g`), `s = g / m` clamp, world `kappa_max`; lower layers `su` once; the 6 new species-file declared values | Scrib `sf_worldgen` (`scatter.rs`, `scatter_species.rs`) | E2n | neutral fields, two classes side by side (T 100 and 400, `m` from a fine grid mean in the test, `g` 1), 4 km2 each: candidates/ha of each class's land within 3 Thomas sd of `T`, sd `= sqrt(T A (1 + offspring_mean)) / A` (3 sd ~5.4 % at T 100; REVIEW: 5 % on 1 km2 at T 50 was ~1 sd); a class with `m` below `1 / intensity_scale_max` gets `T m s_max` and its flag; halving `su` halves the candidates (not quarters) of a lower-layer species, and of canopy with `m` held and no mosaic clearings; tile alone bitwise | `test -p sf_worldgen` + clippy dependents |
| WG-E2o.2 | class census + calibration (Design 4), `calibration.toml` write/read | Scrib `sf_worldgen` (new `scatter_calib.rs`, `tests_scatter_calib.rs`) | E2o.1 | census: two-class fixture, `A` exact, `m` == brute mean, same on 1 and N threads; calibration: a target below the ceiling reaches `W` within `calib_tol`; a target above it is flagged within `calib_iters`; same inputs same `g` | same |
| WG-E2o.3 | adapter wiring: `mw_worldgen::flora` runs census + calibration before the passes, class names to ids, warnings to the bake log, `calibration.toml` beside the product | Scrib `mw_worldgen` (`flora.rs`, `tests_flora.rs`) | E2o.2, E2j | fixture guide: class means exact; a target change makes the product stale; a read product has its calibration file | `test -p mw_worldgen` + clippy `bevyrrowind` |
| WG-E2o.m | measurement: winners/ha per class vs target, lower-layer counts, bake s, product size | `mw_worldgen` (test only) | E2o.3 | re-run `canopy_density_trace_on_x10s`, `scatter_v2_counts_and_seconds_on_x10s` | ledger line; a class off > 25 % sets its `size_max_m` row or parks the look |
| WG-E2o.d | diagnosis: calibration `reached` vs the world at the same `g`, term by term | `mw_worldgen` (test only) | E2o.m | `canopy_density_trace_on_x10s` gap table | ledger line naming the cause and the fix row |
| WG-E2o.e | spec: calibration tiles = a hash-ordered sample of the class's tiles (one world order), `W` = class winners / class land with no rescale (`m_t` reported), joint check at the final `g` reopening converged classes out of tol (Design 4) | docs (`worldgen-scatter.md`) | E2o.d | n/a | ledger line |
| WG-E2o.f | build E2o.e: `calib_tiles` sample, no `bias`, joint check loop, `m_t` + tiles + joint checks in `calibration.toml` | Scrib `sf_worldgen` (`scatter_calib.rs`, `tests_scatter_calib.rs`) | E2o.e | two-class fixture whose purest tiles differ in weight from the class mean, class land above `calib_area_ha`: `reached` within `calib_tol` of the class's winners/ha over its whole land at the kept `g` (fails with the purest-tile choice); a class converging first on mixed tiles is re-measured and reopened; same on 1 and N threads; same inputs same `g`; then `canopy_density_trace_on_x10s`: reached vs world (census land) per class, tiles per class, calibration s | `test -p sf_worldgen -p mw_worldgen` + clippy dependents; ledger line |
| WG-E2o.g | spec: calibration sample size from the sampling error (E2o.f: tundra 0.88, rainforest 0.91, taiga 1.14 of the world on 37-40 pure tiles): the inventory sample-size rule `n = (z CV / E)^2` (Cochran 1977, U) on the measured between-tile CV of the first run, the sample extended along the same order (prefix kept), its tunables and cost on x10s | docs (`worldgen-scatter.md` Design 4, Tunables) | E2o.f | n/a | ledger line |
| WG-E2o.h | build E2o.g | Scrib `sf_worldgen` (`scatter_calib.rs`, `tests_scatter_calib.rs`) | E2o.g | per E2o.g; then `canopy_density_trace_on_x10s`: reached / world within `calib_tol` for every class not flagged, calibration s | `test -p sf_worldgen -p mw_worldgen` + clippy dependents; ledger line |
| WG-E2p | full radius round a focus at load (Design 10), one global remainder budget | Scrib `sf_worldgen` (`scatter.rs` `Params` + `budget_cut_layers` remainder form), `mw_worldgen` (`flora.rs` `place`, `extra`), app `world.rs` (pass the eye of `:89` to the content fn and on at `:291`; `--set scatter.full_radius_m` read like `scatter.budget`) | E2o.3 | inside the radius == unbudgeted set; total == `max(budget, inside)` capped by the set; outside nests with the budget; no focus == today's cut | `test -p mw_worldgen` + clippy `bevyrrowind` |
| WG-E2q | stand cap per tile at load (`canopy_stems_per_ha_max`, Design 10), before the radius and budget | Scrib `sf_worldgen` (value moved from `scatter_species.rs` to `scatter.rs` `Params`: out of the product hash), `mw_worldgen` (`flora.rs` `place`), app `world.rs` (`--set` like `scatter.budget`) | E2p | no tile above the cap, inside the radius too; the cap's survivors are the lowest ranks; cap 2000 == uncapped on a 400/ha fixture; a cap change reads the product (no rebuild) | same |
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
- **WG-E2j.1** (2026-11-27, `scatter_product.rs`): `InputsHash` (blake3 of named, length-prefixed parts under a
  format tag: `guide`, `climate`, `seed`, `value`, `bytes`; the budget is not an input, the cut stays at load) and
  `write` / `read` / `cached` of `<guide>/scatter/` (`plants.bin` 64-byte and `rocks.bin` 72-byte f64 records in output
  order, `meta.json` with inputs hash, counts, per-file + product blake3; `cached` = None when absent or stale). 2 tests.
- **WG-E2j.2** (2026-11-27, `mw_worldgen` `flora.rs`): `extra` (the `view world:` load) goes through `scatter_cached`:
  `product_inputs` feeds `PRODUCT_VERSION`, the guide, the product `GridFields` (`InputsHash::fields`), seed, `tile_m`,
  water margin, tile filter and settings, the learned file, slots and prior, the mosaic mask and region grid, clearing
  values, `TownMask::digest` and the bytes of `PRODUCT_FILES`; a match reads, else the passes run (`winners`) and write.
  Then `place` cuts the budget and poses. Written at load, not by a separate command (UE's derived-data-cache pattern):
  the game's inputs (bake-store model bounds, the guide's towns) exist only there, so another command would hash
  differently. `worldgen-flora` (census) runs without a product. 1 test (write, read, budget read, seed / file stale, damaged rebuilt).
- **WG-E2m** (2026-11-27, `mw_worldgen` `tests_flora.rs`, ignored): x10s copied to a temp folder, climate made there
  (2.5 s), no towns or clearings. Release: passes + write 57.8 s (18.47 M kept before the budget: 0.93 M canopy, 6.36 M
  understorey, 11.19 M ground; the product ~1.2 GB at 64 B a plant), product read + budget + pose 4.4 s. Canopy stems/ha
  4.8-10.8 in every wooded class against declared 60-250 (taiga 4.8 / 250, rainforest 8.5 / 250), crown cover 3-7 %.
- **WG-E2n** (2026-11-27, diagnosis, `tests_flora::canopy_density_trace_on_x10s`, ignored; 1/7 of the x10s tiles): the
  candidates match Design 4 within 1-4 % (mean field `parent_per_ha x offspring_mean x E[su^2 D prior]`), so the loss is
  the formula: `class.<name>.stems_per_ha` and `canopy_stems_per_ha_max` are parsed but never read; the intensity is
  per species (48/ha ceiling) x `su` three times (`D` = noise x best canopy `su`, Design 3; thinning; offspring mean) x
  noise (mean 0.5) x the clearing cut (35-68 % of land), so shrubland (target 10) gets 11.4/ha and taiga (250) 4.3.
  Competition then keeps 63-73 %; the learned canopy size range is 1-40 m (the column's max), crowns up to 8.75 m.
  The targets ask 8.89 M canopy on x10s (81/ha); the world-total budget (200 k) cuts it evenly to 0.2 %. Fix: WG-E2o.
- **WG-E2o SPEC** (2026-11-27, docs): Design 3, 4, 10, Counts "Ceiling", Tunables, build rows E2o.1-E2q. `D` keeps
  `su` and the weight enters once; canopy from the class target over a census mean, the competition scale by bake
  calibration with a ceiling flag; at load a stand cap per tile and a full radius round the start position.
- **WG-E2o REVIEW** (2026-11-27, docs, against `scatter.rs:443-555`, `scatter_stand.rs`, `flora.rs` `place`/
  `product_inputs`/`placements`, `world.rs:89-100`): 6 fixes. The bound clamps `s = g / m` (`T / m` was unbounded;
  `intensity_scale_max` 16); calibration includes rocks and rescales `W` by `m / m_t` (core-tile bias), `W = 0` and
  the clamp are ceilings; load values go to v1 `Params` (the species file is hashed whole, so the stand cap there
  rebuilt the product); the full radius uses one global remainder (per-group overshot the budget); E2o.1's 5 % test
  was ~1 sd. Checked true: Matern II `g0`, ceilings 164/55 per ha, RSA 2.19x, 480-draw arithmetic, E2m/E2n rates.
- **WG-E2o.1** (2026-11-27, `scatter.rs` `CanopyTargets`, `species_candidates`; `scatter_species.rs`): canopy per
  Design 4 (`T(c) s(c) D share prior`, one acceptance at the parent and at a background point, offspring Poisson
  plain), lower layers `su` once; `census_step_m`, `calib_*`, `intensity_scale_max` declared. `flora` passes
  `CanopyTargets::uncalibrated` (`m` = `g` = 1) until E2o.3. Test `canopy_candidates_meet_the_class_target_and_the_weight_enters_once`.
- **WG-E2o.2** (2026-11-27, new `scatter_calib.rs`; `scatter.rs` `square_canopy`, `canopy_weight`): `census` (row
  sums in order, per class and per parent tile) and `calibrate` (classes iterate together, one canopy run per step
  over the active classes' purest tiles; fixed point then log-log secant), `Calibration::write`/`read`
  `<dir>/scatter/calibration.toml`, warnings returned. Fixture: T 100 met at the first guess, T 400 in 4 steps, 2000 flagged.
- **WG-E2o.3** (2026-11-27, `flora.rs` `calibration`, `with_rules`; `winners`/`scatter_tiles` take the targets): the
  adapter runs census (over `World::rect`) + calibration before the passes; class ids = the fields' class order;
  `scatter_cached` writes `calibration.toml` before the product and reads it back with a read product
  (`FloraRun::calibration`); warnings go to the `worldgen flora:` log and the census table. `canopy_weight` is pub.
- **WG-E2o.m** (2026-11-27, measurement, x10s, release): world canopy/ha vs T: tundra 5.3/5, grassland 6.6/5,
  shrubland 12.0/10, woodland 52.9/60, temperate forest 64.3/150, rainforest 61.9/250, taiga 47.1/250 (the last four
  flagged at the crown ceiling). Off > 25 %: taiga, temperate forest, rainforest (under: `size_max_m` rows) and grassland
  (over). Calibration `reached` sits under the world count in every class (woodland 35.3 vs 52.9). Bake 262 s (was 58).
- **WG-E2o.d** (2026-11-27, diagnosis, x10s): the calibration counts exactly (raw `W` == the passes on its tiles
  built alone == the world run on them, every class): no halo, attribution or pass gap. The gap is the tile choice:
  the purest tiles' `m_t` is far from `m` (0.12-0.96 vs 0.20-0.38) and `W` is not linear in weight, so `x m / m_t`
  misreads the class (rainforest 21 vs world 62, woodland 35 vs 53, shrubland 9.9 vs 12.1; grassland, 20 % share, 5.1 vs
  6.6: converged before its neighbours' `g` settled). Fix: E2o.e/E2o.f (sampled tiles, no rescale, final joint check).
- **WG-E2o.e** (2026-11-27, spec): Design 4 calibration samples its tiles (one world hash order shared by every
  class, each class takes the tiles holding its land until `calib_area_ha`), `W` = class winners / class land with no
  `m / m_t` (FIA ratio-of-means form, `m_t` reported), and a joint check at the final `g` of all classes reopens a
  converged class out of tol (at most `calib_iters` checks). Cost and sampling error stated (D); E2o.f builds it.
- **WG-E2o.f** (2026-11-27, `scatter_calib.rs` `tile_order`, `calib_tiles` -> `Sample`, `drive`; product `FORMAT` 2):
  as E2o.e; a converged class's joint measurement is pushed, ceiling and out-of-steps classes keep their `g`; the last
  check reopens nothing; the file adds `calib_tiles`, `calib_weight_mean`, `joint_checks`, `reopened`. x10s: 14.1 s, 2
  checks (grassland reopened); reached / world 0.99-1.00 on mixed samples, 0.88 (tundra), 0.91 and 1.14 on ~38 tiles.

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
| `canopy_stems_per_ha_max` | 1/ha | 50-2000 | 400 | big-world-rendering.md "stand" (V); load only, moves to v1 `Params` at E2q (today a hashed species-file value) |
| `region_prior_max` | ratio | 1-10 | 3 | Guess |
| `class.<name>.stems_per_ha` | 1/ha | per row, 0-50 (desert, tundra, grassland) to 0-1000 (forests) | table above | Guess (`scatter_species.toml` `[classes]`) |
| `class.<name>.size_max_m` | m | 1-40 | unset | Reineke (U); set only after E2o.m (Counts "Ceiling") |
| `census_step_m` | m | 4-128 | 16 | Guess (~15 samples per 250 m stand wavelength) |
| `calib_area_ha` | ha | 10-2000 | 200 | Guess (~2000 winners at 10/ha: ~8 % clustered sampling error, D); class land of the hash-ordered tile sample (E2o.e) |
| `calib_iters` / `calib_tol` / `calib_saturation` | count / ratio / ratio | 1-8 / 0.01-0.25 / 0-1 | 4 / 0.05 / 0.25 | research Finding 3 (2-4 steps, D); tol and saturation Guess; `calib_iters` also bounds the joint checks (E2o.e) |
| `intensity_scale_max` | ratio | 1-40 | 16 | Guess: bound of the class multiplier `s = g / m` (Design 4; E2n: `m` ~0.3, `g` 2-3 for dense classes gives `s` ~10); candidates reach `s x region_prior_max x T` locally and the parent draws scale with it |
| `full_radius_m` | m | 0-8000 | 1000 | Guess (UE PCG generation radius, U; Design 10 arithmetic); 0 = off. Load only: v1 `Params` beside `budget`, not hashed |
| `tile_m`, `seed`, `water_margin_m`, `budget`, `size_priority` | | | v1 | kept (`scatter.rs:49-73`) |
| species `parent_per_ha` | 1/ha | 0-500 | 4 | Guess (lower layers only since E2o; canopy follows the class target) |
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

Removed with the v1 path (E2i.4; `census_step_m` returns at E2o with a new meaning, the class census): `clump_wavelength_m`, `clump_strength`, `occupancy_prior`, `occupancy_max`, `census_step_m`,
slope classes (`slope_class_deg`, `slope_classes`), bands (`band_base_m`, `band_ratio`, `bands`).

## Performance

- Bake (WG-E2m, x10s, release, all cores): 57.8 s for the whole world, inside the bake rule; the load from the product
  4.4 s at the 200k budget (v1 at load was 0.9 s), 63.6 s with no budget (pose of 18.5 M dominates). The product is
  ~1.2 GB (64 B per kept plant).
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
