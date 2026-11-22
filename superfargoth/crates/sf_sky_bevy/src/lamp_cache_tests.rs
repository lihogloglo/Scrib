//! The lamp shadow cache's tracker (ROADMAP 1.25 CW-4 S3, `lamp_cache.rs`): which lamps are
//! cached, what each frame's `CachedPointShadowMap` asks the renderer for, and the switch. No
//! render world: `prepare_lights` needs a device, so the faces a lamp gets views for are read
//! from the component through the vendored decision function (`point_shadow_view_faces`), and a
//! system in `Last` stands in for the render world's feedback (every capture drawn is confirmed).

use bevy::MinimalPlugins;
use bevy_app::{App, Last, PostUpdate};
use bevy_camera::Camera3d;
use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::{CubemapVisibleEntities, InheritedVisibility, ViewVisibility, VisibilitySystems};
use bevy_ecs::prelude::*;
use bevy_light::{PointLight, SimulationLightSystems};
use bevy_math::Vec3;
use bevy_mesh::Mesh3d;
use bevy_pbr::{CachedPointShadowMap, CachedPointShadowReport, PointShadowCacheStore, point_shadow_view_faces};
use bevy_transform::components::GlobalTransform;
use sf_tunables::Type;
use sf_tunables_bevy::Tunables;

use crate::lamp_cache::{LampCacheReports, LampCacheStats, LampShadowMobility};
use crate::lamp_cache_render_tests::RenderExtract;
use crate::{BudgetedShadow, PointShadowBudget, PointShadowPlugin, ShadowMapCached, ShadowSlot};

/// What the stand-in render world saw each frame: `(view, draw, load)` of the written component.
#[derive(Resource, Default)]
struct Frames(Vec<(u8, u8, u8)>);

/// Confirms every capture of a written plan (faces drawn without loading), as the render world
/// does once their pipelines are compiled; logs the masks.
fn render_stand_in(lamps: Query<(Entity, Ref<CachedPointShadowMap>)>, mut reports: ResMut<LampCacheReports>, mut frames: ResMut<Frames>) {
    for (lamp, cached) in &lamps {
        if cached.is_changed() {
            frames.0.push((cached.view_faces, cached.draw_faces, cached.load_faces));
            let confirmed = cached.draw_faces & !cached.load_faces;
            reports.0.push((lamp, CachedPointShadowReport { plan: cached.plan, confirmed, lost: false }));
        } else {
            frames.0.push((cached.view_faces, 0, 0));
        }
    }
}

struct Room {
    app: App,
    lamp: Entity,
    camera: Entity,
    /// Statics at +X and -Y (faces 0 and 3).
    statics: [Entity; 2],
}

fn caster(app: &mut App, at: Vec3) -> Entity {
    app.world_mut()
        .spawn((
            Mesh3d::default(),
            Aabb::from_min_max(Vec3::splat(-0.5), Vec3::splat(0.5)),
            GlobalTransform::from_translation(at),
            InheritedVisibility::VISIBLE,
        ))
        .id()
}

/// A lamp (range 10 m) in a room with two statics, a viewer 5 m away, one budget slot, no fade
/// time, 100 ms frames, the cache switched `on` or not.
fn room(on: bool) -> Room {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins).add_plugins(PointShadowPlugin);
    app.finish();
    app.cleanup();
    app.insert_resource(PointShadowBudget { max: 1, fade_time: 0.0, ..PointShadowBudget::default() });
    app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(std::time::Duration::from_millis(100)));
    app.init_resource::<Frames>().add_systems(Last, render_stand_in);
    let camera = app.world_mut().spawn((Camera3d::default(), GlobalTransform::from_xyz(0.0, 0.0, -5.0))).id();
    let lamp = app
        .world_mut()
        .spawn((
            PointLight { range: 10.0, ..Default::default() },
            BudgetedShadow::default(),
            GlobalTransform::IDENTITY,
            InheritedVisibility::VISIBLE,
            ViewVisibility::VISIBLE,
        ))
        .id();
    let statics = [caster(&mut app, Vec3::new(3.0, 0.0, 0.0)), caster(&mut app, Vec3::new(0.0, -3.0, 0.0))];
    set_switch(&mut app, on);
    Room { app, lamp, camera, statics }
}

fn set_switch(app: &mut App, on: bool) {
    let mut tunables = app.world_mut().resource_mut::<Tunables>();
    // These tests exercise the CW-4 cube-cache path; keep the 2D atlas switch off here.
    let atlas_id = tunables.id_of(&sf_sky::shadow_atlas::ENABLED).expect("registered");
    tunables.set(atlas_id.untyped(), false.to_value()).expect("a bool");
    let id = tunables.id_of(&sf_sky::lamp_cache::ENABLED).expect("registered");
    tunables.set(id.untyped(), on.to_value()).expect("a bool");
}

impl Room {
    fn run(&mut self, frames: usize) {
        for _ in 0..frames {
            self.app.update();
        }
    }

    fn cached(&self) -> Option<&CachedPointShadowMap> {
        self.app.world().get::<CachedPointShadowMap>(self.lamp)
    }

    /// Face views the renderer keeps for the lamp: six on the stock path, the mask when cached.
    fn views(&self) -> u8 {
        let faces = self.cached().map(CachedPointShadowMap::faces);
        let shadowed = self.light().shadow_maps_enabled || faces.is_some();
        if shadowed { point_shadow_view_faces(faces.as_ref()) } else { 0 }
    }

    fn light(&self) -> &PointLight {
        self.app.world().get::<PointLight>(self.lamp).expect("lamp")
    }

    fn fade(&self) -> f32 {
        self.app.world().get::<ShadowSlot>(self.lamp).map_or(0.0, |s| s.fade)
    }

    fn ready(&self) -> bool {
        self.app.world().get::<ShadowMapCached>(self.lamp).is_some_and(|c| c.ready)
    }

    fn list(&self, face: usize) -> Vec<Entity> {
        self.app.world().get::<CubemapVisibleEntities>(self.lamp).expect("lists").get(face).entities.clone()
    }

    fn take_frames(&mut self) -> Vec<(u8, u8, u8)> {
        std::mem::take(&mut self.app.world_mut().resource_mut::<Frames>().0)
    }

    fn translate(&mut self, entity: Entity, by: Vec3) {
        let mut t = self.app.world_mut().get_mut::<GlobalTransform>(entity).expect("transform");
        *t = GlobalTransform::from_translation(t.translation() + by);
    }

    fn stats(&self) -> LampCacheStats {
        *self.app.world().resource::<LampCacheStats>()
    }
}

#[test]
fn still_room_ends_with_no_lamp_views_after_capture() {
    let mut room = room(true);
    room.run(1);
    assert!(!room.ready() && room.fade() < 1e-6, "a new slot shows no shadow before its faces are captured");
    room.run(5);
    let frames = room.take_frames();
    // The first written plan captures the two faces with statics (two per frame), cleared first.
    assert_eq!(frames[0], (0b1001, 0b1001, 0), "first plan: capture +X and -Y");
    assert!(room.ready(), "every face captured and confirmed");
    assert_eq!(room.views(), 0, "a clean cube has no face view");
    let cached = room.cached().expect("cached");
    assert_eq!((cached.draw_faces, cached.load_faces), (0, 0));
    assert!(!room.light().shadow_maps_enabled, "the stock scan skips a cached lamp");
    assert!((room.fade() - 1.0).abs() < 1e-6, "shadow faded in once ready");
    assert_eq!(room.list(0), vec![room.statics[0]], "the capture listed the +X static");
    assert_eq!(room.list(3), vec![room.statics[1]]);
    // Still frames draw nothing and write nothing.
    room.run(10);
    assert!(room.take_frames().iter().all(|&(view, draw, _)| view == 0 && draw == 0));
    assert_eq!(room.stats().frame.captures, 0);
}

#[test]
fn a_walker_gets_views_only_on_its_faces_then_none_after_linger() {
    let mut room = room(true);
    room.run(6);
    let walker = caster(&mut room.app, Vec3::new(0.0, 0.0, 3.0));
    room.app.world_mut().entity_mut(walker).insert(LampShadowMobility::Movable);
    room.take_frames();
    room.run(3);
    for (view, draw, load) in room.take_frames() {
        assert_eq!(view, 0b10_0000, "only the walker's face (world +Z: cube face 5) has a view");
        assert_eq!((draw, load), (0b10_0000, 0b10_0000), "drawn every frame within 16 m, over the depth");
    }
    assert_eq!(room.list(5), vec![walker]);
    assert!(room.stats().movers >= 1);

    // It walks out of range: the face lingers 0.5 s, then nothing.
    room.translate(walker, Vec3::new(0.0, 0.0, 30.0));
    room.run(2);
    assert_eq!(room.views(), 0b10_0000, "lingering");
    room.run(6);
    assert_eq!(room.views(), 0, "no view once the linger is over");
}

#[test]
fn a_spawned_static_recaptures_its_face() {
    let mut room = room(true);
    room.run(6);
    room.take_frames();
    let chair = caster(&mut room.app, Vec3::new(0.0, 0.0, -3.0));
    room.run(40);
    let frames = room.take_frames();
    // New and rigid: a mover on world -Z (cube face 4) until it settles (1 s) ...
    assert_eq!(frames[0], (0b1_0000, 0b1_0000, 0b1_0000), "drawn as a mover meanwhile");
    // ... then that face alone is captured again, cleared first, with the chair listed.
    let capture = frames.iter().position(|&(_, draw, load)| draw & 0b1_0000 != 0 && load & 0b1_0000 == 0);
    let capture = capture.expect("the -Z face is captured again");
    assert_eq!(frames[capture].1, 0b1_0000, "no other face recaptured");
    assert!(frames[capture + 1..].iter().all(|&(_, draw, _)| draw == 0), "clean after the capture");
    assert_eq!(room.list(4), vec![chair]);
    assert_eq!(room.views(), 0);
}

#[test]
fn a_moved_lamp_takes_the_stock_path_until_still() {
    let mut room = room(true);
    room.run(6);
    let lamp = room.lamp;
    room.translate(lamp, Vec3::new(0.5, 0.0, 0.0));
    room.run(2);
    assert!(room.cached().is_none(), "a moving lamp is not cached");
    assert!(room.light().shadow_maps_enabled, "stock cube map");
    assert_eq!(room.views(), 0b11_1111, "six stock face views");
    assert!((room.fade() - 1.0).abs() < 1e-6, "no pop: the stock path drew it");
    // Still for the settle time: cached again.
    room.run(12);
    assert!(room.cached().is_some(), "cached again once still");
    assert!(!room.light().shadow_maps_enabled);
}

#[test]
fn an_origin_shift_changes_nothing() {
    let mut room = room(true);
    room.run(6);
    room.take_frames();
    let shift = Vec3::new(8192.0, 0.0, -8192.0);
    for e in [room.lamp, room.camera, room.statics[0], room.statics[1]] {
        room.translate(e, shift);
    }
    room.run(30);
    assert!(room.cached().is_some(), "still cached");
    assert!(room.take_frames().iter().all(|&(view, draw, _)| view == 0 && draw == 0), "no recapture, no view");
}

#[test]
fn switch_off_is_the_stock_path() {
    let mut room = room(false);
    room.run(6);
    assert!(room.cached().is_none());
    assert!(room.app.world().get::<ShadowMapCached>(room.lamp).is_none());
    assert!(room.light().shadow_maps_enabled, "the budget's stock cube map");
    assert_eq!(room.views(), 0b11_1111, "six face views, as today");
    assert!((room.fade() - 1.0).abs() < 1e-6);
    assert!(room.list(0).is_empty(), "the cache wrote no list");
    assert_eq!(room.app.world().resource::<PointShadowCacheStore>().layers, 0, "no store texture");
    assert_eq!(room.stats(), LampCacheStats::default());
}

#[test]
fn flipping_the_switch_live_both_ways() {
    let mut room = room(false);
    room.run(3);
    set_switch(&mut room.app, true);
    for _ in 0..6 {
        room.run(1);
        assert!((room.fade() - 1.0).abs() < 1e-6, "the stock path's fade kept through the switch (no pop)");
    }
    assert!(room.cached().is_some() && room.ready(), "on: cached and captured");
    assert_eq!(room.views(), 0);
    assert!(room.app.world().resource::<PointShadowCacheStore>().layers > 0);

    set_switch(&mut room.app, false);
    room.run(1);
    assert!(room.cached().is_none() && room.app.world().get::<ShadowMapCached>(room.lamp).is_none(), "off: components gone");
    assert!(room.light().shadow_maps_enabled, "stock cube map again");
    assert_eq!(room.views(), 0b11_1111);
    assert!((room.fade() - 1.0).abs() < 1e-6, "no pop");
    assert_eq!(room.app.world().resource::<PointShadowCacheStore>().layers, 0);

    room.take_frames();
    set_switch(&mut room.app, true);
    for _ in 0..6 {
        room.run(1);
        assert!((room.fade() - 1.0).abs() < 1e-6, "no pop on again");
    }
    assert!(room.cached().is_some() && room.ready(), "on again: captured again");
    assert_eq!(room.take_frames().iter().filter(|f| f.1 != 0).count(), 1, "one capture plan (two faces), confirmed");
    assert_eq!(room.views(), 0);
}

/// Stand-in for `bevy_light`'s `check_point_light_mesh_visibility`: a shadowed lamp's six lists,
/// each caster in range on the face of its dominant axis; unshadowed (cached) lamps untouched.
fn stock_visibility(
    mut lamps: Query<(&PointLight, &GlobalTransform, &mut CubemapVisibleEntities)>,
    casters: Query<(Entity, &GlobalTransform), With<Mesh3d>>,
) {
    for (light, at, mut lists) in &mut lamps {
        if !light.shadow_maps_enabled {
            continue;
        }
        for face in 0..6 {
            lists.get_mut(face).entities.clear();
        }
        for (caster, t) in &casters {
            let d = t.translation() - at.translation();
            if d.length() > light.range {
                continue;
            }
            let a = d.abs();
            let face = if a.x >= a.y && a.x >= a.z {
                usize::from(d.x < 0.0)
            } else if a.y >= a.z {
                2 + usize::from(d.y < 0.0)
            } else {
                4 + usize::from(d.z > 0.0)
            };
            lists.get_mut(face).entities.push(caster);
        }
    }
}

/// The live handover the owner did (2026-11-17 crash): a stock lamp, the switch flipped on, the
/// lamp moving (stock path with the very lists it had before), still again (cached), the switch
/// off and on again. After every frame the render app's extraction runs and each face view
/// `prepare_lights` would create has its list.
#[test]
fn every_face_view_has_its_list_through_live_switches_and_moves() {
    let mut room = room(false);
    room.app.add_systems(
        PostUpdate,
        stock_visibility
            .in_set(SimulationLightSystems::CheckLightVisibility)
            .after(VisibilitySystems::VisibilityPropagate),
    );
    let mut extract = RenderExtract::new();
    extract.link(room.app.world_mut(), room.lamp);
    let mut step = |room: &mut Room, frames: usize, what: &str| {
        for _ in 0..frames {
            room.run(1);
            extract.extract(room.app.world_mut());
            let missing = extract.faces_without_list();
            assert!(missing.is_empty(), "{what}: face views without a list: {missing:?}");
        }
    };

    step(&mut room, 3, "stock");
    assert_eq!(room.list(0), vec![room.statics[0]], "the stand-in lists the stock faces");
    set_switch(&mut room.app, true);
    step(&mut room, 6, "switched on");
    assert!(room.cached().is_some() && room.ready());

    let lamp = room.lamp;
    room.translate(lamp, Vec3::new(0.5, 0.0, 0.0));
    step(&mut room, 2, "moved");
    assert!(room.cached().is_none() && room.light().shadow_maps_enabled, "stock path while moving");
    step(&mut room, 12, "settling");
    assert!(room.cached().is_some(), "cached again once still");

    set_switch(&mut room.app, false);
    step(&mut room, 2, "switched off");
    assert!(room.cached().is_none() && room.light().shadow_maps_enabled);
    set_switch(&mut room.app, true);
    step(&mut room, 6, "switched on again");
    assert!(room.cached().is_some());
}

/// SH-33 (2026-11-20): an atlas rect is sampled only once a draw went into it since it was
/// (re)allocated; before, Live tiles were sampled on the mover scheduler's skip frames and read
/// another lamp's old depth (flashing stray shadows).
#[test]
fn atlas_rects_publish_only_after_their_first_draw() {
    use crate::lamp_cache::AtlasWritten;
    const A: [u16; 3] = [0, 0, 256];
    const B: [u16; 3] = [256, 0, 256];
    let none = [None; 6];
    let one = |r: [u16; 3]| {
        let mut rects = [None; 6];
        rects[2] = Some(r);
        rects
    };
    let mut w = AtlasWritten::default();

    // Static rect captured the frame it is allocated: published at once.
    let (s, _) = w.gate(one(A), none, 1 << 2, 0);
    assert_eq!(s, one(A));
    // Live rect allocated for a visible mover on a skip frame: not sampled yet.
    let (s, l) = w.gate(one(A), one(B), 0, 0);
    assert_eq!((s, l), (one(A), none));
    // Movers drawn into it: published, and stays published on later skip frames.
    let (_, l) = w.gate(one(A), one(B), 0, 1 << 2);
    assert_eq!(l, one(B));
    let (_, l) = w.gate(one(A), one(B), 0, 0);
    assert_eq!(l, one(B));
    // Tier resize: the new Static rect waits for its recapture.
    let (s, _) = w.gate(one(B), none, 0, 0);
    assert_eq!(s, none);
    let (s, _) = w.gate(one(B), none, 1 << 2, 0);
    assert_eq!(s, one(B));
    // Freed and later reallocated at the same place: unwritten again.
    let (s, _) = w.gate(none, none, 0, 0);
    assert_eq!(s, none);
    let (s, _) = w.gate(one(B), none, 0, 0);
    assert_eq!(s, none);
}

/// SH-34 (2026-11-22): a carried torch's idle sway is slower than the 1 mm per frame "still" test,
/// so it used to settle, be admitted as a static lamp (fade back to zero, six captures, fade in)
/// and be thrown back to the moving path by its next faster movement, every second or two. With
/// [`CarriedLight`] it stays in the Live Atlas; without it, it is a static lamp again.
#[test]
fn a_carried_light_stays_on_the_moving_path_until_put_down() {
    use bevy_pbr::AtlasPointShadowMap;

    use crate::CarriedLight;

    /// 120 fps.
    const FRAME: std::time::Duration = std::time::Duration::from_micros(8_333);
    /// Three seconds of frames.
    const FRAMES: usize = 360;
    /// Idle sway: 0.2 mm per frame (0.024 m/s), turning every quarter second.
    const STEP_M: f32 = 2e-4;
    const TURN_FRAMES: usize = 30;

    let mut room = room(true);
    {
        let mut tunables = room.app.world_mut().resource_mut::<Tunables>();
        let id = tunables.id_of(&sf_sky::shadow_atlas::ENABLED).expect("registered");
        tunables.set(id.untyped(), true.to_value()).expect("a bool");
    }
    room.app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(FRAME));
    let lamp = room.lamp;
    let atlas = |room: &Room| room.app.world().get::<AtlasPointShadowMap>(lamp).copied();
    let on_static_path = |room: &Room| atlas(room).is_some_and(|a| a.draw_static_rects.iter().any(Option::is_some));
    let on_moving_path = |room: &Room| {
        atlas(room).is_some_and(|a| a.static_rects == [None; 6] && a.draw_static_rects == [None; 6]) && room.ready()
    };

    // Held and swaying: on the moving path from the first frames, and it never leaves it.
    room.app.world_mut().entity_mut(lamp).insert(CarriedLight);
    let mut fade = 0.0_f32;
    for frame in 0..FRAMES {
        let dir = if (frame / TURN_FRAMES).is_multiple_of(2) { 1.0 } else { -1.0 };
        room.translate(lamp, Vec3::new(STEP_M * dir, 0.0, 0.0));
        room.run(1);
        if frame >= 2 {
            assert!(on_moving_path(&room), "frame {frame}: a carried light left the moving path: {:?}", atlas(&room));
            assert!(atlas(&room).is_some_and(|a| a.draw_faces != 0), "frame {frame}: redrawn every frame");
        }
        assert!(room.fade() >= fade, "frame {frame}: the shadow faded back from {fade} to {}", room.fade());
        fade = room.fade();
    }
    assert!((fade - 1.0).abs() < 1e-6, "the carried light's shadow faded in: {fade}");

    // Put down (mark removed, no more movement): a static lamp again after the settle time.
    room.app.world_mut().entity_mut(lamp).remove::<CarriedLight>();
    let admitted = (0..FRAMES).find(|_| {
        room.run(1);
        on_static_path(&room)
    });
    assert!(admitted.is_some(), "a put-down light is admitted as a static lamp: {:?}", atlas(&room));
    assert!(admitted.is_some_and(|f| f >= 60), "not before the settle time: frame {admitted:?}");

    // Grabbed again without moving: out of the static path at once.
    room.app.world_mut().entity_mut(lamp).insert(CarriedLight);
    room.run(2);
    assert!(on_moving_path(&room), "a grabbed light is back on the moving path: {:?}", atlas(&room));
}

/// F4b (2026-11-22): new holders used to enter the Static Atlas in query order, so when it was
/// nearly full a far lamp listed first took the tiles of the lamp next to the viewer. They now
/// enter by [`ShadowSlot::importance`]: a Static Atlas with room for four tiles (no handover
/// reserve), eight lamps with one static face each, the far ones first and last in spawn order.
#[test]
fn a_full_static_atlas_goes_to_the_most_important_lamps() {
    use bevy_pbr::AtlasPointShadowMap;

    /// Each lamp's place along Z from the viewer, in spawn order; the four nearest are in the middle.
    const OFFSETS_M: [f32; 8] = [-6.0, 7.0, 1.0, -2.0, 3.0, -4.0, 5.0, -8.0];
    /// Static Atlas side: four tiles of the 512 tier every lamp here asks for.
    const STATIC_PX: u32 = 1024;
    const TILES: usize = 4;

    let mut room = room(true);
    {
        let mut tunables = room.app.world_mut().resource_mut::<Tunables>();
        let id = tunables.id_of(&sf_sky::shadow_atlas::ENABLED).expect("registered");
        tunables.set(id.untyped(), true.to_value()).expect("a bool");
        let id = tunables.id_of(&sf_sky::shadow_atlas::STATIC_SIZE_PX).expect("registered");
        tunables.set(id.untyped(), STATIC_PX.to_value()).expect("a count");
        // No texels held back for handovers: the whole atlas is for admissions.
        let id = tunables.id_of(&sf_sky::shadow_atlas::HANDOVER_SPARES).expect("registered");
        tunables.set(id.untyped(), 0_u32.to_value()).expect("a count");
    }
    // Only this test's lamps and statics.
    for entity in [room.lamp, room.statics[0], room.statics[1]] {
        room.app.world_mut().despawn(entity);
    }
    let eye = room.app.world().get::<GlobalTransform>(room.camera).expect("camera").translation();
    // A lamp of 1 m reach with one small static on its +X face, out of every other lamp's reach.
    let lamps: Vec<(Entity, f32)> = OFFSETS_M
        .iter()
        .map(|&offset| {
            let at = eye + Vec3::new(0.0, 0.0, offset);
            let world = room.app.world_mut();
            world.spawn((
                Mesh3d::default(),
                Aabb::from_min_max(Vec3::splat(-0.1), Vec3::splat(0.1)),
                GlobalTransform::from_translation(at + Vec3::new(0.5, 0.0, 0.0)),
                InheritedVisibility::VISIBLE,
            ));
            let lamp = world
                .spawn((
                    PointLight { range: 1.0, ..Default::default() },
                    BudgetedShadow::default(),
                    GlobalTransform::from_translation(at),
                    InheritedVisibility::VISIBLE,
                    ViewVisibility::VISIBLE,
                ))
                .id();
            (lamp, offset.abs())
        })
        .collect();

    room.run(5);

    let world = room.app.world();
    let mut by_distance = lamps.clone();
    by_distance.sort_by(|a, b| a.1.total_cmp(&b.1));
    for (lamp, distance) in &lamps {
        let slot = world.get::<ShadowSlot>(*lamp).expect("every lamp is ranked and holds a slot");
        assert!(slot.held && slot.importance > 0.0, "the lamp at {distance} m holds a ranked slot: {slot:?}");
    }
    for pair in by_distance.windows(2) {
        let score = |lamp: Entity| world.get::<ShadowSlot>(lamp).expect("slot").importance;
        assert!(score(pair[0].0) > score(pair[1].0), "the nearer lamp scores higher: {pair:?}");
    }
    let with_tiles = |lamp: Entity| world.get::<AtlasPointShadowMap>(lamp).is_some();
    let holders: Vec<f32> = by_distance.iter().filter(|(lamp, _)| with_tiles(*lamp)).map(|(_, d)| *d).collect();
    let nearest: Vec<f32> = by_distance.iter().take(TILES).map(|(_, d)| *d).collect();
    assert_eq!(holders, nearest, "the Static Atlas holds the nearest lamps (distances in metres)");
    let (farthest, distance) = by_distance[by_distance.len() - 1];
    assert!(!with_tiles(farthest), "the lamp at {distance} m has no tiles");
}
