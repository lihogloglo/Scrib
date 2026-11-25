//! NPC sight in the game (`MA14`): `sf_ai_bevy`'s [`AiSightPlugin`] over the walkers' ground, every
//! placed actor an observer, the player's body a target.
//!
//! - Ground: the sight rays probe the same resource the walkers' mover steps on ([`add`] is called
//!   with it: `sf_physics_bevy::Physics`, or `MwGround` with `--off physics`), so statics, terrain
//!   and props block sight and characters never do (ORCA keeps them apart).
//! - Observers: `walkers::attach` gives each placed actor ([`sf_ai_bevy::AiActor`]) an
//!   [`AiObserver`] from [`observer`] next to its mover. The eye sits at [`EYE_RATIO`] of the
//!   standing height (the fitted capsule top, or the default capsule times the race height). Actors
//!   outside the processing range are `Dormant` and skip their checks, which also keeps observers in
//!   other worldspaces (whose coordinates overlap) from seeing the player.
//! - Player: the body (`PlayerBody` with its `ActionBody`) gets [`Perceivable`] as
//!   [`PerceptTarget::Player`] of [`HOST_KEY`] and a [`PerceivablePose`] the fixed tick copies from
//!   the action body after it moves ([`ActionSystems::Move`]) and before the sight checks, so the
//!   NPCs see this tick's position. Chest and head rays aim at [`CHEST_RATIO`] and [`HEAD_RATIO`] of
//!   its capsule height.
//!
//! Not covered: profiles and states (`AiPerceptionProfile`/`States`, no adapter yet), NPCs seeing
//! each other (targets are flagged actors only, ai.md "Stimuli"), sneak visibility (`AI8d`). Light exposure's
//! inputs: [`crate::light_inputs`] (`MA16`).

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use mw_game_core::player::{HOST_KEY, PlayerBody};
use sf_action_bevy::{ActionBody, ActionSystems};
use sf_ai::PerceptTarget;
use sf_ai_bevy::{AiObserver, AiPerceptionSystems, AiSightPlugin, Perceivable, PerceivablePose};
use sf_controller::GroundProbe;

/// Eye height over standing height (Drillis and Contini 1966 segment ratios, 0.936 H).
// tunables: const (anthropometry: a body proportion, not a trade-off; the body sets the height)
pub const EYE_RATIO: f32 = 0.936;
/// Chest (nipple line) height over standing height (Drillis and Contini 1966, 0.720 H).
// tunables: const (anthropometry: a body proportion, not a trade-off; the body sets the height)
pub const CHEST_RATIO: f32 = 0.720;
/// The head ray's aim over standing height: the target's eyes (can the observer see its face).
// tunables: const (anthropometry: the eye ratio again, not a trade-off)
pub const HEAD_RATIO: f32 = EYE_RATIO;

/// Adds the sight system on ground `G` (the walkers' mover ground) and the player target's
/// systems, the light inputs ([`crate::light_inputs`]) and the player's noise
/// ([`crate::noise_emitters`]). Idempotent for the plugin only.
pub fn add<G: GroundProbe + Resource>(app: &mut App) {
    if !app.is_plugin_added::<AiSightPlugin<G>>() {
        app.add_plugins(AiSightPlugin::<G>::default());
    }
    app.add_systems(
        FixedUpdate,
        (perceive_player, player_pose)
            .chain()
            .after(ActionSystems::Move)
            .before(AiPerceptionSystems::Sight),
    );
    // NPC light perception's inputs from game state (MA16).
    crate::light_inputs::add(app);
    // The player's footsteps and landings as noise NPCs hear (AI8c2).
    crate::noise_emitters::add(app);
}

/// The observer of an actor whose standing height is `height_m` (feet to crown, meters).
pub fn observer(height_m: f32) -> AiObserver {
    AiObserver {
        eye_m: EYE_RATIO * height_m,
    }
}

/// Makes a new player body a sight target (the body comes and goes with the free camera).
#[allow(clippy::type_complexity)]
fn perceive_player(
    mut commands: Commands,
    bodies: Query<(Entity, &ActionBody), (With<PlayerBody>, Without<Perceivable>)>,
) {
    for (entity, body) in &bodies {
        let height = body.params.height;
        commands.entity(entity).try_insert((
            Perceivable {
                target: PerceptTarget::Player(HOST_KEY),
                chest_m: CHEST_RATIO * height,
                head_m: HEAD_RATIO * height,
            },
            PerceivablePose {
                feet: body.loco.position,
                velocity: body.loco.motion.velocity,
            },
        ));
    }
}

/// The player target's pose from its action body, this tick.
fn player_pose(mut bodies: Query<(&ActionBody, &mut PerceivablePose), With<PlayerBody>>) {
    for (body, mut pose) in &mut bodies {
        pose.feet = body.loco.position;
        pose.velocity = body.loco.motion.velocity;
    }
}
