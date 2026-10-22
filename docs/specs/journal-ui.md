# Journal screen logic (`sf_rpg_ui`)

Status: **implemented** (journal-ui slice 1, 2026-10-22). Logic only, no rendering: the Bevy-free
view-model a journal screen binds to. Data comes from `sf_rpg` (rpg.md "Quests", "Journal", known
topics); widgets, styling and input are `sf_ui_screens` (ui.md). Roadmap: Phase 2.4 / M5 screens.

## Goal

What every RPG journal screen needs, as data and pure functions: the dated journal paged like a
book (one or two pages per spread), quests split into active and finished, a topic index with
letter tabs, full-text search with highlight ranges, and "new" markers that clear when the player
has seen the thing. No text layout here (that needs fonts); a measuring hook stands in for it.

## Research (2026-10-22)

- **Journal shapes.** Morrowind: a book; dated entries flow across two pages; tabs for quests
  (active / all) and a topic index with A–Z letter tabs, each topic page listing what NPCs said.
  Oblivion / Skyrim: quest list split active / completed, entries per quest, newest first.
  The Witcher 3, Baldur's Gate 3, Pillars of Eternity: quest list with "new" markers that clear
  on view; Pillars and BG3 add journal search. Common model: entries are an append-only log,
  quests and topics are filtered views of it, "new" is per-player seen state.
- **Pagination.** Paged text is CSS Fragmentation Level 3 (W3C): lines flow into fixed-height
  fragments; a block may split between lines; `orphans` / `widows` (default 2) bound how few
  lines a split leaves at a page bottom / top; headings carry `break-after: avoid` so they never
  end a page. Line breaking itself comes from the text engine (Bevy 0.20 lays text out with
  parley, whose layout exposes per-line text ranges).
- **Search.** Case-insensitive substring per term, all terms required (AND), highlights for
  every occurrence. Folding by `char::to_lowercase`, the same rule as `sf_ui::ListFilter`.

## Reuse (Bevy → crates → own)

1. **Bevy** has no pagination or journal; text layout (parley) is the measuring side, Bevy-only.
2. **Crates.** `sf_ui` (ours, Bevy-free) already has `ListModel` (keyed rows, sort, search over
   fields, exact-match tab filters, windows) and `ViewModel`: the quest list and the topic index
   are `ListModel`s, letter tabs are an `equals` filter. No `sf_ui` paginator exists yet (no
   branch or PR as of 2026-10-22), so pagination sits behind a `Paginator` trait here.
   `textwrap` (MIT) wraps by character width, returns strings rather than byte ranges and knows no
   pages; real wrapping comes from parley on the Bevy side, so the fixed-width test measure is ~40
   lines of our own instead.
3. **Own:** the line paginator (fragmentation rules above), folding search, seen state, the screen
   state machine.

## Data model

All in `sf_rpg_ui`; deps `sf_rpg`, `sf_ui`, `serde`.

- **`JournalLabels`** (adapter hook): `quest_name`, `topic_name` (fallback: the id),
  `day_heading(day)` (the calendar is the adapter's), `entry_text(&JournalEntry)` (re-rendered
  localised text; default `None` = the snapshot, rpg.md open question 2).
- **Book.** `Block { key: BlockKey, style: BlockStyle, text, new, highlights }`; `BlockStyle` is
  `Title | Heading | Body`; headings and titles keep with the next block. `BlockKey` is a stable
  anchor (`Title`, `DayBefore(seq)`, `Entry(seq)`, `Speaker(i)`, `Heard(i)`). `Page { slices }`,
  `PageSlice { block, range }` (byte range of the block's text; `range.start == 0` starts the
  block, `range.end == len` ends it).
- **`Paginator`** trait: `paginate(&[Block]) -> Vec<Page>`. Stock implementation
  `LinePaginator<M: TextMeasure>`: `TextMeasure::lines(text, style)` returns the visual lines as
  contiguous byte ranges covering the text (parley's line ranges), `line_height(style)`;
  `PageMetrics { height, block_gap, orphans, widows }`. Rules: a block that fits stays whole;
  else it splits at a line so the page keeps ≥ `orphans` lines and the next starts with
  ≥ `widows` (else the whole block moves on); a heading moves on unless the first `orphans`
  lines of its next block fit after it; an empty page always takes at least one line
  (progress on tiny pages). `FixedWidthMeasure` (greedy word wrap at N chars) for tests and
  headless use.
- **Quests.** `QuestRow { id, name, finished, stage, entries, last_seq, last_day, new }`
  (`ViewModel` kind `journal.quest`), one per quest with journal entries, in first-entry order;
  status from `QuestLog`. `QuestFilter::{Active, Finished, All}` is an `equals("finished")`
  filter; `QuestSort::{Name, Recent}`.
- **Topics.** `TopicRow { id, name, initial, learned, heard, new }` (kind `journal.topic`),
  sorted by name. `Alphabet`: ordered tab labels (default Latin A–Z) plus aliases (`Ä → A`);
  `initial` is the tab of the first character, uppercased, or `#`. `tabs()` gives every
  label with its topic count and a new flag (empty tabs show disabled, as in Morrowind).
- **Search.** `SearchQuery::parse` (whitespace terms, folded). Hits: entries (with highlight
  ranges, shown as a book), quests by name, topics by name or heard text.
- **`JournalSeen`** (serde, per player): entries seen as a watermark plus the set seen above it;
  topics as heard-line counts seen. An entry is new until a page showing it is marked seen; a
  quest is new while it has an unseen entry; a topic is new until opened and again when a new
  line is heard under it. `mark_all_seen` for "mark all read".
- **`JournalScreen`** (state machine): content `Journal | Quest(id) | Topic(id) | Search`,
  `pages_per_spread` (2 book, 1 controller preset), spread navigation, quest filter / sort,
  letter tab, topic-name filter, search text. `refresh(ctx)` rebuilds rows and pages after
  `RpgEvent`s or a resize and **keeps the reader's place** (anchor = first block and byte offset
  of the current spread). The journal opens at its last spread (newest entries), other
  contents at their first. `mark_visible_seen()` clears new markers for what is on screen.
  `JournalCtx { world: &WorldRpg, labels: &dyn JournalLabels, paginator: &dyn Paginator }`
  (`dyn` at the crate boundary, §8b rule 7).

## Co-op

Read-only over host world state (`WorldRpg`, replicated); no commands, nothing written to the
world. `JournalSeen` is per-player presentation state (rule 3), kept by each client with its
profile/save, never on the host. No clock or RNG. Stable ids only (`RecordId`, entry `seq`).

## Industry comparison

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Book journal, dated entries across pages | Morrowind; Daggerfall | line paginator, 1–2 pages per spread | **match** |
| Active / completed quest split | Oblivion, Skyrim, Witcher 3, BG3 | `QuestFilter` | **match** |
| Topic index with letter tabs | Morrowind | `Alphabet` tabs over `ListModel` | **match** |
| Journal search | Pillars of Eternity, BG3 | AND terms, highlight ranges | **match** |
| New markers clearing on view | Witcher 3, BG3, Skyrim | per-player `JournalSeen` | **match** |
| Widow / orphan / keep-with-next | CSS Fragmentation 3, print typesetting | `PageMetrics` | **match** |
| Tracked quest / map markers | Skyrim, Witcher 3 | none (no objectives in `sf_rpg`) | **gap (P2)**: needs quest objectives (M7) |
| Locale-aware collation of the index | ICU collation | lowercase + aliases | **differ**: enough for Latin packs; ICU when a localised pack needs it |

## Tests

Synthetic journals: paginator (fit, split with orphans / widows, keep-with-next, tiny pages,
contiguous coverage), fixed-width wrap, quest split and sorts, letter tabs and aliases, topic
filter, search folding and highlights (multi-byte text), seen watermark compaction, screen
navigation, opening at the last spread, place kept across new entries and resize, serde round trip
of `JournalSeen`.
