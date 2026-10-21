# Roadmap

Live plan and status: short on purpose. **Rewritten 2026-10-19 (phase0)** after the owner's audit: the work is
ordered in **phases**, not milestones. Milestone numbers (M2–M10, ARCHITECTURE §9) stay as scope labels for specs.
Everything before 2026-10-19 (as-built notes, measurements, the session log) is in
[`archive/ROADMAP-to-2026-10-19.md`](archive/ROADMAP-to-2026-10-19.md): grep it, don't read it whole.

**Rules for this file.** At most ~250 lines. One line per task: status mark, ID, what, where the detail lives
(spec, visual check, archive). As-built detail goes in the spec's "As built" part and the commit message, not here.
A task is `[x]` only when its Verify ran; visual tasks need the owner's eyes (`visual-checks.md`).
Legend: `[x]` done, `[~]` started, `[ ]` not started, `[p]` parked (needs the owner's go to resume).

## Owner decisions

Decided 2026-10-19:
1. **Frame target:** High preset, 1440p + DLSS Quality on the owner's laptop (Ryzen 7 7435HS, RTX 4060 Laptop):
   ≤ 11 ms median exterior (~90 fps), stretch 7 ms (~140 fps). Today ~30 fps.
2. **Ray tracing:** raster is the main path and must meet the target; Solari is the "Ultra" tier and may cost
   more ("ray tracing will impact perfs, that's okay"; WISHLIST §0: 60 fps).
   **Not a priority** (owner, later 2026-10-19): RT comes "once we have a nice game already, like cherry on top"
   → moved to Phase 4.
3. **Shadows: something simpler that works well first.** Park the far static sun map and the point-shadow cube
   cache. New techniques only when they are the better, industry-standard way, not as extra layers.
   **2026-10-21 (after the render audit):** (A) stock Bevy shadows, delete the injected layers (lamp cache, far
   static map, extra sun views where possible), nearest lamps + torches by budget, short contact shadows, GTAO;
   then (B) carry Bevy PRs #26149/#26161 as a vendored patch. Torch-bearers cast shadows (within the budget).
   Own shadow atlas (C) only if A+B fall short. First: diagnose the ~0.8 ms GPU per shadowed lamp.

Still open (low priority): storm spill past region borders (~2 km vs strict); co-op guest imports a single-player
character (default: create only); cargo-deny in superfargoth; UAL2 clip pack purchase.

## Phase 0: stop and clean up (now)

- [x] 0.1 Audit (2026-10-19, owner session): causes = docs eat agent context, no perf gate, no temporal resolve
  (grain), five half-built shadow systems, owner check queue overflowing, RT role unclear.
- [x] 0.2 Visual-check queue pass with the owner (2026-10-19, 14 windows): 17 confirmed, 14 partial/rejected with
  notes, 50 retired as stale, 23 deferred to the phase that reworks them, 4 left for the next pass.
- [~] 0.3 Doc diet: ROADMAP (this file) and ARCHITECTURE (§10/§11 → `superfargoth/docs/DECISIONS.md`) done;
  visual-checks.md → open checks only (closed ones to `archive/`); start-of-session reading in both CLAUDE.md files.
- [ ] 0.4 Specs over ~25 KB split into design + short "As built" (weather.md 172 KB, animation.md 108 KB,
  physics.md 72 KB, water.md 62 KB, locomotion.md, character.md, streaming.md, ui.md, scripting.md); history to
  `archive/` next to them. Do each when its phase starts, not all at once.
- [x] 0.5 `portal_spike.rs` + its shader, `--portals` flags and the 9 `portal-*` scenes deleted (owner go 2026-10-19;
  git history keeps them, design and results in `superfargoth/docs/specs/portals.md`; `mw_world::portal` stays).

## Phase 1: performance, stability, image quality

Goal: the frame target (decision 1, raster path) in Balmora, Seyda Neen and the flight bench, with clean, grain-free shadows
and effects. Every task reports its ms cost per preset.

- [~] 1.1 **Stability first.** Built 2026-10-19 (phase1), awaiting owner (DR-4, AU-1 in visual-checks). DR-4 crash (command on a despawned entity on leaving an interior): streamed
  entities via `try_insert`/`queue_silenced`; shipped builds set Bevy's error handler to log, dev keeps panicking.
  Weather/region sound plays indoors (AU-1). Door travel: black fade held until the destination's near ring is
  spawned (timeout), no rough far-field shapes or loading objects on arrival (DR-4).
- [~] 1.2 **Frame budget + gates.** 2026-10-19 (phase1): median 11 ms gates + budget table + A/B and trace in `perf.md`
  "Frame budget": 14.3-19.3 ms, **render thread bound** (sun cascades ~4.8 ms, actors ~3 ms main); caster size rule
  ~0.3 ms; cube-cache plugin dropped (parked). Eye-height walk `balmora_walk` 26 ms: the near-ring statics are the
  cost (`--off statics` 7.5 ms); NoCpuCulling tried (81 ms, dropped); anim budget on. Next: merge statics per cell at
  bake time (OpenMW/MGE practice), fewer shadow views; Nsight per pass (UAC). Nsight per pass + Tracy per system on the owner's laptop; a budget table per
  preset in `perf.md`; `benchmarks/exterior_*.json` get a median gate at the target (they only checked 33 ms, so
  the fall from ~11 ms on 10-12 to ~30 fps never failed). Then cut whatever is over budget.
- [~] 1.3 **Temporal resolve as the base.** 2026-10-19 (phase1): `Upscaling::Taa` (Bevy TAA) default when DLSS is off
  or missing, MSAA on Low only, F4 cycles; 1440p TAA 15.4 ms vs MSAA 18.5; history reset on door travel. Awaiting
  owner (TA-1); motion vectors for translucents (RO-6) still open. Bevy TAA on the raster path when DLSS is off (Medium+), DLSS when on;
  every pass writes motion vectors (render-order.md RO-6/RO-7); MSAA off where TAA replaces it. Fixes the grain
  in weather/fog/soft shadows (WE-2, WE-3), grazing-light edge shimmer (SH-12), water moiré (WA-1).
- [ ] 1.4 **One shadow design, finished.** Sun: cascades + contact shadows + terrain far shadow (keep SH-7/8/10).
  Fix distant flashes on camera move (SH-11); lamp shadows: short reach, grain, flicker, wrong guard outline
  (SH-13); leaf shadows on some trees only (LM-1: cutout classifier missed thin-twig leaf atlases, so they stayed
  blended without shadows; sparse-atlas rule added 2026-10-21, awaiting owner); translucent casters (RO-1). Far static map and cube cache
  parked (decision 3). Owner 2026-10-20: shadows still bad overall ("only showing when super close");
  **contact shadows confirmed** as the thin view-dependent bands at object edges (SH-14: shorten to Bevy's
  0.3 m or close range only); soft-shadow (PCSS) size is in cascade texels, so penumbrae jump between
  cascades; with the cube-cache plugin dropped, a shadowed lamp redraws 6 faces/frame (~1.9 ms): decide
  High = 0 lamp shadows or unpark. Two A/B lessons: no rule that depends on view-refit cascades (caster cull),
  no budget that freezes poses (anim budget).
  **Lights done first (2026-10-20, lights1, `specs/lights.md`):** unshadowed lights are ~free (GPU clustering), no
  budget built; a Bevy leak of stale lamp cube-face views fixed on our side (walk 14.7 → 12.7 ms). One lamp
  shadow still costs ~1.5 ms on the walk: owner picks High = 0 lamps, 1 at this cost, or cache slice 2b.
  **V0 + V1 built (2026-10-20, shadows2):** lamp cache fixed (it cached nothing) and on; High = 8 lamps,
  fade 30-40 m; census 8 lamps 16.8 → ~6.3 ms; walk gate not met (walkers by lamps). Owner: SH-15, High's count.
  **Audit 2026-10-21 (render-audit):** no renderer rewrite; shadow lane reset proposed (Bevy patch instead of
  injection, delete layers, GTAO + capsule shadows); owner decision pending. `research/shadows/2026-10-21-audit.md`.
  **Reset A1 done (2026-10-21, shadowA1):** lamp contact off, sun 0.3 m, sun/moon out of interiors, GTAO; lab 8 lamps
  12.5 → 6.9 ms, census GPU 9.97 → 5.90; torch-bearers in the budget (verified). SH-16 awaiting owner.
  **Reset A2 done (2026-10-21, shadowA2):** lamp maps captured during the door fade (gate, +0.35 s census, cap 1.5 s);
  far static map, caster cull, `shadowtune`/`shadowsplits` deleted (~820 LOC); no perf change. SH-17 awaiting owner. Next B.
  **Reset B1 done (2026-10-21, shadowB1):** Bevy PRs #26149/#26161 vendored (superfargoth `vendor/`); alone they lose to
  the cache (walkers/NPCs redraw whole cubes: census 15.0 vs 9.0 ms), so the cache stays and the patch ships under it:
  census 9.0 → 6.5 ms, walk 13.3 → 13.0 (perf.md "Shadow reset B1"). SH-18 awaiting owner.
  **Reset B2 done (2026-10-21, shadowB2):** own `bevy_pbr` hunks: point/spot shadow fetch skipped where the light adds
  nothing (exact), lamps on 2x2 hardware PCF (`--off lampfilter`): -0.1 ms GPU per shadowed lamp, census opaque
  2.94 → 2.08; frames render-thread bound, medians ~unchanged (perf.md "Shadow reset B2"). SH-19 awaiting owner.
  Spec: `superfargoth/docs/specs/shadows.md`, research `research/shadows/README.md`.
- [~] 1.5 Ambient occlusion: Bevy's SSAO (GTAO), per preset. Built in A1 (Low off, Medium/High/Ultra = Bevy Low/Medium/
  High; ~0.9 ms GPU at 1440p native on High, perf.md "Shadow reset A1"); look awaiting owner (SH-16).
- [x] 1.6 Object-terrain blending removed (2026-10-20, lights1): code, `--off terrainblend`, F11, TB-1..3; `lod.near` v3.
  Walk 1440p 25.9 → 14.7 ms, Balmora 15.9 → 13.1 (perf.md "Terrain blend removed"). Redo is eye candy (WISHLIST §13).
- [~] 1.7 Night brightness (LT-1 rejected: "can't see anything"): exposure floor + night ambient retune; then
  the deferred night checks (SK-2, NS-2..4). 2026-10-21 (renderfix1): night ambient 1 → 4 nits (~2 EV under mid grey
  on stone), EV100 floor 3.5 kept; LT-1 awaiting owner.
- [ ] 1.8 Draw-order and far-field bugs: ash/blight storm drawn over terrain and objects (WE-5: precip streaks ruled
  out 2026-10-21; suspect ash height fog or cloud composite; next A/B `--off clouds|fog|froxels` at the owner's view); Ghostfence field
  has no far copy and its far light sprites float in the gap (GF-1, B5); far-field L2/L3 never selected or never
  tinted (FF-2: 25 L2 + 4 L3 nodes resident).
- [~] 1.9 Streaming and culling levers from the old M2 list (archive "Next up" 1, 6, 7): render-thread spikes,
  `NoCpuCulling` A/B (J2), shadow-caster rules (J4), pipeline warm-up (J7), precombine evaluation (J6);
  deferred checks ST-2, ST-3, SC-2. **Precombine (J6, owner go 2026-10-19)**: `superfargoth/docs/specs/
  precombine.md`; slices 1-3 built (2026-10-20, uncommitted): meshes -45 % but frame unchanged (perf.md); the
  walk's gap was terrain blend (1.6). Default on/off still the owner's call. `NoCpuCulling` retry once every view does GPU culling
  (perf.md "Lead"); big_space propagation patch; skinned bounds (research/perf/README.md).
- [~] 1.10 Volumetric fog black bands (FG-1, owner 2026-10-20): dark vertical bands in light-lit fog at night
  when the camera turns fast. 2026-10-21 (renderfix1): the Sadrith Mora repro was a static lamp-range edge (428k lm,
  29 m range), not the bug (TAA, sun cascades, froxel history ruled out there). Owner's bars unreproduced: need the
  owner's camera (F9) + DLSS mode at the cave/mushroom spot, then A/B foghistory / F4 / froxels (`spin.sh`).
- [ ] 1.11 Thin binary (ARCHITECTURE §8b rule 4): the app is 13.5k lines and `exterior.rs` is the most-edited
  file; move systems into plugin crates. Re-measure the dev-loop gates (last measured 10-12, half the crates).

**Next session, in order (owner 2026-10-20):** (1) 1.6 remove terrain blend (done); (2) lights (done, lights1) then 1.4 shadows (contact shadows SH-14, PCSS consistency, lamp reach SH-13, cube-cache
decision); actor perf parked until the animation system exists; (3) FG-1 fog bands; (4) owner checks still open: PA-4 (particles), DR-4,
AU-1, TA-1; Nsight per pass (UAC).

## Phase 2: a playable vertical slice (the basics)

Done when: new game in Seyda Neen → walk in third person → talk to Fargoth → get his ring back → journal entry →
save, quit, load. Break it down (and write the missing specs: dialogue, journal, saves) before building.

- [~] 2.1 Player character: third-person controller + camera (`specs/action.md`, `camera.md`; `sf_action`, `sf_camera`),
  activation with a "press to use" prompt. `sf_action` A1 done 2026-10-21 (commands, RON move rules, two-stage buffer,
  chooser, jump buffer/coyote/landing, use prompt, net-ready state); `sf_camera` rig math done (orbit spring, push-in,
  lock-on, first-person limits, shake); `sf_controller` launch + serde `Locomotion` + shared `PlayerId` (`sf_ids`) done;
  next A2 (`sf_action_bevy`, movers on the fixed tick, app wiring) and D1 (`sf_camera_bevy`).
- [ ] 2.2 UI: widget kit + Morrowind theme (M5 U3), HUD + message box (U4), inventory on minimal `sf_rpg` stacks
  (U5), settings/options + keybinds (U6). `sf_ui`, `sf_ui_bevy`, `sf_ui_css` exist (U1, U2 confirmed).
- [ ] 2.3 Text: fonts, books and scrolls, notifications.
- [~] 2.4 Dialogue + journal + minimal quest store (`specs/rpg.md`, `rpg-adapter.md`; `sf_rpg`, `mw_rpg`), MWScript legacy runtime for the slice (M5 W3;
  MWScript → Luau translation exists, W1/W2). `sf_rpg` core done (2026-10-21: response DB + session, stacks/loadout,
  quests/journal/topics, topic linker). `mw_rpg` done 2026-10-21 (INFO order, filters with OpenMW semantics, macros,
  journal, equip slots): Fargoth's ring headless test passes on real data + save round trip. Next: W3 Luau result
  runner (`ResultRunner`), a game `MwView` (stats table until M7), dialogue UI (2.2) and app wiring.
- [ ] 2.5 Saves (M7 subset): world overlay + player; net-ready per `specs/net.md`.
- [~] 2.6 Sound pass: footsteps from animation text keys, UI sounds, music, interior/exterior ambience
  (`sf_audio`, `sf_audio_bevy`). Framework done 2026-10-21 (cues, attenuation, music moods, footstep hook, ambience
  layers; positional one-shots/emitters on bevy_audio). Morrowind wiring done 2026-10-21 (`mw_audio`: SOUN/SNDG cues,
  door sounds, loop emitters, vanilla footsteps, VFS music; app `audio.rs`); AU-2 awaiting owner. Left: UI sounds (no
  menus yet), swinging-door sounds, boots class from equipment, modern-preset beds/surfaces/bus volume settings.

## Phase 3: animation, combat, physics

- [ ] 3.1 Traversal/action per R11 decisions (`research/animation/11-tradeoffs.md`, `specs/action.md` phases);
  old P11. Clip sources: packs only (free first).
- [ ] 3.2 Ragdolls (PH-2 rejected). **Lead:** the physics capsules behave; the skinned mesh doesn't follow them →
  pose write-back bug (body/bone offset, frame, or floating origin). Then powered falls (PH-9: no fall animation).
- [ ] 3.3 Physics follow-ups: Oblivion-style hold/drag/throw (PH-4), items with no shape (ceiling candle holders,
  small boxes: PH-7), immovable candles (PH-1), ball pushing items through walls (archive 6a).
- [ ] 3.4 Deferred character checks: SN-2/3, CH-1/3/4, SB-1/2/3/5, HI-3, AP-2/3, PH-10.
- [ ] 3.5 Combat (M10).

## Phase 4: eye candy and the wider world (after phase 2)

- [p] Weather polish: storm blows from Red Mountain (WE-5), clouds too tall in some weathers, ash particles,
  precipitation splashes (PR2), surfaces/wetness/snow (SU1, SU2), wind on trees (WD2, WD3).
- [p] Water polish (WA-1 notes, WISHLIST §12): shore swash + gradual wavelets, textured foam, whitecaps,
  weather-driven wave height (verify), crest SSS; W4 reflections, W5 underwater, W6 buoyancy, W7 rivers.
- [p] RT as the Ultra tier (decision 2; was 1.10): exterior RT content (`ray_tracing_lod`, `NoRayTracing` on proxies, terrain RT
  proxy), Solari sunset patch; `--rt` in the streamed view (it still opens the single-cell viewer: no actors, no
  doors); characters in the RT scene (compute skin cache, DECISIONS 2026-10-11); black bands at contact edges (DL-4);
  DL-3 confirmed 2026-10-19, DL-2 open (needs a `solari,dlss` build).
- [p] Raster GI & sky occlusion (owner 2026-10-21): Bevy 0.20 `IrradianceVolume` / sky-visibility SH probes baked per cell in `sf_bake` (occludes sky cubemap under overhangs/alleys/interiors + 1-bounce color bleed) + screen-space GI/bent normals over the depth/normal/history prepass.
- [p] Foliage light transmission (WISHLIST §11).
- [p] Cloth: capes/banners/tents (robes stay authored skin; PH-3 confirmed).
- [p] Seamless portals (M9), far-field performance (M8: HLOD, texture streaming, PBR generation).
- [p] Co-op N1–N8 (`specs/net.md`; N1–N5 after this phase, N6–N8 after M7).
- [p] Lua/Luau beyond L3; mods milestone demo (M5 E1, U7, X1, X2).
- [p] Crate "update all" pass once `bevy_hanabi` supports Bevy 0.20 (archive Backlog).
- [p] Vanilla option (owner 2026-10-20, not urgent): players choose classic or modernized. (a) Classic graphics
  preset: short view distance + fog, no shadows/volumetrics, Morrowind-like simple lighting, original sky; built
  on the `--off` switches (feature ladder, perf.md). (b) Vanilla rules profile (data-driven, after Phase 3).
  OpenMW as behaviour reference only, no GPL code in superfargoth.
- [p] Groundcover (owner 2026-10-20, low priority; replaces archived G0–G2). (a) Procedural grass: evaluate
  `bevy_grass` (github.com/GEMI/bevy-grass: MIT, Bevy 0.20, GPU compute scatter, 3 LODs, wind, trampling, far
  colour; git only, pre-release) before building our own; masks from LTEX, wind from `sf_wind`. (b) Later: load
  existing Morrowind groundcover mods (mesh instances from their plugins; bevy_grass has no custom meshes).

## Recent (last ~10 entries; older ones drop into the archive)

- 2026-10-21 (action-a1): mover launch (no ground snap while rising, head bump; 4 failing-first tests), serde
  `Locomotion`, `PlayerId` merged into new `sf_ids`; A2 prerequisites done.
- 2026-10-21 (renderfix1): LM-1: 66 flora textures (Bitter Coast leaves, branches, roobrush, wickwheat...) were
  rejected as cutouts and stayed blended (no shadows); sparse-atlas rule + `mw_bake` `foliage_report` example.
- 2026-10-21 (rpg-core): `mw_rpg`: Morrowind dialogue on `sf_rpg` (OpenMW-order INFO filters, macros, journal, equip);
  Fargoth's ring flow passes headless on the real ESMs (12 + 1 tests); result scripts behind `ResultRunner`.
- 2026-10-21 (audio-pass): Morrowind sound in game: `mw_audio` (SOUN/SNDG cues, GMST ranges, music playlists), load-door
  sounds, light/activator loop emitters, stride footsteps, explore music; audio tests no longer build the renderer.
- 2026-10-21 (action-a1): `sf_camera` rig math, Bevy-free (orbit + critically damped pivot, sphere-cast push-in, lock-on
  hysteresis/LOS, first-person limits, trauma shake; 16 tests); action.md split (30.5 KB + archive); dolly rejected.
- 2026-10-21 (renderfix1): night ambient 1 → 4 nits (LT-1 awaiting owner); FG-1 repro + `--off foghistory`: not the
  froxel history, diagnosis open.
- 2026-10-21 (action-a1): `sf_action` A1, Bevy-free: input sampler + command feed, RON move rules with load errors,
  Sekiro two-stage buffer, chooser, jump/coyote/landing, use prompt; deterministic at 30/60/144 fps (21 tests).
- 2026-10-21 (audio-pass): `sf_audio` cues/sound groups, distance curves, music mood playlists with crossfade, footstep
  hook, ambience layers; `sf_audio_bevy` positional one-shots + loop emitters on bevy_audio (30 tests).
- 2026-10-21 (rpg-core): specs `rpg.md` + `rpg-adapter.md`; `sf_rpg` built (dialogue response DB + session, item
  stacks/loadout, quests, journal, topic linker; 19 tests); Fargoth quest IDs verified against the installed ESMs.
- 2026-10-21 (shadowB2): shadow reset B2: own bevy_pbr patch, lamp shadow fetch skipped where unlit + lamps on 2x2
  hardware PCF (sun keeps Temporal): -0.1 ms GPU per shadowed lamp at 1440p; SH-19 (edge look) awaiting owner.
- 2026-10-21 (shadowB1): shadow reset B1: Bevy PRs #26149/#26161 vendored (3 crates); cache kept (patch alone loses with
  walkers), patch + cache: census 9.0 → 6.5 ms; `perf sweep --var a+b` steps two flags together.
- 2026-10-21 (doorwalls): SH-17 fixed: back in an interior after a trip outside, meshes drew depth only for seconds
  (main-pass pipelines compiling; a start indoors was LDR and the sky made the camera HDR). Camera HDR everywhere; door fade
  holds while visible meshes wait for pipelines (`pipeline_watch.rs`, also logs compiles).
- 2026-10-21 (shadowA2): shadow reset A2: lamp shadow captures while the door fade is black (no arrival flicker), dead
  shadow layers deleted (far static map, caster cull, legacy cascades; ~820 LOC); perf runner ignores idle-clock GPU load.
- 2026-10-21 (shadowA1): shadow reset A1: lamp contact shadows off, sun contact 0.3 m, sun/moon off interior views
  (layer-0 leak; they never lit interiors), GTAO per preset (+0.9 ms GPU High 1440p); lab 8 lamps 12.5 → 6.9 ms.

- 2026-10-21 (lightlab): light lab: level `light-lab` + `perf sweep` (one variable per run); cost model in perf.md:
  per lamp 0.11 ms unshadowed, +0.49 ms shadowed (cube+PCF), +1.15 ms with contact (0.80 march); sun leak reproduced.

- 2026-10-21 (render-audit): rendering + shadows audit (code, Bevy ecosystem, industry); shadows2 + particles1
  work committed. Owner decides the shadow reset (superfargoth `research/shadows/2026-10-21-audit.md`).

- 2026-10-20 (shadows2): shadows v2 V0 measured + V1 lamp cache made to work (8 bugs: non-casters tracked, writes
  taken for moves, full `Core3d` per face, capture frames dropping movers, streaming recaptures...); High 8 lamps.

- 2026-10-20 (particles1): NIF particles drawn by vertex pulling (ShaderBuffer + MeshTag, CPU sim kept): Balmora
  960p 9.9 → 9.0 ms (= `--off particles`), walk unchanged (perf.md "GPU particles"); PA-4 awaiting owner.

- 2026-10-20 (lights1): 1.6 terrain blend removed (walk 25.9 → 14.7 ms, Balmora 15.9 → 13.1); lights researched
  (`specs/lights.md`: no budget needed), Bevy stale shadow-view leak worked around (walk → 12.7 ms). Then audit:
  big_space patch, env map every 8 frames (perf.md "Audit"); shadows v2 spec; V1 lamp cache built, off (handoff).
