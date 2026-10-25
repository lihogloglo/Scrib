# Roadmap

Live plan and status: short on purpose. **Rewritten 2026-10-19 (phase0)** after the owner's audit: the work is
ordered in **phases**, not milestones. Milestone numbers (M2–M10, ARCHITECTURE §9) stay as scope labels for specs.
Everything before 2026-10-19 (as-built notes, measurements, the session log) is in
[`archive/ROADMAP-to-2026-10-19.md`](archive/ROADMAP-to-2026-10-19.md): grep it, don't read it whole.

**Rules for this file.** At most ~250 lines. One entry per task, <= ~400 bytes: status mark, ID, what, where the detail lives
(spec, visual check, archive). As-built goes to the spec or `archive/ROADMAP-2026-10-19-to-10-25.md`; Recent lives in `CHANGELOG.md`.
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
- [ ] 0.4 Specs over ~25 KB split into design + short "As built"; history to `archive/` next to them. Do each when its phase starts (weather, animation, physics, water, locomotion, character, streaming, ui, scripting).
- [x] 0.5 `portal_spike.rs` + its shader, `--portals` flags and the 9 `portal-*` scenes deleted (owner go 2026-10-19;
  git history keeps them, design and results in `superfargoth/docs/specs/portals.md`; `mw_world::portal` stays).

## Phase 1: performance, stability, image quality

Goal: the frame target (decision 1, raster path) in Balmora, Seyda Neen and the flight bench, with clean, grain-free shadows
and effects. Every task reports its ms cost per preset.

- [~] 1.1 **Stability first.** DR-4 crash fix, indoor weather sound (AU-1), door-travel fade hold built; awaiting owner (DR-4, AU-1). Owner ask 2026-10-25: hold the fade until the interior is fully loaded (meshes + textures); objects still pop in.
- [~] 1.2 **Frame budget + gates.** Median 11 ms gates, budget table and A/B in `perf.md` "Frame budget"; render-thread bound. Next: merge statics per cell at bake time, fewer shadow views, Nsight per pass + Tracy per system, per-preset budget table, `benchmarks/exterior_*.json` median gate at target.
- [~] 1.3 **Temporal resolve as the base.** Bevy TAA default when DLSS is off (MSAA on Low only), F4 cycles; awaiting owner (TA-1). Open: motion vectors for translucents (RO-6/RO-7). Fixes grain WE-2/WE-3, SH-12, WA-1.
- [ ] 1.4 **One shadow design, finished.** Stock Bevy shadows + vendored PRs #26149/#26161 (decision 3); resets A1/A2/B1/B2 built, awaiting owner (SH-14..SH-19). Open: lamp-shadow count for High (owner pick), LM-1 leaf shadows, RO-1 translucent casters, SH-11, SH-13; keep SH-7/8/10. Specs `superfargoth/docs/specs/shadows.md`, `lights.md`; `research/shadows/README.md`.
- [~] 1.5 Ambient occlusion: Bevy GTAO per preset (Low off); look awaiting owner (SH-16).
- [x] 1.6 Object-terrain blending removed (2026-10-20, lights1); redo is eye candy (WISHLIST §13).
- [~] 1.7 Night brightness: ambient retuned, LT-1 confirmed 2026-10-21. Left: deferred night checks SK-2, NS-2..4.
- [ ] 1.8 Draw-order and far-field bugs: ash/blight storm drawn over terrain and objects (WE-5; next A/B `--off clouds|fog|froxels`); Ghostfence far copy missing (GF-1, B5); far-field L2/L3 never selected/tinted (FF-2).
- [~] 1.9 Streaming and culling levers (archive "Next up" 1, 6, 7): render-thread spikes, `NoCpuCulling` retry (J2), caster rules (J4), warm-up (J7), precombine (J6, `superfargoth/docs/specs/precombine.md`, slices 1-3 built; default on/off is the owner's call); checks ST-2, ST-3, SC-2.
- [~] 1.10 Volumetric fog black bands (FG-1): not reproduced 2026-10-21; reopen if seen; `--off foghistory` A/B kept.
- [~] 1.11 Thin binary (ARCHITECTURE §8b rule 4): the app is 13.5k lines and `exterior.rs` is the most-edited
  file; move systems into plugin crates. Re-measure the dev-loop gates (last measured 10-12, half the crates). Plan `specs/app-split.md`: slice 1 (`mw_game_core`) done, next `mw_boot_bevy`.
- [ ] 1.12 Owner pass 2026-10-21 bugs: LT-4 lamp light cut at cluster edges (`lightclusters`); SH-20 choppy character shadows (`charshadows`); RO-B smoke over sky (`smokeorder`); TR-3 glossy terrain (awaiting owner); WE-5 ash occlusion (`--off precipocc`). Owner 2026-10-25, not urgent: cap ash shelter distance (a few m behind a vertical object) + steeper angle.
- [~] 1.13 Dev console (`sf_console_bevy`, `console.md`): Backquote, live `ssao`, MWScript lines, `TCL`, `COC` built; CO-1 awaiting owner. Left: contact shadows live (SH-16), other `--off` parts, tab-complete, console functions (TGM/TAI/TFOW/TWF...), `set x to y`, exterior `coc`.
- [~] 1.14 Build churn: workspace-hack (`cargo hakari`) built, covers the merged workspace; left: re-measure dev-loop gates incl. the app.

**Next session order (owner 2026-10-20):** 1.4 shadows, then FG-1 (1.10), then owner checks PA-4, DR-4, AU-1, TA-1; Nsight per pass (UAC).

## Phase 2: a playable vertical slice (the basics)

Done when: new game in Seyda Neen → walk in third person → talk to Fargoth → get his ring back → journal entry →
save, quit, load. Break it down (and write the missing specs: dialogue, journal, saves) before building.

- [~] 2.1 Player: third-person controller + camera + use prompt (`specs/action.md`, `camera.md`; `sf_action`, `sf_camera`, `player.rs`); works in game (owner 2026-10-25), PL-1 confirmed. Items: E takes loose items (`inventory.rs`), IV-1 awaiting owner. Owner notes (log only): too much inertia; carried object sinks the capsule (3.3). Left: container window, ownership/theft, stack counts.
- [~] 2.1b Input (with 2.2 U6): `controls.rs` built, menu works; labels in `l10n/en/bevyrrowind.ftl`. Owner: everything HOLD by default (sprint holds, sneak toggles today): per-action hold/toggle option.
- [~] 2.2 UI (`sf_ui*`, `sf_rpg_ui`): kit, Morrowind theme, HUD/message box, dialogue/journal/inventory screens built. Next: Escape menu + options (queued), 8-piece frames, inventory icons, theme hot reload.
- [~] 2.3 Text: fonts, books and scrolls, notifications. Framework slices 1-2 built 2026-10-23 (`ui-text.md`); left:
  the Morrowind markup parser and MessageBox mapping.
- [~] 2.4 Dialogue + journal + quest store (`specs/rpg.md`, `rpg-adapter.md`; `sf_rpg`, `mw_rpg`, `dialogue.rs`), MWScript runtime (W3). Fargoth ring flow confirmed by owner. Next: HUD.
- [~] 2.5 Saves (`sf_save*`, `saves.rs`, `save.md`; net-ready `specs/net.md`): F5/F9 quicksave/load works headless; SV-1 awaiting owner. Left: part 2 (load screen, save list, naming).
- [~] 2.6 Sound (`sf_audio*`, `mw_audio`, `audio.rs`): cues, ambience, music, footsteps, voices built; AU-1/2 confirmed, AU-3..5 awaiting owner. Left: impact sounds, volume mixer (with U6), UI sounds, voice LOS + barks (Phase 3), modern-preset beds.

## Phase 3: animation, combat, physics

- [ ] 3.1 Traversal/action per R11 decisions (`research/animation/11-tradeoffs.md`, `specs/action.md` phases);
  old P11. Clip sources: packs only (free first).
  Potential (owner 2026-10-22): evaluate MotionBricks as an animation source (licence, rig fit, quality vs packs).
- [~] 3.2 Ragdolls: R0 fixed, AN-1..21 fixed, PH-2 awaiting owner (`level-anim-drop`). Left: limp_drop/stairs stretch gates, beast toe pin, powered falls (PH-9). Order `superfargoth/docs/specs/animation-plan.md`; test level `anim-lab`.
- [p] 3.2b Potential (owner 2026-10-22): delirium-physics active-ragdoll experiment, bevyrrowind only (slice R9 in
  `superfargoth/docs/research/animation/12-delirium-physics.md`). Fan project: accept the provenance risk, remove if challenged.
- [ ] 3.3 Physics follow-ups: hold/drag/throw (PH-4), shapeless items (PH-7), immovable candles (PH-1), ball through walls (archive 6a). Owner 2026-10-25: props jump on entering a room (spawn settling); held object vs capsule.
- [~] 3.4 Deferred character checks: SN-2/3, CH-1/3/4, SB-1/2/3/5, HI-3, AP-2/3, PH-10 (CH-1/3, SB-5 awaiting owner); next stairs 5b, powered falls R3 (`animation-plan.md`).
- [ ] 3.5 Combat (M10). Owner 2026-10-22: not Morrowind's rules; evaluate Funny-Bones/ELDEN-RING-Playermodel-Rust (MIT code, clips not shippable) as base for action.md E1; keep a hit-resolution seam (ARCHITECTURE §1b), classic swing later.

## Phase 4: eye candy and the wider world (after phase 2)

- [p] Weather polish: storm from Red Mountain (WE-5), tall clouds, ash, splashes (PR2), wetness/snow (SU1, SU2), tree wind (WD2, WD3). Owner 2026-10-21/22: cheap rain puddles + wet objects, persisted in saves, SSR reflections first (`research/borrow-audit-2026-10.md`).
- [p] Water polish (WA-1 notes, WISHLIST §12): shore swash + gradual wavelets, textured foam, whitecaps,
  weather-driven wave height (verify), crest SSS; W4 reflections, W5 underwater, W6 buoyancy, W7 rivers.
- [p] RT as the Ultra tier (decision 2): exterior RT content, Solari sunset patch, `--rt` in the streamed view, characters in the RT scene, DL-4 contact bands, DL-2 open (needs a `solari,dlss` build).
- [p] Raster GI & sky occlusion (owner 2026-10-21): Bevy 0.20 `IrradianceVolume`/sky-visibility probes baked per cell in `sf_bake` plus screen-space GI/bent normals.
- [p] Foliage light transmission (WISHLIST §11).
- [p] Cloth: capes/banners/tents (robes stay authored skin; PH-3 confirmed).
- [p] Crowd LOD via vertex animation textures (owner 2026-10-25: adopt `bevy_open_vat`; `research/crates-2026-10-25.md`): tier below anim LOD 3; spike one creature first.
- [p] Seamless portals (M9), far-field performance (M8: HLOD, texture streaming, PBR generation).
- [p] Co-op N1–N8 (`specs/net.md`; N1–N5 after this phase, N6–N8 after M7).
- [p] Lua/Luau beyond L3; mods milestone demo (M5 E1, U7, X1, X2).
- [p] Crate "update all" pass once `bevy_hanabi` supports Bevy 0.20 (archive Backlog).
- [p] Classic option (owner 2026-10-20/23, ARCHITECTURE §1b): (a) Classic graphics preset on the `--off` switches; (b) classic rules (`mw_rules`, from OpenMW) behind the feel-system seams, after each modern version exists.
- [p] Groundcover (owner 2026-10-20, low priority; 2026-10-25 adopts `bevy_grass`): masks from LTEX, wind from `sf_wind`; later load Morrowind groundcover mods (`research/crates-2026-10-25.md`).

## Recent

One line per session in [`CHANGELOG.md`](CHANGELOG.md) (newest first); older history in `archive/`.

