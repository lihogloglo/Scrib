# Interaction (b): destruction (`sf_interaction`)

Status: design 2026-11-11, updated 2026-11-25 (IN27; moved out of [interaction.md](interaction.md) for size). Part
(b) of the interaction spec: its shared core (defs, events, tick and net rules, the `interact.debris.*` tunables),
"Budgets" and "Build slices" (IN1, IN4, IN37, IN38, IN12, IN13) stay there. Lane roadmap:
[interaction](../../../docs/roadmaps/interaction.md) decision 5. Research:
[gameplay-2026-11-25/interaction](../../../docs/research/gameplay-2026-11-25/interaction.md) §4.

<!-- agent-summary -->
> **Agent summary.** Destructible props as data on `InteractableDef`: health, material, stages. Tier 1 stage swap
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
pub struct DestructibleDef { pub health: f32, pub material: TagId /* wood, clay, glass, stone */,
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
**Net**: host-authoritative; `Destroyed { tick, placement, stage, point, dir, seed }`; clients swap and spawn debris
locally (seeded, own LOD). **Save**: an overlay blob `interaction` v1 per placement: health, stage, door state and
lever position (save.md "Overlay blobs", blob version table); spilled, looted and salvaged items are overlay spawned
placements; a removed object is an overlay removal; reset classes per [world-reactions.md](world-reactions.md) (IN15).
**Risk**: low for tier 1; medium for chunks (content); high for structures.


## Shell and game (IN1b, IN1c, IN38b; spec IN1-S 2026-11-27)

Pattern: the light shell (`sf_interaction_bevy/src/light.rs`, IN19b) and its game hook (`mw_interaction_bevy`
`light.rs`, IN19c). The shell owns state and events; every model, item, sound and save byte is the host's.

**Registry** (IN1b, `sf_interaction_bevy` `destroy.rs`). `Destructibles(BTreeMap<PlacementId, Destructible { def:
Arc<InteractableDef>, object: Frame, state: DestructState, gone: bool }>)`, kept by `destroy::register` in the
`InteractionSystems::Apply` chain after `light::register` (same tick, send order): an `IndexChange::Insert` whose def
has `destructible` registers it (`DestructState::intact`); a re-insert keeps the state (a stage swap re-sends the
broken model's def); an insert without one, or a `Remove`, drops it; `Move` refreshes `object`. Then, after all index
changes, `DestructRest { placement, state }` (a restored save or a streamed-in cell) sets the state clamped to the
def (stage <= `stages.len()`, health in `0..=def.health`); no event: the host that wrote it shows its own stage.

**Damage in.** One message any source writes, ordered `.before(DestroySystems::Damage)`:

```rust
// Illustrative, not compiled.
#[derive(Message)] pub struct PropDamage { pub placement: PlacementId, pub amount: f32, pub kind: TagId,
    pub point: Vec3 /* world m, active space */, pub dir: Vec3, pub culprit: Option<UserKey> }
```

Writers: combat C14 prop targets (later), impacts IN4, rules/fire IN17, the dev source (IN38b). `kind` is the
source's damage tag (adapter data: blunt, blade, arrow, hand, fire, impact); the shell scales `amount` by
`DestructionRules` (a resource the adapter fills: `(kind, material) -> factor`, default 1; the framework never names
a stat), so IN4 and IN17 write raw amounts. Dropped: unknown placement, `gone`, factor 0, and every kind but
`DAMAGE_SCRIPT` (`TagId::of("script")`) on an `ignore_external` def (Bethesda's flag: only a quest breaks it).

**Order** (`FixedUpdate`, host, `DestroySystems::{Damage, Release}` chained after `InteractionSystems::Apply`):

1. **Damage**: each `PropDamage` in send order -> `apply_damage` on the current state at once (health is
   authoritative now: two same-tick hits add, a stage is never crossed twice); every crossed stage pushes one pending
   break `{ hit tick, placement, stage, point, dir, culprit, overkill (last crossed only, else 0) }` into one
   `BreakQueue`; a crossed stage whose `model` is `None` sets `gone` (later hits drop). One `DestructChanged {
   placement, state }` per placement whose state changed this tick (its last state): the host's save write.
2. **Release**: `take_tick(interact.debris.breaks_per_tick)` -> one `Destroyed(sf_interaction::Destroyed { tick,
   placement, stage, point, dir, seed, culprit, overkill, removes })` each, FIFO (a multi-stage hit stays in def
   order across ticks). `tick` is the hit's tick, so `seed = splitmix64(placement, tick, stage)` (a local fn in
   `destroy.rs`; no hash crate is a dependency) is the same on host and clients whatever the queue did. A released
   break with `removes` also writes `IndexChange::Remove { placement, reason: OwnerDestroyed }`: claims on it end in
   the next tick's `Apply` (one tick, like any host `Remove` written after `Apply`; a sitter stands, a carrier drops).

The cap delays presentation and removal, never health: the state saved is always the true one.

**Host stage hook** (IN38b, `mw_interaction_bevy` `breakable.rs`), on `Destroyed`, by `def.stages[stage]`: a model
swap re-sends `IndexChange::Insert` with the broken def; `None` removes the placement (hide or despawn its model,
remove its body, overlay removal); `on_break` spills the contents as placed items keeping their owner, damaged by
`overkill` through the adapter's curve, rolls `loot`, places `salvage`; plays `effect`. Per `DestructChanged` it writes
`DestructState::rest_bytes` under `DESTRUCT_STATE_OWNER` (a removed placement needs none); on cell load it sends
`DestructRest` after the `Insert` and shows the saved stage's model. Crime's vandalism hook and noise (IN28) read
`Destroyed` too. Classic rules: no destructible defs, so nothing here runs.

**Dev source** (IN38b, owner check IN-1 before C14): behind the game's `dev` feature, a dev key writes one
`PropDamage { amount: def.health, kind: blunt }` at the player's `PlayerFocus` placement, so one press runs every
stage of a one-stage crate; the build slice greps the game's dev key table for a free key.

**Debris** (IN1c, `sf_interaction_bevy` `debris.rs`; adds the `sf_physics_bevy` dependency, which the game links
already: IN1-REV checks `tools/bevy_variants.sh` stays in group [1]). Every client (host included) runs it on
`Destroyed`; never gameplay, never saved:

- **Gate**: `debris_lod(distance from this client's camera, interact.debris.lod_m)`; beyond it no bodies.
- **Pieces**: `DebrisSets` (host resource: debris record -> pieces `{ shape: ShapeRef, local: Frame, mass }`) for
  each of the stage's `debris` records; a seeded stream from `seed` picks the impulse jitter around `dir` at `point`,
  so the same event spawns the same set. Each piece is an entity with `DebrisPiece { set, piece }` and a `PhysicsBody`
  (`BodyKind::Dynamic`, `Layer::DEBRIS`, `ITEM_DAMPING`, inserted through the active `PhysicsWorld`); the host's
  visual hook gives it a mesh on `Added<DebrisPiece>` (the framework never loads a model).
- **Cap**: at most `interact.debris.awake_max` pieces alive in the client's active space; spawning past it starts the
  fade of the oldest first (the newest break is where the player looks), never refuses a break.
- **Fade**: a piece starts fading when its body sleeps or `interact.debris.fade_s` after spawn, whichever first, then
  scales to zero over `interact.debris.shrink_s` and despawns (its body goes with the component).

**Reuse check.** Bevy 0.20: buffered `Message`s (send order, readable from `FixedUpdate`, several readers) carry
every step, as `IndexChange` and `LightChanged` do; observers and `EntityEvent` need a target entity, and a static
placement has none before promotion, so they don't fit. Crates: physics is `sf_physics` (rapier) already; avian or a
second rapier for debris would be a second world; runtime fracture crates stay rejected (tier 1 needs none).

**Industry comparison.** Bethesda `DEST` (Creation Kit): stages by health percent, a model per stage, a debris form
plus count spawned at the stage, "ignore external damage", and self damage per second to cook an object off [1]:
match, except self DPS, which our fire rules (IN17c) cover. UE Chaos: pieces are removed on sleep (sleep min/max
timers) and shrink on removal (`bScaleOnRemoval`, default on) [2]; it has no global piece cap (a game scripts its
own), so ours adds the cap with oldest-first fade. RDR2 (RAGE breakables): U (no public source read).

[1] https://ck.uesp.net/wiki/Destructible_Objects , https://falloutck.uesp.net/wiki/Destructible_Object_Data
[2] https://dev.epicgames.com/documentation/unreal-engine/API/Runtime/GeometryCollectionEngine/UGeometryCollection
(U: API summary via search, page not read in full)

**Tunables added**: `interact.debris.shrink_s` (s, 0.5, 0.1-3, Guess; Industry: Chaos scale on removal [2];
Presentation), declared by IN1c with `awake_max` and `fade_s` (already in interaction.md's table).

**Tests.** IN1b (headless exact): one hit crosses two stages -> two `Destroyed` in def order, seeds differ and are
equal across two runs; more breaks than the cap -> the rest next tick, `tick` still the hit's; a hit on a `gone`
placement and an `ignore_external` non-script hit drop; the `DestructionRules` factor scales; `DestructRest` gives no
event and clamps; a removing stage writes `Remove { OwnerDestroyed }` and the sitter's claim ends next tick; one
`DestructChanged` per placement per tick. IN1c (headless): same seed same piece set and impulses; the cap holds
(oldest fades first); beyond LOD no bodies; all despawned after fade + shrink; sanity: no NaN, pieces rest.
IN38b (headless exact): the hook removes the placement once, spill count, overlay round trip; owner scene IN-1.

## As built (IN1a, 2026-11-27)

`sf_interaction::destroy` (`destroy.rs`): `DestructState { health, stage }` (stage = stages reached, 0 intact),
`apply_damage(def, state, damage) -> Hit { state, crossed, overkill }`: health floors at 0; every stage from the current
one with `health / def.health <= at` is crossed in def order (descending `at`, the cook's job), several per hit, never
twice; overkill = damage past zero / `def.health`; zero, negative or non-finite damage is a no-op. `debris_lod` (inclusive,
NaN = no bodies), `BreakQueue<T>` (`take_tick(n)`, FIFO carry-over), rest state `[v1, stage, health f32 LE]` under owner
`destruct` (stage past the def is the shell's to clamp). `interact.debris.{lod_m, breaks_per_tick}` bound in
`sf_interaction_bevy`; `awake_max` / `fade_s` wait for the shell (IN1b). Tests: `tests_destroy.rs` (6).
