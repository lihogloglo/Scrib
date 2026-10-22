//! Rich text and pagination on synthetic text with fake metrics.

use crate::*;

/// Fake text engine: every character advances `advance × size`, a line is
/// `line_height × size` tall (the tallest piece on it), greedy word wrap at
/// spaces, `'\n'` breaks, an inline image is a word of its own size. Every
/// size is multiplied by `scale` (the UI text scale).
struct Mono {
    advance: f32,
    line_height: f32,
    scale: f32,
}

const NATIVE_IMAGE: Size = Size {
    width: 64.0,
    height: 32.0,
};

impl Mono {
    fn new(scale: f32) -> Self {
        Self {
            advance: 1.0,
            line_height: 10.0,
            scale,
        }
    }

    fn image(&self, image: &ImageRef) -> Size {
        Size {
            width: image.width.unwrap_or(NATIVE_IMAGE.width) * self.scale,
            height: image.height.unwrap_or(NATIVE_IMAGE.height) * self.scale,
        }
    }
}

struct Atom {
    offset: usize,
    ch: char,
    advance: f32,
    height: f32,
}

struct Token {
    start: usize,
    word: f32,
    spaces: f32,
    height: f32,
    newline: bool,
}

impl TextMeasure for Mono {
    fn lines(&mut self, paragraph: &Paragraph, width: f32) -> Vec<LineMetrics> {
        let mut atoms = Vec::new();
        for (range, inline) in paragraph.pieces() {
            match inline {
                Inline::Text { text, style } => {
                    for (i, ch) in text.char_indices() {
                        atoms.push(Atom {
                            offset: range.start + i,
                            ch,
                            advance: self.advance * style.size * self.scale,
                            height: self.line_height * style.size * self.scale,
                        });
                    }
                }
                Inline::Image(image) => {
                    let size = self.image(image);
                    atoms.push(Atom {
                        offset: range.start,
                        ch: OBJECT_REPLACEMENT,
                        advance: size.width,
                        height: size.height,
                    });
                }
            }
        }
        let mut tokens = Vec::new();
        let mut i = 0;
        while i < atoms.len() {
            let mut t = Token {
                start: atoms[i].offset,
                word: 0.0,
                spaces: 0.0,
                height: 0.0,
                newline: false,
            };
            if atoms[i].ch == OBJECT_REPLACEMENT {
                t.word = atoms[i].advance;
                t.height = atoms[i].height;
                i += 1;
            } else {
                while i < atoms.len() && !matches!(atoms[i].ch, ' ' | '\n' | OBJECT_REPLACEMENT) {
                    t.word += atoms[i].advance;
                    t.height = t.height.max(atoms[i].height);
                    i += 1;
                }
            }
            while i < atoms.len() && atoms[i].ch == ' ' {
                t.spaces += atoms[i].advance;
                t.height = t.height.max(atoms[i].height);
                i += 1;
            }
            if i < atoms.len() && atoms[i].ch == '\n' {
                t.height = t.height.max(atoms[i].height);
                t.newline = true;
                i += 1;
            }
            tokens.push(t);
        }

        let len = paragraph.len();
        let empty_height = self.line_height * self.scale;
        let mut lines = Vec::new();
        let (mut start, mut x, mut height) = (0, 0.0, 0.0_f32);
        for t in &tokens {
            if x > 0.0 && x + t.word > width {
                lines.push(LineMetrics {
                    range: start..t.start,
                    height,
                });
                (start, x, height) = (t.start, 0.0, 0.0);
            }
            x += t.word + t.spaces;
            height = height.max(t.height);
            if t.newline {
                let end = tokens
                    .iter()
                    .map(|t| t.start)
                    .find(|&s| s > t.start)
                    .unwrap_or(len);
                lines.push(LineMetrics {
                    range: start..end,
                    height,
                });
                (start, x, height) = (end, 0.0, 0.0);
            }
        }
        if start < len || lines.is_empty() {
            lines.push(LineMetrics {
                range: start..len,
                height: if height > 0.0 { height } else { empty_height },
            });
        }
        lines
    }

    fn image_size(&mut self, image: &ImageRef) -> Size {
        self.image(image)
    }
}

fn para(text: &str) -> Block {
    let mut p = Paragraph::new(Align::Start);
    p.push_text(text, &RunStyle::default());
    Block::Paragraph(p)
}

/// `n` lines of exactly `width` characters each ("aaaa bbbb ..." words of 4).
fn lines_text(n: usize, width: usize) -> String {
    let words_per_line = (width + 1) / 5;
    (0..n * words_per_line)
        .map(|i| {
            char::from(b'a' + u8::try_from(i % 26).unwrap())
                .to_string()
                .repeat(4)
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn doc(blocks: Vec<Block>) -> RichDoc {
    RichDoc {
        blocks,
        background: None,
    }
}

/// Lines per placed fragment, page by page.
fn shape(p: &Pagination) -> Vec<Vec<(usize, usize)>> {
    p.pages()
        .iter()
        .map(|page| {
            page.items
                .iter()
                .map(|item| match &item.fragment {
                    Fragment::Lines { lines, .. } => (item.block, *lines),
                    Fragment::Image { .. } => (item.block, 0),
                })
                .collect()
        })
        .collect()
}

/// Every paragraph's fragments, in order, rebuild its text exactly; pages
/// never overflow unless a single item is taller than the page.
fn check_invariants(d: &RichDoc, p: &Pagination) {
    let height = p.format().height.unwrap_or(f32::INFINITY);
    let mut rebuilt = vec![String::new(); d.blocks.len()];
    let mut last = DocPos::default();
    for page in p.pages() {
        assert!(page.start >= last, "page starts go forward");
        last = page.start;
        if let Some(first) = page.items.first() {
            assert_eq!(page.start, first.start());
        }
        let overflow = page.content_height() > height + 1e-3;
        assert!(
            !overflow || page.items.len() == 1,
            "only a lone item overflows"
        );
        let mut y = 0.0;
        for item in &page.items {
            assert!(item.y >= y - 1e-3, "items stack downwards");
            y = item.y + item.height;
            if let Fragment::Lines { range, .. } = &item.fragment {
                let slice = d.paragraph(item.block).unwrap().slice(range.clone());
                rebuilt[item.block].push_str(&slice.text());
            }
        }
    }
    for (i, block) in d.blocks.iter().enumerate() {
        if let Block::Paragraph(par) = block {
            assert_eq!(
                rebuilt[i],
                par.text(),
                "block {i} rebuilt from its fragments"
            );
        }
    }
}

#[test]
fn paragraph_runs_and_slices() {
    let red = RunStyle {
        color: Some(Rgba8::rgb(200, 0, 0)),
        ..RunStyle::default()
    };
    let mut p = Paragraph::new(Align::Center);
    p.push_text("Hello ", &RunStyle::default());
    p.push_text("dark ", &red);
    p.push_text("élf", &red);
    p.push_image(ImageRef::new("art/seal.dds").with_size(16.0, 16.0));
    p.push_text("!", &RunStyle::default());
    assert_eq!(p.inlines.len(), 4, "same-style runs merge");
    assert_eq!(p.text(), "Hello dark élf\u{FFFC}!");
    assert_eq!(p.len(), p.text().len());

    let s = p.slice(6..15);
    assert_eq!(s.align, Align::Center);
    assert_eq!(s.text(), "dark élf", "a bound inside 'é' moves back");
    let with_image = p.slice(11..p.len());
    assert_eq!(with_image.text(), "élf\u{FFFC}!");
    assert!(matches!(with_image.inlines[1], Inline::Image(_)));
    assert_eq!(
        p.slice(16..18).text(),
        "",
        "a cut through the placeholder drops the image"
    );

    let mut d = doc(vec![Block::Paragraph(p), Block::PageBreak, para("Second.")]);
    d.push(Block::Image {
        image: ImageRef::new("art/map.dds"),
        align: Align::Center,
    });
    assert_eq!(d.plain_text(), "Hello dark élf!\nSecond.\n");
    assert!(d.paragraph(1).is_none());
}

#[test]
fn colours_parse_from_hex() {
    assert_eq!(Rgba8::from_hex("000000"), Some(Rgba8::rgb(0, 0, 0)));
    assert_eq!(Rgba8::from_hex("#FF8000"), Some(Rgba8::rgb(255, 128, 0)));
    assert_eq!(
        Rgba8::from_hex("10203040"),
        Some(Rgba8 {
            r: 0x10,
            g: 0x20,
            b: 0x30,
            a: 0x40
        })
    );
    for bad in ["", "FFF", "GG0000", "#12345", "ééé"] {
        assert_eq!(Rgba8::from_hex(bad), None, "{bad:?}");
    }
}

#[test]
fn fake_metrics_wrap_like_a_text_engine() {
    let mut m = Mono::new(1.0);
    let Block::Paragraph(p) = para("aaaa bbbb cccc\ndd") else {
        unreachable!()
    };
    let lines = m.lines(&p, 10.0);
    let ranges: Vec<_> = lines.iter().map(|l| l.range.clone()).collect();
    assert_eq!(ranges, vec![0..10, 10..15, 15..17]);
    assert_eq!(
        m.lines(&Paragraph::default(), 10.0).len(),
        1,
        "empty = one blank line"
    );
}

#[test]
fn scroll_is_one_page_and_ignores_breaks() {
    let d = doc(vec![
        para(&lines_text(30, 24)),
        Block::PageBreak,
        para(&lines_text(20, 24)),
    ]);
    let p = Pagination::paginate(
        &d,
        &mut Mono::new(1.0),
        PageFormat::scroll(24.0).with_gap(5.0),
    );
    assert_eq!(shape(&p), vec![vec![(0, 30), (2, 20)]]);
    let page = &p.pages()[0];
    assert!((page.items[1].y - 305.0).abs() < 1e-3, "gap between blocks");
    assert!((page.content_height() - 505.0).abs() < 1e-3);
    check_invariants(&d, &p);
}

#[test]
fn pages_fill_and_split_between_lines() {
    // 24 wide, 100 tall: 10 lines a page.
    let d = doc(vec![para(&lines_text(25, 24))]);
    let p = Pagination::paginate(&d, &mut Mono::new(1.0), PageFormat::page(24.0, 100.0));
    assert_eq!(shape(&p), vec![vec![(0, 10)], vec![(0, 10)], vec![(0, 5)]]);
    check_invariants(&d, &p);
}

#[test]
fn orphans_and_widows() {
    // 5 lines a page; A takes 3, leaving room for 2.
    let format = PageFormat::page(24.0, 50.0);
    let split = |b_lines, orphans, widows| {
        let d = doc(vec![
            para(&lines_text(3, 24)),
            para(&lines_text(b_lines, 24)),
        ]);
        let p = Pagination::paginate(
            &d,
            &mut Mono::new(1.0),
            PageFormat {
                orphans,
                widows,
                ..format
            },
        );
        check_invariants(&d, &p);
        shape(&p)
    };
    // 4 lines: 2 stay (orphans 2), 2 move (widows 2).
    assert_eq!(split(4, 2, 2), vec![vec![(0, 3), (1, 2)], vec![(1, 2)]]);
    // 3 lines: keeping 2 would leave 1 widow, keeping 1 an orphan: all move.
    assert_eq!(split(3, 2, 2), vec![vec![(0, 3)], vec![(1, 3)]]);
    // With the rules off the page fills.
    assert_eq!(split(3, 1, 1), vec![vec![(0, 3), (1, 2)], vec![(1, 1)]]);
    // 6 lines: 2 stay, 4 move (no widow problem).
    assert_eq!(split(6, 2, 2), vec![vec![(0, 3), (1, 2)], vec![(1, 4)]]);
    // Widows pull a line back: 6 lines on 5-line pages from an empty page.
    let d = doc(vec![para(&lines_text(6, 24))]);
    let p = Pagination::paginate(&d, &mut Mono::new(1.0), format);
    assert_eq!(shape(&p), vec![vec![(0, 4)], vec![(0, 2)]]);
}

#[test]
fn forced_breaks_never_make_blank_pages() {
    let d = doc(vec![
        Block::PageBreak,
        para("one"),
        Block::PageBreak,
        Block::PageBreak,
        para("two"),
        Block::PageBreak,
    ]);
    let p = Pagination::paginate(&d, &mut Mono::new(1.0), PageFormat::page(24.0, 100.0));
    assert_eq!(shape(&p), vec![vec![(1, 1)], vec![(4, 1)]]);

    let empty = Pagination::paginate(
        &RichDoc::default(),
        &mut Mono::new(1.0),
        PageFormat::page(24.0, 100.0),
    );
    assert_eq!(empty.pages().len(), 1, "an empty book still has a page");
    assert_eq!(empty.view_count(PageView::Spread), 1);
}

#[test]
fn block_images_fit_the_page_and_never_split() {
    let format = PageFormat::page(40.0, 100.0).with_gap(2.0);
    let d = doc(vec![
        para(&lines_text(6, 40)),
        Block::Image {
            image: ImageRef::new("big").with_size(40.0, 50.0),
            align: Align::Center,
        },
        Block::Image {
            image: ImageRef::new("huge").with_size(80.0, 400.0),
            align: Align::Center,
        },
    ]);
    let p = Pagination::paginate(&d, &mut Mono::new(1.0), format);
    // 60 + 2 + 50 > 100: the image moves on; the huge one is scaled to the page.
    assert_eq!(shape(&p), vec![vec![(0, 6)], vec![(1, 0)], vec![(2, 0)]]);
    let Fragment::Image { size } = &p.pages()[2].items[0].fragment else {
        unreachable!()
    };
    assert_eq!(
        *size,
        Size {
            width: 20.0,
            height: 100.0
        }
    );
    let Fragment::Image { size } = &p.pages()[1].items[0].fragment else {
        unreachable!()
    };
    assert!((size.height - 50.0).abs() < 1e-6, "fits: not scaled");
    check_invariants(&d, &p);
}

#[test]
fn inline_images_and_sizes_set_line_heights() {
    let mut p = Paragraph::new(Align::Start);
    p.push_text("small ", &RunStyle::default());
    p.push_image(ImageRef::new("icon").with_size(8.0, 30.0));
    p.push_text(" text", &RunStyle::default());
    let big = RunStyle {
        size: 2.0,
        family: Some("handwritten".into()),
        ..RunStyle::default()
    };
    let mut q = Paragraph::new(Align::End);
    q.push_text("BIG", &big);
    let d = doc(vec![Block::Paragraph(p), Block::Paragraph(q), para("")]);
    let pages = Pagination::paginate(&d, &mut Mono::new(1.0), PageFormat::scroll(100.0));
    let heights: Vec<f32> = pages.pages()[0].items.iter().map(|i| i.height).collect();
    assert_eq!(
        heights,
        vec![30.0, 20.0, 10.0],
        "image line, 2 em line, blank line"
    );
    check_invariants(&d, &pages);
}

#[test]
fn a_line_taller_than_the_page_goes_alone() {
    let mut p = Paragraph::new(Align::Start);
    p.push_image(ImageRef::new("tall").with_size(8.0, 300.0));
    let d = doc(vec![para("before"), Block::Paragraph(p), para("after")]);
    let pages = Pagination::paginate(&d, &mut Mono::new(1.0), PageFormat::page(24.0, 100.0));
    assert_eq!(
        shape(&pages),
        vec![vec![(0, 1)], vec![(1, 1)], vec![(2, 1)]]
    );
    check_invariants(&d, &pages);
}

#[test]
fn spreads_and_single_pages() {
    // 7 pages of 10 lines.
    let d = doc(vec![para(&lines_text(70, 24))]);
    let p = Pagination::paginate(&d, &mut Mono::new(1.0), PageFormat::page(24.0, 100.0));
    assert_eq!(p.pages().len(), 7);
    assert_eq!(p.view_count(PageView::Spread), 4);
    assert_eq!(p.view(0, PageView::Spread).len(), 2);
    assert_eq!(
        p.view(3, PageView::Spread).len(),
        1,
        "last spread: left page only"
    );
    assert!(p.view(4, PageView::Spread).is_empty());
    assert_eq!(p.view_count(PageView::Single), 7);
    assert_eq!(p.view(5, PageView::Single), &p.pages()[5..6]);
    let pos = p.pages()[5].items[0].start();
    assert_eq!(p.page_of(pos), 5);
    assert_eq!(p.view_of(pos, PageView::Spread), 2);
    assert_eq!(p.view_of(pos, PageView::Single), 5);
}

#[test]
fn reading_position_survives_rescaling() {
    let mut blocks = Vec::new();
    for i in 0..12 {
        blocks.push(para(&lines_text(3 + i % 5, 30)));
        if i == 6 {
            blocks.push(Block::PageBreak);
        }
    }
    let d = doc(blocks);
    let format = PageFormat::page(30.0, 120.0).with_gap(4.0);
    let small = Pagination::paginate(&d, &mut Mono::new(1.0), format);
    let large = Pagination::paginate(&d, &mut Mono::new(1.5), format);
    check_invariants(&d, &small);
    check_invariants(&d, &large);
    assert!(large.pages().len() > small.pages().len());

    // The reader is on spread 2 at 100 %; switch to 150 %.
    let bookmark = small.view(2, PageView::Spread)[0].start;
    let page = large.page_of(bookmark);
    let here = &large.pages()[page];
    assert!(here.start <= bookmark);
    if let Some(next) = large.pages().get(page + 1) {
        assert!(bookmark < next.start, "the bookmark is on the page we open");
    }
    // The bookmarked text is drawn on that page.
    let shown = here.items.iter().any(|item| match &item.fragment {
        Fragment::Lines { range, .. } => {
            item.block == bookmark.block && range.contains(&bookmark.offset)
        }
        Fragment::Image { .. } => item.block == bookmark.block,
    });
    assert!(shown);
}

#[test]
fn random_documents_keep_every_line() {
    // Synthetic books of mixed blocks at several formats.
    let mut rng = Lcg(0x2545_f491);
    for _ in 0..40 {
        let mut blocks = Vec::new();
        for _ in 0..rng.below(20) {
            match rng.below(10) {
                0 => blocks.push(Block::PageBreak),
                1 => blocks.push(Block::Image {
                    image: ImageRef::new("img")
                        .with_size(10.0 + 10.0 * rng.float(8), 10.0 + 20.0 * rng.float(8)),
                    align: Align::Center,
                }),
                _ => {
                    let mut p = Paragraph::new(Align::Start);
                    for _ in 0..=rng.below(6) {
                        if rng.below(5) == 0 {
                            p.push_image(
                                ImageRef::new("glyph").with_size(6.0, 6.0 + rng.float(20)),
                            );
                        } else {
                            let style = RunStyle {
                                size: [1.0, 1.0, 1.5, 0.8][rng.below(4)],
                                ..RunStyle::default()
                            };
                            let mut text = lines_text(rng.below(4), 20);
                            if rng.below(3) == 0 {
                                text.push('\n');
                            }
                            text.push_str(" é ");
                            p.push_text(&text, &style);
                        }
                    }
                    blocks.push(Block::Paragraph(p));
                }
            }
        }
        let d = doc(blocks);
        for format in [
            PageFormat::page(30.0, 80.0),
            PageFormat::page(50.0, 140.0).with_gap(6.0),
            PageFormat {
                orphans: 1,
                widows: 3,
                ..PageFormat::page(25.0, 60.0)
            },
            PageFormat::scroll(40.0),
        ] {
            let p = Pagination::paginate(&d, &mut Mono::new(1.25), format);
            check_invariants(&d, &p);
        }
    }
}

/// Deterministic pseudo-random numbers (Knuth's MMIX LCG) for synthetic books.
struct Lcg(u64);

impl Lcg {
    fn below(&mut self, n: u64) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        usize::try_from((self.0 >> 33) % n).unwrap()
    }

    fn float(&mut self, n: u64) -> f32 {
        f32::from(u16::try_from(self.below(n)).unwrap())
    }
}
