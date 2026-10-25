//! The raster / ray-traced render path switch.
//!
//! Ray tracing is Bevy Solari (cargo feature `solari`). Solari needs a
//! deferred G-buffer, `Msaa::Off`, a storage-capable main texture, and
//! meshes registered with `RaytracingMesh3d`; the raster path is forward
//! with a depth prepass so `OcclusionCulling` works (ARCHITECTURE §10).
//! Switching therefore:
//!
//! 1. sets `DefaultOpaqueRendererMethod` (deferred for Solari, forward for
//!    raster) and marks every `StandardMaterial` modified so it is
//!    re-prepared: a one-off hitch, like applying a graphics setting;
//! 2. snapshots and restores the camera components Solari changes;
//! 3. mirrors `Mesh3d` into `RaytracingMesh3d` (and back) for visible
//!    meshes that don't opt out with [`NoRayTracing`], using the
//!    [`RayTracingMesh`] variant when the entity has one;
//! 4. lets `proxy.rs` spawn or despawn the point-light proxies, and
//!    `apply_lights` drop shadow maps and pre-attenuate the sun.

use bevy_app::App;
use bevy_asset::Handle;
use bevy_camera::Camera3d;
use bevy_ecs::prelude::*;
use bevy_mesh::Mesh;

use crate::{ActiveRenderPath, RenderPathSetting};

/// Whether this build and GPU can run the ray-traced path. Set once at
/// startup (plugin `finish`).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RayTracingSupport(pub bool);

/// Keeps a mesh out of the ray-traced scene (e.g. grass, decals, effects,
/// LOD levels other than the one the ray tracer uses).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct NoRayTracing;

/// A 3D camera whose render path this plugin manages (G-buffer, MSAA,
/// prepasses, Solari settings). [`crate::SkyCamera`] requires it; interior
/// cameras without a sky add it alone.
#[derive(Component, Clone, Copy, Debug, Default)]
#[require(Camera3d)]
pub struct RenderPathCamera;

/// The mesh the ray tracer uses for this entity instead of its `Mesh3d`.
///
/// Solari takes only uncompressed position/normal/uv0/tangent meshes with
/// u32 indices and silently skips anything else, while the raster path
/// wants compressed vertices and extra attributes (vertex colors). Loaders
/// build this variant only when the ray-traced path is in use.
#[derive(Component, Clone, Debug)]
pub struct RayTracingMesh(pub Handle<Mesh>);

/// The render path systems (camera setup, mesh mirroring); upscaling runs
/// after them.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PathSystems;

pub(crate) fn build(app: &mut App) {
    app.init_resource::<RayTracingSupport>();
    app.configure_sets(bevy_app::Update, PathSystems.in_set(crate::SkySystems::RenderPath));
    #[cfg(feature = "solari")]
    solari::build(app);
}

pub(crate) fn finish(app: &mut App) {
    #[cfg(feature = "solari")]
    solari::finish(app);
    let supported = app.world().resource::<RayTracingSupport>().0;
    let requested = app.world().resource::<RenderPathSetting>().0;
    let active = requested.resolve(supported);
    if requested != active {
        log::warn!("ray-traced lighting unavailable on this GPU or build; using raster");
    }
    app.insert_resource(ActiveRenderPath(active));
    #[cfg(feature = "solari")]
    solari::set_default_method(app.world_mut(), active);
}

#[cfg(feature = "solari")]
mod solari {
    use bevy_app::{App, Update};
    use bevy_asset::{AssetMut, Assets};
    use bevy_camera::{Camera3d, CameraMainTextureUsages};
    use bevy_core_pipeline::prepass::{
        DeferredPrepass, DeferredPrepassDoubleBuffer, DepthPrepass, DepthPrepassDoubleBuffer,
        MotionVectorPrepass,
    };
    use bevy_ecs::prelude::*;
    use bevy_ecs::system::EntityCommands;
    use bevy_mesh::Mesh3d;
    use bevy_pbr::{DefaultOpaqueRendererMethod, MeshMaterial3d, StandardMaterial};
    use bevy_render::RenderApp;
    use bevy_render::camera::{MipBias, TemporalJitter};
    use bevy_render::occlusion_culling::OcclusionCulling;
    use bevy_render::render_resource::TextureUsages;
    use bevy_render::renderer::RenderDevice;
    use bevy_render::view::Msaa;
    use bevy_solari::SolariPlugins;
    use bevy_solari::prelude::{RaytracingMesh3d, SolariLighting};
    use bevy_solari::scene::RaytracingScenePlugin;

    use bevy_camera::visibility::InheritedVisibility;

    use super::{NoRayTracing, RayTracingMesh, RayTracingSupport, RenderPathCamera};
    use crate::{ActiveRenderPath, RenderPathSetting, SkyRenderOptions, SkySystems, UpscalingSetting};
    use sf_sky::RenderPath;

    /// Camera state before the ray-traced path changed it.
    #[derive(Component, Clone)]
    #[expect(
        clippy::struct_excessive_bools,
        reason = "one flag per independent marker component"
    )]
    pub(super) struct RasterSnapshot {
        msaa: Msaa,
        usages: CameraMainTextureUsages,
        deferred: bool,
        depth: bool,
        motion_vectors: bool,
        occlusion_culling: bool,
        /// DLSS adds these (required components) and they outlive it.
        jitter: bool,
        mip_bias: bool,
    }

    /// The path a camera is currently configured for.
    #[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
    pub(super) struct CameraPath(RenderPath);

    /// Marks a `RaytracingMesh3d` this module added (and will remove).
    #[derive(Component, Clone, Copy, Debug)]
    pub(super) struct MirroredRayTracing;

    pub(super) fn build(app: &mut App) {
        if !app.is_plugin_added::<RaytracingScenePlugin>() {
            app.add_plugins(SolariPlugins);
        }
        app.add_systems(
            Update,
            (
                switch_path,
                restore_raster_msaa,
                configure_cameras,
                mirror_meshes,
            )
                .chain()
                .in_set(SkySystems::RenderPath)
                .in_set(super::PathSystems),
        );
    }

    pub(super) fn finish(app: &mut App) {
        let supported = app
            .get_sub_app(RenderApp)
            .and_then(|render| render.world().get_resource::<RenderDevice>())
            .is_some_and(|device| {
                device
                    .features()
                    .contains(SolariPlugins::required_wgpu_features())
            });
        app.insert_resource(RayTracingSupport(supported));
    }

    /// Solari's plugin makes deferred the global default; the raster path
    /// wants forward (occlusion culling, MSAA).
    pub(super) fn set_default_method(world: &mut World, path: RenderPath) {
        world.insert_resource(match path {
            RenderPath::RayTraced => DefaultOpaqueRendererMethod::deferred(),
            RenderPath::Raster => DefaultOpaqueRendererMethod::forward(),
        });
    }

    fn switch_path(
        setting: Res<RenderPathSetting>,
        support: Res<RayTracingSupport>,
        mut active: ResMut<ActiveRenderPath>,
        mut method: ResMut<DefaultOpaqueRendererMethod>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        if !setting.is_changed() {
            return;
        }
        let path = setting.0.resolve(support.0);
        if path == active.0 {
            return;
        }
        log::info!("render path: {:?} -> {path:?}", active.0);
        active.0 = path;
        match path {
            RenderPath::RayTraced => method.set_to_deferred(),
            RenderPath::Raster => method.set_to_forward(),
        }
        // Re-prepare every material under the new default method.
        let ids: Vec<_> = materials.ids().collect();
        for id in ids {
            materials.get_mut(id).map(AssetMut::into_inner);
        }
    }

    #[allow(clippy::type_complexity)]
    fn configure_cameras(
        mut commands: Commands,
        active: Res<ActiveRenderPath>,
        options: Res<SkyRenderOptions>,
        upscaling: Res<UpscalingSetting>,
        cameras: Query<
            (
                Entity,
                Option<&CameraPath>,
                Option<&Msaa>,
                Option<&CameraMainTextureUsages>,
                Has<DeferredPrepass>,
                Has<DepthPrepass>,
                Has<MotionVectorPrepass>,
                Has<OcclusionCulling>,
                (Has<TemporalJitter>, Has<MipBias>),
                Option<&RasterSnapshot>,
            ),
            (With<RenderPathCamera>, With<Camera3d>),
        >,
        denoiser: Denoiser,
    ) {
        for (
            entity,
            current,
            msaa,
            usages,
            deferred,
            depth,
            motion,
            occlusion,
            (jitter, mip_bias),
            snapshot,
        ) in &cameras
        {
            // A new camera starts out configured for raster.
            if current.map_or(RenderPath::Raster, |c| c.0) == active.0 {
                if current.is_none() {
                    commands.entity(entity).insert(CameraPath(active.0));
                }
                continue;
            }
            let mut e = commands.entity(entity);
            e.insert(CameraPath(active.0));
            match active.0 {
                RenderPath::RayTraced => {
                    let usages = usages.copied().unwrap_or_default();
                    e.insert((
                        RasterSnapshot {
                            msaa: msaa.copied().unwrap_or_default(),
                            usages,
                            deferred,
                            depth,
                            motion_vectors: motion,
                            occlusion_culling: occlusion,
                            jitter,
                            mip_bias,
                        },
                        Msaa::Off,
                        usages.with(TextureUsages::STORAGE_BINDING),
                    ));
                    // Occlusion culling doesn't support deferred shading.
                    e.remove::<OcclusionCulling>();
                    let with_rr = denoiser.attach(&mut e, &options, upscaling.0);
                    e.insert(SolariLighting {
                        restir: options.ray_tracing_restir.unwrap_or(!with_rr),
                        ..SolariLighting::default()
                    });
                }
                RenderPath::Raster => {
                    // Solari inserted its prepasses as required components;
                    // drop the ones the camera didn't have. MSAA comes back
                    // next frame, once the render world has no deferred view.
                    // ReSTIR's double buffers require (and keep re-adding) the
                    // prepasses, and Solari only manages them while it is on.
                    e.remove::<(
                        SolariLighting,
                        DeferredPrepassDoubleBuffer,
                        DepthPrepassDoubleBuffer,
                    )>();
                    Denoiser::detach(&mut e);
                    if let Some(snapshot) = snapshot {
                        e.insert(snapshot.usages);
                        if !snapshot.deferred {
                            e.remove::<DeferredPrepass>();
                        }
                        if !snapshot.depth {
                            e.remove::<DepthPrepass>();
                        }
                        if !snapshot.motion_vectors {
                            e.remove::<MotionVectorPrepass>();
                        }
                        if snapshot.occlusion_culling {
                            e.insert(OcclusionCulling);
                        }
                        if !snapshot.jitter {
                            e.remove::<TemporalJitter>();
                        }
                        if !snapshot.mip_bias {
                            e.remove::<MipBias>();
                        }
                    }
                }
            }
        }
    }

    /// DLSS Ray Reconstruction on ray-traced cameras (feature `dlss`): it
    /// denoises and upscales Solari's output. Without the feature, or on a
    /// GPU without support, Solari runs undenoised.
    #[derive(bevy_ecs::system::SystemParam)]
    pub(super) struct Denoiser<'w> {
        #[cfg(feature = "dlss")]
        supported: Option<Res<'w, bevy_anti_alias::dlss::DlssRayReconstructionSupported>>,
        #[cfg(not(feature = "dlss"))]
        _unused: std::marker::PhantomData<&'w ()>,
    }

    impl Denoiser<'_> {
        #[cfg_attr(not(feature = "dlss"), expect(clippy::unused_self, reason = "no-op without dlss"))]
        fn attach(
            &self,
            camera: &mut EntityCommands,
            options: &SkyRenderOptions,
            upscaling: sf_sky::Upscaling,
        ) -> bool {
            #[cfg(feature = "dlss")]
            if options.ray_tracing_denoiser && self.supported.is_some() {
                camera.insert(bevy_anti_alias::dlss::Dlss::<
                    bevy_anti_alias::dlss::DlssRayReconstructionFeature,
                > {
                    perf_quality_mode: crate::upscale::dlss::ray_reconstruction_mode(upscaling),
                    ..Default::default()
                });
                return true;
            }
            let _ = (camera, options, upscaling);
            false
        }

        fn detach(camera: &mut EntityCommands) {
            #[cfg(feature = "dlss")]
            camera.remove::<bevy_anti_alias::dlss::Dlss<
                bevy_anti_alias::dlss::DlssRayReconstructionFeature,
            >>();
            #[cfg(not(feature = "dlss"))]
            let _ = camera;
        }
    }

    /// Second step of leaving the ray-traced path: MSAA, one frame after
    /// the deferred prepass is gone (MSAA with deferred is invalid).
    fn restore_raster_msaa(
        mut commands: Commands,
        cameras: Query<(Entity, &CameraPath, &RasterSnapshot)>,
    ) {
        for (entity, path, snapshot) in &cameras {
            if path.0 == RenderPath::Raster {
                commands
                    .entity(entity)
                    .insert(snapshot.msaa)
                    .remove::<RasterSnapshot>();
            }
        }
    }

    /// On the ray-traced path, registers visible meshes with the ray tracer
    /// (their [`RayTracingMesh`] when they have one); on raster, unregisters
    /// the ones it added. Solari ignores visibility, so hidden entities stay
    /// out and leave when they become hidden. Meshes Solari can't use
    /// (layout other than position/normal/uv0/tangent, u16 indices) are
    /// skipped by Solari itself: loaders provide a [`RayTracingMesh`].
    #[allow(clippy::type_complexity)]
    fn mirror_meshes(
        mut commands: Commands,
        active: Res<ActiveRenderPath>,
        options: Res<SkyRenderOptions>,
        unmirrored: Query<
            (Entity, &Mesh3d, Option<&RayTracingMesh>, &InheritedVisibility),
            (
                With<MeshMaterial3d<StandardMaterial>>,
                Without<RaytracingMesh3d>,
                Without<NoRayTracing>,
            ),
        >,
        hidden: Query<
            (Entity, &InheritedVisibility),
            (With<MirroredRayTracing>, Changed<InheritedVisibility>),
        >,
        mirrored: Query<Entity, With<MirroredRayTracing>>,
    ) {
        if active.0 == RenderPath::RayTraced && options.auto_ray_tracing_meshes {
            for (entity, mesh, variant, visibility) in &unmirrored {
                if visibility.get() {
                    let mesh = variant.map_or(&mesh.0, |v| &v.0).clone();
                    commands
                        .entity(entity)
                        .try_insert((RaytracingMesh3d(mesh), MirroredRayTracing));
                }
            }
            for (entity, visibility) in &hidden {
                if !visibility.get() {
                    commands
                        .entity(entity)
                        .try_remove::<(RaytracingMesh3d, MirroredRayTracing)>();
                }
            }
        } else if active.is_changed() || options.is_changed() {
            for entity in &mirrored {
                commands
                    .entity(entity)
                    .try_remove::<(RaytracingMesh3d, MirroredRayTracing)>();
            }
        }
    }
}
