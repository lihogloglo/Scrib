# OpenMW UI look map (research R5a, 2026-11-11)

Purpose: the map for building the classic (`mw_classic`) UI so it looks exactly like OpenMW (owner 2026-11-11: layouts AND
Morrowind art). Source: github.com/OpenMW/openmw `master` at commit `97722d48f98f14b323c3880a698e8ce81babf7b3` (read
2026-11-11; same pin as `openmw-settings-window-2026-11-11.md`). Paths below are relative to that repo. Nothing was run;
sizes come from the XML/C++ (texture pixel sizes are NOT in the repo: they live in the user's `Morrowind.bsa`, see section 11).

## 0. What the folder holds and the licence
`files/data/mygui/` (relevant): layouts `openmw_mainmenu`, `openmw_settings_window`, `openmw_loading_screen`,
`openmw_savegame_dialog`, `openmw_confirmation_dialog`, `openmw_messagebox`, `openmw_interactive_messagebox`
(+`_notransp`), `openmw_hud`; skins `openmw_windows.skin.xml`, `openmw_button.skin.xml`, `openmw_box.skin.xml`,
`openmw_list.skin.xml` (also holds the scrollbar layouts), `openmw_scroll.skin.xml`, `openmw_text.skin.xml`,
`openmw_progress.skin.xml`, `openmw_hud_box.skin.xml`, `openmw_hud_energybar.skin.xml`, `openmw_resources.xml`
(cursors, `TabControl*` layout resources), `openmw_layers.xml`, `core.skin`, `skins.xml`. `openmw_mainmenu.skin.xml` is empty.
Logic: `apps/openmw/mwgui/{mainmenu,messagebox,settingswindow,loadingscreen,textcolours}.cpp`,
`components/widgets/tags.cpp`.

Licence: repo `LICENSE` is GPL-3 (v3, 29 June 2007, verified at the pin). OpenMW's own files (layouts, skins, C++) may be
ported with a "file + commit" citation in the module doc and a `THIRD_PARTY_NOTICES.md` entry (CONTRIBUTING "Licence").
Not covered: (a) every `textures\*.dds` the skins name (`menu_*`, `tx_*`, `target`, `compass`) is Bethesda art from the
player's install; we read it from their BSA at bake time and never ship it. OpenMW's own `omw_menu_scroll_*.dds` (scroll
arrows and tracks) live in the repo's data folder (confirm that folder's licence before copying; if unclear, re-draw).
(b) Fonts: Morrowind's are bitmap `Magic Cards` / `Daedric`; OpenMW ships Mystic Cards (SIL OFL 1.1), already our font.
(c) String ids `#{Interface:*}` come from the ESM GMSTs, `#{OMWEngine:*}` from `files/data/l10n/OMWEngine/en.yaml` (GPL).

## 1. How MyGUI skins are built (so we can map them)
- **Coordinates are px at gui scale 1**; the whole UI scales by `[GUI] scaling factor` (and `dw/w` of the window vs. its
  logical size, `windowmanagerimp.cpp` ~218); fonts scale with it. Layouts are authored at their literal sizes.
- **8-piece frames are `Child` widgets, not a slice**: each edge is a `TileRect` skin (`TileH`/`TileV` true) over its own
  texture, each corner a `MainSkin` (single image). `MW_Box` (thin frame): 2 px edges, 2x2 corners, `menu_thin_border_*`.
  `MW_Button`: 4 px frame (`menu_button_frame_*`), 136x24 minimum, text rect `4 3 128 16`. `MW_Dialog`: 4 px thick frame
  (`DB_*` = `menu_thick_border_*`), `BlackBG` fill inset 4. `MW_Window`: a 4 px thick OUTER frame, an INNER thick frame
  starting at y=24 (under the caption), corners built from three pieces (`TB_TL_T` 10x4, `TB_TL` 4x4, `TB_TL_B` 4x10: a
  14 px arm), client area inset `8 28 .. -8 -8`, caption `MW_Caption` at `4 4 x 20` made of `HB_*` head-block pieces
  (2 px frame, `menu_head_block_middle` tiled fill) with the title in `SandText` centred; the drag area is an invisible
  button over the caption. `MW_Window_NoCaption` drops the caption and inner top frame; `MW_Window_Pinnable` adds the
  pin button (`menu_rightbuttonup_*` / `_down_*`, 19x19).
- **Fill**: `BlackBG` = the `transparent` texture coloured by `#{fontcolour=background}` and the runtime `menu transparency`
  setting (`windowmanagerimp.cpp` `setMenuTransparency`); `FullBlackBG` / `MW_DialogNoTransp` = opaque black. Window
  interiors are **translucent black**, not a texture.
- **States** in `SimpleText` are text colours only (`disabled/normal/highlighted/pushed` plus `_checked` variants), from
  `#{fontcolour=X}` = Morrowind.ini `[FontColor] color_X` (`components/widgets/tags.cpp`: key `FontColor_color_X`). Button
  frames do NOT change on hover/press, only the label: `normal` -> `normal_over` -> `normal_pressed`; checked buttons use
  `active*`; `disabled`. Our `VANILLA_COLORS` already carries these keys.
- Font: `Default` = Magic Cards (our Mystic Cards TTF). Headers `NormalText` (colour `header`), body `SandText` (`normal`),
  values `SandTextRight` (`positive`/`negative`), links `link*`, answers `answer*`.
- **Containers**: MyGUI `HBox`/`VBox` with `Spacing`/`Padding`, `UserString HStretch/VStretch`, `AutoSizedButton` (width =
  text + padding), `AutoSizedEditBox` (height = wrapped text). These map directly to flexbox.
- **Layers** (`openmw_layers.xml`): Scene, FadeToBlack, HitOverlay, HUD, JournalBooks (a `ScalingLayer` 600x520: book art
  scales as a unit), Windows, DragAndDrop, MainMenuBackground, MainMenu, Settings, LoadingScreen(Background), Debug,
  Console, Modal, Popup, Notification, Video, InputBlocker, Pointer. Our z-order: HUD < Windows < MainMenu < Settings <
  Loading < Modal < Popup < Notification.

## 2. Main menu (`openmw_mainmenu.layout` + `mainmenu.cpp`)
- The layout is only a full-screen widget (layer `MainMenu`) holding `VersionText` (bottom-right, 280x50, right-aligned,
  black text shadow; shown only when no game is running). **The buttons are created in C++** (`updateMenu`).
- Each button is an `ImageButton` with three textures `textures\menu_<id>.dds`, `_over`, `_pressed` for ids `return,
  newgame, savegame, loadgame, options, credits, exitgame` (our spec table lists the same set). Clicks carry the id.
- **Order and visibility**: `return` (game running only), `newgame`, `savegame` (running, past chargen, saving allowed),
  `loadgame` (if any save exists), `options`, `credits` (title only), `exitgame`.
- **Sizing**: scale = `requested.height / 64` (the art is assumed 64 px tall; HD art scales down to 64 logical); each
  button is `requested.width / scale` wide, centred inside the widest button; the image tile is cropped by `16*scale` and
  the **vertical step is `height/scale - 16` = 48 px**: neighbours overlap 16 px (the art has transparent padding). The button
  box width = widest button.
- **Placement**: at the title (no game): box centred horizontally, bottom-aligned with **24 px bottom padding**
  (`mHeight - boxH - 24`), over `textures\menu_morrowind.dds` drawn full-screen (`[GUI] stretch menu background` chooses
  stretch vs. aspect-fit on black) or `video\menu_background.bik` if the VFS has it. In game (pause/escape): the same column
  **centred on both axes** (`mHeight/2 - boxH/2`), no background drawn (the paused game shows behind). No window frame or
  title anywhere: image buttons only.
- Focus: title starts on `loadgame` if visible else `newgame`; in game on `return`. D-pad up/down moves; B or Start = return.
  New game and quit go through the confirmation dialog (`NewGameConfirmation`: "Do you want to start a new game and lose
  the current one?"; `QuitGameConfirmation`: "Quit the game?"). Save/Load opens the save dialog (section 5); Options opens
  the settings window.
- **The pause menu is this same screen in the running state** (L3 is a different button set and placement, not a new widget).

## 3. Options window (`openmw_settings_window.layout`)
Settings content: `openmw-settings-window-2026-11-11.md`. Look only:
- Root: `Window` skin `MW_Window` (outer + inner thick frames, caption), layer `Settings`, layout `400 485` with
  `MinSize 640 550` (effectively 640x550; position/size persists per window). Inside: a top-level `TabControl`
  (`skin="TabControl"`, at `8 8`, stretching, `ButtonAutoWidth`) and the **OK** `AutoSizedButton MW_Button` bottom-right
  (`320 420`, expands left).
- **Tab control** (`openmw_resources.xml` `TabControl`): tab headers are `MW_Button_RightPadding` buttons (4 px right pad)
  with a space either side of the label (`" Preferences "`), in a 24 px `HeaderPlace`, with left/right scroll arrows
  (`MW_ArrowLeft/Right` in tiny 15x14 `MW_Box`es) when tabs overflow; the checked tab uses the `active` text colour (blue).
  The body is a `MW_Box` thin frame, 4 px inset. `TabControlInner` (Video's inner tabs) has no frame and no arrows.
- Tab order: Prefs, Audio, Controls, Video (inner: Video, Detail Level, Water, Lights, Shadows), Scripts, Language.
- Rows: a `NormalText` header above a **`ScrollBar` `MW_HScroll`** (14 px high, `Range 10000`, `Page 300`, `MoveToClick`)
  with `SandText` min/max captions below, left and right. Booleans are **`AutoSizedButton MW_Button` toggles
  (`SettingType CheckButton`) whose caption is `Off`/`On`**, label in a `SandText` beside (checkboxes are text buttons).
  Dropdowns are `ComboBox` `MW_ComboBox`.
- **Scrollbar/slider** (`openmw_list.skin.xml`): `MW_HScroll` 90x14 / `MW_VScroll` 14x90: a `MW_Box` thin frame as the
  background, two 15x14 end boxes holding 10x10 arrow buttons (`omw_menu_scroll_{left,right,up,down}.dds`) and a track
  thumb (`omw_menu_scroll_center_{h,v}.dds`, tiled, min size 14). The same widget is the slider.

### 3b. Controls (keybinds) tab
- Header row: two toggle buttons `Mouse/Keyboard` and `Controller` (`HBox`, spacing 4) switching `mKeyboardMode`.
- Body: a `MW_Box` thin frame (4 px inset) holding a `ScrollView` `MW_ScrollView` (`ControlsBox`) with its scrollbar.
- Rows (`settingswindow.cpp` ~940-975): per action two `SandTextButton`s in a shared-state group: left = action
  description (left aligned), right = binding name (right aligned, clickable). Hovering either highlights both
  (`normal_over`). Plain text rows, no frame per row; order from `getActionKeySorting()`.
- Click a binding: its caption becomes `None`, a static message box shows **"Press a key or button to rebind this control."**
  (`RebindAction`), the mouse is disallowed and the next key/button binds. Below: `Reset Controls` (`MW_Button`, bottom-left;
  asks `ConfirmResetBindings`: "Reset all controls to the default?") and On/Off toggles (`Invert x axis`, ...).

## 4. Loading screen (`openmw_loading_screen.layout`, `loadingscreen.cpp`)
- Full screen: a `Scene` background (last frame) and a `Splash` `BackgroundImage` (random file from `splash/`, i.e. the
  BSA's `Splash\*`) on the `LoadingScreenBackground` layer, stretched or aspect-fit.
- A `Window` `MW_Dialog` (`LoadingBox`, 300x48 design) with a `SandText` label (`LoadingText`, centred) over a `ScrollBar`
  `MW_ProgressScroll_Loading` (260x6 bar, `menu_bar_gray` tinted). Box **width fits the label**; bottom-centre, 8 px above
  the bottom edge (`height - boxH - 8`); an "important" label centres it on screen.

## 5. Save / load dialog (`openmw_savegame_dialog.layout`)
`VBox` skin `MW_DialogNoTransp` (opaque thick frame), layer Modal, 600x400, padding 8, spacing 8. Top: `ComboBox`
`MW_ComboBox` (24 high, "Select Character"). Middle `HBox`: left a stretching `ListBox` `MW_List` (one line per save,
`SandTextButton`-style rows) with scrollbar; right a 263-wide column: `MW_Box` 263x137 holding the screenshot (259x133),
cell name (`NormalText`, centred), info text (`SandText`: date, level, playtime). Bottom `HBox` spacing 4: `Delete`, an
edit box `MW_TextEdit` (save name), `OK`, `Cancel` (all `AutoSizedButton MW_Button`). Load mode hides the name edit.

## 6. Message boxes
- **Notification** (`openmw_messagebox.layout`): `VBox` skin `HUD_Box` (translucent black + thin frame), layer Notification,
  padding 10, auto-resize, text 300 wide centred. Stacks upward: 48 px above the bottom, 4 px between boxes; fades after a few s.
- **Confirmation Yes/No** (`openmw_confirmation_dialog.layout`): `Window` `MW_DialogNoTransp`, layer Modal, 360x130, centred;
  message 338 wide, centred, wrapped; a bottom `HBox` spacing 8 with two `AutoSizedButton MW_Button` (`Yes`, `No`),
  centred 89 px from the top.
- **Interactive** (`openmw_interactive_messagebox*.layout`, `messagebox.cpp` ~221): `Window` `MW_Dialog`, C++ resizes it:
  text 400 wide, paddings 10 (text/box, text/buttons, between buttons, buttons/bottom), button width = text + 2*12,
  height = text + 2*4; buttons in a row if they fit the text width else a column; centred on screen.

## 7. HUD (`openmw_hud.layout`; units px, design size 300x200, everything anchored to the screen's bottom)
- Three bars 65x12 at x=12, 15 px pitch: Health (red) y 147, Magicka (blue) 162, Fatigue (green) 177 (y is from the 200
  design top, so they sit 11-41 px above the bottom). Skins `MW_EnergyBar_*` (frames `menu_small_energy_bar_{top,bottom,vert}`,
  track `menu_bar_gray` tinted `fontcolour health/magic/fatigue`). Enemy health bar (yellow) 15 px above, only with a target.
  Hover tooltip layout `HealthToolTip`.
- Equipped weapon box (36x36 `HUD_Box`, 32x32 icon) at x=81 with a 36x6 condition bar below; spell box x=121 (+ charge bar);
  sneak indicator x=161 (36x36); spell-effect box right of it.
- Mini-map: 65x65 `HUD_Box` bottom-right, 61x61 map with a rotating `compass.dds` 32x32.
- Centre crosshair `textures\target.dds` 27x27. Timed `SandText` popups: weapon/spell name (left) and cell name
  (right-aligned) above the bar cluster. Drowning bar: `MW_Dialog` 230x58, top-centre.
`HUD_Box` = `BlackBG` fill + `MW_Box` thin frame.

## 8. MyGUI concept -> bevy_ui / bevy_flair
| MyGUI | Ours |
|---|---|
| 8-piece frame (4 `TileRect` edges + 4 `MainSkin` corners as children) | One packed image + `ImageNode` with `TextureSlicer` (`border` = piece thickness, edges `SliceScaleMode::Tile`) emitted by the `ui.frames` bake as `-bevy-image-mode: sliced(..)` (spec "Menu art for mw_classic"); fallback 9 nodes. Pieces are 2/4 px: pad the atlas, integer scale stays crisp |
| `MW_Window` outer + inner frame + caption | a `.mw-window` component. The 14 px composite corners make rebuilding pieces painful: pack the whole window skeleton (outer ring + inner ring under the caption) as ONE sliced image (insets ~18 / 28) |
| `BlackBG` + menu transparency | `background-color: rgb(0 0 0 / var(--mw-menu-alpha))`, var driven by the Prefs slider |
| `MW_Button` (fixed frame, label colour per state) | `.mw-button` with the `menu_button_frame` sliced image (4 px), `:hover`, `:active`, `:focus-visible` (over colour), `.checked` (`color_active*`), `.disabled` |
| `ImageButton` (3 textures) | `ImageNode`/`background-image` swapped by `:hover` / `:active` / `:focus-visible` rules; 64 logical px height; step 48 |
| `TabControl` | row of `.mw-tab` buttons (button skin + `.checked`), body `.mw-box`; overflow arrows optional |
| `ScrollBar` / slider | `.mw-scroll`: thin-frame track, two 15x14 arrow boxes, tiled thumb; one skin for both |
| `ListBox`, `ScrollView` | label rows with per-state colours, `overflow: scroll` + `.mw-scroll` |
| `HBox/VBox/AutoSized*` | flexbox, `gap`, `padding` |
| `#{fontcolour=x}` | `--mw-x` custom properties (already generated from the ini) |
| Layers | `GlobalZIndex` per screen root |
| `ScalingLayer` (books) | one `UiScale` on the book subtree |

## 9. Kit gaps (see `sf_ui_screens`, `mw_boot_bevy/src/ui_theme.rs`)
1. **Frames bake** (`ui.frames`) not built; the theme draws bronze CSS borders. Verify `bevy_flair` 0.9 handles `sliced` with per-edge Tile (spec says yes).
2. **Framed window component**: no `.mw-window` (caption block, thick frame, drag, pin), `.mw-dialog`, `.mw-box`.
3. **Image buttons** with `_over/_pressed` art. Our main/pause menus are text buttons with a stretched background. OpenMW's menu has no window and no title; we need a "no-chrome image entries" mode, 48 px overlap step, bottom-24 vs centred placement, title splash, version text.
4. **Tab strip** skin, **scrollbar/slider** art (`.mw-scroll`), combo box, text edit, On/Off text-toggle look, list row states.
5. **Keybind rows** as shared-hover two-column text rows (our `sf-keybinds-row` exists; add hover pair, "None", rebind notice).
6. **HUD**: bar frames (`menu_small_energy_bar_*` + tinted `menu_bar_gray`), `HUD_Box`, bottom-up notification stack.
7. **Message boxes**: auto-fit sizes (400 text width, 10 px paddings).

## 10. Proposed slices (one deliverable each; tests are headless structure/count only)
- **L0 (tiny prereq)**: list `Morrowind.bsa` `menu_*` DDS sizes, verify `bevy_flair` sliced Tile, record in the spec. Test: the bake writes a dims table that a unit test reads.
- **L1 Frames bake + framed window**: `ui.frames` packer for `thick_border`, `thin_border`, `button_frame`, `head_block`, `menu_rightbutton*`; emits `frames.css`; `.mw-window/.mw-dialog/.mw-box/.mw-button` classes on the existing `panel`/`button`. Test: expected image count and insets; a spawned `panel` carries an `ImageNode` with slicer insets 4 / 2.
- **L2 Image buttons + main menu as OpenMW**: image-entry mode, 7 ids, `_over/_pressed`, 48 px step, bottom-24 placement over `menu_morrowind`, version text. Test: entry order and visibility by game state, spacing = 48 logical px, focus start rule.
- **L3 Pause menu**: same widget in the running state (return, newgame, savegame, loadgame, options, exitgame), centred, no background. Test: set and centring in a headless layout pass.
- **L4 Options window + tabs**: `.mw-window` 640x550, tab strip, `.mw-scroll` slider, text-button toggles (Prefs + Audio tabs first, data-driven). Test: tab count and order, one control per settings record.
- **L5 Keybinds tab**: Mouse/Keyboard + Controller toggles, framed scroll list of two-column rows, hover pair, "None" + rebind notice, Reset Controls + confirm. Test: row count = actions; rebind state machine.
- **L6 Save/load + message boxes**: `MW_DialogNoTransp` save dialog and Yes/No (360x130), interactive, notification boxes. Test: widget presence, button order, sizes from the C++ constants.
- **L7 Scroll/list art + loading screen + HUD bars**: `omw_menu_scroll_*` art, list row states, loading dialog, HUD bars/boxes. Test: structure only.
- Sign-off: one `visual-checks/player-ui.md` entry per slice, "awaiting owner" (no agent screenshots).

## 11. Not verified
- ~~Pixel sizes of every `menu_*.dds`~~ L0 done (coordinator, 2026-11-11, `bevyrrowind ls` + `extract`, DDS headers):
  all uncompressed. Corners: thick and button 4x4, thin and head block 2x2. Edges: thick 512x4 / 4x512, thin 512x2 /
  2x512, button 128x4 / 4x16, head block 256x2 / 2x16 + `middle` 256x16. Main menu buttons (7 ids, `_over`,
  `_pressed`) 128x64; `menu_morrowind` 1024x512; `tx_menubook` 512x256; scroll pieces 8-32 px, bars 512x2 / 2x512.
  `bevy_flair` 0.9 parses `sliced(t r b l <center> <sides> <max_corner_scale>)` with `tile(..)` sides (verified).
- Skins `MW_ComboBox`, `MW_TextEdit`, `MW_List`, `MW_ProgressScroll_Loading`, `MW_EnergyBar_*` (in `openmw_edit.skin.xml`, rest of `openmw_list.skin.xml`, `openmw_hud_energybar.skin.xml`) were only partly read; shapes above follow their uses.
- Licence of `omw_menu_scroll_*` textures; `[FontColor]` defaults beyond our `VANILLA_COLORS`.
