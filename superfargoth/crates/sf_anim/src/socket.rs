//! Baked socket tracks (`docs/specs/combat.md` "Hit and hurt volumes"): a socket's transform in
//! the character root's space over a clip, sampled at a fixed rate when the clip loads, so
//! gameplay (weapon sweeps) reads a table and never the evaluated pose.
//!
//! Root space is the clip's pose with its root motion extracted (what [`Clip::sample`] gives under
//! an identity root): the root-motion track carries the travel, this track the swing.

use glam::{Affine3A, Vec3};
use serde::{Deserialize, Serialize};

use crate::math::nlerp;
use crate::{Clip, LocalPose, ModelPose, Skeleton, Trs, local_to_model};

/// A socket's root-space transform at uniform times over `[0, duration]`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocketTrack {
    /// Seconds; the last frame lands on it.
    pub duration: f32,
    /// At least one frame (a zero-length clip has exactly one).
    pub frames: Vec<Trs>,
}

impl SocketTrack {
    /// `f(t)` at `hz` frames per second over `[0, duration]` (the last frame on `duration`).
    pub fn from_fn(duration: f32, hz: f32, mut f: impl FnMut(f32) -> Trs) -> Self {
        let duration = if duration.is_finite() { duration.max(0.0) } else { 0.0 };
        // Truncation is the intent: a whole frame count, at least one interval.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let n = ((duration * hz.max(1.0)).ceil() as usize).max(1);
        #[allow(clippy::cast_precision_loss)]
        let frames = (0..=n).map(|i| f(duration * i as f32 / n as f32)).collect();
        Self { duration, frames }
    }

    /// The transform at `t` (clamped), interpolated between frames.
    pub fn sample(&self, t: f32) -> Trs {
        let last = self.frames.len() - 1;
        if last == 0 || self.duration <= 0.0 {
            return self.frames[0];
        }
        #[allow(clippy::cast_precision_loss)]
        let x = (t / self.duration).clamp(0.0, 1.0) * last as f32;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let i = (x as usize).min(last - 1);
        #[allow(clippy::cast_precision_loss)]
        let a = x - i as f32;
        let (p, q) = (self.frames[i], self.frames[i + 1]);
        Trs { translation: p.translation.lerp(q.translation, a), rotation: nlerp(p.rotation, q.rotation, a), scale: p.scale.lerp(q.scale, a) }
    }

    /// A socket-space point (a weapon segment end) in root space at `t`.
    pub fn point(&self, t: f32, local: Vec3) -> Vec3 {
        let s = self.sample(t);
        s.translation + s.rotation * (s.scale * local)
    }
}

/// Samples joint `bone`'s model-space transform (root space: identity root, root motion
/// extracted) over `clip` at `hz`. Runs at load (or bake); one pose evaluation per frame.
///
/// # Panics
/// If `bone` is not a joint of `skeleton`, or the clip is for another skeleton.
pub fn sample_socket_track(skeleton: &Skeleton, clip: &Clip, bone: usize, hz: f32) -> SocketTrack {
    assert!(bone < skeleton.len(), "socket joint {bone} out of range");
    let mut local = LocalPose::new(skeleton.len());
    let mut model = ModelPose::new(skeleton.len());
    SocketTrack::from_fn(clip.duration, hz, |t| {
        clip.sample(t, &mut local, None);
        local_to_model(skeleton, &local, &mut model, &Affine3A::IDENTITY);
        Trs { translation: model.translation(bone), rotation: model.rotation(bone), scale: Vec3::ONE }
    })
}

#[cfg(test)]
mod tests {
    use glam::Quat;

    use super::*;
    use crate::{ClipRules, CookSettings, cook_clip, fixtures};

    #[test]
    fn socket_track_matches_the_pose_and_leaves_travel_to_root_motion() {
        let s = fixtures::biped();
        let src = fixtures::gait(&s, "walk", 1.0, 1.2, 0.4, 0.6, &[("thigh_l", "foot_l", 0.0), ("thigh_r", "foot_r", 0.5)]);
        let clip = cook_clip(&s, &src, &ClipRules::default(), &CookSettings::default()).expect("cooks");
        let foot = s.find("foot_r").expect("foot");
        let track = sample_socket_track(&s, &clip, foot, 60.0);
        assert_eq!(track.frames.len(), 61);
        // A frame time samples the evaluated pose.
        let (mut local, mut model) = (LocalPose::new(s.len()), ModelPose::new(s.len()));
        clip.sample(0.5, &mut local, None);
        local_to_model(&s, &local, &mut model, &Affine3A::IDENTITY);
        assert!(track.sample(0.5).translation.distance(model.translation(foot)) < 1e-4);
        // The root travels 1.2 m; the foot in root space stays within a stride of the hips.
        let (x, z): (Vec<f32>, Vec<f32>) = track.frames.iter().map(|f| (f.translation.x, f.translation.z)).unzip();
        assert!(z.iter().all(|z| z.abs() < 0.6), "{z:?}");
        assert!(x.iter().all(|x| (x + 0.1).abs() < 0.05));
        assert!(clip.root_at(1.0).0.z > 1.0, "travel is in the root motion track");
    }

    #[test]
    fn synthetic_track_interpolates_and_clamps() {
        let track = SocketTrack::from_fn(0.5, 60.0, |t| Trs { translation: Vec3::new(t, 0.0, 0.0), rotation: Quat::IDENTITY, scale: Vec3::ONE });
        assert_eq!(track.frames.len(), 31);
        assert!((track.sample(0.25).translation.x - 0.25).abs() < 1e-5);
        assert!((track.sample(9.0).translation.x - 0.5).abs() < 1e-6);
        assert!((track.point(0.1, Vec3::Y).y - 1.0).abs() < 1e-6);
        assert_eq!(SocketTrack::from_fn(0.0, 60.0, |_| Trs::IDENTITY).sample(0.3), Trs::IDENTITY);
    }
}
