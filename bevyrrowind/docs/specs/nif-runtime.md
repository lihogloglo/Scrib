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

Moved to [history](archive/nif-runtime-history.md#actor-parts-as-built-2026-10-12-actorfx2).

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
