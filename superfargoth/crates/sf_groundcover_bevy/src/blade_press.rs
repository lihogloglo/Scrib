//! The trail ring's press drives the blades (slice GC-3d2): the glue between `sf_surface_bevy`'s ring
//! ([`TrailViews`], render world) and the vendored `bevy_grass` external trample source
//! ([`GrassTrampleSource`], own patch "trample source"), generic for any game with both plugins.
//!
//! Every frame the render world's [`GrassTrampleSource`] gets this frame's ring and last frame's, with their
//! placements ([`ring_placement`]: the ring's `place`/`cells` in the blade roots' space, which is render space
//! as the ring's is) and the lookup the terrain and the mesh plants use ([`ring_lookup`]: A, the same edge
//! margin and fade as `layer.wesl` `surface_ring_lookup`, the press resolution as the direction floor). The
//! blades then read the press the mesh plants read, the same frame: the ring pass runs before every camera
//! and before the blade generate ([`order_generate_after_trail`]).
//!
//! While the ring is off (no placement) or `groundcover.render.trample_amount` is 0 the source stays, with
//! both placements [`GrassTramplePlacement::NONE`]: nothing is fetched and the crate's own trample passes
//! (decay, stamp) stay skipped. Only without the trail plugin's render part (no [`TrailViews`]) is there no
//! source, and the crate tramples by itself (no interactors are spawned, so nothing).

use bevy_ecs::prelude::*;
use bevy_grass::{GrassGenerateSystems, GrassTrampleLookup, GrassTramplePlacement, GrassTrampleSource};
use bevy_math::{IVec4, UVec2, Vec4, Vec4Swizzles};
use bevy_render::renderer::RenderGraph;
use bevy_render::{Extract, ExtractSchedule, Render, RenderSystems};
use sf_surface_bevy::sf_surface::trail::PRESS_FLOOR;
use sf_surface_bevy::trail::{RING_EDGE_FADE_M, RING_EDGE_MARGIN_M};
use sf_surface_bevy::{TrailPassSystems, TrailPlacement, TrailViews};
use sf_tunables_bevy::Tunables;

use crate::read;
use crate::tunables::TRAMPLE_AMOUNT;

/// The ring channel that holds the press (A, GC-T1).
// tunables: const (format: the ring's channel layout, `sf_surface_bevy::trail`)
const PRESS_CHANNEL: u32 = 3;

/// A ring placement (`SurfaceUniform::trail` layout: `place` xy corner, z texels/m; `cells` xy first texel,
/// z size) as `bevy_grass`'s; [`GrassTramplePlacement::NONE`] when no ring is placed.
#[must_use]
#[allow(clippy::cast_sign_loss)]
pub fn ring_placement(place: Vec4, cells: IVec4) -> GrassTramplePlacement {
    let scaled = place.z > 0.0; // false for NaN too
    if !scaled || cells.z <= 0 || cells.x < 0 || cells.y < 0 {
        return GrassTramplePlacement::NONE;
    }
    GrassTramplePlacement {
        corner: place.xy(),
        texels_per_metre: place.z,
        size: cells.z as u32,
        first_texel: UVec2::new(cells.x as u32, cells.y as u32),
    }
}

/// How the blades read the ring: the press channel, `amount` (`groundcover.render.trample_amount`) as the
/// strength, the ring's own edge margin and fade, the press resolution as the direction floor (as the mesh
/// plants' `TRAIL_PRESS_FLOOR`).
#[must_use]
pub fn ring_lookup(amount: f32) -> GrassTrampleLookup {
    GrassTrampleLookup {
        channel: PRESS_CHANNEL,
        strength: amount.max(0.0),
        edge_margin: RING_EDGE_MARGIN_M,
        edge_fade: RING_EDGE_FADE_M,
        direction_floor: PRESS_FLOOR,
    }
}

/// The source's placements (this frame, last frame) and lookup from the ring's placements (`None`: no ring
/// views, no source). Amount 0 (or below) presses nothing: both placements are none.
#[must_use]
pub fn press_source(
    ring: Option<(TrailPlacement, TrailPlacement)>,
    amount: f32,
) -> Option<(GrassTramplePlacement, GrassTramplePlacement, GrassTrampleLookup)> {
    let (now, then) = ring?;
    let lookup = ring_lookup(amount);
    if lookup.strength <= 0.0 {
        return Some((GrassTramplePlacement::NONE, GrassTramplePlacement::NONE, lookup));
    }
    Some((ring_placement(now.place, now.cells), ring_placement(then.place, then.cells), lookup))
}

/// Render world: this frame's `groundcover.render.trample_amount` (live).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
struct BladePressAmount(f32);

impl Default for BladePressAmount {
    fn default() -> Self {
        Self(TRAMPLE_AMOUNT.default_value())
    }
}

#[allow(clippy::needless_pass_by_value)]
fn extract_amount(tunables: Extract<Option<Res<Tunables>>>, mut amount: ResMut<BladePressAmount>) {
    let next = tunables.as_deref().map_or(TRAMPLE_AMOUNT.default_value(), |t| read(&t.0, &TRAMPLE_AMOUNT));
    if amount.0.to_bits() != next.to_bits() {
        amount.0 = next;
    }
}

/// Writes [`GrassTrampleSource`] from [`TrailViews`] (before `bevy_grass` builds its bind group in
/// `PrepareBindGroups`); removes only a source it wrote, once the views are gone.
#[allow(clippy::needless_pass_by_value)]
fn write_source(
    mut commands: Commands,
    views: Option<Res<TrailViews>>,
    amount: Res<BladePressAmount>,
    source: Option<ResMut<GrassTrampleSource>>,
    mut wrote: Local<bool>,
) {
    let ring = views.as_deref().map(|v| (v.placement, v.previous_placement));
    let (Some(views), Some((current_placement, previous_placement, lookup))) = (views, press_source(ring, amount.0)) else {
        if std::mem::take(&mut *wrote) {
            commands.remove_resource::<GrassTrampleSource>();
        }
        return;
    };
    let next = GrassTrampleSource {
        current: views.current.clone(),
        previous: views.previous.clone(),
        current_placement,
        previous_placement,
        lookup,
    };
    *wrote = true;
    match source {
        Some(mut source) => *source = next,
        None => commands.insert_resource(next),
    }
}

/// Orders the blade generate after the ring pass in the root render graph (both run before
/// `camera_driver`), so the blades read the ring this frame's pass wrote.
pub fn order_generate_after_trail(schedule: &mut Schedule) {
    schedule.configure_sets(GrassGenerateSystems.after(TrailPassSystems));
}

/// The render app's part (called by [`BladesPlugin`](crate::blades::BladesPlugin)).
pub(crate) fn build(render_app: &mut bevy_app::SubApp) {
    render_app
        .init_resource::<BladePressAmount>()
        .add_systems(ExtractSchedule, extract_amount)
        .add_systems(Render, write_source.in_set(RenderSystems::PrepareResources));
    render_app.edit_schedule(RenderGraph, order_generate_after_trail);
}
