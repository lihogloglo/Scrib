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

