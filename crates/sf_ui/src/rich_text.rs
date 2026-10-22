//! Rich-text documents for books, scrolls and notes (ui-text.md "Model").
//!
//! A [`RichDoc`] is a list of [`Block`]s: paragraphs of styled runs and
//! inline images, block images, and forced page breaks. A content pack's
//! markup parser builds one; [`Pagination`](crate::Pagination) cuts it into
//! pages; the Bevy side draws each page fragment as one text node per
//! paragraph slice ([`Paragraph::slice`]).
//!
//! Positions inside a paragraph are byte offsets into its *flattened text*
//! ([`Paragraph::text`]): the runs' text in order, with one
//! [`OBJECT_REPLACEMENT`] character standing for each inline image (as
//! `NSAttributedString` text attachments do), so every image has its own
//! offset and a line range says unambiguously which images it holds.

use std::ops::Range;

/// U+FFFC, the placeholder for an inline image in [`Paragraph::text`].
pub const OBJECT_REPLACEMENT: char = '\u{FFFC}';

/// An sRGB colour with straight alpha.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rgba8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba8 {
    /// An opaque colour.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Parses `RRGGBB` or `RRGGBBAA` hex digits, with or without a leading `#`.
    pub fn from_hex(hex: &str) -> Option<Self> {
        let digits = hex.trim().strip_prefix('#').unwrap_or(hex.trim());
        if !digits.is_ascii() || !matches!(digits.len(), 6 | 8) {
            return None;
        }
        let byte = |i: usize| u8::from_str_radix(&digits[i..i + 2], 16).ok();
        let a = if digits.len() == 8 { byte(6)? } else { 255 };
        Some(Self {
            r: byte(0)?,
            g: byte(2)?,
            b: byte(4)?,
            a,
        })
    }
}

/// Horizontal alignment of a paragraph or block image. `Start` and `End`
/// follow the text direction (left and right in left-to-right scripts).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
    Justify,
}

/// How a run of text looks. Unset fields fall back to the theme (CSS).
#[derive(Clone, Debug, PartialEq)]
pub struct RunStyle {
    /// Theme font family key (a CSS family-list name such as `handwritten`);
    /// `None` is the theme's body face. The theme maps keys to font files.
    pub family: Option<String>,
    /// Size as a multiple of the body size (CSS `em`).
    pub size: f32,
    /// `None` is the theme's text colour.
    pub color: Option<Rgba8>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

impl Default for RunStyle {
    fn default() -> Self {
        Self {
            family: None,
            size: 1.0,
            color: None,
            bold: false,
            italic: false,
            underline: false,
        }
    }
}

/// An image placed in a document. The source is resolved by the content
/// pack (a VFS path for the asset loader).
#[derive(Clone, Debug, PartialEq)]
pub struct ImageRef {
    pub source: String,
    /// Width in document units; `None` derives it from the image (its native
    /// size, or the aspect ratio when only the height is given).
    pub width: Option<f32>,
    /// Height in document units, as `width`.
    pub height: Option<f32>,
}

impl ImageRef {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            width: None,
            height: None,
        }
    }

    #[must_use]
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
}

/// One piece of a paragraph.
#[derive(Clone, Debug, PartialEq)]
pub enum Inline {
    /// Styled text. `'\n'` is a hard line break inside the paragraph.
    Text { text: String, style: RunStyle },
    /// An image that flows with the text, like a glyph.
    Image(ImageRef),
}

impl Inline {
    /// Length of this piece in the flattened text.
    pub fn len(&self) -> usize {
        match self {
            Self::Text { text, .. } => text.len(),
            Self::Image(_) => OBJECT_REPLACEMENT.len_utf8(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A block of text laid out as one unit: lines wrap inside it, and it splits
/// across pages only between lines.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Paragraph {
    pub align: Align,
    pub inlines: Vec<Inline>,
}

impl Paragraph {
    pub fn new(align: Align) -> Self {
        Self {
            align,
            inlines: Vec::new(),
        }
    }

    /// Appends text, merging it into the last run when the style is the same.
    pub fn push_text(&mut self, text: &str, style: &RunStyle) {
        if text.is_empty() {
            return;
        }
        if let Some(Inline::Text {
            text: last,
            style: s,
        }) = self.inlines.last_mut()
            && s == style
        {
            last.push_str(text);
            return;
        }
        self.inlines.push(Inline::Text {
            text: text.to_owned(),
            style: style.clone(),
        });
    }

    pub fn push_image(&mut self, image: ImageRef) {
        self.inlines.push(Inline::Image(image));
    }

    /// Length of the flattened text in bytes.
    pub fn len(&self) -> usize {
        self.inlines.iter().map(Inline::len).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.inlines.iter().all(Inline::is_empty)
    }

    /// The flattened text: runs in order, [`OBJECT_REPLACEMENT`] per image.
    pub fn text(&self) -> String {
        let mut out = String::with_capacity(self.len());
        for inline in &self.inlines {
            match inline {
                Inline::Text { text, .. } => out.push_str(text),
                Inline::Image(_) => out.push(OBJECT_REPLACEMENT),
            }
        }
        out
    }

    /// Each piece with its byte range in the flattened text.
    pub fn pieces(&self) -> impl Iterator<Item = (Range<usize>, &Inline)> {
        let mut at = 0;
        self.inlines.iter().map(move |inline| {
            let start = at;
            at += inline.len();
            (start..at, inline)
        })
    }

    /// The part of this paragraph inside `range` of the flattened text (a
    /// page fragment), same alignment. Text runs are clipped (bounds that
    /// fall inside a character move back to its start); an image is kept
    /// when its placeholder starts inside the range.
    #[must_use]
    pub fn slice(&self, range: Range<usize>) -> Self {
        let mut out = Self::new(self.align);
        for (span, inline) in self.pieces() {
            let start = range.start.max(span.start);
            let end = range.end.min(span.end);
            if start >= end {
                continue;
            }
            match inline {
                Inline::Text { text, style } => {
                    let a = text.floor_char_boundary(start - span.start);
                    let b = text.floor_char_boundary(end - span.start);
                    out.push_text(&text[a..b], style);
                }
                Inline::Image(image) if start == span.start => out.push_image(image.clone()),
                Inline::Image(_) => {}
            }
        }
        out
    }
}

/// A top-level piece of a document.
#[derive(Clone, Debug, PartialEq)]
pub enum Block {
    Paragraph(Paragraph),
    /// An image on its own, never split; scaled down to fit the page.
    Image {
        image: ImageRef,
        align: Align,
    },
    /// Forced page break (paged layouts only; a scroll ignores it).
    PageBreak,
}

/// A book, scroll or note: blocks in reading order.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RichDoc {
    pub blocks: Vec<Block>,
    /// Image drawn behind the page(s), if the source format names one.
    pub background: Option<ImageRef>,
}

impl RichDoc {
    pub fn push(&mut self, block: Block) {
        self.blocks.push(block);
    }

    /// The paragraph at `block`, if that block is one.
    pub fn paragraph(&self, block: usize) -> Option<&Paragraph> {
        match self.blocks.get(block) {
            Some(Block::Paragraph(p)) => Some(p),
            _ => None,
        }
    }

    /// The text alone, one line per paragraph, images dropped: for screen
    /// readers, search and logs.
    pub fn plain_text(&self) -> String {
        let mut out = String::new();
        for block in &self.blocks {
            if let Block::Paragraph(p) = block {
                for inline in &p.inlines {
                    if let Inline::Text { text, .. } = inline {
                        out.push_str(text);
                    }
                }
                out.push('\n');
            }
        }
        out
    }
}
