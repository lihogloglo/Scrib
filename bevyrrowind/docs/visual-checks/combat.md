# Visual checks: combat

Open visual checks: melee combat (CB-*; superfargoth `docs/specs/combat.md`, `docs/specs/combat-adapter.md`). Index and how-to:
[`../visual-checks.md`](../visual-checks.md).

## Combat arena (`levels/arena.ron`, scene `level-arena`)

40 m floor; the player starts 3 m in front of a training dummy (Fargoth, scaffold `TrainingDummy`: never moves or attacks, health 100,
refills at 0); a second dummy 10 m to the left; a 1 m ledge. Cameras `behind`, `chain` (side view). Each hit on a dummy is logged
(`training dummy: hit N <tier> from the <side>, health h/100`).

### CB-1: Light chain and roll on the dummy (C1b)
- `.\bevyrrowind scene level-arena` (or `scene level-arena --camera chain`). Keys: F readies / sheathes the weapon (an attack press while
  sheathed draws instead); LMB or pad right bumper attacks (three presses = the light chain); Shift or pad East: tap rolls (on release), hold
  sprints; O toggles the combat gizmos (hurt capsules green, cyan in i-frames; blade yellow, red while a hit window is open).
- Look for: hits land when the blade arrives at the dummy (not before); hit-stop reads as impact, not a stall; the dummy's reaction matches the
  side; the stamina (green) bar drains per swing and refills after; the roll moves ~2 m with no slide at its end; sprint stops when winded.
- Tune in `bevyrrowind/crates/mw_combat/data/movesets/sword_1h.ron` (windows, hit timings, hit-stop, costs, weapon segment) and the slot clips
  in `mw_graphs/data/animation/sources.ron` (re-bake).
- Known gaps: no weapon mesh (the stance is gameplay only); capsules pass through the dummy, so the chain's root motion can walk the player
  into or past it and later swings whiff; medium and large reactions share the knockback clip at full travel (no per-tier root scale yet);
  no pad default for ready weapon; the dummy is talkable (dialogue picks up every NPC character).
- Agent live check (2026-10-27): dummies stand on the floor, light1 lands Small front hits (100 -> 90 -> 80), light3's second hit lands
  Medium; gizmos draw capsules and the blade at the hand. Roll and sprint not confirmed live (BRP input).
- Owner 2026-10-27: animations work well. Bugs: (1) holding Shift never sprints, every press rolls; (2) the chain walks the player
  forward too far (UAL2 attacks carry 0.8-1.6 m root; ER steps ~0.3-0.5 m: root scale per move + capsule collision); (3) mouse
  not captured in 3rd person (windowed: cursor shows, a click lands in another window). Status: open.
