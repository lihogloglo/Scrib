# R4: AC1-rs animation, blending and IK

Status: done (r4, 2026-10-18).

Source: https://github.com/Goku436/AC1-rs (MIT), read at `6d935e0` (2026-10-08): `crates/game/src/{animation,
move_blend,gait,character,climb}.rs`, the `ik` crate (`lib`, `chain`, `placement`, `mirror`, `cloth`, `ragdoll`),
`docs/PIKVR.md`, `docs/NOTES.md`, `forge::anim` (interface only). R3 (`03-ac1-climbing-parkour.md`) covers
climbing, jumps, end-pose validation and climbing IK policy; not repeated here.

**Clean room.** The ground blend and speed model are AC1-rs's rewrite of another MIT project's port of the game's
executable. Mechanisms and rounded ranges only; no clip names, addresses, slot tables or format details.

**`PIKVR.md` is not VR.** It is the "Parkour, IK, Visuals, Research" test campaign. AC1-rs has **no first person
at all** (orbit, free and scripted cameras only). First-person answers must come from R2 and R7.

## 1. How a frame's pose is made

One hand-written pipeline per character (`character::animate`); no blend tree or graph evaluator.

1. **Source pose** from one of: ground locomotion (`animation::Animator`: stand, idle breaks, start, gait blend);
   a **clip chain** (`climb::WallClimb`, used for climbing *and* every discrete ground move: stops, turns,
   landings, sitting), sampled, optionally **mirrored**, crossfaded from a frozen snapshot of the last shown pose;
   or a procedural fallback.
2. **Partial layers** with binary subtree masks: upper-body loop (crowd walk), upper-body one-shot (~0.15 s in,
   ~0.2 s out), legs-only one-shot. Lerp/slerp per bone; no additive, no per-bone weights.
3. **Exit fade**: when a chain ends or the stand changes, the last pre-IK pose is frozen and faded out over
   0.2–0.4 s (smoothstep).
4. **Ragdoll** replaces the pose when limp (§7).
5. **IK passes** in fixed order (§4), then locals written to joint entities.

**Transitions are snapshot crossfades, not inertialization.** Fade length comes per item from the move graph
(0–0.3 s) or, inside chains, is **sized by how far hands and feet move between the two poses** (largest limb
displacement), so near-identical seams are short. Fades start **from the pose before IK** (fading the post-IK pose
applied the IK twice: a fixed bug).

**Variants are baked mixes.** "These clips at these weights" (jump distance, start speed) yields a **new clip
resampled on normalised time**, cached by weights rounded to hundredths. Each blended move is again one clip with
one root track and one end pose, which keeps end-pose validation single-clip.

## 2. Ground locomotion

**Half-cycles per leading foot.** Each gait is a "left steps" and a "right steps" clip that join exactly (a tool
checks every bone at the seams). One phase per half; at wrap the side flips, so **foot phase is discrete state**:
which foot leads. Stands are per foot ahead; a stop ends on the other foot; a start begins on the standing foot.

**One speed scalar drives a 17-way blend on a shared clock** (`move_blend`). A 0..1 value in four bands (walk,
jog, run, sprint) weights two neighbouring gaits linearly, plus:
- **Lean/bank clips** weighted by the **angle between body heading and wanted (stick) heading**, full at ~90°,
  filtered at a few per second; AI without a wanted heading uses yaw rate. Heading error anticipates the turn,
  yaw rate lags it.
- **Two timer-weighted flavour clips**: a deceleration jog while slowing and a take-off sprint fading over about
  a second after entering the sprint. Acceleration is expressed by clip choice, not a procedural lean.
- **Clock**: a step lasts Σwᵢ·Tᵢ and covers Σwᵢ·dᵢ, so natural speed = **Σwᵢ·dᵢ / Σwᵢ·Tᵢ** (slower between bands
  than a mix of band speeds). Playback rate = body speed / natural speed, clamped ~0.6–1.6.

**The body owns speed; the blend follows.** The speed value rises over about a second and falls along a per-band
curve (fast out of walk and sprint, slow through jog and run); a start jumps straight to the first band. Velocity
approaches the target exponentially, and **running, steered elsewhere, swings round at a fixed turn rate keeping
speed** rather than braking. After a climb or landing the speed value is **resynced down to measured speed** (+ a
margin), never below the start band.

**Responsiveness** (code-picked, root-motion clip chains):
- **Start** from the stand on its foot, blended by start speed (slow walk / walk / jog), into the other foot's step.
- **Stop**: at walk speed, **instant** with a ~0.2 s fade; faster gaits play a stop chain that steering breaks off.
- **Turn on the spot** past ~45°: quarter or half turn by side, foot and profile. The stick **bends the turn**
  (a few rad/s on top of the clip's yaw); in high profile it **breaks into the run** once within ~30°.
- **Run turn-around**: a 180° clip whose root flips under the body at its start (§8 for the fix).
- **Pose-matched re-entry** (`Animator::resume_matching`): handing back to a gait, sample both half-cycles at 24
  points, pick side and phase with the smallest summed **leg-bone rotation difference**. Exits name the gait,
  not the stride point; starting at phase 0 repeated the foot just stepped on.

**Root**: discrete moves use root motion plus a spread correction (R3 §5); locomotion is capsule-driven. Flat clip
paths follow ground height separately. On stairs the root's height follows **ground averaged ~0.35 m behind and
ahead** along the motion, so stairs are a steady climb with foot IK on each step.

## 3. What a clip carries

Duration, per-bone tracks at a fixed rate, a **root track** (translation, yaw), and event/FOV channels that
AC1-rs **ignores**: no contacts, sync markers, warp windows or events. Foot at start/end, profile, gait and target
state are **parsed from clip names**; gates and blend times come from the move graph. Per-character **clip sets**
swap in replacements under shared names. Bones bind by name hash per rig.

## 4. IK end to end

Solvers (`ik` crate, generic): analytic **two-bone** with pole and optional end rotation; **look-at chain** where
each link takes its share of the *remaining* error up to its cap; FABRIK and CCD (unused); **`FootPlacement`**
(vertical probe above each animated ankle; tilt to the normal ≤ ~35° **faded out as the animated foot lifts**
0→~15 cm, so swing feet don't tilt without contact data; **pelvis only lowered**; offsets smoothed ~14/s; also
drops a spine that is the hips' sibling); **`place_hand`** (palm frame from surface normal and "up", wrist = contact
minus rotated palm offset, elbow hint).

Policy (`animate`), in order:

| Pass | Target | Gating / weight | Smoothing |
|---|---|---|---|
| Wall-lean palms | ray into wall, ~5 cm out | while leaning | none |
| Climb reach assist | hold beyond ~98% arm length | clavicle ≤ ~25°, upper spine ≤ ~10°, × shortfall/15 cm | with hand weight |
| Climb hands, feet | R3 §6 (latching, proximity) | hands on wall | hands ≤ ~6/s; feet ~12/s |
| Ground feet | `FootPlacement` | every grounded state, incl. stops and turns | global ramp |
| Arms out of walls | ray shoulder→elbow→hand; push hand out along the wall normal + ~6 cm | an arm segment pierces a wall | ~18/s |
| Look-at | nearest "interest point" < ~9 m, not behind | off while climbing | weight ~3/s, target ~6/s |

Climbing and ground-foot weights ramp **~0.25 s in, ~0.2 s out** through smoothstep. Look-at gives ~10–15% to each
spine bone, ~40% to the neck, the rest to the head (caps ~7–43°). A fix-up turns hood bones (children of the chest)
by head turn minus chest turn. **Absent:** foot locking, contact curves, prop/door hand IK, stride/orientation
warping, full-body IK, IK LOD.

## 5. Mirroring

`ik::mirror` reflects each pair bone's **skinning delta** (model × inverse bind) across the sagittal plane and
applies it to this bone's bind; proper rotations whenever the rest pose is symmetric. Pairs by `Left`/`Right`
names, else by **mirrored rest position (≤ 2 cm) preferring a mirrored parent** (unnamed helpers). One use: only
one half of a hand-over-hand bar step exists, so alternate steps (and the drop after a mirrored one) play mirrored.

## 6. Cloth

`ik::cloth`: the robe's **render-mesh vertices as world-space Verlet particles**, edge constraints plus weaker
bending links across shared edges, 4 Gauss-Seidel passes, **max distance to the skinned position** (0 at the waist,
growing to the hem), capsule push-out (hips, legs), and an optional **shape pull** toward the animated position.
Per-frame damping, dt capped at 1/30: frame-rate dependent.

Ours (`sf_anim::cloth`) already does all of this better (XPBD 1/240 s substeps, character space with inertia
scaling, tethers, swept capsules, teleport handling, a bone lattice). **One thing we lack: the shape pull**
(Chaos "animation drive"): a frame-rate-independent drive toward the animated pose keeps the authored silhouette
under motion.

## 7. Ragdoll

`ik::ragdoll`: **position-based particle ragdoll**, limp/death only. Particles at driven bones plus tips, the
game's radii and masses mapped by name; pelvis and chest braced by all-pairs links; hinge and cone **limits as a
distance range** between the outer particles (law of cosines, momentum-free), spine ranges **relative to the start
pose**; 10 passes; swept-ray collision; contacts damp tangential velocity and remove inward velocity; a 15 m/s
cap. Rotations: two non-collinear children give a full frame (pelvis, chest); others aim at their child **keeping
takeover twist**. **Settled** (low mean speed for 2 s): simulation stops. No powered mode, no get-up.

`sf_ragdoll` (rapier, PD drives, get-up) stays. Take **settle-and-freeze** and the **velocity cap**. A particle
ragdoll could serve as a cheap fallback beyond our 8-powered budget; measure rapier sleeping first.

## 8. Failure modes

- **Profile stand switch jumped the body 13 cm** → ~0.3 s fade.
- **IK on/off at once** (climb corners, top-outs; foot IK off for every ground move) → feet jumped 10–13 cm per
  seam (`6638335`, `2579907`) → global ramps; foot IK stays on through in-place moves.
- **Fades**: always ran against a fixed length; faded from post-IK pose (double IK).
- **Run turn-around** swung the body round in one frame, then crossed the legs mid-fade (`ffc948b`). Fixes: set
  the capsule facing **in the frame the move starts**; fade from the shown pose **re-expressed in the new facing**
  (turned slightly short of 180° so the way round is defined); blend the root bone **the way round nearest last
  frame's**, not the shortest arc, which flips as the gap passes 180°.
- **Gait restart repeated a foot** → pose-matched re-entry.
- **Receptions** played faster lurched the root to 16 m/s; played out stood still → cut where the clip's step
  speed falls to run speed (`899a243`).
- **Sprint weaving**: 5–7 cm limb pops from bank smoothing (open).
- **Per-frame margins** broke at high frame rates → per-second rates, test at several frame rates.
- **Tooling**: IK-off A/B on the same run; per-bone recorder; **pop detector** (a bone's root-relative speed above
  ~3× last frame's + 1.5 m/s, aligned to the recorder's lag so hitches aren't pops); gait-seam and clip-jump
  checkers; confirm the log names the clip before trusting a scripted run.

## 9. Situations

Sneak: slow-walk band and lean clips, low-profile edge halt, no crouch. Roof to roof, ledge, climb: R3, with baked
variant mixes and receptions cut to run pace. Sword: removed. Sit: bench chains by approach angle, no seat IK.
Door/lever: none. **First person: none.** Third person: all of the above.

## 10. For us

Our runtime is stronger: weighted masks, inertialization (`sf_anim_graph::inertial`), marker sync, Delaunay
blend spaces, foot locks, stride/orientation warping, `FullBodyIk`, model-space mirroring (equivalent to
`ik::mirror`), XPBD cloth, powered ragdoll. AC1-rs's value is **policy and failure knowledge**.

**Adopt**
1. **Gap-sized transition length**: inertialization duration from the largest hand/foot gap between the outgoing
   pose and the new source's first frame, clamped ~0.05–0.3 s (`duration: Auto` on transitions). Suits
   mixed-source clips with no authored blend times.
2. **Pose-matched cycle entry**: a `Player` start mode choosing phase and lead foot by leg-joint rotation match
   over N samples, beside "phase 0" and "sync marker". Works on unannotated clips (UE's pose-search plugin offers
   pose-matched start positions; full pose search stays "Later").
3. **Large-turn transitions**: our `rot_diff` takes the shortest arc. For pivots and 180° turns: set the mover's
   facing on the switch frame, re-express the previous output in the new character frame before computing offsets,
   keep **winding continuity** past ~150°. Test: a 180° pivot with no one-frame swing and no leg cross.
4. **IK policy as data** (R3's Layer 2 gap): per-op global ramps (~0.25 s in, ~0.2 s out, smoothstep), gating by
   state class, proximity weights, latching, rate limits; **foot placement stays on through in-place moves**.
5. **`ArmClearance` op**: arm segments raycast against the world, hand pushed out along the normal with clearance,
   smoothed, two-bone solve. Wall-hugging, corridors, crowds; the spec has nothing like it.
6. **Swing-foot tilt fade by animated foot height** as the fallback contact signal for clips without contact
   curves (R9).
7. **Heading-error-driven lean/bank** for the `Lean` op and bank axes (yaw rate for AI), and **gait speed resync to
   measured speed** after landings and climbs.
8. **Reach assist** (clavicle + upper spine toward an out-of-reach target, by shortfall) where `FullBodyIk` is off by
   LOD.
9. **Cloth animation drive** (§6); **ragdoll settle-and-freeze** (§7).
10. **Tooling**: IK A/B, pop detector, half-cycle/loop seam check at `anim.cook`, multi-frame-rate tests.

**Adapt**
- **Per-foot structure**: we use full cycles; derive **leading foot from foot markers** and expose it as a graph
  parameter so stops, turns, landings and jumps pick the per-foot variant or play mirrored.
- **Acceleration flavour clips**: an optional blend-space axis fed by smoothed acceleration, when a source has them.
- **Baked variant mixes**: evaluate blended end poses at validation time instead; bake only if cost shows.
- **Stair root**: locomotion.md smooths height with a lagging spring; averaging ground ahead and behind along the
  predicted path starts the rise before the step. Check on Balmora stairs.

**Reject**: snapshot crossfades, binary subtree masks, clip-name metadata, proximity-only look-at (targets come from
`sf_interaction`/AI), vertex-level Verlet cloth, particle ragdoll as primary.

**Port MIT code?** No. Every `ik` module has an equal or stronger counterpart in `sf_anim`/`sf_anim_graph`; the
pop detector (`examples/pops`) is worth re-reading when we write ours.

**Where our spec is wrong, thin or missing**
- **Evaluation / SpeedMatch** (verify): `instance.rs` takes a blended `Player`'s natural speed as the weight-averaged
  clip velocity. With lanes on one normalised clock the true speed is **Σw·d / Σw·T**; clips of different cycle
  lengths leave a residual for `StrideWarp`.
- **Canonical technique**: no rule for transition length, no large-angle inertialization (adopt 1, 3).
- **Procedural pass and IK**: no ramps or gating, no arm anti-penetration, no reach assist, tilt depends on contact
  curves; "Hand IK" has targets but no policy.
- **Layer 2**: add adopt 4–6 and 8 to R3's policy list. **Layer 4**: settle/freeze; physics.md "Cloth": drive.
- **Starts/stops/pivots**: distance matching is still a P2 gap; AC1-rs shows the responsiveness rules that matter
  without it (instant walk stop, stops broken by steering, turns bent by the stick and broken into the run, turn
  rate kept at speed, speed resync). These belong in locomotion.md and R11's action graph.
- **"Prior art"** says AC1-rs's `ik` has "nothing to adopt": true for solvers, wrong for policy and fixes.
