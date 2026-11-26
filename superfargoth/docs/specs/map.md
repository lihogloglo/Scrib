# Map: world map, reveal, places, pins, compass, guidance (`sf_map`)

Status: **spec (MAP0, 2026-11-26)**, nothing built. Lane roadmap: `docs/roadmaps/travel.md` (decision 10, Stage 2
MAP1-MAP5 and TV16, Stage 6 TV19/TV20 pointers, owner Q3). Related: [travel.md](travel.md) (gates, `Undiscovered`,
TV16), [nav.md](nav.md) "Route graph" (NV12, roads and journeys), [terrain.md](terrain.md) "Data model" (height
pyramid, macro albedo), [bake.md](bake.md) (steps, store, budget), [ui.md](ui.md) "Screens" (the stock map screen),
[save.md](save.md), [tunables.md](tunables.md). Game side: "Adapter contract" below, mapped in
`bevyrrowind/docs/specs/ui-adapter.md` "Map (MAP5)".

> **Agent summary.** A map layer per worldspace. The look is **baked once** from data the world already has (terrain
> heights -> hillshade, layer or region colour, water mask and coast, route-graph roads, place footprints) into a
> BC7 KTX2 **tile pyramid**, styled by a `MapStyle` record (painted, paper, satellite: data, never code). Place names
> are **live labels**, never baked. **`MapReveal`** is per-player gameplay state: a fog bitset per layer (revealed by
> an exploration radius, by `RevealRecord`s such as bought maps or guides, by scripts), the set of discovered
> `MapPlace`s (the one discovery truth travel reads) and the player's pins. The map screen streams tiles like a
> slippy map and draws fog as a mask in a `UiMaterial`. The compass strip shows places (quest markers are an
> option a game may turn off). Guidance line (TV20) and auto-ride (TV19) share one `sf_nav` journey query.

## Goal

A world map for any pack and for a generated world 10x the source size, with no hand-painting and a bake of
seconds; reveal by exploration and region reveals, saved per player; places discovered on approach; pins; a compass;
one route for guidance, auto-ride and opt-in map travel. Non-goals: the local map ("Later"), quest objectives
(dialogue-quests lane), travel and its gates ([travel.md](travel.md)), route search ([nav.md](nav.md)), a 3D map.

## Owner decisions

| Date | Decision | Where |
|---|---|---|
| 2026-11-25 | Map = baked tile pyramid, data style profile, saved `MapReveal` fog, pins | travel roadmap decision 10 |
| 2026-11-25 | Reveal: exploration plus guides and bought maps in Modern; exploration only in Classic (Q3) | Reveal sources |
| 2026-11-25 | Morrowind compass without quest markers; quest markers `None` for Morrowind, `Compass` for the framework | Compass |
| 2026-11-25 | No free teleport by default: "travel anywhere discovered" (TV16) is opt-in, off, gated | Map travel |
| 2026-11-11 | Generic renderer, packs are data profiles converted at bake | Map bake |

## Reuse check (CONTRIBUTING rule 0)

Checked 2026-11-26 (Bevy 0.20 sources in the cargo registry; crates.io/GitHub search).

**Bevy 0.20 gives the whole screen:** `ImageNode` (`rect`, atlas; `bevy_ui-0.20.0/src/widget/image.rs:18`) and
`UiTransform` for tile quads under a clipped `Node`; `UiMaterial` + `MaterialNode<M>`
(`bevy_ui_render-0.20.0/src/ui_material.rs:103`, `:182`) for the tile + fog shader; `AssetServer` with the KTX2
loader through our `baked://` source (bake.md "Loading"); `bevy_picking`, `bevy_input_focus` and `Text` for pins
and labels; an image render target + `ViewportNode` for the later local map. It has no tile pyramid or map widget.

| Need | Crates (2026-11-26) | Choice |
|---|---|---|
| Slippy viewer, minimap | `bevy_slippy_tiles` 0.10, `bevy_geo_tiles` 0.1 (Bevy 0.17, Web-Mercator tiles from a server); `bevy_ecs_tilemap`, `bevy_fast_tilemap` (sprite grids); no 0.20 minimap crate | **Own** (~300 lines): wrong domain, old Bevy, grids are not raster pyramids |
| Hillshade, downsample | `terrand-rs` 0.1, `image-pyramid` 0.2 (licences not stated) | **Own**: Horn (1981) gradient + Lambert is ~40 lines; 2x2 box filter in linear space, fixed order (determinism) |
| AA strokes and fills | `tiny-skia` 0.11.4 (BSD-3, pure Rust, in `Cargo.lock`) | **Reuse** |
| BC7, KTX2 write, decode | `intel_tex_2` 0.5, `ktx2` 0.5, `sf_texture::ktx2_out`, `image` 0.25 (all in tree) | **Reuse** (terrain and de-light path) |
| Reveal bitset | `fixedbitset` 0.5.7 (MIT/Apache, in `Cargo.lock`), `bitvec`, `roaring` | **`fixedbitset`**; the save writes own run lengths |
| Route | `sf_nav::journey` (NV12e) | **Reuse** |

All are MIT, Apache-2.0 or BSD-3: GPL-3.0-or-later compatible. New crates: `sf_map` (Bevy-free core),
`sf_map_bake` (Bevy-free bake steps; holds `tiny-skia`), `sf_map_bevy` (thin plugin); the screen and compass live
in `sf_ui_screens`. `sf_map` and `sf_map_bake` go in hakari `final-excludes` (CONTRIBUTING "Building").

## Industry comparison

Sources are memory unless marked; the building slices verify what they copy.

| Game | Map image | Reveal | Places and pins | Guidance |
|---|---|---|---|---|
| Morrowind | drawn from land heights; local map per cell | explored cells; local fog per cell | visited named cells; `ShowMap`/`FillMap` | none (journal text) |
| OpenMW | generated from land data; local map rendered per cell | explored cells; fog saved | visited locations; custom notes | none |
| Oblivion | painted, fully visible | none | proximity discovery; one custom marker | quest markers |
| Skyrim | 3D from terrain, clouds | none | undiscovered hint on compass; one custom marker | quest markers |
| Elden Ring | painted | blank until the region's fragment | 100 icon markers; co-op beacon | none |
| RDR2 | painted paper | fog by radius | waypoint | GPS road line; auto-ride |
| BotW / TotK | topographic | towers reveal regions | pins, stamps, Hero's Path | pins only |
| Witcher 3 | painted from terrain, visible | none; `?` points | custom pin, signposts | minimap route |
| KCD2 | hand-drawn period map | none | markers can be hidden | sparse |
| Ghost of Tsushima | painted | fog clears by exploring | pins | guiding wind |

**What we take.** A map generated from data (Morrowind, OpenMW, Skyrim, Witcher 3: a 10x generated world cannot be
hand-painted), styled painted or paper by data (Elden Ring, RDR2, KCD2); radius fog on a coarse grid (RDR2, GoT);
region reveals (Elden Ring fragments, BotW towers) as `RevealRecord`s; proximity discovery (Oblivion, Skyrim); icon
pins with optional text (Elden Ring, BotW); optional markers (KCD2, Elden Ring); one route for the route line,
a wind-style hint and auto-ride (RDR2, Witcher 3, GoT). **Not taken:** Skyrim's 3D map (a second far-field render
while paused); a later style could add it through `ViewportNode` on the same data.

## Data model (`sf_map`, Bevy-free)

```rust
pub struct MapLayerId(pub RecordId);  // a worldspace (later an interior); stable across mods
pub struct MapLayer { id: MapLayerId, bounds_m: Rect2, reveal_cell_m: f32, m_per_px: f32, style: RecordId } // `sf.map.layer`
pub struct MapPlace {                  // `sf.map.place`
    id: RecordId, layer: MapLayerId, name: LocKey, match_name: String, // match_name: language-free, for prefix filters
    kind: RecordId, pos: Vec2, footprint: Option<Vec<Vec2>>, region: Option<RecordId>, discover: DiscoverRule,
}
pub enum DiscoverRule { Always, Near { radius_m: Option<f32> }, Inside, Scripted }
pub struct MapRegion { id: RecordId, layer: MapLayerId, name: LocKey }   // geometry: `map.regions` product
pub enum RevealShape { Disc { at: Vec2, r_m: f32 }, Polygon(Vec<Vec2>), Region(RecordId), PlaceFootprint(RecordId), Layer }
pub struct RevealRecord { id: RecordId, layer: MapLayerId, shapes: Vec<RevealShape>, places: Vec<RecordId> } // `sf.map.reveal`
pub struct MapReveal { players: BTreeMap<PlayerId, PlayerMap> }          // host state, saved
pub struct PlayerMap { fog: BTreeMap<MapLayerId, RevealGrid>, places: BTreeSet<RecordId>,
                       applied: BTreeSet<RecordId>, pins: PinBook, last_stamp: Option<(MapLayerId, Vec2)> }
pub struct RevealGrid { origin: Vec2, cell_m: f32, w: u32, h: u32, bits: FixedBitSet, revision: u32 }
pub enum MapCommand {                  // the only way to change MapReveal; host applies on the fixed tick
    Explore { player, layer, at: Vec2 }, Apply { player, record }, Reveal { player, layer, shape },
    Discover { player, filter: PlaceFilter /* Id | NamePrefix | Layer | All */ }, Pin(PlayerId, PinOp),
}
pub struct PlaceDiscovered { player: PlayerId, place: RecordId, cause: DiscoverCause }          // message
pub trait DiscoverySource { fn is_discovered(&self, player: PlayerId, place: RecordId) -> bool; } // MapReveal; travel reads it
```

**Layers and overlays.** `MapLayerId` names a surface: one per exterior worldspace now, interiors later (local map).
Places, roads, regions, pins, fog, players and the route are **overlays** drawn on a layer, toggled per screen
(`MapOverlay` flags), not layers of their own. Roads and coast are baked into the tiles (static); places, regions'
names, pins and fog are live.

**Place kinds** (`sf.map.place_kind`): icon (VFS path), label class (CSS `map-label.<class>`), label priority (u8),
zoom range, compass rule (`Never | Discovered | NearHint`), whether discovering it reveals its footprint.

## Reveal and discovery (MAP1)

- **Grid.** One `RevealGrid` per (player, layer), cells of `reveal_cell_m` aligned to `bounds_m`. A cell is revealed
  when its centre lies inside the shape (a disc: centre distance <= r); the fog draw softens edges, so cell size is
  a memory and save trade, not a look limit. 50 km at 64 m: 782 x 782 cells = 75 KiB in memory per layer per player.
- **Exploration.** The plugin's fixed-tick system sends `Explore` for each player whose position moved more than
  `map.reveal.step_m` since the last stamp (or changed layer); the core stamps a disc of `map.reveal.radius_m`.
  Interiors stamp nothing until the local map. Teleports stamp at arrival.
- **Region reveal.** `Apply(record)` ORs each shape into the grid and discovers the listed places; applying twice is
  a no-op (`applied` set). `Region(id)` reads the baked `map.regions` raster (u16 region index per reveal cell).
  Sources: an item read or bought (effects/services call `Apply`), a guide's dialogue result, an interaction (tower,
  viewpoint), a script. Which of them exist is preset data (owner Q3).
- **Discovery.** After a stamp, places within reach are tested through a uniform bucket grid (bucket =
  `map.place.bucket_m`): `Near` when the player is within its radius, `Inside` when inside the footprint, `Always`
  at load, `Scripted` only by `Discover`. Each new discovery fires one `PlaceDiscovered` (notifications, journal, XP
  hooks, travel). Discovering a place reveals fog only when its kind says so.
- **Scripts.** `Discover { NamePrefix }` (case-insensitive over `match_name`, never the localised label, so peers in
  other languages agree) and `Discover { All }`; `Reveal { Layer }` uncovers the whole fog. The adapter maps its opcodes onto these.
- **Travel reads** `DiscoverySource` for its `Undiscovered` gate (travel.md "Teleport anchors and gates"); discovery
  is stored only here.

## Map bake (MAP2, `sf_map_bake`)

| Step (bake.md) | Subject | Input -> output | Class |
|---|---|---|---|
| `map.index` | (layer, style) | layer, style, places -> levels, tile grid, region name anchors | P |
| `map.tile` | (layer, style, level, x, y) | level 0: raster pass + vector pass -> BC7 sRGB KTX2, 256 px; level n: its 2x2 children box-filtered in linear space, then encoded | P |
| `map.regions` | layer | region id (u16) per reveal cell, from the adapter's region source | G (reveal shapes read it) |

- **Inputs** (dynamic reads: the trace is the invalidation): `terrain.pyramid` heights at the level whose spacing is
  <= `m_per_px` (terrain.md "Data model"), water bodies and sea level (water.md "Water query"), terrain layer weights
  or `terrain.macro`, the route graph (nav.md "Route graph": polylines by `EdgeKind`, `Anchor` nodes), place
  footprints, `MapStyle`, VFS art. A style change rebuilds map steps only; no route graph = no roads, logged.
- **Raster pass:** Horn gradient hillshade, Lambert against the style's light (azimuth 315, altitude 45 deg: the
  cartographic north-west convention, Imhof), style exaggeration, optional contours; colour by the style rule
  (`Height(gradient)`, `Layers(palette per TerrainLayer, fallback macro albedo)`, `Macro`, `Regions(palette)`) times
  shade; water by depth gradient. **Vector pass** (`tiny-skia`): coast stroke and shallow band, roads per `EdgeKind`
  stroke, footprints, paper texture multiply. Labels are never baked (localisation, declutter, theme fonts).
- **Authored beats derived** (bake.md): `map/<layer>/<style>.png` plus a georeference record replaces the raster
  pass (fixed-geography packs, map mods); `map/<layer>/<style>/<level>/<x>_<y>.png` replaces one tile.
- **Budgets.** 50 km at 8 m/px = 625 level-0 tiles x 64 KiB = 39 MiB, ~52 MiB with the pyramid; over
  `map.bake.max_mib` the bake doubles `m_per_px` until under and logs it. Time: **5 s wall** per layer (raster ~0.5
  CPU-s at 39 Mpx, BC7 the bulk); a new "Map" row in bake.md's budget table.
- **Log:** `map: bake <layer> style=<id> levels=L tiles=T MiB=M wall_ms=W`.

## Map screen (MAP3)

- **View-model** (`sf_map::view`, Bevy-free, the test surface): `MapView { layer, centre: Vec2, zoom: f32 }` ->
  visible tiles with screen rects (a missing tile shows its loaded parent, slippy-map standard), fog mask rect,
  labels after declutter (greedy by priority then id, screen-space bucket grid, `map.screen.label_gap_px`; only
  discovered places, region names by zoom), pins, player and party markers, route polyline (guidance), cursor world
  position, hovered item. Zoom clamps to the pyramid plus `map.screen.max_magnify`.
- **Screen** (`sf_ui_screens::MapScreen`, menu layer): clipped root, `MaterialNode<MapTileMaterial>` per visible
  tile (tile texture, fog mask `R8` texture uploaded when the grid's `revision` changes, fog look from the style:
  colour or VFS texture, edge softness), `Text` labels with CSS classes, pin and place icons as `ImageNode`s with
  picking. Tile residency: an LRU of `map.screen.resident_tiles`; closing the screen drops tiles (keeps the top two
  levels).
- **Input:** mouse drag pans, wheel zooms about the cursor; gamepad left stick pans, triggers zoom, the cursor snaps
  to icons within `map.screen.snap_px` (Witcher 3, Elden Ring); actions: place pin, edit pin, remove pin, track,
  centre on player, toggle overlays, "Travel here" (TV16, only when enabled), route to (guidance, when TV20 lands).
- **Pins** (`PinBook` per player, in `sf_map`): `MapPin { id: PinId(u32, never reused within a save), layer, pos,
  icon: RecordId, text: Option<String> (<= 64 chars, a capacity const), tracked: bool }`. At most `map.pins.max`;
  over it the op is refused (`PinsFull`). At most `map.pins.tracked_max` tracked. System pins (travel stops, quest
  objectives when markers are on, a party member's beacon) are views computed from their owners, never stored here.
- **Script hooks:** availability flag (a game's "map menu enabled" switch), open/close, centre on a place.
- **Frame budget** (ui.md "Performance"): map screen CPU <= 1 ms (heavy menu), UI GPU <= 0.3 ms at 1440p with ~70
  tiles and ~200 labels; zero cost when closed except the stamping system (below).

## Compass (MAP4)

- HUD strip in the `hud.compass` slot (replaces the heading label in games that turn it on): cardinal points plus
  markers at their bearing from the camera yaw, across `map.compass.fov_deg`; alpha fades past
  `map.compass.range_m - map.compass.fade_m`.
- **What shows**, filtered by the compass preset: discovered places within range (kind rule `Discovered`);
  undiscovered places within `map.compass.hint_range_m` as an unnamed marker (kind rule `NearHint`, Skyrim; 0
  turns it off); the tracked pin; party members (co-op); quest objectives only when the quests setting
  `QuestMarkers` is `Compass` or `Guidance` (dialogue-quests roadmap decision 7; a game may ship `None`).
  Objective markers arrive as a list from quests.md "Markers" (`objective_markers`, DQ-21) and skip the place
  range and fade (quest markers show at any distance, Skyrim/Witcher 3).
- Cost: query the place buckets around the player once per tick, bearings per frame: <= 0.05 ms CPU (inside the HUD
  0.5 ms budget).

## Guidance, auto-ride, map travel (pointers)

- **One route.** A target (pin, place, stop) -> `sf_nav::journey` from the player (host for gameplay, client-local
  for UI) -> a polyline cached until the player leaves it by `map.guide.reroute_m`. Presenters: the route line on the
  map screen and the next-turn bearing on the compass (TV20; RDR2, Witcher 3), an optional wind-direction hint
  through the weather adapter (GoT), and auto-ride (TV19) that follows the same polyline. Guidance is shown only when
  `QuestMarkers` is `Guidance` or the player asked for a route to a pin.
- **TV16 "travel here".** Off by default (`travel.anywhere.enabled = false`). When on, the action appears on a
  discovered place that has an arrival pose (its travel `Stop`, else the adapter's arrival rule), calls travel's
  `TravelCost::check` with caller `MapTravel` (gates: `TeleportBlocked`, `HostilesAlerted`, `NoTravel`,
  `Undiscovered`, `Adapter`; travel.md TV10), spends the journey's route time, and runs the instant presenter
  (fade, `GameClock::skip`). Nothing in `sf_map` teleports.

## Co-op and saves

- **Host-authoritative.** `MapReveal` is host state per player: stamps from host positions, records applied on the
  host, discoveries fired there. Each client receives its own `PlayerMap` deltas (fog runs, place ids, pins) and
  draws locally. Default **per player** (Elden Ring: each world its own map; Valheim: personal maps shared only at a
  table); `map.coop.shared_reveal = true` makes the host OR every stamp and discovery into the whole party.
- **Pins:** personal, saved with the player; a **beacon** (one per player, not saved) is shared with the party
  (Elden Ring beacon). Tracked pins are personal.
- **Save section `sf/map` v1** (host): per player: per layer `{ origin, cell_m, w, h, runs }` (alternating
  run lengths, varint), sorted place ids, applied record ids, pins, next pin id. Typical < 16 KiB. Load: a removed
  place, record or layer is dropped with one log line; a changed grid (cell size, bounds) is resampled
  conservatively (a target cell is revealed when any overlapping source cell was). The map camera (last centre and
  zoom) is presentation, kept in the screen's per-machine state, not the save.

## Budgets and tunables

Runtime: `Explore` stamp <= 2 us per player (r = 120 m over 64 m cells is ~13 cells) plus discovery test <= 5 us;
one distance check per player per tick otherwise. Memory with the screen closed: grids (75 KiB per layer per player
at 50 km), place buckets, two top pyramid levels (< 2 MiB); open: + `resident_tiles` x 64 KiB (12 MiB at 192). Bake:
5 s wall and the `max_mib` cap per layer. Measured by `#[ignore = "measurement: ..."]` tests in MAP1, MAP2a, MAP3a.

Tunables (prefix `map.`; a game sets defaults by base-mod record, per preset where marked):

| Name | Default | Range | Unit | Net | Source |
|---|---|---|---|---|---|
| `map.reveal.cell_m` | 64 | 8-4096 | m | Gameplay | Guess (memory vs edge; fog is softened) |
| `map.reveal.radius_m` (preset) | 120 | 0-2000 | m | Gameplay | Guess (RDR2/GoT reveal radius, memory) |
| `map.reveal.step_m` | 8 | 1-200 | m | Gameplay | Guess |
| `map.place.discover_radius_m` | 60 | 0-1000 | m | Gameplay | Guess (Skyrim/Oblivion proximity discovery) |
| `map.place.bucket_m` | 256 | 32-4096 | m | Gameplay | Guess |
| `map.bake.m_per_px` | 8 | 0.5-128 | m/px | Presentation | Guess (layer overrides; 52 MiB at 50 km) |
| `map.bake.max_mib` | 96 | 8-1024 | MiB | Presentation | Guess |
| `map.bake.relief_exaggeration` | 2.0 | 0.5-10 | ratio | Presentation | Guess (style overrides) |
| `map.screen.resident_tiles` | 192 | 32-2048 | count | Presentation | Guess (1440p full screen ~70 tiles) |
| `map.screen.max_magnify` | 4 | 1-16 | ratio | Presentation | Guess |
| `map.screen.label_gap_px` | 12 | 0-64 | px (reference 1080p) | Presentation | Guess |
| `map.screen.snap_px` | 32 | 0-128 | px (reference 1080p) | Presentation | Guess |
| `map.pins.max` | 100 | 0-1000 | count | Gameplay | Reference(Elden Ring marker cap, memory) |
| `map.pins.tracked_max` | 1 | 0-16 | count | Gameplay | Reference(Skyrim one custom marker, memory) |
| `map.compass.enabled` (preset) | true | bool | none | Presentation | Owner(2026-11-25) |
| `map.compass.fov_deg` | 180 | 60-360 | deg | Presentation | Guess |
| `map.compass.range_m` | 500 | 0-5000 | m | Presentation | Guess |
| `map.compass.fade_m` | 100 | 0-1000 | m | Presentation | Guess |
| `map.compass.hint_range_m` (preset) | 150 | 0-1000 | m | Presentation | Guess (Skyrim undiscovered hint) |
| `map.guide.reroute_m` | 15 | 1-200 | m | Presentation | Guess |
| `map.coop.shared_reveal` | false | bool | none | Gameplay | Guess (owner question 2) |

`const`: tile size 256 px, BC7 format, save version, pin text capacity (64 chars), hillshade maths. The light
direction and colours are style data.

## Tests

Exact (synthetic layers, places and styles; no game data):

- MAP1: a disc reveals exactly the cells with centres within r; a move under `step_m` stamps nothing; `Apply` twice
  = once; `Region` reveals exactly its raster cells; players and layers isolated; `shared_reveal` = union; each
  discover rule fires one `PlaceDiscovered` per player and place; `NamePrefix` case-insensitive; `All`; `sf/map`
  round trip bit-identical; load drops removed places/records/layers and resamples a changed grid (conservative OR).
- MAP2a: hillshade of a synthetic cone is brightest on the north-west flank; two bakes give identical object
  hashes (determinism); tile count per level and `max_mib` coarsening; an authored image replaces the raster pass;
  missing route graph bakes without roads and logs.
- MAP3a: visible tile set for a viewport and zoom; parent fallback; zoom and pan clamps; label declutter
  deterministic and only discovered places; pins: add to `max` then `PinsFull`, ids not reused, text clamp,
  `tracked_max`, round trip, a pin on a removed layer dropped.
- MAP4: bearings (a place due east of a north-facing camera sits at +90 deg), range and fade filter, `NearHint`,
  `QuestMarkers::None` hides objective markers.
- TV16: off -> no action and the command is refused; on -> each `MapTravel` gate refuses alone.

Log checks: the `map: bake` line (MAP2b), `map: <n> places, <m> regions` at load. Owner checks (entries written by
the building slices in `bevyrrowind/docs/visual-checks/map.md`): **MAP-1** baked map look at every zoom (MAP2b),
**MAP-2** map screen: pan, zoom, fog edge, labels, pins, gamepad (MAP3b), **MAP-3** compass (MAP4), **MAP-4**
Morrowind profile and reveal rule in play (MAP5), **MAP-5** guidance line (TV20).

## Slices

Roadmap IDs unchanged; sub-IDs MAP1b, MAP2a/b and MAP3a/b/c are new (one deliverable each).

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| MAP1 | `sf_map` core: layers, places, `MapReveal`, `RevealRecord`, `MapCommand`, discovery, `sf/map` codec | `sf_map` | MAP0 | exact; round trip |
| MAP1b | Plugin: fixed-tick stamping, `PlaceDiscovered` message, save provider, `DiscoverySource` for travel | `sf_map_bevy` | MAP1 | headless app test |
| MAP2a | Bake steps `map.index/tile/regions`, `MapStyle` record, synthetic world | `sf_map_bake` | MAP0 | exact determinism; log |
| MAP2b | Game wiring: terrain pyramid, water, route graph, places into the bake; bake.md budget row | `mw_bake` | MAP2a | log ms, MiB; owner MAP-1 |
| MAP3a | `MapView` view-model, label declutter, `PinBook` | `sf_map` | MAP1 | exact |
| MAP3b | `MapScreen`: tile material, fog, labels, pins, input | `sf_ui_screens` | MAP3a, MAP2a | headless spawn; owner MAP-2 |
| MAP3c | Game wiring: open-map action, availability flag, `ShowMap`/`FillMap` | `mw_rpg_bevy`, `mw_scripts_bevy` | MAP3b, MAP1b | exact per opcode |
| MAP4 | Compass strip | `sf_ui_screens` (+ `sf_map` filter) | MAP1b | exact bearings; owner MAP-3 |
| MAP5 | Morrowind profile, places and regions tables, reveal rule | `mw_*` data | MAP2b, MAP3c | owner MAP-4 |
| TV16 | Opt-in "travel here", off, gated | `sf_travel_bevy` | MAP3b, TV10 | exact: off; gated |
| TV20 | Guidance line (pointer) | `sf_ui_screens` | TV19, MAP4 | owner MAP-5 |

**Later:** local map (interiors and towns as `MapLayerId`s; a top-down render at bake from cooked meshes, or Bevy's
image render target; OpenMW renders and saves per-cell fog), Hero's-Path trail, stamps, 3D map style.

## Adapter contract

The adapter supplies, as data and pure rules, never as framework code:

1. **Layers:** one `MapLayer` per exterior worldspace with bounds, `m_per_px` and reveal cell overrides.
2. **Places and kinds:** `MapPlace` records from its named locations (position, footprint, region, kind,
   discover rule); travel stops reference them (`Stop.place`).
3. **Regions:** `MapRegion` records and the region source for `map.regions`.
4. **Style:** a `MapStyle` record (colour rule, palette, hillshade light, water, road strokes per `EdgeKind`, paper
   art, icons, fog look) or authored art with a georeference.
5. **Reveal rule per preset:** radius and cell, which `RevealRecord` sources exist (items, guides, towers), which
   places reveal fog.
6. **Scripts:** its opcodes mapped onto `Discover` and `Reveal`; the map-menu availability flag.
7. **Compass preset:** on or off, `hint_range_m`, `QuestMarkers` value.
8. **Map travel:** whether TV16 may be turned on and the arrival rule for a place without a stop.

Morrowind (detail in ui-adapter.md "Map (MAP5)"):

- Layer: Vvardenfell; places: named exterior cells, discover `Inside`, no fog (vanilla "visited location").
- Classic: exploration only, reveal cell = one exterior cell painted on entry (verify OpenMW `globalmap.cpp` + commit
  in MAP5), no compass (vanilla has none). Modern: radius plus guides and bought region maps (owner Q3), compass on,
  places only (`QuestMarkers::None`).
- `ShowMap "prefix"` -> `Discover { NamePrefix }`, `FillMap` -> `Discover { All }` (labels, not fog; verify OpenMW in
  MAP3c); `EnableMapMenu` -> availability flag; TV16 off in both presets.

## Open questions (owner: taste and scope only)

1. **Morrowind map look:** a painted parchment relief (cartographic hillshade, paper art, ink coast and roads) or
   vanilla's plain height-coloured relief? Recommended: painted parchment as the one default profile (one bake);
   vanilla relief as an optional second profile for Classic (+~5 s bake when chosen).
2. **Co-op reveal:** each player explores their own map, or the party shares one? Recommended: per player by
   default (Elden Ring, Valheim), with a "shared map" party option.
3. **Modern Morrowind, who reveals regions:** recommended: traders and publicans sell "map of <region>" items
   (reading one applies its `RevealRecord`), and travel operators and guild guides discover their destinations as
   places (no fog).
4. **Modern Morrowind compass hints:** show undiscovered places nearby (150 m) as unnamed markers (Skyrim)?
   Recommended: yes in Modern; Classic has no compass.

Decided by the industry standard (no owner question): tile pyramid at 256 px BC7, live labels, fog as a runtime
mask, radius reveal on a coarse grid, 100 pins, one tracked pin, beacon shared in co-op, guidance on one route query.
