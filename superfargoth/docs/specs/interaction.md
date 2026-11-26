# Interaction: smart objects, carrying, destruction, mounts and vehicles (`sf_interaction`)

Status: design 2026-11-11 (ROADMAP 3.11), updated 2026-11-25 (IN27: lane roadmap decisions 1-5, 9, 11); nothing built
except IN20 ((c)). Lane roadmap: [interaction](../../../docs/roadmaps/interaction.md) (decisions, stages, owner answers).
Research: [world-interactivity-2026-11-11](../research/world-interactivity-2026-11-11.md),
[gameplay-2026-11-25/interaction](../../../docs/research/gameplay-2026-11-25/interaction.md). Adapter (first content
pack): [interaction-adapter](../../../bevyrrowind/docs/specs/interaction-adapter.md). Touches [ai.md](ai.md) "Smart
objects", [action.md](action.md) (Interaction category, `Claim(point)`), [physics.md](physics.md) "Carry and throw",
[locomotion.md](locomotion.md) (`Docked` mover), [nav.md](nav.md) "Runtime patches" and NV9 links, [crime.md](crime.md)
(`LockState`, decision 8), [combat.md](combat.md) "The rules seam", [static-scene.md](static-scene.md) §1 "Promotion"
(read only; scene-coord owns it), [world-reactions.md](world-reactions.md), [save.md](save.md), [net.md](net.md),
[tunables.md](tunables.md). Activation contract consumer: DQ-6 in
[dialogue-quests](../../../docs/roadmaps/dialogue-quests.md).

<!-- agent-summary -->
> **Agent summary.** One interaction-point model for player and NPC alike: objects carry **slots** of a kind (pose:
> seat, bed, lean, counter; door, lever, ladder, workstation, light; (c) saddle, vehicle seat) with entrances by side,
> a **verb** + priority, clips, IK targets and adverts. A **host-only claim table** on `FixedUpdate` (Free → Reserved →
> Occupied; claim **priority**: a higher user takes a Reserved claim, never an Occupied one; **queue tickets** for
> doors and workstations; leases, groups, invalidation) serves `sf_ai` and `sf_action`. Every use is an
> `Activate(tag)` command the host turns into `Activated` (a script may suppress the default; DQ-6 consumes it).
> On it: (a) carrying, one `HeldProp` link with two presentations (third person attach + clip for player and NPC,
> first person the built spring), carry classes gate speed, sprint, roll; (b) destruction (stage swap first, Chaos-style
> cluster chunks later, debris distance LOD, baked glass shards, loot and salvage, lock bashing; buildings parked);
> (c) mounts and vehicles (TV0b). World rules (fire, `Lit`, noise) are [(d)](interaction-world-rules.md) (IN28). Slices: "Build slices".
<!-- /agent-summary -->

## Goal

The owner (2026-11-11): "Games feel real when it's not just a decor but stuff can be interacted with. Crates can be
broken, buildings can be damaged. NPCs can move the chair around to sit on it. They can grab stuff, just like we do.
Same for the vehicles or mounts: being able to mount something, go inside a vehicle." References: Cyberpunk 2077,
Crimson Desert.

Concretely: player and NPCs sit, lie, lean and use counters through the same points; furniture is pulled out, moved,
carried; crates break and spill; buildings take authored damage; creatures are ridden; the player rides travel
vehicles and later walks on moving decks. All of it saves, replicates (host-authoritative, net.md) and fits the frame target (≤ 8.3 ms median, ≤ 11 ms p99).
The adapter says which records become interactable; the framework names none.


## Reuse check (CONTRIBUTING rule 0)

- **Bevy 0.20 (native).** Observers, entity events and relationships (a seat or held-prop link can be a custom
  relationship in the shell), `MeshPickingPlugin`, `FixedUpdate`. No smart objects, reservations, physics,
  destruction, mounts or vehicles. Verdict: Bevy is glue only (thin `sf_interaction_bevy`).
- **Physics is rapier, not Avian.** `rapier3d` 0.36.1 under our own `sf_physics` (physics.md; Avian stays the
  documented fallback): joints, kinematic bodies, `ContactForceEvent` thresholds, a ray-cast vehicle controller, but
  **no breakable joint** and no moving-platform rule (both ours). Anything Avian-only is a design reference.
- **Crates** (research 2026-11-25 §7; 2026-11-11 note §2). `avian_pickup` 0.7.0 (MIT OR Apache-2.0, Avian 0.8, Bevy
  0.20): **design reference only** (wrong physics engine): one held object per actor, a held prop cannot be stolen by
  another puller, input and output as messages (SFX, VFX), works for player and AI; untested over a network, no mass
  limits (our `HolderLimits` and host link already go further). `bevy_picking_rapier` 0.20: ray picking only, no grab;
  our focus uses the point index. `bevy_carnage` (gore mesh fracture, Bevy 0.19, git), `bevy_shatter` (runtime
  Voronoi glass, Bevy 0.16, stale) and `csgrs` (CSG): **runtime fracture is rejected** (decision 5); pre-fracture and
  glass shard sets are offline content (Blender Cell Fracture output, parry3d VHACD hulls). Licences of `bevy_carnage`
  and `saddle-physics-object-interaction` (Bevy 0.18, Avian 0.6) are unverified: check before any port. No
  smart-object, slot-claim, chemistry, mount, vehicle or seat crate; utility-AI crates are stale, BT/FSM crates don't
  model slots, and we have our HTN core (ai.md "Reuse").
- **Projects.** Veloren (GPL-3.0) `common/src/mounting.rs`: mount and ship-seat **links** by stable `Uid` with
  create/persist/delete validation and a per-vehicle seat map; our seat link follows it (design reference; a port
  cites the file and commit). OpenMW has no furniture use, mounts or destruction; Morrowind mount mods are script
  hacks. Nothing else to adopt.
- **So: our own** `sf_interaction` (Bevy-free core) + `sf_interaction_bevy` (thin), because nothing above models a
  host-owned slot table shared by player, AI, saves and the net layer.

## Industry comparison

Tags as in the research note: [M] = from memory, unverified.

| Concern | Industry | Ours |
|---|---|---|
| Point data | On the object: UE Smart Object slots, Bethesda furniture markers, GTA/RDR2 scenario points on prop models, Cyberpunk workspots | `InteractableDef` records: slots by kind, entrances, verbs, clips, IK targets, adverts |
| Choosing | Sims adverts (need × value − distance); UE Gameplay Tag queries and user-required tags | `find_slots` by tags; `users` filter; adverts feed `sf_ai` considerations |
| Reservation | UE Claim → Use → Release, invalidated when the object changes; **claim priority**: a higher claim takes a lower one unless it is in use | host `Claims`: Reserved → Occupied → Free, priority (player/quest > schedule > ambient) preempts Reserved only, leases, groups, invalidation |
| Crowding | UE leaves it to the user [M]; Sims and RDR2 scenario queues [M] | a **queue ticket** per door and workstation slot |
| Verbs | RDR2 prompt with hold for alternatives, Cyberpunk context list [M]; Bethesda one activate | slot `verb` + priority: focus shows the best, hold lists the rest |
| Activation | Bethesda `OnActivate` replaces the default, `Activate` runs it | `Activate(tag)` command → `Activated`; a handled script suppresses the default (DQ-6) |
| Approach | UE entrance annotations (nav/collision-validated); Bethesda entry flags + L/R variants | entrances by side, nav-validated + `EndPoseFree` |
| Use | enter/loop/exit clips, user aligned and attached | Interaction move (Warp), then a Docked-like `Seated` owner |
| Doors | animated hinged doors NPCs open and close (Bethesda, RDR2) [M] | door slot: Open/Closed, kinematic hinge, doorway claim + queue; Locked read from crime |
| Ladders | ladder mode on a vertical chain plus a nav link (RDR2, TLOU, UE NavLink) [M] | ladder slot = edge chain for the `Docked` mover, both entrances, NV9 link |
| Work | Cyberpunk workspots, RDR2 scenarios, Skyrim crafting furniture (NPC idles, player menu) [M] | workstation slot: NPC job loop (data), the pack's own menu for the player |
| Carry | third person attach to hand + carry clip (RDR2, TLOU) [M]; first person physics grab (HL2, Skyrim) | one `HeldProp` link, two presentations; carry class gates speed, sprint, roll |
| Destruction | Bethesda `DEST` stages; UE Chaos geometry collections with per-cluster damage thresholds and distance debris removal; The Finals/Frostbite structural, server-side; Teardown syncs by determinism | stage swap first, cluster chunks later, debris LOD, buildings parked; `Destroyed` event + seed, debris per client |
| Persistence | Skyrim respawn timers and persistent refs; items saved mid-air [M] | overlay deltas, at-rest snapping, holder link wins (IN15, world-reactions.md) |
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
                            FixedUpdate, emits InteractionEvent, attaches held props and riders, save section,
                            turns Activate commands into Activated → gates → default, swings doors
```

Dependencies: `sf_interaction` → `sf_ids`, `sf_coords`, `sf_world` (`PlacementId`, `WorldPos`), `sf_records`, serde.
Never Bevy, Rapier, `sf_ai` or `sf_crime`, and never a game's formats, ids or names (framework rule). `sf_ai` may
depend on it (Bevy-free); `sf_action` does not: the shell evaluates the claim and passes the
result in `Sense` (as it does for probes), so the action core stays independent.

## The shared core

Tick, net, data (decision 11): claims, uses and breaks advance on `FixedUpdate` (net rule 1), host-authoritative
(rule 5); presentation (swing pose, debris, effects) is seeded per client (rule 4); all content comes through
`sf_records` / `sf_mods` records; the core is Bevy-free and never names a game, `sf_ai` or `sf_crime`.

### Definitions (`sf.interact.def` records, data, mod-patchable)

```rust
// Illustrative, not compiled.
pub struct InteractableDef {
    pub id: RecordId,
    pub slots: Vec<SlotDef>,
    pub verbs: SmallVec<[VerbDef; 2]>,          // slotless uses: carry, take, break, read, the pack's defaults
    pub destructible: Option<DestructibleDef>,  // (b)
    pub carry: Option<CarryDef>,                // (a)
    pub adverts: Vec<(TagId, f32)>,             // what using it satisfies (rest, social, work), Sims-style
}
pub struct SlotDef {
    pub kind: SlotKind,
    pub local: Iso,                  // slot frame in the object's frame (pelvis/root target; door: the doorway)
    pub band: f32,                   // seat/bed/top height, m (picks the clip band, motion-generation.md)
    pub verb: VerbDef,               // "sit", "sleep", "open", "pull", "climb", "work", "snuff" ...
    pub entrances: SmallVec<[Entrance; 4]>,     // side (Front/Back/Left/Right, ladder Bottom/Top), approach point, enter/exit clips
    pub clips: ClipFamily,           // slot ids: "<kind>.<family>.{enter.<side>,loop,exit.<side>}" (motion-generation ids); one-shot families (door, lever, reactions) have only enter.<side>
    pub ik: IkTargets,               // contact points in the object frame: hands (armrests, table edge, rungs, reins), feet (floor, stirrups), look
    pub reach: f32,                  // m: hand claims allowed within this radius while Occupied (a seat at a table); 0 = none
    pub occupancy: Occupancy,        // Pose (aligned, mover off) | Attach (rider follows a moving owner, (c))
    pub control: Option<Control>,    // Driver | Passenger ((c))
    pub users: TagFilter,            // who may use it (body type, size class, player/NPC, armed), UE user tags
    pub group: u8,                   // exclusion bit mask: slots sharing a bit exclude each other (bench middle 0b11, ends 0b01/0b10); 0 = none
    pub queue: Option<u8>,           // ticket capacity; None = no queue (defaults: door, workstation = `interact.queue.capacity`)
}
pub struct VerbDef { pub verb: TagId, pub priority: i16 /* focus order */, pub requires: TagFilter /* state tags, e.g. Lit */ }
pub enum SlotKind {
    Pose { family: TagId },          // seat, bed, lean, counter, idle; saddle and vehicle_seat in (c)
    Door(DoorDef),                   // hinge axis + point, open angle, swing one way or both, close_behind
    Lever(LeverDef),                 // positions (2 = on/off), links: [(target placement, signal tag)]
    Ladder(LadderDef),               // bottom and top points (an edge chain), rung spacing, Bottom/Top entrances
    Workstation(WorkstationDef),     // jobs: TagFilter over job records; player_verb (the pack's menu)
    Light,                           // snuff/relight a light-bearing object: see [(d) World rules](interaction-world-rules.md) (IN28)
}
```

- Defs come from the adapter (`InteractionContent::def_for(base, model)`) and from mods; `interact.cook` (one-time
  bake, mod-aware, seconds) measures slot frames and bands from collision top surfaces (the MG4 census logic,
  motion-generation.md "Furniture and race census", promoted to a cook step), and door hinges and ladder chains from
  the model's bounds where the adapter gives none. Unknown tags, kinds or clip slots are load errors naming the mod
  (mods.md). The adapter may switch a kind or verb off (Classic rules): its slots are then not indexed.
- **Points are derived, never saved**: the index is rebuilt from placements (and the overlay's moved transforms) on
  cell load. Object state that is saved (door open, lever position, health, stage) lives in the `interaction` overlay
  blob ((b) "Save").

### Slot kinds

- **Pose** (seat, bed, lean, counter, idle): "Using a slot" below; beds also fire the adapter's rest hook (IN3).
- **Door** (IN33): state `Closed | Open`, saved. "Locked" is not core state: the shell reads it from crime's
  `LockState` through the activation gate (crime.md decision 8; neither core names the other). The leaf is a
  **kinematic hinge** body turned over `interact.door.swing_s`, away from the user when the def swings both ways; a
  blocked sweep (shape cast against characters) pauses the swing, never pushes a character into geometry. The
  doorway is the slot: an NPC passing claims it (Occupied while passing), others take queue tickets; `UseSlot(door)`
  = claim, open if closed, pass, close behind after `interact.door.close_behind_s` if it found it closed and the def
  says `close_behind`, release. The player never claims a doorway (NPCs wait for them through local avoidance).
  Every state change is a `WorldChange` (IN14) and feeds nav's door state (NV10). **Travel (load) doors are not door
  slots**: their default stays the adapter's (Bevyrrowind: the fade in `mw_doors_bevy`, AI12).
- **Lever** (IN34): positions, saved; the use toggles at the clip's contact event and sends its signal to the linked
  targets (a door, a gate, a light); scripted activators stay scripts (the adapter's). `WorldChange` per toggle.
- **Ladder** (IN43): the def's bottom-to-top points form an **edge chain** consumed by locomotion.md's
  `MoverMode::Docked { edge, s, side, offset }`; entrances at both ends (enter clip onto the chain, exit clip off it;
  jump off mid-way releases). One user at a time (claim; `queue` optional by def). The def registers an off-mesh
  `ladder` link between its entrances (nav.md NV9) whose traversal is `UseSlot(ladder)`, so NPCs cross it (AI33).
- **Workstation** (IN42): a pose slot plus a job set. An NPC runs a **job loop**; the player's use opens the pack's
  own menu (`player_verb`, e.g. alchemy or repair; no new crafting in this lane). **Jobs are mod data** (owner
  2026-11-25): a job is a TOML record file loaded through `sf_records` / `sf_mods` (like Magna Urbis cultures)
  naming its slot kinds, steps (clip slot, duration range, props attached or spawned, IK), loop rule and outputs
  (records or a `WorldChange`); dev builds hot-reload job files (a running loop finishes its step, then reads the
  new def); adding a job never recompiles. The queue keeps five NPCs off one stool.
- **Light** (IN19): a slot whose verbs are snuff and relight, gated on the `Lit` state tag; one user at a time so
  two NPCs don't relight one lamp. State, rules and `LightChanged`: see [(d) World rules](interaction-world-rules.md) (IN28).

### Point index and focus

- `PointIndex` per resident cell: `SlotRef { placement: PlacementId, slot: u8 }` → world frame, kind, band, owner
  entity. A moving owner (a mount, a vehicle, a carried chair) updates its slots each tick from its body; a resting
  object updates once when it falls asleep.
- `find_slots(near: WorldPos, radius, filter: TagFilter, user) -> impl Iterator<(SlotRef, score)>` (grid buckets;
  score = advert value for the user's need × distance falloff, adapter curve). `sf_ai` calls it; the player's focus
  query (shell ray/cone, `sf_action::Focus`) gets slots and slotless verbs as targets.
- **Verbs** (decision 3, RDR2 and Cyberpunk prompts): `rank_verbs(def, state_tags, user) -> SmallVec<VerbChoice>`
  collects the slot verbs whose claim would not be refused and the slotless verbs whose `requires` matches, sorted by
  `priority` (descending), then def order. Focus shows the first; holding interact for `interact.focus.hold_s` lists
  the rest. **Stable focus** (GTA 6's prompt misfires, `docs/research/gameplay-2026-11-25/gta6.md` §2): the focused
  target changes only when another target's score leads by `interact.focus.switch_margin` for `interact.focus.switch_s`;
  while interact is held, the target and its verb list are frozen. An object with several uses (a bed: sleep, sit, pick lock) has several slots or verbs, never a special
  case.

### Claims (host only, net rule 5)

```rust
// Illustrative, not compiled.
pub struct Claims { by_slot: HashMap<SlotRef, Claim>, by_user: HashMap<UserKey, Hold>,
                    queues: HashMap<SlotRef, SmallVec<[Ticket; 4]>> }
pub struct Claim  { pub user: UserKey /* ActorKey | PlayerKey */, pub phase: Phase /* Reserved | Occupied */,
                    pub priority: u8, pub since: Tick, pub lease_until: Option<Tick> }
pub struct Ticket { pub user: UserKey, pub priority: u8, pub since: Tick, pub lease_until: Tick }
pub enum Hold { Claim(SlotRef), Ticket(SlotRef) }   // one per user
```

- **Priority** (decision 1, UE Smart Objects): each claim carries a priority from its user's reason, ordered
  player = quest > schedule > ambient (values are tunables below). `claim(slot, user, priority, tick)`:
  - Free (slot and its exclusion `group`), user filter passes, user holds nothing → **Reserved**.
  - Reserved by a **lower** priority → preempted: the old user's claim becomes a ticket at the head of the queue if the
    slot has one, else it is released with `Invalidated(Preempted)`; the new user is Reserved. Equal or higher → refused.
  - **Occupied → never preempted**, whatever the priority. A group conflict preempts only if every conflicting claim
    in the group is Reserved and lower.
  - Refusals are typed: `Taken`, `GroupTaken`, `UserBusy`, `Filter`, `Disabled`, `QueueFull`.
- **Queue** (decision 1): `enqueue(slot, user, priority, tick)` on a slot with `queue: Some(capacity)` gives a ticket
  ordered by priority (descending), then `since`, then `UserKey`; full → `QueueFull` (the AI picks another slot).
  A release or invalidation **promotes** the head ticket to Reserved with a fresh lease in the same tick. A ticket has
  its own lease (`interact.queue.ticket_lease_s`) and is cancelled when its user leaves. The core orders tickets; the
  shell places waiters at the def's approach points (or along the approach). The player never takes a ticket.
- **One body hold per user, plus hand claims**: a body claim (seat, bed, door, ladder, workstation) or a ticket; a
  second body request is `UserBusy` (the caller releases first). While Occupied in a body slot, the user may also hold
  up to `interact.claim.hand_max` **hand claims** on take/use targets within that slot's `reach`: the seated chain
  (sit, take the cup on the table, drink, put it back; RDR2 scenario props, UE Smart Objects allow several claims per
  user). A hand claim is released on put-back, and every hand claim ends with the body claim (a forced stand-up drops
  or returns the held prop per (a)).
- **Same tick**: the shell hands the core one batch per tick; the core sorts it by (priority descending, `UserKey`,
  request index), so co-op conflicts and NPC races resolve identically on every replay.
- **Lease**: a Reserved claim expires unless renewed by progress (`renew(user, tick)`: an NPC's path progress) within
  `interact.claim.lease_s`; Occupied has no lease. Leases count ticks (seconds × fixed rate).
- `occupy` on entering (holder only), `release` on the exit's end. **Invalidation** releases, emits an event and
  promotes the queue when the owner object is destroyed, moved beyond `interact.claim.move_tolerance_m` or
  `_deg`, unloaded, or its slot is disabled (a broken bed, a locked door's doorway), or when the user dies, is knocked
  down (`Takeover`) or leaves its worldspace. The user's executor then exits (a forced stand-up) or replans.
- Claims are **not saved**: on load, `sf/ai` restores the NPC's activity and the AI's promotion resolve re-claims
  (ai.md "Tiers and budget"); a seated player saves `seated: Option<SlotRef>` in `CharacterSave` and re-claims first.

### Using a slot (one path for player and NPC)

1. **Approach**: `pick_entrance(slot, user_pos, valid)` returns the nearest entrance (distance to its approach point)
   that `valid` accepts (the shell's checks: NPC nav-validated approach point, nav.md; all: the `EndPoseFree` capsule
   test at the slot's end pose), ties by side order Front, Left, Right, Back (Bottom, Top); none valid → `None`.
2. **Enter**: an `Interaction` move rule (action.md: Warp to an interaction slot): `validate: [Claim(point),
   EndPoseFree]`, warp aligns the enter clip's end onto the slot frame; the claim becomes Occupied. A ladder enters
   the `Docked` mover instead of `Seated`.
3. **Loop**: a `Seated` owner (Docked-like): root pinned to the slot frame, mover off, the capsule replaced by the
   slot's seated shape (contact-pair filter against the object, physics.md); loop clips and variants
   (motion-generation.md activities; a workstation's job steps), IK to the slot's targets (hands on armrests or
   table, feet on the floor, animation.md two-bone IK), look-at free. **Clips fit any body** (GTA 6 retargets
   interactions by body size): clips are generated on one canonical body against the slot's contact points, and IK
   fits each race and size to them, so body size needs no clip variants (side, entry and height band do). Style variants (age, build, health, mood:
   owner 2026-11-25, old people don't move like young ones) are clip sets or layers from the animation lane, not IK.
4. **Exit**: by input (player: Move, Jump or Interact; NPC: plan step done) or invalidation; `pick_exit` prefers the
   entry side, else the nearest free side by `EndPoseFree` (Bethesda's L/R rule), else the shell places the user on
   the nearest navmesh point; then release.

The NPC path is `sf_ai`'s `UseSlot(slot)` operator (claim, approach, enter, loop, exit), emitting the same `Command`
(`Activate(tag)` + target) the player's input does (net rule 2). Smart objects add method fragments per slot kind
(ai.md "Behaviours"), so new furniture adds no AI code.

### Activation contract (for DQ-6)

Every use, from player input, an NPC's `UseSlot` or a script, is one path (decision 3):

1. **Command**: `Activate { verb: TagId, target: PlacementId, slot: Option<u8> }` in the user's tick command list
   (net rule 2). Player: the focused best verb or the hold-list choice; NPC: its plan step.
2. **Validate** (host, `FixedUpdate`): target resident and in reach, verb offered by `rank_verbs` for this user.
   Refused commands emit nothing.
3. **Event**: the host writes the existing `Activated { target, actor }`
   (`superfargoth/crates/sf_script_bevy/src/lib.rs:66`); IN31 adds the verb as an optional field (`None` = the
   target's primary use, as today's callers), the script payload keeps `{ target, actor }` and gains `verb`.
4. **Scripts** run in the `Update` slot; a handler returning `"handled"` suppresses the default (`ActivationOutcome`).
   This is where DQ-6's `OnActivate` lives; whether NPC activations reach scripts is the adapter's call (the event
   carries the actor).
5. **Gates**, when not handled: registered gates run in order before the default; crime's lock and trap gate marks
   `Locked` / `TrapSprung` handled (crime.md decision 8). A refused use reports its reason to the prompt.
6. **Default**: the verb's action from this lane: pose verbs claim and run "Using a slot"; open/close toggle a door;
   pull toggles a lever; climb docks on a ladder; work runs the job loop (NPC) or the pack's menu (player);
   snuff/relight set `Lit`; carry makes the `HeldProp` link. The pack's existing classes (talk, container, book, take
   item, travel door) register as default verbs (IN32), unchanged in behaviour.
7. **Script `Activate`** (DQ-6) re-enters at step 5 for its target (gates, then default), never step 4 again, so a
   script cannot recurse into itself.

### Events (net rule 8)

`InteractionEvent { tick, user, slot, kind: Claimed | Queued | Promoted | Entered | Exited | Released |
Invalidated(reason) }` with `reason: Preempted | LeaseExpired | OwnerDestroyed | OwnerMoved | OwnerUnloaded |
SlotDisabled | UserDied | UserKnockedDown | UserLeftWorldspace`, plus `DoorChanged`, `LeverChanged`, `Destroyed`,
`Mounted`, `Dismounted` below. Scripts, audio, barks and crime read them. NPC reactions to changed objects read
`WorldChange` events ([world-reactions.md](world-reactions.md), IN14-IN16). Noise from doors, drops, throws and
breaks: see [(d) World rules](interaction-world-rules.md) (IN28).

### Tunables (tunables.md; framework values, a pack's base mod may override)

| Name | Unit | Default | Range | Source | Net |
|---|---|---|---|---|---|
| `interact.claim.priority_ambient` | count | 10 | 0-255 | Guess (order: Industry, UE Smart Objects) | Gameplay |
| `interact.claim.priority_schedule` | count | 20 | 0-255 | Guess | Gameplay |
| `interact.claim.priority_quest` | count | 30 | 0-255 | Guess (= player, roadmap decision 1) | Gameplay |
| `interact.claim.priority_player` | count | 30 | 0-255 | Guess | Gameplay |
| `interact.claim.lease_s` | s | 10 | 1-60 | Guess (spec 2026-11-11) | Gameplay |
| `interact.claim.move_tolerance_m` | m | 0.25 | 0.05-2 | Guess | Gameplay |
| `interact.claim.move_tolerance_deg` | deg | 20 | 1-90 | Guess | Gameplay |
| `interact.queue.capacity` | count | 3 | 0-16 | Guess | Gameplay |
| `interact.queue.ticket_lease_s` | s | 30 | 5-300 | Guess | Gameplay |
| `interact.focus.hold_s` | s | 0.35 | 0.1-1.5 | Guess (RDR2-style hold prompt) | Presentation |
| `interact.focus.switch_margin` | ratio | 0.2 | 0-1 | Guess (hysteresis against prompt flicker, gta6.md §2) | Presentation |
| `interact.focus.switch_s` | s | 0.15 | 0-1 | Guess | Presentation |
| `interact.claim.hand_max` | count | 2 | 0-4 | Guess (two hands) | Gameplay |
| `interact.door.swing_s` | s | 0.8 | 0.2-3 | Guess | Gameplay |
| `interact.door.close_behind_s` | s | 1.0 | 0-10 | Guess | Gameplay |
| `interact.carry.present_blend_s` | s | 0.2 | 0-1 | Guess | Presentation |
| `interact.debris.lod_m` | m | 25 | 5-100 | Guess (Industry: Chaos debris removal by distance [M]) | Presentation |
| `interact.debris.awake_max` | count | 48 | 0-256 | Guess (spec 2026-11-11) | Presentation |
| `interact.debris.breaks_per_tick` | count | 6 | 1-32 | Guess | Gameplay |
| `interact.debris.fade_s` | s | 8 | 1-60 | Guess | Presentation |

All `Scope::Dev`, `Lifetime::Live` except `interact.focus.hold_s` (`Scope::Player`, a menu row). Record data (carry
class multipliers, verb priorities, queue capacities per def, job durations) is mod data with its own source, not a
tunable.

### Cost, net, save, risk (core)

- Cost: index rebuild per cell load (hundreds of slots, < 1 ms on the loader); `find_slots` a bucket scan at AI
  decision rate (tier 2: 5-10 Hz); claims and tickets O(1) per request plus a sort of the tick's batch; door swings
  one kinematic transform each while moving. Frame cost negligible.
- Net: host owns claims, tickets, door and lever state and the index; replicated is each character's current
  activity (slot + phase, ai.md "Co-op") and object state changes as events; a client's use is a request the host
  validates; clients predict only their own enter clip; swing poses and debris are local presentation.
- Save: door, lever, health and stage in the `interaction` overlay blob; points derived, claims rebuilt.
- Risk: low. Claim-table bugs (UE forum reports of double claims) → exact tests for every transition.

## (a) Moving and carrying (player and NPC)

Moved to [interaction-carry.md](interaction-carry.md) (2026-11-25, size). In short (decision 4): one thing held;
one `HeldProp` link with two presentations (third person: kinematic attach to the hand socket + grip IK + carry
clips, player and NPC alike; first person: the built `sf_physics` spring); `CarryClass` data (speed multiplier,
`can_sprint`, `can_roll`); `HolderLimits` gate carry, drag or refuse; furniture is promoted from the static scene
to be carried and persists as moved placements; chair pull-out folded into IN9; NPC `Relocate` is an HTN fragment.

## (b) Destruction (crates first, buildings parked)

Moved to [interaction-destruction.md](interaction-destruction.md) (2026-11-25, size). In short (decision 5): tier 1
stage swap (IN1: health, material, stages, spill with damage, loot and salvage, `Destroyed` + seed, cosmetic debris
under caps and the `interact.debris.lod_m` distance LOD, baked glass shard sets); lock and door bashing by strength
(IN37; crime settles the lock); tier 2 pre-fractured chunks with Chaos-style cluster thresholds (IN12, later);
tier 3 buildings parked (IN13, owner). Runtime fracture crates rejected. Saved as one stage integer (plus door and
lever state) in the `interaction` overlay blob.

## (c) Mounts and vehicles (seats, attach, control hand-off)

Moved to [interaction-vehicles.md](interaction-vehicles.md) (2026-11-25, size; TV0b rewrites it). Seat slots whose
occupant is attached to a moving owner, control hand-off to a mount or vehicle, `SeatLink` after Veloren; based
movement on moving bodies is IN11 (travel.md Stage 3).

## Budgets (High preset, 1440p + DLSS Quality, RTX 4060 laptop, ≤ 8.3 ms median, ≤ 11 ms p99; targets to measure)

Index rebuild < 1 ms per cell on the loader (off the frame path); claims, tickets, door swings and slot follow
< 0.05 ms per tick; debris ≤ `interact.debris.awake_max` awake bodies per region and
`interact.debris.breaks_per_tick` breaks per tick, then queued, none beyond `interact.debris.lod_m` (measure the
physics step); held props and riders one attach transform each; moving platforms measured in IN11. IN44 measures
claims, debris (and (d)'s spread and burn grid) in ms on High; results go in `docs/ledger/perf.md`.

## Build slices (cheapest first; one deliverable plus its test each)

IDs and order as the lane roadmap ([interaction](../../../docs/roadmaps/interaction.md) §4, stages 1-5; docs slices
IN27-IN30 and TV0b are stage 0). IN5 is folded into IN9 (owner 2026-11-11). The first build slice creates
`docs/ledger/interaction.md`.

| Slice | Deliverable | Crate | Depends on | Verify |
|---|---|---|---|---|
| IN0 (IN0a, IN0b done) | Core, Bevy-free (`sf_ids`, `sf_coords`, `sf_world`, `sf_records`, serde; in hakari `final-excludes`): types `InteractableDef`, `SlotDef`, `SlotKind` (Pose, Door, Lever, Ladder, Workstation, Light; (c) kinds as data only), `VerbDef`, `Entrance`, `CarryDef`, `CarryClass`, `DestructibleDef`/`Stage` (types only), `InteractionEvent`; `PointIndex` + `find_slots`; `rank_verbs`; `Claims`: priority with preemption of Reserved only, queue tickets (order, capacity, promotion, ticket lease), lease + `renew`, exclusion groups, one body hold per user plus hand claims within `reach`, typed refusals, invalidation reasons, sorted same-tick batch; `pick_entrance` / `pick_exit`; the "Tunables" table declared; `interact.cook` stub (defs in = defs out) | `sf_interaction` | IN27 | headless exact: Free→Reserved→Occupied→Free and Reserved→Free (release, lease expiry; `renew` extends); only the holder occupies; higher priority preempts Reserved (old user re-queued at the head, or `Invalidated(Preempted)` without a queue), equal/lower refused, Occupied never preempted; group conflicts (and preempting across a group only when all are Reserved and lower); queue order (priority, since, `UserKey`), `QueueFull`, head promoted in the same tick on release and on each invalidation, ticket expiry; `UserBusy`; a hand claim only while Occupied and within `reach`, at most `hand_max`, all released with the body claim; a permuted same-tick batch gives the same result; each invalidation reason releases once, emits one event (move within tolerance keeps the claim); entrance nearest-valid, fallback, `None`, side-order ties; exit prefers the entry side; `rank_verbs` order and `requires` filter; defs serde round trip; tunables registered with ranges |
| IN31 | Shell: index on cell load, claims on `FixedUpdate`, focus best verb + hold list with stable focus (switch margin + dwell, frozen while held), `Activate` → `Activated` (+ verb) → gates → default | `sf_interaction_bevy` | IN0 | headless: handled suppresses the default; gate refusal; script `Activate` runs gates + default once |
| IN32 | Adapter defs (MG4 census) + each existing class's activation as a default verb, Classic switches | `mw_interaction` | IN31, IN29 | exact per class, behaviour unchanged; def counts |
| IN2 | Player sits and stands: Interaction move, `Seated` owner, side entrances, Kimodo sit clips | `sf_interaction`, `sf_action` | IN31 | exact: claim/occupy/release via `step`; sanity: pelvis on the seat, no capsule through it; owner IN-2 |
| IN3 | Beds and lean spots (lie down, the adapter's rest hook), hand IK on armrests/tables | `sf_interaction_bevy`, `mw_interaction` | IN2 | exact: rest hook fires once; owner IN-3 |
| IN33 | Swing doors (doors, containers): Open/Closed, kinematic hinge, doorway claim + queue, close behind, "Locked" from crime's `LockState`, `WorldChange` | `sf_interaction(_bevy)`, `mw_interaction` | IN31 | exact transitions; blocked swing pauses; owner IN-33 |
| IN35 | Player third-person `HeldProp`: attach, grip IK, carry class gates (speed, sprint, roll), one thing held; first-person spring kept | `sf_interaction_bevy`, `sf_physics` | IN0, MG7b | exact gates; link survives a camera switch; owner IN-35 |
| IN6 | Player moves furniture: promotion + carry/drag, slots follow, persistence | `sf_interaction_bevy` | IN2, IN35, static-scene B5a-2 | exact: overlay round trip moves slots; sanity: comes to rest; owner IN-6 |
| IN1 | Crates and barrels break, tier 1: prop targets, health, stages, spill with damage, loot/salvage, debris + LOD, `Destroyed`, overlay blob | `sf_interaction(_bevy)` | IN0, combat C2c/C3 | exact: stage math, LOD gate, save round trip, event; sanity: debris rests, no NaN; owner IN-1 |
| IN38 | Adapter breakables (crates, barrels, urns, sacks, pottery), materials, `NoiseKind` | `mw_interaction` | IN1, IN29 | exact rows on game data |
| IN4 | Impact damage (thrown items, falls) from `ContactForceEvent` thresholds | `sf_interaction_bevy` | IN1 | exact: threshold maps to damage; sanity; owner IN-4 |
| IN18 | `NoiseEvent` emitters: impacts, breaks, doors, throws, drops, bashes, lock picks (crime LW13); see [(d)](interaction-world-rules.md) | `sf_interaction_bevy` | IN4, IN33, AI8a | exact loudness per case |
| IN37 | Bash locks/doors: strength-scaled lock damage, contents risk, `Bashed`, `LockRequest::Force` + offence hook | `sf_interaction`, `mw_interaction` | IN1, IN33 | exact thresholds |
| IN14 | `WorldChange` vocabulary + placement diff ([world-reactions.md](world-reactions.md)) | `sf_interaction` | IN1, IN6, IN33 | one event per transition |
| IN15 | `ResetRules`, classes (+ `burnt`), ttl, cap, lazy reconcile, at-rest snap, holder link wins | `sf_interaction`, `sf/overlay` | IN1, IN30 | expiry, eviction, old saves load, mid-air save snaps |
| IN36 | Thrown props hit people: damage/stagger by mass × speed (capped), assault when seen, Classic off | `sf_interaction_bevy`, `sf_combat` | IN4, Q2 | exact curve; owner IN-36 |
| IN19 | `Lit`, snuff/relight verb, light follows, `LightChanged`; see [(d)](interaction-world-rules.md) | `sf_interaction(_bevy)`, `mw_interaction` | IN31 | exact events; owner IN-19 |
| IN17 | Rules core: `InteractionRules`, states + TTL, `Wet`, budgeted wind spread; see [(d)](interaction-world-rules.md) | `sf_interaction` | IN28, IN1 | exact on a synthetic grid; rain stops it |
| IN39 | Fire in game: effects (capped lights), charred stage, damage, extinguish | `sf_interaction_bevy`, `sf_effects` | IN17 | sanity; owner IN-39 |
| IN40 | Ground burn grid: groundcover/terrain tint, fast regrow | `sf_interaction_bevy` | IN17 | exact regrow; owner IN-40 |
| IN41 | Adapter: ignition sources, flammables, town rule (Q1), Classic off | `mw_interaction` | IN39, IN40 | exact rows; Classic ignites nothing |
| IN44 | Measurement: claims, spread, debris, burn grid ms, High, 4060 laptop | tests | IN39, IN40 | `#[ignore = "measurement: ..."]`; perf ledger |
| IN8 | = AI14 (AI lane builds): `UseSlot`, `find_slots`, adverts in considerations, claim priority | `sf_ai`, `sf_interaction` | IN0, IN2, IN3 | headless deterministic: AI picks, queues and releases; tier invariance; owner IN-8 |
| IN9 | NPC held props + `Relocate` (chair pull-out folded in, was IN5) | `sf_interaction`, `sf_ai` | IN8, IN35, MG7b | headless plan; sanity: prop stays in hands; owner IN-9 |
| IN16 | Loose-item `take` slots, ownership on take, `LootRules` ([world-reactions.md](world-reactions.md)) | `sf_interaction` | IN8, LW10 | ownership and ledger |
| IN42 | Workstations: NPC job loop, player menu (Q4), queue; job loader for TOML mod files (no recompile, dev hot reload) with the first jobs | `sf_interaction`, `mw_interaction` | IN8 | exact queue; a new job file loads without a rebuild; owner IN-42 |
| IN34 | Levers, buttons, switches: toggle, linked target, saved (scripted activators stay scripts) | `sf_interaction(_bevy)` | IN31, IN14 | exact; round trip |
| IN43 | Ladders: edge-chain slot, both entrances, `Docked` enter/exit, NV9 link | `sf_interaction`, `sf_controller` | IN0, `Docked`, NV9 | exact; an NPC crosses (AI33); owner IN-43 |
| (c) | IN20 (done 2026-11-25), IN11, IN21, IN7, IN23, IN24, IN22, IN25, IN26, IN10: "(c) Mounts and vehicles" and [travel](../../../docs/roadmaps/travel.md) (owner IN-7, IN-10, IN-11) | | | |
| later | IN12 pre-fractured chunks with cluster thresholds (owner IN-12); IN13 buildings (parked, owner); placement mode | | | |

Visual checks go in `bevyrrowind/docs/visual-checks/interaction.md` (prefix IN-) with camera, time, flags and what to
look for; agents never judge them from screenshots.

## As built (IN0a, 2026-11-25)

`superfargoth/crates/sf_interaction` (`def.rs`, `claims.rs`, `params.rs`, `tests_claims.rs`; 26 exact tests). Departures
from the text above, all kept: exclusion groups are **bit masks** (a plain id could not express a bench whose ends
both exclude the middle but not each other); `TagId` is local with `sf_action`'s FNV-1a hash (values match),
`TagFilter { all, any, none }`, `Tick` a local `u64`. **Invalidation by scope**: owner/slot reasons (`OwnerDestroyed`,
`OwnerMoved`, `OwnerUnloaded`, `SlotDisabled`) also end that slot's tickets and promote nobody; user reasons and lease
expiry promote the queue head the same tick. Queue edges: a preempted user facing a full queue gets
`Invalidated(Preempted)`; `enqueue` = claim else ticket (so it can preempt); a claim on a free slot whose queue head
ranks equal or higher is `Taken`. Extra refusals `NoQueue` (also for players), `NotHolder`, `NotOccupied`,
`OutOfReach`, `HandsFull`, `NoHold`. Hand claims emit no events; `Exited` is the shell's. `SlotTerms::of(def, params)`
fills default queue capacities; the shell (IN31) binds `InteractParams` from `interact.*` and `tick_hz` from the
fixed step.

## As built (IN0b, 2026-11-26)

`index.rs`, `verbs.rs`, `entrance.rs`, `cook.rs`, `tests_queries.rs` (13 exact tests; crate total 39). Departures, kept:
**one `PointIndex<O>` for all loaded cells** (border queries need no merge; the shell removes a cell's placements on
unload), owner handle `O` and focus key `T` generic (Bevy-free); `find_slots` falloff is a caller function of metres,
scores ≤ 0 dropped, an unadvertised need scores 0, no need scores 1; `rank_verbs(def, placement, state_tags,
would_claim)` (the closure carries user, tags and priority; built from the new `Claims::would_claim`, same checks as
`claim`); `pick_entrance`/`pick_exit` take the user position in the object frame (`to_local`). `FocusStabilizer`: hold
freezes only an existing target; with none (or it vanished) the best is taken at once; a challenger must beat
current × (1 + `focus_switch_margin`) for `focus_switch_s`. `cook` is identity for now.

## As built (IN31a, 2026-11-26)

`superfargoth/crates/sf_interaction_bevy` (`lib.rs`, `tests.rs`; 6 headless tests). `InteractionPlugin`; resources
`InteractionSettings`, `InteractionClaims`, `InteractionIndex` (`PointIndex<Entity>`), `InteractionTick`; messages in
`ClaimRequest(Request)`, `InvalidateClaims`, `IndexChange::{Insert, Move, Remove}`; out `ClaimChanged(InteractionEvent)`
and `ClaimAnswered` (callers need refusals back: the AI picks another slot on `QueueFull`). One `FixedUpdate` run in
`InteractionSystems::Apply`: tick + 1, index changes (sorted by placement), invalidations (sorted), one `apply_batch`,
lease `tick`, answers and events out; send order never matters. `tick_hz` from `Time<Fixed>`; 13 `interact.*`
tunables live. A claim on a slot not in the index is refused `Disabled` in the shell. **IN31a-fix (2026-11-26):** `InteractionFrames`
keeps each object's current frame and, while any slot is held, its **anchor** (the frame when first held); `Move`
measures from the anchor (small drifts add up) and calls `Claims::owner_moved`; a re-insert first ends holds on slots
it disables or no longer has (`SlotDisabled`), then treats a new frame as a move. Anchors reconcile once per tick from
the core's events (`Claims::held_slots`). Hosts write requests `.before(Apply)`, read
events `.after(Apply)`.

## As built (IN31b1, 2026-11-26)

`sf_interaction_bevy/src/activate.rs` (+ `tests_activate.rs`, 6 headless tests): the activation contract above.
`ActivateCommand { user, actor, target, verb, slot, priority, tags }` (tags: the user's, for slot `users` filters)
validated in `InteractionSystems::Activate` (after `Apply`): a target with a def (`InteractionDefs`, kept from
`Insert`) needs an offered verb (`rank_verbs` over `would_claim`; a slot the user already holds counts as offered,
so `UseSlot` can claim, approach, then activate); `verb: None` there takes the best offered verb; a def-less target
passes only with `verb: None, slot: None`. Refused commands write nothing. `Activated` (`sf_script_bevy`) gained
`verb: Option<u32>` (script payload `verb`, nil = primary use). In `Update` after the script slot,
`ActivationSystems::{Collect, Gates, Default}`: unhandled `ActivationOutcome`s from any writer plus `ScriptActivate`
(step 7, never reaches scripts) fill `PendingUses`; gates mark refusals (first wins); `Default` writes `UseApproved`
or `UseRefused { reason }`. A refusal never substitutes another verb (ai.md "No escalation ladder").
`InteractionStateTags` holds per-placement state tags (empty until IN19/IN33). Open: `UseApproved` carries no slot
yet (pose/door defaults need it: IN2/IN33); reach is the host's check.

## As built (IN31b2, 2026-11-26)

`sf_interaction_bevy/src/focus.rs` (+ `tests_focus.rs`, 6 headless tests): per-player focus. The host's ray/cone query
writes `FocusCandidates { player, candidates: (PlacementId, score), held, priority, tags }` each fixed tick;
`InteractionSystems::Focus` (fixed order `Apply` → `Focus` → `Activate`, so a host input system between the last two
is validated the same tick) keeps `PlayerFocus` (`BTreeMap<UserKey, FocusState>`, one `FocusStabilizer` each).
`FocusState { target, verbs, list_open }`: verbs come from `offered_verbs` (shared with activation validation, so
the prompt offers exactly what is accepted); while held on an unchanged target the verb list is frozen too; a player
with no message keeps its state, an empty list clears focus. `FocusState::command(player, actor, choice)` builds the
`ActivateCommand` (best verb + slot, a hold-list choice, or a def-less primary use). Game wiring: IN32.

## As built (IN2a, 2026-11-26)

Sit and stand in `sf_action` (`tests_seat.rs`, 7 exact tests via `step`). `Sense::seat: Option<SeatSense { target,
slot, enter, exit }>` (poses from the feet, world axes; the shell fills it only while this user holds the claim, and
on the same tick as the Interact press). `WarpTarget::Slot` lands on `enter` (no seat: `Refused::Slot`; over
`refuse_ratio`: `MaxCorrection`); `Owner::Seated` + `ExitOn::Stand` hold the body (root pinned via the warp output,
mover off) until the stick passes `deadzone`, Jump or Interact, or the seat sense is gone or changed (forced
stand-up); `WarpTarget::SlotExit` lands on the sensed exit, else the pose recorded at entry, and is never refused (a
seated body can't get stuck). `EventKind::SeatEntered(slot)` / `SeatExited(slot)` once each (target in the event):
IN2b turns them into `occupy` / `release`. Rules are data in `seat_rules.ron` (`SEAT_RULES`: `interact.sit.enter`,
`.loop`, `.exit`; not in `BASE_RULES`). `ActionState::seat` is `serde(skip)`: a save made while seated loads standing
(the spec's `seated: Option<SlotRef>` in `CharacterSave` is still to do, with IN2b).
**IN2b1:** the activation press is gone by the time the claim lands, so the claimed seat itself is the request: rule
entry flag `offered: true` (no request classes) starts `interact.sit.enter` on the tick `Sense::seat` turns up new
(none last tick, or another target/slot; `ActionState::seat_sensed`); if no offered rule starts that tick,
`EventKind::SeatRefused(slot)` once (not grounded, `MaxCorrection`, wrong previous move), no retry until a new
offer: the shell releases the claim. `sf_action_bevy::SeatAim(Option<SeatSense>)` is the shell's write (before
`ActionSystems::Step`).

## As built (IN2b2, 2026-11-26)

`sf_interaction_bevy/src/seat.rs` (+ `tests_seat.rs`, 5 headless tests with `SfActionPlugin`): the default for
`SlotKind::Pose`. Host component `InteractionUser { user, actor, priority, tags }` (requires `SeatAim`). `UseApproved`
(and `PendingUse`) carry `slot: Option<u8>` from validation; a slotless approval (script `Activate`) takes the best
offered pose slot. Phases `SeatUses`: Asked → Granted (claim) → Offered (`SeatAim` = slot frame and `pick_exit` exit,
from the feet; seat `TargetId` = placement id's low 64 bits) → Seated (`SeatEntered` → `Occupy`) → Leaving
(`SeatExited`/`SeatRefused` → `Release`, aim cleared). A lost claim (invalidation, lease, preemption) is read from the
claim table each tick and clears the aim at once (no Release after it, so hand claims aren't touched). Exit faces the
slot; no entrance or none free → where the feet were at the grant (entrances all blocked → claim released).
`EndPoseFree` is the host's capsule seam (accepts all by default). Latency at one fixed tick per frame: command N →
enter N+1; entered M → Occupied M+1; exited K → released K+1. Open: `interact.sit.exit` has no `stand`, so the
stand-up carries ~7 cm past the exit pose (action-layer rule data).

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

Lane questions of 2026-11-25 and their answers (fire, thrown objects, doors, workstations, decorating; jobs as data):
[interaction roadmap](../../../docs/roadmaps/interaction.md) §7.
