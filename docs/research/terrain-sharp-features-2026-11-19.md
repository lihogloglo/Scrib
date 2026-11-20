# Sharp terrain: cliffs, escarpments, boulders, rock outcrops (research, 2026-11-19)

Question (owner): the 10x Vvardenfell looks blobby; an erosion filter gives "blobs with grooves". How do shipped games
and current research get sharp edges, cliffs, bouldery ground, outcrops (Etna, Teide, Iceland lava, Scottish highlands)
on a ~50 x 50 km island, and what do we build? Research only; no code or builds. Marks: **V** = a source was opened or
a search result states it; **S** = secondary / from memory of talks and papers, to verify before use; **G** = my guess.
Web search in this session was thin (many hits were forum or listing pages); where a talk's content could not be
checked it is marked S. Repo context read: `superfargoth/docs/specs/worldgen.md` (Pipeline, Industry comparison),
`docs/ledger/worldgen.md`, the TD-2 research note.

## 0. Diagnosis: why it is blobby, and where sharpness really lives

1. **Uniform scaling preserves slope.** Morrowind's heightmap is smooth and gently sloped; scaling x and y by the same
   10 (18.3 m samples) keeps every slope angle, so the shapes stay blobs, only 10x bigger. Sharp forms need *new
   information* in the band **20 m to ~2 km**, which neither the guide (18 m samples, smooth) nor the runtime filter
   (160 m down to 1.25 m, one gradient-aligned gully operator) provides. Bethesda faked this very problem with the
   `terrain_rock_*` cliff meshes, which our adapter drops (`worldgen-adapter.md:67`, `classify.rs:113`).
2. **Erosion makes one kind of shape.** Stream power plus gully filters give V/U channels and dendritic ridges: soft,
   convex-concave. Ledger WG-M: ours is 3-11x too rough at 100 m-1 km and still "blobs with grooves" to the owner.
   Roughness amount is not the problem; **roughness type** is. Real sharp terrain has a heavy-tailed slope
   distribution: mostly gentle ground, with discrete steep faces (10-50 m flow fronts, 100-1000 m caldera walls),
   benches between them, and block-scale (0.3-10 m) clutter on top.
3. **Where Etna/Teide/Iceland look sharp (S, geology and ledger notes):** (a) collapse walls and scarps (Las Canadas
   wall about 500 m, Valle del Bove walls up to about 1 km); (b) dikes and lava-flow margins (aa fronts 5-30 m high with
   35-45 degree rubble faces, channel levees); (c) cinder cone rims; (d) the m-scale surface (aa clinker, pahoehoe
   plates, block lava, boulder fields); (e) sea cliffs. Km-scale flanks of Etna/Teide are *smooth* (ledger: real has
   less mid-detail than our erosion). So the fix is landform strokes plus m-scale block texture, **not more
   dissection**. This agrees with the owner's rejection of the over-dissected "motherboard" looks.

What a heightfield cannot do: overhangs, arches, caves/lava tubes, truly vertical faces (at 18 m spacing a 100 m cliff
is about 80 degrees, but features under about 18 m wide alias), undercut cap-rock. Those need mesh overlays (steps 4-5).

## 1. Industry: how shipped games make cliffs and rock

| Game | Technique | Mark |
|---|---|---|
| Horizon Zero Dawn / FW (Decima) | World = collection of 2D maps (all generated, paintable, streamed); all nature (500+ asset types, 100k+ objects in scene, about 250 us avg GPU busy) placed by GPU rules in density, generate, placement passes. Terrain stays a heightfield with sculpted cliffs; rocks are placed meshes. GDC 2017, J. van Muijden. | V ([slides](https://www.guerrilla-games.com/media/News/Files/GDC2017_VanMuijden_GPUBasedProceduralPlacementInHorizonZeroDawn.pdf), [GDCVault](https://www.gdcvault.com/play/1024120/GPU-Based-Run-Time-Procedural)); cliff authoring S |
| Far Cry 5 | GDC 2018 "Procedural World Generation of Far Cry 5": procedural biomes, water, texturing and **cliff rocks**; heights stay authored. | V (session summary, [GDCVault](https://gdcvault.com/play/1025557/Procedural-World-Generation-of-Far)); internals S |
| Ghost Recon Wildlands | World Machine base on a PC farm, Houdini for roads/rivers, "procedural and manual don't mix" (in our spec). | V in spec |
| Skyrim / Fallout 4 / Starfield | Heightfield plus large static **cliff/rock meshes** snapped to steep terrain, bespoke LOD; Starfield adds procedural terrain and rock scatter. | S (modding common knowledge; CK docs not opened) |
| Morrowind | Coarse heightmap plus hundreds of `terrain_rock_*` NIFs (per region) placed as references to fake cliffs. | V meshes exist and are dropped by our adapter; placement S |
| UE5 | Landscape + PCG framework (graph scatter on slope/curvature/biome) + Nanite meshes for cliffs (Megascans kit) + **Nanite tessellation/displacement** (5.4+, experimental). Reported buggy on landscapes: displacement direction wrong with non-1 Z scale (UE-367789, still in 5.8), collision ignores displacement. | V for the bugs ([forum](https://forums.unrealengine.com/t/nanite-tessellation-pushes-displacement-in-the-wrong-direction-on-landscapes-with-z-scale-ue-367789-still-in-5-8/2845651)); PCG/Megascans S |
| RDR2, Cyberpunk, Ghost of Tsushima, Death Stranding, MSFS | No usable public terrain talk found. Common knowledge (S): heightfield/mesh terrain, slope-triplanar rock materials, large placed rock meshes for cliffs; MSFS uses photogrammetry/Bing data plus classified procedural scatter. Death Stranding rocks carry nav-cost areas (V, [GameDeveloper](https://gamedeveloper.com/programming/in-i-death-stranding-i-navigating-the-world-was-harder-for-npcs-than-players)). | S |

Common pattern (S, consistent with the verified rows): **the heightfield carries mass and walkable surface; sharpness on
it comes from (1) sculpted or tool-made steep faces, (2) slope-driven rock materials, (3) placed meshes for anything
vertical or bouldery.** Nobody ships a pure-erosion heightfield as the final look. Runtime displacement (Nanite
tessellation) is the newest tool and still unreliable on landscapes.

Material tricks used everywhere (S, standard): slope-selected **triplanar** rock (top-down UVs stretch on faces),
**height-blended** layers (the layer's own height map drives the transition: crisp rock-through-soil edges), near-camera
detail normal/POM, macro variation breakup. Cheap, and they read as "sharp" far above their cost.

## 2. Heightfield operators that make sharp forms

Tool vocabulary (Gaea, World Machine, Houdini HeightField; S, product knowledge, not re-opened): **Terrace/Stratify**
(stepped benches by elevation), **Crumble/Shatter/Fracture** (Voronoi block displacement), **Sharpen/Fold/Slump**,
**Hard/soft thermal** (per-layer talus angle: hard cap-rock keeps steep faces, soft layers slump to scree), **slope
masks** (the "cliff" mask for texture and rocks), **strata erosion** (layers of different erodibility). The unifying
mechanism is **heterogeneous rock plus thermal/slump with different repose angles**, giving cliff-and-talus profiles
(cuesta, mesa, hoodoo) that stream power never makes.

Papers (V = found this session; others S):

- **Paris et al. 2019, "Terrain Amplification with Implicit 3D Features"**, ACM TOG (V: volumetric cliffs, overhangs,
  arches, stratified cliffs, hoodoos, caves from a heightfield plus feature graph; shape grammar guided by stratified
  erosion; [PDF](https://people.cs.uct.ac.za/~Jgain/wp-content/papercite-data/pdf/paris2019.pdf)). Implicit field, but
  needs a feature graph and a polygonizer: a **bake-to-mesh** tool for hero cliffs, not a per-sample height function.
- **Paris et al. 2020, "Modeling rocky scenery using implicit blocks"** (V listed, [HAL](https://hal.archives-ouvertes.fr/hal-02926218v2/file/2020-rocks-author.pdf)):
  rock scenery from implicit block primitives. Same class: baked meshes.
- **CEIG 2024 short paper, alpine rocky terrain with sharp ridges and frost-wedging weathering** (V found,
  [PDF](https://diglib.eg.org/bitstream/handle/10.2312/ceig20241139/01_ceig20241139.pdf); content only from a search
  summary, unread). Candidate for crest sharpening.
- **Cordonnier et al. 2016/2017, Schott et al. 2023** (S, our stage 2): large forms; sharp ridges via the slope exponent,
  not cliffs.
- **Grenier et al. CGF 2024, real-time erosion patterns** (V title, [PDF](https://diglib.eg.org/bitstream/handle/10.1111/cgf14992/v43i1_05_cgf14992.pdf)):
  the family our WG-F filter already follows; channels, not cliffs.
- **Galin et al. STAR 2019** (S): heightfield (cheap, no overhangs) vs layered (strata) vs implicit/volumetric (overhangs,
  costly). **Peytavie et al. 2009 layered terrain / arches** (S): storage that allows overhangs; unnecessary if
  overhangs are mesh overlays.
- **Guerin et al. 2017 cGAN, Zhou et al. 2007 example-based terrain** (S): the data-driven line (section 3).

### Stateless per-sample vs bake

| Operator | Stateless GPU + CPU twin? | Notes |
|---|---|---|
| Elevation terraces / strata with warped, jittered bench heights, thickness per rock id | **Yes** | Must be band-limited per clipmap level (step width at least 2 samples) or it shimmers. Masked by rock/slope/altitude from the guide. |
| Voronoi/Worley fracture blocks (block lava, columnar joints, shattered cap-rock): F2-F1 edge, per-cell tilt/height | **Yes** (integer hash, 3x3 search) | Fits the existing hash-noise path. Amplitude by slope/lava mask, faded by octave. |
| Ridged/crease/abs noise, slope-dependent exponent | **Yes** | Crests, not cliffs. |
| Slope steepening along the gradient | Partly (gradient already computed by the gully filter) | Bounded displacement along the guide normal; avoid self-feedback at coarse levels. |
| Escarpment strokes (throw, wall angle, talus apron, noisy trace) | **Yes if the stroke field is a raster** (distance/throw) sampled plus noise; stroke *placement* is a bake | The Far Cry / Gaea cliff brush as data. |
| Thermal with heterogeneous talus angle (cap-rock 60-75, soft 30 degrees) | **No** (iterative) -> bake at 5-10 m | Our stage 3 has thermal; add a hardness field and per-layer repose. Minutes. |
| Lava flows: vent, D8/MFD downhill route, lobe thickness, levees, steep aa fronts, channels | **No** (routing) -> bake; result = height delta + lava-age mask for the runtime filter | Stage 3 already plans lava fronts; rifts and cones exist in WG-VO. |
| Columnar basalt | Yes (hex Voronoi variant) | Decorative, geology-gated. |
| Overhangs, arches, tubes | No | Mesh chunk overlay. |

Takeaway: the runtime function can carry terraces/strata, fracture blocks, ridged sharpening, and stroke rasters; the
bake must carry flow landforms, escarpment placement, and hardness-aware thermal. This fits the existing split (bake =
stage 3 landform operators, runtime = stage 7).

## 3. Learned and data-driven

**Terrain Diffusion 30 m** (MIT, ledger TD-1..3): decoder resolution is 30 m on a 30 m DEM (Copernicus GLO-30 class; S
for training data). It reproduces calderas, gorges, cones (crater 268/320 m kept) but nothing under about 60 m, and it
was already judged not for the bake. It cannot add m-scale blocks or 5-30 m flow fronts. A finer model needs
retraining on 1-10 m data (documented path needs an 18 GB GPU): large, risky, provenance questions. **Not recommended
for sharpness**; keep as an optional large-form source.

**Exemplar-based detail is the better data-driven route and can stay stateless:**

- Take real 1-2 m DEM tiles of target landforms (aa/pahoehoe fields, flow fronts, cone slopes, scarps, boulder fields)
  as *displacement exemplars* (e.g. 2048^2 at 1-2 m, a few terrains). Sample at runtime with **stochastic/hex tiling with
  histogram-preserving blending** (Deliot and Heitz 2019, Unity HDRP; S): never visibly repeats, a few texture fetches
  per height sample, and the CPU twin does the same bilinear math. Rotate/scale per region, amplitude by mask, fade by
  LOD. This is Zhou et al. 2007 reduced to its stateless core. The exemplar's normal map also feeds step 1.
- Procedural alternative: fit spectrum plus Worley-block statistics to the exemplar and drive the section 2 operators.
  WG-M already measures band RMS and PSD on real DEMs, so targets exist.
- **Licences (verify before ingest):** Copernicus GLO-30: free, attribution and a modified-data notice "(c) DLR e.V.",
  licence acceptance (V, [licence](https://documentation.dataspace.copernicus.eu/APIs/SentinelHub/Data/DEM/resources/license/License-COPDEM-30.pdf));
  a 28 July 2026 restriction concerns the Sentinel Hub *view service*, not downloaded/AWS copies (V from search, re-check).
  USGS 3DEP 1 m lidar (Hawaii lava): US government data, normally public domain (S). ÍslandsDEM 2 m believed CC BY 4.0
  (S, unverified). Spain PNOA-LiDAR via CNIG believed CC BY 4.0 (S, unverified). **Start with USGS 3DEP** (public
  domain); add others after reading the official terms; CC BY data needs an entry in `THIRD_PARTY_NOTICES.md`.

## 4. Rocks and boulders as objects

- **Scatter rules (Horizon-style density, generate, place; V):** inputs slope, curvature (convex = outcrop, concave =
  deposit), exposure/height above local base, rock type, lava-age mask, distance to cliff strokes, and for **talus
  fans** the distance below a steep face along the fall line: density rises at the base and decays, size decays with
  distance (big blocks travel least). Stage 8 of our spec already has this scaffolding.
- **Size distribution:** truncated power law N(>d) ~ d^-b, b about 1-2.5 (S, rock-fall and joint-spacing geomorphology),
  per biome. Use 3-4 size classes (0.3-1, 1-3, 3-10, 10-40 m). Small: GPU instanced, near only; large: fewer, LODed, shadow
  casting. Sink 20-40 percent into the ground, blend orientation to the normal. Below about 1 m prefer the detail
  height function (block noise): zero draw calls.
- **Mesh sources:** (a) procedural: noise-displaced icosphere or Voronoi-cut polyhedra, 3-5 LODs, a few hundred lines of
  our own code; no rock-specific Rust/Bevy crate found (`bevy_generative` 0.3.1 is a map/terrain generator, MIT/Apache per
  its README; `bevy_procedural_meshes`, `procedural_modelling` are generic mesh builders; all V by listing only, check
  Bevy-0.20 support and licence before any use); (b) CC0 scans (Poly Haven, ambientCG; S); Megascans/Fab terms outside
  Unreal not verified, avoid until read. **Cliff kit:** 20-100 m slabs, columns, ledges, snapped to steep stroke
  segments, generated at bake (implicit blocks to mesh).
- **Morrowind `terrain_rock_*` at 10x:** they are low-poly, smooth cliff fakes tuned to 1:1 terrain; scaled 10x they would
  be giant but just as smooth as the terrain (blobs again) with textures stretched 10x. **Keep dropping them.** Optionally
  reuse at 1-3x as small outcrop variants after retexturing, and reuse their *placement statistics* (WG-R3 occupancy
  method) to learn where Morrowind wanted rocks, as scatter density rules.
- **Bevy 0.20:** instanced meshes through the existing path; virtual geometry (meshlets) is experimental (S; check
  `docs/wiki/bevy/`). No Nanite equivalent, so cliff-kit pieces are hand-LODed.

## 5. Recommendation (ranked)

**Step 1: slope-driven rock material (shader, GPU runtime). Fixes the read from the ground and from afar.** Triplanar
rock on slopes above about 30-35 degrees (blend width by slope), height-blended against soil/ash, near-camera
normal/parallax detail from the step 2 exemplars, crevice darkening from curvature. Cost **G** 0.3-0.8 ms at 1440p DLSS Q
(measure). Risk low; highest value per ms. The owner judges the look: register a view scene, do not judge from images.

**Step 2: sharp-forms operator pack in the runtime detail function (`tiles.wgsl` plus CPU twin). Fixes the 1-100 m band.**
Behind declared tunables (Guess allowed) and per-region masks: (a) strata/terrace benches by rock id, band-limited;
(b) Voronoi fracture blocks, amplitude by slope and lava mask; (c) ridged crest sharpening; (d) stochastic-tiled
real-DEM exemplar displacement (USGS 3DEP lava tiles). Reduce the gully filter where these apply (answers "grooves").
Stateless and deterministic, same GPU-vs-CPU twin test as G2. Cost **G** 0.5-1.5 ms extra at tile fill only (fills are
amortised; watch the 3 ms per 512^2 L0 fill). Risk: seam shimmer between levels (partition of unity already used),
CPU twin cost near the player.

**Step 3: bake-time landform pack in `sf_worldgen` stage 3. Fixes the 20 m-2 km band where escarpments and flow fronts
live.** (a) Cliff/escarpment strokes as guide primitives (fault scarp, caldera-wall oversteepening, sea cliff, throw,
angle, talus apron); the Morrowind adapter places them from its own features (Ghostfence, Red Mountain wall, coast)
and the volcano recipe (collapse scar, rifts). (b) Hardness-aware thermal (WG-S5 Worley K field reused) with per-layer
repose angle. (c) Lava-flow operator from the rift/cone vents (route, lobe thickness, levees, steep aa fronts), writing
a height delta and a lava-age mask consumed by step 2. Cost: a few minutes at 5-10 m once; hash-cached; fixed-order
solvers keep determinism. Risk: tuning. Add **slope-distribution tests** to WG-M (area above 35/50 degrees, curvature
kurtosis, 1 m slope variance) against Etna/Teide/Iceland/Hawaii: current metrics reward roughness, not sharpness.

**Step 4: mesh overlays (scatter, then cliff kit). Covers what a heightfield cannot.** Boulder/talus scatter first
(stage 8; rules above); then a procedural cliff kit snapped to stroke segments for vertical faces, overhangs, arches,
tube mouths; optionally implicit-block baked hero cliffs (Paris 2019/2020) at the handful of landmark places. Cost:
draw calls and memory (instancing, LOD, culling; sub-metre rocks go into the height function). Risk: largest content
effort; ship scatter first, the kit only if steps 1-3 do not satisfy.

Why this order: 1 and 2 are cheap, visible at once and need no new bake; 3 gives the big forms no fine detail can; 4 is
the only route to true vertical faces. Framework stays generic (operators, masks, stroke primitives in `sf_worldgen`;
rock types, stroke sources and asset names in the `mw_worldgen` adapter); all deterministic (hash noise, fixed-order
solvers, CPU twin tests); baking a few minutes once; frame cost confined to step 1's shader and step 4's draws.

## 6. Open items

- Verify: ÍslandsDEM and PNOA licence texts; Megascans/Fab terms outside Unreal; contents of the CEIG 2024 and Grenier 2024
  papers; Bevy 0.20 meshlet status.
- ~~Owner question (design-level): steps 2-3 add steep faces where Morrowind's source has none; accept losing some
  recognisable gentle silhouettes?~~ **Answered (owner 2026-11-20):** yes, aggressive dramatic cliffs and escarpments
  are welcome; keep iteration fast so we can test arms quickly.

## 7. Addendum (2026-11-20): Confirmed talks, fast steerability, forests, GPU foliage & anti-tiling

Verification and expansion of section 1-5 open items, plus the linked WG-X problems (sprayed flora, 26 fps at full
density, tiled ground textures):

1. **Far Cry 5 (V, GDC 2018, Etienne Carrier, "Procedural World Generation of Far Cry 5"):**
   - Cliffs: Houdini cliff tool analyzes terrain slope and topology, generates cliff geometry and blends displacement
     into terrain deterministically across world sectors (so collision and navmesh stay seamless).
   - Forests / biomes: driven by ecological factors (slope, exposure, north/south sun orientation, hydrology/soil
     humidity) and species competition rules, while artists override via POI/exclusion masks and iterate overnight.
2. **Horizon Zero Dawn / Forbidden West (V, Decima, GDC 2017 J. van Muijden; Forbidden West GDC talks):**
   - Runtime GPU placement in a camera-centred chunk grid via compute shaders: large canopy trees placed first with
     Poisson/jitter spacing, then undergrowth/rocks/debris conditioned on canopy density and slope/moisture masks
     (avoids storing millions of global static transforms).
   - Forbidden West tackles dense foliage overdraw with deferred texturing + compute VRS and motion vectors.
3. **Ghost of Tsushima (V, Sucker Punch, GDC 2021):**
   - Compute-driven procedural grass/foliage placement into indexed instanced draws (1M+ blades) fed by biome masks and
     global wind force fields; modular rock/cliff kits blended into terrain heightmaps by a dedicated world workflow.
4. **Forest structure ("sprayed like bits on a donut"):**
   - Current `sf_worldgen::scatter` runs independent per-species Bernoulli draws over one 48 m value-noise clump field,
     so species ignore each other and form uniform drizzle.
   - Industry + academic standard (**Makowski et al. SIGGRAPH 2019, "Synthetic Silviculture"** [V]; Far Cry 5;
     Horizon): (a) macro stand vs. clearing masks driven by aspect/valley/moisture/noise; (b) multi-layer vertical
     hierarchy where canopy trees place first with crown-spacing inhibition and maturity/size spread, then sub-canopy,
     shrubs and groundcover condition on local canopy shade (shade-tolerant under canopy, sun-loving at edges/clearings);
     (c) geology-coupled rock scatter with truncated power-law sizes (3-4 tiers, sub-1 m in heightfield, 10-40 m cliff
     slabs) on convex outcrops and downslope talus fans below steep faces.
5. **Dense trees & renderer path (WG-X6 + Chlorophyll/Horizon direction):**
   - WG-X's 26 fps at 7.2 M flora happened because `WorldgenPlacementStatics` disables `sf_scene_bevy`, precombine and
     `sf_lod_bevy` (23,028 ECS entities, 707 materials, 890-pipeline warm-up with alpha-blend PBR foliage).
   - **Stage 1 (WG-X6):** bake the 10x world as its own worldspace so `sf_scene_bevy` + `sf_lod_bevy` (HLOD/impostors)
     run normally.
   - **Stage 2 (true dense forests):** the independent **Chlorophyll** foliage renderer project (V, YouTube devlogs)
     and commercial open worlds (Horizon, Ghost of Tsushima, UE5 Nanite Foliage) show that millions of dense trees/plants
     require a dedicated GPU-driven vegetation pipeline: compute chunk scatter near camera, `AlphaMode::Mask` + folded
     materials (no alpha-blend PBR foliage stalls), octahedral impostors / canopy HLOD beyond ~150-300 m, and CPU trunk
     queries near the player only.
6. **Fast steerable sharp terrain for 10x Morrowind:**
   - Keep `tiles.wgsl` + CPU twin as the zero-bake, live-steerable layer for 1-100 m sharpness (per-region strata,
     Voronoi fracture blocks, ridged crests, stochastic-tiled 1 m USGS 3DEP exemplars).
   - Keep the macro guide pass fast (~5-15 s) via escarpment/cliff strokes + fast hardness-aware thermal + lava lobes.
   - **Morrowind-specific steering mask:** although `mw_worldgen` rightly drops `terrain_rock_*` meshes (scaling low-poly
     cliff fakes 10x makes giant blobs), their **source positions, footprints and heights** should be harvested as an
     automatic cliff/outcrop intent mask driving both terrain cliff strokes and modern rock/cliff scatter, alongside
     aggressive wilderness escarpments approved by the owner (2026-11-20).
7. **Ground texture anti-tiling (V, Mikkelsen 2022, "Practical Real-Time Hex-Tiling", JCGT; WG-X7a):**
   - `sf_terrain_bevy/src/shaders/surface.wesl` already uses explicit `dpx`/`dpy` derivatives with `textureSampleGrad`,
     2-pass Mishkinis height-blending and whiteout triplanar projection.
   - Mikkelsen's 3-tap simplex/hex grid needs no precomputed histogram LUT: rotate local UV offsets around each hex
     vertex center by a per-vertex random angle `theta`, rotate `ddx`/`ddy` by `R(theta)`, and **inverse-rotate the
     decoded tangent-space normal XY by `R(-theta)`** (keeping normal Z invariant).
   - Using `pow(w, contrast)` (Mikkelsen `g_exp = 7.0`) preserves contrast and concentrates weight onto 1 tap over ~65%
     of pixels, allowing sub-LSB tap pruning (`w < 0.002`) in non-uniform control flow (legal with `textureSampleGrad`).
   - Because `sf_terrain::bake::quantize_tiling` ensures `R = 1024 / tiling_m` is an exact integer, choosing integer
     `period_q` and **even** integer `period_r` makes the hex lattice exact-periodic over 1024 m and 100% invariant
     across floating-origin shifts.
