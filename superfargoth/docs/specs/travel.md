# Travel: networks, stops, operators, rides, teleport anchors (`sf_travel`)

Status: **spec (TV0, 2026-11-25)**, nothing built. Lane roadmap: `docs/roadmaps/travel.md` (decisions 1-2, 7-10;
stages 1, 4-7). Related: [services.md](services.md) (the dialogue-side offer and its `ServiceEffect`s),
[clock.md](clock.md) (`GameClock::skip(ms)`), [nav.md](nav.md) (NV12 coarse graph, `WorldGraph`),
[interaction.md](interaction.md) "(c) Mounts and vehicles" (seats, decks, passenger state; TV0b), a later `map.md`
(MAP0: reveal, pins), [tunables.md](tunables.md). Game side: `bevyrrowind/docs/specs/travel-adapter.md` (TV0c)
maps the adapter contract below to Morrowind data.

> **Agent summary.** Travel is data: a `TravelNetwork` of `Stop`s joined by `TravelRoute`s, each with a mode
> (teleport or vehicle), a path, a speed, a price rule and a schedule. An operator is an ordinary actor bound to a
> stop and is always replaced when lost. A trip is a `RideState`; Stage 1 (every preset) is a ride **skipped at
> boarding**: quote, pay, fade, `GameClock::skip(route ms)`, arrive with followers. Real rides (Stage 4+) reuse the
> same state on a route-driven vehicle. Mark/Recall-style teleports use a saved `RecallAnchor` per player and a
> nearest-tagged-place query; every trip passes one `TravelCost` + gate rule. Host-authoritative, fixed tick, pure.

## Goal

- Any content can declare travel networks (caravans, ferries, guild teleports, gondolas, trains) as records; the
  framework never knows which kind is which (`kind` is an opaque tag).
- **Stage 1:** every network is usable in every preset as instant travel with the right price, game time spent and
  followers, through the existing services list screen.
- **Later stages:** the same data drives real rides on walkable decks and seats, leaving a vehicle anywhere, ambushes,
  mounts riding the road graph and co-op boarding, without a second model.
- No place is ever cut off: a lost operator is replaced; a stop with no live or pending operator is logged.
- Teleport effects (mark/recall, "nearest shrine") share the gate and cost rule with paid travel.

Non-goals: the map (MAP0), seat/deck physics and passenger pose (interaction (c)), mount locomotion (IN25), the
dialogue offer and price screen (services.md), script position teleports (scripting; they may reuse the executor).

## Owner decisions

| Date | Decision | Where it lands |
|---|---|---|
| 2026-11-25 (Q5) | Until the interaction stage lands, all travel is an instant teleport: fade, game time spent, followers come | Instant travel |
| 2026-11-25 (Q2) | Players can always leave any vehicle anywhere, a ship at sea included (allowed, not advised) | Rides |
| 2026-11-25 (Q2) | Ships are walkable moving decks (Sea of Thieves), never a mount disguised as a ship | Rides; interaction (c) |
| 2026-11-25 (Q6) | A passenger leaves a boat at sea: the boat slows down and stops; after a while the crew loses patience and goes on, like land vehicles (wait = tunable) | Rides |
| 2026-11-25 (Q4) | Operators are always replaced, in every preset: a place is never cut off | Operators |
| 2026-11-25 | No free teleport by default ("travel anywhere discovered" is an opt-in, off) | Gates |
| 2026-11-25 | Skipped rides spend game time (the route's time) | Instant travel, Rides |
| 2026-11-25 (Q1) | Ambushes in Modern can wreck the ride (Stage 5); never in Classic | Rides, Slices |
| 2026-11-08 (services Q1) | Co-op time: a party rest vote, else the trip completes without passing time | Co-op |

## Reuse check (CONTRIBUTING rule 0)

- **Bevy 0.20.** No travel, route-network or fast-travel feature. `bevy_curve` 0.20 (split out of `bevy_math`;
  `bevy_reflect` optional) has cubic splines and the `Curve` trait with `reparametrize*`, but no arc-length
  parameterisation (checked in its source). Candidate for TV2's spline sampling only; the arc-length table is ours.
  `Time<Fixed>`/`FixedUpdate` drive rides (TV4); fade is a UI overlay (presentation). Verdict: glue only.
- **Crates (crates.io / GitHub, 2026-11-25).** No crate models travel networks, operators or rides. Found:
  `bevy_northstar` (HPA* grid pathfinding, Bevy 0.16, grid-only: not our navmesh), `vleue_navigator`/`bevy_pathmesh`,
  `oxidized_navigation`, `bevy_landmass`, `bevy_rerecast` (navigation; our `sf_nav` already covers this),
  `bevy_follow_path` (Bevy 0.8, abandoned 2022), `prime-splines`, `spanda`, `animato-path` (arc-length curve maths,
  no Bevy, small; TV2 may take one if it beats ~60 lines of our own table), `route16` (road-network routing
  for maps, overkill), `petgraph` 0.8 (already in the workspace; a network of tens of stops needs no graph library,
  adjacency `Vec`s suffice). Verdict: none fits the domain; our own Bevy-free `sf_travel`.
- **Our own.** `sf_ids::GameClock` / `GameInstant` (ms, `skip(ms)` works while paused) is the only clock.
  `sf_nav` `route(Location, Location, class)` and the NV12 coarse graph give road lengths and times; road paths come
  from Magna Urbis roads via NV12 and `large` nav paths (read, never made here). `sf_world::Location`/`WorldPos`,
  `sf_ids::ActorKey`, `sf_records::RecordId` are the ids. services.md owns the offer, the list screen, `Pay`,
  `Teleport`, `PassTime`; sf_travel supplies the rows and the plan. `sf_interaction` owns seats and passenger pose.

## Industry comparison

| Game | Network and operator | Presentation | Time | Risk, gates |
|---|---|---|---|---|
| OpenMW / Morrowind | NPC lists destinations (star); fare from distance, flat fee for guild teleports | menu, 1 s fade out/in, arrive | hours from 2D distance, slept through | none; followers within a radius come, hostile ones stay |
| Skyrim | carriage driver at hold stables, fixed list (memory) | fade, arrive | hours pass (memory) | driver killable (memory) |
| RDR2 | stations and stagecoach posts; camp map fast travel upgrade | ride cinematic, skippable (sources disagree) | time passes | none on the cinematic (memory) |
| Witcher 3 | signposts (discovered only, from a signpost); boats are real, player-sailed | load screen; boats ridden | (memory) | blocked in combat (memory) |
| Sea of Thieves | ships are physics bodies with walkable decks and crews | always live | real time | fall off and the ship sails on with its momentum |
| KCD2 | travel map over known places | hero walks the route on the map, time and food pass | real route time | encounters interrupt the trip |
| Dragon's Dogma 2 | oxcart at village gates | real cart, sleep or fight | real time | attacks; cart or ox destroyed |
| Elden Ring | graces, any rested one | instant warp | none | blocked near enemies, in some areas |

**Where we stand.** We take OpenMW's network-on-a-provider and fare/time rules (as adapter rules), DD2/KCD2's "a fast
trip is a skipped real trip" (same route, same time, same later risk), Sea of Thieves' walkable decks, Elden Ring's
combat gate, Witcher's discovered-only opt-in for free travel. Beyond all of them: operators are replaced, so a
network never breaks (no shipped game above promises that). Gaps: none known for Stage 1; timetable boats and
physics boats are "Later" (roadmap).

## Data model

Crates: `sf_travel` (Bevy-free: `sf_ids`, `sf_coords`, `sf_world`, `sf_records`, serde; listed in hakari
`final-excludes`) and, from TV4, a thin `sf_travel_bevy`. Stage 1 needs no Bevy crate: the services host executes
the plan.

```rust
pub struct TravelNetwork {                 // record `sf.travel.network`, mod-patchable; adapters may synthesize them at load
    pub id: RecordId,
    pub kind: RecordId,                    // opaque tag ("caravan", "ferry"...): data profiles key on it, code never matches it
    pub stops: Vec<Stop>,
    pub routes: Vec<TravelRoute>,
}
pub struct StopKey(pub RecordId);          // stable across mods (never an index)
pub struct Stop {
    pub key: StopKey,
    pub arrive: Pose,                      // where travellers appear: Location + yaw
    pub board: Option<Location>,           // where the operator stands / the "Board" point (None = arrival-only stop)
    pub place: Option<RecordId>,           // the place it belongs to (map, discovery, nearest-place tags)
    pub discover: Discover,                // Always | Place(RecordId): read from the discovery source, never stored here
    pub operator: Option<OperatorSlot>,
}
pub struct Pose { pub at: Location, pub yaw: f32 }
pub struct TravelRoute {
    pub from: StopKey, pub to: StopKey,
    pub directed: bool,                    // provider lists need not be symmetric
    pub mode: RouteMode,                   // Teleport (no vehicle, e.g. a guild guide) | Vehicle(RecordId vehicle profile)
    pub path: PathRef,
    pub arrive: Option<Pose>,              // None = the destination stop's `arrive`; Some = this route's own landing pose
    pub speed_mps: Option<f32>,            // None = the profile's, else `travel.route.speed_mps`
    pub price_rule: RecordId,              // adapter pure rule (below)
    pub time_rule: Option<RecordId>,       // None = route time (length / speed); Some = adapter rule (e.g. classic hours)
    pub schedule: Schedule,                // Always | OnCall | Timetable { period_ms, departures_ms: Vec<u64> }
    pub tags: Vec<RecordId>,               // gate data (e.g. "interior-only"), read by adapter gates
}
pub enum PathRef {
    Estimate { length_m: f32 },            // Stage 1 scaffold: sf_nav::route length, else 2D distance x `travel.route.detour_ratio`
    Baked(PathId),                         // TV2/TV12: arc-length spline with clearance; replaces Estimate, removal path = TV2
}
pub struct OperatorSlot {
    pub template: RecordId,                // adapter template for a replacement
    pub actor: Option<ActorKey>,           // the live operator, if any
    pub status: OperatorStatus,            // Live | Absent(reason) | Vacant { since: GameInstant } | Pending { spawn_after: GameInstant }
}
pub struct RideState {                     // one trip; Stage 1 creates and resolves it in the same tick
    pub id: RideId,                        // host counter, saved
    pub network: RecordId, pub route: (StopKey, StopKey),
    pub phase: RidePhase,                  // Boarding | UnderWay | Halted(HaltReason) | Arrived
    pub depart_at: GameInstant,
    pub s_m: f64,                          // arc length travelled (off-ring: this number is the vehicle)
    pub passengers: Vec<Passenger>,        // PlayerId or ActorKey; seat/pose lives in interaction (IN24)
    pub fare_paid: i64,
}
pub struct TravelCost { pub gold: i64, pub game_ms: u64, pub needs: Vec<Need>, pub encounter_risk: f32 }
pub struct RecallAnchor { pub player: PlayerId, pub at: Pose }   // one per player (`travel.recall.anchors_count`)
```

Indices: `TravelBook` (resource-like plain struct) holds all networks with `StopKey -> (network, stop)` and
outgoing-route maps, built once at load; `destinations(stop, ctx) -> Vec<Quote>` filters by direction, schedule,
discovery and gates, then prices.

**Rules are pure functions the adapter registers by id** (services.md pattern): `price(PriceCtx) -> i64` with
`PriceCtx { route, length_m, dist3d_m, dist2d_m, followers, buyer stats view }`; `time(TimeCtx) -> u64 ms`. The
default time rule is the route time: `game_ms = ceil(remaining_m / speed_mps * 1000) * time_scale` with the clock's
saved `time_scale`, integer maths, so a skipped ride passes exactly what the real ride would have.

## Instant travel (Stage 1, every preset)

A ride skipped at boarding (decision 2). Net-ready: the client only sends commands; the host runs every step on its
fixed tick; all maths are pure and integer.

1. **Offer.** The provider's services offer (services.md list service) asks `TravelBook::destinations(stop, ctx)`;
   each `Quote { to, cost: TravelCost, refusal: Option<Refusal> }` becomes a `ServiceEntry` row (refused rows show
   their reason, as services does). The list screen is unchanged.
2. **Request.** The UI sends the services purchase request (`ServiceRequest`, player id, row, tick).
3. **Re-check (host).** `quote` again at the request tick: gates (below), price, operator `Live`; a stale row is
   refused, never bought at an old price (services `buy_entry` rule).
4. **Plan.** `plan_instant(quote, followers) -> TravelPlan { ride: RideState, effects }`. `RideState` is created
   `Boarding` at `now`, then skipped to `Arrived`: `s_m = length`, arrival instant = `now + cost.game_ms`.
5. **Effects, in order:** `Pay { amount }` (always first), presentation cue `FadeOut` (`travel.fade.out_s`, real
   time, presentation only), `Teleport { to: route.arrive or stop.arrive, with_followers: true }`, then the clock skip of
   `cost.game_ms` (sleeping, so wake-ups and rest rules apply as for rest), presentation `FadeIn`, `TravelEvent::Arrived`.
   Gameplay never waits on the fade; the fade is drawn over the same tick's state change.
6. **Followers.** The adapter's follower query returns actors following the buyer within `travel.follow.radius_m`;
   the adapter's exclusion rule drops hostile followers (their follow is cleared) and actors marked to stay
   outside. Followers arrive at the stop's pose with a small ring offset (`travel.follow.ring_m`), same tick.
7. **Events (data):** `TravelEvent { tick, player, ride, kind }` with `Quoted | Refused(reason) | Departed | Arrived`;
   the adapter's "player is travelling" flag reads `Departed..Arrived` (set for one update in Stage 1).

Mode `Teleport` routes (no vehicle) take the same path; their `time_rule` may return 0. Classic and Modern differ only
in the rules the adapter registers (classic fare and hours formula vs route time), never in this flow.

## Rides (Stages 4-7, design only)

- **Motion.** A route-driven vehicle moves on `FixedUpdate`: `s_m(t) = clamp((t - depart_at) * speed)` with halts
  subtracted; its pose (bob, roll included) is a pure function of `(route, s_m, tick)`, so peers agree. Off the
  streaming ring a vehicle is only `s_m`; on the ring it is one kinematic body (decision 8: one transform).
- **Places.** Vehicle profile = motion x places: walkable deck (based movement, IN11) or seats (`SeatLink`, IN7).
  Passenger state `{ vehicle, seat or base-local pose }` is interaction's (IN24); `RideState.passengers` only lists who.
- **Departure.** `Always`: leaves when boarded and confirmed; `OnCall`: the vehicle is at the stop when the operator
  is; `Timetable`: departs at `departures_ms` mod `period_ms`. Co-op adds the boarding window (below).
- **Hold to skip.** `skip_ms = time(remaining)`; the clock skips, `s_m` jumps to the end, or (Stage 5) to the first
  ambush point `f` the seeded roll finds in the skipped span: the ride halts there, `Halted(Ambush)`.
- **Leaving any vehicle, anywhere (owner Q2, Q6).** A passenger may walk off or jump off at any time, with the
  deck's point velocity (interaction). Then the vehicle, land or water alike:
  1. decelerates to a stop (`travel.ride.stop_decel_mps2`, per profile; boats slower),
  2. waits `travel.ride.wait_s` (sim seconds, per profile) as `Halted(PassengerLeft)`,
  3. re-boarding during the wait cancels it and the ride resumes; otherwise the crew goes on to the route's end.
  The fare is spent; the player may board again at a stop (new fare). A ride with no passengers left finishes its
  route and returns to its schedule. Overboard at sea is the player's problem (swimming, water spec).
- **Wreck (Stage 5).** An ambush may destroy the vehicle or kill the operator: the ride ends `Halted(Wrecked)`, the
  player is left at a reachable road point (anti-strand), the operator slot goes `Vacant` (replacement applies).

## Operators and replacement

- An operator is an ordinary actor (`ActorKey`) bound to an `OperatorSlot`; it can talk, fight, sleep, die. The
  network slice (TV5) registers each operator with its stop at load.
- **Status.** `Live` (offers rides), `Absent(reason)` (asleep, hostile to this player, quest-closed: no rides,
  no replacement), `Vacant { since }` (dead or despawned: no rides), `Pending { spawn_after }`.
- **A vacant stop keeps arrivals:** other stops still travel to it; only departures from it stop.
- **Replacement, every preset (owner Q4).** On `Vacant`, set `spawn_after = since + travel.operator.replace_delay_s`
  (game time). Each host tick past that, the stop's board point is tested against every player's view sample
  (`ViewSample { pos, forward, fov }` passed in as data): out of every frustum and farther than
  `travel.operator.spawn_min_dist_m`. Then the core emits `SpawnRequest { template, at: board, slot }`; the game host
  spawns it (an ordinary actor from the adapter template) and reports the new `ActorKey`; status `Live`.
- **Orphan check.** At load and after each network patch: every stop with outgoing routes has a `Live` or `Pending`
  operator, or none is needed (`board: None` and no departures). Each orphan logs one line
  `travel: orphan stop <key> (<network>)`; the game-data target is 0 (TV26).

## Teleport anchors and gates

- **`RecallAnchor`.** Set: the anchor becomes the player's current pose (replacing the old one; capacity
  `travel.recall.anchors_count`). Recall: gate check, then the same executor as instant travel with
  `with_followers: false` and 0 ms (adapter effects may say otherwise). Anchors are per player, saved.
- **Nearest tagged place.** `nearest_place(from: Location, tag) -> Option<Pose>`: places carry tags (opaque, adapter
  data; e.g. a shrine marker). From an exterior: the nearest tagged place in that worldspace by the adapter's metric
  (`Euclid2d` default, or `CellRing` = Chebyshev distance over grid cells, then Euclid); ties by `RecordId` order.
  From an interior: tagged places in that interior win; else walk door links breadth-first, checking each visited
  interior's tagged places first (OpenMW `worldimp.cpp` 3191-3245), (via `sf_nav::WorldGraph`, passed in as a trait
  `ExteriorResolver`) to the first exterior, at most `travel.place.max_door_hops`, then as exterior. None = refuse.
- **`TravelCost` and gates (TV10).** One rule for services, rides, teleport effects and the opt-in map travel:
  `check(ctx) -> Result<TravelCost, Refusal>`, gates evaluated in this order, first failure returned:
  `TeleportBlocked` (a global or region flag scripts set), `HostilesAlerted` (an enemy has the player as target,
  from the AI/combat reader), `NoTravel` (place or region flag), `Undiscovered` (destination not discovered, from
  the discovery source), `OperatorAbsent`, `Needs` (items, rank: adapter), `CannotAfford`, `NotAboard` (co-op),
  `Adapter(id)` (interior rules and the like). Each gate is data-on (a profile lists which gates apply per caller).
- **Free travel off by default.** "Travel anywhere discovered" (TV16) is `travel.anywhere.enabled = false`;
  when on it still passes every gate and spends route time.

## Saves

Section `sf/travel` v1 (serde, host-owned), only mutable state; networks are records and are not saved:

- operator slots: `StopKey -> { actor: Option<ActorKey>, status }` (only those not `Live` with the load-time actor);
- active rides: `Vec<RideState>` (Stage 4+; Stage 1 never saves one: it resolves in one tick), `next_ride_id`;
- recall anchors: `PlayerId -> RecallAnchor`.

Discovery is not stored here (map's `MapReveal` or the adapter's discovery source is the one truth). Load: a slot
whose stop no longer exists (mod removed) is dropped with a log line; a ride on a removed route arrives at its
target stop if it exists, else at its origin. Unknown fields are ignored (forward compatible); version bumps
migrate as `sf/clock` does.

## Co-op

Net rules: fixed tick, input as commands (`TravelCommand::{Board, Skip, Leave, SetAnchor, Recall, Confirm}` plus the
services purchase), state apart from presentation (fade, deck draw), pure steps, one owner: **all travel state is
host-owned**; clients never move a vehicle or a teleport target.

- **Stage 1.** The buyer and their followers teleport; other players stay. Game time follows services' co-op rule
  (owner 2026-11-08): the skip is a party rest request; if the vote fails or is pending, the trip completes with no
  time passed. Gates use the buyer's state; `HostilesAlerted` checks the buyer only.
- **Stage 7 (parked behind net N1-N5).** Vehicles are host state with seats. Departure when every present party member
  is aboard, or `travel.coop.boarding_window_s` ends, or the leader confirms; players left behind stay and catch a
  later ride, walk, or teleport to the host (a safe road point near the host, blocked in combat; mounts follow).
  Skip is a vote and refused while any party member is not aboard. Replication is base-relative. A rider far from
  the host is a second streaming/nav anchor (TV25). Mark/Recall stay personal.

## Budgets and tunables

Budgets (host, release): `TravelBook` build at load < 1 ms for 2,000 routes (a 10x-scale world); `destinations`
< 20 µs per call; instant plan < 50 µs; per-tick ride update < 2 µs per active ride, off-ring rides are one add;
replacement check < 5 µs per vacant stop per tick (vacant stops are few); `sf/travel` save < 16 KiB typical. Frame
cost of Stage 1 outside a trip: zero (no per-frame system). Measured in TV1/TV4 tests (`#[ignore = "measurement"]`).

Tunables (tunables.md; prefix `travel.` to add to `docs/SYSTEMS.md`); a game sets its own defaults by base-mod record:

| Name | Default | Range | Unit | Net | Source |
|---|---|---|---|---|---|
| `travel.fade.out_s` / `travel.fade.in_s` | 1.0 / 1.0 | 0-5 | s | Presentation | Reference(OpenMW travel window) |
| `travel.follow.radius_m` | 12 | 0-100 | m | Gameplay | Guess (game mod sets its own) |
| `travel.follow.ring_m` | 1.5 | 0.5-10 | m | Gameplay | Guess |
| `travel.route.detour_ratio` | 1.3 | 1-3 | ratio | Gameplay | Guess (Stage 1 scaffold, gone with TV2) |
| `travel.route.speed_mps` | 4.0 | 0.5-50 | m/s | Gameplay | Guess (profiles override) |
| `travel.operator.replace_delay_s` | 86,400 | 0-2,592,000 | s (game) | Gameplay | Guess (one game day) |
| `travel.operator.spawn_min_dist_m` | 60 | 0-500 | m | Gameplay | Guess |
| `travel.recall.anchors_count` | 1 | 1-16 | count | Gameplay | Guess |
| `travel.place.max_door_hops` | 8 | 1-64 | count | Gameplay | Guess |
| `travel.anywhere.enabled` | false | bool | none | Gameplay | Owner(2026-11-25) |
| `travel.ride.stop_decel_mps2` (land / water) | 2.0 / 0.4 | 0.05-20 | m/s² | Gameplay | Guess (owner Q6: boats slow, then stop) |
| `travel.ride.wait_s` (land / water) | 30 / 45 | 0-600 | s | Gameplay | Guess (owner Q6: a tunable) |
| `travel.coop.boarding_window_s` | 15 | 0-120 | s | Gameplay | Guess (research §6: 10-20 s) |

Stage 5 adds `travel.encounter.*` (hazard per km by time of day); Stage 6 the mount profile (interaction lane).
Constants that stay `const`: save version, id formats, the gate order.

## Slices

The `sf_travel` rows of `docs/roadmaps/travel.md` (IDs unchanged; full table there):

| ID | Deliverable | Crate | Stage |
|---|---|---|---|
| TV1 | Core: networks, stops, operators, `RideState`, ETA, skip maths (instant = skip at boarding), saves | `sf_travel` | 1 |
| TV10 | `TravelCost` rule + gates for services and teleport effects | `sf_travel`, `sf_rpg` | 1 |
| TV26 | Operator replacement: vacancy, out-of-view spawn after the delay, orphan-stop check | `sf_travel`, `mw_rpg` | 1 |
| TV14 | `RecallAnchor` (saved) + nearest-tagged-place query | `sf_travel` | 1 |
| TV16 | Opt-in "travel anywhere discovered", off, gated | `sf_travel_bevy` | 2 |
| TV2 | Route bake: nav polyline to arc-length spline with ground clearance (replaces `PathRef::Estimate`) | `sf_travel` | 4 |
| TV4 | Ride runtime: kinematic deck on `FixedUpdate`, depart/arrive, hold-to-skip, off-ring progress | `sf_travel_bevy` | 4 |
| TV8 | Leave any vehicle anywhere: land and water vehicles slow, stop, wait, then go on (owner Q6); re-board at a stop | `sf_travel_bevy` | 4 |
| TV12 | Water-route bake on the water mask with coast clearance | `sf_travel`, `mw_bake` | 4 |
| TV7 | Encounter tables + seeded tick roll; skip resolves to "ambush at f" | `sf_travel` | 5 |
| TV9 | Ambush runtime: halt, spawn ahead out of view, damageable operator/vehicle, wreck, anti-strand | `sf_travel_bevy` | 5 |
| TV19 | Auto-ride the road graph to a pin or stop | `sf_travel_bevy` | 6 |
| TV21 | NPC riders and caravans on roads, tier 0 off-screen | `sf_ai`, `sf_travel` | 6 |
| TV22-TV24 | Boarding window and leader confirm; skip vote; teleport-to-host | `sf_travel_bevy` | 7 |

Game-side rows using this spec: TV5 (networks from records), A4 (instant travel presenter), TV15 (effects on TV14 +
TV10), TV3, TV13, TV11.

## Adapter contract (mapped in bevyrrowind travel-adapter.md, TV0c)

The adapter supplies, as data and pure rules, never as framework code:

1. **Networks:** every provider kind turned into `TravelNetwork` records: stops with an arrival pose (position, yaw,
   optional interior location), operator bindings with a replacement template, directed routes with mode, profile
   and schedule; arrival-only stops where a destination has no provider.
2. **Rules by id:** price (from 3D/2D distance, follower count, flat fees, barter hook) and time (a classic hours
   rule; `None` = route time); whether the skip counts as sleep.
3. **Followers:** the follower query, the exclusion rule (hostile, stay-outside) and its radius default.
4. **Gates:** interior and region rules, the global teleport-block flag source, needs (items, rank), discovery source.
5. **Places:** tags for nearest-place queries, the metric choice, the effects (mark, recall, nearest-shrine) that
   call TV14; script position teleports stay scripting's.
6. **Presentation:** the cast/fade cue per route mode and the "travelling" flag on `Departed..Arrived`.

## Tests

Exact (TV1, TV10, TV14, TV26; synthetic networks, no game data):

- arrival instant = `now + game_ms` for route time at time scales 1, 30, 120; integer ms, no drift;
- skip at boarding equals a ride ticked to the end at 1/64 s and at 1/30 s fixed ticks (same arrival instant);
- instant plan yields `Pay` first, then teleport, then the skip, in that order; a stale row is refused;
- price rule called with the right follower count; refused rows carry the right `Refusal`;
- each gate refuses alone, and the gate order holds when several fail;
- directed routes: B lists no return when A to B is directed;
- operator: `Vacant` -> no spawn while any view sample sees the board point or is within the distance -> spawn
  after the delay once out of view; a vacant stop still receives arrivals; orphan check counts on a crafted network;
- nearest place: exterior metric and tie by id; interior through door hops; hop limit refuses;
- `sf/travel` round trip: slots, anchors, (Stage 4) a mid-ride `RideState`; removed stop or route on load.

Measurement (`#[ignore = "measurement: ..."]`): `TravelBook` build and `destinations` µs at 2,000 routes.
Game log check (TV26): `travel: orphan stop` count = 0 on the game's data; TV5 logs counts per kind.

## Open questions (design only)

1. **Clock skip granularity in services.** `PassTime { hours: u16 }` cannot carry route time in ms; Stage 1 needs
   `PassTime` in ms (the clock is ms) or a travel effect that calls `GameClock::skip(ms)`. Proposed: widen
   `PassTime` to ms in services.md when TV0's pointer edit lands (classic whole hours still fit).
2. **Stop merging.** Two providers at one place (two operators, one dock): one stop with two slots, or two stops
   sharing a pose? Proposed: one stop per board point; the adapter decides, the framework allows several slots later.
3. **Hostile operator vs replacement.** An operator hostile to every player (crime) never dies, so the stop is closed
   for that player only; is a "closed for everyone for N days" case needed? Proposed: no (`Absent` is per player).
4. **Name.** `TravelRoute` (not the roadmap's `Route`) to avoid a clash with `sf_nav::Route`.
