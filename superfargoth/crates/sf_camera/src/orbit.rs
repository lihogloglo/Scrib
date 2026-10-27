//! Third-person orbit rig (camera.md "Third person"): a pivot on the render body, the eye on an
//! arm behind it along the look, offset to a shoulder. Position lag is a critically damped spring
//! on the pivot; rotation follows the look directly (no lag on aim).

use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_anim::spring::{critical_spring, damper};
use sf_coords::WorldPos;

use crate::{Look, ease_factor, wrap_angle};

/// Camera mode: picks the arm length. The shell maps the action layer's mode and lock state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum RigMode {
    #[default]
    Normal,
    Sneak,
    FreeRun,
    Locked,
}

/// Arm length per mode, metres.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ModeDistances {
    pub normal: f32,
    pub sneak: f32,
    pub free_run: f32,
    pub locked: f32,
}

impl Default for ModeDistances {
    fn default() -> Self {
        Self { normal: 3.2, sneak: 2.4, free_run: 4.0, locked: 3.6 }
    }
}

impl ModeDistances {
    pub fn get(&self, mode: RigMode) -> f32 {
        match mode {
            RigMode::Normal => self.normal,
            RigMode::Sneak => self.sneak,
            RigMode::FreeRun => self.free_run,
            RigMode::Locked => self.locked,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Shoulder {
    #[default]
    Right,
    Left,
}

impl Shoulder {
    pub fn sign(self) -> f32 {
        match self {
            Self::Right => 1.0,
            Self::Left => -1.0,
        }
    }
}

/// Tuning for [`OrbitRig`]. Angles in radians, times are half-lives in seconds.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OrbitParams {
    /// Pivot above the render body's root (chest height).
    pub pivot_height: f32,
    /// Lateral pivot offset toward the active shoulder (screen right for [`Shoulder::Right`]).
    pub shoulder_offset: f32,
    pub pitch_min: f32,
    pub pitch_max: f32,
    pub distance: ModeDistances,
    /// Arm length ease between modes.
    pub distance_halflife: f32,
    /// Horizontal and vertical pivot spring (vertical longer: stairs, step-ups, landings).
    pub pivot_halflife_xz: f32,
    pub pivot_halflife_y: f32,
    /// A body jump in one frame beyond this snaps (teleports, cell loads), metres.
    pub max_lag: f32,
    /// Shoulder swap time.
    pub swap_halflife: f32,
}

impl Default for OrbitParams {
    fn default() -> Self {
        Self {
            pivot_height: 1.45,
            shoulder_offset: 0.45,
            pitch_min: -70f32.to_radians(),
            pitch_max: 60f32.to_radians(),
            distance: ModeDistances::default(),
            distance_halflife: 0.25,
            pivot_halflife_xz: 0.06,
            pivot_halflife_y: 0.12,
            max_lag: 2.0,
            swap_halflife: 0.12,
        }
    }
}

/// One frame of the rig: where the arm starts, the clamped look, and the desired (unobstructed)
/// eye offset from the pivot. Feed `pivot` and `arm` to [`crate::ArmCollision::update`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OrbitPose {
    pub pivot: WorldPos,
    pub look: Look,
    /// Desired eye minus pivot (shoulder offset plus the arm behind the look).
    pub arm: Vec3,
}

/// The orbit rig's frame-rate state (presentation; not saved, not replicated).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OrbitRig {
    /// Smoothed pivot minus the body's pivot, and its velocity relative to the body.
    lag: Vec3,
    lag_vel: Vec3,
    /// Last pivot target, body velocity and lead.
    last: Option<(WorldPos, Vec3, f32)>,
    shoulder: Shoulder,
    side: f32,
    side_vel: f32,
    distance: Option<f32>,
    /// Extra pivot height (a traversal's destination, [`Self::set_lead`]).
    lead: f32,
}

impl OrbitRig {
    pub fn new(shoulder: Shoulder) -> Self {
        Self { shoulder, side: shoulder.sign(), ..Self::default() }
    }

    pub fn shoulder(&self) -> Shoulder {
        self.shoulder
    }

    /// Side swap (eased by `swap_halflife`).
    pub fn swap_shoulder(&mut self) {
        self.shoulder = match self.shoulder {
            Shoulder::Right => Shoulder::Left,
            Shoulder::Left => Shoulder::Right,
        };
    }

    /// Current pivot lag (smoothed minus target), metres.
    pub fn lag(&self) -> Vec3 {
        self.lag
    }

    /// Raise the pivot's target `height` m above the body from the next update on (0 = none).
    /// For a warped traversal (mantle, climb): the shell passes how far the move's destination
    /// is above the drawn root, so the pivot leads to where the body will stand instead of
    /// riding the root through the ledge it climbs (camera.md "As built (traversal pivot)").
    /// A change eases in through the vertical spring and never counts as a teleport.
    pub fn set_lead(&mut self, height: f32) {
        self.lead = height.max(0.0);
    }

    /// Forget the smoothing (teleport, view switch): the next update starts on the body.
    pub fn reset(&mut self) {
        *self = Self { shoulder: self.shoulder, side: self.shoulder.sign(), ..Self::default() };
    }

    /// Advance one frame. `body` is the render body's root, `body_vel` its gameplay velocity (the
    /// smooth one, not the frame delta, so snaps such as step-ups are filtered while steady motion
    /// is tracked with no lag); `look` is the player's look (its pitch is clamped here).
    pub fn update(&mut self, p: &OrbitParams, body: WorldPos, body_vel: Vec3, look: Look, mode: RigMode, dt: f32) -> OrbitPose {
        let target = body.offset(Vec3::Y * (p.pivot_height + self.lead));
        if let Some((prev, prev_vel, prev_lead)) = self.last {
            // Spring in the body's frame: motion the velocity explains moves the pivot with the
            // body; the rest (snaps, velocity changes) becomes lag the spring removes. Exact for
            // a body at constant velocity, at any dt. The spring runs first and this frame's
            // snap lands at its end, since the body sample at the end of the frame holds it.
            let mut xz = glam::Vec2::new(self.lag.x, self.lag.z);
            let mut xz_vel = glam::Vec2::new(self.lag_vel.x, self.lag_vel.z);
            critical_spring(&mut xz, &mut xz_vel, glam::Vec2::ZERO, p.pivot_halflife_xz, dt);
            let (mut y, mut y_vel) = (self.lag.y, self.lag_vel.y);
            critical_spring(&mut y, &mut y_vel, 0.0, p.pivot_halflife_y, dt);
            self.lag = Vec3::new(xz.x, y, xz.y);
            self.lag_vel = Vec3::new(xz_vel.x, y_vel, xz_vel.y);
            let snap = prev.delta_f32(target) - body_vel * dt;
            self.lag -= snap;
            self.lag_vel -= body_vel - prev_vel;
            // A teleport: a jump in one frame that neither the velocity nor a lead change
            // explains (a lead change, e.g. a climb's destination 2.5 m up, eases in).
            if (snap - Vec3::Y * (self.lead - prev_lead)).length() > p.max_lag {
                self.lag = Vec3::ZERO;
                self.lag_vel = Vec3::ZERO;
            }
        }
        self.last = Some((target, body_vel, self.lead));

        critical_spring(&mut self.side, &mut self.side_vel, self.shoulder.sign(), p.swap_halflife, dt);
        let goal = p.distance.get(mode);
        let distance = self.distance.map_or(goal, |d| damper(d, goal, p.distance_halflife, dt));
        self.distance = Some(distance);

        let look = Look { yaw: wrap_angle(look.yaw), pitch: look.pitch.clamp(p.pitch_min, p.pitch_max) };
        let arm = look.right() * (p.shoulder_offset * self.side) - look.forward() * distance;
        OrbitPose { pivot: target.offset(self.lag), look, arm }
    }
}

/// Auto-recentre behind travel (comfort switch, off by default).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RecentreParams {
    /// Seconds without look input before recentring starts.
    pub delay: f32,
    /// Minimum planar travel speed, m/s.
    pub min_speed: f32,
    pub halflife: f32,
}

impl Default for RecentreParams {
    fn default() -> Self {
        Self { delay: 1.5, min_speed: 1.0, halflife: 0.8 }
    }
}

/// Eases the look's yaw behind the travel direction once the player has left the look alone for
/// `delay` seconds while moving; pitch is left alone. The shell writes the result back into its
/// look (it is still the player's look, the camera only proposes it).
pub fn recentre_behind(look: Look, travel: Vec3, idle_look_secs: f32, p: &RecentreParams, dt: f32) -> Look {
    let planar = Vec3::new(travel.x, 0.0, travel.z);
    if idle_look_secs < p.delay || planar.length() < p.min_speed {
        return look;
    }
    let goal = planar.x.atan2(planar.z);
    let t = ease_factor(p.halflife, dt);
    Look { yaw: wrap_angle(look.yaw + wrap_angle(goal - look.yaw) * t), pitch: look.pitch }
}
