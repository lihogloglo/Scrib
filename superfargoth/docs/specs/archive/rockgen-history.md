# Rock generation: history (`../rockgen.md`)

As-built notes of finished rows moved out of the spec (2026-11-27, ecosystems loop), oldest first: RG-1, RG-4, RG-3,
RG-4b, RG-2, RG-2b and RG-8a. Current truth is the spec's Design and Build plan; later rows (RG-9j, RG-P) stay in its
"As built".

## As built (RG-1 to RG-8a)

**RG-1 (2026-11-22).** `superfargoth/crates/sf_rockgen` (`lib.rs` mesh types + hash, `noise.rs` counter-hash value
noise, `boulder.rs` SDF + generator, `boulder/tunables.rs`, `tests.rs`). Tunables are declared (`TUNABLES`) but not
yet registered in the game's catalog (RG-3, with the viewer). Measured 2026-11-22 (`test -p sf_rockgen --release
measure_boulders -- --ignored --nocapture --test-threads=1`, owner laptop, 24 seeds, resolution 40), ms per rock
and mean triangles LOD0/1/2/3: default block 18.7 ms, 429/214/107/53; platy 7.3 ms, 352/175/87/43; cobble 8.4 ms,
445/222/110/54; chipped (24 chips, 12 m) 24.8 ms, 415/207/104/51. Phases (default): field sampling ~9 ms (≈ 80 ns per
evaluated sample, noise in the band dominates), surface nets ~1 ms (12.5 k raw triangles), LOD chain ~4 ms, normals
+ compaction the rest. A 6-class × 16-variant library ≈ 2 s on one thread. LOD0 is low (~430 triangles for any size:
error is in cells) by design for triplanar-shaded rock; the owner judges at RG-3 (raise `resolution` or lower
`lod0_error_voxels` if edges read faceted).

**RG-4 (2026-11-22).** `sf_rockgen`: `sdf.rs` (`Solid`: hard planes per piece, `snap`, `surface`, pits), `classes.rs`
(`CLASSES`: 9 presets with family, analogue, source, tint; the 4 RG-3 classes unchanged), `columns.rs` (Voronoi
prisms, shared faces as `Piece::hard` planes), `mesher.rs` (ambiguous-face resolve, `separate_coincident` at 2^-17,
`resplit_quads` along the crease). Sampling keeps rounding >= 2 cells for topology, the snap uses the hard field.
LOD chain on snapped positions, falling back to the smooth net if no closed LOD0 (level valid only above 0.5x the
net's volume). `RockMesh::split_creases` (angle-weighted smoothing groups) on every LOD. Resolution scales as
res x max(1, size/2 m)^0.5, capped 128. Rock lab: 9 columns 22 m apart, keys N crease, Q edge snap, cameras
`eroded`, `sharp`, `sharp_near`. Measured (release, 24 seeds; ms/rock, LOD0 tris, wrong-way LOD0 tris / total):
block 2 m 25.9 ms 428 (0); talus 2 m 29.8 ms 117 (9 %); jagged 2 m 59.6 ms 92, 12 m 542 ms 705 (7 %); scoria 2 m
64.6 ms 1687 (0); slab 2 m 15.6 ms 109 (1 %); columnar 2 m 364 ms 24 141 tris = broken budget (its snapped LOD0 never
closes, falls back to the smooth chain; 3.1 s at 12 m).

**RG-3 (2026-11-22).** `superfargoth/crates/sf_rockgen_bevy` (`lib.rs` plugin, mesh conversion, LOD ranges, viewer
classes, tunable readers; `material.rs`; `shaders/rock.wesl`; `tunables.rs`; `tests.rs`, `shader_tests.rs`).
- **Mesh:** `rock_mesh` maps `sf_rockgen`'s frame (+Z short axis) to Bevy's +Y up, `(x, y, z) → (x, z, -y)` (a
  rotation: winding kept), positions + normals + `u32` indices, no UVs or tangents.
- **LOD:** one entity per LOD under the rock's root, each with Bevy's `VisibilityRange` (Bevy 0.20
  `bevy_camera::visibility::range`: per-entity distance ranges with dithered crossfade, the far field's mechanism):
  LOD k from `size × lod1_distance × growth^(k-1)` (8 sizes, ×2), crossfade ±`fade` (10 %), last LOD open-ended.
  Tunables `rockgen.lod.*` (Respawn).
- **Material:** `RockMaterial = ExtendedMaterial<StandardMaterial, RockExt>`, forward only, non-bindless (a field
  uses texture sets × LODs materials). Fragment: Bevy's `pbr_input_from_standard_material`, then base colour, normal
  (whiteout-blended, XY rebuilt so RGB and BC5 maps both work), roughness/metallic (Bevy ORM channels G/B) and
  occlusion (R, only for a packed ORM) from a world-axis triplanar of the texture set with Mikkelsen hex-tiling per
  projection; prepass and shadows are Bevy's (geometric normal). Projection origin is the rock's translation plus a
  `MeshTag` offset, so the texture is fixed on the rock under the floating origin; the adapter passes the terrain's
  land-texture tiling (texel density matches the ground). Live tunables `rockgen.material.*` (blend exponent 6, hex
  on, rotation 1, contrast 7, scale 1: the terrain's values). **Scaffold:** `surface.wesl`'s triplanar/hex functions
  read the terrain's arrays and parameter slots and could not be imported unedited while `wg-lodfix` edits that file,
  so `rock.wesl` carries a named minimal copy ("rock triplanar scaffold", non-periodic hex grid); RG-3b removes it.
- **Textures** come from the adapter as `RockTextures` (the game builds them with the statics' path,
  `cooked_material_with` over the bake's companions: a texture no static references may have albedo only; RG-A1 routes
  rock layers through the terrain's companion conversion).
- **Tunables live:** `RockgenPlugin` registers `sf_rockgen`'s `rockgen.boulder.*` and its own lists in the store; the
  game's catalog (`mw_devtools_bevy::perf::tunables::CATALOG`) lists both. Viewer classes (`CLASSES`: block, platy,
  cobble, chipped) are the RG-1 test overrides, adapter data at RG-A1.
- **Viewer:** `bevyrrowind` `level:rock-lab` (scene `level-rock-lab`, `test-levels.md` "Rock lab"): 56 rocks generated
  on the async compute pool; keys X texture, Z class, R new seed, L LOD tint, M wireframe, I info.
- Tests: `rock_mesh_conversion_is_sane` (counts, indices unchanged, unit normals, no NaN, rotation, Y short),
  `lod_ranges_chain`, `tunable_defaults_are_boulder_defaults`, `rock_fragment_compiles_for_every_def_set` (wesl link +
  naga validate, 4 def sets incl. the dither and the game's view features), `uniform_and_flags_match_rust`.

**RG-4b (2026-11-22, rockgen-col).** `edge_snap` default false (owner: rounded edges; snap = the lab's Q arm), which
exposed columnar's LOD0 not closing. Cause (measured): every meshopt level pinched (2-54 doubly used directed edges) so
LOD0 fell back to the raw net (24-160 k tris), from sealed interior bubbles along shared column faces, sub-cell slits
where per-piece rounding of shared floor/chip planes disagreed, and coincident sheets at concave steps. Fix (all
`sf_rockgen`): `Solid::with_cap` (shared planes smooth-maxed once over the union), `mesher::fill_cavities` (flood fill of
the noise band, only for solids with shared faces), `mesher::unpinch` (non-manifold split: radial face pairing + one
vertex copy per fan, as CGAL PMP `duplicate_non_manifold_vertices`; retry schedule + raw-net fallback kept behind),
`separate_duplicates` on the unsnapped path, two exact speed-ups (ambiguity re-passes only near flips, union early-out).
Columnar 2 m 388 -> 97 ms, LOD0/1/2/3 26803/20340/11123/762 -> 1360/679/338/168; 12 m 3013 -> 727 ms, 162 k -> 4267
LOD0; snap arm closes now (805 at 2 m). 17 of 384 other rocks changed (pinched levels now at the nominal ratio, e.g.
scoria LOD3 306 -> 224); same error bound. Left: columnar interior stays near zero along shared faces (block skip
can't prune; a per-shared-face union term would make the cavity fill unnecessary).

**RG-2 (2026-11-22, rockgen-rg2).** `sf_rockgen::library` (`build_library(params, &LibrarySettings, seed) -> Library {
key, size_m, variants }`, `Library::pick(placement_seed)`, `scale_for(size_m)`, `content_hash`; `Variant { seed, lods,
hull }`; `Hull { points, triangles, error_m }` plain arrays, `Hull::to_parry()` -> `ConvexPolyhedron` behind feature
`collision` = optional `parry3d`). Variant seeds don't depend on the count (growing a library keeps earlier variants);
variants built at the class size, instances scaled; `library_key` covers every clamped param + count + hull settings
(a new `BoulderParams` field must join it to compile); parallel by index, equal to serial. **Hull from LOD0 points**
(the spec's "lowest LOD" would miss LOD0 vertices, every LOD is a subset of LOD0), reduced by a greedy quickhull
refinement (parry3d has no hull reducer) until every vertex is within `min(0.01 x size, 0.05 m)` or 255 vertices
(PhysX `PxConvexMeshDesc` vertex limit). The hull is inside the mesh's convex hull by at most `error_m`; over
concavities (pits, grooves) it is solid where the mesh is open, as any single-hull simple collision. Tunables
`rockgen.library.variants` 16 (8-32), `rockgen.collision.hull_tolerance` 0.01, `hull_tolerance_max_m` 0.05,
`hull_max_vertices` 255. Release, 16 variants at 2 m: library 19-185 ms per class (all 9: 0.67 s), hull 48-109
vertices (exact hull 89-249), 0.9-4.9 ms per hull, max error 20 mm.

**RG-2b (2026-11-22, rockgen-rg2b).** `sf_rockgen::bake` (feature `bake` = sf_bake + serde + postcard; crate stays
Bevy-free): step `rockgen.library`, subject = class name, all 9 `CLASSES`; `version()` = `library::LIBRARY_FORMAT`
(doc lists the generator files that must bump it; also inside `library_key`, so one bump re-keys everything). Planned
parameters: `library_key(params, settings, seed)` + whether hulls are built; reads nothing, so one class's param change
re-bakes only that class. Class seeds `class_seed(LIBRARY_SEED, name)`. Output = postcard `Library` (store-compressed,
as `lod.policy` / `nav.*`); `decode(&[u8])`. Registered by the game in `mw_boot_bevy/src/bake.rs`
(`bake_rock_libraries`, features `bake,collision`), after the pipeline catalog, before the manifest. Release: 9 x 16
variants 0.59 s cold (4142 KiB), 8.6 ms warm. **Gap:** the bake passes the tunables' defaults (no tunable store in the
bake yet), so a mod override of `rockgen.*` doesn't reach the baked library ("everything is a mod"): the placement
slice or a bake-tunables slice passes adapter/mod values through `LibraryStep::new`.

**RG-8a (2026-11-23, first try, owner ask "spires, volcanic towers, arches").** `sf_rockgen::formations`: presets
`FORMATIONS` (Guess numbers, as `CLASSES`) over three recipes, all unions of convex pieces in the boulder `Solid`, so
the mesher, LODs, crease normals and snap are shared: `Stack` (polygonal frustum bands between jittered boundaries,
hardness-recessed, optional caprock and wave notch, lean, twist, drift, foot flare; hoodoo, pinnacle, sea_stack),
`Neck` (the columnar cluster standing tall, tapered by shared cap planes; volcanic_neck) and `Arch` (half ring of
convex segments, radial cuts + outer tangent + inner chord, thinning to the crown, on flared legs; the spec's capsule
subtraction needs an SDF difference the `Solid` doesn't have). Pieces overlap 15 % (≥ 3 cells) so the union has no
seam. `generate_formation(f, mesh, size_m, seed)` takes the boulder params for meshing only. Also
`rockgen.boulder.aspect_spread` (octaves, default 1, in the library key): each seed scales both aspect ratios by its
own `2^(±spread)` on a separate stream. Shown in the rock lab's formation row. Open: recipes as adapter data (RG-A1),
placement rules, V-HACD or per-piece hulls for collision, an SDF difference for carved arches and caves (RG-7).
