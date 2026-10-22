//! Page breaking for rich-text documents (ui-text.md "Paginator").
//!
//! Line breaking belongs to the text engine that draws the text (parley in
//! Bevy), so pages always match what is drawn: the paginator asks a
//! [`TextMeasure`] for each paragraph's lines at the page width and only
//! decides where pages break. Rules follow CSS Fragmentation: paragraphs
//! split between lines, with `orphans` and `widows` minimums; images are
//! monolithic; a forced break starts a new page; nothing is lost when a
//! rule can't be met (a too-tall line or image goes alone on a page).

use std::ops::Range;

use crate::rich_text::{Block, ImageRef, Paragraph, RichDoc};

/// Width and height in layout units (whatever the measurer uses: physical
/// pixels at the current UI scale on the Bevy side).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

/// One laid-out line of a paragraph.
#[derive(Clone, Debug, PartialEq)]
pub struct LineMetrics {
    /// Byte range in the paragraph's flattened text
    /// ([`Paragraph::text`]). A paragraph's lines are contiguous and cover
    /// all of it, including trailing spaces and the `'\n'` that ends a line.
    pub range: Range<usize>,
    /// Line box height, inline images included.
    pub height: f32,
}

/// Text measurement: the text engine's line layout, behind a trait so the
/// paginator stays engine-free and testable.
pub trait TextMeasure {
    /// Lays `paragraph` out at `width` and returns its lines in order. An
    /// empty paragraph is one empty line (a blank line in the book).
    fn lines(&mut self, paragraph: &Paragraph, width: f32) -> Vec<LineMetrics>;

    /// The drawn size of a block image (its [`ImageRef`] size scaled, or the
    /// image's own size when the reference has none). The paginator scales
    /// it down further to fit the page.
    fn image_size(&mut self, image: &ImageRef) -> Size;
}

/// Page geometry and break rules.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PageFormat {
    /// Content width of one page.
    pub width: f32,
    /// Content height of one page; `None` is a scroll (one page, as long as
    /// the text, forced breaks ignored).
    pub height: Option<f32>,
    /// Vertical space between blocks on the same page (dropped at a page top).
    pub block_gap: f32,
    /// Minimum lines of a split paragraph left at the bottom of a page.
    pub orphans: usize,
    /// Minimum lines of a split paragraph carried to the top of the next page.
    pub widows: usize,
}

impl PageFormat {
    /// A book page, with the CSS defaults of 2 orphans and 2 widows.
    pub fn page(width: f32, height: f32) -> Self {
        Self {
            width,
            height: Some(height),
            block_gap: 0.0,
            orphans: 2,
            widows: 2,
        }
    }

    /// A scroll: one page of unbounded height.
    pub fn scroll(width: f32) -> Self {
        Self {
            height: None,
            ..Self::page(width, 0.0)
        }
    }

    #[must_use]
    pub fn with_gap(mut self, block_gap: f32) -> Self {
        self.block_gap = block_gap;
        self
    }
}

/// A position in a document: block index, then byte offset in the
/// paragraph's flattened text (0 for other blocks). Ordered in reading
/// order. The reader's bookmark: unlike a page number it survives
/// re-pagination at another text scale or font.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DocPos {
    pub block: usize,
    pub offset: usize,
}

/// What a placed item draws.
#[derive(Clone, Debug, PartialEq)]
pub enum Fragment {
    /// Consecutive lines of the paragraph at `block`: draw
    /// `paragraph.slice(range)` at the page width.
    Lines { range: Range<usize>, lines: usize },
    /// The block image at `block`, at this size (already fitted to the page).
    Image { size: Size },
}

/// One fragment on a page.
#[derive(Clone, Debug, PartialEq)]
pub struct Placed {
    pub block: usize,
    /// Top of the fragment from the page's content top.
    pub y: f32,
    pub height: f32,
    pub fragment: Fragment,
}

impl Placed {
    /// Where this fragment starts in the document.
    pub fn start(&self) -> DocPos {
        let offset = match &self.fragment {
            Fragment::Lines { range, .. } => range.start,
            Fragment::Image { .. } => 0,
        };
        DocPos {
            block: self.block,
            offset,
        }
    }
}

/// One page of a pagination.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Page {
    /// Where the page's content starts (default for an empty document).
    pub start: DocPos,
    pub items: Vec<Placed>,
}

impl Page {
    /// Height used by the content.
    pub fn content_height(&self) -> f32 {
        self.items.last().map_or(0.0, |p| p.y + p.height)
    }
}

/// How many pages one view shows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PageView {
    /// One page at a time (controller preset, notes).
    Single,
    /// Two facing pages, left then right (a book spread).
    #[default]
    Spread,
}

impl PageView {
    pub fn pages(self) -> usize {
        match self {
            Self::Single => 1,
            Self::Spread => 2,
        }
    }
}

/// A document cut into pages for one [`PageFormat`]. Rebuild it when the
/// format, text scale or fonts change, and find the reader's place again
/// with [`Pagination::page_of`].
#[derive(Clone, Debug, PartialEq)]
pub struct Pagination {
    format: PageFormat,
    pages: Vec<Page>,
}

/// Slack for float sums when checking whether lines fit.
const FIT_EPSILON: f32 = 1e-3;

impl Pagination {
    /// Cuts `doc` into pages. There is always at least one page.
    pub fn paginate(doc: &RichDoc, measure: &mut impl TextMeasure, format: PageFormat) -> Self {
        let mut b = Builder {
            format,
            pages: Vec::new(),
            current: Page::default(),
            y: 0.0,
        };
        for (index, block) in doc.blocks.iter().enumerate() {
            match block {
                Block::Paragraph(p) => {
                    let lines = measure.lines(p, format.width);
                    b.paragraph(index, &lines);
                }
                Block::Image { image, .. } => {
                    let size = measure.image_size(image);
                    b.image(index, size);
                }
                Block::PageBreak => {
                    if format.height.is_some() {
                        b.break_page();
                    }
                }
            }
        }
        if !b.current.items.is_empty() || b.pages.is_empty() {
            b.pages.push(b.current);
        }
        Self {
            format,
            pages: b.pages,
        }
    }

    pub fn format(&self) -> &PageFormat {
        &self.format
    }

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }

    /// The page holding `pos`: the last page starting at or before it.
    pub fn page_of(&self, pos: DocPos) -> usize {
        self.pages
            .partition_point(|p| p.start <= pos)
            .saturating_sub(1)
    }

    /// Number of views (spreads or single pages).
    pub fn view_count(&self, view: PageView) -> usize {
        self.pages.len().div_ceil(view.pages())
    }

    /// The pages of view `index` (the last spread may hold one page); empty
    /// past the end.
    pub fn view(&self, index: usize, view: PageView) -> &[Page] {
        let n = view.pages();
        let start = (index * n).min(self.pages.len());
        let end = (start + n).min(self.pages.len());
        &self.pages[start..end]
    }

    /// The view holding `pos`.
    pub fn view_of(&self, pos: DocPos, view: PageView) -> usize {
        self.page_of(pos) / view.pages()
    }
}

struct Builder {
    format: PageFormat,
    pages: Vec<Page>,
    current: Page,
    /// Next free y on the current page.
    y: f32,
}

impl Builder {
    fn remaining(&self) -> f32 {
        self.format.height.map_or(f32::INFINITY, |h| h - self.y)
    }

    fn page_empty(&self) -> bool {
        self.current.items.is_empty()
    }

    /// Gap before a new block at the current position.
    fn gap(&self) -> f32 {
        if self.page_empty() {
            0.0
        } else {
            self.format.block_gap
        }
    }

    /// Ends the current page; a no-op on an empty page, so breaks never make
    /// blank pages.
    fn break_page(&mut self) {
        if !self.page_empty() {
            self.pages.push(std::mem::take(&mut self.current));
            self.y = 0.0;
        }
    }

    fn place(&mut self, block: usize, height: f32, fragment: Fragment, new_block: bool) {
        if new_block {
            self.y += self.gap();
        }
        let placed = Placed {
            block,
            y: self.y,
            height,
            fragment,
        };
        if self.page_empty() {
            self.current.start = placed.start();
        }
        self.y += height;
        self.current.items.push(placed);
    }

    fn paragraph(&mut self, block: usize, lines: &[LineMetrics]) {
        let mut next = 0;
        let mut new_block = true;
        while next < lines.len() {
            let gap = if new_block { self.gap() } else { 0.0 };
            let room = self.remaining() - gap + FIT_EPSILON;
            let rest = &lines[next..];
            let mut used = 0.0;
            let fit = rest
                .iter()
                .take_while(|l| {
                    used += l.height;
                    used <= room
                })
                .count();
            let mut take = fit;
            if take < rest.len() {
                take = take.min(rest.len().saturating_sub(self.format.widows));
                if take < self.format.orphans.max(1) {
                    take = 0;
                }
            }
            if take == 0 {
                if !self.page_empty() {
                    self.break_page();
                    continue;
                }
                // A fresh page can't do better: break where it fills, keep
                // at least one line so the text always advances.
                take = fit.max(1);
            }
            let taken = &rest[..take];
            let range = taken[0].range.start..taken[take - 1].range.end;
            let height = taken.iter().map(|l| l.height).sum();
            self.place(
                block,
                height,
                Fragment::Lines { range, lines: take },
                new_block,
            );
            next += take;
            new_block = false;
            if next < lines.len() {
                self.break_page();
            }
        }
    }

    fn image(&mut self, block: usize, size: Size) {
        let size = fit_image(size, self.format.width, self.format.height);
        if size.height > self.remaining() - self.gap() + FIT_EPSILON {
            self.break_page();
        }
        self.place(block, size.height, Fragment::Image { size }, true);
    }
}

/// Scales `size` down, keeping its aspect ratio, to fit `width` x `height`.
fn fit_image(size: Size, width: f32, height: Option<f32>) -> Size {
    let mut scale: f32 = 1.0;
    if size.width > width && size.width > 0.0 {
        scale = scale.min(width / size.width);
    }
    if let Some(h) = height
        && size.height > h
        && size.height > 0.0
    {
        scale = scale.min(h / size.height);
    }
    Size {
        width: size.width * scale,
        height: size.height * scale,
    }
}
