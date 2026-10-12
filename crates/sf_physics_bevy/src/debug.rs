//! Collider wireframes around the focus (the `debug` feature).

use bevy_app::{App, Update};
use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_gizmos::prelude::Gizmos;
use bevy_math::Vec3;
use sf_coords::WorldPos;
use sf_physics::Layer;
use sf_world_bevy::WorldOrigin;

use crate::{Physics, PhysicsFocus, PhysicsSystems};

/// Collider wireframes of the active world within `radius` of the focus:
/// statics cyan, terrain green, everything else orange.
#[derive(Resource, Clone, Debug)]
pub struct PhysicsDebug {
    pub enabled: bool,
    pub radius: f32,
    /// Segment cap per frame.
    pub max_lines: usize,
}

impl Default for PhysicsDebug {
    fn default() -> Self {
        Self { enabled: false, radius: 40.0, max_lines: 400_000 }
    }
}

pub(crate) fn add(app: &mut App) {
    // Without the app's gizmo plugin (headless tests), nothing to draw with.
    let gizmos = resource_exists::<bevy_gizmos::config::GizmoConfigStore>;
    app.init_resource::<PhysicsDebug>().add_systems(Update, draw.after(PhysicsSystems::WriteBack).run_if(gizmos));
}

#[allow(clippy::needless_pass_by_value)]
fn draw(
    debug: Res<PhysicsDebug>,
    physics: Res<Physics>,
    focus: Res<PhysicsFocus>,
    origin: Option<Res<WorldOrigin>>,
    mut gizmos: Gizmos,
    mut lines: Local<Vec<(Vec3, Vec3, Layer)>>,
) {
    if !debug.enabled {
        return;
    }
    let (Some(world), Some(center)) = (physics.worlds.active_world(), focus.0) else { return };
    let frame = WorldPos { cell: origin.map_or(center.cell, |o| o.cell), local: Vec3::ZERO };
    lines.clear();
    world.debug_lines(center, debug.radius, frame, debug.max_lines, &mut lines);
    for &(a, b, layer) in &*lines {
        let color = if layer == Layer::STATIC {
            Color::srgb(0.2, 0.9, 1.0)
        } else if layer == Layer::TERRAIN {
            Color::srgb(0.3, 1.0, 0.3)
        } else {
            Color::srgb(1.0, 0.6, 0.1)
        };
        gizmos.line(a, b, color);
    }
}
