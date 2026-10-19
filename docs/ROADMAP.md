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

0. **Owner go (2026-10-18), start these next, in parallel (max 3 agents), to "works + committed", no polish:**
   M5 **U1** (`sf_ui` + `sf_ui_bevy`; the input crate is allowed now, an exception to the crate "update all" batch;
   pick `bevy_enhanced_input` or leafwing in ui.md OQ4), M4 **P10** cloth, M6 **WD1** wind + **WE1** weather core.
   The owner wants the big roadmap carried on, not polish of existing systems: log leftovers, don't chase them.
   **Added (owner go 2026-10-18): TS1 synthetic test levels**, before P10's last slice (PH-3 needs it, and the
   traversal course tests after the animation study): a `scenes.ron` entry may name a synthetic level instead of a
   cell: greybox geometry (boxes, ramps, stairs, ledges, gaps), Morrowind actors by id with an outfit and a scripted
   path/action loop, props and interaction points, sky/light rig; same viewer, BRP, F1 A/B keys and `--screenshot`;
   loads in seconds without world streaming; RON first, Blender → glTF components (Skein, if on Bevy 0.20) later; the
   same level runs headless for scripted-input tests (faith-runner course tests, research/animation/01).
   **Animation study** (owner 2026-10-18): `../superfargoth/docs/research/animation/README.md` (R1-R11, research-only
   agents, 2 beside the 3 build lanes); its R11 rewrites the traversal/action spec before P11 starts.
   **Queued after these (owner go 2026-10-18):** M5 **U2** (`sf_ui_css`), M6 **WE2** (`mw_weather`) then **WE3**
   (`sf_weather_bevy` + app), as build slots free.
   Items 1-6a below are older; don't pick them over item 0.
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
6a. **Owner visual pass 2026-10-13 (play/ built from HEAD 27a3f86; notes only, nothing fixed yet).** Read before picking work:
   - Fog/froxels: foliage (impostors?), lava and the Ghostfence render on top of the fog: visible at any distance. **Fixed (fogfix 2026-10-19, superfargoth 8aa4fba + this commit):** cause = blended/additive NIF materials (Morrowind leaves: blend + test ref 0; Ghostfence field) draw after the fog composite as plain StandardMaterial; now FroxelFogMaterial (`transparent_fog.rs`, `--off transfog` A/B), far range continued past the froxels; impostors were already fogged (alpha-mask, depth). Leaves as alpha-test **done (leafmask 2026-10-19)**: authored blend + test GREATER 0 -> Mask 0.5 (`mw_nif::Cutout`, importer v3 re-bake); plain blends whose texture alpha is holes, not translucency -> Mask at load; they write depth, take the composite fog, cast and receive shadows, write motion vectors; additive, faded, animated-alpha and particles stay blended; `--off leafmask` A/B; LM-1 awaiting owner. Left: coverage-preserving (Castaño) mips need a texture cook (DDS loads as authored). Open: light sprites, fog pulse (see .sessions/_archive fogfix handoff); FG-5 awaiting owner.
   - God rays barely visible (Godotwind's settings looked okay: compare, don't copy constants).
   - Ashstorm: the original has a particle effect (blowing ash), not just dense fog: add precipitation-style ash particles (M6 PR).
   - Shadows: most shadows only appear very close (a few cm to m): check CSM cascade/fade settings, shadow distances per
     preset, and the point-light budget fade. A torch-bearer casts no shadow from their own torch (expected with point light
     at the torch, but the body should still cast the sun/moon shadow and other lights' shadows: check).
     **F1+F2 built (shf12 2026-10-19):** Bevy contact shadows (sun + budgeted point lights; Medium 8 / High+ 16 steps, 0.5 m; needs the depth+MV prepass on Medium+), sun normal bias 1.8 -> 0.8, Hardware2x2 on Low; `--off contactshadows`, `--off shadowtune`; SH-7 awaiting owner, GPU A/B owed. Next: F3 (250 m CSM + cached far map), F6, F5.
     **F3 step a built (shf3 2026-10-19):** practical-split sun cascades to 250 m (`sf_sky_bevy::CascadeSplits`; Low 3 / 200 m), texels 1.4 / 3.8 / 10.5 / 35 cm (was 1.4 / 5.2 / 19 / 70); GPU ≈ 0; `--off shadowsplits` = old 504 m; SH-8 awaiting owner. **F3 step b (cached far static map) designed, not built** (weather.md "Sun cascades and the far static map"): until then L0 objects at 250-440 m cast no shadows. Next: F3b, F6, F5.
   - Grazing-light artefact: a blocky, half-transparent outline on objects/terrain at grazing light; shimmers with DLSS off.
     Suspects: shadow acne / normal-offset bias, dithered cascade end, terrain blend, or RT/denoiser. Owner asks for a live
     RT on/off toggle at any time (render path switch at runtime).
   - Water: too milky turquoise but looks good; colour must vary by area (tropical, arctic, muddy: region optics, W3
     in progress); shore outline too white and generic; wave height by weather for open water only (not lakes).
   - Physics: a thrown ball can push an item (e.g. the staff) through a wall (CCD / solver push-out against statics);
     some items have no collider (big baskets, small boxes: body.cook failures or open meshes?); carry like Oblivion: hold
     and move with the item, camera angular velocity drives the force on the held object (keep the dedicated throw key).
   - Ragdolls: buggy (known unfinished). Knockdown is messy: some body parts spin needlessly, and there is no fall
     animation: the NPC snaps to the lying pose. Needs a powered fall toward the ground before limp, and joint-limit/
     spin damping.
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
   - Lighting job [~] (lighting 2026-10-13: built, LT-1..3 awaiting owner): night exposure floor EV100 3.5
     (`LightCalibration::exposure_floor_ev100`, `--exposure-floor`; `sf_sky::ExposureRange` clamps the schedule and the
     auto-exposure curve) + night ambient 1 nit (`--night-ambient`); budgeted point-light shadows
     (`sf_sky_bevy::PointShadowBudget`, `BudgetedShadow`, distance + time fade by light splitting; `--point-shadows`,
     Low/Medium 0, High 1, Ultra 4: +0.8 ms for 1, +3.9 ms for 2 at 1440p, render-thread CPU per caster in range).
     Open: local-light shadow caster culling (size/distance per shadow view) would make 2-4 affordable.
     Was: Night exposure: **owner chose A (2026-10-13)**: clamp auto-exposure's range (a night floor, e.g.
     EV ~3–4 tuned by eye) so night stays dark and placed/carried lights stay physical (no exposure-scaled lights);
     fixes walls blowing out near lamps at night (torchfix open item). Also: torches and other carried lights cast no
     shadows (owner 2026-10-13); budgeted point-light shadows for the nearest few lights is the usual approach.
   - Owner notes 2026-10-13, actors: ~~some Balmora guards at night have a wonky walk cycle~~ (walkers5: walk lanes
     synced past their loop end froze after turns; fixed, HI-4 awaiting owner); ~~Dagoth Ur's hand effect is bluish
     particles with no light~~ (walkers5: matches the NIF data, no light in the model; PA-3 awaiting owner). Owner on uphill strides: people take shorter steps going up than down, but
     don't add edge cases to a locomotion system we may overhaul; note it for the locomotion refactor (slope-aware
     stride from speed + slope, not a special case).
   - Later (owner: not now, but important): object-terrain blending (R-b, M2 item) reads as "bottom of meshes darker",
     not terrain blending: TB-1..3 FAILED. Revisit the technique (terrain albedo/normal carried onto the mesh base).
   - S2 [~] (nightsky 2026-10-13: built, NS-1 awaiting owner) sun disk too small (owner): drawn disk 2.5° (`sf_sky::SunDiskLook`,
     `--sun-size`; the light keeps the physical 0.53°) + a glare halo drawn into the sky before the clouds (clouds hide it).
   - S3 [~] (nightsky 2026-10-13: built, NS-2..4 awaiting owner) night sky: Masser + Secunda (`sf_sky::Moon`: N moons, each with
     its own phase cycle, path and textures; Morrowind `tx_masser_*`/`tx_secunda_*` 8 phases, `[Moons]` sizes and tilts,
     `--day`), one moonlight summing the moons, procedural stars fading in at dusk; `sf_sky_bevy` night pass 0.26 ms at
     night / 0.08 ms at noon (1440p). Bug fixed: red night streaks with `--clouds clear` (the cirrus was lit by the set sun
     through Bevy's horizon-clamped transmittance LUT). Open: thin cirrus reads as dark wisps at night against Bevy's
     leaky twilight sky; Masser size and star occluder set by eye.
   - Then fog: now M6 tasks F1–F3 ("## M6: Water, weather, surfaces"), before item 5.
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
- **Crate "update all" pass (owner 2026-10-18):** one batch once `bevy_hanabi` has a 0.20 release on crates.io: adopt the
  crates that reached 0.20 (`bevy_flair` 0.9, input crate for U1, `bevy-inspector-egui` 0.38 as a dev add-on, hanabi) and
  bump the rest, from the tracker in ARCHITECTURE §4 (re-check it first). Until then, no piecemeal crate additions.
- **Shadows pass (owner 2026-10-18, "we need a big work on shadows generally"):** in the Census office, characters'
  and objects' shadows only show very close (interiors: 1 point-light shadow, `PointShadowBudget` faded out over
  15-25 m; `POINT_SHADOWS_DEFAULT` in `render.rs`). Start with an industry audit (shadow atlas for many local
  lights, cached static shadows, contact shadows, baked static-light shadows, RT via Solari) covering interiors and
  exteriors (SH-1..6 still awaiting owner). Not started: needs the owner's go.

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
- [~] LA1 near foliage vanishing (owner 2026-10-19, lodaudit): CPU chain (streaming, ranges, AABBs, proxy handoff) checked live via BRP,
  never hid a near placement. Structural fix: Bevy keeps one GPU slot per distinct `VisibilityRange` for the app lifetime (65535 max,
  then slot 0 aliasing dithers meshes out); our per-instance ranges were near-unique, now snapped by `sf_world_bevy::shared_range`
  (streaming.md "Visibility range slots"). Not yet confirmed as the owner's trigger; other suspects: blended leaves (leafmask), fogfix
  material swap (`--off transfog`). Verify: LA-1. Left: `mw_assets` interior ranges unsnapped.

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
- [~] P2 `sf_physics_bevy` + app wiring: `PhysicsPlugin` (60 Hz `FixedUpdate`, ≤ 3 steps/frame), write-back as
  `CellCoord + Transform` from `WorldPos`, activation hooks insert statics (replacing the walkers' `StaticWorld`
  build; `--off physics` keeps it) and terrain heightfield chunks (64 m, 0.5 m) in R0, debug gizmos.
  Verify: headless test (spawn, step, rebase, despawn); `cargo dev view` Balmora with physics debug draw shows statics +
  terrain; walkers unchanged (feet stats, 150-walker probe budget 0.2 ms); fly-through stutter counters not worse.
  **Built 2026-10-13 (physics-p2), uncommitted:** `superfargoth/crates/sf_physics_bevy` (`PhysicsPlugin`: 60 Hz
  `FixedUpdate`, ≤ 3 steps/frame, `follow` the streaming anchor by whole cells, interpolated write-back of awake
  `PhysicsBody` entities, despawn removes; `PhysicsHook`/`hook_physics` on the placement spawners; shapes and scaled
  copies built on the task pool; inserts under the 0.5 ms budget with a query refresh every 32 bodies and the refresh
  cost reserved; terrain chunks 64 m / 0.5 m over the active worldspace's R0 cells from `TerrainHeightsMap`, built
  on the task pool; `physics/...` counters; `PhysicsDebug` wireframes), `StaticShape::trimesh()` (one `Arc<TriMesh>`
  shared with physics), app `physics.rs` (`--physics-debug`, `G`, `--off physics`, 10 s peak log). Headless test
  passes; clippy pedantic clean. Balmora + Census Office debug screenshots line up (PH-6, scenes
  `ph-balmora-colliders`/`ph-census-colliders`, awaiting owner). Flight bench (60 m/s, shared machine, other agents
  building): ~1,400–2,150 bodies + 60–72 chunks resident; idle step peak 0.11–0.13 ms (gate 0.3); the step after a
  rebase 0.9–1.45 ms (gate 2); insert peaks 0.66–0.79 ms/frame before the refresh reserve was added (re-measure).
  **Open:** A/B stutters vs `--off physics` (the flight bench fails its gates either way on this machine: compare
  counts), re-measure insert peaks, and move walkers.rs `MwGround` to `PhysicsProbe` once walkers3 is done.
  **P2 A/B (physics-p3, 2026-10-13, release, shared machine):** 60 m/s flight, `--off physics` vs on: stutters 3.4/min
  (worst 45.9 ms, p99 33.5 ms) vs 2.5/min (worst 31.6 ms, p99 26.5 ms): physics on is not worse. Insert peaks with the
  refresh reserve **still 0.49–0.81 ms/frame** (1–6 frames per 10 s over 0.5 ms; 67–96 bodies): gate missed, P4 must
  time-slice the first refresh after a batch or lower `insert_batch`. Idle step 0.15–0.17 ms; step across a rebase
  0.87–1.16 ms. Fixed in P3: `PhysicsWorld::refresh_queries` before the first step silently dropped newly inserted
  dynamic bodies from rapier's islands (they never simulated, awake or woken); it now waits for a step while a dynamic
  insert is pending (test `awake_bodies_fall`). P1's spike "active pile" numbers predate this fix only if the spike
  refreshed before stepping (it doesn't call `refresh_queries`, so they stand).
- [~] P3 `body.cook` + `items.check` (framework bake steps) and `MwBodyRules` (`mw_bake`): authored `RootCollisionNode`
  else primitive / hull / V-HACD; per-page support and overlap check.
  Verify: `cargo test -p sf_physics --features bake`; `body_report` example on the install (models, shape kinds, hulls,
  overlaps > 1 cm, unsupported items per cell, bake time; full bake stays inside the 2 min gate).
  **Built 2026-10-13 (physics-p3), uncommitted:** `sf_physics::cook` (`BodyShape`/`ShapePart`, primitive/hull/V-HACD
  fit, `SFBD`), `sf_physics::steps` (feature `bake`: `body.cook`, `items.check` as a 0.5 s settle test per page,
  `SFIC`), `ShapeRef::Body`, debug draw of hulls/primitives/compounds, `PhysicsBodySource` in `sf_physics_bevy` (items
  draw orange, `Fixed` until P4), `mw_bake::body` (`MwBodyRules`, `MwItemsRules`, `item_mass`, `bake_bodies`,
  `check_items`, example `body_report`), both steps in `bevyrrowind bake` with report lines. Tests: `cargo test -p
  sf_physics --features bake` (fits on fixtures: box, round, flat, open cone, concave L, authored; settle: resting,
  hanging, off-edge, sunk, stacked, rule-held, no shape), `mw_bake` body tests; clippy pedantic clean.
  **Census (vanilla + expansions):** 1,055 item models, 14 failed (skirt/VFX models without static geometry); sources
  Authored 37, Primitive 26, Thin 12, Hull 809, Decomposition 171; single-part kinds box 31, cylinder 7, hull 846;
  2,041 hull parts (4–32 vertices, mean 29); decompositions 2–8 pieces (mean 7.5); mass over 3,396 item records
  0.2–60 kg (1,057 clamped up to 0.2 kg, 2 down to 60). **items.check:** 1,350 pages, 42,582 items: No 23,953,
  Unsupported 16,057, Rules (carryable lights) 2,572; 21,753 overlaps > 1 cm; 32,489 statics + 468 terrain patches.
  **Bake time:** cold (new steps) body.cook 21.4 s wall + items.check 7.2 s, whole `bevyrrowind bake` 31.7 s; warm
  2.75 s: inside the 2 min gate. **Open (P4 first):** the unsupported (38 %) and overlap (51 %) rates are implausibly
  high: suspect item-item interpenetration cascades in the all-at-once settle and render hulls bigger than the visual
  footprint (most items are open meshes → one hull; primitives rarely fit within 15 %); try per-item settles against
  statics + already-resting items, a 1–2 cm overlap allowance (shrunk hulls / contact skin), and check shapes in PH-7.
  Holding is the safe failure (held items stay put). PH-7 sanity screenshot not usable: the default interior camera is
  outside the 40 m debug radius; scene `ph-tradehouse-items` needs a `camera:` near the counter.
  **P3 open items moved to P4** (physics-p4, 2026-10-13): plausibility fix and census under P4 below.
- [~] P4 Dynamic items (`sf_physics_bevy` + `mw_physics`): `Dynamic` placements as sleeping bodies on R0 activation,
  `HOLD_UNTIL_GRABBED`, weight → mass, debris/prop layers, CCD, coverage freeze, fell-out recovery, persistence through
  `PlacementOverlay::move_to` + `physics` state blob on deactivation.
  Verify: Arrille's Tradehouse: 0 wakes in 10 s with nobody near; a thrown item topples the counter clutter, all rest
  within 3 s; PH-5 (leave, unload, return: items where they rested); dense-town R0 idle step ≤ 0.3 ms. Visual PH-1.
  **In progress 2026-10-13 (physics-p4), uncommitted; compiles, tests pass, not yet run in the app.** items.check fix:
  sequential settle (each item alone, lowest first, over statics + already-tested items held fixed where placed;
  1 cm/side overlap allowance by shrinking the test shape; early stop), census **before** (P3, all-at-once): Unsupported
  16,057 / 42,582 (38 %), overlaps > 1 cm 21,753 (51 %); **after**: Unsupported 15,194 (36 %), overlaps 10,945 (26 %),
  9.3 s wall (116.8 s summed). Overlaps halved; unsupported barely moved. Spot checks (report samples): the dropped items
  are thin books, bowls and platters on shelves (5–9 cm drops), i.e. they fall *through* the static trimesh: placed
  sunk deeper than half their thickness, two-sided contacts push them out the bottom. Fix landed, census pending: the
  physics copy of static trimeshes is now one-sided with internal-edge fixes (`TriMeshFlags::ORIENTED |
  FIX_INTERNAL_EDGES`, the controller keeps its two-sided mesh), items.check version 3 so the next `bevyrrowind bake`
  re-runs it. P4 proper: items (`Dynamic` + carryable lights via `hook_items`) are sleeping dynamic bodies (CCD,
  `ITEM_DAMPING`), `Fixed` when `items.check` holds them (read by placement id on a startup thread) or always for
  carryable lights; mass `mw_physics::item_mass` (new crate `mw_physics`: `ItemRecords`, `carryable_lights`, rules
  shared with `mw_bake`); `DEBRIS` < 15 kg and < 0.6 m else `PROP`; visuals under an `ItemPivot` that follows the body;
  deactivation writes moved items to the `PlacementOverlay` (`move_to` + `physics` blob v1), activation reads it;
  inserts two-phase (statics + refresh every 16, time-sliced by a per-body refresh estimate; then ≤ 32 items without a
  refresh); rolling resistance (20 rad/s², contact-gated) since rapier has none; dev key `V` throws a 3 kg ball.
  Tests: `sf_physics` `box_dropped_on_table_items_wakes_and_settles`, `refresh_before_first_step_keeps_new_bodies`,
  settle fixtures; `sf_physics_bevy` `moved_items_persist_through_deactivation` (PH-5 headless). Visual PH-1/PH-5
  added (awaiting owner).
  **physics-p4b (2026-10-13), uncommitted; runs in the app.** Census v3 (one-sided statics) did not help: Unsupported
  16,245 (38 %). Cause (body_report `SF_ITEMS_CELL` + new `SF_ITEMS_NEAR`): bookshelves and cabinets have an authored
  collision proxy that is **one box around the whole piece**, so their clutter starts inside it. Fix (UE simple vs complex
  collision): `items.check` tests whether a static's collision encloses an item center (`MeshShape::encloses`: the
  nearest triangle below is a back face) and then uses the new `support.cook` (the `mesh.cook` LOD0 as a
  `CollisionMesh`) for that placement, listed in `ItemChecks::supports` (format 2); the runtime gives those placements
  the same mesh (`ItemContent::is_support`, `PhysicsBodySource::supports`), characters keep the proxy. Unsupported now
  means the item's center **and** lowest bounds corner both dropped > 5 cm (a book tipping over in its row, or a bowl
  rocking, is not a fall). **Census v6: Unsupported 3,478 / 42,582 (8.2 %)**, No 36,532, Rules 2,572; overlaps > 1 cm
  12,491 (29 %); 2,288 statics (217 models) with support meshes; items.check 82 s summed, whole warm `bevyrrowind bake`
  8.7 s. Top unsupported: weapons/shields/fishing poles on wall racks (Raven Rock smith, Maar Gan outpost: correct),
  and books on `furn_de_bookshelf_02` in the Vivec libraries (still dropping: open). In the app (Arrille's, release):
  **0 awake bodies in the 10 s after load** (peak 6 while loading), idle step 0.028 ms; load insert peak 0.80 ms/frame
  (78 bodies, over the 0.5 ms gate). Fixed on the way: inserts **stalled forever** when one refresh estimate exceeded the
  budget (2,015 bodies pending: now at least one static per frame); items woke before the counter/floor under them
  existed and **fell through the world** (a shortsword at 82 m/s): items now wait until their worldspace has no static
  left to insert; `--all-features` tests failed without a gizmo plugin (debug draw now `run_if` gizmos). PH-7 camera
  added + sanity screenshot (`shots/physics/ph7_tradehouse_a.png`, shapes line up). `physics:` log: items, peak awake,
  `awake item ...` lines. **Open (P5 handoff, in order):** flight bench (60 m/s) for insert peaks and items' first-step
  cost (not run: tool cap); load insert peak 0.80 ms > 0.5 (time-slice the items phase too); `PhysicsParams::kill_height`
  is `None` in the app: set a per-worldspace kill height (lowest static/terrain - margin) so fell-out recovery runs;
  the Vivec library bookshelf case (`SF_ITEMS_CELL="Vivec, Library of Vivec" SF_ITEMS_NEAR="5.9,0.31,-3.65"`); one bowl
  on the Tradehouse support shelf rocked until it slept (watch `awake item` lines); coverage freeze; held items released
  by an impulse; `support.cook` surfaces are `DEFAULT` (render materials not mapped).
- [~] P5 Controller on physics (`sf_controller(_bevy)`): kinematic capsule bodies, sweeps against `STATIC | PROP`,
  capped push impulses on props and debris, `ImpactOnCharacter`, sliding on unwalkable slopes (locomotion.md open
  question 3); the mover stays in `Update`.
  Verify: `cargo test -p sf_controller` (push caps, debris never blocks, slope slide); walkers through a cluttered
  floor scatter debris without stalling; a crate blocks then yields to sustained pushing; 150-walker budget holds.
  **P4 leftovers closed first (physics-p5, 2026-10-13, uncommitted).** Flight bench before the changes (60 m/s,
  release, no other build at start): insert peaks 0.53–0.65 ms/frame (1.31 in a rebase window), idle step peaks
  0.41–0.58 ms in windows without a rebase (over the 0.3 gate: items' first steps), 0.86–1.27 ms with rebases; frames
  19.1 ms median, 36.3 worst, 4.2 stutters/min (gates fail as before). Items insert phase now time-sliced (decaying
  max per-insert cost for statics and items, a 15 µs prior before any measurement, one insert per frame guaranteed):
  Tradehouse load peak 0.80 → 0.57 ms/frame (95 bodies) before the prior; re-measure pending (P6 session: the app
  stopped compiling at the end of this session on an in-progress `sf_atmos` edit by m6-f2). Automatic kill height
  (`kill_margin` 10 m under the lowest fixed body; test `automatic_kill_height_below_lowest_static`), coverage freeze
  at 4 Hz (`stream::coverage`, overlay write + body removal), held items as `BodyKind::Held` released by
  `PhysicsWorld::release` or any impulse. Not done: the Vivec library bookshelf books (optional).
  **P5 built (physics-p5, uncommitted).** `sf_physics::character` (kinematic `CHARACTER` capsule 3 cm inside the
  mover's, `move_character`, capped `push` with `min(m_body · Δv_n, 400 N · dt)` from the intended velocity; spec
  formula corrected in physics.md), `sf_controller` slope sliding (`Locomotion::slide`), `sf_physics_bevy` feature
  `controller` (capsules for non-`Dormant` movers, pushes, `ImpactOnCharacter`, `CharacterStats`, `PhysicsEvents`,
  `impl GroundProbe for Physics` over `STATIC | TERRAIN | PROP`); the walkers' mover runs on `Physics` unless
  `--off physics` (their `StaticWorld` build is skipped then). Tests: `character_tests` (crate blocks then yields,
  every push ≤ cap; debris never blocks; a 25 m/s ball hits a walker; no tunnelling through a 4 cm plank at 7 m/s),
  `unwalkable_slopes_slide`. **wk-balmora (release, 75 s, quiet machine):** physics on vs `--off physics`: 61 vs 67
  movers, mover stage median **584 µs vs 319 µs** (9.6 vs 4.8 µs per mover: the physics probe costs about 2× the
  `StaticWorld`, after dropping the surface lookup on wall sweeps), strides 10 skated / 94 stances vs 11 / 211, 0 vs 0
  legs without a plant, IK unplanted 20 vs ~21, walks given up 44 vs 55. **Open (P6 session first):** the probe
  cost (profile `PhysicsProbe`; likely rapier's generic query path and heightfield casts where the `StaticWorld`
  path used the analytic `TerrainProbe`: try `Nearest(TerrainProbe, physics STATIC|PROP)`), the 150-walker budget,
  walkers through cluttered floors (no pushes seen in wk-balmora: the walkers' paths are clear), visual PH-8.
  **P5 follow-ups (physics-p6, 2026-10-13, uncommitted).** Probe cost: `sf_physics` `probe_bench` (release; 150
  movers × capsule sweep + sphere down + 2 rays over the P1 R0) shows heightfield sphere/capsule casts at ~13 µs each:
  `StaticWorld` + `TerrainProbe` 2.14 ms, physics `STATIC|TERRAIN|PROP` 6.36 ms, `Nearest(TerrainProbe, physics
  STATIC|PROP)` 2.58 ms. `impl GroundProbe for Physics` now does the latter (`Physics::terrain` = the active
  worldspace's `TerrainHeights`). wk-balmora (80 s, shared machine: builds ran during part of each run) mover stage
  median physics vs `--off physics` **424 vs 272 µs** (69 vs 68 movers; before 584 vs 319): 1.56× instead of 1.83×;
  the rest is rapier's all-bodies query BVH and the one-sided internal-edge statics (a statics-only BVH view is the
  next step); both paths are over the 150-walker 0.2 ms budget at this mover count. Tradehouse load (quiet): insert
  peak **0.60 ms/frame** (35 bodies; 8 frames over budget; gate 0.5: still missed, the per-insert estimate under-reads
  some inserts), step peak 0.50 ms in the load window, then 0 awake, idle step 0.041 ms. Flight 60 m/s (quiet, no
  builds): insert peaks **0.45–0.55 ms/frame** (was 0.53–0.65), idle step peaks without a rebase **0.45–0.56 ms**
  (gate 0.3: still missed, items' first steps), with rebases 0.99–2.0 ms; frames median 18.65 ms, p99 26.5, worst
  32.9. Not done: walkers through cluttered floors (PH-8 scene work).
  **Fall-through fix (fallfix, 2026-10-18).** NPCs in the Census office fell from load at 50 m/s (-11.8 km): R0
  activation woke movers while their floor static still waited for insert budget. Collision-ready gate: movers start
  `Dormant` and wake only when `PhysicsStreaming::collision_loading` is false and the terrain heights around them are
  resident; safety net `character::fell_out` (in the air > 2 m under resident terrain, else under the kill height:
  back to the last walkable ground, `CharacterFellOut` logged); pure swimmers float (`Fly`, scaffold until M6 Swim).
  Tests `streamed_ground` (sf_physics_bevy); office NPCs at y 2.74 for 3+ min, Seyda Neen 0 fell-outs over 2 min.
- [~] P6 Carry, drag and throw: pick ray, kinematic hand + 6-DOF motor joint with force cap, break distance,
  drag mode (ball joint at the grab point, holder slowed) for objects over the carry mass and for limp ragdolls by the
  grabbed limb (dead bodies), `HolderLimits` (carry force, carry mass, drag mass, throw impulse) from the adapter,
  rotate, drop, throw; holder-space interpolation; dev `--fps-cap`; Morrowind holder limits from Strength
  (physics-adapter.md table; owner request 2026-10-12).
  Verify: view-space jitter of a held item < 1 mm at 30/60/144 fps while walking and turning (automated); throw speed
  within 1 % of the cap; carry/drag/too-heavy switch at the Strength thresholds; a 75 kg ragdoll dragged by a wrist with
  no joint stretch > 1 cm (needs P7; until then drag is verified on heavy items). Visual PH-4 (carry + corpse drag).
  **Built 2026-10-13 (physics-p6), uncommitted; corpse drag waits for P7.** `sf_physics::carry` (`Carrier`,
  `HolderLimits`, `CarryParams`; kinematic hand + rapier generic joint with acceleration-based motors, all six axes
  for carry, linear only at the grab point for drag; pick ray; break 1 m / 0.3 s; throw capped at 15 m/s; per-pair
  contact exclusion with the holder's capsule; hand target = the view at the step's time; holder-space render pose),
  `sf_physics_bevy::carry` (`Holder`, `CarryInput`/`CarryFeedback`, `HeldPoses` in the write-back and item pivots),
  `mw_physics::holder_limits(strength)` (adapter table) + `DEFAULT_STRENGTH` 50, app debug driver on the free camera
  (hold LMB grab/carry/drag, release drop, `V` throw, `[`/`]` turn; `BEVYRROWIND_STRENGTH`, dev frame limiter
  `BEVYRROWIND_FPS_CAP` instead of a `--fps-cap` flag). Change from the spec: the drag force cap is
  `max(carry_force, 0.6 · g · drag_mass)` (240 N could not slide a 40 kg crate). Tests (`sf_physics` `carry_tests`):
  view-space spread 0.008 / 0.010 / 0.044 mm at 30 / 60 / 144 fps (gate 1 mm); throw at the cap 15.00 m/s (±1 %);
  a 6 m/s throw lands 3.320 m vs 3.333 ballistic; carry/drag/too-heavy at the S 30 and S 100 limits; a 40 kg crate
  drags 2.3 m in 3 s (tips onto its edge, lift 0.12 m); an 11 kg item on a 40 N grip breaks away; the holder's capsule
  never touches the held item; `mw_physics` `strength_maps_to_the_table`. PH-4 awaiting owner. **P7 handoff:** add
  `RAGDOLL` (limp only) to `CarryParams::pick_layers`; the mode mass for a limb is the whole ragdoll's mass (pass it
  to `Carrier::grab_body` via a mass override instead of `world.mass(body)`); drag anchors at the hit point already;
  the ragdoll stays `Limp` and awake while dragged; check joint stretch < 1 cm dragging a 75 kg ragdoll by a wrist;
  the drag force cap may need the joint iterations raised. The host's mover slowdown reads `Holder::speed_scale`
  (the dev camera scales its speeds).
- [~] P7 `ragdoll.cook` + limp ragdolls (`sf_ragdoll`, `mw_physics::RagdollRules`): bodies/joints/limits/masses from
  `RigProfile` + skin extents, override files, creature policy table, death handoff with velocity carry-over, sleep +
  persisted pose, placed corpses settle once.
  Verify: cook report for the 3 humanoid skeletons + all CREA skeletons (bodies, joints, failures); no joint stretch
  > 1 cm in a fall down stairs; dead guard on the Balmora stairs (PH-2 first half).
  **Built 2026-10-13 (physics-p7), uncommitted; PH-2 awaiting owner.** New crate `superfargoth/crates/sf_ragdoll`
  (`cook`: capsules per spine segment (≤ 3, girdle-wide), neck+head, limb joint, tail/long extra chain (≤ 4); ball
  limits by role, knee/elbow/hock hinges bending the rest pose's way; masses balanced to ≤ 4:1 between neighbors and
  scaled to the adapter's total; rest overlaps → `no_collide`; `RagdollInstance` spawn at an animated pose with
  velocities, `local_pose` readback; `ragdoll.cook` step + `RagdollRules`, `SFRD` product). `sf_physics`: grab mode
  mass (`set_mode_mass`: a limb answers for the whole ragdoll), ragdoll bodies skip the item mass clamp, grabbed
  bodies keep/restore their iterations and stay awake, `RAGDOLL` in the default pick layers. `sf_anim_bevy`
  `PhysicsPose` (physics pose blended after the graph; P8 moves it to `PoseInput`). `sf_physics_bevy` feature
  `ragdoll` (`Ragdoll` request on the `Animator` entity, `RagdollSource`, death-clip delay, velocity carry-over,
  0.1 s blend, cleanup on despawn). Adapter: `mw_physics::ragdoll_params` (humanoids 75 kg, ghosts/wraiths/wisps
  death clip only), `mw_bake::ragdoll` in `bevyrrowind bake` (census line), app `I` kills the character under the
  crosshair (graph `dead`, mover `Dormant` + `Dead`, ragdoll after 0.4 s); corpses drag by the grabbed limb.
  **Stretch (release, 75 kg fixture biped; impulse joints, +8 iterations, ratio cap 4):** 0.5 m drop 0.49 cm,
  stairs 0.32 cm (settles in 4.5 s), dragged by the wrist 0.07 cm (follows 1.1 m in 3 s): gate 1 cm met; the P1
  2.0–2.6 cm is gone. Table (`stretch_table`): +0/4/8/12/16 iterations drop 0.89/0.61/0.49/3.09/3.03 cm, drag
  0.16/0.16/0.07/0.04/0.03; without the mass cap (lightest 0.1 kg) every drag breaks away. **Census** (vanilla +
  expansions, `BEVYRROWIND_RAGDOLL_REPORT=1 bevyrrowind bake`): 93 skeletons (10 humanoid incl. biped creatures, 83
  creature: all 89 creature body models' rigs), **0 failed**, 2 death-clip only (ancestor ghost, dwarven specter);
  3–37 bodies; humanoids 18–22 bodies at 75 kg; guar 24 bodies, cliff racer 21, kwama forager 3. **Creature masses
  are implausible without skin extents** (volume of length-based capsules): netch bull 12.9 t, cliff racer 1.27 t,
  guar 1.04 t, durzog 0.97 t vs horker 4.6 kg, babelfish 1 kg, rust rat 103 kg. Tests: `sf_ragdoll` 8 (+1 ignored
  table), sf_physics/sf_physics_bevy/sf_anim_bevy/mw_physics pass; clippy pedantic clean (superfargoth crates before a
  half-created `sf_script_bevy` from another session broke the workspace glob; bevyrrowind mw_physics/mw_bake/app).
  **Open (P8 handoff, in order):** creature radii and masses from skin extents (`mesh.rig` of the creature body:
  80th-percentile vertex distance per joint) or per-creature totals; the powered crumple (P8 drives replace the
  0.4 s clip cut); root follows the pelvis (culling, cell moves); sleep → quantized `ragdoll` state blob and placed
  corpses; PH-2 sanity screenshot (needs a scene with a guard on the stairs; the kill needs a key press);
  `PhysicsPose` → graph `PoseInput`; filter powered (P8) ragdolls out of `pick_layers`.
- [~] P8 Powered ragdolls + blend (`sf_ragdoll`, `sf_anim_graph` `PoseInput`, `mw_graphs` states): implicit motors from
  the animated pose, per-region `StrengthProfile`, world pelvis drive, hit impulse + recovery, balance → stumble /
  knockdown, get-up (face up/down, `Knockout@tail`, `ROLL_TO_BACK` scaffold) with inertialization, budget of 8.
  Verify: tracking ≤ 5 cm RMS / ≤ 5° at full strength while walking; get-up angular velocity continuous at shrinking
  dt; ≤ 0.05 ms per powered character; PH-2 second half (knockdown on stairs, gets up with feet placed).
  **Core built 2026-10-13 (physics-p8), uncommitted; not yet wired into the app.** `sf_physics`: `Layer::POWERED`
  (contacts as `RAGDOLL`, outside the carry pick layers: P7 handoff 7), `JointKind::Free`, `set_joint_target` (a
  rotation target as rapier's per-axis `2·asin(q.v)` errors + velocity feed-forward: converges on swing and twist
  together, fixing the P1 residual), `set_joint_linear_drive`. `sf_ragdoll::powered`: `Drive`/`StrengthProfile`/
  `HitImpulse`/`DriveParams`, `RagdollInstance::drive` (stable PD through acceleration-based motors with
  **composite-inertia gain scaling**: stiffness × subtree inertia about the joint ÷ the child's own, without which the
  spine folded 121° and effectors hung 2.7 m off; a kinematic anchor + six-axis motor joint as the world pelvis
  drive; bodies `POWERED` while driven), `hit` + recovering `Dip`s, `nearest_body`, `pelvis_speed`,
  `implied_frame` (root follows pelvis; face up/down), `center_of_mass`, `balance` (convex support hull: Stable /
  Stumble / Fall), `get_up_root` (yaw + translation putting the get-up clip's first-frame pelvis on the ragdoll's;
  `ROLL_TO_BACK` scaffold for face-down bodies). `sf_anim_graph`: the graph's built-in physics stage
  (`GraphInstance::set_physics(pose, per-joint weights)`, blended after inertialization, before history and the
  procedural ops) and `inertialize_from` (the hand-back); `sf_anim_bevy` `PhysicsPose` gains per-joint `weights` and
  goes through the graph (write_bones only writes it on frames the graph doesn't pose). `mw_physics`:
  `CREATURE_MASSES`/`creature_mass` (52 name fragments: guar 400 kg, netch bull 600, cliff racer 30, kwama queen
  1,500, rat 6, ...; P7 handoff 1 by per-creature totals; census re-run pending). **Measured** (release, quiet
  machine, 75 kg fixture biped walking 1.2 m/s, no floor, ω 160 rad/s, pelvis ω 12): worst joint **3.2° RMS**
  (thigh; gate 5° met), effectors **11.5 cm RMS** (gate 5 cm **missed**: the world pelvis placement; joints
  are right), knockdown (120 N·s shove) lies face up at rest in 1.1–1.6 s; hand-back pose step 5.8/1.3 cm per frame
  at 60/240 Hz vs a 59 cm cut, velocity jump 0.56/0.09 m/s (continuous at shrinking dt: met). **Step cost: 1
  powered 0.14 ms, 8 powered 1.02 ms = 0.128 ms per character (gate 0.05: missed, 2.6×**; 8 extra iterations per
  body). Lower ω (40) tracked 19° (gravity sag g/(d·ω²) on hands/feet). Tests: `sf_ragdoll` 13 (+2 ignored:
  `stretch_table`, `powered_cost`), `sf_anim_graph` `physics_handback_is_continuous_at_shrinking_dt`, `mw_physics`
  `creature_masses_are_plausible`; clippy pedantic clean on all touched crates.
  **P8b (2026-10-13, physics-p8b, uncommitted):** pelvis drive was underdamped (ζ = 1/√load ≈ 0.37 from scaling only
  the stiffness of an acceleration motor; rang ±8 cm at 2 Hz): now critically damped for the composite load +
  gravity compensation (anchor offset −g/ω², weight torque about the pelvis). Effectors 11.5 → 6.8 cm in air (pelvis
  9.0 → 2.2 cm, joints 5.5°), 23.8 → 11.0 cm on a floor (synthetic walk slides/sinks its feet): gate 5 cm still open.
  Cost: `DriveParams::powered_iterations` (+8: 0.129, +4: 0.090, +2: 0.071, +0: 0.051 ms/char, 8 powered on a floor;
  tracking degrades 11 → 26 cm); rapier `parallel` (new sf_physics feature) slower (0.153, shared machine). Default +8.
  `sf_physics_bevy`: `RagdollMode::Death { crumple } | Knockdown { point, impulse, get_up, get_up_flag }`, FixedUpdate
  `drive`, crumple 0.45 s, brace 0.25 over 0.8 s, budget 8, per-joint weights from `def.owner`, read-back relative to
  the current frame, corpse frame follows `implied_frame` (heading), get-up at pelvis < 0.2 m/s for 0.5 s
  (`get_up_root`, `LYING_REST_GET_UP` scaffold, `inertialize_from`, clears the flag, `RagdollGotUp`).
  `ImpactOnCharacter.direction` (contact normal; `PhysicsEvent::ContactForce.normal`). App: `I` = crumple death,
  impacts ≥ 15 N·s knock walkers down (`KnockedDown`, root takes the get-up move). Census re-run: 93 skeletons (10
  humanoid, 83 creature), 0 failed, 2 death-clip only, 3–37 bodies, 1–1500 kg. NOT run in the app yet (tool cap).
  **Open (P8b → P9 handoff):** app run `I`/`V` + PH-2/PH-9 screenshots; sleep-state blob + placed corpses;
  get-up clip first-frame root body from the Knockout clip tail (removes `LYING_REST_GET_UP`); mover yaw on get-up;
  effector gate (per-effector world drives or kinematic legs on floors); cost gate.
  **Open (P8b, in order):** effector gate (pelvis drive: try the anchor at the target's COM, or drive the pelvis
  with the full-body linear load; measure on a floor); cost (fewer extra iterations while powered, sleep limp
  bodies, rapier `parallel`); `sf_physics_bevy` modes (`Ragdoll { mode: Death { crumple } | Knockdown }`: call
  `drive` each fixed step with the Animator's model pose, fade `StrengthProfile::scaled` over 0.3–0.6 s for the
  crumple, budget 8 by `POWERED` count, per-region `PhysicsPose.weights` from `def.owner`, read-back relative to the
  entity's current frame); knockdown from `ImpactOnCharacter` (add a direction), get-up when `pelvis_speed` < 0.2 m/s
  for 0.5 s → `get_up_root` with `Knockout@tail` sampled at 0 → move the walker root, graph `knocked_out` false,
  `inertialize_from` the last two ragdoll poses (relative to the new root); root follows `implied_frame` while
  dead; sleep → `ragdoll` state blob + placed corpses; app `I`/`V` run, PH-2 screenshot scene, PH-9.
- [~] P9 `HitReact` + `SpringBones` ops (animation.md layer 4; `SpringBones` is the base the cloth op extends).
  Verify: `cargo test -p sf_anim_graph` (spring decay, no drift at any frame rate); a guar's tail and an NPC hit below
  A0 react kinematically.
  **As built (physics-p9 2026-10-13, committed superfargoth 06fd609 / bevyrrowind 0a1405d; app-run by p9fin
  2026-10-18):** `sf_anim_graph::secondary`: `HitReact` (hit joint + up to `depth − 1` reactive ancestors kicked about
  `lever × direction`, Holden closed-form critical rotation springs, peak `gain · strength` after `halflife / (2 ln 2)`,
  exact return to the animation; runs after foot placement), `GraphInstance::hit(Hit)`; `SpringBones` (Verlet chains in
  character space, fixed 1/120 s substeps, frequency Hz + damping ratio, gravity, inertia from `UpdateInputs::moved`,
  `max_angle` cone). Reuse check in animation.md (Bevy has none; `bevy_skeleton_modifiers` runs outside our op stack).
  `mw_graphs`: both ops on the humanoid and creature templates (not first person). `sf_physics_bevy::clip_root_body`:
  the get-up root from `Knockout@tail`'s first frame (`LYING_REST_GET_UP` only for skeletons without it). App
  (`physics.rs`): impacts 1–15 N·s → `hit` (strength impulse / 15), ≥ 15 N·s knockdown, mover yaw after the get-up,
  `BEVYRROWIND_AUTO` dev hook (kill / throw / screenshot / exit; frames a new target 0.5 s before it throws, releases
  slow balls close so `:2` stays a hit react, leads walking targets); `mark_settled` now always runs (the hook waits
  for `Settled`, which only `--use-door` used to insert). **App run** (release, `scene wk-balmora`, all throws hit):
  `ragdoll: killed`, 0 failed; 12 m/s ball `knocked down (36 N·s)`, `got up` 2.2–2.5 s later, no scaffold line;
  2 m/s ball `hit react (5.9 N·s, strength 0.40)`, also right after a get-up. 3 tests (frame-rate independence at
  30/60/144 fps, no drift, sag); clippy pedantic clean. Shots `shots/p9/{kill,knock,getup,hit}.png`.
  **Open:** owner PH-2/PH-9/PH-10 (the flinch and tails need eyes; tails never run in the app); joint stretch peaks
  1.4–2.8 cm per ragdoll (5.6 cm once on a knockdown right after a get-up) vs the 1 cm PH-2 target; one intermittent
  panic in run 5 (a command on a despawned entity, 77983v0, at the first auto refocus; not reproduced in 3 reruns);
  P8b items unchanged: sleep-state ragdoll blob + placed corpses (not started), effector gate (6.8/11.0 cm vs 5),
  cost gate (0.129 ms/char vs 0.05); no performance gate measured for P9.
- [~] P10 Cloth (`cloth.cook` in `sf_character`, `ClothLattice` op in `sf_anim`, `mw_physics::ClothRules`): robe and
  skirt lattices, outfit joints in the merged palette, XPBD in character space with inertia scaling, tethers, max
  distance, leg/pelvis/tail capsules; census of robe/skirt parts.
  **PH-3 FAILED (owner 2026-10-18), parked for a later session:** a robe hides the legs (Morrowind's part rules), so a
  foot stepping past the robe envelope shows a missing leg; the envelope doesn't follow the legs at all. Before more
  tuning: research how games do long robes over walking legs (keep legs drawn under cloth? per-leg skirt panels /
  leg-driven bones like UE skirt setups? collision vs skinning blend), then redesign. Capes, flags, tents (mesh cloth)
  were never tried; they're the "mesh cloth" item after M4.
  Verify: unit tests (hem stretch ≤ 5 %, no particle inside a capsule at run speed, teleport stable); ≤ 0.03 ms per robe;
  visual PH-3 (`character --walk` in a robe, Balmora Temple priests).
  Split: p10a solver core → p10b `cloth.cook` + census + `ClothRules` → p10c `ClothLattice` op + app + PH-3.
  **Built (p10a, 2026-10-18): solver core `sf_anim::cloth`** (Bevy-free; physics.md "Cloth" as built):
  `ClothLattice::grid` (pinned top row, structural/shear/bend edges, geodesic tethers, max distance), `ClothState::step`
  (XPBD, 1/240 s substeps, one pass each; character space with fictitious-force inertia 0.5 linear / 0.3 angular;
  swept leg/pelvis capsules last; teleport keep-shape mode), `rotations()` for the cloth joints. Unit tests: hem
  stretch 1.3–1.9 % at 30/60/144 fps through run, 180° turn and dead stop; no particle inside a capsule; 50 m + 143°
  teleport stable; **0.014 ms per robe** at 60 fps (release). Clippy pedantic clean.
  **Built (p10b, 2026-10-18): `cloth.cook` + colliders + census + rules** (physics.md / physics-adapter.md "As built
  (p10b)"): `sf_character::cook_cloth` (rows × columns on the outermost rest surface by farthest ray hit, particles
  ride the girdle joint with per-particle offsets, per-particle authored skin `targets`, `SkinRebind` with bilinear
  lattice weights below row 1 and a harmonic band at the waist), `ClothRules`/`ClothSpec`;
  `RagdollDef::cloth_colliders` (core, legs, tails below the line); `mw_physics::MwClothRules` (robe = Chest part,
  skirt = Skirt part, 10 × 5 / 8 × 3, `Bip01 Pelvis`). Census: 64 robes + 20 skirts, 50 cloth parts all skinned,
  48 cook (9 ms median); **no hem holds a 50° run stride (+13..+133 % stretch)**. Tests + clippy pedantic clean.
  **Built (p10c/p10d, 2026-10-18): V1 end to end, ran in the app.** `sf_anim_graph::secondary::ClothRun` op on
  `GraphInstance` (after foot placement; target per particle = lerp(girdle ride, authored leg skin, 0.7 × row
  fraction); capsules posed per frame; kinematic above cloth LOD 1 or with `ClothSettings::simulate` off),
  `sf_character::cloth_decoded` (lattice joints in the outfit palette), `sf_character_bevy::cloth` (scaffold
  `ClothCookAtLoad`: cooks on the build task, cached per skeleton + part; removal path = `cloth.cook` bake step),
  app: `MwCharacters` cloth hooks, **H** simulated ↔ kinematic, `cloth:` cost log; BRP in interactive `character`
  runs. Tests (`secondary::cloth_op_tests`, sf_anim + sf_character cloth) and clippy pedantic clean. Run: two robes
  walk and run, nothing explodes or vanishes, H A/B works; **20-26 µs per simulated robe** (dev build, gate 30 µs);
  Balmora Temple: 9 cloth parts, 6 simulated, 22 µs each. At a run the simulated hem hangs straighter than the
  stride (feet show front and back; owner to judge). PH-3 awaiting owner (`scene ph3-balmora-temple`, `character`
  lineup, `scene level-robe-walk`).
  **Redesign (owner 2026-10-19, research `../superfargoth/docs/research/cloth/long-robes.md`): full robe simulation
  is overkill.** **Built (p10s, S1): robes and skirts drawn as authored by default** (physics-adapter.md "Cloth"):
  `mw_physics::ClothMode` (`Authored` default, `Robes` opt-in), `MwClothRules::with_mode`, `cloth()` → `None` when
  authored, so `sf_character_bevy` takes the "drawn as authored" path (no cook, rebind, lattice joints or op: 0 µs);
  **opt-in `--cloth robes`** on `view`/`scene`/`level` and `character` keeps the V1 simulation (H A/B) for A/B and
  mods; census opts in. Unit test (robes/skirts `None` by default, `Some` with the opt-in), clippy pedantic clean;
  agent run of the `character` robe lineup at walk and run: no `cloth:` line, legs covered. PH-3 rewritten, awaiting
  owner (`scene level-robe-walk`, `character` lineup, `scene ph3-balmora-temple`). No code deleted.
  **Parked** (only matter for simulated robes): leg-follow tuning (`CLOTH_LEG_FOLLOW` 0.7), hem widening for a run
  stride (+13..+133 %), the `cloth.cook` bake step (scaffold `ClothCookAtLoad` stays until a default path cooks cloth:
  capes, mesh cloth); smooth bodies on cloth parts dropped for robes.
  **Next slice S2 (owner idea 2026-10-19, low priority, not started): hem-only drape on the authored skin.** Keep
  every vertex on its authored Morrowind skin and simulate only the bottom rows of hem vertices. Each simulated
  vertex is tethered to its own skinned position by a max distance painted from 0 at the waist (and on every
  unsimulated row) ramping to a few cm at the hem: the standard "painted max distance" of Chaos Cloth and NvCloth.
  Coverage guarantee: no vertex can stray past its radius from the skinned position, and the skinned position already
  covers the legs, so the legs stay covered at any stride. Target ≤ 5 µs per robe (`cloth:` log). Verify: S1's shots
  unchanged at a dead stop; the hem settles a little behind on a stop or turn.
  **Left [~]: PH-3 awaiting owner (S1).** Next (log, not polish): S2 on owner request; the op into the compiled op
  list if graphs need to order it.
- [ ] P11 Traversal actions and interaction points (already slotted for M4 by animation.md layer 3): vault/mantle/ledge
  from `ContextProbes` with warping, `sf_interaction` points (benches, doors, levers). Not physics; may run in parallel
  with P3–P10. Verify: per animation.md "Layer 3".
  Scope widened (owner 2026-10-18): one general traversal and interaction layer (mantle, vault, parkour, climbing,
  ladders, sitting, doors, levers). Learn from AC1-rs, faith-runner, ELDEN-RING-Combat-Rewrite, sekiro-rs and ozz,
  don't copy them: animation.md "Prior art: open Rust/Bevy game reimplementations". First step: pick a clip source
  (Morrowind has no climb/vault/sit clips), then write the traversal spec. Not started: needs the owner's go.
  **Spec: `../superfargoth/docs/specs/action.md`** (R11b; owner answered its open questions 2026-10-19) with `camera.md`; P11 is its
  phases **A1-B5** ("Build plan": A1 backbone, A2 fixed tick in Bevy incl. movers on the fixed tick (physics.md
  "Controller" revised in co-op N0), B1 progress + chain warp, B2 mantle and vault, B3 ledge catch and hang, B4 edge
  index + ladders, B5 free-run); C1 interactions and D1 camera follow. Net-ready per `specs/net.md`.
- [x] TS1 Synthetic test levels (owner go 2026-10-18; specs: superfargoth `test-levels.md` format + research,
  `docs/specs/test-levels.md` game side): greybox levels authored as `levels/*.ron`, opened through `scene`/`view` with
  the usual flags, BRP and F1 keys; uses: PH-3 cloth on slopes, PH checks, traversal courses (research 01).
  **Built (ts1a, 2026-10-18):** `sf_level` (Bevy-free: RON format, validation, flat-shaded meshes with meter UVs;
  6 tests); app `level.rs`: `view level:<name>` / scenes `level: "<name>"` run the level as its own worldspace in the
  streamed viewer (empty `MemoryIndex` over the bake's), one mesh per primitive over a 1 m prototype grid, the same
  triangles as `Fixed` mesh colliders on `STATIC`; levels `flat-slopes` (10-40° ramps, 3 stair flights) and `ledges`
  (0.3-2.5 m boxes, thin vs deep, 1-4 m gaps, 0.3 m beam, wall); scenes `level-flat-slopes`, `level-ledges` (TS-1
  awaiting owner). Load: ready 2.1-2.8 s after process start (content 0.34 s, spawn 1-2 ms), dev build.
  Format decision: RON, not `DynamicScene`/BSN (no `.bsn` files in 0.20) or Skein (Bevy 0.19 only, revisit as a
  glTF primitive). **Built (ts1b, 2026-10-18):** level `actors` (record id at a spawn, outfit, scripted path:
  loop/back-and-forth/once at a speed, walk or run, and a periodic graph action) as actor placements in the level's
  index, dressed by a level spawner, driven by `sf_controller::PathFollower`; coverage markers so every cell under the
  level is active; level `robe-walk` + scene `level-robe-walk` (priest loop, Fargoth walking and Elone running over
  ramp/plateau/stairs; p10c's cloth already simulates on them; TS-2 awaiting owner). Fix: a level worldspace never gets
  `TerrainCamera` (TS-1's black flashes were Vvardenfell terrain patches; 0 patches after a 400 m camera jump).
  Headless course runner `sf_level/tests/course.rs` (2 tests). **Verified (ts1c, 2026-10-19, BRP):** `U` walk mode
  (player dressed at `start`, W 1.6 m/s, Shift 4.5 m/s, follow camera; start/stop logged) and props (`V` balls shove the
  crates); `--camera` fix: `view` repeated flags override (`scene X --camera` over a scene camera was a clap error),
  `level::pick_camera` + tests. TS-1/TS-2 awaiting owner. Leftovers: course cases (vault/mantle once traversal exists),
  compound table, camera collision + jump in walk mode, `--walk` start flag.

- Verify (milestone): PH-1 (table knock-over), PH-2 (dead guard ragdoll + knockdown/get-up), PH-3 (robes) signed off
  by the owner; a benchmark entry for Balmora with physics on (idle step, a collapse, 2 ragdolls) within the gates.
  After M4: pose-search node (`posesearch.cook`) once a bring-your-own locomotion library is baked; mesh cloth for
  capes/banners and buoyancy (M6).

## M5: Mods, Lua and UI

Specs (draft 2026-10-13, m5-spec): `../superfargoth/docs/specs/scripting.md` (Luau runtime, curated API, limits,
hot reload), `../superfargoth/docs/specs/mods.md` (pipeline end to end, reports, hot reload, bake fingerprints),
`docs/specs/scripting-adapter.md` (MWScript → Luau, M5 vs M7 scope), `../superfargoth/docs/specs/ui.md` + `docs/specs/ui-adapter.md`
(UI slice, written 2026-10-11). Decisions in ARCHITECTURE §10 (2026-10-13, m5-spec). **Runtime: Luau through `mlua`
0.12.2 directly** (features `luau`, `serialize`, `send`), one Luau state per mod, one shared state for translated
legacy scripts; `bevy_mod_scripting` 0.22 (on Bevy 0.20) not adopted (reflection-first, "expect significant API
changes"). **MWScript is translated to Luau at bake time** (corpus: 1,207 scripts / 49k lines + 17,144 dialogue results).
Done when (ARCHITECTURE §9): "A Lua mod and a texture mod both work, hot reload works; a theme mod, a CSS patch and a
Lua inventory column work together, gamepad-only."

**Lua parked (owner 2026-10-18):** finish L3 (started), then no further Lua/Luau work (U7, W3, the Lua parts of E1)
until the owner reopens it: "not urgent", and other projects can be scavenged. Candidate: Mudcrab
(github.com/Mudcrab-Team/mudcrab, Skyrim engine on Bevy 0.19, MIT/Apache, Papyrus `.pex` → Luau in sandboxed `mlua`;
`docs/specs/converters/pex-to-lua.md`, `docs/roadmap/03-luau-and-ui.md`): compare with it and adopt or port before
extending `sf_script` (CLAUDE.md rules 9-10). **Checked 2026-10-18 (R10, `../superfargoth/docs/research/animation/10-mudcrab-borrow.md`):
Mudcrab's Luau runtime is a ~105-line sandbox; ours is far ahead, so nothing to wait for. Lua stays parked for
priority only.** UI tasks: U1 done, U2 queued (owner 2026-10-18).

Three lanes that can run side by side once A1 and L1 land: mods + Lua (A, L), UI (U), legacy scripts (W). Tasks in
dependency order:

- [x] A1 (2026-10-13, m5-a1, uncommitted) built: `ModRepository::scan` (adapters' synthesized mods + mods dirs, asset
  roots indexed once, declared files checked), `Profile` TOML (`enabled`, `[settings.<id>]`, `[limits]`), manifest
  `[optional]`/`[conflicts]`/`[ui]`, `ContentAdapter` (discover, mount → layers, record formats by extension),
  `load_profile` (drops broken mods and their dependents with errors, mounts in load order, one record source per
  mod), `ModReport` (every step 1-4 issue type, per-mod override view, text + serde), `ScriptPlan` for
  `sf_script::ContextConfig`; 18 tests, clippy clean; spec changes in mods.md "As built (A1)". Original task text:
  `sf_mods` pipeline core (mods.md "Data model", "Pipeline" steps 1-4): `ModRepository::scan`, `Profile`
  (`profiles/<name>.toml`), manifest `[optional]`/`[conflicts]`/`[ui]`, `api` compatibility check, `ContentAdapter`
  trait, `load_profile`, `ModReport` with every issue type. Verify: `cargo test -p sf_mods` (scan, profile round trip,
  each issue type, optional/conflict ordering).
- [x] L1 (2026-10-13, m5-l1, uncommitted) `sf_script` built: per-mod contexts (and shared multi-mod contexts), sandbox
  escape list, memory limit, latched watchdog, fault policy, require over `ScriptSource` (impl for `sf_vfs::Vfs`),
  `@sf/core|events|timers`, storage, named timers, `StateValue` snapshots, events next update, reload; 16 tests, clippy
  clean, both `send` modes. Spike: decision 1 confirmed; `send` on, app-data world guard, `luau-jit` off; budgets
  corrected in scripting.md "Spike results (L1)" (context 394 KiB, save/restore ~30 ms/MiB). Original task text:
  `sf_script` core + spike (scripting.md "Design", Bevy-free): per-mod contexts, sandbox, memory limit,
  interrupt watchdog, require-by-string over `sf_vfs` (own files, `@sf/*`, dependency libraries), `storage`, named
  timers, state serialization (postcard) + rejection paths, error isolation and suspension, per-context reload.
  Decide by measurement: `send` vs `NonSend`, world guard (`scope` vs guarded pointer), `luau-jit` off/on.
  Verify: `cargo test -p sf_script` (sandbox escape list, limits, isolation, round trip, determinism hash, reload keeps
  state); `cargo bench -p sf_script` numbers written into scripting.md "Performance budget"; clean-build delta of
  `luau0-src` recorded in the session log.
- [x] W1 (2026-10-13, m5-w1, uncommitted) built: `mw_script` (Bevy-free, no deps): lenient lexer + line parser with 40
  named rules (accepted / repaired / warning / error = inert), `functions.txt` table (452 functions: arg kinds, returns,
  status family), AST with `VarRef` = where state lives (no Luau globals), `resolve` over an `Env`. Real install: 1,231
  script texts (1,207 live) + 17,144 results parse, **0 unrecoverable**, per-rule counts in scripting-adapter.md "As
  built (W1)"; 11 tests, clippy clean. Original task text: `mw_script` lexer + lenient parser + function table
  (scripting-adapter.md "Pipeline" 1-2), Bevy-free, can run in parallel with everything after A1/L1. Verify: real-install corpus test parses all 1,231 script texts and 17,144
  dialogue results, leniency rules counted, 0 unrecoverable (or each listed with a reason); counts only, no vanilla
  text committed.
- [x] A2 (2026-10-13, m5-a2, uncommitted) built: kind registry (`register_serde`, fingerprints blake3+postcard,
  `set_fingerprint`), native RON/TOML record files (`apply_record_file`, `sf_mods::NativeRecords` adapter), generic
  `set`/`merge`/`remove`/`append` over a serde value tree with raw-RON leaves (enums, Options, tuples, map keys), every
  written field path checked, field-level touches, conflict vs merge in `ConflictReport` and as per-mod `ModReport`
  issues, positioned errors per operation; sf_records 20 + sf_mods 19 tests, clippy clean; real-install patch check
  waits for A3 (TES3 types need serde). Original task text: `sf_records` native record files (mods.md "Native record
  files"): kind registry with serde + fingerprints,
  RON `define`/`patch` (`set`/`merge`/`remove`/`append`)/`delete`, field-level touches, conflict vs merge in the report.
  Verify: `cargo test -p sf_records` (every op on nested types, field conflict vs merge, RON error positions).
- [x] W2 (2026-10-13, m5-w2 + m5-w2b, uncommitted) built: `mw_script::emit` (module per script, module per topic
  with a function per result, all state in module tables, `-- L<n>` source lines, inert → stub module), `coverage`
  (call sites by function/family/status, `oracle_flags`), `functions.txt` status column (live/stub/stub=n/later),
  `luau/mw.luau` runtime skeleton; `mw_bake::script` (`mwscript.translate` per `script/<id>` and `topic/<topic>`:
  Luau + level-2 bytecode; `mwscript.coverage`), in `bevyrrowind bake`. Real install: 1,207 scripts + all 17,144
  results (15,852 live in the merged DB) → 0 inert, 0 compile errors, 0 errors running each once in a sandboxed
  `sf_script` context; translate 0.10 s + compile 0.20 s single-threaded; bake 1.4 s CPU summed (5.6 s wall cold,
  mostly store writes), 0.19 s no-op. Coverage (live DB): 25,449 call sites, 57.1 % live, 5,320 stub, 5,590 later.
  Oracle-pending: 0 integer divisions, 44 short/float comparisons. 18 mw_script tests, clippy clean. Details and W3
  handoff in scripting-adapter.md "As built (W2)". Original task text:
  MWScript → Luau emitter + `mwscript.translate` bake step + `mw` runtime skeleton + coverage report
  (scripting-adapter.md "Generated shape", "The `mw` runtime library"). Verify: every vanilla script and dialogue
  result translates and compiles under Luau; corpus ≤ 2 s across cores; fixture goldens (our own scripts); coverage
  report (call sites live / stub / later) in the session log.
- [x] A3 (2026-10-13, m5-a3, uncommitted) built: new Bevy-free crate `mw_mods` (`MwAdapter`: one synthesized mod
  per ESM/ESP with masters as dependencies, `KNOWN_ORDER` as `load_after`, its same-stem ini BSA as its layer;
  `morrowind-data` = loose Data Files + unclaimed BSAs after the plugins; builtin mod `mw` = mw_graphs + the `mw`
  Luau runtime at `scripts/mw/`, pinned first; generated default profile; `UserDirs`), TES3 kinds `register_serde`d
  (tes3 `serde` feature), bake record fingerprints = `RecordDb::fingerprint` (CELL included), profile fingerprint
  (sf_mods, stats from the directory listing via new `VfsSource::stat`) stored beside the bake manifest,
  `ShadowedModern` warning, every app command loads through `--profile`/`--user-data`, `bevyrrowind mods
  list|order|report|enable|disable`. Real install: 62,025 live records, load 0.29 s (was ~0.42 s); RON patch on
  `light_com_candle_01` shows `dim_candles (patch data.radius)` in `mods report`; texture mod wins in `which` and
  re-bakes nothing; no-op bake 3.0-3.2 s total (**gate 2.5 s missed by other steps**: exterior source 0.52 s, model
  summary 0.37 s, models 0.33 s, interiors 0.25 s; content load is 0.29 s of it). Details: mods.md "As built (A3)".
  Original task text: A3 Bevyrrowind profile (mods.md "Mods directory and profiles"): `<user data>/mods` + `profiles/default.toml`,
  legacy content as synthesized mods through `ContentAdapter` (`mw_records`, `mw_vfs`), builtin mod (graphs, themes,
  `mw` runtime) mounted lowest, `bevyrrowind mods list|order|report`, profile fingerprint in the bake manifest.
  Verify: `info` with the vanilla profile still loads 62,025 live records; a loose texture mod wins in `which`; release
  `bake` no-op ≤ 2.5 s, and adding a texture-only mod re-bakes nothing (or only terrain layers if a land texture changed).
- [x] L2 (2026-10-13, m5-l2, uncommitted) built: `sf_script_bevy` (`Scripts` resource around `ScriptRuntime`,
  `ScriptsPlugin`, `ScriptSlot` Input/Update/Ui exclusive systems in Pre/Update/PostUpdate, `Activated` →
  `"activated"` → `ActivationOutcome { handled }`, generic `ScriptFact` per slot, `ScriptDiagnostics` + `recent()`,
  `script/...` sf_diag counters per slot and context), **world access = declared slot resources moved into one cell
  shared by all contexts** (cost per slot independent of the context count; "outside a script slot" error),
  `ScriptPlacements` handle map + `@sf/world` L2 subset (`player`, `is_loaded`, `record`, `worldspace`, `position`);
  4 headless tests, clippy clean. App: `scripts.rs` (contexts from the profile's `script_contexts()` + shared
  `mw.legacy` owned by `mw`, R0 spawner hook → handles, scaffold `CrosshairPick` on Space, `--activate N`,
  `CellLoaded` → `"cell_loaded"`). Real run: a test mod in a scratch `--user-data` logs activation of
  `furn_com_tapestry_03` in the Census office (handle, position, player actor) and its timer fires. Not in L2 (to
  L3/W3/A4): entity scripts + overlay state, spawn, dev console, doors honouring `handled`, hot-reload wiring.
  Details: scripting.md "As built (L2)". Original task text:
  L2 `sf_script_bevy` (scripting.md "Execution model", "Entity handles"): `Input`/`Update`/`Ui` exclusive slots,
  world guard, `PlacementId` handles, entity scripts with state in the placement overlay, engine messages
  (`Activated`, `CellLoaded`, `InputAction`, `Died` when it exists), activation pipeline with `"handled"`, per-mod
  `sf_diag` spans and error counters, dev Lua console. Generic `Activated` + crosshair target: share with M4 P11's
  `sf_interaction` (whichever lands first builds it; doors.rs moves onto it). Verify: headless integration test (mod
  reacts to `Activated`, spawns, sets a timer, edits storage; a suspended mod doesn't block others).
- [x] A4 (2026-10-13, m5-a4, uncommitted) built: `sf_vfs` `SharedVfs` (`arc-swap` 1.9, lock-free load/store),
  `Vfs::rebuild(dirty)` (rescans only dirty `DirSource` layers via new `VfsSource::watch_root`/`rescan`, layer positions
  kept, diff = winner's layer or indexed stat changed, shadowed edits silent), feature `watch`: `VfsWatcher` on
  `notify-debouncer-full` 0.7 / `notify` 8.2 (Bevy's own pair, newest stable; Bevy's `FileWatcher` itself is one dir per
  source with no shadowing), `bevy::shared_asset_source` (Bevy's event channel via `with_watcher`, winning
  `Added/Modified/RemovedAsset`); `sf_mods` `LoadedProfile::route` → `HotReloadPlan` + `reload_records` (whole-DB
  step 4 re-run, record issues replaced); `sf_script_bevy` `Scripts::reload_changed`. App (`dev` feature only):
  `hot_reload.rs` (watched default source, routing, `LiveRecords` + `RecordsChanged`, reloaded-image log). Tests:
  sf_vfs 15 (rebuild/winners/shadowed, swap never blocks readers, real watcher), sf_mods 20 (RON patch re-applies),
  sf_script_bevy 5 (reload keeps state); clippy clean. Real run (release + dev, scratch `--user-data`, Census office):
  editing the test mod's `main.luau` → "script context hello_activate reloaded (state kept)", new code logged
  "activation count kept = 2" ~0.25 s after the save; editing its `tx_imp_wall_01.png` → "texture reloaded" ~0.15 s
  after; rebuild 11 ms. Not built: dev re-bake of changed bake inputs during play (logged, "run `bevyrrowind bake`"),
  CSS route (U2), MS-1 scene (E1's `texture_demo`). Details: mods.md "As built (A4)". Original task text:
  A4 Hot reload (mods.md "Hot reload"): VFS watcher over `DirSource` layers (Bevy `file_watcher` / notify 8.2),
  index rebuild + `Arc<Vfs>` swap, winning-change `AssetSourceEvent`s, routing (assets, CSS, scripts, record files,
  dev re-bake of affected products). Verify: `cargo test -p sf_vfs --all-features` (winner changes, shadowed edits
  silent, readers never block); MS-1.
- [x] L3 (2026-10-13, m5-l3; compiled + committed 2026-10-18 wipfix, superfargoth aed6116 / bevyrrowind 8c8efbb;
  real run 2026-10-18 l3fin) **done for v0.1**. Real run (dev exe, scratch `--user-data` with `hello_activate` +
  `l3_test`, Census office, `--activate 4` on `furn_com_tapestry_03`): `on_load start` logs the player position;
  activation 1 attaches the `counter` entity script and spawns two copies (LiveSpawn: 1 entity each, visible in
  the room, `shots/l3/l3_spawn.png`); 2 → counter `n = 1`, default action; 3 → `n = 2`, "handled by a script",
  temp copy despawned; 4 → handled; `tick` ages the state ~1 s per second; `hello_activate`'s timers still fire.
  Space on the office door (`door-census` camera, `--activate 1`): scripts see it, leave it unhandled → door
  travel to Seyda Neen (`shots/l3/l3_door.png`). No fixes needed. **Parked (owner 2026-10-18, Lua parked):**
  `@sf/records`, `@sf/input`, `@sf/anim`, `@sf/physics`, MS-2 (not built). As built: API v0.1 `@sf/core`
  (+ `version`, `settings()`, `game_time()`, `on_load`), `@sf/events`, `@sf/timers`, new `@sf/interfaces` (sync cross-mod calls, copied plain data, callee
  watchdog, cycles rejected), `@sf/world` (+ `nearby`, `spawn`/`despawn` via `WorldCommand` queue, entity scripts
  `define_script`/`attach`/`detach`/`state` with state in the mod's snapshot, activated before global handlers, `tick`
  while loaded), handle methods (`h:position()`), **`on_load` = queued, runs at the next `Update` slot (world access)**,
  generated `superfargoth/docs/api/sf.d.luau` (golden test), example mod `superfargoth/examples/mods/api_tour` run
  headless; mlua `serialize` dropped. sf_script 22 + sf_script_bevy 8 tests, clippy clean. App: `LiveSpawn` scaffold
  applies `WorldCommand`s, doors open from unhandled `ActivationOutcome`, `game_time` from the sky clock. Details:
  scripting.md "As built (L3)". Original task text:
  L3 API v0.1 (scripting.md "Curated API"): `@sf/core`, `events`, `timers`, `interfaces`, `world`, `records`,
  `input`, thin `anim` (slots) and `physics` (impulse, raycast); `sf.d.luau` + docs generated from the registry.
  Verify: golden `.d.luau`; example mods in `superfargoth/examples/mods/` run headless; MS-2.
- [x] U1 **Built part 1 (u1a, 2026-10-18):** input crate `bevy_enhanced_input` 0.27 (ui.md OQ4 closed);
  `superfargoth/crates/sf_ui` (UI record kinds, `UiRegistry` + conflicts, `ScreenStack` routing rule, `ViewModel`/
  `ListModel`, `SettingsStore` over profile tables, Fluent `Localizer`), 14 tests, clippy clean; ui.md "As built
  (U1a)". **Built part 2 (u1b, 2026-10-18):** `superfargoth/crates/sf_ui_bevy` (`SfUiPlugin`): router = one BEI
  context entity per screen input context id, only the input target active, above gameplay, every UI action
  `consume_input` + `require_reset`; HUD contexts carry only open menu/map; `UiBindings` rebinding (menu open
  included); confirm → `bevy_ui_widgets::Activate`; Back closes menu/modal targets; Bevy `AutoDirectionalNavigator`
  kept to the target screen, focus memory per screen, held-direction repeat; `InputMode` last-used device + glyph
  table. Verify ran: 6 headless router tests; example `ui_nav --autotest` drives HUD → menu → modal → back with a
  synthetic gamepad (40 checked steps, PASS); visual check UI-1 (real pad) awaiting owner. ui.md "As built (U1b)".
  **Built part 3 (u1c, 2026-10-18):** glue to the mod pipeline: Fluent sources from each mod's VFS
  `l10n/<locale>/<mod id>.ftl` in load order with per-mod issues (`Localizer::from_profile`; a file named after
  another mod is reported and ignored), `UiRecordKinds` adapter, `SettingsStore` ↔ `Profile::settings`, new
  `Profile::bindings` (`[bindings.ui]`, reflected input names like `key:Ctrl+KeyS`, `pad:South`, non-defaults only),
  `insert_from_profile` + `UiProfile` (saved when the player leaves the menus or on exit), `UiCommands` param,
  `ScreenDef::presets` enforced (`push_in`). Verify ran: 3 + 4 new tests (two mods × two locales + override,
  settings and bindings across a restart through the profile file, presets), clippy clean, `ui_nav --autotest`
  PASS. ui.md "As built (U1c)". Leftovers: the game app doesn't call `insert_from_profile` yet; `UiSettings` not fed
  from `ui.setting` values; PlayStation/Switch glyph names. Original task text:
  U1 `sf_ui` (Bevy-free registry, view-models, keyed list models, settings registry, Fluent) + `sf_ui_bevy`
  (CommonUI layers, screen stack, input router, actions with rebinding, `bevy_input_focus` navigation, device glyphs).
  Input crate: `bevy_enhanced_input` 0.27 and `leafwing-input-manager` 0.22 are on Bevy 0.20 (crates.io 2026-10-14/16):
  no fork needed; pick one in ui.md OQ4.
  Verify: headless registry/router tests; gamepad-only navigation in a framework example.
- [x] U2 `sf_ui_css`: **done (u2 superfargoth c3d1fcb, u2b)**: composer (flat layers `mods-<n>-<id>`, flair 0.9 has no dotted
  layer names), sources `sf_ui` (memory root) + `sf_ui_content` (app-registered), root classes + `input` attribute, per-mod
  errors via strict probe loads (`UiCssReport`); 6 tests pass (layer order = load order through flair, errors → mod id),
  clippy clean; licence set in ui.md "As built (U2)". `ui_css --autotest`: a mod sheet edited on disk restyles in 0.33 s
  through Bevy's loader-dependency reload (no extra code), before/after shots `shots/u2b/`; UI-2 awaiting owner;
  `ui_nav --autotest` PASS. Original task text:
  U2 `sf_ui_css`: `bevy_flair` 0.9.0 from crates.io (on Bevy 0.20 since 2026-10-14; MPL deps allowed unmodified, owner
  2026-10-11), composer with `@layer mods.<id>` in load order, per-mod error attribution, CSS hot reload.
  Verify: composer tests (layer order = load order, errors → mod id); edit a sheet in the example, see it apply.
- [ ] U3 Widget kit + both themes (modern; `mw_classic` with TTF fonts and the `ui.frames` bake, ui-adapter.md) +
  both presets + scale settings. Layouts as `bsn!` templates until Bevy 0.21 ships `.bsn` files (owner decision 2
  below). Verify: screenshot matrix theme × preset × {100 %, 200 % text at 1280×720}; AccessKit pass with Narrator.
- [ ] U4 HUD shell with slots (bars, notifications) + message box (timed and modal with buttons). Verify: gamepad-only
  modal; HUD ≤ 0.5 ms CPU in `sf_diag`.
- [ ] U5 Virtualized inventory on the **minimal `sf_rpg` inventory stacks** (record id → count; shared with W3) plus a
  1,000-item synthetic container: `ui.column` records, sort/search/tabs, tooltip slot. Verify: `sf_diag` scenario
  scrolling 1,000 items at full frame rate, heavy menu ≤ 1 ms CPU, ~40 live rows.
- [ ] U6 Settings + keybinds + mod settings pages (settings registry records, values per profile). Verify: a mod's page
  appears, edits persist in the profile, gamepad-only.
- [ ] U7 `sf_ui_lua` (`@sf/ui`, ui.md "Luau surface"): slots, columns, screens, settings pages, message boxes; per-mod
  UI time. Verify: the value-per-weight column mod works in both presets; HUD Lua ≤ 0.2 ms.
- [ ] W3 Legacy runtime (scripting-adapter.md "Instances", "What M5 needs from M7"): `mw_script_bevy`, shared
  `mw.legacy` context, global/local instances, MWScript active-cell region, edge flags, activation consumption,
  message boxes on U4, **minimal `sf_rpg` quest store** (quest index, entries, known topics; no journal UI) and
  inventory stacks (U5), live families per the spec table, neutral stubs for the rest, console snippets. Verify:
  behaviour fixtures (OnActivate once, message-box state machine, timers, short wrap, `id.var`, save/restore, reload);
  Balmora 5 min with all vanilla start/local scripts: 0 runtime errors (or each listed), legacy script time ≤ 0.3 ms;
  MS-4, MS-5.
- [ ] E1 Milestone demo + gates: example mods in `bevyrrowind/examples/mods/` (texture replacer, Lua door-notes mod
  with a settings page, theme mod, CSS patch, value-per-weight column), `sf_diag` scenario (Balmora, inventory open
  and scrolling, 10 mods), visual checks MS-1..MS-6 in `docs/visual-checks.md`. Verify (milestone): the §9 "Done
  when" sentence, gamepad-only, owner sign-off on MS-1..MS-6; benchmark entry: script slots ≤ 1 ms, UI budgets per ui.md.
- [ ] X1 (stretch; may move to M7) ESP field-level merge (§6.1 item 4): each ESP record diffed against its master so
  only changed fields become patches; conflict report per field for legacy plugins. Verify: two ESPs editing
  different fields of one record both apply; TES3Merge-style cases as fixtures.
- [ ] X2 (after M5) native archive mods (zip). Verify: a zipped copy of the texture mod behaves like the folder.

Owner test scenes (added to `docs/visual-checks.md` by the task that builds them, status "awaiting owner"):

- MS-1 texture mod: enable `examples/mods/texture_demo` (a loud test-pattern replacement for a texture in
  "Seyda Neen, Census and Excise Office"); the pattern shows; edit the file while the game runs (dev build): it updates
  within a second, no restart.
- MS-2 Lua mod: `door_notes` shows a notification naming each door you activate and counts them in its storage; edit
  the message text in the `.luau` file while running: new text, count kept.
- MS-3 UI together: theme mod (token colors) + a CSS patch in its own layer + the value-per-weight inventory column,
  inventory open in both presets, theme switched live, **gamepad only** (mouse unplugged).
- MS-4 legacy script: "Balmora, Temple", activate the Shrine of St. Rilm (`shrineRilm`) with gamepad: donation prompt
  (35 gold, non-member stub) → blessing menu → choice; 35 gold leaves the inventory, the cast is logged as a stub.
- MS-5 legacy activation override: a door whose script asks before passing (vanilla `passageScript`; find a placed
  instance with `uses`): "Yes" travels, "No" stays.
- MS-6 mod settings: the door_notes settings page (toggle, gamepad), 200 % text at 1280×720 lays out without loss.

**Owner decisions for M5** (recommendations; details in ARCHITECTURE §10 2026-10-13 m5-spec):
1. Runtime: Luau via mlua directly, state per mod, shared state for legacy translations, bevy_mod_scripting not
   adopted. Recommend: yes.
2. `.bsn` files: Bevy 0.21 isn't out (0.20 shipped 2026-10-08), so M5 layouts are `bsn!` templates in Rust and
   move to `.bsn` files when 0.21 lands; M5 counts as done without `.bsn` files. Recommend: yes.
3. Minimal `sf_rpg` data (inventory stacks, quest store) built in M5 ahead of M7, so scripts and the inventory slice use
   real data. Recommend: yes.
4. Dev-build hot re-bake of a changed product during play (exception to "nothing bakes during play", dev and "mod author
   mode" only). Recommend: yes.
5. Script faults: abort a handler after 50 ms (release), suspend a mod for the session after 3 aborts or 30 errors a
   minute, memory default 64 MiB per mod. Recommend: yes, tune after L1.
6. Framework script API stays `0.x` (no compatibility promise) until M7's RPG modules exist, then `1.0`. Recommend: yes.
7. X1 ESP field-level merging inside M5 (stretch) or in M7. Recommend: after E1, keep M5's done-when unchanged.

## M6: Water, weather, surfaces

Specs: `../superfargoth/docs/specs/weather.md` + `docs/specs/weather-adapter.md` (weather, clouds, fog, wind, precipitation,
surface state, distant shadows; 2026-10-11), `../superfargoth/docs/specs/water.md` + `docs/specs/water-adapter.md` (water;
draft 2026-10-13, m6-spec), `../superfargoth/docs/specs/sky-lighting.md`; HD grass needs its own spec first (G0). Decisions
in ARCHITECTURE §10 (2026-10-13, m6-spec). **Water: our own `sf_water` + `sf_water_bevy`, porting bevy-aqua's wave, foam
and optics code (MIT/Apache)**; bevy-aqua 0.1.3 itself is on Bevy 0.19 (no 0.20 branch, last commit 2026-09-23), has no
floating origin, one global bed image, GPU-readback queries and planar mirror cameras, so most of it would be rewritten.
Done when (ARCHITECTURE §9): "The Bitter Coast in a rainstorm: puddles, wet wading, a boat that floats; an ash storm at
Red Mountain; sunrise god rays in Balmora; Bitter Coast morning fog; all within the weather.md budgets."

Already built (not repeated here): S1 volumetric clouds + cloud shadows (clouds2/3, `sf_atmos`), distant shadows tier 1
(CSM 504 m) and tier 2 v1 (terrain horizon march + CPU shadow-height map, shadows2), S2/S3 sun disk and night sky,
night exposure floor + point-light shadow budget (lighting). Next up 6b's last bullet ("then fog") is F1-F3 below.

Lanes (the orchestrator runs at most 3 agents): **weather** (F, WD, WE, SU, PR, C), **water** (W), **grass and shadows**
(G, T). Tasks in dependency order; a task names what it waits for.

- [~] F1 Haze and dust now (weather.md N1/N2; Next up 6b): `--haze` per-time-of-day curve on `sf_sky::Weather::haze`
  (2-3 clear, 4-8 Bitter Coast mornings), absorbing dust term in `AtmosphereModel` (`Weather::dust`), until `sf_weather`
  drives both. Skip N3 (Bevy `VolumetricFog` prototype; owner decision 7). Verify: `summit_vivec`, `redmtn_view`,
  `bittercoast` scenes at 3 haze values and dust 0/1 (FG-1, owner picks defaults); GPU unchanged (LUT rebuilds only on
  change).
  Built (m6-f1, 2026-10-13): `Weather::dust`/`dust_tint`, dust term in `AtmosphereModel::earth`, `WeatherScale`, rate-limited
  rebuild (step + 0.25 s, `MediumStats`), app `--haze`/`--dust` + per-weather table + morning curve following `--clouds`
  (weather-adapter.md "Haze and dust"). Haze recalibrated for 1-5 km views (clear 20, not 2-3). Rebuild ~2.8 ms CPU,
  GPU unchanged. Open: FG-1 (owner picks defaults), sea-horizon seam at noon with dust, frame-time A/B not run.
- [x] WD1 `sf_wind` + `sf_wind_bevy` (weather.md "Wind"): `WindState`, `WindSource` (directional, radial, vortex, ≤ 32
  in a storage buffer), gust field ported from Megalophobia `grass_wind.wgsl` (licensed 2026-10-12), CPU `velocity_at`
  + WGSL `sf_wind::sample` twins, origin phase, altitude profile; `sf_sky::Wind` moves here (re-exported). Clouds read it.
  Verify: `cargo test -p sf_wind` (CPU/WGSL twin within 1e-4 on golden points, gusts never upwind, phase-exact across
  an origin shift); framework example `wind` (arrow field + a radial burst); upload ≤ 0.02 ms CPU.
  Built (wd1a, 2026-10-18): Bevy-free `sf_wind` (weather.md "Wind as built"): PCG lattice hash, 1024 m period with a
  split whole-cell + fraction origin phase, CPU-integrated drift, log profile, 32 sources with envelopes;
  `sf_wind::WESL` (pure functions); `sf_sky::Wind` re-exported. Twin run headless on the GPU (`wgpu` dev-dep): 1.1e-5
  relative; phase test 2.9e-5 m/s.
  Built (wd1b, 2026-10-18): `sf_wind_bevy` (weather.md "Wind as built (WD1 part 2)"): `WindPlugin`, `WindField` (state
  + this/last frame CPU twin), app-written `WindOrigin`; one fixed 3 264 B `ShaderBuffer` (params, prev_params,
  sources[32], prev_sources[32]) rewritten in place, so materials bind it with `#[storage]`; motion vectors from the
  previous copy re-framed at the current origin; WESL `sf_wind::field` + `sf_wind_bevy::wind` (`wind_sample`,
  `wind_sample_prev`; slot from `SF_WIND_GROUP`/`SF_WIND_BINDING` defs). Upload 0.0025 ms (32 sources; gate 0.02).
  Example `wind` ran (CPU arrows agree with the GPU-shaded ground, burst; visual check WD-0). App: weather wind →
  `WindField`, terrain origin → `WindOrigin`, fog drift and cloud direction read the field (clippy clean; not run in
  game). Open: no GPU consumer in game yet (grass G1, trees WD2, rain PR1); `WindTerms` easing is WE1.
- [x] WE1 `sf_weather` core (weather.md "Weather: state machine and presets as data"): `WeatherPreset`/`Climate`/
  `PrecipKind` records, per-climate state machine, forecast as a pure function of (seed, climate, index), channel
  lead/lag blending, border band, lightning schedule, cues, tags, `WeatherFrame` + `visibility_m`. Bevy-free.
  Verify: `cargo test -p sf_weather` (forecast determinism + save/restore with overrides, lead/lag curves, discrete
  crossfades never morph, border blend, Poisson lightning rate); `evaluate` ≤ 0.05 ms.
  Built (we1, 2026-10-18; weather.md "Weather core as built"; superfargoth cbf4c44): records as `sf_records` kinds
  (`weather.preset`/`.climate`/`.precip`, all `serde(default)`), `WeatherBook` (index form), `WeatherState { seed,
  overrides }` (forces and weight edits are timestamped overrides, not record patches); slots `phase + k·mean + hashed
  jitter` (O(1) lookup), hashed weighted draw; 6 channels (clouds/sky/wind/fog continuous; precipitation and cues
  crossfade with in/out windows); per-preset wind wander, `sky.wind` = the `WindField` target; lightning by thinning a
  hashed Poisson stream (cap 120/game hour); `storms` (climate storm source × preset `wind.storm`); `visibility_m`
  (Koschmieder). Depends on `sf_atmos` (frame carries `CloudWeather`/`FogSettings`); `RecordId` gained serde.
  10 tests green, clippy clean; `evaluate` 0.0006 ms (release, 2 climates mid-transition). Open: overrides never
  pruned; `gust_length` swaps once mid-transition; lightning rate per game hour (adapter converts).
- [x] WE2 `mw_weather` (weather-adapter.md): REGN → climates, `Morrowind.ini [Weather*]` → presets in a synthetic mod
  layer, precipitation kinds (rain, snow, ash, blight), Red Mountain radial storm source, MWScript mapping table for M5
  W3, `bevyrrowind weather forecast <region> --days 3`. Waits for WE1. Verify: `cargo test -p mw_weather` (adapter
  spec list); forecast frequencies within sampling error of REGN weights over 1,000 changes; ini edit re-runs only
  `weather.climate`.
  [~] Built part 1 (we2a, 2026-10-18): Bevy-free `mw_weather`: `MwWeather::build(db, ini)` → 10 presets
  `mw_weather/<type>`, one climate per REGN `mw_region/<id>` (weights in REGN index order, zero chances dropped,
  change interval `Hours Between Weather Changes` ± 25 %, transition = chance-weighted `Transition Delta` read as Hz in
  real time, × timescale), `mw_precip/{rain,snow,ash,blight}`; precipitation intensity/start threshold, loop-sound cues,
  lightning `per_hour = f × 60 / timescale` + thunder ids, blight tag; vanilla fallbacks per key, ini fingerprint;
  `define_into` = synthetic source "Morrowind.ini weather". `MorrowindIni` keeps every value (`get`). 6 unit tests +
  real install (16 climates), clippy clean. Part 2 (we2b): clouds/fog/sky/wind terms, Red Mountain radial storm,
  region sounds, MWScript mapping table, `weather forecast` command + 1,000-slot frequency check.
  [~] Built part 2 (we2b, 2026-10-18): `mw_weather::visual`: clouds = genus per type (the app's `CLOUD_CHOICES`) at
  half heights, speed `Cloud Speed` × 10 m/s; sky haze/dust/tint + morning-haze `haze_by_hour`, fog (Koschmieder
  height fog, banks, ash/blight albedo, dawn `height_by_hour`/`banks_by_hour`) = the app's `SKY_*`/`FogLook` values;
  `Wind Speed` 0-1 → 1-15 m/s ± 25 %, ash/blight `wind.storm = 1`, climates with ash or blight get a radial
  `storm_source` at the Red Mountain summit (highest LAND vertex, cell 3,9; 6 m/s, 3 km); `Flash Decrement` →
  `Lightning::fade_s` (new in sf_weather, + `pub use` of sf_atmos/sf_sky/sf_wind). Tests: presets match the app tables
  over 24 h (WE3 switch-over is a no-op), storm blows outward from 4 sides; 11 tests green, clippy clean. Part 3
  (we2c): region sounds, MWScript table, `weather forecast` + frequency check, app calls `define_into`.
  [~] Built part 3a (we2c1, 2026-10-19): region sounds → sf_weather `OneShotSet` (new, generic "scatterer": sounds with
  a chance per draw, rest silence, interval in real seconds; the audio layer schedules) on `Climate.one_shots`, reported
  as `WeatherFrame.one_shots`; `WeatherPreset.climate_ambience` (0..1, cue channel) hushes the climate's loops and
  one-shots, 0 for weathers with an `Ambient Loop Sound ID` (ash, blight, blizzard). REGN chance % / 100, interval = ini
  `Min/Max Time Between Environmental Sounds` (vanilla 1-5 s). `mw_weather::script`: `ChangeWeather` → `force`,
  `ModRegion` → `set_weights` (all zero = Clear), `GetCurrentWeather` → `current` (the target at once), `GetWindSpeed`
  → inverse of the `Wind Speed` map; table in weather-adapter.md. 13 mw_weather tests (2 real install) + 11 sf_weather
  green, clippy clean. Part 3b (we2c2): `weather forecast` + 1,000-slot frequency check, app calls `define_into`.
  [x] Built part 3b (we2c2, 2026-10-19): the records load as the synthesized mod `morrowind-weather` (mw_mods: after
  the plugins + `morrowind-data`, before user mods, so record mods patch them; built from every plugin's REGN + ini when
  its generated marker record file applies; `ensure_weather` keeps it in profile files; stamp = ini size/mtime);
  real install: 16 climates, 10 presets, 4 precip kinds in `info`. `bevyrrowind weather forecast <region> [--days 3]
  [--seed] [--from]` prints one row per change slot + counts. Frequency tests: synthetic (3 regions × 2 seeds) and
  real install (16 regions), 1,000 slots each, within 4.5 σ. Verify: tests + frequencies met; "ini edit re-runs only
  `weather.climate`" moot (no weather bake step: records rebuilt at every load). Leftovers: positional one-shots;
  REGN edits from an ESP in a folder mod ordered after `morrowind-weather` are ignored (weather-adapter.md).
- [~] WE3 `sf_weather_bevy` + app (weather.md "Canonical technique"): `WeatherPlugin` (clock → evaluate → sky haze/dust,
  clouds genus blend replacing `CLOUD_CHOICES`, wind, fog, precip, surface rates), lightning flash light + cloud
  emission, cue messages, `--weather <type>`, region border blend; `SunDiskLook` glare per weather. Waits for WE2, WD1.
  Verify: time-lapse clear → overcast → rain over 2 game hours (WE-1: clouds lead, rain lags, no pops); Ascadian Isles →
  Ashlands crossing (WE-2: border blend); `sf_diag` `weather/*` counters; CPU ≤ 0.05 ms.
  **Part 1 done (we3a, 2026-10-19):** new `sf_weather_bevy` (`WeatherPlugin`: clock → evaluate at the app-written
  `WeatherSite` → `CurrentWeather`; drives sky haze/dust/tint and the wind field + storm sources; `weather/*` counters;
  evaluate 0.0027 ms); app maps CELL grid → REGN climate, `--weather <type>`, `SkyLook` reduced to fog, `--clouds`
  defaults to the start weather; WE-0 awaiting owner. **Part 2 (we3b):** cloud genus blend (keep `CloudLook.scaled(0.5)`),
  fog, sky cover/dimming, precipitation, lightning/thunder, region border blend, WE-1/WE-2.
  **Part 2a done (we3b1, 2026-10-19):** clouds (genus blend + speed), fog (height, banks, albedo) and sky
  cover/dimming follow `CurrentWeather` (`sf_weather_bevy` `apply_clouds`/`apply_fog`; no pops: per-channel windows,
  integrated scroll); app tables `CLOUD_CHOICES`/`SkyLook`/`FogLook`/`SKY_*` removed, `--clouds` = genus pin; live
  controls C (next weather), T (clock x1/x4/x20), BRP `set weather/transition_hours/time_scale`, `status.weather`;
  WE-1 awaiting owner. **Part 2b done (we3b2, 2026-10-19):** precipitation (new `sf_precip_bevy`: closed-form streaks/
  flakes in 3 camera-wrapped boxes, wind slant, ash/blight as blowing streaks; no particle crate: `bevy_hanabi` not on 0.20),
  lightning (pure `flash_at`, shadowless flash light, cloud ambient glow as the emission stand-in), `WeatherSound`
  messages (loops; thunder after distance / 343 s), region border blend in `weather_site`; WE-2 awaiting owner. CPU:
  `cost_per_frame` gate 0.05 ms holds. **Left for [x]:** owner sign-off WE-1 + WE-2; `SunDiskLook` glare per weather;
  audio playback of `WeatherSound` (no audio backend yet); rain occlusion under roofs/trees is PR1's.)
- [~] WE4 Distant weather (owner go 2026-10-19; after WE3): see weather in other regions from afar (a storm over the
  Ashlands seen from Balmora, rain shafts under distant clouds), as RDR2 / MSFS / Horizon do. Research first (weather.md
  has no spatial weather yet): evaluate neighbouring regions (pure function, ~0.003 ms each) into a coarse weather map
  around the camera that the volumetric clouds sample (coverage, genus, darkness per location), precipitation shafts
  and lightning flashes under distant cells, far fog by location. Co-op: the map is a pure function of (seed, clock,
  regions), so it replicates for free. Verify: WE-3 from a ridge with a forced storm in a neighbouring region; cost
  in the cloud pass at 1440p.
  Part a built (we4a, 2026-10-19; weather.md "WE4a as built"): `sf_weather::WeatherMap` (Bevy-free): `ClimateRaster`
  (region raster, sea → nearest region by Euclidean dilation, `site_at` = border blend), per-climate table (each climate
  evaluated once at weight 1, 31 continuous channels), world-anchored toroidal texel window (1/8 recenter steps, strips
  only), uniform-day skip; `sample(camera)` = `evaluate(site)` within 1e-5 on a 2 m border walk (5 tests). Table 0.0034 ms
  for 16 climates (release; estimate 0.02). Next: WE4b GPU map + clouds read it, then WE4c app (`site_at` replaces
  `ClimateMap::border_at`, dilated sea raster, WE-3 scene).
- [~] F2 Height fog, ground fog and far fog (weather.md "Fog" rows 2-3): analytic exponential height fog (UE formula, the
  Low preset path), Megalophobia ground-fog bank density with a time-of-day curve, far fog raymarch at ¼ res inside the
  cloud node, `DistanceFog` off on the exterior camera. Waits for WE3 (or `--fog` flags until then). Verify: FG-2
  Bitter Coast 6:30 foggy (banks in the swamp, continuing past 400 m from a ridge); A/B `--off fog`; far fog ≤ 0.1 ms
  over the cloud pass at 1440p.
  Built (m6-f2, 2026-10-13; weather.md "Fog as built"): `sf_atmos::fog` (settings, drift, CPU twin, 6 tests); in the
  cloud node the trace marches the banks (16 steps, to 6 km) into a per-pixel modulation, a 66×1 `clouds_fog_light`
  pass lights it once per frame, the composite applies height + bank layers in closed form per full-res pixel (exact
  depth edges), in front of clouds, stars and moons; runs with `--off clouds`. App: `SkyLook::fog` per weather +
  `ground_fog(hour)`, `--fog <x>`, `--off fog`; sea scaffold opaque; sky-view LUT 32 samples (F1 seam). Cost:
  `clouds_fog_light` 0.004 ms; bank march +0.05-0.17 ms in `clouds_trace`; composite +0.36 ms with per-pixel lighting,
  within noise after the fog-light texture (0.72 vs 0.73 ms) (release, 1440p, 4060 Laptop, shared machine: another
  `bevyrrowind.exe` on the GPU in every run, so the ≤ 0.1 ms gate is not proven; remeasure quiet). Open: FG-2 (owner: bank density/height, ash-storm visibility; banks hard to read under foggy haze 90),
  transparents unfogged until F3, banks by region wait for climates.
- [~] F3 Froxel fog + god rays (weather.md "Fog" steps 1-4): 160×90×64 froxels (High), density (height + banks +
  precipitation mist + local fog volume records), lighting (sun/moon × CSM × shadow-height map × cloud shadow map, sky
  ambient, clustered point lights capped per froxel), temporal reprojection, integration, apply on opaque, the
  `sf_atmos::fog` WESL function for transparents (water, glass, particles). Interiors keep `DistanceFog` (M9 volumes).
  Waits for F2. Verify: FG-3 Balmora 6:15 clear (shafts between buildings and from the western ridge), FG-4 torch-lit
  street in fog at night; budget 0.7 / 0.5 ms (High, 4060 / 3070); no temporal smear at 60 m/s (flight).
  Built (m6-f3 + m6-f3b, 2026-10-13): `sf_atmos::froxel` (High 160×90×64 to 384 m), inject (F2 density, key light ×
  CSM × shadow-height map × cloud shadow, sky fill, 32 nearest point lights, temporal) + integrate compute, composite
  with the analytic layers past the froxels, shared `fog_bank.wesl`, `FroxelVolume` + `FroxelFogMaterial` for
  transparents, `--off froxels`; Bevy `VolumetricFog` rejected again (weather.md). GPU 0.16 ms at 1440p (4060 Laptop).
  Open: FG-3/FG-4 (owner), app NIF/particle materials onto `FroxelFogMaterial`, flight smear check, 3070, local fog
  volumes and precipitation mist (wait for records / PR), F2's quiet `--off fog` gate.
- [x] W0 Water census (water-adapter.md "Census", docs + ignored real-install test): exterior water heights, interior
  `HAS_WATER` levels (visible vs hidden), boat placements (moored vs beached), authored water meshes and textures,
  sea-level water components, regions with water. Can run any time. Verify: counts in water-adapter.md and the
  session log (no vanilla data committed).
  Built (m6-w1, 2026-10-13): `cargo run --release -p mw_world --example water_census` (counts only, 1.1 s); results in
  water-adapter.md "Census results": exterior 1,559 cells, 0 water heights (one sea level confirmed); interiors 237 with
  water (216 visible, 21 below all geometry, levels −43..+218 m); 39 moored-boat candidates; 35 water-ish models, 9
  water textures; sea-level water 26.0 km² open sea + 149 enclosed pools (0.12 km²); Bitter Coast 1.63 km² of its water
  **connected to the open sea** (only 87 pools, 0.054 km², enclosed), so W3's swamp calm must come from openness, not
  per-component fetch.
- [x] W1 `sf_water` core (water.md "Data model", "The water query"): `WaterBody`/`WaterOptics` records, bodies from
  `Worldspace::sea_level`, body BVH + priority, `WaterQuery` (`sample`, `sample_many`, `immersion`), spectrum
  (JONSWAP/TMA/spreading, `h0` blend), CPU twin of cascade 0 (`rustfft`), displaced-height inversion, `WaterSource`/
  `WaterOpticsSource` hooks; `mw_water` skeleton (bodies, optics by region, ini `[Water]` mapping). Verify:
  `cargo test -p sf_water -p mw_water` (water.md "Verification" unit list: spectrum energy, twin vs DFT, inversion,
  origin-shift bit-identical heights, BVH priority); query ≤ 2 µs, twin ≤ 0.05 ms per step (bench).
  Built (m6-w1, 2026-10-13, uncommitted): `sf_water` (records, JONSWAP/TMA/Donelan-Banner spectrum, power-of-two cascades
  split at size/2, `h0` keyed by wave vector, CPU twin with 4 packed rustfft IFFTs + Catmull-Rom sampling, 3-step
  inversion, `WaterWorld`/BVH/priority query, `WaterSource`/`WaterOpticsSource` hooks; bevy-aqua code ported with its
  notice in `THIRD_PARTY_NOTICES.md`) + `mw_water` skeleton (bodies, optics by region, `[Water]` ini). 11 + 3 tests,
  clippy clean. Twin vs exact field 2.4 mm (Hs 1.9 m); release: twin step 0.027 ms, sample 0.25 µs. As built and the
  W2 handoff: water.md "As built (W1)".
- [~] W2 Ocean surface + water pass (water.md "Ocean surface", "Water pass"), replacing `SeaPlaneScaffold`
  (water-adapter.md): GPU FFT cascades + foam (ported from bevy-aqua), geometry clipmap + horizon skirt, colour + depth
  snapshot, water draw (refraction with validity, Beer-Lambert + scattering, GGX sun/moon + clustered lights, env-map
  reflection, slope-variance far roughness, aerial + froxel fog), velocity redraw, origin phase; wind from WD1 (or a
  constant until it lands); `--water scaffold|off`. Waits for W1. Verify: framework example `water_ocean`; WA-1 (calm →
  storm at Seyda Neen, A/B scaffold), WA-8 coast flight (no jumps at origin shifts); `water/*` GPU counters within the
  High column (FFT 0.15, draw 0.5, copies 0.1 ms); DLSS on: no wave smear. Then delete the scaffold.
  Status (m6-w2, 2026-10-13): built (`sf_water_bevy`, app `water.rs`, `--water on|scaffold|off`); golden GPU vs twin
  1.9 mm; FFT 0.22 ms (over 0.15: amortise), snapshot 0.04, draw ~0.5 ms (example, shared machine); WA-1, WA-8 awaiting
  owner; DLSS smear and in-app flight not yet checked. Handoff: water.md "As built (W2)".
  W2 leftovers (2026-10-18, m6-w3 + w3fin): FFT amortised + fused (0.139 ms, example, shared machine), quality tiers,
  slope mips, pipeline warm-up, height fog when froxels are off: done; shore damping: done in W3. Open: in-app 1440p
  counters, DLSS smear, WA-8 flight (no quiet machine), scaffold deletion after WA-1.
- [~] W3 Shore, bodies, interiors (water.md "Shoreline and depth colour", "Water bodies at any height"): `water.shore`
  bake (EDT, openness per connected component, land-only block mask), shore attenuation + shore/contact foam, wet band
  via the surface hook (or a stub until SU1), water-level clipmap, lakes/pools/interior `Bounds` pools from `WHGT`,
  optics by region (swamp, sea, coastal, ash, north, interior), body overlap report. Waits for W2. Verify:
  `cargo test` (EDT vs brute force, openness on a synthetic bay + inland lake); bake adds ≤ 10 s cold, 0 warm; WA-2
  swamp still and murky, WA-6 interior pool; sea under land not drawn (counter).
  Status (2026-10-18, w3fin): built and wired in the app: shore bake 0.5-0.6 s at start (worker thread), shore
  attenuation + foam, region optics map (optics retuned: no longer milky, hue by region, fainter shore foam), interior
  pools (`WaterSurface`) at `WHGT`, height fog input; shots `shots/w3/`; WA-2, WA-6 awaiting owner. Interior pool
  black fixed (w3pool: unwritten froxel volume applied in interiors, `sf_atmos_bevy` `FroxelVolume.params.w` now 0
  without an active `CloudCamera`). Owner 2026-10-18: WA-6 and region colour confirmed; **unfinished, parked by the
  owner**: shore foam is a uniform white band (Vivec waterfront), open-sea swell very small even at wind 14/20 m/s.
  Open: in-app GPU counters, sea-under-land counter, wet band
  (SU1), body overlap report, `water_bodies` example. Handoff: water.md / water-adapter.md "As built (W3)".
- [ ] W4 Reflections (water.md step 2, "Off-screen reflections"): SSPR compute for the dominant plane, camera-centred dynamic reflection probe (sky + terrain + far proxies, one face per 4 frames), content probes, Fresnel; reflection A/B
  `--off sspr`. Waits for W2. Verify: WA-4 Vivec canals at sunset (cantons reflected, no streaks at screen edges),
  WA-1 dock reflection, far shore reflected when looking down; SSPR 0.3 + probe 0.15 ms High.
- [ ] W5 Underwater (water.md "Underwater"): camera medium from the query, analytic underwater post (Low/Medium),
  waterline meniscus, Snell's window / total internal reflection on the back face, `water.caustics` bake + caustics in
  the surface hook (after SU1; terrain-only before), froxel water medium on High (after F3), `underwater` /
  `surface_break` cues. Waits for W3. Verify: WA-5 through the surface at Seyda Neen (meniscus, caustics on sand,
  shafts, colour near the ini's `UnderwaterColor`); underwater ≤ 0.1 ms (analytic) / +0.1 ms (froxel).
- [ ] W6 Buoyancy (water.md "Buoyancy"; needs M4 P4 dynamic items): `sf_physics` `add_force_at` + `Floating` wake rule,
  `water.pontoons` bake over `body.cook`, floaters system (pontoon caps, drag vs water velocity, optional hull
  clipping), ≤ 32 awake floaters, items float/sink by density, moored boats (soft joint to the authored pose; owner
  decision 3), demo rowboat `--spawn-floater`. Waits for W2 (W1 for headless tests). Verify: `cargo test -p sf_water
  -p sf_physics` (density 500 box at half draft ± 2 %, 2,000 sinks, rebase unchanged); framework example `water_float`;
  WA-7 at Seyda Neen (boat rocks in the storm, wood floats, iron sinks); floaters ≤ 0.1 ms CPU.
- [ ] W7 Rivers as baked flow (water.md "Rivers"): `water.hydro` (priority-flood + D8 at 16 m), `water.flow` (inflows,
  geodesic direction, discharge routing, flow map tiles), flow-advected normals and foam, flow in the query (floaters
  drift); Bevyrrowind `FlowOnly` (owner decision 4); framework `Generate` path in the `water_bodies` example. Waits for
  W3. Verify: `cargo test` (no sinks after flood, flow toward the outlet, still dead-end canal); `water.hydro` ≤ 2 s
  cold; WA-3 Odai at Balmora (flows to its mouth; a thrown crate drifts); Vivec canals still.
- [ ] W8 Wading and interaction (water.md "Wading, wetness and interaction hooks"): dynamic ripple sim (32 m, injected
  by feet, floaters, raindrops), `immersion` for actors, swim switch from `fSwimHeightScale` (controller `Swim`
  surface from the query), footstep/swim/splash cue mapping. Waits for W3. Verify: WA-2 walk into the swamp (rings
  around the legs, wake behind a moving rowboat); ripples ≤ 0.05 ms.
- [ ] W9 Authored water meshes (water-adapter.md): water mesh material on the census's water parts (Vivec waterfalls,
  plaza pools, Mournhold), `Pool` bodies from horizontal water submeshes. Waits for W0, W3. Verify: Vivec plaza pool
  (wade in, reflections); waterfalls keep their scroll; nif-sweep unchanged.
- [ ] SU1 `sf_surface` + `WorldMaterial` + occlusion camera (weather.md "Surface state", "Precipitation: occlusion"):
  surface-state clipmap (wet, puddle, snow, ash), exposure from the occlusion depth map (orthographic camera, amortized),
  update compute, `surface_state.wesl` moves to `sf_surface_bevy`, `MaterialMapper` → `WorldMaterial =
  ExtendedMaterial<StandardMaterial, SurfaceExt>` for cooked statics (bindless kept), wet porous response + puddles
  (Lagarde), permeability per terrain layer, coarse persistent grid in saves, wet footsteps query. Waits for WE3.
  Verify: `cargo test -p sf_surface` (accumulation/drying curves, grid seeding); SU-1 Seyda Neen after rain (dark wet
  wood, puddles on paths, dry under the docks' roofs); A/B `--off surface`; 0.3 / 0.2 ms High; draw calls unchanged.
- [~] PR1 `sf_precip_bevy` (weather.md "Precipitation") **volumes done in WE3 part 2b (we3b2)**; left: occlusion camera +
  map, fog on streaks, textures;: 3 camera-wrapped closed-form volumes, rain streaks, snow and ash
  flakes with wind flutter, occlusion from SU1's map, rate × local cloud cover, rain mist into the froxels. Waits for
  SU1, WD1. Verify: PR-1 Seyda Neen thunderstorm (no rain under roofs, streaks lean with the wind), PR-2 ash storm at
  Ghostgate (ash blows away from the summit); 0.4 / 0.3 ms High; check DLSS performance mode for smear (weather.md open
  question 9).
- [ ] PR2 Splashes, ripples, reactive mask: splash sprites at the occlusion depth (water level from the water-level
  clipmap on water), ripple atlas on puddles and water, lightning lights the rain; the DLSS bias-mask upstream patch
  + mask writes in precipitation/particles/water foam (weather.md open question 9), or a post-upscale precipitation
  pass if the patch stalls. Waits for PR1 (W2 for water ripples). Verify: PR-1 again at DLSS performance (streaks
  survive), splashes on the pier and on the sea.
- [ ] SU2 Snow and ash accumulation (weather.md "Surface state"): snow/ash channels on up-facing surfaces with height
  blend, terrain displacement through the deformation stack (or deferred to M7's layer if not built), ash blown off at
  high wind, far field wetness-only. Waits for SU1, PR1. Verify: SU-2 Red Mountain after an ash storm (grey dust on
  rocks and roofs, clean under overhangs), SU-3 Solstheim snowfall; budget within the surface row.
- [ ] C1 Character wetness (weather.md "Surface state: Characters", WISHLIST §5): per-character wetness uniform (rain
  exposure from the occlusion map at the head, immersion line from `WaterQuery::immersion`, drying per hour), character
  material darkening/smoothing below the line. Waits for SU1, W1. Verify: SU-4 an NPC walks through the swamp then
  stands in the rain (legs wet to the waterline, then all wet, drying after); cost ≤ 0.02 ms for 50 characters.
- [ ] WD2 Trees and impostors in the wind (weather.md "Bake-time wind data"): `wind.paint` bake (trunk/branch/leaf
  levels, pivots, stiffness from connected components; adapter `wind_profile`: `flora_*`, parasols stiff, kelp flutter),
  two optional vertex streams in `sf_model_bevy` (dropped from the RT LOD), hierarchical sway in forward/prepass/shadow/
  motion vectors, impostor per-instance bend (atlas header stiffness). Waits for WD1. Verify: `cargo test` (pivots on a
  synthetic tree, deterministic product); WD-1 Ascadian Isles in a thunderstorm (trunks bend, leaves flutter, impostors
  agree at the handoff); bake ≤ +10 s cold; 0.15 ms High.
- [ ] WD3 Other wind consumers: NIF particle systems (smoke, ash, embers) drift with the field, cloth (M4 P10) gets
  `velocity_at`, fog banks and cloud advection read `sf_wind`, wind cues. Waits for WD1. Verify: WD-2 Balmora chimney
  smoke and a robed walker in a storm leaning with the same wind; no extra GPU cost beyond the wind row.
- [ ] G0 HD grass spec (`../superfargoth/docs/specs/grass.md`, docs only; ARCHITECTURE §7.1, research first: Tsushima
  procedural grass, UE/Unity grass, current Bevy crates such as `bevy_feronia`): placement masks from LTEX rules, GPU
  scatter, near blades / mid clumps / far colour, wind, interaction trail, water mask, budgets, adapter rules. Verify:
  spec with an "Industry comparison" section; ROADMAP G1/G2 refined.
- [ ] G1 Grass near field (per G0): GPU compute scatter per terrain tile, Bézier blades with clumping, view thickening,
  `sf_wind` bend in every pass, no grass under water (water-level clipmap) or on roads, adapter LTEX → species. Waits
  for G0, WD1. Verify: GR-1 Ascadian Isles meadow at noon and in wind; grass budget from G0 (target ≤ 1 ms High);
  flight: no popping ring.
- [ ] G2 Grass mid and far + interaction: instanced clumps/cards to the far colour, terrain albedo carry, trail map bend
  from actors and physics bodies, flatten/burn mask. Waits for G1. Verify: GR-2 walk through grass (bent trail
  recovering), GR-1 from a ridge (no visible band).
- [ ] T2 Far shadows tier 2 completion (weather.md "Distant shadows"): far-field proxies cast into the shadow-height map
  (top-down caster-height raster: cantons, strongholds, Ghostfence), per-preset cut (map-only terrain on Low/Medium),
  the split profiled (terrain march vs fetches vs proxies) so vistas meet 0.15 + 0.08 ms High; the ocean's horizon
  skirt reads the map (water.md open question 4). Verify: SH-* re-run, Vivec at sunset (canton shadows on the water
  past 500 m), vista budget in perf.md.
- [ ] E1 Milestone demo + gates: the four owner scenes below as `scenes.ron` entries and `benchmarks/*.json` cameras,
  `perf suite` coast/balmora/red_mountain/flight per preset with each weather at 1440p, all FG/WE/WD/WA/SU/PR/GR checks
  listed. Verify (milestone): the §9 sentence, budgets per weather.md and water.md tables (High ≈ 3.3 + 1.35 ms on the
  3070 with DLSS), owner sign-off on WX-1..WX-4.

Owner test scenes (added to `docs/visual-checks.md` by E1, status "awaiting owner"; the task checks above come with the
tasks that build them):

- WX-1 Bitter Coast rainstorm: Seyda Neen docks, 14:00, `--weather thunder`: structured overcast, rain streaks
  occluded under the docks' roofs, puddles on the paths, dark wet wood, splashes and ripples on the sea, a lightning
  frame; walk into the shallows (ripples, wet legs after); the demo rowboat rocking and drifting; moored boats bobbing.
- WX-2 Ash storm at Red Mountain: near Ghostgate looking up the mountain, 16:00, `--weather ash`: low visibility,
  brown haze (dust term), ash streaks blowing away from the summit, ash on up-facing rocks, the Ghostfence visible.
- WX-3 Sunrise god rays in Balmora: 6:15, clear/cloudy: shafts between buildings (froxels + CSM) and from the western
  ridge (shadow-height map), red-lit cloud undersides, the Odai flowing.
- WX-4 Bitter Coast morning fog: 6:30, `--weather foggy`: ground fog banks in the swamp drifting with the wind, trees
  fading into it, still murky water, fog continuing past the froxel range from a ridge view.

**Owner decisions for M6** (recommendations; details in ARCHITECTURE §10 2026-10-13 m6-spec, water.md, water-adapter.md):
1. Water: our own `sf_water` porting bevy-aqua's wave/foam/optics code (MIT/Apache, notices kept), not bevy-aqua as a
   dependency or fork; revisit if it ships on Bevy 0.20 with origin-relative coordinates and a CPU query. Recommend: yes.
2. Water reflections: screen-space planar reflections (SSPR) + probes + environment map on raster; no planar mirror
   cameras (a second render of the streamed world); Bevy SSR only if weather.md open question 4 resolves; RT
   reflections from the water pass later. Recommend: yes.
3. Moored boats: placed rowboats, longboats and gondolas in water bob on a soft mooring; ships stay static; the demo's
   free rowboat is spawned by the scene. Recommend: yes.
4. Rivers for Bevyrrowind: flow only on existing sea-level water (the Odai flows, canals stay still), no generated
   rivers above sea level (the framework can; a mod could opt in). Recommend: yes.
5. Budget: water adds ≈ 1.35 ms (High, 1440p) to weather's ≈ 3.3 ms; High targets the 3070 with DLSS, features cut per
   preset when over. Recommend: accept.
6. Swamp look: Bitter Coast water murky (~3 m visibility), no waves or foam, by region optics. Recommend: yes, tune on WA-2.
7. Skip weather.md N3 (Bevy `VolumetricFog` god-ray prototype): go straight to froxels (F3). Recommend: skip.

## M7–M10

Not broken down yet. Scope per milestone: ARCHITECTURE.md §9; designs:
`../superfargoth/docs/WISHLIST.md`. Break a milestone down here (with a
spec per major system) before starting it. (M6 is broken down above, 2026-10-13.)

**Renumbered 2026-10-13 (owner):** M7 is now the RPG core (dialogue, journal, stats/rules, magic, AI, saves; was
M10+); far-field performance → M8, seamless interiors (portals) → M9, combat → M10. Text below written before the
renumbering keeps the old numbers: "M8 portals" here means M9. **Priority: M4, M5, M6** (owner), then M7.

The M6 weather stub order that stood here moved into "## M6" (F, WD, WE, SU, PR, T tasks); weather.md near-term fixes
A1-A3 and B1-B3 are done, N1/N2 are M6 F1, B5 stays in the Backlog.
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

## Co-op (N0-N8)

2-4 player co-op: one player hosts, host-authoritative with prediction of your own character only; shared world,
own characters (owner 2026-10-19). Research R12 `../superfargoth/docs/research/coop/README.md`; spec
`../superfargoth/docs/specs/net.md` (rules, ownership, replication, context player, saves, quest model, audit).
**Timing (owner): N0 now; N1-N5 after M6; N6-N8 after M7.** Transport: direct IP/LAN first, Steam later (opt-in).
Each slice 1-2 steps, verified and committed before the next.

- [x] N0 Net spec + audit (docs only): `specs/net.md` (8 net-ready rules binding for every spec, ownership per
  system, `PlayerId` + context player, multi-region host, saves, quest model, N1 crate criteria); audit table of all
  specs (spec | violation | required change | slice); physics.md "Controller": mover on the fixed tick with
  interpolation; specs README index + rule; ARCHITECTURE §10.
- [ ] N1 Crate spike (after M6): two processes, capsules on `sf_level` courses, lightyear vs replicon + own
  one-entity rollback at 100 ms / 1 % loss; clean build time, bandwidth, feel. Done: crate picked, §10 entry.
- [ ] N2 `sf_net` thin plugin (after M6): host/join over UDP (direct IP, LAN discovery), content fingerprint
  handshake, `PlayerId`, remote players interpolated. Done: 2 players walk Balmora together.
- [ ] N3 Predicted locomotion, jump and swim (after M6): reconciliation, net debug overlay. Done: no visible
  correction at 100 ms RTT.
- [ ] N4 Predicted actions (after M6): attacks, dodge, traversal with warps; host hit authority, lag compensation,
  host-allocated spawn ids. Done: two players fight one NPC; mantles stay correct.
- [ ] N5 World (after M6): several R0 regions and physics regions on the host, interest management, overlay per
  cell, props, carry/throw, ragdolls. Done: players 5 km apart, host frame within budget.
- [ ] N6 Scripts and RPG (after M7): host-only MWScript/Lua with the context player, routed MessageBox and dialogue,
  container/barter requests, per-player menus. Done: a scripted main-quest chain completes in co-op.
- [ ] N7 Saves (after M7): host world + per-player character, join in progress, reconnect, clock, rest vote. Done:
  save, quit, resume with a guest.
- [ ] N8 Steam transport (after M7): lobby, relay, invites, opt-in game feature. Done: invite a friend through Steam.

## Session log

Newest first. One or two lines each: what changed, what was verified.

- 2026-10-13 (m6-spec, docs only): M6 broken down ("## M6: Water, weather, surfaces": F1-F3, WD1-WD3, WE1-WE3, W0-W9, SU1-SU2, PR1-PR2, C1, G0-G2, T2, E1; WX-1..4; 7 owner decisions); new specs water.md (superfargoth) and water-adapter.md; ARCHITECTURE §10 entry; "M6–M10" → "M7–M10". bevy-aqua 0.1.3 evaluated (Bevy 0.19, no 0.20 branch, no floating origin, GPU queries, planar cameras): ported, not adopted. No builds.

- 2026-10-13 (m5-w1, M5 W1): `mw_script` MWScript front end (lenient lexer, line parser, 452-function table, AST with state-location `VarRef`s, resolver). Real install: 1,231 script texts + 17,144 dialogue results parse with 0 unrecoverable; 20 rules fire (commas 2,307, `End name` 591, `=` as `==` 79, stray `EndIf` 30, extra-args 13, `ref->( call )` 33, ...), `Plague_Gedna`'s `GetStat` reads 0 instead of going inert; parse+resolve of everything ~0.1 s. 11 tests, clippy clean. Handoff for W2 in scripting-adapter.md "As built (W1)".
- 2026-10-13 (m5-spec, docs only): M5 broken down ("## M5: Mods, Lua and UI", A1-A4, L1-L3, U1-U7, W1-W3, E1, X1-X2, MS-1..MS-6, 7 owner decisions); new specs scripting.md, mods.md (superfargoth) and scripting-adapter.md; ARCHITECTURE §10 entry. Runtime Luau via mlua 0.12.2 (state per mod), MWScript → Luau at bake time; corpus measured from the install (1,231 script texts / 981 KB / 49k lines, 17,144 dialogue results). No builds.
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
- 2026-10-13 (walkers4, partial): contact cook v4 ends stances at heel-off (`ContactRules::lift`, trimmed in the stance's 2nd half; release 2.0, drift clamp dropped): slope_walk passes 15-30° + stairs (slide <= 0.005 m/s), census walk 33 / strafe 43 / run 23 / run-strafe 35 / sneak 52 % planted, 80 low clips (57 without heel-off, 243 before v4); bake 49 s. Mover steps over short steep bevels onto walkable ground within step_up (UE StepUp); scaffold remembers steep/wall-blocked PGRD edges and reroutes (`PathGraph::path_avoiding`, `PathWalkScaffold::replan_around`). In game (night Balmora 150 s): 145 give-ups (119 steep, 24 wall, 2 other), 0 rerouted: give-ups are first legs from spawn into >0.45 m steep ground (no grid edge to block); 2 torch-bearers still freeze their walk lane at t=1.333 s (stride-stuck 76). Open: those two, first-leg give-ups, Dagoth Ur hand effect (not started). Not committed.
- 2026-10-13 (physics-p2): M4 P2 built: `sf_physics_bevy` (fixed step, rebase, write-back, R0 statics + terrain heightfields under a 0.5 ms insert budget, counters, collider gizmos), `StaticShape::trimesh()`, app `physics.rs` (`--physics-debug`/`G`, `--off physics`); headless test + Balmora/Census debug shots line up (PH-6 awaiting owner); flight: idle step ≤ 0.13 ms, rebase step ≤ 1.45 ms; A/B stutters and walkers on `PhysicsProbe` open.
- 2026-10-13 (lighting): night exposure floor (owner option A): `sf_sky::ExposureRange` (UE Min/Max EV100) clamps the scheduled EV100 and is baked into the viewer auto-exposure curve; Bevyrrowind floor EV100 3.5 + night ambient 1 nit (`LightCalibration`, `--exposure-floor`/`--night-ambient`): `hi-balmora-night` EV100 1.62 -> 3.5, blown pixels 1.63 % -> 0.26 %, mean luma 72 -> 38; the cave (Addamasartus) unaffected (EV 7 scheduled, auto meters above the floor). Budgeted point-light shadows `sf_sky_bevy::PointShadowBudget` (N nearest by distance/priority, hysteresis, 15-25 m + 0.3 s fade via a shadowed/unshadowed light split, torches 2x, 0.25 m near plane, light props `NotShadowCaster`): 1440p median 9.4 / 10.2 / 13.3 / 13.8 ms for N = 0/1/2/4 (render-thread bound) -> High 1, Ultra 4, Low/Medium 0 (`--point-shadows`). LT-1..3 awaiting owner; shots/lighting.
- 2026-10-13 (m5-a2): M5 A2 built (uncommitted): native record files in `sf_records` (kind registry with serde + fingerprints, RON/TOML `define`/`patch`/`delete`, generic `set`/`merge`/`remove`/`append` on a serde value tree whose RON leaves stay raw until typed, field-level touches, conflict vs merge) and `sf_mods::NativeRecords` + per-mod `RecordConflict`/`RecordMerge` issues; `sf_records` gains ron/serde/toml/blake3/postcard deps (lockfiles). sf_records 20, sf_mods 19 tests, clippy clean; mw_records clippy clean, 10 tests pass. Real-data patch check left to A3 (TES3 types need serde). Spec: mods.md "As built (A2)", ARCHITECTURE §10.
- 2026-10-13 (walkers5): frozen walk strides (owner "wonky guards at night"): not torch-specific; the walk blend space marker-synced a lane to an outro marker past its loop end, and once that lane led (after a turn) it played to the clip end (1.333 s) and parked. `sf_anim_graph` sync now syncs looping clips over their loop only (pairs wrap at the loop end, a follower always lands inside); tests `synced_followers_stay_inside_their_loop`, `mw_graphs` `turning_walkers_keep_every_walk_lane_cycling`. Night Balmora 150 s: stride-stuck 76 -> 0 (3 runs). Give-ups were not pathgrid first legs but straight off-grid walks (no node in reach): `WanderPlanner` remembers given-up headings/first-leg nodes per spot (`blocked_starts`, 1 m) and re-rolls away (±45°) or starts at the next-nearest node; give-ups 106 (before 123-167). Dagoth Ur: matches the NIF (texture-coloured additive particles, lavender-blue spray, no light node; nothing to wire). HI-4, WK-1, PA-3 awaiting owner. Not committed.
- 2026-10-13 (physics-p3): M4 P3 built (uncommitted): `body.cook` (primitive/hull/V-HACD, 1,055 item models, 14 failed) and `items.check` (per-page settle test; 16k of 42.6k items flagged unsupported: too many, P4 investigates), both in `bevyrrowind bake` (cold +29 s, warm 2.75 s total); fixed `refresh_queries` dropping new dynamic bodies; P2 A/B: physics on not worse for stutters, insert peaks still 0.49–0.81 ms (gate 0.5).
- 2026-10-13 (physics-p4): M4 P4 in progress (uncommitted, compiles, tests pass, not run in the app): items.check sequential settle (overlaps 51 % -> 26 %, unsupported 38 % -> 36 %; thin items fall through two-sided static trimeshes, so physics statics are now oriented + internal-edge-fixed, census pending), items as sleeping dynamic bodies with holds/mass (new `mw_physics`), item pivots, overlay persistence, two-phase inserts, rolling resistance, `V` ball; PH-1/PH-5 awaiting owner; handoff in ROADMAP M4 P4 "Open".
- 2026-10-13 (nightsky): sky S2 + S3 built (uncommitted): `sf_sky` night model (`Moon` N moons with own phase cycle, steps/textures, `MoonPath::Synodic|Circuit`; `StarField`; `SunDiskLook` drawn size + glare; one moonlight summing the moons; 4 tests), `sf_sky_bevy` night pass (fullscreen at the far plane, depth-tested to sky pixels, additive after `render_sky` and before the cloud composite), app `night_sky.rs` (Masser/Secunda from `[Moons]` + phase textures, sun 2.5°) and `--day`/`--sun-size`; red night streaks fixed (cloud key light by light at the camera + planet shadow in `cloud_trace.wesl`); night pass 0.26 ms night (shared machine) / 0.08 ms noon at 1440p; scenes ns-*, NS-1..4 awaiting owner.
- 2026-10-13 (m5-l1): M5 L1 built (uncommitted): new Bevy-free `sf_script` (mlua 0.12.2 + Luau 0.740; workspace adds `mlua` dep and the crate), 16 tests pass in `send` and `NonSend`, clippy pedantic clean; spike (`cargo bench -p sf_script`, idle machine) confirms decision 1: Rust->Luau 66 ns raw / 148 ns guarded, Luau->Rust 38 ns, 50 contexts start in 15 ms, 394 KiB per empty context (budget raised to 512 KiB), save/restore ~30 ms/MiB (budget 5 -> 40), watchdog costs ~25 ns per safepoint (mlua trampoline); `send` kept, app-data world guard, `luau-jit` off. Clean-build delta: `mlua-sys` Luau C++ 66 s, new deps 82 s wall at -j 8 (jit +80 s).
- 2026-10-13 (m5-a1): M5 A1 built (uncommitted): `sf_mods` repository (adapters + mods dirs, roots indexed once), profiles (`profiles/<name>.toml`: enabled, settings, limits), manifest `[optional]`/`[conflicts]`/`[ui]`, `ContentAdapter`, `load_profile` (steps 2-4; a broken mod drops with its dependents instead of failing the profile), `ModReport` (all step 1-4 issue types, MO2-style per-mod overrides), `ScriptPlan` for `sf_script::ContextConfig`; `sf_mods` now depends on `sf_vfs` + `sf_records` (lockfiles updated). 18 tests, clippy clean; mw_records still builds and passes. Spec: content paths are VFS paths inside the mod's roots, mount returns layers, limits profile-only (mods.md "As built (A1)", ARCHITECTURE §10).
- 2026-10-13 (m6-f1): M6 F1 haze + dust built (uncommitted, [~]): `sf_sky` `Weather::dust`/`dust_tint`, `WeatherScale`, `MediumScales`, absorbing grey-brown dust term (2 km, SSA 0.87/0.81/0.71); `sf_sky_bevy` rate-limited medium rebuild (step + 0.25 s, `MediumStats`, ~2.8 ms CPU per rebuild, GPU unchanged); app `--haze`/`--dust`, per-weather haze/dust table following `--clouds` + morning curve; haze recalibrated for 1-5 km views (clear 20, not the spec's 2-3); scenes fg-*, check FG-1 awaiting owner; shots/f1. Known: noon sea-horizon seam with dust (F2).
- 2026-10-13 (m6-f2): M6 F2 height fog + ground-fog banks + far fog built (uncommitted, [~]): `sf_atmos::fog` (UE exponential layers, Megalophobia bank mask on the weather map G channel, drift, CPU twin + 6 tests); cloud node: bank march → per-pixel modulation in the trace, 66×1 `clouds_fog_light` pass, closed-form fog per full-res pixel in the composite in front of clouds/stars/moons, runs with `--off clouds`; app `SkyLook::fog` table + `ground_fog(hour)`, `--fog`, `--off fog`, opaque sea scaffold, sky-view LUT 32 samples; clouds example `--fog`; scenes fg-bittercoast-ridge, fg-balmora-foggy-noon, fg-night-fog, check FG-2 awaiting owner; shots/f2. Cost noisy (shared GPU), see F2.
- 2026-10-13 (physics-p4b): M4 P4 in the app: items.check support meshes for statics whose box proxy encloses their clutter (`support.cook`) + fall = center and lowest point drop: Unsupported 38 % -> 8.2 % (wall racks mostly); Tradehouse 0 awake in 10 s; fixed insert starvation and items falling through before their statics loaded; flight bench, kill height, impulse release left for P5.
- 2026-10-13 (physics-p5): M4 P4 leftovers (insert time-slicing: Tradehouse peak 0.80 -> 0.57 ms, prior added after; automatic kill height; 4 Hz coverage freeze; `BodyKind::Held` released by impulse) + P5 built (uncommitted, [~]): kinematic character capsules, capped pushes (`min(m_body·Δv_n, 400 N·dt)`, spec fixed), `ImpactOnCharacter`, slope sliding, walkers' mover on `impl GroundProbe for Physics`; 4 + 1 + 1 new tests, pedantic clippy clean; wk-balmora walker stats comparable, mover stage 2x the StaticWorld cost (584 vs 319 µs): profile first in P6.
- 2026-10-13 (m5-w2b, resumed m5-w2 after an API cut-off): M5 W2 built (uncommitted): MWScript → Luau emitter (`mw_script::emit`, no Luau globals, inert → stub module), `functions.txt` live/stub/later status, coverage report + `oracle_flags`, `luau/mw.luau` runtime skeleton, `mwscript.translate` + `mwscript.coverage` bake steps in `bevyrrowind bake`. Real install: 1,207 scripts + 17,144 results translate, compile and run once in a sandboxed `sf_script` context with 0 inert / 0 compile / 0 run errors; bake 1.4 s CPU (5.6 s wall cold, 0.19 s no-op). Coverage (live DB, call sites): 25,449, live 14,539 (57.1 %), stub 5,320, later 5,590; by family (corpus test, all files) journal 6,105 live, dialogue 3,546 later, inventory 3,490 (3,354 live), placement 3,122 (3,106 live), disposition 2,211 stub, ai 1,557 (896 stub / 661 later), faction 807 (91/716), combat 748 (180/568), magic 652 (281/371), ui 596 (540 live), map 543 stub, audio 499 stub, stats 436 (420 stub), activation 426 / scripts 394 / menu 297 / deaths 247 / clock 238 live, events 258 stub. Oracle-pending: 0 integer divisions, 44 short/float comparisons. Fixed 3 pedantic clippy warnings in mw_bake script.rs. W3 handoff in scripting-adapter.md "As built (W2)".
- 2026-10-13 (physics-p6): M4 P5 follow-ups + P6 (uncommitted): movers' terrain back on the analytic `TerrainProbe` composed with physics `STATIC|PROP` (probe_bench: heightfield casts 13 us; wk-balmora mover median 584 -> 424 us vs 272 off, shared machine); Tradehouse insert peak 0.60 ms, flight insert 0.45-0.55 ms, idle step 0.45-0.56 ms (gates 0.5/0.3 still missed); P6 carry/drag/throw: `sf_physics::carry` (hand + motor joint, holder-space interpolation, per-pair holder exclusion, drag force cap from drag mass), `sf_physics_bevy::carry` (Holder, messages, HeldPoses), `mw_physics::holder_limits` (Strength), app LMB/V/[ ] driver + `BEVYRROWIND_FPS_CAP`; 5 carry tests (view-space jitter <= 0.044 mm at 30/60/144 fps, throw cap, ballistic landing, thresholds, break, holder exclusion); PH-4 awaiting owner; P7 handoff in M4 P6.
- 2026-10-13 (physics-p7): M4 P7 [~] (uncommitted): new `sf_ragdoll` (`ragdoll.cook` from rig profiles, limp `RagdollInstance`), death handoff in `sf_physics_bevy` feature `ragdoll` + `sf_anim_bevy::PhysicsPose`, corpse drag by limb (whole-ragdoll mode mass); impulse joints + 8 iterations + 4:1 mass cap: drop 0.49 cm, stairs 0.32, 75 kg wrist drag 0.07 (gate 1 cm); census 93 skeletons, 0 failed (creature masses implausible without skin extents); app `I` kills; PH-2 awaiting owner.
- 2026-10-13 (physics-p8): M4 P8 [~] (uncommitted): powered ragdoll core (`sf_ragdoll::powered`: stable-PD joint targets with composite-inertia gains, world pelvis anchor, hits + dips, balance, get-up root, `ROLL_TO_BACK` scaffold), `Layer::POWERED`, graph physics stage + `inertialize_from` hand-back, per-joint `PhysicsPose` weights, `mw_physics::creature_mass`; joints 3.2° RMS (met), effectors 11.5 cm (missed), hand-back continuous (met), 0.128 ms per powered character (missed); app wiring, PH-2/PH-9 runs are P8b.
- 2026-10-13 (physics-p8b): M4 P8b [~] (uncommitted, tool cap): critically damped + gravity-compensated pelvis drive (effectors 11.5 → 6.8 cm air / 11.0 cm floor, gate open), powered-iteration knob (0.129 → 0.090 ms/char at +4; parallel no gain), `sf_physics_bevy` Death crumple / Knockdown + get-up modes, budget 8, impact direction, app knockdown wiring, census 93/0 failed; app run + PH-2/PH-9 screenshots handed to P9.
- 2026-10-13 (physics-p9): M4 P9 [~] (uncommitted, stopped on usage limit before the app run): `sf_anim_graph::secondary` `HitReact` (closed-form critical rotation springs, `GraphInstance::hit`) + `SpringBones` (Verlet chains, fixed 1/120 s substep, frequency/damping, inertia), 3 tests (frame-rate independent, no drift); mw_graphs humanoid/creature carry both; `clip_root_body` get-up pose replaces the `LYING_REST_GET_UP` scaffold; app hit reactions under 15 N·s, mover yaw after get-up, `BEVYRROWIND_AUTO` dev hook (app physics.rs not yet compiled: blocked by other sessions); P8b numbers in physics.md; PH-10 added. Handoff: `../.sessions/physics-p9.md`.
- 2026-10-13 (m5-a3): M5 A3 built (uncommitted): `mw_mods` (Morrowind `ContentAdapter`, builtin `mw` mod pinned lowest with graphs + `mw` Luau runtime, `morrowind-data`, generated default profile, `<user data>/{mods,profiles}`), TES3 kinds serde-registered, bake record fingerprints via `RecordDb::fingerprint`, profile fingerprint beside the manifest (directory-listing stats: 375 → 4 ms), `ShadowedModern`, app loads through profiles + `mods list|order|report|enable|disable`; real install 62,025 records, load 0.29 s, RON radius patch visible in `mods report`, texture mod wins `which` and re-bakes nothing; no-op bake 3.0-3.2 s (gate 2.5 s missed by non-A3 steps).
- 2026-10-13 (m6-f3b): M6 F3 [~] (resumed m6-f3, uncommitted): froxel fog + god rays in `sf_atmos_bevy` (inject/integrate compute, CSM + shadow-height + cloud shadow, 32 point lights, temporal; analytic layers past 384 m), shared `fog_bank.wesl`, `FroxelVolume`/`FroxelFogMaterial` for transparents, `--off froxels`, clear/fair dawn haze; Bevy `VolumetricFog` rejected (weather.md); GPU 0.16 ms at 1440p; FG-3/FG-4 awaiting owner (shots/f3/).
- 2026-10-13 (m5-l2): M5 L2 built (uncommitted): new `sf_script_bevy` (Bevy-thin: app/ecs/time/sf_diag_bevy) with `Scripts` resource, Input/Update/Ui exclusive slots, `Activated` → `ActivationOutcome { handled }`, `ScriptFact`, diagnostics messages + `script/...` counters; world access decided as declared slot resources moved into one shared cell (no per-context guard cost); `ScriptPlacements` + `@sf/world` subset; 4 headless tests + clippy clean. App `scripts.rs`: profile contexts + `mw.legacy`, R0 handle hook, crosshair pick (scaffold) on Space / `--activate N`; a scratch-folder Lua mod logged a real activation in the Census office and its timer fired.
- 2026-10-13 (m6-w1): M6 W0 + W1 built (uncommitted): water census example (`mw_world` `water_census`: one exterior sea level, 216 visible / 21 hidden interior pools, 39 moored-boat candidates, Bitter Coast water mostly connected to the sea, so swamp calm comes from openness), new `sf_water` (records, JONSWAP/TMA/Donelan-Banner spectrum, power-of-two cascades, `h0` keyed by wave vector, rustfft CPU twin with cubic sampling, BVH/priority `WaterQuery`; twin 2.4 mm from exact, 0.027 ms/step, 0.25 µs/sample) and `mw_water` skeleton; bevy-aqua notice in `THIRD_PARTY_NOTICES.md`; W2 handoff in water.md "As built (W1)".
- 2026-10-13 m6-w2: M6 W2 ocean + water pass built (sf_water_bevy: GPU FFT 4×256², clipmap + skirt, snapshot water draw, aerial + froxel fog, velocity redraw; golden 1.9 mm); WA-1/WA-8 awaiting owner; `SeaPlaneScaffold` behind `--water scaffold`.
- 2026-10-13 (m5-a4): M5 A4 hot reload built (uncommitted): `sf_vfs` `SharedVfs` (arc-swap) + `Vfs::rebuild` of dirty dir layers with winning-change diff + `watch::VfsWatcher` (notify-debouncer-full 0.7, Bevy's crate) + Bevy `shared_asset_source`; `sf_mods` `route`/`reload_records`; `sf_script_bevy` `reload_changed`; app `hot_reload.rs` (dev only). 15+20+5 tests, clippy clean; real run: Lua edit reloads with state kept, texture edit reloads in the Census office, each under 0.3 s. Dev re-bake during play not built.
- 2026-10-13 (m5-l3): M5 L3 [~] (uncommitted): script API v0.1 in sf_script/sf_script_bevy (core `on_load`/settings/game_time, new `@sf/interfaces`, `@sf/world` spawn/despawn queue + nearby + entity scripts with snapshot state, handle methods), golden `docs/api/sf.d.luau`, example mod `examples/mods/api_tour` headless, mlua `serialize` dropped; 22 + 8 tests, clippy clean. App LiveSpawn scaffold + doors on `ActivationOutcome` written but not compiled (app build blocked by m6-w3 WIP); real run pending.
- 2026-10-13 (m6-w3, stopped early at owner usage limit, uncommitted): W2 leftovers part 1 + W3 groundwork: fused 2-dispatch FFT with runtime grid (128/256), `WaterQuality` tiers, High alternates the 256/64 m cascades (example 1600×900, shared machine: `water_fft` 0.22 → 0.139 ms + `water_mips` 0.023 ms; golden 1.0 mm), slopes mips, pipeline warm-up at startup, analytic height fog path, `sf_water::shore` (FH EDT + openness by geodesic distance, CPU query attenuation; 3 tests), `WaterSurface` (lakes/pools/interiors), region optics map in the shader, `mw_water` `wet_mask`/`optics_map` + less milky optics (2 tests). App wiring, screenshots, 1440p and DLSS checks not done; handoff in `../.sessions/m6-w3.md`.
- 2026-10-18 (l3fin): M5 L3 done for v0.1: real run in the Census office (dev exe, scratch `l3_test` mod, `--activate 4`): `on_load` with player position, entity script `counter` attach/state/tick, two LiveSpawn copies seen in the room, temp copy despawned, "handled by a script" from the 3rd activation; Space on the office door still travels (unhandled `ActivationOutcome`). No fixes. `@sf/records`/`input`/`anim`/`physics` and MS-2 parked (owner, Lua parked).
- 2026-10-18 (w3fin): M6 W3 [~] wired in the app: shore field baked at start (0.5 s, worker thread) for wave damping + shore foam, region optics map with retuned optics (backscatter-sized body colour: not milky; swamp olive, lagoons teal, sea blue), interior pools at WHGT, height fog input, `--quality` -> water tier; scene wa-interior-pool; WA-2/WA-6 awaiting owner (interior pool draws black: open); in-app GPU measurements open (no quiet machine).
- 2026-10-18 (p9fin): M4 P9 [~] app-run in `scene wk-balmora` (release, `BEVYRROWIND_AUTO`): kill, 12 m/s knockdown (36 N·s) + get-up from the `Knockout@tail` clip (no scaffold), 2 m/s hit react (5.9 N·s); fixes: `mark_settled` always runs (the auto hook never fired), auto throws frame first, release slow balls close and lead walkers; PH-2/PH-9/PH-10 agent-run, awaiting owner; open: stretch peaks 1.4–5.6 cm, one intermittent despawned-entity panic, P8b sleep/effector/cost items.
- 2026-10-18 (w3pool): WA-6 interior pool black fixed: not the snapshot but the froxel fog (interiors drop `CloudCamera`, the unwritten `FroxelVolume` was still flagged live, transmittance 0); `sf_atmos_bevy` `update_volume` sets `params.w` = 1 only with an active cloud camera. Shots `shots/w3pool/`; WA-6 awaiting owner.
- 2026-10-18 (fallfix): NPCs falling through floors fixed (M4 P5): collision-ready gate for movers (wait for queued statics + resident terrain heights), fell-out safety net for characters, swimmers float until Swim; tests in sf_physics_bevy; verified over BRP (Census office + Seyda Neen).
- 2026-10-18 (p9soft): PH-10 soft-hit bug (M4 P9): impacts are the incoming body's momentum at first contact (`PhysicsEvent::Hit`, pre-solve `CollisionEvent::Started`, `sf_physics::impact_impulse`), not solver contact force (walking into a resting ball: 423 N·s before, no hit now); character pushes no longer bat away incoming bodies; threshold 1 N·s; 4 sf_physics tests. Re-run: soft hit 6.4 N·s, 12 m/s knockdown 35 N·s; open: first auto throw fell short (aim), ragdoll-on-neighbour hit-react spam, no strength tuning. PH-10 awaiting owner.
- 2026-10-18 (orchestrator): BRP live inspection (`remote.rs`, port 15712, `tools/run_dev.ps1`, bevy_brp MCP registered); rules: check builds from all projects, reuse whole projects, finish before starting, Lua parked (Mudcrab); crate "update all" waits for hanabi; owner batch: CH-4 staff stand, WA-6 + WA-2 region colour confirmed, WA-2 shore foam/swell unfinished (parked); shadows pass + P11 general traversal (prior art in animation.md) in backlog, awaiting owner go.
- 2026-10-18 (p10a): M4 P10 [~] solver core `sf_anim::cloth` (Bevy-free XPBD bone lattice: geodesic tethers, max distance, swept capsules, character space with fictitious-force inertia, teleport keep-shape); tests: hem stretch 1.3–1.9 % at 30/60/144 fps (run, 180° turn, stop), no capsule penetration, teleport stable; 0.014 ms per robe (release). Next p10b cook + census + ClothRules.
- 2026-10-18 (u1a): M5 U1 part 1 [~]: input crate `bevy_enhanced_input` 0.27 over leafwing 0.22 (prioritised, input-consuming contexts = CommonUI per-screen input; ui.md OQ4 closed, ARCHITECTURE §10); new Bevy-free `sf_ui` (UI record kinds, `UiRegistry` with owners/conflicts, `ScreenStack` router rule, keyed `ListModel`, `SettingsStore`, Fluent `Localizer`), 14 tests, clippy clean. Next: u1b `sf_ui_bevy` + gamepad example.
- 2026-10-18 (wd1a): M6 WD1 [~] part 1: Bevy-free `sf_wind` (Wind moved from `sf_sky`, re-exported; WindState/WindSource x32/WindFrame; Megalophobia gust field with a PCG lattice hash and split origin phase; log profile; `sf_wind.wesl`); CPU/WGSL twin run headless on the GPU (1.1e-5 rel), never-upwind and origin-shift tests; next wd1b (`sf_wind_bevy`, `wind` example, clouds).
- 2026-10-18 (wd1b): M6 WD1 [x] part 2: `sf_wind_bevy` (WindPlugin/WindField/WindOrigin; one fixed ShaderBuffer with current + previous frame for motion vectors, materials bind it with `#[storage]`; WESL `sf_wind::field` + `sf_wind_bevy::wind` with a def-chosen slot); upload 0.0025 ms (gate 0.02); `wind` example (CPU arrows over the GPU-shaded field, burst/vortex keys; visual check WD-0); app fog/cloud drift read the field. Next: WE1.
- 2026-10-18 (p10b): M4 P10 [~] `sf_character::cook_cloth` (farthest-hit lattice placement, girdle-ride offsets, per-particle authored skin targets, `SkinRebind` bilinear + harmonic waist band), `RagdollDef::cloth_colliders`, `mw_physics::MwClothRules`; census: robes are Chest parts (not Skirt), 50 cloth parts all skinned, 48 cook in 9 ms median, no hem holds a 50° run stride (+13..+133 %). Next p10c op + app + PH-3.
- 2026-10-18 (u1b): M5 U1 [x] part 2: `sf_ui_bevy` (screen-stack router over per-input-context BEI entities, consume + require_reset UI actions, rebindable `UiBindings`, confirm → `Activate`, layer-aware `AutoDirectionalNavigation` + focus memory + repeat, `InputMode` + glyphs); UI sets must run before BEI `Prepare` (else a closing modal's B reached the menu); 6 headless tests, `ui_nav --autotest` synthetic-gamepad PASS, clippy clean; UI-1 awaiting owner. Next: u1c glue (VFS Fluent, settings/bindings ↔ profile) or U2.
- 2026-10-18 (u1c): M5 U1 part 3 (glue): Fluent from each mod's VFS `l10n/<locale>/<mod id>.ftl` in load order with per-mod issues, `UiRecordKinds` adapter, settings ↔ `Profile::settings`, new `Profile::bindings` (reflected input names, non-defaults), `insert_from_profile` + save on leaving menus/exit, `UiCommands`, presets enforced on open; 7 new tests, clippy clean, `ui_nav --autotest` PASS (superfargoth 860d027). Next: U2 `sf_ui_css`.
- 2026-10-18 (ts1a): TS1 [~] part 1: synthetic test levels: `sf_level` (Bevy-free RON greybox format, validation, meshes = colliders; 6 tests), app `level.rs` (`view level:<name>`, scenes `level:`; own worldspace with an empty streaming index, prototype-grid materials, `Fixed` mesh colliders), levels `flat-slopes` + `ledges`, scenes `level-*`, TS-1 awaiting owner; ready 2.1-2.8 s after start. RON over DynamicScene/BSN/Skein (no Bevy 0.20 Skein yet). Next ts1b: actors by id, props, headless runner.
- 2026-10-18 (we1): M6 WE1 [x]: Bevy-free `sf_weather` (presets/climates/precip kinds as records, forecast as a pure function of seed + climate + slot with timestamped overrides in the save, per-channel lead/lag with never-morphing crossfades, border band, Poisson lightning by thinning, `WeatherFrame` with wind target, storms, `visibility_m`); `evaluate` 0.0006 ms (gate 0.05); superfargoth cbf4c44. Next: WE2 `mw_weather`, then WE3 `sf_weather_bevy`.
- 2026-10-18 (u2): M5 U2 [~] part 1: `sf_ui_css` on bevy_flair 0.9.0 (composer with flat `mods-<n>-<id>` cascade layers in load order, memory root source + app-registered `sf_ui_content`, root classes + `input` attribute, per-mod CSS errors via strict probe loads); 6 tests (flair cascade = load order, errors → mod id), clippy clean (superfargoth c3d1fcb). Licence set in ui.md (MPL only cssparser/selectors/dtoa-short, unmodified). Next: U2 part 2 (run `ui_css --autotest` hot reload, UI-2 visual check).
- 2026-10-18 (ts1b): TS1 part 2: level actors (outfit, scripted path/action, `PathFollower`) through the level index, `robe-walk` level + scene (TS-2 awaiting owner); TS-1 terrain-flash fix (no `TerrainCamera` outside the exterior, 0 patches verified via BRP); walk mode (`U`), dynamic props and the headless course runner (`sf_level/tests/course.rs`) written but not screen-verified.
- 2026-10-18 (u2b): M5 U2 [x] part 2: `ui_css --autotest` ran (before/after `shots/u2b/`): `broken_mod` attributed, a mod sheet edited on disk restyles in 0.33 s via Bevy's loader-dependency reload of the root sheet (the planned `server.reload` fallback was not needed); UI-2 awaiting owner; `ui_nav --autotest` PASS; clippy clean. Next: U3 widget kit + themes.
- 2026-10-18 (p10d): M4 P10 [~] cloth V1 first run: tests + clippy clean, two robes walk/run in `character` (BRP now on in interactive `character`), H A/B works, 20-26 µs per simulated robe (gate 30); Balmora Temple 9 parts / 6 simulated; at a run the hem hangs straighter than the stride; PH-3 awaiting owner (`scene ph3-balmora-temple`, `shots/p10d/`). Next: `cloth.cook` bake step.
- 2026-10-18 (we2a): M6 WE2 [~] part 1: Bevy-free `mw_weather` (REGN → climates, `[Weather*]` → presets, 4 precip kinds, lightning per game hour, transition from `Transition Delta` as real-time Hz, synthetic source "Morrowind.ini weather"); `MorrowindIni::get`; 7 tests (1 on the real install) green, clippy clean. Next: we2b (clouds/fog/wind terms, Red Mountain storm, region sounds, MWScript table, forecast command).
- 2026-10-18 (we2b): M6 WE2 [~] part 2: `mw_weather` presets carry the visual fields (cloud genus + `Cloud Speed`, haze/dust/fog with hour curves = the app's `SKY_*`/`FogLook` tables, `Wind Speed`, ash/blight storm + Red Mountain radial source, `Flash Decrement` → new `Lightning::fade_s`); 11 tests (2 real install) green, clippy clean. Next: we2c (region sounds, MWScript table, forecast command, app `define_into`).
- 2026-10-19 (we2c1): M6 WE2 [~] part 3a: REGN sounds → new generic sf_weather `OneShotSet` ("scatterer", real-second intervals, audio layer schedules) + `WeatherPreset.climate_ambience` (storms hush the region); `mw_weather::script` maps ChangeWeather/ModRegion/GetCurrentWeather/GetWindSpeed onto `WeatherState`, table in weather-adapter.md; mw_weather 13 + sf_weather 11 tests green, clippy clean. superfargoth d2c1898. Next: we2c2 (forecast command + frequency check, app `define_into`).
- 2026-10-19 (ts1c): TS1 [x] part 3: `--camera` through `scene` (ts1b report not reproduced; real bug fixed: a repeated `view` flag now overrides, so `scene X --camera` works on scenes with their own camera; `level::pick_camera` + 2 tests); walk mode verified via BRP (player dressed at `start`, W/Shift move it, follow camera, start/stop log line); props verified (`V` balls shove the crates; G colliders on meshes). TS-1/TS-2 awaiting owner.
- 2026-10-19 (we2c2): M6 WE2 [x] part 3b: weather records load as the synthesized mod `morrowind-weather` (mw_mods, after content, before user mods; a record mod patches a preset in a test); `bevyrrowind weather forecast <region> --days 3`; 1,000-slot frequency tests vs REGN (synthetic + all 16 real regions) green; clippy clean. Next: WE3 (we3a `sf_weather_bevy` WeatherPlugin + sky/wind).
- 2026-10-19 (p10s): M4 P10 [~] S1 robes and skirts drawn as authored by default (owner: full robe sim is overkill): `mw_physics::ClothMode`, `--cloth robes` opt-in keeps V1; unit test + clippy clean; walk/run lineup shows legs covered, no `cloth:` line; PH-3 rewritten (awaiting owner); parked leg follow, hem widening, `cloth.cook` bake; S2 hem-only painted-max-distance drape logged.
- 2026-10-19 (n0, docs only): co-op N0 [x]: `superfargoth/docs/specs/net.md` (listen server, own-character prediction, 8 net-ready rules binding, ownership per system, context player, multi-region host, saves, quest model, N1 crate criteria) + audit table of all specs; physics.md "Controller" mover on the fixed tick; specs README index (action, camera, net); new "## Co-op (N0-N8)" section; P11 points to action.md A1-B5; §10 entry. No builds.
- 2026-10-19 (we3a): M6 WE3 [~] part 1: new `sf_weather_bevy` `WeatherPlugin` (SkyTime → `sf_weather::evaluate` at the camera's REGN climate → `CurrentWeather`; drives sky haze/dust/tint + wind field and storm source; `weather/*` counters; 0.0027 ms, gate 0.05; `WindState::source_mut`); app `--weather <type>`, haze/dust table replaced by the presets, `--clouds` defaults to the start weather; 4 tests green, clippy clean; Seyda Neen clear/overcast screenshots sane, WE-0 awaiting owner. Next: we3b (clouds genus blend, fog, precip, lightning, border blend).
- 2026-10-19 (we3b1): M6 WE3 [~] part 2a: clouds (genus blend, speed), fog (height, banks, ash/blight albedo) and sky cover/dimming follow the live weather via `sf_weather_bevy` (`apply_clouds`/`apply_fog`, `WeatherOverrides.clouds`/`fog_scale`); app scaffold tables removed (`CLOUD_CHOICES`, `SkyLook`/`FogLook`/`SKY_*`); live weather controls (C next weather, T clock x1/x4/x20, BRP `set weather`/`transition_hours`/`time_scale`, `status.weather`); 6 framework tests green (transition glides, clouds lead), clippy clean; Seyda Neen clear → overcast shots sane, WE-1 awaiting owner. Next: we3b2 (precip, lightning/thunder, border blend).
- 2026-10-19 (fogfix): fog on blended NIF surfaces (leaves, Ghostfence, glows): `transparent_fog.rs` swaps them to FroxelFogMaterial, far fog past the froxels (superfargoth 8aa4fba); pipeline audit in weather.md; FG-5 awaiting owner.
- 2026-10-19 (we3b2): M6 WE3 [~] part 2b: precipitation (new superfargoth `sf_precip_bevy`: closed-form camera-wrapped streak/flake volumes, wind slant, blowing-ash streaks), lightning (pure flash + thunder delay, flash light, cloud glow), `WeatherSound` cue messages, region border blend in the app; tests green, clippy clean; BRP rain/ashstorm/thunderstorm sane (target/we3b2); WE-2 awaiting owner. No occlusion map or audio yet.
- 2026-10-19 (lodaudit): foliage-vanishing audit: CPU visibility chain clean live (BRP step-in); per-instance `VisibilityRange`s exhausted Bevy's 65535 app-lifetime GPU slots (slot-0 aliasing dithers near meshes out) -> `shared_range` log-grid snapping (superfargoth); LA-1 awaiting owner.
- 2026-10-19 (rainwater): rain hidden over the ocean = Transparent3d sort key (water clipmap and precip boxes both shader-placed at the origin, order flipped with pitch; water erased the rain). New superfargoth `sf_render_order_bevy` (`DrawBand`: water Refractive first, translucents Sorted, precip Overlay last, one PhaseSort system); water.md "Render order" table; 3 headless tests; BRP shots rain over sea ok; WE-1 note awaiting owner.
- 2026-10-19 (shimmer): fog pulse root cause = one Halton jitter for the whole froxel volume (16-frame cycle, history 0.9, read at the jittered point): coherent periodic error TAA cannot clamp; measured ~1-2 % frame luminance swing with a still camera at x1 (not a time-lapse effect). Fix: per-froxel R3+pcg3d jitter, history at froxel centres (superfargoth sf_atmos FroxelJitter); A/B `--off fogdither`/`fogjitter`; FG-6 awaiting owner.
- 2026-10-19 (renderorder): render-order audit after the day's four order bugs: superfargoth specs/render-order.md (frame graph in order, per-material fog/sort/depth/prepass/shadow inventory, six rules, UE/Frostbite comparison, violations RO-1..12) + guard test `sf_render_order_bevy` `translucent_guard`. Top: RO-1 blended/additive NIF surfaces and particles cast shadows (High), RO-2 distant light sprites and RO-3 precipitation unfogged, RO-4 translucent fog swap lives in the host. No fixes (cheap ones touch leafmask files).
- 2026-10-19 (physcrash): owner crash `rapier persistent.rs member(link.body1) || member(link.body2)` (Seyda Neen, time-lapse) = `refresh_queries` ran rapier's collision-only pass, corrupting persistent islands under streaming churn; now it only updates the broad-phase BVH (superfargoth ff33eb3); headless `churn_tests` (chunks, item piles, capsules, rebases, 20 steps/frame) pass with rapier validation.
- 2026-10-19 (leafmask): Morrowind foliage as alpha-tested cutouts (`mw_nif` AuthoredAlpha::resolve, CUTOUT_CUTOFF 0.5; texture-alpha classifier for plain blends; importer v3, bake 62 s): leaves write depth, take the composite fog, cast/receive sun shadows, write motion vectors; translucents stay blended; `--off leafmask` A/B (dev build, Seyda Neen: GPU 8.03 vs 8.07 ms, render thread +1.7 ms); LM-1 awaiting owner. Left: Castaño coverage mips need a texture cook.
- 2026-10-19 (we4a): M6 WE4 part a: `sf_weather::WeatherMap` CPU world weather map (climate raster with sea dilated to the nearest region, per-climate table at weight 1, toroidal world-anchored texel window, uniform skip); map at the camera = `evaluate(site)` (1e-5, 2 m border walk); table 0.0034 ms / 16 climates release. 5 tests, clippy clean. Next WE4b (GPU resolve + clouds sample the map).
- 2026-10-19 (shf12): shadows F1+F2: Bevy `ContactShadows` on the exterior camera (+ depth/MV prepass on Medium+), `contact_shadows_enabled` on the sun (`sf_sky_bevy::SunShadow`) and budgeted point lights (`PointShadowBudget::contact_shadows`), sun normal bias 0.8 texels, `ShadowFilteringMethod` per preset; `--off contactshadows`; weather.md "Shadow bias and contact shadows"; SH-7 awaiting owner; GPU cost A/B not run (owner was testing).
- 2026-10-19 (rofix1): render-order RO-1/2/3 fixed: translucents cast no shadows (`FroxelFogExt::enable_shadows` false; cutouts keep casting), distant light sprites and rain/snow fogged through the new shared `sf_render_order_bevy::TranslucentFog` + `translucent_fog` shader module (moved from sf_atmos_bevy); guard test checks fog binding + shadow opt-out; RO-A awaiting owner.
- 2026-10-19 (shf3): shadows F3 step a: practical-split sun cascades to 250 m per preset (`sf_sky_bevy::CascadeSplits`, Bevy keeps sphere fit + texel snap + overlap blend), texels 2× sharper in the last cascade, GPU ≈ 0; 504 m rule replaced (ARCHITECTURE §10); far static map (step b) designed in weather.md, not built; `--off shadowsplits`; SH-8 awaiting owner.
