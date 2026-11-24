# Visual checks: groundcover

Spec `superfargoth/docs/specs/groundcover.md`; adapter `bevyrrowind/docs/specs/groundcover-adapter.md`; ledger
`docs/ledger/groundcover.md`.

### GC-2: vanilla grass and ferns scattered from the land textures (2026-11-22, groundcover-research)

Status: owner 2026-11-23, first look (`gc-ascadian`): 10 fps in the first run (a build from another session was likely running;
a rerun read 53-55 fps over BRP, ~60 with `--off groundcover`), too sparse, barely sways even in ashstorm, most clumps half
buried, looks like one species. Fixed: per-species `z_offset_m` from vanilla placements (burial), grass density x2.5,
`groundcover.render.bend_per_mps` 0.05 -> 0.15; awaiting owner on the rebuild. Open: frame cost unmeasured (A/B), steady
flutter, variety (only `flora_grass_01-04` in the Ascadian Isles: GC-3 blades or mod models).
Was: awaiting owner. Scenes `gc-ascadian` (cell 2,-7, 08:00), `gc-grazelands` (11,8, 14:00), `gc-bittercoast`
(-3,-8, 10:00), all at eye height; A/B `--off groundcover`. Models: vanilla `flora_grass_01-07`, `flora_bc_grass_*`,
`flora_bc_fern_*` with the MVR PBR textures (first mesh of each model, no normal/ORM maps yet). Every density, spacing and
range is a `Guess` (`bevyrrowind/data/groundcover.ron`). Log line: `groundcover: species S, channels matched M/N,
unmatched [..], models loaded L/13`.
Look for: plants only on grass land textures (none on roads, rock, mud, cobbles, under water), standing on the ground
(not floating or sunk), no hard edge where the ring ends, sway without popping, no tiles visibly appearing as you walk.
Known: Bitter Coast ground is mostly `Tx_BC_mud` (no species yet, so sparse there); no shadows past the near range.

### GC-M2: mesh plants sway, flutter and show gust fronts (2026-11-23, groundcover)

Status: awaiting owner (with owner look 2). Scenes `gc-ascadian`, `gc-grazelands`; try `--weather` clear and a storm.
Look for: each clump swaying gently out of step with its neighbours, a faster shiver at the tips, gust fronts pushing
visibly harder than the lulls between them, no motion in still air, no smearing or ghosting on moving plants (motion
vectors), no jump once an hour (time wrap). Live tunables `groundcover.render.gust_contrast`, `sway_amount`, `sway_hz`,
`flutter_amount`, `flutter_hz` (all `Guess`); A/B by setting the amounts to 0.

### GC-3: grass blades carpet (`bevy_grass`), first look (2026-11-23, groundcover)

Status: owner 2026-11-23 (look 1): "carpet is good but could be denser"; density to be taken from the industry standard. Was: awaiting owner (owner look 1, after GC-3b and GC-3c). Scene `gc-blades`
(= `gc-ascadian` with `--set groundcover.blades.enabled=true`); blades are off by default until this look.
Look for: a dense carpet of blades on the grass ground only (none on roads, rock or under water), standing on the ground,
the vanilla clumps among them as accents, blades staying in place as you walk (no reshuffle when the world origin
shifts), wind fronts crossing the carpet in the weather's direction, no hard edge where the blades end.
Known: one style ("meadow") everywhere, fixed tint (not yet the terrain's colour), no blade shadows, no trampling yet.
Log line: `groundcover: blade species B, styles registered R`.

### GC-M4: no groundcover under statics, blades at 150/m² (2026-11-23, groundcover)

Status: awaiting owner (launched 2026-11-23 22:58). Scene `gc-blades`. Look for: blades denser than look 1 (150/m², was
60), no blades or clumps poking through rocks, house walls, docks or stairs, grass still under trees and bushes, no hard
rectangle of bare ground around objects (margin 0.25 m). Log: `groundcover: statics read P placements, S boxes,
excluded E, floating F, no ground G, terrain tiles T in X ms` (1174 ms on the first run: synchronous at start, to move).

### GC-3n: denser blades near the camera (2026-11-23, groundcover)

Status: awaiting owner. Scene `gc-blades`. Look for: the ground at your feet covered (no "last few hairs"), no visible
ring where the extra density ends (4 m full, fading out by 7 m), no blades popping or reshuffling as you walk. Live:
`groundcover.blades.near_density` (1 = off, 2.5 default, max 4), `near_start_m` 4, `near_end_m` 7. perf ab: GPU +0.06 ms.
Known: colour is still the bright MEADOW green (GC-3e2 next).
