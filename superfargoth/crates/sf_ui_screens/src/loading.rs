//! The loading screen (menus.md §3 "Intro videos and loading screen") over a view-model the app
//! fills: [`LoadingModel`], a splash image, the progress (`None`: unknown, drawn as an animated
//! sweep) and an already-localized hint line (empty: hidden). The app spawns a
//! [`LoadingScreen`] when its load starts, writes the progress each frame and despawns the
//! screen when the world is ready; the screen sends nothing and takes no input.
//!
//! A full-window opaque root (the world builds behind it), the image fitted `cover`
//! ([`sf_ui::cover_size`]: aspect kept, centred, the overflow clipped by a layer), hidden until
//! its size is known; the hint and the bar near the bottom.
//!
//! **Layer** `Modal`, outside the screen stack: the root is not a `UiScreenRoot` and is never
//! pushed, so it is never the input target, owns no focus and Back never closes it (a
//! non-interactive overlay, as a HUD element is). It carries [`loading_z`], the bottom of the
//! Modal band: over the HUD and every menu, under any modal pushed meanwhile (a message box).
//!
//! Parts: `sf-loading`, `-backdrop`, `-image`, `-footer`, `-hint`, `-bar`, `-bar-fill` (with
//! `indeterminate` as a class while the progress is unknown).

use bevy_asset::{Assets, Handle};
use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_text::TextColor;
use bevy_time::Time;
use bevy_ui::ComputedNode;
use bevy_ui::prelude::*;
use sf_ui::Layer;
use sf_ui_bevy::screen_z;

use crate::widgets::{ScreenColors, classes, label, part, part_as, role, show};

/// Seconds for one sweep of the indeterminate bar.
const SWEEP_SECONDS: f32 = 1.5;
/// The indeterminate segment's width, in percent of the bar.
const SWEEP_WIDTH: f32 = 25.0;

/// What the loading screen shows.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LoadingModel {
    /// The splash, full window (`cover`); none: the opaque root alone.
    pub image: Option<Handle<Image>>,
    /// Done fraction, clamped to 0..=1; none: unknown (an animated sweep).
    pub progress: Option<f32>,
    /// One line (already localized; empty: hidden), above the bar.
    pub hint: String,
}

/// On a root entity: the loading screen. The app may replace `model` at any time.
#[derive(Component, Clone, Debug, Default)]
#[require(Node)]
pub struct LoadingScreen {
    pub model: LoadingModel,
}

impl LoadingScreen {
    pub fn new(model: LoadingModel) -> Self {
        Self { model }
    }
}

/// The global z-index of a loading screen's root: just under the first modal.
pub fn loading_z() -> i32 {
    screen_z(Layer::Modal, 0) - 1
}

/// A loading screen's parts, built on spawn.
#[derive(Component, Debug)]
pub struct LoadingParts {
    backdrop: Entity,
    image: Entity,
    hint: Entity,
    bar: Entity,
    fill: Entity,
    seen: Option<LoadingModel>,
}

impl LoadingParts {
    /// The clipping layer the image is fitted in.
    pub fn backdrop(&self) -> Entity {
        self.backdrop
    }

    pub fn image(&self) -> Entity {
        self.image
    }

    pub fn hint(&self) -> Entity {
        self.hint
    }

    pub fn bar(&self) -> Entity {
        self.bar
    }

    pub fn fill(&self) -> Entity {
        self.fill
    }
}

/// A full-size absolute node.
fn full() -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(0.0),
        top: Val::Px(0.0),
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..Node::default()
    }
}

pub(crate) fn spawn_loading(
    mut commands: Commands,
    colors: Res<ScreenColors>,
    roots: Query<Entity, Added<LoadingScreen>>,
) {
    for root in &roots {
        commands.entity(root).insert((
            part("sf-loading"),
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                padding: UiRect::bottom(Val::Percent(6.0)),
                row_gap: Val::Px(8.0),
                ..full()
            },
            // Opaque: the world builds behind it.
            BackgroundColor(Color::BLACK),
            GlobalZIndex(loading_z()),
        ));
        // First child: drawn behind the footer.
        let backdrop = commands
            .spawn((
                part("sf-loading-backdrop"),
                Node {
                    overflow: Overflow::clip(),
                    ..full()
                },
                ChildOf(root),
            ))
            .id();
        let image = commands
            .spawn((
                part("sf-loading-image"),
                Node {
                    position_type: PositionType::Absolute,
                    // Until the image's size is known (no placeholder flash).
                    display: Display::None,
                    ..Node::default()
                },
                ChildOf(backdrop),
            ))
            .id();
        let footer = commands
            .spawn((
                part_as("sf-loading-footer", role::WINDOW),
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.0),
                    width: Val::Percent(60.0),
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id();
        let hint = commands
            .spawn((
                label("sf-loading-hint", "", &colors),
                Node {
                    display: Display::None,
                    ..Node::default()
                },
                ChildOf(footer),
            ))
            .id();
        let bar = commands
            .spawn((
                part_as("sf-loading-bar", role::BOX),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(10.0),
                    overflow: Overflow::clip(),
                    ..Node::default()
                },
                BackgroundColor(colors.bar_track),
                ChildOf(footer),
            ))
            .id();
        let fill = commands
            .spawn((
                part("sf-loading-bar-fill"),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    height: Val::Percent(100.0),
                    ..Node::default()
                },
                BackgroundColor(colors.bar_fill),
                ChildOf(bar),
            ))
            .id();
        commands.entity(root).insert(LoadingParts {
            backdrop,
            image,
            hint,
            bar,
            fill,
            seen: None,
        });
    }
}

/// The fill's (left, width) in percent: the done fraction, or the sweep at `seconds`.
fn fill_span(progress: Option<f32>, seconds: f32) -> (f32, f32) {
    if let Some(p) = progress {
        (0.0, if p.is_finite() { p.clamp(0.0, 1.0) * 100.0 } else { 0.0 })
    } else {
        let phase = (seconds / SWEEP_SECONDS).rem_euclid(1.0);
        (-SWEEP_WIDTH + phase * (100.0 + SWEEP_WIDTH), SWEEP_WIDTH)
    }
}

/// Model → image, hint, bar class (on change); the fill every frame (the sweep moves); the
/// image's `cover` fit once its size is known.
#[allow(clippy::needless_pass_by_value)] // system parameters
pub(crate) fn sync_loading(
    mut commands: Commands,
    colors: Res<ScreenColors>,
    time: Option<Res<Time>>,
    images: Option<Res<Assets<Image>>>,
    mut screens: Query<(&LoadingScreen, &mut LoadingParts)>,
    computed: Query<&ComputedNode>,
    mut nodes: Query<&mut Node>,
) {
    let seconds = time.map_or(0.0, |t| t.elapsed_secs());
    for (screen, mut parts) in &mut screens {
        let model = &screen.model;
        if parts.seen.as_ref() != Some(model) {
            let was = parts.seen.take();
            let first = was.is_none();
            let was = was.unwrap_or_default();
            if first || was.image != model.image {
                match &model.image {
                    Some(image) => commands.entity(parts.image).insert(ImageNode::new(image.clone())),
                    None => commands.entity(parts.image).remove::<ImageNode>(),
                };
            }
            if first || was.hint != model.hint {
                commands
                    .entity(parts.hint)
                    .insert((Text::new(model.hint.clone()), TextColor(colors.text)));
                if let Ok(mut node) = nodes.get_mut(parts.hint) {
                    show(&mut node, !model.hint.is_empty());
                }
            }
            if first || was.progress.is_none() != model.progress.is_none() {
                commands.entity(parts.fill).insert(classes(
                    "sf-loading-bar-fill",
                    &[("indeterminate", model.progress.is_none())],
                ));
            }
            parts.seen = Some(model.clone());
        }

        let (left, width) = fill_span(model.progress, seconds);
        if let Ok(mut node) = nodes.get_mut(parts.fill) {
            let span = (Val::Percent(left), Val::Percent(width));
            if (node.left, node.width) != span {
                (node.left, node.width) = span;
            }
        }

        // The `cover` fit, in percent of the clipping layer, centred.
        let size = model
            .image
            .as_ref()
            .and_then(|h| images.as_deref()?.get(h))
            .map(Image::size_f32);
        let container = computed
            .get(parts.backdrop)
            .map(ComputedNode::size)
            .unwrap_or_default();
        let shown = match size {
            Some(size)
                if container.x > 0.0 && container.y > 0.0 && size.x > 0.0 && size.y > 0.0 =>
            {
                let (w, h) = sf_ui::cover_size((container.x, container.y), (size.x, size.y));
                let (w, h) = (w / container.x * 100.0, h / container.y * 100.0);
                if let Ok(mut node) = nodes.get_mut(parts.image) {
                    let fit = (
                        Val::Percent(w),
                        Val::Percent(h),
                        Val::Percent((100.0 - w) / 2.0),
                        Val::Percent((100.0 - h) / 2.0),
                    );
                    if (node.width, node.height, node.left, node.top) != fit {
                        (node.width, node.height, node.left, node.top) = fit;
                    }
                }
                true
            }
            _ => false,
        };
        if let Ok(mut node) = nodes.get_mut(parts.image) {
            show(&mut node, shown);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::input::InputPlugin;
    use bevy::picking::{InteractionPlugin, PickingPlugin};
    use bevy::prelude::*;
    use bevy::text::TextPlugin;
    use bevy::time::TimeUpdateStrategy;
    use bevy::ui::UiPlugin;
    use bevy_enhanced_input::prelude::EnhancedInputPlugin;
    use sf_ui_bevy::{ScreenStackRes, SfUiPlugin, UiPart, UiRegistryRes, UiTextPlugin};

    use super::*;
    use crate::{SfScreensPlugin, demo};

    fn app() -> App {
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
            EnhancedInputPlugin,
            SfUiPlugin,
            UiTextPlugin,
        ))
        .add_plugins(SfScreensPlugin)
        .insert_resource(UiRegistryRes(demo::registry()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(100)));
        app.finish();
        app.update();
        app
    }

    fn parts(app: &App, root: Entity) -> &LoadingParts {
        app.world().get::<LoadingParts>(root).expect("loading parts")
    }

    fn fill_span_of(app: &App, root: Entity) -> (Val, Val) {
        let node = app.world().get::<Node>(parts(app, root).fill()).unwrap();
        (node.left, node.width)
    }

    /// Spawns every named part on the Modal band, outside the screen stack.
    #[test]
    fn loading_screen_spawns_its_parts() {
        let mut app = app();
        let root = app.world_mut().spawn(LoadingScreen::default()).id();
        app.update();
        let names: Vec<&str> = app
            .world_mut()
            .query::<&UiPart>()
            .iter(app.world())
            .map(|p| p.0)
            .filter(|n| n.starts_with("sf-loading"))
            .collect();
        for name in [
            "sf-loading",
            "sf-loading-backdrop",
            "sf-loading-image",
            "sf-loading-footer",
            "sf-loading-hint",
            "sf-loading-bar",
            "sf-loading-bar-fill",
        ] {
            assert!(names.contains(&name), "{name} missing: {names:?}");
        }
        assert_eq!(app.world().get::<GlobalZIndex>(root).map(|z| z.0), Some(loading_z()));
        assert!(app.world().resource::<ScreenStackRes>().0.input_target().is_none());
        // Empty hint: hidden; no image: hidden.
        let hint = app.world().get::<Node>(parts(&app, root).hint()).unwrap();
        assert_eq!(hint.display, Display::None);
        let image = parts(&app, root).image();
        assert_eq!(app.world().get::<Node>(image).map(|n| n.display), Some(Display::None));
    }

    /// Unknown progress sweeps (the fill moves frame to frame, stays finite); known progress
    /// fills from the left, clamped; a hint shows.
    #[test]
    fn loading_progress_none_and_some_draw() {
        let mut app = app();
        let root = app.world_mut().spawn(LoadingScreen::default()).id();
        app.update();
        let a = fill_span_of(&app, root);
        app.update();
        let b = fill_span_of(&app, root);
        assert_ne!(a, b, "the sweep moves");
        for (left, width) in [a, b] {
            let (Val::Percent(l), Val::Percent(w)) = (left, width) else { panic!("percent span") };
            assert!(l.is_finite() && w.is_finite());
        }

        for (progress, want) in [(0.4, 40.0), (2.0, 100.0), (f32::NAN, 0.0)] {
            app.world_mut().get_mut::<LoadingScreen>(root).unwrap().model.progress = Some(progress);
            app.world_mut().get_mut::<LoadingScreen>(root).unwrap().model.hint = "Hint".into();
            app.update();
            let (left, width) = fill_span_of(&app, root);
            assert_eq!(left, Val::Percent(0.0));
            let Val::Percent(w) = width else { panic!("percent width") };
            assert!((w - want).abs() < 1e-3, "{progress} → {w}");
        }
        let hint = app.world().get::<Node>(parts(&app, root).hint()).unwrap();
        assert_ne!(hint.display, Display::None);
    }

    #[test]
    fn loading_sweep_span_wraps() {
        assert_eq!(fill_span(None, 0.0), (-SWEEP_WIDTH, SWEEP_WIDTH));
        let (l, _) = fill_span(None, SWEEP_SECONDS * 0.5);
        assert!((l - (-SWEEP_WIDTH + f32::midpoint(100.0, SWEEP_WIDTH))).abs() < 1e-3);
        assert_eq!(fill_span(None, SWEEP_SECONDS), fill_span(None, 0.0));
    }
}
