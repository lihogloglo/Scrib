# Pipeline warm-up (shader/pipeline precompilation)

> **Agent summary (2026-11-11).** Status: W0 (measure) and W1 (reveal gate) built, owner runs pending; ROADMAP Phase 1 `[~]` line "Shader/pipeline
> precompilation" (owner go 2026-11-11). Goal: no render pipeline compiles while a loaded scene is visible; a reveal
> (new game, load, door) waits until what it shows, and what is around it, can draw. Technique: UE5-style
> **predictive precache** of a **content catalog** through a vendored `bevy_pbr` patch, a **reveal gate** that blocks
> on it, a **miss log**, and **measured** disk/front-end caching. Research: [`shader-precompile-2026-11-11`](../research/shader-precompile-2026-11-11.md).
> Game side (Morrowind catalog adapter): §9. Bake interface: §7.

## 1. Goal and non-goals

- **Goal.** (1) A reveal never shows a frame with a draw held back by a pipeline, a texture or a spawn that is
  within the reveal radius; (2) after a reveal, turning around or a lamp's shadow first seeing a caster compiles
  nothing (first-sight hitches, SH-17 kind); (3) during exterior play, content entering the view draws on its first
  visible frame in ≥ 99 % of cases, the rest are counted; (4) warm-up time is bounded: first launch inside the bake
  budget slot (bake.md: 20 s reserved), later launches and door crossings under the hold caps.
- **Non-goals.** Pipeline libraries / shader objects (wgpu exposes neither). A fallback material for late
  pipelines (UE strategy 1: changes the look; owner question 3). Background compiling of settings the player is
  not using during play (costs frame time; owner question 4).

## 2. The four causes of pop-in, and who owns each

| Cause (research §1) | Owner | Fix |
|---|---|---|
| A. pipeline compiling | this spec | precache before reveal; gate on it |
| B. pipeline never requested (not visible yet) | this spec | precache by catalog and by residency, not by visibility |
| C. asset not prepared (mesh, material, texture upload) | streaming.md / fade.rs, gate defined here | the reveal gate counts it (today's counter misses it) |
| D. spawn order and budgets | streaming.md | the reveal gate counts it; exterior ring order is streaming's |

The one reveal gate (§5.4) is the shared contract; the door fade (ROADMAP 1.1) and the `Loading` → `InGame`
transition (menus.md §1) both use it.

## 3. Canonical technique and industry comparison

**Canonical (UE 5.2+ PSO precaching, the current industry answer):** predict every pipeline a primitive may need
when it loads, from (material, vertex layout, pass), compile in the background, deduplicated; keep the loading screen
up until outstanding precaches reach zero; during play a primitive whose pipelines are not ready is **not drawn**
(skip) rather than hitching; record what prediction missed into a small bundled cache; count Missed / Hit / Too late.

| Engine | How pipelines are found | When | Not ready | Disk | Us |
|---|---|---|---|---|---|
| UE5 | predicted per component at `PostLoad` + bundled recorded cache for globals | load, background | skip draw (default) or default material | driver cache, bundled PSO cache | **match**: predict (§5.2) + miss log (§5.5), skip during play, block the reveal |
| Unity 6 | recorded `GraphicsStateCollection` (dev builds), `traceCacheMisses` | load or progressive | compile at draw (hitch) | collection file, driver cache | we predict instead of relying on a recording: recordings go stale with mods |
| Clockwork | material catalog from VFS stamps, Vulkan pipeline libraries | startup, with progress | n/a (all compiled) | `VkPipelineCache` | **match** on catalog + progress; no libraries in wgpu |
| id Tech 7 | few pipelines, bindless | startup | n/a | driver cache | **match** in spirit: material classes keep the count small |
| Bevy 0.20 stock | specialise on first visibility | first visible frame | skip draw | none | the gap this spec closes |
| Bethesda / most open-world | loading screens and fades hide it | load | hidden by the load | driver cache | the reveal gate |

Gap kept: pipeline libraries (fast-link at draw) need wgpu support; listed in `industry-audit.md` row "pipeline
warm-up" when this spec is built.

## 4. Reuse check

1. **Bevy:** `PipelineCache` (async compile, states, `waiting_pipelines`), `SpecializedMaterialPipelineCache`,
   `RenderPlugin::synchronous_pipeline_compilation`. No precache entry point; specialisation runs only for entities
   in a view's `RenderVisibleEntities` (`vendor/bevy_pbr/src/material.rs:1068-1100`). Upstream issue
   [bevy#10871](https://github.com/bevyengine/bevy/issues/10871) is open with no PR.
2. **Upstream PRs to carry as vendored patches** (rule 4): [bevy#19809](https://github.com/bevyengine/bevy/pull/19809)
   (`wgpu::PipelineCache`, open) and the [bevy#26152](https://github.com/bevyengine/bevy/issues/26152) approach
   (parallel shader composition; issue only, no PR: ours to write and offer).
3. **Crates:** `bevy_pipelines_ready` counts ready pipelines; our `PendingDraws` already does more. Nothing predicts.
4. **Own code:** the precache patch (§5.2, an own patch offered upstream as the answer to #10871), the catalog and the
   gate.

## 5. Design

### 5.1 Permutations: keep the set small and fixed during play

Pipeline count = material key × vertex layout × entity bits × view key × pass. Rules:
- **Material state:** static materials fold into classes (`static-scene.md` §5, `sf_world_bevy::matclass`: 8 as
  built, target ≤ ~16). Custom materials (terrain, water, particles, precipitation, impostors, light sprites,
  kick-up, froxel fog, view model, far shadow) are few and enumerable.
- **View key** (`MeshPipelineKey` view bits: MSAA, HDR, tonemap method, deband, prepasses, TAA jitter, SSAO, SSR,
  contact shadows, env map, irradiance volume, distance fog, atmosphere, OIT, shadow filter, GI_PROBES,
  SURFACE_WEATHER): fixed by the graphics preset. A setting that flips a view bit (F4 TAA/DLSS/MSAA cycle, preset
  change, `--off` switches that toggle a view component) is a **settings change**: it re-warms behind a short black
  hold with the same gate (§5.6), as PC games do on "Apply". Dev `--off` switches are exempt from the budget.
- **Gameplay must not flip view bits** (weather, interiors/exteriors, time of day). Today: GI_PROBES and
  SURFACE_WEATHER follow the view's components; slice W0 lists every view-bit change seen in a session so any
  per-worldspace flip is caught (an interior without a probe volume = a different key for everything).

### 5.2 Predictive precache (vendored `bevy_pbr` patch "PW")

A render-world request queue, `PipelinePrecache`, of **specimens**:

```text
Specimen { mesh: AssetId<Mesh>, material: UntypedAssetId, flags: SpecimenFlags }   // flags: entity-level key bits
SpecimenFlags: SHADOW_CASTER, VISIBILITY_RANGE_DITHER, LIGHTMAPPED, HAS_PREVIOUS_SKIN, HAS_PREVIOUS_MORPH, NOT_SHADOW_RECEIVER ...
```

For each specimen whose `RenderMesh` and `PreparedMaterial` exist (otherwise it waits, counted as `asset`), and for
**each live view** (camera views and their shadow views: directional cascades, point faces, spot), the patch runs
**the same key-building code** as `specialize_material_meshes`, `specialize_prepass_material_meshes` and the shadow
specialiser (factored into shared functions so precache and draw can never disagree), and calls the material's
`base_specialize`. The resulting ids are kept per (view, specimen key) so the precache knows when they are `Ok`.
Results land in the same specialiser caches the draw path uses, so the first real draw is a hash hit.

- Shadow views come and go (lamps stream in and out): a precache covers the **view kinds** (directional cascade,
  point face, spot) present, plus one template per kind if none is live yet (the shadow pipeline does not depend on
  which lamp). Same for cameras: main camera, first-person view model camera, any reflection/probe views.
- Public surface (main world): `PrecacheRequest` events or a `Precache` resource with `request(specimens, tag)` and
  `status(tag) -> { pending_assets, queued, compiling, ready, failed }`. Tags group a load (e.g. "arrival", "catalog").
- Non-material pipelines (post-process, compute: cull, GI, atmosphere, shadow cache, static-scene compute) are
  created at plugin init or view setup; the gate waits on `waiting_pipelines() == 0` for them (UE's "global shaders").
- Marker `superfargoth patch (own, PW)`, a vendor README row, offered upstream on bevy#10871.

### 5.3 What is requested, when

| Moment | Specimens | Holds |
|---|---|---|
| **Setup screen / first launch after a bake or driver change** | the whole **catalog** (§7) × the current preset's view keys | setup screen progress row (bake.md); budget slot 20 s |
| **Game start (`Loading`)** | the catalog again (all `Ok` from the driver cache: fast) or, if over budget (W0 measures), the start area's residency set + globals | `Loading` → `InGame` gate |
| **Door / blackout (fade hold)** | the arrival worldspace's resident set within the reveal radius (all of it, not only camera-visible: cause B) | fade hold gate |
| **Streaming (exteriors)** | each cell's placements when the cell is **loaded** (ring R1 load radius, before it is drawn) | none: a late one skips its draw and is counted |
| **Actor or effect spawn** | the actor's body parts and equipment, the effect's materials, at spawn (equip changes: at equip) | none during play |

With the catalog warmed at start, the per-load requests are pure hash hits; they exist for content the catalog
missed (mods added since the bake, runtime-generated materials) and to keep the gate honest.

### 5.4 The reveal gate

One readiness contract in the framework, `RevealGate` (`sf_world_bevy`, beside `FadeTransition`), fed by:
- **CPU:** streaming `arrival_ready` / `Ready`, textures within the reveal radius (today's `fade.rs` checks).
- **Assets:** resident entities within the reveal radius whose mesh, material or texture is not prepared (cause C;
  counted in the render world from the pending queues, not from `SpecializedMaterialPipelineCache`).
- **Pipelines:** precache status of the reveal's tag (not ready = queued, compiling or waiting on assets), plus
  `waiting_pipelines()` for non-material pipelines, plus visible draws still skipped (today's `PendingDraws`, extended
  to shadow views).

The fade hold and `Loading` → `InGame` both wait on it. **Not ready is never drawn late on purpose**: the reveal
waits. Caps stay as a safety net (a broken shader must not hang the game): today's `gate_max_secs` 1.5 s becomes a
pipeline cap measured in W0 (proposal: 10 s, same as `max_hold_secs`), and hitting a cap logs a `warn!` with the
pending specimens, never silently. Door crossings show the fade, `Loading` shows its progress bar (pipelines
compiled / requested).

### 5.5 Miss log (recorded fallback, UE bundled cache / Unity `traceCacheMisses`)

Every pipeline created **after a reveal** for a key the catalog did not hold is a miss. Dev builds write each miss's
specimen description (mesh layout signature, material type and key, flags, pass, view kind) to
`<store>/gpu/<adapter key>/misses.ron`; the next warm-up adds them, and `<game> bake` folds them into the catalog
so the pattern gets fixed in the predictor, not just replayed. Release builds keep only the counters.

### 5.6 Settings changes

Applying a setting that changes a view bit: fade to black, swap the view components, request the catalog for the new
key set, hold on the gate, fade in. The in-memory specialiser caches keep the old keys, so cycling back is free within
a session. Whether to precompile the alternates (F4 cycle, other presets) on the setup screen is owner question 4.

### 5.7 Disk and front-end caches (measured, then adopted or dropped)

- **Front end (WESL composition, naga, SPIR-V): parallelise.** Bevy holds the shader-cache lock for composition and
  module creation (`vendor/bevy_render/src/render_resource/pipeline_cache.rs:555-583`), so it runs one shader at a
  time. Patch per bevy#26152 (compose outside the lock, short lock for module creation); also raise the async
  compute pool during `Loading`/setup if W0 shows the 4-thread default (`TaskPoolOptions`) is the limit.
- **Driver back end: `wgpu::PipelineCache`** (Vulkan only; DX12 is a no-op in wgpu 30) by carrying bevy#19809:
  one cache per adapter, loaded at device creation, written (temp + rename) after warm-up and at exit, stored at
  `<store>/gpu/<adapter key>/pipeline-cache.bin` with `wgpu::util::pipeline_cache_key`. Desktop drivers keep their
  own caches, so this pays only on a cold driver cache; W5 measures it cold and warm and drops it if it buys < 10 %.
- **Persisting composed modules** (skip WESL on later launches): only if W0 shows the front end dominates warm
  launches even after parallelising. Not designed here.

## 6. Measurement (log lines and counters only; no screenshots)

Counters through `sf_diag` (`DiagCounters`), log lines through the pipeline watch; the owner runs the scenes, agents
read the logs:

| Counter / line | Meaning |
|---|---|
| `pipelines/created`, `pipelines/waiting`, `pipelines/total` | exist (`sf_diag_bevy::churn`) |
| `pipelines/compile_ms` (histogram), split `front_ms` / `driver_ms` | per pipeline, timed in the vendored `pipeline_cache.rs` (one marker hunk) |
| `pipelines/after_reveal` | pipelines created within 10 s after a reveal; **target 0** |
| `draws/skipped_late` | visible draws skipped for a pipeline after a reveal (UE "too late"); exterior target ≤ 1 % of new visible draws |
| `precache/requested`, `/hit`, `/missed`, `/pending_assets` | UE validation set |
| `warmup: catalog N specimens, M pipelines, T s (front F s, driver D s)` | one line per warm-up |
| `reveal: <kind> held X s (cells a s, assets b s, pipelines c s), capped: <none/which>` | one line per reveal |
| `view key changed: <bits> (cause)` | each view-key flip |

Cold runs: the owner clears the driver cache (NVIDIA: `%LOCALAPPDATA%\NVIDIA\DXCache` and `GLCache`) and our
`gpu/` folder; warm runs keep both. Reported per preset (Low, High), in ms and s.

## 7. Bake interface (bake.md owns the bake; this is what it must provide)

- **Content product `pipeline.catalog`** (an `sf_bake` step, class P, per profile): the distinct specimens of the
  content, as data independent of Bevy versions: (vertex layout signature, material type, material key fields that
  select a variant, entity flags) plus one representative (model path, sub-mesh, material index) to load as the
  specimen. Computed from cooked products (mesh.cook layouts, material traits / classes), cheap (no GPU). Changes
  with content and mods, like other products. The adapter supplies the enumeration (§9).
- **Machine artifact** under `gpu/<adapter key>/` (bake.md reserves it; not an `sf_bake` step: keyed by GPU, driver,
  wgpu/Bevy version, shader set, preset view keys, catalog hash): `pipeline-cache.bin` (§5.7), `warm.stamp`
  (catalog hash + view-key hash last warmed + timings), `misses.ron` (§5.5).
- **Setup screen:** one progress row "Compiling shaders" fed by `Precache::status("catalog")`; it runs after the
  content bake (needs the catalog) and needs a render device (setup.md: the window exists).
- **Game start:** if `warm.stamp` mismatches (driver update, new mods, preset change), the `Loading` screen runs the
  full catalog; otherwise the reduced set (§5.3).

## 8. Slices (each one deliverable + its verify step)

| # | Slice | Verify |
|---|---|---|
| W0 | **Measure** (built 2026-11-11, owner run pending: as built below). `compile_ms` front/driver split (vendored marker hunk), `after_reveal`, `skipped_late`, view-key-change line, asset-pending count; move `PendingDraws` counting from `mw_devtools_bevy::pipeline_watch` to `sf_diag_bevy` (game-agnostic). No behaviour change. | Unit test for the counter logic; owner runs cold + warm: new game Seyda Neen, door into Census and Excise Office, turn 360° inside, F4 cycle, `flight` bench. Report counts and seconds per preset. |
| W1 | **Reveal gate** (built 2026-11-11, owner check RG-1 pending: as built below). `RevealGate` with the asset + shadow-view terms; fade hold and `Loading` → `InGame` use it; caps from W0 with `warn!`. | Headless tests: gate stays closed while a specimen's material is unprepared or its pipeline queued, opens when ready, cap fires with a warning. Owner: door crossing log shows `reveal:` line; visual-check entry "awaiting owner". |
| W2 | **Parallel front end** (bevy#26152 approach, vendored `bevy_shader`/`bevy_render` patch; async-compute threads during load if W0 says so). | `cargo test`; warm-up line seconds before/after on the same catalog (owner run). |
| W3 | **Precache patch PW** (shared key functions, specimen queue, per-view-kind templates, status by tag). | Headless render-world tests: requested specimen ⇒ the same pipeline id the draw path later uses (no second compile); counters hit/miss. |
| W4 | **Catalog**: `pipeline.catalog` bake step (framework) + Morrowind enumeration (§9); setup-screen and `Loading` warm-up; residency and spawn requests (§5.3). | Bake test: catalog deterministic, counts printed; owner cold run: `after_reveal` = 0 after door and 360° turn. |
| W5 | **Disk cache**: carry bevy#19809, store under `gpu/<adapter key>/`; keep only if it pays. | Owner cold/warm timings with and without; decision logged in DECISIONS.md. |
| W6 | **Settings changes and miss log** (§5.5, §5.6). | F4 cycle shows a black hold then no `after_reveal`; a forced miss lands in `misses.ron` and is hit next launch. |

**W0 as built (2026-11-11, measurement only; owner run pending).** Vendored `bevy_render` marker `own: pipeline
compile timing` (`vendor/README.md` row): per pipeline task, `queued` (pool) + `lock_wait` (shader/layout locks),
`front` (composition + module creation under the lock), `driver` (`create_*_pipeline`; on Vulkan that includes naga's
SPIR-V write); opt-in, so stock when nobody records; plus `PrepareNextFrameAssets::len`. `sf_diag_bevy::warmup`
(GPU-free: `PendingDraws` moved from `mw_devtools_bevy`, `Reveal`, `AfterReveal`, `CompileStats`,
`describe_key_change`; 7 exact tests) and `PipelineWatchPlugin` (feature `pipelines`, render world). Counters:
`pipelines/{compile,front,driver,wait}_ms` (per frame), `pipelines/after_reveal`, `draws/skipped_late`. Lines:
`pipelines: compiled N (front, driver, waiting; p50, p95, slowest; ms histogram); session ...` per burst, `view key
changed: <+FLAG -FLAG FIELD a->b> (cause; view)`, `reveal: <kind> held X s (cells, textures, gates; assets W1,
pipelines W1), capped: <none/max_hold/gate>, "<place>"; at reveal: ...` and `reveal: <kind> +10.0 s: after_reveal N
pipelines, skipped_late D draws in F frames (peak, % of visible), unprepared peak U`. Emitters: `fade.rs` (door) and
`mw_devtools_bevy::pipeline_watch` (`OnEnter(Loading)` → `OnEnter(InGame)`; W1 moves it into the gate).
Cause C today = visible meshes with a material but no pipeline + meshes/images retried next frame (materials' own
queues are per type: W1). The fade hold (pipelines at black, 1.5 s gate cap) is unchanged.
**Owner run** (release build or `--features dev`, High then Low preset, cold = delete `%LOCALAPPDATA%\NVIDIA\DXCache`
and `%LOCALAPPDATA%\NVIDIA\GLCache` first, warm = run again): (1) New game into Seyda Neen; (2) door into "Seyda Neen,
Census and Excise Office"; (3) turn 360° inside; (4) F4 through every mode; (5) the `flight` bench. Send back every
line starting `reveal:`, `pipelines: compiled`, `view key changed:`, the `pipelines: +N new` bursts, and the bench's
`pipelines/*` / `draws/*` counter rows.

**W1 as built (2026-11-11; owner check RG-1 pending).** `sf_world_bevy::RevealGate` (`reveal.rs`; pure `step` + the
`RevealCheck` system param clients call each frame of their cover): terms in order cells (client), textures
(`StandardMaterial` images within `radius_m` 48 m, CPU, sticky), assets (`near_unprepared`: resident shown meshes within
the radius whose `RenderMesh` or `PreparedMaterial` is missing, on camera or not, so textures' GPU upload too; plus
visible meshes with no pipeline), pipelines (main draws skipped + shadow/prepass entries of
`SpecializedShadow/PrepassMaterialPipelineCache` queued or compiling + `waiting_pipelines()`), gates (`FadeTransition::hold`
callers, ≤ `gates_max_secs` 1.5 s as before). All clear for `SETTLE_FRAMES` 3; cap `max_secs` 10 s after the cells (textures
included) (`reveal_cap`), the fade's own `max_hold_secs` (`max_hold`); a cap logs `warn!` with the pending terms, counts
and up to 4 mesh names. `progress()` 0..1 (cells, textures, then share of the peak assets/pipelines; monotonic).
Render side: `sf_diag_bevy::PendingDrawsPlugin` (always on; `WorldStreamingPlugin` adds it) counts the probe terms only
while the gate holds (`RevealProbe` in `PendingDraws`, extracted near meshes); `PendingMaterialPlugin<M>` adds a
material type's retry queue to the global `assets` count (vendored `erased_render_asset.rs` read-only `len`).
Clients: the door fade, and `mw_exterior_bevy` `phase.rs` `Loading` → `InGame` (a save's arrival stays the fade's).
The devtools fade hold on pending draws and its `reveal: loading` line are gone (the gate writes both). Loading-screen
bar: not wired (menus session: combine `RevealGate::progress` with the streaming fraction in `loading_screen.rs`).

**W2 as built (2026-11-15; owner timing run pending).** W0's walk data (`benchmarks/runs/b2_on`, balmora_walk 1:
compile 18.9 s = driver 17.5 s + front 1.4 s; queue + lock wait 291 s over the run) says the driver compile on 4
async-compute threads is the burst limit and the serial front end the trickle, so W2 is two parts. **W2a, front end
outside the lock** (vendored `bevy_shader`, marker `own, W2a`; `bevy_render` `pipeline_cache.rs` `stage_modules`):
`ShaderCache::prepare` (hit, or a `ShaderJob` with `Arc` copy-on-write snapshots of the sources) under the shader-cache
lock, `ShaderJob::compile` (WESL, naga, module creation) outside it, `finish` under it again; a module whose shader or
imports changed meanwhile is dropped as stale (per-shader change epoch) and retried; a module already being composed
answers "retry" (no double compile); the layout cache has its own short lock. Side effect: `extract_shaders` (main
thread, extract) no longer waits on a composition. **W2b, compile pool** (`bevy_render` marker `own, W2b`;
`sf_world_bevy::PipelineCompilePoolPlugin`, added by `WorldStreamingPlugin`): a pool of all logical cores but 2 (14 on
the owner's laptop) takes new pipeline tasks while the `RevealGate` holds (fade at black, `Loading`); in play they stay
on Bevy's 4 async threads so a late compile never competes with the frame. `StreamingConfig::pipeline_compile_threads`
(`None` auto, `Some(0)` off = stock); game switch `--env BEVYRROWIND_COMPILE_THREADS=<n>` (0 = off). New counter
`pipelines/lock_ms` (the first lock's wait; `wait_ms` minus it = queued for a thread). Tests: `sf_world_bevy`
`warmup_tests` (7, exact). **Measured** (`census` load, cold + 2 warm per exe, `benchmarks/runs/w2_census`; ledger perf.md):
cold hold 12.9 s (capped at 10 s) -> 8.9 s (pipelines 9.9+ -> 6.3 s, W2b); warm pipelines term 0.95 -> 0.65 s (W2a); late
draws skipped after the reveal 10-17 % -> 2 % of visible; `after_reveal` stays 46 (pipelines first needed after the reveal:
W3/W4 predict them).

**W3 as built (2026-11-15; not measured: W4 wires it).** Reuse check: Bevy 0.20 has no precache API (bevy#10871 open) and
built keys inline per entity, so the vendored `bevy_pbr` patch `own, W3` (`precache.rs`) factors them into shared
functions (`main_pass_mesh_key`, `prepass_mesh_key`, `shadow_mesh_key`, `shadow_view_key`, `material_pipeline_key`) that
the three draw specialisers now call; UE5 PSO precaching is the model (mesh + material, per pass and view kind). Results
land in `SpecializedMeshPipelines<Material/PrepassPipelineSpecializer>` (keyed `(layout, key)`) through the material's own
`base_specialize`/`prepass_specialize`, so a later draw is a hash hit; the per-entity caches (`SpecializedMaterialPipelineCache`
...) map entities to ids and cannot be filled ahead. Specimen = `{ mesh, material, flags }`, flags = the entity bits
(`SpecimenFlags`: shadow caster, lightmapped/bicubic, crossfading range, previous skin/morph; the first-frame variant
without previous data is planned too; not-shadow-receiver is a uniform flag, not a key bit). View kinds: the live cameras'
main and prepass keys (the last ones while no camera is live; a live set replaces them), shadow = directional and
perspective (point face = spot) templates always. Main world: `sf_world_bevy::Precache` (`request(specimens, tag)`,
`status(tag) -> { pending_assets, queued, compiling, ready, failed }`, `release`), `PrecachePlugin` (in `WorldStreamingPlugin`;
inert until a request), counters `precache/requested`, `/hit`, `/missed` (distinct draw pipelines predicted or not), `/pending_assets`.
Tests (`precache_tests`, 7, GPU-free): shared keys = the former inline code bit for bit over 160 materials x 48 entity bit sets
x 3 meshes x 2/8/2 view keys; plan = the draw keys per view kind; `precache_specimens` in a bare world with a `(layout, key)`
stand-in specialiser: the draw's id is precached, no new entry; hit/miss and tag status. Needs a GPU (not tested): the real
`PipelineCache`, a full render app's views. Open: draw-function presence and views first seen after a resolve (W6) are not rechecked.

**W4a as built (2026-11-15; catalog only, not wired: W4b requests it).** Reuse check: Bevy 0.20 has nothing (no content-side
pipeline list, bevy#10871), nor any crate; the model is UE5's bundled PSO cache / PSO precaching (cook-time (vertex factory,
material, pass) tuples) and Unity's `ShaderVariantCollection`. Framework: `sf_model::catalog` (Bevy-free), step
`pipeline.catalog` v1 (`CATALOG_FORMAT`), subject `all`, postcard `Catalog { entries, builtins, stats }`, read back by
`load_catalog(store, manifest)`. Entry = `VertexLayout` (uvs, colours, tangents, skinned, morphs; position + normal always) x
`MaterialSignature` (type + named fields, the adapter's reading) x `EntityFlags` (shadow caster, crossfade) + one
`Representative` (mesh step, model, lod, submesh, material index: first in sorted model order, every LOD level read) + use count.
Adapter hook `CatalogSource` (fingerprint, models as (step, subject, flags), material reader built per run through `RunCx`,
builtins). Cooks are read as the run goes, not planned: a failing model is counted (`stats.failed`), not fatal. Morrowind:
`mw_bake::catalog` (`mesh.cook` of every referenced model + `mesh.rig` creature bodies; actor flags for body parts, creatures,
NPC meshes; signature = runtime alpha incl. resolved cutout candidates and additive, two-sided, unlit, foliage, normal/depth
companion from `texture.cook.index`), wired into `bevyrrowind bake` after the texture cook (prints a `pipeline.catalog:` line).
Tests: sf_model `catalog::tests` (bake twice in any model order = byte-identical; shared class + layout = one entry and
representative; skinned rig = its own entry; synthetic: 3 specimens, 1 skinned) + 3 mw_bake signature tests. Follow-ups (W4b):
models `mesh.cook` skips (runtime NIF path: counted as `skipped`), particles (`ParticleExt`), far-field proxies, builtins by name
only (terrain, water, sky, precipitation: W4b maps them); mesh compression and A/B switches are applied by loading the representative.

**W4b1 as built (2026-11-15; not measured yet).** `sf_world_bevy::CatalogWarmup` (`catalog_warm.rs`; `WorldStreamingPlugin` adds it,
idle until `set_catalog`): started by `start()` or any reveal hold, it loads each representative (`baked://<step>/<model>.<mesh_ext>`
or a `loader`), takes its submesh and material through streaming's own cache and fold (`stream.rs` `cached_material`,
`Streaming::template_material`: a later placement binds the same handle) or, skinned, `skinned_materials` (the character mapping),
flags from the entry (`entry_flags`: shadow caster, crossfade = a non-abrupt range, previous skin/morph), waits for its textures, then
`Precache::request(.., "catalog")`; handles stay alive. Failed loads/textures are counted and logged once, never waited for. Builtins:
`BuiltinSpecimens` (name -> builder: `Ready`, `NotYet` retried while a reveal holds, `Skip`; unknown = skip + `warn!`). Gate:
`RevealInputs::catalog` = `CatalogWarmup::pending` (representatives loading + the tag's assets/queued/compiling) joins the pipelines
term (cap warning names `catalog N`); `Precache` now counts specimens in flight until the render world reports (a read between extract
and report looked settled). Line `warmup: catalog N specimens (B builtin specimens), M pipelines (specimens resolved), T s`. Game
(`mw_exterior_bevy` `catalog_warm.rs`): catalog loaded on `OnEnter(Loading)`; terrain, water, precipitation = their systems' live
entities; sky skipped (view pipelines, held by `waiting_pipelines()`); `mesh.rig` registered as a baked extension. Tests: `catalog_warm_tests` (3).

**W4b1-fix as built (2026-11-15; measured: census warm hold 7.56 s (base 3.6, W4b1 4.4-5.0), after_reveal 2 (base 46, W4b1 23), skipped_late 0 %; cold capped 14.1 s with 0 visible draws skipped; ledger perf.md; next: throttle only while a draw-path pipeline waits).** W4b1's census run: cold, the catalog's ~250 pipelines queued ahead of the
visible draws' (cap hit); warm, 23 `after_reveal` from `+GI_PROBES` 0.38 s after the reveal. **Low priority** (UE5 PSO precache):
`PipelinePrecache::set_limit` (patch `own, W3`) keeps at most K precache pipelines queued or compiling (a plan longer than K goes
alone), resolving one specimen at a time and counting only what it really queued; held-back specimens stay `waiting` (= queued in the
tag status, so the gate's catalog term holds on them); draws are never held. The pipeline cache spawns every queued task in one
`process_queue` (FIFO pools), so holding back at submission suffices; `pipeline_cache.rs` untouched. K = `precache_limit`: half the
compile threads while a reveal holds (W2b pool, else async-compute), 1 in play; counters `precache/throttled`, `/outstanding`.
**GI before the reveal:** `sf_world_bevy::RevealHolds` (named readiness sources, pipelines term, 10 s cap, warning `holds N; holds: gi`) fed by `sf_gi_bevy::GiReadiness` (a volume that will trace is not shown yet, +2 tail frames; clear without GPU support, GI off, pipeline error) via `mw_exterior_bevy` `gi_volume.rs` (replaces its fade-only 1.5 s `FadeTransition::hold`). Tests: `precache_tests` throttle, `reveal_tests` holds, `sf_gi_bevy` 2.

**W4b1-throughput as built (2026-11-16; measured: census warm hold 6.3 s vs W4b1-fix 6.6 s (noise), first warm run after a cold one 7.1-8.3 s vs 12.2-12.4 s; cold capped in both; no-catalog base on the same setup warm 2.9 s, cold 8.1 s; ledger perf.md).** UE5 caps precache PSOs only against PSOs a draw needs, so the limit is two: `PrecacheLimit { idle, draw_waiting }` (`PipelinePrecache::set_limits`, `Precache::set_limits`; `compile_pool.rs` `precache_limit`: all compile threads while a reveal holds, half of them while a draw-path pipeline not of the precache's own is queued or compiling, 1/1 in play; never unlimited, so a later draw waits at most one task). A draw counts as waiting from its first draw while its pipeline is in progress (`note_draw_with`, pruned each frame). Admission per pipeline, not per specimen (a specimen may resolve over several frames and stays queued in its tag's status); a hash hit or an id already outstanding takes no slot; a `Creating` task that finished frees its slot before `process_queue` stores it. `precache_specimens_systems()` orders the precache after `specialize_material_meshes` and `specialize_shadows` (the prepass specialiser runs earlier, in `PrepareMeshes`), so it sees the frame's draws; misses settle at the end of the step (a pipeline predicted in the same frame is a hit) and log once each: `precache missed: <Pass> pipeline <id> mesh key 0x.. <flags> material key 0x.. (waiting N); nearest predicted differs by ..` (`waiting 0` = a true miss; shadow misses `key unknown`), cap 256 lines. Counters `precache/draw_waiting`, `/limit`. The last 2 misses were a skinned mesh with morph targets drawn with previous skin but no previous morph: `plan_specimen` now plans every subset of a specimen's previous-frame bits (census warm after_reveal 0/0/0; the misses left come from a view key flipping during or after the hold: W6). **Where the warm hold goes now:** the catalog makes ~250 pipelines where the view uses ~107, and the shader front end (composition + naga, ~45 ms each) is 11-14 s summed over threads against 3.7 s, so the throttle is no longer the limit; the next step is not holding the reveal on the whole catalog (UE5 waits for the shown components' PSOs and leaves the rest of the precache to the background) and a front-end module cache. Measuring: each exe on its own empty GLCache, cold then warm x4 (the NVIDIA cache fills over 3-4 runs); the shared live cache (~1 GB) evicts and is not a warm baseline. Tests: `precache_tests` +7 (limits, hits free, finished tasks free, same-frame misses) and the subset plan.

**W4b3 as built (2026-11-16; not measured yet).** The reveal waits only for the shown set; the catalog compiles in the background (UE5 PSO precaching: the loading screen waits for the "required" PSOs of the components it shows, the bundled precache keeps going at low priority). **Shown set** (`sf_world_bevy` `resident_warm.rs`, `ResidentWarmup`, tag `RESIDENT_TAG` = "resident"): while a reveal holds, every frame (`PostUpdate`, after transform propagation, `ResidentScan`), each mesh entity within the reveal radius (`ResidentWarmup::radius_m`, default `RevealGate::radius_m` 48 m) of the hold's focus (`RevealGate::focus`, set by `RevealCheck`), not hidden (`InheritedVisibility`), camera-visible or not, is requested once per distinct (mesh, material, flags) from its real components (`specimen_flags`: `NotShadowCaster`, `VisibilityRange`, `Lightmap`, `SkinnedMesh`/`MeshMorphWeights` as previous skin/morph, so the subset plan covers them); `StandardMaterial` textures still loading wait (counted), a failed one skips the specimen (`catalog_warm.rs` `textures`). Materials: `StandardMaterial` by the plugin, others via `add_resident_material::<M>` (the game adds terrain, water, precipitation: `mw_exterior_bevy` `catalog_warm.rs` `add`). A new hold releases the last one's tag (`Precache::release` now also clears its last status), so a capped reveal's leftovers keep compiling in play until the next hold. It scans during the hold rather than at cell load (§5.3's streaming row): the arrival cells all spawn during the hold, the scan is cheap, and it needs no hook in streaming; GPU-batched statics (no `Mesh3d`) are held by the draw terms (their batch entities draw in every view). **Gate** (`reveal.rs`): `RevealInputs::resident` (`ResidentWarmup::pending`: textures waiting + the tag's assets, queued, compiling) replaces the catalog in the pipelines term (`RevealPending::of(sample, resident, holds)`); `RevealInputs::catalog` is information only (cap warning `...; catalog N (background)`; line `warmup: reveal <kind> "<place>": resident N specimens (P pending), catalog C pending (background)` at each reveal). **Background** (vendored `bevy_pbr` `precache.rs`, patch `own, W3`): `PipelinePrecache::set_background(tag)` (main world `Precache::set_background`, the catalog's tag) plans and admits a background tag's specimens after every other tag's (request order within each class, `precache_specimens_with` `order`), so under the limit the shown set goes first even when requested later (W4b1's lesson: 498 visible draws skipped behind the catalog). The gate's `waiting_pipelines()` term counts the catalog's in-flight compiles too, so when only compiles are left (`RevealInputs::drains_background`: cells, textures, assets, visible and shadow draws, the shown set and the holds all clear) the gate pauses background admission (`pause_background`; the ones in flight finish, about one round of the pool) and unpauses at the reveal or when no hold runs (`end_reveal_frame`); this avoids subtracting the catalog's share from a render-world count sampled at another point of the frame. After the reveal the catalog runs at the play limit (1/1, `precache_limit`), behind any shown-set leftovers. Line `warmup: catalog N specimens (B builtins), M pipelines, T s (before the reveal | +R s after the reveal)`. Tests: `reveal_tests` +2 (catalog never holds, shown set does, drain conditions; a client pauses only to drain and unpauses at the reveal), `precache_tests` +2 (shown set admitted ahead of the background catalog under a limit, paused admits nothing; main-world background tags and release), `catalog_warm_tests` (the gate does not wait for the catalog; the shown set requested with its flags, deduplicated, near and shown only, holds the reveal, released by the next hold).

W0 first: it decides whether the front end (W2) or prediction (W3/W4) is the bigger win, and the caps. W1 is useful
alone (it fixes the "opens early" cases of ROADMAP 1.1). W3 needs the static scene's batch path accounted for:
`GpuBatchedMesh3d` batches specialise through the same material path (patch GS), so specimens cover them; coordinate
with `scene-coord` before W3.

## 9. Adapter: Morrowind (`mw_assets`, `mw_bake`)

Enumerates specimens for the catalog: every distinct (`CookedMaterials` material after `matclass` folding, cooked
mesh vertex layout: position/normal/uv/colour/tangent, skinned joints/weights) over the profile's models; body parts
and equipment (skinned layouts × their materials); particle systems (`ParticleExt`); terrain, water, sky, precipitation
and far-field materials (one each per variant). Morrowind specifics (NIF flags, which controllers make a mesh
non-static) stay here; the framework sees only specimens. A game-side spec (`bevyrrowind/docs/specs/`) is written
with W4 if the enumeration needs more than this section.

## 10. Risks

- **Key divergence** between precache and draw path: prevented by sharing the key functions in the patch; detected by
  `precache/missed` and `after_reveal`.
- **Patch upkeep:** PW touches `bevy_pbr` specialisation, which also carries GS (#25978) and our view-bit patches;
  rebase cost on each Bevy bump. Upstream offer on #10871 is the exit.
- **Budget:** catalog × view keys may exceed 20 s cold; then the setup screen warms the default preset only and the
  `Loading` screen warms the start area (W0 decides).
- **Driver cache eviction** (UE notes NVIDIA reloading precached PSOs costs ms): measured by `after_reveal` on long
  sessions.

## 11. Open questions for the owner

1. Go for the slices (W0 first)? W0 and W1 change no visuals beyond longer holds.
2. Hold caps: wait up to 10 s at a door / load for pipelines (proposal), with a progress indicator on long loads?
3. Late pipeline during exterior play: skip the draw (Bevy stock, UE default, proposal) or draw with a stand-in
   material until ready (no pop-out, but a visible material swap)?
4. Settings changes: re-warm behind a black hold on Apply (proposal), and should the setup screen also precompile the
   other presets / F4 modes (longer first launch, instant switching)?
5. Backend fact to confirm from the startup log: Vulkan (DLSS requires it) vs DX12 (`gs-patch-plan.md:379` says
   "our shipping backend"). The disk cache (W5) exists only on Vulkan.

Decided 2026-11-11 by industry standard: (1) go, W0 first (owner asked for this pipeline). (2) 10 s cap with a progress
indicator, `warn!` when hit. (3) Skip the draw (UE default). (4) Re-warm behind a short black hold on Apply; the setup
screen warms the current settings only (UE precaches what is in use), others warm on change. (5) Vulkan is the shipping
backend: `sf_dlss` drives NGX through its Vulkan API; `gs-patch-plan.md:379` is to be corrected by its owner (scene-coord).
