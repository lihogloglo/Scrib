# Lane roadmap: Travel (rides, fast travel, mounts, maps)

Research: `docs/research/gameplay-2026-11-25/travel.md` (the "§n" references below). Specs it drives:
`superfargoth/docs/specs/services.md` (list services), `interaction.md` "(c) Mounts and vehicles", a new
`superfargoth/docs/specs/travel.md`, a new `map.md`, and `bevyrrowind/docs/specs/services-adapter.md` "Travel".
Visual checks: new file `bevyrrowind/docs/visual-checks/travel.md` (prefixes TV-, MAP-; IN- in `interaction.md`).

## 1. Player-facing goal

**Modern preset, early (Stage 1).** Every Morrowind network works: pay a caravaner, shipmaster, guild guide or
gondolier, fade, the hours pass, you arrive with your followers. Mark, Recall, Intervention and propylons teleport.
A killed operator is always replaced: no place is ever cut off.

**Modern preset, done.** Stand or sit on the strider as it walks its route through the live world; hold to skip.
Walk a boat's deck as it sails. Leave any vehicle anywhere, even overboard at sea (allowed, not advised). Ambushes can
wreck a ride. Mounts steer like RDR2 horses, come to a whistle, ride a road to a pin. No free teleport unless turned
on. A world map paints as you explore and takes your pins.

**Classic option.** Morrowind's menu: pick, pay, fade, arrive with the hours passed (`services-adapter.md` "Travel"
formula), no ambushes, exploration-only reveal; operators are replaced here too (owner Q4).

## 2. Where we are (2026-11-25)

- **Services:** SV1-SV3a (`ServiceEffect::Teleport`/`PassTime`) unit-tested, untested in game; A1 rules (incl.
  `mw.services.travel`), TR1, A3 headless only; PUI-SV3/SV4 awaiting owner. **A4** (travel service): not started
  (services-adapter.md "Build plan"; ROADMAP 2.7).
- **Routes, stops, operators, vehicles:** none; travel is a menu row (§1, gap 1). TV-R1 research running.
- **Seats, decks:** `interaction.md` (c) designs `SeatLink`, four vehicle kinds, IN7, IN10, IN11; ROADMAP 3.11 `[ ]`.
  `GroundHit` carries the ground body, its point velocity and angular velocity (IN20, done 2026-11-25).
- **Nav:** `humanoid`, `large` baked (ledger 2026-11-18); NV12 a straight-line stub (nav research §2: road graph);
  AI10 waits for NV12. **Clock:** `skip(ms)` (`clock.md`). **Maps:** no spec. **Co-op:** N1-N8 parked `[p]`.
- **Owner:** interaction 2026-11-11 (mounts first, mounted combat later); DECISIONS 2026-11-25; section 7 answers.
- **Frozen (owner 2026-11-25):** ships (vessels spec SHP-0 and after; ship questions stay open) and mount research
  (TV-R2). IN11 goes ahead as generic moving-platform support.

## 3. Decisions

1. **One data model** (§1-2; DD2, KCD2): `TravelNetwork { stops, routes }`, `Route { a, b, path, speed, price_rule,
   schedule: Always | OnCall | Timetable }`; operator = an ordinary actor; vehicle = motion source + passenger places.
2. **Instant first = a ride skipped at boarding** (owner 2026-11-25): until Stage 3 lands, every trip in every preset
   is A4's fade (price, followers, route time through `skip(ms)`); TV6 later adds a presenter on the same networks.
   Paths are baked (`large` nav paths, water-mask search, Magna Urbis roads via NV12; read, never made).
3. **Vehicle = motion x place** (replaces "strider = rail vehicle"; owner Q2, Q5). Motion: route-driven kinematic body
   (bob/roll are part of its pose, a pure function of tick and route, so co-op agrees), a mount controller, later a
   physics body (kind 3, buoyancy 3.7). Place: walkable deck or seat. Ships are decks, never a disguised mount;
   characters put no force on a deck (Sea of Thieves, memory).
4. **Moving decks = based movement** (Sea of Thieves; UE `MovementBase` / Mover 2.0 based movement; Jolt
   `CharacterVirtual` ground velocity): the probe returns the ground body and point velocity (rapier via `sf_physics`;
   the mover is ours, so is the carry); the mover keeps a base-local pose, applies the base's tick delta first,
   collides in that frame, leaves with the point velocity. Drawing, camera, saves, replication are base-relative.
5. **Seats = attach + IK** (spec (c) `SeatLink`; RDR2 mount by side; UE socket attach + Control Rig IK): mode
   `Sit | Stand | Astride`; pelvis to seat, feet to footrest/stirrup/deck, hands to rail/reins; leave by a dismount
   spot or jump off with the point velocity, anywhere (owner Q2). Land vehicles wait (tunable); boats slow down and stop, wait, then go on (Q6).
6. **Mounts steer like RDR2 horses** (owner Q5): a `MoverParams` profile (speed, slow acceleration, momentum, turn
   rate falling with speed, gait ladder on sprint taps); `RiderToMount` maps the rider's `Command` (Unreal possession)
   for players, remote players and NPC riders. Legs: N-leg foot placement over the creature's own gaits, pelvis/spine
   pitch and roll from foot heights (Bereznyak "IK Rig", GDC 2016; UE Control Rig). Whistle, stamina, buck-off.
7. **Operators always replaced, every preset** (owner Q4): network slices register operators with TV26; a vacant stop
   keeps arrivals; a replacement (adapter template) spawns when the stop is next out of view, after a minimum delay;
   a load check logs stops with no live or pending operator.
8. **Cost**: a vehicle is one transform in the ring, a number outside. **Encounters** (§2, real rides only):
   per-route hazard per km by time of day, host-seeded tick roll, spawn ahead out of view; a wreck leaves the player
   at a reachable road point; a skipped ride stops at the ambush.
9. **Gates** (§3): `TravelCost { gold, hours, needs, encounter_risk }` rule; gates (alerted hostiles, `no_travel`,
   undiscovered, adapter interior rules) for services and teleport effects; "travel anywhere discovered" opt-in, off.
   Mark/Recall/Intervention are effect teleports (`RecallAnchor`, nearest tagged place); propylons are `Position*`.
10. **Map** (§5): baked tile pyramid, data style profile, saved `MapReveal` fog, pins; Morrowind compass without quest
    markers. **Co-op** (§6): vehicles are host state; departure when all are aboard, a window ends or the leader
    confirms; skip needs everyone aboard. **Tunables** (`Guess`): speeds, windows, waits, hazards, ranges, delays,
    mount profile, deck tolerances.

## 4. Stages

### Stage 0: specs and research (docs only)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| TV-R1 | OpenMW (file + commit): providers, `DODT` counts per kind, placements, propylons, Mark/Recall (running: `travel-data.md`) | docs | - | counts per kind |
| TV-R2 | Quadruped gaits (Morrowind creature clips, N-leg IK, crates), mount practice, deck-local nav | docs | - | **parked** (owner 2026-11-25: no mount research for now) |
| TV0 | New spec `travel.md`: decisions 1-2, 7-10, saves, budgets, reuse check, industry comparison | docs | - | doccheck |
| TV0b | `interaction.md` (c) rewrite: decisions 3-6, passenger state; rows IN7, IN10, IN11, IN20-IN26 | docs | TV0, TV-R2 | doccheck |
| TV0c | `services-adapter.md` "Travel": networks (strider star, boats, guides, gondolas), operator templates, options | docs | TV-R1, TV0 | doccheck |

### Stage 1: instant travel on every network (player-usable)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| TV1 | `sf_travel` core: networks, stops, operators, `RideState`, ETA, skip math (instant = skip at boarding), saves | `sf_travel` | TV0 | exact: arrival tick, skip ms, round trip |
| TV5 | Networks from records, every provider kind (widened 2026-11-25, was striders); operators registered with TV26 | `mw_rpg` | TV1, TV0c | exact synthetic; log counts per kind |
| A4 | Instant travel, every preset: price, hours (Classic formula, Modern route time), fade, followers, `GetPCTraveling` | `mw_rpg(_bevy)` | A1, A3, TV5 | exact; owner PUI-SV5 + TV-1 (a trip per kind) |
| TV10 | `TravelCost` rule + gates for services and teleport effects | `sf_travel`, `sf_rpg` | TV1 | exact per gate |
| TV26 | Operator replacement: vacancy, out-of-view spawn after the delay, orphan-stop check | `sf_travel`, `mw_rpg` | TV5 | exact; log 0 orphan stops on game data |
| TV14 | `RecallAnchor` (saved) + nearest-tagged-place query | `sf_travel` | TV1 | exact; round trip |
| TV15 | Mark, Recall, Divine/Almsivi Intervention on TV14 + TV10 | effects adapter | TV14, E-slices | exact per effect |

### Stage 2: maps (no dependency on rides)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| MAP0 | New spec `map.md`: layers, `MapReveal`, pins, pyramid, compass/guidance, profiles | docs | TV0 | doccheck |
| MAP1 | `MapReveal` core: bitset per layer, reveal radius/region, discovery events, save | `sf_map` | MAP0 | exact; round trip |
| MAP2 | Map bake: hillshade, tint, coast, roads, settlements, names into a tile pyramid | `sf_map`, `mw_bake` | MAP0 | log bake ms, MiB; owner MAP-1 |
| MAP3 | Map screen: pan/zoom, fog, places, pins, `ShowMap`/`FillMap` | `sf_ui_screens`, `mw_rpg_bevy` | MAP1, MAP2 | view-model; owner MAP-2 |
| MAP4 | Compass strip (Morrowind: places only) | `sf_ui_screens` | MAP1 | owner MAP-3 |
| MAP5 | Morrowind painted profile + reveal rule (owner Q3) | `mw_*` data | MAP2, MAP3 | owner MAP-4 |
| TV16 | Opt-in "travel anywhere discovered", off by default, gated | `sf_travel_bevy` | MAP3, TV10 | exact: off; gated |

### Stage 3: standing, sitting and walking on moving things (interaction lane, framework)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| IN20 | `GroundHit` carries the ground body and point velocity; kinematic velocity from the tick pose delta | `sf_physics`, `sf_controller` | - | **done 2026-11-25** (4 exact tests, 30/144 Hz) |
| IN11 | Based movement on any moving body (lifts, platforms, cart beds, decks; owner 2026-11-25): walk on a moving, turning, bobbing base (decision 4) | `sf_controller` | IN20 | idle drift in tolerance, jump lands on the spot, 30/144 fps alike, rebase; ms |
| IN21 | Deck presentation: drawn pose, foot IK, camera interpolated base-local | `sf_controller_bevy`, `sf_camera` | IN11 | drawn offset constant at rest; owner IN-11 (`level:` deck) |
| IN7 | Seat links on a moving owner: create/persist/delete, seat modes, dismount spot, jump off (rail half dropped) | `sf_interaction` | IN2, IN20 | exact transitions; lands on navmesh |
| IN23 | Seat IK: pelvis to seat, feet to footrest/stirrup/deck, hands to rail/reins | `sf_anim_graph` | IN7, Kimodo clips | pelvis in tolerance; owner IN-23 |
| IN24 | Passenger state `{ vehicle, seat or base-local pose }`: save, `Boarded`/`Left`, base-relative replication | `sf_interaction`, `sf_save` | IN7, IN11 | exact: mid-ride save/load |
| IN22 | Deck-local nav: navmesh in the vehicle frame; NPCs walk a moving deck | `sf_nav(_bevy)` | IN11 | NPC walks bow to stern under way |

### Stage 4: real rides, striders and boats (needs Stage 3)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| TV2 | Route bake: nav polyline to arc-length spline with ground clearance | `sf_travel` | TV1 | exact synthetic heightfield |
| TV3 | Strider routes from `large`-class paths between stops | `mw_bake` | TV2, TV0c | log built/failed, ms |
| TV4 | Ride runtime: kinematic deck on `FixedUpdate`, depart/arrive, hold-to-skip, off-ring progress | `sf_travel_bevy` | TV1, IN11 | same arrival at 30/144 fps; skip delta exact |
| TV6 | Modern presenter: "Board", stand or sit on the strider, followers aboard, skip, arrive | `mw_rpg_bevy`, `bevyrrowind` | A4, TV3, TV4, IN7, IN21, IN24 | owner TV-2 (Seyda Neen to Balmora); hitches/min |
| TV8 | Leave any vehicle anywhere (Q2): land vehicles wait then go on, boats slow down, stop, wait, then go on (Q6); re-board at a stop | `sf_travel_bevy` | TV4, IN7, IN11 | exit on navmesh or water; vehicle resumes |
| TV12 | Water-route bake on the water mask with coast clearance | `sf_travel`, `mw_bake` | TV2 | exact synthetic island; log |
| TV13 | Boats as walk-on decks: ridden routes, tick-pure bob/roll, crew on deck nav | `mw_rpg(_bevy)` | TV12, TV6, IN22 | owner TV-3 (Ebonheart to Sadrith Mora) |

### Stage 5: risk on the road (needs real rides)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| TV7 | Encounter tables + seeded tick roll; skip resolves to "ambush at f" | `sf_travel` | TV1 | exact: same seed same result |
| TV9 | Ambush runtime: halt, spawn ahead out of view, operator/vehicle damageable, wreck, anti-strand | `sf_travel_bevy` | TV7, TV8, AI15, AI4 | reachable road point; owner TV-4 |
| TV11 | Morrowind tables from region creature lists; Modern on, Classic off (Q1) | `mw_rpg` | TV7, TV9 | exact; owner TV-5 |

### Stage 6: mounts and riding the roads

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| IN25 | Mount locomotion profile (RDR2): speed, inertia, turn rate by speed, gait ladder | `sf_controller`, `sf_action` | TV-R2 | exact stop distance, turn radius |
| IN26 | Procedural quadruped legs: N-leg foot placement, pelvis/spine pitch-roll, stride warp | `sf_anim_graph` | TV-R2 | no foot through a slope; owner IN-26 (guar) |
| IN10 | Creature mount: mount/dismount by side, `RiderToMount`, astride seat, mounted gaits | `sf_interaction` | IN7, IN23, IN25, IN26 | exact hand-off; owner IN-10 |
| TV17 | Whistle: mount paths to the caller, out-of-view fallback | `sf_interaction`, `sf_ai` | IN10, AI6 | arrives or fallback, never in view |
| TV18 | Stamina, buck-off on a heavy hit, door dismount (mount tied, saved) | `sf_interaction` | IN10 | exact thresholds; lands on navmesh |
| TV19 | Auto-ride the road graph to a pin or stop, hands-off camera | `sf_travel_bevy` | NV12, MAP3, TV17 | synthetic road graph; owner TV-6 |
| TV20 | Guidance line along the same route | `sf_ui_screens` | TV19, MAP4 | owner MAP-5 |
| TV21 | NPC riders and caravans on roads, tier 0 off-screen | `sf_ai`, `sf_travel` | TV19, AI10 | tier invariance; owner TV-7 |

### Stage 7: co-op travel (waits for net N1-N5, parked)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| TV22 | Boarding window, leader confirm, left-behind players stay | `sf_travel_bevy` | TV6, IN24, N-slices | loopback exact |
| TV23 | Skip vote; refused while anyone is not aboard | `sf_travel_bevy` | TV22 | loopback exact |
| TV24 | Teleport-to-host to a safe road point; mounts follow | `sf_travel_bevy` | TV22, TV17 | loopback exact |
| TV25 | A far rider as a second streaming/nav anchor | streaming, `sf_nav_bevy` | NV5, TV22 | residency set exact |

**Later:** timetable boats, physics carts/rowboats (kind 3), mounted combat, flying mounts, local map, mount bond,
ride camera. **Dropped (owner 2026-11-25):** the standing-idle strider passenger (old decision 4) and IN7's rail half
(route motion is TV4).

## 5. Spec changes (as spec slices, before build)

- TV0: new `superfargoth/docs/specs/travel.md`; services.md "List services" and "Co-op" point to it (`Teleport` +
  `PassTime` is the instant presenter for every preset until TV6). TV0b: `interaction.md` (c) and its build rows.
  TV0c: `services-adapter.md` "Travel", "Options". MAP0: new `map.md`. NV12 and IN22 are reviewed by the nav lane.

## 6. Dependencies on other lanes

- **Interaction lane owns** TV0b and every IN row: Stage 3 (IN20, IN11, IN21, IN7, IN23, IN24, IN22) and the mount
  body (IN25, IN26, IN10), on IN0 and IN2; no lane roadmap yet, so these rows are its plan. **Travel owns** TV*, MAP*,
  A4. Stages 1-2 need nothing from interaction; Stage 4 waits for Stage 3, Stage 5 for Stage 4, TV17-TV18 for IN10.
- **Anim:** sit, stand and astride rider clips (enter/exit by side, gait-matched loops) from Kimodo (owner), not combat
  clips, for IN7, IN23, IN10; creature gaits are Morrowind's own; IN26 extends `FootPlacement`/`StrideWarp` to N legs
  (the anim lane may take it and TV-R2's gait part).
- **Physics / water:** IN20 body velocities; buoyancy 3.7 for kind 3 only; swimming 3.6 before overboard is checked.
- **Nav:** NV12 before TV19-TV21, NV5 before TV25. **AI:** AI4 and AI15 before TV9, AI6 before TV17, AI10 before
  TV21. **Effects / scripting:** TV15 requests, `Position*` propylons. **Net:** N1-N5 before Stage 7.
- **Provides:** ETAs and road queries to AI tier 0; gates to effects; map reveal and pins to dialogue-quests and
  worldgen.

## 7. Owner questions (each with a recommended answer)

1. **Can Modern strider and boat rides be ambushed?** Recommended: yes, rare, scaled by region danger; never in
   Classic. **Answered 2026-11-25:** yes, as recommended.
2. **Can players leave a moving vehicle mid-route?** Recommended: on land only. **Answered 2026-11-25:** "No: players
   can always leave a vehicle, also a ship at sea (walk off, jump overboard: allowed, not advised). Ships are walkable
   moving decks (Sea of Thieves), never "a mount disguised as a ship"."
3. **Map reveal: exploration only, or also guides and bought maps?** Recommended: both in Modern, exploration in
   Classic. **Answered 2026-11-25:** yes, as recommended.
4. **Is a killed caravaner or shipmaster replaced?** Recommended: Modern only. **Answered 2026-11-25:** "Operators are
   always replaced, in every preset: a place must never be cut off from the travel network."
5. **Mount bond (RDR2-style)?** Recommended: later, a data stat. **Answered 2026-11-25:** "Mounts (horses etc.) steer
   like RDR2 horses: faster, more inertia, procedural leg animation. The character sits or stands naturally on the
   creature or vehicle, and walks on moving decks: that is the big interaction slice. Plan all of it now; until that
   slice lands, travel is an instant teleport (fade, game time spent)." Applied in sections 3-4; bond stays "Later".
6. **Leaving a boat at sea: does it sail on or come about for you?** Recommended: sails on to its stop (fare spent);
   land vehicles wait a short while, then go on.
   **Answered (owner 2026-11-25):** the boat slows down and eventually stops (Sea of Thieves); after a while the crew
   loses patience and goes on, like land vehicles (wait time = a tunable).
