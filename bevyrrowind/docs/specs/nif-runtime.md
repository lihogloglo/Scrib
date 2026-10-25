# NIF runtime: controllers, particles, billboards, lights

Status: implemented (2026-10-11)
Crates: mw_nif (data + evaluation, Bevy-free), mw_world (light records),
mw_assets (Bevy shell)

## Goal

Everything a static Morrowind model does on its own at runtime: animated
nodes, materials and textures, particle effects (flames, smoke, sparks),
camera-facing billboards, and the light a lamp or torch emits, positioned
and flickering. Non-goals (later milestones): skinned/actor animation and
`.kf` files (M3; skin and morph *data* import is built, see "Skins and
geometry morphs"), spell effects driven by
gameplay (M6), path controllers (5 vanilla files, all spell effects).

`bevyrrowind nif-sweep` measures coverage: all 7,319 vanilla NIFs
convert; remaining unsupported types are listed with example files.

## Canonical technique

- **Controllers → curves sampled each frame.** Every NIF key type is
  normalized at load: linear and Bezier keys are cubic Hermite (linear keys
  get segment-delta tangents, which interpolate exactly linearly), TCB keys
  get Kochanek–Bartels tangents, rotations slerp (quaternion keys) or
  compose three Euler curves. Controller time is `t * frequency + phase`
  wrapped by the cycle type (loop, ping-pong, clamp) into `[start, stop]`.
  Values are converted to engine space at load. Exact key conventions:
  "Keyframe conventions" below.
- **Who plays.** Controllers under `NiBSAnimationNode`/`NiBSParticleNode`
  play when the node's auto-play flag (0x20) is set, with a random start
  unless 0x40. Outside those nodes, material, visibility and particle
  controllers play; keyframes don't (they belong to actor skeletons). A
  non-skinned model with an `Idle` text-key group loops it.
- **Materials** are shared per model, so material controllers (alpha,
  material color, UV offset/tiling) animate the model's material assets
  once per frame on the global clock. Materials animated by different
  controllers are never deduplicated together.
- **Fixed-function shading mapped to PBR.** NIF shading is
  `texture * (emissive + ambient + diffuse * lights)`. The emissive color
  becomes Bevy emissive with the base texture as emissive texture, scaled
  to "fully lit" at the scene exposure (`mw_assets::emissive_nits`) and
  exposed like lit surfaces (`mw_assets::set_emissive`: Bevy 0.20's
  `emissive_exposure_weight` defaults to 0, which left emissive unexposed,
  ~150x white, until torchfix 2026-10-12). This is what makes flames glow:
  they are lit by their own light plus emissive.
- **Self-lit surfaces** (`mw_assets::self_lit`). A light model's own
  geometry around its light (a carried torch's head) is inside the light's
  saturation radius, where the original shows the plain texture; Bevy's
  point lights have no near clamp, so the light centimeters away blew it
  out to white. Such opaque surfaces map to their texture glowing at
  "fully lit" (the emissive rule with emissive 1), black base, no
  specular; blended ones (flames, smoke) keep their material. Applied to
  carried lights' props (`held.rs`); placed light models still take their
  own light (open: same rule by distance to `AttachLight`).
- **Particles: CPU simulation, per-instance dynamic mesh.** Box emitter
  (array emitters: random node of the emitter subtree, on its bone),
  emission window on the controller clock,
  declination/planar shoot angles, birth rate (or pool size / lifetime),
  planar and spherical gravity (×1.6, observed), grow/fade (size), color
  keys over normalized age, in-plane rotation; prewarmed to steady state.
  Particle size is a radius. Quads face the camera and are rebuilt each
  frame; alpha-blended systems sort back to front.
- **Billboards** align the node's Morrowind-space axes with view space
  (X right, Y up, Z toward the viewer): world rotation =
  `camera * Rx(90°)` in engine space.
- **Lights** spawn at the model root and move under the `AttachLight` node
  once the model instance exists. Flicker = two octaves of smooth value
  noise, pulse = cosine; per-light seeds desync them.
- **Root transforms are dropped.** The engine replaces a model root's
  transform with the reference transform (e.g. `in_nord_fireplace_01`
  carries a 180° root turn that never shows in game).

### Why our own particles (not bevy_hanabi)

Checked 2026-10-11: bevy_hanabi 0.19.0 (2026-06-27) is the newest release
and requires Bevy `^0.19`; we are on 0.20. Beyond the version gap, vanilla
emitters hold 4-70 particles each, the NIF semantics need to be exact
(spawn windows, pool-size rates, grow/fade, color keys), and a Bevy-free
simulation is unit-tested without a GPU. Revisit for dense exterior weather
(`blizzard.nif`, ash storms) if profiling shows CPU cost, or when hanabi
tracks our Bevy version: the `ParticleSystem` description maps onto its
modifiers.

## Data model

- `mw_nif::Model.controllers: Vec<anim::Controller>` (timing, auto-play,
  random start, target: node transform/visibility or material
  alpha/color/UV) and `Model.particles: Vec<particles::ParticleSystem>`.
- `mw_nif::particles::Simulation`: per-instance state, deterministic seed.
- Bevy: `NifAnimation` asset per model (labeled `Animation`), components
  `ModelAnimation` (root, keeps the asset alive), `NodeAnimation`,
  `ParticleEmitter`, `Billboard`, `AttachLight`; runtime-only
  `ParticleInstance`, `AnimationPhase`, `AnimatedLight`, `CarriedLight`.
- `mw_world::LightAnimation` (steady/flicker/pulse, slow variants).

## Interfaces

- `MwAssetsPlugin` registers the loader and all runtime systems.
- `mw_assets::lights::spawn_light(commands, calibration, info, model_root,
  transform, seed)`.
- `LightCalibration` resource: `lumens_per_radius_sq` (8000), `range_scale`
  (4.0), `ambient_nits` (300), chosen visually on the Census and Excise
  Office and Sud at `SCENE_EV100` = 7. `view --lumens/--range/--ambient`
  override them for further tuning.

## Verification

- `cargo test -p mw_nif -p mw_world -p mw_assets` (curves, timing,
  particles, light animation, billboard axes, root transforms).
- `cargo dev nif-sweep`: 0 failures; unsupported list.
- Screenshots: `cargo dev view "Seyda Neen, Census and Excise Office"
  --camera "-0.57,4.6,2.0,-0.57,2.6,6.74"` (fireplace flames),
  `--camera "3.85,4.25,3.1,3.35,4.05,2.43"` (candle), `cargo dev view
  "Sud" --camera "26.5,-5.0,13.0,31.5,-6.5,5.5"` (torches); two
  `--settle` values on `"Nchuleftingth, Test of Pattern"` (dynamo wheel
  turns) and `--model x/ex_vivec_waterfall_01.nif` (UV scroll); `--model
  r/heart_akulakhan.nif` from two sides (billboards).

## Skins and geometry morphs

Built 2026-10-12 (`mw_nif::skin`, `mw_bake` NIF importer v2). The runtime NIF
path still draws skinned shapes baked into their rest pose (`Mesh::positions`,
via `tes3`'s `apply_skin_deforms`); the skin and morph data ride along for the
bake, which cooks such models to `Cooked::Skinned` (`../superfargoth/docs/specs/bake.md`,
"Skinned and morphed models").

**How Morrowind skins work.** A `NiTriShape` links a `NiSkinInstance` (skeleton
root node, bone node list, `NiSkinData`). `NiSkinData` holds an overall
skin transform `S` and, per bone, a skin → bone transform `Dᵢ` (the inverse
bind), a bounding sphere and a sparse `(vertex, weight)` list. Gamebryo
deforms `v` as `world(G) · S · world(R)⁻¹ · Σ wᵢ · world(Bᵢ) · Dᵢ · v`;
`OpenMW`'s `RigGeometry` composes the same chain (`InvBind × BoneInSkeleton ×
SkinToSkel × mData->mTransform`, row vectors), skins on the CPU with any number
of influences per vertex and doesn't renormalize weights. `NiSkinPartition`
(a GPU-friendly split of the same weights) is ignored by both.

**Mapping to `sf_model`** (glTF form `Σ wᵢ · Jᵢ · IBᵢ · p`, `Jᵢ` the joint's
model-space rest transform):

| NIF | `mw_nif::skin` / `sf_model` |
|---|---|
| shape vertex data (before deforming), morph target 0 if it morphs | `Skin::positions`/`normals` → `MeshData::positions`/`normals` (bind space; node transform ignored) |
| `NiSkinInstance::bones` | `SkinBone::node` → `SkinJoint::node` (bones that aren't in the tree are dropped with their weights) |
| `Dᵢ`, `S`, shape and skeleton-root placement | `IBᵢ = Jᵢ⁻¹ · M · Jᵢ · Dᵢ`, `M = world(G) · S · world(R)⁻¹`, all conjugated to engine space (`coords::affine`) |
| per-bone weight lists | per-vertex `(bone, weight)` lists, as authored (the cook keeps the 4 largest and renormalizes) |
| `NiGeomMorpherController` + `NiMorphData` | `Mesh::morph`: target 0 is the base (as `OpenMW`'s `MorphGeometry`), targets 1.. become deltas (absolute targets minus target 0) with their weight curves and the controller timing; `MorphTarget` named `"<node>#<i>"`, `default_weight` = curve at the controller's start |

`M` is the identity whenever the shape and the skeleton root sit where they
were bound (the normal case), and then every pose is exact; otherwise only the
rest pose is (flagged `Skin::offset_bind`, counted by the report below). The
rest pose of the mapped skin equals the baked mesh (unit tests on synthetic
NIFs with bind ≠ rest and `S` ≠ identity; real-install test on body parts and
creatures).

**Bake rules.** A skinned model's root is marked animated (actors move) and so
is every node whose mesh morphs, so `model.info` keeps them out of the static
set the app instances; rigid meshes of a skinned model become single-joint
parts in the cook. A skinned shape whose skin didn't resolve keeps
`skinned` without data, and the cook skips the model (as before v2). The
weight curves stay in `mw_nif` (heads' talk/blink are driven by text keys,
as in `OpenMW`'s `HeadAnimationTime`; wiring is M3 actor work).

Whole install (`skin_report`, 7,319 NIFs): 556 skinned models / 3,069 skinned
meshes, all resolve, none with an offset bind; influences per vertex 1: 390k,
2: 73k, 3: 21k, 4: 3.6k, 5: 107 vertices (the cook drops the 5th); weights sum
to 1 within 2.4e-7; skin rest pose vs baked within 6.2e-5 m. 273 models / 324
meshes morph (843 targets); 37 morph-only models outside `meshes/b/` (magic
effects, bows' strings, werewolf, Sotha Sil) now keep the runtime path instead
of cooking statically. No `NiSkinPartition` in the install. 748 models cook to
`Cooked::Skinned` in 4.4 s of CPU (58.5 MB); 7 skip (skeleton-only files such
as `base_anim.nif`, a head whose morph has no target besides the base).

Verify: `cargo test -p mw_nif`, `cargo test -p mw_bake --test skins_real_install -- --nocapture`,
`cargo run --release -p mw_bake --example skin_report` (whole install).

## Keyframe conventions

`mw_nif::anim` serves both the runtime controllers (M1) and the KF/NIF
animation import (`mw_anim`, which divides each tangent by its segment's
duration for `sf_anim`). Verified 2026-10-12 (kftangents) against OpenMW
(`components/nif/nifkey.hpp`, GPL, fine here) and the base-game data; audit:
`cargo test -p mw_anim --release --test kf_curve_conventions -- --ignored --nocapture`.

| Key type | Stored | As built |
|---|---|---|
| 1 linear | value | Hermite with chord tangents (exactly linear); quaternions slerp |
| 2 quadratic (Bezier) | value, in tangent, out tangent (that order) | Hermite on each segment's `[0, 1]`: `a.out`, `b.in` used unscaled. Quaternion keys store no tangents and slerp |
| 3 TCB | value, tension, continuity, bias | Kochanek–Bartels tangents with the key-spacing adjustment: incoming × `2 dt_prev / (dt_prev + dt_next)`, outgoing × `2 dt_next / (dt_prev + dt_next)`; end keys reuse their one chord. Quaternion keys slerp (measured below) |
| 4 XYZ rotation | three float key groups + axis order | three float curves composed per sample |

Evidence for the quadratic convention (1,567 position channels): at keys
with uneven neighbor segments, 747 satisfy `in / dt_prev == out / dt_next`
(C1 under per-segment tangents) against 4 with `in == out` (per-second)
and 2 swapped; the median of `|out| / |velocity × dt_next|` is 1.00. The
large tangent-to-chord ratios the cook survey flagged (slaughterfish,
dreugh, weapons) are authored data: zero-length chords with non-flat
handles, and root-motion wrap snaps (a key jumping back hundreds of units
in one frame) feeding smooth neighbors.

TCB before 2026-10-12 lacked the spacing adjustment (base-game TCB channels
have a median 4.7× ratio between neighbor segments), so at 106 of 507
continuity-0 keys the velocity jumped by more than 2×; now it is
continuous, as Kochanek–Bartels and OpenMW (MR !4784, 2025) define it.

**Translation snaps are curve breaks (kfsnaps 2026-10-12, `mw_anim`
`convert::snap_breaks`, NIF importer v6, KF v5).** Creature KFs lay their
groups end to end, and the root jumps back to its start in one frame
between them (dreugh `Root Bone` 31 m from `Walkforward: Stop` to
`Attack1: Start`, slaughterfish 5.5 m and 11 m). TCB tangents, and the
exporter's auto-smooth Bezier handles, take that snap key as a neighbor, so
the smooth segment before it swings far past its keys: the dreugh's
`Walkforward` ending dives 3.8 m (an authored in-handle of +26 m on a
0.7 m segment), the slaughterfish's and durzog's 0.9 m. OpenMW plays the
same curves; the original engine never showed horizontal root travel, our
root-motion extraction does. A snap is a segment ≤ 0.1 s that moves
≥ 5 cm and ≥ 4× as fast as both neighbors (2,420 in the base game: 2,228
at or between group marks, 141 inside groups). The snap segment becomes
linear; a key bounding it takes its other segment's chord as tangent
(a curve end key, as if each side were exported alone) only when its
tangent differs from that chord by more than ¼ of the jump, so authored
handles that didn't see the snap keep their shape. Cutting a group never
interpolates across a snap at its ends either (`Start`/`Stop` keys bound
the cut), so loops wrap from their own end pose. Audit:
`cargo test -p mw_anim --release --test kf_snaps -- --ignored --nocapture`.
Cut clips with translation overshoot past a segment's keys: max 3.77 →
0.84 m, over 10 cm 44 → 33, over 1 cm 157 → 149; the rest are authored
zero-chord handles (netch, hircine spear, slaughterfish `Hit1`). Cook
survey (3,307 clips): 142 change, 33 lower error, 6 higher; over 0.1 mm
853 → 852 (4 newly under: slaughterfish, babelfish, dreugh `Walkforward`
1.5e-2 / 5.8e-3 → 3.6e-4 / 3.2e-4 m, hircine `Death1`; 3 newly over: bear
`Death3`, 6e-5 → 1.1e-4, a snap inside the clip now linear), p99 error
2.9e-3 → 2.4e-3 m, size 129.4 → 129.5 MB; 1 and 2 threads byte-identical.

**TCB quaternions: slerp, measured (kfsnaps 2026-10-12).** Gamebryo fits a
squad spline with Kochanek–Bartels quaternion tangents; OpenMW slerps key
to key and so do we. `kf_snaps` builds the squad (KB tangents in each
key's log space, spacing-adjusted like positions) for the 731 TCB rotation
channels of actor animation and compares: per segment the squad leaves the
slerp by p50 0.7°, p90 4.1°, p99 8.7°; per channel the worst segment is
p50 4.4°, p90 10.8°, p99 20°. Slerp's angular velocity jumps at keys by
p50 73°/s, p90 420°/s (keys p50 0.27 s apart). Through rotation snaps and
half turns (1,900 segments left out of those numbers) squad swings up to
90° off, so it would need rotation curve breaks first, and `sf_anim`
source rotations are linear, so squad would be sampled into keys (cook
size and time). Decision: keep slerp (OpenMW-compatible). The difference
is a few degrees with kinks at keys, not a clear visual defect; revisit
only if an owner A/B of creature idles shows the kinks (open questions).

## Baking

Static NIFs are cooked offline (`bake-adapter.md`). Everything this spec
covers (controllers, particles, billboards, switch nodes) marks a model
dynamic, and such models keep this runtime path. So do placements that
carry a light, which need the `AttachLight` node.

## Actor parts (as built 2026-10-12, actorfx2)

Controllers in an actor's own NIF key the same timeline as its animation
groups (`dremora.nif` keys visibility over 0-667 s), and play at the
current animation's time, not on a free clock: `OpenMW`'s
`Animation::addSingleAnimSource` assigns the whole-body animation time to
every object-root controller without a source. So `mw_anim::effects` turns
visibility and material controllers into clip curves next to the morph
weights (`hide:<node>` / `show:<node>`, `mat:<texture file>:<alpha|diffuse.r..|emissive.r..|uv.u|uv.v|tile.u|tile.v>`),
cut per group (NIF importer v5, KF importer v4 via the `x<model>.nif`
companion); the graph blends them and `bevyrrowind::actor_fx` writes them
after `AnimSystems::Apply`:

- visibility: a joint hides its subtree by scale 0 (Unreal's
  `HideBoneByName` does the same) while its `hide:` curve is above 0.5
  (a `show:` curve, for a node the file hides, at or below it);
  the next pose restores it. **Curves are rest-neutral** (dremora, 2026-10-12):
  the graph outputs 0 for a curve no playing clip carries, and biped
  creatures play `xbase_anim.kf` groups (Idle, walks, attacks), which carry
  none of their own curves; so 0 must be the rest state. With the earlier
  `vis:` curves (1 shown) every dremora idled with its 12 armor parts
  hidden (legs, upper arms, shoulders, braces: "no legs and biceps"). This
  is the Unity "write defaults" / UE default-curve-value rule: an
  unanimated property keeps its rest value. `mat:` curves stay absolute: no
  creature playing another file's clips has a material controller on
  rendered geometry (the census asserts it; make them deltas from the rest
  value if one ever does). Nodes hidden in the file carry no geometry
  (`to_model` drops them; their `show:` curves gate particles);
- materials: each character gets its own copy of the matching outfit
  material (`OutfitMesh::material_name`), since outfits share materials.

**Billboards in a rig.** `NiBillboardNode`s are skeleton joints
(`rig::skeleton_source`), so rigid parts under them bind to them; after the
pose `actor_fx` turns each (`rig::billboard_joints`) so its world rotation
is `billboard_rotation(camera)`, composing the rig's fresh local rotations
up to the body.

Census (`cargo test -p mw_bake --test actor_fx_real_install -- --nocapture`):
1,310 `BODY` models carry no controllers, billboards or particles; 89 `CREA`
models: 101 visibility controllers in 13 models (mostly particle emitters
and death pieces), 30 material controllers in 6 (alpha: ghosts, Dagoth Ur,
dremora, dwarven specter, heart; color: clannfear_daddy; UV: heart), one
billboard model (heart_akulakhan, 6), particles in 16. No creature model
carries a root transform, so the kept rigid-part root transforms
(feetfix) don't interact with these. Left: geometry hidden in the file and
shown only by a controller (dremora and ghost death dust) is dropped by the
bake (`to_model` skips culled nodes); visibility of skinned geometry merged
into an outfit mesh (dwarven specter robe, all shown in practice);
creature particles; controllers under `NiBSAnimationNode` follow the
animation time too (none seen on creatures).

Part census (`cargo test -p mw_bake --test creature_parts_real_install --
--nocapture`, dremora 2026-10-12): every triangle shape of the 89 `CREA`
models against the cooked rig (`mesh.rig` LOD0) and against what the idle
clip (from the files the bake layers) leaves shown. 331,292 source
triangles; 91,751 in 367 shapes hidden in the file (shadows, death dust,
editor pieces) are left out by design; 239,541 cooked, none lost; shown at
idle 239,541 (before the `hide:` fix: 237,519, the dremora's 12 rigid
armor parts, 2,022 triangles, on all 9 dremora records). Asserted: no
rendered shape lost by the rig cook, no part hidden throughout the idle, no
material curve absent from a playable clip. The heart of Akulakhan's rays
and glow are alpha 0 at idle by their own keys (they flash in one window).
Still missing (printed by the census, not asserted): 14 shapes (1,820
triangles) hidden in the file that a visibility controller shows are
dropped by `to_model`: heart_akulakhan's frost / magica / shock shields
(its `Idle2`-`Idle4`), fabricant_imperfect's six `Falloff` pieces,
ascendedsleeper's `Skull`, and the death-dust spheres of dremora,
ancestorghost (2) and dwarvenspecter. Keeping them needs a "hidden at rest"
flag on `sf_model::Node` so `cook_rest` (far meshes) leaves them out while
`cook_rig` keeps them; the `show:` curves already hide them at rest.

## Particles on actors (as built 2026-10-12, actorparticles)

Held torches' flames and the emitters inside creature models (16 `CREA`
files) use the static path's CPU simulation; only the anchoring is new.

- **Source.** Cooks drop particle nodes, so the app
  (`apps/bevyrrowind/src/actor_particles.rs`) reads the NIF itself: the
  `Animation` sub-asset (`<model>#Animation`, the data a placed fireplace
  uses) and, per particle node, `particle_anchors`: the nearest ancestor that
  is a skeleton joint (creatures; `SkeletonBinding::find`) or the model root
  (held props), the node's rest transform in that space, and the names of
  the nodes in between.
- **Placement.** One `ParticleEmitter` entity per particle node, a child of
  that joint (creatures, on each new body) or of the prop root (held items,
  on new props), so it follows the pose through transform propagation (UE
  attaches Niagara components to sockets the same way). Markers:
  `ActorEmitter` (every system plays: actor controllers run on the
  animation time, so the auto-play flag doesn't apply) and
  `ParticleRange(60 m)` (hidden and paused beyond, since no visibility
  range covers moving actors).
- **World space.** `NiBSParticleNode` flag 0x80 means local space, so
  particles move with the node (`ParticleSystem::local_space`). Without it,
  the default and what OpenMW does (`ABSOLUTE_RF` reference frame),
  particles stay where they were emitted:
  each step maps the live particles by `now⁻¹ · before` of the node's world
  transform (`Simulation::carry`), so a swung torch trails its flame. Gravity
  stays in node space, as in OpenMW (its gravity affector turns the direction
  with the node). A step longer than 4 m (teleport, floating-origin shift)
  doesn't carry. A degenerate node (a joint `actor_fx` hid by scale zero)
  pauses the system.
- **Gating.** `hide:<node>` / `show:<node>` clip curves on nodes between the joint and the
  particle node show/hide the emitter (`Visibility`); a curve on a joint
  hides it with the joint (scale zero). Particle nodes hidden in the file
  without a curve are skipped. Emission: each creature system emits while
  its `emit:<particle node>` clip curve is at least 0.5 (`ActorEmitter`, see
  "Emission clocks" below); a held prop's systems run on their own
  controller clock, as on a placed model.
- **Budget.** Off view and beyond the range, emitters pause (streaming.md
  "Spikes": every simulated emitter re-uploads its mesh each frame).
- **Not yet:** first-person props.

## Array emitters, live emitters, emission clocks (as built 2026-10-12, particles2)

Census: `cargo test -p mw_bake --test particles_real_install -- --nocapture`
(all 7,319 vanilla NIFs): 678 particle controllers, 42 of them
`NiBSPArrayController` (21 creature models and their `x` variants: atronachs,
Dagoth Ur, dremora, ascended sleepers, bonewalker, dwarven specter), every one
with flag 0x8 and none with 0x10; every controller has an emission window,
257 of them shorter than their controller range; 209 emitters sit under a
keyframed node (mostly spell effects in `e/`).

- **Array emitters.** A `NiBSPArrayController` is a particle controller whose
  emitter is a subtree, not a box. Format facts (NifTools; OpenMW as a black
  box, `nifosg` loader and particle emitter): flag 0x10 emits at the vertices of the
  emitter's first geometry; otherwise each particle picks a random `NiNode` of
  the emitter's subtree (the emitter included) and the box is placed in that
  node's space; with 0x8 the particle starts at a random point between that
  node's parent and the node (on the bone). So vanilla array emitters burn all
  over a skeleton (`Bip01 Pelvis` and its 50 nodes on the flame atronach),
  never at vertices: the "emit from vertices" item was a misreading. We
  implement nodes and bones (`EmitFrom::Nodes`, `ParticleSystem::targets`
  with rest transforms relative to the particle node); 0x10 falls back to the
  nodes (no vanilla file; add it when a mod needs it).
- **Live emitters.** A host passes the live emitter transform and live
  targets to `Simulation::step` (`mw_assets::LiveEmitter`: per system, the
  entities and offsets of the emitter, each target and its parent). Actors:
  `actor_particles.rs` anchors the emitter and every target at its nearest
  skeleton joint (`LiveAnchor`). Placed models: `mw_nif` marks systems whose
  emitter or targets hang below a keyframed node that isn't above the particle
  node (`ParticleSystem::animated_emitter`); the loader tags those nodes
  `NifNode(index)` and `start_particles` follows their entities.
- **Emission clocks.** `ParticleSystem::emits_at(t)`: the controller time
  `key_time(t)` inside `[emit_start, emit_stop)` (OpenMW: start inclusive,
  stop exclusive). Placed models run it on the model's controller clock: the
  phase is the one `assign_phases` gives the model's keyframed nodes (random
  per instance root unless flag 0x40), so a window lines up with the motion;
  prewarm runs up to the current clock. Actors (particles3, 2026-10-12): the
  window keys the creature's animation timeline (OpenMW assigns the
  whole-body animation time to the model's controllers, as for visibility
  curves). `mw_anim::effects` turns each particle controller into an
  `emit:<particle node>` step curve on the file timeline (1 while
  `emits_at(t)`, edges found per cycle period, so a short looping
  controller repeats; `emission_keys`), cut into every group's clip next to
  `vis:` (importer versions: NIF anim 7, KF anim 6). Exporters name every
  array node `PArray`, so further systems on a same-named node get `#1`,
  `#2`... (`emission_names`: Dagoth Ur has eight). `actor_particles.rs` maps
  each system to its curve and writes `ActorEmitter::emitting` after the
  pose; `simulate_particles` uses it, and systems without a curve (held
  props) fall back to their own clock. The graph blends the curves like any
  float curve, so a crossfade switches emission at 0.5. One deliberate
  deviation: a window covering its controller range within 5 %
  (`emits_throughout`) is a constant 1. The flame atronach's body flames
  stop 1 s before the end of its 28.7 s timeline, which falls in `Idle3`:
  read literally, its body would go dark for a second on every `Idle3`
  loop; we take that as an export artifact. Census
  (`particles_real_install`, test `actor_emission_on_clip_timelines`): every
  creature system emits in at least one clip. Examples: fire atronach body
  in all clips, its hands (`Weapon`) in `Attack1`-`3`; Dagoth Ur's hands
  in `Idle2` and `SpellCast`, his body in his deaths; dremora dust in
  `Death1`-`5` (1.67 of 2.67 s); ghost sprays in its deaths; centurion steam
  in walk, idle and death moments. Industry: Niagara spawn driven by anim
  notify states, Unity animation events; ours is the same idea as data
  (a curve per emitter), as the original stores it.

## Industry comparison

J9 (superfargoth `industry-audit.md`), 2026-10-12, docs only (`.sessions/j9rest.md`).
"Unverified" marks claims from memory not re-read this session.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Keyframed controllers as sampled curves | universal (Gamebryo/NetImmerse controllers, OpenMW's controller port, UE/Unity curves) | Hermite/TCB/slerp normalized at load | **match** (OpenMW semantics) |
| Small emitters on the CPU | UE Niagara CPU sim recommended for low counts; Unity Shuriken (CPU) next to VFX Graph (GPU); OpenMW CPU (OSG port of NiParticleSystem) | CPU, Bevy-free, per-instance mesh | **match**; GPU sim only for dense weather (weather/precipitation-surface.md precipitation is closed-form anyway) |
| **Soft particles** (fade by scene depth near intersections) | UE "Depth Fade", Unity soft particles, OpenMW soft particles (shader setting, 0.47+, unverified version) | none: flames and smoke cut hard lines into floors and walls | **GAP (P2)**: needs the scene depth (depth prepass, blocked by the dither pitfall until audit job J5). Open question below |
| Transparents under DLSS/TAA | reactive/transparency masks (superfargoth weather.md open question 9) | none | shared gap, tracked there |
| Fixed-function → PBR mapping | remasters (Oblivion Remastered on UE5 re-authors materials; RTX Remix replaces them, unverified detail); OpenMW keeps fixed-function lighting | emissive scaled to exposure, PBR lit | **differ**: physically lit by design (sky-lighting.md); PBR texture generation is M7 |
| Light flicker/pulse | UE light functions / curves; OpenMW flicker and pulse from light flags | noise flicker, cosine pulse | **match** |

Sources: UE Niagara and depth-fade docs, Unity soft particles docs, OpenMW
shader settings docs: not re-read this session (unverified detail).

## Open questions

- ~~Particles in world space~~: done (actorparticles, "Particles on actors").
- ~~`NiBSPArrayController` should emit from vertices~~: vanilla array
  emitters emit from the nodes under the emitter (0x10 unused); done
  (particles2, "Array emitters").
- ~~Animated emitter nodes use the rest-pose emitter transform~~: done (live
  emitters, actors and placed models).
- ~~Actor emission windows on the animation timeline~~: done (particles3,
  `emit:` clip curves, "Emission clocks"; `ACTOR_EMISSION_SCAFFOLD` removed).
- Is the 1 s gap at the end of the flame atronach's body window really an
  export artifact? We emit throughout (`emits_throughout`); an OpenMW
  side-by-side of its `Idle3` would settle it.
- Spell-effect NIFs (`e/`, 209 emitters under keyframed nodes) are not on
  any actor timeline: they wait for gameplay (magic effects, M5+).
- Flicker/pulse rates are tuned by eye, not measured from the original.
- TCB quaternion squad as an owner A/B (see "TCB quaternions: slerp,
  measured" above): only if a side-by-side of creature idles shows the
  slerp kinks; needs rotation snap breaks first.
- Soft particles (J9, P2): fade particle alpha by `saturate((scene_depth −
  particle_depth) / fade_m)` (UE Depth Fade, Unity soft particles, OpenMW's
  soft-particles setting). Needs the scene depth: after superfargoth
  industry-audit job J5 (depth prepass dither fix), bind the prepass depth in
  the particle material; until then flames and smoke keep hard intersections.
