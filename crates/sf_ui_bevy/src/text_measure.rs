//! `sf_ui::TextMeasure` on Bevy's own text layout (ui-text.md "Bevy side").
//!
//! The measurer feeds a paragraph to [`TextPipeline::update_buffer`], the
//! call `bevy_ui`'s text systems make for every `Text` node, with the same
//! fonts, sizes, line heights, line-break mode and inline boxes the reader
//! spawns ([`RichTextTheme`] builds both), at the page node's physical
//! width and scale factor. Its lines are therefore the lines that are drawn.
//! Results are cached per paragraph content, width and scale
//! ([`MeasureCache`]); a theme or font change clears the cache.
//!
//! Units: the paginator works in **physical pixels** (what parley lays out
//! at the UI scale factor); nodes are placed in logical pixels by dividing
//! by the scale factor.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use bevy_asset::{AssetEvent, Assets, Handle};
use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_image::Image;
use bevy_math::Vec2;
use bevy_text::{
    ComputedTextBlock, Font, FontCx, FontSource, FontStyle, FontWeight, InlineBox, InlineBoxKind,
    Justify, LayoutCx, LetterSpacing, LineBreak, LineHeight, RemSize, TextBounds, TextElement,
    TextFont, TextPipeline,
};
use parley::PositionedLayoutItem;
use sf_ui::{
    Align, ImageRef, Inline, LineMetrics, OBJECT_REPLACEMENT, Paragraph, RunStyle, Size,
    TextMeasure,
};

/// Line breaking for documents: at word boundaries, inside a word only when
/// it is longer than the line (CSS `overflow-wrap: anywhere`), so long words
/// never overflow the page.
pub const DOC_LINE_BREAK: LineBreak = LineBreak::WordOrCharacter;

/// How documents look where it changes metrics: the theme's resolved fonts
/// and sizes. Both the measurer and the reader's text nodes read this, so a
/// theme sets fonts here (from its `ui.theme` font tokens), not in CSS on
/// the text nodes, which would desync what is drawn from what was paginated.
/// CSS styles everything else (page frames, backgrounds, buttons, spacing).
#[derive(Resource, Clone, Debug)]
pub struct RichTextTheme {
    /// Body face (`RunStyle::family == None`).
    pub body: FontSource,
    /// Theme font keys (`RunStyle::family`) to faces; unknown keys fall back
    /// to the body face.
    pub families: HashMap<String, FontSource>,
    /// Body size in logical pixels at text scale 1.
    pub font_size: f32,
    /// Line height as a multiple of the font size.
    pub line_height: f32,
    /// Text colour when a run has none.
    pub color: Color,
    /// Space between blocks, in `em` of the body size.
    pub block_gap: f32,
    /// Logical pixels per document unit for images (at text scale 1).
    pub image_scale: f32,
}

impl Default for RichTextTheme {
    fn default() -> Self {
        Self {
            body: FontSource::default(),
            families: HashMap::new(),
            font_size: 18.0,
            line_height: 1.25,
            color: Color::srgb(0.12, 0.09, 0.06),
            block_gap: 0.5,
            image_scale: 1.0,
        }
    }
}

impl RichTextTheme {
    /// The font of a run at `text_scale`.
    pub fn text_font(&self, style: &RunStyle, text_scale: f32) -> TextFont {
        let font = style
            .family
            .as_ref()
            .and_then(|key| self.families.get(key))
            .unwrap_or(&self.body)
            .clone();
        TextFont {
            font,
            font_size: bevy_text::FontSize::Px(self.font_size * style.size * text_scale),
            weight: if style.bold {
                FontWeight::BOLD
            } else {
                FontWeight::NORMAL
            },
            style: if style.italic {
                FontStyle::Italic
            } else {
                FontStyle::Normal
            },
            ..TextFont::default()
        }
    }

    pub fn line_height(&self) -> LineHeight {
        LineHeight::RelativeToFont(self.line_height)
    }

    /// A run's colour.
    pub fn color(&self, style: &RunStyle) -> Color {
        style
            .color
            .map_or(self.color, |c| Color::srgba_u8(c.r, c.g, c.b, c.a))
    }

    /// Block gap in logical pixels at `text_scale`.
    pub fn gap(&self, text_scale: f32) -> f32 {
        self.block_gap * self.font_size * text_scale
    }
}

/// Bevy's justification for a paragraph alignment.
pub fn justify(align: Align) -> Justify {
    match align {
        Align::Start => Justify::Start,
        Align::Center => Justify::Center,
        Align::End => Justify::End,
        Align::Justify => Justify::Justified,
    }
}

/// Image handles by document source path. The reader resolves sources it
/// hasn't seen with [`RichImages::resolver`] (by default the asset server's
/// `load`); a content pack replaces the resolver to map its paths into the
/// VFS, or inserts handles directly.
#[derive(Resource, Default)]
pub struct RichImages {
    handles: HashMap<String, Handle<Image>>,
    pub resolver: Option<ImageResolver>,
}

/// Turns a document image source into a handle.
pub type ImageResolver = Box<dyn Fn(&str, &bevy_asset::AssetServer) -> Handle<Image> + Send + Sync>;

impl RichImages {
    pub fn insert(&mut self, source: impl Into<String>, handle: Handle<Image>) {
        self.handles.insert(source.into(), handle);
    }

    pub fn get(&self, source: &str) -> Option<&Handle<Image>> {
        self.handles.get(source)
    }

    /// The handle of `source`, resolving it on first use.
    pub fn resolve(
        &mut self,
        source: &str,
        assets: Option<&bevy_asset::AssetServer>,
    ) -> Option<Handle<Image>> {
        if let Some(h) = self.handles.get(source) {
            return Some(h.clone());
        }
        let assets = assets?;
        let handle = match &self.resolver {
            Some(resolve) => resolve(source, assets),
            None => assets.load(source.to_owned()),
        };
        self.handles.insert(source.to_owned(), handle.clone());
        Some(handle)
    }
}

/// Scale of one measurement: physical pixels per logical pixel (window
/// scale factor times `UiScale`) and the player's text scale.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeasureScale {
    pub scale_factor: f32,
    pub text_scale: f32,
}

impl Default for MeasureScale {
    fn default() -> Self {
        Self {
            scale_factor: 1.0,
            text_scale: 1.0,
        }
    }
}

/// Logical size of an image as drawn: the reference's document size (a
/// missing side from the aspect ratio), else the image's own pixel size,
/// times the theme's image scale and the text scale. Zero when unknown.
pub fn image_logical_size(
    image: &ImageRef,
    native: Option<Vec2>,
    theme: &RichTextTheme,
    text_scale: f32,
) -> Vec2 {
    let native = native.filter(|n| n.x > 0.0 && n.y > 0.0);
    let doc = match (image.width, image.height, native) {
        (Some(w), Some(h), _) => Vec2::new(w, h),
        (Some(w), None, Some(n)) => Vec2::new(w, w * n.y / n.x),
        (None, Some(h), Some(n)) => Vec2::new(h * n.x / n.y, h),
        (None, None, Some(n)) => n,
        _ => Vec2::ZERO,
    };
    doc * theme.image_scale * text_scale
}

/// Measured lines per paragraph, width and scale. Exact: an entry is used
/// only when its paragraph equals the one asked for (the hash only finds it).
#[derive(Resource, Default, Debug)]
pub struct MeasureCache {
    entries: HashMap<u64, CacheEntry>,
    pub hits: u64,
    pub misses: u64,
}

#[derive(Debug)]
struct CacheEntry {
    paragraph: Paragraph,
    width: u32,
    scale: (u32, u32),
    lines: Vec<LineMetrics>,
}

/// Entries kept before the cache starts over (a few books' worth).
const CACHE_LIMIT: usize = 8192;

impl MeasureCache {
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

fn paragraph_key(p: &Paragraph, width: u32, scale: (u32, u32)) -> u64 {
    let mut h = std::hash::DefaultHasher::new();
    (width, scale, p.align).hash(&mut h);
    for inline in &p.inlines {
        match inline {
            Inline::Text { text, style } => {
                (0u8, text, &style.family, style.size.to_bits()).hash(&mut h);
                (style.bold, style.italic).hash(&mut h);
            }
            Inline::Image(image) => {
                (1u8, &image.source).hash(&mut h);
                (
                    image.width.map(f32::to_bits),
                    image.height.map(f32::to_bits),
                )
                    .hash(&mut h);
            }
        }
    }
    h.finish()
}

/// Clears the measure cache when the theme or a font changes.
pub(crate) fn invalidate_measure_cache(
    theme: Res<RichTextTheme>,
    mut fonts: MessageReader<AssetEvent<Font>>,
    mut cache: ResMut<MeasureCache>,
) {
    let fonts_changed = fonts.read().count() > 0;
    if theme.is_changed() || fonts_changed {
        cache.clear();
    }
}

/// Everything a measurement needs from the world.
#[derive(SystemParam)]
pub struct RichTextMeasure<'w> {
    pipeline: ResMut<'w, TextPipeline>,
    fonts: Res<'w, Assets<Font>>,
    font_cx: ResMut<'w, FontCx>,
    layout_cx: ResMut<'w, LayoutCx>,
    theme: Res<'w, RichTextTheme>,
    cache: ResMut<'w, MeasureCache>,
    images: ResMut<'w, RichImages>,
    image_assets: Res<'w, Assets<Image>>,
}

impl RichTextMeasure<'_> {
    /// A measurer at this scale.
    pub fn at(&mut self, scale: MeasureScale) -> ParleyMeasure<'_> {
        ParleyMeasure {
            pipeline: &mut self.pipeline,
            fonts: &self.fonts,
            font_cx: &mut self.font_cx,
            layout_cx: &mut self.layout_cx,
            theme: &self.theme,
            cache: &mut self.cache,
            images: &self.images,
            image_assets: &self.image_assets,
            scale,
            block: ComputedTextBlock::default(),
            incomplete: false,
        }
    }

    pub fn theme(&self) -> &RichTextTheme {
        &self.theme
    }

    pub fn images(&self) -> &RichImages {
        &self.images
    }

    pub fn images_mut(&mut self) -> &mut RichImages {
        &mut self.images
    }

    pub fn image_assets(&self) -> &Assets<Image> {
        &self.image_assets
    }
}

/// [`TextMeasure`] over Bevy's text pipeline (parley), in physical pixels.
pub struct ParleyMeasure<'a> {
    pipeline: &'a mut TextPipeline,
    fonts: &'a Assets<Font>,
    font_cx: &'a mut FontCx,
    layout_cx: &'a mut LayoutCx,
    theme: &'a RichTextTheme,
    cache: &'a mut MeasureCache,
    images: &'a RichImages,
    image_assets: &'a Assets<Image>,
    scale: MeasureScale,
    block: ComputedTextBlock,
    incomplete: bool,
}

impl ParleyMeasure<'_> {
    /// Whether a layout failed (a font not loaded yet): the result is a
    /// placeholder, paginate again later.
    pub fn incomplete(&self) -> bool {
        self.incomplete
    }

    /// Logical size of an inline or block image.
    pub fn image_logical(&self, image: &ImageRef) -> Vec2 {
        let native = self
            .images
            .get(&image.source)
            .and_then(|h| self.image_assets.get(h))
            .map(|i| i.size().as_vec2());
        image_logical_size(image, native, self.theme, self.scale.text_scale)
    }

    /// Lays `paragraph` out at `width` physical pixels and returns the
    /// parley lines, uncached. `None` when a font isn't loaded yet.
    fn layout(&mut self, paragraph: &Paragraph, width: f32) -> Option<Vec<LineMetrics>> {
        let text_scale = self.scale.text_scale;
        let fonts: Vec<TextFont> = paragraph
            .inlines
            .iter()
            .map(|i| match i {
                Inline::Text { style, .. } => self.theme.text_font(style, text_scale),
                Inline::Image(_) => TextFont::default(),
            })
            .collect();
        let boxes: Vec<InlineBox> = paragraph
            .inlines
            .iter()
            .map(|i| match i {
                Inline::Image(image) => InlineBox {
                    kind: InlineBoxKind::InFlow,
                    size: self.image_logical(image),
                },
                Inline::Text { .. } => InlineBox::default(),
            })
            .collect();
        let line_height = self.theme.line_height();
        let elements = paragraph.inlines.iter().enumerate().map(|(i, inline)| {
            let element = match inline {
                Inline::Text { text, .. } => TextElement::Text {
                    text,
                    font: &fonts[i],
                    color: Color::WHITE,
                    line_height,
                    letter_spacing: LetterSpacing::default(),
                },
                Inline::Image(_) => TextElement::Box(&boxes[i]),
            };
            // Inline box ids are entity bits: index i + 1 names inline i.
            (Entity::from_bits(i as u64 + 1), 1, element)
        });
        self.pipeline
            .update_buffer(
                self.fonts,
                elements,
                DOC_LINE_BREAK,
                justify(paragraph.align),
                TextBounds::new_horizontal(width),
                self.scale.scale_factor,
                &mut self.block,
                self.font_cx,
                self.layout_cx,
                Vec2::ZERO,
                RemSize::default(),
            )
            .ok()?;
        Some(flattened_lines(paragraph, &self.block))
    }
}

/// parley's lines (byte ranges over the text without image placeholders)
/// as ranges over the paragraph's flattened text, where each image is one
/// U+FFFC. A line holding an inline box takes its placeholder.
fn flattened_lines(paragraph: &Paragraph, block: &ComputedTextBlock) -> Vec<LineMetrics> {
    let placeholder = OBJECT_REPLACEMENT.len_utf8();
    let layout = block.buffer();
    let mut lines = Vec::new();
    let mut boxes_before = 0;
    let mut text_at = 0;
    for line in layout.lines() {
        let boxes = line
            .items()
            .filter(|item| matches!(item, PositionedLayoutItem::InlineBox(_)))
            .count();
        // A line of only inline boxes has no text: parley reports an
        // inverted sentinel range; it sits where the previous line ended.
        let mut text = line.text_range();
        if text.start > text.end {
            text = text_at..text_at;
        }
        text_at = text.end;
        let start = text.start + boxes_before * placeholder;
        boxes_before += boxes;
        let end = text.end + boxes_before * placeholder;
        // The layout's height is the sum of its line heights (glyphs and
        // boxes may overhang a line box, as in CSS).
        lines.push(LineMetrics {
            range: start..end,
            height: line.metrics().line_height,
        });
    }
    // parley ends a paragraph with an empty line after a final hard break
    // or a trailing space that overflows the line. It draws nothing: not a
    // line of the paragraph.
    if lines.len() > 1 && lines.last().is_some_and(|l| l.range.is_empty()) {
        lines.pop();
    }
    if let Some(last) = lines.last_mut() {
        last.range.end = last.range.end.max(paragraph.len());
    }
    lines
}

impl TextMeasure for ParleyMeasure<'_> {
    fn lines(&mut self, paragraph: &Paragraph, width: f32) -> Vec<LineMetrics> {
        let scale = (
            self.scale.scale_factor.to_bits(),
            self.scale.text_scale.to_bits(),
        );
        let width_bits = width.to_bits();
        // An empty paragraph is a blank line of the body face.
        let blank;
        let measured = if paragraph.is_empty() {
            let mut p = Paragraph::new(paragraph.align);
            p.push_text(" ", &RunStyle::default());
            blank = p;
            &blank
        } else {
            paragraph
        };
        let key = paragraph_key(measured, width_bits, scale);
        if let Some(e) = self.cache.entries.get(&key)
            && e.width == width_bits
            && e.scale == scale
            && e.paragraph == *measured
        {
            self.cache.hits += 1;
            return e.lines.clone();
        }
        self.cache.misses += 1;
        let Some(mut lines) = self.layout(measured, width) else {
            // Fonts not loaded: a placeholder, never cached (the reader
            // waits for fonts before paginating).
            self.incomplete = true;
            return vec![LineMetrics {
                range: 0..paragraph.len(),
                height: self.theme.font_size * self.theme.line_height * self.scale.text_scale,
            }];
        };
        if paragraph.is_empty() {
            lines.truncate(1);
            if let Some(l) = lines.first_mut() {
                l.range = 0..0;
            }
        }
        if self.cache.entries.len() >= CACHE_LIMIT {
            self.cache.entries.clear();
        }
        self.cache.entries.insert(
            key,
            CacheEntry {
                paragraph: measured.clone(),
                width: width_bits,
                scale,
                lines: lines.clone(),
            },
        );
        lines
    }

    fn image_size(&mut self, image: &ImageRef) -> Size {
        let logical = self.image_logical(image) * self.scale.scale_factor;
        Size {
            width: logical.x,
            height: logical.y,
        }
    }
}
