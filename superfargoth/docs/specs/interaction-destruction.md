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
Decorative `STAT` urns are static-scene instances: their removal is its own row (IN38c, after IN38b2). The
deactivation hook (`mw_interaction_bevy` `index.rs:91`) writes `IndexChange::unload` too: ordered after `Release`, the
shell's `OwnerDestroyed` is sent first and wins (the step sorts stable by placement; the second `Remove` finds nothing).
Crime's vandalism hook and noise (IN28) read `Destroyed` too. Classic rules: no destructible defs, nothing runs.

**Dev source** (IN38b1, owner check IN-1 before C14): behind the game's `dev` feature, a dev key writes one
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
placement's break removes it from the overlay; owner scene IN-1.

## As built (IN1a, 2026-11-27)

`sf_interaction::destroy` (`destroy.rs`): `DestructState { health, stage }` (stage = stages reached, 0 intact),
`apply_damage(def, state, damage) -> Hit { state, crossed, overkill }`: health floors at 0; every stage from the current
one with `health / def.health <= at` is crossed in def order (descending `at`, the cook's job), several per hit, never
twice; overkill = damage past zero / `def.health`; zero, negative or non-finite damage is a no-op. `debris_lod` (inclusive,
NaN = no bodies), `BreakQueue<T>` (`take_tick(n)`, FIFO carry-over), rest state `[v1, stage, health f32 LE]` under owner
`destruct` (stage past the def is the shell's to clamp). `interact.debris.{lod_m, breaks_per_tick}` bound in
`sf_interaction_bevy`; `awake_max` / `fade_s` wait for the shell (IN1b). Tests: `tests_destroy.rs` (6).
