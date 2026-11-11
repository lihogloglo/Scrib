//! Setting row conditions (menus.md "Settings v2": `enabled_if`, `visible_if`).
//!
//! All conditions in a list must hold; an empty list always does. Conditions read
//! other settings' values, the platform traits the app reports (`dlss`, `rt`,
//! `multi_monitor`, `gamepad`; the `SettingTraits` resource on the Bevy side)
//! and the menu phase, so no setting needs code of its own to grey out or hide.

use serde::{Deserialize, Serialize};

use crate::settings::SettingValue;

/// The front end (main menu) phase.
pub const PHASE_FRONTEND: &str = "frontend";
/// In a loaded game (pause menu).
pub const PHASE_IN_GAME: &str = "in_game";

/// One condition. Written as a small table, the shape picks the variant:
/// `{ setting = "sf.upscaler", is = "dlss" }`, `{ trait = "dlss" }`,
/// `{ phase = "frontend" }`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Condition {
    /// Another setting's current value matches.
    Setting {
        setting: String,
        is: SettingValueMatch,
    },
    /// A named platform trait is present.
    Trait {
        r#trait: String,
    },
    /// The menu is open in this phase ([`PHASE_FRONTEND`], [`PHASE_IN_GAME`]).
    Phase { phase: String },
}

/// What a [`Condition::Setting`] accepts: one value (`is = "dlss"`) or any of
/// several (`is = ["dlss", "fsr"]`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SettingValueMatch {
    AnyOf(Vec<SettingValue>),
    Is(SettingValue),
}

impl SettingValueMatch {
    /// Whether `value` matches; an integer and a float compare as numbers.
    pub fn matches(&self, value: &SettingValue) -> bool {
        match self {
            Self::Is(want) => same(want, value),
            Self::AnyOf(wants) => wants.iter().any(|w| same(w, value)),
        }
    }
}

#[allow(clippy::cast_precision_loss, clippy::float_cmp)]
fn same(a: &SettingValue, b: &SettingValue) -> bool {
    match (a, b) {
        (SettingValue::Int(n), SettingValue::Float(x))
        | (SettingValue::Float(x), SettingValue::Int(n)) => *n as f64 == *x,
        _ => a == b,
    }
}

impl Condition {
    /// Whether this condition holds. A setting with no value fails.
    pub fn holds(
        &self,
        values: &impl Fn(&str) -> Option<SettingValue>,
        traits: &impl Fn(&str) -> bool,
        phase: &str,
    ) -> bool {
        match self {
            Self::Setting { setting, is } => values(setting).is_some_and(|v| is.matches(&v)),
            Self::Trait { r#trait } => traits(r#trait),
            Self::Phase { phase: want } => want == phase,
        }
    }
}

/// Whether every condition holds (an empty list always does).
pub fn evaluate(
    conds: &[Condition],
    values: &impl Fn(&str) -> Option<SettingValue>,
    traits: &impl Fn(&str) -> bool,
    phase: &str,
) -> bool {
    conds.iter().all(|c| c.holds(values, traits, phase))
}
