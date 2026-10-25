# Characters: assembly, equipment, standard rig, retargeting, views
Segmented legacy bodies and rigid armor bending apart at joints: see   smooth-bodies.md (seam weld + weight blending per outfit, on by default; smoothbody 2026-10-12).

Status: draft (2026-10-11); `sf_character` core built (2026-10-11, see "As implemented")
Crates: `sf_character` (new, Bevy-free: humanoid rig schema, rig maps,
slot resolution, recipes, retarget poses), `sf_character_bevy` (new:
spawning, part merging, attachments, faces, first and third person),
`sf_model` (extended: skin data in `Model` and `.sfmesh`), `sf_anim`
(retarget math, animation.md)

<!-- agent-summary -->
> **Agent summary (2026-10-26).** Status: `sf_character` core and `sf_anim::retarget` built 2026-10-11 ("As implemented"); ROADMAP 3.4 [~] holds deferred checks (CH-1/3 and SB-5 awaiting owner; SN-2/3, CH-4, SB-1/2/3, AP-2/3 deferred). Designed, not built: character creation UI, body sliders, runtime cloth (M4).
> Crates / key types: `sf_character` (Bevy-free; `HumanoidBone`, `BoneSet`, `auto_map`, `RigMap`), `sf_character_bevy`, `sf_model`, `sf_anim`. Decisions: one skinned mesh per material merged at spawn; slot equipment with coverage; VRM 1.0 humanoid as retarget hub, one rig map per skeleton; auto-map lets hierarchy decide the bone ("Canonical technique").
> Read next: "Data model" for types; "Assembly at runtime" for spawn/merge; "Retargeting" for clips; "Open questions" 4 (twist joints) and 6 (morph parts).
<!-- /agent-summary -->

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

Moved to [history](archive/character-history.md#morph-bearing-parts-m3-a-as-built-2026-10-12).

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
