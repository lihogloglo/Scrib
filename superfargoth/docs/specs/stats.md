# Character stats: data-defined values, pools, modifiers, progression seams (`sf_stats`)

Status (2026-11-05): S0-S2 built (`sf_stats`, `mw_stats` table + init; as-built detail: `stats-as-built.md`); S3 next. Game side:
`../../../bevyrrowind/docs/specs/stats-adapter.md` (Morrowind's 8 attributes, 27 skills, GMSTs, OpenMW formulas).
Unblocks ROADMAP 2.7 (every OpenMW content mechanic needs stats: `../../../bevyrrowind/docs/research/openmw-mechanics-2026-11-05.md`
"Finding zero"). The spell/effect runtime is the **next** spec: this one defines only the seam it writes through
("The effect seam").

## Goal

One source of truth for every number a character has: attributes, skills, level, health/magicka/stamina pools,
reputation, bounty, faction memberships, progression counters. Game-agnostic: the content pack defines which stats exist
(data), how pools and derived values are computed (a rules model), and how skills grow (the `SkillUseModel` seam, owner
2026-11-05). Consumers read one API: combat vitals, HUD bars, dialogue filters, MWScript stat functions, the effect
runtime, later character creation and the stats screen.

Non-goals (first cuts): the effect engine (next spec), conditional/tagged modifiers, perks, chargen/level-up UI, stats
screen (UI slices after S9), AI settings (`sf_ai`).

Today: `sf_combat::Vitals` seeded from the `player` record (`mw_player_bevy/src/combat.rs`), HUD `PlayerVitals` a
scaffold, dialogue reads `PlayerStatsPlaceholder` (`mw_rpg_bevy/src/dialogue.rs`), MWScript stat functions are stubs.

## Canonical technique

**Actor values with layered modifiers aggregated from tracked sources, plus a rules layer for derivation and
progression**: Creation Engine actor values and UE GAS attribute sets (details in "Industry comparison"); OpenMW stores
the same layers. Derived maxima and progression sit behind seams so modern and classic rule sets swap them (§1b).

## Reuse (Bevy, then crates, then our own)

- **Bevy** has no stat or attribute system.
- **`bevy_gauge`** (DEMIURGE-studio, MIT/Apache-2.0, 0.5.1 targets Bevy 0.19, 2026-07): string-named attributes on
  entities, string expression modifiers with dependency tracking, tag-filtered modifiers, cross-entity references
  `Name@Entity`. Doesn't fit: the core is Bevy-bound (build-perf.md rule 1: logic in Bevy-free crates), not on 0.20,
  cross-entity refs by `Entity` break net rule 7, string expressions instead of ported, testable Rust formulas, and no
  damage layer or pool-current semantics (OpenMW's `setCurrent` flags, below). Ideas taken: dependency-driven recompute;
  tag filters noted as a later gap.
- Others: `bevy_diesel` (0.1, ability engine on `bevy_gauge`: the effect spec's reuse check), `leafwing_abilities`
  (cooldowns and pools tied to leafwing-input-manager; no attributes or progression), `bevy_attr` (2022, Bevy 0.9,
  abandoned). OpenMW (GPL) is ported in the adapter only: formulas and layering semantics.

So: our own small Bevy-free core (~1.5k lines with tests), shaped after GAS/Creation, OpenMW's semantics expressible.

## Architecture

Three layers: **definitions** (content data: which stats exist), **sheet** (per character state + the pure `apply`
step), **rules models** (traits the content implements: derivation, regeneration, skill use, levelling).

### Crates

- **`sf_stats`** (Bevy-free; deps `sf_ids`, serde, smallvec, thiserror; in hakari `final-excludes`): `StatTable`,
  `Sheet`, `Modifier`, `StatOp`, `apply`, `StatEvent`, the model traits, a `TestModel` for tests. No Morrowind names.
- **`sf_stats_bevy`** (thin): `Stats` component, `StatTableRes`, `StatRules` resource (boxed models), `StatSystems` sets
  in `FixedUpdate`, `StatCommand` queue, `StatEvent` messages, record kind registration. Plugin `SfStatsPlugin`.
- Game side: `mw_stats` (Bevy-free: table from records, GMST snapshot, init, the OpenMW models) and glue in
  `mw_rpg_bevy` / `mw_player_bevy` (stats-adapter.md "Crates").
- `sf_combat` does **not** depend on `sf_stats` (combat.md "The rules seam": the framework's combat never reads stats);
  the game's glue maps vitals both ways ("Consumers").

## Data model

All types derive `Clone, Debug, PartialEq, Serialize, Deserialize` (sheet types also `Reflect` in the shell).

### Definitions (`StatTable`, from content)

`StatDef { key: StatKey, kind: StatKind, group: TagId, public: bool, label: LocKey }`. `StatKey` is a case-folded
string (`"strength"`, `"longblade"`, `"health"`), the **save and wire key**; at load the table assigns dense
`StatIdx(u16)` for runtime arrays. Definitions arrive as `sf_records` records (kind `stat`, RON) so a mod can add a stat;
an adapter may synthesise them from its format (Morrowind: SKIL records). `group` is the content's grouping
(attribute, skill, hidden counter) for UI and queries.

| `StatKind` | State | Modified value | Examples |
|---|---|---|---|
| `Value { min, cap: Option<f32> }` | base, damage | `clamp((eff_base - damage + temp) * (1 + Σmul), min, cap)` | attributes, skills, magnitude stats (resist fire) |
| `Pool { depleted_below, rebase }` | base (max), current | max = `max(0, (eff_base + temp) * (1 + Σmul))`; current separate | health, magicka, fatigue/stamina |
| `Counter { min, max }` | one number, no modifiers | itself | level, bounty, reputation, skill progress, level-up tallies |
| `Derived` | none (cached, never saved) | `DerivedModel::derived` | carry capacity, encumbrance, fatigue term |

`eff_base = base + Σ Permanent-layer modifiers` (Creation's permanent bucket; OpenMW abilities that "affect base
values"); `temp = Σ Temporary-layer Add modifiers`. Reads: `base()`, `effective_base()`, `modified()`, `damage()`;
pools `current()`, `max()`, `ratio()`. `Rebase` (what a pool's current does when its max moves): `KeepRatio` (OpenMW
magicka and fatigue), `KeepCurrent`, `KeepDeficit`. `depleted_below`: the current below which `Depleted` fires (OpenMW
health: `< 1`, `creaturestats.cpp` `setDynamic`).

**Roles** (`RoleMap`, content-supplied): the few stats framework consumers must find without knowing names: `Health`,
`Mana`, `Stamina`, `Level`, `CarryCapacity`, `Encumbrance`, `Bounty`. HUD bars, combat glue and the stats screen read
roles; everything else is by key.

### Sheet (per character)

```rust
pub struct Sheet {
    values: Vec<ValueStat>,     // by StatIdx (only Value stats)
    pools: Vec<PoolStat>,
    counters: Vec<f32>,
    derived: Vec<f32>,          // cache, recomputed when dirty
    mods: SmallVec<[Modifier; 8]>,
    groups: BTreeMap<RecordId, Membership>,
    dirty: bool,                // whole sheet: models declare no dependencies (as built)
    building: bool,             // new or loading: no Rebase until settle()
    changed_since_init: bool,   // saves only touched NPC sheets
}
pub struct ValueStat { base: f32, damage: f32 }            // modified values cached alongside
pub struct PoolStat { base: f32, current: f32 }
pub struct Modifier { stat: StatIdx, layer: Layer /* Permanent | Temporary */, op: ModOp /* Add | Mul */, value: f32, source: SourceId }
pub struct Membership { rank: Option<i32>, reputation: i32, expelled: bool }   // None: not a member
pub struct SourceId { kind: SourceKind /* Effect, Equipment, Ability, Script, Rules, Debug */, id: u64 }
```

- **Numbers are `f32`**, as OpenMW (continuous drain needs fractions); content rounds where it shows integers.
- **Modifiers are aggregated, never mutate-and-undo.** Adding or removing a source recomputes the stats it touches;
  removing a source that was never added is a no-op. This is GAS's aggregator, and it removes OpenMW's and Creation's
  drift class (an effect lost before its "undo" leaves its delta behind forever).
- **Damage is state, not a modifier**: `Damage` raises it, `Restore` lowers it to 0 at least (OpenMW
  `AttributeValue::damage/restore`, `stat.cpp`). Content maps its effect kinds onto damage or modifiers (Morrowind's
  drain = damage on start + restore on expiry; stats-adapter.md "Effect mapping").
- **Groups** (factions, guilds, holds): membership with rank, group reputation, expelled flag. The group's records
  (rank names, requirements, reactions) are content; the sheet only holds the character's standing.

### Operations (`StatOp`) and the pure step

```rust
pub enum StatOp {
    SetBase { stat, value }, ModBase { stat, delta },
    Damage { stat, amount, cap_at_modified: bool }, Restore { stat, amount },
    AddModifier(Modifier), RemoveSource(SourceId),
    PoolAdjust { pool, delta, below_zero: bool, above_max: bool },   // OpenMW DynamicStat::setCurrent flags
    PoolSetCurrent { pool, value }, PoolModMax { pool, delta },       // ModMax: base and current together (OpenMW modDynamicStat)
    CounterSet { stat, value }, CounterAdd { stat, delta },
    Join { group }, SetRank { group, rank }, Expel { group }, ClearExpelled { group }, GroupRep { group, delta, set: bool },
    UseSkill { skill, use_kind: u8, scale: f32 },                      // through SkillUseModel
    RaiseSkill { skill, source: IncreaseSource },                     // trainer, book, script, usage, jail...
    LevelUp { picks: SmallVec<[StatIdx; 3]> },                        // through LevelingModel
    Rest { hours: f32, sleep: bool },                                 // through DerivedModel::rest
}

pub fn apply(table: &StatTable, rules: &Rules, sheet: &mut Sheet, ops: &[StatOp], ctx: &ApplyCtx, out: &mut Vec<StatEvent>);
pub fn tick(table: &StatTable, rules: &Rules, sheet: &mut Sheet, ctx: &TickCtx, out: &mut Vec<StatEvent>);   // regen, recompute
```

- `apply` runs the ops **in order, atomically per call** (an `Absorb` is two calls in one host step, one per owner,
  with the same `cause`). After the ops it recomputes dirty modified values, derived pool maxima
  (`DerivedModel::pool_base`, with the pool's `Rebase`), clamps currents (unless `above_max` put them over), refreshes
  derived caches, and appends events.
- `ApplyCtx { owner: StatOwner, cause: Cause /* carries the tick */, inputs: &Inputs }`; `Inputs` are external
  numbers the content's models need (carried weight, is-player, in combat), supplied by the caller. No clock, no RNG
  inside `sf_stats` (net rule 4); a model that needs randomness gets a seed in `Inputs` (hash of owner, tick).
- **Pure**: same (sheet, ops, ctx) gives the same sheet bytes. Tested with postcard round-trips.

### Events (`StatEvent`, data: net rule 8)

`Changed { owner, stat, old, new, cause }` (modified value or pool current; coalesced to one per stat per `apply`),
`Depleted { owner, pool, cause }`, `SkillUsed { owner, skill, use_kind, gain }` (the hook the research asked for: a
mechanic reports use, a modern progression or a quest can listen), `SkillRaised { owner, skill, base, source }`,
`LevelReady { owner }`, `LevelChanged { owner, level }`, `RankChanged { owner, group, old, new }`, `Expelled { owner, group }`.
`Cause { kind: CauseKind /* Combat, Effect, Script, Dialogue, Rules, Regen, Rest, Load, Debug */, source: Option<SourceId>, tick }`.
`StatOwner = Player(PlayerKey) | Actor(ActorKey)`; both keys move to `sf_ids` first (save.md "Co-op" already plans
`PlayerKey`'s move; slice S0).

## The rules seam

```rust
pub struct Rules { pub derived: Box<dyn DerivedModel>, pub skill_use: Box<dyn SkillUseModel>, pub leveling: Box<dyn LevelingModel> }

pub trait DerivedModel: Send + Sync {
    /// A pool's base (max before modifiers) derived from other stats; `None` = stored base (changed only by ops).
    fn pool_base(&self, s: &SheetRead, pool: StatIdx, inputs: &Inputs) -> Option<f32>;
    /// A `Derived` stat's value (capacity, encumbrance, fatigue term).
    fn derived(&self, s: &SheetRead, stat: StatIdx, inputs: &Inputs) -> f32;
    /// Per fixed tick regeneration (deltas per pool; `dt` = tick length in seconds).
    fn regen(&self, s: &SheetRead, dt: f32, inputs: &Inputs, out: &mut SmallVec<[StatOp; 3]>);
    /// Rest or wait for `hours` (sleep = in a bed or allowed rest).
    fn rest(&self, s: &SheetRead, hours: f32, sleep: bool, inputs: &Inputs, out: &mut SmallVec<[StatOp; 4]>);
}

pub trait SkillUseModel: Send + Sync {
    /// One use: returns the gain (for `SkillUsed`), pushes its ops (progress counter; `RaiseSkill` when a level is due).
    fn use_skill(&self, s: &SheetRead, skill: StatIdx, use_kind: u8, scale: f32, inputs: &Inputs, out: &mut SmallVec<[StatOp; 2]>) -> f32;
    /// Ops for one skill level gained or lost from `source` (base, level progress, tallies; may refuse at a cap).
    fn raise(&self, s: &SheetRead, skill: StatIdx, source: IncreaseSource, out: &mut SmallVec<[StatOp; 6]>);
}

pub trait LevelingModel: Send + Sync {
    fn ready(&self, s: &SheetRead) -> bool;
    /// What the level-up screen offers: number of picks, gain per pickable stat.
    fn offer(&self, s: &SheetRead) -> LevelOffer;
    fn level_up(&self, s: &SheetRead, picks: &[StatIdx], out: &mut SmallVec<[StatOp; 8]>) -> Result<(), Refused>;
}
```

- `SheetRead` is the read-only view (modified values, pools, counters, groups) with the table.
- **`UseSkill` flow**: `apply` calls `use_skill`, emits `SkillUsed` with the gain, applies the returned ops; a
  `RaiseSkill` among them calls `raise` and emits `SkillRaised`. The model owns its counters; the core owns order and events.
- **Which formulas are rules (swappable) and which are content**: see the adapter's table. Rule of thumb from §1b:
  numbers authored records depend on (record init, autocalc, chargen bonuses, faction requirements, effect semantics) are
  content, one implementation; how a character grows and recovers (derived maxima, regen, skill gain, level-up) is a
  feel system behind these three traits.
- `Rules` is chosen per new game (modern profile, classic profile) and stored in the world save's meta as a rule-set
  id, like combat's rules (combat.md "The rules seam").

## The effect seam (for the next spec)

What the effect runtime gets from stats, and nothing more:

- **Persistent effects** (fortify, ability, equipment enchantment, curse): `AddModifier { source: Effect(instance id) }`
  on start, `RemoveSource` on end. `Layer::Permanent` for abilities that count as base (OpenMW `Flag_AffectsBaseValues`),
  else `Temporary`. Instance ids are host-allocated stable ids (net rule 7), so removal survives saves and the wire.
- **Instant and per-tick effects**: `Damage`/`Restore` (attributes, skills), `PoolAdjust` with the two flags
  (damage/restore health, drain below zero, fortify current over max), `PoolModMax`, `CounterAdd`. Per-tick magnitudes
  are the runtime's job (it calls `apply` on the fixed tick).
- **Absorb** = two `apply` calls (target loses, caster gains) in one host step with one `Cause`.
- **Magnitude stats**: numbers the content queries as values (resistances, attack bonus, chameleon, fortify maximum
  magicka) can be `Value` stats with base 0 whose modifiers the runtime adds, so one modifier path, one query path, one
  UI (Creation's choice: resistances are actor values). The effect spec decides per effect; the core supports it at no
  extra cost.
- **Signals back**: `Depleted(health)` (death), pool events (knockdown at fatigue ≤ 0 is content: classic only),
  `Changed` for UI.
- The runtime never edits a `Sheet` field directly; everything goes through `StatOp`, so saves, replication and events
  stay correct.

## Consumers

| Consumer | Reads | Writes |
|---|---|---|
| Combat vitals (`sf_combat::Vitals`) | game glue refreshes `Combatant` vitals from the roles before `CombatSystems::Stats` (health 1:1, stamina by a content scale) | after the combat step, each pool's per-tick difference goes back as `PoolAdjust`, `Cause::Combat` (exactly what combat applied, its clamping included) |
| Combat regen | `CombatRules::regen` stays the stamina regen of `Combatant`s (it knows when regen is open); `DerivedModel::regen` skips the stamina role for them (`Inputs::combatant`) | via the vitals difference |
| HUD bars | roles `Health`, `Mana`, `Stamina`: `current/max` of the local player's sheet | none |
| Dialogue filters | values by key, groups (rank, expelled, reputation), counters (bounty, reputation, level) via the adapter's `MwView` | result scripts write through MWScript |
| MWScript stat functions | values, pools, counters, groups | ops, applied **synchronously** on the host (MWScript reads its own write in the same frame; scripting-adapter.md "MWScript semantics") |
| Effect runtime | everything | ops (above) |
| Mechanics (alchemy, barter, training...) | values | `UseSkill`, `RaiseSkill` (training), `CounterAdd` |
| Character creation, level-up, stats screen (later) | table, sheet, `LevelOffer`, modifier list per stat (tooltips) | requests the host turns into ops |

## Per-tick pipeline (fixed tick, 60 Hz)

`StatSystems` in `FixedUpdate`: **Apply** (drain the `StatCommand` queue: requests validated by the host, effect ticks,
mechanics), before `CombatSystems::Stats` (vitals refresh); **Combat** (vitals differences back), after the combat step;
**Tick** (`tick`: regen for pools below max, recompute dirty sheets, `Depleted` checks); **Events** (publish `StatEvent`
messages). Only dirty sheets recompute; regen visits pools below max only. Presentation (HUD, UI) reads in `Update`.
Budget: 500 resident actors under 0.05 ms per tick on the target laptop (printed by an `#[ignore = "measurement"]` test).

## Saves

- **What is saved**: per stat by `StatKey`: value base and damage, pool base and current, counters; groups; nothing
  else. **Modifiers are not saved**: each source re-registers on load (equipment from the loadout, effects from the
  effect runtime's saved active list, abilities from the spell list), the GAS rule that keeps a sheet equal to its
  sources. Derived caches are recomputed.
- **Load order**: sheets load with clamping suspended; providers re-add their modifiers; then one recompute clamps (a
  fortified current above the unfortified max survives the load).
- **Player**: `CharacterSave` gains `stats: Option<SheetSave>`: `sf.character` **v3**; step v2 → v3 sets `None` and
  the adapter's init hook builds the sheet on first spawn (the new-game path). Frozen `legacy::v2`, shape golden v3.
- **NPCs and creatures**: overlay blob `stats` v1 per placement or spawn, only when `changed_since_init` (an
  untouched NPC re-initialises from its record, as OpenMW stores only changed refs).
- **Key drift**: an unknown key on load (its mod was removed) is dropped and reported (save.md "Mod-list changes"); a
  missing key takes its definition's default through the init hook.

## Co-op (net-ready rules)

1: fixed tick (MWScript calls: the allowed host-frame exception). 2: clients send requests (level-up picks, chargen,
training); the host makes ops. 3: plain serde state. 4: pure, RNG seeded through `Inputs`. 5: host-owned (net.md
"Ownership per system"); own stamina spend stays predicted by combat, corrected by the replicated pool. 6: `StatOwner`,
no singleton. 7: `StatKey`, `SourceId`, `ActorKey`, never `Entity`. 8: events carry tick, cause, owner.
**Replication**: the owner's client gets its full sheet (values, pools, counters, groups; modifier list for tooltips);
other clients get the `public` stats only (health ratio and level by default) through the actor snapshot.

## Industry comparison

| Engine | What it does | Us |
|---|---|---|
| Bethesda Creation (Skyrim, FO4) | Fixed actor-value list (~160 incl. resistances, AI values) defined in records; value = base + permanent + temporary + damage modifiers; Papyrus `GetBaseActorValue`, `ModActorValue`, `DamageActorValue`/`RestoreActorValue`, `ForceActorValue`; regen restores the damage bucket; skills gain XP by use, player level from skill XP; saves store the modifier totals | **Match**: the same four layers (base, permanent, temporary, damage); stats defined as records; magnitude stats like resistances. **Differ**: modifiers aggregated from sources and rebuilt on load instead of saved totals (no drift when an effect is lost); stat set is data, not fixed |
| UE Gameplay Ability System | `AttributeSet` (BaseValue, CurrentValue); `GameplayEffect` instant (changes base), duration/infinite (aggregator: Add, Multiply, Divide, Override), periodic; Modifier Magnitude and Execution calculations; `PreAttributeChange` clamps; meta attributes (incoming Damage applied in `PostGameplayEffectExecute`); attributes replicated, predicted with prediction keys; gameplay tags | **Match**: source-tracked modifiers, aggregator recompute, instant vs persistent ops, clamping in one place, damage as an op not a stored attribute. **Differ**: no Override/Divide (no content needs them), no tags (gap: conditional modifiers, add when an effect needs it), no general prediction (host-only; stamina predicted by combat) |
| OpenMW | `Stat` (base, modifier), `AttributeValue` (base, modifier, damage, `getModified = max(0, base - damage + modifier)`), `DynamicStat` (`setCurrent` with below-zero and above-max flags) (`mwmechanics/stat.{hpp,cpp}`); effects mutate on apply and invert on remove (`spelleffects.cpp`); skill progression in Lua behind handler hooks (`files/data-mw/scripts/omw/playerskillhandlers.lua`, `I.SkillProgression`) | **Match**: the same layers and both pool flags, so Morrowind semantics port exactly; OpenMW's own skill-progression handler seam confirms ours. **Differ**: aggregation instead of mutate-and-invert |
| FromSoftware (Elden Ring) | Fixed attributes; derived maxima and scaling from parameter curves (`CalcCorrectGraph`); level from spent currency | A modern `DerivedModel`/`LevelingModel` can be curve tables; nothing in the core assumes Morrowind's linear formulas |
| Unity (no built-in) | Common pattern (asset-store and community stat systems): flat, then percent-add, then percent-multiply, ordered by priority | **Match**: Add then Mul (summed percentages); no multiplicative "more" stacking (Path of Exile's distinction) until content needs it |

## Tests

Logic and data: exact assertions (CONTRIBUTING "Tests"): layers, sources (add then remove = byte-identical), pool
flags against OpenMW's `setCurrent`, seams through `TestModel`, groups, determinism (postcard bytes, `SheetSave` shape
golden), a tick-cost measurement. The full list: `stats-as-built.md` "Tests".

## Build plan

One deliverable plus its test per slice; S1-S3 are Bevy-free and need no game run.

| Slice | Deliverable | Test |
|---|---|---|
| **S0** ids [x] 2026-11-05 | `ActorKey`, `PlayerKey` move to `sf_ids` (re-exported by `sf_rpg`) | workspace builds; save goldens unchanged |
| **S1** core [x] 2026-11-05 | `sf_stats`: table, sheet, modifiers, ops, `apply`/`tick`, events, model traits, `TestModel` | "Tests" list above (23 + measurement 0.040 ms) |
| **S2** Morrowind table + init [x] 2026-11-05 | `mw_stats`: defs from attributes + SKIL, roles, GMST snapshot, record / autocalc / chargen init; `MwDerived` maxima and derived stats (init needs them) | formula tests against OpenMW (14); real install: 3041 NPCs, 396 creatures, 420 chargen sheets sane |
| **S3** Morrowind models | `mw_stats`: `MwDerived` regen + rest, `OpenMwSkillUse`, `OpenMwLeveling`, faction rank requirements | exact formula tests with vanilla GMSTs |
| **S4** Bevy plugin | `sf_stats_bevy` + game glue: `Stats` on the player, NPCs and creatures at spawn; fixed-tick sets; events | headless app: spawn, apply, regen, events in order |
| **S5** combat + HUD | vitals refresh/difference wiring; `PlayerVitals` and `PlayerCombat` health/fatigue removed; HUD bars from roles | a hit on the dummy lowers its sheet's health; roll cost lowers the player's fatigue; HUD view-model reads the sheet. Visual check HD-x "bars follow damage and regen" (awaiting owner) |
| **S6** dialogue | `MwView::function` and `player_rank` from the sheet; `PlayerStatsPlaceholder` removed | existing filter tests plus PC stat and rank filters |
| **S7** MWScript | stats, faction and crime families `live` (functions.txt) with OpenMW semantics | interp tests per op (adapter "MWScript") |
| **S8** saves | `sf.character` v3 + legacy v2 + step; overlay blob `stats` v1; source re-registration order | golden v2 save loads into v3; modified values equal after a round trip with a fortify active |
| **S9** skill-use sources | combat hits, armor hits, blocks, athletics, acrobatics call `UseSkill` (adapter "Use sources") | counts of `SkillUsed` per scripted fight |

Later, other specs: the effect runtime (next), level-up and chargen UI, stats screen, `DispositionModel` real inputs
(persuasion lane), crime (bounty writers), `mw_rules` classic variants of the three models if the modern ones diverge.

## Open questions (owner)

Answered 2026-11-05: defaults below accepted (owner: "generic in code, Morrowind easy to port"); DECISIONS.md.

1. **First implementations = OpenMW's formulas for all three models**, not only skill use (owner 2026-11-05)? Default
   yes: cheapest to verify, the modern variants (retroactive health, no attribute multipliers, Skyrim-like
   level-from-skill-XP) come later behind the same traits.
2. **Stamina vs fatigue**: one pool (Morrowind fatigue, combat sees it scaled x0.5 as today) or two pools (a modern
   stamina pool for combat and Morrowind fatigue for the classic fatigue term)? Default: one pool, as built today.
3. **Magnitude stats** for resistances and similar effect values (Creation style), or a separate effect-magnitude map
   (OpenMW style)? Default: magnitude stats; decided in the effect spec.
4. **Health growth**: Morrowind's non-retroactive health (Endurance early matters) in the modern profile too? Default
   yes for now (part of question 1).
