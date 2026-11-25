//! Bevy-free crime core (`superfargoth/docs/specs/crime.md`).
//!
//! `LW4a` ships the stealth seam ("Stealth and detection"): [`DetectionRules::concealment`] turns
//! what an observer sees of a target ([`DetectIn`], its light, stance and speed in `sf_ai`'s
//! [`VisibilityFactors`]) into a concealment number; `sf_ai` turns that into sight visibility with
//! [`visibility_from_concealment`] and runs the detection model (crime owns the formula, AI owns
//! detection: decision 6). [`StandardDetection`] is the framework's modern default (stance, speed,
//! light, facing); a game swaps in its own rules (classic ports live in the game's adapter).
//!
//! Ownership, offences, locks, traps and settlement follow in later slices.

mod stealth;

pub use sf_ai::{Stance, StatId, VisibilityFactors};
pub use stealth::{
    DetectIn, DetectionRules, ObserverView, STAT_VIEW_CAPACITY, Speed, StandardDetection, StandardDetectionConfig,
    StatView, StealthState, TargetView, visibility_from_concealment,
};
