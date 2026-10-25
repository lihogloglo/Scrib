//! Ring overlay (feature `debug`): the active worldspace's streaming cells
//! as gizmo boxes colored by level and request state, and each anchor's
//! load radii as circles (streaming.md, "Verification": checked visually).

use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_gizmos::gizmos::Gizmos;
use bevy_shape::Aabb3d;
use bevy_math::{Isometry3d, Quat, Vec3};
use bevy_transform::components::Transform;
use big_space::prelude::{CellCoord, Grid};
use sf_world::{AnchorRole, Level, RequestKind, Stage};

use crate::coords::{cell_ivec, from_grid, origin_relative};
use crate::stream::{ActiveWorldspace, Streaming, StreamingAnchor};
use crate::worldspace::WorldspaceRoot;

/// Overlay switches.
#[derive(Resource, Clone, Copy, Debug)]
pub struct StreamingDebug {
    pub enabled: bool,
    /// Also outline cells with a pending request.
    pub requests: bool,
    /// Anchor ring circles.
    pub rings: bool,
}

impl Default for StreamingDebug {
    fn default() -> Self {
        Self { enabled: false, requests: true, rings: true }
    }
}

/// Colors: far = blue, near = green, active = orange; loads in flight
/// white (fetch/assets) or yellow (spawn/upload), unloads red.
pub fn level_color(level: Level) -> Color {
    match level {
        Level::Unloaded => Color::srgb(0.4, 0.4, 0.4),
        Level::Far => Color::srgb(0.25, 0.45, 1.0),
        Level::Near => Color::srgb(0.2, 0.9, 0.3),
        Level::Active => Color::srgb(1.0, 0.55, 0.1),
    }
}

pub(crate) fn draw_rings(
    debug: Res<StreamingDebug>,
    streaming: Res<Streaming>,
    active: Res<ActiveWorldspace>,
    roots: Query<(Entity, &WorldspaceRoot, &Grid)>,
    anchors: Query<(&StreamingAnchor, &CellCoord, &Transform, &ChildOf)>,
    mut gizmos: Gizmos,
) {
    if !debug.enabled {
        return;
    }
    let Some(ws) = &active.0 else { return };
    let Some((root, _, grid)) = roots.iter().find(|(_, r, _)| r.id == *ws) else { return };
    let origin_cell = cell_ivec(&grid.local_floating_origin().cell());
    let scheduler = streaming.scheduler();
    let Some(dir) = scheduler.directory(ws) else { return };
    let partition = dir.partition;

    let cell_box = |cell| -> Option<Aabb3d> {
        let i = dir.find(cell)?;
        let summary = &dir.cells[i];
        let base = origin_relative(origin_cell, partition.cell_origin(cell));
        let (min, max) = if summary.bounds.is_empty() {
            let edge = partition.cell_extent().unwrap_or(1.0);
            (Vec3::ZERO, Vec3::splat(edge))
        } else {
            (summary.bounds.min, summary.bounds.max)
        };
        // Shrink a little so neighbors' outlines stay apart.
        let center = base + (min + max) * 0.5;
        let half = ((max - min) * 0.5 - Vec3::splat(0.5)).max(Vec3::splat(0.25));
        Some(Aabb3d::new(center, half))
    };

    for (cell, level) in scheduler.resident(ws) {
        if let Some(b) = cell_box(cell) {
            gizmos.aabb_3d(b, Transform::IDENTITY, level_color(level));
        }
    }
    if debug.requests {
        for r in scheduler.requests().filter(|r| r.worldspace == *ws) {
            let color = match (r.kind, r.stage) {
                (RequestKind::Unload, _) => Color::srgb(1.0, 0.15, 0.15),
                (_, Stage::Fetch | Stage::Assets) => Color::WHITE,
                _ => Color::srgb(1.0, 1.0, 0.2),
            };
            if let Some(mut b) = cell_box(r.cell) {
                // Requests draw slightly inside the level box.
                b.max -= bevy_math::Vec3A::splat(1.0);
                b.min += bevy_math::Vec3A::splat(1.0);
                gizmos.aabb_3d(b, Transform::IDENTITY, color);
            }
        }
    }
    if debug.rings {
        let rings = scheduler.config().rings;
        for (anchor, cell, transform, parent) in &anchors {
            if parent.parent() != root {
                continue;
            }
            let center = origin_relative(origin_cell, from_grid(cell, transform));
            let flat = Isometry3d::new(center, Quat::from_rotation_x(std::f32::consts::FRAC_PI_2));
            let radii: &[(Level, f32)] = match anchor.role {
                AnchorRole::Gameplay => &[(Level::Active, rings.active.load)],
                AnchorRole::Visual => &[(Level::Near, rings.near.load), (Level::Far, rings.far.load)],
            };
            for &(level, radius) in radii {
                gizmos.circle(flat, radius * anchor.radius_scale, level_color(level)).resolution(128);
            }
        }
    }
}
