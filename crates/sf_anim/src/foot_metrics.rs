//! Foot-slide and penetration metrics (animation.md "Verification"): how
//! well a character's feet stay put and on the ground while their contact
//! curves say they are planted. Engine-free: feed one [`FootFrame`] per
//! foot per update (world position, contact weight, ground height under
//! the foot) to a [`FootMeter`], or a whole recording to [`measure`].
//!
//! What is measured, per foot:
//! - **slide**: horizontal distance the foot travels while planted, per
//!   stance and per second of stance (a planted foot should not move; a
//!   skating one does);
//! - **penetration / float**: how far the sole sits below / above the
//!   ground on planted frames;
//! - **stances**, and the longest stretch spent without one (a leg that
//!   never plants) or in one (a leg stuck planted while the body moves).
//!
//! Conventions: Y up, meters, seconds.

use glam::Vec3;

/// One foot on one update.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FootFrame {
    /// The foot joint (effector), world space.
    pub position: Vec3,
    /// Contact weight (the clip's `contact.<joint>` curve after blending).
    pub contact: f32,
    /// Ground height under the foot, if the probe hit.
    pub ground: Option<f32>,
}

/// How [`FootMeter`] reads its frames.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeterRules {
    /// Contact weight at or above which the foot counts as planted.
    pub planted: f32,
    /// Seconds at the start of a stance not counted as slide: a runtime
    /// foot lock blends in over about 0.1 s, and the authored touch-down
    /// still carries the swing's last motion.
    pub settle: f32,
    /// Height of the foot joint over the ground when the sole is flat on
    /// it (rig profile `foot.sole`).
    pub sole: f32,
}

impl Default for MeterRules {
    fn default() -> Self {
        Self { planted: 0.5, settle: 0.1, sole: 0.0 }
    }
}

/// One foot's totals over the measured frames.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FootMetrics {
    /// Time measured.
    pub seconds: f32,
    /// Stances (planted spans) seen, including one already running at the
    /// first frame.
    pub stances: u32,
    /// Time spent planted.
    pub stance_seconds: f32,
    /// Horizontal distance travelled while planted, after each stance's
    /// `settle`, m.
    pub slide: f32,
    /// The largest slide of any one stance, m.
    pub max_stance_slide: f32,
    /// Planted frames with a ground height.
    pub grounded_frames: u32,
    /// Deepest and summed depth of the sole below the ground on planted
    /// frames, m.
    pub max_penetration: f32,
    pub penetration_sum: f32,
    /// Highest and summed height of the sole above the ground on planted
    /// frames, m.
    pub max_float: f32,
    pub float_sum: f32,
    /// Longest stretch without being planted, and longest single stance, s.
    pub longest_swing: f32,
    pub longest_stance: f32,
}

impl FootMetrics {
    /// Mean slide per stance, m.
    #[must_use]
    pub fn slide_per_stance(&self) -> f32 {
        #[allow(clippy::cast_precision_loss)]
        if self.stances == 0 { 0.0 } else { self.slide / self.stances as f32 }
    }

    /// Slide per second of stance, m/s.
    #[must_use]
    pub fn slide_speed(&self) -> f32 {
        if self.stance_seconds > 0.0 { self.slide / self.stance_seconds } else { 0.0 }
    }

    /// Mean depth below the ground over planted frames (zero where above), m.
    #[must_use]
    pub fn mean_penetration(&self) -> f32 {
        #[allow(clippy::cast_precision_loss)]
        if self.grounded_frames == 0 { 0.0 } else { self.penetration_sum / self.grounded_frames as f32 }
    }

    /// Mean height above the ground over planted frames (zero where below), m.
    #[must_use]
    pub fn mean_float(&self) -> f32 {
        #[allow(clippy::cast_precision_loss)]
        if self.grounded_frames == 0 { 0.0 } else { self.float_sum / self.grounded_frames as f32 }
    }

    /// Whether the foot spent `seconds` or longer without planting or in
    /// one stance (meaningful while the body moves).
    #[must_use]
    pub fn stuck(&self, seconds: f32) -> bool {
        self.longest_swing >= seconds || self.longest_stance >= seconds
    }
}

/// Accumulates [`FootMetrics`] for one foot, frame by frame.
#[derive(Clone, Debug)]
pub struct FootMeter {
    rules: MeterRules,
    metrics: FootMetrics,
    last: Option<FootFrame>,
    /// Time in the current stance or swing.
    span: f32,
    stance_slide: f32,
}

impl FootMeter {
    #[must_use]
    pub fn new(rules: MeterRules) -> Self {
        Self { rules, metrics: FootMetrics::default(), last: None, span: 0.0, stance_slide: 0.0 }
    }

    /// Adds one frame, `dt` seconds after the previous one.
    pub fn push(&mut self, frame: FootFrame, dt: f32) {
        let planted = frame.contact >= self.rules.planted;
        let was = self.last.map(|l| l.contact >= self.rules.planted);
        let m = &mut self.metrics;
        if was.is_some() {
            m.seconds += dt;
        }
        if was == Some(planted) {
            self.span += dt;
            if planted {
                m.stance_seconds += dt;
                if self.span > self.rules.settle {
                    let d = frame.position - self.last.map_or(frame.position, |l| l.position);
                    let d = d.with_y(0.0).length();
                    m.slide += d;
                    self.stance_slide += d;
                    m.max_stance_slide = m.max_stance_slide.max(self.stance_slide);
                }
            }
        } else {
            // A new stance or swing (or the first frame).
            self.span = 0.0;
            self.stance_slide = 0.0;
            m.stances += u32::from(planted);
        }
        if planted {
            m.longest_stance = m.longest_stance.max(self.span);
            if let Some(g) = frame.ground {
                let h = frame.position.y - g - self.rules.sole;
                m.grounded_frames += 1;
                m.max_penetration = m.max_penetration.max(-h);
                m.penetration_sum += (-h).max(0.0);
                m.max_float = m.max_float.max(h);
                m.float_sum += h.max(0.0);
            }
        } else {
            m.longest_swing = m.longest_swing.max(self.span);
        }
        self.last = Some(frame);
    }

    /// Totals so far.
    #[must_use]
    pub fn metrics(&self) -> FootMetrics {
        self.metrics
    }
}

/// [`FootMeter`] over a whole recording of one foot at a fixed `dt`.
#[must_use]
pub fn measure(frames: &[FootFrame], dt: f32, rules: MeterRules) -> FootMetrics {
    let mut meter = FootMeter::new(rules);
    for &f in frames {
        meter.push(f, dt);
    }
    meter.metrics()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DT: f32 = 1.0 / 60.0;
    const SOLE: f32 = 0.08;

    /// One foot of a biped walking +Z at `speed` with a 1 s cycle, planted
    /// for the cycle's first 60 % (offset by `phase`). While planted it
    /// drifts forward at `skate` m/s and sits `sink` m into the ground; a
    /// perfect cycle has both at zero.
    fn foot(seconds: f32, phase: f32, speed: f32, skate: f32, sink: f32) -> Vec<FootFrame> {
        let stride = speed; // 1 s cycle
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let n = (seconds / DT).round() as usize;
        (0..n)
            .map(|k| {
                #[allow(clippy::cast_precision_loss)]
                let t = k as f32 * DT + phase;
                let (cycle, u) = (t.floor(), t.fract());
                // Plant point of this cycle; the swing moves on to the next.
                let plant = cycle * stride;
                let (z, y, contact) = if u < 0.6 {
                    (plant + skate * u, SOLE - sink, 1.0)
                } else {
                    let s = (u - 0.6) / 0.4;
                    let from = plant + skate * 0.6;
                    (from + (plant + stride - from) * s, SOLE + 0.1 * (s * std::f32::consts::PI).sin(), 0.0)
                };
                FootFrame { position: Vec3::new(0.1, y, z), contact, ground: Some(0.0) }
            })
            .collect()
    }

    fn rules() -> MeterRules {
        MeterRules { sole: SOLE, ..MeterRules::default() }
    }

    #[test]
    fn planted_cycle_does_not_slide() {
        for phase in [0.0, 0.5] {
            let m = measure(&foot(5.0, phase, 1.5, 0.0, 0.0), DT, rules());
            assert!((5..=6).contains(&m.stances), "{m:?}");
            assert!(m.slide < 1e-4, "{m:?}");
            assert!(m.max_penetration < 1e-4 && m.max_float < 1e-4, "{m:?}");
            assert!((m.longest_stance - 0.6).abs() < 2.0 * DT && (m.longest_swing - 0.4).abs() < 2.0 * DT, "{m:?}");
            assert!(!m.stuck(2.0));
        }
    }

    #[test]
    fn skating_cycle_slides_at_its_drift() {
        let m = measure(&foot(5.0, 0.0, 1.5, 0.2, 0.0), DT, rules());
        // 0.2 m/s over each stance after the 0.1 s settle: 0.2 × 0.5 m per stance.
        assert!((m.slide_per_stance() - 0.1).abs() < 0.01, "{m:?}");
        assert!((m.slide_speed() - 0.2 * 0.5 / 0.6).abs() < 0.02, "{m:?}");
        assert!((m.max_stance_slide - 0.1).abs() < 0.01, "{m:?}");
    }

    #[test]
    fn sunk_foot_reports_its_depth() {
        let m = measure(&foot(3.0, 0.0, 1.5, 0.0, 0.03), DT, rules());
        assert!((m.max_penetration - 0.03).abs() < 1e-4 && (m.mean_penetration() - 0.03).abs() < 1e-4, "{m:?}");
        assert!(m.max_float == 0.0 && m.mean_float() == 0.0, "{m:?}");
        let m = measure(&foot(3.0, 0.0, 1.5, 0.0, -0.05), DT, rules());
        assert!((m.mean_float() - 0.05).abs() < 1e-4 && m.mean_penetration() == 0.0, "{m:?}");
    }

    #[test]
    fn legs_that_never_plant_or_never_lift_are_stuck() {
        let mut never = foot(3.0, 0.0, 1.5, 0.0, 0.0);
        for f in &mut never {
            f.contact = 0.0;
        }
        let m = measure(&never, DT, rules());
        assert_eq!(m.stances, 0);
        assert!(m.stuck(2.0) && m.longest_swing > 2.9, "{m:?}");
        let mut always = foot(3.0, 0.0, 1.5, 0.0, 0.0);
        for f in &mut always {
            f.contact = 1.0;
        }
        let m = measure(&always, DT, rules());
        assert_eq!(m.stances, 1);
        // The swing frames now count: the "planted" foot travels with the body.
        assert!(m.stuck(2.0) && m.slide > 3.0, "{m:?}");
    }
}
