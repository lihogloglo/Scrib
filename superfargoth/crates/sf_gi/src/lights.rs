//! The GI light list (`docs/specs/gi.md` "Scene and lights > Light list"): point and spot lights
//! shaded at probe-ray hits with the raster shader's units and falloff, the nearest N to the
//! volume, and which of them changed this frame (their probes use a lower hysteresis).
//!
//! Units follow `bevy_pbr` 0.20: `PointLight`/`SpotLight::intensity` is luminous power (lumens),
//! uploaded as luminous intensity `intensity / 4pi` (candela) premultiplied by the linear colour
//! (`render/light.rs`, `color_inverse_square_range`); the falloff is `getDistanceAttenuation` and
//! the spot cone Filament's `saturate(cd * scale + offset)^2` (`render/pbr_lighting.wesl`).
//! `probe_update.wesl` mirrors [`distance_attenuation`], [`spot_attenuation`], [`GiLight::irradiance`]
//! and [`probe_hysteresis`]; [`GiLight::to_gpu`] is its `GiLight` struct.

use std::collections::HashMap;
use std::hash::Hash;

use glam::Vec3;

/// A light moved further than this (metres) since last frame: changed.
pub const MOVED_DISTANCE: f32 = 0.05;
/// A light's power changed by more than this fraction since last frame: changed. Above the
/// small per-frame steps of a candle's flicker, below a lamp toggling.
pub const POWER_CHANGE: f32 = 0.1;

/// Bevy's range falloff: `saturate(1 - (d^2 / r^2)^2)^2` (`pbr_lighting.wesl` `getRangeFalloff`).
pub fn range_falloff(distance_sq: f32, inverse_range_sq: f32) -> f32 {
    let factor = distance_sq * inverse_range_sq;
    let smooth = (1.0 - factor * factor).clamp(0.0, 1.0);
    smooth * smooth
}

/// Bevy's punctual-light attenuation: range falloff over `max(d^2, 1e-4)`
/// (`pbr_lighting.wesl` `getDistanceAttenuation`).
pub fn distance_attenuation(distance_sq: f32, inverse_range_sq: f32) -> f32 {
    range_falloff(distance_sq, inverse_range_sq) / distance_sq.max(0.0001)
}

/// Bevy's spot-cone constants from the inner and outer half-angles (radians; `render/light.rs`):
/// `scale = 1 / max(cos inner - cos outer, 1e-4)`, `offset = -cos outer * scale`. The outer angle
/// is clamped below pi/2 and the inner to the outer, as Bevy does.
pub fn spot_scale_offset(inner: f32, outer: f32) -> (f32, f32) {
    let outer = outer.min(std::f32::consts::FRAC_PI_2 - 1e-4);
    let inner = inner.min(outer);
    let cos_outer = outer.cos();
    let scale = 1.0 / (inner.cos() - cos_outer).max(1e-4);
    (scale, -cos_outer * scale)
}

/// Bevy's spot cone: `saturate(cd * scale + offset)^2`, `cd` = cosine between the spot's
/// direction and the light-to-point direction (`pbr_lighting.wesl` `spot_light`).
pub fn spot_attenuation(cos_angle: f32, scale: f32, offset: f32) -> f32 {
    let a = (cos_angle * scale + offset).clamp(0.0, 1.0);
    a * a
}

/// A spot light's cone: forward direction (unit) and Bevy's [`spot_scale_offset`] constants.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spot {
    pub direction: Vec3,
    pub scale: f32,
    pub offset: f32,
}

/// One light of the list, in the units of Bevy's GPU light.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GiLight {
    pub position: Vec3,
    /// Range cutoff (metres): no light at or past it.
    pub range: f32,
    /// Linear colour x luminous intensity (candela): Bevy's `color_inverse_square_range.rgb`.
    pub color_intensity: Vec3,
    pub spot: Option<Spot>,
    /// Occluders nearer the light than this don't shadow it (the light's `shadow_map_near_z`:
    /// a lantern's cage or a torch's head around the flame).
    pub near: f32,
    /// Moved, appeared, vanished or changed power since last frame ([`LightTracker`]).
    pub changed: bool,
}

/// Words per light in the GPU buffer (4 x vec4, `GiLight` in `probe_update.wesl`).
pub const GPU_LIGHT_WORDS: usize = 16;
/// `GiLight::flags` bit in the shader: a spot light.
pub const FLAG_SPOT: u32 = 1;
/// `GiLight::flags` bit in the shader: changed this frame.
pub const FLAG_CHANGED: u32 = 2;

impl GiLight {
    /// A point light from Bevy's component values: linear colour, `intensity` in lumens.
    pub fn point(position: Vec3, linear_color: Vec3, lumens: f32, range: f32, near: f32) -> Self {
        Self {
            position,
            range,
            color_intensity: linear_color * (lumens / (4.0 * std::f32::consts::PI)),
            spot: None,
            near,
            changed: false,
        }
    }

    /// Makes this light a spot light (Bevy also divides a spot's lumens by 4pi): its forward
    /// direction and cone half-angles (radians).
    #[must_use]
    pub fn with_spot(self, direction: Vec3, inner: f32, outer: f32) -> Self {
        let (scale, offset) = spot_scale_offset(inner, outer);
        Self { spot: Some(Spot { direction: direction.normalize_or_zero(), scale, offset }), ..self }
    }

    pub fn inverse_range_sq(&self) -> f32 {
        if self.range > 0.0 {
            1.0 / (self.range * self.range)
        } else {
            f32::MAX
        }
    }

    /// Unshadowed irradiance at point `p` with unit normal `n`: `NdotL` x attenuation x cone x
    /// colour-intensity. The shader adds a visibility ray and multiplies by albedo / pi.
    pub fn irradiance(&self, p: Vec3, n: Vec3) -> Vec3 {
        let to_light = self.position - p;
        let d2 = to_light.length_squared();
        if d2 * self.inverse_range_sq() >= 1.0 || d2 <= 0.0 {
            return Vec3::ZERO;
        }
        let l = to_light / d2.sqrt();
        let n_dot_l = n.dot(l);
        if n_dot_l <= 0.0 {
            return Vec3::ZERO;
        }
        let cone = self.spot.map_or(1.0, |s| spot_attenuation(s.direction.dot(-l), s.scale, s.offset));
        self.color_intensity * (n_dot_l * distance_attenuation(d2, self.inverse_range_sq()) * cone)
    }

    /// The shader's record as raw 32-bit words: `position, 1/range^2 | colour x intensity,
    /// flags | spot direction, spot scale | spot offset, near, 0, 0`.
    pub fn to_gpu(&self) -> [u32; GPU_LIGHT_WORDS] {
        let (dir, scale, offset) = self.spot.map_or((Vec3::ZERO, 0.0, 0.0), |s| (s.direction, s.scale, s.offset));
        let mut flags = 0;
        if self.spot.is_some() {
            flags |= FLAG_SPOT;
        }
        if self.changed {
            flags |= FLAG_CHANGED;
        }
        let (p, c) = (self.position, self.color_intensity);
        [
            p.x.to_bits(),
            p.y.to_bits(),
            p.z.to_bits(),
            self.inverse_range_sq().to_bits(),
            c.x.to_bits(),
            c.y.to_bits(),
            c.z.to_bits(),
            flags,
            dir.x.to_bits(),
            dir.y.to_bits(),
            dir.z.to_bits(),
            scale.to_bits(),
            offset.to_bits(),
            self.near.to_bits(),
            0,
            0,
        ]
    }
}

/// Distance from `p` to the axis-aligned box `[min, max]` (0 inside).
pub fn box_distance(p: Vec3, min: Vec3, max: Vec3) -> f32 {
    (p.clamp(min, max) - p).length()
}

/// Keeps the lights whose range reaches the volume box `[min, max]`, nearest to its centre
/// first, at most `n`.
pub fn select_nearest(lights: &mut Vec<GiLight>, min: Vec3, max: Vec3, n: usize) {
    let centre = (min + max) * 0.5;
    lights.retain(|l| box_distance(l.position, min, max) < l.range);
    lights.sort_by(|a, b| a.position.distance_squared(centre).total_cmp(&b.position.distance_squared(centre)));
    lights.truncate(n);
}

/// The hysteresis a probe at `p` blends with: `changed_hysteresis` (if lower than `base`) when a
/// changed light's range covers it, else `base`.
pub fn probe_hysteresis(p: Vec3, lights: &[GiLight], base: f32, changed_hysteresis: f32) -> f32 {
    let near_change = lights.iter().any(|l| l.changed && p.distance_squared(l.position) * l.inverse_range_sq() < 1.0);
    if near_change {
        base.min(changed_hysteresis)
    } else {
        base
    }
}

#[derive(Clone, Copy, Debug)]
struct Seen {
    position: Vec3,
    power: f32,
    range: f32,
    frame: u32,
}

/// Remembers each light's position and power between frames to flag changes.
#[derive(Debug)]
pub struct LightTracker<K> {
    seen: HashMap<K, Seen>,
    frame: u32,
}

impl<K> Default for LightTracker<K> {
    fn default() -> Self {
        Self { seen: HashMap::new(), frame: 0 }
    }
}

impl<K: Hash + Eq + Copy> LightTracker<K> {
    /// Starts a frame: every light still present is [`observe`](Self::observe)d before
    /// [`end_frame`](Self::end_frame).
    pub fn begin_frame(&mut self) {
        self.frame = self.frame.wrapping_add(1);
    }

    /// Records light `key` this frame (`power`: lumens, or any brightness measure); true if it
    /// is new, moved more than [`MOVED_DISTANCE`] or its power changed by more than [`POWER_CHANGE`].
    pub fn observe(&mut self, key: K, position: Vec3, power: f32, range: f32) -> bool {
        let now = Seen { position, power, range, frame: self.frame };
        match self.seen.insert(key, now) {
            None => true,
            Some(old) => {
                old.position.distance_squared(position) > MOVED_DISTANCE * MOVED_DISTANCE
                    || (power - old.power).abs() > POWER_CHANGE * old.power.abs().max(power.abs())
            }
        }
    }

    /// Ends the frame: forgets lights not observed and returns them as dark, changed lights at
    /// their last position, so the probes they lit also update fast.
    pub fn end_frame(&mut self) -> Vec<GiLight> {
        let frame = self.frame;
        let mut gone = Vec::new();
        self.seen.retain(|_, s| {
            let keep = s.frame == frame;
            if !keep {
                gone.push(GiLight {
                    position: s.position,
                    range: s.range,
                    color_intensity: Vec3::ZERO,
                    spot: None,
                    near: 0.0,
                    changed: true,
                });
            }
            keep
        });
        gone
    }

    /// Lights remembered (observed last frame).
    pub fn len(&self) -> usize {
        self.seen.len()
    }

    pub fn is_empty(&self) -> bool {
        self.seen.is_empty()
    }
}
