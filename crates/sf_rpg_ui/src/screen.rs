//! The journal screen's state machine (journal-ui.md "`JournalScreen`").

use std::collections::HashSet;
use std::ops::Range;

use sf_rpg::{JournalEntry, RecordId, WorldRpg};
use sf_ui::{ListFilter, ListModel, SortSpec, UiValue};

use crate::book::{Block, BlockKey, BlockStyle, Page, Paginator};
use crate::labels::{self, JournalLabels};
use crate::rows::{Alphabet, LetterTab, QuestRow, TopicRow, letter_tabs, quest_rows, topic_rows};
use crate::search::SearchQuery;
use crate::seen::JournalSeen;

/// What the book shows.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum BookContent {
    /// Every entry, dated, oldest first.
    #[default]
    Journal,
    /// One quest's entries under its name.
    Quest(RecordId),
    /// What was heard under one topic.
    Topic(RecordId),
    /// Entries matching the current search, highlighted.
    Search,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QuestFilter {
    #[default]
    Active,
    Finished,
    All,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QuestSort {
    /// By name.
    #[default]
    Name,
    /// Latest entry first.
    Recent,
}

/// Everything a rebuild reads. `dyn` at the crate boundary (ARCHITECTURE §8b rule 7).
#[derive(Clone, Copy)]
pub struct JournalCtx<'a> {
    pub world: &'a WorldRpg,
    pub labels: &'a dyn JournalLabels,
    pub paginator: &'a dyn Paginator,
}

/// An entry that matched the search.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntryHit {
    pub seq: u32,
    /// Highlight ranges in the entry's displayed text.
    pub highlights: Vec<Range<usize>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SearchResults {
    /// Matching entries, oldest first (the search book).
    pub entries: Vec<EntryHit>,
    /// Quests whose name matches, in first-entry order.
    pub quests: Vec<RecordId>,
    /// Topics whose name or heard lines match, in learning order.
    pub topics: Vec<RecordId>,
}

/// View-model of the journal screen: book pages, quest list, topic index, search, new markers.
pub struct JournalScreen {
    alphabet: Alphabet,
    pages_per_spread: usize,
    content: BookContent,
    blocks: Vec<Block>,
    pages: Vec<Page>,
    /// First page of the visible spread.
    first_page: usize,
    quests: ListModel<RecordId, QuestRow>,
    quest_keys: Vec<RecordId>,
    quest_filter: QuestFilter,
    quest_sort: QuestSort,
    topics: ListModel<RecordId, TopicRow>,
    topic_keys: Vec<RecordId>,
    tabs: Vec<LetterTab>,
    letter: Option<String>,
    topic_filter: String,
    query: SearchQuery,
    results: SearchResults,
    seen: JournalSeen,
}

impl JournalScreen {
    /// `pages_per_spread`: 2 for an open book, 1 for a single page (controller preset).
    pub fn new(alphabet: Alphabet, pages_per_spread: usize, seen: JournalSeen) -> Self {
        let mut screen = Self {
            alphabet,
            pages_per_spread: pages_per_spread.max(1),
            content: BookContent::Journal,
            blocks: Vec::new(),
            pages: Vec::new(),
            first_page: 0,
            quests: ListModel::new(),
            quest_keys: Vec::new(),
            quest_filter: QuestFilter::default(),
            quest_sort: QuestSort::default(),
            topics: ListModel::new(),
            topic_keys: Vec::new(),
            tabs: Vec::new(),
            letter: None,
            topic_filter: String::new(),
            query: SearchQuery::default(),
            results: SearchResults::default(),
            seen,
        };
        screen.apply_quest_view();
        screen.apply_topic_view();
        screen
    }

    // ------------------------------------------------------------ book

    /// Shows `content`: the journal at its last spread (newest entries), anything else at its
    /// first.
    pub fn open(&mut self, ctx: &JournalCtx<'_>, content: BookContent) {
        self.content = content;
        self.rebuild_rows(ctx);
        self.rebuild_book(ctx);
        self.first_page = if self.content == BookContent::Journal {
            self.align(self.pages.len().saturating_sub(1))
        } else {
            0
        };
    }

    /// Rebuilds rows and pages after world changes (`RpgEvent`s), a new paginator (resize,
    /// text scale) or new labels (language), keeping the reader's place.
    pub fn refresh(&mut self, ctx: &JournalCtx<'_>) {
        let anchor = self.anchor();
        self.rebuild_rows(ctx);
        self.rebuild_book(ctx);
        self.first_page = match anchor {
            Some((key, offset)) => self.align(self.page_at(&key, offset).unwrap_or(0)),
            None => 0,
        };
    }

    pub fn content(&self) -> &BookContent {
        &self.content
    }

    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    pub fn pages_per_spread(&self) -> usize {
        self.pages_per_spread
    }

    /// Switches between one and two pages per spread, keeping the visible page.
    pub fn set_pages_per_spread(&mut self, n: usize) {
        self.pages_per_spread = n.max(1);
        self.first_page = self.align(self.first_page);
    }

    /// Index of the first visible page.
    pub fn first_visible_page(&self) -> usize {
        self.first_page
    }

    /// The visible pages (fewer than `pages_per_spread` on the last spread; none in an empty book).
    pub fn visible_pages(&self) -> &[Page] {
        let start = self.first_page.min(self.pages.len());
        let end = (start + self.pages_per_spread).min(self.pages.len());
        &self.pages[start..end]
    }

    pub fn spread_count(&self) -> usize {
        self.pages.len().div_ceil(self.pages_per_spread)
    }

    pub fn spread_index(&self) -> usize {
        self.first_page / self.pages_per_spread
    }

    /// `false` on the last spread.
    pub fn next_spread(&mut self) -> bool {
        let next = self.first_page + self.pages_per_spread;
        if next < self.pages.len() {
            self.first_page = next;
            true
        } else {
            false
        }
    }

    /// `false` on the first spread.
    pub fn prev_spread(&mut self) -> bool {
        if self.first_page == 0 {
            return false;
        }
        self.first_page = self.first_page.saturating_sub(self.pages_per_spread);
        true
    }

    pub fn first_spread(&mut self) {
        self.first_page = 0;
    }

    pub fn last_spread(&mut self) {
        self.first_page = self.align(self.pages.len().saturating_sub(1));
    }

    /// Shows the spread holding `page` (clamped).
    pub fn go_to_page(&mut self, page: usize) {
        self.first_page = self.align(page.min(self.pages.len().saturating_sub(1)));
    }

    /// First page showing the block `key`.
    pub fn page_of(&self, key: &BlockKey) -> Option<usize> {
        self.page_at(key, 0)
    }

    fn align(&self, page: usize) -> usize {
        page - page % self.pages_per_spread
    }

    /// First block and byte offset of the visible spread.
    fn anchor(&self) -> Option<(BlockKey, usize)> {
        let slice = self.pages.get(self.first_page)?.slices.first()?;
        Some((self.blocks[slice.block].key.clone(), slice.range.start))
    }

    /// Page holding byte `offset` of block `key` (its last page if the text got shorter).
    fn page_at(&self, key: &BlockKey, offset: usize) -> Option<usize> {
        let block = self.blocks.iter().position(|b| &b.key == key)?;
        let mut found = None;
        for (p, page) in self.pages.iter().enumerate() {
            for s in page.slices.iter().filter(|s| s.block == block) {
                if s.range.start <= offset {
                    found = Some(p);
                }
                if offset < s.range.end {
                    return found.or(Some(p));
                }
            }
        }
        found
    }

    fn rebuild_book(&mut self, ctx: &JournalCtx<'_>) {
        self.blocks = match &self.content {
            BookContent::Journal => self.entry_blocks(ctx, |_| Some(Vec::new())),
            BookContent::Quest(id) => {
                let title = Block::new(
                    BlockKey::Title,
                    BlockStyle::Title,
                    labels::quest_name(ctx.labels, id),
                );
                let mut blocks = vec![title];
                blocks.extend(
                    self.entry_blocks(ctx, |e| (e.quest.as_ref() == Some(id)).then(Vec::new)),
                );
                blocks
            }
            BookContent::Topic(id) => self.topic_blocks(ctx, id),
            BookContent::Search => {
                let hits = &self.results.entries;
                self.entry_blocks(ctx, |e| {
                    hits.binary_search_by_key(&e.seq, |h| h.seq)
                        .ok()
                        .map(|i| hits[i].highlights.clone())
                })
            }
        };
        self.pages = ctx.paginator.paginate(&self.blocks);
    }

    /// Dated entry blocks for the entries `pick` accepts (returning their highlights).
    fn entry_blocks(
        &self,
        ctx: &JournalCtx<'_>,
        pick: impl Fn(&JournalEntry) -> Option<Vec<Range<usize>>>,
    ) -> Vec<Block> {
        let mut blocks = Vec::new();
        let mut day = None;
        for e in ctx.world.journal.entries() {
            let Some(highlights) = pick(e) else { continue };
            if day != Some(e.time.day) {
                day = Some(e.time.day);
                let heading = ctx.labels.day_heading(e.time.day);
                blocks.push(Block::new(
                    BlockKey::DayBefore(e.seq),
                    BlockStyle::Heading,
                    heading,
                ));
            }
            let mut block = Block::new(
                BlockKey::Entry(e.seq),
                BlockStyle::Body,
                labels::entry_text(ctx.labels, e),
            );
            block.new = !self.seen.entry_seen(e.seq);
            block.highlights = highlights;
            blocks.push(block);
        }
        blocks
    }

    fn topic_blocks(&self, ctx: &JournalCtx<'_>, id: &RecordId) -> Vec<Block> {
        let mut blocks = vec![Block::new(
            BlockKey::Title,
            BlockStyle::Title,
            labels::topic_name(ctx.labels, id),
        )];
        let Some(known) = ctx.world.topics.get(id) else {
            return blocks;
        };
        let seen = self.seen.topic_heard_seen(id).unwrap_or(0);
        for (i, line) in known.heard.iter().enumerate() {
            if !line.speaker.is_empty() {
                blocks.push(Block::new(
                    BlockKey::Speaker(i),
                    BlockStyle::Heading,
                    line.speaker.clone(),
                ));
            }
            let mut block = Block::new(BlockKey::Heard(i), BlockStyle::Body, line.text.clone());
            block.new = i >= seen;
            blocks.push(block);
        }
        blocks
    }

    // ------------------------------------------------------------ quests

    pub fn quest_filter(&self) -> QuestFilter {
        self.quest_filter
    }

    pub fn set_quest_filter(&mut self, filter: QuestFilter) {
        self.quest_filter = filter;
        self.apply_quest_view();
    }

    pub fn quest_sort(&self) -> QuestSort {
        self.quest_sort
    }

    pub fn set_quest_sort(&mut self, sort: QuestSort) {
        self.quest_sort = sort;
        self.apply_quest_view();
    }

    /// Quests passing the filter, in sort order.
    pub fn quest_list(&mut self) -> &[RecordId] {
        self.quests.view()
    }

    /// The quest list model (rows, versions, windows for a virtualized list).
    pub fn quest_model(&mut self) -> &mut ListModel<RecordId, QuestRow> {
        &mut self.quests
    }

    fn apply_quest_view(&mut self) {
        let equals = match self.quest_filter {
            QuestFilter::Active => vec![("finished".to_owned(), UiValue::Bool(false))],
            QuestFilter::Finished => vec![("finished".to_owned(), UiValue::Bool(true))],
            QuestFilter::All => Vec::new(),
        };
        self.quests.set_filter(ListFilter {
            equals,
            ..ListFilter::default()
        });
        self.quests.set_sort(Some(match self.quest_sort {
            QuestSort::Name => SortSpec {
                field: "name".to_owned(),
                descending: false,
            },
            QuestSort::Recent => SortSpec {
                field: "last_seq".to_owned(),
                descending: true,
            },
        }));
    }

    // ------------------------------------------------------------ topics

    /// Every letter tab with its topic count and new flag.
    pub fn tabs(&self) -> &[LetterTab] {
        &self.tabs
    }

    pub fn letter(&self) -> Option<&str> {
        self.letter.as_deref()
    }

    /// Shows one letter tab, or every topic with `None`.
    pub fn set_letter(&mut self, letter: Option<&str>) {
        self.letter = letter.map(str::to_owned);
        self.apply_topic_view();
    }

    /// Filters the topic index by name (case-insensitive substring).
    pub fn set_topic_filter(&mut self, text: &str) {
        text.clone_into(&mut self.topic_filter);
        self.apply_topic_view();
    }

    /// Topics passing the tab and name filter, by name.
    pub fn topic_list(&mut self) -> &[RecordId] {
        self.topics.view()
    }

    /// The topic index model (rows, versions, windows for a virtualized list).
    pub fn topic_model(&mut self) -> &mut ListModel<RecordId, TopicRow> {
        &mut self.topics
    }

    fn apply_topic_view(&mut self) {
        self.topics.set_filter(ListFilter {
            search: self.topic_filter.clone(),
            search_fields: vec!["name".to_owned()],
            equals: self
                .letter
                .iter()
                .map(|l| ("initial".to_owned(), UiValue::Text(l.clone())))
                .collect(),
        });
        self.topics.set_sort(Some(SortSpec {
            field: "name".to_owned(),
            descending: false,
        }));
    }

    // ------------------------------------------------------------ search

    /// Searches entries, quest names and topics, and opens the matching entries as a book.
    /// Empty text clears the search and returns to the journal.
    pub fn search(&mut self, ctx: &JournalCtx<'_>, text: &str) {
        self.query = SearchQuery::parse(text);
        if self.query.is_empty() {
            self.results = SearchResults::default();
            self.open(ctx, BookContent::Journal);
        } else {
            self.open(ctx, BookContent::Search);
        }
    }

    pub fn query(&self) -> &SearchQuery {
        &self.query
    }

    pub fn results(&self) -> &SearchResults {
        &self.results
    }

    fn rebuild_results(&mut self, ctx: &JournalCtx<'_>) {
        if self.query.is_empty() {
            self.results = SearchResults::default();
            return;
        }
        let world = ctx.world;
        let entries = world
            .journal
            .entries()
            .iter()
            .filter_map(|e| {
                let highlights = self.query.find(&labels::entry_text(ctx.labels, e))?;
                Some(EntryHit {
                    seq: e.seq,
                    highlights,
                })
            })
            .collect();
        let quests = self
            .quest_keys
            .iter()
            .filter(|id| {
                self.quests
                    .get(id)
                    .is_some_and(|r| self.query.find(&r.name).is_some())
            })
            .cloned()
            .collect();
        let topics = self
            .topic_keys
            .iter()
            .filter(|id| {
                let name = self.topics.get(id).map_or("", |r| r.name.as_str());
                let heard = world
                    .topics
                    .get(id)
                    .map(|k| k.heard.as_slice())
                    .unwrap_or_default();
                self.query.matches_any_of(
                    std::iter::once(name).chain(heard.iter().map(|h| h.text.as_str())),
                )
            })
            .cloned()
            .collect();
        self.results = SearchResults {
            entries,
            quests,
            topics,
        };
    }

    // ------------------------------------------------------------ new markers

    pub fn seen(&self) -> &JournalSeen {
        &self.seen
    }

    /// Marks what is on screen as seen: entries on the visible pages, and the open topic's
    /// lines. Then refreshes the markers.
    pub fn mark_visible_seen(&mut self, ctx: &JournalCtx<'_>) {
        let start = self.first_page.min(self.pages.len());
        let end = (start + self.pages_per_spread).min(self.pages.len());
        for page in &self.pages[start..end] {
            for slice in &page.slices {
                if let BlockKey::Entry(seq) = self.blocks[slice.block].key {
                    self.seen.mark_entry(seq);
                }
            }
        }
        if let BookContent::Topic(id) = &self.content {
            let heard = ctx.world.topics.get(id).map_or(0, |k| k.heard.len());
            self.seen.mark_topic(id, heard);
        }
        self.refresh(ctx);
    }

    /// "Mark all read".
    pub fn mark_all_seen(&mut self, ctx: &JournalCtx<'_>) {
        self.seen.mark_all_seen(ctx.world);
        self.refresh(ctx);
    }

    // ------------------------------------------------------------ rows

    fn rebuild_rows(&mut self, ctx: &JournalCtx<'_>) {
        let quests = quest_rows(ctx.world, ctx.labels, &self.seen);
        self.quest_keys = sync(
            &mut self.quests,
            &self.quest_keys,
            quests.into_iter().map(|r| (r.id.clone(), r)),
        );
        let topics = topic_rows(ctx.world, ctx.labels, &self.seen, &self.alphabet);
        self.tabs = letter_tabs(&self.alphabet, &topics);
        self.topic_keys = sync(
            &mut self.topics,
            &self.topic_keys,
            topics.into_iter().map(|r| (r.id.clone(), r)),
        );
        self.rebuild_results(ctx);
    }
}

/// Upserts `rows` into `model` and removes the keys of `old` that are gone; returns the new keys.
fn sync<T: sf_ui::ViewModel + PartialEq>(
    model: &mut ListModel<RecordId, T>,
    old: &[RecordId],
    rows: impl Iterator<Item = (RecordId, T)>,
) -> Vec<RecordId> {
    let mut keys = Vec::new();
    for (k, row) in rows {
        model.upsert(k.clone(), row);
        keys.push(k);
    }
    let live: HashSet<&RecordId> = keys.iter().collect();
    for k in old.iter().filter(|k| !live.contains(k)) {
        model.remove(k);
    }
    keys
}
