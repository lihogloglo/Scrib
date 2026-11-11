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
| Settings persistence | our `sf_mods` profile (`[settings.<owner>]`), plus a machine file (below) | Bevy 0.20's optional `bevy-settings` crate is evaluated in S1 for the machine file: adopt it if its format and path fit, else our TOML |
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

### 4. Settings v2

`SettingDef` (`sf_ui/src/records.rs:224`) gains, all optional with today's behaviour as default:

| Field | Values | Default |
|---|---|---|
| `apply` | `Live` (consumer applies on change), `Confirm` (staged; Apply/Revert bar; with `keep_seconds` a keep-or-revert countdown), `Restart` (stored; badge "applies after restart") | `Live` |
| `scope` | `Profile` (`[settings.<owner>]` in the profile), `Machine` (graphics/display, `user/machine.toml` or `bevy-settings`), `Save` (stored in the save; later) | `Profile` |
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
  level (derived each change, never stored as truth). Mods add keys to a level in load order.
- **Change contract**: the store emits `SettingChanged { key, old, new }` (a Bevy message in `sf_ui_bevy`);
  consumers are ordinary systems, one per setting family, registered by the owning crate. `Restart` settings
  are read once from the store at app build (before plugins are added), which is how the game's start-time
  switches become settings without making each one live on day one.
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

### 6. Framework settings shipped (built-in, owner `sf`)

Display (`Machine`): window mode (windowed / borderless / exclusive; `Confirm`, 15 s), monitor
(`options_from`), resolution (`options_from`, windowed/exclusive), vsync, frame cap, UI scale. Audio: as built.
Interface: theme, layout preset, text scale, reduce motion, high contrast (from ui.md). The game adds the
Graphics page (adapter) because what a feature is and costs is game-specific.

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
  `Live` is its own logged slice.
- **S5** Interface and accessibility pages wired to `UiSettings`.

## Owner answers (2026-11-11)

1. Video decoder: pure-Rust Bink first (video.md).
2. Intro logos: **first launch only** (`sf.intro_videos` = `first_launch` default, `always`, `off`), skippable.
3. Order: **front end first** (F1-F4, B1), then video (V1-V3, B2, I1), then options (S1-S5), then textured art.
