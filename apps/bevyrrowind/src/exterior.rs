//! `bevyrrowind view <exterior cell>`: the streamed exterior (ROADMAP M2).
//!
//! Everything comes from bake products: the world index streams placements
//! around the camera (`sf_world_bevy`), the terrain streams its clipmaps
//! (`sf_terrain_bevy`), the sky lights it (`sf_sky_bevy`). The content pack
//! supplies the parts only it knows (`PlacementSpawner`s):
//!
//! - cooked statics, doors, containers, activators and items: the
//!   framework's cooked LOD chains, with Morrowind's material mapping
//!   ([`mw_assets::cooked::cooked_material`]);
//! - models the bake didn't cook (animated, skinned): the runtime NIF path;
//! - lights: the NIF (lights attach at `AttachLight`) plus a point light;
//! - actors: dressed, idling characters (`sf_character_bevy`, `actors.rs`).
//!
//! Spec: `docs/specs/world-adapter.md`, `superfargoth/docs/specs/streaming.md`.

use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use bevy::asset::LoadState;
use bevy::asset::io::AssetSourceId;
use bevy::camera::visibility::RenderLayers;
use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};
use bevy::core_pipeline::prepass::{DepthPrepass, MotionVectorPrepass};
use bevy::render::occlusion_culling::OcclusionCulling;
use bevy::light::{CascadeShadowConfig, CascadeShadowConfigBuilder, ShadowFilteringMethod};
use bevy::pbr::ContactShadows;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::transform::TransformPlugin;
use bevy::world_serialization::{WorldAsset, WorldAssetRoot};
use glam::DVec3;
use mw_assets::{LightCalibration, MwAssetsPlugin};
use mw_world::{LightInfo, ids};
use sf_bake::{Manifest, Store};
use sf_coords::WorldPos;
use sf_diag_bevy::{DiagCounters, DiagScenario, FlyThroughPose};
use sf_model::steps::MeshCookStep;
use sf_model_bevy::{BAKED_SOURCE, BakedManifest, baked_source};
use sf_sky_bevy::sf_sky::{SkyClock, SkySettings};
use sf_sky_bevy::{MoonLight, SkyCamera, SkyPlugin, SkyRenderOptions, SunLight, TemporalHistory};
use sf_weather_bevy::{WeatherOverrides, WeatherPlugin, WeatherSite, WeatherSystems};
use sf_terrain_bevy::sf_terrain::bake::BakedTerrain;
use sf_terrain_bevy::{GroundBlendPlugin, TerrainCamera, TerrainDebug, TerrainOrigin, TerrainPlugin, TerrainSettings, TerrainStats, TerrainSystems, TerrainWorld};
use sf_vfs::Vfs;
use sf_world::bake::BakedIndex;
use sf_world::{IndexedPlacement, PlacementClass};

use crate::perf::{Off, Part};
use sf_world_bevy::big_space::prelude::{CellCoord, FloatingOrigin, Grid};
use sf_world_bevy::{
    ActiveWorldspace, FadeTransition, GroundBlend, GroundBlendModels, LoadedModel, ModelMaterials, ModelTemplate, PlacementSpawner, PlacementSpawners, SpawnCx,
    StaticSpawner, Streaming, StreamingAnchor, StreamingConfig, StreamingDebug, StreamingSystems, SubtreeLayers, SubtreeRange, TemplateLod,
    WorldIndexSource, WorldStreamingPlugin, WorldspaceRegistry, WorldspaceRoot, spawn_template, to_grid,
};

/// Settings of an exterior run.
#[expect(clippy::struct_excessive_bools, reason = "independent command-line flags")]
pub struct ExteriorOptions {
    pub title: String,
    /// Camera eye and target, world meters (engine axes).
    pub eye: DVec3,
    pub target: DVec3,
    pub screenshot: Option<PathBuf>,
    pub settle: f32,
    pub store: Store,
    pub manifest: Manifest,
    pub lod_tint: bool,
    pub terrain_lod: bool,
    pub wireframe: bool,
    pub rings: bool,
    /// Physics collider wireframes at start (`--physics-debug`).
    pub physics_debug: bool,
    /// Ring load radii overrides (R1, R2), meters; unload keeps the default margin.
    pub near_ring: Option<f32>,
    pub far_ring: Option<f32>,
    /// Spawn no point lights (calibration runs).
    pub no_lights: bool,
    /// Tint far-field proxies by level (L1 green, L2 blue, L3 purple).
    pub far_tint: bool,
    /// Depth + motion-vector prepass with occlusion culling (A/B experiment).
    pub prepass: bool,
    pub hour: f64,
    /// Game day for the clock (`--day`; moon phases).
    pub day: f64,
    /// Drawn sun disk diameter, degrees (`--sun-size`).
    pub sun_size: Option<f32>,
    /// A pinned cloud genus (`--clouds`) instead of the weather's clouds.
    pub clouds: Option<CloudPreset>,
    /// Fixed haze and dust (`--haze`, `--dust`) instead of the weather's (weather.md N1/N2).
    pub haze: Option<f32>,
    pub dust: Option<f32>,
    /// Fog multiplier (`--fog`) on the weather's fog (weather.md "Fog", F2).
    pub fog: Option<f32>,
    /// The live weather (M6 WE3): records, state and the grid's climates.
    pub weather: Option<crate::weather::WeatherSetup>,
    /// Quality preset (`--quality`): view-distance bias and size culling.
    pub quality: Quality,
    /// Benchmark, window size and A/B switches.
    pub perf: crate::perf::Perf,
    /// Light parameters of every `LIGH` record (lowercase id).
    pub lights: HashMap<String, LightInfo>,
    pub calibration: LightCalibration,
    /// Ray-traced lighting (`--rt`) and DLSS (`--dlss`).
    pub render: crate::render::RenderSettings,
    /// Worldspaces, door links and interior environments (door travel).
    pub world: crate::doors::WorldLinks,
    /// `--use-door`: cross the nearest door once the start has settled.
    pub use_door: u32,
    /// `--activate N`: activate the crosshair target N times (`scripts.rs`).
    pub activate: u32,
    /// `--portals`: the portal spike's links (scaffold `portal_spike`).
    pub portals: Option<crate::portal_spike::PortalLinks>,
    /// Loaded records: actor placements spawn as dressed characters
    /// (`sf_character_bevy`; `--off actors` drops them).
    pub actors: Option<Arc<sf_records::RecordDb>>,
    /// `--bodies`: smooth (default) or vanilla segmented bodies.
    pub bodies: sf_character_bevy::SmoothBodies,
    /// `--cloth`: robes and skirts authored (default) or simulated (opt-in).
    pub cloth: mw_physics::ClothMode,
    /// The worldspace the camera starts in (the exterior, or an interior).
    pub worldspace: sf_world::WorldspaceId,
    /// The water's shore and region inputs (`water.rs`, W3).
    pub water: crate::water::WaterInputs,
    /// A synthetic test level (`level.rs`, TS1): `worldspace` is its own.
    pub level: Option<crate::level::LevelSetup>,
}

#[derive(Resource)]
struct Start {
    worldspace: sf_world::WorldspaceId,
    eye: DVec3,
    target: DVec3,
    bench: bool,
    prepass: bool,
    /// Screen-space contact shadows (`None`: off; `--off contactshadows`).
    contact: Option<ContactShadows>,
    shadow_filter: ShadowFilteringMethod,
}

#[derive(Resource)]
struct Capture {
    path: PathBuf,
    settle: f32,
}

#[derive(Resource)]
struct Started(Instant);

/// Inserted once the start has fully streamed in (`--use-door` and
/// `BEVYRROWIND_AUTO` wait for it; `--use-door` removes it for each trip).
#[derive(Resource)]
pub struct Settled;

/// The streaming camera (visual anchor, floating origin).
#[derive(Component)]
pub(crate) struct ExteriorCamera;

/// The gameplay anchor (R0), following the camera.
#[derive(Component)]
struct Player;

/// Scaffold `SeaPlaneScaffold`: a flat sea at Morrowind's sea level, so
/// coasts read on screen. Removal path: `sf_water` (M6) replaces it.
#[derive(Component)]
struct SeaPlaneScaffold;

#[allow(clippy::too_many_lines)] // one linear app setup
pub fn run(vfs: Arc<Vfs>, options: ExteriorOptions) -> Result<(), Box<dyn std::error::Error>> {
    let ext = ids::exterior();
    let manifest = Arc::new(options.manifest);
    if manifest.get(sf_world::bake::DIR_STEP, ext.as_str()).is_none() {
        return Err("no baked world index for the exterior: run `bevyrrowind bake`".into());
    }
    let terrain = Arc::new(BakedTerrain::new(options.store.clone(), &manifest, ext.clone()));
    if terrain.tiles().is_empty() {
        return Err("no baked terrain for the exterior: run `bevyrrowind bake`".into());
    }

    let mut app = App::new();
    let render = options.render;
    crate::render::RenderSettings::init(&mut app);
    // Watched and swappable in dev builds (M5 A4 hot reload).
    let vfs_source = crate::hot_reload::asset_source(&mut app, vfs.clone());
    app.register_asset_source(AssetSourceId::Default, vfs_source);
    let mut extensions = BTreeMap::from([(MeshCookStep::ID.to_owned(), mw_bake::SFMESH_EXT.to_owned())]);
    extensions.extend(sf_lod_bevy::baked_extensions());
    // The far field (far-field.md) when the bake has it and `--off farfield` doesn't drop it.
    let far_field = manifest.get(sf_lod::products::DIR_STEP, ext.as_str()).is_some() && !options.perf.off.has(Part::FarField);
    app.register_asset_source(
        BAKED_SOURCE,
        baked_source(options.store.clone(), BakedManifest::new((*manifest).clone()), extensions),
    );
    let perf = options.perf;
    let bench = perf.bench.is_some();
    crate::actors::add_world(&mut app, options.actors.clone(), perf.off.has(Part::Actors), &vfs, &options.store, &manifest, options.cloth);
    app.insert_resource(sf_character_bevy::CharacterSettings { bodies: options.bodies });
    if let (Some(db), false) = (options.actors.clone(), perf.off.has(Part::Actors) || perf.off.has(Part::Walkers)) {
        crate::walkers::add(&mut app, db, options.store.clone(), (*manifest).clone(), !perf.off.has(Part::Physics));
    }
    if !perf.off.has(Part::Physics) {
        crate::physics::add(&mut app, options.store.clone(), (*manifest).clone(), options.physics_debug, options.actors.clone());
    }
    // The quality preset scales LOD selection, size culling and the
    // far-field handoff with one bias (`--off sizecull`: no size culling).
    let screen = options.quality.screen(!perf.off.has(Part::SizeCull));
    let mut far_settings = sf_lod_bevy::FarFieldSettings::default();
    far_settings.levels.screen.bias = screen.bias;
    // The far field sets its own rings; `--near-ring`/`--far-ring` still override them.
    let base = if far_field {
        sf_lod_bevy::far_streaming_config(StreamingConfig::default(), &far_settings.levels)
    } else {
        StreamingConfig::default()
    };
    let mut streaming = streaming_config(base, options.near_ring, options.far_ring);
    streaming.lod = screen;
    app.add_plugins(
        DefaultPlugins
            .build()
            // big_space propagates transforms.
            .disable::<TransformPlugin>()
            .set(WindowPlugin { primary_window: Some(perf.window(options.title.clone(), Some((1280, 720)))), ..WindowPlugin::default() }),
    )
    .add_plugins((MwAssetsPlugin { vfs: vfs.clone() }, FreeCameraPlugin))
    .add_plugins(WorldStreamingPlugin { config: streaming })
    .insert_resource(WorldIndexSource(Arc::new(BakedIndex::new(options.store.clone(), manifest.clone()))))
    .insert_resource(ActiveWorldspace(Some(options.worldspace.clone())))
    .insert_resource(StreamingDebug { enabled: options.rings, ..StreamingDebug::default() })
    .insert_resource(options.calibration);
    if let Some(level) = options.level {
        crate::level::add(&mut app, level);
    }

    // Cooked materials map exactly like the NIF path's.
    let mapper_vfs = vfs.clone();
    app.insert_resource(ModelMaterials(Arc::new(move |m, server| mw_assets::cooked::cooked_material(m, &mapper_vfs, server))));

    // The night exposure floor and ambient (LightCalibration, owner option A).
    let mut settings = SkySettings::default();
    options.calibration.apply_sky(&mut settings);
    crate::night_sky::apply(&mut settings);
    if let Some(degrees) = options.sun_size {
        settings.sun_disk.angular_size = degrees.to_radians();
    }
    let mut clock = SkyClock::at(options.day, options.hour, &settings.calendar);
    clock.paused = options.screenshot.is_some() || bench;
    render.insert(&mut app);
    let off_parts = &perf.off;
    let contact_shadows = !off_parts.has(Part::ContactShadows) && options.quality.contact_shadows().is_some();
    app.insert_resource(sf_sky_bevy::PointShadowBudget { contact_shadows, ..render.point_shadow_budget(options.quality.point_shadows()) });
    app.add_plugins(SkyPlugin { settings, clock, render_path: render.path }).insert_resource(SkyRenderOptions {
        ray_tracing_denoiser: render.denoiser,
        cascades: sun_cascades(options.quality, &far_settings.levels, off_parts.has(Part::ShadowTune), off_parts.has(Part::ShadowSplits)),
        sun_shadow: sun_shadow(off_parts.has(Part::ShadowTune), contact_shadows),
        ..SkyRenderOptions::default()
    });
    // Shadows F3b: the far static sun map (objects' shadows past the cascades, on the terrain;
    // weather.md "Sun cascades and the far static map"). `--off farshadows` turns it off too.
    app.add_plugins(sf_sky_bevy::FarShadowPlugin)
        .add_observer(no_far_shadow_terrain)
        .add_observer(no_far_shadow_proxy)
        .add_systems(Update, sun_shadow_input);
    if off_parts.has(Part::FarShadows) {
        app.insert_resource(sf_sky_bevy::FarShadowConfig(None));
    }
    // M6 WE3: the weather (`sf_weather_bevy`) drives haze, dust, cloud cover, the clouds, the
    // fog and the wind from the camera's region; `--haze`/`--dust`/`--clouds` pin terms, `--fog`
    // scales the fog. Live: the C key and BRP `bevyrrowind/set weather` (`weather::force_live`).
    if let Some(weather) = options.weather {
        let overrides = WeatherOverrides {
            haze: options.haze,
            dust: options.dust,
            clouds: options.clouds.map(pinned_clouds),
            fog_scale: options.fog,
        };
        // WE4c: the world weather map over the region raster (`sf_weather_bevy::WorldWeatherMap`):
        // distant regions show their own clouds; the camera's site comes from it.
        let raster = weather.raster();
        // WE3 part 2b: rain, snow, ash and blight fall from the frame (`sf_precip_bevy`).
        app.add_plugins((WeatherPlugin { book: weather.book, state: weather.state }, sf_weather_bevy::sf_precip_bevy::PrecipPlugin))
            .insert_resource(overrides);
        if let Some(raster) = raster {
            app.insert_resource(sf_weather_bevy::WorldWeatherMap::new(raster))
                .add_systems(Update, weather_site.before(WeatherSystems::Evaluate));
        }
    }
    // weather.md "Wind" (WD1): one wind field; fog and cloud drift read it.
    app.add_plugins(sf_wind_bevy::WindPlugin::default())
        .add_systems(PostUpdate, wind_origin.after(sync_origin).before(sf_wind_bevy::WindSystems::Advance));
    // The cascades keep Bevy's 2048² maps: 4096² measured +1.4 ms at 1440p
    // (weather.md A1, 2026-10-12); shadows F3 sharpens them with a 250 m range instead.
    // weather.md "Clouds" and "Fog": on by default; `--off clouds` or `--off fog` drops
    // one (the node runs for the other), `--off sky` both.
    let (clouds_on, fog_on) = (!perf.off.has(Part::Clouds), !perf.off.has(Part::Fog));
    if (clouds_on || fog_on) && !perf.off.has(Part::Sky) {
        let fog = sf_atmos_bevy::sf_atmos::FogSettings::default();
        app.add_plugins(sf_atmos_bevy::CloudsPlugin { settings: cloud_settings(), fog, seed: 0x5EED_C10D })
            .add_systems(Update, cloud_shadow_light);
        let mut clouds = app.world_mut().resource_mut::<sf_atmos_bevy::Clouds>();
        clouds.enabled = clouds_on;
        clouds.settings.quality.map_per_sample = options.quality.cloud_map_per_sample();
        let mut fog = app.world_mut().resource_mut::<sf_atmos_bevy::Fog>();
        fog.enabled = fog_on;
        // F3: the froxel volume (High) near the camera; `--off froxels` keeps the
        // analytic layers only (the Low preset). Shafts past the cascades come from
        // the terrain's shadow-height map.
        if perf.off.has(Part::Froxels) {
            fog.froxels = None;
        }
        // weather.md "Temporal sampling": `--off fogdither` is the old shared jitter, `--off fogjitter` none.
        if let Some(f) = fog.froxels.as_mut() {
            use sf_atmos_bevy::sf_atmos::FroxelJitter;
            if perf.off.has(Part::FogJitter) {
                f.jitter = FroxelJitter::Off;
            } else if perf.off.has(Part::FogDither) {
                f.jitter = FroxelJitter::Shared;
            }
        }
        if !perf.off.has(Part::FarShadows) {
            app.add_systems(Update, fog_shadow_heights);
        }
        // Blended and additive NIF surfaces draw after the fog composite: they
        // fog themselves from the froxels (`--off transfog` is the A/B).
        if fog_on && !perf.off.has(Part::TransFog) {
            crate::transparent_fog::add(&mut app);
        }
    }

    if far_field {
        app.add_plugins(sf_lod_bevy::FarFieldPlugin { settings: far_settings })
            .insert_resource(sf_lod_bevy::FarFieldDebug { level_tint: options.far_tint })
            .insert_resource(far_worldspaces(&manifest))
            .add_systems(Update, far_lights_night);
        // weather.md tier 2: the terrain's far shadow map shadows proxies and
        // impostors too (`--off farshadows`: neither gets it).
        if !perf.off.has(Part::FarShadows) {
            app.add_systems(Update, far_shadow_input);
        }
    }

    app.add_plugins(TerrainPlugin { settings: TerrainSettings::default() })
        .insert_resource(TerrainWorld { baked: terrain })
        .insert_resource(TerrainDebug { lod_colors: options.terrain_lod, wireframe: options.wireframe, no_far_shadows: perf.off.has(Part::FarShadows), ..TerrainDebug::default() });
    // Object-terrain blending (terrain.md): rocks, roots and foundations
    // blend into the ground. Raster only: Solari's RT scene reads plain
    // `StandardMaterial`s, so blended parts would drop out of it.
    if !perf.off.has(Part::TerrainBlend) && render.path != sf_sky_bevy::sf_sky::RenderPath::RayTraced {
        app.insert_resource(GroundBlendModels(Arc::new(mw_assets::ground_blend_band)))
            .add_plugins(GroundBlendPlugin::<GroundBlend> { read: |g| (&g.worldspace, g.band) });
    }

    app.insert_resource(Spawners {
        lights: Arc::new(if options.no_lights { HashMap::new() } else { options.lights }),
        lod_tint: options.lod_tint,
    })
        .insert_resource(Start { worldspace: options.worldspace, eye: options.eye, target: options.target, bench: bench || options.screenshot.is_some(),
            prepass: options.prepass,
            contact: if perf.off.has(Part::ContactShadows) { None } else { options.quality.contact_shadows() },
            shadow_filter: options.quality.shadow_filter(),
        })
        .insert_resource(Started(Instant::now()))
        // Chained: both reserve the worldspace root through `Commands`; unordered,
        // one could queue `ChildOf(root)` ahead of the root's spawn (a panic with
        // the `solari` feature, whose Startup systems change the order).
        .add_systems(Startup, (setup, setup_spawners).chain())
        .add_systems(Update, (follow_pose, follow_player).chain().before(StreamingSystems::Drive))
        .add_systems(Update, (light_layers, status))
        .add_systems(
            PostUpdate,
            sync_origin
                .after(TransformSystems::Propagate)
                .before(TerrainSystems::Update)
                .before(sf_sky_bevy::SkySystems::Origin)
                .before(sf_atmos_bevy::CloudSystems::Drive),
        );

    crate::water::add(&mut app, options.water, options.quality, &options.world.worldspaces);
    crate::doors::add(&mut app, options.world, options.use_door);
    crate::scripts::add(&mut app, vfs.clone(), options.activate);
    if let Some(links) = options.portals {
        crate::portal_spike::add(&mut app, links);
    }
    // `--use-door` and the `BEVYRROWIND_AUTO` dev hook (physics.rs) wait for it.
    app.add_systems(Update, mark_settled);
    crate::live_ab::add(&mut app, !bench && options.screenshot.is_none());
    if bench {
        app.add_systems(Update, (hold_warmup, counters)).add_systems(Startup, |mut s: ResMut<Streaming>| s.begin_burst());
    } else if let Some(path) = options.screenshot {
        app.insert_resource(Capture { path, settle: options.settle })
            .add_systems(Update, capture)
            .add_systems(Startup, |mut s: ResMut<Streaming>| s.begin_burst());
    }
    perf.add_plugins(&mut app)?;
    app.run();
    Ok(())
}

// -- spawners ----------------------------------------------------------------------

#[derive(Resource, Clone)]
struct Spawners {
    lights: Arc<HashMap<String, LightInfo>>,
    lod_tint: bool,
}

/// Models the bake didn't cook: the runtime NIF path (animated, skinned,
/// particles). Actors wait for M3.
struct NifSpawner {
    lights: Arc<HashMap<String, LightInfo>>,
}

impl NifSpawner {
    fn spawn_nif(world: &mut World, cx: &SpawnCx<'_>, model: &str) -> Entity {
        let handle = world.resource::<AssetServer>().load(model.to_owned());
        // The placement's range on every mesh once the scene spawns (B1).
        world
            .spawn((WorldAssetRoot(handle), cx.local, Visibility::Inherited, ChildOf(cx.parent), SubtreeRange::of(cx), SubtreeLayers::of(cx)))
            .id()
    }

    /// A light at the placement (moved to the model's `AttachLight` node
    /// once the model spawns), lighting objects and terrain.
    fn spawn_light(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement, root: Option<Entity>) -> u32 {
        let Some(info) = self.lights.get(&placement.placement.base.id.as_str().to_ascii_lowercase()) else { return 0 };
        let calibration = *world.resource::<LightCalibration>();
        #[allow(clippy::cast_possible_truncation)]
        let seed = placement.placement.id.0 as u32;
        let mut commands = world.commands();
        let light = mw_assets::lights::spawn_light(&mut commands, &calibration, info, root, cx.local, seed);
        // The worldspace's layer: lights reach only their own worldspace's meshes.
        commands.entity(light).insert(cx.layers.clone());
        if root.is_none() {
            commands.entity(light).insert(ChildOf(cx.parent));
        }
        world.flush();
        1
    }
}

impl PlacementSpawner for NifSpawner {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        let p = &placement.placement;
        if p.class == PlacementClass::Actor {
            return 0;
        }
        let Some(model) = &p.model else { return 0 };
        let root = Self::spawn_nif(world, cx, model.as_str());
        let lights = if p.class == PlacementClass::Light { self.spawn_light(world, cx, placement, Some(root)) } else { 0 };
        1 + lights
    }

    /// The NIF loads before the level reveals (B3).
    fn prepare(&self, assets: &AssetServer, placement: &IndexedPlacement, _: Option<&LoadedModel>) -> Option<UntypedHandle> {
        let p = &placement.placement;
        if p.class == PlacementClass::Actor {
            return None;
        }
        p.model.as_ref().map(|m| assets.load::<WorldAsset>(m.as_str().to_owned()).untyped())
    }
}

/// Lights: the NIF (cooking flattens the `AttachLight` node away) and the light.
struct LightSpawner(Arc<NifSpawner>);

impl PlacementSpawner for LightSpawner {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        let root = placement.placement.model.as_ref().map(|m| NifSpawner::spawn_nif(world, cx, m.as_str()));
        u32::from(root.is_some()) + self.0.spawn_light(world, cx, placement, root)
    }

    fn needs_model(&self, _: &IndexedPlacement) -> bool {
        false
    }

    fn prepare(&self, assets: &AssetServer, placement: &IndexedPlacement, model: Option<&LoadedModel>) -> Option<UntypedHandle> {
        self.0.prepare(assets, placement, model)
    }
}

/// Debug view (`--lod-tint`): cooked levels drawn in flat, unlit colors.
struct TintSpawner {
    levels: Arc<[Handle<StandardMaterial>]>,
}

impl PlacementSpawner for TintSpawner {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        let Some(LoadedModel::Cooked(template)) = cx.model else { return StaticSpawner.spawn(world, cx, placement) };
        let tinted = ModelTemplate {
            lods: template
                .lods
                .iter()
                .enumerate()
                .map(|(level, lod)| TemplateLod {
                    error: lod.error,
                    parts: lod.parts.iter().map(|(mesh, _)| (mesh.clone(), self.levels[level.min(self.levels.len() - 1)].clone())).collect(),
                })
                .collect(),
            aabb: template.aabb,
            ground_blend: None,
        };
        spawn_template(world, cx, &tinted)
    }
}

/// LOD tint colors: green, yellow, orange, red, magenta (as the interior viewer).
const LEVEL_TINTS: [Color; 5] = [
    Color::srgb(0.3, 1.0, 0.3),
    Color::srgb(1.0, 1.0, 0.2),
    Color::srgb(1.0, 0.55, 0.1),
    Color::srgb(1.0, 0.2, 0.2),
    Color::srgb(1.0, 0.2, 1.0),
];

fn setup_spawners(
    mut commands: Commands,
    config: Res<Spawners>,
    off: Res<Off>,
    mut registry: ResMut<WorldspaceRegistry>,
    characters: Option<Res<sf_character_bevy::Characters>>,
    walkers: Option<Res<crate::walkers::MwGround>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Reserves the exterior root (see the chain in `run`).
    registry.ensure(&mut commands, &ids::exterior());
    let nif = Arc::new(NifSpawner { lights: config.lights.clone() });
    let mut spawners = PlacementSpawners::default();
    spawners
        .set(PlacementClass::Light, Arc::new(LightSpawner(nif.clone())))
        .set_uncooked(Some(nif));
    if characters.is_some() {
        spawners.set(PlacementClass::Actor, Arc::new(sf_character_bevy::CharacterSpawner));
    } else {
        spawners.clear(PlacementClass::Actor);
    }
    if config.lod_tint {
        let levels: Arc<[Handle<StandardMaterial>]> = LEVEL_TINTS
            .iter()
            .map(|c| materials.add(StandardMaterial { base_color: *c, unlit: true, ..StandardMaterial::default() }))
            .collect();
        let tint: Arc<dyn PlacementSpawner> = Arc::new(TintSpawner { levels });
        for class in [PlacementClass::Static, PlacementClass::Dynamic, PlacementClass::Door] {
            spawners.set(class, tint.clone());
        }
        spawners.set_default(Some(tint));
    }
    crate::perf::apply_off(&mut spawners, &off);
    if walkers.is_some() {
        crate::walkers::hook_colliders(&mut spawners);
    }
    commands.insert_resource(spawners);
}

// -- scene ----------------------------------------------------------------------------

fn setup(
    mut commands: Commands,
    mut registry: ResMut<WorldspaceRegistry>,
    start: Res<Start>,
    off: Res<Off>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let entry = registry.ensure(&mut commands, &start.worldspace);
    let (cell, mut transform) = to_grid(WorldPos::from_f64(start.eye));
    #[allow(clippy::cast_possible_truncation)]
    let look = (start.target - start.eye).as_vec3();
    transform.rotation = Transform::default().looking_to(look, Vec3::Y).rotation;
    // No depth prepass by default: in Bevy 0.20 an opaque mesh's depth-only
    // prepass has no fragment stage, so it ignores the `VisibilityRange` dither
    // and its depth blocks whatever should draw in the dithered pixels (holes at
    // every LOD and far-field crossfade). Measured: the prepass also cost 0.4 ms
    // here. With motion vectors the prepass gets a fragment stage that applies
    // the dither, which is how DLSS (`--dlss`) and `--prepass` run.
    let layers = RenderLayers::from_layers(&[0, entry.layer]);
    let mut camera = commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: 70f32.to_radians(),
            near: 0.1,
            far: 60_000.0,
            ..PerspectiveProjection::default()
        }),
        cell,
        transform,
        FloatingOrigin,
        ExteriorCamera,
        crate::doors::DoorUser,
        StreamingAnchor::visual(),
        layers.clone(),
        ChildOf(entry.root),
    ));
    let camera_id = camera.id();
    // A/B runs (`--off`) leave the terrain or the sky out entirely.
    // Only the exterior has land (`doors::worldspace_look` keeps this as the
    // camera changes worldspace).
    if !off.has(Part::Terrain) && start.worldspace == ids::exterior() {
        camera.insert(TerrainCamera);
    }
    if !off.has(Part::Sky) {
        camera.insert(SkyCamera { environment_light: !off.has(Part::Envmap), ..SkyCamera::default() });
        if !off.has(Part::Clouds) || !off.has(Part::Fog) {
            camera.insert(sf_atmos_bevy::CloudCamera);
        }
    }
    if start.prepass {
        camera.insert((DepthPrepass, MotionVectorPrepass, OcclusionCulling));
    }
    // weather.md "Shadow bias and contact shadows" (shadows F1): contact shadows ray-march
    // the depth prepass, which needs motion vectors here so it applies the LOD dither
    // (see the note above).
    camera.insert(start.shadow_filter);
    if let Some(contact) = start.contact {
        camera.insert((contact, DepthPrepass, MotionVectorPrepass));
    }
    if !start.bench {
        camera.insert(FreeCamera { walk_speed: 12.0, run_speed: 80.0, ..FreeCamera::default() });
    }
    // Benchmark A/B (perf.md "Frame pacing"): render at a fixed size offscreen
    // and show it scaled to the window, so pixel count and window size vary
    // independently.
    if let Some((w, h)) = crate::perf::render_size() {
        use bevy::camera::RenderTarget;
        use bevy::render::render_resource::TextureFormat;
        use bevy::ui::IsDefaultUiCamera;
        let image = images.add(Image::new_target_texture(w, h, TextureFormat::Bgra8UnormSrgb, None));
        commands.entity(camera_id).insert(RenderTarget::Image(image.clone().into()));
        commands.spawn((Camera2d, Camera { order: 1, ..Camera::default() }, IsDefaultUiCamera));
        commands.spawn((Node { width: percent(100), height: percent(100), ..Node::default() }, ImageNode::new(image)));
    }
    commands.spawn((cell, transform, StreamingAnchor::gameplay(), Player, ChildOf(entry.root)));

    // The sea scaffold: 64 km square under the camera, at sea level, drawn
    // in the exterior's layer (whichever worldspace the camera starts in).
    // Opaque (F2): the fog composite runs before the transparent pass and fogs
    // by the depth buffer, so a blended sea showed unfogged over fogged sand.
    // Transparents get fog with F3's `sf_atmos::fog` function; `sf_water` (W2)
    // replaces the scaffold. No shadow caster: nothing lies under it.
    let exterior_layer = registry.ensure(&mut commands, &ids::exterior()).layer;
    // W2: only behind `--water scaffold` (A/B until WA-1 passes; water.rs draws the ocean).
    if crate::water::mode() != crate::water::WaterMode::Scaffold {
        return;
    }
    commands.spawn((
        SeaPlaneScaffold,
        Mesh3d(meshes.add(Plane3d::default().mesh().size(64_000.0, 64_000.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.05, 0.16, 0.2),
            perceptual_roughness: 0.08,
            reflectance: 0.4,
            ..StandardMaterial::default()
        })),
        bevy::light::NotShadowCaster,
        Transform::IDENTITY,
        GlobalTransform::IDENTITY,
        RenderLayers::layer(exterior_layer),
    ));
}

/// The floating origin's cell drives the terrain's render origin and the
/// atmosphere's sea level; the sea scaffold follows the camera. Runs after
/// `big_space`'s propagation, before the terrain update, the sky's atmosphere
/// placement and the clouds' drive, so all of them move in the same frame as
/// the origin (a frame late, the clouds jumped by a cell at every shift). The origin is that of the worldspace
/// the terrain camera stands in (the exterior's, also when a portal camera
/// looks out of an interior: `--portals`), else the active worldspace's.
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
fn sync_origin(
    roots: Query<(&WorldspaceRoot, &Grid)>,
    active: Res<ActiveWorldspace>,
    camera: Query<&GlobalTransform, With<ExteriorCamera>>,
    terrain_camera: Query<(&GlobalTransform, &ChildOf), (With<TerrainCamera>, Without<SeaPlaneScaffold>)>,
    mut terrain_origin: ResMut<TerrainOrigin>,
    (mut sky, mut history, cloud_origin): (ResMut<SkyRenderOptions>, ResMut<TemporalHistory>, Option<ResMut<sf_atmos_bevy::CloudsOrigin>>),
    mut sea: Query<&mut GlobalTransform, (With<SeaPlaneScaffold>, Without<ExteriorCamera>)>,
    counters: Option<Res<DiagCounters>>,
) {
    let terrain_cam = terrain_camera.iter().next();
    let by_camera = terrain_cam.and_then(|(_, parent)| roots.get(parent.parent()).ok());
    let by_active = || active.0.as_ref().and_then(|ws| roots.iter().find(|(r, _)| r.id == *ws));
    let Some((_, grid)) = by_camera.or_else(by_active) else { return };
    let c = grid.local_floating_origin().cell();
    let origin = WorldPos::new(IVec3::new(c.x, c.y, c.z), Vec3::ZERO);
    if terrain_origin.0 != origin {
        terrain_origin.0 = origin;
        // The terrain's motion vectors pair this frame's positions with last
        // frame's view: across a shift they are off by a cell, so temporal
        // upscalers start over (one frame of plain anti-aliasing).
        history.reset = true;
        // big_space re-expresses every resident transform: a hitch suspect.
        if let Some(c) = &counters {
            c.add_named("world/origin_shifts", "count", 1.0);
        }
    }
    #[allow(clippy::cast_precision_loss)]
    let sea_level = -(c.y as f32) * sf_coords::GRID_CELL;
    // The clouds read the origin as a texture phase (they never move on shifts).
    if let Some(mut o) = cloud_origin {
        let edge = f64::from(sf_coords::GRID_CELL);
        let abs = DVec3::new(f64::from(c.x) * edge, f64::from(c.y) * edge, f64::from(c.z) * edge);
        if o.0 != abs {
            o.0 = abs;
        }
    }
    if (sky.sea_level - sea_level).abs() > f32::EPSILON {
        sky.sea_level = sea_level;
    }
    let cam = terrain_cam.map(|(g, _)| g).or_else(|| camera.single().ok());
    if let (Some(cam), Ok(mut t)) = (cam, sea.single_mut()) {
        let p = cam.translation();
        // Snapped, so the sea's own shading never swims.
        let snap = |v: f32| (v / 256.0).round() * 256.0;
        *t = GlobalTransform::from_translation(Vec3::new(snap(p.x), sea_level, snap(p.z)));
    }
}

type SkyLightsWithoutLayers = (Or<(With<SunLight>, With<MoonLight>)>, Without<RenderLayers>);
type CameraNotPlayer = (With<ExteriorCamera>, Without<Player>);

/// The sky's sun and moon light (and shadow) the exterior's render layer too.
fn light_layers(
    mut commands: Commands,
    registry: Res<WorldspaceRegistry>,
    lights: Query<Entity, SkyLightsWithoutLayers>,
) {
    let Some(entry) = registry.get(&ids::exterior()) else { return };
    for e in &lights {
        commands.entity(e).insert(RenderLayers::from_layers(&[0, entry.layer]));
    }
}

/// Fly-through: the scenario's pose (f64) as grid coordinates.
fn follow_pose(pose: Option<Res<FlyThroughPose>>, mut cams: Query<(&mut CellCoord, &mut Transform), With<ExteriorCamera>>) {
    let Some(pose) = pose.and_then(|p| p.0) else { return };
    let Ok((mut cell, mut transform)) = cams.single_mut() else { return };
    let (c, mut t) = to_grid(WorldPos::from_f64(pose.position));
    t.rotation = pose.rotation.as_quat();
    *cell = c;
    *transform = t;
}

/// The R0 anchor walks with the camera, into other worldspaces too.
fn follow_player(
    mut commands: Commands,
    cams: Query<(&CellCoord, &Transform, &ChildOf), CameraNotPlayer>,
    mut players: Query<(Entity, &mut CellCoord, &mut Transform, &ChildOf), With<Player>>,
) {
    if let (Ok((c, t, root)), Ok((player, mut pc, mut pt, player_root))) = (cams.single(), players.single_mut()) {
        *pc = *c;
        *pt = *t;
        if player_root.parent() != root.parent() {
            commands.entity(player).insert(ChildOf(root.parent()));
        }
    }
}

/// `--use-door`: marks the start as streamed in.
#[allow(clippy::too_many_arguments)] // a system's parameters
fn mark_settled(
    mut commands: Commands,
    streaming: Res<Streaming>,
    terrain: Res<TerrainStats>,
    server: Res<AssetServer>,
    nifs: Query<&WorldAssetRoot>,
    materials: Res<Assets<StandardMaterial>>,
    off: Res<Off>,
    done: Option<Res<Settled>>,
    active: Res<ActiveWorldspace>,
) {
    if done.is_none() && settled(&streaming, &terrain, &server, &nifs, &materials, &off, &active) {
        commands.insert_resource(Settled);
    }
}

fn status(
    time: Res<Time<Real>>,
    streaming: Res<Streaming>,
    terrain: Res<TerrainStats>,
    cams: Query<(&CellCoord, &Transform), With<ExteriorCamera>>,
    mut windows: Query<&mut Window>,
    mut acc: Local<(f32, u32)>,
    active: Res<ActiveWorldspace>,
) {
    acc.0 += time.delta_secs();
    acc.1 += 1;
    if acc.0 < 0.5 {
        return;
    }
    #[allow(clippy::cast_precision_loss)]
    let fps = acc.1 as f32 / acc.0;
    *acc = (0.0, 0);
    let (Ok((c, t)), Ok(mut window)) = (cams.single(), windows.single_mut()) else { return };
    let p = WorldPos::new(IVec3::new(c.x, c.y, c.z), t.translation).to_f64();
    let s = streaming.stats();
    window.title = format!(
        "Bevyrrowind {} | {fps:.0} fps | ({:.0}, {:.0}, {:.0}) | cells {} | placements {} | models {} | terrain patches {}",
        active.0.as_ref().map_or("-", sf_world::WorldspaceId::as_str),
        p.x,
        p.y,
        p.z,
        s.resident_cells,
        s.placements_spawned,
        s.models_loaded,
        terrain.patches
    );
}

// -- readiness, screenshots, benchmarks ---------------------------------------------

/// Whether streaming, terrain (in the exterior), NIF models and textures
/// have all settled.
fn settled(
    streaming: &Streaming,
    terrain: &TerrainStats,
    server: &AssetServer,
    nifs: &Query<&WorldAssetRoot>,
    materials: &Assets<StandardMaterial>,
    off: &Off,
    active: &ActiveWorldspace,
) -> bool {
    let loading = |id: bevy::asset::UntypedAssetId| matches!(server.get_load_state(id), Some(LoadState::Loading));
    let terrain_ok = off.has(Part::Terrain)
        || active.0.as_ref() != Some(&ids::exterior())
        || (terrain.height_levels == 10 && terrain.loads_in_flight == 0 && terrain.upload_bytes == 0);
    // The start's own index must be in (before it, nothing is requested yet).
    let directory = active.0.as_ref().is_some_and(|w| streaming.has_directory(w));
    directory
        && streaming.is_idle()
        && terrain_ok
        && nifs.iter().all(|r| {
            server.is_loaded_with_dependencies(&r.0) || matches!(server.get_load_state(&r.0), Some(LoadState::Failed(_)))
        })
        && materials.iter().all(|(_, m)| {
            [&m.base_color_texture, &m.emissive_texture].into_iter().flatten().all(|h| !loading(h.id().untyped()))
        })
}

/// [`settled`] as a system parameter, for runs that ask on demand ([`crate::remote`]).
#[cfg(feature = "remote")]
#[derive(bevy::ecs::system::SystemParam)]
pub(crate) struct Readiness<'w, 's> {
    streaming: Option<Res<'w, Streaming>>,
    terrain: Option<Res<'w, TerrainStats>>,
    server: Res<'w, AssetServer>,
    nifs: Query<'w, 's, &'static WorldAssetRoot>,
    materials: Res<'w, Assets<StandardMaterial>>,
    off: Option<Res<'w, Off>>,
    active: Option<Res<'w, ActiveWorldspace>>,
}

#[cfg(feature = "remote")]
impl Readiness<'_, '_> {
    /// `None` outside the exterior viewer.
    pub(crate) fn settled(&self) -> Option<bool> {
        let (Some(streaming), Some(terrain), Some(off), Some(active)) = (&self.streaming, &self.terrain, &self.off, &self.active) else {
            return None;
        };
        Some(settled(streaming, terrain, &self.server, &self.nifs, &self.materials, off, active))
    }
}

#[allow(clippy::too_many_arguments)]
fn capture(
    mut commands: Commands,
    capture: Res<Capture>,
    streaming: Res<Streaming>,
    terrain: Res<TerrainStats>,
    server: Res<AssetServer>,
    nifs: Query<&WorldAssetRoot>,
    materials: Res<Assets<StandardMaterial>>,
    off: Res<Off>,
    started: Res<Started>,
    entities: Query<()>,
    mut ready_at: Local<Option<f32>>,
    mut taken: Local<Option<u32>>,
    shots: Query<(), With<Screenshot>>,
    door: (Res<crate::doors::AutoDoor>, Res<FadeTransition>, Res<ActiveWorldspace>),
    mut exit: MessageWriter<AppExit>,
) {
    let now = started.0.elapsed().as_secs_f32();
    if let Some(frames) = taken.as_mut() {
        *frames += 1;
        if *frames > 5 && shots.is_empty() {
            exit.write(AppExit::Success);
        }
        return;
    }
    // `--use-door`: the shot is of the other side.
    if door.0.remaining > 0 || door.1.is_active() {
        *ready_at = None;
        return;
    }
    let ok = settled(&streaming, &terrain, &server, &nifs, &materials, &off, &door.2) || now > 120.0;
    let Some(at) = *ready_at else {
        if ok {
            *ready_at = Some(now);
            let s = streaming.stats();
            info!(
                "settled {now:.1} s after start: {} cells, {} placements, {} models, {} uncooked, {} size-culled, {} preloaded NIFs, {} entities, {} terrain patches",
                s.resident_cells,
                s.placements_spawned,
                s.models_loaded,
                s.uncooked,
                s.size_culled,
                s.prepared,
                entities.iter().count(),
                terrain.patches
            );
        }
        return;
    };
    if now - at >= capture.settle {
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(capture.path.clone()));
        *taken = Some(0);
    }
}

fn hold_warmup(
    streaming: Res<Streaming>,
    terrain: Res<TerrainStats>,
    server: Res<AssetServer>,
    nifs: Query<&WorldAssetRoot>,
    materials: Res<Assets<StandardMaterial>>,
    (off, active): (Res<Off>, Res<ActiveWorldspace>),
    scenario: Option<ResMut<DiagScenario>>,
) {
    if let Some(mut s) = scenario {
        s.hold = !settled(&streaming, &terrain, &server, &nifs, &materials, &off, &active);
    }
}

/// Terrain and world counters for the report (streaming's own come from `sf_world_bevy`).
fn counters(
    terrain: Res<TerrainStats>,
    counters: Res<DiagCounters>,
    meshes: Query<(), With<Mesh3d>>,
    lights: Query<(), With<PointLight>>,
) {
    #[allow(clippy::cast_precision_loss)]
    {
        counters.add_named("terrain/select/ms", "ms", f64::from(terrain.select_ms));
        counters.add_named("terrain/update/ms", "ms", f64::from(terrain.update_ms));
        counters.add_named("terrain/upload/bytes", "bytes", terrain.upload_bytes as f64);
        counters.set_named("terrain/patches", "count", terrain.patches as f64);
        // `Query::count` on archetypal queries is O(archetypes); entity totals
        // come from `sf_diag_bevy` (`ecs/entities`).
        counters.set_named("world/meshes", "count", meshes.count() as f64);
        counters.set_named("world/point_lights", "count", lights.count() as f64);
    }
}

/// Distant lights glow from dusk to dawn: 0 with the sun 0.05 rad above the
/// horizon, 1 once it is set.
fn far_lights_night(sun: Query<&GlobalTransform, With<SunLight>>, mut night: ResMut<sf_lod_bevy::FarLightsNight>) {
    let Ok(sun) = sun.single() else { return };
    // The light shines along its forward; the sun's elevation is minus that.
    let elevation = -sun.forward().y;
    let value = (0.3 - elevation * 6.0).clamp(0.0, 1.0);
    if (night.0 - value).abs() > 1e-3 {
        night.0 = value;
    }
}

/// Copies the far static sun map into the terrain's input (shadows F3b).
fn sun_shadow_input(fsm: Option<Res<sf_sky_bevy::FarSunShadow>>, input: Option<ResMut<sf_terrain_bevy::SunShadowInput>>) {
    let (Some(fsm), Some(mut input)) = (fsm, input) else { return };
    if input.image != fsm.image || input.clip_from_world != fsm.clip_from_world || input.params != fsm.params {
        input.image = fsm.image.clone();
        input.clip_from_world = fsm.clip_from_world;
        input.params = fsm.params;
    }
}

/// Terrain patches stay out of the far sun map: the terrain shadows itself
/// with its own horizon term past the cascades.
fn no_far_shadow_terrain(
    add: On<Add<MeshMaterial3d<sf_terrain_bevy::TerrainMaterial>>>,
    mut commands: Commands,
) {
    commands.entity(add.entity).insert(sf_sky_bevy::NoFarShadow);
}

/// Far proxies stay out of the far sun map: they sample it (a pass can't
/// write the depth texture its materials bind), and the terrain's own far
/// term already shadows them past the cascades.
fn no_far_shadow_proxy(add: On<Add<MeshMaterial3d<sf_lod_bevy::ProxyMaterial>>>, mut commands: Commands) {
    commands.entity(add.entity).insert(sf_sky_bevy::NoFarShadow);
}

/// Copies the terrain's far shadow map and the far static sun map into the
/// far field's input (proxies sample both past the cascades).
fn far_shadow_input(
    map: Option<Res<sf_terrain_bevy::FarShadowMap>>,
    sun: Option<Res<sf_sky_bevy::FarSunShadow>>,
    mut input: ResMut<sf_lod_bevy::FarShadowInput>,
) {
    let Some(map) = map else { return };
    let wanted = sf_lod_bevy::FarShadowInput {
        image: Some(map.image.clone()),
        params: map.params,
        generation: map.generation,
        sun_map: sun.as_ref().map(|s| s.image.clone()),
        sun_clip_from_world: sun.as_ref().map_or(Mat4::IDENTITY, |s| s.clip_from_world),
        sun_params: sun.as_ref().map_or(Vec4::ZERO, |s| s.params),
    };
    if *input != wanted {
        *input = wanted;
    }
}

/// Copies the terrain's far shadow map into the fog's (the froxels' sun shafts
/// from ridges past the cascades, weather.md "Fog", F3).
fn fog_shadow_heights(map: Option<Res<sf_terrain_bevy::FarShadowMap>>, mut input: ResMut<sf_atmos_bevy::FogShadowHeights>) {
    let Some(map) = map else { return };
    let wanted = sf_atmos_bevy::FogShadowHeights { image: Some(map.image.clone()), params: map.params };
    if *input != wanted {
        *input = wanted;
    }
}

/// `config` with optional R1/R2 load radii (calibration runs).
fn streaming_config(mut config: StreamingConfig, near: Option<f32>, far: Option<f32>) -> StreamingConfig {
    // A/B switch (perf.md): meshes of cooked models loading at once, 0 = no cap.
    if let Some(n) = std::env::var("BEVYRROWIND_LOADING_MESHES").ok().and_then(|v| v.parse().ok()) {
        config.max_loading_meshes = n;
    }
    let rings = &mut config.scheduler.rings;
    if let Some(load) = near {
        let margin = rings.near.unload - rings.near.load;
        rings.near = sf_world::RingRadii::new(load, load + margin);
    }
    if let Some(load) = far {
        let margin = rings.far.unload - rings.far.load;
        rings.far = sf_world::RingRadii::new(load, load + margin);
    }
    config
}

/// The sun's shadow cascades (shadows F3, weather.md "Distant shadows"): a
/// practical split to ~250 m per preset ([`Quality::sun_cascades`]), 2× sharper
/// at mid range than the A1 layout; past them the terrain shadows itself
/// (`sf_terrain_bevy`'s far shadow) and, once built, the cached far static map
/// takes the objects. `splits_off` (`--off shadowsplits`) is the A1 layout (4
/// geometric cascades past every individual object: handoff + band, +10 %, + 20 m
/// = 504 m), `legacy` (`--off shadowtune`) the one before it, for A/B runs.
fn sun_cascades(quality: Quality, levels: &sf_lod::FarLevels, legacy: bool, splits_off: bool) -> CascadeShadowConfig {
    if !legacy && !splits_off {
        return quality.sun_cascades().build();
    }
    let objects = levels.handoff(1) + levels.band_width(1);
    let (first, max) = if legacy { (20.0, 600.0) } else { (10.0, objects * 1.1 + 20.0) };
    CascadeShadowConfigBuilder {
        num_cascades: 4,
        minimum_distance: 0.1,
        first_cascade_far_bound: first,
        maximum_distance: max,
        overlap_proportion: 0.2,
    }
    .build()
}

/// The sun's shadow bias (weather.md "Shadow bias and contact shadows", shadows F2):
/// Bevy's 1.8-texel normal offset lifts every lookup off the surface by 1.3 m in
/// the last cascade, so small casters lose the shadow where they touch the ground;
/// `SUN_NORMAL_BIAS` texels keep acne away at grazing sun. `legacy` (`--off shadowtune`)
/// is Bevy's default.
fn sun_shadow(legacy: bool, contact_shadows: bool) -> sf_sky_bevy::SunShadow {
    let defaults = sf_sky_bevy::SunShadow::default();
    if legacy {
        return sf_sky_bevy::SunShadow { contact_shadows, ..defaults };
    }
    sf_sky_bevy::SunShadow { depth_bias: SUN_DEPTH_BIAS, normal_bias: SUN_NORMAL_BIAS, contact_shadows }
}

/// Sun shadow depth bias, meters along the light (Bevy: 0.02).
const SUN_DEPTH_BIAS: f32 = 0.02;
/// Sun shadow normal bias, texels of each cascade (Bevy: 1.8).
const SUN_NORMAL_BIAS: f32 = 0.8;

// -- clouds ------------------------------------------------------------------------

/// Morrowind's world is compressed, so cloud altitudes **and sizes** are
/// halved together (weather.md open question 8; `mw_weather::visual`).
pub(crate) const CLOUD_WORLD_SCALE: f32 = mw_weather::visual::CLOUD_WORLD_SCALE;

use sf_atmos_bevy::sf_atmos::CloudPreset;

// -- live weather (weather.md "Fog", "Clouds" and "Distant weather", M6 WE3/WE4c) ------

/// The weather's site (WE4c): the climates at the camera from the world
/// weather map's region raster (`WorldWeatherMap::site_at`: own region,
/// blended with a neighbour's across `sf_weather::BORDER_BAND_M`; sea takes
/// the nearest region's), so the near weather equals the map's there. Also
/// places the map's window on the camera. Kept as is in interiors (the
/// camera there is not on the exterior's grid), where precipitation is off
/// (no roofs in the occlusion yet: there is none).
fn weather_site(
    camera: Query<&GlobalTransform, With<ExteriorCamera>>,
    origin: Res<TerrainOrigin>,
    active: Res<ActiveWorldspace>,
    mut map: ResMut<sf_weather_bevy::WorldWeatherMap>,
    mut site: ResMut<WeatherSite>,
    precip: Option<ResMut<sf_weather_bevy::sf_precip_bevy::Precip>>,
) {
    let exterior = active.0.as_ref().is_none_or(|ws| ws.as_str() == ids::EXTERIOR);
    if let Some(mut precip) = precip
        && precip.enabled != exterior
    {
        precip.enabled = exterior;
    }
    let Some(camera) = camera.iter().next() else { return };
    if !exterior && !site.climates.is_empty() {
        return;
    }
    let p = WorldPos::new(origin.0.cell, camera.translation()).to_f64();
    #[expect(clippy::cast_possible_truncation, reason = "altitude in meters fits f32")]
    let altitude = p.y as f32;
    let xz = bevy::math::DVec2::new(p.x, p.z);
    map.camera = Some(xz);
    let climates = map.site_at(xz);
    let same = site.climates.len() == climates.len()
        && site.climates.iter().zip(&climates).all(|(a, b)| a.0 == b.0 && (a.1 - b.1).abs() < 1e-3);
    if !same {
        site.climates = climates;
    }
    if (site.altitude - altitude).abs() > 1.0 {
        site.altitude = altitude;
    }
}

/// The wind field's floating origin (the terrain's, set by [`sync_origin`]).
fn wind_origin(origin: Res<TerrainOrigin>, mut wind: ResMut<sf_wind_bevy::WindOrigin>) {
    if wind.0 != origin.0 {
        wind.0 = origin.0;
    }
}

/// The clouds before the weather's first frame (it writes `settings.weather`
/// from then on, `sf_weather_bevy`): the look with sizes halved with the
/// altitudes ([`CLOUD_WORLD_SCALE`], we2b's noise halving; the presets'
/// heights come halved from `mw_weather::visual`).
pub(crate) fn cloud_settings() -> sf_atmos_bevy::sf_atmos::CloudSettings {
    let mut settings = sf_atmos_bevy::sf_atmos::CloudSettings::preset(CloudPreset::Fair);
    settings.weather = pinned_clouds(CloudPreset::Fair);
    settings.look = settings.look.scaled(CLOUD_WORLD_SCALE);
    settings
}

/// A genus pinned over the weather's clouds (`--clouds`, BRP `clouds`), at
/// Morrowind's halved altitudes; its wind stays the weather's.
pub(crate) fn pinned_clouds(genus: CloudPreset) -> sf_atmos_bevy::sf_atmos::CloudWeather {
    sf_atmos_bevy::sf_atmos::CloudWeather::preset(genus).scaled_heights(CLOUD_WORLD_SCALE)
}


/// The sun receives the cloud shadow map.
fn cloud_shadow_light(mut commands: Commands, suns: Query<Entity, (With<SunLight>, Without<sf_atmos_bevy::CloudShadowLight>)>) {
    for sun in &suns {
        commands.entity(sun).insert(sf_atmos_bevy::CloudShadowLight);
    }
}

/// Quality presets (industry-audit.md J1, ROADMAP Next up 4): one view-distance
/// `bias` scales LOD selection, size culling and the far-field handoff `d1`
/// (the baked 400 m × bias, so the rings follow), plus the size-culling cutoff
/// in reference pixels and its floor. High matches the small-feature cutoff of `OpenMW`
/// (2 px), Ultra our 1 px LOD budget.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Quality {
    Low,
    Medium,
    #[default]
    High,
    Ultra,
}

impl Quality {
    pub const ENV: &'static str = "BEVYRROWIND_QUALITY";

    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            "ultra" => Some(Self::Ultra),
            _ => None,
        }
    }

    /// `--quality`, else `BEVYRROWIND_QUALITY`, else High.
    pub fn resolve(flag: Option<Self>) -> Self {
        flag.or_else(|| std::env::var(Self::ENV).ok().and_then(|v| Self::parse(&v))).unwrap_or_default()
    }

    /// `(bias, min_pixels, min_draw_distance)`.
    const fn params(self) -> (f32, f32, f32) {
        match self {
            Self::Low => (0.75, 4.0, 16.0),
            Self::Medium => (0.875, 3.0, 20.0),
            Self::High => (1.0, 2.0, 24.0),
            Self::Ultra => (1.0, 1.0, 32.0),
        }
    }

    /// Whether the clouds read the world weather map's layer per density
    /// sample (High/Ultra) or per ray, 3 fetches lerped (Medium/Low;
    /// `sf_atmos::CloudQuality::map_per_sample`, weather.md, distant weather as built).
    pub const fn cloud_map_per_sample(self) -> bool {
        matches!(self, Self::High | Self::Ultra)
    }

    /// Point lights with shadow maps and their cube face size
    /// (`sf_sky_bevy::PointShadowBudget`, ranked by `shadow_importance`). Bevy
    /// redraws each map every frame: census office bench (720p, RTX 4060 laptop,
    /// shf6 2026-10-19) median frame 4.8 ms with none, 6.6 / 12.2 / 21.7 ms with
    /// 1 / 4 / 8, render-thread bound (GPU ~0.55 ms per light at 1024²). Counts
    /// rise once cached maps land (superfargoth `docs/specs/shadows.md`).
    pub const fn point_shadows(self) -> (usize, usize) {
        match self {
            Self::Low | Self::Medium => (0, 512),
            Self::High => crate::render::POINT_SHADOWS_DEFAULT,
            Self::Ultra => (4, 1024),
        }
    }

    /// Screen-space contact shadows (shadows F1, weather.md "Shadow bias and contact
    /// shadows"): Bevy's depth-prepass ray march toward the sun and the budgeted point
    /// lights, so small objects stay grounded at any cascade texel size. Off on Low
    /// (it costs the depth + motion-vector prepass).
    pub const fn contact_shadows(self) -> Option<ContactShadows> {
        match self {
            Self::Low => None,
            Self::Medium => Some(ContactShadows { linear_steps: 8, thickness: 0.1, length: 0.5 }),
            Self::High | Self::Ultra => Some(ContactShadows { linear_steps: 16, thickness: 0.1, length: 0.5 }),
        }
    }

    /// Sun cascades (shadows F3): practical split (PSSM log/uniform blend, split
    /// near 1 m), Bevy's sphere fit + texel snap + 20 % overlap blend. At 70° FOV,
    /// 16:9, 2048² the texels are 1.4 / 3.8 / 11 / 35 cm on High (A1: 1.4 / 5.2 / 19 /
    /// 70 cm, `sf_sky_bevy` test `practical_split_cascades_and_texel_sizes`). Low
    /// drops a cascade (one less caster pass) and ends at 200 m.
    pub const fn sun_cascades(self) -> sf_sky_bevy::CascadeSplits {
        let (num_cascades, maximum_distance, lambda) = match self {
            Self::Low => (3, 200.0, 0.85),
            Self::Medium | Self::High | Self::Ultra => (4, 250.0, 0.9),
        };
        sf_sky_bevy::CascadeSplits { num_cascades, minimum_distance: 0.1, split_near: 1.0, maximum_distance, lambda, overlap_proportion: 0.2 }
    }

    /// Shadow-map filtering (shadows F2): Hardware2x2 on Low (one tap), Gaussian
    /// above (Temporal needs TAA, which raster doesn't run).
    pub const fn shadow_filter(self) -> ShadowFilteringMethod {
        match self {
            Self::Low => ShadowFilteringMethod::Hardware2x2,
            Self::Medium | Self::High | Self::Ultra => ShadowFilteringMethod::Gaussian,
        }
    }

    /// The streaming screen of this preset; `size_cull: false` keeps every
    /// object to its ring or handoff (`--off sizecull`).
    pub fn screen(self, size_cull: bool) -> sf_model::LodScreen {
        let (bias, min_pixels, min_draw_distance) = self.params();
        sf_model::LodScreen { bias, min_pixels: if size_cull { min_pixels } else { 0.0 }, min_draw_distance, ..sf_model::LodScreen::default() }
    }
}

/// The worldspaces with a baked far field (the exterior): prefetched
/// interiors ask for none.
fn far_worldspaces(manifest: &Manifest) -> sf_lod_bevy::FarWorldspaces {
    sf_lod_bevy::FarWorldspaces(Some(
        manifest
            .products
            .keys()
            .filter(|k| k.step == sf_lod::products::DIR_STEP)
            .map(|k| sf_world::WorldspaceId::new(&k.subject))
            .collect(),
    ))
}
