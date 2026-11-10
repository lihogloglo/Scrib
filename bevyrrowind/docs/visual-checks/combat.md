# Visual checks: combat

Open visual checks: melee combat (CB-*; superfargoth `docs/specs/combat.md`, `docs/specs/combat-adapter.md`). Index and how-to:
[`../visual-checks.md`](../visual-checks.md).

## Combat arena (`levels/arena.ron`, scene `level-arena`)

40 m floor; the player starts 3 m in front of a training dummy (Fargoth, scaffold `TrainingDummy`: never moves or attacks, health 100,
refills at 0); a second dummy 10 m to the left; a 1 m ledge. Cameras `behind`, `chain` (side view). Each hit on a dummy is logged
(`training dummy: hit N <tier> from the <side>, health h/100`).

### CB-1: Light chain and roll on the dummy (C1b)
- `.\bevyrrowind scene level-arena` (or `scene level-arena --camera chain`). Keys: F readies / sheathes the weapon (an attack press while
  sheathed draws instead); LMB or pad right bumper attacks (four presses = the light chain); Shift or pad East: tap rolls (on release), hold
  sprints; O toggles the combat gizmos (hurt capsules green, cyan in i-frames; blade yellow, red while a hit window is open).
- Look for: hits land when the blade arrives at the dummy (not before); hit-stop reads as impact, not a stall; the dummy's reaction matches the
  side; the stamina (green) bar drains per swing and refills after; the roll moves ~2 m with no slide at its end; sprint stops when winded.
- Tune in `bevyrrowind/crates/mw_combat/data/movesets/sword_1h.ron` (windows, hit timings, hit-stop, costs, weapon segment) and the slot clips
  in `mw_graphs/data/animation/sources.ron` (re-bake).
- Known gaps: no weapon mesh (the stance is gameplay only); medium and large reactions share the knockback clip at full travel (no
  per-tier root scale yet); no pad default for ready weapon; the dummy is talkable (dialogue picks up every NPC character).
- Agent live check (2026-10-27): dummies stand on the floor, light1 lands Small front hits (100 -> 90 -> 80), light3's second hit lands
  Medium; gizmos draw capsules and the blade at the hand. Roll and sprint not confirmed live (BRP input).
- Owner 2026-10-27: animations work well. Bugs: (1) holding Shift never sprints, every press rolls; (2) the chain walks the player
  forward too far (UAL2 attacks carry 0.8-1.6 m root; ER steps ~0.3-0.5 m: root scale per move + capsule collision); (3) mouse
  not captured in 3rd person (windowed: cursor shows, a click lands in another window).
- Fixed 2026-10-27 (afternoon): hold sprint (a stray unbound Sprint release cancelled the shared Shift / pad B press, `sf_action`
  `InputSampler::release`); cursor confined on Windows; light chain ~0.4 m per swing (per-move `root_scale` in `sword_1h.ron`); body stays
  on the capsule through the chain (`sf_anim` cook `start_anchor`: UAL2 split-take clips kept a ~0.95 m pelvis offset); player vs dummy
  collision (`separate_moved`: the mover stops/slides, the dummy is never shoved). Owner confirmed all live (copy c, arena).
- Status: confirmed. Owner 2026-10-27 evening (copy d): the 4-hit chain (UAL2 `Sword_Light_A-D`, combat.md "C1b" table) and M1 (cursor
  stays hidden on a left click and in F11) confirmed.

### CB-2: Lock-on, locked facing and side rolls (C1c parts 1-4)
- `.\bevyrrowind scene level-arena`. Q (or middle mouse, pad R3): lock the dummy in front; Q again releases; Q with nothing in the
  15 m / 35 deg cone recentres the camera. Then walk away past 20 m (breaks), mantle the 1 m ledge (breaks, re-offered 1 s after landing).
- Look for: the camera frames the target; a small white dot (dark rim) sits on the locked dummy's chest, gone when unlocked or off screen;
  locked, the body keeps facing the dummy: A/D circle it, S backs away facing it, standing still turns to it, sprint faces its run;
  locked Shift tap + A/D = a side dodge (~2 m around it, no pause at its end); W/S + tap = the forward roll squared to the stick.
- Known gaps: locomotion still plays the forward cycles (strafe clips C1c-2b); no back roll / backstep clip; no target switching (C4).
- Owner 2026-10-27 (copy d): lock, release, recentre and locked facing confirmed ("main logic solid"); asked for the marker and faster
  side rolls (both built after, 2709b71, ddc252d). Status: lock + facing confirmed; marker and side rolls awaiting owner.
- Owner 2026-10-30 (dev3 copy): marker, side rolls and the rest work great. Status: confirmed.

### CB-3: F never draws a sword (owner report)
- Kind: synthetic. `scene level-arena` or any anim-lab level, player (F11).
- Seen 2026-10-30: F (ready weapon, `combat.rs` `ready_weapon`) never shows a drawn sword, and never did. Unknown whether
  `Stance` toggles without a visible weapon or the action never fires.
- Fixed 2026-11-05 (stance-visible): F did toggle `Stance`, but nothing showed it: the avatar never got
  `CharacterReadiness`, and the `player` record holds no weapon. Now the avatar follows the stance, levels `arena` and
  `anim-lab` equip `iron longsword` (`player_held`), and a sheathed click or right click does nothing (no implicit draw).
- Run: `cargo dev scene level-arena` (F11: player). Look for: no sword in hand at start; LMB / RMB do nothing; F: the
  sword appears in the right hand and the idle / run become the one-handed set; attacks swing it, the blade follows the
  hand; F again: the sword vanishes. Real cells: the player has no weapon yet (no equip).
- 2026-11-05 (draw-clips): F plays Morrowind's one-handed draw / sheathe (`WeaponOneHand` equip / unequip) on the
  upper body; the sword appears in the hand at the clip's attach key and vanishes at its detach key. Look for: walking
  or running while drawing keeps the legs going (upper body only); the sword shows as the hand reaches it (not at the
  press, not after the clip); LMB during the draw does nothing, LMB after it swings; F again: hand goes back, sword
  vanishes mid-clip, LMB refused from the press. Log line: `combat: draw .. s (attach at ..)`. No sheath model (original).
- Status: confirmed 2026-11-05 (owner: "the sheathe thing works well"). Follow-up: the sword attack clips (Quaternius UAL Sword_Light_*) are poor; generate new ones (3.8 clip factory).

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
