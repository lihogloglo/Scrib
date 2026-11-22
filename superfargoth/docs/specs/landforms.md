# Landforms (`sf_worldgen`: placed landforms, geology map, geology-driven runtime detail)

Status: spec (2026-11-22); LF-0, LF-1 and LF-2 built ("As built"). System row: `docs/SYSTEMS.md` "World generation". Ledger:
`docs/ledger/worldgen.md` (`LF-` lines). Parent spec: `worldgen.md` (stages 3, 6 and 7 of its "Pipeline"). Game side:
`bevyrrowind/docs/specs/worldgen-adapter.md` "Landforms". Vertical faces, overhangs and caves: `rockgen.md` (this spec
exports what it needs). Slice IDs `LF-*` (framework) and `LF-A*` (adapter).

## Goal

Generated relief that reads as *places*: mostly calm, walkable ground, with a limited number of discrete, readable
landforms (an escarpment, a mesa, a canyon, a caldera wall, a sea cliff, a lava front, a terrace flight, the fans at
their feet) put where the geology and the drainage say they belong, each with the classic profile **flat top, sharp
lip, 60-80° face, 30-35° talus, plain**. The rest of the land is left alone. Fine detail on the ground follows the rock
it sits on (strata on layered cliffs, blocks on outcrops, gullies on soil) and never makes the ground unwalkable.

Replaces the everywhere-noise passes of `sf_worldgen::cliffs` (2026-11-20) and the runtime `sharp_offset`
(`filter.rs`), which the owner rejected (below). Framework only: the operators, masks and products are generic; which
regions get which landforms, rock types and game sources live in the adapter.

## Owner decisions (2026-11-22, quoted from the session brief)

1. "Prefers the vibe of the 10x Morrowind + additive Red Mountain apron base (`target/worldgen/x10s`, from `x10v/s55`)
   from BEFORE the sharp-cliff change. That base stays the reference." Every arm here is a delta on that base.
2. "The previous 'sharp' approach reads spiky, not varied; cliffs don't read as cliffs; terrain not walkable. Wants
   landforms placed 'a bit smarter'."
3. "Optional 'calm the base' pass is OK as an A/B with a live/declared strength, never a replacement."
4. "Automated checks = printed numbers only; the owner judges the look."
5. A later "human stuff" pass (settlement pads, graded roads, bridges) comes after nature; **not in this spec beyond
   its interface**: a human mask that suppresses runtime detail, and roads/pads run after landforms.
6. Vertical faces, overhangs and caves come from rockgen; this spec exports "feature list / cliff mask / strata +
   hardness per point".
7. (Owner 2026-11-22) "Canyons can be barriers, yes (we can always put bridges when needed)." Canyons need not
   stay crossable on foot: crossings at heads/junctions are optional per region, and the cut-off spans are exported
   as bridge candidates for the human pass. Other landforms keep their routes.
8. (Owner 2026-11-22) Summit calderas: "ideally several ways into the mountain (like in the game)": several breaches,
   not one.
9. (Owner 2026-11-22) Small-cliff density is not a number to ask for: "we'll iterate". It is a declared tunable
   tuned in look loops.
10. (Owner 2026-11-22) Climbing: "yes, but optional and much faster to walk around"; mounts, airships and powers carry
    big traversal. Steep faces are climbable but cost more than walking around them: routes must keep walking around
    a face clearly cheaper than climbing it; no per-face climb markup.

**What went wrong (ledger 2026-11-22, the lesson this design is built on).** At 18.288 m spacing, land at ≥ 35°: Etna
1.8 %, Teide 6.2 %, `x10s` 33 %, `x10c-sharp` 49 %, `x10cs` 56 %; largest walkable region (≤ 46°,
`MoverParams::max_slope_deg`) 99.9 / 98.7 / 60 / 14 / 5 % of land. Real volcanic islands have their steep tail in a few
places on mostly flat ground (Etna: 52 % of land under 10°; the sharp arms 10-12 %). **Flat ground is what lets a cliff
read.** Causes (ledger has the detail): `cliffs.rs` passes are periodic textures gated only by slope (one 620 m cuesta
wavelength, 58 m shelves island-wide, ×1.35 unsharp steepening, scarps on flat ground); `filter.rs` `sharp_offset`
made 43-52° risers 6-10 m tall on 15-20° hillsides from local slope alone, non-zero mean, and faded the staircase's
amplitude instead of filtering it (CDLOD L0→1 moved the surface 49-58 px). Now off (`worldgen.sharp 0`).

## Reuse check (rule 0-2, 2026-11-22)

**Bevy:** nothing for terrain generation (n/a). The runtime side reuses our own `sf_terrain_bevy` clipmap fill and
`surface.wesl` material path (WG-X7a/b); only the inputs change.

**Our own code (reused, not rewritten):** `lithology::RockMap` (warped Voronoi rock units, `unit_weights` smooth
blend, edge bands); `thermal::relax` (Jacobi, per-sample repose angle); the Veloren erosion port (`lem/`, WG-S5:
basin tree, multi-receiver stream power) as the fluvial settle; `mfd.rs` / `flood.rs` (priority-flood, flow
accumulation); the landform census (`landforms/`, WG-L2a: plateau, headland, crater, gorge, valley detectors) as
*placement candidates*; `cliffs.rs` `route_lava_lobes`, `stroke_delta`, `SlopeStats` (moved into the new module);
`measure` `Walkability` (2026-11-22); `stamps` weights (anchors) as the first human mask; `tests_tiles.rs`
`measure_lod_switch_error` as the band-limit check.

**Crates:** `rstar` 0.13 (MIT/Apache-2.0, already in `Cargo.lock`) for nearest-segment queries when rasterising lip
distance fields and for spacing checks. Polyline simplification, contour extraction and the analytic profile filters
are a few dozen lines each; `geo` / `contour` (MIT/Apache, georust) are the fallback if ours grow (not added now:
compile time, and they don't match our grid frame). No crate places landforms; none does analytic filtered
profiles. Noise stays our `pcg3d` hash (CPU/WGSL bit-identical, the twin requirement), not `noise`/`fastnoise-lite`.

**Open-source and tool prior art:**

- **Gaea / World Machine node vocabulary** (S: docs.quadspinner.com search summaries; WM product knowledge): Gaea
  `Terraces` (multi-octave, spacing/intensity/shape), `Stratify` ("broken strata or rock layers... non-linear"),
  `Fold` (slants, breaks, folds; "use early in the graph"), `Stacks` (a stratified mesa in one node where it otherwise
  takes 12-20), erosion with talus; WM Terrace, Thermal (talus angle), Layout/brush shapes. We take the vocabulary
  (one operator per landform, masks out of every operator) and reject the usage pattern of *global* filters over the
  whole terrain, which is what `cliffs.rs` was. Proprietary tools: ideas only.
- **Veloren** (V, `world/src/sim/erosion.rs` @6c99b292, GPL-3.0, already ported in WG-S5): rock-strength noise and
  uplift drive erosion; no placed cliffs. Our fluvial settle.
- **terrain-diffusion** (V, our TD-1..3 trials, `tools/terrain-diffusion/`, ledger 2026-11-18): plausible detail
  below 1 km, but it ignores our drainage and fills craters, and steering is slow (15 min / 830 km² inversion; the
  encoder 0.35-0.6 s). Not a placement tool; stays a possible exemplar source for runtime detail, out of scope here.
- **Schott et al. 2023**, "Large-scale terrain authoring through interactive erosion simulation", ACM TOG 42(5) (V:
  abstract + `H-Schott/StreamPowerErosion` README): paint uplift, stream power makes drainage-consistent ridges
  interactively. Confirms *forcing fields, not target heights* (our WG-S5); it makes ridges, not cliffs.
- **Cordonnier et al. 2016** (S, our macro stage) and **2018** geology/uplift (S); **Paris et al. 2019** implicit 3D
  features (V via research doc §2): the bake-to-mesh route, which is rockgen's.

## Industry comparison (V = source opened, here or in `docs/research/terrain-sharp-features-2026-11-19.md`; S = secondary; U = unverified)

| Who | What they do | Us |
|---|---|---|
| Horizon ZD/FW (V, GDC 2017 van Muijden) | World as 2D maps; nature placed by GPU rules over height, flow, deposition, ecotope maps | Match: landforms placed by rules over geology, drainage and intent maps |
| Far Cry 5 (V, GDC 2018 Carrier) | Houdini cliff tool reads slope/topology, makes cliff geometry, blends displacement into terrain deterministically | Match: heightfield landforms here, cliff meshes in rockgen, one cliff mask for both |
| Ghost of Tsushima (S, Sucker Punch world artist's portfolio via search) | Terrain hand-sculpted for horse navigation; cliffs "with a clear visual language for navigation... minimally obstructive"; river widths and fords show what is crossable | Match in intent: budgets, routes (ramps, relay ramps, fans, canyon heads) and per-region walkability numbers |
| RDR2 (U) | Cliffs marked for horse AI | Match: `features.json` + cliff mask give nav/AI a steepness label |
| Bethesda games (S; their cliff-mesh placements V in our adapter census) | Heightfield + placed cliff/rock meshes | Match: a game's own cliff-mesh footprints become intent masks (adapter) |
| Gaea/WM/Houdini (S) | Per-landform nodes with masks; strata + hard/soft thermal for cliff-and-talus | Match for operators; we drop global application |
| Schott 2023 (V abstract) / Cordonnier 2018 (S) | Uplift- and geology-driven relief | Match: geology map drives placement and settle |
| Quilez, filtered procedurals (S, iquilezles.org articles) | Box-filter a pattern analytically over the pixel/sample footprint; frequency-clamp what can't be integrated | Match: every runtime operator here |

Gap logged for `industry-audit.md` at LF-6: no learned or exemplar detail on faces (rockgen meshes cover the near range).

## Data model

All lengths metres, guide frame (x east, y north, row 0 north), heights absolute. Bevy-free types in
`sf_worldgen::geology` and `sf_worldgen::placed` (new modules; the name `landforms` is taken by the WG-L2a census).

- **`RockUnit`** (table, adapter data): `name`, `hardness` [0, 1], `k` (erodibility, as `RockType`), `layered`
  (bool), `bed_m` [min, max] (bed thickness, hashed per bed), `soft_bed_share` (beds below hardness 0.5),
  `dip_deg`, `dip_mode` (`Fixed(azimuth)`, `Radial(centre)`: dips away from a centre, volcano flanks; `Surface`:
  beds parallel to the low-passed surface), `repose_hard_deg`, `repose_soft_deg`, `outcrop` (blocks allowed),
  material layer ids `{top, face, scree}`.
- **`GeologyMap`** (bake product, guide folder `geology/`, one sample per guide sample, little-endian, row-major):
  `unit.u8` (index into `geology.json`), `hardness.u8` (/255), `datum.f16` (strata datum: a bed boundary sits at
  `z − datum = Σ beds`; smooth, low-passed to ≥ 4 samples so it interpolates), `cliff.u8` (face weight),
  `talus.u8` (scree weight), `human.u8` (0 now; later roads/pads; stamps' weight is folded in at load), `lip.f16`
  (signed distance to the nearest feature lip, + outward/down, clamped ±`lip_band_m`), `feature.u16` (0 = none,
  else index + 1 into `features.json`). 10x island (3329 × 3073): 12 bytes/sample = 123 MB on disk; hashed in
  `meta.json`.
- **`Bed` query** (CPU, the twin of the WGSL function): `bed_at(x, y, z) -> {index: i64, hardness, frac}`, from
  `unit`, `datum` and the per-bed hash (`pcg3d(unit, index, seed)`). One function serves runtime geometry, texture
  banding and rockgen's strata SDF, so meshes, ground and texture show the same beds.
- **`Feature`** (`features.json`, format 1): `id`, `kind`, `seed`, `lip: [[x, y]]` (polyline or closed ring), `top_m`,
  `base_m`, `face_deg`, `talus_deg`, `face_width_m`, `apron_m`, `units: [top, face, base]`, `routes: [[x, y]]`
  (ramp points), `bbox`. Rockgen's band finder and the scatter's talus rule read it; nav/AI may read `kind`.
- **`LandformRecipe`** (TOML, adapter-supplied): per region `{ operator: { weight, budget_face_share, size_b,
  size_min_m, size_max_m, spacing_ratio } }`, the unit shares, and `intent` masks (paths to `*.f32` rasters).

## Pipeline (bake order)

Each step is a pure function of its inputs + seed, cached by content hash (`blake3` of inputs + declared values),
so changing a placement value reruns steps 4-8 only. Stage rules of `worldgen.md` apply: no step creates or drains
water (wet/dry audit after each), masks are rebuilt from the current field, anchors are fixed boundary conditions.

0. **Inputs:** base guide (`x10s` heights: `x10v/s55` + anchors), `regions.png`, `stamps.json` (anchor weights),
   adapter `geology.toml` + `landforms.toml`, intent masks (`cliff_mask.f32`, adapter-made), optional
   `[[stroke]]`s (kept from `cliffs.rs`).
1. **Calm base (optional, off).** Band split by Gaussian differences: `low` (> `calm.hi_m` 1 km), `mid`
   (`calm.lo_m` 100 m - 1 km), `high` (< 100 m). `h' = low + (1 − s·w)·mid + high`, `s` =
   `worldgen.landforms.calm_strength` (default **0** = identity, step skipped), `w` per region (adapter) × (1 −
   anchor weight dilated by its band) × coast guard (never moves a sample across sea level). Owner A/B arms s = 0 /
   0.3 / 0.6. Purpose: a uniformly scaled source turns 30 m hills into 300 m steep ones; this softens that band only.
2. **Geology map:** `RockMap` per region shares (adapter) → unit; hardness; datum by `dip_mode` (`Surface` uses
   the 2 km low-pass of step 1's field, so volcano beds lie parallel to the cone: strata show only where something
   cuts across them, which is what real layered volcanoes do).
3. **Analysis:** priority-flood + MFD flow accumulation `A`; slope; plan curvature; census detectors on the field
   (plateaus, headlands, craters, gorges); distance to coast; anchor mask. Cached; no heights change.
4. **Placement** (operators below, fixed kind order, each reading the current field): candidates from a per-kind
   score raster (geology × drainage × region weight × intent), sizes from a truncated power law `N(>L) ∝ L^−b`
   (`size_b`, `size_min_m`, `size_max_m` per region), ranked by `score × hash` and accepted while (a) the region's
   face budget is left, (b) no accepted feature of the kind lies within `spacing_ratio × L`, (c) it avoids anchor
   bands. Each writes a height delta **pre-filtered at the guide footprint** (profile box-filtered over
   `2 × spacing`, so the bicubic base doesn't ring), plus `lip`, `feature`, `cliff`, `talus`.
5. **Routes:** walkability (≤ 46°, 4-connected) per region at guide spacing. While the largest walkable component
   is below `route.min_largest` of the region's land and fewer than `route.max_ramps` were cut: join the two
   largest components through the cheapest face crossing (fewest samples > 46°) with a **ramp**: the face replaced
   by a ≤ `route.ramp_deg` slope, `route.ramp_width_m` wide, written into `routes`. Fixed order; deterministic.
6. **Settle:** `thermal::relax` with per-sample repose from the unit and bed (hard 68-72°, soft 33-35°), only within
   `settle.margin_m` of a changed sample; then the Veloren fluvial port, `settle.fluvial_steps` short steps, no
   uplift, anchors and sea fixed, so channels on new tops and fans connect to the network.
7. **Masks again** from the final field: `cliff` = feature face where slope > `cliff.slope_deg` (+ intent), `talus`
   = below a face, slope in the scree band, down the fall line within `apron_m`. Products written (heights, `geology/`,
   `features.json`, `walkability` report, hillshade + overlay PNG).
8. **After this spec (interface only):** the human pass reads `features.json` + heights, writes pads and graded
   roads as height edits and `human.u8`; it always runs after step 7.

### Operator catalog

| Operator | Placed where (score) | Profile and size | Leaves a route by |
|---|---|---|---|
| `fault_scarp` | rock-unit boundaries picked per boundary hash (`fault_share`), boundary traced from the unit map, simplified, warped | upthrown side flat, lip, face 60-75°, talus 33°, plain; throw tapers to 0 at segment ends (elliptic along strike, max throw `throw_ratio × length`, S: displacement-length scaling, Cowie & Scholz 1992); en-echelon segments overlap | relay ramps between overlapping segments; tapered tips |
| `mesa` / `butte` | census plateaus on units under a hard `layered` cap (TPI > 0, slope low) | top flattened to the cap bed's base, face through the cap, talus on soft beds; buttes = detached remnants near mesa edges, size law per region | talus chutes at `route` points; buttes are islands, not barriers |
| `canyon` | channel reaches with `A > canyon.min_area_m2` on hard layered units, thalweg slope < `canyon.max_bed_deg` | depth `∝ A^m` capped, width `∝ A^0.5` (S, bedrock hydraulic geometry); hard beds steep, soft beds benches (stepped walls); depth 0 at the head knickpoint | may be a barrier (owner 2026-11-22): side talus ramps at heads/junctions with probability `canyon.crossing_share` per region; cut-off spans exported as bridge candidates (`features.json`) for the human pass; the walkability repair never ramps across a canyon |
| `caldera` | census craters + adapter intent (summits) | ring lip, inner wall 60-75°, flat floor (infill), outer flank untouched | several breaches (owner 2026-11-22: "several ways into the mountain"): `caldera.breaches` (count, default 3, Guess), placed where the adapter's paths/channels meet the rim |
| `sea_cliff` | census headlands on hard units; bays stay beaches | flat top, face to a shore platform ≥ sea + `sea.platform_m`, no new water | bays and coves between headlands |
| `lava_lobe` | adapter vents (cones, rifts) | `route_lava_lobes` (exists): flat top, levees, 5-30 m aa fronts at 35-45° | flow fronts are low; lobes end |
| `terraces` | layered units on 15-35° valley sides; river terraces along major channels | flat treads ≥ `terrace.tread_min_m`, risers 4-20 m (cliffs) | treads are walkable; riser gaps every `route` spacing |
| `fan` | foot of every face (talus) and canyon mouths onto plains | talus cone 30-35° up to the face; alluvial fan 2-10° (S, Bull 1964: fan area grows with catchment) | fans are the ramps up to cliff feet |

`[[stroke]]` hero escarpments stay as a manual `fault_scarp` with given trace and throw. The `cliffs.rs` cuesta,
fixed-step shelves, unsharp steepening and slope-gated scarps are **not** carried over (dead once LF-4 lands; deleted
then, tagged, per the delete-dead-code rule; the `x10c` arms stay reproducible from the tag).

## Runtime detail

Runs in the GPU clipmap fill (`tiles.wgsl`) with the CPU twin (`filter.rs` / `tiles.rs`), reading the
`geology/` grids (same path as `LayerField`: guide-resolution grids for the tiles the fill touches). **Rule: every
operator is analytically band-limited at the level's sample footprint `w_L`** (box filter integrated in closed form,
or frequency-clamped where it can't be), so a CDLOD switch moves the surface < ~2 px at p99 (measured by
`measure_lod_switch_error`). Large forms are never runtime: they are in the guide.

1. **Lip restore.** The guide holds each profile filtered at `2 × spacing`; the fill adds `P(lip, w_L) − P(lip,
   w_guide)` for the feature in `feature.u16`, `P` the piecewise-linear profile (top, face, talus, plain) box-filtered
   analytically (piecewise quadratic). Coarse levels add 0; near levels get the sharp lip and straight face back.
   Removes the rounded "blob" lips without a finer guide.
2. **Strata** only where `unit.layered` and `cliff > 0`: the bed function `bed_at` gives a staircase in `z − datum`
   (hard beds steep risers, soft beds recessed), **box-filtered in the stratigraphic coordinate over `w_L × slope`**
   (closed form of the integrated staircase), and **zero-mean** against the ramp it replaces. Elsewhere on layered rock
   risers are capped at `detail.riser_cap_m` = 0.45 m (`MoverParams::step_up`): the ground gets ledges you walk up, or
   only texture bands.
3. **Blocks** (Voronoi F2−F1 fracture) only where `unit.outcrop` and (`cliff > 0` or slope > `blocks.slope_deg`),
   zero mean per cell, amplitude frequency-clamped by `saturate((cell / w_L − 2) / 2)`.
4. **Ridges** only aligned with the downslope direction (the gully filter's cascade gradient); the random-direction
   ridged term is dropped.
5. **Gullies** (the existing filter, `relief_fraction` 0.25) kept on soil and soft units, attenuated on faces (strata
   take over) and on talus (scree is smooth at this scale).
6. **Human mask:** all of the above × (1 − `human`), stamps' weight included (as today).

Cost: tile fill only (amortised, ~3 ms per 512² L0 fill today); expected +0.3-0.8 ms per fill on the 4060 (Guess,
measured at LF-5 untraced vs `worldgen.sharp 0`); the frame is unaffected except through fill throughput. CPU twin
cost per height query printed at LF-5.

## Materials (terrain shader, `surface.wesl`)

- **Per-unit layers:** `P_CLIFF`'s single `rock_layer_id` becomes a small table indexed by `unit.u8`: face layer,
  top layer (else the LTEX/splat layer), scree layer, band strength. Weights height-blended as today (WG-X7b);
  `cliff.u8` raises the face weight where slope alone would miss it (lip, short faces).
- **Texture strata banding** from the same `bed_at` (index → hashed tint/brightness, hardness → slight albedo
  contrast), so colour bands sit on the geometric ledges; filtered by the pixel footprint the same way.
- **Scree** layer on `talus.u8`; fans keep the soil layer below `fan.scree_deg`.
- Rockgen meshes use the same layer table and `bed_at` (rockgen "Materials"): one rock look across ground and meshes.

## Interfaces

- **Rockgen** (`rockgen.md` "Cliff bands", "Interfaces"): bands where `cliff.u8` > threshold *or* a feature face
  lies within range; `features.json` gives lip, top/base heights and kind; `bed_at` gives strata + hardness per point
  (its `recess_m` uses the bed hardness); heights = the CPU twin including lip restore.
- **Scatter** (WG-N2): talus fans from `talus.u8` and `features.json` (boulder size grows down the apron), outcrop
  blocks from `unit.outcrop`; flora avoids faces.
- **Nav/AI** (later): `feature.kind` + `cliff` as a steepness label (the RDR2 pattern).
- **Human pass** (later): runs after step 7, reads features and routes, writes `human.u8`.

## Measurement (printed, never pass/fail limits)

`scrib-worldgen measure` gains, per region (from `regions.png`) and for the whole land: the **Walkability table**
(slope bands 0-15/15-25/25-35/35-46/46-60/>60, walkable share, largest component, component count), land share ≥ 35°
and under 10°, face area per operator kind, feature counts and the fitted `b` of the size law, **untouched land**
(|Δh| < 0.5 m vs the base), Δh p50/p95, wet/dry audit, bake time per step and the product hash. Reference rows:
`real/etna-20m` and Teide at the same spacing. The LOD-switch test prints p50/p99/max px per switch for the runtime
operators. Tests (exact, logic): determinism (same hash twice), each operator's profile on a plane fixture (top flat,
face angle, talus angle within the declared values), relay ramp and canyon head connectivity on fixtures, wet/dry 0,
anchors bit-exact inside footprints, CPU twin = WGSL for `bed_at` and the filtered profiles.

**Targets used for budgets (declared, Reference = the real DEMs):** land ≥ 35° per region 2-8 %, largest walkable
component ≥ 95 % of the region's land, untouched ≥ 70 %. These steer the defaults; the owner judges the look.

## Tunables (declared per `tunables.md`; all `Lifetime::Bake` unless marked, `Scope::Dev`)

| Name (`worldgen.landforms.*` unless noted) | Default | Unit, range | Source |
|---|---|---|---|
| `calm_strength` | 0 | ratio 0-1 | Owner(2026-11-22: optional, off) |
| `calm.lo_m` / `calm.hi_m` | 100 / 1000 | m | Guess |
| `face_budget` (per region, adapter overrides) | 0.04 | ratio 0-0.3 | Reference(Etna 1.8 %, Teide 6.2 % ≥ 35°) |
| `size_b` | 1.3 | 0.5-3 | Industry(S: fault-length power laws b ≈ 1-2) |
| `spacing_ratio` | 1.5 | ratio | Guess |
| `face_soft_deg` / `face_hard_deg` / `talus_deg` | 45 / 75 / 33 | deg (face linear in the upthrown unit's hardness) | Industry(V: Wallace 1978 fresh scarps 50-90; cap-rock 60-75, repose 32-36) |
| `eroded`, `throw_noise` / `throw_noise_m`, `segment_throw_var`, `tip_ratio` | 1, 0.35 / 1200, 0.3, 0.2 | switch, ratio / m, ratio, ratio | Guess (LF-2b) |
| `chute_min_m` / `chute_max_m` / `chute_b`, `chute_width_share`, `chute_wall_deg`, `chute_depth_max` | 50 / 300 / 1.0, 0.9, 40, 0.85 | m / m / -, ratio, deg, ratio | Owner(2026-11-22 brief: 50-300 m) / Guess |
| `cone_deg`, `facet_talus_share`, `prefilter_taps` | 12, 0.1, 5 | deg, ratio, count | Industry(S: Blair & McPherson 1994 fans 1.5-25) / Guess |
| filter `steep_fade` / `_lo_deg` / `_hi_deg` (per world, `filter.toml`) | 0 / 40 / 50 | switch / deg | Owner(2026-11-22: no dents on faces) / Guess / Reference(rockgen cliff.slope_deg) |
| `throw_ratio` | 0.02 | ratio 0.005-0.1 | Industry(S: Cowie & Scholz D/L ≈ 0.01-0.03) |
| `route.min_largest` | 0.95 | ratio | Reference(Teide 98.7 %) |
| `route.ramp_deg` / `route.ramp_width_m` / `route.max_ramps` | 30 / 24 / 400 | deg / m / count | Guess (≤ 46 with margin) |
| `canyon.min_area_m2` / `canyon.max_bed_deg` | 2e7 / 6 | m² / deg | Guess |
| `canyon.crossing_share` | 0.5 | ratio (per region) | Guess (owner 2026-11-22: canyons may be barriers; bridges later) |
| `caldera.breaches` | 3 | count | Guess (owner 2026-11-22: several ways into the mountain) |
| `sea.platform_m` | 1.5 | m | Guess |
| `settle.margin_m` / `settle.fluvial_steps` | 300 / 8 | m / count | Guess |
| `cliff.slope_deg` | 45 | deg | Reference(rockgen `cliff.slope_deg`) |
| `lip_band_m` | 120 | m | Guess |
| `worldgen.detail.riser_cap_m` (Live) | 0.45 | m 0-2 | Reference(sf_controller `MoverParams::step_up`) |
| `worldgen.detail.strata` / `.blocks` / `.lip` (Live) | 1 | scale 0-2 | Guess (A/B switches) |
| `worldgen.detail.blocks.slope_deg` (Live) | 35 | deg | Guess |

Runtime `Live` values go through the console like `worldgen.sharp` today; bake values through the recipe TOML.

## Build plan

One deliverable + its test per slice; bake times are expected on the 10x island (3329 × 3073, 10.2 M samples, 8
threads), printed by the slice.

- **LF-0** measure: per-region Walkability table + land ≥ 35° / < 10° + untouched share in `scrib-worldgen measure`.
  Test: two-region fixture, exact shares. Baseline rows for `x10s`, `x10c-sharp`, Etna, Teide. (< 5 s)
- **LF-1** geology map: `RockUnit` table, `GeologyMap` products, `bed_at` CPU. Test: determinism, `Surface` dip gives
  no bed crossings on a cone, `Fixed` gives the expected count on a plane. (~3-5 s)
- **LF-2** placement core + `fault_scarp` + `fan` (talus), routes step, settle (thermal only). Test: plane fixture
  profile angles, relay ramp keeps one component, budget respected. First owner arm: `x10s` + scarps. (~10-20 s)
- **LF-3** `mesa`/`butte`, `sea_cliff`, `caldera`, `lava_lobe` (moved). Test: profile fixtures, wet/dry 0,
  headland-only cliffs. Owner arm. (~15-25 s)
- **LF-4** `canyon` + `terraces` + fluvial settle; `cliffs.rs` periodic passes deleted (tagged). Test: canyon head
  connectivity, channel network drains (priority-flood no pits). Owner arm. (~30-60 s; fluvial is the unknown: if
  over 60 s it runs on the changed region's basins only)
- **LF-A1/A2** (adapter, beside LF-1..3): the game's rock units + region landform table; intent masks (adapter spec).
- **LF-calm** calm-base step with arms 0 / 0.3 / 0.6. Test: s = 0 is bit-identical, anchors unchanged, no sea
  crossing. (~2-4 s extra)
- **LF-5** runtime: lip restore + geology strata/blocks/ridges/gully gating in `filter.rs` + `tiles.wgsl`, the
  geology grids in the fill. Test: GPU twin equality, `measure_lod_switch_error` printed (target p99 < 2 px), zero
  mean. Fill cost A/B untraced. Visual check LF-5. (no bake)
- **LF-6** materials: per-unit layer table, bands from `bed_at`, scree on talus (`sf_terrain_bevy`, after
  `wg-lodfix` lands). Test: `P_CLIFF` table encoding. Visual check LF-6.
- **LF-7** exports for rockgen and scatter (`features.json` reader API, `bed_at` shared module). Test: rockgen band
  finder fixture reads a scarp.

## As built

- **LF-0** (2026-11-22): `sf_worldgen::measure` `land_rows` + `region_labels`; `scrib-worldgen measure --base GUIDE`
  prints "Landforms baseline per region" (zone `all` + each region of `regions.png`: slope bands, >= 35 / < 10 deg,
  walkable, largest component cut at the region border, untouched + `|dh|` p50/p95 vs the base when the samples
  match). Face area per kind, feature counts, `b`, wet/dry and step times wait for the placement slices. Baseline:
  ledger 2026-11-22, report `target/worldgen/lf0/measure.md`.
- **LF-1** (2026-11-22): `sf_worldgen::geology` (`RockUnit`, `GeologyTable` TOML, `build`, `GeologyMap::bed_at`,
  `bed`, `scrib-worldgen geology`). Differences from "Data model": `datum` is **f32** (f16 steps 2 m at 2-4 km
  heights, which would step every bed boundary); `hardness.u8` is the unit hardness blended over `unit_blend_m`
  (the bed's own hardness comes from `bed_at`); a unit is a whole Voronoi cell drawn from the shares of the region
  holding the cell's centre; `Fixed`/`Radial` beds are anchored at each cell's centre on the low-passed surface (each
  cell a fault block), `Surface` follows the 2 km low-pass; bed boundaries `t(i + a(u_i - 1/2))` keep `bed_at` O(1)
  for the WGSL twin. No `geology/` reader yet (LF-2 builds in process). 10x island: build ~1.9 s.
- **LF-2** (2026-11-22): `sf_worldgen::placed` (`mod.rs` bake order + declared values + `place` / `run_place`,
  `scarp.rs` traces, en-echelon segments, profile, `routes.rs` repair), `scrib-worldgen place --base --geology
  [--recipe] [--set] --out` (full guide folder: base files copied, new `heights.f32` + `meta.json`, `geology/` with
  cliff/talus/lip/feature filled, `features.json` with `facing` (unit vector the face looks along) and `ramps`,
  README). Differences from the spec: traces follow the **outline of the harder unit** against any softer neighbour
  (pair-specific boundaries broke at every triple junction into < 2.4 km pieces); a fault is a **tilted block**
  (top falls back at `back_deg` 6 deg to the base, so the delta stays compact and the lip is the high edge), raise
  only (no new water by construction); `fan` is the talus wedge of each face profile (`fan_share` of the throw at
  `talus_deg`), written to the talus mask, not a feature of its own; profiles box-filtered analytically through the
  piecewise-quadratic antiderivative; along strike a smoothstep taper over the relay overlap (finite tip gradient,
  so relay ramps are walkable) instead of an elliptic profile; water gaps where D8 area (census routing) passes
  `gap_area_m2`; spacing rule = centre distance >= `spacing_ratio x (L + L') / 2`; face budget = estimated face width
  + prefilter footprint integrated along strike; routes run **last** (after settle) as a 0-1 BFS Prim-style join per
  region with straight graded ramps (core >= 3 samples, lengthened until the grade fits, `route.max_cross_m` /
  `route.max_ramp_m` caps, centre-line cut/fill <= `route.max_cut_m` 25 m, so no causeway crosses a ravine or a
  canyon); candidates whose mean water-gap factor is under `gap_reject` (a boundary along a river) are dropped; `settle.margin_m` default 60 m (spec 300: the base around faults stays untouched); repose
  of a face sample spanning more than a mean bed uses the unit's bed-averaged hardness. New declared values:
  `fan_share`, `back_deg`, `fault_share`, `segment_m`, `relay_overlap`, `relay_aspect`, `tip_min_m`,
  `trace_smooth_m`, `calm_slope_deg`, `calm_window_m`, `contrast_floor`, `gap_area_m2`, `gap_width_m`,
  `coast_guard_m`, `anchor_band_m`, `prefilter_samples`, `gap_reject`, `route.min_component_m2` / `max_cross_m` /
  `max_ramp_m` / `max_cut_m` / `shoulder_m` / `rounds`, `settle.iterations` / `rate`. Owner arm `target/worldgen/lf2-scarps` (game table
  `bevyrrowind/worldgen/geology.toml`): bake 6.9 s, 80 faults (82 segments, throw p50 21 m, max 47 m), 37 ramps,
  untouched 97.7 %, land >= 35 deg 33.3 -> 33.4 %, largest walkable 60.1 -> 60.5 % (ledger 2026-11-22). The base's
  own steepness dominates every number: walkability needs LF-calm, not ramps.
- **LF-2b** (2026-11-22, owner on LF-2: "thin blades of very vertical (and dented) terrain, long and not very high"):
  `placed/facets.rs`. **Morphology** (Basin-and-Range range fronts; V: Wallace 1978, J. Res. USGS 6: spur facets
  25-35 deg, fresh scarps 50-90 deg; S: Blair & McPherson 1994 fans; Hovius 1996 outlet spacing): a segment's face is
  `z = max(min(block, chutes), cones)`: chutes on a truncated power law of spacing (50-300 m), each a V (walls 40 deg)
  whose floor leaves the face at the mouth `z_a` and climbs back to the top behind the lip (headward retreat), so facets
  between chutes narrow upward to triangles; a debris cone (12 deg) at each mouth replaces the uniform talus (facets
  keep a 10 % talus); chute size = mean of its two spacings sets its depth (half-width at the lip 0.9 x size / 2) and
  cone apex (`fan_share x sqrt(size / 300 m)`). No closed-form box filter for the min/max: 5 x 5 midpoint taps over the
  2-sample footprint in (strike, dip); segments without chutes keep the LF-2 analytic profile. Face 45-75 deg linear in
  the upthrown unit's hardness (owner: dramatic; real facets degrade to 25-35); throw x (1 +- 0.35 noise, 1.2 km) x (1
  +- 0.3 per segment); outer tips taper over 20 % of the length. **Throws:** per-region `throw_ratio` / `size_min_m` /
  `size_max_m` knobs, adapter table `bevyrrowind/worldgen/landforms.toml`. Spacing rejections are same-trace overlaps
  (each trace draws `ceil(length / size_min)` candidates); the face budget never binds (spacing limits density).
  **Runtime:** filter `steep_fade` multiplies the gully amplitude by `1 - smoothstep(tan 40, tan 50, |grad h0|)`
  (`filter.rs`, `tiles.wgsl` slots 68/69), a per-world value read by `GuideTiles::open` from `filter.toml` (only
  `steep_fade*` keys accepted; absent = off, `x10s` unchanged bit for bit); `scrib-worldgen place --world-filter`.
  Arm `target/worldgen/lf2b-scarps`: bake 6.2 s, 32 faults, 674 chutes, throw p50/p90/max 33/91/143 m, land >= 35 deg
  33.6 % (x10s 33.3), largest walkable 60.5 % (60.1), untouched 96.7 % (ledger 2026-11-22). LF-2 stays reproducible
  with `--set eroded=0 throw_noise=0 segment_throw_var=0 tip_ratio=0 face_soft_deg=70 face_hard_deg=70` and no recipe.

## Open items

1. Guide resolution: features under ~2 samples (37 m) are runtime (lip restore, strata) or rockgen. Small-cliff
   (5-30 m) density is a tunable iterated with the owner (decision 9); if the look loops want more than the runtime
   path can carry, a 9.144 m landforms guide costs 4× memory (decide at LF-5 from the measured LOD error and fill cost).
2. An adapter cliff mask made of radial footprints has no direction; orienting faces needs the rock's yaw and height from the
   adapter (LF-A2).
3. Fluvial settle time on 10 M samples is unmeasured (LF-4).
4. Which operators each region gets and how much is the owner's look call per arm; the defaults come from the
   reference DEMs.
