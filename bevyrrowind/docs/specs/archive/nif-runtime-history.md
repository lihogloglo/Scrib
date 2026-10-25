# NIF runtime history

Dated history blocks moved verbatim from [nif-runtime.md](../nif-runtime.md) (doc cleanup S10b, 2026-10-26).

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

