# Interaction (c): mounts and vehicles (`sf_interaction`)

Status: **spec (TV0b, 2026-11-26)**; IN20 and IN11a-c2f built, the rest design. Replaces the 2026-11-11 design
(moved out of [interaction.md](interaction.md) for size on 2026-11-25). Binding design: [travel roadmap](../../../docs/roadmaps/travel.md)
§3 decisions 3-6, §7 owner answers Q2, Q5, Q6. The shared core (defs, slots, claims, "Using a slot"), "Budgets" and
"Build slices" stay in [interaction.md](interaction.md); routes, ride phases, leaving and wait rules are
[travel.md](travel.md) "Rides". This file owns seats, decks, based movement, passenger state and the mount body.

## Goal

- Any character (player, NPC, remote player) stands, walks, jumps and fights on a moving body (lift, cart bed,
  strider's back, ship's deck) that turns, bobs and rolls, at 30-144 fps alike, with no drift or jitter.
- A character sits, stands or rides astride a seat on a moving owner, enters and leaves by a side, and may leave
  anywhere with the owner's motion, overboard included. Who is aboard what, and where, is saved and replicated.
- Mounts steer like RDR2 horses with procedural legs (design only; build after TV-R2).

**Non-goals.** Routes, departure, skip, leave-and-wait, encounters: travel.md. Ships (sailing, crews): vessels spec,
frozen. Physics vehicles (kind 3), mounted combat, flying mounts, mount bond: later. **Mount research TV-R2 is parked
(owner 2026-11-25)**: the mount sections fix interfaces and rules, never gait numbers; TV-R2's questions are at the end.

## Owner decisions

| Date | Decision | Where applied |
|---|---|---|
| 2026-11-11 (interaction Q4) | Mounts first as the general mechanic (interact by a mount, climb-on clip, drive it); mounted combat later | Mounts |
| 2026-11-11 (interaction Q6) | Kimodo clips for the player too (sit, stand, rider) | Seat IK |
| 2026-11-25 (travel Q2) | Always leave any vehicle anywhere, a ship at sea included (allowed, not advised) | Seat links; Based movement |
| 2026-11-25 (travel Q2) | Ships are walkable moving decks (Sea of Thieves), never a disguised mount; characters put no force on a deck | Motion x place |
| 2026-11-25 (travel Q5) | RDR2 horses (speed, inertia, procedural legs); sit or stand naturally on creatures and vehicles, walk on decks; plan all now; instant teleport until it lands | Whole spec |
| 2026-11-25 (travel Q6) | A boat left at sea slows, stops, waits, goes on, like land vehicles (wait = tunable) | travel.md "Rides" (linked) |
| 2026-11-25 | IN7's rail half dropped (route motion is TV4); standing-idle strider passenger dropped (walk the deck); IN11 = generic moving-platform support; TV-R2 and ships frozen | Motion x place; Seat links |

## Reuse check (CONTRIBUTING rule 0)

| Candidate | Checked | Fit |
|---|---|---|
| Bevy `ChildOf` hierarchy | `bevy_ecs` 0.20.1 `hierarchy.rs:107`; `bevy_transform` 0.20.0 `plugins.rs:38-46` (propagation in `PostUpdate`) | **Not for attach**: propagation runs after `FixedUpdate` on the drawn `Transform`; gameplay poses are `WorldPos` on the tick (net rules 1, 3, 7) and riders keep their own `big_space` cell. Attach = base-local pose in tick state; the drawn pose composes it (IN21) |
| Bevy `Time<Fixed>` | `bevy_time` 0.20.0 `fixed.rs:105` `from_hz`, `:172` `set_timestep_hz` | **Used**, one owner of the rate (IN11c2g). Bevy has no character controller |
| rapier3d `KinematicCharacterController` | 0.36.1 `control/character_controller.rs:~545` | **No**: `kinematic_parent.velocity_at_point` fixes the normal part and adds a tangent drag; no yaw carry, base-local pose, crush or deck frame. `velocity_at_point` itself is reused (IN20) |
| avian3d `kinematic_character_3d` | 0.7.0 example | **No**: move-and-slide without moving platforms; not our backend |
| bevy-tnua | 0.29 newest found (Bevy 0.18, no 0.20 release) | **No**: floating rigid-body controller reading the ground velocity; would replace `sf_controller` (batched probes, pure net-ready step) |
| Bevy seat/mount crates | crates.io + GitHub, 2026-11-26 | **None** for seats or mounts; `saddle-vehicle-ground-vehicle` (Bevy 0.18, suspension) to re-check for kind 3 |
| Ours | `sf_controller` mover, `sf_action` seat path (IN2), `sf_interaction` claims | **Built on**: seats extend IN2's path, never a second one |

## Industry comparison

| Reference | What it does | Our gap | What we take |
|---|---|---|---|
| Sea of Thieves | Walk a pitching, rolling ship in its frame; crew and props aboard stay put; overboard hands you to world space | Pitch/roll, airborne frame | Deck frame (IN11c3), no force on decks, base-relative draw and replication |
| UE `MovementBase` (CMC) | `FBasedMovementInfo` (base, relative pose) replicated; base ticks first; `bImpartBaseVelocity*`, `bImpartBaseAngularVelocity`, `bIgnoreBaseRotation` | Relative replication, save | Base first (done IN11b), point velocity on leave (IN11a), relative pose in saves/snapshots (IN24) |
| UE Mover 2.0 | Based movement as rollback-friendly sim state | - | Passenger pose is predicted state, not presentation |
| Jolt `CharacterVirtual` | `UpdateGroundVelocity` per step; `mCanPushCharacter`, mass, max strength | Dynamic bodies don't push | Mass-threshold pushes (IN11c2d) |
| Star Citizen physics grids | Ships are local physics zones; occupants simulate inside | No local frame airborne | Deck frame volume (IN11c3b) |
| RDR2 horses | Momentum, gait ladder on sprint taps, turn radius by speed, mount by side, whistle, stamina, buck-off | No mount body | Mount profile, `RiderToMount`, TV17-TV18 |
| Elden Ring Torrent | Instant mount, light inertia, mounted moveset | - | Short mount enter clips; mounted combat later (combat.md) |
| GTA | Seat indices (driver, passengers), enter by nearest side, jump out of a moving vehicle with a tumble | - | `Control` per slot, `pick_entrance`, jump-off with point velocity |
| UE Control Rig; Bereznyak "IK Rig" (GDC 2016) | Procedural legs for any leg count, pelvis/spine from foot heights | Two-legged `FootPlacement` | N-leg placement (IN26) |
| Veloren mount links | `Link`: `create` / `persist` / `delete`; riders on creatures and on volumes (airships) | - | `SeatLink` lifecycle (IN7) |

Gaps go in [industry-audit.md](industry-audit.md) when a row ships without its "what we take".

## (c) Mounts and vehicles (seats, attach, control hand-off)

### 1. Vehicle = motion x place (decision 3)

```rust
// Illustrative, not compiled. A field of InteractableDef (interaction.md "Definitions").
pub struct VehicleDef { pub motion: MotionSource, pub decks: SmallVec<[DeckDef; 2]> }  // seats = SlotDefs (section 4)
pub enum MotionSource {
    Route,    // kinematic body; pose = f(route, s_m, tick), bob/roll included (travel TV4 writes it)
    Mount,    // the owner is a character: its mover writes the pose (section 8)
    Physics,  // dynamic body (kind 3, later: buoyancy 3.7 or a wheel controller)
}
pub struct DeckDef { pub frame_volume: Aabb /* owner-local, IN11c3b */, pub nav: bool /* IN22 */, pub board_points: SmallVec<[Iso; 2]> }
```

- **Old kinds map onto it**: rail vehicles (1) = `Route` + deck/seats (IN7's rail half dropped); creature mounts (2) =
  `Mount` + an `Astride` seat; physics vehicles (3) = `Physics`; walk-on vehicles (4) = any motion with a deck.
- **No character force on a deck** (owner Q2): deck bodies carry a `DeckBody` layer bit and the capsule's contact
  pair with them is filtered (physics.md), so a crowd at one rail never lists a ship. Decks carry characters only by
  based movement; a moving deck's walls still push them (IN11c1, IN11c2d).
- **Route bob/roll is tick-pure** (travel.md "Rides"): peers agree and the deck's twist per tick is exact.
  Interaction only reads the body through `GroundHit` (IN20).

### 2. Based movement (decision 4; IN11)

**As built** (2026-11-25): `GroundHit` carries `body`, point `velocity`, `angular_velocity`, `base_pose`
(`sf_controller/src/probe.rs:69`, `:97`). `refresh_base_motion` (`sf_physics_bevy/src/character.rs:133`) turns the
base's pose change since the ground cast into the tick's twist (a dropped physics step rides nothing, a doubled one
both). `Locomotion::ride_base` (`sf_controller/src/mover.rs:406`) moves the feet about the base's instantaneous
centre and turns yaw with it; leaving keeps the horizontal point velocity in `inherited` (`:322`), `launch` (`:389`)
adds the base's vertical speed to a jump. `KinematicMover::carry` sweeps the ride and applies `GroundProbe::contacts`
pushes (`sf_physics/src/probe.rs:277`, kinematic bodies only). Vertical rides are swept (`headroom`); a squeeze past
`move.crush.depth_m` (`Squeeze` `mover.rs:246`, `Crush` `:268`) gives one `CharacterCrushed` per episode. Physics
steps before movers (`character.rs:104`) at one rate (`PhysicsParams::hz`). Numbers: [ledger physics](../../../docs/ledger/physics.md)
2026-11-25; limits: [wiki](../../../docs/wiki/engine/locomotion-and-action.md) "Known limits".

**IN11c2g one fixed tick.** `SfActionPlugin` inserts `Time<Fixed>` from `TICK_HZ` (`sf_action_bevy/src/lib.rs:85`),
`PhysicsPlugin` from `PhysicsParams::hz` (`sf_physics_bevy/src/lib.rs:254`): the last added wins. Design: with
physics present `PhysicsParams::hz` is the source; `SfActionPlugin` inserts `Time<Fixed>` only if absent; a change
of `PhysicsParams::hz` calls `set_timestep_hz` the same frame. Action rule data counts ticks at `TICK_HZ` (net rule
1), so a differing app rate logs one startup error naming both; rate sweeps run physics alone or set both.

**IN11c2c3 crush follow-ups.** The game's action-body host calls `sf_physics_bevy::crushed` after
`ActionSystems::Move` (`mw_walkers_bevy/src/physics.rs` near :517), so the player gets `CharacterCrushed`. A
squeezed body stays grounded for the chooser (no fall state while pinned). The episode ends when the depth stays
under `move.crush.release_depth_m` (the lift reverses); a pusher stopped while overlapping keeps it `Held`.
`move.crush.depth_m` joins the CATALOG. Consequences are host rules: travel halts a route on a passenger crush
(TV4), the adapter maps damage.

**IN11c2d mass-threshold push** (Jolt `mCanPushCharacter`). `PhysicsProbe::contacts` also returns awake dynamic
bodies of mass >= `move.push.dynamic_mass_kg` with closing point velocity, pushing like kinematic ones (gap = skin),
and emits no `PhysicsEvent::Hit` for them (the solver owns hits: the cause of IN11c2a's P5 double hits). Lighter
bodies stay the solver's. `DeckBody` bodies take no character force.

**IN11c2e ride in every state.** `Docked` edges on a moving owner are owner-local and ride its tick delta. Warp moves
(mantle, vault, seat enter) onto a moving target re-resolve it owner-local each tick. `Fly` and `Swim` never ride
(water and air are world-fixed); contacts still push a swimmer off a hull. A jump keeps the full point velocity at
take-off (linear plus the tangential part of the base's spin; UE impart base velocity + angular), so on a base in
constant linear motion it lands on the take-off spot exactly.

**IN11c3a pitching and rolling bases.** `ride_base` applies the full rigid delta `T_now * T_prev^-1`: feet follow,
the capsule stays world-upright, facing takes only the twist about world up (swing-twist), the walkable test uses
the deck normal against world up. Idle on a deck rolling within the walkable slope: no slide. Reports ms.

**IN11c3b deck frame and rebase.** Inside a deck's `frame_volume` (+ `move.base.frame_margin_m`) a character stays
in the deck's frame even airborne (Star Citizen physics grid): airborne motion integrates base-local, so jumps on a
turning, pitching deck land as on land. Leaving the volume (overboard) hands it to world space with its point
velocity there. A floating-origin `rebase` while riding (physics, whole cell) shifts base and rider alike;
`refresh_base_motion` takes its delta in one origin, so a rebase never pops a rider.

### 3. Deck presentation (IN21)

Today the drawn body lerps the last two **world** tick poses; on a turning deck that cuts chords against the deck's
own interpolated pose, and the rider swims.
- **Drawn pose**: `drawn = base_drawn * lerp(local_prev, local_now, overstep)` (`base_drawn` = the deck's physics
  `pose_at`); the render-correction spring runs base-local too. At rest the drawn offset to the deck is constant.
- **Foot IK**: `FootPlacement` probes the deck at its drawn pose and keeps plant/lock points base-local.
- **Camera** (`sf_camera`): the arm collides with the deck's drawn pose; the look yaw is carried by the base's yaw
  delta each tick (`camera.base.follow_yaw`, as UE does not ignore base rotation by default), inside `Command.view`;
  deck pitch and roll never tilt the horizon (comfort).
- Owner check IN-11 on a `level:` greybox deck (turning, bobbing, rolling) at 30 and 144 fps.

### 4. Seat links (IN7; decision 5)

A seat is a `SlotDef` (interaction.md) with `occupancy: Attach` and `control: Some(Driver | Passenger)`:

```rust
// Illustrative, not compiled.
pub enum Occupancy { Pose, Attach(AttachDef) }
pub struct AttachDef { pub mode: SeatMode /* Sit | Stand | Astride */, pub jump_off: bool }
pub struct SeatLink { pub seat: SlotRef, pub rider: UserKey }   // stable ids (net rule 7)
```

- **Modes.** `Sit` (bench, cart seat), `Stand` (a pinned standing place at a rail where there is no deck: a cart's
  tail board, a gondola's stern for its operator), `Astride` (saddle). Walking passengers walk the deck (section 2).
- **One path: IN2's sit** (interaction.md "As built (IN2a)", "(IN2b2)"): claim, `pick_entrance`, `WarpTarget::Slot`,
  `Owner::Seated` + `ExitOn::Stand`, `SeatEntered` -> `occupy`, `SeatExited` -> `release`. Attach adds: `SeatSense`
  poses owner-local and re-resolved each tick; the pinned root is `owner_pose * slot.local` every tick, never
  simulated against the owner; the seated shape is filtered against the owner.
- **create** = the claim turning Occupied; refused as claims are (taken, `UserBusy`, dead rider). **persist** each
  tick: owner and rider alive, seat registered, same worldspace, else delete with a forced stand-up. Attach slots
  are exempt from `OwnerMoved` invalidation (`interact.claim.move_tolerance_*`): moving is what they do.
- **delete**: below `interact.seat.jump_off_speed_mps` of seat point speed, exit by `EndPoseFree` (entry side first,
  then any side, then the nearest navmesh point within `interact.seat.exit_search_m`; an exit onto a deck is based at
  once). At or above it, with `jump_off`, the rider is released with the seat's point velocity and lands through the
  mover, anywhere (owner Q2; overboard into `Swim`).
- **Events** (net rule 8): `Mounted { tick, rider, seat, mode }`, `Dismounted { tick, rider, seat, how: Exit(side) |
  JumpOff | Forced(reason) }` (interaction.md "Events").
- **Control.** `Driver` routes the rider's `Command` to the owner (a mount via `RiderToMount`; a kind 3 controller
  later); `Passenger` looks (fights with mounted combat). Route vehicles have no driver seat: the operator sits or
  stands in a crew-tagged `Passenger` seat while TV4 moves the body.

### 5. Seat IK (IN23)

Frame-rate presentation from the owner's drawn pose with the existing ops (`LimbIk`, `ChainIk`, `FullBodyIk`,
`LookAt`; [animation](animation.md)) and the slot's `IkTargets`. `Sit`: pelvis to the slot frame, feet to footrests or
the probed deck, hands to armrest or rail. `Stand`: feet locked base-local, hands to the rail, an additive lean from
the base's local acceleration. `Astride`: pelvis to the saddle point on the mount's spine bone, legs two-bone to
stirrups, hands to the reins point on its neck; loops gait-matched (`seat.astride.loop.<gait>`, phase from the
mount). Clips (Kimodo, canonical body, IK-fitted per body: interaction.md "Using a slot" 3) use slot ids
`seat.<mode>.{enter.<side>,loop,exit.<side>}`; mount enters are short (Torrent-like).

### 6. Passenger state (IN24)

```rust
// Illustrative, not compiled. Gameplay state, serde, on the character.
pub struct Aboard { pub vehicle: VehicleKey /* PlacementId | spawn id */, pub at: AboardAt }
pub enum AboardAt { Seat(SlotRef), Deck { deck: u8, local: Iso, local_velocity: Vec3 } }
```

- **Aboard** = on a deck body, inside a deck's frame volume, or holding a seat link. `Boarded { tick, who, vehicle }`
  on the first tick; `Left { tick, who, vehicle, how }` after `interact.vehicle.leave_s` off deck and volume (a jump
  is not a leave). travel's `RideState.passengers` lists who; this says where.
- **Saves** ([save.md](save.md) catalog): the player's `CharacterSave` extension blob `interaction` v1 holds
  `seated: Option<SlotRef>` (IN2's to-do, static furniture) and `aboard: Option<Aboard>`; an NPC's goes in `sf/ai`
  activity state. The vehicle's pose is its owner's: `sf/travel` `RideState.s_m` for a route (pure pose), the actor
  location for a mount, the overlay's moved placement for kind 3. **Load**: vehicles first, then passengers at
  `vehicle_pose * local` with `local_velocity`, so a mid-air save on a deck lands on the deck.
- **Replication** ([net.md](net.md) "Prediction", rules 5, 8): an aboard snapshot carries `(vehicle, local pose, local
  velocity)` instead of a `WorldPos`; peers compose it with their own vehicle pose (UE `FBasedMovementInfo`), so
  remote riders never jitter. Own prediction runs base-local: exact on a pure route deck, corrected on a kind 3 deck.
  The ridden own mount is predicted with its rider (net.md "Prediction", own ridden mount).

### 7. Deck-local nav (IN22)

Each `nav: true` deck gets a navmesh baked once at cook time in the owner frame (per model, cached like collision),
registered as a **moving nav island** posed by the deck's tick pose: queries, paths and ORCA among agents on the
island run deck-local, and steering outputs a base-local velocity the mover rides. At a halt, `board_points` become
off-mesh links to the world mesh ([nav.md](nav.md) NV9), removed when the vehicle moves. The nav lane reviews this
section before IN22 is built (travel roadmap §5).

### 8. Mounts (IN25, IN26, IN10; TV17, TV18; decision 6, design only)

- **IN25 profile.** A `MoverParams` profile in record data per creature: speed per gait, acceleration and braking
  half-lives (momentum), turn rate falling with speed, a **gait ladder** (e.g. walk, trot, canter, gallop): a sprint
  tap raises a gait, holding keeps it, releasing settles one gait per `interact.mount.gait_settle_s`. A pack's
  strider or guar differs by numbers only; the mover stays `sf_controller`'s.
- **`RiderToMount`** (Unreal possession): while a rider holds a `Driver` seat its `Command` maps to the mount's
  (stick -> heading and speed, camera-relative; sprint taps -> gait; jump -> mount jump; interact -> dismount), the
  same for player, remote player (host applies its commands) and NPC riders (AI writes the rider's `Command`). The
  mount's AI keeps reactions only (panic, flee), surfacing through buck-off.
- **IN26 legs.** N-leg foot placement over the creature's own gaits: per-leg probes, plant/lock from each leg's
  contact curve, pelvis/spine pitch from front versus hind foot heights, roll from left versus right, `StrideWarp`
  to the profile speed; extends `FootPlacement` / `StrideWarp` (the anim lane may take it).
- **IN10 mount.** Saddle = `Astride` `Driver` seat with Left/Right entrances (RDR2 by side; from behind later). A
  ridden mount is one tier-2 character, pinned while ridden, streaming with its rider.
- **TV17 whistle.** Within `interact.mount.whistle_range_m` the mount paths to the caller (`sf_ai`, AI6); with no path
  or no arrival in `interact.mount.whistle_timeout_s`, it is placed at a nav point at least
  `interact.mount.whistle_spawn_m` away, out of every player's view, and runs in. It never appears in view.
- **TV18.** Stamina drains per gait, recovers at rest, caps the ladder at zero. A hit staggering mount or rider past
  `interact.mount.buck_poise_ratio` of the mount's poise bucks the rider (`Dismounted { Forced(Bucked) }` with point
  velocity, then a combat reaction). A load door while mounted dismounts and ties the mount at the door's world side
  (tied flag + position in its `sf/ai` state); it waits there.

## Co-op, budgets and tunables

Seat links, claims, `Aboard` and crush episodes are host state (net rule 5); `Mounted`, `Dismounted`, `Boarded`,
`Left`, `CharacterCrushed` are events with tick and stable ids (rule 8); route decks are pure (rule 4); seat entry is
a host-validated request (`Activate` + claim, as IN2). Departure and skip votes are travel's (Stage 7).

Budgets (High, 4060 laptop, per tick, to measure per row): based movement <= 10 µs per based character (IN11c2a:
6.1 µs in contact); seat persist <= 1 µs per link; deck frame and rebase zero off a deck; island queries <= 1.1x a
static one; N-leg IK <= 30 µs per visible mount per frame; a ridden mount = one tier-2 character. Results in
`docs/ledger/perf.md`.

Tunables ([tunables.md](tunables.md); a pack's base mod may override; `Guess` until measured):

| Name | Unit | Default | Range | Source | Net |
|---|---|---|---|---|---|
| `move.crush.depth_m` | m | 0.1 | 0-1 | Guess (built IN11c2c2) | Gameplay |
| `move.crush.release_depth_m` | m | 0.05 | 0-1 | Guess (hysteresis) | Gameplay |
| `move.push.dynamic_mass_kg` | kg | 200 | 1-100,000 | Guess (Jolt uses a flag) | Gameplay |
| `move.base.frame_margin_m` | m | 0.5 | 0-5 | Guess | Gameplay |
| `camera.base.follow_yaw` | bool | true | - | Industry (UE `bIgnoreBaseRotation` false) | Gameplay |
| `interact.vehicle.leave_s` | s | 0.5 | 0-5 | Guess | Gameplay |
| `interact.seat.jump_off_speed_mps` | m/s | 1.5 | 0-30 | Guess | Gameplay |
| `interact.seat.exit_search_m` | m | 3 | 0.5-20 | Guess | Gameplay |
| `interact.seat.attach_blend_s` | s | 0.2 | 0-1 | Guess | Presentation |
| `interact.mount.gait_settle_s` | s | 1.5 | 0-10 | Guess (TV-R2) | Gameplay |
| `interact.mount.whistle_range_m` | m | 150 | 0-2,000 | Guess (TV-R2) | Gameplay |
| `interact.mount.whistle_timeout_s` | s | 20 | 1-120 | Guess | Gameplay |
| `interact.mount.whistle_spawn_m` | m | 40 | 5-500 | Guess | Gameplay |
| `interact.mount.buck_poise_ratio` | ratio | 1.0 | 0-10 | Guess | Gameplay |

Mount profile fields (gait speeds, half-lives, turn curve, stamina) are record data with units and ranges, defaults
after TV-R2. Travel's waits and decelerations are travel.md's. `TICK_HZ`, the seat mode enum and save versions stay
`const`.

## Dependencies

Anim: Kimodo rider clips (sit, stand, astride by side, gait loops) for IN7, IN23, IN10; creature gaits from the
game's own clips; IN26. Physics: buoyancy 3.7 for kind 3 only; swimming 3.6 before overboard is checked in game.
Nav: islands and halt links (review). AI: AI6 before TV17, `UseSlot` (IN8/AI14) for NPC passengers. Travel: TV4
before decks ride in game; TV6, TV8, TV13 consume IN7, IN21, IN22, IN24. Net: own-mount prediction in net.md.

## Slices (rows of the [travel roadmap](../../../docs/roadmaps/travel.md) Stages 3 and 6)

Done: IN20, IN11a, IN11b, IN11c1, IN11c2a, IN11c2b, IN11c2f, IN11c2c1, IN11c2c2. Tests: logic exact, feel sanity
with printed numbers (CONTRIBUTING "Tests"); every based test at 30 and 144 fps.

| Row | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| IN11c2g | One fixed tick from `PhysicsParams::hz`; mismatch error | `sf_action_bevy`, `sf_physics_bevy` | IN11c2f | exact: one value sets both; runtime change applies |
| IN11c2c3 | Player crush event, grounded while squeezed, release hysteresis, held on a still pusher | `mw_walkers_bevy`, `sf_controller` | IN11c2c2 | headless: player event; reverse releases |
| IN11c2d | Mass-threshold dynamic pushes, no probe hits | `sf_physics`, `sf_physics_bevy` | IN11c2b | crate pushes, cup doesn't; P5 tests pass |
| IN11c2e | Ride in `Docked` and warps; jump with full point velocity | `sf_controller`, `sf_action_bevy` | IN11c2b | exact: jump lands on the take-off spot |
| IN11c3a | Full-rotation ride, upright capsule, twist-only facing | `sf_controller` | IN11c2e | no slide-off at rest; ms |
| IN11c3b | Deck frame volume, rebase while riding | `sf_controller`, `sf_physics_bevy` | IN11c3a | airborne drift sanity; rebase pop 0 |
| IN21 | Base-local drawn pose, foot locks, camera yaw carry | `sf_controller_bevy`, `sf_camera` | IN11c3a | drawn offset constant at rest; owner IN-11 |
| IN7 | `AttachDef`, `SeatLink` on IN2's path, exits, jump-off, events | `sf_interaction(_bevy)`, `sf_action` | IN2, IN20 | exact transitions, refusals, `OwnerMoved` exempt, jump-off velocity; lands on navmesh |
| IN23 | Seat IK per mode | `sf_anim_graph` | IN7, Kimodo clips | pelvis in tolerance; owner IN-23 |
| IN24 | `Aboard`, `Boarded`/`Left`, save blobs, load order, snapshot shape | `sf_interaction`, `sf_save` | IN7, IN11c3b | exact: hysteresis; mid-ride save/load (deck, seat) |
| IN22 | Deck nav islands, halt links, deck-local avoidance | `sf_nav(_bevy)` | IN11c3a, NV9 | NPC walks bow to stern under way |
| IN25 | Mount profile, gait ladder | `sf_controller`, `sf_action` | TV-R2 | exact stop distance, turn radius, ladder steps |
| IN26 | N-leg placement, pelvis/spine pitch-roll, stride warp | `sf_anim_graph` | TV-R2 | no foot through a slope; owner IN-26 |
| IN10 | Creature mount: astride driver seat, `RiderToMount`, mounted gaits | `sf_interaction`, `sf_action` | IN7, IN23, IN25, IN26 | exact mapping per command; owner IN-10 |
| TV17 | Whistle, out-of-view fallback | `sf_interaction`, `sf_ai` | IN10, AI6 | arrives or fallback, never in view |
| TV18 | Stamina, buck-off, door dismount and tie | `sf_interaction` | IN10 | exact thresholds; tie saved; lands on navmesh |

Owner checks go in `bevyrrowind/docs/visual-checks/interaction.md` (IN-11, IN-23, IN-26, IN-10); PH-17 (movement
feel after IN11b) already awaits the owner.

## Open questions

**Decided by the industry standard (meta, 2026-11-26; were owner questions):**
1. Jumping off a seat or deck at speed: a GTA/RDR2-style tumble or stagger above 3x
   `interact.seat.jump_off_speed_mps`; a normal landing below.
2. Mount from behind (RDR2 vault) or onto a moving mount: later; the first mount slice mounts by side only.
3. Seated passengers use items and non-attack spells; attacks come with mounted combat.

**For TV-R2 (parked; do not invent the answers):** which creatures a pack may ride and where the saddle sits (bone
or authored point); their gait clips, foot-contact curves and loop phases; leg chain types (two-bone, digitigrade,
more than four legs); N-leg IK prior art and crates on Bevy 0.20; profile numbers (gait speeds, half-lives, turn
curve, stamina); the Kimodo rider set; deck-local nav practice in shipped engines.
