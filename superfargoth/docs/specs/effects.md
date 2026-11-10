# Magic effects: definitions, active instances, spell books, casting (`sf_effects`)

Status (2026-11-10): **agreed** (owner 2026-11-08). E1-E3 written 2026-11-08, compiled + unit tests + pedantic clippy 2026-11-09; E4 (`sf_effects_bevy`, game glue, saves) 2026-11-10, headless tests; not run in game ("As built"). Builds on [stats.md](stats.md) "The effect seam" (writes only
through `StatOp` with `SourceId`s) and [rule-hooks.md](rule-hooks.md) "Declaring a rule". First content pack's
mapping: `../../../bevyrrowind/docs/specs/effects-adapter.md`. Blocks: spells, potions, alchemy, enchanting, spellmaking,
MWScript `AddSpell`/`Cast`, stats S8 (source re-registration).

## Goal

One runtime for everything that puts a timed or permanent effect on a character: cast spells, innate abilities,
daily powers, afflictions (diseases, curses), consumables (potions, ingredients), item enchantments (cast once, on
use, on strike, constant). Definitions are records; active effects are plain host-owned data per target, keyed by
stable ids, advanced on the fixed tick, saved by source and re-registered on load. Formulas with randomness or
feel (cast chance, resist, reflect, absorb, magnitude roll, item charge) are rules behind one seam, modern first.
Out of scope: projectiles and their flight (combat/physics later; this spec takes the impact), VFX/sound (presentation
reads events), the crafting screens (alchemy, enchanting, spellmaking: consumers of the cost functions here).

## Canonical technique

Spec-then-instance with aggregated modifiers (Unreal GAS): a **definition** (GameplayEffect / Creation
`MagicEffect`) is applied as a **spec** (rolled magnitude, caster, source) and lives on the target as an **active
instance** with a duration policy (instant, duration, infinite) and optional period; its modifiers go through the
aggregator and are removed by handle, never by inverse arithmetic. Creation and OpenMW group effects by **source**
(OpenMW `ActiveSpellParams`); we keep that two-level shape because dispel, cure, stacking and spell removal act on sources.

## Reuse (Bevy, then crates, then our own)

Bevy: `FixedUpdate`, messages, `Reflect`. Crates: none fits (GAS-like Bevy crates own their attributes, which
`sf_stats` already does, and are not seeded or saveable). Ours: `sf_stats`, `sf_records` (defs, runtime-created
records), `sf_rules`, `sf_ids` (keys, seeded rolls), `sf_rpg::GameTime` (world clock).

## Architecture

### Crates: new `sf_effects` (Bevy-free) + `sf_effects_bevy` (thin), not inside `sf_stats`

- **`sf_effects`**: definitions, effect lists, source definitions, `ActiveSet`, `SpellBook`, `EffectOp`, pure
  `apply`/`tick`, the rules seam and its modern defaults, events, save types. Deps: `sf_stats`, `sf_records`,
  `sf_rules`, `sf_ids`, `serde`, `smallvec`.
- **`sf_effects_bevy`**: `Effects` component (`ActiveSet` + `SpellBook`) on characters, `EffectCommand` queue,
  `EffectSystems` fixed-tick sets, event messages, save hooks. No logic.
- **Why not `sf_stats`**: effects have their own lifecycle (sources, stacking, dispel, books, casting), reach
  non-sheet systems (locks, teleport, summons, AI) through requests, and need the world clock; `sf_stats` stays a
  value store with one write path (its effect seam assumes a separate runtime), as `sf_combat` sits beside it.

## Data model

All types `Clone, Debug, PartialEq, Serialize, Deserialize`; shell types also `Reflect`.

### Effect definitions (`EffectDef`, record kind `effect`)

`EffectDef { key: EffectKey, archetype: Archetype, arg: ArgKind /* None | Stat(group) | Record(kind) */, flags,
school: TagId, base_cost: f32, resisted_by: SmallVec<[StatKey; 2]>, weakened_by: SmallVec<[StatKey; 1]>,
stack: StackPolicy, label: LocKey, presentation: PresentationKeys }`. `EffectKey` is a case-folded string (save and
wire key); the table assigns dense `EffectIdx(u16)`. Content may synthesise defs from its format.

`EffectFlags`: `no_magnitude`, `no_duration`, `applied_once` (lands once, reverted at end; else a rate per second),
`harmful`, `unreflectable`, `caster_linked` (ends if the caster dies or is not an actor), `non_recastable`,
`ranges` (self, touch, target allowed), `crafting` (spellmaking, enchanting allowed). Presentation keys (vfx, sounds,
icon, colour) are read by presentation only (net rule 3).

| `Archetype` | Start | Each tick (rate) | End | Examples (generic) |
|---|---|---|---|---|
| `Modify { stat: StatTarget, layer: LayerRule }` | `AddModifier` (Add) | none | `RemoveSource` | fortify attribute/skill, resist X, attack bonus |
| `MagnitudeStat { stat }` | `AddModifier` on a base-0 `Value` stat | none | `RemoveSource` | reflect, absorb-spell chance, chameleon, silence, paralyze, light |
| `Damage { stat, cap_at_modified }` | (applied_once: `Damage`) | `Damage(m·dt)` | none | damage attribute/skill |
| `Restore { stat }` | (once) | `Restore(m·dt)` | none | restore attribute/skill |
| `Drain { stat }` | `Damage(m)` | none | `Restore(m)` | drain attribute/skill (exact inverse, saved magnitude) |
| `Pool { pool, mode }` | `PoolAdjust` / `AddModifierShift` per mode | `PoolAdjust(±m·dt)` | inverse if `mode` reverts | damage/restore/drain/fortify health, magicka, stamina |
| `Absorb { inner: Box<Archetype> }` | target gets `inner`, caster the inverse, one `Cause` | same | same | absorb health, absorb skill |
| `Cure { filter: CureFilter }` | ends matching sources (instant) | none | none | cure disease/poison/paralysis, dispel, remove curse |
| `Request { kind: TagId }` | `EffectRequest::Start` | `Tick` if periodic | `End` | open/lock, teleport, summon, bound item, levitate, calm/frenzy |

- `StatTarget = Fixed(StatKey) | Arg` (the effect entry picks the attribute or skill). `LayerRule = BySource`:
  `Permanent` when the source class is innate (OpenMW `Flag_AffectsBaseValues`), else `Temporary`.
- `PoolMode`: `Hurt` (no below zero), `Heal`, `Drain` (below zero allowed, reverted), `Fortify` (max and current
  together: `AddModifierShift`, `Permanent` when innate; `RemoveSourceShift` at the end), `Boost` (the current over
  the max, reverted below zero allowed; innate as `Fortify`: OpenMW's Fortify Health / Magicka / Fatigue).
- `CureFilter = Archetype(EffectKey) | SourceTag(TagId) | Dispel { harmful_only, chance_from_magnitude }`.
- `Request` effects are the escape hatch for anything that is not a sheet: the runtime emits data, the game's
  handler acts (lock state, teleport, AI intent, spawning) and may answer `Invalid` (the effect ends, still
  queryable, as OpenMW `Flag_Invalid`). Handlers never mutate the active set directly.

### Effect lists and sources (`EffectList`, `SourceDef`)

`EffectEntry { effect: EffectKey, arg: Option<ArgRef>, range: Range /* Self_ | Touch | Target */, area: f32 /* m */,
duration: f32 /* s, 0 = instant */, magnitude: (f32, f32) /* min, max */ }`; `EffectList = SmallVec<[EffectEntry; 4]>`.

`SourceDef { id: RecordId, class: SourceClass, effects: EffectList, cost: Option<f32>, flags: SourceFlags /*
always_succeeds, auto_cost, starts_known */, tags: SmallVec<[TagId; 2]> }`, built by content from its records.

| `SourceClass` | Lifetime of its instance | Typical stacking |
|---|---|---|
| `Cast` (spell) | entry durations | `Replace` per (source, caster) |
| `Power { recharge: Recharge }` | entry durations; recharge on the world clock (`GameDays(1)`) or a tick cooldown | as `Cast` |
| `Innate` (ability, birthsign, racial) | while in the book | single |
| `Affliction { tag }` (disease, blight, curse) | while in the book; cured by tag | single |
| `Consumable` (potion, ingredient) | entry durations | `Stack` |
| `ItemCast { kind: Once \| OnUse \| OnStrike }` | entry durations; costs item charge | `Replace` per (source, caster, item) |
| `ItemConstant` | while the item is equipped | single per item |

`StackPolicy` (per effect def, defaulted from the source class): `Replace` (same key: old instance ends, new one
starts; OpenMW non-stackable), `Refresh` (keep, reset duration: GAS "duration refresh"), `Stack { limit }`,
`Ignore` (second add is a no-op). `StackKey = BySourceCaster | BySourceCasterItem | BySource` (GAS "aggregate by
source / by target"). `non_recastable` refuses the cast while the source is active on the target.

### Active state per target (`ActiveSet`)

```rust
pub struct ActiveSet { sources: Vec<ActiveSource> /* insertion order */, dirty_rate: bool }
pub struct ActiveSource {
    id: InstanceId,              // u64, host-allocated, saved (net rule 7)
    source: SourceRef,           // { record: RecordId, item: Option<ItemKey> }
    class: SourceClass, caster: Option<ActorRef /* ActorKey | PlayerKey */>,
    started: Tick, flags: InstanceFlags,  // ignore_resist, ignore_reflect, ignore_absorb, from_reflect
    effects: SmallVec<[ActiveEffect; 4]>, // a copy of the rolled entries: a removed record still ends cleanly
}
pub struct ActiveEffect {
    index: u8, effect: EffectKey, arg: Option<ArgRef>,
    magnitude: f32,              // rolled once at creation, after resist; never re-rolled
    duration: Option<u32>,       // ticks; None = lives as long as the source (innate, constant, affliction)
    elapsed: u32, applied: f32,  // amount actually applied (what Drain/Fortify revert), state for re-registration
    phase: Phase,                // Pending | Running | Invalid | Ended
}
```

- **Stats `SourceId`** for an effect: `SourceId { kind: Effect, id: (instance << 6) | index }` (≤ 64 effects per
  source; content lists are ≤ 8). One effect can end before its siblings; `RemoveSource` is exact.
- **Magnitude queries**: `ActiveSet::magnitude(effect, arg)` sums running effects (OpenMW `MagicEffects`, script
  `GetEffect`); values other systems read every tick are `MagnitudeStat`s on the sheet instead (one query path).
- `InstanceIds` come from a host counter in the world save meta (`EffectIdAlloc { next }`).

### Spell book (`SpellBook`, per character)

`SpellBook { known: Vec<RecordId> /* insertion order */, selected: Option<RecordId>, power_used: BTreeMap<RecordId,
GameTime>, cooldowns: BTreeMap<RecordId, Tick> }`. Adding an `Innate` or `Affliction` source starts its instance
(resistances ignored, as OpenMW `ActiveSpellParams(spell, actor, true)`); removing it ends that instance in the same
step. `Cast` and `Power` entries are only castable. The book is the only owner of "which abilities exist"; the
active set mirrors it (OpenMW re-syncs every frame in `ActiveSpells::update`; we sync at the op).

## Lifecycle

### Operations (`EffectOp`) and the pure step

```rust
pub enum EffectOp {
    Inflict { source: SourceRef, caster: Option<ActorRef>, entries: EffectList, flags: InstanceFlags, rolls: RollSeed },
    Cast { source: RecordId, target: Option<ActorRef>, rolls: RollSeed },  // book check, cost, chance, then Inflict
    UseItem { item: ItemKey, source: RecordId, kind: ItemCastKind, target: Option<ActorRef>, charge: f32, rolls: RollSeed },
    Equip { item: ItemKey, source: RecordId }, Unequip { item: ItemKey },
    Learn { source: RecordId }, Forget { source: RecordId }, Select { source: Option<RecordId> },
    End { instance: InstanceId, reason: EndReason }, EndBySource { record: RecordId }, EndByCaster { caster: ActorRef },
    Cure { filter: CureFilter, rolls: RollSeed }, Skip { seconds: f32 /* rest, wait, travel */ }, Clear,
}
pub fn apply(defs: &EffectTable, rules: &EffectRules, set: &mut ActiveSet, book: &mut SpellBook, target: SheetRead,
             caster: Option<SheetRead>, ops: &[EffectOp], ctx: &EffectCtx, out: &mut EffectOut);
pub fn tick(defs: &EffectTable, set: &mut ActiveSet, ctx: &EffectCtx, out: &mut EffectOut);
pub struct EffectOut { stat_ops: Vec<(StatOwner, StatOp, Cause)>, requests: Vec<EffectRequest>, events: Vec<EffectEvent> }
```

`EffectCtx { target: ActorRef, tick: Tick, dt: f32, time: GameTime, ids: &mut EffectIdAlloc }`. The step reads
sheets, never writes them: it returns `StatOp`s the host applies through `sf_stats::apply` in the same tick (an
`Absorb` gives two owners, one `Cause { kind: Effect, source }`). **Pure**: same inputs, same bytes and outputs.
Randomness only through `RollSeed` (hash of instance id, effect index, tick, salt; `sf_ids` roll helper).

### Application pipeline (one source lands on one target)

1. **Filter** entries by range (self / touch / target) and area membership; drop `caster_linked` without an actor caster.
2. **Recast and stacking**: `non_recastable` + active → refuse (`CastRefused`); `StackPolicy` decides replace,
   refresh, stack or ignore by `StackKey` (OpenMW `addToSpells`, `merge`).
3. **Protection** (temporary, hostile-capable sources from another actor only): rule `protect` → `Applied`,
   `Reflected` (re-inflict on the caster with `from_reflect`, no second reflect), `Absorbed` (source dropped, the
   target gains the content's absorb amount as a `PoolAdjust`).
4. **Magnitude**: rule `magnitude` per entry (once), then **resist** rule per entry → multiplier; 0 = `Resisted`
   event, entry dropped; `no_magnitude` effects are resisted all-or-nothing.
5. **Allocate** `InstanceId`, push `ActiveSource`, emit start ops per archetype (table above), `Applied` events.

### Instant, duration, periodic

- **Instant** (`duration == 0` or `no_duration`): start ops, `Ended { Expired }` the same tick; nothing kept
  (GAS instant; cures, dispel, restore once).
- **Applied once** (`applied_once`): start ops, kept until `elapsed == duration` or the source ends, end ops revert
  `applied` exactly.
- **Rate** (default for damage/restore/pool effects): per tick `amount = magnitude · dt`, last tick clipped
  (`min(remaining, dt)`, OpenMW `applyMagicEffect`), so the total is exactly `magnitude · duration`. Durations are
  ticks (`round(s · 60)`); the per-tick amount is fixed at creation.
- **Periodic** (`period: Option<u32>` on the def, modern content): one pulse of `magnitude` every `period` ticks
  (GAS period); classic content uses rate.
- **Skip** (rest, wait, travel): closed form, one step: rate effects deliver `magnitude · min(remaining, s)`, expiries
  fire in instance order (OpenMW `Actors::rest` advances by `hours · 3600 / timescale` in one update).

### Removal

By instance (`End`), by source record (`EndBySource`: `RemoveSpell`, unequip), by caster (`EndByCaster`: caster
death for `caster_linked`), by filter (`Cure`: archetype, source tag, dispel with a chance per source from the
magnitude), `Clear` (death, debug). Every path emits end ops then `Ended { reason }`. Removing an unknown instance
is a no-op (aggregation: nothing to drift).

## The rules seam

```rust
pub struct EffectRules { pub cast: Box<dyn CastModel>, pub land: Box<dyn LandModel>, pub items: Box<dyn ItemModel> }
pub trait CastModel: Send + Sync {
    /// Cost, chance (0..=1) and cooldown of casting `source` now; refusal reasons (silenced, no magicka, recharging).
    fn plan(&self, q: &CastQuery) -> CastPlan;
}
pub trait LandModel: Send + Sync {
    fn magnitude(&self, q: &RollQuery) -> f32;          // once per entry
    fn protect(&self, q: &ProtectQuery) -> Protection; // Applied | Reflected | Absorbed { gain }
    fn resist(&self, q: &ResistQuery) -> f32;          // magnitude multiplier, 0 = fully resisted
}
pub trait ItemModel: Send + Sync {
    fn charge_cost(&self, q: &ItemQuery) -> f32;       // per use / per strike
    fn max_charge(&self, q: &ItemQuery) -> f32;
}
```

- Queries carry `SheetRead`s (caster, target), the defs, the source and the **uniform rolls** the runtime drew
  (`rolls: [f32; 2]`). Each implementor gathers its numbers into one rule input and calls a pure `fn(&In) -> Out`
  (rule-hooks.md: the whole input in one struct, GMSTs as fields, outputs numbers or small records, never ops; no
  `sf_rules` registry yet, owner 2026-11-08: Rust first, the id lives in the fn's doc). Framework rules: `sf.effects.cast`, `sf.effects.magnitude`, `sf.effects.protect`,
  `sf.effects.resist`, `sf.effects.item_charge`; content declares its own (`mw.effects.*`) for its formulas.
- **Modern defaults (`StandardEffectRules`)**: casting never fizzles (chance 1 when the pool covers the cost; the
  skill lowers the cost); magnitude uniform in `[min, max]`; resist deterministic `1 - clamp(resist - weakness, -100,
  100)/100`; reflect and spell absorption roll their percentage; cooldown per source in ticks (`SourceDef` field).
- **Content, not rules**: what an archetype does to stats, which stat resists which effect, stacking, cure filters,
  the 24 h power recharge (content semantics, one implementation, ARCHITECTURE §1b).
- `EffectRules` is chosen per new game and stored in the world meta's rule-set id, like `Rules` and `CombatRules`.

## Casting

`Cast` op: the source must be in the book (scripts use `Inflict` directly: OpenMW `mScriptedSpell` skips book, chance
and cost). `CastModel::plan` → refusal (`CastFailed { reason }`, no cost) or `{ cost, chance, cooldown }`; the host
spends `cost` with `PoolAdjust` on the caster's `Mana` role; roll `rolls[0] < chance` else `CastFailed { Fizzle }`
(cost spent); powers record `power_used = time`; then delivery: self entries `Inflict` on the
caster now; touch entries on `target` (the caller's contact: a melee-range ray or combat hit volume); target entries
are handed to the projectile owner (`EffectRequest::Launch { source, entries, caster }`) which later calls `Inflict`
on impact. **Area**: on landing at a point, the caller passes the candidates within the largest area sorted by stable
id; each entry lands on those within its own radius. The cast animation and its release frame are the action layer's
(action.md); this op runs at the release tick.

Items: `UseItem` spends charge through `ItemModel` (charge is `sf_rpg` item state; the new value comes back as an
event); the caller consumes `Once` items; `OnStrike` follows a landed combat hit (combat.md "The rules seam");
`Equip`/`Unequip` keep `ItemConstant` instances equal to the loadout.

## Events (data, net rule 8)

`EffectEvent`: `Applied { target, instance, effect, magnitude, caster, tick }`, `Resisted`, `Reflected { back_to }`,
`Absorbed`, `Ended { target, instance, effect, reason: Expired | Dispelled | Cured | SourceRemoved | Unequipped |
CasterGone | Invalid | Cleared }`, `CastSucceeded { caster, source, tick }`, `CastFailed { caster, source, reason }`,
`CastRefused`, `PowerUsed`, `BookChanged { owner, source, added }`, `ChargeChanged { item, charge }`.
`EffectRequest { kind: TagId, phase: Start | Tick | End | Launch, target, caster, instance, magnitude, arg }`.
Presentation (VFX, sounds, HUD icons, messages) reads events and the active set, never writes them.

## Per-tick pipeline (fixed tick, 60 Hz)

`EffectSystems` in `FixedUpdate`, after the action layer (cast release) and combat (strikes), before
`StatSystems::Apply`: **Commands** (the `EffectCommand` queue in order: casts, item uses, equips, script and book
ops); **Tick** (expiries, rate effects; only sets with timed effects); **Requests** (game handlers; answers return as
ops next tick); **Out** (stat ops into the same tick's `StatCommand` queue; events in command order, then by
`ActorRef`). Script ops run synchronously on the host (the allowed exception), as in stats.

**Clocks** (E4; ROADMAP 1.17): durations, rates and cooldowns count fixed ticks (`Time<Fixed>`, numbered by
`StatTick`; real seconds as OpenMW, no timescale); power recharges read `EffectClock(GameMinutes)`, fed by the game
from its calendar. The 1.17 fix writes `EffectClock` (and `StatTick`) from the one host clock; `sf_effects` is unchanged.

## Saves

- **What is saved** per character (`EffectsSave { tick, active, book }`): `SpellBook` (by record id) and the whole
  `ActiveSet`, each effect with `magnitude`, `elapsed`, `applied`, `phase`; `started` and cooldowns are rebased from
  `tick` on load. Innate sources are kept (E4): re-landing them from the book would re-apply base changes the sheet
  already holds (a pool fortify's shifted current) and churn ids; E5 decides whether `ItemConstant` sources are
  rebuilt from the loadout instead. `EffectIdAlloc` in the world section **`sf/effects`** v1 (E4: `sf/meta` belongs to
  `sf_save` and would need a `META_VERSION` bump; owner question); restored sets also move it past their ids.
- **Load order** (stats S8): sheets load clamped-off; the effect runtime **re-registers modifiers only**
  (`sf_effects::reregister`: `AddModifier` of running effects, shifting ones as plain; no `Damage` or `PoolAdjust`:
  those results are already in the saved sheet) on the building sheet, then one settle. The sheet and its effects load in one step
  (player at `CHARACTERS` order, NPCs at spawn), so nothing runs between them.
- **Player**: `CharacterSave` gains `effects: Option<EffectsSave>`: `sf.character` **v4**; step v3 → v4 sets `None`,
  and the adapter's init hook builds the book from the character's records (new-game path). Frozen `legacy::v3`.
- **NPCs and creatures**: overlay blob `effects` v1 when the book differs from the record or the active set is not
  empty; on unload the same blob is written (one path for unload and save) and timers resume on reload (no time passes
  while unloaded). OpenMW purges temporary effects on unload instead (owner question).
- **Migrations / drift**: an unknown `EffectKey` (mod removed) drops that effect with a report; an unknown source
  record keeps the instance (it carries its entries) until it ends; unknown book entries are dropped and reported.

## Co-op (net-ready rules)

1: fixed tick; durations in ticks; MWScript the allowed exception. 2: clients send intents (cast, use item, equip,
select spell); the host validates and makes ops. 3: plain serde; presentation reads events. 4: pure; rolls from
`RollSeed`, never a thread RNG. 5: host-owned, not predicted (the cast animation is predicted by the action layer; a
refused cast cancels it). 6: `ActorRef`, no singleton player; "player" in rules is a query field. 7: `InstanceId`,
`RecordId`, `ItemKey`, `ActorRef`, never `Entity`. 8: events carry tick, source, target, caster.
**Replication**: a client gets its own book and active set (spell menu, HUD icons); for other actors only
`Applied`/`Ended` events (VFX). Sheet changes replicate through stats.

## Performance budget

500 resident actors, 2 000 active effects, 50 rate effects: `tick` under 0.03 ms per fixed tick on the target laptop
(sets without timed effects skipped by a flag), plus the stat ops it emits (stats' budget); an `Inflict` under 5 µs.
Printed by `#[ignore = "measurement"]` tests. An `ActiveSource` with 4 inline effects ≈ 200 B.

## Industry comparison

| Engine | What it does | Us |
|---|---|---|
| UE GAS ([Epic](https://dev.epicgames.com/documentation/en-us/unreal-engine/gameplay-effects-for-the-gameplay-ability-system-in-unreal-engine), [tranek](https://github.com/tranek/GASDocumentation)) | Instant / duration / infinite, period, stacking (source or target, limit, refresh), magnitude calcs, executions, tags, immunity, removal by handle, prediction | **Match**: spec vs instance, policies, period, stacking with limit and refresh, removal by handle and query, caller-supplied rolls. **Differ**: no tags or immunity (gap, medium: resist and `CureFilter` cover the first pack); no prediction (host-only) |
| Creation ([CK: Magic Effect](https://ck.uesp.net/wiki/Magic_Effect)) | Archetyped `MagicEffect`s, flags, casting type, delivery, resist value, conditions; spell types; Papyrus events | **Match**: archetype enum, flags, actor-value resistance, spell types as source classes, scripts via `Request`. **Differ**: no per-effect conditions (gap, low); no concentration casting (gap, medium: periodic effect refreshed while held) |
| OpenMW ([`activespells.cpp`](https://github.com/OpenMW/openmw/blob/ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f/apps/openmw/mwmechanics/activespells.cpp)) | `ActiveSpellParams` of `ActiveEffect`s, mutate-and-invert, abilities re-synced each frame | **Match**: two-level model, flags, stacking key (source, caster, item), clipped last tick, reflect/absorb before resist. **Differ**: aggregation; magnitude rolled once (OpenMW re-rolls rates); host op, not re-sync; NPC effects kept across unload |

## Tests

Exact assertions, synthetic defs, `TestRules` (fixed rolls): fortify adds/removes by source; drain restores exactly
what it damaged after a `Restore`; rate damage totals `m · d` with the clipped last tick; `Skip` equals ticking;
instant ends the same tick; replace, refresh, limit, ignore, `non_recastable`; reflect once; resist 0 drops; absorb
gives two owners, one cause; cure by archetype/tag; dispel chance; book sync; equip/unequip; power recharge on
`GameTime`; refused vs fizzled; save round trip re-registers modifiers only (sheet bytes equal); unknown key
reported; same seed, same bytes.

## Build plan

One deliverable plus its test per slice; E1-E3 are Bevy-free.

| Slice | Deliverable | Test |
|---|---|---|
| **E1** core (written) | `sf_effects`: stat archetypes, `ActiveSet`, `Inflict`/`End`/`EndBySource`/`Skip`, `apply`/`tick`, stacking, events, `TestRules` | lifecycle + stacking + determinism tests above |
| **E2** rules (written) | `EffectRules` traits, `StandardEffectRules`, `sf.effects.*` rule declarations | exact formula tests; rule In/Out round trip |
| **E3** book + casting (written) | `SpellBook`, `Learn`/`Forget`/`Cast`/`Cure`, powers on game time, cooldowns, `Request` archetype | book sync, cast refused/fizzle/success, recharge |
| **E4** Bevy + saves (built) | `sf_effects_bevy` (component, sets, commands, events); `sf.character` v4, NPC blob v1, re-registration order (stats S8) | headless app; save round trip with a fortify and a drain active; old goldens load |
| **E5** items + delivery | `UseItem`, `Equip`/`Unequip`, `ItemModel`, touch and area landing, `Launch` request; on-strike from combat outcomes | charge spent; area radius filter; constant effect follows the loadout |

Content slices (first effect set, consumables, abilities, script functions, enchantments, afflictions): adapter spec.

## As built

Detail per slice: [archive/effects-history.md](archive/effects-history.md) (E1-E4b). Open questions: answered
2026-11-08. E4 (2026-11-10): `sf_effects_bevy` (`Effects`, `EffectCommand` in, messages out, sets before
`StatSystems::Apply`), saves (`EffectsSave`, `reregister`, `sf.character` v4, `sf/effects` v1, NPC blob `effects`
v1), glue `mw_rpg_bevy::effects`. 0.089 ms effect tick at 500 characters / 2 000 effects (dev profile). Pool fortifies fixed
2026-11-10 (E1 double count on `KeepRatio` pools, A3 innate fortify lost on a derived pool): `Fortify` is a shifting
modifier, `Boost` added (DECISIONS 2026-11-10).
