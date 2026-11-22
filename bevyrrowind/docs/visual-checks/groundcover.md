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
