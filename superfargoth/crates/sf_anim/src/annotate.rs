//! Baked hand and foot contacts (action.md B2 "minimal `clip.annotate`", "Windows"
//! `contact(hand/foot, side)`): per arm and support limb of the rig profile, the spans where its
//! effector is planted, with where it is at plant time. Contact or sync markers baked per clip, as
//! engines tag them (UE anim notifies and foot sync markers, motion-matching contact tags); a
//! traversal move asks when its hands touch the lip and where they are against the root, and its
//! warp window can be bounded by them ([`crate::ClipMeta::contact_span`]).
//!
//! Detection, per effector, on the cooked clip in clip space (root motion applied, so the ground
//! is still), from its speed per frame step: planted below [`CONTACT_SPEED`] limb lengths per
//! second and released above [`CONTACT_RELEASE`] times that (a Schmitt trigger, as
//! [`crate::ContactRules`]); spans shorter than [`MIN_CONTACT`] dropped; a hand's span kept only if
//! its girdle moves at least [`HAND_BRACE`] limb lengths against it (the body pulls or pushes on
//! the hand). A hand also plants as a pivot: below [`HAND_SPEED`] and under [`HAND_PIVOT`] times
//! the speed its girdle moves past it (released above [`CONTACT_RELEASE`] times that speed, or past
//! [`HAND_PIVOT_RELEASE`]). Hand contacts on climbs are commonly detected so ("nearly still in the
//! world while the body moves past it"): keyframed and mocap mantles slide the gripping palm up
//! the face at 0.4-0.9 m/s while the shoulders pass it at 2-4 m/s, too fast for a still test.
//! No height test: a climb ends a metre above where it starts, so a foot planted on top
//! is nowhere near the clip's lowest point, and a hand on a lip is at whatever height the lip is.
//! Known limit: a foot hanging perfectly still in the air (a keyframed hang) reads as planted.
//!
//! A retargeted clip inherits its source's contacts ([`transfer_contacts`]): their times are
//! rig-independent, while the target's scaled root (leg ratio) and arms (arm ratio) slide its
//! hands against the lip, so re-detecting there misses grips the source shows. The IK pins a hand
//! to the environment from `start`, so only the times and the target's own positions matter.
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
/// Fastest pivoting hand, limb lengths per second: about 1.1 m/s on a 0.55 m arm. A gripping palm
/// slides up a face at 0.4-0.9 m/s in the measured mantles (UAL2 `ClimbUp_1m`/`_2m`); a reach or
/// a swing at its middle moves 2-8 m/s.
pub const HAND_SPEED: f32 = 2.0;
/// Pivot: a hand plants below this ratio of its own speed to the speed its girdle moves past it
/// (0 a hand fixed in the world under a moving body, about 1 a hand swinging from a still one,
/// unbounded a hand carried by the body). Measured grips run 0.1-0.45.
pub const HAND_PIVOT: f32 = 0.5;
/// A pivoting hand releases above this ratio: under 1, so a swing slow at its turn never holds.
/// Measured grips peak at 0.6, releases jump past 1.2.
pub const HAND_PIVOT_RELEASE: f32 = 0.8;

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
/// `girdle` (hands) the girdle joint per frame for the pivot and brace tests, `length` the limb's
/// length, `dt` the frame step. Speeds are per frame step (frame `f` to `f + 1`), so a span runs
/// from the frame the effector stops to the frame it leaves, without smearing the fast approach
/// into it.
pub(crate) fn planted_spans(positions: &[Vec3], girdle: Option<&[Vec3]>, length: f32, dt: f32) -> Vec<(usize, usize)> {
    let n = positions.len();
    if n < 2 || dt <= 0.0 || girdle.is_some_and(|g| g.len() != n) {
        return Vec::new();
    }
    let length = length.max(0.05);
    let (still, still_release) = (CONTACT_SPEED * length, CONTACT_SPEED * CONTACT_RELEASE * length);
    let (grip, grip_release) = (HAND_SPEED * length, HAND_SPEED * CONTACT_RELEASE * length);
    // Step `i`: the effector's speed, and (hands) the girdle's speed against it.
    let step = |i: usize| {
        let v = positions[i + 1].distance(positions[i]) / dt;
        let past = girdle.map_or(0.0, |g| (g[i + 1] - positions[i + 1]).distance(g[i] - positions[i]) / dt);
        (v, past)
    };
    let mut runs = Vec::new();
    let mut start = None;
    for i in 0..n - 1 {
        let (v, past) = step(i);
        let planted = if start.is_some() {
            v <= still_release || (v <= grip_release && v <= HAND_PIVOT_RELEASE * past)
        } else {
            v < still || (v < grip && v < HAND_PIVOT * past)
        };
        match (planted, start) {
            (true, None) => start = Some(i),
            (false, Some(a)) => {
                runs.push((a, i));
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

/// The profile's contact limbs: arms (hands) and support limbs (feet).
fn contact_limbs(profile: &RigProfile) -> Vec<(usize, ContactKind)> {
    profile
        .limbs
        .iter()
        .enumerate()
        .filter_map(|(i, l)| match l.kind {
            LimbKind::Arm => Some((i, ContactKind::Hand)),
            _ if l.support => Some((i, ContactKind::Foot)),
            _ => None,
        })
        .collect()
}

/// The root at frame `f`: root motion and attitude, clip space.
fn root_of(clip: &Clip, f: usize) -> Affine3A {
    clip.root_motion.as_ref().map_or(Affine3A::IDENTITY, |rm| {
        let [x, y, z, yaw] = rm.frames[f.min(rm.frames.len() - 1)];
        let attitude = rm.attitude.get(f).map_or(Quat::IDENTITY, |&[p, r]| attitude_rotation(glam::Vec2::new(p, r)));
        Affine3A::from_rotation_translation(Quat::from_rotation_y(yaw) * attitude, Vec3::new(x, y, z))
    })
}

/// Replaces `clip.meta.contacts` with the planted spans of the profile's arms (hands) and support
/// limbs (feet), sorted by start time. Additive clips and single frames get none.
pub fn annotate_contacts(skeleton: &Skeleton, profile: &RigProfile, clip: &mut Clip) {
    clip.meta.contacts.clear();
    let n = clip.frame_count as usize;
    if clip.meta.additive || n < 2 || clip.rate <= 0.0 {
        return;
    }
    let limbs = contact_limbs(profile);
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
        let root = root_of(clip, f);
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

/// Replaces `clip.meta.contacts` (a retargeted clip) with `source`'s contacts (its source clip's,
/// [`annotate_contacts`] on the source rig): the same kinds, sides and times, on the target
/// profile's limb of that kind and side (the same-named one first), with the target's effector and
/// its positions at `start`. A contact without such a limb is dropped; additive clips get none.
pub fn transfer_contacts(source: &[Contact], skeleton: &Skeleton, profile: &RigProfile, clip: &mut Clip) {
    clip.meta.contacts.clear();
    let n = clip.frame_count as usize;
    if clip.meta.additive || n == 0 {
        return;
    }
    let limbs = contact_limbs(profile);
    let mut pose = LocalPose::new(clip.joint_count());
    let mut model = Vec::new();
    for c in source {
        let side = |&&(i, kind): &&(usize, ContactKind)| kind == c.kind && profile.limbs[i].side == c.side;
        let Some(&(i, _)) = limbs.iter().filter(side).find(|(i, _)| profile.limbs[*i].name == c.limb).or_else(|| limbs.iter().find(side))
        else {
            continue;
        };
        let limb = &profile.limbs[i];
        let (start, end) = (c.start.clamp(0.0, clip.duration), c.end.clamp(0.0, clip.duration));
        // Sampled at `start` (the target may cook at another rate than its source).
        clip.sample(start, &mut pose, None);
        model_of(skeleton, &pose.to_vec(), &mut model);
        let local = Vec3::from(model[usize::from(limb.effector)].translation);
        let (at, yaw) = clip.root_at(start);
        let root = Affine3A::from_rotation_translation(Quat::from_rotation_y(yaw) * attitude_rotation(clip.attitude_at(start)), at);
        clip.meta.contacts.push(Contact {
            kind: c.kind,
            side: c.side,
            limb: limb.name.clone(),
            joint: skeleton.name(usize::from(limb.effector)).to_owned(),
            start,
            end,
            position: root.transform_point3(local),
            local,
        });
    }
    clip.meta.contacts.sort_by(|x, y| x.start.total_cmp(&y.start));
}
