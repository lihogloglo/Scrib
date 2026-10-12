# Characters: assembly, equipment, standard rig, retargeting, views
Segmented legacy bodies and rigid armor bending apart at joints: see   smooth-bodies.md (seam weld + weight blending per outfit, on by default; smoothbody 2026-10-12).

Status: draft (2026-10-11); `sf_character` core built (2026-10-11, see "As implemented")
Crates: `sf_character` (new, Bevy-free: humanoid rig schema, rig maps,
slot resolution, recipes, retarget poses), `sf_character_bevy` (new:
spawning, part merging, attachments, faces, first and third person),
`sf_model` (extended: skin data in `Model` and `.sfmesh`), `sf_anim`
(retarget math, animation.md)

## Goal

Build a character from parts (body, head, hair, equipment, held items)
onto one skeleton, cheaply enough for crowds. Let any animation set made
for a standard humanoid play on any humanoid skeleton, including legacy
rigs with their own names and rest poses. Support first-person and
third-person views of the same character. Adapters say which parts a
character wears and how their rigs map. The framework decides how the
character is built, drawn and animated.

Non-goals for M3: character creation UI and body sliders, cloth on robes
and capes (M4), physics-driven hair, skin and hair shading models (a
rendering spec), runtime retargeting between unrelated morphologies.
Creatures play their own clips by default; creature rigs that share a
template and limb names may retarget at bake through the same
`anim.retarget` (chains by name). Every skeleton is described by a rig
profile ([rig-profile.md](rig-profile.md)); the humanoid map is an
optional layer that produces one.

## Canonical technique

- **Modular characters on a shared skeleton, merged at spawn.** Parts are
  skinned to one skeleton by joint name. Rigid parts (a pauldron bolted to
  the upper arm) become single-joint skinned meshes at bake. The visible
  parts are concatenated into **one skinned mesh per material** when the
  outfit changes (UE's skeletal mesh merging and Mutable; the common
  technique for modular NPCs). This matters in Bevy in particular: every
  `SkinnedMesh` uploads its own copy of the joint palette, so 20 parts on a
  60-joint skeleton would upload 1,200 matrices per NPC instead of 60.
- **Slot-based equipment with coverage**, as in the Creation Engine's biped
  slots: an item occupies some slots and hides the base parts in them.
  Resolution is deterministic by layer priority.
- **A standard humanoid rig as the retargeting hub.** The bone set is VRM
  1.0's humanoid (55 bones, 15 required: hips, spine, head, upper/lower
  leg, foot, upper/lower arm, hand; open spec, vrm-c/vrm-specification),
  which matches Unity's Humanoid avatar. The canonical reference pose is a
  T-pose, Y-up, facing +Z (glTF and VRM convention), in meters. Each
  skeleton has a **rig map** (bone → humanoid bone, chains, retarget
  pose). Sources and targets map once to the hub, so N rigs need N maps,
  not N².
- **Chain-based retargeting** as in UE5's IK Retargeter: align the
  retarget poses, copy rotations as global deltas per chain, scale root
  translation by the hip-height ratio, and optionally fix end effectors
  with IK (feet planted, hands on a shared prop). It is done **at bake** per
  (clip, target skeleton), so runtime cost is zero. A runtime path exists
  only as a fallback when a product is missing.
- **First person: separate view-model arms plus a shadow-casting body**,
  the approach of Skyrim and Starfield: arms rig, own clips, narrow FOV.
  "True first person" (the full body with the head hidden, camera at the
  head) is a framework option for content that is animated for it.

**Reuse check (2026-10-11).** No Rust crate does modular character
assembly or skeleton retargeting (crates.io "retarget" finds only
unrelated hits). `bevy_animation_graph` and the IK crates have no
retargeting. The UE Retargeter and Unity Humanoid are the reference
designs. The CC0 Quaternius Universal Animation Library (glTF, 2.0 from
2026-01, 8-direction locomotion on a standard humanoid) is the framework's
retarget test set.
VRM crates (`serde_vrm`, `vrm-spec`, `vrm-runtime`) model the VRM file
extension (string-keyed humanoid maps of files that already carry one) and
`bevy_vrm`/`bevy_vrm1` are Bevy-bound loaders: none auto-maps an arbitrary
skeleton, so `sf_character` has its own dense 55-variant enum with parent,
side and mirror tables.

## Data model

```rust
pub enum HumanoidBone { Hips, Spine, Chest, UpperChest, Neck, Head, LeftEye, /* … VRM 1.0 set … */ RightLittleDistal }

/// Per skeleton; an adapter provides it, or it is auto-detected by name
/// heuristics (fuzzy matching as UE and Unity do) and saved for review.
pub struct RigMap {
    pub bones: Vec<(JointName, HumanoidBone)>,
    pub root_motion: JointName,           // the joint whose motion becomes root motion
    pub retarget_pose: Vec<(JointName, Quat)>, // local rotations that bring the rest pose to the T-pose
    pub mirror: Vec<(JointName, JointName)>,   // left/right pairs (Mirror node)
    pub extra_chains: Vec<Chain>,         // tails, wings, skirt joints: matched by name to another rig, or left alone
}
```

- **Retarget pose.** Rest poses differ (relaxed arms, A-pose). An
  auto-align fills `retarget_pose`: it rotates each limb chain so upper
  arms point ±X, legs point −Y, and the spine is vertical, as UE's "Auto
  Align" does. Adapters can override it.
- **Body slots** are a registry, not an enum: content registers slot names
  (`"torso"`, `"left_forearm"`…), like record kinds. A default humanoid slot
  set ships for native content. Each slot names the humanoid region it
  covers, used for hiding and for cloth later.
- **Parts and items** (records, native kinds registered by `sf_character`;
  adapters convert their own records into them at load, mods patch them):

```rust
pub struct BodyPart  { pub model: VfsPath, pub slot: SlotId, pub kind: Skinned | Rigid { joint: JointName } }
pub struct Wearable  { pub parts: Vec<(SlotId, PartRef)>, pub hides: Vec<SlotId>, pub layer: Layer, // Layer(u8): SKIN 0 < UNDER 50 < CLOTHING 100 < ARMOR 150 < OUTER 200
                       pub first_person: Vec<(SlotId, PartRef)> }                                // view-model parts (gauntlets)
pub enum RigRef { Humanoid(RigMapRef), Profile(RigProfileRef) } // creatures need no humanoid map
pub struct CharacterRecipe {
    pub skeleton: VfsPath,
    pub rig: RigRef,
    pub base: Vec<PartRef>,               // skin parts: from species/race and gender
    pub head: Option<PartRef>, pub hair: Option<PartRef>, // a creature may have neither
    pub worn: Vec<WearableRef>,
    pub held: Vec<HeldItem>,              // { item, model, hand, drawn/sheathed: Option<Mount>, light }: see "Held items"
    pub scale: Vec3,                      // root scale; non-uniform allowed
    pub graph: AnimGraphRef,
    pub tint: TintParams,                 // skin, hair: material parameters
    pub first_person: Option<FirstPersonRecipe>,
}
```

**Slot resolution** (pure, `sf_character`): start from the base parts
(skin, then head, then hair). For each worn item by ascending layer (ties:
the later equip wins), first clear the slots it `hides`, then place its
parts, replacing what is in their slots. An item thus hides what lies
*under* it (a robe hides skin legs and trousers), and a higher layer can
still show in a slot a lower one hid. The result is the visible part list,
plus a hash: the **outfit key** (blake3 of the (slot *name*, part id) pairs
in slot-name order, so it is the same for the same look however it was
reached, and stable across sessions; slot ids are session-local).
Whether two items that share slots may be worn together (biped-slot games
unequip the other) is a gameplay rule (`Wearable::overlaps` helps), not
part of resolution. The first-person view resolves the same way from the
`FirstPersonRecipe` parts and the items' `first_person` parts.

**As implemented (`sf_character`, 2026-10-11).**
- **Humanoid**: `HumanoidBone` (VRM 1.0 names, dense index, parent,
  required, side, mirror, finger/limb helpers), `BoneSet` (u64 mask, used
  for slot regions).
- **Auto-map** (`auto_map(&Skeleton) -> (RigMap, AutoMapReport)`): names
  only *classify* joints (hand, foot, finger, toe, helper, end…) and anchor
  the search; the **hierarchy decides the bone**. UE's chain auto-map is a
  fuzzy string match and Unity combines keyword patterns with hierarchy
  checks; pure name matching fails across conventions (Mixamo's `LeftLeg`
  is a lower leg, `LeftArm` an upper arm; 3ds Max hangs thighs off the
  spine and clavicles off the neck). Steps: hips (unsided hips/pelvis with
  the largest subtree; only joints below it count, so `ik_*` helpers under
  a root never compete), neck/head by name, spine = path hips→neck (1:
  spine; 2: + chest; ≥3: spine, middle chest, top upperChest), hands (the
  sided hand with most fingers) and feet by name, limbs by walking from the
  hand/foot up to the torso skipping helpers (names pick upper/lower when
  present, else position), fingers grouped by name or 3ds-Max
  `Finger<f><segment>`, segments by depth (non-thumb metacarpals and end
  sites skipped), toes, eyes, jaw. Root motion: nearest ancestor named
  root, else the hips (adapters override). Mirror pairs by side-neutral
  name. Leftover linear chains (tails, a digitigrade extra joint) become
  `extra_chains` named by their words (`tail`). The report lists missing
  bones, unmapped joints, guesses (sides from position) and
  `RigMap::validate` errors (unknown/duplicate joints, missing required,
  VRM hierarchy as *ancestor* relations, upperChest without chest).
- **Auto-align** (`RigMap::auto_align`): against fixed canonical
  directions, not another rig (N maps). (1) Facing: left (left − right
  limbs) and up (hips → head) snapped to axes; if not +X/+Y, the topmost
  joint above the hips is turned in the retarget pose (a rig facing −Z or
  Z up still lines up). **Retargeted clips therefore face +Z** (the
  framework convention, locomotion.md); an adapter whose native clips face
  elsewhere turns its skeleton at import. (2) Swing-only, parents first:
  hips/spine/neck → +Y, upper/lower legs → −Y, feet keep their rest
  direction to the toes (flat), upper/lower arms and hands → ±X,
  non-thumb finger segments → ±X; shoulders, thumbs, head keep theirs.
  Twist is left alone. Measured on the Quaternius UAL1 rig against its own
  `A_TPose` clip: arms within 1°, legs 0.4–4.6°, spine 7° (we straighten
  the spine, Quaternius keeps its curve; both rigs of a pair are aligned
  the same way, so deltas stay consistent; adapters may override).
- **`RigMap::retarget_rig`** builds `sf_anim`'s `RetargetRig`: chains
  `hips`, `spine` (every joint from spine to the highest torso bone, unmapped
  ones included, so 5 and 3 spine joints pair by arc length), `neck` (neck
  to the head's parent), `head`, `jaw`, `<side>_eye|shoulder|toes`,
  `<side>_arm` (upper, lower, hand), `<side>_leg` (upper, lower, foot; IK
  with `RetargetOptions::ik_legs`), `<side>_<finger>`, plus extra chains.
- **Rig profiles and digitigrade legs (2026-10-12, rig-profile.md's
  proposed changes, applied).** `RigMap::profile(&Skeleton)` builds an
  `sf_anim::RigProfile`: template `Biped`; support limbs
  `leftLeg`/`rightLeg` = the joint path from upper leg to foot (four
  joints with a beast-race metatarsal: `TwoBoneAnkle`, kind
  `Digitigrade`), toes as the foot's toe; `leftArm`/`rightArm` from the
  shoulder when mapped (passive) to the hand; spine, neck, head, eyes
  from the humanoid bones; extra chains named `tail…` become tails, the
  rest extra. `retarget_rig`'s `<side>_leg` chain is the same full path
  with the solver set, so the IK fix-up keeps a digitigrade target's foot
  contacts (test: Bip01 → beast Bip01 with a `HorseLink`, feet within
  1 cm). `CharacterRecipe::rig` is a `RigRef`; creature recipes name a
  `rig.profile` subject.
- **Slots**: `SlotRegistry` (register by name with a `BoneSet` region;
  re-registering extends the region; `with_humanoid_defaults`: head, hair,
  face, neck, chest, abdomen, pelvis, back, tail, and per side shoulder,
  upper arm, forearm, hand, upper leg, lower leg, foot). `SlotId`s are
  registration indices, never saved.
- **Records**: `sf.body_part` (`BodyPart`), `sf.wearable` (`Wearable`),
  `register_kinds`; `Layer` is an open `u8` scale with named bands (a
  closed enum had no room for content between bands); `resolve` (pure) and
  `resolve_recipe` (looks records up through `PartLookup`, implemented for
  `RecordDb`).
- **Part selection and socket offsets (2026-10-12, first adapter use).**
  `BodyPart::node_filter: Option<String>`: only geometry under a node whose
  name (or an ancestor's) contains the text, ASCII case-insensitive, belongs
  to the part (`BodyPart::keeps_node`), because one model file may hold
  several parts (both sides of a limb, a head and its hair), as one glTF file
  holds several named meshes. `None` is the whole model.
  `PartKind::Rigid { joint, offset }`: `offset` is the model's transform
  relative to the joint, a socket offset as engine sockets carry
  (`PartKind::rigid(joint)` = identity). A negative determinant mirrors the
  part so one model serves both sides; `CookedSkinned::rigid` flips the triangle winding
  then. Adapters may also implement `PartLookup` themselves and build part
  records on first use instead of defining them all at load. The cook keeps
  what both need (per-part node paths; `CookedSkinned::rigid` for the offset, see below).
- **Held items (as built, helditems 2026-10-12, data only).** `HeldItem { item: RecordId, model, hand:
  Hand (Main | Off), drawn: Option<Mount>, sheathed: Option<Mount>, light: Option<HeldLight> }`,
  `Mount { socket, offset: Affine3A }` (a socket joint plus the model's transform relative to it, as UE
  skeletal-mesh sockets and Godot's `BoneAttachment3D` carry one; negative determinant mirrors),
  `HeldItem::mount(Readiness) -> Option<&Mount>` (`None`: hidden at that readiness; games without
  visible sheaths leave `sheathed` empty, a shield next to a drawn two-hander leaves `drawn` empty),
  `HeldLight { attach_node, color (sRGB), radius (m) }` for torches and lanterns. `Readiness`
  (`Sheathed` | `Drawn`) is runtime state the host keeps per character, not part of the recipe or
  `CharacterState` (toggling it re-mounts props without resolving anything). Held items are never merged
  into the outfit, so they are not in the outfit key; a `CharacterState::held` change reuses the cached
  outfit. Adapters fill `CharacterRecipe::held` (Morrowind: actors-adapter.md "Held items as built").
  Heavy side (open): spawn each item as a child of its mount's joint entity (static rigid mesh, offset as
  its local transform), move or hide it on readiness changes, spawn the `HeldLight` under the
  `attach_node` entity of the item's model, attach to the first-person skeleton's same-named joints in
  that view. `CharacterState` lost `Eq` (held items carry transforms).
- Tests: `cargo test -p sf_character` (18 unit + 2 integration). Test rigs:
  `crates/sf_character/tests/data/` holds the node hierarchies (no meshes,
  a few KB each) of two CC0 Quaternius rigs, the Universal rig (UE-style
  names, 65 joints) and the UAL1 rig (Rigify `DEF-` names), plus UAL1's
  `A_TPose` rotations (sources in its LICENSE.md). Synthetic rigs with
  scrambled joint axes cover 3ds Max Biped (Bip01), Mixamo, UE5 (five spine
  joints, twist joints, metacarpals, IK helpers) and VRM naming.

**Skin data in `sf_model`** (as built 2026-10-12; details in bake.md,
"Skinned and morphed models"): `MeshData::skin` (palette of node +
inverse bind, per-vertex influence lists) and `MeshData::morph_targets`.
`mesh.cook` cooks skinned and morphed models to `Cooked::Skinned`: one
submesh per source mesh with its node path (so `node_filter` can pick parts
before the per-material merge), one model-wide palette by joint name +
inverse bind, `SkinVertex { joints: [u16; 4], weights: [f32; 4] }` (the
four largest, renormalized; `f32` rather than `unorm8` so the Bevy loader
maps it straight to `ATTRIBUTE_JOINT_WEIGHT`, and meshopt's codec packs the
mostly 0/1 weights small), LODs simplified in bind space with a joint-center
attribute, and vertices welded only when every stream matches. Rigid meshes
inside a skinned model become single-joint skins at cook. Rigid *parts*
(`PartKind::Rigid`) become single-joint skins at assembly through
`CookedSkinned::rigid(&cooked, joint, offset)`, which also flips the
winding for a mirroring offset (the renderer never sees a negative scale).
Still to do here: `node_filter` matching on `CookedPart::node_path` in the
assembly (a `BodyPart::keeps_node(path)` call per part).

## Assembly at runtime (`sf_character_bevy`)

1. Resolve the recipe, then the outfit key. A hit in the **merged-mesh
   cache** (in memory, LRU, keyed by (skeleton, outfit key, LOD)) skips to 4.
2. On a task: load the cooked parts and remap each part's palette to
   skeleton joint indices by name. Parts naming a joint the skeleton lacks
   are reported and bound to the nearest mapped ancestor.
3. Concatenate per material into one mesh per material per LOD, except
   morph-bearing submeshes (below). Bounds come from the rest pose plus a
   margin, and Bevy 0.19+'s skinned bounds take over at runtime. Steps 2–3
   are Bevy-free: `sf_character::assemble` (as built, M3-a).
4. Spawn: the skeleton's bone entities (`SkeletonBinding`, animation.md),
   one `SkinnedMesh` entity per material sharing the joint list, held
   items as child entities of socket joints (they change often and later
   carry physics and trails), head morph targets (`MorphWeights`), and the
   `Animator`.

An equipment change re-runs 1–4 for the body only, behind a one-frame
swap. The old mesh stays until the new one is ready, so it never pops.

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
  lights' path; `Off by default` ignored), no shadows; `HeldLightBudget` (default 12) keeps the
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
- **Carried-light near field (actorparticles).** The torch's `PointLight` is a child of the slot,
  scaled by `HeldLightTuning::intensity_scale` (0.7) and kept `clearance_m` (0.7 m) horizontally
  off the carrier's vertical axis (pushed outward from the flame, last frame's transforms).
  Morrowind lights saturate inside a third of their radius; ours are inverse-square, and Bevy has
  no per-light near clamp (Frostbite `max(d², r²)`, UE source radius), so a 3.66 m torch 0.25 m
  from the body lit it about 24x past "fully lit". Now the body is about 2x and the ground pool
  about half (bevyrrowind `held.rs`, test `carried_lights_keep_clear_of_the_carrier`).
- **Flames.** Particle systems of held props are bevyrrowind's (`actor_particles.rs`, nif-runtime.md
  "Particles on actors").
- **Not yet:** the bow string pull (needs clip curves on the prop joints), first-person props,
  interiors' darkness rule (needs the actor's cell), combat readiness, sheath mounts (vanilla shows none).

## Retargeting (`anim.retarget`, at bake)

For a source clip on skeleton S and a target skeleton T, both with rig maps:
1. Compute model-space retarget poses `Rs`, `Rt` per mapped bone.
2. Per frame, per mapped bone b: `qT_model(b) = qS_model(b) · Rs(b)⁻¹ · Rt(b)`,
   converted back to local with T's hierarchy. Chains with different
   joint counts (two spine joints against three) are matched by normalized
   arc length along the chain. Unmapped target joints keep T's rest
   rotation.
3. Root and hips translation is scaled by `hips_height(T) / hips_height(S)`,
   and root motion likewise.
4. Optional IK fix-up per chain: feet keep the source's scaled contact
   positions during contact (from the cooked contact curves); hands follow
   a shared socket for two-handed props.
5. Events, curves, markers and meta are carried over. Speed is rescaled.

**As implemented (`sf_anim::retarget`, 2026-10-11).** The math lives in
`sf_anim`, so it doesn't wait for `sf_character`. It takes a
`RetargetRig` per skeleton, which `sf_character` will build from the
`RigMap`:
- `chains`: named joint chains, root first. Chains pair across rigs by
  name. A humanoid bone is a one-joint chain named after it, and a spine
  or a tail is a longer chain. Chains with equal joint counts pair by
  index. Otherwise each target joint takes the nlerp of the two source
  joints around its normalized arc length (measured in the retarget pose).
- `retarget_pose`: the T-pose's local rotations, **replacing** the rest
  rotation of the joints listed (not deltas on top of it).
- `hips`: the joint whose translation crosses over. Translation uses the
  offset form `pT = PT + s·(pS − PS)` (P = retarget-pose position,
  s = hips-height ratio), so S → T → S is exact. Unmapped target joints keep
  their retarget-pose local transform. Joint local axes may differ between
  rigs: only model-space deltas cross over.
- IK fix-up (step 4): a target chain marked `ik` (three joints) pins its
  end to `PT_end + s·(pS_end − PS_end)` with two-bone IK, the FK knee as
  pole. The weight is the source's `contact.<end>` curve when the clip has
  one, else 1. Hands on shared sockets wait for `sf_character`.
- `contact.<joint>` curves and `down.<joint>` markers are renamed to the
  paired target joint. Additive clips are refused: retarget the base clip,
  then make the result additive on the target.

Tests check the round trip at ≤ 1° per mapped bone and ≤ 1 cm at the
effectors, between rigs with different joint axes, rest poses (A-pose
against T-pose) and an extra unmapped joint.

Which (clip, skeleton) pairs get baked is the graph's choice: the clip
sets a graph names for a character type, on the skeletons that use that
graph. Retargeting therefore never multiplies across every clip and
skeleton.

## First and third person

- **Third person**: the full assembled character.
- **First person (view model)**: a `FirstPersonRecipe` (arms skeleton,
  arm parts, clip set, graph) spawned on its own render layer and drawn
  by a second camera with its own FOV (default 60° vertical), and depth
  cleared, so arms never clip into walls. The third-person body stays
  spawned as a **shadow-only** caster with the head hidden, so the player
  has a shadow and a reflection.
- **True first person** (option): the third-person body is visible with
  the head and neck parts hidden. The camera sits at a head socket, with
  stabilization (camera-only damping on the head bone's motion).
- Held items attach to the active view's sockets. Switching views swaps
  attachments in one frame.

## Interfaces

```rust
/// Implemented by adapters (Bevy-free), like `WorldSource`.
pub trait CharacterSource: Send + Sync {
    /// Resolve a base record (NPC, creature) plus its runtime state
    /// (equipment overlay) into a recipe.
    fn recipe(&self, kind: &str, id: &RecordId, state: &CharacterState) -> Result<CharacterRecipe, CharacterError>;
    fn rig_maps(&self) -> Vec<(VfsPath, RigMap)>;   // per skeleton source
}
```

As built, `recipe` takes the base record's kind and id: `RecordRef` lives in
`sf_world`, and depending on it for one type would tie this crate to a
fast-changing one. Follow-up: move `RecordRef` to `sf_records`, then take
`&RecordRef`. `CharacterState { worn, held }` holds `Option`s: `None` keeps
the base record's own equipment.

`sf_character_bevy::spawn_character(commands, recipe, transform)` returns
the root entity. Equipment changes go through `CharacterState` (owned by
`sf_rpg` later; a component in M3).

## Verification

- Unit tests: slot resolution (layers, hides, ties, outfit key stable
  under reordering), rig auto-map on the test sets, a retarget round trip
  (S → T → S stays within 1° per bone and 1 cm at end effectors for
  same-proportion rigs), and palette remapping.
- Visual: the viewer's `character` mode shows a recipe in its rest pose,
  then walking. Quaternius locomotion retargeted onto a legacy rig, side
  by side with the source. Outfit swaps with no pop. First-person arms in
  a narrow corridor do not clip.
- Bench: draw calls and skin palette uploads per NPC (target ≤ 1 skinned
  mesh per material, ≤ 4 materials typical). Spawn cost per NPC on a
  merged-cache miss is ≤ 2 ms on a task, and 0 ms on the main thread
  beyond entity spawn.

## Industry comparison

J9 (industry-audit.md), 2026-10-12, docs research only (`.sessions/j9m3.md`).
Bevy 0.20 facts were read from `bevy_pbr-0.20.0`/`bevy_mesh-0.20.0` source
(see animation.md "Industry comparison").

| Technique | What shipped engines do | Us | Verdict |
|---|---|---|---|
| Modular assembly | UE offers three ways (Epic's modular-character chart): **Leader Pose** (followers copy the leader's bones; minimal game thread, high render thread, no per-part physics), **Copy Pose From Mesh** (each part runs its own anim instance; most expensive), **Skeletal Mesh Merge** (one mesh; low game and render thread cost, highest setup, **no morph targets**). UE Mutable (built in from 5.1, beta in 5.5): generates merged skeletal meshes, materials and textures in the background at runtime, "bakes morphs at character generation", then costs what a pre-made mesh costs. RAGE peds: 12 component slots, each a drawable plus one of its texture variations, props (hats, glasses) separate; streamed parts per ped. Creation Engine: biped slots on armor addons; NPC heads are **pre-baked per NPC** (FaceGen mesh + face tint texture generated in the editor from slider and tint data, not built at runtime). OpenMW: each body part is its own skinned mesh on the shared skeleton (observation) | merge per material at spawn, merged-mesh LRU cache, slots with coverage | **match** UE merge / Mutable; better than per-part draws for crowds in Bevy, where each `SkinnedMesh` entity gets its own palette copy |
| **Morph targets on merged meshes** | UE's merge drops morphs; Mutable bakes shape morphs into the generated mesh; Creation Engine bakes face shape into a per-NPC head mesh, leaving only expression morphs runtime | assembly step 4 gives the head `MorphWeights`; the cook stores target-major blocks for every model-wide target with zeros where a part lacks one | **closed by M3-a, 2026-10-12** (was GAP P1; see "Morph-bearing parts"): in Bevy 0.20 a morph target costs 48 bytes per vertex of the *whole mesh* it is attached to (dense `MorphAttributes`), and every vertex loops over every target in every pass. A 12k-vertex merged body carrying 30 head targets is ~17 MB per cached outfit, against ~2.9 MB for a 2k-vertex head kept apart and shared by every NPC wearing it. Rule to adopt (job M3-a): morph-bearing parts are never merged with other parts (own `SkinnedMesh` on the same joints, one extra palette copy); targets that are all zero on a part are stripped from it; per-character *shape* weights that never animate are baked into the vertices at assembly (Mutable, FaceGen) and only animated targets (blink, mouth, visemes) stay runtime; runtime morphs are dropped from LOD ≥ 2 (UE remaps morphs per LOD but each LOD may carry fewer). Open question 6 |
| Outfit caching | Mutable generates once and reuses; RAGE streams fixed per-ped drawables | LRU keyed by (skeleton, outfit key, LOD); pre-baked common outfits is open question 3 | **match** |
| Variation | RAGE texture variations per drawable and head blends; Creation Engine tint masks (skin, warpaint, complexion) | `TintParams` as material parameters; part choice in the recipe | **match** for color variation; per-part texture variants ride on parts |
| Retarget hub | Unity Humanoid avatar: runtime retarget through **muscle space**, range-of-motion limits per muscle. UE IK Retargeter: per-skeleton IK Rig with retarget chains, a pelvis (retarget root), chains auto-generated from templates by bone name ("Auto Retarget Chains"), retarget poses with an Auto Align tool for T vs A pose | VRM 1.0 bone hub (≈ Unity Humanoid's set), chain retarget at bake, auto-map, auto-align | **match** UE; **differ** from Unity: no muscle space and no runtime retarget, so runtime cost is zero; bake-time pairs only (graph-driven) |
| Twist and corrective deformation | UE: twist correction in retarget/post-process, Pose Driver (RBF) driving corrective bones or morphs; Unity Animation Rigging Twist Correction / Twist Chain constraints | twist joints left at the retarget pose (open question 4); no correctives | **differ**: legacy rigs have neither twist joints nor correctives; a `TwistDistribute` op is the cheap fix when a library rig needs it. Pose-driven correctives: not planned, noted |
| First person | Skyrim, Starfield: separate first-person arms; UE 5.5+ has a first-person primitive type (seen in `cast_far_shadow`'s docs: far-cascade casting is skipped for first-person primitives) | view-model arms + shadow-only body | **match**; Bevy still lacks a "shadow only" flag (open question 1) |
| Character LOD | UE skeletal mesh LODs with per-LOD bone removal and influence limits; RAGE ped LOD distances high / medium / low (and in vehicles) in `VisualSettings.dat` (modder reports) | `mesh.cook` LODs of the merged outfit, joint LOD masks per animation tier | **match** |
| Faces | MetaHuman rig logic, ARKit-style expression sets, viseme tracks | morph curves in clips, loudness-driven mouth | **differ, justified** for legacy content; visemes later |

Sources (checked 2026-10-12):
- UE modular characters: https://dev.epicgames.com/documentation/en-us/unreal-engine/working-with-modular-characters-in-unreal-engine
- UE Mutable overview: https://dev.epicgames.com/documentation/unreal-engine/mutable-overview-in-unreal-engine?lang=en-US ; wiki (5.5, beta status): https://github.com/anticto/Mutable-Documentation/wiki/ ; skeletal mesh generation: https://dev.epicgames.com/documentation/unreal-engine/mutable-skeletal-mesh-generation-in-unreal-engine
- UE skeletal mesh LOD reduction (`Remap Morph Targets`, morph error tolerance): https://dev.epicgames.com/documentation/en-us/unreal-engine/python-api/class/SkeletalMeshLODInfo ; https://docs.unrealengine.com/5.0/en-US/PythonAPI/class/SkeletalMeshOptimizationSettings.html
- UE IK Retargeter and auto retarget chains: https://dev.epicgames.com/documentation/en-us/unreal-engine/ik-rig-animation-retargeting-in-unreal-engine ; https://dev.epicgames.com/documentation/en-us/unreal-engine/auto-retargeting-in-unreal-engine
- Unity humanoid and muscle space: https://docs.unity.cn/6000.4/Documentation/Manual/AnimationOverview.html ; https://docs.unity3d.com/460/Documentation/Manual/MuscleDefinitions.html
- RAGE ped components (12 slots, drawable + texture): https://docs.fivem.net/docs/scripting-manual/using-new-game-features/collection-based-natives/ ; https://docs.sollumz.org/tutorials/basic-clothes-editing
- Creation Engine FaceGen (community docs): https://ck.uesp.net/wiki/Dark_Face_Bug ; https://www.nexusmods.com/skyrim/articles/413
- UE `cast_far_shadow` (first-person exception): https://dev.epicgames.com/documentation/en-us/unreal-engine/python-api/class/SkinnedMeshComponent

**Unverified**: RAGE ped LOD distance names (`ped.lod.distance.*`, from a
mod page's settings list, not Rockstar); OpenMW's per-part skinned meshes
(observation, not a doc); UE Pose Driver details (from memory); the 17 MB /
2.9 MB figures are arithmetic from Bevy's 48-byte `MorphAttributes`, not
measured.

## Open questions

1. Shadow-only rendering in Bevy 0.20: no built-in "cast shadows, don't
   draw" flag was found. A render-layer setup or a small material flag;
   decide at implementation.
2. Body-under-armor poke-through: slot hiding covers legacy content. Per
   vertex region masks (hide skin triangles under a tight item) are a later
   refinement. Segmented bodies and rigid armor coming apart at the joints:
   see smooth-bodies.md (seam weld and weight blending per outfit, on by
   default; smoothbody 2026-10-12).
3. Whether to also bake merged meshes for the most common outfits (guards)
   to skip spawn-time merging entirely. Measure the cache first.
4. Twist joints (`upperarm_twist_01`, `ForeTwist`) are unmapped, so a
   retargeted clip leaves them at the retarget pose. UE drives them with
   twist-distribution post-processing; decide whether that is a graph op
   (`sf_anim_graph`) or part of `anim.retarget` when a rig with skinned
   twist joints shows candy-wrapping.
5. Auto-align straightens the spine (7° off Quaternius's authored T-pose
   on UAL1). Consistent for any pair aligned the same way; revisit only if
   a hand-authored retarget pose meets an auto-aligned one.
6. Morph-bearing parts (gap, P1, job M3-a in industry-audit.md): keep them
   out of the per-material merge, strip all-zero targets per part, bake
   non-animated shape weights at assembly, and drop runtime targets from
   LOD ≥ 2. Open: where a morphed part and a merged part share a material
   (one extra draw per character, accepted), and whether the adapter or
   the cook marks a target as "shape" (baked) or "expression" (runtime).
   **Answered (M3-a, 2026-10-12)**: built as above ("Morph-bearing parts");
   the shared-material case is accepted (own mesh); the adapter marks the
   kind (`MorphTarget::kind`), the cook and assembly obey it.
7. Morrowind outfits keep 11–15 materials after the per-material merge
   (measured, "Morph-bearing parts"), so 12–16 skinned meshes and palette
   copies per NPC. A per-outfit texture atlas at assembly (UE Mutable) would
   make it one or two; measure the draw and palette cost in M3-c first.
