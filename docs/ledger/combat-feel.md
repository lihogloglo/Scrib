# Ledger: combat feel (combat slices, lock-on, guard, first-person melee and view)

Format: `date | what was tried | measured result (scene, ms, counters) | verdict | commit or doc`. Newest first. Sources:
`bevyrrowind/docs/CHANGELOG.md`, ROADMAP 3.5 / 3.4b, `visual-checks.md` (CB-, FP-). Everything below is headless-tested and
judged by the owner only where a verdict is named. Companion: [animation](animation.md). Engine page: [locomotion and action](../wiki/engine/locomotion-and-action.md).

2026-11-12 | Audit plan for combat: phase C (C2c backstep and knockdown, heavy charge, strafe on the new gait set, combat shake and view kick, attack clip pass with `ik.feet` curves) and phase D (aim pitch layer, head-joint eye, kick, world-hit recoil, view-model held items, framing pass); owner answers: dedicated kick in both views with physics pushes, movement keys choose the swing direction, both first-person camera styles (stable eye default, body-driven option), eye height follows the character's real height | design only; nothing built | **open** | `superfargoth/docs/research/animation/15-audit-and-plan-2026-11-12.md` §5, §6
2026-11-10 | FP-2b: view-model material (`ViewModelMaterial`, WESL copies of Bevy 0.20 mesh and prepass vertex stages, one pass not a second camera: a second camera would re-render 4 sun cascades) | headless tests; shader-compile log line is the only automated check; off by default (`flip viewmodel`) | built; FP-2 **awaiting owner** | CHANGELOG 2026-11-10 (FP-2b, FP-2a, FP-2 design)
2026-11-10 | FP-1 + FP-1b: roles split the outfit (first person hides the head only); per-worldspace shadow-only layer `S` so the hidden head still casts under sun, lamps, torches and RT | headless tests | built; FP-1, FP-1b **awaiting owner** | CHANGELOG 2026-11-10 (FP-1, FP-1b)
2026-11-10 | C2b: guard on right mouse / L1 (80 deg arc after 4 ticks, guarded damage and stamina, guard break at 0 stamina, regen x0.5; heavy moved to R; first-person body follows the view at 360 deg/s) | headless | built; CB-3b **awaiting owner** | CHANGELOG 2026-11-10 (C2b)
2026-11-10 | C4b: first-person attack look limits (`MoveRule.look`, view held within 45 deg of the body until `Cancel(Move)`, rolls free) | headless | built; CB-5b **awaiting owner** (judge with view model on and off) | CHANGELOG 2026-11-10 (C4b)
2026-11-10 | C4 soft aim: nearest target within 3 m and +-30 deg with line of sight on the swing's first 1/6 s; first-person attacks face the view, no hard lock in first person | headless | built; CB-5 **awaiting owner** | CHANGELOG 2026-11-10 (C4 soft aim)
2026-11-10 | C2a: sparring dummy strikes through ordinary `Command`s on a beat (light every 2.5 s, reach 2 m); player reactions by tier and side in both views (`level-sparring`) | headless | built; CB-3a **awaiting owner** | CHANGELOG 2026-11-10 (C2a)
2026-11-05 | Draw and sheathe: sheathed clicks do nothing, F shows the sword on the hand bone with Morrowind equip/unequip clips, attacks gated until drawn | owner CB-3 confirmed; owner: the Quaternius sword attack clips are poor, Kimodo swings unusable (MG-C0), mocap pack ruled out later (no mocap, free packs only) | kept; attack clip pass **open** (phase C4) | CHANGELOG 2026-11-05, ROADMAP 3.5
2026-11-05 | Hit and hurt volumes rethink: balls pass through arms outside the mover capsule, swings sometimes pass through a character with no reaction; per-bone hurt shapes (`character-shapes.md`, S0 and S3 built, PH-15) | `level-shapes-bench`, budget High <= 0.3 ms | **open** (S1-S7, owner questions) | `superfargoth/docs/specs/character-shapes.md`
2026-10-30 | CB-2 lock-on: Q / MMB / R3, locked facing and strafe, locked side rolls, marker | confirmed | kept; strafe clips (C1c-2b) not done | visual-checks CB-2
2026-10-27 | 4-hit light chain (UAL2 `Sword_Light_A-D`), M1 cursor | confirmed that evening | kept | CHANGELOG 2026-10-27 (feel-coord)
2026-10-27 | C1b: player vs training dummy (`sf_combat_bevy`, 14 UAL slots, F stance, LMB chain, Shift tap roll / hold sprint, stamina HUD, `level-arena`) | after the afternoon bug pass (sprint hold, chain step, mouse capture, dummy collision): CB-1 confirmed | kept | CHANGELOG 2026-10-27 (combat-c1b)
2026-10-27 | C1a: Bevy-free combat core (`sf_combat`: sheet, vitals, sweeps, contact order, `StandardRules`, hit-stop) | 26 headless tests found and fixed clip-rate root motion, chain ranks, `End` parsing, hit-stop hold | kept | CHANGELOG 2026-10-27 (combat-c1a)

## Standing facts

- Direction (owner 2026-10-27): Elden Ring-like third person with a classic-rules seam (`CombatRules`); the player attacks only with the weapon drawn.
- Combat is data: sheets in `bevyrrowind/crates/mw_combat/data/movesets/sword_1h.ron` on UAL clip slots, run on the 60 Hz tick; the sparring partner is just another `Command` writer.
- First-person melee: attacks face the view, soft aim 3 m / 30 deg, view limited to 45 deg of the body during a swing, no hard lock in first person.
- Every combat feel check after CB-3 is awaiting the owner (CB-3a, CB-3b, CB-5, CB-5b, FP-1, FP-1b, FP-2): confirmed so far are CB-1, CB-2, CB-3.
- The sword attack clips are the weakest part (owner 2026-11-05); the attack clip pass needs a free pack or generation, never mocap.
- The first-person view model is one render with a view copy, not a second camera (Bevy 0.20 builds sun cascade views per camera).
- Fixed eye height 1.62 m and no head-joint eye yet; the plan (F2) is eye from the head joint, low-passed, rotation from the view.
- Logged, not chased: actor separation has no wall sweep; NPC walkers (ORCA) do not block the player; dummies have `CellCoord` but no `big_space` regrid.
