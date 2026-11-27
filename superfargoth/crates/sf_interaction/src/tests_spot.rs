//! `IN46b` errand spots: nearest free matching slot, the placement-only anchor, other users' claims
//! skipped (own kept), resting props and moving placements, the floor spiral, the anchor last
//! resort, and the same spot from an index built in another order.

use glam::{Quat, Vec3};
use sf_ids::ActorKey;
use sf_records::RecordId;
use sf_world::PlacementId;
use smallvec::{SmallVec, smallvec};

use crate::*;

fn tag(s: &str) -> TagId {
    TagId::of(s)
}

fn at(x: f32, y: f32, z: f32) -> Frame {
    Frame { translation: Vec3::new(x, y, z), rotation: Quat::IDENTITY }
}

fn slot(kind: SlotKind, verb: &str, local: Frame) -> SlotDef {
    SlotDef {
        kind,
        local,
        band: 0.45,
        verb: VerbDef { verb: tag(verb), priority: 0, requires: TagFilter::default() },
        entrances: SmallVec::new(),
        clips: ClipFamily("place".into()),
        ik: IkTargets::default(),
        reach: 0.0,
        occupancy: Occupancy::Pose,
        control: None,
        users: TagFilter::default(),
        group: None,
        queue: None,
    }
}

/// A shelf: one surface board 1 m up; a chair: one seat (never a spot).
fn shelf() -> InteractableDef {
    let board = slot(SlotKind::Surface { family: tag("place.shelf") }, "place", at(0.0, 1.0, 0.0));
    InteractableDef { id: RecordId::new("shelf"), slots: vec![board], verbs: smallvec![], destructible: None, carry: None, adverts: vec![] }
}

fn chair() -> InteractableDef {
    let seat = slot(SlotKind::Pose { family: tag("seat") }, "sit", at(0.0, 0.0, 0.0));
    InteractableDef { id: RecordId::new("chair"), slots: vec![seat], verbs: smallvec![], destructible: None, carry: None, adverts: vec![] }
}

fn p(n: u128) -> PlacementId {
    PlacementId(n)
}

fn s(n: u128, i: u8) -> SlotRef {
    SlotRef { placement: p(n), slot: i }
}

fn actor(n: u128) -> UserKey {
    UserKey::Actor(ActorKey(n))
}

fn place() -> TagFilter {
    TagFilter { any: smallvec![tag("place")], ..TagFilter::default() }
}

/// A chair at 0.5 m (nearest, wrong kind), shelves 1 and 2 at x = 1 and x = 2, shelf 3 at x = 9
/// (outside the 5 m radius), inserted in `order`.
fn room(order: &[u128]) -> PointIndex<u32> {
    let mut ix = PointIndex::new();
    for &n in order {
        let (d, x) = match n {
            0 => (chair(), 0.5),
            1 => (shelf(), 1.0),
            2 => (shelf(), 2.0),
            _ => (shelf(), 9.0),
        };
        ix.insert_placement(p(n), &d, &at(x, 0.0, 0.0), &0, |_| true);
    }
    ix
}

fn query(filter: &TagFilter) -> SpotQuery<'_> {
    SpotQuery {
        anchor: SpotAnchor { at: Vec3::ZERO, placement: None },
        filter,
        user: actor(1),
        tags: &[],
        radius_m: 5.0,
    }
}

fn picked(spot: Spot) -> SlotRef {
    match spot {
        Spot::Slot { slot, .. } => slot,
        Spot::Floor(f) => panic!("floor {f}"),
    }
}

/// A flat floor at y = 0.
fn flat(v: Vec3) -> Vec3 {
    Vec3::new(v.x, 0.0, v.z)
}

#[allow(clippy::unnecessary_wraps)] // the `floor` signature
fn floor(v: Vec3) -> Option<Vec3> {
    Some(flat(v))
}

#[test]
fn nearest_free_matching_slot_and_the_placement_anchor() {
    let ix = room(&[0, 1, 2, 3]);
    let claims = Claims::new(InteractParams::default());
    let pr = InteractParams::default();
    let occ = SpotBlockers { claims: &claims, props: &[], moving: &[] };
    let f = place();
    let q = query(&f);
    let cands = spot_candidates(&ix, &q, &occ, &pr);
    assert_eq!(cands.iter().map(|c| c.0).collect::<Vec<_>>(), vec![s(1, 0), s(2, 0)], "chair and far shelf out");
    assert_eq!(cands[0].1.translation, Vec3::new(1.0, 1.0, 0.0), "the board's world frame");
    assert_eq!(pick_spot(&ix, &q, &occ, &pr, floor), Spot::Slot { slot: s(1, 0), frame: at(1.0, 1.0, 0.0) });
    let only_two = SpotQuery { anchor: SpotAnchor { at: Vec3::ZERO, placement: Some(p(2)) }, ..q };
    assert_eq!(picked(pick_spot(&ix, &only_two, &occ, &pr, floor)), s(2, 0), "this shelf only");
}

#[test]
fn another_users_claim_is_skipped_and_the_own_kept() {
    let ix = room(&[0, 1, 2, 3]);
    let pr = InteractParams::default();
    let f = place();
    let q = query(&f);
    let terms = SlotTerms::of(&shelf().slots[0], &pr);
    for (holder, want) in [(actor(2), s(2, 0)), (actor(1), s(1, 0))] {
        let mut claims = Claims::new(pr);
        claims.claim(s(1, 0), &terms, holder, &[], pr.priority_ambient, Tick(0)).expect("free slot");
        let occ = SpotBlockers { claims: &claims, props: &[], moving: &[] };
        assert_eq!(picked(pick_spot(&ix, &q, &occ, &pr, floor)), want, "held by {holder:?}");
    }
}

#[test]
fn resting_props_and_moving_placements_block_their_slots() {
    let ix = room(&[0, 1, 2, 3]);
    let claims = Claims::new(InteractParams::default());
    let pr = InteractParams::default();
    let f = place();
    let q = query(&f);
    // A crate 0.2 m off board 1 (within the 0.35 m clearance); one 0.5 m off board 2 (clear).
    let props = [Vec3::new(1.2, 1.0, 0.0), Vec3::new(2.0, 1.5, 0.0)];
    let occ = SpotBlockers { claims: &claims, props: &props, moving: &[] };
    assert_eq!(picked(pick_spot(&ix, &q, &occ, &pr, floor)), s(2, 0));
    let occ = SpotBlockers { claims: &claims, props: &[], moving: &[p(1)] };
    assert_eq!(picked(pick_spot(&ix, &q, &occ, &pr, floor)), s(2, 0), "the carried shelf's own board");
}

#[test]
fn none_in_radius_takes_the_first_valid_spiral_floor_point() {
    let ix = room(&[0, 1, 2, 3]);
    let claims = Claims::new(InteractParams::default());
    let pr = InteractParams::default();
    let f = place();
    let far = SpotQuery { anchor: SpotAnchor { at: Vec3::new(-20.0, 0.5, 0.0), placement: None }, ..query(&f) };
    assert!(spot_candidates(&ix, &far, &SpotBlockers { claims: &claims, props: &[], moving: &[] }, &pr).is_empty());
    // A prop on the anchor refuses point 0; the floor refuses point 1; point 2 is the spot.
    let props = [Vec3::new(-20.0, 0.0, 0.0)];
    let occ = SpotBlockers { claims: &claims, props: &props, moving: &[] };
    let p1 = flat(spiral_point(far.anchor.at, 1, pr.place_spread_m));
    let p2 = flat(spiral_point(far.anchor.at, 2, pr.place_spread_m));
    assert!((p2 - Vec3::new(-20.0, 0.0, 0.0)).length() > pr.place_clearance_m, "point 2 clears the prop");
    let refuse_p1 = |v: Vec3| Some(flat(v)).filter(|f| f.distance(p1) > 1e-4);
    assert_eq!(pick_spot(&ix, &far, &occ, &pr, refuse_p1), Spot::Floor(p2));
    // Radius √k × spread, golden-angle turns.
    let r2 = Vec3::new(p2.x + 20.0, 0.0, p2.z).length();
    assert!((r2 - pr.place_spread_m * 2.0_f32.sqrt()).abs() < 1e-5, "{r2}");
}

#[test]
fn a_floor_that_refuses_every_point_leaves_the_anchor() {
    let ix = room(&[0, 1, 2, 3]);
    let claims = Claims::new(InteractParams::default());
    let pr = InteractParams::default();
    let f = place();
    let far = SpotQuery { anchor: SpotAnchor { at: Vec3::new(-20.0, 0.5, 0.0), placement: None }, ..query(&f) };
    let occ = SpotBlockers { claims: &claims, props: &[], moving: &[] };
    assert_eq!(pick_spot(&ix, &far, &occ, &pr, |_| None), Spot::Floor(far.anchor.at));
}

#[test]
fn an_index_built_in_another_order_gives_the_same_spot() {
    let claims = Claims::new(InteractParams::default());
    let pr = InteractParams::default();
    let f = place();
    // Anchor halfway between boards 1 and 2: an exact distance tie, broken on SlotRef.
    let mid = SpotQuery { anchor: SpotAnchor { at: Vec3::new(1.5, 1.0, 0.0), placement: None }, ..query(&f) };
    let occ = SpotBlockers { claims: &claims, props: &[], moving: &[] };
    let a = room(&[0, 1, 2, 3]);
    let b = room(&[3, 2, 1, 0]);
    assert_eq!(spot_candidates(&a, &mid, &occ, &pr), spot_candidates(&b, &mid, &occ, &pr));
    assert_eq!(picked(pick_spot(&b, &mid, &occ, &pr, floor)), s(1, 0));
}
