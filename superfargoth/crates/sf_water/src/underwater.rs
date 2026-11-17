//! The underwater medium's colour, the CPU reference of the underwater post (water.md
//! "Underwater", "Medium"; research `underwater-2026-11-17.md` "Underwater medium", slice U3).
//! `sf_water_bevy` `water_underwater.wesl` computes the same per pixel; its constants are these.
//!
//! When the camera is under the water, every pixel's path from the lens to what it sees runs
//! through the body: spectral Beer-Lambert extinction `T = exp(−σt·d)` per channel (clear water
//! loses red first), plus single scattering of the light that reached the camera's depth,
//! `albedo · (ambient + sun) · exp(−Kd·z)`, filling in what was taken away: `scene·T + in·(1 − T)`.
//! `Kd` is the extinction itself times a scale (the diffuse attenuation coefficient
//! `Kd ≈ a + b_b`; Crest lights its underwater fog with the same out-scattering by depth). A ray
//! that hits nothing (looking down into the deep, or up past the surface before U2 draws it) is
//! infinitely long: `T = 0`, the medium's own colour. The optics are the body's own, the ones its
//! surface draw uses for depth colour, so the surface and the volume agree.

use glam::Vec3;

use crate::WaterOptics;

/// Extinction floor per metre: a body of zero optics still has an albedo (no 0/0) and an infinite
/// path still ends black instead of NaN (0·∞). The water draw floors it the same.
pub const MIN_EXTINCTION: f32 = 1e-4; // tunables: const (maths: a division guard, the shaders' floor)

/// The medium the camera is in: per-metre absorption and scattering (linear RGB) and the scale
/// on the extinction that sets how fast the light dims with depth (the `Kd` source).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnderwaterOptics {
    pub absorption: Vec3,
    pub scattering: Vec3,
    /// `Kd = kd_scale · σt` (1 = the extinction itself).
    pub kd_scale: f32,
}

impl UnderwaterOptics {
    /// From a body's optics record, with `Kd` = `kd_scale` × its extinction.
    #[must_use]
    pub fn from_optics(o: &WaterOptics, kd_scale: f32) -> Self {
        Self { absorption: Vec3::from(o.absorption), scattering: Vec3::from(o.scattering), kd_scale }
    }

    /// `σt = a + s` per channel, floored at [`MIN_EXTINCTION`].
    #[must_use]
    pub fn extinction(&self) -> Vec3 {
        (self.absorption + self.scattering).max(Vec3::splat(MIN_EXTINCTION))
    }

    /// Single-scattering albedo `s / σt` (0 to 1 for non-negative optics).
    #[must_use]
    pub fn albedo(&self) -> Vec3 {
        self.scattering.max(Vec3::ZERO) / self.extinction()
    }

    /// Transmittance over `distance` metres of the medium: 1 at 0 (or a negative / NaN distance),
    /// 0 at infinity.
    #[must_use]
    pub fn transmittance(&self, distance: f32) -> Vec3 {
        let d = if distance > 0.0 { distance } else { 0.0 };
        (-self.extinction() * d).exp()
    }

    /// The share of the surface light left at `depth` metres below the surface:
    /// `exp(−Kd·z)`, 1 at or above the surface, 0 at infinite depth.
    #[must_use]
    pub fn light_at_depth(&self, depth: f32) -> Vec3 {
        let z = if depth > 0.0 { depth } else { 0.0 };
        let kd = self.extinction() * self.kd_scale.max(0.0);
        // 0 · ∞ (no falloff, infinite depth) is no falloff.
        if kd.min_element() <= 0.0 {
            return Vec3::ONE;
        }
        (-kd * z).exp()
    }
}

/// The light at the surface: the sun term and the ambient, both radiance-like linear RGB (the same
/// scale the scene colour is lit at, before the view's exposure).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UnderwaterLight {
    /// [`sun_term`] summed over the directional lights.
    pub sun: Vec3,
    pub ambient: Vec3,
}

/// One directional light's contribution: illuminance × colour (`illuminance`, linear RGB) times the
/// cosine of its elevation over π (`to_light`: unit direction toward the light, +Y up), the radiance
/// a white diffuse surface lit by it reflects (Bevy shades the water's body colour that way). 0 below
/// the horizon.
#[must_use]
pub fn sun_term(illuminance: Vec3, to_light: Vec3) -> Vec3 {
    illuminance * (to_light.y.max(0.0) / std::f32::consts::PI)
}

/// The colour of a pixel seen through `distance` metres of the medium (∞: nothing hit) by a
/// camera `depth` metres below the surface (∞: no surface light reaches it):
/// `scene·T + albedo·(ambient + sun)·exp(−Kd·z)·(1 − T)`, `T = exp(−σt·d)` per channel.
#[must_use]
pub fn underwater_colour(scene: Vec3, distance: f32, depth: f32, optics: &UnderwaterOptics, light: UnderwaterLight) -> Vec3 {
    let t = optics.transmittance(distance);
    let inscatter = optics.albedo() * (light.ambient + light.sun) * optics.light_at_depth(depth);
    scene * t + inscatter * (Vec3::ONE - t)
}
