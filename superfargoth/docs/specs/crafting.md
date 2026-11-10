# Crafting sessions and created records (`sf_crafting`, `sf_rpg_ui`, `sf_ui_screens`)

Status: spec written 2026-11-10; nothing built. Builds on [effects.md](effects.md) (`EffectList`, `SourceDef`,
`SpellBook`, `ItemModel`), [services.md](services.md) (`ServiceShape::Custom`, `Currency`, `Purse`, `DealVeto`,
the host-owned session shape), [rpg.md](rpg.md) (items, `Attrs`, `RpgCommand`), [stats.md](stats.md) "Consumers"
(`UseSkill`), [save.md](save.md) (`sf/records`), [rule-hooks.md](rule-hooks.md) (pure rules), [net.md](net.md)
"Net-ready rules". First user: Bevyrrowind ROADMAP 2.7 ([crafting-adapter.md](../../../bevyrrowind/docs/specs/crafting-adapter.md)).

<!-- agent-summary -->
> **Agent summary.** One generic crafting session for "combine inputs, get an item or a record": the adapter
> declares a `CraftKind` (slot layout, effect-editor limits, rules); a host-owned `CraftSession` stages slots and
> an effect draft, `preview()` is pure, `commit()` validates, pays, rolls and returns a `CraftSettlement`
> (`RpgCommand`s, `CreatedDraft`s, `CraftEffect`s such as skill use). Records made in play (potions, spells,
> enchantments, enchanted item variants) go through the existing `sf/records` overlay with a fingerprint dedup
> index, so they save, load and mod-patch like content. Alchemy, enchanting, spellmaking and recharge are four
> kinds of this one session; spellmaking and NPC enchanting are `ServiceShape::Custom` offers. Read next:
> "Design", "Created records", "Co-op", "Build plan".

## Goal

Alchemy, enchanting (with soul gems and recharge) and spellmaking are the same loop: the player stages inputs,
sees an exact preview (effects, cost, chance, price), commits once, and gets items or new records, possibly with
a failure that still consumes inputs. A game supplies slot layouts and pure rules; the framework supplies the
session, the staged commit, created records that survive saves and mod changes, the skill-use hook, view-models
and screens.

Non-goals: the formulas and effect semantics (adapter), the effect runtime (effects.md), disenchanting, repair
(a `ServiceEntry`, services.md), item upgrades (smithing: a later kind of the same session), crafting from a
recipe book (a `Recipe` kind is sketched in "Recipes" but not built). No world crafting stations as entities
(`CraftOpener::Station` only names the hook).

## Canonical technique

Every shipped RPG crafts through one of two shapes, and we support both with one session:

1. **Free combination** (Morrowind alchemy: shared ingredient effects; Oblivion/Skyrim alchemy: any 2-3
   ingredients, shared effects). The output is a function of the inputs and the crafter's skill; nothing is
   authored per recipe. Preview shows the result (Morrowind: only the effects the skill reveals).
2. **Designer-built records** (Morrowind spellmaking and enchanting, Oblivion altar of spellmaking/enchanting,
   Skyrim enchanting): the player edits an effect list within limits; cost, capacity and chance derive from the
   effects; the result is a *created record* (Bethesda's "created objects"; OpenMW inserts dynamic records into
   the ESM store, `ESMStore::insert`, and reuses an identical dynamic record instead of making a duplicate:
   `alchemy.cpp` 239-281, `enchanting.cpp` 232-267).
3. **Recipe lookup** (Elden Ring cookbooks, Valheim, Skyrim smithing): inputs named by the recipe, output fixed.
   Not needed by 2.7; the session's slot filter and fixed-output rule cover it (see "Recipes").

Common spine in all engines: staged inputs, a deterministic quote, one commit that consumes inputs, a skill-use
call on success. That is the session.

## Reuse (Bevy, then crates, then our own)

Bevy: nothing for crafting; its UI and `FixedUpdate` are what the screens and host use. Crates: no crafting
crate fits (inventory/crafting crates for Bevy own their item model, none saves created records or runs a rules
seam); `blake3` (already a dependency of `sf_records` fingerprints). Ours, all existing: `sf_records`
(`RecordDb::define_value`, `fingerprint`, `PatchOp`/`apply_ops`), `sf_save_bevy::sections::records`
(`RecordOverlay::create`, `SAVE_NAMESPACE`, built and tested), `sf_effects` (`EffectList`, `entry_cost`,
`SourceDef`), `sf_rpg` (`Attrs`, `RpgCommand`, `service::{Currency, Purse, DealVeto}`), `sf_ids` (seeded rolls),
`sf_stats` (`UseSkill`). **The research note's "runtime records are the real framework work" is mostly done**:
the overlay exists. What is new: the get-or-create dedup, a Bevy-free front for the overlay, live registration of
created records with the running tables, and the session. Section "Created records".

## Design

### Crate and layers

New **`sf_crafting`** (Bevy-free; deps `sf_rpg`, `sf_effects`, `sf_records`, `sf_ids`, `serde`, `smallvec`);
view-models in `sf_rpg_ui` (`CraftView`, `EffectEditorView`), screens in `sf_ui_screens`; host wiring in the game
crate (as services). Same split as services (`sf_rpg::service` + `sf_rpg_ui` + `sf_ui_screens`): logic never
imports Bevy (build-perf).

### Kinds, layouts and slots

```rust
pub struct CraftKind(pub RecordId);               // opaque: "mw.alchemy", "mw.enchanting"; never matched here
pub struct SlotId(pub u8);

pub struct SlotSpec {
    pub id: SlotId, pub role: SlotRole,           // Input | Tool | Catalyst (consumed, may fail) | Target (item to change)
    pub accepts: ItemFilter,                      // opaque tag set + adapter predicate id, not Morrowind types
    pub required: bool,
    pub unique_item: bool,                        // two slots may not hold the same item record (alchemy)
    pub batch: bool,                              // limits `count` to the smallest stack in batch slots
}
pub struct CraftLayout { pub slots: SmallVec<[SlotSpec; 8]>, pub editor: Option<EditorLimits>, pub naming: Naming,
                         pub batch_max: u32, pub provider: ProviderUse }
pub struct EditorLimits { pub max_entries: u8, pub distinct_effects: bool,   // Morrowind: 8; one entry per effect w/o argument
                          pub magnitude: (f32, f32), pub duration: (f32, f32), pub area: (f32, f32) }
pub enum Naming { None, Required, Suggested }     // potion/spell/item name (UTF-8, trimmed, 1..=64 chars)
pub enum ProviderUse { None, Optional, Required } // an NPC service or a self-service
```

`CraftRules::layout(kind, ctx)` returns the layout (it may depend on the staged target: an enchant target of
class "ammo" changes `batch`). `ItemFilter` is evaluated by `CraftRules::accepts(slot, stack)`; the framework
passes tags/attrs, never item types.

### Session (host-owned)

```rust
pub struct CraftSessionId(pub u32);               // monotonic per host run; not saved
pub struct CraftSession {
    pub id: CraftSessionId, pub player: PlayerId, pub kind: CraftKind,
    pub provider: Option<ActorKey>,               // NPC service (spellmaking, enchanting for gold); None = self
    pub opener: CraftOpener,                      // Service(offer) | Item(StackId) | Station(PlacementId)
    pub fills: SmallVec<[(SlotId, SlotFill); 8]>, // SlotFill { holder, stack, item, attrs } (one unit each)
    pub draft: EffectDraft,                       // effect list being built (designer kinds)
    pub cast_style: u8,                           // adapter-defined mode index (enchant: once/strike/used/constant)
    pub name: String, pub count: u32,             // batch count
    pub attempts: u16,                            // feeds the roll seed
}
```

Operations (all pure over the session and a read-only `CraftWorld`, which is `Holdings` plus `RecordDb` reads
and the crafter's `SheetRead`):

- `fill(slot, holder, stack)` / `clear(slot)`: re-checks `accepts`, `unique_item`, that the stack exists and is
  not staged elsewhere. Staging moves nothing (like the trade session; cancel is free).
- `edit(EffectEdit)`: `Add(effect, arg)`, `Set(index, magnitude, duration, area, range)`, `Remove(index)`,
  `Style(n)`, `Rename`. Bounds come from `EditorLimits`; the adapter's `known_effects` gates `Add`.
- `preview(world, rules) -> CraftPreview`: **pure and total**, never refuses: it reports what would happen.
  ```rust
  pub struct CraftPreview {
      pub effects: EffectList,          // as it will be created (alchemy: computed; designer kinds: the draft)
      pub shown: SmallVec<[bool; 8]>,   // per-entry visibility to this crafter (alchemy hides unrevealed effects)
      pub cost: Option<f32>,            // per-use cost the item or spell will have
      pub capacity: Option<(f32, f32)>, // (used, max): enchant points vs the item's capacity
      pub chance: Option<f32>,          // 0..=1 of success; None = cannot fail
      pub price: Option<i64>,           // provider fee for this commit
      pub max_count: u32,               // how many units the staged inputs allow
      pub issues: SmallVec<[CraftIssue; 4]>, // blocking: NoTool, NoName, NoEffects, OverCapacity, CantAfford...
  }
  ```
- `commit(world, rules, roll) -> Result<CraftSettlement, CraftRefusal>`, in this order (OpenMW dialogs check the
  same list in the same order, `enchantingdialog.cpp` 304-345, `alchemywindow.cpp` 120-157):
  1. `NothingStaged` / first blocking `issue` from the preview (a refusal changes nothing; the session stays open).
  2. **Re-validate** every fill against current holdings (`StockChanged`); no partial crafts.
  3. `rules.vet(session)`: stolen-goods style veto (reuses `DealVeto`: confiscation effects, closes the session).
  4. `CantAfford` if the provider fee exceeds the player's currency.
  5. Roll: per unit, `roll_i = seeded(player, session, attempt, tick, i)`; `rules.outcome(&In, roll_i)` gives
     `Success | Fail(reason)`.
  6. Build the settlement: inputs removed (`RemoveItem`/`SetStackAttr`, consumed on failure too when the rule
     says so), `CreatedDraft`s for the successes, outputs `AddItem` (the id is filled by the host after
     `get_or_create`, see below), fee `Pay`, `purse_delta`, `UseSkill` effects.
  ```rust
  pub struct CraftSettlement {
      pub commands: Vec<RpgCommand>,       // inputs out, outputs in (outputs reference `CreatedRef`s)
      pub created: Vec<CreatedDraft>,      // records to get-or-create before the commands apply
      pub effects: Vec<CraftEffect>,       // Pay, UseSkill{stat,use_kind,scale} per success, Learn(spell), Adapter
      pub purse_delta: i64,
      pub outcome: CraftOutcome,           // Done{made, failed} | AllFailed(reason) | Vetoed
  }
  ```
- `cancel()` drops the session; nothing moved.

`CraftEffect::Learn(RecordId)` adds a spell to the crafter's `SpellBook` (spellmaking). `UseSkill` is the stats
hook (`StatOp::UseSkill`, stats.md "Consumers"): one per successful unit, as OpenMW calls
`skillUsageSucceeded` once per brewed potion (`alchemy.cpp` 573-595).

### Created records (the real framework work)

Existing, built: `sf_save_bevy::sections::records::RecordOverlay` (`create(kind, stem, &T) -> save:<stem>.<n>`,
`patch`, `apply`), saved as section `sf/records`, applied after all mods as one more source; skipped entries are
reported and kept. Missing, in this spec:

1. **A Bevy-free front.** `sf_crafting::Created` wraps `{ overlay: RecordOverlay, index: FingerprintIndex }`
   behind `get_or_create(&mut self, db: &mut RecordDb, source: SourceId, draft: &CreatedDraft) -> Result<RecordId>`.
   (The overlay type stays where it is; `sf_crafting` takes it as a trait `CreatedStore` implemented by the
   game crate, so `sf_crafting` does not depend on `sf_save_bevy`. If `RecordOverlay` moves into `sf_records`
   later, the trait goes away.)
2. **Get-or-create dedup** (OpenMW `Alchemy::getRecord`, `Enchanting::getRecord`: reuse an existing *dynamic*
   record with identical stats, never a content record, so brewing cannot mint quest items). Ours: the index
   maps `blake3(kind, canonical value)` to the first created id; built on load by fingerprinting the overlay's
   crafting-kind records (`RecordDb::fingerprint`), so it is not saved. Two identical potions stack because they
   share an id; "same name, different effects" does not collide.
3. **Derived items.** An enchanted item is the base item record with changes (`mwclass/weapon.cpp` 256-269:
   copy, new name, enchant id, flags). `CreatedDraft::Derived { kind, base: RecordId, ops: Vec<PatchOp>, stem }`
   resolves `base` from the db (`value_of`), applies `ops` with `sf_records::patch::apply_ops`, and creates the
   result. Field-path ops survive struct changes (save.md); a missing base at load skips and reports that record,
   keeping the entry for a returning mod.
4. **Live registration.** After `define_value` the host must make the record visible to its consumers in the same
   tick: the `sf_effects` `SourceDef` table (a created potion/spell/enchantment is a `SourceDef`), the item
   defs, the spell list UI. One host function `register_created(id, kind)` calls the adapter's
   `CraftRules::map_created(kind, value) -> CreatedMapped` (source defs, item def); this is the adapter's
   existing record-to-def mapping run on one record, not a new path (effects-adapter "Records to SourceDef").
5. **No garbage collection** in v1 (OpenMW never frees dynamic records). Budget: a record is ~300 B saved; 5,000
   created records are 1.5 MB in the save. A later `sf/records` compaction can drop created records referenced by
   no inventory, spell book or placement (owner question 5).
6. **Ids.** `save:<stem>.<n>` from `RecordOverlay::next_id`; `stem` is the kind ("potion", "spell", "ench",
   "item"). Ids are stable (net rule 7) and cross the wire as `RecordId`; co-op clients receive created records
   as `CreatedRecordEvent` (below) before any item that references them.

### Items: charge and soul

Per-instance item data lives in `Attrs` (adapter-defined keys, services.md). Crafting adds two conventional
keys the rules read and write through `CraftRules::{charge_key, soul_key}`: `charge` (`AttrValue::Float`; absent
= full, OpenMW's `-1` sentinel, `spellcasting.cpp` 254-256) and `soul` (`AttrValue::Id`: the trapped source
record, or `Int` for a bare number in a game with no source records). `SetStackAttr` already writes both. A
used-up soul gem is `RemoveItem`; a gem that refills itself (Azura's Star) is an adapter flag in the settlement.
Charge spending on use is `ItemModel::charge_cost` (effects.md); crafting only creates and refills charge.

### Knowledge

What a crafter may put into a designer kind is `CraftRules::known_effects(sheet, book) -> Vec<EffectKey>`,
normally derived from the `SpellBook` (every effect of a known spell that the effect def flags as craftable,
`EffectDef.crafting`). Ingredient reveal (alchemy) is `CraftRules::reveal(sheet, index) -> bool`, per preview
entry. Skyrim-style discover-by-use is a different `reveal` that reads a per-player discovery set (a saved
`sf/crafting` section, owner question 1); the session does not care which.

### Rules seam

```rust
pub struct CraftRulesSet { pub rules: Box<dyn CraftRules> }
pub trait CraftRules: Send + Sync {
    fn layout(&self, kind: &CraftKind, q: &LayoutQuery) -> CraftLayout;
    fn accepts(&self, slot: SlotId, stack: &StackView) -> bool;
    fn known_effects(&self, q: &KnownQuery) -> SmallVec<[EffectKey; 32]>;
    fn preview(&self, q: &CraftQuery) -> CraftPreview;        // gathers one rule input, calls a pure fn(&In) -> Out
    fn outcome(&self, q: &CraftQuery, roll: f32) -> CraftOutcomeUnit;
    fn build(&self, q: &CraftQuery, ok: u32) -> BuildResult;  // CreatedDrafts + output stacks + skill uses
    fn vet(&self, q: &CraftQuery) -> Option<DealVeto>;
    fn map_created(&self, kind: &str, v: &Value) -> CreatedMapped;
    fn charge_key(&self) -> RecordId; fn soul_key(&self) -> RecordId;
}
```

Each implementor gathers its numbers into one input struct and calls a pure `fn(&In) -> Out` with GMSTs as
fields and rolls as inputs (rule-hooks.md; ids in the fn's doc, no registry yet). Framework rule ids with modern
defaults (`StandardCraftRules`, used when a game declares none): `sf.crafting.chance` (smooth
`clamp(skill_term - load, 0.05, 1)`, never 0 so a low-skill crafter can still try), `sf.crafting.capacity`
(item capacity = item value x mult), `sf.crafting.fee`. Content declares its own (`mw.alchemy.potion`,
`mw.enchant.result`, already named in rule-hooks.md "First rules"). **Modern first, classic behind the seam**:
the adapter's classic column (OpenMW rules) and modern column are two `CraftRules` implementations or option
flags on one (crafting-adapter.md "Options").

### Recipes (sketch only)

A fixed-output kind is a `Recipe { id, inputs: [(ItemFilter, count)], output, skill, station }` record read by a
`RecipeRules: CraftRules` whose `preview` matches staged slots to a recipe. Elden Ring-style cookbooks are a
`Known` set of recipe ids. Not built until a game needs it; the session API does not change.

### Services integration

`ServiceShape::Custom(RecordId)` offers (services.md) open crafting kinds: `mw.spellmaking` opens
`CraftKind("mw.spellmaking")`, `mw.enchanting` (NPC) opens `mw.enchanting` with `provider = Some(npc)`. The
dialogue refusal check, `Currency`, the NPC `Purse` credit (`purse_delta`) and stolen-goods veto are the services
code, called, not copied. `ServiceRequest::Open` with a `Custom` shape returns `ServiceOpened { shape: Custom,
session: None }` and the host then emits `CraftOpened { session, kind }`; the dialogue screen stays beneath, as
for trade. Self-service kinds (alchemy via an apparatus item, self-enchant via a filled soul gem, recharge via a
gem and an item) open from `CraftOpener::Item`; the game crate maps "use item" to `CraftRequest::Open`.

### Commands and events (net rules 2, 5, 8)

Client to host (`CraftRequest`, in the player's command stream): `Open { kind, opener }`,
`Fill { session, slot, holder, stack }`, `Clear { session, slot }`, `Edit { session, edit }`,
`SetCount { session, count }`, `SetName { session, name }`, `Commit { session }`, `Cancel { session }`.
Host to clients (`CraftEvent`, each with tick, player, provider): `CraftOpened { session, kind, layout }`,
`CraftPreviewed { session, preview }` (to its player only; recomputed after every request), `CraftRefused
{ session, issue }`, `CraftDone { session, outcome, made: Vec<(RecordId, u32)> }`, `RecordCreated { id, kind }`
(broadcast to all players first, so every client can resolve the id), `SkillRaised`/`ItemsChanged` as today.

### Screens

`sf_rpg_ui::CraftView` over the inventory view's parts:

- **Slot strip** from the layout; A/click opens a filtered `InventoryView` picker (non-matching rows greyed with the
  reason); drag works as in the container screen. Tool slots auto-fill with the best owned tool and can be swapped.
- **Effect editor** (`EffectEditorView`): known-effect list (school filter, search), the draft, per-entry sliders (range,
  magnitude, duration, area), gamepad-first; ranges an effect forbids are hidden, each entry shows its cost.
- **Result bar**: preview cost, capacity bar, chance, fee, "?" for rows with `shown == false`, name field, count stepper,
  Commit, Cancel; refusals toast on the bar. NPC kinds add a provider header with both gold totals.

Screens send intents only; every number comes from `CraftPreview`. Themes style `sf-craft-*` parts.

## Co-op (net-ready rules)

1. **Fixed tick**: requests apply on the host tick; the roll seed includes it.
2. **Commands only**: screens send `CraftRequest`s and never touch inventories or the record db.
3. **State apart**: `CraftSession`, `EffectDraft`, `CraftPreview` are serde data; screens read `CraftEvent`s.
4. **Pure steps**: layout, accepts, preview, outcome, build are pure; rolls are inputs seeded from (player, session,
   attempt, tick, unit), splitmix64 as services SV1.
5. **One owner**: host-owned. NPC services are exclusive through the provider claim (rpg.md "Co-op"); self-service
   sessions are per player. Only the host calls `define_value`.
6. **No singleton player**: `CraftWorld` carries the `PlayerId`; previews use that sheet.
7. **Stable ids**: created records are `save:` `RecordId`s, deduplicated world-wide, so two players brewing the same
   potion get one record and two normal stacks. Records stay in the world overlay when a player leaves (Q3, Q5).
8. **Events are data**; `RecordCreated` precedes `ItemsChanged` for the new id.

## Saves

- `sf/records` (existing, owner `sf_save_bevy::sections::records`): created records. Unchanged format; kinds
  used: `potion`, `spell`, `enchantment`, `item` variants (weapon/armor/clothing/book derived records).
- `sf/crafting` (new, owner `sf_crafting`, version 1, optional): `Discovery { per player: effects/ingredients
  learned }` only when a game uses discover-by-use (default: empty and omitted). Sessions are never saved; saving
  closes any open session first (staged items never moved, nothing is lost).
- Item `charge` and `soul` ride in the item's saved `Attrs` (inventory section). Spells learned are the
  character's `SpellBook` (`sf.character` `effects`).
- Missing base record at load: the created entry is skipped and reported, kept in the overlay (save.md).

## Performance

A preview is arithmetic over at most 8 entries and 8 slots: well under 50 microseconds (budget **<= 0.2 ms host
per request**, measured in CR1's bench), re-run after every request, never per frame. `known_effects` for a book
of 300 spells is built once per open (<= 0.1 ms) and cached in the session. The dedup index is a `HashMap<[u8;
32], RecordId>` built at load (5,000 records: ~2 ms in the load job). Batch brewing of 99 units: 99 outcomes plus
at most 99 `CreatedDraft`s that dedup to one record each distinct result (<= 0.5 ms). UI: effect list is a
virtualized list; the sliders panel has at most 8 rows.

## Industry comparison

| Engine / game | Technique | Us |
|---|---|---|
| OpenMW / Morrowind ([alchemy.cpp](https://github.com/OpenMW/openmw/blob/ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f/apps/openmw/mwmechanics/alchemy.cpp), [enchanting.cpp](https://github.com/OpenMW/openmw/blob/ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f/apps/openmw/mwmechanics/enchanting.cpp)) | free combination with apparatus tiers; slider-built spells and enchantments; dynamic records deduplicated; skill use per success | **match**; adds exact preview, batches, refusal reasons |
| Oblivion | same model (altars, apparatus) | **differ** only in rules, a different `CraftRules` |
| Skyrim | any 2-3 ingredients, effects discovered by eating; enchanting learned by disenchanting; no spellmaking | **option**: `reveal`/`known_effects` abstract discovery; disenchant-to-learn is a later kind |
| Elden Ring / Valheim | recipe cookbooks, fixed output | **covered** by the Recipe sketch, not built |
| Modern ARPG benches | rerolls/upgrades of an item with preview odds | out of scope; a `Target` slot plus `SetStackAttr` fits |

Gaps (industry-audit.md): disenchanting, recipe kind, world stations.

## Tests

Bevy-free, exact assertions, synthetic records (`sf_crafting` tests):

- **Layout and fill**: `unique_item` rejects a second ingredient of one record; `accepts` false keeps the slot
  empty; a stack staged in two slots counts once against its stack size; `clear` frees it.
- **Preview totality**: every combination of missing slots yields issues, never a panic, never a refusal.
- **Commit order**: each refusal in order (`NothingStaged`, blocking issue, `StockChanged` after a hidden
  `RemoveItem`, veto with confiscation, `CantAfford`), no state change on any of them.
- **Roll**: same (player, session, attempt, tick, i) gives the same roll; a second attempt differs; batch
  `count = 3` consumes inputs for failed units too.
- **Settlement**: exact `RpgCommand` list for a success, a failure and a mixed batch; `purse_delta`; `Pay` first.
- **Created records**: dedup (two identical results share an id; different name or magnitude does not); a
  *content* record with identical stats is never reused; `Derived` applies ops to a copy and leaves the base
  untouched; missing base on load skips and reports; the index rebuilt from a loaded overlay equals the live one.
- **Charge/soul attrs**: `SetStackAttr` writes `charge`, clearing re-merges with a full stack.
- **Saves**: `sf/records` round trip of crafting kinds; shape golden unchanged; `sf/crafting` serde round trip.
- **Co-op** (headless, two `PlayerId`s): both brew the same potion, one record, two stacks; the second player's
  NPC-service `Open` on a claimed provider fails `Busy`; previews differ with the crafters' stats.
- **View-models** (`sf_rpg_ui`): picker filter by `accepts`, slider clamps from `EditorLimits`, `shown=false`
  rows render "?".

## Build plan (one deliverable per slice)

| Slice | Deliverable |
|---|---|
| **CR1** | `sf_crafting` core: kinds, layout, `CraftSession`, `preview`/`commit`, `CraftRules` trait, `StandardCraftRules`, settlement; tests above except created records; bench |
| **CR2** | Created records: `CreatedStore` trait, fingerprint index, `Derived` drafts, live registration hook, `sf/crafting` section; tests; Bevy glue in `sf_save_bevy` |
| **CR3** | `sf_rpg_ui`: `CraftView`, `EffectEditorView`, picker filter; dialogue `Custom` offers wired |
| **CR4** | `sf_ui_screens`: craft screens, host wiring of `CraftRequest`/`CraftEvent`; visual check entry (gamepad + mouse) |
| CR5+ | adapter slices (crafting-adapter.md "Build plan") |

CR1 and CR2 do not need the services screens; CR3/CR4 reuse the SV2/SV3 parts once SV3 is tested in game.

## Open questions (owner)

1. **Ingredient discovery**: Morrowind skill-gated reveal (port, default; stateless) or Skyrim-style
   discover-by-eating/combining (a saved discovery set, `sf/crafting`)? Default: skill-gated reveal in the
   classic column, modern column shows all effects of an ingredient the player has ever used or bought (a
   discovery set), so nothing is a hidden guess.
2. **Failure feel**: Morrowind rolls a failure per unit and consumes ingredients silently-ish. Default: keep the
   roll (classic) but modern preview shows the exact odds and a failed batch reports "made N of M".
3. **Created-record scope**: world-shared overlay (default; stable ids, dedup global) or per-character records?
4. **Batch crafting** (brew or enchant ammo in bulk): on by default (OpenMW has it for potions and ammo).
5. **Cleanup of unused created records**: never (default, v1) or a compaction at save when a record is referenced
   by no inventory, spell book or placement?
6. **Disenchanting / Skyrim-style learning** and a **recipe** kind: wanted for a later phase, or never?
