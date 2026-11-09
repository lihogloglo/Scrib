# SV2 views (sf_rpg_ui trade + service list view-models)

Status: DONE 2026-11-08, written without compiling: UNTESTED (owner test pass 2026-11-10).

## Files touched
- NEW superfargoth/crates/sf_rpg_ui/src/trade.rs (TradeView, TradePane, TradeRow, RowBlock, BarLine, DealBar, DealProblem, CountDialog, CountTarget, TradeFocus, Held, DropTarget, TradeToast, TradeNav)
- NEW superfargoth/crates/sf_rpg_ui/src/service_list.rs (ServiceListView, ServiceRow [ViewModel "service.entry"], RowReason, ListToast, ListNav, ListOutcome, ConfirmRule, confirm_lasting, SERVICE_ROW_FIELDS)
- NEW superfargoth/crates/sf_rpg_ui/src/tests_trade.rs (14 tests), tests_service_list.rs (7 tests)
- EDIT superfargoth/crates/sf_rpg_ui/src/lib.rs (doc paragraph, `mod service_list; mod trade;`, 2 cfg(test) mods, 2 pub use blocks at the end)
- EDIT superfargoth/crates/sf_rpg_ui/Cargo.toml (+ sf_ui_inventory.workspace = true; description). Internal workspace crate; sf_rpg_ui and sf_ui_inventory are both in hakari final-excludes, so no hakari regen expected (run the hakari check anyway).
- EDIT superfargoth/docs/specs/services.md (status line 3, "As built (SV1, SV2)"), 25.6 KB
- EDIT superfargoth/docs/specs/archive/services-history.md (+ "As built (SV2)")
- Not touched: dialogue.rs, tests_dialogue.rs, sf_rpg, sf_ui_inventory.

## Owner test commands (from repo root, via bash bevyrrowind/tools/cargo_turn.sh)
- cargo test -p sf_rpg_ui
- cargo clippy -p sf_rpg_ui --all-targets -- -D warnings
- cargo fmt -p sf_rpg_ui (lines > 100 cols were written; no rustfmt.toml found, so default width applies)

## Unverified guesses / risk spots
- Borrowck: TradeView::shown (closure over `&mut InventoryView` reborrowed immutably), deal() (closure-in-closure `bar_line`), nav() count-dialog block (restructured to avoid a live &mut).
- clippy::match_same_arms on ServiceRow::new (TooExpensive arm vs guarded Available arm with same body); if it fires, merge as `EntryState::TooExpensive => ..., EntryState::Available if ...`.
- `Option::is_none_or` (Rust 1.82+; toolchain 1.98.1, MSRV 1.90 OK).
- Test assumption: InventoryView with no sort keeps inventory order (view.rs apply_sort comment says so), Loadout::add of distinct items gives distinct stacks in insertion order.
- Depends on SV1 (uncompiled) types: sf_rpg::service::{ServiceRequest, ServiceEventKind, TradeLine, TradeRefusal::closes, MerchantMoney, TradeConfig, Currency, DealVeto, ServiceEntry, EntryState, EntryDetail, EntryRefusal, EntryId, LabelRef, ServiceKind, SessionId, Side}. If SV1 types change while fixing its build, these files follow.
- Name note: crate root already exports dialogue's `EntryId`; service EntryId is used by path only (no re-export) to avoid the clash.

## Design readings (also in archive/services-history.md "As built (SV2)")
- Price column = per-pane quote map in TradeView (not ItemRow::price); unpriced stacks greyed and never staged -> SV3 host must re-send pane_prices after TradeDone / new stacks.
- Currency hidden by filtering InventoryView::visible(); follow-up: native hide filter in sf_ui_inventory.
- One stock source per pane (multi-source stock_sources open for SV3/adapter).
- After TradeDone: same SessionId continues empty; view does Pool -= balance for display.
- ConfirmRule fn pointer; stock rule confirms travel rows only.

## For the orchestrator
- CHANGELOG: `2026-11-08 services SV2: sf_rpg_ui TradeView (two InventoryView panes, price column, deal bar, haggle stepper, count dialog, drag) and ServiceListView (disabled rows with reasons, travel confirmation, BuyEntry lock), 21 tests; written without compiling, UNTESTED (owner test pass 2026-11-10).`
- ROADMAP: services SV2 line -> written/untested (owner test 2026-11-10).
- test-pass-2026-11-10.md: add `cargo test -p sf_rpg_ui` (21 new tests: tests_trade, tests_service_list) + clippy -p sf_rpg_ui.
- SYSTEMS.md services row: add sf_rpg_ui trade.rs / service_list.rs if the row lists files.
- Possible conflict: the dialogue-buttons agent may also want lib.rs re-exports; I appended my two `pub use` blocks at the end of lib.rs only.
