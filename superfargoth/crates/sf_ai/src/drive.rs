//! Drives: utility arbitration between content and reactions (ai.md "Drives (utility
//! arbitration)", slice AI7).
//!
//! **Canonical technique:** Dave Mark's Infinite Axis Utility System (IAUS; GDC 2013 "Architecture
//! Tricks: Managing Behaviors in Time, Space, and Depth" with Kevin Dill, GDC 2015 "Building a
//! Better Centaur: AI at Massive Scale" for Guild Wars 2; Game AI Pro 3 ch. 13, Mike Lewis,
//! "Choosing Effective Utility-Based Considerations"). Each consideration maps one input through
//! two bookends to a 0..=1 axis and a response curve; a drive's score is the product of its
//! considerations, each first raised by the **compensation factor**, `v + (1 - v)(1 - 1/n) v` for
//! `n` considerations, so a drive with many considerations isn't punished for having them;
//! times the drive's weight. Hysteresis is the industry's two tools: a **momentum** bonus on the
//! running drive (IAUS "commitment"/inertia) and a **minimum commit time**. The `Directive` drive
//! may carry a **floor**: content wins unless a reaction clearly beats it (ai.md "Drives").
//!
//! - Records (mod-patchable, `sf_records` serde kinds): [`CONSIDERATION_KIND`]
//!   ([`ConsiderationDef`]) and [`DRIVE_KIND`] ([`DriveDef`]). Every tunable (bookends, curve
//!   parameters, weight, momentum, commit ticks, floor) lives in them; no defaults in code.
//! - [`Drives::build`] resolves input names against the adapter's [`Inputs`] vocabulary and
//!   consideration names against the records, once, at load; every unknown name and bad value is
//!   collected into one [`DriveLoadReport`] (as AI2's `htn::LoadReport`). Ids follow **record id
//!   order**, not load order, so the tie-break order is stable across mod lists.
//! - [`arbitrate`] is pure: `(inputs, previous ArbiterState, tick) -> Arbitration`. Time is only
//!   the logical tick passed in (no wall clock), so a tier that arbitrates less often switches at
//!   the first decision on or after the same tick (tier invariance). Ties go to the lowest
//!   [`DriveId`], never to hash order.
//! - [`serving_for`] maps the choice onto AI4's [`Serving`]: the `Directive` drive (or no drive)
//!   serves the stack, any other drive suspends it. Callers that react to directive completion
//!   must read `DirectiveDone` in the tick it is emitted or poll `DirectiveStack::status` (events
//!   aren't saved, ai-as-built.md "AI4").
//! - [`ArbiterState`] is serde; its save form is [`SavedArbiter`] (the drive by record id, since a
//!   [`DriveId`] is a load-time index). Not in the `sf/ai` section yet (a later version step).
//!
//! **NaN rule:** a NaN input (unknown, or an input the host didn't fill: a slice shorter than the
//! vocabulary reads NaN past its end) gives that consideration 0, which vetoes the drive. Every
//! curve output is clamped to 0..=1 and a NaN output is 0, so scores are always finite and >= 0.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use sf_records::{RecordDb, RecordError, RecordId};

use crate::directive::Serving;
use crate::htn::At;
use crate::ids::{ConsiderationId, DriveId, InputId};
use crate::trace::{ConsiderationScore, DriveReason, DriveScore, TraceEntry};

/// Record kind of considerations ([`ConsiderationDef`]); the record id is its name.
pub const CONSIDERATION_KIND: &str = "sf.ai.consideration";
/// Record kind of drives ([`DriveDef`]); the record id is its name.
pub const DRIVE_KIND: &str = "sf.ai.drive";

/// Registers [`CONSIDERATION_KIND`] and [`DRIVE_KIND`]. Call once before loading mods.
pub fn register_drive_kinds(db: &mut RecordDb) -> Result<(), RecordError> {
    db.register_serde::<ConsiderationDef>(CONSIDERATION_KIND)?;
    db.register_serde::<DriveDef>(DRIVE_KIND)?;
    Ok(())
}

// --- Records ---------------------------------------------------------------------------------

/// A response curve over `x` in 0..=1 (the IAUS set: Lewis's "Curvature" types). Every output
/// is clamped to 0..=1; a NaN output (e.g. a negative base with a fractional exponent) is 0. All
/// parameters are unitless and must be finite.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Curve {
    /// `y = slope * (x - x_shift)^exponent + y_shift`. `exponent` 1 is linear; 2, 3 ... ease in;
    /// 0.5 eases out.
    Polynomial { slope: f32, exponent: f32, x_shift: f32, y_shift: f32 },
    /// `y = slope / (1 + e^(-steepness * (x - x_shift))) + y_shift`: an S centred on `x_shift`
    /// (`steepness` around 10 spans the axis; negative falls instead of rising).
    Logistic { slope: f32, steepness: f32, x_shift: f32, y_shift: f32 },
    /// `y = slope * ln(u / (1 - u)) + y_shift` with `u = clamp(x - x_shift, 0, 1)`: the
    /// logistic's inverse, steep at both ends, flat in the middle. `u` 0 gives 0 and `u` 1 gives 1
    /// for a positive slope (the reverse for a negative one; slope 0 at the ends is NaN, so 0).
    Logit { slope: f32, x_shift: f32, y_shift: f32 },
    /// Linear between `(x, y)` points (x strictly increasing, at least one), flat outside them.
    Piecewise(Vec<(f32, f32)>),
}

impl Curve {
    /// The curve at `x`, clamped to 0..=1; NaN `x` gives 0.
    pub fn eval(&self, x: f32) -> f32 {
        if x.is_nan() {
            return 0.0;
        }
        let y = match self {
            Self::Polynomial { slope, exponent, x_shift, y_shift } => slope * (x - x_shift).powf(*exponent) + y_shift,
            Self::Logistic { slope, steepness, x_shift, y_shift } => {
                slope / (1.0 + (-steepness * (x - x_shift)).exp()) + y_shift
            }
            Self::Logit { slope, x_shift, y_shift } => {
                let u = (x - x_shift).clamp(0.0, 1.0);
                slope * (u / (1.0 - u)).ln() + y_shift
            }
            Self::Piecewise(points) => piecewise(points, x),
        };
        unit(y)
    }

    /// Why the parameters are invalid, if they are.
    fn invalid(&self) -> Option<&'static str> {
        let finite = |v: &[f32]| v.iter().all(|x| x.is_finite());
        match self {
            Self::Polynomial { slope, exponent, x_shift, y_shift } => {
                (!finite(&[*slope, *exponent, *x_shift, *y_shift])).then_some("has a non-finite parameter")
            }
            Self::Logistic { slope, steepness, x_shift, y_shift } => {
                (!finite(&[*slope, *steepness, *x_shift, *y_shift])).then_some("has a non-finite parameter")
            }
            Self::Logit { slope, x_shift, y_shift } => {
                (!finite(&[*slope, *x_shift, *y_shift])).then_some("has a non-finite parameter")
            }
            Self::Piecewise(points) => {
                if points.is_empty() {
                    Some("has no points")
                } else if !points.iter().all(|(x, y)| x.is_finite() && y.is_finite()) {
                    Some("has a non-finite point")
                } else if points.windows(2).any(|w| w[1].0 <= w[0].0) {
                    Some("points are not in strictly increasing x")
                } else {
                    None
                }
            }
        }
    }
}

/// `y` clamped to 0..=1; NaN and -0.0 give 0.0 (so scores compare with `total_cmp` as numbers).
fn unit(y: f32) -> f32 {
    if y > 0.0 { y.min(1.0) } else { 0.0 }
}

fn piecewise(points: &[(f32, f32)], x: f32) -> f32 {
    let (Some(&first), Some(&last)) = (points.first(), points.last()) else {
        return 0.0;
    };
    if x <= first.0 {
        return first.1;
    }
    if x >= last.0 {
        return last.1;
    }
    // First point right of x; 1..len, since first.0 < x < last.0.
    let i = points.partition_point(|p| p.0 <= x);
    let (a, b) = (points[i - 1], points[i]);
    a.1 + (b.1 - a.1) * (x - a.0) / (b.0 - a.0)
}

/// An `sf.ai.consideration` record: one input through bookends and a response curve.
///
/// No per-consideration weight (IAUS keeps every consideration in 0..=1 so the product means
/// "how true are all of these"; the curve's slope and shift already scale it). Weight is the
/// drive's.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsiderationDef {
    /// An input of the registered vocabulary ([`Inputs`]).
    pub input: String,
    /// Bookends `(at0, at1)`, in the input's own units: `at0` maps to curve `x` 0 and `at1` to 1,
    /// clamped between (IAUS bookends). `at0 > at1` reverses the axis (a nearer threat scores
    /// higher). Finite and different.
    pub range: (f32, f32),
    pub curve: Curve,
}

/// An `sf.ai.drive` record (`Directive`, `Combat`, `Flee`, ...). Every field but the flags is a
/// tunable with no code default.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DriveDef {
    /// `sf.ai.consideration` record ids, multiplied (compensated). None: the drive always scores
    /// its weight.
    pub considerations: Vec<String>,
    /// Multiplies the compensated product: score units, finite and >= 0 (IAUS uses a few fixed
    /// levels, e.g. 1, 2, 3, 5, so an emergency outranks a chore at any product).
    pub weight: f32,
    /// The running drive's bonus: its score times `1 + momentum`. Unitless fraction, finite and
    /// >= 0 (0.1 to 0.5 is typical; 0 disables it).
    pub momentum: f32,
    /// Minimum commit, in fixed AI ticks (the logical tick `arbitrate` gets): once chosen, the
    /// drive keeps the actor this long whatever the other scores, unless its own raw score drops
    /// to 0. 0 disables it.
    pub commit_ticks: u32,
    /// While the drive applies (raw > 0), its score is at least this: score units, finite and
    /// >= 0. Meant for the `Directive` drive.
    #[serde(default)]
    pub floor: Option<f32>,
    /// This is the `Directive` drive: when it wins, the actor serves its directive stack
    /// ([`serving_for`]). At most one drive sets it.
    #[serde(default)]
    pub directive: bool,
}

// --- Vocabulary ------------------------------------------------------------------------------

/// The input vocabulary: names considerations may read, registered by the adapter (perception,
/// stats, the stack) before the drives load. Names fold to ASCII lowercase; ids follow
/// registration order and index the `inputs` slice [`arbitrate`] gets.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Inputs {
    names: Vec<String>,
    by_name: HashMap<String, InputId>,
}

impl Inputs {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers (or finds) `name`.
    ///
    /// # Panics
    /// Past `u16::MAX` inputs.
    pub fn add(&mut self, name: &str) -> InputId {
        let name = name.to_ascii_lowercase();
        if let Some(&id) = self.by_name.get(&name) {
            return id;
        }
        let id = InputId(u16::try_from(self.names.len()).expect("fewer than 65536 inputs"));
        self.names.push(name.clone());
        self.by_name.insert(name, id);
        id
    }

    pub fn get(&self, name: &str) -> Option<InputId> {
        self.by_name.get(&name.to_ascii_lowercase()).copied()
    }

    /// `"?"` for an id from another vocabulary.
    pub fn name(&self, id: InputId) -> &str {
        self.names.get(usize::from(id.0)).map_or("?", String::as_str)
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}

// --- Compiled drives -------------------------------------------------------------------------

/// A resolved consideration.
#[derive(Clone, Debug, PartialEq)]
pub struct Consideration {
    pub input: InputId,
    pub range: (f32, f32),
    pub curve: Curve,
}

impl Consideration {
    /// The consideration's value for a raw `input`: bookends, clamp, curve. NaN gives 0.
    pub fn eval(&self, input: f32) -> f32 {
        let (at0, at1) = self.range;
        // NaN stays NaN through the clamp and the curve returns 0.
        self.curve.eval(((input - at0) / (at1 - at0)).clamp(0.0, 1.0))
    }
}

/// A resolved drive.
#[derive(Clone, Debug, PartialEq)]
pub struct Drive {
    pub considerations: Vec<ConsiderationId>,
    pub weight: f32,
    pub momentum: f32,
    pub commit_ticks: u32,
    pub floor: Option<f32>,
    pub directive: bool,
    /// IAUS modification factor `1 - 1/n` (0 for n <= 1).
    compensation: f32,
}

impl Drive {
    /// `raw` lifted to the floor while the drive applies.
    fn lift(&self, raw: f32) -> f32 {
        match self.floor {
            Some(floor) if raw > 0.0 => raw.max(floor),
            _ => raw,
        }
    }
}

/// One problem found while building [`Drives`].
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum DriveLoadError {
    #[error("{at}: unknown input `{name}`")]
    UnknownInput { at: At, name: String },
    #[error("{at}: unknown consideration `{name}`")]
    UnknownConsideration { at: At, name: String },
    #[error("{at}: {field} {why}")]
    BadValue { at: At, field: &'static str, why: &'static str },
    #[error("{at}: a second directive drive (the first is `{first}`)")]
    SecondDirective { at: At, first: RecordId },
    #[error("too many {0} records")]
    TooMany(&'static str),
}

/// Every problem found while building [`Drives`], considerations first, each kind in record id
/// order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DriveLoadReport {
    pub errors: Vec<DriveLoadError>,
}

impl fmt::Display for DriveLoadReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AI drives: {} error(s)", self.errors.len())?;
        for e in &self.errors {
            write!(f, "\n  {e}")?;
        }
        Ok(())
    }
}

impl std::error::Error for DriveLoadReport {}

/// The record-built drives and considerations, ids in record id order.
#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::struct_field_names, reason = "the drive tables are named after what they hold")]
pub struct Drives {
    considerations: Vec<Consideration>,
    consideration_names: Vec<RecordId>,
    consideration_ids: HashMap<RecordId, ConsiderationId>,
    drives: Vec<Drive>,
    drive_names: Vec<RecordId>,
    drive_ids: HashMap<RecordId, DriveId>,
    directive: Option<DriveId>,
    inputs: Inputs,
}

impl Drives {
    /// Builds the drives from the live [`CONSIDERATION_KIND`] and [`DRIVE_KIND`] records of `db`.
    pub fn build(db: &RecordDb, inputs: &Inputs) -> Result<Self, DriveLoadReport> {
        Self::from_defs(
            db.iter::<ConsiderationDef>().map(|(id, d)| (id.clone(), d)),
            db.iter::<DriveDef>().map(|(id, d)| (id.clone(), d)),
            inputs,
        )
    }

    /// Builds the drives from definitions (ids are the names), in any order: ids follow record
    /// id order. Collects every error before failing.
    pub fn from_defs<'d>(
        considerations: impl IntoIterator<Item = (RecordId, &'d ConsiderationDef)>,
        drives: impl IntoIterator<Item = (RecordId, &'d DriveDef)>,
        inputs: &Inputs,
    ) -> Result<Self, DriveLoadReport> {
        let mut cons: Vec<_> = considerations.into_iter().collect();
        let mut defs: Vec<_> = drives.into_iter().collect();
        cons.sort_by(|a, b| a.0.cmp(&b.0));
        defs.sort_by(|a, b| a.0.cmp(&b.0));
        if u16::try_from(cons.len()).is_err() {
            return Err(DriveLoadReport { errors: vec![DriveLoadError::TooMany(CONSIDERATION_KIND)] });
        }
        if u16::try_from(defs.len()).is_err() {
            return Err(DriveLoadReport { errors: vec![DriveLoadError::TooMany(DRIVE_KIND)] });
        }
        let mut errors = Vec::new();

        let mut considerations = Vec::with_capacity(cons.len());
        for (id, def) in &cons {
            let at = || At { kind: CONSIDERATION_KIND, id: id.clone() };
            let input = inputs.get(&def.input);
            if input.is_none() {
                errors.push(DriveLoadError::UnknownInput { at: at(), name: def.input.clone() });
            }
            let (at0, at1) = def.range;
            #[allow(clippy::float_cmp)] // equal bookends divide by zero: exactly equal is the bad case
            if !(at0.is_finite() && at1.is_finite()) || at0 == at1 {
                errors.push(DriveLoadError::BadValue { at: at(), field: "range", why: "must be two finite, different values" });
            }
            if let Some(why) = def.curve.invalid() {
                errors.push(DriveLoadError::BadValue { at: at(), field: "curve", why });
            }
            considerations.push(Consideration {
                input: input.unwrap_or(InputId(0)),
                range: def.range,
                curve: def.curve.clone(),
            });
        }
        let consideration_ids: HashMap<RecordId, ConsiderationId> =
            (0u16..).zip(&cons).map(|(i, (id, _))| (id.clone(), ConsiderationId(i))).collect();

        let mut compiled = Vec::with_capacity(defs.len());
        let mut directive: Option<DriveId> = None;
        for (i, (id, def)) in (0u16..).zip(&defs) {
            let at = || At { kind: DRIVE_KIND, id: id.clone() };
            let mut list = Vec::with_capacity(def.considerations.len());
            for name in &def.considerations {
                match consideration_ids.get(&RecordId::new(name)) {
                    Some(&c) => list.push(c),
                    None => errors.push(DriveLoadError::UnknownConsideration { at: at(), name: name.clone() }),
                }
            }
            let bad = |v: f32| !(v.is_finite() && v >= 0.0);
            if bad(def.weight) {
                errors.push(DriveLoadError::BadValue { at: at(), field: "weight", why: "must be finite and >= 0" });
            }
            if bad(def.momentum) {
                errors.push(DriveLoadError::BadValue { at: at(), field: "momentum", why: "must be finite and >= 0" });
            }
            if def.floor.is_some_and(bad) {
                errors.push(DriveLoadError::BadValue { at: at(), field: "floor", why: "must be finite and >= 0" });
            }
            let compensation = match u16::try_from(def.considerations.len()) {
                Ok(0 | 1) => 0.0,
                Ok(n) => 1.0 - 1.0 / f32::from(n),
                Err(_) => {
                    errors.push(DriveLoadError::BadValue {
                        at: at(),
                        field: "considerations",
                        why: "has more than 65535 entries",
                    });
                    0.0
                }
            };
            if def.directive {
                match directive {
                    Some(first) => errors.push(DriveLoadError::SecondDirective {
                        at: at(),
                        first: defs[usize::from(first.0)].0.clone(),
                    }),
                    None => directive = Some(DriveId(i)),
                }
            }
            compiled.push(Drive {
                considerations: list,
                weight: def.weight,
                momentum: def.momentum,
                commit_ticks: def.commit_ticks,
                floor: def.floor,
                directive: def.directive,
                compensation,
            });
        }
        if !errors.is_empty() {
            return Err(DriveLoadReport { errors });
        }
        let drive_ids = (0u16..).zip(&defs).map(|(i, (id, _))| (id.clone(), DriveId(i))).collect();
        Ok(Self {
            considerations,
            consideration_names: cons.into_iter().map(|(id, _)| id).collect(),
            consideration_ids,
            drives: compiled,
            drive_names: defs.into_iter().map(|(id, _)| id).collect(),
            drive_ids,
            directive,
            inputs: inputs.clone(),
        })
    }

    pub fn drive(&self, id: DriveId) -> Option<&Drive> {
        self.drives.get(usize::from(id.0))
    }

    pub fn drive_id(&self, name: &str) -> Option<DriveId> {
        self.drive_ids.get(&RecordId::new(name)).copied()
    }

    /// `"?"` for an unknown id.
    pub fn drive_name(&self, id: DriveId) -> &str {
        self.drive_names.get(usize::from(id.0)).map_or("?", RecordId::as_str)
    }

    pub fn consideration(&self, id: ConsiderationId) -> Option<&Consideration> {
        self.considerations.get(usize::from(id.0))
    }

    pub fn consideration_id(&self, name: &str) -> Option<ConsiderationId> {
        self.consideration_ids.get(&RecordId::new(name)).copied()
    }

    /// `"?"` for an unknown id.
    pub fn consideration_name(&self, id: ConsiderationId) -> &str {
        self.consideration_names.get(usize::from(id.0)).map_or("?", RecordId::as_str)
    }

    /// The `Directive` drive, if one is defined.
    pub fn directive(&self) -> Option<DriveId> {
        self.directive
    }

    pub fn inputs(&self) -> &Inputs {
        &self.inputs
    }

    /// Number of drives (ids `0..len`).
    pub fn len(&self) -> usize {
        self.drives.len()
    }

    pub fn is_empty(&self) -> bool {
        self.drives.is_empty()
    }

    /// A drive's raw score for `inputs` (indexed by [`InputId`]; NaN past the end): the
    /// compensated product of its considerations times its weight. 0 for an unknown id.
    pub fn raw(&self, id: DriveId, inputs: &[f32]) -> f32 {
        self.drive(id).map_or(0.0, |d| self.raw_score(d, inputs, None))
    }

    /// The compensated product times the weight; with `out`, every consideration is evaluated
    /// and recorded, without it the product stops at 0 (same result: 0 stays 0).
    fn raw_score(&self, drive: &Drive, inputs: &[f32], mut out: Option<&mut Vec<ConsiderationScore>>) -> f32 {
        let mut product = 1.0_f32;
        for &c in &drive.considerations {
            let con = &self.considerations[usize::from(c.0)];
            let input = inputs.get(usize::from(con.input.0)).copied().unwrap_or(f32::NAN);
            let value = con.eval(input);
            product *= value + (1.0 - value) * drive.compensation * value;
            match out.as_deref_mut() {
                Some(out) => out.push(ConsiderationScore { consideration: c, input, value }),
                None if product <= 0.0 => break,
                None => {}
            }
        }
        // `unit` already made every factor >= +0.0; normalise a 0 product's sign anyway.
        if product > 0.0 { product * drive.weight } else { 0.0 }
    }
}

// --- Arbitration -----------------------------------------------------------------------------

/// The arbiter's memory per actor: the running drive and the tick it started. Serde; saved as
/// [`SavedArbiter`] (a [`DriveId`] is a load-time index).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArbiterState {
    pub running: Option<DriveId>,
    /// The tick `running` was chosen (its commit time counts from here).
    pub since: u64,
}

impl ArbiterState {
    /// The save form: the running drive by record id.
    pub fn to_saved(&self, drives: &Drives) -> SavedArbiter {
        SavedArbiter {
            drive: self.running.and_then(|d| drives.drive_names.get(usize::from(d.0)).cloned()),
            since: self.since,
        }
    }
}

/// [`ArbiterState`]'s save form, stable across mod lists (the `sf/ai` section takes it in a later
/// version step).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedArbiter {
    pub drive: Option<RecordId>,
    pub since: u64,
}

impl SavedArbiter {
    /// Back to an [`ArbiterState`]; a drive no longer defined (its mod is gone) runs nothing, so
    /// the next arbitration picks freely.
    pub fn resolve(&self, drives: &Drives) -> ArbiterState {
        let running = self.drive.as_ref().and_then(|name| drives.drive_ids.get(name).copied());
        ArbiterState { running, since: self.since }
    }
}

/// One drive's scores this decision, as [`DriveScore`] has them (without the breakdown).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Scored {
    pub raw: f32,
    pub score: f32,
}

/// What [`arbitrate`] decided.
#[derive(Clone, Debug, PartialEq)]
pub struct Arbitration {
    /// The winning drive; `None` when every drive scored 0.
    pub drive: Option<DriveId>,
    pub reason: DriveReason,
    /// The state for the next decision.
    pub state: ArbiterState,
    /// Every drive's scores, indexed by [`DriveId`].
    pub scores: Vec<Scored>,
}

/// `a` (drive `ai`) beats `b` (drive `bi`): higher score, ties to the lower id.
fn beats(a: f32, ai: usize, b: f32, bi: usize) -> bool {
    match a.total_cmp(&b) {
        Ordering::Greater => true,
        Ordering::Less => false,
        Ordering::Equal => ai < bi,
    }
}

/// The best drive by score (ties to the lowest id), skipping `skip`.
fn best_of(scores: &[Scored], skip: Option<usize>) -> Option<usize> {
    let mut best: Option<usize> = None;
    for (i, s) in scores.iter().enumerate() {
        if Some(i) == skip {
            continue;
        }
        if best.is_none_or(|b| beats(s.score, i, scores[b].score, b)) {
            best = Some(i);
        }
    }
    best
}

/// Picks the drive for this decision. Pure: the same `(drives, inputs, prev, tick)` gives the
/// same result on every machine and tier; `trace` (identical result with `None`) gets every
/// drive's scores with each consideration's raw input and curve value, the winner and its
/// [`DriveReason`].
///
/// Rules, in order: score every drive (`raw`, lifted to its floor while it applies, times
/// `1 + momentum` if running); the highest score wins, ties to the lowest id; if nothing scores
/// above 0, no drive runs ([`DriveReason::Idle`]); a running drive inside its minimum commit
/// (`tick - since < commit_ticks`) keeps the actor while its raw score is above 0
/// ([`DriveReason::Commit`]). A switch restarts `since` at `tick`.
pub fn arbitrate(
    drives: &Drives,
    inputs: &[f32],
    prev: &ArbiterState,
    tick: u64,
    mut trace: Option<&mut TraceEntry>,
) -> Arbitration {
    let running = prev.running.map(|d| usize::from(d.0)).filter(|&r| r < drives.drives.len());
    if let Some(e) = trace.as_deref_mut() {
        e.drives.clear();
    }
    let mut scores = Vec::with_capacity(drives.drives.len());
    for (i, drive) in drives.drives.iter().enumerate() {
        let mut breakdown = trace.is_some().then(|| Vec::with_capacity(drive.considerations.len()));
        let raw = drives.raw_score(drive, inputs, breakdown.as_mut());
        let lifted = drive.lift(raw);
        let score = if Some(i) == running { lifted * (1.0 + drive.momentum) } else { lifted };
        scores.push(Scored { raw, score });
        if let (Some(e), Some(considerations)) = (trace.as_deref_mut(), breakdown) {
            e.drives.push(DriveScore { drive: DriveId(index_u16(i)), raw, score, considerations });
        }
    }

    let (mut choice, mut reason) = match best_of(&scores, None) {
        Some(w) if scores[w].score > 0.0 => (Some(w), why(drives, &scores, w, running)),
        _ => (None, DriveReason::Idle),
    };
    if let Some(r) = running {
        let committed = tick.saturating_sub(prev.since) < u64::from(drives.drives[r].commit_ticks);
        if choice != Some(r) && committed && scores[r].raw > 0.0 {
            (choice, reason) = (Some(r), DriveReason::Commit);
        }
    }

    let drive = choice.map(|c| DriveId(index_u16(c)));
    let state = if choice.is_some() && choice == running {
        ArbiterState { running: drive, since: prev.since }
    } else {
        ArbiterState { running: drive, since: tick }
    };
    if let Some(e) = trace {
        e.drive = drive;
        e.drive_reason = Some(reason);
    }
    Arbitration { drive, reason, state, scores }
}

/// Drive ids fit `u16`: `from_defs` refuses more.
fn index_u16(i: usize) -> u16 {
    u16::try_from(i).expect("drive ids fit u16")
}

/// Why `w` (score above 0) is the best: the first rule it needed against the runner-up.
fn why(drives: &Drives, scores: &[Scored], w: usize, running: Option<usize>) -> DriveReason {
    let Some(r) = best_of(scores, Some(w)) else {
        return DriveReason::Best;
    };
    let drive = &drives.drives[w];
    let rival = scores[r].score;
    let raw = scores[w].raw;
    let momentum = if Some(w) == running { 1.0 + drive.momentum } else { 1.0 };
    if Some(w) == running && !beats(drive.lift(raw), w, rival, r) {
        DriveReason::Momentum
    } else if !beats(raw * momentum, w, rival, r) {
        DriveReason::Floor
    } else if scores[w].score.total_cmp(&rival) == Ordering::Equal {
        DriveReason::Tie
    } else {
        DriveReason::Best
    }
}

/// AI4's [`Serving`] for this decision: a reaction drive (any drive but the `Directive` one)
/// suspends the stack; the `Directive` drive, or no drive at all, serves it: `Top` after a
/// suspension, otherwise `current` is kept (a `Directive(id)` someone named stays).
pub fn serving_for(drives: &Drives, chosen: Option<DriveId>, current: Serving) -> Serving {
    match chosen {
        Some(d) if Some(d) != drives.directive => Serving::Suspended,
        _ => match current {
            Serving::Suspended => Serving::Top,
            other => other,
        },
    }
}
