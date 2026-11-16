//! Morrowind's sky bodies on the framework's night-sky model (`sf_sky` `Moon`, `StarField`,
//! `SunDiskLook`; spec: weather-adapter.md "Moons, stars and sun disk"): Masser and Secunda from
//! the `[Moons]` section of `Morrowind.ini`, their phase textures from the VFS, and the sun's
//! drawn size.
//!
//! The moons follow `OpenMW`'s `MoonModel` ([`mw_weather::moons`], a port of
//! `apps/openmw/mwworld/weather.cpp` at commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`): each
//! is an [`MoonPath::Driven`] moon whose driver maps the game clock to `OpenMW`'s game day and hour
//! and hands the framework the moon's direction (`Moon::setState`), phase, opacity
//! (`earlyMoonShadowAlpha * hourlyAlpha`: hidden near the horizon and by day) and texture blend
//! (`shadowBlend`: a plain star-hiding disk low down, the textured moon higher up; the night pass
//! draws it like `OpenMW`'s `paintMoon`). The drawn size is `OpenMW`'s moon quad
//! ([`mw_weather::moons::quad_angular_size`]: Masser 20.8 deg, Secunda 9.0 deg at the vanilla
//! sizes 94 and 40).
//!
//! NS-3 (Secunda never visible), the cause: the earlier invented circuit (`MoonPath::Circuit`,
//! staggered `hour_angle_at_epoch` -30 deg, tilt 50, highest 40 deg) put Secunda at 4.5 deg in the
//! east at 21:30 on day 0 and day 168 (`ns-night-clear`, `ns-moons-full`), below the horizon at
//! 20:30 (`ns-dusk-stars`) and at 18 deg at 23:00 (`ro16-terrain-veil --hour 23`), behind
//! Balmora's valley sides each time; its texture, alpha and disk radiance were fine (radiance per
//! texel 1.4x Masser's). `old_circuit_put_secunda_at_the_horizon` in the tests prints them.

use mw_install::MorrowindIni;
use mw_weather::moons::{MoonModel, MoonName, game_day_hour, quad_angular_size};
use sf_sky_bevy::sf_sky::{Moon, MoonDrive, MoonDriver, MoonPath, SkyClock, SkySettings};

/// Drawn degrees per unit of `Masser Size` / `Secunda Size` of the earlier, smaller disks, at
/// which [`MoonLook::illuminance`] was set: the disks keep that radiance (nits per texel) at
/// `OpenMW`'s larger size, so the larger disk is not dimmer.
const DEG_PER_SIZE: f32 = 0.1;

/// The light and tint of one moon (not in the ini).
struct MoonLook {
    /// Full-moon illuminance (lux) over a disk of `Size * DEG_PER_SIZE` degrees; scaled to the
    /// drawn size. Masser's share of the moonlight is its part of the sum.
    illuminance: f32,
    tint: [f32; 3],
}

/// Masser, then Secunda ([`MoonName::ALL`]; Secunda draws over Masser, as in `OpenMW`).
const MOONS: [MoonLook; 2] = [
    MoonLook { illuminance: 0.8, tint: [1.0, 0.9, 0.82] },
    MoonLook { illuminance: 0.2, tint: [0.9, 0.93, 1.0] },
];

/// The driver of one moon: the game clock's days to `OpenMW`'s day and hour, then the model.
pub fn driver(model: MoonModel) -> MoonDriver {
    MoonDriver::new(move |clock: &SkyClock| {
        let (day, hour) = game_day_hour(clock.days);
        let state = model.state(day, hour);
        MoonDrive {
            direction_enu: state.direction(),
            phase: state.phase.cycle_fraction(),
            alpha: state.alpha,
            texture_blend: state.shadow_blend,
        }
    })
}

/// Masser, Secunda, stars and the sun disk on `settings`, with the vanilla `[Moons]` values.
pub fn apply(settings: &mut SkySettings) {
    apply_with(settings, None);
}

/// [`apply`] with the player's `Morrowind.ini` (each key falls back to vanilla).
pub fn apply_with(settings: &mut SkySettings, ini: Option<&MorrowindIni>) {
    settings.moons = MoonName::ALL
        .iter()
        .zip(&MOONS)
        .map(|(&name, look)| {
            let model = MoonModel::from_ini(name, ini);
            let size = model.settings().size;
            let angular_size = quad_angular_size(size);
            // Same radiance as the earlier disk: illuminance over the drawn solid angle (pi r^2).
            let earlier = (size * DEG_PER_SIZE).to_radians();
            let scale = if earlier > 0.0 { (angular_size / earlier).powi(2) } else { 1.0 };
            Moon {
                name: name.texture_name().into(),
                angular_size,
                phase_period_days: 0.0,
                phase_at_epoch: 0.5,
                phase_steps: 8,
                path: MoonPath::Driven(driver(model)),
                illuminance: look.illuminance * scale,
                tint: look.tint,
                textures: name.phase_textures(),
                // The textures leave a margin around the disk.
                occluder: 0.85,
            }
        })
        .collect();
    // Morrowind's sun reads large; `Glare View` 1 (clear) is the default glare.
    settings.sun_disk.angular_size = 2.5_f32.to_radians();
}

#[cfg(test)]
#[path = "night_sky_tests.rs"]
mod tests;
