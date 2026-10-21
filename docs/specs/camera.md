# Camera: third person, first person, framed moves, comfort

Status: **draft, awaiting owner review (R11b, 2026-10-19).** Rig math built 2026-10-21 (`sf_camera`,
see "As built (rig math)" at the end, which wins where it differs); no Bevy shell yet (the game has a
free fly camera only). Crates: `sf_camera` (Bevy-free rig math, new), `sf_camera_bevy` (thin plugin, new).
Sources: `../research/animation/11-tradeoffs.md` §5-6 and "Decisions"; R2 §5-7, R5 §4, R6 §5, §9,
R7 §2, R8 §1.4, §2-3. Companion: [action.md](action.md).

## Rules

- **Third person is primary**; first person must be playable and comfortable (decision).
- **The camera is presentation.** It reads `ActionOutput` and the render body; it never writes
  gameplay state. Aim reaches gameplay only as `Command.view` (yaw, pitch), which the input shell
  fills from the player's look input, not from the camera's final pose (action.md "Co-op
  readiness"). Shake, framing and collision push-in never change aim.
- One gameplay body in both views. Switching views changes no gameplay tick.

## Third person (primary)

- **Free orbit** around a pivot on the render body (chest height, shoulder offset, side swap),
  yaw/pitch from look input, pitch limits, distance by mode (closer in Sneak, further in FreeRun).
- **Collision**: a sphere cast from the pivot to the desired eye; push in on hit, ease back out
  slower than in; near geometry and the character fade by dither when too close (R5 adapt).
- **Lag**: position follows with a short spring; rotation follows look input directly (no lag on
  aim). Auto-recentre behind travel is a comfort option, off by default.
- **Optional lock-on** (Combat stance only): acquire in a cone and range in front of the view;
  break at a longer range than acquire, on line-of-sight loss for a while, or when any traversal
  or interaction starts (action.md). Locked: the camera sits behind the character on the line to
  the target, eased, pitch by the height gap within limits; a lock press with no target recentres
  (R6 §5). **Re-acquire on landing**: if a traversal broke the lock, the same target is offered
  back for a short time after touchdown if still in range.
- **Close-combat aids** (R8 adapt): off-screen attacker indicators; these are UI, fed by `sf_ai`
  later.

## First person

- **Eye** = the render body's baked **virtual camera track** for the current clip (`clip.annotate`:
  low-passed head position, a few degrees of head pitch and roll, no head yaw; authored tracks and
  per-clip overrides win; R2 adapt) composed with view yaw/pitch. Each full-body move's track is
  reviewed in visual checks before it ships (R7 §5).
- **Per-move look limits**: yaw and pitch limits around the move's entry yaw, eased in over the
  rule's `ease` time, and `ignore_look` windows (action.md `look`). Hanging limits the look-down;
  a mantle limits yaw to the lip direction. Limits release eased.
- **Soft aim, no hard lock-on in first person** (decision). Soft aim is gameplay, in the action
  layer: early in an attack (`turn` window start), the nearest valid target in a small cone around
  `Command.view` and within reach becomes the lunge/turn target (R6 §5). The camera does not turn.
- **Render bodies** (decision; R7 §2.1, R8 §3): a first-person body (arms, and legs when looking
  down) with its own FOV and scale toward the camera, head hidden; plus the world body for shadows
  and reflections. Per action: an **arms layer** for weapons and spells (a content pack's own
  first-person clips may play here as presentation on the shared clock), or the **full body** with
  the baked camera for traversal, sitting and paired moves. Bevy has no shadow-only flag (character.md
  open question 1): render layers until it does. character.md's first-person section is rewritten
  to this model when D2 starts.

## Framed cameras (paired moves)

Clip windows `camera(preset)` on paired moves (takedowns, ripostes) take the camera for the move:
a profile (distance curve closing in, look-at on the clip's camera track or the pair's midpoint,
limits), eased in and handed back eased over ~1-2 s (R6 §9). Third person: allowed (decision). First
person: stays first person with framed look limits by default; a per-move third-person cut is an
option (action.md open question 3). Never used for traversal.

## Comfort switches (settings)

From XAG 117 and DL/ME practice (R7 §2.2, R8 §1.4):
- camera shake on/off and strength; shake is **rotation only** (trauma model);
- head bob: **off by default** in first person;
- auto-camera (recentre, framed cameras) on/off; framed cameras fall back to look limits only;
- FOV slider (world), separate first-person body FOV;
- centre dot on/off;
- motion blur on/off;
- assisted/manual parkour (lives in action.md, listed here with the other comfort options);
- view choice: first or third person, switchable any time.

## Verification

- Unit (`sf_camera`): collision push-in never puts the eye inside a static; look limits hold and
  ease; lock-on acquire/break hysteresis; re-acquire after a traversal.
- Gameplay isolation: a scripted command stream produces identical `ActionState` per tick with the
  camera in third person, first person, and absent (headless).
- Visual checks (bevyrrowind `docs/visual-checks.md`): CAM-1 third-person orbit, collision and
  lock-on on `ledges` and `seats-doors`; CAM-2 first-person mantle, hang and sit with look limits.

## Industry comparison

| Technique | Shipped practice | Us | Verdict |
|---|---|---|---|
| Third-person rig | UE Spring Arm (probe, lag), Souls lock-on behind-the-character (R6 §5) | orbit + sphere-cast push-in + optional lock-on | **match** |
| First-person body | UE 5.5 first-person rendering (own FOV/scale + world body for shadows); Mirror's Edge authored camera per clip (R7 §2.1) | FP render body + world body; baked camera track per clip | **match**, camera track is baked not authored |
| FP look limits | Mirror's Edge per-move limits (R2 §6) | per-move limits in move rules | **match** |
| Lock-on in FP | KCD hard lock; most FP melee soft aim (R8 §2) | soft aim only | **match** (owner chose soft aim) |
| Comfort | XAG 117; DL tuned FOV and clip speed; ME centre dot, no bob (R7 §2.2) | settings list above | **match** |

## Reuse (2026-10-21)

- **Bevy** has no gameplay camera rig (only `Camera`/`Projection` and free-fly examples).
- **`dolly`** (MIT/Apache, Bevy-agnostic rig composition: `Position`, `YawPitch`, `Arm`, `Smooth`,
  `LookAt`) is the closest fit in shape, but: last release 0.6.0 (2024-07) pins `glam <= 0.28` (we
  are on 0.33: a second glam plus `mint` conversions every frame); `Smooth` is exponential, not the
  critically damped spring with velocity feed-forward the pivot needs; and it has none of the hard
  parts (collision through our probe, lock-on, look limits, trauma shake). The part it would cover
  (yaw/pitch + arm composition) is ~20 lines. Not adopted; revisit if it moves to current glam.
- **`smooth-bevy-cameras`, `bevy_third_person_camera`**: Bevy ECS plugins that own input and
  `Transform`, so not usable in a Bevy-free core with a command-driven look; their collision (where
  present) is not our `GroundProbe`. Not adopted.
- Reused in-house: `sf_anim::spring` (Holden's critically damped spring and damper) and
  `sf_controller`'s `GroundProbe` + `ProbeQuery::sphere` (same statics and terrain as the mover).

## Open questions

None beyond action.md open question 3.

## As built (rig math, 2026-10-21)

`crates/sf_camera` (Bevy-free; deps glam, serde, `sf_anim` (springs), `sf_controller` (probe),
`sf_coords`; not `sf_action`, which changes daily: the shell maps modes and ids). Frame-rate
presentation: every function takes the frame `dt`; nothing here is saved or replicated except
`LockTimers`. Tests: `src/tests.rs` (16).
- **`Look`** (yaw about +Y, 0 faces +Z, pitch up; same as `ViewDir`): `forward`, `right`,
  `rotation` (camera -Z onto forward), `turned` (input, yaw wrap, pitch clamp), `eased`.
- **`OrbitRig::update(params, body, body_vel, look, RigMode, dt) -> OrbitPose { pivot, look, arm }`**:
  pivot at `pivot_height`, shoulder offset with an eased `swap_shoulder`, pitch clamped, arm length
  per `RigMode` (Normal/Sneak/FreeRun/Locked) eased. The pivot spring works in the body's frame:
  motion the gameplay velocity explains is followed exactly (no steady lag at any `dt`), snaps and
  velocity changes become lag a critically damped spring removes (separate horizontal/vertical
  half-lives); lag past `max_lag` snaps (teleports). `recentre_behind` (comfort, off by default)
  proposes a look the shell writes back.
- **`ArmCollision::update(probe, pivot, arm, params, dt) -> ArmResult { eye, length, free, fade,
  blocked }`**: one sphere cast (radius + margin) pivot to eye; the length is never above this
  frame's free length (push-in at once, never inside), and eases out with `out_halflife`; `fade`
  is a smoothstep of the arm length for the body's dither.
- **Lock-on** (target in `sf_action`, math here): `acquire` (cone + range + LOS, score mixes
  angle and distance), `switch_target` (smallest yaw step to a side), `LockTimers` (serde, kept
  with the target: `check` breaks past `break_range` > `acquire_range`, on a lost target, or
  after `los_timeout` without LOS; `break_for_traversal` / `touchdown` / `reacquire` offer the
  same target back for `reacquire_window` after landing), `locked_look` (yaw on the target line,
  pitch from the height gap plus a bias, clamped; ease with `Look::eased`).
- **`FirstPersonLook`**: `set_limits(Option<LookLimits>, entry_yaw)` on move change,
  `update(look, delta, ignore_look, free_pitch, dt)` blends free and limited look by a
  smoothstepped weight ramped over `ease`; the shell stores the result as its look, so input
  pushed past a limit never needs unwinding.
- **Comfort**: `ComfortSettings` (shake on + strength, head bob off, auto-camera off, world and
  body FOV, centre dot, motion blur, `ViewChoice`); `TraumaShake` (trauma decays linearly, shake =
  trauma² x hashed smooth noise, rotation only, compose as `look.rotation() * shake`).

Next: `sf_camera_bevy` (D1 shell: look from `bevy_enhanced_input`, candidates and LOS from the
world, dither fade, CAM-1 check); `sf_action` gains the lock target + `LockTimers` + a
`breaks_lock` rule field (E2); the first-person eye track arrives with D2.
