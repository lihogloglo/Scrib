//! Precipitation, lightning and sound cues from the frame (weather.md "WE3
//! as built (part 2b)").
//!
//! - [`apply_precip`]: `frame.precip` (up to [`sf_precip_bevy::SLOTS`]
//!   kinds, each kept in its slot while it falls so a crossfade never
//!   swaps boxes) and the ground wind into [`Precip`]; brightness from the
//!   sun's elevation, the overcast dimming and the lightning flash.
//! - [`apply_lightning`]: each strike restarts [`LightningFlash`]; the
//!   flash ([`LightningStrike::flash_at`], pure) drives a shadowless
//!   directional light from the strike's azimuth and the clouds' ambient
//!   (an emission stand-in) and queues the thunder `distance / 343` s later.
//! - [`emit_cues`]: [`WeatherSound`] messages: loop volumes on start, stop
//!   and change, thunder one-shots when due. Playing them is the host's
//!   (no audio backend in the workspace yet).

use bevy_camera::visibility::Visibility;
use bevy_ecs::prelude::*;
use bevy_light::DirectionalLight;
use bevy_math::{Dir3, Vec3};
use bevy_platform::time::Instant;
use bevy_time::{Real, Time};
use bevy_transform::components::Transform;
use sf_diag_bevy::DiagCounters;
use sf_precip_bevy::{Precip, PrecipShape, PrecipSlot, SLOTS};
use sf_records::RecordId;
use sf_weather::{LightningStrike, PrecipIx, PrecipKind, PrecipLook};

use crate::{CurrentWeather, WeatherRecords};

/// Lightning light at flash 1, lux (a near strike lights the land like a
/// cloudy day for an instant).
pub const LIGHTNING_LUX: f32 = 40_000.0;
/// Cloud ambient gain at flash 1 (the clouds light up from inside).
pub const LIGHTNING_CLOUD_GAIN: f32 = 6.0;
/// Strike elevation seen from the camera, radians (the flash comes from the
/// cloud base, not the horizon).
pub const LIGHTNING_ELEVATION: f32 = 0.95;
/// Loop volume change that sends a new [`WeatherSound::Loop`].
pub const CUE_STEP: f32 = 0.02;

/// A weather sound for the host's audio (messages; the weather never plays
/// sound itself).
#[derive(Message, Clone, Debug, PartialEq)]
pub enum WeatherSound {
    /// A loop's target volume; 0 stops it.
    Loop { sound: RecordId, volume: f32, region: bool },
    /// A one-shot (thunder) from `azimuth` (radians from +X toward +Z) at
    /// `distance_m`.
    OneShot { sound: RecordId, volume: f32, azimuth: f32, distance_m: f32 },
}

/// The latest strike and its flash now.
#[derive(Resource, Clone, Debug, Default)]
pub struct LightningFlash {
    pub strike: Option<LightningStrike>,
    /// Real seconds since the strike.
    pub age_s: f32,
    /// This frame's flash (0 = none).
    pub level: f32,
}

/// The lightning's directional light (spawned on the first strike).
#[derive(Component)]
pub struct LightningLight;

/// Thunder waiting for its sound to arrive: (due, real s; strike).
#[derive(Default)]
pub struct PendingThunder(Vec<(f64, LightningStrike)>);

fn shape(look: PrecipLook) -> (PrecipShape, f32, f32) {
    // (shape, flutter m, wind share): flakes ride the wind, rain mostly falls.
    match look {
        PrecipLook::Streak => (PrecipShape::Streak, 0.0, 0.8),
        PrecipLook::Flake => (PrecipShape::Flake, 0.15, 1.0),
        // Blowing grit: wind streaks (owner: ash storms need visible blowing ash).
        PrecipLook::Dust => (PrecipShape::Streak, 0.3, 1.0),
    }
}

/// The drawn slot of a precipitation kind at `intensity`.
pub fn precip_slot(kind: &PrecipKind, intensity: f32) -> PrecipSlot {
    let (shape, flutter, wind_follow) = shape(kind.look);
    PrecipSlot {
        shape,
        intensity: intensity.clamp(0.0, 1.0),
        fall_speed: kind.fall_speed,
        size: kind.size,
        color: kind.tint.into(),
        flutter,
        wind_follow,
    }
}

/// Light on the drops: night floor to day by the sun's elevation (radians),
/// dimmed by overcast, plus the lightning flash.
pub fn precip_brightness(sun_elevation: f32, overcast_dimming: f32, flash: f32) -> f32 {
    let t = ((sun_elevation + 0.1) / 0.3).clamp(0.0, 1.0);
    let day = t * t * (3.0 - 2.0 * t);
    (0.04 + 0.96 * day) * (1.0 - 0.5 * overcast_dimming.clamp(0.0, 1.0)) + 1.5 * flash
}

/// Kinds into slots: a kind keeps its slot while it falls; new kinds take
/// free slots, the strongest first.
pub fn assign_slots(assigned: &mut [Option<PrecipIx>; SLOTS], falling: &[(PrecipIx, f32)]) {
    for a in assigned.iter_mut() {
        if a.is_some_and(|k| !falling.iter().any(|(f, i)| *f == k && *i > 0.0)) {
            *a = None;
        }
    }
    let mut order: Vec<_> = falling.iter().filter(|(_, i)| *i > 0.0).collect();
    order.sort_by(|a, b| b.1.total_cmp(&a.1));
    for (k, _) in order {
        if assigned.contains(&Some(*k)) {
            continue;
        }
        if let Some(free) = assigned.iter_mut().find(|a| a.is_none()) {
            *free = Some(*k);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn apply_precip(
    current: Res<CurrentWeather>,
    records: Res<WeatherRecords>,
    flash: Res<LightningFlash>,
    sky: Option<Res<sf_sky_bevy::CurrentSky>>,
    precip: Option<ResMut<Precip>>,
    mut assigned: Local<[Option<PrecipIx>; SLOTS]>,
) {
    let Some(mut precip) = precip else { return };
    if !current.valid {
        return;
    }
    let frame = &current.frame;
    let falling: Vec<(PrecipIx, f32)> = frame.precip.iter().map(|p| (p.kind, p.intensity)).collect();
    assign_slots(&mut assigned, &falling);
    let mut slots = [None; SLOTS];
    for (slot, kind) in slots.iter_mut().zip(assigned.iter()) {
        *slot = kind.and_then(|k| {
            let intensity = falling.iter().find(|(f, _)| *f == k)?.1;
            Some(precip_slot(records.0.precip_kind(k), intensity))
        });
    }
    let elevation = sky.map_or(1.0, |s| s.0.sun.elevation);
    let brightness = precip_brightness(elevation, frame.sky.overcast_dimming, flash.level);
    let wind = frame.wind().velocity;
    if precip.slots != slots || precip.wind != wind || (precip.brightness - brightness).abs() > 1e-3 {
        precip.slots = slots;
        precip.wind = wind;
        precip.brightness = brightness;
    }
}

/// The direction the lightning light shines (from the strike, high up).
pub fn lightning_direction(azimuth: f32) -> Vec3 {
    let (s, c) = azimuth.sin_cos();
    let e = LIGHTNING_ELEVATION;
    -Vec3::new(c * e.cos(), e.sin(), s * e.cos())
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
pub fn apply_lightning(
    mut commands: Commands,
    current: Res<CurrentWeather>,
    real: Option<Res<Time<Real>>>,
    mut flash: ResMut<LightningFlash>,
    mut pending: Local<PendingThunder>,
    mut sounds: MessageWriter<WeatherSound>,
    mut light: Query<(&mut DirectionalLight, &mut Transform, &mut Visibility), With<LightningLight>>,
    clouds: Option<ResMut<sf_atmos_bevy::Clouds>>,
    mut cloud_base: Local<Option<f32>>,
    counters: Option<Res<DiagCounters>>,
) {
    let t0 = Instant::now();
    let (now, dt) = real.map_or((0.0, 0.0), |r| (r.elapsed_secs_f64(), r.delta_secs()));
    flash.age_s += dt;
    if current.valid
        && let Some(s) = current.frame.lightning.last()
    {
        flash.strike = Some(s.clone());
        flash.age_s = 0.0;
        for s in &current.frame.lightning {
            pending.0.push((now + f64::from(s.thunder_delay_s()), s.clone()));
        }
    }
    let level = flash.strike.as_ref().map_or(0.0, |s| s.flash_at(flash.age_s));
    flash.level = if level > 1e-3 { level } else { 0.0 };
    // Thunder.
    pending.0.retain(|(due, s)| {
        if *due > now {
            return true;
        }
        if let Some(sound) = &s.thunder {
            sounds.write(WeatherSound::OneShot {
                sound: sound.clone(),
                volume: s.thunder_volume(),
                azimuth: s.azimuth,
                distance_m: s.distance_m,
            });
        }
        false
    });
    // The light.
    let on = flash.level > 0.0;
    if let Ok((mut l, mut tf, mut vis)) = light.single_mut() {
        let lux = LIGHTNING_LUX * flash.level;
        if (l.illuminance - lux).abs() > 1e-2 {
            l.illuminance = lux;
        }
        if on && let Some(s) = &flash.strike {
            let to = Dir3::new(lightning_direction(s.azimuth)).unwrap_or(Dir3::NEG_Y);
            let want = Transform::IDENTITY.looking_to(to, Vec3::Y);
            if *tf != want {
                *tf = want;
            }
        }
        let want = if on { Visibility::Inherited } else { Visibility::Hidden };
        if *vis != want {
            *vis = want;
        }
    } else if flash.strike.is_some() {
        commands.spawn((
            LightningLight,
            DirectionalLight {
                illuminance: 0.0,
                shadow_maps_enabled: false,
                ..DirectionalLight::default()
            },
            Transform::IDENTITY,
            Visibility::Hidden,
            Name::new("weather/lightning"),
        ));
    }
    // Cloud glow: the look's ambient raised during the flash, restored after.
    if let Some(mut clouds) = clouds {
        if on {
            let base = *cloud_base.get_or_insert(clouds.settings.look.ambient);
            clouds.settings.look.ambient = base * (1.0 + LIGHTNING_CLOUD_GAIN * flash.level);
        } else if let Some(base) = cloud_base.take() {
            clouds.settings.look.ambient = base;
        }
    }
    if let Some(c) = counters {
        c.add_named("weather/fx/ms", sf_diag_bevy::sf_diag::unit::MS, t0.elapsed().as_secs_f64() * 1000.0);
    }
}

/// Loop cues: a message when one starts, stops or moves by [`CUE_STEP`].
#[allow(clippy::needless_pass_by_value)]
pub fn emit_cues(current: Res<CurrentWeather>, mut sent: Local<Vec<(RecordId, f32)>>, mut sounds: MessageWriter<WeatherSound>) {
    if !current.valid {
        return;
    }
    let cues = &current.frame.cues;
    sent.retain(|(sound, _)| {
        let keep = cues.iter().any(|c| c.sound == *sound && c.volume > 0.0);
        if !keep {
            sounds.write(WeatherSound::Loop { sound: sound.clone(), volume: 0.0, region: false });
        }
        keep
    });
    for c in cues.iter().filter(|c| c.volume > 0.0) {
        match sent.iter_mut().find(|(s, _)| *s == c.sound) {
            Some((_, v)) if (*v - c.volume).abs() < CUE_STEP => {}
            Some((_, v)) => {
                *v = c.volume;
                sounds.write(WeatherSound::Loop { sound: c.sound.clone(), volume: c.volume, region: c.region });
            }
            None => {
                sent.push((c.sound.clone(), c.volume));
                sounds.write(WeatherSound::Loop { sound: c.sound.clone(), volume: c.volume, region: c.region });
            }
        }
    }
}
