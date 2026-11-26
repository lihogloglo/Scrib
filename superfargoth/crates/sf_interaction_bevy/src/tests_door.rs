//! `IN33b2a` headless tests: the door registry follows the index changes, an approved use toggles a
//! registered door by the verb rule (one `DoorChanged`, the `door.open` tag kept), the swing side
//! comes from the user's feet in the object's frame, the door tunables are bound live.
//! `IN33b2b`: the swing on the fixed tick (48 steps at 60 Hz, a blocked pause, a reversal), the
//! close-behind armed by the opener's release through the claim table, a removal mid-swing.
//! `UseApproved` is written directly: the activation pipeline is tested in `tests_activate`.

use std::f32::consts::FRAC_PI_2;

use glam::{Quat, Vec3};
use sf_action_bevy::ActionBody;
use sf_controller::{Locomotion, MoverParams};
use sf_coords::WorldPos;
use sf_ids::{ActorKey, PlayerKey};
use sf_interaction::{
    ClipFamily, DOOR_OPEN, DoorDef, IkTargets, Occupancy, SlotDef, SlotKind, SlotTerms, TagFilter, TagId, Tick,
    UserKey, VERB_CLOSE, VERB_OPEN, VerbDef,
};
use sf_records::RecordId;
use sf_script_bevy::PLAYER;
use sf_script_bevy::sf_script::Handle;
use sf_tunables_bevy::{SetBy, SetTunable, TunablesPlugin};
use smallvec::{SmallVec, smallvec};

use super::*;

const DOOR: PlacementId = PlacementId(10);
const CHAIR: PlacementId = PlacementId(7);

fn player() -> UserKey {
    UserKey::Player(PlayerKey(1))
}

fn slot(kind: SlotKind, verb: VerbDef) -> SlotDef {
    SlotDef {
        kind,
        local: Frame::default(),
        band: 0.0,
        verb,
        entrances: smallvec![],
        clips: ClipFamily("door".into()),
        ik: IkTargets::default(),
        reach: 0.0,
        occupancy: Occupancy::Pose,
        control: None,
        users: TagFilter::default(),
        group: None,
        queue: None,
    }
}

fn verb(verb: TagId, requires: TagFilter) -> VerbDef {
    VerbDef { verb, priority: 0, requires }
}

/// Hinge at +0.5 x, axis Y, 90 degrees, doorway at the origin: a positive turn moves the leaf
/// centre toward +z (the +n side). Slotless `open` (no `door.open`) and `close` (`door.open`).
fn door(both_ways: bool) -> Arc<InteractableDef> {
    door_with(both_ways, false)
}

/// [`door`], optionally closing behind its opener.
fn door_with(both_ways: bool, close_behind: bool) -> Arc<InteractableDef> {
    let leaf = DoorDef { hinge: Vec3::new(0.5, 0.0, 0.0), axis: Vec3::Y, open_deg: 90.0, both_ways, close_behind };
    let closed = TagFilter { none: smallvec![DOOR_OPEN], ..TagFilter::default() };
    let open = TagFilter { all: smallvec![DOOR_OPEN], ..TagFilter::default() };
    Arc::new(InteractableDef {
        id: RecordId::new("door"),
        slots: vec![slot(SlotKind::Door(leaf), verb(VERB_OPEN, closed.clone()))],
        verbs: smallvec![verb(VERB_OPEN, closed), verb(VERB_CLOSE, open)],
        destructible: None,
        carry: None,
        adverts: Vec::new(),
    })
}

fn chair() -> Arc<InteractableDef> {
    Arc::new(InteractableDef {
        id: RecordId::new("chair"),
        slots: vec![slot(SlotKind::Pose { family: TagId::of("seat") }, verb(TagId::of("sit"), TagFilter::default()))],
        verbs: SmallVec::new(),
        destructible: None,
        carry: None,
        adverts: Vec::new(),
    })
}

#[derive(Resource, Default)]
struct Log(Vec<sf_interaction::DoorChanged>);

fn gather(mut log: ResMut<Log>, mut changed: MessageReader<DoorChanged>) {
    log.0.extend(changed.read().map(|c| c.0));
}

fn insert(app: &mut App, placement: PlacementId, def: Arc<InteractableDef>, object: Frame) {
    let owner = app.world_mut().spawn_empty().id();
    app.world_mut().write_message(IndexChange::Insert { placement, def, object, owner, disabled: SmallVec::new() });
    app.world_mut().run_schedule(FixedUpdate);
}

/// The door and a chair resident, after two fixed ticks (the tick is `Tick(2)`).
fn app_with(door_def: Arc<InteractableDef>, object: Frame) -> App {
    let mut app = App::new();
    app.add_plugins((TunablesPlugin, InteractionPlugin))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .init_resource::<Log>()
        .add_systems(Update, gather.after(DoorSystems::Default));
    // Binds the tunables; no `TimePlugin`, so the fixed loop runs only when a test asks.
    app.update();
    insert(&mut app, DOOR, door_def, object);
    insert(&mut app, CHAIR, chair(), Frame::default());
    app
}

fn app() -> App {
    app_with(door(false), Frame::default())
}

/// The player as a slot user with no body.
fn spawn_player(app: &mut App) {
    app.world_mut().spawn(InteractionUser { user: player(), actor: PLAYER, priority: 30, tags: SmallVec::new() });
}

/// One approved use by the player, then one frame; this frame's `DoorChanged`s.
fn approve(app: &mut App, target: PlacementId, verb: Option<TagId>) -> Vec<sf_interaction::DoorChanged> {
    let activation = Activated { target: Handle(target.0), actor: PLAYER, verb: verb.map(|v| v.0) };
    app.world_mut().write_message(UseApproved { activation, verb, slot: None });
    app.world_mut().resource_mut::<Log>().0.clear();
    app.update();
    app.world().resource::<Log>().0.clone()
}

fn state(app: &App) -> Option<sf_interaction::DoorState> {
    app.world().resource::<Doors>().get(DOOR).map(|d| d.state)
}

fn tagged(app: &App) -> bool {
    app.world().resource::<InteractionStateTags>().tags(DOOR).contains(&DOOR_OPEN)
}

fn changed(open: bool, user: Option<UserKey>) -> sf_interaction::DoorChanged {
    sf_interaction::DoorChanged { placement: DOOR, open, user, tick: Tick(2) }
}

// --- Tests --------------------------------------------------------------------------------------

#[test]
fn an_approved_use_opens_a_closed_door_and_tags_it() {
    let mut app = app();
    spawn_player(&mut app);
    assert_eq!(app.world().resource::<Doors>().len(), 1, "the chair is no door");
    assert_eq!(state(&app).map(|s| s.open), Some(false));
    assert_eq!(approve(&mut app, DOOR, None), vec![changed(true, Some(player()))]);
    let s = state(&app).unwrap();
    assert!(s.open);
    assert_eq!(s.opener, Some(player()));
    assert!(tagged(&app));
}

#[test]
fn a_second_approval_closes_it_and_clears_the_tag() {
    let mut app = app();
    spawn_player(&mut app);
    approve(&mut app, DOOR, None);
    assert_eq!(approve(&mut app, DOOR, None), vec![changed(false, Some(player()))]);
    assert_eq!(state(&app).map(|s| s.open), Some(false));
    assert!(!tagged(&app));
    assert!(app.world().resource::<InteractionStateTags>().tags(DOOR).is_empty());
}

#[test]
fn open_and_close_act_only_on_the_other_state() {
    let mut app = app();
    spawn_player(&mut app);
    let closed = state(&app);
    assert_eq!(approve(&mut app, DOOR, Some(VERB_CLOSE)), vec![], "close on a closed door");
    assert_eq!(state(&app), closed);
    assert_eq!(approve(&mut app, DOOR, Some(VERB_OPEN)), vec![changed(true, Some(player()))]);
    let open = state(&app);
    assert_eq!(approve(&mut app, DOOR, Some(VERB_OPEN)), vec![], "open on an open door");
    assert_eq!(state(&app), open);
    assert!(tagged(&app));
    assert_eq!(approve(&mut app, DOOR, Some(VERB_CLOSE)), vec![changed(false, Some(player()))]);
    assert!(!tagged(&app));
}

#[test]
fn another_verb_does_nothing() {
    let mut app = app();
    spawn_player(&mut app);
    assert_eq!(approve(&mut app, DOOR, Some(TagId::of("sit"))), vec![]);
    assert_eq!(state(&app), Some(sf_interaction::DoorState::default()));
    assert!(!tagged(&app));
}

#[test]
fn a_use_on_a_non_door_writes_nothing() {
    let mut app = app();
    spawn_player(&mut app);
    assert_eq!(approve(&mut app, CHAIR, None), vec![]);
    assert_eq!(approve(&mut app, PlacementId(99), None), vec![], "not resident at all");
    assert_eq!(state(&app), Some(sf_interaction::DoorState::default()));
}

#[test]
fn a_script_use_toggles_with_no_user() {
    let mut app = app();
    assert_eq!(approve(&mut app, DOOR, None), vec![changed(true, None)]);
    assert_eq!(state(&app).and_then(|s| s.opener), None);
}

#[test]
fn removal_drops_the_door_and_its_tag_and_a_reinsert_keeps_the_state() {
    let mut app = app();
    spawn_player(&mut app);
    approve(&mut app, DOOR, None);
    let open = state(&app).unwrap();
    assert!(open.open);

    let moved = Frame { translation: Vec3::new(3.0, 0.0, 0.0), rotation: Quat::IDENTITY };
    insert(&mut app, DOOR, door(false), moved);
    // The insert's fixed tick also swings the kept state one step.
    let p = InteractParams::default();
    assert_eq!(state(&app), Some(open.stepped(1.0 / 60.0, p.door_swing_s)), "a re-insert keeps the state");
    assert_eq!(app.world().resource::<Doors>().get(DOOR).map(|d| d.object), Some(moved));
    assert!(tagged(&app));

    let shifted = Frame { translation: Vec3::new(4.0, 0.0, 0.0), rotation: Quat::IDENTITY };
    app.world_mut().write_message(IndexChange::Move { placement: DOOR, object: shifted });
    app.world_mut().run_schedule(FixedUpdate);
    assert_eq!(app.world().resource::<Doors>().get(DOOR).map(|d| d.object), Some(shifted));

    app.world_mut().write_message(IndexChange::unload(DOOR));
    app.world_mut().run_schedule(FixedUpdate);
    assert!(app.world().resource::<Doors>().get(DOOR).is_none());
    assert!(!tagged(&app));

    insert(&mut app, DOOR, door(false), Frame::default());
    assert_eq!(state(&app), Some(sf_interaction::DoorState::default()), "a new door starts closed");
}

#[test]
fn a_reinsert_without_a_door_slot_drops_the_door_and_its_tag() {
    let mut app = app();
    spawn_player(&mut app);
    approve(&mut app, DOOR, None);
    assert!(tagged(&app));
    insert(&mut app, DOOR, chair(), Frame::default());
    assert!(app.world().resource::<Doors>().get(DOOR).is_none());
    assert!(!tagged(&app));
}

#[test]
fn the_door_tunables_are_bound_live() {
    let mut app = app();
    let d = InteractParams::default();
    let p = app.world().resource::<InteractionSettings>().0;
    assert_eq!((p.door_swing_s.to_bits(), p.door_close_behind_s.to_bits()), (d.door_swing_s.to_bits(), d.door_close_behind_s.to_bits()));
    app.world_mut().write_message(SetTunable::new(DOOR_SWING_S.name(), "1.5", SetBy::Console));
    app.world_mut().write_message(SetTunable::new(DOOR_CLOSE_BEHIND_S.name(), "4", SetBy::Console));
    app.update();
    let p = app.world().resource::<InteractionSettings>().0;
    assert_eq!((p.door_swing_s.to_bits(), p.door_close_behind_s.to_bits()), (1.5_f32.to_bits(), 4.0_f32.to_bits()));
}

#[test]
fn a_both_ways_door_swings_away_from_a_user_with_a_body() {
    // The object turned a quarter about Y at (10, 0, 5): its local +z (the +n side) is world +x.
    let object = Frame { translation: Vec3::new(10.0, 0.0, 5.0), rotation: Quat::from_rotation_y(FRAC_PI_2) };
    let mut app = app_with(door(true), object);
    let feet = Vec3::new(11.0, 0.0, 5.0);
    let body = ActionBody::new(Locomotion::new(WorldPos::from_f64(feet.as_dvec3()), 0.0), MoverParams::default(), WorldPos::ORIGIN);
    let user = InteractionUser { user: player(), actor: PLAYER, priority: 30, tags: SmallVec::new() };
    app.world_mut().spawn((body, user));
    assert_eq!(approve(&mut app, DOOR, None), vec![changed(true, Some(player()))]);
    assert_eq!(state(&app).map(|s| s.sign), Some(-1), "away from the user on the +n side");
}

// --- The swing (IN33b2b) ------------------------------------------------------------------------

/// What the fixed ticks wrote, gathered after [`DoorSystems::Commit`].
#[derive(Resource, Default)]
struct Swing {
    moved: Vec<DoorMoved>,
    changed: Vec<sf_interaction::DoorChanged>,
}

/// Ticks left to block every sweep, between Plan and Commit (the host's test).
#[derive(Resource, Default)]
struct BlockTicks(u32);

fn gather_swing(mut swing: ResMut<Swing>, mut moved: MessageReader<DoorMoved>, mut changed: MessageReader<DoorChanged>) {
    swing.moved.extend(moved.read().copied());
    swing.changed.extend(changed.read().map(|c| c.0));
}

fn block_all(mut left: ResMut<BlockTicks>, mut sweeps: ResMut<DoorSweeps>) {
    if left.0 == 0 {
        return;
    }
    left.0 -= 1;
    for (_, sweep) in sweeps.iter_mut() {
        sweep.blocked = true;
    }
}

/// Adds the gatherer and the blocker to an app.
fn track(app: &mut App) {
    app.init_resource::<Swing>()
        .init_resource::<BlockTicks>()
        .add_systems(FixedUpdate, block_all.after(DoorSystems::Plan).before(DoorSystems::Commit))
        .add_systems(FixedUpdate, gather_swing.after(DoorSystems::Commit));
}

/// `runs` fixed ticks; the `DoorMoved`s they wrote.
fn fixed(app: &mut App, runs: usize) -> Vec<DoorMoved> {
    let before = app.world().resource::<Swing>().moved.len();
    for _ in 0..runs {
        app.world_mut().run_schedule(FixedUpdate);
    }
    app.world().resource::<Swing>().moved[before..].to_vec()
}

/// An app with the player, tracked, the door just toggled open (`t` = 0).
fn opened(def: Arc<InteractableDef>) -> App {
    let mut app = app_with(def, Frame::default());
    track(&mut app);
    spawn_player(&mut app);
    assert_eq!(approve(&mut app, DOOR, None).len(), 1);
    app
}

fn t(app: &App) -> f32 {
    state(app).unwrap().t
}

fn leaf_now(app: &App) -> Frame {
    let d = app.world().resource::<Doors>().get(DOOR).unwrap();
    d.state.leaf(d.def.door().unwrap().2, &d.object)
}

fn params(app: &App) -> InteractParams {
    effective_params(app.world().resource::<InteractionSettings>(), Some(app.world().resource::<Time<Fixed>>()))
}

fn claim_request(user: UserKey, release: bool) -> ClaimRequest {
    let slot = SlotRef { placement: DOOR, slot: 0 };
    let terms = SlotTerms { group: None, queue: None, reach: 0.0, users: TagFilter::default(), enabled: true };
    let op = if release { Op::Release } else { Op::Claim { slot, terms, tags: SmallVec::new() } };
    ClaimRequest(Request { user, priority: 30, op })
}

/// `user` claims the door slot on one tick and releases it on the next; the release's tick.
fn pass_through(app: &mut App, user: UserKey) -> Tick {
    let slot = SlotRef { placement: DOOR, slot: 0 };
    app.world_mut().write_message(claim_request(user, false));
    fixed(app, 1);
    assert_eq!(app.world().resource::<InteractionClaims>().0.claim_on(slot).map(|c| c.user), Some(user));
    app.world_mut().write_message(claim_request(user, true));
    fixed(app, 1);
    assert!(app.world().resource::<InteractionClaims>().0.claim_on(slot).is_none());
    app.world().resource::<InteractionTick>().0
}

#[test]
fn a_toggled_door_arrives_after_exactly_48_ticks_at_60_hz() {
    let mut app = opened(door(false));
    let p = params(&app);
    assert_eq!((p.tick_hz.to_bits(), p.door_swing_s.to_bits()), (60.0_f32.to_bits(), 0.8_f32.to_bits()));
    let mut moved = fixed(&mut app, 47);
    assert!(state(&app).unwrap().is_moving());
    moved.extend(fixed(&mut app, 1));
    assert_eq!(moved.len(), 48);
    assert!(moved.iter().all(|m| m.placement == DOOR));
    let s = state(&app).unwrap();
    assert_eq!(s.t.to_bits(), 1.0_f32.to_bits());
    assert!(!s.is_moving());
    assert_eq!(moved.last().map(|m| m.leaf), Some(leaf_now(&app)));
    assert_eq!(fixed(&mut app, 1), vec![], "at rest");
    assert_eq!(app.world().resource::<DoorSweeps>().iter().count(), 0);
}

#[test]
fn a_blocked_door_pauses_and_resumes_from_the_same_t() {
    let mut app = opened(door(false));
    assert_eq!(fixed(&mut app, 10).len(), 10);
    let paused = state(&app).unwrap();
    app.world_mut().resource_mut::<BlockTicks>().0 = 5;
    for _ in 0..5 {
        assert_eq!(fixed(&mut app, 1), vec![], "blocked");
        assert!(app.world().resource::<DoorSweeps>().get(DOOR).is_some_and(|s| s.blocked));
        assert_eq!(state(&app), Some(paused), "t and target kept");
    }
    assert_eq!(fixed(&mut app, 37).len(), 37);
    assert!(state(&app).unwrap().is_moving());
    assert_eq!(fixed(&mut app, 1).len(), 1);
    assert_eq!(t(&app).to_bits(), 1.0_f32.to_bits());
    assert!(!state(&app).unwrap().is_moving());
}

#[test]
fn a_toggle_mid_swing_reverses_and_arrives_at_zero() {
    let mut app = opened(door(false));
    fixed(&mut app, 20);
    let mut last = t(&app);
    let reversed = sf_interaction::DoorChanged { placement: DOOR, open: false, user: Some(player()), tick: Tick(22) };
    assert_eq!(approve(&mut app, DOOR, None), vec![reversed]);
    for _ in 0..20 {
        assert_eq!(fixed(&mut app, 1).len(), 1);
        assert!(t(&app) < last, "closing");
        last = t(&app);
    }
    assert_eq!(t(&app).to_bits(), 0.0_f32.to_bits());
    assert!(!state(&app).unwrap().is_moving());
    assert_eq!(fixed(&mut app, 1), vec![]);
}

#[test]
fn the_openers_release_closes_the_door_behind_after_the_delay() {
    let mut app = opened(door_with(false, true));
    fixed(&mut app, 48);
    assert!(!state(&app).unwrap().is_moving());
    // Forget the opening toggle (the gatherer's first run reads it).
    app.world_mut().resource_mut::<Swing>().changed.clear();
    let k = params(&app).door_close_behind_ticks();
    assert!(k > 1);

    // Another user's pass schedules nothing.
    pass_through(&mut app, UserKey::Actor(ActorKey(5)));
    assert_eq!(state(&app).and_then(|s| s.close_at), None);
    assert_eq!(state(&app).and_then(|s| s.opener), Some(player()));

    let released = pass_through(&mut app, player());
    assert_eq!(state(&app).and_then(|s| s.close_at), Some(released.plus(k)));
    let k_runs = usize::try_from(k).unwrap();
    assert_eq!(fixed(&mut app, k_runs - 1), vec![]);
    assert!(app.world().resource::<Swing>().changed.is_empty());
    assert!(tagged(&app));

    let moved = fixed(&mut app, 1);
    let closed = sf_interaction::DoorChanged { placement: DOOR, open: false, user: None, tick: released.plus(k) };
    assert_eq!(app.world().resource::<Swing>().changed, vec![closed]);
    assert!(!tagged(&app));
    assert_eq!(moved.len(), 1, "the swing starts on the closing tick");
    assert_eq!(fixed(&mut app, 47).len(), 47);
    assert_eq!(t(&app).to_bits(), 0.0_f32.to_bits());
    assert!(!state(&app).unwrap().is_moving());
    assert_eq!(app.world().resource::<Swing>().changed.len(), 1);
}

#[test]
fn a_door_removed_mid_swing_writes_no_more_moves() {
    let mut app = opened(door(false));
    assert_eq!(fixed(&mut app, 10).len(), 10);
    app.world_mut().write_message(IndexChange::unload(DOOR));
    assert_eq!(fixed(&mut app, 5), vec![]);
    assert!(app.world().resource::<Doors>().get(DOOR).is_none());
    assert_eq!(app.world().resource::<DoorSweeps>().iter().count(), 0);
}
