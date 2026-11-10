# `prepare_preprocess_bind_groups`: is the per-frame bind-group rebuild worth fixing?

Research only (2026-11-10, nothing built or run). No cargo command was run for this document; no agent here can
launch the game or judge rendering.

## Verdict

**Worth doing — after one fresh trace that only the owner can run, and then as a single combined slice.** The
technique is right, the upstream PR ([bevyengine/bevy#26418](https://github.com/bevyengine/bevy/pull/26418)) is
**open, mergeable and unobjected**, its hunks land on functions our tree has not touched, and we already ship the
identical pattern in our own `materialize.rs` (`vendor/bevy_pbr/src/gpu_instance_batch/materialize.rs:81`,
`:265-269`) — so the design risk is near zero. What is missing is the *size of the prize*. The 2.3 ms is a single
trace from **2026-10-19**, taken inside a **15.3-25.8 ms** frame, before the 2026-10-28 pass that cut frames by
~35 %; the "~9 % of a 10.5 ms frame" in the coordinator brief is a cross-run mash-up whose own arithmetic gives
22 %, and a sibling review of the same day claims 20 % of 11 ms from the same source number. Upstream's own issue
says flatly "We have not isolated the effect of these bind groups on frame time." A fresh number costs the owner
one `perf trace` run and **no source edit and no build slot of ours**; implementing blind costs the single most
expensive build in the repo (a vendored `bevy_pbr` edit rebuilds the whole graph) on an unknown, and its only
agent-checkable outcome is "compiles, counter dropped". Measure first, then one slice. The deliverable brief is
therefore `docs/briefs/perf-bind-group-cache.md`, written as a **measurement** brief with the implementation
design attached so the slice is read-and-go the moment the number lands.

Second, independent finding that the coordinator note gets wrong: **#26418 does not fix the thing the 2.3 ms
measured.** That span covers three separate bind-group families and #26418 caches only the third (§1). Our S0
counter lumps all three into one number, so neither the trace nor the counter can currently say which family
dominates. Splitting the counter is ~10 lines and must ride in the *same* slice, because a second vendored edit
means a second full rebuild (§5).

---

## 1. What the code does today

`superfargoth/vendor/bevy_pbr/src/render/gpu_preprocess.rs:2541` — `pub fn prepare_preprocess_bind_groups`,
a `RenderSystems::PrepareBindGroups` system registered at `:607`. Confirmed by reading `:2541-2740`.

It builds **three** unrelated families, all unconditionally, every frame:

| # | Family | Per what | Where | Cached today? |
|---|---|---|---|---|
| A | Mesh preprocessing | per view × per phase; 1-4 `BindGroup`s each | loop opens `:2590`, builder constructed `:2618`, dispatched `:2635-2673`, stored `:2687-2689` | no |
| B | Indirect-parameter building | per phase (once, if any indirect) | `:2695-2704` → `create_build_indirect_parameters_bind_groups` (`:3691`) | no |
| C | Bin unpacking + uniform allocation | per view × phase × **batch set** × indexed/non-indexed | loop `:2708-2727` → `create_bin_unpacking_bind_groups` (`:3705`) and `create_uniform_allocation_bind_groups` (`:3857`) | no |

Family C has the worst fan-out by construction — it is per *batch set*, not per phase — which matches upstream
issue #26257's allocation counts (below). **#26418 touches only family C.**

What it allocates per view, beyond the `BindGroup`s themselves: a fresh `TypeIdHashMap` at `:2591`
(`let mut bind_groups = TypeIdHashMap::default();`) and a `commands.entity(view_entity).insert(PreprocessBindGroups(..))`
at `:2687-2689`, i.e. one component write per view per frame.

### Why it cannot be cached today

Nothing in the data model forbids it; the code simply never tried. Two concrete obstacles, both real:

1. **Our own `#26095` patch deliberately throws family C away at the top of the function** —
   `:2566-2569`:
   ```rust
   // superfargoth patch (bevyengine/bevy#26095)
   // Clear out old bind groups, since they may refer to stale metadata.
   bin_unpacking_bind_groups.clear();
   uniform_allocation_bind_groups.clear();
   ```
   This is correct as written: the bind group hard-binds a *byte offset* into the metadata buffer, so a bind group
   kept across a frame in which the job's metadata index changed would read another job's metadata. Caching
   therefore cannot be bolted on without removing that offset from the bind group — which is exactly the second
   half of #26418 (§3).

2. **The metadata offset is baked into the binding.** `create_bin_unpacking_bind_group` (`:3805`) binds
   `offset: job.bin_unpacking_metadata_index.uniform_offset() as u64` (`:3828`), and
   `create_uniform_allocation_bind_group` (`:3940`) the same at `:3969`. That offset changes whenever the job's
   slot in the per-frame metadata vector changes, i.e. most frames. **This, not buffer identity, is what actually
   changes between frames** for family C — the buffers themselves are stable.

### How ours differs from upstream

The structs are upstream's, unchanged: `BinUnpackingBindGroups` (`:494`), `ViewPhaseBinUnpackingBindGroups`
(`:500`), `ViewPhaseBinUnpackingBindGroup` (`:510`), `UniformAllocationBindGroups` (`:528`),
`ViewPhaseUniformAllocationBindGroups` (`:534`), `ViewPhaseUniformAllocationBindGroup` (`:544`).

Our divergence inside the function is seven marked sites, from `grep -n "superfargoth patch"`:

| Line | Marker | Effect |
|---|---|---|
| `:2543` | `(own: skip view preprocessing)` | `views` query gains `Without<SkipRootViewRendering>` (#26161 fork) |
| `:2558` | `(own: preprocess bind group count)` | counter paired into a tuple parameter (16-param limit) |
| `:2566` | `(bevyengine/bevy#26095)` | the two `.clear()` calls above |
| `:2648`, `:2665` | `(bevyengine/bevy#25978)` | `..` rest patterns on `PreprocessWorkItemBuffers::Indirect` (new `ranges_*` fields) |
| `:2680`, `:2695`, `:2730` | `(own: preprocess bind group count)` | S0 counter accumulation |

**The functions #26418 rewrites are pristine in our tree.** `sed -n '3705,3940p' … | grep "superfargoth patch"`
returns nothing: `create_bin_unpacking_bind_groups`, `create_bin_unpacking_bind_group`,
`create_uniform_allocation_bind_groups` and `create_uniform_allocation_bind_group` carry no patch marker.
`bin_unpacking_bind_group_layout_entries` (`:2068`) and `uniform_allocation_bind_group_layout_entries` (`:2093`)
are likewise clean — the `#25978` marker at `:2112` belongs to the *following* function,
`range_unpacking_bind_group_layout_entries`.

So the GS patch did land in this file, but **not in #26418's blast radius**: patch GS added
`prepare_range_unpacking_bind_groups` (`:898`, a *fourth*, separate family — see §5) and
`dispatch_range_unpacking` (`:995`), and added the two rest patterns above. Conflict surface is one function.

---

## 2. Is the 2.3 ms real, now? No — it is stale, and the percentage attached to it is wrong

### The one primary measurement

`bevyrrowind/docs/archive/perf-log-2026-10.md:286-294`, under heading `### Render-thread baseline, A/B and trace
(phase1, 2026-10-19)` (`:269`):

> Its biggest spans: `prepare_preprocess_bind_groups` 2.3 ms, `queue_submit` 2.4 ms (33 submits), shadow
> `write_binned_instance_buffers` 1.4 ms + shadow pass encoding 1.2 ms, skins 1.2 ms.

Conditions, same block (`:271-274`):

> Measured 2026-10-19 (phase1, `target/perf/p1-dlssq`, profiling `dlss` build, 2 runs each, quiet machine,
> GPU 2.58 GHz, 65 °C) … at 44-52 k meshes and 65-88 k entities.

- **Date**: 2026-10-19 — 22 days old.
- **Machine**: not named in the block. `bevyrrowind/docs/hard-problems.md:22` gives the project's one bench
  machine as "High, 1440p + DLSS Quality, **RTX 4060 Laptop**" — *unverified* that this trace is that machine,
  though the `p1-dlssq` tag and the suite default (`bevyrrowind/docs/perf.md:26-27`,
  `--resolution 2560x1440 --env BEVYRROWIND_DLSS=quality`) make it near certain.
- **Preset/resolution**: *unverified*; inferred High, 2560×1440 + DLSS Quality from the `dlssq` tag.
- **Scene**: **ambiguous.** The A/B table in the same section is Balmora standing at 15.3 ms frame / 13.7 ms
  render (`:276-279`); the `balmora_walk` block at `:296` is a separate section at 25.8-26.0 ms. The trace
  paragraph names no scenario. *Unverified which.*
- **View count**: not in the trace line. Same section `:290-291`: "visibility 2-4 ms (×16 views)" and "fewer
  views (check what the 16 visibility passes are)" → **~16 views**, which is also what
  `superfargoth/docs/research/perf/README.md:54-59` assumes.

### Everything else is a re-quote, and the re-quotes disagree

`superfargoth/docs/research/perf/README.md:54-59` is a *source reading*, not a measurement (its own header at
`:3`: "Research only (2026-10-19, nothing built or run)"), and its line citation
(`gpu_preprocess.rs:2188`) predates the vendor patches — the view loop is now at `:2590`. Item 8 at `:39` sizes
the win as "render −1.5 ms of the 2.3 ms **(guess)**" — marked a guess in the source.

Four documents carry four incompatible readings of the same 2.3 ms:

| Source | Claim |
|---|---|
| `docs/briefs/static-scene-coordinator-2026-11-09.md:289-291` (and `s3b-coordinator.md:22`) | "2.3 ms in an old trace, **~9 %** of a 10.5 ms frame" |
| `superfargoth/docs/research/static-scene-architecture-review-2026-11-09.md:139-144`, `:387` | "**20 %** of the 11 ms frame"; "depend on bevy**#26257**" |
| `superfargoth/docs/specs/static-scene.md:362` | "≤ 1 ms (estimate; today several ms, `prepare_preprocess_bind_groups` alone 2.3 ms)" |
| `bevyrrowind/docs/test-pass-2026-11-10.md:73` | "2.3 ms **today**" — cites no new run |

The arithmetic settles it: **2.3 / 10.5 = 22 %, not 9 %.** 2.3 / 25.8 = 8.9 % — so "~9 %" is the ratio against
the *2026-10-19 walk frame* pasted next to the *2026-10-28 walk median*. The 10.5 ms is from a different run on a
different day: `bevyrrowind/docs/perf.md:32` gives `balmora_walk 10.5`, measured per `:41` "2026-10-28
(`slice4b_final` + `slice4c_flight`, 2560x1440 + `BEVYRROWIND_DLSS=quality`, ×3; down from 2026-10-19's
`14.3 / 17.7 / 17.2 / — / 19.3 ms`)". **The frame shrank ~35 % between the trace and that median, and no trace
was re-taken.** If the 2.3 ms itself did not shrink, it is now a *larger* share than any of these numbers; if the
same pass shrank it too, it may be nearly gone. Nothing on record distinguishes those.

Only the S3b brief states this honestly — `docs/briefs/static-scene-s3b.md:94-96`: "2.3 ms in an **old** trace …
**Measure with S0's `preprocess/bind_groups` first.**"

### What would have changed it since 2026-10-19

- **The GS patch** added a *fourth* uncached family (`prepare_range_unpacking_bind_groups`, `:898`) — but in a
  *separate system*, so it does **not** inflate this span. It does add work to the same frame.
- **The shadow fork** (#26149/#26161) reduces the view count reaching this system: the `views` query at `:2543`
  now excludes `SkipRootViewRendering`, so a skipped shadow view builds nothing. Family A and C both scale with
  view count, so the 2.3 ms should have fallen — by how much, unknown.
- **The 2026-10-28 optimisation pass** cut `bigspace/total`, `anim/evaluate_cpu`, `controller/move` and others;
  those are main-thread, so they should not have moved this render-thread span, but the frame they are a share of
  moved a lot.

### The S0 instrumentation: a counter exists, a millisecond does not

A `preprocess/bind_groups` gauge **does exist and is reported.** Producer:
`vendor/bevy_pbr/src/render/gpu_preprocess.rs:355` `pub struct PreprocessBindGroupCount { created, ranges,
materialize }`, `init_resource` at `:591`. Read in exactly one place —
`superfargoth/crates/sf_diag_bevy/src/views.rs:194` (`Option<Res<PreprocessBindGroupCount>>` in `count_views`,
copied at `:201-205`) — and printed by `superfargoth/crates/sf_diag/src/views.rs:311-321`:

```rust
if let Some(b) = self.bind_groups {
    for (name, v) in [
        ("preprocess/bind_groups", b.total()),
        ("preprocess/bind_groups_created", b.created),
        ("preprocess/bind_groups_ranges", b.ranges),
        ("preprocess/bind_groups_materialize", b.materialize),
    ] {
        counters.set_named(name, unit::COUNT, f(u64::from(v)));
    }
}
```

**But there is no millisecond anywhere for this system, and there cannot be one from S0.** Confirmed three ways:

1. No ms counter for it exists in our crates. `sf_diag`'s ms sources are frame/stage counters,
   `render/<span>/elapsed_gpu|_cpu` from Bevy's `RenderDiagnosticsPlugin`
   (`superfargoth/crates/sf_diag/src/markdown.rs:8-11`), the Nsight CSV (`sf_diag/src/nsys.rs`), and the offline
   Chrome-trace analyser (`sf_diag/src/trace.rs`).
2. `RenderDiagnosticsPlugin` **structurally cannot** time it: it adds spans around and inside the `RenderGraph`
   schedule only (`vendor/bevy_render/src/diagnostic/mod.rs:77-92`), and
   `prepare_preprocess_bind_groups` is a `PrepareBindGroups` system, not a graph pass. Every one of our own
   `pass_span` call sites is a GPU pass (`sf_atmos_bevy/src/render.rs:1010`, `sf_water_bevy/src/render.rs:476`,
   `sf_shadow_cache_bevy/src/render.rs:380`, …) — none in the preprocess prepare path.
3. `SystemInformationDiagnosticsPlugin` gives only `process/cpu_usage` / `system/cpu_usage`
   (`bevyrrowind/apps/bevyrrowind/src/perf/cli.rs:368-369`) — process-level.

So S0 answers "how many bind groups", never "how many milliseconds". **The Chrome trace is the only ms path, and
it is the same tool that produced the 2.3 ms.** Exact commands are in §2 of the brief.

Worse: the S0 baseline itself has never been run. `bevyrrowind/docs/perf.md:109-110`: "#### S0 baseline results —
**Awaiting the owner's run** (ROADMAP 1.16, S0). Paste the table here."

And the counter is a *sum*: `created` lumps families A + B + C (`:2680`, `:2695`, `:2730`), so even once the S0
run lands, **the count cannot attribute cost to the family #26418 fixes.** `total()` then adds `ranges` and
`materialize` on top (`sf_diag/src/views.rs:193-199`).

**Plain answer: the number is stale and must be re-measured, and the existing counter is too coarse to
substitute for the trace.** This is the main result of this research.

---

## 3. Upstream

### bevyengine/bevy#26418 — "Cache scene-unpacking bind groups", **OPEN**

<https://github.com/bevyengine/bevy/pull/26418>. Verbatim from
<https://api.github.com/repos/bevyengine/bevy/pulls/26418>:

```
"state":"open"  "draft":false  "merged":false  "merged_at":null  "closed_at":null
"mergeable":true  "rebaseable":false  "mergeable_state":"clean"
"comments":4  "review_comments":4  "commits":5
"additions":366  "deletions":165  "changed_files":3
```

- Author `alken1t15` (`author_association: NONE`, new contributor). Opened **2026-11-03**, last activity
  **2026-11-07**. Base `bevyengine:main` (sha `526f9e40`), head `fix/cache-scene-unpacking-bind-groups`
  (`6d46e7df`).
- Labels `C-Bug`, `A-Rendering`, `S-Needs-Review`; project "Rendering", status "Needs SME Triage".
  **`"milestone":null`** — no version target. The linked issue benchmarks against 0.20.0, so it lands on
  post-0.20 `main`; which release it ships in is *unverified*.
- Review: one reviewer, `highway900` (2026-11-06), two comments, **both documentation nits** ("Comments for the
  new cache members"), addressed in commit `2a1ab03`; re-review requested and still awaited. **No objections, no
  requested changes, and nobody raised a correctness or invalidation concern.** `alice-i-cecile`: "This is over
  my head in terms of rendering PRs, sorry!"
- **Status contrast with our shadow fork**: unlike #26149/#26161 — self-closed unmerged on 2026-10-30, branches
  deleted, zero reviews (`superfargoth/vendor/README.md`, "The Bevy shadow patch — own fork") — #26418 is live,
  clean-merging and reviewed. Planning around it is legitimate under the vendored-patch rule, *provided* the
  slice is written so it survives the PR never merging (it is 366 lines of self-contained code; it would).

#### What it changes

New private type in `gpu_preprocess.rs`:

```rust
struct SceneUnpackingBindGroupCache<const N: usize> {
    entries: HashMap<[BufferId; N], (BindGroup, u8)>,
}
```

Key = the `BufferId` of **every** buffer the group binds; value = the `BindGroup` plus a `u8` age.
`begin_frame` ages every entry (`saturating_add(1)`), `get_or_create(buffers, || …)` resets age to 0 and clones,
`finish_frame` evicts with `entries.retain(|_, (_, age)| *age < 2)`. Instances live as new
`indexed_cache`/`non_indexed_cache` fields: `SceneUnpackingBindGroupCache<5>` on
`ViewPhaseBinUnpackingBindGroups`, `<4>` on `ViewPhaseUniformAllocationBindGroups`.

The half that makes caching *sound*, and the half the coordinator note omits: **binding 0 becomes a dynamic
offset.** `uniform_buffer::<GpuBinUnpackingMetadata>(false)` → `(true)`, same for
`GpuUniformAllocationMetadata`; the `BufferBinding` becomes `offset: 0`; and the per-job offset moves to dispatch
time — `set_bind_group(0, …, &[])` → `&[metadata_index.uniform_offset()]` in both `allocate_uniforms` and
`unpack_bins`. Without this, the baked offset (§1 obstacle 2) makes the cache wrong, which is precisely the bug
our `#26095` patch defends against by clearing.

It also adds `#[cfg(test)] mod scene_unpacking_cache_tests` (helpers `device()`, `exercise_cache::<N>()`; tests
`bin_unpacking_cache_reuses_invalidates_and_evicts`, `uniform_allocation_cache_reuses_invalidates_and_evicts`,
`metadata_bindings_use_dynamic_offsets`) and a dev-dependency `wgpu = { version = "30", default-features = false,
features = ["noop"] }` in `crates/bevy_pbr/Cargo.toml`.

#### Does it apply to our tree?

**Yes, with one hand-merge.**

- Our tree **has** the scene-unpacking architecture the PR targets — `BinUnpackingBindGroups` (`:494`),
  `SceneUnpackingBuffersKey`, `unpack_bins.wesl`, `allocate_uniforms.wesl` all present. The structs are
  upstream's verbatim, minus the new cache fields.
- The four `create_*` functions and both layout-entry functions are **pristine** (§1) → those hunks apply by
  context.
- `prepare_preprocess_bind_groups` **will conflict**: the PR replaces the top (clear → `begin_frame`) and the
  bottom (→ `retain` + `finish_frame`), which is exactly where our `#26095` clear and our S0 counter sit. Seven
  marker sites to preserve (§1 table). One function, hand-merged.
- Its third file, `bevy_render/src/batching/gpu_preprocessing.rs` → `clear_scene_unpacking_buffers`: **already
  applied here.** Our `#26095` patch at `:3418-3424` already clears `indexed_unpacking_jobs` /
  `non_indexed_unpacking_jobs` for every view. That hunk is a no-op for us — a genuine head start, since the PR
  author calls that clearing a prerequisite ("without it they can use another job's metadata, and dynamic
  offsets do not solve this problem").
- **Conflict with the GS patch: none in the touched region.** Patch GS's additions to this file are a different
  system (`:898`) and two rest patterns (`:2648`, `:2665`) outside #26418's hunks.

#### The 256-byte precondition holds

Dynamic offsets must satisfy `Limits::min_uniform_buffer_offset_alignment` (typically 256) —
<https://docs.rs/wgpu/latest/wgpu/struct.RenderPass.html#method.set_bind_group>: "These offsets have to be
aligned to `Limits::min_uniform_buffer_offset_alignment` or `Limits::min_storage_buffer_offset_alignment`
appropriately." Verified in our tree:

- `GpuBinUnpackingMetadata` (`vendor/bevy_render/src/batching/gpu_preprocessing.rs:1190`): 3 × `u32` +
  `pad: [u32; 61]` = 64 × 4 = **256 B**.
- `GpuUniformAllocationMetadata` (`:1245`): 4 × `u32` + `pad: [u32; 60]` = **256 B**.
- `uniform_offset()` is `self.get() * size_of::<…>()` (`:1533-1535`, `:1547-1549`) = index × 256 → always a
  legal dynamic offset.

So #26418's assertion `size_of::<…>() % 256 == 0` passes here unchanged. (The literal 256 is from
`wgpu::Limits`, which I did not fetch — *unverified* as a number, but the PR's own test asserts the modulus, not
the constant, so the slice does not depend on it.)

### Linked issue #26257 — still open, and it is the one our own docs cite

<https://github.com/bevyengine/bevy/issues/26257>, "gpu_preprocess recreates the bin-unpacking and
uniform-allocation bind groups every frame", opened by `MinimalCpp` 2026-10-20; labels `C-Bug`, `A-Rendering`,
`C-Performance`, `S-Ready-For-Implementation`. GitHub lists it under "Successfully merging this pull request may
close these issues."

Its measurement, with a counting global allocator on 0.20.0 release, **17 views**, nine materials:
**435 heap allocations/frame on Vulkan, 1334 on DX12** attributed to `create_bind_group` — split
`create_bin_unpacking_bind_group` 240/754 and `create_uniform_allocation_bind_group` 195/580. DX12 is ~3× worse
because `StandardMaterial` is not bindless there (#26182).

**And, decisively for our verdict, verbatim: "We have not isolated the effect of these bind groups on frame
time."** Upstream has the allocation count and not the millisecond, exactly as we do.

This also resolves a repo inconsistency: our docs cite **both** numbers for the same work —
`vendor/README.md` ("or stops rebuilding them every frame, bevy #26257"),
`static-scene-architecture-review-2026-11-09.md:143`, `:387` and
`superfargoth/docs/research/shadows/2026-10-21-audit.md:27` say #26257; the briefs say #26418. Both are right:
**#26257 is the issue, #26418 is the PR that fixes it.** §8 lists the lines to correct.

### The canonical technique: cache descriptor sets on resource identity, vary per-draw with dynamic offsets

This is not a quick fix; it is the textbook answer, and #26418 implements both halves of it.

- **Khronos Vulkan Samples, "Descriptor and buffer management"** (Arm-authored) —
  <https://docs.vulkan.org/samples/latest/samples/performance/descriptor_management/README.html>. Verbatim:
  "The most straightforward and flexible approach is to re-create them for each frame, but doing so might be very
  inefficient"; "A major way to reduce descriptor set updates is to re-use them as much as possible"; "**The
  cache could be a hashmap with the contents of the descriptor set (images, buffers) as key.** This approach is
  used in our framework by default." And for the per-draw variation: "each object will record its data and will
  receive a **dynamic offset** to be used at `vkCmdBindDescriptorSets()` time", so "more objects can share the
  same one: they will all reference the same `VkBuffer`, but at different dynamic offsets." Do-list: "Consider
  caching your descriptor sets when feasible"; "Consider using a single (or few) `VkBuffer` per frame with
  dynamic offsets." Don't-list: "Allocate, free or update descriptor sets every frame, unless it is necessary."
  Its own measured effect on a 2019 high-end phone: 44 ms → 27 ms, "a 38% decrease in frame time".
  **#26418 is precisely this pattern, hashmap-on-bound-buffers plus dynamic offset.**
- **NVIDIA, "Advanced API Performance: Descriptors"** —
  <https://developer.nvidia.com/blog/advanced-api-performance-descriptors/>. Recommended: "Cache descriptors on
  GPU-visible descriptor heaps (DirectX 12) or sets (Vulkan) with a known offset"; "Use dynamic uniform and
  storage buffers for per-draw call changes." Not recommended: "Keep descriptors around persistently instead of
  re-allocating or copying them in each frame"; "Prevent the excessive creation or copying of descriptors during
  the frame." Same advice in "Vulkan Dos and Don'ts"
  (<https://developer.nvidia.com/blog/vulkan-dos-donts/>): "Use dynamic uniform/storage buffers for per draw
  call changes."
- **Kapoulkine, "Writing an efficient Vulkan renderer"** —
  <https://zeux.io/2020/02/27/writing-an-efficient-vulkan-renderer/>: "Don't allocate or update descriptor sets
  if nothing in the set changed"; "prefer dynamic uniform buffers to updating uniform buffer descriptors";
  "Dynamic uniform buffers allow to specify offsets into buffer objects using `pDynamicOffsets` … **without
  allocating and updating new descriptors**"; "it may make sense to cache descriptor sets based on the
  descriptors they contain if they can be reused later in the frame."
- **Unity's SRP Batcher** is the same idea at engine level —
  <https://docs.unity.com/en-us/engine/current/manual/analysis/graphics-performance-profiling/in-urp/reduce-draw-calls-urp/srpbatcher/srpbatcher>:
  "makes material data persist in GPU memory"; "All materials have persistent constant buffers located in GPU
  memory, which are ready to use"; "**If the material content doesn't change, the SRP Batcher doesn't make any
  render-state changes**"; "Dedicated code manages a large per-object GPU constant buffer for all per-object
  properties." The offset binding is the API half:
  `CommandBuffer.SetGlobalConstantBuffer(GraphicsBuffer, int nameID, int offset, int size)`
  (<https://docs.unity.com/en-us/engine/current/script-reference/unityengine/rendering/commandbuffer/setglobalconstantbuffer>).
  *Unverified*: the SRP Batcher page never writes the word "offset", so "persistent CBuffer + bind by offset" is
  assembled from two Unity pages, not stated on one.
- **Unreal Engine: unverified.** Epic's docs pages for `FVulkanDescriptorSetCache` / `FD3D12DescriptorCache`
  return an empty JS shell (HTTP 200, no body) and `github.com/EpicGames/UnrealEngine` requires EULA linkage
  (404 unauthenticated). The only citable trace is the Khronos sample's own "Further resources", which lists
  verbatim 'The "DescriptorSet cache" section from [Bringing Fortnite to Mobile with Vulkan and OpenGL ES — GDC
  2019]' (<https://youtu.be/XCUfk5vRblo?t=2057>) — i.e. Epic ships a descriptor-set cache on mobile Vulkan, but
  I could not verify its mechanism. **Do not cite UE as precedent in the brief.**

### Does wgpu make a cache unnecessary or cheap? No.

- `wgpu::BindGroup` (<https://docs.rs/wgpu/latest/wgpu/struct.BindGroup.html>) is "Handle to a binding group …
  the set of resources bound to the bindings described by a `BindGroupLayout`"; all fields private, no mutating
  method. So it is de-facto immutable and safe to keep. *Unverified*: the page says nothing about whether it
  keeps its buffers alive by strong reference. **This matters** — see §4.
- `Device::create_bind_group` (<https://docs.rs/wgpu/latest/wgpu/struct.Device.html#method.create_bind_group>)
  is documented as exactly "Creates a new `BindGroup`." **There is no documented memoization**: identical
  descriptors do *not* return the same object, and nothing says what it costs. So "wgpu already caches this" is
  false, and the measured cost is whatever #26257's allocator counts show (435-1334 allocations/frame).
- **Layout** reuse is already in place and is not the problem: both creators fetch the layout from
  `pipeline_cache.get_bind_group_layout(...)` (`:3821`, `:3955`), so layouts are shared already. Only the groups
  are rebuilt.

---

## 4. The design, if it is worth doing

### The key

`BufferId`, and it is **provably safe to key on**. `vendor/bevy_render/src/render_resource/buffer.rs:5`
`define_atomic_id!(BufferId)`; `Buffer::id()` at `:20`. The macro
(`~/.cargo/registry/src/index.crates.io-*/bevy_utils-0.20.0/src/atomic_id.rs`) is a
`static COUNTER: AtomicU32`, `fetch_add(1, Relaxed)`, wrapped in `NonZero<u32>`, with a hard panic on exhaustion
and **no free list**:

> "Defines a 32-bit id type which guarantees global uniqueness via atomics on a static global."
> "By only implementing `new()`, we indicate that each atomic ID created will be unique."

**No id is ever reused after a buffer is dropped, so the cache has no ABA hazard** — the classic failure mode of
pointer- or slot-keyed caches. It derives `Hash + Eq + Ord`, so `[BufferId; N]` is a hashmap key with no work.
There is no generation counter and none is needed.

What is *not* a safe key: the metadata **offset**. It changes per frame (§1), and #26418's answer is to remove it
from the key *and from the bind group*, moving it to `set_bind_group`'s dynamic offsets. The cache key must
therefore cover **every buffer bound and nothing else** — which is why #26418's own doc comment excludes metadata
offsets and dispatch counts, "both can change without reallocating a buffer."

### We already run this exact design, in our own tree

`vendor/bevy_pbr/src/gpu_instance_batch/materialize.rs` — patch GS, already written, already compiling:

```rust
bound_buffers: Option<[BufferId; 4]>,                        // :81
…
let buffers = [output.id(), input.id(), culling.id(), ranges.id()];   // :265
if job.bound_buffers == Some(buffers) { continue; }                  // :266
job.bound_buffers = Some(buffers);                                   // :268
```

and the counter doc at `gpu_preprocess.rs:366-369` records the result: "**cached** on the buffers it binds, so
normally 0 after a batch's first frame and non-zero again when its output buffer is resized."

This is the single strongest argument for the slice: the technique is not new here, it is ours, and the
implementer can copy a working in-repo pattern rather than invent one. **Reuse ladder**: Bevy as-is does not
cache (§3, `create_bind_group` has no memoization) → the crate layer offers nothing (wgpu has no cache) →
a vendored patch carrying #26418, with our own `materialize.rs` as the in-tree reference.

### Where invalidation must happen

The task note is right that buffer reallocation is the hazard, and names the right one. Three sites:

1. **A batch's output buffer replaced on capacity change** (the GS port). `materialize.rs:265` already handles
   it — `output.id()` is in the key, so the resize produces a new id and the next frame rebuilds. **The new
   family-C cache must see the same class of event**, which it does if and only if the key names every buffer:
   for bin unpacking (N=5) the metadata buffer, `job.render_binned_mesh_instance_buffer`, the work-item buffer,
   `job.bin_metadata_buffer`, `job.bin_index_to_bin_metadata_index_buffer`; for uniform allocation (N=4) the
   metadata buffer, `job.bin_metadata_buffer`, the indirect-parameters metadata buffer, `job.fan_buffer`. That
   is #26418's key set exactly, and it matches the bindings our `create_*` functions build at `:3826-3846` and
   `:3965-3978`. **An entry omitted from the key is the bug**, and it is silent.
2. **`write_buffer` reallocating under you.** `materialize.rs:258-263` documents the trap for the range table:
   "`write_buffer` reserves, so the buffer's identity can change here, which is why it joins `bound_buffers`."
   The implementer must check the same for every family-C buffer: any `write_buffer` between the key read and
   the dispatch invalidates the key.
3. **A pre-existing latent fragility in our own GS cache, worth an eye while here.** `materialize.rs:265` keys
   on four buffers but binds five — `job.metadata.binding()` at `:276` is not in the key, and `job.metadata` is
   written each frame at `:250`. For a fixed-size `UniformBuffer<T>` the allocation happens once and never
   grows, so this is almost certainly safe — but it is safe by an *unstated* invariant, not by the key.
   **Unverified**; flagged because the new cache must not copy the shortcut, and because it is a one-line fix
   (add `job.metadata.buffer()?.id()` to the array) in the same slice if confirmed.

Also *unverified and load-bearing*: #26418 evicts at `age < 2` because "Phase instance and indirect parameter
buffers alternate between two allocations." I grepped our `vendor/bevy_render/src/batching/gpu_preprocessing.rs`
for `double|alternat|swap` and found **nothing**. Either the alternation lives elsewhere or our tree differs. If
our buffers do not alternate, `age < 2` is merely conservative (one wasted frame of retention, harmless); if
they alternate on a longer period, it would thrash. The implementer must confirm before copying the constant.

### Failure mode, and the assertable invariant

A stale bind group binds the **wrong buffer** or the **wrong metadata offset**. That renders silently wrong —
instances in the wrong place, a batch drawing another batch's geometry, or garbage — and **nobody on this machine
can see it.** The owner's rule applies in full: never judge rendering from screenshots, and this is a ThinkPad
with neither the game nor its data. So the design must carry a cheap invariant that a CPU-side test can assert.
Three that are exact, not eyeballs:

- **Counter invariant, the strongest.** With the cache in place, `preprocess/bind_groups_created` must fall to a
  small constant in a static scene and spike **only** on a frame where a keyed buffer id changed. That is a
  number in the S0 table, not a judgement — and it is free, the gauge already exists. It needs the counter split
  (§5) to be readable: today `created` lumps A + B + C, so a drop in C is diluted.
- **Key-completeness, assertable without a GPU.** A unit test that constructs the key array and asserts its
  length and order match the `BindGroupEntries::sequential` list — i.e. that `N` equals the binding count and
  every binding's buffer appears. #26418 ships the equivalent (`exercise_cache::<N>()`,
  `metadata_bindings_use_dynamic_offsets` asserting `has_dynamic_offset: true` and `size_of % 256 == 0`). These
  are exact assertions on data, which is our testing rule, and they run on wgpu's `noop` backend — **no GPU
  needed**, which is why the PR adds `wgpu = { features = ["noop"] }` as a dev-dependency.
- **A debug assertion at dispatch** that the dynamic offset passed to `set_bind_group` is 256-aligned and within
  the metadata buffer's written length. Cheap, `debug_assert!`, catches the offset half of the bug.

**Is this safely testable without a GPU on this machine? Partly, and the honest split matters:**
the cache *logic* (hit, invalidate on id change, evict) and the *layout* (dynamic offset, alignment) are fully
testable headless via the `noop` backend, and those tests are not vacuous — they assert exact values and would
fail if the key or the layout flag were wrong. What is **not** testable here is that the bound buffer is the
right buffer *for the shader*: that the dynamic offset at dispatch names the same job whose metadata the CPU
wrote. No headless test reaches that, because it is a GPU-visible correspondence. That half ends as a
**visual-check entry marked "awaiting owner"**, per the rendering rule. Any test that claims to cover it on this
machine would be exactly the vacuous kind the repo already has ~100 of; the brief says so explicitly.

---

## 5. Cost and sequencing

### Build slots

- **A vendored `bevy_pbr` edit is the most expensive build in this repo**: it invalidates `bevy_pbr` and
  everything downstream, which is effectively the whole graph. 20-40 min incremental on the coordinator's
  machine, per the slice brief's own figure. The slice needs **one** such slot for the implementation plus
  clippy, and realistically **one more** for the fix-up pass after the first compile — call it **1 slot, 2 if
  the hand-merge of `prepare_preprocess_bind_groups` fights back.**
- **Therefore the counter split must ride along.** Splitting `created` into per-family fields (A: mesh
  preprocess, B: indirect params, C: scene unpacking) is ~10 lines in `gpu_preprocess.rs` plus ~8 in
  `sf_diag/src/views.rs` and `sf_diag_bevy/src/views.rs` — trivial, except that *alone* it costs the same full
  rebuild. Doing it as a separate "measure better first" slice would spend the expensive slot twice. The split
  is already half-done: `:2730-2739` computes `unpacking + allocation` as a local before folding it into
  `created`.
- **Line estimate**: #26418 is +366/−165 over 3 files, of which one file is already applied here and one is a
  3-line `Cargo.toml` dev-dependency. For us: ~**250-320 lines** in `vendor/bevy_pbr/src/render/gpu_preprocess.rs`
  (cache type + 4 rewritten creators + 2 layout flags + 2 dispatch sites + the test module), ~10 lines for the
  counter split, ~16 lines across the two `sf_diag*` crates, 1 line in `vendor/bevy_pbr/Cargo.toml`, plus a
  `vendor/README.md` row and a `THIRD_PARTY_NOTICES.md` entry citing #26418 at head `6d46e7df`.
- **No `sf_*` or `mw_*` crate changes** beyond the two diag crates — this does not collide with any other lane's
  dirty files.

### Order against the `sf_scene_bevy` slices

`superfargoth/docs/specs/scene-runtime-plan.md:521-539` (§10 Steps): B1 "Create `sf_scene_bevy`: crate, manifest,
workspace member, hakari" touches **root `Cargo.toml`, `Cargo.lock`, `workspace-hack`** and costs "one cold
rebuild — bundle with B2" (`:532-533`); B3-B6 are inside `sf_scene_bevy`; C1-C2 touch
`mw_boot_bevy`/`mw_bake`/`apps/bevyrrowind`/`mw_game_core`/`mw_exterior_bevy` (`:538-539`). §10's own note at
`:628-629` schedules them as "**4 agent slices**: A1-A6 (Bevy-free, one test slot), B1-B2 (crate + seam, one cold
build), B3-B4, B5-B6+C1-C2".

**No file collision with this work at all.** But the build economics collide badly, and in a specific direction:

- B1 changes the **workspace manifest and hakari**, which is itself a near-global invalidation. A vendored
  `bevy_pbr` change is *another* near-global invalidation.
- Doing them in either order costs two full rebuilds. Doing the bind-group slice **inside the same slot as B1-B2
  cannot be done safely by two agents** — B1 edits shared files this slice must not touch.

**Recommendation: do nothing in a build slot yet. Sequence it as:**

1. **Now, owner, no build slot of ours** — one `perf trace` run to replace the 2026-10-19 number (§2 of the
   brief). This needs no source edit, runs in its own `profiling-trace` target dir, and is the only thing that
   can turn a 22-%-or-2-% question into a decision. It can be bundled with the already-pending S0 baseline run
   that `bevyrrowind/docs/perf.md:109-110` is waiting on.
2. **Then B1-B6 + C1/C2 keep the build slot**, as `scene-runtime-plan.md` §10 already schedules. They are
   committed, specified work with a dependency chain; this is a speculative optimisation with an unknown prize.
   Do not pre-empt them.
3. **Then one combined bind-group slice**, after B6 or in the gap before C1/C2 (which are bake-binary and
   wiring, cheaper), carrying: #26418's cache + dynamic offsets, the counter split, the `materialize.rs`
   metadata-id check, the headless tests, the vendor README row and notices entry, and a
   `bevyrrowind/docs/visual-checks/gpu-preprocess.md` entry marked "awaiting owner".

**Why this order and not "cheapest win first":** the phrase "cheapest independent win" is true of the *code* and
false of the *build*. The code is 300 lines copied from an open upstream PR onto a pattern we already ship; the
build is the single most expensive one available, spent on a number nobody has re-measured since the frame it was
measured in shrank 35 %. The owner's own standing rule — "Diagnose before optimizing; a stale trace is not a
measurement" — decides it. And the diagnosis is genuinely cheap: it is a run the owner was already going to make.

**If the fresh trace shows ≥ 1 ms**, promote the slice ahead of C1/C2 immediately; the design is settled and this
document is the whole research half. **If it shows < 0.3 ms**, close the item and delete the budget row in
`static-scene.md:362` rather than carry it.

---

## 6. What I could not settle

- **The current ms.** It needs the owner's workstation (`perf trace`, `profiling-trace` build). No agent here
  can produce it; no counter in the repo substitutes for it (§2).
- **Which of the three families dominates the 2.3 ms.** The trace span is the whole system; the S0 counter is a
  sum. Resolvable two ways: the counter split (needs the expensive build) or reading the per-call-site breakdown
  out of a fresh Chrome trace, which `sf_diag/src/trace.rs:257-279` prints per span name — *unverified* whether
  the `create_*` helpers appear as their own spans in that trace (they are not `#[instrument]`-ed as far as I
  read, so probably not).
- **Whether our phase-instance / indirect-parameter buffers alternate between two allocations**, which is the
  premise of #26418's `age < 2` eviction (§4). Grep found nothing; needs a read of the allocation path.
- **Whether `wgpu::BindGroup` keeps its buffers alive by strong reference.** Not documented on the rustdoc page.
  It bears on whether a cached entry can keep a dead buffer's memory resident. #26418's two-frame eviction
  bounds any leak to two frames, so this is a footnote, not a blocker.
- **Unreal's mechanism.** Epic's doc pages serve empty bodies and the GitHub mirror needs EULA auth (§3).
- **Which Bevy release #26418 ships in.** `milestone: null`.
- **The exact machine behind the 2026-10-19 trace**, and whether it traced `balmora` or `balmora_walk` (§2).
  This is why the 9 %/20 %/22 % spread cannot be reconciled from the record.

## 7. Flagged for the owner

Nothing here reverses a standing decision. One judgement call to confirm: the coordinator logged this as "the
cheapest independent win, not started" and the owner approved taking it on. **This document does not implement
it** — it concludes the honest next step is a measurement the owner alone can run. If the intent was "implement
it this session regardless", that is a reversal of *my* reading, not of a standing decision, and the
implementation design in §3-§4 is complete enough to act on without further research.

## 8. Where existing docs are wrong (report lines — I did not edit any shared file)

| File:line | Wrong | Should say |
|---|---|---|
| `docs/briefs/static-scene-coordinator-2026-11-09.md:289-291`, `docs/briefs/s3b-coordinator.md:22` | "2.3 ms in an old trace, ~9 % of a 10.5 ms frame" | Cross-run mash-up: 2.3 ms is 2026-10-19 in a 15.3-25.8 ms frame; 10.5 ms is 2026-10-28. 2.3/10.5 = 22 %. Drop the percentage; say "needs re-tracing". |
| `superfargoth/docs/research/static-scene-architecture-review-2026-11-09.md:139-144`, `:387` | "20 % of the 11 ms frame" | Same stale numerator against a newer denominator. Also `:139` cites `hard-problems.md:24`; the claim is at `:27-29`. |
| `bevyrrowind/docs/test-pass-2026-11-10.md:73` | "`prepare_preprocess_bind_groups` (2.3 ms today)" | Not "today" — 2026-10-19, un-retraced. |
| `superfargoth/docs/specs/static-scene.md:362` | "`prepare_preprocess_bind_groups` alone 2.3 ms" | Mark the figure 2026-10-19 and pre-#26161. |
| `superfargoth/docs/research/perf/README.md:54`, `:39`, `:34` | cites `gpu_preprocess.rs:2188` | The view loop is now `:2590`; the system is `:2541`. |
| `superfargoth/vendor/README.md` (S0 row) | "helpers ~326-358, set in `prepare_preprocess_bind_groups` ~2273-2447, `create_build_indirect_parameters_bind_groups` returns its count ~3394" | Actual: struct and helpers `:355`/`:375-394`; counter writes `:2565`, `:2681`, `:2696`, `:2739`; `create_build_indirect_parameters_bind_groups` `:3691`. |
| `superfargoth/vendor/README.md` (S0 row, "Drop when") | "bevy #26257" vs the briefs' "#26418" | Both: **#26257 is the issue, #26418 the PR** (open, 2026-11-03, mergeable). Same for `superfargoth/docs/research/shadows/2026-10-21-audit.md:27`. |
| `bevyrrowind/docs/perf.md:109-110` | "S0 baseline results — Awaiting the owner's run" | Still true; this research depends on it, and on a trace run it does not yet mention. |
