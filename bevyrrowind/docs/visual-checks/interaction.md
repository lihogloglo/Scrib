# Visual checks: interaction

Open visual checks: smart objects, activation, furniture use, doors (IN). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

### IN-32: Activation through the interaction contract, behaviour unchanged (`owner_launch.sh view "Seyda Neen, Census and Excise Office"`; IN32c1/c0/c2)
- Kind: game location, player mode, default flags. Proof in `last.log`: `interact: N furniture defs loaded` (N > 0) and `interact: indexed M furniture placements in` (M > 0).
- Ask (yes/no): (1) Face Socucius, press E: does the dialogue open as before? (2) A chest or crate, E: does its container window open as before? (3) A loose item, E: is it taken as before? (4) A chair: does the prompt read "Sit"? (E now sits the player since AN26c2 508a628a: the sitting itself is anim's check ANIM-11, same scene.) (5) The door out, E: one fade to Seyda Neen as before?
- Second scene, activators (IN32d: `ACTI` placements now have collision and can be focused; before, none had either): `owner_launch.sh view "Seyda Neen, Arrille's Tradehouse"`. Proof in `last.log`: `interact: indexed M furniture placements in` (M > 0). Ask (yes/no): (6) Go up to the bed on the top floor and walk into it: are you blocked (before: you walked through it)? (7) Looking at the bed: does a prompt with its name "Bed" show? (8) Press E on it: nothing breaks (no jump, no slide; lying down waits for the hold list and AN30 clips)?
- Status: awaiting owner (built 2026-11-26).
