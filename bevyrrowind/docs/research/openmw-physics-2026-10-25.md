# OpenMW physics: how it tames Morrowind's assets (research note, 2026-10-25)

Purpose: guide fixes to our rapier-based physics (`sf_physics`, `mw_physics`) for four owner symptoms:
(1) loose items move by themselves on interior load, (2) a carried object vibrates and carrying pushes the player
capsule through the floor, (3) player falls through some floors (Seyda Neen, Census and Excise Office), (4) a loaded
save sometimes spawns the player too low (falls into the void).

Sources (all read, nothing built or run):
- OpenMW master, shallow clone, commit `4e718b500ffee6657db90297d7b8a08a57d439bc` (2026-10-23). Line numbers below
  are at this commit. Paths relative to repo root.
- Jolt MR: gitlab.com/OpenMW/openmw/-/merge_requests/3919 ("Jolt physics implementation", open, 140 files,
  branch `jolt-physics`; head commit `953ff1c3`, 2026-03-15; predecessors !4680 and !5153 closed). Code read from
  `refs/merge-requests/3919/head`. Reviewer discussion was NOT readable (notes API returns 401, page HTML has no
  comments): see Gaps.
- Industry cross-check is forum/blog grade, flagged as such.

GPL note: OpenMW is GPL-3.0-or-later like us, so porting code is allowed; below I mostly describe algorithms.

## 0. Headline findings (read this first)

1. **OpenMW has no dynamic items at all.** Every placed reference (items, furniture, statics) is a static,
   non-moving collision object. Only actors (kinematic boxes driven by a custom solver) and projectiles move. Nothing
   in OpenMW can "move by itself on cell load" because nothing is simulated. Authored placement is treated as ground
   truth. (Confirmed: `apps/openmw/mwphysics/object.cpp` builds a plain collision object; `physicssystem.cpp:addObject`
   line 401. Dropped items: `mwworld/worldimp.cpp:dropObjectOnGround` line 2033 raycasts the *render* scene once and
   places the item at the hit Z; it never simulates.)
2. **The Jolt MR does not change that.** It reserves layers `DYNAMIC_WORLD` ("sweet roll"), `DEBRIS`, `SENSOR` in
   `apps/openmw/mwphysics/joltlayers.hpp`, but no code creates a dynamic world body: the only
   `EMotionType::Dynamic` use is projectiles (`projectile.cpp:31`). Actors are still kinematic, still moved by
   `MovementSolver`. So there is **no OpenMW precedent for sleep thresholds / settling passes for items.** For items we
   must rely on rapier practice and Skyrim/Havok lore (section F).
3. **Character "does not fall through floors" is a stack of cheap rules, not one trick**: synchronous cell load,
   discard first frame, trace-down on load, grounded-by-default from saved fall height, depenetration ("unstuck"),
   swept convex casts only, 62-unit snap-down, and a "lowest point in cell" void guard that teleports home. Section B/D.
4. **Collision shape selection is mostly "triangle mesh everywhere"**: per-geometry BVH triangle meshes in one compound
   per NIF; no convex hulls. Actors use a single box from the NIF "Bounding Box" node. Section A.

## A. Collision shapes from NIF

Files: `components/nifbullet/bulletnifloader.cpp` (328 lines), `components/nif/node.cpp` (geometry -> mesh),
`components/resource/bulletshape.{hpp,cpp}`, `components/resource/bulletshapemanager.cpp`.

### A1. Which geometry collides (`BulletNifLoader::load` line 34, `handleRoot` 110, `handleNode` 185, `handleGeometry` 266)
What OpenMW does (pre-Gamebryo = Morrowind NIF, `handleRoot` lines 137-180):
- Reads `NiStringExtraData` on the root node. `"MRK"` => `mHasTriMarkers`, later skips any geometry whose name starts
  with `"Tri EditorMarker"` (line 274). `"NC"` prefix (case-insensitive) => no physical collision: sets
  `mVisualCollisionType = Default` ("NCC" with an upper-case third C => `Camera`, collides only with the camera).
  So `NCO`, `NC`, `NCC` all fall under the `NC` prefix test. `"RCN"` => search for the RootCollisionNode recursively
  instead of only direct children.
- `findRootCollisionNode` (`components/nif/node.cpp:209`) searches **children in reverse order** ("yes, this search needs
  to be reversed" comment), returns the LAST RootCollisionNode.
- If a RootCollisionNode exists and has children: only geometry under it collides (`mGenerateCollision` turned on
  when `handleNode` reaches that node, line 214-232). If none exists: **fallback = all rendered geometry collides**
  (`mGenerateCollision = true`, line 173). If it exists but is empty: treated as the NCC flag, the render geometry
  becomes camera-only collision (lines 174-179).
- Extra/unexpected RootCollisionNodes inside the tree are treated as normal visible geometry (lines 218-227, logged).
- `AvoidNode` subtrees go into a separate `mAvoidCollisionShape` (navmesh avoidance), not into the physical shape.
- `NiSwitchNode` and `NiFltAnimationNode`: only the first child collides (lines 258-261). `NiCollisionSwitch` with
  inactive flag: skipped (line 188).
- **Hidden nodes are NOT skipped** (no `isHidden()` check anywhere in the loader; I read the whole file). Only the
  named Tri EditorMarker case is dropped.
- Geometry that is skinned: `args.mAnimated = false` (line 277), and skin partitions are used for triangle lists.
- Only `NiTriShape`/`NiTriStrips` produce shapes; `NiLines`/`NiParticles` return null (`node.cpp:389,394`).
Why it matters: our symptom 3 (holes in floors) can come from taking only the wrong subset: the reverse-order
"last RootCollisionNode wins" rule and the "no RCN => everything collides" fallback are exactly the cases that break
if one picks "first RCN" or "no RCN => no collision". Also markers (`Tri EditorMarker`, `MRK`) must not collide or
items get blocked by invisible volumes.
Recommendation: **PORT** the whole decision tree (it is 70 lines) into `mw_nif`/`mw_physics`; verify against the
Census and Excise Office mesh specifically.

### A2. Triangle mesh vs convex vs compound
- Every collidable geometry becomes a `btBvhTriangleMeshShape` (`Resource::TriangleMeshShape`, quantized AABB
  compression, BVH built eagerly); all geometries of one NIF go into one `btCompoundShape` with child transforms
  (`handleGeometry` 309-325). No convex decomposition, no hulls, anywhere in the object path.
- Triangle sources: `NiTriShape::getCollisionShape` (`node.cpp:306`): vertices added with `findOrAddVertex(..., false)`
  (no merging), triangle lists; **strip conversion drops degenerate triangles** where two indices are equal
  (`stripsToBtTriangleMesh`, `node.cpp:36-58`) and alternates winding by parity. Plain index lists are NOT filtered
  (line 30-34), i.e. zero-area triangles from index lists are kept (Bullet tolerates them). Empty mesh => null shape
  (geometry skipped).
- Jolt MR differs: `JPH::MeshShapeSettings::Create()` fails on degenerate/duplicate triangles, so the loader catches the
  error, calls `Sanitize()` and retries, logging only if it still fails (`components/nifjolt/joltnifloader.cpp:375-390`,
  same in `components/resource/physicsshapemanager.cpp:150`). Build quality `FavorRuntimePerformance` (line 120).
Why it matters: rapier's `TriMesh` with validation flags (e.g. `FIX_INTERNAL_EDGES`, `DELETE_DEGENERATE_TRIANGLES`)
behaves like Jolt here. Degenerate triangles are a plausible source of bad contact normals ("snags", floor holes).
Recommendation: **ADAPT**: sanitize on bake (we have `mw_bake`), cache the cooked trimesh, keep one trimesh per
geometry and a compound per NIF; don't invent convex hulls for statics.

### A3. Bounding box ("Bounding Box" node) is for ACTORS only
- `findBoundingBox` (`bulletnifloader.cpp:85`): the first node (depth-first across roots) named `"Bounding Box"`
  (case-insensitive) with a `BOX_BV` volume and all extents > 0 gives `mCollisionBox{extents, center}`; otherwise warn.
  It is not added to `mCollisionShape`; statics never collide with it.
- Actor construction (`apps/openmw/mwphysics/actor.cpp:22-100`): half extents = bbox extents. If an NPC-less creature
  has no box, autogenerate from the shape AABB (comment "we can not create actor without collisions - he will fall
  through the ground", lines 40-62). Box shape for everything, with a cylinder variant for roughly square footprints
  (`extRatio` within 1/1.1..1.1 and no XY mesh translation), margin 0.001 (line 98: "make sure bullet isn't using the
  huge default convex shape margin of 0.04"). Actor collision object is `CF_KINEMATIC_OBJECT`, `DISABLE_DEACTIVATION`.
Recommendation: **ADAPT**: we already use a capsule; keep it, but copy the "no box => derive from mesh AABB, never
create a body-less actor" rule and the near-zero margin lesson.

### A4. Animated collision (doors, platforms)
- A geometry is "animated" if the file name starts with `x` (`pathFileNameStartsWithX`, line 22, "xmodel.nif has
  keyframes in xmodel.kf") or its node has an active `NiKeyframeController`/`NiPathController`/`NiRollController`
  (`handleNode` 191-207). Animated children are indexed (`mAnimatedShapes: nif record index -> compound child index`).
- Each physics frame `PhysicsSystem::stepSimulation` (`physicssystem.cpp:689`) calls `Object::animateCollisionShapes`
  (`object.cpp`, ~line 105-150) for animated objects: look up the scene node by record index (cached path), read its
  world matrix, `orthoNormalize`, update the compound child transform and local scale, then `updateSingleAabb` so the
  broadphase follows. Doors are a separate layer (`CollisionType_Door`) and rotate through `Object::setRotation` +
  `commitPositionChange`; collision with doors is what actors test via the movement mask.
Recommendation: **ADAPT** later (Phase 3). For now: doors/platforms = kinematic position-based colliders updated at
the fixed step from the animation pose, never dynamic.

### A5. Scale and sharing
- Scale: `handleGeometry` splits the node transform: `getScale()` goes to the shape (`ScaledTriangleMeshShape` /
  `setLocalScaling`), the rest is `orthoNormalize`d into a rigid transform (lines 284-307). **No explicit handling of
  negative or mirrored scale** (the orthonormalize would fold it; I found no guard) and no clamp for tiny/huge scale
  beyond Bullet's own tolerance. The Jolt MR treats "not uniform and close to 1" as needing a `ScaledShape`
  (`isScaleUniformAndCloseToOne`, joltnifloader.cpp:~388) and notes "Animated mesh scaling" / "Scaled meshes have
  incorrect navmesh" as todos (checked done in the MR text).
- Reference scale (ESM `scale`) is uniform per placement: `Object` ctor sets `mShapeInstance->setLocalScaling` (object.cpp
  ~line 22). The scaled wrapper shares the BVH child.
- Sharing: `BulletShapeManager::getShape` (`bulletshapemanager.cpp`, `getShape` ~line 90) caches the parsed shape per
  VFS path (ref-counted object cache with expiry); `getInstance`/`makeInstance` (`bulletshape.cpp`) creates a
  `BulletShapeInstance` per placement by `duplicateCollisionShape`: compound is deep-copied, but triangle meshes become
  `btScaledBvhTriangleMeshShape` wrapping the **shared** BVH. So N instances cost one BVH plus N small compounds.
  Pre-instances are cached when a cell is preloaded.
Recommendation: **PORT the idea**: one cooked trimesh per (mesh path), instances reference it with a scale in the
collider (rapier: shared `SharedShape` + per-collider scaling or bake-time scale only when non-1). For negative scale,
decide explicitly (flip winding) rather than leaving it to chance.

## B. Actors on cell load and on save load

### B1. Loading is synchronous; physics is skipped on the first frame
- Cell loading (`mwworld/scene.cpp:Scene::changeToInteriorCell` ~line 934, `loadCell`) runs under a loading screen on the
  main thread: all `Object`s are created (`addObject` ~line 147) before the player is placed. There is no frame where
  the player simulates against a half-built cell.
- `World::mDiscardMovements` is set true on any cell change (`worldimp.cpp:822,862,885`) and passed as `skipSimulation`
  to `PhysicsSystem::stepSimulation` (`worldimp.cpp:1387`; `physicssystem.cpp:716` calls `resetSimulation`) so the
  first frame after load runs no actor movement. Also `Actor::updatePosition` sets `mSkipSimulation = true`
  (`actor.cpp:166-174`) so the next solved position is not applied when the position was just set externally.
Why it matters (symptom 4): the likely cause on our side is simulating before the cell's static colliders exist (or the
broadphase was not yet updated), so gravity runs against nothing. Recommendation: **PORT**: gate gravity/movement
behind a "cell collision ready" barrier plus one skipped frame after any teleport/load.

### B2. Save load: re-place at the saved position, then trace down once
- `mwstate/statemanagerimp.cpp` (~line 620-645): loads player cell, calls `changeToCell(cellId, savedPos,
  adjustPlayerPos=false, ...)`; fallback for a missing cell is the default exterior with `pos.z = 0` and
  `adjustPlayerPos=true`.
- `Scene::changePlayerCell` (`scene.cpp:852`): comment: "The player is loaded before the scene and by default it is
  grounded, with the scene fully loaded, we validate and correct this." It calls
  `mPhysics->traceDown(player, savedPos, 10.f)` (line ~866, only when the player's cell is unchanged from before,
  i.e. initial load): a short (10 unit) downward sweep that sets/clears the actor's on-ground flag. It does **not move**
  the player in that case; it only fixes the flag so the first frame knows whether to apply gravity.
- Initial on-ground flag: `Actor` ctor `mOnGround(getFallHeight() == 0)` (`actor.cpp:33`): a saved mid-fall player (saved
  fall height != 0) starts airborne; otherwise grounded, so a saved standing player never starts falling.
So OpenMW trusts the saved position (never auto-raises it unless the caller passes `adjustPlayerPos`) and relies on
(a) a loaded world, (b) grounded-by-default, (c) the unstuck pass below to recover from being inside geometry, and
(d) the void guard (B4) as last resort.
Recommendation: **PORT** "grounded unless saved fall height says otherwise" and "after load, do a short trace-down to
set grounded". Additionally **ADAPT** a 'sanity snap': if a saved position is inside geometry or above the lowest
floor by more than N, run the teleport placement below. Our "spawns too low" bug is most likely restore ordering (save
Z applied with an eye/feet offset, or applied before the capsule offset), so log the offset math.

### B3. Placement on teleport / door / COC: `World::adjustPosition` and `traceDown`
- `World::adjustPosition(ptr, force)` (`worldimp.cpp:1214-1254`): pos.z = max(z, terrainHeight) + 20 ("place slightly
  above terrain, will snap down"), then `PhysicsSystem::traceDown(ptr, pos, cellSize)` and take `min(z, traced.z)`.
  Called for the player on all door/COC/teleport paths with `force=true` (`scene.cpp:870-878`; `cellextensions.cpp`,
  `transformationextensions.cpp`), and for NPCs once when their cell activates (`mwmechanics/actors.cpp:1736`,
  `getPositionAdjusted` flag) and from AI travel/wander.
- `MovementSolver::traceDown` (`movementsolver.cpp:90`): convex-sweep the actor box from `pos` downward by `maxHeight`
  (`ActorTracer::findGround`, `trace.cpp:105`, with actor-vs-actor filtering). If nothing hit: stay at `pos`, not on
  ground. If hit, do an **infinitely thin ray** from the original position down, and if the ray hit point differs from
  the box hit by more than 35 units (squared 35*35) or the slope is unwalkable, trust the ray
  (`+ sGroundOffset = 1.0`). Comment: "Required for some broken door destinations in Morrowind.esm, where the spawn
  point intersects with other geometry if the actor's base is taken into account."
- `World::fixPosition` (`worldimp.cpp:1256`, script `FixMe`): try four directions 128 units away (ray test first for a
  free destination), raise 64, trace down, teleport. Used for stuck-in-geometry repair.
- Door teleport destinations are plain ESM positions; OpenMW does nothing special except `force=true` adjust.
Recommendation: **PORT** the two-stage ground find (swept shape then thin ray, 35-unit discrepancy rule) for our
spawn/teleport path; it is exactly the "intersecting door destination" case.

### B4. Fell-into-the-void guard (interior)
- `scene.cpp addObject` (line 147-161): for interiors each static object's AABB min Z updates `mLowestPoint`.
- `Scene::playerMoved` (line 575-610): if interior and `player.z < mLowestPoint - 90` (the constant `-90` is added),
  and actor collision is enabled, teleport to the cell's CoC/marker (`findInteriorPosition`) provided that marker is
  above the lowest point. Logs "Player position has been reset due to falling into the void" (OpenMW bug #1415).
Recommendation: **PORT** (30 lines). It converts symptom 3/4 from fatal to recoverable and is cheap insurance. Exteriors
have no such guard (terrain heightfield is assumed continuous).

### B5. Unstuck (depenetration) every simulation step: `MovementSolver::unstuck` (`movementsolver.cpp:486-584`)
Only runs when the actor intends to move. Algorithm: contact-test the actor box at its current position (`ContactTestWrapper`,
collecting normals/depths via `ContactCollectionCallback`, `movementsolver.cpp:~30-80`); if penetration > 0
(`sAllowedPenetration = 0`, deliberately), compute a summed push-out vector clamped per axis to the largest single
contact, then try in order: (1) the clamped push-out, (2) straight up by |push.z| ("fixes some bad coc floors"),
(3) straight up 10 units; accept the first that reduces penetration. `mStuckFrames` counter: after 10 frames stuck
and not having moved >10 units, give up (avoid fighting); while stuck, the solver forcibly treats the actor as
on flat ground (lines 423-427) because ground detection is unreliable inside geometry.
Recommendation: **PORT** as a depenetration step for the player capsule (rapier: our controller can call
`contact_pair`/`intersection` queries; or use rapier's KinematicCharacterController `autostep`+`snap_to_ground`
and add the upward fallback for bad floors).

## C. Items: OpenMW vs the Jolt MR

- Vanilla Morrowind: items are static references; physics only exists for actors and a few scripted moves
  (`MoveWorld`, `SetPos`, and Dehy-style "physics postponed" placement, `RefData::mPhysicsPostponed`, set by
  `PlaceAt*` scripts in `transformationextensions.cpp:534,580,618` so a freshly created non-actor does not get a
  collision object until positioned). OpenMW matches. Confirmed above (headline 1).
- Jolt MR (`!3919`): pure engine swap for statics/actors/projectiles. Declared motivations: backface collision
  detection, cylinder actor collider performance, missing projectile collisions. Body creation for objects uses
  `makePhysicsBodySettings(shape, position, rotation, layer)` with static motion (object.cpp:~25-40); no sleeping
  configuration, no CCD flags, no contact margins in the object path, because no object is dynamic. `cCollisionSteps = 1`
  (physicssystem.cpp:228), Jolt `Update` runs in a fixed-dt accumulator loop (line 1079-1082) while actors are still
  stepped by the Bullet-style `PhysicsTaskScheduler` + `MovementSolver`. Jolt capacities: `cMaxBodies = 65536`,
  `cMaxBodyPairs = 65536`, `cMaxContactConstraints = 10240` (lines 231-246), 25 MB temp allocator, 4 broadphase
  layers (WORLD, MOVING, DEBRIS, SENSOR). Mutable compound shapes only for animated or collision-switch NIFs.
- Edge cases the MR itself lists: new-game spawn "between the barrels and the staircase (wrong floor)", `placeatpc`
  multi-actor spawn broken (both checked done), scaled-mesh navmesh, animated mesh scaling, "broadphase layers need a
  bit of performance testing", batch add/remove of bodies during cell streaming as a future optimization. The MR's
  reviewers' threads were unreachable (Gaps).
- Takeaway for us: **no sleep/settle recipe exists upstream.** Items stay static in OpenMW for exactly the reason
  our symptom 1 exists: Morrowind placements interpenetrate tables, shelves, other items and the floor. If you
  make them dynamic, depenetration impulses launch them.
Recommendation for items (rapier, not from OpenMW): treat authored placement as truth. Spawn items as **fixed/kinematic
(or sleeping with zero velocity and `can_sleep` already true, `sleeping: true` in the rigid body builder)** and convert
to dynamic only on first interaction (touch, activate, hit). Never run a settling pass at load. If a settling pass is
ever wanted, do it at bake time and store the result, and run it with a very low max depenetration velocity.
Details in section F. **ADAPT** (design, not code).

## D. Character controller details that matter for "through the floor"

All in `apps/openmw/mwphysics/` unless noted.
- Constants (`constants.hpp`, `components/misc/constants.hpp`): `sStepSizeUp = 34`, `sStepSizeDown = 62`,
  `sMaxSlope = 46` degrees, `sGroundOffset = 1.0`, `sCollisionMargin = 0.2`, `sAllowedPenetration = 0`,
  `sMaxIterations = 8`, `sMinStep = 10`, `sMinStep2 = 20`, `GravityUnits = 9.8 * UnitsPerYard`, `MaxFallSpeed = -4005.5`
  units/s. Max fall step at 60 Hz is about 67 units: larger than a thin floor, so the solver must be a *swept* cast,
  never discrete overlap.
- All motion is a convex sweep of the actor shape (`sweepHelper`, `trace.cpp`: `convexSweepTest` with the actor's
  group/mask, `&= ~Actor` when probing ground). `ActorTracer::doTrace` tries a **short 2.1-unit trace first** and only
  does the full-length sweep if nothing is hit (big perf win on long traces against dense meshes; see the long comment
  at `trace.cpp:~63-76`).
- Slide loop (`MovementSolver::move`, `movementsolver.cpp:130-448`): up to 8 iterations; on hit, try `Stepper::step`
  if the hit is lower than 34 units (`stepper.cpp`: trace up, move forward, trace down, with three attempts: normal move,
  a 10-unit fixed move, a 20-unit shorter-lower fixed move: "just try to work around problems with vanilla Morrowind
  assets", `sDoExtraStairHacks = true`); else slide along the plane (remove normal component), back off by 0.2 along the
  movement path, then nudge `2*margin` along the normal ("reduces getting stuck in concave geometry like gaps above
  railings in Ald'ruhn"); seam logic projects onto the crease between the last two planes; "seenGround" turns steep
  walls' normals horizontal so you don't crawl up their bases; slopes steeper than 46 deg block upward velocity.
- Ground snapping (lines 379-430): if inertia.z <= 0, sweep down by `2*sGroundOffset` (+62 if previously on ground) and
  snap to hit + 1 unit; if closer than 1 unit, push up with a short upward trace and average. On a non-walkable
  slope, no snap and gravity continues. Gravity applies only when not on ground (or on a slope).
- Hit objects that are actors never count as ground except "float on top" case; this is where carried bodies matter
  (below).
- Margin: actor shapes use margin 0.001, mesh margins are tiny; this is what lets actors fit through Morrowind's tight
  doorways.
Why it matters for symptoms 2-3: (a) swept casts + 1-unit offset above ground avoid sub-surface starts; (b) the thin-floor
protection is the combination of sweep + unstuck + void guard; (c) the actor is a KINEMATIC body: nothing else pushes it,
so carrying an object can never press the capsule into the floor in OpenMW.
Recommendation: **ADAPT**: rapier's `KinematicCharacterController` offers autostep/snap-to-ground/max slope; set
`offset` ~0.2, `max_slope_climb_angle` ~46 deg, autostep 34 with `include_dynamic_bodies: false`, snap-to-ground 62, and
add the ray-vs-box discrepancy check from B3 plus the unstuck fallback ladder from B5. **PORT** the "short trace first"
trick if profiling shows sweep cost against large trimeshes.

## E. Performance (what is published or visible in code)

- Fixed step: `PhysicsSystem` `mPhysicsDt(1/60)` (`physicssystem.cpp:91`; env `OPENMW_PHYSICS_FPS` override in the MR).
  `PhysicsTaskScheduler::calculateStepConfig` (`mtphysics.cpp:451`) allows at most 2 catch-up steps, 1 if the measured
  per-step cost exceeds 95 percent of dt, more if cheap; beyond that it runs one variable-dt step. Positions are
  interpolated between steps (`interpolateMovements`, `mtphysics.cpp:~270`, `setSimulationPosition`).
- Threads: async physics worker threads (`Settings::physics().mAsyncNumThreads`), shared-lock policy
  (`LockingPolicy`), main thread waits only to sync (`applyQueuedMovements` -> `prepareWork`, line ~498). Position and
  scale changes to objects are queued and committed in `Object::commitPositionChange` at a safe point. Broadphase is
  `btDbvtBroadphase` (`mtphysics.cpp:310`); Jolt MR uses its layered quadtree broadphase with 4 layers; WORLD-layer static
  bodies never test against each other.
- Actors simulate only if they moved (`unstuck` early-outs when no movement intended; `Sim` skipped when
  `setPosition` reports no change).
- No published per-frame numbers or "statics per cell" figures are in the tree or the MR text; I found none (Gap).
  Static colliders are plentiful (one per reference, exteriors have hundreds per cell) and the design assumption is that
  static BVHs are cheap and queries are few (only actors + rays).
Recommendation: **ADAPT**: our fixed-step + interpolation already matches; consider skipping sweeps for idle actors and
the short-trace-first optimization. Measure ours before copying their threading.

## F. Industry cross-check (forum/blog-grade evidence, flagged)

1. Items flying on cell load in Skyrim/Fallout (Havok): community explanation is that clutter is authored with slight
   overlap, so the engine resolves the penetration on cell load and objects hop; frame rate above 60 fps made it worse
   until `fMaxTime` / fixed-step patches (SSE Display Tweaks, High FPS Physics Fix). Sources:
   Steam community thread, steamcommunity.com/app/489830/discussions/0/7091547146188011047; Nexus thread
   forums.nexusmods.com/topic/2359464; mods "SSE Display Tweaks", "High FPS Physics Fix". I did not find a
   primary Bethesda/Havok source for any sleep-on-load fix. Lesson that is consistent with OpenMW: the safe design is
   do not simulate authored placements until disturbed, and keep the step fixed.
2. Held-object jitter, common fixes (blogs/forums, e.g. bugnet.io Unreal guide, itch.io pickup thread, Stride manual
   on interpolation, Skyrim grab-mod changelog nexusmods.com/skyrimspecialedition/mods/43930):
   - A held body that stays simulated fights its attachment: collision pushes it off, the attachment snaps back. Fix:
     disable collision (or simulation) between holder and held object, or make the held object kinematic.
   - Move the handle/target in the fixed physics step, not the render tick; interpolate for display (Stride manual).
   - If force-driven (spring/PD, velocity drive), cap max velocity/force and add damping; Skyrim grab mod fixed
     player jitter by moving held objects to a layer that does not collide with the holder and raising damping.
   - Well-known pattern (Half-Life 2 physcannon / Garry's Mod) is a damped spring on a dynamic body plus no collision with
     the player; I did not find a primary source in my search, treat as general knowledge.
3. For rapier specifically (my assessment, verify by experiment): kinematic-position-based body for the held item with
   `collision_groups` that exclude the player capsule, moved by `set_next_kinematic_position` in the fixed step; the
   capsule controller must ignore it (`QueryFilter::exclude_rigid_body`). If the object must collide with the world,
   do a shape-cast from last to target pose and stop at the first hit (so it can't be pushed into walls and
   floors), and never let the held object's contacts feed back into the player body.

## Mapping to our symptoms

| Symptom | Likely cause (hypothesis) | OpenMW evidence | Fix direction |
|---|---|---|---|
| 1 items move on interior load | dynamic bodies depenetrate their authored overlaps | OpenMW: items are static; Jolt MR adds none | Start items fixed/sleeping, wake on interaction |
| 2 carry vibrates / capsule pushed through floor | held body is dynamic and collides with capsule; capsule is dynamic or resolved by contacts | OpenMW actors are kinematic; held items do not exist | Kinematic held, groups exclude capsule, move in fixed step, shape-cast to targets |
| 3 fall through floor (Census office) | wrong shape subset (RCN rule), degenerate tris, thin single-sided floor missed by discrete test | A1, A2, D | Port RCN rules, sanitize, swept casts, unstuck ladder, void guard |
| 4 spawn too low on load | simulating before colliders exist, or Z/capsule-offset applied wrongly | B1, B2, B3 | Cell-ready barrier + 1 skipped frame, grounded-by-default, trace-down, void guard |

## Ranked: what we should copy (max 10)

1. **Items are static until disturbed** (OpenMW never simulates placements; Jolt MR still doesn't). Spawn fixed or
   sleeping, wake only on interaction; no settling pass at load. (Symptom 1.)
2. **Barrier + skipped first frame**: no gravity/movement until the cell's static colliders are in the query world; skip
   actor simulation on the first frame after any cell load/teleport (`mDiscardMovements`, `Actor::mSkipSimulation`).
   (Symptoms 3, 4.)
3. **Interior void guard**: track lowest static AABB Z per interior; if the player is 90 below it, teleport to the CoC/
   marker (`Scene::playerMoved`, `scene.cpp:575`). (3, 4.)
4. **Two-stage ground find on spawn/teleport**: swept shape trace-down, then thin ray, trust the ray if the two disagree by
   more than 35 units or slope is unwalkable; place at hit + 1 unit (`MovementSolver::traceDown`, `movementsolver.cpp:90`).
5. **Grounded-by-default from saved fall height + one trace-down after save load**
   (`Actor` ctor `actor.cpp:33`, `Scene::changePlayerCell` `scene.cpp:852`). (4.)
6. **NIF collision selection rules**: reverse-order RootCollisionNode, "no RCN => all geometry", empty RCN => camera-only,
   MRK/"Tri EditorMarker" skipped, NC/NCC flags, first-child-only for switch nodes, "Bounding Box" node for actors
   (`bulletnifloader.cpp:110-326`). (3.)
7. **Unstuck ladder**: contact test, clamped push-out, else straight up by push Z, else +10 up; stuck-frame counter forcing
   "on flat ground" (`movementsolver.cpp:486`). (3, 4.)
8. **Sanitize triangle meshes at bake** (drop degenerate/duplicate triangles, strips with repeated indices) and share one
   cooked trimesh per mesh path across instances with per-instance scale (`Sanitize()` in the Jolt loader,
   `BulletShapeManager`/`makeInstance`). Decide negative-scale policy explicitly (OpenMW leaves it undefined). (3.)
9. **Swept-only character motion** with 34 step-up, 62 snap-down, 46 degree slope, 0.2 margin, 1 unit ground offset, max
   fall speed 4005 u/s and a short-2.1-unit-trace-first optimization (`stepper.cpp`, `trace.cpp:doTrace`). (3, perf.)
10. **Carrying (not from OpenMW, from industry + OpenMW's kinematic-actor principle)**: held item = kinematic body in a
    group that excludes the player capsule, driven in the fixed step with interpolation and a shape-cast stop against
    the world; the capsule never receives contact forces from it. (2.)

## Gaps and confidence

- Jolt MR !3919 reviewer comments (edge cases, perf numbers): not retrievable (GitLab notes API 401; page text had no
  discussion). The linked comment id in the description is `1808174555`. A human with a GitLab login should skim the MR
  thread before relying on "no one raised item behaviour".
- No published OpenMW per-frame cost or statics-per-cell numbers found.
- Negative/mirrored/huge scale: described from code reading only (no guard found); I did not test with real assets.
- Hidden-node behaviour read from `bulletnifloader.cpp` only; `NiNode::isHidden` exists but is unused by the collision
  loader. Rendering may hide nodes that still collide.
- Industry section is forum/blog level; no primary Havok or Valve source consulted.
- rapier-specific recommendations are my reading of rapier's API, not verified against our `sf_physics` code beyond
  noting that `mw_physics` already has `PlacementClass::Dynamic` items and `carryable_lights`.
