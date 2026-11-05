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
use bevy_camera::Camera3d;
use bevy_camera::visibility::RenderLayers;
use bevy_ecs::prelude::*;

use crate::{ActiveRenderPath, RenderPathSetting};

/// Whether this build and GPU can run the ray-traced path. Set once at
/// startup (plugin `finish`).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RayTracingSupport(pub bool);

/// Another system needs the ray-traced scene (BLAS/TLAS) on the raster path:
/// dynamic probe GI (`sf_gi_bevy` sets it while a GI volume exists). With it
/// the visible meshes are mirrored into `RaytracingMesh3d` as on the
/// ray-traced path (feature `solari`; without it nothing is mirrored).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GiActive(pub bool);

/// The live switch of the dynamic probe GI (`sf_gi_bevy`; a console's `render.gi`). Present
/// only while the GI can run (its plugin is added and the GPU has the ray-tracing features).
/// False: volumes stay (and with them [`GiActive`]: the ray-traced scene is kept for a quick
/// return) but are neither traced nor shown; true again re-traces them from scratch.
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct GiEnabled(pub bool);

impl Default for GiEnabled {
    fn default() -> Self {
        Self(true)
    }
}

/// The per-entity ray-tracing markers live with the loader that builds the
/// variants (`sf_model_bevy`), so spawners that don't depend on this crate
/// (world streaming) can attach them.
pub use sf_model_bevy::{GiSkipEmissive, NoRayTracing, RayTracingMesh};

/// A 3D camera whose render path this plugin manages (G-buffer, MSAA,
/// prepasses, Solari settings). [`crate::SkyCamera`] requires it; interior
/// cameras without a sky add it alone.
#[derive(Component, Clone, Copy, Debug, Default)]
#[require(Camera3d)]
pub struct RenderPathCamera;

/// The layers the render-path cameras see together (no `RenderLayers`
/// means layer 0); none at all without a camera.
#[cfg_attr(not(feature = "solari"), allow(dead_code))]
pub(crate) fn camera_layers<'a>(cameras: impl Iterator<Item = Option<&'a RenderLayers>>) -> RenderLayers {
    cameras.fold(RenderLayers::none(), |all, layers| {
        all.union(layers.unwrap_or(&RenderLayers::default()))
    })
}

/// Whether a mesh on `layers` (none: layer 0) belongs in the ray-traced
/// scene of cameras seeing `view`: its worldspace is the one shown.
#[cfg_attr(not(feature = "solari"), allow(dead_code))]
pub(crate) fn traced_by(view: &RenderLayers, layers: Option<&RenderLayers>) -> bool {
    view.intersects(layers.unwrap_or(&RenderLayers::default()))
}

/// The render path systems (camera setup, mesh mirroring); upscaling runs
/// after them.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PathSystems;

pub(crate) fn build(app: &mut App) {
    app.init_resource::<RayTracingSupport>();
    app.init_resource::<GiActive>();
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
    use std::collections::HashMap;

    use bevy_asset::{AssetId, AssetMut, Assets, Handle};
    use bevy_camera::{Camera3d, CameraMainTextureUsages};
    use bevy_color::LinearRgba;
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
    use bevy_transform::components::Transform;

    use bevy_camera::visibility::{InheritedVisibility, RenderLayers};

    use super::{GiActive, GiSkipEmissive, NoRayTracing, RayTracingMesh, RayTracingSupport, RenderPathCamera};
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

    /// On a mirrored [`GiSkipEmissive`] mesh (raster path, probe GI): the
    /// child entity that carries its ray-traced instance instead, with an
    /// emission-free copy of its material. The child has no `Mesh3d`, so it
    /// is never rasterized and the mesh keeps its own look on screen.
    #[derive(Component, Clone, Copy, Debug)]
    pub(super) struct GiEmissiveProxy(Entity);

    /// `material` without emission: what probe GI sees of a [`GiSkipEmissive`] mesh.
    pub(super) fn without_emission(material: &StandardMaterial) -> StandardMaterial {
        StandardMaterial {
            emissive: LinearRgba::BLACK,
            emissive_texture: None,
            ..material.clone()
        }
    }

    /// Registers `entity` with the ray tracer. A [`GiSkipEmissive`] mesh on
    /// the raster path gets its instance on a child proxy with
    /// [`without_emission`] of its material (one copy per source material,
    /// kept in `stripped`); false when that material is not loaded yet (retry
    /// next frame). On the ray-traced path every mesh keeps its own material:
    /// there an emissive surface may be the only light (Solari has no point
    /// lights).
    pub(super) fn mirror_one(
        commands: &mut Commands,
        materials: &mut Assets<StandardMaterial>,
        stripped: &mut HashMap<AssetId<StandardMaterial>, Handle<StandardMaterial>>,
        entity: Entity,
        mesh: Handle<bevy_mesh::Mesh>,
        material: &MeshMaterial3d<StandardMaterial>,
        skip_emissive: bool,
    ) -> bool {
        if !skip_emissive {
            commands
                .entity(entity)
                .try_insert((RaytracingMesh3d(mesh), MirroredRayTracing));
            return true;
        }
        let copy = if let Some(copy) = stripped.get(&material.id()) {
            copy.clone()
        } else {
            let Some(source) = materials.get(&material.0) else {
                return false;
            };
            let copy = materials.add(without_emission(source));
            stripped.insert(material.id(), copy.clone());
            copy
        };
        let proxy = commands
            .spawn((
                RaytracingMesh3d(mesh),
                MeshMaterial3d(copy),
                Transform::IDENTITY,
                ChildOf(entity),
            ))
            .id();
        commands
            .entity(entity)
            .try_insert((MirroredRayTracing, GiEmissiveProxy(proxy)));
        true
    }

    /// Undoes [`mirror_one`].
    pub(super) fn unmirror(commands: &mut Commands, entity: Entity, proxy: Option<&GiEmissiveProxy>) {
        if let Some(proxy) = proxy {
            commands.entity(proxy.0).try_despawn();
            commands.entity(entity).try_remove::<GiEmissiveProxy>();
        }
        commands
            .entity(entity)
            .try_remove::<(RaytracingMesh3d, MirroredRayTracing)>();
    }

    pub(super) fn build(app: &mut App) {
        // Headless (no renderer, e.g. unit tests): nothing to ray trace, and
        // Solari's plugins need the render app. The path resolves to raster.
        if app.get_sub_app(RenderApp).is_none() {
            return;
        }
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

    /// On the ray-traced path, or on raster while [`GiActive`], registers
    /// visible meshes with the ray tracer (their [`RayTracingMesh`] when they
    /// have one); otherwise unregisters the ones it added. Solari ignores visibility, so hidden entities stay
    /// out and leave when they become hidden. Meshes Solari can't use
    /// (layout other than position/normal/uv0/tangent, u16 indices) are
    /// skipped by Solari itself: loaders provide a [`RayTracingMesh`].
    ///
    /// Only meshes the render-path cameras' `RenderLayers` see are traced:
    /// each loaded worldspace draws in its own layer, so the other
    /// worldspaces (an exterior kept loaded behind an interior) stay out of
    /// the BLAS/TLAS, and leave it when the camera changes worldspace.
    ///
    /// A [`GiSkipEmissive`] mesh on the raster path is traced through a child
    /// proxy without emission ([`mirror_one`]); a path switch or a newly added
    /// flag re-mirrors it, so Solari always sees the mesh's own material.
    #[allow(clippy::type_complexity, clippy::too_many_arguments)]
    fn mirror_meshes(
        mut commands: Commands,
        active: Res<ActiveRenderPath>,
        options: Res<SkyRenderOptions>,
        gi: Res<GiActive>,
        cameras: Query<Option<&RenderLayers>, With<RenderPathCamera>>,
        mut seen: Local<Option<RenderLayers>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut stripped: Local<HashMap<AssetId<StandardMaterial>, Handle<StandardMaterial>>>,
        unmirrored: Query<
            (
                Entity,
                &Mesh3d,
                Option<&RayTracingMesh>,
                &MeshMaterial3d<StandardMaterial>,
                Has<GiSkipEmissive>,
                &InheritedVisibility,
                Option<&RenderLayers>,
            ),
            (
                Without<RaytracingMesh3d>,
                Without<MirroredRayTracing>,
                Without<NoRayTracing>,
            ),
        >,
        changed: Query<
            (Entity, &InheritedVisibility, Option<&RenderLayers>, Option<&GiEmissiveProxy>),
            (
                With<MirroredRayTracing>,
                Or<(Changed<InheritedVisibility>, Changed<RenderLayers>)>,
            ),
        >,
        flagged: Query<(Entity, Option<&GiEmissiveProxy>, Ref<GiSkipEmissive>), With<MirroredRayTracing>>,
        mirrored: Query<
            (Entity, &InheritedVisibility, Option<&RenderLayers>, Option<&GiEmissiveProxy>),
            With<MirroredRayTracing>,
        >,
    ) {
        let wanted = active.0 == RenderPath::RayTraced || gi.0;
        if wanted && options.auto_ray_tracing_meshes {
            let view = super::camera_layers(cameras.iter());
            let view_changed = seen.as_ref() != Some(&view);
            let raster = active.0 != RenderPath::RayTraced;
            for (entity, mesh, variant, material, skip, visibility, layers) in &unmirrored {
                if visibility.get() && super::traced_by(&view, layers) {
                    let mesh = variant.map_or(&mesh.0, |v| &v.0).clone();
                    mirror_one(
                        &mut commands,
                        &mut materials,
                        &mut stripped,
                        entity,
                        mesh,
                        material,
                        skip && raster,
                    );
                }
            }
            let leaving = |(entity, visibility, layers, proxy): (
                Entity,
                &InheritedVisibility,
                Option<&RenderLayers>,
                Option<&GiEmissiveProxy>,
            )| {
                (!visibility.get() || !super::traced_by(&view, layers)).then_some((entity, proxy.copied()))
            };
            let mut gone: Vec<(Entity, Option<GiEmissiveProxy>)> = if view_changed {
                mirrored.iter().filter_map(leaving).collect()
            } else {
                changed.iter().filter_map(leaving).collect()
            };
            // Re-mirror flagged meshes whose rule changed: the path switched
            // (proxy on raster, own material on Solari) or the flag is new.
            gone.extend(
                flagged
                    .iter()
                    .filter(|(_, _, flag)| active.is_changed() || flag.is_added())
                    .map(|(entity, proxy, _)| (entity, proxy.copied())),
            );
            for (entity, proxy) in gone {
                unmirror(&mut commands, entity, proxy.as_ref());
            }
            *seen = Some(view);
        } else if active.is_changed() || options.is_changed() || gi.is_changed() {
            *seen = None;
            for (entity, _, _, proxy) in &mirrored {
                unmirror(&mut commands, entity, proxy);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use bevy_ecs::system::RunSystemOnce;

        fn glowing() -> StandardMaterial {
            StandardMaterial {
                emissive: LinearRgba::rgb(5.0, 2.0, 0.5),
                ..Default::default()
            }
        }

        /// Runs [`mirror_one`] on `entities` (with their `skip` rule) in one
        /// pass, as `mirror_meshes` does, sharing one copy cache.
        fn mirror(world: &mut World, entities: Vec<(Entity, bool)>) {
            world
                .run_system_once(
                    move |mut commands: Commands,
                          mut materials: ResMut<Assets<StandardMaterial>>,
                          mut stripped: Local<HashMap<AssetId<StandardMaterial>, Handle<StandardMaterial>>>,
                          query: Query<&MeshMaterial3d<StandardMaterial>>| {
                        for &(entity, skip) in &entities {
                            let material = query.get(entity).unwrap();
                            assert!(mirror_one(
                                &mut commands,
                                &mut materials,
                                &mut stripped,
                                entity,
                                Handle::default(),
                                material,
                                skip,
                            ));
                        }
                    },
                )
                .unwrap();
        }

        /// A flagged mesh is traced through a child proxy whose material has
        /// no emission (and is otherwise the same); the mesh keeps its own
        /// material and is not itself an instance. An unflagged mesh is its
        /// own instance with its own, unchanged material. Flagged meshes that
        /// share a material share one copy, and unmirroring removes the proxy.
        #[test]
        fn skip_emissive_traces_an_emission_free_copy() {
            let mut world = World::new();
            world.init_resource::<Assets<StandardMaterial>>();
            let source = world.resource_mut::<Assets<StandardMaterial>>().add(glowing());
            let flame = world.spawn(MeshMaterial3d(source.clone())).id();
            let glass = world.spawn(MeshMaterial3d(source.clone())).id();
            let plain = world.spawn(MeshMaterial3d(source.clone())).id();
            mirror(&mut world, vec![(flame, true), (glass, true), (plain, false)]);

            let materials = world.resource::<Assets<StandardMaterial>>();
            assert_eq!(materials.get(&source).unwrap().emissive, glowing().emissive);
            let proxy_of = |entity: Entity| world.get::<GiEmissiveProxy>(entity).unwrap().0;
            let (flame_proxy, glass_proxy) = (proxy_of(flame), proxy_of(glass));
            for (mesh, proxy) in [(flame, flame_proxy), (glass, glass_proxy)] {
                assert!(world.get::<RaytracingMesh3d>(mesh).is_none());
                assert!(world.get::<MirroredRayTracing>(mesh).is_some());
                assert_eq!(world.get::<MeshMaterial3d<StandardMaterial>>(mesh).unwrap().id(), source.id());
                assert!(world.get::<RaytracingMesh3d>(proxy).is_some());
                assert!(world.get::<Mesh3d>(proxy).is_none());
                assert_eq!(world.get::<ChildOf>(proxy).unwrap().parent(), mesh);
            }
            let copy = world.get::<MeshMaterial3d<StandardMaterial>>(flame_proxy).unwrap().id();
            assert_ne!(copy, source.id());
            assert_eq!(world.get::<MeshMaterial3d<StandardMaterial>>(glass_proxy).unwrap().id(), copy);
            let copied = materials.get(copy).unwrap();
            assert_eq!(copied.emissive, LinearRgba::BLACK);
            assert!(copied.emissive_texture.is_none());
            assert_eq!(copied.base_color, glowing().base_color);

            assert!(world.get::<RaytracingMesh3d>(plain).is_some());
            assert!(world.get::<GiEmissiveProxy>(plain).is_none());
            assert_eq!(world.get::<MeshMaterial3d<StandardMaterial>>(plain).unwrap().id(), source.id());

            world
                .run_system_once(move |mut commands: Commands| {
                    unmirror(&mut commands, flame, Some(&GiEmissiveProxy(flame_proxy)));
                })
                .unwrap();
            assert!(world.get_entity(flame_proxy).is_err());
            assert!(world.get::<GiEmissiveProxy>(flame).is_none());
            assert!(world.get::<MirroredRayTracing>(flame).is_none());
        }
    }
}
