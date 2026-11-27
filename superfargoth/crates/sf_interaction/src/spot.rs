//! Errand spots (interaction.md "Errand spots: `pick_spot`", IN46): where a carried prop goes.
//!
//! One pure answer for tier 2 and tier 0 alike, so an errand run off-screen leaves the prop where an
//! on-screen one would. Candidates are the indexed `Surface` slots (any slot whose tags pass the
//! errand's filter) near the anchor, minus moving props, other users' claims and slots a resting prop
//! already covers, sorted by distance then `SlotRef`. None: a golden-angle spiral of floor points
//! (the caller's projection), then the anchor itself. Find, then claim: the caller reserves the slot
//! with an ordinary [`Claims::claim`] and takes the next candidate on a refusal the index can't see.
//!
//! Deterministic: the index is `BTreeMap` ordered, ties break on `SlotRef`, no unordered sums.

use std::f32::consts::PI;

use glam::Vec3;
use sf_world::PlacementId;

use crate::claims::{Claims, SlotRef, UserKey};
use crate::def::{Frame, TagFilter, TagId};
use crate::index::{IndexedSlot, PointIndex, SlotUser};
use crate::params::InteractParams;

/// The golden angle, rad (π(3 − √5) ≈ 2.399963): successive spiral points never line up.
// tunables: const (maths: the golden angle)
const GOLDEN_ANGLE: f32 = PI * 0.763_932;

/// Where the errand puts the prop down.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpotAnchor {
    /// World point the search centres on.
    pub at: Vec3,
    /// Only this placement's slots (this shelf), else any.
    pub placement: Option<PlacementId>,
}

/// What makes a slot or floor point unusable now.
#[derive(Clone, Copy, Debug)]
pub struct SpotBlockers<'a> {
    pub claims: &'a Claims,
    /// Resting props near the anchor (world). The carried prop is not resting, so never listed.
    pub props: &'a [Vec3],
    /// Carried or dragged props: their own slots are never a spot.
    pub moving: &'a [PlacementId],
}

/// One spot search.
#[derive(Clone, Copy, Debug)]
pub struct SpotQuery<'a> {
    pub anchor: SpotAnchor,
    /// The errand's `spot` filter, tested against each slot's tags (verb, surface family).
    pub filter: &'a TagFilter,
    /// The carrier; its own claims don't block (a re-pick after reserving is stable).
    pub user: UserKey,
    /// The carrier's tags, tested against each slot's `users`.
    pub tags: &'a [TagId],
    /// Search radius, m (3D; the caller's `ai.errand.spot_radius_m`).
    pub radius_m: f32,
}

/// The chosen spot.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Spot {
    /// A put-down slot and its world frame.
    Slot { slot: SlotRef, frame: Frame },
    /// A floor point (the caller's projection), or the anchor as the last resort.
    Floor(Vec3),
}

fn covered(p: Vec3, props: &[Vec3], clearance: f32) -> bool {
    props.iter().any(|q| q.distance(p) < clearance)
}

fn usable<O>(slot: SlotRef, e: &IndexedSlot<O>, q: &SpotQuery<'_>, occ: &SpotBlockers<'_>, p: &InteractParams) -> bool {
    let placement = slot.placement;
    !occ.moving.contains(&placement)
        && occ.claims.hand_holder(placement).is_none()
        && occ.claims.claim_on(slot).is_none_or(|c| c.user == q.user)
        && !covered(e.world.translation, occ.props, p.place_clearance_m)
}

/// The usable put-down slots within `q.radius_m` of the anchor whose tags pass `q.filter` and whose
/// `users` pass `q.tags`, with their world frames, nearest first (then `SlotRef`).
#[must_use]
pub fn spot_candidates<O: Clone>(
    index: &PointIndex<O>,
    q: &SpotQuery<'_>,
    occ: &SpotBlockers<'_>,
    p: &InteractParams,
) -> Vec<(SlotRef, Frame)> {
    let at = q.anchor.at;
    let mut out: Vec<(f32, SlotRef, Frame)> = Vec::new();
    let mut consider = |slot: SlotRef, e: &IndexedSlot<O>, d: f32| {
        if usable(slot, e, q, occ, p) {
            out.push((d, slot, e.world));
        }
    };
    if let Some(placement) = q.anchor.placement {
        for slot in index.placement_slots(placement) {
            let Some(e) = index.get(slot) else { continue };
            let d = e.world.translation.distance(at);
            if d <= q.radius_m && q.filter.matches(&e.tags) && e.users.matches(q.tags) {
                consider(slot, e, d);
            }
        }
    } else {
        let user = SlotUser { tags: q.tags, need: None };
        for (slot, _) in index.find_slots(at, q.radius_m, q.filter, user, |_| 1.0) {
            let Some(e) = index.get(slot) else { continue };
            consider(slot, e, e.world.translation.distance(at));
        }
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    out.into_iter().map(|(_, s, f)| (s, f)).collect()
}

/// Spiral point `k` around `at` in the horizontal plane (x, z) at `at.y`: radius `spread × √k`,
/// angle `k × golden angle`.
#[must_use]
#[allow(clippy::cast_precision_loss)] // k ≤ 255
pub fn spiral_point(at: Vec3, k: u32, spread_m: f32) -> Vec3 {
    let kf = k as f32;
    let (r, a) = (spread_m * kf.sqrt(), kf * GOLDEN_ANGLE);
    at + Vec3::new(r * a.cos(), 0.0, r * a.sin())
}

/// The nearest candidate slot; none: the first of `interact.place.floor_tries` spiral points that
/// `floor` projects and that lies clear of every resting prop; none passes: the anchor.
#[must_use]
pub fn pick_spot<O: Clone>(
    index: &PointIndex<O>,
    q: &SpotQuery<'_>,
    occ: &SpotBlockers<'_>,
    p: &InteractParams,
    floor: impl Fn(Vec3) -> Option<Vec3>,
) -> Spot {
    if let Some(&(slot, frame)) = spot_candidates(index, q, occ, p).first() {
        return Spot::Slot { slot, frame };
    }
    (0..u32::from(p.place_floor_tries))
        .filter_map(|k| floor(spiral_point(q.anchor.at, k, p.place_spread_m)))
        .find(|f| !covered(*f, occ.props, p.place_clearance_m))
        .map_or(Spot::Floor(q.anchor.at), Spot::Floor)
}
