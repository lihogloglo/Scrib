# Architecture vs Bevy 0.20: streaming and rendering (2026-11-12)

> **Correction (orchestrator, 2026-11-12):** `sf_shadow_cache_bevy` is not installed in the game (`POINT_SHADOW_CACHE = false`, `bevyrrowind/crates/mw_environment_bevy/src/render.rs:37`, since 2026-10-21). Lamp faces use the vendored `bevy_pbr` per-face cache. Findings below about the cache's transient capture views describe code that does not run; the 64k work items per face come from stock Bevy adding every GPU-culled mesh to every point face. See `docs/ledger/shadows.md`.

Read-only review from code and Bevy's source only (no project docs read, by brief). Bevy facts and their
line references are in `docs/wiki/bevy/render-and-visibility.md` ("wiki §n" below). Nothing here was run or
profiled; where a cost is attributed it is reasoning from the code plus the day's measurements, and is marked
"inferred". Not read in depth: `sf_lod_bevy/src/runtime.rs`, `sf_scene*` internals, the vendored
`bevy_pbr/src/gpu_instance_batch`, terrain, sky, post-processing.

## Verdict

The base is sound. Scrib draws through Bevy's own path almost everywhere: `Mesh3d` + `StandardMaterial` entities,
`VisibilityRange` LODs, stock `SkinnedMesh`, stock cascades, big_space with dirty-tree pruning. It is not a
case of a custom renderer fighting the engine.

Two things are structurally off:

1. **The lamp shadow cache is built on throwaway views, and Bevy 0.20's renderer is built on views that
   persist.** That single mismatch is what blocked `NoCpuCulling`, which is Bevy's own answer to the per-view
   CPU cost of static meshes.
2. **Three near-static paths exist at once** (per-placement entities, per-cell precombines, the `sf_scene` GPU
   instance batches). The third re-implements what Bevy 0.20 already does (retained bins, GPU bin unpacking,
   GPU culling, multi-draw indirect) outside the ECS.

One caution about priorities, from the measurements themselves: near ring off gives 7.2 ms, so the whole near
ring is about 3.1 ms of 10.3, of which NPCs are about 2 ms and statics about 1.1 ms. Statics are the smallest
of the three pieces. Any statics-only rework has a ceiling near 1 ms. The 7.2 ms floor (terrain, far field,
sky, post, the remaining views) was outside what I read and is where a 7 ms stretch target would be decided.

## 1. Map: what we do vs what Bevy provides

| Area | Ours | Bevy 0.20 | Status |
|---|---|---|---|
| Entity per mesh | One entity per placement x LOD level x material part, `spawn_batch`, `sf_world_bevy/src/spawn.rs:333-369`; exact `Aabb` + `NoAutoAabb` | Same model (`bevy_city`, wiki §9) | Using it |
| Merged statics | Per-cell precombine buckets replace member entities, `stream.rs:1294-1349`, `precombine.rs` | Nothing native (no HLOD/merge) | Fills a real gap |
| LOD | `VisibilityRange` per level, ranges snapped to a shared log grid, `spawn.rs:431-484` | wiki §2 | Using it; the snapping works around the `NonMaxU16` range table |
| CPU culling | Default for everything | wiki §1, §3 | Using it, paying cameras x entities and lamps x entities each frame |
| GPU culling (`NoCpuCulling`) | Opt-in env flag, off, `mw_exterior_bevy/src/exterior.rs:279-282`; `spawn.rs:276-292,363` | The retained path, wiki §1, §4, §6 | Blocked by our shadow cache (below) |
| Retained bins, specialization | Stock, plus vendored #26095/#26418 | wiki §5 | Using it, but transient capture views defeat it |
| Mesh slabs | Stock; we throttle loads (`max_loading_meshes`, `stream.rs:83-91`) because uploads have no per-frame cap | wiki §6 | Using it; sensible workaround |
| Multi-draw indirect | Stock; material classes fold statics into shared pipelines (`matclass.rs`, `StreamingConfig::material_classes`) | wiki §5, §6 | Using it well |
| Occlusion culling | Only with `--prepass`/DLSS path: `exterior.rs:827` | wiki §6 | Using it conditionally |
| Sun shadows | Stock cascades, own split builder, `exterior.rs:1298`; vendored PCSS shader changes | wiki §3, §7 | Using it |
| Point shadows | Own cache: `sf_shadow_cache_bevy` (1.6k lines) + vendored scheduling patches | No caching, six views per light per frame (wiki §7) | Layer on top; needed, but its design fights retention |
| Skinning | Stock `SkinnedMesh`, one bone entity per joint (`sf_anim_bevy/src/lib.rs:413`), per-material merged meshes (`sf_character_bevy/src/lib.rs:887-895`); vendored P5/P6 in `vendor/bevy_pbr/src/render/skin.rs:8,534` | wiki §4 | Using it, patched for sharing |
| Transform propagation | `TransformPlugin` disabled (`exterior.rs:287-290`); big_space propagates; cells are `Stationary` (`stream.rs:1262`); vendored dirty-tree skip (`vendor/big_space/src/grid/propagation.rs:439`, `plugin.rs:183`) | `mark_dirty_trees`, wiki §8 | Equivalent to Bevy's path |
| Floating origin | big_space `Grid(256 m, 32 m)`; a cell switch rewrites every `GlobalTransform`, so all meshes re-extract that frame; streaming pauses around it (`stream.rs:72-81`) | None native | Reasonable; the shift spike is inherent |
| GPU instance path | `sf_scene` (6.5k lines), `sf_scene_bevy` (4k), vendored `gpu_instance_batch`; off unless `--scene` (`exterior.rs:299-304`) | Bevy's retained multidrawable bins do the same job (wiki §5) | Duplicating |

### Where the per-frame CPU goes (inferred from the code)

Main thread, every frame, standing or walking, for N mesh entities that are CPU-culled (about 31.7k):

- two bookkeeping passes over all of them (`reset_view_visibility`, `mark_newly_hidden_entities_invisible`);
- one range pass that rebuilds a hash map of every in-range entity, then a hash lookup per entity per view;
- one frustum pass per camera, one pass per directional light covering its cascades;
- **one full pass per shadowed lamp** (wiki §3). With the High preset's lamp budget this is the largest
  multiplier: lamps x N, each with a range hash lookup and sphere/OBB tests;
- our own additions on the same data: `track_casters` (`sf_shadow_cache_bevy/src/lib.rs:345`),
  `add_gpu_culled_casters` (`:499`), `classify_lights` (`:652`).

Render thread: per view, a sorted-list diff over everything visible (O(visible) even if nothing changed), then
bin inserts and removals for entities that crossed a frustum edge or an LOD band. A crossing costs more than it
looks: an entity invisible in every view is removed from `RenderMeshInstances` and re-extracted when it returns
(wiki §4), and each LOD swap is a removal plus an insertion in every view. Walking moves the main frustum, all
three cascades and the range origin at once, which fits "walking is CPU-bound, standing is GPU-bound". On top
of that, about 22 views means about 22 schedule runs, each with fixed per-view work.

## 2. Right, wrong, delete

### Right

- Entity-per-mesh with exact bounds, `NoAutoAabb`, `spawn_batch`, `RENDER_WORLD`-only meshes: this is the
  configuration Bevy benchmarks.
- Precombine: fewer entities is the one lever that reduces every cost listed above at once, and Bevy has no
  native equivalent.
- Material classes and shared samplers: they make Bevy's multi-draw batch sets large, which is how the retained
  path is meant to be fed.
- Quantised visibility ranges: correct reading of Bevy's range table limits.
- Transform handling: `Stationary` cells plus dirty-tree pruning gives the same result as Bevy's
  `StaticTransformOptimizations`.
- Characters on stock skinning with merged per-material meshes and an outfit cache; animation tiering with a
  budget; bones written only for posed characters.
- Budgeted streaming stages and the origin-shift guard.
- Caching lamp shadows at all: stock Bevy redraws six faces per lamp per frame, so some cache is required.
- Carrying upstream PRs as vendored patches rather than reaching into render internals from outside.

### Wrong

- **Capture views are created fresh for each capture and despawned the next frame**
  (`sf_shadow_cache_bevy/src/render.rs:480-502`, `:504-557`; the code comment says so: "fresh each capture,
  every caster 'added', so Bevy specialises and bins them all for the new views"). In a retained renderer a new
  view key is the most expensive event there is: every entity in its list is specialised and binned from
  nothing, and for GPU-culled meshes Bevy flushes the entire `RenderGpuCulledEntities` table into a just-added
  subview (wiki §4). That matches today's failed A/B exactly: each capture face received all 64k static
  instances. The failure was read as "`NoCpuCulling` is worse"; from the code it is "transient views are
  incompatible with `NoCpuCulling`". (Inferred mechanism; confirm by logging the capture subviews' added count.)
- **Static casters are tracked twice.** With CPU culling Bevy scans all casters per lamp, and the cache also
  keeps its own caster table with settle timers and per-light lists (`lib.rs:280-345`, `:464-499`). The
  opt-in P3 path (lamps take statics from the cache) is the right direction; the default still pays both.
- **Skinned meshes always count as movers** (cache module docs, `lib.rs:26-28`), so one idle NPC inside a
  lamp's range keeps that lamp's faces alive as real views every frame. With 568 characters of which about 20
  are posed per frame, most "movers" are not moving. This is a likely reason the walk has about 18 lamp face
  views instead of a handful (inferred).
- **Every character in the ring is a full rig.** Entity counts suggest about 27k of the 59k entities are bone
  entities (59k total minus 31.7k meshes; inferred). There is no distance tier in the character layer: no
  `VisibilityRange`, no rig-less far representation (`grep` of `sf_character_bevy/src`). The animation tiers cut
  evaluation, but a frozen character still carries its bone entities, its skinned mesh entities in every
  culling pass, and its mover status in the shadow cache.
- **Three statics paths.** Each new path was added beside the previous one rather than replacing it, and the
  streaming code carries the branching for all three (`stream.rs:807-908`, `scene.rs`, `precombine.rs`).
- **All LOD levels stay alive as entities** even inside precombine buckets (`spawn.rs:337-351`). This is
  Bevy's model, so it is not a mistake in itself, but it multiplies N for every main-thread pass. It stops
  mattering once statics are GPU-culled.

### Delete

- `sf_scene`, `sf_scene_bevy`, the vendored `gpu_instance_batch` module and the `--scene` wiring, unless a
  measurement shows it beating precombine + `NoCpuCulling`. It is about 10k lines that duplicate Bevy's retained
  bins, and it adds its own shadow-cache special case (`GpuBatchedLightVisibility`, `ShadowCastersChanged`).
- After change 1 below: `GpuCulledLists` / `add_gpu_culled_casters` as a per-frame system (the cache would list
  static casters only when it captures), and the settle-timer tracking for entities already known to be static.

## 3. Differences from `bevy_city` / `many_cubes`

| | Bevy's examples | Scrib (Balmora walk) | Plausible CPU effect |
|---|---|---|---|
| Views per frame | 1 camera + default cascades | about 22 (camera, 3 cascades, lamp faces) | Fixed per-view schedule cost x 22; per-view diffs and bin churn x 22. Largest structural difference on the render thread |
| Shadowed point lights | none | a budget of lamps, each scanning all casters on the main thread | lamps x N main-thread scans; no equivalent in the examples |
| View lifetime | all views permanent | capture views created and destroyed; face views detached and re-attached | Re-specialise and re-bin on each new key; breaks `NoCpuCulling` |
| `NoCpuCulling` | available as the fast path, inserted once | off | All CPU-culling passes stay on for statics |
| Transforms | stock plugin, static optimisation on | big_space, same optimisation | none expected |
| Skinned characters | `many_foxes`: every fox animates, one directional light | 568 rigs, about 20 posed, plus lamps | Idle rigs still cost culling, extraction scans and shadow views |
| Prepass / occlusion | `bistro`: `OcclusionCulling` on camera and sun | on with DLSS; not on the sun | Sun cascades draw occluded casters; GPU cost, not CPU |
| Extra entities | meshes only | about 27k non-mesh entities (bones, cells, groups) | Table scans for `Changed<GlobalTransform>` cover them; small |
| Render layers | default | per-worldspace layer pairs on every part | Per-entity layer test in each pass; small |

The examples never combine "many views" with "many CPU-culled entities". Scrib does, and both multiply.

## 4. The one or two changes

**1. Make lamp shadow caching work with persistent views, then turn `NoCpuCulling` on for statics.**
Give each cache slot face a stable `RetainedViewEntity` that lives as long as the slot, and run its schedule
only on capture frames (the cache already drops `RootNonCameraView` to park Bevy's face views,
`render.rs:240-298`; the same trick parks a capture view). The view's bins then survive between captures and a
recapture costs only what changed. Keep GPU-culled statics out of the per-frame mover faces (the vendored
`CpuListedPointShadowCasters` already does this for point faces). With that in place the static parts can carry
`NoCpuCulling`, which removes them from every main-thread pass in §1 (camera, ranges, cascades, each lamp) and
from the render thread's per-view list diffs and frustum/LOD bin churn while walking; their instances stay
resident on the GPU. This is exactly the configuration `bevy_city --no-cpu-culling` exercises, so it is
Bevy's path rather than ours. Once it holds, `sf_scene` has no remaining purpose. Expected effect: bounded
by the statics' measured share (about 1 ms of frame time), but it lands on both CPU-bound threads, and it
removes a layer instead of adding one.

**2. Give characters a real distance tier, and stop treating unposed characters as shadow movers.**
NPCs are the largest measured near-ring cost (about 2 ms) while only about 20 of 568 are posed per frame, so
most of that cost is being paid for characters that are doing nothing. The standard approach (crowd tiers in
UE Mass, RDR2, Assassin's Creed) is that only nearby characters are full skeletal actors; farther ones are a
cheaper representation or not materialised. Here that means: beyond a distance, no rig entities and no skinned
meshes (a rigid posed mesh or nothing), promoted when the player approaches; and in the shadow cache, a
character whose animation tier is frozen is a static caster until it is posed again. The second half alone
should cut the number of live lamp face views, which is the main reason Scrib runs about 22 schedules a frame.
Before building, measure the ceiling: `--off` runs for "characters beyond X m" and "skinned casters not in
lamp shadows" give the available milliseconds directly.

Not recommended: further work on per-entity CPU culling cost, more special cases in the shadow cache for new
caster kinds, or continuing `sf_scene`.
