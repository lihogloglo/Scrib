# Procedural Tamriel bundle: what transfers to `sf_worldgen` (WG-R3)

Research only, 2026-11-18. Source: the shared Python project "Procedural Tamriel" (bundle built 2026-10-19, ~2,900
files; read: README, `Documentation/CURRENT_STATUS.md`, `Documentation/guides/*`, `tools/terrain/*.md`, run
reports, configs and a few modules to pin parameters). Paths below are relative to the bundle. Ideas only, we
reimplement in Rust. `unverified` = the bundle itself says so.

Units: 1 GU = Morrowind unit; vertex spacing 128 GU (~1.8 m), cell 8,192 GU (117 m), 1 THU = 8 GU. Their whole
Tamriel field is 19,777 x 11,649 = 230 M vertices, the same count as our 4 m / 60 km level (225 M). That makes
their timings a rough reference for WG-3, though their machine and kernels are different.

## 0. What the project is, and the headline

It fills gaps between hand-made landmasses (TR, Skyrim, Cyrodiil "owners") inside a base `tamriel.esm` and writes
masterless ESMs (heights only; objects follow in Z). Its terrain work went through three generations: (1) a
full-world bake (multifractal + seam continuation + physical erosion), (2) a local border pipeline, (3) a release
default of **harmonic bridge + contour-normal clone + owner restore only**. Every richer seam-matching idea was
tried and rejected by the owner's eyes. Nothing beyond that default is visually accepted: the bundle repeats
"scalars never certify geometry" (`README.md`; `Documentation/CURRENT_STATUS.md` section 1). Cityforge,
buildings, interiors (about 40% of the files) are settlement/building generators: stamp libraries, parcel layout,
a 12-module interior engine, "built but not wired into a downstream consumer" (`CURRENT_STATUS.md`). Skipped.

## 1. Erosion

Files: `tools/terrain/2026-08-25_terrain_erosion_tools.md`, `Documentation/guides/full_terrain_pipeline.md`,
`configs/terrain_enhance_v1.json` (`multifractal_erosion.engine_overrides`), `configs/tamriel_reworked_v1.json`
(`erosion`).

- **Technique** (`src/procgen/terrain_erosion.py::erode_field_physical`): (a) one-time curvature "preweathering"
  (20 iterations, rate 0.32); (b) 16-24 cycles of sediment transport on a two-receiver (multiple-flow) router with
  priority-flooded routing copies, re-routed every 2 cycles, stream power E ~ A^m S^n (**m = 0.5, n = 1.0**, area
  reference 256 verts, K auto-calibrated so the p90 of a channel metric hits 0.25-0.35); entrainment 0.55,
  deposition 0.08, capacity scale 1,400 GU, caps 128 GU erosion / 96 GU deposition per cycle; (c) lateral
  widening from accumulation in three width bands (sigma 1.5 / 4 / 10 verts at area 32 / 512 / 4,096); (d)
  inertial droplets (1 M per crop; 27.8 M on the world: ~0.3 per processed land vertex, 128-step life, brush
  radius 6, inertia 0.42, evaporation 0.018); (e) final talus (drop 40-64 GU, a few iterations).
- **Erodibility by terrain class** (data, not code): mountain 1.0, valley 1.2, scarp 0.9, hills 0.75, coast 0.45,
  flat 0.35, plateau 0.2; hillslope diffusion 0.02. Two broad control fields (erodibility 0.35-1.5, runoff 0.4-1.6)
  modulate it. This is the same idea as our per-region recipes.
- **Fixed set:** void, underwater, shore band (3 verts), the outer 8 verts and anything outside the editable mask
  never move and are not receivers. Owner (hand-made) terrain **takes part as hydraulic context** (rivers flow
  through it) and is restored afterwards over an **8-vertex smootherstep collar**; `owner_blend_weight` and the
  unrestored `process_field` are cached so the collar can be re-tuned without re-eroding.
- **Runtime (world run, 2026-08-28, `full_terrain_pipeline.md`):** 90.7 M processed land vertices (43 M underwater,
  93 M void excluded). Preweathering 8 s, routing 37 s, **16 sediment cycles 1,026 s**, droplets (2.6 G steps)
  462 s, talus 2 s; **erosion total 1,536 s (1,561 s with I/O)**. Only filters, curvature, lateral spreading and
  talus ran on CUDA (PyTorch); receiver construction on Numba `prange`; **ordered flow accumulation, sediment
  propagation and droplets stayed serial**. A first attempt failed on RAM pressure. The signed-F noise bake was
  1,738 s for 230 M vertices on CPU (FastNoiseLite, row slabs of 512).
- **CPU vs CUDA:** not byte-identical after iterative routing and droplets: on 52.7 M vertices median diff 0 GU,
  p95 2.13 GU, p99 7.20 GU, max 452 GU, 95.4% equal after 8-GU quantisation.
- **Accepted/rejected:** the world run passed numeric checks (owner change 0 GU, generated non-finite 0, shape
  correct) but the **owner rejected it visually** (broken Skyrim region, flat terrain with straight seams, eastern
  TR bumps). One cause was a stale water/dry-floor mask reused after seam solving (13,759 to -2,120 GU seam change
  clamped to 12,135 GU: a false 14,255 GU uplift). The later release default turns erosion off. A noise+erosion
  "finish pass" stays an optional pass (97 s on the Sutch crop).
- **Transfers:** parameter set is a usable starting point for WG-2/WG-3 tunables (declare as `Guess`). Their
  serial accumulation/droplet kernels are the bottleneck at 230 M: our WG-3 needs a parallel accumulation (the
  GPU-parallel kind, not a serial topological order) and a determinism rule for droplets (CPU twin or integer
  atomics). Estimate (inference, not in the bundle): a GPU stream-power + pipe model at 225 M should be seconds to
  tens of seconds, not 26 minutes.

## 2. Structured / multifractal field and exemplar pull-back

Files: `tools/terrain/2026-08-25_heightmap_enhancement_tools.md`, `2026-08-25_exemplar_pullback_tools.md`,
`2026-08-25_levelset_continuation_tools.md`, `2026-08-25_descriptor_patch_tools.md`, `2026-08-25_frequency_band_diagnostic.md`,
`configs/terrain_enhance_v1.json`, `configs/prototype_curvature_patch_v3.json`, `tools/terrain/v3proto/*.py`.

- **Style measurement first** (`measure_owner_style.py`): RMS residuals at wavelengths 8/16/32 verts, local-relief
  percentiles, slope, orientation coherence, per class (mountain, hills, plateau, flat, coast; thresholds are
  percentile-based: plateau = elevation p75 + slope p50, mountain = elevation p90 + relief p75, coast = within
  768 GU of sea). Class weights for amplitude: 1.0 / 0.65 / 0.35 / 0.08 / 0.
- **Failed first attempt:** `enhance_heightmap.py` (isotropic fBm-like ridged noise) "discarded coherent residual
  structure, produced dense ridged noise, and lacked a dry-land floor" (deprecated). Structural prototypes
  (ridge graphs, cloud excursions, contour networks, massif chains) were compared; "cloud/contour hybrid" was the
  promising one, never accepted. Rule they kept: **noise organises (supports, spacing, curvature, quiet/active
  patches) but is never a visible fine-noise blanket**; the modulation field is diagnostic and never added to height.
- **Selected "candidate F"** (`generate_multifractal_world.py`, `src/procgen/borderpipe/noise.py`): hybrid
  multifractal (OpenSimplex2S, octave wavelengths 224/112/56/28 verts, hybrid offset 0.65, persistence 0.42-0.68),
  domain warp (wavelength 224, displacement 8-30 verts), angular Gabor (48) + cellular (112) mixes, **seven broad
  control fields** (amplitude, terrain type, erodibility, runoff, persistence, orientation, warp, wavelengths
  320-1,024 verts), signed relief amplitude 1,300 GU, **mean lift derived from clearance** so full signed amplitude
  never crosses the dry floor (soft floor 8 GU), shore fade 10 verts. Noise is sampled in **absolute world
  coordinates, row-slabbed**, with normalisation and filters global (so tiles agree).
- **Pitfall recorded:** the border noise stage samples the whole component window (14.5 M verts) and keeps a band;
  ~42 s almost all discarded (`terrain_border_region_pipeline.md`, "Recorded open engineering items" 1-2); noise
  runs 4x per run (120-150 s of 373 s).
- **Exemplar pull-back** (reflect the owner H8 detail across seam contacts, clean generated side to H24, 4-cell
  smootherstep taper) and **level-set continuation** (12 nested signed-distance fields) are marked prototypes;
  the level-set checkpoint "failed"; neither reached production. **Descriptor-patch** and its successor "v3.4.1
  curvature patch" (`v3proto`: patch database of owner-collar patches, 21 coarse px at downsample 4, stride 3,
  k-d tree of pooled height + gradient + Laplacian descriptors, K = 64, affine fit then elastic warp, vote with
  Hann windows, synthesise **curvature** then Poisson-solve to heights; two bands: shape sigma 8-48 verts, detail
  sigma 8; detail gain gated by slope 25-90 and curvature 2.5-12 so flat land stays near-harmonic) did reach the
  region pipeline as stage "v341" (69 s on the Sky_FA crop after a CUDA descriptor sampler, was 355 s) and
  authored the full-world Sutch ESM, but is the "largest single mover" (deepest cut 2,001 GU) and its visual
  acceptance is pending.
- **Transfers:** (a) the measure-the-source-then-drive-the-generator loop is exactly our WG-A2/WG-5 exemplar
  statistics; keep the three bands and per-class amplitudes as data. (b) Domain-warped controls + clearance-derived
  mean lift are good for WG-5 runtime detail (cheap, deterministic, GPU-friendly). (c) Patch matching is
  expensive and hand-tuned (40+ parameters) for a one-sided seam problem; do not adopt for WG-3, only revisit if
  WG-R1 picks exemplar-based runtime detail. (d) Float64 matters: a float32 descriptor sampler flipped the affine
  optimum (E_grid 4.37 vs 3.62); `grid_sample` takes (x, y), the reverse of (row, col), and a swap shifted the
  solution ~1,500 GU.

## 3. Seam continuation between authored landmasses (our anchor transition bands)

Files: `.opencode/runs/2026-09-18-standalone-heightmap-border-pipeline/2026-09-21_matching_algorithms_failures_handoff.md`
(best single document), `.opencode/runs/v4-structure-continuation/2026-09-22_v4_structure_handoff.md`,
`tools/terrain/2026-08-25_canyon_topology_diagnostic.md`, `2026-09-24_seam_breakup_stage.md`,
`configs/border_pipeline_default_v1.json`.

Problem there: hand-made heights disagree with the generated base by up to ~16,000 GU at borders. Goal: continue
valleys/ridges/plateaus across the seam. What happened, in order:

1. **Harmonic bridge** (Laplace solve over a write band, Dirichlet at the seam = owner heights and at the outer
   edge = base heights; adaptive blend width 4-16 cells, max grade 2,500 GU/cell, outer apron 1 cell; AMG-CG
   solver). Continuous, no character. The owner's verdict: "already lines up fine"; it is the bar and the
   permanent ablation arm.
2. **Template operators** (macro diffusion, plateau/scarp extension, traced canyon carving and ridge embossing by
   mirroring owner lines across the seam, hydro guard, detail re-add). Failed: 27 ridge contacts gave 6,491 wiggly
   centerline vertices, glassy 800 GU blobs with sharp rims, mirrored crests ran along the range, each operator's
   footprint printed as a hillshade line. Verdict: "mirroring random shit". **Deleted.**
3. **Transport v1** (`terrain_seam_transport.py`): base = harmonic solve pinned to owner *macro* (gaussian
   sigma 64); add owner relief sampled at the nearest seam vertex, pyramid-softened (sigma 256), times a smootherstep
   weight over D = 160 verts (2.5 cells). Seam-exact by construction, 145 s vs 547 s. Failed on real data: (i)
   the nearest-seam (EDT) anchor map jumps at medial-axis shocks, giving gradients up to 1,692 GU/vert and straight
   axis-aligned bands; (ii) C1 break: w'(0) = 0 plus a relief-free base gives a "super distinct strip" and
   plateaus that "smoothly descend"; (iii) normal extrusion makes parallel prismatic teeth; (iv) the dropped hydro
   guard dried 175,693 vertices (rivers narrowed to the seam) and flooded banks. Proposed (never built): harmonic
   (shock-free) anchor coordinates, slope transport `S + g*d*taper(d)`, progressive sampling.
4. **v4 structure continuation** (detect lines, trace and fit features, continue across, rasterise, blend-solve): kept
   the seam fade smootherstep(dist/20), border envelope clip at dist <= 32, normal heal, end taper (20 verts) for
   rounded valley ends, heading gate to stop parallel-hug carving. Never accepted ("their latest screenshots still
   show border harshness + dots"; a stage solve gave `nnz = 0` on its crop). The "do not revive warping" note:
   signed-distance reflection printed radial-spoke fans.
5. **Release default:** harmonic matching, owner restore, **smooth-normal clone** (smooth the *ownership contour*,
   never heights, to get a normal; sample each seam height once; paint it outward with overlapping 5-vertex strokes;
   reach 14-64 verts, +1 vertex per 64 GU of mismatch, bounded owner slope 32 GU/vert; smootherstep to exactly zero
   at the reach end), outer fade to exactly 0 at the collar edge (6 border cells), VHGT projection. 98.6 s
   whole-world, 2,184 cells edited, owners bit-exact. A "seam breakup" stage (low-frequency sine-selected reach
   plus dry-collar noise) exists to hide straight lines.
- **Region/seam tooling facts:** seam atlas of 531 edge clusters (416 continental), border-profile decay length
  fits, classification of clusters; component-wise windows with halos; Jacobi stage order (each stage reads the
  pre-stage snapshot); `merge_authorized` masks that abort on overlap; a shared vertex row between adjacent tiles
  must be composed by **partition-of-unity cross-fade** (last-writer-wins made a 1,171 GU hard seam); bump a
  composition version when tile semantics change so resume rebuilds.
- **Transfers (important):** their structure-matching failed because they were *fixing a finished generated
  field after the fact*. Our generator is ours: put the anchor in as a **boundary condition of the macro stage**
  (fixed heights *and* slope at the footprint edge, named rivers forced as channels through it), so valleys
  continue because the erosion routes through the anchor, not because a post pass mirrors lines. The harmonic +
  clone result remains the right **baseline for the transition band** and the right ablation arm. Use C1
  blending (match slope at the footprint edge, Hermite or gradient-domain), a shock-free distance (harmonic or
  smoothed-contour distance, not nearest-seam EDT), and a single outer fade (not one per operator).

## 4. Water and coast preservation

Files: `tools/terrain/2026-09-27_water_mask_preservation.md`, `terrain_border_region_pipeline.md` ("Resolved
defect", "Reverted").

- Origin: an unconstrained harmonic solve dropped a cell-straight strip below sea level. Rule (their "tenet"):
  **never create or destroy water**. Per-stage **source-relative wet/dry audit** after every stage (new-water and
  dried-water vertex counts must be 0), a shore-aware **dry floor rebuilt from the current field** (never reused
  after a solve), 8-vertex smootherstep **dry-shore fade** of edits inland from each source shoreline, whole-edge
  source profile kept where an edge contains water or disagrees with the neighbour. Wet *heights* may move; the
  wet *class* is preserved (1.30 M changed source-wet depths at v341, 0 new/dried). Clamping heights to sea level
  was proposed and **refused**. Guarantee is classification, not "artistic coastline correctness" (owner review
  pending).
- **Transfers:** a hard constraint list for WG-2/WG-8: coast and sea level fixed, named rivers forced (our spec
  already), per-stage wet/dry audit as a test. Their water is a binary class at 0 GU; ours needs lake/river
  surface heights (WG-A5).

## 5. Region and massif masks, region assignment

Files: `tools/terrain/2026-08-31_mountain_massif_mask.md`, `2026-09-01_region_massif_masks.md`,
`Documentation/guides/region_assignment_source_profiles.md`, `configs/region_assignment/*`, `configs/climate_bands/*`.

- Massif masks: sample the height at 512-GU tile centres, threshold elevation and slope (a floor for connectivity,
  a stricter core), keep the connected component, optional configured dilation as a "transition", used later as a
  scatter placement mask. Small, config-driven, analysis-only.
- Region assignment: a 75-region atlas of 6,782 exterior cells keyed by RGNN display name; ordered composition
  (later plugin wins, conflicts kept as provenance); a source-profile registry per region with
  `same_province_only` fallback (no borrowing another province's library); one extraction path serves released
  and in-development plugins. `map:R...` aliases are provenance only.
- **Transfers:** WG-A2 REGN to recipe table: keep "region = identity from source data, recipes are data, fallbacks
  explicit". Massif thresholds are a cheap way to derive an extra mask (volcano flank, highland) when REGN is too
  coarse. At 10x use metres, not tiles.

## 6. Scatter rules from source placements (our WG-N2)

Files: `Documentation/guides/world_scatter_v2.md`, `regional_scatter_target_sets.md`, `rock_occupancy_profiles.md`,
`cliff_seating_profiles.md`, `world_scatter_evidence_completeness.md`, `falkreath_scatter_generation.md`,
`region_mesh_analysis.md`; `tools/scatter/*.md`; `.opencode/runs/2026-09-01-world-scatter-universal-coverage/2026-09-01_full_region_sidecar_completion_report.md`.

- **Measure, then replay.** Per mesh and region: frequency, scale, yaw, z-offset, slope/elevation/water-distance
  context from the source placements; rotation bins by slope (0-8-16-24-32-45 deg), Z-only if a bin has no full
  rotation evidence. Elevation bands in GU: sea, coastal < 500, lowland < 2,000, highland < 8,000, mountain < 15,000.
- **Occupancy tables** (`src/procgen/rock_occupancy.py`): 128-GU control cells with features [slope, elevation,
  relief, curvature, texture]; sparse **terrain-conditioned Bernoulli tables** in persisted order with the global
  rate as fallback; support threshold; runtime scales per source (`min(1, p * scale)`), raw/effective probability and
  table kept in provenance. **Not wired** into the world generator yet ("Stage C").
- **Density targets:** occupied-cell p90 74.5 trees/cell, median 36.5 rocks/cell (Skyrim proving); per-cell quotas
  plus a continuous domain-warped world-space patch/"cloud" field (tree factor 0.35-1.65, mean 1) so density
  varies without per-group collapse; candidate order by exponential race with weight canopy score x taper
  (deterministic, low-score candidates stay possible).
- **Rocks:** size by real mesh bbox, never by name (>= 1,000 GU = cliff giant, sidecar route; smaller = ordinary
  route). Pose = source pose relative to its own slope frame (`R_relative = T_src^T * M_src`) replayed in the target
  frame, with four-corner terrain cover, burial limits (5%, hard cap 15%), road/water/overlap gates; failures give
  absence, never a relaxed placement. Census: 25 regions, 2,690 rock meshes, 128,594 placements; 22,580 cliff-class
  observations, 18,187 pose modes, only 4,067 eligible (11,470 unstable gap heading, 2,650 competing gaps).
- **Palettes** (`configs/climate_rock_palettes/*.json`, 45 files, e.g. `VEL_MOUNTAIN_01.json`): per climate area a
  list of mesh paths per `source_region` with `fit: primary`. Pure data. Climate bands in
  `configs/climate_bands/`.
- **Failures/pitfalls they documented** (`world_scatter_evidence_completeness.md`): "full evidence complete" was
  retracted: TR `x\` folder misclassified as exterior gave false zero-rock regions (Velothi, Shipal-Shin); a merger
  read only the first 4,000 refs; a terrain-composition list omitted one plugin (9 LAND cells lost, 509 rocks);
  a manual window list dropped half the authority cells (Othreleth 37 to 646 rocks); self-reconciliation checked
  products against the same incomplete input; schema drift left 22 of 25 profiles stale; the first mesh-swap driver
  was rejected. Source terrain differs from the generated one (median 1,761 GU on the Kreathi giants): measure
  placements on the **source's own terrain**, replay on the target.
- **Transfers:** direct template for WG-N2 and for the WG-A1 census. At 10x we also need what they never had:
  *amount* scales with area (per-cell tables become per-m2 density), and HLOD/impostors (their output is plain
  references). Fit tables per region at bake from the source's placements; the Bernoulli/table form is cheap on GPU.

## 7. Hand-made vs natural classification

`src/procgen/assets.py::_classify`, `src/procgen/scatter_analysis.py` (categories, `SCATTER_CATEGORIES`).

Vocabulary: exterior, interior, door, terrain, flora, rocks, clutter, other. Rules: mesh **folder** first (`x` =
exterior construction unless an unambiguous natural-rock filename hint; `i` interior; `d` doors; `tdg` shared
terrain; `grass`, `env` flora; `b` body parts excluded; mixed folders `f`, `c`, `o`, `m` classified by documented
word lists; unknown goes to `other`, never guessed into construction). Scatter content = flora + rocks + terrain
categories; everything else is a structure/anchor candidate. This matches our adapter rule (nature = `flora_*`,
`terrain_rock_*`); their lesson is that **TR `x\` meshes mix structures and landscape rocks**, so path class alone
misfires (the Velothi false-zero), and size by bbox is the safer second signal. Put the ambiguous classes into the
WG-A1 census output, as already planned.

## 8. Groundcover

`Documentation/guides/groundcover_generator.md`. Generator: per terrain tile, a section of an FGM INI chosen by
exact assignment region, unique texture, climate fallback, default, none (fallback only if that texture really has
a section), density per tile, exclusions (roads, bridge capsules, static scatter). Shipped coverage is Western
Skyrim + Falkreath only; a refr-numbering bug was fixed the same day but **whether the shipped ESP predates the fix
is `unverified`**. Transfers: WG-N3 hook only: texture/material-field to density mapping is the same as our
material fields; no GPU lessons (OpenMW groundcover plugin, CPU authored).

## 9. Roads and the router failure

Files: `Documentation/guides/road_search_rebuild.md`, `terrain_aware_roads.md`, `tamriel_road_centerlines.md`,
`.opencode/runs/road-search-rebuild/2026-09-11_road_network_state_and_docket.md`, `tools/roads/*.md`.

- Roads are **not generated**: they are skeletonised from the source `road network.png` layer, then repaired,
  aligned, gap-scanned (1,653 dead ends, 321 LINK / 293 JUNC / 581 LOOP / 272 VETO among 1,471 pairs), bridged
  (9 assemblies).
- Router: heading-aware A* on a 1,024-2,048 GU lattice, samples every 128 GU, grade contract 45 deg hard / 30-35
  soft, search ceiling 120 s (pilots < 3 s), constrained refinement (chord shortcuts, vertex relaxation,
  Catmull-Rom for drawing, audit on the polyline). Valley-side traverses stayed at ~38-40 deg whatever the
  lattice (p95 38.6 to 37.8 at 512): "terrain-dictated floor". The "exact grade-first lexicographic router was
  rejected at scale" (`CURRENT_STATUS.md`); **the bundle does not say why** (the docket states no cause);
  `unverified`. The docket's four tasks (connectors, border alignment, slope re-alignment, final smoothing +
  texture paint) are unstarted; "roadbed terrain deformation remains the diagnosed target" (cut/fill: half-width
  512 GU, falloff 1,024, max cut 128, max fill 64, 8-pass gaussian profile smoothing).
- **Transfers:** nothing for WG-1..8. When roads come ("later, not planned"): treat them as terrain-conforming
  strokes that carve a roadbed (cut/fill caps), not as paths over fixed terrain, and let the router pay for cut
  volume instead of enforcing a hard grade.

## 10. Other pitfalls worth keeping

- Format limits shaped their result: Morrowind VHGT stores signed byte deltas per 65-point row, so a "projection"
  stage altered 695-1,000s of vertices and left unresolvable shared corners (2,664 GU). We have no such limit.
- 16 x 16 VTEX tiles per cell: hard-square edges, singleton off-textures, 2x2 diagonals (open defect, replan
  `unverified`). Our splat should come from continuous material fields.
- Coordinate-origin bug (`gx0`/`vx0` off by one cell) made earlier renders untrusted; read rows south-up vs
  north-up carefully; a stale viewer served old PNGs.
- Duplicate keys in hand-edited JSON silently took the last value; verify configs by loading.
- Interrupted run left no manifest so `--resume` rebuilt everything (a hash-cached stage graph avoids it).
- Their review rule that matches ours: the owner's eyes override all scalars; seam-centred strips (before / match /
  ablation) at structure sites are the validation, plus a flat control.

## 11. Ranked list: Take / Adapt / Avoid

**Take**
1. Constraint set and audits (WG-2, WG-3, WG-8, WG-A5): coast/sea level fixed; per-stage wet/dry audit with 0
   new/dried water as a test; dry floor rebuilt from the current field; no sea-level clamping; owner bit-exactness
   test inside footprints.
2. Process field + blend weight cached before restore (WG-1, WG-3, WG-8): anchor collar width is then a cheap
   re-run (their `owner_blend_weight`).
3. Harmonic bridge + contour-normal clone, with a harmonic-only ablation arm, as the **baseline transition band**
   (WG-8): one outer fade to exactly zero, no per-operator footprints.
4. Anchor terrain *takes part* in erosion as context and is restored over a smootherstep collar (WG-2/WG-3/WG-8).
5. Tile composition by partition-of-unity cross-fade; noise/filters sampled in absolute world coordinates with
   global normalisation (WG-3, WG-5); version bump on tile-semantics changes.
6. Source-measured occupancy tables + exponential-race ordering + terrain-frame pose replay + "failure = absence"
   (WG-N2); size by bbox, classify by path then verify (WG-A1).
7. Starting tunables for erosion and per-class erodibility / amplitude weights (WG-2, WG-3), as counted `Guess`.
8. Source-style statistics (RMS at 8/16/32 verts, relief percentiles, orientation coherence per class) as the
   exemplar input (WG-A2, WG-5).

**Adapt**
1. Seven control fields + hybrid multifractal + warp + clearance-derived mean lift (WG-5, recipes in WG-A2): good
   cheap runtime detail; replace "noise blanket" worries with their rule that noise only organises.
2. Anchor as boundary condition of the macro stage, with slope (C1) matching and forced rivers (WG-2 + WG-8):
   addresses exactly why their post-hoc continuation failed; use harmonic/smoothed distance, not nearest-seam EDT.
3. Their serial erosion pipeline to a parallel GPU form (WG-3); needs a CPU-vs-GPU tolerance and a determinism rule
   (their CUDA/CPU p99 7 GU on 52.7 M vertices shows iterative routing is not bit-stable).
4. Massif/region masks from thresholded height + connectivity (WG-A2) in metres.
5. Per-region library with explicit same-province fallback (WG-A2/WG-N2).
6. Roadbed cut/fill caps (later road slice).

**Avoid**
1. Template/mirror/trace structure continuation, nearest-seam EDT transport, per-operator gates and footprints
   (WG-8): all rejected with evidence.
2. Patch-matching/curvature-patch synthesis for the macro or anchor band (WG-3, WG-8): 40+ tuned parameters, one-sided,
   visual acceptance never reached. Revisit only for WG-5 if WG-R1 picks exemplar detail.
3. A full-world erosion that also touches finished anchors (their 1,561 s run, visually rejected); VHGT-style
   projection; texture class grids at tile scale (WG-N1).
4. Evidence "complete" claims from self-reconciliation (WG-A1, WG-N2): reconcile against the source plugins.
5. Grid A* with exact grade constraints as the first road design.
6. Judging on scalars (their mistake, our rule).

## 12. Claims the bundle marks `unverified` (carried here)

Texture replan; groundcover ESP vs the refr fix; Phase 7 TownBrief commit; public-core export after integration;
interior-to-settlement wiring; downstream use of the release package; the road router rejection cause (absent).
Visual acceptance of every terrain stage beyond harmonic + clone is open in the bundle itself.
