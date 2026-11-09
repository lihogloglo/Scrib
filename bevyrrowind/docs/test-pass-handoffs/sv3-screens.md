# SV3a screens (sf_ui_screens TradeScreen + ServiceListScreen)

Status: DONE 2026-11-08, written without compiling: UNTESTED (owner test pass 2026-11-10). Host wiring (SV3b, game crate) not in this slice.

## Files touched
- NEW superfargoth/crates/sf_ui_screens/src/trade.rs: TradeScreen, TradeParts, TradePaneRoot, TradeBarRoot, TradeRowPart, TradeLinePart, TradeButton; shared ServiceScreenEvent (Message) / ServiceIntent {Request(ServiceRequest), Close}; ServiceLabels resource (fixed strings + LabelRef key table).
- NEW superfargoth/crates/sf_ui_screens/src/service_list.rs: ServiceListScreen (pub view, pub progress), ServiceListParts, ServicePart.
- NEW superfargoth/crates/sf_ui_screens/src/tests_services.rs: 4 headless tests (own copy of the Ui harness; tests.rs untouched).
- EDIT sf_ui_screens/src/lib.rs: module docs, `pub mod service_list; pub mod trade;`, `#[cfg(test)] mod tests_services;`, pub use blocks, init ServiceLabels, add_message ServiceScreenEvent, 9 observers, spawn/input/sync systems (sync chain now 15 systems).
- EDIT sf_ui_screens/Cargo.toml: description only (no new deps; bevy_color, bevy_picking already there).
- EDIT superfargoth/docs/specs/services.md: status line 3 + As built heading/pointer (25.76 KB, +~120 B).
- EDIT superfargoth/docs/specs/archive/services-history.md: + "As built (SV3a)".
- Not touched: dialogue.rs, tests.rs, demo.rs, sf_rpg_ui, sf_rpg.

## Owner test commands (repo root, via bash bevyrrowind/tools/cargo_turn.sh)
- cargo test -p sf_ui_screens --features demo tests_services
- cargo test -p sf_ui_screens
- cargo clippy -p sf_ui_screens --all-features --all-targets -- -D warnings
- cargo fmt -p sf_ui_screens (some lines may exceed rustfmt's layout)

## Unverified guesses / risk spots
- Depends on SV1/SV2 code that was never compiled (TradeView, ServiceListView API names as read from sf_rpg_ui today).
- Borrowck in sync_trade (`view = &mut screen.view` alongside `screen.title` / `screen.stat_labels`), in on_trade_drag_start (`grab_line` called in a match guard), in sync_service_list (`view` alongside `screen.progress` / `screen.answer`).
- Deref coercion of `&Res<ScreenLabels>` / `&Res<ServiceLabels>` to `&T` in fn args (same as container.rs).
- PointerDragDrop bubbling: the drop observer runs per ancestor; first hit with a target takes `held` (same scheme as container.rs).
- `Result<&T,_>::copied()` on Query::get; `Option::is_none_or`.
- Spec deviation: panes are keyed lists with a price column, not GridPane icon grids (GridPane shows currency and full counts, has no price column). Reason column shows "not traded"/"on the bar" instead of a tooltip reason. Logged in archive As built (SV3a).
- Tests open under demo registry ids "container"/"inventory" (no "trade"/"service" id in demo.rs, which this slice may not edit). Tests assume DemoContainer has >= 2 stacks, one with count > 1 (bread 5) and that VirtualList::move_focus moves with row_extent 1 / viewport 10.
- Pad map choice: Y (Context2) = Accept, LS = ResetOffer, RS = Bar (Start stays the menu key; TradeNav docs say "Start: accept").
- Clicking a row right after a drag could also fire Activate (same as container).

## For the orchestrator
- CHANGELOG: `2026-11-08 services SV3a: sf_ui_screens TradeScreen (provider/player list panes with price column, deal bar, haggle stepper, count dialog, drag) and ServiceListScreen (disabled rows with reasons, travel confirmation, progress bar), ServiceScreenEvent intents, 4 headless tests; written without compiling, UNTESTED (owner test pass 2026-11-10).`
- test-pass-2026-11-10.md: add `cargo test -p sf_ui_screens` (tests_services, 4 new) + clippy -p sf_ui_screens.
- ROADMAP: services SV3 -> SV3a written/untested; SV3b host wiring (game crate: route ServiceScreenEvent -> ServiceRequest commands, ServiceEvent -> view.apply_event, re-send prices, register "trade"/"service" screen ids) open.
- Visual-check entry for bevyrrowind/docs/visual-checks/player-ui.md (needs SV3b host wiring first):
  ```
  ### PUI-SV3 Trade and service list screens (awaiting owner)
  Talk to a merchant, Barter; then a trainer, Training. Gamepad and mouse.
  - Trade: stock left, your items right, deal bar between; prices on every row; untraded rows grey ("not traded"); no gold rows.
  - Pad: d-pad up/down rows, sideways to/from the bar; LB/RB panes; A stages whole, X asks how many, Y accepts, B backs out then cancels.
  - Bar: lines per side, totals, both purses, balance; haggle stepper left/right (LB/RB 10%), Reset; refusals show as a line on the bar.
  - Mouse: hover focuses, click stages, drag a row onto the bar/other pane, drag a line back; ghost follows, nothing sticks.
  - Service list: title, gold, rows with detail + price; too expensive rows grey with reason; travel asks first; B closes back to dialogue.
  - Steam Deck 1280x800: nothing clipped.
  ```
  Index line: `- PUI-SV3 trade + service list screens (gamepad + mouse): awaiting owner (player-ui.md)`
