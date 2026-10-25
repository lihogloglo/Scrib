//! `bevyrrowind view <cell>`: renders one cell with a free-fly camera.
//!
//! Static models load cooked from the bake cache (`baked://mesh.cook/...`)
//! when the profile has them; animated, skinned, light-carrying, and
//! un-baked models load from their NIF.
//!
//! With `--screenshot`, waits until every model and texture has loaded,
//! lets the renderer settle, saves a PNG, and exits. That's the automated
//! visual check for rendering changes. With `--bench`, runs an `sf_diag`
//! scenario (warm-up held until everything is loaded) and exits with its
//! verdict.

use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use bevy::asset::LoadState;
use bevy::asset::io::AssetSourceId;
use bevy::camera::Exposure;
use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use sf_diag_bevy::{GateStatus, ReadyGate};
use bevy::world_serialization::{WorldAsset, WorldAssetRoot};
use mw_assets::{CookedLodSettings, CookedMaterials, CookedModelRoot, CookedSpawned, MwAssetsPlugin};
use mw_world::{BaseKind, Placement};
use sf_bake::{Manifest, Store};
use sf_diag_bevy::{DiagScenario, FlyThroughCamera};
use sf_model::steps::MeshCookStep;
use sf_model_bevy::{BAKED_SOURCE, BakedManifest, baked_source};
use sf_sky_bevy::sf_sky::RenderPath;
use sf_sky_bevy::{RenderPathCamera, SkyPlugin, SkyRenderOptions};
use sf_vfs::Vfs;

pub struct ViewOptions {
    pub title: String,
    pub placements: Vec<Placement>,
    pub ambient: Option<[u8; 4]>,
    pub screenshot: Option<PathBuf>,
    /// `--screenshot-anyway`: shoot even if the readiness gate times out.
    pub screenshot_anyway: bool,
    /// Seconds to keep rendering after everything loaded, before capturing.
    pub settle: f32,
    /// Overrides the automatic camera: eye position and look-at target (meters).
    pub camera: Option<(Vec3, Vec3)>,
    pub calibration: mw_assets::LightCalibration,
    /// The bake cache, when cooked models should be used.
    pub baked: Option<Baked>,
    /// Debug view: tint cooked models by LOD level.
    pub lod_tint: bool,
    /// `--skinned`: skinned models load from their cooked product.
    pub skinned: Option<SkinnedView>,
    /// Ray-traced lighting (`--rt`) and DLSS (`--dlss`).
    pub render: crate::render::RenderSettings,
    /// Benchmark, window size and A/B switches.
    pub perf: crate::perf::Perf,
}

/// `--skinned` (scaffold, see `skinned_view`).
pub struct SkinnedView {
    /// The skeleton NIF; `None`: each model's own nodes.
    pub skeleton: Option<String>,
    /// `--animate`: an `sf_anim_bevy` rig playing the baked graph (`anim_view`).
    pub animate: bool,
}

/// A bake store and the profile manifest to read cooked products from.
pub struct Baked {
    pub store: Store,
    pub manifest: Manifest,
}

/// The cooked LOD level the ray tracer sees. Interiors are small enough
/// for full detail; exteriors will want a reduced level.
const RAY_TRACING_LOD: usize = 0;

#[derive(Resource)]
struct Scene {
    placements: Vec<Placement>,
    ambient: Option<[u8; 4]>,
    camera: Option<(Vec3, Vec3)>,
    /// Model paths to load cooked.
    cooked: HashSet<String>,
    /// Model paths to load as cooked skinned models (`--skinned`), and the
    /// skeleton NIF override.
    skinned: HashSet<String>,
    skeleton: Option<String>,
    bench: bool,
    /// LOD level cooked models give the ray tracer (ray-traced path only).
    ray_tracing_lod: Option<usize>,
    /// The camera's render path and upscaling are managed by `sf_sky_bevy`.
    managed: bool,
}

#[derive(Resource)]
struct Capture {
    path: PathBuf,
    settle: f32,
    taken_at: Option<f32>,
    /// Loaded and GPU pipelines compiled ([`sf_diag_bevy::ReadyGate`]); `ready_at` is when it passed.
    gate: ReadyGate,
    ready_at: Option<f32>,
    /// `--screenshot-anyway`: shoot on a gate timeout instead of failing.
    anyway: bool,
    failed: bool,
}

/// When everything (models, textures, cooked spawns) finished loading.
#[derive(Resource)]
pub(crate) struct Loading {
    started: Instant,
    ready_at: Option<f32>,
    /// Assets still pending when `ready_at` was set (non-zero: the wait gave up).
    pending: usize,
}

#[cfg(feature = "remote")]
impl Loading {
    /// Everything loaded (or the wait gave up).
    pub(crate) const fn ready(&self) -> bool {
        self.ready_at.is_some()
    }
}

pub fn run(vfs: Arc<Vfs>, options: ViewOptions) -> Result<(), Box<dyn std::error::Error>> {
    let started = Instant::now();
    let cooked = match &options.baked {
        Some(baked) => {
            let wanted: HashSet<&str> = options
                .placements
                .iter()
                // Lights attach at the model's `AttachLight` node, which
                // cooking flattens away; those keep the NIF path for now.
                .filter(|p| p.light.is_none() && p.kind != BaseKind::Actor)
                .filter_map(|p| p.model.as_deref())
                .collect();
            mw_bake::cooked_models(&baked.store, &baked.manifest, wanted)
        }
        None => HashSet::new(),
    };
    let mut anim_views = None;
    let (skinned, skeleton) = match (&options.baked, options.skinned) {
        (Some(baked), Some(view)) => {
            let wanted = options.placements.iter().filter_map(|p| p.model.as_deref());
            if view.animate {
                anim_views = Some(crate::anim_view::AnimViews::new(baked.store.clone(), baked.manifest.clone()));
            }
            (crate::skinned_view::skinned_models(&baked.store, &baked.manifest, wanted), view.skeleton)
        }
        _ => (HashSet::new(), None),
    };
    if !skinned.is_empty() {
        println!("{} skinned models load cooked (--skinned)", skinned.len());
    }
    let unique: HashSet<&str> = options.placements.iter().filter_map(|p| p.model.as_deref()).collect();
    println!("{} of {} models load cooked", cooked.len(), unique.len());

    let mut app = App::new();
    app.register_asset_source(AssetSourceId::Default, sf_vfs::bevy::asset_source(vfs.clone()));
    if let Some(baked) = options.baked {
        let extensions = BTreeMap::from([(MeshCookStep::ID.to_owned(), mw_bake::SFMESH_EXT.to_owned())]);
        app.register_asset_source(BAKED_SOURCE, baked_source(baked.store, BakedManifest::new(baked.manifest), extensions));
    }
    let render = options.render;
    let ray_traced = render.path == RenderPath::RayTraced;
    crate::render::RenderSettings::init(&mut app);
    crate::shipped_errors(&mut app);
    let mut perf = options.perf;
    let bench = perf.bench.is_some();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(perf.window(options.title.clone(), None)),
        ..WindowPlugin::default()
    }))
    .add_plugins((MwAssetsPlugin { vfs }, FreeCameraPlugin))
    .insert_resource(options.calibration)
    .insert_resource(CookedLodSettings { tint_levels: options.lod_tint, ..CookedLodSettings::default() })
    .insert_resource(Scene {
        placements: options.placements,
        ambient: options.ambient,
        camera: options.camera,
        cooked,
        skinned,
        skeleton,
        bench,
        ray_tracing_lod: ray_traced.then_some(RAY_TRACING_LOD),
        managed: render.managed(),
    })
    .insert_resource(Loading { started, ready_at: None, pending: 0 })
    .add_plugins(crate::skinned_view::SkinnedViewPlugin)
    .add_plugins(crate::anim_view::AnimViewPlugin)
    .add_systems(Startup, spawn)
    .add_systems(Update, track_loading);
    if let Some(views) = anim_views {
        app.insert_resource(views);
    }
    if render.managed() {
        // Interiors have no sky: the plugin only provides the render path
        // switch, DLSS and the point-light proxies here (the sun stays dark
        // without a `SkyCamera`).
        render.insert(&mut app);
        app.add_plugins(SkyPlugin { render_path: render.path, ..SkyPlugin::default() });
        render.apply_options(&mut app.world_mut().resource_mut::<SkyRenderOptions>());
    }

    crate::live_ab::add(&mut app, !bench && options.screenshot.is_none());
    if let Some((_, settings)) = &mut perf.bench {
        let models = if app.world().resource::<Scene>().cooked.is_empty() { "nif" } else { "cooked" };
        settings.meta.insert("models".to_owned(), models.to_owned());
        app.add_systems(Update, hold_until_loaded);
    } else if let Some(path) = options.screenshot {
        crate::pipeline_watch::add(&mut app);
        app.insert_resource(Capture {
            path,
            settle: options.settle,
            taken_at: None,
            gate: ReadyGate::new(crate::exterior::CAPTURE_STABLE_FRAMES, crate::exterior::CAPTURE_TIMEOUT_SECS),
            ready_at: None,
            anyway: options.screenshot_anyway,
            failed: false,
        })
        .add_systems(Update, capture);
    }
    if perf.off.has(crate::perf::Part::Lights) {
        app.add_systems(Update, despawn_point_lights);
    }
    perf.add_plugins(&mut app)?;
    app.run();
    Ok(())
}

fn spawn(
    mut commands: Commands,
    scene: Res<Scene>,
    assets: Res<AssetServer>,
    calibration: Res<mw_assets::LightCalibration>,
    mut anim_views: Option<ResMut<crate::anim_view::AnimViews>>,
) {
    let mut lo = Vec3::splat(f32::INFINITY);
    let mut hi = Vec3::splat(f32::NEG_INFINITY);
    let (mut nif, mut cooked, mut actors) = (0, 0, 0);

    for (index, placement) in scene.placements.iter().enumerate() {
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
        let mut root = None;
        if let Some(model) = &placement.model {
            let visibility = if placement.editor_only { Visibility::Hidden } else { Visibility::Inherited };
            let name = Name::new(placement.base_id.clone());
            root = Some(if let (true, Some(views)) = (scene.skinned.contains(model), anim_views.as_deref_mut()) {
                cooked += 1;
                crate::anim_view::spawn(&mut commands, &assets, views, model, scene.skeleton.as_deref(), (transform, visibility, name))
            } else if scene.skinned.contains(model) {
                cooked += 1;
                crate::skinned_view::spawn(&mut commands, &assets, model, scene.skeleton.as_deref(), (transform, visibility, name))
            } else if scene.cooked.contains(model) {
                cooked += 1;
                let handle = mw_assets::cooked::load(&assets, model, scene.ray_tracing_lod);
                let cooked = CookedModelRoot { model: handle, source: model.clone(), scale: t.scale };
                commands.spawn((cooked, transform, visibility, name)).id()
            } else {
                nif += 1;
                let handle: Handle<WorldAsset> = assets.load(model.clone());
                commands.spawn((WorldAssetRoot(handle), transform, visibility, name)).id()
            });
        }
        if let Some(light) = placement.light {
            debug!("light {} at {:.2}", placement.base_id, transform.translation);
            let seed = u32::try_from(index).unwrap_or_default();
            mw_assets::lights::spawn_light(&mut commands, &calibration, &light, root, transform, seed);
        }
    }
    info!("spawned {cooked} cooked and {nif} NIF models, skipped {actors} actors");

    if let Some([r, g, b, _]) = scene.ambient {
        commands.insert_resource(calibration.ambient([r, g, b]));
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
    let mut camera = commands.spawn((
        Camera3d::default(),
        Exposure { ev100: mw_assets::SCENE_EV100 },
        Transform::from_translation(eye).looking_at(target, Vec3::Y),
    ));
    if scene.managed {
        camera.insert(RenderPathCamera);
    }
    if scene.bench {
        camera.insert(FlyThroughCamera);
    } else {
        camera.insert(FreeCamera::default());
    }
}

/// Notes when every model, texture, and cooked spawn has finished loading.
fn track_loading(
    mut loading: ResMut<Loading>,
    server: Res<AssetServer>,
    nif_roots: Query<&WorldAssetRoot>,
    cooked_waiting: Query<(), (With<CookedModelRoot>, Without<CookedSpawned>)>,
    cooked_materials: Res<CookedMaterials>,
) {
    if loading.ready_at.is_some() {
        return;
    }
    let nif_pending = nif_roots
        .iter()
        .filter(|r| {
            !server.is_loaded_with_dependencies(&r.0) && !matches!(server.get_load_state(&r.0), Some(LoadState::Failed(_)))
        })
        .count();
    let pending = nif_pending + cooked_waiting.iter().count() + cooked_materials.pending_textures(&server);
    let elapsed = loading.started.elapsed().as_secs_f32();
    // Give up waiting after a minute; carry on with whatever is there.
    if pending == 0 || elapsed > 60.0 {
        info!("assets ready {elapsed:.2}s after start ({pending} still pending)");
        loading.ready_at = Some(elapsed);
        loading.pending = pending;
    }
}

fn hold_until_loaded(loading: Res<Loading>, scenario: Option<ResMut<DiagScenario>>) {
    if let Some(mut scenario) = scenario {
        scenario.hold = loading.ready_at.is_none();
    }
}

fn capture(
    mut commands: Commands,
    mut capture: ResMut<Capture>,
    loading: Res<Loading>,
    gpu: Res<crate::pipeline_watch::PendingDraws>,
    mut exit: MessageWriter<AppExit>,
) {
    if capture.failed {
        return;
    }
    let now = loading.started.elapsed().as_secs_f32();
    if capture.ready_at.is_none() {
        let cpu = loading.ready_at.is_some() && loading.pending == 0;
        match capture.gate.step(now, cpu, gpu.gpu()) {
            GateStatus::Waiting => return,
            GateStatus::Ready(r) => info!(
                "capture gate: ready after {} frames ({} since assets loaded), GPU pending at first check: {:?}",
                r.frames_waited, r.frames_after_cpu, r.first_pending
            ),
            GateStatus::TimedOut(r) => {
                error!(
                    "capture gate: timed out after {now:.1} s ({} frames): assets loaded {} ({} still pending), {} pipelines compiling, {} visible draws waiting (at first loaded frame: {:?}){}",
                    r.frames_waited,
                    loading.ready_at.is_some(),
                    loading.pending,
                    r.last_pending.pipelines,
                    r.last_pending.draws,
                    r.first_pending,
                    if capture.anyway { "; --screenshot-anyway: taking it" } else { "; no screenshot (pass --screenshot-anyway to force)" }
                );
                if !capture.anyway {
                    capture.failed = true;
                    exit.write(AppExit::from_code(1));
                    return;
                }
            }
        }
        capture.ready_at = Some(now);
    }
    let Some(loaded_at) = capture.ready_at else { return };
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

/// `--off lights`: no point lights.
fn despawn_point_lights(mut commands: Commands, lights: Query<Entity, Added<PointLight>>) {
    for light in &lights {
        commands.entity(light).despawn();
    }
}
