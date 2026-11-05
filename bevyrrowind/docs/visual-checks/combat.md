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
- Status: fixed, awaiting owner.
