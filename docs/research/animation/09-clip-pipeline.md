# R9: the clip pipeline (making any clip work)

Slice R9 of the animation study (README). Research only, 2026-10-18. Inputs: notes 01-06, bevyrrowind
`docs/specs/animation-sources.md`, animation.md "Bake steps", "Optional clips", "Slots and montages",
rig-profile.md, `sf_anim::retarget` (interface), and the owner's folder `D:\Gamedev\Animation morrowind`
(listed read-only; `.blend` files were never opened in Blender, only their ID names read from the
decompressed bytes).

**Answer to R11a "what clips must carry", in one line:** nothing beyond bones. The bake infers
everything it can; every gameplay window has a *category default*; a human authors only the windows
that decide game feel, **once per moveset, not per clip**, in a sidecar file the bake merges. Details
in §6.

## 1. What is in the owner's folder

| Item | What it is | Reusable? |
|---|---|---|
| `Universal Animation Library[Standard]` (folder + zip) | Quaternius UAL1 free tier, CC0: **45 clips** (Godot GLB, Unity FBX, UE FBX) on a 53-joint Rigify `DEF-*` skeleton with `root` | Yes, CC0, already the spec's primary source |
| `Universal Animation Library[Source].zip` | UAL1 paid "Source" tier: **126 clips** + the `.blend` per engine | Yes (CC0); full list below |
| `arp-body.blend` | Morrowind `base_anim` body (`Bip01`, Mannequin, Weapon/Shield/Tri Shadow nodes) under an Auto-Rig Pro rig, one `Idle` action | Shows the method; our bake does this without Blender |
| `test retargeting.blend`, `arp-body_retargeted(-2).blend` | ARP "Remap": the 45 Standard clips, then **all 126 Source clips** (`Rig|...` actions), retargeted onto the Morrowind body | Proves UAL→`Bip01` works; output is a Blender-side bake |
| `backflip.blend`, `Test export/BackFlip.nif` | One clip (`[Raw]BackFlip`) baked and exported to NIF through the Bizarre add-on | End-to-end proof of concept, March 2025 |
| `BizarreMorrowindAnimatonUtilities-main` | Blender 4.3 add-on (no license file, so all rights reserved by default): bake + decimate + export to `.nif`/`.kf` for 3rd- and **1st-person** reference armatures, a **beast (Khajiit/Argonian) retarget** driver rig, constraint mute, an IK rig with Cascadeur-style auto-posing and "mixed kinematics" | Ideas only (beast leg retarget, 1st-person export by reference armature); don't copy code |
| `rig_tools_3.74.37.zip` | Auto-Rig Pro's free companion tools (GPL-2.0+) | Blender-side only |
| `blender-animation-retargeting-stable.zip` | Mwni's Blender bone-mapping retargeter (manual map, alignment, IK correction, baking) | Superseded by our `anim.retarget` |
| `rokoko-studio-live-blender-master.zip` | Rokoko Studio Live add-on (LGPL-3.0): auto bone detection lists + retargeting | Its bone-name auto-detect lists are a useful *reference* for auto-map coverage |

**What the owner was trying (March 2025):** the classic modder route. Retarget Quaternius UAL onto the
Morrowind skeleton in Blender with Auto-Rig Pro's Remap, then bake and export to NIF/KF with the
Bizarre add-on for both views and beast races. It worked for one clip (backflip). It's a per-clip,
manual, Blender-bound loop. Our bake replaces it (`anim.retarget` onto `base_anim`, `base_animkna`,
`base_anim.1st` from the BYO manifest), but two of its ideas are worth keeping: **beast legs via a
driver rig with IK** (our retarget's leg IK fix-up with `TwoBoneAnkle` does this) and **first-person
clips derived from third-person ones by exporting onto the 1st-person reference armature** (§3, §5).

**UAL1 Source, 126 clips** (Standard's 45 are a subset). Locomotion: Walk, Walk_Formal, Jog
Fwd/Bwd/Left/Right + 4 diagonals + Lean L/R, Sprint Enter/Loop/Exit, Crouch Enter/Exit/Idle + 8
directions, Crawl Enter/Exit/Idle + 4 directions, Turn90 L/R, Swim Fwd/Idle. Air: Jump
Start/Loop/Land, Roll (+`_RM`), BackFlip. Traversal: **Climb Enter/Exit/Idle/Up/Down/Left/Right
(+`_RM`), ClimbLedge (+`_RM`)**. Combat: Sword Enter/Exit/Idle/Attack/Attack_Standing (+`_RM`), Punch
Jab/Cross, Kick, PunchKick Enter/Exit, **Dodge Left/Right (+`_RM`)**, Hit Chest/Head/Stomach/Shoulder
L/R, Death01/02, Spell Simple/Double sets, Pistol set. Life: Sitting Enter/Exit/Idle×3/Talking/Nodding,
GroundSit set, Counter (shopkeeper) set, Interact, PickUp Table/Kneeling, Push set, Drink, Fixing,
Idle variants, Dance, Crying, Celebration, Driving. The `_RM` twins carry root motion; the others are
in place. **UAL2** (melee combos, parkour) is not in the folder.

## 2. Requirements matrix: what each clip datum is, and who supplies it

What `anim.cook` already infers (animation.md "Bake steps"): root motion (a moving root, else the
hips projected to the ground), foot contact curves (height + ground speed with hysteresis and
heel-off), sync markers at contact onsets, average velocity, and for **in-place** clips the ground
velocity from the planted feet's slide. Retargeting carries curves and markers across and pins legs
by the contact curve. Everything below is measured against that.

Legend: **I** = the bake infers it (reliability), **D** = a category default stands in, **A** = a human
authors it (per clip, or once per moveset).

| Datum (who needs it) | Supplier | How, and precedent |
|---|---|---|
| Root motion translation + yaw (all; R5 yaw separate) | I, high | Built. In-place sources (Mixamo "In Place", UAL non-`_RM`): ground velocity from foot slide, built; integrate it into a synthetic root track for one-shots. Unity's "Based Upon: Center of Mass" and UE's `MotionExtractor`/`ZeroOutRootBone` modifiers do the same jobs ([UE modifier library](https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/AnimationModifierLibrary?lang=en-US), [Unity root motion](https://docs.unity.cn/2023.2/Documentation/Manual/RootMotion.html)) |
| Foot contacts, sync markers, leading foot (R2, R4) | I, high on mocap, medium on keyframed | Built (contacts, markers). Leading foot at start/end = which foot is planted at frame 0 / last: trivial from contacts. UE's `FootstepAnimEventsModifier` writes sync markers from a ground threshold; thresholds miss some contacts and need checking ([UnderPressure, Mourot 2022](https://arxiv.org/pdf/2208.04598)). Our rate-limited hysteresis is the standard heuristic; a learned detector is not worth it yet |
| Hand contacts (climb, mantle, vault, ladder; R3 end pose, R4 IK latching) | I, medium; root-motion clips only | **Missing.** Feet use "low and still over ground"; hands need **"still in world"**: hand velocity in the root-motion-integrated frame under a threshold for ≥ ~0.1 s. Fails on in-place climb clips (no world frame): require the `_RM` twin or a synthetic root. Contact-aware retargeting is research ([Villegas 2021](https://arxiv.org/abs/2109.07431)) |
| End pose, hand spacing, step length per move (R3 validation) | I, high | Sample the last frame (and chains). AC1-rs takes this from the clip, so it's free for any source |
| Phase (cycles) | I, high | From markers. DeepPhase extracts phase without labels ([Starke 2022](https://github.com/sebastianstarke/AI4Animation)); not needed while markers work |
| Turn windows, turn amount (R4, R5) | I, high | Root yaw rate over a threshold → window; total yaw → turn class (90/180, side) |
| Loop seam quality, half-cycle seams (R4) | I | Seam check at cook, built in spirit (R4 adopt 10) |
| Warp windows (R3, R5; animation.md Layer 1) | I, medium + D | Contacts bound them: vault = take-off foot lift → hand plant; mantle = start → first hand plant; landing = flight → foot plant. Default: whole clip, R3's correction profiles. UE authors them by hand as notify states ([Motion Warping](https://dev.epicgames.com/documentation/unreal-engine/motion-warping-in-unreal-engine)); inference from contacts is our heuristic, no published precedent found |
| Launch / flight / gravity-off (R5) | I, medium | Both feet unplanted + root rising = flight; apex and landing frame from the root's vertical curve |
| IK fade hints (R4 policy) | I, medium | Weight = contact curve of that effector, ramped; plus "foot placement on" for in-place moves (R4) |
| Virtual first-person camera track (R2) | I, medium | Low-passed head position, a few degrees of pitch/roll, no yaw (R2 proposal); review on fast clips (rolls, flips) where it needs an authored override |
| Weapon / hand socket track over hit windows (R6) | I, high | Bake the hand socket in root space at cook; the weapon segment composes at runtime |
| Hit windows (R6) | I, low + **A** | Candidate = weapon-tip speed peak over a threshold, inside the clip's middle. Unreliable for wind-ups, multi-hit swings and thrusts: the bake proposes, a human confirms per moveset |
| Accept / execute / cancel windows per action class (R5, R6) | **D + A** | Not in the motion; they are design. Defaults per clip category (§5); the "restricted" rule makes loops need nothing. Souls authors every clip (TAE); our budget is one table per moveset |
| I-frames, armor tier, deflect, charge windows (R6) | **D + A** | Category default (dodge: i-frames over a fraction of the clip) + moveset override. Morrowind KF's own keys give attack defaults (R6 Adapt) |
| Recovery / reach / first contact for AI (R6) | I | Derived from hit and cancel windows and the socket track |
| Paired sockets (R6 §9) | A, or I from two-subject takes | Relative root transform of the two actors at the pair's first frame. Only CMU's two-subject takes provide pairs; else author an offset per pair |
| Gates, chooser inputs (speed band, height band, side) (R3) | I + A | Height/distance from end pose and root motion (bake), the rest in the move rule (data), not the clip |
| Footstep, hand-slap sounds (R2) | I | From contact onsets; built for feet |
| Other sound, VFX, camera presets, look-at | A | Per moveset, optional |

**What can't be inferred:** the design windows (accept, cancel, i-frames, armor, deflect, charge),
confirmed hit windows, and paired offsets without two-person data. Those decide feel, and they are
cheap to author as numbers in a table. Everything spatial (root, contacts, end pose, warp bounds,
sockets, camera) the bake can measure.

## 3. Retargeting onto our skeletons

**Industry.**
- **UE5 IK Retargeter**: named chains per rig with fuzzy-matched names, a shared retarget pose
  (auto-align), FK chains copying rotation deltas, the pelvis scaled for proportions, and IK goals
  solved afterwards. It's an **op stack** (Pelvis Motion, FK Chains, IK Chains, IK Solve, Root
  Motion, Stride Warp, Speed Planting), with per-op LOD and override sets bound to variables
  ([docs](https://dev.epicgames.com/documentation/en-us/unreal-engine/ik-rig-animation-retargeting-in-unreal-engine)).
  **Speed Planting** pins a foot goal while a per-foot speed curve, baked by a `MotionExtractor`
  modifier, is under a threshold
  ([foot sliding guide](https://dev.epicgames.com/documentation/unreal-engine/fix-foot-sliding-with-ik-retargeter-in-unreal-engine)).
- **Unity Humanoid**: a fixed humanoid bone set and a normalised muscle space. The root is a
  body/centre-of-mass transform, and per-clip "Bake into Pose" settings choose what the root keeps
  ([root motion](https://docs.unity.cn/2023.2/Documentation/Manual/RootMotion.html)). Its known cost
  is losing anything outside the humanoid set.
- **Blender** (what the owner used): Auto-Rig Pro Remap and Rokoko's retargeter, a bone map plus
  optional IK. They're manual and per clip, and the bake happens in the DCC.

**Ours** (`sf_anim::retarget`) is already UE-shaped. It has chains with a retarget pose, model-space
deltas (so Blender +Y bones vs Max +X bones don't matter), hips scaled by hip height, root motion
and speed scaled likewise, and leg IK pinned by the source contact curve (UE's speed planting in
another form). Auto-map and auto-align are built and tested on a BVH → `Bip01` pair (0.03°).
Gaps, by situation:

1. **Hands.** Climbing, ladders, mantles and two-handed grips need hand goals pinned while a hand
   contact curve (§2, missing) is on. Without it, a shorter Morrowind arm leaves the hands short of
   the ledge. Add the off-hand grip constraint (off-hand to a socket on the main-hand weapon).
2. **Measure spatial facts on the target, after retargeting.** End pose, hand spacing, step length
   and warp residuals all change with proportions. Our contacts are detected on the source and
   renamed. That's fine for timing, but R3's validation must read the *target* clip.
3. **Root from in-place one-shots.** Prefer an `_RM` twin (UAL ships them) or Mixamo without
   "In Place". Otherwise, **plant-anchored root reconstruction**: generalise the built foot-slide
   estimator to every planted effector (feet and hands), and bridge flights ballistically between
   the last take-off plant and the first landing plant. Flag the clip so the result gets reviewed.
4. **Frame rates** (24-120 fps) are a non-issue: `anim.cook` resamples to its own searched rate.
   30 fps mocap smears fast sword tips, so hit sweeps sub-step on the baked socket track (R6).
5. **Missing joints.** BVH mocap (100STYLE, LaFAN1, CMU) has no fingers. Apply a per-item grip
   pose layer. Twist bones and props are rig data, not clip data.
6. **Beasts and first person.** Digitigrade legs work through the `TwoBoneAnkle` IK fix-up (the
   Bizarre add-on's driver-rig idea, done at bake). For first person, retarget onto `base_anim.1st`,
   then **re-express the arms relative to the baked virtual camera** (R2) and add a per-item framing
   offset. Industry hand-authors first-person sets, because third-person wind-ups leave the view.
   So derived first-person arms are the fallback, and Morrowind's own 1st-person clips win where
   they exist (weapons, spells, hand-to-hand).
7. **Self-penetration** (arms through wide armour) is research
   ([Villegas 2021](https://arxiv.org/abs/2109.07431)). R4's `ArmClearance` op covers it at runtime.

## 4. Coverage: the move list against our sources

The owner has UAL1 Source (126 clips). The other sources are BYO, per animation-sources.md.
"Private" means NC/ND or Mixamo terms: usable in the user's own bake, not shareable.

| Family (notes) | What the sources give | Gap |
|---|---|---|
| Locomotion, 8-dir, crouch (R2, R4) | UAL1: jog 8-dir + leans, crouch 8-dir, crawl, walk fwd, sprint enter/exit; 100STYLE: fwd/back/side walk and run, idles, transitions × 100 styles; LaFAN1 walk/run/sprint; MW KF | Covered |
| Starts, stops, pivots, turn in place per foot (R4) | UAL1 Turn90 L/R only; 100STYLE transitions; LaFAN1 takes contain them but are unsegmented (private); Mixamo has some | Medium. Cut from LaFAN1/100STYLE, mirror for the other foot, R4's pose-matched entry |
| Jumps, landings, rolls (R1, R2) | UAL1 jump start/loop/land, roll (+RM), backflip; LaFAN1 jumps and falls; Mixamo landings and rolls | Small. Landing tiers come from additive dips (R2) |
| Vault, mantle, climb, hang, shimmy, leaps, top-outs (R3: 100+) | UAL1 climb grid (enter/exit/idle/4 dirs) + ClimbLedge; UAL2 "parkour" (list unseen); LaFAN1 obstacles (private); Mixamo braced/free hang, shimmy, hang-to-climb, ladder; CMU obstacle/playground takes | **Largest.** Perhaps 20-30 usable moves of R3's 100+, from different performers, with inconsistent hand spacing |
| Ladders | Mixamo ladder; UAL climb up/down loops | Enter/exit top and bottom |
| Sit, bench, door, lever (R2, R3) | UAL1 sitting enter/exit/idles, ground sit, interact, push, pick up; Mixamo sits | Door by side/hand, lever, sit by approach angle: use warp + turn + IK reach |
| Sword: attacks, combos, guard, deflect, dodges, hit tiers, deaths, get-ups (R6) | UAL1: 2 sword attacks, dodge L/R, 5 hits, 2 deaths; UAL2: 3-4 hit combos with recoveries; Mixamo sword-and-shield and great-sword packs; MW KF per-weapon chop/slash/thrust, block, hits, knockdown, deaths; LaFAN1 fights and get-ups (private); Bandai combat styles (private) | Medium. Missing: guard hits, deflects, posture breaks, back/forward dodges, directional stagger tiers |
| Paired takedowns, ripostes, backstabs (R6 §9) | None. CMU two-subject takes are not combat | **Total** |
| First-person arms (R2) | MW 1st-person clips (weapons, spells, H2H); everything else derived (§3.6) | FP traversal looks: derived quality only |

**Ways to fill the gaps, cheapest first.**
1. **Fewer clips through procedure.** Mirroring (built) halves per-foot and per-side sets. Warping
   (built) covers height and distance bands. Hang, shimmy and ladder steps can be IK on a few base
   poses driven by the hold index (R3 Adapt: "two stances plus full-body IK reach"). The victim of
   a takedown can be a short reaction plus a powered ragdoll instead of a paired clip. Doors and
   levers can be generic reach + IK on interaction points.
2. **Owner video mocap.** Single-camera tools are good enough for vaults, mantles, combos and,
   with two people, paired kills. Rokoko Vision's free tier has single-camera FBX; BVH and
   dual-camera are paid ([pricing](https://www.rokoko.com/pricing)). Move.ai, QuickMagic and
   DeepMotion are alternatives. Open models (GVHMR, WHAM) have **non-commercial** weights, and the
   SMPL body model is MPI-licensed
   ([GVHMR card](https://huggingface.co/ryanrudes/gvhmr)). That's fine for a private BYO bake.
   Expect skating and weak hand contacts; our contact cleanup and IK fix them.
3. **Text-to-motion for filler** (idles, gestures, sit and talk variants). **HY-Motion 1.0**
   (Tencent, Dec 2025) has 200+ categories under its own community license, free below 100M MAU
   per [CGWORLD](https://cgworld.jp/flashnews/01-202601-HY-Motion.html)
   ([paper](https://arxiv.org/abs/2512.23464v1)); the territory wording is unverified. **Kimodo**
   (NVIDIA, Mar 2026) has Apache-2.0 code and NVIDIA Open Model License weights (commercial use,
   outputs not claimed), plus keyframe constraints, on SOMA/SMPL-X/G1 skeletons
   ([overview](https://rits.shanghai.nyu.edu/ai/nvidia-releases-kimodo-controllable-text-to-motion-for-characters-and-humanoid-robots/)).
   Neither is good at hand-on-ledge precision or combat timing yet. Both need an SMPL/SOMA → BVH
   step.
4. **Keyframe editing** of variants in Blender or Cascadeur. Cascadeur's tiers changed; its free
   tier has been reported as non-commercial
   ([FAQ](https://cascadeur.com/blog/general/cascadeurs-new-licensing-structure-comprehensive-faq)).
5. **Commissioning** a one-day traversal + paired session, as the last resort.

## 5. Style consistency

**The problem.** The sources don't move alike:
- UAL is keyframed: snappy, game-timed, broad.
- Mocap (100STYLE, LaFAN1, CMU, video) is realistic: slower, with micro-motion. Each performer has
  their own stance width, arm carriage and cadence.
- Morrowind KF is stiff 2002 keyframing.

A UAL run into a LaFAN1 vault shows a seam even when the blend is clean. AAA avoids this by
capturing a whole moveset with one performer. Ubisoft's motion matching annotates long takes and
labels **only what can't be inferred**
([Clavet, GDC 2016](https://gdcvault.com/play/1023280/Motion-Matching-and-The-Road)), which is
the model for §6.

**What we can do, in order.**
1. **One primary source per moveset family**: the manifest's slot candidates are already ordered.
   Mix sources across families (combat vs traversal), not within a chain.
2. **Per-moveset timing**: a rate scale per slot, and contact-aligned time warping so foot timing
   matches the moveset tempo. Gap-sized blend times (R4) hide the rest.
3. **Pose normalisation**: an additive stance offset per moveset. It's the difference between the
   source's idle and the moveset's reference idle, weighted off during contact-critical windows.
4. **Learned style transfer**
   ([Aberman 2020](https://arxiv.org/abs/2005.05751); Mason 2022, the 100STYLE paper) is research.
   Watch it, don't build it.
5. **Per-moveset tuning lives in the sidecar** (§6), reviewed in a test level, never keyed by clip
   name in code (R2 Reject).

## 6. Recommendation

**The contract (answers R11a "what clips must carry").** A clip must carry **bones only**, from any
source. Three layers make it playable, each overriding the one before:

1. **Inferred facts** (`anim.cook` + a new **`clip.annotate`** bake step on the *retargeted*
   clip): root motion (reconstructed for in-place one-shots), foot **and hand** contacts, markers,
   leading foot, phase, turn windows, flight, end pose and effector spacing, contact-bounded warp
   window, weapon/hand socket track, virtual camera track, IK weight hints, reach and first-contact
   proposals. Industry precedent: UE's modifier library and Ubisoft's "label only what can't be
   inferred".
2. **Category defaults from the slot**, not the clip name. The manifest already says which slot a
   clip fills (`attack.light.1`, `traverse.vault.low`, `loco.walk_fwd`), and each slot category
   carries default windows:
   - loops: unrestricted (R5's rule);
   - dodges: i-frames over a set fraction, move-cancel late;
   - attacks: accept from the start, cancels at recovery start, hit window = the proposal;
   - traversal: uninterruptible to the last contact, warp = the inferred window.

   Morrowind KF's own keys (hit, follow start/stop) map onto attack defaults (R6 Adapt).
3. **Authored moveset sheet**: one RON sidecar per moveset in mod data, keyed by slot. It holds
   the design numbers: accept/cancel per action class, i-frames, armour tier, deflect, charge,
   confirmed hit windows (nudge the proposal or accept it), paired offsets, rate scale and stance
   offset (§5), sound/VFX/camera events. An authored event inside a source file (FBX markers, a
   future glTF extra) wins over inference, as UE's authored notifies do.

**Minimal authoring:**
- Locomotion, idles, sits, jumps, traversal: **zero** beyond picking the slot (and a frame range
  for long mocap takes).
- Combat: about **one line per attack slot** (cancel times, hit confirm) plus moveset-wide rows
  (dodge i-frames, guard/deflect).
- Paired: an offset and a branch frame per pair.

For a modder bringing a sword moveset of 12 clips, that's roughly 15 rows of numbers, not 12
annotated timelines.

**Lint at bake** (warnings, never silent):
- an unconfirmed hit window;
- warp residual over R3's refusal ratio on the test geometry;
- hand contacts missing on a traversal slot;
- a reconstructed root;
- seam/jolt checks (R2, R4);
- style outliers against the moveset's tempo (cadence off by more than a set percentage).

**Authoring tool: an in-engine clip review scene, not a Blender add-on.**
- The truth is the clip *after* retargeting, IK fix-up and warping on our skeleton. A Blender
  add-on sees the source rig and would re-implement our inference in Python.
- A review scene in the existing `scenes.ron` mechanism shows the retargeted clip on a test
  course: contacts, socket trail, warp window, camera track, windows on a scrubbable timeline.
  It writes the sidecar.
- First version: read-only overlays plus hand-edited RON (the UI crate choice is open; the
  workspace has no egui).
- Blender stays the place to *make* motion (Bizarre-style rigs, Cascadeur), not to annotate it.

**Gap plan:**
1. **Traversal**: procedural IK hang/shimmy/ladder on a few base poses (R3), UAL1 climb + Mixamo
   + LaFAN1 obstacles (private), then owner video mocap for vaults, mantles and top-outs in one
   style.
2. **Paired**: a ragdoll victim first, two-person video mocap later.
3. **Combat**: UAL2 combos + MW KF + Mixamo packs under one moveset sheet.
4. **First-person arms**: derived from third person, MW 1st-person clips where present.

**For the owner's folder:**
- Retire the Blender ARP → Bizarre → NIF loop. The bake does it for every clip at once.
- Register UAL1 Source in the BYO manifest (126 clips, `_RM` twins preferred for one-shots).
- Get **UAL2** (melee combos, parkour). Its clip list is only on Quaternius' anim viewer, so check
  that before buying.

**Spec changes R11b should make:**
- animation.md "Bake steps": add `clip.annotate` and hand contacts, measured on the target.
- Retarget: hand IK goals and the off-hand grip.
- animation-sources.md: slot categories and the moveset sheet, replacing `ClipRules::events` as
  the authoring point.
- Replace WISHLIST §9's authored startup/active/recovery with derived values (R6).
