# Visual checks: physics

Open visual checks: physics, ragdolls, carry/drag (PH-*). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

## Physics (superfargoth `physics.md`, bevyrrowind `physics-adapter.md`; `.sessions/physics-p2.md`)

PH-3 is specified in `docs/specs/physics-adapter.md` and lands with task P10; PH-2 (first half, P7) and PH-4 (carry) are below.

### PH-2 (first half, P7): a dead guard ragdolls on the Balmora stairs
- Note 2026-10-25: `sf_ragdoll` tests `limp_drop` and `settles_on_stairs` fail on their stretch limit (pre-existing).
  After this check they become sanity guards + printed measurements (CLAUDE.md hard rule 5).
- Kind: game location. Run: `.\bevyrrowind view "Balmora"` (release, physics on), fly (free camera) to a guard on
  the stone stairs (the High Town steps up to the Council Club / Hlaalu manor), put the crosshair on him and press
  `I`. He plays his death clip for 0.4 s, then goes limp. Then hold LMB on a hand or foot to drag the body (a 75 kg
  corpse is a drag at the default Strength 50: the camera slows), release to drop.
- What to look for: the clip-to-ragdoll cut is not a pop (velocities carry over); the body tumbles or slumps down the
  steps without exploding, jittering or sinking into them, then rests within a few seconds; limbs bend the natural
  way (knees back, elbows forward) and stay attached (the log's `ragdoll: joint stretch peak` stays under 1 cm);
  the dragged corpse follows by the grabbed limb. Creatures too (a guar, a cliff racer: `I` on them).
- Press F7 + G to see the skinned-mesh wireframe (yellow) over the ragdoll capsules (orange) and joints (magenta):
  the mesh must follow the bodies (heads are not drawn: morph-target meshes crash Bevy's wireframe pass). Both keys are in the F1 overlay.
- Known (P8): no powered crumple yet (the clip cuts to limp); the character root stays where it died (a corpse
  dragged far can be culled early); corpses don't persist across cell unloads; creature masses are volume
  estimates (a guar is 1 t: too heavy to drag).
- Agent run 2026-10-18 (p9fin, release, `scene wk-balmora`, no keyboard):
  `$env:BEVYRROWIND_AUTO="kill_nearest@3,shot:shots/p9/kill.png@4.5,exit@8"`. Log: `ragdoll: killed ...
  (meshes/base_anim.nif)`, 1 spawned, 0 failed; joint stretch peak 1.4–2.1 cm (over the 1 cm target: open, P8b).
  Sanity shot `shots/p9/kill.png`. Not on the stairs (wk-balmora's camera); creatures not run.
- **Lead (owner 2026-10-19, round 11, with `G` collider view):** the ragdoll's physics capsules behave normally when hit; it's the skinned mesh that doesn't follow them. So the solver is fine and the bug is the pose write-back (bodies → bones: bone/body offset, local vs world frame, or the floating-origin offset), which would explain sinking, floating, spinning and the odd joint angles.
- Earlier status: rejected by owner 2026-10-19 (round 10): ragdolls very bad: most bodies sink into the ground, float and turn around; joints look unconstrained (unnatural angles); they don't explode. Drag doesn't match the expected Oblivion-style hold/drag UX (later). Phase 3.
- **R0 fix (anim-refactor, 2026-10-22):** the clip's root still placed the mesh (joints above the pelvis body had physics
  weight 0, and root motion kept moving the frame); non-uniform race build scale skewed lying limbs; foot placement
  bent ragdoll legs. Headless test now holds every bone on its body within 0.1 mm while the frame moves, yaws and
  rebases (`sf_physics_bevy` `bones_follow_bodies_*`). Not checked live by the agent.
- Re-check: `scene level-anim-drop` (AL-6), press `G`: the mesh sits inside the capsules during the fall, the knockdown
  and at rest (no sinking, floating or spinning; limbs follow their capsules); `I` on the walker, then on a Balmora
  guard on the stairs as above. Expect a slight width change at death on races whose build ≠ height (uniform scale
  while ragdolled, restored at get-up). Status: **awaiting owner**.

### PH-7: Item body shapes (`body.cook`)
- Kind: game location. Run: `.\bevyrrowind scene ph-tradehouse-items` (Arrille's Tradehouse with `--physics-debug`;
  `G` toggles). Placed items (MISC, WEAP, ARMO, CLOT, BOOK, ...) draw orange: their `body.cook` shape (a box, capsule,
  cylinder or sphere when one fits within 15 % of the hull, else one hull of at most 32 vertices, or up to 8 hulls for
  concave closed models). Statics stay cyan. Since P4 items are sleeping dynamic bodies (held ones `Fixed`).
- Look for: every item on the counter and shelves has an orange shape hugging it (bottles as cylinders/capsules, books
  and plates as flat boxes, weapons as thin boxes or capsules along the blade); no shape offset, rotated or scaled
  away from its item; no item without a shape; nothing grossly bigger than its item.
  Bookshelves and cabinets that hold items draw their render mesh in cyan (`support.cook`), not a box.
- Status: partial, owner 2026-10-19 (round 11): shapes very good almost everywhere; some items have none (the ceiling candle holder, a small box on Arrille's desk: lights/containers not cooked?). Phase 3.
  clutter in front, Arrille behind his counter at the right; sanity screenshot `shots/physics/ph7_tradehouse_a.png`
  shows orange shapes on the book, staff and shelf items, lined up. `SF_ITEMS_CELL="<cell>"` (+ `SF_ITEMS_NEAR="x,y,z"`
  for the statics around a point) on `cargo run --release -p mw_bake --example body_report` lists a cell's items with
  their `items.check` result.
- Source: physics-p3, physics-p4b.

### PH-9 (P8): knockdown and get-up

Status: **partial, owner 2026-10-19 (round 10): walkers fall and stand back up, but the fall animation is terrible (basically none). Phase 3.** Earlier: agent-run 2026-10-18 (p9fin): impacts ≥ 15 N·s (`KNOCKDOWN_IMPULSE`) knock a walker
down; the get-up root comes from the `Knockout@tail` clip's first frame (`clip_root_body`; the `LYING_REST_GET_UP`
scaffold only for skeletons without that clip, which logs a line).
Agent run (release, no keyboard): `$env:BEVYRROWIND_AUTO="ball_at_nearest@3,shot:shots/p9/knock.png@4.3,shot:shots/p9/getup.png@8,exit@10"; .\bevyrrowind scene wk-balmora`.
Log: `ragdoll: knocked down ... (25–36 N·s)` for the 12 m/s ball, `got up (heading ...)` 2.2–2.5 s later, no
scaffold line. Sanity shots `shots/p9/knock.png` (lying on the street, ball beside), `shots/p9/getup.png` (standing,
feet under it). A second knockdown right after a get-up logged a 5.6 cm joint stretch peak (open, P8b).
Scene: Balmora, a walker on the Temple stairs; free camera 6 m away; press `V` to throw the 3 kg ball at the walker.
What to look for: the walker braces and falls with the hit (no pop from the animated pose into the ragdoll), lies
still within ~2 s, then gets up from where it lies with its feet under it (root placed under the pelvis); face-down
bodies roll over during the get-up blend (the `ROLL_TO_BACK` scaffold); no joint stretch, no spin of the root.

### PH-10 (P9): kinematic hit reactions and tail spring bones

Owner batch 2026-10-18: "didn't work". The window was closed ~13 s in, before the first ball (the `@` times count
from the settled start, ~13 s after launch in release). Orchestrator re-run (`ball_at_nearest:2@4,...`): the balls
fly, but the first 2 m/s ball knocked its walker down at **72 N·s** ~1.9 s after the throw (likely the walker
walking into the ball on the street), and neither soft ball logged a `hit react`. Bug, back to physics (P9).

Fix 2026-10-18 (p9soft): impacts are now the incoming body's momentum at first contact (`PhysicsEvent::Hit`,
`sf_physics::impact_impulse`: `m · min(v_by·n, closing speed)` from rapier's pre-solve `CollisionEvent::Started`),
not the solver's contact force (a walker pressing a resting 3 kg ball into a wall measured 423 N·s that way; now no
hit). Second cause, found by test: the character push zone (wider than the capsule) batted incoming soft balls away
before they touched (no hit at all) and braked balls from behind; pushes now skip trailing-side and incoming
(> 0.25 m/s) bodies. `impact_threshold` 5 → 1 N·s. Tests: sf_physics `character_tests` (soft hit 6.0 N·s, chasing
ball = closing speed, approaching walker, walking into a resting ball = no hit). Agent re-run (release, the
orchestrator's `ball_at_nearest:2@4,...,ball_at_nearest@16` run): `hit react: 3915v0 (6.4 N·s, strength 0.43)`
(second soft ball), `knocked down 3915v0 (35 N·s)` (12 m/s), `got up`; no knockdown from a ball on the street.
Open: the first soft throw at a newly framed walker fell short of the chest (sanity shot: ball in the air at knee
height in front, no contact; an aim issue in the `BEVYRROWIND_AUTO` throw, not detection; it hit in the previous
run, 2.6 N·s); the knocked-down ragdoll tumbling into a neighbour gave it ~10 small `hit react`s (1–5 N·s) in
1.5 s (real contacts; no per-character cooldown yet); hit strength tuning not done (owner: don't polish).
Awaiting owner.

Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
strength 0.40)`, sanity shot `shots/p9/hit.png` at the hit; the flinch itself needs eyes on the live run; tails not run).
- Kind: game location. Run (release): `$env:BEVYRROWIND_AUTO="ball_at_nearest:2@3,shot:shots/p9/hit.png@3.88,ball_at_nearest:2@6,exit@9"; .\bevyrrowind scene wk-balmora`
  (a new target is framed first and the ball leaves 0.5 s later; a 3 kg ball at 2 m/s released 0.6 m from the
  chest, about 6 N·s, under the 15 N·s knockdown; lobbed from 4 m any ball lands at ≥ 6 m/s, 18 N·s, a knockdown;
  the `V` key always throws at 12 m/s, a knockdown). Hits under 1 N·s (`CharacterPhysics::impact_threshold`) don't
  reach the graph at all. Tails: `.\bevyrrowind view "Seyda Neen"` near the guars
  south of the lighthouse road, or `character --walk` on an Argonian/Khajiit.
- What to look for: the hit walker's torso and head flinch away from the ball (peak ~0.09 s after the hit, back on
  the animation within ~0.5 s; no pop, no lasting offset, feet stay planted); log `hit react: ... strength`. A guar's
  or beast race's tail trails a little when it starts, stops or turns and settles without jitter or drift; at
  `BEVYRROWIND_FPS_CAP=30` and uncapped it looks the same.
- Known: hit strength is impulse / 15 N·s (`HIT_REACT_IMPULSE` 1 N·s minimum); no additive directional hit clips yet.

### PH-4: Carry, drag and throw (dev camera; corpse drag waits for P7)
- Kind: game location. Run: `.\bevyrrowind scene ph-tradehouse-items` (Arrille's Tradehouse; or any `view`). The
  free camera is the holder (no player yet), Strength 50 (`--env BEVYRROWIND_STRENGTH=30` / `=100` to compare). Keys:
  **hold the left mouse button** on an item under the screen center (within 2.5 m) to grab it, release to drop; while
  holding, **`V`** throws it (instead of a ball), **`[` / `]`** turn it 15°. Frame rate:
  `--env BEVYRROWIND_FPS_CAP=30` (or 60, 144; for 144 add `--env BEVYRROWIND_PRESENT=mailbox`). The log prints
  `carry: Grabbed { mode, mass }`, `TooHeavy`, `Thrown { speed }`, `Broke`.
- Look for: a bottle, bowl or book lifts into view and stays put relative to the view while you fly and turn, at 30,
  60 and 144 fps (no vibration or shimmer; it may lag a little on fast turns and settle); it pushes other clutter
  aside but never you. Throws arc and knock things over (S 50: 6 N·s, a 0.5 kg bottle leaves at the 15 m/s cap).
  Heavy items (over 15 kg at S 50: crates, sacks, barrels with a body) drag: they stay on the floor, slide or tip
  toward you, the camera slows (`1 − m / (2·drag mass)`); a dragged object is released by `V`, not thrown. Over
  120 kg (S 50; 88 kg at S 30) the log says `TooHeavy` and nothing moves. At S 30 a 12 kg item drags where S 50
  carries it. Pulling a held item through a wall for 0.3 s drops it (`Broke`). Dead bodies by a limb: P7.
- Automated (sf_physics `carry_tests`): view-space spread of a carried item while walking a 1.4 m circle at 1 rad/s:
  0.008 / 0.010 / 0.044 mm at 30 / 60 / 144 fps (gate 1 mm); a 0.3 kg throw leaves at 15.00 m/s (cap, within 1 %);
  a 6 m/s throw from 1.5 m lands 3.320 m out (ballistic 3.333); carry / drag / too heavy at the S 30 and S 100
  limits; a 40 kg crate dragged 2.3 m in 3 s; an 11 kg item on a 40 N grip breaks away; the holder's capsule never
  touches what it holds.
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
- Source: physics-p6.

### PH-11: Physics fix plan, night 2026-10-26 (`cargo dev view "Seyda Neen, Census and Excise Office"`, player mode)
- Fixes: collision-ready gate (slice 1), arrival gate + safety net (2-3), clutter asleep until touched (4), OpenMW NIF
  collision rules + full-triangle fallback (6-7, one collision rebake on first run), carried item drawn on the final
  camera (8) and from the body's tick pose (9). Plan: `specs/physics-fix-plan.md`.
- Look for: (a) F5, walk, F9 several times, also from outside: you never fall through the floor; the fade may hold a
  moment longer. (b) Enter the office and Balmora interiors: bottles, plates, books don't move or roll on entry; NPCs
  standing in clutter don't launch it. (c) Pick up a bottle (first and third person), walk and turn: no vibration;
  in third person it stays in front of the body. (d) The spot where you fell through the Census Office floor: solid now?
  If you still fall: note the F1 overlay coordinates. Also a thrown item still knocks clutter over.
- Status: awaiting owner.
