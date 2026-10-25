# bake mesh.cook

Moved verbatim from [bake.md](bake.md) (doc cleanup S12c, 2026-10-26): the cooked meshes and LODs reference.

## Cooked meshes and LODs (`mesh.cook`)

**Cooking LOD0 too.** Models without animated parts (most statics) are
cooked whole, so streaming never parses a legacy format at runtime: it
decodes `.sfmesh` at GB/s on the task pool. Models with controllers or
particles are skipped (`Cooked::Skipped`) and keep the runtime importer
path; models with skins or morph targets cook to `Cooked::Skinned` (M3, see
"Skinned and morphed models" below). Flattening
and merging per material also cut draw calls, so LOD0 gets cheaper than the
source.

1. **Flatten.** Static nodes are transformed to model space and merged
   per material (mirroring transforms get their winding restored). The
   importer sets each node's `LodPolicy` (`Simplify`, `Keep`, `Drop`). Any
   animated node makes a model without skins or morphs skip cooking (see
   above). Importers leave collision geometry out of render meshes.
2. **Simplify.** Weld (`generate_vertex_remap`), then each level is
   simplified *from LOD0* with `simplify_with_attributes_and_locks`
   (normal, UV, and color weights; `ErrorAbsolute`), targeting half the
   previous level's triangles. The error cap is 1% of the bounding radius
   for LOD1, doubling per level. `Keep` nodes are vertex-locked; `Drop`
   nodes leave after LOD0; open borders are *not* locked by default (the
   error is sub-pixel at each switch, so seams between modular pieces are
   too, and locking froze legacy content made of open pieces: measured on the
   first real bake, most chains stalled at one level); `Prune` removes small disconnected bits from LOD2 on. If a level
   stalls, `simplify_sloppy_with_locks` makes one final level.
3. **Stop** at < 25% reduction, when the previous level is < 128
   triangles, or at 5 levels. Each level stores its absolute error in
   meters, never decreasing along the chain.
4. **Optimize** each level: `optimize_vertex_cache`, `optimize_overdraw`
   (threshold 1.05), `optimize_vertex_fetch`. Then encode.
5. **Runtime distance**: `d = e · H / (2 · tan(fov/2) · τ)` with screen
   height `H` and pixel threshold `τ` (1 px at the preset's reference 1440p
   and 70°), times a global LOD bias. The spawner writes it to
   `VisibilityRange` (crossfade margin 10% of d), clamped to the streaming
   distance table, and recomputes it when settings change.

**Binding**: [`meshopt`](https://crates.io/crates/meshopt) **0.6.2**
(MIT/Apache, compiles the C++ library with `cc`). Bevy 0.20's meshlet
feature depends on the same version, so there is one copy in the tree. It
vendors a pre-1.0 meshoptimizer: attribute-aware simplification with locks,
sloppy simplification, cluster building and partitioning. The v1.x
additions (clamped-error simplification, `simplifyWithUpdate`, the
`clusterlod` hierarchy) arrive with `meshopt` 0.7, which is on the repo's
main branch but not on crates.io (2026-10-11). Pure-Rust ports such as
`meshopt-rs` stopped in 2022.

**Format choice (reuse check).** Cooking to `.glb` would reuse Bevy's glTF
loader, but `bevy_gltf` supports neither `EXT_meshopt_compression` nor
`KHR_mesh_quantization`, so GLB would lose the codec's size and decode
speed. Bevy 0.20's `SerializedMesh` is documented as "for short-term
transfer", not a stable on-disk format. Hence `.sfmesh`. Its Bevy loader
builds plain `Mesh`es and can apply Bevy's own `Mesh::compressed_mesh`
vertex compression for GPU memory.

**Measured (2026-10-11):** about 4 ms to cook a 2,000-triangle prop with its
full chain on one thread. That projects to seconds for a full content pack
across 8 cores, a small slice of the 2-minute budget.

### Skinned and morphed models (M3, as built 2026-10-12)

**Source model.** `MeshData::skin: Option<MeshSkin>`: a palette of
`SkinJoint { node, inverse_bind }` (column-major 4×4, glTF
`inverseBindMatrices`) and per-vertex `(palette index, weight)` lists of any
length. `MeshData::morph_targets`: dense position (and optional normal)
deltas, a name and a default weight. Skinned positions are in bind space and
the mesh node's transform is ignored (the glTF rule). `skinned: true` with
no `skin` still means "skinned, not readable yet": the model is skipped as
before, so an importer can flag skins before it reads them.

**Cook** (`sf_model::skin`). A model with any skin or morph target cooks to
`Cooked::Skinned(CookedSkinned)`:

1. **No flatten, one group per source mesh** (a *part*, with its node path
   root first), so a runtime can apply `BodyPart::node_filter` before it
   merges parts per material (character.md, assembly step 3). Skinned
   meshes stay in bind space. **Rigid meshes become single-joint skins**:
   flattened to model space at rest and bound with weight 1 to the nearest
   node at or above them that moves (a skin joint or an animated node, else
   the root), inverse bind = inverse of that node's rest transform; mirrored
   transforms get their winding restored as in the static path.
2. **Palette.** One model-wide palette of `CookedJoint { name, inverse_bind }`,
   deduplicated by (node, inverse bind). A joint bound in two poses gets two
   entries (exact, where re-binding to one pose would change deformation).
3. **Influences**: repeated joints summed, non-positive weights dropped,
   the four largest kept (largest first) and renormalized to sum 1; unused
   slots are joint 0, weight 0. A vertex with no influence binds fully to
   its mesh's first joint.
4. **Weld** on every stream (vertex, skin, all morph deltas), then the same
   LOD chain as static models, simplified *from LOD0* in bind space. Extra
   simplifier attributes: the vertex's **joint center** (its weights' blend
   of the bind-space joint origins, weight 0.5 per meter) so collapses across
   hard joint boundaries cost about half the joint distance while smooth
   blends still simplify, and its **largest morph displacement** (weight
   1.0) so morphing regions keep vertices longer. Every level indexes a
   subset of LOD0's vertices, so weights stay normalized and joint indices
   valid by construction; per-level vertex fetch order is applied to every
   stream.
5. **Streams** per submesh (`CookedSkinStreams`, parallel to
   `CookedModel::lods[l].submeshes[i]`): `SkinVertex { joints: [u16; 4],
   weights: [f32; 4] }` and target-major `MorphDelta { position, normal }`
   blocks for the submesh's own target list only ("Morph layout" below),
   both meshopt-encoded. `bounds` are the rest pose.

**Rigid body parts** (`PartKind::Rigid { joint, offset }`):
`CookedSkinned::rigid(&cooked, joint, offset)` turns any cooked part
(static or deforming) into a single-joint skin with inverse bind = `offset`
and flips the winding when `det(offset) < 0`. The mirror is owned here, in
data, not by the renderer: Bevy 0.20 culls back faces with a fixed front
face (`MeshFlags::SIGN_DETERMINANT_MODEL_3X3` only fixes tangent frames), so
a negative-scale `Transform` would show the back faces.

**Format v2 → v3.** v2 added `Cooked::Skinned` as a new trailing enum
variant, so every v1 body is a valid v2 body. **v3 (M3-a, 2026-10-12)**
changed `Skinned`'s morph layout (per-submesh target lists), so v2 is
*retired*: `RETIRED_FORMAT_VERSIONS = [2]`, readers accept 1 and 3, a v2
file is a `FormatError::Version(2)` and re-cooks. Files carry the lowest
version that holds them (static and skipped cooks stay v1, byte-identical),
and a skinned body under a v1 header is rejected. `KEY_FORMAT` went 1 → 2
because the same skinned input now cooks to different bytes: the next bake
re-runs every `mesh.cook` / `mesh.rest` product once (≈ 38 s full model
re-cook). Importer versions are unchanged (the NIF importer's `Model` output
is identical: its targets take the default `MorphKind::Expression`, now set
explicitly). The glTF importer moved to version 2 with skins; an adapter
that starts emitting skins must bump its importer version.

#### Morph layout (M3-a, as built 2026-10-12)

Gap G1 (industry-audit.md): Bevy 0.20 stores morph targets densely, one
48-byte `MorphAttributes` (position, normal, tangent deltas, each padded to
16 bytes) per vertex of the whole mesh per target, and its vertex shader
loops over every target for every vertex in every pass. So:

- **`MorphTarget::kind: MorphKind`**, set by the importer (adapter):
  `Expression` (default; animated at runtime: blink, mouth, visemes) or
  `Shape` (per-instance slider that never animates). The NIF adapter marks
  every geometry morph `Expression` (all keyframed); glTF has no flag and
  keeps the default. `MorphTargetInfo` carries the kind.
- **Per-submesh target lists.** `CookedSkinStreams::targets: Vec<u16>`
  (ascending indices into `CookedSkinned::morph_targets`) names the targets a
  submesh carries; `morphs` holds exactly one block per entry, target-major,
  and is `None` exactly when `targets` is empty. No zero blocks.
- **All-zero stripping.** A target is carried by a submesh only if some
  position or normal delta component of its vertices exceeds
  `MORPH_EPSILON = 1e-6` (catches importer noise only). Targets no submesh
  carries are removed from the model list (and the lists renumbered).
- **LOD rule.** `Expression` targets are kept on LODs `0..RUNTIME_MORPH_LODS`
  (= 0 and 1) only; `Shape` targets stay on every level, because assembly
  bakes them (a far LOD would otherwise lose the character's face shape).
- **Shared helpers.** `CookedSkinned::decode_submesh` → `DecodedSubmesh`
  (vertices, indices, skin, targets, deltas); `DecodedSubmesh::bake_shapes`
  adds `weight × delta` of every `Shape` target into the vertices and removes
  it (position and normal; normals stay unnormalized as in Bevy's
  `morph_vertex`, skinning normalizes); `apply_block` is the runtime formula.

**Bevy mapping** (`sf_model_bevy`, built 2026-10-12: `skinned.rs` maps,
`spawn.rs` spawns; per submesh = one `Mesh`):

| Cooked | Bevy 0.20 |
|---|---|
| `Vertex` position, normal, uv, color | `ATTRIBUTE_POSITION` `Float32x3`, `ATTRIBUTE_NORMAL` `Float32x3`, `ATTRIBUTE_UV_0` `Float32x2`, `ATTRIBUTE_COLOR` `Float32x4` (u8 / 255, only if `has_colors`) |
| `SkinVertex::joints` / `weights` | `ATTRIBUTE_JOINT_INDEX` `Uint16x4` / `ATTRIBUTE_JOINT_WEIGHT` `Float32x4` |
| `joints[*].inverse_bind` | one `SkinnedMeshInverseBindposes` (`Mat4::from_cols_array`) per model, shared by its submeshes; `joints[*].name` → skeleton joint entities in `SkinnedMesh::joints` (same order) |
| `DecodedSubmesh` after `bake_shapes(default weights)` | the base vertices (a plain model bakes `Shape` targets at their default weight; a character's are baked by `sf_character::assemble` at the recipe's weights) |
| remaining `targets` (all `Expression`), `morphs` | `Mesh::set_morph_targets(Vec<MorphAttributes>)`: for block `k`, vertex `v`: `morphs[k·n + v]` → `MorphAttributes { position, normal, tangent: ZERO }` (target-major, the order `bevy_gltf` builds); `set_morph_target_names` from `morph_targets[targets[k]].name` |
| weights | one model-wide weight vector (by target name, driven by `sf_anim` curves); each morphing mesh entity gets `MeshMorphWeights::Value` with its own targets' weights gathered through `targets` (`MeshMorphWeights::Reference` only works when a mesh carries the full list in order, as glTF primitives do) |

No morph data on a submesh with empty `targets` (never call
`set_morph_targets`). Limits checked at load: 256 joints per `SkinnedMesh`
(`MAX_JOINTS`), 256 targets per mesh (`MAX_MORPH_WEIGHTS`), and
`vertices × 9 ≤ 2048²` per target layer when Bevy falls back to morph
textures (`MorphTargetImage::new`); above any of them the load fails with
`SfMeshLoadError::Limit` (no silent truncation; splitting is future work).

**As built (skinloader, 2026-10-12).** The loader returns the same
`CookedModelAsset` with `skin: Some(CookedSkin)` (palette names, one
labeled `inverse_bindposes` asset, morph target infos, per-mesh target
lists); static consumers (`sf_world_bevy` streaming, `mw_assets` cooked
spawn) treat a skinned asset as uncooked. `spawn_skinned(root, asset, lod,
joints, material)` copies `bevy_gltf` 0.20's spawn (`loader/mod.rs`): mesh
entities with `Mesh3d`, `SkinnedMesh { inverse_bindposes, joints }` and
`DynamicSkinnedMeshBounds`, joints resolved by name (`resolve_joints`
reports every missing one). Findings from Bevy 0.20's source:
- **Bounds.** `bevy_gltf` defaults to `GltfSkinnedMeshBoundsPolicy::Dynamic`:
  `Mesh::generate_skinned_mesh_bounds` stores per-joint bind-space boxes
  and `DynamicSkinnedMeshBounds` refits the entity `Aabb` from the joint
  transforms every frame, so a posed character is never culled from its
  bind-pose box. We do the same. The boxes are generated **before** Bevy's
  vertex compression (they read `Float32x3` positions) and survive
  `take_gpu_data`, so GPU-only meshes keep them. They ignore morph
  displacement (heads move millimeters; fine).
- **Batching.** On storage-buffer platforms (desktop) skinned and morphed
  meshes batch and go through multi-draw indirect like static ones
  (`no_automatic_skin_batching` / `no_automatic_morph_batching` only add
  `NoAutomaticBatching` on uniform-buffer platforms, WebGL 2). So the
  per-material merge in `sf_character::assemble` pays off; nothing in the
  mapping needs to change for MDI.
- **Compression.** `MeshCompressionArgs::regular()` also quantizes joint
  weights to unorm16; joint indices stay `Uint16x4`. Same position-compression
  scaffold as static meshes (`compression_args`).
- **Morph weights.** `MeshMorphWeights::Value` is read directly by
  `extract_morphs`; the model-wide vector lives in `ModelMorphWeights` on the
  model root and `sync_morph_weights` (PostUpdate) gathers it per mesh. That
  component is the seam `sf_anim_bevy` curves will write into.
- **Skeleton.** `spawn_rest_skeleton` (named scaffold) spawns an
  `sf_model::Model`'s node tree as named joint entities at rest; it stands in
  for `sf_anim_bevy`'s rig spawn. The app's `--skinned` view instead binds to
  the NIF runtime scene's named nodes.
- **Ray tracing.** No Solari variants for skinned models (Solari 0.20 traces
  static BLASes only).

**Reuse check.** Bevy: its skinning and morph runtime is used as is (the
mapping above); `bevy_gltf` does import skins, but only at runtime and
into Bevy types, which the Bevy-free cook can't use. Crates: the `gltf`
crate's reader (all `JOINTS_n`/`WEIGHTS_n` sets, inverse binds, morph
targets with sparse accessors expanded; `extras` feature for
`targetNames`, as `bevy_gltf` does) and meshopt 0.6.2's attribute-weighted
simplifier and codecs. Our own: the palette merge, the rigid → single-joint
conversion, the multi-stream weld and fetch remap (meshopt's
`optimize_vertex_fetch` reorders one stream; 0.6.2's `optimize_vertex_fetch_remap`
returns a remap truncated to the unique count, unusable as a full remap).

**Industry comparison.** glTF 2.0 is the data model (4 influences per set,
inverse bind matrices, dense or sparse morph targets). UE's skeletal mesh
reduction also simplifies from a base LOD, remaps morph targets onto
reduced LODs, offers "Enforce Bone Boundaries" (penalize collapses between
vertices with different major bones; our joint-center attribute is a
continuous version) and per-LOD "Max Bone Influences" and bone removal. We
skip influence reduction: Bevy's skinning always reads four weights, so
fewer would save nothing on the GPU, and joint LOD belongs to the
animation tiers (animation.md A0–A3). Not done: tangent morph deltas (the
cook keeps no tangents), per-target simplifier attributes (only the largest
displacement is weighted). **Gap (J9, 2026-10-12), closed by M3-a the
same day**: the zero blocks "where a part lacks a target" would have become
real GPU memory once parts are merged, since Bevy 0.20 stores morphs densely
(48 bytes per vertex per target) and loops over every target per vertex in
every pass; UE's merge drops morphs and Mutable bakes them. Now: per-submesh
target lists, all-zero stripping, expression targets only on LOD 0–1, shape
targets baked at assembly ("Morph layout" above; character.md
"Morph-bearing parts").

