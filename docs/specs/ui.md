# Game UI

Status: draft, design complete (2026-10-11); owner decisions recorded below. Built from M5.
Crates (proposed): `sf_ui` (Bevy-free: records, registry, view-models, Fluent),
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
- **Not done yet**: the game app doesn't call `insert_from_profile`; `UiSettings` isn't fed from
  `ui.setting` values; PlayStation/Switch glyph names; widgets (U3) and Lua (parked).

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
5. **Subtitles (J9, P2).** A `subtitles` settings group (on/off, size scale independent of text
   scale, background opacity, speaker name, max two lines, minimum display time) and a
   `Layer::Notification`-level caption widget fed by a `Caption { speaker, text, duration }`
   message from the dialogue/audio side. XAG 104 defaults.
6. **Safe area (J9, P2).** A `safe_area` margin (percent per edge, default 0 on PC, 5 % in
   the controller preset when fullscreen on a TV-size display; plus OS-reported insets where
   winit exposes them) applied as padding on the HUD and menu layer roots, adjustable with a
   calibration screen like console titles.
