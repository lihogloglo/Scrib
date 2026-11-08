//! Book and scroll markup to [`RichDoc`] (ui-adapter.md "Books and scrolls: markup → rich
//! text").
//!
//! A BOOK record's text is simplified HTML. [`parse`] reads it with a tolerant tag tokenizer
//! (no HTML engine) and returns the document plus a [`BookReport`] (tag counts, dropped tags,
//! repairs). It never panics: every input yields a document.
//!
//! # Provenance
//!
//! Ported from `OpenMW` (GPL-3.0), commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`
//! (github.com/OpenMW/openmw): `apps/openmw/mwgui/formatting.cpp` (`BookTextParser::next`,
//! `parseTag`, `BookFormatter::markupToWidget`, `handleDiv`, `handleFont`),
//! `components/misc/resourcehelpers.cpp` (`correctBookartPath`, `correctResourcePath`) and
//! `components/fontloader/fontloader.cpp` (`getFontForFace`).
//!
//! # Tokenizer (as `OpenMW`)
//!
//! - `'\r'` is removed. Tag names and attribute keys are ASCII case-insensitive; values keep
//!   their case. A tag runs from `<` to the first `>` (quotes do not protect a `>`).
//! - Attributes: `KEY=value` (up to whitespace), `KEY="value"` or `KEY='value'`; the character
//!   after a closing quote is skipped. A repeated key keeps the last value. A closing tag
//!   (`</FONT>`) has no attributes.
//! - Whitespace is not collapsed: spaces and tabs are text, and a `'\n'` in the text is a hard
//!   line break. The only newlines dropped are those right after a `<BR>` or `<P>` (any number)
//!   and at the very start of the text.
//! - `<BR>` adds one line break and `<P>` two (a blank line), except before the first text,
//!   image or page break, where they add nothing. Closing `</BR>` and `</P>` count the same,
//!   as in `OpenMW`.
//! - Vanilla shows no text after the last `<BR>` or `<P>` (literally those, case-insensitive):
//!   [`parse`] drops it, tags there still apply ([`BookOptions::trim_after_last_break`];
//!   a text with neither shows nothing, as in `OpenMW`). Dropped text is measured in
//!   [`BookReport::dropped_tail_bytes`].
//! - `[pagebreak]` followed by a newline (case-sensitive; an `OpenMW` addition) is a forced
//!   page break.
//! - Entities (`&amp;`) are not decoded (neither does `OpenMW`); they are counted.
//! - `%PCName`-style defines are not expanded here: the caller substitutes them first, as
//!   `OpenMW` does (`Interpreter::fixDefinesBook`) before parsing.
//!
//! # Blocks and lines
//!
//! `OpenMW` cuts the text at every `<DIV>`, `<P>` with attributes, `<IMG>`, `<FONT>` and page
//! break and stacks each piece as its own text box, trimming one newline at the cut so line
//! structure matches vanilla (`markupToWidget`). This port keeps that rule for the block tags
//! (`<DIV>` and `</DIV>`, `<P ...>` with attributes, block images, page breaks) and the same
//! trim, but treats `<FONT>` as an inline style change: `OpenMW`'s own comment says vanilla
//! flows text across tags, and the IR has styled runs. A `<FONT>` in mid-line, where `OpenMW`
//! breaks the line, is counted in [`BookReport::inline_font_changes`].
//!
//! Each piece becomes paragraphs: consecutive non-empty lines form one [`Paragraph`] (lines
//! joined by `'\n'`, the IR's hard break), and each blank line is an empty paragraph (the
//! paginator lays it out as one empty line). Laid out with no gap between blocks, the result
//! has `OpenMW`'s lines. A piece that is empty after trimming still yields one empty line when
//! `OpenMW` would draw its box (after a break or an image).
//!
//! # Attribute mapping
//!
//! - `<DIV ALIGN>` and `<P ALIGN>`: `LEFT` → [`Align::Start`], `CENTER` → [`Align::Center`],
//!   `RIGHT` → [`Align::End`], case-insensitive; other values keep the alignment. It applies
//!   from the next block on and persists (`</DIV>` does not reset it).
//! - `<FONT COLOR>`: leading hex digits (an optional `#` skipped, a tolerant addition), the low
//!   24 bits as `RRGGBB`; no digits or overflow gives black (`std::from_chars` leaves 0).
//! - `<FONT FACE>`: `Daedric` (case-insensitive) → family key [`FAMILY_DAEDRIC`] (`OpenMW`'s
//!   `ScrollFont`); every other face → `None`, the theme's body face (`OpenMW`'s
//!   `DefaultFont`, vanilla "Magic Cards").
//! - `<FONT SIZE>`: `OpenMW` ignores it (a `\todo`). Mapped here as HTML's legacy font size
//!   ([`legacy_font_size`]): `1`-`7` or `+n`/`-n` relative to 3, clamped, to CSS
//!   absolute-size factors `0.75, 8/9, 1, 1.2, 1.5, 2, 3` em (HTML Standard "rules for parsing
//!   a legacy font size"; CSS Fonts 4 absolute-size table). Invalid values keep the size.
//! - `</FONT>` resets every font property to the default (no stack, as `OpenMW`); a nested
//!   `<FONT>` changes only the attributes it names.
//! - `<IMG SRC WIDTH HEIGHT>`: sizes are integers (`MyGUI::utility::parseInt`: surrounding
//!   whitespace, optional sign; anything else is 0). A `bookart` image needs both sizes > 0,
//!   else it is skipped (counted); its path follows [`bookart_path`]. An `img://` source is a
//!   VFS path taken as is; without a width it is an inline image (50 wide, height or 50) at
//!   the tag's place in the text (`OpenMW` puts it at the start of the next text box), with a
//!   width a block image (height or 50). Block images take the current alignment.
//! - `<BODY BACKGROUND>`: the document background, path as `<IMG SRC>`, no size (`OpenMW` MR
//!   2064; not in the commit above, where `<BODY>` is an unknown tag).
//! - Any other tag (and its closing form) is dropped and counted in
//!   [`BookReport::unknown_tags`].
//!
//! # Repairs (where `OpenMW` throws and shows no text)
//!
//! - A `<` with no `>` after it is text ([`BookReport::unterminated_tag`]).
//! - A quoted value with no closing quote runs to the end of the tag
//!   ([`BookReport::malformed_attributes`]).
//! - Tag names end at any ASCII whitespace (not only a space); attribute keys are trimmed and
//!   a value may follow `=` after whitespace (`KEY = "x"`).

use std::collections::BTreeMap;
use std::mem;

use sf_ui::{Align, Block, ImageRef, Inline, Paragraph, Rgba8, RichDoc, RunStyle};
use sf_vfs::VfsPath;

/// Theme font family key for `<FONT FACE="Daedric">`.
pub const FAMILY_DAEDRIC: &str = "daedric";

/// Default size of an `img://` image missing a dimension (`OpenMW`).
pub const IMG_SCHEME_DEFAULT_SIZE: f32 = 50.0;

/// `OpenMW`'s forced page break marker (the newline is part of it).
const PAGE_BREAK: &str = "[pagebreak]\n";

/// `OpenMW`'s source prefix for a VFS path used as is.
const IMG_SCHEME: &str = "img://";

/// HTML legacy font sizes 1-7 as CSS absolute-size factors (`em`).
const LEGACY_SIZES: [f32; 7] = [0.75, 8.0 / 9.0, 1.0, 1.2, 1.5, 2.0, 3.0];

/// How [`parse_with`] reads a text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BookOptions {
    /// Drop text after the last `<BR>` or `<P>` (vanilla and `OpenMW` for BOOK records).
    pub trim_after_last_break: bool,
}

impl Default for BookOptions {
    fn default() -> Self {
        Self {
            trim_after_last_break: true,
        }
    }
}

/// What the parser met and repaired. Keys are lowercase.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BookReport {
    /// Recognised tags by name, closing ones with a leading `/` (`"br"`, `"/font"`).
    pub tags: BTreeMap<String, u32>,
    /// Dropped tags by name, closing ones with a leading `/`.
    pub unknown_tags: BTreeMap<String, u32>,
    /// Mapped attribute values, `tag.attr=value` (`"font.face=daedric"`, `"div.align=center"`,
    /// `"font.size=3"`), for surveys.
    pub values: BTreeMap<String, u32>,
    /// A `<` with no `>` after it was read as text.
    pub unterminated_tag: bool,
    /// Quoted attribute values with no closing quote.
    pub malformed_attributes: u32,
    /// `<IMG>` tags dropped: no `SRC`, a `bookart` image without both sizes, a bad path.
    pub skipped_images: u32,
    /// `<FONT>` / `</FONT>` after text on the same line (`OpenMW` breaks the line there).
    pub inline_font_changes: u32,
    /// Bytes of non-whitespace text dropped after the last `<BR>` / `<P>`.
    pub dropped_tail_bytes: usize,
    /// `&name;` / `&#n;` sequences left undecoded in the text.
    pub entities: u32,
}

/// Parses BOOK text with vanilla rules ([`BookOptions::default`]).
pub fn parse(text: &str) -> (RichDoc, BookReport) {
    parse_with(text, BookOptions::default())
}

/// Parses BOOK text into a document and a report. Never panics.
pub fn parse_with(text: &str, options: BookOptions) -> (RichDoc, BookReport) {
    let text = text.replace('\r', "");
    let plain_end = if options.trim_after_last_break {
        plain_text_end(&text)
    } else {
        text.len()
    };
    let mut p = Parser::new();
    let mut no_close = false;
    let mut i = 0;
    while let Some(rest) = text.get(i..) {
        if rest.is_empty() {
            break;
        }
        if rest.starts_with(PAGE_BREAK) {
            i += PAGE_BREAK.len();
            p.page_break();
            continue;
        }
        if !no_close && let Some(after) = rest.strip_prefix('<') {
            if let Some(len) = after.find('>') {
                i += len + 2;
                p.tag(&after[..len]);
                continue;
            }
            // No `>` from here on, so none after any later `<` either.
            no_close = true;
            p.report.unterminated_tag = true;
        }
        let Some(ch) = rest.chars().next() else {
            break;
        };
        p.plain(ch, i < plain_end);
        i += ch.len_utf8();
    }
    p.finish()
}

/// `OpenMW`'s `mPlainTextEnd`: the byte offset of the last `<br>` or `<p>`, 0 when neither.
fn plain_text_end(text: &str) -> usize {
    let lower = text.to_ascii_lowercase();
    match (lower.rfind("<br>"), lower.rfind("<p>")) {
        (None, None) => 0,
        (Some(a), Some(b)) => a.max(b),
        (Some(a), None) | (None, Some(a)) => a,
    }
}

/// The VFS path of a `bookart` image source (`OpenMW` `correctBookartPath` without the VFS
/// lookups): normalized (lowercase, `/`), cut to start at a `bookart/` or `textures/`
/// directory if it has one (searched in that order), else prefixed with `bookart/`. `None`
/// for an empty path or one with a `..` segment.
///
/// The resolver then tries [`image_candidates`] in order.
pub fn bookart_path(src: &str) -> Option<String> {
    let path = VfsPath::new(src).ok()?;
    let path = path.as_str();
    for dir in ["bookart", "textures"] {
        if let Some(at) = find_directory(path, dir) {
            return Some(path[at..].to_owned());
        }
    }
    Some(format!("bookart/{path}"))
}

/// `OpenMW` `findDirectory`: the offset of `dir` as a whole directory segment followed by more
/// path. Like the original, gives up at a match that ends the path.
fn find_directory(path: &str, dir: &str) -> Option<usize> {
    let mut offset = 0;
    while offset < path.len() {
        let at = offset + path.get(offset..)?.find(dir)?;
        let end = at + dir.len();
        if end >= path.len() {
            return None;
        }
        let starts_segment = at == 0 || path.as_bytes().get(at - 1) == Some(&b'/');
        if starts_segment && path.as_bytes().get(end) == Some(&b'/') {
            return Some(at);
        }
        offset = end;
    }
    None
}

/// The VFS paths to try, in order, for an image this parser produced (`OpenMW`
/// `correctResourcePath` and the sized `correctBookartPath`): the `.dds` version, the source,
/// both under `bookart/` by file name, then (when the reference has both sizes) the `.dds`
/// path with a `_W_H` suffix before the extension and that file under `bookart/`. Duplicates
/// are removed. `OpenMW` looks `img://` sources up as is; trying the fallbacks for them as
/// well only adds lookups.
pub fn image_candidates(image: &ImageRef) -> Vec<String> {
    let source = image.source.as_str();
    let dds = with_dds_extension(source);
    let primary = dds.clone().unwrap_or_else(|| source.to_owned());
    let mut out = Vec::new();
    push_unique(&mut out, primary.clone());
    if dds.is_some() {
        push_unique(&mut out, source.to_owned());
    }
    push_unique(&mut out, format!("bookart/{}", file_name(&primary)));
    if dds.is_some() {
        push_unique(&mut out, format!("bookart/{}", file_name(source)));
    }
    if let (Some(w), Some(h)) = (image.width, image.height) {
        let sized = match primary.rfind('.') {
            Some(dot) => format!("{}_{w}_{h}{}", &primary[..dot], &primary[dot..]),
            None => format!("{primary}_{w}_{h}"),
        };
        let fallback = format!("bookart/{}", file_name(&sized));
        push_unique(&mut out, sized);
        push_unique(&mut out, fallback);
    }
    out
}

fn push_unique(out: &mut Vec<String>, path: String) {
    if !out.contains(&path) {
        out.push(path);
    }
}

fn file_name(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

/// The path with its file extension replaced by `dds`, or `None` when it has no extension or
/// it already is `dds`.
fn with_dds_extension(path: &str) -> Option<String> {
    let file_start = path.rfind('/').map_or(0, |i| i + 1);
    let dot = file_start + path.get(file_start..)?.rfind('.')?;
    let ext = path.get(dot + 1..)?;
    (ext != "dds").then(|| format!("{}.dds", &path[..dot]))
}

/// HTML's legacy font size (`<font size>`) as an `em` factor; `None` when `value` has no
/// digits. See the module docs for the table.
pub fn legacy_font_size(value: &str) -> Option<f32> {
    let s = value.trim_start_matches(|c: char| c.is_ascii_whitespace());
    let (sign, digits) = match s.as_bytes().first() {
        Some(b'+') => (1, &s[1..]),
        Some(b'-') => (-1, &s[1..]),
        _ => (0, s),
    };
    let end = digits
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(digits.len());
    if end == 0 {
        return None;
    }
    let n = digits[..end].parse::<i64>().unwrap_or(i64::MAX);
    let size = match sign {
        1 => 3_i64.saturating_add(n),
        -1 => 3_i64.saturating_sub(n),
        _ => n,
    }
    .clamp(1, 7);
    let index = usize::try_from(size - 1).ok()?;
    LEGACY_SIZES.get(index).copied()
}

/// `<FONT COLOR>` as `OpenMW` reads it (see the module docs).
pub fn font_color(value: &str) -> Rgba8 {
    let s = value.strip_prefix('#').unwrap_or(value);
    let end = s.find(|c: char| !c.is_ascii_hexdigit()).unwrap_or(s.len());
    let v = u32::from_str_radix(&s[..end], 16).unwrap_or(0);
    let [_, r, g, b] = v.to_be_bytes();
    Rgba8::rgb(r, g, b)
}

/// One tag's name and attributes.
#[derive(Debug, Default)]
struct Tag {
    name: String,
    closing: bool,
    attrs: BTreeMap<String, String>,
}

impl Tag {
    fn attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(String::as_str)
    }
}

/// `OpenMW` `parseTag` (with the repairs in the module docs). `inner` is the text between
/// `<` and `>`.
fn parse_tag(inner: &str, report: &mut BookReport) -> Tag {
    let (name, mut rest) = match inner.find(|c: char| c.is_ascii_whitespace()) {
        Some(i) => (&inner[..i], &inner[i + 1..]),
        None => (inner, ""),
    };
    let mut tag = Tag {
        name: name.to_ascii_lowercase(),
        ..Tag::default()
    };
    if tag.name.starts_with('/') {
        tag.name.remove(0);
        tag.closing = true;
        return tag;
    }
    while !rest.is_empty() {
        let Some(eq) = rest.find('=') else {
            break;
        };
        let key = rest[..eq]
            .split_ascii_whitespace()
            .next_back()
            .unwrap_or_default()
            .to_ascii_lowercase();
        rest = rest[eq + 1..].trim_start_matches(|c: char| c.is_ascii_whitespace());
        let Some(first) = rest.chars().next() else {
            break;
        };
        let (value, next) = if first == '"' || first == '\'' {
            let body = &rest[1..];
            if let Some(end) = body.find(first) {
                // The closing quote, then one more character (`OpenMW` expects a space).
                let after = &body[end + 1..];
                let next = after
                    .chars()
                    .next()
                    .map_or("", |c| &after[c.len_utf8()..]);
                (&body[..end], next)
            } else {
                report.malformed_attributes += 1;
                (body, "")
            }
        } else if let Some(end) = rest.find(|c: char| c.is_ascii_whitespace()) {
            (&rest[..end], &rest[end + 1..])
        } else {
            (rest, "")
        };
        tag.attrs.insert(key, value.to_owned());
        rest = next;
    }
    tag
}

/// `MyGUI::utility::parseInt` for image sizes, kept only when positive.
fn image_size(value: Option<&str>) -> Option<u16> {
    let v = value?
        .trim_start()
        .trim_end_matches([' ', '\t'])
        .parse::<i32>()
        .ok()?;
    u16::try_from(v).ok().filter(|&v| v > 0)
}

fn bump(map: &mut BTreeMap<String, u32>, key: String) {
    *map.entry(key).or_default() += 1;
}

/// Tokenizer flags (`BookTextParser`) and formatter state (`BookFormatter`) in one pass.
#[derive(Default)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "a port of OpenMW's parser flags, each one named as there"
)]
struct Parser {
    doc: RichDoc,
    report: BookReport,
    style: RunStyle,
    align: Align,
    /// The current piece: runs and inline images since the last block boundary.
    chunk: Vec<Inline>,
    /// `mIgnoreNewlineTags`: `<BR>`/`<P>` add no newline (before any text).
    ignore_newline_tags: bool,
    /// `mIgnoreLineEndings`: drop `'\n'` in the text (after `<BR>`/`<P>`, at the start).
    ignore_line_endings: bool,
    /// `brBeforeLastTag`: the previous piece ended in a newline (or was empty).
    br_before_last_tag: bool,
    /// `isPrevImg`: the last block boundary was an image.
    is_prev_img: bool,
    /// The current line has text (for [`BookReport::inline_font_changes`]).
    line_has_text: bool,
}

impl Parser {
    fn new() -> Self {
        Self {
            ignore_newline_tags: true,
            ignore_line_endings: true,
            ..Self::default()
        }
    }

    fn plain(&mut self, ch: char, visible: bool) {
        if self.ignore_line_endings && ch == '\n' {
            return;
        }
        if visible {
            let mut buf = [0; 4];
            self.push_text(ch.encode_utf8(&mut buf));
        } else if !ch.is_whitespace() {
            self.report.dropped_tail_bytes += ch.len_utf8();
        }
        self.ignore_line_endings = false;
        self.ignore_newline_tags = false;
    }

    fn page_break(&mut self) {
        self.boundary(false);
        self.doc.push(Block::PageBreak);
        self.ignore_newline_tags = false;
    }

    fn tag(&mut self, inner: &str) {
        let tag = parse_tag(inner, &mut self.report);
        let key = if tag.closing {
            format!("/{}", tag.name)
        } else {
            tag.name.clone()
        };
        match tag.name.as_str() {
            "br" | "p" => {
                bump(&mut self.report.tags, key);
                if !self.ignore_newline_tags {
                    self.push_text(if tag.name == "br" { "\n" } else { "\n\n" });
                }
                self.ignore_line_endings = true;
                // `<BR ...>` has no block meaning, attributes or not (as `OpenMW`).
                if tag.name == "p" && !tag.attrs.is_empty() {
                    self.boundary(false);
                    self.div(&tag);
                }
            }
            "div" => {
                bump(&mut self.report.tags, key);
                self.boundary(false);
                self.div(&tag);
            }
            "font" => {
                bump(&mut self.report.tags, key);
                if self.line_has_text {
                    self.report.inline_font_changes += 1;
                }
                if tag.closing {
                    self.style = RunStyle::default();
                } else {
                    self.font(&tag);
                }
            }
            "img" => {
                bump(&mut self.report.tags, key);
                self.ignore_newline_tags = false;
                self.image(&tag);
            }
            "body" => {
                bump(&mut self.report.tags, key);
                if let Some(src) = tag.attr("background")
                    && !tag.closing
                {
                    self.doc.background = image_source(src).map(ImageRef::new);
                }
            }
            _ => bump(&mut self.report.unknown_tags, key),
        }
    }

    fn note(&mut self, tag: &str, attr: &str, value: &str) {
        let value = value.trim().to_ascii_lowercase();
        bump(&mut self.report.values, format!("{tag}.{attr}={value}"));
    }

    /// `handleDiv`.
    fn div(&mut self, tag: &Tag) {
        let Some(align) = tag.attr("align") else {
            return;
        };
        self.note(&tag.name, "align", align);
        let align = align.trim();
        if align.eq_ignore_ascii_case("center") {
            self.align = Align::Center;
        } else if align.eq_ignore_ascii_case("left") {
            self.align = Align::Start;
        } else if align.eq_ignore_ascii_case("right") {
            self.align = Align::End;
        }
    }

    /// `handleFont`, plus `SIZE`.
    fn font(&mut self, tag: &Tag) {
        if let Some(color) = tag.attr("color") {
            self.style.color = Some(font_color(color));
        }
        if let Some(face) = tag.attr("face") {
            self.note("font", "face", face);
            self.style.family = face
                .trim()
                .eq_ignore_ascii_case("daedric")
                .then(|| FAMILY_DAEDRIC.to_owned());
        }
        if let Some(size) = tag.attr("size") {
            self.note("font", "size", size);
            if let Some(em) = legacy_font_size(size) {
                self.style.size = em;
            }
        }
    }

    /// The `Event_ImgTag` case of `markupToWidget`.
    fn image(&mut self, tag: &Tag) {
        let width = image_size(tag.attr("width"));
        let height = image_size(tag.attr("height"));
        let src = if tag.closing { None } else { tag.attr("src") };
        let Some(src) = src else {
            self.boundary(true);
            self.report.skipped_images += 1;
            return;
        };
        let inline = src.starts_with(IMG_SCHEME) && width.is_none();
        if !inline {
            self.boundary(true);
        }
        let size = if src.starts_with(IMG_SCHEME) {
            let h = height.map_or(IMG_SCHEME_DEFAULT_SIZE, f32::from);
            Some((width.map_or(IMG_SCHEME_DEFAULT_SIZE, f32::from), h))
        } else {
            width.zip(height).map(|(w, h)| (f32::from(w), f32::from(h)))
        };
        let (Some((w, h)), Some(source)) = (size, image_source(src)) else {
            self.report.skipped_images += 1;
            return;
        };
        let image = ImageRef::new(source).with_size(w, h);
        if inline {
            self.chunk.push(Inline::Image(image));
            self.line_has_text = true;
        } else {
            self.doc.push(Block::Image {
                image,
                align: self.align,
            });
        }
    }

    /// Appends text to the current piece in the current style.
    fn push_text(&mut self, s: &str) {
        let Some(last) = s.chars().last() else {
            return;
        };
        self.line_has_text = last != '\n';
        if let Some(Inline::Text { text, style }) = self.chunk.last_mut()
            && *style == self.style
        {
            text.push_str(s);
            return;
        }
        self.chunk.push(Inline::Text {
            text: s.to_owned(),
            style: self.style.clone(),
        });
    }

    /// Ends the current piece at a block tag (`markupToWidget`'s text handling between two
    /// events). `image`: the boundary is a block image.
    fn boundary(&mut self, image: bool) {
        let mut chunk = mem::take(&mut self.chunk);
        if chunk.is_empty() {
            self.br_before_last_tag = true;
        } else {
            let at_start = first_char(&chunk) == Some('\n');
            let at_end = last_char(&chunk) == Some('\n');
            if at_start && !self.br_before_last_tag && !self.is_prev_img {
                pop_front_newline(&mut chunk);
            }
            if at_end && !chunk.is_empty() {
                pop_back_char(&mut chunk);
            }
            if !chunk.is_empty() || self.br_before_last_tag || self.is_prev_img {
                self.emit(chunk);
            }
            self.br_before_last_tag = at_end;
        }
        self.is_prev_img = image;
        self.line_has_text = false;
    }

    /// One piece (`OpenMW`'s text box) as paragraphs: non-empty lines grouped, blank lines as
    /// empty paragraphs.
    fn emit(&mut self, chunk: Vec<Inline>) {
        let mut lines: Vec<Vec<Inline>> = vec![Vec::new()];
        for inline in chunk {
            match inline {
                Inline::Text { text, style } => {
                    for (i, part) in text.split('\n').enumerate() {
                        if i > 0 {
                            lines.push(Vec::new());
                        }
                        if !part.is_empty()
                            && let Some(line) = lines.last_mut()
                        {
                            line.push(Inline::Text {
                                text: part.to_owned(),
                                style: style.clone(),
                            });
                        }
                    }
                }
                image @ Inline::Image(_) => {
                    if let Some(line) = lines.last_mut() {
                        line.push(image);
                    }
                }
            }
        }
        let align = self.align;
        let mut para: Option<Paragraph> = None;
        for line in lines {
            if line.is_empty() {
                if let Some(p) = para.take() {
                    self.doc.push(Block::Paragraph(p));
                }
                self.doc.push(Block::Paragraph(Paragraph::new(align)));
                continue;
            }
            let p = para.get_or_insert_with(|| Paragraph::new(align));
            if !p.inlines.is_empty() {
                let style = last_style(p);
                p.push_text("\n", &style);
            }
            for inline in line {
                match inline {
                    Inline::Text { text, style } => p.push_text(&text, &style),
                    Inline::Image(image) => p.push_image(image),
                }
            }
        }
        if let Some(p) = para {
            self.doc.push(Block::Paragraph(p));
        }
    }

    fn finish(mut self) -> (RichDoc, BookReport) {
        self.boundary(false);
        self.report.entities = count_entities(&self.doc.plain_text());
        (self.doc, self.report)
    }
}

/// The VFS path for an `<IMG SRC>` or `<BODY BACKGROUND>` value.
fn image_source(src: &str) -> Option<String> {
    match src.strip_prefix(IMG_SCHEME) {
        Some(path) => VfsPath::new(path).ok().map(|p| p.as_str().to_owned()),
        None => bookart_path(src),
    }
}

fn first_char(chunk: &[Inline]) -> Option<char> {
    match chunk.first() {
        Some(Inline::Text { text, .. }) => text.chars().next(),
        _ => None,
    }
}

fn last_char(chunk: &[Inline]) -> Option<char> {
    match chunk.last() {
        Some(Inline::Text { text, .. }) => text.chars().last(),
        _ => None,
    }
}

/// Removes the leading `'\n'` (the caller checked it is one).
fn pop_front_newline(chunk: &mut Vec<Inline>) {
    if let Some(Inline::Text { text, .. }) = chunk.first_mut() {
        text.remove(0);
        if text.is_empty() {
            chunk.remove(0);
        }
    }
}

fn pop_back_char(chunk: &mut Vec<Inline>) {
    if let Some(Inline::Text { text, .. }) = chunk.last_mut() {
        text.pop();
        if text.is_empty() {
            chunk.pop();
        }
    }
}

/// The style of the paragraph's last run, for the `'\n'` that ends its line.
fn last_style(p: &Paragraph) -> RunStyle {
    p.inlines
        .iter()
        .rev()
        .find_map(|inline| match inline {
            Inline::Text { style, .. } => Some(style.clone()),
            Inline::Image(_) => None,
        })
        .unwrap_or_default()
}

/// `&name;` and `&#n;` sequences.
fn count_entities(text: &str) -> u32 {
    let mut n = 0;
    for (i, _) in text.match_indices('&') {
        let tail = &text[i + 1..];
        let len = tail
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '#'))
            .unwrap_or(tail.len());
        if len > 0 && tail[len..].starts_with(';') {
            n += 1;
        }
    }
    n
}
