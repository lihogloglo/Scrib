//! The vendored mechanism of the lamp shadow cache (ROADMAP 1.25 CW-4 S2; `bevy_pbr` patch
//! "own, CW-4: lamp shadow cache", `render/point_shadow_cache.rs`): which faces get a view and a
//! phase, which are drawn and how, what extraction hands the render world, and where the copies
//! are recorded. No GPU here: `prepare_lights` itself needs a device, so its per-face decision is
//! tested through the functions it calls (`point_shadow_view_faces`, `point_shadow_face_plan`).

use bevy_camera::visibility::{CubemapVisibleEntities, ViewVisibility};
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemId;
use bevy_light::{DirectionalLightShadowMap, PointLight, PointLightShadowMap};
use bevy_pbr::{
    ALL_POINT_SHADOW_FACES, CachedPointShadowCopy, CachedPointShadowFaces, CachedPointShadowMap,
    ExtractedCachedPointShadowMap, ExtractedPointLight, PointShadowCacheFeedback, PointShadowCacheStore,
    PointShadowFacePlan, ShadowCasterChanges, extract_cached_point_shadow_maps, extract_lights,
    extract_shadow_caster_changes, point_shadow_face_plan, point_shadow_view_faces,
};
use bevy_render::MainWorld;
use bevy_render::sync_world::{MainEntity, RenderEntity};
use bevy_render::view::{RenderShadowMapVisibleEntities, RetainedViewEntity};
use bevy_transform::components::GlobalTransform;
use std::path::PathBuf;

/// Faces with a view (and so a retained shadow phase) and faces drawn, as `prepare_lights` and
/// `skip_unchanged_shadow_views` decide them.
fn views_and_draws(cached: Option<&CachedPointShadowMap>, fresh: bool) -> (usize, usize) {
    let faces = cached.map(CachedPointShadowMap::faces);
    let mask = point_shadow_view_faces(faces.as_ref());
    let views = (0..6).filter(|f| mask & (1 << f) != 0).count();
    let draws = (0..6)
        .filter(|&f| matches!(point_shadow_face_plan(faces.as_ref(), fresh, f), PointShadowFacePlan::Draw { .. }))
        .count();
    (views, draws)
}

#[test]
fn a_stock_lamp_keeps_six_face_views() {
    assert_eq!(point_shadow_view_faces(None), ALL_POINT_SHADOW_FACES);
    assert_eq!(views_and_draws(None, true), (6, 0), "stock faces are drawn by skip_unchanged, not by a plan");
    assert!((0..6).all(|f| point_shadow_face_plan(None, true, f) == PointShadowFacePlan::Stock));
}

#[test]
fn mask_zero_has_no_face_view_and_no_phase() {
    let cached = CachedPointShadowMap::default();
    assert_eq!(views_and_draws(Some(&cached), true), (0, 0));
    let faces = cached.faces();
    assert!((0..6).all(|f| point_shadow_face_plan(Some(&faces), true, f) == PointShadowFacePlan::NoView));
}

#[test]
fn mask_0b101_has_two_face_views() {
    let cached = CachedPointShadowMap { view_faces: 0b101, draw_faces: 0b001, load_faces: 0b001, ..Default::default() };
    assert_eq!(views_and_draws(Some(&cached), true), (2, 1));
    let faces = cached.faces();
    assert_eq!(point_shadow_face_plan(Some(&faces), true, 0), PointShadowFacePlan::Draw { load: true });
    assert_eq!(point_shadow_face_plan(Some(&faces), true, 1), PointShadowFacePlan::NoView);
    assert_eq!(point_shadow_face_plan(Some(&faces), true, 2), PointShadowFacePlan::Kept);
    // Not written this frame: the views stay, nothing is drawn.
    assert_eq!(views_and_draws(Some(&cached), false), (2, 0));
}

#[test]
fn masks_are_sanitised_draw_in_view_load_in_draw_six_faces() {
    let cached = CachedPointShadowMap { view_faces: 0b1100_0001, draw_faces: 0b10, load_faces: 0b1001, ..Default::default() };
    assert_eq!(cached.faces(), CachedPointShadowFaces { view: 0b11, draw: 0b10, load: 0 });
    let capture = CachedPointShadowMap { view_faces: 0b11_1111, draw_faces: 0b11_1111, ..Default::default() };
    let faces = capture.faces();
    assert!((0..6).all(|f| point_shadow_face_plan(Some(&faces), true, f) == PointShadowFacePlan::Draw { load: false }));
}

fn extracted_light(shadow_maps_enabled: bool) -> ExtractedPointLight {
    ExtractedPointLight {
        color: bevy_color::LinearRgba::default(),
        intensity: 1.0,
        range: 10.0,
        radius: 0.0,
        transform: GlobalTransform::default(),
        shadow_maps_enabled,
        contact_shadows_enabled: false,
        shadow_depth_bias: 0.0,
        shadow_normal_bias: 0.0,
        shadow_map_near_z: 0.1,
        spot_light_angles: None,
        volumetric: false,
        soft_shadows_enabled: false,
        affects_lightmapped_mesh_diffuse: true,
        shadow_fade: 1.0,
    }
}

/// One extraction, as the render app runs it (the same system across frames, so its change
/// detection and locals carry over), then the main world moves on a tick.
fn extract(render: &mut World, system: SystemId) {
    render.run_system(system).expect("extract system runs");
    render.flush();
    render.resource_mut::<MainWorld>().increment_change_tick();
}

/// A plan executes once: draws and copies apply on the frame the host wrote the component; an
/// untouched component keeps its masks with nothing to do; removing it hands the light back to
/// the stock flag. The store size reaches the render world.
#[test]
fn extraction_carries_a_written_plan_once_and_restores_the_stock_flag() {
    let mut render = World::new();
    render.init_resource::<MainWorld>();
    let system = render.register_system(extract_cached_point_shadow_maps);
    let render_light = render.spawn(extracted_light(true)).id();
    let main_light = {
        let mut main = render.resource_mut::<MainWorld>();
        main.insert_resource(PointShadowCacheStore { layers: 24 });
        main.spawn((
            PointLight { shadow_maps_enabled: false, ..Default::default() },
            RenderEntity::from(render_light),
            CachedPointShadowMap {
                view_faces: 0b11,
                draw_faces: 0b01,
                load_faces: 0b01,
                copies: vec![CachedPointShadowCopy::Restore { face: 0, layer: 3 }],
                plan: 7,
            },
        ))
        .id()
    };

    extract(&mut render, system);
    let got = render.get::<ExtractedCachedPointShadowMap>(render_light).expect("extracted");
    assert!(got.fresh, "a new component is a written plan");
    assert_eq!(got.faces, CachedPointShadowFaces { view: 0b11, draw: 0b01, load: 0b01 });
    assert_eq!(got.copies, vec![CachedPointShadowCopy::Restore { face: 0, layer: 3 }]);
    assert_eq!(got.plan, 7);
    assert_eq!(render.resource::<PointShadowCacheStore>().layers, 24);

    // Untouched: the masks stay, the copies do not repeat.
    extract(&mut render, system);
    let got = render.get::<ExtractedCachedPointShadowMap>(render_light).expect("still extracted");
    assert!(!got.fresh);
    assert!(got.copies.is_empty());
    assert_eq!(got.faces.view, 0b11);

    // Written again (same values): fresh again.
    render.resource_mut::<MainWorld>().get_mut::<CachedPointShadowMap>(main_light).expect("component").plan = 8;
    extract(&mut render, system);
    let got = render.get::<ExtractedCachedPointShadowMap>(render_light).expect("extracted");
    assert!(got.fresh);
    assert_eq!(got.plan, 8);

    // Removed: the render copy goes, and the light's shadow flag is the stock one again.
    render.resource_mut::<MainWorld>().entity_mut(main_light).remove::<CachedPointShadowMap>();
    extract(&mut render, system);
    assert!(render.get::<ExtractedCachedPointShadowMap>(render_light).is_none());
    assert!(!render.get::<ExtractedPointLight>(render_light).expect("light").shadow_maps_enabled);
}

#[test]
fn feedback_starts_empty() {
    assert!(PointShadowCacheFeedback::default().drain().is_empty());
}

fn vendored(file: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../vendor/bevy_pbr/src").join(file);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// The copies are recorded before any shadow pass: point shadow views run inside
/// `camera_driver` (render graph schedule, as root non-camera views), and the copy system is
/// ordered before it in the same set. Source check (as `shader_tests`): the order is not
/// observable without a GPU.
#[test]
fn copies_are_recorded_before_the_shadow_passes() {
    let squash = |s: &str| s.split_whitespace().collect::<String>();
    let lib = squash(&vendored("lib.rs"));
    assert!(
        lib.contains(
            "record_cached_point_shadow_copies.in_set(bevy_render::renderer::RenderGraphSystems::Render).before(bevy_core_pipeline::schedule::camera_driver)"
        ),
        "lib.rs: the copy system is no longer ordered before camera_driver"
    );
    let cache = squash(&vendored("render/point_shadow_cache.rs"));
    assert!(cache.contains("pending.push_encoder(encoder,\"point_shadow_cache_copies\");"), "copies no longer queued as one encoder");
}

/// Owner run 2026-11-17 (Balmora 22:00, `set shadow.lamp.cache.enabled on`): `prepare_lights`
/// re-arms every existing `ShadowView` each frame, and a cached face kept across frames with
/// `load` (built without a clear value) then panicked in `get_attachment` (`unwrap()` on `None`,
/// `texture_attachment.rs`). The reset must re-arm only aspects that have a clear value. Source
/// check (as above): building a `TextureView` needs a device (wgpu's noop backend is a feature
/// that would split the workspace's Bevy build).
#[test]
fn a_frame_reset_never_rearms_a_load_only_depth_attachment() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../vendor/bevy_render/src/texture/texture_attachment.rs");
    let src = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let reset = src.split("pub fn prepare_for_new_frame(&self) {").last().expect("prepare_for_new_frame");
    let reset = &reset[..reset.find("\n    }\n").expect("end of prepare_for_new_frame")];
    let squash = |s: &str| s.split_whitespace().collect::<String>();
    let reset = squash(reset);
    assert!(!reset.contains(".store(true,"), "a frame reset re-arms a clear without a clear value");
    for aspect in ["depth", "stencil"] {
        assert!(
            reset.contains(&format!("{aspect}_is_first_call.store({aspect}_clear_value.is_some(),")),
            "{aspect}: the reset must re-arm only with a clear value"
        );
    }
    let cache = squash(&vendored("render/point_shadow_cache.rs"));
    assert!(cache.contains("(!load).then_some(0.0),"), "cached faces no longer load without a clear value: recheck this test");
}

/// The render app's light extraction (`extract_shadow_caster_changes`, `extract_lights`,
/// `extract_cached_point_shadow_maps`) run against a main world, and the face views
/// `prepare_lights` would give each shadowed point light: six on the stock path, the view faces
/// when cached (`point_shadow_view_faces`). The 2026-11-17 crash ("Failed to get point light
/// visible entity for face") was a face view without its extracted list.
pub(crate) struct RenderExtract {
    render: World,
    systems: [SystemId; 3],
}

impl RenderExtract {
    pub(crate) fn new() -> Self {
        let mut render = World::new();
        render.init_resource::<MainWorld>();
        render.init_resource::<ShadowCasterChanges>();
        let systems = [
            render.register_system(extract_shadow_caster_changes),
            render.register_system(extract_lights),
            render.register_system(extract_cached_point_shadow_maps),
        ];
        Self { render, systems }
    }

    /// Gives the main-world light `light` a render entity (`sync_world`'s job) and the main world
    /// the shadow map resources `extract_lights` reads.
    pub(crate) fn link(&mut self, main: &mut World, light: Entity) -> Entity {
        main.init_resource::<PointLightShadowMap>();
        main.init_resource::<DirectionalLightShadowMap>();
        let render_light = self.render.spawn_empty().id();
        main.entity_mut(light).insert(RenderEntity::from(render_light));
        render_light
    }

    /// One extraction of `main`, as the render app runs it (the main world is swapped in), then
    /// the main world moves on a tick.
    pub(crate) fn extract(&mut self, main: &mut World) {
        std::mem::swap(main, &mut *self.render.resource_mut::<MainWorld>());
        for system in self.systems {
            self.render.run_system(system).expect("extract system runs");
            self.render.flush();
        }
        std::mem::swap(main, &mut *self.render.resource_mut::<MainWorld>());
        main.increment_change_tick();
    }

    /// Face views `prepare_lights` would create this frame with no visible-entity list:
    /// `(main-world light, face)`. Must always be empty.
    pub(crate) fn faces_without_list(&mut self) -> Vec<(Entity, u32)> {
        let mut missing = Vec::new();
        let mut lights = self.render.query::<(
            &ExtractedPointLight,
            &MainEntity,
            Option<&ExtractedCachedPointShadowMap>,
            Option<&RenderShadowMapVisibleEntities>,
        )>();
        for (light, main, cached, lists) in lights.iter(&self.render) {
            if !light.shadow_maps_enabled || light.spot_light_angles.is_some() {
                continue;
            }
            let views = point_shadow_view_faces(cached.map(|c| &c.faces));
            for face in (0..6).filter(|f| views & (1 << f) != 0) {
                let key = RetainedViewEntity { main_entity: *main, auxiliary_entity: None, subview_index: face };
                if !lists.is_some_and(|l| l.subviews.contains_key(&key)) {
                    missing.push((main.id(), face));
                }
            }
        }
        missing
    }

    /// The render world (extracted components).
    pub(crate) fn world(&self) -> &World {
        &self.render
    }

    /// The faces with an extracted list, as a mask.
    pub(crate) fn listed_faces(&self, render_light: Entity) -> u8 {
        self.render.get::<RenderShadowMapVisibleEntities>(render_light).map_or(0, |lists| {
            lists.subviews.keys().filter(|k| k.subview_index < 6).fold(0, |mask, k| mask | (1 << k.subview_index))
        })
    }
}

/// A lamp switching path keeps a list for every face view: stock (six) -> cached (its view
/// faces) -> stock again with the very lists it had before (its stock hash unchanged: the case
/// that crashed, the stock hash kept through the cached spell matched again) -> cached again
/// (lists re-extracted for the view faces, never the stale stock ones).
#[test]
fn a_lamp_changing_path_always_has_a_list_for_each_face_view() {
    let mut main = World::new();
    let casters = [main.spawn_empty().id(), main.spawn_empty().id()];
    let lamp = main
        .spawn((
            PointLight { shadow_maps_enabled: true, ..Default::default() },
            GlobalTransform::IDENTITY,
            ViewVisibility::VISIBLE,
        ))
        .id();
    let mut extract = RenderExtract::new();
    let render_light = extract.link(&mut main, lamp);
    // The stock scan's lists, rewritten every frame it runs (Bevy derefs them mutably).
    let stock_fill = |main: &mut World| {
        let mut lists = main.get_mut::<CubemapVisibleEntities>(lamp).expect("lists");
        for face in 0..6 {
            lists.get_mut(face).entities.clear();
        }
        lists.get_mut(0).entities.push(casters[0]);
        lists.get_mut(3).entities.push(casters[1]);
    };
    let frame = |main: &mut World, extract: &mut RenderExtract| {
        extract.extract(main);
        let missing = extract.faces_without_list();
        assert!(missing.is_empty(), "face views without a list: {missing:?}");
        extract.listed_faces(render_light)
    };

    for _ in 0..2 {
        stock_fill(&mut main);
        assert_eq!(frame(&mut main, &mut extract), ALL_POINT_SHADOW_FACES, "stock: six lists");
    }

    // Cached: the stock flag off, the host's lists on its capture faces, then a clean cube.
    main.get_mut::<PointLight>(lamp).expect("light").shadow_maps_enabled = false;
    main.entity_mut(lamp).insert(CachedPointShadowMap { view_faces: 0b1001, draw_faces: 0b1001, ..Default::default() });
    assert_eq!(frame(&mut main, &mut extract), 0b1001, "cached: the view faces' lists");
    main.get_mut::<CachedPointShadowMap>(lamp).expect("cached").view_faces = 0;
    main.get_mut::<CachedPointShadowMap>(lamp).expect("cached").draw_faces = 0;
    assert_eq!(frame(&mut main, &mut extract), 0, "clean cube: no view, no list");
    assert_eq!(frame(&mut main, &mut extract), 0, "untouched: kept");

    // Back to the stock path (moved, lost cache, switch off) with the same casters: same hash.
    main.entity_mut(lamp).remove::<CachedPointShadowMap>();
    main.get_mut::<PointLight>(lamp).expect("light").shadow_maps_enabled = true;
    stock_fill(&mut main);
    assert_eq!(frame(&mut main, &mut extract), ALL_POINT_SHADOW_FACES, "stock again: six lists");
    stock_fill(&mut main);
    assert_eq!(frame(&mut main, &mut extract), ALL_POINT_SHADOW_FACES);

    // Cached again with the lists untouched: re-extracted for its view faces only.
    main.get_mut::<PointLight>(lamp).expect("light").shadow_maps_enabled = false;
    main.entity_mut(lamp).insert(CachedPointShadowMap { view_faces: 0b1001, ..Default::default() });
    assert_eq!(frame(&mut main, &mut extract), 0b1001, "cached again: its view faces' lists, not the stock six");

    // Cache removed while the budget turns its shadow off: no view, nothing missing.
    main.entity_mut(lamp).remove::<CachedPointShadowMap>();
    main.get_mut::<PointLight>(lamp).expect("light").shadow_maps_enabled = false;
    assert_eq!(frame(&mut main, &mut extract), 0, "unshadowed: lists dropped");
}

/// CW-13a: `record_batched_point_shadows` is ordered after `record_cached_point_shadow_copies`
/// and before `camera_driver` in `RenderGraphSystems::Render`.
#[test]
fn batched_point_shadows_run_after_copies_and_before_camera_driver() {
    let squash = |s: &str| s.split_whitespace().collect::<String>();
    let lib = squash(&vendored("lib.rs"));
    assert!(
        lib.contains(
            "record_batched_point_shadows.in_set(bevy_render::renderer::RenderGraphSystems::Render).after(record_cached_point_shadow_copies).before(bevy_core_pipeline::schedule::camera_driver)"
        ),
        "lib.rs: record_batched_point_shadows is not ordered after copies and before camera_driver"
    );
}

/// CW-13a: `PointShadowBatchedPass` is registered as `shadow.lamp.batched_pass` (default off) and
/// binds live to the Bevy resource.
#[test]
fn batched_pass_tunable_binds_live() {
    use sf_tunables_bevy::{SetBy, SetTunable};

    let mut app = bevy_app::App::new();
    app.add_plugins((sf_tunables_bevy::TunablesPlugin, crate::ShadowQualityPlugin))
        .init_resource::<bevy_pbr::PointShadowBatchedPass>();
    app.update();
    assert!(!app.world().resource::<bevy_pbr::PointShadowBatchedPass>().0, "default off until A/B confirms win");

    app.world_mut().write_message(SetTunable::new("shadow.lamp.batched_pass", "on", SetBy::Console));
    app.update();
    assert!(app.world().resource::<bevy_pbr::PointShadowBatchedPass>().0, "live toggle turns batched pass on");

    app.world_mut().write_message(SetTunable::new("shadow.lamp.batched_pass", "off", SetBy::Console));
    app.update();
    assert!(!app.world().resource::<bevy_pbr::PointShadowBatchedPass>().0, "live toggle turns batched pass back off");
}

/// CW-13a (Slice A1b): `sync_batched_shadow_view_marker` swaps `RootNonCameraView(Core3dShadows)`
/// <-> `BatchedShadowView` cleanly on live toggles (both stock and cached views), and
/// `prepare_lights` / `point_shadow_cache` insert `BatchedShadowView` directly when enabled so
/// moving lamps and load-state flips avoid per-frame archetype churn.
#[test]
fn batched_shadow_view_marker_swaps_live_and_avoids_churn() {
    use bevy_core_pipeline::schedule::{Core3dShadows, RootNonCameraView};
    use bevy_ecs::schedule::ScheduleLabel;
    use bevy_pbr::{BatchedShadowView, sync_batched_shadow_view_marker};

    let mut world = World::new();
    let view = world.spawn(RootNonCameraView(Core3dShadows.intern())).id();

    let run = |world: &mut World, use_batched: bool| {
        let mut sched = Schedule::default();
        sched.add_systems(move |mut commands: Commands, q: Query<(Entity, Has<RootNonCameraView>, Has<BatchedShadowView>)>| {
            for (e, has_root, has_batched) in &q {
                sync_batched_shadow_view_marker(&mut commands, e, use_batched, has_root, has_batched);
            }
        });
        sched.run(world);
    };

    // Stock -> batched.
    run(&mut world, true);
    assert!(world.get::<RootNonCameraView>(view).is_none());
    assert!(world.get::<BatchedShadowView>(view).is_some());

    // Idempotent second frame on batched.
    run(&mut world, true);
    assert!(world.get::<RootNonCameraView>(view).is_none());
    assert!(world.get::<BatchedShadowView>(view).is_some());

    // Live toggle back to stock.
    run(&mut world, false);
    assert!(world.get::<BatchedShadowView>(view).is_none());
    let root = world.get::<RootNonCameraView>(view).expect("restored RootNonCameraView");
    assert_eq!(root.0, Core3dShadows.intern());

    // Verify `prepare_lights` and `point_shadow_cache` insert `BatchedShadowView` directly when `use_batched`.
    let squash = |s: &str| s.split_whitespace().collect::<String>();
    let light = squash(&vendored("render/light.rs"));
    let cache = squash(&vendored("render/point_shadow_cache.rs"));
    assert!(
        light.contains("ifuse_batched{commands.entity(view_light_entity).insert(BatchedShadowView);}"),
        "light.rs: stock point/spot views should insert BatchedShadowView directly when batched"
    );
    assert!(
        cache.contains("ifinputs.use_batched{commands.entity(view_entity).insert(BatchedShadowView);}"),
        "point_shadow_cache.rs: cached views should insert BatchedShadowView directly when batched"
    );
}

/// CW-13b: `record_point_shadow_atlas` is ordered after `record_batched_point_shadows` and before
/// `camera_driver` in `RenderGraphSystems::Render`.
#[test]
fn point_shadow_atlas_runs_after_batched_and_before_camera_driver() {
    let squash = |s: &str| s.split_whitespace().collect::<String>();
    let lib = squash(&vendored("lib.rs"));
    assert!(
        lib.contains(
            "record_point_shadow_atlas.in_set(bevy_render::renderer::RenderGraphSystems::Render).after(record_batched_point_shadows).before(bevy_core_pipeline::schedule::camera_driver)"
        ),
        "lib.rs: record_point_shadow_atlas is not ordered after batched point shadows and before camera_driver"
    );
}

/// CW-13b: CPU mirror of `shadows.wesl::point_shadow_atlas_project` matches Bevy's exact
/// `CUBE_MAP_FACES` `looking_at` + guard-band `perspective_infinite_reverse` projection across
/// all 6 faces (`+X, -X, +Y, -Y, -Z, +Z`).
#[test]
fn atlas_projection_matches_bevy_cube_faces() {
    use bevy_camera::primitives::{CUBE_MAP_FACES, CubeMapFace};
    use bevy_math::{Mat4, Vec2, Vec3, Vec4, proj};
    use bevy_transform::components::Transform;

    let project_shader = |frag_ls: Vec3| -> (usize, Vec2, f32) {
        let a = frag_ls.abs();
        if a.x >= a.y && a.x >= a.z {
            if frag_ls.x >= 0.0 {
                (0, Vec2::new(frag_ls.z / a.x, frag_ls.y / a.x), a.x)
            } else {
                (1, Vec2::new(-frag_ls.z / a.x, frag_ls.y / a.x), a.x)
            }
        } else if a.y >= a.z {
            if frag_ls.y >= 0.0 {
                (2, Vec2::new(frag_ls.x / a.y, frag_ls.z / a.y), a.y)
            } else {
                (3, Vec2::new(frag_ls.x / a.y, -frag_ls.z / a.y), a.y)
            }
        } else if frag_ls.z < 0.0 {
            (4, Vec2::new(frag_ls.x / a.z, frag_ls.y / a.z), a.z)
        } else {
            (5, Vec2::new(-frag_ls.x / a.z, frag_ls.y / a.z), a.z)
        }
    };

    let near = 0.1_f32;
    let guard_texels = 2.0_f32;
    for tile_px in [128u32, 256, 512] {
        let tan_half = bevy_pbr::atlas_guard_tan_half(tile_px, guard_texels);
        let inv_guard_tan = bevy_pbr::atlas_guard_inv_tan(tile_px, guard_texels);
        let projection = proj::perspective_infinite_reverse(2.0 * tan_half.atan(), 1.0, near);

        let samples = [
            Vec3::new(3.0, 0.7, -1.2),  // +X
            Vec3::new(-4.0, -1.1, 0.9), // -X
            Vec3::new(0.6, 5.0, 1.8),   // +Y
            Vec3::new(-1.3, -3.5, 0.4), // -Y
            Vec3::new(0.8, -0.5, -2.5), // -Z (CUBE_MAP_FACES[4])
            Vec3::new(-0.9, 1.1, 3.2),  // +Z (CUBE_MAP_FACES[5])
        ];
        for (expected_face, p) in samples.into_iter().enumerate() {
            let (face, face_ndc, major) = project_shader(p);
            assert_eq!(face, expected_face, "tile_px={tile_px} sample={p:?}");
            let CubeMapFace { target, up } = CUBE_MAP_FACES[face];
            let view_from_world = Mat4::from(Transform::IDENTITY.looking_at(target, up).compute_affine().inverse());
            let clip = projection * view_from_world * Vec4::from((p, 1.0));
            let ndc = clip.truncate() / clip.w;
            let expected_uv = Vec2::new(ndc.x * 0.5 + 0.5, -ndc.y * 0.5 + 0.5);
            let shader_uv = face_ndc * inv_guard_tan * Vec2::new(0.5, -0.5) + Vec2::splat(0.5);
            assert!(
                (shader_uv - expected_uv).length() < 1e-5,
                "face {face} tile_px={tile_px}: shader_uv={shader_uv:?} vs expected={expected_uv:?}"
            );
            let shader_depth = near / major;
            assert!((ndc.z - shader_depth).abs() < 1e-5, "face {face}: depth {shader_depth} vs {}", ndc.z);
        }
    }
}

/// CW-13b: `pack_shadow_atlas_tile` packs kind, tile resolution, `soft_pcf_radius`, and `inv_guard_tan` accurately.
#[test]
fn atlas_tile_packing_round_trip() {
    let empty = bevy_pbr::pack_shadow_atlas_tile(bevy_pbr::ATLAS_KIND_EMPTY, [0, 0, 0], 4096, 2.0, 1.0);
    assert_eq!(empty.flags_and_size & 3, 0);

    for (kind, rect, atlas_size, pcf_radius, expected_pcf) in [
        (bevy_pbr::ATLAS_KIND_STATIC, [512, 1024, 512], 4096u16, 0.0_f32, 0.0_f32),
        (bevy_pbr::ATLAS_KIND_STATIC, [256, 768, 256], 4096u16, 1.0_f32, 1.0_f32),
        (bevy_pbr::ATLAS_KIND_LIVE, [128, 384, 128], 2048u16, 1.75_f32, 1.75_f32),
        (bevy_pbr::ATLAS_KIND_STATIC, [0, 0, 512], 4096u16, 5.0_f32, 1.75_f32),
    ] {
        let packed = bevy_pbr::pack_shadow_atlas_tile(kind, rect, u32::from(atlas_size), 2.0, pcf_radius);
        assert_eq!(packed.flags_and_size & 3, kind);
        assert_eq!((packed.flags_and_size >> 2) & 0x7FF, u32::from(rect[2]));
        let unpacked_pcf = f32::from(u8::try_from((packed.flags_and_size >> 13) & 7).expect("u3")) * 0.25;
        assert!((unpacked_pcf - expected_pcf).abs() < 1e-6, "pcf_radius={pcf_radius}: got {unpacked_pcf}");
        let unpacked_inv = f32::from(u16::try_from(packed.flags_and_size >> 16).expect("u16")) / 65535.0;
        let expected_inv = bevy_pbr::atlas_guard_inv_tan(u32::from(rect[2]), 2.0);
        assert!((unpacked_inv - expected_inv).abs() < 2e-5);
        assert!((packed.uv_offset.x - f32::from(rect[0]) / f32::from(atlas_size)).abs() < 1e-6);
        assert!((packed.uv_offset.y - f32::from(rect[1]) / f32::from(atlas_size)).abs() < 1e-6);
        assert!((packed.uv_scale - f32::from(rect[2]) / f32::from(atlas_size)).abs() < 1e-6);
    }
}

/// CW-13b: live toggle of `shadow.lamp.atlas.enabled` admits > 8 lamps into `AtlasPointShadowMap`,
/// caps `contact_shadows_enabled` to `shadow.lamp.contact_max`, confirms static captures to zero
/// views, promotes a moving lamp (carried torch) into the Live Atlas, and tears down cleanly when
/// toggled off.
#[test]
fn atlas_live_toggle_contact_cap_and_moving_lamp() {
    use bevy_camera::primitives::Aabb;
    use bevy_camera::visibility::{CubemapVisibleEntities, InheritedVisibility, ViewVisibility};
    use bevy_camera::{Camera, Camera3d};
    use bevy_math::Vec3;
    use bevy_mesh::Mesh3d;
    use bevy_pbr::{AtlasPointShadowMap, AtlasPointShadowReport, PointShadowAtlasConfig};
    use bevy_time::Time;
    use bevy_transform::components::GlobalTransform;
    use sf_tunables_bevy::{SetBy, SetTunable};

    let mut app = bevy_app::App::new();
    app.add_plugins((sf_tunables_bevy::TunablesPlugin, crate::ShadowQualityPlugin, crate::PointShadowPlugin))
        .init_resource::<Time>()
        .init_resource::<PointLightShadowMap>();

    // Configure High-like budget via live tunables (`ShadowQualityPlugin` binds `shadow.lamp.*`).
    for (name, val) in [
        ("shadow.lamp.count", "8"),
        ("shadow.lamp.fade_m", "40,60"),
        ("shadow.lamp.fade_time_s", "0"),
        ("shadow.lamp.contact", "on"),
        ("shadow.lamp.day_gate", "off"),
        ("shadow.lamp.atlas.enabled", "on"),
        ("shadow.lamp.contact_max", "4"),
    ] {
        app.world_mut().write_message(SetTunable::new(name, val, SetBy::Console));
    }

    // Camera at origin.
    app.world_mut().spawn((Camera::default(), Camera3d::default(), GlobalTransform::IDENTITY));

    // Spawn 12 budgeted lamps + 1 nearby static caster per lamp so each has non-empty faces.
    let mut lamp_entities = Vec::new();
    for i in 0..12u16 {
        let pos = Vec3::new((f32::from(i) + 1.0) * 2.5, 2.0, 0.0);
        let lamp = app
            .world_mut()
            .spawn((
                PointLight { intensity: 1500.0, range: 12.0, ..Default::default() },
                crate::BudgetedShadow::default(),
                GlobalTransform::from_translation(pos),
                InheritedVisibility::VISIBLE,
                CubemapVisibleEntities::default(),
            ))
            .id();
        lamp_entities.push(lamp);
        app.world_mut().spawn((
            Mesh3d::default(),
            Aabb::from_min_max(Vec3::splat(-0.5), Vec3::splat(0.5)),
            GlobalTransform::from_translation(pos + Vec3::new(0.0, 0.0, -2.0)),
            InheritedVisibility::VISIBLE,
            ViewVisibility::default(),
            crate::lamp_cache::LampShadowMobility::Static,
        ));
    }

    app.update();
    app.update();

    assert!(app.world().resource::<PointShadowAtlasConfig>().enabled, "render config tracks tunable");
    let atlas_count = lamp_entities.iter().filter(|&&e| app.world().get::<AtlasPointShadowMap>(e).is_some()).count();
    assert_eq!(atlas_count, 12, "all 12 lamps admitted into 2D atlas beyond stock 8-lamp cap");

    // Confirm captures for all lamps via LampShadowAtlasReports until ready.
    for _ in 0..8 {
        let reports: Vec<(Entity, AtlasPointShadowReport)> = lamp_entities
            .iter()
            .filter_map(|&e| {
                let comp = app.world().get::<AtlasPointShadowMap>(e)?;
                let cap = comp.draw_faces & !comp.draw_to_live;
                (cap != 0).then_some((e, AtlasPointShadowReport { plan: comp.plan, confirmed: cap, lost: false }))
            })
            .collect();
        app.world_mut().resource_mut::<crate::LampShadowAtlasReports>().0.extend(reports);
        app.update();
    }

    // Once settled and ready, contact shadows are capped at 4 (not all 12!).
    let contact_count = lamp_entities
        .iter()
        .filter(|&&e| app.world().get::<PointLight>(e).is_some_and(|l| l.contact_shadows_enabled))
        .count();
    assert!(contact_count <= 4, "contact shadows must stay capped at contact_max=4, got {contact_count}");
    assert!(contact_count > 0, "top-ranked hero lamps keep contact shadows");

    // Move lamp 0 -> promotes to Live Atlas (carried torch) with zero static rects and active live rects.
    *app.world_mut().get_mut::<GlobalTransform>(lamp_entities[0]).expect("lamp0") =
        GlobalTransform::from_translation(Vec3::new(10.0, 2.0, 5.0));
    app.update();
    app.update();
    let moved_comp = app.world().get::<AtlasPointShadowMap>(lamp_entities[0]).expect("moved lamp stays on atlas");
    assert_eq!(moved_comp.static_rects, [None; 6], "moving lamp consumes zero static tiles");
    assert!(moved_comp.live_rects.iter().all(Option::is_some), "moving lamp allocates 6 live tiles");
    assert_ne!(moved_comp.draw_to_live, 0, "moving lamp draws into live atlas");

    // Toggle atlas OFF live -> every lamp releases AtlasPointShadowMap cleanly.
    app.world_mut().write_message(SetTunable::new("shadow.lamp.atlas.enabled", "off", SetBy::Console));
    app.update();
    assert!(!app.world().resource::<PointShadowAtlasConfig>().enabled);
    assert!(
        lamp_entities.iter().all(|&e| app.world().get::<AtlasPointShadowMap>(e).is_none()),
        "toggling atlas off removes AtlasPointShadowMap from every lamp"
    );
}

/// SH-33 audit fix: walking close to a lamp (tier resize 256 -> 512) must:
/// 1. Keep static casters baked (zero spurious `movers` / `NoIndirectDrawing` draws).
/// 2. Keep publishing the retiring 256 rect in `static_rects` while targeting the new 512 rect in
///    `draw_static_rects`, switching to 512 ONLY after `AtlasPointShadowReport::confirmed`.
/// 3. Leave `AtlasPointShadowMap` unchanged (`!is_changed()`) on subsequent idle frames.
#[test]
fn atlas_tier_resize_keeps_statics_baked_and_hands_over_on_confirm() {
    use bevy_camera::primitives::Aabb;
    use bevy_camera::visibility::{CubemapVisibleEntities, InheritedVisibility, ViewVisibility};
    use bevy_camera::{Camera, Camera3d};
    use bevy_math::Vec3;
    use bevy_mesh::Mesh3d;
    use bevy_pbr::{AtlasPointShadowMap, AtlasPointShadowReport};
    use bevy_time::TimeUpdateStrategy;
    use bevy_transform::components::GlobalTransform;
    use sf_tunables_bevy::{SetBy, SetTunable};

    let mut app = bevy_app::App::new();
    app.add_plugins((bevy::MinimalPlugins, sf_tunables_bevy::TunablesPlugin, crate::ShadowQualityPlugin, crate::PointShadowPlugin))
        .init_resource::<PointLightShadowMap>()
        .insert_resource(TimeUpdateStrategy::ManualDuration(std::time::Duration::from_secs(1)));
    app.world_mut()
        .resource_mut::<bevy_time::Time<bevy_time::Virtual>>()
        .set_max_delta(std::time::Duration::from_secs(10));

    for (name, val) in [
        ("shadow.lamp.count", "8"),
        ("shadow.lamp.fade_m", "40,60"),
        ("shadow.lamp.fade_time_s", "0"),
        ("shadow.lamp.day_gate", "off"),
        ("shadow.lamp.atlas.enabled", "on"),
        ("shadow.lamp.atlas.tier_512_m", "10"),
        ("shadow.lamp.atlas.tier_256_m", "24"),
        ("shadow.lamp.atlas.resize_cooldown_s", "2"),
    ] {
        app.world_mut().write_message(SetTunable::new(name, val, SetBy::Console));
    }

    // Start 15 m away from lamp -> 256 px tier.
    let cam = app.world_mut().spawn((Camera::default(), Camera3d::default(), GlobalTransform::from_xyz(0.0, 0.0, 15.0))).id();
    let lamp = app
        .world_mut()
        .spawn((
            PointLight { intensity: 1500.0, range: 12.0, ..Default::default() },
            crate::BudgetedShadow::default(),
            GlobalTransform::IDENTITY,
            InheritedVisibility::VISIBLE,
            CubemapVisibleEntities::default(),
        ))
        .id();
    app.world_mut().spawn((
        Mesh3d::default(),
        Aabb::from_min_max(Vec3::splat(-0.5), Vec3::splat(0.5)),
        GlobalTransform::from_xyz(3.0, 0.0, 0.0), // face 0 (+X)
        InheritedVisibility::VISIBLE,
        ViewVisibility::default(),
        crate::lamp_cache::LampShadowMobility::Static,
    ));

    // Initial admission & capture: `static_rects[0]` stays None until confirmed; `draw_static_rects[0]` is 256.
    app.update();
    app.update();
    let comp = *app.world().get::<AtlasPointShadowMap>(lamp).expect("admitted");
    assert_eq!(comp.static_rects[0], None, "unconfirmed initial static rect must not be published");
    let rect_256 = comp.draw_static_rects[0].expect("256 draw rect");
    assert_eq!(rect_256[2], 256);
    assert_ne!(comp.draw_faces & 1, 0, "face 0 captures");
    assert_eq!(comp.draw_to_live, 0, "no movers");

    // Confirm initial capture -> publishes 256 rect.
    app.world_mut()
        .resource_mut::<crate::LampShadowAtlasReports>()
        .0
        .push((lamp, AtlasPointShadowReport { plan: comp.plan, confirmed: 1, lost: false }));
    app.update();
    let comp = *app.world().get::<AtlasPointShadowMap>(lamp).expect("confirmed");
    assert_eq!(comp.static_rects[0], Some(rect_256));
    assert_eq!(comp.draw_faces, 0);

    // Wait out resize cooldown (2 s) and verify idle frame does not mark `AtlasPointShadowMap` changed.
    app.update();
    app.update();
    let changed_idle = app
        .world_mut()
        .query_filtered::<Entity, Changed<AtlasPointShadowMap>>()
        .iter(app.world())
        .count();
    assert_eq!(changed_idle, 0, "idle atlas lamp must not churn Changed<AtlasPointShadowMap>");

    // Walk close to lamp (5 m < tier_512_m = 10 m) -> upgrades 256 -> 512.
    *app.world_mut().get_mut::<GlobalTransform>(cam).expect("cam") = GlobalTransform::from_xyz(0.0, 0.0, 5.0);
    app.update();
    let comp = *app.world().get::<AtlasPointShadowMap>(lamp).expect("resized");
    assert_eq!(comp.draw_to_live, 0, "tier resize must NOT turn static casters into movers");
    assert_eq!(comp.mover_only_faces, 0, "tier resize must NOT trigger mover_only draws");
    assert_eq!(comp.static_rects[0], Some(rect_256), "shader keeps sampling retiring 256 rect until 512 is confirmed");
    let rect_512 = comp.draw_static_rects[0].expect("512 draw target");
    assert_eq!(rect_512[2], 512);
    assert_ne!(comp.draw_faces & 1, 0, "512 capture queued immediately without waiting 2 s");

    // Without confirmation, retiring 256 stays published (e.g. pipeline still compiling).
    app.update();
    let unconfirmed = *app.world().get::<AtlasPointShadowMap>(lamp).expect("still unconfirmed");
    assert_eq!(unconfirmed.static_rects[0], Some(rect_256), "unconfirmed 512 must not replace retiring 256");

    // Confirm 512 capture -> atomic handover to 512 rect and retiring 256 freed.
    app.world_mut()
        .resource_mut::<crate::LampShadowAtlasReports>()
        .0
        .push((lamp, AtlasPointShadowReport { plan: unconfirmed.plan, confirmed: 1, lost: false }));
    app.update();
    let handed_over = *app.world().get::<AtlasPointShadowMap>(lamp).expect("handed over");
    assert_eq!(handed_over.static_rects[0], Some(rect_512), "confirmed 512 rect now published");
    assert_eq!(handed_over.draw_faces, 0);
}

/// SH-33 audit fix: a moving torch with an active camera `Frustum` must only publish `live_rects`
/// for faces drawn this frame (`draw_faces`), never unwritten/culled faces.
#[test]
fn atlas_moving_torch_publishes_only_drawn_faces() {
    use bevy_camera::primitives::{Aabb, Frustum};
    use bevy_camera::visibility::{CubemapVisibleEntities, InheritedVisibility, ViewVisibility};
    use bevy_camera::{Camera, Camera3d};
    use bevy_math::{Mat4, Vec3, proj};
    use bevy_mesh::Mesh3d;
    use bevy_pbr::AtlasPointShadowMap;
    use bevy_shape::ViewFrustum;
    use bevy_time::TimeUpdateStrategy;
    use bevy_transform::components::GlobalTransform;
    use sf_tunables_bevy::{SetBy, SetTunable};

    let mut app = bevy_app::App::new();
    app.add_plugins((bevy::MinimalPlugins, sf_tunables_bevy::TunablesPlugin, crate::ShadowQualityPlugin, crate::PointShadowPlugin))
        .init_resource::<PointLightShadowMap>()
        .insert_resource(TimeUpdateStrategy::ManualDuration(std::time::Duration::from_millis(50)));

    for (name, val) in [
        ("shadow.lamp.count", "8"),
        ("shadow.lamp.fade_m", "40,60"),
        ("shadow.lamp.fade_time_s", "0"),
        ("shadow.lamp.day_gate", "off"),
        ("shadow.lamp.atlas.enabled", "on"),
    ] {
        app.world_mut().write_message(SetTunable::new(name, val, SetBy::Console));
    }

    // Camera at (0, 0, 10) looking toward -Z (narrow FOV so lamp at (0, 0, -2) is ahead, and its +Z
    // or -Z/side faces have a well-defined culled subset — place lamp far off to +X/+Z or directly ahead).
    let cam_tf = GlobalTransform::from_xyz(0.0, 0.0, 10.0);
    let proj_mat = proj::perspective_infinite_reverse(core::f32::consts::FRAC_PI_3, 1.0, 0.1);
    let clip_from_world = proj_mat * Mat4::from(cam_tf.affine().inverse());
    let frustum = Frustum(ViewFrustum::from_clip_from_world_custom_far(
        &clip_from_world,
        &cam_tf.translation(),
        &cam_tf.back(),
        100.0,
    ));
    app.world_mut().spawn((Camera::default(), Camera3d::default(), cam_tf, frustum));

    // Place a lamp slightly behind the camera at (0, 0, 15) with range 3 m so `in_view == false`
    // (`draw_faces == 0`), plus a static caster so moving it triggers `lamp_moved`.
    let lamp = app
        .world_mut()
        .spawn((
            PointLight { intensity: 1500.0, range: 3.0, ..Default::default() },
            crate::BudgetedShadow::default(),
            GlobalTransform::from_xyz(0.0, 0.0, 15.0),
            InheritedVisibility::VISIBLE,
            CubemapVisibleEntities::default(),
        ))
        .id();
    app.world_mut().spawn((
        Mesh3d::default(),
        Aabb::from_min_max(Vec3::splat(-0.5), Vec3::splat(0.5)),
        GlobalTransform::from_xyz(1.0, 0.0, 15.0),
        InheritedVisibility::VISIBLE,
        ViewVisibility::default(),
        crate::lamp_cache::LampShadowMobility::Static,
    ));

    app.update();
    app.update();
    // Move lamp to promote it into `moving_atlas` while still behind the camera (`draw_faces == 0`).
    *app.world_mut().get_mut::<GlobalTransform>(lamp).expect("lamp") = GlobalTransform::from_xyz(0.0, 0.0, 16.0);
    app.update();
    app.update();

    let comp = *app.world().get::<AtlasPointShadowMap>(lamp).expect("moving torch");
    assert_eq!(comp.draw_faces, 0, "behind-camera moving torch draws no faces");
    assert_eq!(comp.live_rects, [None; 6], "undrawn moving-torch faces must never publish unwritten live rects");
}

/// SH-33 audit fix:
/// 1. Removing `retained` from `LightKeyCache` (as `point_shadow_atlas.rs` now does on every
///    `shadow_render_phases.remove`) makes `check_views_lights_need_specialization` mark
///    `DirtySpecializations::views`, so `queue_shadows` re-queues all visible entities even when
///    `added_entities()` is empty.
/// 2. Source contract checks for the render-world fixes that need a GPU device to run end-to-end
///    (`light_key_cache.remove(&retained)`, preserving `existing_prev` on non-draw frames, and
///    using `world_from_view.back()` per face).
#[test]
fn atlas_phase_wipe_invalidates_light_key_cache_and_non_draw_preserves_mode() {
    use bevy_pbr::{LightEntity, LightKeyCache, Shadow, check_views_lights_need_specialization};
    use bevy_render::batching::gpu_preprocessing::GpuPreprocessingMode;
    use bevy_render::camera::DirtySpecializations;
    use bevy_render::render_phase::ViewBinnedRenderPhases;
    use bevy_render::view::{ColorGrading, ExtractedView};

    // 1. Live ECS check: removing from `LightKeyCache` forces `DirtySpecializations::views`.
    let mut render = World::new();
    render.init_resource::<ViewBinnedRenderPhases<Shadow>>();
    render.init_resource::<LightKeyCache>();
    render.init_resource::<DirtySpecializations>();

    let main_light = MainEntity::from(Entity::from_bits(42));
    let retained = RetainedViewEntity::new(main_light, None, 0);
    render
        .resource_mut::<ViewBinnedRenderPhases<Shadow>>()
        .prepare_for_new_frame(retained, GpuPreprocessingMode::PreprocessingOnly);
    let view_entity = render
        .spawn((
            LightEntity::Point { light_entity: Entity::from_bits(7), face_index: 0 },
            ExtractedView {
                retained_view_entity: retained,
                viewport: bevy_math::UVec4::new(0, 0, 256, 256),
                world_from_view: GlobalTransform::IDENTITY,
                clip_from_world: None,
                clip_from_view: bevy_math::Mat4::IDENTITY,
                target_format: bevy_pbr::CORE_3D_SHADOW_MAP_FORMAT,
                color_grading: ColorGrading::default(),
                invert_culling: false,
            },
        ))
        .id();
    let _ = view_entity;

    let check = render.register_system(check_views_lights_need_specialization);
    render.run_system(check).expect("first specialization check");
    assert!(
        render.resource::<DirtySpecializations>().must_wipe_specializations_for_view(retained),
        "initial view must be marked dirty"
    );

    // Next frame: clear dirty set, keep light_key_cache -> NOT dirty (steady frame).
    render.resource_mut::<DirtySpecializations>().views.clear();
    render.run_system(check).expect("steady frame");
    assert!(
        !render.resource::<DirtySpecializations>().must_wipe_specializations_for_view(retained),
        "steady frame stays clean"
    );

    // Mode switch: wipe phase AND remove from `LightKeyCache` -> MUST be marked dirty again!
    render.resource_mut::<ViewBinnedRenderPhases<Shadow>>().remove(&retained);
    render.resource_mut::<LightKeyCache>().remove(&retained);
    render
        .resource_mut::<ViewBinnedRenderPhases<Shadow>>()
        .prepare_for_new_frame(retained, GpuPreprocessingMode::Culling);
    render.run_system(check).expect("mode-switched frame");
    assert!(
        render.resource::<DirtySpecializations>().must_wipe_specializations_for_view(retained),
        "wiping shadow phase + light_key_cache must force full re-queue in queue_shadows"
    );

    // 2. Source contract checks on `point_shadow_atlas.rs`.
    let squash = |s: &str| s.split_whitespace().collect::<String>();
    let atlas = squash(&vendored("render/point_shadow_atlas.rs"));
    assert!(
        atlas.contains("shadow_render_phases.remove(&retained);light_key_cache.remove(&retained);"),
        "point_shadow_atlas.rs: every shadow_render_phases.remove must also remove from light_key_cache"
    );
    assert!(
        atlas.contains("elseifletSome(prev)=existing_prev{letto_live=prev.texture_id==live_tex_id;(to_live,prev.mover_only,Some(prev.dst_rect))}"),
        "point_shadow_atlas.rs: non-draw frames must preserve existing view mode/rect"
    );
    assert!(
        atlas.contains("letface_backward=world_from_view.back();") && !atlas.contains("light.transform.back()"),
        "point_shadow_atlas.rs: custom far plane must use per-face world_from_view.back(), not light.transform.back()"
    );
}

/// SH-33 audit fix: `fs_blit` guard-band remapping aligns the 90 deg cube seam exactly when
/// blitting between different Static and Live tile resolutions (e.g. 256 -> 512 or 512 -> 256).
#[test]
fn atlas_blit_guard_ratio_aligns_cube_seam_across_tile_sizes() {
    let guard = 2.0_f32;
    for (src_size, dst_size) in [(256u32, 512u32), (512, 256), (128, 512), (256, 256)] {
        let tan_src = bevy_pbr::atlas_guard_tan_half(src_size, guard);
        let tan_dst = bevy_pbr::atlas_guard_tan_half(dst_size, guard);
        let guard_ratio = tan_dst / tan_src;
        // A point right on the 90 deg cube seam (`t = 1.0`) sits at `ndc = 1.0 / tan_half`.
        let ndc_dst_seam = 1.0 / tan_dst;
        let ndc_src_seam = ndc_dst_seam * guard_ratio;
        let expected_ndc_src_seam = 1.0 / tan_src;
        assert!(
            (ndc_src_seam - expected_ndc_src_seam).abs() < 1e-6,
            "{src_size}->{dst_size}: {ndc_src_seam} vs {expected_ndc_src_seam}"
        );
    }
}

/// Industry-standard shadow stack contracts:
/// 1. 2D Shadow Atlas defaults to ON and `shadow.lamp.atlas.soft_pcf_radius` binds live to `PointShadowAtlasConfig`.
/// 2. `shadows.wesl` keeps the 1-tap fast path (`pcf_radius_px <= 0.0`) and clamps every 4-tap PCF tap inside `[uv_min, uv_max]`.
/// 3. `pbr_functions.wesl` uses the single-fetch screen-space-clamped Bend-style contact shadow marcher (`SH-14`).
/// 4. `far_shadow.wesl` uses camera-independent world-space march steps (`RO-20` hill swimming fix).
#[test]
fn shadow_stack_soft_pcf_contact_and_far_contracts() {
    use bevy_pbr::PointShadowAtlasConfig;
    use sf_tunables_bevy::{SetBy, SetTunable};

    let mut app = bevy_app::App::new();
    app.add_plugins((sf_tunables_bevy::TunablesPlugin, crate::ShadowQualityPlugin, crate::PointShadowPlugin))
        .init_resource::<bevy_time::Time>()
        .init_resource::<PointLightShadowMap>();
    app.update();

    let cfg = *app.world().resource::<PointShadowAtlasConfig>();
    assert!(cfg.enabled, "2D point shadow atlas must default ON after SH-33 + shadow_a3_atlas win");
    assert!((cfg.soft_pcf_radius - 1.0).abs() < 1e-6, "default soft_pcf_radius is 1.0 texel");

    app.world_mut().write_message(SetTunable::new("shadow.lamp.atlas.soft_pcf_radius", "0", SetBy::Console));
    app.update();
    assert!(app.world().resource::<PointShadowAtlasConfig>().soft_pcf_radius.abs() < 1e-6, "live tunable reaches PointShadowAtlasConfig");

    app.world_mut().write_message(SetTunable::new("shadow.lamp.atlas.soft_pcf_radius", "1.5", SetBy::Console));
    app.update();
    assert!((app.world().resource::<PointShadowAtlasConfig>().soft_pcf_radius - 1.5).abs() < 1e-6);

    let shadows = vendored("render/shadows.wesl");
    assert!(shadows.contains("if (pcf_radius_px <= 0.0)"), "shadows.wesl keeps 1-tap fast path when soft_pcf_radius == 0");
    assert!(shadows.contains("clamp(base_uv + o3, uv_min, uv_max)"), "shadows.wesl clamps every 4-tap PCF tap inside tile rect");

    let pbr = vendored("render/pbr_functions.wesl");
    assert!(pbr.contains("raymarch::depth_sample_nearest(sample_uv)"), "pbr_functions.wesl uses single nearest depth fetch per step");
    assert!(pbr.contains("ray_fade * thick_fade * edge_fade"), "pbr_functions.wesl applies smooth triple feathering (SH-14)");
    assert!(pbr.contains("min(world_len, max(0.03, view_dist * 0.05))"), "pbr_functions.wesl clamps contact ray length in screen space");

    let far_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../sf_terrain_bevy/src/shaders/far_shadow.wesl");
    let far = std::fs::read_to_string(&far_path).expect("read far_shadow.wesl");
    assert!(far.contains("const FAR_FIRST_M: f32 = 4.0;"), "far_shadow.wesl uses camera-independent first step (RO-20)");
    assert!(!far.contains("FAR_FIRST_PER_M"), "far_shadow.wesl must not shift raymarch steps with camera distance");
}
