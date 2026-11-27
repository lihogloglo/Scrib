# Interaction (b): destruction (`sf_interaction`)

Status: design 2026-11-11, updated 2026-11-25 (IN27; moved out of [interaction.md](interaction.md) for size). Part
(b) of the interaction spec: its shared core (defs, events, tick and net rules, the `interact.debris.*` tunables),
"Budgets" and "Build slices" (IN1, IN4, IN37, IN38, IN12, IN13) stay there. Lane roadmap:
[interaction](../../../docs/roadmaps/interaction.md) decision 5. Research:
[gameplay-2026-11-25/interaction](../../../docs/research/gameplay-2026-11-25/interaction.md) §4.

<!-- agent-summary -->
> **Agent summary.** Destructible props as data on `InteractableDef`: health and stages (material on the def itself, IN17a). Tier 1 stage swap
> (IN1) with spill, loot and salvage, cosmetic seeded debris under caps and a distance LOD, baked glass shard sets;
> lock and door bashing by strength (IN37, crime settles the lock); tier 2 pre-fractured chunks with Chaos-style
> cluster thresholds (IN12, later); tier 3 buildings parked (IN13, owner). One `Destroyed` event + seed; saved as
> one stage integer in the `interaction` overlay blob.
<!-- /agent-summary -->

## Scope

Tiers (decision 5; Bethesda `DEST`, UE Chaos, Teardown; research §4): stage swap first; pre-fractured chunks later
with Chaos-style cluster thresholds; buildings parked (owner 2026-11-11). Runtime fracture crates are rejected
("Reuse check"). Owner 2026-11-11: everything breakable breaks; contents spill and take damage by hit strength;
breaking an owned object is a vandalism offence (the adapter's).

**Tier 1: stage swap** (IN1; the cheapest, saves and replicates as one integer).

```rust
// Illustrative, not compiled.
// `material: Option<TagId>` (wood, clay, glass, stone) sits on `InteractableDef` since IN17a (a banner burns
// without being destructible); `DestructionRules` reads it from there.
pub struct DestructibleDef { pub health: f32,
    pub stages: SmallVec<[Stage; 4]>, pub ignore_external: bool,
    pub loot: Option<RecordId> /* rolled on break: the pack's levelled list, adapter */,
    pub salvage: SmallVec<[(RecordId, u16); 2]> /* a barrel yields planks */ }
pub struct Stage { pub at: f32 /* health fraction */, pub model: Option<ModelRef> /* None = remove */,
    pub debris: SmallVec<[DebrisRef; 4]>, pub chunks: Option<ChunkSetRef> /* tier 2 */,
    pub effect: Option<RecordId>, pub on_break: BreakAction /* SpillContents | Loot | None */ }
```

- **Damage in**: (1) melee and projectile hits on a prop: combat's swing/contact path gains **prop targets** (the
  object's collision as a hurt volume; combat.md "Hit and hurt volumes" today targets characters only); (2) physics
  impacts: `ContactForceEvent` over a material threshold (a thrown crate breaks a jar, a fall breaks itself; IN4);
  (3) spells, explosions and fire (effects area; fire: see (d) World rules (IN28)). The adapter's `DestructionRules`
  turns (source kind, damage type, material) into damage (blunt vs blade vs fire); the framework never names a stat.
- **On a stage** (on `FixedUpdate`, host): the swap (an entity's model change; a static-scene placement is promoted
  first), debris spawned, the effect played, contents spilled onto the floor as placed items (owner), damaged by the
  overkill (a vaporised crate leaves little; curve in `DestructionRules`), the `loot` roll and `salvage` placed as
  items, `Destroyed` event. Noise of the break: see interaction.md (d) World rules (IN28).
- **Debris** is cosmetic: `DEBRIS` bodies from a per-material generic set (planks, shards) or the model's own
  pre-cut chunks; each client spawns them from the event's seed; they sleep then fade (`interact.debris.fade_s`),
  capped per region (`interact.debris.awake_max`; breaks over `interact.debris.breaks_per_tick` queue to the next
  tick). **Distance LOD**: beyond `interact.debris.lod_m` from that client's camera, no bodies spawn, only the stage
  swap and the effect (the Chaos "debris removal" practice). Never saved, never gameplay.
- **Glass** (windows, bottles) uses **baked shard sets**: a few offline Voronoi shard sets per size class, picked by
  the event's seed; no runtime fracture.

**Bashing locks and doors** (IN37; owner 2026-11-25: by strength with weapons or fists, risking health and
contents, a crime if reported). Doors are bashed open, not destroyed. A bash is a verb on a locked target: the
adapter turns the holder's strength and weapon into a blow force; the core computes lock damage (force × the
target's material factor, a `BashRules` record), the risk to breakable contents per blow and the attacker's own
damage (fists), and emits `Bashed { tick, placement, force, seed }`. Crime's host reads `Bashed` and sends its own
`LockRequest::Force` (the dependency is one-way, crime on interaction); crime settles `hp`, `broken` and the offence
(crime.md decision 8; vandalism if owned). Noise: see (d) (IN28). Design: "Bashing (IN37)" below.

**Tier 2: pre-fractured chunks** (IN12, later): an offline-cut chunk set per model (Blender Cell Fracture or a
parry/CSG bake tool) organised as **clusters with damage thresholds** (UE Chaos geometry collections): at the break
only the top cluster releases its children as bodies, the rest stay welded (one compound body) until a later hit
passes their level's threshold, which keeps collision counts low. The impact impulse is applied at the hit point.
Chunks are debris (same LOD, cap and seed); the saved state is still the stage integer.

**Tier 3: buildings** (IN13, **parked**, owner 2026-11-11). Open-world buildings are huge statics in the static
scene and interior cells behind doors; structural collapse (Red Faction, The Finals) needs a structural graph, nav
patches and interior consistency, a hard problem (`bevyrrowind/docs/hard-problems.md`). Owner direction: the outer
shell fractures and the linked interior's contents spawn and are destroyed too; it needs its own brainstorm. Towns
burn only through authored stages (owner 2026-11-25). The known cheap route is **authored damage states** (quest or
siege set pieces, a breachable wall) as stage swaps with large debris and an authored nav link switch (nav.md link
flags), which is how most open worlds ship it; Teardown confirms the network rule (event + seed, not data).

**Dependencies**: combat prop targets (sf_combat C2c/C3), physics contact events (sf_physics), effects (sf_effects),
container contents (sf_rpg inventory), crime `LockState` (IN37), static-scene promotion for statics (crates and
barrels are containers, so entities already: no scene work for tier 1). **Cost**: a break is one swap plus N debris
bodies within the caps above, zero bodies beyond the LOD distance; measured on the 4060 laptop in IN44 (ms per
preset). **Nav**: a removed crate frees space (no carving default); a breached wall flips an authored link.
**Net**: host-authoritative; `Destroyed { tick, placement, stage, point, dir, seed }` (plus culprit, overkill, removes: "Core additions" below); clients swap and spawn debris
locally (seeded, own LOD). **Save**: an overlay blob `interaction` v1 per placement: health, stage, door state and
lever position (save.md "Overlay blobs", blob version table); spilled, looted and salvaged items are overlay spawned
placements; a removed object is an overlay removal; reset classes per [world-reactions.md](world-reactions.md) (IN15).
**Risk**: low for tier 1; medium for chunks (content); high for structures.


## Shell and game (IN1b, IN1c, IN38b; spec IN1-S 2026-11-27, review IN1-REV 2026-11-27)

Pattern: the light shell (`sf_interaction_bevy/src/light.rs`, IN19b) and its game hook (`mw_interaction_bevy`
`light.rs`, IN19c). The shell owns state and events; every model, item, sound and save byte is the host's.

**Core additions** (IN1b, `sf_interaction` `destroy.rs`; IN1a has no event type yet). `Destroyed { tick: Tick,
placement, stage: u8, point: Vec3, dir: Vec3, seed: u64, culprit: Option<UserKey>, overkill: f32, removes: bool }`,
the core event the shell wraps like `DoorChanged` / `LightChanged` (`pub struct Destroyed(pub
sf_interaction::Destroyed)`); "Net" above names its first six fields. `break_seed(placement, tick,
stage) -> u64`: `h = 0; for w in [placement.0 >> 64, placement.0 as u64, tick.0, stage] { h = splitmix64(h ^ w) }`
(`PlacementId` is a `u128`; splitmix64 is a few lines, no hash crate). Pure, so the seed is tested exactly in core.

**Registry** (IN1b, `sf_interaction_bevy` `destroy.rs`). `Destructibles(BTreeMap<PlacementId, Destructible { def:
Arc<InteractableDef>, object: Frame, state: DestructState, gone: bool }>)` (`object` is for area sources: fire and
explosion radii, IN17), kept by `destroy::register` in the `InteractionSystems::Apply` chain after `light::register`
(same tick, send order): an `IndexChange::Insert` whose def has `destructible` registers it (`DestructState::intact`);
a re-insert keeps the state, clamped to the new def (a stage swap re-sends the broken model's def, which keeps the same
`destructible`); an insert without one, or a `Remove`, drops it; `Move` refreshes `object`. Then, after all index
changes, `DestructRest { placement, state }` (a restored save or a streamed-in cell) sets the state clamped to the
def (stage <= `stages.len()`, health in `0..=def.health`; `gone` if a reached stage removes); no event: the host that
wrote it shows its own stage.

**Damage in.** One message any source writes, ordered `.before(DestroySystems::Damage)`:

```rust
// Illustrative, not compiled.
#[derive(Message)] pub struct PropDamage { pub placement: PlacementId, pub amount: f32, pub kind: TagId,
    pub point: Vec3 /* world m: interaction frames, seat.rs "Space" */, pub dir: Vec3, pub culprit: Option<UserKey> }
```

Writers: combat C14 prop targets (later), impacts IN4, rules/fire IN17, the dev source (IN38b1). `kind` is the
source's damage tag (adapter data: blunt, blade, arrow, hand, fire, impact; combat C14 folds its damage type and hit
class into it, combat-adapter.md "Prop targets"); the shell scales `amount` by `DestructionRules` (a resource the
adapter fills: `(kind, material) -> factor`, default 1; the framework never names a stat), so IN4 and IN17 write raw
amounts. Dropped: unknown placement, `gone`, factor <= 0, and every kind but `DAMAGE_SCRIPT` (`TagId::of("script")`)
on an `ignore_external` def (Bethesda's flag: only a quest breaks it).

**Order** (`FixedUpdate`, host, `DestroySystems::{Damage, Release}` chained after `InteractionSystems::Apply`):

1. **Damage**: the tick's `PropDamage`s sorted by a total key (placement, then kind, culprit, `amount`, `point` and
   `dir` bits): several writer systems run unordered under the multi-threaded executor, so send order is not
   deterministic (the claim step sorts its index edits the same way). Each -> `apply_damage` on the current state at
   once (health is authoritative now: two same-tick hits add, a stage is never crossed twice); every crossed stage
   pushes one pending break `{ hit tick, placement, stage, point, dir, culprit, overkill (last crossed only, else 0),
   removes }` into one `BreakQueue`; a crossed stage whose `model` is `None` sets `gone` (later hits drop). One
   `DestructChanged { placement, state }` per placement whose state changed this tick (its last state): the host's
   save write.
2. **Release**: `take_tick(interact.debris.breaks_per_tick)` -> one `Destroyed` each, FIFO (a multi-stage hit stays in
   def order across ticks). `tick` is the hit's tick, so `seed = break_seed(placement, tick, stage)` is the same on
   host and clients whatever the queue did. A queued break whose placement left the registry meanwhile (its cell
   unloaded) still releases: health was saved at the hit, so the hook applies it to the overlay alone. A released
   break with `removes` on a registered placement also writes `IndexChange::Remove { placement, reason:
   OwnerDestroyed }`: claims on it end in the next tick's `Apply` (one tick, like any host `Remove` written after
   `Apply`; a sitter stands, a carrier drops).

The cap delays presentation and removal, never health: the state saved is always the true one.

**Game wiring** (IN38b1, `mw_interaction_bevy` `breakable.rs`): the index hook attaches IN38a's `destructible_def` to
a breakable placement's def (the registry then registers it from the `Insert`), fills `DestructionRules`, writes
`DestructState::rest_bytes` under `DESTRUCT_STATE_OWNER` per `DestructChanged` (a removing state writes none), and on
activation sends `DestructRest` after the `Insert` from the overlay's bytes.

**Host stage hook** (IN38b2, `mw_rpg_bevy` `breakable.rs`: the crate that owns container contents and
`take_placement`, and depends on `sf_interaction_bevy` already), ordered `.after(DestroySystems::Release)`, on
`Destroyed`, by `def.stages[stage]`: a model swap re-sends `IndexChange::Insert` with the broken def; `None` removes the
placement live (deactivation hooks, overlay removal, despawned visuals); `on_break` spills the contents as placed items
keeping their owner, damaged by `overkill` through the adapter's curve, rolls `loot`, places `salvage`; plays
`effect`. **Gap (IN1-REV):** the game's only live removal, `take_placement` (`mw_rpg_bevy` `inventory.rs:310`), runs
through `ActivePlacements` (`mw_scripts_bevy` `lib.rs:233`), which holds only `Dynamic` and `Light` placements: a
`CONTAINER` (`PlacementClass::Custom(1)`, `mw_world` `source.rs:25`) has no live removal yet, so IN38b2 first adds
`CONTAINER` to `ActivePlacements` and a removal that despawns its spawned entity. MISC pottery is `Dynamic` already.
Decorative `STAT` urns are static-scene instances: their removal is its own row (IN38c, after IN38b2). **Split
(IN38b2, 2026-11-27):** the spill, loot, salvage and effect are IN38b3: a live item into a loaded cell goes through
`Streaming::spawn_into` (`sf_world_bevy` `stream.rs`), which refuses a placement whose model is not loaded and starts
no load, so spilled items would show only at the cell's next fetch; the model load comes first. Until then a broken
container's store stays keyed by its placement (nothing lost). The
deactivation hook (`mw_interaction_bevy` `index.rs:91`) writes `IndexChange::unload` too: ordered after `Release`, the
shell's `OwnerDestroyed` is sent first and wins (the step sorts stable by placement; the second `Remove` finds nothing).
Crime's vandalism hook and noise (IN28) read `Destroyed` too. Classic rules: no destructible defs, nothing runs.

**Dev source** (IN38b1, owner check IN-38 before C14 (IN-1 was the keymap check, now KM-1)): behind the game's `dev` feature, a dev key writes one
`PropDamage { amount: def.health, kind: blunt }` at the player's `PlayerFocus` placement, so one press runs every
stage of a one-stage crate; the build slice greps the game's dev key table for a free key.

**Debris** (IN1c, `sf_interaction_bevy` `debris.rs` behind a new `physics` feature, `sf_physics_bevy` an optional
dependency: the `sf_audio_bevy` `physics` pattern. `sf_physics_bevy` brings `sf_terrain_bevy`, `sf_world_bevy`,
`bevy_camera` and `bevy_asset`, which the shell's claims, doors and lights don't need; the feature is set once on the
root `[workspace.dependencies]` entry, and IN1c runs `tools/bevy_variants.sh` plus the selection `-p
sf_interaction_bevy --features physics`, all plain ones in group [1]). Every client (host included) runs it on
`Destroyed`; never gameplay, never saved:

- **Gate**: `debris_lod(distance from the client's viewpoint, interact.debris.lod_m)`; the viewpoint is
  `PhysicsFocus` (the streaming anchor, a few metres from a 3P camera against a 25 m default; no camera dependency);
  beyond it no bodies.
- **Pieces**: `DebrisSets` (host resource: debris record -> pieces `{ shape: ShapeRef, local: Frame, mass }`) for
  each of the stage's `debris` records; a seeded stream from `seed` picks the impulse jitter around `dir` at `point`,
  so the same event spawns the same set. Each piece: `Physics::insert(ws, &BodyDesc { mass, damping: ITEM_DAMPING,
  ..BodyDesc::new(BodyKind::Dynamic { asleep: false }, pose, shape, Layer::DEBRIS) })`, then
  `PhysicsWorld::apply_impulse`, on an entity with `DebrisPiece { set, piece }` and `PhysicsBody { worldspace, id }`;
  positions convert with `WorldPos::from_f64` (as seat.rs `world_pos`). The host's visual hook gives it a mesh on
  `Added<DebrisPiece>` (the framework never loads a model).
- **Cap**: at most `interact.debris.awake_max` pieces alive in the client's active space; spawning past it starts the
  fade of the oldest first (the newest break is where the player looks), never refuses a break.
- **Fade**: a piece starts fading when `PhysicsWorld::is_sleeping` or `interact.debris.fade_s` after spawn, whichever
  first, then scales to zero over `interact.debris.shrink_s` and despawns (its body goes with the component).

**Reuse check.** Bevy 0.20: buffered `Message`s (send order, readable from `FixedUpdate`, several readers) carry
every step, as `IndexChange` and `LightChanged` do; observers and `EntityEvent` need a target entity, and a static
placement has none before promotion, so they don't fit. Crates: physics is `sf_physics` (rapier) already; avian or a
second rapier for debris would be a second world; runtime fracture crates stay rejected (tier 1 needs none).

**Industry comparison.** Bethesda `DEST` (Creation Kit): stages by health percent, a model per stage, a debris form
plus count spawned at the stage, "ignore external damage", and self damage per second to cook an object off [1]:
match, except self DPS, which our fire rules (IN17c) cover. UE Chaos: pieces are removed on sleep (sleep min/max
timers) and shrink on removal (`bScaleOnRemoval`, default on) [2]; it has no global piece cap (a game scripts its
own), so ours adds the cap with oldest-first fade. RAGE (GTA V, RDR2) fragments (`.yft`): a drawable plus physics
groups linked to bones that break off as separate parts, sometimes a whole destroyed-variant drawable (gas tanks) [3]:
the variant is our stage swap, the groups our tier 2 clusters (IN12); their break thresholds and RDR2's own data: U.

[1] https://ck.uesp.net/wiki/Destructible_Objects , https://falloutck.uesp.net/wiki/Destructible_Object_Data
[2] https://dev.epicgames.com/documentation/unreal-engine/API/Runtime/GeometryCollectionEngine/UGeometryCollection
(U: API summary via search, page not read in full)
[3] https://docs.sollumz.org/documentation/fragments.yft (modding docs; U: search summary, page not read in full)

**Tunables added**: `interact.debris.shrink_s` (s, 0.5, 0.1-3, Guess; Industry: Chaos scale on removal [2];
Presentation), declared by IN1c with `awake_max` and `fade_s` (already in interaction.md's table).

**Tests.** IN1b: core `break_seed` exact (fixed values, differs per stage and tick); headless exact: one hit crosses
two stages -> two `Destroyed` in def order, seeds equal across two runs; more breaks than the cap -> the rest next
tick, `tick` still the hit's; the same hits from two writer systems in either order -> the same `Destroyed` list; a
hit on a `gone` placement and an `ignore_external` non-script hit drop; the `DestructionRules` factor scales;
`DestructRest` gives no event and clamps; a removing stage writes `Remove { OwnerDestroyed }` and the sitter's claim
ends next tick; a queued break of an unloaded placement releases without a `Remove`; one `DestructChanged` per
placement per tick. IN1c (headless, `--features physics`): same seed same piece set and impulses; the cap holds
(oldest fades first); beyond LOD no bodies; all despawned after fade + shrink; sanity: no NaN, pieces rest. IN38b1
(headless exact): a breakable placement registers, rest bytes round trip through the overlay, the dev key damages the
focused placement. IN38b2 (headless exact): the hook removes a container once, spill count and owners, an unloaded
placement's break removes it from the overlay; owner scene IN-38. IN38b3a-c: see "Spill".

## Spill (IN38b3; spec IN38b3-S 2026-11-27)

Owner 2026-11-11 (interaction.md "Owner answers" 1): contents spill on the floor and take damage scaled by the hit's
strength, "a vaporised crate leaves little". The Morrowind table has no `loot` and no `salvage` (IN38a: the contents
are the loot) and the break effect is IN1c's debris, so IN38b3 is the spill only: three rows, one deliverable each.

**IN38b3a, framework: a queued spawn that waits for its model** (`sf_world_bevy` `stream.rs`, additive: the NPC-AI
lane's `spawn_into` (AI10c3c1, `stream.rs:727`, used by `mw_walkers_bevy` `bodiless.rs`) keeps its signature and
refusals). `Streaming::spawn_when_ready(ws, cell, placement) -> Queued | Refused`: refused like `spawn_into` (cell
not resident, id already covered or listed, no spawner for the class) except for a model that is not loaded; queued
otherwise, taking one model user at once (`acquire_model`, so the load starts now). `drive_streaming` drains the
queue in order, at most `stream.pending_spawns_per_frame` spawns a frame (count, 8, 1-64, Guess; standard: spawn work
time-sliced per frame, U for engine numbers): an entry whose cell is in flight waits; one whose model is ready spawns
through `spawn_into`'s path, its queued user becoming the near group's (one user per spawned placement, released with
the group); one whose cell left residency, whose model failed (warn) or whose id the cell's own fetch already spawned
is dropped and releases its user. The queue is never saved: the overlay is the truth and the cell's next load spawns
what was dropped. Textures load behind the spawn (`prepare` is not waited on), as for any late Bevy entity. Industry:
data first, 3D when loaded: UE spawns an actor after an async load of its soft-referenced assets
(`FStreamableManager`); Bethesda creates the reference at once and attaches its 3D when the model is in (`PlaceAtMe`,
engine internals U); OpenMW's `placeObject` loads the model on the calling frame (a hitch its resource cache keeps
small). Test (headless exact, new `tests_pending.rs`, not `tests.rs`: the NPC-AI lane edits it): a model not loaded ->
`Queued`, no entity, the model's users +1; once it settles -> exactly one entity, listed in an Active cell; the cell
unloads first -> dropped, users back; the fetch got it first -> dropped, one entity; the per-frame cap holds.

**IN38b3b, framework: an item starts moving when its overlay says so** (`sf_physics_bevy` `stream.rs`). The `physics`
state blob (`STATE_VERSION` 1: asleep flag + linear and angular velocity, `stream.rs:112`) is written on deactivation
(`stream.rs:415`) and read by nobody: an item that left a cell mid-flight comes back asleep where it was. The blob gets
a public encoder/decoder (`ItemMotion { asleep, linear, angular }`, used by that writer) and item activation reads it:
`asleep == false` inserts the body awake with those velocities and never held (`ItemBody::hold` is for resting items);
no blob, or asleep -> as now. Industry: a moving reference's motion state is saved with it (Bethesda/Havok: U); UE
restores a saved actor's velocity only through game code. Test (headless exact): an awake blob with a downward
velocity -> an awake body with that velocity; no blob -> asleep (unchanged); the writer's blob decodes to its values.

**IN38b3c, game: the spill** (`mw_rpg_bevy` `breakable.rs` `apply_breaks`; the pure maths in `mw_interaction`
`breakable.rs`, tunables in its `BreakParams`). On a removing `Destroyed` whose def's `on_break` is `SpillContents`:
1. Capture the container's live placement (record, transform, worldspace, cell) before `remove_broken`, seed the store
   if never opened (`container_store` at the player's level: an unopened crate spills its seeded roll), and take it out
   of `RpgState::inventories` (nothing stays keyed by a gone placement). `Gone` spills nothing; `Unloaded` spills into
   the overlay only (its next load spawns the items); no transform found -> the store stays keyed by the dead id
   (nothing lost, one warn).
2. Per stack, in store order, draws in [0, 1) from `splitmix64(seed, stack index, k)` (seed = the event's): `r =
   overkill` (the event's, already damage past zero over `def.health`); lost fraction `f = loss_max * clamp((r - loss_knee) / loss_span, 0, 1)`; kept `= count -
   floor(count * f + u)` (stochastic rounding: a single item is lost with probability `f`, iff `u >= 1 - f`); gold never loses count (coins
   scatter, they do not break). Items with a health (`WEAP` with `weapon_has_health`, `ARMO`: the `mw_rpg` `barter.rs`
   rule) lose `ceil(min(r * health_per_overkill, health_max) * max)` from `item_health`, down to 0 (broken, still
   repairable, as in Morrowind), written as `ATTR_HEALTH`. A restocking stack spills its magnitude once and drops
   `ATTR_RESTOCK` (a loose item does not restock).
3. One spawned placement per kept stack (`PlacementOverlay::spawn`: a fresh `sf.spawned` id, saved with the overlay):
   base, model and class from `mw_world::resolve_base`, at the container's position plus a scatter inside its former
   footprint (`radius_m * sqrt(u2)` at angle `2π u3`, lifted `lift_m + i * layer_m` so stacks do not overlap; the crate
   is gone, so its volume is free), random yaw; each item's stream cell from its own position. Its blobs: `physics`
   (IN38b3b) awake with zero velocity, so it falls and settles; `mw.loose` v1 = `[1, count u32 LE, owned u8, health
   i32 LE (-1 = full)]`, owned = the container's `RefOwners` flag (owner identity comes with crime, IN28, for every
   reference at once, as blob v2). Live: `spawn_when_ready` into the item's cell.
4. Take (`apply_takes`): a loose item with a `mw.loose` blob gives its count and health attr instead of `RefCounts` and
   empty attrs; the overlay forgets the blob with the spawned placement (`PlacementOverlay::remove`).

Tunables (adapter, `mw.rules.interaction.break_spill_*`, all Guess, set from the owner's "a vaporised crate leaves
little"): `loss_knee` (ratio, 0.5, 0-4: an ordinary smash loses nothing), `loss_span` (ratio, 2.0, 0.1-8), `loss_max`
(fraction, 0.9, 0-1), `health_per_overkill` (fraction per ratio, 0.5, 0-2), `health_max` (fraction, 0.75, 0-1),
`radius_m` (m, 0.25, 0-1), `lift_m` (m, 0.3, 0-1), `layer_m` (m, 0.05, 0-0.3). Industry: BG3 drops a broken
container's contents on the ground (community reports, no official doc: U) [4]; OpenMW `dropObjectOnGround` copies the
object into the cell, and its drop path clears the owner because the item was the player's [5]; ours keeps the
container's owner: the items were someone's, so taking them is theft (crime IN28). Classic rules: no destructible defs,
nothing spills. Test (headless exact, `tests_breakable.rs`): a seeded crate with gold, a counted stack and a weapon,
broken at overkill 0 -> every stack spilled whole, health full, owned flag copied, one spawned placement each in the
overlay and live once the model settles; at a vaporising overkill -> the kept counts and the weapon's health the
formula gives for the fixed seed, gold whole; taking a spilled stack gives its count and health; an unloaded container
spills into the overlay only.

[4] https://steamcommunity.com/app/1086940/discussions/0/3812913197997418205 (U: forum, search summary)
[5] https://git.cijber.net/mirror/openmw/commit/5a31320d8d3f7c71d4b71e54b41a4281d2403a05 (OpenMW mirror, "preserve
refnum on drop"; U: search summary of the diff)

## Static removal (IN38c; spec IN38c-S 2026-11-27)

Decorative `STAT` urns and pots (IN38a's `Source::Static` breakables, counted at startup since IN38b1) are scriptless
and unreferenced, so both batched paths take them: the static scene (default) draws them as instance slots with no
entity, the near precombine (`--off scene`, the A/B arm) merges them into a cell mesh. Their collider is not batched:
`PhysicsHook::activate` (`sf_physics_bevy` `stream.rs:395`) queues a `Fixed` body per placement on activation, which
covered placements still get (`CellScene::covered` docs, `sf_world_bevy` `scene.rs`), and `deactivate` removes it.
Persistence is solved: an overlay-removed placement is not drawn at the next fetch (scene B5b, `sf_scene_bevy`
`residency.rs:986`; precombine `valid_members` drops the whole merge and spawns per placement). The live half is missing.

**Industry: instance buffers remove an instance, merged meshes exclude what can change.** UE's instanced static meshes
remove one instance (`UInstancedStaticMeshComponent`, `EInstanceDeletionReason`), and its Instanced Actors plugin turns
an instance into its full actor on interaction (`UInstancedActorsData::ActorClass`, hydration internals U) [6]; our scene
already has that swap (`PromoteToEntity`, scene-runtime-plan.md §7). Bethesda's precombined meshes cannot drop a member:
editing or disabling a precombined reference breaks the cell's precombine, and the Creation Kit leaves references that
may change out of the merge (disabled refs; scrap-recipe statics reportedly; community knowledge, U) [7]. So: hide the
instance in the scene, keep breakables out of the precombine at bake time.

**IN38c1, framework: hide a scene-drawn placement** (`sf_world_bevy` `stream.rs` + `scene.rs`, `sf_scene_bevy`
`residency.rs`). `Streaming::hide_covered(&mut self, world: &mut World, ws, cell, id) -> bool`: true when the cell's
near group draws `id` through its scene and has not promoted it: the id joins the group's `promoted` set (a later
`PromoteToEntity` is refused as "already promoted"; one way, dies with the group like a promotion) and
`CellScene::hidden(world, ws, cell, id)` runs; false and nothing changed otherwise (cell not resident or not at Near,
a precombine member, not covered, already promoted or hidden). `CellScene::hidden` defaults to nothing; `SceneHook`
writes the same `PromotePlacement` as `promoted` (every table instance of the placement inactive, all its LOD chains;
`scene/promoted` counts both). Removal needs no model and no entity, so it is immediate, unlike a promotion. Test
(headless exact, `sf_scene_bevy` `residency_tests.rs` with its fixture): hiding a covered placement queues one inactive
write per instance and returns true; a second hide and a later promotion of it change nothing; a placement the scene
does not draw, or a cell not resident, returns false with no write; release then re-acquire draws it again unless the
overlay removed it (B5b).

**IN38c2, game: live removal of a breakable static** (`mw_interaction_bevy` `breakable.rs`, `mw_scripts_bevy` `lib.rs`,
`mw_rpg_bevy` `breakable.rs`, host `mw_exterior_bevy` `exterior.rs`). `BreakablesTable` lists the `Source::Static`
breakables like `CONT`/`MISC` (the index hook already wraps `PlacementClass::Static`, `index.rs:134`, so the def attaches
and the registry registers them). `mw_scripts_bevy` gains `LiveStatics(HashSet<record id>)` (a resource the host fills
from the table; the rule switch is live, so under Classic rules the list stays but is inert: no def attaches, nothing
breaks, an entity-drawn urn only gains its root): a `Static` placement whose base is listed joins `ActivePlacements` and,
when it spawns as an entity (scene off, a failed chain, the entity parts of a `Split`, a promotion), spawns under a
`LiveRoot` like a container; no other static changes (thousands per active ring stay out of both). `remove_broken`'s
`Live` arm for a static: `ActivePlacements::remove_live` (deactivation hooks: the collider goes; overlay removal; roots),
then `Streaming::hide_covered` with the entry's worldspace and cell (false for an entity-drawn one, already despawned).
A static breaks with its def's debris (IN1c) and spills nothing (IN38a: spill is for containers). Test (headless
exact, `mw_rpg_bevy` `tests_breakable.rs`): a listed static, activated and scene-covered, broken -> its body gone,
overlay-removed, `hide_covered` called once (a recording `CellScene`); the same static spawned as an entity -> its root
despawned; an unlisted static is not in `ActivePlacements`.

**IN38c3, game: breakables out of the precombine** (`mw_bake` `far.rs` `MwFarSource::precombine_exclude`): also true
for a `Source::Static` breakable (`mw_interaction::breakable::classify` over the same facts the table uses; `mw_bake`
already depends on `mw_interaction`), whatever the runtime rule switch (the bake is per install, not per save: under
Classic rules they are plain per-placement statics in the A/B arm, a few entities a cell). The fingerprint takes a
version bump so `lod.near` products re-bake once. The scene's `is_referenced` stays as is (it hides live). Test (exact,
`mw_bake` tests): `build_near` over a synthetic cell with an urn and a wall keeps the wall a member and the urn out;
the fingerprint differs from the previous version's.

**Known limits.** The far field (L1 at the 400 m handoff) draws baked proxies: a removed urn may still show in its L1
node once its cell drops to far, if the node kept an object that small (L1 policy and size cull, U: not measured);
Bethesda's and UE's HLOD bakes have the same staleness (U). MWScript `Disable` on statics needs nothing here: referenced
statics never enter either batch (`mw_bake` `scene.rs:261`, `far.rs:116`).

[6] https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/InstancedActors/UInstancedActorsData (U: API
listing via search summary); https://dev.epicgames.com/documentation/unreal-engine/instanced-static-mesh-component-in-unreal-engine
[7] https://falloutck.uesp.net/wiki/Precombined_References; https://forums.nexusmods.com/topic/5446017-precombined-meshes-what-destroys-them
(U: search summaries, community knowledge)

## Impacts (IN4; spec IN4-S 2026-11-27, review IN4-REV 2026-11-27)

"Damage in" (2): a thrown jar breaks on a wall, a crate dropped on a jar breaks it, a bottle knocked off a counter may
break itself. Host only (clients get `Destroyed`); damage only, no noise (IN18b reads the same contacts itself).

**Trigger: `PhysicsEvent::Contact`, not `ContactForce`.** `ContactForce` (`sf_physics` `body.rs:144`, `world.rs:696`)
is rapier's contact-force event: it fires every step the pair's summed force passes `contact_force_threshold` (200 N
default, `body.rs:240`) [8], so a 30 kg crate resting on a floor (294 N) reports each step it is awake, and the force
spreads over steps for soft hits (the PH-10 lesson, `sf_physics_bevy` `character.rs:348`). `Contact` (`body.rs:171`)
is the pair's solver normal `impulse` (N·s) this step plus `speed`, the normal velocity change it caused (`impulse ×
(1/m_a + 1/m_b)`, fixed side infinite mass), reported only for awake dynamic `Layer::MOVING` bodies at `speed` or
`slide` >= 0.3 m/s, never for bodies still settling, each pair once (`world.rs` `contact_reports`). A resting or
sliding body changes its normal velocity by `g·dt` (0.16 m/s at 60 Hz) a step, so a speed gate removes it whatever its
mass. It is the event impact audio already reads (`sf_audio_bevy` `physics.rs:31`). Measured: a ball dropped 2 m
(6.3 m/s) reports a first `speed` in 5.5-7.5 m/s, then goes quiet within 30 steps (`sf_physics` `contact_tests.rs:46`;
bodies have no restitution by default, so `speed` is the approach speed). A speculative contact can leave a small
remainder of one landing to the next step: below the speed gate it is dropped, so a hit's damage is that of its main
step (the dead zone is per event, a known small loss).

**Core** (IN4a, `sf_interaction` `impact.rs`, pure): `impact_damage(impulse_ns, speed_mps, &InteractParams) -> f32`
= `interact.impact.damage_per_ns × (impulse_ns − interact.impact.impulse_min_ns)` when `speed_mps >=
interact.impact.speed_min_mps` and the impulse passes the dead zone, else 0; non-finite input -> 0. `DAMAGE_IMPACT =
TagId::of("impact")` beside `DAMAGE_SCRIPT` (`destroy.rs:37`). **Both bodies take the same raw amount** (Newton's third
law: one impulse acts on both): fragility is per material and per health, not per mass, so the jar's 1-2 hp and the
crate's 15 hp (`mw_interaction` `tunables.rs:58-68`) do the split, and the shell's `DestructionRules` `(impact,
material)` factor scales it (stone 0, glass high: the adapter's data). A fixed side (floor, wall, a static crate) has
no velocity change but the impulse is still real, so a jar thrown at a static crate chips the crate: impulse, not
per-body Δv, is why a fixed target works. Impulse vs energy: Chaos compares collision strain (from impulses) with each
cluster's damage threshold, scaled per physical material [9]; Source scales impact damage per prop
(`physdamagescale`), ignores hits under a minimum (`minhealthdmg`), and its impact tables (`physics_impact_damage.cpp`,
damage to players and NPCs from collision energy) have a minimum linear speed and a 2 kg floor [10]; Source's measure is
energy, ours impulse (review IN4-REV: impulse is what the solver reports and what a fixed target feels). Our three tunables are those three knobs (scale, dead zone, speed floor); the mass floor is
not needed (a light body's impulse is small already).

**Shell** (IN4b, `sf_interaction_bevy` `impact.rs`, `physics` feature, `FixedUpdate`, `.after(PhysicsSystems::Step)
.before(DestroySystems::Damage)`; no ordering cycle: nothing orders `Damage` before `Step`; in the game `Step` already runs after `Apply` through
`DoorSystems::Commit` -> `door_body::move_bodies` (`mw_interaction_bevy` `lib.rs:187`), and debris and the game's
`apply_breaks` (`mw_rpg_bevy` `breakable.rs:82`) run after `Release`, so they stay after `Step` too):

- **This step's events only.** `PhysicsEvents` (`sf_physics_bevy` `lib.rs:168`) holds the whole frame's steps and is
  cleared in `First`; its readers today run in `Update`. The shell keeps an `ImpactCursor(usize)` resource reset to 0
  in `First` and reads `events.0[cursor..]`, then sets it to `len`: two steps in one frame never read the same event.
- **Body -> placement.** `PhysicsStreaming` maps placement -> body only (`stream.rs:289` `body(ws, id)`); the shell
  builds a `BTreeMap<BodyId, PlacementId>` from `Destructibles` (not `gone`) and `ActiveWorldspace` on a step with at
  least one gated contact (the destructibles of loaded cells, a few hundred: negligible), so no change in
  `sf_physics_bevy`. A body on neither side registered -> nothing.
- **Filters.** A pair where either side is a debris-shell piece (an entity with `DebrisPiece` and its `PhysicsBody`,
  `debris.rs:71`: cosmetic, never gameplay), on `PROJECTILE` (combat's hit path, C14, would count it twice) or on
  `HELD` (the carry spring pressing into a wall is a push, not an impact; a released body is back on its own layer,
  `sf_physics` `carry.rs:482`) gives no damage (`PhysicsWorld::layer`, `world.rs:410`). **Not the `DEBRIS` layer:**
  streaming puts every loose item under `debris_mass` 15 kg and `debris_extent` 0.6 m on `DEBRIS` (`sf_physics_bevy`
  `stream.rs:565-570`, `lib.rs:111`), so the bottles and pots this section is for are `DEBRIS` bodies (review
  IN4-REV: the spec's first layer filter would have dropped them all). `RAGDOLL` and `POWERED` count (a body falling
  onto a jar). Character pushes are impulses outside the solver (`sf_physics` `character.rs:12`, capsule inset), so
  walking into a jar is no contact with the capsule; a jar pushed at run speed into a wall is a real wall impact
  (about 5 m/s x 0.4 kg = 2 N·s, 0.75 at the defaults: a glass item may chip, not break), tuned by the gate.
- **Out.** Per registered side, `PropDamage { placement, amount: impact_damage(impulse, speed), kind: DAMAGE_IMPACT,
  point, dir, culprit: None }`, `point` the event's `WorldPos` in interaction frames (`point.to_f64().as_vec3()`, the
  inverse of `debris.rs:143` `world_pos`), `dir` the contact normal into that side (`normal` for `b`, `-normal` for `a`); zero
  amounts are not sent. The damage step's total-key sort (above, "Order") makes the result independent of event order.
- **Culprit (IN4c, built; was `None` in IN4b).** `CarryFeedback { holder, outcome: Thrown }` (`sf_physics_bevy` `carry.rs:99`)
  names no body, so a thrown body's thrower is unknown; IN18's "Thrown landing" noise needs the same link. Row IN4c
  (later, with the carry owner): the thrown `BodyId` in the feedback, a `Thrown(BodyId -> (UserKey, tick))` table in the
  shell kept until the body sleeps, its key as the culprit (vandalism reads `Destroyed.culprit`).
- **Needs** `PhysicsParams::contact_report_layers` non-empty (default `Layer::MOVING`); empty: no impacts, no cost.

**Tunables** (`interact.impact.*`, on `InteractParams`, bound in `sf_interaction_bevy`, interaction.md table):

| Key | Unit | Default | Range | Source | Kind |
|---|---|---|---|---|---|
| `interact.impact.speed_min_mps` | m/s | 3.0 | 0.5-10 | Guess (a 0.46 m fall; resting 0.16 m/s; Source min speed [10]) | Gameplay |
| `interact.impact.impulse_min_ns` | N·s | 0.5 | 0-50 | Guess (Source `minhealthdmg` dead zone [10]) | Gameplay |
| `interact.impact.damage_per_ns` | ratio | 0.5 | 0-10 | Guess (Source `physdamagescale`, Chaos threshold [9]) | Gameplay |

Worked numbers at the defaults (inelastic, factor 1): a 0.4 kg bottle off a 0.9 m counter (4.2 m/s, 1.7 N·s) takes
0.6 of its 1 hp and survives; thrown at 10 m/s (4 N·s) it takes 1.75 and breaks; a 1 kg pot dropped 3 m (7.7 m/s,
7.7 N·s) takes 3.6 of 2 hp and breaks, and the same pot thrown at 10 m/s into a static crate (10 N·s) chips 4.75 of
the crate's 15 hp (and breaks).

**Tests.** IN4a (exact): under `speed_min_mps` 0 at any impulse; at the dead zone 0; linear above it (two points,
exact bits); NaN/inf in -> 0; same inputs same bits. IN4b (headless exact, `--features physics`): a registered dynamic
prop dropped from 3 m onto a fixed floor gets one `PropDamage` per impact with `impact_damage` of the event's own
impulse and speed (read back from `PhysicsEvents`), and breaks at 1 hp; a 0.2 m drop gives none; a dynamic body
falling on a registered fixed placement damages that placement only; a registered light item on `Layer::DEBRIS`
(below `debris_mass` / `debris_extent`) takes impact damage like a `PROP` one; an unregistered pair, a debris-shell
piece (`DebrisPiece`) landing on a registered placement and a `HELD` body give none; two physics steps in one frame do
not double an event.

**Industry comparison.** Matches Chaos (impulse-driven strain against a per-cluster threshold, per-material
multiplier [9]) and Source in its knobs (per-prop scale, minimum damage, speed floor [10]; Source's measure is collision energy); Unity's usual `OnCollisionEnter`
practice is the same impulse threshold. Bethesda (Havok in Skyrim/Fallout 4) destructibles taking physics-impact
damage: U (not verified). RAGE fragment break strength (GTA V, RDR2): U.

[8] https://rapier.rs/docs/user_guides/templates/advanced_collision_detection/ (contact force events: summed force
magnitudes over a per-collider threshold, every step the contact lasts; U: search summary)
[9] https://dev.epicgames.com/documentation/en-us/unreal-engine/chaos-fields-user-guide-in-unreal-engine,
https://dev.epicgames.com/documentation/unreal-engine/API/PhysicsCore/FPhysicalMaterialDamageModifier (U: search
summaries)
[10] https://developer.valvesoftware.com/wiki/Impact_Damage_Table, https://developer.valvesoftware.com/wiki/Prop_physics
(U: search summaries; IN4-REV search: the table page is a stub listing the SDK's player/NPC thresholds, 24 in/s
minimum linear speed, nothing under 2 kg, small objects under 5 kg above 36 in/s)

## Bashing (IN37; spec IN37-S 2026-11-27)

Owner 2026-11-25: locks and doors are bashed by strength, with a weapon or fists, risking health and contents, a crime
if reported; doors are bashed open, never destroyed. Crime owns the lock (`LockState.hp`, `broken`, the offences,
the forced-lock noise: crime-locks.md "Force (IN37)"); this lane owns the blow.

**Reuse check.** Bevy 0.20: nothing (a game rule). Crates: none (crates.io "bash", "lock", "breach" 2026-11-27: no
lock or breach crate; the RPG crates seen for crime-locks.md cover stats and magic). Ours, reused: the activation
pipeline (the `bash` verb is a `VerbDef` like `snuff`: `requires` is the adapter's, so the core never names a lock),
`PropDamage` (the combat and impact damage path), `NoiseInbox` (`NoiseCase::Bash`, `interact.noise.bash_reach_m`,
built in IN18a/b), the change inbox (`CHANGE_BASHED` exists, IN14a), `break_seed`'s splitmix64 scheme.
`DestructionRules` is not reused for the lock factor: it scales damage to the *object's* material, and a door's
wood says nothing of its lock; the threshold axis is new.

**Industry comparison.**

| Game | Forcing a lock | Ours |
|---|---|---|
| Daggerfall [1] | each weapon blow on a door rolls under (20 - lock strength)%; bashing wears the weapon, trains no skill, noise draws guards in town | **match** wear, noise, no skill; **departs**: hp per blow, not a roll (no lucky first hit; deterministic for co-op) |
| Baldur's Gate 3 [2] | doors and chests have hit points and a damage threshold (a blow under it does nothing; over it, full damage); some damage types double; fire or acid on a chest destroys contents; examine shows hp | **match**: hp (crime's), `threshold`, per-kind factor, contents risk per blow |
| Cyberpunk 2077 | "force open" doors gated by the Body attribute (one check, no blows) | **departs**: a strength gate only through `threshold` (a weak blow never forces a strong door) |
| Skyrim, KCD2, Morrowind/OpenMW | no forcing: pick, key or spell | **departs** (owner); Classic preset keeps bash off (interaction-adapter.md) |

**Core** (`sf_interaction/src/bash.rs`, pure):

```rust
pub const VERB_BASH: TagId = TagId::of("bash");
pub struct Blow { pub force: f32 /* the adapter's: strength x weapon */, pub kind: TagId /* blunt, blade, hand.. */,
                  pub bare_handed: bool }
pub struct BashRules { pub kind_factor: BTreeMap<TagId, f32> /* unknown = 1 */,
                       pub threshold: BTreeMap<TagId, f32> /* by the target def's material; unknown = 0 */,
                       pub self_per_force: f32, pub wear_per_force: f32,
                       pub contents_risk_per_force: f32, pub contents_risk_max: f32 }  // adapter record; neutral default
pub struct BashResult { pub lock_damage: u32, pub self_damage: f32, pub weapon_wear: f32, pub contents_risk: f32 }
pub fn bash(rules: &BashRules, blow: &Blow, material: Option<TagId>) -> BashResult;
pub const fn bash_seed(placement: PlacementId, user: UserKey, tick: Tick) -> u64;   // splitmix64 chain as break_seed
```

`f = force x kind_factor(kind)`; `lock_damage = 0` if `f < threshold(material)`, else `floor(f)` (BG3: no mitigation
over the threshold); `self_damage = force x self_per_force` only bare-handed, `weapon_wear = force x wear_per_force`
only armed; `contents_risk = min(force x contents_risk_per_force, contents_risk_max)` (raw force: the contents feel
the blow, not the lock). Non-finite or negative force = a zero result. `Bashed { tick, placement, culprit, kind,
force, result, point, seed }` is the core event. `from_bashed(&Bashed, &ChangeSite) -> WorldChange` (kind
`CHANGE_BASHED`, cause the blow's kind, `ChangeDetail::Force(u32)` = `lock_damage`: world-reactions "Carries" `force`).

**Shell** (`sf_interaction_bevy/src/bash.rs`, `BashSystems`):

- **Inputs, host-filled:** `BashRules` (resource; default neutral: factor 1, threshold 0, risks 0) and `BashBlows`, a
  closure `UserKey -> Option<Blow>` (the FireWetness pattern; unfilled or `None` = the use does nothing).
- **Verb** (`Update`, after `ActivationSystems::Default`): a `UseApproved` with verb `bash` on a placement whose def
  offers `bash` queues one `PendingBlow { user, placement, blow, land: tick + interact.bash.windup_s }`, the blow read
  at the press. One pending blow per user; a press inside `interact.bash.recover_s` of the user's last landed blow is
  dropped. A script `Activate` with `bash` (no user) does nothing.
- **Hits** (`FixedUpdate`, `BashSystems::Land`): a `PropDamage` with a culprit on a placement that is **not** a
  registered destructible and whose def offers `bash` with its `requires` met is a blow landing now: `Blow { force:
  amount, kind, bare_handed: kind == hand }` (the attack already swung; combat prop targets C2c/C3 send these). `DAMAGE_IMPACT` is
  never a blow, so a thrown crate never forces a lock (a thrown body's impact names its thrower as culprit,
  `impact.rs:210`; impacts reach only registered destructibles today, a guard for later). Destructibles keep their own path: a locked crate breaks,
  lock included (interaction-adapter.md "Breakables").
- **Land** (`FixedUpdate`, in the IN39 chain after `InteractionSystems::Apply`, before `DestroySystems::Damage`):
  due blows in (land tick, user, placement) order. Dropped when the target left (no def) or its `requires` fail now
  (unlocked meanwhile), or the user's `ActionBody` feet are over `interact.bash.reach_m` from the point. Else: `bash`,
  one `Bashed` message; a registered destructible target also gets `PropDamage { amount: force, kind, culprit }`
  (same tick); one `Noise` (`NoiseCase::Bash`, bash reach x material ratio, maker the culprit, revealed: the basher is
  in plain sight of the sound) into `NoiseInbox`; `from_bashed` into the change inbox. Point = the def's door slot
  doorway when it has one, else the object origin (`InteractionFrames::current`).
- **Not saved:** a pending blow is under a second; lost on save or unload. Lock hp is crime's (`sf/crime` overlay).
- **Net:** host-authoritative; clients get `Bashed` (seed included) and play the swing; the result never comes from a
  client.

**Crime's side (LW6, crime lane, not IN37):** `sf_crime_bevy` reads `Bashed` for every target with an `ObjectLaw` and
sends `LockRequest::Force { object, damage: lock_damage }`, a zero damage included (an attempt: `UnlockAttempted`,
trespass when owned); on `Forced` it writes `CHANGE_LOCK_BROKEN` into the change inbox and, on a placement with a door
slot, one `ScriptActivate { verb: open }` with the basher as actor (the gate now passes; the door default swings it
open). The dependency stays one-way: crime's host reads interaction's message.

**Game (`mw_interaction(_bevy)`):** fills `BashBlows` from Strength and the readied weapon (OpenMW's melee scale
`fDamageStrengthBase + fDamageStrengthMult x 0.1 x Strength` on the weapon's max damage, `apps/openmw/mwclass/npc.cpp`
`hit`, U: not re-read this pass; fists: `mw.interaction.bash.fist_force` x the same scale), `BashRules` from
`mw.interaction.bash.*` (`Guess`; the scale is set against crime's `mw.crime.lock_hp_per_level`, so an iron weapon at
Strength 50 forces a level-50 lock in about 7 blows), the `bash` verb on locked `DOOR`/`CONT` defs (hold list, after
`pick`, `requires: [locked]`, the tag crime mirrors; Modern only), and applies each `Bashed` result: `self_damage` to
health, `weapon_wear` to the weapon's condition (Morrowind's `fWeaponDamageMult` wear on a hit, U), and per fragile
item in a container (potions, glass: the adapter's list) one seeded roll (`seed`, item index) under `contents_risk`
removes it as broken.

**Tunables:** `interact.bash.windup_s` (s, 0.1-1.5, `Guess` 0.45: a heavy swing to contact), `interact.bash.recover_s`
(s, 0.3-3, `Guess` 1.0), `interact.bash.reach_m` (m, 0.5-4, `Guess` 2.0), all Gameplay. The game's `mw.interaction.bash.*`
(kind factors, thresholds by material, `self_per_force`, `wear_per_force`, contents risks, `fist_force`) are `Guess`.

**Rows:**

| Row | Deliverable | Claims | Test (exact) |
|---|---|---|---|
| IN37a | core `bash`, `Blow`, `BashRules`, `BashResult`, `Bashed`, `bash_seed`, `from_bashed` + `ChangeDetail::Force` | `sf_interaction/src/{bash.rs,lib.rs,world_change.rs,tests_bash.rs}` | under threshold 0 damage, over = floor(f); kind factor; fists self damage, armed wear only; risk capped; NaN = zero; seed stable; mapper field for field |
| IN37b | shell: `BashRules`/`BashBlows` resources, verb queue, `PropDamage` hits, land, `Bashed`, noise, change, destructible `PropDamage`, 3 tunables | `sf_interaction_bevy/src/{bash.rs,lib.rs,tests_bash.rs}` | headless: press -> one `Bashed` at land tick with the result; second press in recovery dropped; requires lost before land -> none; out of reach -> none; culprit hit on a non-destructible lock -> `Bashed` same tick, impact none; locked destructible -> `PropDamage` too; one noise, one `WorldChanged` |
| IN37c | game: `BashBlows`, `BashRules` from `mw.interaction.bash.*`, `bash` verb on locked DOOR/CONT (Modern), apply self damage, wear, contents rolls | `mw_interaction(_bevy)` | exact: blow for Strength 50 + iron weapon, fists; verb only when `locked`; seeded contents rolls remove the expected items |
| IN37d | presenter: the readied weapon's power attack clip (Morrowind clips) for the swing, impact sparks or splinters; owner look | `mw_interaction_bevy` presenter | owner check (visual-checks entry) |

Live play needs crime LW6 (the `locked` mirror and `Force`): until then IN37b/c test with the tag set by hand.

[1] https://en.uesp.net/wiki/Daggerfall:Lockpicking (U: via search summary; Daggerfall Unity's `AttemptBash` source
not read). [2] https://www.gamerevolution.com/guides/943548-baldurs-gate-3-break-sturdy-medium-toughness-greater-toughness-doors-walls-objects,
https://www.pcinvasion.com/how-to-open-locked-doors-and-chests-in-baldurs-gate-3/ (U: guides via search summaries).
Cyberpunk 2077 Body door gate: common knowledge, no source read (U).

## As built (IN1a, 2026-11-27)

`sf_interaction::destroy` (`destroy.rs`): `DestructState { health, stage }` (stage = stages reached, 0 intact),
`apply_damage(def, state, damage) -> Hit { state, crossed, overkill }`: health floors at 0; every stage from the current
one with `health / def.health <= at` is crossed in def order (descending `at`, the cook's job), several per hit, never
twice; overkill = damage past zero / `def.health`; zero, negative or non-finite damage is a no-op. `debris_lod` (inclusive,
NaN = no bodies), `BreakQueue<T>` (`take_tick(n)`, FIFO carry-over), rest state `[v1, stage, health f32 LE]` under owner
`destruct` (stage past the def is the shell's to clamp). `interact.debris.{lod_m, breaks_per_tick}` bound in
`sf_interaction_bevy`; `awake_max` / `fade_s` wait for the shell (IN1b). Tests: `tests_destroy.rs` (6).

## As built (IN1b, 2026-11-27)

Core: `Destroyed` (stage = index into `stages`), `DAMAGE_SCRIPT`, `splitmix64`, `break_seed` (`destroy.rs`). Shell
`sf_interaction_bevy/src/destroy.rs`: `Destructibles` (`register` last in the Apply chain), `DestructionRules`
(`(kind, material)`, factor 1 when absent or the def has no material), `PropDamage`, `DestructRest`, `DestructChanged`,
`Destroyed`, `PendingBreaks`; `DestroySystems::{Damage, Release}` chained after Apply. A hit that changes nothing writes
no `DestructChanged`. `awake_max` / `fade_s` stay with IN1c. Tests: core 7 (+ splitmix64 reference value and fixed
seeds), shell `tests_destroy.rs` (8).

## As built (IN1c, 2026-11-27)

`sf_interaction_bevy` `debris.rs` behind `physics` (optional `sf_physics_bevy`, `sf_world_bevy`, `sf_records`,
`bevy_transform`; on at the root entry, all plain selections in Bevy group [1]): `DebrisSets` (record -> `DebrisPieceDef
{ shape, local, mass }`), `DebrisPiece` + `DebrisFading`, `DebrisSystems::{Fade, Spawn}` after `Release` and
`PhysicsSystems::Step`, run only with `Physics` + `PhysicsFocus`. Launch = `set_velocity` (exact speed whatever the mass
clamp) away from the hit along `dir`, `interact.debris.{speed_mps, spread}` (new, Guess) jitter from
`splitmix64(seed ^ rank)`. Cap counts pieces not fading (a shrinking piece keeps its body for `shrink_s`); a tick past
the cap keeps its newest pieces. Pieces carry `CellCoord` + `Transform` (physics write-back); the game's sets and mesh
hook are not built yet, so the game shows no debris. Tests `tests_debris.rs` (4 headless).

## As built (IN38b1, 2026-11-27)

`mw_interaction_bevy/src/breakable.rs`: `BreakablesTable::build(db)` classifies every `CONT` / `MISC` / `STAT` record
(own script with an `OnActivate`, `OnDeath`, `OnMurder`, `OnKnockout` or `OnPCHitMe` block, or named by any `SCPT` /
dialogue token via `mw_world::source::collect_script_tokens`, refuses); `STAT` urns are only counted (IN38c). The
index hook (`index.rs`, now also wrapping `Dynamic` for MISC pottery) attaches `destructible_def` (live `BreakSettings`,
the 12 `mw.rules.interaction.break*` tunables bound) and the material to the furniture def, or a slotless def of its
own; a breakable stays indexed with that def while sitting is off. `DestructRest` follows the `Insert` from overlay
owner `destruct`; `save_rest` after `Damage` writes standing states only. `sync_rules` refills `DestructionRules`. Dev
key `Delete` (`dev_break`, registered under the crate's `dev` feature, which the app's `dev` turns on). Host:
`exterior.rs` calls `breakable::insert_records`. Tests: `tests_breakable.rs` (4).

## As built (IN38b2, 2026-11-27)

`mw_scripts_bevy`: `LIVE_CLASSES` (Dynamic, Light, `CONTAINER`) fill `ActivePlacements`; a container's visuals spawn
under a `LiveRoot(PlacementId)` per level (pose on the root, scale on the visuals, as `ItemPivot`);
`ActivePlacements::remove_live` = deactivation hooks, then overlay removal, then its roots. `mw_rpg_bevy`
`breakable.rs`: `apply_breaks` (`FixedUpdate`, `.after(DestroySystems::Release)`) runs `remove_broken` on every
removing `Destroyed`: `Live` (`remove_live` + its `ItemPivot`), `Unloaded` (overlay only, any visual left despawned),
`Gone`. A standing stage changes nothing (no model swap stage exists yet). Host call: `exterior.rs`. Tests:
`mw_scripts_bevy` `tests.rs` (1), `mw_rpg_bevy` `tests_breakable.rs` (2). Uncooked models (runtime importer) spawn
through the uncooked hook without a root: they would stay drawn until the cell reloads (U: count unknown).

## As built (IN38b3a, 2026-11-27)

`sf_world_bevy` `stream.rs`: `Streaming::spawn_when_ready(world, spawners, ws, cell, placement) -> SpawnWhenReady`
(`Queued | Refused`; refusals as `spawn_into`'s minus the model, plus an id already queued), one model user taken at
once; `drain_pending` after the main-thread stages (skipped on origin-guard frames, like them): entries wait on a
stage in flight or the model (plus one settle frame outside burst, as loads and promotions), drop on a changed or
gone near group, an id the cell holds (`holds`, shared with `spawn_into`), or no spawner for the resolved model
(warn); spawns go through `place` (`spawn_into`'s tail, the user passing to the near group). `pending_spawns()`,
`StreamStats::pending_dropped`. Tunable `stream.pending_spawns_per_frame` (`sf_world::STREAM_PENDING_SPAWNS`, bound
live in `mw_exterior_bevy` `restream.rs`). Tests: `tests_pending.rs` (6 headless exact).

## As built (IN38b3b, 2026-11-27)

`sf_physics_bevy` `stream.rs`: public `ItemMotion { asleep, linear, angular }` with `encode` / `decode` (v1, 26 bytes;
`decode` refuses another version, length or a non-finite velocity) and `ItemMotion::DROPPED` (awake, zero velocity: the
spill's blob); the deactivation writer uses `encode`. Item activation decodes the overlay blob into `Pending`; an awake
one inserts `Dynamic { asleep: false }` with CCD (never `Held`, whatever `ItemBody::hold`) and `set_velocity` right
after the insert. Tests: `motion_tests.rs` (4 headless exact).

## As built (IN38b3c, 2026-11-27)

`mw_interaction` `breakable.rs`: `spill_draw` (`splitmix64(splitmix64(seed ^ index) ^ k)`, top 24 bits), `lost_fraction`,
`kept_count` (share in f32, draw added in f64), `health_loss`, `scatter` (+Y up), `LooseItem` (`mw.loose` v1, 10 bytes);
`SpillParams` (8 `break_spill_*` tunables, `Live`) inside `BreakParams`, bound by `mw_interaction_bevy`. `mw_rpg_bevy`
`breakable.rs`: `apply_breaks` spills when the registry's stage says `SpillContents` (a CONT record once the registry let
go); the container is captured from `remove_live` (Live) or `overlay.placed` (Unloaded: moved or spawned containers only;
an unmoved index reference has no transform there, so its store stays keyed by the dead id, warn); `spill_stacks` is the
pure per-stack rule; Live queues each item through `Streaming::spawn_when_ready` into `Streaming::partition(ws).cell_of`
its position (new accessor). The host passes the live params as `SpillSource(fn(&World) -> SpillParams)`
(`mw_interaction_bevy::breakable::spill_params`): `mw_rpg_bevy` takes no dependency on that shell. Take reads `mw.loose`
before the removal forgets it. Tests: `mw_interaction` `tests_breakable.rs` (4 exact), `mw_rpg_bevy` `tests_breakable.rs`
(4 headless exact); the "live once the model settles" half stays covered by `sf_world_bevy` `tests_pending.rs` (a
`Streaming` with cooked models is not built in `mw_rpg_bevy` tests).

## As built (IN38c1, 2026-11-27)

`Streaming::hide_covered` (`sf_world_bevy` `stream.rs`, after `partition`) checks the cell's Near group's `covered`
set, inserts the id into `promoted` (shared with promotions, so either one refuses the other) and calls
`CellScene::hidden` (new default no-op, `scene.rs`); `SceneHook::hidden` (`sf_scene_bevy` `residency.rs`) forwards to
`promoted` (one `PromotePlacement`). Tests: `sf_world_bevy` `tests.rs` `hide_covered_is_one_way_and_only_for_what_the_scene_draws`
(recording `TestScene`: one `hidden`, second hide false, promotion after it dropped, uncovered / not resident / unknown
false); `sf_scene_bevy` `residency_tests.rs` `hidden_queues_one_inactive_write_per_instance_and_stays_gone_with_the_overlay`.

## As built (IN38c2, 2026-11-27)

`BreakablesTable::build` lists `STAT` breakables (counted apart in `BreakCounts::statics`); `static_ids(db)` and
`insert_records`'s return give their ids, which the host puts in `mw_scripts_bevy::LiveStatics(Arc<HashSet<String>>)`
(record id text: the crate takes no `sf_records` edge). `ScriptHandleHook`'s `liveness` makes a listed static live and
rooted. `mw_rpg_bevy` `take_out`: a `Static` from `remove_live` then `hide_static` (`Streaming::hide_covered` under
`resource_scope`, no-op without `Streaming`). Tests: `mw_scripts_bevy` `tests.rs` (listed vs unlisted), `mw_rpg_bevy`
`tests_breakable.rs` (scene- and entity-drawn urn); the hide's true half is `sf_world_bevy`'s (no streamed cell here).

## As built (IN38c3, 2026-11-27)

`mw_bake` `far.rs`: `MwFarSource` keeps `breakable_statics` (lowercase `STAT` ids `classify` lists, `named_by_script`
false: a named one is already vetoed by its token); `precombine_exclude` = script token or breakable static. The set is
hashed into the fingerprint and `RULES_VERSION` 3 -> 4, so `lod.near` re-bakes once. Tests (`far.rs` `breakable_tests`):
`build_near` over a wall and an urn keeps only the wall; adding an urn changes the fingerprint, a plain static does not.

## As built (IN4a, 2026-11-27)

`sf_interaction` `impact.rs` `impact_damage(impulse_ns, speed_mps, &InteractParams)` as "Impacts (IN4)" Core; a finite
impulse whose damage overflows f32 saturates at `f32::MAX` (it still breaks), a NaN tunable gives 0. `DAMAGE_IMPACT` in
`destroy.rs`; `impact_{speed_min_mps, impulse_min_ns, damage_per_ns}` on `InteractParams`, bound in `sf_interaction_bevy`
(`bind_rules`). Tests `tests_impact.rs` (4 exact): speed gate, dead zone + two linear points, non-finite, same bits.

## As built (IN4b, 2026-11-27)

`sf_interaction_bevy` `impact.rs` (`physics` feature): `ImpactCursor` (reset in `First`), `ImpactSystems` as "Shell",
run only with `Physics`, `PhysicsEvents` and `PhysicsStreaming`; the body map is built from the active world only on a
step with a nonzero hit. New seam `PhysicsStreaming::insert_placed(ws, placement, body)` (`sf_physics_bevy`
`stream.rs`): a host-spawned body `body()` finds (tests use it). Tests `tests_impact.rs` (6 headless exact): every
frame's hits equal the spec's from that frame's own contacts (3 m drop breaks 1 hp, 0.2 m none, fixed target only, a
`DEBRIS` bottle counts, two steps a frame each event once), no hit from unregistered/`DebrisPiece`/`PROJECTILE`/`HELD`.

## As built (IN4c, 2026-11-27)

`sf_physics_bevy` `CarryOutcome::Thrown { body, speed }` (the thrown body). `sf_interaction_bevy` `impact.rs`: `Thrown`
(body -> `ThrownBy { user, tick }` in the throw's world) fed in `ImpactSystems` from `CarryFeedback` of a holder with an
`InteractionUser` (the game's player body has one, `mw_interaction_bevy` `input.rs`), pruned after the step when the body
sleeps, is `HELD` again or is gone, cleared on a world change; a side's culprit is the other body's thrower, else its
own. 2 headless exact tests: thrown -> every hit on both sides carries the thrower; rested -> forgotten, dropped again
-> none; a holder with no `InteractionUser` -> none.
