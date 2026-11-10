# Brief: **measure first** — `prepare_preprocess_bind_groups`, before any bind-group cache is written

Written 2026-11-10 by the research agent for `superfargoth/docs/research/preprocess-bind-groups-2026-11-10.md`
(branch `perf/preprocess-bind-groups`). **This is a measurement brief, not an implementation brief.** The research
concluded the design is settled and derisked but the prize is unmeasured: the only number on record is a
**2026-10-19** trace, taken inside a **15.3-25.8 ms** frame, before the 2026-10-28 pass that cut frames ~35 %, and
the "~9 % of a 10.5 ms frame" in `docs/briefs/static-scene-coordinator-2026-11-09.md:289-291` is a cross-run
mash-up whose own arithmetic gives 22 %. Upstream's issue says verbatim "We have not isolated the effect of these
bind groups on frame time."

§1-§2 are **for the owner** and need no agent and no build slot. §3-§7 are the implementation slice, to be run
**only once §2 returns ≥ 1 ms** — the design is complete here so that slice is read-and-go, not re-research.

**Decision rule, up front.** From §2's `self time per frame` for `prepare_preprocess_bind_groups`:

| Fresh number | Do this |
|---|---|
| **≥ 1.0 ms** | Run the slice (§3-§7). One vendored `bevy_pbr` build slot. |
| **0.3-1.0 ms** | Owner's call. The slice is cheap in code and expensive in build; it competes with `scene-runtime-plan.md` §10 C1/C2. |
| **< 0.3 ms** | Close the item. Delete the budget row at `superfargoth/docs/specs/static-scene.md:362` and the "cheapest independent win" line in the coordinator briefs rather than carry them. |

---

## 1. Why the existing instrumentation cannot answer this (do not re-litigate)

A `preprocess/bind_groups` gauge **exists and works** — it is a **count**, and counts cannot settle a millisecond
question. Verified:

- Producer: `superfargoth/vendor/bevy_pbr/src/render/gpu_preprocess.rs:355`
  `pub struct PreprocessBindGroupCount { created, ranges, materialize }`, `init_resource` at `:591`, written at
  `:2565` / `:2681` / `:2696` / `:2739`.
- Sole reader: `superfargoth/crates/sf_diag_bevy/src/views.rs:194`
  (`bind_groups: Option<Res<PreprocessBindGroupCount>>` in `fn count_views`, copied `:201-205`).
- Printed by `superfargoth/crates/sf_diag/src/views.rs:311-321`, as built:
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
  `total()` = `created.saturating_add(ranges).saturating_add(materialize)` (`sf_diag/src/views.rs:193-199`).

**There is no ms for this system and none can be added cheaply:**

- `RenderDiagnosticsPlugin` structurally cannot reach it — it adds spans around and inside the `RenderGraph`
  schedule only (`superfargoth/vendor/bevy_render/src/diagnostic/mod.rs:77-92`), and
  `prepare_preprocess_bind_groups` (`gpu_preprocess.rs:2541`, registered `:607`) is a
  `RenderSystems::PrepareBindGroups` system, **not a graph pass**. Every one of our own `pass_span` sites is a GPU
  pass (`sf_water_bevy/src/render.rs:476`, `sf_shadow_cache_bevy/src/render.rs:380`, …).
- `SystemInformationDiagnosticsPlugin` gives only `process/cpu_usage` / `system/cpu_usage`
  (`bevyrrowind/apps/bevyrrowind/src/perf/cli.rs:368-369`).
- **The Chrome trace is the only ms path** — and it is the same tool that produced the 2.3 ms.

**And the count is too coarse even as a proxy.** `created` is the **sum of three unrelated families**
(`:2680` mesh preprocessing, `:2695` indirect-parameter building, `:2730` bin unpacking + uniform allocation), and
the upstream fix touches **only the third**. A drop in family C is diluted in `created`. §5 splits it — in the
same slice, because a second vendored edit means a second full rebuild.

---

## 2. The measurement — owner, on the workstation

Two runs. Neither needs a source edit, an agent, or the shared build slot.

### 2a. The trace (the decisive one, ~30 min)

Verbatim from `bevyrrowind/docs/perf.md:403-410`, `## CPU per system (trace)`:

```sh
cargo build --profile profiling-trace -p bevyrrowind --features trace   # own target dir
target/profiling-trace/bevyrrowind.exe perf suite trace1 balmora --trace   # TRACE_CHROME per run
target/profiling-trace/bevyrrowind.exe perf trace target/perf/trace1/<run>/trace.json   # measured window by default
target/profiling-trace/bevyrrowind.exe perf spikes <trace.json> --min-ms 3
```

> "Delete the multi-GB traces afterwards. `--features tracy` is the GUI alternative (Tracy)."

Handler: `bevyrrowind/apps/bevyrrowind/src/perf/cli.rs:208-212`. `--trace` sets `TRACE_CHROME` per run
(`cli.rs:507-508`).

**Change from the 2026-10-19 run: trace `balmora_walk` as well as `balmora`.** The original trace paragraph
(`bevyrrowind/docs/archive/perf-log-2026-10.md:286-294`) never said which scenario it traced, and the 15.3 ms
standing frame and the 25.8 ms walk frame in the same section give the 9 %/20 %/22 % spread that this brief
exists to kill. Trace both and the ambiguity never recurs:

```sh
target/profiling-trace/bevyrrowind.exe perf suite trace2 balmora balmora_walk --trace
target/profiling-trace/bevyrrowind.exe perf trace target/perf/trace2/balmora/trace.json
target/profiling-trace/bevyrrowind.exe perf trace target/perf/trace2/balmora_walk/trace.json
```

**The exact output to read.** `superfargoth/crates/sf_diag/src/trace.rs:257-279`, as built:

```rust
"{} frames in {:.1}..{:.1} s ({:.1} fps traced)"
"\n== top self time per frame (ms), calls/frame, worst call (ms)"
"{:8.3}  x{:7.1}  max {:7.2}  {}"   // self_us/frames/1e3, calls/frame, max_us/1e3, span name
```

So the line to find begins with a self-ms figure and ends with the span name
`prepare_preprocess_bind_groups`. **Record, per scenario**: that self-ms, its calls/frame, its worst call, and —
for the denominator that was missing last time — the `frames in … (… fps traced)` header line and the frame
median from the same build.

**Record the caveat too**: `profiling-trace` adds tracing overhead, so this ms is inflated against the
`profiling` bench build. *Unverified* by how much. Compare the *2026-10-19 trace* to the *new trace*, never a
trace figure to a `perf.md` frame median — that mistake is the whole reason this brief exists.

### 2b. The S0 baseline (already pending; gives the counts, costs nothing extra)

`bevyrrowind/docs/perf.md:109-110`: "#### S0 baseline results — **Awaiting the owner's run** (ROADMAP 1.16, S0).
Paste the table here." Verbatim recipe from `perf.md:95-103`:

```sh
cargo build --profile profiling -p bevyrrowind --features dlss
target/profiling/bevyrrowind.exe perf suite s0_baseline \
    balmora seyda_neen red_mountain run balmora_walk flight balmora_night balmora_walk_night \
    --repeat 3 --resolution 2560x1440 --env BEVYRROWIND_DLSS=quality --save benchmarks/runs/s0_baseline
target/profiling/bevyrrowind.exe perf suite s0_baseline_noprecombine \
    balmora seyda_neen red_mountain run balmora_walk flight balmora_night balmora_walk_night \
    --repeat 3 --resolution 2560x1440 --env BEVYRROWIND_DLSS=quality --off precombine --save benchmarks/runs/s0_baseline_noprecombine
```

No extra feature flag is needed: `sf_diag_bevy`'s `views` feature is `views = ["render", "dep:bevy_core_pipeline",
"dep:bevy_pbr"]` (`superfargoth/crates/sf_diag_bevy/Cargo.toml:14`) and `bevyrrowind` enables it unconditionally
(`bevyrrowind/crates/mw_devtools_bevy/Cargo.toml:67`,
`sf_diag_bevy = { workspace = true, features = ["assets", "views"] }`). `RenderViewsPlugin` is added at
`mw_devtools_bevy/src/perf.rs:173`, inside the `if let Some((scenario, settings)) = self.bench` block
(`perf.rs:165`) — i.e. **only under a benchmark scenario**, which `perf suite` supplies. Keep the night
scenarios: lamp cube-face views exist only there (`perf.md:90-93`, both set `--hour 22`).

**What to record.** The `summary.md` column is literally named `preprocess bind groups`
(`bevyrrowind/apps/bevyrrowind/src/perf/cli.rs:379`,
`GaugeColumn { label: "preprocess bind groups".into(), counter: "preprocess/bind_groups".into() }`), one of five
S0 columns at `cli.rs:374-379`. Per-run detail is the `## Gauges (counts, sizes)` table in `<run>.md`
(`superfargoth/crates/sf_diag/src/markdown.rs:235-246`), rows
`| preprocess/bind_groups{,_created,_ranges,_materialize} | count | median | max | last |`.

Read `created` against the view count in the same row: family A and C both scale with views, and
`gpu_preprocess.rs:2543` now excludes `SkipRootViewRendering` views (our #26161 fork), so the ~16 views behind
the 2026-10-19 figure may no longer be 16. Caveats that bend the reading are listed at `perf.md:112-139` — in
particular skipped views under-report, `cascade0` is overloaded with 3 non-cascade depth views, and the counters
are **not comparable with the pre-S0 figures in the same file**.

### 2c. Paste the result where the stale numbers live

`bevyrrowind/docs/perf.md` (S0 results section at `:109`), and strike the stale claims listed in §6 of
`superfargoth/docs/research/preprocess-bind-groups-2026-11-10.md`.

---

## 3. The upstream fix, as built — quoted, so the slice does not re-research it

[**bevyengine/bevy#26418**](https://github.com/bevyengine/bevy/pull/26418) "Cache scene-unpacking bind groups",
author `alken1t15`. From <https://api.github.com/repos/bevyengine/bevy/pulls/26418>, verbatim:

```
"state":"open"  "draft":false  "merged":false  "merged_at":null  "closed_at":null
"mergeable":true  "mergeable_state":"clean"  "additions":366  "deletions":165  "changed_files":3
```

Opened 2026-11-03, last activity 2026-11-07, base `bevyengine:main` (`526f9e40`), head `6d46e7df`, labels
`C-Bug`/`A-Rendering`/`S-Needs-Review`, **`"milestone":null`**. One review (`highway900`, 2026-11-06): two
**documentation nits** only, addressed in `2a1ab03`. **No objections and no correctness concern from anyone.**
Fixes open issue [#26257](https://github.com/bevyengine/bevy/issues/26257) (`S-Ready-For-Implementation`), which
measures **435 heap allocations/frame on Vulkan, 1334 on DX12** at 17 views from these two creators — and says
verbatim "We have not isolated the effect of these bind groups on frame time."

**Note for `vendor/README.md` and the audit docs**: our tree cites *both* numbers for this work. Both are right —
**#26257 is the issue, #26418 the PR.** Unlike #26149/#26161 (self-closed unmerged 2026-10-30, now our own fork),
**#26418 is live and clean-merging**, so this is a legitimate vendored patch carrying an upstream PR, not a fork.
Write it so it survives the PR never merging — it is self-contained and would.

### The mechanism, both halves

```rust
struct SceneUnpackingBindGroupCache<const N: usize> {
    entries: HashMap<[BufferId; N], (BindGroup, u8)>,
}
```

Key = the `BufferId` of **every** buffer the group binds; value = the `BindGroup` plus a `u8` age.
`begin_frame` ages every entry (`saturating_add(1)`); `get_or_create(buffers, || …)` resets age to 0 and clones;
`finish_frame` evicts with `entries.retain(|_, (_, age)| *age < 2)`. Instances become new
`indexed_cache`/`non_indexed_cache` fields: `SceneUnpackingBindGroupCache<5>` on
`ViewPhaseBinUnpackingBindGroups`, `<4>` on `ViewPhaseUniformAllocationBindGroups` (both gain `Default`).
The PR's own doc comment excludes metadata offsets and dispatch counts from the key because "both can change
without reallocating a buffer."

**The second half is not optional and the coordinator note omits it: binding 0 becomes a dynamic offset.**
`uniform_buffer::<GpuBinUnpackingMetadata>(false)` → `(true)` and the same for `GpuUniformAllocationMetadata`;
the `BufferBinding` becomes `offset: 0`; the per-job offset moves to dispatch —
`set_bind_group(0, …, &[])` → `&[metadata_index.uniform_offset()]` in both `allocate_uniforms` and
`unpack_bins`. Without this the baked offset makes a cached group read **another job's metadata**, which is
precisely what our `#26095` clear defends against today.

---

## 4. Our tree: exact `file:line`, and the conflict surface

All in `superfargoth/vendor/bevy_pbr/src/render/gpu_preprocess.rs` unless noted.

### The one function that conflicts

`:2541` `pub fn prepare_preprocess_bind_groups`. It builds **three** families, all every frame:

| Family | Per what | Lines | #26418? |
|---|---|---|---|
| A mesh preprocessing | view × phase, 1-4 groups each | loop `:2590`, builder `:2618`, dispatch `:2635-2673`, store `:2687-2689` | **no** |
| B indirect-parameter building | phase, once if any indirect | `:2695-2704` → `create_build_indirect_parameters_bind_groups` `:3691` | **no** |
| C bin unpacking + uniform allocation | view × phase × **batch set** × indexed/non-indexed | `:2708-2727` | **yes, only this** |

Our seven marker sites inside it, from `grep -n "superfargoth patch"` — **all must survive the merge**:

| Line | Marker | What to keep |
|---|---|---|
| `:2543` | `(own: skip view preprocessing)` | `views` query keeps `Without<SkipRootViewRendering>` |
| `:2558-2563` | `(own: preprocess bind group count)` | the counter stays in a **tuple parameter**; a system takes at most 16 parameters and this one is at the limit. As built: |

```rust
// superfargoth patch (own: preprocess bind group count): paired with the counter in one tuple
// parameter, since a system takes at most 16 parameters.
(mut uniform_allocation_bind_groups, mut bind_group_count): (
    ResMut<UniformAllocationBindGroups>,
    ResMut<PreprocessBindGroupCount>,
),
```

| `:2566-2569` | `(bevyengine/bevy#26095)` | **this is what #26418 replaces.** As built: |

```rust
// superfargoth patch (bevyengine/bevy#26095)
// Clear out old bind groups, since they may refer to stale metadata.
bin_unpacking_bind_groups.clear();
uniform_allocation_bind_groups.clear();
```

Replace the two `.clear()` calls with `begin_frame()` on all four caches — **and only because the dynamic-offset
half removes the stale-metadata hazard the comment names.** Keep the marker, retitled to cite #26418, and keep a
comment saying why the clear is no longer needed. Do **not** drop the clear without the layout change.

| `:2648`, `:2665` | `(bevyengine/bevy#25978)` | `..` rest patterns on `PreprocessWorkItemBuffers::Indirect` (patch GS added `ranges_*` fields) |
| `:2680`, `:2695`, `:2730-2739` | `(own: preprocess bind group count)` | counter accumulation; `:2730-2739` already computes `unpacking + allocation` separately — §5 uses that |

### What applies by context (verified pristine)

`sed -n '3705,3940p' … | grep "superfargoth patch"` returns **nothing**. These carry no patch marker:

- `:3705` `create_bin_unpacking_bind_groups`
- `:3805` `create_bin_unpacking_bind_group` — the baked offset is `:3828`,
  `offset: job.bin_unpacking_metadata_index.uniform_offset() as u64`; its 5 bindings are `:3826-3846`
- `:3857` `create_uniform_allocation_bind_groups`
- `:3940` `create_uniform_allocation_bind_group` — baked offset `:3969`; its 4 bindings `:3965-3978`
- `:2068` `bin_unpacking_bind_group_layout_entries() -> BindGroupLayoutEntries<5>`
- `:2093` `uniform_allocation_bind_group_layout_entries() -> BindGroupLayoutEntries<4>`

The `#25978` marker at `:2112` belongs to the **next** function (`range_unpacking_bind_group_layout_entries`), not
to either layout fn. **Patch GS is not in #26418's blast radius**: its additions here are a separate system
(`:898` `prepare_range_unpacking_bind_groups`, `:995` `dispatch_range_unpacking`) and the two rest patterns.

### The third file is already applied

#26418's `bevy_render/src/batching/gpu_preprocessing.rs` hunk clears the per-view unpacking jobs. **We already do
it**, via #26095 at `:3418-3424`:

```rust
// superfargoth patch (bevyengine/bevy#26095)
// Clear out the jobs for every view, including views that we won't see this
// frame, since they refer to the metadata above.
for view_phase_buffers in scene_unpacking_buffers.view_phase_buffers.values_mut() {
    view_phase_buffers.indexed_unpacking_jobs.clear();
    view_phase_buffers.non_indexed_unpacking_jobs.clear();
}
```

That hunk is a **no-op for us** — and a real head start, since the PR author calls this clearing a prerequisite
("without it they can use another job's metadata, and dynamic offsets do not solve this problem"). Verify it is
still there before concluding the dynamic-offset change is safe.

### The 256-byte precondition holds — verified, do not re-check

Dynamic offsets must be aligned to `Limits::min_uniform_buffer_offset_alignment`
(<https://docs.rs/wgpu/latest/wgpu/struct.RenderPass.html#method.set_bind_group>: "These offsets have to be
aligned to `Limits::min_uniform_buffer_offset_alignment` or `Limits::min_storage_buffer_offset_alignment`
appropriately"). In `superfargoth/vendor/bevy_render/src/batching/gpu_preprocessing.rs`:

- `:1190` `GpuBinUnpackingMetadata` — 3 × `u32` + `pad: [u32; 61]` = 64 × 4 = **256 B**
- `:1245` `GpuUniformAllocationMetadata` — 4 × `u32` + `pad: [u32; 60]` = **256 B**
- `:1533-1535` and `:1547-1549` `uniform_offset()` = `self.get() * size_of::<…>()` = index × 256

So every offset is already a legal dynamic offset and #26418's `size_of::<…>() % 256 == 0` assertion passes
unchanged.

### The key is provably safe — and we already ship this pattern

`superfargoth/vendor/bevy_render/src/render_resource/buffer.rs:5` `define_atomic_id!(BufferId)`, `Buffer::id()`
at `:20`. The macro (`bevy_utils-0.20.0/src/atomic_id.rs`) is a `static COUNTER: AtomicU32` with
`fetch_add(1, Relaxed)`, `NonZero<u32>`, a panic on exhaustion, and **no free list**:

> "By only implementing `new()`, we indicate that each atomic ID created will be unique."

**No id is ever reused after a buffer is dropped — no ABA hazard**, the classic failure of slot-keyed caches. It
derives `Hash + Eq + Ord`, so `[BufferId; N]` is a key for free.

**Copy our own working precedent, not the PR's prose.**
`superfargoth/vendor/bevy_pbr/src/gpu_instance_batch/materialize.rs` (patch GS), as built:

```rust
bound_buffers: Option<[BufferId; 4]>,                                 // :81
…
let buffers = [output.id(), input.id(), culling.id(), ranges.id()];   // :265
if job.bound_buffers == Some(buffers) { continue; }                  // :266
job.bound_buffers = Some(buffers);                                   // :268
```

with the `write_buffer` trap documented at `:258-263`: "`write_buffer` reserves, so the buffer's identity can
change here, which is why it joins `bound_buffers`." The result is recorded in the counter doc at
`gpu_preprocess.rs:366-369`: "**cached** on the buffers it binds, so normally 0 after a batch's first frame and
non-zero again when its output buffer is resized."

**Reuse ladder** (CONTRIBUTING rule 2): Bevy as-is does not cache — `Device::create_bind_group` is documented as
exactly "Creates a new `BindGroup`." with no memoization
(<https://docs.rs/wgpu/latest/wgpu/struct.Device.html#method.create_bind_group>). The crate layer offers nothing:
wgpu has no bind-group cache, identical descriptors do not return the same object. Layout reuse is *already*
in place and is not the problem — both creators fetch layouts from `pipeline_cache.get_bind_group_layout(...)`
(`:3821`, `:3955`). So: a vendored patch carrying #26418, with `materialize.rs` as the in-tree reference.

---

## 5. What the slice must do, in one build slot

1. **Port #26418's `gpu_preprocess.rs` change**, hand-merging `prepare_preprocess_bind_groups` per §4. Mark every
   hunk `// superfargoth patch (bevyengine/bevy#26418)`.
2. **Split the counter in the same slot.** `created` lumps A + B + C (§1). Add per-family fields to
   `PreprocessBindGroupCount` (`:355`) and gauge names in `sf_diag/src/views.rs:311-321`. `:2730-2739` already
   computes `unpacking + allocation` as a local, so the split is ~10 lines in the vendored file and ~8 across the
   two diag crates. **Doing this as a separate slice would cost a second full rebuild** — that is the whole
   reason it rides along. Keep `created` as the existing total so the pre-slice S0 table stays comparable (the
   S0 baseline's stated design, `:348-352`: "they are never summed here, so `sf_diag` can report the parts as
   well as the total and the slice S0 baseline … stays comparable").
3. **Check — and if confirmed, fix — the latent gap in our own GS cache.** `materialize.rs:265` keys on four
   buffers but binds five: `job.metadata.binding()` (`:276`) is not in the key, and `job.metadata` is written
   every frame at `:250`. For a fixed-size `UniformBuffer<T>` the allocation happens once and never grows, so
   this is *probably* safe — but by an **unstated** invariant, not by the key. **Unverified.** If confirmed a
   real hazard it is a one-line fix (add the metadata buffer's id to the array); if confirmed safe, say so in a
   comment so the next reader does not re-derive it. The new family-C cache must not copy the shortcut.
4. **Confirm the `age < 2` eviction constant before copying it.** #26418 justifies it in-code as "Phase instance
   and indirect parameter buffers alternate between two allocations." A grep of
   `vendor/bevy_render/src/batching/gpu_preprocessing.rs` for `double|alternat|swap` found **nothing** —
   **unverified for our tree.** If our buffers do not alternate, `age < 2` is merely conservative (one wasted
   frame of retention, harmless). If they alternate on a longer period it would thrash. Read the allocation path
   and record what you find in the vendor README row.
5. **Port the tests** (§6).
6. **Records**: a `superfargoth/vendor/README.md` row (cite #26418 at head `6d46e7df`, the three-file diff, and
   that our #26095 already covers its `bevy_render` hunk), a `superfargoth/THIRD_PARTY_NOTICES.md` entry, module
   docs citing source file + commit. Both files are shared — **report the lines, do not edit them**.
7. **A visual-check entry.** `bevyrrowind/docs/visual-checks/gpu-preprocess.md`, marked **"awaiting owner"**. A
   stale bind group binds the wrong buffer or the wrong metadata offset — silently wrong rendering (instances in
   the wrong place, a batch drawing another batch's geometry). **No agent on the owner's ThinkPad can see it**:
   the game and its data are not there, and judging rendering from screenshots is forbidden anyway. The entry
   must name what to look at: a static exterior at rest (no popping or displaced statics), then a walk (statics
   stable as batches resize), then a night scenario (lamp cube-face views exercise the per-view path).

---

## 6. Tests — and the one that would be vacuous

The repo already has ~100 install-gated tests that pass having asserted nothing. **No new test may join them.**

**Write these — exact assertions on logic and data, no GPU needed.** #26418 ships the equivalents
(`mod scene_unpacking_cache_tests`, helpers `device()` / `exercise_cache::<N>()`, tests
`bin_unpacking_cache_reuses_invalidates_and_evicts`, `uniform_allocation_cache_reuses_invalidates_and_evicts`,
`metadata_bindings_use_dynamic_offsets`). They run on wgpu's **`noop` backend**, which is why the PR adds
`wgpu = { version = "30", default-features = false, features = ["noop"] }` to `crates/bevy_pbr`
`[dev-dependencies]` — port that line into `superfargoth/vendor/bevy_pbr/Cargo.toml`.

1. **Cache hit / invalidate / evict.** Same key twice → one `create_bind_group`; one id changed → a new entry;
   an entry untouched for the eviction window → gone. Assert exact entry counts, not "is some".
2. **Key completeness** — the invariant that actually prevents the silent bug. Assert `N` equals the binding
   count of the matching `BindGroupLayoutEntries<N>` (`:2068` → 5, `:2093` → 4) and that the key array is built
   from the same buffers, in the same order, as the `BindGroupEntries::sequential` list at `:3826-3846` and
   `:3965-3978`. **An omitted buffer is the bug, and it is silent** — this test is the cheap assertable
   invariant that replaces an eyeball.
3. **Dynamic-offset layout.** `has_dynamic_offset: true` on binding 0 of both layouts, and
   `size_of::<GpuBinUnpackingMetadata>() % 256 == 0` / same for `GpuUniformAllocationMetadata` (both are 256 B
   today — §4 — so this is a regression guard on the padding at
   `gpu_preprocessing.rs:1190`/`:1245`).
4. **A `debug_assert!` at dispatch**, not a test: the offset passed to `set_bind_group` is 256-aligned and within
   the metadata buffer's written length.
5. **A measurement test, not a gate.** If you print a cache hit rate or an entry high-water, do it from an
   `#[ignore = "measurement: ..."]` test — never a pass/fail limit on a tuned number.

**The test that would be vacuous — do not write it.** Anything claiming to verify that *the right buffer ends up
bound for the right job on the GPU*: that the dynamic offset at dispatch names the same job whose metadata the
CPU wrote. That is a GPU-visible correspondence; headless, it can only assert that the code passed the value it
passed. A test shaped like `assert!(bind_group.is_some())` after a cache hit is the same trap. **That half of
correctness ends at §5.7's visual-check entry marked "awaiting owner", not at a green test.**

Scope honestly in the handoff: the cache *logic* and the *layout* are fully covered headless and would fail if
the key or the flag were wrong; the GPU correspondence is not covered by anything on this machine.

---

## 7. Open questions, with the recommended default

1. **Does the slice go before or after `scene-runtime-plan.md` §10 B1-B6 + C1/C2?** Default: **after.** No file
   collides — B1 touches root `Cargo.toml`, `Cargo.lock`, `workspace-hack` (`:532`), B3-B6 live in
   `sf_scene_bevy`, C1/C2 in `mw_boot_bevy`/`mw_bake`/`apps/bevyrrowind` (`:538-539`) — but the build economics
   do: B1's manifest+hakari change and a vendored `bevy_pbr` change are each a near-global invalidation, so
   either order costs two full rebuilds, and they cannot share a slot (B1 edits shared files this slice must not
   touch). B1-C2 is committed, specified work with a dependency chain; this is a speculative optimisation with an
   unmeasured prize. Promote it ahead of C1/C2 (bake-binary and wiring, cheaper) **only** if §2 returns ≥ 1 ms.
2. **Does family A (mesh preprocessing, the per-view × per-phase groups, `:2590-2689`) need caching too?**
   Default: **not in this slice.** #26418 does not touch it; its key would have to include the work-item buffers,
   which genuinely change; and the counter split (§5.2) will say whether A or C dominates — decide then, with a
   number. Note A also allocates a fresh `TypeIdHashMap` per view per frame (`:2591`) and does a component write
   per view (`:2687-2689`); both are cheap-looking and unmeasured.
3. **Family D: `prepare_range_unpacking_bind_groups` (`:898`, patch GS) is also uncached** — its counter doc at
   `:360-364` says so: "Rebuilt every frame like `Self::created`." It is **our own code**, so it needs no
   upstream anything, and `materialize.rs`'s pattern drops straight in. Default: **include it** if the trace
   shows it; it is in the same file and the same build slot, so it is nearly free once you are there. Its
   `ranges.metadata.write_buffer` at `:951` is exactly the identity-change trap of `materialize.rs:258-263` —
   the metadata buffer's id must be in the key.
4. **Carry #26418 or wait for the merge?** Default: **carry it**, as a vendored patch citing the open PR —
   `mergeable_state: clean`, reviewed, no objections, but `milestone: null`, so no release is predictable, and
   #26149/#26161 show what waiting costs here. Write it to survive the PR never merging.
5. **Does `wgpu::BindGroup` keep its buffers alive by strong reference?** Not documented
   (<https://docs.rs/wgpu/latest/wgpu/struct.BindGroup.html> says only "Handle to a binding group"; all fields
   private, no mutating method, so it is de-facto immutable and safe to keep). Default: **ignore it** — the
   two-frame eviction bounds any retention to two frames. A footnote, not a blocker.

---

## 8. Build economy

- **Every** cargo command goes through `bash bevyrrowind/tools/cargo_turn.sh <args>` from the repo root, one
  build at a time. It **always exits 0** — it loses cargo's exit code. **Judge every run by its output, never by
  `$?`**: `^error`, `could not compile`, `test result:`, `warning:`, `Finished`. Redirect to a file and grep it;
  a backgrounded tool call still dies at its `timeout`.
- **Never `--workspace`.** Always `-p <crate>`. Always `--all-targets` on clippy.
- Workspace lints already set `clippy::pedantic` to warn, so `-D warnings` denies it. **Do not pass
  `-W clippy::pedantic`** — it re-enables three lints the owner set to allow.
- The repo is **not** `cargo fmt`-clean and fmt is not in the gate: match the surrounding style (~120 columns),
  **never run `cargo fmt`**.
- **This is the expensive one.** A vendored `bevy_pbr` edit invalidates `bevy_pbr` and everything downstream —
  effectively the whole graph: **20-40 min incremental** on this machine. Budget **one** slot for the port and
  realistically **a second** for the fix-up after the first compile. `cargo_turn.sh` also waits for **any**
  cargo/rustc on the machine, so expect queueing behind other lanes.
- **Consequence for the plan**: everything that touches vendored `bevy_pbr` goes in **one** slice — the port, the
  counter split, the `materialize.rs` check, and (if taken) question 3. Two slices = two full rebuilds.
- The gate, after the port:
  ```sh
  bash bevyrrowind/tools/cargo_turn.sh clippy -p bevy_pbr --all-targets -- -D warnings
  bash bevyrrowind/tools/cargo_turn.sh test   -p bevy_pbr --all-targets
  bash bevyrrowind/tools/cargo_turn.sh clippy -p sf_diag -p sf_diag_bevy -p mw_devtools_bevy \
      --all-targets --features sf_diag_bevy/views -- -D warnings
  ```
  (the third line is the S0 instrumentation's own gate, from
  `bevyrrowind/docs/test-pass-handoffs/s0-instrument.md`). Record the before/after counts for each.
- §2's runs cost **no** agent build slot: 2a builds into its own `profiling-trace` target dir on the owner's
  machine, 2b is the already-pending S0 baseline.

## 9. Where existing docs are wrong

Full table in `superfargoth/docs/research/preprocess-bind-groups-2026-11-10.md` §8. The ones that would mislead
this slice:

| File:line | Wrong | Actually |
|---|---|---|
| `docs/briefs/static-scene-coordinator-2026-11-09.md:289-291`, `docs/briefs/s3b-coordinator.md:22` | "2.3 ms in an old trace, ~9 % of a 10.5 ms frame" | Cross-run mash-up: 2.3 ms is 2026-10-19 in a 15.3-25.8 ms frame; 10.5 ms is 2026-10-28. 2.3/10.5 = 22 %. |
| `superfargoth/docs/research/static-scene-architecture-review-2026-11-09.md:139-144`, `:387` | "20 % of the 11 ms frame" | Same stale numerator, newer denominator. `:139` cites `hard-problems.md:24`; the claim is at `:27-29`. |
| `bevyrrowind/docs/test-pass-2026-11-10.md:73` | "2.3 ms today" | 2026-10-19, un-retraced. |
| `superfargoth/docs/research/perf/README.md:54`, `:39`, `:34` | cites `gpu_preprocess.rs:2188` for the view loop; sizes the win "−1.5 ms of the 2.3 ms **(guess)**" | The loop is `:2590`, the system `:2541`. The saving is marked a guess in its own source — treat it as one. |
| `superfargoth/vendor/README.md` (S0 row) | "helpers ~326-358 … ~2273-2447 … ~3394"; "Drop when … bevy #26257" | Actual: `:355`, `:375-394`; writes `:2565`/`:2681`/`:2696`/`:2739`; `create_build_indirect_parameters_bind_groups` `:3691`. **#26257 is the issue, #26418 the PR.** |
| The coordinator framing "the cheapest independent win" | — | True of the **code** (~300 lines off an open PR onto a pattern we already ship), false of the **build** (the most expensive slot in the repo). |
