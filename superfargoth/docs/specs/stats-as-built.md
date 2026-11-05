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
- **Damage cap**: `cap_at_modified` stops damage at `eff_base + temp` (before `Mul`, ≥ 0), so the modified value
  bottoms at 0; a damage already above the threshold is lowered to it (OpenMW `AttributeValue::damage` as recalled:
  S2/S3 verify against the OpenMW source with a commit).
- **Pool currents**: `set_current` follows OpenMW's `DynamicStat::setCurrent` flags (8 branches,
  tested; written from the documented behaviour, S3 verifies against the OpenMW source). `PoolAdjust` =
  `set_current(current + delta, ..)` against the max after earlier ops of the same call.
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
