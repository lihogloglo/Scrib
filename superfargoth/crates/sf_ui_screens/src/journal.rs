//! The journal (journal-ui.md) over `sf_rpg_ui::JournalScreen`: the book on the left, the
//! quest list and the topic index (letter tabs) on the right.
//!
//! The book is the same kind of view as `sf_ui_bevy`'s book reader (ui-text.md): pages are
//! measured on the laid-out page body node, and lines come from Bevy's own text layout
//! (`RichTextMeasure`, fonts from `RichTextTheme`), so text never clips. The journal keeps its
//! own paginator (`sf_rpg_ui::LinePaginator`: orphans, widows, headings kept with their
//! entry), fed by a line measure over that same layout. A resize, a text-scale change, a
//! theme change or new game data re-paginates and keeps the reader's place.
//!
//! Game data comes through a [`JournalSource`] resource ([`JournalUiPlugin`] is generic over
//! it). "New" markers clear when the player leaves a spread (turns the page, opens something
//! else); every change of the seen state goes out as [`JournalSeenChanged`] for the profile.

use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Range;

use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use bevy_text::{TextBackgroundColor, TextColor, TextLayout, TextSpan};
use bevy_ui::prelude::*;
use bevy_ui::{ComputedNode, ScrollPosition};
use bevy_ui_widgets::{Activate, ScrollArea};
use sf_rpg::{RecordId, WorldRpg};
use sf_rpg_ui::{
    Block, BlockStyle, BookContent, JournalCtx, JournalLabels, JournalScreen, JournalSeen,
    LinePaginator, PageMetrics, QuestFilter, TextMeasure as BookMeasure,
};
use sf_ui::{Inline, Paragraph, RunStyle, TextMeasure};
use sf_ui_bevy::{
    DOC_LINE_BREAK, MeasureScale, ParleyMeasure, RichTextMeasure, RichTextTheme, UiAction,
    UiActionEvent, UiDefaultFocus, UiScreenRoot, UiSettings,
};

use crate::keyed::KeyedChildren;
use crate::labels::ScreenLabels;
use crate::widgets::{
    ScreenColors, centred_window, classes, panel, part, part_with, relabel, set_disabled, show,
    spawn_button,
};
use crate::{ScreenSystems, screen_of};

/// The game's journal data: the replicated world state and the content pack's labels.
pub trait JournalSource: Resource {
    fn world(&self) -> &WorldRpg;
    fn labels(&self) -> &dyn JournalLabels;
}

/// The journal screen for a [`JournalSource`] `R` (after [`crate::SfScreensPlugin`]).
pub struct JournalUiPlugin<R>(PhantomData<fn() -> R>);

impl<R> Default for JournalUiPlugin<R> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<R: JournalSource> Plugin for JournalUiPlugin<R> {
    fn build(&self, app: &mut App) {
        app.add_message::<JournalSeenChanged>()
            .add_observer(on_journal_button)
            .add_systems(Update, spawn_journal.in_set(ScreenSystems::Spawn))
            .add_systems(Update, journal_input.in_set(ScreenSystems::Input))
            .add_systems(Update, layout_journal::<R>.in_set(ScreenSystems::Sync));
    }
}

/// The seen state changed (save it with the player's profile).
#[derive(Message, Clone, Debug, PartialEq)]
pub struct JournalSeenChanged {
    pub screen: Entity,
    pub seen: JournalSeen,
}

/// The side panel's list.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum JournalPanel {
    #[default]
    Quests,
    Topics,
}

/// Something the player asked of the journal; applied where the game data is at hand.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JournalIntent {
    Prev,
    Next,
    Open(BookContent),
    Panel(JournalPanel),
    QuestFilter(QuestFilter),
    Letter(Option<String>),
    MarkAllSeen,
}

/// On a screen root: the journal.
#[derive(Component)]
#[require(Node)]
pub struct JournalUi {
    pub screen: JournalScreen,
    panel: JournalPanel,
    intents: Vec<JournalIntent>,
    opened: bool,
}

impl JournalUi {
    pub fn new(screen: JournalScreen) -> Self {
        Self {
            screen,
            panel: JournalPanel::default(),
            intents: Vec::new(),
            opened: false,
        }
    }

    /// Queues an intent (applied in the next layout pass).
    pub fn request(&mut self, intent: JournalIntent) {
        self.intents.push(intent);
    }

    pub fn panel(&self) -> JournalPanel {
        self.panel
    }
}

/// A journal button and its intent.
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct JournalButton(pub JournalIntent);

/// A page body of the book: fragments of visible page `index` go here.
#[derive(Component, Clone, Copy, Debug)]
pub struct JournalPageBody(pub usize);

/// A drawn block slice: the block index in the book.
#[derive(Component, Clone, Copy, Debug)]
pub struct JournalFragment(pub usize);

#[derive(Clone, Debug, PartialEq)]
struct MadeFor {
    size: (u32, u32),
    scale: MeasureScale,
    pages: usize,
}

/// What the side panel was drawn for.
#[derive(Clone, Debug, PartialEq, Eq)]
struct IndexKey {
    panel: JournalPanel,
    quests: u64,
    topics: u64,
    letter: Option<String>,
    content: BookContent,
}

/// The journal's parts.
#[derive(Component, Debug)]
pub struct JournalParts {
    bodies: [Entity; 2],
    right_page: Entity,
    prev: Entity,
    next: Entity,
    page_label: Entity,
    filters: Entity,
    letters: KeyedChildren<String>,
    letters_box: Entity,
    rows: KeyedChildren<RecordId>,
    row_shown: HashMap<RecordId, (u64, bool)>,
    rows_for: Option<JournalPanel>,
    index_seen: Option<IndexKey>,
    made_for: Option<MadeFor>,
    shown: Option<(u64, usize)>,
    generation: u64,
}

impl JournalParts {
    pub fn bodies(&self) -> [Entity; 2] {
        self.bodies
    }

    /// A row of the side list (quest or topic).
    pub fn row(&self, id: &RecordId) -> Option<Entity> {
        self.rows.get(id)
    }

    pub fn rows(&self) -> impl Iterator<Item = (&RecordId, Entity)> {
        self.rows.iter()
    }

    pub fn letter(&self, label: &str) -> Option<Entity> {
        self.letters.get(&label.to_owned())
    }

    pub fn next_button(&self) -> Entity {
        self.next
    }

    pub fn prev_button(&self) -> Entity {
        self.prev
    }
}

#[allow(clippy::too_many_lines)]
pub(crate) fn spawn_journal(
    mut commands: Commands,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    roots: Query<Entity, Added<JournalUi>>,
) {
    for root in &roots {
        commands.entity(root).insert((
            part("sf-journal"),
            centred_window(Node {
                column_gap: Val::Px(16.0),
                padding: UiRect::all(Val::Px(12.0)),
                ..Node::default()
            }),
            BackgroundColor(colors.panel),
        ));
        let book = commands
            .spawn((
                part("sf-journal-book"),
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id();
        let pages = commands
            .spawn((
                part("sf-reader-pages"),
                Node {
                    column_gap: Val::Px(32.0),
                    ..Node::default()
                },
                ChildOf(book),
            ))
            .id();
        let mut bodies = [Entity::PLACEHOLDER; 2];
        let mut right_page = Entity::PLACEHOLDER;
        for (index, body) in bodies.iter_mut().enumerate() {
            let page = commands
                .spawn((
                    part("sf-reader-page"),
                    Node {
                        width: Val::Px(340.0),
                        height: Val::Px(460.0),
                        padding: UiRect::all(Val::Px(20.0)),
                        ..Node::default()
                    },
                    BackgroundColor(bevy_color::Color::srgb(0.86, 0.8, 0.66)),
                    ChildOf(pages),
                ))
                .id();
            if index == 1 {
                right_page = page;
            }
            *body = commands
                .spawn((
                    part("sf-reader-page-body"),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        ..Node::default()
                    },
                    JournalPageBody(index),
                    ChildOf(page),
                ))
                .id();
        }
        let controls = commands
            .spawn((
                part("sf-reader-controls"),
                Node {
                    column_gap: Val::Px(16.0),
                    align_items: AlignItems::Center,
                    ..Node::default()
                },
                ChildOf(book),
            ))
            .id();
        let (prev, _) = spawn_button(
            &mut commands,
            Some(controls),
            "sf-reader-prev",
            labels.prev.clone(),
            &colors,
        );
        commands
            .entity(prev)
            .insert(JournalButton(JournalIntent::Prev));
        let page_label = commands
            .spawn((
                part("sf-journal-page-number"),
                Text::new(""),
                colors.font(0.9),
                TextColor(colors.text),
                ChildOf(controls),
            ))
            .id();
        let (next, _) = spawn_button(
            &mut commands,
            Some(controls),
            "sf-reader-next",
            labels.next.clone(),
            &colors,
        );
        commands
            .entity(next)
            .insert((JournalButton(JournalIntent::Next), UiDefaultFocus));

        let side = commands
            .spawn((
                panel(
                    "sf-journal-index",
                    Node {
                        flex_direction: FlexDirection::Column,
                        width: Val::Px(280.0),
                        row_gap: Val::Px(6.0),
                        ..Node::default()
                    },
                    &colors,
                ),
                ChildOf(root),
            ))
            .id();
        let tabs = commands
            .spawn((
                part("sf-journal-tabs"),
                Node {
                    column_gap: Val::Px(4.0),
                    ..Node::default()
                },
                ChildOf(side),
            ))
            .id();
        for (name, text, intent) in [
            (
                "sf-journal-tab",
                labels.journal.clone(),
                JournalIntent::Open(BookContent::Journal),
            ),
            (
                "sf-journal-tab",
                labels.quests.clone(),
                JournalIntent::Panel(JournalPanel::Quests),
            ),
            (
                "sf-journal-tab",
                labels.topics.clone(),
                JournalIntent::Panel(JournalPanel::Topics),
            ),
        ] {
            let (e, _) = spawn_button(&mut commands, Some(tabs), name, text, &colors);
            commands.entity(e).insert(JournalButton(intent));
        }
        let filters = commands
            .spawn((
                part("sf-journal-filters"),
                Node {
                    column_gap: Val::Px(4.0),
                    ..Node::default()
                },
                ChildOf(side),
            ))
            .id();
        for (text, filter) in [
            (labels.active.clone(), QuestFilter::Active),
            (labels.finished.clone(), QuestFilter::Finished),
            (labels.all.clone(), QuestFilter::All),
        ] {
            let (e, _) = spawn_button(
                &mut commands,
                Some(filters),
                "sf-journal-filter",
                text,
                &colors,
            );
            commands
                .entity(e)
                .insert(JournalButton(JournalIntent::QuestFilter(filter)));
        }
        let letters_box = commands
            .spawn((
                part("sf-journal-letters"),
                Node {
                    flex_wrap: FlexWrap::Wrap,
                    column_gap: Val::Px(2.0),
                    display: Display::None,
                    ..Node::default()
                },
                ChildOf(side),
            ))
            .id();
        let list = commands
            .spawn((
                part("sf-journal-list"),
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    overflow: Overflow::scroll_y(),
                    ..Node::default()
                },
                ScrollPosition::default(),
                ScrollArea,
                ChildOf(side),
            ))
            .id();
        commands.entity(root).insert(JournalParts {
            bodies,
            right_page,
            prev,
            next,
            page_label,
            filters,
            letters: KeyedChildren::new(letters_box),
            letters_box,
            rows: KeyedChildren::new(list),
            row_shown: HashMap::new(),
            rows_for: None,
            index_seen: None,
            made_for: None,
            shown: None,
            generation: 0,
        });
    }
}

/// `sf_rpg_ui`'s line measure on Bevy's text layout: a block's text in its style is one
/// paragraph, laid out at the page width by the same call that draws it.
pub struct JournalMeasure<'a> {
    inner: RefCell<ParleyMeasure<'a>>,
    width: f32,
    theme: RichTextTheme,
    scale: MeasureScale,
}

/// The run style a block style draws with.
pub fn block_run_style(style: BlockStyle) -> RunStyle {
    match style {
        BlockStyle::Title => RunStyle {
            size: 1.35,
            bold: true,
            ..RunStyle::default()
        },
        BlockStyle::Heading => RunStyle {
            bold: true,
            ..RunStyle::default()
        },
        BlockStyle::Body => RunStyle::default(),
    }
}

impl JournalMeasure<'_> {
    fn incomplete(&self) -> bool {
        self.inner.borrow().incomplete()
    }
}

impl BookMeasure for JournalMeasure<'_> {
    fn lines(&self, text: &str, style: BlockStyle) -> Vec<Range<usize>> {
        if text.is_empty() {
            return Vec::new();
        }
        let paragraph = Paragraph {
            inlines: vec![Inline::Text {
                text: text.to_owned(),
                style: block_run_style(style),
            }],
            ..Paragraph::default()
        };
        self.inner
            .borrow_mut()
            .lines(&paragraph, self.width)
            .into_iter()
            .map(|l| l.range)
            .filter(|r| !r.is_empty())
            .collect()
    }

    fn line_height(&self, style: BlockStyle) -> f32 {
        let size = block_run_style(style).size;
        self.theme.font_size
            * size
            * self.scale.text_scale
            * self.theme.line_height
            * self.scale.scale_factor
    }
}

/// Re-paginates on a new page size, scale, preset, theme or game data; applies intents (they
/// need the game data); draws the visible spread; syncs the index.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn layout_journal<R: JournalSource>(
    mut commands: Commands,
    source: Option<Res<R>>,
    settings: Option<Res<UiSettings>>,
    colors: Res<ScreenColors>,
    computed: Query<&ComputedNode>,
    fragments: Query<(Entity, &ChildOf), With<JournalFragment>>,
    mut journals: Query<(Entity, &mut JournalUi, &mut JournalParts)>,
    mut nodes: Query<&mut Node>,
    mut measure: RichTextMeasure,
    theme: Res<RichTextTheme>,
    mut seen_out: MessageWriter<JournalSeenChanged>,
) {
    let Some(source) = source else {
        return;
    };
    let theme_changed = theme.is_changed();
    let text_scale = settings.as_ref().map_or(1.0, |s| s.text_scale);
    let pages = if settings
        .as_ref()
        .is_some_and(|s| s.preset.as_str() == "controller")
    {
        1
    } else {
        2
    };
    for (entity, mut ui, mut parts) in &mut journals {
        let ui = &mut *ui;
        let parts = &mut *parts;
        let Ok(body) = computed.get(parts.bodies[0]) else {
            continue;
        };
        let size = body.content_box().size();
        if size.x <= 0.0 || size.y <= 0.0 {
            continue; // not laid out yet
        }
        let scale = MeasureScale {
            scale_factor: body.inverse_scale_factor().recip(),
            text_scale,
        };
        let made_for = MadeFor {
            size: (size.x.to_bits(), size.y.to_bits()),
            scale,
            pages,
        };
        let gap = theme.gap(text_scale) * scale.scale_factor;
        let fresh = parts.made_for.as_ref() != Some(&made_for);
        // Pagination and intents need the game data and the measure; most frames need neither.
        let changed =
            !ui.opened || fresh || source.is_changed() || theme_changed || !ui.intents.is_empty();
        if changed {
            let measure = JournalMeasure {
                inner: RefCell::new(measure.at(scale)),
                width: size.x,
                theme: theme.clone(),
                scale,
            };
            let paginator = LinePaginator::new(
                measure,
                PageMetrics {
                    height: size.y,
                    block_gap: gap,
                    ..PageMetrics::default()
                },
            );
            let ctx = JournalCtx {
                world: source.world(),
                labels: source.labels(),
                paginator: &paginator,
            };
            let refresh = fresh || source.is_changed() || theme_changed;
            apply_journal(entity, ui, pages, refresh, &ctx, &mut seen_out);
            parts.made_for = if paginator.measure.incomplete() {
                None // fonts still loading: measure again next frame
            } else {
                Some(made_for)
            };
        }

        // The visible spread, redrawn when the book or the spread changed.
        if changed {
            parts.generation += 1;
        }
        let shown = (parts.generation, ui.screen.first_visible_page());
        if parts.shown != Some(shown) {
            parts.shown = Some(shown);
            for (fragment, parent) in &fragments {
                if parts.bodies.contains(&parent.parent()) {
                    commands.entity(fragment).despawn();
                }
            }
            let blocks = ui.screen.blocks();
            for (i, page) in ui.screen.visible_pages().iter().enumerate() {
                let body = parts.bodies[i];
                for (n, slice) in page.slices.iter().enumerate() {
                    let block = &blocks[slice.block];
                    spawn_slice(
                        &mut commands,
                        body,
                        slice.block,
                        block,
                        slice.range.clone(),
                        n > 0,
                        &theme,
                        scale,
                        gap / scale.scale_factor,
                        &colors,
                    );
                }
            }
            let spread = ui.screen.spread_index();
            let count = ui.screen.spread_count();
            commands.entity(parts.page_label).insert(Text::new(format!(
                "{} / {}",
                if count == 0 { 0 } else { spread + 1 },
                count
            )));
            set_disabled(&mut commands, parts.prev, spread == 0);
            set_disabled(&mut commands, parts.next, spread + 1 >= count);
        }
        if let Ok(mut node) = nodes.get_mut(parts.right_page) {
            show(&mut node, ui.screen.pages_per_spread() == 2);
        }

        sync_index(&mut commands, ui, parts, &colors, &mut nodes);
    }
}

/// Opens or refreshes the book (first time, new geometry or data) and applies the queued
/// intents, reporting seen-state changes.
fn apply_journal(
    entity: Entity,
    ui: &mut JournalUi,
    pages: usize,
    refresh: bool,
    ctx: &JournalCtx<'_>,
    seen_out: &mut MessageWriter<JournalSeenChanged>,
) {
    if !ui.opened {
        ui.screen.set_pages_per_spread(pages);
        ui.screen.open(ctx, BookContent::Journal);
        ui.opened = true;
    } else if refresh {
        ui.screen.set_pages_per_spread(pages);
        ui.screen.refresh(ctx);
    }
    let intents = std::mem::take(&mut ui.intents);
    let seen_before = (!intents.is_empty()).then(|| ui.screen.seen().clone());
    for intent in intents {
        match intent {
            JournalIntent::Prev | JournalIntent::Next => {
                let before = ui.screen.first_visible_page();
                ui.screen.mark_visible_seen(ctx);
                ui.screen.go_to_page(before);
                if intent == JournalIntent::Next {
                    ui.screen.next_spread();
                } else {
                    ui.screen.prev_spread();
                }
            }
            JournalIntent::Open(content) => {
                ui.screen.mark_visible_seen(ctx);
                ui.screen.open(ctx, content);
            }
            JournalIntent::Panel(panel) => ui.panel = panel,
            JournalIntent::QuestFilter(filter) => {
                ui.panel = JournalPanel::Quests;
                ui.screen.set_quest_filter(filter);
            }
            JournalIntent::Letter(letter) => {
                ui.panel = JournalPanel::Topics;
                ui.screen.set_letter(letter.as_deref());
            }
            JournalIntent::MarkAllSeen => ui.screen.mark_all_seen(ctx),
        }
    }
    if seen_before.is_some_and(|s| s != *ui.screen.seen()) {
        seen_out.write(JournalSeenChanged {
            screen: entity,
            seen: ui.screen.seen().clone(),
        });
    }
}

/// One block slice: a container (for the `new` marker) holding a text whose spans split at
/// search highlights.
#[allow(clippy::too_many_arguments)]
fn spawn_slice(
    commands: &mut Commands,
    body: Entity,
    index: usize,
    block: &Block,
    range: Range<usize>,
    gap_before: bool,
    theme: &RichTextTheme,
    scale: MeasureScale,
    gap: f32,
    colors: &ScreenColors,
) {
    let style_class = match block.style {
        BlockStyle::Title => "title",
        BlockStyle::Heading => "heading",
        BlockStyle::Body => "body",
    };
    let new = block.new && range.start == 0;
    let container = commands
        .spawn((
            part_with(
                "sf-journal-block",
                classes("sf-journal-block", &[(style_class, true), ("new", new)]),
            ),
            JournalFragment(index),
            Node {
                margin: UiRect::top(Val::Px(if gap_before { gap } else { 0.0 })),
                ..Node::default()
            },
            ChildOf(body),
        ))
        .id();
    if new {
        commands.spawn((
            part("sf-journal-new-marker"),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(-10.0),
                top: Val::Px(6.0),
                width: Val::Px(5.0),
                height: Val::Px(5.0),
                ..Node::default()
            },
            BackgroundColor(colors.new),
            ChildOf(container),
        ));
    }
    let run = block_run_style(block.style);
    let text = commands
        .spawn((
            Text::new(""),
            TextLayout {
                linebreak: DOC_LINE_BREAK,
                ..TextLayout::default()
            },
            theme.text_font(&run, scale.text_scale),
            theme.line_height(),
            TextColor(theme.color),
            ChildOf(container),
        ))
        .id();
    // Spans: plain, highlighted, plain, ... clipped to this slice.
    let mut at = range.start;
    let mut pieces: Vec<(Range<usize>, bool)> = Vec::new();
    for h in &block.highlights {
        let (s, e) = (h.start.max(range.start), h.end.min(range.end));
        if s >= e {
            continue;
        }
        if s > at {
            pieces.push((at..s, false));
        }
        pieces.push((s..e, true));
        at = e;
    }
    if at < range.end {
        pieces.push((at..range.end, false));
    }
    for (piece, lit) in pieces {
        let Some(text_piece) = block.text.get(piece) else {
            continue;
        };
        let mut span = commands.spawn((
            TextSpan::new(text_piece.trim_end_matches('\n')),
            theme.text_font(&run, scale.text_scale),
            theme.line_height(),
            TextColor(theme.color),
            ChildOf(text),
        ));
        if lit {
            span.insert((
                TextBackgroundColor(colors.highlight),
                classes("sf-journal-hit", &[]),
            ));
        }
    }
}

/// The side panel: quest filters or letter tabs, and the keyed rows.
fn sync_index(
    commands: &mut Commands,
    ui: &mut JournalUi,
    parts: &mut JournalParts,
    colors: &ScreenColors,
    nodes: &mut Query<&mut Node>,
) {
    let key = IndexKey {
        panel: ui.panel,
        quests: ui.screen.quest_model().version(),
        topics: ui.screen.topic_model().version(),
        letter: ui.screen.letter().map(str::to_owned),
        content: ui.screen.content().clone(),
    };
    if parts.index_seen.as_ref() == Some(&key) {
        return;
    }
    parts.index_seen = Some(key);
    let topics = ui.panel == JournalPanel::Topics;
    if let Ok(mut node) = nodes.get_mut(parts.filters) {
        show(&mut node, !topics);
    }
    if let Ok(mut node) = nodes.get_mut(parts.letters_box) {
        show(&mut node, topics);
    }
    if parts.rows_for != Some(ui.panel) {
        // Another list: rows are different records.
        parts.rows.clear(commands);
        parts.row_shown.clear();
        parts.rows_for = Some(ui.panel);
    }
    let content = ui.screen.content().clone();
    if topics {
        let tabs = ui.screen.tabs().to_vec();
        let current = ui.screen.letter().map(str::to_owned);
        parts.letters.sync(
            commands,
            tabs.iter().map(|t| t.label.clone()),
            |commands, l| {
                let (e, _) = spawn_button(commands, None, "sf-journal-letter", l.clone(), colors);
                commands
                    .entity(e)
                    .insert(JournalButton(JournalIntent::Letter(Some(l.clone()))));
                e
            },
        );
        for tab in &tabs {
            if let Some(e) = parts.letters.get(&tab.label) {
                let selected = current.as_deref() == Some(tab.label.as_str());
                commands.entity(e).insert(classes(
                    "sf-journal-letter",
                    &[
                        ("selected", selected),
                        ("new", tab.new),
                        ("disabled", tab.count == 0),
                    ],
                ));
                set_disabled(commands, e, tab.count == 0);
            }
        }
        let keys = ui.screen.topic_list().to_vec();
        let model = ui.screen.topic_model();
        sync_rows(
            commands,
            parts,
            &keys,
            |id| {
                let row = model.get(id)?;
                Some((
                    model.row_version(id)?,
                    row.name.clone(),
                    row.new,
                    false,
                    content == BookContent::Topic(id.clone()),
                ))
            },
            |id| JournalIntent::Open(BookContent::Topic(id.clone())),
            colors,
        );
    } else {
        let keys = ui.screen.quest_list().to_vec();
        let model = ui.screen.quest_model();
        sync_rows(
            commands,
            parts,
            &keys,
            |id| {
                let row = model.get(id)?;
                Some((
                    model.row_version(id)?,
                    row.name.clone(),
                    row.new,
                    row.finished,
                    content == BookContent::Quest(id.clone()),
                ))
            },
            |id| JournalIntent::Open(BookContent::Quest(id.clone())),
            colors,
        );
    }
}

/// Keyed rows of the side list; a row redraws when its row version or selection changes.
fn sync_rows(
    commands: &mut Commands,
    parts: &mut JournalParts,
    keys: &[RecordId],
    row: impl Fn(&RecordId) -> Option<(u64, String, bool, bool, bool)>,
    intent: impl Fn(&RecordId) -> JournalIntent,
    colors: &ScreenColors,
) {
    parts
        .rows
        .sync(commands, keys.iter().cloned(), |commands, id| {
            let (e, _) = spawn_button(commands, None, "sf-journal-row", "", colors);
            commands.entity(e).insert(JournalButton(intent(id)));
            e
        });
    parts.row_shown.retain(|k, _| keys.contains(k));
    for id in keys {
        let (Some(entity), Some((version, name, new, finished, selected))) =
            (parts.rows.get(id), row(id))
        else {
            continue;
        };
        if parts.row_shown.get(id) == Some(&(version, selected)) {
            continue;
        }
        parts.row_shown.insert(id.clone(), (version, selected));
        commands.entity(entity).insert(classes(
            "sf-journal-row",
            &[("new", new), ("finished", finished), ("selected", selected)],
        ));
        let color = if new {
            colors.new
        } else if finished {
            colors.muted
        } else {
            colors.text
        };
        relabel(commands, entity, name, color);
    }
}

/// Shoulder buttons turn pages; Context1 switches the side list.
pub(crate) fn journal_input(
    mut actions: MessageReader<UiActionEvent>,
    mut journals: Query<(&UiScreenRoot, &mut JournalUi)>,
) {
    for action in actions.read() {
        for (root, mut ui) in &mut journals {
            if action.screen != root.0 {
                continue;
            }
            let intent = match action.action {
                UiAction::TabPrev => JournalIntent::Prev,
                UiAction::TabNext => JournalIntent::Next,
                UiAction::Context1 => JournalIntent::Panel(match ui.panel {
                    JournalPanel::Quests => JournalPanel::Topics,
                    JournalPanel::Topics => JournalPanel::Quests,
                }),
                _ => continue,
            };
            ui.request(intent);
        }
    }
}

/// A journal button activated (click, confirm).
pub(crate) fn on_journal_button(
    activate: On<Activate>,
    buttons: Query<&JournalButton>,
    parents: Query<&ChildOf>,
    mut journals: Query<&mut JournalUi>,
) {
    let Ok(JournalButton(intent)) = buttons.get(activate.entity) else {
        return;
    };
    if let Some(root) = screen_of(activate.entity, &parents, |e| journals.contains(e))
        && let Ok(mut ui) = journals.get_mut(root)
    {
        ui.request(intent.clone());
    }
}
