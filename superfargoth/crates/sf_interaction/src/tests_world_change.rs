//! `IN14a` `WorldChange` core: one event per transition (a door opened twice in one tick is one `Opened`, open
//! then close in one tick none), culprit, owner and home copied, the cause from the verb or the rule, only the
//! final damage stage a `Destroyed`, `Doused` no change, and the shipped kinds distinct.

use glam::Vec3;
use sf_ids::{ActorKey, PlayerKey};
use sf_world::PlacementId;

use crate::*;

const DOOR: PlacementId = PlacementId(7);
const LAMP: PlacementId = PlacementId(8);
const CRATE: PlacementId = PlacementId(9);

fn player() -> UserKey {
    UserKey::Player(PlayerKey(1))
}

fn site() -> ChangeSite {
    ChangeSite {
        at: Vec3::new(10.0, 0.0, -4.0),
        owner: Some(ChangeOwner::Actor(ActorKey(42))),
        home: Some(PlacementId(3)),
    }
}

fn door(tick: u64, open: bool, user: Option<UserKey>) -> WorldChange {
    from_door(
        &DoorChanged {
            placement: DOOR,
            open,
            user,
            tick: Tick(tick),
        },
        &site(),
    )
}

fn folded(mut v: Vec<WorldChange>) -> Vec<WorldChange> {
    fold_tick(&mut v);
    v
}

#[test]
fn shipped_kinds_are_distinct() {
    for (i, a) in SHIPPED_KINDS.iter().enumerate() {
        assert!(
            SHIPPED_KINDS[i + 1..].iter().all(|b| a != b),
            "kind {i} collides"
        );
    }
}

#[test]
fn door_open_copies_culprit_owner_home_and_verb() {
    let c = door(5, true, Some(player()));
    assert_eq!(
        c,
        WorldChange {
            tick: Tick(5),
            kind: CHANGE_OPENED,
            object: DOOR,
            at: Vec3::new(10.0, 0.0, -4.0),
            culprit: Some(player()),
            owner: Some(ChangeOwner::Actor(ActorKey(42))),
            home: Some(PlacementId(3)),
            cause: Some(VERB_OPEN),
            detail: ChangeDetail::None,
        }
    );
    let behind = door(6, false, None);
    assert_eq!(
        (behind.kind, behind.culprit, behind.cause),
        (CHANGE_CLOSED, None, None)
    );
}

#[test]
fn one_event_per_transition() {
    // Opened twice in one tick: one `Opened`.
    let twice = folded(vec![
        door(5, true, Some(player())),
        door(5, true, Some(player())),
    ]);
    assert_eq!(twice, vec![door(5, true, Some(player()))]);
    // Open then close in one tick: the door ends as it started, no change.
    assert_eq!(
        folded(vec![
            door(5, true, Some(player())),
            door(5, false, Some(player()))
        ]),
        vec![]
    );
    // Open, close, open: one `Opened`, the last one's culprit.
    let other = UserKey::Actor(ActorKey(9));
    assert_eq!(
        folded(vec![
            door(5, true, Some(player())),
            door(5, false, None),
            door(5, true, Some(other))
        ]),
        vec![door(5, true, Some(other))]
    );
    // Different ticks are different transitions; other objects keep their order around them.
    let lamp = from_light(
        &LightChanged {
            tick: Tick(5),
            placement: LAMP,
            lit: false,
            at: Vec3::Y,
            culprit: None,
        },
        &site(),
        Some(TagId::of("water")),
    );
    assert_eq!(
        folded(vec![door(5, true, None), lamp, door(6, false, None)]),
        vec![door(5, true, None), lamp, door(6, false, None)]
    );
}

#[test]
fn light_cause_is_the_verb_or_the_rule() {
    let water = TagId::of("water");
    let light = |tick, lit, culprit| {
        from_light(
            &LightChanged {
                tick: Tick(tick),
                placement: LAMP,
                lit,
                at: Vec3::new(1.0, 2.0, 3.0),
                culprit,
            },
            &site(),
            Some(water),
        )
    };
    let snuff = light(4, false, Some(player()));
    assert_eq!(
        (snuff.kind, snuff.at, snuff.cause, snuff.culprit),
        (
            CHANGE_SNUFFED,
            Vec3::new(1.0, 2.0, 3.0),
            Some(VERB_SNUFF),
            Some(player())
        )
    );
    let rain = light(4, false, None);
    assert_eq!((rain.culprit, rain.cause), (None, Some(water)));
    let relit = light(9, true, Some(player()));
    assert_eq!(
        (relit.kind, relit.cause),
        (CHANGE_RELIT, Some(VERB_RELIGHT))
    );
    // Snuffed and relit within one tick: no change.
    assert_eq!(folded(vec![snuff, light(4, true, Some(player()))]), vec![]);
}

fn destroyed(stage: u8, removes: bool) -> Destroyed {
    Destroyed {
        tick: Tick(11),
        placement: CRATE,
        stage,
        point: Vec3::ZERO,
        dir: Vec3::X,
        seed: 1,
        culprit: Some(player()),
        overkill: 0.0,
        removes,
    }
}

#[test]
fn only_the_final_stage_is_destroyed() {
    let s = site();
    assert_eq!(
        from_destroyed(&destroyed(0, false), 3, &s, Some(DAMAGE_IMPACT)),
        None
    );
    assert_eq!(
        from_destroyed(&destroyed(1, false), 3, &s, Some(DAMAGE_IMPACT)),
        None
    );
    let last =
        from_destroyed(&destroyed(2, false), 3, &s, Some(DAMAGE_IMPACT)).expect("final stage");
    assert_eq!(
        (
            last.kind,
            last.object,
            last.culprit,
            last.cause,
            last.detail,
            last.owner
        ),
        (
            CHANGE_DESTROYED,
            CRATE,
            Some(player()),
            Some(DAMAGE_IMPACT),
            ChangeDetail::Stage(Some(2)),
            s.owner
        )
    );
    // A stage with no model removes the object, final or not.
    let gone = from_destroyed(&destroyed(1, true), 3, &s, None).expect("removed");
    assert_eq!(gone.detail, ChangeDetail::Stage(None));
    // Two final stages in one tick (an overkill): one `Destroyed`, the first.
    assert_eq!(folded(vec![last, gone]), vec![last]);
}

#[test]
fn fire_ignites_and_burns_doused_is_none() {
    let torch = TagId::of("torch");
    let ev = |output| FireEvent {
        placement: CRATE,
        output,
    };
    let lit = from_fire(
        &ev(FireOutput::Ignited),
        Tick(20),
        &site(),
        Some(player()),
        Some(torch),
    )
    .expect("ignited");
    assert_eq!(
        (
            lit.kind,
            lit.tick,
            lit.culprit,
            lit.cause,
            lit.at,
            lit.detail
        ),
        (
            CHANGE_IGNITED,
            Tick(20),
            Some(player()),
            Some(torch),
            site().at,
            ChangeDetail::None
        )
    );
    let burnt = from_fire(
        &ev(FireOutput::Burnt { stage: Some(2) }),
        Tick(80),
        &site(),
        Some(player()),
        Some(torch),
    )
    .expect("burnt");
    assert_eq!(
        (burnt.kind, burnt.detail),
        (CHANGE_BURNT, ChangeDetail::Stage(Some(2)))
    );
    assert_eq!(
        from_fire(&ev(FireOutput::Doused), Tick(30), &site(), None, None),
        None
    );
}
