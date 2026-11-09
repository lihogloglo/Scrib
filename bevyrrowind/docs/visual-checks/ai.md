# Visual checks: NPC AI (superfargoth `docs/specs/ai.md`, prefix `AI-`)

## Synthetic scenes (framework examples, no game data)

### AI-1: AI debug overlay on a synthetic trace (`cargo run -p sf_ai_bevy --example ai_overlay --features debug`, slice AI3)
Status: **awaiting owner** (built and headless-tested 2026-11-09; nothing decides in game before AI4/AI6, so the
trace is pushed at startup: the overlay shows only what the store holds). Backtick opens the console, WASD flies.
1. Click the left capsule, type `ai.select`: amber ring at its feet; above its head, facing the camera, four lines:
   `ai 000...a1 (watched)`, `#110 t2 plan kept_not_better`, `plan [p_wander] | paused`, `op wander running`.
2. `ai.select a2` (middle): red ring, `plan no_plan`, `plan -`, `op -`. `ai.select a3` (right): grey ring,
   `no decision traced yet`. `ai.trace` prints the steps (a1: two lines, `rej be_guard:patrol pre0`).
3. `ai.gizmos off` hides it, `ai.gizmos on` brings it back; `ai.select off` removes it. Look for: text readable
   from ~5 m, not mirrored, follows the camera when you fly round; nothing drawn on the unselected capsules.
