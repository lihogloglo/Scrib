//! `bevyrrowind view <cell>`: renders one cell with a free-fly camera.
//!
//! With `--screenshot`, waits until every model and texture has loaded,
//! lets the renderer settle, saves a PNG, and exits. That's the automated
//! visual check for rendering changes.

use std::path::PathBuf;
use std::sync::Arc;

use bevy::asset::LoadState;
use bevy::asset::io::AssetSourceId;
use bevy::camera::Exposure;
use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};
use bevy::light::GlobalAmbientLight;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::world_serialization::{WorldAsset, WorldAssetRoot};
use mw_assets::MwAssetsPlugin;
use mw_world::{BaseKind, Placement};
use sf_vfs::Vfs;

pub struct ViewOptions {
    pub title: String,
    pub placements: Vec<Placement>,
    pub ambient: Option<[u8; 4]>,
    pub screenshot: Option<PathBuf>,
    /// Seconds to keep rendering after everything loaded, before capturing.
    pub settle: f32,
    /// Overrides the automatic camera: eye position and look-at target (meters).
    pub camera: Option<(Vec3, Vec3)>,
}

#[derive(Resource)]
struct Scene {
    placements: Vec<Placement>,
    ambient: Option<[u8; 4]>,
    camera: Option<(Vec3, Vec3)>,
}

#[derive(Resource)]
struct Capture {
    path: PathBuf,
    settle: f32,
    loaded_at: Option<f32>,
    taken_at: Option<f32>,
}

#[derive(Resource, Default)]
struct Models(Vec<Handle<WorldAsset>>);

pub fn run(vfs: Arc<Vfs>, options: ViewOptions) {
    let mut app = App::new();
    app.register_asset_source(AssetSourceId::Default, sf_vfs::bevy::asset_source(vfs.clone()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { title: options.title.clone(), ..Window::default() }),
            ..WindowPlugin::default()
        }))
        .add_plugins((MwAssetsPlugin { vfs }, FreeCameraPlugin))
        .insert_resource(Scene { placements: options.placements, ambient: options.ambient, camera: options.camera })
        .init_resource::<Models>()
        .add_systems(Startup, spawn);
    if let Some(path) = options.screenshot {
        app.insert_resource(Capture { path, settle: options.settle, loaded_at: None, taken_at: None })
            .add_systems(Update, capture);
    }
    app.run();
}

fn spawn(mut commands: Commands, scene: Res<Scene>, assets: Res<AssetServer>, mut models: ResMut<Models>) {
    let mut lo = Vec3::splat(f32::INFINITY);
    let mut hi = Vec3::splat(f32::NEG_INFINITY);
    let (mut spawned, mut actors) = (0, 0);

    for placement in &scene.placements {
        let t = placement.transform;
        let transform = Transform {
            translation: Vec3::from_array(t.translation),
            rotation: Quat::from_array(t.rotation),
            scale: Vec3::splat(t.scale),
        };
        lo = lo.min(transform.translation);
        hi = hi.max(transform.translation);

        if placement.kind == BaseKind::Actor {
            actors += 1; // assembled actors arrive in M3
            continue;
        }
        if let Some(model) = &placement.model {
            let handle: Handle<WorldAsset> = assets.load(model.clone());
            models.0.push(handle.clone());
            let visibility = if placement.editor_only { Visibility::Hidden } else { Visibility::Inherited };
            commands.spawn((WorldAssetRoot(handle), transform, visibility, Name::new(placement.base_id.clone())));
            spawned += 1;
        }
        if let Some(light) = placement.light {
            debug!("light {} at {:.2}", placement.base_id, transform.translation);
            let [r, g, b] = light.color;
            commands.spawn((
                PointLight {
                    color: Color::srgb(r, g, b),
                    // Morrowind radii are falloff distances; give the light
                    // enough power to read at that range.
                    intensity: 4_000.0 * light.radius * light.radius,
                    range: light.radius * 1.5,
                    radius: 0.05,
                    shadow_maps_enabled: false,
                    ..PointLight::default()
                },
                transform,
            ));
        }
    }
    info!("spawned {spawned} models, skipped {actors} actors");

    if let Some([r, g, b, _]) = scene.ambient {
        commands.insert_resource(GlobalAmbientLight {
            color: Color::srgb_u8(r, g, b),
            brightness: 400.0,
            ..GlobalAmbientLight::default()
        });
    }

    let (eye, target) = scene.camera.unwrap_or_else(|| {
        if lo.x.is_finite() {
            let center = (lo + hi) / 2.0;
            let extent = (hi - lo).max_element().max(4.0);
            (center + Vec3::new(0.0, extent * 0.45, extent * 0.55), center)
        } else {
            (Vec3::new(0.0, 2.0, 5.0), Vec3::ZERO)
        }
    });
    commands.spawn((
        Camera3d::default(),
        Exposure::INDOOR,
        Transform::from_translation(eye).looking_at(target, Vec3::Y),
        FreeCamera::default(),
    ));
}

fn capture(
    mut commands: Commands,
    mut capture: ResMut<Capture>,
    models: Res<Models>,
    assets: Res<AssetServer>,
    time: Res<Time>,
    mut exit: MessageWriter<AppExit>,
) {
    let now = time.elapsed_secs();
    if capture.loaded_at.is_none() {
        let pending = models
            .0
            .iter()
            .filter(|h| {
                !assets.is_loaded_with_dependencies(*h)
                    && !matches!(assets.get_load_state(*h), Some(LoadState::Failed(_)))
            })
            .count();
        debug!("{pending}/{} models pending at {now:.2}s", models.0.len());
        // Give up waiting after a minute; capture whatever is there.
        if pending == 0 || now > 60.0 {
            info!("assets ready at {now:.1}s ({pending} still pending)");
            capture.loaded_at = Some(now);
        }
        return;
    }
    let loaded_at = capture.loaded_at.unwrap_or(now);
    match capture.taken_at {
        None if now - loaded_at >= capture.settle => {
            commands.spawn(Screenshot::primary_window()).observe(save_to_disk(capture.path.clone()));
            capture.taken_at = Some(now);
        }
        Some(taken) if now - taken > 1.0 => {
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}
