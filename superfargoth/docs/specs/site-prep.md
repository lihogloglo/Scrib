# Site preparation: river-valley terraces (`sf_worldgen::valley_terrace`, SP)

Status: spec 2026-11-24 (st-m2), not built. Research: `docs/research/settlements-site-prep-2026-11-24.md` (measurements,
industry, cut/fill model). Consumer: the settlement tool's Milestone 2 (scrib-worldgen `docs/specs/settlements.md`
"Milestone 2 plan", owner answers 2 and 3).

## Goal

Before a town is planned beside a river in a generated world, reshape the river's banks into a terraced valley so the
planner (Magna Urbis: streets <= 0.12, lanes 0.18, stairs 0.65) gets contiguous buildable ground from the water's
edge up to the bank top, and the town is then planned on **true heights**. Generic: a guide, a sea level and a town
extent in, one terrain stamp out; no game names, units or ids.

## Owner decisions (2026-11-24)

- Reshape the valley before planning (rejected: cutting the plan into the gorge, keeping the cliffs).
- Keep the river at full width (crossed later by one long multi-arch bridge): the stamp never touches wet ground.
- A **stone quay** at the water (a wall mesh, later slice) with stairs down.
- **Risers: retaining walls with stairs inside the town, earth banks where the terraces run out past the buildings.**

## The site (measured, research §1)

First user: the 10x world's Odai at Balmora 2.0. South of the centre both banks are a plain at +23..+28 m over the
water (slope 0.00-0.07 for 150-250 m), cut by a 140-165 m wide trench (bed -12..-40 m) whose wall, ~29 m wide (9-80 m)
at slope 0.5-1.3, is the steep ground. North: cliffs and east-bank hills at +57..+119 m. Water exists only below the
guide's sea level, so the channel is `h <= sea`. Dry ground within 300 m of the water under 0.12: 34 % (south 56 %,
north 13 %).

## Reuse check and industry

Canonical technique: a **blended height stamp with falloff**, capped cut and fill, blend margin >= cut / tan(talus)
(UE5 landscape splines and Landscape Patch, CityEngine align-terrain, Procedural Tamriel's cityforge `flatten_pad`,
Minecraft's structure "beard"). Ours already exists: guide stamps (`sf_worldgen::stamps`, format v1:
`mix(h_gen, h_stamp, w)`, detail x (1 - w), `w >= 1` returns the stamp exactly, `tiles.rs:411-431`; CPU and GPU
twin), so SP adds a producer only, no runtime code. Form: unpaired river (fluvial) terraces cut into the plain, with
dry-stone risers of 3-7 m as on terraced coasts (Amalfi). No crate builds terraces.

## Design (operator `valley_terrace`, Bevy-free)

**Inputs:** guide heights (as the runtime samples them), sea level, town extent (centre, radius `reach_m`; from the
settlement site's `centre_m`), the town core radius for the wall/bank rule. Everything derives from the channel's
signed distance field `d` (0 at the wet edge, + on dry ground; exact EDT on the stamp grid); no centreline needed.

1. **Bank top `P(x)`:** mean guide height over dry samples with `d` in `[ref_lo_m, ref_hi_m]` within `ref_radius_m`
   (smaller than half the channel plus banks, so the banks never mix), capped at `sea + bank_cap_m`. Follows the
   river's gradient and differs per bank (unpaired for free).
2. **Treads:** quay level `q = sea + quay_freeboard_m`, quay tread `quay_w_m` wide; tread count
   `n = clamp(round((P - q) / riser_h_m), 1, treads_max)` decided per along-valley cell of `cell_len_m`, neighbouring
   cells cross-faded over a quarter cell (no count switch per sample); levels `z_i = q + i (P - q) / n`; tread width
   `tread_w_m` with cross-fall `tread_fall` toward the water (monotone).
3. **Variation:** tread width and riser height modulated by value noise (`vary_wavelength_m`, `vary_amp`), the riser
   lines offset by a second noise (`meander_m`, `meander_wavelength_m`), so risers bend like cut banks and spurs.
4. **Risers:** rise between levels over a run of rise / `riser_slope`. Inside the town core a riser is flagged
   **wall** (the heightfield holds a 45 degree bank behind where the wall mesh will stand: 0.5 m is the finest height
   sample); outside it is an **earth bank** at `bank_slope` (gentler, grass holds). The quay edge is flagged **quay**.
5. **Cut cap:** where the guide at the profile's top stands higher than `P + cut_cap_m` (the north hills), the top
   tread stops at `P` and the last riser becomes a natural bank at `talus_slope` meeting the hill; no cut deeper than
   `cut_cap_m`.
6. **Weight:** 1 on the profile, **0 on wet samples** (bed and water untouched), smootherstep to 0 over `blend_m`
   beyond the last riser and along the extent edge; `blend_m >= cut_cap_m / talus_slope` (checked at load).

**Output:** one stamp (heights in final metres + weights) on a `spacing_m` grid covering the extent plus `blend_m`,
and `valley.json`: tread/riser/quay polylines with their kind (wall, bank, quay), cut and fill m3 per bank, tread
area share, share of dry ground under the street grade before and after. The polylines feed the wall/quay/stairs
mesh slice (settlements PL-5); SP only flags.

**Composition:** stamps never overlap with weight > 0. The derive step (settlements S2) merges this stamp and the
later town grading stamp on one grid exactly: `w_m = 1 - (1 - w_t)(1 - w_s)`,
`h_m = (w_t h_t + (1 - w_t) w_s h_s) / w_m` (0 where `w_m = 0`), equal to `mix(mix(h, h_s, w_s), h_t, w_t)` with
detail x (1 - w_m). It refuses an extent whose blend touches another stamp's weight > 0.

## Tunables (declared per `tunables.md`, `Lifetime::Bake`; values from research §4)

`quay_freeboard_m` 2.5 m (1-4, Guess: no water above sea) · `quay_w_m` 8 m (4-16, Guess) · `riser_h_m` 5 m (3-7,
Amalfi dry-stone walls) · `treads_max` 5 (2-8, Guess) · `tread_w_m` 28 m (16-50: one row of 8-16 m kit depths plus a
street) · `tread_fall` 0.02 (0.01-0.04, Guess, far under 0.12) · `riser_slope` 1.0 (0.7-2.0: wall-backed) ·
`bank_slope` 0.6 (0.4-0.8, Guess) · `talus_slope` 0.6 (0.4-0.8, thermal talus) · `cut_cap_m` 12 m (6-25, Guess) ·
`blend_m` 40 m (20-120: cut / tan(talus)) · `ref_radius_m` 90 m (50-150) · `ref_lo_m`/`ref_hi_m` 90/160 m (measured
plain start) · `bank_cap_m` 40 m (20-80) · `vary_wavelength_m` 250 m, `vary_amp` 0.25 · `meander_m` 6 m,
`meander_wavelength_m` 70 m · `cell_len_m` 120 m · `spacing_m` 1.0 m (the town stamp's grid) · `reach_m` from the site ·
`core_radius_m` from the site (the town's planned extent).

## Tests (headless fixtures, exact)

1. Weight 0 on every wet sample and beyond the extent plus blend.
2. Along any perpendicular from the wet edge the target never decreases; tread levels rise by at most
   `riser_h_m (1 + vary_amp)`.
3. Slopes: tread samples <= `tread_fall` + eps (and <= 0.12); riser samples <= their kind's slope + eps; blend samples
   <= max(talus, guide slope).
4. Cut cap holds where `w = 1`; `valley.json` cut/fill equal sums recomputed from the stamp.
5. No step between neighbours steeper than the riser slope inside the weighted area (no seam at the extent edge).
6. The written stamp read back through `sf_worldgen::stamps` returns every grid value bit for bit; same input, same
   blake3.
7. Merge: `mix(mix(h, h_s, w_s), h_t, w_t) == mix(h, h_m, w_m)` within 1e-4 m.
8. Riser kinds: wall inside `core_radius_m`, bank outside, quay at the quay edge.
Measurement (`#[ignore = "measurement: ..."]`): on the real reach, cut/fill per bank and the share under 0.12 before
and after (today 34 %).

## As built (SP-1a, 2026-11-24)

`sf_worldgen/src/valley_terrace.rs` (+ `tests_valley_terrace.rs`, 11 tests): the operator, `merge_over`,
`ValleyReport`, `ValleyTerrace::write` (`stamps.json` + `stamps.f32` + `valley.json`). Departures from the design:
1. Tread count uses `ceil`, not `round` (a rounded count let a rise reach 1.5x `riser_h_m`); the rise is capped at
   `riser_h_m (1 + vary_amp)`.
2. The cut cap limits only the cut into ground above the top tread; the stair's own dig to the quay is deeper on a
   +25 m plain (to ~22 m). Where the cap applies, a talus cone from the capped ground forms the natural bank, never
   standing above the guide (a filling version added ~7 M m3).
3. The fade outside the extent is `max(blend_m, (bank_cap_m - quay_freeboard_m) / talus_slope)` (62.5 m at the
   defaults): 40 m left seams steeper than 1.0 where the stair's cut fades along the river.
4. Riser slope eases from wall to bank over `cell_len_m / 4` just inside the core (a hard switch moved the risers
   above by ~3 m in one sample).
5. Tread width and meander noise are sampled once per perpendicular (at its water foot); runs and falls are
   lengthened by `hypot(1, d(line)/d(edge))` so slopes hold on oblique riser lines. Open: feet spreading on the
   inside of tight bends (a 325 m inner disc passes).
6. Banks = 4-connected dry areas of the grid, labelled in scan order (the report gives centroids).
Test tolerances: treads along the perpendicular at `tread_fall` x 1.1, risers and test 5 at x 1.03 (noise
curvature), natural banks at `talus` x 1.0275 (the chamfer cone's maximum) + the top tread's slope.
Measured (arm B site, reach 600 m, core 300 m, 1 m grid 1327 x 1327, 0.54 s): cut 3.46 M m3 (west 2.13, east 1.33),
fill 0.04 M m3, treads 28 % of the area, dry ground under 0.12 within 300 m 36.6 % -> 47.0 %; 225 wall, 231 bank,
176 quay polylines.

## Ramps (SP-1c, owner 2026-11-24)

First plan on the terraced Odai at true heights (relief 1.0, arm B site re-exported from `x10s-b2v`): medina 4 lots,
european-organic 52, korean 340: Magna Urbis's streets (grade <= 0.12, stairs 0.65) cannot climb a 45 degree riser,
so the town stays on the quay tread by the bridge. Real terraced towns connect terraces with ramps and stair
streets. Owner (2026-11-24): **keep the terraces and cut ramps into the steps** (rejected: one gentle bank; plan
first then terrace). Design: along every riser, every `ramp_spacing_m` (Guess 100 m), a ramp `ramp_w_m` wide (Guess
6 m, a Magna Urbis street) running along the riser at `ramp_grade` (Guess 0.10, under the 0.12 street grade) from
the lower tread to the upper one (run = rise / grade, ~50 m for 5 m), cut into the upper tread's edge, directions
alternating between successive risers (switchbacks); ramps join both treads without a step. Ramp centrelines go
into `valley.json` (kind `ramp`) for the later wall/stairs meshes. Key test: from the quay tread, every tread and
the bank top are reachable over grid steps no steeper than the street grade.

**Stairs (SP-1d, owner remark 2026-11-24: "Balmora is already like that: terraces and stairs").** Vanilla Balmora
joins its levels with Hlaalu stair flights (kit: `ex_hlaalu_steps_03/04/05` flights 3.7 m wide, 5.3 / 9.0 / 12.6 m
long; `steps_06/07` 8 x 8 m landings and turns), and Magna Urbis routes stair streets up to grade 0.65. So most
terrace crossings are **stair corridors**: every `stair_spacing_m` (Guess 40 m) along each riser, a corridor
`stair_w_m` wide (3.7 m, the flight width) graded at `stair_grade` (Guess 0.5, under 0.65; to be set from the
vanilla flights' rise/run), reported as `stairs` lines in `valley.json`; the layout tool places the Hlaalu flights on
them (with the walls, PL-5). Ramps stay, sparser, for the main streets that Magna Urbis only routes at 0.12.

## Build plan

- **SP-1a** operator + tests 1-8 in `sf_worldgen` (core, Bevy-free), writing the stamp + `valley.json` to a folder.
- **SP-1b** the derive command (settlements S2) calls it for a named site and appends the stamp; then the site is
  re-exported (ST-2) from the derived guide and planned at `relief_scale` 1.0.
- Owner look: the reach in `view "world:<derived guide>"`, camera over the quay looking up the river at noon and from
  the east tread across: terraces read as a valley (not stairs), riser steepness, no seam at the old ground, the
  water edge unchanged (`bevyrrowind/docs/visual-checks/worldgen.md`).
