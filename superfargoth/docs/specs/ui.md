# Game UI

Status: design complete (2026-10-11), owner decisions below; framework (U1-U2), inventory and keybinds logic
(U5, U6) and the stock screens built 2026-10-18 to 10-25: "As built" at the end, which wins where it differs.
Crates: `sf_ui` (Bevy-free: records, registry, view-models, list models, keymap, Fluent), `sf_rpg_ui`
(Bevy-free RPG screen view-models: dialogue, journal), `sf_ui_inventory` (Bevy-free inventory logic),
`sf_ui_bevy` (runtime, layers, input router), `sf_ui_css` (bevy_flair integration and stylesheet composer,
isolated on purpose), `sf_ui_screens` (widget kit and stock screens); not built: `sf_ui_lua` (Luau bindings,
part of `sf_script`'s API surface), `sf_devui` (dev tools only).
History (research R1-R5, as-built logs U1a-U2, U5, U6, dialogue view-model detail, industry table, closed
questions): [archive/ui-history.md](archive/ui-history.md).

<!-- agent-summary -->
> **Agent summary (2026-10-26).** Status: framework (U1-U2), inventory and keybinds logic (U5, U6) and stock screens built; pause menu and options screens wired in the game 2026-10-26; ROADMAP 2.2 open (8-piece frames, inventory icons, theme hot reload). Designed, not built: `sf_ui_lua`, `sf_devui`.
> Crates / key types: `sf_ui`, `sf_rpg_ui`, `sf_ui_inventory`, `sf_ui_bevy`, `sf_ui_css` (bevy_flair), `sf_ui_screens` (dialogue, journal, inventory, keybinds, HUD, `SaveLoadScreen`, `PauseMenuScreen`, `OptionsScreen`); `UiScreenRoot`, `UiSettings`, `InputMode`.
> Decisions: two themes and two layout presets over one view-model ("Owner decisions"); modder styling is CSS via bevy_flair; Bevy first, Feathers dev-only ("Stack decision").
> Read next: "As built: current state" (wins on conflicts); "Stack decision" for crate choices; "Interfaces" for types.
<!-- /agent-summary -->

## Goal

A game UI that is good on day one and stays good under mods. Concretely:

- **Moddable like the rest of the framework.** Stock screens are built with the same public
  API mods use. Mods add screens, replace screens, inject widgets into named slots, restyle
  with CSS in load order, and declare settings that the framework renders. Nothing privileged.
- **Two looks, two layouts, one data layer.** Themes and layout presets are data over shared
  view-models, switchable live (owner decisions 1-2).
- **Gamepad-complete everywhere**, with no virtual cursor, and **mouse-and-keyboard-dense on
  PC** (sortable, searchable lists), with instant input-device switching.
- **Scalable, localizable, accessible**: crisp text at any scale, 200 % text without loss,
  Fluent strings, bidi, AccessKit, high-contrast and reduce-motion options.
- **Cheap**: virtualized lists, fine-grained updates, per-mod UI time in `sf_diag`.

Non-goals: dev tools (separate `sf_devui`, egui); a general HTML engine; game-specific screens
or art (content packs supply them, see `bevyrrowind/docs/specs/ui-adapter.md` for the first).

## Owner decisions (2026-10-11)

1. **Look: ship both themes from day one.** A game-flavoured theme supplied by the content
   pack (parchment, bronze frames, a period display face tuned for modern readability) and a
   modern minimal theme. A theme is data, swappable live.
2. **Layout: both presets** over the same view-models. *PC-dense*: lists with columns,
   optional draggable classic windows, mouse first. *Controller-first*: full-screen tabbed
   screens. Both are gamepad-complete.
3. **Modder styling is CSS, via `bevy_flair`**, layered by mod load order (R2b). This replaces
   the draft's own `ui.style` records.
4. **Timing: spec now, build at M5** (Lua), on `.bsn` scene files once Bevy 0.21 ships them.
   No UI code before M5.

## Canonical technique

Retained ECS UI (bevy_ui) organized like **Unreal CommonUI** (dev.epicgames.com, "Overview of
advanced multiplatform user interfaces with Common UI" and "CommonUI input technical guide"):
layers (HUD, menu stack, modal, notification, tooltip); **activatable widgets** in stacks, where
input routes only to the top-most active widget and falls back to the next layer when it
closes; per-screen input configs; action bars with platform glyphs; style data assets.
Data flows **MVVM** (XAML/NoesisGUI bindings; UE5's UMG Viewmodel plugin): game systems write
view-models, screens bind to them, screens never query gameplay state. Styling is **CSS with
cascade layers** (W3C CSS Cascade 5 `@layer`), which is exactly "later mods override earlier
ones without replacing them". Behavior comes from headless widgets (bevy_ui_widgets,
controlled pattern), focus from bevy_input_focus.


## Research findings

Full findings (2026-10-11) in [archive/ui-history.md](archive/ui-history.md#research-findings): R1 Bevy
0.20 native, R2 crates, R2b bevy_flair, R3 toolkits outside Bevy, R4 what players and modders hated and
loved, R5 controller-first and readability lessons. The design below keeps what they taught.

### R2b. bevy_flair: licence, limits, fallback

Binds the mod contract ([full text](archive/ui-history.md#r2b-bevy_flair-licence-limits-fallback)).
- **Licence**: `MIT OR Apache-2.0`; its Servo dependencies (`cssparser`, `selectors`, ...) are MPL-2.0,
  allowed unmodified (owner, 2026-10-11), contained in `sf_ui_css`.
- **Limits**: one stylesheet per entity, no global sheets (we compose one per layer root); `!important`
  ignored; no `var()` fallback; no `%` + `px` `calc()` until Bevy's `Val::Calc` (#25653); no custom media
  features, so presets and input mode are root classes/attributes.
- **Fallback**: the mod contract is *the CSS subset and property names documented here*, not flair's API;
  flair types never leave `sf_ui_css`. Lagging flair → a `[patch]` fork; abandoned → `sf_ui_css` on
  `cssparser` + `selectors` (or `lightningcss`), mod files untouched.
- **Buttons and states** (uipolish 2026-10-25): every `bevy_ui_widgets::Button` gets the class `sf-button`
  and picking's `Hovered` (required component, `sf_ui_css`), so themes style `:hover`, `:active` (the widget's
  `Pressed`) and `:focus-visible` (`InputFocusVisible`: on for pad and arrow-key navigation, off once the mouse
  is used, `sf_ui_bevy` device/nav). The code focus ring (`ScreenColors::focus_ring`) follows the same rule;
  a theme that shows focus itself sets it to `None`.

## Stack decision

1. **Bevy first**: bevy_ui, bevy_ui_widgets, bevy_input_focus + `AutoDirectionalNavigation`,
   bevy_picking (span-level text picking), bevy_text (parley), bevy_a11y, BSN (`.bsn` assets
   when 0.21 ships).
2. **Feathers: dev tools only** (its docs say so).
3. **Crates**: `bevy_flair` for styling (decision 3; MPL caveat, R2b), inside `sf_ui_css`.
   `fluent-bundle` for strings. `bevy_enhanced_input` 0.27 for actions (chosen 2026-10-18,
   Open questions 4). Evaluate `bevy_ui_anchor` in the slice.
   `bevy_egui` + inspector for dev UI only.
4. **Own** (no crate does these on 0.20): screen registry and layers, input router, extension
   slots, view-models and list models, virtualized list, glyph prompts, stylesheet composer,
   settings registry, Luau bindings. bevy_hui/haalka/lunex overlap BSN or replace bevy_ui.

## Data model and moddability

All UI content comes through `sf_vfs` / `sf_records` / `sf_mods` (everything is a mod).

- **Theme** record `ui.theme`: id, display name (Fluent key), stylesheet entry
  (`ui/themes/<id>/theme.css`), font family lists with fallbacks, icon set, sound set,
  supported presets. Tokens are **CSS custom properties** on `:root` (`--color-surface`,
  `--font-body`, `--space-2`, `--radius`, `--sound-confirm`), so a mod changes a token with
  one CSS rule. Accessibility token sets (high contrast, colorblind) are theme sub-sheets.
- **Preset** record `ui.preset`: `pc_dense` and `controller` built in; root class
  (`.preset-pc` / `.preset-controller`), stylesheet entry, density defaults, whether classic
  windows are allowed. Screens may carry a per-preset layout; otherwise one layout restyled.
- **Stylesheets** (CSS, decision 3): a mod lists its sheets in its manifest
  (`[ui] styles = ["ui/<mod>/style.css"]`). The composer (`sf_ui_css`) serves one generated
  root sheet per (theme, preset, mod set) through an asset source, e.g.:

  ```css
  @layer sf, preset, theme, mods, user;
  @import "ui/sf/base.css" layer(sf);
  @import "ui/presets/pc_dense.css" layer(preset);
  @import "ui/themes/classic/theme.css" layer(theme);
  @import "ui/better_inventory/style.css" layer(mods.better_inventory);  /* load order */
  @import "ui/hud_tweaks/style.css" layer(mods.hud_tweaks);
  @import "user/ui.css" layer(user);   /* the player's own overrides, last */
  ```

  Later layers win, as load order does. Every import is layered, so no mod escapes into the
  unlayered (strongest) cascade. A mod may still replace a whole file by VFS override; the
  mod manager shows both kinds. Flair's per-rule errors are reported with the owning mod
  (layer name → mod id) in the mod manager and `sf_diag`.
- **Screen** records `ui.screen`: id, layer, input context, pause policy, supported presets,
  window capability (PC classic windows), implementation (`.bsn` asset, registered Rust
  template, or Lua entry), view-model kinds it binds. Mods **add or replace** screens.
- **Extension slots**: named anchors in stock screens (`hud.top_right`,
  `inventory.columns`, `item_tooltip.after_stats`, `dialogue.topic_actions`); contributions
  ordered by load order, then priority. `#Name` ids are unique per scene scope (fixes MWSE).
- **Columns** records `ui.column`: list column definitions (header key, width, sort kind,
  value source: a view-model field or a Lua function), so SkyUI-style columns are data.
- **Layout files**: `.bsn` through sf_vfs once 0.21 lands, loaded with an allowlist of
  UI-safe component types; a mod's layout patch stacks on the base scene with BSN field
  patching.
- **Settings registry** (MCM equivalent): settings as records (key, type, default, range,
  Fluent label, page/group, which mod), rendered by the framework; values saved per profile.
- **View-models**: typed data that game systems (Rust or Lua) write and screens read
  (Interfaces). List models have stable keys, so sorting/filtering never rebuilds rows.

## Interfaces

Sketch for the M5 slice, updated to the code where "As built" says so; other names are provisional.

```rust
// ---- sf_ui (Bevy-free) ----
pub enum Layer { Hud, Menu, Modal, Notification, Tooltip }      // fixed order, bottom to top
pub struct ScreenDef {
    pub id: ScreenId, pub layer: Layer, pub input: InputContextId,
    pub pause: PausePolicy, pub presets: PresetSet, pub windowed: bool,
    pub implementation: ScreenImpl,        // Bsn(VfsPath) | Template(TemplateId) | Lua { mod_id, entry }
}
pub struct SlotContribution { pub slot: SlotId, pub mod_id: ModId, pub priority: i32, pub content: ScreenImpl }

/// Built once per mod set from records, in load order; replacements resolved; conflicts reported.
pub struct UiRegistry { /* screens, slots, columns, settings, themes, presets */ }
impl UiRegistry {
    pub fn build(records: &RecordDb) -> (Self, Vec<UiConflict>);   // record sources are mods in load order
    pub fn screen(&self, id: &ScreenId) -> Option<&ScreenDef>;
    pub fn slot(&self, id: &SlotId) -> &[SlotContribution];        // already ordered
}

/// MVVM; not `Reflect`, so `sf_ui` stays Bevy-free. Lists are keyed.
pub trait ViewModel: Send + Sync + 'static {
    const KIND: &'static str; fn fields() -> &'static [&'static str]; fn field(&self, name: &str) -> Option<UiValue>;
}
pub struct ListModel<K, T> { /* keyed rows, version, sort + filter descriptors */ }

// ---- sf_ui_bevy ----
#[derive(Resource)] pub struct UiSettings {                  // live; change = restyle, no respawn
    pub theme: ThemeId, pub preset: PresetId,
    pub text_scale: f32, pub hud_scale: f32, pub menu_scale: f32,
    pub high_contrast: bool, pub reduce_motion: bool, pub classic_windows: bool,
}
#[derive(Resource)] pub struct InputMode(pub Device);       // Kbm | Gamepad; root attribute `input`
pub struct UiCommands<'w, 's> { /* push/pop/replace screen on its layer, open window, focus */ }
// Bindings inside templates: `Bind::text("item.name")`, `Bind::list("inventory.items", row_template)`.
// Router: the top active screen of the highest input-taking layer gets actions (CommonUI rule).
// Systems run in `UiSystems::{ApplyScriptCommands, Bind, Layout}`, after camera/transform
// propagation and before bevy_ui layout, so script-driven and world-anchored UI is same-frame.

// ---- sf_ui_css ----
// Composer asset source + FlairPlugin; root sheet handle on each layer root's `Styled`.
// Root classes: preset-*, theme-*, a11y flags; attribute `input=kbm|gamepad`.
```

Luau surface (`@sf/ui`; placeholder until `sf_script`'s API rules apply, `api_version`-gated; Lua is
parked): slot contributions and columns (`ui.slot(..):add`), elements from scenes with fine-grained setters
on their handles (no tree re-diff), virtualized lists, settings pages, screen replacement, UI events and
synchronous text measuring. Sketch: [archive](archive/ui-history.md#luau-surface-sketch-from-interfaces).

Handles are generational and become invalid when their element dies. A mod mutates only
elements it created and slots it fills; UI code can't trigger gameplay actions except through
the curated API (WoW's lesson). Per-mod UI time goes to `sf_diag`.

## Input

Named UI actions (navigate, confirm, back, tab prev/next, context 1-4, open menu, open map)
with KBM and gamepad defaults; **all rebindable, including menu open** (Elden Ring lesson).
No virtual cursor: drag-and-drop has a focus equivalent (pick up, place), text entry has an
on-screen keyboard and default names, held directions repeat. The last-used device switches
glyphs and focus visuals within the frame; it never swaps layout (BG3 lesson). Classic windows
on gamepad: shoulder buttons cycle open windows, the d-pad moves inside one.

**Keybinds screen (U6).** One keymap holds the UI actions and whatever a game registers
(gameplay context), each with slots per device (keyboard/mouse, gamepad), defaults and the
scopes where it is live. The screen lists rows per context and device, captures an input for a
slot, flags conflicts per scope (with explicit exemptions such as Esc for back and menu open),
asks swap / unbind / keep both, resets an action, a context or everything (per device or all),
and saves only what differs from the defaults to the profile. Confirm, back, menu open and the
d-pad/arrows are rebindable but never left unbound on a device (no lock-out). Glyphs follow the
last-used pad's family (Xbox, PlayStation, Nintendo). Model: UE5 `EnhancedInputUserSettings`
(player-mappable keys in slots, `ResetMappingToDefault`/`ResetKeyProfileToDefault`, a failure
reason on `MapPlayerKey`) and Unity Input System binding overrides (defaults stay, only
overrides are saved; the rebinding sample offers swap on duplicates).

## Presets, theming, scaling, localization, accessibility

- **PC-dense**: lists with columns, sort, search, category tabs, compare; hover tooltips;
  optional classic windows (several open at once, draggable and resizable, geometry saved per
  screen). **Controller-first**: full-screen screens with tabs, larger rows, action bar,
  focus-driven tooltips. Default preset from the device at first launch; switchable live.
- Layout in `Rem`/`Em`; root size = reference (1080p) × UI scale × text scale; separate HUD,
  menu and text scales. Defaults meet XAG 101 (18 px PC, 26 px controller preset); every screen
  lays out at 1280×720 with 200 % text (a test). Icons mipmapped.
- Fluent strings; parley bidi + `complex_script_segmentation`; RTL mirroring later.
- AccessKit roles from headless widgets; high-contrast and colorblind token sets;
  reduce-motion turns off transitions and animations (root class); screen-reader pass with
  Narrator.

## Screens

Framework (generic, data-driven): main menu shell, settings + keybinds + mod settings, **mod
manager with the sf_records conflict report and CSS errors**, save/load, HUD shell with slots,
inventory/container/barter (columns, search, categories, compare), dialogue, journal/quests,
map, stats (derived values with explanations and before/after deltas), spell list, loading
screen, notifications, message boxes (modal with buttons, or timed). Content pack: themes,
fonts, icons, column definitions, game-specific screens.

**Dialogue hyperlinks**: topic words are `TextSpan`s with click/hover observers (R1). For
gamepad, a link cursor moves through links in reading order using `run_geometry` bounds, and
shows focus with `TextBackgroundColor`; the topic list beside the text is a normal focusable
list, which is also how screen readers reach links (spans have no AccessKit nodes).

**Books, scrolls and notes**: rich-text documents and pagination are in [ui-text.md](ui-text.md).

**World-space UI**: nameplates, markers and prompts are screen-space nodes anchored to
entities, positioned in `UiSystems` after transform propagation (same frame); diegetic surfaces
render UI to a texture (`UiTargetCamera` on an image, `ViewportNode` picking).

## Dev UI

`sf_devui` (bevy_egui, inspector, Feathers, bevy_dev_tools) behind the `dev` feature, never a
dependency of game UI.

## Performance and compile time

Budgets (to calibrate at M5): HUD ≤ 0.5 ms CPU, heavy menu ≤ 1 ms CPU, UI GPU ≤ 0.3 ms at
1440p, all HUD Lua ≤ 0.2 ms, style recalculation 0 ms when nothing changed; a 1,000-item
inventory scrolls at full frame rate (virtualized, ~40 live rows). OpenMW for comparison:
9.8 ms for 1,028 widgets. Crate split follows §8b: `sf_ui` Bevy-free; `sf_ui_css` holds
flair and the Servo CSS crates so they compile once and leave other crates untouched;
`sf_ui_screens` changes often and has few dependents. No `bevy_feathers` in game crates.

## Milestone and first vertical slice

**M5** (Lua), on `.bsn` files once Bevy 0.21 ships them (PR #26043). Slice: data-driven HUD
(bars, notifications) + virtualized inventory (1,000 synthetic items, sort/search/tabs,
tooltip slot) + settings with mod settings + message box; both themes and both presets,
swapped live; gamepad-complete with repeat and rebinding; 100-200 % scale live; CSS and Lua
hot reload; AccessKit checked with Narrator; `sf_diag` gates. Done when a theme mod, a CSS
patch in its own layer, and a Lua mod adding an inventory column work together in both presets.

## Verification

Headless tests: registry build, screen replacement, slot order, composer output (layer order
= load order), CSS error attribution to mods. Screenshot tests per theme × preset × scale
(100 %, 200 % text at 1280×720). An `sf_diag` scenario with the inventory open and scrolling.
A gamepad-only pass over every stock screen. Hot-reload test: edit a mod's CSS, see it apply.

## As built: current state

What the code does where it differs from the sketches above. Details, tests and measurements:
[archive/ui-history.md](archive/ui-history.md) ("As built (U1a)" to "As built (U2)", "As built (U5)",
"As built (U6)", "Dialogue window view-model").

- **Records** `ui.screen`, `ui.slot` (one contribution each), `ui.column`, `ui.theme`, `ui.preset`,
  `ui.setting` are serde record kinds; the owner is the mod whose define a value is based on.
- **Slot and column order: priority, then load order** (not load order, then priority).
- `ListModel` has per-row versions, stable sort, filter and `window()` for virtualization.
- **Router** (`ScreenStack`, Bevy-free; `sf_ui_bevy`): one `bevy_enhanced_input` context entity per
  input context id at `UI_CONTEXT_PRIORITY` (2^16), only the input target's active; every UI action
  consumes its input and requires a reset; HUD contexts carry only `OpenMenu`/`OpenMap`. It runs in
  `PreUpdate` before `EnhancedInputSystems::Prepare`. Confirm triggers `bevy_ui_widgets::Activate`.
- **Navigation**: Bevy's `AutoDirectionalNavigator`, limited to the target screen's focusables;
  focus is remembered per screen; held directions repeat (0.4 s, then 0.08 s). `InputMode` follows
  the last-used device within the frame.
- **Profile**: settings and rebinding persist in the `sf_mods` profile (`[bindings.<context>]` with
  our text input names), written when no menu takes input or on exit. Fluent files are
  `l10n/<locale>/<mod id>.ftl`, attributed to the mod that ships them. `UiCommands` refuses a
  screen that doesn't support the active preset.
- **CSS** (`sf_ui_css`, bevy_flair 0.9.0): layer names are flat `mods-<load position>-<id>` (flair
  parses one identifier per layer), content is imported from the named asset source
  `sf_ui_content`, screen roots carry the preset/theme/a11y classes and `input=kbm|gamepad`, and
  each sheet is also probed alone to attribute errors to their mod (`UiCssReport`). Hot reload
  needs no extra code.
- **Built-in settings** (2026-10-22): framework settings without records, like the presets (owner `sf`,
  values in `[settings.sf]`): the audio page's volume slider and mute per bus (`AUDIO_BUSES`,
  `audio_volume_key`/`audio_muted_key`; audio.md "Volume mixer"). A record with a built-in's id retunes it.
- **Not done yet**: `UiSettings` isn't fed from `ui.setting` values; Lua (parked). (The game app now calls
  `insert_from_profile`; U6 added PlayStation/Switch glyph names.)

### Stock screens (`sf_ui_screens`, U3-U4, 2026-10-22 to 10-25)

Widget kit (parts carry `Name` + `UiPart` + `UiClass`; buttons, panels, value bars, `ScreenColors`,
`ScreenLabels`) and `KeyedChildren` (keyed reconciliation of view-model rows into child entities). Screens:
dialogue, journal ([journal-ui.md](journal-ui.md)), inventory (virtual icon grid, drag and drop, split
dialog, compare tooltips; `InventoryEvent` intents), keybinds, HUD (value bars, compass slot, crosshair, use
prompt). A game spawns a `UiScreenRoot` with the screen's component and pushes its id; screens whose
view-model owns focus carry `UiOwnsFocus`. Save/load (2026-10-25): `SaveLoadScreen` over an app-filled
`SaveLoadModel` (mode, `SaveEntryView` rows: id, order, title, subtitle, reason, thumbnail, damaged; no
`sf_save` dependency), listed newest first, "New save" on top in save mode with the app's generated name (typed
names wait for router text capture); focus selects, confirm runs load / save, overwrite and delete ask first
(cancel focused), Context1 deletes; choices go out as `SaveLoadEvent`, Back closes. Pause menu (2026-10-25):
`PauseMenuScreen` over an app-filled `PauseMenuModel` (`PauseEntry { choice, enabled }`, `PauseChoice` Resume, Save,
Load, Options, Quit; default all five enabled); focus starts on Resume, navigation skips disabled entries (class
`disabled`); Quit asks yes/no first (No focused; the question widget is shared with save/load); choices go out as
`PauseMenuEvent`; Back = Resume, which closes the screen. Its `ui.screen` record declares `pause = "WhileOpen"`, so
`ScreenStack::pauses_game` drives the app's pause. Options (2026-10-26): `OptionsScreen` over
`OptionsModel::from_settings(registry, store, localizer)`: `ui.setting` records grouped by `page` (else the owning
mod; built-in audio page first) as tabs (tab prev/next), rows toggle / choice (cycles) / int or float slider (record
step, else range/20; clamped), `Text` settings left out; left/right change, confirm flips or cycles, arrows click;
a "Key bindings" row and Back. The screen applies nothing: `OptionsEvent` `Set(key, value)`, `OpenKeybinds`, `Back`
(closes); the app writes `UiSettingsStore` and saves. No record condition yet: the app disables rows. Message boxes and toasts: [ui-text.md](ui-text.md)
"Notifications". Content packs bring names, icons, themes and host logic.

HUD in the game (2026-10-26): Bevyrrowind spawns the `HudScreen` root on its "hud" record and writes `HudView` from
the player (`mw_player_bevy/src/hud.rs`: static bars from the player record, camera heading, crosshair, the use
prompt from its own focus ray) and hides the root while a Menu/Modal screen or the console is open (the framework
doesn't hide lower layers). `UsePrompt::action` is optional: `None` is a name-only prompt, no glyph.

Main menu (2026-10-26, framework only): `MainMenuScreen` over `MainMenuModel` (app-ordered `MainMenuEntry { id,
label (localized), enabled }`, optional `quit` id that asks Yes/No first, optional `back` id Back chooses (else Back
does nothing), `title_image` / `background` image slots, `title_text`). Sends `MainMenuEvent { screen, id }`; never
closes itself. Parts `sf-main-menu-*`, each entry classed by its id.

### Dialogue window view-model (`sf_rpg_ui`, 2026-10-22)

Bevy-free logic of the dialogue screen over an `sf_rpg` session (rpg.md "Dialogue"); `sf_rpg_ui` is a leaf,
so `sf_ui` stays RPG-agnostic and `sf_rpg` presentation-free. Morrowind's scrollback, topic links and topic
list with Goodbye; new vs explored (Skyrim, Witcher 3); the pick echoed (Disco Elysium, BG3). Net-ready:
input → `DialogueRequest { Ask | Choose | Goodbye }` → host applies → `refresh`. Scrollback `Entry`s with
stable ids (cap 256): `Line` (`TextRun`s from `TopicLinker`, `LinkState` `New`/`Known`/`Unavailable`),
`Chosen`, `JournalUpdated`. Topic list `ListModel<RecordId, TopicRow>`. Modes `Talking`, `Choosing` (Back
does nothing), `GoodbyeOnly`, `Ended`. Focus has two regions: the topic list and a **link cursor** over
links in reading order (spans aren't layout nodes, so directional navigation can't reach them). No geometry
in the view: `take_scroll_request()`, `version()`.

### Inventory logic (U5, 2026-10-22)

`sf_ui_inventory` (Bevy-free), one view per holder (inventory, container, both barter sides); own
virtualization, as Stack decision 4 planned. `sf_ui` list model: sort chains, `ListFilter::one_of` tabs, word
search, `reorder`, per-row cached keys; `sf_ui::VirtualList<K>`: fixed row extent, keyed focus surviving sort
and filter, stable pool slots. `InventoryView::sync` diffs the `Loadout`; rows `ItemRow` (`inventory.item`,
`stat.<id>`) feed `ui.column` records; adapter hook `ItemCatalog` (`describe`, `stats`, `carried_weight`);
tabs are content data. A new column sorts text A→Z and numbers high→low (same column flips; ties by name,
then inventory order). `Encumbrance`, compare against what equipping would displace, split/merge commands.
1,000 stacks re-sort in 0.30 ms. Not done: equip/drop/transfer commands, variable row heights,
diacritic-folding search, barter prices.

### Keybinds (U6, 2026-10-22)

`sf_ui` (`input`, `keymap`, `rebind`), `sf_ui_bevy` (`actions`, `keybinds`, `profile`), screen in
`sf_ui_screens`. Over `bevy_enhanced_input` (no per-device slots, scopes or capture there). Bindings live
only in `Profile::bindings` (not BEI's `SettingsGroup`), per action only the devices that differ
(`confirm = { kbm = ["key:Space"] }`); unregistered actions' entries pass through. Input names are the model
(`key:Ctrl+KeyS`, `pad:South`, `stick:Left`), validated by the engine. `sf_ui::Keymap` (`ActionDef` with
slots, defaults, `required`, scopes): conflicts when scopes meet and inputs overlap (partial included),
`propose` → `apply(Swap | Unbind | KeepBoth)`, required actions keep a binding per device, resets,
`rows(context)`. Capture (`Rebinder`) arms after release, has no cancel input (5 s timeout), and blocks UI
actions until release. `UiKeymap` is the source of truth (games link their scope to `HUD_SCOPE`); glyphs per
`GamepadFamily`. Leftovers: layout-aware key labels (AZERTY), per-pad bindings for local co-op.

## Industry comparison

J9 (2026-10-12), table in the [archive](archive/ui-history.md#industry-comparison): matches on every
technique, beyond on cascade layers for load order. Gaps: subtitles and safe area (open questions 5-6);
unbatched icon draws, no job unless the inventory budget fails (fix: a bake-time icon atlas).

## Open questions

1, 2, 4 closed (MPL-2.0 allowed unmodified in `sf_ui_css`; hot reload via the `sf_ui_content` watcher;
`bevy_enhanced_input` 0.27: its prioritized, consuming input contexts are CommonUI's router rule, glyphs and
device detection stay ours, bindings live in `Profile::bindings`); full text in the archive.

3. If M5 starts before Bevy 0.21: stock screens start as `bsn!` templates in Rust (same scene
   model, no file loader) and move to `.bsn` files when they land.
5. **Subtitles (J9, P2).** A `subtitles` settings group (on/off, size scale independent of text
   scale, background opacity, speaker name, max two lines, minimum display time) and a
   `Layer::Notification`-level caption widget fed by a `Caption { speaker, text, duration }`
   message from the dialogue/audio side. XAG 104 defaults.
6. **Safe area (J9, P2).** A `safe_area` margin (percent per edge, default 0 on PC, 5 % in
   the controller preset when fullscreen on a TV-size display; plus OS-reported insets where
   winit exposes them) applied as padding on the HUD and menu layer roots, adjustable with a
   calibration screen like console titles.
