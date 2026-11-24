# Settlements M2, Scrib side: what the game offers for "Balmora 2.0" (2026-11-24)

Research only, no code, no builds, game not launched. Question: for milestone 2 of `scrib-worldgen/docs/specs/settlements.md`
(arm B: a generated town on both banks of the 10x Odai, seen in the game), what does the Scrib side already do, and
what is missing? **V** = verified (file:line read, or a read-only `bevyrrowind` command run), **U** = unverified
(inference, memory of other games, or not checked). Paths are from `D:\Gamedev\Bevyrrowind`. Method: grep, then read
the lines; the existing survey `settlements-industry-2026-11-23.md` is not redone.

## 1. How the 10x world's settlements reach the game

**V, the path today.** `view world:<guide dir>` (`bevyrrowind/apps/bevyrrowind/src/world.rs:44`) opens the guide as
terrain (`GuideTiles::open`, `sf_worldgen/src/tiles.rs:246`, which also reads `stamps.json` and `layers.json`), loads
the whole content (`mw_boot_bevy::load`, `world.rs:89`), reads `anchors.json` (`AnchorTransform::read`, `world.rs:104`)
and, only if it exists, builds `WorldgenContent` (`moved_content`, `world.rs:164-199`). Every exterior placement of the
content goes through `world_content::remap` (`world_content.rs:104-150`): a placement whose key is an anchor member moves
rigidly (`S*c + (p - c)`, height + L); one not a member but inside a **stamped** anchor's footprint moves with that
anchor; everything else is dropped. The result is `WorldgenContent { placements, extra }` (`mw_exterior_bevy/src/worldgen.rs:81-88`)
and becomes an in-memory index (`content_index`, `worldgen.rs:~135`). There is no CLI or flag to add or drop anything:
the game's only inputs are the guide folder files and the content.

**V, what is carried.** `remap` moves all classes (statics, doors, lights, NPCs, containers: `Placement.class`,
`sf_world/src/placement.rs:67-79`); the census line prints the counts. **Door links are not remapped**: the spec says
"door destinations, travel and scripts are not remapped yet (WG-A3b)" (`worldgen-adapter.md`, "Known limits"), and
`WorldLinks::build(db)` (`main.rs:702`, `mw_doors_bevy/src/lib.rs:57`) takes no anchor transform. So in the 10x world a
moved Balmora door is a mesh whose link still sits at the 1:1 source position (U: that a click on it does nothing is
inferred from `mw_doors_bevy/src/lib.rs:246-256`, where a door placement id must equal `LinkEnd.door`; the link's
`frame` stays in source metres).

**V, one extra-pass seam already exists.** `WorldgenContent.extra: Vec<(String, Vec<Placement>)>` is "extra
placements from later passes ... ids must not collide" (`worldgen.rs:86-88`), used by the flora scatter
(`mw_worldgen/src/flora.rs:461`, `world.rs:196`). A settlement pass is one more entry in `extra`, indexed with
everything else (`worldgen.rs:~125-139`). Placements there are in the final engine frame already (`placed_at`,
`worldgen.rs:91-94`: `(x, height, -y)`).

**V, an existing record path for generated placements.** The framework has the record kind `sf.placement_group`
(`sf_world/src/source.rs:96-135`: `PlacementGroup { worldspace, placements: [GroupPlacement { key, base: RecordRef,
model, transform, class, flags }] }`; ids hash group id + key, so they survive edits, `source.rs:112-116`; "mods add
groups or patch other mods' groups", `source.rs:93`). `NativeRecords` loads `.ron`/`.toml` record files as a mod's
content (`sf_mods/src/native.rs:7-40`) and the kind is registered in the game's record db (`mw_world/src/worldspaces.rs:126`,
`sf_world::register_kinds`). So a generated "mod" of `.ron` placement groups plus `sf.door_link` records is the
honest "everything is a mod" carrier, and the settlements spec already assumes it ("a placements file becomes a
generated mod that Scrib loads like any mod", `settlements.md:~60`).
**But V: nothing in the game reads it yet.** `NativeWorldSource` (`source.rs:136-185`) has no user outside tests (grep
over `bevyrrowind/` and `superfargoth/crates`, no non-test hit), and `remap` would drop a group's placements anyway
(not anchor members, not inside a stamped footprint). U: whether a native `.ron` mod in the default profile loads in
`view world:` (`mw_boot_bevy::load(install, false)` was not traced).

**Least-privileged ways, by task.**
- (a) **Drop Balmora and its stamp: a new guide folder, not a game change.** V: the guide folder is the deterministic
  product of each step (spec "Pipeline"); `anchors.json` per anchor has `name`, `stamp` index, member keys
  (`anchors.rs:8-14`); `remap` drops what is neither member nor in a *stamped* footprint, so deleting the anchor entries
  drops their statics, doors, NPCs and the nature inside. V: the Balmora family in `target/worldgen/x10s/anchors.json`
  is three entries: `Balmora` (index 2, stamp 2, 742 members, L 53.0, source centre (-217.3, -191.6) m), `Balmora 2`
  (index 200, stamp 188, 8 members, L 0) and `Balmora 3` (index 513, point anchor, 2 members, L 143.3); other anchors
  such as `Bal Isra` are different towns. The terrain half must keep **stamp indices stable**, as the settlement tool's
  own ST-2 does: `guide_tiles` sets the dropped stamp's weights to 0 and leaves every index (`mw_worldgen/src/site.rs:595-612`),
  because `layers.json` grids and `anchors.json` refer to stamps by index (`layers.rs:7-9` of `sf_worldgen`). **Key V
  fact for the plan:** ST-2's site raster already assumes Balmora's stamp is gone (`site.rs:22-24`), so the guide the
  game shows must really have that stamp zeroed, or the plan's heights differ from the ground. An all-zero-weight
  stamp is legal (weights 0..1, `stamps.rs:113-115`). A small guide-writer step ("derive guide folder: drop anchors X,
  zero their stamps and layer grids' reach") is a data tool, no runtime change.
- (b) **Add placements from a file.** Options, least to most change: (1) a guide-folder file read by an extra
  `world.rs` pass into `WorldgenContent.extra` (one new reader, in `bevyrrowind/` since it names Morrowind records;
  closest to the flora seam, no framework change, but a file format of our own); (2) native `.ron` mod with
  `sf.placement_group` + `sf.door_link` records, read through the record db (framework kinds already exist; needs
  the one reader that feeds the groups into `extra` and a check that the mod loads in `world:`); (3) after X6 (below),
  the same group records as a `WorldSource` of the baked 10x worldspace. (2) is the end state; (1) is a legitimate first
  step only if its file is just a serialised `PlacementGroup` (so (1) becomes (2) by changing the loader, not the
  generator). The settlements spec contract says the placements file should become "a generated mod".
- **What X6 changes.** V: spec "Next": "X6 the 10x world as its own baked worldspace (index, static scene, precombine, far
  field) instead of the load-time scaffold `WorldgenPlacementStatics`" (`worldgen-adapter.md`, "Next"). The scaffold
  exists because the static scene and precombine are keyed by source cell (`worldgen.rs:10-14`, `exterior.rs:362-364`).
  After X6 the world has a real `WorldSource` (the pipeline `world.region -> world.page -> world.dir` has fingerprinted
  regions per group, `streaming.md:~405-420`): a `PlacementGroup` is exactly one region, so a settlement mod would be a
  region that changes only when its file changes. U: whether X6 has started (ROADMAP line 155 and the worldgen ledger
  carry no X6 status). Recommendation for the plan: write the generator's output as `PlacementGroup` data now so X6 does
  not change the producer.

## 2. Terrain grading

**V.** Yes: a planned-heights raster enters as a height stamp (format v1: `stamps.json` + `stamps.f32`, `stamps.rs:1-30`;
sample at a point is bilinear in heights and weights, runtime `mix(h_gen, h_stamp, w)` with runtime detail scaled by
`1 - w`, and `w >= 1` returns the stamp height exactly, `tiles.rs:411-431`). CPU twin and GPU fill share the formula
(`tiles.rs:40-44`; GPU words packed after the heights). Stamps are read from the guide folder independently of anchors
(`tiles.rs:255-257`), so a new town stamp needs no anchor entry. Validation: any `spacing_m > 0`, dims >= 2, under
2^28 samples, weights in 0..1 (`stamps.rs:100-118`).
**Weight raster is enough for a blend.** Weight 1 on pads, streets and plazas, a smoothstep down to 0 across the verge
reproduces the vanilla band (spec step 4: smootherstep). Stamps must not overlap where both have weight > 0 (spec
"Stamp format v1"; where rectangles overlap the larger weight wins, `stamps.rs:~21`): the new town must keep clear of
other stamps (U: distance of "Balmora 2.0" from stamped neighbours at the Odai reach was not checked).
**Resolution.** V: vanilla stamps are at the source 1.8288 m spacing (`stamps.json` of `x10s`: all 278 stamps, e.g.
Balmora stamp 304 x 304, 556 m); any spacing is legal, so the town can use 1 m or less. Level-0 terrain samples are 0.5 m
(`tile.rs:104-109`), so a 1 m stamp resamples bilinearly: a 5 m street is 5 samples wide, flat in the middle.
A sharp kerb is not possible at 1 m (bilinear ramp 1 m), only a ramp; walls and retaining faces must be static
meshes (the kit has `ex_hlaalu_wall_*`, `canal_*`, `buttress_*`, section 4). The heightfield cannot carry an
overhang or a vertical gorge wall.
**Gorge problem (V from the settlements spec):** the 10x Odai runs in a gorge with banks of 23-25 m (`settlements.md`,
ST-3e paragraph). A heightfield stamp can cut and fill and make terraces; the plan's streets must follow it, so the
grading is a design problem of the planner (`relief_scale 0.25`), not of the game.
**One source of truth for the ground: V.** The provider is `TerrainTiles` (`tiles.rs`), which feeds the clipmaps
(GPU fill) and the CPU `TerrainHeightsMap` (`sf_terrain_bevy/src/lib.rs:5, 615`); physics chunks read
`TerrainHeightsMap` (`sf_physics_bevy/src/stream.rs:23, 783`); groundcover reads the same height tiles and level-0
layer maps (`sf_groundcover_bevy/src/terrain_ground.rs:3-5`); `world_content` already uses `tiles.height` as the
ground for placements (`world.rs:178`). The far field is off in generated worlds (`exterior.rs:238`).
**Water: V.** The generated world shows one open-water plane at the guide's sea level (`world.rs:96`, `options.water =
WaterInputs::default()`; `site.rs:~30`); the known limit "rivers inside lifted towns run dry; water bodies at their own
height are WG-A5" (spec). So arm B's town sits on the 10x Odai only as far as the river is a below-sea-level channel;
a river above sea level does not exist as water in the game today (U: the exact Odai reach used by the planner was not
checked).

## 3. Street ground texture

**V, how the layers are built.** `layers.json` (format 1): a palette of layer record ids, a `global` grid (x10s: 832 x 768
at 73.152 m, blend 54.9 m) and per stamp a grid in world metres at any spacing, used wherever **that stamp's weight is
over 0.5** (`sf_worldgen/src/layers.rs:1-16`, `STAMP_LAYER_WEIGHT = 0.5`). Vanilla stamps use 7.3152 m texels
(Balmora's grid: 78 x 78, blend 5.49 m, from `layers.json`); the grid's `spacing_m` is free, so a town stamp's layer
grid can be 1 m. Texels hold palette indices (`u16`, `u16::MAX` = default layer), optional per-texel sRGB tint. The
tile's `LayerMap` is level 0 at 1 m samples (`tile.rs:23-24`; `terrain_ground.rs:6`), so **1 m is the finest
the ground texture can show**, and a 5 m street is 5 texels. The mean-albedo macro look comes from the same grid
(`layers.rs:~35-40`). A bake of the layer arrays must contain the layer: `with_exterior_layers` (`world.rs:119-175`)
greys palette ids missing from the bake's layer table.
**V, the road textures exist in the palette** of `x10s/layers.json` (131 entries): `wg_mainroad_01`, `wg_dirtroad_01`,
`wg_road`, `wg_cobblestones`, `ai_dirtroad`, `ac_dirtroad_01`, `al_road_01`, `gl_dirtroad`, `tx_ai_mainroad_01.tga`,
`tx_bc_mainroad_01.tga`, `ai_grass_cobbles`. The tool's own pattern list (`data/worldgen-site.toml`: `road`, `cobble`,
`path`) matches these.
**Cleanest way.** Per town one stamp whose **layer grid** is rasterised from the layout (street ranks and plazas
-> palette indices; verge and yards -> the surrounding dirt/grass ids; `NO_LAYER` where the generated world should
show through is *not* possible inside a stamp: inside w > 0.5 the stamp grid wins entirely, so the town grid must carry
the surrounding ground layers as well; U: the transition at the verge is sampled from the global grid under w <= 0.5,
a 73 m-texel grid, so the town grid should extend past the weight-0.5 contour). New palette ids are appended to
`layers.json`; palette order is stable per guide. The planner's verge shape is therefore in the weight raster; the
layer grid should be generated from the same layout on the stamp's lattice. Spec said "road LTEX stays only inside
footprints, roads re-routed later" (adapter spec "Anchors"): this is that later step for one town.

## 4. Balmora's building kit (what a "Hlaalu town" is made of)

Method: `target/release/bevyrrowind.exe` read-only (`find`, `uses`, `record`, `nif`), anchor census, `site.json`.
**Existing outputs.** `site.json` claim (`target/worldgen/sites/balmora-vanilla/site.json`) holds 742 footprints, all
`kind: other`, each only `{id, kind, polygon}`; the id is the placement key (32 hex), **no model name** (V:
`site.rs:15-17`, "`kind` is `other` for every footprint ... No doors (Milestone 2)"). `anchors-census.md`
(`target/worldgen/x10s/`, rows 133, 163) lists only the top 10 groups. So per-model counts and sizes were rebuilt from
`uses` (counts of references in the cells named Balmora; sums match the census: 44 buttress_03, 41 dsteps_03, 55 of the
56 loaddoors) and `nif` (visible-mesh bounds, "engine meters", model frame, not yaw-turned).

**Hlaalu prefab buildings** (whole-building meshes `ex_hlaalu_b_NN`, one static each; counts in the 4 cells of Balmora):
36 references of 26 distinct models, plus `ex_hlaalu_bal_01` (3), `bal_02` (4), `fb_01` (4), `striderport_01` (1):
about **48 buildings, 30 models**. Most models are used once or twice, i.e. Balmora is mostly unique prefabs.
Size w x d x h in metres (model frame; **h includes the part below the ground line**, see caveat):

| model | uses | w x d x h | model | uses | w x d x h |
|---|---|---|---|---|---|
| b_01 | 1 | 16.3 x 9.7 x 18.3 | b_16 | 2 | 12.8 x 8.0 x 11.0 |
| b_02 | 1 | 14.2 x 7.9 x 10.8 | b_17 | 2 | 16.5 x 8.0 x 11.4 |
| b_03 | 1 | 15.3 x 8.0 x 9.7 | b_18 | 1 | 8.0 x 15.6 x 11.0 |
| b_04 | 3 | 11.7 x 8.0 x 8.9 | b_20 | 1 | 12.8 x 8.0 x 11.0 |
| b_05 | 2 | 13.0 x 8.0 x 9.4 | b_21 | 1 | 14.2 x 9.7 x 12.0 |
| b_06 | 1 | 16.6 x 11.1 x 14.6 | b_22 | 2 | 14.2 x 8.0 x 11.4 |
| b_07 | 1 | 12.3 x 11.1 x 11.0 | b_24 | 1 | 23.7 x 11.1 x 15.9 |
| b_08 | 1 | 15.4 x 8.0 x 12.9 | b_25 | 2 | 12.5 x 8.0 x 11.0 |
| b_09 | 1 | 13.6 x 11.1 x 14.4 | b_26 | 2 | 17.5 x 10.5 x 11.0 |
| b_10 | 2 | 13.1 x 11.1 x 10.8 | b_27 | 1 | 14.4 x 10.4 x 11.0 |
| b_11 | 1 | 13.2 x 7.9 x 12.2 | b_28 | 1 | 11.7 x 8.0 x 9.3 |
| b_12 | 1 | 15.3 x 9.8 x 12.2 | bal_01 | 3 | 5.9 x 9.4 x 7.5 |
| b_13 | 1 | 7.8 x 10.2 x 18.3 | bal_02 | 4 | 5.9 x 9.4 x 7.5 |
| b_14 | 1 | 8.0 x 12.6 x 11.0 | fb_01 | 4 | 5.9 x 4.1 x 9.0 |
| b_15 | 2 | 7.8 x 10.2 x 18.3 | striderport_01 | 1 | 4.4 x 11.2 x 14.3 |

(b_19 and b_23 exist as records but are not placed in the Balmora cells.)
Typical lot front 8-17 m, depth 8-11 m; towers (b_01, b_13, b_15) 18 m including foundations.
**Caveat (U):** `nif` prints bounds symmetric about the origin (every `ymin = -h/2`), so the printout is probably the
box about its own centre or the models are authored about their mid-height; the ground-contact offset must be read
from the baked `model.info` (`sf_model::steps::ModelInfoStep`, via `mw_exterior_bevy::worldgen::model_infos`,
`worldgen.rs:~100-117`), not from `nif`. Not checked.

**Seating and street furniture (V counts):** `ex_hlaalu_buttress_03` 44 (0.7 x 2.6 x 8.9), `_04` 13 (1.1 x 6.1 x 8.1),
`_05` 9; `dsteps_03` 41 (2.9 x 1.5 x 2.9), `steps_03..07` 14 in total (5.3 to 12.6 m wide), `dsteps_01` 1; walls
`ex_hlaalu_wall_01` 23 (3.7 x 2.2 x 7.5), `wall_up_01/02` 8, `wall_gate_01/02/03` 12 (gate_03 7.3 x 2.2 x 11.4);
`ex_hlaalu_pole_01` 22; `ex_hlaalu_win_01` 17; bridges `ex_hlaalu_bridge_01` 10, `_04` 10, `_05/06/07` 2 each (3.7 to 4.3 m
wide, 3.7 to 9.4 m long); canal pieces `canal_01` 16 (3.7 x 4.8), `canal_03` 9 (7.3 x 5.8), `canal_02` 3, `canal_11` 4
(14.6 x 12.2); `furn_de_lightpost_01` 37 (+ 37 `light_de_streetlight_01` light records, census); railings
`ex_de_railing_01..03` 1 each; crates `contain_crate_01/02` 36 + 37 (container records, census). Load doors:
`hlaalu_loaddoor_ 02` (a space in the id: `bevyrrowind record DOOR "hlaalu_loaddoor_ 02"`), 56 in the census, mesh
`d\Hlaalu_loaddoor_ 02.NIF`, 1.8 x 3.0 x 0.1 m, a **separate DOOR reference** placed against a building's wall.

**Is there a kit export? Partly, V.** Usable now, read-only: `bevyrrowind find <KIND> <pattern>`, `uses <id>`,
`record <KIND> <id>`, `nif <path>` (node tree, materials, bounds "engine meters", incl. a hidden `Collision` node).
`worldgen-site` exports claim polygons (convex hull per reference, `site.rs:15-17`) but no model name, no door, no
per-model size. **Missing for a "kit export" command** (`bevyrrowind worldgen-kit`, Morrowind side, writes a
generic file that scrib-worldgen reads): per model id: record id, base kind, model path, footprint box from baked
`model.info` (w, d, h, and the height of its base above/below the origin), whether it is a prefab building (class
rules like `classify.rs`), per-model counts per anchor; and per building **door anchor**: for each building reference
the DOOR reference with a destination nearest to it, expressed in the building's local frame (offset, facing, so one
number per model, if consistent across its uses) plus the door record id and its destination cell name. U: the
door offsets are stable per model (Balmora's b_ models used 2-4 times, no check); U: a few buildings have several
doors (55 doors for ~48 buildings: some shops have two or a stair door).

## 5. Doors: how Scrib keys them and what a generated building's door could be

**V, how they are keyed.** Vanilla door references with a destination are paired into `DoorLink` records by position
and destination (`mw_world/src/worldspaces.rs:143-262`: `door_ends`, `link_doors`, `PAIR_DISTANCE_M` 8 m), each
`LinkEnd { worldspace, frame, arrival, door: Option<PlacementId>, extent }` (`sf_world/src/records.rs:187-209`). In play
the Use key on an activated door placement finds the link end whose `door` equals that placement id; a link end without
a door object is taken by **proximity and facing** (`mw_doors_bevy/src/lib.rs:132-157, 246-256`). Links are records
(`sf.door_link`, `records.rs:216`), so a mod can **define links and doorless link ends** as data, with no code, and a
`PlacementGroup` door placement gets a stable id from `PlacementGroup::placement_id(group, key)`. Interiors are
worldspaces named by their cell (`worldspaces.rs:~108-114`), `Partition::Single`.
**Things that bite (V/U).** (1) A link's exterior end is in the exterior's own frame, here the guide frame: fine for a
generated town. (2) A one-way link from the new door to an existing interior leaves the interior's own exit door
pointing at the **old 1:1 Balmora position** (exits come from the interior's DOOR references; remap does not touch them,
WG-A3b): the return link must also be defined (two-way link record, `DoorLink::exits`, `records.rs:219`), and it
overrides or competes with the vanilla pair (U: which wins when two links share an interior end was not traced).
(3) Door destinations carry a `PositionCell` in source metres.

**Options shipped games chose for generated or extra buildings (U: from memory of the games, no sources opened),
with cost, not a decision:**
1. **Enterable shared generic interiors.** Each lot's door goes to an existing vanilla interior of the right kind
   (shop, house, inn, tavern, guild) reused many times. Cost: one link record per building + the return link; the same
   room behind many doors (Skyrim and Oblivion reuse interiors for generic houses; Daggerfall uses a few block
   layouts for all houses). Needs a rule table kind -> interior cells; two buildings share one space only if the
   interior is not "instanced" (state shared: containers, NPCs, owned items).
2. **Instanced interior per building.** A copy of an interior cell per door (Skyrim Hearthfire / settlement mods,
   most "housing" systems). Cost: Scrib interiors are worldspaces built from the cell's references; copying means
   emitting records per copy (U: no existing copy path); state is separate; save size and bake size grow per copy.
3. **Closed facades.** The model with its door mesh but no link, or a locked / "no way in" message (Cyberpunk 2077
   non-interactable buildings, RDR2 and GTA facades, Assassin's Creed house fronts): cost near zero, just no
   `door_link`; the door mesh can stay (decoration). Consequence: only the original ~55 enterable interiors of
   Balmora lose their entrances and the new town has no enterable ones. A hybrid is common: a handful of key
   enterable buildings (guild, inn, shop), the rest closed.
4. **Doorless proximity links** (a portal frame on the wall, no door mesh). Cost: the same link record, no
   placement; the player walks through a wall opening (gate arches, market stalls). Fits "arch" entries more than houses.
5. **Quest/world interactions for old interiors** (U: a Morrowind script or quest that references the old Balmora door
   by id, e.g. the player's house or faction quests, breaks when the anchor is removed; not surveyed).

## 6. Collision, nav, groundcover, LOD for new statics

- **Collision: comes with the normal static path, V.** R0 statics and doors become `Fixed` colliders from the model's
  `collision.cook` shape (`mw_walkers_bevy/src/physics.rs:3-5`); terrain heightfield chunks follow `TerrainHeightsMap`
  (section 2). Needs: the models are in the bake (`bevyrrowind bake` cooks every model the records reference; a kit
  of vanilla models is already cooked; a pack model needs a bake). Terrain collision follows the graded stamp.
- **Navmesh: not in generated worlds, V.** `if !worldgen { mw_walkers_bevy::nav::add(...) }` (`exterior.rs:261-262`;
  "the baked navmesh is the content's ground"). So a new town has no NPC navigation, and neither does any moved anchor.
  Nothing to do for M2 unless NPCs must walk (a later nav bake on the 10x world).
- **Groundcover: does not come for free, gap.** GC-M4 builds exclusion regions from the index's grounded statics
  but reads **baked source terrain** to decide "reaches the ground" (`mw_exterior_bevy/src/groundcover_statics.rs:236-262`,
  `BakedTerrain::new`, `terrain_under`), while the generated world's ground comes from `GuideTiles`. U: how it behaves
  at 10x positions (statics outside the bake's extent have no ground: probably not excluded, grass through houses),
  not run. Also V: groundcover rules (LTEX -> species, `data/groundcover.ron`) read the layer maps (`terrain_ground.rs:3-5`),
  so a road-layer texel plants nothing if "layers with no rule (roads)" is respected (`groundcover.md:93-95`, U for
  the exact road ids in the 10x palette).
- **LOD / far field / precombine: off in generated worlds, V** (`exterior.rs:238, 362-364`): every placement spawns one
  entity (`worldgen.rs:10-14`). Fine for a town of ~1500 statics (Balmora 742 members). The town is invisible from a
  distance (no far proxies) until X6.
- **Lights / NPCs / containers:** V that `remap` moves them as placements; for a generated town the generator would
  emit `Light`-class placements for street lamps (census: a lamp is a light record + a lightpost mesh at the same place).

## What the plan can rely on

1. **V** The game takes placements in the final guide frame from `WorldgenContent.extra` (existing seam, flora uses it);
   ids must be unique (`worldgen.rs:86-88`).
2. **V** `sf.placement_group` + `sf.door_link` are existing framework record kinds loadable as native `.ron` mod files;
   no reader in the game yet (gap).
3. **V** A height stamp + per-stamp ground-layer grid, both at any spacing, are a finished mechanism with CPU/GPU twin; 1 m is
   the finest ground-texture sample, 0.5 m the finest height sample; physics, groundcover and render read one provider.
4. **V** Vanilla Balmora's removal is a guide-folder edit (drop 3 anchor entries; zero stamps 2 and 188, keep indices);
   the settlement tool's ST-2 site already assumes it.
5. **V** Door links are data records; a link end can have a door placement id or none.
6. **V** Navmesh, far field, static scene and precombine are off in `view world:`; statics collide via `collision.cook`.
7. **V** Kit: ~48 prefab buildings from 30 models (8-24 m front, 8-16 m deep), ~15 seating and wall types, bridges and canal
   pieces, lampposts; measures above; sizes are model-frame boxes.

## Gaps that need a slice

1. **Settlement reader**: `world.rs` pass: native `.ron` placement groups -> `WorldgenContent.extra`; tests with a tiny group.
2. **Guide-derive step** (drop anchors by name, zero their stamps, append a town stamp + its layer grid and palette
   ids): a `bevyrrowind`/`scrib-worldgen` data command with tests on index stability.
3. **Door links in a 10x world** (WG-A3b): at minimum, links of the new town; vanilla links of moved anchors remain broken.
4. **Kit export** command (section 4) incl. baked `model.info` boxes and door anchors per model.
5. **GC-M4 on a generated world**: read the guide's ground, not the bake's; plus a check that statics of the new town
   exclude groundcover.
6. **Stamp non-overlap and verge** between the town stamp and existing stamps (check at planning time).
7. Optional: X6 (baked 10x worldspace) for distance and far proxies; nav bake for NPC walkers; WG-A5 for river water above
   the sea plane.

## Owner questions (player-facing only)

1. **Doors:** which buildings in the new town can the player enter? (a) none, the town is a facade; (b) a few key
   buildings (inn, shops, guild halls, temple) behind reused vanilla interiors, the rest closed; (c) every building,
   interiors reused from a pool (the same room behind many doors); (d) every building with its own copy (separate item
   and NPC state, bigger saves). Cost order: a < b < c < d.
2. **What happens to the quests and NPCs of old Balmora** (vanilla quests, the player's house, guild halls, the strider
   port, Caius Cosades' house) that live in its interiors: re-attach them to buildings of the new town (needs the
   door-link and quest-key work), drop them, or keep old Balmora as a separate 1:1 place somewhere (the town the 10x
   world already has, "Balmora 2.0" being a second town)?
3. **Look:** are Hlaalu stone prefabs on every lot acceptable (variety limited to ~30 models), or must the new quarters
   mix in other kits (wood, imperial) per district?
