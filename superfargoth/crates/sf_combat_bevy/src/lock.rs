//! Lock-on shell (combat.md "Lock-on and camera", C1c part 1): the lock press, the candidates
//! and their line of sight, `sf_combat::update_lock` per tick, and the mirrors.
//!
//! - **Input.** [`LockOnAction`] on a local player's `PlayerControls` context (the host binds it;
//!   [`spawn_lock_action`]): a press requests `LockRequest::Toggle` from the player's sampler, at
//!   frame rate in `ActionSystems::Gather`.
//! - **Tick.** [`CombatSystems::Lock`] (after `ActionSystems::Command`, before the step): every
//!   [`LockOwner`] gets its candidates, the other [`Lockable`] action characters that aren't dead
//!   (stable id `TargetId(CharacterId)`, aim point `Lockable::aim_height` above the feet), with
//!   line of sight from its eye by a thin sphere cast through the ground probe `G` (statics and
//!   terrain; characters are not in it). `ActionState::lock` is the truth; [`LockAim`] carries the
//!   target to `Sense::lock_target` (Clip `Turn` windows aim at it), `LockOn` on the owner mirrors
//!   it for the camera's framing (third person only), and a press with nothing to lock recentres
//!   the camera that follows the owner.
//! - Not yet: locked locomotion facing and strafe (C1c-2), locked rolls (C1c-3), switching by a
//!   stick flick (C4; `LockRequest::Next` already switches).

use std::marker::PhantomData;

use bevy_app::{App, FixedUpdate, Plugin, RunFixedMainLoop};
use bevy_ecs::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_math::Vec3;
use sf_action::{LockRequest, TICK_HZ, TargetId};
use sf_action_bevy::{ActionBody, ActionCharacter, ActionSystems, LockAim, PlayerControls, PlayerInput};
pub use sf_camera::{BreakReason, LockParams, LockTimers};
use sf_camera::Candidate;
pub use sf_camera_bevy::{LockOn, Lockable};
use sf_camera_bevy::PlayerCamera;
use sf_combat::{LockEvent, LockInput, update_lock};
use sf_controller::{GroundProbe, ProbeQuery};
use sf_coords::WorldPos;

use crate::{CombatSystems, Combatant};

/// A character that owns a lock-on (local players; AI later). Its target is `ActionState::lock`.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
#[require(LockAim)]
pub struct LockOwner {
    pub timers: LockTimers,
    /// The last event that changed the lock (HUD, sounds, tests).
    pub last: Option<LockEvent>,
}

/// Lock-on tuning.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct LockConfig {
    pub params: LockParams,
    /// The owner's eye above its feet: range, angle and line of sight are measured from it, m.
    pub eye_height: f32,
    /// Radius of the line-of-sight cast (a thin sphere: a fence post doesn't break the lock), m.
    pub los_radius: f32,
}

impl Default for LockConfig {
    fn default() -> Self {
        Self { params: LockParams::default(), eye_height: 1.5, los_radius: 0.1 }
    }
}

/// Lock on / release (a button on the player's `PlayerControls` context).
#[derive(InputAction)]
#[action_output(bool)]
pub struct LockOnAction;

/// Spawns the unbound [`LockOnAction`] on `context` (an entity with `PlayerControls`); the host
/// binds it, as `sf_action_bevy::spawn_player_actions`' actions.
pub fn spawn_lock_action(commands: &mut Commands, context: Entity) -> Entity {
    commands.spawn((ActionOf::<PlayerControls>::new(context), Action::<LockOnAction>::new())).id()
}

/// The lock-on with the ground probe `G` for line of sight (the one `sf_action_bevy` uses). Needs
/// `SfActionPlugin`, `ActionInputPlugin` and a combat plugin (it orders against
/// [`CombatSystems`]).
pub struct SfLockOnPlugin<G> {
    _ground: PhantomData<fn() -> G>,
}

impl<G> Default for SfLockOnPlugin<G> {
    fn default() -> Self {
        Self { _ground: PhantomData }
    }
}

impl<G: GroundProbe + Resource> Plugin for SfLockOnPlugin<G> {
    fn build(&self, app: &mut App) {
        app.init_resource::<LockConfig>()
            .configure_sets(FixedUpdate, CombatSystems::Lock.after(ActionSystems::Command).before(ActionSystems::Step))
            .add_systems(RunFixedMainLoop, gather.in_set(ActionSystems::Gather))
            .add_systems(FixedUpdate, update_locks::<G>.in_set(CombatSystems::Lock));
    }
}

/// Frame rate: a lock press into its player's sampler (kept until the next tick samples it).
fn gather(presses: Query<(&ActionOf<PlayerControls>, &ActionEvents), With<Action<LockOnAction>>>, mut players: Query<&mut PlayerInput>) {
    for (of, events) in &presses {
        if events.contains(ActionEvents::START)
            && let Ok(mut input) = players.get_mut(**of)
        {
            input.sampler.request_lock(LockRequest::Toggle);
        }
    }
}

/// Unobstructed from `eye` to `aim` (no probe: always).
fn line_of_sight(probe: Option<&dyn GroundProbe>, eye: WorldPos, aim: WorldPos, radius: f32) -> bool {
    let Some(probe) = probe else { return true };
    let d = eye.delta_f32(aim);
    let len = d.length();
    len < 1e-3 || probe.probe_one(ProbeQuery::sphere(eye, d / len, (len - radius).max(0.0), radius)).is_none()
}

type LockQuery = (
    Entity,
    &'static mut ActionCharacter,
    &'static ActionBody,
    Option<&'static Lockable>,
    Option<&'static Combatant>,
    Option<&'static mut LockOwner>,
    Option<&'static mut LockAim>,
    Option<&'static LockOn>,
);

/// One tick of every owner's lock (module docs).
#[allow(clippy::needless_pass_by_value)] // system parameters
fn update_locks<G: GroundProbe + Resource>(
    mut commands: Commands,
    config: Res<LockConfig>,
    ground: Option<Res<G>>,
    mut cameras: Query<&mut PlayerCamera>,
    mut characters: Query<LockQuery>,
) {
    #[allow(clippy::cast_precision_loss)]
    let dt = 1.0 / TICK_HZ as f32;
    let probe = ground.as_deref().map(|g| g as &dyn GroundProbe);
    let targets: Vec<(Entity, TargetId, WorldPos)> = characters
        .iter()
        .filter_map(|(e, c, body, lockable, combatant, ..)| {
            let lockable = lockable?;
            (!combatant.is_some_and(|k| k.fighter.dead()))
                .then(|| (e, TargetId(c.id.0), body.loco.position.offset(Vec3::Y * lockable.aim_height)))
        })
        .collect();
    let mut candidates = Vec::with_capacity(targets.len());
    for (e, mut c, body, _, _, owner, aim, lock_on) in &mut characters {
        let Some(mut owner) = owner else { continue };
        let c = &mut *c;
        let request = c.command.lock_request;
        // Idle (no lock, no press, no re-offer pending): nothing to sense.
        if c.state.lock.is_some() || request != LockRequest::None || owner.timers.offered.is_some() {
            let eye = body.loco.position.offset(Vec3::Y * config.eye_height);
            // Held without a press: only the target's own line of sight matters.
            let all = c.state.lock.is_none() || request != LockRequest::None;
            candidates.clear();
            candidates.extend(targets.iter().filter(|t| t.0 != e).map(|&(_, id, pos)| Candidate {
                id,
                pos,
                los: (all || c.state.lock == Some(id)) && line_of_sight(probe, eye, pos, config.los_radius),
            }));
            let input = LockInput {
                request,
                origin: eye,
                view_yaw: c.command.view.yaw,
                candidates: &candidates,
                traversing: c.state.warp.is_some() || body.loco.traversing,
                grounded: body.loco.grounded.is_some(),
            };
            let event = update_lock(&mut c.state.lock, &mut owner.timers, &input, &config.params, dt);
            if event != LockEvent::Kept {
                owner.last = Some(event);
            }
            if event == LockEvent::Recentre {
                cameras.iter_mut().filter(|cam| cam.follow == e).for_each(|mut cam| cam.recentre());
            }
        }
        let target = c.state.lock.and_then(|id| targets.iter().find(|t| t.1 == id));
        if let Some(mut aim) = aim {
            let to = target.map(|t| body.loco.position.delta_f32(t.2));
            if aim.0 != to {
                aim.0 = to;
            }
        }
        let framed = target.map(|t| t.0);
        if lock_on.map(|l| l.0) != framed {
            match framed {
                Some(t) => commands.entity(e).insert(LockOn(t)),
                None => commands.entity(e).remove::<LockOn>(),
            };
        }
    }
}
