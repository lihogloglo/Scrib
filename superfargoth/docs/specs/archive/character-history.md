# Character: as-built history

Moved out of [character.md](../character.md) on 2026-10-26 (doc budget, S11). Append-only; grep it.

### Morph-bearing parts (M3-a, as built 2026-10-12)

Rule (gap G1, industry comparison below; cooked layout in bake.md "Morph
layout"):

- Morph targets carry a kind from the adapter (`sf_model::MorphKind`):
  **`Shape`** (per-character slider, never animates) or **`Expression`**
  (blink, mouth, visemes; the default).
- At assembly every `Shape` target is **baked into the vertices** at the
  recipe's weight (`CharacterRecipe::shape: ShapeWeights`, by target name;
  default weight when unnamed), as UE Mutable and FaceGen do. The merged-mesh
  cache key is therefore (skeleton, outfit key, LOD, `ShapeWeights::key()`);
  the key is 0 without sliders, so those characters share entries.
- A submesh that still carries `Expression` targets is **never merged**: it
  becomes its own mesh with only its own targets, on the outfit's shared
  palette (same joint entities; one extra draw and palette copy, accepted
  even when it shares the body's material). Every merged mesh carries **no
  morph data**.
- The cook drops `Expression` targets from LOD ≥ 2, so at A2/A3 distances a
  head merges like any other part.
- The adapter decides the kind (it knows which targets animate); the cook
  only obeys it. Morrowind: every NIF geometry morph is keyframed (head
  talk/blink, effects), so all are `Expression` and `shape` stays empty.

API (`sf_character::assemble`): `assemble(&[AssemblyPart { cooked, body_part }],
lod, &ShapeWeights) -> AssembledOutfit { joints, merged, morphed, has_colors }`;
rigid parts arrive already converted by `CookedSkinned::rigid`; the node
filter is `BodyPart::keeps_node` on each submesh's part path; the palette
is deduplicated by (joint name, inverse bind); `gpu_bytes()` estimates Bevy
memory (56 B/vertex + 16 with colors, 4 B/index, 48 B/vertex/target,
64 B/joint per mesh). Not done here: joints missing from the skeleton
(bound by name in the Bevy layer; "nearest mapped ancestor" needs the
part's joint hierarchy, which `CookedSkinned` doesn't keep), bounds.

**Memory per cached outfit, before → after** (tests
`sf_character tests::assembly::outfit_memory_before_and_after` and
`mw_actors tests/outfit_memory.rs`, LOD 0; "before" = v2 cook + plain
per-material merge, every model target dense over the merged mesh):

| Outfit | Before | After | Morph bytes before → after |
|---|---|---|---|
| Library example: 12k-vertex body + 1,968-vertex head, 30 targets (24 expression, 6 shape), one skin material | 21.22 MB | 3.38 MB (2 meshes) | 20.11 → 2.27 MB |
| Morrowind Fargoth (21 parts, 2,284 vertices, 57 joints) | 238,940 B | 238,940 B | 29,472 B both |
| Ajira / Caius Cosades / Hasphat Antabolis | 267,944 / 219,816 / 224,212 B | same | 32 / 26 / 26 KB both |

Morrowind gains nothing at LOD 0 (heads have their own texture, so they
were never merged with the body, and 2 targets that both move), and drops
the head morphs at LOD 2 (~26–32 KB each). Finding from the same run:
Morrowind outfits keep **11–15 distinct materials** after the merge, so a
merged NPC is still 12–16 skinned meshes, each with its own palette copy
(57 joints × 64 B × 12 ≈ 44 KB); collapsing them needs a texture atlas per
outfit (Mutable does this), not a better merge. Open question 7.

**Faces.** Morph-target clips (blink, mouth shapes) are curve tracks in
clips (animation.md). Lip sync starts as mouth-open driven by voice
loudness. Phoneme or viseme tracks come later. Eyes are look-at targets.

**Proportions.** Per-character root scale covers height and build.
Foot placement absorbs the leg-length differences. Playback-rate matching
uses the scaled clip speed, so a tall character takes longer strides,
not faster ones.

### As built (`sf_character_bevy`, charbevy 2026-10-12)

- **API.** Spawn a `Character` component (`Record { kind, id, state }` or
  `Recipe`); insert `Characters::new(Arc<dyn CharacterContent>)` and add
  `SfCharacterPlugin`. `CharacterContent` is the adapter hook: `source()`
  (recipes), `parts()`, `slots()`, `cooked(model)` (decoded `mesh.cook`
  product), `animation(recipe)` (`AnimAsset`), `material()` (cooked material
  → `StandardMaterial`), `placement_rotation()` (Morrowind: `FACING_TURN`).
  Feature `world`: `CharacterSpawner`, the `PlacementSpawner` for
  `PlacementClass::Actor` (near ring only, `DISABLED` skipped, no model
  wait). `spawn_character(commands, recipe, transform)` from "Interfaces"
  became the `Character` component (Bevy's required-components idiom).
- **Pipeline.** Steps 1–3 run on `AsyncComputeTaskPool` (recipe →
  `resolve_recipe` → key → stop if resident, else cooked parts →
  `CookedSkinned::rigid` for rigid parts → `assemble`). Bevy-free caches
  shared by tasks: parts by part id, animation assets by (skeleton, graph),
  the resident-key set. Main thread: upload (one `Mesh` per assembled mesh,
  `assembled_mesh_to_mesh` = the loader's attribute mapping; one shared
  inverse-bindpose asset; materials deduplicated by value), then the body
  child (race scale, `spawn_rig`, `Animator`, `AnimLod`) and one
  `SkinnedMesh` + `DynamicSkinnedMeshBounds` entity per mesh. Missing joints
  bind to the rig's root joint and are counted (`CharacterStats`).
- **Cache.** `OutfitCache`: key (skeleton, outfit key, LOD, shape key) →
  `Arc<OutfitAssets>`; characters hold the Arc; unused entries past
  `capacity` (256) are evicted. LOD 0 only so far.
- **Swap.** A changed `Character` rebuilds; the old meshes stay until the new
  outfit is uploaded and 2 frames have passed (`SETTLE_FRAMES`: Bevy may
  prepare a new mesh in a later render frame), then old despawn and new
  spawn in one command flush; the rig and animator stay on the same
  skeleton. Headless tests: two identical NPCs share one entry, 1 mesh per
  material (mirrored rigid part merged); swap keeps the rig and never has a
  frame without meshes.
- **Measured on Morrowind** (`bevyrrowind character`, M3 lineup): guard 18
  meshes (17 materials + head), Ajira 15, Huleeya 19, female dark elf 13,
  naked Fargoth 14, guar 16, cliff racer 6; 0 missing parts or joints; all
  seven dressed ~1.3 s after the window opened (dev build, cold caches,
  owner on the machine: not a benchmark). One mesh per material holds;
  the material count is the cost (open question 7: texture atlas per
  outfit, which is what Mutable does: "can merge the textures of different
  meshes if they share material", texture layouts).
- **Industry check** (2026-10-12, beyond the J9 table above): UE Mutable
  generates merged meshes/materials/textures in the background and then
  costs what a pre-made skeletal mesh costs; it also removes hidden parts
  and atlases textures per layout block ([overview](https://dev.epicgames.com/documentation/unreal-engine/mutable-overview-in-unreal-engine?lang=en-US),
  [texture layouts](https://dev.epicgames.com/documentation/unreal-engine/texture-layouts)).
  We match the background build + merged result; no texture atlas yet.
  RAGE peds (component drawables + texture variations, J9) and our
  per-material merge both keep one draw per material. Bevy 0.20 batches
  skinned meshes on storage-buffer platforms (skinloader finding), so
  identical cached outfits share mesh and material handles and batch.
- **Left:** distance LODs (LOD 2 drops head morphs), morph curves → head
  weights, held items at sockets, first-person view model, tints, leveled
  lists beyond the scaffold pick, texture atlas, crowd timings. (Node-
  animated creatures: done, next section.)

### Node-animated creatures (as built, nodecreatures 2026-10-12)

- **Technique: rigid skinning.** A body animated per node (no skin, a
  "segmented" model) and the rigid parts of mixed bodies are cooked as a
  skin whose rigid meshes carry one joint at full weight: their nearest
  transform node (`sf_model::cook_rig`, bake step `mesh.rig`). Mesh nodes
  are never joints (a node animated only by a morph, material or
  visibility controller follows its transform parent), because rigs are
  made of transform nodes. The result is an ordinary `Cooked::Skinned`, so
  the whole character path (per-material merge, outfit cache, swap,
  `spawn_rig`, the graph, walkers) applies unchanged and
  `CharacterContent` gains nothing: the adapter's `cooked(model)` hands
  over the `mesh.rig` product for creature bodies (Morrowind: every `CREA`
  model; `mesh.cook` stays what the world path uses).
- **Why not rigid mesh entities parented to joint entities.** Bevy 0.20
  `extract_skins` writes one `Mat4` per joint whose `GlobalTransform`
  changed into a storage buffer (double-buffered for motion vectors), and
  skinned meshes batch/MDI on storage-buffer platforms (skinloader). A
  parented part is its own render instance: its own visibility and
  frustum test, mesh-uniform extraction when it moves, and a separate mesh
  per part (no per-material merge, so no sharing across parts). A scrib
  as rigid skin is 8 meshes (= its materials) and 32 joint matrices; as
  parented parts it would be one instance per part. Transform propagation
  is the same either way (the joints exist for the animation). Rigid
  skinning also keeps one code path (cache, LOD, bounds, swap).
- **Industry.** UE's FBX import turns rigid animation into a skeletal mesh
  ("Import Rigid Animation") and by default converts meshes nested in a
  bone hierarchy into bones unless "Import Meshes in Bone Hierarchy" is set
  ([FBX import options](https://dev.epicgames.com/documentation/unreal-engine/fbx-import-options-reference-in-unreal-engine));
  sockets with static mesh components are the per-part alternative, used
  for things that change at runtime (weapons), which is where our held
  items go. OpenMW keeps the NIF scene graph and drives the keyframed nodes
  directly (keyframe controllers as node update callbacks, observed from
  its sources; `x<model>.kf` naming), i.e. the parented-node approach in a
  scene-graph engine.
- **Skeleton rule (adapter).** `mw_anim::rig::skeleton_source` also takes
  the root's transform children that hold unskinned geometry (storm
  atronach's orbiting `Rock_n`, keyed in its KF but outside `Bip01`).
- **Census** (`mw_bake` test `creature_rigs_real_install`, 89 unique `CREA`
  models, 396 records): 19 skinned, 63 mixed, 7 node-animated (scrib,
  kwama forager, shalk, bonelord, centurion sphere, heart of Akulakhan,
  1 more), 0 static. Before: 2 had no skin product at all (minescrib,
  heart_akulakhan: skipped), and 45 bound rigid parts to joints the
  animation skeleton lacked (wolf/bear/goblin heads, kwama antennae,
  jaws, centurion blades, storm atronach rocks), which the runtime bound to
  the root. Now: every model cooks; only heart_akulakhan keeps 6 parts on
  the root (its rays and meshes hang under billboard nodes, which are not
  rig joints). Headless test `node_animated_creature_binds_its_parts_to_joints_that_the_graph_moves`.
- **Not covered:** (morph curves: done, see "Morph playback") geometry-morph animation (the kwama forager's body,
  jaws) waits for morph curves; material/UV/visibility controllers on
  creature parts (flames, glows) are static; billboards and particles on
  creatures are not rendered by this path.

### Morph playback (as built, actorfx 2026-10-12)

**Industry check.** glTF and Bevy put morph weights in clips as curves
(`bevy_animation` animates `MorphWeights`; the Many Morph Targets stress test
plays them from an `AnimationClip`); UE drives morph targets from animation
curves. Bevy 0.20 stores targets in a texture (one layer per target) and every
vertex of the mesh loops over every target, so zero weights are not free: keep
targets on the small part that owns them (M3-a rule) and drop them at far LODs.
Faces: OpenMW (observation, GPL; game side only) keys heads on one timeline
bounded by `Talk: Start/Stop` and `Blink: Start/Stop` text keys and picks a time
from voice loudness (talking) or a blink timer (2-8 s pauses); legacy creature
bodies (kwama forager) key geometry-morph weights on the same timeline as their
`.kf` bones.

**Built** (`sf_character_bevy::morph`, game-neutral):
- `CharacterMorphs` on a dressed root: runtime targets by name (`OutfitMesh`
  now carries `morph_targets` names), defaults, current weights, bound meshes.
- Source 1, animation curves: a clip curve named like a target drives it; the
  graph already blends curves with the pose, so whole-body morphs follow
  idle/walk/attack and their blends. Bound once per `AnimAsset`.
- Source 2, `MorphCurvePlayer { tracks, time, weight }`: curves sampled at a time
  the game sets (faces), applied over source 1.
- `drive_character_morphs` (`PostUpdate`, after `AnimSystems::Apply`) writes each
  morph mesh's `MeshMorphWeights::Value` only when a weight changed. Facial LOD:
  morph meshes exist only on LOD 0-1 (assembly) and characters whose animator
  did not pose this frame (frozen/culled tiers) keep their last weights.
- Bake: `AnimImporter::companion` + `import_with` (sf_anim, default off): an
  importer may read a sibling file the step records as an input. `mw_anim`'s
  KF importer (v3) reads `x<model>.nif` next to `x<model>.kf` and cuts its
  geometry-morph curves (`"<node>#<target>"`, as `mw_bake` names cooked
  targets) into every group clip. Kwama forager: 9 of 9 targets bound.
- Game (`bevyrrowind/apps/bevyrrowind/src/faces.rs`): NPC head NIF → curves +
  talk/blink segments (cached per head); blink after a random 2-8 s pause;
  talking maps loudness ×2 (clamped) onto the talk segment and wins over
  blinking. Scaffold "talk key": `Y` toggles fake loudness on every NPC until
  dialogue/voice set `Talking`.
- Tests: `morph_curves_in_clips_animate_the_body_and_a_player_overrides_them`
  (sf_character_bevy), `blinks_fire_every_two_to_eight_seconds_and_talking_maps_loudness`
  (app), `mw_anim/tests/morph_real_install.rs` (forager clips, head text keys).
- **Not yet:** material/UV/visibility/alpha controllers on character parts;
  billboard joints (heart_akulakhan); morph-aware bounds.

### Held items at runtime (as built, heldrender 2026-10-12)

`sf_character_bevy::held`: each `CharacterRecipe::held` item becomes one **prop entity, a child
of its mount's socket joint** with `Transform::from_matrix(mount.offset)`, so Bevy's transform
propagation carries it with the animated joint (no per-frame copy). Under it: one `Mesh3d` +
material per cooked LOD0 submesh (`sf_model_bevy::submesh_to_mesh`), uploaded once per model
(`HeldModelCache`; materials share the outfit cache's map). The model comes through
`CharacterContent::held_model` (default `cooked`), loaded with the build on the task pool
(`Shared` caches it by model and light node).

- **Separate entity, not merged into the outfit** (the decision the M3 plan left open). The
  outfit is cached by look; held items change far more often (readiness, dusk/dawn torches,
  loot, combat), so merging would multiply outfit cache entries (look × loadout) and
  re-assemble per change, and the item would have to be skinned (one-joint) to follow its hand.
  A rigid prop costs one entity per submesh and no skinning; all props of one model share mesh
  and material handles, which Bevy 0.20's GPU preprocessing batches into the same multi-draw
  across characters, as it does for repeated static meshes. Physics, trails and lights hang off
  the prop later. Industry: UE skeletal-mesh sockets, Unity bone children, Godot `BoneAttachment3D`.
- **Readiness.** `CharacterReadiness(Readiness)` on the root (absent = sheathed). Each prop keeps
  both mounts resolved to joint entities; a change re-parents (only when the socket differs),
  sets the transform, or hides it (`Visibility::Hidden`, mount `None`). No rebuild.
- **Held-only changes.** A `Character` change whose outfit key and cached outfit are unchanged
  (a torch at dusk) keeps the outfit meshes; `sync_held` keeps props whose `HeldItem` is equal,
  despawns the rest, spawns the new ones.
- **Visibility.** Rig joints carry no visibility components; `sync_held` gives the chain from a
  socket up to the body `Visibility::Inherited`, so a hidden or culled character hides its props
  (Bevy B0004 otherwise).
- **Lights.** An item with a `HeldLight` gets a `HeldLightSlot { item, light }` child at
  `CharacterContent::node_position(model, attach_node)` (cooks flatten nodes; the adapter reads
  the source), else the bounds center. The host adds the light (calibration, animation, budget).
- **Stats.** `CharacterStats::{held, missing_held}`. Missing sockets warn once per name.
- **Morrowind glue (bevyrrowind `apps/bevyrrowind/src/held.rs`).** Darkness from the sky clock
  (`mw_actors::held::exterior_dark`: 19:00-6:00) re-asks `MwActors::held` for every actor on a
  change and for actors spawning in the dark (`CharacterState::held`; by day the recipe's own).
  Slots get the `LIGH` record's `LightCalibration::point_light` + `AnimatedLight` flicker (placed
  lights' path; `Off by default` ignored); shadows within the point-shadow budget (lighting
  2026-10-13: `sf_sky_bevy::BudgetedShadow`, torches rank 2x placed lights, the light's own prop is
  `NotShadowCaster`; sky-lighting.md "Local-light shadows"); `HeldLightBudget` (default 12) keeps the
  nearest lit torches to the camera on (no global light budget exists yet: placed lights rely on
  range cutoff; each clustered light costs every fragment its range covers). The `stance` graph
  parameter follows `Stance::of_weapon(readiness, main-hand WEAP type)` on body, readiness or
  held changes. `held_model` takes `mesh.cook`, else `mesh.rest` (the bake now adds `mesh.rest`
  keys for weapons and carryable lights: a torch's animated flame node).
- **Test:** `held_items_mount_on_their_socket_follow_it_and_move_with_readiness` (joint, follows
  within 1e-4 m over 30 frames, readiness toggle, hidden character hides props, held-only change
  keeps the outfit meshes, light slot at the content's node). App: `darkness_follows_the_clock`,
  `torch_slots_light_with_the_record_or_the_slot`.
- **Skinned props (actorparticles, 2026-10-12).** A `Cooked::Skinned` held model (bows and
  crossbows: the limbs are skinned to string bones) is a prop on **its own small skeleton**: one
  joint entity per palette name under the prop root, at its model-space rest transform
  (`CharacterContent::node_transforms`, default `node_position` as translations; a joint the
  content can't place keeps its bind pose `inverse_bind⁻¹`), and the submeshes as `SkinnedMesh`
  (`sf_model_bevy::skinned_submesh_to_mesh`, shared inverse bindposes, `DynamicSkinnedMeshBounds`).
  The skeleton follows the socket like a rigid prop. Static at rest for now; clips can later drive
  the joints by name (the string pull). Binding them to the character's hand joints was rejected:
  the bow's bones aren't in the character skeleton. Industry: UE attaches a skeletal-mesh
  component to a socket (its own skeleton), Unity a child `SkinnedMeshRenderer` with its own bones.
  `HeldModelAssets::skin` (`HeldSkinAssets`). Test: `skinned_prop_joints_are_one_per_name_and_the_palette_points_at_them`.
- **Carried-light near field (actorparticles; recalibrated by torchfix 2026-10-12).** The torch's
  `PointLight` is a child of the slot, kept `clearance_m` (0.7 m) horizontally off the carrier's
  vertical axis (pushed outward from the flame, last frame's transforms). Morrowind lights saturate
  inside a third of their radius; ours are inverse-square, and Bevy 0.20 has no per-light near clamp
  (its diffuse is `1 / max(d², 1 cm²)`; `PointLight::radius` only widens specular; Frostbite clamps
  `max(d², r²)`). At the placed lights' power the carrier was ~50x "fully lit" at 0.5 m and the
  ground under the torch ~8x. Now the carried light is dimmed until "fully lit" falls at a third of
  its radius (`HeldLightTuning::fully_lit_fraction`, from `LightCalibration::fully_lit_distance`):
  it matches the original at the edge of its clamp (the ground under the torch) and is brighter only
  inside it (carrier ~6x for white). Source radius 0.12 m (the flame) softens highlights. The light's
  own model takes no light: bevyrrowind maps it `mw_assets::self_lit` (texture at "fully lit", the
  original's saturated look), since it sits centimeters from the light (bevyrrowind `held.rs`, tests
  `carried_lights_keep_clear_of_the_carrier`, `carried_lights_read_fully_lit_where_the_original_saturates`).
- **Flames.** Particle systems of held props are bevyrrowind's (`actor_particles.rs`, nif-runtime.md
  "Particles on actors").
- **Not yet:** the bow string pull (needs clip curves on the prop joints), first-person props,
  interiors' darkness rule (needs the actor's cell), combat readiness, sheath mounts (vanilla shows none).
