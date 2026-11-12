# NPC CPU cost on the Balmora walk (research, 2026-11-12)

Scope: why NPCs cost ~2.0 ms per frame (10.95 ms with, 8.90 ms with `--off actors`, 1440p DLSS Q, render thread
bound). No code was changed or run. Evidence is the two Balmora walk reports (`ceil_base/balmora_walk@2560x1440-3.md`
"base", `ceil_actors_off/...off=actors-3.md` "off"), `target/perf/tr_on.trace.txt` ("on"), `tr_off.trace.txt` ("off"),
and the sources named. Anything not measured is marked **unverified**.

## Findings in one paragraph

NPC parts are already merged per material and Bevy 0.20 batches skinned and morph meshes on desktop, so the draw
count is not the problem. The cost is per-joint and per-skin CPU work: every mesh of an NPC carries its own copy of
the whole outfit's joint palette, and Bevy 0.20 extracts and uploads one joint-matrix block per skinned mesh entity
every frame, also for NPCs whose pose is frozen. Plus a bone write-back that walks every bone of every NPC each
frame. These add up to about the measured 2.0 ms.

## 1. How many NPCs, meshes, joints, draws

- Characters: `anim/characters/count` median 568, max 579 (base). Tiers: a0 0, a1 0, a2 1, **a3 566** (median); only
  20 are evaluated per frame (`anim/evaluated/count`, `controller/movers/count` 20). So ~99% of the NPCs are
  frozen (A3, under 20 reference pixels or off-screen) and still pay the render-side costs below.
- Entities: 59.2 k (base) vs 35.0 k (off) = 24.2 k extra, ~43 per character (joints + body + part meshes).
  Joint count per rig and mesh count per NPC are **unverified** (no gauge; `CharacterStats.meshes` at
  `sf_character_bevy/src/lib.rs:380` counts them but is not in the report). From 43 entities, an estimate of
  ~35 joints and ~5 meshes per NPC is consistent; treat it as an assumption.
- Parts are merged: `sf_character/src/assemble.rs:8-16,237` merges all parts per material (and role group) into
  one mesh; only the morph (face) part stays separate. `upload` (`sf_character_bevy/src/lib.rs:703-730`) creates
  one mesh entity per merged mesh, so an NPC = (distinct materials) + (face if morphed) skinned mesh entities.
  Merging per NPC into one mesh is therefore already done; one draw per material is the floor, since materials
  differ.
- **Palette duplication (the finding):** all those mesh entities get the whole outfit palette:
  `SkinnedMesh { inverse_bindposes, joints: joints.clone() }` at `sf_character_bevy/src/lib.rs:891` (same in
  `held.rs:358`). Bevy then treats each as an independent skin.
- Draw items (base vs off): `views/main/items` 8476 vs 7278 (+1200), `views/main/bins` 2892 vs 2086 (+806),
  `views/main/batch_sets` 44 vs 34 (+10), `phases/opaque3d/items` 3422 vs 2794 (+628), `opaque3d/batch_sets` 15 vs
  10, `views/cascade/items` 8172 vs 7086 (+1086), `phases/shadow/items` 9614 vs 8444 (+1170). `work_items`
  +1200 main. So the NPCs add ~1200 main-view and ~1100 shadow items but only ~10 batch sets.

## 2. Does batching break?

No, on this hardware. Bevy forces `NoAutomaticBatching` on skinned or morph meshes only when
`skins_use_uniform_buffers` (no storage buffers, e.g. WebGL2): `vendor/bevy_pbr/src/render/skin.rs:502-514` and
`render/morph.rs:433-447` both return early on desktop. Skin offset and morph descriptor are per-instance fields
in the mesh input (`render/mesh.rs:599-603,670-674`), so same-mesh instances batch. What does not batch: each
distinct (merged mesh, material) is its own bin (+806 bins), so work scales with bins and items, not with draw
calls. The ~83 batch sets in `views/all/batch_sets` are statics plus ~10 NPC ones; the >1 ms recording is not
explained by NPC unbatched draws (NPCs add ~10 sets out of 83). Whether `render_unbatchable_meshes`
(`vendor/bevy_render/src/render_phase/mod.rs:1435`) runs for any NPC part is **unverified**: the gauges
`phases/*/instance_batches|instance_draws` are 0 in both runs, which does not distinguish. Cheapest check: a counter
for unbatchable items in the phase (one line in `sf_diag`).

## 3. Where the 2.0 ms goes (tr_on vs tr_off)

Systems in tr_on's top list that are absent from tr_off's list (tr_off lists down to 0.13 ms, so they are below
that there):

| ms/frame | system | source and why |
|---:|---|---|
| 0.77 | `bevy_pbr::render::skin::extract_skins` | `skin.rs:272-326`: loops **every** `SkinnedMesh` entity; for each one with an allocation (visible) it loops all palette joints and does a `changed_transforms.get(joint)` (`skin.rs:392-414`). Cost = visible skins x palette joints. A frozen NPC still pays the lookups (all miss). |
| 0.39 | `prepare_skins` | `skin.rs:176-250`: one `write_buffer` of the entire staging buffer every frame; no dirty ranges. Bytes = visible skins x palette joints x 64. |
| 0.29 | `(&AnimJoint, &mut Transform)` par_for_each | `sf_anim_bevy/src/lib.rs:664-695` (`write_bones`): visits every bone of every character (~568 x 35 = ~20 k) and does `animators.get(joint.character)` per bone; only 20 characters are posed. |
| 0.20 | `(&mut Animator, &mut AnimLod, ..)` | the evaluate loop (`lib.rs:728`); ~20 evaluated of 568, so mostly iteration overhead. |
| 0.32 + 0.27 | `extract_render_asset` / `prepare_assets<GpuShaderBuffer>` | see section 4. **Cause unverified.** |
| 0.32 | `prepared_mesh_producer` | per-item mesh preprocessing (shared with statics); scales with the +1200 items. |

Sum of the NPC-specific rows: 0.77 + 0.39 + 0.29 + 0.20 + 0.59 = 2.24 ms, against the 2.05 ms measured
difference; so the list covers the cost. The 2.1 ms `InheritedVisibility/ViewVisibility` par_for_each exists in
both runs; the NPC share is not separable from these files.

## 4. `GpuShaderBuffer` per frame

Creators and writers (`grep ShaderBuffer`): wind field `sf_wind_bevy/src/lib.rs:186-199` (clear + extend every
frame), terrain params `sf_terrain_bevy/src/lib.rs:1004-1008` (every frame), the particle buffer
`mw_assets/src/particles.rs:255-263` (`upload_particles`: clear + `extend_from_slice` of the **whole** buffer when
any emitter wrote). `simulate_particles` (`particles.rs:550+`) writes each live emitter within 12 m and in frustum
every frame (`write` at 147-163 sets `dirty`). Each modified `ShaderBuffer` is cloned out of the main world by
`extract_render_asset` and rewritten by `prepare_asset` (`vendor/bevy_render/src/storage.rs:304-350`). Wind and
terrain exist in both runs, but the two GpuShaderBuffer systems are above 0.13 ms only with actors. So the actor
path is the likely reason: actor particles (`mw_actors_bevy/src/actor_particles.rs`: emitters on creature or
skeleton particle nodes) and held-item fire dirty the shared particle buffer, which then copies all slots
(1024-particle chunks of 48 B each, grown by slot allocation) three times per frame (asset, extract clone, GPU
write). **Unverified**: no gauge shows the buffer size or dirty frames. This is a diagnosis slice, not a fix.

## 5. Animation update rate and LOD

- Tiers exist and work: `select_tiers` (`sf_anim_bevy/src/lib.rs:440-535`) picks A0-A3 from projected pixels (thresholds
  120/50/20 reference px, `lod.rs`), A2 every 2 frames, off-screen to A3 (`cull_offscreen`), and
  `idle_skip_a3` stops evaluation for A3 without physics. UE's URO/Animation Budget Allocator is matched here.
- Joints are not written when unchanged or frozen (`lib.rs:683-689`, compare then assign), so transform
  propagation stays quiet for frozen NPCs. The cost is the visit itself (write_bones above).
- Skin extraction is already limited to *visible* skins (`add_or_delete_skins`, `skin.rs:329-366`: skins whose
  `ViewVisibility` is false get no allocation and the joint loop returns at `skin.rs:388`). But "visible" is
  main-view-visible regardless of pose age, so far frozen NPCs (A3, <20 px) are extracted and uploaded each frame.
  Re-extraction when nothing changed is the waste.

## 6. Industry standard and what maps to Bevy 0.20

- UE: one `FSkeletalMeshObject` holds one bone-matrix buffer for all its sections (materials); the sections share
  it. Modular characters are merged (`FSkelMeshMergeUtil`) per material, which we already do. Update-rate
  optimisation (URO), `VisibilityBasedAnimTick` and the Animation Budget Allocator skip evaluation and bone
  transform updates for far or off-screen meshes; far crowds use impostors or vertex-animation textures.
- Unity: SkinnedMeshRenderer combining (per material); shared bone buffer per renderer; GPU skinning; quality
  `skinWeights` and animator culling (`CullUpdateTransforms`) stop updates when not visible.
- Bethesda/OpenMW: Morrowind NPCs are many NIF part nodes on one skeleton, one skin per part, no merge; OpenMW
  keeps animation but draws distant actors; no skin-buffer sharing is documented. Not a model for us beyond the
  rule "do not animate what is far".
- Mapping: merging per NPC (done) -> **share one joint block per outfit** (the UE shape) -> **do not upload frozen
  poses** -> **skip per-bone visits for frozen characters** (URO). Switching far frozen NPCs to non-skinned
  geometry is the next step after that and larger.

## 7. Recommended slices (in order)

**Slice A: one joint block per outfit (vendored patch of `skin.rs`).** `extract_skins`, `add_skin`, `remove_skin` and
`SkinUniforms::skin_uniform_info` (`skin.rs:272-498`) key allocations per skinned-mesh entity. Patch: key by
(inverse-bindposes asset id, hash of the `joints` Vec) with a refcount; entities with an equal key reuse one
allocation and the joint loop runs once per key. Upstream-style carried patch in `superfargoth/vendor/` per rule 4;
no visual change, since matrices are identical (same joints, same inverse bindposes: required equality). Expected:
if NPCs have n skinned meshes, the joint work divides by n; with the assumed n ~ 5, `extract_skins` 0.77 ->
~0.2 (fixed per-entity part ~0.15-0.2 remains) and `prepare_skins` 0.39 -> ~0.1: **~0.8-0.9 ms**. First read
`CharacterStats.meshes / characters` to confirm n; at n = 3 the gain is ~0.55 ms.
Verify: perf suite `balmora_walk` (same command as the ceiling runs) and compare `extract_skins` and
`prepare_skins` in the trace; add a gauge `skin/allocations` and `skin/joints` in `extract_skins`. Test:
equal skins share an offset, different `joints` do not, removing one entity keeps the block until the last user is gone.
Risk: previous-frame skin matrices (motion vectors) use the same block and swap, so shared is correct; a mesh
whose palette differs from its siblings (held items, `held.rs:358`) gets its own key.

**Slice B: `write_bones` per character, not per bone** (`sf_anim_bevy/src/lib.rs:664-695`). Iterate characters
with `Animator.posed` or a `PhysicsPose` that has a fully physical joint, and write through
`SkeletonBinding.joints` (`lib.rs:317`) with `Query::get_unchecked` or `iter_many_mut` (bones are unique per
character, so the writes cannot alias). This removes ~20 k `animators.get` calls for ~566 frozen characters:
**~0.25 ms** (0.29 -> ~0.04: 20 posed x 35 bones). No visual change (identical writes). Verify: same suite;
`anim/apply/ms` and the par_for_each line; unit test that a posed character's bones equal the old result and a
frozen one's are untouched. Keep the existing `bones_written` stat.

**Slice C: diagnose `GpuShaderBuffer` (counters first).** Add gauges: modified `ShaderBuffer`s per frame, bytes
uploaded (particle `data.len() * 48`, wind, terrain), emitter count and `dirty` frames in `upload_particles`.
Run the walk with and without actors to attribute 0.59 ms. Likely fix, decided after the numbers: upload only the
changed slot ranges (or skip actor/held emitters beyond the already existing `range` / frustum filter), expected up to
**~0.4-0.5 ms** of the 0.59. This is one deliverable (diagnosis), the fix is a separate slice.

Sum if all work: ~0.8 + 0.25 + 0.4 = ~1.4 ms of the 2.0 (to ~9.5 ms on the walk). Slices A and B are independent
and safe to land first. Per the "measure the ceiling" rule, these are measured against `--off actors` 8.90 ms: the
floor for any NPC cost fix is that number.

## 8. Open

1. Per-NPC skinned mesh count and palette length: read `CharacterStats` (and log `outfit.joints.len()`) on the
   walk; the Slice A gain scales with the former.
2. Whether any NPC part ends up on `render_unbatchable_meshes` (gauge for unbatchable items per phase).
3. Which writer dirties `GpuShaderBuffer` only with NPCs present (section 4 is inference).
4. Share of `ViewVisibility`/`InheritedVisibility` par_for_each (2.1 + 1.1 ms) caused by NPC part and joint entities:
   joints carry `Visibility`/`InheritedVisibility` (see `actor_particles.rs:336` `visible_chain`), 20 k joint
   entities; test: do joints need `Visibility` at all?
5. Shadow items: NPC +1170 shadow items; a cascade distance cap for far A3 NPCs (visual change; owner's call).
6. Far frozen NPCs as non-skinned geometry or impostors (UE far-LOD shape): larger slice, after A to C, only if
   `extract_skins` remains in the top list.
