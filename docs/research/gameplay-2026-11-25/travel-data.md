# TV-R1: how Morrowind travel works, in data and in OpenMW

Research slice for `superfargoth/docs/specs/travel.md` (roadmap `docs/roadmaps/travel.md`). Facts only; behaviour is
described, no OpenMW code is copied. OpenMW citations: github.com/OpenMW/openmw at master commit `7161a35271a9`
(files fetched raw at that commit, 2026-11-25). Counts were **measured by a throwaway script** reading the installed
`Morrowind.esm`, `Tribunal.esm`, `Bloodmoon.esm` (Steam install, Data Files) and `Morrowind.bsa`; the script is not
in the repo (section 2 says how a repo test could print them).

## 1. Travel service in data and in OpenMW

**Data.** A travel provider is an `NPC_` record that carries one or more `DODT` subrecords (24 bytes: position x y z,
rotation x y z, floats), each optionally followed by a `DNAM` (destination cell name). No `DNAM` = the exterior cell at
that position. `components/esm3/transport.cpp` 13-34 (a `DODT` starts an entry, a `DNAM` fills the name of the last
one; a `DNAM` with no `DODT` is skipped with a warning). There is **no travel bit in the AI services flags** (`AIDT`
services are barter classes, Spells, MagicItems, Potions, Training, Spellmaking, Enchanting, Repair); the "Travel"
topic exists exactly when the NPC has a non-empty destination list (`mwclass/npc.cpp` 588-605, `getTransport()
.empty()`; topic text is GMST `sTravel`). Only `NPC_` records carry destinations (no `CREA` has `DODT` in any of the
three plugins), so the strider or ship is scenery and the provider is always a person. The class name (Caravaner,
Shipmaster, Gondolier, Guild Guide) is only a label; behaviour keys on the destination list and on the cell the
provider stands in.

**Price** (`mwgui/travelwindow.cpp` 48-100). Provider in an interior (guild guide): flat GMST `fMagesGuildTravel`
(10 in the data). Provider in an exterior: 3D distance (double) from the **player's** current position to the `DODT`
position, divided by `fTravelMult` (4000 in the data; 0 means divide by 1), truncated to int. Then multiplied by
`1 + followers` (OpenMW's own rule; the code comment says in vanilla the first follower travelled free), then
`max(1, price)`, then the barter offer (`getBarterOffer(provider, price, buying)`). A destination the player cannot
pay is disabled. An unknown interior destination cell is skipped with an error log (lines 140-167).

**Time** (same file 199-215). Exterior providers only: 2D distance (x, y) from the player to the destination divided
by GMST `fTravelTimeMult` (16000 in the data), truncated to int hours; then `rest(hours, sleep=true)` (stats and cells
rest as if sleeping) and `advanceTime(hours)`. Guild guides pass no time. Example: a 32000-unit hop = 2 hours,
about 8 gold before barter.

**Sequence on click** (travelwindow.cpp 177-229): pay gold (it goes into the provider's gold pool); set the
`GetPCTraveling` flag (`setPlayerTraveling(true)`, reset on the next world update); guild guides play sound
`mysticism cast`; fade out 1 s; time passes (exterior only); close the travel GUI mode; `ActionTeleport(cell, pos,
followers = true)`; fade out 0, fade in 1 s. The destination is the raw `DODT` position and rotation: no landing
search, no ride.

**Followers** (`mwworld/actionteleport.cpp` 20-109). With the follower flag on, every actor in
`getActorsFollowing(player)` that passes the filters is teleported **before** the player, to the same cell and the
same position (no offset). Filters: within 800 units of the player (squared compare); an actor whose script has local
`stayoutside == 1` and who stands in an exterior stays behind when the destination is an interior; the fare counter
calls the same function. Teleport and travel pass `includeHostiles = true`, but a teleported non-player actor that is
in combat with the player has its combat stopped and is **not** moved (lines 55-70). Teleported actors land, flag
`teleported`, and lose Water Walking when the target disallows it.

**Arrival.** Cell change at the `DODT` position and rotation, no confirmation; weather reacts to the player teleport
(`worldimp.cpp` 3357).

**GMSTs read from `Morrowind.esm`:** `fTravelMult` 4000.0, `fTravelTimeMult` 16000.0, `fMagesGuildTravel` 10,
`sTravel` "Travel". The repo already ports the formula as `mw.services.travel` (`bevyrrowind/crates/mw_rpg/src/
services.rs`, spec `services-adapter.md` "Travel"); the source above agrees with it.

## 2. Counts in vanilla (Morrowind + Tribunal + Bloodmoon)

Measured, unique `NPC_` ids with at least one `DODT` (Tribunal adds none, Bloodmoon adds 4):

| Provider kind (class) | Providers | Destination entries | Exterior / interior |
|---|---|---|---|
| Caravaner (silt strider) | 9 | 32 | 32 / 0 |
| Shipmaster | 15 (11 + 4 Bloodmoon) | 43 | 43 / 0 |
| Gondolier (Vivec) | 5 | 16 | 16 / 0 |
| Guild Guide (mages guild) | 5 | 20 | 0 / 20 |
| other (Rogue 1, Monk 1, Pauper 1, Guard "todd" test 1) | 4 | 6 | 5 / 1 |
| **Total** | **38** | **117** | |

Per provider 1-4 destinations (strider 2-4, ship 1-4, gondolier 3-4, guide always 4). Bloodmoon shipmasters:
`s'virr` (Khuul), `basks_in_the_sun` (Fort Frostmoth), `veresa alver` (Raven Rock), `wind_in_his_hair`. The odd four
(`rindral dralor`, `vevrana aryon`, `blatta hateria`, `todd`) are quest or test providers. Guild guide destinations are
mages guild interior cells ("Balmora, Guild of Mages", "Ald-ruhn, Guild of Mages", "Vivec, Guild of Mages", "Sadrith
Mora, Wolverine Hall: Mage's Guild"); a caravaner, ship or gondola destination is an exterior position at the other
stop's dock. Example (`darvame hleran`, Seyda Neen): 4 stops at (-21319, -18232, 1178), (32207, -72224, 1006),
(53159, -48229, 984), (-86786, 89452, 1131).

**Reproducing in the repo:** no tool prints these. `bevyrrowind/crates/mw_world/src/adapter_tests.rs` already builds
`TES3Object::Reference` and `TravelDestination` values (the `tes3` crate exposes NPC travel destinations with
translation, rotation, cell), and `mw_rpg/src/services.rs` and `mw_doors_bevy` read them, so a
`#[ignore = "measurement: ..."]` test over the loaded plugins could print the table in a few lines. Not run here.

## 3. Strider and boat placements

- **Town striders:** the provider stands near an **activator** `a_siltstrider` (`ACTI`, mesh `meshes\r\siltstrider.nif`,
  270 KB), placed 9 times (Seyda Neen, Balmora, Ald-ruhn, Gnisis, Maar Gan, Khuul, Suran, Molag Mar, Vivec), exactly
  one per caravaner. It is an animated creature mesh whose NIF carries text keys: `Idle: Start`..`Idle: Stop`,
  `Idle2: Start`/`Stop` with `Idle2: Loop Start`/`Loop Stop`, and `Sound:` keys (`Silt_1EDTA`, `Silt_2VU`,
  `Silt_3VUUB`). `meshes\r\xsiltstrider.nif` + `xsiltstrider.kf` carry the skeleton and the same keys. There is no
  walk, board or ride animation in vanilla.
- **Camp and decor striders:** `ex_siltstrider_01..05` are `STAT` carcass shells (21 placements in Ashlander camps and
  the wastes), not providers. The repo lists silt striders among the 26 "unrepresented" far-proxy models
  (`docs/ledger/statics.md`, 2026-11-19).
- **Ships:** `ex_de_ship` (STAT, `x\ex_de_ship.nif`, 379 KB, no animation keys), 6 placements (Sadrith Mora, Vos, Dagon
  Fel, Ebonheart, Ald Velothi, Fort Frostmoth in Bloodmoon via `ex_co_ship_baro` ACTI); also `ex_longboat`,
  `ex_longboat01/02`, `ex_longboatwrk` (wrecks), `a_ex_de_rowboat` (ACTI, Khuul), Bloodmoon `a_ex_colonyboat` (ACTI).
  Docked static meshes: nothing sails in vanilla.
- **Gondolas:** `ex_gondola_01` (ACTI, 5 placements, one per gondolier in the Vivec canton cells), no animation keys.
- **Operator distance** (measured: provider reference vs nearest vehicle reference in the same cell, 3D, game units,
  about 70 units = 1 m): strider 804-1414 (median about 1100), ship 416-1399 (median about 700), gondola 1276-1600. So
  the operator stands **6-20 m from the vehicle**, with no seat, no boarding point and no path between them. Three
  providers (Daynas Darys, Baleni Salavel in Hla Oad, S'virr in Khuul) have no vehicle reference in their own cell.

## 4. Propylons and magic teleports

**Propylon chambers (10 Dunmer strongholds; data only).** Each propylon is an `ACTI active_port_<x>` (mesh
`i\active_port_<X>.NIF`, name "<Stronghold> Propylon") with script `Warp_<X>`: on `OnActivate` (ignored in menu mode)
it tests `Player->GetItemCount "Index_<X>"`; with the index it plays `Thunder2` and calls `Player->PositionCell` into
the cell "<Stronghold>, Propylon Chamber" at a hard-coded position; without it, message box "You do not have the
Index for this Propylon." The key is `index_<x>` (`MISC`, mesh `m\misc_portal_shard.NIF`, "<Stronghold> Propylon
Index"). Stems: andra, beran, falas, falen, hlor, indo, maran, roth, telas, valen. **Topology measured from cell
refs:** each chamber holds two pylons that warp to its two ring neighbours (the Berandas chamber has the Andasreth and
Falasmaryon pylons), so 10 chambers form a ring with 20 directed links. Index placements: andra Hlormaren Dome, beran
Gnisis Arvs-Drelen, falas Maar Gan Shrine, falen Maelkashishi Shrine (Forgotten Galleries), hlor Caldera (Irgola
Pawnbroker), indo Tel Fyr Hall of Fyr, maran Vivec St. Olms Temple, telas Telasero Lower Level, valen Urshilaku Camp
Wise Woman's Yurt; no placed `index_roth` was found (unconfirmed: script-added or quest reward). Dialogue topic
`MS_propylon` goes with them. The jump is a script `PositionCell`: no fare, no time, no followers. Scripts here are
MWScript, so the port rides the script runtime; a data-driven `TravelNetwork` stop with an item gate would reproduce
it too.

**Mark and Recall** (`mwmechanics/spelleffects.cpp` 503-525): both player-only (other targets get an invalid flag)
and gated by `world->isTeleportingEnabled()` (flag saved as `TELE`, script-togglable; message `sTeleportDisabled`
when the player casts). Mark stores cell + position (`mwworld/player.cpp` 239-250); the save writes `MARK` + cell id
(`components/esm3/player.cpp` 18-20, 120-121). Recall teleports to that cell and position with **no followers**
(`ActionTeleport(.., false)`) and does nothing if no mark exists.

**Almsivi and Divine Intervention** (spelleffects.cpp 476-502): player-only, same teleport-enabled gate; resolve via
`World::teleportToClosestMarker(player, "templemarker" or "divinemarker")` (worldimp.cpp 3338-3352), teleport without
followers to the marker's own cell and position, then play `VFX_Summon_end` on the caster. Markers are `STAT`s
`TempleMarker` (`Marker_Temple.nif`) and `DivineMarker` (`Marker_Divine.nif`) placed in the vanilla world.
`getClosestMarker` (worldimp.cpp 3191-3245): from an **exterior**, it uses the player's last known exterior position;
from an **interior** it walks cells breadth-first through teleport doors (each hop = distance 1), first looking for a
marker in the cell itself; the first door that leads to an exterior hands the search to the exterior resolver from the
**door's destination position**; an isolated interior finds nothing ("no closest marker", no teleport). The exterior
resolver (3247-3330) takes the markers on the smallest square ring of cells around the start cell (a marker in the
start cell wins at once) and breaks ties along a SW, SE, NE, NW walk (the source comment calls it a stand-in for a
"Todd spiral"). So distance is cell-ring distance, not units. `findInteriorPositionInWorldSpace` (3141-3189) is a
separate breadth-first helper returning the first exterior door destination; it is not the marker resolver.

## 5. Co-op relevant facts

- Followers within 800 units teleport with the player on Travel and on door-style teleports with the follower flag;
  Mark/Recall, Almsivi/Divine and propylon scripts take none (`actionteleport.cpp`, `spelleffects.cpp`).
  Fare multiplies with followers (OpenMW rule `1 + followers`).
- Time passes for the whole world (`rest` + `advanceTime`), so co-op needs the planned party-rest vote (services spec).
  No other NPC-state handling happens on arrival.
- **Save impact:** travel stores only position, gold (player and the provider's pool), world time and rested stats. The
  pieces that matter are the player's `MARK` + marked cell, the world `TELE` flag, and the transient `GetPCTraveling`
  flag (one update, not saved). Destination lists are static `DODT` data, never saved. Followers are saved as
  ordinary actors at their new position.

## Facts our spec needs

- Provider = `NPC_` with a non-empty `DODT` list, no services flag; destination = position + rotation + optional cell
  name, empty = exterior (`transport.cpp` 13-34; `npc.cpp` 588).
- 38 providers: 9 caravaner (32 dests), 15 shipmaster (43), 5 gondolier (16), 5 guild guide (20, flat fee, interior), 4
  odd (6); 117 destination entries; Tribunal adds none (measured, three ESMs).
- Fare: exterior = int(dist3d / 4000) * (1 + followers), min 1, then barter offer; guide = flat 10 (`travelwindow.cpp`
  57-82; GMSTs from `Morrowind.esm`).
- Time: exterior only, int(dist2d / 16000) hours via rest(sleep) + advanceTime (`travelwindow.cpp` 199-215).
- Fade out 1 s, teleport to the raw `DODT` spot, fade in 1 s; guides play `mysticism cast`; `GetPCTraveling` set for one
  update (`travelwindow.cpp` 177-229).
- Followers within 800 units go along, hostile ones are cleared and left, `stayoutside` locals respected
  (`actionteleport.cpp` 20-109).
- Strider = `a_siltstrider` ACTI with NIF text keys Idle/Idle2/Loop/Sound (9 placed, one per caravaner); operators
  stand 6-20 m away; ships and gondolas are static or key-less meshes; no vanilla vehicle moves (measured, ESM + BSA).
- Propylons are scripts (`Warp_<X>` on `active_port_<X>`, `Index_<X>` check, `PositionCell`), 10 chambers in a ring,
  2 pylons each, no fare, no followers (measured, `Morrowind.esm`).
- Mark/Recall: player-only, one mark (`MARK` + cell), no followers, global teleport-enabled gate (`spelleffects.cpp`
  503-525; `player.cpp` 239-250).
- Divine/Almsivi: nearest `DivineMarker`/`TempleMarker` by cell-ring distance from the exterior position; from an
  interior by door hops to the first exterior; no followers (`worldimp.cpp` 3191-3352; `spelleffects.cpp` 476-502).
- Unconfirmed: magic effect numeric ids (names confirmed in source); whether `index_roth` is placed or script-added;
  meaning of the `Warp_*` rotation argument as yaw; whether any vanilla script calls `DisableTeleporting`.
