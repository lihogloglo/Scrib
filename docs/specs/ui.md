# Game UI

Status: draft, design complete (2026-10-11); owner decisions recorded below. Built from M5.
Crates (proposed): `sf_ui` (Bevy-free: records, registry, view-models, Fluent),
`sf_ui_bevy` (runtime, widget kit, layers, input router), `sf_ui_css` (bevy_flair
integration and stylesheet composer, isolated on purpose), `sf_ui_screens` (stock screens),
`sf_ui_lua` (Luau bindings, part of `sf_script`'s API surface), `sf_devui` (dev tools only)

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

### R1. Bevy 0.20 native (checked in the local 0.20.0 sources)

- **Layout**: `bevy_ui` on `taffy` 0.14: Flexbox and CSS Grid. `Val` has `Px`, `Percent`,
  `Vw`, `Vh`, `VMin`, `VMax`, `Em` (node font size, `EmSize`) and `Rem` (`RemSize` resource).
  `UiScale` multiplies only `Px`. `UiTransform`, `ZIndex`/`GlobalZIndex`, `ScrollPosition`,
  `Outline`, `BoxShadow`, `TextShadow`, gradients, nine-slice (`ui_texture_slice`),
  `UiMaterial` (custom WGSL per node), `ViewportNode` (a camera's render target in a node, with
  picking forwarded), `UiTargetCamera` (UI on any camera, including an image target: diegetic
  UI). `experimental::GhostNode` flattens wrappers out of layout.
- **Headless widgets** `bevy_ui_widgets`: button, checkbox, radio, slider, scrollbar, scroll
  area, list, menu, popover, dialog, modal, tabs, text input; tree view merged upstream
  (#25821). Controlled state (`ValueChange<T>`/`Activate`), `*_self_update` for uncontrolled.
  All but popover set an AccessKit role.
- **Feathers** (`bevy_feathers`): its crate docs say it is *"deliberately not intended"* for
  game UI. **Dev tools only**; we borrow its token pattern.
- **Focus and navigation**: `bevy_input_focus` (`InputFocus`, bubbling `FocusedInput`, tab
  navigation, manual `DirectionalNavigationMap`, autofocus) plus bevy_ui's
  `AutoDirectionalNavigation` (edges from screen position, manual edges override). Its docs
  argue against gamepad virtual cursors, matching console practice.
- **Picking and text links**: the UI picking backend resolves hits **per text span**:
  `pick_ui_text_section` maps the pointer to `TextLayoutInfo::run_geometry[i].section_index`
  and returns that `TextSpan`'s entity (`bevy_ui/src/picking_backend.rs`). So a dialogue
  hyperlink is a `TextSpan` child with `Pointer<Click>`/`Over` observers; `Underline`,
  `UnderlineColor` and `TextBackgroundColor` style it. Gaps: spans are not layout nodes, so
  directional navigation and AccessKit don't see them (handled in Screens, below).
- **Text**: `bevy_text` uses **parley 0.11** (shaping, bidi, line breaking) and **swash**
  rasterization into per-size glyph atlases. Fonts load from **TTF/OTF only**. Text is
  re-rasterized at each size, so it is crisp at every UI scale without SDF (the OpenMW
  blurry-bitmap problem doesn't arise). CSS-style family lists (`FontSource::Families`) give
  fallback chains; `FontFeatures`/`FontVariations`, `LetterSpacing`, `LineHeight`;
  `InlineBox` and `InlineImage` put icons inside text runs (button glyphs, book images).
  Features `complex_script_segmentation`, `system_font_discovery`; text editing with IME.
- **Accessibility**: `bevy_a11y` (AccessKit: Windows UIA, macOS, Unix).
- **Scenes**: BSN: `bsn!`, templates, field-level patching, composition, cached scene assets,
  `on(...)` observers, `#Name` references. **The `.bsn` file format is not in 0.20**: PR
  #26043 "BSN: file format lexing, parsing, and AST" is open on the **0.21** milestone, with
  a loader queued after it (GitHub, 2026-10-11). Incremental UI layout (#25653, enables
  `Val::Calc`) is open, unmilestoned.
- **Dev tools**: FPS/diagnostics overlays, frame graph, UI debug overlay, picking debug,
  `inspection` module.

### R2. Crates (crates.io API, 2026-10-11, three days after Bevy 0.20)

| Crate | Latest | Bevy | License | Note |
|---|---|---|---|---|
| `bevy_flair` | 0.8.1 (2026-08); 0.9 on git `main` | 0.19; **0.20 on main** | MIT/Apache | **adopted for styling (R2b)** |
| `bevy_egui` | 0.43.1 | 0.20 | MIT | dev UI only |
| `bevy-inspector-egui` | 0.37.0 | 0.19 | MIT/Apache | dev inspector |
| `bevy_enhanced_input` | 0.26.0 | 0.19 | MIT/Apache | input contexts with priority, rebinding |
| `leafwing-input-manager` | 0.21.0 | 0.19 | MIT/Apache | input fallback |
| `bevy_fluent` | 0.15.0 | 0.19 | MIT/Apache | or `fluent-bundle` directly (Bevy-free) |
| `bevy_ui_anchor` | 0.12.0 | 0.19 | MIT | world-anchored nodes; evaluate |
| `bevy_hui` | 0.8.0 (2026-10-11) | 0.20 | MIT/Apache | HTML-like templates; not needed with BSN |
| `bevy_lunex` | 0.7.0 | 0.19 | MIT/Apache | parallel UI stack: loses widgets/focus/a11y |
| `haalka` | 0.7.1 | 0.18 | MIT/Apache | FRP signals; lagging |
| `bevy_cobweb_ui` | 0.22.2 | 0.17 | MIT/Apache | archived 2026-01 |
| `sickle_ui`, `bevy_quill`, `bevy-ui-navigation`, `bevy_ecss` | old | ≤ 0.14 | MIT/Apache | abandoned; ideas went upstream |
| `slint` | 1.18.1 | n/a | GPL / royalty-free / commercial | license excludes it |

### R2b. bevy_flair (github.com/eckz/bevy_flair), checked 2026-10-11

- **Status**: 164 stars, one main author (eckz, ~118 of ~128 commits; bus factor 1). Releases
  follow Bevy closely: 0.6 (2025-11), 0.7 (2026-02), 0.8.0 (2026-06-22, about a week after Bevy
  0.19), 0.8.1 (2026-08). "Bevy 0.20 (#60)" merged to `main` on 2026-10-02 against
  0.20 release candidates; **0.9 is not on crates.io yet**. Last push 2026-10-02. ~34k lines.
- **License**: crate is `MIT OR Apache-2.0` (the repo's LICENSE file is MIT). **But it depends
  on Servo's `cssparser`, `selectors` and `cssparser-color`, all `MPL-2.0`.** MPL-2.0 is
  file-level copyleft: using the unmodified crates is compatible with shipping MIT/Apache code
  and with GPL-3 (MPL §3.3), but it is outside our MIT/Apache/BSD/zlib rule, and nothing in
  superfargoth's lock file is MPL today. **Needs the owner's OK** (Open questions 1);
  `sf_ui_css` contains it so no other crate links MPL code.
- **Features that matter to us**: stylesheets are Bevy assets (`.css`, so they load through
  `sf_vfs`'s asset source); `Styled` on a root, inherited by the subtree; selectors via
  `selectors` (`#id` = `Name`, `.class` = `ClassList`, type selectors incl. the
  bevy_ui_widgets types, attribute selectors, `:hover :active :focus :focus-visible :checked`,
  `:has() :is() :not()`, nesting); `var()`; `calc()` with min/max/clamp; `@media`
  (color scheme, width, height, resolution, aspect ratio); **`@layer` with nesting and
  `@import url(...) layer(name)`** (layer order from `LayersHierarchy`); `@import` paths
  relative to the importing file (0.9); `@font-face`; transitions and `@keyframes`; nine-slice
  via `background-image` + `-bevy-image-mode: sliced(..)`; custom properties and parsers
  (its `sound_effects` example: sounds as CSS properties); `em`/`rem` (0.9); **hot reload** on
  `AssetEvent::Modified`, and a changed `Styled` handle restyles its subtree (live theme swap).
  Invalid properties are reported per rule; malformed CSS must not panic (stated goal).
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

### R3. Outside Bevy

Slint (license), NoesisGUI and Coherent Gameface (proprietary SDKs; a GPL conflict for the
binary), RmlUi (C++, second renderer), Blitz (beta, heavy): all out. Kept as models: XAML's
bindings + styles + control templates, Gameface's lesson that UI artists want web authoring.

### R4. What players and modders hated and loved

**OpenMW** (GPL: lessons only). Built-in screens are hardcoded and replaceable only whole
(`I.UI.registerWindow`; "Adding new windows is not supported yet"); drag-and-drop exists only
in hardcoded windows (#8953); no layer query (#8958). Lua UI is a declarative tree re-diffed
into MyGUI: 1,028 widgets updated per frame cost 9.8 ms (#9291). Lua runs a frame behind, so
world-anchored UI wobbles (#9233, #9368), text can't be measured synchronously (#9289),
widgets have no identity to focus (#9193). Bitmap fonts blur or pixelate off-size (#7577);
icons scale nearest-neighbour (#8221); minimum window sizes break at 1.5× (#9313); dialogue
can't be scaled separately (#8790). Controller support came late (#8800, #8872): hidden
emulated cursor, no on-screen keyboard, no key repeat in lists, unbindable GUI buttons, menus
forget state (#8772). Community answer: third-party UI toolkits on top of the raw API.
**Lesson: the toolkit (widgets, focus, navigation, tooltips, settings) is the framework's job,
and stock screens use the public API.**

**MWSE** (original engine): patch the vanilla element tree in place (`findChild`, register
before/after); powerful (UI Expansion improves many menus at once), but ids aren't unique, so
mods grab the wrong element. **SkyUI** won by PC usability (sortable columns, search,
categories) and its **Mod Configuration Menu**, used by hundreds of mods. **World of
Warcraft**: the default UI is open Lua/XML that addons hook (`hooksecurefunc`); taint limits
what UI code may *do*, not what it may draw. **Factorio**: mods add elements under fixed roots
or `relative` to a built-in window; styles are data prototypes; elements know their mod.
**Lessons: settings registry, dense lists, named extension slots, styles as data, capability
boundary on actions.**

### R5. Controller-first and readability lessons

- **Xbox Accessibility Guideline 101**: text ≥ 18 px at 1080p on PC, 26 px on TV; scale to
  200 % without losing content; icons scale with text; a sans-serif option; ≤ 80 chars per
  line, 1.5 line spacing.
- **Starfield**: console-first inventory panned on PC; StarUI (compact lists, stat columns,
  sorting) became the fix. **Baldur's Gate 3**: separate controller UI, multi-second switch
  between KBM and gamepad. Lesson: switching input devices is instant and changes glyphs and
  focus visuals only, never the layout.
- **Elden Ring** (PC Gamer, "Elden Ring's half-assed mouse and keyboard controls";
  Steam forum 1245620): main menu hard-bound to Esc and map to G; controller glyphs shown by
  default on PC; no mouse on the map. Stats explained nowhere in game (poise thresholds,
  hidden buildup; wikis fill the gap). Loved: item descriptions as lore, level-up screen
  previewing every derived stat before you commit, a minimal HUD. Criticized as too plain
  next to Dark Souls 3 and Bloodborne. **Lessons: every action rebindable including menu
  open; prompts follow the last-used device automatically; show derived numbers and before/
  after deltas, with a tooltip on every stat; an ornate theme is worth shipping.**
- **Unreal CommonUI**: see Canonical technique.

## Stack decision

1. **Bevy first**: bevy_ui, bevy_ui_widgets, bevy_input_focus + `AutoDirectionalNavigation`,
   bevy_picking (span-level text picking), bevy_text (parley), bevy_a11y, BSN (`.bsn` assets
   when 0.21 ships).
2. **Feathers: dev tools only** (its docs say so).
3. **Crates**: `bevy_flair` for styling (decision 3; MPL caveat, R2b), inside `sf_ui_css`.
   `fluent-bundle` for strings. `bevy_enhanced_input` for actions once it is on 0.20
   (fallback `leafwing-input-manager`). Evaluate `bevy_ui_anchor` in the slice.
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
local ui = require("sf.ui")
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

## Industry comparison

J9 (industry-audit.md), 2026-10-12, docs research only (`.sessions/j9rest.md`). R3–R5 above
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

1. **MPL-2.0 dependencies** (`cssparser`, `selectors`, `cssparser-color`, via bevy_flair):
   allow MPL-2.0 for unmodified third-party crates in superfargoth, contained in `sf_ui_css`?
   (Recommended: yes. Otherwise CSS styling needs our own parser, which would also be MPL
   unless written from scratch.)
2. Hot reload needs an `AssetWatcher` over `sf_vfs`'s loose-directory layers (dev builds);
   `sf_vfs` is read-only today. Small addition at M5.
3. If M5 starts before Bevy 0.21: stock screens start as `bsn!` templates in Rust (same scene
   model, no file loader) and move to `.bsn` files when they land.
4. `bevy_enhanced_input` vs `leafwing-input-manager`: pick whichever is on 0.20 at M5.
5. **Subtitles (J9, P2).** A `subtitles` settings group (on/off, size scale independent of text
   scale, background opacity, speaker name, max two lines, minimum display time) and a
   `Layer::Notification`-level caption widget fed by a `Caption { speaker, text, duration }`
   message from the dialogue/audio side. XAG 104 defaults.
6. **Safe area (J9, P2).** A `safe_area` margin (percent per edge, default 0 on PC, 5 % in
   the controller preset when fullscreen on a TV-size display; plus OS-reported insets where
   winit exposes them) applied as padding on the HUD and menu layer roots, adjustable with a
   calibration screen like console titles.
