# Game UI: history

Moved out of [ui.md](../ui.md) on 2026-10-22 (doc diet, ROADMAP 0.4): the 2026-10-11 research
findings, the as-built logs of U1a-U2 and the closed open questions, word for word. ui.md and the
`sf_ui*` code win where they differ. Headings are unchanged, so older references (DECISIONS.md,
code comments) such as `ui.md "As built (U1b)"` or `ui.md R1` resolve here.
Second move on 2026-10-25 (docdiet1), also word for word, from "Research findings (ui.md text,
2026-10-25)" on: ui.md's R2b, the Luau surface sketch, the dialogue window view-model, the U5 and
U6 as-built logs and the industry comparison. ui.md keeps a condensed copy of each.

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

## As built (U1a, 2026-10-18)

`superfargoth/crates/sf_ui`, Bevy-free (deps `fluent-bundle` 0.16, `unic-langid` 0.9, `sf_records`, `toml`).
14 headless tests, clippy pedantic clean. Changes against the Interfaces sketch:

- **Records**: kinds `ui.screen`, `ui.slot` (one contribution each), `ui.column`, `ui.theme`, `ui.preset`,
  `ui.setting`, registered as serde kinds (`register_kinds`), so mods write them in native RON/TOML record files and
  replace or patch them like any record. The record id is the object id; the **owner** is the mod whose define the
  value is based on (sf_mods names each record source by mod id). Built-in presets `pc_dense` (18 px, classic windows)
  and `controller` (26 px) exist without records; a record with the same id replaces them.
- **`UiRegistry::build(&RecordDb)`**: no `LoadOrder` argument, since record sources already are mods in load order.
  Findings (`UiConflict`, with `Severity` and owning mod): screen replaced (info), contribution to a slot no screen
  offers and unknown preset (warning), invalid setting default/range (error, setting dropped). Screens list the slots
  they offer (`ScreenDef::slots`).
- **Slot and column order: priority, then load order** (the sketch said load order, then priority, which makes
  priority useless across mods; WoW/Factorio-style explicit ordering wins, load order breaks ties).
- **`ScreenStack`** (the CommonUI router rule, Bevy-free): stacks per layer, reopen moves to top, `replace_top`,
  `input_target()` = top screen of the highest input-taking layer (Hud, Menu, Modal; notifications and tooltips never
  take input), `pauses_game()`, a version counter for the Bevy side. A screen's input context defaults to its id.
- **`ViewModel` is not `Reflect`**: `KIND`, `fields()`, `field(name) -> UiValue`. Bevy-free (§8b) and exactly what
  bindings, columns (`ValueSource::Field`) and sorting need; `sf_ui_bevy` may still derive `Reflect`.
- **`ListModel<K, T>`**: rows in insertion order with per-row versions (`upsert` of an equal value is a no-op),
  `changed_since(version)`, stable sort on a field (case-insensitive text, mixed int/float), filter (search over
  fields + exact field matches for tabs), lazily rebuilt view, `window(first, count)` for virtualization.
- **`SettingsStore`**: typed values over `ui.setting`; numbers clamp and snap to the step; profile I/O over
  `BTreeMap<mod id, toml::Table>` (`sf_mods::Profile::settings`), saving only non-defaults and passing unknown entries
  through (a disabled mod keeps its values). Profile key = setting id minus a leading `<owner>.`.
- **`Localizer`**: one `fluent_bundle::concurrent::FluentBundle` per locale of the chain; mod files in load order with
  `add_resource_overriding` (a later mod rewords another's message); `format("id" | "id.attr", args)` falls back
  through the chain, then to the key; syntax errors become `L10nIssue`s per mod and the valid messages still load.
  VFS convention `l10n/<locale>/<mod id>.ftl` (files never shadow each other). `negotiate()` is our own small filter
  (exact, then same language, then default): `fluent-langneg` 0.14 moved to `icu_locid` while `fluent-bundle` 0.16
  uses `unic-langid`.
- Not in U1a: `sf_ui_bevy` (U1b: input contexts, focus, glyphs, example), CSS (U2), widgets (U3), Lua (parked).

## As built (U1b, 2026-10-18)

`superfargoth/crates/sf_ui_bevy`, a thin plugin (`SfUiPlugin`) on Bevy sub-crates plus `bevy_enhanced_input` 0.27
(`default-features = false`: no `bevy_state`). 6 headless tests (`cargo test -p sf_ui_bevy`), clippy pedantic clean;
example `ui_nav` (`--autotest [png]`: synthetic gamepad, 40 checked steps). Changes against the Interfaces sketch:

- **Resources**: `UiRegistryRes`, `ScreenStackRes` (push/pop on it; no `UiCommands` param yet), `UiSettingsStore`,
  `UiLocalizer`, `UiSettings` (the sketch's fields; nothing consumes them before U2/U3), `InputMode(Device)`.
- **Router = one BEI context entity per input context id** (`UiScreenInput { id }`), spawned the first time a screen
  using it becomes the input target, at `UI_CONTEXT_PRIORITY` (2^16, above gameplay). On a `ScreenStack::version()`
  change only the target's entity is `ContextActivity::ACTIVE`. Every UI action has `consume_input` and
  `require_reset`, so gameplay contexts lose exactly the inputs the UI binds, and a context that gains input while its
  button is still held doesn't fire (Start opening the menu doesn't also close it; B closing a modal doesn't also close
  the menu). A game adds screen-specific actions to that entity (`UiInputContexts::get(id)`). **HUD-layer contexts
  carry only `OpenMenu`/`OpenMap`**, so the HUD never swallows the d-pad or A in play.
- **Ordering matters**: the router and rebinding run in `PreUpdate` after `InputSystems` and **before
  `EnhancedInputSystems::Prepare`** (BEI collects the held inputs that `require_reset` keeps consuming there; running
  between Prepare and Update let a just-closed modal's B reach the menu). Dispatch and navigation run in `Update`.
- **Actions** (`UiAction`): `NavUp/Down/Left/Right` (bool) + `NavStick` (Vec2, `DeadZone`), `Confirm`, `Back`,
  `TabPrev/Next`, `Context1-4`, `OpenMenu`, `OpenMap`. Bindings live in the `UiBindings` resource (KBM + pad defaults:
  A/Enter, B/Esc/Backspace, LB/RB = Q/E, X Y LS RS = F R X C, Start/Esc, View/M); editing it respawns the actions of every
  UI context, so all are rebindable including menu open. Started actions go out as `UiActionEvent { action, screen }`
  messages for the target screen. Confirm triggers `bevy_ui_widgets::Activate` on the focused entity (the headless
  widgets' contract, so mouse, keys and pad converge), once per entity per frame (the widgets' own Enter handling
  isn't doubled). Back closes a Menu/Modal target unless its root has `UiHandlesBack`.
- **Navigation**: Bevy's `AutoDirectionalNavigator` (manual `DirectionalNavigationMap` edges first, then screen
  position). Bevy's auto navigation is layer-agnostic (its docs' workaround 3), so the router moves the
  `AutoDirectionalNavigation` marker onto the `UiFocusable`s under the target's `UiScreenRoot` only. Focus moves into
  the target when it is elsewhere: remembered focus of that screen (a closed modal returns focus to the button that
  opened it), else `UiDefaultFocus`, else the first focusable. `NavRepeat`: four-way, fires at once, then after 0.4 s,
  then every 0.08 s (d-pad, arrows and stick past 0.5 alike).
- **Device**: `InputMode` switches on new use within the frame: key/mouse-button press, > 4 px mouse travel or wheel
  vs a newly pressed pad button or a stick crossing 0.5 (edges on the `Gamepad` components, so a held button never
  flips it back; pad wins a tie). `glyph(&UiBindings, action, device)` labels the first bound input of that device
  (Xbox names, key names); icon atlases per pad family wait for the widget kit (U3).
- Leftovers (logged in ROADMAP U1): Fluent sources from each mod's VFS `l10n/<locale>/<mod>.ftl` and `SettingsStore` ↔
  `Profile::settings` glue (u1c); rebinding persisted to the profile; `UiCommands` system param; `ScreenDef::presets`
  enforcement on push; PlayStation/Switch glyph names; focus visuals by `InputMode` (U2 `input` attribute).

## As built (U1c, 2026-10-18)

Glue between `sf_ui`/`sf_ui_bevy` and the mod pipeline (`sf_mods`, `sf_vfs`); `sf_ui` now depends on `sf_mods`
(both Bevy-free).

- **Fluent from mods** (`sf_ui::l10n_sources`, `Localizer::from_profile`): every VFS path
  `l10n/<locale>/<name>.ftl` is attributed to the mod whose layer holds it (`LoadedProfile` layers), sources come out
  in load order, and the chain is negotiated over the locales mods actually ship. A file named after another mod is
  reported against the mod that ships it and ignored (a rewording mod writes the message ids it changes into its own
  `<its id>.ftl`; files never shadow each other), as are bad locale folders, unreadable and non-UTF-8 files.
- **Record kinds in the pipeline**: `sf_ui::UiRecordKinds`, a `ContentAdapter` that only registers the UI kinds; apps
  pass it beside `NativeRecords` and the content pack's adapters.
- **Settings ↔ profile**: `SettingsStore::load_from(&Profile)` / `save_to(&mut Profile) -> changed` over
  `Profile::settings` (`ModId` keys; non-defaults plus passed-through entries, as in U1a).
- **Bindings in the profile**: new `sf_mods::Profile::bindings` (`[bindings.<context>]`, action name → list of input
  names), game-agnostic so a game's gameplay context can use it too. UI actions use context `ui` and store only
  actions that differ from `UiBindings::default()`. Input names are our own small text form (`key:Enter`,
  `key:Ctrl+Shift+KeyS`, `mouse:Left`, `pad:South`, `axis:LeftStickX`, `mouse_motion`, `mouse_wheel`, `any_key`),
  with Bevy's variant names read through `bevy_reflect` (unit variants only). Why not serde: BEI's `serialize`
  feature turns on `bevy/serialize` (feature churn across the workspace) and writes nested tables with bitflags,
  not something a player edits; reflection is already compiled in.
- **Bevy side** (`sf_ui_bevy::profile`): `insert_from_profile(world, &LoadedProfile, &UiLocales, dir)` builds and
  inserts `UiRegistryRes`, `UiSettingsStore`, `UiLocalizer`, `UiBindings` and `UiProfile`, returning a `UiLoadReport`
  (registry conflicts, string issues, setting errors, binding issues). `sync_profile` (`Last`) copies changed
  settings and bindings into `UiProfile` and writes the file when no Menu or Modal screen takes input, or on
  `AppExit` (save on leaving the options menu, never per slider step); `UiProfile::last_error` keeps a failed save.
- **`UiCommands`** system param (push, replace_top, pop, close) and **presets enforced on open**:
  `ScreenStack::push_in` / `replace_top_in` refuse a screen whose `presets` list doesn't name the active preset
  (`StackError::UnsupportedPreset`); `UiCommands` passes `UiSettings::preset`. Plain `push` stays unchecked.
- Tests: `sf_ui` 3 new (two mods × two locales with a rewording override, a misnamed file and a syntax error; settings
  through a saved profile file; presets), `sf_ui_bevy` 4 new (input names, bindings table, settings + bindings across
  a restart through the profile file, `UiCommands` presets).
- Not done (logged): the game app doesn't call `insert_from_profile` yet (Bevyrrowind loads mods outside the UI so
  far); `UiSettings` (live presentation) isn't fed from `ui.setting` values; PlayStation/Switch glyph names; focus
  visuals by `InputMode` (U2).

## As built (U2, 2026-10-18)

`superfargoth/crates/sf_ui_css` on `bevy_flair` 0.9.0 (crates.io, Bevy 0.20). 6 tests (`cargo test -p sf_ui_css`),
clippy pedantic clean. The only crate linking flair; flair types stay inside it (`UiClass` wraps its class list).

- **Licence set** (`cargo tree -p sf_ui_css`, 2026-10-18): bevy_flair, bevy_flair_core/_style/_css_parser
  `MIT OR Apache-2.0`; **MPL-2.0, unmodified**: `cssparser` 0.37, `cssparser-color` 0.5, `cssparser-macros` 0.7.1,
  `selectors` 0.38, `dtoa-short` 0.3.5 (Open question 1, owner OK 2026-10-11); the rest MIT/Apache, Zlib, BSD,
  CC0, MIT-0, Unlicense, Unicode-3.0 (icu_*). No GPL/LGPL. New in the lock: ariadne, phf*, servo_arc, yansi.
- **Composer** (`Composition` → `ComposedSheet`, Bevy-free text): `@layer sf, preset, theme, mods-<n>-<id>..., user;`
  then one `@import url("sf_ui_content://<vfs path>") layer(..)` per sheet; a mod's sheets share its layer.
  **Layer names are flat `mods-<load position>-<id>`, not `mods.<id>`**: flair 0.9 parses one identifier in
  `layer(..)` and `@layer a, b` (dotted names only via nested `@layer mods { }` blocks, and `@import` is top-level
  only); mod ids may contain `.`. Ordering is the same as sub-layers.
- **Sources**: the root sheet lives in an in-memory source `sf_ui`; imports resolve against the importing sheet's
  source and a path can't name Bevy's default source, so content is imported from a **named source
  `sf_ui_content`** that the app registers with `UiCssSourcePlugin::new(builder)` before `DefaultPlugins` (same
  builder kind as its default source: `sf_vfs::bevy::shared_asset_source` in the game, `platform_default` in
  examples). `UiCssPlugin` (after `SfUiPlugin`) adds `FlairPlugin`.
- **Inputs**: `UiCssSheets { framework, mods, user }` (`mods_from_profile(&LoadedProfile)` = each mod's
  `[ui] styles` in load order); preset and theme sheets (+ the `high_contrast` variant) from `UiSettings` and the
  registry. A change recomposes, rewrites the root and `AssetServer::reload`s it; flair restyles (live swaps).
- **Roots**: every `UiScreenRoot` gets `Styled` = the root sheet, classes (`<preset root_class>`, `theme-<id>`,
  `high-contrast`, `reduce-motion`) and attribute `input=kbm|gamepad` from `InputMode` (focus visuals per device are
  CSS). sf_ui_css owns flair's `ClassList`/`AttributeList` on styled nodes (`UiClass` for others).
- **Error attribution**: flair only logs errors (or fails the whole sheet with `ReturnError`), so each source sheet is
  also loaded alone with `ReturnError` (a "probe"); `AssetLoadFailedEvent<StyleSheet>` paths map back to the owner →
  `UiCssReport { issues: [CssIssue { kind, mod_id, path, message }], version }` + a `warn!`; a clean reload clears it.
  The root imports leniently, so a broken mod keeps its valid rules. Cost: each sheet parses twice at load.
- Tests: composer (layer order = load order, owners, identifier names); headless flair app (a later mod beats a more
  specific earlier rule, reordering mods restyles live; an invalid value is reported against `broken` while its valid
  rule applies; `input` attribute and preset class select rules).
- **Verified (part 2, u2b)**: example `ui_css` (three mod sheets, HUD shows the report). `--autotest [dir]` on a
  temp copy: buttons styled by the later mod (#2e7d32), exactly one issue attributed to `broken_mod`, then the mod's
  sheet is rewritten on disk and the new colour applies after 0.33 s (`ui_css_before.png` / `ui_css_after.png`).
  **Hot reload needs no extra code**: the `sf_ui_content` watcher marks the edited file changed, Bevy reloads every
  asset that has it as a loader dependency (the root sheet imports it) and the probe, and flair restyles; the
  `server.reload(root)` fallback planned for this case wasn't needed. `ui_nav --autotest` still PASS; clippy
  pedantic clean. Visual check UI-2 (bevyrrowind `docs/visual-checks.md`) awaiting owner.
- Leftovers: flair logs the same sheet error twice (root import + probe); the examples log Bevy's "no default
  assets dir" watcher warning (only `sf_ui_content` is used).

## Closed open questions

1. **MPL-2.0 dependencies** (`cssparser`, `selectors`, `cssparser-color`, via bevy_flair):
   allow MPL-2.0 for unmodified third-party crates in superfargoth, contained in `sf_ui_css`?
   (Recommended: yes. Otherwise CSS styling needs our own parser, which would also be MPL
   unless written from scratch.)

4. ~~`bevy_enhanced_input` vs `leafwing-input-manager`~~ **Closed 2026-10-18 (u1a): `bevy_enhanced_input`
   0.27** (MIT/Apache; on Bevy 0.20 since 2026-10-14; github.com/simgine/bevy_enhanced_input, last push
   2026-10-14). Why: its model *is* CommonUI's (both follow UE Enhanced Input): input contexts are
   components evaluated newest-first or by `ContextPriority`, and actions consume their inputs by default
   (`ActionSettings`), so the top screen's context shadows the layers below it. That is the router rule
   above, with no clash code of ours. Actions and bindings are entities (BSN-declarable), queryable as
   `Action<A>` without observers; conditions and modifiers (hold, tap, chord, dead zones) are built in;
   `GamepadDevice` restricts a context to one pad; its `keybinding_menu` example rebinds at runtime and
   persists through Bevy 0.20's `SettingsGroup` (`serialize` feature). `leafwing-input-manager` 0.22
   (Bevy 0.20 since 2026-10-15, larger user base) keeps one `InputMap`/`ActionState` per entity with a
   `ClashStrategy`: layered contexts and input consumption would be ours to build. Neither ships device
   glyphs or last-used-device detection: both stay ours (Stack decision 4). Its Bevy dependency is the
   `bevy` umbrella with only `mouse`, `keyboard`, `gamepad`, `touch`, `bevy_scene`, `bevy_log`; it lives in
   `sf_ui_bevy`, never in `sf_ui`. Bindings persist in the profile through our settings registry (U6), so
   profiles stay the single source; U6 decides whether to bridge or bypass `SettingsGroup`.

2. Hot reload needs an `AssetWatcher` over `sf_vfs`'s loose-directory layers (dev builds);
   `sf_vfs` is read-only today. Small addition at M5.

## Research findings (ui.md text, 2026-10-25)

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

## Luau surface (sketch from Interfaces)

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



## Open questions 1, 2, 4 (ui.md text, 2026-10-25)

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
