//! Door approach points, Bevy-free (`MA21`, moved here by `NV12c2`): the game's worldspace graph
//! (`mw_exterior_bevy::nav_links`) and the route bake (`mw_bake::nav::routes`) call the one
//! [`approach_point`], so a route graph's door nodes sit exactly on the `WorldGraph` approaches.
//!
//! Spec: `bevyrrowind/docs/specs/ai-adapter.md` (MA21, "Route source" rule 5).

use glam::{EulerRot, Quat, Vec3};
use mw_world::ids;
use sf_coords::WorldPos;
use sf_tunables::{Def, Lifetime, Net, Scope, Source, Unit, def};
use sf_world::LinkEnd;

use crate::pathgrid::{CellKey, PathGraph, PathGrids};

/// `mw.nav.door_approach_m`: the farthest pathgrid node [`approach_point`] takes.
pub const DOOR_APPROACH_M: Def<f32> = def!(float "mw.nav.door_approach_m" = 4.0)
    .range(0.5, 16.0)
    .unit(Unit::Meters)
    .help("A door's approach point is the pathgrid node in front of it nearest its arrival point, at most this far; none: the arrival point")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// The yaw part of an arrival rotation (Y-Z-X Euler), as `mw_doors_bevy::arrival_rotation`
/// turns travellers on arrival.
pub fn arrival_yaw(rotation: Quat) -> Quat {
    let (yaw, _, _) = rotation.to_euler(EulerRot::YZX);
    Quat::from_rotation_y(yaw)
}

/// The pathgrid node an approach to `end` stands on: among the nodes of the end's cell (the
/// exterior: the arrival's grid square and its neighbours), those not behind the door plane
/// (`end.frame.pos`, facing the arrival's yaw) and within `max_m` of the arrival, the nearest to
/// the arrival. `None`: no such node.
pub fn approach_point(grids: &PathGrids, end: &LinkEnd, max_m: f32) -> Option<WorldPos> {
    let arrival = end.arrival.pos;
    let forward = arrival_yaw(end.arrival.rotation) * Vec3::NEG_Z;
    let keys: Vec<CellKey> = if end.worldspace == ids::exterior() {
        let CellKey::Exterior(x, y) = CellKey::exterior_at(arrival) else {
            unreachable!("exterior_at gives an exterior key")
        };
        (-1..=1).flat_map(|dy| (-1..=1).map(move |dx| CellKey::Exterior(x + dx, y + dy))).collect()
    } else {
        vec![CellKey::interior(end.worldspace.as_str())]
    };
    let max2 = max_m * max_m;
    keys.iter()
        .filter_map(|k| grids.get(k))
        .flat_map(PathGraph::positions)
        .filter(|&p| end.frame.pos.delta_f32(p).dot(forward) >= 0.0)
        .map(|p| (p, arrival.delta_f32(p).length_squared()))
        .filter(|&(_, d2)| d2 <= max2)
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(p, _)| p)
}
