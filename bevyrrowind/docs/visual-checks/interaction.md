# Visual checks: interaction

Open visual checks: smart objects, activation, furniture use, doors (IN). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

### IN-32: Activation through the interaction contract, behaviour unchanged (`owner_launch.sh view "Seyda Neen, Census and Excise Office"`; IN32c1/c0/c2)
- Kind: game location, player mode, default flags. Proof in `last.log`: `interact: N furniture defs loaded` (N > 0) and `interact: indexed M furniture placements in` (M > 0).
- Ask (yes/no): (1) Face Socucius, press E: does the dialogue open as before? (2) A chest or crate, E: does its container window open as before? (3) A loose item, E: is it taken as before? (4) A chair: does the prompt read "Sit"? (E now sits the player since AN26c2 508a628a: the sitting itself is anim's check ANIM-11, same scene.) (5) The door out, E: one fade to Seyda Neen as before?
- Second scene, activators (IN32d: `ACTI` placements now have collision and can be focused; before, none had either): `owner_launch.sh view "Seyda Neen, Arrille's Tradehouse"`. Proof in `last.log`: `interact: indexed M furniture placements in` (M > 0). Ask (yes/no): (6) Go up to the bed on the top floor and walk into it: are you blocked (before: you walked through it)? (7) Looking at the bed: does a prompt with its name "Bed" show? (8) Press E on it: nothing breaks (no jump, no slide; lying down waits for the hold list and AN30 clips)?
- Status: awaiting owner (built 2026-11-26).

### IN-33: Swing doors open and close (`owner_launch.sh view "Seyda Neen, Arrille's Tradehouse" --sound`; IN33a-c3)
- Kind: game location, player mode, default flags, sound on (the door sounds are part of the check). Proof in `last.log`: `interact: 1 swing doors indexed in` (the one inner door, `in_c_door_arched`). Inner doors used to show "Open" and do nothing (AU-5, rejected 2026-10-30: "no swing, no sound"): they now turn on their hinge, and the door sound is back because the door swings, so AU-5 is judged here.
- Ask (yes/no): (1) Face the inner door, press E: does it turn open about its hinge edge, without a jump at the start or the end? (2) While it opens and again while it closes: do you hear a door sound each time, coming from the door? (3) Look at the open door: does the prompt read "Close" (and "Open" once it is shut)? (4) Open it, stand in the doorway where the leaf passes, press E to close: does the door stop at you instead of pushing or passing through you, and finish closing once you step away? (5) Leave it open, go out to Seyda Neen and come back in: is it still open?
- Second scene if wanted (three swing doors): `owner_launch.sh view "Balmora, Eight Plates" --sound`, proof `interact: 3 swing doors indexed in`, same five questions.
- Known, not part of this check: NPCs do not use doors yet (AI14), locked inner doors are not gated yet (crime LW6), a door seen from far outside its cell's active ring is drawn at its rest state only.
- Status: awaiting owner (built 2026-11-26).
