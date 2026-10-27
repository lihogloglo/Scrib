//! Player movement feel: THE place to tune how the player starts, stops and turns
//! (owner PL-2 2026-10-27, "driving a car"; visual check PL-4; locomotion.md "Player feel").
//!
//! Reference: ELDEN RING Combat Rewrite (Funny-Bones, MIT): run speed in ~0.15 s, stop in
//! ~0.13 s, facing capped at 1080 deg/s running and 480 sprinting, the motion follows the
//! facing so a reversal turns at speed. NPCs keep `MoverParams::default()` (0.27 s springs).
//!
//! Elsewhere, shared with NPCs: the gait blends of the humanoid graph
//! (`mw_graphs::build` `GAIT_BLEND` 0.15 s, `DIRECTION_SMOOTHING` 0.05 s; re-bless the RONs
//! after a change, `MW_GRAPHS_BLESS=1 cargo test -p mw_graphs`). Speeds per gait:
//! `sf_action::GaitTable` (walk 1.6, run 4.5, sprint 7.0 m/s).
//!
//! A classic (Morrowind, instant) preset would be: both half-lives 0.0, turn rates
//! `f32::INFINITY` (`sf_controller` snaps on both).

use sf_controller::{MoverParams, SpringParams, TurnRates};

/// Velocity half-life while speeding up, seconds (90 % of the target speed in ~2.9x this).
pub const ACCEL_HALFLIFE: f32 = 0.07;
/// Velocity half-life while slowing down, seconds (stop distance from run ~2 v x this / 1.39).
pub const BRAKE_HALFLIFE: f32 = 0.05;
/// Top turn rates of the facing and of the direction of travel, deg/s.
pub const TURN_WALK_DEG: f32 = 720.0;
pub const TURN_RUN_DEG: f32 = 1080.0;
pub const TURN_SPRINT_DEG: f32 = 480.0;
pub const TURN_SNEAK_DEG: f32 = 540.0;
/// Half-life of the levelling of an authored tilt (and the facing spring if the turn
/// rates are removed), seconds.
pub const FACING_HALFLIFE: f32 = 0.1;
/// Smoothing of the drawn motion the animation feed measures, seconds (NPCs:
/// `sf_controller_bevy::MEASURE_HALFLIFE`, 0.05). Shorter: the legs catch up sooner on
/// starts and stops; longer hides step-up spikes.
pub const MEASURE_HALFLIFE: f32 = 0.025;

/// The player's motion springs.
pub fn springs() -> SpringParams {
    SpringParams {
        halflife_velocity: ACCEL_HALFLIFE,
        halflife_facing: FACING_HALFLIFE,
        halflife_brake: Some(BRAKE_HALFLIFE),
        turn: Some(TurnRates {
            walk: TURN_WALK_DEG.to_radians(),
            run: TURN_RUN_DEG.to_radians(),
            sprint: TURN_SPRINT_DEG.to_radians(),
            sneak: TURN_SNEAK_DEG.to_radians(),
        }),
    }
}

/// The player's mover: the generic human shape with [`springs`].
pub fn mover_params() -> MoverParams {
    MoverParams { springs: springs(), ..MoverParams::default() }
}
