# Seamless interiors: prior art and a design for M9 (research, 2026-11-08)

Owner ask: walk between interiors and exteriors with no loading screen, fade or teleport; NPCs path and move across
interior/exterior and interior/interior boundaries; weather and lighting follow (rain stops at the doorway, sky light
fades, interior ambient/fog/exposure blend, audio). Morrowind interiors are separate cells with arbitrary coordinates
joined by door teleports, so this covers placement, overlaps, doorways, navigation links, saves, mods and co-op.
Research only: our docs and code, two open-source engines read at source level, shipped engines from docs. Items
marked **(verify)** come from memory or a page not re-read.

## 1. Summary

- **We already chose the right data model, and both new prior-art engines independently chose it too.** Interiors
  stay their own worldspaces; a two-way door link is a rigid portal (`b.frame · R_y(π) · a.frame⁻¹`); crossing moves
  the traveller's `Location` from one space to the other. The OpenMW seamless fork and SPUD (Skyrim) both keep
  interiors as separate spaces with their own coordinates, join them by a rigid turn-about-up transform at the door
  plane, and keep each cell's logical identity, so scripts and saves don't notice. Nobody places Morrowind/Skyrim
  interiors in world space: they are bigger on the inside. Keep ours; don't build "pockets" (Godotwind's pocket
  manager was its largest bug source, `bevyrrowind/docs/research/godotwind-audit-2026-10-26-b.md`).
- **What is missing is not rendering, it's the single-active-worldspace assumption.** `ActiveWorldspace` is one global
  resource that the look (`mw_doors_bevy::worldspace_look`), physics stepping (`sf_physics::PhysicsWorlds`, "only the
  active one steps"), actor processing (DECISIONS 2026-10-12 walkers2), audio (`ListenerOutdoors(bool)`), GI volume
  lifetime and precipitation (`Precip.enabled` off indoors) all key off. Seamless needs a **simulated set** of
  worldspaces (the anchor's own plus the far side of every open portal near it) and **per-viewer** environment. Our
  co-op spec (`net.md` "Multi-region host simulation") already demands the same ("the host never swaps a global
  active worldspace"); doing it once serves both.
- **Three mechanisms carry most of the value**, all cheap: (1) the door's swing is the new loading screen (stream and
  build behind it; the doorway is passable only once the far side is ready); (2) a symmetric transition weight across
  the door plane drives every blend (ambient, fog, exposure bias, audio exposure, reverb, wind draft), so nothing
  jumps at the crossing; (3) a doorway "passage" filter in the character/physics queries (OpenMW fork's
  `doorwayPassable`) so the wall each door is set into doesn't stop you.
- **Recommended plan:** 16 slices (section 6), framework first, M9 portal view resurrected from the 2026-10-12 spike
  as `sf_portal_bevy`, AI crossing on our pathgrid scaffold before `sf_ai` exists, off-screen AI parked on `sf_ai`.
  Five existing decisions should change (section 5).

## 2. What Scrib already has

| Area | What exists | Where |
|---|---|---|
| Data model | `Worldspace` (partition, `environment: Option<RecordRef>` "blended across links in M8"), `DoorLink { a, b, two_way }`, `LinkEnd { worldspace, frame, arrival, door, extent }`, `DoorLink::cross`, `cross_direction` (rigid, scale ignored) | `superfargoth/crates/sf_world/src/records.rs` (LinkEnd 187, cross 228); `specs/streaming.md` "Data model" |
| Every interior indexed | 1,328 interiors baked as `Partition::Single` worldspaces; dangling link ends fail the bake (vanilla 0) | streaming.md "Every worldspace is indexed"; DECISIONS 2026-10-12 (interiors) |
| Runtime spaces | One `big_space` root + one `RenderLayers` layer per loaded worldspace; `OriginProxy` holds the floating origin of roots no camera is in; layer 0 shared | `superfargoth/crates/sf_world_bevy/src/worldspace.rs` |
| Door prefetch | Anchor within 24 m of a link end makes the far worldspace Prefetched around `arrival`; the prefetch driver already loads R0 at `Level::Active` (colliders built), it's only not simulated or drawn | `sf_world/src/scheduler.rs` (~767, `Driver::Prefetch` ~949) |
| Return cache | Residency ghost of the worldspace just left (5 s linger, kept while near a link back) | streaming.md "Residency cache" |
| Fade scaffold | `FadeTransition` (0.3 s, burst hold until `arrival_ready` + textures, 10 s cap), `TravelArrived` | `sf_world_bevy/src/fade.rs`; removal path is M8/M9 portals |
| Door use + look | Use key picks a door in front, `StartTravel`; `worldspace_look` swaps sky/terrain/cloud cameras, ambient, clear colour, exposure, `ListenerOutdoors` on the single player camera | `bevyrrowind/crates/mw_doors_bevy/src/lib.rs` (activate 211, worldspace_look 309) |
| Door links (adapter) | DODT/DNAM pairing within 8 m: 1,791 two-way, 37 one-way, 0 dangling | `bevyrrowind/docs/specs/world-adapter.md` "Door links"; `mw_world::worldspaces` |
| Portal frames (adapter) | `mw_world::portal`: sill-origin frames from door bounds, normal sign from the landing marker; 1,361 portal-ready, 251 need an extent fix, 216 fade-only (one-way, trapdoors); hinge fraction known; **not yet wired into the bake** ("Next step") | `bevyrrowind/crates/mw_world/src/portal.rs`, `examples/portal_report.rs`; world-adapter.md "Portal frames: feasibility" |
| Portal spike (deleted, in git) | RTT portal camera per open portal, oblique `near_clip_plane`, opening quad, position-only `Gate` crossing (symmetric, no cooldown), viewer-owned exposure (`AutoExposure` + compensation curve), +1.5 ms at 1440p looking in; open items: motion vectors in the opening, void band behind the far plane, ~1.3 EV step, env map rebuilt from scratch, looking-out cost unmeasured, Solari TLAS ignores layers | `superfargoth/docs/specs/portals.md`, `specs/archive/portals-history.md`; ROADMAP 0.5; DECISIONS 2026-10-12 (portal spike, portal2) |
| M9 design (written) | `sf_portal_bevy` view pool (1, up to 2), per-view look hooks, environment blending (ambient/fog/reverb volumes), keep fade for fade-only links | portals.md "M8 design"; WISHLIST §6; ARCHITECTURE M9 "Walk into the Census Office without a load or fade" |
| Audio | Ambience layers `Space::{Outdoor, Indoor, Any}` weighted by an eased outdoor exposure (0.5 s); weather bus scaled; host sets `ListenerOutdoors(bool)`; "sound through the opening" is a logged GAP | `superfargoth/crates/sf_audio/src/ambience.rs`; `sf_audio_bevy/src/lib.rs` (64, `ease_outdoor` 566); audio.md "Snapshots"; portals.md comparison |
| Rain under roofs | Precipitation occlusion map: one ortho depth view along the drops' travel kills drops under roofs/trees/lee of walls; hosts switch `Precip.enabled` off indoors | `sf_precip_bevy/src/occlusion.rs`, `lib.rs`; weather/state.md "Interiors have no weather" |
| Sky visibility | Surface-weather shelter + canopy maps (top-down, 128 m) for wetness/snow; W8 plans character wetness by roof ray ("walk from rain into a doorway") | `specs/surface-weather.md` |
| Fog / environment | Interior fog stays the Bevy fog "until the environment volumes of M9"; `CellEnvironment` is an adapter kind awaiting a framework environment kind | weather/fog.md 36, 99; world-adapter.md open question 3 |
| GI | Interior probe grid per `Single` worldspace, rebuilt (despawned) on `ActiveWorldspace` change; meshes traced only for the render-path cameras' layers | `specs/gi.md` "Probe volumes"; `bevyrrowind/crates/mw_exterior_bevy/src/gi_volume.rs` |
| Physics | `PhysicsWorlds<K>`: one world per loaded worldspace, only the active steps; walker statics per worldspace (coordinates overlap) | `sf_physics/src/world.rs` 977; `mw_walkers_bevy/src/walkers.rs` 93-112 |
| AI | No `sf_ai` spec; `mw_ai` has PGRD pathgrids per `CellKey::{Exterior, Interior}` with A*, wander, a named path-walk scaffold; actors simulate only in the active worldspace inside R0 (`Dormant` elsewhere); off-screen tier waits for `sf_ai` (M10) | `bevyrrowind/crates/mw_ai/src/pathgrid.rs`; streaming.md "Decided"; locomotion.md "Processing range" |
| Co-op | Per-player gameplay anchors, a physics region per interior a player is in, interest set "same worldspace, ~R1", door crossing per player, doors host-authoritative | `specs/net.md` 133, 159, 214-231 |
| Status | ROADMAP `[p] Seamless portals (M9)`; owner 2026-10-12: portals an optional setting, classic fade default | ROADMAP 144; portals.md status |

**Gaps in one line each:** portal frames not baked; no framework environment record or blend; one global active
worldspace; no simulated set (AI/physics behind an open door); no door swing for load doors; no passage through the
wall behind a door; boolean audio exposure; far-side sounds not routed; no navigation link across worldspaces; no
cross-space distance/LOS query; GI volume and precip occlusion assume one worldspace per frame; no co-op interest
through portals.

## 3. Prior art

### 3.1 OpenMW seamless fork (`duron27/openmw-seamless`, branch `seamless-doors`)

One commit, `48392fca` "Seamless cells and seamless doors" (2026-11-07), on top of OpenMW master `5b75e3eb`: 36 files,
~3,000 added lines, GPL-3.0 (portable here with file + commit citation). Brand new, no issues or discussion found; read
as an idea source, not a tested design. Settings (all default off): `seamless cells` (no loading screen or fade on a
cell change in a running game), `seamless doors`, `load cells over frames` (objects within `cell load near radius` =
1024 units at once, the rest nearest-first at `cell load budget` = 3 ms/frame).

| Concern | Mechanism (file in the diff) |
|---|---|
| Placement | No world placement. `SeamlessDoors::open` finds the door back (a teleport door in the destination cells whose destination is nearest this door, within 1024 units), derives each door's front from where the partner's destination marker lies, and builds `mToOther = T(-floorCentre) · Rz(turn) · T(otherFloorCentre)`: floors meet, turn about up only (`mwworld/seamlessdoors.cpp`). Same as our `DoorLink::cross` with sill frames. |
| Door | Activating a teleport door opens it instead of teleporting (`mwclass/door.cpp`). Procedural swing in 1.1 s about a hinge from the model bounds; the interior's door swings into the interior; from outside the exterior door is hidden and a copy of it swings on the far side; the leaf's back shows the partner door's model. Activating again shuts it; left open it swings shut when the player is 1000 units away unless someone behind it is pursuing. |
| Rendering | **Stencil portal in the main pass**, no second camera (`components/sceneutil/doorportal.cpp`): bin 3 writes stencil where the door shape (or, with the camera inside the doorway, a 56-unit-deep box, so the near plane never cuts it) is visible; bin 4 clears depth there and paints a sky gradient + cloud texture (black inside); bin 5 draws the far side transformed by `otherToWorld`, stencil-tested, **shadows disabled, lit by a fixed ambient/sun uniform**; bin 7 clears the stencil. Exterior seen out to 7000 units (~100 m), fogged to the horizon colour. |
| Streaming | "Secondary cells" (`mwworld/scene.cpp/.hpp`): the far side's cells are loaded into the scene beside the current ones (physics, actors, scripts) but drawn under the portal group, without navigator data, water or ambient. One cell per frame while the door swings; what is seen through the door is built at 4 ms/frame; the reverse view is built after. Crossing calls a real `changeToCell` with `keepOnChange`: current and secondary cells swap roles, nothing reloads. A stand-in copy of what was seen covers objects still being inserted (30 s cap). |
| Crossing | Player: position past the door's mid-plane inside the opening, door > 50% open → same pose and heading on the other side; followers teleported to the authored marker. Other actors: `crossActors` moves any actor past a doorway's middle to the other side's cell (`world.moveObject`). |
| Physics | Not separated: secondary cells sit in the one Bullet world **at their own coordinates**, so an interior whose arbitrary coordinates overlap loaded exterior cells collides with them (latent bug). Passage: `MWPhysics::doorwayPassable` ignores contacts inside an oriented doorway box (walls, door, low lintel) but keeps floors (`mwphysics/doorway.cpp`, hooked into `actorconvexcallback.cpp` and `movementsolver.cpp`). Projectiles: segment-vs-doorway test, position and heading mapped, node reparented (`passDoorway`). |
| AI | No navmesh behind the door (pathgrid or straight line, `pathfinding.cpp`). `wayThrough`: target on the other side → walk to 80 units in front of the doorway, then straight through, opening the door. Patched into `aicombat`, `aipursue`, `aifollow`, `aipackage` (inactive-cell check skipped behind the door). Awareness, crime witnesses and alarms use `positionFor` (the other side's position mapped through the door) and `heard` (path length to the doorway plus from it ≤ range) in `actors.cpp`, `mechanicsmanagerimp.cpp`. |
| Weather / light | Weather is computed while indoors too and fed to the portal as fog/sky/ambient/sun colours and cloud textures (`mwworld/weather.cpp`). No rain through the door, no light through the door into the room. |
| Audio | Sound positions of far-side sources mapped through the door (`mwsound/soundmanagerimp.cpp`); no occlusion, diffraction or far-side ambience bed. |
| Saves | Unchanged: crossing is a real cell change, so the player is always in exactly one cell. |

Limitations, from the code: one open door at a time (a singleton); exterior to exterior unsupported; far side flat-lit
and shadowless; overlapping-coordinate physics; behind-the-door AI without navmesh; only a few AI packages made
portal-aware (every distance check had to be patched by hand: `positionFor` is sprinkled through mechanics).

**Lessons for us:** the swing as latency mask; build the reverse view before the crossing; swap roles instead of
reloading (our residency ghost already does it); the passage filter; path-length hearing; and the warning that
portal-awareness sprinkled through gameplay code doesn't scale: make it one framework query.

### 3.2 SPUD (`chasmlol/spud`, Skyrim SE)

New C++/Vulkan engine (MIT, "much of this code was written with AI assistance"), created 2026-11-06, read at
`8c99042` (2026-11-07). Its world model is exactly ours: "a set of spaces, each with its own coordinate frame,
connected by portals" (`docs/ARCHITECTURE.md` "World model: spaces and portals").

- **Placement:** child worldspaces that share the parent's coordinates (Skyrim cities) are merged at import; interiors
  stay spaces. Each `XTEL` door pair is a portal: `Doorway { centre, normal, right, half extents }` from the door
  base's bounds (thin horizontal axis = normal, signed by the partner's teleport marker), `PortalTransform` = rigid,
  about Z, floors meet (`src/engine/doors.hpp`). Logical identity kept (region tags, cells) so conditions keep working.
- **Streaming:** every interior whose door is within 40 m is read on a worker; exteriors stay resident (terrain is
  expensive to rebuild); interiors retire after no frame in flight uses them (`app_doors.cpp` `retire_spaces`).
- **Rendering:** the far space is a **whole frame of its own** (shadows, fog, water, particles, image space) from the
  camera carried through the door, culled to a frustum narrowed to the doorway and scissored to its screen rectangle,
  clipped at the far doorway plane with an oblique near plane; the main frame then starts from that scene inside the
  doorway so eye adaptation and bloom see it; the doorway takes the far frame's image by depth coverage, keeping MSAA
  edges (`src/render/renderer_portal.cpp`, `shaders/portal_stamp.frag`). One portal view, no recursion. No TAA in the
  engine at all, so it never had to solve the temporal problem we have.
- **Crossing:** camera step crosses the plane inside the opening → swap the current space **and its physics world**
  with the far one, carry camera, player and script executor through the transform, open the door behind
  (`App::cross_door`).
- **Light through the door:** a "door light" at the far doorway: the near side's directional ambient along the
  normal + 0.25 × sun + its point lights reaching the doorway, ×2 gain, 7 m radius, scaled by openness
  (`door_light`). Interior lighting templates blend between rooms (`Space::room_blend`); "Show Sky" interiors pick a
  weather from their region once per visit so the view in matches what you walk into.
- **Deferred:** NPCs and dynamic bodies crossing, navmesh links through doors, sound through doorways.

**Lessons:** separate physics world per space swapped at the crossing (what `PhysicsWorlds` already is); a fake
light at the doorway is enough for the "pool of daylight" read; one portal view without recursion is an accepted cut.

### 3.3 Shipped engines (concise)

| Engine | Interiors | Weather/light at the boundary | Navigation |
|---|---|---|---|
| Bethesda (Morrowind to Starfield) | separate cells, load doors + teleport markers; Skyrim/FO4 add room bounds + portals inside interiors for culling, and lights flagged "portal-strict" reach only rooms joined by portals (SPUD `docs/formats/osw.md`, from Skyrim data) | "Show Sky"/"Use Sky Lighting" interior flags, lighting templates per cell | navmesh door triangles linked to the teleport door at Finalize; NPCs path through load doors ([CK World Hookup](https://ck.uesp.net/wiki/Bethesda_Tutorial_World_Hookup), [Nexus navmesh errors](https://www.nexusmods.com/skyrimspecialedition/articles/6854)); AI process levels simulate unloaded cells coarsely **(verify)** |
| RAGE (GTA V, RDR2) | interiors are MLOs inside the world: rooms + portals in the YTYP archetype ([FiveMX MLO guide](https://fivemx.com/blog/how-to-create-fivem-mlos)); per-room timecycle modifiers and interior audio, portal flags gate exterior visibility **(verify)** | per-room timecycle (ambient, fog, exposure) blends at portals **(verify)** | one navmesh incl. interiors **(verify)** |
| CryEngine (Kingdom Come: Deliverance) | interiors in the level; VisAreas + portals define indoor areas for culling and lighting; portals must protrude both sides ([CE3 VisArea and Portals](https://cryengine.com/docs/static/engines/cryengine-3/categories/1114113/pages/1048727), [CE5](https://cryengine.com/docs/static/engines/cryengine-5/categories/23756816/pages/26215443)) | "IgnoreSkyColor": time-of-day sky colour not applied indoors; environment probes since 3.6 | level navmesh |
| REDengine (Witcher 3, Cyberpunk) | interiors inside the exterior partition; Umbra visibility ([GDC 2014](https://www.gdcvault.com/play/1020686/Solving-Visibility-and-Streaming-in)) | environment-area volumes with blend priorities **(verify)** | one navmesh |
| Unreal (World Partition) | level instances / data layers in the world partition (streaming.md cites it) | Post Process Volume blend radius; Audio Gameplay Volumes (reverb, filter/occlusion, submix send by listener inside/outside) ([AGV quick start](https://dev.epicgames.com/documentation/unreal-engine/audio-gameplay-volumes-quick-start?lang=en-US), [Audio Volume](https://dev.epicgames.com/documentation/unreal-engine/audio-volume-actor-in-unreal-engine)) | Nav Link Proxy / Recast off-mesh links **(verify)** |
| Middleware audio | Wwise Spatial Audio Rooms and Portals: a far room's sounds reach the listener from the portal, with diffraction and the room's reverb (portals.md cites it, **verify**) | | |
| Rain/sky occlusion | Top-down depth/height maps around the camera (AC3/AC4 world AO and rain, [Wronski AC4 deck](https://api.sambuz.com/doc/assassin-s-creed-4-black-flag-ppt-presentation-541266); CoH2 "snow shadow" map, [Barrero](https://fr2.slideshare.net/proyZ/daniel-barrero-coh2renderingtech)); ours is the same technique | | |

Takeaway: engines whose interiors fit their shells put them in the world and blend by volumes (RAGE, Cry, RED, UE);
engines with bigger-inside interiors keep separate spaces and doors (Bethesda). Only the two hobby engines above join
separate spaces seamlessly, and both with rigid portals. Every shipped engine blends environment by **volumes with a
blend distance**, never by a switch, and every one links navigation across the boundary in data (door triangles,
off-mesh links) rather than in AI code.

## 4. Recommended architecture

Principle: **a portal is a link both sides can see, hear, walk and path through; a worldspace is a coordinate frame,
not a mode.** Nothing in the framework names a cell, door record or Morrowind unit; the adapter supplies frames,
extents, hinges, environments and door rules.

### 4.1 Placement, overlaps, identity

- **Keep separate worldspaces joined by rigid portals.** Overlaps are a non-issue by construction: each worldspace has
  its own root, origin, render layer, physics world, nav graph and GI volume. Nothing may share them except layer 0.
- **Add one framework hook for content whose interiors fit**: `Worldspace.embed: Option<(WorldspaceId, WorldTransform)>`
  (an interior that lives inside a parent's frame, UE data-layer style: same root and physics world, no portal view,
  the link degenerates to a plain doorway). Morrowind never sets it; other games or true-scale interior mods may.
- **Identity:** at any tick every spatial entity is in exactly one worldspace (`InWorldspace` component, section
  4.2). Crossing is instantaneous (position-only `Gate`), so scripts, saves and quests see a normal cell change.

### 4.2 Membership and the simulated set (framework, `sf_world` + `sf_world_bevy`)

- `InWorldspace(WorldspaceId)` on actors, items, projectiles, emitters, cameras, anchors. `ViewerWorldspace` per
  camera replaces the global `ActiveWorldspace`; keep `ActiveWorldspace` derived from the local player during
  migration (consumers listed in slice SI3).
- Scheduler: a new **portal driver**. When a gameplay anchor is within `portal_reach` (default 12 m, tuned) of a
  portal-ready link end, or the link is open, the far side gets an anchor at the mapped position (`cross` of the
  anchor's pose) driving R0 at `Level::Active` (prefetch already does this at `arrival`; moving it to the mapped pose
  keeps the far R0 centred where the player will stand).
- **Simulated set** = every anchor's worldspace ∪ the far worldspace of every open portal near an anchor, each with
  the R0 region around the anchor or mapped anchor. AI, physics, scripts tick there; everything else is dormant
  (processing range rule unchanged inside each worldspace). Co-op uses the same set per player.
- **One framework query for cross-space geometry** (Bevy-free, `sf_world::portal_query`): `map(Location, to_ws)`,
  `path_distance(a, b) -> Option<(f32, LinkId)>` (via the open doorway's centre), `line_of_sight` (ray in A to the
  opening, ray in B from the mapped point). AI, audio, crime witnesses, net interest and quest markers call it,
  instead of OpenMW's sprinkled `positionFor`.

### 4.3 Doorways, door swing and crossing (`sf_portal` core + `sf_portal_bevy`)

- `PortalState { openness: 0..1, opening: bool, passable: bool }` per link, persistent in the placement overlay (state
  blob of the door placement) so a door left open stays open after a save.
- **Swing is the loading screen:** activation starts the swing (adapter-defined duration, ~1 s); `passable` only when
  `openness > 0.5` **and** `Scheduler::arrival_ready` for the mapped pose (+ collision ready, Godotwind audit #9). On a
  miss the door stays ajar and the player bumps the leaf; never a stall, never a fade.
- **Crossing:** position-only `Gate` (centre past the mid-plane, inside the extent; spike rule kept) for every body,
  not only the player: pose and velocity through `DoorLink::cross`/`cross_direction`, `InWorldspace` swapped,
  `TravelArrived { seamless: true }` (spike history noted the velocity need). Camera history reset only for the portal
  view, not the main view (section 4.4).
- **Passage filter:** an oriented box around the opening (extent × ±0.7 m) where character probes and Rapier contacts
  ignore everything but up-facing floor (OpenMW `doorwayPassable` rule), on both sides, while passable. Rapier: a
  query predicate for our controller's casts and `PhysicsHooks::filter_contact_pair` for dynamic bodies
  **(verify the hook names on rapier3d 0.36)**.
- **Projectiles:** segment-vs-opening test per tick, carried through like bodies (OpenMW `passDoorway`).

### 4.4 Rendering through the doorway

- Keep the decided render-to-texture portal view (DECISIONS 2026-10-12) rather than OpenMW's stencil pass: Bevy has
  no stencil sub-view, and RTT gives the far side its own shadows, fog, sky and exposure, which OpenMW's flat-lit
  copy lacks and SPUD shows is worth it. Pool of 1 view (High: 2), viewport clamped to the opening's screen rectangle,
  rendered only while the opening is on screen and above a screen-area threshold.
- **Fix the temporal problem properly:** the opening's material prepass writes **motion vectors and depth sampled
  from the portal camera's own prepass**, so DLSS/TAA reproject the far side by its own motion (custom `Material`
  prepass, no Bevy patch). This replaces "history reset at the crossing" for the main view.
- **Thick portal:** with the camera within 0.6 m of the plane, the opening becomes a box extruded into the far side
  (OpenMW's 56-unit box, Valve's thick portal) so the near plane never shows the void band.
- Lights, ambient, env maps and fog already confine per view through layers and camera components (DECISIONS
  2026-10-12); light probes don't (upstream). Solari's TLAS ignoring layers stays a known Ultra-tier gap.
- **Light through the door** (both sides): one `PortalLight` per open portal, a spot light just behind the opening
  aimed into the space, colour = the far side's radiance at the door (SPUD's `door_light` formula: ambient along the
  normal + k × sun + far lamps reaching the door), × openness. In the interior's GI tracing scene, an emissive quad at
  the opening with the exterior's sky radiance (a "light portal", as offline renderers do), visible to the GI rays
  only: the dynamic probes then light the doorway region by themselves. Keep both worldspaces' GI volumes alive while
  the portal is open.

### 4.5 Environment blending (framework record + per-viewer blend)

- New framework kind `sf.environment`: ambient colour/intensity, fog (colour, density, height), exposure
  compensation, sky visible (bool), weather (`Inherit` | `None`), wind factor, reverb preset. `Worldspace.environment`
  points at one; the adapter maps `CellEnvironment` onto it (world-adapter open question 3). Optional local
  `EnvironmentVolume` (box, blend distance, priority) for porches, caves, canton interiors: the UE/Unity/Cry pattern.
- **Symmetric transition weight.** For a viewer at signed distance `d` from an open portal's plane on its own side,
  inside the opening's projected footprint, `w_far(d) = 0.5 · openness · s(1 − d / D)` with `s` a smoothstep and `D`
  the transition depth (~3 m). At the plane `w_far = 0.5`; after the crossing the same formula, roles swapped, gives
  0.5 again, so every blended quantity is continuous. Off-axis falloff by the opening's solid angle.
- Per viewer (camera and listener separately): blended ambient, fog, exposure bias (on top of the viewer's
  `AutoExposure`), reverb, outdoor exposure. Replaces `worldspace_look`'s switch and `ListenerOutdoors(bool)`.

### 4.6 Weather, precipitation, wind, sky light

- Interiors have no weather (weather/state.md); "behaves like exterior" interiors use `weather: Inherit`.
- **Rain stops at the doorway for free on the exterior side**: the existing precipitation occlusion map handles
  porches and lintels. Inside, there is no rain in the interior worldspace; looking out, rain draws in the portal
  view because drop boxes follow whichever camera renders them.
- **One exterior viewer rule:** the occlusion map, shelter map and cloud/atmosphere cameras anchor on the camera that
  sees the exterior: the main camera when outside, the portal camera when looking out. One map, no second copy.
- **Wind draft:** inside, within D of an open portal, wind = exterior wind projected on the portal normal × openness
  × falloff (smoke, flames, cloth near the door move). Zero elsewhere.
- **Sky light / audio exposure:** outdoors, exposure = open-sky fraction from the shelter map at the viewer (so a
  porch is already partly "indoors"); indoors, the portal solid-angle term of 4.5. One scalar feeds audio, ambience
  layers and character wetness (W8).

### 4.7 Audio

- `ListenerExposure(f32)` from 4.6 instead of the boolean; layers and the weather bus already take a 0..1 exposure.
- **Far-side sources:** a voice whose `InWorldspace` differs from the listener's and is reachable through an open
  portal within its range plays from a virtual position on the listener's side: the opening's point nearest the line
  to the mapped source, pushed back to the path length; gain by path length, a low-pass by the angle off the portal
  normal (cheap diffraction), × openness. That is Wwise rooms-and-portals behaviour in one function.
- **Portal ambience emitter** (the GAP in portals.md): one emitter at each open opening playing the far side's beds
  (rain, wind, region ambience or the tavern) at the exposure weight.

### 4.8 AI and navigation

- **Per-worldspace navigation** (pathgrid now via `mw_ai`, navmesh later) plus **links in data**: each `LinkEnd` gets
  an approach point (the nearest nav node in front of the door, from the adapter); the framework adds an off-mesh
  edge "traverse link" between the two approach points (Recast/Detour off-mesh connection, UE Nav Link, Bethesda door
  triangles).
- **Two-level planning:** a `WorldGraph` (Bevy-free: worldspaces as nodes, links as edges with walking cost)
  routes across spaces; the local planner paths to the next link's approach point; the traverse action opens the door
  if shut (activation rules apply), waits for `passable`, walks through the `Gate`. The same mechanism serves quest
  markers and compass ("route through the door").
- **Fade-only links** (trapdoors, one-way): actors take them only when no viewer sees either end (Bethesda style
  teleport out of view).
- **Perception across the door:** sight via `line_of_sight` through the opening, hearing via `path_distance` (OpenMW
  `heard`): alarms, crimes and combat pursue across portals with no per-package hacks.
- **Off-screen simulation:** actors outside the simulated set run a coarse schedule on the `WorldGraph` (location
  updates with travel time, no physics, no animation), placed on the nearest valid nav node when their space becomes
  simulated: the Bethesda low-process tier. This belongs to `sf_ai` (no spec yet); park it, but give `sf_ai` the
  `WorldGraph` and `Location`-based schedule as its seam.
- **Followers:** walk through like anyone; if blocked for N s and out of every viewer's sight, take the link
  (vanilla's follower teleport as the fallback, never the default).

### 4.9 Physics

- `PhysicsWorlds<K>` already holds one Rapier world per loaded worldspace: **step every world in the simulated set**
  (fixed tick, same order every tick; net rule). The far R0 near a door is small (an interior or a few exterior cells).
- Body crossing = remove/insert with transformed pose and velocities in the same tick (no body in two worlds). A body
  straddling the plane keeps colliding with its own side only, which the passage filter makes harmless. Valve-style
  cloned collision near the portal is not needed at walking speeds; revisit for thrown items if the owner sees
  clipping.

### 4.10 Saves and mods

- **No save format change** beyond the per-door `PortalState` blob in the overlay; `Location` (worldspace + pos) and
  `CharacterSave` already suffice. A save taken mid-doorway restores the authoritative side (it can't be ambiguous:
  the `Gate` already decided). Load keeps the burst + fade (both prior-art engines keep a loading screen on load).
- **Mods:** links, frames and portal readiness are bake products of the merged load order (fingerprint includes door
  model bounds, world-adapter "Next step"). A mod door that is thick, horizontal, one-way or mismatched > 2× stays
  fade-only automatically. Scripted doors keep working because opening a door *is* the activation event (see 4.11).
- **Seamless stays a setting** (owner 2026-10-12), with fade as the classic path; both share the data.

### 4.11 Morrowind adapter (`mw_*`)

- `mw_world`: wire `portal::analyze` into `WorldRecords::build_with` (frames, per-dimension minimum extent, hinge
  side); approach points from the nearest PGRD node in front of each door (`mw_ai::pathgrid`).
- `mw_doors_bevy`: activation → `OnActivate` scripts first (locked, trapped, scripted doors that never call
  `Activate` simply don't open); procedural swing (hinge from bounds, interior leaf swings inward, OpenMW convention;
  exterior leaf hidden and the far copy seen, or the far door's back drawn on the leaf); auto-close rule; door sounds.
- `CellEnvironment` (AMBI ambient, sunlight colour, fog colour/density, "behaves like exterior") → `sf.environment`;
  water level unchanged.
- MWScript: `CellChanged`, `GetPCCell`, `GetInterior` fire at the `Gate` crossing; `COC`, `Position`, `PositionCell`
  and fast travel keep the fade.
- Arrival markers stay for fade links and actor teleports; portal-ready crossings ignore them (rigid transform).

### 4.12 Co-op (net.md)

- Interest set = same worldspace within ~R1 **∪ the far side of the player's open portals within ~R0 of the far
  door** (what they can see and hear through it), using `portal_query::map`.
- Door `PortalState` is host-authoritative (net.md already lists doors); the client predicts its own `Gate` crossing
  (deterministic from position) and the host confirms. Remote players: interpolation history stores per-sample
  worldspace; a sample pair that straddles a crossing is interpolated in one space through the link transform (else
  the remote avatar lerps across the map).
- The host's simulated set is the union over players (each player's far sides join that player's physics region);
  the host renders portal views only for its own player.

### 4.13 Sketch (illustrative, not compiled)

```rust
// sf_portal (Bevy-free)
pub struct PortalState { pub openness: f32, pub opening: bool, pub passable: bool }

/// Weight of the far side's environment for a viewer at `local` (in the near end's frame:
/// x across, y up from the sill, z out of the opening into the near space).
pub fn far_weight(local: Vec3, extent: Vec2, openness: f32, depth: f32) -> f32 {
    if local.z < 0.0 { return 0.0; }                       // already crossed: caller swaps roles
    let t = (1.0 - local.z / depth).clamp(0.0, 1.0);
    let s = t * t * (3.0 - 2.0 * t);
    let off = footprint(local, extent);                     // 1 inside the opening's footprint, falls off outside
    0.5 * openness * s * off                                // 0.5 at the plane on both sides: continuous
}

pub fn gate(prev: Vec3, now: Vec3, extent: Vec2) -> bool {  // centre crossed the plane inside the opening
    prev.z >= 0.0 && now.z < 0.0 && now.x.abs() <= extent.x * 0.5 && (0.0..extent.y).contains(&now.y)
}
```

## 5. Existing decisions that should change

1. **"Only the active worldspace simulates"** (streaming.md "Decided" walkers2; DECISIONS 2026-10-12): becomes the
   simulated set (4.2). Processing range inside each worldspace is unchanged.
2. **`PhysicsWorlds`: "only the active one steps"** (`sf_physics/src/world.rs` 977): step the simulated set.
3. **Global `ActiveWorldspace` and the app-level `worldspace_look`**: per-viewer worldspace and environment
   (portals.md M8 design item 3 already says it; net.md says it for co-op). Derived `ActiveWorldspace` during migration.
4. **`ListenerOutdoors(bool)`** (audio.md "Snapshots"): a continuous `ListenerExposure`.
5. **GI: interior volume "rebuilt when the worldspace changes"** (gi.md): volumes live per loaded, visible worldspace,
   not per active one.
6. Portals.md "history reset at the crossing" for the main view: replaced by motion vectors written through the
   opening; keep the reset for the portal camera when it changes link.
7. Milestones: ARCHITECTURE says M9 = "portal rendering, crossing, environment blending". NPC crossing needs the
   simulated set and nav links, and off-screen AI needs `sf_ai` (M10 era): split M9 into M9a (player, look, sound)
   and M9b (actors, physics, AI), or accept that M9 lands without NPCs walking through.

## 6. Slice plan (one deliverable + test each)

| Slice | Deliverable | Test | Owner check |
|---|---|---|---|
| **SI0** | Spec: `superfargoth/docs/specs/portals.md` v2 from this note (framework) + world-adapter "Portal frames" wiring section; DECISIONS bullets for section 5 after owner go | doccheck | read and decide section 7 |
| **SI1** | Bake portal frames: `WorldRecords::build_with` sets frames/extents/hinge for portal-ready and adjust links; fingerprint includes door bounds | `real_install`: link counts by class (1,361 / 251 / 216), every portal-ready end has extent > 0, sills grounded | none (data) |
| **SI2** | `sf_portal` core: `PortalState`, `gate`, `far_weight`, swing timing, passable rule | exact: cross∘cross⁻¹ = id, weight 0.5 at the plane from both sides, gate symmetric, no double crossing | none |
| **SI3** | `InWorldspace` + per-camera `ViewerWorldspace`; `ActiveWorldspace` derived; consumers migrated (look, GI, terrain, precip, audio) | two cameras in two worldspaces get the right layers; derived resource tracks the player | none |
| **SI4** | Door swing (adapter): procedural hinge swing, auto-close, `OnActivate` routing, state in overlay | hinge side from bounds; overlay round-trip; scripted door that blocks `Activate` stays shut | DR-: Census door opens both ways |
| **SI5** | `sf_portal_bevy` view pool from the spike: oblique clip, screen-rect viewport, opening quad, thick-portal box, on-screen gating | headless plugin smoke: camera layers, clip plane, viewport inside window; log line on view start/stop | PT-: look in from the street, out from the office |
| **SI6** | Player crossing with no fade + passage filter in `sf_controller` probes; `TravelArrived { seamless }` | headless walk through the Census exit: worldspace changes once, no fall, no stuck, velocity mapped | PT-: walk in and out at a run |
| **SI7** | Motion vectors and depth through the opening (portal material prepass) | shader compile log per preset | PT-: DLSS on, pan across the doorway: no smear |
| **SI8** | `sf.environment` kind + adapter mapping + per-viewer blend (ambient, fog, exposure bias) | weight continuity; `CellEnvironment` mapping exact | PT-: no jump in light or fog at the crossing |
| **SI9** | `PortalLight` + GI light-portal quad; GI volumes kept for open portals | light colour formula; volume lifetime logic | GI-/PT-: daylight pool inside the open door, night reverses |
| **SI10** | Weather through portals: "one exterior viewer" anchoring for precip/shelter/cloud cameras; draft wind | anchor selection logic; draft zero when shut | WE-: rain seen through the door, none indoors, porch dry |
| **SI11** | Audio: `ListenerExposure`, far-side voice routing, portal ambience emitter | exposure continuity; virtual position and path-length math | AU-: rain fades in as you approach the door, tavern heard from the street |
| **SIM** | Measurement: portal view cost looking in and looking out (Census, Balmora street door, Vivec canton), per preset | `#[ignore = "measurement: ..."]`; owner runs the suite | owner |
| **SI12** | Simulated set: portal driver anchor at the mapped pose; physics steps the set; body and projectile crossing | crate pushed through lands in the other world with mapped velocity, nothing NaN; set membership exact | PH-: throw an item through the door |
| **SI13** | `WorldGraph` + `portal_query` (map, path_distance, line_of_sight) | exact routes on a synthetic 3-space graph; LOS through an opening | none |
| **SI14** | AI door traversal on the `mw_ai` scaffold: approach points, traverse action, follower/pursuit through doors, perception via `portal_query` | headless: an NPC follows the player through the Census door; a guard pursues out | AI-: follower through Balmora doors |
| **SI15** | `[p]` Off-screen coarse AI tier on `WorldGraph` (in the `sf_ai` spec) | schedule placement determinism | parked on `sf_ai` |
| **SI16** | `[p]` Co-op: interest through portals, interpolation across crossings | interest set exact; straddling interpolation stays on the link | after net N5 |

Order: SI0 → SI1 → SI2 → SI3 → SI4/SI5 → SI6 → SI7 → SIM → SI8 → SI9/SI10/SI11 → SI12 → SI13 → SI14. SI3 is the
risky refactor; do it before any rendering so the rest is additive.

## 7. Risks and open questions for the owner

**Risks.**
- *Frame time.* +1.5 ms looking in was measured; looking out is roughly a second exterior frame inside the opening
  (sky, atmosphere, shadow cascades), and H1 already has Balmora over budget. Mitigations: screen-rect viewport,
  on-screen gating, cheaper far view (fewer cascades, no clouds raymarch, lower resolution), one view on Medium.
- *Temporal artefacts* in the opening if the motion-vector write-through (SI7) falls short; fallback is the spike's
  history reset (a visible flicker).
- *SI3 touches many systems* (look, terrain, GI, precip, audio, physics): it must land in one compiling slice with a
  derived `ActiveWorldspace` so nothing else breaks.
- *Upstream gaps:* light probes ignore `RenderLayers`; Solari's TLAS ignores layers; no pre-exposure.
- *Content edge cases:* 251 mismatched openings (sill-aligned minimum works, larger side shows extra wall margin),
  ambiguous Balmora shop doors (252), floating arrival markers (only matter for fade links), scripted doors.
- *Simulation cost* of live far sides: an open door in Vivec makes a canton's actors tick; cap by R0 and actor tiers.

**Questions.**
1. Make seamless the **default** for portal-ready links once SI6-SI8 pass (today: optional, fade default)?
2. Doors: visible swing with auto-close (OpenMW fork) or doorways that stay open once used? Swing time ~1 s?
3. How "live" is the far side: full AI + physics within R0 of the far door, or only visible actors idling until you
   cross (cheaper, OpenMW's stand-ins)?
4. Split M9 into M9a (player, rendering, look, audio) and M9b (actors, physics, AI), given no `sf_ai` spec yet? Write
   the `sf_ai` spec (with the `WorldGraph` seam) before SI14?
5. Portal views: one on Medium, two on High; chained spaces (exterior → canton → plaza) show the second door shut. OK?
6. Fade-only links (216: trapdoors, one-way, > 2× mismatch): keep the fade, or animate trapdoors as a drop later?
7. Followers: walk through, with the vanilla teleport only as an out-of-sight fallback?

## Sources

- OpenMW seamless fork, commit `48392fca` on `seamless-doors`: https://gitlab.com/duron27/openmw-seamless/-/commit/48392fca
  (files cited above: `apps/openmw/mwworld/seamlessdoors.{hpp,cpp}`, `components/sceneutil/doorportal.{hpp,cpp}`,
  `apps/openmw/mwphysics/doorway.{hpp,cpp}`, `apps/openmw/mwworld/scene.{hpp,cpp}`, `weather.cpp`,
  `mwsound/soundmanagerimp.cpp`, `mwmechanics/{actors,aicombat,aifollow,aipackage,pathfinding,mechanicsmanagerimp}.cpp`,
  `docs/source/reference/modding/settings/cells.rst`); base: OpenMW master `5b75e3eb`, https://gitlab.com/OpenMW/openmw
- SPUD at `8c9904207a5c573a0193fb903fb5c433f127f102`: https://github.com/chasmlol/spud (`docs/ARCHITECTURE.md`,
  `docs/formats/osw.md`, `src/engine/doors.hpp`, `src/engine/app_doors.cpp`, `src/render/renderer_portal.cpp`,
  `shaders/portal_stamp.frag`)
- Creation Kit, Bethesda Tutorial World Hookup: https://ck.uesp.net/wiki/Bethesda_Tutorial_World_Hookup
- Skyrim navmesh door links (finalize, door triangles): https://www.nexusmods.com/skyrimspecialedition/articles/6854
- CryEngine VisArea and Portals: https://cryengine.com/docs/static/engines/cryengine-3/categories/1114113/pages/1048727 ,
  https://cryengine.com/docs/static/engines/cryengine-5/categories/23756816/pages/26215443
- RAGE MLO rooms and portals (modding guide, secondary source): https://fivemx.com/blog/how-to-create-fivem-mlos
- Umbra in The Witcher 3 (GDC 2014): https://www.gdcvault.com/play/1020686/Solving-Visibility-and-Streaming-in
- Unreal Audio Gameplay Volumes: https://dev.epicgames.com/documentation/unreal-engine/audio-gameplay-volumes-quick-start?lang=en-US ;
  Audio Volume: https://dev.epicgames.com/documentation/unreal-engine/audio-volume-actor-in-unreal-engine
- Top-down occlusion maps: Wronski, AC4 lighting/weather deck https://api.sambuz.com/doc/assassin-s-creed-4-black-flag-ppt-presentation-541266 ;
  Barrero, CoH2 rendering https://fr2.slideshare.net/proyZ/daniel-barrero-coh2renderingtech
- Wwise Rooms and Portals (not re-read): https://www.audiokinetic.com/en/library/edge/?source=SDK&id=spatial_audio_roomsportals.html
- Our docs: `superfargoth/docs/specs/{portals,streaming,net,gi,audio,surface-weather}.md`, `weather/{state,fog}.md`,
  `specs/archive/portals-history.md`, `WISHLIST.md` §6, `bevyrrowind/docs/specs/world-adapter.md`
