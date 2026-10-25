# Game UI

Status: draft, design complete (2026-10-11); owner decisions recorded below. Built from M5.
Crates (proposed): `sf_ui` (Bevy-free: records, registry, view-models, Fluent),
`sf_rpg_ui` (Bevy-free RPG screen view-models: dialogue),
`sf_ui_bevy` (runtime, widget kit, layers, input router), `sf_ui_css` (bevy_flair
integration and stylesheet composer, isolated on purpose), `sf_ui_screens` (stock screens),
`sf_ui_lua` (Luau bindings, part of `sf_script`'s API surface), `sf_devui` (dev tools only).
Built so far: `sf_ui`, `sf_ui_bevy`, `sf_ui_css` (U1a-U2, "As built: current state" below).
Research findings, as-built logs and closed questions: [archive/ui-history.md](archive/ui-history.md).

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

Full findings (2026-10-11) in [archive/ui-history.md](archive/ui-history.md#research-findings):
R1 Bevy 0.20 native (layout, headless widgets, focus and navigation, span-level text picking,
parley text, AccessKit, BSN), R2 crates, R2b bevy_flair, R3 toolkits outside Bevy, R4 what
players and modders hated and loved (OpenMW, MWSE, SkyUI, WoW, Factorio), R5 controller-first
and readability lessons (XAG 101, Starfield, BG3, Elden Ring). What the design keeps from them is
in the sections below; R2b's limits and fallback plan stay here because they bind the mod contract.

### R2b. bevy_flair: licence, limits, fallback

- **Licence**: the crate is `MIT OR Apache-2.0`; its Servo dependencies (`cssparser`, `selectors`,
  `cssparser-color`, ...) are `MPL-2.0`, allowed unmodified (owner, 2026-10-11) and contained in
  `sf_ui_css`.
- **Limits**: one stylesheet per entity and no global stylesheets (we compose one sheet per
  layer root, below); `!important` ignored; no `var()` fallback; `calc()` can't mix `%` and
  `px` until Bevy's `Val::Calc` (#25653); media features are fixed (no custom ones), so presets
  and input mode are root classes/attributes instead.
- **Fallback plan** (also in ARCHITECTURE §4): (a) until 0.9 publishes, pin `main` by `rev`;
  (b) if it lags a Bevy release by more than our window, carry a `[patch]` fork (its Bevy
  surface is bevy_ui/text/picking/asset, so ports are mechanical); (c) if abandoned, the mod
  contract survives: it is *the CSS subset and property names documented in this spec*, not
  flair's API, so `sf_ui_css` could be reimplemented on `cssparser` + `selectors` (or
  `lightningcss`) without touching mod files; (d) never let flair types leak out of `sf_ui_css`.
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

Sketch for the M5 slice; names are provisional.

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
    pub fn build(records: &RecordDb, mods: &LoadOrder) -> (Self, Vec<UiConflict>);
    pub fn screen(&self, id: &ScreenId) -> Option<&ScreenDef>;
    pub fn slot(&self, id: &SlotId) -> &[SlotContribution];        // already ordered
}

/// MVVM: a view-model is a Reflect struct with a change tick; lists are keyed.
pub trait ViewModel: Reflect + Send + Sync + 'static { const KIND: &'static str; }
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

Luau surface (placeholder until `sf_script`'s API rules apply; `api_version`-gated):

```lua
local ui = require("@sf/ui")   -- require-by-string alias (scripting.md "Modules and require")
ui.slot("inventory.columns"):add{ id = "value_per_weight", header = "col-value-weight",
  width = "5rem", sort = "number", value = function(item) return item.value / math.max(item.weight, 0.01) end }
local compass = ui.create{ layer = "hud", scene = "ui/my_mod/compass.bsn", class = "compass" }
compass:set("heading", 90)                        -- fine-grained setter on a handle, no tree re-diff
ui.list(model, "ui/my_mod/row.bsn")               -- virtualized
ui.settings.page{ id = "my_mod", title = "my-mod-title",
  { key = "show_compass", type = "bool", default = true } }
ui.screens.replace("inventory", { lua = "scripts/my_mod/inventory.lua" })
ui.on("screen_opened", function(id) end)
local w, h = ui.measure_text("Hello", { class = "body" })   -- synchronous
```

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

## As built: current state (U1a-U2, 2026-10-18)

What the code does where it differs from the sketches above. Details, tests and measurements:
[archive/ui-history.md](archive/ui-history.md) ("As built (U1a)" to "As built (U2)").

- **Records** `ui.screen`, `ui.slot` (one contribution each), `ui.column`, `ui.theme`, `ui.preset`,
  `ui.setting` are serde record kinds; the owner is the mod whose define a value is based on.
  `UiRegistry::build(&RecordDb)` takes no `LoadOrder` (record sources are already mods in load order).
- **Slot and column order: priority, then load order** (not load order, then priority).
- **`ViewModel` is not `Reflect`** (`KIND`, `fields()`, `field(name) -> UiValue`), so `sf_ui` stays
  Bevy-free. `ListModel` has per-row versions, stable sort, filter and `window()` for virtualization.
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
- **Not done yet**: the game app doesn't call `insert_from_profile`; `UiSettings` isn't fed from
  `ui.setting` values; PlayStation/Switch glyph names; widgets (U3) and Lua (parked).

## Dialogue window view-model (`sf_rpg_ui`, 2026-10-22)

The logic of the stock dialogue screen, Bevy-free, over an `sf_rpg` session (rpg.md "Dialogue"). No rendering:
`sf_ui_screens` binds it later. Crate `sf_rpg_ui` (deps `sf_rpg`, `sf_ui`, `serde`): a leaf, so `sf_ui` stays
RPG-agnostic and `sf_rpg` stays presentation-free (net rule 3); journal and inventory view-models go here too.

**Research.** Morrowind: scrollback of answers with the topic as a heading, topic hyperlinks in the text, a sorted topic
list beside it with Goodbye under it, choices as links at the end of the text, "journal updated" inline. Skyrim, Fallout
4, The Witcher 3: options already explored are dimmed, unexplored ones bright (our `new`). Disco Elysium and Baldur's
Gate 3: the log echoes the player's pick; numbered choices for keyboard. Yarn Spinner for Bevy (`bevy_yarnspinner` 0.8,
2026-03; its `example_dialogue_view` crate) renders a node-graph runner's lines and options through Bevy events: graph
model, Bevy-coupled, no topic list or links (rpg.md "Reuse"), so not reusable here. No Bevy-free dialogue view-model
crate exists; ours reuses `sf_ui::ListModel` for the topic list (sort, search, per-row versions).

**Data flow** (net-ready: the view never touches game state; requests are what a client sends):
input (`nav(DialogueNav)` or `activate(DialogueFocus)` for a click) → `DialogueRequest { Ask(topic) | Choose(value) |
Goodbye }` → host `request.apply(&mut session, db, rules)` → `apply_outputs(db, &outputs)` (lines) → result scripts →
`apply_events(&rpg_events)` (notices) → `refresh(db, state, available, known)` (or `refresh_from(session, ..)` on the
host).

- **Scrollback**: `Entry { id: EntryId, kind }`, ids monotonic and stable when old entries drop (cap
  `DEFAULT_MAX_ENTRIES` = 256, `with_max_entries`). Kinds: `Line(ViewLine { topic, response, heading, runs })`,
  `Chosen { label }` (the echo), `JournalUpdated { quests }` (consecutive journal entries of one step share one notice;
  the screen formats it with Fluent and may also toast it). `heading` = topic name for `Conversation` topics, none for
  greetings and service replies.
- **Hyperlinks**: a line's text is split by the db's `TopicLinker` into `TextRun { text, link: Option<TopicLink {
  topic, state }> }` (one `TextSpan` each). `LinkState`: `New`, `Known`, `Unavailable` (the speaker has nothing on it:
  styled, not followable). States are restyled on every refresh and ask.
- **Topic list**: `ListModel<RecordId, TopicRow { name, new, enabled }>` sorted by name (case-insensitive), rows = the
  session's `available_topics`; `new` = no heard line under the topic in `Topics` and not asked in this conversation;
  `enabled` only while talking. `set_topic_search` filters by name (PC-dense).
- **Mode** from the session state: `Talking`, `Choosing` (only choices live), `GoodbyeOnly` (forced goodbye),
  `Ended` (`is_closed`). `choices()`, `can_goodbye()`, `links_active()`; `DialogueView` is a `ViewModel` (`speaker`,
  `mode`, `can_goodbye`, `links_active`).
- **Focus** (`DialogueFocus::{Topic, Link { entry, run }, Choice, Goodbye}`), driven by `DialogueNav::{Prev, Next,
  PrevRegion, NextRegion, Confirm, Back}`; the stock screen maps up/down/left/right/confirm/back to them. Two regions
  while talking: the topic list (rows, then Goodbye) and the **link cursor** (followable links in reading order, ui.md
  "Screens": spans are not layout nodes, so Bevy's directional navigation can't reach them). Prev/Next clamp, no wrap
  (held-direction repeat). PrevRegion enters the text on the newest line with a live link; NextRegion returns to the
  remembered row (or the row now at its index). Choosing: choices only, Back does nothing (a choice must be made).
  Forced goodbye: Goodbye only. Back = goodbye otherwise. A focus the view placed itself follows the best target until
  the player moves it (the greeting arrives before the topic list); an invalid focus falls back after every change
  (trimmed line, search, mode change).
- **Scroll**: `take_scroll_request() -> Option<EntryId>`: the new line (read from its top) or the line under the link
  cursor; the screen scrolls, the view holds no geometry.
- `version()` changes on any change (view counter + the topic list's), for cheap rebinding.

Tests (`cargo test -p sf_rpg_ui`, 5): a host test double (response table with conditions, rendered texts, result
scripts that offer choices, set stages with journal entries and force goodbye) drives a whole conversation by pad input:
greeting runs and link states, sorted list with new/known, ask, choice lock and echo, merged journal notice, link cursor
order and follow, goodbye; pointer activation and search; scrollback cap with a trimmed focused link; no linker / no
greeting.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Topic list + hyperlinked keywords | Morrowind (keyword topics, links in text) | same, from `TopicLinker` spans | **match** |
| Explored vs unexplored options | Skyrim/Fallout 4 dim explored lines; Witcher 3 greys read options | `TopicRow::new`, `LinkState` | **match** |
| Player line echoed in the log | Disco Elysium, BG3 | `EntryKind::Chosen` | **match** |
| Numbered choices (1-9 keys) | BG3, Disco Elysium | `activate(Choice(i))`; the key map is the screen's | **match** (screen side) |
| Gamepad-reachable inline links | Morrowind links are pointer-only (unverified for later ports) | link cursor region | **beyond** |

## As built (U5, 2026-10-22): inventory logic

`superfargoth/crates/sf_ui_inventory`, Bevy-free (deps `sf_ui`, `sf_rpg`; no rendering), plus list
extensions in `sf_ui` and stack commands in `sf_rpg` (rpg.md). 8 tests + 1 timing report, `sf_ui` 6 new,
`sf_rpg` 2 new; clippy pedantic clean. Serves inventory, container and both barter sides (one view per holder).

**Research.** SkyUI (Nexus 12604/49095 pages): stat columns sortable, equipped/stolen/enchanted shown as
multi-state columns, sorting by equipped groups worn items but keeps names A→Z inside, full-text search;
its known bug: the sort resets when a view lacks the column (our tabs filter one list with one column set,
so the sort survives tab switches). Unity UI Toolkit `ListView` (manual 2022.3): `FixedHeight`
virtualization is the fast path, rows recycled through `makeItem`/`bindItem`/`unbindItem`; Unreal
`UListView` pools entry widgets the same way (unverified this session). Compare-with-equipped: R5 (Elden
Ring / Diablo before/after deltas). **Reuse**: Bevy 0.20 has a scroll area but no virtualization or list
models (R1); `bevy_scroller` 0.1 is Bevy 0.11-era and not virtual; `virtualizer` (docs.rs, not fetched
here) is range math only, the ~10 lines we'd save, without keyed focus, slot recycling or model
integration; `egui_virtual_list`/floem are other toolkits. Own, as Stack decision 4 planned.

- **`sf_ui` list model**: sort *chains* (`set_sort_keys`: column, then name, then insertion order);
  `ListFilter::one_of` (a tab over several categories) and word search (every whitespace word must appear,
  any order: "iron sw" finds "Iron Longsword"); `reorder(keys)` moves rows without bumping row versions;
  sort keys and lowercased search text are **cached per row** until the row, sort or searched fields
  change (a rebuild after one changed row re-sorts cached keys). `iter()`, `position()`.
- **`sf_ui::VirtualList<K>`**: fixed row extent, viewport, overscan (2), clamped scroll (wheel, rows,
  scrollbar thumb fractions), `index_at(y)` hit test; **keyed focus** that survives sort and filter (a row
  that disappears hands focus to the row now at its index); `move_focus`, `page` (visible rows - 1),
  `reveal_focus`; `update(&mut ListModel) -> ListFrame` assigns **stable pool slots** (keys that stay in the
  window keep their widget; `rebind` only for new keys or changed row versions; released slots are reused
  before the pool grows). Row-extent changes (UI/text scale) scale the scroll so the same rows stay.
- **`InventoryView`** (holder, tabs, row extent, viewport): `sync(&Loadout, &SlotTable, &dyn ItemCatalog)`
  diffs the loadout: unchanged stacks keep their row (no `describe`, no rebind), equipment-only changes
  update markers, rows follow inventory order through `reorder`; `refresh` re-describes all (mod reload,
  language). Row view-model `ItemRow` (`inventory.item`): `name category count weight total_weight value
  total_value value_per_weight equipped slots icon` and `stat.<id>`, so `ui.column` records bind them.
- **Adapter hook `ItemCatalog`**: `describe(&ItemStack) -> ItemInfo { name, category, weight, value, icon,
  stats }`, `stats() -> &[StatDef { id, label, better: Higher|Lower|Neither }]`, and `carried_weight(stack,
  info, equipped)` (default: every piece counts, Bethesda's rule). Tabs are `InventoryTab { categories,
  equipped_only }` data from the content pack; `tab_counts()` ignores the search (badges).
- **Sort**: `sort_by_column(&ColumnDef)`: a new column sorts text A→Z and numbers high→low (our choice:
  heaviest, most valuable first), the same column flips; ties go by name, then inventory order; Lua columns
  refuse until the Lua UI. Optional equipped-first pinning (off by default).
- **Totals**: `Encumbrance { carried, capacity }` (`ratio`, `is_over`, `remaining`, `after(delta)` for
  take-all/barter previews; what "over" does is the game's), total value, stacks, pieces.
- **Compare** (`compare(stack, &Loadout, &dyn EquipRules, &dyn ItemCatalog)`): against what equipping
  would displace (the placement's slots, exactly what `Loadout::equip` takes off), each stat summed over
  them (a two-hander against sword + shield), deltas `Better/Worse/Same/Changed` by `StatDef::better`;
  a worn item compares with itself; unequippable → `None`.
- **Split/merge**: `QuantityPicker` (split: 1..=count-1, starts at half; `step`, `set_fraction`);
  `actions()` for the action bar; `split_command` / `merge_command` build `RpgCommand::SplitStack` /
  `MergeStacks` after the same checks the host runs; `on_event` moves focus to the split-off or merged-into
  stack and says when to re-sync.
- **Timing** (`cargo test -p sf_ui_inventory --release -- --ignored --nocapture`, cloud Linux container,
  not the reference laptop): 1,000 stacks: re-sync 43 µs, one new stack sync + frame 0.11 ms, re-sort
  0.30 ms, search 0.26 ms, scroll frame 3.4 µs; 5,000: 0.29 / 1.06 / 1.78 / 1.58 ms / 5 µs; 20,000: 1.6 /
  7.7 / 11 / 7.7 ms. Within the menu budget (≤ 1 ms) at the 1,000-item target; a re-sort at 5,000+ is a
  one-frame hitch on the click.
- Not done: the Bevy screen (`sf_ui_screens`, needs the U3 widget kit), equip/unequip/drop/transfer
  commands (equip needs the adapter's rules on the host), variable row heights, grid (icon) layout,
  diacritic-folding search, barter price columns (the adapter's formulas).

## As built (U6, 2026-10-22)

Keybinds screen logic: Bevy-free in `sf_ui` (`input.rs`, `keymap.rs`, `rebind.rs`), engine side in
`sf_ui_bevy` (`actions.rs`, `keybinds.rs`, `profile.rs`). 13 `sf_ui` tests + 2 `sf_ui_bevy` tests, clippy
pedantic clean. The screen widget itself waits for the widget kit (U3); this is everything under it.

- **Reuse check (rule 8)**: Bevy 0.20 has no rebinding model. `bevy_enhanced_input` 0.27 binds inputs and
  its `keybinding_menu` example rebinds, but has no slots per device, conflict scopes, required actions or
  capture flow, and persists through Bevy's `SettingsGroup` (needs its `serialize` feature, U1c). We keep BEI
  for evaluation and build the keymap over it. `bevy_input_prompts` 0.2 (glyphs, MIT) is on Bevy 0.16 and
  ties glyphs to the Kenney/Xelu asset packs: not adopted; our glyphs are text plus icon ids a content pack maps.
- **Open question 4 closed (bypass `SettingsGroup`)**: bindings persist only in the profile
  (`Profile::bindings`), so profiles stay the single source; the keymap mirrors `SettingsStore`
  (`load_from(&Profile)` / `save_to(&mut Profile) -> changed`, saved by `sync_profile` on leaving the menus).
- **Input names are the model.** The keymap stores the U1c text names (`key:Ctrl+KeyS`, `pad:South`, ...) plus
  new `stick:Left|Right` (a whole stick for 2D actions; the engine side expands it to both axes). `sf_ui` knows
  only the grammar: device class by prefix, modifiers, canonical spelling, overlap. The engine checks that a
  name exists through `Keymap::set_validator` (`sf_ui_bevy::validate_input`). `Device` moved to `sf_ui`
  (re-exported by `sf_ui_bevy`).
- **Keymap** (`sf_ui::Keymap`): `ActionDef { id: (context, action), label (Fluent id), shape: Button | Stick,
  slots [kbm, gamepad], defaults, required, scopes }`. Conflicts: two actions collide when a scope meets
  (equal, or linked with `link_scopes`) and their inputs overlap: the same input, or **partial** (a key
  without modifiers fires on its chord in BEI, `any_key` on every key, `stick:Left` on `axis:LeftStickX`).
  `allow_shared` exempts a pair. `propose(slot, input) -> Proposal { previous, conflicts }`, then
  `apply(proposal, Swap | Unbind | KeepBoth)`; refused when stale or when a required action would lose its
  last binding on a device. Moving an input between two slots of the same action swaps them. `unbind`,
  `set` (code/scripts), `reset(action, device?)`, `reset_context`, `reset_all`; `rows(context)` is the
  screen's view-model (slots per device, `conflicted`, `modified`).
- **Profile format**: per action, only the devices that differ, `confirm = { kbm = ["key:Space"] }` (an empty
  list unbinds that device). U1c's flat list still loads (it sets every device). Unregistered actions'
  entries pass through and apply when the action registers later (a disabled mod's actions keep their keys).
  Bad names, unknown devices, too many inputs and wrong shapes are reported (`KeymapIssue`) and dropped; a
  required action left unbound gets that device's defaults back.
- **Capture** (`sf_ui::Rebinder`): arms only once every input is released (the A/Enter/click that started it
  is never captured); ignores other devices and shapes; a modifier key waits for a chord and binds alone if
  released alone; **no cancel input** (every input is bindable, menu open on Esc included), so capture ends on
  a 5 s timeout or `cancel()`; after a capture UI actions stay blocked until every input is released, so a
  captured Esc or B doesn't also close the menu. A conflict waits in `pending()` for `resolve`.
- **Bevy side**: `UiKeymap` (resource, `ui_keymap()` = the UI actions; a game registers its gameplay actions
  in it and links its scope to `HUD_SCOPE`, where open menu/map live) is the source of truth; `UiBindings` is
  derived from it (`apply_keymap`, only when different), then respawns contexts as before. `UiRebinder` +
  `drive_rebind` (`PreUpdate`, first in `UiSystems::Route`) feed keys, mouse buttons, pad buttons and stick
  crossings as names and emit `UiRebindEvent`s; `dispatch_actions` and `navigate` run only under
  `ui_input_open`. `GamepadFamily` follows the last-used pad's vendor id (Sony 0x054C, Nintendo 0x057E,
  else Xbox). `glyph(&UiKeymap, action, device, family) -> Glyph { label, icons }`: Xbox / PlayStation /
  Switch names (Bevy's pad buttons are positional: `South` is A, Cross or B), key names, chords as
  `Ctrl+S`; icon ids such as `pad-ps-South` and `mod-Ctrl` for atlases (U3) and `glyph-<id>` Fluent rewording.
- Tests: input grammar and overlap; glyphs per family; registration checks; conflicts per scope (UI vs
  gameplay, HUD linked, shared Esc, partial chord); swap / unbind / keep-both, stale and required refusals;
  slot moves; resets; profile tables (only changed devices, U1c form, issues, pass-through and late
  registration); capture (arming, device filter, draining, chords, lone modifier, sticks, conflict, timeout,
  refused swap). Headless Bevy: a pad capture rebinds menu open while the A that started it is ignored, a
  B capture conflicts with back without closing the menu, the swap drives the engine bindings.
- Leftovers: keyboard-layout-aware key labels (`KeyQ` reads Q on AZERTY; Bevy 0.20 has no physical → logical
  lookup outside key events); the screen widget and icon atlases (U3); per-pad bindings for local co-op
  (BEI `GamepadDevice`); gameplay actions are registered by the game (Bevyrrowind) once it has a gameplay
  context.

## Industry comparison

J9 (industry-audit.md), 2026-10-12, docs research only (`.sessions/j9rest.md`). R3–R5 (archive)
already compare toolkits and player lessons; this table gives verdicts per technique.
"Unverified" marks claims from memory not re-read this session.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Layered screens, activatable stacks, input routing to the top widget | UE CommonUI (layers, activatable widget stacks, input routing, action bars) | same model (`Layer`, router) | **match** |
| Data binding | UE5 UMG Viewmodel (MVVM plugin); Unity 6 UI Toolkit runtime data binding; XAML/NoesisGUI; Gameface data binding (unverified detail) | view-models + `Bind`, keyed list models | **match** |
| Styling as stylesheets | Unity UI Toolkit USS (a CSS subset) + UXML; Coherent Gameface (HTML/CSS, used by many recent PC titles, unverified per title); Bethesda Skyrim/Fallout 4 and RAGE (GTA V) on Scaleform (Flash), now discontinued (Autodesk ended it ~2020, unverified date) | CSS with cascade layers via bevy_flair | **match** (UI Toolkit/Gameface family). Cascade layers for mod load order go beyond them |
| Moddable built-in screens | SkyUI/MCM on Scaleform swf replacement; WoW open Lua/XML UI; Factorio relative GUI | records + slots + Lua + CSS layers | **match** (designed around their lessons, R4) |
| Gamepad navigation without a cursor | console practice; CommonUI; Unity UI Toolkit navigation events | bevy_input_focus + directional navigation | **match** |
| Device-following glyphs, full rebinding | CommonUI input data + platform glyphs; XAG 107 (input remapping) | yes | **match** |
| Text: shaping, bidi, fallback, crisp at any scale | UE Slate (HarfBuzz/ICU, font atlas per size, unverified detail); Unity TextMeshPro / UI Toolkit (SDF atlases); Gameface (SDF, unverified) | parley + swash, per-size atlases | **match** for screen UI. SDF would matter only for large world-space text and continuous zoom animation; none planned |
| Virtualized long lists | UE `ListView`/`TileView` (entry widget pooling), Unity UI Toolkit `ListView` (recycling) | virtualized list (~40 live rows) | **match** |
| Accessibility: text size, contrast, colorblind, screen reader, reduced motion | Xbox Accessibility Guidelines 101 (text), 102 (contrast), 103 (non-color cues), 112 (screen narration), 117 (motion); UE/Unity AccessKit-like screen reader support is partial (UE Slate accessibility, unverified) | yes (AccessKit, token sets, reduce-motion) | **match** |
| **Subtitles/captions settings** | XAG 104 (subtitle size, background, speaker name, ≤ 2 lines); standard options menu entry in modern RPGs | not in this spec (dialogue text exists; voiced lines have no caption style) | **GAP (P2, small)**: open question 5 |
| **Safe area** (TV overscan, handheld/notched displays) | UE `SafeZone` widget + `r.DebugSafeZone`; Unity `Screen.safeArea`; console certification (title-safe) | none (layout is full window) | **GAP (P2, small)**: open question 6. The controller preset targets TV, where it matters |
| Draw batching of icons | sprite/icon atlases (Unity Sprite Atlas, Slate texture atlas for small brushes, unverified) | Bevy 0.20 UI breaks a batch per image change (`bevy_ui_render` `prepare_uinodes`) | **differ, no job**: ~40 visible rows cost ~40–80 small draws; revisit only if the inventory budget fails (a bake-time icon atlas would fix it) |

**Sources** (checked 2026-10-12, beyond those in R1–R5):
- Xbox Accessibility Guidelines (101 text, 104 subtitles and captions, 112 UI navigation and narration): https://learn.microsoft.com/en-us/gaming/accessibility/guidelines (index; individual pages not re-read)
- UE CommonUI overview and input guide (cited in Canonical technique); UE UMG Viewmodel: https://dev.epicgames.com/documentation/en-us/unreal-engine/umg-viewmodel-for-unreal-engine (not re-read)
- Unity UI Toolkit (USS, UXML, runtime binding, ListView): https://docs.unity3d.com/Manual/UIElements.html (not re-read)
- UE safe zone widget: https://dev.epicgames.com/documentation/en-us/unreal-engine/umg-safe-zones-in-unreal-engine (not re-read; unverified URL)
- Bevy 0.20 `bevy_ui_render/src/lib.rs` batching (local registry).


## Open questions

1. ~~MPL-2.0 dependencies~~ **Closed: allowed unmodified, contained in `sf_ui_css`** (owner,
   2026-10-11). Full text in the archive.
2. ~~Hot reload over `sf_vfs`~~ **Closed (U2)**: the `sf_ui_content` source's watcher reloads
   edited sheets (archive, "As built (U2)").
3. If M5 starts before Bevy 0.21: stock screens start as `bsn!` templates in Rust (same scene
   model, no file loader) and move to `.bsn` files when they land.
4. ~~`bevy_enhanced_input` vs `leafwing-input-manager`~~ **Closed 2026-10-18 (u1a): `bevy_enhanced_input`
   0.27**: its input contexts with priority and input consumption are CommonUI's router rule. Device
   glyphs and last-used-device detection stay ours. Full comparison in the archive.
   **U6 (2026-10-22): bypass.** Bindings live in `Profile::bindings` through `sf_ui::Keymap` (As built U6).
5. **Subtitles (J9, P2).** A `subtitles` settings group (on/off, size scale independent of text
   scale, background opacity, speaker name, max two lines, minimum display time) and a
   `Layer::Notification`-level caption widget fed by a `Caption { speaker, text, duration }`
   message from the dialogue/audio side. XAG 104 defaults.
6. **Safe area (J9, P2).** A `safe_area` margin (percent per edge, default 0 on PC, 5 % in
   the controller preset when fullscreen on a TV-size display; plus OS-reported insets where
   winit exposes them) applied as padding on the HUD and menu layer roots, adjustable with a
   calibration screen like console titles.
