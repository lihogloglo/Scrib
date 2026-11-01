//! Dynamic probe GI on the raster path (`docs/specs/gi.md`), first slice ("Spike").
//!
//! A [`GiVolume`] is a probe grid around its entity's position. Each frame a budget of its
//! probes is re-traced with hardware ray queries through Bevy Solari's ray-tracing scene
//! (`RaytracingScenePlugin` only: no `SolariLightingPlugin`, the renderer stays forward):
//! hits are shaded with our light list (sun with a shadow ray, point and spot lights with a
//! visibility ray, Bevy's units and falloff: `sf_gi::GiLight`), emissive and last frame's probes
//! (second bounce); misses read the sky environment map. Results blend into DDGI's octahedral
//! irradiance and visibility atlases (`sf_gi::AtlasLayout`, render-world textures), which the
//! vendored `bevy_pbr` samples per pixel with visibility weights (`GiProbeVolumeView`, its
//! `gi_probes.wesl`): the volume replaces the environment map's diffuse inside its box.
//!
//! The volume is shown (the render world gives the cameras' views the probe bindings, the main
//! world marks it [`GiShown`]) only once every probe has been traced (the render world raises
//! [`GiTraced`]): until then, and whenever nothing traces (no ray-tracing GPU features,
//! [`GiSupport`] false, or an empty ray-tracing scene), today's ambient path draws, never a
//! black box. Without the features the system is absent: no render systems, no pipelines.
//!
//! GPU timer: `render/gi/probe_update/elapsed_gpu` (Bevy's `RenderDiagnosticsPlugin`, added by
//! the app); [`GiPlugin`] logs it once per second (`gi: probe_update ...`).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use bevy_app::{App, Plugin, Update};
use bevy_asset::embedded_asset;
use bevy_diagnostic::{DiagnosticPath, DiagnosticsStore};
use bevy_ecs::prelude::*;
use bevy_mesh::{Indices, Mesh, PrimitiveTopology};
use bevy_render::renderer::RenderDevice;
use bevy_render::RenderApp;
use bevy_solari::scene::RaytracingScenePlugin;
use bevy_solari::SolariPlugins;
use bevy_time::{Real, Time};
use bevy_transform::components::Transform;
use sf_gi::GiSettings;

mod render;

pub use sf_gi;

/// Whether this GPU can run the GI (Solari's required ray-query and binding-array features).
/// Set at plugin `finish`; false: the system is absent.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GiSupport(pub bool);

/// Shared by both worlds: set by the render world once every probe of the volume has been
/// traced at least once; the main world then shows the volume.
#[derive(Resource, Clone, Default)]
pub(crate) struct GiTraced(pub(crate) Arc<AtomicBool>);

/// The volume is attached: the render world traces it (its atlases live there).
#[derive(Component, Clone, Copy)]
pub(crate) struct GiVoxels;

/// Marks a volume whose every probe has been traced once: from then on the cameras' views
/// sample it (`GiProbeVolumeView` in the render world). Removed with [`GiVoxels`].
#[derive(Component, Clone, Copy, Debug)]
pub struct GiShown;

/// A probe grid centred on its entity's translation (rotation and scale are the plugin's:
/// it sets the scale to the grid's extent). Spawn it with a `Transform`
/// in the space the meshes live in; the plugin marks it [`GiShown`] and the cameras sample it
/// once the grid has been traced.
#[derive(Component, Clone, Copy, Debug)]
#[require(Transform)]
pub struct GiVolume {
    pub settings: GiSettings,
    /// Multiplier on the sampled probe irradiance (1: the traced radiance as is).
    pub intensity: f32,
    /// Second bounce: hits read last frame's probes.
    pub bounce: bool,
}

impl Default for GiVolume {
    fn default() -> Self {
        Self { settings: GiSettings::default(), intensity: 1.0, bounce: true }
    }
}

/// The GI plugin. Add it before `sf_sky_bevy::SkyPlugin`, so the sky finds Solari's scene
/// plugin present and does not add the full `SolariPlugins` (lighting included).
#[derive(Default)]
pub struct GiPlugin;

impl Plugin for GiPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "probe_update.wesl");
        // Absent without the GPU features: the scene plugin itself warns and stays empty.
        if !app.is_plugin_added::<RaytracingScenePlugin>() {
            app.add_plugins(RaytracingScenePlugin);
        }
        app.init_resource::<GiSupport>()
            .init_resource::<sf_sky_bevy::GiActive>()
            .init_resource::<GiLog>()
            .init_resource::<GiTraced>()
            .init_resource::<GiSceneStats>()
            .add_systems(Update, (disable_volumes, attach_volumes, show_volumes, update_active, log_gi).chain());
    }

    fn finish(&self, app: &mut App) {
        let supported = app
            .get_sub_app(RenderApp)
            .and_then(|render| render.world().get_resource::<RenderDevice>())
            .is_some_and(|device| device.features().contains(SolariPlugins::required_wgpu_features()));
        app.insert_resource(GiSupport(supported));
        if supported {
            app.insert_resource(sf_sky_bevy::GiEnabled(true));
            render::build(app);
        } else {
            log::info!("gi: ray-tracing features missing on this GPU; dynamic GI off");
        }
    }
}

/// [`sf_sky_bevy::GiEnabled`] off: the volumes lose their texture and light probe (nothing
/// traces or shows them; today's ambient path draws), the entities and the ray-traced scene stay.
fn disable_volumes(
    mut commands: Commands,
    enabled: Option<Res<sf_sky_bevy::GiEnabled>>,
    volumes: Query<Entity, (With<GiVolume>, With<GiVoxels>)>,
) {
    let Some(enabled) = enabled else { return };
    if !enabled.is_changed() || enabled.0 {
        return;
    }
    for entity in &volumes {
        commands.entity(entity).remove::<(GiVoxels, GiShown)>();
    }
    log::info!("gi: off (volumes kept, not traced)");
}

/// Attaches each new volume (while
/// [`sf_sky_bevy::GiEnabled`]; switched back on, the volumes get new textures and re-trace).
fn attach_volumes(
    mut commands: Commands,
    support: Res<GiSupport>,
    enabled: Option<Res<sf_sky_bevy::GiEnabled>>,
    traced: Res<GiTraced>,
    mut volumes: Query<(Entity, &GiVolume, &mut Transform), Without<GiVoxels>>,
) {
    if !support.0 || enabled.is_some_and(|e| !e.0) {
        return;
    }
    for (entity, volume, mut transform) in &mut volumes {
        // A new volume is untraced: no stale "traced" from the last volume shows it early (the
        // render world clears it too once it sees the new volume and makes its atlases).
        traced.0.store(false, Ordering::Relaxed);
        transform.rotation = bevy_math::Quat::IDENTITY;
        transform.scale = volume.settings.grid().extent();
        commands.entity(entity).insert(GiVoxels);
        log::info!(
            "gi: volume {}x{}x{} probes at {} m, {} rays, {} per frame, hysteresis {}",
            volume.settings.dims.x,
            volume.settings.dims.y,
            volume.settings.dims.z,
            volume.settings.spacing,
            volume.settings.rays(),
            volume.settings.probes_per_frame,
            volume.settings.hysteresis
        );
    }
}

/// Marks the traced volumes [`GiShown`] once the whole grid was traced (the render world then
/// binds the atlases to the cameras' views).
fn show_volumes(
    mut commands: Commands,
    traced: Res<GiTraced>,
    volumes: Query<Entity, (With<GiVoxels>, Without<GiShown>)>,
) {
    if volumes.is_empty() || !traced.0.load(Ordering::Relaxed) {
        return;
    }
    for entity in &volumes {
        commands.entity(entity).insert(GiShown);
        log::info!("gi: volume traced, sampled by the PBR shader (DDGI probes)");
    }
}

/// The ray-traced scene is needed (meshes mirrored, `sf_sky_bevy` `path.rs`) while a volume exists.
fn update_active(support: Res<GiSupport>, volumes: Query<(), With<GiVolume>>, mut active: ResMut<sf_sky_bevy::GiActive>) {
    let wanted = support.0 && !volumes.is_empty();
    if active.0 != wanted {
        active.0 = wanted;
    }
}

#[derive(Resource, Default)]
struct GiLog {
    next: f64,
}

/// Once per second: `gi: probe_update N ms, P probes, R rays` plus the render world's scene counts.
fn log_gi(
    time: Res<Time<Real>>,
    mut state: ResMut<GiLog>,
    support: Res<GiSupport>,
    diagnostics: Option<Res<DiagnosticsStore>>,
    volumes: Query<&GiVolume, With<GiVoxels>>,
    scene: Res<GiSceneStats>,
) {
    let now = time.elapsed_secs_f64();
    if !support.0 || now < state.next || volumes.is_empty() {
        return;
    }
    state.next = now + 1.0;
    let (mut probes, mut rays) = (0u32, 0u32);
    let mut tiles = bevy_math::UVec2::ZERO;
    for v in &volumes {
        let count = v.settings.probes_per_frame.min(v.settings.grid().probe_count());
        probes += count;
        rays += count * v.settings.rays();
        tiles = sf_gi::AtlasLayout::irradiance(v.settings.dims).tiles();
    }
    let path = DiagnosticPath::const_new("render/gi/probe_update/elapsed_gpu");
    let ms = diagnostics.as_ref().and_then(|d| d.get(&path)).and_then(bevy_diagnostic::Diagnostic::smoothed);
    let ms = ms.map_or_else(|| "n/a (no GPU timing yet, or no RenderDiagnosticsPlugin)".to_owned(), |ms| format!("{ms:.3} ms"));
    let s = scene.get();
    log::info!(
        "gi: probe_update {ms}, {probes} probes, {rays} rays, atlas {}x{} tiles; ray-traced meshes {} mirrored, {} accepted (BLAS), rejected {} \
         (not enabled {}, attributes {}, topology {}, not u32 indices {}), unclassified {}; Solari lights {}; \
         lights {} ({} changed), sun {}",
        tiles.x,
        tiles.y,
        s.mirrored,
        s.accepted,
        s.rejected(),
        s.disabled,
        s.attributes,
        s.topology,
        s.indices,
        s.unclassified,
        s.lights,
        s.gi_lights,
        s.gi_lights_changed,
        if s.sun { "yes" } else { "no" }
    );
}

/// Render-world scene counts, refreshed every frame (`render.rs` `scene_stats`), read by the log.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct SceneCounts {
    /// `RaytracingMesh3d` entities in the render world.
    pub(crate) mirrored: u32,
    /// Instances in Solari's TLAS (their mesh has a BLAS).
    pub(crate) accepted: u32,
    /// Rejected by Solari's BLAS filter, by first failing reason.
    pub(crate) disabled: u32,
    pub(crate) attributes: u32,
    pub(crate) topology: u32,
    pub(crate) indices: u32,
    /// Mesh not (yet) seen by the render world.
    pub(crate) unclassified: u32,
    /// Solari's light sources (directional lights + emissive instances).
    pub(crate) lights: u32,
    /// Our light list (`render.rs` `extract`): lights in it, of which changed this frame.
    pub(crate) gi_lights: u32,
    pub(crate) gi_lights_changed: u32,
    /// A sun is visible to the GI cameras.
    pub(crate) sun: bool,
}

impl SceneCounts {
    fn rejected(&self) -> u32 {
        self.disabled + self.attributes + self.topology + self.indices
    }
}

/// Shared by both worlds (the render world writes, the log reads).
#[derive(Resource, Clone, Default)]
pub(crate) struct GiSceneStats(pub(crate) Arc<Mutex<SceneCounts>>);

impl GiSceneStats {
    fn get(&self) -> SceneCounts {
        self.0.lock().map(|s| *s).unwrap_or_default()
    }
}

/// Why Solari would not build a BLAS for a mesh.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Verdict {
    Compatible,
    /// `Mesh::enable_raytracing` false.
    Disabled,
    /// Not exactly position/normal/uv0/tangent.
    Attributes,
    /// Not a triangle list.
    Topology,
    /// No index buffer, or u16 indices.
    Indices,
}

/// Solari's BLAS filter (`bevy_solari` 0.20 `scene/blas.rs` `is_mesh_raytracing_compatible`),
/// with the first failing reason: exactly position/normal/uv0/tangent, triangle list, u32 indices.
/// `None`: the mesh's data is gone (render-world only).
pub(crate) fn solari_verdict(mesh: &Mesh) -> Option<Verdict> {
    let attributes = mesh.try_attributes().ok()?.map(|(attribute, _)| (attribute.id, attribute.format)).eq([
        (Mesh::ATTRIBUTE_POSITION.id, Mesh::ATTRIBUTE_POSITION.format),
        (Mesh::ATTRIBUTE_NORMAL.id, Mesh::ATTRIBUTE_NORMAL.format),
        (Mesh::ATTRIBUTE_UV_0.id, Mesh::ATTRIBUTE_UV_0.format),
        (Mesh::ATTRIBUTE_TANGENT.id, Mesh::ATTRIBUTE_TANGENT.format),
    ]);
    let indices = mesh.try_indices_option().ok()?;
    Some(if !mesh.enable_raytracing {
        Verdict::Disabled
    } else if !attributes {
        Verdict::Attributes
    } else if mesh.primitive_topology() != PrimitiveTopology::TriangleList {
        Verdict::Topology
    } else if !matches!(indices, Some(Indices::U32(..))) {
        Verdict::Indices
    } else {
        Verdict::Compatible
    })
}
