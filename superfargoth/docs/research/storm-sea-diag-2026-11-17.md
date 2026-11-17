# Why the storm sea is tame (diagnosis, 2026-11-17)

Code and maths only, no runs. Hs / Tp / lambda_p computed offline from the code's own JONSWAP
(`sf_water/src/spectrum.rs:79-139`: alpha = 0.076 (U²/Fg)^0.22, wp = 22 (g²/UF)^(1/3), Hs = 4 sqrt(m0), band
k >= 2π/256). Industry lines marked (recalled) are from memory, not re-read this session.

## 1. The chain, with numbers

- Weather: `Wind Speed` (ini.rs:64-83) -> `mean = 1 + 14·s` m/s at 10 m (`visual.rs:50-52,263-266`), speed wanders
  0.75-1.25 x mean. Clear .1 -> 2.4 m/s; Cloudy .2 -> 3.8; Rain .3 -> 5.2; **Thunderstorm .5 -> 8.0 (6-10)**; Ashstorm
  .8 -> 12.2; Blizzard .9 -> 13.6.
- Copy: `exterior.rs:1567` copies `WindField.velocity` verbatim; `SeaWind` only low-passes (tau 30 s) and blends
  (`sea_wind.rs:42-53`). No gain, no amplitude change. `SpectrumParams.amplitude` stays 1.0 (`mw_water/lib.rs:159`
  uses `..default()`); nothing in `sf_water_bevy` scales or clamps amplitude (grep `amplitude`: shore waves only).
- Fetch: `SEA_FETCH_M` = 20 km, fixed, never weather-dependent (`mw_water/lib.rs:20`).

Sea state at 20 km fetch (what the player gets) vs 100 km and 1000 km:

| U 10 m (m/s) | case | Hs 20 km | Tp / lambda_p | Hs 100 km | Hs 1000 km |
|---|---|---|---|---|---|
| 2.4 | clear | 0.25 m | 2.3 s / 8 m | 0.60 | 2.2 |
| 5.2 | rain | 0.49 | 2.9 s / 13 m | 1.20 | 4.2 |
| **8.0** | **thunderstorm** | **0.72** | **3.4 s / 18 m** | 1.75 | 5.4 |
| 15 | | 1.25 | 4.2 s / 27 m | 3.06 | 6.0 |
| **30** | `--water-wind 30` | **2.3** | 5.3 s / 43 m | 5.65 | 7.6 |

Clear to thunderstorm is only 0.25 -> 0.72 m (x2.9, steepness 0.04): a storm reads as a slightly bigger breeze.
`--water-wind 30` (Hs 2.3 m, lambda 43 m) is the look the owner calls nice; it is also an unphysical 30 m/s
(Beaufort 11), which shows the fix needs an exaggerated wind, not just a correct one.

## 2. Binding limits

1. **The weather wind map (primary).** 8 m/s is Beaufort 4-5 (moderate breeze); a real thunderstorm is 15-25 m/s
   in gusts, and Morrowind's own ini scale is a ground-wind scale for foliage/particles, not a sea-state scale.
2. **The 20 km fetch (co-binding).** Even at 15 m/s it caps Hs at 1.25 m, and the peak wavelength at 27 m. Raising
   wind alone saturates: Hs grows only ~U^1.1 at fixed F.
3. Cascades do **not** bind: 256 m tile vs lambda_p <= 61 m for every case worth using (4+ waves per tile; the
   cascade 0/1 band boundary is wavelength 32 m, so lambda 40-60 m sits in cascade 0). Full-k Hs equals the cascade-limited Hs to 2 decimals up to 15 m/s
   at 20-100 km. Only >= 1000 km fetch (lambda 280-585 m) would clip.
4. No scale or clamp: amplitude 1.0, none in shader. (A 30 s low-pass adds delay, not a cap.)

## 3. Industry (recalled)

- Shipped games do not map real wind to waves; they author a sea-state curve per weather and exaggerate it: Sea of
  Thieves (Rare, GDC 2018 "Sea of Thieves: Water technology") drives wave amplitude/length from the storm state, far
  above physical for the wind; AC IV (Ubisoft) FFT ocean with a per-weather Beaufort-like preset; RDR2/Decima use a
  weather-keyed sea parameter set. Crest (open source) exposes `Wind speed` and `Wind turbulence` directly, with
  documentation advising raising wind and `Fetch` for storms; UE5 Water uses per-wave-source amplitude art-directed
  by designers/weather. Common thread: sea state is its own authored parameter, decoupled from rain/foliage wind.
- Physics standard for a storm: fully developed sea (Pierson-Moskowitz, Hs ~ 0.21 U²/g: 8 m/s 1.4 m, 15 m/s 4.8 m)
  = JONSWAP with unlimited fetch. Distant-storm swell (long, low, from another direction) is the extra layer; it
  needs a second spectrum (parked, see below).
- Verify the citations before they go into the spec (Reuse/Industry sections require checked sources).

## 4. Recommendation: one slice, "sea state curve" (W-SEASTATE)

**What.** A tunable piecewise-linear map from the weather's 10 m wind u to the sea's spectrum wind, plus a larger
fixed fetch. Keep it in the Morrowind adapter (the framework must not know weather conventions):

- `mw_water`: `SEA_FETCH_M` 20 km -> tunable `water.sea_fetch_m`, default **50 km** (Guess, from the table: lets
  16 m/s reach Hs 2.2 m, lambda 52 m, Tp 5.8 s).
- `mw_water` (new fn `sea_wind_for(u: f32) -> f32`) with tunable points `water.sea_state_curve` default
  (u -> U_sea m/s): (0, 0), (2.4, 2.6), (3.8, 4.5), (5.2, 7.0), (8.0, 16.0), (12.0, 22.0), (16.0, 26.0), clamp above.
  Source: Beaufort-class exaggeration; target thunderstorm Hs ~2.2 m, lambda 52 m, near the `--water-wind 30`
  look at 20 km (2.3 m, 43 m); clear stays ~0.5 m, rain ~1.0 m. Cap 26 m/s keeps Hs <= ~3 m at 50 km.
- Apply in `exterior.rs:sea_wind` (1570-1576): `sea.weather = Some(v.normalize() * sea_wind_for(v.length()))`,
  before the existing low-pass/transition. `--water-wind` keeps overriding everything.
- A/B switch: `--off seastate` = identity map and 20 km fetch (today's behaviour); `--set water.sea_state_curve=...`.
- Side effects to list in the visual-check: shore waves/surf amplitude and the CPU twin follow (same params), so
  boats and breakers grow with the storm; whitecap/foam coverage rises; clear/default 7 m/s sea also grows
  (0.8 -> ~1.0 m) because of the 50 km fetch (look-pending, owner).

**Test.** `mw_water` unit tests: curve monotone, identity off, endpoints exact, u=8 gives sea wind 16;
`sf_water` realized-variance test at (16 m/s, 50 km): Hs within 5 % of 2.2 m and lambda_p < 256/3 m (cascade
holds it), `omega_peak` assertion; weather -> sea integration: thunderstorm frame produces sea wind >= 14 after the
low-pass. Sanity only on the look (CONTRIBUTING: owner signs off; register an awaiting-owner visual check
"thunderstorm at sea, camera 30 m offshore, `--weather thunderstorm`" comparing `--off seastate`).

**Parked (not in this slice).** Distant-storm swell as a second low-amplitude spectrum layer from an authored
direction; weather-dependent fetch (needs `SeaWind` to carry fetch); fully developed PM mode.
