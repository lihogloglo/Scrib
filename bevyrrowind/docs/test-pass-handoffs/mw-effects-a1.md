# mw-effects-a1 (sub-agent, 2026-11-08)

Scope: effects adapter slice A1: new Bevy-free crate bevyrrowind/crates/mw_effects.
Status: DONE, written without compiling: UNTESTED (owner test pass 2026-11-10). Nothing committed.

## Files
- bevyrrowind/crates/mw_effects/Cargo.toml (new; modelled on mw_ui; deps mw_stats, sf_effects, sf_records,
  sf_stats, smallvec, tes3[esp]; dev mw_install)
- src/lib.rs, src/mgef.rs (143-row table generated from OpenMW loadmgef.cpp), src/stats.rs, src/source.rs,
  src/tests.rs (14 unit tests), tests/real_install.rs (1 #[ignore] sweep)
- bevyrrowind/docs/specs/effects-adapter.md: status line + "As built (A1)" (17.4 KB)

## Owner test commands
1. `cargo hakari generate && cargo hakari manage-deps` (after adding the lines below)
2. `bash bevyrrowind/tools/cargo_turn.sh test -p mw_effects`
3. `bash bevyrrowind/tools/cargo_turn.sh test -p mw_effects --test real_install -- --ignored --nocapture`
4. `bash bevyrrowind/tools/cargo_turn.sh clippy -p mw_effects --all-targets -- -D warnings`; `cargo fmt -p mw_effects`

## Unverified guesses / risk spots
- `as` casts on tes3 fieldless enums (`EffectId as i32`, `EffectId2 as i16`, `SkillId2/AttributeId2 as i8`,
  `EffectSchool as usize`): believed lint-free (not pedantic), unverified.
- Hex literals grouped `0x1_1184`: assumed to satisfy `unreadable_literal` / `unusual_byte_groupings`.
- Tests' op expectations rely on E1 step behaviour read from step.rs (Modify start = AddModifier with
  class layer; Drain start = Damage cap false; Pool Hurt tick = PoolAdjust(-m*dt)); E2 may have changed apply.
- `from_records(std::iter::empty())` type inference; `(i32, &[&str])` tuple binding uses `&SKILLS[..]`.
- Magnitude stats are NOT in `MwTable` (mw_stats builds its own table; mw_effects -> mw_stats, so no cycle-free
  push). Glue must use `mw_effects::stat_defs()` or move the 22 defs into mw_stats::defs().
- Real install: asserts no bad MGEF index and no dropped effect; prints the rest. If Tribunal/Bloodmoon are not
  installed, indices 137-142 may lack MGEF records (printed, not asserted).

## For the orchestrator
- Root Cargo.toml `[workspace.dependencies]`: `mw_effects = { path = "bevyrrowind/crates/mw_effects" }`
- .config/hakari.toml final-excludes.workspace-members: add `"mw_effects",` (between mw_delirium.. / after "mw_combat"? keep alphabetical)
- CHANGELOG: `2026-11-08 effects adapter A1 (untested): new mw_effects crate: MGEF table as EffectDefs (OpenMW hardcoded flags, resist/weakness maps), 22 magnitude stats, A1 archetypes, SPEL/ALCH -> MwSource with calcSpellCost; 14 unit tests + real-install sweep.`
- THIRD_PARTY_NOTICES: `bevyrrowind/crates/mw_effects`: ported from OpenMW (GPL-3.0) commit ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f: components/esm3/loadmgef.cpp (HardcodedFlags, effect names, GMST ids, resistance/weakness maps), apps/openmw/mwmechanics/spellutil.cpp (calcEffectCost, getTotalCost, calcSpellCost), spellresistance.cpp (getEffectResistanceAttribute), mwworld/esmstore.cpp (effect-list validation).

**Update (magnitude-stats, 2026-11-08):** the 22 magnitude stats now live in `mw_stats::MwTable` (`MAGNITUDE_STATS`, indices 116..=137); `mw_effects::stat_defs`/`stat_table` were removed. See `magnitude-stats.md`.
