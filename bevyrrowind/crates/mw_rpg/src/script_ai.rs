//! The AI package functions on the game API (ai-adapter.md "Packages -> directives", slice
//! `MA2c1`): a run turns `mw_script`'s [`AiCall`] pushes into [`GameOp::Ai`] and answers the reads
//! (`GetAIPackageDone`, `GetCurrentAIPackage`) through the world's [`MwAi`].
//!
//! Ported from `OpenMW` (GPL-3.0) at commit `5f3aafb`, `apps/openmw/mwscript/aiextensions.cpp`
//! (argument reading: `mw_script::aifn`):
//! - Every push is a no-op on the player and on non-actors (`ptr == MWMechanics::getPlayer()`
//!   guards, :54, :85, :123, :164, :245, :352, :393): [`Target::Player`] and [`Target::Stack`]
//!   (an inventory item) push nothing. Whether another target is an actor is the host's to check.
//! - `AIEscortCell` with an empty cell id is a no-op (:167-168); a cell no record names (:170-171)
//!   is the host's to check, it owns the cell records.
//! - Pushes stack on top of the actor's AI (`AiSequence::stack`); the reads see what ran last
//!   (`getLastRunTypeId`, `isPackageDone`), not this run's pushes, so a run's reads never look at
//!   its own ops.
//!
//! Ours: the push is the call's shape ([`AiPush`]), not a `sf_ai::DirectiveSpec`. The spec needs
//! the world (the escorted actor's key, the actor's worldspace and position), which a run does
//! not own; [`AiPush::package`] gives the record struct that `mw_ai::packages::directive` maps,
//! so the host (`MA2c2`) builds the same spec an `AI_*` record would, with a script source.

use mw_script::AiCall;
use mw_script::aifn::{Point, WANDER_IDLES, package_type};
use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use sf_rpg::ConversationCtx;
use tes3::esp::{
    AiEscortPackage, AiFollowPackage, AiPackage, AiTravelPackage, AiWanderPackage, FixedString,
};

use crate::api::Target;

/// A package a script pushed, as the call gave it (Morrowind units in the destination cell, or
/// in the actor's own cell; durations in game hours).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AiPush {
    /// `AIWander`: `range` units, `hours` (0: forever), authored `hour`, the `Idle2`-`Idle9`
    /// chances given.
    Wander { range: i32, hours: i32, hour: i32, idles: Vec<u8>, repeat: bool },
    /// `AITravel`.
    Travel { to: Point, repeat: bool },
    /// `AIEscort` / `AIEscortCell`: whole hours, 0 or less = no limit.
    Escort { actor: RecordId, cell: Option<RecordId>, hours: i32, to: Point, repeat: bool },
    /// `AIFollow` / `AIFollowCell`: hours, 0 or less = no limit.
    Follow { actor: RecordId, cell: Option<RecordId>, hours: f32, to: Point, repeat: bool },
}

impl AiPush {
    /// The push a call makes; `None` for the reads.
    pub fn from_call(call: &AiCall) -> Option<AiPush> {
        let rid = |id: &mw_script::ast::Ident| RecordId::new(&id.key);
        Some(match call {
            AiCall::Wander { range, hours, hour, idles, repeat } => AiPush::Wander {
                range: *range,
                hours: *hours,
                hour: *hour,
                idles: idles.clone(),
                repeat: *repeat,
            },
            AiCall::Travel { to, repeat } => AiPush::Travel { to: *to, repeat: *repeat },
            AiCall::Escort { actor, cell, hours, to, repeat } => AiPush::Escort {
                actor: rid(actor),
                cell: cell.as_ref().map(rid),
                hours: *hours,
                to: *to,
                repeat: *repeat,
            },
            AiCall::Follow { actor, cell, hours, to, repeat } => AiPush::Follow {
                actor: rid(actor),
                cell: cell.as_ref().map(rid),
                hours: *hours,
                to: *to,
                repeat: *repeat,
            },
            AiCall::PackageDone | AiCall::CurrentPackage => return None,
        })
    }

    /// `GetCurrentAIPackage`'s id for this package once it runs ([`package_type`]).
    pub fn type_id(&self) -> i32 {
        match self {
            AiPush::Wander { .. } => package_type::WANDER,
            AiPush::Travel { .. } => package_type::TRAVEL,
            AiPush::Escort { .. } => package_type::ESCORT,
            AiPush::Follow { .. } => package_type::FOLLOW,
        }
    }

    /// The `AI_*` record struct with the same meaning, for `mw_ai::packages::directive`.
    /// Durations take the format's `int16_t` hours (saturated; follow's fraction is dropped),
    /// wander's range `uint16_t` and hour `uint8_t` (clamped), missing idle chances 0.
    pub fn package(&self) -> AiPackage {
        let cell = |c: &Option<RecordId>| c.as_ref().map_or_else(String::new, |c| c.as_str().to_owned());
        match self {
            AiPush::Wander { range, hours, hour, idles, repeat } => {
                let mut idle = [0u8; WANDER_IDLES];
                for (slot, chance) in idle.iter_mut().zip(idles) {
                    *slot = *chance;
                }
                AiPackage::Wander(AiWanderPackage {
                    distance: u16::try_from((*range).clamp(0, i32::from(u16::MAX))).unwrap_or(u16::MAX),
                    duration: esm_hours(*hours),
                    game_hour: u8::try_from((*hour).clamp(0, i32::from(u8::MAX))).unwrap_or(u8::MAX),
                    idle2: idle[0],
                    idle3: idle[1],
                    idle4: idle[2],
                    idle5: idle[3],
                    idle6: idle[4],
                    idle7: idle[5],
                    idle8: idle[6],
                    idle9: idle[7],
                    reset: i8::from(*repeat),
                })
            }
            AiPush::Travel { to, repeat } => {
                AiPackage::Travel(AiTravelPackage { location: *to, reset: u8::from(*repeat) })
            }
            AiPush::Escort { actor, cell: c, hours, to, repeat } => AiPackage::Escort(AiEscortPackage {
                location: *to,
                duration: esm_hours(*hours),
                target: FixedString(actor.as_str().to_owned()),
                reset: u8::from(*repeat),
                cell: cell(c),
            }),
            AiPush::Follow { actor, cell: c, hours, to, repeat } => AiPackage::Follow(AiFollowPackage {
                location: *to,
                duration: esm_hours(mw_script::interp::to_int(f64::from(*hours))),
                target: FixedString(actor.as_str().to_owned()),
                reset: u8::from(*repeat),
                cell: cell(c),
            }),
        }
    }
}

/// Hours as the record format stores them: an `int16_t` (saturated) in a `u16` field.
#[allow(clippy::cast_sign_loss)]
fn esm_hours(hours: i32) -> u16 {
    let h = i16::try_from(hours.clamp(i32::from(i16::MIN), i32::from(i16::MAX))).unwrap_or(0);
    h as u16
}

/// What the AI reads need from the world (`MA2c2` implements it over the actors' directive
/// stacks). Default ([`NoAi`]): no AI runs.
pub trait MwAi {
    /// `GetAIPackageDone`: whether `target`'s current package finished.
    fn package_done(&self, ctx: &ConversationCtx, target: &Target) -> bool;
    /// `GetCurrentAIPackage`: the [`package_type`] id of the package `target` ran last,
    /// [`package_type::NONE`] for none, a non-actor or a dead actor.
    fn current_package(&self, ctx: &ConversationCtx, target: &Target) -> i32;
}

/// No AI: nothing is done, no package runs.
pub struct NoAi;

impl MwAi for NoAi {
    fn package_done(&self, _: &ConversationCtx, _: &Target) -> bool {
        false
    }
    fn current_package(&self, _: &ConversationCtx, _: &Target) -> i32 {
        package_type::NONE
    }
}

/// Whether a push on `target` reaches an actor's AI (`OpenMW`'s player and non-actor guards).
pub(crate) fn pushes_to(target: &Target, push: &AiPush) -> bool {
    if matches!(target, Target::Player | Target::Stack { .. }) {
        return false;
    }
    !matches!(push, AiPush::Escort { cell: Some(c), .. } if c.as_str().is_empty())
}
