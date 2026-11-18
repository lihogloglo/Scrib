# Open-source terrain generators survey (WG-R4), 2026-11-18

Scope: OSS terrain / landscape generators, any language, for **layout-driven, realistic, 50 km** terrain
(`superfargoth/docs/specs/worldgen.md`). Not repeated here (see `worldgen-survey-2026-11-18.md` sections 2-3):
fastlem, noise crates, bevy_erosion_filter, Cordonnier 2016, Schott 2023, Mei 2007, Terrain Diffusion, runevision.
Marks: **V** = read in the source/README/API this session (GitHub API for licence, last push, language), **U** =
search snippet or memory, unverified. Dates are the last push seen on 2026-11-18. Licence column is the GitHub SPDX
id unless noted. "GPL" projects can be *ported* (GPL-3.0-or-later target) only if they are GPL-3.0 (or "-or-later");
GPL-2.0-only cannot be mixed in.

## 1. Table

| Project | Produces / technique | Lang | Licence | Activity | Guide input? | Verdict |
|---|---|---|---|---|---|---|
| **Veloren `world/src/sim/erosion.rs`** github.com/veloren/veloren | A whole game world (chunk grid, default 1024^2 chunks) from a **Rust** port of FastScape: implicit stream power (O(n) pass over height-sorted nodes), **multi-receiver flow** (`get_multi_rec`, `mwrec`), lakes (Cordonnier 2016 connection), ADI hillslope diffusion (from fastscapelib-fortran), sediment, soil production, `rayon`. 100 steps x `erosion_quality`. Uplift = Worley noise + turbulence (V) | Rust | GPL-3.0 (V) | 2025-09 (file), repo active 2026-11 | uplift/erodibility per cell: yes (any closure `kf(posi)`, uplift array); sea outlets from map | **PORT** (best find). Same language, GPL-3.0, already does what stage 2 needs. Read it as the reference for the data layout, then rewrite for our fixed-point/determinism and anchors |
| **fastscapelib** github.com/fastscape-lem/fastscapelib | Library: D8 `single_flow_router`, **`multi_flow_router(slope_exp)`**, basin-graph + MST sink resolvers (`pflood`), implicit `spl_eroder` (Newton for n != 1, multi-flow supported with caveats), ADI diffusion, raster/trimesh/healpix grids (V: file list + headers) | C++ (Py bindings) | GPL-3.0 (V) | 2026-11 | uplift, K arrays: yes | **PORT** the algorithms (licence ok), **reference oracle** for our Rust output. Sink resolution (basin graph) is the part to copy: it replaces our depression filling |
| fastscapelib-fortran | Original FastScape: Braun and Willett 2013 implicit solver, MFD, diffusion, orographic options | Fortran | GPL-3.0 (V) | 2024-11 | yes | Reference oracle (numbers), already ported into Veloren and fastscapelib |
| fastscape (Python models) | xarray-simlab model layer on fastscapelib: uplift, orography, lithology | Python | BSD-3-Clause (V) | 2026-11 | yes | Reference oracle; quick prototyping of recipes on 1-5 km tests |
| **H-Schott/StreamPowerErosion** | Schott 2023 code: **GPU (GLSL compute) iterated stream power**, D8-or-MFD hybrid: area *advected* per step, MFD weights `slope^p_flow / sum` with `p_flow` tunable (1 = diffusive, large = D8), `p_sa`=0.8, `p_sl`=2, hillslope Laplacian + debris slope modes, per-cell uplift and per-cell k "noise" buffer (V shader) | C++/GLSL | MIT (V) | 2026-09 | uplift painting; ridge/river constraints only via uplift (V README) | **PORT** (MIT). Gives the author-side interactive preview; the `p_flow` MFD exponent is the cheap D8-artifact lever. Already in survey as "Schott"; new here: the MFD weighting and per-cell K buffer are explicit |
| **Landlab** github.com/landlab/landlab | The reference research toolkit: `FlowAccumulator` with D4/D8/**Dinf/MFD**, `DepressionFinder`, `StreamPowerEroder`, `FastscapeEroder`, `SPACE`, `LinearDiffuser`, `Lithology`, `OrographicPrecipitation`, `LateralEroder` (U for component list, from docs memory) | Python | MIT (V) | 2026-11-17 | uplift fields; layered lithology | **Reference oracle** (best documented behaviours of every term). Idea source: lithology + orographic rain to break uniformity |
| Badlands github.com/badlands-model/badlands | Basin + landscape evolution, SPL + diffusion + marine, climate/tectonic forcing | Python+Fortran | LGPL-3.0 (V) | 2026-05 | uplift, rain maps | Idea only (heavy, mesh-based) |
| goSPL github.com/Geodels/gospl | Global scalable paleo-LEM, **MFD** on unstructured spherical mesh, PETSc/MPI | Python | GPL-3.0 (V) | 2026-11 | uplift, climate | Idea only / oracle for MFD at scale |
| TerrainBento (MIT) / Tadpole (MIT, C, MIT Geomorph, 2026-11) / DAGGER (GPL-3.0, C++, 2025-07, Gailleton) / GraphFlood (GPL-3.0, 2024) | LEM model families; DAGGER = graph-based flow/topology library; GraphFlood = fast shallow-water flow approximation on terrain graphs for **river width / flood depth** (V descriptions) | Py/C/C++ | as listed (V) | active | n/a | Idea only. **GraphFlood** is worth a look for sizing river channels and lakes from flow (stage 5) |
| **OpenLEM** (Hergarten) hergarten.at/openlem; Zenodo 10.5281/zenodo.6794117 | LEM behind Hergarten and Pietrek 2023 (ESurf 11, 741): **channels and hillslopes self-organise** with a *topological* channel definition (a cell with one lower neighbour is a channel) + hillslope model with m = 0 equilibrium; D8; claims no obvious resolution dependence (V paper summary) | U (Fortran?) | U (page unreachable; Zenodo archive) | 2023 | uplift | **Idea only, important**: the cleanest published answer to "grid-dependent catchment spacing" on D8 (see section 3) |
| **erosiv/geotransport** + **soillib** github.com/erosiv | McDonald and Cordonnier, "Stochastic Geomorphological Transport", TOG 45(4) 2026 (V). Stochastic point (particle) transport with momentum conservation: **meanders, braids, fans, deltas, debris scars**, resolution invariant across 768^2-1536^2 grids, 500 ky runs (V). geotransport = minimal reference (CUDA kernel), soillib = full toolbox (GeoTIFF, GPU) | C++/CUDA, Py | MIT (geotransport, V); LGPL-3.0 (soillib, V) | 2026-07 / 2026-10 | sources/forces; paper does not mention uplift (V) | **Reference oracle** (CUDA, not deterministic: unordered atomics, V via HighMap doc). **Idea**: only open method giving real meanders and braids. Too slow and non-deterministic for the bake; use as a mid-stage research option |
| SimpleHydrology / SoilMachine (weigert) | Particle hydrology: discharge + momentum maps, "river meandering behavior" (README claim, V); layered soil model | C++ | **README says MIT, repo has no LICENSE file** (V): treat as unclear | 2023 | none | Idea only; superseded by soillib/geotransport |
| **Infinigen terrain** github.com/princeton-vl/infinigen | Photoreal terrain for scenes: SDF "elements" (caves, voronoi rocks, land tiles) + **SoilMachine erosion** (`land_process/erosion.py`: N = 512 then 2048 grid, 1e4 then 5e5 iterations, tile-size aware) + snowfall (V) | Py/C++/CUDA | BSD-3-Clause (V) | 2026-11-11 | procedural params only | Idea only. Confirms: the industry-grade OSS pipeline is coarse-to-fine particle erosion on tiles, then detail. Not 50 km |
| TerraForge3D Jaysmito101 | Node editor, GPU hydraulic erosion (U) | C++ | MIT (V) | 2026-10 | node graph | Skip (hobby particle erosion) |
| Sebastian Lague Hydraulic-Erosion | Particle droplets, 1 kernel; the usual "blobs and pits" look at scale | C# | MIT (V) | 2024 | none | Skip |
| dandrino/terrain-erosion-3-ways | Sim, GAN, and **river-network-first** (Poisson-disc nodes + Delaunay specifically to avoid horizontal/vertical grid artefacts, rivers grown upstream from the sea, then terrain from the network) (V README) | Py | MIT (V) | 2022 | rivers are the generator | Idea only: **non-grid node set for drainage** (same lesson as fastlem/Cordonnier) |
| **HighMap** + **Hesiod** github.com/otto-link | Terrain library behind a node app. Hydrology: `flow_accumulation_d8`, **`_dinf`** (talus-referenced MFD exponent, tiled), **`_stochastic`** (geotransport port), **`_dinf_perturbed`** (average of noise-perturbed D-inf runs), `carve_riverbed`, `flow_fixing` (drainage-basin), depression filling, flooding, coastal/aeolian/strata erosion (V headers) | C++ (+OpenCL) | **HighMap: LICENSE file LGPL-2.1, README says GPL-3.0 (V: contradictory); Hesiod GPL-3.0 (V)** | 2026-11-17 | heightmaps, masks | Idea only (licence unclear; read, then reimplement). Best catalogue of D8 fixes in one place |
| **Undiscovered Worlds** JonathanCRH (+ fegennari fork) | Map generator: world + regional maps from **hand-drawn terrain / mountain-range input**, then climate and rivers (README snippets, U) | C++ | GPL-3.0 (V) | 2024-09 | **yes (painted)** | Idea only (layout import UX); not an erosion model (U) |
| mapgen4 (redblobgames) | Paint coarse elevation on a Delaunay mesh, rainfall + rivers, generates relief (U for algorithm) | TS | Apache-2.0 (V) | 2026-03 | **yes (painted strokes)** | Idea only: UX of painting a layout |
| mewo2/terrain; Azgaar Fantasy Map Generator | Voronoi/mesh heightfields; mewo2 erodes by slope x sqrt(area) (U); Azgaar is not drainage based | JS/TS | NOASSERTION in both (V) = unclear | 2016 / 2026-11 | none / stylised | Skip (drainage only toy-level) |
| **TDN** TDNResearch (AAAI-24); **Arches-Team Interactive Authoring of Terrain using Diffusion Models** (CGF 2023); **TerraFusion**; **MESA**; Talus; nanoxas/sketch-to-terrain | Learned **sketch-to-terrain** (rivers, ridges, basins, peaks) | Python | TDN: **no licence, no weights** (V); Arches: MIT code, weights on Google Drive (V); TerraFusion: NOASSERTION (V); Talus: Apache-2.0 (V) game-map model, not DEM; sketch-to-terrain: NOASSERTION (V) | 2021-2026 | **yes (sketch)** | Skip for the bake (weights from DEM data, CUDA, 256-1024 px tiles, no determinism). **Reference oracle only**: Arches sketch encoding (ridge/river/peak/basin channels) is the cleanest sketch vocabulary to copy for guide strokes |
| Paris et al. SIGGRAPH Asia 2023 `aparis69/Meandering-rivers` | **Vector** meander simulation: curvature-based migration, cutoffs, oxbows, avulsions on a low-res river graph over a terrain; runs "at interactive rates" (V page) | C++ | MIT (V; the paper page states none) | 2024-02 | **yes (river graph + trajectory constraints)** | **PORT (idea + MIT code)** as a stage-5 operator: run on the river centrelines extracted from stage 2 |
| FirstPersonTerrainSketch (ftasse) | Sketch-based terrain editor | C++ | **GPL-2.0 only** (V), 2014 | dead | sketch | Skip (licence and age) |
| Terra (brooteskan) / nerthus (danemadsen) / bevy_generative | Rust node-based terrain apps (layers, erosion) | Rust | MIT / MIT / Apache-2.0 (V) | 2025-2026 | paint / nodes | Skip (young, shallow erosion); Rust app patterns only |
| Barnes 2019 GPU LEM (r-barnes/Barnes2019-Landscape) | Parallel LEM + graph processing on GPU (accompanying "Accelerating a LEM with parallelism"), D8 only (V snippet) | C++/CUDA | **none** (V) | 2023 | uplift | Idea only: parallel graph algorithms for a later GPU macro stage |

## 2. Ranked shortlist for layout-driven, realistic, 50 km terrain

1. **Veloren `erosion.rs` (GPL-3.0, Rust): port the macro solver.** It already contains what spec stage 2 asks for
   in our language: height-sorted implicit stream power, **multi-receiver flow with weights**, lakes via
   Cordonnier's connection, ADI diffusion, per-cell erodibility closure and uplift array. Take: the data layout
   (`mrec`, `mstack`, `mwrec`), the order of passes, and the uplift recipe (Worley + turbulence). Change: fixed
   point or fixed summation order for determinism, anchor boundary conditions, tiled/streaming memory for 5.8 M
   cells (theirs is 1 M chunks).
2. **fastscapelib (GPL-3.0) as algorithm source and oracle.** Take: basin-graph + MST sink resolver (replaces
   priority-flood), the Newton step for n != 1, `multi_flow_router(slope_exp)`. Run the Python models offline on
   a 2000 x 2000 test to produce the reference heights our Rust output is compared against.
3. **Schott `StreamPowerErosion` (MIT): the author-side preview and the MFD exponent.** Take: the advected-area GPU
   loop with `p_flow` (1 = diffusive MFD, large = D8) and the per-cell K buffer (K variation is the main lever
   against uniform grooves; see section 3). Same algorithm on both sides, as the survey already requires.
4. **Hergarten and Pietrek 2023 (+ OpenLEM) as the *fix* for parallel channels, not as code.** Take: the
   topological channel definition (one lower neighbour = channel) with a straight or convex hillslope law, which
   removes the grid-spaced catchment dependence on plain D8. One paper, Zenodo code, testable in a day against
   our current macro stage.
5. **Paris 2023 meanders (MIT) + HighMap's flow catalogue (idea only) for the river layer.** Take: vector
   meander migration on the centrelines from stage 2 (a real meandering river on lowlands, which stream power
   alone never makes) and, from HighMap, the D-inf perturbed / stochastic accumulation as cheap alternatives to
   test against the D8 look. Reimplement HighMap ideas (licence unclear); port Paris code (MIT).

Runner-up: soillib/geotransport (braids and fans, CUDA, non-deterministic): an optional author-side look-dev tool,
never the bake.

## 3. What this changes in the current approach

- **"Parallel evenly spaced grooves" is partly physics, not only D8.** Uniform uplift + uniform erodibility on a
  smooth start self-organises into evenly spaced valleys whose spacing is set by the diffusion/incision ratio
  (Perron, Kirchner, Dietrich, Nature 2009; U, from memory). So the fix is a set, not one switch: (a) **per-cell K
  and uplift heterogeneity** from the layout (rock type, region, stochastic lithology: Veloren uses Worley uplift,
  Landlab has `Lithology`), (b) **MFD or topological channelisation** instead of pure D8 (Schott `p_flow`,
  fastscapelib multi flow, Hergarten), (c) **non-grid or perturbed drainage** (Poisson-Delaunay nodes in
  dandrino/fastlem/Cordonnier; HighMap's noise-perturbed D-inf), (d) a **random, not regular, start** (noise
  initial height, as "from a low start" already implies) and a finite-time run rather than strict steady state
  (steady state is the most regular state).
- **The layout map should enter as uplift plus K fields, not as a target height**, which matches the spec; the
  sketch/learned tools (TDN, Arches) treat strokes as a conditioning image. Borrow their stroke vocabulary
  (ridge, river, peak, basin) for the guide strokes in the spec, not their models.
- **Rivers need their own stage.** Stream power gives dendritic valleys; it never gives meanders, braids or fans.
  Only Paris 2023 (vector, MIT) and geotransport (particles) do, so plan meanders as an operator on extracted
  centrelines (spec stage 5), and keep braids/fans as optional landforms.
- **Use Veloren as the implementation reference before writing the Rust solver**; it removes the language
  translation step that C++/Fortran sources would add.
- Open items that need a human or a longer read: Hergarten's OpenLEM licence and language (page unreachable),
  HighMap's real licence (LGPL-2.1 file vs GPL-3.0 README), the Lochner repo link
  (`github.com/terrain-diffusion/terrain-diffusion` was 404; the working one is Arches-Team, MIT), and the
  Perche StyleDEM add-on (GitLab, licence not checked).
