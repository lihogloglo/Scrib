# Shader and pipeline precompilation: research (2026-11-11)

Owner ask (2026-11-11): objects appear one by one when a scene loads; nothing should compile on a loaded, visible
scene. ROADMAP Phase 1 `[p]` "Shader/pipeline precompilation"; related ROADMAP 1.1 (hold the door fade until the
interior is fully loaded). Design: [`specs/pipeline-warmup.md`](../specs/pipeline-warmup.md). Research only, no code.

## 1. Why things pop in: four separate causes

| Cause | Mechanism in Bevy 0.20 (our vendored tree) | What the player sees |
|---|---|---|
| **A. Pipeline not compiled** | `PipelineCache` compiles on first use, on `AsyncComputeTaskPool` (`vendor/bevy_render/src/render_resource/pipeline_cache.rs:1057-1068`, `create_pipeline_task`). `SetItemPipeline` skips the draw while the pipeline is `Queued`/`Creating` (`render_phase/mod.rs:2512-2530`). | The object is missing for N frames, then appears (one by one, as compiles finish). Its shadow appears separately (shadow views specialise their own pipelines). |
| **B. Pipeline never requested yet** | `specialize_material_meshes` only specialises entities **visible to a view** (`vendor/bevy_pbr/src/material.rs:1068-1100`: it iterates `RenderVisibleEntities`). Same for prepass and shadow views. | Turning around inside a fresh interior, or a lamp's shadow first seeing a caster, starts new compiles: "first-sight" pop/hitch (SH-17 kind). Warming only what the camera sees at the reveal is not enough. |
| **C. Asset not prepared** | A mesh whose `RenderMesh`, material instance or `PreparedMaterial` is missing goes to the pending queue and is not specialised (`material.rs:1110-1135`); a material whose texture is still loading does not prepare. | Object missing until its textures upload, then a pipeline compile on top (A after C). |
| **D. Streaming order and budgets** | Cells and placements spawn over several frames under per-frame budgets (streaming.md "Scheduler and budgets"). | Objects appear in spawn order (exteriors: at the ring edge; at a door: inside the hold if the gates cover it). |

Our code already handles part of this: `mw_devtools_bevy/src/pipeline_watch.rs` (`PendingDraws`) counts camera-visible
meshes whose main-pass pipeline is still compiling and holds the door fade on it (`pipelines_at_black`), and
`sf_world_bevy/src/fade.rs` holds on cells and textures within 48 m (`ready_radius_m`). **Gaps found:**
- `count_pending` only counts entities already present in `SpecializedMaterialPipelineCache`, so meshes still in
  cause C (not specialised yet) count as zero, and the gate can open before they have even queued a pipeline.
- Shadow views are left out (documented), so a shadow can pop after the reveal.
- Only camera-**visible** meshes count (cause B): everything behind the camera compiles after the reveal.
- The pipeline gate is capped by `gate_max_secs` = 1.5 s (`fade.rs:160`); a cold compile of a new interior can run
  longer, and then the fade opens on a half-drawn room.
- The new-game / load path (`AppPhase::Loading` → `InGame`, menus.md §1) waits on streaming `Ready` only.

## 2. Where the compile time goes (Bevy 0.20 + wgpu 30.0.1, Vulkan)

1. **Front end, per unique (shader, shader defs):** WESL composition (`bevy_shader::ShaderCache::get`), naga
   parse/validate, and on Vulkan the SPIR-V write inside `create_shader_module`
   (`wgpu-hal-30.0.1/src/vulkan/device.rs:1855-1900`; deferred to pipeline creation only when the module has
   overrides). Bevy holds the `shader_cache` and `layout_cache` locks for this whole step
   (`pipeline_cache.rs:555-583`, dropped at `:583`), so **all front-end work is serialised** however many threads
   there are: [bevy#26152](https://github.com/bevyengine/bevy/issues/26152) (open, 2026-10-13; a split `prepare`/
   `finish` patch took a warm-start load from 5.0 to 3.1 s on an M1 Pro). Modules are cached in memory only.
2. **Back end, per pipeline:** `device.create_render_pipeline` (driver compile). Runs in parallel, on the async
   compute pool: Bevy default 25 % of cores, **at most 4 threads** (`bevy_app-0.20.0/src/task_pool_plugin.rs:129-135`;
   we don't override `TaskPoolOptions`), shared with any other async-compute work. `cache: None` on every descriptor (`pipeline_cache.rs:647,
   709, 847`): no `wgpu::PipelineCache` is used.
3. **Driver disk caches** (NVIDIA's, AMD's) already keep compiled pipelines between runs on desktop, so a second launch
   mostly pays (1). A driver update or a cleared cache pays (1) + (2).
4. **No dedup:** `PipelineCache` gives every `queue_render_pipeline` a new id, identical descriptors included
   ([bevy#10871](https://github.com/bevyengine/bevy/issues/10871) notes this); the material specialisers dedup by key
   in front of it, so normal material use is fine, but a replayed descriptor log would compile twice.

**Backend.** DLSS needs Vulkan (`bevy_anti_alias` dlss module docs; `vendor/dlss_wgpu/src/initialization.rs` creates
the Vulkan instance), so the High preset (1440p + DLSS Q) runs on Vulkan. `gs-patch-plan.md:379` calls DX12 "our
shipping backend": a contradiction to settle from the startup adapter log line (open question in the spec). On DX12,
wgpu 30's `create_pipeline_cache` is a no-op (`wgpu-hal-30.0.1/src/dx12/device.rs:2269-2274`): no
`ID3D12PipelineLibrary`.

**wgpu `PipelineCache`** (`wgpu-30.0.1/src/api/pipeline_cache.rs`, [docs](https://docs.rs/wgpu/30.0.1/wgpu/struct.PipelineCache.html)):
`Features::PIPELINE_CACHE` (bit 41), Vulkan only; `get_data()` to persist, `util::pipeline_cache_key(adapter_info)`
as the file name, atomic rename on write. Its own docs: "most desktop GPU drivers will manage their own caches,
meaning that little advantage can be gained from this on those platforms". It does not cache the front end (1).

## 3. Industry

**Unreal Engine 5: PSO precaching (5.1 experimental, 5.2+ default) plus bundled PSO caches.**
[Docs](https://dev.epicgames.com/documentation/en-us/unreal-engine/pso-precaching-for-unreal-engine);
[Epic tech blog, 2025](https://www.unrealengine.com/en-US/tech-blog/game-engines-and-shader-stuttering-unreal-engines-solution-to-the-problem)
(not fetched: HTTP 403; summarised from the docs and search excerpts); [Tom Looman's setup guide](https://tomlooman.com/psocaching-unreal-engine).
- **Prediction, not replay.** Primitive components request every PSO they might need right after load (`PostLoad`):
  each mesh pass processor adds initialisers from (material, vertex factory, precache params). Background tasks
  compile them, deduplicated through a shared cache. Epic moved from recorded caches toward prediction because
  recordings go stale and miss content.
- **Not-ready policy.** `r.PSOPrecache.ProxyCreationWhenPSOReady` (default on) delays the render proxy until its
  PSOs are compiled; `ProxyCreationDelayStrategy` 0 = skip the draw (default), 1 = draw with the default material.
  Without delay the draw blocks and hitches. Landscape cannot delay, so it pops.
- **Loading screens** stay up until `FShaderPipelineCache::NumPrecompilesRemaining()` reaches zero.
- **Global shaders** precached at startup; global graphics PSOs created in the first frames still hitch, covered by
  a small **bundled** (recorded) cache. So: predict content PSOs, record the rest.
- **Validation counters** (`r.PSOPrecache.Validation`, `stat PSOPrecache`): Missed, Untracked, Hit, **Too late**,
  Used, Precached; runtime compiles over `r.PSO.RuntimeCreationHitchThreshold` (20 ms) logged with material, vertex
  factory and pass. `-clearPSODriverCache` for cold tests (driver caches mask misses).
- Driver cache detail: precached PSOs are released after compiling and reloaded from the driver cache on first use,
  which can still cost milliseconds; NVIDIA-only `KeepInMemoryUntilUsed`. First-run cold compile target: under a
  minute on mid-range CPUs.

**Unity 6: PSO tracing + `GraphicsStateCollection` warm-up.**
[PSO tracing](https://docs.unity3d.com/6000.4/Documentation/Manual/shader-pso-trace.html),
[prewarming](https://docs.unity3d.com/Manual/shader-prewarm.html),
[`WarmUpProgressively`](https://docs.unity3d.com/ScriptReference/Rendering.GraphicsStateCollection.WarmUpProgressively.html),
[Unity's post](https://discussions.unity.com/t/prevent-shader-compilation-stutters-with-pso-tracing-in-unity-6/951031).
Record PSOs in development builds (`BeginTrace`/`EndTrace`, `.graphicsstate` file), ship the collection, warm it at
load (`WarmUp`) or spread over frames (`WarmUpProgressively(count)`), `traceCacheMisses` to record what the
collection missed. Legacy `ShaderVariantCollection.WarmUp` warmed variants by drawing dummy geometry (no full PSO).

**Vulkan / D3D12 APIs.**
[`VK_EXT_graphics_pipeline_library`](https://docs.vulkan.org/features/latest/features/proposals/VK_EXT_graphics_pipeline_library.html)
and [Valve's Khronos post on Source 2](https://www.khronos.org/blog/reducing-draw-time-hitching-with-vk-ext-graphics-pipeline-library):
compile vertex-input / pre-raster / fragment / output parts separately at load and fast-link at draw time (cheap
enough to avoid hitches), usually with dynamic state; Zink does the same ([zmike](https://zmike.github.io/through-the-loop)).
Clockwork uses it (`frame/pipelinetable.*`, `LibraryPart`). wgpu does not expose pipeline libraries or shader objects,
so this is out of reach without a wgpu fork. `VkPipelineCache` / D3D12
[`ID3D12PipelineLibrary`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12pipelinelibrary)
are the on-disk blobs; Steam's shader pre-caching (Fossilize) replays Vulkan pipeline state on Linux/Deck.

**Fewer pipelines in the first place.** id Tech 6/7 (DOOM 2016, Eternal) are known for a small pipeline count and
bindless resources ([AMD GPUOpen, "Vulkan and DOOM"](https://gpuopen.com/learn/vulkan-and-doom/)); the often-quoted
"about 1,000 pipelines" figure is anecdotal (forum comment, no primary source found). Our static-scene material
classes (`static-scene.md` §5: 8 classes as built, target ≤ ~16) are the same lever.

**Hide what remains.** Bethesda (Creation Engine) and most open-world games hide loads behind loading screens or a
held fade; many recent DX12 PC ports compile shaders at first launch or in the main menu (reported widely, e.g.
The Last of Us Part I; no primary source fetched). Clockwork precompiles a material catalog (VFS stamps,
`materialStamps`) with progress bars at startup and keeps a `VkPipelineCache` on disk
([our notes](clockwork-code-2026-10-25.md), addendum 2026-11-01).

## 4. Bevy ecosystem

- **Upstream:** no precache or warm-up API. [bevy#10871](https://github.com/bevyengine/bevy/issues/10871) (open since
  2023-12): pipeline log for loading screens; JMS55 later points at Epic's predictive approach ("scan new materials
  and guess views + meshes"). [bevy#19809](https://github.com/bevyengine/bevy/pull/19809) (open, +383/-14,
  tychedelia, last update 2026-08): `PersistentPipelineCache` resource wiring `wgpu::PipelineCache` (Vulkan) into
  `PipelineCache`, with JMS55's caution that it mostly helps Android. [bevy#26152](https://github.com/bevyengine/bevy/issues/26152):
  serialised shader composition (above). `RenderPlugin::synchronous_pipeline_compilation` blocks the render
  thread per compile (fine for tests and captures, wrong for play).
- **Bevy's `loading_screen` example** and [`bevy_pipelines_ready`](https://github.com/rparrett/bevy_pipelines_ready)
  (0.9.0, 2026-06) count ready pipelines into the main world for a loading state. Our `PendingDraws` already does
  better (counts visible draws held back, not a global total); the crate adds nothing we need.
- No crate does predictive precaching for Bevy materials.

## 5. Conclusions for the spec

1. Canonical technique: **predictive precache** (UE) over a **content catalog**, with a **recorded miss log** for
   what prediction cannot see (UE bundled cache, Unity `traceCacheMisses`), and **block the reveal** until the
   precache count is zero (UE loading-screen rule). During play, a late pipeline **skips the draw** (Bevy stock =
   UE strategy 0) and is counted.
2. Bevy has no precache entry point, and specialisation is tied to view visibility (cause B), so prediction needs a
   **vendored `bevy_pbr` patch** that runs the existing per-phase key building for "specimens" (mesh, material,
   entity flags) against each live view: an own patch offered upstream as bevy#10871's answer.
3. Front-end cost (serialised WESL + naga) is likely the dominant warm-launch cost and is fixed by the bevy#26152
   approach, not by any disk cache. Measure before choosing.
4. `wgpu::PipelineCache` (bevy#19809, Vulkan only) is cheap to carry and helps only the cold-driver-cache case; adopt
   it behind a measurement, store it in the bake store's `gpu/<adapter key>/`.
5. Reduce permutations first and keep them reduced (material classes, view keys that don't flip in play).
