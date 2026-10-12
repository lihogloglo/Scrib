# Rig profile: limb chains for any skeleton (creatures first-class)

Status: core built 2026-10-12 (see "As implemented"); mover parts not built
Crates: `sf_anim` (new module `rig`: `RigProfile`, auto-detection, the
`rig.profile` bake step), `sf_anim_graph` (ops read limbs from the
profile), `sf_controller` (mover modes and shapes per profile),
`sf_character` (the humanoid map produces a profile)

## Goal

Every procedural feature (foot placement, stride warping, look-at, limb
IK, full-body IK, motion warping, ragdoll generation, contact detection)
works on any skeleton: bipeds, quadrupeds, hexapods and octopods,
flyers, floaters, swimmers and serpents. A skeleton describes itself
once, as a **rig profile**: girdles, a spine, N limbs, a neck and head,
tails and extra chains. The humanoid rig (character.md, VRM 1.0 bones) is
an optional layer on top that adds retargeting to the humanoid hub. It is
not a prerequisite for any procedural op.

Non-goals: retargeting between unrelated morphologies at runtime (Spore's
problem; we retarget at bake, by chain name, between rigs that share a
template); learned quadruped controllers (MANN, below) before data exists.

## Canonical technique

- **Morphology described by limb chains, not bone names.** Spore's
  animation system (Hecker et al., *Real-time Motion Retargeting to Highly
  Varied User-Created Morphologies*, SIGGRAPH 2008) describes creatures by
  semantic parts (spine, limbs, feet, hands, heads) and generalizes motion
  over them. UE5's IK Rig and IK Retargeter use the same idea in a
  production tool: named **retarget chains** and **goals** per skeleton,
  with any number of legs (UE's quadruped samples), and a
  `Leg`/`Arm`/`Spine` chain type that selects the solver.
- **Quadruped locomotion:** authored or mocap cycles per gait, matched by
  speed, with procedural spine bending and per-girdle body adjustment
  (Frey, *Animating Quadruped Characters in The Flame in The Flood*, GDC
  2016: a rigid-spine boar against a procedural-spine wolf; Oud, *Bringing
  Life to the Machines of Horizon Zero Dawn*, GDC 2018 Animation
  Bootcamp). Foot placement per leg, with the front and hind girdles
  adjusted separately so the body pitches and rolls on slopes and stairs
  (the standard "pelvis plus chest" extension of Johansen 2009's
  semi-procedural locomotion, which already handles N legs).
- **Data-driven quadruped motion** (Zhang, Starke, Komura, Saito,
  *Mode-Adaptive Neural Networks for Quadruped Motion Control*, SIGGRAPH
  2018; Starke et al., *DeepPhase*, SIGGRAPH 2022, which works on dogs) is
  the research frontier. It needs quadruped mocap we don't have; the
  profile keeps the door open (limb phases, contact curves per limb).

## Data model

```rust
/// Per skeleton (bake product of `rig.profile`, next to `skeleton.cook`).
/// Character space: Y up, facing +Z (sf_anim's convention).
pub struct RigProfile {
    pub template: RigTemplate,          // Biped | Quadruped | Multiped | Flyer | Floater | Swimmer | Serpent | Rolling | Custom(name)
    pub root_motion: u16,               // joint
    pub girdles: Vec<Girdle>,           // [pelvis] for bipeds; [pelvis, chest] for quadrupeds; more for centipedes
    pub spine: Option<Chain>,           // pelvis girdle → neck base, root first
    pub neck_head: Option<NeckHead>,    // neck chain + head joint + aim axis + eyes
    pub limbs: Vec<Limb>,
    pub tails: Vec<Chain>,              // tail, antennae, tentacles not used for support
    pub extra: Vec<Chain>,              // anything else named (jaw, ears, skirt joints)
    pub mirror: Vec<(u16, u16)>,        // left/right pairs (Mirror node)
    pub measures: Measures,             // derived at bake: hip height, body length, leg lengths
}
pub struct Girdle { pub joint: u16, pub limbs: Vec<u8> }   // pelvis or shoulder girdle and the limbs it carries
pub struct Chain { pub name: String, pub joints: Vec<u16> } // root first
pub struct Limb {
    pub name: String,                   // "leg_fl", "leftLeg", "wing_l": pairs across rigs by name
    pub kind: LimbKind,                 // Leg(LegKind) | Arm | Wing | Fin | Tentacle
    pub side: Side,                     // Left | Right | Center
    pub order: u8,                      // front to back (0 = front pair)
    pub girdle: u8,
    pub joints: Vec<u16>,               // girdle child → effector, 2–6 joints
    pub effector: u16,                  // foot, hoof, claw tip, hand, wing tip
    pub solver: LimbSolver,             // TwoBone | TwoBoneAnkle | Fabrik | Ccd | None
    pub pole: PoleHint,                 // Animated (default) | Axis(Vec3) in girdle space, for straight rest poses
    pub foot: Option<FootSpec>,         // toe joint, sole offset, max tilt, contact curve name
    pub support: bool,                  // carries weight: foot placement, contacts, stride warping
}
pub enum LegKind { Plantigrade, Digitigrade, Unguligrade } // humans/bears; dogs, guars, beast races; hooves
pub struct NeckHead { pub neck: Chain, pub head: u16, pub forward: Vec3, pub eyes: Vec<u16> }
```

- **Solvers.** `TwoBone` is today's analytic solver (hip, knee, ankle).
  `TwoBoneAnkle` covers digitigrade and unguligrade legs (thigh, shin,
  metatarsal, toe: four joints): the metatarsal keeps its animated angle
  relative to the shin scaled by how far the leg is compressed, then the
  first three joints solve analytically (the common game approach for dog
  and horse legs; UE's Leg IK node handles any bone count by FABRIK
  instead, which we keep as the fallback). `Fabrik`/`Ccd` serve arms of
  insects, tentacles and long necks. `None` marks decorative limbs.
- **Templates** only choose defaults (which ops a template graph turns on,
  mover mode, contact thresholds). Nothing branches on the template inside
  an op: ops read limbs, girdles and chains.
- **Measures** scale every threshold that is in meters today: contact
  height and speed (fraction of leg length), stride warp limits, step-up,
  foot probe ranges, ragdoll capsule radii. A 0.2 m scrib and a 6 m
  centurion use the same rules.

## Auto-detection (`rig.profile`, at bake)

Input: the cooked skeleton, its tags, and optional adapter hints
(`RigHints`: template, joint → limb overrides, extra chains). Steps:

1. **Effectors**: leaf-most joints (ignoring `end`/`twist` helpers) of
   branches whose rest-pose lowest point lies within 5% of the skeleton's
   height above its lowest point are support feet. Tags (`foot`, `toe`,
   `hand`) refine the choice when present.
2. **Limbs**: walk each effector up to the first joint with two or more
   limb-bearing children: that joint is its girdle. Limb joints are the
   path, minus helper joints. Joint count and rest bend pick the solver
   and leg kind (three segments: plantigrade `TwoBone`; four with the
   ankle above the ground and the toe on it: digitigrade `TwoBoneAnkle`).
3. **Spine**: the path between the two outermost girdles; the head is the
   leaf of the branch past the front girdle that points forward. Tails are
   branches behind the hind girdle; wings are non-support limbs whose rest
   chain is longer than the body width and roughly horizontal.
4. **Side and order** from the rest position relative to the spine (+X is
   left, as in the humanoid canonical pose), then front to back.
5. **Report**: the profile is written as RON next to the product, as
   character.md does for rig maps, for review and for an adapter or mod
   override (`rigs/<skeleton>.rigprofile.ron` in the VFS).

A skeleton the rules can't resolve (a wheel, a floating orb) gets
`Custom` with no limbs: it still animates, with root motion and look-at
only.

## How the systems read it

| System | Today (biped assumption) | With the profile |
|---|---|---|
| `FootPlacement` | one `pelvis`, legs of exactly 3 joints | one offset per girdle, body pitch and roll fitted from girdle offsets and spread over the spine; any `support` limb, solver by kind |
| `StrideWarp` | scales about the single pelvis | per girdle center; limbs of any solver |
| `OrientationWarp` | pelvis/spine yaw split | biped template only; quadrupeds get `SpineBend` (yaw spread over the spine by turn rate, head leading) |
| `LookAt` | already generic (chain + axis) | default chain from `neck_head` |
| `HandIk` | 3-joint chain to a position | becomes `LimbIk`: any limb or chain, position and optional rotation target, solver by limb |
| Full-body IK | — | effectors on limb effectors and the head, girdles as roots (animation.md "Next-gen roadmap") |
| Contact cook | joints tagged `foot`, 5 cm absolute | `support` limb effectors (and toes), thresholds × leg length |
| Retarget IK fix-up | chains of exactly 3 joints | any `Leg` chain, solver by kind; per-limb scale for IK goals |
| Ragdoll (M4) | — | one body per limb segment, spine and neck joint; limits by limb kind (hinge knee/elbow, cone-twist hip/shoulder) |
| Mover | upright capsule, ground only | shape and mode by template (below) |

**Mover by template** (locomotion.md): bipeds keep the upright capsule.
Quadrupeds and multipeds use a horizontal capsule along the body (length
from `measures`) in M4's physics controller; the M3 kinematic mover
approximates them with an upright capsule of half the body length, which
is acceptable for separation and walls in towns. Flyers and floaters move
in 3D (`Fly` mode: no ground snap, optional hover height above ground,
banking from lateral acceleration). Swimmers use `Swim` (3D inside water
volumes, M6 queries). Rolling creatures use root motion only.

## Interfaces

```rust
// sf_anim::rig
pub fn detect_profile(skeleton: &Skeleton, hints: &RigHints) -> Result<RigProfile, RigError>;
pub trait AnimRules { /* existing */ fn rig_hints(&self, _skeleton: &str) -> RigHints { RigHints::default() } }
// sf_anim_graph: GraphRules (animgraph.compile) gains
fn profile(&self, skeleton: &str) -> Option<RigProfile>;
// and graph assets may write `legs: Profile` / `limbs: Profile(support)` instead of listing joints.
```

A graph authored against limb **names** (`leg_fl`, `leftLeg`) compiles on
any skeleton whose profile has those limbs, so one quadruped graph template
serves guar-like, dog-like and cat-like creatures.

## Proposed changes to character.md (character session owns that file)

1. `RigMap` additionally produces an `sf_anim::RigProfile` (template
   `Biped`, limbs `leftLeg`/`rightLeg` (support), `leftArm`/`rightArm`,
   spine and neck from the humanoid bones). `extra_chains` become the
   profile's `tails`/`extra`.
2. `CharacterRecipe::rig` becomes `RigRef = Humanoid(RigMapRef) |
   Profile(RigProfileRef)`, so creatures without a humanoid map are
   recipes too.
3. Non-goal "creature retargeting (creatures play their own clips)" is
   relaxed to: creatures play their own clips by default; creature rigs
   that share a template and limb names may retarget at bake through the
   same `anim.retarget` (chains by name).
4. Beast-race digitigrade legs map to `leftUpperLeg`/`leftLowerLeg`/
   `leftFoot`/`leftToes` for the hub, with the extra metatarsal joint kept
   in the leg chain (`TwoBoneAnkle`), so retargeted human clips keep their
   foot contacts through the IK fix-up.

## Verification

- Unit tests (`sf_anim::rig`): auto-detection on fixtures (the CC0
  Quaternius humanoid rigs already in `sf_character/tests/data`, plus a
  CC0 quadruped and a bird rig to add, e.g. from Quaternius' animal packs
  or mesh2motion's rigs) yields the expected limbs, sides, girdles and
  solvers; the report round-trips through RON.
- `TwoBoneAnkle`: reaches reachable targets without flips; the metatarsal
  angle stays within 5° of the animated one at full extension.
- Ops on a synthetic quadruped: on a 20° slope and a 0.3 m step, every
  support foot within 2 cm of the ground (p95), body pitch within 3° of
  the slope fit, foot slide ≤ 2 cm/s during contact.
- Viewer: profile overlay (girdles, limbs colored by kind, effectors,
  poles) on every creature skeleton of a content pack; the review report
  lists skeletons that fell back to `Custom`.

## As implemented (anim-rig, 2026-10-12)

- **`sf_anim::rig`**: the data model above, with `RigProfile::build`
  (shared by detection and `sf_character`: girdles root-most first,
  sides, front-to-back orders, generated names `leg_fl`/`leg_bl`
  (two support rows), `leg_l0..` (more), `arm_l`, `wing_l`; poles stored
  as `PoleHint::Axis` in the girdle joint's frame, from the rest bend or a
  per-kind default (hind/biped knees +Z, elbows and front legs −Z, insect
  knees +Y); `FootSpec { toe, sole, max_tilt, contact }`; mirror pairs;
  `Measures`), `detect_profile` and `RigHints` (template, limb hints by
  joint names, extra chains, support height, full-profile override). The
  product is postcard (`SFRP` v1); the RON review/override file is read
  by the adapter into `RigHints::profile` (no RON in `sf_anim` at runtime).
- **Detection rules as built**: helpers are `end`/`twist`-tagged leaves;
  support leaves lie within 5% of the height above the lowest joint and
  are not under a `tail` tag; a leg root is the highest ancestor whose
  leaves are all support leaves, stopping below a joint whose support
  branches lie on both sides of the midline (the root's x, ± 2% of the
  height): that joint carries several legs and is their girdle even with
  nothing else below it (a spider body, a legs-only pelvis; before this,
  all its legs became one limb rooted there), while the splayed toes of
  one foot stay on one side and never stop the walk (rigfix, 2026-10-12;
  test `a_joint_carrying_legs_on_both_sides_is_their_girdle`); trailing `toe`-tagged joints (or one
  near-horizontal last segment) become the foot's toe; 3 joints →
  `TwoBone`; 4–5 → `TwoBoneAnkle` when the last segment is steep and its
  top joint is above 10% of the leg, unless the first segment is under
  60% of the longest (a passive scapula/coxa/hip helper: `TwoBone` on the
  last three); 6+ → FABRIK. Body root = root-most support girdle (else
  the LCA of free leaves); head = tagged head, else the free leaf most up
  and forward (moved to the skull when ears/jaw branch within 10% of the
  height). Branches off the root→head path: chains ending in a tagged
  hand or a branching joint are arms (or wings when tagged `wing`);
  linear chains behind the body root or tagged `tail` are tails; linear
  chains ending in a contact-tagged joint are non-support legs (a flyer's
  tucked legs); long, flat linear chains are wings; the rest are extra.
  Templates from the counts (2 legs Biped, 4 Quadruped, 6+ Multiped,
  wings with ≤ 2 legs Flyer, legless long chains Serpent).
- **Solvers** (`sf_anim::ik`): `LimbSolver::{TwoBone, TwoBoneAnkle,
  Fabrik, Ccd, None}` with `span()`; `two_bone_ankle_ik` (metatarsal rigid
  to the shin, extra hock bend = `AnkleShare::share` (0.5) × compression
  ratio, then two bones to the toe with the shin+metatarsal virtual bone);
  `solve_limb` with an optional effector rotation; `limb_pole` (animated
  bend, else the axis when the bend is under 5% of the chain).
- **`rig.profile` step** (`sf_anim::steps`, version 1; key: the skeleton
  product + `AnimRules::rig_hints` bytes). `anim.cook` (version 2) reads it:
  contacts default to the support-limb effectors with thresholds in leg
  lengths (`ContactRules { height: 0.06, speed: 0.35 }`), else every joint
  tagged `foot`/`claw`/`hoof`/`contact` with a leg length estimated from
  the two joints above (0.85 m fallback for a bare joint).
- **Graph ops** (`sf_anim_graph`): limbs of any length with solver,
  girdle and pole (`legs`/`limbs` table, or `profile_limbs: Support | All`
  from the `rig.profile` product, a dynamic input of `animgraph.compile`);
  `FootPlacement` per girdle (`body: Lower | Fit`, `roll`), `StrideWarp`
  per girdle, `SpineBend`, `Lean`, `LimbIk` (rotation targets),
  `ChainIk`. `HandIk` compiles to `LimbIk`.
- **Measured** (synthetic fixtures, `sf_anim::fixtures`, 60 Hz):
  quadruped (digitigrade hind legs) on 10°/20° slopes: foot slide ≤ 0.02
  mm/s, height error p95 1.4 cm, body pitch 9.2°/17.9° (gate 3°);
  stairs (0.15/0.3 m) and 0.3 m steps: slide ≈ 0. Hexapod on 15° and
  stairs: slide ≈ 0, p95 0.3–0.6 cm. Biped regression on 15°/stairs:
  slide ≤ 1.7 mm/s, p95 1.0 cm.
- **Creature fixes (anim-creatures, 2026-10-12).** Downhill 20°: the
  nearly straight front legs over-reached once the body pitched (2.7 cm/s
  slide). Lowering only the short girdle re-pitched the spine, converged
  geometrically (8 passes still 1–2 mm short) and over-pitched (−23.2°).
  The reach correction now lowers the whole body by the worst shortfall: a
  pure translation moves every hip by exactly that, so one pass is exact
  and the fitted pitch is kept (UE's Foot Placement pelvis adjustment and
  Johansen 2009 lower the pelvis by the most extended leg the same way).
  Now slide ≈ 0, pitch −18.7°, p95 1.4 cm. Step edges: the test harness
  probed in the frame before the mover moved (one extra frame, a foot
  landing within ~2 cm past an edge sampled the step below; p95 13 cm);
  it now probes as the controller glue does (after the move), and
  `GraphOutput::probe_origins` lead the animated effectors by their
  model-space motion over the last update (first-order footprint
  prediction, Johansen 2009), so the probe lands where the next update's
  foot is (prediction error 0.9 mm against 8.5 mm at the current
  effector). Stairs 0.15/0.3 m and 0.3 m steps: p95 1.4 cm, slide ≈ 0;
  biped stairs p95 1.8 → 1.0 cm. Mover shapes and modes by template:
  locomotion.md "As implemented".
- Not built yet: ragdoll, full-body IK, and a real CC0 quadruped/bird fixture (the
  synthetic ones stand in).
- Known gap: branches are read only between the body root and the head,
  so a tail hanging off an ancestor of the body root (a biped whose legs
  sit on the neck girdle, with the tail on the spine above it) is missed;
  a hint (`extra`) or a scan of the body root's ancestors would fix it.

## Industry comparison

J9 (industry-audit.md), 2026-10-12, docs research only (`.sessions/j9m3.md`).

| Technique | What shipped engines do | Us | Verdict |
|---|---|---|---|
| Describing a skeleton semantically | UE IK Rig: per-skeleton goals, solvers and **retarget chains** plus a pelvis (retarget root); "Auto Retarget Chains" matches the hierarchy to templates **by bone name** (exact names not required, conventions matter). Unity: an Avatar is Humanoid (mapped bones, muscle space, built-in humanoid IK) or Generic (no mapping; humanoid IK and retargeting don't apply). Spore: semantic parts generalize motion over user-made bodies. Creation Engine: one skeleton and behavior graph authored per creature type. OpenMW: plays each creature's authored clips; no procedural limbs (observation) | rig profile auto-detected from **geometry** (support feet near the ground, girdles at branch points, spine between girdles), tags and adapter hints; humanoid map optional on top | **match** UE's idea; **differ** in detection: geometry works on unnamed or oddly named legacy rigs where name templates fail, and hints cover the rest. Better than Unity, where a generic (creature) rig gets no IK |
| Multi-leg foot placement | UE Foot Placement node: a leg list, pelvis height modes including `FrontPlantedFeetUphill_FrontFeetDownhill` (a quadruped-style rule); Frey (GDC 2016) and Horizon's machines (Oud, GDC 2018): authored cycles plus per-girdle procedural adjustment | per-girdle offsets, body pitch and roll from them (planned, animation.md audit row 2) | **match** |
| Ragdoll from the skeleton | UE Physics Asset generation: one body per bone, capsule by default, bones under "Min Bone Size" skipped or merged ("Min Weld Size"), profiles for constraints; Unity Ragdoll Wizard (humanoid only, unverified) | `ragdoll.cook` from the profile: capsules from bone length and skin-weight extents, masses by volume, limits by limb kind | **match**; our limits by limb kind go beyond UE's uniform default profile |
| Data-driven creature motion | MANN (Zhang et al., SIGGRAPH 2018), DeepPhase (Starke et al., 2022): research | door kept open (limb phases, contact curves) | **differ, justified**: no quadruped mocap |
| Procedural stepping for clip-less creatures | indie and tech-demo practice (step when a foot's target drifts past a radius, alternate leg groups); not a documented AAA engine feature we could find | creatures need authored cycles; `Custom` rigs get root motion and look-at only | considered, **not adopted**: every creature in the content has clips. Reconsider for mods with clip-less rigs (open question 3) |

Sources (checked 2026-10-12):
- UE IK Rig retargeting and auto retarget chains: https://dev.epicgames.com/documentation/en-us/unreal-engine/ik-rig-animation-retargeting-in-unreal-engine ; https://dev.epicgames.com/documentation/en-us/unreal-engine/auto-retargeting-in-unreal-engine
- UE Foot Placement pelvis modes: https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/AnimationWarpingRuntime/EPelvisHeightMode
- UE Physics Asset tools (Min Bone Size, primitive type, Min Weld Size): https://dev.epicgames.com/documentation/unreal-engine/physics-asset-editor-in-unreal-engine---tools-and-profiles
- Unity humanoid vs generic (muscle space): https://docs.unity.cn/6000.4/Documentation/Manual/AnimationOverview.html
- Oud, GDC 2022 (Horizon machines and characters, abstract): https://gdcvault.com/play/1027894/Evolving-the-Horizon-Series-Animating

**Unverified**: Unity's built-in IK being humanoid-only and the Ragdoll
Wizard being humanoid-only (from memory); UE Foot Placement taking any
number of legs (the leg list is in the API, quadruped samples unverified).

## Open questions

1. Whether `rig.profile` is its own bake product or a section of
   `skeleton.cook` (simpler keys, but every profile tweak re-cooks
   skeletons and everything downstream). Own product for now.
2. Banked flight and wing flapping synchronized to vertical speed: a
   procedural op or authored blend spaces per creature. Authored first.
3. Procedural stepping for creatures with no locomotion clips (Industry
   comparison): a `ProceduralGait` op (feet step when their ground target
   drifts past a radius, alternating limb groups from the profile) would
   let mod rigs walk without clips. Not needed for current content; revisit
   when a content pack ships clip-less creatures.
