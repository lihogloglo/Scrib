# Visual checks: groundcover

Spec `superfargoth/docs/specs/groundcover.md`; adapter `bevyrrowind/docs/specs/groundcover-adapter.md`; ledger
`docs/ledger/groundcover.md`.

### GC-2: vanilla grass and ferns scattered from the land textures (2026-11-22, groundcover-research)

Status: awaiting owner. Scenes `gc-ascadian` (cell 2,-7, 08:00), `gc-grazelands` (11,8, 14:00), `gc-bittercoast`
(-3,-8, 10:00), all at eye height; A/B `--off groundcover`. Models: vanilla `flora_grass_01-07`, `flora_bc_grass_*`,
`flora_bc_fern_*` with the MVR PBR textures (first mesh of each model, no normal/ORM maps yet). Every density, spacing and
range is a `Guess` (`bevyrrowind/data/groundcover.ron`). Log line: `groundcover: species S, channels matched M/N,
unmatched [..], models loaded L/13`.
Look for: plants only on grass land textures (none on roads, rock, mud, cobbles, under water), standing on the ground
(not floating or sunk), no hard edge where the ring ends, sway without popping, no tiles visibly appearing as you walk.
Known: Bitter Coast ground is mostly `Tx_BC_mud` (no species yet, so sparse there); no shadows past the near range.
