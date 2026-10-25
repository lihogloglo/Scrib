# Smooth bodies: seam welding, weight blending and weight transfer for segmented characters

Status: research done, Bevy-free prototype built and measured (smoothbody, 2026-10-12), per-seam
vanilla guard and survey on corrected input (smoothbody3, "Survey rerun and guard"),
made robust and surveyed over every playable race × sex and eight outfits (smoothbody2,
2026-10-12, "Survey"), live in `sf_character_bevy` with an A/B (smoothlive, "Live"). Seam shading (seams, "Seam shading"). Bake pre-warm not yet.
Crate: `sf_character`, module `smooth` behind feature `smooth` (parry3d). Adapter
example: `bevyrrowind/crates/mw_actors/examples/smooth_body.rs`.

## Goal

Legacy bodies are often **segmented**: each limb piece (upper arm, forearm, wrist, knee,
ankle…) is a rigid mesh on one joint, so cracks open and pieces shear at the joints when the
skeleton bends. Armor and clothing are built the same way. The framework turns any such part
set into a smoothly skinned one **with no new art**:

- **On by default**, the vanilla segmented look as an option (owner decision 2026-10-12,
  ARCHITECTURE §10).
- Works on any content: base game, mods, and body replacers. A replacer body that is already
  skinned (Better Bodies, Robert's Bodies) has no seams, so the step leaves it unchanged and it
  keeps working as plain data.
- Armor and clothing bend with the body, so a smooth body never pokes through rigid armor.
- Game-neutral: the algorithm sees surfaces, skin weights and a skeleton parent table. Which
  parts make up a body and the units are the adapter's business.

## Canonical technique

Two operations from the skinning literature, both forms of **weight inpainting**
(Abdrashitov et al. 2023): keep weights where they are trustworthy and solve a smooth
interpolant (here harmonic, as in Pinocchio's heat equilibrium and BBW's family) everywhere
else.

1. **Seam welding and blending** (`smooth_body`). Over all parts in rest space:
   - Exact duplicates inside a part (UV/normal splits) are merged for topology, and open edges
     give each part's boundary vertices.
   - **Seams**: each boundary vertex is linked to the closest point (parry3d BVH, surface not
     vertices) of every other part within `link` (default 4 cm). A link counts only when the
     two parts' dominant joints are skeleton-adjacent: at most `max_hops` (4) edges apart, and
     one of them at most one below their common ancestor. A segment meets its parent chain; a
     hand resting on a thigh, or two thighs touching, is not a seam.
   - Links closer than `weld` (2 mm) between boundary vertices are **welded**: one position,
     one averaged normal (no shading seam), one weight set. That is the standard seam fix for
     modular meshes: matching positions, normals and weights at the boundary.
   - A **band** of `band` (8 cm) geodesic distance (Dijkstra over mesh edges plus seam links)
     around the seams is re-solved: inside it, weights are the harmonic interpolant
     (Gauss-Seidel on an inverse-edge-length graph Laplacian). Outside it they stay fixed, which
     is the boundary condition. Rigid mid-segment, blended across the joint. Harmonic weights
     stay in [0, 1] and sum to 1 (maximum principle). Then the top 4 influences are kept and
     renormalized.
2. **Weight transfer** (`transfer_weights`). Target vertices (clothing, armor, hair) within
   `max_distance` (10 cm, about 5% of the body's bounding-box diagonal) of the smoothed body,
   whose normal agrees within 35° (either facing), copy the barycentric weights of the
   closest body point. The rest are inpainted over the target's own surface. Pieces with no
   match keep their weights.

Everything works in **rest space**: positions are the skeleton's rest pose in model space,
and joint `j`'s inverse bind is `rest_model[j]⁻¹` (`bind_palette`). Rigid parts
(`CookedSkinned::rigid`) and authored skins both convert exactly
(`SkinSurface::from_cooked`), so mixing their weights is valid.

### Finding: for rigid armor, smooth the outfit as one segmented set

The prototype measured three ways to handle armor (table below). Transfer from the naked body
(the BodySlide / UE approach) is **worse** than vanilla for bulky Morrowind armor. Plates stand
off the body by more than the match distance, so about 77% of armor vertices are inpainted from
a few matched rims, plates bend where they shouldn't, and seams between armor pieces aren't
enforced. Running `smooth_body` on the **visible dressed set** (body parts left visible, plus
every armor piece) treats armor pieces like body segments: each plate stays rigid in its
middle and blends with its neighbors (pauldron ↔ bracer, greave ↔ knee guard, bracer ↔
bare wrist). That is the best result on every pose with no new pokes. So:

- **Rigid pieces** (vanilla armor, clothing and body parts): `smooth_body` over the visible
  outfit. The result is per outfit, keyed by the outfit key.
- **Close-fitting skinned clothing that lacks weights where it should bend**, and hair:
  `transfer_weights` from the smoothed naked body (it suits tight cloth; hair copies the head
  and neck weights).
- Content that is already skinned and seam-free passes through unchanged (no seams, no band).

## Measured (prototype, 2026-10-12)

Male dark elf (`sovor trandel`, `base_anim.nif`): 20 parts, 1,850 vertices. Armor: iron
cuirass, greaves, both pauldrons (12 pieces, 2,375 vertices). The visible body when dressed is
the head, neck, hands, wrists, feet and ankles. Poses: elbows 90°, knees 90°, shoulders raised
57°, and `WalkForward` at 25%. "Seam opening" means how far linked seam pairs move apart
relative to rest (max / p95, mm). "Poke" counts visible-body vertices that were inside the armor
at rest and are outside it in pose (vertices > 1 mm, max depth). Full output:
`shots/smoothbody/metrics.txt`.

| Pose | Naked vanilla | Naked smooth | Dressed vanilla | Dressed, transfer | **Dressed, outfit smooth** |
|---|---|---|---|---|---|
| elbow 90° | 45.2 / 38.4 | **1.9 / 0.0** | 56.5 / 30.2, poke 0 | 13.3 / 12.9, poke 7 (37 mm) | **1.6 / 1.4, poke 0** |
| knee 90° | 0 / 0 (seam not found, see below) | 0 / 0 | 108.3 / 12.5, poke 0 | 0 / 0, poke 16 (33 mm) | **27.4 / 1.2, poke 0** |
| shoulder | 0 / 0 | 15.3 / 0.6 | 0 / 0, poke 0 | 63.9 / 30.9, poke 10 | **0 / 0, poke 0** |
| walk frame | 40.1 / 26.4 | **12.3 / 5.0** | 37.3 / 17.2, poke 1 (6 mm) | 69.2 / 25.1, poke 5 | **7.1 / 2.8, poke 1 (6 mm)** |

"Smooth body + rigid armor" (no armor step) leaves the vanilla armor gaps (elbow 56.5 mm) and
adds pokes at the shoulder (2 vertices, 26 mm) and in the walk (2, 30 mm). That confirms the
owner's concern: armor must get the step too.

Gaps found by the prototype, both fixed in smoothbody2 ("Robustness rules"):
- **Knee**: the upper-leg and knee pieces overlap 4–8 cm deep, out of reach of the 4 cm gap
  link. Fixed by the overlap (volume) links.
- **Shoulder (naked)**: one chest-skin ↔ upper-arm link opened 15 mm because the band
  re-solved authored chest weights. Fixed by keeping authored skins as boundary values.
- Not measured: triangle self-intersection (no tri-tri test), elbow volume loss (linear blend
  skinning candy-wrapping; dual quaternions are a renderer question).

**Cost** (release build with the owner on the machine, so indicative only): `smooth_body` 15–17 ms
per naked body or dressed outfit, `transfer_weights` about 40 ms for the armor set, 145–170
Gauss-Seidel sweeps. The output is a new weight stream only (24 B per vertex: `SkinVertex`)
plus welded positions: about 100 KB for body plus armor.

## Robustness rules (smoothbody2)

The first prototype missed overlapping pieces and re-solved authored skins. The survey below
drove these rules, all game-neutral (they look only at surfaces, weights and the skeleton):

1. **Overlap links (volume test).** A boundary vertex also links to an adjacent part when it lies
   *inside* it (the way to the closest point runs against that face's outward normal, within
   `overlap`, 8 cm), or when its rim is a *shell over* it (outside it, its own normal facing
   away). Closest points on the other part's open rim edges don't count: beyond an open tube's
   end the normal says nothing. Face orientation comes from the sign of the enclosed volume,
   so mirrored (inward-wound) pieces work. This finds the knee (thigh and knee pieces overlap
   4–8 cm deep) and every sleeve-like armor overlap.
2. **One partner per rim vertex**: the nearest skeleton-adjacent part, with coincident (welded)
   links first, then overlap links, then gap links. Linking to everything in reach made false
   seams (ankle ↔ thigh, ankle ↔ skirt hem, thigh ↔ tail). `max_hops` is now 3.
3. **Authored skins are boundary values** (`keep_authored`, default on). A part is *rigid* when
   every vertex is fully on the same joint; anything else is authored and never re-solved. This
   fixes the naked shoulder (the chest skin kept its clavicle weights, the rigid upper arm blends
   into it).
4. **Seam vertices follow the other side** (a hard constraint in the Gauss-Seidel solve, not a
   soft edge): the child-side vertex (deeper dominant joint; ties by part order) takes the
   barycentric weights of its closest point on the parent side; next to an authored skin the
   rigid side follows. A follower never follows another follower, so there are no cycles.
5. **Overlap copy**: after the solve, a rigid part's vertices hidden inside a linked neighbor
   take the neighbor's weights at their closest point (the inner surface moves with the shell
   and cannot poke out); seam followers are re-applied after the copy.
6. **Determinism**: adjacency is sorted, not hashed, so the result is bit-identical across runs
   and threads (unit test `deterministic`, and checked on every survey case).

Moved to [history](archive/smooth-bodies-history.md#survey-smoothbody2-2026-10-12).


## Where it runs (decision)

The owner's decision was a bake step. The equipment combinations a player can wear are
unbounded, so a bake keyed on the part set can't cover them. The same function therefore runs
in two places, keyed by the outfit key:
1. **Assembly** (`sf_character_bevy` build task, already off the main thread): between
   resolving the cooked parts and `assemble`, convert to `SkinSurface`, run `smooth_body` (and
   `transfer_weights` for hair and skinned cloth), write the new weights and `bind_palette`
   back, then assemble. `OutfitCache` already caches by outfit key; add the smoothing mode to
   `OutfitCacheKey`. At 15–40 ms on a worker it is within the spawn budget (character.md: task
   ≤ 2 ms *main*, background unconstrained).
2. **Bake pre-warm** (optional, later): NPC outfits are known from records. A `outfit.smooth`
   step keyed by (skeleton, outfit key, settings hash, part product hashes) stores the
   weights. That means no runtime cost for the common case and a byte-identical result.

Mode: `SmoothBodies::{Smooth (default), Vanilla}` in the character settings and the cache key.
Vanilla skips the step.

## Industry comparison

| Technique | What's used | Us | Verdict |
|---|---|---|---|
| Automatic skinning from scratch | Pinocchio "bone heat" (Baran & Popović 2007; Blender's automatic weights), Bounded Biharmonic Weights (Jacobson et al. 2011; needs a tet mesh, Adobe Character Animator), Geodesic Voxel Binding (Dionne & de Lasa 2013; Maya's default bind, robust to non-manifold, multi-component meshes) | not used | **differ, justified**: we already have trustworthy weights (rigid in mid-segment, authored on skin parts). Recomputing everything discards them and needs a watertight volume or a voxelization of multi-part, overlapping meshes. GVB is the fallback if a body without any weights ever shows up |
| Weight transfer | Outfit Studio / BodySlide "Copy Bone Weights" (proximity from a reference body, distance setting, masks; community docs), UE `FTransferBoneWeights` and `CopySkinWeightAttributeFromMesh` (closest triangle, barycentric), UE Chaos Cloth `TransferSkinWeights` node ("InpaintWeights", based on Abdrashitov et al. 2023) | `transfer_weights`: closest point + normal test, harmonic inpainting | **match** (the UE 5 inpainting method; harmonic, not their Laplacian-squared energy), and measured as the **wrong tool** for bulky rigid armor (above) |
| Seam welding of modular meshes | matching vertex positions, normals and weights on the boundary (Better Bodies 2.2's "seamless" fix for lighting seams; UE/Mutable modular parts authored to match) | weld < 2 mm, averaged normals, shared weights; blend band beyond | **match**, automatic |
| Morrowind body replacers | Better Bodies, Robert's Bodies: hand-made continuous skinned bodies, plus matching clothing (Better Clothes) built for the unsegmented body; vanilla-mesh races then clash (Tamriel Rebuilt issue) | automatic, no art, and vanilla- and mod-compatible | **better** for coverage, **worse** for shape: replacers also remodel the body. They still load as plain data and pass through unchanged |

**Reuse check.** Bevy 0.20 has no skin-weight tools (it only consumes weights). Crates:
- No Rust crate does skin weight transfer or inpainting.
- Sparse solvers: `faer` (MIT, sparse Cholesky / LDLT) and `nalgebra-sparse` (Apache-2.0,
  CSC Cholesky). Both are license-compatible. Neither is needed at our sizes: a few hundred to
  a thousand unknowns converge in under 200 Gauss-Seidel sweeps (~15 ms). Adding `faer` would
  cost compile time (ARCHITECTURE §8b) for no gain. Revisit if a biharmonic energy (the paper's
  smoother transitions) or 50k-vertex bodies need a direct solver.
- Closest-point queries: **parry3d** (Apache-2.0, already in the workspace for `sf_controller`)
  `TriMesh` + `CompositeShapeRef::project_local_point`. It sits behind the `smooth` feature so
  runtime-only users of `sf_character` don't pull it in.
- The reference implementation of Abdrashitov et al. (github.com/rin-23/RobustSkinWeightsTransferCode,
  Python + libigl) is listed as MIT. We did not copy it: our version is written from the
  paper's description, with a different energy and solver.

Sources (checked 2026-10-12):
- Abdrashitov, Raichstat, Kavan, Jacobson et al., "Robust Skin Weights Transfer via Weight Inpainting", SIGGRAPH Asia 2023 TC: https://www.dgp.toronto.edu/~rinat/projects/RobustSkinWeightsTransfer/ ; code: https://github.com/rin-23/RobustSkinWeightsTransferCode
- UE: https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/DynamicMesh/FTransferBoneWeights ; https://dev.epicgames.com/documentation/ja-jp/unreal-engine/node-reference/Dataflow/TransferSkinWeights ; https://dev.epicgames.com/documentation/unreal-engine/create-an-action-utility-for-transferring-skin-weights-in-unreal-engine
- BBW: https://homes.cs.washington.edu/~jovan/papers/jacobson-2011-bbw.pdf ; Pinocchio: https://homes.cs.washington.edu/~jovan/papers/baran-2007-ara.pdf
- GVB: https://diglib.eg.org/handle/10.2312/15001 ; Maya: https://help.autodesk.com/cloudhelp/2016/ENU/Maya/files/GUID-5EFDB81B-E332-4D6C-B1BB-0B989AD2F2C7.htm
- Outfit Studio rigging: https://wiki.beyondskyrim.org/wiki/Arcane_University:Rigging_in_Outfit_Studio
- Better Bodies: https://en.uesp.net/wiki/Tes3Mod:Better_Bodies ; TR issue: https://tamriel-rebuilt.org/node/5706

**Unverified**: the paper's defaults are from memory (distance 5% of the bounding-box
diagonal, 30° normal angle, energy `−L + L M⁻¹ L`), because the PDF text wasn't readable here.
Outfit Studio's matching rule is not documented beyond its "distance" setting. Better Bodies'
internal weighting is not documented (observation only).

## Interfaces (as built)

```rust
// sf_character, feature "smooth"
SkinSurface { positions, normals, triangles, skin: Vec<SkinVertex> }   // rest space, skeleton joints
SkinSurface::from_cooked(&CookedSkinned, lod, Option<&BodyPart>, joint_index, rest_model)
SkinSurface::posed(&skinning) -> Vec<Vec3>
smooth_body(&mut [SkinSurface], parents, &SmoothSettings) -> SmoothReport  // weld + band + harmonic
transfer_weights(&body, &mut targets, &TransferSettings) -> TransferReport
find_seams(&parts, parents, &SmoothSettings) -> Vec<SeamLink>         // metric seams
seam_openings(&parts, &seams, &skinning) -> Vec<f32>                  // per link, one pose
guard_seams(&original, &mut smoothed, &seams, &probe_poses, tolerance, radius) -> GuardReport
blend_seam_normals(&mut parts, parents, &SmoothSettings) -> usize       // seam shading, normals only
penetration(rest_pts, pts, rest_surface, posed_surface, max_dist) -> Vec<f32>
bind_palette(names, rest_model) -> Vec<CookedJoint>; skinning_matrices(rest, posed)
```

`SmoothSettings { weld, link, overlap, max_hops, band, keep_authored, iterations, tolerance }`
(defaults 2 mm, 4 cm, 8 cm, 3, 8 cm, true); `SeamLink { a, b, inside }`.

Tests (`smooth::tests`): `segmented_limb_seam_closes_and_weights_blend` (two rigid tubes at a
hinge: the vanilla seam opens at 90°, the smoothed seam stays closed (< 0.1 mm), the seam
blends 50/50, weights outside the band stay rigid, a sleeve takes the limb's weights, a
seamless tube is left alone); `overlapping_rim_is_linked_and_stays_inside` (a child rim 7 cm
inside a sleeve: no gap link, an overlap link both ways; vanilla pokes out when bent, smoothed
< 5 mm); `authored_skins_pass_through` (seamless skinned body and two welded authored pieces
unchanged; a rigid piece next to an authored one blends, the authored side is kept);
`deterministic` (bit-identical across runs and four threads).

## Next steps

1. Done (smoothlive, "Live"). Was: **Live A/B** (`sf_character_bevy` + app; heavy builder): add `SmoothBodies` mode to
   `CharacterContent`/settings and `OutfitCacheKey`. In the build task, after the cooked parts
   are resolved: `SkinSurface::from_cooked` per part (keep a per-submesh vertex map), then
   `smooth_body` over the visible outfit (+ `transfer_weights` for hair). Write the weights and
   positions back into the decoded submeshes, with `bind_palette` as the palette (skip
   `CookedSkinned::rigid` for parts the step rewrote). Then `assemble` as today. To do this,
   extend `assemble` to accept pre-decoded parts (`AssemblyPart::decoded`), or add
   `assemble_surfaces`. App: `--bodies vanilla|smooth` on `view`/`character`, a key to toggle
   (rebuilds through the no-pop swap), and a scene `char-smooth` (lineup in the elbow/walk pose)
   in visual-checks.md, awaiting the owner.
2. Done (smoothbody2): volume test, authored pinning, survey. Open: the four remaining
   failure groups under "Survey" (female knee overlap, armor wrist/pauldron pieces, beast
   tails with outfits, robe ↔ wrist); rerun `smooth_survey` after each change.
3. `outfit.smooth` bake pre-warm for NPC outfits from records (keyed as above). Measure the
   whole-install time first: 3,041 NPCs at ~20 ms CPU ≈ 60 s single-thread, a few seconds on
   rayon.
4. Optional: biharmonic energy (smoother transitions), a per-joint band from bone length,
   dual-quaternion skinning for elbow volume (renderer).
