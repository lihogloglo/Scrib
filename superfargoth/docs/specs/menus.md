# Menus: front end, main menu, pause menu, options

Status: design (2026-11-11), owner answers at the end. Builds on [ui.md](ui.md) (screen stack, layers, CSS
themes, `ui.setting` records, `MainMenuScreen`, `PauseMenuScreen`, `OptionsScreen`, all built) and
[video.md](video.md) (menu and intro videos). Game side: `bevyrrowind/docs/specs/ui-adapter.md` "Menus and
front end". ROADMAP 2.2 (main menu over title art/video, MM-1 follow-up) and 2.2b (Options screen).
Research: [menus-settings-industry-2026-11-11](../research/menus-settings-industry-2026-11-11.md),
[video-playback-2026-11-11](../research/video-playback-2026-11-11.md),
[menus-video-art-2026-11-11](../../../bevyrrowind/docs/research/menus-video-art-2026-11-11.md).

<!-- agent-summary -->
> **Agent summary (2026-11-11).** Status: design, not built. Adds a top-level `AppPhase` state (Boot, Frontend,
> Loading, InGame) so the world exists only in game; a content-pack backdrop list (video, then image) behind the
> main menu; settings v2 (apply timing, scope, conditions, presets with derived "Custom", dynamic options); an
> options screen v2 on `bevy_ui_widgets` (slider drag, description pane, apply/revert, keep-or-revert countdown).
> Read next: "Design" for types, "Build plan" for slices.
<!-- /agent-summary -->

## Goal

- The main menu is a **front end**, not an overlay on the live world (owner 2026-10-30, MM-1 rejected): title
  art or a looping video behind it, chosen by the content pack and overridable by mods; the world is built only
  when the player starts, continues or loads.
- **One options screen** for main and pause menus, data-driven from setting records, that a PC player of 2026
  expects: graphics preset with derived "Custom", display settings with a keep-or-revert countdown, mouse, key
  and pad operation, descriptions, reset, restart badges.
- Moddable like the rest: mods add settings pages (MCM equivalent), backdrops, intro videos and styles through
  records, the VFS and CSS.

Non-goals here: character creation (not Phase 2), the mod manager screen (ui.md "Screens", later), benchmark
button and VRAM estimates (nice-to-have, later), subtitles (ui.md open question 5).

## Canonical technique

- **Front end as its own mode**: UE5 Lyra's frontend experience (a minimal front-end map plus UI layers; the
  gameplay world loads on New/Load), Bethesda and FromSoftware titles (title art or a light scene, world loads
  after). Flow: logo videos (skippable) → main menu (Continue = newest save, default focus) → loading screen →
  game; pause menu → "Main menu" tears the world down.
- **Settings as registered, typed objects**: Lyra's GameSettings plugin (`UGameSettingRegistry`,
  `UGameSettingValueDiscrete/Scalar`, edit conditions, dependencies, dirty/apply/revert) and OpenMW's Lua
  `I.Settings` pages/groups/renderers as the modding reference. Presets are data; "Custom" is derived.
- **Bevy idiom**: top-level `States` + `DespawnOnExit` state-scoped entities (Bevy 0.20, batched despawn), run
  conditions `in_state(..)` to gate world systems.

## Reuse (Bevy, then crates, then our own)

| Need | Choice | Why |
|---|---|---|
| Phases | Bevy `States`, `SubStates`, `ComputedStates`, `DespawnOnExit` | stock, exactly the job |
| Widgets | `bevy_ui_widgets` 0.20 `Slider`, `Checkbox`, `RadioGroup`, `TabList`/`Tab`, `TextInput`, `Dialog`, scrollbars (checked in the 0.20 source) | headless, styleable by our CSS; replaces our hand-made option rows |
| Focus, navigation | `bevy_input_focus` + `AutoDirectionalNavigation` | already used |
| Styling | `bevy_flair` 0.9 (`sf_ui_css`) | stack decision |
| Settings persistence | our `sf_mods` profile (`[settings.<owner>]`), plus our machine file `<user dir>/machine.toml` (S1c) | Bevy 0.20's `bevy-settings` (evaluated S1c): same TOML layout and per-user folder, but each group is a compile-time reflected `SettingsGroup` resource; our settings are records that mods add at load, so it doesn't fit a key → value store |
| Video | [video.md](video.md) | |
| Menu crates | none | haalka, cobweb_ui, alt-ui-navigation, bevy_pkv, bevy-settings (3rd party) are stale or replace bevy_ui (research §4) |

Ours (no crate does it on 0.20): the setting registry v2 (extends `sf_ui` records), presets, conditions, the
options screen layout, the backdrop resolver. BSN files have not shipped: screens stay Rust templates.

## Design

### 1. App phases

`AppPhase` (framework state, `sf_app_bevy`):

| Phase | What runs | Enters on |
|---|---|---|
| `Boot` | profile, mods and records load; machine settings applied before the first presented frame; intro videos | start |
| `Frontend` | main menu over the backdrop; options; save list (Load). **No world**: no streaming, player, sky, physics | Boot done, or "Main menu" from game |
| `Loading` | loading screen (splash + progress); the world builds at the start or save location | New game, Continue, Load |
| `InGame` | the game; pause menu and other screens over it | streaming reports the start area resident (`Ready`) |

- World entities and resources carry `DespawnOnExit(AppPhase::InGame)`-equivalent scoping (spawned in `Loading`,
  kept into `InGame`, so use a `ComputedStates` `WorldAlive` = Loading | InGame). The app gates world plugins'
  public system sets with `configure_sets(.., run_if(in_state(WorldAlive)))`, so no framework crate needs to
  know the phase; `Startup` world setup moves to `OnEnter(WorldAlive)`; the front end has its own UI camera.
- `AppPhase` lives in a new thin framework crate `sf_app_bevy` (`bevy_app`, `bevy_ecs`, `bevy_state` only).
- Pause stays as built (`ScreenStack::pauses_game` drives `Time<Virtual>`); no `Paused` sub-state needed.
- **Dev runs** (`view`, `scene`, `cargo dev`) go Boot → Loading → InGame directly (owner 2026-10-26: dev runs
  skip the menu). Only the release start shows the front end.
- **Load from the front end**: the save names its world location; `Loading` builds the world there, then the
  existing load-apply path runs at the fade cut (save.md). **Load in game** keeps today's path. **Main menu from
  the pause menu** asks (unsaved progress), leaves `InGame` (world despawned), enters `Frontend`.
- Teardown must leave nothing behind (a test: InGame → Frontend → InGame twice, entity and asset counts back to
  the first frontend's, no panics).

**As built (F1, 2026-11-11).** `sf_app_bevy`: `AppPhase`, `WorldAlive` (no same-state transitions, so Loading →
InGame doesn't re-run `OnEnter`), `AppPhasePlugin { after_boot }` (leaves Boot after `PostStartup`). Game wiring in
`mw_exterior_bevy/src/phase.rs`: gates `StreamingSystems`, `TerrainSystems`, `FarFieldSystems`, `SurfaceSystems`
and the viewer's own world systems on `WorldAlive`; `setup` (camera, player anchor, roots) and test levels run on
`OnEnter(WorldAlive)`, the placement spawners stay at `Startup`; front end UI camera on `OnEnter/OnExit(Frontend)`.
Loading → InGame: the start worldspace's directory in, then (release) a burst armed after it has ended. Interim
until F3: Continue/Load build the world at the new-game start, then load or open the save list in game. Plugin
`Startup` resources (sky entities, precipitation layers, HUD root) still spawn in Boot: F2's teardown must scope them.

**As built (F2-F3, 2026-11-11).** Teardown on `OnExit(WorldAlive)` (`mw_exterior_bevy/src/teardown.rs`): worldspace
roots despawned recursively, `Streaming`/registry/overlay/fade/start resources replaced. New game applies a
`NewGameBaseline` captured on the first Frontend (`sf_save_bevy/src/baseline.rs`; session reset to default, the
first save picks the world id). Pause menu "Main menu" asks Yes/No, then Frontend. Continue/Load from the front end:
`LoadingSave` holds streaming's `Drive` set, the `LoadRequest` goes out on entering Loading with a zero fade-out, the
character section's `StartTravel { reload }` moves the player at the cut, streaming starts there; a failed load returns
to Frontend. The save screen opens over the main menu (`DeferLoads` → `LoadChosen`).

### 2. Backdrop and title

- The content pack registers an ordered **backdrop candidate list** (adapter hook `MenuBackdropCandidates`):
  each entry is a VFS path and a kind (`Video` or `Image`). The resolver takes the first that exists and loads
  (a video whose decoder fails falls to the next). Mods win through the VFS (loose over archive) and may also
  prepend candidates in their manifest (`[ui] menu_backdrop = [...]`).
- Fit: `cover` (default), `contain` (letterbox, black), `stretch`; a setting (`sf.menu_backdrop_fit`).
- `MainMenuModel.background` takes the resolved `Handle<Image>` (a video's frame image is a normal `Image`, see
  video.md), `title_image` an optional title image from the same hook.
- Menu music: the pack names a cue (`MenuMusic`), played on the music bus in `Frontend`.

### 3. Intro videos and loading screen

- Intro list from the adapter (`IntroVideos`: ordered paths), played in `Boot`, each skippable (Confirm, Back,
  Escape, click); a missing file is skipped silently. Setting `sf.intro_videos`: `first_launch` (default, owner 2026-11-11), `always`, `off`.
- A "new game" video (adapter hook) plays in `Loading` for New game, skippable, before the world fades in.
- Loading screen (`LoadingScreen`, `Layer::Modal`): a splash image from the adapter's list (random pick), a
  progress bar from the streaming burst's resident/requested counts, an optional hint line (Fluent keys).

*As built (I1):* `sf_app_bevy::BootHolds` (named holds; none: Boot ends after `PostStartup` as before, else in the
`PostUpdate` the last is released). `sf.intro_videos` is a framework built-in (`sf_ui::IntroVideos`, page
`interface`). Playback is the game's `mw_exterior_bevy/src/intro.rs` (adapter list `IntroVideos`, letterboxed with
`sf_ui::contain_size`, own UI camera). Rule 0: Bevy has no movie player (`sf_video_bevy`); UE Startup Movies and
Bethesda play logos skippable, one by one. First-run flag: a marker file `<user dir>/intro_seen` written after the
first played sequence (UE/Unity keep such flags in the user config; a file of its own is not a fake setting).

### 4. Settings v2

`SettingDef` (`sf_ui/src/records.rs:224`) gains, all optional with today's behaviour as default:

| Field | Values | Default |
|---|---|---|
| `apply` | `Live` (consumer applies on change), `Confirm` (staged; Apply/Revert bar; with `keep_seconds` a keep-or-revert countdown), `Restart` (stored; badge "applies after restart") | `Live` |
| `scope` | `Profile` (`[settings.<owner>]` in the profile), `Machine` (graphics/display, `<user dir>/machine.toml`), `Save` (stored in the save; later) | `Profile` |
| `enabled_if`, `visible_if` | a small condition list, ANDed: `{ setting = "sf.upscaler", is = "dlss" }`, `{ trait = "dlss" }`, `{ phase = "frontend" }` | always |
| `disabled_reason` | Fluent key shown on a disabled row | - |
| `order`, `keywords` | int within group; search words (Fluent key) | load order; - |
| `reset` | included in "Reset page" | `true` (`false` for language, bindings) |
| `format` | value display: `percent`, `ms`, `x`, `raw` (+ `log` slider scale for floats) | `raw` |
| `options_from` (Choice) | id of a registered provider (`display.monitors`, `display.resolutions`, `ui.locales`) | static `options` |
| `kind = Action` | a button row that emits `OptionsEvent::Action(id)` (open a sub-screen, run calibration) | - |

- **Platform traits** (`SettingTraits` resource, filled by the app: `dlss`, `rt`, `multi_monitor`, `gamepad`)
  feed conditions; no code per setting.
- **Presets** (new record `ui.setting_preset`; not `ui.preset`, which is the layout preset): `id`, `driver`
  (the Choice setting showing the levels, e.g. `sf.graphics_preset`), `levels` (`low = { "sf.shadows" = "low",
  ... }`). Picking a level writes its values; the driver reads **Custom** whenever current values match no
  level (derived each change, never stored as truth). Mods add keys to a level in load order. *As built (S1b):*
  a mod ships its own `ui.setting_preset` with the same `driver` (sf_records patch keys can't contain the dots of
  setting keys); records sharing a driver merge into the first in load order, last writer of a key wins (as slot
  contributions). `derived_level` checks levels in level-id byte order, first match wins; "custom" is reserved.
- **Change contract**: the store emits `SettingChanged { key, old, new }` (a Bevy message in `sf_ui_bevy`);
  consumers are ordinary systems, one per setting family, registered by the owning crate. `Restart` settings
  are read once from the store at app build (before plugins are added), which is how the game's start-time
  switches become settings without making each one live on day one. *As built (S1c):* `SettingsStore` logs
  every effective write (set, reset, loads, presets) per key with its first old value; `sf_ui_bevy::emit_setting_changes`
  drains it in `PostUpdate` (changed-and-back reports nothing). `Machine` keys skip the profile; `load_machine_settings(world,
  <user dir>/machine.toml)` right after the store is inserted at app build, saved on the profile's rule; a corrupt file
  loads defaults and returns the error. `Save` keys stay in the profile until saves carry settings.
- **Dynamic providers** register `fn(&World) -> Vec<(value, label)>` by id; the options model re-queries on
  open and on `WindowCreated`/monitor change.
- Values are clamped and validated on load (range, option still offered, else default + a log line).

### 5. Options screen v2

`OptionsScreen` (`sf_ui_screens/src/options.rs`) keeps its model/event split; changes:

- Rows on `bevy_ui_widgets`: `Slider` (mouse drag and click, pad left/right; PM-1 follow-up), `Checkbox`,
  choices as a left/right stepper (pad) that also opens a dropdown on click (`menu`/`popover` widgets); pages
  on `TabList`. Bindings stay the keybinds screen, reached by a Controls page row.
- **Description pane** (right or bottom by preset): label, description, disabled reason, "applies after
  restart", `format`ted current/default value.
- **Footer action bar** (glyphs per device): Back, Reset page, Apply/Revert (only while `Confirm` settings are
  staged; leaving with staged changes asks).
- **Keep-or-revert dialog** for `keep_seconds` settings (display mode, resolution, monitor): "Keep these
  settings? Reverting in 15 s", focus on Revert, timeout reverts (Windows' and most engines' rule).
- **Pages** (framework order; packs and mods add pages): Gameplay, Display, Graphics, Audio, Controls,
  Interface (UI and text scale, theme, layout preset), Accessibility, then one page per mod with settings.
  Search box: later (keywords are in the data now). A content pack may supply a second **page layout** (which
  pages, their order, which settings sit on each) selected by its classic option; the settings and the screen
  are the same, only the grouping differs (Bevyrrowind: OpenMW's categories, ui-adapter.md).
- **Two entry points, one screen**: conditions (`phase`) hide or disable what doesn't apply (e.g. language only
  in the front end). In game the panel is translucent over the paused world, so `Live` graphics settings
  preview; in the front end it sits over the backdrop.

*As built (S2a, 2026-11-11):* rule 0: Bevy's `bevy_ui_widgets` 0.20 `Slider` (drag, `TrackClick::Snap`) and
`Checkbox` cover it, no crate; UE CommonUI / Unity UI Toolkit sliders also take drag + click + step + pad. A slider
row's `StatBar` is the `Slider` (thumb in a travel node inset by its width); `ValueChange` snaps to the step grid
(`OptionRow::snapped`), one `Set` per grid change; steps move `SliderValue`/thumb, flips `Checked`. Toggles have a
checkbox, no arrows; disabled rows carry `InteractionDisabled`. Parts: `slider`, `thumb`, `checkbox`; no theme CSS.

*As built (S2b1, 2026-11-11):* rule 0: Bevy has no detail pane (plain nodes; `PointerEnter`/`PointerLeave` for hover);
CommonUI and most PC ports (Nixxes, Cyberpunk 2.0) show a right-side pane on focus/hover. `sf-options-pane` (right;
bottom-by-preset later): label, description, reason (while disabled), restart badge, "Default: <value>"; empty lines hidden.
Hovered row, else focused (`OptionsScreen::described`); hover never moves focus and alone reaches a disabled row's reason.

### 6. Framework settings shipped (built-in, owner `sf`)

Display (`Machine`): window mode (windowed / borderless / exclusive; `Confirm`, 15 s), monitor
(`options_from`), resolution (`options_from`, windowed/exclusive), vsync, frame cap, UI scale. Audio: as built.
Interface: theme, layout preset, text scale, reduce motion, high contrast (from ui.md). The game adds the
Graphics page (adapter) because what a feature is and costs is game-specific.

### 7. Screen flow v3 (2026-11-23, owner PP-4: "the whole flow of windows in the menu is wrong")

Diagnosis `../research/menu-flow-diagnosis-2026-11-23.md` (path:line there). The causes are structural:

- **Lower screens are never covered.** The stack sets only z-order and the input context (`sf_ui_bevy/src/router.rs`);
  every sub-menu is pushed on the same Menu layer and stays drawn, and it is translucent (`theme.css` rgba 0.78), so
  pause shows through options. Their click handlers don't check that they are on top
  (`options.rs:1304`, `pause_menu.rs:497`, `main_menu.rs:779`), so they light up and click behind options.
- **Windows size to their content** and re-centre (`widgets.rs:237-245`): hovering a row changes the description
  pane's text, the window grows or shrinks, and the rows slide under the pointer; value changes and tab switches
  do the same. Hover CSS only changes colour.
- Keybinds replaces the game's `Node` and lands top-left (`keybinds.rs:215`); long pages clip (no scroll); four
  copies of close-and-despawn in the game; option rows ignore `enabled_if`/`visible_if`.

The standard (UE CommonUI activatable stacks, Lyra's settings screen, Xbox XAG 112/113 for focus) and our fix:

- **The framework owns the lifecycle.** A screen record gains `covers: Below | Nothing` (default `Below` on the
  Menu layer, `Nothing` for HUD, toasts, modal dialogs). A covered screen is hidden (`Visibility::Hidden`) and not
  pickable; only the top non-modal screen and the modals above it are drawn and take pointer input. Handlers act
  only for the stack's input target. Pop despawns through one framework system (`DespawnOnPop`), replacing the four
  game copies. Modal confirm and keep-or-revert use `bevy_ui_widgets` `ModalDialog` + barrier.
- **Fixed window frame.** Menu window size comes from the layout preset (CSS `width`/`height`, a share of the
  viewport), never from content; the panel keeps today's translucent look (once covered screens are hidden it no longer shows
  another window through it; opacity is a look call for the owner); the description pane and value column reserve their space; the row list
  scrolls (bevy_ui `Overflow::scroll_y`). Hover and focus never change size, padding or border width; focus has its
  own indicator, always visible (XAG 112).
- **Options is one panel with page tabs** (Lyra): Gameplay, Display, Graphics, Audio, Controls (key bindings are
  this page, not a separate window), Interface, Accessibility, mod pages; footer bar Back, Reset page,
  Apply/Revert. Pause and main menu open it; it covers them.
- **Rows read the record fields that exist**: `enabled_if`/`visible_if` with `disabled_reason`, `order`, `format`.
- Screens never replace a `Node` the game spawned; they style it.

Test rule (headless): after any push/pop sequence exactly one non-modal Menu screen is visible and pickable; a
window's layout rect is the same before and after hover, value change and tab switch.

*As built (2026-11-23, MF-1a..MF-6, owner check MF-1 confirmed):* `sf_ui` `Covers` (Menu `Below`, others `Nothing`),
`ScreenStack::is_covered/visible`; `sf_ui_bevy` PostUpdate chain `despawn_closed_roots` (`UiSeenOpen`, opt-out
`UiKeepWhenClosed`) -> `order_screen_roots` -> `hide_covered_roots` (`UiCovered`; Bevy picking already skips hidden nodes),
before bevy_ui layout; handlers gated by `sf_ui_screens::on_target`. Frame: `widgets::WindowFrame`/`framed_window`,
`OPTIONS_FRAME` 60% x 70% clamped 640x420..1280x900 (keybinds uses it too; game spawns bare roots, the screen owns the
`Node`), pane 280 px, value column 160 px, rows `Overflow::scroll_y` + `ScrollArea`. Options already had tabs (Q/E).
Footer [Key bindings][Reset page]..[Back], Reset asks with `widgets::Question`, `OptionsChange::Reset` -> store reset.
Rows honour `enabled_if`/`visible_if`/`order`/`format` (`OptionsModel::refresh`; phase and traits wait for
`set_condition_context`). Advanced page (`mw_exterior_bevy/src/advanced_page.rs`): rows generated from player-scope
switches at `cleanup`, choices incl. `custom` = preset decides; `SettingsStore::add_settings`. RPG screens settle on a
stack check (`end_closed_talk`). Open: MF-5, phase/traits source, `log_scale`, preset pick clearing Advanced picks, two
strings not localized, `mw_controls_bevy` unused `bevy_ui` dependency.

## Industry comparison

| Engine/game | Has | Us |
|---|---|---|
| Lyra GameSettings | registry, edit conditions, dependencies, apply/revert, scalability presets | same model as records (§4); dependencies are conditions plus preset derivation |
| OpenMW Lua settings | mod pages/groups, renderers per type | `ui.setting` pages + `kind`; custom renderer hook later (Lua parked) |
| Nixxes PC ports, Cyberpunk 2.0 | live preview over paused game, descriptions, restart flags | §5; preview images and VRAM bar later |
| Windows / most engines | 15 s keep-or-revert on display change | §5 |
| Elden Ring (counter-example) | thin options, no per-action hold/toggle | we have hold/toggle (keybinds screen) |

Gaps (to `industry-audit.md` when built): no settings search yet, no benchmark, no preview images, no
per-save scope until saves carry settings.

## Build plan

Each is one slice with its test; F and V lines can run in parallel (different crates).

- **F1** `AppPhase` + `WorldAlive` in the framework; streaming and world setup gated; release start enters
  `Frontend` with no world; New game goes Loading → InGame. Test: headless phase walk, no world entities in
  Frontend.
- **F2** "Main menu" from the pause menu: teardown to Frontend. Test: the twice-round-trip count test (§1).
- **F3** Continue / Load from the front end (save location → Loading → apply). Test: headless load from Frontend.
- **F4** Loading screen (splash, progress). Test: shows in Loading, gone in InGame.
- **B1** Backdrop resolver + image backdrop + title image + menu music hook. Test: candidate order resolution.
- **V1-V3** video.md build plan; **B2** video backdrop once V2 lands; **I1** intro videos in Boot.
- **S1** `SettingDef` v2 fields, presets + derived Custom, conditions, machine scope (Bevy-free `sf_ui`, exact
  tests). Includes the `bevy-settings` evaluation.
- **S2** Options screen v2 (widgets, description pane, footer, apply/revert, keep dialog, reset page).
- **S3** Display settings live (window mode, monitor, resolution, vsync, frame cap) with the countdown.
- **S4** Graphics page (game adapter): preset + per-feature settings, `Restart` first; each later conversion to
  `Live` is its own logged slice. *As built (tunables T5d, 2026-11-16):* the page is the preset drivers only (overall +
  8 `sg.*` groups, owner: groups only), all live (the tunables' presets, not `ui.setting_preset` values), `Machine`
  scope, "Applies after restart" on rows writing `Restart` tunables; choices step over `custom`. Per-feature rows: later.
- **S5** Interface and accessibility pages wired to `UiSettings`.

Screen flow v3 (§7; 2026-11-23, before S2b-S5 continue):

- **MF-1** (built) `covers` on screen records; covered screens hidden and not pickable; handlers gated on the input target;
  one framework despawn on pop (game copies removed). Test: push/pop sequences leave one visible non-modal screen.
- **MF-2** (built) fixed window frame from the layout preset, reserved pane and value column,
  scrolling row list. Test: window rect unchanged across hover, value change, tab switch (headless layout).
- **MF-3** (built: tabs existed, keybinds framed MF-3b, footer MF-3c) options as one tabbed panel (`TabList`), keybinds as the Controls page (no `Node` replacement), footer
  bar (Back, Reset page). Test: tab switch keeps focus and rect; Back from a page returns to the opener.
- **MF-4** (built) rows read `enabled_if`/`visible_if`/`disabled_reason`/`order`/`format`. Test: conditions, exact.
- **MF-5** Apply/Revert + keep-or-revert dialog (`ModalDialog`), then S3 display settings on it.
- **MF-6** (built) Graphics > Advanced: player-scope feature switches with cost class ([feature-switches.md](feature-switches.md);
  only if the owner wants per-feature rows for players).

## Owner answers (2026-11-11)

1. Video decoder: pure-Rust Bink first (video.md).
2. Intro logos: **first launch only** (`sf.intro_videos` = `first_launch` default, `always`, `off`), skippable.
3. Order: **front end first** (F1-F4, B1), then video (V1-V3, B2, I1), then options (S1-S5), then textured art.
