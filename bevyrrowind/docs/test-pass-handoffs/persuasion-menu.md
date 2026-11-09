# persuasion-menu (sub-agent, 2026-11-08)
Scope: the Persuasion action button + six-option submenu, host side (mw_rpg_bevy dialogue.rs + dialogue/persuasion.rs), routing of DialogueRequest::Action, tests, rpg-adapter.md As built.
Status: started 2026-11-08. Read handoffs dialogue-actions.md + disposition-wiring.md, sf_rpg_ui action API, OpenMW dialogue.cpp (PersuasionDialog::onOpen: bribe enabled when playerGold >= 10/100/1000; sPersuasion entry NPCs only, first before services) and openmw_persuasion_dialog.layout (sAdmire, sIntimidate, sTaunt, "sBribe 10 Gold", "sBribe 100 Gold", "sBribe 1000 Gold") at ca5e2576. Note: apps/openmw/mwgui/persuasiondialog.cpp 404s at that commit; the class lives in mwgui/dialogue.cpp.

## Handoff (status: code complete, written 2026-11-08 without compiling: UNTESTED (owner test pass 2026-11-10))
Files:
- bevyrrowind/crates/mw_rpg_bevy/src/dialogue/persuasion.rs: `PERSUASION_ACTION`, `PERSUASION_OPTIONS` (id, kind, GMST, English fallback), `persuasion_kind`, `gmst_text`; `Rpg::{persuasion_action, refresh_actions, route_action}`; module doc; 3 headless tests + menu/route block at the end of the install-gated `persuasion_through_the_host_matches_the_formula`.
- bevyrrowind/crates/mw_rpg_bevy/src/dialogue.rs: re-exports (line ~60), `StatTick` import; `Rpg::request` = `request_at(.., 0)`; new `Rpg::request_at(.., tick)` routes `DialogueRequest::Action` to `route_action`; `after()` calls `refresh_actions` before the view refresh; `host_dialogue` takes `Option<Res<StatTick>>` and calls `request_at`.
- bevyrrowind/docs/specs/rpg-adapter.md: "As built (persuasion in the host...)": dropped "No menu UI yet", two bullets.

Unverified guesses / risk spots:
- Fallback labels assume Morrowind.esm English values ("Admire", "Intimidate", "Taunt", "Bribe 10 Gold", "Persuasion"); the install test prints the real ones (`persuasion menu:` eprintln).
- Glob `use super::*` in tests picks up persuasion.rs's private imports (RecordDb, ActorKey, GameSetting, Attrs, SessionState...); I removed the tests' now-duplicate `sf_records::RecordDb` / `sf_rpg::ActorKey` imports.
- `let options = ...collect();` / `into_iter().collect()` rely on inference from `with_submenu` / `set_actions`.
- Headless db: `mw_records::register_kinds` + one `Npc` + one `GameSetting`; `Rpg::new`/`open` over a db with no DIAL assumed fine (`DialogueSession::open` gives NoResponse).
- In-game the buttons need sf_ui_screens to draw/send actions (other session: dialogue-buttons-screen.md).
- No gold readout (OpenMW's `sGold: N` label in the menu): the view-model has no slot for it.

Owner test commands:
    cargo test -p mw_rpg_bevy persuasion
    cargo clippy -p mw_rpg_bevy --all-targets -- -D warnings
In-game: talk to Fargoth (Seyda Neen) → Persuasion button → submenu: Admire/Intimidate/Taunt live, bribes greyed with no gold; pick Admire → an "Admire Success/Fail" line; disposition change visible in the log (`persuasion: Admire on ...` at debug) / filters; Shift+U-style gold (console `player->additem gold_001 100`) then reopen → Bribe 10/100 enabled.

For the orchestrator:
- CHANGELOG: "2026-11-08 Persuasion menu (host): NPC dialogue gets a Persuasion action (Admire/Intimidate/Taunt/Bribe 10/100/1000, GMST labels, bribes disabled when the player can't pay, refreshed every step) routed to `persuade_and_reply`; services actions a logged no-op (UNTESTED)."
