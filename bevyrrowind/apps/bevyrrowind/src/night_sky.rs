//! Morrowind's sky bodies on the framework's night-sky model (`sf_sky`
//! `Moon`, `StarField`, `SunDiskLook`; spec: weather-adapter.md "Moons, stars
//! and sun disk"): Masser and Secunda from the `[Moons]` section of
//! Morrowind.ini (the shipped defaults below; the adapter reads the ini
//! later, like the cloud speeds), their phase textures from the VFS, and the
//! sun's drawn size.
//!
//! Behavior (black-box, not `OpenMW` code): both moons rise in the east and
//! set in the west, a little later each day (`Daily Increment`, hours), on
//! paths tilted by `Axis Offset`; they share one 8-phase cycle of 3 days per
//! phase (24 days), starting full. Morrowind hides them by day with fixed
//! fade hours; here the day sky's own brightness does (not a 1:1 port).

use sf_sky_bevy::sf_sky::{Moon, MoonPath, SkySettings};

/// Drawn degrees per unit of `Masser Size` / `Secunda Size` (calibrated by
/// eye against the original: Masser at 94 is several times the sun).
const DEG_PER_SIZE: f32 = 0.1;
/// Days per phase step (UESP: 8 phases, a 24-day cycle).
const DAYS_PER_PHASE: f32 = 3.0;
/// Phase textures, frame k = phase k/8 from new.
const PHASES: [&str; 8] = ["new", "one_wax", "half_wax", "three_wax", "full", "three_wan", "half_wan", "one_wan"];

/// One `[Moons]` entry (Morrowind.ini defaults).
struct MoonIni {
    name: &'static str,
    size: f32,
    axis_offset: f32,
    daily_increment: f32,
    /// Hour angle at day 0 (degrees): where the circuit starts. Not in the
    /// ini; staggered so the moons don't start in conjunction.
    hour_angle_at_epoch: f32,
    illuminance: f32,
    tint: [f32; 3],
}

const MOONS: [MoonIni; 2] = [
    MoonIni {
        name: "masser",
        size: 94.0,
        axis_offset: 35.0,
        daily_increment: 1.0,
        hour_angle_at_epoch: 0.0,
        illuminance: 0.8,
        tint: [1.0, 0.9, 0.82],
    },
    MoonIni {
        name: "secunda",
        size: 40.0,
        axis_offset: 50.0,
        daily_increment: 1.2,
        hour_angle_at_epoch: -30.0,
        illuminance: 0.2,
        tint: [0.9, 0.93, 1.0],
    },
];

/// Masser, Secunda, stars and the sun disk on `settings`.
pub(crate) fn apply(settings: &mut SkySettings) {
    let cycle = DAYS_PER_PHASE * 8.0;
    settings.moons = MOONS
        .iter()
        .map(|m| Moon {
            name: m.name.into(),
            angular_size: (m.size * DEG_PER_SIZE).to_radians(),
            phase_period_days: cycle,
            // Both start full (observed on new games and loads).
            phase_at_epoch: 0.5,
            phase_steps: 8,
            path: MoonPath::Circuit {
                circuit_days: 1.0 + m.daily_increment / 24.0,
                hour_angle_at_epoch_deg: m.hour_angle_at_epoch,
                tilt_deg: m.axis_offset,
                declination_deg: 0.0,
            },
            illuminance: m.illuminance,
            tint: m.tint,
            textures: PHASES.iter().map(|p| format!("textures/tx_{}_{p}.dds", m.name)).collect(),
            // The textures leave a margin around the disk.
            occluder: 0.85,
        })
        .collect();
    // Morrowind's sun reads large; `Glare View` 1 (clear) is the default glare.
    settings.sun_disk.angular_size = 2.5_f32.to_radians();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masser_is_bigger_and_both_have_eight_phases() {
        let mut s = SkySettings::default();
        apply(&mut s);
        assert_eq!(s.moons.len(), 2);
        assert!(s.moons[0].angular_size > 2.0 * s.moons[1].angular_size);
        assert!(s.moons.iter().all(|m| m.textures.len() == 8 && m.phase_steps == 8));
        assert_eq!(s.moons[0].textures[4], "textures/tx_masser_full.dds");
    }
}
