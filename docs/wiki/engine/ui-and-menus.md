# UI and menus

## What it does
Game UI is data-driven: screens, slots, themes and settings are mod records, styled by CSS, input-routed through a
screen stack. The framework ships stock screens (dialogue, journal, inventory, trade, options, pause, main menu);
the game supplies models, a Morrowind theme and the front-end flow. A Quake-style dev console exists beside it.

## How it works
- **Core (Bevy-free)**: `superfargoth/crates/sf_ui/src/lib.rs:1` : record kinds `ui.screen/slot/column/theme/preset/
  setting` (`records`), `UiRegistry` (resolved once per mod set, conflicts reported), `ScreenStack` (layers, Common UI
  model; `input_target` `stack.rs:148`, `pauses_game` `:157`), `ViewModel`/`ListModel`, settings store, keymap, l10n.
- **Bevy glue**: `SfUiPlugin` (`sf_ui_bevy/src/lib.rs:204`): resources `UiRegistryRes`, `ScreenStackRes`,
  `UiSettingsStore`; input router (`router.rs`) gives one `bevy_enhanced_input` context per screen and activates only
  the stack target's, above gameplay priority; named UI actions and rebinding (`actions.rs`, `keybinds.rs`); toasts
  (`notify.rs`); profile sync (`profile.rs`).
- **CSS**: `sf_ui_css` is the only crate linking `bevy_flair`. `UiCssSourcePlugin` (`plugin.rs:36`, before
  `AssetPlugin`) and `UiCssPlugin`; `compose.rs:80` builds one root sheet per (theme, preset, mod set): framework,
  preset, theme, one layer per mod, player sheet. Screen roots get `preset-*`, `theme-*`, `input=kbm|gamepad` classes.
- **Stock screens**: `SfScreensPlugin` (`sf_ui_screens/src/lib.rs:127`): widgets, keyed row reconciliation, dialogue,
  journal, inventory, container, trade, service list, keybinds, save/load, pause, main menu, options, HUD. A screen is
  a `UiScreenRoot` entity plus its component, pushed on the stack; intents leave as events (`PauseMenuEvent`, ...).
- **Morrowind theme**: `mw_boot_bevy/src/ui_theme.rs:121` `add` selects `theme-morrowind`; sheet =
  `ui/morrowind/theme.css` plus tokens read from `Morrowind.ini` (`:88`, `:106`).
- **Phases**: Boot (intro videos, `mw_exterior_bevy/src/intro.rs`, first launch only, marker `intro_seen`), Frontend
  (main menu, no world), Loading, Playing (`phase.rs:67`). Release start (no subcommand) goes Frontend; `view`/`scene`
  skip to Loading.
- **Main menu**: `main_menu.rs`: Continue (newest save), New game, Load, Options, Quit; art buttons when VFS has
  `menu_*.dds`; backdrop = first of video/texture candidates via `sf_ui::resolve_backdrop`, looped `VideoPlayer`.
- **Pause menu**: `pause_menu.rs`: Esc opens (HUD screen is the input target of play); Resume, Save, Load, Options,
  Main menu, Quit; `pause = "WhileOpen"` pauses `Time<Virtual>`. Options from `ui.setting` records, stored in
  `profiles/<name>.toml`, written on menu exit.
- **Console**: `sf_console` (Bevy-free: commands, typed variables with `SetBy` priority, completion) and
  `sf_console_bevy` (`ConsolePlugin` `lib.rs:34`, `ConsoleSubmit`, `ConsoleVarChanged`). Game front end:
  `mw_devtools_bevy/src/console.rs`: backquote opens it in `view`/`scene` only; `render.*` variables, `on/off/flip`
  switches, any other line runs as MWScript.
- **Book text and messages**: `mw_ui` (Bevy-free): BOOK markup to `RichDoc`, MessageBox formatting (ported from OpenMW).

## Bevy features used
`bevy_ui`, `bevy_enhanced_input` (contexts), `bevy_flair` (CSS), `bevy_text`, `DirectionalNavigationPlugin`
(gamepad focus, `sf_ui_bevy/src/lib.rs:211`), `Time<Virtual>` pause.

## Switches and counters
Console variables (`help`, `vars`); settings `sf.intro_videos` (`first_launch`/`always`/`off`); `--off` parts do not
touch UI. `ui_frames` (`mw_boot_bevy/src/ui_frames.rs`) holds the Morrowind frame art.

## Known limits
No Credits screen. Options changes apply only where a consumer reads the store (audio page does); the rest are
stored. No console in screenshot or benchmark runs. Visual state of menus is owner-judged (`visual-checks`, MM-1).

## Specs
`superfargoth/docs/specs/ui.md`, `menus.md`, `console.md`, `journal-ui.md`, `ui-text.md`;
`bevyrrowind/docs/specs/ui-adapter.md`.
