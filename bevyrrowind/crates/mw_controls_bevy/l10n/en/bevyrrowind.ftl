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
