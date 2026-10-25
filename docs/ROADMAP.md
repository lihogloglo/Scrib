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

Decided 2026-10-23 (`superfargoth/docs/ARCHITECTURE.md` §1b):
4. **Modern rules are the goal; classic rules are a cheap nice-to-have, built last.** Content mechanics both need
   (MWScript, dialogue, magic effects, levelled lists, factions, crime) are ported from OpenMW here (cite file +
   commit), not re-derived. Feel systems (combat, progression, AI, formulas, movement, UI flow) are modern first,
   each behind a seam a classic `mw_rules` version can fill later. Modern AI is very different from Morrowind's
   packages: content goals (escort, follow, travel, wander, fight) are the seam, not package execution.

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
  spawned (timeout), no rough far-field shapes or loading objects on arrival (DR-4). Owner 2026-10-25 (player mode):
  interior objects still visibly pop in after the fade: hold it until the interior is fully loaded (meshes + textures).
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
  on stone), EV100 floor 3.5 kept; LT-1 confirmed by owner 2026-10-21. Left: the deferred night checks.
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
  when the camera turns fast. Not reproduced by the owner 2026-10-21 (fog code unchanged; sun cascades changed in
  shadow reset A2/B3, likely the fix). Reopen if seen; `--off foghistory` A/B kept.
- [ ] 1.11 Thin binary (ARCHITECTURE §8b rule 4): the app is 13.5k lines and `exterior.rs` is the most-edited
  file; move systems into plugin crates. Re-measure the dev-loop gates (last measured 10-12, half the crates).
- [ ] 1.12 Owner pass 2026-10-21 bugs: lamp light cut along cluster block edges, view-dependent (LT-4, lane
  `lightclusters`); choppy character shadows (SH-20, `charshadows`); smoke not drawn in front of the sky (RO-B,
  `smokeorder`); terrain looks glossy (TR-3: SSAO + specular occlusion added to `terrain.wesl` 2026-10-25, small visible change,
  awaiting owner at ground level; then env-specular-only A/B or roughness clamp); ash storm occlusion (WE-5: map now looks along the
  travel direction, >= 15 deg, terrain occludes, `--off precipocc`). Owner 2026-10-25, not urgent: lee far too
  long (no ash in Balmora); want shelter only a few metres behind a vertical object: cap the shelter distance (particle
  depth minus occluder depth ≤ N m, one subtraction in the shader) plus a steeper angle; GF-2 Ghostfence look confirmed by owner 2026-10-25 (no change).
- [~] 1.13 Dev console (owner 2026-10-21): front end built 2026-10-23 (`sf_console_bevy`, `console.md`); left: register
  the app's switches (bevyrrowind slice). In-game console with live on/off for every `--off` part (SSAO, contact
  shadows, lamp filter, fog history/jitter/froxels...), so the owner can A/B without relaunching; SH-16/SH-19 wait on it.
- [~] 1.14 Build churn (2026-10-21): built 2026-10-23 (superfargoth `workspace-hack`); left: re-measure. Workspace-hack (`cargo hakari`) so per-`-p` feature sets stop rebuilding Bevy
  (22-32 cached `bevy_render` variants per target); re-measure dev-loop gates.

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
  A2 slice 1 done 2026-10-22 (`sf_action_bevy`: fixed tick, interpolation, correction spring, input sampling);
  D1 `sf_camera_bevy` shell done 2026-10-23. A2 slice 2 + D1 app wiring built 2026-10-25 (`player.rs`: third person in
  view/level runs, F11 player/free camera, walk/jump identical at 30/60/144 fps; Space jump, E use). Owner 2026-10-25:
  "everything works" (capsule walk). Step 2 (player2): scenes with a camera keep it, `level/walk.rs` retired, doorless
  links picked from the body. Step 3 (player3): focus = physics ray + actor capsules (sphere-pick scaffold deleted),
  E blocked in menus, interior starts at the door marker or on the floor; PL-1 awaiting owner. Owner notes 2026-10-25 (log only, fix later): too much
  inertia (move rules); carrying an object pushes the capsule through the floor, jittery (see 3.3).
- [~] 2.1b Input (with 2.2 U6): built 2026-10-25 (input, input2; `controls.rs`, tests green), IN-1 awaiting owner. Game actions in `UiKeymap` (kbm + gamepad defaults), F10 keybinds screen, per-action
  hold/toggle option for sprint/sneak (owner: everything HOLD by default; today sprint holds, sneak toggles).
- [~] 2.2 UI: widget kit + Morrowind theme (M5 U3), HUD + message box (U4), inventory on minimal `sf_rpg` stacks
  (U5), settings/options + keybinds (U6). `sf_ui`, `sf_ui_bevy`, `sf_ui_css` exist (U1, U2 confirmed); U5/U6 logic and
  stock dialogue/journal/inventory/keybinds/HUD screens (`sf_ui_screens`, `sf_rpg_ui`) built 2026-10-22/23. Left:
  app wiring done (2.4). U3 Morrowind theme 2026-10-25 (mwtheme): builtin-mod `ui.theme` (ini [FontColor] tokens,
  tx_menubook journal, OpenMW Mystic Cards OFL) over dialogue/journal/message box; screenshots OK (font, colours, book). Next: 8-piece frames, journal layout
  (page nav outside the book, side panel detached), theme hot reload.
- [~] 2.3 Text: fonts, books and scrolls, notifications. Framework slices 1-2 built 2026-10-23 (`ui-text.md`); left:
  the Morrowind markup parser and MessageBox mapping.
- [~] 2.4 Dialogue + journal + minimal quest store (`specs/rpg.md`, `rpg-adapter.md`; `sf_rpg`, `mw_rpg`), MWScript legacy runtime for the slice (M5 W3;
  MWScript → Luau translation exists, W1/W2). `sf_rpg` core done (2026-10-21: response DB + session, stacks/loadout,
  quests/journal/topics, topic linker). `mw_rpg` done 2026-10-21 (INFO order, filters with OpenMW semantics, macros,
  journal, equip slots): Fargoth's ring headless test passes on real data + save round trip. W3 result runner done
  2026-10-23 (native `mw_script` interpreter). Slice 1 built 2026-10-25 (`dialogue.rs`: `MwView`
  host, content-only disposition seam, stats placeholders, stock dialogue window; Fargoth test through the window
  passes). Step 2 (dialogue2): E talks to NPCs (unhandled `ActivationOutcome`), controls paused in menus, J journal
  (quest, entries, topic index), placement-keyed actors; DJ-1 awaiting owner. Next: Morrowind theme, HUD.
- [~] 2.5 Saves (M7 subset): world overlay + player; net-ready per `specs/net.md`. Framework S1+S2 built 2026-10-23
  (`sf_save`, `sf_save_bevy`, `save.md`); left: game sections + app wiring (after lane `player`).
- [~] 2.6 Sound pass: footsteps from animation text keys, UI sounds, music, interior/exterior ambience
  (`sf_audio`, `sf_audio_bevy`). Framework done 2026-10-21 (cues, attenuation, music moods, footstep hook, ambience
  layers; positional one-shots/emitters on bevy_audio). Morrowind wiring done 2026-10-21 (`mw_audio`: SOUN/SNDG cues,
  door sounds, loop emitters, vanilla footsteps, VFS music; app `audio.rs`); AU-1 + AU-2 confirmed by owner 2026-10-21.
  NPC voices, boots class, swinging-door sounds built 2026-10-23 (voices; `mw_audio::voice`, dev builds only),
  awaiting owner (AU-3..5). Left: object impact sounds (physics, e.g. the V ball), volume mixer (bus sliders in
  settings, with 2.2 U6), UI sounds (no menus yet), voice line-of-sight + combat/crime barks (Phase 3 hooks),
  modern-preset beds/surfaces.

## Phase 3: animation, combat, physics

- [ ] 3.1 Traversal/action per R11 decisions (`research/animation/11-tradeoffs.md`, `specs/action.md` phases);
  old P11. Clip sources: packs only (free first).
  Potential (owner 2026-10-22): evaluate MotionBricks as an animation source (licence, rig fit, quality vs packs).
- [~] 3.2 Ragdolls (PH-2 rejected). R0 write-back fixed headless 2026-10-22 (PH-2 awaiting owner, `level-anim-drop`, F7+G).
  Owner passes AN-1..21 fixed 2026-10-22 (AN-16..21 confirmed). Left: limp_drop/stairs stretch gates (more solver
  iterations add energy on contact), beast left toe pin. Then powered falls (PH-9), which the owner expects to help most.
  Order: `superfargoth/docs/specs/animation-plan.md`; test level `anim-lab`.
- [p] 3.2b Potential (owner 2026-10-22): delirium-physics active-ragdoll experiment, bevyrrowind only (slice R9 in
  `superfargoth/docs/research/animation/12-delirium-physics.md`). Fan project: accept the provenance risk, remove if challenged.
- [ ] 3.3 Physics follow-ups: Oblivion-style hold/drag/throw (PH-4), items with no shape (ceiling candle holders,
  small boxes: PH-7), immovable candles (PH-1), ball pushing items through walls (archive 6a). Owner 2026-10-25:
  physics props jump on entering a room untouched (spawn settling); held object vs player capsule (sinks, jitter).
- [~] 3.4 Deferred character checks: SN-2/3, CH-1/3/4, SB-1/2/3/5, HI-3, AP-2/3, PH-10. CH-1/3 variety and SB-5 feet
  done headless 2026-10-22 (awaiting owner); stairs 5b and powered falls R3 next (`animation-plan.md`).
- [ ] 3.5 Combat (M10). Owner 2026-10-22: not Morrowind's rules; adopt before building: evaluate
  github.com/Funny-Bones/ELDEN-RING-Playermodel-Rust (MIT code, Bevy 0.19; its timings/clips are extracted ER data,
  not shippable) as the base for action.md E1 (chains, guard, hit-stop, reactions) on our own clips.
  Keep a hit-resolution seam (ARCHITECTURE §1b) so classic swing + hit-chance can plug in later; don't build it now.

## Phase 4: eye candy and the wider world (after phase 2)

- [p] Weather polish: storm blows from Red Mountain (WE-5), clouds too tall in some weathers, ash particles,
  precipitation splashes (PR2), surfaces/wetness/snow (SU1, SU2), wind on trees (WD2, WD3). Owner 2026-10-21: rain
  puddles + wet (glossier) objects, cheap like the Clockwork renderer does it (research its approach first).
  Researched 2026-10-22 (`superfargoth/docs/research/borrow-audit-2026-10.md`: bevy_atmos technique, MIT/Apache).
  Owner 2026-10-22: wetness persists in saves; puddles reflect the scene with screen-space reflections
  (Bevy's SSR first), sky/probe as the fallback where SSR misses.
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
- [p] Classic option (owner 2026-10-20/23, nice-to-have, ARCHITECTURE §1b). (a) Classic graphics preset: short
  view distance + fog, no shadows/volumetrics, Morrowind-like simple lighting, original sky; built on the `--off`
  switches (feature ladder, perf.md). (b) Classic rules (`mw_rules`, ported from OpenMW) behind the feel-system
  seams, after each modern version exists. Morrowind's rules and numbers, not bug-for-bug OpenMW parity; our UI.
- [p] Groundcover (owner 2026-10-20, low priority; replaces archived G0–G2). (a) Procedural grass: evaluate
  `bevy_grass` (github.com/GEMI/bevy-grass: MIT, Bevy 0.20, GPU compute scatter, 3 LODs, wind, trampling, far
  colour; git only, pre-release) before building our own; masks from LTEX, wind from `sf_wind`. (b) Later: load
  existing Morrowind groundcover mods (mesh instances from their plugins; bevy_grass has no custom meshes).

## Recent (last ~10 entries; older ones drop into the archive)

- 2026-10-25 (player, dialogue, terrain-gloss): Phase 2 wiring step 1: third-person player (`player.rs`) and Morrowind
  dialogue window on `mw_rpg` (`dialogue.rs`), both headless-tested, not yet run in game; TR-3 terrain occlusion parity.

- 2026-10-23 (voices): NPC voice lines (`mw_audio::voice`: Hello/Idle/... INFOs via `mw_rpg` filters, greet GMSTs,
  idle rate, one line per NPC, RMS lip-sync `LipSync`), boots footsteps, swinging-door sounds; app glue `dev`-only.
- 2026-10-23 (pr-merge): merged sf #12-15 (camera D1, saves S1+S2, console front end, text/books) and bw #1 (MWScript
  runtime); tests green incl. real-ESM `fargoths_ring`. Owner decision 4: modern first, classic rules later (§1b).
- 2026-10-22 (anim-fixes-3): owner pass fixes: AN-16 stride (SpeedMatch `rate_share` 0.3), AN-17 loop-seam contact gap, AN-19
  footskate cleanup in the cook (`anim.cook` v8), AN-21 slope stride + lean (`GRAPH_FORMAT` 6), AN-18/20 ragdoll mass and
  inertia conditioning + anatomical cones (`ragdoll.cook` v5); smooth bodies off by default. All confirmed by the owner.
- 2026-10-22 (a2-slice1): `sf_action_bevy`: chooser + batched mover on a 60 Hz `FixedUpdate`, `WorldPos` interpolation,
  render-body correction spring (`sf_controller::RenderCorrection`), `bevy_enhanced_input` sampling; ticks identical at 30/60/144 fps.
- 2026-10-22 (anim-fixes-2): AN-9 heel-to-toe foot lock (cooked roll curves), AN-10 face-down get-up roll axis,
  AN-13 creature knockout exit, AN-14 death during knockdown stays dead; AN-11 spine limits 30 to 15 degrees a segment (run 2); AN-12/15 open.

- 2026-10-22 (anim-refactor run 3): owner pass AN-1..8 at the root: capsule targets per physics step, A2 feeds
  compose root deltas, footsteps from clip keys, constant-decel arrival, in-place get-up clip + hand-back blend, creature shapes from bounds.
- 2026-10-22 (anim-refactor): `anim-lab` test level (AL-1..7); R0 ragdoll write-back (every joint at weight 1,
  frame follows pelvis); idle/crowd variety; foot metrics, strafe/back slide 0; feet on ramps; R2 reaction controller.
- 2026-10-21 (owner pass, bw-owner10): confirmed LT-1, LM-1, AU-1, AU-2, SH-17, SH-18, TA-1, PA-4, PC-1, DR-3, DR-4,
  SK-1, PH-6; FG-1 not reproduced; WE-5 cause found; SH-16/19 need a console; new bugs in 1.12.
- 2026-10-21 (action-a1): mover launch (no ground snap while rising, head bump; 4 failing-first tests), serde
  `Locomotion`, `PlayerId` merged into new `sf_ids`; A2 prerequisites done.
