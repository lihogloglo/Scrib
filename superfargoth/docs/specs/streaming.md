# World streaming: worldspaces, door links, rings, budgets

Status: agreed (2026-10-11); `sf_world` core, its `world.*` bake steps,
and `sf_world_bevy` implemented (2026-10-11)
Crates: `sf_world` (new, Bevy-free: records, index, scheduler),
`sf_world_bevy` (new: plugin, spawning, big_space glue)

## Goal

Stream an arbitrarily large exterior and any number of interiors around the
player without frame spikes, from content that only adapters understand.
This settles WISHLIST early decision 2: **worldspaces with door links are
the streaming unit, and interiors are worldspaces, not special cases.**

Non-goals for M2 (the data model already allows them): seamless portal
rendering and crossing (M8; M2 uses a named fade scaffold), texture streaming
(M7), simulation of unloaded areas.

## Canonical technique

A **world partition**: a regular grid of streaming cells, independent of
source "cells". **Streaming sources** (anchors) with per-layer loading
ranges and hysteresis, as in UE5 World Partition. Work is split into async
IO and decode on task pools plus **time-sliced main-thread stages** with
per-frame budgets. Priority uses distance, view direction, and velocity
prediction (prefetch along travel). Worldspaces are linked by portals
(door links).

Deviation: the partition is fixed per worldspace (a power of two of
`GRID_CELL`, world-coordinates.md). The cell → placements index is a bake
product, so nothing at runtime walks the whole world.

Prior art checked during implementation (2026-10-11):
- UE5 World Partition: a cell touched by several streaming sources takes
  the highest source priority and the highest target state (activated >
  loaded); cells sort by source priority, then grid level, then custom
  priority, then distance and angle to the source. Concurrent cell loads
  are capped (`wp.Runtime.MaxLoadingLevelStreamingCells`), and "block on
  slow streaming" stalls the game when cells fall behind. Our class →
  effective distance order, the per-cell max over anchors, and the
  in-flight caps follow the same shape; our answer to falling behind is
  burst mode behind a fade, never a stall mid-play. UE's docs describe no
  unload hysteresis; we keep ours (it is what prevents thrash at ring edges).
- Crates: nothing to reuse for the scheduler itself (engine streamers are
  engine-bound). Spatial search needs no crate: the directory is
  two-level (1 km pages, then cells), so a tick scans page bounds and only
  the cells of pages in reach. Queues are re-sorted each tick from small
  sets with explicit tie-breaks, which is simpler and deterministic, so no
  priority-queue crate (`priority-queue` is LGPL/MPL, excluded anyway).
  Reused: `bitflags` (placement flags), `blake3` (ids, fingerprints),
  `postcard` (binary forms).

## Data model

**Records** (`sf_records` kinds registered by `sf_world`; adapters define
them, mods patch them like any other record):

```rust
pub struct Worldspace {
    pub name: String,
    pub partition: Partition,         // Grid { shift: -1 } = 128 m cells | Single (load whole)
    pub rings: Option<RingOverrides>, // per-worldspace radius tweaks
    pub has_terrain: bool,            // terrain.md
    pub sea_level: Option<f32>,       // until sf_water bodies replace it (M6)
    pub environment: Option<RecordRef>, // ambient / fog / sky preset (None: game default); blended across links in M8
}

pub struct LinkEnd {
    pub worldspace: WorldspaceId,
    pub frame: WorldTransform,        // portal frame: on the door plane, +Z pointing into this worldspace
    pub arrival: WorldTransform,      // where a traveler appears when arriving here
    pub door: Option<PlacementId>,    // the door object, if any
    pub extent: Vec2,                 // portal width/height; zero = point link (teleport)
}
pub struct DoorLink { pub a: LinkEnd, pub b: LinkEnd, pub two_way: bool }
```

`frame` is unused in M2 but required now, so M8 doesn't change the record.
Crossing from a to b maps a pose by `b.frame · R_y(π) · a.frame⁻¹`. This
is the standard portal transform: the half-turn makes leaving through a's
plane mean entering through b's (`DoorLink::cross`, `cross_direction`;
frame scale is ignored, portals are rigid). Door prefetch measures to
`frame.pos`, so adapters fill `frame` with at least the door's position
even for point links.

`WorldspaceId(RecordId)` and `Location { worldspace, pos: WorldPos }` live
in `sf_world` (not `sf_coords`): they name a record kind this crate owns.
Kind names are namespaced: `sf.worldspace`, `sf.door_link`,
`sf.placement_group` (`sf_world::register_kinds`).

**Placements** are what streaming spawns. They aren't records, because
each adapter expresses them in its own records:

```rust
pub struct Placement {
    pub id: PlacementId,              // u128: hash of an adapter key stable across load orders
    pub base: RecordRef,              // (kind, RecordId) of the base object
    pub model: Option<VfsPath>,
    pub transform: WorldTransform,
    pub class: PlacementClass,        // Static | Dynamic | Actor | Light | Door | Marker | Custom(u16)
    pub flags: PlacementFlags,        // bitflags: GROUNDED, EDITOR_ONLY, NO_LOD, DISABLED, NEAR_ONLY
}
```

`PlacementId::from_key(namespace, key)` is blake3 over the length-prefixed
namespace (adapter or source name) and key, truncated to 128 bits. Text
formats write it as 32 hex digits.

**World index** (bake products, per worldspace; bake.md).
Every placement belongs to exactly one streaming cell, chosen by its pivot.
The index has two parts:
- **Pages** (`IndexPage`, one bake product per 1 km page): the cells of
  an 8×8×8 block (cells are 3D like the grid; worlds are flat, so a page
  is usually one layer of 8×8). Per cell: the placements, sorted by class,
  then by descending bounds radius, then by id (so the order doesn't depend
  on source order), each with its bounds radius.
- **Directory** (`CellDirectory`, one per worldspace, always resident):
  per cell a `CellSummary`, the **content AABB** (union of the placements'
  model bounds, from `model.info`, relative to the cell origin), which is
  what ring distances measure against, so large objects arrive in time;
  byte estimates for the far subset and for everything (unique models once
  per cell plus a per-placement overhead), placement counts, and the
  coarsest-LOD distance. Plus each page's slice of the cell list. The
  scheduler only ever reads the directory; pages are fetched per cell load.

Binary forms are postcard with a magic and version: pages carry a string
table (record kinds, ids, model paths once per page), exact `WorldPos`
(i32 cell + f32 local; `WorldPos`'s text serde accepts two shapes, which
a non-self-describing format can't decode), and one float for uniform
scale. A dense page costs about 67 bytes per placement before LZ4
(test-enforced < 72).

Moved references need no special handling: adapters deliver placements at
their final position, and the index assigns cells by position, not by
source region.

**Runtime overlay** (`PlacementOverlay`, serialized by `sf_save` through
`to_bytes`): removed ids; moved *and* spawned placements, stored whole
(worldspace + full `Placement` at its new transform), bucketed by
(worldspace, 256 m grid cell); and per-placement state blobs keyed by
owning system. A moved placement can carry state too, so these aren't
exclusive variants. Spawned objects get ids from the same `PlacementId`
space (hash of a save-persisted counter). A cell load applies the overlay
after reading the page (`apply`: index order minus removed and moved,
then moved/spawned placements now in the cell, by id), so streaming never
reverts gameplay changes. Overlay additions count as near-ring content.

## Rings

Rings decide what's **resident**. Bevy's `VisibilityRange` decides which
LOD is **drawn**. Both read one distance table owned by `sf_world`
(WISHLIST §1: distance tiers come from streaming).

| Ring | Resident content | Load / unload radius (default) |
|---|---|---|
| R0 Active | everything: physics, scripts, AI, gameplay entities | 192 / 256 m |
| R1 Near | all visual placements and lights at full LOD chain; dynamic objects frozen, visual-only | 768 / 896 m (far field: `d1` + band + 80 / + 208 m = 520 / 648 m) |
| R2 Far | with a far field: the cell's L1 node (proxy mesh, impostor and light batches; far-field.md), measured to the cell's page; without one: placements with bounds radius ≥ 4 m at coarsest LOD | 3 / 3.25 km (far field: 0.85 / 0.95 × `d2`) |
| R3 Horizon | terrain (own clipmap residency, terrain.md) + L2/L3 HLOD proxies (far-field.md) | view distance |

The radius is measured from the anchor to the cell's content AABB, except
R2 with a far field (`SchedulerConfig::far_by_page`): it measures to the
cell's page (the union of the page's cell boxes, = the far field's L2
node), so all L1 nodes under an L2 node arrive and leave together; the
request priority still uses the cell's own distance. Each cell
has one residency level (the highest ring that wants it). It moves one level
at a time, and each step adds or removes only the difference (R2's large
objects are kept when the cell is promoted to R1).

Precisely, per cell and tick: `load` is the highest level whose load radius
reaches the cell from any anchor, `keep` the highest whose unload radius
does, and `target = max(load, min(committed, keep))`, where `committed` is
the current level or, while a load is in flight, the level it loads to. A
cell below target gets a one-level load request, a cell above it a
one-level unload request. A load in flight is cancelled only once its
level's unload radius no longer reaches the cell, so an anchor wobbling
across a ring edge never thrashes. R3 has no per-cell level: its radius is
in the table for terrain and `VisibilityRange` (`RingConfig::resident_until`).

**Gap found by the industry audit (2026-10-12, `industry-audit.md`): this
section is wrong about small objects.** R1 spawns *every* placement of a
cell, and each chain's `VisibilityRange` ends at `d1` (or the ring radius)
whatever the object's size, so a cup is resident and drawn at 400 m, a third
of a pixel wide. Shipped engines tie draw *and* load distance to screen size:
UE per-primitive max draw distance and cull distance volumes (size →
distance), Unity LODGroup "Culled", Bethesda per-category fade distances,
RAGE's per-entity `lodDist` (which is also the unload distance), OpenMW's
small-feature culling (2 px). Planned fixes (ROADMAP Next up 6):
- **J1 (built 2026-10-12, industry-audit.md "J1 as built and measured")**: on vanilla exteriors it culls
  3 % of placements at High (8 % at Low), mostly past 128 m; entities unchanged; Balmora CPU floor
  -0.3 ms (6.32 -> 6.02 ms at 960x600), GPU and stutters unchanged. Rule as planned:
  `d_draw = clamp(K·2r / s_min · bias, d_floor, d_end)` with `K =
  H / (2·tan(fov/2))` (the `LodScreen` constant), `s_min` 4/3/2/1 px for
  Low/Medium/High/Ultra; chains end at `d_draw` with the usual dither margin,
  and LOD levels past it are not spawned. Placements in their L1 proxy keep
  `d_draw = d1`; actors, doors and `NO_SIZE_CULL` are exempt.
- **J3**: R1 gets **detail tiers** keyed on `d_draw` (≥ 256, 128, 64, 32 m,
  floor), loading at `T_k + 80 m` and unloading at `1.25·T_k + 80 m`. A cell
  spawns only the radius-sorted prefix of placements whose `d_draw` reaches
  it, and R0 activation is decoupled from visual tiers.
- **J2**: streamed statics try `NoCpuCulling`. Bevy then culls frustum and
  `VisibilityRange` on the GPU in every view, cascades included, and the
  per-frame CPU visibility passes (O(entities × views)) drop out.

**Anchors**: `Anchor { id, role: Gameplay | Visual, location, velocity,
view_dir, radius_scale }`. The player is `Gameplay` (drives R0). The camera
is `Visual` (drives R1+), so a free camera can roam without activating
physics. Several anchors are allowed (cutscene cameras, a remote scripted
event). The anchor's own worldspace is the only one it drives.

## Scheduler and budgets

The scheduler is pure data in `sf_world` and is deterministic for a given
input. Each frame it takes the anchors (location, velocity, view direction),
the resident set, and the configuration, and outputs a priority-ordered list
of load and unload requests.

- **Priority.** Effective distance `d = min(dist(now), dist(now + v·t))`,
  with lookahead `t = clamp(2 × measured load latency, 0.5 s, 4 s)`. The
  same `d` decides residency, so cells ahead of a fast traveler load early
  (prefetch along travel). For R1+ only, the priority (not the residency
  decision) is `d × (1 + 0.5·(1 − cos θ))` to the view direction (cells
  behind count double): weighting residency by view would load and unload
  as the camera turns. Requests sort by class (0 = R0, 1 = door prefetch,
  2 = R1+), then that distance, then request id; R0 requests always come
  first. Requests whose cell left the unload radius are cancelled. Latency
  is an EMA (α = 0.2) of fetch start → assets loaded, measured from the
  engine's completion calls (the scheduler reads no clock: `now` is input).
- **Stages.** (1) *fetch*: index page read plus overlay, async on the IO
  pool; (2) *assets*: models, LOD meshes, and textures through Bevy's
  `AssetServer`; (3) *spawn* on the main thread: entities spawn hidden and
  the cell's visibility flips at once when complete, so there is no partial
  pop-in; (4) *activate* (R0 only): colliders are built on the task pool and
  then inserted, and scripts and AI start; (5) *upload*: GPU (Bevy uploads an
  asset when it loads; its per-frame limiter stays off, see below). Unload runs in reverse: deactivate
  (dynamic state into the overlay), despawn, and asset handles drop.
  Per request (one level step): load to R2/R1 = fetch → assets → spawn →
  upload (reveal when done); load to R0 = fetch (normally a CPU-cache hit)
  → activate; unload R0 → R1 = deactivate; other unloads = despawn. Async
  stages are started by `update` within the in-flight caps and finished by
  the engine (`fetched`, `assets_loaded`); main-thread stages run through
  `run_stage(stage, work)`, where `work` does one item (a few placements, a
  mesh) and reports its measured cost.
- **Budgets** (defaults for the 120 fps target, scaled with the target frame
  time):

  | Stage | Budget per frame |
  |---|---|
  | spawn | 1.0 ms main thread |
  | activate | 0.5 ms |
  | deactivate | 0.5 ms |
  | despawn | 0.5 ms |
  | GPU upload | 8 MiB |
  | in flight | 8 cell fetches, 64 asset loads |
  | decoded-cell CPU cache | 256 MiB, LRU |

  A stage stops at the first item that crosses its budget. The rest waits
  for the next frame. Exactly: an item starts only while the stage's usage
  is below its limit, so a frame overshoots by at most one item (and one
  oversized item can't starve the stage). In-flight caps use the same rule;
  asset loads are weighted by each request's asset count from its fetch.
- **Burst mode** behind a fade (teleport, fast travel, loading a save):
  every budget and cap ×20 until the destination's R0 and R1 are resident
  (no load request whose cell targets R1 or above remains); R2+ fills in
  after the fade lifts. `update` reports when burst mode ends.

## Worldspaces at runtime

Each loaded worldspace is one big_space `BigSpace` root plus its own
`RenderLayers` layer, because worldspaces overlap in coordinates. A
worldspace has three states:
- **Inactive**: no entities.
- **Prefetched**: R0 cells around a link end are resident but invisible to
  the main camera and not simulated.
- **Active**: the player's worldspace. In M2 only this one simulates.

**Door prefetch.** When a gameplay anchor is within 24 m of a link end, the
other end's worldspace becomes Prefetched around `arrival`, at the priority
just below R0. The scheduler adds a prefetch anchor at `arrival` that
drives R0's radius at class 1; it stays until the anchor is 32 m from the
end (hysteresis). One-way links prefetch only from `a`. An anchor or
prefetch into a worldspace without a directory is reported
(`missing_worldspaces`) so the engine can load it.

**Transition (M2): the `FadeTransition` scaffold.** Fade out (0.3 s),
swap the active worldspace, move the player to `arrival`, fade in. The fade
holds at full black in burst mode until the arrival can be shown (DR-4,
2026-10-19): at least 3 frames, the content cells within `ready_margin_m`
(16 m) of the arrival at Near or above and no Near load pending in the
worldspace (`Scheduler::arrival_ready`; burst restarts if it ended before
the anchor arrived), and every mesh within `ready_radius_m` (48 m) of the
traveler with its textures loaded (a material whose image is loading doesn't
draw, so objects popped in). `max_hold_secs` (10 s) caps it with a warning.
The fade is a full-window UI node (after post-processing, so truly black;
the earlier 3D quad was fogged and drawn over by later passes). Removal path: M8's
portal crossing replaces it with no change to the data model.

**Residency cache for the worldspace just left** (2026-10-12). When an
anchor changes worldspace, the scheduler keeps a keep-only ghost of it where
it was (same rings, loads nothing, priority class 3): the cells it had stay
resident and hidden. The ghost ends when the anchor comes back, or once it is
older than `SchedulerConfig::linger_secs` (5 s) and no gameplay anchor stands
within `door_prefetch.unload` of a link end leading back. This is the cell
cache of door-based engines: a shop visit next to the door keeps the town
loaded, a dungeon crawl lets it go. Burst mode (the fade's hold) waits only
for the active worldspace's loads. The fade writes `TravelArrived` the frame
it moves the traveler, so apps resync camera controllers that cache a pose
(Bevy's `FreeCamera` keeps its own yaw and pitch).

**Every worldspace is indexed** (2026-10-12). The bake indexes each
worldspace with the partition its `Worldspace` record names
(`WorldBake::with_overrides`; interiors are `Single`, one cell that loads
whole), and `bake_worldspaces` resolves all layouts in one parallel call
before the pages, so a thousand small interiors cost one batch, not a
thousand sequential lookups. Adapters report door link ends whose
worldspace has no directory (must be 0). Each worldspace draws only in its
own render layer: cooked parts get it at spawn, runtime-imported scenes
through `SubtreeLayers` on their root (as `SubtreeRange`), and point lights
take `SpawnCx::layers`, so a prefetched interior never shows or lights the
exterior view. What a worldspace looks like (sky, terrain, ambient, fog,
exposure) is the content pack's: it follows `ActiveWorldspace`.

**Industry comparison (door transitions).** Bethesda's engines (Morrowind
through Starfield) and OpenMW keep each interior as its own cell space,
reached by a load door that names a teleport marker; activating it shows a
loading screen while the target cell loads ([CK wiki](https://ck.uesp.net/wiki/Bethesda_Tutorial_World_Hookup),
[Starfield CK: Cell](https://starfieldwiki.net/wiki/Starfield_Mod:Starfield_Creation_Kit:Cell)).
OpenMW hides most of it by preloading the cell behind a door once the
player is within `preload distance` (1000 units, ~14 m) and keeping up to
20 preloaded cells cached ([OpenMW cells settings](https://openmw.readthedocs.io/en/stable/reference/modding/settings/cells.html)).
Seamless open worlds (The Witcher 3, Cyberpunk 2077) place interiors inside
the exterior's streaming partition, so there is no worldspace swap at all;
Unreal's World Partition does the same with level instances and data
layers, whose "loaded but hidden" state is the pre-warm step before
"activated" ([UE World Partition](https://dev.epicgames.com/documentation/en-us/unreal-engine/world-partition-in-unreal-engine)).
We follow the first model for data (Morrowind interiors overlap the
exterior's coordinates and are larger inside than out, so they can't join
its partition) with the second model's states: Prefetched is UE's
"loaded, hidden", and the 24 m prefetch radius is wider than OpenMW's so a
door reached at a run is resident; a short fade replaces the loading
screen and holds only on a miss. The spec needed no change: the worldspace
just left stays resident without a cell cache, because the arrival point
stands next to the return door, whose prefetch keeps the other side's R0.
Seamless crossing stays M8 (portal frames), as planned.

## Interfaces

```rust
/// Implemented by adapters (Bevy-free). sf_world also ships a native
/// implementation over a `PlacementGroup` record kind, so native mods can
/// place objects without any adapter.
pub trait WorldSource: Send + Sync {
    /// The adapter's own source regions (e.g. its cells) with a fingerprint
    /// for the bake cache. Unchanged fingerprint ⇒ cached index pages reused.
    fn regions(&self, ws: &WorldspaceId, out: &mut dyn FnMut(RegionId, Fingerprint));
    /// Every placement of one region, at its final position, in stable order.
    fn placements(&self, ws: &WorldspaceId, region: RegionId, out: &mut dyn FnMut(Placement));
}
```

`RegionId(u64)` is a hash of the adapter's region key; `Fingerprint` is a
blake3 digest. **No terrain hook here** (changed 2026-10-11): terrain.md's
`TerrainSource` takes `&WorldspaceId`, so `sf_terrain` depends on
`sf_world`; a `terrain()` method returning an `sf_terrain` trait object
would be a dependency cycle, and a placeholder or `Any`-typed hook would be
a kludge. The adapter hands its `TerrainSource` to `sf_terrain`'s bake steps
directly, next to its `WorldSource`.

`NativeWorldSource::from_db(&RecordDb)` snapshots the live
`PlacementGroup { worldspace, placements: Vec<GroupPlacement> }` records:
one region per group, fingerprinted by blake3 of the group's postcard
bytes; placement ids hash (group id, key), so they survive reordering.

**Index building** is pure functions the bake wraps: `bucket_by_page`
(placements → per-page buckets, for per-region caching), `build_pages`
(placements + a `model.info` lookup `&dyn Fn(&VfsPath) -> Option<ModelInfo>`
→ pages and stats: missing models, duplicate ids), `build_directory`, and
`build_index` (all of it from a `WorldSource`). Mapped onto `sf_bake`
(string subjects): `world.region` (`ws/region`, input = the region
fingerprint as a param) → page buckets; `world.layout` (`ws`) reads each
region's page list → page → regions map; `world.page` (`ws/page`) reads
the layout, the buckets of only the regions touching it, and `model.info`
of each model it meets (dynamic inputs) → `IndexPage` bytes; `world.dir`
(`ws`) reads every page → `CellDirectory` bytes. An unchanged region
fingerprint is a trace hit for its bucket step, so only pages touching
changed regions rebuild.

Implemented as `sf_world::bake` (feature `bake`, so the runtime core
doesn't pull `sf_bake`/`sf_model`): `WorldBake::new(source, params)
.register(&mut baker)`, then bake `WorldBake::directory_key(ws)`. Details
settled during implementation:
- Subjects are `<ws>/<region hex>` and `<ws>/<x>,<y>,<z>`, split at the
  last `/`, so worldspace ids may contain slashes.
- Region fingerprints are snapshotted once per bake session (the planner
  asks for them per subject).
- A region blob starts with its page list, so `world.layout` reads only
  that prefix. Granularity is per region, not per (region, page): a
  changed region rebuilds every page it touches, and pages whose bytes
  come out the same stop propagation there. Test: changing one region
  that spans two pages re-runs 1 region, the layout, 2 pages (one with new
  bytes) and the directory; changing one model file re-runs its
  `model.info` and the one page using it.
- `model.info` failures don't fail the page: the model is indexed as a
  point and its file is recorded as a dynamic input, so fixing it
  rebuilds the page.
- The index's per-model byte estimate is `(vertices · 36 + triangles · 12)
  · 1.5` from `model.info` (decoded size plus the LOD chain);
  `lod_distance` stays 0 until `mesh.cook` errors feed it.
- The runtime reads products through `IndexReader` (`BakedIndex` over a
  store and manifest; `MemoryIndex` for tests and generated worlds).

Bevy side: `WorldStreamingPlugin`; a `PlacementSpawner` hook per
`PlacementClass` (the content pack spawns actors, lights, and doors with its
own components; the framework spawns statics); `CellLoaded`/`CellUnloaded`
and `WorldspaceActivated` events for gameplay. Statics are low-precision
children of their stream-cell entity, which is a `Stationary` grid entity.
Dynamic objects get their own `CellCoord` when activated.

### `sf_world_bevy` (implemented 2026-10-11)

The heavy logic stays in `sf_world` (scheduler, index, `LevelContent`
level steps, `PageCache`, `IndexReader`); the plugin is glue.

```rust
app.register_asset_source(BAKED_SOURCE, baked_source(store, manifest, exts)) // before AssetPlugin
   .add_plugins(DefaultPlugins.build().disable::<TransformPlugin>())          // big_space propagates
   .add_plugins(WorldStreamingPlugin { config: StreamingConfig::default() })
   .insert_resource(WorldIndexSource(Arc::new(BakedIndex::new(store, manifest))))
   .insert_resource(ActiveWorldspace(Some(ws)))
   .insert_resource(DoorLinks(links)).insert_resource(WorldspaceRecords(records));
// A camera: child of `registry.ensure(&mut commands, &ws).root`, with
// CellCoord + Transform, FloatingOrigin, StreamingAnchor::visual(),
// RenderLayers::from_layers(&[0, entry.layer]). The player: StreamingAnchor::gameplay().
```

- **Resources**: `Streaming` (the scheduler and engine state; `begin_burst`,
  `is_idle`, `stats`), `WorldspaceRegistry` (root, origin proxy, render
  layer per worldspace; layer 0 stays shared), `WorldOverlay`
  (`Arc<RwLock<PlacementOverlay>>`, read by every fetch),
  `PlacementSpawners`, `ModelMaterials` (cooked material → Bevy material
  mapping), `FadeTransition`, `WorldOrigin`, `StreamingDebug` (feature
  `debug`).
- **One exclusive system** (`StreamingSystems::Drive`, in `Update`) runs
  the tick and every main-thread stage, so each item's `Instant` timing is
  the real cost (spawning through `Commands` would only time the queueing).
- **Fetch**: an `IoTaskPool` task reads the page through the
  `PageCache` (LRU, 256 MiB), applies the overlay, and returns the level
  step's placements (`LevelContent::step`).
- **Assets**: cooked models load as `baked://mesh.cook/<model>.sfmesh`,
  refcounted per model; a loaded model becomes a template (LOD parts with
  meshes and materials, materials deduplicated by description). A model
  that fails to load or was skipped by `mesh.cook` is *uncooked*: the
  `PlacementSpawners` uncooked hook (the content pack's runtime importer)
  spawns it; without one it is counted and skipped.
- **Prepared assets** (weather/distant-shadows.md bug B3, 2026-10-12): once a load's
  models resolve, the assets stage asks each placement's spawner
  (`PlacementSpawners::spawner_for`: uncooked hook or class hook) for
  `PlacementSpawner::prepare(&AssetServer, placement, model)`, an asset the
  spawner will instance itself (the runtime-imported model), deduplicated
  per load and held until the work ends. The stage completes only when
  each has loaded or failed with its dependencies, so a group never
  reveals before its runtime-imported models (`StreamStats::prepared`).
- **Ranges on late meshes** (bug B1): a spawner whose meshes appear later
  (a scene) puts `SubtreeRange::of(cx)` on the root; `apply_subtree_ranges`
  gives each new `Mesh3d` below it the placement's `VisibilityRange`, its
  end pushed out by the node's offset from the root so no part fades
  before the far proxy is drawn (far-field.md, "Lead bands").
- **Entities**: stream cell (high precision, `Stationary`, at the cell's
  origin corner) → one group per level (hidden until the level completes)
  → one batch per spawn item → one entity per LOD level and material part
  with `Mesh3d`, `MeshMaterial3d`, `VisibilityRange`, `RenderLayers`, and an
  `Aabb` from the model bounds (cooked meshes keep no CPU copy, so Bevy
  couldn't compute one). Despawn is budgeted one batch at a time.
- **LOD**: `lod_ranges(error × max scale, LodScreen, ring unload radius)`.
  Deviation: far-ring placements spawn their whole chain, not only the
  coarsest level. `VisibilityRange` shows the coarsest one at R2
  distances anyway, and promotion to R1 then needs no respawn.
  With a far field, far-eligible placements (`Placement::far_eligible`:
  statics, doors, lights without `NO_LOD`/`NEAR_ONLY`/`EDITOR_ONLY`/
  `DISABLED`) end their chain at the far field's handoff `d1` with a
  dithered band `[d1, d1 + w]` (`SpawnCx::end_band`), and every
  placement spawns at R1 (`LevelContent::proxies`).
- **Far-field hook** (`FarProxies`, a `CellProxies` implementation from
  `sf_lod_bevy`): a load to `Far` acquires the cell's L1 node in the assets
  stage, waits for it with the models, spawns it as the first spawn item
  under the level group (revealed with the group), and releases it when
  the cell drops below `Far` or the load is cancelled. Its byte estimate
  feeds the upload budget. Without a hook, streaming behaves as before.
- Statics carry `NoAutoAabb` (their `Aabb` comes from the cooked bounds),
  so Bevy's AABB pass skips them.
- **Upload**: the stage reveals the level group; its cost is the
  directory's byte estimate (8 MiB budget). Bevy's own per-frame limiter
  (`RenderAssetBytesPerFrame`) is **off** (changed 2026-10-11): in Bevy 0.20 a
  mesh instance collected while its mesh or material upload is deferred
  caches empty offsets or a stale slot and is never rebuilt, so objects
  vanished at random on Morrowind (far-field.md, "Bevy 0.20 pitfalls").
  Uploads are bounded by the in-flight asset caps instead (measured: no
  frame over 21 ms on the 60 m/s flight at 1440p). Restore the limiter from
  the same budget once Bevy re-extracts such instances.
- **Activate/deactivate**: `PlacementSpawner::activate`/`deactivate` per
  placement (no-ops for statics until M4).
- **Counters** (`sf_diag`): `stream/<stage>/ms|items|queue`,
  `stream/upload/bytes`, `stream/fetch/items`, `stream/tick/ms`,
  `stream/drive/ms`, `stream/requests`, `stream/resident_cells`,
  `stream/models`, `stream/page_cache/bytes`, `stream/oldest_request/s`,
  `stream/latency/s`, and big_space's `bigspace/{high,low}_precision/ms`.
- **`FadeTransition`**: a `StartTravel { traveler, to, rotation }` message
  starts it; at full black the traveler is reparented under the target
  root with new grid coordinates, a camera gets the target's render layer,
  `ActiveWorldspace` switches and burst mode starts; the fade holds until
  burst mode ends (or 10 s), then fades in. The overlay is an unlit black
  quad parented to the camera.
- **Ring overlay**: resident cells as content-AABB boxes (far blue, near
  green, active orange), pending requests inside them (white fetch/assets,
  yellow spawn/upload, red unload), anchor load radii as circles.

### Spikes (measured 2026-10-12, `.sessions/spikes.md`)

**Industry practice.** Streaming engines budget main-thread integration in
*time* per frame and share it across add and remove (UE level streaming:
`s.LevelStreamingActorsUpdateTimeLimit`, incremental component
(un)registration; Unity: `Application.backgroundLoadingPriority`, 2–50 ms of
integration per frame; DOTS: sections deserialized off-thread, then one bulk
`MoveEntitiesFrom`). They also keep *other* frame-level work off the frames
that already carry a big one-off cost, and they pause what nobody sees
(particle and animation update by visibility, as Niagara's scalability and
Unity's culling groups do). Our stages were already time-budgeted (above); the
spikes came from costs the budgets didn't see.

**Attribution** (RTX 4060 Laptop, 1440p, AC, release; `perf suite` hitch rows
plus a `profiling-trace` Chrome trace of the 60 m/s flight, `perf spikes` per
spike window). At 1440p the GPU is the limiter (97–99%), and the render thread
is the critical CPU path (10.3 ms per frame including 1.3 ms waiting on
present), so a frame spikes when *render-thread* work spikes:

| source | render / main thread cost | frequency |
|---|---|---|
| origin shift (every 256 m): every `GlobalTransform` changes, Bevy re-collects every mesh instance (`collect_meshes_for_gpu_building`) | 3–4 ms render; big_space 2–3 ms main | 20 per flight |
| particles: every resident emitter rewrote its mesh each frame, so Bevy freed, reallocated and re-uploaded ~120–250 meshes per frame (`allocate_and_free_meshes`) | 1.2 ms render **every frame** | constant |
| mesh-add bursts (a cell's models load together; Bevy's upload limiter is off, pitfall 1 in far-field.md) | 2–7 ms render | per loaded cell |
| `extract_visibility_ranges` (any `VisibilityRange` added/removed re-walks all ~40k) | ~1.1 ms extract | per spawn/despawn frame |
| terrain: every origin shift despawned and respawned all ~350 patches, and clipmap uploads (0.6–1 MB) land in the frame after | 1.5–2 ms main + entity churn | per shift |

**Fixes (built).**
- Particles pause outside the view: an emitter simulates and rewrites its
  mesh only while `ViewVisibility` (its visibility range: particles opt out of
  frustum culling) and a 12 m sphere in the camera frustum both hold
  (`mw_assets` particles). Meshes modified per frame: ~120 → ~8.
- Terrain origin shift updates the live patches in place (new transform and
  bounds on the same entities, no archetype move) instead of respawning them,
  and clipmap uploads wait 2 frames after a shift (`sf_terrain_bevy`).
- **Origin-shift guard** (`StreamingConfig::origin_shift_guard_s`, 0.05 s):
  while a floating-origin anchor is within 0.05 s of outward travel of
  big_space's recenter distance (`GRID_CELL / 2 + GRID_SWITCH`), and for
  `SHIFT_COOLDOWN_FRAMES` (2) after its cell changed, the main-thread stages
  (spawn, reveal, despawn, (de)activation) wait; at most `MAX_GUARDED_FRAMES`
  (6) in a row, never in burst mode; an anchor at rest never triggers it.
  Counter `stream/shift_guard/frames`.

**Measured** (3 runs each, back to back, release `solari,dlss` with
`BEVYRROWIND_DLSS=off`, 2560×1440; `target/perf/ab2-*`):

| scenario | before: median / worst (×) | after: median / worst (×) | frames > 1.5× median |
|---|---|---|---|
| flight 60 m/s | 11.14–11.31 / 20.0–22.4 ms (1.80–2.01×) | 10.83–10.93 / 18.2–20.2 ms (1.67–1.86×) | 32 → 9 |
| run 7 m/s | 11.04–11.14 / 17.1–48.8 ms (1.54–4.4×) | 10.53–10.60 / 16.2–21.3 ms (1.53–2.0×) | 4 → 12 |
| Balmora standing (no streaming) | | 12.07–12.16 / 14.2–16.4 ms (1.17–1.35×) | 0 |

The gate (worst ≤ 1.5× median) still fails. Of the 25 spike frames left,
about 10 carry no streaming, shift or asset churn and are followed by a short
frame (7–9 ms): late presents, not work (the run's count rose because the
median dropped 0.5 ms and these now cross the line). About 5 are origin
shifts themselves (render re-collection, big_space up to 5.7 ms), 4 follow
mesh-add bursts (200–425 meshes), 3 follow despawn of a whole cell, the rest
mixed. Occasional 30–1,600 ms stalls with every counter inflated appear in
before and after runs alike (machine, not streaming).

**Load budget (built, spikes2–3).** Bevy's upload limiter can't pace mesh
bursts (far-field.md pitfall 1, correction): the 0.20 mesh allocator writes
every extracted mesh in its extraction frame. So pacing happens where the
loads start: `StreamingConfig::max_loading_meshes` (default 64, 0 = no cap,
never capped in burst mode) queues model loads (`stream/model_queue`) while
the models in flight × a running mean of meshes per model (EMA 0.05) would
exceed it, and one settle frame separates a load's assets becoming ready
from its spawn, so the mesh upload and the spawn's extraction don't land in
the same render frame. App A/B: `BEVYRROWIND_LOADING_MESHES=<n>`. Measured
against unpaced (spikes3, 1440p window, GPU thermally throttled at 86 °C /
~2.3 GHz, 3 runs each): flight 0–2 frames over 1.5× median either way,
run medians ~1 ms lower paced but inside the thermal drift; not worse, kept.

**Measurement setup matters** (spikes3–4; diag.md "Frame pacing" has the
matrix). At 2560×1440 on the 1080p laptop panel the GPU is the limiter and
its waits absorb render-thread spikes: 0–7 stutters a minute on the flight,
whether the window is oversized or fits and renders 1440p offscreen. In a
borderless 1080p window, uncapped (~118 fps), the render thread becomes the
critical path and the flight shows 21–27 stutters a minute (run 3–9), about
90 % of them with a work spike in the same or next frame: render-thread
busy 12–15 ms against a ~6 ms median, mostly in frames with no mesh,
spawn or origin churn, plus ~4 origin shifts per flight. Fifo at 60 Hz
hides them all (0 stutters). Pacing runs therefore use a borderless window
at the display's resolution, uncapped, and gate on presented stutters
(≤ 1 a minute, `benchmarks/exterior_{run,flight}.json`); today they fail.

**Left (levers, in order):** pace mesh uploads by bytes (stagger a request's
model loads, or restore Bevy's limiter once streaming waits for GPU residency
before spawning: far-field.md pitfall 1); upstream an incremental
`extract_visibility_ranges` (Bevy re-walks every range on any change; its
`RenderVisibilityRanges` fields are private, so no isolated workaround);
background-priority IO/async task-pool threads (Bevy's `TaskPoolBuilder::
on_thread_spawn`) if loaders contending with the render thread explain the
churn-free spikes; find what the churn-free render-thread spikes are (a `profiling-trace` of a
borderless 1080p flight, spikes only: likely candidates are pipeline or bind
group creation, buffer reallocation and GPU-preprocessing re-collection);
the lazy floating origin (a separate origin entity following the camera with
two-cell hysteresis, so shifts drop by half or more; the shift guard's
prediction in `stream.rs` must follow it). The gate formulation is settled
(diag.md "Frame pacing").

### Visibility range slots (lodaudit, 2026-10-19)

Bevy 0.20 keeps one GPU slot per **distinct** `VisibilityRange` for the app's lifetime
(`RenderVisibilityRanges` in bevy_render `view/visibility/range.rs`: never reused, so baked mesh
indices stay valid) and has 65535 of them; past that, new ranges share slot 0 (one `warn_once`), and the
shader dither (`get_visibility_range_dither_level`) reads slot 0's bounds while the CPU check still says
visible: a near mesh can dither fully out. Our ranges were per instance (scale × LOD error, size-cull
radius, subtree offset, far-node lead), so nearly every placement minted new ones and a long flight walks
toward the cap. Industry engines share LOD distances per level or per class, never per instance (UE
screen-size per LOD, HLOD per layer). As built: `sf_world_bevy::shared_range` rounds every bound **up**
onto a log grid (`RANGE_STEPS_PER_OCTAVE` = 32, ~2.2 %); monotone and bit-stable, so matched crossfades
and proxy lead overlaps survive. Used by `visibility_range` (static chains, subtree ranges) and
`sf_lod_bevy` node ranges. Left: `mw_assets` interior cooked path still builds raw ranges (few per cell);
unverified live that the cap was the owner's trigger (warning not hit in a 5-spot BRP flight).

## Verification

- Unit tests (`sf_world`): scheduler outputs are deterministic for scripted
  anchor paths; an anchor oscillating across a ring boundary causes at most
  one load and one unload per real crossing; budgets are never exceeded by
  more than one item.
- Index tests: each placement lands in exactly one cell; content AABBs
  contain all placement bounds; an unchanged region fingerprint reuses the
  cached pages (sf_world checks the fingerprint side; the cache hit is
  sf_bake's).
- Perf baseline (2026-10-11, release, shared machine; `cargo test -p
  sf_world --release -- --ignored --nocapture tick_cost`): 100,820
  placements in 5,041 cells index in 219 ms; a player plus camera flying at
  60 m/s cost 45 µs per scheduler tick on average, 303 µs worst (probably the first
  tick, which creates every request at once).
- `sf_diag` fly-throughs on a scripted spline across the largest exterior, at
  a run (7 m/s) and a fast flight (60 m/s). Frame-time CSV plus per-stage
  counters (time, items, queue depth, oldest request age). Gate (WISHLIST
  §0, as measured since 2026-10-12, diag.md "Frame pacing"): at most 1
  presented stutter (frame > 1.5× the moving median of 60) a minute,
  none > 33 ms, uncapped in a borderless window that fits the display.
  Measured on Morrowind with the far field (2026-10-11, RTX 4060 Laptop,
  1440p, AC, idle): run 11.3 ms median / 18.4 worst, flight 10.9 / 20.9
  (none over 33 ms; the 1.5× ratio still fails: streaming frames cost
  spawn/despawn 1–3 ms plus Bevy's full `VisibilityRange` re-extraction);
  without it 23–28 ms medians, 53–66 ms worst. Resident entities in a town
  dropped from ~350k to ~48k, which is what the R2 far ring held.
  2026-10-12 ("Spikes" above): flight 10.9 ms median, worst 1.67–1.86×, frames
  over 1.5× down from 32 to 9 per 3 runs; the gate still fails.
  2026-10-12 (spikes4, borderless 1080p uncapped): flight 8.5–8.7 ms median,
  21–27 stutters/min; run 8.7–8.9 ms, 3–9 stutters/min; the gate fails.
- A scripted walk through a door: total fade ≤ 0.6 s when prefetched;
  screenshots on both sides. Measured 2026-10-12 (Morrowind, release, `view Balmora ... --use-door 2`):
  into the Balmora Mages Guild 0.63 s (prefetched: 0.6 s of fade plus a frame), back out 1.02 s: the
  exterior's R1 was dropped while inside and the fade holds for it. Fixed 2026-10-12 (doorfix) with
  the residency cache below: back out 0.64 s, in 0.63 s (dev build). Starting in an interior, the first
  trip out holds ~1 s (only the door's R0 was ever prefetched). Screenshots: `shots/doors/`.
- Debug overlay: stream cells colored by ring and state, checked visually.

## Decided (2026-10-11, owner's priorities: speed, visuals)

- **Ring radii scale with the quality preset.** The radii above are the
  "high" defaults. The fly-through calibrates them to the largest radii
  that still meet the §0 gates, so view distance is bought with measured
  frame time, never guessed.
- **No colliders outside R0.** Long-range projectiles raycast against
  terrain (`TerrainHeights`) and coarse bounds instead.
- **Processing range = R0 coverage (walkers2, 2026-10-12).** An actor simulates (AI,
  movement, foot probes) only in the active worldspace and only while every cell within a small
  margin of it is `Level::Active` (`Scheduler::covers(ws, pos, margin, Level::Active)`); R0
  activation is what builds its colliders, so the activation ring and the collision ring are
  the same ring by construction. Outside, actors are dormant (`sf_controller_bevy::Dormant`:
  parked, driver paused, low animation tier) and resume in place when covered again. Spec:
  locomotion.md "Processing range". Empty cells (not in the directory) count as unloaded.

## Portal crossing (M8)

The 2026-10-12 spike and the M8 design are in `portals.md`: per-end frames
from the bake, one render-to-texture portal view, crossing through
`DoorLink::cross` with no fade; `FadeTransition` stays for fade-only links.

## Industry comparison

J9 (industry-audit.md), 2026-10-12, docs research only (`.sessions/j9rest.md`).
The cell-streaming comparison is industry-audit.md (rows 1–3, 18–21, 30,
34, with sources); door transitions are compared above; this table indexes
them and adds what neither covered. "Unverified" marks claims from memory.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Grid partition, per-source ranges, hysteresis, velocity prefetch | UE World Partition, RAGE/Decima/Spider-Man cell streaming | yes | **match** (audit row 18) |
| Time-budgeted integration, upload pacing | UE `s.LevelStreamingActorsUpdateTimeLimit`, texture/mesh pools; Unity async upload | yes / partial | audit rows 19–20 |
| Size-based draw and load distance, per-category importance | UE cull distances, RAGE `lodDist`, Bethesda fade categories, OpenMW min size | J1 built, J3 planned | audit rows 1, 2, 34 |
| Load doors + prefetch | Bethesda load doors, OpenMW preload; UE "loaded, hidden" | Prefetched state, fade | **match** (above) |
| Fast I/O path | DirectStorage + GPU decompression (GDeflate), PS5 Kraken hardware decode (Ratchet & Clank), UE IoStore + Oodle; Unity async read manager | LZ4 `.sfmesh` decoded on the task pool, one file per product | **differ**: decode runs at GB/s and isn't the bottleneck (spikes are main/render-thread work); revisit only with HD texture packs (texture streaming, audit row 23) |
| **Runtime-toggled placement groups** | UE World Partition **Data Layers** (runtime states Unloaded/Loaded/Activated, per-layer HLODs, unverified detail); Creation Engine **enable parents** (one marker toggles a group; previs/precombines are broken by toggled refs, hence the "don't disable precombined refs" rule, unverified); older Creation-era scripts toggle single refs | `DISABLED` flag + overlay "removed ids"; no group toggle, no re-enable state, far field baked without toggles | **GAP (P2)**: open question 2 |

**Sources:** industry-audit.md "Sources"; UE Data Layers:
https://dev.epicgames.com/documentation/en-us/unreal-engine/world-partition---data-layers-in-unreal-engine
(not re-read); Creation Kit enable parent:
https://ck.uesp.net/wiki/Enable_Parent (not re-read).

## Open questions

1. Portal frames for M8: adapters must derive `frame` and `extent` from
   door geometry. Point links work until then.
   *Feasibility measured (2026-10-12, bevyrrowind `world-adapter.md`
   "Portal frames: feasibility"):* the Morrowind adapter derives frames
   from door model bounds + the landing arrival marker (which fixes the
   front side); of 1,828 links, 1,361 portal-ready, 251 need an extent
   fix (openings differ, or a thick end like a cave mouth), 216 stay
   fade-only (one-way, trapdoors). Proposed convention, not yet in the
   type docs: `frame` sits at the opening's **bottom center** and
   `extent` spans x ∈ ±w/2, y ∈ 0..h, so floors line up when the two
   openings differ in size; a link's two ends share one extent (the
   per-dimension minimum). Arrival markers are not foot positions (they
   float ~1.2 m in Morrowind): arrivals must be grounded at runtime.
   *Industry comparison:* Valve's Portal ships fixed-size rectangular
   portals, each with a linked partner; the crossing transform is
   `partner · R_y(π) · self⁻¹` (ours), views render recursively through a
   stencil with an oblique near plane on the exit plane, and the player
   teleports when its center crosses the plane. Engine-agnostic
   "seamless portal" kits (UE `SceneCaptureComponent2D` + render target,
   Unity seamless-portal assets) author per portal: the partner, the
   frame transform, the opening size, a crossing trigger volume, and a
   recursion limit; "bigger on the inside" spaces (Antichamber, the
   Tardis trick, CodeParade's non-Euclidean rooms) are rigid portals
   whose two sides are simply laid out apart, as here. Scaled portals are
   rare and change the traveler's size, so we stay rigid and clamp the
   extent. Bethesda's Creation Engine keeps load doors + a teleport
   marker (no portal), consistent with Morrowind's data having only the
   marker; Prey (2017) sells one continuous station but still loads
   between its sections at transit doors (no see-through portals). What
   adapters must supply per end is therefore exactly frame + extent (+
   the door placement to hide or swing), which `LinkEnd` already has.
2. **Runtime-toggled groups (J9, P2).** A per-placement `enabled` state in
   the overlay (not removal: re-enable must restore it) and an optional
   `group: RecordRef` on placements so one toggle flips a set (UE Data
   Layers, Creation enable parents); the cell index keeps disabled
   placements so toggling never re-reads the source. Far field: placements
   in a toggleable group are excluded from merged proxies (or get their own
   proxy per group state, like UE per-data-layer HLOD), so the horizon never
   shows a building that a quest removed. Size the cost on the content
   pack's script-toggled refs first (adapter count).
