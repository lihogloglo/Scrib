# Ledger: physics, collision, items, ragdolls

Format: `date | what was tried | measured result (scene, ms, counters) | verdict | commit or doc`. Newest first.
Sources: `bevyrrowind/docs/specs/physics-fix-plan.md`, CHANGELOG, `visual-checks/physics.md`. Companions: [animation](animation.md), [combat-feel](combat-feel.md).

2026-11-11 | Owner: a friend's project replaces the GTA IV weights Delirium needs; adopt it when it lands, do not build our own | n/a | open | ROADMAP 3.2b
2026-11-10 | Delirium off by default: delirium-physics 0.5+ reads numbers from the player's GTA IV program (`kv::unlock`), takeovers panic without it | opt in with `--features delirium`; tests: 2622/2645 pass on the laptop pull-build | opt-in | 6c9fdb3
2026-11-09 | Re-pin delirium-physics from the vanished `91ee246` to `f4ff84d` (0.7.0); new `Joint.centred` and `BuildPart.length` at defaults | `mw_delirium`(`_bevy`) tests pass; in-game check pending | kept | 3e219bf
2026-11-08 | Platform decision: Delirium stays default (owner overruled agent Q8) | n/a | superseded 2026-11-10 | 7294aac
2026-11-05 | Character shapes S3: per-part kinematic proxies on Delirium-held bodies (`Layer::BODY_PROXY`, `sf_physics_bevy::proxies`) | rapid balls push held bodies each time; PH-15 confirmed | kept | ffc18a6
2026-11-05 | Delirium knee sink: actor scale was counted twice | PH-15 knee sink fixed | kept | e0fa727
2026-11-05 | Delirium hit reactions: every physics hit on a human walker >= 10 N s is a push with the balancer on, forced fall >= 100 N s, dying window for hit deaths | PH-14/15 confirmed; fall threshold should later scale with Strength/Agility/Fatigue | kept | 664e938
2026-11-05 | Delirium on by default, human knockdowns and deaths routed to it via `RagdollTakeover`; console `kill` replaces the `I` key; beasts and creatures keep the Rapier ragdoll | PH-14, PH-15 confirmed; PH-16 confirmed for console kill, hit deaths unseen until combat C6 | kept (feature later made opt-in) | 271e061
2026-10-30 | Owner pass: PH-2 retired (clip + limp ragdoll deprecated, Delirium from now on); PH-11 partial; PH-12 floor hole and PH-13 drop-on-load logged | hole at F1 coordinates 11, 6, -13; owner: diagnose the class of bug, do not patch the spot | open | af6c335
2026-10-27 | Delirium slice 2b: recovery blends out to the idle, get-ups from LaFAN1 by lying pose, roll to the clip's side first, limp while down | owner pass DE-1 confirmed (push in F11 only); logged: crouched landing sinks/slides ~1.5 s before get-up | kept | 8bf4724, 9437caa, dafad58
2026-10-26 | Delirium slice 2a: takeover of animated human NPCs (`mw_delirium::takeover`, `mw_delirium_bevy`), dev click-to-push | headless tests; in-game owner check later | kept | c3337a6
2026-10-26 | Delirium slice 1 (ROADMAP 3.2b): character built from the `Bip01` skeleton, pose mapping both ways | headless harness tests | kept | 2c7f701
2026-10-26 | Fix plan slice 10: sweep starting inside a floor recovers | test first; recovery only if red | kept | dfb0699
2026-10-26 | Fix plan slices 8-9: held item drawn after the camera rig (`PhysicsSystems::HeldPoses`), hand from the body's tick pose | steady carry; leftovers: drag slow-down only on the free camera | kept | 750565e, a3e830d
2026-10-26 | Fix plan slice 7: collision fallback = full LOD0 triangles (rules version 3) | fallback 326,579 to 475,411 triangles (2,334 models); all collision 9.1 to 10.8 MiB; Balmora (-3,-2) 19.0k to 21.0k; insert ms unmeasured | kept | 441128f
2026-10-26 | Fix plan slice 6: NIF collision rules parity with OpenMW `bulletnifloader.cpp:110-326`, unit test per rule | tests per rule | kept | be68d1b
2026-10-26 | Fix plan slice 5: Census Office collision census (`mw_bake/examples/collision_report.rs`) | no walkable hole in baked collision (74 authored floor/wall modules, 9 LOD0 fallbacks, 52 NC decor); lead is the runtime arrival/streaming path | diagnosis; owner still saw a hole on 2026-10-30 (PH-12) | d81117c
2026-10-26 | Fix plan slice 4: items asleep until disturbed (`undisturbed` flag, re-settle on wakes caused by fixed partners) | late static does not wake items; thrown ball still knocks the cup | kept; PH-11 partial | f8adc50
2026-10-26 | Fix plan slices 1-3: collision-ready gate for un-activated cells, player arrival gate, safety net (`last_ground`, `kill_height`, interior void guard) | body restored before its floor exists does not fall | kept; PH-13 drop-on-load still reported 2026-10-30 | 30f0d22, b4a12e4, 9ac6bd6
2026-10-26 | Slice 11 (player physics capsule) held for the owner | thrown items pass through the player; carry exclusion is dead code | open | physics-fix-plan.md
