use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use sf_wind::WindSource;

use super::*;

fn burst_field() -> WindField {
    let mut field = WindField::new(Wind::default(), WindSettings::default());
    for i in 0..MAX_SOURCES {
        #[expect(clippy::cast_precision_loss, reason = "small index")]
        let x = i as f64 * 10.0;
        let source = WindSource::radial(WorldPos::from_f64(bevy::math::DVec3::new(x, 0.0, 5.0)), 8.0, 20.0)
            .with_lifetime(0.05, 1.5, 1.0);
        field.state.add_source(source).expect("capacity");
    }
    field
}

#[test]
fn pack_layout_matches_wgsl() {
    let mut field = burst_field();
    field.step(0.1, WorldPos::default());
    field.step(0.1, WorldPos::default());
    let mut data = [[0.0; 4]; DATA_VEC4S];
    field.pack(&mut data);
    assert_eq!(DATA_BYTES, 3264);
    assert_eq!(data[..6], field.frame().params.to_array());
    assert_eq!(data[6..12], field.prev_frame().params.to_array());
    assert_eq!(data[12..15], field.frame().sources[0].to_array());
    let prev_sources = 12 + 3 * MAX_SOURCES;
    assert_eq!(data[prev_sources..prev_sources + 3], field.prev_frame().sources[0].to_array());
    assert_eq!(data[DATA_VEC4S - 3..], field.prev_frame().sources[MAX_SOURCES - 1].to_array());
}

#[test]
fn prev_frame_is_last_frame_at_the_current_origin() {
    let mut field = burst_field();
    let origin = WorldPos::default();
    field.step(0.25, origin);
    let first = field.frame().clone();
    field.step(0.25, origin);
    assert_eq!(field.prev_frame(), &first);
    assert_ne!(field.frame(), &first);
    // An origin shift re-expresses last frame around the new origin.
    let shifted = WorldPos::from_f64(bevy::math::DVec3::new(256.0, 0.0, -512.0));
    let before = field.state.clone();
    field.step(0.25, shifted);
    assert_eq!(field.prev_frame(), &before.frame(&field.settings, shifted));
}

/// Advance + frame both states + pack + buffer write, with all 32 sources
/// live (the worst case), against the 0.02 ms budget (weather.md "Budget").
#[test]
fn upload_cost() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_asset::<ShaderBuffer>()
        .add_plugins(WindPlugin::default());
    let mut field = burst_field();
    field.settings = WindSettings::default();
    app.insert_resource(field);
    let frames = 2_000;
    let mut total = 0.0;
    for _ in 0..frames {
        app.update();
        total += app.world().resource::<WindStats>().last_ms;
    }
    let handle = app.world().resource::<WindBuffer>().0.clone();
    let buffers = app.world().resource::<Assets<ShaderBuffer>>();
    let gpu = buffers.get(&handle).expect("buffer");
    assert_eq!(gpu.buffer_size(), DATA_BYTES as u64);
    let mut expected = [[0.0; 4]; DATA_VEC4S];
    app.world().resource::<WindField>().pack(&mut expected);
    assert_eq!(gpu.cast_slice::<[f32; 4]>().expect("initialized"), &expected[..]);
    let mean = total / f32::from(u16::try_from(frames).expect("small"));
    println!("wind upload (32 sources): mean {mean:.4} ms per frame");
    // Debug builds run ~10x slower than release; the gate is checked in release.
    assert!(mean < if cfg!(debug_assertions) { 0.2 } else { 0.02 }, "upload {mean} ms");
}
