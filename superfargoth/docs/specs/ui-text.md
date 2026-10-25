# Rich text and pagination (books, scrolls, notes)

Status: built (ROADMAP 2.3): core 2026-10-22 (slice 1, `sf_ui`), measurer, readers and notifications
2026-10-23 (slice 2, `sf_ui_bevy`). Next: the first content pack's markup parser and MessageBox mapping
(bevyrrowind `specs/ui-adapter.md` "Books and scrolls", "MWScript UI calls").
Split from [ui.md](ui.md) (over the spec size budget); ui.md's stack, themes and presets apply.

## Goal

Readable in-game documents from any game's markup: styled runs, alignment, inline and block images,
forced page breaks; shown as a two-page book spread, a single page (controller preset), or a scroll (one
long page). Pages are recomputed when text scale or fonts change and the reader keeps their place.
Non-goals: an HTML engine, editing, floats, tables, columns, hyphenation.

## Research (2026-10-22)

- **Fragmentation is the industry model.** CSS Fragmentation Level 3 (w3.org/TR/css-break-3): content
  breaks between line boxes under `orphans`/`widows` (CSS 2.1 initial value 2), forced breaks win over
  other restrictions, replaced elements (images) are monolithic, and a fragmentainer that can't satisfy
  the rules breaks anyway so content always advances. EPUB readers (Readium, Apple Books) paginate
  reflowable books this way and keep the reader's place as a content locator (EPUB CFI), not a page
  number, because page numbers change with font size. Haskell's `balkon` (`paginate` with orphan/widow
  options) is the same shape. TeX goes further (penalty-scored page breaks); too much for game books.
- **Line breaking stays in the text engine.** Bevy 0.20 lays text out with parley 0.11 (UAX #14 line
  breaking, shaping, bidi, inline boxes); its `Layout` exposes lines with text ranges and line metrics.
  Pagination that re-implemented line breaking would disagree with what is drawn, so the paginator asks
  the engine for lines and only breaks pages.
- **Inline objects in text** are a placeholder character in the attributed string (Apple
  `NSTextAttachment` uses U+FFFC, the Unicode object replacement character). parley's `InlineBox`
  instead takes no bytes (an index into the text); the Bevy measurer maps between the two (below).
- **Reuse (rule 8).** Bevy: `bevy_text` lays out one text block; no fragmentation. Crates (2026-10-22):
  parley, cosmic-text: lines, no pages; typst (Apache-2.0) paginates but is a whole typesetting compiler;
  no maintained crate breaks pages over an external line layout. So our own, ~400 lines.
- **Fit check against two game markups.** The first content pack's book tags (`DIV ALIGN`, `FONT COLOR
  SIZE FACE`, `BR`, `P`, `IMG SRC WIDTH HEIGHT`, `BODY BACKGROUND`) and the Creation Kit's book markup
  (`<p align>`, `<font face size color>`, `<b> <i> <u>`, `<img>`, a page-break token; ck.uesp.net/wiki/Book,
  from memory, not re-read) both map onto the model below with an adapter only (litmus test, rule 1).

## Model (`sf_ui::rich_text`)

- `RichDoc { blocks, background: Option<ImageRef> }`; `Block::{Paragraph, Image { image, align },
  PageBreak }`.
- `Paragraph { align, inlines }`, `Inline::{Text { text, style }, Image(ImageRef)}`; `'\n'` in text is a
  hard line break (`BR`). `push_text` merges a run into the previous one when the style is equal.
- `Align::{Start, Center, End, Justify}` (direction-relative; `LEFT` → `Start` in LTR).
- `RunStyle { family: Option<String>, size: f32 (em), color: Option<Rgba8>, bold, italic, underline }`.
  `family` is a **theme font key** (a CSS family-list name the theme maps to files), `None` = body face; a
  game's face names and size steps map to keys and `em` in its adapter. `Rgba8::from_hex` (`RRGGBB[AA]`).
- `ImageRef { source, width, height }`: source is a content path the adapter resolves; sizes in document
  units, either may be `None` (native size or aspect ratio, known to the measurer).
- **Positions**: byte offsets into a paragraph's **flattened text** (`Paragraph::text()`: runs in order,
  one U+FFFC per inline image, so every image has its own offset). `Paragraph::slice(range)` returns the
  fragment to draw (runs clipped, images kept when their placeholder starts inside).
  `RichDoc::plain_text()` is the text for screen readers (one line per paragraph, images dropped).

## Paginator (`sf_ui::paginate`)

- `trait TextMeasure { fn lines(&mut self, &Paragraph, width) -> Vec<LineMetrics { range, height }>;
  fn image_size(&mut self, &ImageRef) -> Size }`. Lines are contiguous and cover the flattened text; an
  empty paragraph is one blank line. All scaling (UI scale, text scale) lives in the measurer: the
  paginator works in its units.
- `PageFormat { width, height: Option<f32> (None = scroll), block_gap, orphans, widows }`;
  `PageFormat::page(w, h)` (orphans = widows = 2, the CSS default), `PageFormat::scroll(w)`.
- `Pagination::paginate(&doc, &mut measure, format)`, greedy, one pass:
  paragraphs split between lines; `k` lines stay when `k ≥ orphans` and `n − k ≥ widows` (else fewer, or
  the whole paragraph moves); a fresh page that can't satisfy the rules fills anyway (≥ 1 line). Block
  images are scaled down to the page (aspect kept) and move to the next page whole. A forced break ends
  the page, never making a blank one (a break on an empty page is a no-op); scrolls ignore breaks. A
  gap is dropped at a page top. A single line taller than the page goes alone (the renderer clips).
- Output: `Page { start: DocPos, items: Vec<Placed { block, y, height, fragment }> }`,
  `Fragment::{Lines { range, lines }, Image { size }}`. Always ≥ 1 page.
- **Views**: `PageView::{Spread, Single}`; `view_count`, `view(i)` (pages `2i, 2i+1`; the last spread may
  be one left page), `view_of(pos)`.
- **Reading position** = `DocPos { block, offset }` (ordered). Keep the first page's `start` of the open
  view as the bookmark; after re-pagination `page_of(bookmark)` opens the page that draws it.

## Bevy side (`sf_ui_bevy`, `UiTextPlugin`)

- **Measurer** (`text_measure.rs`, `ParleyMeasure` from the `RichTextMeasure` system param): lays a
  paragraph out with `bevy_text::TextPipeline::update_buffer`, the call `bevy_ui` makes for every `Text`
  node, with the same `TextFont`s, line height, `LineBreak::WordOrCharacter` (long words break, never
  overflow) and in-flow `InlineBox`es the reader spawns. Units: **physical pixels** at the page node's
  scale factor (`1 / ComputedNode::inverse_scale_factor`); nodes are placed at `y / scale`.
  parley quirks handled: a layout's height is Σ `line_height` (block coordinates overlap); a line of
  only inline boxes reports an inverted sentinel range; a final hard break (or an overflowing trailing
  space) adds an empty line, dropped. Images: `ImageRef` size (doc units) or the image's pixels, ×
  `image_scale` × text scale. `MeasureCache`: exact entries per (paragraph, width, scale), cleared on a
  theme or font change. Bevy 0.20 has no cosmic-text: parley is the engine.
- **Theme** (`RichTextTheme` resource): body face, theme font keys → `FontSource`, body size, line
  height, colour, block gap, image scale. Fonts and sizes live here, not in CSS on the text nodes: the
  measurer must see what is drawn. CSS (`sf_ui_css`) styles the chrome: every part has a `Name`
  (`#sf-reader-page`, `#sf-reader-page-body`, `#sf-reader-prev`, `#sf-message-box-button`, `#sf-toast`,
  …) and a `UiPart` (mapping it to a class is a one-line `sf_ui_css` follow-up).
- **Readers** (`RichReader { doc, layout: Book | Scroll, view, takeable, bookmark }` on a node): chrome
  = pages row (doc background) → page → page body; prev/next/(take)/close buttons. It paginates **at
  the laid-out page body's content size**, like a browser fragmentainer, so a theme sizes pages and
  text never clips; re-paginates when that size, scale, text scale, theme or view changes (`ReaderState`
  keeps what it was made for) and reopens the bookmark's view. Images wait for their assets. Book view:
  `PageView::Single` in the `controller` preset, else a spread (or `view`). Scroll: one page in an
  `Overflow::scroll_y` viewport, bookmark = first visible item.
- **Fragments**: one `Text` per `Lines` fragment (`TextSpan` per run, `InlineImage` per image,
  `justify` from `align`), absolute at `y`, height fixed to the fragment's. A paragraph that goes on
  draws its **next line, to the end of its word, transparent** (`ReaderTail`): the last glyph keeps its
  kerning against the next one (else it no longer fits) and greedy breaking sees the whole next word;
  the page's last line is justified, as in CSS fragmentation. Block images: `ImageNode`, aligned.
- **Events** (`ReaderEvent { reader, action }`): `Prev`/`Next` turn pages (scroll: 90 % of the
  viewport), `Scroll(px)`, `Take`/`Close` for the game. From the buttons (`Activate`), the reader's
  screen's UI actions (`TabPrev`/`TabNext` = shoulders, `Context1` take, `Back` close) and the wheel
  (input target's scroll readers, 40 px a line). Prev/next get `InteractionDisabled` at the ends.

## Notifications

Research (2026-10-23): Android `Toast` has two lengths (2 s and 3.5 s by framework convention), Material
snackbars short/long/indefinite (developer.android.com, material-components-android docs); WCAG 2.2.1
"Timing Adjustable" asks for user-extendable timing; subtitle rules of thumb give ~15-20 characters a
second. Morrowind's script `MessageBox` is modal with buttons or timed without (ui-adapter.md). Reuse
(rule 8): Bevy has neither; `bevy_toast` (Bevy 0.6), `bevy-notify` (egui, 2022), `bevy_ui_mod_alerts`
(pre-0.15 bundles) are stale or egui. So our own, Bevy-free logic in `sf_ui::notify`, drawn by `sf_ui_bevy`.

- **Message boxes** (`MessageBoxQueue`, resource `MessageBoxes`): FIFO, one shown; buttons are
  localized labels (none → one OK label from `MessageBoxConfig`); `choose(index)` → `MessageBoxChoice
  { id, index }`, sent as `MessageBoxAnswer`; `cancel` = the box's Back button (optional), `withdraw`.
  Drawn as a backdrop that is the root of the `sf.message_box` Modal screen (pushed while a box is up
  when the registry has that record), so the router gives it input and focus starts on button 0.
- **Toasts** (`ToastQueue`, resource `Toasts`): hold = clamp(1.5 s + 0.06 s × chars, 2.5 s, 10 s) ×
  `scale`; fade 0.15 s in, 0.5 s out (none with reduce motion); ≤ 4 on screen: when full the oldest is
  retired (fades out) and the new one shows as its slot frees, so nothing is lost and a burst never
  covers the screen; ≤ 32 waiting (oldest dropped); a repeat of a visible text restarts it as
  `text (xN)`. Real time (keeps counting in pause menus). One `#sf-toasts` column, oldest first.

## Co-op

Presentation only: no gameplay state. A document is content (records); the bookmark is per local
player and not replicated (net.md rule 3). Taking a book or reading a skill book is a gameplay command
elsewhere.

## Industry comparison

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Page breaking between lines with orphans/widows, forced breaks, monolithic images | CSS Fragmentation 3; EPUB reflowable pagination (Readium, Apple Books); print engines (TeX penalties) | greedy CSS-style rules | **match** (CSS level; TeX-quality scoring not needed) |
| Pagination over the drawing engine's line layout | browsers (layout tree fragments); UE `URichTextBlock` / Slate wraps once per width (books are scroll boxes in many UE games, unverified) | `TextMeasure` = parley lines | **match** |
| Reading position independent of page numbers | EPUB CFI | `DocPos` | **match** |
| Inline images as text attachments | `NSTextAttachment` (U+FFFC), Android `ImageSpan`, UE rich text decorators | U+FFFC in flattened text | **match** |
| Markup | game-specific (HTML-like subsets in Bethesda games, BBCode-like in others) | adapter per game into `RichDoc` | **match** (no shared markup on purpose) |
| Fragments measured in the real fragmentainer, shaped in context | browsers (layout in the page box; line fragments keep their paragraph's shaping and justification) | page body's laid-out size; invisible next-line tail | **match** |
| Toast timing and stacking | Android Toast (fixed lengths, one at a time), Material snackbar (one at a time) | reading-time hold, user scale, ≤ 4 stacked, repeat merge | **beyond** (WCAG-adjustable, bursts stay readable) |
| Modal message box with a result | Unreal CommonUI modal layer, UE `FMessageDialog` returns the choice | Modal-layer screen, choice as a message | **match** |

## Verification

`cargo test -p sf_ui` (`tests_text.rs`, fake monospace metrics with greedy word wrap, sizes, inline
images and a text scale): runs and slices, hex colours, scroll (one page, breaks ignored, gaps), page
fill, orphans/widows cases, forced breaks without blank pages, block-image fit and move, inline-image
and `em` line heights, a too-tall line alone, spreads and single views, bookmark across 100 % → 150 %,
and 40 random books × 4 formats checking that fragments rebuild every paragraph exactly, pages start in
reading order, and only a lone item overflows a page.

Slice 2: `cargo test -p sf_ui` (`tests_notify.rs`: box order, fallback button, cancel, withdraw;
toast durations, fades, the cap with early retirement, bounded queue, repeat merging, reduce motion) and
`cargo test -p sf_ui_bevy` (`tests_text.rs`, headless `bevy_ui` with a dummy camera target and
bevy_text's built-in font): 12 synthetic books (prose in 0.8-1.7 em and bold, all alignments, hard
breaks, blank paragraphs, 60-letter words, inline-image rows, block images, forced breaks) at UI × text
scale 100-200 %, checked view by view against `bevy_ui`'s own layout: line count and heights equal
the pagination, no line wider than the page, the tail starts exactly after the paginated lines, no
text past the page bottom, fragments tile each paragraph and draw as many glyphs as the paragraph laid
out alone; the same for 6 books in a proportional font when `DejaVuSans.ttf` is on the system; cache
reuse; page turns, disabled ends, bookmark across 100 → 150 % text; UI actions routed to the screen's
reader only; scroll; message box flow through the router; toast timing on real time.

## As built (slice 1, 2026-10-22)

As above. 13 tests; clippy pedantic clean; no new dependencies.

## As built (slice 2, 2026-10-23)

As above. `sf_ui` 7 tests, `sf_ui_bevy` 8 tests; clippy pedantic clean. New direct dependencies of
`sf_ui_bevy`, already in the tree through `bevy_ui` (same versions, no compile cost): `bevy_text`,
`parley` (to read line items), `bevy_asset`, `bevy_image`, `bevy_color`. Before landing, the matching
check also passed 200 monospace and 100 proportional books. Known limits: complex scripts shape the
first glyph of a fragment without left context; the right page's chrome is hidden, not removed, in
single view; the scroll bookmark is paragraph-level; gamepad stick scrolling is the app's binding.
