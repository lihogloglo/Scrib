# Morrowind menus: video, title art, UI textures (research, 2026-11-11)

Companion to `superfargoth/docs/research/video-playback-2026-11-11.md` (decoder choice, `VideoSource` seam). Menu texture
list: `bevyrrowind/docs/specs/ui-adapter.md` lines 51-73 (not repeated here). OpenMW references are at commit
`97722d48f98f14b323c3880a698e8ce81babf7b3` (master, 2026-11-11), paths under `apps/openmw/`.

## What is on this machine (headers only, nothing copied)

All 17 files in `Data Files/Video/` are Bink 1 revision `i` (`BIKi`), 640x480, about 30 fps. Header byte 0x28 (audio track
count) shows which have sound:

| File | Frames | Audio | Role (from `Morrowind.ini` `[Movies]`) |
|---|---|---|---|
| `bethesda logo.bik` | 481 | yes | `Company Logo`, first thing at startup |
| `mw_logo.bik` | 491 | no | `Morrowind Logo`, after it the title music plays |
| `mw_intro.bik` | 1678 | yes | `New Game` (and `[Question 1] New Game`), plays on starting a new game |
| `mw_menu.bik` | 885 (about 29.5 s) | **no** | `Options Menu`; silent 640x480 loop, the only candidate for a looping menu background |
| `mw_credits.bik` | 4051 | no | credits (OpenMW plays `mw_credits.bik` from the menu's Credits button) |
| `mw_cavern.bik`, `mw_end.bik` | 1260 / 1950 | yes | cutscenes (cavern; ending) |
| `bm_*.bik` (11 files) | 181-1887 | yes | Bloodmoon cutscenes (bear hunt, ceremony, frost giant, werewolf, endgame) |

`Morrowind.ini` also lists `Loading=mw_load.bik`, which is not in the install (no such file; OpenMW ignores it too).
Title music is `Data Files/Music/Special/morrowind title.mp3`. `Splash/` has 11 TGAs (`Splash_Bonelord.tga` and others; the
sampled header is an uncompressed-ish 1024x1024 TGA, matching OpenMW's comment that Morrowind splashes are 1024x1024).
Unverified: whether vanilla's main menu actually displays `mw_menu.bik` (I believe it is the animated menu loop, but only the
ini label "Options Menu" and the silent 30 s loop shape support this); confirm by watching the original once or reading the
exe's string table.

## What OpenMW does

- **Startup order** (`engine.cpp` lines 928-933 and 1045-1058): unless `--skip-menu`, play `Movies_Company_Logo` from the
  fallback map (ini `Company Logo`) while data loads, skippable by key; then push the main-menu GUI mode, start the title music
  (`MWSound::titleMusic`, warns "Title music not found" if absent), and play `Movies_Morrowind_Logo` (skippable, other sounds not
  paused). With `--skip-menu` none of these play and a new game starts straight away (`--new-game` is ignored without
  `--skip-menu`, `main.cpp` line 143). There is no `--no-intro` option in this tree. The new-game intro (`mw_intro.bik`) is
  played by the script/state code from the `New Game` movie name; every name is looked up as `video\<name>` through the VFS
  (`windowmanagerimp.cpp` line 2146), so a mod can override by name.
- **Menu background** (`mwgui/mainmenu.cpp` lines 46, 69, 112): `MainMenu` checks `mVFS->exists("video/menu_background.bik")`
  (`mHasAnimatedMenu`). If present, a black letterbox `ImageBox` holds a `VideoWidget` that plays `video\menu_background.bik`
  on a thread, restarts it when `update()` returns false (loop), and pauses while the menu is hidden. If absent, a
  `BackgroundImage` of `textures\menu_morrowind.dds` is shown, stretched or letterboxed by the `stretch menu background`
  GUI setting. The video exists only in the main-menu state; `showBackground(false)` destroys it. Note OpenMW uses its own
  name `menu_background.bik`, **not** `mw_menu.bik`.
- **Buttons**: textures `textures\menu_<id>.dds` with `_over` and `_pressed`, ids `return newgame savegame loadgame options
  credits exitgame`; click sound `Menu Click`; credits video `mw_credits.bik`.
- **Loading splash** (`mwgui/loadingscreen.cpp` `findSplashScreens`, lines 59-75, 210-218): iterates `splash/` in the VFS
  recursively, collects every file, picks one at random (`Misc::Rng::rollDice`) per loading screen, and shows it with
  `stretch menu background` applied. Warns when no splash exists. So any mod file placed in `Splash/` joins the pool.
- **Video playback** (`windowmanagerimp.cpp` `playVideo`, lines 2144-2211): modal loop; hides the scene, hides the cursor,
  optionally pauses all non-movie sounds (`overrideSounds && hasAudioStream()`), pumps input and the viewer until the video ends
  or a key is pressed, then resumes sounds.

## Audio in the Bink files

Yes, the intro, cavern, end, Bethesda logo and Bloodmoon clips carry one audio track and it is used; `mw_logo`, `mw_menu` and
`mw_credits` have none (the credits and logo rely on the music stream). OpenMW decodes audio with FFmpeg and hands it to its
sound manager as a `Movie`-type stream: `mwsound/movieaudiofactory.cpp` creates `playTrack(decoder, MWSound::Type::Movie)` and
`MovieAudioDecoder::getAudioClock()` makes the audio the master clock for sync. Movie volume is a separate sound type, so the
player's music and effects sliders do not scale it (OpenMW has a movie category via `Type::Movie`). We should do the same: a
movie bus plus pausing the other buses.

## What the modding scene ships

- **Animated main menu replacers** (Nexus Morrowind: Animated Main Menu for Morrowind 43341, Sunrise over Vvardenfell 50282,
  Morrowind Definitive Menu Animation 50080, Subtle Animated Menu Background 50037, inom - Main menu replacers 47895): all are
  **one replacement video** copied to `Data Files/Video/` and named `menu_background.bik` (OpenMW) or replacing the vanilla
  menu video (original engine with the Morrowind Code Patch); they are Bink, typically 1080p, authored at 16:9 (the 1080p
  ones need `stretch menu background = true` on OpenMW). inom ships seven loops. A commenter on one page reports an `.mp4`
  renamed to `.bik` works on OpenMW, which only works because FFmpeg sniffs the real container: our loader should do the
  same (probe the header magic, not the extension). Sources: https://modding-openmw.com/mods/morrowind-definitive-menu-animation
  (OpenMW "Fully Working"), https://modding-openmw.com/mods/animated-main-menu-for-morrowind/ (listed not working on OpenMW),
  Nexus pages listed. Nexus blocks automated fetches, so file layouts above come from search summaries, not from archive listings.
- **Splash replacers and HD menu texture packs**: not researched in detail (Nexus blocked). Splash mods replace or add
  files in `Data Files/Splash/` (TGA, or DDS and PNG on OpenMW, which reads any format its image loader knows); texture packs
  (e.g. Better Morrowind-style UI or HD menu button sets) replace `Textures/menu_*.dds`, `tx_menubook*`, `menu_morrowind.dds`
  inside loose `Textures/` folders or BSAs. Treat both as ordinary VFS overrides.
- Mod problems seen: OpenMW issue "Vvardenfell Animated Main Menu is skipping" (https://gitlab.com/OpenMW/openmw/-/issues/8323)
  shows loop and sync glitches are the thing users notice; keep the loop path simple and fully buffered.

## VFS lookup order to support (proposal)

Use the normal `sf_vfs` priority (loose files and later mods override earlier BSAs; case-insensitive, `/` and `\` equal).

Menu background, first hit wins:
1. `video/menu_background.bik` (OpenMW mod convention; this is what nearly all menu mods ship)
2. `video/mw_menu.bik` (vanilla name, also what Code Patch style mods overwrite)
3. any other container named `video/menu_background.*` (mp4, webm, ogv, mkv) via the optional FFmpeg back end
4. static `textures/menu_morrowind.dds` (title art; also the fallback when the video fails to decode or no decoder is built)
5. a built-in plain background, as today

Startup: `video/<Company Logo>` then `video/<Morrowind Logo>` names read from `[Movies]` in `Morrowind.ini` (default
`bethesda logo.bik`, `mw_logo.bik`), new game `video/<New Game>` (`mw_intro.bik`), credits `video/mw_credits.bik`; skip on key
and a setting to disable the logos; no file means no video and no error (OpenMW skips silently).

Splash: every file under `splash/` (recursive, any mod and BSA), random pick, TGA/DDS/PNG; stretch or letterbox setting.

UI art: `textures/menu_*.dds`, `tx_*`, cursor, in the order in `ui-adapter.md`; a loose mod file overrides the BSA copy.

## Using Morrowind's textured UI art (notes for the next slice)

Not a decoder problem: the art is DDS and TGA, already read by `sf_texture`/the VFS. OpenMW uses the same texture names as
MyGUI skins (8-piece frame textures with fixed corner sizes) and does button state swaps by suffix (`_over`, `_pressed`), see
`mainmenu.cpp`. In our CSS-based UI (`sf_ui_css`) this wants 9-slice `border-image` support plus a per-state image swap; check what
`sf_ui_css` already parses before writing anything. Stretch rules: menu title art is not square-pixel-safe at 16:9 (hence
OpenMW's stretch toggle); offer "contain/cover/stretch".

## Sources

- OpenMW at commit 97722d48f98f14b323c3880a698e8ce81babf7b3: `apps/openmw/engine.cpp`, `apps/openmw/main.cpp`,
  `apps/openmw/mwgui/mainmenu.cpp`, `apps/openmw/mwgui/loadingscreen.cpp`, `apps/openmw/mwgui/windowmanagerimp.cpp`,
  `apps/openmw/mwsound/movieaudiofactory.cpp`, `extern/osg-ffmpeg-videoplayer/videostate.cpp`
  (https://github.com/OpenMW/openmw). OpenMW is GPL-3.0; read for behaviour only, port with a notice if we copy any logic.
- Local file headers (`BIKi`, 640x480), `Morrowind.ini` `[Movies]`, directory listings of the Steam install.
- Nexus pages: https://www.nexusmods.com/morrowind/mods/47895 , /50282 , /50080 , /50037 , /43341 (via search summaries).
- https://modding-openmw.com/mods/morrowind-definitive-menu-animation , https://modding-openmw.com/mods/animated-main-menu-for-morrowind/
