//! Bevy-free melee combat core (`docs/specs/combat.md`): moveset sheets compiled into
//! `sf_action` move rules and a combat move table, fighters and vitals, hit and hurt volumes with
//! sub-stepped weapon sweeps, the contact order, the [`CombatRules`] seam with the modern
//! [`StandardRules`], reaction tiers and hit-stop.
//!
//! Slice C1a. The feel layer (this crate and `sf_action`) owns timing and never computes damage;
//! rules decide numbers; content supplies sheets, clips and stats. Per fixed tick, a host runs
//! [`gate`] into `Sense::refused`, `sf_action::step`, its mover, then [`step`].
//!
//! Values (regen 45/s, x0.5 guarding, five sweep sub-steps, hit-stop curve) follow the spec, whose
//! research cites the MIT ELDEN-RING-Combat-Rewrite reference (`docs/research/combat/`); no code
//! is ported from it.

// Geometry reads best with its textbook names (a, b, p, q, s, t).
#![allow(clippy::many_single_char_names)]

mod fighter;
mod hitstop;
mod react;
mod rules;
mod shape;
mod sheet;
mod step;

pub use fighter::{Defense, FightState, Fighter, Offense, Pool, SwingState, Vitals};
pub use hitstop::{HitStop, ticks};
pub use react::{ReactionTable, Side, Tier};
pub use rules::{CombatRules, Contact, DefenderState, Outcome, PoolDelta, RegenState, StandardRules, Swing, SwingPlan, roll};
pub use shape::{HurtShape, SWEEP_STEPS, WeaponShape, closest_on_segment, closest_points, segment_distance};
pub use sheet::{
    At, CompiledSheet, Costs, Defaults, HitDef, HitMask, HitRow, HitStopDef, MaskDef, MoveDef, MoveSpec, MoveTable, Movesets, SheetDef, SheetError,
    SlotSpan, WeaponDef, WindowDef, compile, load,
};
pub use step::{Body, CombatCtx, CombatEvent, CombatOutput, HOSTILE_RANGE, SocketSource, Unit, gate, step};
