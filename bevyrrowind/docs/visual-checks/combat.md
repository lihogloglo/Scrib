# Visual checks: combat

Open visual checks: melee combat (CB-*; superfargoth `docs/specs/combat.md`, `docs/specs/combat-adapter.md`). Index and how-to:
[`../visual-checks.md`](../visual-checks.md).

## Combat arena (`levels/arena.ron`, scene `level-arena`)

40 m floor; the player starts 3 m in front of a training dummy (Fargoth, scaffold `TrainingDummy`: never moves or attacks, health 100,
refills at 0); a second dummy 10 m to the left; a 1 m ledge. Cameras `behind`, `chain` (side view). Each hit on a dummy is logged
(`training dummy: hit N <tier> from the <side>, health h/100`).



### CB-1 re-check: sword attacks on Morrowind clips by direction, step-in (anim C4)
- `owner_launch.sh scene level-arena` (synthetic greybox; F11 player, F to draw). Not built yet: launch only after anim C4 is committed.
- Questions (yes/no): (1) Does each direction play its own Morrowind swing (stick forward or back: thrust; sideways: slash; still or
  diagonal: chop; OpenMW `getMovementBasedAttackType`, `character.cpp` 2927-2935 @ `4e718b5`)? (2) Does the step-in close the gap to the
  dummy without sliding feet or overshooting it? (3) Does the dummy react as the blade arrives (no early or late hit)? (4) Is LMB x3 still
  a fluent chain on the Morrowind clips (no stall between swings)?
- Proof log: `training dummy: hit N <tier> from the <side>` in last.log. Status: planned (after anim C4).

### CB-6: Spammable roll: press during a move rolls at once, mashing chains (roll-chain, 2026-11-25)
- Bake first (`bevyrrowind bake`: roll rate changed). `cargo dev scene level-sparring` (cameras `behind`, `side`), F11, F to draw.
- Feel for: mash Shift: rolls chain every ~0.58 s with no dead time (was ~0.9-1.3 s, presses dropped); Shift pressed during an
  attack's recovery rolls on the press, not on release; from standing, a tap still rolls on release and a hold still sprints;
  holding Shift through a roll keeps sprinting after it; the roll reads quicker (rate 1.25 -> 1.6, same ~4.5 m travel,
  i-frames 0.02-0.40 s); no pop when a roll restarts on itself.
- Tune: `mw_combat/data/movesets/sword_1h.ron` roll (`rate`, `Accept` 0.25, cancels 0.56-0.60), tunable `move.input.tap_hold_s`.
  Clip is still UAL1 `Roll` (Morrowind has none). Measurement: `mw_bake` `roll_root_travel_against_time` (ignored test).
- Status: confirmed 2026-11-25 (owner: "roll is good"). Source: gameplay-roadmaps roll-chain (`docs/research/gameplay-2026-11-25/anim-diagnosis.md` §2).

### CB-5: First-person melee with soft aim (C4 soft-aim part, 2026-11-10)
- `cargo dev scene level-arena`, F11 player, Tab to first person, F to draw. The dummy stands 3 m ahead (just at the soft-aim reach:
  step ~0.5 m closer). Keys as CB-1; Q (lock) does nothing in first person.
- Feel for: LMB swings where you look; with the dummy up to ~30 deg off the crosshair and within ~3 m the swing bends onto it in its
  first ~0.1 s (the view never turns for you); turning the mouse mid-swing doesn't drag the blade (it lands on its line, the view
  leaves the arms); locked in third person then Tab: the lock (dot) goes. Third person unlocked: swings bend onto a near dummy too.
- Tune: `LockConfig::soft` (reach, cone, window; `sf_combat_bevy`), the sheet's `Turn`/`NoTurn` windows (`sword_1h.ron`).
- Known gaps: the body faces the stick, not the view, while walking (an attack snaps it to the view); no view-model arms (FP-2);
  look limits during a swing: CB-5b. Status: awaiting owner.

### CB-5b: First-person attack look limits (C4b, 2026-11-10)
- As CB-5 (`level-arena`, F11, Tab, F). Run it twice: view model off (default) and on (dev console `on viewmodel`).
- Feel for: LMB, then whip the mouse sideways: the view stops ~45 deg off the body (soft, eased in ~0.1 s, no snap) and the arms
  and blade stay on screen; up/down stops at ~-45 / +20 deg; the stop lifts as the swing's recovery starts (light1 ~0.57 s) and
  the view moves on without a jump; chained lights move the stop to each new swing; a roll (Space) never limits; Tab to third
  person: no limits. With the view model on the arms draw ~1.24x larger: judge whether 45 / -45..20 still frames them.
- Tune: `look` per attack in `sword_1h.ron` (yaw, pitch, ease); default off if it feels like a wall. Status: awaiting owner.


### CB-3b: Guard in both views; guard break (C2b, 2026-11-10)
- Bake first (`bevyrrowind bake`: new slots `guard.sword_1h.{raise,idle,break}`). `cargo dev scene level-sparring` (cameras `behind`,
  `side`), F11, F to draw, then hold right mouse (or L1): the guard (UAL2 Sword_Block raise into a crouched hold). Third person, then
  Tab to first person, view model off and `on viewmodel`.
- Feel for: guard up within ~4 frames, the hold pose steady while held (loop seam: no pop), down on release. Facing the dummy its
  hits are guarded: hit-stop, no reaction, health -3.5 (not -10), stamina bar dips ~13 then refills at half rate; turn your back or
  side past ~80 deg: a normal hit. Third person the guard turns to the stick or faces a lock; first person the body follows the
  mouse at 360 deg/s (a fast flick leads, the guard catches up), pitch held to -45..20 like attacks. Drain stamina (roll, attack)
  then block: the guard breaks (UAL2 Idle_Shield_Break), and still held, comes back up after it. Sheathed: right mouse does nothing.
- Tune: `mw_combat/data/movesets/sword_1h.ron` `guard` (raise, arc, Turn rate, look), hit rows `guard_damage`, `mw_combat`
  `GUARD_ABSORB` / `GUARD_STABILITY`; clips in `mw_graphs/data/animation/sources.ron`. Known gaps: no guard-hit recoil clip (an
  Additive flinch, not wired to the avatar's hit springs yet), no guard walk (stationary), no HUD guard cue.
- Owner 2026-11-12: **partial**. The held guard is a looping clip that visibly jumps (not a still hold); stamina refills too fast to
  test the guard break. Clips judged poor overall (Quaternius): regenerate (3.8 clip factory). Break not yet judged.
