# R5: sekiro-rs mobility and behavior graph

Status: done (r5, 2026-10-18).

Source: https://github.com/AKJama/sekiro-rs (Apache-2.0), read at `2ad2539` (2026-10-18):
`docs/MOBILITY.md`, `docs/HKS-INTERFACE.md`, `docs/FIDELITY.md`, `docs/PROGRESS.md`, the crate
`sim` modules `behavior`, `player`, `character`, `tae`, `input`, `camera`, `docking`, `wire`, the
`formats` graph loader (concepts only), `examples/root_motion.rs`, tests `it/mobility.rs` and
`it/camera_turn_movement.rs`, and commits #3, #5, #90.

**Clean room.** sekiro-rs runs Sekiro's own graph, script and clips from the user's install, and
much of `MOBILITY.md` was traced from the executable (partly via FwLo's MIT RustSekiro). This note
gives mechanisms and rounded *ranges* only: no script function names, query ids, node class names,
clip ids, param tables or constants. sekiro-rs's own modules are named.

## 1. The three-way split: script decides, graph plays, clip events gate

FromSoftware's character runs on three layers, and sekiro-rs reproduces all three:

- **Animation graph (Havok Behavior).** A tree of generators: state machines (each state owns a
  generator), layers (weights bound to variables, toggled by events), blenders, selectors (index
  bound to a variable), a custom **clip selector** mapping an *animation id* plus an
  equipment-derived *set prefix* to the clip (one "attack 1" state plays the katana or spear
  variant), clips (once, loop, ping-pong), a reference to a shared sub-graph (all humanoid NPCs
  share one), and modifiers (look-at twists, foot IK via script callbacks, a custom "docking"
  generator that moves the body along an edge). Typed variables bind to node members. Transitions
  carry an event, a blend effect and flags, and **no conditions: FromSoftware leaves them empty.**
  The graph makes no decisions; it is playback addressed by events.
- **Character script (HKS, a Lua dialect).** One large script per archetype. The engine calls a
  global update, then an update hook for *every active state* (over a thousand have one), plus
  activate/deactivate hooks. Nearly every update runs: upkeep, **pick the next behavior** (an
  ordered "can I start X now?" list: damage, death, fall, then actions by priority), leave the state
  when its clip ended. It reads the world through a narrow query interface (~1,600 call sites: input
  requests, hold time, ground/fall, search results, lock-on, damage, "is window *R* open") and acts
  through commands (~700: fire event, set variable, face direction, run a search, show a prompt,
  pre-compute root motion, constrain lock-on turning).
- **Per-clip event tracks (TAE).** Timed windows: action flags (accept and cancel per action class,
  disable turning), turn speed, velocity launches with an air-control mode, status effects whose
  *reference id* is how the script asks "is this window open" (the most-called query),
  invulnerability, camera presets, weapon location and sheathing, look-at targets, reach scaling,
  authored blend-in time, and grapple windows (sync, look at, gravity off, travel).

**Who decides what.** The script decides *every* transition and picks the clip variant (it reads
an engine-written angle and sets a selector variable). The clip's windows decide *when* the script's
choice is allowed (input is only reported inside execute windows, an air ledge-catch only inside a
clip window that permits it, a wall jump only inside its window). The engine decides *where*:
world searches, docking, physics, root motion, steering. Graph structure decides only *how* to blend
and which states run in parallel (upper/lower body machines that hear the same event; a "sync start"
mode makes a newly active machine start in the state its sibling is in).

**Frame order** (sekiro-rs `player.rs`, `character.rs`; their reading, not traced): input gated by
the *previous* step's windows → engine writes variables (stick level, turn/move angle, twists, edge
angle) → script updates → queued events applied, deactivate/activate hooks may fire more (settle,
up to 8 rounds) → clip ends send end events, settle → clocks advance (an end reached now fires now)
→ read this step's windows → move the body (root motion, steering, launches, gravity).

**Input buffer: two windows, no timer** (`input.rs`, from FwLo's trace). A press is *latched* only
inside an accept window for its action class (mashing before it does nothing), and *reported* to the
script only inside an execute window; it waits with no expiry until an action starts, which clears
the buffer. Clips with **no action flags at all** (idle, locomotion loops) accept and execute
everything; any flag makes the clip "restricted". Combat details are R6.

## 2. Root motion and turning

- **Translation from the clip, yaw from the engine.** The full-body clip's root delta moves the
  body; the body's yaw is steered separately toward a target at the clip window's turn speed (high
  by default, several hundred °/s; zero or a disable flag during commits; up to a few thousand °/s
  in aim windows such as grapple chaining). Locked on, the steer target is the enemy; free, it is the
  stick; a "fixed rotation" window stops tracking (what makes side-steps and late swings dodgeable).
- **Turning clips.** Each step's root delta is rotated into the root's own frame (undo the clip's
  yaw so far) before applying along the facing. Without this, a sprint start that turns 90° while
  travelling carried the body sideways (failure, fixed).
- **Free move start: snap the body, unwind the look.** From standstill the body yaw snaps to the
  stick and the difference becomes a *lower-body twist* the renderer unwinds at the default rate;
  the script sees "stick ahead" and plays a forward start (or a turn-and-go above ~135°). Before,
  it picked strafe starts in free movement and the character slid sideways (failure).
- **Locked-on strafing.** Four directional clips with ~10° class hysteresis; the residual angle
  becomes a *master twist*: legs and travel turn onto the stick, the spine counter-turns to keep the
  chest on the target, root motion is rotated by it. Diagonal dodges tilt the nearest of four. A
  locked dodge disables turning first, then a turn window re-faces the target.
- **Turn angle from travel, not facing**, or a locked back sprint can never start (failure).
- **Pre-computed arrival.** For climbs, hangs and grapple starts the script asks the engine to
  evaluate a clip's *total* root motion so travel can be scaled onto the target: warping by command.
  sekiro-rs projects the root track onto its total displacement (progress 0..1) and fits that to
  the real ledge top.
- **Hand-over continuity.** A clip entered on the previous clip's end takes its first `dt` of
  motion, else every start→loop seam hitches (failure).

## 3. Mobility

**Shared world data: edges.** Ledges, cover walls and grapple anchors are all *authored segment
chains* with a kind and a variation (hang with/without foot hold; cover look permissions and peek;
grapple rideable or not, recognition cones, landing offset). One edge manager serves grapple and
docking; thousands per region; doors carry cover edges. Edges have no normal: sekiro-rs derives the
outward side from collision.

**Search: request, then query.** Each frame its state permits, the script asks the engine to run
the hang (cover, air-catch) search; the next query returns yes/no, a reach type and the edge
variation, and the edge's angle to the facing lands in a graph variable. Thresholds are engine-side
(sekiro-rs placeholders: reach ~1 m, facing within ~45°, height tolerance ~0.4 m). Prompts are a
separate command.

**Ledge hang / shimmy / climb.** Entry clip by the edge angle's quadrant × foot-hold variation.
Hanging, the body is **docked**: closest point on the edge + an offset, gravity and pushes off. The
offset must be *read from the hang clip* (hands ~1.5 m above and ~0.6 m in front of the root);
guessed placeholders put the body in the wall (failure). Shimmy takes stick input only in a side
cone (roughly 30–150° off facing), stops at chain ends by obstacle probes, and moves by the clip's
root distance projected onto the segment. Corners are separate moves. Climb-up needs a top-surface
test and fits its root motion (~1 m in, ~1.5 m up) to the real top; late cancel windows. Drop on
dodge. Air catch only inside a clip window that permits it, grab held, lip ~1–2.5 m above the feet.
Wall jump: chest-height probe ring, one per flight (script counter reset on landing), launch from
the clip's velocity event. Hang clips sheathe the sword by event; cover clips don't, so sekiro-rs
added a "sheath while docked" policy (failure: blade through the roof).

**Grapple.** Search over placed anchors: distance bands (a few m to ~15 m to act, ~25 m to find;
area overrides up to ~50 m via region triggers), screen-space filter box and soft lock, per-anchor
recognition cones, chest line of sight; sekiro-rs ranks by angle from view centre, then distance.
Shoot windows: rope tip flies, the character looks and turns at a high rate toward the anchor,
gravity off, then a launch event. Start clip by elevation band (~15° and ~50°). **Flight is a solved
ballistic arc** under lighter gravity: initial velocity from displacement and time, horizontal
re-solved each tick, **arrival time = the clip's authored camera-cut event** (~0.7–0.8 s), so camera
and body agree. Rideable anchors land on a "virtual perch" (anchors sit beside the walkable top);
non-rideable ones fly to an offset below and beyond and keep velocity. Search continues in flight
for chains; leaving into an attack keeps momentum. Skinned rope failed visually; a line ships.

**Jumps, air control, falls.** Normal gravity is heavy (roughly 2–2.5 g); grapple flight lighter
(~2 g). Air control (FwLo's description): horizontal air velocity decays slowly; the stick adds
acceleration split along and across it; across always applies, along applies fully when braking and
when accelerating only up to a small cap: **a fast launch can be steered and braked but never pushed
faster.** Some launches are uncontrollable (fall-death clips). No double jump; the wall jump is the
second jump. Landing clip by fall height band (soft; short stiff from a few metres; long stiff from
~20 m); death height per surface material (deep water much higher); voids handled by authored
"event wall" volumes that fade out and return the player to the last safe standing spot (failure:
helper collision let the player stand on event walls; fixed by honouring collision filters, #3).
Units matter: a cm-vs-m mismatch silently disabled all stiff landings (failure).

**Wall hug (cover).** Same search and docking as hanging; entry clip by angle; slide along the
segment by root distance with a ~30° dead cone around wall-normal input (so pushing into or away
from the wall doesn't slide); peek at an end only if the variation allows; exit on the button or
stick away; door edges included. Offset out from the wall ~0.7 m (FwLo).

**Sneaking and crouch.** A script-side toggle that switches locomotion sets, plus a "can I stand
up" ceiling query (stubbed). Stealth, perception and deathblows from behind are not started.

**Swimming.** Deep-water collision triangles are excluded from solid, camera and LOS; the surface
pins the body origin; diving runs between seabed and surface; speeds come from root motion. No
ladders or free climbing in the script (ladders are object interactions).

## 4. Camera

- **Free camera**, stick camera-relative. `camera_turn_movement.rs` checks travel stays within a
  few degrees of the stick through a full camera sweep, an instant flip, every facing × direction,
  quick reverse, sprint quick turn, start→loop, locked strafes and diagonals (written after a "tank
  controls after spinning the camera" report).
- **Lock-on**: range radius, pitch limits that differ locked vs free and lerp with the height gap
  to the target, separate XZ and Y chase rates. Sprinting releases the lock-on facing.
- **Clip-driven camera**: clip events select a preset (distance, FOV, focus height, pitch limits)
  with authored blend-in/out times and clamp look angles; grapples, deathblows, throws frame
  themselves.
- **Obstruction**: sphere push-in plus dithered fade of near-camera translucent geometry (a blind
  with no collision hid the fight). Lags in fast descents (open). World visibility follows the
  collision part last stood on, not the camera (#5: the world vanished after a grapple).

## 5. Failure modes

Besides those above: a clip end processed a frame late let the script see an ended jump wind-up as
"free" and cancel the jump into running (fire end events inside the advance); placeholder search
thresholds made most planned test spots fail first (LOS blocked, wrong side of a ledge).

## 6. Situations

Sneak: crouch set and wall hug with peek; no stealth or backstab yet. Roof-to-roof: heavy-gravity
jump with capped air control, grapple chains, air ledge catch, one wall jump. Ledge: hang, shimmy,
climb-up, drop; no ground vault or mantle (grapple and hang replace them). Sword: R6, same split.
Sit, door, lever: object interactions, not studied. Third person only.

## 7. For us

**Should "script decides, graph plays, clip events gate" be our model? Yes, adapted.** Our pieces
almost have this shape (`sf_anim_graph` plays, clip `Event` windows exist, montages play one-shots),
but the decider must be neither Luau nor one monolithic script per archetype:
- FromSoftware's script runs synchronously **inside** the character's frame with a settle loop. Our
  Luau runs in exclusive slots with events delivered next frame (scripting.md), and Lua is parked; a
  frame of latency per decision is what makes controls mushy. The decider belongs in Rust,
  Bevy-free: an **action layer** (`sf_controller` or a new `sf_action`) evaluating **data-driven
  move rules** (R1/R3's move schema: entry state, input, probe result, speed bands, required window,
  priority) in the same step as the graph, with a bounded settle loop.
- A 1,600-query script is unmoddable; ordered rules are (a mod adds a rule and a clip). Luau later
  gets *hooks* into the action layer, never the per-frame loop.

**Adopt**
1. **Window vocabulary on clips** (animation.md "Events"): accept(class), execute/cancel(class),
   turn rate, disable turn, fixed rotation (stop tracking), launch (velocity + air-control mode),
   gravity off, i-frames, camera preset, equipment location/sheath, look-at target, reach scale,
   warp. Plus the **"restricted" rule**: a clip with no action windows allows everything, so
   locomotion loops need no authoring. Defaults per clip category for mixed-source clips (R9).
2. **Two-stage input buffer** (latch in accept, fire in execute, no timer, clear on start) in the
   action layer; R6 to confirm values.
3. **Yaw separate from root translation**: steering toward a target at the window's turn rate,
   independent of `Drive` root motion, and root deltas rotated into the root's own frame. Layer 1's
   `max_turn` covers warps only; this is needed for every `Drive` clip.
4. **Body snap + lower-body twist unwind** for free move starts, and **master twist** for locked-on
   strafes (legs onto stick, spine back to target) in `sf_anim` procedural ops. Turn angle measured
   from travel.
5. **Docked mover mode** in `sf_controller` (`MoverMode::Docked { edge, offset }`: hang, cover,
   later ladders and beams): position on a segment chain, gravity and pushes off, movement = clip
   root distance projected onto the edge, side-input cone, end and corner probes; offset baked from
   the clip (hand joint vs root), not a constant.
6. **Edges as interaction data** in `sf_interaction`: segment chains with kind and variation flags
   (foot hold, look permissions, rideable, landing offset, recognition cone), authored or baked
   from collision (R3's hold index). Grapple anchors fit the same type for mods.
7. **State-gated searches**: probes run when the state or a clip window asks (air catch only in its
   window), not only when the trajectory is blocked.
8. **Ballistic flight with authored arrival time** as a warp mode ("arc"): launch velocity from
   displacement and a clip-given time, horizontal re-solve each tick, collision sweep.
9. **Air control** "steer and brake, never past launch speed"; landing by fall-height band; death
   by surface; void return to the last safe stand.
10. **End events inside the advance**, and a clip entered on a previous end takes its first `dt` of
   motion (`sf_anim_graph` tests).
11. **Port the camera-relative movement test suite** to `sf_controller` + graph tests.

**Adapt**
- **Clip selector by id + set** (weapon category, race, sneak set): our graph can do it with a
  selector bound to a parameter or with late-bound slot clips; specify one "clip set" mechanism
  rather than duplicating states. Morrowind's weapon-type groups need exactly this.
- **Global any-state transitions that reach nested states** (one "hang" event from anywhere): we have
  `"*"` per machine; specify nested reach, or route such actions through full-body montages.
- **Clip-driven camera presets** with blend times: there is **no third-person camera spec** at all
  (character.md covers first person only). Write one: free orbit, lock-on (range, pitch limits by
  height gap, chase rates), clip presets, obstruction push-in plus dither fade.

**Reject**
- Event-only transitions with no conditions: keep our param/phase conditions for locomotion blends;
  only *actions* go through the decider.
- Instant switches (sekiro-rs does not model blends); we keep inertialization.
- Hard-coded searches with untraced thresholds: ours scale with rig measures.

**Where our spec is wrong or thin**
- **animation.md "Prior art"**: sekiro-rs is Apache-2.0, not "nothing specific about animation";
  it is the best open description of the FromSoftware split.
- **animation.md "Shape"** ("authored clips plus warping plus IK") misses the decider layer and the
  window vocabulary, which are what make it feel tight.
- **Root motion modes** lack rotation control (turn-rate windows, fixed rotation, steer target) and
  the twist ops.
- **Slots/montages** lack **window-gated interruption**: interrupt only where the clip's cancel
  windows allow the requested class (today: priority only).
- **locomotion.md**: no `Docked` mode; probes lack state gating, cover and arc targets.
- **scripting.md** should say Luau never runs in the per-frame action loop.
