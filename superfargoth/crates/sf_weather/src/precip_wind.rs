//! Precipitation's wind (`docs/specs/weather/wind.md` "Precipitation tilt as built"): how far the
//! drops lean in the weather's wind, and the slowly turning wind they lean in. Presentation only:
//! nothing here feeds gameplay (the frame's wind stays the gameplay value).
//!
//! Physics: a drop reaches the air's horizontal speed within its relaxation time (`v_t / g`,
//! under a second), so it falls along `atan(u / v_t)` from vertical, `u` the horizontal wind at
//! drop height and `v_t` the terminal fall speed (rain up to ~9 m/s, snow 0.5-2 m/s: AMS Glossary
//! "terminal fall velocity"). A 5 m/s breeze leans 8 m/s rain ~30°, which reads as a gale on
//! screen. Games keep rain near vertical and lean it only in storms (`OpenMW` `sky.cpp`
//! `updateRainParameters`, d0c0fc0: `atan(wind / 50)`, a few degrees), and drive every effect
//! from one global wind that turns slowly (Ghost of Tsushima, the `PlayStation` Blog 2020 "How stunning
//! visual effects bring Ghost of Tsushima to life"). So:
//!
//! - [`drop_drift`]: the drawn horizontal drift: physical below the cap, a soft calm onset
//!   (`u² / (u + CALM_MPS)`: light air leaves rain vertical) and a soft cap
//!   (`k · tanh(d / k)`, `k = v_t · tan(max tilt)`: linear at first, never past the cap). The
//!   cap is preset data (`WindTerms::precip_tilt_max_deg`, blended into the frame).
//! - [`PrecipWindFilter`]: the wind the drops see, low-passed: the direction turns toward the
//!   target exponentially in angle ([`DIRECTION_TAU_S`]), the speed follows over
//!   [`SPEED_TAU_S`]; both exact for any step size (`1 - exp(-dt / τ)`), so 30 and 144 Hz agree.

use glam::Vec2;

/// Horizontal drift (m/s) below which the drops stay near vertical: the calm onset's scale.
pub const CALM_MPS: f32 = 1.5;
/// Time constant of the direction the drops lean toward, seconds (weather turns over minutes;
/// a transition's 30-60 s swing spreads over about two minutes).
pub const DIRECTION_TAU_S: f32 = 30.0;
/// Time constant of the drift speed, seconds.
pub const SPEED_TAU_S: f32 = 8.0;
/// Target wind below which the direction holds (calm has no direction).
pub const TURN_MIN_MPS: f32 = 0.25;
/// Height above ground of the drops the eye reads, meters: the wind profile factor is taken
/// there (the weather's wind is at the 10 m reference height).
pub const DROP_HEIGHT_M: f32 = 3.0;
/// A tilt cap at or above this many degrees is no cap (snow and ash blow free).
pub const UNCAPPED_DEG: f32 = 90.0;

/// The calm onset: `u² / (u + CALM_MPS)`, ~`u² / CALM_MPS` in light air, `u - CALM_MPS` in wind.
pub fn calm_onset(u: f32) -> f32 {
    let u = u.max(0.0);
    u * u / (u + CALM_MPS)
}

/// Drawn horizontal drift (m/s) of drops falling at `fall_speed` (m/s) in a horizontal wind
/// `u` (m/s, at drop height, times the kind's follow share), capped softly at `max_tilt_deg`
/// from vertical ([`UNCAPPED_DEG`] and above: no cap).
pub fn drop_drift(u: f32, fall_speed: f32, max_tilt_deg: f32) -> f32 {
    let d = calm_onset(u);
    if fall_speed <= 0.0 || max_tilt_deg >= UNCAPPED_DEG {
        return d;
    }
    let k = fall_speed * max_tilt_deg.max(0.0).to_radians().tan();
    if k <= 0.0 { 0.0 } else { k * (d / k).tanh() }
}

/// Tilt from vertical, degrees, of drops drifting at `drift` while falling at `fall_speed`.
pub fn tilt_deg(drift: f32, fall_speed: f32) -> f32 {
    drift.atan2(fall_speed).to_degrees()
}

/// The precipitation's low-passed wind (see the module docs). Not saved: it snaps to the
/// target on the first update.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PrecipWindFilter {
    dir: Vec2,
    speed: f32,
    started: bool,
}

impl Default for PrecipWindFilter {
    fn default() -> Self {
        Self { dir: Vec2::X, speed: 0.0, started: false }
    }
}

impl PrecipWindFilter {
    /// Moves toward `target` (m/s, x and z) over `dt` seconds and returns the smoothed wind.
    pub fn update(&mut self, target: Vec2, dt: f32) -> Vec2 {
        let speed = target.length();
        if !self.started {
            self.started = true;
            self.speed = speed;
            if speed > TURN_MIN_MPS {
                self.dir = target / speed;
            }
            return self.velocity();
        }
        let dt = dt.max(0.0);
        if speed > TURN_MIN_MPS {
            // atan2, not `angle_to` (an acos: it rounds small angles to 0 and stalls the turn).
            let to = target / speed;
            let angle = self.dir.perp_dot(to).atan2(self.dir.dot(to));
            let turn = angle * (1.0 - (-dt / DIRECTION_TAU_S).exp());
            self.dir = Vec2::from_angle(turn).rotate(self.dir).normalize_or(Vec2::X);
        }
        self.speed += (speed - self.speed) * (1.0 - (-dt / SPEED_TAU_S).exp());
        self.velocity()
    }

    /// The smoothed wind, m/s.
    pub fn velocity(&self) -> Vec2 {
        self.dir * self.speed
    }

    /// The unit direction the drops lean toward.
    pub fn direction(&self) -> Vec2 {
        self.dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f32, b: f32, eps: f32, what: &str) {
        assert!((a - b).abs() <= eps, "{what}: {a} vs {b}");
    }

    #[test]
    fn drift_is_the_closed_form() {
        // Calm onset: u² / (u + 1.5).
        close(calm_onset(3.0), 9.0 / 4.5, 1e-6, "onset");
        close(calm_onset(-2.0), 0.0, 0.0, "negative wind");
        // Uncapped: the onset alone.
        close(drop_drift(3.0, 1.0, 90.0), 2.0, 1e-6, "uncapped");
        close(drop_drift(3.0, 0.0, 20.0), 2.0, 1e-6, "no fall speed");
        // Capped: k tanh(d / k), k = v tan(cap).
        let k = 8.0 * 20f32.to_radians().tan();
        close(drop_drift(3.0, 8.0, 20.0), k * (2.0 / k).tanh(), 1e-5, "capped");
        close(drop_drift(3.0, 8.0, 0.0), 0.0, 0.0, "zero cap is vertical");
        close(tilt_deg(8.0, 8.0), 45.0, 1e-4, "tilt");
    }

    #[test]
    fn tilt_never_passes_the_cap_and_grows_with_wind() {
        let mut last = -1.0;
        for i in 0..=400u16 {
            let u = f32::from(i) * 0.25;
            let t = tilt_deg(drop_drift(u, 8.2, 20.0), 8.2);
            assert!(t <= 20.0 + 1e-4 && t >= last, "u {u}: tilt {t} after {last}");
            last = t;
        }
        assert!(last > 19.9, "a gale reaches the cap: {last}");
        // Light air stays near vertical.
        assert!(tilt_deg(drop_drift(1.0, 8.2, 20.0), 8.2) < 3.0);
    }

    /// Steps of `1 / hz` for `seconds` toward a fixed target.
    fn run(hz: u16, seconds: f32, target: Vec2) -> PrecipWindFilter {
        let mut f = PrecipWindFilter::default();
        f.update(Vec2::new(4.0, 0.0), 0.0);
        let dt = 1.0 / f32::from(hz);
        #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "small positive step counts")]
        let steps = (seconds * f32::from(hz)).round() as u32;
        for _ in 0..steps {
            f.update(target, dt);
        }
        f
    }

    #[test]
    fn smoothing_is_frame_rate_independent_and_converges() {
        // A 90° turn and a speed change, 10 s at 30 Hz vs 144 Hz.
        let target = Vec2::new(0.0, 6.0);
        let (a, b) = (run(30, 10.0, target), run(144, 10.0, target));
        let between = |p: Vec2, q: Vec2| p.perp_dot(q).atan2(p.dot(q));
        close(between(a.direction(), b.direction()), 0.0, 1e-4, "direction 30 vs 144 Hz");
        close(a.velocity().length(), b.velocity().length(), 1e-3, "speed 30 vs 144 Hz");
        // Exact exponential: the angle left is 90° · exp(-10 / τ).
        let left = between(a.direction(), Vec2::Y).abs();
        close(left, std::f32::consts::FRAC_PI_2 * (-10.0 / DIRECTION_TAU_S).exp(), 1e-4, "angle left");
        close(a.velocity().length(), 6.0 + (4.0 - 6.0) * (-10.0 / SPEED_TAU_S).exp(), 1e-3, "speed left");
        // Converges.
        let c = run(60, 20.0 * DIRECTION_TAU_S, target);
        assert!(c.velocity().distance(target) < 1e-3, "{:?}", c.velocity());
    }

    #[test]
    fn calm_holds_the_direction_and_first_update_snaps() {
        let mut f = PrecipWindFilter::default();
        assert_eq!(f.update(Vec2::new(0.0, -3.0), 0.0), Vec2::new(0.0, -3.0));
        f.update(Vec2::ZERO, 100.0);
        assert_eq!(f.direction(), Vec2::NEG_Y);
        assert!(f.velocity().length() < 1e-3);
    }
}
