//! Distance attenuation (audio.md "Positional"): full volume inside `min_m`, silent from `max_m`.
//! Like Wwise/FMOD/UE attenuation settings, the curve is ours and the backend only pans: the
//! Bevy plugin scales the backend's built-in inverse-square law out of the audible range.

/// Curve shape between `min_m` and `max_m`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Rolloff {
    /// Physical `min / d` shape (-6 dB per doubling), offset and rescaled so it reaches exactly 0
    /// at `max_m` instead of popping out at the cull edge. The default for world sounds.
    #[default]
    Inverse,
    /// Straight line from 1 at `min_m` to 0 at `max_m` (big ambient beds, designer-friendly).
    Linear,
}

/// Per-sound distance attenuation in metres.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Attenuation {
    pub min_m: f32,
    pub max_m: f32,
    pub rolloff: Rolloff,
}

impl Default for Attenuation {
    /// A generic world sound: full within 1 m, gone by 40 m.
    fn default() -> Self {
        Self { min_m: 1.0, max_m: 40.0, rolloff: Rolloff::Inverse }
    }
}

impl Attenuation {
    pub const fn new(min_m: f32, max_m: f32) -> Self {
        Self { min_m, max_m, rolloff: Rolloff::Inverse }
    }

    /// Gain (0..1) at `distance_m` from the listener. `max_m <= min_m` acts as a hard edge.
    pub fn gain(&self, distance_m: f32) -> f32 {
        let min = self.min_m.max(1e-3);
        let d = distance_m.max(0.0);
        if d <= min {
            return 1.0;
        }
        if d >= self.max_m {
            return 0.0;
        }
        match self.rolloff {
            Rolloff::Linear => 1.0 - (d - min) / (self.max_m - min),
            Rolloff::Inverse => {
                let floor = min / self.max_m;
                ((min / d - floor) / (1.0 - floor)).clamp(0.0, 1.0)
            }
        }
    }

    /// Whether a sound at `distance_m` is worth starting at all (cull before playing).
    pub fn audible(&self, distance_m: f32) -> bool {
        distance_m < self.max_m.max(self.min_m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_inside_min_silent_from_max() {
        for rolloff in [Rolloff::Inverse, Rolloff::Linear] {
            let a = Attenuation { min_m: 2.0, max_m: 20.0, rolloff };
            assert!((a.gain(0.0) - 1.0).abs() < 1e-6);
            assert!((a.gain(2.0) - 1.0).abs() < 1e-6);
            assert!(a.gain(20.0).abs() < 1e-6);
            assert!(a.gain(50.0).abs() < 1e-6);
            assert!(a.audible(19.9) && !a.audible(20.0));
            // Monotonic.
            let mut last = 1.0;
            for i in 0..=40 {
                #[expect(clippy::cast_precision_loss, reason = "small")]
                let g = a.gain(i as f32 * 0.5);
                assert!(g <= last + 1e-6, "{rolloff:?} at {i}");
                last = g;
            }
        }
    }

    #[test]
    fn inverse_falls_faster_near_than_linear() {
        let inv = Attenuation::new(1.0, 30.0);
        let lin = Attenuation { rolloff: Rolloff::Linear, ..inv };
        // At 2 m the inverse curve has lost about half, the line only a few percent.
        assert!((0.45..0.55).contains(&inv.gain(2.0)), "{}", inv.gain(2.0));
        assert!(lin.gain(2.0) > 0.95);
    }

    #[test]
    fn degenerate_range_is_a_hard_edge() {
        let a = Attenuation::new(5.0, 5.0);
        assert!((a.gain(4.0) - 1.0).abs() < 1e-6);
        assert!(a.gain(6.0).abs() < 1e-6);
    }
}
