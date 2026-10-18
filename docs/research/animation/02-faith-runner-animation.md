# R2: faith-runner animation and first-person body (`faith_anim`, `look`, `camera`)

Study note, slice R2 (README.md). Research only, 2026-10-18. Source:
[tnrjns/faith-runner](https://github.com/tnrjns/faith-runner) (MIT) at
[`2e8b92d`](https://github.com/tnrjns/faith-runner/tree/2e8b92d): `crates/faith_anim`
(`driver.rs`, `rig.rs`, `sound.rs`, `tests.rs`), `faith_move/src/look.rs` and `camera.rs`,
`me_assets/src/anim.rs` (interface only), `src/me_viewmodel.rs`, `src/viewmodel.rs`, README,
commits. Movement is R1 (`01-faith-runner-movement.md`).

**Clean room.** The driver reproduces a commercial game's first-person animation tree (clip
choice, blend times, rate rules, camera behaviour) from that game's scripts, tree data and native
code. This note gives mechanisms and rounded ranges only, names no clips, and names only the
repo's own modules.

## 1. Shape

Per frame, the host runs:
1. `Controller::step` (R1).
2. `LookLimiter::apply` (§6).
3. `CameraFx::update`, the procedural camera (§7).
4. `Rig::update`, which calls `Driver::update`: pose, camera bone, body and leg placement, final
   camera (§2-§5).
5. `sound::Director::update` (§8).

`faith_anim` is engine-free (glam plus the movement crate). **Animation reads the controller and
never writes it**, except the load-time calibration (`Rig::tune`, R1 §4). The sim owns the feet;
animation owns what you see and hear.

## 2. State to clip

The driver keeps a **stack of crossfading layers** and at most one **one-shot**. Three sources:

**(a) Events start one-shots** (jump, wall jump, dodge, ledge grab, hard landing, springboard,
step-up, attacks...), each with its own blend-in, blend-out and sometimes rate. The variant is
picked from motion state at the event:
- jump clip by take-off speed along the facing (three bands);
- ledge grab by vertical impact speed (three bands; the hardest adds a camera jolt);
- side from the wall or dodge direction;
- springboard leg from the **locomotion phase** (which foot is forward);
- wall-run start slowed (about 0.6×) when there is wall overhead;
- wall-climb loop rate from the time to the top (0.1-1).

**(b) The state's own animation**, when no one-shot plays:
- loops (hang, wall run, fall);
- the locomotion blend (§3);
- clips **driven by gameplay progress** (§4);
- two-pose mixes by a scalar (beam lean, swing angle).

**(c) Overlays** applied after the blend (§5): landing offset, wall-braced arm, hang reach, lag
springs, camera-only clips.

**Blending.**
- Blend-ins are mostly 0.1-0.2 s, 0.05 s for violent impacts, and 0.3-0.6 s for soft arrivals.
- A new layer fades in linearly; older layers are capped at 1 minus its weight.
- What follows a one-shot blends in over the larger of its own blend-in and the one-shot's
  blend-out.
- A one-shot **cut short** blends out slower (about 0.4 s instead of 0.2 s): it isn't back at
  rest yet.

**One-shot lifetime** caused most bugs (§9). A one-shot ends at the first of:
- its length, or a gameplay hold (the hard-landing stun ends as the clip starts blending out);
- **move ownership**: a move's clip survives only while that move lasts (the grab must not run
  into the pull-up);
- **touchdown**, for clips owned by a jump (a dodge clip is about twice the airtime);
- **running off** (idles, standing landings), with a 0.3 s blend-out;
- a **follow-up** it names (wind-up then strike; kick-off held for its input window, then the
  jump).

The vault's clip outlives its move: the move hands back to walking early, so the clip is picked
up mid-way as a one-shot and its tail plays over the run.

## 3. Locomotion: gait, phase, rate

- **Six gait states by raw speed** (stand, sneak, walk, jog, run, sprint), with **no
  hysteresis**.
  - Crossfades get longer as speed rises: about 0.2 s for slow states, 0.6-0.8 s into run and
    sprint.
  - Each state has forward, "stiff" diagonal (arms forward, legs turned) and backward cycles,
    chosen by velocity along the facing (backward below about −0.2, diagonal below about 0.85).
- **One shared phase** (a sync group).
  - The heaviest cycle leads, at `clamp(speed / authored speed, min, max) / length`.
  - Rate ranges are wide: about 0.2-1.4 (sprint 0.6-1.4). There is no stride warping; the feet
    are rarely seen.
  - Followers sample the shared phase plus a **per-clip phase offset**: backward cycles sit
    half a step later.
  - No sync markers. Older cycles keep their proportions under a newcomer.
- **Turn in place.** The legs hold until the view is about 25° off them. Between about 25° and
  65° they wait about 1 s, then step 45°. Past that they step 90° at once, evenly over the clip,
  with the clip's root yaw discarded.
- **Idles** after 30-40 s untouched; moving cancels them, looking around doesn't.
- Crouch and wall-run cycles are speed-scaled within about 0.6-1.4 and 0.7-1.3.

## 4. Fitting a clip to the planned move

1. **Driven playhead.** For traversals (vault, mantle, pull-up), slides, rolls, ladder steps,
   shimmy steps, stumbles and get-ups, the layer's time is **set each frame from gameplay
   progress**, not advanced by dt.
   - Traversals use normalised progress × length; slides and rolls use the state clock.
   - Descending a ladder plays the step clip **backwards**.
   - The clip cannot drift from the motion at any frame rate.
2. **Calibration** (R1): durations from clip lengths, turns from root-yaw curves, ladder paths
   from root position. The sim is timed to the clips, so (1) is exact.
3. **Travel re-anchoring** (`is_travel`, `anchor_travel`).
   - At load each clip is sampled at 10 points. If the camera bone strays more than about 40 cm
     from rest, the clip carries the body through space the sim already moves it through (a
     name list overrides a few).
   - Each frame, such a layer is shifted by its root **so its camera bone sits on the gameplay
     eye, before blending**. Other layers stay on the feet.
   - Placing the *blended* pose part by eye and part by feet made the view lurch as a vault
     faded (`9470d75`). A test now holds the camera within about 25 cm of the eye for 40 frames
     after a vault.
   - Turn clips' root yaw is **removed from body placement**, so a turn animation doesn't double
     the gameplay turn.

## 5. First-person body: the camera is a bone

**True first person, one skeleton.**
- An arms mesh and a legs mesh (which also carries the torso) share one pose.
- The camera is the skeleton's authored **camera joint** plus the player's look:
  - position: origin + body rotation × camera bone + procedural offsets + swan neck;
  - rotation: look (yaw, clamped pitch, procedural roll) × lag-spring bank × swing tilt ×
    (camera-bone rotation minus the clip's yaw) × camera-channel clip.
- So **head bob, wall-run tilt, landing dips, the roll's full tumble and the hard-landing slump
  come from clips**. With clips loaded, the procedural layer drops to light shake and a landing
  kick (`CameraFxSettings::animation_driven`) so nothing doubles.
- The hidden asset is that **every clip was authored with a camera joint**.

**Facing** (`rig.rs`).
- The upper body follows view yaw exactly, except in **locked moves** (slide, wall run, ladder,
  hang, vault, zipline, beam, swing, lying down). There it eases to the move's facing (about
  12/s), so only the head turns.
- On unlock it returns at about 9 rad/s, then sticks with no lag.
- Jumping off a wall snaps the body to the view, so the jump clip isn't seen side-on.
- **Legs have their own yaw**:
  - moving: they face the velocity up to 90° off the view, and the backward cycle plays past
    about 95°;
  - standing: they use the turn-in-place steps;
  - never more than about 100° from the view.

**Overlays.**
- **Landing**: normal landings play no clip. An additive offset **dips spine and eye**, scaled
  by fall height (minimum about 20%, full at hard-landing height), in 0.1 s and out 0.4 s.
  Landing at a run also mixes part of a landing clip into the run cycle briefly. Only hard
  landings play a full clip.
- **Lazy springs**: lagging copies of view yaw and pitch (0.15-0.25 s catch-up) turn the torso
  against fast looks (up to about 5.5°) and **bank the camera into turns**, scaled by speed.
- **Camera channel**: clips applied to the camera only (wall-run impact jolt, crouch-to-stand,
  hard-grab jolt), with their own blends.
- **Wall-braced arm**: near a side wall, a braced pose blends onto that arm (in about 0.35 s,
  out 0.55 s). Two-bone IK puts the hand on the trace hit, clamped to a box beside the shoulder.
- **Hang reach**: looking more than 90° off the wall while hanging plays reach clips and rotates
  the free arm's chest joint by the look. One hand reaches where you look; the other stays on
  the lip.
- **Forearm twist morphs**, weighted by roll-bone twist, stop the forearm pinching.

**Seeing the body.**
- **Arms** are drawn on their own render layer by a second camera with **depth cleared**, so
  they never clip into walls. That camera's FOV is about 10° wider than the world's, which keeps
  edge-of-screen hands visible.
- Looking down from about 20° to 50°, the arms FOV **blends to the world's** (smoothstep), so
  arms and legs meet at the waist.
- **Legs** render in the world, depth-tested and shadowed, with the near plane at 1 cm.
- Gripping a bar or pipe draws the arms depth-tested in the world, so the bar covers the
  fingers.
- **Swan neck** (`look.rs`): looking down past about 15° moves the eye forward and down over the
  chest, up to about 35 cm and 30 cm on a quadratic curve, eased at `dt / 0.07`. You look at
  your legs from above your chest, not from inside your torso. It is set per move: from 0° and
  further forward while hanging (to see the drop), and off during a roll.

**Third person: none.**

## 6. Look limits and comfort

**Per-move limits** (`look.rs`): a pitch range and an optional yaw half-range around the **yaw
at move entry** (around the turning body for a slide).

| Move | Limit |
|---|---|
| Hang | Look down only about 18° |
| Slide | About ±55° yaw |
| Vault | Pitch about −16° to +33°, yaw about ±44° |
| Wall run | About ±70° pitch, ±90° yaw |
| Roll, lying down | Almost no look-down, about ±27° yaw |
| Air, ground, wall climb | Free (about ±85°) |

**Easing.** On entering a move, the view eases into the new limit (about 14/s) for 0.3 s, then
clamps hard: a snap would jerk the camera. Some moves ignore look input entirely (stun, grab
transfer, ladder entry).

**Comfort.**
- The roll somersault can be switched off.
- FOV changes approach slowly (about 4/s).
- Shake is quadratic in trauma, so small events stay subtle.
- Hard landings level the view.

There is no head stabilisation: the authored camera joint is the stabilised camera.

## 7. Procedural camera and arms (`CameraFx`, no-clip fallback)

- **Bob**: one footstep phase at `base + k × speed` steps/s. Vertical at twice the phase, sway
  and roll at the phase. At sprint: about 4-5 cm vertical, 3 cm sway, 1° roll, scaled by gait
  weight × √(speed fraction).
- **Lean**: 1.5-2° into strafes and turns, about 12° off the wall when wall-running.
- **Springs**: dip and pitch kick, substepped at 240 Hz so hitches can't blow them up, kicked by
  landings (in proportion to impact), grabs, vaults and dodges.
- **Shake**: trauma 0-1 added per event (jump a few %, landing up to about 1/3, hard landing
  about 0.8), decaying about 1.5/s and squared, with a sprint and slide floor, as sum-of-sines
  noise.
- **Somersault**: 360° pitch over 0.5 s on a skill roll.
- **Look-up assist** while wall-climbing or hanging.
- **FOV**: about +10° at sprint, +4° wall-running, +3° vaulting; a zoom-in at a vertigo edge;
  radial speed blur.
- **Procedural arms**: forearm and hand sticks posed in camera space per state (pumping on the
  gait phase, palm on the wall, world-anchored on a ledge). They sway behind fast looks, with
  teleports and 180° turns excluded.

## 8. Sound from animation

- **Notifies on clips**: a time plus a foot/hand step (a sound type and a side), a character
  sound tag (cloth, breath, effort) or a sound path.
- **Firing**: a notify fires when the playhead crosses it, wrap-aware. Driven layers fire too,
  so hand slaps land on the frame.
- **Weight rules**:
  - A **fresh** one-shot fires from its first frame even while fading in (an attack swoosh sits
    at about 1 ms).
  - Locomotion and fading layers fire only at weight ≥ 0.5.
  - Locomotion fires only its **dominant cycle**. When that cycle changes, its previous time
    resets to now, so the switch doesn't fire every step in between (`e4d67bb`).
  - Tests: walking over 5-30 cm clutter, or on a flickering gait line, steps no more than
    walking on the flat.
- **The Director** (`sound.rs`):
  - surface lookup separate for feet and hands, with a fallback;
  - random variant, volume and pitch;
  - **looping sounds a move started stop when the move kind changes**;
  - landings, rolls, slides, impacts and doors come from gameplay events (landings debounced
    0.15 s);
  - breathing only from notifies;
  - wind volume follows speed and fall speed.
- Without clips, footsteps fire on the bob phase.

## 9. Clip contract, missing clips, failures

**Clip shape** (`me_assets::anim`, in our words): name, length, rate, per-bone key tracks, and
sorted notifies. **No sync markers, curves, root-motion metadata or contacts.**

Everything else is in the driver, keyed by clip name:
- authored speed, rate clamps, sync offsets, blend times;
- travel status and the camera joint;
- move ownership.

**Missing clips** degrade quietly:
- Lengths fall back to guesses (0.33-1 s).
- A missing layer samples the **rest pose**. Hence a test that every clip the moves name exists:
  two misnamed clips had never played (`e4d67bb`).
- A missing camera clip is skipped.
- With no install, the app uses procedural arms and the full `CameraFx`, and `tune` doesn't run.

| Failure | Fix |
|---|---|
| Stops jolted (a frame moving the hands far more than its neighbours) | Gait from raw speed, per-state blends, master-led sync (`e4d67bb`). Test: no frame's hand travel exceeds 2.5× the median of its 6 neighbours |
| Vault camera lurched as the clip faded | Re-anchor travel layers before blending (`9470d75`) |
| Grab clip ran into the pull-up, hands above the lip | One-shots owned by their move |
| Dodge pose hung over the run | Jump-owned clips end at touchdown |
| Turn clips turned the wrong way; turns doubled | Root rotation handedness (README); clip yaw removed from placement |
| Step bursts on cycle switch, double steps in blends | Dominant cycle only, time reset on switch |
| Forearm pinching | Twist morphs |

## 10. Situations

| Situation | What it does |
|---|---|
| Roof-to-roof jump | Fully covered in first person: take-off clip by speed, landing dip by fall height, roll tumble, wall-run tilt and jolt, travel clips on the eye |
| Ledge grab, mantle, climb | Grab by impact speed, hang with look reach, progress-driven shimmy, pull-up and ladder (both ways). No ledge hand IK: hands land right only because the sim puts the eye where the clip expects |
| Sneak | Sneak cycle at rates from 0.2×, crouch look limits; no lean or peek |
| Sword fight | No. Unarmed one-shots by context, hit/miss follow-ups, combo blends; no weapon layer or upper/lower split |
| Sit, door, lever | Door barge and kick only |
| First person | The reference, in every situation above |
| Third person | None |

## 11. For us

**Our baseline.**
- `sf_anim_graph` has synced blend spaces with markers, inertialization, slots and montages
  (blends, `end_early`, interrupt), `SpeedMatch` with `StrideWarp`, and highest-weight event
  firing. These cover faith-runner's blending, sync and events, and do better.
- character.md has a few lines of first-person plan: view-model arms plus a shadow body,
  and "true first person" with a stabilised head socket as an option. There is an
  `mw_first_person` graph template.
- **There is no player camera** (bevyrrowind has a free camera), and animation.md and
  locomotion.md never mention one.

**Adopt**
1. **Driven-playhead clips.** A `Clip` node whose time comes from a parameter (normalised or
   seconds, either direction): UE's sequence evaluator. Ours has rate and start only.
   Traversals set it from planned progress. Any user clip then fits a planned move in time;
   warping fits it in space.
2. **Clip calibration as a bake product** (R1 adopt 6), with defaults for missing clips.
3. **Anchor each source, then blend**: eye in first person, root or warp target in third.
   Never re-place an already blended pose. Add **jolt tests** (hand and camera frame-to-frame
   travel against the neighbour median) and an off-eye test after every traversal.
4. **Montage end rules as data**: owned by action X, end on touchdown, yield to locomotion above
   speed S, then play Y, slower blend-out when cut short. Without them they become per-move
   code.
5. **Event details**:
   - montages fire from frame 0 (check that this holds under our inertialization);
   - reset the previous time when the sync leader changes;
   - move-scoped looping sounds;
   - hand and foot surface queries;
   - the clutter and flicker footstep tests.

   Our gait hysteresis (animfix) already beats theirs.
6. **One-shot variants picked by chooser inputs** (speed band, impact band, side, leading foot
   from the sync phase), not one state per variant.
7. **Event-driven additive envelopes**: a landing dip weighted by fall height, a camera-only
   clip channel, and lag springs on the spine and the camera bank. These need an `Additive`
   whose weight follows an attack/hold/release envelope.

**Adapt: a first-person and camera spec** (missing; R11). It must decide:
- **Body strategy.** True first person (arms layer plus world legs) is what makes faith-runner
  feel physical: hands on the lip, legs underfoot. But it rests on clips with an **authored
  camera joint**. Ours (Morrowind KF, UAL, Mixamo, mocap) are third-person clips. Their head
  motion is too big, and a raw head-socket camera nauseates.
  - Proposal: **bake a virtual camera track per clip** in `anim.cook`. Low-pass the head
    position; keep only a few degrees of the head's pitch and roll; drop its yaw. Per-clip
    overrides and authored tracks win.
  - Compose the camera as in §5.
  - Keep view-model arms where Morrowind's first-person clips exist (weapons, spells), and pick
    the body per action.
- **Rendering**: an arms layer with depth cleared at about +10° FOV, blended to the world FOV
  when looking down; legs in the world with shadows; a near plane of about 1 cm; a per-action
  depth-tested switch.
- **Facing**: per-action body-yaw lock with eased release; leg yaw from velocity (±90°, then the
  backward cycle); turn-in-place zones; per-action swan neck.
- **Look limits** per action, around the entry yaw, eased in, with ignore-look windows.
- **A procedural layer for fallback and comfort**: bob from the sync phase, squared-trauma
  shake, substepped springs, FOV adds, and comfort switches.
- **Third person**: R4 and R7 must supply it. Both views come from one rig and switch in a
  frame.

**Reject**
- Per-clip tuning keyed by clip name in code. It belongs in clip meta and the graph asset.
- Linear crossfade stacks: we keep inertialization.
- The rest pose for missing clips: we keep Optional-clip fallbacks and errors at bake.
- Rates down to 0.2× in third person: stride warping and `SpeedMatch` clamps are right there.
  The wide range only suits first person.

**Spec gaps**
- *Missing*:
  - a camera spec (first and third person);
  - first-person body rules;
  - look limits;
  - driven playheads;
  - montage end conditions;
  - additive envelopes;
  - a camera track in `Clip`.
- *Thin*:
  - character.md's first person assumes a stabilised head socket is enough;
  - animation.md's root-motion modes (Drive, Match, Ignore) lack "the sim drives the clip"
    (progress-driven), how any parkour keeps clips and planned paths in step.
- *Imprecise*: animation.md's prior-art entry calls faith-runner IK-free. It has wall-arm IK.
  Its real lesson is the camera and body coupling, not just the move set.
