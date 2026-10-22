//! Book pages (journal-ui.md "Book"): blocks of text flowed into fixed-height pages.
//!
//! Text layout needs fonts, so it stays outside: a [`TextMeasure`] reports where lines break
//! (parley's line ranges on the Bevy side, [`FixedWidthMeasure`] in tests), and
//! [`LinePaginator`] packs those lines into pages with the CSS Fragmentation rules (orphans,
//! widows, headings kept with what follows). Screens depend on the [`Paginator`] trait only.

use std::ops::Range;

/// Text style of a block; the measure picks font and size per style.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BlockStyle {
    /// Name at the top of a quest or topic book.
    Title,
    /// Day heading, speaker name.
    Heading,
    /// Entry or heard text.
    Body,
}

impl BlockStyle {
    /// Titles and headings never end a page (`break-after: avoid`).
    pub fn keeps_with_next(self) -> bool {
        matches!(self, Self::Title | Self::Heading)
    }
}

/// What a block shows; a stable anchor across re-pagination.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BlockKey {
    Title,
    /// The day heading in front of the entry with this sequence number.
    DayBefore(u32),
    /// A journal entry, by sequence number.
    Entry(u32),
    /// Speaker of the heard line at this index (topic books).
    Speaker(usize),
    /// Heard line at this index (topic books).
    Heard(usize),
}

/// One paragraph of a book.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub key: BlockKey,
    pub style: BlockStyle,
    pub text: String,
    /// Shows a "new" marker.
    pub new: bool,
    /// Byte ranges to highlight (search hits), sorted and disjoint.
    pub highlights: Vec<Range<usize>>,
}

impl Block {
    pub fn new(key: BlockKey, style: BlockStyle, text: impl Into<String>) -> Self {
        Self {
            key,
            style,
            text: text.into(),
            new: false,
            highlights: Vec::new(),
        }
    }
}

/// The part of a block on one page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageSlice {
    /// Index into the book's blocks.
    pub block: usize,
    /// Byte range of the block's text. Starts at 0 on the block's first page, ends at its
    /// length on its last.
    pub range: Range<usize>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Page {
    pub slices: Vec<PageSlice>,
}

/// Flows blocks into pages. `sf_ui` has no paginator yet; when it gets one, it implements this.
pub trait Paginator {
    fn paginate(&self, blocks: &[Block]) -> Vec<Page>;
}

/// Line breaking for one page width.
pub trait TextMeasure {
    /// Visual lines of `text` in `style`: non-empty byte ranges, in order, contiguous, together
    /// covering the whole text (trailing spaces belong to the line they end). Empty text has
    /// no lines.
    fn lines(&self, text: &str, style: BlockStyle) -> Vec<Range<usize>>;

    /// Height of one line in `style`, in the page's units.
    fn line_height(&self, style: BlockStyle) -> f32;
}

/// Page size and fragmentation rules.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PageMetrics {
    /// Usable page height.
    pub height: f32,
    /// Space between two blocks on the same page (none at a page top).
    pub block_gap: f32,
    /// Fewest lines of a split block left at the bottom of a page.
    pub orphans: usize,
    /// Fewest lines of a split block carried to the top of the next page.
    pub widows: usize,
}

impl Default for PageMetrics {
    fn default() -> Self {
        // CSS Fragmentation 3 defaults for orphans and widows.
        Self {
            height: 600.0,
            block_gap: 0.0,
            orphans: 2,
            widows: 2,
        }
    }
}

/// The stock paginator: lines from a [`TextMeasure`], pages of [`PageMetrics::height`].
pub struct LinePaginator<M> {
    pub measure: M,
    pub metrics: PageMetrics,
}

impl<M: TextMeasure> LinePaginator<M> {
    pub fn new(measure: M, metrics: PageMetrics) -> Self {
        Self { measure, metrics }
    }

    /// Whole lines of height `line` that fit in `space`.
    fn fit(space: f32, line: f32) -> usize {
        if line <= 0.0 {
            return usize::MAX;
        }
        let n = (space / line + 1e-4).floor();
        if n <= 0.0 {
            0
        } else {
            // Bounded by the page height over the line height; never near usize::MAX.
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            {
                n as usize
            }
        }
    }

    /// Height block `j` needs at a page bottom so that nothing kept with it is stranded: a
    /// heading's lines plus the lead of what follows it, a body's first `orphans` lines (all of
    /// it when it is too short to split). `None` past the end or for an empty block.
    #[allow(clippy::cast_precision_loss)] // line counts are small
    fn lead(&self, blocks: &[Block], laid: &[Laid], j: usize) -> Option<f32> {
        let m = self.metrics;
        let l = laid.get(j).filter(|l| !l.lines.is_empty())?;
        let n = l.lines.len();
        Some(if blocks[j].style.keeps_with_next() {
            n as f32 * l.height
                + self
                    .lead(blocks, laid, j + 1)
                    .map_or(0.0, |f| m.block_gap + f)
        } else {
            let orphans = m.orphans.max(1);
            let first = if n >= orphans + m.widows { orphans } else { n };
            first as f32 * l.height
        })
    }
}

struct Laid {
    lines: Vec<Range<usize>>,
    height: f32,
}

#[allow(clippy::cast_precision_loss)] // line counts are small
impl<M: TextMeasure> Paginator for LinePaginator<M> {
    fn paginate(&self, blocks: &[Block]) -> Vec<Page> {
        let m = self.metrics;
        let laid: Vec<Laid> = blocks
            .iter()
            .map(|b| Laid {
                lines: self.measure.lines(&b.text, b.style),
                height: self.measure.line_height(b.style),
            })
            .collect();

        let mut pages = Vec::new();
        let mut page = Page::default();
        let mut y = 0.0_f32;
        let break_page = |page: &mut Page, pages: &mut Vec<Page>, y: &mut f32| {
            if !page.slices.is_empty() {
                pages.push(std::mem::take(page));
            }
            *y = 0.0;
        };

        for (i, block) in blocks.iter().enumerate() {
            let lines = &laid[i].lines;
            let h = laid[i].height;
            let mut first = 0; // first line not yet placed
            while first < lines.len() {
                let empty = page.slices.is_empty();
                let gap = if empty { 0.0 } else { m.block_gap };
                let room = Self::fit(m.height - y - gap, h);
                let left = lines.len() - first;

                let take = if left <= room {
                    // Fits whole. A heading also needs the start of what follows.
                    let follow = if block.style.keeps_with_next() {
                        self.lead(blocks, &laid, i + 1)
                            .map_or(0.0, |f| m.block_gap + f)
                    } else {
                        0.0
                    };
                    if !empty && y + gap + left as f32 * h + follow > m.height + 1e-4 {
                        0
                    } else {
                        left
                    }
                } else {
                    // Split: keep orphans here and widows there, else move the block on.
                    let k = room.min(left.saturating_sub(m.widows));
                    if k < m.orphans.max(1) { 0 } else { k }
                };

                // An empty page always takes something.
                let take = if take == 0 && empty {
                    left.min(room.max(1))
                } else {
                    take
                };
                if take == 0 {
                    break_page(&mut page, &mut pages, &mut y);
                    continue;
                }

                let start = lines[first].start;
                let end = lines[first + take - 1].end;
                // A block whose first line starts after 0 (leading text skipped by the measure)
                // still starts at 0 on its first page; likewise for the end.
                let start = if first == 0 { 0 } else { start };
                let end = if first + take == lines.len() {
                    block.text.len()
                } else {
                    end
                };
                page.slices.push(PageSlice {
                    block: i,
                    range: start..end,
                });
                y += gap + take as f32 * h;
                first += take;
                if first < lines.len() {
                    break_page(&mut page, &mut pages, &mut y);
                }
            }
        }
        if !page.slices.is_empty() {
            pages.push(page);
        }
        pages
    }
}

/// Greedy word wrap at a fixed number of characters per line (monospace text, tests, headless
/// tools). Every style has the same width and a line height of 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixedWidthMeasure {
    pub columns: usize,
}

impl TextMeasure for FixedWidthMeasure {
    fn lines(&self, text: &str, _style: BlockStyle) -> Vec<Range<usize>> {
        let width = self.columns.max(1);
        let mut out = Vec::new();
        let mut para_start = 0;
        for para in text.split_inclusive('\n') {
            wrap_paragraph(para, para_start, width, &mut out);
            para_start += para.len();
        }
        out
    }

    fn line_height(&self, _style: BlockStyle) -> f32 {
        1.0
    }
}

/// Wraps one paragraph (ends with its `\n`, if any) at `base` into `out`.
fn wrap_paragraph(para: &str, base: usize, width: usize, out: &mut Vec<Range<usize>>) {
    let chars: Vec<(usize, char)> = para.char_indices().collect();
    let mut line_start = 0; // index into `chars`
    while line_start < chars.len() {
        // Visible characters on this line: up to `width` non-trailing-space chars.
        let mut end = line_start; // exclusive, in chars
        let mut last_break = None; // char index just after a space run
        let mut count = 0;
        while end < chars.len() {
            let c = chars[end].1;
            if c == '\n' {
                end += 1;
                last_break = None;
                break;
            }
            if c.is_whitespace() {
                end += 1;
                // Spaces after a word may hang past the width.
                while end < chars.len() && chars[end].1.is_whitespace() && chars[end].1 != '\n' {
                    end += 1;
                }
                last_break = Some(end);
                count = end - line_start;
                continue;
            }
            if count >= width {
                break;
            }
            count += 1;
            end += 1;
        }
        let stop = if end < chars.len() && chars[end - 1].1 != '\n' {
            // Overflow: break after the last space, or hard inside a long word.
            last_break.filter(|&b| b > line_start).unwrap_or(end)
        } else {
            end
        };
        let from = chars[line_start].0;
        let to = chars.get(stop).map_or(para.len(), |c| c.0);
        out.push(base + from..base + to);
        line_start = stop;
    }
}
