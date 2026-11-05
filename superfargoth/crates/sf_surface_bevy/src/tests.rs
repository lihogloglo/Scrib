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

fn id(s: &str) -> Worldspace {
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
            U_SLOTS
        ],
        [0, 4, 5, 6, 7, 8, 9, 10]
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
fn app() -> App {
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
        app.world_mut().insert_resource(SurfaceSettings { layer, wet, shelter });
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
