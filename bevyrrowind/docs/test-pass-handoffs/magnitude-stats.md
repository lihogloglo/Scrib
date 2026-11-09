# magnitude-stats (sub-agent, 2026-11-08): DONE, handoff

Scope: move the 22 A1 magnitude stats (elemental shields, Weakness*, Resist*) from `mw_effects` into the real
`MwTable` (`mw_stats::defs()`), appended at the end. Owned: `bevyrrowind/crates/mw_stats/src/table.rs`, mw_stats
tests, `bevyrrowind/crates/mw_effects/`. Read-only: `mw_stats/src/gmst.rs` (untouched).

Status: written 2026-11-08 without compiling: UNTESTED (owner test pass 2026-11-10).

## Files
- `bevyrrowind/crates/mw_stats/src/table.rs`: new `pub const MAGNITUDE_STATS: [(&str, &str); 22]` (key, label GMST,
  MGEF index order 4-6, 28-36, 90-99); `defs()` appends them after `fatigue.term` (Value min 0, no cap, group
  `magnitude`, not public); `MwIdx::magnitudes: [StatIdx; 22]` filled in `with_skills`. Capacity hint 128 -> 140.
- `bevyrrowind/crates/mw_stats/src/lib.rs`: re-exports `MAGNITUDE_STATS`.
- `bevyrrowind/crates/mw_stats/src/tests.rs`: table length +22; new `magnitude_stats_are_appended_after_the_older_indices`
  (pins every `MwIdx` field's old position 0..=115, magnitudes at 116..=137, group/label/kind, case-insensitive find).
- `bevyrrowind/crates/mw_effects/src/stats.rs`: now only `MAGNITUDE_STAT_EFFECTS`; `magnitude_stat_defs`, `stat_defs`,
  `stat_table` deleted (MwTable has them). `lib.rs` re-export and crate doc updated.
- `bevyrrowind/crates/mw_effects/src/tests.rs`: test-local `stat_table()` = `MwTable::with_skills(..).table`; new
  `magnitude_stats_match_the_mgef_table` (each MGEF index's `magnitude_stat_key` / `effect_label` equals
  `MAGNITUDE_STATS[n]`); `every_a1_stat_is_in_the_table` drops the old append-length checks.
- Docs: `bevyrrowind/docs/specs/effects-adapter.md` As built (A1) magnitude bullet; `stats-adapter.md` stat-table row.

## Save compatibility
`SheetSave` stores values/pools/counters by `StatKey` (`superfargoth/crates/sf_stats/src/save.rs`), not index. An
older save loads with the 22 keys in `LoadReport::missing`; `mw_rpg_bevy::stats` runs `ops_for_missing` (init has no
op for them) so they stay at `Sheet::new`'s 0 = correct base. No `STATS_BLOB_VERSION` bump needed. Per-kind slots
(values) also only grow at the end.

## Unverified guesses
- `MAGNITUDE_STATS.map(|(key, _)| at(key))`: `at` is a Copy closure (captures `&table`), as `ATTRIBUTES.map(at)` already relies on.
- Test code: `crate::mgef::magnitude_stat_key` (pub(crate), private module) reachable from the child `tests` module.
- No external caller of the deleted `mw_effects::{stat_table, stat_defs, magnitude_stat_defs}` (grepped repo: none).

## Owner test commands
- `bash bevyrrowind/tools/cargo_turn.sh test -p mw_stats -p mw_effects`
- `bash bevyrrowind/tools/cargo_turn.sh clippy -p mw_stats -p mw_effects --all-targets -- -D warnings`
- Wider (table length changed): `cargo_turn.sh test -p mw_rpg -p mw_rpg_bevy -p mw_saves_bevy`

## Risk spots
- Any test elsewhere pinning the table length (116) or a save-blob size would now fail (grep found only mw_stats's own).
- Others' glue (mw_rpg_bevy, A2) that appended `mw_effects::stat_defs()` would now get duplicate-key errors: none found.

## For the orchestrator
- CHANGELOG: `mw_stats`: the 22 A1 magnitude stats (elemental shields, Weakness*, Resist*) now in `MwTable`
  (`MAGNITUDE_STATS`, `MwIdx::magnitudes`, appended last, saves unaffected); `mw_effects` drops its own copies (untested).
