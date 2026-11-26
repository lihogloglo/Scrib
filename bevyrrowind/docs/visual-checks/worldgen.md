# Visual checks: worldgen

Open visual checks for world generation (WG-*). Index and how-to: [`../visual-checks.md`](../visual-checks.md).
Specs: `superfargoth/docs/specs/worldgen.md`, `../specs/worldgen-adapter.md`.

### RG-3: rock lab, procedural boulders + triplanar rock material (`bash bevyrrowind/tools/owner_launch.sh scene level-rock-lab`; built 2026-11-22)

Status: owner 2026-11-22: "correct rocks in the same family", but they read like river rocks, "smooth and eroded, big granite boulders": next = sharp/angular families (crease normals, feature-preserving meshing, volcanic classes). Was: awaiting owner. Scene `level-rock-lab` (`levels/rock-lab.ron`, 09:00 clear; cameras `wide` (start), `small`, `big`,
`slope`, `far`, `top`; `--rock-textures a,b` swaps the set). Four columns, one per class (x = -36 block, -12 platy, 12
cobble, 36 chipped), 10 sizes 0.5-12 m on a power law from near (z = 24) to far, plus 2 rocks on the 30 deg slope face
and 2 at its toe per class. Keys: **X** texture (`tx_rock_wg_01` Tamrielic, `tx_ashland_rock_03` and `tx_ac_rock_01`
SaintJ, `tx_bm_rock_01` MVR), **Z** class (all, then one), **R** new seed, **L** LOD tint (green 0, yellow 1, orange 2,
red 3), **M** wireframe, **I** info; console `set rockgen.boulder.<name> <v>` then R (e.g. `resolution 64`,
`lod0_error_voxels 0.1`, `chips 12`). Look for: do the four classes read as natural boulders (no blobs, holes, spikes,
pinched edges), resting on the ground, not floating; is LOD0 (~430 tris) faceted up close (`big`); does the triplanar
texture sit on the rock without visible seams or stretching on vertical faces, and is the texel density right next to
the plate; LOD switches (L, then `far`): dithered, no pops. Log: `rock-lab texture ...: albedo/normal/orm` per texture
(a texture with no cooked companion shows albedo only) and `rock-lab: N rocks generated in S s`. No colliders yet (RG-2).

### RG-8a: hero formations + proportion spread in the rock lab (`bash bevyrrowind/tools/owner_launch.sh scene level-rock-lab --camera forms`; built 2026-11-23)

Status: owner 2026-11-23 "good" (first look; tuning per formation open).
Scene: row at z = -135 beyond the plateau, x = -84..84: hoodoo 14 m, pinnacle 30 m, sea_stack 24 m, volcanic_neck 36 m,
arch 30 m span; cameras `forms`, `forms_low`, `arch`; R new seeds, Z steps through classes then formations.
Look for: each reads as its analogue (Bryce hoodoo, aiguille, Old Harry, Devils Tower, Delicate Arch), standing on its
foot, no holes or spikes; boulder columns now mix long, stubby and flat rocks (`set rockgen.boulder.aspect_spread 0` = old).
Known: arch rib faceted; pinnacle tip and neck joints soft at `resolution_max` 128 (192 sharpens).

### RG-4: sharp rock families in the rock lab (`bash bevyrrowind/tools/owner_launch.sh scene level-rock-lab --camera sharp`; built 2026-11-22)

Status: owner 2026-11-22: "they look amazing, excited to see more types"; edge snap **off** (`Q`, RG-3 bevels, rounded
edges) "much better" than snapped hard edges -> flip `rockgen.boulder.edge_snap` default to false (queued slice);
crease normals (`N`, `rockgen.boulder.crease_deg` 35) no verdict. Columnar skipped (budget bug) -> fixed 2026-11-22 (RG-4b): owner re-look 2026-11-22 "they're fine" (columnar (floor and chips now
rounded once across the columns, LOD0 ~1.4 k triangles instead of the raw net)); snap default now off. Owner asks for parallax on rocks/terrain (queued question).
Scene: 9 columns 22 m apart (eroded block/platy/cobble/chipped at -x; talus, jagged, columnar, scoria, slab at +x),
cameras `sharp`, `sharp_near`, `eroded`. Keys N crease, Q edge snap, Z class, R seed, X texture, I info.
Look for: flat fracture faces and crisp-reading edges on the sharp classes, scoria pits + darker tint, no holes or
spikes, eroded family unchanged. Known: 1-9 % wrong-way LOD0 triangles on sharp classes with snap on only (0 with snap off).

### WG-S2: 10x Vvardenfell erosion look test (images; `target/release/scrib-worldgen.exe run bevyrrowind/worldgen/vvardenfell-s2.toml`)

Status: partial (owner 2026-11-18): A1 and A2 show many parallel straight lines ("feels like a motherboard": D8 single-flow channels on smooth upscaled slopes); hillshades are hard to judge, owner asked for a 3D scene → WG-S2b routing fix + WG-V viewer. Open `target/worldgen/s2/sheet-*.png` (overview, red-mountain, ashlands,
bitter-coast, west-gash; 12 × 12 km crops, 20 m grid, light 315°/45°, true vertical scale). Arms left to right:
**A0** vanilla ×10 no erosion · **A1** stream power, uniform rock · **A2** per-region rock + Red Mountain as a
concave stratovolcano to ~3.3 km · **A3** A2 + talus/scree. Numbers and crop centres: `target/worldgen/s2/README.md`.
Look for: does any arm read as real land (branching valleys, sharp ridges, a real volcano) rather than blobs? Rate by
elimination (reject / maybe / keep). Is A2's Red Mountain shape right? Does per-region rock (A1 vs A2) or talus
(A2 vs A3) matter? Too little erosion overall? Known: only the macro stage (20 m); no mid-scale hydraulic detail
(WG-3) or runtime detail yet, so close-up texture is missing by design.

### WG-V: 3D terrain viewer (`owner_launch.sh worldgen-view <arm dirs>`; built 2026-11-18, `mw_viewers_bevy::worldgen_view`)

Status: viewer works (owner 2026-11-18, 144 fps); terrain verdict: A2/A3 rejected: "mountain pulled upward (vertices)", many evenly spaced grooves "like a bad definition problem", scale hard to read; "this ain't it at all, rethink entirely". Screenshot: owner ShareX bevyrrowind_c7Yd8shYSQ.jpg. Keys: WASD + mouse, Space/Q up-down,
Shift ×5, wheel = fly speed, **G walk** (1.7 m eye), **1-9 switch arm, same camera**, `[` `]` sun azimuth, `,` `.` sun
elevation, H HUD, F10 quit. Starts 15 km south of the peak at 500 m. Look for: the WG-S2 questions in 3D (real land or
blobs, the volcano's shape, parallel channels); also whether the viewer itself is good enough to judge terrain on
(shadows, fog, LOD pop at 6 / 24 km, no cracks). FPS is vsync-capped; fly mode keeps the camera above the ground, so
switching to a higher arm lifts it.
- Owner 2026-11-18 (viewer with real Etna/Teide/Iceland vs S2b B3, `--plain-terrain`; contour shader path bug: invisible
  terrain, fix running): real terrain has **less mid-detail** than ours; Morrowind-derived B3 is "more rugged (more gullies,
  valleys) for the same scale". → less dissection (wider valley spacing, higher channel threshold, finite time), volcanic
  regions shaped by lava-flow landforms, and real-DEM statistics as tuning targets (WG-M).
- Owner 2026-11-18 (contours fixed; real places + filter pairs + S4 C2/C3): Iceland/Hekla "definitely not"; Etna and
  Teide feel **VAST** (wide views, long slope from the sea); Vvardenfell = flat lowlands with the volcano surging out
  of them, **more vertical/dramatic than real life** (Etna the better ref for the pop); Teide's barrancos/fractures =
  foyadas. Erosion filter: etna 5 "looks amazing", 7 > 6. C2 parametric volcano "great great start": less
  cone-symmetric, gullies too simple/radial, higher, steeper, sub-peaks. Decisions: gentle apron coast → base,
  **summit ~5.5 km**.
- Owner 2026-11-18 (WG-VO V1-V4 vs C2, Etna, Teide): "very promising". Gripes, fine for now: the cones feel like
  "different cones merged together" (artificial); the littoral is very smooth and undeveloped; relief stops being
  interesting down the volcano's slopes. Next: original settlements in place + trees, for a screenshot.

### WG-G: generated island in the game renderer (G1 2026-11-18, G2 2026-11-19)

Launch: `bash bevyrrowind/tools/owner_launch.sh view "world:D:/Gamedev/Bevyrrowind/target/worldgen/vo/V3"` (camera
11 km south of the summit, 400 m up). Look for: gullies and ridges down to ~1-2 m when walking; console
`worldgen.filter false/true` shows what the filter adds; hitches while flying vs the `--off gputerrain` arm; clouds
(base ~3.1 km) with the 5.5 km summit through them. Placeholders: flat height/slope colours, no shore foam.
- Owner 2026-11-19 (G1, CPU tiles): runs, fps good, a lot of hitches, erosion not visible. → G2 awaiting owner.

### WG-X: Vvardenfell 10x, same map (2026-11-19, spec `worldgen-adapter.md` "Same map at 10x")

Launch: `bash bevyrrowind/tools/owner_launch.sh view "world:D:/Gamedev/Bevyrrowind/target/worldgen/x10v/<arm>"` (arms s55,
s45, s55c; bare base `target/worldgen/x10`). Look for: Red Mountain's height and prominence, the apron (gentle rise from
the lowlands), Morrowind's crater and foyadas still recognisable on top; north side is steep by geometry (coast 10 km).
- Owner 2026-11-19 (bare x10 base, no volcano, no content): "I think it works"; wants the volcano higher with a gentle slope → X1 arms.
- Owner 2026-11-19 (s55): "very nice like that"; s55 is the same-map base. Later: the blobbiness inherited from Morrowind's coarse heightmap (not now).
- Owner 2026-11-19 (x10s Balmora, all of WG-X1-X5, 200k flora): "alright"; textures super repeated, world extremely empty → more
  flora/rocks (`--set scatter.budget=N`, full density 7.2 M), anti-tiling + VCLR tint later. Log: 66-108 fps over 10 s
  windows once loaded, first ~25 s near 8 fps; 890 pending pipelines at the 14.7 s reveal cap (alpha-blend PBR pipelines
  up to 5 s each in the driver), 23,028 shown entities, 707 StandardMaterials, 2.07 GB of textures.
- Owner 2026-11-19 (x10s, flora 1M then full 7.2 M, end of session): 26 fps; still empty; no proper forest (trees not
  snug together), rocks all one size, "sprayed some assets over the terrain like little bits on a donut"; relief still
  smooth and blobby (next big thing); some settlements misplaced (Balmora must sit on both banks of its river).
  Next: study the Chlorophyll engine for vegetation at scale.

### WG-X7a: Terrain ground texture anti-tiling (Mikkelsen 2022 hex-tiling, 2026-11-20)

Status: **confirmed (owner 2026-11-20)**: "world of difference", all live toggles (`terrain.hex_tiling`, `.rot_strength`, `.contrast`, `.scale`) work. Kept on by default.
Launch: `bash bevyrrowind/tools/owner_launch.sh view "world:D:/Gamedev/Bevyrrowind/target/worldgen/x10s" --hour 12` (also applies to classic Seyda Neen / Balmora).
Live A/B in console (`~`): `set terrain.hex_tiling off` vs `set terrain.hex_tiling on`; also live: `set terrain.hex_tiling.rot_strength 0` (offset-only vs full rotation `1`), `set terrain.hex_tiling.contrast 7`, `set terrain.hex_tiling.scale 1`.
Look for: repeating tile grid gone across large 10x single-layer expanses (hillsides around Balmora, Ashlands, Red Mountain apron); lighting on bumpy rock/dirt normals stays consistent (no inverted or wobbling highlights from rotated tiles); no seams at hex borders or when crossing 1024 m floating-origin shifts; height-blended layer transitions stay crisp. Check cobblestone/paved roads in Balmora under `rot_strength 1` vs `0`.

### WG-SHARP: Sharp landforms + cliff rock shading + Option A (WG-X7b/c + Phase 3a/3b, 2026-11-20)

Launch on `target/worldgen/x10cs` (dramatic) or `target/worldgen/x10c-sharp` (sharp) and compare vs `target/worldgen/x10s` (smooth base).
- Owner 2026-11-20 (v1 `x10cs` / `x10c-sharp`): "both look nice", spotted seams between terrain tiles / near range → fixed clipmap edge fades (`layer_edge_fade`, morph-blended `terrain_curvature`, `terrain_normal`/`macro_albedo` border cross-fades) and added Option A (per-region geology, routed volcanic lava lobes, `VCLR` macro tint `layers.tint`).
- Owner 2026-11-22: prefers the vibe of the 10x + apron base (`x10s`) from before the cliff change; sharp passes read spiky, not varied; cliffs not real cliffs; terrain not walkable. New bug: **terrain visibly changes shape as the camera gets close** (also in earlier tests). Audit 2026-11-22: `x10c-sharp`/`x10cs` have 49/56 % of land ≥ 35° (Etna 1.8 %, Teide 6.2 %), largest walkable (≤ 46°) region 14/5 % of land vs 60 % on `x10s`; square-grid seam bug in `cliffs.rs` scarp dip. Next: rework (placed landforms, geology-driven runtime detail), LOD-change diagnosis.
Live console (`~`) A/B toggles:
- Shader cliff rock & crevices (`WG-X7b`): `set terrain.cliff.enabled off` / `on`, `set terrain.cliff.slope_start_deg 28`, `set terrain.cliff.slope_full_deg 45`, `set terrain.cliff.crevice_strength 0.45`.
- Runtime sharp strata/Voronoi/ridges (Phase 3a): default now `worldgen.sharp 0` (off, owner 2026-11-22); `set worldgen.sharp 1` (full profile) / `2` (aggressive) to compare; `set worldgen.filter false` / `true`.
Look for:
- **Clipmap / range seams gone:** fly 300–1200 m above terrain and sweep across the ~900–1000 m near-splat ring and clipmap level borders — no hard square lines in normals, shading, crevices, or near-to-far albedo transition.
- **Per-region geology:** Bitter Coast stays low/marshy (quiet scarps), Ascadian Isles has gentle terraced rolling hills, Grazelands reads as flat-topped stepped mesas, West Gash has towering canyon escarpments, Azura's Coast / Sheogorad have sheer rocky sea-cliff headlands.
- **Volcanic lava lobes:** Molag Amur / Ashlands / Red Mountain slopes show downhill-routed dark rocky lava ribbons with raised lateral levees and steep aa fronts.
- **Morrowind `VCLR` macro tint (`WG-X7c`):** subtle regional soil/road/shadow colour variation carried onto both far macro and near ground splats.
- Settlements still sit cleanly on their stamps without floating/burying; no CDLOD culling holes or wet/dry sea glitches.

### LF-2b: eroded fault scarps, taller and fewer, no dents on faces (2026-11-22, lf-2b, spec `landforms.md` As built LF-2b)

Status: owner 2026-11-22, spots 1-3: Ashlands 143 m reads as a natural rise ("alright"); Ashlands 135 m and Molag Amur 100 m "nice and very vertical" (not quite vertical) but very smooth, wants hard edges (cliff bands RG-5/6, lip restore LF-5, assets); Grazelands 33 m: smooth from a distance, close up the face gets a jagged sawtooth (grass teeth along the top and bottom of the rock face; owner screenshots ShareX bevyrrowind_9WHYE0MHrB.jpg far, bevyrrowind_qjvL1Kl8CS.jpg near): bug diagnosed 2026-11-22 (ledger worldgen): full-strength runtime gullies on a face whose base slope (max 38 deg) never reaches the 40 deg steep fade, and the slope-driven cliff rock outlines the gully walls; fix slices queued (material mask from base slope, gully cap on low scarps). Launch `owner_launch.sh view "world:D:/Gamedev/Bevyrrowind/target/worldgen/lf2b-scarps" --hour 12`
(A/B: `lf2-scarps` = the LF-2 blades; `x10s` = the base). 32 faults (LF-2: 80), throw 85-143 m in the Ashlands, 85-100 m
West Gash / Molag Amur, 25-33 m Grazelands. Faces cut into triangular facets by ravines (chutes) every 50-300 m, a debris
cone below each ravine mouth, face 60-75 deg by rock, height varying along the face, long tapered ends; this world's
`filter.toml` turns the gully dents off on slopes over 40-50 deg. Places (`--camera` eye, target; engine X, Z):
- Ashlands, 143 m block (capped sediment over tuff): `--camera "179,+40,-1003,-209,+86,-687"`.
- Ashlands, 135 m (tuff over tephra): `--camera "-34,+40,-8980,-105,+81,-8485"`.
- Molag Amur, 100 m basalt face: `--camera "5867,+40,4102,5580,+60,3692"`; West Gash 91 m: `--camera "-6393,+40,-6884,-6893,+55,-6898"`.
- Grazelands, low 33 m scarp (should read gentle): `--camera "9957,+40,-17485,10417,+20,-17681"`.
Look for: does a scarp read as an eroded range front (facets and ravines, fans spreading on the plain) rather than a
wall or a blade; does the height vary along it; are the rock faces smooth of gully dents (gentler ground beside them
still gullied); can you walk up a ravine/fan. Known: rocks still follow fault footprints in lines (diagnosed, not
fixed: flora slope classes); guide spacing 18 m softens the facet edges (runtime lip restore is LF-5).

### LF-2: placed fault scarps on the x10s base (2026-11-22, lf-2, spec `landforms.md` As built LF-2)

Status: owner 2026-11-22: "thin blades of very vertical (and dented) terrain, long and not very high"; rocks "placed badly" along lines elsewhere. Read as walls, not cliff landforms → LF-2b. Was: awaiting owner. Launch `owner_launch.sh view "world:D:/Gamedev/Bevyrrowind/target/worldgen/lf2-scarps" --hour 12`
(A/B: same with `target/worldgen/x10s`). 80 scarps on harder-rock outlines, the rest of the land as `x10s` (97.7 % untouched).
Each: flat top tilting gently back (6 deg) from a lip, steep face, a 33 deg scree apron, then the untouched plain; throw 20-47 m.
Places (`--camera` eye, target; engine X, Z = guide x, -y):
- Grazelands, 47 m scarp in two en-echelon segments: `--camera "11382,+60,-11025,11077,+5,-10853"`; relay ramp at (11341, -10416).
- Azura's Coast, relay ramp between two offset 47 m faces: `--camera "16627,+60,-10859,16452,+5,-11160"` (walk up it from the low side).
- Ashlands, 43 m single face (tuff over tephra), tapered tips: `--camera "-1209,+60,-6684,-968,+5,-6430"`; Molag Amur 39 m: `--camera "13325,+60,6298,13672,+5,6252"`.
- Ashlands, a repair ramp (279 m graded corridor N-S, 28 deg, cut/fill <= 25 m): `--camera "-998,+60,-10580,-1298,+5,-10580"`.
Look for: does a scarp read as a cliff band on calm ground (step, lip, apron) or as a bump; are tips and relays walkable;
no new spikes or seams, settlements and coast unchanged. Known: at guide spacing faces show at 40-50 deg (sharp lips need
the runtime lip restore, LF-5); the base's own steepness (33 % of land >= 35 deg) is untouched by design (LF-calm).

### WG-LOD: terrain shading no longer changes at mesh LOD switches (2026-11-22, wg-lodfix)

Status: **confirmed (owner 2026-11-22)**: detailed pass on x10s with the 6-point checklist: "everything looks great". Kept on by default. Launch `owner_launch.sh view "world:D:/Gamedev/Bevyrrowind/target/worldgen/x10s" --hour 12`
(Balmora ring, noon sun). Normals, crevice darkening and cliff rock now come from the finest height level resident
around the camera (`shade_levels`), not the mesh LOD. Look for: fly, then walk, toward hillsides and cliffs from ~2 km
down to 20 m: rock patches, crevice darkening and slope lighting should no longer "grow in" or snap at fixed distances
(48, 96, 192, 384, 768 m); detail still sharpens gradually with approach (mip-like), and the silhouette still morphs.
No seams at clipmap borders, no shimmer far away. A/B live: `set terrain.shade.decoupled off` (the old coupling) / `on`;
`set terrain.shade.blend_texels 16|96` (cross-fade width); `set terrain.cliff.curvature_scale_m 1|4` (crevice scale);
`set terrain.cdlod.morph_start 0.78` (longer morph; cracks possible on slopes, watch for sparkle seams). Known: rock
cover still thins with distance (coarse levels smooth slopes); classic Morrowind terrain uses the same shaders.

### TN-1: Balmora 2.0, medina street wall of Hlaalu buildings on the 10x terrain (2026-11-24, st-m2, scrib-worldgen `settlements.md` Milestone 2)

Status: **awaiting owner** (exploratory first look: no grading, no paving, no bridge yet).
Launch: `owner_launch.sh view "world:D:/Gamedev/Bevyrrowind/target/worldgen/x10s-b2" --hour 12 --camera -2450,+90,1850,-2690,+5,1650`
(`x10s` minus old Balmora; town mod `x10s-b2/mods/balmora2-medina`: 267 Hlaalu buildings, 1403 Hlaalu wall pieces,
13 gate arches, planned by Magna Urbis `medina` on relief scaled 0.25, each piece seated on the real ground at its
vanilla height). Log line: `worldgen town: worldgen.town.medina: 1683 placements, 0 unresolved bases`.
Look for: do houses + walls read as a medina street wall in Balmora's style; houses standing on the ground (not
floating, not buried); doors facing streets; walls closing frontages; arches at lane mouths. Known and expected:
streets are bare generated ground (no paving or grading), the trench walls have no lots (too steep), no bridge
over the Odai, no doors open, streets were planned on flattened relief so some climb steeply.

### TN-3: Balmora 2.0 on the low-shore terrain, Hlaalu-sized plan, streets and garden trees (2026-11-24, world10x)

Status: **works** (owner 2026-11-24 evening: "works"; many tweaks to come).
Launch: `owner_launch.sh view "world:C:/cargo-target/scrib/worldgen/x10hs-b2w" --hour 12 --camera -2480,+70,2120,-2866,+8,2040`
(`x10hs`: world-wide height curve HC-1, plain 6-8 m over the Odai; town = Magna Urbis medina streets + `hlaalu` culture pack
(kit house sizes, minFrontage 8.4 m), fit `match` (one kit house per MU building, set back up to 6 m, back sunk up to 2 m,
towers last): 734 of 813 plot lots, 801 buildings; MU streets as ribbons (flagstone main, cobble streets, dirt lanes);
temporary garden scaffold: 182 trees, 1117 shrubs on empty plots and slivers (MU garden lots left open as fields).
Log lines: `worldgen roads: guide ...`, `worldgen roads: worldgen/roads/balmora2-medina.json 496 segments`, `worldgen town:`.
Look for: buildings reaching the river front on both banks; houses packed along streets, doors to the street; backs sunk on
slopes (no floating foundations); street ribbons on the ground, not through houses; trees only in empty lots.
Known: regional roads don't join the town (S-2d diagnosis), bridges are gaps (no decks), no corner-lot side doors, squares unpaved.

### TN-4: the regional road meets Balmora 2.0's streets (2026-11-26, worldgen-5, S-2c2)

Status: **awaiting owner**.
Launch: `owner_launch.sh view "world:C:/cargo-target/scrib/worldgen/x10hs-b2w" --hour 12 --camera -2915,+35,2486,-2906,+2,2427`
(the south-west road, 60 m before its join; the two other joins: east road from the south at engine x,z -2501,2422, from the
north at -2355,1897. Regional roads are routed to the town centre, then cut where they first come within 8 m of a town
street and snapped onto it; the town is the chain layout of 2026-11-26: 2335 placements, 443 streets.)
Log lines: `worldgen roads: guide ... 178 segments`, `worldgen roads: worldgen/roads/balmora2-medina.json 443 segments`,
`worldgen town:` (0 unresolved bases).
Look for: the road ribbon runs up to a town street and stops on it (no gap of bare ground); from the join inwards one
ribbon only (no second ribbon on top of or beside the street); the same at the two other joins.
Known: the 4 m dirt road meets a 1.7-2.5 m dirt lane (width step at the join); bridges are gaps; A/B = the old files in
`x10hs-b2w-pre-s2c/` (`roads.json` + `mods/`).

### TN-2: Balmora 2.0 with rivers, lakes and regional roads (2026-11-24, world10x, Magna Urbis water + roads)

Status: **rivers parked for a refactor** (round 3, 2026-11-24, owner: speed good after G-W7, but the river is a ribbon
hovering over the ground, very flat (no visible normals, no particles); later: carve a bed for the mesh, flow reacting to
objects and walking characters). Town: owner reviews Magna Urbis's own plan first (no fit work until then). Round 2: flowmaps
pure white (speed foam), town half empty. Round 1: town bad/unfinished, rivers hover, lakes short of the shore.
Launch: `owner_launch.sh view "world:C:/cargo-target/scrib/worldgen/x10s-b2w" --hour 12 --camera -2450,+90,1850,-2690,+5,1650`
(`x10s-b2` + Magna Urbis water chain, mode none: 186 lakes, 95 rivers 9-17 m wide; 142 regional road segments; town
`mods/balmora2-medina`: Balmora + Suran Hlaalu kit only, 235 houses fitted to whole lots, 33 `ex_hlaalu_bal_01/02` towers at
wall corners, 2478 Hlaalu wall pieces, 15 gates). Log lines: `worldgen water: 186 lakes, 95 rivers` (+ river speeds),
`worldgen roads: 142 segments`, `worldgen town:`, flora summary `towns (... excluded)`.
Look for: rivers in their beds (round 1: 27 m median hover, now 0.6 m), lake edges under the shore (41 % -> 2.8 % of
outline points short), rivers moving with the flow (flow-advected normals, speed from slope, foam above 1.5 m/s), no rocks
or logs in the town, no Imperial towers, Balmora towers at corners. Known: roads stop at the town edge, bridges are gaps,
no deck on the Odai crossing, lake/river optics borrow the coastal look, terrain detail may poke through narrow beds.
Note 2026-11-26: rounds 1-3 were seen on `x10s-b2w` (launch line above); the chain's current folder is `x10hs-b2w` (HC-1 low shores), where round 4 launches after WG-W1g (`worldgen-hydrology.md` "Verification").
