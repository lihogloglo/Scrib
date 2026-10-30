# Physics fix plan (2026-10-26)

Owner, 2026-10-25: fix items moving on room entry, held-object vibration, falls through floors, falls on save load.
Bar: correct across Morrowind's bad assets, efficient, fast; port OpenMW freely. Inputs (read the section a slice
names, not the whole note): [diagnosis of our code](../research/physics-diagnosis-2026-10-25.md) (S1-S4, "Fix
slices", "Systemic gaps") and [OpenMW research](../research/openmw-physics-2026-10-25.md) (OpenMW 4e718b50, Jolt MR
!3919). Backend: rapier3d 0.36.1 (not avian; H4 text is stale).

## Root causes (high confidence, from code reading; each slice's red-first test confirms its own)

- **Collision readiness is blind to un-activated cells.** The scheduler activates placements under a time budget
  (`sf_physics_bevy/src/stream.rs:1208`); `collision_loading` and the item gate see only activated ones. Walkers
  check `covers(.., Level::Active)` (`walkers.rs:500-503`); the player and items don't.
- **New contacts wake sleepers** (rapier `narrow_phase/contacts.rs:363`), including a freshly inserted static: an
  item settles, its table arrives frames later, it wakes and moves (S1).
- **Save load / door arrival:** colliders are re-streamed at the fade cut, the player body is restored the same
  frame and falls during the hold; no gate, no recovery (S4; also a candidate for S3).
- **Fallback collision = render LOD1** (unlocked borders, up to 8 % radius error, sloppy pass drops triangles) for
  2,497 models / 97k placements; OpenMW uses every triangle (S3 class cause).
- **Held item drawn against last frame's camera** in player mode (camera rig runs after carry) (S2).

## What we copy from OpenMW (and what we don't)

- Port: gravity off until the area's static collision exists, skip the first simulation frame after load or
  teleport; two-stage ground find on arrival (shape trace, then ray, place at hit + small offset); interior void
  guard (lowest static AABB per interior, fall below it -> back to the arrival marker); NIF collision selection
  rules (last RootCollisionNode, empty one = no physical collision, skip MRK / editor markers, NC/NCC flags, Switch
  = first child, "Bounding Box" node = actors only); unstuck ladder if overlap recovery is needed.
- Adapt: OpenMW never simulates clutter. We keep physical items (owner's M4 goal) but make them **asleep until
  gameplay disturbs them**: a late static or an NPC capsule re-settles them instead of moving them.
- Skip: OpenMW's per-frame settings (units, 4005 u/s fall cap) unless a slice shows a need; we calibrate in metres.

## Slices (one deliverable + its test each; one physics lane, serial unless marked)

**Status 2026-10-26 (night):** 1-10 done and pushed (30f0d22, b4a12e4, 9ac6bd6, f8adc50, d81117c, be68d1b,
441128f, 750565e, a3e830d, dfb0699); owner check PH-11. Leftovers: drag slow-down only on the free camera
(`carry_feedback`); arrival spot over a hole loops with a warning; fade gate_max 1.5 s may show the frozen body
briefly; overlap deeper than step_up (0.45 m) not recovered; doorless link ends ignore `PlayerMode.paused`;
Balmora insert ms after slice 7 unmeasured (needs `--bench`). 11 waits for the owner.

1. **G1 collision-ready covers un-activated cells**: `stream.rs` `collision_loading` + `apply_inserts` item gate
   use `covers(ws, pos, margin, Level::Active)` + no pending static under the item; fix stale doc `stream.rs:224`.
2. **S4-a player arrival gate**: marker in `sf_action_bevy` that `move_bodies` skips, inserted on `TravelArrived`
   and on save restore, removed when G1 is clear around the feet; fade holds while set; two-stage ground find on
   release. Test: body restored before its floor exists does not fall.
3. **S4-b player safety net**: `last_ground` set on arrival; world `kill_height()`; interior void guard (OpenMW
   `Scene::playerMoved`). Test: body over a hole returns to the arrival spot.
4. **S1-a items asleep until disturbed**: per-body `undisturbed` flag in `sf_physics/src/world.rs`, re-settle on a
   wake caused only by fixed/undisturbed partners. Tests: late static does not wake placed items; a thrown ball
   still knocks the cup.
5. **S3-a cell collision census** (`mw_bake/examples/collision_report.rs`): names the Census Office floor model,
   its source (authored / fallback) and error. Diagnosis only. Can run beside 1-4 (bake side).
   **Result 2026-10-26:** no walkable hole in the office's baked collision (74 authored RootCollisionNode floor/wall
   modules, 9 LOD0 fallbacks, 52 NC-flagged decor; only unreachable tops and ceiling-height reversed winding). The
   owner's fall is most likely the runtime arrival/streaming path (fixed by slices 1-3); recheck with the owner.
   Tool: `SF_COLLISION_CELL="<cell>" cargo run --release -p mw_bake --example collision_report`.
6. **S3-n NIF collision rules parity** with OpenMW `bulletnifloader.cpp:110-326`: unit tests per rule in `mw_bake`
   / `mw_nif`, fix any rule we break. Bake side, after 5.
7. **S3-b fallback = full LOD0 triangles** (bump rules version so the bake re-runs); measure Balmora insert ms and
   memory (expect ~373k -> ~750k triangles, +9 MiB). Middle path if too costly: collision-only simplification,
   locked borders, no sloppy pass, ~1 cm error cap.
   **Result 2026-10-26:** full triangles shipped (`Source::AllGeometry`, rules version 3; `.glb` fallback LOD0).
   Whole install: fallback 326,579 -> 475,411 triangles (2,334 models), all collision 9.1 -> 10.8 MiB; Balmora
   (-3,-2) cell 19.0k -> 21.0k collision triangles. Insert ms not measured (needs the windowed bench).
8. **S2-a held item drawn after the camera rig** (`PhysicsSystems::HeldPoses` in PostUpdate after the rig).
9. **S2-b hand from the character's tick pose**, holder on the player body.
10. **S3-c sweep starting inside a floor recovers** (test first; overlap recovery only if red).
11. Owner decision first: **player physics capsule** (spec says every character has one; ours has none, so thrown
    items pass through the player and the carry exclusion is dead code).

Tests follow the look-and-feel rule: sanity asserts (doesn't fall through, comes to rest, no NaN); tuned numbers are
printed `#[ignore = "measurement"]` tests. Owner checks after 1-4 land: PH-4 carry, SV-1 load, a room entry with
clutter (Census Office), and the floor hole once 5-7 land.

## Open owner questions

- S2 "carrying pushes the capsule through the floor": first or third person, did the view sink or the drawn
  capsule? (The player has no physics capsule today.)
- Exact spot of the Census Office hole (F1 overlay coordinates) if the census doesn't find it.
- Slice 11: give the player a physics capsule now or in Phase 3?
