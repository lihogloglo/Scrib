# Visual checks: player-ui

Open visual checks: player, inventory, saves, input, dialogue and journal, console (PL, FP, IV, SV, IN, DJ, CO). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

## Player (ROADMAP 2.1; `crates/mw_player_bevy/src/lib.rs`)



### FP-1: First person shows your body and arms, head hidden (`cargo dev view "Seyda Neen"`, 10:00; 3.4b, superfargoth first-person.md)
- Kind: game location, player mode, default flags. Tab to first person; F readies the weapon (if the player carries one), F again sheathes.
- Look for: looking ahead, no face, hair or helmet in view (no inside-of-head polygons at the screen edges); looking down,
  chest, legs and feet; your arms and hands (and the drawn weapon) in their third-person pose, following the clips.
  Tab back: the full body (head included) in third person, no frame with a missing or doubled part at the switch.
- Known for this slice: arms use the world FOV and can clip into walls (view model is FP-2). Report the look-down
  view (neck/collar line, torso). The headless shadow is fixed by FP-1b below.
- Status: awaiting owner (built 2026-11-10, not run here: no display).

### FP-1b: In first person your shadow keeps its head (`cargo dev view "Seyda Neen"`, 10:00, then a lit interior at night; 3.4b)
- Kind: game location, player mode, default flags (raster); optionally `--rt` (solari build) after.
- Look for: under the sun (10:00, stand with the sun behind you on open ground), Tab between first and third person: the
  ground shadow is the same full body, head and helmet included, no flicker or one-frame loss at the switch. Then at
  night next to a lamp (an interior or a lit street), and holding a torch: the head's shadow on the wall/floor in first
  person. Walk through a door: still there on the other side. Nothing of your own head ever on screen in first person.
  With `--rt`: the head casts like the rest of the body there (Solari 0.20's skinned-mesh gap, rendering.md, applies).
- Status: awaiting owner (built 2026-11-10, not run here: no display).

### FP-2: First-person view model: arms and weapon at their own FOV, never in walls (`cargo dev view "Seyda Neen"`, 10:00; 3.4b)
- Kind: game location, player mode. Tab to first person, F readies the weapon, then backquote console `flip viewmodel`
  (A/B in one run; off is FP-1b's look). First GPU run, log line check: no `ERROR` naming `view_model.wesl` or
  `view_model_prepass.wesl` (shader or pipeline creation), and the arms are visible at all with it on.
- Look for: the drawn weapon and hands slightly larger (body FOV 50 vs world 60); stand face against a wall (Census
  office): arms and blade stay drawn, never cut by the wall; arm lighting matches the body/world under the sun (10:00),
  next to a lamp at night and holding a torch (no dark or over-bright arms, shadows on them plausible); turn fast with
  DLSS on and off (F4) and TAA: no smear or ghost trail on the arms; SSAO: a dark halo around the arm silhouette
  (known cost of the depth slice); look down: the shoulder seam where the larger arms meet the torso (spec open
  question 4: accept, blend, or `fov_body = fov_world`?). Report the toggle's default you want.
- Status: awaiting owner (built 2026-11-10, not run here: no GPU).



### IV-1: Take items and Fargoth's ring (`cargo dev view "Seyda Neen, Census and Excise Office"`) (partial)
- E on a loose item (bottle, plate, book): it vanishes at once and stays gone after leaving and re-entering the
  office (overlay removal); log line `take: <id> (now N)`. Doors, NPCs and statics are unchanged.
- E on the ring barrel (`chargen barrel fatigue`, IV-2): the container window; take the ring. The barrel is
  now solid (containers got collision). Then talk to Fargoth outside, topic `ring`, "Yes": quest Fargoth's Ring
  finishes (J shows it), the ring leaves your inventory. Shift+U (dev ring) still works.
- Known: no theft for owned items, no weight. The barrel now opens the container window (IV-2).
- I (pad Y / Triangle, rebindable on F10's gameplay rows) opens the inventory: record names, counts, DDS icons, tabs All/Weapon/
  Apparel/Magic/Misc; after the barrel it lists Fargoth's Ring. I or Esc closes; you can't walk or press E while open.
  Look for: icons drawn (not blank squares). Read-only: no equip/drop yet. Shot: `--open-ui inventory` (dev sample items).
- Owner 2026-10-25: partial. Inventory opens; layout looks off, hover works poorly, no paper-doll view, no
  drag-to-floor drop. Queued after the physics work (layout, hover, drop, then paper doll).

### IV-2: Container window: the ring barrel outside the Census office (`cargo dev view "Seyda Neen, Census and Excise Office"`)
- The ring barrel (`chargen barrel fatigue`) is outside the office, not in it (exterior Seyda Neen
  -2,-9, near the office). E on it opens the container window titled "Barrel": its stacks left (Fargoth's ring), yours right.
- Click the ring (or A / drag it across): it moves to your pane; click it there: it goes back. X on a stack of more
  than one asks how many. Take all (Y) moves everything and closes the window; E again shows it empty.
- Look for: both grids drawn with icons, the window centred, Esc / B / Close shut it, no walking while open; I then
  lists the ring. Owned containers in the office show the owned heading colour (flag only: no theft yet).
- Known: no weight limit, no locks or traps, levelled entries skipped, organic containers refuse puts silently.
- Owner 2026-10-30: ring flow, move both ways, Take all work. Wanted: item icons show only on hover (must always draw);
  the key that opened the container (E) should also close it; inventory UI wonky overall. Status: confirmed with follow-up.

### SV-1: Quicksave / quickload, save, quit, load (`cargo dev view "Seyda Neen, Census and Excise Office"`) (confirmed with follow-up)
- In player mode take a loose bottle (the ring barrel is outside the office, IV-2), step into the middle of the room, Tab to first
  person, F5: toast "Quicksaved" (log `saved ...\saves\single\<world>\000001-quick.sfsave`).
- Walk elsewhere, take another item, F9: fade, you stand where you saved facing the same way, first person, toast
  "Loaded"; I lists the ring; the barrel gives nothing more; the first bottle stays gone.
- Go outside (door), F9: fade back into the office at the saved spot (cross-worldspace load).
- Quit, start again, F9 straight away: same spot and inventory (newest quicksave of any world).
- Load correctness: F5, take a bottle and knock a crate or basket across the floor, open the inventory (or talk to
  someone), F9: the menu closes, the bottle is back once (no double), the crate lies where it was at the save; you can
  walk and look right away. Jump, F5 mid-air, F9: you land as from the jump.
- Known: F5/F9 rebind on F10 (gameplay rows), no gamepad default. Dev keys moved: far tint Shift+F6, copy camera Shift+F12.
- Owner 2026-10-25: save/load works. BUG: on load the player sometimes spawns too low and falls through the floor
  into the void. Also seen: an invisible hole in the Census Office floor. Both in the physics fix plan (2026-10-26).

### SV-2: Save/load screens from the pause menu (`cargo dev view "Seyda Neen, Census and Excise Office"`, player mode)
- Esc, Save: the save screen over the paused game, "New save" on top. Confirm it (select, confirm again): toast
  "Saved"; the list gets a row titled "Seyda Neen, Census and Excise Office", subtitle "Day N - 16 Last Seed 9 a.m. ·
  Played ... · Saved <date> UTC" (OpenMW's fields).
- Select that row, confirm, Yes: overwritten (a `.bak` next to it). F5 once: a "Quicksave" row.
- Back: the pause menu. Load: the same saves, newest first; pick one: fade, the menus close, you stand at the save.
- Load screen, a save selected, Delete (or Context1), Yes: the row goes away.
- Known: no typed names (generated, not stored: sf/meta has no name field), no thumbnails, UTC dates.
- Owner 2026-10-30: save, overwrite and load work; thumbnails missing (known gap, now wanted). Status: confirmed with follow-up.

### KM-1 (was IN-1): Keymap controls, rebind, gamepad, sneak hold vs toggle (`cargo dev view "Seyda Neen"`, 10:00)
- F10 opens the keybinds screen (UI + gameplay rows). Rebind Jump (keyboard slot) to J, close with F10 or Esc; J jumps,
  Space doesn't. Quit, start again: J still jumps (saved in `profiles/<profile>.toml`, `[bindings.gameplay]`).
- Gamepad (Xbox layout): left stick walks (dead zone, no drift), right stick looks at a steady rate, A jumps, X uses
  (door / NPC prompt), L3 sprints (held), B sneaks while held, R3 switches first/third person.
- Ctrl / B: sneak only while held (default). On the Sneak row press the Hold cell (reads Toggle): one press sneaks,
  the next stands; restart keeps Toggle. Sprint row works the same. Look for: no stuck sneak after closing the menu.
- Labels (uipolish, awaiting owner): rows read Move Forward / Move Back / Move Left / Move Right, Jump, Run, Sneak,
  Activate, Toggle POV, Inventory, Camera Zoom, Look; menu rows Select, Back, Menu Up, Previous Tab... F10 no longer
  toggles clouds (Shift+C does, or console `flip clouds`).
- Status: menu works (owner 2026-10-25); labels fixed (above). Rebind persistence, gamepad, hold/toggle not reported yet.

## Dialogue and journal (superfargoth `journal-ui.md`, `ui.md`; ROADMAP 2.4 / 2.2; `.sessions/dialogue2.md`)

### DJ-1: Talk to Fargoth on E, ring flow, journal on J (`cargo dev view "Seyda Neen"`)
- Walk (third person) to Fargoth outside the Census office; face him: E opens the dialogue window;
  WASD and mouse look stop while it is open. Ask "ring", answer yes: no ring, disposition drops.
- Shift+U (dev: gives the ring), ask "ring" again, yes: he thanks you; Goodbye or Esc closes; controls return.
- J: the journal opens at the newest spread with both Fargoth's Ring entries (Day 1 headings); quest list (Finished tab) shows
  "Fargoth's Ring" (finished, new marker); topic index R tab has "ring" with his lines. J or Esc closes it.
- Theme (U3 mwtheme): Mystic Cards font, ini palette: speaker/headings pale gold, text tan, links blue, answers
  dark red, topics tan; black translucent panels in bronze borders. Message boxes (`--open-ui message`): framed.
- uipolish (awaiting owner): no bright outline on any button with the mouse; every button (topics, answers, Goodbye,
  journal tabs/rows/< >, Yes/No, keybinds cells) turns lighter on hover and near-white while pressed; with
  arrows/pad the focused one shows the hover colour. Journal: both pages' text inside the parchment (no text
  on the covers or past the bottom), `< 1/2 >` centred on the book's bottom edge, index panel beside the book.
- dialogue3 (awaiting owner): dialogue window and journal open centred. In the Census office (new game, no chargen):
  the guard lists latest rumors/little advice/little secret, Sellus Gravius Caius Cosades + my trade; Socucius only
  imperial legion once a greeting named it (his only topics in the data). Shot: `--open-ui dialogue`. sellus:
  Sellus Gravius greets "I am Sellus Gravius. My background is not important...", not "Where are your papers?".
- Known gaps: frames are flat CSS borders, not the 8-piece menu textures; no HUD; day headings placeholders;
  seen markers not saved.
- Status: flow confirmed by owner 2026-10-25 (E talk, Shift+U ring, journal updates); Sellus greeting confirmed
  2026-10-25 ~23:40. No barter/persuasion yet (expected). Wish (later): resizable dialogue window; check OpenMW's
  recent UI refactor first. Journal layout, outlines and hover (uipolish) not reported yet.

### DJ-2: Dialogue action buttons and the Persuasion submenu (`cargo dev view "Seyda Neen"`)
- Written 2026-11-08 without compiling (UNTESTED; `test-pass-2026-11-10.md`); needs the persuasion-menu host slice too.
- Talk to Fargoth (E): a "Persuasion" button sits above the topic list, same look as the topics; Goodbye stays under it.
- Click it (or pad: up from the first topic, A): the window dims and a centred panel lists Admire, Intimidate, Taunt
  and the three Bribes; bribes you can't afford are greyed and skipped by the pad/arrows.
- With the panel open, topics, links and Goodbye behind it don't react to hover or clicks.
- Esc / B closes the panel, focus back on Persuasion. Picking Admire closes it and Fargoth replies; disposition changes.
- Focused rows show the hover colour with arrows/pad, no bright outline with the mouse (as uipolish).
- Known gaps: no Morrowind theme rules for the new classes yet; clicking the dimmed backdrop doesn't close the panel.
- Status: awaiting owner.

### PUI-SV3: Trade screen over the dialogue (`cargo dev view "Seyda Neen, Arrille's Tradehouse"`)
- Built 2026-11-10 (TR2), headless tests only. Talk to Arrille (E), press Barter: the trade window opens centred over the dialogue.
- Left his stock, right your items, deal bar between; a price on every row; rows he won't buy greyed ("not traded"); no gold rows.
- Pad: d-pad rows and sideways to/from the bar, LB/RB panes, A stages whole, X asks how many, Y accepts, B backs out then cancels.
- Bar: lines per side, totals, both purses, balance; haggle stepper left/right (LB/RB 10%), Reset; a refusal shows on the bar.
- Mouse: hover focuses, click stages, drag a row onto the bar or the other pane, drag a line back; nothing sticks.
- Accept (enough gold): the window closes back to the dialogue; the items and gold moved (I after Goodbye). Barter again: new stock.
- Cancel or Esc: back to the dialogue, nothing moved. Goodbye with the window open is not reachable (it has the input).
- Known gaps: English fallback labels, not the GMST strings; no "deal done" message; no Morrowind theme rules for `sf-trade-*`.
- Status: awaiting owner.

### PUI-SV4: Training list over the dialogue (`cargo dev view "Balmora, Fighters Guild"`)
- Built 2026-11-10 (A3), headless tests only. Talk to Eydis Fire-Eye (named from memory: any NPC with a Training button works), press Training.
- A list opens centred over the dialogue: title "Training", your gold, her three best skills (by her base skill), each with "skill now -> next" and a price.
- Rows she can't teach stay greyed with the reason (her skill not above yours; yours at its governing attribute); rows above your gold greyed "Too expensive".
- A or click a live row: "are you sure?" (2 hours pass), yes: gold drops by the price, "bought" line, the row shows the skill +1 within a tick; the list stays open.
- After: the sky and the hour moved 2 h (HUD/console clock), the skill +1 in the stats screen; Back returns to the dialogue; Goodbye from there closes all.
- Known gaps: no fade or progress bar (time jumps); the "are you sure?" and toasts use English fallbacks; no Morrowind theme rules for `sf-service-*`.
- Status: awaiting owner.

## Dev console (superfargoth `console.md`; ROADMAP 1.13; `.sessions/console.md`)


### PM-1: Escape menu and options (`cargo dev view "Seyda Neen"`, player mode)
- Esc opens the pause menu (Resume, Save, Load, Options, Quit; Save/Load greyed out until the save screens are wired);
  the world stops (NPCs, sky clock, physics). Esc or Resume closes it and the world continues.
- Options: Audio page (Master/Music/Sfx/Voice/Ambience/UI volume and mute); left/right change a value and you hear it
  at once; holding an arrow repeats. "Key bindings" opens the F10 screen on top; Esc returns to options. Leave the
  menus, quit, restart: the volumes are kept (`profiles/<name>.toml`, `[settings.sf]`).
- Quit asks Yes/No (No is focused); Yes exits. Not in free camera or while the console is open.
- Known: frames are flat CSS (no 8-piece textures); screen titles and On/Off are English defaults.
- Owner 2026-10-30: Esc pauses everything; the mixer works but its UI is bad: volume should drag with the mouse
  (sliders), mute should be one small icon per row. Wanted: a "Main menu" button in the pause menu. Status: confirmed with follow-up.



### MM-1: Release start: main menu with no world behind it, New game (`bevyrrowind` with no arguments)
- Build `cargo build -p bevyrrowind --features dev`, run `target/debug/bevyrrowind.exe` (no subcommand).
- Front end (F1 + B1, 2026-11-11): the main menu over Morrowind's looping menu video (`mw_menu.bik`, silent; log
  `main menu: backdrop video video/mw_menu.bik`), filling the window without stretching (cover: at 16:9 the
  4:3 video is cropped top and bottom, nothing squashed), **no world**: no terrain, sky, NPCs or HUD behind it, window title stays "Bevyrrowind" (no fps/cells line). Esc does nothing. Options, Quit as before.
- New Game: the menu closes, Seyda Neen streams in outside the Census and Excise Office, the player can walk; Esc opens
  the pause menu. Log: `main menu: new game`, then `app phase: start area resident, in game`.
- Continue / Load Game (F3): arrive at the save's place with no glimpse of Seyda Neen; Load shows the save list over
  the menu (Back returns to it). Log: `app phase: loading`, `app phase: save loaded`, then `start area resident, in game`.
- Pause menu (F2c): "Main menu" between Options and Quit asks Yes/No (No focused); Yes returns to the menu over the
  video with no world left; New game from there starts fresh (log `main menu: new game from baseline`).
- Loading screen (F4): New game, Continue and Load show a random Morrowind splash (`splash/`, a new one each time)
  with a progress bar that only moves forward (a sweep until the first loads), gone the moment the world shows.
- Dev runs (`view`, `scene`) unchanged: no menu, no loading screen, the world at once.
- Intro (I1): first launch plays the Bethesda then Morrowind logo with sound, letterboxed; Esc, Enter or a click skips
  one. Next launch none (`intro: already seen`; delete `<user data>/intro_seen` to replay). Options > Interface.
- Owner 2026-10-30: rejected the menu over the live world (wants title art/video). Owner 2026-11-11: flow confirmed (intro, menu, New game, Continue/Load, Main menu); look crude ("text over a gray box", "a bad html webpage"), nothing like OpenMW or Morrowind. Status: confirmed with follow-up.

### OP-1: Options screen rows on stock widgets (S2a, PM-1 follow-up)
- Same run, Options from the main or pause menu. Audio page: drag a volume thumb, it follows the mouse in 5 % steps,
  the % text and the volume change live; a click on the track jumps there; left/right and the arrows still step it.
- Mute rows are checkboxes (click, Enter or left/right flips; no arrows); greyed rows ignore the mouse; the thumb never
  sticks out past the track. S2b1: a pane right of the rows shows the hovered (else focused) row: label, description,
  disabled reason, "Default: 100%"; hover never moves the keyboard highlight. Fallback colours. Owner 2026-11-11: works; "just a list", keybinds worse (separate window, not centred). Status: confirmed with follow-up.

### LK-1: OpenMW frames on the stock screens (look pass L1a + L1b, 2026-11-11)
- Release start on the real install (frames come from its `menu_*` pieces; log `ui.frames: ... skipped []`).
- Windows (options, keybinds, pause, save/load, yes/no modal, inventory, container, trade, dialogue, journal index):
  OpenMW's thick bronze frame, crisp, no blur or seams, over translucent black (world or video shows through).
- Inner boxes (option rows, description pane, save list, item grid, trade lists, dialogue log): the thin frame, nothing over it.
- Framed buttons (Back, Keybinds, Yes/No, Load/Save/Delete, Goodbye, OK, Take All, Accept): button frame, at least 136 px wide, label centred.
  Bare text buttons (option rows, tabs, topics, pause entries) have no frame. Dialogue submenu opaque.
- Loading screen: the footer framed, the bar's fill inside its frame. Main-menu buttons still text (L2 next); HUD bars keep CSS borders.
- Owner 2026-11-11: opening a submenu (e.g. Options) does not hide the menu it came from: both draw on top of each other, unreadable. Logged, not fixed.

### LK-2: Title menu as OpenMW's image buttons (look pass L2, 2026-11-11)
- Same release start as LK-1 (no arguments), on the title screen; art from the install's `menu_<id>{,_over,_pressed}.dds`.
- No panel or frame: a column of New Game / Load Game (only with a save) / Options / Exit art, centred, 24 px from the
  bottom, neighbours an even ~48 px apart, no art clipped. Continue (only with a save) is gold text above, same width.
- Mouse hover or arrows/d-pad show the "over" art, holding the button the "pressed" art. "Bevyrrowind 0.1.0" bottom-right
  (a black shadow may be missing). Without the art, entries fall back to text, none blank. Owner question: keep Continue?
- Owner 2026-11-11: keep Continue, but it must match the other entries' style (or the others align to Continue; prettier later). Logged, not fixed.

### PL-6: Recovery delay after jumping down from a high box (owner report)
- Kind: synthetic. `scene level-anim-ledges`, F11; stand on a high box, Space off it toward the ground, keep W held.
- Seen 2026-10-30: after the impact the character waits before it runs again.
- Cause (code read, not run): rule `land.hard` (`sf_action/src/base_rules.ron`): touchdown faster than 9 m/s down (about a
  4 m drop, reached by jumping off a ~2-3 m box) plays `air.land.hard` Committed with `steer: false` until the clip ends.
  Feel question for the owner: raise the threshold, shorten/cancel the clip on input, or roll out of it (Elden Ring only
  locks on big falls).
- Status: known bug: cause found, tuning queued (owner pass 2026-10-30).
- Owner decision 2026-10-30: a movement lock belongs to damage only; no damage logic exists yet, so `land.hard` no
  longer commits or stops steering (base_rules.ron; test `hard_landing_never_stops_the_stick`). Re-check in a build
  after 2026-10-30 14:00. Status: awaiting owner.

### PL-7: Player foot placement on slopes and stairs (P-IK1, 2026-11-11)
- Kind: synthetic. `cargo dev scene level-flat-slopes` (cameras `ramps`, `stairs`) and `cargo dev scene level-anim-stairs`,
  F11 player, Tab third person, 12:00. K is the A/B (log `walker foot placement on/off`; it switches walkers and player).
- Before: the player got no ground samples (diagnosis `superfargoth/docs/research/ik-feel-diagnosis-2026-11-11.md`):
  feet sank or floated 8-34 cm on ramps and treads, planted feet skated, no hip drop, trunk upright on ramps.
- Walk and run up and down the 10/20/30° ramps and the three stairs; stop and turn sharply on a ramp. Look for: feet on
  the ramp and on each tread (no sinking, no floating; the sim proxy floats a foot ~15 cm on 0.15 m treads at walk:
  say if you see it), planted feet staying put on stops and quick turns, the hips
  dropping onto the lower foot, the trunk leaning uphill; K off brings back the old floating feet.
- Not this slice: a once-per-step hip bob on ramps, worse downhill (walker cause 2, slice P-IK3), now shows on the
  player too. Mantles, attacks and rolls drop the probes (their clips place the feet).
- Status: failed (owner 2026-11-11): the IK overrides the animation, legs spread a lot; the slope lean is far too strong on the player (head very tilted), though AN-21 on Fargoth was fine. Diagnosis P-IK1b.
- Re-test (owner 2026-11-11, later): feet ARE placed correctly on the ground. Wrong: on stopping a leg (often the right)
  pops back brutally and the blend looks odd; sometimes the whole body rises from the hips as if standing up; on slopes
  the arms stay straight and point backwards on steep ones; the head is too tilted (people look a few metres ahead).
  Walking down a steep slope the steps get outrageously long. Owner wants a deep rethink of stops (distance-matched
  stop steps or a Sekiro-like system), not symptom fixes.

### WALK-0: A/B ladder on the player's walk, one op off at a time (3.4c A0, 2026-11-12)
- Kind: synthetic. `cargo dev scene level-flat-slopes` (run `bevyrrowind bake` first: graph format 10), 10:00, camera `overview`
  then `ramps`; F11 player, Tab third person, orbit to the side. Backquote console, one at a time, `on <name>` to restore:
  `off speed_match` (clips at their authored rate: walk 0.92 -> 1, run 1.41 -> 1, the stride warp takes the rest up to its cap, feet may skate), `off stride_warp` (steps at
  authored length), `off foot_placement` (= K: no foot IK, player and walkers), `off lean` (no slope trunk lean; ramps),
  `off side_lanes` (forward lane only: no side/back clip during turns; turn sharply and circle).
- Look for: which switch makes the flat walk and run look right (or least wrong), and on the ramps which one fixes the
  tilted head and long downhill steps. Taste questions: does a stop feel too short or too long; should walking uphill
  slow you down; where should the head look (ahead along the travel, a few metres down the slope, or the camera)?
- Status: awaiting owner.


### PL-9: Player run looks sideways (owner report 2026-11-15)
- Kind: game location, third person, any exterior; run (and sprint) straight ahead, then strafe-run.
- Seen by owner 2026-11-15: "the animation of running is kinda sideways, character runs in diagonal". Not diagnosed. Leads to
  check: run clip root/facing yaw after retarget, `side_lanes` / lean in the walk ladder (WALK-0 A/B), speed-match picking a
  strafe-diagonal clip, body yaw lagging the travel direction. Status: known bug.

### MF-1: Menu flow v3, one window at a time, nothing moves on hover (ROADMAP 2.2b, 2026-11-23)
- Kind: game location, dev build. `view "Balmora" --hour 22`. Esc > Options > Key bindings > Back > Back; then Options from
  the main menu (release start) the same way.
- Look for: only the top window is drawn (pause is gone while Options is open, Options gone while Key bindings is open);
  Back shows the one below at once; hovering rows, changing a value or switching page (Q/E or the tabs) moves nothing;
  long pages and the key list scroll (wheel, arrows); Key bindings sits exactly where Options sat; footer [Key bindings]
  [Reset page] [Back]: Reset page asks Yes/No (No first) and puts only that page back; new page **Advanced** after
  Graphics lists the player feature switches (fog volume, groundcover, bloom...), each live behind the menu, kept after a
  restart. Also judge: the options window is now 60% x 70% of the screen (1152x756 at 1080p, a bit larger than before).
- Status: confirmed 2026-11-23 (owner: "seems to be working fine"). Source: feature-toggles (MF-1a..MF-6). Unblocks PP-4.

### PL-10: Slope lean: small, only while moving, head level, arms hang (lean-fix, 2026-11-25)
- Kind: synthetic. Bake first (`bevyrrowind bake`: graph format 12). `cargo dev scene level-flat-slopes` (cameras `ramps`,
  `stairs`), F11 player, Tab third person, 12:00; walk then jog up and down the 10/20/30 deg ramps; NPC walkers share it.
- Before (owner PL-7 2026-11-11, again 2026-11-25): trunk tipped up to 26 deg, head pitched with it (+23 deg at 30 deg),
  hands ~11 cm behind the back. Now (headless, 30 deg up): walk trunk +5.8, head -0.2, hands 1.1 cm forward; jog trunk
  +2.3, head -2.6. Round 2 (owner: "much better", but the lean showed standing, not walking: the first cut faded out
  at 2.5-6.5 m/s): now none standing, whole from 1.2 m/s; 30 deg up walk trunk +5.8, jog +4.6, head level. Look for: a
  slight lean into the slope while walking and jogging, none standing still, the head ahead, arms hanging naturally.
- Tune: `mw_graphs/src/build.rs` humanoid `Lean` (up 0.3, max 0.15 rad, rise 0.2-1.2 m/s, counter head 0.9, upper arms 0.85).
  Not changed: the Kimodo walk's own backward arm swing on flat ground (clip style, separate issue).
- Status: confirmed 2026-11-25 round 2 ("slope thing is all good"; round 1: "much better", lean only showed standing). Source: gameplay-roadmaps lean-fix (`docs/research/gameplay-2026-11-25/anim-diagnosis.md` §1).
