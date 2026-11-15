# Main thread PostUpdate: what the 6.2 ms are (2026-11-15)

Diagnosis only, from existing runs, trace summaries and source. No new runs. **F** = fact (file:line), **I** = inference.
Raw chrome traces are not kept (only `*.trace.txt` top lists), so the per-system wall split below is an estimate;
section 4 says how to measure it.

## 1. Breakdown

Facts:
- Untraced wall, `b2_on` x3: 6.16 / 5.83 / 5.99 (`balmora_walk@2560x1440-{1,2,3}.md` :73/:72/:73). 12 threads: 5.84
  (`thr12`). **Pipelining off: 4.69** (`nopipe/...-1.md`, 1 run; Update 2.28 -> 2.10). So ~1.4 ms is overlap with the
  render thread and ~4.7 ms is PostUpdate's own.
- Traced wall 7.16 (`tcd_trbase.trace.txt:74`) and 6.30 (`tr_on.trace.txt:82`): the schedule wall is inflated only
  ~15 % by tracing. The "10x" applies to tiny spans, not here.
- The main thread only waits: PostUpdate span self time 6.78 of 7.16 (`tcd_trbase:5`). Systems run on the pool; the
  calling thread does not run pool tasks (`bevy_ecs .../multi_threaded.rs:274` `scope_with_executor(false, ..)`, `:668`).
- Traced thread-ms per frame (`tcd_trbase`, sum over threads): camera scan 2.13 in 38 tasks (:12), sun cascades 1.27
  (:17), lamps 1.16 (:19), ranges 0.62 + 0.26 (:24, :46), mesh-asset change scan 0.36 (:31), material change scan
  0.23 (:54), `mark_dirty_trees` 0.25 (:48), big_space low precision 0.24 (:52), anim 0.20 (:61),
  `update_terrain` 0.32 (:36), `residency_pass` 0.22 (:55), `check_point_light_mesh_visibility` self 0.20 (:60).
  Total ~7.5 thread-ms; no PostUpdate system above 0.32 self. (`drive_streaming`, `hold_warmup` are in Update.)
- Untraced gauges: `bigspace/total` 0.58-0.64 median, p99 1.7-2.0 (:85); `anim/evaluate` 0.11-0.20.
- Near ring off: PostUpdate 3.42 traced (cpu audit §1); the ring's 6.1 thread-ms of scans cost +2.9 ms wall
  (`statics-cpu-cost-2026-11-12.md:22-33`), about 2x parallelism on 8 threads.

Why ~7.5 thread-ms cost ~4.7 ms wall (**I**, mechanism is F): every entity-scaled step is a short fork/join on one
serial chain, and each fork/join ramps up slowly. `async-executor` 1.14.1 pushes every task to one global queue and
wakes **one** sleeping worker per notify (`src/lib.rs:352-360`, `:708-721`); that worker moves half the queue to its
local queue (`:1006`) and wakes the next. Workers are asleep most of the time (pool 28 % busy), and a Windows thread
wake costs tens of us. This also explains why 12 threads change nothing (ledger M2).

Estimated split of the 6.2 ms (**I**, +-50 % per row):

| part | ms | scales with |
|---|---:|---|
| visibility chain: reset, ranges, camera, cascades, lamps, mark hidden | 2.5-3.0 | entities x (1 + sun + shadowed lamps updating) |
| overlap with the render thread | ~1.4 (F) | render load |
| transforms: big_space + `mark_dirty_trees` + `sync_simple_transforms` | 0.7-0.9 (0.6 F) | filter scans all, work on changed |
| change scans: mesh assets, materials (one system per material type), bounds | 0.3-0.5 | 31.7 k meshes |
| animation chain (Probe, Evaluate, Apply) | ~0.3 | characters |
| our small systems (terrain, residency, shadow budget, surface, water, clouds) + executor hops | 0.8-1.2 | system count |

Entity-scaled (59 k, all with `ViewVisibility`): `reset_view_visibility`, camera scan, `mark_newly_hidden`
(`bevy_camera .../visibility/mod.rs:731, 786, 908`). Mesh-scaled (31.7 k): cascades, each lamp scan, ranges, change
scans. View-scaled: only lamps (one full scan per shadowed lamp, `bevy_light lib.rs:594`; ~1 per frame median, spikes
3.3-4.0 ms, `tr_on.spikes.txt`). The 46 views cost the render thread, not PostUpdate.

## 2. Serial vs parallel

One chain, forced by `.after` and by `&mut ViewVisibility` (F): anim sets chained (`sf_anim_bevy/src/lib.rs:79`) ->
big_space recenter -> local origins -> high -> low precision (`vendor/big_space/src/plugin.rs:127-194`) -> shadow
budget, 3 chained (`sf_sky_bevy/src/shadow_budget.rs:216-220`) -> bounds / frusta / visibility propagate
(`visibility/mod.rs:504-521`) -> `CheckVisibility` -> cluster assign -> light frusta -> sun scan, lamp scan (both
write `ViewVisibility`, so serial) (`bevy_light lib.rs:187-232`) -> `mark_newly_hidden`. Parallel only inside each
`par_iter`. Not checked: where `check_visibility_ranges` sits exactly, the number of systems and sync points.

## 3. Fixes, ranked by expected untraced ms

| # | change | reference | ms | size | risk |
|---|---|---|---:|---|---|
| 1 | Near statics out of the per-frame scans: cull per cell / cluster bounds, entities inside are not scanned | UE HISM cluster tree and GPU instance culling; id Tech 7 cell visibility; Bevy `NoCpuCulling` (`bevy_city`) | 1.5-2.5 (includes 2-4) | L | high: `NoCpuCulling` lost twice for render-side reasons (statics ledger H1, P3) |
| 2 | One scan for camera + cascades (same entities, 4 frusta), fixed batch size | Frostbite "Culling the Battlefield" (flat arrays, one job set); UE `ParallelFor` | 0.5-0.8 | M, vendor `bevy_camera` + `bevy_light` | medium |
| 3 | Lamp casters from cached per-lamp lists, rebuilt on change, instead of a full scan per lamp | UE cached light-primitive interactions; H1's list code exists | 0.4-0.6, p99 -2..3 | M | medium |
| 4 | Range test inline in the scan, no per-frame `EntityHashMap` rebuild (`range.rs:188, 230`) | LOD picked in the visibility pass (UE) | 0.3-0.5 | M, vendor | low |
| 5 | Gate the asset / material change scans on "an asset event of that type arrived" | Bevy docs: `AssetChanged` is a full scan | 0.2-0.3 | S, vendor `bevy_pbr` | low |
| 6 | Static transforms never visited (check that statics carry big_space `Stationary`, `stationary.rs`) | UE Mobility = Static | 0.2-0.4 | S-M | low |
| 7 | Faster fork/join: upstream executor replacement (bevy_tasks own executor, `forte`; PR numbers not verified offline); meanwhile fewer tasks per scan | rayon / UE task graph: caller takes part, workers spin before sleeping | up to 1-2 | adopt, don't build | - |
| 8 | Render overlap | shrinks as render-thread encode work is cut | up to 1.4 | free | - |

Target check: main busy 9.8 needs about -2 ms to stop being the next bound; #2-#5 together give ~1.4-2.2.

## 4. Measurements to run (walk, x3, read `stage/main/post_update`)

No code:
- `BEVYRROWIND_NO_PIPELINED=1` x3 (confirms the 1.4 ms overlap; only 1 run exists).
- `BEVYRROWIND_COMPUTE_THREADS=4` and `=2`: if PostUpdate does not rise at 4, the slow fork/join model holds.
- `--off near` (floor), `--off shadows` (sun + lamp scans), `--off lights` (lamp scans only), `--off actors,walkers`
  (anim, skinned transforms), `--off terrain,surface` (our systems), `BEVYRROWIND_GPU_CULL_STATICS=1` (9.6 k statics
  out of the scans).

One small code slice, exact split: PostUpdate set gauges `stage/post/<set>`, same method as the render sets
(`sf_diag_bevy/src/stages.rs:75-123`, marker `after_ignore_deferred` + `completion_deltas`), in this order:
`AnimSystems::Apply`, `TransformSystems::Propagate`, `VisibilitySystems::{CalculateBounds, VisibilityPropagate,
CheckVisibility}`, `SimulationLightSystems::{AssignLightsToClusters, UpdateLightFrusta, CheckLightVisibility}`,
`VisibilitySystems::MarkNewlyHiddenEntitiesInvisible`, `TerrainSystems::Update`, `SurfaceSystems`,
`CloudSystems::Drive`; plus a count gauge of PostUpdate systems. Also keep one raw trace next to its summary.
