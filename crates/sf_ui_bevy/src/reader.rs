//! Book and scroll readers for paginated rich text (ui-text.md "Bevy side").
//!
//! Insert a [`RichReader`] on a UI node (usually under a screen root); the
//! reader spawns its chrome as children and fills it:
//!
//! - **Book**: two facing pages (one in the controller preset or with
//!   `view: Some(PageView::Single)`), prev/next/take/close buttons. Page
//!   turns come from the buttons, the screen's `TabPrev`/`TabNext` actions
//!   (shoulder buttons) and anyone writing [`ReaderEvent`]s.
//! - **Scroll**: one long page in a scrolling viewport; prev/next scroll by
//!   most of a viewport, the mouse wheel by lines.
//!
//! Pages are measured **from the laid-out page body node**, as browsers
//! fragment into the real fragmentainer: a CSS theme sizes pages (padding,
//! width, height) and the reader paginates at that node's physical content
//! size and scale factor with [`RichTextMeasure`], so text never clips.
//! It re-paginates when that size, the scale, the text scale, the theme or
//! the page view changes, and reopens the page holding the bookmark.
//!
//! Every reader part carries a `Name` (`#sf-reader-page`, ...) and a
//! [`UiPart`], so theme sheets can style it.

use std::sync::Arc;

use bevy_asset::{AssetServer, Assets};
use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_input::mouse::{MouseScrollUnit, MouseWheel};
use bevy_text::{TextColor, TextLayout, TextSpan, Underline};
use bevy_ui::prelude::*;
use bevy_ui::widget::InlineImage;
use bevy_ui::{InteractionDisabled, ScrollPosition};
use bevy_ui_widgets::{Activate, Button};
use sf_ui::{
    Block, DocPos, Fragment, ImageRef, Inline, Page, PageFormat, PageView, Pagination, Paragraph,
    RichDoc, RunStyle, TextMeasure,
};

use crate::nav::UiFocusable;
use crate::router::{UiActionEvent, UiScreenRoot};
use crate::text_measure::{
    DOC_LINE_BREAK, MeasureScale, RichImages, RichTextMeasure, RichTextTheme, image_logical_size,
    justify,
};
use crate::{ScreenStackRes, UiAction, UiSettings};

/// A styling hook on every part the text widgets spawn (also its `Name`).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UiPart(pub &'static str);

/// How a reader lays its document out.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ReaderLayout {
    #[default]
    Book,
    Scroll,
}

/// A document on screen. Change `doc` (or the other fields) to update it.
#[derive(Component, Clone, Debug)]
#[require(Node, ReaderState)]
pub struct RichReader {
    pub doc: Arc<RichDoc>,
    pub layout: ReaderLayout,
    /// Pages per view in book layout; `None` picks from the preset (one page
    /// in `controller`, a spread otherwise).
    pub view: Option<PageView>,
    /// Show a Take button (the book or scroll can be picked up).
    pub takeable: bool,
    /// Reading position: the first page start of the open view. Kept across
    /// re-pagination; set it to open the document elsewhere.
    pub bookmark: DocPos,
}

impl RichReader {
    pub fn book(doc: impl Into<Arc<RichDoc>>) -> Self {
        Self {
            doc: doc.into(),
            layout: ReaderLayout::Book,
            view: None,
            takeable: false,
            bookmark: DocPos::default(),
        }
    }

    pub fn scroll(doc: impl Into<Arc<RichDoc>>) -> Self {
        Self {
            layout: ReaderLayout::Scroll,
            ..Self::book(doc)
        }
    }

    #[must_use]
    pub fn takeable(mut self, takeable: bool) -> Self {
        self.takeable = takeable;
        self
    }

    #[must_use]
    pub fn with_view(mut self, view: PageView) -> Self {
        self.view = Some(view);
        self
    }
}

/// Something to do with a reader. Prev, Next and Scroll are applied by the
/// reader; Take and Close are for the game (take the item, close the screen).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReaderAction {
    Prev,
    Next,
    Take,
    Close,
    /// Scroll layout: scroll by this many logical pixels (down is positive).
    Scroll(f32),
}

/// A reader action, from its buttons, the screen's UI actions, the mouse
/// wheel or any system.
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct ReaderEvent {
    pub reader: Entity,
    pub action: ReaderAction,
}

/// Button labels (already localized by the app).
#[derive(Resource, Clone, Debug)]
pub struct ReaderLabels {
    pub prev: String,
    pub next: String,
    pub take: String,
    pub close: String,
}

impl Default for ReaderLabels {
    fn default() -> Self {
        Self {
            prev: "<".into(),
            next: ">".into(),
            take: "Take".into(),
            close: "Close".into(),
        }
    }
}

/// Logical pixels one wheel line scrolls.
pub const WHEEL_LINE: f32 = 40.0;

/// What a reader shows now. Read-only for other systems.
#[derive(Component, Debug, Default)]
pub struct ReaderState {
    pagination: Option<Pagination>,
    /// Open view (book) index.
    view: usize,
    page_view: PageView,
    /// What the pagination was made for.
    made_for: Option<MadeFor>,
    /// Content of the view on screen (pagination generation, view).
    shown: Option<(u64, usize)>,
    generation: u64,
    /// Bookmark written by the reader itself (so an outside change is seen).
    own_bookmark: Option<DocPos>,
    chrome: Option<Chrome>,
}

impl ReaderState {
    pub fn pagination(&self) -> Option<&Pagination> {
        self.pagination.as_ref()
    }

    /// Open view index (a spread or a single page; always 0 for scrolls).
    pub fn view(&self) -> usize {
        self.view
    }

    pub fn view_count(&self) -> usize {
        self.pagination
            .as_ref()
            .map_or(0, |p| p.view_count(self.page_view))
    }

    pub fn page_view(&self) -> PageView {
        self.page_view
    }

    /// The scale the pagination was measured at (physical px per logical).
    pub fn scale(&self) -> Option<MeasureScale> {
        self.made_for.as_ref().map(|m| m.scale)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct MadeFor {
    doc: usize,
    size: (u32, u32),
    scale: MeasureScale,
    page_view: PageView,
    theme_tick: u32,
}

#[derive(Clone, Debug)]
struct Chrome {
    /// Page bodies (where fragments go): two for a book, one for a scroll.
    bodies: Vec<Entity>,
    /// The right page (hidden in single view).
    right_page: Option<Entity>,
    viewport: Option<Entity>,
    prev: Entity,
    next: Entity,
}

/// A page body: fragments of page `index` of the open view go here.
#[derive(Component, Clone, Copy, Debug)]
pub struct ReaderBody {
    pub reader: Entity,
    pub index: usize,
}

/// A reader button.
#[derive(Component, Clone, Copy, Debug)]
pub struct ReaderButton {
    pub reader: Entity,
    pub action: ReaderAction,
}

/// Content spawned for the open view (replaced on every page turn).
#[derive(Component, Clone, Copy, Debug)]
pub struct ReaderFragment {
    /// The `Placed` item this draws: block index.
    pub block: usize,
}

fn part(name: &'static str) -> (Name, UiPart) {
    (Name::new(name), UiPart(name))
}

/// Spawns the chrome of new readers.
pub(crate) fn spawn_reader_chrome(
    mut commands: Commands,
    labels: Res<ReaderLabels>,
    mut readers: Query<(Entity, &RichReader, &mut ReaderState), Added<ReaderState>>,
) {
    for (entity, reader, mut state) in &mut readers {
        let mut bodies = Vec::new();
        let mut right_page = None;
        let mut viewport = None;
        commands.entity(entity).insert(part("sf-reader"));
        let pages = commands
            .spawn((
                part("sf-reader-pages"),
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(32.0),
                    ..Node::default()
                },
                ChildOf(entity),
            ))
            .id();
        match reader.layout {
            ReaderLayout::Book => {
                for index in 0..2 {
                    let page = commands
                        .spawn((
                            part("sf-reader-page"),
                            Node {
                                width: Val::Px(360.0),
                                height: Val::Px(480.0),
                                padding: UiRect::all(Val::Px(24.0)),
                                ..Node::default()
                            },
                            ChildOf(pages),
                        ))
                        .id();
                    bodies.push(spawn_body(&mut commands, page, entity, index));
                    if index == 1 {
                        right_page = Some(page);
                    }
                }
            }
            ReaderLayout::Scroll => {
                let view = commands
                    .spawn((
                        part("sf-reader-scroll"),
                        Node {
                            width: Val::Px(480.0),
                            height: Val::Px(560.0),
                            padding: UiRect::all(Val::Px(24.0)),
                            overflow: Overflow::scroll_y(),
                            ..Node::default()
                        },
                        ScrollPosition::default(),
                        ChildOf(pages),
                    ))
                    .id();
                bodies.push(spawn_body(&mut commands, view, entity, 0));
                viewport = Some(view);
            }
        }
        let controls = commands
            .spawn((
                part("sf-reader-controls"),
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    ..Node::default()
                },
                ChildOf(entity),
            ))
            .id();
        let mut button = |name: &'static str, label: &str, action: ReaderAction| {
            commands
                .spawn((
                    part(name),
                    Node::default(),
                    Button,
                    UiFocusable,
                    ReaderButton {
                        reader: entity,
                        action,
                    },
                    ChildOf(controls),
                    children![Text::new(label)],
                ))
                .id()
        };
        let prev = button("sf-reader-prev", &labels.prev, ReaderAction::Prev);
        let next = button("sf-reader-next", &labels.next, ReaderAction::Next);
        if reader.takeable {
            button("sf-reader-take", &labels.take, ReaderAction::Take);
        }
        button("sf-reader-close", &labels.close, ReaderAction::Close);
        state.chrome = Some(Chrome {
            bodies,
            right_page,
            viewport,
            prev,
            next,
        });
    }
}

fn spawn_body(commands: &mut Commands, parent: Entity, reader: Entity, index: usize) -> Entity {
    commands
        .spawn((
            part("sf-reader-page-body"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Node::default()
            },
            ReaderBody { reader, index },
            ChildOf(parent),
        ))
        .id()
}

/// A reader button was activated (click, Enter, gamepad confirm).
pub(crate) fn on_reader_button(
    activate: On<Activate>,
    buttons: Query<&ReaderButton>,
    mut out: MessageWriter<ReaderEvent>,
) {
    if let Ok(b) = buttons.get(activate.entity) {
        out.write(ReaderEvent {
            reader: b.reader,
            action: b.action,
        });
    }
}

/// The screen of `entity`: its nearest [`UiScreenRoot`] (or itself).
fn screen_root<'a>(
    entity: Entity,
    parents: &Query<&ChildOf>,
    roots: &'a Query<&UiScreenRoot>,
) -> Option<&'a UiScreenRoot> {
    let mut current = entity;
    loop {
        if let Ok(root) = roots.get(current) {
            return Some(root);
        }
        current = parents.get(current).ok()?.parent();
    }
}

/// Screen UI actions and the mouse wheel → reader events. Shoulder buttons
/// (tab prev/next) turn pages, Context1 takes, Back closes.
pub(crate) fn reader_input(
    mut actions: MessageReader<UiActionEvent>,
    mut wheel: MessageReader<MouseWheel>,
    readers: Query<(Entity, &RichReader)>,
    parents: Query<&ChildOf>,
    roots: Query<&UiScreenRoot>,
    stack: Option<Res<ScreenStackRes>>,
    mut out: MessageWriter<ReaderEvent>,
) {
    for event in actions.read() {
        let action = match event.action {
            UiAction::TabPrev => ReaderAction::Prev,
            UiAction::TabNext => ReaderAction::Next,
            UiAction::Context1 => ReaderAction::Take,
            UiAction::Back => ReaderAction::Close,
            _ => continue,
        };
        for (entity, reader) in &readers {
            if action == ReaderAction::Take && !reader.takeable {
                continue;
            }
            if screen_root(entity, &parents, &roots).is_some_and(|r| r.0 == event.screen) {
                out.write(ReaderEvent {
                    reader: entity,
                    action,
                });
            }
        }
    }
    let mut lines = 0.0;
    for w in wheel.read() {
        lines -= match w.unit {
            MouseScrollUnit::Line => w.y,
            MouseScrollUnit::Pixel => w.y / WHEEL_LINE,
        };
    }
    if lines != 0.0 {
        let target = stack
            .as_ref()
            .and_then(|s| s.0.input_target())
            .map(|s| s.id.clone());
        for (entity, reader) in &readers {
            let root = screen_root(entity, &parents, &roots);
            let live = match (&target, root) {
                (Some(t), Some(r)) => *t == r.0,
                (_, None) => true,
                (None, Some(_)) => false,
            };
            if live && reader.layout == ReaderLayout::Scroll {
                out.write(ReaderEvent {
                    reader: entity,
                    action: ReaderAction::Scroll(lines * WHEEL_LINE),
                });
            }
        }
    }
}

/// Applies Prev, Next and Scroll.
pub(crate) fn apply_reader_events(
    mut events: MessageReader<ReaderEvent>,
    mut readers: Query<(&RichReader, &mut ReaderState)>,
    mut scrolls: Query<(&mut ScrollPosition, &ComputedNode)>,
) {
    for event in events.read() {
        let Ok((reader, mut state)) = readers.get_mut(event.reader) else {
            continue;
        };
        match (reader.layout, event.action) {
            (ReaderLayout::Book, ReaderAction::Prev) => state.view = state.view.saturating_sub(1),
            (ReaderLayout::Book, ReaderAction::Next) => {
                state.view = (state.view + 1).min(state.view_count().saturating_sub(1));
            }
            (ReaderLayout::Scroll, ReaderAction::Prev | ReaderAction::Next | ReaderAction::Scroll(_)) => {
                let Some(viewport) = state.chrome.as_ref().and_then(|c| c.viewport) else {
                    continue;
                };
                let Ok((mut pos, node)) = scrolls.get_mut(viewport) else {
                    continue;
                };
                let page = 0.9 * node.content_box().height() * node.inverse_scale_factor();
                let delta = match event.action {
                    ReaderAction::Prev => -page,
                    ReaderAction::Next => page,
                    ReaderAction::Scroll(d) => d,
                    _ => 0.0,
                };
                // bevy_ui clamps the offset to the content during layout.
                pos.y = (pos.y + delta).max(0.0);
            }
            _ => {}
        }
    }
}

/// Every image a document draws.
fn doc_images(doc: &RichDoc) -> impl Iterator<Item = &ImageRef> {
    doc.blocks.iter().flat_map(|b| -> Box<dyn Iterator<Item = &ImageRef>> {
        match b {
            Block::Paragraph(p) => Box::new(p.inlines.iter().filter_map(|i| match i {
                Inline::Image(image) => Some(image),
                Inline::Text { .. } => None,
            })),
            Block::Image { image, .. } => Box::new(std::iter::once(image)),
            Block::PageBreak => Box::new(std::iter::empty()),
        }
    })
}

/// Whether every image of `doc` has a known size (loaded, failed, or never
/// going to load), resolving sources on the way.
fn images_ready(
    doc: &RichDoc,
    images: &mut RichImages,
    server: Option<&AssetServer>,
    assets: &Assets<Image>,
) -> bool {
    doc_images(doc).all(|image| match images.resolve(&image.source, server) {
        None => true,
        Some(h) => {
            assets.contains(&h)
                || server.is_none_or(|s| s.load_state(&h).is_failed() || s.get_load_state(&h).is_none())
        }
    })
}

/// Re-paginates readers whose page size, scale, theme, view or document
/// changed, keeping the bookmark, and draws the open view.
#[allow(clippy::too_many_arguments)]
pub(crate) fn layout_readers(
    mut commands: Commands,
    mut readers: Query<(&mut RichReader, &mut ReaderState)>,
    bodies: Query<&ComputedNode, With<ReaderBody>>,
    fragments: Query<(Entity, &ChildOf), With<ReaderFragment>>,
    mut scrolls: Query<&mut ScrollPosition>,
    mut nodes: Query<&mut Node>,
    settings: Option<Res<UiSettings>>,
    mut measure: RichTextMeasure,
    server: Option<Res<AssetServer>>,
    image_assets: Res<Assets<Image>>,
    theme_ticks: Res<ThemeTick>,
) {
    let text_scale = settings.as_ref().map_or(1.0, |s| s.text_scale);
    let preset_view = if settings.as_ref().is_some_and(|s| s.preset.as_str() == "controller") {
        PageView::Single
    } else {
        PageView::Spread
    };
    for (mut reader, mut state) in &mut readers {
        let state = &mut *state;
        let Some(chrome) = state.chrome.clone() else {
            continue;
        };
        let Ok(body) = bodies.get(chrome.bodies[0]) else {
            continue;
        };
        let size = body.content_box().size();
        if size.x <= 0.0 || (reader.layout == ReaderLayout::Book && size.y <= 0.0) {
            continue; // not laid out yet
        }
        let page_view = match reader.layout {
            ReaderLayout::Book => reader.view.unwrap_or(preset_view),
            ReaderLayout::Scroll => PageView::Single,
        };
        let scale = MeasureScale {
            scale_factor: body.inverse_scale_factor().recip(),
            text_scale,
        };
        // A scroll's body grows with its content: only its width counts.
        let height = if reader.layout == ReaderLayout::Book { size.y } else { 0.0 };
        let made_for = MadeFor {
            doc: Arc::as_ptr(&reader.doc) as usize,
            size: (size.x.to_bits(), height.to_bits()),
            scale,
            page_view,
            theme_tick: theme_ticks.0,
        };
        // An outside change to the bookmark reopens there.
        if state.own_bookmark.is_some_and(|b| b != reader.bookmark)
            && let Some(p) = &state.pagination
        {
            state.view = p.view_of(reader.bookmark, page_view);
        }
        if state.made_for.as_ref() != Some(&made_for) {
            if !images_ready(&reader.doc, measure.images_mut(), server.as_deref(), &image_assets) {
                continue;
            }
            let Some(pagination) = paginate(&reader, size, &mut measure, scale) else {
                continue; // fonts still loading
            };
            state.view = pagination.view_of(reader.bookmark, page_view);
            state.pagination = Some(pagination);
            state.page_view = page_view;
            state.made_for = Some(made_for);
            state.generation += 1;
            if let Some(right) = chrome.right_page
                && let Ok(mut node) = nodes.get_mut(right)
            {
                show(&mut node, page_view == PageView::Spread);
            }
        }
        let Some(pagination) = &state.pagination else {
            continue;
        };
        state.view = state.view.min(pagination.view_count(page_view).saturating_sub(1));
        let shown = (state.generation, state.view);
        if state.shown == Some(shown) {
            // A scroll's bookmark follows its scroll offset.
            if reader.layout == ReaderLayout::Scroll
                && let Some(viewport) = chrome.viewport
                && let Ok(pos) = scrolls.get(viewport)
                && let Some(at) = scroll_bookmark(&pagination.pages()[0], pos.y * scale.scale_factor)
                && at != reader.bookmark
            {
                reader.bookmark = at;
                state.own_bookmark = Some(at);
            }
            continue;
        }
        for (fragment, parent) in &fragments {
            if chrome.bodies.contains(&parent.parent()) {
                commands.entity(fragment).despawn();
            }
        }
        let pages = pagination.view(state.view, page_view);
        let width = pagination.format().width;
        draw_pages(&mut commands, &chrome, &reader.doc, pages, width, &mut measure, scale, &image_assets);
        if reader.layout == ReaderLayout::Scroll
            && let Some(viewport) = chrome.viewport
            && let Ok(mut pos) = scrolls.get_mut(viewport)
            && let Ok(mut body) = nodes.get_mut(chrome.bodies[0])
        {
            open_scroll(&pagination.pages()[0], reader.bookmark, scale, &mut pos, &mut body);
        }
        state.shown = Some(shown);
        let bookmark = pages.first().map_or(DocPos::default(), |p| p.start);
        if reader.layout == ReaderLayout::Book && reader.bookmark != bookmark {
            reader.bookmark = bookmark;
        }
        state.own_bookmark = Some(reader.bookmark);
        let last = pagination.view_count(page_view).saturating_sub(1);
        set_disabled(&mut commands, chrome.prev, reader.layout == ReaderLayout::Book && state.view == 0);
        set_disabled(&mut commands, chrome.next, reader.layout == ReaderLayout::Book && state.view >= last);
    }
}

/// The reader's document cut for a page body of `size` (physical pixels);
/// `None` while a font is still loading.
fn paginate(
    reader: &RichReader,
    size: bevy_math::Vec2,
    measure: &mut RichTextMeasure,
    scale: MeasureScale,
) -> Option<Pagination> {
    let format = match reader.layout {
        ReaderLayout::Book => PageFormat::page(size.x, size.y),
        ReaderLayout::Scroll => PageFormat::scroll(size.x),
    }
    .with_gap(measure.theme().gap(scale.text_scale) * scale.scale_factor);
    let mut m = measure.at(scale);
    let pagination = Pagination::paginate(&reader.doc, &mut m, format);
    (!m.incomplete()).then_some(pagination)
}

/// Draws the open view's pages into the reader's page bodies.
#[allow(clippy::too_many_arguments)]
fn draw_pages(
    commands: &mut Commands,
    chrome: &Chrome,
    doc: &RichDoc,
    pages: &[Page],
    width: f32,
    measure: &mut RichTextMeasure,
    scale: MeasureScale,
    image_assets: &Assets<Image>,
) {
    let tails: Vec<_> = {
        let mut m = measure.at(scale);
        pages
            .iter()
            .map(|page| fragment_tails(doc, page, &mut m, width))
            .collect()
    };
    let ctx = FragmentCtx {
        theme: measure.theme(),
        scale,
        images: measure.images(),
        image_assets,
    };
    for (i, page) in pages.iter().enumerate() {
        if let Some(&body) = chrome.bodies.get(i) {
            spawn_page(commands, body, doc, page, &tails[i], &ctx);
        }
    }
}

fn show(node: &mut Node, visible: bool) {
    let display = if visible { Display::Flex } else { Display::None };
    if node.display != display {
        node.display = display;
    }
}

/// Where a scroll is read: the first item still visible at scroll offset `y`
/// (physical pixels).
fn scroll_bookmark(page: &Page, y: f32) -> Option<DocPos> {
    page.items
        .iter()
        .find(|p| p.y + p.height > y)
        .map(sf_ui::Placed::start)
}

/// Scrolls to the item holding `bookmark` and sizes the scroll body to the
/// text, so the viewport scrolls to its end.
fn open_scroll(page: &Page, bookmark: DocPos, scale: MeasureScale, pos: &mut ScrollPosition, body: &mut Node) {
    let y = page
        .items
        .iter()
        .find(|p| p.start() >= bookmark)
        .map_or(0.0, |p| p.y);
    pos.y = y / scale.scale_factor;
    body.height = Val::Px(page.content_height() / scale.scale_factor);
}

fn set_disabled(commands: &mut Commands, entity: Entity, disabled: bool) {
    if disabled {
        commands.entity(entity).insert(InteractionDisabled);
    } else {
        commands.entity(entity).remove::<InteractionDisabled>();
    }
}

/// Bumped when the theme changes (the reader re-paginates).
#[derive(Resource, Default, Debug)]
pub(crate) struct ThemeTick(pub u32);

pub(crate) fn bump_theme_tick(theme: Res<RichTextTheme>, mut tick: ResMut<ThemeTick>) {
    if theme.is_changed() {
        tick.0 = tick.0.wrapping_add(1);
    }
}

/// What drawing a page needs.
pub struct FragmentCtx<'a> {
    pub theme: &'a RichTextTheme,
    pub scale: MeasureScale,
    pub images: &'a RichImages,
    pub image_assets: &'a Assets<Image>,
}

impl FragmentCtx<'_> {
    fn image_logical(&self, image: &ImageRef) -> bevy_math::Vec2 {
        let native = self
            .images
            .get(&image.source)
            .and_then(|h| self.image_assets.get(h))
            .map(|i| i.size().as_vec2());
        image_logical_size(image, native, self.theme, self.scale.text_scale)
    }
}

/// Spawns one page's fragments under `body`, placed absolutely in logical
/// pixels (the pagination is in physical pixels).
///
/// `tails[i]` is where the line after item `i` ends, for a paragraph that
/// goes on past it ([`fragment_tails`]): that line is drawn too, invisible,
/// so the fragment shapes and justifies as inside the whole paragraph (the
/// last glyph keeps its kerning against the next one, the page's last line
/// is justified). The node keeps the paginated height; the tail, and the
/// empty line parley adds after a final hard break, hang below it unseen.
pub fn spawn_page(
    commands: &mut Commands,
    body: Entity,
    doc: &RichDoc,
    page: &Page,
    tails: &[Option<usize>],
    ctx: &FragmentCtx,
) {
    let s = ctx.scale.scale_factor;
    for (i, placed) in page.items.iter().enumerate() {
        let top = Val::Px(placed.y / s);
        match (&placed.fragment, &doc.blocks[placed.block]) {
            (Fragment::Lines { range, .. }, Block::Paragraph(p)) => {
                let slice = p.slice(range.clone());
                if slice.is_empty() {
                    commands.spawn((
                        ReaderFragment {
                            block: placed.block,
                        },
                        Node {
                            position_type: PositionType::Absolute,
                            top,
                            height: Val::Px(placed.height / s),
                            width: Val::Percent(100.0),
                            ..Node::default()
                        },
                        ChildOf(body),
                    ));
                    continue;
                }
                let tail = tails
                    .get(i)
                    .copied()
                    .flatten()
                    .map(|end| p.slice(range.end..end));
                let text = spawn_paragraph(commands, &slice, tail.as_ref(), ctx);
                commands.entity(text).insert((
                    ReaderFragment {
                        block: placed.block,
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        top,
                        width: Val::Percent(100.0),
                        height: Val::Px(placed.height / s),
                        ..Node::default()
                    },
                    ChildOf(body),
                ));
            }
            (Fragment::Image { size }, Block::Image { image, align }) => {
                let width = size.width / s;
                let left = match align {
                    sf_ui::Align::Start | sf_ui::Align::Justify => Val::Px(0.0),
                    sf_ui::Align::Center => Val::Percent(50.0),
                    sf_ui::Align::End => Val::Percent(100.0),
                };
                let margin = match align {
                    sf_ui::Align::Center => Val::Px(-width / 2.0),
                    sf_ui::Align::End => Val::Px(-width),
                    _ => Val::Px(0.0),
                };
                let handle = ctx.images.get(&image.source).cloned().unwrap_or_default();
                commands.spawn((
                    ReaderFragment {
                        block: placed.block,
                    },
                    part("sf-reader-image"),
                    Node {
                        position_type: PositionType::Absolute,
                        top,
                        left,
                        margin: UiRect::left(margin),
                        width: Val::Px(width),
                        height: Val::Px(size.height / s),
                        ..Node::default()
                    },
                    ImageNode::new(handle),
                    ChildOf(body),
                ));
            }
            _ => {}
        }
    }
}

/// For each item of `page`: where the line after it ends, when the item is
/// lines of a paragraph that continues on a later page (see [`spawn_page`]).
/// A line that ends inside a word (a word longer than a line) extends to the
/// word's end: whether the line before breaks depends on the whole word.
pub fn fragment_tails(
    doc: &RichDoc,
    page: &Page,
    measure: &mut impl TextMeasure,
    width: f32,
) -> Vec<Option<usize>> {
    page.items
        .iter()
        .map(|placed| match (&placed.fragment, &doc.blocks[placed.block]) {
            (Fragment::Lines { range, .. }, Block::Paragraph(p)) if range.end < p.len() => {
                let next = measure
                    .lines(p, width)
                    .iter()
                    .find(|l| l.range.start == range.end)?
                    .range
                    .end;
                let text = p.text();
                let word_end = text[next..]
                    .find(char::is_whitespace)
                    .map_or(text.len(), |i| next + i);
                Some(word_end)
            }
            _ => None,
        })
        .collect()
}

/// A drawn but invisible span: the line after a fragment (shaping context).
#[derive(Component, Clone, Copy, Debug)]
pub struct ReaderTail;

/// Spawns `paragraph` as one `Text` with a `TextSpan` per run and an
/// `InlineImage` per inline image, styled exactly as the measurer lays it
/// out, then `tail` the same way but transparent ([`ReaderTail`]). Returns
/// the text entity (no `Node` placement yet).
pub fn spawn_paragraph(
    commands: &mut Commands,
    paragraph: &Paragraph,
    tail: Option<&Paragraph>,
    ctx: &FragmentCtx,
) -> Entity {
    let ts = ctx.scale.text_scale;
    let text = commands
        .spawn((
            Text::new(""),
            TextLayout {
                justify: justify(paragraph.align),
                linebreak: DOC_LINE_BREAK,
            },
            ctx.theme.text_font(&RunStyle::default(), ts),
            ctx.theme.line_height(),
            TextColor(ctx.theme.color),
        ))
        .id();
    let shown = paragraph.inlines.iter().map(|i| (i, true));
    let hidden = tail.into_iter().flat_map(|t| &t.inlines).map(|i| (i, false));
    for (inline, visible) in shown.chain(hidden) {
        let mut entity = match inline {
            Inline::Text { text: run, style } => {
                let color = if visible {
                    ctx.theme.color(style)
                } else {
                    Color::NONE
                };
                let mut span = commands.spawn((
                    TextSpan::new(run.clone()),
                    ctx.theme.text_font(style, ts),
                    ctx.theme.line_height(),
                    TextColor(color),
                    ChildOf(text),
                ));
                if style.underline && visible {
                    span.insert(Underline);
                }
                span
            }
            Inline::Image(image) => {
                let size = ctx.image_logical(image);
                let handle = ctx.images.get(&image.source).cloned().unwrap_or_default();
                commands.spawn((
                    InlineImage {
                        image: handle,
                        width: Some(size.x),
                        height: Some(size.y),
                        color: if visible { Color::WHITE } else { Color::NONE },
                        ..InlineImage::default()
                    },
                    ChildOf(text),
                ))
            }
        };
        if !visible {
            entity.insert(ReaderTail);
        }
    }
    text
}
