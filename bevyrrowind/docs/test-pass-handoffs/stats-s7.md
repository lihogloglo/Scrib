# stats-s7 + rule-hooks (2026-11-08) — handoff, written without compiling

Second slice, same session (docs only): `superfargoth/docs/specs/rule-hooks.md` (new), one-line links in
`docs/SYSTEMS.md` (new row after Scripting), `scripting.md` "Later:", `stats.md` rules-seam bullet, ROADMAP Lua `[p]`
line, CHANGELOG, DECISIONS. Not touched: nav.md, static-scene.md, precombine.md (another session, 18:32).

Scope: stats S7, MWScript stat/faction/crime functions in result scripts. Nothing committed.
Status: code + tests written on the laptop (no Rust toolchain use). First compile happens on the main workstation.

## Files
- New: `bevyrrowind/crates/mw_script/src/statfn.rs`, `bevyrrowind/crates/mw_rpg/src/script_stats.rs`,
  `bevyrrowind/crates/mw_rpg/src/tests_script_stats.rs`
- Changed: `mw_script` (interp.rs `Host::stat` + dispatch, lib.rs, functions.txt 31 rows -> live, functions.rs test,
  interp_tests.rs), `mw_rpg` (runner.rs `MwWorld::{sheets,actor,facts}` + `RunHost::stat`, stats.rs `MwSheets::rules`,
  result.rs `ResultEffect::Stats`, lib.rs, runner_tests.rs, tests/fargoth.rs), `mw_rpg_bevy/src/dialogue.rs`
  (View hooks, sheet rules, `stat_commands` + `send_stat_commands`), `sf_stats` (ops.rs `StatOp::Leave`, sheet.rs
  `last_inputs()`, tests.rs)
- Docs: stats.md status, stats-adapter.md status + "Consumers" as built, ROADMAP 2.7, CHANGELOG, DECISIONS.
- Not mine (were already modified/untracked): CHANGELOG spud line, hard-problems.md, docs/vision-*, research/*-2026-11-08.

## Verify (from the repo root)
```
bash bevyrrowind/tools/cargo_turn.sh test -p sf_stats -p mw_script -p mw_rpg
bash bevyrrowind/tools/cargo_turn.sh clippy -p sf_stats -p mw_script -p mw_rpg -p mw_rpg_bevy --all-targets -- -D warnings
bash bevyrrowind/tools/cargo_turn.sh test --workspace
```
Most likely compile snags (unverified API guesses): `Commands::write_message` in `send_stat_commands` (pattern from
mw_delirium_bevy), elided lifetimes on `MwWorld::facts`/`sheets` defaults, clippy pedantic casts in `script_stats::step`.

## In game (owner)
Console with an NPC selected or none: `player->ModStrength 10`, `PCRaiseRank "Fighters Guild"`, `ModPCCrimeLevel 50`,
then check the stats/HUD and that a save/load keeps them. Log line on a bad target: `result scripts: stats of ... have no sheet owner`.
