//! Comfort settings (camera.md "Comfort switches", XAG 117) and the rotation-only trauma shake
//! (Eiserloh, "Math for Game Programmers: Juicing Your Cameras With Math", GDC 2016): impacts add
//! trauma, trauma decays linearly, the shake is trauma² times smooth noise, rotation only.

use glam::Quat;
use serde::{Deserialize, Serialize};

/// First or third person, switchable any time (no gameplay tick changes).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewChoice {
    FirstPerson,
    #[default]
    ThirdPerson,
}

/// Player camera settings (saved with the options, not with the game).
#[allow(clippy::struct_excessive_bools, reason = "independent settings switches")]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ComfortSettings {
    pub shake: bool,
    /// 0..1 scale on the shake angles.
    pub shake_strength: f32,
    /// First-person head bob; off by default.
    pub head_bob: bool,
    /// Auto-recentre behind travel and framed cameras; off by default (framed cameras then fall
    /// back to look limits only).
    pub auto_camera: bool,
    /// Vertical FOV of the world camera and of the first-person body, degrees.
    pub fov_world: f32,
    pub fov_body: f32,
    pub centre_dot: bool,
    pub motion_blur: bool,
    pub view: ViewChoice,
}

impl Default for ComfortSettings {
    fn default() -> Self {
        Self {
            shake: true,
            shake_strength: 1.0,
            head_bob: false,
            auto_camera: false,
            fov_world: 60.0,
            fov_body: 50.0,
            centre_dot: false,
            motion_blur: false,
            view: ViewChoice::ThirdPerson,
        }
    }
}

impl ComfortSettings {
    /// Shake strength after the on/off switch, 0..1.
    pub fn shake_scale(&self) -> f32 {
        if self.shake { self.shake_strength.clamp(0.0, 1.0) } else { 0.0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ShakeParams {
    /// Peak angles at trauma 1, radians.
    pub max_yaw: f32,
    pub max_pitch: f32,
    pub max_roll: f32,
    /// Noise frequency, Hz.
    pub frequency: f32,
    /// Trauma lost per second.
    pub decay: f32,
}

impl Default for ShakeParams {
    fn default() -> Self {
        Self { max_yaw: 2f32.to_radians(), max_pitch: 2f32.to_radians(), max_roll: 3f32.to_radians(), frequency: 18.0, decay: 1.6 }
    }
}

/// Trauma shake state (presentation; one per camera).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TraumaShake {
    trauma: f32,
    time: f32,
    seed: u32,
}

impl TraumaShake {
    pub fn new(seed: u32) -> Self {
        Self { seed, ..Self::default() }
    }

    pub fn trauma(&self) -> f32 {
        self.trauma
    }

    /// An impact adds trauma (0..1, saturating).
    pub fn add(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount.max(0.0)).min(1.0);
    }

    /// Advance one frame and return the shake rotation to compose after the look rotation
    /// (`look.rotation() * shake`). Identity once trauma is spent or the setting is off.
    pub fn update(&mut self, p: &ShakeParams, settings: &ComfortSettings, dt: f32) -> Quat {
        self.trauma = (self.trauma - p.decay * dt).max(0.0);
        self.time += dt;
        let amp = self.trauma * self.trauma * settings.shake_scale();
        if amp <= 0.0 {
            return Quat::IDENTITY;
        }
        let t = self.time * p.frequency;
        let yaw = p.max_yaw * amp * noise(self.seed, t);
        let pitch = p.max_pitch * amp * noise(self.seed.wrapping_add(0x9e37_79b9), t);
        let roll = p.max_roll * amp * noise(self.seed.wrapping_add(0x3c6e_f372), t);
        Quat::from_euler(glam::EulerRot::YXZ, yaw, pitch, roll)
    }
}

fn hash(seed: u32, i: i32) -> f32 {
    // Integer hash (lowbias32, Chris Wellons) mapped to -1..1.
    #[allow(clippy::cast_sign_loss)]
    let mut x = seed ^ (i as u32).wrapping_mul(0x9e37_79b9);
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846c_a68b);
    x ^= x >> 16;
    #[allow(clippy::cast_precision_loss)]
    let unit = (x >> 8) as f32 / (1u32 << 24) as f32;
    unit * 2.0 - 1.0
}

/// Smooth 1D value noise in -1..1 (hashed lattice, smoothstep blend).
pub fn noise(seed: u32, t: f32) -> f32 {
    let cell = t.floor();
    #[allow(clippy::cast_possible_truncation)]
    let index = cell as i32;
    let blend = crate::smoothstep(t - cell);
    let (lo, hi) = (hash(seed, index), hash(seed, index.wrapping_add(1)));
    lo + (hi - lo) * blend
}
