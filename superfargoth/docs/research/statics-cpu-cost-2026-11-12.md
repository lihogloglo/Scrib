# Statics CPU cost on the Balmora walk (2026-11-12, research slice for perf-push)

Question: why the near ring costs ~3.7 ms of CPU on the walk (10.9 ms with it, 7.2 ms GPU-bound without), and
what Bevy 0.20 already has to remove it. Evidence: `target/perf/tr_{on,off}.{trace,spikes}.txt` (traced, slower
than real), `bevyrrowind/benchmarks/runs/ceil_base/balmora_walk@2560x1440-3.md` (untraced gauges, "base" below),
`ceil_statics_off/balmora_walk@2560x1440_off=statics-1.md` ("off"). Registry sources are
`~/.cargo/registry/.../bevy_{camera,light}-0.20.0`; `vendor/` means `superfargoth/vendor/`.

## 1. Where the time goes

**Both threads are long, and the render thread is longer.** Traced, the main app takes 11.35 ms and the render sub-app
12.63 ms per frame (off: 7.32 / 8.82). With pipelined rendering the frame is the slower of the two, so a fix that
only cuts PostUpdate (main thread) shows on the frame only once the render thread is also cut. Each slice below says
which thread it cuts.

**Counts (base, medians).** 22 views = 1 main + 3 sun cascades + 18 lamp faces (3 shadowed lamps x 6), max 54
(base:271, 281, 290). Mesh entities 31.7k (off 3.2k), entities 59.2k (off 8.1k), 286 point lights, 568 characters
(base:315, 316, 117, 114). So the near ring holds ~28.5k mesh entities: the ~14k statics plus NPC body parts and
the rest. No gauge splits that yet (Open 1). Lamp faces are mostly skipped: `views/lamp_face/work_items` median 0,
max 2577, while 1210 items stay queued (base:294, 296).

**PostUpdate (+2.9 ms wall traced) is CPU culling: one full scan of every mesh entity per pass.** Thread time per
frame, on vs off (`tr_on.trace.txt`, `tr_off.trace.txt`):

| pass (Bevy system) | query in the trace | on | off |
|---|---|---:|---:|
| camera cull `check_visibility_cpu_culling` (bevy_camera `visibility/mod.rs:748-876`) | `(Entity, &InheritedVisibility, &mut ViewVisibility, Option<..>` | 2.12 | 0.25 |
| point lights `check_point_light_mesh_visibility` (bevy_light `lib.rs:517-690`) | same prefix, second row | 1.14 | - |
| sun cascades `check_dir_light_mesh_visibility` (`lib.rs:342-470`) | `(Entity, &InheritedVisibility, Option<&RenderLayers>..` | 1.23 | 0.19 |
| `check_visibility_ranges` (bevy_camera `range.rs:230-282`) | `(Entity, &GlobalTransform, Option<&Aabb>, &VisibilityRange)` + system | 1.59 | 0.28 |

(The two `&mut ViewVisibility` rows are truncated in the trace text; which is camera and which is lamps is a guess.
Their sum is what matters.) That is ~6.1 ms of thread time against a +2.9 ms wall delta: the scans run at about two
threads' worth of parallelism, because they are chained (ranges -> camera cull -> light culls -> mark hidden:
`visibility/mod.rs:517-541`, `bevy_light lib.rs:217-229`) and each fork/join is short.

How it scales: the point-light pass runs one `par_iter` over **all** mesh entities for **each** shadowed lamp
(`lib.rs:594`, inside the per-light loop), and also writes `ViewVisibility`. The sun pass is one scan with 3-4
frustum tests each. So the cost is about `N_mesh x (camera + sun + ranges + N_shadowed_lamps)`: ~31.7k x 6 for the
walk median. At 8 shadowed lamps (48 faces) that becomes 31.7k x 11, which fits the 18 spikes of 3.3-4.0 ms in
`check_point_light_mesh_visibility` (`tr_on.spikes.txt`).

**CPU culling also keeps the retained render bins churning.** A CPU-culled mesh that enters or leaves a frustum changes
`ViewVisibility`, so it is re-extracted (`Changed<ViewVisibility>` in `vendor/bevy_pbr/src/render/mesh.rs:2185`),
re-queued and its GPU bin entries rewritten. Deltas: `write_binned_instance_buffers<Shadow>` +0.33,
`<Opaque3d>` +0.21, `<Opaque3dPrepass>` +0.17, `queue_shadows` +0.15, extraction of changed meshes +0.34
(in Extract, which stalls both threads).

**Render (+3.8 ms) is mostly command recording plus per-frame bind groups.** `per_view_shadow_pass` +1.13,
`shared_shadow_pass` +0.65, main opaque + early prepass +1.36, `prepare_preprocess_bind_groups` +0.55. Multi-draw
indirect is in use: one draw per batch set (`vendor/bevy_render/src/render_phase/mod.rs:1382-1426`; 83 batch sets
in all views, base:269). That alone does not explain over a millisecond of recording. The suspects are per-entity
draws: `render_unbatchable_meshes` (`mod.rs:1435-1492`), possibly NPC skinned or morph parts, and none of our gauges
counts them (Open 2). Preprocess bind groups are rebuilt every frame per (view, phase): 186 created out of 186 (base:174;
`vendor/bevy_pbr/src/render/gpu_preprocess.rs:2541-2600`).

## 2. What Bevy 0.20 offers

| feature | removes | costs | shadow views? | issues |
|---|---|---|---|---|
| **`NoCpuCulling` per entity** (`visibility/mod.rs:61-69`, upstream #22767, #23107) | the entity drops out of every CPU pass above: all of them filter `Without<NoCpuCulling>` (`mod.rs:771, 733, 909`, `range.rs:236`, `lib.rs:367, 547`). Its `ViewVisibility` follows `InheritedVisibility` only on change (`mod.rs:884-906`). The render world keeps a retained table, `RenderGpuCulledEntities` (`vendor/bevy_pbr/src/render/mesh.rs:1934-2026`), and gives each view only the added and removed entities (`mesh.rs:2547-2620`), so bins stop churning. #23107 measured `check_visibility` 1.19 ms and `check_dir_light_mesh_visibility` 4.33 ms disappearing on bevy_city | each such entity sits in **every** view's bins (filtered only by `RenderLayers`). The GPU frustum- and range-culls it per view (`mesh_preprocess.wesl:217-252`). A view that appears later gets a **full flush**: every GPU-culled entity is queued into it at once (`mesh.rs:2565-2583`) | yes. Cascades and point and spot faces all run GPU preprocessing with `FRUSTUM_CULLING` (`gpu_preprocess.rs:628-680`). Lamp faces use the light's own position for ranges, since we spawn no `ShadowLodOrigin` (`vendor/bevy_render/src/view/mod.rs:1255-1275`) | #24252 reverted the GPU range culling that #23115 had tied to `NoCpuCulling` (#23991, shadows vanished in point views). The vendored 0.20 shader still culls by range for any mesh with a range index (`mesh_preprocess.wesl:230-252`): verify in the slice. #25764 (open): changing a view's render layers with `NoCpuCulling` meshes does not work |
| retained bins, "queue only on change" | already on: `iter_to_queue` / `iter_to_dequeue` (`vendor/bevy_pbr/src/material.rs:1277, 1294`, `render/light.rs:3461-3467`); bins live in GPU buffers unpacked on the GPU (`gpu_preprocessing.rs:3194-3305`) | per-frame cost then grows with **churn**, which CPU culling produces | yes | - |
| `VisibilityRange` | distance culling plus dithered crossfade | under CPU culling it adds one more full scan (`range.rs:256`) | yes | above |
| mesh slabs + multi-draw indirect | draw calls per bin (14 slabs, 493 MB) | - | yes | DX12 has no indirect count (wgpu #7974; we are on Vulkan) |
| point-light cube faces | #26149/#26161 (own fork) skip unchanged faces in batching and recording (`gpu_preprocessing.rs:2369, 3214`, `light.rs:1424-1470`) | the main-world scan per lamp stays | - | bevy #25576 open |
| `RenderLayers` | a coarse per-view filter, which also works for GPU-culled entities (`mesh.rs:2526-2540`) | changes what other cameras and lights see | yes | #25764 |

Conclusion: `NoCpuCulling` on statics is Bevy's own answer to this exact profile. The one problem it creates for us
is the lamps. Our `sf_shadow_cache_bevy` gives cached lamp faces movers only and draws statics through its own
capture views (`crates/sf_shadow_cache_bevy/src/lib.rs:1-43`, `render.rs:476-493`). If statics were GPU-culled in
every lamp face, the cached faces would redraw them, and each capture subview (fresh every capture) would full-flush
all ~14k statics into its bins: 6 x 14k queue operations per capture, a spike.

## 3. Industry standard

- **GPU culling for every view.** UE5 GPU Scene with instance culling per view (main, cascades/VSM, local lights);
  Unity's GPU Resident Drawer / BatchRendererGroup; Haar and Aaltonen, "GPU-Driven Rendering Pipelines"
  (SIGGRAPH 2015); Wihlidal, Frostbite (GDC 2016). The CPU keeps a coarse cull at most: by cell or cluster, never
  by part per view.
- **Draw commands cached for static primitives.** UE's cached mesh draw commands are rebuilt only when a primitive or
  its bindings change. Bevy's retained bins are the same idea, but they need stable visibility to pay off.
- **Static shadow caching for local lights, dynamic casters on top.** Unity HDRP's cached punctual shadows with
  "always draw dynamic", UE's stationary-light static shadow depths plus per-object dynamic casters. We already do
  this (sf_shadow_cache). The caster list of a cached static light is computed when the light is captured, not every
  frame.
- **Sun shadows: cached or staggered cascades.** UE VSM caches static pages and invalidates them when the light or a
  caster moves; staggered far-cascade updates are common elsewhere.
- **Merged statics per cell / HLOD.** Already built as precombine (`docs/specs/precombine.md`); what remains are the
  classes it excludes (doors, lights, items, scripted statics, translucents).

Least new code on Bevy 0.20: GPU culling via `NoCpuCulling` for statics, plus a cached per-light caster list owned
by sf_shadow_cache. Both mechanisms exist; the work is wiring them together and a small vendored filter.

## 4. Recommended slices

### Slice 1: `NoCpuCulling` for near statics; lamps take static casters from the cache's list

- **Spawn.** Statics get `NoCpuCulling`, where `apply_subtree_ranges` already tags new meshes
  (`crates/sf_world_bevy/src/spawn.rs:215-240`). A static here is a non-skinned mesh whose placement class is
  static and that is not `ShadowMobility::Movable`; precombine buckets count. NPC parts, doors, items and anything
  scripted stay CPU-culled.
- **Lamps.** The point-light faces must not take GPU-culled entities. Option A, preferred: a ~10-line vendored filter in
  `collect_gpu_culled_meshes` (`vendor/bevy_pbr/src/render/mesh.rs:2526-2540`). Point and spot shadow subviews skip
  `RenderGpuCulledEntities` when the light has a marker such as `CpuListedShadowCasters`; it carries a
  `superfargoth patch (own)` comment and a vendor README row. Option B, no patch: statics on layer 1 only, camera
  and sun on {0, 1}, lamps on {0}. That touches every other camera or view (water, probes, GI, view model) and hits
  #25764, so take it only if A is refused.
- **Cache.** `sf_shadow_cache_bevy` builds each lamp's static caster list from its own `Casters` map (bounds sphere
  against light range, `lib.rs:264-290`; `touches` is there) when it captures, instead of reading Bevy's
  `CubemapVisibleEntities`. Bevy's scan then sees only CPU-culled movers. Lamps the cache does not hold (`uncached`,
  `lib.rs:552-556`) need the same list in their faces (one bounds test per caster per light, done only when a static
  changes or the light gains shadows).
- **Expected.** Main thread: the scans lose the statics' share (~14k of 31.7k mesh entities, 44 %) of ~6.1 ms
  thread time = ~2.7 ms thread time. At the measured ~1.9x parallelism that is **~1.4 ms traced PostUpdate**,
  ~1.0-1.1 ms untraced (x 10.86/14.02). The point-light spikes shrink by the same share or more. Render plus
  Extract: churn costs (~1.2 ms of deltas listed in section 1) drop by roughly half, **~0.5-0.6 ms traced**. GPU:
  cascades and main get all statics as work items, about +56k items, ~+0.1 ms on a GPU with ~3 ms of headroom
  (`gpu/timed/ms` 7.47, base:128). Frame: **~0.5-0.8 ms**, capped by the render thread, which slices 2-3 address.
- **Verify.** `--off statics` and base walk A/B, untraced, 3 runs. Gauges: `views/cascade/items` and
  `views/main/items` go up (all statics, culled on the GPU), `views/lamp_face/items` stays about the same,
  `ecs/entities` stays the same. Traced: the four scans shrink, `write_binned_instance_buffers<Shadow>` drops, and
  `check_point_light_mesh_visibility` spikes go. Unit test: a static with `NoCpuCulling` lands in a lamp's capture
  list and not in its Bevy face lists.
- **Risks.** Range culling now happens only on the GPU: confirm the range index is set for `NoCpuCulling` meshes
  (`mesh.rs:888-915`) and that statics still end at their ring and handoff edges (owner check: far-field handoff
  band, walk). Lamp shadows from statics must look unchanged (owner check: night street with lamps). A view
  spawned mid-game full-flushes all statics: watch for a hitch when a cascade count changes.

### Slice 2: render thread, stop rebuilding preprocess bind groups every frame (and count draws)

- `prepare_preprocess_bind_groups` (`vendor/bevy_pbr/src/render/gpu_preprocess.rs:2541+`) creates 186 bind groups per
  frame (base:174). Cache them per (view, phase), keyed on the `BufferId`s they bind, which is UE's cached-command
  rule. The same slice adds gauges for unbatchable and non-mesh draws per phase (`render_phase/mod.rs:1435-1527`),
  so the recording cost (+2.5 ms traced: shadow + opaque + prepass) can be assigned.
- **Expected.** ~0.4 ms traced render (0.55 delta, most of it creation), ~0.3 untraced. No visual change.
- **Verify.** `preprocess/bind_groups_created` near 0 on a still frame, the trace row drops. Risk: a stale bind
  group after a buffer grows; the key must include every buffer id.

### Slice 3: staggered (then cached) sun cascades

- Draw the far cascades every other frame and the near cascade every frame (`sf_sky_bevy` sun view and cascade
  config). A later step reuses the lamp-cache mechanism: static depth cached per cascade, movers drawn on top,
  invalidated when the sun turns past a threshold.
- **Expected.** The cascades hold most of the shadow recording (+1.13 per-view, +0.65 shared,
  +0.33 `write_binned<Shadow>`). Skipping 2 of 3 cascades on half the frames saves about a third of that, ~0.6 ms
  traced render. The GPU saves too.
- **Risk.** Visual: lag in far-cascade shadows while the camera moves fast or the sun moves. Needs an owner check
  scene (walk plus fast turn, dusk).

## 5. Does this make `--scene` unnecessary?

For CPU visibility, yes. Slice 1 gives entity statics what `GpuBatchedMesh3d` gives batched statics: no per-view CPU
work and stable bins, without the range-table and sub-draw machinery that held `--scene` to parity
(`docs/research/static-scene-perf-audit-2026-11-11.md`, "Measured after fixes"). Entity count still costs what
`--scene` removes: spawning, change-detection scans over all meshes (`Or<Changed<..>>` rows, ~0.7 ms thread time
traced) and transform and extraction tables. Keep `--scene` parked (S3a3) until slice 1 is measured, and revive it
only if those entity-count rows lead the new profile.

## Open

1. No census of the ~28.5k near-ring mesh entities (statics vs NPC parts vs doors, items and lights). The 44 % statics
   share above rests on the brief's ~14k. The `--off actors` run plus a counter split by class will fix the
   estimate.
2. The recording cost per batch set is not explained: unbatchable (skinned or morph) draws are suspected, not
   counted. If NPC parts are unbatchable, the render-thread lever is actors (character batching, or impostors for the
   566 characters at a3), not statics.
3. GPU range culling for `NoCpuCulling` meshes is read from the vendored shader, not exercised. Upstream history
   (#23115 reverted by #24252) means it must be tested on our build.
4. Not checked: which other cameras exist in play (water, probes, GI, view model). That matters for option B and for
   full-flush hitches.
