//! `IN0b` query core: `find_slots` (order, radius, filter, need, user filter), moving owners and
//! removal; `rank_verbs` (order, `requires`, refused claims hidden); `FocusStabilizer` (margin,
//! dwell, hold freeze, no flicker); `pick_entrance` / `pick_exit`; the cook stub.
#![allow(clippy::many_single_char_names)] // a, b, s: targets and slots in short scenarios

use glam::{Quat, Vec3};
use sf_ids::ActorKey;
use sf_records::RecordId;
use sf_world::PlacementId;
use smallvec::{SmallVec, smallvec};

use crate::*;

fn tag(s: &str) -> TagId {
    TagId::of(s)
}

fn verb(v: &str, priority: i16, requires: TagFilter) -> VerbDef {
    VerbDef { verb: tag(v), priority, requires }
}

fn any_of(t: &str) -> TagFilter {
    TagFilter { any: smallvec![tag(t)], ..TagFilter::default() }
}

fn none_of(t: &str) -> TagFilter {
    TagFilter { none: smallvec![tag(t)], ..TagFilter::default() }
}

fn at(x: f32, y: f32, z: f32) -> Frame {
    Frame { translation: Vec3::new(x, y, z), rotation: Quat::IDENTITY }
}

fn slot_def(kind: SlotKind, v: VerbDef, local: Frame, entrances: SmallVec<[Entrance; 4]>) -> SlotDef {
    SlotDef {
        kind,
        local,
        band: 0.45,
        verb: v,
        entrances,
        clips: ClipFamily("seat.chair".into()),
        ik: IkTargets::default(),
        reach: 0.0,
        occupancy: Occupancy::Pose,
        control: None,
        users: TagFilter::default(),
        group: None,
        queue: None,
    }
}

fn seat(v: &str) -> SlotDef {
    slot_def(SlotKind::Pose { family: tag("seat") }, verb(v, 0, TagFilter::default()), at(0.0, 0.0, 0.0), smallvec![])
}

fn def(slots: Vec<SlotDef>, verbs: SmallVec<[VerbDef; 2]>, adverts: Vec<(TagId, f32)>) -> InteractableDef {
    InteractableDef { id: RecordId::new("chair"), slots, verbs, destructible: None, carry: None, adverts }
}

fn p(n: u128) -> PlacementId {
    PlacementId(n)
}

fn s(n: u128, i: u8) -> SlotRef {
    SlotRef { placement: p(n), slot: i }
}

/// Linear falloff to zero at 5 m.
fn linear(d: f32) -> f32 {
    1.0 - d / 5.0
}

fn refs(it: impl Iterator<Item = (SlotRef, f32)>) -> Vec<SlotRef> {
    it.map(|(r, _)| r).collect()
}

/// A one-seat chair ("sit") per placement at the given positions; placement 4 is a door ("open").
fn street() -> PointIndex<u32> {
    let mut ix = PointIndex::new();
    let chair = def(vec![seat("sit")], smallvec![], vec![]);
    let door = def(
        vec![slot_def(SlotKind::Light(LightDef::default()), verb("open", 0, TagFilter::default()), at(0.0, 0.0, 0.0), smallvec![])],
        smallvec![],
        vec![],
    );
    ix.insert_placement(p(1), &chair, &at(1.0, 0.0, 0.0), &1, |_| true);
    ix.insert_placement(p(2), &chair, &at(2.0, 0.0, 0.0), &2, |_| true);
    ix.insert_placement(p(3), &chair, &at(10.0, 0.0, 0.0), &3, |_| true);
    ix.insert_placement(p(4), &door, &at(3.0, 0.0, 0.0), &4, |_| true);
    ix.insert_placement(p(5), &chair, &at(-1.0, 0.0, 0.0), &5, |_| true);
    ix.insert_placement(p(6), &chair, &at(0.0, 0.0, -3.0), &6, |_| true);
    ix
}

#[test]
fn find_slots_orders_by_score_then_ref_within_radius_and_filter() {
    let ix = street();
    assert_eq!(ix.len(), 6);
    let found: Vec<(SlotRef, f32)> =
        ix.find_slots(Vec3::ZERO, 5.0, &any_of("sit"), SlotUser::default(), linear).collect();
    // p3 is 10 m out, p4's verb fails the filter; p1 and p5 tie at 1 m (ref order).
    assert_eq!(
        found,
        vec![(s(1, 0), linear(1.0)), (s(5, 0), linear(1.0)), (s(2, 0), linear(2.0)), (s(6, 0), linear(3.0))]
    );
    // The pose family is a tag too; the empty filter passes the door.
    assert_eq!(refs(ix.find_slots(Vec3::ZERO, 5.0, &any_of("seat"), SlotUser::default(), linear)).len(), 4);
    assert_eq!(
        refs(ix.find_slots(Vec3::ZERO, 3.0, &TagFilter::default(), SlotUser::default(), |_| 1.0)),
        vec![s(1, 0), s(2, 0), s(4, 0), s(5, 0), s(6, 0)],
        "equal scores: ref order; 3 m is inside"
    );
    assert_eq!(refs(ix.find_slots(Vec3::ZERO, 0.5, &TagFilter::default(), SlotUser::default(), |_| 1.0)), vec![]);
    // Falloff at or below zero drops the slot (p6 at 3 m under a 2 m linear curve).
    let short = |d: f32| 1.0 - d / 2.0;
    assert_eq!(
        refs(ix.find_slots(Vec3::ZERO, 5.0, &any_of("sit"), SlotUser::default(), short)),
        vec![s(1, 0), s(5, 0)]
    );
}

#[test]
fn find_slots_scores_by_need_and_tests_user_tags() {
    let mut ix: PointIndex<u32> = PointIndex::new();
    let plain = def(vec![seat("sit")], smallvec![], vec![(tag("social"), 1.0)]);
    let bed = def(vec![seat("sleep")], smallvec![], vec![(tag("rest"), 2.0)]);
    let mut big = seat("sit");
    big.users = TagFilter { all: smallvec![tag("biped")], ..TagFilter::default() };
    let biped_only = def(vec![big], smallvec![], vec![(tag("rest"), 1.0)]);
    ix.insert_placement(p(1), &plain, &at(1.0, 0.0, 0.0), &1, |_| true);
    ix.insert_placement(p(2), &bed, &at(4.0, 0.0, 0.0), &2, |_| true);
    ix.insert_placement(p(3), &biped_only, &at(0.0, 1.0, 0.0), &3, |_| true);
    let rest = SlotUser { tags: &[], need: Some(tag("rest")) };
    let found: Vec<(SlotRef, f32)> = ix.find_slots(Vec3::ZERO, 5.0, &TagFilter::default(), rest, linear).collect();
    assert_eq!(found, vec![(s(2, 0), 2.0 * linear(4.0))], "p1 advertises no rest, p3 wants a biped");
    let biped = [tag("biped")];
    let rest_biped = SlotUser { tags: &biped, need: Some(tag("rest")) };
    assert_eq!(
        refs(ix.find_slots(Vec3::ZERO, 5.0, &TagFilter::default(), rest_biped, linear)),
        vec![s(3, 0), s(2, 0)],
        "1 * 0.8 beats 2 * 0.2"
    );
}

#[test]
fn moving_owner_rebuckets_and_recomputes_frames() {
    let mut ix: PointIndex<u32> = PointIndex::new();
    let mut off = seat("sit");
    off.local = at(0.0, 0.5, 1.0);
    let saddle = def(vec![off, seat("sit")], smallvec![], vec![]);
    ix.insert_placement(p(7), &saddle, &at(1.0, 0.0, 0.0), &7, |_| true);
    assert_eq!(ix.get(s(7, 0)).map(|e| e.world.translation), Some(Vec3::new(1.0, 0.5, 1.0)));
    assert_eq!(ix.get(s(7, 0)).map(|e| e.owner), Some(7));
    assert_eq!(ix.move_placement(p(7), &at(20.0, 0.0, 0.0)), 2);
    assert_eq!(refs(ix.find_slots(Vec3::ZERO, 5.0, &TagFilter::default(), SlotUser::default(), |_| 1.0)), vec![]);
    assert_eq!(
        refs(ix.find_slots(Vec3::new(20.0, 0.0, 0.0), 2.0, &TagFilter::default(), SlotUser::default(), |_| 1.0)),
        vec![s(7, 0), s(7, 1)]
    );
    // A turned owner turns its slot offsets.
    let turned = Frame { translation: Vec3::ZERO, rotation: Quat::from_rotation_y(std::f32::consts::FRAC_PI_2) };
    ix.move_placement(p(7), &turned);
    let w = ix.get(s(7, 0)).map_or(Vec3::NAN, |e| e.world.translation);
    assert!(w.distance(Vec3::new(1.0, 0.5, 0.0)) < 1.0e-5, "{w}");
    assert_eq!(ix.move_placement(p(8), &turned), 0, "unknown placement");
}

#[test]
fn remove_and_remove_placement_empty_the_index() {
    let mut ix = street();
    assert!(ix.remove(s(1, 0)).is_some());
    assert!(ix.remove(s(1, 0)).is_none());
    assert_eq!(
        refs(ix.find_slots(Vec3::ZERO, 1.5, &TagFilter::default(), SlotUser::default(), |_| 1.0)),
        vec![s(5, 0)]
    );
    for n in 2..=6 {
        assert_eq!(ix.remove_placement(p(n)), 1);
    }
    assert_eq!(ix.remove_placement(p(2)), 0);
    assert!(ix.is_empty());
    assert_eq!(refs(ix.find_slots(Vec3::ZERO, 50.0, &TagFilter::default(), SlotUser::default(), |_| 1.0)), vec![]);
    // Adapter-disabled slots are not indexed; reinserting a placement replaces its slots.
    let two = def(vec![seat("sit"), seat("sit")], smallvec![], vec![]);
    ix.insert_placement(p(1), &two, &at(0.0, 0.0, 0.0), &1, |i| i != 0);
    assert_eq!((ix.len(), ix.get(s(1, 0)).is_none()), (1, true));
    ix.insert_placement(p(1), &two, &at(0.0, 0.0, 0.0), &1, |_| true);
    assert_eq!(ix.len(), 2);
}

/// sit (5), sleep (10), snuff (10, requires lit); slotless carry (0), take (10), read (3, not lit).
fn bed() -> InteractableDef {
    let mut sit = seat("sit");
    sit.verb.priority = 5;
    let mut sleep = seat("sleep");
    sleep.verb.priority = 10;
    let snuff = slot_def(SlotKind::Light(LightDef::default()), verb("snuff", 10, any_of("lit")), at(0.0, 0.0, 0.0), smallvec![]);
    def(
        vec![sit, sleep, snuff],
        smallvec![
            verb("carry", 0, TagFilter::default()),
            verb("take", 10, TagFilter::default()),
            verb("read", 3, none_of("lit"))
        ],
        vec![],
    )
}

fn verbs_of(choices: &[VerbChoice]) -> Vec<TagId> {
    choices.iter().map(|c| c.verb).collect()
}

#[test]
fn rank_verbs_sorts_by_priority_then_def_order_and_filters_requires() {
    let d = bed();
    let lit = rank_verbs(&d, p(1), &[tag("lit")], |_| true);
    assert_eq!(verbs_of(&lit), vec![tag("sleep"), tag("snuff"), tag("take"), tag("sit"), tag("carry")]);
    assert_eq!(lit[0].target, VerbTarget::Slot(s(1, 1)));
    assert_eq!(lit[2].target, VerbTarget::Slotless { placement: p(1), verb: 1 });
    let dark = rank_verbs(&d, p(1), &[], |_| true);
    assert_eq!(verbs_of(&dark), vec![tag("sleep"), tag("take"), tag("sit"), tag("read"), tag("carry")]);
}

#[test]
fn rank_verbs_hides_slot_verbs_whose_claim_would_be_refused() {
    let d = bed();
    let terms: Vec<SlotTerms> = d.slots.iter().map(|sd| SlotTerms::of(sd, &InteractParams::default())).collect();
    let mut c = Claims::default();
    let other = UserKey::Actor(ActorKey(9));
    let me = UserKey::Actor(ActorKey(1));
    c.claim(s(1, 1), &terms[1], other, &[], 20, Tick(0)).expect("free bed");
    c.occupy(s(1, 1), other, Tick(1)).expect("holder");
    let would = |r: SlotRef| c.would_claim(r, &terms[usize::from(r.slot)], me, &[], 10).is_ok();
    let choices = rank_verbs(&d, p(1), &[tag("lit")], would);
    assert_eq!(verbs_of(&choices), vec![tag("snuff"), tag("take"), tag("sit"), tag("carry")], "sleep is taken");
    // would_claim changed nothing and agrees with claim.
    assert_eq!(c.would_claim(s(1, 1), &terms[1], me, &[], 10), Err(Refusal::Taken));
    assert_eq!(c.claim(s(1, 1), &terms[1], me, &[], 10, Tick(2)), Err(Refusal::Taken));
    assert_eq!(c.would_claim(s(1, 0), &terms[0], me, &[], 10), Ok(()));
    assert!(c.claim_on(s(1, 0)).is_none());
}

/// A shelf with one put-down point (slot 0) and a seat beside it (slot 1).
fn shelf() -> InteractableDef {
    let surface = slot_def(
        SlotKind::Surface { family: tag("place.shelf") },
        verb("place", 5, TagFilter::default()),
        at(0.0, 1.2, 0.0),
        smallvec![],
    );
    def(vec![surface, seat("sit")], smallvec![], vec![])
}

#[test]
fn surface_slot_is_indexed_with_its_family_and_never_offered() {
    let d = shelf();
    let mut ix: PointIndex<u32> = PointIndex::new();
    ix.insert_placement(p(1), &d, &at(2.0, 0.0, 0.0), &1, |_| true);
    let entry = ix.get(s(1, 0)).expect("surface indexed");
    assert_eq!(entry.kind, KindTag::Surface { family: tag("place.shelf") });
    assert_eq!(entry.tags.as_slice(), &[tag("place"), tag("place.shelf")]);
    for f in [any_of("place"), any_of("place.shelf")] {
        assert_eq!(refs(ix.find_slots(Vec3::ZERO, 5.0, &f, SlotUser::default(), linear)), vec![s(1, 0)]);
    }
    let choices = rank_verbs(&d, p(1), &[], |_| true);
    assert_eq!(verbs_of(&choices), vec![tag("sit")], "the surface is a claim target, the seat a use");
    assert_eq!(choices[0].target, VerbTarget::Slot(s(1, 1)));
}

/// 10 Hz: switch 0.15 s = 2 ticks, hold 0.35 s = 3 ticks (0.35 as f32 is just under, 3.4999 rounds down).
fn focus_params() -> InteractParams {
    InteractParams { tick_hz: 10.0, ..InteractParams::default() }
}

#[test]
fn focus_takes_the_best_at_once_and_ignores_leads_below_the_margin() {
    let pr = focus_params();
    let mut f: FocusStabilizer<u32> = FocusStabilizer::new();
    assert_eq!(f.update(&[], false, &pr), None);
    assert_eq!(f.update(&[(2, 1.0), (1, 1.0)], false, &pr), Some(1), "tie: lower key");
    // 1.19 < 1.0 * 1.2: never enough.
    for _ in 0..20 {
        assert_eq!(f.update(&[(1, 1.0), (2, 1.19)], false, &pr), Some(1));
    }
    // The focused target gone: the best now, no dwell.
    assert_eq!(f.update(&[(2, 0.1), (3, 0.5)], false, &pr), Some(3));
}

#[test]
fn focus_switches_after_the_dwell() {
    let pr = focus_params();
    assert_eq!(pr.focus_switch_ticks(), 2);
    let mut f: FocusStabilizer<u32> = FocusStabilizer::new();
    f.update(&[(1, 1.0)], false, &pr);
    assert_eq!(f.update(&[(1, 1.0), (2, 1.3)], false, &pr), Some(1), "first tick of the lead");
    assert_eq!(f.update(&[(1, 1.0), (2, 1.3)], false, &pr), Some(2), "led for 0.15 s");
    // A different challenger restarts the dwell.
    assert_eq!(f.update(&[(2, 1.0), (3, 1.5)], false, &pr), Some(2));
    assert_eq!(f.update(&[(2, 1.0), (4, 1.5)], false, &pr), Some(2));
    assert_eq!(f.update(&[(2, 1.0), (4, 1.5)], false, &pr), Some(4));
}

#[test]
fn focus_freezes_while_interact_is_held() {
    let pr = focus_params();
    let mut f: FocusStabilizer<u32> = FocusStabilizer::new();
    assert_eq!(pr.focus_hold_ticks(), 3);
    f.update(&[(1, 1.0)], false, &pr);
    for i in 1..=10 {
        assert_eq!(f.update(&[(2, 5.0)], true, &pr), Some(1), "held: frozen even with the target gone");
        assert_eq!(f.list_open(&pr), i >= 3, "verb list after 0.35 s held");
    }
    assert_eq!(f.update(&[(1, 1.0), (2, 5.0)], false, &pr), Some(1), "released: the dwell starts over");
    assert!(!f.list_open(&pr));
    assert_eq!(f.update(&[(1, 1.0), (2, 5.0)], false, &pr), Some(2));
    f.clear();
    assert_eq!(f.current(), None);
}

#[test]
fn focus_does_not_flicker_on_alternating_scores() {
    let pr = focus_params();
    let mut f: FocusStabilizer<u32> = FocusStabilizer::new();
    f.update(&[(1, 1.0)], false, &pr);
    for i in 0..40 {
        let b = if i % 2 == 0 { 1.5 } else { 0.8 };
        assert_eq!(f.update(&[(1, 1.0), (2, b)], false, &pr), Some(1), "tick {i}");
    }
}

/// A seat at the origin with an entrance on each side 1 m out; the slot frame sits 0.2 m forward.
fn chair() -> SlotDef {
    slot_def(
        SlotKind::Pose { family: tag("seat") },
        verb("sit", 0, TagFilter::default()),
        at(0.0, 0.0, 0.2),
        smallvec![
            Entrance { side: Side::Back, approach: Vec3::new(0.0, 0.0, -1.0) },
            Entrance { side: Side::Right, approach: Vec3::new(1.0, 0.0, 0.0) },
            Entrance { side: Side::Left, approach: Vec3::new(-1.0, 0.0, 0.0) },
            Entrance { side: Side::Front, approach: Vec3::new(0.0, 0.0, 1.0) },
        ],
    )
}

fn side(e: Option<Entrance>) -> Option<Side> {
    e.map(|e| e.side)
}

#[test]
fn pick_entrance_nearest_valid_with_side_order_ties() {
    let c = chair();
    assert_eq!(side(pick_entrance(&c, Vec3::new(0.0, 0.0, 3.0), |_| true)), Some(Side::Front));
    assert_eq!(side(pick_entrance(&c, Vec3::new(3.0, 0.0, 0.0), |_| true)), Some(Side::Right));
    // Front refused: Left and Right tie at sqrt(10), Back is 4 m: Left by side order.
    let no_front = |e: &Entrance| e.side != Side::Front;
    assert_eq!(side(pick_entrance(&c, Vec3::new(0.0, 0.0, 3.0), no_front)), Some(Side::Left));
    // All four equidistant: Front first.
    assert_eq!(side(pick_entrance(&c, Vec3::ZERO, |_| true)), Some(Side::Front));
    assert_eq!(side(pick_entrance(&c, Vec3::ZERO, |e| e.side == Side::Back)), Some(Side::Back));
    assert_eq!(pick_entrance(&c, Vec3::ZERO, |_| false), None);
    // World to object frame: an object at x = 10 turned half a turn.
    let obj = Frame { translation: Vec3::new(10.0, 0.0, 0.0), rotation: Quat::from_rotation_y(std::f32::consts::PI) };
    let local = to_local(&obj, Vec3::new(10.0, 0.0, -3.0));
    assert!(local.distance(Vec3::new(0.0, 0.0, 3.0)) < 1.0e-5, "{local}");
}

#[test]
fn pick_exit_prefers_the_entry_side_then_nearest_valid() {
    let c = chair();
    assert_eq!(side(pick_exit(&c, Side::Right, |_| true)), Some(Side::Right));
    assert_eq!(side(pick_exit(&c, Side::Back, |_| true)), Some(Side::Back), "entry side beats nearer Front");
    // Right blocked: nearest to the slot frame (0, 0, 0.2) is Front (0.8 m).
    let no_right = |e: &Entrance| e.side != Side::Right;
    assert_eq!(side(pick_exit(&c, Side::Right, no_right)), Some(Side::Front));
    // Right and Front blocked: Left (1.02 m) beats Back (1.2 m).
    let sides = |e: &Entrance| matches!(e.side, Side::Left | Side::Back);
    assert_eq!(side(pick_exit(&c, Side::Right, sides)), Some(Side::Left));
    assert_eq!(pick_exit(&c, Side::Right, |_| false), None);
}

#[test]
fn cook_stub_round_trips_defs() {
    let defs = vec![bed(), def(vec![chair()], smallvec![], vec![(tag("rest"), 1.0)])];
    assert_eq!(cook(defs.clone()), defs);
}
