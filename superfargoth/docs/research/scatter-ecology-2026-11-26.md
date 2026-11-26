# Scatter v2: ecology simulation vs rules and point processes (WG-R, 2026-11-26)

Research for specs-6, slices WG-E2 (trees, flora) and WG-E3 (rocks). No code. Marks: **V** = primary source read in
full text by me (PDF or slides parsed locally); **U** = abstract, snippet, summary or my background knowledge, not
checked against the primary text; **D** = derived by me (arithmetic or a design proposal of ours, not a published result).

## 1 Summary and recommendation

**Hybrid, leaning on rules: marked point processes plus a local competition pass, not an ecosystem simulation.**
Canopy trees = a clustered parent process (Thomas or Matern: parents are stands and seed clumps), a size mark per tree
from a reverse-J or Weibull law, then a fixed-radius, hash-priority competition (Matern II style, a few rounds, the bigger
tree wins, radius k x (crown_i + crown_j)). Understorey = a second pass conditioned on the light the canopy leaves. Rocks = a
talus pass conditioned on a distance-to-scarp raster, power-law sizes. Species suitability comes from the climate rasters
(WG-E1), by Nocca's fitness `min` or Makowski's Gaussians.

Why. (a) The three signatures of a real forest (clumping at stand scale, regularity at crown scale, reverse-J sizes) come out of
cluster plus competition at seconds of cost. (b) Time-stepped simulation (Cordonnier 2017, Makowski 2019) keeps global state over decades:
it breaks the tile-alone rule and costs hours (Makowski: 500K plants interactive on one GPU, V). (c) Shipped games use rules and density
maps; Horizon gets "snug, not overlapping" from a dither pattern whose spacing is the asset footprint (V). (d) Our v1 is a weakly
modulated Poisson process with no interaction, hence spray (section 7). The one published method that fits a bake of minutes is
Nocca 2021 (V), a non-simulated steady-state competition on density maps: take it for suitability and species proportions only.

## 2 Sources

| Source | Mark | Gives |
|---|---|---|
| Makowski et al. 2019, Synthetic Silviculture (author PDF) | V | Individual-plant growth, seeding in a Gaussian disc around mature plants, shade tolerance `stol`, shadow grid (bottom cells about 2 m), climate adaptation as a 2D Gaussian in (T, P), succession and gap dynamics, 500K plants interactive (GTX 1050), 3/2 self-thinning check above 5 cm diameter. |
| van Muijden, Horizon Zero Dawn, GDC 2017 (all slides) | V | World data = 2D maps (0.5 to 2 m, about 4 MB/km2, generated and paintable). Per layer: DENSITYMAP, GENERATE (dither pattern: "even spread thresholds, maximize 2D distance", scaled to footprint, "responsible for collision"), PLACEMENT. Painted density decoded into Sparse, Edge, Inner forest; Forest inverted gives Clearing. 500+ asset types, 100,000+ objects, about 250 us GPU. Goals: deterministic, locally stable. |
| Ecormier-Nocca et al. 2021, Authoring Consistent Landscapes with Flora and Fauna (Purdue PDF) | V | Not a simulation. Inputs: DEM, minerals, orientation, latitude, altitude, sea-level temperature extremes; derived yearly moisture, temperature, sunlight maps. `fit(s,R) = min over resources r of overlap([Fmin,Fmax], [r_min,r_max]) / (r_max - r_min)`, in [0,1]. Greedy: add one population unit at a time to the best-fit region while resources remain (with or without target species proportions). Output: density per species per cell. 16 km2 at 1 m cells, Python: 15 min total, about 5 min vegetation. Matches expert flora maps 24.8 to 69.5 % IoU. |
| Cordonnier et al. 2017, Ecosystem and Terrain Erosion | **U, full text NOT obtained** | Layered grid (bedrock, rock, sand, humus), live and dead vegetation as density per type, water, two-way coupling with erosion, centuries, 10 km x 10 km at 10 m (thesis summary), simulations "hours" (a later paper). Nocca (V) says its comparison run used three species. The 30 MB PDF came at 10 to 60 KB/s from every mirror and HAL blocks fetch tools. |
| Gain 2017 EcoBrush; Kapp 2020 | U (abstracts) | EcoBrush: biome databases indexed by T, rain, sun, slope built by simulation, brushes, 5 x 5 km2. Kapp: ML canopy height model, canopy trees fitted by a constrained iteration, understorey from simulations. |
| Epic Procedural Foliage and Open World property reference (fetch tool summaries); PCG Biome Core | U | Seed steps: Num Steps, Initial Seed Density (per 10 m, squared), Average Spread Distance/Variance, Seeds Per Step, Max Age, Collision Radius, Shade Radius, Overlap Priority (lower removed), Scale Curve. PCG: Density filter, Bounds Modifier + Self Pruning (tutorial). |
| Wildlands (80.lv), Far Cry 5 (PlayStation blog) | U | Wildlands: rules per material splat, "cascade" big then medium then small, delete thresholds between rules. FC5: Houdini rules, species with growing conditions competing for ground; GDC slides behind a login. |
| Forestry and statistics (Yoda, Reineke, de Liocourt, Weibull, Thomas/Matern, Illian et al.) | U | Section 3 formulas; the Thomas g(r) and Matern II deletion rule were confirmed in spatstat and arXiv hits only. |
| Rocks (Turcotte 1986, Ord 2021, Kirkby and Statham 1975, Evans and Hungr 1993, Wegner 2021) | U | Section 4. |
| crates.io API, 2026-11-26 | V | Section 6. |
| Our code and ledger | V | Section 7. |

**Not found:** any public material on a "Chlorophyll" engine; Far Cry 5 and Wildlands slides; a per-hectare tree density from
Valheim, Elden Ring or Skyrim (only: Elden Ring about 80 % of vegetation placement procedural, Edge interview, U; Skyrim 46,430 trees in
total, fan count, U).

**Fit to a minutes-long, tile-deterministic bake.** Fits: Nocca's local fitness and competition (pure function of rasters, no time).
Its cost scales with area: 5 min per 16 km2 in Python is about 13 h for 2500 km2 (D, x156), so it needs a Rust port at 4 to 8 m cells. Does not fit:
Makowski and Cordonnier time stepping (global state, no tile-alone). Their ideas do: Gaussian climate adaptation, seeding disc, `stol`, succession.

## 3 The technique, stated precisely

Skeleton shared by Horizon, Wildlands, FC5 and UE5 PCG: **maps in, density out, a discretiser that enforces spacing, then poses**. We add the
statistics a forest needs. Numbers are typical values to declare as tunables, not verified constants.

**Pass 0, rasters** (per tile plus halo): height, slope, aspect, curvature, wetness index, T, P, region, exclusion, distance to scarp.
T falls linearly with height, `T(h) = T(0) - gamma h`, gamma about 6.5 K/km (form V in Makowski, value U).

**Pass 1, suitability.** Per species, per driver a response; combine by Liebig minimum (Nocca, V) or product. Makowski (V):
`o = N_T(T) N_P(P) / (N_T(T_A) N_P(P_A))`, 1 at the optimum. Add trapezoids for slope, wetness, altitude; above the treeline trees survive only
in sheltered clumps (Makowski citing Buckley 2016, V). Suitability scales parent intensity and offspring count; mixing follows overlapping responses.

**Pass 2, stand mosaic.** A low-frequency field D(x) at 100 to 500 m, plus wetness and suitability, decoded by thresholds into clearing, sparse, edge,
inner forest as Horizon does (V). Edge = band around the threshold contour (distance transform). Forced clearings come from exclusion rasters
(roads, sites). This replaces our 48 m multiplicative noise.

**Pass 3, canopy = marked Neyman-Scott process plus competition.**
1. Parents: Poisson, intensity `kappa(x) = kappa0 x suitability x D(x)`.
2. Offspring (Thomas): Poisson(mu) per parent, isotropic Gaussian offset sigma_c. `g(r) = 1 + exp(-r^2/(4 sigma_c^2)) / (4 pi kappa sigma_c^2)` (U).
   Typical sigma_c 5 to 20 m, mu 3 to 30; plus a 10 to 30 % uniform background. Matern cluster: uniform disc instead.
3. Size mark, drawn before competition. Even-aged: Weibull shape c about 2 to 4. Natural stand: reverse-J, de Liocourt quotient q about 1.2 to 2 per
   5 cm class, i.e. `f(d) = lambda exp(-lambda (d - d_min))`; Weibull with c <= 1.5 is also reverse-J (all U).
4. Competition (zone of influence): conflict if `dist < R_ij = k (r_i + r_j)`, r = crown radius from allometry, k 0.6 to 1.0. **Bigger crown wins,
   ties by hash.** Matern II deletes a point with a smaller-mark neighbour within R: survivors `lambda_II = (1 - exp(-lambda pi R^2)) / (pi R^2)`;
   type I `lambda_I = lambda exp(-lambda pi R^2)` (U). UE's Overlap Priority and Wildlands' delete thresholds are the same idea (U). One round covers at most about
   25 % of the area (D); to get a closed canopy run **k rounds** (round r re-tests losers against round-(r-1) winners only), a bounded dart throwing
   (random sequential adsorption saturates near 55 % for equal discs, U). k = 3 should reach 40 to 45 % cover (D, measure).
5. Density as an output. Closed canopy stems per ha `N = 10,000 f / (pi r_c^2)` (D): crown radius 2 m about 800 f, 3 m 350 f, 5 m 125 f, 8 m 50 f, f = cover 0.4 to 0.9.
   Real stands: mature temperate roughly 300 to 700 stems/ha, boreal 1000 to 3000 (U, general forestry; a forest table I saw had means 628 and 1595, type unstated).
   Reineke: `N_max = SDI (D/25 cm)^-1.605` per ha, SDI cap 700 to 1100 (U); Yoda -3/2 in mass vs density (U), reproduced by Makowski above 5 cm (V). **The spec states
   density as stems/ha at closed canopy and lets competition reach it**, not a per-m2 count to replay.

**Pass 4, understorey by light.** `L(x) = exp(-sum_i a_i(x))`, a_i = crown shading disc of radius r_i (a 0.5 to 1.5). Makowski's `Q_eff = lerp(stol, 1, Q * Q_G)` is the
same idea (V). Density `kappa_u x suitability x response(L; stol)`; saplings cluster in gaps (V). Competition inside the layer at a smaller radius; across layers
only against trunk footprints (D). Horizon uses separate layers for trees, bushes, undergrowth (V); Wildlands a cascade (U).

**Pass 5, pose.** Yaw uniform; scale from the size mark (not uniform in a range); tilt toward the normal per species; ground height read at the exact position plus root offset.

**What must be visible:** denser, shrubbier edges; clearings with regrowth fringe; big trees in the core, small at edges and gaps; 2 to 3 layers; one or two dominant
species with companions; mixing along moisture, altitude, aspect.

## 4 Rocks (WG-E3)

Rocks come from sources (scarps, outcrops, deposits) and obey transport. Sources are all U.
1. **Talus mask.** Scarp = slope above 45 to 60 degrees. Talus lies below it on 15 to 40 degrees (repose of loose rock 30 to 38). Reach is bounded by the shadow angle, about 22 to 28
   degrees below horizontal from the scarp top (Evans and Hungr). Raster: distance to scarp and drop height. Gives "rocks at scarp bases", which slope-class occupancy cannot.
2. **Sizes.** `N(>s) ~ s^-D`, D about 1.6 to 3 (Turcotte 1986; Ord et al. 2021 say gamma, extreme-value or log-normal often fit better). Truncated Pareto sampler (D):
   `s = s_min (1 - u (1 - (s_min/s_max)^D))^(-1/D)`. Map s to mesh class plus scale and a non-uniform stretch 0.7 to 1.4.
3. **Fall sorting.** Large blocks travel further, fines stay near the apex (Evans and Hungr, Statham 1976); a 2021 field study found it absent on some cones. Model as a tendency:
   probability at distance x about `exp(-x/ell(s))`, `ell = ell0 s^beta`, beta 0.3 to 0.7 (D).
4. **Clustering.** Neyman-Scott: parents = fall points along the scarp, offspring thrown downslope with rule 3; Thomas clumps (sigma 3 to 15 m) for outcrops and erratics.
5. **Competition.** Pass 3 with k 0.8 to 1 (rocks may touch). Rocks run first; flora treats rock footprints as exclusion (UE tutorials do this with Difference/Self Pruning).
6. **Pose.** Half-buried, sink 10 to 40 % of height (D guess), tilt to the normal plus up to 15 degrees; loose rocks not above about 45 degrees; scarp-attached outcrops exempt.
7. Houdini scree workflows use thermal erosion plus scatter (U); I found no talk giving a game size distribution. This is our design; the owner judges the look.

## 5 Determinism and cost

Keep counter-based hashing (`scatter.rs:472-495`, splitmix64), the same idea as Horizon's per-tile evaluation. Extensions (all D):
* **Clusters.** Parent id = hash(seed, species, parent tile, index). Tile T takes parents from every tile within `3 sigma_c + k R_max` of its square, rebuilds their offspring from the
  *parent's* stream (offspring j = hash(parent id, j)), keeps those inside T. Halo about 81 m for sigma_c 15, k R_max 36.
* **Competition.** Fixed rounds k and radius R_max: a point's state after round r depends only on points within r x R_max. Tile T runs k rounds over its square grown by `k R_max`
  on a uniform grid, keeps survivors inside T. Priority = (crown size, hash). **Never an unbounded greedy sweep or "until converged"** (unbounded dependency breaks tile-alone).
* **Rasters.** Scarp and edge distances capped at their reach and computed from tile plus halo heights; flow accumulation baked once, never per tile.
* **Cost (estimates, measure with the census harness).** Candidates grow by `((256 + 2H)/256)^2` about 2.7 at H = 81 m (tile 512: about 1.9). Competition about 0.1 to 0.5 us per candidate.
  Today 7.21 M items take 6.3 s (ledger WG-X5); expect 5 to 10 times that, about 30 to 60 s for 7 M, 5 to 10 s CPU-parallel per million kept.
* **Budget.** Today's Bernoulli budget nests (`scatter.rs:19-22`); competition breaks nesting (removing a tree frees a neighbour). Bake the closed-canopy set once with a per-instance
  rank (size, then hash) and let the budget cut by rank at load (LOD-style, smallest first). Settle with the rendering lane.
* **Tests:** the existing tile-alone bitwise and thread/order tests (`tests_scatter.rs:104-148`), now including halo tiles.

## 6 Crates (crates.io API, 2026-11-26)

| Crate | Version | Licence | Last release | Verdict |
|---|---|---|---|---|
| `fast_poisson` | 1.0.2 | MIT or Apache-2.0 | 2025-03-26 | Bridson over one region, own RNG, not tile-hash-stable; no |
| `faster_poisson` | 1.3.0 | MIT | 2025-10-13 | same; no |
| `bluenoise` | 0.2.2 | MIT or Apache-2.0 | 2025-10-26 | global Poisson disk; no |
| `poisson` | 0.10.1 | MIT | 2019-10-04 | stale; no |
| `blue-noise` | 0.2.2 | MIT | 2026-06-12 | void-and-cluster dither textures; optional (Horizon-style thresholds) |
| `map_scatter` | 0.4.1 | MIT or Apache-2.0 | 2026-01-17 | rule field graph, 1.5 k downloads, source not read (U); not tile-hash-deterministic by design |
| `mermin-stats` | 0.5.0 | MIT or Apache-2.0 | 2026-10-25 | "spatial statistics, correlation functions", 162 downloads, unchecked (U); test-side g(r) at most; own 40-line estimator is simpler |
| `rstar` 0.14.0, `kiddo` 6.3.2 | | MIT or Apache-2.0 | 2026-11 | spatial indices; a uniform grid is simpler and deterministic |

No maintained crate does tile-deterministic cluster plus competition placement: build on our counter hash with a uniform grid (rule 2).

## 7 Why ours looks sprayed (V, `path:line`)

* `scatter.rs:577-607`: each candidate is an independent uniform point in the tile (578), accepted by one Bernoulli draw (587). Independent uniform points are complete spatial
  randomness: no clumping, no spacing, overlap and random gaps. No point sees another.
* `scatter.rs:586`, `66-68`: the only structure is `clump = 1 + 0.6 value_noise` at 48 m: acceptance within 0.4 to 1.6 x mean, a mild Cox process, no hard stand edges, no clearings.
  Thomas-like g(r) peaks cannot occur.
* `scatter.rs:597`: scale uniform in the learned [min, max]: flat size law; rocks "all one size" because the learned range is narrow.
* `scatter.rs:558-575`, `484`: species are independent hash streams, so trees, bushes and rocks interpenetrate; no canopy over understorey, no dominance.
* `scatter.rs:435-470`: the budget thins by species keep probability (random removal ranked by size), not competitively; more density sprays more.
* `scatter.rs:585`: occupancy is slope class x height band only (`scatter.slope_class_deg` 5, `band_ratio` 2): no moisture, aspect, temperature, curvature, scarp distance.
* `flora.rs:204-259`, `scatter.rs:162-183`: what is learned is a region-mean `density_m2`. The mean of a patchy forest, replayed, is a flat spray: the learning step discards patches
  and clearings. The source (hand placement) is a sparse proxy, and no neighbour statistics are learned.
* `flora.rs:231-234`: wet-ground references (38,572) are never placed, so shores are bare.
* It is a correct, fast sampler of the wrong statistical model.

## 8 What the spec should carry

**Passes:** 0 rasters, 1 suitability, 2 stand mosaic, 3 canopy, 4 understorey, 5 rocks (run first), 6 pose, 7 rank and budget.
**Inputs:** the section 3 rasters at 4 to 8 m baked once; a per-universe species table (suitability optima and widths, crown radius per size, shade tolerance, seeding, layer). Learned region
densities become only a prior on stems/ha.
**Tests (`tests_scatter.rs`):** (a) tile-alone bitwise with halo and clusters; (b) g(r) with edge correction: near 0 below 0.8 R_ij, above 1.5 near the clump scale, 0.8 to 1.2 beyond 3 x clump scale
(sanity bands, not tuned look limits); (c) Clark-Evans R in a stated band; (d) size-histogram slope matches the declared law; (e) no pair closer than 0.5 R_ij; (f) talus: rocks inside reach, size rank vs
distance Spearman above 0; (g) an `#[ignore = "measurement"]` print of stems/ha and cover per class.

**Tunables (unit, range, default; source `Guess` unless noted):**
`stand_wavelength_m` m 50-1000, 250; `stand_clearing_threshold` ratio 0-1, 0.35; `edge_band_m` m 5-60, 25; `cluster_sigma_m` m 2-40, 12 (Thomas, U); `cluster_offspring_mean` count 1-60, 12 (Thomas, U);
`background_fraction` ratio 0-0.5, 0.2; `size_law_shape` ratio 0.5-6, 1.2 (Weibull/de Liocourt, U); `crown_k` ratio 0.3-1.2, 0.8 (zone of influence, U); `competition_rounds` count 1-6, 3 (D);
`halo_m` m 16-256, derived; `canopy_cover_max` ratio 0.2-0.95, 0.8 (U); `shade_exponent_per_crown` ratio 0.2-3, 1.0 (Makowski, V); `lapse_k_per_km` K/km 4-10, 6.5 (U);
`rock_size_dim` ratio 1-3.5, 2.0 (Turcotte, U); `rock_sort_beta` ratio 0-1, 0.4; `rock_shadow_angle_deg` deg 15-35, 25 (Evans and Hungr, U); `talus_slope_min_deg`/`max_deg` deg 10-45, 15/40 (repose, U);
`rock_burial_ratio` ratio 0-0.6, 0.25. All under the `scatter.` prefix.

## 9 Open points (taste and scope)

1. May dense forest block sight lines, with guaranteed clearings at roads and sites? This design supports both (`canopy_cover_max`, forced clearing rasters). No game gives trees per hectare; the closed-canopy
   arithmetic gives about 100 to 800 stems/ha for crown radius 8 to 2 m.
2. Later, a one-off coarse Nocca-style competition as an input raster for emergent species mixing? Out of scope for the first slices.
3. Rocks as their own budget group with their own rank cut (suggested yes).

## 10 Sources (URLs)

Makowski 2019 https://wp.faculty.wmi.amu.edu.pl/papers/Makowski.etal-2019-Synthetic-Silviculture.pdf · van Muijden 2017 https://www.guerrilla-games.com/media/News/Files/GDC2017_VanMuijden_GPUBasedProceduralPlacementInHorizonZeroDawn.pdf ·
Nocca 2021 https://www.cs.purdue.edu/homes/bbenes/papers/Nocca21ToG.pdf · Cordonnier 2017 https://cs.purdue.edu/homes/bbenes/papers/Cordonnier17ToG.pdf (DOI 10.1145/3072959.3073667) ·
EcoBrush https://hal.archives-ouvertes.fr/hal-01519852 · Kapp 2020 https://showcase.sit.uct.ac.za/event/showcase-2020/data-driven-authoring-of-large-scale-ecosystems ·
Epic https://dev.epicgames.com/documentation/en-us/unreal-engine/procedural-foliage-tool-in-unreal-engine and https://dev.epicgames.com/documentation/unreal-engine/open-world-tools-property-reference-in-unreal-engine ·
Wildlands https://80.lv/articles/vegetation-generation-in-ghost-recon-wildlands/ · FC5 https://blog.playstation.com/2018/03/22/the-procedural-world-generation-of-far-cry-5/ ·
Thomas/Matern https://rdrr.io/cran/spatstat.model/man/Kmodel.clusterprocess.html, https://arxiv.org/pdf/1103.1598 · Self-thinning https://www.waldwachstum.wzw.tum.de/fileadmin/publications/576.pdf,
https://www.frontiersin.org/articles/10.3389/ffgc.2020.00062/full · Stand structure https://annforsci.biomedcentral.com/articles/10.1007/s13595-016-0563-4 ·
Rocks https://nhess.copernicus.org/articles/21/1159/2021/, https://link.springer.com/doi/10.1007/BF00874486, https://orca.cardiff.ac.uk/id/eprint/146759/1/1-s2.0-S0191814121002200-main-2.pdf ·
Poisson disk https://diglib.eg.org/items/19b32179-fc46-4efd-9aea-5794320cac13/full · crates https://crates.io/crates/<name> (names in section 6) ·
In repo: `superfargoth/docs/research/worldgen-suite-2026-11-26.md` section 4, 5; `climate-fields-2026-11-26.md`; `docs/ledger/worldgen.md`.

## Verified (2026-11-26, second pass)

Verdicts use the marks above: only what a source showed this session. "Page" = fetched page text, "snippet" = search result text only (weaker).

| Claim | Verdict | Source | Detail |
|---|---|---|---|
| Cordonnier: layered grid, vegetation as density per type, cell 10 m, 10 x 10 km | confirmed | HAL author PDF (below) | "size of a cell ... 10 x 10 m2 ... authoring of landscapes up to 10 x 10 km2 ... at interactive rates". Layers: bedrock B, granular R, C, H, vegetation V (grass, shrubs, trees), dead D. |
| Cordonnier: "simulations hours" | could not verify | same | Table 1: ecosystem events 0.027 s (n=128) to 0.94 s (n=1024) per yearly step; whole step 38 s at n=1024, rain is 75 % of cost. |
| Epic: Num Steps, Initial Seed Density, Avg Spread Distance, Spread Variance, Seeds Per Step, Max Age, Collision Radius, Shade Radius, Overlap Priority, Scale Curve | confirmed | dev.epicgames.com/documentation/unreal-engine/open-world-tools-property-reference-in-unreal-engine (page) | Initial Seed Density: "number of seeds ... along 10 meters ... implicitly squared to cover a 10m x 10m area". Overlap Priority: lower value is removed. Shade Radius: "ignored" if the type can grow in shade. Spread distances read as cm in the page's example. |
| Epic: seed property | corrected | same | No plain "Seed": Distribution Seed (initial seeds) and Random Seed (simulation). Also Can Grow in Shade, Spawns in Shade (second pass, needs Can Grow in Shade). |
| Epic tutorial values | new | dev.epicgames.com/documentation/en-us/unreal-engine/procedural-foliage-tool-in-unreal-engine (page) | Tutorial uses Num Steps 3 to 4, Initial Seed Density 0.125 to 0.25, Max Age 15 to 20, Shade Radius 50. Num Steps 0 gives uniform trees. |
| Reineke exponent -1.605 | confirmed | frontiersin.org/articles/10.3389/ffgc.2020.00062/full (page) | "the coefficient -1.605 appeared invariant across many species". |
| Reineke reference diameter 25 cm | confirmed (snippet) | Woodall et al. 2003; Heiderman and Kimsey 2021 (search snippets) | SDI = trees/ha at quadratic mean diameter 25 cm; Reineke used 10 in. |
| SDI cap 700 to 1100 /ha | could not verify | | Only a ponderosa snippet (SDI max 100, unit unclear). |
| Matern I and II intensities | confirmed (snippet) | arxiv.org/pdf/1505.07215 | II: (1 - exp(-rho w D^d)) / (w D^d); I: rho exp(-rho w D^d); w_2 = pi. Matches our formulas with R = D. |
| Thomas g(r) | could not verify | spatstat page and arXiv 1103.1598 gave no formula | Stays U. |
| Rock shadow angle 22 to 28 deg (Evans and Hungr) | partly confirmed | catalog abstract of Evans and Hungr 1993 (snippet); USGS Yosemite report | Abstract: minimum shadow angle 27.5 deg. The 22 deg is from a USGS report (25 outlier boulders), not shown to be in Evans and Hungr. Use 27.5 for them. |
| Fall sorting weak or absent on some cones | confirmed | nhess.copernicus.org/articles/21/1159/2021/ (page) | "gravitational sorting did not occur on all of the studied debris cones"; "only weak correlations between block volume and runout distance". The paper fits no size law. |
| Turcotte N ~ r^-D, D 1.6 to 3 | partly confirmed | Turcotte 1986 abstract (snippet) | N ~ r^-D defines the fractal dimension. No D range found. A Science News piece quotes D = 1.6 (Sammis). |
| Crown width linear in DBH | confirmed for open-grown trees (snippet) | Brack and Wood, fennerschool-associated.anu.edu.au/mensuration/BrackandWood1998/CROWN.HTM | Radiata pine: CW = 0.7544 + 0.2073 d (m, cm). Evidence is mixed across studies; Hemery 2005 coefficients not found. |
| Stand densities 300 to 700 and 1000 to 3000 /ha; Weibull c, de Liocourt q; Ripley K shape in real stands | could not verify | Springer and Getzin pages were blocked or gave only abstracts | Stays U. |
| Bake 30 to 60 s | unmeasured: becomes a measurement row | | |

## Cordonnier 2017 in full

Read: the HAL author PDF (hal.science/hal-01518967/document), text recovered from a truncated download (about 5 MB of 30 MB, throttled to about 10 KB/s). That covered sections 3 to 7 (method, ecosystem events, implementation, results start) but not the later results, discussion or references. Purdue copies stayed too slow to finish. Not a full read of the paper, but a full read of the vegetation method.

The model is an Eulerian, cell-based aggregate simulation, not per-plant positions. The doc's "density per type" is right. Per cell and per layer (trees, shrubs): plant count n, sum of heights h, sum of ages a. Grass: density only. The paper rejects circular-footprint plant competition as too costly beyond about 1 km2.
* **Events:** each step draws about N > 256 x 256 x 10 random (cell, event) pairs. Time step is 1 year. Ecosystem events run per cell on average once a year.
* **Viability:** monthly hat function v(c) per condition (temperature, moisture, sun), 0 to 1 style trapezoid with limits Emin, Emax and ideal Imin, Imax (after Gain 2017). V = min_i v_i (Liebig). Result in [-1, 1]; negative = stress (drought, frost).
* **Inputs:** monthly temperature lowered at 6.5 C per 1000 m (value now V); moisture from the yearly layer M split by monthly rain; sun hours from ray casting on the terrain.
* **Yearly values:** vigor = mean viability in months with mean temperature at least 5 C. Stress = mean of the 4 worst monthly values, used only if negative.
* **Density:** d = n x (r x mean height)^2 / w^2 (pi lost in text extraction), r = crown radius to height ratio, w = cell width. d > 1 forces self-thinning; d also darkens shrubs under trees (sun cut in proportion to d).
* **Germination:** seedlings added in proportion to vigor and free space (1 - d), only with no stress, from an assumed soil seed bank (no dispersal). Woody establishment 0.24 saplings per m2 (Prentice 1993).
* **Growth:** constant yearly height gain scaled by vigor.
* **Death, in order:** thin to d < 1; remove in proportion to stress; random old-age loss above an age threshold. Dead matter to litter; 30 % of litter becomes humus per year; biovolume b = 0.52 h^2.55 m.
* **Cost:** per-step total 0.11 s (n=128) to 38 s (n=1024); CPU threads gave x4 on 12 threads.

Consequence for us: it has no per-tree positions, so it cannot give crown-scale spacing. Reusable ideas: the hat-function min viability, the 5 C growing season, the density d as a cap, 0.24 saplings per m2.

## Still unverified

* Cordonnier: sections after 7, the parameter tables if any, and the "hours" run time.
* EcoBrush (Gain 2017) and Kapp 2020: HAL blocked, no full text tried successfully.
* Forestry: stem density ranges, SDI cap 700 to 1100, Weibull shape and de Liocourt q values, crown allometry per species, Ripley K or g(r) shape in real stands.
* Thomas g(r) formula; random sequential adsorption 55 % saturation.
* Rocks: talus 15 to 40 deg, scarp 45 to 60 deg, Turcotte D range, beta 0.3 to 0.7 (ours, D).
* Wildlands, Far Cry 5, UE PCG Biome Core and Self Pruning pages (not rechecked).
* Bake 30 to 60 s: unmeasured, becomes a measurement row.
