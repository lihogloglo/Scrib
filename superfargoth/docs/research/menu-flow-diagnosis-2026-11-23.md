# Menu flow diagnosis (2026-11-23)

Read-only diagnosis for the menu/options overhaul (ROADMAP 2.2b). Owner reports: PP-4
(`bevyrrowind/docs/visual-checks/rendering.md:332`, "earlier windows stay visible and overlap new ones, items move
on hover") and LK-1 (`bevyrrowind/docs/visual-checks/player-ui.md:205`, "opening a submenu does not hide the menu it
came from: both draw on top of each other"). Code only, no run.

## 1. Current flow as built

```
Frontend: main_menu -Options-> options -Key bindings-> keybinds;  main_menu -Load-> saveload
In game:  hud (Hud layer) -Esc-> pause -Options-> options -Key bindings-> keybinds;  pause -Save/Load-> saveload
          every arrow = a push on the SAME Menu layer; nothing below is hidden
```

- **Records**: `pause`, `options` (`mw_exterior_bevy/src/pause_menu.rs:64-79`), `main_menu` (`main_menu.rs:76`),
  `saveload` (`mw_saves_bevy/src/menu.rs:57`), `keybinds` (`mw_controls_bevy/src/lib.rs:68`) are all `layer = "Menu"`.
  A record has no "covers what is below" or "modal" field (`sf_ui/src/records.rs:92-112`).
- **Stack**: `push` appends, moving an open id to the top (`sf_ui/src/stack.rs:46-65`); `close` removes anywhere
  (`:114-123`); `input_target` = top of highest input layer (`:148-154`); `pauses_game` (`:157-162`).
- **Spawn/despawn is split**: the game spawns a `UiScreenRoot` entity and pushes its id
  (`pause_menu.rs:174-175`, `:235-236`, `mw_controls_bevy/src/lib.rs:442-443`, `mw_saves_bevy/src/menu.rs:325`);
  the framework screen fills it on `Added<...>` (`sf_ui_screens/src/options.rs:737`, `pause_menu.rs:315`). Screens
  close themselves by editing the stack (`options.rs:1350`, `:1286`; `pause_menu.rs:490`, `:532`), and four separate
  game systems despawn roots that are off the stack (`pause_menu.rs:296-305`, `main_menu.rs:516-522`,
  `mw_saves_bevy/src/menu.rs:418-423`, `mw_controls_bevy/src/lib.rs:450-454`).
- **What the stack drives**: only (a) the active `bevy_enhanced_input` context (`sf_ui_bevy/src/router.rs:79-128`),
  (b) `GlobalZIndex` per root (`router.rs:152-181`), (c) which focusables get directional navigation
  (`router.rs:195-278`), (d) Back closing the target unless it has `UiHandlesBack` (`router.rs:297-348`), and (e) each
  screen's own `is_target` check before it writes `InputFocus` (`sf_ui_screens/src/lib.rs:277`, `options.rs:1220`,
  `pause_menu.rs:451`). Visibility and pointer input are not driven by the stack.
- **At once**: every open root is spawned, drawn and pickable. After Esc > Options > Key bindings: pause, options,
  keybinds all drawn; only keybinds gets actions and focus; all three take hover and clicks where uncovered.
- **Only hiding that exists**: the game hides its HUD while a Menu/Modal screen is open
  (`mw_player_bevy/src/hud.rs:146-149`); the spec says outright "the framework doesn't hide lower layers"
  (`superfargoth/docs/specs/ui.md:352-353`).
- **Esc / Back**: Esc on the HUD is OpenMenu, opens pause if no Menu screen is open (`pause_menu.rs:156-180`). On a
  menu, Back goes to the target only; options, pause, main menu, keybinds, saveload all handle Back themselves
  (`UiHandlesBack`, `options.rs:459`, `pause_menu.rs:178`); options closes itself (`options.rs:1280-1286`), pause
  answers "no" or resumes (`pause_menu.rs:479-491`). This works as Common UI intends.

## 2. Root causes

**(a) Earlier windows stay visible and overlap.**
1. *Nothing hides or deactivates the screens under the top one.* The stack only sets z and input context
   (`router.rs:159-181`, `:79-128`); no system sets `Visibility`/`Display` on lower roots. All submenus are pushed on
   the same Menu layer, so pause or main menu stays drawn under options/saveload/keybinds. Sure.
2. *Windows are translucent.* Morrowind theme `.sf-window` = `rgba(0,0,0,0.78)` (`mw_boot_bevy/ui/morrowind/theme.css:15`,
   `:57-60`), fallback `ScreenColors::panel` alpha 0.88 (`sf_ui_screens/src/widgets.rs:93`). Where options covers the
   pause window, the pause text shows through at 22% (LK-1 "both draw on top of each other, unreadable"). Sure.
3. *Windows of different size share one centre.* Pause, options, saveload use `centred_window` (`widgets.rs:237-245`),
   content-sized; the smaller/larger one sticks out around the other. Main menu is full screen with entries at the
   bottom (`main_menu.rs:412-425` framework, `theme.css:109-112`), fully visible beside options. Sure.
4. *Keybinds is not centred.* The game spawns its root with `left: 15%, top: 8%` absolute
   (`mw_controls_bevy/src/lib.rs:436-442`), then `spawn_keybinds` inserts a whole new `Node` via `panel_as`
   (`sf_ui_screens/src/keybinds.rs:215-226`), replacing the game's Node: the window lands top-left (OP-1 "not
   centred"). Sure.

**(b) Items move on hover.**
1. *Options window resizes with its content and re-centres.* The window has no fixed size (`options.rs:745-756`), its
   centring is `translate(-50%,-50%)` of its own size (`widgets.rs:243`). Hovering a row changes the description pane
   (`options.rs:1364-1402`, `:1196-1218`): description text wraps in a 280 px pane (`options.rs:71`, `:825-838`) and
   the restart/reason/default lines toggle `Display::None` (`:1206-1215`). When the pane is taller than the rows
   (Graphics page: few rows, restart badge on some), the body and window grow or shrink, the window re-centres, and
   every row shifts vertically under the pointer. Likely (the exact trigger height was not measured).
2. *Same mechanism on value change and tab switch*: the value label has only `min_width: 80px` (`options.rs:935`),
   "Extremely Low" is wider, so a step widens the window and shifts it sideways; a tab switch resizes it and the tabs
   jump. Sure.
3. *Feedback loop*: a row moving out from under the pointer fires `PointerLeave`/`PointerEnter` on another row, which
   changes the pane again (flicker). Guess.
4. Ruled out: hover CSS changes colour only (`theme.css:30-46`); the focus ring is an `Outline` (`widgets.rs:465-495`);
   frames are background images (`ui_frames.rs:124-137`); main menu art is fixed 128x64 (`theme.css:128-137`).

**(c) Other defects found.**
1. *Lower screens take mouse input.* Activate observers do not check `is_target`: `on_options_part`
   (`options.rs:1304-1362`), `on_pause_menu_part` (`pause_menu.rs:497-539`), `on_main_menu_part`
   (`main_menu.rs:779-810`). With options open, clicking a visible pause entry (Resume, Main menu, Save) or a main menu
   entry (New game) acts on the screen underneath. Buttons get `Hovered` (`sf_ui_css/src/plugin.rs:94-96`), so lower
   entries also light up on hover (`theme.css:35`) and main menu art swaps (`main_menu.rs:562-563`). Sure.
2. *No modal scrim or input blocker* between a submenu and what is below. Sure.
3. *Options has no scroll*: rows column has no `Overflow` (grep: none in `options.rs`) and the window caps at
   `max_height: 100%` (`:755`); a long page clips. Saveload does scroll (`saveload.rs:333`). Likely on small windows.
4. *Lifecycle duplicated*: four game copies of one despawn system (above); a screen without one stays drawn after
   closing. Design gap, sure.
5. *Stale doc*: `options.rs:18` says "`ui.setting` has no condition yet"; records have `enabled_if`/`visible_if`
   (`records.rs:252-255`) but rows are always `enabled: true` (`options.rs:370`). Sure.
6. Fine: no per-frame rebuild (`options.rs:1062-1189`, `graphics_page.rs:247-274`); focus returns on close
   (`options.rs:1220-1227`, `pause_menu.rs:451-460`).

## 3. Gap list vs `menus.md` §4 and §5

| Spec item | State |
|---|---|
| §4 fields `apply/scope/enabled_if/visible_if/disabled_reason/order/keywords/reset/format` | In the record (`records.rs:232-270`) |
| Built and used | `apply = Restart` badge (`options.rs:373`), `scope = Machine`, `SettingChanged` (`sf_ui_bevy/src/lib.rs:240`) |
| Record only, unused by options | `Confirm`/`keep_seconds` (`records.rs:301-305`), `enabled_if`/`visible_if` (evaluator `sf_ui/src/conditions.rs`, no caller, no `SettingTraits` filled), `order`, `keywords`, `format` (own heuristic `options.rs:219-240`), `options_from`, `Action` rows (skipped, `options.rs:359`) |
| `ui.setting_preset` + Custom | Built in `sf_ui`; graphics page uses `sf_tunables` presets (`graphics_page.rs:23`) |
| §5 Slider, Checkbox | Built (S2a) |
| Choice stepper | Built (arrows); dropdown on click not built (click cycles) |
| Pages on `TabList` | Not built (plain buttons + `active` class, `options.rs:1083-1100`) |
| Bindings via a Controls page row | Not built: footer button opening a separate screen (`options.rs:794-800`) |
| Description pane right | Built; bottom-by-preset not; `format`ted values not |
| Footer action bar with glyphs, Reset page, ask-on-leave | Not built (Keybinds + Back only) |
| Keep-or-revert dialog; framework page order; pack page layout | Not built |
| One screen, two entry points | Built (`pause_menu.rs:224`, `main_menu.rs:438`); `phase` conditions not evaluated |
| Translucent over the paused world | Built, but with the pause window still drawn under it |

## 4. What a fix must change (shape)

- One stack-driven presentation rule in the framework: a screen record says whether it covers what is below
  (Common UI's activatable widget stack shows only the top widget); covered screens are hidden or collapsed, not
  just z-ordered.
- Submenus (options, saveload, keybinds) either replace the parent in one menu container or push with "cover", so
  the pause/main menu is not drawn behind them; Back restores it.
- Inactive screens are inert to the pointer too: pickability (or a full-screen blocker per pushed screen) follows the
  stack, and Activate handlers ignore non-target screens.
- The framework owns the screen lifecycle (spawn on push, despawn on close) in one system; game code only asks to
  open a screen with its model.
- Fixed menu geometry: windows get a size from the layout preset (or CSS), not from content; panes, value columns
  and tabs reserve space; the rows area scrolls.
- Roots keep the framework's placement: screens insert their own layout fields without replacing the game's `Node`
  (or the game never sets one).
- Options model reads the record fields it already has (`enabled_if`/`visible_if`, `order`, `format`, Action rows,
  `Confirm`) before new UI is added on top.
