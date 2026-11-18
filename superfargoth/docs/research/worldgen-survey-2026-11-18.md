# World generation survey (WG-R1), 2026-11-18

Research only. Marks: **V** = I read the source this session; **U** = unverified (from memory, or a search snippet
without the primary page). Fetch limits: Nexus Mods returned HTTP 403 (so item 7 is thin), Schott's project page
404, the Grenier 2024 PDF 403, the Copernicus licence PDF was binary and unreadable by the fetch tool.

## 1. Bevy 0.20 and wgpu

- Bevy 0.20 was released 2026-10-08 (V: https://bevy.org/news/bevy-0-20/ by search snippet; the page itself not read).
  No terrain generation in Bevy (U for 0.20 specifically; consistent with the existing terrain spec).
- Our lockfile pins wgpu and naga 30.0.1 (V, `Cargo.lock`). Standalone wgpu compute needs no Bevy: `sf_worldgen_gpu`
  can depend on `wgpu` alone and must use the same version (a second wgpu would be a second compile).
- WGSL floating point is not bit-exact across GPUs: the spec lets compilers reassociate and fuse multiply-add
  (U, secondary source: https://luma.gl/docs/api-guide/shaders/gpu-floating-point-precision; I did not read the WGSL
  spec section). Integer arithmetic (u32/i32) is exact everywhere (V by definition). Plan for integer or
  fixed-point kernels where bit-exactness matters.

## 2. Rust crates

| Crate | Version / date | Licence | Notes | Verdict |
|---|---|---|---|---|
| `fastlem` | 0.1.4, 2024-01-26 (V crates.io API) | **MPL-2.0** (V) | Repo archived read-only 2025-06-05 (V github.com/TadaTeruki/fastlem). Delaunay-graph landscape evolution; drainage after Cordonnier 2016 (V); per-site erodibility and an `is_outlet` flag fixing height 0 (V). Uplift map not documented (V that the page does not mention it). No scale claims; the author warns the API may change in 0.1.x (V). | Do not adopt: archived, v0.1, MPL only allowed unmodified, graph (not grid) model, no fixed-height constraints except sea level. Reference only. |
| `noise` | 0.9.0, 2024-03-23 (V) | MIT/Apache-2.0 (V) | 3 M downloads. CPU only, f64/f32 maths; stalled since 2024. | Fine on CPU if pinned, but we want a GPU/CPU twin, so a small own hash + gradient noise is better. |
| `fastnoise-lite` | 1.1.1, 2024-03-05 (V) | MIT (V) | Port of FastNoiseLite, CPU. | Same. |
| `bevy_erosion_filter` | 0.3 targets Bevy 0.20 (V docs.rs) | **MPL-2.0** (V) | Analytic gully "erosion filter", no simulation, port of Rune Skovbo Johansen's filter (V). | Technique is the key finding (see 3). Crate is Bevy-bound and MPL: do not depend; reimplement from the paper-style description. |
| GPU erosion crates | none maintained found | n/a | Only app-level code: `kaylendog/kosmos` (wgpu hydraulic+thermal WGSL), `IoTone/burisu` (V via search snippets, licences not checked). | No crate; write kernels (small). |
| GeoTIFF/DEM readers | not researched in depth | U | Candidates `tiff` (MIT/Apache) for raw GeoTIFF, `gdal` bindings (needs the C library, avoid). Copernicus tiles are plain float32 GeoTIFF. | Use `tiff`; verify at WG-1. |

## 3. Papers

| Work | Produces | Scale, runtime | 60 km in minutes on a 4060? | Determinism |
|---|---|---|---|---|
| Cordonnier et al., CGF 35(2) 2016, doi 10.1111/cgf.12820 (V abstract) | Heights from a painted uplift map: stream graph + stream power, then blending landform kernels. | "Low cost" (V); numbers U. | Yes at macro scale (25 m, 5.8 M cells). Algorithm is O(n) per step on a sorted drainage tree. | CPU, ordered: deterministic if we fix iteration order. |
| Schott et al., ACM TOG 42(5) 2023, doi 10.1145/3592787 (V abstract); code H-Schott/StreamPowerErosion, MIT, C++/CMake (V) | Uplift-map authoring with an approximated drainage area and flow routing, interactive. | Interactive (V claim); timings U. | Yes, and it is the author-side interactive preview model. Approximation changes the result, so the *same* algorithm must run on both author and player side. | Port to Rust ourselves (MIT code, citable); deterministic if single-threaded order is fixed. |
| Mei et al. 2007 pipe model GPU erosion (U, standard reference) | Water height, flux, sediment on a grid; per-cell gather kernels. | Mid-detail; 100s of iterations. | Yes on GPU: 225 M cells (4 m) tiled with overlap is ~10^11 cell-iterations at 300 its, a few seconds to tens of seconds of bandwidth on a 4060 (U, my estimate). CPU would be too slow. | Gather-only kernels can be exact if done in fixed-point; float versions diverge. |
| Grenier et al., CGF 43(1) 2024, "real-time terrain enhancement" with spatially varying erosion patterns (V via search snippet; PDF blocked) | Procedural erosion patterns added on GPU in real time to a coarse terrain. | Real-time generation and rendering (V claim). | This is the runtime-detail (stage 7) precedent: stateless, evaluable per sample. Read it at WG-S. | Stateless per-sample: twin-able. |
| Runevision "Fast and Gorgeous Erosion Filter", 2026-03-30 (V blog.runevision.com); Shadertoy | Gully/ridge stripes aligned to the height gradient, per-cell random pivots, octaves, a fade mask; any height function with a gradient. | No timings in the post (V). "Every point evaluated in isolation", GPU friendly (V). | Fits stage 7 almost exactly. Determinism not discussed (V); per-cell hash pivots are deterministic if the hash is integer. | Integer hash + float maths: CPU/GPU twin to tolerance (< 1 cm), not bit-exact. Licence **MPL-2.0** (V): reimplement from the description, do not copy the code. |
| Terrain Diffusion, Goslin, arXiv 2512.08309, Dec 2025 (V abstract; V github.com/xandergos/terrain-diffusion) | Learned DEM generator, hierarchical diffusion, "InfiniteDiffusion": lazy, seed-consistent, O(1) random access. Two models: 30 m/px (7.7 km coarse) and 90 m/px (23 km coarse). Trained on ETOPO 30 arc-second and (for the 30 m model) Copernicus GLO-30 (V HF card). | Claims "9x orbital velocity on a consumer GPU" (V). CUDA PyTorch; CPU slow; no timings (V). | Not practical here: PyTorch/CUDA stack, 30 m output is coarser than our 2-4 m target, extent is Earth-like not our scenario, and it cannot take "uplift plus fixed anchors" as constraints (U). Weights and code MIT (V), but weights derive from GLO-30 data: provenance unclear. | Claims seed-consistency, but CUDA matmul across GPUs is not bit-exact (U). Reject for the bake; maybe a later optional import path. |
| Guérin et al. 2017 cGAN terrain authoring; Paris et al. 2019 terrain amplification; Galin et al. 2019 STAR "A Review of Digital Terrain Modeling" | Sketch-to-terrain GAN, amplification, field overview. | U | Read the STAR first as planned. | GANs: not bit-reproducible across hardware (U). |

Newer than Schott 2023 with the same goal (stream power at scale on GPU) I did not find beyond the above; the field's
2024-2026 energy is in learned generators (Terrain Diffusion) and analytic erosion filters (Grenier, runevision).

## 4. Industry (draft citations verified)

| Citation | What it actually is |
|---|---|
| Far Cry 5, "Procedural World Generation of Far Cry 5", Etienne Carrier, **GDC 2018**, Visual Arts (V gdcvault 1025557) | Set of procedural tools: biomes, terrain texturing, freshwater networks, cliff rocks. Houdini link: via SideFX talk page and the Wildlands article, not the GDC abstract (U for FC5). It is not terrain heights generation. |
| Ghost Recon Wildlands, "Terrain Tools and Technology", Guillaume Werle, **GDC 2017** (V title via search; contents via 80.lv article V) | In-house terrain editor, 50 cm precision plus tessellation; **World Machine on a render farm of ~80 PCs** for base terrain, **Houdini** for roads, rivers, carving, placement. Key quote: "procedural and manual don't mix very well"; procedural data must be locked before manual edits. This validates our anchors-first design and warns of re-run losing hand edits. |
| Horizon Zero Dawn, "GPU-Based Run-Time Procedural Placement", Jaap van Muijden, **GDC 2017** (V gdcvault + 80.lv) | Runtime GPU placement of environment content from artist logic graphs. Placement only, not terrain. Fits stage 8. |
| Outerra | Planet engine: real DEM plus GPU fractal refinement to centimetre scale, parametrised by elevation and land class (V via search snippets of Outerra/gamedeveloper pages, not primary). Microsoft Flight Simulator not researched (U). |
| Gaea / World Machine / Houdini | Gaea Erosion2 (Gaea 2.2): GPU-friendly hydraulic+thermal+gravity, "determinism on GPU improved" (V 80.lv/Gaea docs snippets); classic Erosion has a deterministic option forcing single-core (V). World Machine: erosion device, used at Ubisoft on a farm (V). Houdini HeightField Erode: hydro + thermal solvers (U, not researched). Note that even Gaea treats GPU determinism as something to *improve*: nobody promises cross-GPU bit-exactness. |
| Authored terrain plus procedural detail in shipped games | Wildlands (above): locked procedural base, hand edits on top. Details of others (RDR2, Cyberpunk) not verified; do not cite. |
| Curvature drop on a flat world | The technique is "world bending": vertex-shader drop by distance squared, physics stays flat (V search snippets, https://notslot.com/tutorials/2020/04/world-bending-effect, Blightbound GDC/Gamasutra article). Used by Animal Crossing, Subway Surfers (V snippet). No shipped realistic open-world with a physical-scale horizon drop found (U). Known problems: view-space bend breaks with camera rotation (use world-space about the camera position), VFX and effects look offset, shadows must use the same drop. |

## 5. DEM sources as exemplars

| Source | Licence | Ship statistics or patches in a mod? |
|---|---|---|
| Copernicus GLO-30 | Dedicated "Copernicus WorldDEM-30" licence (V search): free to the public; unmodified credit "(c) DLR e.V." ; modified credit "Produced using Copernicus WorldDEM-30 (c) DLR e.V."; must add a prescribed sentence to licence notices when distributing to the public. The 28 July 2026 notice restricts the *view service* only (V). I could not read the licence text itself (PDF unreadable), so redistribution of derived patches is U. | Probably yes with attribution (statistics and derived products are anticipated by the modified-data notice), but get the PDF read by a person before shipping patches. Shipping statistics (spectra, slope histograms) is safest. |
| SRTM (NASA/USGS) | US-government, public domain (U, not re-checked). 30 m 1 arc-second void-filled. | Yes (U). Void filling and noise are worse than GLO-30. |
| ETOPO 2022 (NOAA) | Public domain (U). 15-60 arc-second. | Too coarse for exemplars. |
| ALOS AW3D30 | JAXA terms, free with attribution, redistribution limits (U). | Avoid. |

Rule for the spec: ship **recipe parameters and statistics we measure** (power spectra, slope/curvature histograms per
landform), never raw DEM tiles; keep a bundled-patch option behind a licence check.

## 6. Determinism

- Industry practice found: Gaea offers a deterministic mode (single-core, V) and states GPU determinism was an
  improvement target (V). Ubisoft ran terrain on a render farm and shipped the result (V, a bake you ship, not a
  regenerate-per-player design). We differ: each player regenerates, so we need cross-machine agreement ourselves.
- WGSL does not guarantee bit-exact float results (U, see 1). Options, in order of preference:
  1. **CPU authority** for macro stage (5.8 M cells, seconds): scalar f32 or f64, fixed iteration order, no rayon
     reductions with variable order, no fast-math. Bit-exact on x86-64 (U: avoid `f32::mul_add` unless hardware-free
     fallback is also fused; avoid libm functions that differ, use own polynomials).
  2. **GPU mid stage with integer fixed-point kernels** (gather only, i32 heights in 1/1024 m, no atomics on floats,
     no float transcendentals), CPU reference twin with the same integers: bit-exact, so the hash check is a real
     equality. Cost: designing the pipe model in fixed point.
  3. Float GPU with quantisation to the stored u16 step (2 cm) and a hash of the quantised tile: LSB flips happen at
     rounding boundaries, so hashes would differ on some tiles. Not acceptable for co-op hashes.
- Runtime detail (< 2 m): float, a few millimetres of tolerance, never feeds saves; physics/nav use the CPU twin.

## 7. Morrowind and TES large-terrain projects

- "Distant Landscapes of Tamriel" (nexusmods.com/morrowind/mods/58155): **not fetched (HTTP 403)**; search returned no page for it.
  Nothing claimed.
- Related, from search snippets (U): TESAnnwyn extracts/imports Morrowind heightmaps (UESP "Heightmap" page);
  a Nexus mod supplies normal heightmaps for Tamriel Rebuilt for OpenMW; a placeholder mod fills the map edge around
  Tamriel Rebuilt; OpenMW has distant land and MGE XE builds distant terrain meshes from the mod list.
- None found that regenerates terrain procedurally at 10x. Prior art is heightmap import and distant LOD, not generation.
  Hand WG-R3 the Nexus pages (needs a browser).

## Findings that change the plan

1. `fastlem` is out (archived, MPL, v0.1). Write the macro stage ourselves (Cordonnier/Schott formulation, grid).
2. Runtime detail should be an analytic, stateless erosion filter (runevision/Grenier family), not exemplars or ML.
3. Determinism: CPU authority for macro; mid erosion on GPU only as fixed-point integer kernels with a CPU twin.
4. Terrain Diffusion exists with MIT code and weights, but is CUDA/PyTorch, 30-90 m, and trained on GLO-30: not for the bake.
5. Wildlands confirms "procedural and manual don't mix": lock the generated base, keep hand edits as a separate layer.

## (a) Proposed "Reuse check" replacement

- **Bevy 0.20 / wgpu:** no terrain generation in Bevy. Standalone wgpu 30 (our pinned version) compute is used by
  `sf_worldgen_gpu` with no Bevy dependency. WGSL floats are not bit-exact across GPUs, so exactness-critical kernels
  are integer/fixed-point.
- **Crates:** `fastlem` rejected (MPL-2.0, archived 2025-06, v0.1.4, graph-based, no uplift API); `bevy_erosion_filter`
  rejected (MPL-2.0, Bevy-bound; technique reused); `noise`/`fastnoise-lite` (MIT/Apache, stalled 2024) usable on the
  CPU side but we need a GPU twin, so a small own integer-hash gradient noise; no maintained GPU erosion crate;
  GeoTIFF via the `tiff` crate (to confirm).
- **Algorithms to implement (own code, cited):** stream power on a grid (Cordonnier 2016, Schott 2023 with MIT
  reference code), pipe-model hydraulic erosion (Mei 2007) and thermal erosion in fixed-point, an analytic
  gradient-aligned erosion filter for runtime detail (runevision 2026, Grenier 2024, reimplemented).
- **Learned amplification:** Terrain Diffusion (MIT code and weights, 30/90 m, CUDA) evaluated and not adopted; optional
  later import path.
- **Tools:** Gaea, World Machine, Houdini are import sources for intent heights, never dependencies.
- **Data:** Copernicus GLO-30 (own licence, attribution required, derived-data redistribution to confirm), SRTM and
  ETOPO (public domain, U). Ship measured statistics, not tiles.

## (b) Proposed "Industry comparison" rows

| Technique | Who | Us |
|---|---|---|
| Uplift + stream power for large form | Cordonnier 2016 (V); Schott 2023 (V), MIT reference code; Gaea/World Machine erosion | stage 2, own grid implementation, CPU authority |
| Hydraulic + thermal for mid detail | Mei 2007 (U); Gaea Erosion2 GPU, Houdini HeightField Erode (U) | stage 3, fixed-point wgpu kernels + CPU twin; vendors treat GPU determinism as an open issue |
| Authored content first, procedural between | Wildlands GDC 2017: World Machine farm + Houdini, "procedural and manual don't mix"; Far Cry 5 GDC 2018: procedural biome/water/cliff tools | anchors; generated base locked, hand edits a separate layer |
| Rule-based placement | Horizon Zero Dawn GDC 2017 (runtime GPU placement of objects, not terrain) | stage 8 |
| Runtime amplification of coarse base | Outerra (DEM + GPU fractals); Grenier 2024 real-time erosion patterns; runevision 2026 analytic filter | stage 7 analytic erosion filter, GPU + CPU twin |
| Learned amplification | Guérin 2017 cGAN (U); Terrain Diffusion 2025: CUDA, 30-90 m | rejected for bake; no determinism guarantee |
| Curvature | "World bending" vertex trick (Animal Crossing, Blightbound); planet engines use real spheres | flat gameplay, world-space drop about the camera |
| Field overview | Galin et al. 2019 STAR (U, to read) | read first |

## (c) Recommended answers to the open questions

1. **Determinism authority:** CPU for macro (stage 2). Mid stage (3) on GPU only as integer fixed-point gather kernels
   with an identical CPU reference; the product hash is over integers so it is a true equality. If a player GPU fails the
   spot check (reference tile compare at bake start), fall back to the CPU path (slow, but correct). Float GPU is allowed
   only for the runtime detail, which never feeds saves or nav. Reason: WGSL does not promise float agreement.
2. **Runtime detail:** analytic gradient-aligned erosion filter plus noise, evaluated per sample in a GPU kernel with a
   CPU twin, reimplemented clean-room from the published description (MPL code not copied). Exemplar statistics only
   tune its parameters. Learned amplification: no (stack, resolution, determinism, data provenance).
3. **Recipe operators:** data-driven over a closed registry of framework-shipped operators (each with GPU kernel, CPU
   twin and hash test); mods ship parameters, strokes, graphs and recipes, not code. New landform operators arrive with
   framework releases or a reviewed plugin API later. Reason: mods cannot ship GPU/CPU twin kernels safely, and every
   operator must be reproducible on every machine.
