# Godotwind audit, half A (water, controller, animation, deformation, interaction), 2026-10-26

Source: `D:\Gamedev\Godotwind\godotwind` (MIT). Scope: water + buoyancy, character controller/moves, animation, deformation,
interaction. Method: file headers and main functions, compared with our specs' agent summaries and crates.
Headline: **our specs are already well ahead of Godotwind in all five areas** (Godotwind's controller is a 9-state
CharacterBody3D machine, its buoyancy a box-cell Archimedes sum, its animation a Godot AnimationTree + IK wrapper).
The useful parts are small: a few algorithms, one missing feature (ground deformation), and a lessons list.
Godotwind constants (speeds, strengths, thresholds) are not designs; they are listed only as "starting point to measure".

## 1. Water, swimming, buoyancy (ROADMAP 3.6, 3.7)

| What it does | Godotwind file(s) | Our counterpart | Value | Effort | Note |
|---|---|---|---|---|---|
| Swim move: camera-relative 3D velocity, vertical intent (jump up, crouch down), spring toward equilibrium depth `clamp(depth*k, -2, 2)`, feet depth 1.2 m for 1.8 m body | `character/controller/moves/swim_move.gd`, `swim_idle_move.gd` | `Swim { surface }` in `locomotion.md`; `sf_controller` | low | S | Ours takes the surface from the query. Only idea worth keeping: swim_idle damps vertical velocity inside a +-0.3 m band so the idle bob settles (a starting point to measure). |
| Water detection: ocean wave height every frame plus Area volumes; flags `is_in_water`, `water_surface_y` injected into input | `docs/systems/character_controller.md` | water query (`water.md` "The water query") | low | - | Same contract as ours. Nothing to take. |
| Buoyancy by box cells: submerged fraction by cell top/bottom vs water height, `F = rho*V_sub*g`, quadratic drag `v^2 * V_sub`, gravity applied per cell (parent gravity off) | `water/buoyant_body.gd`, `buoyancy_body.gd`, `buoyancy_probe.gd` | pontoon spheres, spherical-cap fraction, relative-velocity drag (`water.md` "Buoyancy") | low | - | Ours is better (flow-relative drag, torque damping). Tip: they disable engine gravity and apply it per cell; we do not need that. |
| Frozen-body guard: a frozen RigidBody still ticks `_physics_process`; early-out on `freeze` | `docs/systems/interaction_system.md` 6.4 | `BodyKind::Held`/asleep in `sf_physics` | low | S | Test idea only: assert floaters skip asleep and `Held` bodies (no force accumulation, no wake). |
| Auto-probe layout for items: 4 bottom AABB corners + centre, generated at spawn | `carryable_body_factory.gd` | `water.pontoons` bake (voxel + k-means) | low | - | Ours is better. A cheap fallback if the bake lacks data for an item. |
| CPU wave query synced to GPU FFT: top-K spectral components by energy (<=256), truncated DFT sum for height/displacement/normal; no readback ("Sea of Thieves") | `water/ocean_physics_evaluator.gd` | `sf_water/src/twin.rs` (CPU twin of the GPU cascades, agree within 1 cm) | low | - | We already have the stronger exact twin. Only if the twin proves too slow at 32 floaters: top-K truncation is the fallback (measure first). |
| Impact + wake impulses from probes: surface-crossing detection (depth crosses +-0.02 m), splash strength from vertical + relative lateral speed, wake only inside a band (0.45 m) when relative speed > 0.15 m/s, emit by interval or distance, wake direction and length from relative velocity, gated by coverage; optional flow obstacles (block/wake strength) | `water/water_interactor.gd`, `water_interaction_sim.gd` | "Wading, wetness and interaction hooks" + dynamic ripples (iWave 256^2) in `water.md`; `sf_precip_bevy` | **med** | S | Good ready recipe for the emitter side of 3.6 (player wade/swim splash and wake, thrown item splash). Use relative velocity (minus water velocity), as ours already specifies. Thresholds: starting points to measure. |
| Retained wetness on movable objects: wet when submerged, dries at a rate (`dry_rate`), shared manager, opt-in component | `water/wettable_object.gd`, `wetness_manager.gd` | `sf_surface` wetness (character wet line, dries) in `water.md`; ROADMAP SU1/SU2 | low | S | Same idea as ours; confirms that dropped items should be wettable too (3.7). |
| Underwater motes: camera-followed GPU particles with world-snapped spawn cell (4 m anchor), so swimming passes through fixed motes instead of a camera shell | `water/underwater_particulates.gd` | none (froxel water medium only) | **med** | S | Cheap motion-parallax cue that fog alone lacks. In Bevy: world-anchored `bevy_hanabi`/instanced points inside the froxel volume; quality tiers off/low/med/high. Fits 3.6. |
| Optical profile: `visibility_distance_m` -> extinction so transmittance hits 1% at that distance, per-channel bias (red dies first), clamp 0.5-500 m, turbidity multiplier <=2x | `water/water_optical_profile.gd` | extinction/albedo in the water medium (`water.md` "Underwater"); `mw_water` | med | S | Authoring by visibility distance (`sigma = -ln(0.01)/d`) is easier to tune than raw coefficients; adopt as the `WaterRecord` optics input. Default values: starting point to measure. |
| Crest spray from FFT crest energy (choppiness + height gate) | `water/ocean_spray.gd` | not in spec | low | M | Phase 4 eye candy only. |
| Lessons: a one-mega-compositor waterline path was abandoned; "do not tune visual offsets"; receiver-only waterline compositor is not production | `docs/systems/ocean/architecture.md` | our waterline split is a per-pixel test in the underwater post (Crest style) | low | - | Supports our choice; no action. |

## 2. Controller, moves, input (3.1)

| What it does | File(s) | Our counterpart | Value | Effort | Note |
|---|---|---|---|---|---|
| Priority-based move state machine, `check_relevance` returns next move; combat can force a move | `controller/move.gd`, `move_container.gd`, `input_package.gd` | `sf_action` + `sf_anim_graph` state machines, R11b decisions | low | - | Ours is the next-gen design already. |
| Edge cases: keep Y velocity across moves (jump while running); backward speed 70% when input >120 deg off facing; 0.1 s lockout after landing to stop physics-bounce re-jumps; playback speed scaled by velocity ratio against foot slide | `character_controller.md` | `sf_controller` kinematic mover; stride warping in `animation.md` | low | S | Re-jump lockout and backward-speed factor are cheap sanity tests for the kinematic mover (values to measure). |
| Combo / consecutive attack chaining | `controller/combo.gd`, `consecutive_attack.gd` | `action.md` (E1 combat) | low | M | Combat deferred (3.5). |

## 3. Animation (3.2-3.4)

| What it does | File(s) | Our counterpart | Value | Effort | Note |
|---|---|---|---|---|---|
| Text-key parser: `sound:`, `soundgen:` (left/right foot, swim left/right, moan, roar, scream, land), `hit`/`chop hit`/`slash hit`/`thrust hit`, start/stop/loop, equip/unequip; keys fire when crossed between frames | `animation/text_key_handler.gd` | `mw_anim/src/text_keys.rs`, `sf_audio/src/footsteps.rs` | low | S | We have it. Check only that `swimleft/swimright` soundgens route to swim splash sounds in 3.6. |
| 4 MW blend masks (lower body, torso from Spine1, L arm from L Clavicle, R arm from R Clavicle), 7 priority levels | `animation_blend_mask.gd`, `animation_priority.gd` | `animation-layers.md`, `sf_anim_graph` layers | low | - | Same masks as OpenMW; confirm our layer roots match (Spine1/Clavicle). |
| Pelvis offset applied after the AnimationTree, before foot IK; look-at as post-animation modifier; LOD tiers (FULL IK, HIGH, MEDIUM no IK, LOW single clip, CULLED) | `pelvis_offset_modifier.gd`, `look_at_modifier.gd`, `animation_lod_controller.gd` | foot placement, look-at chain, LOD in `animation.md` | low | - | Ours specifies the same order and more (full-body IK in `sf_anim/ik`). Confirms ordering rule: pelvis before foot IK. |
| Skeleton bone renaming Bip01 -> humanoid names; skip T-pose clips on load | `skeleton_utils.gd`, `animation_loader.gd` | `mw_anim/rig.rs`, `rig-profile.md` | low | - | Covered. Their note that MW bodyparts are partly skinned, partly rigid-attached and mirrored is the known quirk our smooth-bodies work addresses. |
| Retargeting Mixamo: attempted, "no success" | `EXPERIMENTS.md` | `sf_anim` retarget math, `animation-sources.md` | - | - | Negative result only; nothing to take. |

## 4. Ground deformation (snow, ash, mud, sand)

| What it does | File(s) | Our counterpart | Value | Effort | Note |
|---|---|---|---|---|---|
| RTT heightfield of footprint stamps around the player (stamp radius ~0.5 m), streamed per region with persistence, time-based recovery pass (once per second), per-material recovery multiplier | `deformation/deformation_*.gd`, `shaders/deformation_stamp.gdshader`, `deformation_recovery.gdshader` | **missing** (only `sf_precip_bevy` ripples; ROADMAP has SU1/SU2 snow/wetness, not footprints) | med | M | Candidate for Phase 4 (Solstheim snow, Red Mountain ash). Design is standard (stamp into a camera-centred height texture, recover with a decay pass); re-derive in WGSL/WESL rather than port. |
| MW terrain-texture-name -> deformation height class: "snow" deep, `snow_light` shallow, `snow_grass` and `snow_rock` excluded (names lie: they are grass/rock), `ash` (red vs generic), ice/lava/rock/marble/cobble/metal = 0 | `deformation/terrain_texture_name_parser.gd` | missing | **med** | S | The one real Morrowind data quirk here: texture names are misleading, so exclusion order matters (rock/ice/etc. first, then snow with exclusions, then ash). Reusable as a `mw_*` rule table feeding a per-texture "softness" field, also for footstep sounds and wetness. Heights (0.25/0.15/0.12 m): starting points to measure. |

## 5. Interaction: take / carry / drag (items, 2.1, 3.3)

| What it does | File(s) | Our counterpart | Value | Effort | Note |
|---|---|---|---|---|---|
| Tap vs hold on one button (0.20 s): tap = default verb, hold = grab; release after hold = drop/throw; modal UIs register as input gates so holding E to read a book does not grab it | `interaction/*`, `interaction_system.md` 3, 3.1, 8 | `sf_action`, `sf_ui` modal handling; E take in 2.1 | **med** | S | Check we gate on open UI and that carry mode suppresses doors/dialogue (rule 1 in their section 8). Hold-vs-tap prompt text "Tap E: Take / Hold E: Lift" is a nice UX idea. |
| Carry by velocity drive toward a point ahead of the camera, roll zeroed, gravity off + damping while held; release keeps velocity (free throw) | `carry_controller.gd` | soft 6-DOF joint to a kinematic hand body (`physics.md` "Carry and throw"; slice "steady carry") | low | - | Ours is more capable. Their saga is a useful negative list: render-rate writes to a physics body fight interpolation; wall-pushback that reads its own mutated target drifts. |
| Held body must not collide with the player (mask flip) and mass cap; grab/release symmetry contract; mutations at transitions deferred | `carry_controller.gd`, `interaction_system.md` 6.3, 6.5 | `Held`, layers in `physics.md` | low | S | Test idea: grab then release restores layers/damping exactly (round-trip test). |
| Carried item survives cell unload: orphan container, re-home on cell load, expiry (5 min or 8 cells), walk-back ground snap (ray 3 m down / 50 m up, snap if >0.25 m off) | `interaction_system.md` 10 | R0 ring unload "take them along" (`physics.md` line ~220); streaming | med | S | Check the dropped/thrown-item case: items left in an unloaded cell need persistence (`sf_save` placement overrides), and a ground-snap fallback for items that load into missing collision. Numbers: starting points. |
| Frame the pickup path: `store_item` returns OK / FULL / FORBIDDEN / INVALID, world body despawns only on OK | `inventory_service.gd`, `pickup_interactable.gd` | `mw_rpg_bevy/inventory.rs` (take with stack count, NAM9) | low | - | Ours is OpenMW-faithful. |
| Item collision shape by name pattern table (bottle/flask/bowl/cup/goblet = cylinder, else box) and mass = weight clamped [0.1, 200] | `collision-shapes.yaml` (root), `carryable_body_factory.gd` | OpenMW NIF collision (`physics-adapter.md`: RootCollisionNode, volume x 400 kg/m3) | low | - | Ours uses authored collision; the name table is a Godot-era shortcut. Skip. |
| Carryable set: WEAP, ARMO, BOOK, CLOT, MISC, INGR, ALCH, REPA, PROB, LOCK, APPA, SCRL, plus LIGH with can-carry flag | `morrowind/mw_carryable_registry.gd` | `physics-adapter.md` item classes | low | S | Cross-check our dynamic class list against this one (LIGH flag, SCRL, LOCK/PROB). |

## Top 5 to take

1. **Terrain-texture-name softness table** (snow/ash exclusion order) plus footprint stamp-and-recover pass: new ROADMAP item under Phase 4 (eye candy, with SU1/SU2); the table also feeds footsteps and wetness.
2. **Visibility-distance water optics** (`sigma = -ln(0.01)/d`, per-channel red bias) as the authoring input for the medium: ROADMAP 3.6 (underwater medium).
3. **Underwater world-anchored motes** (spawn snapped to a 4 m world grid, not camera-locked): ROADMAP 3.6 (cheap parallax cue).
4. **Water impact/wake emitter recipe** (surface-crossing detection, relative-velocity wake band, interval/distance emit): ROADMAP 3.6 hooks and 3.7 (thrown items, splash).
5. **Interaction edge rules**: modal gate + tap/hold split + carry-exclusive mode, carried/dropped items across cell unload with ground-snap fallback, grab/release round-trip test: ROADMAP 3.3 (physics follow-ups) and 2.1 item use.

Not worth taking: swim/buoyancy math, controller state machine, animation modifiers and LOD, spray, carry controller (all superseded by our specs). Out of scope for this half: terrain/streaming/UI.
