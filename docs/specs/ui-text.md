# Rich text and pagination (books, scrolls, notes)

Status: core built (2026-10-22, ROADMAP 2.3 books, slice 1). Crate: `sf_ui` (Bevy-free). Next: the Bevy
measurer and book screen in `sf_ui_bevy`; the first content pack's markup parser (bevyrrowind
`specs/ui-adapter.md` "Books and scrolls").
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

## Bevy side (next slice, `sf_ui_bevy`)

- `ParleyMeasure`: builds a parley layout per paragraph with the theme's resolved fonts at the current
  scale, text without the U+FFFC placeholders and an `InlineBox` per image at the mapped index; maps
  each line's range back to flattened offsets (a line holding an inline box includes its placeholder,
  found through the line's `PositionedLayoutItem::InlineBox` ids). Line heights from parley's metrics.
- Book screen: per page, one `Text` per `Lines` fragment (`TextSpan` per run, `InlineBox` + image node
  per inline image, `TextLayout` justify from `align`), an image node per block image; pages laid out
  absolutely from `y`. Re-paginate on `UiSettings` scale, theme or font change, keep the bookmark.
  Spread in the PC preset, single page with shoulder-button page turns in the controller preset.
- Measure only the open book (books are small: the longest vanilla-style books are tens of KB);
  cache the `Pagination` per (doc, format, scale).

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

## Verification

`cargo test -p sf_ui` (`tests_text.rs`, fake monospace metrics with greedy word wrap, sizes, inline
images and a text scale): runs and slices, hex colours, scroll (one page, breaks ignored, gaps), page
fill, orphans/widows cases, forced breaks without blank pages, block-image fit and move, inline-image
and `em` line heights, a too-tall line alone, spreads and single views, bookmark across 100 % → 150 %,
and 40 random books × 4 formats checking that fragments rebuild every paragraph exactly, pages start in
reading order, and only a lone item overflows a page.

## As built (slice 1, 2026-10-22)

As above. 13 tests; clippy pedantic clean; no new dependencies.
