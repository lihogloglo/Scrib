# SPUD's frame against ours: why 375 fps there and 80 here (2026-11-15)

Research only. Nothing built or run, in SPUD or here. Extends [spud-code-2026-11-08.md](spud-code-2026-11-08.md) and
[cpu-frame-audit-2026-11-15.md](cpu-frame-audit-2026-11-15.md) section 3, and corrects both in places.

**Source.** `https://github.com/chasmlol/spud`, `main` at **`e5f2de4b5a612eef3f2cd15e4a84672e9e0a0f1c`** (2026-11-13),
163 commits. Paths below are relative to that repo. Since `8c99042` (2026-11-07) the commits are gameplay, doors,
jointed models and Papyrus; nothing touches the renderer's frame structure, `docs/PERF_CHANGES.md` (last change
2026-11-07) or `tools/perf` (last change 2026-11-06). The numbers below are therefore the same ones we read a week ago.
**F** = fact (file:line or doc), **I** = inference.

## 1. Short answer

1. **The 375 fps is on an RTX 5090 with a Ryzen 7 9850X3D** (F). Our 80 fps is on an RTX 4060 Laptop. That GPU is
   roughly 5x ours at 1440p (I, public benchmark ratios). SPUD's 1.75 ms of GPU would be about 8-9 ms on our laptop
   (I): about 110-125 fps, GPU bound. **Most of the "375 vs 80" is hardware.** Our earlier notes said "hardware not
   stated"; it is stated, in two files we did not open.
2. **SPUD is not feature-poor.** Our audit said it has no SSAO, bloom, volumetrics or shadowed lamps. Wrong: it draws
   Skyrim SE's full set (2 sun cascades at 4096², up to 4 shadowed lights, a 320x192x90 froxel light volume, full-res
   AO, half-res SSR, water with a reflection cube, HDR/bloom/DoF) at **native** 1440p with 4x MSAA (F, section 3).
   It shades 2.25x more pixels than our DLSS Quality internal 1707x960.
3. **The real difference is CPU cost per frame.** SPUD's whole main thread (game + cull + draw build + record +
   submit) is about 1.3 ms of work per frame on its CPU (I from F); our render thread alone is 11.5 ms. That part is
   engine, not hardware, and it is the part of the gap that is ours to close.
4. **Which bound are we?** On the walk: **CPU (render thread)**. Frame 12.27 ms against 7.46 ms of timed GPU passes
   (audit :16, :25). Standing: frame 10.96 against 8.87 of timed GPU, so **near GPU bound**. The owner's reading is
   right about what comes next: remove all CPU cost and the frame stops at 7.5-8.9 ms (112-133 fps), not at SPUD's
   figure. With the near ring off we already sit at 7.2-7.7 ms with 7.24 of GPU (audit :39).

## 2. Conditions of SPUD's numbers

| item | value | source |
|---|---|---|
| GPU / CPU | RTX 5090, Ryzen 7 9850X3D (8 cores) | F: baseline file name `tools/perf/baselines/nvidia-geforce-rtx-5090-amd-ryzen-7-9850x3d-8-core-processor-headless.json`; `tools/perf/budgets.json:4-5` "Dev machine" |
| resolution | 2560x1440 native, no upscaler | F: `tools/perf/tour/tour.bench:7`, `budgets.json:5` |
| AA | MSAA 4x | F: `src/engine/app.cpp:1430` |
| preset | `user-2026-11`: the owner's Skyrim INI. uGridsToLoad 11, shadows 4096² to 10000 units, volumetric on, SSR on, AO on | F: `src/engine/app.cpp:1426-1440` |
| LOD | High, 1.0 px error; whole map drawn | F: `budgets.json:5`, `docs/ARCHITECTURE.md` "LOD" |
| scene | "the 300 fps tour": the real game, 12 sections of ~11 s (Whiterun market, a door crossing, dialogue, flight, forest, night, dungeon...), noon, fixed weather | F: `tour.bench:1-30` |
| window | **headless**: hidden tool window, presents every frame, immediate mode (else mailbox). No vsync | F: `docs/AUTOMATION.md:12`, `tools/perf/perf.py:139-140` |
| "fps" | 1000 / mean `cpu_ms`: wall time of a whole main-loop frame, present included. `gpu_ms` is timestamp queries around the passes | F: `tools/perf/perf.py:216`, `src/engine/bench.cpp:202`, `src/render/renderer.cpp:437-451` |
| tick | world steps at 60 Hz; other frames are "still": they draw, nothing steps. About 1 frame in 6 runs a tick | F: `src/engine/app.cpp:1194-1207`, `docs/PERF_CHANGES.md` "The rest of the tick" |
| result | 375-378 fps mean (2.65 ms), GPU mean 1.73-1.75 ms, present 0.76 ms; noise ~10 % | F: `docs/PERF_CHANGES.md` "Where it ended" |
| budget | 600 fps mean / 480 fps 1 % low on that machine (was 1000 / 800) | F: `budgets.json:5-13` |
| other scenes | city-orbit (Whiterun, 40 m up) CPU 1.40 / GPU 1.19 ms; forest-low 0.94 / 0.75; interior 0.60 / 0.42; bare present 0.17 (stored 2026-11-04, when present was 0.17 ms) | F: the baseline json |

Caveats they state themselves (F, `PERF_CHANGES.md`): never run in a real window; the headless present is a
compositor copy (0.76 ms, was 0.17 on 4-5 November: 502-524 fps then); frame = GPU time + present.
No public post, video or third-party benchmark found (one web search, nothing relevant). Nobody has published SPUD
numbers on mid-range hardware. **Unstated:** draw counts, visible instance counts, per-pass GPU split of the tour
(the harness records them per run; the results are not in the repo).

## 3. SPUD's frame, end to end

**Threads (F).** One main thread does game logic, culling, draw building, command recording and the submit. Beside it:
a present thread (`src/render/output.cpp:61`, waits on the frame fence then presents, `:312-324`), an upload thread on a
transfer queue (`src/render/uploader.cpp:261`), a streaming job pool (`src/render/jobs.hpp`, hardware threads - 1), a
player physics thread (`src/engine/player.cpp:89`), an audio thread, and a parallel-for used for actor animation
graphs (`src/engine/actors.cpp:4265`). There is no render thread and no second world.

**Per frame on the main thread (F unless marked):**

1. Tick or still (`app.cpp:1194-1207`). On a still frame nothing steps: no AI, no physics result, no skinning upload
   (`app.cpp:941`), no tree sway (`:765`). Actors blend between the two last ticks.
2. Wait for the frame slot's fence, 2 frames in flight (`renderer.cpp:463-479`, `src/render/vk.hpp:30`).
3. `Space::build_draws` (`src/engine/space.cpp:963`): single-threaded loop over per-cell flat arrays. Cell AABB vs
   frustum, terrain horizon, per-reference sphere, sub-pixel cull, rooms, occluder planes, LOD pick. Output: POD
   `DrawItem`s. No compute culling exists (the only compute shaders are `skin.comp`, `skin_decal.comp`,
   `vl_volume.comp`); `docs/ARCHITECTURE.md` "Rendering" describes Hi-Z compute culling that is not built.
4. `Renderer::render` (`renderer.cpp:1162`, begin `:1190`): **one command buffer, one `vkQueueSubmit2`** (`:1932`). One bindless
   descriptor set for the whole frame (`:1307`, re-bound only after a pass that changes layout; 65536 texture slots, `src/render/bindless.hpp:34`);
   everything else by buffer device address in push constants (`:1293` onward). Per-frame data goes into one
   host-visible arena sized up front (`:1231`).
5. `build_mesh_draws` (`:612`): three-pass counting sort into the arena; `record_runs` (`:579-594`) issues one
   `vkCmdDrawIndexedIndirect` per pipeline-state run. Cull mode, front face, depth test/write and depth bias are
   dynamic state (`src/render/pipelines.cpp:542-544`), so state changes cost a command, not a pipeline.
6. Pipelines: 80 programs (`src/render/pipelines.hpp:18-`), mesh shaders in 5 specialised variants (`:112-117`), blend
   pipelines as an 11x11 factor table (`:100`). All built at startup in parallel through a disk `VkPipelineCache`
   (`pipelines.cpp:316, :636`). Nothing compiles in play.

**Passes, in order (F: `gpu_pass_name`, `renderer.cpp:99-118`, and the record calls `:1311-1317` onward):**

| pass | what | size / rate |
|---|---|---|
| prepare | GPU skinning compute, water reflection cube (2 faces a frame), flow maps, offscreen views | `docs/RENDERING.md` allowlist |
| sun | **2 cascades** (iNumSplits 2; max 3), one depth pipeline, casters bucketed per cascade | 4096² each (`src/render/view.hpp:224-236`). **The far cascade is redrawn only on tick frames** unless the camera moved 4 m or turned 3° (`app.cpp:1342-1374`): at 375 fps that is ~1 frame in 6 |
| volume | froxel light volume, compute | 320x192x90 (`src/render/renderer_post.cpp:39`) |
| shadow | **at most 4 shadowed lights**; a spot is 1 pass, an omni is a dual paraboloid (2 halves of one map) | 4096² (`view.hpp:225`, `renderer_shadows.cpp:281-332`) |
| opaque, terrain, alpha_test, sky | forward lit, 2 colour targets (HDR + normals), 4x MSAA, reverse-Z | native |
| ao, ssr | AO full res (raw + blur); SSR half res | `renderer_post.cpp:101-113` |
| composite | AO/fog/volumetric apply, MSAA resolve | |
| water, alpha | water in its own target with history; blended draws sorted, one command each | |
| image_space, post, ui | HDR luminance chain, bloom, eye adaptation, DoF, tonemap, lens effects | quarter-res chain |
| door | one extra scene pass, scissored, when a portal is in view | |

**Lighting (F).** Forward. Each object carries a list of 8 light indices built at load from all the world's lights,
7 used, plus 2 slots for moving lights (`src/render/light_grid.hpp:17-22`). No clustered or tiled culling.
**Textures (F).** DDS as shipped, bindless, loaded on workers, LRU eviction over an 8 GB budget
(`src/render/textures.hpp:6, :145`). No mip streaming seen. The 5090 has 32 GB.
**AA (F).** MSAA 4x, alpha-to-coverage, Toksvig specular AA; TAA and upscalers are banned (`docs/RENDERING.md`).
MSAA 4x costs them 0.25 ms GPU (`PERF_CHANGES.md` "What MSAA 4x costs").

## 4. Side by side

Ours: `balmora_walk`, High, 1440p + DLSS Quality, RTX 4060 Laptop (audit :16-29). Theirs: tour mean, RTX 5090.

| cost item | ours (ms) | SPUD | mark |
|---|---|---|---|
| Frame | 12.27 walk / 10.96 standing | 2.65 (tour), 1.40 (city orbit, stored when present was 0.17) | F / F |
| Game logic | main thread 9.27 busy, not the bound; hundreds of small systems every frame | 60 Hz tick on ~1 frame in 6; tick frames +0.17 ms on average | F / F |
| Extract (second world) | ~1.1 | none | F / F |
| Visibility | part of main; chained `par_iter` scans, ~1.9x parallel | one single-threaded loop over flat cell arrays | F / F |
| Prepare / queue / bins | ~4.2 (46 views, 7351 bins, 19.5 k phase items) | counting sort into an arena; cost not published, inside the ~1.3 ms total | F / I |
| Command recording | ~0.5 record + 3-4 wall `encoder.finish`, one encoder per pass | one command buffer, recorded once, no validation layer in between | F / F |
| Submit | ~1.5 serial, 36-61 command buffers | 1 command buffer; present on its own thread | F / F |
| Pipelines / draw runs | 93 pipelines, 113 batch sets | draw calls = state runs; 6 states dynamic | F / F (count unpublished) |
| Views | 46: 1 main, 3 cascades, 42 lamp faces (most cached) | main + 2 cascades (far one ~1/6 rate) + ≤ 4 lights (1-2 passes) + water cube faces | F / F |
| **CPU total on the bound thread** | **~11.5 render + 1.1 extract** | **~1.3** = 2.65 frame, "about half waiting for the GPU" | F / I from F |
| GPU total (timed) | 7.46 walk / 8.87 standing | 1.75 on a 5090; ~8-9 on our GPU | F / I |
| GPU: AA / upscale | DLSS 2.08 on 1.64 MP shaded | MSAA 0.25 (5090) on 3.69 MP shaded | F / F |
| GPU: main opaque + prepass | 1.66 + 0.52 | forest opaque + alpha test 1.13 (5090); no depth prepass | F / F |
| GPU: sun shadows | 3 cascades each frame, PCSS | 2 cascades, 16 fixed taps, far one held | F / F |
| GPU: lamp shadows | cube faces, face cache | ≤ 4 lights, spot or paraboloid | F / F |
| GPU: sky / fog | atmosphere LUTs 0.32, clouds, froxel fog | sky mesh, froxel light volume | F / F |
| GPU: water | velocity + SSPR 0.50, FFT | reflection cube 2 faces a frame + SSR + history | F / F |
| GPU: bloom, AO | 0.32, SSAO | bloom chain, full-res AO | F / F |

Hardware scaling behind the "I" rows: a 5090 is about 1.3x a 4090 and a 4090 about 3.7-4.3x a desktop 4060 at
1440p-4K; a 140 W laptop 4060 is slightly under the desktop card. So ~5x on GPU. A 9850X3D is perhaps 1.3-1.8x our
laptop CPU on one thread (our CPU model is not in the audit; wide range on purpose). So SPUD here: GPU ~8-9 ms,
CPU ~2-2.5 ms, **GPU bound at roughly 110-125 fps** (I). Their 8 GB texture budget and MSAA targets would also press
on our 8 GB of VRAM (I).

## 5. Where our 12.3 ms goes, in SPUD's terms

The walk is CPU bound, so the split is of CPU time on the critical path. All three are estimates (I) built from the
audit's measured rows; they are ranges, not a decomposition we measured.

- **(a) Features SPUD does not have: ~1.0-1.5 ms.** A third sun cascade drawn every frame, lamp cube faces beyond
  SPUD's 4-light cap, the depth prepass, DLSS's extra passes, atmosphere LUTs. Basis: all shadows off is -1.55 ms and
  leanest High is 9.37 against 12.3 (audit :40, :110), but SPUD keeps shadows, AO, SSR, bloom and a fog volume, so
  only part of that 2.9 ms counts here. On the GPU side the list is short too: DLSS 2.08 + LUTs 0.32 + PCSS, against
  their MSAA and 2.25x the shaded pixels. **Features do not explain the gap.**
- **(b) Bevy / wgpu architecture: ~4-5 ms.** Extract 1.1; a per-pass encoder whose real encode happens again at
  `finish`; wgpu-core validation and usage tracking; `queue.submit` walking 36-61 buffers; retained bins and phase
  items per view; cull mode, depth write and bias baked into the pipeline (WebGPU has no dynamic state for them);
  one shared 8-thread pool. This is the price of the stack and stays unless we patch upstream.
- **(c) Things SPUD does well that we can fix: ~3-5 ms.** Ranked below.
- The rest, ~2-2.5 ms, is what SPUD's own frame would cost on our CPU.

| # | (c) item | canonical name | expected walk | in audit plan? |
|---|---|---|---|---|
| 1 | Fewer state runs: one vertex layout, one slab per layout, fold key bits | state-sorted draw streams; unified vertex factory + shared vertex/index arena | -1.0 to -2.0 | yes, item 2 |
| 2 | Fewer, shorter encodes and one submit | single command list per frame / parallel command lists per view | -1.0 to -2.0 with the cascade split | yes, items 1 + 3 |
| 3 | Far cascade redrawn on a rule, not a timer: hold it until the camera moves 4 m or turns 3°, re-offset its rows by the camera delta | cached shadow maps (UE CSM caching, scrolling cascades) | -0.4 to -0.8 | item 4, but **the rule is new**: movement-triggered, no fixed stagger, no visible lag when still |
| 4 | **Budget shadowed lamps: a hard cap, spots as 1 view, omni as dual paraboloid (2 views, not 6)** | shadow-casting light budget; dual-paraboloid shadow maps (Brabec 2002; Skyrim, GTA) | -0.2 to -0.5 CPU (lamp faces 1.76 thread-ms traced, off the wall-critical path), plus GPU when faces miss the cache | **new** |
| 5 | **Two cascades at 4096² instead of three** on High (Skyrim SE's own default) | cascade count as a preset knob | -0.3 to -0.6 (cascades are 48 of 113 batch sets) | **new** (plan only staggers) |
| 6 | **World steps at a fixed 60 Hz; frames between ticks only interpolate and draw** | fixed timestep with render interpolation ("Fix Your Timestep") | 0 on the walk today (main is not the bound); frees the shared pool; matters above 60 fps once render is cut | **new** as a frame-rate lever (we have it as a net-ready rule) |
| 7 | No depth prepass where overdraw is low | forward without Z-prepass, front-to-back state sort | -0.3 to -0.5 CPU (one fewer full-scene encode), GPU unknown: measure first | **new**, needs an A/B; DLSS and SSAO need its outputs |

Not adoptable: presenting from our own thread (inside wgpu), dynamic cull/depth state (WebGPU), MSAA instead of DLSS
(owner decision), SPUD's static path as a whole (our `--scene` equivalent reached parity only and is frozen).

**What the audit plan should change.** Nothing in its order. Add items 4, 5 and 7 above as measurements (each is a
flag flip or a small A/B), and take the movement rule for item 3. Correct audit section 3: SPUD's feature list and
hardware. And set the expectation: **closing all of (c) lands near 8-9 ms, where the GPU takes over**; 120 fps then
needs the GPU items (plan item 7), not more CPU work.

## 6. Is the content comparable?

Roughly, not measured (I). Their city scene is Whiterun with uGridsToLoad 11 (121 cells resident at full detail) and
the whole map drawn through HLOD and impostors; ours is Balmora with a near ring and far proxies. Skyrim SE meshes
are denser per object; Morrowind towns have more small low-poly objects per screen, which is the worse case for a
per-draw-set cost like ours and nearly free for an instanced multi-draw path like theirs. Lamps: Skyrim caps
shadowed lights at 4 by design; our walk carries 42 lamp faces, i.e. 7 shadowed lamps. SPUD publishes no draw or
instance counts, so a like-for-like count needs their harness output (`state <file.json>`), which we cannot run.

## 7. Not established

- SPUD on any hardware near ours. The 8-9 ms figure is a ratio estimate.
- SPUD's CPU split (cull vs build vs record). Only the total is derivable.
- Our laptop CPU's single-thread ratio to a 9850X3D.
- Whether dropping the prepass or the third cascade is acceptable visually: owner checks.
