# Travel adapter: Morrowind travel networks, operators and teleport effects on `sf_travel`

Status: **spec (TV0c, 2026-11-26)**, nothing built. Framework: [travel.md](../../../superfargoth/docs/specs/travel.md)
(`TravelNetwork`, `Stop`, `TravelRoute`, `OperatorSlot`, `RideState`, gates, `RecallAnchor`; this file maps its
"Adapter contract" items 1-6). Price and hours formula: [services-adapter.md](services-adapter.md) "Travel" (rule
`mw.services.travel`, built in A1, unchanged here). Lane roadmap: `docs/roadmaps/travel.md`. Research:
`docs/research/gameplay-2026-11-25/travel-data.md` (TV-R1). Followers:
[followers.md](../../../superfargoth/docs/specs/followers.md) "Travel". Effects: [effects-adapter.md](effects-adapter.md)
(`Request` effects). OpenMW citations: github.com/OpenMW/openmw at `7161a35271a9` (as TV-R1), paths under
`apps/openmw/` unless they start with `components/`.

> **Agent summary.** At load, `mw_rpg::travel` turns every `NPC_` with a `DODT` list into one stop per provider
> (key `mw.travel.stop:<kind>:<npc id>`), joins each `DODT` entry to the nearest same-kind stop within
> `mw.travel.stop_merge_m` (else an arrival-only stop), and groups stops into one `TravelNetwork` per kind (strider,
> boat, gondola, guide, misc) from a class/vehicle data table. Price is always `mw.services.travel`; time is the
> Classic hours rule or Modern route time; guides are 0 h teleports with the cast cue. A dead operator is replaced
> by a synthesized per-stop template (same race, sex, class, `DODT`; never the original id). Mark/Recall and the two
> Interventions are effect teleports on `RecallAnchor` and nearest-tagged-place over `TempleMarker`/`DivineMarker`.
> Propylons stay MWScript `PositionCell`.

## Goal

Every Morrowind travel provider (and every mod's, through the same records) becomes `sf_travel` data at load, so
Stage 1 instant travel works in every preset with OpenMW's fares, Classic or Modern time, followers and
`GetPCTraveling`, and no stop is ever cut off (owner Q4). The same data later drives real strider and boat rides.

Non-goals: the framework model and flow (travel.md); the fare/hours formula (services-adapter "Travel"); the
services list screen and refusal topic (services-adapter "Offers"); propylon scripts and `PositionCell`
(scripting-adapter); the map and discovery (MAP0); vehicle seats and decks (interaction (c)).

## Reuse check and industry comparison

- **Bevy / crates:** nothing applies (travel.md "Reuse check"). The `tes3` crate already exposes NPC travel
  destinations (position, rotation, cell), read today by `mw_world` adapter tests and `mw_doors_bevy`.
- **OpenMW** (behaviour ported as data and rules, no code copied): `components/esm3/transport.cpp` 13-34,
  `mwclass/npc.cpp` 588-605, `mwgui/travelwindow.cpp` 48-229, `mwworld/actionteleport.cpp` 20-109,
  `mwmechanics/spelleffects.cpp` 476-525, `mwworld/worldimp.cpp` 3191-3352, `components/esm3/player.cpp` 18-20.

| Game | Network source | Landing | Lost operator | What we take |
|---|---|---|---|---|
| Morrowind / OpenMW | `DODT` list on the NPC, a star per provider; OpenMW fare x (1 + followers) | raw `DODT` pose | network breaks | the data, fares, hours, follower filters, marker search |
| Skyrim (memory) | carriage drivers at hold stables, fixed list | fade at the stable | breaks | nothing new |
| Mod networks, e.g. Tamriel Rebuilt (memory) | same `DODT` providers, new classes | raw pose | breaks | the kind table is data, so mods need no code |

Beyond all of them: replacement keeps every network whole (travel.md "Operators and replacement").

**Records read:** `NPC_` `DODT` (pos xyz, rot xyz) + optional `DNAM` cell, class, race, sex, level; `CELL` refs
(provider placements, vehicle refs, `TempleMarker`/`DivineMarker`); GMSTs `fTravelMult` 4000, `fTravelTimeMult`
16000, `fMagesGuildTravel` 10, `sTravel`, `sTeleportDisabled`. No `AIDT` travel bit exists: a provider is an `NPC_`
with a non-empty destination list (`npc.cpp` 588-605); no `CREA` carries `DODT` (TV-R1 sec. 1).

## Networks per provider kind

**Kind lookup** (record `mw.travel.kinds`, base-mod data, mod-patchable; first match wins):
1. the provider's class id in the table (`Caravaner`, `Shipmaster`, `Gondolier`, `Guild Guide`);
2. else the nearest vehicle reference (strider, ship, boat, gondola ids in the table) within
   `mw.travel.vehicle_bind_m` of the placement;
3. else the provider's cell: interior gives `mw.travel.guide` rules on a `mw.travel.misc` network, exterior gives
   `mw.travel.misc` with vehicle-less Teleport mode.

The class is only a label in vanilla (TV-R1 sec. 1), so code never matches it: the table maps a class to
`{ kind, network, mode, profile, schedule }`, and mod classes are one more row.

| Kind (network id) | Providers / entries (TV-R1 sec. 2) | Mode | Profile | Schedule | Time Classic / Modern | Cue |
|---|---|---|---|---|---|---|
| Silt strider `mw.travel.strider` | 9 / 32, all exterior, 2-4 each | Vehicle | `mw.vehicle.strider` | OnCall | hours / route time | fade |
| Boat `mw.travel.boat` | 15 (11 + 4 Bloodmoon) / 43, exterior, 1-4 | Vehicle | `mw.vehicle.boat` | OnCall | hours / route time | fade |
| Gondola `mw.travel.gondola` | 5 / 16, exterior, 3-4 | Vehicle | `mw.vehicle.gondola` | OnCall | hours / route time | fade |
| Mages Guild guide `mw.travel.guide` | 5 / 20, all interior, 4 each | Teleport | none | Always | 0 / 0 | `mysticism cast` + fade |
| Misc `mw.travel.misc` | 4 / 6 (5 exterior, 1 interior): `rindral dralor`, `vevrana aryon`, `blatta hateria`, `todd` | by lookup | by lookup | Always | by lookup | by lookup |

- **Schedule.** Vanilla has no timetables. `OnCall` ("vehicle at the stop when the operator is") is set now so the
  data does not change at TV6/TV13; in Stage 1 it behaves as `Always`, since the vehicle is scenery.
- **Routes.** One directed route per `DODT` entry (`directed: true`): provider lists are not symmetric and a one-way
  entry stays one-way. `path: Estimate { length_m }` (travel.md) until TV3/TV12. `price_rule = mw.travel.price`,
  `time_rule` per the table above ("Rules by id").
- **Board point** = the provider's placement (where the operator stands); arrival-only stops have `board: None`.
- **Arrival pose.** Each route lands on its own raw `DODT` position and yaw (no landing search). The stop's `arrive`
  is the incoming `DODT` pose nearest its board point (map pins, queries); with no incoming route, the board point
  and the provider's yaw. Needs a per-route landing pose in the framework (`TravelRoute.arrive: Option<Pose>`,
  None = the target stop's): a TV1 request, open question 1.
- **Targets.** A `DNAM` interior: that cell + the `DODT` position; an unknown cell skips the entry with one error
  line (`travel: unknown cell "<name>" in <npc id> DODT #n`; `travelwindow.cpp` 140-167); a `DNAM` with no `DODT` is
  dropped by the parser (`transport.cpp` 13-34). No `DNAM`: the exterior cell at the position.
- **Guides** stand in interiors and send to the other guild halls ("Balmora, Guild of Mages", "Ald-ruhn, Guild of
  Mages", "Vivec, Guild of Mages", "Sadrith Mora, Wolverine Hall: Mage's Guild"). The flat fee keys on the
  provider's cell being an interior, never on the class (`travelwindow.cpp` 57-82): rule input `provider_interior`
  comes from the stop's board location.
- **No vehicle in the cell** (Daynas Darys, Baleni Salavel in Hla Oad, S'virr in Khuul; TV-R1 sec. 3): kind from
  the class row, empty vehicle binding until TV6/TV13. **`todd`** (test cells): built like any other.

**Propylons are not a network.** The 10 chambers form a ring of 20 directed links, each an `ACTI active_port_<x>`
whose script `Warp_<X>` checks `Index_<X>` and calls `Player->PositionCell` (TV-R1 sec. 4). They stay MWScript on the
scripting port (no fare, time or followers; the missing-index message is the script's): a data copy would duplicate
running content and diverge when mods edit the scripts. The scripting port's player `PositionCell` may call the
travel executor (`with_followers: false`, 0 ms; travel.md non-goals) so the "travelling" event stays one path.

## Stop identity

- **Key:** `mw.travel.stop:<kind>:<npc base id>` for a provider stop (`#<n>` in reference order if one base is
  placed twice, which vanilla never does); `mw.travel.stop:<kind>:arr:<npc base id>#<entry index>` for an
  arrival-only stop, after the first (provider id, entry index) pair in record id order that lands there.
- **Why the base id:** the key is computed from static records at every load, so a replaced operator (new actor,
  new record) keeps it; base ids survive load-order changes and mod patches, unlike reference indices or positions;
  one provider per stop keeps vanilla's per-NPC menus exact. A mod that deletes the provider turns its stop into an
  arrival-only one at the same spot, logged once.
- **Providers never merge** (one operator slot per framework stop; two lists may differ). Only **targets** merge: a
  `DODT` entry lands on the nearest same-kind provider stop whose board point is within `mw.travel.stop_merge_m`
  (same cell for interiors), else on an arrival-only stop of that kind; arrival-only targets within the radius of
  each other share one. Kinds never cross. TV5's measurement test prints target-to-provider distances per kind and
  same-kind providers within the radius, so the radius leaves `Guess`.

## Operators and replacement (TV26, every preset)

- **Binding:** TV5 registers each placed provider as its stop's `OperatorSlot` (`Live`), template
  `mw.travel.op:<stop key>`.
- **Status:** dead = `Vacant { since }`; disabled by a script = `Absent(disabled)` (quest intent: no replacement,
  arrivals still land); hostile to or fighting the buyer = `Absent(hostile)` for that player; farther than
  `mw.travel.operator_leash_m` from the board point (escorted, scripted away) = `Absent(away)` until back. The
  "Service Refusal" topic stays the services refusal, not a slot status.
- **Template:** a synthesized `NPC_` built at every load from the original's static data: same race, sex, class,
  level and `DODT` list; face and hair from the race's parts seeded by the stop key; the class's default clothing;
  no script or faction; the class name as display name. It keeps class- and race-filtered dialogue and the Travel
  service, and none of the original's id-filtered lines: it is a different person.
- **The original id is never re-spawned:** scripts and dialogue address NPCs by id (`<id>->`, `GetDeadCount`,
  id-filtered infos); a live copy would make them ambiguous and undo the death for quests.
- **Flag `quest_refs`** (the id appears in a script or an id-filtered info with a journal or script result) only
  changes the log line (`travel: replacing quest-relevant operator <id> at <stop>`); it never blocks replacement.
- **Spawn** at the original placement and yaw, out of view, past `travel.operator.spawn_min_dist_m` and
  `travel.operator.replace_delay_s` (framework defaults kept). **Orphan check** at load: target 0 on the three ESMs.

## Rules by id

| Rule id | Classic | Modern | Source |
|---|---|---|---|
| `mw.travel.price` (all kinds) | `mw.services.travel` price, `FollowerPricing::Vanilla` | same, `FollowerPricing::OpenMw` (owner 2026-11-08) | services-adapter "Travel" |
| `mw.travel.time.classic` (strider, boat, gondola, misc exterior) | `int(dist_2d / fTravelTimeMult)` h, as `mw.services.travel` hours | not registered | `travelwindow.cpp` 199-215 |
| route time (`time_rule: None`) | not used | `length / profile speed x time_scale` (travel.md) | owner 2026-11-25 |
| `mw.travel.time.none` (guides) | 0 | 0 | `travelwindow.cpp` 199-215 |
| sleep flag | on (rest as sleeping) | on | `rest(hours, true)` |

- The price is one rule in both presets (the fare is a content mechanic). The adapter rule converts the framework's
  locations back to Morrowind units (factor 128/1.8288 per metre, f64) and calls `travel_price`: the player's
  position to the route's landing pose, 3D for the fare, 2D for the hours. An exterior provider sending to an
  interior `DODT` prices from interior-local coordinates, as OpenMW does (Classic keeps it); Modern route time for an
  interior target uses the distance to that interior's first exterior door (`ExteriorResolver`), else the Classic
  hours.
- The preset picks the `time_rule` TV5 writes (`Some(mw.travel.time.classic)` or `None`); the flow is the same.
  Modern speeds are tunables (`mw.vehicle.<kind>.speed_mps`), set against the Classic figures that TV5's
  measurement test prints beside route time for all 117 entries.

## Followers

- **Query:** `followers_of(player)` in mode `Follow` (followers.md "Travel"), i.e. actors with an `AiFollow` package
  on the player, within `travel.follow.radius_m`; the base mod sets it to **11.43 m** (800 units,
  `actionteleport.cpp` 20-109; Reference).
- **Exclusion:** an actor in combat with the player has its combat stopped and stays (`actionteleport.cpp` 55-70);
  an actor whose script local `stayoutside == 1` stands in an exterior and the target is an interior stays. The fare
  counts the followers left after exclusion (same function, as OpenMW).
- **Placement:** framework ring offset (`travel.follow.ring_m`) in both presets; OpenMW stacks them on the player's
  spot, which only overlaps bodies.
- Effects (Mark, Recall, Interventions) and propylons take no followers (`spelleffects.cpp`, TV-R1 sec. 5).

## Gates (TV10)

Per-caller gate profiles (travel.md "`TravelCost` and gates"), record `mw.travel.gates`:

| Gate | Paid travel | Recall / Interventions | Morrowind source |
|---|---|---|---|
| `TeleportBlocked` | off | on (`sTeleportDisabled`) | world flag `TELE`, set by `DisableTeleporting` / `EnableTeleporting` (`spelleffects.cpp` 476-525) |
| `HostilesAlerted` | Modern on, Classic off (owner 2026-11-26) | off | none in vanilla |
| `OperatorAbsent`, `CannotAfford` (row disabled) | on | n/a | slot status; `travelwindow.cpp` 48-100 |
| `Adapter(mw.travel.no_mark)` / `(no_marker)` | n/a | Recall with no mark / no marker found = refuse | `spelleffects.cpp` 476-525 |

`NoTravel`, `Undiscovered` and `Needs` are off for every caller: Morrowind has no region or cell no-travel flag,
lists every destination (only TV16's opt-in reads discovery) and its guides serve everyone. `TELE` lives in the
script runtime's saved state, read through a source trait; Mark is gated the same way. Whether any vanilla script
calls `DisableTeleporting` is unconfirmed (TV-R1): TV15's real-install test counts the calls. A refused spell is
still spent; Classic shows nothing (vanilla), Modern the refusal text (`sTeleportDisabled` or a new string record).

## Places and tags (TV14, TV15)

- **Places:** every `TempleMarker` reference becomes a place tagged `mw.place.temple_marker`, every `DivineMarker`
  one `mw.place.divine_marker`, pose = the reference's cell, position and yaw. Almsivi Intervention queries the
  first tag, Divine the second; teleport with no followers, 0 ms, then the effect's `VFX_Summon_end` on the caster
  (`spelleffects.cpp` 476-502).
- **Metric:** Classic `CellRing` (OpenMW: smallest square ring of exterior cells, `worldimp.cpp` 3247-3330); the
  framework breaks ties on a ring by Euclid, where OpenMW takes the first along a SW, SE, NE, NW walk (its own comment
  calls it a stand-in): accepted deviation. Modern `Euclid2d`.
- **From an interior:** breadth-first over door links, each hop 1, **checking the visited interiors for a tagged
  place first**, then the first exterior door hands over to the exterior search from the door's destination
  (`worldimp.cpp` 3191-3245). travel.md only walks to the first exterior: the per-cell check is a TV14 request (open
  question 1). `travel.place.max_door_hops`: base mod 16 (OpenMW has no cap; Guess). Isolated interior: none.
- **Mark** sets the player's `RecallAnchor` to the current cell, position and yaw (`player.cpp` 239-250); **Recall**
  runs the executor with `with_followers: false`, 0 ms; player-only, other targets are invalid (`spelleffects.cpp`
  503-525). Recall and the Interventions wait for a nonzero tick (effects-adapter "Deferred effects").
- **Stops get a place** (`Stop.place`): the map lane's place record for the stop's exterior cell name (or the
  interior's), None until MAP0 lands. `discover: Always` for every paid stop.

## Presentation

Same in both presets (`travelwindow.cpp` 177-229): Vehicle routes fade out 1 s and in 1 s (until TV6's real rides);
Teleport routes (guides) add the sound `mysticism cast`; Recall and the Interventions cut, with the effect's own VFX
and sound; propylons keep their script's `Thunder2` and cut. **`GetPCTraveling`** reads the framework's
`Departed..Arrived` window for the buyer: in Stage 1 both fall on one tick, so the flag holds for one update
(`setPlayerTraveling(true)`, reset next update). Effects and propylons do not set it. TV6 checks the vanilla scripts
that read it before widening it to a real ride. Confirmation and refused-row reasons: services-adapter "Options".

## Options (classic vs modern)

`MwTravelOptions` in the game settings, read when TV5 builds routes and when gates are profiled (never inside rules).
Fare options stay in services-adapter "Options".

| Option | Modern | Classic | Source |
|---|---|---|---|
| Trip time | route time (profile speed x time scale) | `mw.travel.time.classic` hours | owner 2026-11-25 |
| Skip counts as sleep | on | on | `rest(hours, true)` |
| Operator replacement | on | on | owner Q4 |
| Paid travel blocked while hostiles alerted | on | off | owner 2026-11-26 |
| Teleport spells blocked while hostiles alerted | off | off | owner 2026-11-26 |
| Intervention metric | `Euclid2d` | `CellRing` | `worldimp.cpp` 3247-3330 |
| Ambushes on rides (Stage 5) | on | off | owner Q1 |
| Travel anywhere discovered (TV16) | off | off | owner 2026-11-25 |

## Tunables (tunables.md, `Guess` unless stated)

| Name | Default | Range | Unit | Source |
|---|---|---|---|---|
| `mw.travel.stop_merge_m` | 40 | 1-200 | m | Guess (TV5 measures) |
| `mw.travel.vehicle_bind_m` | 30 | 1-100 | m | Guess (operator 6-20 m from its vehicle, TV-R1 sec. 3) |
| `mw.travel.operator_leash_m` | 50 | 5-500 | m | Guess |
| `mw.vehicle.strider.speed_mps` / `boat` / `gondola` | 4.0 / 5.0 / 1.5 | 0.5-50 | m/s | Guess |
| `travel.follow.radius_m` (base-mod value) | 11.43 | 0-100 | m | Reference(`actionteleport.cpp`, 800 units) |
| `travel.place.max_door_hops` (base-mod value) | 16 | 1-64 | count | Guess (OpenMW: no cap) |

## Later stages (design pointers)

- **TV3 strider routes:** a `large`-class nav path between the board points of two strider stops, baked by TV2 into
  `PathRef::Baked`; the vehicle is the `a_siltstrider` reference bound at TV5 (9 placed, one per caravaner, NIF
  idle and sound keys only, no walk; TV-R1 sec. 3).
- **TV12 water routes** between boat stops on the water mask with coast clearance; gondolas use Vivec's canals the
  same way.
- **TV13 boats as walk-on decks:** ships are frozen by the owner (2026-11-25); nothing here until unfrozen.
- **TV11 encounters:** tables from each region's sleep-creature levelled list (memory, check), Modern on, Classic
  off (owner Q1).
- Camp and decor striders (`ex_siltstrider_01..05`, 21 statics) and wreck boats are never stops.

## Saves

The adapter adds no section. Networks, stops and templates are rebuilt from records at every load (deterministic,
so a saved replacement actor's base record `mw.travel.op:<stop key>` exists before actors spawn). Slots and recall
anchors are the framework's `sf/travel`; `TELE` is the script runtime's; `GetPCTraveling` is not saved. A save whose
slot names a stop that is gone drops it with a log line (framework rule).

## Tests

Exact, synthetic records (TV5, A4, TV10, TV14, TV15, TV26):

- kind lookup by class, by a nearby strider reference, interior fallback, bare exterior; targets joining a stop,
  arrival-only, two arrival-only targets merged, a target beside another kind's provider (own kind);
- one-way entry = one directed route; unknown `DNAM` cell skipped with one error line; stop keys unchanged after
  shuffling record order and after a mod moves a provider;
- fare parity with `travel_price` on raw units at `n x 4000 +- 1`, both follower pricings; hours parity at
  `n x 16000 +- 1`; guide 10 gold, 0 ms, cast cue; Modern route time from `Estimate`;
- followers: 800-unit edge, hostile cleared and left, `stayoutside` exterior to interior;
- replacement: kill, `Vacant`, delay, out of view, `SpawnRequest` with the template (race, sex, class, `DODT`, no
  script); the original id still counts dead; a disabled operator stays `Absent`, arrivals still land;
- Mark/Recall round trip; Recall with no mark refused; `TELE` off refuses Recall, not paid travel; Intervention by
  exterior ring, interior marker in a visited cell, door hop, isolated interior refused.

Real install (`#[ignore = "needs Morrowind.esm"]`, exact on the three ESMs, TV-R1 sec. 2): 38 providers, 117 entries;
caravaner 9/32, shipmaster 15/43, gondolier 5/16, guide 5/20 (all interior), misc 4/6; Tribunal adds none,
Bloodmoon 4 shipmasters; 0 unknown cells; 0 orphan stops; 9 strider vehicle bindings. It also prints (measurement)
target-to-provider distances, Classic vs Modern hours per entry and `DisableTeleporting` call counts.

## Slices (existing IDs, no new ones)

| ID | Adapter part |
|---|---|
| TV5 | kind table, stops, keys, routes, arrival poses, vehicle bindings, templates synthesized, operator registration, count test |
| A4 | rules registered per preset, followers query and exclusion, cues, `GetPCTraveling`, `MwTravelOptions` (destinations now come from TV5) |
| TV10 | gate profiles `mw.travel.gates`, `TELE` source, refusal texts |
| TV26 | slot status from Morrowind state, spawn of the template in the host, `quest_refs` log, orphan check on game data |
| TV14 | marker places and tags, metric choice, door-hop default |
| TV15 | Mark, Recall, Almsivi and Divine Intervention as `Request` effects on TV14 + TV10 |

## Open questions

Framework requests (decided from the data; **landed in travel.md 2026-11-26**, "Data model" and "Teleport anchors and gates"):

1. `TravelRoute.arrive: Option<Pose>` (per-route landing; Morrowind lands each entry on its own `DODT` pose), and
   `nearest_place` testing tagged places in each visited interior before following its doors (`worldimp.cpp`
   3191-3245).

Answered (were owner questions):

2. **Modern: can you take a paid trip while enemies are hunting you?** **Owner 2026-11-26:** no in Modern (Elden Ring and
   Witcher block it), yes in Classic; Recall and Interventions are never blocked by enemies in either preset (they
   are Morrowind's escape magic).
3. **A replacement operator: same race and sex as the dead one, a new face, named after the class ("Caravaner")?**
   Decided by the industry standard (meta 2026-11-26): yes.
