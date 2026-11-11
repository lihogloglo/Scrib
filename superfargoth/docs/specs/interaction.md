# Interaction: smart objects, carrying, destruction, mounts and vehicles (`sf_interaction`)

Status: design 2026-11-11 (ROADMAP 3.11), nothing built. Research:
[world-interactivity-2026-11-11](../research/world-interactivity-2026-11-11.md). Adapter (first content pack):
[interaction-adapter](../../../bevyrrowind/docs/specs/interaction-adapter.md). Touches [ai.md](ai.md) "Smart objects",
[action.md](action.md) (Interaction category, `Claim(point)`), [physics.md](physics.md) "Carry and throw",
[nav.md](nav.md) "Runtime patches", [combat.md](combat.md) "The rules seam", [static-scene.md](static-scene.md) §1
"Promotion" (read only; scene-coord owns it), [save.md](save.md), [net.md](net.md).

<!-- agent-summary -->
> **Agent summary.** One interaction-point model for player and NPC alike: objects carry **slots** (seat, bed, lean,
> counter, saddle, vehicle seat) with entrances by side, a clip family, IK targets and adverts; a **host-only claim
> table** (Free → Reserved → Occupied, leases, invalidation) serves `sf_ai` and `sf_action`. On it: (a) moving and
> carrying (player physics carry exists; NPCs get attached held props), (b) destruction (Bethesda-style stage swap
> first, crates before buildings), (c) mounts and vehicles (a seat slot whose occupant is attached, control hand-off
> by routing the driver's `Command`). Slices IN0-IN13 below, cheapest first.
<!-- /agent-summary -->

## Goal

The owner (2026-11-11): "Games feel real when it's not just a decor but stuff can be interacted with. Crates can be
broken, buildings can be damaged. NPCs can move the chair around to sit on it. They can grab stuff, just like we do.
Same for the vehicles or mounts: being able to mount something, go inside a vehicle." References: Cyberpunk 2077,
Crimson Desert.

Concretely: player and NPCs sit, lie, lean and use counters through the same points; furniture is pulled out, moved,
carried; crates break and spill; buildings take authored damage; creatures are ridden; the player rides travel
vehicles and later walks on moving decks. All of it saves, replicates (host-authoritative, net.md) and fits ≤ 11 ms.
The adapter says which records become interactable; the framework names none.

## Reuse check (CONTRIBUTING rule 0)

- **Bevy 0.20 (native).** Observers, entity events and relationships (a seat link can be a custom relationship in
  the shell), `MeshPickingPlugin`. No smart objects, reservations, physics, destruction, mounts or vehicles.
  Verdict: Bevy is glue only (thin `sf_interaction_bevy`).
- **Crates** (table in the research note §2). No smart-object/slot-claim crate; utility-AI crates are stale, BT/FSM
  crates don't model slots, and we have our HTN core (ai.md "Reuse"). `rapier3d` 0.36.1 stays (physics.md): joints,
  `ContactForceEvent` thresholds, a ray-cast vehicle controller, but **no breakable joint** and no moving-platform
  rule (both ours). `bevy_carnage` (gore cutting) and `csgrs` (CSG) are not prop destruction; pre-fracture is offline
  content (Blender Cell Fracture output, parry3d VHACD hulls). No mount/vehicle/seat crate.
- **Projects.** Veloren (GPL-3.0) `common/src/mounting.rs`: mount and ship-seat **links** by stable `Uid` with
  create/persist/delete validation and a per-vehicle seat map; our seat link follows it (design reference; a port
  cites the file and commit). OpenMW has no furniture use, mounts or destruction; Morrowind mount mods are script
  hacks. Nothing else to adopt.
- **So: our own** `sf_interaction` (Bevy-free core) + `sf_interaction_bevy` (thin), because nothing above models a
  host-owned slot table shared by player, AI, saves and the net layer.

## Industry comparison

| Concern | Industry | Ours |
|---|---|---|
| Point data | On the object: UE Smart Object slots, Bethesda furniture markers, GTA/RDR2 scenario points on prop models, Cyberpunk workspots | `InteractableDef` records: slots, entrances, clips, IK targets, adverts |
| Choosing | Sims adverts (need × value − distance); UE tag filters | `find_slots` by tags; adverts feed `sf_ai` considerations |
| Reservation | UE Claim → Use → Release, invalidated when the object changes | host `Claims`: Reserved → Occupied → Free, leases, invalidation |
| Approach | UE entrance annotations (nav/collision-validated); Bethesda entry flags + L/R variants | entrances by side, nav-validated + `EndPoseFree` |
| Use | enter/loop/exit clips, user aligned and attached | Interaction move (Warp), then a Docked-like `Seated` owner |
| Carry | HL2/Skyrim physics grab (player); socket + hand IK + carry clips (NPC, unverified) | built for the player; NPC `HeldProp`; relocation as an HTN plan |
| Destruction | Bethesda `DEST` stages; Chaos pre-fracture; The Finals/Frostbite structural, server-side; replicate the event, debris local | stage swap first, chunks later, buildings authored; `Destroyed` event + seed |
| Mounts, vehicles | mount is a character, rider on a saddle socket, input routed (Elden Ring, Skyrim, RDR2); GTA seat indices; Star Citizen physics grids | `SeatLink` (Veloren); rail vehicles first; moving platforms later |

Gaps go in [industry-audit.md](industry-audit.md) when built (none known beyond the hard problems below).

## Architecture

```
content (adapter): which base records/models are interactable → InteractableDef records (mod-patchable)
  interact.cook (bake, per model): slot transforms from collision (seat/bed/table tops), bands, grips
sf_interaction (Bevy-free): defs, PointIndex (per cell, by PlacementId), Claims, entrance pick,
                            Destructible stages, SeatLink, HeldProp rules            ← pure, serde, host state
  ↑ find_slots / claim / release        ↑ Claim(point) validator, focus targets       ↑ hits, impacts
sf_ai (UseSlot, Relocate fragments)    sf_action (Interaction, Seated, Mounted)      sf_combat / sf_physics events
sf_interaction_bevy (thin): builds the index on cell load, follows moved bodies, runs claims and destruction on
                            FixedUpdate, emits InteractionEvent, attaches held props and riders, save section
```

Dependencies: `sf_interaction` → `sf_ids`, `sf_coords`, `sf_world` (`PlacementId`, `WorldPos`), `sf_records`, serde.
Never Bevy, Rapier or `sf_ai`. `sf_ai` may depend on it (Bevy-free); `sf_action` does not: the shell evaluates the
claim and passes the result in `Sense` (as it does for probes), so the action core stays independent.

## The shared core

### Definitions (`sf.interact.def` records, data, mod-patchable)

```rust
// Illustrative, not compiled.
pub struct InteractableDef {
    pub id: RecordId,
    pub slots: Vec<SlotDef>,
    pub destructible: Option<DestructibleDef>,  // (b)
    pub carry: Option<CarryDef>,                // (a): grips, carry class, relocatable, slide axis
    pub adverts: Vec<(TagId, f32)>,             // what using it satisfies (rest, social, work), Sims-style
}
pub struct SlotDef {
    pub kind: TagId,                 // seat, bed, lean, counter, saddle, vehicle_seat, idle ...
    pub local: Iso,                  // slot frame in the object's frame (pelvis/root target)
    pub band: f32,                   // seat/bed/top height, m (picks the clip band, motion-generation.md)
    pub entrances: SmallVec<[Entrance; 4]>,     // side (Front/Back/Left/Right), approach point, enter/exit clips
    pub clips: ClipFamily,           // slot ids: "<kind>.<family>.{enter.<side>,loop,exit.<side>}" (motion-generation ids)
    pub ik: IkTargets,               // hands (armrests, table edge, reins), feet (floor, stirrups), look
    pub occupancy: Occupancy,        // Pose (aligned, mover off) | Attach (rider follows a moving owner)
    pub control: Option<Control>,    // Driver | Passenger (mounts and vehicles)
    pub users: TagFilter,            // who may use it (body type, size class, player/NPC), adapter tags
    pub group: Option<u8>,           // slots that exclude each other (a bench's middle vs ends)
}
```

- Defs come from the adapter (`InteractionContent::def_for(base, model)`) and from mods; `interact.cook` (one-time
  bake, mod-aware, seconds) measures slot frames and bands from collision top surfaces (the MG4 census logic,
  motion-generation.md "Furniture and race census", promoted to a cook step). Unknown tags or clip slots are load
  errors naming the mod (mods.md).
- **Points are derived, never saved**: the index is rebuilt from placements (and the overlay's moved transforms) on
  cell load.

### Point index and focus

- `PointIndex` per resident cell: `SlotRef { placement: PlacementId, slot: u8 }` → world frame, kind, band, owner
  entity. A moving owner (a mount, a vehicle, a carried chair) updates its slots each tick from its body; a resting
  object updates once when it falls asleep.
- `find_slots(near: WorldPos, radius, filter: TagFilter, user) -> impl Iterator<(SlotRef, score)>` (grid buckets;
  score = advert value for the user's need × distance falloff, adapter curve). `sf_ai` calls it; the player's focus
  query (shell ray/cone, `sf_action::Focus`) gets slots as targets with their tag and prompt verb.

### Claims (host only, net rule 5)

```rust
pub struct Claims { by_slot: HashMap<SlotRef, Claim>, by_user: HashMap<UserKey, SlotRef> }
pub struct Claim { pub user: UserKey /* ActorKey | PlayerKey */, pub phase: Phase /* Reserved | Occupied */,
                   pub since: Tick, pub lease_until: Option<Tick> }
```

- `claim(slot, user, tick)`: refused if the slot, its exclusion `group` or the user is taken, or the user fails the
  filter. One claim per user. Same-tick conflicts (two players, co-op) resolve deterministically: lower `UserKey`
  first, then the request order of the tick's command list.
- **Lease**: a Reserved claim expires unless renewed by progress (an NPC walking to it; default 10 s of no path
  progress, adapter-tunable); Occupied has no lease.
- `occupy` on entering, `release` on the exit's end. **Invalidation** releases and notifies (event) when the owner
  object is destroyed, moved beyond a tolerance (0.25 m or 20°), unloaded, or the user dies, is knocked down
  (`Takeover`), or leaves its worldspace. The user's executor then exits (a forced stand-up) or replans.
- Claims are **not saved**: on load, `sf/ai` restores the NPC's activity and the AI's promotion resolve re-claims
  (ai.md "Tiers and budget"); a seated player saves `seated: Option<SlotRef>` in `CharacterSave` and re-claims first.

### Using a slot (one path for player and NPC)

1. **Approach**: the entrance by side (nearest valid; NPC: nav-validated approach point, nav.md; all: the `EndPoseFree`
   capsule test at the slot's end pose).
2. **Enter**: an `Interaction` move rule (action.md: Warp to an interaction slot): `validate: [Claim(point),
   EndPoseFree]`, warp aligns the enter clip's end onto the slot frame; the claim becomes Occupied.
3. **Loop**: a `Seated` owner (Docked-like): root pinned to the slot frame, mover off, the capsule replaced by the
   slot's seated shape (contact-pair filter against the object, physics.md); loop clips and variants
   (motion-generation.md activities), IK to the slot's targets (hands on armrests or table, feet on the floor,
   animation.md two-bone IK), look-at free.
4. **Exit**: by input (player: Move, Jump or Interact; NPC: plan step done) or invalidation; exit clip by side, chosen
   by `EndPoseFree` (blocked side → another, Bethesda's L/R rule), then release.

The NPC path is `sf_ai`'s `UseSlot(slot)` operator (claim, approach, enter, loop, exit), emitting the same `Command`
(`Activate(tag)` + target) the player's input does (net rule 2). Smart objects add method fragments per slot kind
(ai.md "Behaviours"), so new furniture adds no AI code.

### Events (net rule 8)

`InteractionEvent { tick, user, slot, kind: Claimed | Entered | Exited | Released | Invalidated(reason) }`, plus
`Destroyed`, `Mounted`, `Dismounted` below. Scripts (adapter: MWScript `OnActivate`), audio, barks and crime read them.
NPC reactions to changed objects read `WorldChange` events ([world-reactions.md](world-reactions.md), IN14-IN16).

### Cost, net, save, risk (core)

- Cost: index rebuild per cell load (hundreds of slots, < 1 ms on the loader); `find_slots` a bucket scan at AI
  decision rate (tier 2: 5-10 Hz); claims O(1). Frame cost negligible.
- Net: host owns claims and the index; replicated is each character's current activity (slot + phase, ai.md
  "Co-op"); a client's sit press is a request the host validates; clients predict only their own enter clip.
- Save: nothing new for the core (points derived, claims rebuilt).
- Risk: low. Claim-table bugs (UE forum reports of double claims) → exact tests for every transition.

## (a) Moving and carrying (player and NPC)

**Player.** Carry, drag and throw exist (physics.md "Carry and throw", `HolderLimits` from strength). New:
- **Furniture is grabbable** when its def has `carry`: a static in the static scene is **promoted** first (static-scene
  §1: one flag write, an entity spawned with the same placement id), then gets a dynamic body. Heavy pieces drag
  (mass > `carry_mass`), light ones carry; its slots follow the body and refresh on sleep.
- **Moved furniture persists** through `sf/overlay` moved placements (save.md; the physics blob already stores item
  poses). Ownership: moving is not theft (physics-adapter.md), taking is.

**NPC.** A simulated grab (joint + motors) for every NPC is costly and unpredictable; the industry attaches. So:
- **Chair pull-out** (the common case, cheap): a slot entrance may carry `pre_move: Slide { axis, distance }`; the enter
  clip's first phase slides the chair kinematically along its axis (0.3-0.5 m, synced to the clip's hand contact), the
  exit pushes it back. The chair body is kinematic during the slide, then rests; the new pose is saved only if it stays.
- **Held props** (`HeldProp { holder, grips: [hand; 2], carry_class }`): the object becomes a kinematic body attached
  to the holder's hand socket(s) (the `Weapon Bone`-style attach of first-person.md "Held" and MG7a), hands on the
  def's grip points by IK, collision against the holder filtered, a carry clip set by class (`carry.one_hand`,
  `carry.two_hand`, `drag.chair`, `carry.shoulder`). Mass gate: the NPC's `HolderLimits`, as for the player.
- **Relocate** (the owner's "NPC moves the chair to sit"): an HTN fragment `Relocate(obj, spot)` = `MoveTo(obj)` →
  `Pick` → `MoveTo(spot)` → `Place` (navmesh-validated floor spot, capsule-free, facing from the def's `pairs_with`
  tags, e.g. a table edge) → `UseSlot`. The AI picks it when no free slot scores above a threshold but a movable object
  with that advert is near (Sims-style). Placement drops the object dynamic onto the floor, it sleeps, the index updates.

Dependencies: static-scene promotion (built B5a, B5a-2 running); AI wired into the game (ROADMAP 3.10) and nav (NV
slices) for NPCs; carry clips (3.8 clip factory, MG7b grip table); hand IK (animation.md, the mantle lip IK path).
**Cost**: a held prop is one kinematic transform per tick; a moved furniture body sleeps within ~1 s (physics budget
unchanged). **Nav**: moved furniture is not carved by default (nav.md: ORCA and the mover avoid it); a record may opt
into carving (`carves_nav`, NV10). **Static scene**: a moved or destroyed scene placement must stay an entity on the
next cell load (the overlay must make it ineligible): to confirm with scene-coord (adapter eligibility hook, §1).
**Net**: the host simulates; a held prop replicates as its link (holder, grips), not a transform stream; dynamic
furniture replicates like any body. **Save**: moved placements in `sf/overlay`; a held prop saves as placed at its
current pose (the plan is rebuilt on load). **Risk**: medium (hand IK quality, carry clips); relocation is ahead of
the industry (no verified shipped example), so it comes after the cheap pull-out.

## (b) Destruction (crates first, buildings later)

**Tier 1: stage swap** (Bethesda `DEST`, the cheapest, saves and replicates as one integer).

```rust
pub struct DestructibleDef { pub health: f32, pub material: TagId /* wood, clay, glass, stone */,
    pub stages: SmallVec<[Stage; 4]>, pub ignore_external: bool }
pub struct Stage { pub at: f32 /* health fraction */, pub model: Option<ModelRef> /* None = remove */,
    pub debris: SmallVec<[DebrisRef; 4]>, pub effect: Option<RecordId>, pub on_break: BreakAction /* SpillContents | Loot | None */ }
```

- **Damage in**: (1) melee and projectile hits on a prop: combat's swing/contact path gains **prop targets** (the
  object's collision as a hurt volume; combat.md "Hit and hurt volumes" today targets characters only); (2) physics
  impacts: `ContactForceEvent` over a material threshold (a thrown crate breaks a jar, a fall breaks itself); (3)
  spells and explosions (effects area). The adapter's `DestructionRules` turns (source kind, damage type, material)
  into damage (blunt vs blade vs fire), the framework never names a stat.
- **On a stage**: the swap (an entity's model change; a static-scene placement is promoted first), debris spawned, the
  effect played, contents spilled (container items become placed items or a loot bag, adapter), `Destroyed` event.
- **Debris** is cosmetic: `DEBRIS` bodies from a per-material generic set (planks, shards) or the model's own
  pre-cut chunks; each client spawns them from the event's seed; they sleep then fade (8 s), capped per region
  (budget below). Never saved, never gameplay.

**Tier 2: pre-fractured chunks** (optional stage visual, IN12): an offline-cut chunk set per model (Blender Cell
Fracture or a parry/CSG bake tool), the whole model swapped for its chunks at the break, the impact impulse applied at
the hit point. Still the same event and saves.

**Tier 3: buildings.** Morrowind-scale buildings are huge statics in the static scene and interior cells behind doors;
structural collapse (Red Faction, The Finals) needs a structural graph, nav patches and interior consistency, a hard
problem (proposal for `bevyrrowind/docs/hard-problems.md`, owner to decide). Cheap version first: **authored damage
states** for chosen buildings (quest or script set pieces, siege damage, a breachable wall) as stage swaps with large
debris and an authored nav link switch (nav.md link flags), which is how most open worlds ship it.

**Dependencies**: combat prop targets (sf_combat), physics contact events (sf_physics), effects (sf_effects),
container contents (sf_rpg inventory), static-scene promotion for statics (crates and barrels are containers, so
entities already: no scene work for tier 1). **Cost**: a break is one swap plus N debris bodies; cap 48 awake debris
bodies per region and 6 breaks spawned per tick (queued), to be measured on the 4060 laptop (perf work reports ms per
preset). **Nav**: a removed crate frees space (no carving default); a breached wall flips an authored link.
**Net**: `Destroyed { tick, placement, stage, point, dir, seed }`; clients swap and spawn debris locally.
**Save**: an overlay blob `interaction` v1 per placement: health and stage (save.md "Overlay blobs", blob version
table); spilled items are overlay spawned placements; a removed object is an overlay removal. **Risk**: low for
tier 1; medium for chunks (content); high for structures.

## (c) Mounts and vehicles (seats, attach, control hand-off)

**Seat links.** A seat is a slot with `occupancy: Attach` on a moving owner. `SeatLink { seat: SlotRef, rider:
CharacterId }` (stable ids) follows Veloren's link: `create` refuses a taken seat, a rider already seated or dead;
`persist` re-validates each tick (owner and rider alive, seat still registered, same worldspace); `delete` places the
rider at a free dismount spot (exit side by `EndPoseFree`, else any side, else the nearest navmesh point). The rider
is **attached** (root = seat frame each tick, never simulated against the owner), in a `Mounted` owner (Docked-like):
mounted clip set (idle, gait-matched loops, enter/exit by side), leg IK to stirrups or footrests, hands to reins.

**Control hand-off.** A `Driver` seat routes the rider's `Command` to the owner: a mount's own action and locomotion
rules (a quadruped gait set; the mount's AI keeps only reactions such as panic and flee), or a vehicle controller.
`Passenger` seats only look and (later) fight. In co-op the driver's client predicts the mount as it predicts its own
character (net.md "Prediction" extended to "own mount"); the host stays authoritative.

**Kinds, cheapest first.**
1. **Rail vehicles** (travel boats, giant creatures on routes): the vehicle follows an authored route spline at a
   speed; the player sits in a seat and may skip (fade) at any time. "Go inside a vehicle" with no physics risk.
2. **Creature mounts**: the mount is a character (own controller, capsule, gaits, stats); call/summon, follow when
   unridden (`sf_ai` `Follow`), mount by side, dismount, mounted locomotion; mounted combat later (Elden Ring-style
   moveset rules, combat.md).
3. **Physics vehicles** (rowboats, carts): one body, buoyancy pontoons (ROADMAP 3.7) or Rapier's ray-cast vehicle
   controller; seats as above.
4. **Walk-on vehicles** (ship decks, a strider's howdah): needs **moving-platform support** in `sf_controller` (the
   ground body's point velocity added to the move, the mover solved in the platform's frame, as in moving-platform
   controllers) and nav in the platform's local frame; hard, last.

**Dependencies**: `sf_controller` (Mounted mode; moving platforms for kind 4), a quadruped action/locomotion set
(creature locomotion is not built), rider clips (no free mounted pack is known; Kimodo can make a seated-astride
rider, the creature's gaits come from the game's own clips), `sf_ai` Follow (3.10), physics buoyancy (3.7) for boats,
streaming (a ridden mount changes cells like an actor). **Cost**: a ridden mount is one more tier-2 character (pinned
while ridden); a rail vehicle is one transform; a physics boat one body plus pontoons. **Net**: seat links are host
state, replicated as events (`Mounted`/`Dismounted` with tick and ids) plus the link in the character's replicated
state. **Save**: the rider's `CharacterSave` / `sf/ai` activity holds its seat (`SlotRef`); the mount's own state is
its actor save; vehicles' positions are overlay moved or spawned placements; a rail vehicle saves its route progress
in the `interaction` blob. **Risk**: medium (mounts: clips and quadruped locomotion), high (walk-on vehicles).

## Budgets (High preset, RTX 4060 laptop, ≤ 11 ms median; targets to measure)

Index rebuild < 1 ms per cell on the loader (off the frame path); claims and slot follow < 0.05 ms per tick; debris
≤ 48 awake bodies per region and 6 breaks per tick, then queued (measure the physics step); held props and riders one
attach transform each; moving platforms measured in IN11.

## Build slices (cheapest first; one deliverable plus its test each)

| Slice | Deliverable | Depends on | Verify |
|---|---|---|---|
| IN0 | `sf_interaction` core: defs, `PointIndex`, `Claims` (lease, groups, invalidation), entrance pick; `interact.cook` stub | - | headless exact tests: every claim transition, same-tick conflict order, lease expiry, invalidation reasons |
| IN1 | Crate and barrel breaking, tier 1: prop targets in combat, health, stage swap/remove, spill contents, generic debris, `Destroyed`, overlay blob | IN0, combat C-slices | exact: stage math, save round-trip, event; sanity: debris rests, no NaN; visual check IN-1 (owner) |
| IN2 | Player sits and stands on seats: Interaction move, `Seated` owner, side entrances, generated sit clips | IN0, 3.8 clips | exact: claim/occupy/release via `step`; sanity: pelvis on the seat, no capsule through it; IN-2 |
| IN3 | Beds and lean spots (lie down, the adapter's rest hook), hand IK on armrests/tables | IN2 | exact: rest hook fires once; IN-3 |
| IN4 | Impact damage (thrown items, falls) from `ContactForceEvent` thresholds | IN1 | exact: threshold maps to damage; sanity; IN-4 |
| IN5 | Chair pull-out slide on enter, push back on exit | IN2 | sanity: chair ends inside tolerance; IN-5 |
| IN6 | Player moves furniture: promotion + carry/drag, slots follow, persistence | IN2, static-scene B5a-2 | exact: overlay round-trip moves slots; sanity: comes to rest; IN-6 |
| IN7 | Seat links + rail vehicle (ride a travel route seated, skip) | IN2 | exact: link create/persist/delete; sanity; IN-7 |
| IN8 | NPCs use slots (`UseSlot`, `find_slots`, adverts in considerations) | IN2, 3.10 wired, NV route | headless: AI picks and releases; tier invariance; IN-8 |
| IN9 | NPC held props + `Relocate` | IN8, MG7b grips | headless plan test; sanity: prop stays in hands; IN-9 |
| IN10 | Creature mount: summon, mount/dismount by side, hand-off, mounted gaits | IN7, quadruped locomotion | exact: hand-off routing; sanity; IN-10 |
| IN11 | Moving platforms in `sf_controller`; walk on a moving deck | IN7, 3.7 buoyancy | sanity: no slide-off at rest, no tunnelling; measure ms; IN-11 |
| IN12 | Pre-fractured chunk visuals for a stage | IN1 | sanity; IN-12 |
| IN13 | Building damage: design slice (structure vs authored stages) | owner decision | doc |
| IN14-IN16 | `WorldChange` events, reset/persistence classes, loose-item `take` slots ([world-reactions.md](world-reactions.md)) | there | there |

Visual checks go in `bevyrrowind/docs/visual-checks/interaction.md` (prefix IN-) with camera, time, flags and what to
look for; agents never judge them from screenshots.

## Owner questions

1. Destruction scope: every crate and barrel, or only some ("fragile" list per material)? Which weapons break what
   (blunt yes, blade slower, fists?) Contents spill on the floor or into a loot bag?
2. Crime for breaking owned objects: map to the game's existing offences or add a vandalism offence (adapter)?
3. Buildings: park structural destruction as a hard problem and ship authored damage stages (sieges, breaches)?
4. Mounts: which creature for the first pack, owned or summoned, bought from a stable? Mounted combat in scope?
5. Travel vehicles: keep fade travel as the classic option and add ride-along (real time, skippable) as modern?
6. Clip sources: generated (Kimodo, 3.8) sit, lie, carry and rider clips for the player too, not only NPCs?
7. Free sitting (ground, ledges, no furniture) like RDR2: worth an early cheap slice?
8. NPCs moving chairs: pull-out first (IN5), full relocation (IN9) after AI is in the game: agreed?

Owner answers 2026-11-11: (1) everything breakable breaks; contents spill on the floor and take damage scaled by the
hit's strength (a vaporised crate leaves little). (2) A new vandalism offence. (3) Parked, much later; direction: the
outer shell fractures and the linked interior's contents spawn and are destroyed too (needs its own brainstorm).
(4) First the general mechanic only: interact next to a mount, climb-on clip, then drive it; mounted combat later.
(5) Decided by the standard: ride-along on in the modern preset, fade travel in Classic. (6) Yes, Kimodo clips for the
player too. (7) No free sitting for the player (conflicts with ledge mantling); only on furniture (benches, chairs).
(8) Park NPC chair moving until AI is in the game: drop IN5 as a separate slice, fold it into IN9. (9) Yes, a "Sit"
prompt on furniture.
