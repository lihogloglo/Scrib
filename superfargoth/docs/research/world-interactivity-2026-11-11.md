# World interactivity: smart objects, carrying, destruction, mounts and vehicles (research, 2026-11-11)

For ROADMAP 3.11 (owner 2026-11-11: "crates can be broken, buildings can be damaged, NPCs can move the chair around to
sit on it, they can grab stuff just like we do, mount something, go inside a vehicle"). Feeds
[specs/interaction.md](../specs/interaction.md) and [bevyrrowind/docs/specs/interaction-adapter.md](../../../bevyrrowind/docs/specs/interaction-adapter.md).
Points marked **(unverified)** come from recall of talks, games and mod communities without a source read this pass;
they shape the design only where a verified source or our own constraints agree.

## 1. Bevy 0.20 (native)

- Bevy 0.20.1 (2026-10-28, [crates.io](https://crates.io/api/v1/crates/bevy)); [release notes](https://bevy.org/news/bevy-0-20/).
- Has: observers and entity events (glue for "interaction started/ended"), relationships (`ChildOf`, custom
  `#[relationship]` pairs: a rider/seat link can be one in the shell), `MeshPickingPlugin` (render-side ray picks; our
  focus already uses physics rays, physics.md "Carry and throw"), BSN scenes (still evolving).
- Has not: smart objects, slot reservation, physics, destruction, mounts, vehicles. Verdict: glue only.

## 2. Crates (Rust ecosystem, checked 2026-11-11)

| Crate | Version / date | Bevy | Licence | Verdict |
|---|---|---|---|---|
| `rapier3d` (ours, 0.36.1) | 0.36 | Bevy-free | Apache-2.0 | **Fits.** Impulse + multibody joints (multibody springs 0.34, compliant motors 0.35.2), `ContactForceEvent` with a threshold (impulse = whole step since 0.35.2: usable for "break when hit hard"), `DynamicRayCastVehicleController` maintained (0.35 brake fixes), KCC pushes dynamics (0.36). **No native breakable joint** (read impulses, remove the joint: our code; [joints guide](https://rapier.rs/docs/user_guides/rust/joints), [CHANGELOG](https://raw.githubusercontent.com/dimforge/rapier/master/CHANGELOG.md)). No moving-platform velocity in the KCC (unverified; we use our own `KinematicMover` anyway). |
| `avian3d` | 0.8.0, 2026-10-20 | 0.20 | MIT/Apache | Not a dependency (we chose Rapier; physics.md "Backend decision"). Reference only. |
| `bevy_tnua` | 0.33.2, 2026-11-06 | 0.20 | MIT/Apache | ECS-coupled controller; its moving-platform handling (ground entity velocity) is a reference (unverified detail). |
| `bevy_carnage` | 0.8.13, 2026-11-02 | 0.19? (unverified) | MIT/Apache | Runtime gore mesh cutting into capped chunks, mesh-cut core said to be ECS-free. Young (2 months). Watch for later; not prop destruction. |
| `csgrs` | 0.20.1, 2025-07-24 | Bevy-free | MIT | BSP CSG; could cut Voronoi cells offline. Robustness unchecked. |
| `big-brain` | 0.22, 2024-11 | 0.14 | Apache | Stale. No. |
| `bevy_observed_utility` | 0.2, 2025-03 | old | MIT/Apache | Stale. No. |
| `bevy_behave`, `beet`, `seldom_state`, `bonsai-bt` | 2026 | mixed | MIT/Apache | BT/FSM frameworks; we already have an HTN core (ai.md "Reuse"). No smart-object layer in any. |
| bevy "interact"/"mount"/"vehicle"/"seat" crates | none found | - | - | Nothing to adopt (search not exhaustive). |

Offline pre-fracture tools: Blender's Cell Fracture add-on (GPL, fine as an offline tool whose output is data),
parry3d's VHACD for chunk hulls (we already link parry), a small Voronoi-plane cutter on `csgrs` if a bake step is ever
wanted. NVIDIA Blast (C++, licence not re-checked this pass) has no maintained Rust binding (unverified).

## 3. Open-source projects

- **Veloren** (Rust, GPL-3.0): mounts and ship riders in `common/src/mounting.rs`
  ([source](https://gitlab.com/veloren/veloren/-/raw/master/common/src/mounting.rs)). `Mounting { mount: Uid, rider: Uid }`
  is a *link* with `create` (refuses a taken spot, a rider already riding, the dead), `persist` (re-validated every tick:
  both alive, spot still registered, block unchanged) and `delete` (removes markers, moves the rider to a nearby free
  spot, forces a client update). Riding a ship is `VolumeMounting { pos: VolumePos, block, rider }`; the ship carries a
  `VolumeRiders` map from block position to link, so seats are claimable spots; a rider's world matrix is derived from
  the ship's `Pos`/`Ori` (attached, not simulated against it). Links use stable `Uid`s, so they serialise and sync.
  The closest shipped match to our net rules; design reference (porting is allowed under GPL-3, with a notice).
- **OpenMW**: no furniture use, mounts or destruction in the engine. Morrowind mods: guar mounts exist but are
  MWSE/vanilla-script hacks, several marked "not compatible with OpenMW" ([Guar Pack mount](https://nexusmods.com/morrowind/mods/48247),
  [Smart mountable guars](https://www.nexusmods.com/morrowind/mods/43008), [Rot for the Riders](https://www.nexusmods.com/morrowind/mods/46868));
  OpenMW-Lua "Devilish Horse Riding" and its [Nix-Mount](https://www.nexusmods.com/morrowind/mods/57187) variant,
  [Traveling Guar Riders (OpenMW)](https://modding-openmw.com/mods/traveling-guar-riders-openmw) (NPC riders),
  [guar-cart travel](https://www.nexusmods.com/morrowind/mods/45645). The community wants mounts; nothing to port.
- Not checked this pass: Daggerfall Unity (horse/cart), O3DE, Godot demos.

## 4. Industry

### Smart objects (shared player/NPC interaction points)

- **The Sims** (Wright; Forbus's "smart objects"): objects *advertise* the needs they satisfy; agents score adverts
  against their needs and distance; the object carries the animation and the interaction script, so new objects add
  behaviour without growing the agent (unverified detail; the model is widely documented).
- **UE5 Smart Objects** ([overview](https://dev.epicgames.com/documentation/en-us/unreal-engine/smart-objects-in-unreal-engine---overview),
  [claim handle API](https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/SmartObjectsModule/FSmartObjectClaimHandle)):
  a definition asset (shared, immutable) with N **slots** (transform relative to the anchor, tags, behaviour
  definition overrides); `FindSmartObjects` filters by user tags, activity tags, behaviour class and area; lifecycle
  **Free → Claimed** (`Claim`, a handle; invalid = try another) **→ Occupied** (`Use` on arrival, returns the
  behaviour) **→ Free** (`Release`, the user's duty); the object changing state (destroyed) **invalidates** all claims
  and notifies users. **Entrance annotations** give several approach points per slot, ranked and optionally validated
  against navmesh and collision ([FindSlotEntranceLocation](https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/GameplayInteractionsModule/FStateTreeTask_FindSlotEntranceL-)).
  A persistent collection keeps objects simulated while their actors are unloaded. Player and AI are both
  "interactors"; the object supplies data, the user runs the logic. Forum reports show double-claim and entrance-query
  bugs (5.6), a reminder to test the claim table hard.
- **Bethesda** (Creation Kit): furniture (`FURN`) carries **markers** with entry-direction flags (front, behind, left,
  right, up; a separate "lean" flag) ([CK Furniture](https://ck.uesp.net/wiki/Furniture), [FURN format](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/FURN));
  directions that lack enter clips are handled with object variants (`CommonWritingChair01L/R/LR`); idle markers
  ([IdleMarker](https://ck.uesp.net/wiki/IdleMarker)) play idles at a spot; AI packages (Sandbox, UseFurniture) pick an
  unclaimed marker (unverified detail). Moved Havok props persist as reference overrides in the save (save bloat in
  Skyrim, unverified).
- **GTA V / RDR2 scenario points**: typed spots (bench sit, smoke, lean...) with time windows and probability, placed
  in map data and also **attached to prop models**, so every bench of that model offers the spot; peds claim, path,
  play the anim set (unverified; community tools such as the [CScenarioPoint patch](https://gta5mod.net/gta-5-mods/tools/cscenariopoint-patch-1-1/)
  confirm the type/model-set tables).
- **Cyberpunk 2077 workspots** ([RED modding wiki](https://wiki.redmodding.org/cyberpunk-2077-modding/modding-guides/world-editing/ai-and-npcs/placing-aispot-nodes),
  [workspots](https://wiki.redmodding.org/cyberpunk-2077-modding/modding-guides/animations/animations/abusing-workspots)):
  `.workspot` files hold a tree of work sequences (clip lists, idle and anim names); AISpot nodes place them; the
  "community" system schedules NPCs into spots. Same model as scenario points.
- **Shared pattern**: interaction data lives on the object, not the agent; claim is separate from use; entry/exit
  chosen by side; the user is attached/aligned to the slot while using it; player and NPC share the API.

### Moving and carrying

- Player: Half-Life 2, Amnesia, Skyrim carry/drag (already ours, physics.md "Carry and throw").
- NPC carrying chairs/props: no verified shipped example of an NPC *relocating* furniture to then use it. The common
  technique for NPC carrying (crates on shoulders, sacks) is a carry clip set with the prop attached to a hand socket,
  hand IK to grip points, collision off while held (unverified). Colony sims (RimWorld, Dwarf Fortress) "haul" as jobs
  with abstract visuals. **So "an NPC moves a chair to sit" is ahead of the industry; we design it as a two-step plan
  (carry the object to a free spot, then use its slot) on the same primitives, not as a bespoke system.**
- Crimson Desert: shown grappling enemies and throwing them into walls and other enemies, force-scaled destruction of
  wooden structures ([80.lv BlackSpace summary](https://80.lv/articles/pearl-abyss-demonstrated-crimson-desert-s-blackspace-engine-tech-advancements);
  [fan wiki](https://thegameswiki.com/crimson-desert/wiki/environmental-combat-and-physics), weak source). No public
  detail of NPC object use (unverified).

### Destruction

| Tier | Example | Cost | Co-op |
|---|---|---|---|
| Stage swap (model per health band, debris + explosion at a stage) | Bethesda `DEST` ([CK](https://ck.uesp.net/wiki/Destructible_Objects), [DEST field](https://content2.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/DEST_Field): health, stages with health %, model stage index, flags cap/disable/destroy/ignore-external, debris and explosion refs) | one mesh swap | one integer per object |
| Pre-fractured chunks with a cluster hierarchy | UE5 Chaos Geometry Collections; Frostbite | chunk bodies awake for seconds | weak: Chaos debris replication is a recurring forum problem ([1](https://forums.unrealengine.com/t/5-3-chaos-destruction-replication/1326436), [2](https://forums.unrealengine.com/t/what-is-the-current-state-of-destruction-in-multiplayer/2037228)) |
| Structural graph (supports, collapse) | Red Faction Guerrilla, The Finals (server-side, [GDC](https://gdcvault.com/play/1034280/Eng)), Battlefield 6 ([EA](https://www.ea.com/en/news/how-battlefield-6-redefined-destruction): prioritised, culled part destruction and debris replication) | high | needs a dedicated design |
| Runtime fracture / voxels | Teardown | very high, single-player | poor |

Consensus for co-op: replicate the **destruction event** (object id, stage, impact point and direction, seed), never
debris; debris is client-local cosmetic physics; gameplay-relevant pieces (a blocking beam) are few and replicated.

### Mounts and vehicles

- Elden Ring Torrent (summoned, a separate character, mounted moveset, double jump), Witcher 3 Roach (whistle, road
  follow), Skyrim horses (actors; mount from either side by approach angle), RDR2 horses (actors with bonding),
  Ghost of Tsushima (unverified details throughout). Common technique: **the mount is its own character** with its
  own controller; the rider is **attached to a seat socket** and plays a mounted animation set, legs/hands by IK;
  **control hand-off = input routing** (the rider's command drives the mount while seated).
- Vehicles: GTA V seat indices with per-seat enter/exit clips by door side, door bones, jacking variants
  (unverified). Walking inside moving vehicles: Star Citizen physics grids (local physics frame per ship), most games
  use a moving-platform rule (character velocity relative to the base) (unverified).
- Zelda TotK: Ultrahand/Fuse are physics constraints between props; player-built vehicles are constrained assemblies
  (unverified). Scope for us: later, mod territory.

## 5. What this means for us

1. **One interaction-point model for player and NPC**, data on the object (Sims/UE/Bethesda/GTA agree): slots with
   tags, entrances by side, a clip family (enter/loop/exit), IK targets, a host-owned claim table with leases and
   invalidation. Our `sf_ai` already expects `find_slots`/`claim`/`release` (ai.md "Smart objects") and `sf_action`
   already has `Claim(point)` and an Interaction move category (action.md).
2. **Carrying** reuses our physics carry (player) and adds an attached "held prop" mode for NPCs (socket + hand IK, body
   kinematic), because simulated NPC carry is costly and unpredictable; "NPC moves a chair" = carry + place + use.
3. **Destruction**: stage swap first (cheap, saves and replicates as one integer, matches Bethesda), pre-fractured
   chunks later as an optional stage visual; buildings need a structural design of their own (a hard problem).
4. **Mounts/vehicles**: seat = a slot whose occupant is attached (Veloren link model), the mount/vehicle is its own
   character or body, control hand-off routes the driver's `Command`; walking on moving decks needs a moving-platform
   rule in `sf_controller` first.
