//! Shadow-caster size culling per sun cascade (shadows.md "Caster rules",
//! ROADMAP 1.2/1.4, audit J4): a caster whose bounding radius is under
//! [`CasterCulling::min_texels`] texels of a cascade leaves that cascade's
//! draw list. It would cover a texel or two there, which the filter blurs
//! away, but it still cost a draw in every cascade it touched. This is UE's
//! `r.Shadow.RadiusThreshold` per cascade; Bevy 0.20 culls casters only by
//! frustum and visibility range.
//!
//! Runs in `PostUpdate` right after Bevy's `check_dir_light_mesh_visibility`
//! and before extraction, on the lists Bevy just built (every frame), so
//! nothing has to be undone when a caster grows or a cascade moves.
//! Counters: `shadow/cascade_casters` (entries left in all cascade lists)
//! and `shadow/casters_culled` (entries this rule removed), per frame.

use bevy_app::{App, PostUpdate};
use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::{CascadesVisibleEntities, VisibilitySystems};
use bevy_ecs::prelude::*;
use bevy_light::{Cascades, DirectionalLight, SimulationLightSystems};
use bevy_tasks::ComputeTaskPool;
use bevy_transform::components::GlobalTransform;
use sf_diag_bevy::DiagCounters;
use sf_diag_bevy::sf_diag::unit;

/// The caster size rule (a quality setting).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct CasterCulling {
    /// Casters with a bounding radius under this many texels of a cascade
    /// don't draw into it. 0 turns the rule off.
    pub min_texels: f32,
}

impl Default for CasterCulling {
    /// Off (2026-10-19): measured ~0.3 ms in Balmora, and the cascades refit
    /// to the view as the camera turns, so small casters crossing a cascade
    /// boundary lost and regained their shadows (owner: shadows following
    /// the camera angle). A rule that pays must be stable per caster.
    fn default() -> Self {
        Self { min_texels: 0.0 }
    }
}

pub(crate) fn build(app: &mut App) {
    app.init_resource::<CasterCulling>().add_systems(
        PostUpdate,
        cull_small_casters
            .after(SimulationLightSystems::CheckLightVisibility)
            .after(VisibilitySystems::CheckVisibility),
    );
}

/// Whether a caster of bounding radius `radius` stays in a cascade with
/// texels of `texel_size` meters.
pub fn keeps_caster(radius: f32, texel_size: f32, min_texels: f32) -> bool {
    radius >= min_texels * texel_size
}

#[allow(clippy::cast_precision_loss)] // counts far below 2^52
fn cull_small_casters(
    settings: Res<CasterCulling>,
    mut lights: Query<(&Cascades, &mut CascadesVisibleEntities), With<DirectionalLight>>,
    bounds: Query<(&Aabb, &GlobalTransform)>,
    counters: Option<Res<DiagCounters>>,
) {
    let count = |lights: &Query<(&Cascades, &mut CascadesVisibleEntities), With<DirectionalLight>>| -> usize {
        lights.iter().flat_map(|(_, v)| v.entities.values()).flatten().map(|l| l.entities.len()).sum()
    };
    let before = counters.as_ref().map(|_| count(&lights));
    if settings.min_texels <= 0.0 {
        if let (Some(c), Some(n)) = (&counters, before) {
            c.set_named("shadow/cascade_casters", unit::COUNT, n as f64);
        }
        return;
    }
    let min_texels = settings.min_texels;
    for (cascades, mut visible) in &mut lights {
        for (view, lists) in &mut visible.entities {
            let Some(view_cascades) = cascades.cascades.get(view) else { continue };
            // One task per cascade: the lists are independent.
            ComputeTaskPool::get().scope(|scope| {
                for (list, cascade) in lists.iter_mut().zip(view_cascades) {
                    let bounds = &bounds;
                    scope.spawn(async move {
                        list.entities.retain(|e| {
                            bounds.get(*e).ok().is_none_or(|(aabb, t)| {
                                keeps_caster(t.radius_vec3a(aabb.half_extents), cascade.texel_size, min_texels)
                            })
                        });
                    });
                }
            });
        }
    }
    if let (Some(c), Some(n)) = (&counters, before) {
        let after = count(&lights);
        c.set_named("shadow/cascade_casters", unit::COUNT, after as f64);
        c.set_named("shadow/casters_culled", unit::COUNT, n.saturating_sub(after) as f64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// High's cascades (1.4 / 3.8 / 11 / 35 cm texels): a 10 cm cup keeps
    /// its near shadow and leaves the far cascades; a house stays in all.
    #[test]
    fn small_casters_leave_coarse_cascades_only() {
        let texels = [0.014, 0.038, 0.11, 0.35];
        let kept = |r: f32| texels.map(|t| keeps_caster(r, t, 1.5));
        assert_eq!(kept(0.1), [true, true, false, false]);
        assert_eq!(kept(5.0), [true; 4]);
        assert_eq!(kept(0.01), [false; 4]);
    }
}
