# Visual checks: player-ui

Open visual checks: player, inventory, saves, input, dialogue and journal, console (PL, FP, IV, SV, IN, DJ, CO). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

## Player (ROADMAP 2.1; `crates/mw_player_bevy/src/lib.rs`)

### PL-1: Third-person walk and the use prompt (`cargo dev view "Seyda Neen"`, 10:00)
- Starts in player mode (the dressed `player` body, PL-2; the capsule is drawn only with characters off). WASD, Shift, Space jump, Tab 1st/3rd, wheel zoom; walk to the Census and
  Excise Office door, then to an NPC (Fargoth by the lighthouse path).
- Look for: smooth walk and camera (no jitter, camera pushes in against walls); under the centre dot the prompt reads
  `[E] Open  <door>` / `[E] Talk  <npc>` / `[E] Take  <item>`, it clears when you look away or step past ~2.7 m;
  E on the door travels, the body arrives at the marker facing out.
- Fixed 2026-10-25 (player3), check: focus is now a physics ray plus actor capsules, so in the Census and Excise Office
  the dot on an NPC's chest reads `[E] Talk  <npc>` (was `Take  Broom`); walls block items behind them.
  `cargo dev view "Seyda Neen, Census and Excise Office"` starts on the floor at the door's arrival marker, facing in
  (was: on the roof, grey void). E does nothing while the journal or dialogue is open.
- Owner 2026-10-30 (release start, New game): walk, camera and use prompt fine. Status: confirmed.

### PL-2: Animated player body (action.md A2 slice 2; `cargo dev view "Seyda Neen"`, 10:00)
- Starts in player mode; Tab to third person. Walk (W, stick), run (Shift sprint), Ctrl sneak, Space jump, turn on the spot.
- Look for: the `player` record's character (race, sex, head, hair, clothes) in place of the capsule; walk, run, sneak
  cycles matching the ground speed (no skating), jump/fall clip off the ground and landing back into locomotion, no
  jitter at any frame rate (the body follows the fixed-tick capsule, interpolated, through the correction
  spring). Tab to first person hides the body (no arms yet). Shot: `shots/player-body-3-walk.png` (indoors, 3rd person).
- Status: confirmed with follow-up. Owner 2026-10-27 (after the IK audit fixes): much better, pelvis and jump fixed. Open:
  movement feel has too much inertia ("like driving a car"); never designed on purpose. Follow-up: PL-4.
- Owner 2026-10-27 (arena): switching third -> first person can leave the view separated from the capsule (bug, open);
  first-person arms not built yet (known).
- Fixed 2026-10-27 (afternoon): the split was the body leaving its capsule, not the camera: Delirium click-to-push hit the
  player's own body on a left-click attack (now F11 only, never action characters) plus the UAL2 split-take clip offset
  (cook `start_anchor`). Owner confirmed the view switch live. Status: confirmed (first-person arms still not built).

### FP-1: First person shows your body and arms, head hidden (`cargo dev view "Seyda Neen"`, 10:00; 3.4b, superfargoth first-person.md)
- Kind: game location, player mode, default flags. Tab to first person; F readies the weapon (if the player carries one), F again sheathes.
- Look for: looking ahead, no face, hair or helmet in view (no inside-of-head polygons at the screen edges); looking down,
  chest, legs and feet; your arms and hands (and the drawn weapon) in their third-person pose, following the clips.
  Tab back: the full body (head included) in third person, no frame with a missing or doubled part at the switch.
- Known for this slice: the player's shadow has no head in first person (shadow layer is slice FP-1b); arms use the
  world FOV and can clip into walls (view model is FP-2). Report the look-down view (neck/collar line, torso).
- Status: awaiting owner (built 2026-11-10, not run here: no display).

### PL-4: Movement feel: starts, stops, reversals (`cargo dev scene level-anim-overview`, F11 player, Tab 3rd person)
- Player-only motion tuning (Elden-Ring-style): fast speed ramp, faster brake, facing capped at 720/1080/480 deg/s
  (walk/run/sprint) with the motion following the facing, so a reversal turns at speed. NPCs keep their 0.27 s springs.
- Try: run (W), release (stop), W then S (reversal), W then D (90 deg), Shift sprint turns (heavier), tap a direction
  from standing. Look for: responsive, no car-like slide; feet not skating at starts/stops; no pop entering run.
- Numbers (`cargo test -p mw_player_bevy measure_player_feel -- --ignored --nocapture`), before -> after: 90 % speed
  0.77 -> 0.20 s; stop from run 1.12 s / 1.73 m -> 0.22 s / 0.32 m; 180 reversal 1.08 s (speed dips to 0.16 m/s) ->
  0.17 s (keeps 4.5 m/s); 90 deg turn 0.90 -> 0.08 s.
- Tweak: `crates/mw_player_bevy/src/feel.rs` (half-lives, turn rates, feed smoothing); gait blends `mw_graphs` build.rs
  `GAIT_BLEND` 0.15 s / `DIRECTION_SMOOTHING` 0.05 s (shared with NPCs, re-bless). Status: confirmed with follow-up
  (owner 2026-10-27: "much much better, about right"; jump air time is long and floaty). Wanted later: run + Ctrl slide.
  Jump fix 2026-10-27: player gravity 9.81 -> 18 m/s² (`feel.rs` `GRAVITY`; height still 1 m): air 0.90 -> 0.67 s. Confirmed (owner 2026-10-27: "jump feels better").

### PL-3: Jump take-off held in the air, landing on touchdown (`cargo dev scene level-anim-ledges`, F11 player)
- Humanoid graph B2: `jump` plays `Jump@head` and holds it while airborne; touchdown plays `Jump@tail` (`land`), then
  locomotion; running cuts the landing into the run.
- Jump in place, then running. Look for: one tuck at take-off, held in the air, knees absorb on touchdown (land
  sound with it); no second leg lift mid-air.
- B3b feed: walking off a ledge (here or in anim-lab) shows the fall hold (no take-off tuck) after ~0.1 s, then lands;
  running down stairs or over small steps shows no fall pose at all.
- Status: confirmed (owner 2026-10-27: jump feels correct).

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

### IN-1: Keymap controls, rebind, gamepad, sneak hold vs toggle (`cargo dev view "Seyda Neen"`, 10:00)
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

## Dev console (superfargoth `console.md`; ROADMAP 1.13; `.sessions/console.md`)

### CO-1: Live SSAO and A/B switches from the console
- Kind: game location. Build `cargo build --release -p bevyrrowind --features dev` (no `solari` needed), run
  `.\bevyrrowind view "Seyda Neen, Census and Excise Office"` (also SH-16's Balmora spot). Backquote (key left of 1,
  `²` on AZERTY) opens/closes the console; F1 stays the A/B help.
- Type `off ssao` / `on ssao` (or `toggle render.ssao`, `render.ssao off`): room corners and under-table darkening
  go and come back the same frame, no hitch beyond one pipeline compile. `flip sun_shadows`, `off clouds`,
  `on wireframe`, `flip rings`: same as F2/Shift+C/F7/F8, overlay line updates. `off froxels` says "startup only,
  relaunch with --off froxels". `help on` lists the live names; Tab completes command and variable names. Keys don't reach the game while open.
- Scope: GI dropped (owner 2026-10-25: raster GI not built yet, Phase 4). SSAO off keeps the prepasses (image A/B,
  not cost; cost = a `--off ssao` run). Unblocks SH-16 (AO) re-check.
- Owner 2026-10-25: partial. COC works for interiors only (`coc balmora` fails: exterior lookup, console slice 3);
  can't paste; wants Down to select a suggestion and Enter to accept it. No upscaling/DLSS console switch exists yet.
- 2026-10-26 (console slice 3), re-check: `coc balmora`, `coc "seyda neen"`, `coc "bitter coast region"`, `coc -2 -9`
  fade to the exterior square's centre on the ground (no fall); `coc "seyda neen, census and excise office"` still
  goes inside. Type `re`: Down/Up move the `>` mark, Enter puts the candidate in the line (a second Enter runs it),
  Escape closes the popup first. Ctrl+V and Shift+Insert paste at the cursor (several lines join with `; `).
- Owner 2026-10-30: `off/on ssao` works, every coc (interior and exterior) works, Down/Enter suggestions and paste work.
  Status: confirmed.

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

### HD-1: Minimal HUD: bars, compass, crosshair, use prompt (`cargo dev view "Seyda Neen, Census and Excise Office"`, player mode)
- Bottom left: three thin framed bars, red health, blue magicka, green fatigue (full: the `player` record's 35/50/200),
  no labels, Morrowind-sized. Bottom right: heading (`N 000` ... `NW 315`) follows the mouse look; north matches the map.
- A small dot at the screen centre, in first and third person (Tab): what it covers is what E uses.
- Look at a door or NPC: `E Open <name>` / `E Talk <name>` under the dot; rebind Use (F10) and the glyph follows;
  a creature shows its name only, no key.
- Esc (pause), talk (dialogue), J (journal), inventory, the console: the HUD hides; it returns on close. F11 (free camera) hides it.
- Known: bars are static (no damage or regeneration yet); the compass is a heading label, no local map.
- Owner 2026-10-30: HUD fine. Status: confirmed.

### HD-2: Bars follow damage and regen (stats S5: the bars are the player's stat sheet)
- Build `cargo build -p bevyrrowind --features dev`; run `powershell -File bevyrrowind/tools/run_dev.ps1 scene level-arena`
  (F11 if the free camera is on). Synthetic: the player 3 m from a training dummy.
- Magicka bar: full at the player's real maximum (it was a static record copy); health and fatigue full.
- Shift tap (roll) or F then LMB (attacks): the green fatigue bar drops at each roll or swing, then refills smoothly a
  moment after; it never drops on its own while standing still, and refills no faster than before S5.
- Hit the dummy: the log reads `training dummy: hit ... health h/100` falling per hit; at 0 it refills to 100.
- Known: nothing hurts the player yet (the health bar's drop needs an attacker: C2 sparring dummy). Owner 2026-11-05: magicka at max, fatigue drops and refills, health bar still (as expected); dummy health "seems to go down", to re-confirm. Status: confirmed except the dummy log line.

### MM-1: Release start: main menu over Seyda Neen, New game (`bevyrrowind` with no arguments, release build)
- Build `cargo build --release -p bevyrrowind`, run `target/release/bevyrrowind` (no subcommand). The world streams in at
  Seyda Neen, just outside the Census and Excise Office (the `CharGen Exit Door` marker), with the main menu over it.
- Entries: Continue (greyed), New Game (focused), Load Game (greyed), Options, Quit. No HUD; NPCs, sky and physics
  stand still; Esc does nothing (no pause menu under it).
- Options opens the options screen on top, Esc returns. Quit asks Yes/No (No focused); Yes exits.
- New Game closes the menu: HUD appears, the player stands at the office door and can walk; Esc now opens the pause menu.
- Dev runs (`cargo dev view ...`, `scene`) never show it. Known: no title image or background art yet.
- 2026-10-26 (M3): with a save on disk, Continue (focused) and Load Game are live: Continue loads the newest save (any kind) and play resumes there; Load Game opens the save list, Esc returns to the main menu.
- Owner 2026-10-30: the buttons work, but a menu over the live world is not a main menu: Morrowind shows a title
  image or video, not the game. Wanted: background art/video behind the menu; "Main menu" from the pause menu.
  Status: rejected (menu over title art, not the world).

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
