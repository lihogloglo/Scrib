# 14. Rewrites re-read, first-person melee, industry update (2026-11-12)

Research for the animation audit (`15-audit-and-plan-2026-11-12.md`). Adds to notes 01-13; does not repeat
them. Marks: [S] read this session (`gh api`, commit given), [K] known practice, not re-fetched this session,
[I] our inference. No web search tool was used; UE/Unity/shipped-game statements are [K] unless marked.
Clean room (superfargoth CLAUDE.md rule 3) applies to every repo below: they run on the original game's data
and several were checked against disassembly, so we take **technique and structure, never tables, constants
or code derived from a game binary**, whatever the repo's licence says.

## 1. The five owner references

| Repo | Commit read | Licence | Language, engine | New since our notes |
|---|---|---|---|---|
| AKJama/sekiro-rs | `970cde8f` (pushed 2026-11-02) | MIT OR Apache-2.0 (both files at root; GitHub shows Apache-2.0) | Rust, Bevy 0.20 | re-read 2026-11-11 by `locomotion-rethink` §2; nothing newer |
| chasmlol/spud | `8c990420` (2026-11-07) | MIT | **C++** (Vulkan, Jolt), not Rust | new to us |
| Banned445/AC1-Movement-Rewritten | `e3c83883` (2026-11-01) | MIT | Rust | ~30 commits since note 03/04 |
| tnrjns/faith-runner | `2e8b92d8` (2026-10-15) | MIT | Rust, Bevy | none (last push predates notes 01/02) |
| Nacholmo/sinhonor | `d3071eb8` (2026-10-13) | MIT OR Apache-2.0 | Rust, Bevy; `dis_motion` depends only on `glam` | new to us |

All five licences are GPL-3-compatible. None ships clips; all read the user's install.

### 1.1 spud (Skyrim SE) [S]

What it is: an engine that imports Skyrim's data into its own formats and runs it. Animation is a **Havok
Behavior runtime written from behaviour specs**: `src/anim/graph.cpp` (60 KB), `state_machine.cpp` (88 KB),
`generators.cpp` (103 KB), `modifiers.cpp` (104 KB), `clip.cpp`, `ragdoll.cpp`. Like sekiro-rs, the locomotion
logic (start, stop, turn, 1st/3rd person sets) lives in the game's behaviour files, not in the repo.
`docs/MOVEMENT.md`: the player is a Jolt character controller on a 60 Hz fixed tick with render interpolation,
simulated one tick ahead on its own thread; mouse look is applied per frame outside the simulation. Speeds come
from the game's movement-type records; acceleration and stop times are placeholders (0.25 s / 0.18 s).

How the parts we care about work (structure only):
- **A frame is update, drain events (up to 5 rounds), generate.** Events raised by transitions and clip
  annotations are consumed inside the same frame, so a state chain settles before the pose is made.
- **Foot IK is a driver outside the graph, steered from inside it.** The graph writes a per-leg *gains* track
  and an "on fraction" (a control-data track); a separate foot-IK driver reads that track after generation,
  ray-casts through a host callback, solves and reports how far it moved the root. So **each state or clip can
  turn foot IK down or off** (attacks, jumps, staggers) without the solver knowing about states.
- **Modifiers present:** foot IK (two forms), look-at (with a limit cone), twist chain, "direct at" (aim a bone
  chain at a target: the bow and head-tracking), rotate-character, limb IK, damping, timers, event-range (fire
  events from a variable's range), pose matching against the ragdoll for get-ups, synchronized clips for paired
  animations (a shared scene object set before the participants' generators activate), mirrored skeleton.
- **First person:** eye height comes from the camera node of the game's separate first-person skeleton
  (`MOVEMENT.md` table): Skyrim keeps technique A (a second skeleton and clip set). First-person head bob is
  listed as a known gap. `src/game/combat.cpp` (67 KB) and `app_kill_camera.cpp` exist; not read in detail.

Take: (1) the **foot-IK gain as an animated control track** (we gate with one bool `on_ground`; a per-clip
`ik.feet` curve, default 1, lets start/stop/attack clips fade the lock without code); (2) **same-frame event
draining** as a rule for our graph (check ours: a transition that needs two frames to settle shows as a one-
frame wrong pose); (3) paired-animation "scene" object as the shape for takedowns (combat C6). Not portable:
C++, and the node semantics are Havok's. Nothing to port.

### 1.2 sinhonor (Dishonored) [S]

What it is: a first-person **motion kit** (`crates/dis_motion`: `controller.rs` 67 KB, `blink.rs`, `camera.rs`,
`melee.rs`) that knows nothing of Bevy, plus a demo (`sinhonor_demo`: `hands.rs`) that loads the game's
first-person arms, sword, 385 first-person clips, sounds and particles from the install.

- **Body: arms only** (technique A). The view **is the skeleton's camera joint**, so the arms sit exactly
  where the animators framed them at the game's FOV. Arms and sword are drawn by a second camera on their own
  layer (the Bevy example pattern we rejected for FP-2 on sun-shadow cost; fine for an unlit-shadow demo).
- **Layers:** a base layer for the sword hand and body motion (idle/walk/run/sprint/sneak/slide/swim/mantle/
  land/swings/assassinations), a left-arm layer for the power hand; changes crossfade over a short fixed time.
- **Camera motion from clips:** head bob is the game's additive head cycles picked by speed and direction;
  mantles and kills move the camera along the clip's camera joint; landing dip is a spring. Lean is a head
  point on a lever about a low pivot.
- **Sword** (`dis_motion/src/melee.rs`): each swing has four times read from the clip's notifies: when the
  blade can hit (start + duration), from when a press is kept for the next swing, from when the next swing may
  start, and the end. Forehand/backhand alternate inside a chain timeout; crouched gives a sneak attack; a blow
  that would kill plays a finisher; **the blade striking the world swaps the swing for a recoil clip** and
  shakes the camera. The hit test is **a box swept from the camera along the view** for the reach (longer when
  moving forward fast), not the blade's animated path. The kit decides what is hit, the host applies damage.
- **Paired kills:** the victim is moved to where its half of the paired clip expects it (Corvo is not), a room
  probe falls back to a generic stab, the view follows the camera joint and control returns on the clip's
  release notify.
- **Mantle:** box sweeps at rising heights find the first clear lip; low lips are a three-step move with a
  short view blend, medium/high ones follow the clip's root motion with its rise stretched to the lip height
  (motion warping, as our `WarpTarget`).

Take: the **kit shape** confirms ours (`sf_action` Bevy-free, host applies results). For first-person melee:
(1) **world-hit recoil** as a move outcome (Dark Messiah, Mordhau and Chivalry do the same; we have no "blade
hit a wall" result today); (2) reach that grows with forward speed (cheap lunge feel); (3) the camera joint as
the eye for full-body moves. Reject the camera box sweep as our hit test: we already sweep baked socket tracks
(combat.md "Hit and hurt volumes"), which is the Souls/Mordhau standard and works in third person. MIT/Apache
code may be read; `melee.rs`'s structure is original, its numbers are the game's (do not copy).

### 1.3 AC1-Movement-Rewritten, changes since notes 03/04 [S]

Commits since 2026-10-18 (from messages; code not re-read): vault over wall tops (auto and manual hold), a
blend speed cap and turn rates, ladder and beam fixes, "the animation runtime's slot and blend rules", limb-IK
markers settled ("foot lift removed"), and two that matter to our stop problem: **"the stop and run take-off
foot ... take the pose's leading foot"** and "the exact reversal's pivot side". So AC1 too picks stop, start and
pivot clips **by which foot leads in the current pose**, the rule `locomotion-rethink` §3.1 proposes. Many
commit messages cite binary addresses "verified in IDA": treat the repo as reverse-engineered; ideas only.

### 1.4 sekiro-rs and faith-runner

No new material. Standing takes (notes 02, 05, 06, `locomotion-rethink` §2): FromSoft foot IK is visual only
with asymmetric pelvis gains and no foot lock; body yaw snaps to the stick and a visual twist unwinds; the
Mirror's Edge driver lets the sim drive the playhead and bakes or authors a camera joint per clip.
**Correction to `locomotion-rethink` §2 and note 05:** sekiro-rs's licence files are MIT and Apache-2.0 at the
repo root; GitHub's API reports only Apache-2.0. Cite it as "MIT OR Apache-2.0" (unchanged in effect).

## 2. First-person melee: what the references ship

| Game | Body | Camera | Hit test | What sells it |
|---|---|---|---|---|
| Dark Messiah (Source, 2006) [K] | full body awareness: legs and torso visible looking down; arms/weapon as a view model sharing the player's animation state | view punch on hits and swings; camera roll on power strikes; first-person death and finisher cameras | melee traces in front of the view during the strike window; power strikes charged with a direction key choose the swing | **kick** as a dedicated button: short stagger, physics impulse, environment kills (spikes, ledges, fire); stamina-free but with a recovery; adrenaline finishers; props thrown by physics |
| Dying Light 1/2 [K] | true full body, head hidden, camera on the head joint with stabilisation | strong camera animation per move, limited by a comfort setting | weapon-path sweeps; hit reactions by body part; dropkick and vault-kick from traversal states | traversal and melee share one body; hit-stop and directional reactions |
| Chivalry 2, Mordhau [K] | 1P arms/torso mesh distinct from the 3P body but driven by the same move clock; the 3P body is authoritative for hits | view yaw/pitch **drags the swing** (the spine follows view pitch; turning accelerates or delays the hit) | tracers along the blade from the 3P pose, host-validated | swing direction from mouse movement or keys, chambers/ripostes, world hits stop the blade |
| Skyrim/Starfield [K]; spud [S] | separate 1st-person skeleton and clip set | camera node on the 1st-person skeleton | reach cone/ray at an animation event | technique A; no full body (mods add it) |
| Dishonored [K]; sinhonor [S] | arms only, camera joint | clip-driven camera, lean lever | camera box sweep in a notify window | recoil on world hit, paired kills |
| Kingdom Come [K] | true full body | head-joint camera with heavy stabilisation | physical weapon collision | directional stances |
| UE 5.5+ "First Person Rendering" [K] | any primitive tagged first-person gets its own FOV and scale in the vertex stage, one pass; world-space copy for shadows | - | - | the engine-level answer to FOV and wall clipping; our FP-2 matches it |

What a Dark Messiah-like system needs that a view model alone does not give:
1. **The upper body follows the view pitch** (aim offset or spine pitch split over spine/chest/clavicles,
   usually 60-100% of the pitch): looking down at a crawling rat, the swing must go down there. Every full-body
   first-person game does this; shooters call it aim offset.
2. **The eye rides the head joint, stabilised**: position from the head (low-passed), rotation from the view,
   with a per-move amount of clip camera motion (Dying Light's comfort slider). Otherwise the body animates
   under a floating camera: kicks, hit reactions and mantles look detached and the neck stump or chest clips
   the near plane.
3. **Moves authored or checked from the inside**: third-person attack clips often swing outside a 90° view or
   through the camera. Standard fixes: a first-person arms layer (per-move override clips), an additive
   "bring the weapon into frame" pose, or hand IK toward a view-space target during the windup.
4. **A kick**: one button, short committed move, foot socket sweep, a stagger tier plus a physics impulse on
   the target and on props. It is the cheapest high-value Dark Messiah mechanic and reuses our hit pipeline.
5. **Blade stops on the world** (recoil move) and **view punch** on hits given and taken.
6. **Host-authoritative shape**: view pitch is already input (`Command.view`); the spine pitch is presentation
   but the socket sweep must use the same pitch, so sweeps compose the baked socket track with the commanded
   view pitch on the fixed tick (Mordhau/Chivalry validate the same way).

## 3. Locomotion standard, restated for the audit [K]

- **UE5 Lyra**: capsule-driven. One clip set per gait whose authored speed is close to the gameplay speed;
  the play rate is clamped near 1 (typical clamp 0.8-1.2) and **stride warping takes the remainder**; outside
  the clamp the answer is another clip (walk, jog, sprint), not more rate. Starts advance by distance, stops
  and pivots match distance to the predicted stop; orientation warping covers the angles between 4 or 8
  directional clips; Foot Placement runs last with plant by foot-speed curve; turn-in-place consumes a root yaw
  offset so the mesh does not spin with the capsule.
- **UE5 GASP / PoseSearch**: motion matching over hundreds of clips; the same Foot Placement and offset-root
  node (the mesh lags the capsule by a spring for translation and rotation: capsule snaps, body eases).
- **FromSoftware**: clip-driven with authored start/loop/stop/turn; the capsule's yaw snaps to the stick while
  the body twists and unwinds.
- **Offset root / "steering" [K]**: the common thread. A snappy capsule (720-1080°/s) never rotates the visible
  body at that rate: the mesh yaw follows with a spring (UE `Offset Root Bone`, FromSoft twist unwind), and the
  travel direction relative to the mesh is covered by orientation warping or lean.
- **Unity**: Animator blend trees with normalised-time sync, Animation Rigging for IK, Kinematica (motion
  matching) discontinued; shipped Unity ARPGs use blend trees plus Final IK grounder.
- **Decima (Horizon, Death Stranding), RAGE (RDR2, Euphoria)**: motion matching or large parametric clip sets
  with physics-driven reactions; not reachable with our clip supply, except Delirium-style reactions (note 12).

## 4. Bevy 0.20 and crates (checked against crates.io 2026-11-12) [S]

| Crate | Latest | Status for us |
|---|---|---|
| `bevy` | 0.20.1 (2026-10-28) | we are on 0.20; `bevy_animation` has `AnimationGraph` (blend/add nodes, masks), no state machine, no sync groups, no inertialization, no IK, no root motion, no distance matching (animation.md "Bevy integration", unchanged) |
| `bevy_animation_graph` | 0.11.0 (2026-07-11), Bevy 0.19 | still one Bevy behind, no release in 4 months; stays "reference only" |
| `bevy_mod_inverse_kinematics` | 0.12.0 (2026-09-01), Bevy 0.19 | not on 0.20; solves on entity transforms; not needed |
| `bevy_tnua` | 0.33.2 (2026-11-06) | maintained floating controller; our `sf_controller`/`sf_action` cover it; no change |
| `bevy_ahoy` | 0.2.0 (2026-08-06) | Source-style first-person kinematic controller (mantle, crane); reference for FP traversal feel only |
| motion matching | no library crate found (`bevy_motion_matching` is an app) | as animation.md: reference only |

Conclusion unchanged from animation.md's reuse check: the ecosystem offers no graph with sync, inertialization,
warping and foot placement on Bevy 0.20, so `sf_anim_graph` stays ours. Nothing found that should replace a
piece we built.

## 5. Corrections to our series

- `locomotion-rethink` §2 says Lyra/GASP as the model but does not state the **play-rate clamp**; our SpeedMatch
  range 0.5-2.2 (`mw_graphs/src/build.rs:925-936`) is far outside shipped practice. See audit finding 1.
- `11-tradeoffs` §5 chose "one body for both views" with a baked virtual camera per clip. Still right, but the
  note under-weights the aim-pitch requirement (§2 item 1 above); camera.md has no line for it. Added to the
  plan as F1.
- README R9 lists "purchased if needed" and "mocap" among clip sources; the owner's later rule is free packs,
  generated Kimodo clips and Morrowind clips only (2026-11-05). The README seed text is stale on that point.
- `ik-feel-diagnosis` §2 "Root motion vs capsule ... No conflict" holds, but "the same op stack as Lyra" hides
  that Lyra's stack sits on **speed-matched clips and an offset root**; ours sits on mismatched clips and a
  rigidly attached body. The ops are not the weak part; their inputs are.

## Open

- UE/Unity/Dark Messiah/Mordhau statements are from memory; verify the Lyra play-rate clamp value and the UE
  5.5 first-person primitive fields against Epic's docs before they go into a spec.
- spud `src/game/combat.cpp`, `app_kill_camera.cpp` and sinhonor `controller.rs`, `camera.rs`, `hands.rs` were
  not read line by line (README, module docs and symbol lists only).
- AC1 code since note 04 was read from commit messages only.
