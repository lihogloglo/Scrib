//! The perf panel's `bevy_ui` tree (ROADMAP 1.25 CW-17b): spawned once when the panel shows (or
//! its look changes: mode, scale, graph shape, colours), then only mutated: texts, bar widths and
//! colours at the refresh rate, the graph and sparkline images at the graph rate. About 30 nodes
//! compact, ~80 full; nothing is spawned per frame.
//!
//! Layout (top right, dark translucent):
//! ```text
//! frame 8.10 ms (123 fps)  med 7.90  p99 10.80  max 14.20
//! bound by: GPU                                  F3 compact
//! +--------------------------------------------+ 25 ms
//! |   frame columns (bound colour), main/render | 11
//! |   /GPU lines, dashed 8.3 / 11 ms targets    | 8.3
//! +--------------------------------------------+
//! [] main [] render [] GPU [] none            last 5.0 s
//! main   [#####.................]  3.10 ms  38%
//! render [########..............]  4.20 ms  52%
//! GPU    [###################...]  7.60 ms  94%  < bound
//! -- full only --
//! GPU passes: 2.10 [####....] main_opaque_pass_3d ...
//! main stages: 1.20 [##......] Update ...
//! views / lights / encoders / entities text
//! submit ms [sparkline] 0.42 (max 1.10)    views [sparkline] 14 (max 22)
//! preset + tunables above the preset
//! ```

use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_color::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_image::{Image, ImageSampler};
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_text::prelude::*;
use bevy_ui::prelude::*;

use super::graph::{Palette, Rgba};
use crate::perf_panel_model::PanelMode;

/// What the tree was built for; a change rebuilds it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Look {
    pub mode: PanelMode,
    /// Logical px per authored px (`diag.panel.scale` over the window's scale factor).
    pub scale: f32,
    /// Physical px per authored px (images are drawn at this, one texel per pixel).
    pub px_scale: f32,
    /// The graph's authored size (px).
    pub graph: [f32; 2],
    pub top_ms: f32,
    pub targets: [f32; 2],
    pub palette: Palette,
    pub panel_alpha: f32,
    pub passes: usize,
}

/// A bar row: label, track with its fill, value text.
#[derive(Clone, Copy)]
pub(super) struct BarRow {
    row: Entity,
    label: Entity,
    fill: Entity,
    value: Entity,
}

/// A sparkline row: its image and value text.
pub(super) struct Spark {
    pub image: Option<Handle<Image>>,
    pub size: [u32; 2],
    value: Entity,
}

/// The spawned tree's handles.
pub(super) struct PanelUi {
    pub look: Look,
    pub root: Entity,
    pub(super) header: Entity,
    bound: Entity,
    pub graph: Option<Handle<Image>>,
    pub graph_px: [u32; 2],
    span: Entity,
    threads: [BarRow; 3],
    pub(super) notes: Entity,
    stages: Vec<BarRow>,
    passes: Vec<BarRow>,
    counts: Option<Entity>,
    pub sparks: Vec<Spark>,
    footer: Option<Entity>,
}

/// Main stages shown in the full panel.
// tunables: const (a layout: the three stages the text panel listed)
pub(super) const STAGES: usize = 3;

pub(super) fn colour(c: Rgba) -> Color {
    Color::srgba_u8(c[0], c[1], c[2], c[3])
}

/// The panel's greys (text and tracks); the data colours are tunables.
const TEXT: Color = Color::srgb(0.88, 0.9, 0.92);
const MUTED: Color = Color::srgb(0.55, 0.57, 0.6);
const TRACK: Color = Color::srgba(0.16, 0.17, 0.19, 0.9);

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "sizes of a few hundred px")]
fn texels(v: f32) -> u32 {
    v.round().max(1.0) as u32
}

fn new_image(images: &mut Assets<Image>, w: u32, h: u32) -> Handle<Image> {
    let mut image = Image::new_fill(
        Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.sampler = ImageSampler::nearest();
    images.add(image)
}

struct Spawner<'a, 'w, 's> {
    commands: &'a mut Commands<'w, 's>,
    s: f32,
}

impl Spawner<'_, '_, '_> {
    fn node(&mut self, parent: Entity, node: Node) -> Entity {
        self.commands.spawn((node, ChildOf(parent))).id()
    }

    fn text(&mut self, parent: Entity, text: &str, size: f32, color: Color, node: Node) -> Entity {
        self.commands.spawn((Text::new(text), TextFont::from_font_size(size * self.s), TextColor(color), node, ChildOf(parent))).id()
    }

    fn row(&mut self, parent: Entity) -> Entity {
        let s = self.s;
        self.node(parent, Node { flex_direction: FlexDirection::Row, align_items: AlignItems::Center, column_gap: px(6.0 * s), ..Node::default() })
    }

    /// `label | track(fill) | value`, the label `label_w` wide (0: no label column).
    fn bar(&mut self, parent: Entity, label: &str, label_w: f32, track_w: f32, height: f32, fill: Color) -> BarRow {
        let s = self.s;
        let row = self.row(parent);
        let label = self.text(row, label, 12.0, TEXT, Node { width: px(label_w * s), ..Node::default() });
        let track = self.commands.spawn((Node { width: px(track_w * s), height: px(height * s), ..Node::default() }, BackgroundColor(TRACK), ChildOf(row))).id();
        let fill = self.commands.spawn((Node { width: percent(0.0), height: percent(100.0), ..Node::default() }, BackgroundColor(fill), ChildOf(track))).id();
        let value = self.text(row, "", 12.0, TEXT, Node::default());
        BarRow { row, label, fill, value }
    }

    /// A value-first bar (GPU passes, main stages): `value | track | name`.
    fn list_bar(&mut self, parent: Entity, track_w: f32, fill: Color) -> BarRow {
        let s = self.s;
        let hidden = Node { display: Display::None, flex_direction: FlexDirection::Row, align_items: AlignItems::Center, column_gap: px(6.0 * s), ..Node::default() };
        let row = self.node(parent, hidden);
        let value = self.text(row, "", 11.0, TEXT, Node { width: px(40.0 * s), ..Node::default() });
        let track = self.commands.spawn((Node { width: px(track_w * s), height: px(7.0 * s), ..Node::default() }, BackgroundColor(TRACK), ChildOf(row))).id();
        let fill = self.commands.spawn((Node { width: percent(0.0), height: percent(100.0), ..Node::default() }, BackgroundColor(fill), ChildOf(track))).id();
        let label = self.text(row, "", 11.0, MUTED, Node::default());
        BarRow { row, label, fill, value }
    }
}

/// Spawns the tree for `look` (no images without `Assets<Image>`: text and bars only).
pub(super) fn spawn(commands: &mut Commands, mut images: Option<&mut Assets<Image>>, look: Look) -> PanelUi {
    let s = look.scale;
    let p = look.palette;
    let [gw, gh] = look.graph;
    let root = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: px(6.0),
                right: px(6.0),
                flex_direction: FlexDirection::Column,
                row_gap: px(3.0 * s),
                padding: UiRect::all(px(7.0 * s)),
                ..Node::default()
            },
            BackgroundColor(Color::srgba(0.02, 0.025, 0.03, look.panel_alpha)),
            GlobalZIndex(1000),
        ))
        .id();
    let mut sp = Spawner { commands, s };
    let header = sp.text(root, "perf panel: measuring...", 15.0, TEXT, Node::default());
    let line = sp.node(root, Node { flex_direction: FlexDirection::Row, justify_content: JustifyContent::SpaceBetween, ..Node::default() });
    let bound = sp.text(line, "bound by: n/a", 12.0, TEXT, Node::default());
    sp.text(line, &format!("F3 {}", look.mode.label()), 10.0, MUTED, Node::default());

    // The graph, its scale labels on the right edge.
    let graph_px = [texels(gw * look.px_scale), texels(gh * look.px_scale)];
    let frame = sp.node(root, Node { width: px(gw * s), height: px(gh * s), ..Node::default() });
    let graph = images.as_deref_mut().map(|images| new_image(images, graph_px[0], graph_px[1]));
    if let Some(handle) = &graph {
        sp.commands.spawn((ImageNode::new(handle.clone()), Node { width: percent(100.0), height: percent(100.0), ..Node::default() }, ChildOf(frame)));
    }
    let label_h = 10.0 * s;
    for (ms, c) in [(look.top_ms, MUTED), (look.targets[1], colour(p.target_p99)), (look.targets[0], colour(p.target))] {
        let top = (gh * s * (1.0 - ms / look.top_ms) - label_h * 0.6).clamp(0.0, (gh * s - label_h).max(0.0));
        let text = if (ms - look.top_ms).abs() < f32::EPSILON { format!("{ms:.0} ms") } else { format!("{ms}") };
        sp.text(frame, &text, 10.0, c, Node { position_type: PositionType::Absolute, right: px(3.0 * s), top: px(top), ..Node::default() });
    }
    // Legend and the span the graph covers.
    let legend = sp.node(root, Node { flex_direction: FlexDirection::Row, align_items: AlignItems::Center, column_gap: px(5.0 * s), ..Node::default() });
    for (name, c) in [("main", p.main), ("render", p.render), ("GPU", p.gpu), ("none", p.wait)] {
        sp.commands.spawn((Node { width: px(8.0 * s), height: px(8.0 * s), ..Node::default() }, BackgroundColor(colour(c)), ChildOf(legend)));
        sp.text(legend, name, 10.0, MUTED, Node { margin: UiRect::right(px(4.0 * s)), ..Node::default() });
    }
    let span = sp.text(legend, "", 10.0, MUTED, Node { margin: UiRect::left(Val::Auto), ..Node::default() });

    // Thread bars: the track is the frame.
    let (label_w, value_w) = (52.0, 120.0);
    let track_w = (gw - label_w - value_w - 12.0).max(40.0);
    let threads = [("main", p.main), ("render", p.render), ("GPU", p.gpu)].map(|(n, c)| sp.bar(root, n, label_w, track_w, 10.0, colour(c)));
    let notes = sp.text(root, "", 10.0, MUTED, Node::default());

    let mut ui = PanelUi { look, root, header, bound, graph, graph_px, span, threads, notes, stages: Vec::new(), passes: Vec::new(), counts: None, sparks: Vec::new(), footer: None };
    if look.mode != PanelMode::Full {
        return ui;
    }
    let list_w = (gw * 0.4).max(40.0);
    sp.text(root, "GPU passes (ms; bar = share of the frame)", 10.0, MUTED, Node { margin: UiRect::top(px(3.0 * s)), ..Node::default() });
    ui.passes = (0..look.passes).map(|_| sp.list_bar(root, list_w, colour(p.gpu))).collect();
    sp.text(root, "main stages (ms)", 10.0, MUTED, Node { margin: UiRect::top(px(3.0 * s)), ..Node::default() });
    ui.stages = (0..STAGES).map(|_| sp.list_bar(root, list_w, colour(p.main))).collect();
    ui.counts = Some(sp.text(root, "", 11.0, TEXT, Node { margin: UiRect::top(px(3.0 * s)), ..Node::default() }));
    let spark_w = (gw * 0.3).max(30.0);
    let spark_px = [texels(spark_w * look.px_scale), texels(14.0 * look.px_scale)];
    for name in ["submit ms", "views"] {
        let row = sp.row(root);
        sp.text(row, name, 11.0, MUTED, Node { width: px(70.0 * s), ..Node::default() });
        let image = images.as_deref_mut().map(|images| new_image(images, spark_px[0], spark_px[1]));
        let holder = sp.node(row, Node { width: px(spark_w * s), height: px(14.0 * s), ..Node::default() });
        if let Some(handle) = &image {
            sp.commands.spawn((ImageNode::new(handle.clone()), Node { width: percent(100.0), height: percent(100.0), ..Node::default() }, ChildOf(holder)));
        }
        let value = sp.text(row, "n/a", 11.0, TEXT, Node::default());
        ui.sparks.push(Spark { image, size: spark_px, value });
    }
    ui.footer = Some(sp.text(root, "", 11.0, TEXT, Node { margin: UiRect::top(px(3.0 * s)), ..Node::default() }));
    ui
}

/// Write access to the tree's components.
#[derive(SystemParam)]
pub(super) struct UiWrite<'w, 's> {
    texts: Query<'w, 's, &'static mut Text>,
    nodes: Query<'w, 's, &'static mut Node>,
    backgrounds: Query<'w, 's, &'static mut BackgroundColor>,
    colours: Query<'w, 's, &'static mut TextColor>,
    pub images: Option<ResMut<'w, Assets<Image>>>,
}

impl UiWrite<'_, '_> {
    fn text(&mut self, e: Entity, value: &str) {
        if let Ok(mut t) = self.texts.get_mut(e)
            && t.0 != value
        {
            value.clone_into(&mut t.0);
        }
    }

    fn text_colour(&mut self, e: Entity, c: Color) {
        if let Ok(mut t) = self.colours.get_mut(e) {
            t.0 = c;
        }
    }

    fn shown(&mut self, e: Entity, on: bool) {
        if let Ok(mut n) = self.nodes.get_mut(e) {
            let d = if on { Display::Flex } else { Display::None };
            if n.display != d {
                n.display = d;
            }
        }
    }

    fn fill(&mut self, bar: &BarRow, fraction: f32, c: Color) {
        if let Ok(mut n) = self.nodes.get_mut(bar.fill) {
            n.width = percent(fraction * 100.0);
        }
        if let Ok(mut b) = self.backgrounds.get_mut(bar.fill) {
            b.0 = c;
        }
    }
}

/// One refresh's content for the tree (the Bevy half computes it from the snapshot).
pub(super) struct View<'a> {
    pub header: String,
    pub bound: String,
    pub bound_colour: Color,
    pub span: String,
    /// main, render, GPU: fill share, value text, highlighted (the bound).
    pub threads: [(f32, String, bool); 3],
    pub notes: String,
    /// (value ms, share of the frame, name).
    pub passes: &'a [(f64, f32, String)],
    pub stages: &'a [(f64, f32, String)],
    pub counts: String,
    pub sparks: [String; 2],
    pub footer: String,
}

/// Writes `view` into the tree.
pub(super) fn update(ui: &PanelUi, w: &mut UiWrite, view: &View) {
    let p = ui.look.palette;
    w.text(ui.header, &view.header);
    w.text(ui.bound, &view.bound);
    w.text_colour(ui.bound, view.bound_colour);
    w.text(ui.span, &view.span);
    for ((bar, base), (fraction, value, bound)) in ui.threads.iter().zip([p.main, p.render, p.gpu]).zip(&view.threads) {
        let c = colour(base);
        w.fill(bar, *fraction, if *bound { c } else { c.with_alpha(0.55) });
        w.text(bar.value, value);
        w.text_colour(bar.label, if *bound { c } else { TEXT });
    }
    w.text(ui.notes, &view.notes);
    w.shown(ui.notes, !view.notes.is_empty());
    for (rows, items, base) in [(&ui.passes, view.passes, p.gpu), (&ui.stages, view.stages, p.main)] {
        for (i, bar) in rows.iter().enumerate() {
            let item = items.get(i);
            w.shown(bar.row, item.is_some());
            if let Some((ms, fraction, name)) = item {
                w.fill(bar, *fraction, colour(base));
                w.text(bar.value, &format!("{ms:.2}"));
                w.text(bar.label, name);
            }
        }
    }
    if let Some(e) = ui.counts {
        w.text(e, &view.counts);
    }
    for (spark, value) in ui.sparks.iter().zip(&view.sparks) {
        w.text(spark.value, value);
    }
    if let Some(e) = ui.footer {
        w.text(e, &view.footer);
    }
}
