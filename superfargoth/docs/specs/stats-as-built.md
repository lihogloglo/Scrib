# Character stats: as built (`stats.md` companion)

As-built detail per slice, so `stats.md` stays the design. Read the spec first.

## S0 ids (2026-11-05)

`PlayerKey` and `ActorKey` moved from `sf_rpg/src/ids.rs` to `sf_ids` (unchanged shape and serde names), re-exported by
`sf_rpg` (`pub use sf_ids::{ActorKey, MAX_PLAYERS, PlayerId, PlayerKey}`). Save goldens unchanged (`sf_save_bevy` tests
green, no re-bless).

## S1 core (2026-11-05): `superfargoth/crates/sf_stats`

Bevy-free (hakari `final-excludes`); deps `sf_ids`, `sf_records` (`RecordId` for groups), serde, smallvec, thiserror.
Modules: `table` (`StatKey`, `StatIdx`, `StatKind`, `Rebase`, `StatDef`, `Role`, `StatTable`), `sheet` (`Sheet`,
`SheetRead`, modifiers, `Membership`, `Settle`), `ops` (`StatOp`, `apply`, `tick`, `StatEvent`, `Cause`, `StatOwner`,
`set_current`), `rules` (`Inputs`, the three traits, `Rules`, `LevelOffer`, `Refused`, `IncreaseSource`), `save`
(`SheetSave`, `LoadReport`), `test_model` (`TestModel`: linear rules by index, public so the Bevy shell and adapters can
smoke-test). 23 tests + 1 measurement.

Decisions taken while building (the spec text is the design; these are the details it left open):

- **Research check** (2026-11-05): the canonical technique holds. Creation keeps base + permanent + temporary + damage
  buckets per actor value; GAS aggregates source-tracked modifiers per attribute and recomputes on removal (Add, then
  Multiply summed as `1 + Σ(m)`, then Divide, Override). Ours is the same aggregator without Divide/Override. No spec
  change.
- **Dirty is per sheet, not per stat.** `DerivedModel` declares no dependencies, so any change recomputes values, then
  pool maxima (`pool_base`), then derived stats in table order. A per-stat bitset would not let pools or derived stats
  skip. Inputs that changed (seed excluded) also recompute.
- **Pool max** = `max(0, (eff_base + temp) × (1 + Σmul))`: pools take `Mul` modifiers like values (no content needs it
  today; one formula). A `Mul` in the Permanent layer acts on the whole value like a Temporary one.
- **Rebase** runs when a recompute moves a max: `KeepRatio` (a 0 old max leaves 0), `KeepCurrent` (lowered to the new
  max only when the max falls below it), `KeepDeficit`. `PoolModMax` moves base, current and the cached max together,
  so no rebase follows it. `PoolSetCurrent` sets exactly (no clamping).
- **Building mode + `Settle`** carry the load order: `Sheet::new` and `Sheet::from_save` start building (maxima move
  without rebase); sources re-add modifiers; `settle(.., Settle::Loaded)` recomputes and keeps saved currents (a
  fortified current over the unfortified max survives), `Settle::NewCharacter` fills pools and clears
  `changed_since_init`.
- **Damage cap** (fixed in S2 against OpenMW `ca5e2576` `spelleffects.cpp` `damageAttribute`/`damageSkill`:
  `magnitude = min(getModified(), magnitude); damage(magnitude)`; `AttributeValue::damage` itself never caps):
  `cap_at_modified` caps the *amount* at the value left before `Mul` (`eff_base - damage + temp`, ≥ 0). Damage already
  past it (an uncapped drain) is kept; S1 lowered it to the threshold, which let a later partial Restore show value
  OpenMW doesn't. Test `capped_damage_keeps_damage_already_past_the_value`.
- **Pool currents**: `set_current` = OpenMW `DynamicStat::setCurrent` (`stat.cpp` at `ca5e2576`), checked branch by
  branch in S2: identical, no change (`getModified` = max(0, base + modifier) = our pool max). `PoolAdjust` =
  `set_current(current + delta, ..)` against the max after earlier ops of the same call.
- **Derived refresh** (S2): derived stats are also refreshed after pool currents move without a recompute (end of
  `apply`, after regen in `tick`, after `settle` fills pools): a derived stat may read a current (Morrowind's fatigue
  term); S1 left it stale until the next recompute. Measurement after: 0.043 ms per tick (was 0.040).
- **Rejected ops**: an op on the wrong stat kind, an unknown index, a refused level-up or model recursion deeper than 4
  changes nothing; `apply` returns the count. A skill raise the model refuses (a cap) is not rejected: no event.
- **Groups**: `Membership.rank` is `Option<i32>`; reputation and the expelled flag exist without membership (scripts
  change faction reputation of non-members). `SetRank` joins; `Join` on a member is a no-op; `RankChanged` carries
  `Option` ranks.
- **Events**: every variant carries `cause` (net rule 8; `ApplyCtx` has no separate tick, it's `cause.tick`).
  `Changed` covers value modified, pool current and counters, one per stat per call, after the inline events
  (`SkillUsed`, `SkillRaised`, `RankChanged`, `Expelled`); then `Depleted`, `LevelChanged` (the `Level` role counter
  changed), `LevelReady` (ready false → true across the call).
- **Regen** ops are `PoolAdjust` / `PoolSetCurrent` only (others ignored), so a clean sheet's tick observes its pools
  only. `tick` returns at once when the sheet is clean, its inputs are unchanged and every pool is full.
- **Definitions**: `StatDef.group` and `label` are plain strings (no tag or loc-key type exists yet); `StatKey` folds
  ASCII case and serializes as a string; `StatTable::new` checks duplicates and role kinds (health/mana/stamina pools,
  level/bounty counters); `reindex()` after deserializing a table.
- **`Inputs`**: `seed`, `is_player`, `in_combat`, `combatant`, `carried`, `extra: SmallVec<[f32; 4]>` (content's own
  numbers by convention).
- **Saves**: `SheetSave { values, pools, counters: Vec<{key, ..}>, groups }`, shape golden
  `sf_stats/tests/shapes/sf_stats.SheetSave.v1.json` (bless with `SF_SAVE_BLESS=1`); `from_save` reports `dropped`
  (unknown key or another kind now) and `missing` (in the table, not in the save: the adapter's init fills them).
  Wiring into `sf.character` v3 and the overlay blob is S8.
- **Measurement** (release, owner laptop, other builds idle-ish): 500 sheets × 77 stats, 10 % applying one op per tick,
  all 500 regenerating: **0.040 ms per tick** (budget 0.05 ms). Snapshots are inline `SmallVec`s (no heap per step).

## Tests (`sf_stats`, moved from the spec)

Logic and data, exact: layers (the modified formula; Restore stops at 0 damage; capped damage; Mul sums); sources (add
then remove = byte-identical sheet; unknown remove = no-op; two sources on one stat remove independently); pools
(`PoolAdjust` on OpenMW's 8 `setCurrent` flag cases; `KeepRatio`; `Depleted` once per crossing); seams through
`TestModel` (`UseSkill` emits `SkillUsed`, then `SkillRaised` at the threshold; a wrong pick count refuses `LevelUp`);
groups (join, rank, expel, clear, reputation); determinism (one op stream twice = identical postcard bytes; `SheetSave`
shape golden); measurement (`#[ignore = "measurement: ..."]`): 500 sheets, 10 % dirty, tick cost. 24 tests after S2.

## S2 Morrowind table + init (2026-11-05): `bevyrrowind/crates/mw_stats`

Bevy-free (hakari `final-excludes`); deps `sf_stats`, `sf_records`, `tes3`, smallvec, thiserror. Ported from OpenMW
`ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f` (`THIRD_PARTY_NOTICES.md` "OpenMW (character stats)"). Modules: `table`
(`MwTable { table, idx: MwIdx, skills: [SkillInfo; 27] }`, `defs()`, `roles()`), `gmst` (`MwGmst`: 23 named GMSTs +
`iLevelUp01-10Mult`, `vanilla()`, `from_db` → (snapshot, missing names), `entries()`), `init` (`npc_ops`,
`creature_ops`, `chargen_ops` + `Chargen`, `new_sheet`, `ops_for_missing`, the pure autocalc functions), `derived`
(`MwDerived` + pure formulas). Details the adapter spec left open:

- **Table** (88 stats, index order): 8 attributes, 27 skills (`OpenMW` RefId keys, labels = their GMST name ids such as
  `sSkillLongblade`), magnitude `fortifymaximummagicka`, pools `health` (stored, `KeepCurrent`, depleted below 1),
  `magicka`/`fatigue` (derived, `KeepRatio`, depleted below 0), counters `level`, `reputation` 0-255, `bounty`,
  `level.progress`, `progress.<skill>`, `levelup.attr.<attr>`, `levelup.spec.<spec>`, `werewolf.kills`, internal
  `magicka.offset`, `fatigue.offset`, `actor.creature` (0/1), derived `capacity`, `encumbrance`, `fatigue.term`.
  `MwTable::from_db` requires all 27 SKIL records (governing attribute and specialization in range).
- **Record magicka and fatigue as offsets.** OpenMW keeps a non-autocalc NPC's or creature's stored magicka and fatigue
  until an attribute changes, then snaps them to the formula (`creaturestats.cpp` `setAttribute`, `recalculateMagicka`).
  Here `MwDerived::pool_base` = formula + `<pool>.offset`, the offset set at init to record - formula(record attributes):
  the record value at spawn, then the max follows attribute changes by their delta instead of snapping. Real install:
  140 of 1102 stored NPCs and 385/394 of 396 creatures have a non-zero offset (magicka/fatigue).
- **Creature capacity** is Str x 5 (`creature.cpp`), NPCs Str x `fEncumbranceStrMult`: the `actor.creature` counter tells
  `MwDerived`. Encumbrance = `Inputs::carried` (Feather/Burden, werewolf 0: effect runtime).
- **Autocalc**: the flag or a 12-byte NPDT (no stored stats) both autocalc (OpenMW would read zeros for the latter and
  spawn the NPC dead). Arithmetic in `f32` in OpenMW's order, `round_ties_even` for `round_ieee_754`, skills iterate in
  ESM index order (OpenMW iterates its skill store; sums of 0.2/0.5/1.0 can differ in the last bit only).
- **Reputation**: a faction member's is `iAutoRepFacMod` x (rank + 1) + `iAutoRepLevMod` x (level - 1) for every NPC
  (`ensureCustomData`, autocalc or not), else NPDT; membership = `SetRank { faction, NPDT rank }`.
- **Chargen** = `buildPlayer`: the `player` record's level, reputation and NPDT values; race (if chosen) replaces
  attributes (by sex) and sets skills to 5 + bonus; class (if chosen) adds +10 favoured, +10 minor, +25 major, +5
  specialization (no 100 cap, as OpenMW); health floor((Str + End) / 2) from base attributes; offsets 0.
- **Not in init**: spells, abilities, powers (race, birthsign, NPC lists, autocalc spells): the effect runtime adds their
  modifiers as sources (the chargen test adds Fortify Maximum Magicka 15 / 5 by hand: 125 / 75). Base disposition and AI
  settings are not stats.
- **`MwDerived`** in S2 covers `pool_base` (magicka, fatigue) and `derived`; `regen`/`rest` push nothing until S3.
- **GMSTs**: every `vanilla()` value equals the installed Morrowind + Tribunal + Bloodmoon value (0 missing);
  `iLevelUpMajorMult`, `iLevelUpMinorMult`, the three `*MultAttribute`, `iLevelupSpecialization` are all 1,
  `iAutoRepFacMod` 2, `iAutoRepLevMod` 0.
- **Tests**: 14 exact (autocalc level-10 warrior attributes, health, skills incl. half-to-even ties 82.5 → 82 and
  63.5 → 64; level 0; missing race/class refused; stored NPC + rank + reputation; record pools follow attributes by
  delta with ratio kept and the fatigue term; creature; chargen Altmer-/Breton-like 125/75; dev start; formulas;
  12-byte NPDT; `ops_for_missing`). Real install (`#[ignore = "real install"]`, `cargo test -p mw_stats --release --test
  real_install -- --ignored --nocapture`, 0.6 s): 3041 NPCs (1939 autocalc, levels 0-99, 0 init errors, 25 values over
  100, 84 placed with 0 health), 396 creatures (82 values over 100, 21 at 0 health), chargen 10 races x 21 classes x 2
  sexes; all finite, values in 0..=255, pools full; prints the GMST snapshot and SKIL table.

## S3 Morrowind models (2026-11-05): `bevyrrowind/crates/mw_stats`

New modules `progression` (`OpenMwSkillUse`, `OpenMwLeveling`, `openmw_rules`, `skill_progress_requirement`,
`levelup_attribute_multiplier`) and `faction` (`FactionRanks`, `RankRequirement`); `MwDerived` gained `regen` and `rest`
(+ pure `fatigue_regen_per_second`, `sleep_restoration_per_hour`, `normalized_encumbrance`, `rest_fatigue_per_hour`).
Ported from OpenMW `ca5e2576` (NOTICES "OpenMW (character stats)"; files in each module's docs). No new dependency.

- **Class on the sheet** (coordinator question 2): neither offered option fits the trait signatures. `Inputs` reaches
  `use_skill` but not `raise`, `ready`, `offer` or `level_up`; no model method gets the owner, so a model-owned table
  keyed by owner can't be read. The class is character data, so it lives on the sheet: 27 hidden counters
  `class.<skill>` (misc 0, minor 1, major 2) and `class.specialization` (specialization + 1, 0 = none), set by
  `class_ops` from `npc_ops` (autocalc, and stored NPCs whose class exists), `chargen_ops` (when a class is chosen).
  Saved and replicated with the sheet, deterministic, no framework change; an old save missing them gets them from
  `ops_for_missing`. Table: 88 → 116 stats. Nothing class-named in `superfargoth/`.
- **Regen** (`calculateRestoration`): fatigue only, `dt` x (`fFatigueReturnBase` + `fFatigueReturnMult` x End modified)
  while current < max; nothing for a `Combatant` (`Inputs::combatant`) or the dead (health < 1, `isDead`).
- **Rest** (`restoreDynamicStats`): one call for `hours` (OpenMW loops 1 h at a time; every term is constant over the
  rest, so the result is equal); sleep adds 0.1 x End to health and `fRestMagicMult` x Int to magicka per hour; rest and
  wait add 3600 x (base + mult x (1 - min(1, carried / capacity))) x `fEndFatigueMult` x End fatigue per hour while below
  the max (`getNormalizedEncumbrance`: 0 carried = 0, 0 capacity = 1 + 1e-6). Applies to combatants too (a time skip).
  Waiting for the effect runtime: Stunted Magicka, Feather/Burden, werewolf (encumbrance 0, no skill gain).
- **Skill use** (`skillUsed` + `skillUsedHandler`): gain = SKIL use value x scale (use kinds 0..=3, else gain 0 and no
  ops); refused (gain 0) at base 100 gaining or 0 losing; progress in `f64` stored `f32` (Lua doubles writing a C++
  float); `RaiseSkill { Usage }` at progress ≥ 1. `SkillUsed` reports 0 for a refused use.
- **Raise** (`skillLevelUpHandler`): base ± 1 by `ModBase`; tallies (level progress, governing attribute, specialization)
  skipped when their GMST is 0 (misc level progress); usage resets progress, training/book keep it. `Penalty` = OpenMW's
  jail source (-1, no tallies; Security and Sneak +1 with tallies, as the Lua does). `Script`/`Debug` = training.
- **Level-up**: ready at `level.progress` ≥ `iLevelUpTotal`; offer = attributes with base < 100, picks min(3, that),
  gain min(mult, 100 - base); refusals `NotReady`, `PickCount`, `BadPick` (duplicate, not pickable). Ops: `SetBase`
  min(100, base + mult), progress - total (≥ 0), non-zero attribute tallies cleared (**specialization tallies stay**:
  `levelUp` clears only `mSkillIncreases`), `PoolModMax` health + End (after the picks) x `fLevelUpHealthEndMult`
  (+ `PoolSetCurrent` 1 if that leaves it below 1), level + 1. Missing `iLevelUpNNMult` GMSTs take vanilla values
  (`MwGmst::from_db`), where OpenMW uses 1.
- **Faction ranks** (content, rule-set independent): `FactionRanks::of(&Faction)`; `has_skills_for_rank` (sorted faction
  skill bases truncated to int: best ≥ primary, next two ≥ favoured; fewer skills check fewer, none passes; rank > 9
  false), `meets_skill_requirements` (+ both favoured attributes' base; an unset attribute slot requires nothing),
  `meets_reputation` (membership reputation, 0 without), `rank_requirement` (the dialogue function: +1 skills, +2
  reputation for the next rank; 0 at rank ≥ 9; non-member checks rank 0).
- **Tests** (13 new, 27 in `mw_stats`): class counters; requirements (major/minor/misc x specialization); 19 uses of
  Long Blade (major, combat, base 30, requirement 18.6) to raise, with level progress, Strength and combat tallies;
  refusal at 100; raise sources (training keeps progress, book, misc, jail); multipliers 0..=10 and out of range;
  level-up after 10 major raises (Str 5 / End 2 / Agi 3 / Luck 0 tallies → +3/+2/+2/+1, picks Str End Luck, health +
  0.1 x 52, refusals); cap 100 with one pick; fatigue regen 3.5/s at End 50 (combatant and dead skipped); rest formulas
  (18144 / 18000 per hour); sleep vs wait restoration; determinism (an op stream twice: equal sheets and events, no
  clock or RNG); faction rank edge cases. Real install: every chargen class sets 5 majors + 5 minors.
