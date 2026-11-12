# UI overhaul mods: what to take (R5b, 2026-11-11)

Research only. Sources: GitHub pages/API for both repos (fetched 2026-11-11), Nexus/Steam search snippets for SkyUI.
Per-screen detail for SkyUI is from its well-known v5 behaviour plus search results; the Community repo README
documents almost nothing. Items marked (unverified) come from memory, not a fetched page.

## 1. The two named mods

### doodlum/SkyUI-Community
- **Target:** Skyrim Special Edition. Flash/Scaleform SWF + ActionScript + Papyrus + SKSE64, built with CMake and JPEXS.
  Not engine-portable: our UI is Bevy UI, so nothing can be reused as code.
- **Status:** created 2026-02-16, last push 2026-06-03, 161 commits, 19 issues, 16 open PRs, no releases. Community
  maintenance fork of SkyUI 5 ("6.0+ modern improvements and fixes").
- **Licence:** none declared on the repo (all rights reserved by default). Original SkyUI is also not a clear open
  licence. **Inspiration only; port nothing.**
- **Layouts (SkyUI 5 lineage; the README gives no detail):**
  - Inventory/container/barter/gift: one wide item list with a left **category column** (icons, selectable icon
    themes), a **search box** (full text) and a **gear button** toggling extra columns (weight, value, damage/armor,
    value/weight). Columns are click-to-sort, click again to reverse. Right side: 3D item preview plus an item card
    (stat lines with compare-to-equipped arrows). Equipped and favourite items are flagged by an icon in the row.
    Favourites live in their own tab (without sorting, a known complaint).
  - Magic: the same list pattern, per school, with columns for cost, school level, effect summary.
  - Map: full-text location search and highlight of discovered places (unverified detail beyond snippet).
  - Crafting (smithing/enchant/alchemy): the same list with sort/filter (v5).
  - MCM (Mod Configuration Menu): two-pane page. Left list of mods, right a page of rows: toggle, slider, menu
    (stepper), text/keymap entry, header, empty spacer; a one-line info text at the bottom explains the hovered row;
    pages per mod. Other mods register pages through Papyrus; this is what made it a de facto standard.
  - Gamepad: weak in vanilla SkyUI (list navigation works, search box and columns are mouse-first). This is the
    gap Modern Morrow targets.
- **Why better than vanilla:** density (many rows at once), sort + search + filter that vanilla lacked, readable
  columns, and a settings framework other mods share.

### DurhamG/modern-morrow
- **Target:** a fork of the **OpenMW C++ engine** (not a Lua mod; the author intends to convert once the Lua UI API
  can replace the screens). New screens are drawn in code over OpenMW's hidden windows, which still run the game
  logic. "Controller-first interface for OpenMW"; gamepad, TV, handheld, Steam Deck.
- **Status:** created 2026-03-03, pushed 2026-11-11 (active). README says it is experimental, written by Claude under
  the author's direction, unreviewed by OpenMW devs, saves incompatible with vanilla OpenMW.
- **Licence:** **GPL-3.0** (bundled fonts: DejaVu licence and two SIL OFL fonts). Compatible with our
  GPL-3.0-or-later, but OpenMW's own code is GPL-3 and the fork changes its engine classes: layouts and flows are
  freely adoptable; code only with a THIRD_PARTY_NOTICES entry (file + commit) and only if it is not
  engine-coupled (most of it is). Rewriting from the described behaviour is cleaner.
- **Layouts:**
  - Inventory: opens to an **equipment overview, one card per body area**; drilling into an area (chest: cuirass,
    shirt, robe) lists candidates with stat deltas vs the equipped item, **colour-coded better/worse**.
  - Container/loot: list of contents, with upgrade and stolen markers.
  - Barter: buy list and separate sell tab; "better than your gear" marker; camera faces the NPC.
  - Magic: Active tab (active effects, equipped magic, recently used) and a Spells tab **grouped by school**,
    selecting equips.
  - Map: 3D local map, right stick pans, d-pad zooms, indoor ceilings cut away, X drops a pin at the crosshair.
  - Journal: quest list grouped **pinned / active / finished** (not a book).
  - Dialogue: larger text, **unseen topics first, seen greyed**, services (barter, training) on a separate tab.
  - HUD: **auto-hides** unless in combat or a bar is not full; no minimap; permanent effects not shown.
  - Others: alchemy puts matching ingredients first; enchanting/spellmaking show creation and choices side by
    side with live cost; quick-keys screen with d-pad slots (tap vs hold); rest/level-up, book, save/load,
    on-screen keyboard for character creation, persuasion, repair, options, large-text messages.
  - Gamepad: the design centre. D-pad quick keys, glyphs switch to Xbox names when a pad is active, SDL3 for
    Steam Controller. Extras: rolling quicksaves, 5-minute autosave, auto-continue newest save.
- **Why better than vanilla:** nothing needs the mouse; every screen is one focus column with big text; decisions
  (equip, buy, learn) show their delta in place.

## 2. Others worth a look (2-5)
| Mod | Engine | Licence | Note |
|---|---|---|---|
| Inventory Extender / Magic Window Extender / Stats Window Extender (OpenMW Lua, Nexus) | OpenMW Lua | per-mod, Nexus (check; likely permissive/GPL, unverified) | Adds search, filter and sort tabs to the vanilla windows, removes the paper doll in favour of list space. The nearest thing to a "classic-but-better" layout; compatible with our classic preset idea. |
| MWSE inventory helpers ("UI Expansion"-style: search/filter, hotkey equip) (unverified exact names) | MWSE (Lua on Morrowind.exe) | Nexus, mostly permissive | Search box and category filters on inventory/barter/spell lists; the origin of the "search first" habit in Morrowind. |
| S3ctor's UI Modes (slim) | OpenMW Lua | see repo (unverified) | Streamlined menu interactions: fewer clicks to open and close modes; useful for flow, not layout. |
| SkyUI 5 original (Nexus) | Skyrim LE/SE | custom, free for use | The reference sortable-column list; see section 1. |
| Contrast: Elden Ring / BG3 | proprietary | n/a | ER: icon grid + right detail pane with compare arrows; BG3: grid inventory with tooltips and full gamepad radial/hotbar. One line each, no further study. |

## 3. Common patterns
1. **List first, with sort and filter.** Dense row lists with category tabs/icons, click-to-sort columns, a search box.
   (SkyUI, Inventory Extender, MWSE helpers.) Modern Morrow keeps rows but drops columns for gamepad.
2. **Detail/compare card.** Hover or focus shows a card with stats and a delta against what is equipped, colour-coded
   (SkyUI arrows, Modern Morrow green/red).
3. **Equipment as slots/cards** rather than a paper doll (Modern Morrow, Elden Ring, Inventory Extender).
4. **Settings as a registry-driven page** with a per-row hover description and per-mod pages (SkyUI MCM). Matches our
   Options v2 (description pane, mod pages); the missing parts are search and the hover-info line being always visible.
5. **Context-reduced HUD**: minimal, shown on demand (Modern Morrow auto-hide).
6. **Journal as a task list** with pin/active/done instead of a book.
7. **Dialogue read-state**: unseen topics emphasised, seen dimmed; services split from talk.
8. **Controller-first focus model**: one focus column, tab pages on shoulder buttons, glyph footer, quick slots on d-pad.
   The mouse is an accelerator, not the model.

## 4. What our modern preset should adopt, per screen
Framework-level (data-driven; no Morrowind code in `superfargoth/`). Each item has a source.
- **Inventory / container / trade (`inventory.rs`, `container.rs`, `trade.rs`, `service_list.rs`):** one shared item
  list widget: category tab row with icons (SkyUI), search box (SkyUI, MWSE helpers), click/pad-cycled sort,
  optional columns toggle (weight, value, key stat; SkyUI), right-hand detail card with delta vs equipped, colour
  coded (Modern Morrow, Elden Ring). Flags in row: equipped, stolen, upgrade (Modern Morrow). Equipment overview
  as slot cards that drill into a candidate list (Modern Morrow). Trade: buy and sell as tabs plus "better than
  worn" marker (Modern Morrow).
- **Magic (if present in packs):** group by school/category with an Active tab (Modern Morrow); select equips.
- **Journal (`journal.rs`):** pinned/active/finished grouping with a detail pane (Modern Morrow); keep the book view
  as the classic layout only.
- **Dialogue (`dialogue.rs`):** topic read-state (unseen first, seen dim) and a separate services tab (Modern Morrow).
- **HUD (`hud.rs`):** auto-hide bars unless in combat or not full (Modern Morrow); make it a setting so classic stays
  always-on.
- **Options (`options.rs`):** keep the v2 plan; add the search box now rather than later (SkyUI MCM, keywords already
  in data), keep the always-visible one-line description (MCM), per-mod pages (MCM).
- **Keybinds (`keybinds.rs`):** glyphs switch with last-used device (Modern Morrow); quick-slot assignment screen
  for d-pad slots with tap versus hold (Modern Morrow).
- **Save/load (`saveload.rs`):** rolling quicksave, timed autosave and an auto-continue option (Modern Morrow); a
  settings-level decision, owner to confirm.
- **Main/pause menus:** one focus column, big text, footer glyph bar (Modern Morrow); nothing needing a mouse.
- **Map:** search + highlight (SkyUI) and cursor-placed pins (Modern Morrow); local-map 3D is Morrowind/pack scope.

## 5. Notes for the owner
- Neither named repo is a portable code source. SkyUI-Community has no licence; Modern Morrow is GPL-3 but
  engine-coupled and openly AI-written. Treat both as layout references; re-implement.
- Modern Morrow is the closest peer (same goal, same engine content, active this week). Worth re-checking its
  README before each UI slice; its screens list is a ready checklist for the gamepad pass.
- Not verified: SkyUI-Community's actual changes (README empty), exact licences of the OpenMW Lua UI extenders.
