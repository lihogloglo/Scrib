//! A scene's live A/B (CONTRIBUTING "A look comparison is one scene with a live toggle", owner
//! 2026-11-26): a scene of `scenes.ron` names one tunable and two values (`ab:`,
//! [`mw_game_core::scenes::SceneAb`]). The run starts on arm A, [`KEY`] switches arms at the
//! console layer, and one line at the bottom of the window names the live arm. Each switch logs
//! one `[A/B]` line.
//!
//! The first switch to an arm that changes a shader def compiles its pipelines, and Bevy skips a
//! draw whose pipeline is compiling: the line says `compiling shaders` for those frames, so a
//! blink there is not read as the effect under test.

use bevy_app::prelude::*;
use bevy_color::prelude::*;
use bevy_ecs::prelude::*;
use bevy_input::prelude::*;
use bevy_text::prelude::*;
use bevy_ui::prelude::*;
use mw_game_core::scenes::SceneAb;
use sf_tunables_bevy::sf_tunables::Lifetime;
use sf_tunables_bevy::{SetBy, TunableId, Tunables, Value};

/// Switches arms.
pub const KEY: KeyCode = KeyCode::Tab;

/// One of the two arms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    A,
    B,
}

impl Arm {
    const fn other(self) -> Self {
        match self {
            Self::A => Self::B,
            Self::B => Self::A,
        }
    }

    const fn letter(self) -> char {
        match self {
            Self::A => 'A',
            Self::B => 'B',
        }
    }
}

/// A scene's A/B checked against a tunable store: the tunable and both parsed values.
#[derive(Clone, Debug, PartialEq)]
pub struct Arms {
    pub id: TunableId,
    pub values: [Value; 2],
}

impl Arms {
    /// The arm whose value is in effect; `None` when something else set a third value.
    #[must_use]
    pub fn live(&self, tunables: &sf_tunables_bevy::sf_tunables::Tunables) -> Option<Arm> {
        let now = tunables.get(self.id);
        if *now == self.values[0] {
            Some(Arm::A)
        } else if *now == self.values[1] {
            Some(Arm::B)
        } else {
            None
        }
    }

    const fn value(&self, arm: Arm) -> &Value {
        match arm {
            Arm::A => &self.values[0],
            Arm::B => &self.values[1],
        }
    }
}

/// Checks a scene's `ab` against `tunables`: the tunable exists and applies while running, both
/// values parse for its kind, and they differ.
///
/// # Errors
/// What is wrong, as one line for the scene's author.
pub fn resolve(tunables: &sf_tunables_bevy::sf_tunables::Tunables, ab: &SceneAb) -> Result<Arms, String> {
    let name = &ab.set;
    let id = tunables.id(name).ok_or_else(|| format!("ab: no such tunable {name:?}"))?;
    let var = tunables.var(id);
    let lifetime = var.meta().lifetime;
    if matches!(lifetime, Lifetime::Respawn | Lifetime::Restart | Lifetime::Bake) {
        return Err(format!("ab: {name} does not change what is on screen while running ({lifetime:?})"));
    }
    let parse = |arm: Arm, text: &str| var.kind().parse(text).map_err(|e| format!("ab: arm {} of {name}: {e}", arm.letter()));
    let values = [parse(Arm::A, &ab.a.value)?, parse(Arm::B, &ab.b.value)?];
    if values[0] == values[1] {
        return Err(format!("ab: both arms set {name} = {}", values[0]));
    }
    Ok(Arms { id, values })
}

/// The on-screen line: the live arm and its label, then the key.
#[must_use]
pub fn line(ab: &SceneAb, arms: &Arms, tunables: &sf_tunables_bevy::sf_tunables::Tunables, compiling: bool) -> String {
    let mut out = match arms.live(tunables) {
        Some(Arm::A) => format!("A: {}", ab.a.label),
        Some(Arm::B) => format!("B: {}", ab.b.label),
        None => format!("neither arm: {} = {}", ab.set, tunables.get(arms.id)),
    };
    if compiling {
        out.push_str("  (compiling shaders)");
    }
    out.push_str("   |   Tab: switch A/B");
    out
}

/// The run's A/B: the scene's definition and how it resolved against the live tunables.
#[derive(Resource)]
pub struct SceneAbState {
    def: SceneAb,
    arms: Result<Arms, String>,
}

/// The text of the arm line.
#[derive(Component)]
struct ArmText;

/// Adds the key and the arm line for a scene with an `ab` ([`crate::live_ab::LiveAbPlugin`] adds it).
pub struct SceneAbPlugin(pub SceneAb);

impl Plugin for SceneAbPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SceneAbState { def: self.0.clone(), arms: Err("not started".to_owned()) })
            .add_systems(Startup, start)
            .add_systems(Update, (key, show).chain());
    }
}

/// Sets `arm` at the console layer and logs it. The log line is the check's proof line.
fn switch_to(tunables: &mut Tunables, ab: &SceneAb, arms: &Arms, arm: Arm) {
    let value = arms.value(arm).clone();
    let label = match arm {
        Arm::A => &ab.a.label,
        Arm::B => &ab.b.label,
    };
    match tunables.set_by(arms.id, value.clone(), SetBy::Console) {
        Ok(_) => println!("[A/B] arm {} ({label}): {} = {value}", arm.letter(), ab.set),
        Err(why) => println!("[A/B] arm {} refused: {} = {value}: {why}", arm.letter(), ab.set),
    }
}

/// Resolves the scene's `ab` against the registered tunables, starts on arm A and spawns the line.
fn start(mut commands: Commands, mut state: ResMut<SceneAbState>, tunables: Option<ResMut<Tunables>>) {
    let state = &mut *state;
    state.arms = match tunables {
        Some(mut tunables) => resolve(&tunables, &state.def).inspect(|arms| switch_to(&mut tunables, &state.def, arms, Arm::A)),
        None => Err("this run has no tunables".to_owned()),
    };
    if let Err(why) = &state.arms {
        println!("[A/B] not available: {why}");
    }
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(24.0),
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            justify_content: JustifyContent::Center,
            ..Node::default()
        })
        .with_children(|row| {
            row.spawn((
                ArmText,
                Text::new(""),
                TextFont::from_font_size(22.0),
                TextColor(Color::srgb(1.0, 1.0, 0.9)),
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                Node { padding: UiRect::axes(Val::Px(14.0), Val::Px(6.0)), ..Node::default() },
            ));
        });
}

/// [`KEY`] switches to the other arm (to A from a value that is neither). Not while the console
/// is open: Tab completes a name there.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn key(
    input: Res<ButtonInput<KeyCode>>,
    state: Res<SceneAbState>,
    tunables: Option<ResMut<Tunables>>,
    console: Option<Res<sf_console_bevy::ConsoleOpen>>,
) {
    if !input.just_pressed(KEY) || console.is_some_and(|c| c.0) {
        return;
    }
    let (Ok(arms), Some(mut tunables)) = (&state.arms, tunables) else { return };
    let next = arms.live(&tunables).map_or(Arm::A, Arm::other);
    switch_to(&mut tunables, &state.def, arms, next);
}

/// Rewrites the arm line when what it says changed.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn show(
    state: Res<SceneAbState>,
    tunables: Option<Res<Tunables>>,
    pending: Option<Res<crate::pipeline_watch::PendingDraws>>,
    mut text: Query<&mut Text, With<ArmText>>,
) {
    let Ok(mut text) = text.single_mut() else { return };
    let out = match (&state.arms, tunables) {
        (Ok(arms), Some(tunables)) => line(&state.def, arms, &tunables, pending.is_some_and(|p| p.gpu().pipelines > 0)),
        (Err(why), _) => format!("A/B not available ({why})"),
        (Ok(_), None) => return,
    };
    if text.0 != out {
        text.0 = out;
    }
}

#[cfg(test)]
#[path = "scene_ab_tests.rs"]
mod tests;
