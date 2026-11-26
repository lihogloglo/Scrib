//! Tests of the in-view marker (`MA20a`): the pure frustum test's exact cases, then a headless app
//! where two actors swap `AiInView` when the viewer turns round, and the seen one ranks first in the
//! allocator.

use std::f32::consts::{FRAC_PI_2, PI};

use bevy_app::{App, FixedUpdate};
use bevy_math::{Quat, Vec3};
use bevy_time::{Fixed, Time};
use sf_ai::{Tier, TierCaps};
use sf_ai_bevy::{AiActor, AiAllocPlugin, AiAllocSettings, AiInView, AiTier};
use sf_controller_bevy::{Dormant, Mover, MoverParams};
use sf_coords::WorldPos;
use sf_ids::ActorKey;

use super::*;

/// A 90° vertical FOV, square aspect: the side planes are at 45°, easy to place points against.
fn square_view(rotation: Quat) -> View {
    View { eye: WorldPos::ORIGIN, rotation, fov_y: FRAC_PI_2, aspect: 1.0 }
}

fn wpos(x: f32, y: f32, z: f32) -> WorldPos {
    WorldPos::ORIGIN.offset(Vec3::new(x, y, z))
}

const RULES: ViewRules = ViewRules { margin_m: 2.0, max_m: 60.0 };

#[test]
fn a_point_in_front_is_seen_and_one_behind_is_not() {
    let view = square_view(Quat::IDENTITY);
    assert!(sees(&view, wpos(0.0, 0.0, -10.0), 0.0, RULES));
    assert!(!sees(&view, wpos(0.0, 0.0, 10.0), 0.0, RULES));
    // Behind, but within the margin of the near plane: counts.
    assert!(sees(&view, wpos(0.0, 0.0, 1.5), 0.0, RULES));
}

#[test]
fn the_margin_widens_the_side_planes() {
    let view = square_view(Quat::IDENTITY);
    // 10 m ahead the right plane is at x = 10; a point at x = 12 is sqrt(2) m outside it.
    assert!(sees(&view, wpos(12.0, 0.0, -10.0), 0.0, RULES));
    assert!(!sees(&view, wpos(12.0, 0.0, -10.0), 0.0, ViewRules { margin_m: 0.0, ..RULES }));
    // 4 m past the plane in x (2.83 m away): past a 2 m margin.
    assert!(!sees(&view, wpos(14.0, 0.0, -10.0), 0.0, RULES));
    // The vertical planes too.
    assert!(sees(&view, wpos(0.0, 12.0, -10.0), 0.0, RULES));
    assert!(!sees(&view, wpos(0.0, -14.0, -10.0), 0.0, RULES));
}

#[test]
fn the_aspect_widens_the_horizontal_fov_only() {
    let wide = View { aspect: 2.0, ..square_view(Quat::IDENTITY) };
    let rules = ViewRules { margin_m: 0.0, ..RULES };
    // Horizontal half-tangent 2: x = 19 at 10 m is inside; y = 19 is not (vertical stays 1).
    assert!(sees(&wide, wpos(19.0, 0.0, -10.0), 0.0, rules));
    assert!(!sees(&wide, wpos(0.0, 19.0, -10.0), 0.0, rules));
}

#[test]
fn past_the_max_distance_is_not_seen() {
    let view = square_view(Quat::IDENTITY);
    assert!(sees(&view, wpos(0.0, 0.0, -59.0), 0.0, RULES));
    assert!(!sees(&view, wpos(0.0, 0.0, -61.0), 0.0, RULES));
    // The sphere's nearest point counts: a 2 m sphere 61 m away reaches 59 m.
    assert!(sees(&view, wpos(0.0, 0.0, -61.0), 2.0, RULES));
}

#[test]
fn a_sphere_reaching_in_is_seen_though_its_centre_is_outside() {
    let view = square_view(Quat::IDENTITY);
    let rules = ViewRules { margin_m: 0.0, ..RULES };
    // Centre 2 * sqrt(2) = 2.83 m outside the right plane: a 3 m sphere reaches in, a 2 m one does not.
    assert!(!sees(&view, wpos(14.0, 0.0, -10.0), 2.0, rules));
    assert!(sees(&view, wpos(14.0, 0.0, -10.0), 3.0, rules));
    // Behind the eye by 1 m, radius 1.5: it straddles the near plane.
    assert!(sees(&view, wpos(0.0, 0.0, 1.0), 1.5, rules));
}

#[test]
fn the_rotation_turns_the_frustum() {
    // Yaw 90° left: forward -Z turns to -X.
    let view = square_view(Quat::from_rotation_y(FRAC_PI_2));
    assert!(view.forward().abs_diff_eq(Vec3::NEG_X, 1e-6), "{:?}", view.forward());
    assert!(sees(&view, wpos(-10.0, 0.0, 0.0), 0.0, RULES));
    assert!(!sees(&view, wpos(0.0, 0.0, -10.0), 0.0, RULES));
}

#[test]
fn any_viewer_of_several_counts_and_none_sees_nothing() {
    let ahead = wpos(0.0, 0.0, -10.0);
    let views = [square_view(Quat::from_rotation_y(PI)), square_view(Quat::IDENTITY)];
    assert!(seen_by_any(&views, ahead, 0.0, RULES));
    assert!(!seen_by_any(&views[..1], ahead, 0.0, RULES));
    assert!(!seen_by_any(&[], ahead, 0.0, RULES));
}

// --- Headless ---------------------------------------------------------------------------------

fn actor(app: &mut App, key: u8, z: f32) -> Entity {
    let feet = wpos(0.0, 0.0, z);
    let mover = Mover::new(feet, 0.0, MoverParams::default(), WorldPos::ORIGIN);
    app.world_mut().spawn((AiActor(ActorKey(u128::from(key))), mover)).id()
}

/// The marker and the allocator, one Full slot; a viewer at the eye height looking down -Z.
fn app() -> (App, Entity) {
    let mut app = App::new();
    app.insert_resource(Time::<Fixed>::from_hz(60.0));
    app.add_plugins(AiAllocPlugin { timings: false });
    app.world_mut().resource_mut::<AiAllocSettings>().set_caps(TierCaps { full: 1, reduced: 0 });
    add(&mut app);
    let view = View { eye: wpos(0.0, 1.7, 0.0), ..square_view(Quat::IDENTITY) };
    let viewer = app.world_mut().spawn(AiViewer(view)).id();
    (app, viewer)
}

fn step(app: &mut App) {
    app.world_mut().run_schedule(FixedUpdate);
}

fn in_view(app: &App, e: Entity) -> bool {
    app.world().entity(e).contains::<AiInView>()
}

#[test]
fn the_actor_in_front_is_in_view_and_swaps_when_the_viewer_turns() {
    let (mut app, viewer) = app();
    let front = actor(&mut app, 1, -10.0);
    let back = actor(&mut app, 2, 10.0);
    step(&mut app);
    assert!(in_view(&app, front) && !in_view(&app, back));
    // The seen one ranks first: it takes the only Full slot.
    let tier = |app: &App, e: Entity| app.world().get::<AiTier>(e).map(|t| t.tier);
    assert_eq!(tier(&app, front), Some(Tier::Full));
    assert_ne!(tier(&app, back), Some(Tier::Full));

    app.world_mut().get_mut::<AiViewer>(viewer).unwrap().0.rotation = Quat::from_rotation_y(PI);
    step(&mut app);
    assert!(!in_view(&app, front) && in_view(&app, back));
}

#[test]
fn no_viewer_and_dormant_actors_are_never_in_view() {
    let (mut app, viewer) = app();
    let front = actor(&mut app, 1, -10.0);
    let dormant = actor(&mut app, 2, -12.0);
    app.world_mut().entity_mut(dormant).insert(Dormant);
    step(&mut app);
    assert!(in_view(&app, front) && !in_view(&app, dormant));
    app.world_mut().despawn(viewer);
    step(&mut app);
    assert!(!in_view(&app, front));
}
