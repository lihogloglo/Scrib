# Roadmap

Live plan and status: short on purpose. **Rewritten 2026-10-19 (phase0)** after the owner's audit: the work is
ordered in **phases**, not milestones. Milestone numbers (M2–M10, ARCHITECTURE §9) stay as scope labels for specs.
Everything before 2026-10-19 (as-built notes, measurements, the session log) is in
[`archive/ROADMAP-to-2026-10-19.md`](archive/ROADMAP-to-2026-10-19.md): grep it, don't read it whole.

**Rules for this file.** At most ~250 lines. One line per task: status mark, ID, what, where the detail lives
(spec, visual check, archive). As-built detail goes in the spec's "As built" part and the commit message, not here.
A task is `[x]` only when its Verify ran; visual tasks need the owner's eyes (`visual-checks.md`).
Legend: `[x]` done, `[~]` started, `[ ]` not started, `[p]` parked (needs the owner's go to resume).

## Owner decisions open

1. **Frame target** (proposal): High preset, 1440p + DLSS Quality on the owner's laptop (Ryzen 7 7435HS,
   RTX 4060 Laptop): ≤ 11 ms median exterior (~90 fps), stretch 7 ms (~140 fps). Today ~30 fps.
2. **Ray tracing role** (proposal): raster is the main path at the frame target; Solari is the "Ultra" tier
   (60 fps target, WISHLIST §0). Alternative: RT-first with raster as fallback.
3. Park the far static sun map (`FAR_STATIC_SHADOWS` off) and the point-shadow cube cache slice 2b
   (built, no gain): proposal **park both** (phase 1.4 picks one shadow design).
4. Older, low priority: storm spill past region borders (~2 km vs strict); co-op guest imports a single-player
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
- [ ] 0.5 `portal_spike.rs` (53 KB, parked until M9) out of the default build (feature `portals`) or deleted.

## Phase 1: performance, stability, image quality

Goal: the frame target (decision 1) in Balmora, Seyda Neen and the flight bench, with clean, grain-free shadows
and effects. Every task reports its ms cost per preset.

- [ ] 1.1 **Stability first.** DR-4 crash (command on a despawned entity on leaving an interior): streamed
  entities via `try_insert`/`queue_silenced`; shipped builds set Bevy's error handler to log, dev keeps panicking.
  Weather/region sound plays indoors (AU-1). Door travel: black fade held until the destination's near ring is
  spawned (timeout), no rough far-field shapes or loading objects on arrival (DR-4).
- [ ] 1.2 **Frame budget + gates.** Nsight per pass + Tracy per system on the owner's laptop; a budget table per
  preset in `perf.md`; `benchmarks/exterior_*.json` get a median gate at the target (they only checked 33 ms, so
  the fall from ~11 ms on 10-12 to ~30 fps never failed). Then cut whatever is over budget.
- [ ] 1.3 **Temporal resolve as the base.** Bevy TAA on the raster path when DLSS is off (Medium+), DLSS when on;
  every pass writes motion vectors (render-order.md RO-6/RO-7); MSAA off where TAA replaces it. Fixes the grain
  in weather/fog/soft shadows (WE-2, WE-3), grazing-light edge shimmer (SH-12), water moiré (WA-1).
- [ ] 1.4 **One shadow design, finished.** Sun: cascades + contact shadows + terrain far shadow (keep SH-7/8/10).
  Fix distant flashes on camera move (SH-11); lamp shadows: short reach, grain, flicker, wrong guard outline
  (SH-13); leaf shadows on some trees only (LM-1); translucent casters (RO-1). Park the rest (decision 3).
  Spec: `superfargoth/docs/specs/shadows.md`, research `research/shadows/README.md`.
- [ ] 1.5 Ambient occlusion: Bevy's SSAO (GTAO), per preset.
- [ ] 1.6 Object-terrain blending redo (TB-1..3 rejected: reads as a darkened base). Standard technique: terrain
  albedo/normal blended onto the mesh base by height above the terrain; AO does the contact darkening.
- [ ] 1.7 Night brightness (LT-1 rejected: "can't see anything"): exposure floor + night ambient retune; then
  the deferred night checks (SK-2, NS-2..4).
- [ ] 1.8 Draw-order and far-field bugs: ash/blight storm drawn over terrain and objects (WE-5); Ghostfence field
  has no far copy and its far light sprites float in the gap (GF-1, B5); far-field L2/L3 never selected or never
  tinted (FF-2: 25 L2 + 4 L3 nodes resident).
- [ ] 1.9 Streaming and culling levers from the old M2 list (archive "Next up" 1, 6, 7): render-thread spikes,
  `NoCpuCulling` A/B (J2), shadow-caster rules (J4), pipeline warm-up (J7), precombine evaluation (J6);
  deferred checks ST-2, ST-3, SC-2.
- [ ] 1.10 RT per decision 2: exterior RT content (`ray_tracing_lod`, `NoRayTracing` on proxies, terrain RT
  proxy), Solari sunset patch; checks DL-2, DL-3 (needs a `solari,dlss` build).
- [ ] 1.11 Thin binary (ARCHITECTURE §8b rule 4): the app is 13.5k lines and `exterior.rs` is the most-edited
  file; move systems into plugin crates. Re-measure the dev-loop gates (last measured 10-12, half the crates).

## Phase 2: a playable vertical slice (the basics)

Done when: new game in Seyda Neen → walk in third person → talk to Fargoth → get his ring back → journal entry →
save, quit, load. Break it down (and write the missing specs: dialogue, journal, saves) before building.

- [ ] 2.1 Player character: third-person controller + camera (`specs/action.md`, `camera.md`; `sf_action` exists),
  activation with a "press to use" prompt.
- [ ] 2.2 UI: widget kit + Morrowind theme (M5 U3), HUD + message box (U4), inventory on minimal `sf_rpg` stacks
  (U5), settings/options + keybinds (U6). `sf_ui`, `sf_ui_bevy`, `sf_ui_css` exist (U1, U2 confirmed).
- [ ] 2.3 Text: fonts, books and scrolls, notifications.
- [ ] 2.4 Dialogue + journal + minimal quest store (M7 subset), MWScript legacy runtime for the slice (M5 W3;
  MWScript → Luau translation exists, W1/W2).
- [ ] 2.5 Saves (M7 subset): world overlay + player; net-ready per `specs/net.md`.
- [ ] 2.6 Sound pass: footsteps from animation text keys, UI sounds, music, interior/exterior ambience
  (`sf_audio` exists, minimal).

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
- [p] Foliage light transmission (WISHLIST §11).
- [p] Cloth: capes/banners/tents (robes stay authored skin; PH-3 confirmed).
- [p] Seamless portals (M9), far-field performance (M8: HLOD, texture streaming, PBR generation).
- [p] Co-op N1–N8 (`specs/net.md`; N1–N5 after this phase, N6–N8 after M7).
- [p] Lua/Luau beyond L3; mods milestone demo (M5 E1, U7, X1, X2).
- [p] Crate "update all" pass once `bevy_hanabi` supports Bevy 0.20 (archive Backlog).

## Recent (last ~10 entries; older ones drop into the archive)

- 2026-10-19 (phase0): audit; owner check pass (14 windows); ROADMAP rewritten into phases; ARCHITECTURE §10/§11
  moved to `superfargoth/docs/DECISIONS.md`.
