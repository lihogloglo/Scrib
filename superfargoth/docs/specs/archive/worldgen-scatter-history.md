# Worldgen scatter v2: history (`../worldgen-scatter.md`)

As-built notes of finished rows moved out of the spec (WG-E2s, 2026-11-27), oldest first: WG-E2a to WG-E2h, WG-E3a to
WG-E3d, WG-E2i, WG-E2j, WG-E2m, WG-E2n and WG-E2o (SPEC, REVIEW, .1, .2, .3, .m, .d). Current truth is the spec's Design;
later rows stay in its "As built".

## As built (WG-E2a to WG-E2o.d)

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
