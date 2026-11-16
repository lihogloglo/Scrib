//! Headless: parsing, defaults, the uniform's bytes, the target size, and the console command
//! driving the view (no GPU: the render world is absent, the main-world side is checked).

#![allow(clippy::float_cmp)] // exact: the uniform bytes are the contract

use bevy_app::App;
use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_image::Image;
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_ui::widget::ImageNode;
use sf_console_bevy::sf_console::Level;
use sf_console_bevy::{ConsolePlugin, ConsoleSubmit, DevConsole};
use sf_diag_bevy::texview::{DebugTexture, DebugTextureAppExt, DebugTextures};

use crate::{
    Channel, SampleKind, ShowArgs, Shown, TARGET_MAX, TexCommand, TexView, TexViewFrame, TexViewPlugin, params, parse,
    resolve, target_size,
};

fn show(name: &str, channel: Option<Channel>, range: Option<(f32, f32)>) -> TexCommand {
    TexCommand::Show(ShowArgs { name: name.to_owned(), channel, range })
}

#[test]
fn parse_actions_channels_and_ranges() {
    assert_eq!(parse(None, None), Ok(TexCommand::List));
    assert_eq!(parse(Some("list"), Some("")), Ok(TexCommand::List));
    assert_eq!(parse(Some("hide"), None), Ok(TexCommand::Hide));
    assert_eq!(parse(Some("show"), Some("trail")), Ok(show("trail", None, None)));
    assert_eq!(parse(Some("show"), Some("trail G")), Ok(show("trail", Some(Channel::G), None)));
    assert_eq!(parse(Some("show"), Some("surface.shelter r 1 0")), Ok(show("surface.shelter", Some(Channel::R), Some((1.0, 0.0)))));
    assert_eq!(parse(Some("show"), Some("trail -0.5 0.25")), Ok(show("trail", None, Some((-0.5, 0.25)))));
    assert_eq!(parse(Some("show"), Some("x rgb 0 4e3")), Ok(show("x", Some(Channel::Rgb), Some((0.0, 4000.0)))));
}

#[test]
fn parse_errors() {
    assert_eq!(parse(Some("list"), Some("x")).unwrap_err(), "`tex list` takes no arguments");
    assert_eq!(parse(Some("hide"), Some("x")).unwrap_err(), "`tex hide` takes no arguments");
    assert_eq!(parse(Some("show"), None).unwrap_err(), "missing name: tex show <name> [r|g|b|a|rgb] [min max]");
    assert_eq!(parse(Some("show"), Some("trail r 1")).unwrap_err(), "expected [r|g|b|a|rgb] [min max] after the name, got \"1\": tex show <name> [r|g|b|a|rgb] [min max]");
    assert_eq!(parse(Some("show"), Some("trail q 0 1")).unwrap_err(), "expected [r|g|b|a|rgb] [min max] after the name, got \"q 0 1\": tex show <name> [r|g|b|a|rgb] [min max]");
    assert_eq!(parse(Some("show"), Some("trail x 1")).unwrap_err(), "bad number \"x\" (channel: r, g, b, a or rgb)");
    assert_eq!(parse(Some("show"), Some("trail 0 inf")).unwrap_err(), "bad number \"inf\" (channel: r, g, b, a or rgb)");
    assert_eq!(parse(Some("show"), Some("trail 2 2")).unwrap_err(), "empty range 2..2: min and max must differ");
    assert_eq!(parse(Some("open"), None).unwrap_err(), "unknown action \"open\": list, show or hide");
}

#[test]
fn channel_and_sample_kind_per_format() {
    assert_eq!(Channel::default_for(TextureFormat::Depth32Float), Channel::R);
    assert_eq!(Channel::default_for(TextureFormat::R16Float), Channel::R);
    assert_eq!(Channel::default_for(TextureFormat::Rgba16Float), Channel::Rgb);
    assert_eq!(SampleKind::of(TextureFormat::Depth32Float), Ok(SampleKind::Float));
    assert_eq!(SampleKind::of(TextureFormat::Rgba16Float), Ok(SampleKind::Float));
    assert_eq!(SampleKind::of(TextureFormat::Rgba32Float), Ok(SampleKind::Float));
    assert_eq!(SampleKind::of(TextureFormat::R32Uint), Ok(SampleKind::Uint));
    assert_eq!(SampleKind::of(TextureFormat::Rg16Sint), Ok(SampleKind::Sint));
    assert_eq!(
        SampleKind::of(TextureFormat::Depth24PlusStencil8).unwrap_err(),
        "Depth24PlusStencil8 is depth+stencil: register a depth-only view or copy"
    );
    assert_eq!(SampleKind::Float.shader_def(), None);
    assert_eq!(SampleKind::Uint.shader_def(), Some("SAMPLE_UINT"));
    assert_eq!(SampleKind::Sint.shader_def(), Some("SAMPLE_SINT"));
}

#[test]
fn params_pack_range_then_channel() {
    let bytes = params(Channel::A, -1.0, 3.0);
    let f = |i: usize| f32::from_le_bytes(bytes[4 * i..4 * i + 4].try_into().unwrap());
    let u = |i: usize| u32::from_le_bytes(bytes[4 * i..4 * i + 4].try_into().unwrap());
    assert_eq!([f(0), f(1), f(2), f(3)], [-1.0, 0.25, 0.0, 0.0]);
    assert_eq!([u(4), u(5), u(6), u(7)], [3, 0, 0, 0]);
    // Inverted range: negative scale.
    let inv = params(Channel::Rgb, 1.0, 0.0);
    assert_eq!(f32::from_le_bytes(inv[4..8].try_into().unwrap()), -1.0);
    assert_eq!(u32::from_le_bytes(inv[16..20].try_into().unwrap()), 4);
}

#[test]
fn target_keeps_small_sources_and_caps_the_long_side() {
    assert_eq!(target_size((1024, 1024)), (1024, 1024));
    assert_eq!(target_size((512, 256)), (512, 256));
    assert_eq!(target_size((4096, 2048)), (TARGET_MAX, 512));
    assert_eq!(target_size((1, 3000)), (1, TARGET_MAX));
    assert_eq!(target_size((0, 0)), (1, 1));
}

fn registry() -> DebugTextures {
    let mut reg = DebugTextures::default();
    reg.register(DebugTexture::render("surface.trail", "ring", (1024, 1024), TextureFormat::Rgba16Float, |_| None));
    reg.register(DebugTexture::render("surface.shelter", "depth", (1024, 1024), TextureFormat::Depth32Float, |_| None));
    reg.register(DebugTexture::render("x.ds", "ds", (8, 8), TextureFormat::Depth24PlusStencil8, |_| None));
    reg
}

#[test]
fn resolve_fills_defaults_and_refuses_bad_entries() {
    let reg = registry();
    let args = |name: &str, channel, range| ShowArgs { name: name.to_owned(), channel, range };
    assert_eq!(
        resolve(&reg, &args("shelter", None, None)),
        Ok(Shown { name: "surface.shelter".into(), channel: Channel::R, min: 0.0, max: 1.0 })
    );
    assert_eq!(
        resolve(&reg, &args("surface.trail", Some(Channel::G), Some((-0.3, 0.1)))),
        Ok(Shown { name: "surface.trail".into(), channel: Channel::G, min: -0.3, max: 0.1 })
    );
    assert_eq!(resolve(&reg, &args("trail", None, None)).unwrap().channel, Channel::Rgb);
    assert_eq!(resolve(&reg, &args("surface", None, None)).unwrap_err(), "\"surface\" is ambiguous: surface.trail, surface.shelter");
    assert_eq!(resolve(&reg, &args("x.ds", None, None)).unwrap_err(), "Depth24PlusStencil8 is depth+stencil: register a depth-only view or copy");
}

fn app() -> App {
    let mut app = App::new();
    app.init_resource::<Assets<Image>>().add_plugins((ConsolePlugin, TexViewPlugin));
    let image = Image::new_uninit(
        Extent3d { width: 512, height: 256, depth_or_array_layers: 1 },
        TextureDimension::D2,
        TextureFormat::Depth32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    app.register_debug_texture(DebugTexture::image("precip.occlusion", "occlusion depth", Handle::default(), &image));
    app.register_debug_texture(DebugTexture::render("surface.trail", "ring", (1024, 1024), TextureFormat::Rgba16Float, |_| None));
    app.update();
    app
}

fn submit(app: &mut App, line: &str) {
    app.world_mut().write_message(ConsoleSubmit::typed(line));
    app.update();
}

fn last_line(app: &App) -> (Level, String) {
    let line = app.world().resource::<DevConsole>().log().lines().last().unwrap().clone();
    (line.level, line.text)
}

fn nodes(app: &mut App) -> usize {
    app.world_mut().query::<&ImageNode>().iter(app.world()).count()
}

#[test]
fn console_show_hide_drive_the_view() {
    let mut app = app();
    assert_eq!(nodes(&mut app), 0);
    assert!(app.world().resource::<TexViewFrame>().0.is_none());

    submit(&mut app, "tex list");
    let lines: Vec<String> = app.world().resource::<DevConsole>().log().lines().map(|l| l.text.clone()).collect();
    assert!(lines.contains(&"precip.occlusion  512x256  Depth32Float  occlusion depth".to_owned()), "{lines:?}");
    assert!(lines.contains(&"surface.trail  1024x1024  Rgba16Float  ring".to_owned()), "{lines:?}");

    submit(&mut app, "tex show occl 1 0");
    assert_eq!(last_line(&app), (Level::Info, "tex: precip.occlusion 512x256 Depth32Float, r, 1..0 (bottom right; `tex hide`)".to_owned()));
    assert_eq!(
        app.world().resource::<TexView>().shown,
        Some(Shown { name: "precip.occlusion".into(), channel: Channel::R, min: 1.0, max: 0.0 })
    );
    assert_eq!(nodes(&mut app), 1);
    assert_eq!(app.world().resource::<Assets<Image>>().len(), 1);
    let (_, target) = app.world().resource::<Assets<Image>>().iter().next().unwrap();
    assert_eq!((target.width(), target.height()), (512, 256));
    let frame = app.world().resource::<TexViewFrame>().0.clone().unwrap();
    assert_eq!(frame.kind, SampleKind::Float);
    assert_eq!(frame.params, params(Channel::R, 1.0, 0.0));

    // Switching replaces the view (one node, one target).
    submit(&mut app, "vis show trail a -1 1");
    assert_eq!(nodes(&mut app), 1);
    assert_eq!(app.world().resource::<Assets<Image>>().len(), 1);
    assert_eq!(app.world().resource::<TexViewFrame>().0.as_ref().unwrap().params, params(Channel::A, -1.0, 1.0));

    // An unknown name fails and keeps the view.
    submit(&mut app, "tex show water");
    assert_eq!(last_line(&app), (Level::Error, "tex: no debug texture \"water\" (`tex list` lists them)".to_owned()));
    assert_eq!(app.world().resource::<TexView>().shown.as_ref().unwrap().name, "surface.trail");

    submit(&mut app, "tex hide");
    assert_eq!(nodes(&mut app), 0);
    assert_eq!(app.world().resource::<Assets<Image>>().len(), 0);
    assert!(app.world().resource::<TexViewFrame>().0.is_none());
}
