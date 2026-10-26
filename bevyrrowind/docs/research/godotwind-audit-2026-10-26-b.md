# Godotwind audit, half B (2026-10-26): world, data pipeline, dialogue/UI/weather

Scope: streaming/world, ESM/NIF/bake, dialogue/UI/console/weather, EXPERIMENTS + docs. Source: `D:\Gamedev\Godotwind\godotwind`.
Method: README/AGENTS/EXPERIMENTS, `docs/audit`, `docs/plans`, `docs/systems`, file headers; code read only where the headline mattered.
Overall: Godotwind is mostly a source of **lessons and test ideas**, not logic to copy. Its own audits call its streaming
architecture the problem (GDScript main-thread assembly, advisory budgets, HLOD that crashes). Its ESM/NIF/dialogue
code is a plain port of OpenMW structure; we already have Rust equivalents and a full script interpreter (`mw_script`).
Constants below are "starting point to measure" only.

## 1. Streaming, cell loading, far field, door travel (ROADMAP 1.1, 1.2, 1.8, 1.9)

| What it does | Godotwind file(s) | Our counterpart | Value | Effort | Note |
|---|---|---|---|---|---|
| Perf finding: standing still 142 fps, moving 57 fps; render is cheap, **assembly/teardown on cell crossing is the cost**; draw calls were not the bottleneck (halving draws moved fps 34.2 to 34.4) | `docs/audit/distant_rendering_perf_audit_2026_07_05.md` | `perf.md` "Frame budget", ROADMAP 1.2 | high | S | Check our A/B separates "standing" from "moving" cost; if not, add that split (render_cpu/gpu vs per-frame streaming ms) |
| MultiMesh/bucket batching premise fails on Morrowind: 47 cells gave 6,822 instances in 1,733 buckets (2.3 per group) | same, F5 | `precombine` spec, ROADMAP 1.2 "merge statics per cell at bake time" | high | S | Run the same census on our content before building more instancing; it supports bake-time merge by material, not runtime buckets |
| Runtime merge without simplify (their "HLOD") made fps worse (57 to 44), stalled 861 ms, segfaulted on unload churn. Fix direction: offline simplify + atlas | `object_paging.gd`, `NativeObjectPagingKernel.cs`, audit F2 | `sf_lod`, `sf_lod_bevy` (impostors), `precombine` | high | S | Confirms "bake offline, publish 1 instance per chunk"; do not add runtime merging |
| Budgets are advisory when work items are 10-250 ms; time is checked only between items | audit F3, `streaming_publication_budget.gd` | ROADMAP 1.9 render-thread spikes, J7 warm-up | high | M | Make work items fine-grained (per N refs) and assert worst single-item ms in a test, not just the average |
| Per-cell prebaked "first playable" manifest (3x3 start refs grouped by model, static descriptors) plus lazy per-cell ref pages instead of expanding every ref at startup | `docs/plans/loading_speedup_2026_06_18.md` P0-P4, `NativeCellManifest.cs`, `world_cell_manifest.gd` | `mw_bake`, `mw_exterior_bevy` | med | M | Check our boot: do we expand all refs of all cells? Main-menu-before-world split is already ours (menu screen exists) |
| Velocity-extrapolated two-phase preload: data warm (async, no scene nodes) ahead of the camera, instantiate only on entry | `cell_preloader.gd` | `mw_exterior_bevy` (one file mentions preload) | med | M | Also preload door destinations and fast-travel targets (OpenMW does); feeds 1.1 "interior popping in after fade" |
| Door-travel readiness split: `data_ready` / `visual_ready` / `transition_ready`; commit only after destination floor collision, spawn clearance and exit door exist; noncritical clutter streams after reveal | audit `interior_exterior_transition_audit_2026_05_30.md` #9 | `mw_doors_bevy` (fade hold with gates for shadows, pipelines) | high | S | Our gates cover render readiness; add collision/exit-door/spawn-clearance gate (owner ask 2026-10-25) |
| After a teleport (interior exit, fast travel) force a full desired-cell reconciliation; frozen-tracking unfreeze that only updates "current cell" skips the refresh (stale exterior) | audit #10 | `mw_exterior_bevy` | high | S | Write a regression test: exit door then assert the 3x3 around the exit is requested next frame |
| Fade as an explicit state machine (black stays black through commit; one reveal path); proximity-deferred door requeue must run per frame, not only on cell change; physics-layer rewrite must not erase query-only interaction areas | audit #1, #2, #11 | `mw_doors_bevy` | med | S | Pure test ideas; cheap to mirror as unit tests (listed in audit "Tests") |
| Interior as an offset "pocket" in the same world with layer masks; stencil portal through the door | `interior_pocket_manager.gd` (2,821 lines), `door_portal.gd` | `sf_world`, `mw_world` interior path | low | L | Their pocket code is the largest bug source in their audits; do not copy. Seamless doorway view only if the owner asks |
| Distant lights: real omni lights 150-600 m budgeted nearest-first, plus billboard glow sprites to view distance | `distant_light_manager.gd` | `sf_lod_bevy`, `sf_render_order_bevy` (mentions) | med | M | Idea matches OpenMW MR 5212; Phase 4. Bevy clustered lights make a few hundred unshadowed omnis cheap, measure first |
| Terrain horizon-map self-shadow bake (8 directions, 2 RGBA8) | `horizon_map_baker.gd`, `NativeHorizonMapBaker.cs` | none (we use shadow cascades) | low | M | Parked (owner: simpler shadows first) |
| Shadow budget by distance with hysteresis (cube < 10 m, paraboloid < 30 m) | `light_shadow_budget.gd` | ROADMAP shadow decisions | low | S | Hysteresis idea only; tiers are theirs |

## 2. Records, NIF, textures, bake pipeline

| What it does | Godotwind file(s) | Our counterpart | Value | Effort | Note |
|---|---|---|---|---|---|
| NIF parse failures: 64 of 4,948 models; cause was property readers under-reading 4 bytes, giving "string length 0x53694E00" (= `\0NiS`) out-of-sync signature | `docs/reference/nif_unsupported.md`, `nif_reader.gd` | `mw_nif` (`census.rs`) | high | S | Run our census on the same list (glass/ebony armor, `active_port_*`, `magic_*`, Telvanni organic) and add the `\0NiS` sync check as a parser self-test. Suspects: NiStringExtraData, NiTextKeyExtraData, NiVertexColorProperty, NiZBuffer/NiSpecularProperty, particle controllers, NiBSAnimationNode |
| 28 "bake failures" were particle-only NIFs with zero meshes (`e\magic_*`, `vfx_pattern*`) | same | `mw_bake` | med | S | Make sure our bake reports "no geometry" as a class, not a failure; they must still reach the VFX path |
| NIF statistics from a full bake: texture slots (base 100%, dark 0.1%, glow 0.1%, others 0), apply modes (MODULATE 99%, HILIGHT 283), controller counts (keyframe 5,954; particle 665; morph 318; vis 309; alpha 183; UV 98; path 56; material color 10) | `docs/systems/nif_pipeline.md` | `mw_nif` `census.rs`, `material.rs` | med | S | Cross-check our census counts; any mismatch is a missed record type |
| Collision sidecar per prototype (`.shapes.res`) so first sight of a model costs sub-ms; AABB/shape memo per prototype for doors (10.7 ms cost) | `static_shape_pack.gd`, `interaction_shape_cache.gd`, `collision_shape_library.gd` | `sf_physics`, `mw_bake` | med | S | Verify our colliders are baked per model, not rebuilt per instance |
| NIF without RootCollisionNode/bhk falls back to trimesh (open item "Part 2") | `docs/plans/nif_collision_part2.md` | `mw_physics` | med | S | Check our rule for collision-less statics (stairs, ramps) |
| ESM: DELE sub-record handling, FRMR/MVRF moved refs, deleted INFO filtering | `core/esm/records/*`, `esm_manager.gd` | `mw_records` | low | S | Plain port; nothing beyond OpenMW. Skim only if a deleted-ref bug appears |
| Interior GI via prebaked VoxelGI (since Godot cannot script lightmap bakes) | `interior_gi_cache.gd`, `interior_gi_bake_runner.gd` | none | low | L | Godot-specific; our GI path is a separate decision (Solari/probes) |
| Prebake orchestration: targeted rebake, headless full rebake, progress state | `src/tools/prebaking/*` | `mw_bake`, `sf_bake` | low | S | Targeted rebake by asset list is a nice dev tool; check we have one |

## 3. Dialogue, UI, console, weather (Phase 2)

| What it does | Godotwind file(s) | Our counterpart | Value | Effort | Note |
|---|---|---|---|---|---|
| Dialogue filter in 4 steps (actor, player, select structs, disposition), INFO pre-filter dropping 90% of topics, 74 function IDs, context greetings (naked, sneaking, diseased) | `mw_dialogue_filter.gd`, `mw_dialogue_functions.gd`, `dialogue.md` | `mw_rpg`, `mw_script` (functions.txt) | low | S | Same OpenMW port we have; use only as a cross-check list of the 74 functions |
| Journal index = INFO "disposition" field (non-contiguous 5, 10, 15...); QSTF marks finish even with higher indices (alternate endings) | `mw_quest_adapter.gd`, `dialogue.md` | `mw_rpg` quests | med | S | Add a test with an alternate-ending quest |
| Typed `Response` envelope (error enum, topics discovered, disposition change, quest metadata) so UI never sees bare nulls | `dialogue_provider.gd` | `sf_rpg_ui` | low | S | API shape idea for the dialogue screen |
| Book/note HTML-like markup to rich text (BR, P, DIV ALIGN, FONT COLOR/SIZE, IMG SRC, B, I, HR) with injected image loader | `text_formatter.gd`, `mw_text_formatter.gd`, `book_viewer.gd` | none found in Rust (grep) | med | M | Needed for books/scrolls in Phase 2; the tag list is a ready checklist (read OpenMW book parsing for edge cases) |
| Console: command registry, click-to-pick object by AABB without physics, hover tooltip, selection outline | `command_registry.gd`, `object_picker.gd` | `sf_console`, `sf_console_bevy` | low | S | AABB-pick for console "prid" selection is a cheap feature if we lack it |
| Weather: 4-phase time-of-day colour blend with per-category transition windows; pure/stateless interpolator | `weather_interpolator.gd` | `sf_weather`, `mw_weather` | low | S | Pure-function split is already our shape |
| Benchmark framework: subsystem toggles (`isolate`, A/B), scripted 85 s flyby, ladder (additive rungs), render_cpu/gpu split | `docs/systems/benchmarking.md`, `subsystem_toggles.gd` | `sf_diag`, `perf.md` `--off` flags | med | S | Their audit found 6 duplicate harnesses and 5 dead all-zero result files; keep one ladder + one A/B. Hygiene: clean tree, note fps cap, repeat runs (same-commit variance was 31.7 vs 75.6 fps) |

## Not worth taking
Pocket/portal interiors, MultiMesh buckets, runtime HLOD, VoxelGI bakes, Godot ocean/shader notes (other half). Their constants
(0.14/0.003 min-size, tiers, budgets): the audit itself found them imported wrongly from OpenMW (real default 0.01, unit-free).

## Top 5 to take
1. Door-travel readiness split plus forced streaming refresh after teleport, with the audit's 9 regression tests: ROADMAP 1.1 (stability, owner's "popping after fade").
2. NIF `\0NiS` out-of-sync self-test and run our census against their 64-file failure list; particle-only NIFs as a class: ROADMAP 1.8/Phase 2 asset correctness (mw_nif).
3. Streaming perf method: split standing vs moving cost; assert worst single work-item ms, not average; instance-per-group census to justify bake-time merge: ROADMAP 1.2, 1.9 (precombine).
4. Lazy per-cell refs and a first-playable manifest (they measured 4.4 s of eager record expansion and 30 s to first playable): ROADMAP 1.2 / loading.
5. Book/scroll markup renderer with the tag checklist and alternate-ending journal test: Phase 2 dialogue/UI.
