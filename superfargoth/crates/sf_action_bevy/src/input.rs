//! Local players' input: `bevy_enhanced_input` actions → [`InputSampler`] at frame rate → one
//! [`sf_action::Command`] per player and fixed tick. Bindings are the host's (keybind settings);
//! this module defines the actions and the context only.

use bevy_app::{App, Plugin, RunFixedMainLoop};
use bevy_ecs::prelude::*;
use bevy_ecs::relationship::RelationshipTarget;
use bevy_enhanced_input::prelude::*;
use bevy_math::Vec2;
use sf_action::{ActionClass, InputSampler, PlayerId, TapHold};

use crate::{ActionCharacter, ActionSystems, ActionTick};

/// The `bevy_enhanced_input` context of a local player's character (one per local player).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct PlayerControls;

/// A local player's character: its id on commands and the frame-rate half of sampling. The
/// camera (D1) calls `sampler.set_view` each frame.
#[derive(Component, Clone, Debug)]
pub struct PlayerInput {
    pub player: PlayerId,
    pub sampler: InputSampler,
    /// The sampler's shared Dodge/Sprint button (`TapHold::DODGE_SPRINT`: tap = roll on release, hold
    /// = sprint), fed by [`PlayerAction::DodgeSprint`].
    pub dodge_sprint: usize,
}

impl PlayerInput {
    pub fn new(player: PlayerId) -> Self {
        let mut sampler = InputSampler::new();
        let dodge_sprint = sampler.bind(TapHold::DODGE_SPRINT);
        Self { player, sampler, dodge_sprint }
    }
}

/// Tags each spawned action entity with what it feeds, so one query reads them all.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerAction {
    /// The move stick (WASD or the left stick), `Vec2`, length up to 1.
    Stick,
    /// A button class (never [`ActionClass::Move`]: that is the stick).
    Button(ActionClass),
    /// One button for Dodge and Sprint ([`PlayerInput::dodge_sprint`]): tap rolls on release, hold
    /// sprints. A host binds either this or the separate Dodge and Sprint buttons.
    DodgeSprint,
}

macro_rules! player_actions {
    ($($ty:ident => $class:ident;)*) => {
        $(
            #[derive(InputAction)]
            #[action_output(bool)]
            pub struct $ty;
        )*

        fn insert_button(entity: &mut EntityCommands<'_>, class: ActionClass) {
            match class {
                $(ActionClass::$class => { entity.insert(Action::<$ty>::new()); })*
                ActionClass::Move => {}
            }
        }
    };
}

player_actions! {
    PlayerLight => Light;
    PlayerHeavy => Heavy;
    PlayerOffhand => Offhand;
    PlayerDodge => Dodge;
    PlayerJump => Jump;
    PlayerUp => Up;
    PlayerDown => Down;
    PlayerGuard => Guard;
    PlayerInteract => Interact;
    PlayerItem => Item;
    PlayerSprint => Sprint;
    PlayerSneak => Sneak;
}

/// The shared Dodge/Sprint button.
#[derive(InputAction)]
#[action_output(bool)]
pub struct PlayerDodgeSprint;

/// The move stick action.
#[derive(InputAction)]
#[action_output(Vec2)]
pub struct PlayerMove;

/// Spawns the player's action entities on `context` (an entity with [`PlayerControls`]),
/// unbound. Returns them with their tags so the host can attach bindings.
pub fn spawn_player_actions(commands: &mut Commands, context: Entity) -> Vec<(PlayerAction, Entity)> {
    let mut out = Vec::with_capacity(ActionClass::ALL.len());
    let stick = commands.spawn((ActionOf::<PlayerControls>::new(context), PlayerAction::Stick, Action::<PlayerMove>::new())).id();
    out.push((PlayerAction::Stick, stick));
    for class in ActionClass::ALL {
        if class == ActionClass::Move {
            continue;
        }
        let mut entity = commands.spawn((ActionOf::<PlayerControls>::new(context), PlayerAction::Button(class)));
        insert_button(&mut entity, class);
        out.push((PlayerAction::Button(class), entity.id()));
    }
    let shared = commands.spawn((ActionOf::<PlayerControls>::new(context), PlayerAction::DodgeSprint, Action::<PlayerDodgeSprint>::new())).id();
    out.push((PlayerAction::DodgeSprint, shared));
    out
}

/// Registers the [`PlayerControls`] context and gathers its actions every frame. Needs
/// `EnhancedInputPlugin` and [`crate::SfActionPlugin`].
pub struct ActionInputPlugin;

impl Plugin for ActionInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_input_context::<PlayerControls>().add_systems(RunFixedMainLoop, gather.in_set(ActionSystems::Gather));
    }
}

/// Frame rate: action state into the sampler (edges are kept until the next tick samples them).
fn gather(mut players: Query<(&mut PlayerInput, &Actions<PlayerControls>)>, actions: Query<(&PlayerAction, &TriggerState, &ActionValue)>) {
    for (mut input, list) in &mut players {
        for entity in list.iter() {
            let Ok((tag, state, value)) = actions.get(entity) else { continue };
            match *tag {
                PlayerAction::Stick => input.sampler.set_stick(value.as_axis2d()),
                PlayerAction::Button(class) if *state == TriggerState::Fired => input.sampler.press(class),
                PlayerAction::Button(class) => input.sampler.release(class),
                PlayerAction::DodgeSprint if *state == TriggerState::Fired => {
                    let b = input.dodge_sprint;
                    input.sampler.press_shared(b);
                }
                PlayerAction::DodgeSprint => {
                    let b = input.dodge_sprint;
                    input.sampler.release_shared(b);
                }
            }
        }
    }
}

/// Fixed tick: advances the tick and samples every local player's command for it.
pub(crate) fn sample(mut tick: ResMut<ActionTick>, mut players: Query<(&mut PlayerInput, &mut ActionCharacter)>) {
    tick.0 = tick.0.next();
    for (mut input, mut character) in &mut players {
        let player = input.player;
        character.command = input.sampler.sample(tick.0, Some(player));
    }
}
