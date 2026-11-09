//! Feature `debug`: the selected actor's overlay as immediate-mode gizmos (UE Gameplay Debugger
//! style: the selected actor gets a ring at its feet and its decision text above its head, facing
//! the camera). Bevy 0.20's stroke-font `Gizmos::text` draws the text, so there's no UI tree to
//! keep in sync; the lines are [`overlay_lines`]. Tier colours, paths, cones and claims wait for
//! the data (AI6 paths, AI8 perception, AI9 tiers, AI14 claims): nothing is drawn for missing data.
//!
//! Drawn only when the app has Bevy's gizmo plugin (headless tests have none) and the console
//! variable `ai.gizmos` is on. The ring's colour is the last plan outcome: green planned,
//! continued or replaced; amber kept (a replan didn't win); red no plan; grey nothing traced.

use bevy_app::{App, PostUpdate};
use bevy_camera::Camera;
use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_gizmos::config::GizmoConfigStore;
use bevy_gizmos::prelude::Gizmos;
use bevy_math::{Isometry3d, Quat, Vec2, Vec3};
use bevy_transform::TransformSystems;
use bevy_transform::components::GlobalTransform;
use sf_ai::{PlanOutcome, Trace};
use sf_console_bevy::overlay::ConsoleOverlayAppExt;
use sf_console_bevy::sf_console::{VarId, VarSpec};
use sf_console_bevy::{ConsoleAppExt, console_flag};

use crate::{AiActor, AiLibrary, AiSelected, AiTraces, actor_hex, overlay_lines};

/// The `ai.gizmos` console variable's id.
#[derive(Resource, Clone, Copy, Debug)]
pub struct AiGizmoVar(pub VarId);

/// Text height, meters (the stroke font scales to it in world space).
const TEXT_SIZE: f32 = 0.12;
/// Line spacing, meters.
const LINE_STEP: f32 = 0.17;
/// Bottom text line above the actor's origin (its feet), meters.
const TEXT_LIFT: f32 = 2.1;
const RING_RADIUS: f32 = 0.55;

pub(crate) fn add(app: &mut App) {
    let var = app.add_console_var(VarSpec::bool("ai.gizmos", true).help("Draw the selected AI actor's plan overlay"));
    app.insert_resource(AiGizmoVar(var))
        .add_systems(
            PostUpdate,
            draw.after(TransformSystems::Propagate)
                .run_if(resource_exists::<GizmoConfigStore>)
                .run_if(console_flag(var)),
        )
        .add_inspector_rows(|world, e, rows| {
            if let Some(actor) = world.get::<AiActor>(e) {
                let traced = world.resource::<AiTraces>().get(actor.0).map_or(0, Trace::len);
                rows.push(("ai actor".into(), format!("{} ({traced} trace entries)", actor_hex(actor.0))));
            }
        });
}

fn draw(
    selected: Res<AiSelected>,
    traces: Res<AiTraces>,
    library: Res<AiLibrary>,
    actors: Query<(&AiActor, &GlobalTransform)>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    let Some(key) = selected.0 else { return };
    // The actor isn't loaded (or isn't spawned yet): `ai.trace` still reads it; nothing to draw.
    let Some((_, at)) = actors.iter().find(|(a, _)| a.0 == key) else { return };
    let feet = at.translation();
    let facing = cameras
        .iter()
        .filter(|(c, _)| c.is_active)
        .max_by_key(|(c, _)| c.order)
        .map_or(Quat::IDENTITY, |(_, t)| t.rotation());
    let outcome = traces.get(key).and_then(Trace::last).map(|e| e.outcome);
    // The last arm also takes outcomes `sf_ai` adds later (white), so this match never breaks.
    let color = match outcome {
        None => Color::srgb(0.6, 0.6, 0.6),
        Some(Some(PlanOutcome::Planned | PlanOutcome::Continued | PlanOutcome::Replaced)) => Color::srgb(0.3, 0.9, 0.4),
        Some(Some(PlanOutcome::KeptNotBetter | PlanOutcome::KeptNoPlan)) => Color::srgb(1.0, 0.75, 0.2),
        Some(Some(PlanOutcome::NoPlan)) => Color::srgb(1.0, 0.3, 0.25),
        Some(_) => Color::WHITE,
    };
    gizmos.circle(
        Isometry3d::new(feet + Vec3::Y * 0.02, Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        RING_RADIUS,
        color,
    );
    let lines = overlay_lines(&traces, &library.library, key);
    let up = facing * Vec3::Y;
    let top = feet + Vec3::Y * TEXT_LIFT;
    for (i, line) in lines.iter().enumerate() {
        #[allow(clippy::cast_precision_loss)]
        let pos = top - up * (i as f32 * LINE_STEP);
        let line_color = if i == 0 { color } else { Color::WHITE };
        gizmos.text(Isometry3d::new(pos, facing), line, TEXT_SIZE, Vec2::ZERO, line_color);
    }
}
