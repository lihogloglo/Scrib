#![allow(clippy::float_cmp)]

use std::sync::Arc;

use bevy_app::App;
use bevy_asset::{Assets, Handle};
use bevy_camera::Camera;
use bevy_camera::visibility::InheritedVisibility;
use bevy_image::Image;
use bevy_light::NotShadowCaster;
use bevy_math::{DVec2, IVec2, IVec3, Vec3, Vec4};
use bevy_mesh::Mesh3d;
use bevy_mesh::skinning::SkinnedMesh;
use bevy_transform::components::GlobalTransform;
use sf_coords::WorldPos;
use sf_sky_bevy::sf_sky::SkyClock;
use sf_sky_bevy::{SkyConfig, SkyTime};
use sf_surface::view::DebugView;
use sf_surface::{Channel, Worldspace};
use sf_weather_bevy::sf_weather::{
    Climate, ClimateRaster, WeatherBook, WeatherPreset, WeatherState,
};
use sf_weather_bevy::{SurfaceDebug, WeatherPlugin, WeatherRecords, WorldSurface, WorldWeatherMap};
use sf_wind_bevy::WindField;

use crate::shelter::{CanopyFrame, ShelterAnchor, ShelterFrame};
use crate::uniform::*;
use crate::*;

pub(crate) fn id(s: &str) -> Worldspace {
    Worldspace::new(s)
}

/// The uniform's Rust slots follow the WGSL struct, field for field.
#[test]
fn uniform_matches_the_wgsl_struct() {
    let wgsl = include_str!("layer.wesl");
    let body = wgsl
        .split("struct SurfaceUniform {")
        .nth(1)
        .unwrap()
        .split('}')
        .next()
        .unwrap();
    let fields: Vec<(&str, &str)> = body
        .lines()
        .map(str::trim)
        .filter(|l| !l.starts_with("//") && l.contains(':'))
        .map(|l| {
            let (name, ty) = l.split_once(':').unwrap();
            (name.trim(), ty.trim().trim_end_matches(','))
        })
        .collect();
    assert_eq!(
        fields,
        [
            ("shelter_clip_from_world", "mat4x4<f32>"),
            ("window", "vec4<f32>"),
            ("cells", "vec4<i32>"),
            ("shelter", "vec4<f32>"),
            ("misc", "vec4<u32>"),
            ("time", "vec4<f32>"),
            ("canopy", "vec4<f32>"),
            ("snow_layer", "vec4<f32>"),
            ("ash_layer", "vec4<f32>"),
            ("cover_uv", "vec4<f32>"),
            ("trail", "vec4<f32>"),
            ("trail_cells", "vec4<i32>"),
            ("trail_prev", "vec4<f32>"),
            ("trail_prev_cells", "vec4<i32>"),
        ]
    );
    assert_eq!(
        [
            U_SHELTER_CLIP,
            U_WINDOW,
            U_CELLS,
            U_SHELTER,
            U_MISC,
            U_TIME,
            U_CANOPY,
            U_COVER,
            U_COVER_UV,
            U_TRAIL,
            U_TRAIL_CELLS,
            U_TRAIL_PREV,
            U_TRAIL_PREV_CELLS,
            U_SLOTS
        ],
        [0, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 16, 17]
    );
    let konst = |name: &str| -> u32 {
        let line = wgsl
            .lines()
            .find(|l| l.starts_with(&format!("const {name}: u32 = ")))
            .expect(name);
        line.split('=')
            .nth(1)
            .unwrap()
            .trim()
            .trim_end_matches("u;")
            .parse()
            .unwrap()
    };
    assert_eq!(
        [
            konst("FLAG_WET"),
            konst("FLAG_PUDDLE"),
            konst("FLAG_SNOW"),
            konst("FLAG_ASH")
        ],
        [FLAG_WET, FLAG_PUDDLE, FLAG_SNOW, FLAG_ASH]
    );
    for v in DebugView::ALL {
        assert_eq!(
            konst(&format!("DEBUG_{}", v.name().to_ascii_uppercase())),
            v as u32
        );
    }
    assert_eq!(
        SurfaceUniform::flags_of([0, 3, 0, 1]),
        FLAG_PUDDLE | FLAG_ASH
    );
}

fn book() -> WeatherBook {
    WeatherBook::new(
        [(id("clear"), WeatherPreset::default())],
        [(
            id("land"),
            Climate {
                weights: vec![(id("clear"), 1.0)],
                ..Climate::default()
            },
        )],
        [],
    )
}

/// A headless app (no render app): weather, the surface state over a 256 m raster, the plugin,
/// and a floating origin one 256 m grid cell east.
pub(crate) fn app() -> App {
    let mut app = App::new();
    app.insert_resource(Assets::<Image>::default())
        .insert_resource(SkyTime(SkyClock::default()))
        .init_resource::<SkyConfig>()
        .init_resource::<sf_sky_bevy::WeatherState>()
        .insert_resource(WindField::new(
            sf_wind_bevy::sf_wind::Wind::default(),
            sf_wind_bevy::sf_wind::WindSettings::default(),
        ))
        .add_plugins(WeatherPlugin {
            book: Arc::new(book()),
            state: WeatherState::new(7),
        })
        .add_plugins(SurfaceWeatherPlugin)
        .insert_resource(SurfaceOrigin(WorldPos::new(
            IVec3::new(1, 0, 0),
            Vec3::ZERO,
        )));
    let land = app
        .world()
        .resource::<WeatherRecords>()
        .0
        .climate(&id("land"))
        .unwrap();
    let raster = ClimateRaster::new(DVec2::ZERO, 128.0, 2, 2, &[Some(land); 4], 256.0).unwrap();
    app.insert_resource(WorldWeatherMap::new(raster).with_worldspace(id("ws")));
    app
}

fn uniform(app: &App) -> SurfaceUniform {
    *app.world().resource::<SurfaceUniform>()
}

fn stats(app: &App) -> SurfaceStats {
    *app.world().resource::<SurfaceStats>()
}

/// The texel holding window cell `cell` among this frame's writes, if written.
fn written(app: &App, cell: IVec2) -> Option<[u8; 4]> {
    let t = cell.rem_euclid(IVec2::splat(128)).as_uvec2();
    app.world()
        .resource::<CellWrites>()
        .0
        .iter()
        .rev()
        .find_map(|w| {
            let l = t.as_ivec2() - w.texel.as_ivec2();
            (l.x >= 0 && l.y >= 0 && l.x < w.size.x.cast_signed() && l.y < w.size.y.cast_signed())
                .then(|| {
                    let i = 4 * (l.y.unsigned_abs() * w.size.x + l.x.unsigned_abs()) as usize;
                    [w.data[i], w.data[i + 1], w.data[i + 2], w.data[i + 3]]
                })
        })
}

#[test]
#[allow(clippy::too_many_lines)]
fn headless_plugin_keeps_window_shelter_and_uniform_sane() {
    let mut app = app();
    // Dry and no camera: nothing placed, nothing drawn.
    app.update();
    assert!(
        app.world().get_resource::<SurfaceGpu>().is_some(),
        "resources exist"
    );
    assert_eq!(uniform(&app).window.w, 0.0);
    assert!(app.world().resource::<ShelterFrame>().0.is_none());

    let camera = app
        .world_mut()
        .spawn((
            Camera::default(),
            SurfaceWeatherCamera,
            GlobalTransform::from_translation(Vec3::new(-240.0, 5.0, 20.0)),
        ))
        .id();
    // No directional light at all (night, or the sky turned the sun's shadows off): the maps
    // belong to the plugin's own anchor and are drawn anyway.
    let visible = InheritedVisibility::VISIBLE;
    let at = |x: f32, y: f32| GlobalTransform::from_translation(Vec3::new(x, y, 20.0));
    let roof = app
        .world_mut()
        .spawn((Mesh3d(Handle::default()), at(-238.0, 9.0), visible))
        .id();
    let tree = app
        .world_mut()
        .spawn((Mesh3d(Handle::default()), at(-236.0, 12.0), visible, ShelterFoliage))
        .id();
    app.world_mut()
        .spawn((Mesh3d(Handle::default()), at(500.0, 9.0), visible));
    app.world_mut().spawn((
        Mesh3d(Handle::default()),
        at(-238.0, 0.0),
        visible,
        NoSurfaceShelter,
    ));
    app.world_mut().spawn((
        Mesh3d(Handle::default()),
        at(-238.0, 0.0),
        visible,
        NotShadowCaster,
    ));
    app.world_mut().spawn((
        Mesh3d(Handle::default()),
        at(-238.0, 1.0),
        visible,
        SkinnedMesh::default(),
    ));
    app.update();
    // World camera (16, 20) -> cell (0, 0); window from (-64, -64); its corner in render space
    // is world (-2048, -2048) minus the origin (256, 0).
    let u = uniform(&app);
    assert_eq!(u.window, Vec4::new(-2304.0, -2048.0, 1.0 / 32.0, 1.0));
    assert_eq!(u.cells, bevy_math::IVec4::new(64, 64, 128, 0));
    assert_eq!(u.flags, 0, "dry: no layers");
    assert_eq!(u.shelter.x, 0.0, "dry and no debug view: no shelter map");
    let full = stats(&app).full_uploads;
    assert!(full >= 1);

    // Rain-soaked land: the window is rewritten, the layer turns on, the shelter map is drawn.
    sf_weather_bevy::surface_set(app.world_mut(), Channel::Wet, 0.5, "land").unwrap();
    app.world_mut().resource_mut::<CellWrites>().0.clear();
    app.update();
    assert_eq!(stats(&app).full_uploads, full + 1);
    assert_eq!(
        written(&app, IVec2::new(0, 0)),
        Some([128, 0, 0, 0]),
        "camera cell, wet 0.5"
    );
    assert_eq!(
        written(&app, IVec2::new(-1, 0)),
        Some([0, 0, 0, 0]),
        "outside the raster: dry"
    );
    let u = uniform(&app);
    assert_eq!(u.flags, FLAG_WET);
    assert_eq!(u.shelter, Vec4::new(1.0, 350.0, 0.125, 64.0));
    let req = app
        .world()
        .resource::<ShelterFrame>()
        .0
        .clone()
        .expect("shelter redraw");
    assert_eq!(
        req.casters.iter().map(|c| c.1.id()).collect::<Vec<_>>(),
        vec![roof],
        "only the near, unskinned caster"
    );
    assert_eq!((req.subview, req.size), (SHELTER_SUBVIEW, 1024));
    let gpu = app.world().resource::<SurfaceGpu>().clone();
    assert_eq!(req.image, gpu.shelter.id());
    let anchor = app.world().resource::<ShelterAnchor>().0;
    assert_eq!(req.light, anchor, "owned by the anchor, not a light");
    let leaves = app
        .world()
        .resource::<CanopyFrame>()
        .0
        .clone()
        .expect("canopy redraw on the same frame");
    assert_eq!(
        leaves.casters.iter().map(|c| c.1.id()).collect::<Vec<_>>(),
        vec![tree],
        "leaves go to the canopy map only"
    );
    assert_eq!(
        (leaves.light, leaves.subview, leaves.image, leaves.clip_from_world),
        (anchor, CANOPY_SUBVIEW, gpu.canopy.id(), req.clip_from_world)
    );
    assert_eq!(u.canopy, Vec4::new(1.0, sf_surface::view::SHELTER_LEAF_PASS, 0.0, 0.0));
    // The roof sits over the camera's ground in the map, higher.
    let ground = u
        .shelter_clip_from_world
        .project_point3(Vec3::new(-238.0, 5.0, 20.0));
    let top = u
        .shelter_clip_from_world
        .project_point3(Vec3::new(-238.0, 9.0, 20.0));
    assert!(ground.truncate().distance(top.truncate()) < 1e-6 && top.z > ground.z);
    // The uniform's slots (what the render world writes), no NaN.
    assert!(u.slots().iter().flatten().all(|b| !f32::from_bits(*b).is_nan()));

    // Steady: no writes, no redraw.
    app.world_mut().resource_mut::<CellWrites>().0.clear();
    let before = stats(&app);
    app.update();
    assert!(app.world().resource::<CellWrites>().0.is_empty());
    assert!(app.world().resource::<ShelterFrame>().0.is_none());
    assert_eq!(stats(&app).shelter_redraws, before.shelter_redraws);

    // One cell east: one 128-texel column enters; 32 m is past the shelter's 16 m recenter.
    app.world_mut()
        .entity_mut(camera)
        .insert(GlobalTransform::from_translation(Vec3::new(
            -208.0, 5.0, 20.0,
        )));
    app.update();
    let after = stats(&app);
    assert_eq!(after.cell_texels - before.cell_texels, 128);
    assert_eq!(after.shelter_redraws, before.shelter_redraws + 1);
    assert_eq!(
        written(&app, IVec2::new(1 + 63, 0)),
        Some([0, 0, 0, 0]),
        "the entering column (outside the raster)"
    );

    // Dry again but a debug view on: the shelter map stays.
    {
        let mut s = app.world_mut().resource_mut::<WorldSurface>();
        s.state.clear();
        s.touch();
    }
    app.world_mut()
        .insert_resource(SurfaceDebug(DebugView::Exposure));
    app.update();
    let u = uniform(&app);
    assert_eq!(
        (u.flags, u.debug, u.shelter.x),
        (0, DebugView::Exposure, 1.0)
    );
    app.world_mut()
        .insert_resource(SurfaceDebug(DebugView::Off));
    app.update();
    assert_eq!(uniform(&app).shelter.x, 0.0, "off: no map");

    // A/B switches: wet again; `--off shelter` (Low) keeps the flag but draws no map,
    // `--off wet` and `--off surface` clear the flags (the hook stays compiled in).
    sf_weather_bevy::surface_set(app.world_mut(), Channel::Wet, 0.5, "land").unwrap();
    app.update();
    assert_eq!((uniform(&app).flags, uniform(&app).shelter.x), (FLAG_WET, 1.0));
    let set = |app: &mut App, layer, wet, shelter| {
        app.world_mut().insert_resource(SurfaceSettings { layer, wet, puddles: true, shelter, cover: true, trails: true });
        app.update();
        let u = uniform(app);
        (u.flags, u.shelter.x)
    };
    assert_eq!(set(&mut app, true, true, false), (FLAG_WET, 0.0));
    assert_eq!(set(&mut app, true, false, true), (0, 0.0));
    assert_eq!(set(&mut app, false, true, true), (0, 0.0));
    assert_eq!(set(&mut app, true, true, true), (FLAG_WET, 1.0));

    // Interior: the camera loses its mark; the window goes.
    app.world_mut()
        .entity_mut(camera)
        .remove::<SurfaceWeatherCamera>();
    app.update();
    assert_eq!(uniform(&app).window.w, 0.0);
    assert_eq!(uniform(&app).flags, 0);
}

/// `--off puddles` clears the puddle flag only; rain at the camera is the wetting rate over the
/// storm's; the ring clock runs only while rings can show and stops them when the rain stops.
#[test]
fn puddle_switch_and_ring_clock() {
    let off = SurfaceSettings {
        puddles: false,
        ..SurfaceSettings::default()
    };
    assert_eq!(off.flag_mask(), FLAG_WET | FLAG_SNOW | FLAG_ASH);
    assert_eq!(SurfaceSettings::default().flag_mask() & FLAG_PUDDLE, FLAG_PUDDLE);
    assert_eq!(rain_of(2.0, 2.0), 1.0);
    assert_eq!(rain_of(1.0, 2.0), 0.5);
    assert_eq!(rain_of(5.0, 2.0), 1.0);
    assert_eq!(rain_of(0.0, 2.0), 0.0);
    assert_eq!(rain_of(1.0, 0.0), 0.0);

    let mut app = App::new();
    let mut weather = CurrentWeather {
        valid: true,
        ..Default::default()
    };
    weather.frame.surface.wet = 1.0;
    app.insert_resource(SurfaceUniform {
        flags: FLAG_PUDDLE,
        ..SurfaceUniform::default()
    })
    .insert_resource(weather)
    .init_resource::<WorldSurface>()
    .init_resource::<Time>()
    .add_systems(bevy_app::Update, ripple_clock);
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_mins(25));
    app.update();
    assert_eq!(uniform(&app).time, Vec4::new(500.0, 0.5, 0.0, 0.0));
    // Rain stops: rain 0 (no rings), the clock holds.
    app.world_mut()
        .resource_mut::<CurrentWeather>()
        .frame
        .surface
        .wet = 0.0;
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_secs(10));
    app.update();
    assert_eq!(uniform(&app).time, Vec4::new(500.0, 0.0, 0.0, 0.0));
    // Raining with no puddles in the window: no rings either.
    app.world_mut()
        .resource_mut::<CurrentWeather>()
        .frame
        .surface
        .wet = 2.0;
    app.world_mut().resource_mut::<SurfaceUniform>().flags = FLAG_WET;
    app.update();
    assert_eq!(uniform(&app).time.y, 0.0);
}

/// The cover layer array (W5a): the neutral placeholder without layers, 4 slices of 512² with
/// full mips once a layer is set (the unset channel neutral, its uniform slot zero), and the
/// plugin repacking it when the host's layers change.
#[test]
fn cover_array_packs_layers_and_placeholder() {
    use crate::cover::{
        COVER_SIZE, CoverChannel, LayerImage, NEUTRAL_ALBEDO, NEUTRAL_NRAO, SurfaceLayer, cover_slots, pack,
    };
    let empty = pack(&[None, None]);
    assert_eq!((empty.size, empty.mips, empty.slices, empty.data.as_slice()), (1, 1, 1, &NEUTRAL_ALBEDO[..]));
    assert_eq!(cover_slots(&[None, None]), [Vec4::ZERO; 2]);

    // Ash only: a flat sRGB grey 2×2 albedo with height 200, no normal or arm map.
    let mut layer = SurfaceLayer::new(CoverChannel::Ash, "ash.dds");
    layer.tiling_m = 8.0;
    layer.roughness = 0.6;
    layer.contrast = 3.0;
    let source = CoverSource {
        layer,
        albedo_height: LayerImage { width: 2, height: 2, rgba: [90, 90, 90, 200].repeat(4) },
        normal: None,
        arm: None,
    };
    let layers = [None, Some(source)];
    let array = pack(&layers);
    let per_slice: usize = (0..10).map(|m| (512usize >> m).pow(2) * 4).sum();
    assert_eq!((array.size, array.mips, array.slices, array.data.len()), (COVER_SIZE, 10, 4, per_slice * 4));
    let texel = |slice: usize| &array.data[slice * per_slice..slice * per_slice + 4];
    // Snow (unset): neutral slices. Ash: its albedo (sRGB kept through the linear filter, height
    // in alpha) and a flat normal with the record's roughness (0.6 · 255 = 153) and AO 1.
    assert_eq!((texel(0), texel(1)), (&NEUTRAL_ALBEDO[..], &NEUTRAL_NRAO[..]));
    assert_eq!((texel(2), texel(3)), (&[90, 90, 90, 200][..], &[128, 128, 153, 255][..]));
    // The last mip of the last slice ends the buffer.
    assert_eq!(&array.data[array.data.len() - 4..], &[128, 128, 153, 255]);
    assert_eq!(cover_slots(&layers), [Vec4::ZERO, Vec4::new(0.125, 0.03, 0.0, 3.0)]);

    // The plugin: the placeholder at startup, repacked on insert.
    let mut app = app();
    app.update();
    let slices = |app: &App| {
        let gpu = app.world().resource::<SurfaceGpu>();
        app.world().resource::<Assets<Image>>().get(&gpu.cover).unwrap().texture_descriptor.size.depth_or_array_layers
    };
    assert_eq!(slices(&app), 1);
    app.insert_resource(CoverLayers(layers));
    app.update();
    assert_eq!((slices(&app), stats(&app).cover_slices), (4, 4));
    assert_eq!(uniform(&app).cover[1], Vec4::new(0.125, 0.03, 0.0, 3.0));
    assert_eq!(uniform(&app).slots()[U_COVER + 1], [0.125f32, 0.03, 0.0, 3.0].map(f32::to_bits));
}

/// W5b: a plain albedo gets a height from its luminance and a normal from that height (the
/// bitmap-to-material fallback); a real height in alpha is kept; the cover UV wraps the origin in
/// f64; `--off cover` clears the snow and ash flags only.
#[test]
fn cover_derives_height_normal_and_uv() {
    use crate::cover::{LayerImage, cover_uv, normal_from_height, with_height};
    // A 4×1 ramp, dark to bright, all opaque: heights spread 0..255 in luminance order.
    let ramp = LayerImage { width: 4, height: 1, rgba: [20, 80, 160, 250].iter().flat_map(|&v| [v, v, v, 255]).collect() };
    let h: Vec<u8> = with_height(&ramp).rgba.as_chunks::<4>().0.iter().map(|p| p[3]).collect();
    assert_eq!((h[0], h[3]), (0, 255));
    assert!(h.windows(2).all(|w| w[0] < w[1]), "{h:?}");
    // RGB untouched; a height already in alpha is kept.
    assert_eq!(with_height(&ramp).rgba.as_chunks::<4>().0[1][..3], [80, 80, 80]);
    let real = LayerImage { width: 1, height: 1, rgba: vec![10, 10, 10, 77] };
    assert_eq!(with_height(&real), real);
    // Normals: flat height, flat normal; height rising toward +X tilts the normal to -X; toward
    // the image's top (row 0 high), to -Y (the normal leans away from the rise).
    let flat = LayerImage { width: 2, height: 2, rgba: [50, 50, 50, 128].repeat(4) };
    assert!(normal_from_height(&flat).rgba.as_chunks::<4>().0.iter().all(|p| *p == [128, 128, 255, 255]));
    let mut px = [0u8, 0, 0, 0].repeat(9);
    for (i, p) in px.as_chunks_mut::<4>().0.iter_mut().enumerate() {
        #[allow(clippy::cast_possible_truncation)]
        let x = (i % 3) as u8;
        p[3] = 60 * x;
    }
    let n = normal_from_height(&LayerImage { width: 3, height: 3, rgba: px.clone() });
    let centre = n.rgba.as_chunks::<4>().0[4];
    assert!(centre[0] < 128 && centre[1] == 128, "{centre:?}");
    for (i, p) in px.as_chunks_mut::<4>().0.iter_mut().enumerate() {
        #[allow(clippy::cast_possible_truncation)]
        let y = (i / 3) as u8;
        p[3] = 60 * (2 - y);
    }
    let centre = normal_from_height(&LayerImage { width: 3, height: 3, rgba: px }).rgba.as_chunks::<4>().0[4];
    assert!(centre[0] == 128 && centre[1] < 128, "{centre:?}");

    // UV at the origin: X, Z over the tiling, wrapped, exact far from the world origin.
    let slots = [Vec4::new(0.25, 0.15, 1.0, 1.0), Vec4::ZERO];
    let uv = cover_uv(bevy_math::DVec3::new(1_000_001.0, 7.0, -2.5), &slots);
    assert_eq!(uv, Vec4::new(0.25, 0.375, 0.0, 0.0));

    let off = SurfaceSettings { cover: false, ..SurfaceSettings::default() };
    assert_eq!(off.flag_mask(), FLAG_WET | FLAG_PUDDLE);
}

/// Trails (W6a): stamps in range reach the frame (out of range dropped, read once), the ring is
/// placed around the camera in whole texels, and no cover look or `--off trails` turns it off.
#[test]
fn trails_take_stamps_and_place_the_ring() {
    use bevy_math::Vec2;
    use sf_surface::trail::{FOOT_HALF_M, RING_SIZE, window_min};
    let frame = |app: &App| app.world().resource::<TrailFrame>().clone();
    let mut app = app();
    app.world_mut().spawn((Camera::default(), SurfaceWeatherCamera, GlobalTransform::from_translation(Vec3::new(10.0, 0.0, -3.0))));
    app.update();
    // No cover look: nothing to cut.
    assert!(!frame(&app).on);
    assert_eq!(uniform(&app).trail, Vec4::ZERO);
    app.world_mut().resource_mut::<SurfaceUniform>().cover[0] = Vec4::new(0.25, 0.15, 1.0, 1.0);
    let origin = app.world().resource::<SurfaceOrigin>().0;
    let at = |x: f32, z: f32| origin.offset(Vec3::new(x, 0.0, z));
    let stamp = |p| TrailStamp { at: p, forward: Vec2::new(0.0, 2.0), half_m: FOOT_HALF_M, depth: 1.0 };
    app.world_mut().write_message(stamp(at(12.0, -3.0)));
    app.world_mut().write_message(stamp(at(60.0, -3.0)));
    app.update();
    let f = frame(&app);
    assert!(f.on);
    assert_eq!(f.stamps.len(), 1, "the stamp 50 m away is out of range");
    assert_eq!((f.stamps[0].center, f.stamps[0].axis, f.depth_m), (Vec2::new(12.0, -3.0), Vec2::Y, 0.15));
    let world = origin.offset(Vec3::new(10.0, 0.0, -3.0)).to_f64();
    let min = window_min(DVec2::new(world.x, world.z));
    assert_eq!(f.new_min, min);
    let u = uniform(&app);
    assert_eq!((u.trail.w, u.trail.z, u.trail_cells.z), (0.15, 16.0, RING_SIZE));
    assert_eq!(IVec2::new(u.trail_cells.x, u.trail_cells.y), min.rem_euclid(IVec2::splat(RING_SIZE)));
    // The camera sits half a ring (32 m) past the corner, within a texel.
    let past = 10.0 - u.trail.x - 32.0;
    assert!((0.0..0.0625).contains(&past), "{past}");
    assert_eq!(stats(&app).trail_stamps, 1);
    // Messages are read once.
    app.update();
    assert!(frame(&app).stamps.is_empty());
    // `--off trails`: off, the shading ignores the ring.
    app.world_mut().resource_mut::<SurfaceSettings>().trails = false;
    app.world_mut().write_message(stamp(at(12.0, -3.0)));
    app.update();
    assert!(!frame(&app).on);
    assert_eq!(uniform(&app).trail, Vec4::ZERO);
}

/// SW-5/SW-6 "resets": a shelter redraw draws its casters even when no camera or light frustum
/// sees them (Bevy extracts only meshes some view sees): the redraw marks them visible, a far
/// proxy out of the camera's `VisibilityRange` excepted, and only on redraw frames.
#[test]
fn shelter_redraw_makes_its_casters_visible() {
    use bevy_camera::visibility::{ViewVisibility, VisibilityRange, VisibleEntityRanges};
    let mut app = app();
    app.init_resource::<VisibleEntityRanges>();
    app.world_mut().spawn((Camera::default(), SurfaceWeatherCamera, GlobalTransform::from_translation(Vec3::new(-240.0, 5.0, 20.0))));
    let at = |x: f32, y: f32| GlobalTransform::from_translation(Vec3::new(x, y, 20.0));
    let caster = |app: &mut App, x: f32| {
        app.world_mut()
            .spawn((Mesh3d(Handle::default()), at(x, 9.0), InheritedVisibility::VISIBLE, ViewVisibility::HIDDEN))
            .id()
    };
    // Behind the camera, outside every frustum: nothing marked it visible.
    let roof = caster(&mut app, -238.0);
    let proxy = caster(&mut app, -230.0);
    app.world_mut().entity_mut(proxy).insert(VisibilityRange::abrupt(500.0, 2000.0));
    let visible = |app: &App, e| app.world().get::<ViewVisibility>(e).unwrap().get();
    app.update();
    assert!(!visible(&app, roof), "dry: no shelter map, nothing marked");
    sf_weather_bevy::surface_set(app.world_mut(), Channel::Snow, 1.0, "land").unwrap();
    app.update();
    let req = app.world().resource::<ShelterFrame>().0.clone().expect("shelter redraw");
    let mut casters: Vec<_> = req.casters.iter().map(|c| c.1.id()).collect();
    casters.sort_unstable();
    let mut expected = vec![roof, proxy];
    expected.sort_unstable();
    assert_eq!(casters, expected);
    assert!(visible(&app, roof), "the redraw's caster is extracted");
    assert!(!visible(&app, proxy), "out of the camera's visibility range: its near LOD casts instead");
    // A steady frame: no redraw, nothing marked (Bevy's own reset hides it again).
    *app.world_mut().get_mut::<ViewVisibility>(roof).unwrap() = ViewVisibility::HIDDEN;
    app.update();
    assert!(app.world().resource::<ShelterFrame>().0.is_none());
    assert!(!visible(&app, roof));
}

/// SW-5/SW-6 "resets": a floating-origin shift with the camera at the same world point moves
/// nothing the layer samples in the world: the cell window's world corner and texel, the trail
/// ring's world corner and texel, no full rewrite, and the origin grid the shader's hash grids
/// wrap to is the host's (`sf_coords::GRID_CELL`).
#[test]
fn origin_shift_keeps_the_world_anchored_inputs() {
    assert_eq!(sf_surface::layer::ORIGIN_GRID_M, sf_coords::GRID_CELL);
    let mut app = app();
    let cam = app.world_mut().spawn((Camera::default(), SurfaceWeatherCamera, GlobalTransform::from_translation(Vec3::new(10.0, 0.0, -3.0)))).id();
    app.world_mut().resource_mut::<SurfaceUniform>().cover[0] = Vec4::new(0.25, 0.15, 1.0, 1.0);
    app.update();
    sf_weather_bevy::surface_set(app.world_mut(), Channel::Snow, 1.0, "land").unwrap();
    app.update();
    let world_of = |app: &App| {
        let u = uniform(app);
        let o = app.world().resource::<SurfaceOrigin>().0.to_f64();
        let w = |x: f32, z: f32| DVec2::new(f64::from(x) + o.x, f64::from(z) + o.z);
        (w(u.window.x, u.window.y), u.cells, w(u.trail.x, u.trail.y), u.trail_cells, app.world().resource::<TrailFrame>().new_min)
    };
    let before = world_of(&app);
    let full = stats(&app).full_uploads;
    assert!(uniform(&app).trail.w > 0.0, "the ring is on");
    // Big_space recentres: render space moves one grid cell, the camera stays put in the world.
    app.world_mut().resource_mut::<SurfaceOrigin>().0 = WorldPos::new(IVec3::new(2, 0, 0), Vec3::ZERO);
    *app.world_mut().get_mut::<GlobalTransform>(cam).unwrap() = GlobalTransform::from_translation(Vec3::new(10.0 - 256.0, 0.0, -3.0));
    app.update();
    assert_eq!(world_of(&app), before);
    assert_eq!(stats(&app).full_uploads, full, "a shift rewrites nothing");
}
