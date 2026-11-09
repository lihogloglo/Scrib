# barter-a1 (services adapter A1, 2026-11-08)

Scope: `mw_rpg::services` Bevy-free formulas (BarterTerms, barter_offer, six rules) + tests; GMSTs in
`mw_stats/src/gmst.rs`; re-exports in `mw_rpg/src/lib.rs`; "As built (A1)" in services-adapter.md.
Not touching: trade sessions, Bevy, mw_rpg_bevy, shared docs.

## Handoff (status: code complete, NOT compiled; UNTESTED until owner pass 2026-11-10)

Files:
- NEW bevyrrowind/crates/mw_rpg/src/services.rs (PartyTerms, BarterTerms, barter_offer, effective_value, 6 rules
  barter_price/barter_haggle/training_price/travel_price/spell_price/repair_price with In/Out structs + In::new(..,&MwGmst),
  FollowerPricing, TrainingRefusal, RULE_* ids)
- NEW bevyrrowind/crates/mw_rpg/src/tests_services.rs (25 tests; expected values replayed in f32 with node Math.fround,
  script: scratchpad barter/sim.js)
- EDIT bevyrrowind/crates/mw_rpg/src/lib.rs: `mod services;`, `pub use services::{..}`, `#[cfg(test)] mod tests_services;`
- EDIT bevyrrowind/crates/mw_stats/src/gmst.rs: +12 GMSTs (bargain_offer_base/multi, disposition_mod,
  barter_success/fail_disposition, barter_gold_reset_delay, training_mod, travel_mult, travel_time_mult,
  mages_guild_travel, spell_value_mult, repair_mult); vanilla from opencs defaultgmsts.cpp @ca5e2576 (file is CRLF, kept)
- EDIT bevyrrowind/docs/specs/services-adapter.md: status line + "As built (A1)" (25.8 KB)

Owner test commands:
- cargo test -p mw_rpg tests_services
- cargo test -p mw_stats
- cargo test -p mw_stats --test real_install -- --ignored --nocapture   (prints the 12 new GMSTs vs vanilla())
- cargo clippy -p mw_rpg -p mw_stats --all-targets -- -D warnings
- cargo fmt --check -p mw_rpg -p mw_stats (lib.rs pub use wrapping, multi-line `new` signatures written by hand)

Unverified guesses / risk spots:
- clippy float_cmp on `i.travel_mult == 0.0` (believed exempt: comparison to zero constant).
- clippy many_single_char_names / similar_names allowed on barter_offer and barter_haggle (allow, not expect).
- fMagesGuildTravel / integer GMSTs read via `as i32` (OpenMW getInteger() on a float variant truncates: assumed).
- rustfmt may re-wrap the `pub use services::{..}` block or test lines (no rustfmt.toml found; width ~120 by habit).
- Deviation (documented): travel In takes positions, not distances; training skill/attr are f32; creature haggle delta 0.

## For the orchestrator
- CHANGELOG: "2026-11-08 services A1: `mw_rpg::services` Morrowind price formulas (barter offer, 75% cap, haggle,
  training, travel, spells, repair) as pure In->Out rules + 12 GMSTs in `MwGmst`; written without compiling, UNTESTED."
- ROADMAP: services adapter A1 -> written, untested (owner pass 2026-11-10).
- THIRD_PARTY_NOTICES (OpenMW port, GPL-3.0): "bevyrrowind/crates/mw_rpg/src/services.rs ports Morrowind barter and
  service price formulas from OpenMW (https://github.com/OpenMW/openmw, GPL-3.0-or-later) at commit
  ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f: apps/openmw/mwmechanics/mechanicsmanagerimp.cpp (getBarterOffer 563-590),
  apps/openmw/mwgui/tradewindow.cpp (38-114, 447-457, 608-645), trainingwindow.cpp (118-121, 172-191),
  travelwindow.cpp (50-83, 203-213), spellbuyingwindow.cpp (54-59), merchantrepair.cpp (64-83); GMST defaults from
  apps/opencs/model/world/defaultgmsts.cpp."
- test-pass-2026-11-10.md: the four cargo commands above.
