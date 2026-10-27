//! Baked hand and foot contacts (action.md B2 "minimal `clip.annotate`", "Windows"
//! `contact(hand/foot, side)`): per arm and support limb of the rig profile, the spans where its
//! effector is planted, with where it is at plant time. Contact or sync markers baked per clip, as
//! engines tag them (UE anim notifies and foot sync markers, motion-matching contact tags); a
//! traversal move asks when its hands touch the lip and where they are against the root, and its
//! warp window can be bounded by them ([`crate::ClipMeta::contact_span`]).
//!
//! Detection, per effector, on the cooked clip in clip space (root motion applied, so the ground
//! is still): its speed by central differences, planted below [`CONTACT_SPEED`] limb lengths per
//! second and released above [`CONTACT_RELEASE`] times that (a Schmitt trigger, as
//! [`crate::ContactRules`]); spans shorter than [`MIN_CONTACT`] dropped; a hand's span kept only if
//! its girdle moves at least [`HAND_BRACE`] limb lengths against it (the body pulls or pushes on
//! the hand). No height test: a climb ends a metre above where it starts, so a foot planted on top
//! is nowhere near the clip's lowest point, and a hand on a lip is at whatever height the lip is.
//! Known limit: a foot hanging perfectly still in the air (a keyframed hang) reads as planted.
//!
//! Conventions: Y up, meters, seconds, clip space = the root at time 0 (origin, facing +Z: the
//! space of the root track and of a [`crate::WarpPlan`]'s baked chain).

use glam::{Affine3A, Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::cook::{frame_time, model_of};
use crate::{Clip, LimbKind, LocalPose, RigProfile, Side, Skeleton, attitude_rotation};

/// Speed below which an effector plants, limb lengths per second. As
/// [`crate::ContactRules`]' default: about 0.3 m/s on a 0.9 m leg, 0.2 m/s on a 0.6 m arm. A hand
/// pressing on a lip drifts a few cm/s in mocap; a swinging one is this slow only at its turn.
pub const CONTACT_SPEED: f32 = 0.35;
/// Hysteresis: a planted effector releases above `CONTACT_SPEED × CONTACT_RELEASE` (as
/// [`crate::ContactRules::release`]), so a slow lift-off or a drifting palm doesn't flicker.
pub const CONTACT_RELEASE: f32 = 2.0;
/// Shortest contact, seconds: a swing's turning point is slow for a frame or two (30-60 ms at
/// 30 Hz); a hand on a lip or a foot in a stance stays for 0.2 s or more.
pub const MIN_CONTACT: f32 = 0.1;
/// Bracing, limb lengths: a still hand is a contact only if its girdle (shoulder) moves at least
/// this far against it over the span (about 6 cm on a 0.6 m arm). A hand at rest in an idle, or
/// carried along by a still body, isn't one. Feet carry weight whenever still: no brace test.
pub const HAND_BRACE: f32 = 0.1;

/// Which kind of effector a [`Contact`] is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContactKind {
    /// An arm's effector.
    Hand,
    /// A support limb's effector.
    Foot,
}

/// One planted span of one effector.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    pub kind: ContactKind,
    pub side: Side,
    /// The rig profile limb ([`crate::Limb::name`]).
    pub limb: String,
    /// The effector joint.
    pub joint: String,
    /// Clip time of the first and last planted frame.
    pub start: f32,
    pub end: f32,
    /// The effector at `start`, clip space.
    pub position: Vec3,
    /// The effector at `start` in the root's frame at `start` (the cooked pose's model space).
    pub local: Vec3,
}

/// Planted spans of one effector, as inclusive frame ranges: `positions` per frame in clip space,
/// `girdle` (hands) the girdle joint per frame for the brace test, `length` the limb's length,
/// `dt` the frame step.
pub(crate) fn planted_spans(positions: &[Vec3], girdle: Option<&[Vec3]>, length: f32, dt: f32) -> Vec<(usize, usize)> {
    let n = positions.len();
    if n < 2 || dt <= 0.0 {
        return Vec::new();
    }
    let length = length.max(0.05);
    let (plant, release) = (CONTACT_SPEED * length, CONTACT_SPEED * CONTACT_RELEASE * length);
    let speed = |f: usize| {
        let (a, b) = (f.saturating_sub(1), (f + 1).min(n - 1));
        #[allow(clippy::cast_precision_loss)]
        let step = (b - a) as f32 * dt;
        positions[b].distance(positions[a]) / step
    };
    let mut runs = Vec::new();
    let mut start = None;
    for f in 0..n {
        let s = speed(f);
        let planted = if start.is_some() { s <= release } else { s < plant };
        match (planted, start) {
            (true, None) => start = Some(f),
            (false, Some(a)) => {
                runs.push((a, f - 1));
                start = None;
            }
            _ => {}
        }
    }
    if let Some(a) = start {
        runs.push((a, n - 1));
    }
    #[allow(clippy::cast_precision_loss)]
    let long_enough = |&(a, b): &(usize, usize)| (b - a) as f32 * dt + 1e-4 >= MIN_CONTACT;
    let braced = |&(a, b): &(usize, usize)| {
        girdle.is_none_or(|g| {
            let offset = |f: usize| g[f] - positions[f];
            let first = offset(a);
            (a..=b).map(|f| offset(f).distance(first)).fold(0.0, f32::max) >= HAND_BRACE * length
        })
    };
    runs.into_iter().filter(|r| long_enough(r) && braced(r)).collect()
}

/// Replaces `clip.meta.contacts` with the planted spans of the profile's arms (hands) and support
/// limbs (feet), sorted by start time. Additive clips and single frames get none.
pub fn annotate_contacts(skeleton: &Skeleton, profile: &RigProfile, clip: &mut Clip) {
    clip.meta.contacts.clear();
    let n = clip.frame_count as usize;
    if clip.meta.additive || n < 2 || clip.rate <= 0.0 {
        return;
    }
    let limbs: Vec<(usize, ContactKind)> = profile
        .limbs
        .iter()
        .enumerate()
        .filter_map(|(i, l)| match l.kind {
            LimbKind::Arm => Some((i, ContactKind::Hand)),
            _ if l.support => Some((i, ContactKind::Foot)),
            _ => None,
        })
        .collect();
    if limbs.is_empty() {
        return;
    }
    let girdle_of = |i: usize| {
        let l = &profile.limbs[i];
        profile.girdles.get(usize::from(l.girdle)).map(|g| usize::from(g.joint)).or_else(|| l.joints.first().map(|&j| usize::from(j)))
    };
    // Per frame: the root, then each limb's effector (and a hand's girdle) in clip and root space.
    let mut pose = LocalPose::new(clip.joint_count());
    let mut model = Vec::new();
    let mut world = vec![Vec::with_capacity(n); limbs.len()];
    let mut local = vec![Vec::with_capacity(n); limbs.len()];
    let mut girdles = vec![Vec::with_capacity(n); limbs.len()];
    for f in 0..n {
        clip.sample(frame_time(f, n, clip.duration), &mut pose, None);
        model_of(skeleton, &pose.to_vec(), &mut model);
        let root = clip.root_motion.as_ref().map_or(Affine3A::IDENTITY, |rm| {
            let [x, y, z, yaw] = rm.frames[f.min(rm.frames.len() - 1)];
            let attitude = rm.attitude.get(f).map_or(Quat::IDENTITY, |&[p, r]| attitude_rotation(glam::Vec2::new(p, r)));
            Affine3A::from_rotation_translation(Quat::from_rotation_y(yaw) * attitude, Vec3::new(x, y, z))
        });
        for (k, &(i, kind)) in limbs.iter().enumerate() {
            let p = Vec3::from(model[usize::from(profile.limbs[i].effector)].translation);
            local[k].push(p);
            world[k].push(root.transform_point3(p));
            if let (ContactKind::Hand, Some(g)) = (kind, girdle_of(i)) {
                girdles[k].push(root.transform_point3(Vec3::from(model[g].translation)));
            }
        }
    }
    let dt = 1.0 / clip.rate;
    for (k, &(i, kind)) in limbs.iter().enumerate() {
        let limb = &profile.limbs[i];
        let girdle = (kind == ContactKind::Hand && girdles[k].len() == n).then_some(girdles[k].as_slice());
        for (a, b) in planted_spans(&world[k], girdle, limb.length, dt) {
            clip.meta.contacts.push(Contact {
                kind,
                side: limb.side,
                limb: limb.name.clone(),
                joint: skeleton.name(usize::from(limb.effector)).to_owned(),
                start: frame_time(a, n, clip.duration),
                end: frame_time(b, n, clip.duration),
                position: world[k][a],
                local: local[k][a],
            });
        }
    }
    clip.meta.contacts.sort_by(|x, y| x.start.total_cmp(&y.start));
}
