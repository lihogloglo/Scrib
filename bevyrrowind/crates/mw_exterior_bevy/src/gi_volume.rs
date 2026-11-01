//! Where the dynamic probe GI volume goes (`superfargoth/docs/specs/gi.md` "Probe volumes"):
//! one grid per interior worldspace (one that loads whole, `Partition::Single`), fitted to the
//! worldspace's content bounds from the world index (the cell directory's content AABB, known
//! before any placement streams in) with `sf_gi::fit_volume`, padded by a probe and within the
//! probe budget. When the active worldspace changes (a door teleport, at the fade's cut) the
//! old volume goes at once and the new worldspace gets its own fit; streamed worldspaces
//! (exteriors) get none until the camera-following cascades. While the fade holds at black,
//! the new volume keeps it black until its first full trace shows (`FadeTransition::hold`,
//! capped by the fade's gate limit), so the room appears with its GI converged.

use bevy::prelude::*;
use glam::DVec3;
use sf_coords::WorldPos;
use sf_gi_bevy::sf_gi::{fit_volume, GiSettings, VolumeBudget};
use sf_gi_bevy::{GiShown, GiVolume};
use sf_world::{Partition, WorldspaceId};
use sf_world_bevy::{ActiveWorldspace, FadeTransition, Streaming, StreamingSystems, WorldspaceRegistry, to_grid};

/// Adds the placement (call only when the GI plugin is in the app).
pub(crate) fn build(app: &mut App) {
    app.init_resource::<GiPlacement>()
        .add_systems(Update, place_gi_volume.after(StreamingSystems::Drive).before(StreamingSystems::Transition));
}

/// The worldspace the current decision is for, and its volume.
#[derive(Resource, Default)]
struct GiPlacement {
    worldspace: Option<WorldspaceId>,
    /// The worldspace was decided (a volume spawned, or none for a streamed one).
    decided: bool,
    volume: Option<Entity>,
}

/// The content bounds of a worldspace's index cells, in world meters: `None` while nothing
/// has bounds.
fn content_bounds(streaming: &Streaming, worldspace: &WorldspaceId) -> Option<(DVec3, DVec3)> {
    let directory = streaming.scheduler().directory(worldspace)?;
    let mut bounds: Option<(DVec3, DVec3)> = None;
    for cell in directory.cells.iter().filter(|c| !c.bounds.is_empty()) {
        let origin = directory.partition.cell_origin(cell.cell).to_f64();
        let (lo, hi) = (origin + cell.bounds.min.as_dvec3(), origin + cell.bounds.max.as_dvec3());
        bounds = Some(bounds.map_or((lo, hi), |(a, b)| (a.min(lo), b.max(hi))));
    }
    bounds
}

/// Follows the active worldspace: despawns a volume left in another one, fits and spawns the
/// interior's (once its directory is loaded), and holds the fade until it is first shown.
#[allow(clippy::too_many_arguments)]
fn place_gi_volume(
    mut commands: Commands,
    mut state: ResMut<GiPlacement>,
    active: Res<ActiveWorldspace>,
    streaming: Res<Streaming>,
    registry: Res<WorldspaceRegistry>,
    enabled: Option<Res<sf_sky_bevy::GiEnabled>>,
    mut fade: ResMut<FadeTransition>,
    shown: Query<Has<GiShown>, With<GiVolume>>,
) {
    if state.worldspace != active.0 {
        if let Some(old) = state.volume.take() {
            commands.entity(old).try_despawn();
        }
        state.worldspace.clone_from(&active.0);
        state.decided = false;
    }
    if let Some(volume) = state.volume {
        // Converge behind the black: the hold ends once the first full trace shows.
        if fade.is_holding() && enabled.is_some_and(|e| e.0) && shown.get(volume).is_ok_and(|shown| !shown) {
            fade.hold();
        }
    }
    if state.decided {
        return;
    }
    let Some(worldspace) = state.worldspace.clone() else { return };
    let Some(directory) = streaming.scheduler().directory(&worldspace) else { return };
    if directory.partition != Partition::Single {
        info!("gi: no volume (exterior)");
        state.decided = true;
        return;
    }
    let Some(root) = registry.get(&worldspace).map(|e| e.root) else { return };
    state.decided = true;
    let Some((lo, hi)) = content_bounds(&streaming, &worldspace) else {
        info!("gi: no volume (worldspace {worldspace} has no content bounds)");
        return;
    };
    // Fitted relative to the bounds' corner: exact in f32 whatever the worldspace's origin.
    #[allow(clippy::cast_possible_truncation)]
    let Some(fit) = fit_volume(Vec3::ZERO, (hi - lo).as_vec3(), VolumeBudget::default()) else { return };
    let (cell, transform) = to_grid(WorldPos::from_f64(lo + fit.centre.as_dvec3()));
    let settings = fit.settings(GiSettings::default());
    let extent = hi - lo;
    info!(
        "gi: volume {}x{}x{} probes at {} m (worldspace {worldspace}; bounds {:.1} x {:.1} x {:.1} m)",
        fit.dims.x, fit.dims.y, fit.dims.z, fit.spacing, extent.x, extent.y, extent.z
    );
    let volume = commands
        .spawn((Name::new("gi volume"), GiVolume { settings, ..GiVolume::default() }, cell, transform, ChildOf(root)))
        .id();
    state.volume = Some(volume);
}
