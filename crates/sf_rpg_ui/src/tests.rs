// Highlight lists with one range are the expected values, not a range typo.
#![allow(clippy::single_range_in_vec_init)]

use std::collections::HashMap;

use sf_rpg::{EntryDraft, GameTime, HeardLine, RecordId, RpgCommand, StageFlags, WorldRpg};

use super::*;

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

/// Names from a table; days as "Day N".
#[derive(Default)]
struct Labels {
    names: HashMap<RecordId, String>,
}

impl Labels {
    fn with(mut self, id_: &str, name: &str) -> Self {
        self.names.insert(id(id_), name.to_owned());
        self
    }
}

impl JournalLabels for Labels {
    fn quest_name(&self, quest: &RecordId) -> Option<String> {
        self.names.get(quest).cloned()
    }

    fn topic_name(&self, topic: &RecordId) -> Option<String> {
        self.names.get(topic).cloned()
    }

    fn day_heading(&self, day: u32) -> String {
        format!("Day {day}")
    }
}

fn entry(world: &mut WorldRpg, quest: &str, stage: i32, day: u32, text: &str) {
    stage_entry(world, quest, stage, StageFlags::default(), day, text);
}

fn stage_entry(
    world: &mut WorldRpg,
    quest: &str,
    stage: i32,
    flags: StageFlags,
    day: u32,
    text: &str,
) {
    let draft = EntryDraft {
        source: None,
        text: text.to_owned(),
        time: GameTime { day, minute: 0 },
    };
    world
        .apply(RpgCommand::SetStage {
            quest: id(quest),
            stage,
            flags,
            entry: Some(draft),
        })
        .expect("world command");
}

fn hear(world: &mut WorldRpg, topic: &str, source: &str, speaker: &str, text: &str) {
    let line = HeardLine {
        source: id(source),
        speaker: speaker.to_owned(),
        text: text.to_owned(),
    };
    world
        .apply(RpgCommand::HearLine {
            topic: id(topic),
            line,
        })
        .expect("world command");
}

/// `n` ten-letter words: exactly `n` lines at 10 columns.
fn body(n: usize) -> String {
    vec!["aaaaaaaaaa"; n].join(" ")
}

fn paginator(height: f32) -> LinePaginator<FixedWidthMeasure> {
    LinePaginator::new(
        FixedWidthMeasure { columns: 10 },
        PageMetrics {
            height,
            block_gap: 0.0,
            orphans: 2,
            widows: 2,
        },
    )
}

fn texts<'a>(text: &'a str, ranges: &[std::ops::Range<usize>]) -> Vec<&'a str> {
    ranges.iter().map(|r| &text[r.clone()]).collect()
}

/// Lines of block `b` on each page, as (page, line count).
fn spread_of(pages: &[Page], blocks: &[Block], b: usize) -> Vec<(usize, usize)> {
    let m = FixedWidthMeasure { columns: 10 };
    let mut out = Vec::new();
    for (p, page) in pages.iter().enumerate() {
        for s in page.slices.iter().filter(|s| s.block == b) {
            let text = &blocks[b].text[s.range.clone()];
            out.push((p, m.lines(text, BlockStyle::Body).len()));
        }
    }
    out
}

// ---------------------------------------------------------------- measure

#[test]
fn fixed_width_wraps_words_newlines_and_long_words() {
    let m = FixedWidthMeasure { columns: 8 };
    let text = "The quick brown fox\njumps";
    assert_eq!(
        texts(text, &m.lines(text, BlockStyle::Body)),
        ["The ", "quick ", "brown ", "fox\n", "jumps"]
    );
    let long = "abcdefg";
    assert_eq!(
        texts(
            long,
            &FixedWidthMeasure { columns: 3 }.lines(long, BlockStyle::Body)
        ),
        ["abc", "def", "g"]
    );
    // Character counts, byte ranges.
    let accents = "éèà";
    assert_eq!(
        FixedWidthMeasure { columns: 2 }.lines(accents, BlockStyle::Body),
        [0..4, 4..6]
    );
    assert!(m.lines("", BlockStyle::Body).is_empty());
    assert_eq!(
        texts(
            &body(3),
            &FixedWidthMeasure { columns: 10 }.lines(&body(3), BlockStyle::Body)
        )
        .len(),
        3
    );
}

// ---------------------------------------------------------------- paginator

#[test]
fn blocks_fit_split_with_orphans_and_widows() {
    let p = paginator(5.0);
    // 3 + 4 lines on 5-line pages: the second splits 2 / 2.
    let blocks = vec![
        Block::new(BlockKey::Entry(0), BlockStyle::Body, body(3)),
        Block::new(BlockKey::Entry(1), BlockStyle::Body, body(4)),
    ];
    let pages = p.paginate(&blocks);
    assert_eq!(pages.len(), 2);
    assert_eq!(spread_of(&pages, &blocks, 0), [(0, 3)]);
    assert_eq!(spread_of(&pages, &blocks, 1), [(0, 2), (1, 2)]);

    // 3 + 3: a 2 / 1 split would leave a widow, so the block moves on whole.
    let blocks = vec![
        Block::new(BlockKey::Entry(0), BlockStyle::Body, body(3)),
        Block::new(BlockKey::Entry(1), BlockStyle::Body, body(3)),
    ];
    let pages = p.paginate(&blocks);
    assert_eq!(spread_of(&pages, &blocks, 1), [(1, 3)]);

    // 4 + 3: one line of room is fewer than the orphans, so the block moves on.
    let blocks = vec![
        Block::new(BlockKey::Entry(0), BlockStyle::Body, body(4)),
        Block::new(BlockKey::Entry(1), BlockStyle::Body, body(5)),
    ];
    let pages = p.paginate(&blocks);
    assert_eq!(spread_of(&pages, &blocks, 1), [(1, 5)]);
}

#[test]
fn headings_keep_with_the_next_block() {
    let p = paginator(5.0);
    let blocks = vec![
        Block::new(BlockKey::Entry(0), BlockStyle::Body, body(4)),
        Block::new(BlockKey::DayBefore(1), BlockStyle::Heading, "Day 2"),
        Block::new(BlockKey::Entry(1), BlockStyle::Body, body(3)),
    ];
    let pages = p.paginate(&blocks);
    assert_eq!(pages.len(), 2);
    assert_eq!(
        pages[1].slices.iter().map(|s| s.block).collect::<Vec<_>>(),
        [1, 2]
    );

    // A title, a heading and a body chain together.
    let blocks = vec![
        Block::new(BlockKey::Entry(0), BlockStyle::Body, body(2)),
        Block::new(BlockKey::Title, BlockStyle::Title, "T"),
        Block::new(BlockKey::DayBefore(1), BlockStyle::Heading, "Day 2"),
        Block::new(BlockKey::Entry(1), BlockStyle::Body, body(2)),
    ];
    let pages = p.paginate(&blocks);
    assert_eq!(pages.len(), 2);
    assert_eq!(
        pages[1].slices.iter().map(|s| s.block).collect::<Vec<_>>(),
        [1, 2, 3]
    );
}

#[test]
fn tiny_pages_still_make_progress() {
    let p = paginator(0.5);
    let blocks = vec![Block::new(BlockKey::Entry(0), BlockStyle::Body, body(3))];
    let pages = p.paginate(&blocks);
    assert_eq!(pages.len(), 3);
    assert!(pages.iter().all(|pg| pg.slices.len() == 1));
}

#[test]
fn slices_cover_every_block_in_order() {
    let words = [
        "fargoth",
        "ring",
        "hidden",
        "in",
        "a",
        "tree",
        "stump",
        "near",
        "Seyda",
        "Neen's",
        "lighthouse",
    ];
    let mut blocks = Vec::new();
    for i in 0..40u32 {
        let n = (i as usize * 7) % 23 + 1;
        let text: Vec<&str> = (0..n)
            .map(|k| words[(k + i as usize) % words.len()])
            .collect();
        let style = if i % 5 == 0 {
            BlockStyle::Heading
        } else {
            BlockStyle::Body
        };
        blocks.push(Block::new(BlockKey::Entry(i), style, text.join(" ")));
    }
    blocks.push(Block::new(BlockKey::Entry(99), BlockStyle::Body, ""));
    for (height, gap, columns) in [
        (5.0, 0.0, 12),
        (9.0, 1.0, 20),
        (3.0, 0.5, 7),
        (40.0, 1.0, 30),
    ] {
        let p = LinePaginator::new(
            FixedWidthMeasure { columns },
            PageMetrics {
                height,
                block_gap: gap,
                ..PageMetrics::default()
            },
        );
        let pages = p.paginate(&blocks);
        for (b, block) in blocks.iter().enumerate() {
            let ranges: Vec<_> = pages
                .iter()
                .flat_map(|pg| &pg.slices)
                .filter(|s| s.block == b)
                .map(|s| s.range.clone())
                .collect();
            if block.text.is_empty() {
                assert!(ranges.is_empty());
                continue;
            }
            assert_eq!(ranges.first().unwrap().start, 0);
            assert_eq!(ranges.last().unwrap().end, block.text.len());
            assert!(
                ranges.windows(2).all(|w| w[0].end == w[1].start),
                "contiguous {b}"
            );
        }
        // Blocks appear in order, and every page holds something.
        let order: Vec<usize> = pages
            .iter()
            .flat_map(|pg| &pg.slices)
            .map(|s| s.block)
            .collect();
        assert!(order.windows(2).all(|w| w[0] <= w[1]));
        assert!(pages.iter().all(|pg| !pg.slices.is_empty()));
    }
}

// ---------------------------------------------------------------- search

#[test]
fn search_folds_case_and_maps_highlights_to_the_original_text() {
    let text = "Ébène and ÉBÈNE";
    assert_eq!(
        SearchQuery::parse("ébène").find(text),
        Some(vec![0..7, 12..19])
    );
    assert_eq!(SearchQuery::parse("ÉBÈNE missing").find(text), None);
    // Overlapping terms merge.
    assert_eq!(SearchQuery::parse("an AND").find(text), Some(vec![8..11]));
    // A character that lowercases to two highlights whole.
    assert_eq!(SearchQuery::parse("i").find("İstanbul"), Some(vec![0..2]));
    assert!(SearchQuery::parse("   ").is_empty());
    assert_eq!(SearchQuery::parse("").find(text), None);
    assert_eq!(highlights_in(&[0..7, 12..19], &(5..14)), [5..7, 12..14]);
}

// ---------------------------------------------------------------- seen

#[test]
fn seen_watermark_compacts_and_round_trips() {
    let mut seen = JournalSeen::new();
    seen.mark_entry(2);
    seen.mark_entry(0);
    assert!(seen.entry_seen(0) && !seen.entry_seen(1) && seen.entry_seen(2));
    seen.mark_entry(1);
    assert!(seen.entry_seen(1));
    assert_eq!(seen, {
        let mut s = JournalSeen::new();
        for i in 0..3 {
            s.mark_entry(i);
        }
        s
    });
    assert!(seen.topic_new(&id("t"), 0));
    seen.mark_topic(&id("t"), 2);
    assert!(!seen.topic_new(&id("t"), 2) && seen.topic_new(&id("t"), 3));
    seen.mark_entry(7);
    let bytes = postcard::to_allocvec(&seen).unwrap();
    assert_eq!(postcard::from_bytes::<JournalSeen>(&bytes).unwrap(), seen);
}

// ---------------------------------------------------------------- quests and topics

fn quest_world() -> WorldRpg {
    let mut w = WorldRpg::default();
    entry(&mut w, "q_b", 10, 1, "Bandits in the cave.");
    entry(&mut w, "q_a", 10, 1, "Find the amulet.");
    let finish = StageFlags {
        finish: true,
        restart: false,
    };
    stage_entry(&mut w, "q_c", 100, finish, 2, "The cave is clear.");
    entry(&mut w, "q_b", 20, 3, "The bandit leader is dead.");
    w
}

fn quest_labels() -> Labels {
    Labels::default()
        .with("q_a", "Amulet")
        .with("q_b", "bandits")
        .with("q_c", "Cave")
}

#[test]
fn quests_split_active_and_finished_and_sort() {
    let world = quest_world();
    let labels = quest_labels();
    let paginator = paginator(10.0);
    let ctx = JournalCtx {
        world: &world,
        labels: &labels,
        paginator: &paginator,
    };
    let mut screen = JournalScreen::new(Alphabet::default(), 2, JournalSeen::new());
    screen.refresh(&ctx);

    assert_eq!(screen.quest_list(), [id("q_a"), id("q_b")]);
    screen.set_quest_filter(QuestFilter::Finished);
    assert_eq!(screen.quest_list(), [id("q_c")]);
    screen.set_quest_filter(QuestFilter::All);
    screen.set_quest_sort(QuestSort::Recent);
    assert_eq!(screen.quest_list(), [id("q_b"), id("q_c"), id("q_a")]);

    let b = screen.quest_model().get(&id("q_b")).unwrap().clone();
    assert_eq!(
        (b.entries, b.stage, b.last_day, b.new, b.finished),
        (2, 20, 3, true, false)
    );
    assert_eq!(
        sf_ui::ViewModel::field(&b, "name"),
        Some(sf_ui::UiValue::Text("bandits".into()))
    );

    // A quest book: its title, then its dated entries.
    screen.open(&ctx, BookContent::Quest(id("q_b")));
    let keys: Vec<_> = screen.blocks().iter().map(|b| b.key.clone()).collect();
    assert_eq!(
        keys,
        [
            BlockKey::Title,
            BlockKey::DayBefore(0),
            BlockKey::Entry(0),
            BlockKey::DayBefore(3),
            BlockKey::Entry(3)
        ]
    );
    assert_eq!(screen.blocks()[0].text, "bandits");
}

#[test]
fn topic_index_letter_tabs_aliases_and_filter() {
    let mut world = WorldRpg::default();
    for t in ["t_ash", "t_ald", "t_ebo", "t_num", "t_vos"] {
        world.apply(RpgCommand::AddTopic(id(t))).unwrap();
    }
    let labels = Labels::default()
        .with("t_ald", "Ald'ruhn")
        .with("t_ash", "ashlanders")
        .with("t_ebo", "Ébonheart")
        .with("t_num", "8 Divines");
    let alphabet = Alphabet::default().with_alias('É', "E");
    assert_eq!(alphabet.tab_of("ébonheart"), "E");
    assert_eq!(alphabet.tab_of("  zainab"), "Z");
    assert_eq!(alphabet.tab_of(""), "#");

    let paginator = paginator(10.0);
    let ctx = JournalCtx {
        world: &world,
        labels: &labels,
        paginator: &paginator,
    };
    let mut screen = JournalScreen::new(alphabet, 2, JournalSeen::new());
    screen.refresh(&ctx);

    let count = |l: &str| {
        screen
            .tabs()
            .iter()
            .find(|t| t.label == l)
            .map(|t| (t.count, t.new))
    };
    assert_eq!(count("A"), Some((2, true)));
    assert_eq!(count("E"), Some((1, true)));
    assert_eq!(count("T"), Some((1, true))); // no name: the id "t_vos"
    assert_eq!(count("#"), Some((1, true)));
    assert_eq!(count("B"), Some((0, false)));
    assert_eq!(screen.tabs().len(), 27);

    assert_eq!(
        screen.topic_list(),
        [
            id("t_num"),
            id("t_ald"),
            id("t_ash"),
            id("t_vos"),
            id("t_ebo")
        ]
    ); // lowercase code-point order: é after t
    screen.set_letter(Some("A"));
    assert_eq!(screen.topic_list(), [id("t_ald"), id("t_ash")]);
    screen.set_topic_filter("LAND");
    assert_eq!(screen.topic_list(), [id("t_ash")]);
    screen.set_letter(None);
    screen.set_topic_filter("");
    assert_eq!(screen.topic_list().len(), 5);
}

// ---------------------------------------------------------------- screen

/// `n` entries over `n / 3` days, each 2-4 lines at 10 columns.
fn long_journal(n: u32) -> WorldRpg {
    let mut w = WorldRpg::default();
    for i in 0..n {
        let quest = if i % 2 == 0 { "q_even" } else { "q_odd" };
        let stage = i32::try_from(i).unwrap();
        entry(&mut w, quest, stage, i / 3, &body(2 + (i as usize % 3)));
    }
    w
}

#[test]
fn journal_opens_at_the_newest_spread_and_pages_through() {
    let world = long_journal(30);
    let labels = Labels::default();
    let paginator = paginator(6.0);
    let ctx = JournalCtx {
        world: &world,
        labels: &labels,
        paginator: &paginator,
    };
    let mut screen = JournalScreen::new(Alphabet::default(), 2, JournalSeen::new());
    screen.open(&ctx, BookContent::Journal);

    let pages = screen.page_count();
    assert!(pages > 10);
    assert_eq!(screen.spread_count(), pages.div_ceil(2));
    assert_eq!(screen.spread_index(), screen.spread_count() - 1);
    let last = screen
        .visible_pages()
        .last()
        .unwrap()
        .slices
        .last()
        .unwrap();
    assert_eq!(screen.blocks()[last.block].key, BlockKey::Entry(29));
    assert!(!screen.next_spread());

    let mut back = 0;
    while screen.prev_spread() {
        back += 1;
    }
    assert_eq!(back, screen.spread_count() - 1);
    assert_eq!(screen.visible_pages().len(), 2);
    assert_eq!(
        screen.blocks()[screen.visible_pages()[0].slices[0].block].key,
        BlockKey::DayBefore(0)
    );

    screen.go_to_page(5);
    assert_eq!(screen.first_visible_page(), 4);
    screen.set_pages_per_spread(1);
    assert_eq!(screen.visible_pages().len(), 1);
    assert_eq!(screen.first_visible_page(), 4);
    assert_eq!(screen.page_of(&BlockKey::Entry(0)), Some(0));
}

#[test]
fn refresh_keeps_the_place_across_new_entries_and_resize() {
    let mut world = long_journal(30);
    let labels = Labels::default();
    let small = paginator(6.0);
    let mut screen = JournalScreen::new(Alphabet::default(), 2, JournalSeen::new());
    screen.open(
        &JournalCtx {
            world: &world,
            labels: &labels,
            paginator: &small,
        },
        BookContent::Journal,
    );
    screen.go_to_page(8);
    let top = screen.visible_pages()[0].slices[0].clone();
    let key = screen.blocks()[top.block].key.clone();

    // New entries arrive: the reader stays on the same spread.
    entry(&mut world, "q_new", 1, 40, &body(3));
    screen.refresh(&JournalCtx {
        world: &world,
        labels: &labels,
        paginator: &small,
    });
    assert_eq!(screen.first_visible_page(), 8);
    assert_eq!(screen.visible_pages()[0].slices[0], top);

    // Bigger pages: the spread still shows the same text.
    let big = paginator(15.0);
    screen.refresh(&JournalCtx {
        world: &world,
        labels: &labels,
        paginator: &big,
    });
    let shown = screen
        .visible_pages()
        .iter()
        .flat_map(|p| &p.slices)
        .any(|s| {
            screen.blocks()[s.block].key == key
                && s.range.start <= top.range.start
                && top.range.start < s.range.end
        });
    assert!(shown, "anchor {key:?} at {} not visible", top.range.start);
    assert!(screen.first_visible_page() < 8);
}

#[test]
fn new_markers_clear_on_view() {
    let mut world = quest_world();
    hear(
        &mut world,
        "t_cave",
        "i1",
        "Fargoth",
        "Bandits live in the cave.",
    );
    hear(&mut world, "t_cave", "i2", "", "Nobody goes there.");
    let labels = quest_labels().with("t_cave", "cave");
    let paginator = paginator(4.0);
    let mut screen = JournalScreen::new(Alphabet::default(), 1, JournalSeen::new());
    let ctx = JournalCtx {
        world: &world,
        labels: &labels,
        paginator: &paginator,
    };
    screen.open(&ctx, BookContent::Journal);
    assert_eq!(screen.seen().unseen_entries(&world), 4);

    // The newest page shows the last entry only.
    screen.mark_visible_seen(&ctx);
    assert_eq!(screen.seen().unseen_entries(&world), 3);
    assert!(
        screen
            .blocks()
            .iter()
            .any(|b| b.key == BlockKey::Entry(0) && b.new)
    );
    assert!(
        screen
            .blocks()
            .iter()
            .any(|b| b.key == BlockKey::Entry(3) && !b.new)
    );
    assert!(screen.quest_model().get(&id("q_b")).unwrap().new); // entry 0 still unseen

    // A topic book: both lines new until opened; a later line is new again.
    screen.open(&ctx, BookContent::Topic(id("t_cave")));
    let keys: Vec<_> = screen
        .blocks()
        .iter()
        .map(|b| (b.key.clone(), b.new))
        .collect();
    assert_eq!(
        keys,
        [
            (BlockKey::Title, false),
            (BlockKey::Speaker(0), false),
            (BlockKey::Heard(0), true),
            (BlockKey::Heard(1), true)
        ]
    );
    assert!(screen.topic_model().get(&id("t_cave")).unwrap().new);
    screen.mark_visible_seen(&ctx);
    assert!(!screen.topic_model().get(&id("t_cave")).unwrap().new);
    assert!(screen.blocks().iter().all(|b| !b.new));

    hear(&mut world, "t_cave", "i3", "Fargoth", "Thank you!");
    let ctx = JournalCtx {
        world: &world,
        labels: &labels,
        paginator: &paginator,
    };
    screen.refresh(&ctx);
    assert!(screen.topic_model().get(&id("t_cave")).unwrap().new);
    assert_eq!(
        screen
            .blocks()
            .iter()
            .filter(|b| b.new)
            .map(|b| b.key.clone())
            .collect::<Vec<_>>(),
        [BlockKey::Heard(2)]
    );
    assert_eq!(screen.seen().new_topics(&world), 1);

    screen.mark_all_seen(&ctx);
    assert_eq!(screen.seen().unseen_entries(&world), 0);
    assert_eq!(screen.seen().new_topics(&world), 0);
    assert!(!screen.quest_model().get(&id("q_b")).unwrap().new);
}

#[test]
fn search_opens_matching_entries_with_highlights() {
    let mut world = quest_world();
    hear(
        &mut world,
        "t_cave",
        "i1",
        "Fargoth",
        "Bandits live in the cave.",
    );
    world.apply(RpgCommand::AddTopic(id("t_ring"))).unwrap();
    let labels = quest_labels().with("t_cave", "cave").with("t_ring", "ring");
    let paginator = paginator(10.0);
    let ctx = JournalCtx {
        world: &world,
        labels: &labels,
        paginator: &paginator,
    };
    let mut screen = JournalScreen::new(Alphabet::default(), 2, JournalSeen::new());
    screen.open(&ctx, BookContent::Journal);

    screen.search(&ctx, "BANDIT");
    assert_eq!(screen.content(), &BookContent::Search);
    let r = screen.results();
    assert_eq!(r.entries.iter().map(|h| h.seq).collect::<Vec<_>>(), [0, 3]);
    assert_eq!(r.entries[0].highlights, [0..6]);
    assert_eq!(r.quests, [id("q_b")]);
    assert_eq!(r.topics, [id("t_cave")]); // by a heard line
    let keys: Vec<_> = screen.blocks().iter().map(|b| b.key.clone()).collect();
    assert_eq!(
        keys,
        [
            BlockKey::DayBefore(0),
            BlockKey::Entry(0),
            BlockKey::DayBefore(3),
            BlockKey::Entry(3)
        ]
    );
    assert_eq!(screen.blocks()[3].highlights, [4..10]);

    // Every term must occur in the same entry.
    screen.search(&ctx, "cave clear");
    assert_eq!(
        screen
            .results()
            .entries
            .iter()
            .map(|h| h.seq)
            .collect::<Vec<_>>(),
        [2]
    );
    assert_eq!(screen.results().quests, Vec::<RecordId>::new());

    screen.search(&ctx, "  ");
    assert_eq!(screen.content(), &BookContent::Journal);
    assert_eq!(screen.results(), &SearchResults::default());
}
