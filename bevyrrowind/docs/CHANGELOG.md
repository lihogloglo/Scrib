# Changelog

One line per finished session, newest first (moved verbatim from the ROADMAP "Recent" section 2026-10-25). Not read at session start. Older history: `archive/`.

## Recent (last ~10 entries; older ones drop into the archive)
- 2026-11-05 (surface-w2): surface weather W2: new `sf_surface_bevy` (128² cell window texture, vertical 1024² shelter depth map as a `sun_view` subview, noise atlas, `SurfaceUniform`, `SurfaceWeatherView`, WESL `sf_surface_bevy::layer`), Bevy-free window/shelter/noise logic in `sf_surface::view`; terrain hook through material bindings (scaffold until W3) with console `surface debug exposure|cells`; naga-validated shader test; SW-2 awaiting owner.
- 2026-11-05 (surface-w1): surface weather W1: new Bevy-free `sf_surface` (wet + puddle per climate, snow + ash per 32 m cell, fixed 10 game-min steps from the weather blend, `sample`, quantized `SurfaceSave`); `sf/weather` v2 carries it (v1 saves migrate and warm up); `sf_weather_bevy::WorldSurface` steps it on game time; console `surface set/dump/clear`; 15 + 1 tests.
- 2026-11-05 (fixes-anims-1105, late): character-shapes S3: Delirium-held bodies get per-part kinematic proxies (`Layer::BODY_PROXY`, `sf_physics_bevy::proxies`, `ImpactOnCharacter.shape`), rapid hits each push (PH-15 confirmed); surface-weather spec (`specs/surface-weather.md`, W1-W7) from the research.
- 2026-11-05 (fixes-anims-1105): brace hands in front of the chest (AL-11 confirmed); vault with UAL2 SafetyVault + trigger fix (AL-12 confirmed, hiccup to tune); slope lean retuned (AN-21 confirmed); Delirium knee sink fixed (actor scale counted twice); Kimodo MG-C0 + gen2 (385 clips, simple prompts; stats tool, keyframe recipes); specs: character-shapes (S0-S7); owner: unresponsive and dragged bodies are Rapier ragdolls, Delirium for responsive ones.
- 2026-11-05 (owner-list-1105, later): Delirium hit reactions: every physics hit on a human walker ≥ 10 N·s is a push along its direction with the balancer on (`TakeoverKind::Hit`, forced fall ≥ 100 N·s, dying window for hit deaths, `Takeover::hit` for combat); PH-14/15 confirmed, AL-11 and CB-3 confirmed with follow-ups; stats spec (`specs/stats.md`, `stats-adapter.md`, S0-S9) written.
- 2026-11-05 (owner-list-1105): owner list pass: wall stop on head-on hits (`MoverParams::wall_stop_deg`) + wall-brace palm frame (rotating `HandIk`, `palm_axes`; AL-11); sheathed clicks never attack, F draws the weapon with Morrowind equip/unequip clips (CB-3); Delirium on by default, human knockdowns and deaths routed to it, console `kill` replaces `I` (PH-14/15/16); research: first-person arms, OpenMW mechanics (blocked on stats + effects); ROADMAP 2.2b, 2.7, 3.4b added.
- 2026-11-05 (d-2b-bake): de-lit textures in the game (D-2b): `bevyrrowind bake` makes `texture.roles` per model, `texture.delight` (BC7 sRGB KTX2, strength 0.5) per albedo texture and an index; a VFS layer above all mods serves a de-lit file only while its source is still the winner the bake saw (mod replacers win); `--off delight` A/B; 3521 textures, 34 s cold, 0.5 s warm; TX-delight awaiting owner.
- 2026-11-05 (d-2a-ktx2): de-lit texture encode (D-2a): `sf_texture::ktx2_out` builds the mip chain (area-weighted box in linear light, alpha-weighted colour, NPOT), BC7 via `intel_tex_2` (ISPC kernels; basic profile, alpha variant only when needed) and a KTX2 file (vkFormat 146, header/DFD via the `ktx2` crate, parses back in Bevy's reader); 12 tests; 512x512 mips + BC7 318 ms (fast 128 ms) on one core; bake.md "De-lighting"; adapter and cache layer are D-2b.
- 2026-11-05 (l-b-lampbody): lamp bodies don't light twice at GI hits (L-B): `GiSkipEmissive` (`sf_model_bevy`) makes probe GI trace a mesh through a child proxy with an emission-free material copy on the raster path (no shader or Solari patch; raster look and Solari path unchanged); `mw_assets` flags the emissive meshes of models carrying a placed light (openmw-rtx `LampBody`); interior GI ambient floor 0.1; GI-lampbody awaiting owner.
- 2026-11-05 (d-1-delight): albedo de-lighting algorithm (D-1): new Bevy-free crate `sf_texture`, `delight` module ports openmw-rtx `ShadingMap` (alpha-weighted linear luminance on a 32x32 grid, 3 wrap/clamp-aware box blurs, mean 1, clamp 0.5..2; apply divides in linear with hue-kept reflectance cap); 18 tests; 512x512 in ~6 ms; bake.md "De-lighting"; bake product is D-2.
- 2026-11-05 (l-a-reach): lamp reach follows shadow state (L-A): `sf_sky_bevy::ShadowReach` lets the point-shadow budget write `PointLight::range` as a blend of a short unshadowed and a long shadowed reach by the shadow fade (no pop); game lamps and torches 1.5 x radius without a map, 4 x with one (`LightCalibration::unshadowed_range_scale`, CLI `--range-unshadowed`); LT-reach awaiting owner.
- 2026-11-05 (rtx-borrow): openmw-rtx (path-traced Vulkan OpenMW fork) reviewed: not pluggable (C++/OSG, Vulkan RT pipelines, moving internal API), Solari stays Ultra; research on albedo de-lighting (`superfargoth/docs/research/delighting-2026-11-05.md`, industry-audit row) and Morrowind light rules (`bevyrrowind/docs/research/mw-lighting-rules-2026-11-05.md`); owner picks in DECISIONS 2026-11-05, slices queued in the ROADMAP.
- 2026-11-02 (gi-tune): GI gain tunable live: dev console `render.gi_intensity` (irradiance gain, as UE's Indirect Lighting Intensity) and `render.gi_ambient` (flat ambient kept inside the volume) write every `GiVolume`, re-fits included; `sf_gi_bevy` rewrites the 48-byte volume uniform on change (no re-trace, history kept); GI-3 gains a tuning line.
- 2026-11-02 (gi-ambient): dynamic probe GI now replaces the flat `AmbientLight` inside its volume (`GiVolume::ambient_scale`, default 0, uniform `bias_intensity.w`; vendored `bevy_pbr` GI hunk scales `ambient_light` where the probes are sampled; game: 0 in interiors), as DDGI/Lumen drop the flat term under probe coverage; gi.md slice 1/1c history moved to `gi-history.md`; GI-3 awaiting owner.
- 2026-11-02 (fg9-fog): FG-9 fix: the fog's sky fill is now one isotropic value per frame, the sky dome's mean radiance (SH L0, as UE/Frostbite volumetric fog; 64 sky-view LUT taps on a key-aligned Fibonacci hemisphere, `sf_atmos::fog::sky_fill` twin + tests), for froxels and far fog alike; was per view azimuth, so a low sun's aureole lit a vertical pillar; `clouds_fog_light` 66x1 -> 3x1; FG-9 awaiting owner.
- 2026-11-02 (owner pass): GI-1/2 partial (runs clean, effect too weak), RO-14 bloom and RO-15 leaves confirmed (`LEAF_DIFFUSE_TRANSMISSION` 0.3 -> 0.5); new FG-9 (fog sky-fill pillar, diagnosed) and SH-22 (exterior fps drop, point shadows ~8 ms, rough A/B); ROADMAP 3.5: drop the `I` kill key, attack only with the weapon drawn.
- 2026-11-01 (gi-sampling): GI slice 3: DDGI probe atlases (octahedral irradiance 6x6 + visibility 14x14 with borders, `sf_gi::ddgi`, from Majercik et al. 2019) written by the probe pass, sampled per pixel in the vendored `bevy_pbr` (`superfargoth patch (own, GI)`: `GI_PROBES` view key, bindings 39-42, 8 probes x trilinear x back-face x Chebyshev) in place of the env-map diffuse; ambient-cube/`IrradianceVolume` bridge removed; Census 0.08 ms (`atlas 105x21 tiles`, 26 lights); GI-2 awaiting owner.
- 2026-11-01 (gi-placement): GI placement: interior volume fitted to the world index's cell bounds (`sf_gi::fit_volume`, 32x16x32 budget, one probe of padding), rebuilt at door teleports (old despawned at the cut, fade held until the new one is traced; exteriors none), live `render.gi` / `on|off|flip gi` (`sf_sky_bevy::GiEnabled`).
- 2026-11-01 (gi-lights): GI slice 2: world-space light list shaded at probe-ray hits (point/spot lights in the active worldspace, nearest 64 to the volume, Bevy units and falloff, one visibility ray each; sun from our extract), 0.8 hysteresis near changed lights; Census office `lights 21`, 0.045-0.077 ms; volume now centred on the start target (was 15 m above the room).
- 2026-11-01 (gi-dispatch): GI slice 1c: the probe pass dispatches in the Census office (0.025 ms, 512 probes x 64 rays): own `bevy_solari` patch builds the scene bind group with no light (interiors have none; Solari's lighting still skips lightless scenes); GI status log counts render-world meshes (mirrored 833, BLAS 682, rejected 151 by attribute set) and Solari lights; `--off gi` clean.
- 2026-11-01 (gi-meshes): GI slice 1b: streamed interiors load ray-tracing mesh variants while GI or the RT path is on (`sf_world_bevy::RayTracingVariants`, `sf_world::RayTracedWorldspaces`, traced level gets `RayTracingMesh`, ray-traced worldspaces skip precombines); `mirror_meshes` traces only the camera's worldspace layers; `sf_sky_bevy` tests run with and without `solari` (13/13); Census run pending (gi.md "Scene and lights").
- 2026-11-01 (visual-niceties): light through leaves: leaf cutouts (`mw_nif::Cutout` rule, NIF and cooked paths) get Bevy `diffuse_transmission` 0.3 + `thickness` 0.05 m and `TransmittedShadowReceiver`; `--off foliagetransmission`; no console var (materials are startup assets); RO-15 awaiting owner.
- 2026-11-01 (visual-niceties): bloom on the game camera: Bevy `Bloom::NATURAL` (energy-conserving, no threshold) at intensity 0.1 on Medium/High/Ultra, runs after DLSS/TAA; `--off bloom`, console `render.bloom` (0 = off) seeded from the camera (2 tests); RO-14 awaiting owner.
- 2026-11-01 (gi-spike): GI slice 1 code: `sf_gi` core (10 tests) + `sf_gi_bevy` ray-query probe update (Solari scene on the raster path, ambient cubes through stock `IrradianceVolume`, `gi/probe_update` span, `--off gi`); forward path survives; pass does not dispatch yet (runtime meshes not Solari-compatible); final clippy/build/run pending the owner's go (gi.md "Spike").
- 2026-11-01 (contact-audit): SSAO/contact-shadow audit (both wired; SSAO is ambient-only by Bevy design, contact shadows were sun-only by A1 config, not broken); lamp contact shadows on by default on High/Ultra (owner confirmed, Census office); console `render.contactlength` / `render.contactthickness`; rule "no automated visual checks" in CONTRIBUTING.md + bevyrrowind/CLAUDE.md; GI research (`gi-2026-11-01.md` baked, rejected; `gi-dynamic-2026-11-01.md` adopted) and spec `superfargoth/docs/specs/gi.md`; GI opened as a Phase 4 lane, spike queued.
- 2026-11-01 (save-fixes): first `/crossreview` (Saves, `docs/reviews/saves-2026-11-01.md`; the review tool lives in its own Crossreview repo) and its 10 confirmed findings fixed: loads apply at the fade cut, save identity filled and checked, crash-safe `.bak`, corrupt-save fallback, real read-back, failed loads block saving, autosave retry, weather saved, `sf.character.v2` (old saves broke after combat C1a).
- 2026-11-01 (shadow-audit + shadow-fixes): audit `superfargoth/docs/research/shadows/2026-11-01-audit.md` (3 agents + independent Opus review) then built: characters follow their worldspace `RenderLayers` (the player and torch lights were layer 0: no lamp shadows on the player, torches saw nothing), shadow budget ranks view-visible lights only (slots went to next-cell / behind-camera lights), shadow filter follows the jitter, contact shadows + SSR march in main-pass pixel space under DLSS, sun far-cascade/PCSS taps on the full receiver slope + half-texel margin (RO-11), bevy#26234 (GTAO at main-pass res) and #26170 vendored; SH-21 + RO-11 awaiting owner (`%TEMP%\bw-owner-2026-11-01`).
- 2026-10-30 (owner pass): B2c wall brace built + gate fix (AL-11 partial: slides sideways on a straight hit, trail pose wrong); hard landings no longer lock (no fall damage yet); borderless fullscreen default; owner visual pass: AL-10, PL-1, HD-1, CO-1, SH-11, CB-2, AU-4, AL-1, AN-1 confirmed, MM-1/FF-2/AU-5 rejected, PH-2 and old knockdown checks retired (delirium from now on), 17 new bugs logged (RO-11..13, PH-12..14, WE-6/7, FG-7/8, AU-6, CB-3, PL-6).
- 2026-10-28 (render-opt): Phase 1 visual bugs + perf pass: Clockwork world-unit directional PCSS, Bevy #26235-26237/#26321/#26095 backports (LT-4), SH-20 close-mover lamp shadow refresh, RO-6 DLSS sub-viewport sweep, DL-5 interior RT guard, RO-B lit smoke + plume Aabb, LM-2 NIF lighting/clamp modes + 16-bit DDS cutouts, SH-11 far-shadow PostUpdate order, FF-2 per-level impostor tint, WE-5 1.6 km storm cap + dry PrecipOcclusion skip, WA-1 horizon variance transfer; H1 CPU/extract/LUT wins (`balmora` 9.48, `seyda_neen` 9.95, `red_mountain` 9.00, `balmora_walk` 10.50, `flight` 11.10 / 10.63 ms warm).
- 2026-10-28 (ambient-anim-audit, end): AN-22 leg snaps fixed (retarget leg IK eased around contact changes, stable knee pole; 339 -> 25 flags; owner confirmed), anim.retarget v4; MG10a interactive clip browser; HTML review dropped (owner).
- 2026-10-28 (ambient-anim-audit, later): MG5b clip-preview fixed (first clip never started); MG4 furniture census (Morrowind furniture 15-30% taller than real); MG6 real furniture and MG7a held prop in clip-preview; MG2 tier1 recipes (15 activities), first batch 268 clips in 15 min.
- 2026-10-28 (ambient-anim-audit): ambient-life audit + 140-activity catalog; MotionBricks/Kimodo tested on the 4060 (5.4 ms per link; ~2 s per 6 s Kimodo clip); spec motion-generation.md (3.8); MG1 motiongen batch tool, MG3 SOMA BVH onto base_anim (generated source), MG5 clip-preview (compiles, first run pending).

- 2026-10-27 (evening, feel-coord): 4-hit light chain (UAL2 Sword_Light_A-D), lock-on C1c parts 1-4 (Q / MMB / R3, facing and
  strafe, locked side rolls, marker; CB-2), high mantle on ClimbUp_2m (AL-10), clip hand/foot contacts (B2b 1, 2a); combat.md C1 briefs
  archived. Owner: chain, M1, lock + facing confirmed; mantle hands aim at the box centre (B2b-2), hard to trigger (B2-T).
- 2026-10-27 (afternoon, feel-coord bug pass): hold sprint (`InputSampler::release` vs shared buttons), cursor confined on Windows,
  light chain ~0.4 m (`root_scale`), cook `start_anchor` (UAL2 split-take pelvis offset), player vs dummy collision (`separate_moved`),
  Delirium push F11 only and never on action characters, get-up rest check + `roll_stall`; full UAL1/UAL2 bundled (Git LFS).
  Owner confirmed CB-1, PL-2, DE-1; M1 (cursor on left click) in progress; leftovers in ROADMAP 3.1, 3.2b, 3.5.
- 2026-10-27 (combat-c1b): combat C1b, player vs training dummy: `sf_combat_bevy`, `mw_combat` sword sheet on 14 UAL slots, F stance,
  LMB light chain, Shift tap roll / hold sprint, HUD stamina, scaffold `TrainingDummy`, `levels/arena.ron` (`scene level-arena`), O combat
  gizmos; CB-1 awaiting owner.

- 2026-10-27 (combat-c1a): combat C1a, Bevy-free core: new `sf_combat` (sheet, vitals, sweeps, contact order,
  `StandardRules`, hit-stop), `sf_action` Clip owner + combat vocabulary, `sf_anim::SocketTrack`; 26 headless tests
  (`sf_combat/tests/combat.rs`, `test_sword.ron`) found and fixed clip-rate root motion, chain ranks, `End` parsing, hit-stop hold.
- 2026-10-27 (combat-spec): combat spec written (`superfargoth/docs/specs/combat.md`, `specs/combat-adapter.md`): new
  `sf_combat(_bevy)`, `CombatRules` seam, `sword_1h` sheet on UAL clips, slices C1a-C7; SYSTEMS row, ROADMAP 3.5; docs only.
- 2026-10-26 (evening, ik-fixes): the two anim audits' fixes. Pose: pelvis-bob harness, ground-plane foot locks, StrideWarp
  drop through a spring + limit, foot IK gates (`gate` param, physics share, no locks during hand-back), physics blend after
  the ops, stride ratio/alpha spring + LOD fade, probes at the lock / post-warp, humanoid data (stride 1.2, rate share 0.5;
  AL-9). Jump: `Jump@head`/`@tail`, JUMP/FALL/LAND + `jumping`, player air debounce (PL-3). Delirium: `Held` marker, floor at
  the probed ground. Mantle progress per tick. anim-lab starts clear at noon (level `hour`/`weather`). Left: sprint cycle, strafe skate.

- 2026-10-26 (day, scrib): save/load screen from the pause menu (save, overwrite, load, delete; OpenMW fields; SV-2);
  minimal HUD (bars from the `player` record, compass, crosshair, use prompt; HD-1); console Down/Enter on suggestions,
  OS paste, exterior `coc` (OpenMW port; CO-1 re-check); BSL-1.0 for two clipboard crates; `cargo_turn.sh` ignores `cargo run`.
  Later: framework main menu + release start (no subcommand: menu over Seyda Neen; New game, Continue, Load), reference stack
  counts and gold piles (OpenMW), framework container window + game looting (IV-2), Godotwind audit + borrow index (1.15), ROADMAP 3.6/3.7.

- 2026-10-26 (night, scrib): physics fix plan slices 1-10 (collision-ready gate, arrival gate + safety net, items asleep
  until disturbed, OpenMW NIF rules + full-triangle fallback, held item on the final camera + tick-view hand; PH-11); app split
  done (15 `mw_*_bevy` crates, binary 2.3k lines; gates re-measured, 3 misses logged); pause menu + options in game (PM-1);
  doors on rebindable Use; doc audit S5-S14 (weather/animation splits, history to archive/, DECISIONS index, summaries).
- 2026-10-25 (merge-ws2): one workspace gated: cargo-hakari over framework + game (opt-ins `dev`/`dlss`/`solari`/`file_watcher`
  no longer unified into default builds), `cargo deny` green (tes3 git allowed, MPL for dirs/MP3), clippy green (default + all-features); tests pending.
- 2026-10-25 (dlss-plugin-1/2): DLSS split out of the game: NVIDIA-free `dlss_wgpu` facade + separately shipped
  `dlss/` plugin package; SR/RR verified in game, pass costs as before, 0 NGX strings in the release exe (DL-5 logged).
- 2026-10-25 (certshots1): `--screenshot` waits on a GPU readiness gate (CPU settled + no pipelines compiling for 10
  frames, `sf_diag_bevy::ReadyGate`); a 120 s timeout now logs what was pending and exits 1 (`--screenshot-anyway` forces).
- 2026-10-25 (console, items, items2, dialogue3, uipolish): dev console with live SSAO; take items + inventory screen;
  centred windows + chargen topics; button hover, journal layout, keybind labels. Owner confirmed TR-3, PL-1 prompt.
- 2026-10-25 (sellus): skipped chargen leaves Sellus Gravius's papers local, so he no longer asks for papers.
- 2026-10-25 (saves2): load correctness: resident cells reload from the loaded overlay at the fade cut (`reload_resident`,
  discard deactivations), moved physics items flushed into saves, menus closed on load, player body saved/restored.
- 2026-10-25 (saves1): saves part 1: game save sections on `sf_save_bevy` and F5/F9 quicksave/quickload restoring in the
  running game through the door fade (any worldspace); headless round trip on real ESMs; SV-1 awaiting owner.
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

