# Visual checks: player-ui

Open visual checks: player, inventory, saves, input, dialogue and journal, console (PL, IV, SV, IN, DJ, CO). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

## Player (ROADMAP 2.1; `crates/mw_player_bevy/src/lib.rs`)

### PL-1: Third-person walk and the use prompt (`cargo dev view "Seyda Neen"`, 10:00)
- Starts in player mode (capsule scaffold). WASD, Shift, Space jump, Tab 1st/3rd, wheel zoom; walk to the Census and
  Excise Office door, then to an NPC (Fargoth by the lighthouse path).
- Look for: smooth walk and camera (no jitter, camera pushes in against walls); under the centre dot the prompt reads
  `[E] Open  <door>` / `[E] Talk  <npc>` / `[E] Take  <item>`, it clears when you look away or step past ~2.7 m;
  E on the door travels, the body arrives at the marker facing out.
- Fixed 2026-10-25 (player3), check: focus is now a physics ray plus actor capsules, so in the Census and Excise Office
  the dot on an NPC's chest reads `[E] Talk  <npc>` (was `Take  Broom`); walls block items behind them.
  `cargo dev view "Seyda Neen, Census and Excise Office"` starts on the floor at the door's arrival marker, facing in
  (was: on the roof, grey void). E does nothing while the journal or dialogue is open.
- Status: awaiting owner.

### IV-1: Take items and Fargoth's ring (`cargo dev view "Seyda Neen, Census and Excise Office"`) (partial)
- E on a loose item (bottle, plate, book): it vanishes at once and stays gone after leaving and re-entering the
  office (overlay removal); log line `take: <id> (now N)`. Doors, NPCs and statics are unchanged.
- E on the barrel by the desk (`chargen barrel fatigue`): log `take all from ... [(ring_keley, 1), ...]`; the barrel is
  now solid (containers got collision). Then talk to Fargoth outside, topic `ring`, "Yes": quest Fargoth's Ring
  finishes (J shows it), the ring leaves your inventory. Shift+U (dev ring) still works.
- Known: no container window yet (the barrel gives everything), no theft for owned items, no weight.
- I (pad Y / Triangle, rebindable on F10's gameplay rows) opens the inventory: record names, counts, DDS icons, tabs All/Weapon/
  Apparel/Magic/Misc; after the barrel it lists Fargoth's Ring. I or Esc closes; you can't walk or press E while open.
  Look for: icons drawn (not blank squares). Read-only: no equip/drop yet. Shot: `--open-ui inventory` (dev sample items).
- Owner 2026-10-25: partial. Inventory opens; layout looks off, hover works poorly, no paper-doll view, no
  drag-to-floor drop. Queued after the physics work (layout, hover, drop, then paper doll).

### SV-1: Quicksave / quickload, save, quit, load (`cargo dev view "Seyda Neen, Census and Excise Office"`) (confirmed with follow-up)
- In player mode take the barrel by the desk (ring) and a loose bottle, step into the middle of the room, Tab to first
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

### SV-2: Save/load screens from the pause menu (`cargo dev view "Seyda Neen, Census and Excise Office"`, player mode) (awaiting owner)
- Esc, Save: the save screen over the paused game, "New save" on top. Confirm it (select, confirm again): toast
  "Saved"; the list gets a row titled "Seyda Neen, Census and Excise Office", subtitle "Day N - 16 Last Seed 9 a.m. ·
  Played ... · Saved <date> UTC" (OpenMW's fields).
- Select that row, confirm, Yes: overwritten (a `.bak` next to it). F5 once: a "Quicksave" row.
- Back: the pause menu. Load: the same saves, newest first; pick one: fade, the menus close, you stand at the save.
- Load screen, a save selected, Delete (or Context1), Yes: the row goes away.
- Known: no typed names (generated, not stored: sf/meta has no name field), no thumbnails, UTC dates.

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
- Status: awaiting owner.

### PM-1: Escape menu and options (`cargo dev view "Seyda Neen"`, player mode)
- Esc opens the pause menu (Resume, Save, Load, Options, Quit; Save/Load greyed out until the save screens are wired);
  the world stops (NPCs, sky clock, physics). Esc or Resume closes it and the world continues.
- Options: Audio page (Master/Music/Sfx/Voice/Ambience/UI volume and mute); left/right change a value and you hear it
  at once; holding an arrow repeats. "Key bindings" opens the F10 screen on top; Esc returns to options. Leave the
  menus, quit, restart: the volumes are kept (`profiles/<name>.toml`, `[settings.sf]`).
- Quit asks Yes/No (No is focused); Yes exits. Not in free camera or while the console is open.
- Known: frames are flat CSS (no 8-piece textures); screen titles and On/Off are English defaults.
- Status: awaiting owner.

### HD-1: Minimal HUD: bars, compass, crosshair, use prompt (`cargo dev view "Seyda Neen, Census and Excise Office"`, player mode)
- Bottom left: three thin framed bars, red health, blue magicka, green fatigue (full: the `player` record's 35/50/200),
  no labels, Morrowind-sized. Bottom right: heading (`N 000` ... `NW 315`) follows the mouse look; north matches the map.
- A small dot at the screen centre, in first and third person (Tab): what it covers is what E uses.
- Look at a door or NPC: `E Open <name>` / `E Talk <name>` under the dot; rebind Use (F10) and the glyph follows;
  a creature shows its name only, no key.
- Esc (pause), talk (dialogue), J (journal), inventory, the console: the HUD hides; it returns on close. F11 (free camera) hides it.
- Known: bars are static (no damage or regeneration yet); the compass is a heading label, no local map.
- Status: awaiting owner.

### MM-1: Release start: main menu over Seyda Neen, New game (`bevyrrowind` with no arguments, release build)
- Build `cargo build --release -p bevyrrowind`, run `target/release/bevyrrowind` (no subcommand). The world streams in at
  Seyda Neen, just outside the Census and Excise Office (the `CharGen Exit Door` marker), with the main menu over it.
- Entries: Continue (greyed), New Game (focused), Load Game (greyed), Options, Quit. No HUD; NPCs, sky and physics
  stand still; Esc does nothing (no pause menu under it).
- Options opens the options screen on top, Esc returns. Quit asks Yes/No (No focused); Yes exits.
- New Game closes the menu: HUD appears, the player stands at the office door and can walk; Esc now opens the pause menu.
- Dev runs (`cargo dev view ...`, `scene`) never show it. Known: no title image or background art yet.
- Status: awaiting owner.
