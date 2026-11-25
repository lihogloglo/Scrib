# Travel, vehicles, mounts, teleports, maps (research slice, 2026-11-25)

Sources: our specs (`services.md` Teleport/PassTime, `services-adapter.md` "Travel", `interaction.md` (c) Mounts and vehicles,
owner answer 5), `nav.md` gaps. Web checked: DD2 oxcart/ferrystone (gamesradar, pcgamer, destructoid 2024), KCD2 fast travel
(pcgamer "KCD2 turns boring fast travel into a thing of beauty", powerpyx, gamepressure), RDR2 fast travel (gamespot, digitaltrends,
shacknews). Everything marked (memory) is from model knowledge, unverified. No crate is needed for this topic: all of it is
state machines and data on our own `sf_interaction`/`sf_rpg::service`/`sf_controller` (checked rule 2: Bevy has no seat or
vehicle feature; Rapier's ray-cast vehicle controller is the only candidate, for physics vehicles later, and is Apache-2.0).

## 1. Travel networks as data

What our spec has: a `travel` list service, a priced row per destination, `Teleport{to, with_followers}` behind a fade,
`PassTime`; price from 3D distance, hours from 2D (Morrowind formula). Destinations live on the NPC (`DODT`). There is no
route, no stop, no operator, no vehicle: the provider is a menu, not a thing in the world.

Industry shape (all are the same abstraction with different presentation):

| Game | Operator | Network | Presentation | Risk |
|---|---|---|---|---|
| Morrowind | NPC (strider, boat, guild guide) | star: each provider lists 1-4 stops | menu, fade, hours pass | none |
| RDR2 | ticket seller / stagecoach post | stops visited before; fee | cinematic of the ride, skippable only by some (gamespot/digitaltrends: sources disagree) | none (memory: cinematic only) |
| Witcher 3 | boat captain, fixed ports | port list | real boat, short, autopilot (memory) | none |
| KCD2 | travel map, anywhere known | free graph over known places | the hero walks the route in real time on a map, time and food pass, **encounters can interrupt** (pcgamer; powerpyx) | ambush/random event |
| Dragon's Dogma 2 | oxcart bell at village gates | route graph | real cart, real time, you may sleep or fight | enemies **attack the cart; cart or ox can be destroyed** (gamesradar, destructoid) |

**Generic model (recommended, data only):**
- `TravelNetwork` record: id, kind tag (opaque), `stops: [Stop{ id, anchor: WorldPos+yaw, place: RecordId, discovered_by }]`,
  `routes: [Route{ a, b, path: RouteRef (road-graph path from nav.md NV12, or an authored spline), speed, price, schedule }]`.
  `schedule`: `Always | OnCall (summon at a stop) | Timetable{departures, period}`. Directed or undirected per route.
- `Operator`: an ordinary actor/creature (strider, ship captain, cart ox + driver), reached by interaction; the stop
  is an `InteractionPoint` ("Board"). The operator is a character with a save key: **killable, can be absent**
  (dead, hostile, asleep, route closed by a quest flag). Dead operator = stop unusable until `respawn_policy` (a
  replacement after N days, as DD2/Morrowind striders do not, (memory) Morrowind never replaces killed ones in vanilla).
- `Vehicle`: entity on a `Route`, with seats (interaction (c) kinds 1/3). In the world even when nobody rides:
  a timetable boat is visible at the dock and sails (tier switch: off-screen it is a number along the route, as nav.md
  gap 4/Q2 does for NPC travellers). One mechanism serves ferries, caravans, trains.
- Morrowind adapter fills the same records: striders = star networks with `OnCall`, price/time through the existing rule
  `mw.services.travel`; guild guides = `Teleport` routes with 0 hours (not vehicles); propylons = a network whose stops are
  unlocked by `discovered_by`; boats = Timetable. The framework names none of them.
- Price is a service rule (existing). Add `Route.price_rule` id; keep the barter hook.

Verdict: **adopt** network+operator+vehicle records as a new section of `services.md` ("List service becomes
boarding"); the old list menu stays for Classic (see §2).

## 2. Ride-along presentation

Owner decided: ride-along in modern, fade in Classic. Design rules from the standard:
- **Same data, two presenters.** Classic: menu -> fade -> `Teleport` + `PassTime` (today's spec). Modern: menu or
  "Board" interaction -> player seated on the vehicle, which follows the route in real time at `Route.speed` (ride
  time = route length / speed, so 10x distances make it long: need the skip).
- **Skip is a time-skip, not a teleport:** hold-to-skip (RDR2/KCD2 style, (memory) KCD2 offers a skip of walking time)
  fast-forwards the vehicle along the route with the world sim running in "catch-up" (§below), ends at the stop. Never
  a plain cut that leaves the world at the old time.
- **Camera:** keep gameplay camera by default (a free-look third person from the seat; DD2, Witcher boats); an
  optional "cinematic" camera = a Cinemachine-like rail of authored shots per route kind (RDR2 train). Cheap: a
  second camera mode; do not build authored shots per route.
- **World sim during the ride** (the real design question): the vehicle moves through the live streamed world at ride
  speed, so streaming/LOD must keep up: cost is that of a fast mount (speed cap ~ 15-20 m/s ground), which streaming
  must handle anyway for horses and flight (ROADMAP 1.2 "flight" bench). Sky/weather/time run normally (game time
  scale unchanged during ride; on skip, time advances by remaining ride time, via `PassTime`, as services already do).
  NPC traffic, schedules and quests run as in any time passage (rest/wait rules).
- **Getting off mid-route:** yes for open vehicles on a road (cart, strider walk-off, DD2 lets you hop off; the vehicle
  stops/idles for N s then continues or returns); no mid-sea. Needs the dismount-spot logic already in `SeatLink::delete`.
  Provides the basis for quests that ask you to leave a ride.
- **Ambush / encounter events:** route gets `encounter_table` (record ref, hazard per km per time-of-day, from the
  region's danger rating); roll on a fixed tick (seeded, host side) while the vehicle moves; result spawns a
  scripted encounter ahead of the vehicle, vehicle halts, **the operator is a real target** (DD2: cart/ox destroyed
  -> you walk the rest; failure mode must not strand the player: spawn point = last passed stop or road node, nav.md).
  Fast-travel-by-skip rolls the same table but resolves it as "ambush happens at fraction f" (KCD2's prompt "ambush
  incoming" (user posts, pcgamer) is the model: warn, then fight). Default off for Morrowind adapter (vanilla has none);
  on in the generic framework.
- Co-op: the vehicle is host state; riders are seats (§6).

## 3. Fast travel designs and rules

Standard menu of designs, ranked by how they treat the world:
1. **Free teleport to discovered points** (Skyrim, BotW shrines/towers, Elden Ring graces (memory); in ER, not in combat, no cost).
   Cheap, kills traversal; made acceptable by the unlock being the content (graces, towers).
2. **Priced/timed** (Morrowind services, RDR2 stagecoach fee, KCD2: time and food pass, encounters possible, discovered
   place only). KCD2's lesson: fast travel as the *walk, abridged*, with risk: pcgamer says it "turns boring fast
   travel into a thing of beauty"; the standard answer to "fast travel breaks open worlds" is cost + risk, not removal.
3. **Physical only** (Morrowind's none-but-services, DD2 oxcart + the rare ferrystone: instant/safe but scarce).
4. **Magic point-to-point:** Mark/Recall (single player-set anchor), Intervention (nearest temple/fort by faction map),
   Almsivi (memory). These are `Teleport` effects, not travel networks.

**Recommended rules (framework):**
- Universal gates, data-flagged per preset: not while hostile actors are aware/alerted; not while encumbered-restricted
  quest locks (`no_travel` fact); destination must be discovered; interiors refuse teleport out (Morrowind's
  `NoMagic`/interior rule is an adapter rule).
- `TravelCost{gold, hours, hunger/fatigue, encounter_risk}` as a rule input, so one rule covers Classic (gold, hours),
  KCD2-like (hours, needs, risk) and Elden-like (all zero).
- Modern preset default: network travel = ride-along; "Travel anywhere discovered" free-teleport is **opt-in** (a game
  setting, not default); magic teleport (Mark/Recall, Intervention) always works as the spell, because it is gameplay
  currency; add `Teleport{.., preserve_followers}` (already there) and a "recall anchor" component (Mark) that is
  persisted in the player save.
- Intervention: `Teleport` to "nearest" of a tagged place set (`Place.tag=temple`) by straight/road distance; 10x world
  makes the nearest temple far: the tag set is data, and a game may use "nearest road stop" instead.
- Time: all teleports go through `PassTime(0..)` so quests, needs and NPC schedules see the elapsed time; magic teleports pass 0 h.

## 4. Mounts

Our spec (interaction.md (c)): seat links, driver seats, control hand-off, four kinds. It matches Veloren's model
(source named in spec). What it lacks (all from memory of shipped games unless cited):
- **Controls** (RDR2/GoT/Elden Ring Torrent/KCD): mount has its own locomotion with gaits (walk, trot, canter, gallop)
  mapped to input intent; RDR2 uses a tap-to-accelerate cadence and a "cruise" auto-ride to waypoint (the horse paths
  along roads by itself while the player takes a hands-off camera: **this is exactly the cheap, valuable bridge
  between mounts and travel**); GoT uses horse-calls and auto-path guiding to waypoint with a guiding wind (memory).
  Torrent: summon anywhere (flute), double jump, no stamina bar or only a rider's, summon unusable in some
  places. KCD: horse is damageable and stat-based.
- **Summon/bond:** (a) stable/hitching post only (Skyrim, KCD), (b) whistle: the mount runs to you via nav, (c) bond
  value with the horse (RDR2: calm, brush, feed raises skills, memory) as an RPG hook, (d) magic summon (Torrent).
  Recommend: whistle (call, mount navigates to the caller using nav.md long-range routing and a fall-back teleport
  behind the camera if > N m unreachable), bonding is data on the mount actor (`Bond` stat modifies stamina/panic).
- **Stamina:** mount has its own stamina pool and gait rules; panic/rearing on fire or predators (sf_ai reactions
  already limited to panic/flee per spec); a bucking-off result when rider is hit for large damage (Elden Ring/RDR2
  dismount on hit, memory) -> `SeatLink::delete` with ragdoll or stagger.
- **Dismount:** exit sides via dismount spots (spec); dismount-on-jump for combat; auto-dismount at doors/interiors
  (mount is left tied up, still a character, follows/idles, saves in the actor save).
- **NPC mounts:** NPC riders use the same `SeatLink`; their AI sends `MoveTo` to the mount's controller (rider AI =
  driver). Caravan wagons, guards on horses and travellers on roads (nav.md Q2) use this; off-screen they are one
  entity on the road graph.
- **Flying/water:** same seat+controller hand-off, different controller: creature flight needs nav.md's height-layer field;
  water vehicles need pontoon buoyancy (ROADMAP 3.7) and a controller. Walk-on decks (kind 4) stay last. Silt striders
  (walk-on howdah) in vanilla are a rail vehicle with an interior seat: model as kind 1 with passengers seated (no
  walk-on needed).

**Generic seat+controller design (recommend, tightened):** `Seat{slot, role: Driver|Passenger, controller_ref}`; a `Driver`
rider's `Command` is rewritten by a `RiderToMount` map (move axis -> mount move intent, jump -> mount jump, sprint -> gait
up, attack -> mounted attack set later) so any controlled body (horse, boat, cart, flier) accepts the same
`MoveIntent` the humanoid emits. The mount does not know the rider is a player (same as a remote player in co-op,
net.md). This is the Unreal "possess and controlled pawn" idea (memory: Unreal Chaos vehicles/Mounts via possession).
Mounted combat later; the seat only exposes "rider can act" flags.

## 5. Maps and orientation

Our specs have no map spec beyond journal and `ShowMap` opcode (quests research: markers None/Compass/Guidance setting).
Standard parts:
- **World map:** texture/vector map with layers; fog of war via revealed-tile mask (Morrowind paints a trail of cells as
  you walk, Skyrim reveals towns; BotW needs towers; Elden Ring map fragments; (memory)). Recommend: a persisted
  `MapReveal` bitset per map layer on a coarse grid (e.g. cells of the streaming grid), revealed by exploring a radius,
  by towers/guides/items (`reveal(region)` effect), and by quest/dialogue (`ShowMap`).
- **Local map:** top-down render of surroundings for interiors/towns (Morrowind's local map; Skyrim's); needs interior
  map stamps (sketch from the cell's baked data): later, UI-lane.
- **Markers:** discovered-place pins, player custom pins (a pin tool is the **player-drawn note** standard: Skyrim custom
  marker, Witcher pins, Elden Ring map markers with icons (memory)); free-text notes are rare and cheap (data string
  per pin), recommend "pins with an icon and optional text".
- **Compass:** strip with place/quest pins, distance fade, tracked quest only (Skyrim/Witcher/Cyberpunk). Framework
  default Compass, Morrowind adapter None (as dialogue-quests recommends); `Guidance` (navigation hint line along the
  road graph; KCD2/RDR2 minimap-path, GoT wind) is the third setting and **reuses road-graph pathing** (nav.md NV12):
  one route query serves guidance line, horse auto-ride and travel ETA.
- **Procedural world map (our 10x Vvardenfell):** generate a **stylised map from data, at bake** (once, under a few
  minutes): terrain height -> hillshade + biome/region colour tint + coast, rivers/roads from road graph, settlement icons
  from the town set, place names from the cell/region records; export as a tiled image pyramid (mip levels) so zoom is
  cheap; fog-of-war is a separate mask drawn over it, never baked in. A "parchment" style is a shader pass over the
  hillshade (cartographic standard: hillshade + hypsometric tints + contours, memory; the Magna Urbis/worldgen
  tools already own heights and roads, so no new generator). Style is a data profile (generic renderer rule), and
  Morrowind adapter ships the 1:1 painted-look profile. Live camera top-down render is the fallback for the local map.
- **Orientation helpers in the generated 10x world:** road signs/signposts at junctions (data: Magna Urbis road graph
  nodes carry names), landmark visibility (Red Mountain, towers) as the main Morrowind navigation (adapter), the
  "ride-along" itself as the map teacher.

## 6. Co-op travel rules (2-4, host authoritative)

- **Who decides:** a vehicle is host state with seats. Boarding is per player (any player may board; seats are taken in
  order); the vehicle departs when (a) all present party members seated, or (b) boarding timer (e.g. 10-20 s, tunable)
  elapsed, or (c) the **leader** (host/party leader) confirms. Players not seated are left behind at the stop and may
  catch a later ride, teleport-to-host, or walk. This is the Valheim/ESO-like "never block" rule (memory).
- **Time:** ride time is real game time on the shared clock; the **skip** is a party vote (or the driver/leader
  choice; consistent with net.md rest vote already in `services.md` open question 1): all seated riders agree or the
  skip is declined; riders are seated so skip can pass real time only for seated players, **if some are on foot the
  time skip is not allowed** (the world shared). Services-spec fallback (travel arrives "instantly" in co-op) stays as
  Classic.
- **Teleport-to-host:** "join host" action: to a safe point near the host (nearest road/node), blocked in combat/during
  cutscenes; costs nothing or a short cast (recommended); also used after being left behind. Mark/Recall are
  personal.
- **Followers/pets/mounts:** `Teleport` moves buyer + followers (spec) and, in co-op, the mount of each rider; your mount is
  summoned at the destination on arrival (whistle rule).
- **Separation:** the nav.md residency rule (NPCs near remote players) must keep the vehicle's route streamed while any
  player is riding; a rider far from the host is a second streaming anchor: the 10x world makes this real.
- Prediction: driver predicts own mount (interaction (c)); vehicles with passengers only are host-simulated with
  snapshots; no mid-ride authority transfer.

## 7. Gaps vs our spec, ranked by player impact

| # | Gap | Rec | Why |
|---|---|---|---|
| 1 | Travel is a menu with a fade; no route/stop/operator/vehicle records, so ride-along has no data | **Add** `TravelNetwork`/`Route`/`Operator`/`Vehicle` records to `services.md`; Morrowind rules map onto them | single model for striders, boats, propylons, carts, trains |
| 2 | 10x world: ride/walk times are minutes; no skip, no auto-ride | **Add** time-skip (catch-up sim with `PassTime`) and a **road-following auto-ride/auto-walk** (RDR2 cruise, KCD2 walk-the-route) | long routes unplayable without it; reuses NV12 road graph |
| 3 | No encounter/ambush model on routes | **Add** per-route `encounter_table` with seeded tick roll, killable operator, anti-strand rule; off by default in Morrowind adapter | DD2/KCD2 standard; makes travel interactive |
| 4 | Mounts spec lacks controls, summon/whistle, stamina, bond, dismount-on-hit, NPC riders | **Extend** interaction (c): `RiderToMount` command map, whistle via nav, mount stamina, bond stat | the listed items are small once seats exist |
| 5 | No map spec (world/local, fog mask, pins, procedural map) | **New spec** `map.md` in the UI lane: layered map, `MapReveal` mask, pin data, procedural stylised map baked from heights+roads | needed for 10x world; owner sees it first |
| 6 | No fast-travel policy or rule gates | **Add** `TravelCost` rule + gates (alert, quest locks, discovered); free fast travel opt-in | answers "fast travel ruins open worlds" without removing it |
| 7 | Co-op travel: only "services pass time" fallback | **Add** boarding/leader/vote rules above; teleport-to-host | before the first co-op build |
| 8 | Walk-on vehicles | **Keep** last, as in spec; strider = rail+seats | nothing changes |
| 9 | Mount locomotion clips/quadruped controller | **Keep** as risk; sequence mounts before mounted combat (owner) | already in spec |

## Candidate owner questions (design/player-facing only, recommended answer)

1. **May an ambush hit you or kill the driver during a ride (cart destroyed, you must walk on)?** Recommended: yes in
   the framework default, off in the Morrowind adapter (vanilla has none); the owner picks the Morrowind default.
2. **When you skip a long ride, is the world time spent (hours pass, needs advance) or is it a cut?** Recommended: time
   is spent (consistent with Morrowind hours and KCD2).
3. **Is free teleport to every discovered place ever allowed in the modern preset?** Recommended: not by default (an
   opt-in setting), because the ride and the road are the 10x world's content; magic teleport (Mark/Recall,
   Intervention) always works.
4. **Do mounts need the player to whistle them over (call from anywhere nearby), or only stables/hitching posts?**
   Recommended: whistle within a range, mount runs to you, with a bond that grows with use.
5. **Does the map reveal as you explore (Morrowind), or do landmarks/guides/maps reveal it?** Recommended: both
   (exploration paints, towns/guides/buying maps reveal a region); Morrowind adapter ships exploration-only.
6. **Can players leave a moving vehicle mid-route?** Recommended: yes on land vehicles, no at sea or in the air.

## Missing / not verified
- No primary source (devs' talks) for RDR2/KCD2 travel internals; only press guides. KCD2 encounter probabilities and
  RDR2's skip rules were not found. Witcher 3 boat behaviour and Torrent specifics are from memory.
- Crate survey: none needed; Rapier vehicle controller version/Bevy-0.20 fork path not checked (only relevant to physics
  carts/boats, a later slice).
- Cinemachine-style rail camera options and UI-lane map rendering cost not measured.
- Source-engine mechanics of Morrowind striders/propylons (OpenMW) not re-read here; the adapter spec covers the
  guide travel rule only, propylon/boat details are for the content lane.
