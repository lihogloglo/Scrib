//! The debug-texture registry (feature `texview`): registration, replacement, lookup.

use bevy_app::App;
use bevy_asset::{Handle, RenderAssetUsages};
use bevy_image::Image;
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::texview::{DebugTexture, DebugTextureAppExt, DebugTextures, TexSource};

fn render(name: &str) -> DebugTexture {
    DebugTexture::render(name, "test", (256, 128), TextureFormat::R16Float, |_| None)
}

#[test]
fn image_entry_reads_size_and_format_from_the_descriptor() {
    let image = Image::new_uninit(
        Extent3d { width: 512, height: 256, depth_or_array_layers: 1 },
        TextureDimension::D2,
        TextureFormat::Depth32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    let tex = DebugTexture::image("a.depth", "a depth map", Handle::default(), &image);
    assert_eq!(tex.size, (512, 256));
    assert_eq!(tex.format, TextureFormat::Depth32Float);
    assert!(matches!(tex.source, TexSource::Image(_)));
    assert_eq!(tex.summary(), "a.depth  512x256  Depth32Float  a depth map");
}

#[test]
fn register_keeps_order_and_replaces_same_name() {
    let mut app = App::new();
    app.register_debug_texture(render("surface.trail"))
        .register_debug_texture(render("surface.shelter"))
        .register_debug_texture(DebugTexture::render("Surface.Trail", "again", (1, 1), TextureFormat::Rgba8Unorm, |_| None));
    let reg = app.world().resource::<DebugTextures>();
    let names: Vec<&str> = reg.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, ["Surface.Trail", "surface.shelter"]);
    assert_eq!(reg.iter().next().unwrap().description, "again");
    assert_eq!(reg.len(), 2);
}

#[test]
#[should_panic(expected = "not empty, no spaces")]
fn register_refuses_a_name_with_spaces() {
    DebugTextures::default().register(render("surface trail"));
}

#[test]
fn find_exact_then_unique_substring() {
    let mut reg = DebugTextures::default();
    for n in ["surface.shelter", "surface.canopy", "surface.trail", "precip.occlusion"] {
        reg.register(render(n));
    }
    assert_eq!(reg.find("SURFACE.TRAIL").unwrap().name, "surface.trail");
    assert_eq!(reg.find("trail").unwrap().name, "surface.trail");
    assert_eq!(reg.find("occl").unwrap().name, "precip.occlusion");
    assert_eq!(reg.find("water").err().unwrap(), "no debug texture \"water\" (`tex list` lists them)");
    assert_eq!(reg.find("surface").err().unwrap(), "\"surface\" is ambiguous: surface.shelter, surface.canopy, surface.trail");
}
