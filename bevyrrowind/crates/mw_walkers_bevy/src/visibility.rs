//! The player's look to NPC sight (`MA18`; crime.md "Stealth and detection"): each fixed tick the
//! player body's [`AiVisibilityFactors`] from its sneak state and sheet, after the action step moves
//! it ([`ActionSystems::Move`]) and before the sight checks ([`AiPerceptionSystems::Sight`]) read it.
//!
//! - **Stance.** The action state's mode ([`ActionCharacter`]'s `state.mode.mode`, the chooser's
//!   sneak state; Ctrl held, `ActionClass::Sneak`), not raw input: [`Mode::Sneak`] is
//!   [`Stance::Crouched`], anything else (free-run overrides sneak) [`Stance::Upright`]. The crime
//!   rules ([`sf_ai_bevy::AiDetectionRules`], default `StandardDetection`) turn it into concealment.
//! - **Modifiers.** A [`StatId`] is the game's stat index (`StatIdx`, `sf_ai::StatId` docs). The
//!   host player's sheet ([`PlayerCharacter`] of [`HOST_KEY`] with its [`Stats`]) gives the modified
//!   value of [`SNEAK_SKILL`] (pushed always: the rules read it only while crouched, `OpenMW`'s
//!   `awarenessCheck` sneak term) then [`CHAMELEON`] (only when the table has that stat; magnitude
//!   0..100). `StandardDetection` ignores both; `MA5`'s `awarenessCheck` port reads them. The keys
//!   are resolved once per table ([`StatTableRes`] change). No table or sheet: no modifiers.
//! - **Writes.** Inserted with the first tick's value on a body without it; afterwards set only when
//!   it changes (`set_if_neq`), so change detection sees mode and skill changes only.
//!
//! Not covered: size (race height), disguise, invisibility (a sight gate, not a factor).

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use mw_game_core::player::{HOST_KEY, PlayerBody, PlayerCharacter};
use sf_action::Mode;
use sf_action_bevy::{ActionCharacter, ActionSystems};
use sf_ai::{Stance, StatId, VisibilityFactors};
use sf_ai_bevy::{AiPerceptionSystems, AiVisibilityFactors};
use sf_stats_bevy::sf_stats::{StatIdx, StatTable};
use sf_stats_bevy::{StatTableRes, Stats};

/// The stat table key of the Sneak skill (`mw_stats::SKILLS`).
pub const SNEAK_SKILL: &str = "sneak";
/// The stat table key of Chameleon's magnitude (the MGEF effect's name, as the effects adapter keys
/// magnitude stats; not in `mw_stats::MAGNITUDE_STATS` yet, so absent until it is).
pub const CHAMELEON: &str = "chameleon";

/// Adds the player's visibility system (called by [`crate::sight::add`]).
pub fn add(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        player_visibility.after(ActionSystems::Move).before(AiPerceptionSystems::Sight),
    );
}

/// The stance of an action mode (module docs).
#[must_use]
pub fn stance(mode: Mode) -> Stance {
    match mode {
        Mode::Sneak => Stance::Crouched,
        Mode::Normal | Mode::FreeRun => Stance::Upright,
    }
}

/// The visibility stats of one table, resolved by key.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VisibilityStats {
    pub sneak: Option<StatIdx>,
    pub chameleon: Option<StatIdx>,
}

impl VisibilityStats {
    #[must_use]
    pub fn resolve(table: &StatTable) -> Self {
        Self { sneak: table.find(SNEAK_SKILL), chameleon: table.find(CHAMELEON) }
    }
}

/// The player's factors for `mode`, with the modified values of `stats` from `sheet` (module docs).
#[must_use]
pub fn factors(mode: Mode, stats: VisibilityStats, sheet: Option<(&StatTable, &Stats)>) -> AiVisibilityFactors {
    let mut f = VisibilityFactors { stance: stance(mode), ..VisibilityFactors::default() };
    if let Some((table, sheet)) = sheet {
        let read = sheet.view(table);
        for idx in [stats.sneak, stats.chameleon].into_iter().flatten() {
            f.push_modifier(StatId(idx.0), read.modified(idx));
        }
    }
    AiVisibilityFactors::from(&f)
}

/// The player body's factors this tick (module docs).
#[allow(clippy::needless_pass_by_value)] // system parameters
fn player_visibility(
    mut commands: Commands,
    table: Option<Res<StatTableRes>>,
    mut resolved: Local<VisibilityStats>,
    sheets: Query<(&PlayerCharacter, &Stats)>,
    mut bodies: Query<(Entity, &ActionCharacter, Option<&mut AiVisibilityFactors>), With<PlayerBody>>,
) {
    match &table {
        Some(t) if t.is_changed() => *resolved = VisibilityStats::resolve(&t.0),
        Some(_) => {}
        None => *resolved = VisibilityStats::default(),
    }
    let sheet = sheets.iter().find(|(p, _)| p.key == HOST_KEY).map(|(_, s)| s);
    let sheet = table.as_deref().zip(sheet).map(|(t, s)| (&*t.0, s));
    for (entity, character, current) in &mut bodies {
        let want = factors(character.state.mode.mode, *resolved, sheet);
        match current {
            Some(mut f) => {
                f.set_if_neq(want);
            }
            None => {
                commands.entity(entity).try_insert(want);
            }
        }
    }
}

#[cfg(test)]
#[path = "visibility_tests.rs"]
mod tests;
