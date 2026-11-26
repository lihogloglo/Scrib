# Water types and their joins (streams to open ocean; flow maps), 2026-11-26

Research only (orchestrator research-1). Code read at HEAD 6233e915 (working tree dirty elsewhere, water files untouched). Web tools were available; each web claim carries its link in section 9. Items I could not open in full are marked **(unverified)**. Out of scope: storm sea / spray, underwater water line.

## 1. Summary

- **Cause of the "not good at all" flow maps (code level):** there is no flow map. Each river vertex gets `tangent_of_centreline * scalar_speed`, the same vector on all 5 columns across the width (`sf_water/src/body_mesh.rs:445,460`). The shader advects the **ocean's time-evolving dispersive FFT slope cascades** (`water_material.wesl:285-290`, `water_fft.wesl:78-100`) by that vector. So ripples keep their own wave motion (random directions, deep-water dispersion) on top of the advection and read as water dancing in place, not carried. The flow has no bank shear, no obstacle deflection, no solved 2D field, and no join with lakes, tributaries or the sea.
- **Canonical technique:** flow field from a solved 2D field (Vlachos 2010 builds it in Houdini by combing a vector field against level geometry; Naughty Dog authors splines in a Maya tool; UE5 writes per-spline-point velocity into a flow map), sampled per pixel, then two-phase advection of a **purpose-made tileable normal map**; transitions and joins are material blends on shared tiles (UE5), waterfalls are dedicated mesh + scroll + particles, characters add a small dynamic wave grid (Crest, UE Fluid Sim, BlackSpace "Shallow Water").
- **Joins are ignored.** `water.json` already carries `joins` and `outlet` per river (scrib-worldgen `run.rs:48-54`); `sf_worldgen::water` drops them (`water.rs:7-8`). Tributaries are overlapping ribbons, river ends are hard stops in lake or sea, waterfalls are a spec marker only (water.md:482, no code).
- **Guide lakes and rivers are visual only:** `worldgen_water.rs` spawns meshes and never builds `WaterBody` records, so the CPU `WaterQuery` (which already handles `Polygon` and `Ribbon` with flow, `query.rs:302-420`) does not know them: no swimming depth, no buoyancy, no current on characters (grep-based, no registration call found).
- **Top fixes (est. GPU, 4060 laptop, High 1440p DLSS Q):** (1) shear + bank slowdown per vertex, 0 ms, 0 bake; (2) river-only flow normal texture, frozen from FFT time, about -0.05 to +0.05 ms; (3) baked solved flow tiles with obstacle deflection, +0.05-0.1 ms, 20-60 s bake; (4) honour `joins`/`outlet` with a river-ness blend at mouths and inlets, +0.02 ms; (5) register guide bodies in `WaterQuery`, 0 GPU, CPU 2 us per sample; (6) waterfalls, 0.1-0.3 ms when visible.
- **Slices proposed:** FL-1 .. FL-7 (section 7). **Owner questions:** 4, taste/scope only (section 8).

## 2. What we do today

### 2.1 Data path (path:line)

| Stage | What happens | Where |
|---|---|---|
| Source | MU water chain writes `water.json` v1: lakes (polygon, level, `kind` open/closed), rivers (`kind` river/braid/distributary/lake_link, `points [x,y,width,level]` upstream to downstream, plus `mu_id`, `joins`, `outlet`, `clipped`) | `D:\Gamedev\scrib-worldgen\crates\water\src\run.rs:28-75, 840-870` |
| Reader | `GuideWater` keeps only `id, kind, points` (rivers) and `id, level_m, outline, holes` (lakes); "extra keys (`frame`, `stats`, `area_m2`, `joins`, ...) are ignored" | `sf_worldgen/src/water.rs:7-8, 38-45` |
| Speed per point | Manning `v = R^(2/3) S^(1/2) / n`, n 0.035 or Jarrett 1984 on steep reaches, clamp 0.2-3 m/s ish, window average. Depth is `bed_depth(width)` (a function of width), not the carved bed | `mw_environment_bevy/src/worldgen_water.rs:147-197` |
| Mesh | `river_mesh`: Catmull-Rom, 4 m spacing, 5 columns across, fold-limited half width; **`flow = tangent * flow_speed`, pushed identically for every column** | `sf_water/src/body_mesh.rs:405-470` (flow at 445, 460); params 194-215 |
| Lake | `lake_mesh`: flat CDT at `level`, no flow attribute | `body_mesh.rs:237-`, `worldgen_water.rs:267` (flow = false) |
| Spawn | one `WaterMeshSurface::still` per river/lake, `ATTRIBUTE_WATER_FLOW` set only for rivers; no `WaterBody` record, no query registration | `worldgen_water.rs:313-320, 336+` |
| Shader | vertex: `flow` rotated by `world_from_local` (water_material.wesl:94-96). Fragment: `speed = length(flow)`; phase `t/cycle + phase_noise * noise(lagrangian/1024)`; two taps offset by `-flow * cycle * advect_scale * phase`; weights = triangle (water_material.wesl:257-275). Taps read the **same slope cascades the ocean uses**, `n_cascades` of them, per tap (281-291). Slope gain `1 + normal_gain * saturate(speed/foam_full)` (296-298). Speed foam `foam_cover * smoothstep(2.4, 3.0, speed)` (372-374) | `sf_water_bevy/src/shaders/water_material.wesl`, `flow.rs:1-144` |
| Cascades | `spectrum_at`: h0 rotated by a per-texel dispersive phase each step, inverse FFT: waves travel at their own phase speed in random directions | `water_fft.wesl:78-100` |
| Query | CPU `WaterQuery` supports `Polygon`/`Ribbon` bodies and returns flow (`query.rs:302-420`), but no guide body is registered | `query.rs`, `worldgen_water.rs` |
| Interaction | spec: dynamic ripple grid (iWave) R1-R4 planned; ledger shows no R2/R3 result; no wake, no splash, no deflection | water.md:575-586 |
| Waterfalls | spec: "Waterfalls where the surface drops more than 2 m over 4 m: a marker ... not simulated". No code | water.md:482 |
| Mesh-body gaps (wiki) | no shore foam, no underwater medium, no motion-vector redraw for mesh bodies | `docs/wiki/engine/terrain-and-water.md:57-59` |

Ledger context (`docs/ledger/water.md:27`, G-W5/6/7): plain Manning on the 10x terrain gives p50 5 m/s (cap), owner: "rivers pure white"; Jarrett + cap 3 m/s gives p10/50/90 0.20/1.85/2.27 m/s, 6.3 % of points foam. That fixed the speed distribution only. Owner look on the flow itself is still pending (TN-2 round 3). WR-1 (`docs/ledger/worldgen.md:9`): 170 lakes, 80 rivers (127 km) from MU, 84 k samples cut > 5 m (gorges where courses cross ridges, max 265 m).

### 2.2 Exact causes of bad flow, ranked by visual weight

| # | Cause | Evidence | Visible symptom |
|---|---|---|---|
| C1 | Advected texture is the **ocean FFT cascade** (self-moving, isotropic-ish wind spectrum, patch sizes tuned for sea), not a flow normal map. The wave field's own phase velocity adds to the advection; at 0.2-0.5 m/s (p10 of the river speeds) the self-motion dominates | `water_material.wesl:285-290`, `water_fft.wesl:78-100` | ripples shimmer and dance in place, no sense of downstream drift, no streaking |
| C2 | Flow is `centreline tangent * scalar`, identical across the width. The doc comment promises "a centre and a quarter line each side carry the flow's shear" but all columns get the same value | `body_mesh.rs:445,460`, params doc 194-201 | banks move as fast as the middle: a conveyor belt; no slow margin, no eddies |
| C3 | No solved field: tangent only, so no response to bends (inner bank speeds up in reality), islands, rocks, bars, widenings. Bisector tangent is fine on a smooth spline but the strip is fold-limited, so on tight bends the vertex rows converge and the interpolated vector turns sharply inside one quad | `body_mesh.rs:373-395, 407-415` | flow lines pinch at bends; no wake behind obstacles |
| C4 | Speed is Manning on a width-derived depth and window-smoothed, not discharge continuity; no confluence summing; no depth-driven slowdown near banks | `worldgen_water.rs:147-197` | speed jumps unrelated to what you see; the same speed on a trickle and a main river of equal slope |
| C5 | Joins unused: `joins`, `outlet`, `kind=lake_link/braid/distributary` dropped by the reader. Tributary = a ribbon overlapping another transparent ribbon with a different flow vector; river end = hard stop in a lake (flat, no flow) or the ocean clipmap (different material setup) | `sf_worldgen/src/water.rs:7-8`; `run.rs:861-870` | visible seams at confluences, at lake inlets and at river mouths; two flows blended by alpha |
| C6 | Speed foam is a pure function of speed (`smoothstep(2.4, 3.0)`), uniform across the width, no foam from depth/gradient/obstacle, no shore foam on mesh bodies | `water_material.wesl:372-374`, wiki:57-59 | foam is all-or-nothing per reach, not at rocks and banks |
| C7 | Phase noise is a 1024 m-periodic noise sampled at 16 m cells; per-vertex flow direction changes in a quad blur the two phases differently | `water_material.wesl:268`, `flow.rs` | residual pulsing and shear smears at bends (Vlachos 2010: distortion only convincing in the first third of the cycle; our cycle is 2 s at up to 3 m/s = 6 m travel) |
| C8 | Normal strength grows with speed (`+0.5` at foam speed); Vlachos scales normal strength **down** with speed | `water_material.wesl:296-298` vs Vlachos slides | fast reaches look noisy and harsh, then white |

C1 and C2 explain "not good at all"; C3 and C5 explain "not like a river".

## 3. Bevy 0.20 native and crates

| Item | Finding | Verdict |
|---|---|---|
| Bevy 0.20 | No water, no flow-map, no fluid in `bevy_pbr`/`bevy_render` (grep of `bevy_pbr-0.20.0/src` finds only `pbr_material.rs` mentioning "water"). Release 2026-10-08 per bevy.org news | nothing to reuse |
| `bevy_water` (Neopallium) | README lists Bevy 0.18 as newest; tiled ocean with waves, no river flow | not usable (old Bevy, no rivers) |
| `bevy-aqua` 0.1.3 (sayhisam1, MIT/Apache-2.0, repo commit 98d755f 2026-09-23, Bevy 0.19, 167 commits, 122 downloads) | Cloned and read: rivers are an **analytic sum of 3 sines along a baked current vector** (`crates/bevy-aqua-core/src/cascade/river.wgsl`), one horizontal level per body, "tilted water is rejected". Current comes from a field texture (`fields.rs`) | reference only: validates "baked current field + per-body optics", but no better than ours for flow look, no waterfalls, no joins |
| `bevy_hanabi` | Registry holds 0.19.0; web shows 0.18 for Bevy 0.18; no 0.20 release found | splash/mist candidate only once a 0.20 release exists; until then our own billboards/PR2 particles (water.md R4) |
| Shallow-water / fluid crates (`salva`, others) | not evaluated in depth (CPU particle SPH, not a 2D height grid); a GPU heightfield is ~100 lines of compute and already planned as R2 | build our own (spec R2), per Crest-style reference |
| Flow-map generators | Houdini (Vlachos), Maya tool (Naughty Dog); no Rust crate found | our own bake: potential-flow solve on the water mask (section 6, FL-3) |

## 4. Industry: canonical techniques

### (a) Water-body model

| Engine | Model | Source |
|---|---|---|
| UE5 Water plugin | Ocean (closed spline, one height), Lake (closed spline, one height, carves terrain), River (open spline, per-point height, **width, depth, velocity**), Custom (static mesh, e.g. pool), Island (raises terrain); Water Zone fills the horizon; Exclusion Volumes cut dry cavities; priority resolves overlaps; **water info texture** + **water data queries** give height/flow/depth on CPU and GPU | UE docs "Water Body Actors", "Water Meshing System" (info-texture channels not in the page: unverified) |
| bevy-aqua | circle / polygon / corridor / river bodies, one horizontal level per body, per-body optics | repo README |
| Ours (spec) | `WaterBody { kind: Ocean/Lake/River/Pool, shape: Infinite/Polygon/Ribbon/Bounds, priority }` + BVH query, planned water-level clipmap = UE's water info texture | water.md:82-123, 428-446 |

Our model matches UE's. The gap is that the generated world bypasses it (meshes only).

### (b) Flow field generation and rendering

| Source | Generation | Rendering |
|---|---|---|
| Vlachos, Valve 2010 (L4D2 / Portal 2) | flow map ~4 texels/m, **built in Houdini: level geometry imported, vector field combed, procedural masks**; painting by hand "impractical" | two layers half a period apart, triangle cross-fade, noise on the phase to hide pulsing, UV offset by flow x time, normal strength scaled **down** by flow length; cost +2 fetches, ~21 ALU |
| Naughty Dog (Uncharted 1-4) | Maya tool: splines give direction and velocity, colour map gives magnitude; also generates foam and phase maps; U4 rivers used a mix of simulation and hand-drawn curves (SideFX interview, secondary) | same two-fetch half-cycle blend (Max and Becker 1996, Neyret 2003); foam scrolls with flow; "Rendering Rapids in Uncharted 4" (SIGGRAPH 2016) slides not read (unverified content) |
| UE5 | per-spline-point velocity written to a flow map; Positive/negative sign = direction; Visualize Water Velocity debug | River material has velocity, flow control, density parameters; rivers carry no waves (they need transition materials at lakes and oceans) |
| Peytavie et al. 2019 "Procedural Riverscapes" (CGF) | river network from hydrology, bed carved, a **blend-flow tree**: procedural flow primitives as leaves, blend operators as nodes; time-varying continuous function | real-time animation incl. foam and floating leaves (details of flow modelling unread: unverified) |
| BlackSpace (Crimson Desert), press | "FFT ocean simulation and shallow water simulation" (80.lv) plus "Shallow Water ... rivers coursing around rocks" (secondary press); former RDR2 water artist praise; GamesRadar interview text could not be fetched (truncated): **method of river flow unverified** | |
| Far Cry 5 (Cutocheras, Grujic GDC 2018) | session listing only; "sloped water for river rapids" (secondary); no technique details retrieved | |
| RDR2 | no official technique source found; do not cite | |
| Horizon Forbidden West | no public river-flow talk found; press only | |
| Sea of Thieves | FFT ocean (Tessendorf); rivers not documented in what I found | |

**Canonical (b):** a 2D vector field solved against the actual geometry (Houdini combing / potential flow / hydrodynamic sim) baked to a tileable texture or per-vertex data **with cross-channel variation**, plus two-phase advection of a **dedicated flow normal map**, phase noise, speed-modulated normal strength, foam from the same field (convergence, speed, depth).

### (c) Joins

| Join | Canonical |
|---|---|
| River into lake / sea | UE: dedicated "River to Lake" / "River to Ocean" transition materials on the shared tile mesh; Min blend mode so the river does not raise the lake's carved terrain; forum reports of seams/gaps at these joins (forums, secondary). Practice (also UE docs implication): the river's flow and waviness fade out over a mixing zone, colour matches |
| Confluence | Peytavie 2019 blend-flow tree (blend operators); UE: overlapping spline bodies with Overlap Material Priority |
| Waterfall | mesh with scrolling stretched noise + particles + mist; plunge pool foam; UE/Naughty Dog treat as authored; no public algorithm to cite |
| Estuary | tidal mixing is not modelled in the shipped games I found (unverified); visual: colour blend, foam line, flow fading into the sea state |

### (d) Dynamic interaction with characters

| Source | Technique |
|---|---|
| Crest (Unity, MIT) | dynamic wave sim around actors (planned R2 reference) **(unverified, recalled)** |
| UE5 | Water plugin has no wading wake by default; Fluid Simulation / Niagara Fluids and Water Body Exclusion are separate **(unverified)** |
| BlackSpace | shallow-water simulation lets rivers flow around rocks and objects, ice arrows freeze a river chunk (press, secondary) |
| Ours | R1-R4 planned (water.md:575-586), nothing built |

**Canonical (d):** a camera-centred GPU wave-equation grid (iWave/Crest) fed by actor emitters; the grid is advected by the local river flow so wakes trail downstream; static obstacles are handled in the baked field, not the sim.

## 5. Why ours differs, point by point

| Topic | Industry | Ours | Delta |
|---|---|---|---|
| Normal source | dedicated tileable flow normal map | ocean FFT cascades, self-animating (C1) | wrong texture and double motion |
| Flow field | solved against geometry, texture ~4 texels/m (Vlachos) or per-point velocity + cross-section profile | tangent x scalar per vertex, same across width (C2, C3) | no shear, no obstacles |
| Speed | authored or simulated | Manning on width-derived depth (C4) | plausible numbers, no continuity |
| Normal strength vs speed | down with speed (Vlachos) | up with speed (C8) | inverse |
| Foam | from flow convergence / depth / obstacles | `smoothstep(speed)` uniform (C6) | flat foam |
| Joins | transition materials, shared tiles, blend priority | ignored `joins`; hard stops (C5) | seams |
| Lake with outflow | lake flows only near inlet/outlet | still, no flow | acceptable but needs an outlet blend |
| Query | water data queries (CPU/GPU) | CPU query exists, guide bodies not registered | no swim, no current on actors |
| Waterfall | mesh + particles + mist | marker only | missing |
| Character wake | dynamic wave grid | none (R2 planned) | missing |

## 6. Ranked fix list

GPU estimates are my own, reasoned from the existing shader cost (2 taps x n cascades today); **none measured**. Bake limits: one-time, under a few minutes.

| Rank | Fix | GPU ms (est.) | CPU / bake | Spec sections / slices | Risk |
|---|---|---|---|---|---|
| 1 | **Cross-section flow profile** in `river_mesh`: per-column speed = `v * profile(u)` (power-law or parabolic `1-(2u)^2`, floor 0.15), direction rotated slightly toward the channel on bends; vertex data only | 0 | 0 | water.md "Rivers", `body_mesh.rs:445`; FL-1 | very low; look decided by owner live (tunables) |
| 2 | **Dedicated flow normal texture** (tileable, 512-1024^2, 2-3 octaves baked once at start from noise, anisotropic along flow) replacing the FFT cascades on `WATER_FLOW` bodies; no self-motion; strength falls with speed (Vlachos); foam channel in the same texture | -0.05..+0.05 (1 texture, 2 taps, vs n cascades x 2 taps) | start-up generation < 1 s | water.md "Shading"; FL-2 | low; owner look |
| 3 | **Baked solved flow field** per river tile: rasterise ribbons (and lake margins) to 1-2 m texels (RG8 snorm + width/shore distance), solve a potential flow on the water mask (Laplace with inflow/outflow, Dirichlet at banks tangential, obstacles = terrain above water inside the mask), scale by the cross-section speed; read per pixel instead of per vertex | +0.05-0.10 (one fetch + shear) | CPU 20-60 s for 127 km at 2 m (est.; 127 km x 40 m = 5 km^2 = 1.3 M texels, tens of ms per solver pass, multigrid) | water.md `water.flow` 1-4 (already specified: fast marching + discharge routing), `water-level clipmap`; FL-3 | medium (new product, streaming); fallback is rank 1 |
| 4 | **Joins**: reader keeps `joins`, `outlet`, `kind`, lake `kind`; end of a river gets a `river_ness` vertex attribute fading to 0 over 2-3 widths (blend to lake/sea material, foam line, flow decays); tributary end ends exactly on the receiver's centreline with matching level and a foam wedge; `lake_link` gets a flow through the lake body | +0.02 (one attribute, one multiply) | 0 | water.md "Rivers: baked flow to the sea" (join at confluences, mixing zone, now owned by code); worldgen.md hydrology; FL-4 | medium: MU output edge cases (clipped lines); needs tests on x10s-b2w |
| 5 | **Register guide bodies as `WaterBody` records** (lakes `Polygon`, rivers `Ribbon` with speed) in the CPU query; priority below ocean for lakes at altitude; swim/wade depth from bed height; current acts on characters and floaters | 0 | query 2 us/sample (spec) | water.md "Water bodies at any height", "Wading"; M6 buoyancy; FL-5 | low; need 'lake above sea level' check in `depth` |
| 6 | **Waterfalls**: detect drops (> 2 m over 4 m, spec marker) in river profiles and at lake outlets; steep ribbon variant with downward 3D flow, stretched scroll noise, foam, mist billboards, plunge ring | 0.1-0.3 per visible fall (est.) | detection at mesh build | water.md:482; new section; FL-6 | medium; look-heavy, owner judges |
| 7 | **Character interaction**: R2 grid advected by the local flow; R3 emitters for wading actors; foam flecks from the same grid; no per-obstacle dynamic flow | 0.05-0.08 (spec estimate for R2) | none | water.md:575-586; FL-7 | low, already planned |
| 8 | **Speed from continuity**: Q accumulates down the network (`joins`), width from Q (hydraulic geometry), speed `Q/(w d)` clamped; replaces width-derived depth | 0 | negligible | water.md `water.flow` 3; part of FL-3 | low |

Estimated total at High: +0.1-0.2 ms median for 1-5, waterfalls and wakes only when visible. Within the 8.3 ms frame target by a wide margin; no p99 effect expected (no allocations, no bakes in play).

Owner taste (lit, dramatic beats strict physics): keep Manning/Jarrett as a speed source but let the look (profile floor, foam threshold, streak length) be live tunables.

## 7. Proposed build slices (one deliverable + its test each)

All new constants are tunables declared with unit, range and source (`Guess` allowed), never bare consts. Framework code stays game-agnostic in `sf_water` / `sf_water_bevy` / `sf_worldgen`; game wiring (Morrowind adapter, `worldgen_water.rs`) in `mw_environment_bevy`.

| Slice | Deliverable | Test | Crates |
|---|---|---|---|
| FL-1 | `river_mesh` writes per-column flow `v * profile(u)`; tunables `water.river.bank_speed_floor`, `water.river.profile_exp` | exact: centre = v, edge = floor x v, all columns parallel to the tangent, monotone in |u|; existing `body_mesh_tests` still pass | `sf_water` |
| FL-2 | River flow normal texture (CPU-generated, 1 s, deterministic from seed) bound for `WATER_FLOW`; `WATER_FLOW` stops reading the FFT cascades; strength scale vs speed per Vlachos; ledger A/B arm `--off waterflow_tex` | shader compile (`shader_tests.rs`), CPU twin of the phases test unchanged, sanity: no NaN | `sf_water_bevy` |
| FL-3 | `sf_water::flow_field`: rasterise ribbons to a mask, potential-flow solve, RG8 tile output, cache product `water.flow` fingerprinted on terrain + bodies; shader fetch replaces the vertex attribute where a tile exists | exact on synthetic: straight channel uniform, bend inner/outer speed ratio, island splits flow and re-merges (divergence-free within 2 %), runtime ms printed by an `#[ignore = "measurement"]` test | `sf_water` (Bevy-free), `sf_water_bevy` |
| FL-4 | Reader keeps `joins/outlet/kind`; `river_ness` attribute; mouth/inlet mixing; tributary snap | exact: a Y confluence ends on the receiver's centreline with equal level; a mouth into a lake/sea has `river_ness` 1 -> 0 over the zone; counts on x10s-b2w water.json | `sf_worldgen`, `sf_water`, `mw_environment_bevy` |
| FL-5 | Guide bodies as `WaterBody` records in `WaterQuery` (priority, depth, current) | exact: a point in a river returns kind River, flow = tangent x speed; a point in a mountain lake returns depth = level - bed; swim immersion through `immersion()` | `mw_environment_bevy`, `sf_water` |
| FL-6 | Waterfall detection (`sf_water::waterfall`) and a waterfall mesh/effect | exact: detection on synthetic profiles (drop thresholds as tunables); the look goes to `visual-checks/water.md` as "awaiting owner" | `sf_water`, `sf_water_bevy` |
| FL-7 | R2/R3: dynamic ripple grid advected by the flow sample, wading emitters | sanity: no NaN, settles to rest, energy decays | `sf_water_bevy` |

Order: FL-1, FL-2 (look now), FL-5 (gameplay), FL-4, FL-3, FL-7, FL-6. Research-first rule: reuse check done in sections 3-4 (Bevy: none, crates: none usable, industry: Vlachos + UE). Ties to worldgen: MU emits the network and levels, the bake only reads them; no Morrowind logic in generation (owner 2026-11-24).

## 8. Owner questions (taste / scope)

1. **Rivers carry no ocean-style waves anywhere (only flow ripples and foam), correct?** Recommend yes (UE and Vlachos do the same; big rivers may later get a low swell).
2. **Should the player be able to swim and be pushed by the current in generated lakes and rivers now (FL-5), before the look work?** Recommend yes: it is cheap and makes every later look check testable by walking in.
3. **Waterfalls: visible, dramatic set pieces wherever the MU levels drop more than 2 m (plus mist), or only on named falls?** Recommend all drops over the threshold, sized by Q, with the threshold as a tunable.
4. **How strongly should a wading character disturb a river (rings only vs rings plus a visible downstream wake that fades)?** Recommend rings plus a short fading wake (FL-7), no change of the river's own flow.

## 9. Sources

Web (fetched or searched, 2026-11-26):
- Vlachos, "Water Flow in Portal 2", SIGGRAPH 2010 Advances: https://www.advances.realtimerendering.com/s2010/Vlachos-Waterflow(SIGGRAPH%202010%20Advanced%20RealTime%20Rendering%20Course).pdf (PDF not text-extracted); details read from https://www.slideshare.net/slideshow/siggraph-2010-water-flow-in-portal-2/4899875 and https://graphicsrunner.blogspot.com/2010/08/water-using-flow-maps.html (blog commentary: noise hides the pulse, did not remove it). Valve wiki: https://developer.valvesoftware.com/wiki/$flow_noise_texture (unread).
- Gonzalez-Ochoa, "Water Technology of Uncharted", GDC 2012: https://www.slideshare.net/slideshow/water-technologyofunchartedgdc2012/12977478
- Gonzalez-Ochoa, "Rendering Rapids in Uncharted 4", SIGGRAPH 2016 Advances (title only, slides not read): https://advances.realtimerendering.com/s2016/ ; SideFX interview (secondary): https://www.sidefx.com/community/fx-adventures-in-uncharted-4-a-thiefs-end
- UE Water Body Actors: https://dev.epicgames.com/documentation/unreal-engine/water-body-actors-in-unreal-engine ; Water Meshing System: https://dev.epicgames.com/documentation/unreal-engine/water-meshing-system-and-surface-rendering-in-unreal-engine ; Water System: https://dev.epicgames.com/documentation/unreal-engine/water-system-in-unreal-engine ; seam reports (forum): https://forums.unrealengine.com/t/waterbody-plugin-in-5-1-issue/726667
- Peytavie et al. 2019, Procedural Riverscapes: https://www.cs.purdue.edu/homes/bbenes/papers/Peytavie19CGF.pdf (abstract level only)
- Crimson Desert BlackSpace (press, secondary): https://80.lv/articles/pearl-abyss-demonstrated-crimson-desert-s-blackspace-engine-tech-advancements ; https://www.gamesradar.com/games/open-world/crimson-desert-does-rivers-better-than-red-dead-redemption-2-says-former-rockstar-artist-in-praise-of-the-open-world-games-attention-to-detail-i-couldnt-be-more-pleased-to-see-this/ (text truncated, claims unverified) ; https://gamesradar.com/games/open-world/at-a-ridiculously-detailed-showcase-of-the-open-world-engine-behind-the-rpg-crimson-desert-i-asked-a-ridiculously-detailed-question-about-water-and-all-hell-broke-loose (truncated)
- Far Cry 5 water talk (listing only): https://80.lv/articles/the-gdc-talk-water-rendering-in-far-cry-5
- Sea of Thieves: https://history.siggraph.org/?p=84316 (secondary)
- Horizon Forbidden West (press only, no river talk found): https://gamesradar.com/horizon-forbidden-west-dev-explains-why-its-water-looks-so-unbelievably-good
- Crates: https://lib.rs/crates/bevy_water ; https://github.com/sayhisam1/bevy-aqua ; https://crates.io/api/v1/crates/bevy-aqua ; https://github.com/djeedai/bevy_hanabi ; Bevy 0.20: https://bevy.org/news/bevy-0-20/
- Not verified and recalled only: Crest dynamic waves/flow, UE Niagara Fluids, Génevaux 2013 hydrology terrain, Teoh RiverWorld, waterfall/estuary practice in shipped games.

Cloned: `C:\Users\leogloglo\AppData\Local\Temp\research-1\water-types\bevy-aqua` @ 98d755f (read `river.wgsl` only; untrusted, not run).
Repo: our paths above at HEAD 6233e915; sibling `D:\Gamedev\scrib-worldgen\crates\water\src\run.rs` (rev not checked).
