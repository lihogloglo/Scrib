//! Impact damage core (interaction-destruction.md "Impacts (IN4)", IN4a): the damage one physics
//! contact deals to each registered side, from the solver's normal impulse and the normal speed
//! change it caused. Host only, pure: same inputs, same bits.
//!
//! - **Speed gate.** Below `interact.impact.speed_min_mps` nothing (a resting or sliding body changes
//!   its normal velocity by `g x dt` a step, so the gate removes it whatever its mass).
//! - **Dead zone.** Impulses up to `interact.impact.impulse_min_ns` deal nothing (Source's
//!   `minhealthdmg`); above it the damage is linear, `damage_per_ns x (impulse - impulse_min_ns)`.
//! - **Both sides take the same raw amount** (one impulse acts on both bodies); the shell's
//!   `(impact, material)` factor and each side's health split it. Non-finite input gives 0; an overflow
//!   saturates at `f32::MAX`.

use crate::InteractParams;

/// Raw impact damage of one contact: `damage_per_ns x (impulse_ns - impulse_min_ns)` when
/// `speed_mps >= speed_min_mps` and the impulse passes the dead zone, else 0 (non-finite in -> 0).
#[must_use]
pub fn impact_damage(impulse_ns: f32, speed_mps: f32, params: &InteractParams) -> f32 {
    if !impulse_ns.is_finite() || !speed_mps.is_finite() || speed_mps < params.impact_speed_min_mps {
        return 0.0;
    }
    let over = impulse_ns - params.impact_impulse_min_ns;
    if over <= 0.0 {
        return 0.0;
    }
    // A finite impulse whose damage overflows saturates (it still breaks), never infinity; NaN
    // (a non-finite tunable) gives 0.
    let damage = params.impact_damage_per_ns * over;
    if damage > 0.0 { damage.min(f32::MAX) } else { 0.0 }
}
