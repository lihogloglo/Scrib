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
