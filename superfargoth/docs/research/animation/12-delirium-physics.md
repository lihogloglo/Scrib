# R12: delirium-physics and physical reactions on authored animation

Research and planning only (session `delirium-plan`, 2026-10-22). Nothing was built or run.
Source read: [PVCPC/delirium-physics](https://github.com/PVCPC/delirium-physics) at `3503d71` (v0.2.0,
2026-10-22), shallow clone in `C:\cargo-target\refs\delirium-physics`. **Licence rule:** that repo is
GPL-3.0-only. Nothing below quotes or paraphrases its code. superfargoth implementation agents must not open
the clone (CLAUDE.md rule 2). This note and the specs are their only source.

## Verdict

**Don't adopt delirium-physics as a dependency, and don't port it.** Build the missing layer on what we
already have. Delirium is an impressive Euphoria-style "behaviour brain" (balance, stumble, catch fall, brace,
writhe, high fall), but it fits our needs badly in five ways:

1. **It inverts our model.** Its character is unanimated: muscles drive the pose, and following an animation
   is a side message. We need the authored clip as the target, with physics on top. That is what
   `sf_ragdoll::powered` already does (P8/P8b).
2. **It brings a second physics engine.** It has its own solver and collision, which would duplicate our
   Rapier world. Its "FOLLOW" mode outputs explicit per-part forces and torques: the explicit-PD path that
   physics.md rejected for stability at high gains.
3. **It handles one body only.** It has a fixed 21-part, 1.8 m, Z-up humanoid; we need every rig in
   rig-profile.md, including creatures.
4. **It is 6 days old.** It has 38 commits, one author, and 56 k dense lines.
5. **Its provenance is unclear.** It reproduces NaturalMotion's proprietary message API (names, parameters,
   defaults). Its playroom also decrypts GTA IV executables, using console keys fetched from RPCS3. A
   "clean-room" provenance section was proposed in issue #1 but is not in the README. The re3 GTA
   decompilation was taken down and sued by Take-Two (2021). A GPL licence does not cure third-party IP, so this
   risk applies to *both* repos.

**Licence recommendation:** keep superfargoth MIT/Apache. Write the techniques **clean-room into
superfargoth from published sources** (papers, engine docs, observed behaviour). These are listed below and
none needs Delirium. Delirium's *public docs* may inform the architecture: behaviours write joint targets and
stiffness, body masks, a feedback list, LEAD vs FOLLOW. That is how-it-works.md, which describes ideas, not
code.

**Alternatives:**
- (A) A bevyrrowind-only `mw_delirium` FOLLOW-mode spike. Licence-legal (bevyrrowind is GPL-3.0-or-later, so
  the binary becomes GPL-3.0 only). Not recommended until provenance is settled and the project matures. See
  slice R9.
- (C) Relicense superfargoth to GPL. Weighed under "Licence options" below. It does not simplify as much as
  hoped: the split exists mainly for the adapter boundary, not the licence.

## What already exists (don't rebuild)

| Piece | Where | State |
|---|---|---|
| Ragdoll cook from any rig (capsules, limits by limb kind, mass balancing) | `sf_ragdoll::cook`, `ragdoll.cook` bake | built P7; 3 humanoid + 83 creature skeletons cook |
| Limp death ragdoll, velocities from the last poses | `sf_physics_bevy::ragdoll`, `PhysicsPose` | built; **PH-2 rejected**: the mesh doesn't follow the capsules |
| Powered ragdoll: stable PD via Rapier implicit motors, composite-inertia gains, world pelvis drive with gravity compensation | `sf_ragdoll::powered`, `PhysicsWorld::set_joint_target` | core built P8/P8b; tracking 6.8 cm in air / 11 cm on floor (gate 5), 0.129 ms/char (gate 0.05) |
| Graph physics stage (per-joint weights, after inertialization, before ops) and `inertialize_from` get-up hand-back | `sf_anim_graph` `set_physics` | built P8; continuity tested |
| `RagdollMode::Death { crumple }`, `Knockdown` (brace 0.25 → limp 0.8 s, get-up on rest), budget 8 | `sf_physics_bevy::ragdoll` | built; **PH-9 partial**: "fall animation basically none" |
| Kinematic `HitReact` springs and `SpringBones` | `sf_anim_graph::secondary` | built P9; **PH-10 bug** (soft hits never logged) |
| `Physics` owner and `Forced` move kind (stagger, knockdown, get-up, death) | action.md | specified; `sf_action` A1 in progress |
| Net shape: ragdolls host-owned, per-body poses at reduced rate, reactions as host events | net.md | specified (N5) |

The gap is therefore **not** the drive layer. It is (1) the 3.2 write-back bug, (2) a **reaction controller**
that turns hits into a strength-and-target timeline, (3) **balance and protective behaviours** (stumble steps,
bracing arms, writhe), (4) **authored reaction clips as drive targets** (Morrowind ships `Hit1-5`, `KnockDown`,
`KnockOut`, `DeathKnockDown`, `DeathKnockOut`, `Death1-5`), and (5) LOD, cost and net wiring.

## Delirium in brief (for the record)

- **Licence and versions:** GPL-3.0-only. The engine crate uses only `bevy_math`/`bevy_transform` 0.18.1 (plus
  `bevy_ecs` behind a feature). The playroom is Bevy 0.18.1 with egui and `rapier3d` 0.36.1 for props. We are
  on Bevy 0.20 with the same Rapier.
- **Model:** 21 capsules and 20 joints (ball with cone and twist, or hinge). Each joint has an "effector": a
  spring-damper with stiffness, damping, passive muscle stiffness and a desired angle. About 55 messages
  (30 core, 25 extra) start or stop behaviours on body masks (whole, upper, arms, legs, spine, neck). Several
  behaviours run at once on disjoint effectors. A feedback list reports events (balance lost, catchFall...).
  It steps at a fixed 60 Hz with up to 8 substeps.
- **Animation input:** a pose of 7 floats per part (`incomingTransforms`, `activePose`), `softKeyframe`,
  `hardConstraint`, and a text pose (`animPose`). Get-up comes from a clip, muscles following a clip, or a
  small evolution-strategies network (`getup.net`).
- **Inputs:** a character file plus a "build dump" or a JSON `nm-character-1`, rescaled by `nm_build_fit`
  (tuned for about 1.8 m). The world is a floor, triangle meshes, and moving boxes reset every step.
- **Performance:** no published numbers. Issue #9 reports "FPS abysmal" (rendering, iGPU). Uses AVX2 via
  `target-cpu=x86-64-v3`. Unmeasured by us: no build in this slice.

## Industry practice (what we follow)

| Approach | Shipped in | Animation role | Our match |
|---|---|---|---|
| Euphoria (NaturalMotion): full active ragdoll with behaviour controllers, game tasks send messages | GTA IV/V, Red Dead 1/2, Force Unleashed | secondary: physics leads during a reaction, then hands back | we take the *behaviour layer idea* for a few close characters only |
| Powered ragdoll with per-body strength, pin to animation, blend weight | UE Physical Animation Component, "Set All Bodies Below Physics Blend Weight"; UE5 Physics Control Component / RigidBodyWithControl; Havok powered ragdolls; Frostbite (Sachania, GDC 2018) | **master**: clip is the target, strength dips react | **our base** (P8 already) |
| Muscle weight + pin weight, puppet states (Puppet, Unpinned, GetUp), collision resistance | Unity PuppetMaster (RootMotion) | master | our `StrengthProfile` + pelvis drive + reaction states |
| Kinematic additive hit clips and springs | most action games | master | `HitReact` (far tiers, beyond budget) |
| Hybrid recovery: find the clip matching the simulated pose and blend back | Zordan, Majkowska, Chiu, Fast, *Dynamic Response for Motion Capture Animation*, SIGGRAPH 2005; Shapiro, Pighin, Faloutsos, *Hybrid Control*, 2003; Mandel, CMU thesis 2004 | master after recovery | get-up and stumble-exit selection |
| Balance feedback and step placement | SIMBICON (Yin, Loken, van de Panne, SIGGRAPH 2007); capture point (Pratt et al., Humanoids 2006) | n/a | stumble steps |

The shape is consensus for animation-first games: the clip is the target, a powered ragdoll tracks it, and
reactions lower strength per region along authored curves. The ragdoll goes full physics only for falls and
death, and pose-matched clips return control. A behaviour brain is a premium extra, budgeted to a few close
characters.

## Pose pipeline with the physics stage

```
 FixedUpdate (60 Hz)                                    Update (per frame)
 ───────────────────                                    ──────────────────
 sf_action tick ── Forced move? ──► ReactionRequest     graph evaluate (sf_anim_graph)
   (hits, poise rules)            (class, body, point,    sample/blend/inertialize → local pose
        │                          impulse, tick, seed)          │ model pose = DRIVE TARGET
        ▼                                 │                      ▼
 sf_ragdoll::react (NEW, Bevy-free) ◄─────┘          ┌── physics blend (set_physics, per-joint weight
   state: Tracking → Stagger → Falling → Down →      │     from region weights; 0 = clip, 1 = body)
          GettingUp;  strength/target timeline       │          ▼
   balance (CoM / capture point vs support)          │   procedural ops (foot IK, look-at, HitReact)
   protective targets (brace arms, head)             │     only where physics weight < 1
        ▼                                            │          ▼
 sf_ragdoll::powered drive (implicit PD motors,      │   write_bones (sf_anim_bevy) → GPU skinning
   pelvis drive) on last frame's drive target        │
        ▼                                            │
 Rapier step (sf_physics) ── interpolated read-back ─┘   feedback events → sf_action (fell, settled, got up)
```

The drive target is the previous frame's evaluated pose: one frame of latency, the same as UE. The reaction
clip (`KnockDown`, `Hit2`...) plays in the graph as a normal `Forced` move. Physics follows it at reduced
strength, so the fall *looks authored* and still collides with stairs and walls.

## Design rules for the slices

1. **The clip stays the target.** The reaction controller changes only strength, region weights, a few
   procedural target offsets (brace arms, head tuck, stumble foot) and impulses, never the clip selection.
   Clip choice is `sf_action`'s job (Forced moves).
2. **Gameplay decides outcome classes; physics flavours them.** Stagger or knockdown comes from rules: poise
   or fatigue in the rules plugin, Morrowind's knockdown rule in the adapter. A balance loss on its own may
   *upgrade* a stagger to a fall only on the host, and is then reported as an event. This keeps co-op
   host-authoritative and design-controllable (action.md: gameplay never reads the evaluated pose).
3. **Net:** clients receive `ReactionRequest` and outcome events with their tick, run the same controller
   locally as presentation, and converge to the host's pelvis and key body poses (net.md N5: reduced-rate
   per-body poses). Corpse rest position (loot, collision) is host state; limb poses are cosmetic.
4. **Game-agnostic:** reaction classes, curves and thresholds are data (RON). Morrowind clip names and rules
   live in `mw_physics` and `mw_graphs`.
5. **LOD tiers** follow `AnimLod` significance:

| Tier | Who | Physics | Cost target (owner's 4060 laptop CPU) |
|---|---|---|---|
| R0 behaviour | ≤ 4: player's target and attackers within 10 m | powered + balance + protective behaviours | ≤ 0.08 ms each |
| R1 powered | ≤ 8 total incl. R0, A0 only | powered tracking, strength dips, falls | ≤ 0.05 ms each (P8 gate) |
| R2 kinematic | other A0-A1 | `HitReact` springs + reaction clips | ~0 physics |
| R3 | A2-A3 | clips only, outcomes still apply | 0 |
| Limp | any dead | limp until asleep (≤ 3 s), then frozen | sleeping ≈ 0 |

Budget: about **0.6 ms** of physics for reactions at peak (4 × 0.08 + 4 × 0.05 + limp). That is about 5 % of
the 11 ms High budget and has to be measured, not assumed. The allocator demotes R0 → R1 → R2 under budget,
as `AnimBudget` already does.

## Implementation plan

Each slice is 1-2 steps for one agent (~60 tool calls, ~100 k tokens). Cargo through
`bash bevyrrowind/tools/cargo_turn.sh`. Every slice ends with headless tests, a spec update (physics.md
"Ragdolls", animation.md Layer 4, physics-adapter.md) and one visual-checks.md entry (≤ 10 lines, "awaiting
owner"). **R0-R1 are ROADMAP 3.2; R2-R5 are 3.2's "powered falls"; R6 needs 3.5; R8 needs N5.**

### R0. Fix the ragdoll pose write-back (3.2 lead)
- **Goal:** the skinned mesh follows the capsules (PH-2 "sink, float, turn, odd angles").
- **Scope:** `sf_ragdoll/src/instance.rs` (`local_pose`), `sf_physics_bevy/src/ragdoll.rs` (`read_back`,
  frame follow), `sf_anim_bevy` (`write_bones`, evaluate with `PhysicsPose`).
- **Suspects to test first, in order:**
  - the frame passed to `local_pose` against the frame the bones are parented to: the entity frame or the
    NIF scene root / `Bip01` offset, and big_space cell rebasing;
  - procedural ops (foot placement) still running on joints at physics weight 1;
  - LOD frames where the graph doesn't pose and physics joints below weight 1 keep stale transforms;
  - `Knockdown` mode, whose frame does not follow the pelvis (only `Death` does).
- **Acceptance:** a new headless test spawns a cooked humanoid ragdoll, drops it on a slope with the entity
  frame moved, rotated and cell-rebased mid-fall, and asserts every body's world pose against its owner
  bone's world transform after write-back (≤ 1 cm, ≤ 2°). PH-2 re-entered with the `G` collider overlay.
- **Risks:** more than one cause. Cap at 3 screenshot iterations, then log and hand off.
- **As built (R0, 2026-10-22):** test `sf_physics_bevy` `ragdoll::tests::bones_follow_bodies_{death,knockdown}` (1.8 m / 75° before,
  < 0.1 mm / 0.07° after). Causes: (1) joints above the root body had physics weight 0, so the clip's root (and its
  root motion moving the frame) placed the mesh; now every joint is weight 1 and root motion is off under a physics
  pose; (2) non-uniform build scale (X/Z ≠ Y) skewed lying bones off the uniformly scaled bodies (8 cm, 18°): the
  handoff makes the entity scale uniform, the get-up restores it; (3) body joints now take positions from their bodies
  (no stretch drift down chains); (4) the frame follows the pelvis in Knockdown too; (5) ops skip weight-1 joints.

### R1. Ragdoll lab test level + reaction scenes
- **Goal:** a repeatable synthetic stage for every later slice, like Delirium's playroom stages (an idea, not
  code): flat floor, stairs, low wall, ledge, slope, and a dummy NPC on a loop. Debug keys: push impulse
  sweep, kill, knockdown, cycle strength profile.
- **Scope:** `bevyrrowind/levels/ragdoll-lab.ron`, `scenes.ron`, `apps/bevyrrowind` debug driver (existing
  `I`/`V` keys), test-levels.md.
- **Depends:** R0. **Acceptance:** `cargo dev scene level-ragdoll-lab` opens. `BEVYRROWIND_AUTO` scripts for
  push/kill/knockdown run headless-ish and log stretch and tracking stats. Visual check RL-1.
- **Risks:** scope creep into a playground UI. Keys and auto scripts only.

### R2. Reaction controller core (`sf_ragdoll::react`)
- **Goal:** a Bevy-free state machine (`Tracking → Stagger → Falling → Down → GettingUp`, plus `Dead`). It
  turns a `ReactionRequest { class, body, point, impulse, tick, seed }` into per-region strength and
  physics-weight timelines from a RON `ReactionSet` (curves per class: stagger dip and recover, knockdown
  powered-fall strength, death crumple). It emits feedback events (`Fell`, `Settled`, `GotUp`). This replaces
  the hardcoded `Knockdown { brace 0.25 / 0.8 s }` and `Death { crumple }` constants.
- **Scope:** `sf_ragdoll` (new `react.rs`), `sf_physics_bevy/src/ragdoll.rs` (modes call it), physics.md.
- **Depends:** R0. **Acceptance:** headless tests check deterministic timelines (same request, same curve),
  frame-rate independence (30/60/144 fps), and that stagger returns to tracking with strength continuous.
  Existing P8 gates still pass. Visual check: R1 lab, stagger vs knockdown side by side.
- **Risks:** overlap with `sf_action` Forced moves. The controller never picks clips (rule 1).
- **As built (R2, 2026-10-22):** `sf_ragdoll::react` (`ReactionController::request/step`, `ReactionSet` RON in
  `data/reactions.ron`, regions Legs/Spine/Arms/Head by `Region::of_name`, events `Fell/Settled/GettingUp/GotUp/Died`);
  17 unit tests (transitions, seed determinism, 30/60/144 fps, stagger continuity, RON round trip, `validate`).
  Wired: `RagdollConfig::reactions` replaces crumple/brace/rest constants; knockdown and death strengths and the
  get-up trigger come from the controller (`knockdown_gets_up_through_the_controller`). Left for R3/R4: Stagger and
  physics-weight outputs are not consumed yet (weights stay 1), `upright`/`grounded` sensing, per-region profiles.
  Known change: a limp (over-budget) knockdown now waits for the 0.8 s curve before getting up.

- **As built (R3-3, 2026-10-22):** knockdown hand-off closed: `RagdollFailed` for requests that can't spawn, every get-up path (incl. skipped `GettingUp`) hands back, `ReactionSet::settle_timeout`; get-up frame on the ground, pelvis-anchored hand-back pose, hand-back blend owns the switch; in-place get-up tails anchored on their last frame (AN-4..6).
### R3. Powered falls on authored clips (PH-9 "no fall animation")
- **Goal:** knockdown and death follow the content pack's reaction clips (`KnockDown`, `DeathKnockDown`,
  `Death1-5`) as drive targets at the R2 strength curve, instead of going limp at once. Falls look authored
  and still collide.
- **Scope:** `mw_graphs` (Forced reaction states on the humanoid and creature graphs), `mw_physics` (class →
  clip map, Morrowind knockdown and knockout rules), `sf_physics_bevy` (drive target from the playing
  reaction clip), physics-adapter.md.
- **Depends:** R2; `sf_action` Forced moves if landed (else a named scaffold `REACTION_DIRECT` that starts the
  clip from the ragdoll driver, removed when the action layer lands).
- **Acceptance:** headless test checks that, during a powered fall on flat ground, joint tracking error to the
  clip stays ≤ 10° RMS until first ground contact, then the pose diverges (collision wins). Re-run PH-9 in the
  lab and on the Balmora stairs.
- **Risks:** the tracking gate (P8b: 11 cm on floors) may need the effector work first. If so, accept on the
  joint angles and log it.

### R4. Balance and stumble steps
- **Goal:** below the knockdown class, pushes cause stumble steps. CoM and capture point are tested against
  the support polygon of `support` limbs' contacts (SIMBICON / capture point). Inside the polygon: strength
  dip only. Outside but recoverable: a step target goes to the mover (intent nudge) and foot IK places the
  stepping foot (legs kinematic-weighted, upper body powered). Beyond that: a `Fell` event (host-side upgrade,
  rule 2).
- **Scope:** `sf_ragdoll::react` (balance), `sf_controller`/`sf_locomotion` intent hook (read only, one
  hook), animation.md Layer 4.
- **Depends:** R2, R1. **Acceptance:** headless impulse sweep (0-40 N·s, 8 directions, flat and 10° slope):
  monotonic recovered → stumbled → fell bands, no fall below the stagger threshold, no foot through the
  floor. Visual check: push sweep in the lab.
- **Risks:** stepping fights root motion. The mover stays the owner (action.md `Sim`). Physics only proposes
  the step.

### R5. Protective behaviours: brace, head tuck, writhe
- **Goal:** the Euphoria flavour, from papers and observation. While `Falling`: arm targets reach toward
  the predicted ground contact (two-bone IK on the drive target) and the head tucks. On `Down`: a short
  low-strength writhe (seeded noise on spine and limb targets) before settling. All of this is added to the
  clip target, never replacing it.
- **Scope:** `sf_ragdoll::react` (protective offsets), `sf_anim` IK helpers (reuse), ReactionSet RON.
- **Depends:** R3. **Acceptance:** in a forward-fall sweep (20 seeds), hands touch the ground before the head
  in ≥ 80 % of falls. The writhe is bounded (≤ 15° offsets) and stops in ≤ 2 s. Cost stays inside the R0 tier
  budget.
- **Risks:** looking floaty. Owner visual sign-off is the gate.

### R6. Get-up and stumble exit by pose matching
- **Goal:** choose the get-up clip (face up, face down, side) and the stumble exit by matching the ragdoll
  pose to clip entry frames (Zordan 2005 style: weighted joint and pelvis-orientation distance), then the
  existing `inertialize_from`. This removes `ROLL_TO_BACK` once a face-down clip exists (packs only, owner
  rule).
- **Scope:** `sf_ragdoll::react`, `sf_anim_graph` (start-frame selection shared with action.md's gap-sized
  blend work), `mw_graphs` slot map.
- **Depends:** R3; a face-down get-up clip from a pack (animation-sources.md). **Acceptance:** angular velocity
  continuous at shrinking dt (existing test), best-match selection unit tests, feet planted after a get-up on
  stairs (PH-2 second half).
- **Risks:** clip availability. Keep the scaffold until a clip lands.

### R7. Combat wiring (3.5)
- **Goal:** `sf_action` Forced moves and E1 hit events create `ReactionRequest`s. The poise/stagger rules
  hook goes in the rules plugin, with the Morrowind adapter mapping fatigue, Hand-to-Hand and spell knockdowns.
  Fix PH-10 (soft hits never reaching `HitReact`) on the way.
- **Scope:** `sf_action` (forced output), `sf_rpg` rules hook, `mw_rpg`/`mw_physics`, app glue.
- **Depends:** R2, ROADMAP 3.5 / action E1. **Acceptance:** scripted-input combat test: light hit → stagger
  class, heavy hit at low fatigue → knockdown, death → powered death fall. Outcome identical with physics
  off (rule 2).
- **Risks:** blocked on combat. Don't start before E1.

### R8. Reaction LOD, budget and cost gate
- **Goal:** the tier table above in the allocator (R0/R1/R2/R3, hysteresis), and the 0.05 ms P8 gate.
  Levers, in order: sleep limp bodies sooner, fewer `powered_iterations` with substep tuning, then Rapier
  multibody (reduced-coordinate) ragdolls as a measured A/B. Record a `sf_diag` gate.
- **Scope:** `sf_physics_bevy` (allocator), `sf_ragdoll::powered`, `sf_physics` (multibody option), perf
  bench scene.
- **Depends:** R2. **Acceptance:** bench "8 powered + 20 limp on stairs" in release: reaction physics
  ≤ 0.6 ms per frame on the owner's laptop (run only when the owner is not testing), demotion logged, no
  visible pop on demotion (inertialized to clip).
- **Risks:** Rapier cost floor. Measure multibodies before giving up on the gate.

### R9 (optional, owner decision). `mw_delirium` FOLLOW-mode spike, bevyrrowind only
- **Goal:** answer "is its brain better than ours?" with numbers, if the owner accepts the provenance risk.
  Map the 21 parts onto the cooked humanoid ragdoll, feed part states from Rapier, apply its drives as
  forces, and run the lab push sweep against R4/R5.
- **Scope:** new `bevyrrowind/crates/mw_delirium` (git dependency pinned to a commit, behind a feature, never in
  superfargoth).
- **Depends:** R1, R4; provenance answered (open question 2). **Acceptance:** sweep table plus ms/char.
  Delete the crate if it loses.
- **Risks:** IP takedown upstream; explicit forces unstable in Rapier; Bevy 0.18 → 0.20 glam mismatch in its
  types.

R0-R5 and R8 make 7 slices for the core feature. R6 and R7 follow their dependencies. R9 is optional.

## Licence options (neutral)

| | (A) Delirium in bevyrrowind only | (B) Clean-room into superfargoth | (C) Relicense superfargoth to GPL |
|---|---|---|---|
| Legal | fine (the binary is already GPL; becomes v3-only) | fine if specs cite papers and docs and implementers never read Delirium code | owner is the **sole author** (147 commits, one author), so it is the owner's call alone |
| Third-party IP (Euphoria) | **inherited** | avoided (techniques from 2003-2007 papers, public engine docs) | doesn't help: GPL can't cure it |
| Architecture | a second physics engine and a humanoid-only path beside `sf_ragdoll` | one ragdoll stack, all rigs | the framework/game split **stays** (see below) |
| Cost | R9 only, plus upkeep against a 6-day-old repo | R2-R5 (already planned in Layer 4) | about an hour of files; no code changes |

**Would GPL simplify the architecture? Partly, and less than it looks.**
- **The split is not mainly a licence wall.** ARCHITECTURE §2 principles 1-2 (game-agnostic litmus test,
  compile-time boundary) and §8b (compile time: Bevy-free cores, wide crates) justify two workspaces on their
  own. GPL would merge neither the adapter boundary nor the crate graph.
- **What GPL would remove:** the "no GPL-derived code" rule in superfargoth (OpenMW, `nif.xml`, MWSE, Mudcrab
  since 2026-10-21, Delirium), and the provenance audits. Its value is modest. R10 found Mudcrab has no
  animation code and a Luau runtime behind ours. OpenMW knowledge already flows freely into bevyrrowind, which
  is where Morrowind specifics belong anyway.
- **What GPL would not remove:** the "clean room for other games" rule (decompiled binaries, REA). That protects
  against the *game owners'* IP, not licence mixing.
- **What it would cost:**
  - superfargoth stops being usable by closed-source games, including most commercial Steam projects.
  - Console ports become impractical: platform SDK NDAs are GPL-incompatible.
  - It falls outside the Bevy ecosystem's MIT/Apache norm, so fewer outside contributors.
  - The vendored Bevy patches meant as upstream PRs must stay separately MIT, with no GPL-derived lines.
- **Dependencies are not a blocker either way:** MIT, Apache-2.0, BSD, zlib and MPL-2.0 are GPLv3-compatible.
- **The real dependency question exists today:** the proprietary NVIDIA DLSS SDK linked into a GPL binary
  (bevyrrowind `dlss` feature). Relicensing the framework would spread that question to it.
- **Reversibility:** relicensing to GPL is easy now. Once GPL-only third-party code lands, going back to MIT is
  impossible. Staying MIT keeps both doors open.

**Recommendation:** (B), keeping MIT/Apache. Revisit (C) only if a concrete GPL body of code (not Delirium,
given its provenance) becomes worth more than framework reuse. (A) only as the R9 experiment.

## Open questions for the owner

1. Licence: keep superfargoth MIT/Apache and clean-room (recommended), or relicense to GPL?
2. Delirium's provenance risk: allow the bevyrrowind-only R9 spike at all? Do you own GTA IV (its playroom
   needs it, so it can serve as a black-box oracle)?
3. Who decides a fall: rules (poise/fatigue, recommended, net-safe), or physics balance loss as well (host
   only)?
4. Budget: are 4 behaviour + 8 powered characters right for Morrowind fights? Should the Classic preset
   (vanilla option) use clips only?
5. Clips: are face-down get-up and stumble-exit clips from free packs acceptable (R6)?
6. Is the existing DLSS-in-a-GPL-binary distribution question already settled?

## As built (Delirium in game, slice 1, 2026-10-26)

Supersedes R9 above: Delirium entered the game as ROADMAP 3.2b, not as the FOLLOW-mode spike.
- `bevyrrowind/crates/mw_delirium` (Bevy-free, GPL-3.0-only; git dep delirium-physics @ 91ee246, `delirium` +
  `delirium-sdk`). `character_from_skeleton(&Skeleton, &Options)` builds the `nm-character-1` asset and its build
  from `base_anim.nif`'s `Bip01` bones (humanoid rig map; Spine3 halfway chest bone to neck), porting
  skyrim-delirium's generator (`make_character.rs` @ e44cb38): pelvis frame a = up / b = left / c = back, zero pose
  arms 55 deg out and legs straight, trunk-low Winter masses (80 kg), mirrored right side with the flips, gains 12/9/8.
- `PartTable` + `pose_to_bones` / `bones_to_parts`: each part owns one bone (Spine2 reads `Bip01 Spine2`, Spine3
  writes it); other bones keep the clip pose; children keep clip translations. `sim`: headless harness (floor, push).
- Spaces: model (Y up, +Z facing) -> authoring (Z up, +Y facing, = NIF axes, metres) -> sim (translation only).
- Results (`tests/real_install.rs`): stands 60 s (pelvis 99 %, still from 0.6 s); push sweep 8 dir x 20/45/80 Ns:
  8 stayed, 16 stepped, 0 fell, **24/24 come to rest** (median 2.6 s, max 10.7 s); catchFall hands before head
  10/10; scaled x1.1 stands; pose round trips exact (bones), 8 mm (simulated parts, via the read-only Spine2).
  0.41-0.57 ms/step (release, shared CPU).
- What mattered vs our d1b spike: the whole port, not one knob. Ablations (one process each, env vars in the test):
  arms down (0 deg) 24/24 rest; even trunk mass 22/24; spawning in the zero pose or `Agent::set_zero` on the spawn
  pose keep it fidgeting while standing (still only at 37.6 / 53 s). Keep: bind-pose spawn, asset zero pose.
- Slice 2 needs: Bevy plugin (takeover from the clip with part velocities, per-actor `Build::scaled`, actor
  transform as `model_from_sim`), click-to-push, copy-back before skinning, hand-back once balanced, get-up clips.

## As built (slice 2a, 2026-10-26): takeover of an animated NPC
- `mw_delirium::takeover` (Bevy-free): `Takeover::start` from part frames (+ per-part velocities from the previous pose,
  `Ragdoll::set_body`), `Build::scaled` from the actor's scale, floor at the given ground (lowered to the lowest part); `bodyBalance` (or `catchFall` when the
  pelvis sinks > 1 m/s). `step` returns `HandBack::{Balanced, Timeout, Fallen, Exploded}` (`Rules`: still = CoM < 0.05 m/s
  for 0.5 s; down = `agent.fallen` or pelvis < 50 %, then 1.5-4 s). `pick`: ray vs part capsules. Tests (real install):
  rest-pose takeover hands back Balanced at 1.0 s; 80 N s at scale 1.05 Balanced at 2.9 s; a falling pose starts
  catchFall, Fallen at 2.7 s; picks hit the aimed part; 0.42 ms/step.
- `mw_delirium_bevy` (app feature `delirium`, in `dev`): request `DeliriumReaction` on the root; two posed frames, then
  takeover, root `Dormant`; FixedUpdate step; copy-back as `PhysicsPose` weight 1 (the ragdoll path: ops skip, no root
  motion), parts interpolated by the fixed overstep; body frame on the floor under the pelvis (heading from the hips),
  uniform scale until hand-back. Floor (audit F5, 2026-10-26): one `Physics` ray down from the pelvis (no hit: the render
  height); the frame sits on it, so the hand-back root move takes the settled height (mover `height_offset` zeroed);
  ramp test: pelvis blends 1.070 to 1.091 m in the frame (was 0.970 to 1.091 at the render height). Hand-back:
  `inertialize_from` 0.3 s; fallen: `knocked_out` set while down, frame to
  `get_up_root` on the `Knockout@tail` first pelvis pose, flag cleared; root takes the body's move (walkers' glue).
  Dev click (left; Ctrl 20 / 45 / Shift 80 N s). Human skeletons only (beasts logged). Live: balanced hand-backs seen.
- Next: fall + get-up live check (BRP clicks were unreliable), Rapier statics into Delirium's world, non-uniform scale
  pelvis anchor at hand-back (as `anchor_pelvis`), a BRP agent tool for scripted pushes.
