# Visual checks: shadows

Open visual checks: sun, lamp and contact shadows (SH-*). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

## Distant shadows (superfargoth `weather/distant-shadows.md`, "Near-term visual fixes", bug A)

The shadows session (2026-10-12) moved the sun's cascades to 504 m (2048² after shadows2; 4096² cost +1.4 ms)
and added a terrain far shadow past them. Screenshots: `shots\shadows\`
(`before_*`, `after_*`, `north_1830_on/off.png`). Cost measured +2.5 ms at
1440p (over budget; being cut).

### SH-9: Many shadowed lights indoors (shadows F6)
- Kind: game location. Run: `.\bevyrrowind view "Seyda Neen, Census and Excise Office" --camera "1.0,4.2,5.5,2.7,3.0,-4.0" --point-shadows 8`
  (desk, chair, chandeliers, two NPCs at 6-10 m); compare `--point-shadows 1` (High today) and `--point-shadows 4` (Ultra).
  Also `view "Balmora, Council Club"` walking past the braziers.
- Look for: with 8, the desk, chair legs and NPCs cast shadows from several lamps at once; chandeliers throw patterns on
  the ceiling and floor; no light leaks through walls into the next room. Walking or turning the camera: no shadow pops
  (the shadowed set is picked by importance incl. lights behind you, with hysteresis and a 0.3 s crossfade);
  candle flicker doesn't make shadows jump between lights.
- Cost: uncached Bevy maps, ~1.9 ms per light (census bench: 4.8 / 6.6 / 12.2 / 21.7 ms for 0 / 1 / 4 / 8), so the
  presets are unchanged until cached maps (superfargoth `docs/specs/shadows.md` slice 2). Owner: is 8 the look you want?
- Status: **rejected 2026-10-19 (owner, 8 shadows):** some shadows flicker, e.g. the thin cable holding a chandelier (thin casters in the cube maps; check cache recapture + candle flicker). Was: (shf6 saw 1 vs 8 at the camera above: 8 adds table/chair/chandelier shadows, clearly moodier).
  Since shf6b (slice 2) maps come from a cache (`--off shadowcache` for Bevy's own): also look for NPC shadows
  following them, no shadow left behind when a door opens or an item is moved, no 1-3 frame flicker of actor
  shadows when a light recaptures. Not yet seen by an agent (no screenshots).

- Source: shf6 (shadows F6 slice 1).


### SH-13: Point-light (lamp) shadows (owner report)
- Kind: game location. Run: `.\bevyrrowind scene lamps-handoff` (Balmora, 22:00).
- Seen by owner 2026-10-19 (round 5): lamp shadows only visible a few metres from the lamp; some have a grainy,
  dithered pattern; walking guards cast a shadow that isn't their outline; some shadows flicker.
  Suspects (unverified): budget of 1 shadowed light at High with a 15-25 m fade (dithered fade = the grain),
  `shadow_importance` switching which light casts (flicker), skinned casters or 1024² cube faces + bias (guards).
- Owner 2026-10-19 (round 9, lt-cave): grain varies, some lamp shadows are clean and others very grainy (per-light: budget fade dither vs. the fully shadowed light?).
- Owner 2026-10-20: shadows overall "only showing when super close from the object". Note: since 2026-10-19 the
  app no longer adds the parked cube-cache plugin, so the one shadowed lamp redraws its 6 faces every frame.
- Status: known bug: Phase 1 shadow pass (1.4). Audit 2026-11-01: now 8 maps at 512², fade 35-45 m; reach = the lamp's range
  (4 × radius); grain = 512² + 1-tap lamp filter (`--off lampfilter`); "random" picks = ranking by camera distance (audit §2).

### SH-14: Contact-shadow bands at object edges (owner report)
- Kind: game location. Run: `.\bevyrrowind view Balmora --camera "-280,+12,200,-295,+1,180" --hour 15 --weather clear`,
  turn the camera; A/B `--off contactshadows`.
- Seen by owner 2026-10-20: thin shadow bands around object edges that move with the camera angle; gone with
  `--off contactshadows` (confirmed cause). Ours march 0.5 m with 0.1 m thickness (Bevy default 0.3 m), Medium 8 /
  High 16 steps (`exterior.rs` `Quality::contact_shadows`). Fix direction: shorter rays, or contact shadows for
  close contact only, as engines use them; then recheck SH-7's grounding.
- Status: known bug: Phase 1 shadow pass (1.4).

### SH-15: 8 cached lamp shadows (shadows v2 V1, shadows2)
- Kind: game location. Run (release): `.\bevyrrowind view "Seyda Neen, Census and Excise Office"`, walk up to the
  desk and the NPCs; then `.\bevyrrowind scene lamps-handoff` (Balmora 22:00), walk the street past torch-bearers.
  A/B: `--point-shadows 1` (old High), `--off shadowcache` (Bevy redraws every face).
- Look for: lamp shadows from several lamps at once, out to ~30-40 m (was a few metres); no flicker when walking
  close to a lamp or when NPCs walk by; NPC shadows follow them without 1-frame gaps; no shadow left behind
  where an object streamed out (≤ 2 s is by design); no shadow "swimming" with a carried torch.
- Owner decides: High's lamp count (8 now; walk ~+2 ms vs 0 lamps, perf.md "Shadows v2").
- Status: partial (owner 2026-10-21, census, 8 lamps): flicker at first, stable once the maps are captured; shadows
  pop in a few metres ahead (end of the hall gets its shadows only when walking through the door): shadowed-set
  changes are visible. Goes to slice A/B (budget + light lab).

### SH-16: Shadow reset A1 (lamp contact off, sun out of interiors, GTAO)
- Kind: game location. Run (release): `.\bevyrrowind view "Seyda Neen, Census and Excise Office"`; `.\bevyrrowind
  scene lamps-handoff` (Balmora 22:00, walk the street); `.\bevyrrowind view Balmora --camera "-280,+12,200,-295,+1,180"
  --hour 15 --weather clear` (SH-14's spot), turn the camera. A/B `--off ssao`, env `BEVYRROWIND_LAMP_CONTACT=0` (lamp contact shadows are on since 2026-11-01).
- Look for: no thin view-dependent bands at object edges near lamps (SH-14), the sun's own contact shadow short
  (0.3 m); soft darkening in room corners, under tables/crates and where walls meet the floor, without dark halos
  around NPCs or a grainy crawl under TAA/DLSS; indoors nothing lit or shadowed from the sun's direction; at night
  torch-bearers' own lights cast moving shadows (within High's 8); outdoors sun shadows, cloud shadows and rain
  occlusion under roofs as before.
- Status: inconclusive 2026-10-21 (bw-owner10): owner can't judge AO / contact shadows without live on/off toggles
  (and is unsure contact shadows work at all); needs the dev console (ROADMAP 1.13, queued).
- 2026-11-01: console toggles exist: backquote, `flip ssao`, `flip contactshadows` (dev builds). Status: awaiting owner.
- 2026-11-01 (owner, dev console): both flips recompile pipelines (expected: view key bits) but `flip
  contactshadows` shows no difference at all and `flip ssao` only a faint darkening here and there. Code trace
  (camera components, sun flag, pipeline key, shader march) finds every link present; Bevy's AO only scales ambient
  and sky light, never direct sun or lamps, so sunlit scenes show little by design. Contact shadows, code audit
  (2026-11-01): every link is upstream Bevy 0.20.0 verbatim (`contact_shadows.rs`, view bind group and offsets,
  `light.rs` flag build, `mesh_view_types.wesl`, the directional block in `pbr_functions.wesl`, the hit maths in
  `ssr/raymarch.wesl`); the RO-11 patch is the identity without DLSS; the march was measured at 0.8 ms per light
  (shadows.md "Reset A1"), so it runs. No defect found; the likely answer is magnitude: Bevy's 0.3 m world-space ray
  and 0.1 m thickness are tuned to its 1 m demo scene, and High's first cascade (1.4 cm texels, PCSS) already
  resolves contact, so the march adds nothing a flip can show. Owner A/B that isolates them: `view Balmora --camera
  "-291,+1.7,187,-297,+0.9,179" --hour 16 --off shadows` vs `--off shadows,contactshadows` (no sun shadow maps:
  short dark bases under props and feet must appear in the first run). Console vars `render.contactlength` /
  `render.contactthickness` built 2026-11-01. Owner, same evening: contact shadows are visible, but only outdoors
  by day. That is the configuration, not a bug: only the sun has `contact_shadows_enabled` (the moon spawns with
  Bevy's default, false, despite shadows.md "sun and moon"; the sun is out of interiors since A1; lamps and torches
  get none unless `BEVYRROWIND_LAMP_CONTACT=1`). Owner 2026-11-01, Census office with `BEVYRROWIND_LAMP_CONTACT=1`:
  lamp contact shadows work and the cost feels acceptable. Status: confirmed; decision pending on making them the
  default (High/Ultra) and on a screen-space ray length (UE style) for unshadowed lamps.
- Owner 2026-11-15 (Census office, `flip ssao`): GTAO works but is "barely visible", a very subtle difference on/off. Expected: Bevy (like UE) applies AO to ambient/indirect light only, and our interiors are lit mostly by direct lamp light over a flat ambient; AO shows more once probe GI carries the indirect fill (GI-1/GI-2: "too weak"). Status: confirmed with follow-up.

### SH-19: Shadow reset B2 (lamp shadows on 2x2 hardware PCF)
- Kind: game location. Run (release): `.\bevyrrowind view "Seyda Neen, Census and Excise Office" --camera
  "1.0,4.2,5.0,1.0,3.6,-6.0"`, then walk to the desk and the NPCs; also `view Balmora --hour 22` by lamps.
  A/B: `--off lampfilter` (lamps back on the preset's Temporal filter, the old look).
- Look for: lamp shadow edges (table legs, chandeliers, NPCs on the floor) as soft as before under TAA, no stair-step
  aliasing or crawling/shimmer on the edges while walking; the sun's shadows outdoors unchanged. Agent screenshot
  pair at that camera looked identical at full-screen scale; motion is the open question.
- Owner decides: keep 2x2 for lamps (-0.1 ms GPU per shadowed lamp at 1440p) or `--off lampfilter` as default.
- Status: inconclusive 2026-10-21 (bw-owner10): owner can't judge the lamp filter without live on/off toggles; needs
  the dev console (ROADMAP 1.13, queued).

### SH-20: Character shadows update very infrequently (owner report)
- Kind: game location. Run: `.\bevyrrowind scene lamps-handoff` (Balmora 22:00) and a daytime street with walkers;
  watch the shadow of a walking NPC.
- Seen by owner 2026-10-21 (bw-owner10, SH-17/18 pass): NPC shadows move in coarse steps, so the shadow animation
  looks choppy while the NPC itself is smooth.
- Cause (charshadows 2026-10-21, confirmed in code, not yet at runtime): lamp shadows only. Shadow reset B3's skip
  rule in vendored `bevy_pbr` light.rs hashes a skinned caster by its root on a ≥ 20 cm grid (joints ignored) and
  refreshes ≤ 2 faces/frame, so a walker's lamp shadow redraws ~5-8 Hz and an idler's never animates. Not the cube
  cache (`POINT_SHADOW_CACHE` off since B3, so `--off shadowcache` is a no-op) nor the anim budget.
- Fix (render-opt 2026-10-28): `ShadowMoverRefresh` in vendored `bevy_pbr/src/render/light.rs` redraws close-up animated character shadow faces (within 16 m of active camera, up to 4 mover faces/frame) on the same frame outside the LRU budget; A/B `--off movershadows`.
- Status: awaiting owner (2026-10-28).
- Owner 2026-10-30 (Census office, release build): indoors the player and NPCs cast no shadow from lanterns or torches
  at all (none moving with them, none at the feet). Exterior check (scene lamps-handoff) still pending. Status: awaiting owner.
- Owner 2026-10-30 (scene wk-balmora, day): sun shadows follow the NPCs and the player smoothly. Lamp part (scene
  lamps-handoff) still pending; interiors showed no lamp shadows on characters (note above). Status: awaiting owner.
- Owner 2026-10-30 (`scene lamps-handoff`, dev copy): every lamp shadow now animates smoothly (fixed), but which lamps cast
  looks random; one lamp shadowed an NPC and not the player; no held torch ever cast a shadow. Leads: the shadowed-lamp
  budget picks (High count still an owner pick, ROADMAP 1.4), player body excluded as a lamp caster?, torch-bearers
  (decision 3 says they cast within the budget; LT-3). Status: partial (smooth yes, consistency no).
- Audit 2026-11-01 (`superfargoth/docs/research/shadows/2026-11-01-audit.md` §1, confirmed in code): the player's meshes and
  held-torch lights have no `RenderLayers` (layer 0) while lamps and NPCs sit on the worldspace layer, so lamps never see the
  player as a caster and torch lights see only layer-0 meshes (the player). Review (Opus) added: the budget ranks lights
  by `Visibility` only, so other-worldspace or behind-camera lights take slots and never render (audit §1). Slices 1-2 there.

### SH-21: Lamp and torch shadows on characters; shadowed-lamp pick (shadow-fixes 2026-11-01)
- Kind: game location. Owner copy `%TEMP%\bw-owner-2026-11-01`: `02-census-office.cmd` (walk under a lantern),
  `03-lamps-handoff.cmd` (Balmora 22:00, torch-bearers), `01-new-game.cmd` (Seyda Neen night: lamps by the door).
- Look for: the player's own shadow under lanterns and lamps indoors and out (was never cast); a torch-bearer's torch
  shadows the bearer, the street and passers-by (was nothing); the shadowed lamps are the ones lighting what you see,
  stable while turning (were "random": slots went to lights of the next cell and behind the camera); no new popping.
- A/B: `--point-shadows 1` / `12`; the log line `point shadow on ...: its render layers ... meet no shadow caster's`
  must not appear (one `warn!` per bad light).
- Status: awaiting owner (built 2026-11-01, tests pass; spec `shadows.md` "Shadow fixes as built").
- Owner 2026-11-01 (first build): shadows do move with every held torch (win); but a lot of flickering, some shadows cut in
  half, character shadows choppy again (SH-20 regressed). Cause (code): a moving torch's six faces take the mandatory arm of
  the vendored refresh rule every frame, zeroing the 2-face LRU budget for every other lamp, and faces of one lamp refresh
  on different frames (the cut at a cube-face seam). Fix slice SH-20b (per-light grouping, separate budgets) in progress.
  Status: partial.
- Owner 2026-11-12: the shadow on the player still flickers hard close to some torches and lamps. Next session.
- Owner 2026-11-15 (texture lab, X orbit lamp at night): lighting/shadow pops off abruptly during the orbit (owner unsure it is when the lamp goes behind an object; the lamp is invisible); not diagnosed (see TX-lab).
- Owner 2026-11-15 (Balmora at night): (a) shadows on a moving NPC flicker when it walks among many lamps; (b) turning the
  camera or walking closer, shadows visibly fade in after the move. Code read (orchestrator, unverified): (b) the budget ranks
  only lamps with `ViewVisibility` (`sf_sky_bevy/src/shadow_budget.rs` module docs), so a turn swaps the shadowed set and new
  holders fade in; Clockwork picks slots by contribution at the camera incl. off-screen lights (clockwork-code-2026-10-25.md
  item 6). (a) likely the per-frame face-refresh budget (SH-20b) spread over many lamps. Fix slice queued after H1.

### SH-22: Exterior fps halves after leaving the Census office (owner report, perf)
- Kind: game location, dev build, 1280x720, High. `run_dev.ps1 view "Seyda Neen, Census and Excise Office"`, walk out the front door.
- Seen 2026-11-02: ~100 fps inside, ~50 outside (BRP frame time); same with `--off gi`, so not the GI. A Seyda Neen
  exterior start read 65-80 fps near spawn (other spot). Owner suspects the 2026-11-01 shadow work (d06b120 +
  uncommitted: visible-light budget, lamp contact shadows on by default, DLSS-space contact shadows).
- Rough A/B 2026-11-02 (manual walk-out, 23:00, torch-bearers moving, BRP fps over ~2 s; indicative only):
  default ~20 ms; `off contactshadows` ~20 ms; `--off movershadows` ~17.8 ms; `--point-shadows 0` ~11.9 ms.
  => point-light (lamp/torch) shadows cost ~8 ms here. Next: a proper `perf suite` run (perf.md) from a fixed
  camera outside the door at night, profiling build, `--point-shadows` default vs 0 and `--off movershadows`, GPU pass ms.
- Status: known bug: point shadows suspected, not measured properly (owner pass 2026-11-02).

### SH-23: Sun shadow early-out looks identical (vendored `bevy_pbr`, 2026-11-09)
- Kind: game location, release build of the patched tree vs one with the directional hunks reverted (shaders are
  embedded: no runtime switch). Runs: `view Balmora --hour 12`, `--hour 18.5` looking west under trees (RO-15 spot),
  `--hour 22` (moon shadows); each also with `--off contactshadows`.
- Look for: before/after identical: sun and moon shadows, contact shadows at wall bases and under props, leaf
  light-through (backlit canopy glow, trunk shadow on it), faces turned from the sun, nothing new at dusk as the sun sets.
- Cascade debug view (`DIRECTIONAL_LIGHT_SHADOW_MAP_DEBUG_CASCADES`): no game switch sets it; unchanged by construction
  (overlay never 0, so it still fetches everywhere), naga-validated only. Perf A/B: `test-pass-2026-11-10.md` "Rendering".
- Status: awaiting owner (naga-validated, never run). Source: sun-shadow-early-out; spec `shadows.md` "Reset B2 as built".

### SH-24: Statics still cast into cached lamp shadows with GPU-culled statics (perf-push P3, 2026-11-12)
- Why: near statics now carry `NoCpuCulling` (Bevy's lamp scan no longer sees them); the lamp shadow cache lists them
  itself and bakes them at capture (`superfargoth/docs/specs/shadows.md` "Caching", As built 2026-11-12).
- Kind: game location. Run: `scene hi-balmora-night` (Balmora 21:30, torches), then `scene lamps-handoff` (22:00) and walk
  the street; A/B with `--off cpuculltag` (old CPU culling).
- Look for: lamp light on walls, stairs and props throws the same static shadows as with `--off cpuculltag` (rails,
  barrels, awnings, door frames), no shadow missing for a second after a cell streams in, no static shadow that
  lingers or doubles; torch-bearers and NPCs still cast moving shadows over them.
- Status: awaiting owner.

### SH-25: Lamp shadows stay through a camera turn (lamp-select, 2026-11-15)
- **Verdict 2026-11-15: keep** (owner). Follow-up queued: retain off-screen holders in vendored `extract_lights` instead of pinning, to win back the +0.34-0.7 ms.
- Why: owner 2026-11-15 (SH-21 b): after turning the camera or walking closer, lamp shadows faded in. The budget only
  ranked lamps in the frustum (and weighted lamps ahead), so every turn swapped the shadowed set. Now it picks all
  around the viewer and a holder keeps its cached map off screen (`superfargoth/docs/specs/shadows.md` "Light selection").
- Kind: game location. Run: `bash bevyrrowind/tools/owner_launch.sh scene lamp-turn` (Balmora street, 22:00, clear,
  walkers with torches; same spot as `hi-balmora-night`).
- Look for: stand among several lamps, turn 180° and back (fast, then slowly), look somewhere, turn, look elsewhere:
  the lamp shadows are already there, nothing fades in after the turn. Walk toward lamps: shadows only strengthen
  with distance (35-45 m band), no half-second fade on arrival except when a nearer lamp really takes a slot from a
  farther one (8 slots on High).
- Also say what you see (not fixed here): (1) a walker passing among several lamps: shadows on it still flicker
  (cause and standard fix in the ledger 2026-11-15; next slice); (2) a far lamp ahead with no shadow while lamps
  behind you have one (the nearest 8 all around win now); (3) orbiting the third-person camera: any shadow swap.
- A/B: `--point-shadows 12` (more slots: fewer takeovers), `--point-shadows 0` (none). No switch back to the old rule.
- Status: awaiting owner (tests pass; perf A/B in the ledger).

### SH-26: Far sun cascades held between redraws (ROADMAP 1.22 F3, 2026-11-15)
- Why: perf (owner 2026-11-15, after the SPUD read): sun cascades are the longest render encode. Low to High now use
  3 cascades as before (2 measured no win with the hold); cascades 1-2 keep their fit and map until the camera moves 4 m, turns 3°,
  the sun moves or 4 frames pass (`SunCascadeHold`, `sf_sky_bevy/src/cascade_hold.rs`). Held fits are padded, so far
  shadows are ~8-11 % coarser.
- Kind: game location. Run: `bash bevyrrowind/tools/owner_launch.sh scene cascade-hold` (Balmora street, 16:00, clear, walkers).
- Look for: walk and turn (fast, then slowly): no popping or swimming on houses and terrain, no shadow gap at the screen
  edges after a fast turn; near shadows (0-20 m) as sharp as before; NPC shadows past ~18 m (cascade 0) may step a little (4-frame refresh).
- A/B: `--off cascadehold` (redraw every frame = old setup); `--sun-cascades 2:205` (2 cascades, opt-in). Walk -1.0 ms (ledger perf 2026-11-16).
- Status: confirmed 2026-11-16 (owner: "shadows look good"). Seen in the same run: lamp shadows fade in at ~10 m on some buildings at night, logged as SH-27.

### SH-27: Lamp shadows fade in close at night (owner 2026-11-16, bug)
- Seen: Balmora at night, some buildings: a lantern's shadow (the "complex" one, cast by the house) only appears within
  ~10 m of the viewer and fades in, although the lamp shadow fade is set to 35-45 m (`LAMP_SHADOW_FADE`, `mw_environment_bevy/src/render.rs:32`).
- Likely cause (not verified): the budget. High gives 8 cube maps (`POINT_SHADOWS_DEFAULT`, `render.rs:27`) to the
  lamps ranked by `importance` (`sf_sky_bevy/src/shadow_budget.rs:115`); in a street with more lanterns than 8, a lamp wins a
  slot only once it is among the nearest, then fades in over 0.3 s. To confirm: `--point-shadows 16` (does the
  distance grow?), `RUST_LOG=bevy_pbr::shadow_refresh=debug`.
- Standard fix direction: many lamp shadows in a shared atlas, resolution by screen size (UE, Frostbite, Decima), so far lamps
  keep a low-res map instead of none. Hard problem H2; owner picks.
- Status: logged, not fixed.

### SH-28: Lamp shadow day gate (CW-3a, ROADMAP 1.25, built 2026-11-16, opt-in)
- Run: `view "Balmora" --hour 12 --weather clear` and again at `--hour 19` (dusk) and `--hour 22`, each with and without
  `set shadow.lamp.day_gate on` (console, live; exe `target/profiling/cw3a.exe`, or a dev build). Lamps must light exactly the same
  in both; only their shadow maps change (`sf_sky_bevy/src/shadow_budget.rs`, contrast gate).
- Look for: at noon, lanterns under awnings/arcades (direct sun is counted even where the receiver is in shade): is a
  missing lamp shadow noticeable there? At dusk (~19-20 h): do lamp shadows fade in smoothly, no popping? At 22 h: no
  difference between the two runs.
- Status: confirmed 2026-11-16 (owner: "seems good", noon run; dusk fade not separately confirmed).

### SH-29: Point shadow face culling (CW-3b, ROADMAP 1.25, built 2026-11-16, opt-in)
- Run: `view "Balmora" --hour 21 --weather clear`, High, with and without `set shadow.lamp.face_cull on` (console, live; exe
  `target/profiling/cw3b.exe` or a dev build; vendored `bevy_pbr/src/render/point_face_cull.rs`). Stand 3-6 m from a street
  lamp with NPCs walking under it; spin 180/360 deg fast in third and first person; look straight up and down under it.
- Look for: a lamp shadow popping in or showing an old pose for a frame as it enters the screen; shadows cut or missing
  along a cube seam (45 deg diagonals from the lamp, straight below it); any difference against the same spin with the switch off.
- Status: confirmed 2026-11-16 (owner: "seems all good"). Owner noted ~17 ms average in some areas (dev build, night, only face culling on; dev builds link Bevy dynamically, not a bench number).

### SH-30: Sun/moon shadow gate (CW-8, ROADMAP 1.25, built 2026-11-16, opt-in)
- Run: `view "Balmora"` with `set shadow.sun.gate on` (console, live; exe `target/profiling/cw8.exe` or a dev build;
  `sf_sky/src/sun_gate.rs`, vendored `DirectionalLightShadowFade`). (1) 16:30 clear -> rain -> clear: shadows full until
  ~17:20, then fade over a few seconds, no pop, no hard far-terrain shadow left. (2) Ashstorm 08:00: weaker or fading
  shadows, look right? (3) Overcast noon: shadows stay (overcast dims sun and sky fill alike). (4) Night, dim moon: no moon shadows.
- Owner question (look, not perf): should overcast look diffuse (cloud cover turning direct sun into sky fill)? Then the
  gate would also drop overcast shadows by day.
- Status: confirmed 2026-11-16 (owner: "everything looks right"; run: rain from 16:30 through dusk to night; ashstorm and the clear->rain transition not run). Overcast-diffuse look: owner no strong opinion, maybe later if cheap, not today.

### SH-31: Lamp shadow cache (CW-4 S3, ROADMAP 1.25, built 2026-11-17, opt-in, first run on a GPU)
- Run (release build): `scene census` and `view "Balmora" --hour 22 --weather clear`, High; `set shadow.lamp.cache.enabled on`
  (console, live; alias `BEVYRROWIND_LAMP_CACHE=1`); F3 full: views by kind and `views/lamp_face/count` before/after, gauges
  `lamp_cache/*`. Code `sf_sky_bevy/src/lamp_cache.rs` + vendored `bevy_pbr/src/render/point_shadow_cache.rs`.
- Look for: lamp shadows identical to the switch off once settled (walls, furniture, crates); a short fade-in when a lamp
  first gets its slot (captures); NPCs and the player keep crisp, current shadows walking past a lamp and across a cube
  seam (45 deg diagonals, straight below), no flicker among several lamps (SH-21a); an item picked up or a door swung next
  to a lamp leaves a ghost for at most ~2 s; flipping the switch both ways shows no pop. Lamp face views should drop to
  near 0 standing still with nobody near a lamp.
- Status: awaiting owner.
- Owner 2026-11-17 (Census, dev build 02:37; the live switch-on no longer panics): no flicker; everything looks the same on
  and off; lamp face views drop when on. Status: Census confirmed; Balmora 22:00 next. Follow-up for later (not the cache,
  on and off alike): lamp shadows fade in too close: entering the Census and walking down the hallway, shadows visibly fade
  in a few metres ahead. Needs the lamp-shadow distance/fade retuned.
- Balmora 22:00 first crashed on the switch-on (`unwrap()` on `None`, vendored `texture_attachment.rs`; fixed same day, own
  patch "CW-4: load-only depth attachments"). Owner 2026-11-17 rerun: no crash, shadows stay crisp, look the same as off.
  Status: confirmed (Census + Balmora). Open: the close lamp-shadow fade-in above.

### SH-32: Single-encoder batched point shadow pass + lamp shadow cache (CW-13a, Slice A1, built 2026-11-19, opt-in)
- Why: CW-4's lamp shadow cache fixed SH-21a character flickering among many lamps but regressed render busy (+0.94 ms)
  because Bevy's `Core3dShadows` schedule opened 5 command encoders per drawn face (up to 45/frame). Slice A1
  (`shadow.lamp.batched_pass`, vendored `bevy_pbr` `record_batched_point_shadows`) batches all active point/spot shadow
  faces into 1 command encoder (−0.70 ms vs base / −1.38 ms vs unbatched cache on `balmora_walk_night`; −0.37 ms `faster`
  on `census`; `encode/queue/command_buffers` 70.6 → 39.0).
- Kind: game location. Run (profiling/release build with DLSS):
  `bash bevyrrowind/tools/owner_launch.sh --profiling view "Balmora" --hour 22 --weather clear --set shadow.lamp.cache.enabled=on --set shadow.lamp.batched_pass=on`
  and `view "Seyda Neen, Census and Excise Office" --set shadow.lamp.cache.enabled=on --set shadow.lamp.batched_pass=on`.
- Look for: walk past street lamps and torch-bearing NPCs in Balmora at 22:00 and through the Census Office; toggle
  `set shadow.lamp.batched_pass off` / `on` live in the console (`` ` ``): shadows on walls, props, NPCs and the player
  must look identical on/off with no missing casters, corrupted depth, seam cuts, or SH-21a character shadow flickering.
  F3 full panel should show `batched_point_shadows` (~0.15–0.22 ms GPU) and `encode/pass:shadow_point_light/count` ≤ 1.
- Owner decides: confirm visuals + whether to flip `shadow.lamp.cache.enabled` and `shadow.lamp.batched_pass` on by default
  before starting Slices A2–A3 (2D Shadow Atlas to remove the 8-lamp ceiling / SH-27 close fade-in).
- Status: **confirmed 2026-11-19 (owner):** "it seems to be working (fps higher)." Follow-up: flip defaults after review / 6-repeat confirmation if desired, then proceed to Slices A2–A3 (`docs/briefs/shadow-atlas-handoff-2026-11-19.md`).

### SH-33: 2D Point Shadow Atlas (CW-13b, Slices A2–A3, built 2026-11-20, opt-in)
- Why: replaces the 8-lamp cube array (and Bevy's 42-cube limit) with a buddy-allocated Static (4096²) + Live (2048²)
  2D Shadow Atlas (`shadow.lamp.atlas.enabled`, default off; `shadow.lamp.atlas.max_lamps`, default 48; contact shadows
  capped at `shadow.lamp.contact_max = 8`). Interleaved A/B `shadow_a3_atlas` (2560x1440 DLSS Quality, 18/18 quiet &
  focus-clean): all 4 comparisons formally **`faster`** (U=0.0): `balmora_walk_night` 10.63 → 9.79 ms (`atlas8`, −0.84)
  / 9.87 ms (`atlas48`, −0.76, 19 lamps + 2 torches); `census` 6.85 → 6.32 ms (`atlas8`, −0.53) / 6.52 ms (`atlas48`,
  −0.32, all 24 interior lamps simultaneously, 1% low 85 → 121 fps).
- Kind: game location. Run (profiling/release build with DLSS):
  1. `bash bevyrrowind/tools/owner_launch.sh --profiling view "Balmora" --hour 22 --weather clear --set shadow.lamp.atlas.enabled=on --set shadow.lamp.atlas.max_lamps=48`
  2. `bash bevyrrowind/tools/owner_launch.sh --profiling view "Seyda Neen, Census and Excise Office" --set shadow.lamp.atlas.enabled=on --set shadow.lamp.atlas.max_lamps=48`
  (Also test live console toggle `` ` ``: `set shadow.lamp.atlas.enabled off` / `on`.)
- Look for:
  1. **No close fade-in (SH-27 / SH-31):** walking down Balmora streets at night and down the Census Office hallway,
     distant street lanterns and hall lamps already cast shadows ahead of you without popping in 5–10 m away.
  2. **Face projection & seams:** walk around and directly under street lamps, torches, and chandeliers; verify shadow
     directions match light positions on all 6 faces (especially ±Z / 45° cube seams, straight down, and overhead), with
     no tile-edge bleeding or missing faces.
  3. **Dynamic movers & carried torches:** NPCs walking past lamps and torch-bearing guards cast smooth, current shadows
     without SH-21a flickering, stale silhouettes, or static depth disappearing behind them.
  4. **Tier transitions & live toggle:** walking toward/away from lamps across 10 m / 22 m shows no distracting shimmer
     or pop on tier resizes; toggling `set shadow.lamp.atlas.enabled off` / `on` live switches cleanly between 2D atlas
     and stock cube maps without crashes or visual glitches.
  5. **F3 full panel:** `point_shadow_atlas` (~0.07–0.11 ms GPU), `shadow_atlas/lamps` (19–24), `shadow_atlas/static_occ_pct`,
     `shadow_atlas/render/live_direct` (single pass).
- Owner decides: confirm visuals + whether to flip `shadow.lamp.atlas.enabled` on by default (High/Ultra/Medium) and
  whether to tune `shadow.lamp.atlas.max_lamps` / tier distances (`10 / 22 / 45 m`).
- Owner 2026-11-20 (first look): "a lot of shadows flashing when I move around: shadows appear, then disappear", outdoors and in the Census interior. Cause: atlas rects sampled before anything was drawn into them (ledger 2026-11-20 "SH-33 fix"); fixed the same day.
- Owner 2026-11-20 (second look): shadows still disappeared when getting close to lamps and flickered before disappearing while moving. Full audit found and fixed 5 P0 + 4 P1 root causes: `prepare_light` wiping `shadow_render_phases` without invalidating `LightKeyCache` (leaving `dirty_specializations` clean so `queue_shadows` dropped all visible casters and cleared tiles empty), non-draw frames (Half/Far skip, 12-face cap, Linger) flipping `mover_only` off/on and triggering phase wipes, tier resize clearing `c.baked` and racing `release_retiring` ahead of GPU confirmation, moving torches publishing culled/unwritten faces, wrong `view_backward` in face frusta, `fs_blit` missing `guard_ratio` across mismatched Static/Live tile sizes, live-toggle `shadow_normal_bias` mismatch, idle `AtlasPointShadowMap` change churn, full-atlas un-admitted lamps leaking stock scans, and pre-bias seam early-out (ledger 2026-11-20 "SH-33 full audit fixes").
- Status: **awaiting owner** (2026-11-20).
