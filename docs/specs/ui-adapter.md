# UI adapter (Morrowind → sf_ui)

Status: draft (2026-10-11), design only; built at M5 with the framework UI. Names of data files
were read from the installed BSAs; formats marked "verify" must be checked on data first.
Crates (proposed): `mw_ui` (Bevy-free: book markup, MessageBox formatting, topic linking,
`.fnt` reader), a `ui.frames` step in `mw_bake`, the classic theme as files in Bevyrrowind's
built-in mod, `mw_script` (MessageBox lowering), `mw_openmw_lua` (later). Framework spec:
`../../../superfargoth/docs/specs/ui.md`.

## Goal

Give the framework UI what it needs to feel like Morrowind without copying 2002 layouts: the
**classic theme** (parchment, bronze frames, original menu art, a Magic Cards-style face),
book and scroll rendering from BOOK markup, dialogue topic links, MWScript message boxes and
menu gating, and a later compatibility shim for OpenMW-Lua UI mods. The framework's modern
theme and both layout presets work unchanged (rule 6: not a 1:1 port). GPL reuse from OpenMW
is allowed here; the framework never sees Morrowind names.

## Themes

- `mw_classic` (default for Bevyrrowind) and the framework's modern minimal theme, both in the
  theme picker; both presets (PC-dense, controller-first) for each.
- `mw_classic` = a `ui.theme` record + `ui/themes/mw_classic/theme.css` (tokens as CSS
  variables: parchment and dark-wood surfaces, bronze borders, the vanilla text colors from
  GMSTs where they exist, e.g. the `fontColor_color_*` family; verify) + generated frame CSS
  from the bake step below. Everything stays CSS-overridable by mods in load order.

## Fonts: bitmap `.fnt` → TrueType

- Vanilla ships `Fonts/Magic_Cards_Regular.fnt`, `century_gothic_font_regular.fnt`,
  `century_gothic_big.fnt`, `daedric_font.fnt`, each with a `*_0_Lod_A.tex` glyph image.
  They are 8-bit, codepage-bound bitmaps (Win-1250/1251/1252 per localization) that blur
  when scaled. `bevy_text` loads TTF/OTF only and re-rasterizes per size, so the industry
  answer is **vector fonts with fallback chains, not converted bitmaps** (OpenMW reached the
  same conclusion and recommends TrueType replacements; openmw.readthedocs.io "Fonts").
- Defaults (bundled in the built-in mod, all user-replaceable through the VFS):
  - Body/headers: **Pelagiad** by Isak Larborn (SIL OFL; the community Magic Cards
    replacement, isaskar.github.io/Pelagiad), falling back to a readable serif and then the
    system UI font (`FontSource::Families`), so non-Latin scripts still render.
  - Daedric: **Ayembedt** by Georg Duffner (OpenMW's suggestion; **license to verify** before
    bundling; else trace `daedric_font.fnt`, below).
  - Century Gothic (stats, numbers): an OFL geometric sans; candidates to pick by screenshot.
  - The modern theme uses an OFL humanist sans, with the XAG sans-serif option in both.
- Optional "authentic glyphs" bake step (later, not M5): read `.fnt` + `.tex` (format: verify;
  OpenMW's font loader documents it and is GPL-reusable here), trace glyphs to outlines, write
  an OTF into the bake store. Only if players ask; never needed for correct rendering.
- No SDF: screen-space text is re-rasterized per size; world labels are screen-space nodes.
- Text encoding: ESM strings must be decoded with the install's codepage before they reach
  Fluent/parley (an `mw_records` setting, not implemented yet).

## Menu art for `mw_classic`

From `Morrowind.bsa` (checked by listing the archive):

| Use | Textures |
|---|---|
| Window frames (8-piece: 4 corners + 4 tiled edges) | `menu_thick_border_*`, `menu_thin_border_*`, `menu_head_block_*` (+ `_middle`), `menu_button_frame_*`, `menu_icon_frame_*`, `menu_rightbuttonup_*` / `menu_rightbuttondown_*` |
| Scrollbars | `menu_scroll_*` (arrows, elevator, bars, scroller pieces), `menu_scrollbar` |
| Bars | `menu_bar_{red,blue,green,yellow,gray}`, `menu_small_energy_bar_*` |
| Main menu buttons | `menu_{newgame,loadgame,savegame,options,credits,exitgame,return}{,_over,_pressed}`, title `menu_morrowind` |
| Journal and books | `tx_menubook` + `tx_menubook_{next,prev,close,take,journal,topics,quests,...}_{idle,over,pressed}`, `tx_menubook_bookmark`; scrolls `scroll`, `scroll_0[1-3]`, `scroll_open_0[1-3]`, `scroll_take`, `scroll_close`, `scroll_fleur` |
| Icons and markers | `menu_icon_{barter,equip,magic,...}`, `menu_map_*`, `compass`, `menu_compass_large`, `menu_on_dot`/`menu_off_dot`, `menu_number_inc/dec`, `menu_divider` |
| Other | `tx_cursor`, `tx_menu_*` fills/bevels, `levelup\*.dds` class art, `bookart\*.dds`, loose `Splash\` loading screens |

- Bevy's nine-slice (`TextureSlicer`) takes one image; Morrowind frames are separate pieces.
  A **`ui.frames` bake step** packs each 8-piece set (plus an optional center) into one image
  with slice insets, edges marked `SliceScaleMode::Tile` as the original tiles them, and emits
  `frames.css` (`background-image` + `-bevy-image-mode: sliced(..)`). Content-hashed, so an HD
  menu texture pack in the VFS is picked up on the next bake. Linear filtering and mips (the
  2002 art is small; at 200 % it must not go nearest-neighbour as in OpenMW #8221).
- `_idle/_over/_pressed` variants map to `:hover`, `:active` and `:focus-visible` rules, so
  gamepad focus shows the "over" art.
- Art is borrowed, layouts are not: the classic look applies to the framework's preset layouts.

## Books and scrolls: markup → rich text

- BOOK text is simplified HTML (verify the tag set over every BOOK record): `<DIV ALIGN=...>`,
  `<FONT COLOR=... SIZE=... FACE=...>`, `<BR>`, `<P>`, `<IMG SRC=... WIDTH=... HEIGHT=...>`
  (images under `bookart\`); the original engine ignores unknown tags. OpenMW adds
  `<BODY BACKGROUND>` (MR 2064); support it so OpenMW-era books work.
- `mw_ui::book` parses with a tolerant tag tokenizer (no HTML engine; OpenMW's parser may be
  reused, GPL) into an IR: blocks (alignment), runs (color, face → theme family, size → `em`
  step), images (block or inline), explicit breaks. Unknown tags dropped and counted.
- Bevy side: one `Text` per paragraph, `TextSpan` per run, `InlineImage` (an `InlineBox`) for
  images. Pagination for the two-page book (`tx_menubook`) measures with parley at the current
  scale, re-paginates on scale or font change, and keeps the reading position as a character
  offset. Scrolls are one column. The controller preset shows books as a single page with
  page-turn on shoulder buttons.

## Dialogue and journal

- Topic links: INFO text is linked against the topics the player knows (case-insensitive,
  longest match first; Aho-Corasick, `aho-corasick` crate, MIT/Unlicense), plus explicit
  `@topic#` markers (none in Morrowind.esm dialogue, counted 2026-10-21). Each link is a `TextSpan` (framework spec,
  "Dialogue hyperlinks"). Text macros (`%PCName`, `%PCRace`, ...) are expanded by `mw_rpg` first (`rpg-adapter.md`).
- `Choice` in dialogue results renders as choice buttons in the dialogue screen.
- Journal uses `tx_menubook` in `mw_classic`; quests, topics index and search come from the
  framework's journal screen.

## MWScript UI calls

- `MessageBox "fmt" [vars...] ["button"...]`: printf-style `%g`/`%f`/`%.Nf` substitution
  (verify the accepted specifiers). No buttons → a timed notification (duration from text
  length, GMST-driven; verify which). Buttons → a modal on the Modal layer; the script polls
  `GetButtonPressed` (−1 until chosen, then the index once). The transpiler lowers this to the
  `mw` library calling `ui.message` / `ui.message_box{ text, buttons }` with a per-script
  pending-choice slot that is saved with script state.
- `MenuMode` → "a menu-layer screen is active". `EnableInventoryMenu`, `EnableMagicMenu`,
  `EnableMapMenu`, `EnableStatsMenu`, `EnableRest` → screen availability flags in a view-model
  (character generation gating). `EnableNameMenu`, `EnableRaceMenu`, `EnableClassMenu`,
  `EnableBirthMenu`, `EnableStatReviewMenu` → content-pack chargen screens. `ShowMap`,
  `FadeIn/FadeOut` → map reveal and the framework's fade overlay.

## OpenMW-Lua UI compatibility (`mw_openmw_lua`, after the core game)

Treat OpenMW's Lua docs as the API spec:

- `openmw.ui` layout tables (`type`, `props`, `events`, `content`, `name`, `layer`,
  `template`, `external.slot`) → elements built from framework widgets. `ui.create` returns an
  element; `element:update()` diffs the table per node, keyed by `name`, into fine-grained
  setters, which avoids OpenMW's whole-tree re-parse (#9291) but can't make per-frame rebuilds
  free; document that.
- Widget types (`Widget`, `Text`, `TextEdit`, `Image`, `Flex`, `Container`) → nodes, text,
  text input, `ImageNode` (`ui.texture{ path, offset, size }` → image rect), flexbox.
- Absolute props (`position`, `relativePosition`, `size`, `relativeSize`, `anchor`) →
  `PositionType::Absolute` with `Px`/`Percent`; `anchor` → `UiTransform` translation in
  percent of the node's own size.
- Layers `HUD`, `Windows`, `Notification`, `MessageBox` → Hud, Menu, Notification, Modal.
- `I.MWUI.templates` (boxes, borders, padding, text styles) → classes in `mw_classic`'s CSS
  plus small templates. `ui.showMessage` → notification. `I.UI.setMode/addMode/registerWindow`
  → screen push and replacement in the registry. `I.Settings.registerPage/registerGroup/
  registerRenderer` → the settings registry (custom renderers as Lua widgets in a slot).
- Events (`mouseClick`, `mouseMove`, `focusGain/Loss`, `keyPress`, `textInput`) → picking
  observers, `InputFocus`, `FocusedInput`. Callbacks run synchronously in the UI schedule slot
  (no one-frame lag, OpenMW #9233).

## Verification

- Parse every BOOK record: zero panics, unknown-tag counts reported, every `IMG` resolves in
  the VFS. Snapshot tests for a few books (alignment, colors, images).
- `ui.frames` bake test: every 8-piece set packs, insets match piece sizes.
- Screenshots: `mw_classic` inventory, a book, dialogue with links, the main menu, at 100 %
  and 200 % text, in both presets; compare against vanilla (black-box oracle) for mood only.
- A vanilla MessageBox with buttons (from a chargen or quest script) works by gamepad.

## Industry comparison

J9 (superfargoth `industry-audit.md`), 2026-10-12, docs only (`.sessions/j9rest.md`).
The UI stack is compared in superfargoth `ui.md`. Adapter-level: remasters
and source ports replace bitmap fonts with vector fonts (OpenMW recommends
TrueType replacements; the Skyrim/Fallout Scaleform era already used vector
fonts) and keep a "classic" skin next to a modern one (OpenMW's vanilla-like
skin plus community UI mods; SkyUI for Skyrim). Ours: TrueType with fallback
chains, classic + modern themes, book markup as rich text (**match**). No gap.

## Open questions

1. Ayembedt's license (bundle, or trace the vanilla Daedric `.fnt`).
2. Which OFL faces replace Century Gothic and serve the modern theme: pick by screenshot.
3. Whether `mw_classic` should also restyle the HUD with the vanilla bar and compass art or
   keep the framework HUD layout with classic colors only. Default: vanilla art, modern layout.
