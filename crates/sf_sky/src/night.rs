//! The drawn sky bodies: any number of moons, a star field, and the sun's
//! drawn disk and glare (spec: `docs/specs/sky-lighting.md`, "Night sky").
//!
//! Lighting stays physical: each moon reflects sunlight by the Lambert
//! phase law of a diffuse sphere, the atmosphere attenuates it, and one
//! moonlight carries the sum toward the brightest visible moon. What is
//! *drawn* is artistic where games usually are: the sun disk's angular size
//! and glare (UE `SkyAtmosphere` disk scale, RDR2's large sun) and moon
//! sizes, so a world can have a huge moon without a huge light.

use std::f64::consts::{PI, TAU};

use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::celestial::{Orbit, SkyPosition};
use crate::clock::{Calendar, SkyClock};
use crate::curve::KeyCurve;

/// The most moons a sky draws at once (GPU binding slots).
pub const MAX_MOONS: usize = 4;

/// How a moon crosses the sky.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum MoonPath {
    /// Physical: an orbit near the ecliptic whose position follows its
    /// phase (new moon next to the sun, full moon opposite, transiting at
    /// midnight). Uses the world's latitude and axial tilt.
    Synodic,
    /// Its own circuit, independent of the phase (fantasy skies). The moon
    /// moves along a circle of the sky like a star on the celestial sphere
    /// of a world whose latitude is `tilt_deg`: rising in the east, highest
    /// in the south (north of the equator) and setting in the west.
    Circuit {
        /// Days for one full circuit relative to the ground. A bit over 1
        /// makes the moon rise later each day (Earth's moon: 1.035).
        circuit_days: f32,
        /// Hour angle at day 0 in degrees: 0 is highest at day 0, midnight.
        hour_angle_at_epoch_deg: f32,
        /// Tilt of the path from the zenith in degrees (0 passes overhead).
        tilt_deg: f32,
        /// Offset of the path north (+) or south of its great circle, degrees.
        declination_deg: f32,
    },
}

/// One moon: orbit, phase cycle, look and light.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Moon {
    /// For logs and settings.
    pub name: String,
    /// Drawn disk angular diameter, radians.
    pub angular_size: f32,
    /// Phase cycle in days (new to new). 0 holds the phase at
    /// `phase_at_epoch`.
    pub phase_period_days: f32,
    /// Phase at day 0, in `[0, 1)`: 0 new, 0.25 first quarter, 0.5 full.
    pub phase_at_epoch: f32,
    /// Discrete phases per cycle (e.g. one texture per phase), 0 continuous.
    /// The phase snaps to the nearest step.
    pub phase_steps: u32,
    pub path: MoonPath,
    /// Illuminance of the full moon above the atmosphere, lux. Earth's is
    /// ≈ 0.25; games exaggerate it so nights stay readable.
    pub illuminance: f32,
    /// Linear RGB tint of the light and the disk.
    pub tint: [f32; 3],
    /// Disk textures, one per phase step: frame `k` shows phase
    /// `k / textures.len()` (frame 0 new). Asset paths, resolved by the
    /// renderer. Empty: a procedurally shaded sphere.
    pub textures: Vec<String>,
    /// Radius of the star-hiding disk as a fraction of the drawn disk (the
    /// unlit side hides stars too). Below 1 when the textures have a margin.
    pub occluder: f32,
}

impl Default for Moon {
    fn default() -> Self {
        Self {
            name: "moon".into(),
            angular_size: 0.009_04,
            phase_period_days: 29.530_59,
            phase_at_epoch: 0.0,
            phase_steps: 0,
            path: MoonPath::Synodic,
            illuminance: 1.0,
            tint: [0.9, 0.93, 1.0],
            textures: Vec::new(),
            occluder: 1.0,
        }
    }
}

/// One moon at one instant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MoonState {
    pub position: SkyPosition,
    /// Phase in `[0, 1)` after snapping to `phase_steps`.
    pub phase: f32,
    /// Lit fraction of the disk.
    pub illuminated: f32,
    /// Brightness relative to full (Lambert phase law).
    pub brightness: f32,
    /// Texture frame for this phase, if the moon has textures.
    pub frame: Option<u32>,
    /// Illuminance above the atmosphere after phase and weather, lux.
    pub illuminance: f32,
    /// Full-moon illuminance above the atmosphere after weather, lux (what
    /// a phase texture is scaled by: it draws the phase itself).
    pub full_illuminance: f32,
    /// Atmospheric transmittance times the visible part of the disk.
    pub transmittance: Vec3,
    /// Drawn disk angular diameter, radians.
    pub angular_size: f32,
    pub tint: Vec3,
    pub occluder: f32,
}

impl MoonState {
    /// Illuminance reaching the ground, lux.
    pub fn ground_illuminance(&self) -> f32 {
        self.illuminance * crate::luminance(self.transmittance)
    }
}

/// Lit fraction and Lambert-sphere brightness (relative to full) at `phase`.
pub fn phase_light(phase: f64) -> (f32, f32) {
    // Phase angle (sun-moon-observer): π at new moon, 0 at full.
    let phase_angle = (PI - TAU * phase).abs();
    let illuminated = f64::midpoint(1.0, phase_angle.cos());
    let brightness = ((PI - phase_angle) * phase_angle.cos() + phase_angle.sin()) / PI;
    #[expect(clippy::cast_possible_truncation, reason = "fractions")]
    (illuminated.clamp(0.0, 1.0) as f32, brightness.clamp(0.0, 1.0) as f32)
}

impl Moon {
    /// Phase at `clock`, snapped to `phase_steps`, and the texture frame.
    pub fn phase(&self, clock: &SkyClock) -> (f64, Option<u32>) {
        let mut phase = if self.phase_period_days > 0.0 {
            clock.days / f64::from(self.phase_period_days) + f64::from(self.phase_at_epoch)
        } else {
            f64::from(self.phase_at_epoch)
        }
        .rem_euclid(1.0);
        if self.phase_steps > 0 {
            let steps = f64::from(self.phase_steps);
            phase = ((phase * steps).round() / steps).rem_euclid(1.0);
        }
        let frame = (!self.textures.is_empty()).then(|| {
            #[expect(clippy::cast_precision_loss, reason = "a handful of frames")]
            let n = self.textures.len() as f64;
            #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "in [0, n)")]
            let k = ((phase * n).round() as u32) % self.textures.len() as u32;
            k
        });
        (phase, frame)
    }

    /// Where the moon is at `clock`, with its phase `phase`.
    pub fn position(&self, orbit: &Orbit, clock: &SkyClock, calendar: &Calendar, phase: f64) -> SkyPosition {
        match self.path {
            MoonPath::Synodic => orbit.synodic_moon(clock, calendar, phase),
            MoonPath::Circuit { circuit_days, hour_angle_at_epoch_deg, tilt_deg, declination_deg } => {
                let turns = if circuit_days > 0.0 { clock.days / f64::from(circuit_days) } else { 0.0 };
                let hour_angle = TAU * turns.fract() + f64::from(hour_angle_at_epoch_deg).to_radians();
                orbit.place(
                    f64::from(declination_deg).to_radians(),
                    hour_angle,
                    f64::from(tilt_deg).to_radians(),
                )
            }
        }
    }
}

/// A star field drawn behind everything in the sky.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StarField {
    pub enabled: bool,
    /// Illuminance of the brightest stars, lux (Sirius ≈ 1e-5). The renderer
    /// spreads it over about a pixel, so stars keep their brightness at any
    /// resolution.
    pub brightest_lux: f32,
    /// Stars per sky cell side (procedural field resolution); more is denser.
    pub density: f32,
    /// Visibility in `[0, 1]` against sun elevation (degrees): an artistic
    /// fade on top of the physical one (the sky's own brightness and the
    /// atmosphere's extinction already hide stars at dusk and low down).
    pub visibility: KeyCurve,
    /// Scintillation strength near the horizon, `[0, 1]`.
    pub twinkle: f32,
}

impl Default for StarField {
    fn default() -> Self {
        Self {
            enabled: true,
            brightest_lux: 2.0e-5,
            density: 1.0,
            visibility: KeyCurve::new(vec![(-14.0, 1.0), (-4.0, 0.0)]),
            twinkle: 0.3,
        }
    }
}

/// How the sun disk is drawn. The light itself keeps the physical size
/// ([`crate::SkySettings::sun_angular_size`]: horizon fade, soft shadows).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SunDiskLook {
    /// Drawn disk angular diameter, radians. Games draw the sun several
    /// times its real 0.53° (UE's default `SkyAtmosphere` disk is the
    /// physical one; most shipped games scale it up by 2-5x).
    pub angular_size: f32,
    /// Disk brightness multiplier. 1 keeps the disk's total light equal to
    /// the sun's (radiance drops as the disk grows; still far above white,
    /// so it saturates and blooms).
    pub intensity: f32,
    /// Glare: the share of the sun's light spread into a halo around the
    /// disk (aureole and lens glare). 0 turns it off.
    pub glare: f32,
    /// Angular radius at which the glare falls to 1/e, radians.
    pub glare_size: f32,
}

impl Default for SunDiskLook {
    fn default() -> Self {
        Self {
            angular_size: 2.0_f32.to_radians(),
            intensity: 1.0,
            glare: 1.2e-3,
            glare_size: 1.5_f32.to_radians(),
        }
    }
}

/// The night sky at one instant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NightSky {
    pub moons: [MoonState; MAX_MOONS],
    pub moon_count: u32,
    /// Star visibility in `[0, 1]` (artistic fade times weather).
    pub stars: f32,
}

impl NightSky {
    /// The moons in this state.
    pub fn moons(&self) -> &[MoonState] {
        &self.moons[..(self.moon_count as usize).min(MAX_MOONS)]
    }
}
