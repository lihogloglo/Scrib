//! `NV11d1`: `NavWorldGraph` rebuilt from a host link resource, headless. Exact: link and skip
//! counts, rebuild counts, the `nav.links` line, which links a route takes.
//!
//! Spaces: exterior `e`, interiors `a` and `b`. Links: `ea` (e-a), `ab` (a-b), `eb` (one-way
//! b -> e). The host resource is a local stand-in shaped like `sf_world_bevy::DoorLinks`.

use bevy::MinimalPlugins;
use bevy_app::App;
use bevy_ecs::prelude::*;
use glam::{IVec3, Vec2, Vec3};
use sf_console_bevy::sf_console::Level;
use sf_console_bevy::{ConsolePlugin, ConsoleSubmit, DevConsole};
use sf_coords::{WorldPos, WorldTransform};
use sf_nav::{DoorState, RouteRules, WorldRoute};
use sf_world::{DoorLink, LinkEnd, LinkId, Location, WorldspaceId};

use crate::{
    NavDoorStates, NavLinkRules, NavPlugin, NavWorldGraph, NavWorldGraphPlugin, NavWorldGraphStats,
};

#[derive(Resource)]
struct HostLinks(Vec<DoorLink>);

fn pos(x: f32, y: f32) -> WorldPos {
    WorldPos::new(IVec3::ZERO, Vec3::new(x, y, 0.0))
}

fn loc(ws: &str, x: f32, y: f32) -> Location {
    Location::new(WorldspaceId::new(ws), pos(x, y))
}

fn end(ws: &str, x: f32, y: f32) -> LinkEnd {
    let frame = WorldTransform::from_pos(pos(x, y));
    LinkEnd {
        worldspace: WorldspaceId::new(ws),
        frame,
        arrival: frame,
        door: None,
        extent: Vec2::ZERO,
    }
}

fn ea() -> DoorLink {
    DoorLink {
        a: end("e", 0.0, 0.0),
        b: end("a", 0.0, 0.0),
        two_way: true,
    }
}

fn ab() -> DoorLink {
    DoorLink {
        a: end("a", 6.0, 8.0),
        b: end("b", 0.0, 0.0),
        two_way: true,
    }
}

fn eb() -> DoorLink {
    DoorLink {
        a: end("b", 80.0, 0.0),
        b: end("e", 0.0, 50.0),
        two_way: false,
    }
}

fn app(links: Vec<DoorLink>) -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        ConsolePlugin,
        NavPlugin,
        NavWorldGraphPlugin::new(|l: &HostLinks| &l.0),
    ))
    .insert_resource(HostLinks(links));
    app.update();
    app
}

fn stats(app: &App) -> NavWorldGraphStats {
    app.world().resource::<NavWorldGraphStats>().clone()
}

/// The route e -> b under the host's door states, walking 1 m/s.
fn route_e_b(app: &App) -> Option<WorldRoute> {
    let world = app.world();
    let states = world.resource::<NavDoorStates>();
    world.resource::<NavWorldGraph>().0.route(
        &loc("e", 3.0, 4.0),
        &loc("b", 0.0, 0.0),
        &|id| states.state(id),
        &RouteRules::walking(1.0),
    )
}

fn links_line(app: &mut App) -> Vec<(Level, String)> {
    let before = app.world().resource::<DevConsole>().log().lines().len();
    app.world_mut()
        .write_message(ConsoleSubmit::exec("nav.links"));
    app.update();
    let log = app.world().resource::<DevConsole>().log();
    log.lines()
        .skip(before)
        .map(|l| (l.level, l.text.clone()))
        .collect()
}

#[test]
fn builds_from_host_links_and_routes() {
    let mut app = app(vec![ea(), ab(), eb()]);
    let s = stats(&app);
    assert_eq!((s.links, s.skipped, s.rebuilds), (3, 0, 1));
    assert_eq!(app.world().resource::<NavWorldGraph>().0.len(), 3);
    let route = route_e_b(&app).expect("e -> a -> b");
    assert_eq!(route.links, vec![ea().id(), ab().id()]);
    // Nothing changed: no rebuild.
    app.update();
    assert_eq!(stats(&app).rebuilds, 1);
    assert_eq!(
        links_line(&mut app),
        vec![(
            Level::Info,
            "nav: world graph 3 links (0 skipped), 5 exits: a 2, b 2, e 1; 1 rebuilds".to_owned()
        )]
    );
}

#[test]
fn replacing_links_rebuilds() {
    let mut app = app(vec![ea(), ab(), eb()]);
    app.world_mut().resource_mut::<HostLinks>().0 = vec![ea(), eb()];
    app.update();
    let s = stats(&app);
    assert_eq!((s.links, s.skipped, s.rebuilds), (2, 0, 2));
    assert!(route_e_b(&app).is_none(), "ab is gone and eb only leaves b");
}

#[test]
fn rules_without_an_approach_point_skip_the_link() {
    let mut app = app(vec![ea(), ab(), eb()]);
    // No walkable point in front of ab's `a` end.
    let blocked = ab().a.frame.pos;
    app.insert_resource(NavLinkRules::new(
        move |end: &LinkEnd| {
            if end.frame.pos == blocked {
                None
            } else {
                NavLinkRules::arrival_approach(end)
            }
        },
        NavLinkRules::classic_flags,
    ));
    app.update();
    let s = stats(&app);
    assert_eq!((s.links, s.skipped, s.rebuilds), (2, 1, 2));
    let graph = &app.world().resource::<NavWorldGraph>().0;
    assert!(graph.door_link(ab().id()).is_none());
    assert!(route_e_b(&app).is_none());
}

#[test]
fn locked_door_refuses_the_route() {
    let mut app = app(vec![ea(), ab(), eb()]);
    let ab_id: LinkId = ab().id();
    // Unreported reads Closed: passable.
    assert_eq!(
        app.world().resource::<NavDoorStates>().state(ab_id),
        DoorState::Closed
    );
    assert!(route_e_b(&app).is_some());
    app.world_mut()
        .resource_mut::<NavDoorStates>()
        .0
        .insert(ab_id, DoorState::Locked);
    assert!(route_e_b(&app).is_none(), "the only way into b is locked");
    app.world_mut()
        .resource_mut::<NavDoorStates>()
        .0
        .insert(ab_id, DoorState::Open);
    assert_eq!(
        route_e_b(&app).expect("open").links,
        vec![ea().id(), ab().id()]
    );
}
