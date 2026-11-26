# Climate fields for sf_worldgen (WG-E1R), 2026-11-26

Marks: V = read in the source this session, U = unverified, secondary or from memory.

## 1. Recommendation

One model, four stages, all on the heightfield grid, deterministic, no FFT.

**(a) Temperature**: linear lapse rate with optional latitude and sea-distance terms (canonical: the standard-atmosphere lapse, U).
`T = T_sea - Gamma * (vert_scale * max(h,0)) / 1000 + g_lat * (y - y0)`; continentality is off by default.

**(b) Precipitation**: upwind moisture-budget sweep (the mapgen4 family, V, but conservative and with a temperature-limited capacity). Process the grid column by column along the dominant wind axis (rows if |wy| > |wx|). The upwind point `x - l*w` (step l = distance to the previous column along the wind, `l = dx / max(|wx|,|wy|)` for unit w) falls between two cells of the previous column; take `q_up` by linear interpolation. All cells of one column are independent, so columns run in parallel and the result is identical on any thread count (each cell reads only the finished previous column). The first column takes `q = 0` (dry inflow) except over sea (see below). At exactly 45 degrees the two taps are still in the previous column, so no tie.
Per cell, with T from stage (a):

1. Capacity `C = C0 * exp(beta * (T - T_sea))` (Clausius-Clapeyron-like, about +7 %/K).
2. Source: over sea `q1 = q_up + (C - q_up)+ * (1 - exp(-l/L_evap))`; over land `q1 = q_up + eps_land * (C - q_up)+ * (1 - exp(-l/L_evap))`.
3. Condensation (uplift happens implicitly because T falls with h): `r_c = (q1 - C)+ * (1 - exp(-l/L_fall))`. Excess not yet fallen stays in `q` and rains further downwind, which gives the Smith-Barstad fallout lag.
4. Background: `r_b = (q1 - r_c) * (1 - exp(-l/L_bg))`.
5. `q_out = q1 - r_c - r_b`; `P = (r_c + r_b)/l_km` in mm/yr (q is a vapour flux in mm/yr * km, so rain per column step divided by step length gives mm/yr).

Lee side: air descending has higher `C`, `q` is already depleted, so nothing rains and land recycling refills only slowly: a rain shadow with no extra rule. Mass budget: `sum(r) = sum(sea source + land source) - outflow` at the last column, exact in f64 accumulation (use f32 grids, f64 sums in tests).

**(c) Moisture/wetness**: climatic aridity blended with a drainage term.
`PET = 58.93 * clamp(T,0,30)` mm/yr (Holdridge biotemperature PET, U), `AI = P / max(PET, 1)`, `m_clim = AI / (1 + AI)`; `m_drain = clamp(log10(A / A0) / log10(A1 / A0), 0, 1)` with A the flow accumulation area we already have; `M = clamp(m_clim + w_drain * m_drain * (1 - m_clim), 0, 1)`. `h_cold = T < 0` gives PET 0 and `m_clim` 1, which scatter rules should read together with temperature (frozen ground is not wet ground).

**(d) Classes**: Whittaker-style table lookup on (T, P): a data file with a 2-D class grid (default 32 x 32 bins, T -15..35 C, P 0..4500 mm/yr), classes named in the data (tundra, taiga, temperate forest, rainforest, grassland, shrubland, savanna, desert, ...). Hard class id per sample (u8) plus the raw T and P rasters, because Magna Urbis' forest field wants the continuous values. Per-region override acts on the rasters before classification and may also pin a class.

**Per-region override** (recipe fields, applied after stage b and before c and d): `T' = T*t_scale + t_add` or `t_pin`; `P' = P*p_scale + p_add` or `p_pin`. Override is not fed back into the sweep (decoupled and cheap); a region may pin the sweep input instead through `source_scale` (multiplies `C0` inside the region, so an ash land can be drier along the wind). Region borders: blend over `blend_m` metres with a separable box blur so classes do not cut in lines.

### Parameters

| Param | Unit | Range | Default | Source of default |
|---|---|---|---|---|
| Gamma lapse | K/km | 4..9.8 | 6.5 | standard atmosphere (U) |
| T_sea | C | -5..30 | 18 | Guess (Morrowind reads temperate-warm) |
| vert_scale | 1 | 0.5..4 | 1 (set so the volcano is about 2.5-3.5 km) | Guess; heights are compressed in the source game |
| g_lat | K/km | -0.05..0.05 | 0 | Guess (single island) |
| wind direction | deg from north, blowing toward | 0..360 | per world | owner/world data |
| C0 | mm/yr * km | 5e4..5e5 | 1.5e5 | Guess; calibrate so a 30 km windward slope gives about 2500-4000 mm/yr |
| beta | 1/K | 0.04..0.08 | 0.07 | Clausius-Clapeyron about 7 %/K (U) |
| L_evap | km | 20..200 | 60 | Guess |
| eps_land | 1 | 0..0.5 | 0.15 | Guess |
| L_fall | km | 2..20 | 6 | Smith-Barstad fallout time 1000 s at 10 m/s = 10 km (V: fastscape default fall_time 1000 s) |
| L_bg | km | 100..1000 | 400 | Guess |
| A0, A1 | m^2 | 1e4..1e8 | 1e5, 1e9 | Guess |
| w_drain | 1 | 0..1 | 0.5 | Guess |
| bins, T/P ranges | | | 32 x 32, see (d) | Guess |

Every `Guess` is counted in the tunables census.

### Cost on 6.4 M samples (estimates, U until measured)

Sweep about 20 flops + 3 exp per cell; tabulate `1-exp(-l/L)` once per run (l is constant per axis choice, so they are three scalars: no per-cell exp except `exp(beta*dT)`, a 1-D table over height). About 0.1-0.2 s single thread, below 0.05 s on 8 threads. Temperature, PET, classes: each a pass of about 10-30 ms. The whole stage is well under 1 s; the 10 s target leaves room for a final 3x3 or 5x5 smoothing pass on P.

### Tests that pin it

1. Lapse on a cone: `T(h)` exact to 1e-4 C at three heights; zero height gives `T_sea`; `vert_scale` scales linearly.
2. Ramp (sea then a linear ramp 0 to 3 km, wind onshore): windward P is higher than the same height on the lee side of a symmetric ridge; lee P at the ridge foot is below 30 % of the windward foot.
3. Flat land, no sea: P decays monotonically with distance from the source column (background term only).
4. Mass conservation: with `eps_land = 0`, `sum(r)*cell_area_term + outflow = source` to 1e-6 relative.
5. Wind axis symmetry: rotating the same field by 90 degrees and the wind by 90 degrees gives the same P (exact for axis-aligned winds); for 30 degrees a cone's lee centroid lies within one cell-row of the wind line.
6. Determinism: bit-identical output for 1, 2 and 8 threads (hash the raster).
7. Override: pin yields the pinned value inside the region; scale and add compose in the stated order; outside the blend the raster equals the unoverridden one.
8. Classifier: corner cases of the table (cold-dry, hot-dry, hot-wet) map to the expected class names from the data file.

## 2. Comparison

| Model | Lee shadow around one big volcano | Artefacts | Cost | Verdict |
|---|---|---|---|---|
| Lapse temperature (1) | n/a | none; needs a sensible vertical scale | trivial | adopt |
| Smith-Barstad FFT (2i), V code in fastscape-lem | good: full linear theory, lee evaporation, lateral spreading; Hergarten and Robl 2022 note it cannot carry moisture over long distances (U) | periodic wrap (the Python code zero-pads up to 200 cells, V); needs a padded power-of-two FFT (about 8.4 M complex values); parameters in atmospheric units (Nm, hw, tau) hard to tune by feel; no moisture budget; coast is ignored (no sea source), a flat plain gets only the uniform base rate | about 0.5-1 s | not chosen: no budget, no dependence on sea distance, harder per-region control |
| Moisture sweep (2ii), V mapgen4 `map.ts assignRainfall` | good: lee drying follows from depletion | striping along the wind on a grid (limit with 2-tap interpolation, which also diffuses laterally); angle sensitivity mild with the dominant-axis sweep | about 0.1 s | adopt |
| Upslope index with shadow ray (2iii) | crude: dry only where a ray hits a higher cell; no depletion by a long range | hard edges, ray-length tunable, noisy on rough terrain | about 0.2-0.5 s | not chosen |

The mapgen4 original: humidity enters at the boundary (=1.0) and over sea (`evaporation * -elevation`), cells are processed in order of projection on the wind vector, each takes the mean humidity of upwind neighbours, rain = `raininess * humidity`, and when humidity exceeds `1 - elevation` the excess times `rain_shadow` rains out (V). It has no temperature term and no budget in physical units; our version adds capacity from T and conserves mass.

## 3. What shipped tools do

- Veloren (GPL-3.0, read `world/src/sim/mod.rs` at HEAD 131d1193 of 2026-11-10; file last changed in 5abad4e6): V. No rain shadow. Temperature = Irwin-Hall CDF mix of a noise field, `1 - altitude_uniform` and a slow noise (weights `[1,2,1]`); humidity = mix of Billow noise, flux (river water flux) CDF and a constant, reduced where temperature exceeds a tropical threshold; biome by thresholds on temp and humidity (Snowland, Desert, Jungle, Savannah, Taiga, Forest, Grassland). Useful idea: flux contributes to humidity. Nothing to port for (b).
- mapgen4 (Red Blob Games, Apache-2.0, `map.ts`): V as above; biome lookup by elevation and moisture is on the blog (U).
- fastscape-lem/orographic-precipitation (MIT, `orographic_precipitation.py`, commit baf8a0c1): V. Defaults conv_time 1000 s, fall_time 1000 s, nm 0.01 1/s, hw 3400 m, lapse -4 and moist lapse -7 K/km.
- Landlab OrographicPrecipitator: U (not read; it is a Smith-Barstad variant).
- Gaea: V from release notes only: Erosion 2 has rain shadow controls, a Biome node classifies by altitude and water-map access, Snow nodes; no published climate model (U on internals). World Machine, Houdini heightfield: U, masks and erosion, no climate model that I know of.
- Horizon Zero Dawn (GDC 2017, van Muijden): V from summaries: artists paint ecotopes, placement is rule and density driven from that painted data. Far Cry 5 and Ghost Recon Wildlands: U (rule-driven placement with painted biome masks, from memory). UE5 PCG biome sample: U (volume and landscape-layer driven).
- Takeaway: shipped games paint or rule-derive biomes; nobody runs a climate sim. Our per-region override is the equivalent of the painted ecotope.

## 4. Crates

| Crate | Version | Licence | Last release | Verdict |
|---|---|---|---|---|
| rustfft | 6.4.1 | MIT OR Apache-2.0 | 2025-09-18 | not needed (no FFT); the fallback if the Smith-Barstad model is ever wanted, with realfft |
| realfft | 3.5.1 | MIT | 2026-10-26 | same |
| noise | 0.9.0 | MIT OR Apache-2.0 (U) | 2024-03-23 | not needed |
| biome | 0.0.0 | placeholder | 2023-01-12 | not fit |
| hxgm30-eco | 0.1.0 | U | 2020-04-26 | not fit: dead, tiny |
| use-biome / use-ecology | 0.1.0 | U | 2026-05-21 | vocabulary only, not fit |
| earths | 0.0.4 | U | 2026-04-08 | planet simulation, huge scope, not fit |

No crate for orographic precipitation or Whittaker classification found (crates.io searches V: "orographic", "climate biome").

## 5. Moisture index choice

| Index | Pro | Con |
|---|---|---|
| TWI `ln(a/tan beta)` (Beven-Kirkby, U) | canonical in hydrology and soil mapping | noisy at 18-55 m, undefined on flats, ignores climate |
| Precipitation-weighted flow accumulation | physically tied to P | dominated by trunk rivers, flat elsewhere |
| HAND | good riparian zone mask | needs a drainage-network pass we do not have |
| Chosen: AI (P/PET) plus log accumulation area | climate first, rivers second, uses data we have | two tunables |

Game scatter rules mostly read altitude, slope and distance-to-water or a painted mask (Horizon, V summary; Gaea Biome node, V); nobody found using TWI. Distance-to-river is the cheap equivalent and can be added later from the flow raster.

## 6. Open points for the owner

None technical. Taste: prevailing wind direction relative to the volcano (which coast is wet) and the intended temperature at sea level; both are recipe data.

## 7. Sources

- V https://github.com/veloren/veloren (world/src/sim/mod.rs, HEAD 131d1193)
- V https://github.com/redblobgames/mapgen4 (map.ts) and https://www.redblobgames.com/maps/mapgen4/ (page names the blog post only)
- V https://github.com/fastscape-lem/orographic-precipitation (orographic_precipitation.py, fastscape_ext.py)
- U Smith and Barstad 2004, J. Atmos. Sci. 61(12):1377-1391, doi 10.1175/1520-0469(2004)061<1377:altoop>2.0.co;2 (abstract via search only)
- U Hergarten and Robl 2022, GMD 15:2063, https://gmd.copernicus.org/articles/15/2063/2022/gmd-15-2063-2022.html
- V (summary) https://www.guerrilla-games.com/media/News/Files/GDC2017_VanMuijden_GPUBasedProceduralPlacementInHorizonZeroDawn.pdf
- V (summary) https://digitalproduction.com/2025/07/18/gaea-2-2-unveils-frosty-nodes-and-selective-erosion/ and https://docs.quadspinner.com/Reference/Color/Biome.html
- V crates.io API for rustfft, realfft, noise, biome
