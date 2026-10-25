# Physics diagnosis, 2026-10-25 (our code; read-only lane `physdiag`)

Four owner symptoms from the 2026-10-25 play test, traced through our code. This note only diagnoses.
The coordinator merges it with the OpenMW research lane into `bevyrrowind/docs/specs/physics-fix-plan.md`.
Nothing here was run live. Code was read at `d499fa7`, and the rapier source at `rapier3d-0.36.1` in the cargo registry.

**Backend, settled:** **rapier3d 0.36.1** (root `Cargo.toml:98`, `sf_physics/Cargo.toml:20`). No crate depends on
avian. `hard-problems.md` H4 ("avian3d has no spherical joint motor") and the root `Cargo.toml:93` comment are stale.
Fix those two doc lines in the plan's doc slice.

**Two facts behind several symptoms:**

- **F1. R0 activation is time-sliced, but our gates assume it is not.** `sf_world_bevy/src/stream.rs:1208-1231`
  (`activate_item`) activates a cell's placements in chunks of `placements_per_item`, under the `activate_ms`
  budget (`sf_world/src/scheduler.rs:146,160`, 0.5 ms × burst). A cell's activation can therefore span several frames.
  `PhysicsStreaming::collision_loading` (`sf_physics_bevy/src/stream.rs:228`) and the item gate in `apply_inserts`
  (`stream.rs:539-545`) only see placements that are *already activated and pending*. A static the scheduler has
  not reached yet is invisible to both. The doc comment at `stream.rs:224-226` says "R0 activation is not
  [time-sliced]", which is wrong. Walkers avoid this: they also require `scheduler.covers(ws, pos, margin,
  Level::Active)` (`apps/bevyrrowind/src/walkers.rs:500-503`). The player and items do not.
- **F2. Rapier wakes the sleeping side of every new contact, including contacts with a newly inserted *fixed*
  collider.** In `rapier3d-0.36.1/src/geometry/narrow_phase/contacts.rs:363-368`, "starts wake unconditionally"
  calls `strong_wake_sleeping_side`. Pairs are recomputed for every *modified* (that is, newly inserted)
  collider (`contacts.rs:80-92`). So a static that arrives under or next to an already-asleep item wakes the item.

---

## S1. Loose items move by themselves on entering a room

**Code path.** Door: `fade.rs:226-246` (cut, `begin_burst`) → the scheduler's Activate stage →
`PhysicsHook::activate` (`stream.rs:280-314`) queues `Pending` for statics and items alike → `apply_inserts`
(`stream.rs:471-650`) inserts statics, then items once the worldspace has no static *pending* (`:539-545`) →
`PhysicsWorld::insert` (`sf_physics/src/world.rs:266-354`) builds `Dynamic { asleep: true }` + `begin_settle`
(`world.rs:578-588`: all axes locked, `time_until_sleep = 0`). The first `step` (`world.rs:616-642`) puts the
item to sleep, and `finish_settle` (`:603-614`) unlocks it and restores the 2 s sleep delay. After that the item is
an ordinary sleeping dynamic body.

**Hypotheses, ranked.**

1. **Statics (or neighbouring items) arrive *after* an item has settled and wake it (F1 + F2). High.**
   Item A goes in during frame k: its chunk was activated and no static of the worldspace is pending. A settles
   and sleeps, possibly with nothing under it. In frame k+1 the scheduler activates the next chunk, and the table,
   shelf or floor under A is inserted. The new contact wakes A (F2). A is no longer settling, so it simulates
   unlocked for at least 2 s:
   - it falls the few mm to cm it hovered,
   - it depenetrates at up to 0.5 m/s (`max_corrective_velocity`, `world.rs:180`) where it was sunk,
   - it rolls on any slope. LOD1 tables are not flat: see S3.

   Wakes then chain across touching items. The same thing happens between items of different batches
   (`item_batch` 32, `sf_physics_bevy/src/lib.rs:98`): a new settling item B touching a sleeping A wakes A, and
   B's island stays awake, so `finish_settle` releases B awake too.
   *Evidence for:* the vanilla data. `physics-adapter.md` "Authored placement" says items "often sit a few
   millimeters into tables or hover above them". The existing test `placed_items_stay_asleep_and_exact`
   (`sf_physics/src/tests.rs:206`) inserts the ground first and every item before the first step, which is exactly
   the order streaming does *not* guarantee.
   *Against:* none in code. It is not confirmed by a run.
2. **NPC kinematic capsules inserted overlapping items. Medium-low, local.**
   `sf_physics_bevy/src/character.rs:174-229` inserts a capsule for every non-`Dormant` mover. An NPC placed at a
   desk whose capsule clips an item is a new contact: the item wakes, and the kinematic body pushes it with
   effectively infinite mass. This explains only items near NPCs.
3. **Floating-origin rebase.** Low. Interiors stay well inside `rebase_distance` (512 m), and `rebase`
   (`world.rs:811-848`) re-settles sleepers anyway.
4. **First-step dt spike.** Low. `max_steps_per_frame` is 3, and each step is a fixed 1/60 s.
5. **Item shapes bigger than visuals.** Low. PH-7 found the shapes "very good almost everywhere"
   (`visual-checks/physics.md:49`).

**Confirm headless.** Add `late_static_does_not_wake_placed_items` next to `placed_items_stay_asleep_and_exact`
(`sf_physics/src/tests.rs`): insert 50 items, `step` 5 times, insert `ground`, step 600 times. Expect, red on
today's code, `Woke` events and moved positions. A second case inserts the items in two batches with steps between
them (stacked pairs). Data: rebuild and run `SF_ITEMS_CELL="Seyda Neen, Census and Excise Office" cargo run
--release -p mw_bake --example body_report`. It lists every item's `items.check` drop, rise and shift. Don't run the
stale Oct-13 `target/release/examples/body_report.exe`: it may write old-format products into the bake store.

---

## S2. A held object vibrates, and "carrying pushes the player capsule through the floor" (H4)

**Code path.** `apps/bevyrrowind/src/physics.rs:334-340` puts the `Holder` on the **camera** entity (`With<FreeCamera>`),
in player mode too: the camera keeps `FreeCamera`, disabled. The sequence for one frame:

- `record_views` (`sf_physics_bevy/src/carry.rs`, `RunFixedMainLoop` / `BeforeFixedMainLoop`) records the camera's
  `CellCoord + Transform`, stamped with `Time<Virtual>` now.
- `drive_hands` (FixedUpdate, before Step) sets the kinematic hand to `view_at(fixed now)` (`sf_physics/src/carry.rs:389-406`).
- `held_poses` (Update, `PhysicsSystems::Carry`, before WriteBack) draws the item at
  `hand(camera Transform now) · lerp(rel)` (`carry.rs:435-442`).
- In player mode the camera is placed by `sf_camera_bevy` `update_rig` in **PostUpdate**
  (`sf_camera_bevy/src/lib.rs:77-81`), after all of the above.

**Hypotheses, ranked.**

1. **One-frame-stale view in player mode. High, for the vibration.** The app orders `PhysicsSystems::Views` after
   Bevy's `run_freecamera_controller` (`physics.rs:100`), which runs in `BeforeFixedMainLoop`
   (`bevy_camera_controller-0.20.0/src/free_camera.rs:45-49`). So in free-camera mode the view is current, and
   PH-4 was checked that way ("The dev camera carries (no player yet)", `physics.rs:329`). In player mode the
   rig moves the camera in PostUpdate, so two things go wrong:
   - **Render side.** `held_poses` uses the *previous* frame's camera, and the camera then moves. The item is drawn
     one frame behind the view: about 4-5 cm at 1 m when turning at 3 rad/s at 60 fps, and it varies with frame
     time. That is a visible shake.
   - **Physics side.** Each recorded view carries a timestamp one frame newer than its pose. On top of that, the
     camera follows the *interpolated* body, which already lags up to one tick. So the hand target gets
     frame-time-dependent noise (about speed × several ms), and the soft joint (ω 5.8) turns it into wobble.
2. **The hand is placed from the render camera, not the character. Medium.** In third person,
   `hand_for` (`carry.rs:255-266`) puts the item 0.6 m + its radius in front of the *camera*, which sits behind the
   player. The item ends up between the camera and the body, or inside the body. It looks like it is "pushing" the
   player's capsule mesh. The pick ray (`carry.rs:269-280`, reach 2.5 m) also starts at the camera.
3. **"Pushes the capsule through the floor": no physical path in today's code.** The player is an `ActionBody`
   with **no physics capsule**: capsules exist only for `Mover`s (`character.rs:181`), and `spawn_body`
   (`player.rs:263-291`) adds none. The player's ground and camera probes use `GROUND_LAYERS = STATIC|TERRAIN|PROP`
   (`sf_physics_bevy/src/lib.rs:122`). A carried body is `Layer::HELD` (`sf_physics/src/carry.rs:329`), so the
   mover never sees it. `holder_body` is never set by any host, so the per-pair exclusion is unused. Likely
   readings of the report:
   - (a) the drawn capsule mesh overlapping the item (hypothesis 2);
   - (b) the item pressed into the floor near the feet when looking down, with the carry motor (240 N) fighting
     the floor contact, which jitters;
   - (c) a different bug seen at the same time (S3 / S4).

   **Ask the owner one question:** first or third person, and did the *view* sink, or the drawn capsule?
4. **Mass ratio / solver.** Low. The hand is kinematic, so its mass is infinite. Held items get extra iterations
   and CCD (`set_grabbed`). Their mass is clamped to 0.2-60 kg.

**Confirm headless.** Add a test in `sf_physics_bevy/src/tests.rs`: a `Holder` camera moved by a system in
`PostUpdate` (like the rig), turning at a constant rate with variable frame `dt`. Assert that the drawn item pose
in camera space (`HeldPoses` vs the camera's final `GlobalTransform`) stays constant to about 1 mm. It is red
today. `carried_item_is_steady_in_view_space` (`sf_physics/src/carry_tests.rs:64`) covers only the core, with a
view that is always current.

---

## S3. Invisible hole in the floor of "Seyda Neen, Census and Excise Office"

**Code path.** Static shape = `collision.cook` (`sf_controller/src/steps.rs:104-120`). The game rules
(`mw_bake/src/collision.rs:89-139`) give either the authored `RootCollisionNode` subtree, or none (skinned / NC /
marker), or the rest pose. **Anything else falls back to `from_cooked` at `CollisionSettings::lod = 1`**
(`sf_controller/src/steps.rs:34-41,83-97`): the **simplified render LOD1**. Then `MeshShape::from_static` builds a
rapier `TriMesh` with `ORIENTED | FIX_INTERNAL_EDGES` (`sf_physics/src/shape.rs:20-21`). The player's
`ActionBody` sweeps it through `PhysicsProbe` (`sf_physics/src/probe.rs:63-67`, `stop_at_penetration: false`).

**Hypotheses, ranked.**

1. **The floor piece has no `RootCollisionNode`, and its collision is a render LOD1 with holes. High as a class,
   unconfirmed for this cell.** LOD1 comes from `sf_model/src/cook.rs:238-290`:
   - it targets half the triangles with `lock_borders: false` (`cook.rs:33-37,51`);
   - the error cap doubles from 1 % up to 8 % of the bounding radius;
   - if that still stalls, it runs the **sloppy simplifier at 16 % of the radius**, which is topology-destroying.

   For a room shell of 10 m radius that means borders moved by up to 0.8 m, or up to 1.6 m with the sloppy pass,
   plus triangles dropped. That is fine for a render LOD seen from far away, and wrong for collision. Scale: 2,497
   models / 97k placements use this fallback (`actors-adapter.md:442`). OpenMW instead uses every triangle of the
   visible geometry (`collision.rs:5-6` notes the difference). Missing pieces would show as holes only where the
   floor model lacks a `RootCollisionNode`, which matches "some floors".
2. **The player was below the surface before the floor existed (S4's mechanism at the door).** Medium.
   - On the arrival cut, the body is teleported (`player.rs:410-433`) and steps every tick with no gate
     (`sf_action_bevy/src/lib.rs:252-290`, `move_bodies` has no `Dormant` filter).
   - The interior's statics stream in over the next frames (F1).
   - A body that dropped a few cm first starts its next sweep inside or under the floor triangle. With
     `stop_at_penetration: false`, a cast from inside does not stop, so it falls through.

   The owner's report ("invisible hole", while walking) fits hypothesis 1 better. If the hole is always at the
   same spot, it is hypothesis 1.
3. **Flipped winding with `TriMeshFlags::ORIENTED | FIX_INTERNAL_EDGES`.** Low-medium. Bad assets often have
   inverted triangles. For rapier *contacts* (items, ragdolls), the internal-edge fix clamps normals to the
   triangle's front, so bodies on a back-facing floor can be pushed down. The *mover* sweeps (shape casts) are not
   affected, so this is not the player's hole. It would explain items sinking.
4. **Thin-floor tunnelling.** Low for the player: sweeps cannot tunnel. Items use CCD on drops (`stream.rs:566`).
5. **Hidden nodes.** Low-medium. The LOD fallback skips hidden (app-culled) geometry (`actors-adapter.md:432-433`).
   If a floor's walkable part is a hidden node outside a `RootCollisionNode`, it has no collision. Check against
   OpenMW's `BulletNifLoader` (research lane).

**Confirm headless.** Extend `mw_bake/examples/collision_report.rs` with
`SF_COLLISION_CELL="<cell>"` (pattern: `body_report.rs:57-121`, `statics_near`). Per static placement it should
print: the model, its `Source` (`collision.rs:42-63`), LOD0 vs collision triangle counts, LOD1 error ÷ radius, and
whether LOD1 came from the sloppy pass. The cook needs to expose that bit; it may have to be inferred from
error ≥ 8 % of the radius. Sort by floor-ness (normal·Y). The Census Office hole should show up as a RenderLod
floor model with a large error. The app has no live collision dump. `--physics-debug` + `G` (PH-7 scene) shows
the cyan statics, which the owner could look at in the cell.

---

## S4. Loading a save sometimes puts the player too low; they fall into the void

**Code path.** The sequence, in order:

1. `saves.rs:224-247` `restore_characters`: `RestoreBody` = the saved `(ActionState, Locomotion)`, and
   `StartTravel { reload: true }`.
2. `fade.rs:226-246` at the cut does three things in **the same frame**:
   - reparents the camera;
   - calls `streaming.reload_resident()`: every resident placement deactivates with `discard` and reactivates,
     so **all colliders are removed and re-streamed**;
   - writes `TravelArrived` before any readiness check.
3. `player.rs:410-433` `arrive` → `body.restore(loco)` (`sf_action_bevy/src/lib.rs:210-215`) puts back the saved
   position, velocity and mode (airborne if saved mid-air).
4. From the next tick, `move_bodies` steps the body under gravity during the whole fade hold (up to
   `max_hold_secs` 10 s). The hold waits for cells and textures (`fade.rs:255-260`), **not** for physics inserts.

**Hypotheses, ranked.**

1. **No collision-ready gate for the player on arrival (door or load). High.** The gate exists only for the first
   spawn (`player.rs:364`, which is itself blind to F1) and for walkers. While the floor is out (removed by
   `reload_resident`, re-queued under time-sliced activation and the 0.5 ms insert budget, plus async shape
   loads), the body falls: about 5 cm after 0.1 s, about 44 cm after 0.3 s. Once below the floor surface it never
   comes back up (S3 hypothesis 2 mechanism). "Sometimes" matches the race between shape-cache hits and frame
   budgets.
2. **No safety net for the player.**
   - `guard_fall` (`player.rs:466-479`) needs `last_ground`. `arrive` clears it (`player.rs:432`), and a body that
     never touched ground after arrival is never recovered.
   - Its limit is 200 m (`player.rs:83`).
   - The kill-height recovery (`sf_physics_bevy/src/character.rs:106-160`, `CharacterFellOut`) only covers
     `Mover`s, not `ActionBody`.

   So the fall is endless ("into the void"). This is an aggravating factor, not the cause.
3. **Saved mid-air or saved render pose.** Medium-low.
   - The save stores `Location` from `loco.render_position()` (interpolated + correction, `saves.rs:188`) but
     restores `loco` (simulated). They differ by less than a tick of motion, harmless.
   - A save while airborne restores `Fall` with its velocity (`.sessions/_archive/saves2-2026-10-25.md:21`, the
     "restored mid-air body" leftover). That is a legitimate fall, unless hypothesis 1 removes the floor under it.

**Confirm headless.** An `sf_physics_bevy` or app test: an `ActionBody` on a floor placement, `reload_resident`
equivalent (remove the floor body, re-queue it to arrive N frames later), step 60 ticks. Expect the feet stay above
the floor (red today). The gate's own test reuses `collision_loading_until_the_static_is_in`
(`sf_physics_bevy/src/tests.rs:253`) with the static *not yet activated* (F1 case), which must still read "loading".

---

## Fix slices (ordered by dependency, then owner impact)

Each slice is one deliverable + its test, sized for one agent at about 100k tokens or less. **P** = perf-relevant.

1. **G1: collision-ready includes un-activated cells** (`sf_physics_bevy/src/stream.rs` `collision_loading` +
   `apply_inserts` item gate; reads `sf_world_bevy::Streaming::scheduler().covers(ws, pos, margin, Level::Active)`,
   as `walkers.rs:500-503`). Items wait per item: their bounds' square must be covered at `Active` and have no
   pending static. Fix the stale doc at `stream.rs:224-226`. Test: `sf_physics_bevy/src/tests.rs`, a two-chunk
   activation keeps items out until the second chunk's floor is in. **P**: one `covers` per pending item per
   frame, which is cheap.
2. **S4-a: player arrival gate** (app `player.rs` + `sf_action_bevy` `move_bodies`). Add a `Frozen`/`Dormant`-style
   marker that `move_bodies` skips (sf_action_bevy cannot depend on sf_controller_bevy:
   `sf_action_bevy/Cargo.toml:9`, so define it in `sf_action_bevy`). Insert it on `TravelArrived`. Remove it when
   G1's check is clear around the feet. Also hold the fade (`FadeTransition::hold`) while it is set. Test: the
   app-level or `sf_action_bevy` test above (body restored before its floor exists does not fall).
3. **S4-b: player safety net** (app `player.rs`). On arrival, set `last_ground` to the arrival or saved feet when a
   down-probe finds ground. Use the world's `kill_height()` (like `character.rs:122-160`) instead of only 200 m
   below the last ground. Test: a body teleported over a hole returns to the arrival spot.
4. **S1-a: re-settle placed items woken by fixed geometry** (`sf_physics/src/world.rs`). Keep a per-body
   `undisturbed` flag, set at placement insert and cleared by grab, impulse, push, script, or contact with a
   moving non-fixed body. When an undisturbed body wakes and all its touching partners are fixed or undisturbed,
   `begin_settle` it again instead of letting it simulate. Detect this through `track_motion`'s `Woke`. This is the
   "keep clutter asleep until gameplay touches it" rule: Morrowind and OpenMW never simulate clutter at all. It
   makes S1 independent of streaming order (exterior neighbour cells, NPC capsules). Test: the
   `late_static_does_not_wake_placed_items` test above, plus "a thrown ball still knocks the cup"
   (`knocked_item_wakes_rests_and_interpolates` stays green). If G1 lands first, S1-a is the robustness half.
   Otherwise do it first: it alone stops the visible motion.
5. **S3-a: cell collision census** (`mw_bake/examples/collision_report.rs`, `SF_COLLISION_CELL`). Diagnosis
   only: name the Census Office floor model and its source and error. Test: example compiles; output in the session
   note.
6. **S3-b: collision fallback = full render triangles (LOD0), not LOD1** (`mw_bake/src/collision.rs` sets
   `CollisionSettings { lod: 0, .. }` through `MwCollisionRules`, or a game override; bump the rules version so the
   bake re-runs). OpenMW parity. Test: a `mw_bake` unit test where a model without a `RootCollisionNode` keeps
   every LOD0 floor triangle, plus a `collision_report` before/after table. **P**: about 373k → up to about
   750k fallback triangles (+ about 9 MiB, BVH builds on the task pool, inserts measured by `insert_static_s`).
   Measure the Balmora insert ms. A middle path, if the cost is too high: a collision-only simplification with
   `lock_borders: true`, no sloppy pass, and an absolute error cap of about 1 cm.
7. **S2-a: held item drawn after the camera rig** (`sf_physics_bevy/src/carry.rs` + set ordering). Add a
   `PhysicsSystems::HeldPoses` set in PostUpdate, after the host's camera rig (the app configures
   `.after(CameraSystems::Rig)`) and before transform propagation. Also move `record_views` to read the final
   camera of the frame, stamped with that frame's time. Test: the PostUpdate-rig test in S2.
8. **S2-b: hand from the character's tick pose** (`sf_physics_bevy/src/carry.rs` + app). The view comes from the
   `ActionBody` tick pose + eye height + look (deterministic, net-ready, right in third person), not from the
   render camera. Put the `Holder` on the player body. Test: carry while walking in third person keeps the item in
   front of the body.
9. **S3-c: sweep from inside a floor** (`sf_controller` / `sf_physics/src/probe.rs`). A test only, at first: a
   capsule starting 2 cm inside a floor trimesh must recover upward, not fall through. If it is red, add the
   standard depenetration pre-pass (Jolt `CharacterVirtual`, PhysX CCT "overlap recovery"). It may be unnecessary
   after slices 2-4.
10. **Doc fix**: `hard-problems.md` H4 backend (rapier, not avian), root `Cargo.toml:93` comment. Trivial; it can
    ride with any slice.

Owner-impact order if slots are scarce: **2 → 4 → 1 → 5 → 6 → 7 → 3 → 8 → 9**. That order fixes the falls into
the void and the room-entry motion first, then the hole, then the jitter.

## Systemic gaps (what a robust layer has and ours lacks)

- **One collision-ready source of truth.** Activation completion plus physics inserts per area. Today it is split
  between `collision_loading` (blind to un-activated cells), walkers' `covers`, and nothing for the player. Engines
  hold actors and gate the load screen on it (UE `bCanEverAffectNavigation`/level streaming "visible + collision
  ready"; Bethesda's cell attach).
- **Fade/loading screen gated on physics readiness**, not just cells and textures (`fade.rs:255-260`).
- **"Asleep until disturbed" for placed clutter.** A settle that survives late statics (F2) and is cleared only by
  gameplay disturbance. Today a settle is one-shot.
- **A player physics capsule.** The spec says every character owns one (`physics.md` "Controller"). The player
  has none, so thrown items pass through the player, the player cannot push props, and the carry holder exclusion
  is dead code.
- **Player fall-out recovery** at parity with movers (kill height, last ground set on arrival).
- **Collision meshes built for collision**: full triangles or authored, never a render LOD with unlocked borders
  or a sloppy pass. Also add a per-model census in the bake report (holes, open-edge length).
- **Overlap recovery in the mover** (start-penetrating sweeps), the standard CCT safety.
- **One frame clock for view-driven physics.** Holder views come from the simulation tick, not the render camera.
- **Already present (keep):** CCD on drops and throws, thin-shape contact skin (`world.rs:305-308`), kill height
  for bodies, rebase that preserves sleep, time-sliced inserts.

## Open questions

- Which Census Office model has the hole? It needs slice 5's census, or the owner's exact spot (coordinates from
  the console or the F1 overlay).
- S2 "capsule through the floor": first or third person, the view or the mesh? This decides whether slice 8 or a
  player-capsule slice is needed.
- Does rapier pair-update a newly inserted fixed collider against a *sleeping* body in every case
  (`collect_pairs_to_update` with `modified_colliders`)? Reading the source says yes. Slice 4's red-first test
  settles it.
- OpenMW's rules for hidden nodes in auto-generated collision, and its item handling (research lane).
