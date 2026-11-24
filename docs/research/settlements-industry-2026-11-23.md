# Settlement generation and town expansion: industry survey (2026-11-23)

Research only, no code. Goal: grow Morrowind towns (Balmora ~10x) on the "same map at 10x" world from the existing 1:1 town as a fixed seed, in the separate scrib-worldgen repo. V = source read (this session), U = unverified (from memory or search-snippet only). The Magna Urbis generator is covered by another agent and appears only in the comparison.

## 0. Answer in short

The canonical industry pipeline is not one algorithm but a chain: **street network (tensor-field / growth rules) -> blocks (planar-graph faces) -> lots (OBB / skeleton subdivision) -> building selection by lot size, frontage and district -> prefab placement**. Shipped games (City Sample / Matrix Awakens, Daggerfall, Manor Lords) differ in which stage a human authors. For "grow an existing core" the best-fitting published family is **growth simulation with fixed existing geometry** (Weber 2009; Lechner), seeded with the existing street graph, over **tensor-field-guided** streets (Chen 2008), then **lot subdivision** and **kit placement with a per-style rule table**. No shipped Morrowind tool does this (section 2).

## 1. Canonical techniques

Columns: what it does / takes an existing town as a fixed seed? / fit for prefab kits / status.

| Technique | What it does | Fixed seed? | Prefab-kit fit | Status |
|---|---|---|---|---|
| **Daggerfall blocks** (RMB exteriors, RDB dungeons) | A town is a grid of hand-authored map blocks (~920 RMB records, each a whole city block with buildings, props, ground) chosen per location; the generator composes, it does not design buildings. Source: UESP "Map Block Records" https://en.uesp.net/wiki/Daggerfall_Mod:BLOCKS.BSA/Map_Block_Records (V for record counts and roles via search snippet; the exact block-picking rule per city size is U). Community mods replace RMB blocks to vary layouts (Nexus mods 485, 720; V snippet only) | Yes in effect: blocks are cells; an existing block can stay | Excellent: the kit IS the block | Shipped 1996; coarse, grid-bound |
| **Manor Lords burgage plots** | Gridless, player-placed roads; the player draws plots that auto-fill with a house plus back-lot buildings and upgrade over time (official wiki https://wiki.hoodedhorse.com/Manor_Lords/Buildings , V via snippet: "gridless... full freedom of placement and rotation"). The game does not generate towns; the plot-fill algorithm is not documented in sources found (U). Organic look comes from road-aligned plots, not from a growth algorithm | Yes (player-authored) | Good: plot = lot with a fixed building set | Shipped 2024; useful as a look and lot-model reference only |
| **CGA shape grammars / CityEngine** (Parish and Mueller, SIGGRAPH 2001 "Procedural Modeling of Cities"; Mueller et al., SIGGRAPH 2006 "Procedural Modeling of Buildings") | L-system street growth with global goals and local constraints (2001); a shape grammar that subdivides lots into masses, facades, floors and tiles (2006). CGA rules reference asset libraries, so prefab kits are first-class | Streets: the 2001 system grows from given seeds and honours existing roads (U on detail); lots/buildings take any lot polygon | Excellent (rules insert models) | Shipped as a product (CityEngine, Esri); papers are the references (U: not re-read) |
| **Houdini city generation** (UE5 City Sample / The Matrix Awakens) | Houdini "City Layout" operator turns a user-drawn layout into roads, blocks, lots and building massing; output points go to Unreal's Rules Processor which places kit pieces; later rebuilt on UE PCG. Sources: https://dev.epicgames.com/documentation/unreal-engine/city-sample-quick-start-for-generating-a-city-and-freeway-using-houdini , https://www.sidefx.com/learn/talks/the-matrix-awakens-generating-a-world/ (V, abstracts) | Layout is authored top-down, so an existing area can be drawn in; no growth step | Excellent: modular kit placed by a rules processor | Shipped, free project. The best template for the lot-to-prefab stage |
| **Watabou Medieval Fantasy City Generator** | Voronoi-style partition into wards (market, craftsmen, slums, castle...), streets from ward edges, lots by recursive subdivision of ward polygons; 2D only. Source: author's repo https://github.com/watabou/TownGeneratorOS (V that it exists; the algorithm details come from memory, U). Author says Voronoi "not really necessary anymore" (V, itch.io devlog comment via snippet) | Not natively (city is born whole, with walls); a seed would be a hack | Fair: building polygons are rectangles-ish, so an OBB kit fits | Shipped web tool, open source (check licence before porting; U) |
| **Townscaper / WFC** (Stalberg; Gumin's WaveFunctionCollapse https://github.com/mxgmn/WaveFunctionCollapse ; https://www.gamedeveloper.com/game-platforms/how-townscaper-works-a-story-four-games-in-the-making , V title only) | Irregular quad grid plus WFC-style constraint tiling of building modules from the player's painted cells | Yes: cells can be pre-collapsed (editable WFC, Boris the Brave https://www.boristhebrave.com/2022/04/25/editable-wfc/ , V title) | Kit modules are tiles, but Morrowind's whole-building meshes (section 3) do not tile, so WFC suits a modular facade kit, not our prefabs | Shipped (Townscaper, Bad North) |
| **Weber et al. 2009** "Interactive Geometric Simulation of 4D Cities", CGF 28(2) https://diglib.eg.org:443/xmlui/handle/10.2312/CGF.v28i2pp481-492 | Geometric (not grid) growth simulation over time: streets, parcels and buildings evolve per step, about 1 s/step (V, abstract). Existing city is the initial state | **Yes, by design** (a city at time t is the input) | Buildings are parametric, but lot/building rules can pick prefabs | Research; no code known (U) |
| **Lechner et al. agent-based** (Lechner, Watson, Wilensky 2003, "Procedural city modeling", U; the title and year are from memory) | Developer/worker agents add roads, lots and land use by local rules | Yes (agents start from existing map) | Lot-level only | Research (U) |
| **Tensor-field streets** (Chen, Esch, Wonka, Mueller, Zhang, SIGGRAPH 2008, ACM TOG 27(3):103, https://www.sci.utah.edu/~chengu/street_sig08/street_project.htm , V) | User-designable tensor field (basis fields: grid, radial, plus terrain-following) -> streamlines traced as roads (major and minor eigenvector), then blocks | Yes: existing roads become constraints and basis fields; fields can follow terrain and a river | Gives blocks; kits come later | Research, widely reimplemented (open-source "RoadGen" style tools, U) |
| **Example-based streets** (Aliaga et al. 2008 TOG https://www.cs.purdue.edu/homes/bbenes/papers/Aliaga08ToG.pdf ; Nishida, Garcia-Dorado, Aliaga, "Example-driven procedural urban roads", CGF 34, 2015/2016 https://diglib.eg.org/items/7a226fc0-3502-41ff-a14c-69b95d6e1dd0/full , V abstract: example patches plus procedural growth adapting to terrain) | Copies road patches from an example city (e.g. Balmora's own streets) and extends with statistics | Yes: the existing town is the example and the seed | Streets/blocks only | Research |
| **Magna Urbis** (github.com/dunkean/magna-urbis) | Other agent's job; compare there. Expected family: walled medieval city from Voronoi/planar graph (U) | See other note | See other note | See other note |

Reading: only the growth family (Weber, Lechner, Nishida, tensor fields with constraints) takes an existing town as input. Watabou, CityEngine 2001 and Magna Urbis-style generators create a town whole; to expand Balmora they would have to be run on the ring outside the seed and stitched, which breaks the road continuity unless the seed's edge roads are inserted as fixed constraints.

## 2. Prior art for Morrowind

### Mods (how they are made)
- **Tamriel Rebuilt** builds cities by hand in the Construction Set (TES Construction Set), whole-building statics placed one at a time, per-city teams over years; Narsis alone has 1,100+ NPCs (https://www.pcgamer.com/games/the-elder-scrolls/morrowinds-new-city-from-tamriel-rebuilt-is-bigger-than-any-playable-elder-scrolls-city-in-general-and-a-single-hero-made-its-exterior , V snippet). No procedural tooling found (U that none exists).
- **Beautiful Cities of Morrowind** (https://modding-openmw.com/mods/beautiful-cities-of-morrowind , V): overhauls nearly every settlement, hand-edited; it shows what players accept as a "bigger" Balmora but does not scale it ~10x.
- **Balmora Expansion** and Balmora-edit mods exist on Nexus (V names only, snippets; internals not read). All hand-made.
- Procedural Morrowind settlement tools: none found in this search (U; one web search only).

### Local prior art: Procedural Tamriel bundle (Liam Mello), ideas only
Extracted to `...\scratchpad\ptb\` and read, nothing run. It targets **Skyrim/Falkreath written into TES3-style JSON via tes3conv**, driven by Python (`src/procgen/*.py`), with run notes in `.opencode/runs/` (V, all below read from files).
- **Staged, hash-pinned pipeline** (`Documentation/guides/cityforge_*.md`): D-STAMP (extract whole-building stamps from the game: 54 eligible stamps, 736 members, each with door and hull) -> D-BRIEF census (`kit_brief.json`: building types, measured spacing and door-step priors; `region_palette.json`: ground textures per semantic surface) -> D-SITE survey (terrain, water mask) -> T1.1 city plan + validator -> T1.2 placement (houses-only: rigid stamp transform, terrain sampling, hull-overlap is the only hard spacing rule, `flatten_pad` request when a lot needs grading) -> T1.3 cityscape (terrain field edits: pads, road grades, cut/fill capped, then repaint ground textures and recompute normals) -> plugin author.
- **Frontage fit** (`frontage_fit.py`, "deterministic composition"): lots seated along authored roads by door-to-road-target, with group characters (compact_cluster, formal_square, sparse_outskirts) and constraints (max gap, side runs). So placement is "stamp doors face the street", not free packing.
- **Density finding** (`parcel-rebuild-run6` report, V): front rows on every road chop blocks into bands 500-1600 game units wide; 76 % of "free" ground cannot hold a 605-1636 x 410-1064 GU house in any orientation; fix is wider blocks, smaller units, or alley-first layout. Directly relevant: block depth must be a design input (road spacing >= 2 x house depth + alley).
- **Road system** (`road-search-rebuild` docket, V): grade-limited least-cost routing, bypass repair, Catmull-Rom smoothing, bridge siting at narrows. Same family as our `sf_worldgen` routes.
- **Failure lesson** (phase7 handoff, V): a run without visual gates produced buried dormers and floating windows and was rejected; they rebuilt a 36-entry house catalog with per-family visual acceptance before any frontage work. Lesson for us: gate the kit before the layout.
- **Terrain grading config** (`configs/cityforge/terrain_grading.json`, V): gate platforms, road grade max 0.125, blend margins 768-2048 GU, cut/fill caps 1024 GU.
Ideas to take: stamp = rigid assembly with hull and door; census of measured spacing from the real game; terrain edit as an explicit request list with caps; deterministic double build compared by hash. Not to take: Skyrim-specific kits, its Python/Blender stack.

## 3. What we already have (V from the repo)

- **Balmora anchor** (`target/worldgen/x10s/anchors-census.md:133,163`): 742 members, footprint 29,545 m2, extent 365 m, band 91 m, lift L 53.0 m, ring mismatch p95 134.2 m, 641 new dry samples (the river dried). Top members: `hlaalu_loaddoor_02` 56 (load doors), `ex_hlaalu_buttress_03` 44, `ex_hlaalu_dsteps_03` 41, `contain_crate_02` 37, `light_de_streetlight_01` 37, `furn_de_lightpost_01` 37, `contain_crate_01` 36, `ex_hlaalu_wall_01` 23, NPC 22, `ex_hlaalu_pole_01` 22. Pattern: each streetlight is a light plus a lightpost mesh pair; steps and buttresses are separate statics beside the building.
- **Anchor model** (`bevyrrowind/docs/specs/worldgen-adapter.md`, "Anchors" ~l.62, "Same map at 10x" ~l.102): human-made refs cluster (single linkage 30 m) into anchors moved rigidly (world = S*c + (p - c), + lift L); heights are stamped (smootherstep band) and ground textures taken from the anchor's source point inside it; roads exist only inside footprints; door destinations and scripts are not remapped yet (WG-A3b); settlements stay 1:1 "and will feel small (owner: expected; a settlement-growth system comes later)" (l.~103).
- **Ledger** (`docs/ledger/worldgen.md`, 2026-11-19 lines): "some settlements misplaced (Balmora must straddle its river)"; Odai dries inside the lifted town; worst ring p95 Dagoth Ur 459 m, Balmora 134 m. The spec says anchors must carry coastlines/rivers as forced constraints (bay for Vivec, river for Balmora, shore for Seyda Neen/Ebonheart).

### How a Morrowind town is built (from the census and spec; hlaalu kit knowledge beyond the census is U)
- **Whole-building prefab meshes** per house type (`ex_hlaalu_b_*` family), placed as static references with rotation; building = one mesh, one door; no modular walls. Add-on statics: steps (`dsteps`), buttresses, poles, walls (`ex_hlaalu_wall_01`) are separate placed meshes dropped around a building to seat it on slopes.
- **Load doors** (`hlaalu_loaddoor_*`, `ex_*_loaddoor_*`) link each building exterior to a hand-made interior cell (door record with destination cell and position). One per enterable building.
- **Street lighting**: `light_de_streetlight_01` (light record, no mesh) plus `furn_de_lightpost_01` mesh at the same place.
- **Containers, NPCs, signposts** (`active_signpost_01`) placed by hand; roads are a ground texture (LTEX) painted in the 7.3 m layer grid, not geometry; terrain heights are authored/shaped under the town.
- **Town = anchor cluster**: one cell group, rigid; per-district kit by culture (Hlaalu in Balmora, Redoran in Bal Isra, imperial/tree-root in Vos and Sadrith Mora per census).

### What a generator must output to look like Morrowind
1. Prefab placements (mesh id, position, yaw, scale 1) chosen from the culture's whole-building list; seating statics (steps, buttresses, walls, poles) per slope situation.
2. Road ground layer (LTEX per texel, paths and plaza surfaces) plus terrain height edits under roads, plazas, building pads (our stamp format v1: heights + weights).
3. Doors with interior links (see section 5), street lights as pairs, signposts at junctions and gates, props (crates, barrels, benches) at plausible density.
4. Water-aware layout: bridges and quays on the Odai, no building in a river or on its dried bed; heights consistent with the river channel forced by WG-A5.
5. Terrain continuity: old core remains a bit-exact stamp; new quarters grade into the generated land using the same band blend.
6. Culture and district rules: per-district building weights, density falloff outward (houses -> sheds -> fields), height-to-slope fit (terraces on slope).
7. Deterministic from a seed, export as records our adapter consumes (statics, doors, lights, ground layer, stamps), no TES3 ESP needed.

## 4. Rust crate ecosystem (crates.io API, queried 2026-11-23; V)

| Need | Crate | Version | Licence | Last release | Notes |
|---|---|---|---|---|---|
| Geometry types, booleans, buffer, OBB, convex hull, simplify | `geo` | 0.33.1 | MIT/Apache-2.0 | 2026-04-20 | georust, ~27M dl; has `MinimumRotatedRect` (U: name from memory); no half-edge |
| Constrained Delaunay / Voronoi / refinement | `spade` | 2.15.1 | MIT/Apache-2.0 | 2026-03-24 | ~25M dl; constrained edges give planar graph with faces iteration; good for block faces |
| Polygon booleans and offsets (fast, robust) | `i_overlay` | 9.2.0 | MIT/Apache-2.0 | 2026-11-15 | active; boolean ops, outline/offset support in `i_float`/`i_shape` family (offset API U) |
| Clipper wrapper | `geo-clipper` | 0.9.0 | ISC | 2025-02-08 | links C++ Clipper; prefer `i_overlay` |
| Polyline offset with arcs | `cavalier_contours` | 0.9.0 | MIT/Apache-2.0 | 2026-08-20 | good for road corridors and setbacks; ~150k dl |
| Polygon buffer via straight skeleton | `geo-buffer` | 0.2.0 | Apache-2.0 | 2023-06-02 | stale; fork `geo-buf` (0.1.x, 2025; U) |
| Straight skeleton | `straight-skeleton` | 0.2.1 | U | 2026-07-17 | i16 integer lattice only, ~200 dl: too coarse for metres at 18 m cells unless scaled; check before use |
| Medial axis / centerline | `centerline` | 0.14.0 | U | 2025-12-04 | codeberg; boostvoronoi based (U) |
| Half-edge / DCEL | `dcel` | 0.8.14 | U | 2026-06-23 | ~1.5k dl, young; alternative: build faces from `spade` or a 150-line own planar-graph face walker |
| Rectangle packing | `rectangle-pack` | 0.4.2 | MIT/Apache-2.0 (U) | 2021-05-03 | bin packing for textures, not lot fitting: unmaintained and wrong problem |
Gap: no crate found for lot subdivision (OBB recursive split, "skeleton-based lots") or for tensor-field tracing; both are small and own-code (rule 2 of CONTRIBUTING: reuse Bevy, then crates, then own with the reason). The repo already uses `sf_worldgen` for routes (check before adding `spade`).

## 5. Player-facing questions to settle (options shipped games chose)

1. **Interiors of generated buildings**: (a) none, doors absent and buildings are shells: Daggerfall's generic buildings use random interiors from templates by building type (U); GTA/Skyrim background buildings are shells; (b) locked doors: Skyrim city filler, Cyberpunk; (c) generated or reused interiors: Daggerfall (shared interior blocks per building type), Starfield procedural; (d) a fixed pool of hand-made interiors reused per kit type (Morrowind's own interiors are 1:1 per door; reuse needs cell instancing). Cost axis: cell count and save size.
2. **NPCs and services in new quarters**: (a) ambient crowds only, no services (Cyberpunk, AC Unity); (b) services pushed to the old core, new quarters residential; (c) generated shopkeepers, trainers and quest givers with names (Daggerfall, Tamriel Rebuilt hand-made); (d) travel network and faction/guild halls extended by data. Needs: dialogue and AI packages from templates, economy limits.
3. **New quarters vs quests and named places**: (a) quests keep original coordinates inside the 1:1 core (current anchor transform, WG-A3b); (b) quest NPCs may live anywhere in the town via a stable-ID marker not coordinates (Skyrim "alias"); (c) new content flagged as radiant/filler, never named. Also: do named streets, guard/crime region, fast-travel markers and door-to-interior spawn points follow the city boundary.
4. **Growth identity**: static at bake (Manor Lords / City Sample), or with story states (Tamriel Rebuilt-style "after-the-war" variants), or by player action (Manor Lords growth). Our rule: nothing bakes during play, so static at world bake.
5. **Town extent**: does the town wall/boundary and guard jurisdiction scale; does Balmora absorb neighbouring farms and the river banks (Balmora must straddle its river); does the 10x land mean 10x area or 10x linear (3x linear?). Pick before layout: this sets population and building count (Balmora anchor ~100-150 buildings, U count).

## 6. Recommended architecture

Name of the chain: **constrained growth from a fixed seed (Weber 2009 style) over a tensor-field street layout (Chen 2008), then planar-graph block extraction, lot subdivision, and rule-table placement of whole-building prefabs (CityEngine/City Sample "rules processor" pattern)**. Stages:

0. **Seed and constraints** (from our anchors): the anchor's hull, existing buildings (rigid, never moved), road texels inside, river/coast polylines from WG-A5 as hard constraints, slope map, other anchors' bands. Output: a planar graph of existing paths from the road LTEX plus door-to-street links (Nishida-style example extraction, U that road recovery from a texture is easy).
1. **Street growth**: tensor field = existing road directions (extended outward) + terrain-following basis + radial around the core; trace major then minor streamlines with min spacing = 2 x house depth + alley (cityforge density lesson); streamline stops at water, slope cap (cityforge max grade 0.125), other anchors; growth schedule by rings (Weber's time steps) so density falls outward. Technique: Chen 2008 + Weber 2009.
2. **Blocks**: half-edge faces of the street graph (`spade` constrained edges or `dcel`); subtract road widths with `i_overlay` / `cavalier_contours` offsets. Technique: standard planar-graph block extraction.
3. **Lots**: recursive OBB subdivision to size range per building type, frontage-first so every lot touches a street; or straight-skeleton strips for irregular blocks. Technique: Parish and Mueller 2001 / Watabou lot subdivision; Manor Lords plot model for organic look.
4. **District and culture**: ward-style Voronoi or radius rings labelling blocks (market, residential, slums, docks) from the seed's culture (Hlaalu, Redoran...) and distance; sets weights. Technique: Watabou wards.
5. **Prefab selection and placement**: per lot choose a whole-building mesh whose footprint hull fits the lot OBB and whose door faces the frontage; rigid placement with hull-overlap as the only hard rule (cityforge T1.2); seating statics (steps, buttresses, walls) from slope delta; props and street-light pairs at fixed pitch along streets; signposts at gates. Technique: CityEngine rule/City Sample Rules Processor, cityforge frontage fit.
6. **Terrain, ground, water**: height stamps for pads, road grades, plaza flats via our stamp format v1 (smootherstep band); LTEX road and plaza painting; bridges where streets cross the forced river channel. Technique: UE Landscape Patch / Houdini heightfield blend (already matched in spec).
7. **Population hooks and export**: door records with interior pool assignment, ambient NPC spawn markers, generated ids, quest-safe zone around original anchor; deterministic output with hash (cityforge double build). Technique: data adapter, no ESP.
8. **Review gates**: kit catalog accepted visually before layout runs (cityforge failure lesson); owner judges the look (CONTRIBUTING: no automated visual verdicts).
Crates: `geo`, `spade`, `i_overlay`, `cavalier_contours`; own code for tensor-field tracing, lot subdivision, rule table. Reuse check against Bevy is n/a (offline tool).

## 7. Not verified

- All Daggerfall details beyond the UESP record counts: how a city's block grid is picked and whether blocks are rotated (U).
- Manor Lords plot-fill algorithm and road growth (no developer source found).
- CGA/Parish 2001 paper details on seeding with existing roads; Lechner 2003 citation (from memory); Watabou algorithm details (source repo not read) and its licence.
- Tamriel Rebuilt tooling beyond "hand-made in the Construction Set"; any procedural Morrowind tool (single search).
- `i_overlay` offset API, `geo` `MinimumRotatedRect`, `straight-skeleton` licence and quality, `dcel` maturity; crate licences marked U.
- Balmora building count and whether `ex_hlaalu_b_*` meshes have per-mesh footprints and door positions available in our static index (the census lists top members only).
- Whether road LTEX texels can be vectorised reliably (needed for stage 0).
- Magna Urbis comparison cell is a placeholder for the other agent's report.
