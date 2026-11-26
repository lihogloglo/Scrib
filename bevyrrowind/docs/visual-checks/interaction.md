# Visual checks: interaction

Open visual checks: smart objects, activation, furniture use, doors (IN). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

### IN-32: Activation through the interaction contract, behaviour unchanged (`owner_launch.sh view "Seyda Neen, Census and Excise Office"`; IN32c1/c0/c2)
- Kind: game location, player mode, default flags. Proof in `last.log`: `interact: N furniture defs loaded` (N > 0) and `interact: indexed M furniture placements in` (M > 0).
- Ask (yes/no): (1) Face Socucius, press E: does the dialogue open as before? (2) A chest or crate, E: does its container window open as before? (3) A loose item, E: is it taken as before? (4) A chair: does the prompt read "Sit", and E leaves you standing, with no jump or slide (sitting waits for AN26c clips)? (5) The door out, E: one fade to Seyda Neen as before?
- Status: awaiting owner (built 2026-11-26).
