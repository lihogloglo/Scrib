# Roadmap

Task-level plan and live status. The *why* lives in
[`../superfargoth/docs/ARCHITECTURE.md`](../../superfargoth/docs/ARCHITECTURE.md)
(milestones in §9, decisions in §10). This file is the *what's next*.

**Every session:** read this first; when you stop, update the status marks,
the "Next up" section, and the session log at the bottom. A task is done
only when its "Verify" step was actually run.

Legend: `[x]` done and verified, `[~]` in progress or partially done,
`[ ]` not started. Crates: `sf_*` = superfargoth, `mw_*` = bevyrrowind.

## Next up

1. [~] M2: streaming spikes (2026-10-12; streaming.md "Spikes", diag.md "Frame pacing"). **Gate settled
   (spikes4):** at most 1 presented stutter a minute (frame > 1.5× the moving median of 60) and none over
   33 ms, run uncapped in a borderless window at the display's resolution (`benchmarks/exterior_{run,flight}.json`,
   perf.md "Frame pacing"). It fails today: borderless 1080p flight 21–27, run 3–9 stutters/min (~118 fps);
   at 1440p the GPU-bound waits hide most of them, fifo at 60 Hz all of them. ~90 % coincide with a
   render-thread busy spike (12–15 ms vs ~6 ms median), mostly in frames with no mesh/spawn/shift churn,
   plus ~4 origin shifts per flight. Built earlier: particles paused out of view, terrain moved in place on
   shifts, origin-shift guard, mesh load budget (`max_loading_meshes` 64). Next levers: (a) trace the
   churn-free render-thread spikes (borderless 1080p flight, `profiling-trace`); (b) lazy floating origin
   (separate origin entity, 2-cell hysteresis; shift-guard prediction in `stream.rs` must follow); (c)
   upstream incremental `extract_visibility_ranges`.
2. M2: upstream the two Bevy 0.20 issues the far field hit (owner files them; far-field.md "Bevy 0.20
   pitfalls"). Bevy's upload limiter stays off: in 0.20 it only defers `prepare_assets`, not the
   GPU copy (far-field.md pitfall 1), so upload pacing stays spawn-side in `sf_world_bevy`. The depth prepass is lower priority: depth + MV
   prepass with occlusion culling measured slower in Balmora (13.9 vs 11.9 ms, `view --prepass`),
   and the motion-vector prepass DLSS uses already applies the dither.
3. M2: perf tooling from Ovid's workflow (`../Ovid bevy/docs/development.md` "Profiling",
   `tools/perf_nsys.ps1`), adapted to `sf_diag`, no Python: Nsight Systems per-pass GPU (all wgpu
   pass labels; elevated, one UAC prompt), suite runner (waits for idle cargo/rustc, `--repeat`,
   `nvidia-smi` sampling), hitch table with entity/asset/pipeline churn, CPU-floor run at 960×600,
   A/B switches, `profiling` build. Then idle-machine baselines (Balmora, fly-throughs). Queued
   until the exterior agent releases the app (owner: don't run three builds at once).
4. M2: fly-through calibration: L1 handoff `d1` done (400 m high, 300 m low candidate; rings follow it
   through `far_streaming_config`). Quality presets done (J1, `--quality low|medium|high|ultra`: bias
   0.75/0.875/1/1 so d1 300/350/400/400). Left: re-measuring the gates at 1440p on a 3070-class desktop.
5. M2 (time-boxed, separate jobs; plan in sky-lighting.md open questions 1 and 3): exterior RT content
   (`sf_world_bevy` `ray_tracing_lod` + one-LOD tagging, `NoRayTracing` on `sf_lod_bevy` proxies,
   `sf_terrain_bevy` terrain RT proxy, then TLAS size and frame time in Balmora) and the `bevy_solari`
   sunset patch. Then decide the `--dlss` default (`auto` on NVIDIA?) after judging foliage smear in motion.
6. **P0, M2: industry gap audit jobs** (`../superfargoth/docs/specs/industry-audit.md`, 2026-10-12). Run these
   next to item 1, before items 3–5. One deliverable each:
   - J1 [x] (2026-10-12, sizecull; industry-audit.md "J1 as built and measured") size-based draw distance: histogram of placements by `d_draw` per preset, then
     `LodScreen::draw_distance(r)` (`s_min` 4/3/2/1 px Low→Ultra, `d_floor`), `SpawnCx` caps each chain and
     its range at `d_draw` (skip LOD levels past it), L1-contributor clamp, class exceptions +
     `NO_SIZE_CULL`, presets wire `s_min`/`d_floor`/`d1`/`bias` (closes item 4's preset part), `--off sizecull`.
     Result: culls 3 % of exterior placements at High (8 % Low; vanilla exteriors have little clutter), entities
     unchanged, Balmora CPU floor -0.3 ms, GPU and stutters unchanged; kept on (standard rule). Makes J3 less
     valuable on vanilla; J2 and J6 are the bigger levers. Left: the adapter does not set `NO_SIZE_CULL` yet,
     draw counts unmeasured (Nsight), interiors not measured.
   - J2 [ ] `NoCpuCulling` A/B for streamed statics + far proxies (GPU still culls frustum and
     `VisibilityRange` in every view, cascades included); keep particles on CPU culling; decide the default.
   - J3 [ ] size-tiered spawn residency in `sf_world`: R1 detail tiers keyed on `d_draw`, prefix spawn by
     cutoff radius (pages are radius-sorted), `CellSummary` tier counts, activation split from visuals.
6b. **Queued (owner 2026-10-12): sky pass**, before exterior ray tracing (item 5). One agent per job, max 3 at once:
   - S1 clouds [~] (clouds3 2026-10-13: in game, in the High budget: shadow map 0.04 ms, clouds 0.91 fair / 0.79 cumulus / <= 0.62 overcast+storm GPU ms at 1440p; CL-1..6 awaiting owner; open: crisp billows, anvils, far fog): cloud types across the full range (cumulus, stratocumulus, altostratus/cirrus high layers, cumulonimbus
     storm cells), not two overcasts + one pattern + clear; visible wind motion (Morrowind `Cloud Speed` per weather,
     weather-adapter.md) and slow shape evolution; cloud shadows actually visible on the ground (CL-1, CL-3 FAILED).
     Owner suspects the sun disk draws in front of the clouds: check the sun behind thick cloud (composite order via
     `SkyOrderScaffold` vs. HDR disk not attenuated enough by cloud transmittance + bloom) and whether the sun's direct
     light dims under cloud cover (would also explain the missing shadows).
     Owner, clouds-flight: no jump/smear (CL-6 confirmed), but clouds are far too big (one cloud covers most of the
     island: weather/shape tile scales), static, same patterns; under a cloud edge at noon there is no shadow edge on
     the ground at all (cookie missing or far too diffuse).
   - Lighting job. Night exposure: **owner chose A (2026-10-13)**: clamp auto-exposure's range (a night floor, e.g.
     EV ~3–4 tuned by eye) so night stays dark and placed/carried lights stay physical (no exposure-scaled lights);
     fixes walls blowing out near lamps at night (torchfix open item). Also: torches and other carried lights cast no
     shadows (owner 2026-10-13); budgeted point-light shadows for the nearest few lights is the usual approach.
   - Owner notes 2026-10-13, actors (queue after walkers3): some Balmora guards at night have a wonky walk cycle, other
     NPCs don't (torch layer + weapon stance + heavy armour?); Dagoth Ur's hand effect is bluish particles with no
     light (compare with the original). Owner on uphill strides: people take shorter steps going up than down, but
     don't add edge cases to a locomotion system we may overhaul; note it for the locomotion refactor (slope-aware
     stride from speed + slope, not a special case).
   - Later (owner: not now, but important): object-terrain blending (R-b, M2 item) reads as "bottom of meshes darker",
     not terrain blending: TB-1..3 FAILED. Revisit the technique (terrain albedo/normal carried onto the mesh base).
   - S2 sun disk too small (owner): artistic angular size / glare instead of the physical 0.53°.
   - S3 night sky: Masser + Secunda (Morrowind textures, `[Moons]` sizes, phases, colors), stars fading in at dusk.
   - Then fog (weather.md N1/N2 haze + dust, height/ground fog, froxel fog + god rays) before item 5.
7. **P1, M2: audit follow-ups**, after J1–J3 are measured:
   - J4 [ ] shadow-caster rules (per-cascade cost in Nsight; small casters `NotShadowCaster` per preset).
   - J5 [ ] depth-prepass dither fix as a local `[patch]` + upstream PR, then depth-only prepass and occlusion
     A/B in Balmora, Vivec, Mournhold (item 2's prepass part).
   - J6 [ ] near-cell precombine evaluation (an L0.5 level, Fallout 4 precombine / OpenMW active-grid paging
     style), keep or drop on numbers.
   - J7 [ ] pipeline warm-up from the `pipelines/created` list on the loading screen; disk cache check
     (wgpu `PipelineCache`).
   - J8 [ ] upstream incremental `extract_visibility_ranges` (item 1's lever).
   - J9 [ ] "Industry comparison" section in each existing spec (docs only).

## Backlog (deferred, owner 2026-10-12: "build strong foundations first")

Wanted, but after the M2/M3 foundations. Not scheduled; pick up when the roadmap reaches them.

- Animation lab test scene (gizmo creatures, draggable FBIK targets, slopes/stairs/ledges, warp and
  probe markers, Quaternius skinned mesh): fold into `sf_anim_bevy`'s Verify step when M3 gets there.
- Weather beyond the running clouds job (`weather.md`): fog/ground fog/god rays, wind field (trees,
  grass), precipitation, wetness/puddles. M6 scope unless pulled forward.
- Ghostfence far proxy (weather.md B5: blended far proxies, scriptless activators far-eligible), its
  white look (visual check GF-2), split cook B4.
- Exterior ray tracing content + `bevy_solari` sunset patch (Next up 5).

## M0: Data foundation (done)

- [x] VFS with layered sources, provenance, alternate formats (`sf_vfs`).
  Verify: `cargo test -p sf_vfs --all-features`.
- [x] Record database with dependency-aware conflicts (`sf_records`).
- [x] Mod manifests and load-order resolution (`sf_mods`).
- [x] Install detection, Morrowind.ini (`mw_install`).
- [x] BSA sources and install mounting (`mw_vfs`).
- [x] ESM/ESP loading with Morrowind merge rules (`mw_records`).
  Verify: `cargo run --release -p bevyrrowind -- info` loads
  Morrowind → Tribunal → Bloodmoon, ~62k records.
- [x] Inspection CLI: `detect`, `info`, `find`, `record`, `ls`, `which`, `nif`.

## M1: Interiors render

- [x] NIF → engine-neutral model (`mw_nif`): scene graph, inherited
  properties, engine-space geometry, material dedup, visibility rules.
- [x] Bevy NIF loader producing `WorldAsset` (`mw_assets`), textures via VFS.
- [x] Cell → placements (`mw_world`): base resolution, reference
  transforms, lights, editor markers.
- [x] Viewer with free camera and `--screenshot` capture.
  Verify: `cargo dev view "Seyda Neen, Census and Excise Office" --screenshot shot.png --camera "1.0,4.2,5.0,1.0,3.6,-6.0"`, then look at it.
- [x] Fast-build setup (lld, dynamic linking, profiles). 2–4 s rebuilds.
- [x] NIF sweep: `cargo dev nif-sweep` converts all 7,319 vanilla NIFs, 0 failures; ranks
  unsupported types by files affected. Left (sweep 2026-10-12, particles2): path controllers
  (5, spell effects), embedded textures (2), planar collider (1); geometry morphs and
  `NiBSPArrayController` now converted.
- [x] NIF controllers (spec `docs/specs/nif-runtime.md`): keyframe, visibility, UV, alpha,
  material color; Hermite/TCB/Euler curves in `mw_nif::anim`, auto-play rules from
  animation-node flags. Verify: two `--settle` shots of `"Nchuleftingth, Test of Pattern"`
  (dynamo turns), `view --model x/ex_vivec_waterfall_01.nif` (UV scrolls).
- [x] NIF particles: own CPU simulation (`mw_nif::particles`), camera-facing dynamic meshes
  (bevy_hanabi 0.19 requires Bevy 0.19). NIF emissive now glows through the base texture.
  Verify: Census Office fireplace `--camera "-0.57,4.6,2.0,-0.57,2.6,6.74"`, candle
  `--camera "3.85,4.25,3.1,3.35,4.05,2.43"`.
- [x] Billboard system (`mw_assets::billboard`). Verify: `view --model r/heart_akulakhan.nif`
  from two sides.
- [x] Light attachment at `AttachLight`; flicker/pulse (`mw_world::LightAnimation`).
  Verify: `cargo dev view "Sud" --camera "26.5,-5.0,13.0,31.5,-6.5,5.5"` (torches).
- [x] Lighting calibration: EV100 7, 8000 lm per m² of radius, range 4× radius, ambient 300
  nits (`LightCalibration`; `view --lumens/--range/--ambient` to retune).
- [x] Model root transforms dropped (engine semantics): fixes mis-rotated models such as
  `in_nord_fireplace_01`.
- [x] Benchmark harness `sf_diag`: frame time, draw counts, load time → CSV/JSON; Census Office baseline.
  `view --bench benchmarks/census_office.json [--bench-update-baseline]`. Baseline (RTX 4060 Laptop,
  720p): median 3.46 ms (289 fps), p99 5.16 ms, worst 13.95 ms; CPU-bound (GPU ~1 ms). Re-record
  on an idle machine (other builds were running).
- [x] Particles: `NiBSPArrayController` array emitters (vanilla: random node of the emitter
  subtree, on its bone; no file emits at vertices), world-space particles, live (animated) emitters
  for actors and placed models, emission windows on the controller clock (nif-runtime.md "Array
  emitters, live emitters, emission clocks"); actor windows on the animation timeline (`emit:` clip
  curves, particles3). Checks PA-1, PA-2, PA-3 awaiting owner.

## M2: Exteriors at scale

The WISHLIST "Early decisions" are settled in ARCHITECTURE.md §10 (2026-10-11).
Specs: `../superfargoth/docs/specs/`. Tasks are in dependency order; each names its spec.

Specs (agreed 2026-10-11; owner's priorities: runtime speed and visuals first, bake once in ≤ 2 min):

- [x] `world-coordinates.md`: big_space 0.14, `GRID_CELL` 256 m, `WorldPos` in `sf_coords`.
- [x] `streaming.md`: `Worldspace`/`DoorLink` records, world index, rings R0–R3, budgets, `WorldSource`.
- [x] `terrain.md`: height stack, CDLOD over height clipmaps, splatting, upsampling, detail displacement.
- [x] `bake.md`: `sf_bake` step graph and cache, `sf_model`/`.sfmesh`, cooked meshes + LODs (meshopt 0.6.2).

Framework (superfargoth):

- [x] `sf_coords`: `WorldPos`, `WorldTransform`, partitions, serde forms (world-coordinates).
  `WorldspaceId`/`Location` moved to `sf_world`. Verify: `cargo test -p sf_coords` (9 tests).
- [x] `sf_model` + `.sfmesh` container; `sf_model_bevy` loader (Bevy mesh compression on) and
  `baked://` source (bake). Done: round-trip tests, headless Bevy app loads a cooked model via
  `baked://`, `CookedModel::to_obj` debug export (OBJ, all LODs side by side).
  Verify: `cargo test -p sf_model -p sf_model_bevy` (14 + 3 tests).
- [x] `sf_bake` core: steps, fingerprints, traces, store, profile manifests, gc (bake).
  Verify: `cargo test -p sf_bake` (13 tests: invalidation matrix, 1 vs 8 threads identical).
- [x] `model.info` and `mesh.cook` steps: cooked static models + LOD chain (bake). Real install:
  5,873 of 6,619 models cooked (94% of placed references), 3,971 of 4,079 models ≥ 128 tris get a
  chain (LOD1 61%, LOD4 11% of LOD0 triangles). Viewer `--lod-tint` screenshot checked.
  Verify: `bevyrrowind bake` report; `cargo dev view ... --lod-tint`.
- [x] glTF `ModelImporter` in the framework (modern formats win, ARCHITECTURE §6.1): `.glb` via the
  `gltf` crate (same version as `bevy_gltf`), `sf_model` feature `gltf`. Verify: `cargo test -p sf_model`.
- [~] `sf_world`: records, `Placement`, `WorldSource`, index, overlay, scheduler (streaming). Core done
  (29 tests; index of 100k placements in ~0.2 s, tick ~45 µs). Missing: `world.*` bake steps.
  Verify: `cargo test -p sf_world` (determinism, hysteresis, budgets, one cell per placement).
- [~] `sf_world_bevy`: big_space glue, one `BigSpace` per worldspace, budgeted spawn/despawn,
  anchors, door prefetch, `FadeTransition` scaffold, ring debug overlay.
  Verify: precision scene at 500 km (no jitter), overlay screenshot. Done on a synthetic world (example
  `stream`: overlay, 1 mm fins identical at 500 km, 7 m/s run light). Left: the 60 m/s run, real content.
- [~] `sf_terrain`: tiles, upsample + constrain, pyramid, layers, `TerrainHeights`, edits (terrain). Core
  done (15 tests; synthetic 6 km world in 1.2 s on 16 threads). Missing: `terrain.*` bake steps (in progress).
  Verify: `cargo test -p sf_terrain` (passes through source samples, seams bitwise equal, footprints).
- [~] `sf_terrain_bevy`: height/layer clipmaps, CDLOD, terrain material with surface-state hook,
  macro albedo, RT proxies. Colliders wait for avian on Bevy 0.20 (M4).
  Verify: LOD-colored and wireframe fly-over checked for cracks; summit/coast/valley screenshots, raster and Solari.
  Runs on a synthetic 16 km world (bakes in 9.9 s, ~1 ms GPU, example `terrain`). Runs on Morrowind's
  LAND (2026-10-11): LOD colors + wireframe over Balmora, no cracks; selection 0.24 ms in a town (target
  0.2), one 16.8 ms update after an origin shift at 60 m/s. Left: Solari (RT proxies not wired in exteriors).
- [x] `sf_lod` (far-field.md): `lod.policy` (+ texture-average albedo via adapter `TextureDecoder`),
  `lod.impostor` (CPU octahedral capture, textured alpha), `lod.l1`–`l3`, `lod.dir`; lead-band ranges + no-hole
  fallback as pure functions. Verified: `cargo test -p sf_lod --all-features` (14: no holes on scripted paths
  incl. lagging loads, error ≤ budget, 1 vs 8 threads identical bytes, octa round trips); Morrowind bake 21.9 s
  total (far 20.6 s): 1,621 L1 / 25 L2 / 4 L3 nodes, L1 4.16M tris, 121 impostor models.
- [x] `sf_lod_bevy`: plugin, `.sfnode` loader, `ImpostorMaterial`, light sprites, residency fallback, `far/*`
  counters; swaps are Bevy `VisibilityRange` lead bands (kept: double draw 0.26 ms GPU at 1440p) with a proxy
  depth offset; streaming hook (`FarProxies`) in `sf_world_bevy`. Verified on Morrowind (far-field.md,
  "Measured on Morrowind"; AC, idle, RTX 4060 Laptop): far CPU 0.14–0.17 ms (≤ 0.3), far entities 2.3–3.5k
  (≤ 5,000), far GPU ~1.2 ms at 1440p; town entities 353k → 48k; Balmora 1440p 22.2 → 11.9 ms; `d1` 400 m
  calibrated (300 m = low preset). `cargo test -p sf_lod_bevy --all-features` (depth-offset units).
- [x] Far field for Morrowind: `mw_bake::far` (flora ≥ 3 m → impostors, LIGH → sprites, DDS/TGA decode) in
  `bevyrrowind bake`; app wiring (`--far-tint`, `--off farfield`). Verify: screenshots in
  `D:/Gamedev/Bevyrrowind/shots/farfield/` (Balmora, Red Mountain view without the old see-through gaps, summit
  toward Vivec by day and with the night skyline lights, Ebonheart at night, forest impostor handoff).
  Two Bevy 0.20 pitfalls fixed on the way: Bevy's upload limiter off (objects vanished at random) and no depth
  prepass in exteriors (holes at every crossfade); far-field.md "Bevy 0.20 pitfalls".
- [~] `sf_diag` streaming fly-throughs (run 7 m/s, flight 60 m/s) with per-stage counters
  (the harness exists; needs `sf_world_bevy`), and the bake-time gate: full first bake ≤ 2 min
  (ceiling 5 min). Bake gate met: 32 s cold, 2.4 s warm (models + exterior index + terrain).
  Morrowind runs (`benchmarks/exterior_{balmora,run,flight}.json`, `view ... --bench <json>
  [--bench-ground]`, release, RTX 4060 Laptop, 720p, shared machine): Balmora standing 21.0 ms median;
  run 23.8 ms median / 54 ms worst; flight 26.5 / 332 ms worst. Stages stay in budget; the gates fail
  on entity count (far field) and spikes (origin shifts, loads). With the far field (2026-10-11, AC, idle,
  1440p): run 11.3 / 18.4 ms worst, flight 10.9 / 20.9; nothing over 33 ms, but worst/median is 1.6–1.9×
  (gate 1.5×): streaming frames (spawn/despawn, Bevy re-extracting every `VisibilityRange`).
  2026-10-12 (spikes, 3 runs each, back to back, 1440p): flight 10.9 ms median, worst 18.2–20.2 (1.67–1.86×,
  was 1.80–2.01×), frames over 1.5× median 32 → 9; run 10.6 / 16.2–21.3 (1.53–2.0×). Left: that ratio
  (streaming.md "Spikes": mesh-upload pacing, upstream `VisibilityRange` extraction, late presents).

Adapter (bevyrrowind; spec `docs/specs/world-adapter.md`):

- [x] `WorldSource` for Morrowind: regions = cells, `PlacementId` = (first-placing file, refnum),
  MVRF re-homed to the destination (vanilla has none; synthetic test), `GROUNDED` from NIF bounds.
  357,692 placements in 1,329 worldspaces, built in ~0.6 s; exterior index 156,778 placements
  in 1,692 cells, 10.3 MB.
- [x] `TerrainSource` from LAND (VHGT, VCLR tint, VTEX grid with its offset, alternating
  triangulation); LTEX → terrain layers (per-plugin LTEX slots in `mw_records`). 1,540 tiles, all
  shared edges exact. Checked 2026-10-11: texture offset matches OpenMW's current source and the
  paving follows the streets on screen; triangulation, axes and `constrain` checked together by
  `cargo run --release -p mw_bake --example ground_check` (grounded pivots: 0.4 mm median, 3.2 cm p99).
- [x] `Worldspace` records (exterior + one per interior, with `mw.cell_environment`); `DoorLink`s:
  1,791 two-way, 37 one-way (8 m pairing), 0 dangling.
  Verify: `cargo test -p mw_world` (incl. real-install test); `cargo run --release -p mw_world --example world_report`.
- [x] Bake + app wiring: world and terrain products in the bake, a streaming exterior view.
  `bevyrrowind bake` adds the exterior index and terrain (`mw_bake::world`; DDS via `ddsfile` +
  `texpresso`); `view <exterior name or "x,y">` streams the whole exterior on big_space with terrain,
  sky, a sea-plane scaffold, cooked statics, NIF-path uncooked models and lights (`apps/bevyrrowind/src/exterior.rs`).
  Verify: `cargo dev view "Seyda Neen" --camera "-100,+3,1012,-170,+4,985" --screenshot a.png`; also
  `"Balmora"`, `"Pelagiad"`, `"2,7" --camera "293,+40,-878,527,0,1346"` (toward Vivec), `--lod-tint`,
  `--terrain-lod --wire`. Not done: `--rt` in exteriors.
- [x] Interiors' index and door links in the exterior view (2026-10-12, `.sessions/interiors.md`): the bake
  indexes all 1,328 interiors as `Single` worldspaces (`mw_bake::world::index_plan`, `WorldBake::with_overrides`;
  layouts in one parallel call), 1,828 door links, 0 unresolved link ends (bake gate). Bake: +8.3 s cold (total
  64.7 s incl. far field, gate 2 min), no-op 1.9-2.1 s (gate 2.5 s). In `view <exterior>`: Space on a door within
  4 m crosses its link with the framework fade, door prefetch keeps the other side's R0 resident, interiors drop
  sky/terrain/clouds/sea and take the cell's ambient + fog color; prefetched worldspaces draw only in their own
  layer (`SubtreeLayers`, light layers, `sf_lod_bevy::FarWorldspaces`). Measured: into the Balmora Mages Guild
  0.63 s, back out 1.02 s (exterior R1 reloads under the held fade). Verify: `cargo test -p sf_world` (interiors
  test), `cargo test -p mw_world --test real_install`, `bevyrrowind bake` (interiors line), scenes
  `door-balmora-guild[-auto]` (DR-1, DR-2 awaiting owner), `view Balmora --camera "-316.6,10.4,200.5,-312.6,10.0,200.35" --use-door 2`.
  Polish (2026-10-12, doorfix): mouse look no longer snaps after a crossing (`TravelArrived` resyncs Bevy's
  `FreeCamera` yaw/pitch); arrival heading is the marker's Z angle (`arrival_rotation`, unit-tested on the DODT
  convention; doorpairs' audit: 3,307 of 3,363 arrivals face away from their door); camera 0.5 m above the marker
  (scaffold `ARRIVAL_EYE_M` until the controller grounds the player); the worldspace just left lingers resident
  (back out 0.64 s, was 1.02 s); `view "<interior>"` opens the streamed view (`--single-cell`, `--rt`, `--no-baked`,
  `--skinned` keep the single-cell viewer). Scene `door-census` (DR-3).
- [x] NIF `ModelImporter` (`mw_bake`), `bevyrrowind bake`, viewer on cooked meshes (spec
  `bake-adapter.md`). First bake 22 s cold (gate ≤ 2 min), 2.2 s warm; Census Office identical to
  the NIF path (PSNR ~51 dB). Verify: `bevyrrowind bake`; `cargo dev view "Seyda Neen, Census and
  Excise Office" --screenshot a.png` vs `--no-baked`.
- [ ] Cook more: `AttachLight` attachment points in `sf_model` (light-carrying models), rest-pose
  cooking of gameplay-driven keyframe models (307), fewer store files if terrain nears the I/O limit.
- [~] Atmosphere + sun; Solari toggle with emissive proxies for point lights. Framework done
  (`sf_sky_bevy`, verified by screenshots: noon, sunset, night, full moon, Solari, live switching).
  Solari is the "ultra" option (owner, ARCHITECTURE §10). Interior ray tracing wired:
  `cargo run -p bevyrrowind --features dev,solari -- view "<cell>" --rt` (cooked meshes get
  load-time RT variants). Sky in exteriors done (exterior view, sun/moon on the worldspace's render
  layer, sea level follows the floating origin). DLSS done (2026-10-12, sky-lighting.md): Ray
  Reconstruction denoises `--rt` (ReSTIR auto-off under it), Super Resolution on raster via
  `view --dlss <off|auto|native|quality|balanced|performance|ultra-performance>` (default off;
  build `--features solari,dlss` or `dev,dlss`). Exterior `--rt` runs but nothing streamed is in
  the RT scene yet (flat, no shadows). Missing: exterior RT content (world objects with
  `ray_tracing_lod`, `NoRayTracing` on far-field proxies, terrain RT proxy); Solari sunsets too
  red (`bevy_solari` patch); flames/glows and NIF-path models not in the RT scene.
- [~] Object-terrain blending (R-b, 2026-10-12, `.sessions/terrainblend.md`; superfargoth terrain.md
  "Object-terrain blending (as built)"): rocks, cliffs, roots and building bases blend into the
  ground through `GroundBlendMaterial` (StandardMaterial + the terrain clipmaps, height-above-terrain
  band per model from `mw_assets::ground_blend_band`). Raster only; not on far-field proxies (vertex-
  colour bake later). `--off terrainblend`, live F11. Awaiting owner: TB-1..3 (visual-checks.md).
- Verify: fly from Seyda Neen to Balmora within the frame-time targets; benchmark entry.

## M3: Characters + animation

Specs (draft 2026-10-11): `animation.md`, `character.md`, `locomotion.md` in
`../superfargoth/docs/specs/`. Bevy's `AnimationGraph` is not used for characters (no root
motion, no IK hook between blend and write-back, no sync/inertialization): cooked clips and a
Bevy-free pose runtime instead; Bevy keeps skinning, morphs, culling. Morrowind mapping notes
(Bip01 → humanoid, KF groups, text keys, body parts) were captured by the M3 research session;
put them in a bevyrrowind adapter spec before starting the adapter tasks.

Framework (superfargoth):

- [x] `sf_model` skin data: joints/weights streams, skin (joint names, inverse binds), morph targets;
  `.sfmesh` v2; glTF importer reads skins and morphs; `mesh.cook` cooks skinned parts (no flatten,
  weights as LOD attributes, rigid parts → single-joint skins) (bake, character).
  Verify: `cargo test -p sf_model`; a Quaternius glTF character round-trips and renders in bind pose.
  Bevy-free part done 2026-10-12 (bake.md "Skinned and morphed models"): `Cooked::Skinned` (v2; v1 files read and
  static cooks stay v1, no re-cook), per-part submeshes with node paths, palette by name + inverse bind, joint-center
  LOD attribute, `CookedSkinned::rigid` (single-joint skin, mirror flips winding). Left: `sf_model_bevy` mapping
  (treats `Skinned` as skipped for now) and the bind-pose render; `mw_nif` skins/morphs (adapter item below).
  M3-a morph layout done 2026-10-12 (m3a; bake.md "Morph layout", character.md "Morph-bearing parts"): `.sfmesh`
  v3 (v2 retired, `KEY_FORMAT` 2: one full re-cook), per-submesh target lists, zero targets stripped, expression
  targets on LOD 0-1 only, `MorphKind` shape/expression; `sf_character::assemble` (merge per material, morphed
  submeshes kept apart, shapes baked). The loader maps the layout in bake.md "Bevy mapping"; nothing else blocks it.
  Bevy side done 2026-10-12 (skinloader; bake.md "As built (skinloader)"): `sf_model_bevy` loads `Cooked::Skinned`
  (`CookedSkin`; limits fail loudly), `spawn_skinned` + `resolve_joints` + `ModelMorphWeights`/`sync_morph_weights`,
  `DynamicSkinnedMeshBounds`, `spawn_rest_skeleton` scaffold. Verified: Quaternius UAL2 mannequin cooked vs `bevy_gltf`
  identical (example `skinned`), guar and dark elf head via `view <nif> --model --skinned`. Owner checks SN-1..SN-3.
- [x] `sf_anim` core: `Skeleton`, cooked `Clip` (uniform rate, quantized), `skeleton.cook`/`anim.cook`
  (root-motion extraction, contact curves, sync markers), sampling, blend/mask/additive,
  local→model, events/windows, curves, glTF clip import (animation).
  Verify: `cargo test -p sf_anim` (error bound, events once across wraps and reverse, additive cancels).
- [x] `sf_anim` IK + retarget math: two-bone, aim chain, FABRIK, CCD, springs; `anim.retarget` (character).
  Verify: tests (reach/clamp without flips; S→T→S round trip ≤ 1° per bone, ≤ 1 cm at effectors).
- [x] `sf_anim_graph`: RON graph asset, `animgraph.compile` (weighted masks, blend-space triangulation),
  evaluator, state machines, inertialization, sync groups/markers, procedural op stack (animation).
  Verify: `cargo test -p sf_anim_graph --all-features` (19 tests: pose and velocity continuous at inertialized
  switches and interruptions, same result on 1 and 8 threads, marker sync, masks, blend spaces, ops, bake step).
  Left: reference-clip additive bases; ops only checked on the synthetic biped (real check in `sf_anim_bevy`).
- [x] `sf_character`: humanoid rig (VRM 1.0 bone set), `RigMap` auto-map + auto-align, slot registry,
  `BodyPart`/`Wearable` records, slot resolution + outfit key, `CharacterSource` (character).
  Verify: `cargo test -p sf_character` (17 + 2: two real CC0 Quaternius rigs in `tests/data` auto-map clean with
  no fixes, plus Bip01-, Mixamo-, UE5- and VRM-named synthetic rigs; auto-align reaches the T-pose from an A-pose
  facing −Z; Quaternius ↔ Bip01-style round trip < 1° / 1 cm; layers, hides, ties, outfit key).
  Left: `RecordRef` → `sf_records` so `CharacterSource::recipe` can take it (character.md).
- [x] `sf_controller`: intent, spring velocity/facing, trajectory prediction, `GroundProbe` with
  `TerrainProbe` + `StaticProbe` (parry3d BVH of R0 statics, `collision.cook`), path follower (locomotion).
  Also the kinematic mover's logic (Bevy-free) and character separation.
  Verify: `cargo test -p sf_controller --all-features` (18 + 3: trajectory = simulation to float precision at
  +1 s incl. frame jitter, stairs, ledges, drops, 30°/60° ramps, wall slide, batched = single probes, bake step;
  `tests/graph_drive.rs` drives a real `sf_anim_graph` graph). 150-character step 0.096 ms (battery).
  M3-b ORCA local avoidance done 2026-10-12 (orca; locomotion.md "Local avoidance (ORCA)"): `Avoidance::solve` on intents
  before the mover, player non-reactive, idle give-way, priority/heading-rule yielding; doorway 4 + 4 swap 13.2 s, 150
  agents ≤ 0.087 ms. Left: the `sf_controller_bevy` stage and `PathWalkScaffold` filling `travel`/`priority`.
- [x] `sf_anim_bevy`: plugin and schedule (Probe → Evaluate par_iter → Apply), bone write-back,
  events as messages + observers, LOD tiers A0–A3 + budget allocator, gizmos, counters.
  Verify: viewer plays a glTF character through a 2D blend space with skeleton gizmos; screenshot.
  Done 2026-10-12 (animbevy; animation.md "Bevy integration / As built"): `spawn_rig` (replaces the
  `spawn_rest_skeleton` scaffold for animated characters), `Animator`/`AnimAsset`, per-bone `par_iter` write-back
  only on change, root motion stored or applied, `AnimEvent` messages + `AnimNotify` observers, tiers by projected
  height + hysteresis + off-screen A3, A2 at half rate, `lod::allocate` (EMA cost per tier), `anim/...` counters,
  tinted bone gizmos. Example `blendspace` (UAL1: 7-clip 2D blend space, AN-1), app `view --skinned --animate`
  (guar AN-2, dark elf skins on base_anim AN-3). 4 tests (headless walk: bones, Drive root motion, events =
  observers, A3 frozen, A2 every other frame; tiers; allocator). Left: A2 interpolation, morph curves, Probe
  stage, crowd timings (owner busy: no benchmarks this session).
- [~] `sf_character_bevy`: spawn from recipe, per-material merge + outfit cache, sockets/held items,
  face morphs, first-person view model + shadow-only body (character).
  Verify: viewer `character` mode screenshot; outfit swap without a pop; 1 skinned mesh per material.
  Core done 2026-10-12 (charbevy; character.md "As built (`sf_character_bevy`)"): `Character` component +
  `CharacterContent` hook, off-thread resolve/load/assemble, `OutfitCache` (identical looks share meshes),
  rig + idle graph, settle-then-swap (no pop, rig kept), `CharacterSpawner` (feature `world`) for actor
  placements; app: `actors.rs` (`MwCharacters`), streamed exteriors/interiors spawn NPC_/CREA/LEVC
  (`--off actors`), `bevyrrowind character [ids]` lineup (CH-1..CH-4). Verified: 2 headless tests; M3 lineup
  dressed (13-19 meshes per NPC = materials + head, 0 missing parts/joints); Balmora street NPCs standing.
  Held items done (heldrender). Left: LODs, morph curves, first person, tints, texture atlas, timings.
  Node-animated creatures done 2026-10-12 (nodecreatures; character.md "Node-animated creatures"): creature
  bodies cook as rigid skins (`sf_model::cook_rig`, bake step `mesh.rig`, rigid mesh -> its transform node);
  census test `mw_bake creature_rigs_real_install`: 89 CREA models (19 skinned, 63 mixed, 7 node-animated,
  0 static) all cook; before, 2 were skipped (minescrib, heart_akulakhan) and 45 had rigid parts on the root
  (wolf/bear/goblin heads, antennae, blades, storm atronach rocks). Left: heart_akulakhan billboard parts on the
  root, morph-animated bodies (kwama forager) still, material/UV/visibility controllers on creature parts.
  Headless test: node-animated joints move under the graph. NC-1, NC-2 awaiting owner.
- [~] Smooth bodies (owner decision 2026-10-12: ON by default, vanilla segmented as an option): seam weld +
  harmonic weight blending for segmented bodies and armor, weight transfer for hair/skinned cloth, no new art,
  replacer bodies pass through (smoothbody; `../superfargoth/docs/specs/smooth-bodies.md`).
  Verify: `cargo run --release -p mw_actors --example smooth_body` (seam opening / poke metrics, posed OBJs in
  `../shots/smoothbody/`) and `--example smooth_survey` (44 cases: all races × sexes + 8 outfits, 7 poses;
  `../shots/smoothbody2/metrics.txt`). **Live** (smoothlive 2026-10-12): `sf_character::smooth_outfit` in the
  build task before `assemble` (pre-decoded parts), mode in `CharacterSettings` + `OutfitCacheKey`, probe poses
  from `mw_actors::probes`; `--bodies vanilla|smooth`, B toggles live (no-pop swap), `character --walk`.
  SB-1..SB-5 awaiting owner (`scene sb-balmora`). Next: `outfit.smooth` bake pre-warm (miss cost 5-116 ms per
  outfit, indicative), the open failures below.
  Robustness pass done 2026-10-12 (smoothbody2): overlap links, authored pinning, seam followers, overlap copy,
  deterministic. smoothbody3: per-seam vanilla guard (`guard_seams`; the live path must pass probe poses),
  female knee was an input bug (fixed by feetfix); open: tail root ↔ pelvis armor 44-66 mm, pauldrons 13-22.
  Bevy-free prototype done 2026-10-12 (`sf_character::smooth`, feature `smooth`): dark elf male + iron set,
  elbow 90° seam opening 45 → 2 mm naked, 56.5 → 1.6 mm dressed (outfit smoothed as one set), walk 37 → 7 mm,
  ~15 ms per outfit.
- [~] `sf_controller_bevy`: `KinematicMover`, batched probes, character separation (locomotion).
  Verify: an NPC walks up stairs, a ramp, and terrain; screenshot sequence with trajectory gizmos.
  Done 2026-10-12 (walkers; locomotion.md "Bevy integration: `sf_controller_bevy` as built"):
  `SfControllerPlugin::<G: GroundProbe>`, `Mover`/`MoveIntent`/`Avoider`/`AvoidObstacle`, one Step system
  (ORCA → `step_batch` → transforms), animation feed in `AnimSystems::Probe` (drive + adapter-named params +
  batched per-leg ground samples), counters `controller/{avoid,move,write,feed}/ms`, trajectory gizmos (J).
  App `walkers.rs`: Balmora wanderers walk pathgrids on terrain + R0 `collision.cook` statics, avoid each
  other and the camera; `mw_humanoid` gains FootPlacement (K toggles). 3 headless tests. Scenes wk-balmora,
  wk-crossing, wk-stairs (WK-1..WK-4, awaiting owner).
  Done 2026-10-12 (walkers2; locomotion.md "Processing range, interiors, race scale, stuck handling"):
  processing range = R0 coverage (`Scheduler::covers` + `Dormant`: no actor moves without loaded colliders
  around it; dormant actors park, idle at low animation significance, resume in place), ground per worldspace
  (interior statics never mix with the exterior; terrain only outdoors), `Mover::body_scale` (race scale in
  foot probes and the motion feed). Tests: dormant never moves / resumes, `covers` per level. Scenes
  wk-interior (WK-5), WK-6 (range), awaiting owner. Left: stairs screenshot sequence with a walker on the
  steps, crowd timings, off-screen schedule tier (M10).
  Done 2026-10-12 (animfix; locomotion.md "Measured feed", animation.md "Foot placement guards"): the
  feed gives the graph the measured velocity (displacement/dt after avoidance and collision) with
  moving/running hysteresis by speed and the turn rate (turn in place); humanoid `SpeedMatch 0.5–1.5` +
  `StrideWarp`; `FootPlacement` guards (step limit, drop limit, unplant radius, reach clamp, weight blend,
  teleport reset, NaN guard; counters in the walkers log). Root cause of the blow-ups: foot locks drifting
  past leg reach pulled the pelvis to the ground. Tests: guards regression (reproduces the blow-up with
  guards off), feed hysteresis, measured = travelled. Awaiting owner: WK-1, WK-4, WK-7, SB-1.
- [~] Actor non-skeletal animation (actorfx 2026-10-12): morph curves play by target name (clip curves +
  `MorphCurvePlayer`, `sf_character_bevy::morph`); kwama forager body morphs ride its KF clips (KF importer
  reads `x<model>.nif`, v3); NPC heads blink (2-8 s) and talk (`Y` talk-key scaffold, `faces.rs`); LEVC
  placements wander as their picked creature. actorfx2: visibility and material (alpha, color, UV)
  controllers of creature models play on the animation timeline as clip curves (`mw_anim::effects`, NIF
  importer v5, KF v4), written after the pose by `actor_fx.rs` (joint scale 0, per-character material copy);
  billboard nodes are rig joints that face the camera (heart_akulakhan: 6 parts, 0 root-bound). Census: BODY
  parts carry none; 13 CREA models visibility, 6 material. particles3: creature particle emission rides the
  animation timeline (`emit:<particle node>` clip curves, NIF anim importer v7, KF v6, `ActorEmitter` gate;
  `ACTOR_EMISSION_SCAFFOLD` removed): Dagoth Ur's hands in `Idle2`/`SpellCast`, atronach hands in attacks,
  dremora dust in deaths, atronach body flames always. Left: initially hidden geometry (dremora/ghost death
  dust meshes: the bake drops culled nodes), death/attack triggers in a test scene (the lineup only idles and
  fidgets), owner pass AF-1..AF-6, PA-1, PA-3.
- [ ] `sf_diag` "crowd street" scenario and gates: animation ≤ 1.0 ms wall, A0 ≤ 25 µs, skinning ≤ 0.5 ms,
  foot slide ≤ 2 cm/s, penetration ≤ 2 cm p95, `anim.cook` ≤ 5 s.
  Verify: the bench passes; baseline recorded.
  Scoped 2026-10-12 (animcheck), not built: no crowd scenario exists yet (`perf/cli.rs` `SCENARIOS` and
  `benchmarks/` have only streaming scenarios). Ready: `anim/evaluate/ms`, `anim/apply/ms`, `anim/a0..a3/count`,
  `controller/*/ms` counters and the `MaxCounter` gate. Needed: (1) `benchmarks/crowd_street.json` (standing camera on
  a Balmora street, walkers on, 15 s) + a `crowd_street` suite entry; (2) a crowd-size control: vanilla Balmora gives
  ~70 active movers, not 150 with 40 at A0 (an extra-NPC spawner or a count knob in `walkers.rs`); (3) foot slide
  and penetration metrics (new: planted-foot world drift per contact and foot-vs-probe height, as `anim/feet/*`
  counters; today `FootStats` only counts guard events); (4) skinning cost: Bevy skins in each pass's vertex
  shader, so measure an A/B (actors on vs `--off actors`, GPU pass times) rather than a pass; (5) A0 ≤ 25 µs from
  `anim/evaluate_cpu/ms` ÷ `anim/evaluated/count` or a release micro-bench; (6) `anim.cook` ≤ 5 s is the animcook
  timing run (owed). About one agent session for (1)-(3) + (5), then a quiet-machine run.

Adapter (bevyrrowind):

- [x] `mw_nif` actor import: skins (`NiSkinInstance`/`NiSkinData` → `sf_model` skin), geometry morphs
  (`NiGeomMorpherController`), KF import (per-bone keyframe controllers, root text keys) → source clips
  split per group. Verify: `nif-sweep` has no skin/morph entries left; `bevyrrowind anim <kf>` lists groups.
  KF part done 2026-10-12 (`mw_anim`, actors-adapter.md "KF import as built"): `KfAnimImporter`, one clip
  per group + sub-action sections, root motion `Bip01`; all 162 install KFs import (2330 clips). Listing via
  `cargo run -p mw_anim --example anim_report -- meshes/xbase_anim.kf`; app subcommand `anim` still to add
  (thin wrapper over `mw_anim::report`). Skins + morphs done 2026-10-12 (nifskin; nif-runtime.md "Skins and
  geometry morphs"): `mw_nif::skin` (bind-space vertices, bones → nodes, inverse binds, influences; morph deltas
  + weight curves), NIF importer v2 → `Cooked::Skinned` for body parts, heads, creatures. `nif-sweep` verified
  2026-10-12 (health): 7,319 NIFs, 0 failed; `NiSkinData`/`NiSkinInstance` (556 files, 3,069 objects) and
  `NiGeomMorpherController`/`NiMorphData` (273 files, 324 objects) all "ok", no `NiSkinPartition`; no skin/morph
  entry left partial or unsupported. Loader mapping done (skinloader); `bevyrrowind anim meshes/xbase_anim.kf` lists
  135 groups (animcheck 2026-10-12): nothing left.
- [~] Rig maps: Bip01 → humanoid for `base_anim`, `base_anim_female`, `base_animkna`, `base_anim.1st`;
  root-motion joint; retarget pose; lower body / torso / left arm / right arm masks.
  Verify: viewer overlays the humanoid map; Quaternius walk retargeted onto Bip01 next to the source.
  Maps, masks, retarget pose done 2026-10-12 (`mw_anim::rig`, actors-adapter.md "Rig maps as built"): all four
  auto-map clean with a two-line hint table (`Bip01*` joints are bones; root motion `Bip01`); skeletons turned to
  face +Z at import; Quaternius → Bip01 → back: body ≤ 0.07°, hips/hand/foot < 1 cm (fingers 2.8°: 2 vs 3
  segments); four masks partition each skeleton. Left: the viewer overlay and side-by-side walk (visual).
- [~] Text keys → events (SoundGen, Sound, attack windows, equip/unequip, shoot, spellcast).
  Verify: table test on `xbase_anim.kf`; footsteps logged while walking.
  Mapping and table test done 2026-10-12 (`mw_anim::events`, `tests/real_install.rs`; every key in the 162
  KFs maps). Left: footsteps logged while walking (needs `sf_anim_bevy` playing a character).
- [~] Humanoid and creature graph assets: locomotion state machine, weapon-stance upper-body layer,
  Idle2–9 variants, hit/knockdown. Verify: viewer param sliders cycle every state on a dressed NPC.
  Assets + Bevy-free validator done 2026-10-12 (`mw_graphs`, actors-adapter.md "Graph assets as
  built"): `mw_humanoid` / `mw_first_person` / `mw_creature` built in code, shipped as RON; group
  references with fallback chains; stance machines (own set or upper-body overlay); `upper`/`full`
  slots; PlayGroup/LoopGroup → `SlotRequest`; biped creatures take the humanoid graph. Real install:
  4 humanoid skeletons + 89 CREA models compile, headless simulation drives every state with
  footsteps, attack windows and the scripted slot. Left: the viewer sliders (heavy).
  Bake wiring done 2026-10-12 (animbake, bake-adapter.md "Animation"): `mw_bake::anim` bakes every
  actor's skeleton, clips and graphs in two phases (96 skeletons, 3,987 clips, 96 graphs; cold 13.5 s,
  warm 0.31 s), `AnimLibrary` loads them Bevy-free, `mw_graphs::builtin_source()` is the lowest VFS
  layer. App wiring done 2026-10-12 (animbevy): builtin graphs mounted in `mount()`, `bevyrrowind bake` runs
  `bake_animation` (96 sets, 182 files, 3,307 group clips, 96 graphs, 0 failures). Viewer keys drive every graph
  parameter on a body (`view --skinned --animate`, AN-2/AN-3); the dressed NPC waits for `sf_character_bevy`.
  Left: `anim.cook` ≤ 5 s (animcook).
- [~] `CharacterSource` for NPC_/CREA: race skin parts from BODY, head/hair, equipment from inventory,
  ARMO/CLOT part lists → `Wearable`s, female → male fallback, beast-race rules, race scale, first person.
  Verify: Balmora NPCs dressed correctly; compare screenshots against OpenMW as a black-box oracle.
  Bevy-free core done 2026-10-12 (`mw_actors`, actors-adapter.md "`CharacterSource` as built"): lazy part
  records (skinned/rigid probe, left-bone mirroring), wearables with the original part priorities as layers,
  auto-equip, autocalc skills, race scale, skeleton by sex/beast race, first person; framework hook
  `BodyPart::node_filter` + `PartKind::Rigid::offset`. `cargo test -p mw_actors`: 6 synthetic + 1 real-install
  (3,041 NPC_ and 396 CREA all give recipes; guard, Khajiit, Argonian, female fallback, naked, guar, cliff racer,
  centurion). Left: held items, leveled inventory, vampire heads, the cook honouring filter/offset, and the
  visual check (needs `sf_character_bevy`).
  Held items (Bevy-free) done 2026-10-12 (helditems; actors-adapter.md "Held items as built"): `sf_character::HeldItem`
  (item, model, hand, drawn/sheathed `Mount`s, `HeldLight`), `mw_actors::held` (OpenMW weapon/shield/torch rules, LEVI
  scaffold pick, `is_dark`), `MwActors::held(kind, id, ctx)`, recipes hold the daylight default; `mw_graphs::Stance::of_weapon`.
  Real data: 3,041 NPC_ drawn stances 1h 1,572, hh 1,060, 2c 261, 2w 137, crossbow 11; 80 of 82 guards armed; 47 torches at
  night; Redoran shields. Heavy side done 2026-10-12 (heldrender; character.md "Held items at runtime"): props as
  socket-joint children (separate entities, per-model shared meshes, not merged into the outfit), `CharacterReadiness`
  drawn/sheathed, held-only changes keep the outfit, torch `HeldLightSlot` -> LIGH point light + flicker, 12 nearest lit,
  dusk/dawn re-ask, `stance` param fed. Checks HI-1..HI-4 awaiting owner. actorparticles 2026-10-12: bows as skinned props on
  their own small skeleton (rest pose), torch flames + creature particles (nif-runtime.md "Particles on actors": joint-anchored
  emitters, world-space carry, vis-curve gating, 60 m range), carried-light near field (0.7 m clearance, 0.7 intensity); AP-1..3
  awaiting owner. Left: bow string pull, NiBSPArrayController vertex emission, first person,
  interior darkness by cell.
- [~] `PathWalkScaffold` driver from PGRD pathgrids + AI_W wander. Verify: Balmora NPCs walk pathgrid edges.
  Bevy-free core done 2026-10-12 (`mw_ai`, actors-adapter.md "NPC life in M3 / As built"): engine-space
  pathgrid graphs + A*, AI_W idle roll and wander planner, `PathWalkScaffold` → `PathFollower` intents.
  Real install: 1,386 grids, 0 errors; 40 of 41 Balmora wanderers with nodes in range get grid walks.
  App wiring done 2026-10-12 (walkers, `walkers.rs`): per-character scaffold, `travel` from the path segment, speed from
  the Speed attribute + GMSTs, Idle2–9 fidgets. Interiors + stuck handling done 2026-10-12 (walkers2): grid keyed
  by the worldspace (interior = cell name; 18 Balmora interiors with wanderers on their grid, homes ≤ 1.6 m
  from a node); walks with no progress for 3 s are given up and replanned (OpenMW `AiWander` behaviour).
  Left: owner checks WK-1, WK-5.
- [~] Static collision source for `StaticProbe` (RootCollisionNode when present, else LOD1).
  Done 2026-10-12 (`mw_bake::collision`, actors-adapter.md "Static collision as built"): OpenMW order
  (skinned, `NC` marker, `RootCollisionNode`, `MRK` → none/authored), rest pose for animated models, else
  LOD1. Real install: 3,168 authored, 2,497 LOD1, 150 rest pose, 794 none; 759k tris; +8.5 s cold bake.
  Verify: `cargo run --release -p mw_bake --example collision_report`; `tests/collision_real_install.rs`.
  `bevyrrowind bake` requests `collision::collision_keys` (checked by animcheck 2026-10-12). Left: footstep surfaces.

Next-gen animation (anim-plan 2026-10-11; specs: `animation.md` "Next-gen roadmap" + audit table, `rig-profile.md`,
`locomotion.md` "Context probes, traversal and creature modes", `docs/specs/animation-sources.md`). Start after
`sf_anim_bevy` plays a character. The creature items gate the creature graph assets above; warping, full-body IK and
context probes are not milestone gates. File:line refs are the audit's.

- [x] `sf_anim::rig`: `RigProfile` (girdles, N limbs, spine, neck/head, tails), auto-detection + `rig.profile` step +
  RON review report, `TwoBoneAnkle` solver; tags `tail`/`wing`/`claw`/`hoof` (`skeleton.rs:124-185`); contacts from
  support limbs with thresholds × leg length (`cook.rs:114`, `:854`); retarget IK fix-up by limb solver (`retarget.rs:43`).
  Verify: tests on the Quaternius humanoid rigs + a CC0 quadruped and bird rig; `TwoBoneAnkle` reaches without flips.
  Done (anim-rig): synthetic quadruped/hexapod/flyer fixtures (`sf_anim` feature `fixtures`) + both Quaternius rigs; no real
  CC0 quadruped/bird fixture yet.
- [x] `sf_anim_graph` creature-ready ops: `Leg` → profile limbs (`compiled.rs:164`, `asset.rs:353`); `FootPlacement` per
  girdle with body pitch/roll (`procedural.rs:160-209`); `StrideWarp` per girdle (`:111-143`); solver by limb + pole hint
  (`:86-93`); `SpineBend`, `Lean`; `HandIk` → `LimbIk` with rotation targets (`:230-242`, `instance.rs:141`); `ChainIk`.
  Verify: synthetic quadruped on a 20° slope and a 0.3 m step: support feet ≤ 2 cm p95, pitch within 3°, slide ≤ 2 cm/s.
  Done: 20° uphill p95 1.4 cm, pitch 17.9°, slide ≈ 0; hexapod and biped pass on slopes and stairs. anim-creatures:
  20° downhill slide ≈ 0 (was 2.7 cm/s), pitch −18.7°; stairs/steps p95 1.4 cm (was 13 cm), all asserts on.
- [x] `sf_character` follow-ups (audit 16–17): `RigMap` → `RigProfile`, `RigRef` so creature recipes need no humanoid map
  (`records.rs:128`), leg chains as the full joint path for digitigrade legs (`rig.rs:189-194`).
  Verify: `cargo test -p sf_character`; `base_animkna`-like fixture keeps foot contacts through retarget.
- [x] Motion warping: `warp` event windows (skew / scale, rotation default / facing, turn-rate and distance limits, warp
  point), `warp_targets` input, `ActiveWarp` output; mover `Traversal` flag for vertical root motion (`mover.rs:251`).
  Verify: tests: end error ≤ 2 cm / 2° (static target), ≤ 5 cm (target at 3 m/s), no root-speed jump > 20%, ≤ 0.5 µs.
  Graph side done: 0.0 mm / 0.000° static, moving within 5 cm, jump 13%, vault 0.0 mm. anim-creatures: mover
  `RootMotion::traversal` (from `ActiveWarp::vertical`); warped mantle onto a 1.2 m box through the controller glue lands
  within 1 mm, capsule never in the box (`tests/graph_drive.rs`); release warp cost 0.10–0.12 µs.
- [x] Optional clips in `animgraph.compile` (`optional` + `fallback`/`drop`, provenance per slot).
  Verify: a graph with missing blend-space samples and a missing state clip compiles and plays; report lists sources.
  Built (optclips): `(source, optional: true, fallback: Clip(..) | Drop)`; drops are structural at compile (sample,
  node, state + its transitions, layer); `GraphRules::clip_files` checks are recorded dependencies (adding a source
  recompiles); `ClipSlot::provenance` + `CompiledGraph::resolutions` / `non_redistributable()`; 3 tests. `mw_bake`'s
  `GraphRules` (manifest → `clip_files`/`provenance`) and the export guard remain (animation-sources.md).
- [x] M3-d montage `Slot` node (industry-audit G4): `Slot(slot, input, group, mask)`, `slot_clips`, `LateClip`,
  `play_slot` (interrupt w/ priority or queue, loops, `from`, `end_early`, inertialized blend in/out, root-motion mode),
  events once + window exits on interrupt, warp windows, `slot_notices`. Verify: `cargo test -p sf_anim_graph
  --all-features slot` (5 tests: events once, root motion = `Drive` player, interrupt continuous, queue/group/mask/stop,
  compile rules). Adapter side (PlayGroup/LoopGroup → `SlotRequest`) is a mapping in animation.md, not wired yet.
- [~] Full-body IK: `sf_anim::ik::FullBody` (position-based, XPBD compliance, hinge/cone-twist limits, twist kept) +
  `FullBodyIk` op. Verify: reachable effectors ≤ 1 cm at 10 iterations, limits hold, 1 = 8 threads, ≤ 8 µs / 30 joints.
  Built (fbik, fbik2): all verifications pass, head goals now ≤ 1 cm (< 0.001 mm); 5.0 µs biped, 7.5 µs quadruped + head,
  8.7 µs 66-joint humanoid (still 0.7 µs over 8 µs). Probes session: body-only refreshes in recovery/polish/root shift, joints
  outside the body once per solve: 8.65 µs, noise-level; the cost left is the limb pass (`solve_limb` refreshes the hand
  subtree, fingers included, per joint turned): next, a deferred-refresh `solve_limb` variant.
- [~] `sf_controller` context probes (obstacle, ledge, drop, wall, ceiling → affordances scaled by rig measures) and mover
  modes `Fly { hover }` / `Swim` with 3D intent (`intent.rs:19`). Verify: classification table tests (0.3–2.5 m boxes,
  ledges, drops), batched = single; a hovering creature holds height ± 5 cm over terrain at 6 m/s.
  Done (anim-creatures): `MoverMode { Ground, Fly { hover }, Swim { surface } }`, 3D intent, ceiling/floor/surface
  clamps, hover ≤ 1.9 cm at 6 m/s (flat and 10° ramp, slope fed forward), attitude from root tilt, pitch/bank params,
  `MoverParams::from_profile` (audit 14). Context probes built (probes): `sf_controller::ContextProbes` (obstacle,
  ledge, drop, wall, ceiling over any `GroundProbe`, ≤ 4 batched stages) → `Affordance` + lip/normal/height/depth/
  clearance/landing, `warp_target`/`hang_point`; table tests (0.3–5 m boxes, ledges, stairs, overhangs, terrain cliff),
  batched = single, a probed mantle lands the warp ≤ 2 cm. Open: 150 all-blocked obstacle probes 0.79 ms single-thread
  (5.3 µs each, 2 µs of it the capsule sweep) vs the 0.2 ms budget; within the spec gating (A0, blocked only) ≤ 0.21 ms.
- [x] `sf_anim` importer features `bvh` and `fbx` (`ufbx` 0.11.5, MIT OR Unlicense), bake only.
  Verify: a 100STYLE BVH and a Quaternius FBX import with correct units and axes; clips round-trip through `anim.cook`.
  Done 2026-10-12 on fixtures (own BVH parser, ufbx FBX, `SourceSpace`, root-motion detection; BVH → Biped retarget
  0.000 cm / 0.034°). Open: `mw_bake` enables the features + manifest spaces, and a real 100STYLE/Quaternius file pass.
- [~] Adapter: creature rig hints, mover mode from CREA flags (`Walks`/`Flies`/`Swims`), creature graph templates per rig
  template (quadruped, multiped, flyer, floater, swimmer). Verify: a guar and a nix-hound on Balmora stairs and terrain
  with all feet placed (screenshot sequence); a cliff racer flies and banks.
  Rig hints and movers done 2026-10-12 (`mw_anim::rig::{creature_profile, CreatureMover}`): guar Biped (digitigrade
  HorseLink legs, no hints), kagouti Biped on its forelimbs, nix-hound Quadruped (4 digitigrade, hinted), cliff racer
  Flyer (wings + rudder fin hinted), centurion spider Multiped (6 legs hinted); Flies → Fly, Swims w/o Walks → Swim.
  Left: graph templates, the visual checks.
- [ ] Adapter: bring-your-own animation sources (`animation-sources.md`): manifest, `anim-sources check|report`,
  retarget onto the Bip01 skeletons, KF fallback per slot, export guard for non-redistributable provenance.
  Verify: bake with only Quaternius UAL1 (free) + 100STYLE succeeds and reports slots; with no sources, pure KF plays.

- Verify (milestone): Balmora exterior with its NPCs walking, feet on the stairs (screenshot sequence),
  and a Balmora "crowd street" benchmark entry within the gates. Creatures too (owner, 2026-10-12): a guar and a cliff
  racer animate, foot-place and move under the same gates as the Balmora NPCs.

## M4: Physics

Specs (draft 2026-10-12; the five decisions owner-confirmed 2026-10-12): `../superfargoth/docs/specs/physics.md` (framework), `docs/specs/physics-adapter.md`
(Morrowind), plus `animation.md` "Next-gen roadmap" layer 4 and `locomotion.md`. **Backend: `rapier3d` 0.36.1 directly,
behind the Bevy-free `sf_physics`** (ARCHITECTURE §10, 2026-10-12): on Bevy 0.20 now, same `parry3d` 0.31.1 as the M3
probes, per-axis joint motors and multibodies for powered ragdolls, our own exact floating-origin rebase; avian 0.8 is
the fallback if P1 misses its gates. Done when (ARCHITECTURE §9): "Knock a table's contents over; a dead guard
ragdolls; robes move." Tasks in dependency order:

- [x] P1 `sf_physics` core + spike (Bevy-free, rapier3d): `PhysicsWorld` per worldspace, anchor + whole-cell rebase,
  layers, `BodyDesc`/`BodyId`, fixed-step interpolation state, events, `PhysicsProbe` (`GroundProbe`), statics from
  `StaticShape`/`collision.cook` sharing one `Arc` per model. Spike inside it: a 50-item pile and a 15-body ragdoll
  chain (impulse joints vs multibody), measured.
  Verify: `cargo test -p sf_physics` (rebase: stack + ragdoll with and without a mid-run rebase within 0.1 mm, no wakes;
  `PhysicsProbe` = `StaticProbe` on fixtures); bench: idle R0-sized static set ≤ 0.3 ms per step, pile + 2 ragdolls
  ≤ 2 ms, rebase ≤ 2 ms. Misses → re-decide backend (avian 0.8) before P2.
  **Done 2026-10-12 (physics-p1): GO on rapier3d, impulse joints.** `superfargoth/crates/sf_physics` (Bevy-free; rapier3d
  0.36.1 + parry3d 0.31.1, one parry in the tree): `PhysicsWorld`/`PhysicsWorlds`, `BodyDesc`/`BodyId` (generational),
  `Layer` pair matrix, `MeshShape` (from `CollisionMesh`, one `Arc` per model, cached scaled copies), `HeightShape`,
  interpolation history + `pose_at`, events (Woke/Slept/ContactForce/FellOut with rest-pose recovery), joints
  (ball/hinge/fixed, impulse or multibody, acceleration-based drives), `PhysicsProbe`. Rapier wakes both sides of every
  new contact and of every moved collider, so bodies inserted asleep (and sleepers during a rebase) **settle**: locked for
  one step with a zero sleep delay, re-slept by the engine in that step, unlocked: 0 wakes, poses bit-exact. 9 tests
  pass, clippy pedantic clean. Spike (`cargo test -p sf_physics --release -- --ignored --nocapture spike`; release,
  shared machine, the owner's build started during the second run), R0 = 2,400 statics (120 meshes, 1.26 M placed
  triangles) + 64 terrain chunks (129²) + 600 sleeping items: **idle 0.001 ms** (gate 0.3); **rebase** 0.09 ms call +
  1.31 ms next step (max 1.55; re-settle of 600 sleepers + broad-phase refit; gate 2); **active** (50-item pile + 2
  ragdolls, 15 bodies each) impulse joints limp 0.41 ms mean / 0.74 max, powered 0.44 / 0.73 (gate 2); multibody
  limp 4.3 ms mean with a 450 ms hitch, powered 3.1 / 4.0: **misses**. Joint stretch in a 0.5 m drop: multibody 0.6 cm,
  impulse 2.0–2.6 cm at 4 extra iterations (1.1 cm at 12), so P7 needs impulse joints with more iterations or tuned
  mass ratios to meet its 1 cm gate (open question 1 leans impulse). Rebase gate: the step across a rebase is within
  float noise for every body and a 30-box stack stays bit-identical over the run; a powered ragdoll's end state is not
  compared because the P1 drives don't settle (0.5 m/s residual: per-axis targets on ball joints; P8 owns swing-twist
  targets). Not yet met: activation insertion 12.7 µs/body and a 43 ms first step when all of R0 goes in at once: P2
  must budget insertion (≈35 bodies per 0.5 ms frame) and measure incremental broad-phase cost.
- [ ] P2 `sf_physics_bevy` + app wiring: `PhysicsPlugin` (60 Hz `FixedUpdate`, ≤ 3 steps/frame), write-back as
  `CellCoord + Transform` from `WorldPos`, activation hooks insert statics (replacing the walkers' `StaticWorld`
  build; `--off physics` keeps it) and terrain heightfield chunks (64 m, 0.5 m) in R0, debug gizmos.
  Verify: headless test (spawn, step, rebase, despawn); `cargo dev view` Balmora with physics debug draw shows statics +
  terrain; walkers unchanged (feet stats, 150-walker probe budget 0.2 ms); fly-through stutter counters not worse.
- [ ] P3 `body.cook` + `items.check` (framework bake steps) and `MwBodyRules` (`mw_bake`): authored `RootCollisionNode`
  else primitive / hull / V-HACD; per-page support and overlap check.
  Verify: `cargo test -p sf_physics --features bake`; `body_report` example on the install (models, shape kinds, hulls,
  overlaps > 1 cm, unsupported items per cell, bake time; full bake stays inside the 2 min gate).
- [ ] P4 Dynamic items (`sf_physics_bevy` + `mw_physics`): `Dynamic` placements as sleeping bodies on R0 activation,
  `HOLD_UNTIL_GRABBED`, weight → mass, debris/prop layers, CCD, coverage freeze, fell-out recovery, persistence through
  `PlacementOverlay::move_to` + `physics` state blob on deactivation.
  Verify: Arrille's Tradehouse: 0 wakes in 10 s with nobody near; a thrown item topples the counter clutter, all rest
  within 3 s; PH-5 (leave, unload, return: items where they rested); dense-town R0 idle step ≤ 0.3 ms. Visual PH-1.
- [ ] P5 Controller on physics (`sf_controller(_bevy)`): kinematic capsule bodies, sweeps against `STATIC | PROP`,
  capped push impulses on props and debris, `ImpactOnCharacter`, sliding on unwalkable slopes (locomotion.md open
  question 3); the mover stays in `Update`.
  Verify: `cargo test -p sf_controller` (push caps, debris never blocks, slope slide); walkers through a cluttered
  floor scatter debris without stalling; a crate blocks then yields to sustained pushing; 150-walker budget holds.
- [ ] P6 Carry, drag and throw: pick ray, kinematic hand + 6-DOF motor joint with force cap, break distance,
  drag mode (ball joint at the grab point, holder slowed) for objects over the carry mass and for limp ragdolls by the
  grabbed limb (dead bodies), `HolderLimits` (carry force, carry mass, drag mass, throw impulse) from the adapter,
  rotate, drop, throw; holder-space interpolation; dev `--fps-cap`; Morrowind holder limits from Strength
  (physics-adapter.md table; owner request 2026-10-12).
  Verify: view-space jitter of a held item < 1 mm at 30/60/144 fps while walking and turning (automated); throw speed
  within 1 % of the cap; carry/drag/too-heavy switch at the Strength thresholds; a 75 kg ragdoll dragged by a wrist with
  no joint stretch > 1 cm (needs P7; until then drag is verified on heavy items). Visual PH-4 (carry + corpse drag).
- [ ] P7 `ragdoll.cook` + limp ragdolls (`sf_ragdoll`, `mw_physics::RagdollRules`): bodies/joints/limits/masses from
  `RigProfile` + skin extents, override files, creature policy table, death handoff with velocity carry-over, sleep +
  persisted pose, placed corpses settle once.
  Verify: cook report for the 3 humanoid skeletons + all CREA skeletons (bodies, joints, failures); no joint stretch
  > 1 cm in a fall down stairs; dead guard on the Balmora stairs (PH-2 first half).
- [ ] P8 Powered ragdolls + blend (`sf_ragdoll`, `sf_anim_graph` `PoseInput`, `mw_graphs` states): implicit motors from
  the animated pose, per-region `StrengthProfile`, world pelvis drive, hit impulse + recovery, balance → stumble /
  knockdown, get-up (face up/down, `Knockout@tail`, `ROLL_TO_BACK` scaffold) with inertialization, budget of 8.
  Verify: tracking ≤ 5 cm RMS / ≤ 5° at full strength while walking; get-up angular velocity continuous at shrinking
  dt; ≤ 0.05 ms per powered character; PH-2 second half (knockdown on stairs, gets up with feet placed).
- [ ] P9 `HitReact` + `SpringBones` ops (animation.md layer 4; `SpringBones` is the base the cloth op extends).
  Verify: `cargo test -p sf_anim_graph` (spring decay, no drift at any frame rate); a guar's tail and an NPC hit below
  A0 react kinematically.
- [ ] P10 Cloth (`cloth.cook` in `sf_character`, `ClothLattice` op in `sf_anim`, `mw_physics::ClothRules`): robe and
  skirt lattices, outfit joints in the merged palette, XPBD in character space with inertia scaling, tethers, max
  distance, leg/pelvis/tail capsules; census of robe/skirt parts.
  Verify: unit tests (hem stretch ≤ 5 %, no particle inside a capsule at run speed, teleport stable); ≤ 0.03 ms per robe;
  visual PH-3 (`character --walk` in a robe, Balmora Temple priests).
- [ ] P11 Traversal actions and interaction points (already slotted for M4 by animation.md layer 3): vault/mantle/ledge
  from `ContextProbes` with warping, `sf_interaction` points (benches, doors, levers). Not physics; may run in parallel
  with P3–P10. Verify: per animation.md "Layer 3".

- Verify (milestone): PH-1 (table knock-over), PH-2 (dead guard ragdoll + knockdown/get-up), PH-3 (robes) signed off
  by the owner; a benchmark entry for Balmora with physics on (idle step, a collapse, 2 ragdolls) within the gates.
  After M4: pose-search node (`posesearch.cook`) once a bring-your-own locomotion library is baked; mesh cloth for
  capes/banners and buoyancy (M6).

## M5–M10

Not broken down yet. Scope per milestone: ARCHITECTURE.md §9; designs:
`../superfargoth/docs/WISHLIST.md`. Break a milestone down here (with a
spec per major system) before starting it.

**Renumbered 2026-10-13 (owner):** M7 is now the RPG core (dialogue, journal, stats/rules, magic, AI, saves; was
M10+); far-field performance → M8, seamless interiors (portals) → M9, combat → M10. Text below written before the
renumbering keeps the old numbers: "M8 portals" here means M9. **Priority: M4, M5, M6** (owner), then M7.

Specs written ahead: M5 game UI (`../superfargoth/docs/specs/ui.md`, `docs/specs/ui-adapter.md`);
the slice to break down is ui.md's "Milestone and first vertical slice".
M6 weather (`../superfargoth/docs/specs/weather.md`, `docs/specs/weather-adapter.md`), stub order: `sf_wind` (+ grass/tree
wind, `wind.paint`) → `sf_weather` + `mw_weather` (presets, climates, forecast) → `sf_atmos_bevy` clouds + cloud shadows →
froxel/height/ground fog → `sf_surface` (wetness, puddles, snow, ash; `WorldMaterial`) → `sf_precip_bevy` → shadow-height
map tier 2. **Can start now (M2 visuals):** weather.md "Near-term visual fixes": A1 CSM to 500 m, A2/A3 far terrain shadows,
~~B1-B3 Ghostfence/uncooked pop~~ (done 2026-10-12; B5 blended far proxies + scriptless activators next), N1/N2 haze and dust.
M8 portals: door portal frames derived and measured (`docs/specs/world-adapter.md` "Portal frames: feasibility": 1,361
portal-ready / 251 adjust / 216 fade-only links); next: fill `LinkEnd.frame`/`extent` in the bake (wiring there), then the
Census Office portal spike. **Spike done (2026-10-12, `--portals`, `../superfargoth/docs/specs/portals.md`):** see-through
both ways + no-fade crossing work on the Census exit pair; frames for 1,612 links fill in 0.02 s; +1.5 ms per open
interior view at 1440p. M8 next: frames in the bake (grounded sills), `sf_portal_bevy` (viewer exposure, motion
vectors in the opening, far-root floating origin, history reset at the crossing), per-view look hooks; check PT-1.
**Owner fixes (portal2, 2026-10-12):** grounded common opening (no void, openings coincide), white flash fixed (sun on
the interior layer + default sky exposure for one frame), viewer exposure (base EV100 7 + `AutoExposure` with a
compensation curve; calibration already one physical scale), position-only crossing gate (back out works, backwards
and strafing); checks PT-2..PT-5 awaiting owner.
**Last pass + parked (portal3, 2026-10-12):** terrain through the door from inside, crossing flash fixed (sky camera
setup a frame late), softer adaptation with live strength keys; seamless portals are an optional setting, **off by
default** (`--portals` / `BEVYRROWIND_PORTALS=1`), parked until M8 (open items in portals.md "Last fix pass"); PT-6.

## Session log

Newest first. One or two lines each: what changed, what was verified.

- 2026-10-13 (clouds2, S1): cloud shadows were never applied (Bevy needs `pbr_clustered_decals` for light textures; now on), clouds 4x too big (weather tile 16 km, now real-scale 8 km + app scales sizes with heights 0.5), static (Morrowind `Cloud Speed` x 10 m/s + two-octave field evolution + rising billows), sun leaked through thick cloud (last 1 % transmittance kept; now snapped to 0; composite order was fine). Nine genus presets + 2-D cirrus sheet; `--clouds` takes genera and Morrowind weathers; `sf_atmos_bevy` example `clouds`; 8 `clouds-*` scenes, CL-1..6 awaiting owner. sf_atmos 14 tests, clippy clean; app clippy clean (mine). Not yet: app screenshots, GPU A/B (shared machine).
- 2026-10-13 (clouds3, S1): ran the clouds in the release game: ground shadows show (pbr_clustered_decals fix confirmed), stratocumulus hides the sun disk and bloom, cumulus still dense from the ground (coverage 0.40 -> 0.34, soft column tops). GPU per-pass timers added (RenderDiagnostics spans); clouds were 1.35-1.70 ms, over the High budget (shadow map alone 0.44-0.69). Now toroidal, time-sliced shadow map (1/16 per frame, `sf_atmos::ShadowSlicer`) 0.04 ms; trace with continuous step length (fixes the horizontal banding), empty-space skipping, lighting LOD, max steps 64: clouds 0.91 fair / 0.79 cumulus / 0.52-0.62 overcast+storm (1440p, RTX 4060 Laptop, shared machine; frames CPU-bound, so pass timers, not frame deltas). sf_atmos 15 tests, clippy clean; CL-1/2/3/6 look-fors updated, awaiting owner.


- 2026-10-12 (slopefix): WK-4 stride collapse on slopes: a path into too-steep ground (or a wall) at a shallow angle slid the walker along the contour while it faced the slope, so the graph played a slow strafe (one leg never plants). Mover now orients to the deflected motion (`Locomotion::deflect`, `DEFLECT_FACING`); headless real-install regression `mw_graphs/tests/slope_walk.rs` (ramps 15-30° up/down, stairs, contour; slide < 1e-5 m/s) + sf_controller test; walkers stride monitor in the log. Open: strafe clip contacts, frozen-contact walkers on flat ground.
- 2026-10-12 (physics-spec, spec only): M4 physics designed: `superfargoth/docs/specs/physics.md`, `docs/specs/physics-adapter.md`, "M4: Physics" (P1–P11), ARCHITECTURE §10. Backend `rapier3d` 0.36.1 direct behind Bevy-free `sf_physics` (bevy_rapier syncs on `Changed<GlobalTransform>`, wrong under big_space; avian main still Bevy 0.19, PR #1070 ports it but no spherical motors); kinematic mover kept; bone-lattice XPBD cloth for robes. No builds.

- 2026-10-12 (cloudjump): clouds jumped at floating-origin shifts: `sync_origin` now runs before `sf_atmos_bevy::CloudSystems::Drive` (was unordered) and the sky's atmosphere placement moved to PostUpdate `SkySystems::Origin` (sea level applied a frame late on vertical shifts). Clippy clean; visual CL-6 awaiting owner.

- 2026-10-12 (helditems): held items as data: `sf_character::{HeldItem, Mount, Hand, Readiness, HeldLight}`, `mw_actors::held` (best weapon by skill/damage + ammo, best shield, torch when dark, `Weapon Bone`/`Shield Bone`, no visible sheaths, LEVI scaffold), `Stance::of_weapon`. 12 + 6 unit, real-install counts (stances, guards, torches); clippy clean. Heavy side open.
- 2026-10-12 actorfx: morph playback (clip curves + face player) in sf_character_bevy, KF companion morph curves (forager 9/9 bound), NPC blink/talk (faces.rs, Y key), LEVC wander; tests: sf_character_bevy 4, mw_anim morph real-install 2, app faces; clippy clean.
- 2026-10-12 (doorpairs): M8 prep, `mw_world::portal`: portal frames (sill origin, +Z into the worldspace, extent) from door model bounds + the landing marker; over 1,828 links: 1,361 portal-ready, 251 adjust, 216 fade-only; arrival forward = rot·(−Z) confirmed (3,307/3,363 face out), markers float ~1.2 m; Census Office exit pair numbers in world-adapter.md "Portal frames: feasibility"; `portal_report` example, 3 unit + 1 real-install test, clippy clean.

- 2026-10-12 (nifskin): NIF skins + geometry morphs → `sf_model` (`mw_nif::skin`, `mw_bake` NIF importer v2): 748 install NIFs cook to `Cooked::Skinned` (556 skinned, 273 morphing; 0 unresolved skins, max rest error 6e-5 m); +4.4 s CPU cook, +58 MB products; `mw_nif` 26 / `mw_bake` 18+2 tests, clippy clean.
- 2026-10-12 (spikes4): streaming gate settled on presented stutters vs a moving median (`sf_diag` `Gate::MaxStutters`, `pacing::stutters`, report stutters/min; `FramePhasePlugin` work counters for attribution; 5 new tests); 15-cell setup matrix (window/pixels/present mode, `target/perf/mx-*`): borderless 1080p flight 21–27, run 3–9 stutters/min, gate fails; `BEVYRROWIND_WINDOW`/`RENDER_SIZE` bench switches, `tools/pacing.awk`.
- 2026-10-12 (pathgrid2): new Bevy-free crate `mw_ai`: PGRD → engine-space `petgraph` graphs (exterior cell offset, directed edges, A*), AI_W `Wander` + OpenMW idle roll + `WanderPlanner`, named scaffold `PathWalkScaffold` driving `sf_controller::PathFollower`. Real install: 1,386 grids / 0 errors, Balmora NPCs 3.3 m median from their grid; mover walks scaffold paths ≤ 0.33 m off, stops ≤ 0.15 m from goals. Workspace deps `petgraph`, `fastrand` (already in the lock). 7 tests, clippy clean.
- 2026-10-12 (spikes): streaming spike attribution (suite hitch rows + Chrome trace): render thread is the critical path; fixed particles re-uploading every mesh each frame (`mw_assets`), terrain respawning all patches on origin shifts (`sf_terrain_bevy`), origin-shift guard (`sf_world_bevy`). Flight 1440p frames > 1.5× median 32 → 9, worst 1.67–1.86×; gate still fails. Visual checks ST-1, ST-2.
- 2026-10-12 (shadows2): distant shadows: CSM 504 m / 2048² (derived), terrain far shadow (march + CPU shadow-height map),
  proxies (`ExtendedMaterial`) and impostors shadowed by the map, dithered cascade end; release A/B: net +0.0-0.4 ms vs before.

- 2026-10-12 (scenes): test-scene harness: `scene <name>|--list` over `scenes.ron` (24 scenes, runs the `view` path), live A/B
  keys + overlay (`live_ab.rs`: F2-F8, hour, F9 camera, F12 shot), `docs/visual-checks.md` (owner pass guide, 26 checks);
  release solari,dlss exe in `../play`. Verified: overlay/keys in Census + Ghostgate from `play`, registry tests, clippy clean.

- 2026-10-12 (anim-creatures): quadruped downhill slide and step-edge heights fixed (whole-body reach correction, probe
  origins led one update); mover traversal flag, Fly/Swim modes, capsule from rig profile. 4 crates' tests + clippy green.
- 2026-10-12 (anim-rig): rig profiles (`sf_anim::rig`, `rig.profile` step, `TwoBoneAnkle`, limb solvers), creature-ready graph
  ops (N-joint limbs, per-girdle body fit, LimbIk/ChainIk/SpineBend/Lean), contacts × leg length, root attitude, motion warping
  (graph side), `RigRef`; 13 of 17 audit items fixed; quadruped 20° slope slide ≈ 0, p95 1.4 cm, pitch within 2.1°.
- 2026-10-12 (dlss): DLSS wired (SR on raster `--dlss`, RR denoiser on `--rt`, `dlss` no longer implies `solari`); 4060 Laptop
  1440p Balmora native 11.9 / DLSS Q 8.9 / P 9.2 / DLAA 11.7 / `--prepass` 13.9 ms; Census rt+RR auto 8.4 (ReSTIR off) vs 15.4 undenoised.

- 2026-10-11 (weather-spec): weather/atmosphere/distant-shadow spec (specs only): `weather.md` + `weather-adapter.md`; traced the
  shadow pop (CSM ends at 300 m, Bevy cuts to lit) and the Ghostfence pop (`ex_gg_fence_s_*` skipped by `mesh.cook`, no far proxy, NIF spawner ignores ranges).

- 2026-10-11 (anim-plan): next-gen animation plan (specs only): warping, full-body IK, context probes, physical animation
  (M4), pose search (later), `rig-profile.md` for creatures, `animation-sources.md` (BYO libraries, license table); 17-item audit.

- 2026-10-11 (character): `sf_character` built: VRM 1.0 humanoid, hierarchy-driven auto-map, auto-align (facing + swing),
  `RigMap` → `RetargetRig`, slot registry, part/wearable records, outfit resolution + key; 19 tests on real Quaternius rigs.
- 2026-10-11 (farfield-lod): far field verified on Morrowind (screenshots `shots/farfield/`): town entities 353k → 48k, Balmora
  1440p 22.2 → 11.9 ms, far CPU 0.15 ms; fixed two Bevy 0.20 pitfalls (upload limiter, depth-prepass dither); d1 400 m.
- 2026-10-11 (controller): `sf_controller` built: Holden springs + exact trajectory, parry3d static world, terrain probe
  (trait over `TerrainHeights`), kinematic mover, path follower, `collision.cook`; 21 tests; 150 chars 0.096 ms (battery).

- 2026-10-11 (anim-graph): `sf_anim_graph` built: RON graphs, `animgraph.compile` (spade Delaunay), evaluator, state machines,
  sync markers, inertialization with target velocity, mirror, procedural ops; 19 tests; A0 update 4.4 µs / 17 joints (battery).

- 2026-10-11 (farfield-ui): game UI spec finished (owner: both themes, both presets, CSS via bevy_flair, build at M5
  on `.bsn`); `ui-adapter.md` written; ARCHITECTURE §4/§5.3/§9/§10/§11 updated. Design only, nothing built.

- 2026-10-11 (farfield-lod): far field built: `sf_lod` (bake steps, lead bands, impostors; 14 tests), `sf_lod_bevy`,
  streaming hook in `sf_world(_bevy)`, `mw_bake::far`; Morrowind far bake 20.6 s. Runtime on Morrowind not yet seen.
- 2026-10-11 (farfield-exterior): Morrowind exterior streams in `view` (terrain + statics + sky), bake 32 s cold;
  Balmora standing, R1/R2 load radii → meshes / median: 768/3000 → 337k / 21.0 ms, 400/3000 → 306k / 16.8 ms,
  768/1000 → 109k / 15.3 ms, 400/1000 → 77k / 9.0 ms, 400/400 → 29k / 7.0 ms (`--near-ring/--far-ring`).

- 2026-10-11 (solari-census): Solari is the "ultra" option; interior ray tracing wired (`view --rt`, load-time
  RT mesh variants); `sf_anim` core + IK/retarget done (28 tests, glTF import, three bake steps).
- 2026-10-11 (m1): M1 runtime done except `sf_diag`: nif-sweep, controllers, particles, billboards,
  AttachLight lights, calibration, root-transform fix; all verified by screenshot (spec `nif-runtime.md`).

- 2026-10-11 (specs): M2 specs written and agreed (world-coordinates, streaming, terrain, bake); WISHLIST early
  decisions settled in ARCHITECTURE §10; M2 broken into tasks. Design only, nothing built.
- 2026-10-11: Wishlist captured in `superfargoth/docs/WISHLIST.md` (perf targets,
  HLOD/impostors, huge worlds, water, surfaces, seamless interiors, cloth,
  animation, DS3-style combat, PBR + texture streaming); milestones M2–M10 re-scoped.

- 2026-10-11: M0 complete and verified on a real Steam install. M1 static
  interiors render; Census Office screenshot checked; rotation convention
  verified; markers hidden by ID; Bloodmoon/Tribunal order fixed via
  `KNOWN_ORDER`. Build setup measured (11 min clean, 2–4 s incremental).

- 2026-10-12 (fbik): full-body IK built: `sf_anim::ik::FullBody` (XPBD particles over the rig profile, limb-root reach spheres,
  then exact per-limb solves) + `FullBodyIk` op in `sf_anim_graph`; biped/quadruped tests green, limits, no flips, 1 = 8 threads;
  7.3 µs biped, 11-13 µs on the 66-joint humanoid and quadruped+head (gate 8 µs: perf follow-up).
- 2026-10-12 (fbik2): FBIK fixed and sped up: the XPBD loop oscillated (period 2) and never converged, so relaxed PBD + root
  residual shift + reach polish; head goals 3 cm → < 0.001 mm; 7.3/13.3/11.3 → 5.0/7.5/8.7 µs (humanoid still over 8 µs).
- 2026-10-12 (nifpop): weather.md bug B1-B3 built: `SubtreeRange` ranges on NIF meshes, `mesh.rest` rest-pose cook in the far
  field (21 lamp/fire models, 4,037 placements), `PlacementSpawner::prepare` preload before reveal (Ghostgate: 59 NIFs, settled
  7.6 s); far rebuild 20.5 s, bake 71 s warm-models. Ghostfence segments are blended-only activators (B5). DLSS defaults to auto.
- 2026-10-12 (clouds): Nubis volumetric clouds built (`sf_atmos` + `sf_atmos_bevy`, Megalophobia math): checkerboard quarter-res raymarch + temporal resolve, atmosphere LUT lighting and aerial perspective, cloud shadows via `DirectionalLightTexture`, floating-origin stable; presets clear/fair/overcast/storm, `--clouds`, `--off clouds`, F10/C keys; +0.6..1.0 ms at 1440p; checks CL-1..6 await the owner.
- 2026-10-12 (probes): context probes built (`sf_controller::ContextProbes`: GASP/ALS-style sweep + down-cast column + room checks, classified by `ContextParams` scaled from the mover; warp targets and hang points; table/stairs/overhang/cliff tests, probe-driven mantle warp ≤ 2 cm); 150 all-blocked 0.79 ms single-thread (over 0.2 ms, gating keeps it ≤ 0.21). FBIK body-only refresh: humanoid 8.65 µs, limb pass next.
- 2026-10-12 (optclips): optional clips in `animgraph.compile` (`sf_anim_graph`): `ClipRef` syntax with fallback chains or `Drop`, structural drops (blend-space samples re-triangulated, states with their transitions, pass-through blends), file checks as recorded bake dependencies so a newly added source recompiles, per-slot `Provenance` + `resolutions` report for export guards; 3 tests, `SFAG` v3. FBIK deferred limb refresh skipped (needs a refresh mode through all four limb solvers).
- 2026-10-12 (animimport): `sf_anim` features `bvh` (own parser; `bvh_anim`/`bvh_anim_parser` rejected: 2019 deps / `cargo` as a runtime dep) and `fbx` (`ufbx` 0.11.5, `cc` builds on MSVC); `SourceSpace` units/axes/handedness, `AnimImporter::settings_key`, `detect_root_motion_joint` + `SkeletonRules::detect_root_motion`; CC0 Blender FBX fixture (129 KB); BVH → auto-map → Biped retarget 0.000 cm / 0.034°.
- 2026-10-12 (mwanim): new crate `mw_anim` (KF/NIF animation → `sf_anim` source clips): `mw_nif::load_kf`, groups from text keys (Start/Stop or key extent; spelling variants folded), one clip per group with exact cubic cuts, sub-action sections, event vocabulary (`footstep`, `attack.hit`, `attack.charge`/`attack.follow` windows, `cast.release`, ...); root motion is on `Bip01`. 162 KFs → 2330 clips, all text keys mapped; `xbase_anim.kf` 135 groups. `anim_report` example stands in for `bevyrrowind anim` (app busy). 5 tests, clippy clean.
- 2026-10-12 (mwrig): `mw_anim::rig`: Morrowind skeletons from NIFs (Bip01/Root Bone subtree, turned to face +Z at import), humanoid maps for the four Bip01 skeletons via `sf_character::auto_map` on `Bip01*` joints + root motion `Bip01` (all clean), four blend masks, Quaternius round trip (body ≤ 0.07°, < 1 cm), creature profiles with per-model hints (nix-hound, cliff racer, centurion spider) and `CreatureMover` from CREA flags; example `rig_report`; guar/kagouti are bipeds (animation-sources.md fixed). No framework change.
- 2026-10-12 (rigfix): `sf_anim::rig` leg walk stops below a joint carrying support branches on both sides of the midline (spider body, legs-only pelvis; toes never split a leg); synthetic regression test; centurion spider hint removed from `mw_anim` (detects alone, real-install test asserts it). Kagouti biped read is correct (legs on the arm bones); its tail above the neck girdle is missed (rig-profile.md known gap).
- 2026-10-12 (collision): `mw_bake::collision`: Morrowind rules for `collision.cook` (skinned / `NC…` marker / empty `RootCollisionNode` / `MRK` → none; `RootCollisionNode` geometry; rest pose for animated models; else LOD1), registered in `model_baker`; `mw_nif` reads `NC` markers as OpenMW does (case-insensitive prefix, NCC too). Real install: 3,168 authored, 2,497 LOD1, 150 rest pose, 794 none, 759k tris, +8.5 s cold / 0.12 s warm. `collision_report` example; app hook left (spikes2 busy). sf_anim_graph/sf_controller/sf_character tests pass after rigfix.
- 2026-10-12 (audit): industry gap audit of streaming/rendering/LOD (`../superfargoth/docs/specs/industry-audit.md`): 34 techniques vs UE/Unity/Bethesda/RAGE/Decima/OpenMW; Bevy 0.20 per-entity costs read from source (CPU visibility O(entities x views incl. 4 cascades); `NoCpuCulling` keeps GPU `VisibilityRange` + frustum culling); missing: size-based draw distance and size-tiered spawning (owner's catch), shadow-caster size rules, precombined near cells, pipeline warm-up. Jobs J1-J9 in Next up 6-7; specs gain an "Industry comparison" section. Docs only.
- 2026-10-12 (mwactors): `mw_actors` (new, Bevy-free): Morrowind `CharacterSource` + `PartLookup` for NPC_/CREA (race skin per sex/view with male and 3P-arm fallbacks, head/hair, ARMO/CLOT wearables with OpenMW part priorities as layers + reserved/hidden slots, auto-equip by effective armor rating, autocalc skills, race scale, skeleton by sex/beast, first person, creature model as skeleton + body); `sf_character` hook `BodyPart::node_filter` + `PartKind::Rigid { joint, offset }` (ARCHITECTURE §10). Tests: mw_actors 6 + 1 real-install (all 3,041 NPC_ / 396 CREA), sf_character 20 + 1 + 2; clippy clean. Not committed.
- 2026-10-12 (skins): `sf_model` skin data (Bevy-free): `MeshData::skin`/`morph_targets`, glTF importer v2 reads skins (all JOINTS/WEIGHTS sets, inverse binds) and morphs (sparse, `targetNames`), `mesh.cook` → `Cooked::Skinned` (`.sfmesh` v2, v1 still read, static cooks unchanged so no re-cook), per-part submeshes with node paths, palette by name + inverse bind, joint-center LOD attribute, rigid → single-joint skins, `CookedSkinned::rigid` owns mirrored winding. sf_model 22 tests, sf_lod 13; clippy clean. One-line compile fixes in sf_lod, sf_controller, mw_bake, sf_model_bevy (not built: Bevy). Not committed.
- 2026-10-12 (sizecull): J1 size-based draw distance built (`LodScreen::draw_distance`, `size_culled_end`, `CellProxies::kept_radius` L1 clamp, `NO_SIZE_CULL`, app `--quality` presets + `--off sizecull`); histogram: 3 % of exterior placements culled at High, 8 % Low; A/B 2 rounds: entities and GPU unchanged, Balmora CPU floor -0.3 ms, stutter gate still fails; audit's L1-clamp math corrected (diameter, not radius).
- 2026-10-12 (j9m3): J9 "Industry comparison" sections for the M3 character specs (superfargoth animation, character, locomotion, rig-profile; bake.md gap note) vs UE5/Unity/Creation/RAGE/Decima/Ubisoft/OpenMW, Bevy 0.20 skin/morph costs read from source; 7 gaps ranked, jobs M3-a..g proposed in industry-audit.md "M3 follow-ups (j9m3)" (P1: morphed parts never merged, ORCA avoidance, skinned GPU cost per pass + cascade rule, montage-style `Slot` node). Docs only, not committed.
- 2026-10-12 (health): dev-loop health check, gates in ARCHITECTURE §8b (perf.md "Dev loop"): `cargo dev` no-op 0.83 s, touch 3.3-8.1 s, body edit 6.2-10.3 s, clean 8 min 38 s (was 11 min); bake cold 47.7 s, warm 2.41 → 1.75 s (parallel report decode; `bake` prints every step). nifpop's 71 s was a partial rebuild, not a regression. `nif-sweep`: no skin/morph entries left. Owner decision: nightly (`-Zthreads`, cranelift) for dev builds.
- 2026-10-12 (m3a): M3-a morph-bearing parts: `.sfmesh` v3 morph layout (per-submesh target lists, all-zero targets stripped, expression targets only on LOD 0-1; v2 retired, `KEY_FORMAT` 1 → 2 = one ~38 s re-cook), `MorphKind` (adapter-set; NIF morphs all Expression), `sf_character::assemble` (Bevy-free per-material merge, morphed submeshes own mesh, shape weights baked, `ShapeWeights` cache key). Memory: library outfit 21.2 → 3.4 MB; Morrowind NPCs unchanged at LOD 0 (heads own material, 2 targets), head morphs gone at LOD 2; finding: MW outfits keep 11-15 materials after merge. sf_model 24, sf_character 23+3, mw_bake 18+2, mw_actors 6+2 tests; clippy clean. Not committed.
- 2026-10-12 (interiors): M2 interiors' index + door links in the exterior view: all 1,328 interiors baked as `Single` worldspaces (0 unresolved link ends; +8.3 s cold, no-op ~2.0 s), Space crosses doors with the fade scaffold (0.63 s in, 1.02 s out), per-worldspace look and render layers, `--use-door [N]`, scenes `door-balmora-guild[-auto]` (DR-1/2); streaming.md industry comparison (Bethesda/OpenMW vs seamless).
- 2026-10-12 (orca): M3-b ORCA local avoidance in `sf_controller::avoid` (from van den Berg et al. 2011; `dodgy_2d` 0.5.5 evaluated and not taken: glam 0.29 + rand 0.9 duplicates, random tie-break, per-call allocation). Additions: player non-reactive, idle give-way, slack split opposite to responsibility, opposing pairs yield by priority or a heading rule on the route direction, keep-right bias, spatial-hash rings. Real mover: head-on gap 4.3 cm, give-way walker swerve 7.5 cm vs 0.74 m sidestep, 1.5 m doorway 4 + 4 swap 13.2 s (deadlocks without yielding), batched = single bit for bit; 150 agents 0.054 ms street / 0.087 ms dense (release, 1 thread). 8 tests (1 timing, ignored), clippy clean, no new deps. Not committed.
- 2026-10-12 (skinloader): `sf_model_bevy` loads `Cooked::Skinned` per bake.md "Bevy mapping" (Uint16x4 joints, Float32x4 weights, shape targets baked, expression targets as Bevy morphs, one inverse-bindposes asset; 256-joint / 256-target / morph-texture limits fail loudly) and spawns it the bevy_gltf way (`SkinnedMesh`, `DynamicSkinnedMeshBounds` generated before compression, per-mesh `MeshMorphWeights::Value` from root `ModelMorphWeights`). Example `skinned` (Quaternius UAL2 cooked vs bevy_gltf: identical), app `view <nif> --model --skinned [--skeleton]` scaffold (K cooked/NIF, M morph): guar and dark elf head render. Visual checks SN-1..3, scenes skin-guar/skin-head. sf_model_bevy 9 tests, clippy clean. Not committed.
- 2026-10-12 (doorfix): M2 door polish: `FreeCamera` yaw/pitch resynced on `TravelArrived` (no snap on the first mouse move), arrival heading from the marker's Z angle (YZX decomposition, unit test), residency cache for the worldspace just left in `sf_world::Scheduler` (back out 0.64 s, was 1.02 s; in 0.63 s), interiors open in the streamed view (`--single-cell` keeps the old viewer), scene `door-census` (DR-3); release `solari,dlss` refreshed in play/.
- 2026-10-12 (slotnode): M3-d montage `Slot` node in `sf_anim_graph` (`slot` module, `SFAG` v4): slots + slot groups, binding (`slot_clips`) or late-bound clips, interrupt/queue/priority, loops, inertialized blends, events/warp/root motion, lifecycle notices; 5 tests (36 + 2 ignored in the crate), clippy clean; animation.md "Slots and montages" + G4 resolved. Not committed.
- 2026-10-12 (mwgraphs): M3 graph assets: new Bevy-free `mw_graphs` (humanoid, first-person, creature templates built in code + shipped RON with a golden test; `AnimSet` CompileSource over KF groups with `Group@section`/`@tail` refs and fallback chains; `Sim` headless simulation; PlayGroup/LoopGroup adapter; creature coverage report). Real install: 4 humanoid skeletons + 89 CREA models compile; every state, stance, footstep, attack window and the scripted slot checked headless. `mw_actors`: real graph paths, biped creatures → humanoid graph. 4 unit + 4 real-install tests, clippy clean. Not committed.
- 2026-10-12 (animbake): M3 animation through the bake: `mw_bake::anim` (actor sets from `mw_actors` recipes, `mw.anim.groups` step, two-phase bake of `skeleton.cook`/`anim.cook`/`animgraph.compile` with `MwAnimRules`, Bevy-free `AnimLibrary` loading API), `mw_anim::NifAnimImporter` + `rig::skeleton_source`, `mw_graphs::builtin_source()` (lowest VFS layer), `sf_anim::AnimBake` per-session import memo. Real install: 96 skeletons, 3,987 clips, 96 graphs all compile and load; cold 13.5 s, warm 0.31 s; 1 vs 16 threads byte-identical. `anim.cook` ≤ 5 s gate missed (rate search on 1,150 clips that never reach 0.1 mm). Not committed.
- 2026-10-12 (portal): M8 portal spike (scaffold `portal_spike`, `--portals`): render-to-texture portal camera (HDR, oblique `near_clip_plane`, far worldspace layer) + screen-space quad on the slab front (Morrowind doors lie on solid walls), larger slab swings/smaller hides, crossing via `DoorLink::cross` 0.15 m ahead of the quad with heading+velocity kept; frames for 1,612/1,828 links at startup in 0.02 s; light confinement read from Bevy source (all per view except light probes; sun moved off layer 0); fixed interiors using the sky's ambient override; +1.5 ms per open interior view at 1440p (clean interleaved A/B); spec `superfargoth/docs/specs/portals.md`, ARCHITECTURE §10, visual check PT-1, scene `portal-census`.
- 2026-10-12 (animbevy): M3 `sf_anim_bevy` (new crate): `spawn_rig`, `Animator`/`AnimAsset`, PostUpdate `Lod → Probe → Evaluate (par_iter) → Apply` before propagation, per-bone write-back only on change (Bevy `animate_targets` shape, pose computed once per skeleton), root motion, `AnimEvent` messages + `AnimNotify` observers, tiers A0–A3 (projected height @1440 ref, hysteresis, off-screen A3, A2 half rate) + budget allocator, counters, bone gizmos; example `blendspace` (Quaternius UAL1, 7-clip 2D blend space). App: builtin graph VFS layer, `bake` runs the animation bake (96 graphs, 0 failures), `view --skinned --animate` plays baked graphs on the guar and dark elf skins (scenes anim-guar/anim-body, AN-1..3). 4 tests, clippy clean. No timings (owner on the machine). Not committed.
- 2026-10-12 (animcook): `sf_anim` `anim.cook` v3: rate search skips rates whose resampling error alone exceeds 1.5x the bound, early-out measurement, source-pose reuse, guessed bit search. Survey (new ignored `mw_bake` test `anim_cook_survey`, 3,307 clips): 35.9 → 24.1 s CPU; real bake `anim.cook` 131 → 54.6 s summed (≈3.4 s at 16 threads, indicative: owner was testing; dedicated timing run owed), cold 13.5 → 9.0 s, 1 vs N threads identical; no clip newly over 0.1 mm. Cause of the over-bound clips: linear keys (kinks) at off-grid times, very fast cubic translations, root-motion yaw extraction, source discontinuities. Intermittent bake deadlock (OnceLock inside rayon) seen twice, pre-existing. Not committed.
- 2026-10-12 (portal2): PT-1 owner fixes on the Census portal: grounded common opening (`mw_world::portal::{floor_offset, common_opening}`, floor rays vs cell statics), white flash root causes (sun queued onto the interior layer for the crossing frame; new `SkyCamera` got `Exposure::default()` for a frame, fixed in sf_sky_bevy), viewer exposure (base EV100 7 on all cameras + Bevy `AutoExposure` with a schedule-matching compensation curve; `--scheduled-exposure` for A/B; calibration verified one physical scale), position-only `Gate` crossing with mirrored trigger (unit test forward/backward/strafe both ways), history reset at the crossing, `portal trace` log, `--portal-walk`; scenes portal-noon/-sunset/-night(-in), portal-walk; checks PT-2..PT-5.
- 2026-10-12 (portal3): last portal pass, then parked until M8 as an optional setting (off by default; `--portals` or `BEVYRROWIND_PORTALS=1`, F1 shows it): terrain seen from inside (portal camera carries `TerrainCamera`, terrain origin follows the terrain camera's root); crossing flash found with the extended `portal trace` (+ per-frame presented-luma readback, raster and Solari): `worldspace_look` raced `setup_sky_cameras`, the crossing frame drew the office with the noon atmosphere (luma 21.8 -> 177.5 -> 38.5; now 21.8 -> 38.8); adaptation softened (strength 0.7 about the interior point, 3/2 EV/s) with live `-`/`=` keys; PT-6; M8 leftovers in portals.md.
- 2026-10-12 (bakelock): fixed the intermittent bake deadlock (rayon worker stealing a job that waited on a product `OnceLock` it was initializing further down its stack). `sf_bake` now plans the reachable graph and runs it with a dependency-counting scheduler; inline resolution never waits (in-flight products are rebuilt, `StepStats::duplicates`). Regression test `sf_bake/tests/concurrency.rs` (watchdog; hung on old code); `mw_bake` anim bake: clips then graphs (0 duplicates, same 3,987 clips). Rule in bake.md "Concurrency", ARCHITECTURE §10.
- 2026-10-12 (charbevy): `sf_character_bevy` (Character component, CharacterContent hook, off-thread assemble, OutfitCache, no-pop swap, CharacterSpawner) + app `actors.rs`, `character` lineup command, actors in streamed cells, `mw_actors::leveled_pick` scaffold; scenes CH-1..CH-4 awaiting owner. No benchmarks (owner on the machine).
- 2026-10-12 (kftangents): audited NIF keyframe conventions against OpenMW and the base-game data (new ignored `mw_anim` test `kf_curve_conventions`). Quadratic tangents are per segment, as built (747 vs 4 vs 2 C1 matches); the cook survey's 8-16x tangent/chord segments are authored data (zero chords, root-motion wrap snaps), not a bug. Bug fixed: TCB tangents lacked the Kochanek–Bartels key-spacing adjustment (`mw_nif::anim::Curve::tcb`; velocity jumps > 2x at 106 of 507 continuity-0 keys, now 0); KF/NIF anim importers v2. Survey: 195 creature clips change, 69 → 131 of them over the 0.1 mm bound (all clips 788 → 850), the faithful curves are harder to resample. Open: TCB quaternions slerp (1,828 channels), root-motion snaps (nif-runtime.md).
- 2026-10-12 (kfsnaps): translation snaps (≤ 0.1 s, ≥ 5 cm, ≥ 4× neighbor speed; root wraps between creature groups) are curve breaks in `mw_anim` (`convert::snap_breaks`: snap segment linear, snap-tainted neighbor tangents one-sided; NIF importer v6, KF v5; new ignored audit `kf_snaps`). Cut-clip overshoot max 3.77 → 0.84 m (dreugh `Walkforward` gone), over 10 cm 44 → 33. Cook survey: 142 clips change (33 better, 6 worse), over 0.1 mm 853 → 852, p99 2.9e-3 → 2.4e-3 m, 129.5 MB, 1 vs 2 threads identical. TCB quaternions: squad vs slerp measured (per segment p50 0.7°, p90 4.1°; slerp kinks p50 73°/s); kept slerp (OpenMW-compatible), squad only after an owner A/B (nif-runtime.md).
- 2026-10-12 (walkers): M3 demo "NPCs walk the streets": new `sf_controller_bevy` (`SfControllerPlugin::<G: GroundProbe>`, ORCA → kinematic mover → transforms in one Step system, animation feed with adapter-named params + batched per-leg ground samples, `controller/...` counters, gizmos), app `walkers.rs` (terrain + R0 `collision.cook` statics via activation hooks, `PathWalkScaffold` per AI_W wanderer, fidgets, camera as non-reactive obstacle, fliers in `Fly` mode, `--off walkers`), `mw_humanoid` FootPlacement (re-blessed RON, rebake 28 graphs). Balmora: 472–478 movers, 65–75 wanderers walking, all but 5 grounded. 3 headless tests; clippy clean; scenes wk-balmora/wk-crossing/wk-stairs (WK-1..4 awaiting owner); play/ refreshed. No timings (owner on the machine). Not committed.
- 2026-10-12 (smoothbody): smooth-bodies research + Bevy-free prototype: `sf_character::smooth` (feature `smooth`: `smooth_body` seam weld + harmonic band inpainting, `transfer_weights`, `find_seams`, `penetration`), `mw_actors` example `smooth_body` (dark elf male + iron armor, 5 poses, metrics + OBJs in `shots/smoothbody/`); finding: smooth the visible outfit as one segmented set (elbow 90° dressed seam opening 56.5 → 1.6 mm), body-to-armor transfer is worse for bulky armor; spec `smooth-bodies.md`, ARCHITECTURE §10 (default ON, vanilla option). Live A/B is next.
- 2026-10-12 (smoothbody2): smooth bodies made robust (Bevy-free, `sf_character::smooth`): overlap (volume) links with open-rim rejection, nearest-partner links, authored skins pinned, seam followers (child follows parent, rigid follows authored), overlap weight copy, sorted adjacency (bit-identical across threads). Survey `mw_actors --example smooth_survey` (all 20 race × sex naked + 8 outfits × 3 bodies, 7 poses incl. walk/run frames): max seam opening e.g. dark elf m 55 → 9 mm, Khajiit 177 → 5, Argonian 183 → 16, pokes about 3x fewer, ~10 ms per outfit; open: female knee overlap (264 → 32-42), armor wrist/pauldron pieces, beast tails with outfits, robe ↔ wrist (spec "Survey"). Nothing committed.
- 2026-10-12 (walkers2): walker correctness: processing range = R0 coverage (`sf_world::Scheduler::covers`, `sf_controller_bevy::Dormant`; industry: OpenMW actors processing range, Bethesda AI process levels, UE significance/animation budget), `MwGround` per worldspace (interiors walk their own PGRD and statics, no exterior terrain indoors), race scale in foot probes/feed (`Mover::body_scale`), `PathWalkScaffold` stuck give-up + replan. Tests: sf_world covers, sf_controller_bevy dormant, mw_ai stuck + real-install interiors; clippy clean. Scenes wk-interior (WK-5), WK-6. Not committed.
- 2026-10-12 (nodecreatures): creatures animated per node (scrib, shalk, bonelord, centurion sphere...) and the rigid parts of mixed creatures now spawn, animate and walk: `sf_model::cook_rig` / `mesh.rig` (rigid skinning; chosen over parented mesh entities, ARCHITECTURE §10), `mw_anim` skeleton takes rigid root branches (`NifAnimImporter` v3); CREA census test (89 models, 0 skipped, 1 partial); NC-1/NC-2 awaiting owner. No benchmarks (owner on the machine).
- 2026-10-12 (feetfix): clothed NPCs' shins were missing (Fargoth's pants ended a foot above his shoes; not a foot IK, root height or rig regression: naked Fargoth and the root were fine). Cause: rigid body parts lost their NIF root transform (dropped for placed objects since M1), which the original engine keeps under the bone; pants ankle parts hang 37 cm up the shin through it (knee 7 cm, groin 12 cm). `mw_nif::Model::root_transform` keeps it, `mw_actors::PartModel` probe puts it in the rigid offset (mirror above it). Test `rigid_parts_keep_their_root_transform_and_close_the_leg` (real install); visual check CH-1b.
- 2026-10-12 (smoothbody3): smooth bodies vs the 4 failure groups, rerun on corrected input (feetfix restored NIF root transforms): female knee failure was misplaced parts (now 3.5 mm, vanilla 46-53); new per-seam vanilla guard `sf_character::guard_seams` + `seam_openings` (probe poses; reverts 25 of 474 seams, 0 seams worse than vanilla, robe ↔ wrist 12-14 → 0.1-2.9 mm, glass bracer seams back to vanilla); sibling overlap order, leaf-only sibling links, authored rims link only to their nearest part (tails with netch/pants 44-60 → 14-18 mm). Trade-offs: naked Argonian tail ↔ groin 16 → 46 mm, iron on imperial f 8 → 18 and daedric on argonian 35 → 66 (guard). Open: tail root ↔ pelvis armor 44-66, pauldrons 13-22 (spec "Survey rerun and guard"). Nothing committed.
- 2026-10-12 (actorfx2): creature NIF effects: visibility/material controllers as clip curves on the animation timeline (as OpenMW: whole-body animation time), joint-scale hiding + per-character material copies (`actor_fx.rs`), billboard joints in rigs (heart_akulakhan 6 parts bound, face the camera); census `mw_bake/tests/actor_fx_real_install.rs` (BODY parts: no controllers, billboards or particles; CREA: 101 vis controllers in 13 models, 30 material in 6, 1 billboard model, particles in 16; no creature root transforms). Left: culled geometry not baked, creature particles. AF-4..6 awaiting owner.
- 2026-10-12 (j9rest): J9 finished (docs only): "Industry comparison" sections in superfargoth terrain, sky-lighting, weather, world-coordinates, diag, ui, far-field, bake, portals, streaming and all seven bevyrrowind adapter specs. Gaps ranked in industry-audit.md "Follow-ups (j9rest)": P1 raster AO + contact shadows (after J5), object-terrain blending, DLSS reactive mask for transparents (Bevy 0.20 passes none); P2 soft particles, interior irradiance bake, hex tiling, placement groups, bloom/clock-lock/A-B stats, subtitles/safe area, portal audio. Proposed jobs R-a..R-h there; Next up untouched.
- 2026-10-12 (smoothlive): smooth bodies live: `sf_character::smooth_outfit` (smooth_body + guard_seams over probe poses, rest-space write-back) in `sf_character_bevy`'s build task, `assemble` takes pre-decoded parts (`AssemblyPart::decoded`), `SmoothBodies` in `CharacterSettings` + `OutfitCacheKey`, `CharacterContent::smooth_probes` (`mw_actors::probes`); app `--bodies vanilla|smooth`, B toggle (no-pop swap, both looks cached), `character --walk` and `id+item` outfits; fixed `character` panicking in DLSS builds (no DlssProjectId). Lineup of 7 walking: 66/112 parts rewritten, guard reverted 2 seams, 5-116 ms per outfit on tasks (indicative). SB-1..SB-5 + scene `sb-balmora` awaiting owner. Nothing committed.
- 2026-10-12 (missingparts): missing boots/armor: install-wide equipment audit (`mw_actors/tests/equipment_audit.rs`: 3,041 NPCs vs the original slot rules, 1,765 visible parts cooked + placed). Fixed: 18 rigid part models (iron/glass/bonemold/adamantium/orcish boots, common shoes 1-2, ...; 392 NPC outfits) skipped by `mesh.cook` for a root keyframe controller, now cooked as `mesh.rig` (`ActorSets::rigid_parts`, rebake needed); integer unarmored rating in auto-equip (+52 pieces); leveled-list worn items via the scaffold pick (+252). The lineup Nord's bare legs: visual-check typo `iron_boots` (record `iron boots`), fixed. Legit drops classified (actors-adapter.md "Equipment audit").
- 2026-10-12 (animfix): walkers' sliding feet, walking in place and pelvis-on-the-ground blow-ups: the feed now sends measured velocity/turn (not intent) with moving/running hysteresis; humanoid SpeedMatch 0.5-1.5 + StrideWarp (clip table: walk 2.20, run 3.18 m/s vs NPC 1.86-2.43); `FootPlacement` guards (root cause: locks drifting past leg reach dragged the pelvis down; regression test reproduces it), `FootStats` counters; graph format 5 (rebake); `character --walk` plays as a treadmill; play/ gains `bevyrrowind-raster.exe` (no solari/dlss).
- 2026-10-12 (terrainblend2): object-terrain blending (R-b) built: terrain bindings renumbered to 100+, surface shading moved to `surface.wesl` (`terrain_surface`, shared), `GroundBlendMaterial` = StandardMaterial + `GroundBlendExt` (`ground_blend.wesl`: height above `height_bicubic`, height-broken edge, triplanar on steep object faces instead of the RVT's top-down streaks), `GroundBlendPlugin<M>` swaps marked parts in the terrain's worldspace; `sf_world_bevy::GroundBlendModels` + `GroundBlend` marker; adapter rule `mw_assets::ground_blend_band` (rocks 0.6, buildings 0.4, flora 0.3 m). `--off terrainblend`, F11. Raster only, proxies excluded. Scenes tb-balmora/tb-bittercoast/tb-ascadian, TB-1..3 awaiting owner.
- 2026-10-12 (heldrender): held items rendered: `sf_character_bevy::held` (props on Weapon/Shield Bone joints, readiness mounts, light slots, visibility chains), app `held.rs` (torches when dark by the sky clock, LIGH light + flicker, 12-light budget, stance feed), `mesh.rest` for held models; `character --drawn --night` + R/N keys, scene hi-balmora-night, HI-1..4. Tests + clippy clean; 3 screenshots.
- 2026-10-12 (actorparticles): particles on actors and held items (`actor_particles.rs`: NIF `#Animation` + per-node anchors at the nearest skeleton joint or prop root, `ActorEmitter`, `ParticleRange` 60 m, vis-curve gating; `mw_nif` local-space flag 0x80 + `Simulation::carry` world-space particles), skinned held props on their own skeleton (bows; `CharacterContent::node_transforms`, `HeldSkinAssets`), carried-light near field (`HeldLightTuning` 0.7 m / 0.7). Tests + clippy clean; AP-1..3 awaiting owner.
- 2026-10-12 (animcheck): M3 health check on the combined tree. Tests pass for the 9 superfargoth animation/character crates (`--all-features`) and mw_anim, mw_graphs, mw_actors, mw_ai, mw_nif, mw_bake and the app (real install). Fixed: `sf_anim_bevy` / `sf_controller_bevy` `debug` gizmo systems panicked headless (now `run_if(resource_exists::<GizmoConfigStore>)`), one clippy lint in the mw_ai test. Clippy clean (except particles2's in-progress `particles_real_install.rs`). Smoke runs (`shots/animcheck/`): lineup 7/7 dressed and posed, Imperial guard's tower shield sideways across the thighs (HI-2); wk-balmora/wk-stairs walk (0 pelvis clamps, 0 non-finite; "walks given up" ~1/s); guar graph idles. Crowd street scoped in M3, not built. `mw_nif` actor import marked done.
- 2026-10-12 (particles2): NIF particles finished for M1: `NiBSPArrayController` array emitters (census `mw_bake/tests/particles_real_install.rs`: 42 controllers, all flag 0x8, none at vertices; random node of the emitter subtree, on its bone, as OpenMW), live emitter nodes for actors (`LiveAnchor` per system) and keyframed placed models (`NifNode`), emission windows on the model's controller clock (`emits_at`); actor windows wait for `emit:` clip curves (scaffold `ACTOR_EMISSION_SCAFFOLD`). nif-sweep: unsupported left = path controllers 5, embedded textures 2, planar collider 1. PA-1, PA-2 awaiting owner.
- 2026-10-12 (particles3): actor particle emission windows on the animation clock: `mw_anim::effects` `emit:<particle node>` step curves (`emission_keys` per controller cycle, `emission_names` numbers same-named `PArray` nodes; whole-range windows emit throughout), NIF anim importer v7 / KF v6, re-bake (anim.cook 3,987 rebuilt, 87 s wall at BelowNormal with the owner's game open: not a timing); `actor_particles.rs` maps systems to curves and writes `mw_assets::ActorEmitter::emitting`, `ACTOR_EMISSION_SCAFFOLD` removed. Census test `actor_emission_on_clip_timelines`: every creature system emits in some clip. Visual checks PA-1 updated, PA-3 new (awaiting owner).
- 2026-10-12 (rootmotion): creature root motion made consistent (owner note on NC-1): 15 CREA skeletons travel on `Root Bone` (scrib, shalk, forager, bonelord, centurions, cliff racer, netches, dreugh, fish, mudcrab) and had no root-motion joint, so their walk/run travel stayed in the bones (walk off, snap back) while the 74 `Bip01` skeletons stepped in place. `mw_anim::rig::root_motion_joint` (Bip01, else Root Bone; NIF importer v8, KF v7); one rule for all actors: travel on the root-motion track, mover drives in the world, lineup stores it (key 3 applies). Census `mw_bake/tests/creature_root_motion_real_install.rs`: 507 clips, 0 flagged, no-extraction control per skeleton. Lineup screenshots at 3.0 / 4.1 s walking: every creature on its mark. Cache already re-cooked by dremora's 23:45 bake (stacked importer versions); release `bake` no-op 3.2 s. NC-1 awaiting owner. Not committed.
- 2026-10-12 (torchfix): held torch (HI-4, AP-1 failed): torch arm = `Torch` group looped on a `left_arm` layer (`mw_graphs` `params::TORCH`, `torch_layer` between `upper` and `full` slots; RON re-blessed; fed by `held.rs`); carried light dimmed to read "fully lit" at radius/3 (`LightCalibration::fully_lit_distance`, `HeldLightTuning::fully_lit_fraction`, source radius 0.12 m); torch model `mw_assets::self_lit`; NIF emissive now exposed (`mw_assets::set_emissive`: Bevy 0.20 `emissive_exposure_weight` 0 had left it ~150x). Tests + clippy clean; HI-4, AP-1 awaiting owner. Open: placed lights blow out at night (EV 7 calibration vs night EV ~1).
- 2026-10-12 (physics-p1): owner confirmed the five M4 physics decisions (ARCHITECTURE §10) and added Strength-based carry/throw limits plus drag mode incl. corpses by a limb (physics.md "Carry and throw", physics-adapter.md table, P6); built M4 P1 `sf_physics` (Bevy-free rapier3d core, settle-on-insert, whole-cell rebase, `PhysicsProbe` = `StaticWorld` on fixtures); spike GO on rapier with impulse joints (idle 0.001 ms, active 0.44 ms, rebase 1.4 ms; multibody misses at 3-4 ms), shared machine.
- 2026-10-12 (dremora): "dremora has no legs and biceps": actor visibility curves are now rest-neutral (`hide:<node>` for nodes shown at rest, `show:<node>` for hidden ones; `mw_anim::effects::hidden`), since the graph outputs 0 for a curve no playing clip carries and biped creatures idle on `xbase_anim.kf` groups that carry none of theirs (the old `vis:` 1-shown curves hid all 12 dremora armor parts, 2,022 tris, on all 9 dremora records). NEW `mw_bake/tests/creature_parts_real_install.rs` (89 CREA models: 331k source tris, 92k hidden in the file by design, 239,541 cooked, 0 lost, 0 hidden at idle, 0 material curves absent from a playable clip). Left: 14 controller-shown hidden shapes (heart shields, fabricant pieces, sleeper skull, death dust) still dropped by `to_model` (nif-runtime.md). Importers NIF 10 / KF 9, re-baked. AF-7 awaiting owner.
- 2026-10-12 (seams): CH-1b seams diagnosed (`mw_actors` example `seam_report`): legacy pieces are closed shells or overlapping rims, not shared vertex loops, so the 2 mm weld fixed only shin/wrist; added `sf_character::blend_seam_normals` (proximity normal blending within 4 cm, both sides meet at one normal; heads for shading only, in bind space) in `smooth_outfit` (Smooth mode). Left: per-piece texture colour steps and zigzag rims are art; options (seam colour levelling, rim shrink-wrap) in smooth-bodies.md "Seam shading"; SB-6 awaiting owner.
- 2026-10-13 (walkers3, partial): frozen walker contacts traced to masked overlays (torch arm, crossbow/spell stance idles) overriding every curve incl. leg contacts; fixed with curve-joint links (`CompiledGraph::curve_joints`, masked curve blend) + regression tests. Contact cook rewritten (height over the loop cycle + speed vs ground and stance reference, hysteresis, anim.cook v4): strafes 4 -> ~50 % planted; one uphill-25° slide regression open, not re-baked. Give-up diagnostics (steep/wall/other); fix plan in .sessions/walkers3.md.
