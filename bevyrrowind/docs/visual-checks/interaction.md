# Visual checks: interaction

Open visual checks: smart objects, activation, furniture use, doors (IN). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

### IN-32: Activation through the interaction contract, behaviour unchanged (`owner_launch.sh view "Seyda Neen, Census and Excise Office"`; IN32c1/c0/c2)
- Kind: game location, player mode, default flags. Proof in `last.log`: `interact: N furniture defs loaded` (N > 0) and `interact: indexed M furniture placements in` (M > 0).
- Ask (yes/no): (1) Face Socucius, press E: does the dialogue open as before? (2) A chest or crate, E: does its container window open as before? (3) A loose item, E: is it taken as before? (4) A chair: does the prompt read "Sit"? (E now sits the player since AN26c2 508a628a: the sitting itself is anim's check ANIM-11, same scene.) (5) The door out, E: one fade to Seyda Neen as before?
- Second scene, activators (IN32d: `ACTI` placements now have collision and can be focused; before, none had either): `owner_launch.sh view "Seyda Neen, Arrille's Tradehouse"`. Proof in `last.log`: `interact: indexed M furniture placements in` (M > 0). Ask (yes/no): (6) Go up to the bed on the top floor and walk into it: are you blocked (before: you walked through it)? (7) Looking at the bed: does a prompt with its name "Bed" show? (8) Press E on it: nothing breaks (no jump, no slide; lying down waits for the hold list and AN30 clips)?
- Owner 2026-11-26 (dev exe at 1c699df9, both scenes): (8) yes ("I press E on bed, broke nothing"). (4) yes by the log (`activate: Sit (furn_com_rm_chair_03 ...)`, 12 presses on 3 seat records); the sit itself failed (ANIM-11: E on "Sit" does nothing; the sit fix S1/S2 is this lane's). (1)-(3), (5)-(7) not answered.
- Status: partly confirmed 2026-11-26 (q4, q8); q1-q3, q5-q7 awaiting owner.

### IN-33: Swing doors open and close (`owner_launch.sh view "Seyda Neen, Arrille's Tradehouse" --sound`; IN33a-c3)
- Kind: game location, player mode, default flags, sound on (the door sounds are part of the check). Proof in `last.log`: `interact: 1 swing doors indexed in` (the one inner door, `in_c_door_arched`). Inner doors used to show "Open" and do nothing (AU-5, rejected 2026-10-30: "no swing, no sound"): they now turn on their hinge, and the door sound is back because the door swings, so AU-5 is judged here.
- Ask (yes/no): (1) Face the inner door, press E: does it turn open about its hinge edge, without a jump at the start or the end? (2) While it opens and again while it closes: do you hear a door sound each time, coming from the door? (3) Look at the open door: does the prompt read "Close" (and "Open" once it is shut)? (4) Open it, stand in the doorway where the leaf passes, press E to close: does the door stop at you instead of pushing or passing through you, and finish closing once you step away? (5) Leave it open, go out to Seyda Neen and come back in: is it still open?
- Second scene if wanted (three swing doors): `owner_launch.sh view "Balmora, Eight Plates" --sound`, proof `interact: 3 swing doors indexed in`, same five questions.
- Known, not part of this check: NPCs do not use doors yet (AI14), locked inner doors are not gated yet (crime LW6), a door seen from far outside its cell's active ring is drawn at its rest state only.
- Owner 2026-11-26 (dev exe at 1c699df9, Arrille's Tradehouse with sound, proof `1 swing doors indexed`): (1) yes ("Door opens and closes"). (2) yes ("the sound of door worked (just tried)"; cues `door heavy open` / `door heavy close` in the log), so AU-5 passes with it. (3)-(5) not answered.
- Status: partly confirmed 2026-11-26 (q1, q2); q3-q5 awaiting owner.

### IN-19: Snuff and relight candles and lanterns (`owner_launch.sh view "Seyda Neen, Arrille's Tradehouse"`; IN19c)
- Kind: game location, player mode, default flags, any hour (interior). Proof in `last.log`: `interact: N LIGH records, M snuffable fire lights` (M > 0) and `interact: K lights indexed in` (K > 0).
- Do: face a candle or a wall lantern (fixed flames have no name), press E, then E again.
- Ask (yes/no): (1) Does the prompt read "Snuff" on a burning candle and "Relight" once it is out? (2) After Snuff: does the light it cast on the walls and table go out, and its flame/glow disappear, with nothing else of the candle or lantern missing? (3) Relight: does it come back exactly as before? (4) Snuff one, walk out to Seyda Neen and back in: is it still out?
- Known, not part of this check: the fire's loop sound keeps playing; NPCs don't see the darkness yet (AI light copy); carryable lanterns keep "Take" (snuff on the hold list, no hold list UI yet).
- Status: awaiting owner.

### IN-38: Break a crate or a pot with the dev key (`owner_launch.sh view "Seyda Neen"`; IN38b1 + IN38b2)
- Kind: game location (dev build), player mode, default flags, daytime, the crates and barrels by the docks. Proof in `last.log`: `interact: breakables: N records` (N > 0), then per press `interact: dev break: ... blunt on <record>` and `break <id> stage 0: Live`.
- Do: face a crate, barrel or sack (not a chest), press `Delete`; then a loose clay pot on a table indoors; then leave the area and come back.
- Ask (yes/no): (1) Does the aimed crate vanish at once, model and collision (you can walk through where it stood)? (2) Does nothing else around it vanish or move? (3) Does the pot vanish the same way? (4) After leaving and coming back, are both still gone?
- Known, not part of this check: no debris, no sound, no spilled contents yet (IN1c, IN38b3); chests, quest crates and decorative urns don't break.
- Status: awaiting owner.

