# Roadmap

Live plan and status: short on purpose. **Rewritten 2026-10-19 (phase0)** after the owner's audit: the work is
ordered in **phases**, not milestones. Milestone numbers (M2–M10, ARCHITECTURE §9) stay as scope labels for specs.
Everything before 2026-10-19 (as-built notes, measurements, the session log) is in
[`archive/ROADMAP-to-2026-10-19.md`](archive/ROADMAP-to-2026-10-19.md): grep it, don't read it whole.

**Rules for this file.** At most ~250 lines. One entry per task, <= ~400 bytes: status mark, ID, what, where the detail lives
(spec, visual check, archive). As-built goes to the spec or `archive/ROADMAP-2026-10-19-to-10-25.md`; Recent lives in `CHANGELOG.md`.
A task is `[x]` only when its Verify ran; visual tasks need the owner's eyes (`visual-checks.md`).
Legend: `[x]` done, `[~]` started, `[ ]` not started, `[p]` parked (needs the owner's go to resume).
How things work: `docs/wiki/index.md`; what was tried: `docs/ledger/`.

**Untested (2026-11-08/09 laptop days):** every change dated 2026-11-08 or 2026-11-09 is unverified in game: nothing was looked at in game; the gameplay crates compile and pass unit tests and clippy since 2026-11-09. Owner test pass 2026-11-10: [`test-pass-2026-11-10.md`](test-pass-2026-11-10.md).

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
- [~] 0.4 Specs over ~25 KB split into design + short "As built"; history to `archive/` next to them. Doc audit S5-S14 (2026-10-26) did weather, locomotion, bake, mods and trimmed others; still over 25 KB: streaming, animation, physics, water, industry-audit, character, scripting, ui, action, net, sky-lighting, save. Do each when its phase starts.
- [x] 0.5 `portal_spike.rs` + its shader, `--portals` flags and the 9 `portal-*` scenes deleted (owner go 2026-10-19;
  git history keeps them, design and results in `superfargoth/docs/specs/portals.md`; `mw_world::portal` stays).

## Phase 1: performance, stability, image quality

Goal: the frame target (decision 1, raster path) in Balmora, Seyda Neen and the flight bench, with clean, grain-free shadows
and effects. Every task reports its ms cost per preset.

- [~] 1.1 **Stability first**: DR-4 crash fix, AU-1 indoor weather sound, door fade hold built (owner: DR-4, AU-1); reveal gate (pipeline warm-up W1) holds fades/loading for textures, assets, pipelines, 10 s cap (owner: RG-1). Wiki `docs/wiki/engine/streaming.md`.
- [~] 1.2 **Frame budget + gates**: all benches ≤ 11 ms 2026-10-28 (balmora 9.48, seyda_neen 9.95, red_mountain 9.00, balmora_walk 10.50; flight 11.10); left: per-preset table, `flight` origin-shift spikes. `docs/ledger/perf.md`.
- [~] 1.3 **Temporal resolve base**: Bevy TAA when DLSS off (MSAA Low only), F4 cycles (owner: TA-1, WA-1 moiré); RO-6 sweep done; open: translucent motion vectors (RO-7). `docs/wiki/engine/frame-and-views.md`.
- [~] 1.4 **One shadow design, finished**: PCSS, SH-11 ordering, SH-20 mover shadows, 2026-11-01 audit fixes built (owner: SH-11, SH-14..20, SH-21, RO-11); open: High lamp count (owner pick), SH-13, audit §5 items 4-7, RO-12. `docs/wiki/engine/shadows.md`, `docs/ledger/shadows.md`.
- [~] 1.5 Ambient occlusion: Bevy GTAO per preset (Low off); look awaiting owner (SH-16).
- [~] 1.5b **Shader/pipeline precompilation**: W0 measure + W1 reveal gate built (owner runs pending: W0, RG-1); next W2-W6; spec `superfargoth/docs/specs/pipeline-warmup.md`, research `research/shader-precompile-2026-11-11.md`.
- [x] 1.6 Object-terrain blending removed (2026-10-20); redo is eye candy (WISHLIST §13).
- [~] 1.7 Night brightness: ambient retuned (LT-1 confirmed); LM-2 vertex lighting + 16-bit DDS cutouts built (owner); left: SK-2, NS-2..4. `docs/wiki/engine/lights.md`.
- [~] 1.8 Draw-order/far-field: WE-5 storm occlusion + cloud cap, FF-2 LOD tint built (owner: WE-5, FF-2); open: Ghostfence far copy (GF-1, B5).
- [~] 1.9 Streaming/culling levers landed 2026-10-28 (visibility ranges, outfit AABBs, `idle_skip_a3`, walker radius, material dedup); open: precombine default (owner call, J6); checks ST-2, ST-3, SC-2. `docs/wiki/engine/streaming.md`, `docs/ledger/streaming.md`.
- [~] 1.10 Volumetric fog black bands (FG-1): not reproduced 2026-10-21; reopen if seen; `--off foghistory` A/B kept.
- [x] 1.11 Thin binary: app split slices 1-14a + 15 game crates, binary 2.3k lines (2026-10-26); `specs/app-split.md`.
- [~] 1.12 Owner pass bugs built, awaiting owner: LT-4, SH-20, RO-B, TR-3, WE-5, DL-5, RO-16 (terrain "veil"). Logged, not diagnosed: RO-11, RO-12, PH-12, PH-13; not urgent: cap ash shelter distance.
- [~] 1.13 Dev console (`sf_console_bevy`, `console.md`): Backquote, `ssao`/`contactshadows`, MWScript, `TCL`, `COC`, paste built (owner: CO-1); left: more `--off` parts, TGM/TAI/TFOW/TWF/COE, `set x to y`, click-to-pick a reference with visible selection and `kill`/`ref->` (owner 2026-11-05).
- [ ] 1.13b Faster time-lapse preset (owner 2026-11-12, during RO-21): a faster speed step than T today for weather and day-cycle checks; dev tools `mw_devtools_bevy`.
- [~] 1.14 Build churn: Bevy no longer rebuilds per `-p` (hakari, BS1c); left R1-R3 core splits, one test binary per crate, stale `debug/deps`; hot-crate gate (8 s) missed, fix = crate graph (split `sf_model`). `superfargoth/docs/build-perf.md`.
- [ ] 1.15 Godotwind borrow pass (owner 2026-10-26, its own pass): `research/godotwind-borrow.md`; first entry FFT-to-analytic shore waves.
- [p] 1.16 **Static scene** (owner go 2026-11-08; **frozen by owner 2026-11-12**: parity only, statics worth ~1 ms of the walk, duplicates Bevy retained bins + GPU culling per the architecture review; code kept, `--scene` opt-in; delete later unless a measurement beats the precombine): S3b code complete; `--scene` A/B still ~parity (11.23 -> 9.44 ms standing, 14.50 -> 11.10 walking vs old 9.31 / 10.71); ceiling: statics cost 2.0 / 3.7 ms. Owner: SC-scene, SC-lamps, call continue (fix 4, S3a3) or freeze; interiors table? `docs/wiki/engine/statics-and-lod.md`, `docs/ledger/statics.md`, spec `static-scene.md`.
- [~] 1.17 **One authoritative game clock** (owner 2026-11-09): C1+C2 built, owner check pending; next C3 calendar globals; replaces `SkyClock`/`GameTime`/`GameMinutes`. `superfargoth/docs/specs/clock.md`.
- [~] 1.18 **IK keeps running while paused**: P-IK2 built (run conditions, `AnimateWhilePaused`); PL-8 awaiting owner, then [x]. `superfargoth/docs/research/ik-feel-diagnosis-2026-11-11.md`.
- [ ] 1.19 **Streaming Upload budget multiply-charges a multi-frame stage** (latent, not live; `sf_world_bevy/src/stream.rs:814`); fix when any stage first spans frames. `docs/wiki/engine/streaming.md`.
- [~] 1.20 **Texture compression + streaming** (owner 2026-11-11, urgent): T0-T4 built (KTX2 cook, remap, normal maps, tangents); TS-1a per-source profiles, TS-1b source formats, TS-2 `_spec` ORM built (owner 2026-11-13: any source); 2026-11-13: tangent handedness fix, T4d statics POM, T5 terrain normals + ARM, T6 BC7/BC5 2048² layer arrays, S1 load cap, S2 rebind, S3a planner, S3b demand data, runtime gauge, `level:texture-lab` + `--off`/`flip` normalmaps/orm/parallax built, need a full re-bake; TX-1, TX-2, TX-3a, TX-4, TX-5, TX-lab awaiting owner; logged: actors get no normals/ORM, non-x4 sizes to RGBA8, `.dds` vs `.tga` precedence, Bevy duplicate ktx2 loader warning, flaky `mw_bake` `changing_a_mods_profile_recooks_only_its_textures` (built counts sf_bake duplicate resolves: Meta/Index steps race to the same cook product, ~1 in 6 runs); owner 2026-11-13: TX-2 confirmed; normals too faint, POM stepped layers, landscape rocks wrong (~2600 normal maps without tangents), texture swimming on some meshes (mushrooms), flip normalmaps skips terrain; wants parallax as a player toggle and a selectable texture pack (spec first); queue in `.sessions/tex-audit.md` HANDOFF 2; then S4 streamer, terrain parallax. Specs `superfargoth/docs/specs/textures.md`, `textures-adapter.md`; `docs/wiki/engine/textures.md`.

**Next session order (owner 2026-10-26):** the Phase 2 build queue is done (main menu, new game, Continue/Load, stack counts, gold, container window, looting). Next: the owner's big debug pass (visual checks incl. SV-2, HD-1, CO-1, PM-1, PH-11, MM-1, IV-2; Phase 2 done test end to end), then Phase 1 perf, then Phase 3 (combat first; 3.6/3.7 water). Godotwind borrow pass (1.15) is its own pass.

## Phase 2: a playable vertical slice (the basics)

Done when: new game in Seyda Neen → walk in third person → talk to Fargoth → get his ring back → journal entry →
save, quit, load. Break it down (and write the missing specs: dialogue, journal, saves) before building.
Dev mode (owner 2026-10-26): dev runs (`view`, `scene`, `cargo dev`) always skip the main menu and character creation and start
in the world; only the release start path (or a session working on the menu) shows them. Character creation is not in Phase 2.

- [~] 2.1 Player controller + camera + use prompt (`specs/action.md`, `camera.md`): works (PL-1 confirmed); IV-1 loose items, IV-2 container window awaiting owner; left: ownership/theft, container weight/locks. `docs/wiki/engine/characters.md`.
- [~] 2.1b Input: `controls.rs` built; owner: everything HOLD by default, per-action hold/toggle option.
- [~] 2.2 UI (`sf_ui*`, `sf_rpg_ui`): kit, theme, HUD, dialogue/journal/inventory, pause + options, front end (no world behind menu, Bink backdrop, Continue/Load, intro logos) built; owner: PM-1, HD-1, LK-1, LK-2; classic look = OpenMW exactly, a big pass: L3 pause, L4 options, L5 keybinds, L6 save/load, L7 art/HUD, new-game video. `docs/wiki/engine/ui-and-menus.md`, `superfargoth/docs/specs/menus.md`.
- [~] 2.2b Options menu (owner 2026-11-05): settings S1a-S1c built, rows on `bevy_ui_widgets` (OP-1 awaiting owner); left S2b-S2c, S3-S5 pages. `superfargoth/docs/specs/menus.md`.
- [~] 2.3 Text: framework slices 1-2 built; left: Morrowind markup parser (`mw_ui::book`), MessageBox formatting, both untested. `ui-text.md`.
- [~] 2.4 Dialogue + journal + quest store + MWScript runtime (W3): Fargoth ring flow confirmed; next HUD. `docs/wiki/engine/rpg-and-scripting.md`.
- [~] 2.5 Saves (`save.md`): F5/F9, save/load screen (SV-2 awaiting owner); cross-review C1-C10 fixed; owner decides CH1 snapshot budget, D1-D4; left: typed names, thumbnails, place label, corrupt fallback. `docs/wiki/engine/saves.md`, `docs/reviews/saves-2026-11-01.md`.
- [~] 2.6 Sound: cues, ambience, music, footsteps, voices, mixer built; AU-1/2 confirmed, AU-3..5 awaiting owner; left: UI sounds, voice LOS + barks, modern beds.
- [~] 2.7 **OpenMW content mechanics** (owner 2026-11-05): stats S0-S7, effects E1-E4b + A1-A5a, services SV1-SV3 + TR1/TR2 + training A3 built, mostly headless/untested in game (HD-2, PUI-SV3, PUI-SV4 awaiting owner); next A5b cast input + HUD spell, A3b, E5, travel/spells, repair, alchemy, enchanting, spellmaking. Specs `stats.md`, `effects.md`, `services.md`, `crafting.md`; `test-pass-2026-11-10.md`; `docs/wiki/engine/rpg-and-scripting.md`.
- [ ] 2.8 **Crime, stealth, locks**: spec `crime.md` LW1-LW9 + adapter A0-A8; owner questions pending; rules-only slices first.

## Phase 3: animation, combat, physics

- [~] 3.1 Traversal/action (R11, `specs/action.md`): mantle (AL-10), lip IK, wall brace/stop (AL-11 awaiting owner), vault (AL-12) built; left: vault hiccup/stall, run + Ctrl slide (later), MotionBricks eval. Clips: packs only. `docs/wiki/engine/locomotion-and-action.md`, `docs/ledger/animation.md`.
- [~] 3.2 Ragdolls: Delirium default feature, human knockdowns/deaths via `RagdollTakeover` (PH-14, PH-15 confirmed); clip+limp deprecated; left: strength-scaled fall threshold, corpse drag/saves, beasts, limp_drop/stairs gates, PH-9. `specs/animation-plan.md`.
- [p] 3.2b Delirium experiment: DE-1 confirmed; open-source replacement weights for the GTA IV ones coming "very soon" (owner 2026-11-12), adopt when they land (owner 2026-11-11); logged: crouch-landing slide, player/dummy takeover hold.
- [~] 3.3 Physics follow-ups: fix plan slices 1-10 built (PH-11 awaiting owner), slice 11 player capsule waits for owner; held object vs capsule, props jump on room entry. `specs/physics-fix-plan.md`.
- [~] 3.4 Deferred character checks (CH-1/3, SB-5, AL-9, PL-3 awaiting owner); IK on player: P-IK1 built (PL-7 failed), P-IK5 done (PL-9 confirmed), P-IK3, P-IK4, P-IK6 wait for 3.4c A3 (no slope-op tuning on clips about to be replaced), P-IK1c = 3.4c A1; AN-21 confirmed. `ik-feel-diagnosis-2026-11-11.md`; `docs/wiki/engine/animation.md`.
- [~] 3.4c **Locomotion rebuild** (owner 2026-11-12: modern preset gets generated walk/jog/sprint + starts, stops, pivots, turns in place; Morrowind clips stay for Classic): plan A0-A6, B0-B6 then C, D; A2+B3a gait card done (no clip fits the walk; owner: Quaternius not good, generate first): Kimodo batch 1 (15 clips) before A3; A1 measured (clip speed and walk<->run phase reset confirmed; no lane bleed: turns are an 83-250 ms body spin, A5 judged on that; no rate jitter on flat, A6 dropped unless slopes show it); A0 built (`off speed_match / stride_warp / foot_placement / lean / side_lanes`, WALK-0 awaiting owner); B3b batch 1 generated (228 clips, 38 groups; LOCO-2 awaiting owner picks, browser has Y/N/M keys; trunk retarget fixed, shoulder offsets building; grader AG1 `motiongen grade` done (R17), next AG2 browser sort, AG4 arm check on the retargeted clip; skate must be measured after the cook); prompting guide R19 (batch 2 starts with a path/CFG sweep); sprint (owner): lower the speed + retime a free sprint, and look for a free sprint clip (100STYLE); loop cutter built + previewed (LOOP-1 confirmed); A3 built (synced walk/jog/sprint gait set on loco1 cycles walk.free.05 / run.3.04 / sprint.1.05, loop-cut, rate clamp 0.8-1.25, sync-group join across states, GRAPH_FORMAT 11, Classic = old graph; full-stick rates 0.92-1.00, switch phase jump 1 frame; WALK-1 awaiting owner); next: retimed free sprint A/B at a lower sprint speed (owner), A4; logged: in-game right-thigh snap (owner), retarget ruled out (`mw_bake/tests/thigh_snap.rs`), suspect a montage/transition with no blend-in from the MW idle (right knee 14 vs 5 deg). `superfargoth/docs/research/animation/15-audit-and-plan-2026-11-12.md` §5; `docs/ledger/animation.md`.
- [p] 3.4d **Locomotion styles** (owner idea 2026-11-12): generated style sets on the 3.4c base, picked from data: gender, vibe (aggressive/timid, e.g. from AI Fight/Flee), over-encumbered, injured/sick; balance conditions (drunk stumble, limp falls) go to Delirium (3.2b). Seam now: A3 looks up `loco.<style>.<gait>.*` with fallback to the base set; generate after the base set is signed off.
- [~] 3.5 Combat (M10, Elden Ring feel, classic seam): C1a-C1c, C2a-b, C4/C4b built; CB-3a confirmed, CB-3b partial (jumpy guard loop, stamina refill too fast to test the break); awaiting owner CB-2, CB-5, CB-5b; next C2c backstep + knockdown; owner 2026-11-11: Quaternius clips bad, try Morrowind attacks, Kimodo pass (3.8); hurt shapes `character-shapes.md` S0/S3 built; left: equip screen, moveset by weapon, MG-C1. `specs/combat.md`, `docs/ledger/combat-feel.md`.
- [~] 3.4b First-person arms (owner 2026-11-05): FP-1, FP-1b, FP-2a/b built (awaiting owner FP-1, FP-1b, FP-2, CB-5b); next FP-2c, FP-3 held items. `superfargoth/docs/specs/first-person.md`.
- [ ] 3.6 Swimming + underwater (owner 2026-10-26): waits for the 3.7 design (water at any altitude, owner 2026-11-05). `water.md`, `docs/wiki/engine/terrain-and-water.md`.
- [ ] 3.7 Buoyancy (owner 2026-10-26): pontoon bodies, drag, thrown items, corpses (`water.md`).
- [~] 3.8 Ambient life clip factory (owner 2026-10-28): MG1-MG6, MG7a, MG10a done, tier1 268 clips; next MG10b, MG7b, MG1b, MG8. `specs/motion-generation.md`.
- [~] 3.9 **NPC navigation** (M7): NV0-NV4b, NV4d, NV4f done (154k tiles baked); next warm re-measure, NV4c + NV-1, NV5-NV12; SI3 before NV5; NV7a+NV8 gate AI6. `specs/nav.md`.
- [~] 3.10 **NPC AI** (M7): AI1-AI5 built headless, not wired into the game (`docs/briefs/ai-next.md`); AI6 blocked on NV7a/NV8/SI3, AI7 needs no nav. `specs/ai.md`, `ai-as-built.md`.
- [ ] 3.11 **World interactivity** (owner 2026-11-11): crates, buildings, chairs, grabbing, mounts; owner answered; next IN0 core, IN1 crates break. `specs/interaction.md`, `world-reactions.md`.

## Phase 4: eye candy and the wider world (after phase 2)

- [~] Weather polish (`surface-weather.md`): W1-W5a, W6b built, SW-2/3/4 and WE-8 confirmed; SW-6 awaiting owner, W7 next; open: cascade-0 shadows, POM; storm from Red Mountain (WE-5), tall clouds, ash, PR2, SU1/SU2, WD2/WD3. `docs/wiki/engine/sky-atmosphere-weather.md`.
- [p] Water polish (WA-1, WISHLIST §12; **owner 2026-11-12: "really look into the water rendering when we have time"**, so it is the next eye-candy item once Phase 2/3 allow): shore swash, foam, whitecaps, wave height, crest SSS, W4 reflections, caustics, W7 rivers.
- [p] RT as the Ultra tier (decision 2): exterior RT, Solari sunset patch, `--rt` streamed view, characters, DL-4, DL-2.
- [~] GI (owner 2026-11-01): ray-query probe volumes, slices 1-3 built (0.025-0.08 ms); verification pending. `superfargoth/docs/specs/gi.md`.
- [ ] Light rules from openmw-rtx (owner 2026-11-05): L-A built (1.5x rejected, default 4x), L-B built (GI-lampbody awaiting owner); later L-E, L-F.
- [~] Albedo de-lighting at bake (owner 2026-11-05): D-1, D-2a/b/c built, on by default at 0.5, `--off delight`; TX-delight awaiting owner.
- [x] Bloom on the game camera (owner 2026-11-01): `Bloom::NATURAL` 0.1, Medium+, `--off bloom` (RO-14 awaiting owner).
- [x] Foliage light transmission (WISHLIST §11): `--off foliagetransmission` (RO-15 awaiting owner).
- [p] Cloth: capes/banners/tents (PH-3 confirmed).
- [p] Crowd LOD via vertex animation textures (`bevy_open_vat`, owner 2026-10-25): spike one creature first.
- [p] Seamless portals: M9a player (SI0-SI11), M9b world (SI12b, NV11, AI12/MA9, AI10, SI16); far-field M8.
- [p] Co-op N1–N8 (`specs/net.md`; N1–N5 after this phase, N6–N8 after M7).
- [p] Lua/Luau beyond L3; mods demo (M5 E1, U7, X1, X2); rule hooks `rule-hooks.md` (Rust first, Luau later, owner 2026-11-08).
- [p] Crate "update all" pass once `bevy_hanabi` supports Bevy 0.20.
- [p] Classic option (owner 2026-10-20/23, ARCHITECTURE §1b): classic graphics preset; classic rules (`mw_rules`) behind the seams.
- [p] Setup screen (owner 2026-10-26, `specs/setup.md`): first-run Morrowind path, mods, downloads, bake queue.
- [p] Living world AI (owner 2026-10-28, `specs/ai.md`; built part in 3.10): Nemesis-like rivalries first (AI13).
- [p] Groundcover (owner 2026-10-20, low priority): adopt `bevy_grass`, masks from LTEX, wind `sf_wind`.

## Recent

One line per session in [`CHANGELOG.md`](CHANGELOG.md) (newest first); older history in `archive/`.

