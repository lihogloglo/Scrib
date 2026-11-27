//! `IN14b` headless tests: the tick's door toggles, light changes and released breaks become
//! [`WorldChanged`]s, one per transition. A door opened then closed on two ticks is `Opened` then
//! `Closed` (owner and home from [`ChangeOwners`], the user and verb only when someone toggled it);
//! opened and closed in one tick is none; an unregistered door none. A snuff is `Snuffed` at the flame
//! with the verb as cause. A hit that breaks a registered jar is one `Destroyed` with the hit's damage
//! kind as cause; a break whose placement left counts only if it removes, at the hit point.
//! `DoorChanged`, `LightChanged` and the unregistered `Destroyed` are written directly (their
//! producers: `tests_door`, `tests_light`, `tests_destroy`).

use glam::{Quat, Vec3};
use sf_ids::ActorKey;
use sf_interaction::{
    CHANGE_CLOSED, CHANGE_DESTROYED, CHANGE_OPENED, CHANGE_SNUFFED, ChangeDetail, ChangeOwner, TagId, VERB_OPEN,
    VERB_SNUFF, WorldChange,
};

use super::tests_noise::{base, door, insert, jar, now, player};
use super::*;

const DOOR: PlacementId = PlacementId(20);
const JAR: PlacementId = PlacementId(21);
const LAMP: PlacementId = PlacementId(22);
const HOUSE: PlacementId = PlacementId(99);
const BLUNT: TagId = TagId::of("blunt");

#[derive(Resource, Default)]
struct Seen(Vec<WorldChange>);

fn see(mut changes: MessageReader<WorldChanged>, mut seen: ResMut<Seen>) {
    seen.0.extend(changes.read().map(|c| c.0));
}

fn app() -> App {
    let mut app = base();
    app.init_resource::<Seen>().add_systems(FixedUpdate, see.after(WorldChangeSystems));
    app
}

/// One fixed tick; the changes written on it.
fn tick(app: &mut App) -> Vec<WorldChange> {
    app.world_mut().run_schedule(FixedUpdate);
    std::mem::take(&mut app.world_mut().resource_mut::<Seen>().0)
}

fn toggle(app: &mut App, open: bool, user: Option<sf_interaction::UserKey>) {
    let t = now(app);
    app.world_mut().write_message(DoorChanged(sf_interaction::DoorChanged { placement: DOOR, open, user, tick: t }));
}

#[test]
fn a_door_opened_then_closed_is_opened_then_closed_and_toggled_back_in_one_tick_none() {
    let mut app = app();
    let object = Frame { translation: Vec3::new(4.0, 0.0, -2.0), rotation: Quat::from_rotation_y(1.0) };
    insert(&mut app, DOOR, door(None), object);
    let owner = Some(ChangeOwner::Actor(ActorKey(5)));
    app.world_mut().resource_mut::<ChangeOwners>().insert(DOOR, ChangeOwnership { owner, home: Some(HOUSE) });

    let t = now(&app);
    toggle(&mut app, true, Some(player()));
    assert_eq!(
        tick(&mut app),
        vec![WorldChange {
            tick: t,
            kind: CHANGE_OPENED,
            object: DOOR,
            at: object.translation,
            culprit: Some(player()),
            owner,
            home: Some(HOUSE),
            cause: Some(VERB_OPEN),
            detail: ChangeDetail::None,
        }]
    );
    // A close-behind: no user, no cause.
    let t = now(&app);
    toggle(&mut app, false, None);
    let closed = tick(&mut app);
    assert_eq!(
        closed.iter().map(|c| (c.tick, c.kind, c.culprit, c.cause, c.owner)).collect::<Vec<_>>(),
        vec![(t, CHANGE_CLOSED, None, None, owner)]
    );
    // Opened and closed again within one tick: the door ends as it started.
    toggle(&mut app, true, Some(player()));
    toggle(&mut app, false, Some(player()));
    assert_eq!(tick(&mut app), vec![]);
    // An unregistered door: none.
    let t = now(&app);
    app.world_mut().write_message(DoorChanged(sf_interaction::DoorChanged {
        placement: PlacementId(77),
        open: true,
        user: None,
        tick: t,
    }));
    assert_eq!(tick(&mut app), vec![]);
}

#[test]
fn a_snuff_is_snuffed_at_the_flame_with_the_verb() {
    let mut app = app();
    let t = now(&app);
    let flame = Vec3::new(1.0, 2.0, 3.0);
    app.world_mut().write_message(LightChanged(sf_interaction::LightChanged {
        tick: t,
        placement: LAMP,
        lit: false,
        at: flame,
        culprit: Some(player()),
    }));
    assert_eq!(
        tick(&mut app),
        vec![WorldChange {
            tick: t,
            kind: CHANGE_SNUFFED,
            object: LAMP,
            at: flame,
            culprit: Some(player()),
            owner: None,
            home: None,
            cause: Some(VERB_SNUFF),
            detail: ChangeDetail::None,
        }]
    );
}

#[test]
fn a_break_is_destroyed_with_the_hits_damage_kind_and_a_left_placement_only_if_it_removes() {
    let mut app = app();
    let object = Frame { translation: Vec3::new(1.0, 0.5, 3.0), rotation: Quat::IDENTITY };
    insert(&mut app, JAR, jar(None, 1.0), object);
    // The hit lands in the next tick (the tick advances first).
    let t = now(&app).plus(1);
    let hit = Vec3::new(1.2, 0.6, 3.0);
    app.world_mut().write_message(PropDamage {
        placement: JAR,
        amount: 5.0,
        kind: BLUNT,
        point: hit,
        dir: Vec3::NEG_Y,
        culprit: Some(player()),
    });
    assert_eq!(
        tick(&mut app),
        vec![WorldChange {
            tick: t,
            kind: CHANGE_DESTROYED,
            object: JAR,
            at: object.translation,
            culprit: Some(player()),
            owner: None,
            home: None,
            cause: Some(BLUNT),
            detail: ChangeDetail::Stage(None),
        }]
    );
    // Released after its placement left (no def to count stages): only a removing stage counts.
    let gone = |stage: u8, removes: bool| {
        Destroyed(sf_interaction::Destroyed {
            tick: t,
            placement: PlacementId(78),
            stage,
            point: hit,
            dir: Vec3::NEG_Y,
            seed: 1,
            culprit: None,
            overkill: 0.0,
            removes,
            kind: BLUNT,
        })
    };
    app.world_mut().write_message(gone(0, false));
    assert_eq!(tick(&mut app), vec![]);
    app.world_mut().write_message(gone(1, true));
    let left = tick(&mut app);
    assert_eq!(
        left.iter().map(|c| (c.object, c.at, c.culprit, c.cause, c.detail)).collect::<Vec<_>>(),
        vec![(PlacementId(78), hit, None, Some(BLUNT), ChangeDetail::Stage(None))]
    );
}
