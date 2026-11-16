# Bevyrrowind's UI strings (builtin mod `bevyrrowind`, locale en). The keybinds screen labels each
# action with the Fluent message `keybind-<context>-<action>` (sf_ui keymap convention); a
# translation ships `l10n/<locale>/bevyrrowind.ftl` with the same ids.
# Names follow Morrowind's own Controls menu where it has the control (Move Forward, Activate,
# Run, Sneak, Jump, Toggle POV), modern wording otherwise.

## Gameplay (controls.rs)
keybind-gameplay-move_forward = Move Forward
keybind-gameplay-move_back = Move Back
keybind-gameplay-move_left = Move Left
keybind-gameplay-move_right = Move Right
keybind-gameplay-move = Move
keybind-gameplay-look = Look
keybind-gameplay-zoom = Camera Zoom
keybind-gameplay-jump = Jump
keybind-gameplay-sprint = Dodge / Sprint
keybind-gameplay-sneak = Sneak
keybind-gameplay-use = Activate
keybind-gameplay-toggle_view = Toggle POV
keybind-gameplay-inventory = Inventory
keybind-gameplay-attack = Attack
keybind-gameplay-heavy_attack = Heavy Attack
keybind-gameplay-guard = Guard
keybind-gameplay-ready_weapon = Ready Weapon
keybind-gameplay-lock_on = Lock On
keybind-gameplay-quicksave = Quicksave
keybind-gameplay-quickload = Quickload

## Menus (sf_ui_bevy UI actions)
keybind-ui-nav_up = Menu Up
keybind-ui-nav_down = Menu Down
keybind-ui-nav_left = Menu Left
keybind-ui-nav_right = Menu Right
keybind-ui-nav_stick = Menu Navigate
keybind-ui-confirm = Select
keybind-ui-back = Back
keybind-ui-tab_prev = Previous Tab
keybind-ui-tab_next = Next Tab
keybind-ui-context_1 = Menu Action 1
keybind-ui-context_2 = Menu Action 2
keybind-ui-context_3 = Menu Action 3
keybind-ui-context_4 = Menu Action 4
keybind-ui-open_menu = Menu
keybind-ui-open_map = Map

## Options screen (sf_ui settings; pause_menu.rs). Pages `set-page-<page>`; the framework's audio
## page labels each bus `set-audio-<bus>-volume` / `-muted` (sf_ui `AUDIO_BUSES`).
set-page-audio = Audio
set-audio-master-volume = Master Volume
set-audio-master-muted = Mute All
set-audio-music-volume = Music Volume
set-audio-music-muted = Mute Music
set-audio-sfx-volume = Effects Volume
set-audio-sfx-muted = Mute Effects
set-audio-voice-volume = Voice Volume
set-audio-voice-muted = Mute Voice
set-audio-ambience-volume = Ambience Volume
set-audio-ambience-muted = Mute Ambience
set-audio-ui-volume = Menu Volume
set-audio-ui-muted = Mute Menus
set-audio-movie-volume = Movie Volume
set-audio-movie-muted = Mute Movies
## Descriptions (`<label>-desc`), shown in the options screen's description pane.
set-audio-master-volume-desc = The loudness of every sound in the game.
set-audio-master-muted-desc = Silences every sound in the game.
set-audio-music-volume-desc = The loudness of the music.
set-audio-music-muted-desc = Silences the music.
set-audio-sfx-volume-desc = The loudness of combat, spells, footsteps and other world sounds.
set-audio-sfx-muted-desc = Silences combat, spells, footsteps and other world sounds.
set-audio-voice-volume-desc = The loudness of spoken dialogue and greetings.
set-audio-voice-muted-desc = Silences spoken dialogue and greetings.
set-audio-ambience-volume-desc = The loudness of wind, water, crowds and other background sounds.
set-audio-ambience-muted-desc = Silences wind, water, crowds and other background sounds.
set-audio-ui-volume-desc = The loudness of menu clicks and interface sounds.
set-audio-ui-muted-desc = Silences menu clicks and interface sounds.
set-audio-movie-volume-desc = The loudness of the intro and other videos.
set-audio-movie-muted-desc = Silences the intro and other videos.

## The options screen's description pane (sf_ui_screens `ScreenLabels`; pause_menu.rs).
options-restart = Applies after restart
options-default = Default

## The interface page: the intro videos choice (sf_ui `INTRO_VIDEOS_KEY`; options `-<id>`).
set-page-interface = Interface
set-intro-videos = Intro Videos
set-intro-videos-first_launch = First Launch Only
set-intro-videos-always = Always
set-intro-videos-off = Off
set-intro-videos-desc = Whether the studio and intro videos play when the game starts.

## The graphics page (mw_exterior_bevy graphics_page.rs, tunables T5d): the overall preset and its
## groups (keys `sf.graphics_preset`, `sg.*`); options `-<level>`, `custom` shown when the values
## match no level.
-graphics-extremely_low = Extremely Low
-graphics-low = Low
-graphics-medium = Medium
-graphics-high = High
-graphics-ultra = Ultra
-graphics-custom = Custom
set-page-graphics = Graphics
set-graphics-preset = Graphics Preset
set-graphics-preset-extremely_low = { -graphics-extremely_low }
set-graphics-preset-low = { -graphics-low }
set-graphics-preset-medium = { -graphics-medium }
set-graphics-preset-high = { -graphics-high }
set-graphics-preset-ultra = { -graphics-ultra }
set-graphics-preset-custom = { -graphics-custom }
set-graphics-preset-desc = Sets every group below at once. Reads Custom once a group differs. Texture resolution, view distance and global illumination change after a restart; the rest at once.
set-graphics-view-distance = View Distance
set-graphics-view-distance-extremely_low = { -graphics-extremely_low }
set-graphics-view-distance-low = { -graphics-low }
set-graphics-view-distance-medium = { -graphics-medium }
set-graphics-view-distance-high = { -graphics-high }
set-graphics-view-distance-ultra = { -graphics-ultra }
set-graphics-view-distance-custom = { -graphics-custom }
set-graphics-view-distance-desc = How far detail reaches: level of detail, small-object culling and the distant land. The distant land's reach changes after a restart.
set-graphics-shadows = Shadows
set-graphics-shadows-extremely_low = { -graphics-extremely_low }
set-graphics-shadows-low = { -graphics-low }
set-graphics-shadows-medium = { -graphics-medium }
set-graphics-shadows-high = { -graphics-high }
set-graphics-shadows-ultra = { -graphics-ultra }
set-graphics-shadows-custom = { -graphics-custom }
set-graphics-shadows-desc = Sun shadow cascades and reach, soft shadows, lamp shadows and contact shadows.
set-graphics-lighting = Lighting
set-graphics-lighting-extremely_low = { -graphics-extremely_low }
set-graphics-lighting-low = { -graphics-low }
set-graphics-lighting-medium = { -graphics-medium }
set-graphics-lighting-high = { -graphics-high }
set-graphics-lighting-ultra = { -graphics-ultra }
set-graphics-lighting-custom = { -graphics-custom }
set-graphics-lighting-desc = Ambient occlusion and global illumination. Global illumination changes after a restart.
set-graphics-post = Post-processing
set-graphics-post-extremely_low = { -graphics-extremely_low }
set-graphics-post-low = { -graphics-low }
set-graphics-post-medium = { -graphics-medium }
set-graphics-post-high = { -graphics-high }
set-graphics-post-ultra = { -graphics-ultra }
set-graphics-post-custom = { -graphics-custom }
set-graphics-post-desc = Bloom and the anti-aliasing and upscaling mode.
set-graphics-textures = Textures
set-graphics-textures-extremely_low = { -graphics-extremely_low }
set-graphics-textures-low = { -graphics-low }
set-graphics-textures-medium = { -graphics-medium }
set-graphics-textures-high = { -graphics-high }
set-graphics-textures-ultra = { -graphics-ultra }
set-graphics-textures-custom = { -graphics-custom }
set-graphics-textures-desc = Texture resolution and relief (parallax) on surfaces. Texture resolution changes after a restart.
set-graphics-effects = Effects
set-graphics-effects-extremely_low = { -graphics-extremely_low }
set-graphics-effects-low = { -graphics-low }
set-graphics-effects-medium = { -graphics-medium }
set-graphics-effects-high = { -graphics-high }
set-graphics-effects-ultra = { -graphics-ultra }
set-graphics-effects-custom = { -graphics-custom }
set-graphics-effects-desc = Weather on surfaces: puddles, and dry ground where roofs and trees shelter it.
set-graphics-water = Water
set-graphics-water-extremely_low = { -graphics-extremely_low }
set-graphics-water-low = { -graphics-low }
set-graphics-water-medium = { -graphics-medium }
set-graphics-water-high = { -graphics-high }
set-graphics-water-ultra = { -graphics-ultra }
set-graphics-water-custom = { -graphics-custom }
set-graphics-water-desc = The quality of the water surface.
set-graphics-sky = Sky
set-graphics-sky-extremely_low = { -graphics-extremely_low }
set-graphics-sky-low = { -graphics-low }
set-graphics-sky-medium = { -graphics-medium }
set-graphics-sky-high = { -graphics-high }
set-graphics-sky-ultra = { -graphics-ultra }
set-graphics-sky-custom = { -graphics-custom }
set-graphics-sky-desc = Cloud detail.

## Save/load screen (mw_saves_bevy menu.rs; fields after OpenMW's savegame dialog). The default
## name is OpenMW's "Day 24 - 16 Last Seed 7 p.m."; reasons `saves-reason-<SaveReason token>`.
saves-reason-quick = Quicksave
saves-reason-auto = Autosave
saves-reason-exit = Exit save
saves-reason-guestleft = Guest left
saves-untitled = Save { $n }
saves-damaged = Damaged save
saves-default-name = Day { $day } - { $date } { $month } { $hour } { $half }
saves-am = a.m.
saves-pm = p.m.
saves-level = Level { $level }
saves-played = Played { $time }
saves-stamp = Saved { $date } UTC
saves-duration-hour = { $n } h
saves-duration-minute = { $n } min
saves-duration-second = { $n } s
saves-place-exterior = Vvardenfell
saves-cant-save = Can't save now
saves-cant-load = Can't load during a transition
saves-delete-failed = Delete failed: { $error }
mw-month-0 = Morning Star
mw-month-1 = Sun's Dawn
mw-month-2 = First Seed
mw-month-3 = Rain's Hand
mw-month-4 = Second Seed
mw-month-5 = Midyear
mw-month-6 = Sun's Height
mw-month-7 = Last Seed
mw-month-8 = Hearthfire
mw-month-9 = Frostfall
mw-month-10 = Sun's Dusk
mw-month-11 = Evening Star

## HUD (mw_player_bevy hud.rs): bar labels `hud-bar-<id>` (Morrowind's own names), use prompt
## verbs `hud-use-<verb>`.
hud-bar-health = Health
hud-bar-magicka = Magicka
hud-bar-fatigue = Fatigue
hud-use-talk = Talk
hud-use-open = Open
hud-use-use = Use
hud-use-read = Read
hud-use-take = Take

## Main menu (mw_exterior_bevy main_menu.rs): entries `main-menu-<id>`, in Morrowind's order
## (OpenMW mwgui/mainmenu.cpp).
main-menu-continue = Continue
main-menu-new = New Game
main-menu-load = Load Game
main-menu-options = Options
main-menu-quit = Quit

## Pause menu (sf_ui_screens pause_menu.rs; mw_exterior_bevy pause_menu.rs localizes these):
## the entry back to the front end and its question.
pause-main-menu = Main menu
pause-confirm-main-menu = Return to the main menu? Unsaved progress will be lost.
