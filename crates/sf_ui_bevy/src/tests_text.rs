//! Headless text tests (ui-text.md "Verification"): the real measurer
//! against `bevy_ui`'s own text layout, readers, message boxes and toasts.
//! A Bevy app without window or GPU: a camera with a fixed render target
//! drives `bevy_ui` layout, `bevy_text` shapes with its built-in font (and
//! with `DejaVuSans` too when the system has it).

use std::sync::Arc;
use std::time::Duration;

use bevy::asset::RenderAssetUsages;
use bevy::camera::{ComputedCameraValues, RenderTargetInfo};
use bevy::input::InputPlugin;
use bevy::picking::{InteractionPlugin, PickingPlugin};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::text::{ComputedTextBlock, FontSource, TextLayoutInfo, TextPlugin};
use bevy::time::TimeUpdateStrategy;
use bevy::ui::UiPlugin;
use bevy_ui::InteractionDisabled;
use bevy_ui_widgets::Activate;
use sf_records::{RecordDb, RecordFormat, apply_record_file};
use sf_ui::{
    Align, Block, DocPos, Fragment, ImageRef, PageView, Paragraph, RichDoc, RunStyle, UiRegistry,
    register_kinds,
};
use bevy_input_focus::InputFocus;

use crate::*;

const TARGET: UVec2 = UVec2::new(1600, 1000);
const DEJAVU: &str = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf";

fn ui_app(scale_factor: f32) -> App {
    ui_app_with(scale_factor, |_| {})
}

fn ui_app_with(scale_factor: f32, extra: impl FnOnce(&mut App)) -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        TransformPlugin,
        AssetPlugin::default(),
        ImagePlugin::default(),
        bevy::image::TextureAtlasPlugin,
        TextPlugin,
        UiPlugin,
        InputPlugin,
        PickingPlugin,
        InteractionPlugin,
        UiTextPlugin,
    ))
    .init_resource::<UiSettings>()
    .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(100)));
    extra(&mut app);
    app.world_mut().spawn((
        Camera2d,
        Camera {
            computed: ComputedCameraValues {
                target_info: Some(RenderTargetInfo {
                    physical_size: TARGET,
                    scale_factor,
                }),
                ..default()
            },
            viewport: Some(bevy::camera::Viewport {
                physical_size: TARGET,
                ..default()
            }),
            ..default()
        },
    ));
    app.finish();
    app.update();
    app
}

/// Uses `DejaVuSans` as the body face when the system has it.
fn use_system_font(app: &mut App) -> bool {
    let Ok(bytes) = std::fs::read(DEJAVU) else {
        return false;
    };
    let font = app
        .world_mut()
        .resource_mut::<Assets<Font>>()
        .add(Font::from_bytes(bytes));
    app.world_mut().resource_mut::<RichTextTheme>().body = FontSource::Handle(font);
    app.update();
    true
}

fn add_image(app: &mut App, source: &str, w: u32, h: u32) {
    let image = Image::new_fill(
        Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[255, 255, 255, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    let handle = app.world_mut().resource_mut::<Assets<Image>>().add(image);
    app.world_mut()
        .resource_mut::<RichImages>()
        .insert(source, handle);
}

/// Deterministic pseudo-random numbers (xorshift).
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: u64) -> usize {
        usize::try_from(self.next() % n).unwrap()
    }
}

fn word(rng: &mut Rng, len: usize) -> String {
    (0..len)
        .map(|_| char::from(b'a' + u8::try_from(rng.below(26)).unwrap()))
        .collect()
}

/// A synthetic book: prose in mixed sizes and weights, every alignment, hard
/// breaks, blank paragraphs, words longer than a line, lines of inline
/// images, block images and forced breaks.
fn synthetic_doc(seed: u64) -> RichDoc {
    let mut rng = Rng(seed);
    let mut doc = RichDoc {
        background: Some(ImageRef::new("parchment")),
        ..RichDoc::default()
    };
    let aligns = [Align::Start, Align::Center, Align::End, Align::Justify];
    for i in 0..28 {
        match rng.below(12) {
            0 => doc.push(Block::Image {
                image: ImageRef::new("plate"),
                align: aligns[rng.below(3)],
            }),
            1 if i > 0 => doc.push(Block::PageBreak),
            2 => doc.push(Block::Paragraph(Paragraph::new(Align::Start))),
            3 => {
                // A line of inline images (an illuminated row).
                let mut p = Paragraph::new(Align::Center);
                for _ in 0..rng.below(9) + 3 {
                    p.push_image(ImageRef::new("glyph").with_size(32.0, 40.0));
                    p.push_text(" ", &RunStyle::default());
                }
                doc.push(Block::Paragraph(p));
            }
            _ => {
                let mut p = Paragraph::new(aligns[rng.below(4)]);
                for _ in 0..rng.below(70) + 5 {
                    let style = RunStyle {
                        size: [0.8, 1.0, 1.0, 1.0, 1.3, 1.7][rng.below(6)],
                        bold: rng.below(5) == 0,
                        ..RunStyle::default()
                    };
                    let w = match rng.below(40) {
                        0 => word(&mut rng, 60), // longer than a line
                        1 => {
                            p.push_image(ImageRef::new("glyph"));
                            continue;
                        }
                        2 => "\n".into(),
                        _ => {
                            let len = rng.below(9) + 1;
                            word(&mut rng, len)
                        }
                    };
                    p.push_text(&w, &style);
                    p.push_text(" ", &style);
                }
                doc.push(Block::Paragraph(p));
            }
        }
    }
    doc
}

fn reader_state(app: &App, reader: Entity) -> &ReaderState {
    app.world().get::<ReaderState>(reader).unwrap()
}

/// Updates until the reader shows its open view laid out.
fn settle(app: &mut App, reader: Entity) {
    for _ in 0..6 {
        app.update();
    }
    assert!(
        reader_state(app, reader).pagination().is_some(),
        "the reader paginated"
    );
}

/// Glyph count of `paragraph` laid out on its own (reference for lost glyphs).
fn reference_glyphs(app: &mut App, paragraph: &Paragraph, width: f32) -> usize {
    let world = app.world_mut();
    let mut state = bevy::ecs::system::SystemState::<(
        Commands,
        Res<RichTextTheme>,
        Res<RichImages>,
        Res<Assets<Image>>,
        Res<UiSettings>,
    )>::new(world);
    let (mut commands, theme, images, assets, settings) = state.get_mut(world).unwrap();
    let ctx = FragmentCtx {
        theme: &theme,
        scale: MeasureScale {
            scale_factor: 1.0,
            text_scale: settings.text_scale,
        },
        images: &images,
        image_assets: &assets,
    };
    let text = spawn_paragraph(&mut commands, paragraph, None, &ctx);
    commands.entity(text).insert(Node {
        position_type: PositionType::Absolute,
        width: Val::Px(width),
        ..default()
    });
    state.apply(world);
    app.update();
    let n = visible_glyphs(app.world().get::<TextLayoutInfo>(text).unwrap());
    app.world_mut().entity_mut(text).despawn();
    n
}

/// Glyphs with ink (spaces have none; drawn fragments drop trailing ones).
fn visible_glyphs(info: &TextLayoutInfo) -> usize {
    info.glyphs
        .iter()
        .filter(|g| g.atlas_info.rect.size().x > 0.0)
        .count()
}

/// The drawn page: for each fragment of each body, compare `bevy_ui`'s layout
/// with the pagination; returns glyphs drawn per block.
fn check_view(app: &mut App, reader: Entity, glyphs: &mut [usize]) {
    let world = app.world_mut();
    let state = world.get::<ReaderState>(reader).unwrap();
    let pagination = state.pagination().unwrap().clone();
    let pages: Vec<_> = pagination.view(state.view(), state.page_view()).to_vec();
    let mut bodies: Vec<(Entity, usize)> = world
        .query::<(Entity, &ReaderBody)>()
        .iter(world)
        .filter(|(_, b)| b.reader == reader)
        .map(|(e, b)| (e, b.index))
        .collect();
    bodies.sort_by_key(|b| b.1);
    let format = *pagination.format();
    for (page, (body, _)) in pages.iter().zip(&bodies) {
        let children: Vec<Entity> = world
            .get::<Children>(*body)
            .map(|c| c.to_vec())
            .unwrap_or_default();
        let body_node = world.get::<ComputedNode>(*body).unwrap();
        assert!((body_node.content_box().width() - format.width).abs() < 0.01);
        let lone = page.items.len() == 1;
        for child in children {
            let Some(fragment) = world.get::<ReaderFragment>(child) else {
                continue;
            };
            let placed = page
                .items
                .iter()
                .find(|p| p.block == fragment.block)
                .expect("a fragment per placed item");
            let node = world.get::<ComputedNode>(child).unwrap();
            let top = placed.y;
            match &placed.fragment {
                Fragment::Lines { lines, range } => {
                    let Some(block) = world.get::<ComputedTextBlock>(child) else {
                        assert!(range.is_empty(), "only blank lines draw without text");
                        assert!((node.size().y - placed.height).abs() <= 1.0);
                        continue;
                    };
                    // The fragment's own lines come first, exactly as paginated;
                    // after them only the invisible tail (the next line, for
                    // shaping context) or the empty line parley adds after a
                    // final hard break.
                    let heights: Vec<f32> = block.buffer().lines().map(|l| l.metrics().line_height).collect();
                    assert!(heights.len() >= *lines, "block {} {range:?}: lines drawn", placed.block);
                    let height: f32 = heights[..*lines].iter().sum();
                    assert!(
                        (height - placed.height).abs() <= 0.01,
                        "block {}: drawn height {height} vs paginated {}",
                        placed.block,
                        placed.height
                    );
                    assert!((node.size().y - placed.height).abs() <= 1.0);
                    let info = world.get::<TextLayoutInfo>(child).unwrap();
                    // No line is wider than the page (hanging spaces aside), and
                    // every glyph with ink sits on the page.
                    for (i, l) in block.buffer().lines().enumerate() {
                        let m = l.metrics();
                        let ink = m.advance - m.trailing_whitespace;
                        assert!(
                            m.offset >= -0.5 && m.offset + ink <= format.width + 0.5,
                            "block {} line {i}: {}..{} outside 0..{}",
                            placed.block,
                            m.offset,
                            m.offset + ink,
                            format.width
                        );
                    }
                    let mut shown = 0;
                    for g in info.glyphs.iter().filter(|g| g.atlas_info.rect.size().x > 0.0) {
                        let section = block.entities()[g.section_index as usize].entity;
                        let tail = world.get::<ReaderTail>(section).is_some();
                        assert_eq!(
                            tail,
                            g.line_index as usize >= *lines,
                            "block {} {range:?}: the tail starts exactly after the paginated lines",
                            placed.block
                        );
                        if !tail {
                            assert!(
                                (-1.0..=format.width + 1.0).contains(&g.position.x),
                                "block {}: glyph at x {} outside 0..{}",
                                placed.block,
                                g.position.x,
                                format.width
                            );
                            shown += 1;
                        }
                    }
                    if let Some(h) = format.height {
                        assert!(lone || top + height <= h + 0.01, "text clipped at the page bottom");
                    }
                    glyphs[placed.block] += shown;
                }
                Fragment::Image { size } => {
                    assert!((node.size().x - size.width).abs() <= 1.0);
                    assert!((node.size().y - size.height).abs() <= 1.0);
                    assert!(size.width <= format.width + 0.01);
                }
            }
        }
    }
}

fn page_through(app: &mut App, reader: Entity, doc: &RichDoc) {
    let views = reader_state(app, reader).view_count();
    assert!(views >= 2, "the synthetic book spans several views");
    let mut glyphs = vec![0; doc.blocks.len()];
    for view in 0..views {
        assert_eq!(reader_state(app, reader).view(), view);
        check_view(app, reader, &mut glyphs);
        app.world_mut().write_message(ReaderEvent {
            reader,
            action: ReaderAction::Next,
        });
        app.update();
        app.update();
    }
    // Every paragraph is drawn whole: the fragments' ranges tile it and draw
    // as many glyphs as the paragraph laid out alone.
    let state = reader_state(app, reader);
    let pagination = state.pagination().unwrap().clone();
    let width = pagination.format().width / state.scale().unwrap().scale_factor;
    for (index, block) in doc.blocks.iter().enumerate() {
        let Block::Paragraph(p) = block else { continue };
        let mut at = 0;
        for placed in pagination.pages().iter().flat_map(|p| &p.items) {
            if let (true, Fragment::Lines { range, .. }) = (placed.block == index, &placed.fragment) {
                assert_eq!(range.start, at, "block {index}: fragments are contiguous");
                at = range.end;
            }
        }
        assert_eq!(at, p.len(), "block {index}: fragments cover the paragraph");
        if !p.is_empty() {
            let expect = reference_glyphs(app, p, width);
            assert_eq!(glyphs[index], expect, "block {index}: glyphs drawn across pages");
        }
    }
}

fn open_book(app: &mut App, doc: &Arc<RichDoc>, view: PageView) -> Entity {
    add_image(app, "plate", 300, 220);
    add_image(app, "glyph", 24, 30);
    add_image(app, "parchment", 64, 64);
    let reader = app
        .world_mut()
        .spawn(RichReader::book(doc.clone()).with_view(view))
        .id();
    settle(app, reader);
    reader
}

#[test]
fn pagination_matches_bevy_layout() {
    // 12 books: UI scale 100-200 %, text scale 100-200 %.
    let scales = [(1.0, 1.0), (1.5, 1.0), (1.25, 1.4), (2.0, 2.0)];
    for seed in 1..=12u64 {
        let (scale_factor, text_scale) = scales[usize::try_from(seed).unwrap() % scales.len()];
        let mut app = ui_app(scale_factor);
        app.world_mut().resource_mut::<UiSettings>().text_scale = text_scale;
        let doc = Arc::new(synthetic_doc(seed));
        let reader = open_book(&mut app, &doc, PageView::Spread);
        page_through(&mut app, reader, &doc);
    }
}

#[test]
fn pagination_matches_with_a_proportional_font() {
    // 6 single-page books, UI scale 100-200 %, text scale 100-130 %.
    for seed in 1..=6u64 {
        let i = usize::try_from(seed).unwrap();
        let mut app = ui_app([1.0, 1.5, 2.0][i % 3]);
        if !use_system_font(&mut app) {
            eprintln!("skipped: no {DEJAVU}");
            return;
        }
        app.world_mut().resource_mut::<UiSettings>().text_scale = [1.0, 1.3][i % 2];
        let doc = Arc::new(synthetic_doc(seed + 100));
        let reader = open_book(&mut app, &doc, PageView::Single);
        page_through(&mut app, reader, &doc);
    }
}

#[test]
fn measure_cache_reuses_paragraphs() {
    let mut app = ui_app(1.0);
    let doc = Arc::new(synthetic_doc(3));
    let reader = open_book(&mut app, &doc, PageView::Spread);
    let misses = app.world().resource::<MeasureCache>().misses;
    assert!(misses > 0);
    // Spread → single at the same page size: no new layouts.
    app.world_mut().get_mut::<RichReader>(reader).unwrap().view = Some(PageView::Single);
    app.update();
    app.update();
    let cache = app.world().resource::<MeasureCache>();
    assert_eq!(cache.misses, misses);
    assert!(cache.hits > 0);
    assert_eq!(reader_state(&app, reader).page_view(), PageView::Single);
    // A theme change clears it.
    app.world_mut().resource_mut::<RichTextTheme>().font_size += 2.0;
    app.update();
    app.update();
    assert!(app.world().resource::<MeasureCache>().misses > misses);
}

#[test]
fn book_turns_pages_and_keeps_its_place_across_text_scale() {
    let mut app = ui_app(1.0);
    let doc = Arc::new(synthetic_doc(17));
    let reader = open_book(&mut app, &doc, PageView::Spread);
    let world = app.world_mut();
    let buttons: Vec<(Entity, ReaderAction)> = world
        .query::<(Entity, &ReaderButton)>()
        .iter(world)
        .map(|(e, b)| (e, b.action))
        .collect();
    let button = |a: ReaderAction| buttons.iter().find(|b| b.1 == a).unwrap().0;
    // The document background is drawn behind the pages.
    let backgrounds = world
        .query::<(&ImageNode, &Name)>()
        .iter(world)
        .filter(|(i, n)| n.as_str() == "sf-reader-pages" && i.image != Handle::default())
        .count();
    assert_eq!(backgrounds, 1);
    assert!(app.world().get::<InteractionDisabled>(button(ReaderAction::Prev)).is_some());
    // Next by button, twice; Prev by message.
    for _ in 0..2 {
        app.world_mut().trigger(Activate {
            entity: button(ReaderAction::Next),
        });
        app.update();
    }
    assert_eq!(reader_state(&app, reader).view(), 2);
    app.world_mut().write_message(ReaderEvent {
        reader,
        action: ReaderAction::Prev,
    });
    app.update();
    assert_eq!(reader_state(&app, reader).view(), 1);
    app.update();
    assert!(app.world().get::<InteractionDisabled>(button(ReaderAction::Prev)).is_none());
    let bookmark = app.world().get::<RichReader>(reader).unwrap().bookmark;
    assert!(bookmark > DocPos::default());
    // 150 % text: more pages, the open view still holds the bookmark.
    let before = reader_state(&app, reader).view_count();
    app.world_mut().resource_mut::<UiSettings>().text_scale = 1.5;
    app.update();
    app.update();
    let state = reader_state(&app, reader);
    assert!(state.view_count() > before);
    let pagination = state.pagination().unwrap();
    assert_eq!(state.view(), pagination.view_of(bookmark, PageView::Spread));
    let pages = pagination.view(state.view(), PageView::Spread);
    assert!(pages[0].start <= bookmark);
    // Last view: Next is disabled and Next does nothing.
    let last = state.view_count() - 1;
    for _ in 0..last + 2 {
        app.world_mut().write_message(ReaderEvent {
            reader,
            action: ReaderAction::Next,
        });
        app.update();
    }
    app.update();
    assert_eq!(reader_state(&app, reader).view(), last);
    assert!(app.world().get::<InteractionDisabled>(button(ReaderAction::Next)).is_some());
}

#[derive(Resource, Default)]
struct SeenReader(Vec<ReaderEvent>);

#[test]
fn ui_actions_drive_the_reader_of_their_screen() {
    let mut app = ui_app(1.0);
    app.init_resource::<SeenReader>().add_systems(
        Update,
        (|mut r: MessageReader<ReaderEvent>, mut seen: ResMut<SeenReader>| {
            seen.0.extend(r.read().copied());
        })
        .after(UiTextSystems::Input),
    );
    add_image(&mut app, "plate", 300, 220);
    add_image(&mut app, "glyph", 24, 30);
    add_image(&mut app, "parchment", 64, 64);
    let doc = Arc::new(synthetic_doc(29));
    let root = app.world_mut().spawn((Node::default(), UiScreenRoot("book".into()))).id();
    let reader = app
        .world_mut()
        .spawn((RichReader::book(doc).takeable(true), ChildOf(root)))
        .id();
    let other = app.world_mut().spawn(RichReader::book(Arc::new(synthetic_doc(30)))).id();
    settle(&mut app, reader);
    for action in [UiAction::TabNext, UiAction::Context1, UiAction::Back, UiAction::Confirm] {
        app.world_mut().write_message(UiActionEvent {
            action,
            screen: "book".into(),
        });
    }
    app.update();
    let seen = &app.world().resource::<SeenReader>().0;
    let expect = [ReaderAction::Next, ReaderAction::Take, ReaderAction::Close];
    assert_eq!(seen.iter().map(|e| e.action).collect::<Vec<_>>(), expect);
    assert!(seen.iter().all(|e| e.reader == reader), "only the screen's reader");
    assert_eq!(reader_state(&app, reader).view(), 1);
    assert_eq!(reader_state(&app, other).view(), 0);
}

#[test]
fn scroll_is_one_page_and_scrolls() {
    let mut app = ui_app(1.0);
    add_image(&mut app, "plate", 300, 220);
    add_image(&mut app, "glyph", 24, 30);
    add_image(&mut app, "parchment", 64, 64);
    let doc = Arc::new(synthetic_doc(41));
    let reader = app.world_mut().spawn(RichReader::scroll(doc.clone())).id();
    settle(&mut app, reader);
    let state = reader_state(&app, reader);
    let pagination = state.pagination().unwrap();
    assert_eq!(pagination.pages().len(), 1);
    assert_eq!(state.view_count(), 1);
    let content = pagination.pages()[0].content_height();
    let world = app.world_mut();
    let (body, body_node) = world
        .query::<(Entity, &ComputedNode, &ReaderBody)>()
        .iter(world)
        .find(|(.., b)| b.reader == reader)
        .map(|(e, n, _)| (e, n.size()))
        .unwrap();
    assert!((body_node.y - content).abs() <= 1.0, "the scroll body is as long as the text");
    let drawn = world.get::<Children>(body).unwrap().len();
    let items = pagination_items(&app, reader);
    assert_eq!(drawn, items);
    let viewport = |app: &mut App| {
        let world = app.world_mut();
        world
            .query::<(&ScrollPosition, &Name)>()
            .iter(world)
            .find(|(_, n)| n.as_str() == "sf-reader-scroll")
            .unwrap()
            .0
            .y
    };
    assert!(viewport(&mut app).abs() < 1e-3);
    app.world_mut().write_message(ReaderEvent {
        reader,
        action: ReaderAction::Next,
    });
    app.update();
    app.update();
    let y = viewport(&mut app);
    assert!(y > 400.0, "next scrolls most of a viewport (got {y})");
    let bookmark = app.world().get::<RichReader>(reader).unwrap().bookmark;
    assert!(bookmark > DocPos::default(), "the bookmark follows the scroll");
    // Wheel: three lines down.
    app.world_mut().write_message(bevy::input::mouse::MouseWheel {
        unit: bevy::input::mouse::MouseScrollUnit::Line,
        x: 0.0,
        y: -3.0,
        window: Entity::PLACEHOLDER,
        phase: bevy::input::touch::TouchPhase::Moved,
    });
    app.update();
    app.update();
    assert!((viewport(&mut app) - y - 3.0 * WHEEL_LINE).abs() < 1.0);
}

fn pagination_items(app: &App, reader: Entity) -> usize {
    reader_state(app, reader).pagination().unwrap().pages()[0].items.len()
}

const MODAL: &str = r#"
[[define]]
kind = "ui.screen"
id = "sf.message_box"
value = { layer = "Modal", input = "modal", implementation = { Template = "message_box" } }
"#;

#[derive(Resource, Default)]
struct Answers(Vec<MessageBoxAnswer>);

fn message_app() -> App {
    let mut db = RecordDb::new();
    register_kinds(&mut db).unwrap();
    let base = db.add_source("base", &[]);
    let outcome = apply_record_file(&mut db, base, MODAL, RecordFormat::Toml).unwrap();
    assert_eq!(outcome.errors, []);
    ui_app_with(1.0, |app| {
        app.add_plugins((bevy_enhanced_input::prelude::EnhancedInputPlugin, SfUiPlugin))
            .insert_resource(UiRegistryRes(UiRegistry::build(&db).0))
            .init_resource::<Answers>()
            .add_systems(
                Update,
                (|mut r: MessageReader<MessageBoxAnswer>, mut seen: ResMut<Answers>| {
                    seen.0.extend(r.read().copied());
                })
                .after(UiTextSystems::Draw),
            );
    })
}

fn box_buttons(app: &mut App) -> Vec<(Entity, MessageBoxButton)> {
    let world = app.world_mut();
    let mut buttons: Vec<_> = world
        .query::<(Entity, &MessageBoxButton)>()
        .iter(world)
        .map(|(e, b)| (e, *b))
        .collect();
    buttons.sort_by_key(|b| b.1.index);
    buttons
}

#[test]
fn message_box_choice_flow() {
    let mut app = message_app();
    let config = app.world().resource::<MessageBoxConfig>().clone();
    let (a, b) = {
        let mut boxes = app.world_mut().resource_mut::<MessageBoxes>();
        let a = boxes.show("Rest until healed?", vec!["Yes".into(), "No".into()], &config);
        let b = boxes.show("You cannot rest here.", Vec::new(), &config);
        (a, b)
    };
    app.update();
    app.update();
    // The first box is up as the modal input target, focus on its first button.
    let stack = &app.world().resource::<ScreenStackRes>().0;
    assert_eq!(stack.input_target().unwrap().id.as_str(), "sf.message_box");
    let buttons = box_buttons(&mut app);
    assert_eq!(buttons.len(), 2);
    assert!(buttons.iter().all(|(_, btn)| btn.id == a));
    assert_eq!(
        app.world().resource::<InputFocus>().get(),
        Some(buttons[0].0)
    );
    let label = |app: &mut App| {
        let world = app.world_mut();
        world
            .query::<(&Text, &Name)>()
            .iter(world)
            .find(|(_, n)| n.as_str() == "sf-message-box-text")
            .map(|(t, _)| t.0.clone())
    };
    assert_eq!(label(&mut app).as_deref(), Some("Rest until healed?"));
    // Answer "No" with the button: the second box replaces it.
    app.world_mut().trigger(Activate {
        entity: buttons[1].0,
    });
    app.update();
    app.update();
    assert_eq!(
        app.world().resource::<Answers>().0,
        [MessageBoxAnswer(sf_ui::MessageBoxChoice { id: a, index: 1 })]
    );
    assert_eq!(label(&mut app).as_deref(), Some("You cannot rest here."));
    let buttons = box_buttons(&mut app);
    assert_eq!(buttons.len(), 1, "OK fallback");
    assert_eq!(buttons[0].1.id, b);
    // Back without a cancel button does nothing (and doesn't close the screen).
    app.world_mut().write_message(UiActionEvent {
        action: UiAction::Back,
        screen: config.screen.clone(),
    });
    app.update();
    assert_eq!(app.world().resource::<Answers>().0.len(), 1);
    assert!(app.world().resource::<ScreenStackRes>().0.is_open("sf.message_box"));
    // With one, Back answers it; the queue is empty: screen closed, view gone.
    app.world_mut().resource_mut::<MessageBoxes>().0.set_cancel(b, Some(0));
    app.world_mut().write_message(UiActionEvent {
        action: UiAction::Back,
        screen: config.screen.clone(),
    });
    app.update();
    app.update();
    assert_eq!(
        app.world().resource::<Answers>().0[1],
        MessageBoxAnswer(sf_ui::MessageBoxChoice { id: b, index: 0 })
    );
    assert!(!app.world().resource::<ScreenStackRes>().0.is_open("sf.message_box"));
    let world = app.world_mut();
    assert_eq!(world.query::<&MessageBoxView>().iter(world).count(), 0);
}

fn toast_views(app: &mut App) -> Vec<(String, f32)> {
    let world = app.world_mut();
    let mut out = Vec::new();
    let views: Vec<(Entity, f32)> = world
        .query::<(Entity, &ToastView, &BackgroundColor)>()
        .iter(world)
        .map(|(e, _, bg)| (e, bg.0.alpha()))
        .collect();
    for (entity, alpha) in views {
        let child = world.get::<Children>(entity).unwrap()[0];
        out.push((world.get::<Text>(child).unwrap().0.clone(), alpha));
    }
    out
}

#[test]
fn toast_queue_timing() {
    let mut app = ui_app(1.0);
    let config = sf_ui::ToastConfig {
        base: 1.0,
        per_char: 0.0,
        min: 1.0,
        max: 1.0,
        fade_in: 0.2,
        fade_out: 0.4,
        max_visible: 2,
        ..default()
    };
    let full = NotifyColors::default().toast_background.alpha();
    app.insert_resource(Toasts::new(config));
    {
        let mut toasts = app.world_mut().resource_mut::<Toasts>();
        toasts.push("You picked up a key.");
        toasts.push("You picked up 5 gold.");
        toasts.push("You picked up 5 gold.");
        toasts.push("Journal updated.");
    }
    // Frames are 0.1 s of real time.
    app.update();
    let views = toast_views(&mut app);
    assert_eq!(views.len(), 2, "two slots");
    let gold = views.iter().find(|(t, _)| t == "You picked up 5 gold. (x2)").unwrap();
    assert!(gold.1 < full, "fading in");
    // The key toast was retired for the journal toast, which shows once
    // its 0.4 s fade is done.
    for _ in 0..5 {
        app.update();
    }
    let views = toast_views(&mut app);
    let texts: Vec<_> = views.iter().map(|(t, _)| t.as_str()).collect();
    assert_eq!(texts.len(), 2);
    assert!(texts.contains(&"Journal updated."), "{texts:?}");
    assert!(!texts.contains(&"You picked up a key."));
    // Everything is gone after its time plus fades.
    for _ in 0..30 {
        app.update();
    }
    assert!(toast_views(&mut app).is_empty());
    assert!(app.world().resource::<Toasts>().queue().visible().is_empty());
    // Reduce motion: full opacity from the first frame.
    app.world_mut().resource_mut::<UiSettings>().reduce_motion = true;
    app.world_mut().resource_mut::<Toasts>().push("Saved.");
    app.update();
    let views = toast_views(&mut app);
    assert_eq!(views.len(), 1);
    assert!((views[0].1 - full).abs() < 1e-4);
}
