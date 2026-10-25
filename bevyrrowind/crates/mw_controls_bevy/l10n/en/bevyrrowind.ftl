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
keybind-gameplay-sprint = Run
keybind-gameplay-sneak = Sneak
keybind-gameplay-use = Activate
keybind-gameplay-toggle_view = Toggle POV
keybind-gameplay-inventory = Inventory
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
