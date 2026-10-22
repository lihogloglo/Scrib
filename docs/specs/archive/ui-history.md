# Game UI: history

Moved out of [ui.md](../ui.md) on 2026-10-22 (doc diet, ROADMAP 0.4): the 2026-10-11 research
findings, the as-built logs of U1a-U2 and the closed open questions, word for word. ui.md and the
`sf_ui*` code win where they differ. Headings are unchanged, so older references (DECISIONS.md,
code comments) such as `ui.md "As built (U1b)"` or `ui.md R1` resolve here.

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
