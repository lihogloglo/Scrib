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
damage (fists), and emits `Bashed { tick, placement, force, seed }`. The shell sends crime's `LockRequest::Force`;
crime settles `hp`, `broken` and the offence (crime.md decision 8; vandalism if owned). Noise: see (d) (IN28).

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
