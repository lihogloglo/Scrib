# R1: faith-runner movement (`faith_move`)

Study note, slice R1 (README.md). Research only, 2026-10-18. Source:
[tnrjns/faith-runner](https://github.com/tnrjns/faith-runner) (MIT) at
[`2e8b92d`](https://github.com/tnrjns/faith-runner/tree/2e8b92d), crate `crates/faith_move`.

**Clean room.** faith-runner ports a commercial game's movement from decompiled scripts and
disassembled native code, and takes most of its tuning from that game's data. Its comments name
the original classes and clips. This note gives mechanisms and rounded ranges only, and names
only the repo's own modules.

## 1. Shape

- **Bevy-free.** The crate depends only on `glam`.
  - The host passes a `World` and an `Input` each frame.
  - It reads back `feet`, `vel`, yaw/pitch, `state`, the frame's `events` and `view()`.
  - Animation and sound only read the controller, apart from the load-time calibration in §4.
- **The body is an axis-aligned box** (about 0.6 m wide, 1.8 m tall, 1.2 m crouched).
- **Fixed 120 Hz substeps**, with frame dt capped at 0.1 s. Presses and the look delta count on
  the first substep only.
- **One flat state enum** of about 30 variants carrying their own data: ground, air, slide, roll,
  hard-landing stun, wall run, wall climb, ledge hang, vault, traverse, step-up, beam, zipline,
  swing, ladder and others. `substep` dispatches to one function per state, which decides its own
  exits. Priority is the order of the `if`s, and there is no transition table.
- **Overlays sit beside the state**: an attack plays over any move, with its kind picked by the
  state (punch, air, slide, wall-run or vault kick). A 180° turn is driven by a yaw curve. About
  ten cooldown timers sit alongside.

## 2. State machine

**Intent comes from held input, polled every airborne substep.**
- Jump and crouch do nearly everything.
- With forward held hard, vault, step-up and wall climb are tested every substep of a jump or
  fall. *The jump is the vault button.*
- Forward held lightly, plus speed, enables a wall run.
- A ledge grab needs no input: facing a wall with a lip in reach, while not rising fast, is
  enough.

**Priorities.**
- Air, first match wins: fixture grab → vault/step-up → wall climb → ledge grab → wall run. A
  dodge skips all of these until it is falling.
- Ground: ladder → rump slide → beam → slide (a crouch *press* at about 3.5 m/s along the view)
  → crouch → step-up → springboard run-in → buffered jump (dodge if the stick is fully sideways,
  else springboard, else plain jump) → locomotion → vertigo edge check.

**Buffering and cooldowns.**
- The jump buffer is about 0.15 s. Every state that consumes a jump clears it.
- Coyote time is about 0.12 s. The code marks it as a guess: the original has none.
- Each move has a redo cooldown (0.3–3 s) so one feature can't re-fire every frame.

**Commitment.**
- *Physics states* (ground, air, slide, wall run/climb, swing, zipline, beam) exit on any substep
  where their conditions fail.
- *Traversals* (vault, step-up, pull-up, transfer, springboard, ladder entry and exit, takedown)
  are planned and validated in full at the start, then run with input ignored. Attack can still
  join a vault (a kick on the way down).
- *Stuns* (hard landing, lying down, stumble, vertigo) ignore move and look input for a set time.
- Every move carries look limits: a pitch range and a yaw half-range around the yaw it started
  with.

**Context memory** across moves: take-off kind, pre-jump speed, take-off point, fall peak,
wall-run chain count, last wall normal, "climbed this airtime". Each move reads what the last one
left, which is why chains feel coherent.

## 3. World queries

**The interface.** `World` (`world.rs`) has two calls:
- `sweep(half, start, delta)`: the first hit (fraction, normal, point), ignoring solids the box
  starts inside.
- `overlaps(aabb)`: whether anything solid is inside a box.

It also offers `fixtures()` (cables, poles, beams, doors, soft pads, ladders). Three worlds
implement it: an exact box world with a grid, a triangle world with a skin, and a layered world
for moving collision. Every probe is built on the two calls.

**Collide-and-slide.**
- Up to three contacts. Walkable normals (y ≥ 0.7) are followed.
- A lip of a few centimetres lifts the box over it.
- A step-up retry goes up, across and down.
- The drawn mesh lags up to about ±25 cm behind the feet and eases back. Our drawn-height offset
  does the same.

**Walls, tops, ledges.**
- Body-slice sweeps at one or two heights give a wall's normal and gap.
- Repeated thin downward sweeps list the walkable tops under a point.
- A ledge is a top just inside the face, within the reach band, with crouch room on top and hand
  room on the near side.

**Vault plan** (`vault.rs`).
1. Traces climb from the ankle to find a face within the distance covered in about 0.4 s.
2. The full view direction, pitch included, must face the face.
3. The top must be under about 2 m, and the hand plant at most about 0.4 s away.
4. *Over vs onto*: it is *over* if the far side lies within the distance the current speed
   covers in about 0.2 s (clamped to about 0.6–1.8 m). A thin rail needs a jog; a deep block is
   mantled.
5. A floor search beyond the far side decides between landing and vaulting into a fall.
6. A standing box must fit at the end, and a crouched box swept across the top and down must be
   clear.
7. The first matching row of a small variant table wins. Each row has:
   - a height band and an over/onto flag;
   - a vertical-speed band (high variants only while still rising);
   - a speed cap and a maximum time to contact;
   - an exit-speed clamp;
   - three phase times;
   - hand-plant and over-point offsets from the lip.

**Wall run** (`find_wallrun`). It searches forward, or sideways if the player strafes toward a
wall near the take-off. A wall qualifies when:
- the approach is between nearly parallel and about 57° (squarer is a wall climb);
- it is at least about 1.9 m tall and vertical (two traces with matching normals);
- the body is moving into it;
- it is not the last wall;
- when falling, there is no ground under the path for the next half second.

Forward reach grows with sprint. Exactly parallel finds nothing, and a test pins that.

**Wall climb**: still rising from a real jump, facing the wall within about 33°, and the wall
continues to about 1.8 m.

**Springboard**: it looks 1 s ahead at the current speed for a step of a set height (±20 cm),
then a 0.8–1.5 m block about 1.1 m behind it. It then **commits early**: a run-in that ignores
input and aborts if blocked, then two foot plants and a launch.

**Fixtures from raw collision** (`fixtures.rs`): thin shapes in host geometry are classified as
cable, pole, beam or drainpipe by thickness, length, slope, clearance and drops.

## 4. Who owns the motion

**The sim always owns the feet; animation never moves the character.**
- **Physics** owns ground, air, slide, roll, wall run/climb, swing (pendulum), zipline (1-D) and
  beam (lean model).
- **Planned interpolation** handles the traversals:
  - A vault is piecewise-linear from start to hand plant, to over-point, to end, over its phase
    times.
  - Pull-ups use a smoothstep where height leads ("hands first").
  - Shimmy steps follow a measured progress profile.
  - Velocity is zero throughout. At the end an **exit velocity** is set (for a vault, the last
    leg's distance over its time).
- **Clip calibration** (`faith_anim/src/rig.rs`, `Rig::tune`). When clips are installed, at load:
  - clip lengths replace the pull-up, roll and attack durations;
  - root-yaw curves, sampled at 60 Hz, become the 180° turn curves;
  - root-position curves become the ladder entry and exit paths.

  Without clips, guessed durations and a smoothstep are used. The sim stays deterministic, and
  moves last exactly as long as their clips.
- **Re-anchoring.** The animation side shifts travel clips so the clip camera sits on the
  gameplay eye. The clip's root motion is cancelled against the sim's path (`9470d75`).

## 5. Momentum and speed thresholds

**Ground speed builds up** (`locomotion.rs`).
- A designer curve of speed against running time: about 4 m/s within 0.5 s, top speed (about
  7 m/s) at 7 s.
- It is converted to acceleration as a function of current speed, so the run resumes the curve
  from any speed.
- A steering term rotates velocity onto the input while keeping its speed, so direction changes
  carry momentum.
- **Turn damping** in proportion to how far the *view* turned this frame means whipping the
  camera sheds sprint.
- Speed above run speed is "energy" that drains when strafing.
- Releasing brakes exponentially. A tap-stop (release within 0.15 s of starting) stops dead.

**Air.**
- Air control is tiny (about 1.5 m/s²), so jumps are committed.
- Gravity is about 1.7 g: a jump of about 1.2 m and about 0.8 s of airtime.
- Every jump is specified by height (`speed_for_height`).

**Speed gates.**
- Slide: about 3.5 m/s along the view.
- Wall run: about 2 m/s.
- Door barge: about 2.5 m/s (slower is a kick).
- Rails: the depth must clear at the current speed.
- Wall-climb boost: grows with run-up speed and vertical speed.

**Dodge, hop tax, slide.**
- A dodge keeps about 30% of velocity plus a fixed sideways launch.
- A plain jump adds about 1 m/s forward, and its landing caps speed slightly below the pre-jump
  speed. Bunny-hops lose speed; dodges and wall jumps are exempt.
- A slide ignores move input and steers toward the view under one-tenth friction. It lasts about
  1.3 s from a sprint, ignores jump, and halves speed on exit.

## 6. Landing, roll, fall

**Fall height from the arc's peak sets the landing, not impact speed.**

| Fall | Landing |
|---|---|
| About 2 m | Rollable |
| About 5 m | Hard landing: dead stop, move and look disabled for about the clip length, view levelled |
| About 10 m | Fatal |

**The roll is a timing skill.** A crouch press counts only within about 0.2 s before touchdown,
and re-arms only 0.6 s after the previous press, so mashing fails. The roll keeps at least run
speed.

**Special landings.**
- A soft pad turns a hard landing soft.
- Falling toward a soft pad sets a "brace" flag for animation.
- A 180° turn in the air lands the character on its back.

A normal landing is not a state. It is an event, `Land{impact, fall}`, and animation scales a
small additive dip by it.

## 7. Wall run, wall climb, ledge

**Wall run.**
- Entry keeps the speed along the wall. If still rising, the character is lifted to about 1.7 m
  above the take-off, under about half gravity and capped by headroom.
- Input adds no acceleration. Speed decays slightly, and a small push into the wall keeps contact.
- It ends:
  - when falling faster than about 5 m/s;
  - when the wall ends (re-probed every substep);
  - on crouch or pull-back.
- It lasts about 1.2 s.

**Wall jump.**
- The further the player looks away from the wall, the harder the push out and the less speed
  along the wall is kept (down to about 10%).
- *Turn* eases the view to face straight out and arms the full push-off: this is how wall-to-wall
  chains work.
- A chained run gets no lift and its gravity is multiplied by the chain count, so chains decay.

**Wall climb.**
- A boost of about 1.3 m, plus up to about 0.6 m for a sprinting run-up.
- It grabs any ledge in reach and ends when rising stops. One climb per airtime.
- *Turn* opens a kick-off window.

**Ledge hang.**
- The feet sit a set distance below the lip. The impact speed is kept for the animation's choice
  of grab.
- After about 0.35 s of settling, the player can pull up onto the highest top within about 0.9 m.
  Sloped roofs work, and the pull-up ends crouched if there is no standing room.
- Transfer: up to a lip about 1.4 m higher.
- Shimmy:
  - fixed steps that always complete;
  - only while looking within 90° of the wall;
  - goes round outside corners.
- Looking past 90° makes jump a backward kick-off.

## 8. Tuning

`tuning.rs` is one `Tuning` struct in sections:
- body, ground (nesting the locomotion port), air;
- wall run, wall climb, ledges;
- slide, landing, dodges, turning;
- springboard, beam, zipline, swing, melee.

Every field's doc comment states its source: about 100 from the original, about 10 marked
"guess". The vault table lives in `vault.rs` and camera feel in `camera.rs`. A few host switches
cover auto step-up height, kill height and ladder release. Clip-derived fields are optional and
filled at load.

## 9. Scripted-input tests

About 120 headless tests. The README asks for them after any tuning change, to "tell you if a
change made something impossible".

**Harness** (`course_tests.rs`). `Sim` spawns at a checkpoint, steps at 1/60 s, and collects
events and deaths. `run(secs, |controller, t| Input)` takes **input as a function of controller
state, not a timeline**. Helpers:
- a `once` latch ("press jump when the feet pass 0.85 m before the rail");
- `toward_x` (steer toward a lane);
- `AngleIn` (turn the view 25° into the wall once airborne).

Scripts survive tuning changes that shift timing.

**Assertions** are outcomes: the expected events happened, no deaths, the run reached the next
checkpoint.

**Coverage.**
- One test per rule, including **negative tests**: no wall run when parallel, jump does nothing
  mid-slide, a dodge can't be steered.
- Every route of every course map, alternatives included.
- Springboard near-miss lanes that must *not* trigger.
- `mesh_tests.rs` repeats the moves on scenes **rotated 27°**, plus a randomized **differential
  test**: box and triangle sweeps must agree to a millimetre.
- "Never ends up inside a rock".

Levels are data in code (`greybox.rs`, `rooftops.rs`), shared by the app and the tests.

## 10. Failure modes from the history

| Problem | Commit | Fix |
|---|---|---|
| Top-only stairs and floor seams stopped the player dead | `5f2b8bf`, `150292b` | A lip lift in slide-move; a level move into a flat top's edge counts as a step |
| Box normals at step edges looked like slopes and started rump slides | `150292b` | Check that the ground really falls away |
| The vault camera lurched as the clip faded | `9470d75` | Re-anchor travel clips to the eye |
| Triangle collision has no inside, so an end point inside a rock passed | `c893608` | A path sweep, plus a holding skin |
| Turn clips' root rotation was read in reverse | README | (not stated) |

Still missing: ledge hand IK and foot placement on uneven ground.

## 11. Situations

| Situation | Coverage |
|---|---|
| Roof-to-roof jump | **Fully covered**; the best reference. Momentum, buffer and coyote time, gaps of 4 m, 8 m (wall run) and 12 m (girder), landing classes, roll timing |
| Ledge grab, mantle, climb | On box-like geometry: auto-grab, wall climb, pull-up onto flat or sloped tops, shimmy and corners, ladders and pipes. No IK and no free climbing (that is R3) |
| Sneak | Nothing beyond crouch-walk (about 1/5 of top speed) and the vertigo stop at drops |
| Sword fight | No. Unarmed attacks chosen by context, a combo window, a target score from facing and distance, limb-sweep hits. Its takedown aligns both actors to spots relative to the clip, which is useful for paired animations |
| Sit, door, lever | Doors only (barge, kick) |
| First person | The movement's eye, look limits per move, camera effects (the body is R2) |
| Third person | None, though only the look limits assume first person |

## 12. For us

Our baseline:
- `sf_controller`'s `KinematicMover`: Holden springs at frame rate, collide-and-slide,
  drawn-height offset, root-motion blend.
- `ContextProbes`: classify into `Affordance` by height and depth, and give warp and hang targets.
- Traversal actions are planned for M4.

**Adopt**
1. **Course tests with scripted input**: input closures that react to controller state, outcome
   assertions, near-miss lanes, rotated variants, and a box-vs-mesh differential test.
   - They run on greybox levels stored as data, shared with an example app.
   - They live in `sf_controller` against `StaticWorld`.
   - R11 should require them for every traversal move.
2. **Fixed substeps for the player**, with presses consumed on the first substep. Holden's closed
   forms cover springs, but not arcs or probe-gated decisions.
3. **Jump buffer, coyote time, per-move redo cooldowns.**
4. **Time-to-contact gating**: probe reach scales with speed (vault within about 0.4 s,
   springboard within about 1 s).
5. **Plan, validate, commit**: end pose free, a swept path across the top and down, an exit
   velocity.
   - `ContextHit` gives the lip and landing. The path sweep and exit velocity are missing.
   - This matches AC1-rs's end-pose validation.
6. **Clip calibration as a bake step**: clip lengths and root yaw/position curves become gameplay
   timing, with defaults when clips are missing. The sim stays authoritative whatever clips a
   user brings (R9), and warping reconciles the rest.

**Adapt**
7. **A move schema, not an enum.** Each move is data plus a rule:
   - entry: state, held input, probe result, speed and vertical-speed bands, facing, time to
     contact;
   - priority;
   - motion owner (physics, planned phases, clip curve);
   - exits and exit velocity;
   - redo time;
   - look limits;
   - input-disable windows.

   faith-runner's ordered `if`s list the fields. This is how mods add moves.
8. **`Affordance` needs speed and vertical speed**, not just height and depth.
   - Over vs onto depends on depth against about 0.2 s of travel. High vaults only while rising.
     Facing comes from the view or intent.
   - Add *WallRun*, *WallClimb*, *Springboard* and fixture kinds. The wall checks become a
     `WallAhead` query.
   - Keep the NPC gating (A0, blocked only), but poll the player on every airborne substep.
9. **A momentum mover mode for the player**: speed build-up curve, view-turn damping, hop tax,
   slide halving. Holden springs stay for NPCs.
10. **A thin-fixture classifier** in a bake step, writing `sf_interaction` points, as with
    AC1-rs's holds.
11. **Attack overlays chosen by context** (vault kick, slide kick) for M9.

**Reject**: the box body, the 125 KB single-file state machine, and the absence of IK. Our
warping and full-body IK replace "clip and path must agree".

**Spec gaps exposed**
- locomotion.md has **no jump, air state, air control or landing**, only gravity and a maximum
  fall speed. It needs:
  - jump by height;
  - fall-peak tracking;
  - landing classes as data: normal, roll window, hard, soft, lethal;
  - coyote time and a jump buffer.
- There is no player-feel model: no build-up, momentum carry or slide.
- Context probes lack wall-run and climb walls, speed and time-to-contact gating, and the
  across-the-top path check.
- Per-action look limits for first person are missing.
