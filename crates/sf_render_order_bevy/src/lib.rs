//! The transparent draw order, in one place (water.md "Render order").
//!
//! Bevy sorts `Transparent3d` back to front by the view depth of each draw's
//! mesh AABB centre (plus the material's `depth_bias`). That key means nothing
//! for draws whose geometry is placed in the shader: the water clipmap and the
//! camera-wrapped precipitation boxes both sit at the world origin, so their
//! order flipped with the camera pitch, and the water (drawn second, writing
//! its refracted colour) erased the rain in front of it.
//!
//! Tag such draws with a [`DrawBand`]; [`RenderOrderPlugin`] rewrites their sort
//! keys before Bevy's sort, so `Transparent3d` draws
//! 1. [`DrawBand::Refractive`]: surfaces that refract the opaque scene (water),
//!    right after the opaque pass and its snapshot;
//! 2. [`DrawBand::Sorted`] (the default, untagged): every other translucent,
//!    back to front by Bevy's key;
//! 3. [`DrawBand::Overlay`]: camera-local volumes (precipitation, particles),
//!    last, ranked by `order` (ascending: higher draws on top).
//!
//! Crates that tag draws add the plugin if it is missing.

use bevy_app::{App, Plugin};
use bevy_core_pipeline::core_3d::{Transparent3d, TransparentSortingInfo3d};
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_render::render_phase::{ViewSortedRenderPhases, sort_phase_system};
use bevy_render::sync_world::{MainEntity, MainEntityHashMap};
use bevy_render::view::ExtractedView;
use bevy_render::{Extract, ExtractSchedule, Render, RenderApp, RenderSystems};

/// Where a blended draw goes in the transparent phase.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawBand {
    /// Refracts the opaque scene (water): drawn before every other translucent.
    Refractive,
    /// Back to front with the other translucents (Bevy's key; the default).
    #[default]
    Sorted,
    /// Camera-local volume (precipitation, particles): drawn after every
    /// sorted translucent; higher `order` draws later (on top).
    Overlay {
        /// Rank within the band (e.g. the far nested box first).
        order: i16,
    },
}

/// Overlay sort distances start here: past any real view depth (m).
pub const OVERLAY_BASE: f32 = 1.0e6;

impl DrawBand {
    /// The sorting info for a draw in this band, given Bevy's own and the
    /// view's world position.
    #[must_use]
    pub fn sorting_info(self, bevy: TransparentSortingInfo3d, view_position: Vec3) -> TransparentSortingInfo3d {
        match self {
            // Sorts first (`f32::NEG_INFINITY`) despite the name: the sort is ascending.
            Self::Refractive => TransparentSortingInfo3d::AlwaysOnTop,
            Self::Sorted => bevy,
            // The view position has view depth 0; the bias carries band + rank.
            Self::Overlay { order } => {
                TransparentSortingInfo3d::Sorted { mesh_center: view_position, depth_bias: OVERLAY_BASE + f32::from(order) }
            }
        }
    }
}

/// Adds the band policy to the render app.
pub struct RenderOrderPlugin;

impl Plugin for RenderOrderPlugin {
    fn build(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
        render_app.init_resource::<ExtractedBands>().add_systems(ExtractSchedule, extract_bands).add_systems(
            Render,
            apply_bands.in_set(RenderSystems::PhaseSort).before(sort_phase_system::<Transparent3d>),
        );
    }
}

/// The tagged draws (non-default bands only), by main-world entity.
#[derive(Resource, Default)]
struct ExtractedBands(MainEntityHashMap<DrawBand>);

#[allow(clippy::needless_pass_by_value)]
fn extract_bands(mut bands: ResMut<ExtractedBands>, tagged: Extract<Query<(Entity, &DrawBand)>>) {
    bands.0.clear();
    bands.0.extend(tagged.iter().filter(|(_, b)| **b != DrawBand::Sorted).map(|(e, b)| (MainEntity::from(e), *b)));
}

#[allow(clippy::needless_pass_by_value)]
fn apply_bands(bands: Res<ExtractedBands>, views: Query<&ExtractedView>, mut phases: ResMut<ViewSortedRenderPhases<Transparent3d>>) {
    if bands.0.is_empty() {
        return;
    }
    for view in &views {
        let Some(phase) = phases.get_mut(&view.retained_view_entity) else { continue };
        let position = view.world_from_view.translation();
        for item in phase.items.values_mut() {
            if let Some(band) = bands.0.get(&item.entity.1) {
                item.sorting_info = band.sorting_info(item.sorting_info, position);
            }
        }
    }
}

#[cfg(test)]
mod tests;
