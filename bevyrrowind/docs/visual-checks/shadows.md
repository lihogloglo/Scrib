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

### SH-11: Distant flashes while moving (owner report)
- Kind: game location. Run: `.\bevyrrowind scene balmora`, fly around.
- Seen by owner 2026-10-19 (round 1): brief flashes in the distance whenever the camera moves; they look like shadows.
- Fix (render-opt 2026-10-28): ordered `far_map::update_far_map` before `update_terrain` and moved `fog_shadow_heights`, `far_shadow_input`, and `FarFieldSystems::FarShadow` to `PostUpdate` after `TerrainSystems::Update` so shifted terrain origins never render 1 stale frame; also replaced Bevy cascade-NDC PCSS with world-unit directional PCSS (OpenMW Clockwork adaptation).
- Status: awaiting owner (2026-10-28).
- Owner 2026-10-30 (scene balmora, flying): the distant shadow-like flashes are gone. Status: confirmed.

### SH-13: Point-light (lamp) shadows (owner report)
- Kind: game location. Run: `.\bevyrrowind scene lamps-handoff` (Balmora, 22:00).
- Seen by owner 2026-10-19 (round 5): lamp shadows only visible a few metres from the lamp; some have a grainy,
  dithered pattern; walking guards cast a shadow that isn't their outline; some shadows flicker.
  Suspects (unverified): budget of 1 shadowed light at High with a 15-25 m fade (dithered fade = the grain),
  `shadow_importance` switching which light casts (flicker), skinned casters or 1024² cube faces + bias (guards).
- Owner 2026-10-19 (round 9, lt-cave): grain varies, some lamp shadows are clean and others very grainy (per-light: budget fade dither vs. the fully shadowed light?).
- Owner 2026-10-20: shadows overall "only showing when super close from the object". Note: since 2026-10-19 the
  app no longer adds the parked cube-cache plugin, so the one shadowed lamp redraws its 6 faces every frame.
- Status: known bug: Phase 1 shadow pass (1.4).

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
  --hour 15 --weather clear` (SH-14's spot), turn the camera. A/B `--off ssao`, env `BEVYRROWIND_LAMP_CONTACT=1`.
- Look for: no thin view-dependent bands at object edges near lamps (SH-14), the sun's own contact shadow short
  (0.3 m); soft darkening in room corners, under tables/crates and where walls meet the floor, without dark halos
  around NPCs or a grainy crawl under TAA/DLSS; indoors nothing lit or shadowed from the sun's direction; at night
  torch-bearers' own lights cast moving shadows (within High's 8); outdoors sun shadows, cloud shadows and rain
  occlusion under roofs as before.
- Status: inconclusive 2026-10-21 (bw-owner10): owner can't judge AO / contact shadows without live on/off toggles
  (and is unsure contact shadows work at all); needs the dev console (ROADMAP 1.13, queued).

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
