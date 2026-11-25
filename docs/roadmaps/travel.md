# Lane roadmap: Travel (rides, fast travel, mounts, maps)

Research: `docs/research/gameplay-2026-11-25/travel.md` (the "§n" references below). Specs it drives:
`superfargoth/docs/specs/services.md` (list services), `interaction.md` "(c) Mounts and vehicles", a new
`superfargoth/docs/specs/travel.md`, a new `map.md`, and `bevyrrowind/docs/specs/services-adapter.md` "Travel".
Visual checks: new file `bevyrrowind/docs/visual-checks/travel.md` (prefixes TV-, MAP-).

## 1. Player-facing goal

**Modern preset.** Pay the caravaner, climb aboard, and the strider walks its route through the live world; hold
to skip and the ride's hours pass. Hop off on land. Ambushes can stop a ride, kill the driver, wreck the cart; you
walk on. Boats sail the same way; guild guides, propylons, Mark/Recall and Intervention teleport. No free teleport
to discovered places unless turned on. A world map paints as you explore and takes your pins; a whistle calls your
mount, which can ride a road to a pin by itself.

**Classic option.** Morrowind's menu: pick a destination, pay, fade, arrive with the hours passed
(`services-adapter.md` "Travel" formula), no ambushes, exploration-only map reveal.

## 2. Where we are (2026-11-25)

- **Services:** SV1-SV3a (`ServiceEffect::Teleport`/`PassTime`) built, unit-tested, untested in game; adapter A1
  rules (incl. `mw.services.travel`) and the list host path (TR1, A3) headless only; PUI-SV3/SV4 awaiting owner.
- **Travel service A4** (destinations, teleport with followers, time pass, `GetPCTraveling`): not started
  (services-adapter.md "Build plan"; ROADMAP 2.7 "next ... travel/spells").
- **Routes, stops, operators, vehicles:** none exist; travel is a menu row (§1, gap 1).
- **Seats:** `interaction.md` (c) designs `SeatLink`, four vehicle kinds, IN7 (rail) and IN10 (mount); ROADMAP
  3.11 `[ ]`, nothing built.
- **Nav:** `humanoid` and `large` classes baked (ledger 2026-11-18), ring, queries, corridor (NV4c, NV6, NV8). NV12 is
  a straight-line stub; nav research §2 proposes NV12 = road/route graph. AI10 (`fast_forward`) waits for NV12.
- **Clock:** `clock.md` has host-owned `skip(ms)` for rest, wait, train and travel.
- **Maps:** no spec; only the `ShowMap` opcode is named in the quests research.
- **Co-op:** N1-N8 parked `[p]` (ROADMAP); services' co-op rule: travel time is a party rest vote, else skipped.
- **Owner decisions:** interaction answers 2026-11-11 (4: general mount mechanic first, mounted combat later; 5:
  ride-along modern, fade Classic); DECISIONS 2026-11-25 (ambushes can wreck the cart, skipped rides spend game time,
  no free teleport by default, whistle for the mount).

## 3. Decisions

1. **One data model, two presenters** (§1-2; DD2, KCD2). `TravelNetwork { stops, routes }`, `Route { a, b, path:
   RouteRef, speed, price_rule, schedule: Always | OnCall | Timetable }`; operator = an ordinary actor (killable,
   can be absent); vehicle = an entity with seats. Classic = today's `Teleport` + `PassTime` row; modern = board and
   ride (the row gains an optional route). New Bevy-free `sf_travel` (+ thin `sf_travel_bevy`): services,
   interaction, AI tier 0 and the map all read networks.
2. **Skip is a time skip, never a cut** (owner 2026-11-25; RDR2/KCD2): the vehicle jumps to the stop and the clock
   advances by the remaining ride time through `skip(ms)`; tier-0 actors catch up through AI10 `fast_forward` once it
   exists (before AI10, schedules and purses see the clock as after a rest).
3. **Route paths are baked** (`RouteRef = Road | Spline | Baked`). Striders: a `large`-class Detour path between
   stops at bake, simplified to an arc-length spline; boats: grid search on the water mask; 10x world: Magna Urbis
   roads via NV12. Travel reads roads, never makes them (owner 2026-11-24).
4. **Strider = rail vehicle** (kind 1, §4), not a walk-on deck. Departs from IN7: the passenger seat may use a
   standing idle until IN2's sit clips exist.
5. **Cost**: one transform per vehicle in the ring, a number outside it; strider speed is below what streaming
   handles for horses and flight (1.2 bench). Hitches/min from one run's gauges.
6. **Encounters** (§2, owner): per-route `encounter_table` (hazard per km by time of day), host-seeded fixed-tick
   roll; spawn out of view ahead, vehicle halts, operator and vehicle are targets; a wreck leaves the player at a
   reachable road point or last stop. A skipped ride stops at the ambush point with a warning.
7. **Gates and cost** (§3): `TravelCost { gold, hours, needs, encounter_risk }` is a rule; gates (alerted hostiles,
   a `no_travel` fact, undiscovered destination, interior rules in the adapter) apply to services and to teleport
   effects alike. Free "travel anywhere discovered" exists as an opt-in setting, off by default (owner 2026-11-25).
8. **Magic and scripted teleports are not networks**: Mark/Recall/Intervention are `effects.md` teleport requests
   using a `RecallAnchor` and a nearest-tagged-place query; propylons are MWScript `Position*` activators; guild
   guides are A4's interior-provider case (0 h).
9. **Mounts** (§4, owner: whistle): `RiderToMount` maps the rider's `Command` to the mount's `MoveIntent` (Unreal
   possession), same for remote players and NPC riders. Whistle = nav route to the caller, out-of-view fallback past
   a range; stamina, buck-off, door dismount. One road route query serves auto-ride, guidance line and ETAs.
10. **Map** (§5): baked once (heights, tints, coast, roads, towns, names) into a tile pyramid; style = data profile;
    fog = a saved `MapReveal` bitset drawn over it. Pins with icon + text. Compass framework default; Morrowind: no
    quest markers (owner 2026-11-25).
11. **Co-op** (§6): vehicles are host state; departure when the party is seated, a window ends or the leader
    confirms; skip needs all riders and is refused while anyone is on foot; teleport-to-host; mounts follow.
12. **Tunables**: ride speed, boarding window, hop-off wait, hazard rates, whistle range, reveal radius (`Guess`).

## 4. Stages

### Stage 0: specs (docs only)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| TV-R1 | Research (OpenMW file + commit): travel providers and `DODT` counts, strider/boat placements, strider mesh animations, propylon scripts, Mark/Recall/Intervention | docs | - | note with counts per kind |
| TV0 | New spec `travel.md`: decisions 1-3, 5-8, 11-12, with saves, budgets, reuse check, industry comparison | docs | - | doccheck links |
| TV0b | `interaction.md` (c) rewrite: kind-1 passenger pose, `RiderToMount`, whistle, stamina, buck-off, NPC riders; IN7/IN10 rows amended | docs | TV0 | doccheck |
| TV0c | `services-adapter.md` "Travel": network mapping (striders `OnCall` star, boats, guides), Classic/Modern option rows, route bake | docs | TV-R1, TV0 | doccheck |

### Stage 1: a silt strider ride end to end (player-usable)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| A4 | Classic fade travel: destinations, price/hours, teleport with followers, `GetPCTraveling` one frame | `mw_rpg`, `mw_rpg_bevy` | A1, A3 host path | exact headless; owner PUI-SV5 (Seyda Neen caravaner to Balmora) |
| TV1 | `sf_travel` core: networks, routes, `RideState` stepping, ETA, skip math (remaining ms, arrival stop), saves | `sf_travel` | TV0 | exact: arrival tick, skip lands at stop with exact ms, round trip |
| TV2 | Generic route bake step: nav path polyline to arc-length spline with ground clearance | `sf_travel` (bake fn) | TV1 | exact on a synthetic heightfield: length, clearance, no self-crossing |
| TV3 | Strider routes baked from `large`-class paths between stop anchors | `mw_bake` | TV2, TV0c | log: routes built/failed, bake ms |
| TV4 | Ride runtime: vehicle on `FixedUpdate` + interpolation, depart/arrive events, hold-to-skip through `skip(ms)`, off-ring progress | `sf_travel_bevy` | TV1, IN7 | headless: same arrival at 30/144 fps; skip clock delta exact |
| TV5 | Strider networks from records: star `OnCall` network, operator = provider NPC, vehicle = nearest strider placement | `mw_rpg` | TV1, TV-R1 | exact on synthetic records; log counts on the game data |
| TV6 | Modern presenter in game: "Board" row, seat on the strider, ride, skip, arrive, dismount at the stop; Classic keeps A4 | `mw_rpg_bevy`, `bevyrrowind` | A4, TV3-TV5, IN0, IN7 | owner TV-1 (Seyda Neen to Balmora, skip halfway); hitches/min during the ride from one run's gauges |

### Stage 2: risk and rules on the road

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| TV7 | Encounter tables + seeded tick roll; skip resolves to "ambush at fraction f" | `sf_travel` | TV1 | exact: same seed same result; hazard per km |
| TV8 | Hop off mid-route on land vehicles; vehicle waits, then continues; re-board at a stop | `sf_travel_bevy` | TV4, IN7 | headless: dismount spot on navmesh; vehicle resumes |
| TV9 | Ambush runtime: halt, spawn out of view ahead, operator/vehicle damageable, wreck, anti-strand, skip warning | `sf_travel_bevy` | TV7, AI15, AI4 | headless: wreck leaves player at a reachable road point; owner TV-2 |
| TV10 | `TravelCost` rule + gates for services and teleport effect requests | `sf_travel`, `sf_rpg` | TV1 | exact per gate (alerted, `no_travel`, undiscovered) |
| TV11 | Morrowind encounter tables from region creature lists; Modern/Classic defaults (owner Q1) | `mw_rpg` | TV7, TV9 | exact on synthetic regions; owner TV-3 (Modern ride with an ambush) |

### Stage 3: the rest of Morrowind's network

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| TV12 | Generic water-route bake: grid search on the water mask with coast clearance | `sf_travel` (bake fn), `mw_bake` | TV2 | exact on a synthetic island; log routes built/failed |
| TV13 | Boats: shipmaster networks, boat vehicle on the water surface (wave bob presentation only), board/ride/skip | `mw_rpg`, `mw_rpg_bevy` | TV12, TV6 | owner TV-4 (Ebonheart to Sadrith Mora by boat) |
| TV14 | `RecallAnchor` (saved) + nearest-tagged-place query (straight or road distance) | `sf_travel` | TV1 | exact: nearest of a tagged set; save round trip |
| TV15 | Mark, Recall, Divine/Almsivi Intervention handlers on TV14 + TV10 gates | effects adapter | TV14, effects E-slices | exact headless per effect |

### Stage 4: maps

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| MAP0 | New spec `map.md`: layers, `MapReveal`, pins, baked pyramid, compass/guidance settings, profiles | docs | TV0 | doccheck |
| MAP1 | `MapReveal` core: coarse bitset per layer, reveal radius/region, place discovery events, save section | `sf_map` | MAP0 | exact: reveal sets, round trip |
| MAP2 | Map bake: hillshade, region tint, coast, roads, settlements, names into a tile pyramid | `sf_map` (bake), `mw_bake` | MAP0 | log: bake ms (< few minutes), MiB; owner MAP-1 |
| MAP3 | World map screen: pan/zoom, fog mask, discovered places, player pins with icon + text, `ShowMap`/`FillMap` | `sf_ui_screens`, `mw_rpg_bevy` | MAP1, MAP2 | headless view-model; owner MAP-2 |
| MAP4 | Compass strip (framework default; Morrowind: places only, no quest markers) | `sf_ui_screens` | MAP1 | owner MAP-3 |
| MAP5 | Morrowind painted map profile + reveal rule (owner Q3) | `mw_*` data | MAP2, MAP3 | owner MAP-4 |
| TV16 | Opt-in "travel anywhere discovered" from the map, off by default, through TV10 gates | `sf_travel_bevy` | MAP3, TV10 | exact: off by default; gated |

### Stage 5: mounts and riding the roads

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| TV17 | Whistle: mount paths to the caller, out-of-view fallback beyond range | `sf_interaction`, `sf_ai` | IN10, AI6 | headless: arrives or fallback; never in view |
| TV18 | Mount stamina, buck-off on a heavy hit, auto-dismount at doors (mount left tied, saved) | `sf_interaction` | IN10 | exact thresholds; sanity: rider lands on navmesh |
| TV19 | Auto-ride along the road graph to a pin or stop (cruise), hands-off camera | `sf_travel_bevy` | NV12 (road graph), MAP3, TV17 | headless on a synthetic road graph; owner TV-5 (`level:` greybox, guar profile) |
| TV20 | Guidance line along the same road route (third map/compass setting) | `sf_ui_screens` | TV19, MAP4 | owner MAP-5 |
| TV21 | NPC riders and caravans on roads; off-screen on the road graph (tier 0) | `sf_ai`, `sf_travel` | TV19, AI10 | headless tier invariance; owner TV-6 |

### Stage 6: co-op travel (waits for net N1-N5, parked)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| TV22 | Boarding window, leader confirm, left-behind players stay at the stop | `sf_travel_bevy` | TV6, N-slices | loopback: departure rules exact |
| TV23 | Seated skip vote; refused while a party member is on foot | `sf_travel_bevy` | TV22 | loopback exact |
| TV24 | Teleport-to-host to a safe road point; riders' mounts follow on arrival | `sf_travel_bevy` | TV22, TV17 | loopback exact |
| TV25 | A far rider as a second streaming/nav anchor | streaming, `sf_nav_bevy` | NV5, TV22 | headless residency set exact |

### Later (not scheduled)

Timetable vehicles (boats on a schedule), physics carts/rowboats (kind 3), walk-on decks (IN11), local map, bond,
ride camera, operator replacement (Q4).

## 5. Spec changes (as spec slices, before build)

- TV0: new `superfargoth/docs/specs/travel.md`; services.md "List services" and "Co-op" point to it (the row's
  `Teleport` + `PassTime` is the Classic presenter; the "arrives instantly" co-op fallback stays for Classic).
- TV0b: `interaction.md` (c) and IN7/IN10 rows. TV0c: `services-adapter.md` "Travel" and "Options".
- MAP0: new `map.md`. The NV12 change (road/route graph) is the nav lane's spec slice; this lane only reads it.

## 6. Dependencies on other lanes

- **Interaction:** IN0 and IN7 before TV4/TV6 (Stage 1 blocks on them); IN10 before Stage 5; IN11 for walk-on decks.
- **Nav:** NV12 as road/route graph (nav research §2) before TV19-TV21; NV5 before TV25. Stages 1-3 need no new nav
  work (baked `large` paths and water mask).
- **AI:** AI4 directives and AI15 combat before TV9; AI6 before TV17; AI10 `fast_forward` for full catch-up on skip
  (TV4 works before it, with clock-only catch-up) and before TV21.
- **Effects / scripting:** effects teleport requests for TV15; scripting `Position*` opcodes for propylons.
- **Anim:** quadruped locomotion and rider clips for IN10; IN2 sit clips upgrade the strider passenger pose.
- **Provides:** route ETAs and road queries to AI tier 0; `TravelCost` gates to effects; map reveal, pins and place
  discovery to dialogue-quests (`ShowMap`, markers) and worldgen (the 10x map).

## 7. Owner questions (each with a recommended answer)

1. **In Modern Morrowind, can strider and boat rides be ambushed?** Recommended: yes, rare and scaled by the region's
   danger (cliff racers, bandits on the Ascadian Isles road); never in Classic.
2. **Can players leave a moving vehicle mid-route?** Recommended: yes on land (strider, cart), no at sea or in the air.
3. **Morrowind map reveal: exploration only (vanilla), or also guides, bought maps and towers?** Recommended: both in
   Modern (exploring paints, buying a regional map or asking a guide reveals a region); exploration only in Classic.
4. **If the player kills a caravaner or shipmaster, does someone replace them?** Recommended: Modern, a new operator
   arrives after a few game days; Classic, never (vanilla).
5. **Mount bond (the mount grows calmer and stronger with use, RDR2-style)?** Recommended: yes, later, as a data stat
   on the mount; not needed for the first whistle and ride.
