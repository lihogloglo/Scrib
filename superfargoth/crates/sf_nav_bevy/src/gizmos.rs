//! Feature `debug`: the `nav.draw` layers as immediate-mode gizmos (nav.md "Debug draw"). Tiles
//! draw as Detour's `duDebugDrawNavMeshPolyBoundaries` does: every polygon edge in its area's
//! colour, inner edges faint and walls (edges with no neighbour) solid; disabled polygons grey.
//! Paths draw as a yellow line through the recorded waypoints with a tick at each.
//!
//! Immediate mode, rebuilt from the backend every frame: tiles stream and door patches change
//! polygon state, so a retained mesh would need invalidation for no gain in a debug view. Drawn
//! only when the app has Bevy's gizmo plugin and a layer is on.

use bevy_app::{App, PostUpdate};
use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_gizmos::config::GizmoConfigStore;
use bevy_gizmos::gizmos::Gizmos;
use glam::Vec3;
use sf_nav::{AreaId, PolyFlags, TileGeometry};

use crate::{NavDebugPaths, NavDraw, NavDrawFrame, NavDrawn, NavMeshes, PathLayer};

/// Lift above the polygons, meters, so lines don't z-fight with the ground they lie on.
const LIFT: f32 = 0.05;
/// Paths sit above the tiles.
const PATH_LIFT: f32 = 0.12;
/// Waypoint tick height, meters.
const TICK: f32 = 0.3;

pub(crate) fn add(app: &mut App) {
    app.add_systems(
        PostUpdate,
        draw.run_if(resource_exists::<GizmoConfigStore>).run_if(
            |d: Res<NavDraw>, n: Res<NavDrawn>| {
                d.tiles || d.paths.is_some() || *n != NavDrawn::default()
            },
        ),
    );
}

/// An area's colour: golden-angle hues, so neighbouring ids differ (Detour's `duIntToCol`).
fn area_color(area: AreaId, flags: PolyFlags, alpha: f32) -> Color {
    if flags.contains(PolyFlags::DISABLED) {
        return Color::srgba(0.5, 0.5, 0.5, alpha);
    }
    let hue = (f32::from(area.0) * 137.508) % 360.0;
    Color::hsla(hue, 0.85, 0.55, alpha)
}

fn draw_tile(gizmos: &mut Gizmos, geo: &TileGeometry, base: Vec3) -> usize {
    let lift = base + Vec3::Y * LIFT;
    for poly in &geo.polys {
        let n = poly.verts.len();
        let inner = area_color(poly.state.area, poly.state.flags, 0.35);
        let wall = area_color(poly.state.area, poly.state.flags, 1.0);
        for i in 0..n {
            let (Some(a), Some(b)) = (
                geo.verts.get(poly.verts[i] as usize),
                geo.verts.get(poly.verts[(i + 1) % n] as usize),
            ) else {
                continue;
            };
            let is_wall = i < 8 && poly.walls & (1 << i) != 0;
            gizmos.line(*a + lift, *b + lift, if is_wall { wall } else { inner });
        }
    }
    geo.polys.len()
}

fn draw(
    meshes: Option<Res<NavMeshes>>,
    layers: Res<NavDraw>,
    frame: Res<NavDrawFrame>,
    paths: Res<NavDebugPaths>,
    mut drawn: ResMut<NavDrawn>,
    mut gizmos: Gizmos,
) {
    let mut count = NavDrawn::default();
    let shown = |ws: &sf_world::WorldspaceId| frame.worldspace.as_ref().is_none_or(|w| w == ws);
    if layers.tiles
        && let Some(meshes) = &meshes
    {
        for t in meshes.0.tile_summaries() {
            if !shown(&t.key.ws) {
                continue;
            }
            let Some(geo) = meshes.0.tile_geometry(&t.key) else {
                continue;
            };
            let base = frame.origin.delta_f32(geo.origin);
            count.polys += draw_tile(&mut gizmos, &geo, base);
            count.tiles += 1;
        }
    }
    if let Some(layer) = &layers.paths {
        let color = Color::srgb(1.0, 0.9, 0.15);
        for (actor, points) in &paths.0 {
            if matches!(layer, PathLayer::Actor(a) if a != actor) {
                continue;
            }
            let at: Vec<Vec3> = points
                .iter()
                .filter(|p| shown(&p.worldspace))
                .map(|p| frame.origin.delta_f32(p.pos) + Vec3::Y * PATH_LIFT)
                .collect();
            gizmos.linestrip(at.iter().copied(), color);
            for p in &at {
                gizmos.line(*p, *p + Vec3::Y * TICK, color);
            }
            count.path_points += at.len();
        }
    }
    if *drawn != count {
        *drawn = count;
    }
}
