//! Swimming (FL-5c2): each fixed tick, every awake [`Mover`] asks the CPU water query how deep its
//! capsule is (`sf_water::swim_medium`, FL-5c1), switches between `Ground` and `Swim { surface }`
//! and is carried by the water's current (`Locomotion::current`, FL-5b). Flying movers (creature
//! scaffolds, noclip) are left alone.
//!
//! The rule's five `swim.*` tunables are bound live into [`SwimSettings`]. Morrowind's
//! `fSwimHeightScale` game setting sets `swim.enter_fraction` on the `Mod` layer (the game data's
//! layer, below presets, the user and the command line): `OpenMW` has an actor swim once the water
//! reaches that share of its height, the same setting the nav bake's swim level uses
//! (`mw_bake/src/nav.rs`).

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use glam::Vec3;
use mw_world::ids;
use sf_controller_bevy::{ControllerSystems, Dormant, Mover, MoverMode, MoverParams};
use sf_coords::WorldPos;
use sf_tunables_bevy::{SetBy, Tunables, TunablesAppExt};
use sf_water::swim::{CURRENT_GAIN, CURRENT_MAX_MPS, CURRENT_WADE_GAIN, ENTER_FRACTION, EXIT_FRACTION};
/// The swim rule's tunables (`swim.*`), for the app's tunables catalog.
pub use sf_water::swim::TUNABLES;
use sf_water::{Capsule, SwimMedium, SwimParams, WaterQuery, swim_medium};
use sf_world::WorldspaceId;
use sf_world_bevy::ActiveWorldspace;

use crate::water::Water;

#[cfg(test)]
#[path = "swim_tests.rs"]
mod tests;

/// The swim rule's settings, bound live from the `swim.*` tunables.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct SwimSettings(pub SwimParams);

/// Registers and binds the `swim.*` tunables, applies `fSwimHeightScale` (when the records have it)
/// to `swim.enter_fraction`, and adds [`swim`] to `ControllerSystems::Drive` after the water's step.
pub(crate) fn add(app: &mut App, swim_height_scale: Option<f32>) {
    type S = SwimSettings;
    app.init_resource::<S>();
    let _enter = app.add_tunable(&ENTER_FRACTION).bind_resource(|s: &mut S, v| s.0.enter_fraction = v);
    let _exit = app.add_tunable(&EXIT_FRACTION).bind_resource(|s: &mut S, v| s.0.exit_fraction = v);
    let _gain = app.add_tunable(&CURRENT_GAIN).bind_resource(|s: &mut S, v| s.0.current_gain = v);
    let _wade = app.add_tunable(&CURRENT_WADE_GAIN).bind_resource(|s: &mut S, v| s.0.wade_gain = v);
    let _max = app.add_tunable(&CURRENT_MAX_MPS).bind_resource(|s: &mut S, v| s.0.max_mps = v);
    if let Some(scale) = swim_height_scale {
        let mut tunables = app.world_mut().resource_mut::<Tunables>();
        crate::shadow_tunables::set_layer(&mut tunables, &ENTER_FRACTION, scale.clamp(0.0, 1.0), SetBy::Mod, "fSwimHeightScale");
    }
    app.add_systems(FixedUpdate, swim.in_set(ControllerSystems::Drive).after(crate::water::step));
}

/// The mover's capsule: end-sphere centres `radius` above the feet and `radius` below the top
/// (both at `radius` above the feet when the capsule is shorter than its diameter).
pub(crate) fn capsule(feet: WorldPos, params: &MoverParams) -> Capsule {
    let radius = params.radius;
    let a = feet.offset(Vec3::Y * radius);
    let b = if params.height < 2.0 * radius { a } else { feet.offset(Vec3::Y * (params.height - radius)) };
    Capsule { a, b, radius }
}

/// The mode after `medium`: `Swim` while it swims, `Ground` once a swimmer stops (any other mode
/// stays as it was).
pub(crate) fn next_mode(mode: MoverMode, medium: &SwimMedium) -> MoverMode {
    match (medium.swim, mode) {
        (true, _) => MoverMode::Swim { surface: medium.surface },
        (false, MoverMode::Swim { .. }) => MoverMode::Ground,
        (false, other) => other,
    }
}

/// Swim state and current of every awake, non-flying mover from the water of the active worldspace.
fn swim(
    water: Res<Water>,
    active: Res<ActiveWorldspace>,
    settings: Res<SwimSettings>,
    mut exterior: Local<Option<WorldspaceId>>,
    mut movers: Query<&mut Mover, Without<Dormant>>,
) {
    let ws = match &active.0 {
        Some(ws) => ws,
        None => exterior.get_or_insert_with(ids::exterior),
    };
    for mut mover in &mut movers {
        let mode = mover.loco.mode;
        if matches!(mode, MoverMode::Fly { .. }) {
            continue;
        }
        let immersion = water.world.immersion(ws, capsule(mover.loco.position, &mover.params));
        let medium = swim_medium(&immersion, matches!(mode, MoverMode::Swim { .. }), &settings.0);
        let next = next_mode(mode, &medium);
        if next != mode {
            mover.loco.mode = next;
        }
        if mover.loco.current != medium.current {
            mover.loco.current = medium.current;
        }
    }
}
