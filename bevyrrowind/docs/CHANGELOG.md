# Changelog

One line per finished session, newest first (moved verbatim from the ROADMAP "Recent" section 2026-10-25). Not read at session start. Older history: `archive/`.

## Recent (last ~10 entries; older ones drop into the archive)

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

