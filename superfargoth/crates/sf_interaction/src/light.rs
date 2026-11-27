//! Light as state (interaction-world-rules.md "Light as state (IN19)"): a light-bearing object's
//! [`LIT`] tag, the snuff / relight verbs that toggle it, rule toggles (water snuffs a candle, fire
//! lights a torch) and the [`LightChanged`] event (net rule 8). `Lit` is a saved state without TTL:
//! [`LightState::initial`] from the def's `lit_default` on first load, then the one-byte rest state
//! ([`LightState::rest_byte`], [`LightState::from_rest_byte`]) stored as the [`LIGHT_STATE_OWNER`]
//! blob of its placement. The core spawns no light: the host's light hook follows `LightChanged`, and
//! the shell keeps the AI's `GameplayLights` from it (`IN19b`). Industry: Thief, Dishonored and Skyrim
//! keep a light's on/off as gameplay state that AI and stealth read, never the renderer's lights.

use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_world::PlacementId;

use crate::claims::{Tick, UserKey};
use crate::def::TagId;

/// State tag a burning light carries (snuff requires it, relight refuses it).
pub const LIT: TagId = TagId::of("lit");

/// The verb that puts a lit light out.
pub const VERB_SNUFF: TagId = TagId::of("snuff");

/// The verb that lights an unlit light.
pub const VERB_RELIGHT: TagId = TagId::of("relight");

/// The owner name of a light's rest state among a placement's state blobs.
pub const LIGHT_STATE_OWNER: &str = "light";

/// Rest byte of an unlit light; every byte other than this and [`LIGHT_REST_LIT`] is rejected
/// (kept free for a later layout).
// tunables: const (save format byte)
pub const LIGHT_REST_UNLIT: u8 = 0;

/// Rest byte of a lit light.
// tunables: const (save format byte)
pub const LIGHT_REST_LIT: u8 = 1;

/// A light-bearing object (the `Light` slot kind's data, from the adapter).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LightDef {
    /// Reach: the gameplay contribution falls to 0 here, meters.
    pub radius_m: f32,
    /// Contribution at the centre, 0..1 typical (`sf_ai::LightSource::intensity`).
    pub intensity: f32,
    /// Lit when first loaded (no saved state yet).
    pub lit_default: bool,
}

/// One light's state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightState {
    pub lit: bool,
}

/// Why a light use was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LightRefusal {
    /// Snuff on a light that is already out.
    AlreadyUnlit,
    /// Relight on a light that already burns.
    AlreadyLit,
    /// The verb is neither [`VERB_SNUFF`] nor [`VERB_RELIGHT`].
    NotALightVerb,
}

/// One change of one light, for the host's light hook, `GameplayLights`, AI (`Darkened`, dusk lamps)
/// and the replay log.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightChanged {
    pub tick: Tick,
    pub placement: PlacementId,
    /// The new state.
    pub lit: bool,
    /// Where the light is, world frame.
    pub at: Vec3,
    /// Who changed it; `None` = a rule (water, fire) or a script.
    pub culprit: Option<UserKey>,
}

impl LightState {
    /// The state of a light loaded for the first time.
    #[must_use]
    pub fn initial(def: &LightDef) -> Self {
        Self {
            lit: def.lit_default,
        }
    }

    /// The state tag this light adds to its placement's state tags.
    #[must_use]
    pub fn tag(&self) -> Option<TagId> {
        self.lit.then_some(LIT)
    }

    /// A user's snuff or relight: snuff needs a lit light, relight an unlit one; anything else is
    /// refused and changes nothing.
    ///
    /// # Errors
    /// [`LightRefusal`]: the light is already in the verb's end state, or `verb` is no light verb.
    pub fn use_verb(
        &mut self,
        verb: TagId,
        placement: PlacementId,
        at: Vec3,
        culprit: Option<UserKey>,
        tick: Tick,
    ) -> Result<LightChanged, LightRefusal> {
        let lit = match verb {
            v if v == VERB_SNUFF => {
                if !self.lit {
                    return Err(LightRefusal::AlreadyUnlit);
                }
                false
            }
            v if v == VERB_RELIGHT => {
                if self.lit {
                    return Err(LightRefusal::AlreadyLit);
                }
                true
            }
            _ => return Err(LightRefusal::NotALightVerb),
        };
        self.lit = lit;
        Ok(LightChanged { tick, placement, lit, at, culprit })
    }

    /// A rule or script sets the state: an event only when it changes.
    pub fn set(
        &mut self,
        lit: bool,
        placement: PlacementId,
        at: Vec3,
        culprit: Option<UserKey>,
        tick: Tick,
    ) -> Option<LightChanged> {
        if self.lit == lit {
            return None;
        }
        self.lit = lit;
        Some(LightChanged { tick, placement, lit, at, culprit })
    }

    /// The state at rest: [`LIGHT_REST_LIT`] or [`LIGHT_REST_UNLIT`].
    #[must_use]
    pub fn rest_byte(&self) -> u8 {
        if self.lit {
            LIGHT_REST_LIT
        } else {
            LIGHT_REST_UNLIT
        }
    }

    /// The state [`Self::rest_byte`] wrote; `None` for any other length or byte.
    #[must_use]
    pub fn from_rest_byte(bytes: &[u8]) -> Option<Self> {
        match bytes {
            [LIGHT_REST_UNLIT] => Some(Self { lit: false }),
            [LIGHT_REST_LIT] => Some(Self { lit: true }),
            _ => None,
        }
    }
}
