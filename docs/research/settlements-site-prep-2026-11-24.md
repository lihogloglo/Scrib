# Settlements: site preparation of the Odai reach (SP-R research for SP-1), 2026-11-24

Research only, no code run in the repo, no build, no game. Marks: **V** verified (file:line, URL, or my own measurement script on
`target/worldgen/sites/balmora-x10s/{site.json,heights.f32,water.u8}`, run with `python -I` from the scratchpad), **U** unverified or my judgement.
Owner decision (2026-11-24, `scrib-worldgen/docs/specs/settlements.md` "Owner answers" 2): reshape the Odai reach into a wider terraced
valley in the generated terrain, then plan on true heights (`relief_scale` 1.0); SP-1 is the Scrib slice that writes it.

## 0. Findings that change the brief

1. **The reach is not a terraced gorge; it is a flat plain with an inner trench.** South of the site centre both banks are a plain at
   +23..+28 m above the water with slope 0.00-0.07 for 150-250 m, cut by a 140 m wide, 25-40 m deep channel whose wall is the only steep
   ground (V, section 1). The cliff-and-terrace ground sits north of the centre (rows 640-760, east bank hills 57-119 m). So SP-1 is
   mostly **a wall-softening operation** (stair the 25 m wall down to the water) plus a harder northern part.
2. **The channel is 140-165 m wide at the sea-level mask** (V, section 1) while Magna Urbis's own crossings stop at about 21 m and the founding
   road bridges "the narrowest water" (`settlements.md:40-42`). Terracing the banks does not solve the crossing. See look question 2.
3. **Water exists only below sea level in generated worlds** (one plane at the guide's sea level: `settlements-m2-scrib-side-2026-11-24.md`
   section 2, "Water: V"). The channel bed here is -12 .. -40 m, the banks +0 .. +25 m: the water mask is simply `h <= sea`. So "channel kept"
   means "stamp weight 0 where the guide is wet", and the lowest tread (quay) sits just above sea level.
4. The stamp mechanism already does what SP-1 needs: `mix(h_gen, h_stamp, w)`, runtime detail scaled by `1 - w`, `w >= 1` returns the stamp
   exactly (`sf_worldgen/src/tiles.rs:411-431`). No new runtime code; SP-1 is a producer of `stamps.json/.f32`.

## 1. The reach as measured (Q1)

Site frame: 1641 x 1641 samples at 1.8288 m (3.0 km square), centre sample (820, 820) = `centre_m` (V, `site.json`); row 0 north. Water mask
= one connected component (V). The channel runs from north-east to south-south-west; the stations are 90 samples (165 m) apart, section
perpendicular to the local channel direction, bilinear, 5 m steps, distances from the wet edge.

| Station (m from centre along the river) | wet width | bed min | west bank | east bank |
|---|---|---|---|---|
| row 640 (-330, north) | 145 m | -11.9 | wall to +26 m in 105 m (0.26-0.57), tread +31 at 105-180 m, then 0.6-0.9 up; **13 % under 0.12** within 300 m | +14 at 30 m, +53 at 80 m (slope 1.0), shelf +74 at 105-155 m; **33 %** |
| row 730 (-165) | 165 m | -27.8 | wall to +30 m in 80 m, then plain +30..+32 at 80-210 m (0.00-0.06); **48 %** | +16, +29, +38, +57 (0.28-0.74): hill; **7 %** |
| row 820 (centre) | 140 m | -40.1 | wall +25 at 30 m, plain +24..+34 (0.01-0.17) to 255 m; **65 %** | wall +23 at 55 m, plain +26..+31 (0.01-0.09) to 280 m; **70 %** |
| row 910 (+165) | 140 m | -29.0 | plain +23 for 105 m, then +26..+62 rising (0.15-0.77); **37 %** | wall +24 at 55 m, plain +26..+29 (0.02) to 280 m; **80 %** |
| row 1000 (+330, south) | 145 m | -28.3 | plain +24 for 105 m, then +29..+73; **40 %** | wall +25 at 55 m, plain +26..+30 (0.01-0.02); **88 %** |

Share = dry samples within 300 m of the channel edge whose central-difference slope (1.83 m) is under 0.12 (V, my script, along each
perpendicular).

Whole-box figures (dry ground within 300 m of the water, box +-600 m of the centre, 77.8 ha; V):
- median slope **0.35**; **34 %** under 0.12, 38 % under 0.18, 72 % under 0.65. North half (rows 492-820): median 0.53, **13.5 %** under 0.12;
  south half: median 0.056, **56.5 %**.
- by height above water: 0-20 m: 10 ha, essentially no ground under 0.12; **+20..+35 m: 38.7 ha, 65 % under 0.12** (the plain); above +35 m:
  29 ha, 4-8 % under 0.12.
- these differ from the owner's figures (median 0.21-0.22, ~40 %): window and slope stencil differ (U: which exact box and stencil the
  planner used). Both say the same thing: the usable ground is the +20..+35 m plain, the gorge wall is not.
- **wall horizontal width** from the wet edge to +18 m: median ~29 m, range 9-80 m over rows 640-1000 (V, 38 stations): a wall slope of about 0.5-1.3, 30-50 degrees.
- the plain rises 0.02 downstream-to-upstream (+23 at the south to +32 at row 730 on the west bank): treads must follow the river gradient.
- old stamps in the site box (V, `target/worldgen/x10s/stamps.json`): only "Balmora" (stamp 2, x -2449..-1893, y -2194..-1638; the site was
  exported without it) and "Balmora 2" (stamp 188, 500 m east). S2 zeroes both, so SP-1 may overlap them freely.

## 2. Natural-looking target forms (Q2)

**Fluvial terraces (V: [Wikipedia, Fluvial terrace](https://en.wikipedia.org/wiki/Fluvial_terrace) and the literature it summarises).** A
terrace is a **tread** (flat surface) separated from the next by a **riser** (scarp). Fill terraces are the top of old valley fill, strath
terraces are bedrock benches with a thin alluvial cap. **Paired** terraces have matching heights on both banks (down-cutting faster than the
channel migrates); **unpaired** have different levels (lateral migration faster than incision, resistant layers, or uplift). Treads dip
gently toward the channel and risers are the cutbank scarps. Typical tread widths and riser slopes: no sourced numbers found (U); I take
treads of tens to a few hundred metres and risers at the angle of repose to bedrock steepness as ranges, to be tuned by eye.

**Design consequence:** use *unpaired* terraces (independent levels and widths per bank, variation along the valley) because paired ones
read as a stair of identical steps; keep a tread cross-fall of 1-3 % toward the river (U, drainage and natural look; also keeps treads far
under the 0.12 street grade).

**Human terracing (V where cited).**
- Amalfi/Cinque-Terre style: dry-stone walls (*macere*) of **3 to 7 m** between groves, built from site stone, thickness decreasing upward,
  drainage behind, with stairs as a core element ([FAO](https://www.fao.org/newsroom/story/the-amalfi-lemon-and-its-layered-resilient-landscape/en),
  [CNN](https://edition.cnn.com/travel/article/climate-change-lemon-farming-amalfi/)). Tread widths not found (U).
- Hill towns: builders followed existing contours, streets curve and climb by stairs, buildings stand at different levels on retaining walls
  ([architectureandtradition](https://www.architectureandtradition.com/p/what-made-the-italian-hill-town-so), [Wikipedia, Alley](https://en.wikipedia.org/wiki/Alley):
  on steep ground alleys can be partly or entirely steps).
- Ronda: gorge about 100-120 m deep and 50-70 m wide, houses on the rim (V, [Ronda Today](https://rondatoday.com/puente-nuevo-and-el-tajo-gorge/),
  [showcaves](https://www.showcaves.com/english/es/gorges/ElTajo.html)): the town sits on top and bridges across; it does not terrace the
  gorge. Cuenca, Toledo (Tagus), Mdina, Petra: not looked up (U); from memory they likewise build along the rim and on natural benches, with
  rock-cut and masonry walls at the rim, not a regular stair down the wall.
- Morrowind flavour (U, memory): Balmora is a Hlaalu town split by the Odai with stone bridges and houses stepping down to the water; Vivec
  has canal-edged cantons. The kit has `ex_hlaalu_wall_*`, `canal_*`, `buttress_*` pieces and stairs (V, `settlements-m2-scrib-side-2026-11-24.md`
  section 4 as cited in `settlements.md` Milestone 2 plan). So **risers as masonry quay/retaining walls (mesh) with a stair, and the quay
  as a canal-like stone edge, read as Hlaalu**; bare earth steps read as generic farmland terraces.

**What the data says about reading as Morrowind/Hlaalu vs generic (U, judgement):** a few broad treads with tall (4-6 m) retaining faces and
stairs reads as an Italian hill town / canal quay; many narrow earth steps reads as rice paddy. Prefer **3-5 treads of 20-40 m, risers 4-6 m**.

## 3. How games and generators prepare town sites (Q3)

Canonical technique (V across the sources): **a blended height raster: target heights under the town footprint with a weight that falls
to zero over a margin (smoothstep/cosine), cut and fill capped, applied before the structures are placed, and structure-aware where the
engine supports it.** Specifics:
- **Landscape splines (UE):** raise/lower terrain along a spline with per-point width and a **cosine-blended side falloff** and an end
  falloff, written into the heightmap ([UE 4.27 docs](https://docs.unrealengine.com/4.27/en-US/BuildingWorlds/Landscape/Editing/Splines);
  the UE5.7 removal claim in a blog is unverified).
- **CityEngine:** "Align terrain to shapes" reshapes terrain around streets and footprints; "Align graph/shapes to terrain" is the inverse;
  lots follow the terrain or sit at the lowest covered point; tutorial 14b grades around houses at a slope of 1:3
  ([Align streets to terrain](https://doc.arcgis.com/en/cityengine/latest/help/help-align-streets-terrain.htm),
  [Blocks](https://doc.arcgis.com/en/cityengine/latest/help/help-layers-block-parameters.htm)). So both directions exist; we choose
  terrain to plan, which is exactly our owner decision.
- **Minecraft jigsaw structures:** `terrain_adaptation` modes `beard_thin` (villages: add terrain under the structure, carve its inside),
  `beard_box`, `bury`, `encapsulate`; road pieces use `terrain_matching` projection, houses `rigid`
  ([Jigsaw structure](https://minecraft.wiki/w/Jigsaw_structure), [Template pool](https://minecraft.wiki/w/Template_pool)). Structure-aware
  density modification, not a pre-graded valley.
- **Valheim hoe / Enshrouded rake:** player tools that level to the standing height; not a generator (V, wiki/guides in search results).
- **Daggerfall-style generators:** blend weight 0 at terrain bounds, 1 at the location, smooth between (V, search result; weaker source).
- **Cityforge (Procedural Tamriel):** `flatten_pad` requests, road grade max 0.125, blend margins 768-2048 game units, cut/fill caps 1024 GU
  (V, `settlements-industry-2026-11-23.md:37-46` and `settlements-m2-scrib-side...md` Procedural Tamriel paragraph).
- Not found or not checked (U): No Man's Sky settlements, Dwarf Fortress site generation, Horizon/Decima, UE5 PCG landscape modification
  (only the spline tool above).

None of these reshapes a valley before planning as one named operation; it is *terrain grading as a weighted height stamp with a blend
margin and caps*, applied at a larger scale. Hill-town practice (section 2) is the form reference.

## 4. Proposed SP-1 profile (Q4): generic `valley_terrace`

Framework-side (`sf_worldgen`, no Morrowind names): a deterministic operator that reads the guide heights, sea level and a town extent and
writes one stamp (`stamps.json` + `stamps.f32`, format v1, `stamps.rs:1-30`). The game-side tool (`bevyrrowind worldgen-derive`, S2) supplies
the extent (from `site.json` `centre_m` and a radius) and appends the stamp; no anchor entry needed (`tiles.rs:255-257`).

**Inputs.** (a) guide heights (bicubic `base.height`, as the runtime sees it); (b) channel = wet component `h <= sea_level` that crosses the
extent (the same mask `site.json`'s `water.u8` is exported from); (c) extent: centre, radius `R_reach_m`. Everything derives from the
channel's distance field `d` (signed, 0 at the wet edge, + dry, computed by an exact EDT on a 1 m grid); no centreline or side label
needed.

**Fields (all functions of `d` and world position):**
1. **Bank reference height `P(x)`**: mean of guide height over dry samples with `d` in `[d_ref_lo, d_ref_hi]` inside a window of radius
   `R_P` (default 90 m, so the two banks, 280+ m apart, never mix), heights clamped to `sea + P_cap_m`. It follows the river gradient and
   differs per bank automatically (unpaired for free).
2. **Tread count per point** `n = clamp(round((P - q) / riser_h), 1, n_max)`, `q = sea + quay_freeboard_m`; heights `z_i = q + i (P - q)/n`
   (`n` changes only across the low-frequency noise below, never per sample; to keep it continuous, blend `z_i` by the noise rather than
   switching counts: define n per **along-valley cell** of `cell_len_m` and cross-fade neighbouring cells over `cell_len_m/4`).
3. **Unpaired and varied**: `riser_h` and tread widths are modulated by a 2D value noise at world (x, y) with wavelength `vary_wavelength_m`,
   amplitude `vary_amp` (width +-25 %, riser +-20 %); the riser line is offset by a second noise of `meander_m` at `meander_wavelength_m`
   so risers are not parallel to the channel (cutbank bends and spurs).
4. **Profile `T(d)`** from the channel outwards: quay tread at `q` of width `quay_w_m`; then for i = 1..n: riser (rise `z_i - z_{i-1}`, run =
   rise / `riser_slope`), tread of width `tread_w_m` with cross-fall `tread_fall` toward the channel (tread height `z_i + tread_fall *
   (d_end - d)` so it stays monotone); the last riser meets `P`; beyond `D_total` the target is the untouched guide.
5. **Weight `w(d)`** = 1 on the profile (`d` from `-0` to `D_total`) except **0 inside the channel** (`d <= 0`, keep bed and water), smootherstep
   (as the existing stamps do, `settlements.md` step 4) down to 0 over `blend_m` beyond `D_total` and along the extent edge; extent edge
   weight is tapered by the same distance. `blend_m >= max cut / tan(talus)` (cityforge rule, PL-4 in the plan).
6. **Cap:** where the guide at the profile's top is higher than `P + cut_cap_m` (hills, north-east bank), the top tread is
   `min(P, ...)` and the last riser becomes a natural bank at `talus_slope` that blends into the hill; no cut deeper than `cut_cap_m`.
7. **Output**: heights `h_s = T(d)` (final metres), weights `w`, on a `spacing_m` grid that covers the extent plus `blend_m`.

**How it is written and composes.** One stamp named e.g. `valley`, spacing 1.0 m (the same grid as the later town stamp, so the merge below is
exact; U: the 1 m choice vs 1.8288 m native costs nothing at this size, ~550 x 300 samples for a 1 km reach x 540 m width, far under the 2^28 limit,
`stamps.rs:100-118`). Order in the derived guide: guide + SP-1 stamp -> **ST-2 exports the site from this derived guide** (the exporter then sees
the new heights; V that the site is produced from a guide, `site.json` "source") -> plan at `relief_scale` 1.0 -> PL-4 grades pads and
streets on top of those heights. Because stamps may not overlap with weight > 0 (`stamps.rs:~21`), S2 merges SP-1 and the town stamp into one
exactly-composing stamp on the common grid (algebra, V by derivation): `w_m = 1 - (1 - w_t)(1 - w_s)`,
`h_m = (w_t h_t + (1 - w_t) w_s h_s) / w_m` (0 where `w_m = 0`); this equals `mix(mix(h_gen, h_s, w_s), h_t, w_t)` and the detail scale
`(1 - w_m)`. The town stamp's `h_t` is planned on the SP-1 heights, so its blend margin only does pads and streets. The runtime stays unchanged.

**Tunables (CONTRIBUTING "No hardcoded tunables": unit, range, source; `Guess` is allowed and counted).**

| name | unit | default | range | source |
|---|---|---|---|---|
| `quay_freeboard_m` | m | 2.5 | 1-4 | Guess (flood margin; no flood sim, no water above sea) |
| `quay_w_m` | m | 8 | 4-16 | Guess; Hlaalu canal/quay edge (U) |
| `riser_h_m` | m | 5 | 3-7 | Amalfi dry-stone walls 3-7 m (FAO) |
| `n_max` | count | 5 | 2-8 | Guess |
| `tread_w_m` | m | 28 | 16-50 | medina lot depth 8-16 m + street, one row + lane; two rows + street ~38 (kit depths 8-16 m, Scrib note section 4) |
| `tread_fall` | m/m | 0.02 | 0.01-0.04 | Guess; far under street grade 0.12 (`settlements.md:30`) |
| `riser_slope` | m/m | 1.0 (45 degrees) | 0.7-2.0 | earthen bank hidden behind a wall mesh; heightfield sample 0.5 m (`tile.rs:104-109`) can't hold a vertical face |
| `walled_riser_share` | 0-1 | 0.6 | 0-1 | share of riser length flagged "wall" in `valley.json` for a later mesh slice |
| `talus_slope` | m/m | 0.6 | 0.4-0.8 | thermal relaxation talus (`thermal.rs`), Guess |
| `cut_cap_m` | m | 12 | 6-25 | Guess; cityforge caps (V earlier note) |
| `blend_m` | m | 40 | 20-120 | cut / tan(talus) (cityforge, PL-4 plan) |
| `R_P_m` | m | 90 | 50-150 | Guess; less than half the 140 m channel + banks |
| `d_ref_lo/hi_m` | m | 90 / 160 | | measured: the plain starts 30-110 m from the edge (section 1) |
| `P_cap_m` | m | 40 | 20-80 | Guess |
| `vary_wavelength_m` / `vary_amp` | m / - | 250 / 0.25 | 100-600 / 0-0.5 | Guess (unpaired terraces, section 2) |
| `meander_m` / `meander_wavelength_m` | m | 6 / 70 | 0-15 / 30-200 | Guess |
| `cell_len_m` | m | 120 | 60-300 | Guess |
| `spacing_m` | m | 1.0 | 0.5-2 | matches the town stamp grid |
| `R_reach_m` | m | 500 | | from `site.json` (town extent) |

**Expected cut and fill on the measured reach (V arithmetic on the real heights; rough model: `P` per row from the 90-160 m band, 45-degree
risers, whole stair cut or filled to the profile, smoothstep blend 40 m; U: not the final operator).**

| profile (tread / riser) | reach | cut | fill | area | mean cut | share under 0.12 within 300 m (before -> after) |
|---|---|---|---|---|---|---|
| tread 10 m, riser 5 m | rows 760-1040 (510 m long, both banks) | 0.60 M m3 | 0.03 M m3 | 21 ha | 2.8 m | not run |
| tread 14, riser 8 | same | 0.58 M | 0.04 M | 21 ha | 2.8 | not run |
| tread 20, riser 6 | same | 1.19 M | 0.02 M | 26 ha | 4.6 | 0.50 -> 0.54 (median slope 0.12 -> 0.075) |
| tread 30, riser 5 | same | 2.92 M | 0.004 M | 34 ha | 8.5 | 0.50 -> 0.60 (median 0.045) |
| tread 24, riser 4 | same | 2.71 M | 0.004 M | 33 ha | 8.1 | 0.50 -> 0.59 |

The south reach's Magna Urbis-visible gain is small (50 % -> 54-60 % under 0.12) because the plain is already flat; the change is
that **the stair gives the town contiguous flat ground down to the quay** instead of a 25 m wall, and puts the cut cost in the stair,
not in the town. The north reach (east bank hills at +57..+119 m, 13 % under 0.12) is where reshaping buys the large gain, and the cut is
not shown by the model above (`P` there is 68-83 m, the rule `cut_cap_m` stops it): U, needs the real operator. Everything about 1.2-3 M
m3 of cut per 500 m is **heightfield only**, free at runtime. Counted share of ground Magna Urbis can then route on is
(treads + plain beyond) / window: 54-60 % in the south, roughly the treads' area share plus today's 13 % in the north.

## 5. Tests and owner look (Q5)

Exact (headless, `sf_worldgen`, fixtures): 
1. **Weight 0 inside the channel and outside**: every sample with guide `h <= sea` has `w == 0`; every sample beyond `D_total + blend_m` or outside the extent has `w == 0`; stamp heights there are irrelevant (assert `h_s == h_gen` at `w == 0` after the merge).
2. **Monotone treads**: along any perpendicular from the channel edge, the target `T(d)` never decreases; each tread is within `tread_fall` of its level; tread levels `z_i` strictly rise by at most `riser_h * (1 + vary_amp)`.
3. **Slope limits**: on tread samples (`w = 1`, tread mask) central-difference slope <= `tread_fall + eps` and <= the street grade 0.12; riser samples <= `riser_slope + eps`; blend samples <= `max(talus_slope, guide slope)`.
4. **Cut cap**: `h_gen - h_s <= cut_cap_m` everywhere with `w == 1`; the report's cut/fill sums equal the sums recomputed from the file.
5. **Continuity**: no sample step between 1 m neighbours larger than the riser slope times spacing (+eps) inside the weighted area (no seams at the extent edge).
6. **CPU twin equals file**: sampling the stamp at every grid point returns the written value bit for bit (`stamps.rs:75` "a grid point returns its own values bit for bit"); `TerrainTiles` height at `w >= 1` equals the stamp (`tiles.rs:419`) and the GPU words (`gpu_words`) round-trip; determinism: same inputs, same blake3 of `stamps.f32`.
7. **Merge exactness**: `mix(mix(h, h_s, w_s), h_t, w_t) == mix(h, h_m, w_m)` within 1e-4 m on the common grid, `w_m` in 0..1.
8. **Planning gate**: ST-2 export from the derived guide at 1.0 relief reports the share of dry ground within 300 m under street grade 0.12; require >= a threshold (measure, then pin in the recipe; today 34 % within the +-600 m box).
Measurement tests (`#[ignore = "measurement: ..."]`, printed, not asserted): cut/fill m3 per bank, tread area share, riser length, wall share.

Owner look (a scene is a real location, `view`, registered in `visual-checks/<system>.md`, status "awaiting owner"): the reach seen from the
centre (camera at +40 m over the quay looking up the river, noon) and from the east tread (looking across at the west terraces); things to
check: do the terraces read as a river valley and not as stairs (variation, meander), is the riser steepness plausible, does the old cliff
beyond blend in without a seam, does the water edge keep its shape. The owner judges the look; agents do not (CONTRIBUTING).

## Recommended SP-1 design

1. New generic operator `valley_terrace` in `sf_worldgen` (module beside `placed/`, core Bevy-free), CLI `bevyrrowind worldgen-derive` calls it; output = one stamp (1 m grid) + `valley.json` report (cut/fill, tread/riser lengths, wall-flagged riser polylines for the later mesh slice).
2. Profile from the channel distance field: weight 0 in the water; quay at sea + 2.5 m; 3-5 unpaired treads of 20-40 m (cross-fall 2 %) with 4-6 m risers at 45 degrees (60 % flagged as walls); blend to the guide over >= 40 m; per-bank bank-top height from the guide's own plain (follows the river gradient); noise-driven width/riser/meander variation.
3. Start at the existing plain (rows south of the centre, +23..+28 m) and let the cut cap decide how far the north reach is reshaped; expect 1-3 M m3 of cut per 500 m of reach, heights only.
4. Order: derive guide + SP-1 stamp, export the site from it, plan at `relief_scale` 1.0, then PL-4 on top; merge stamps with the exact `over` formula in S2.
5. Every parameter in section 4 declared as a tunable (unit, range, source), `Guess` counted; census must pass.
6. Defer risers as meshes (the walls, quay stone, stairs) to PL-5; SP-1 only flags them.

## Look questions for the owner

1. **Quay: stone wall at the water or a natural shore?** The channel bed is 12-40 m below the water-line; a stone quay at +2.5 m (Hlaalu canal edge) vs a sloping bank down to the water changes what the river looks like from the street.
2. **The river is 140-165 m wide; Magna Urbis crosses about 21 m.** Keep the full channel and accept one large bridge (the kit has bridges, but its span is U), or narrow the channel in the reach (a stamp can fill the water bed? only down to sea level; a narrowing moves the water line, a different look)? Decide before planning, since the founding road depends on it.
3. **Riser look: bare earth banks or masonry walls with stairs?** SP-1 only flags them; PL-5 builds them. Say which reads right so the flag share and the riser slope are set.

## Not verified

- Tread widths and riser slopes of natural river terraces (no sourced numbers); Ronda/Cuenca/Toledo/Mdina/Petra terrace dimensions; No Man's Sky, Dwarf Fortress, Horizon/Decima site-prep behaviour; whether UE5.7 removed the landscape-spline deform buttons.
- Morrowind's own Balmora/Vivec terracing and Hlaalu wall/bridge dimensions (memory only; kit sizes come from the Scrib note, not re-read).
- The planner's exact box and stencil behind the owner's 0.21-0.22 / 40 %; Magna Urbis's route sampling resolution (my slopes use 1.83 m central differences).
- The cut/fill table is a simplified model (per-row `P`, stair built on `d` from the wet edge, north reach excluded), not the operator; the north-reach cut is unknown.
- Whether the guide's runtime detail (`filter::detail_offset_banded`) leaves treads flat enough: at `w >= 1` it is exactly the stamp (`tiles.rs:411`), but the blend skirt carries detail scaled by `1 - w`; whether a tree/groundcover pass reads the new heights was not checked beyond the terrain-provider claim in the Scrib note.
- That the water mask produced from the derived guide stays identical (channel kept) after the stamp: true by construction (`w = 0` where wet), to be asserted by test 1.
