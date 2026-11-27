//! `IN31a` headless tests: same-tick claims resolve alike whatever the send order, a Reserved lease
//! expires after `lease_s` of fixed ticks and promotes the queue head, index removal invalidates,
//! a live `interact.claim.lease_s` change reaches the claim table, def defaults = struct defaults;
//! an owner's moves add up from where it was claimed (`OwnerMoved`), a re-insert ends holds on the
//! slots it switches off (`SlotDisabled`).

use sf_ids::{ActorKey, PlayerKey};
use sf_interaction::{
    ClipFamily, EventKind, IkTargets, Occupancy, SlotDef, SlotKind, SlotTerms, TagFilter, TagId, UserKey, VerbDef,
};
use sf_records::RecordId;
use sf_tunables_bevy::{SetBy, SetTunable, TunablesPlugin};
use smallvec::{SmallVec, smallvec};

use super::*;

/// What the fixed tick wrote, gathered after [`InteractionSystems::Apply`].
#[derive(Resource, Default)]
struct Seen {
    events: Vec<InteractionEvent>,
    answers: Vec<ClaimAnswered>,
}

fn gather(mut seen: ResMut<Seen>, mut events: MessageReader<ClaimChanged>, mut answers: MessageReader<ClaimAnswered>) {
    seen.events.extend(events.read().map(|e| e.0));
    seen.answers.extend(answers.read().cloned());
}

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((TunablesPlugin, InteractionPlugin))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .init_resource::<Seen>()
        .add_systems(FixedUpdate, gather.after(InteractionSystems::Apply));
    // Binds the tunables (PreUpdate); no `TimePlugin`, so the fixed loop runs only when a test asks.
    app.update();
    app
}

fn fixed(app: &mut App, runs: usize) {
    for _ in 0..runs {
        app.world_mut().run_schedule(FixedUpdate);
    }
}

fn actor(n: u128) -> UserKey {
    UserKey::Actor(ActorKey(n))
}

fn player(n: u128) -> UserKey {
    UserKey::Player(PlayerKey(n))
}

fn chair() -> Arc<InteractableDef> {
    let seat = SlotDef {
        kind: SlotKind::Pose { family: TagId::of("seat") },
        local: Frame::default(),
        band: 0.45,
        verb: VerbDef { verb: TagId::of("sit"), priority: 0, requires: TagFilter::default() },
        entrances: smallvec![],
        clips: ClipFamily("seat.chair".into()),
        ik: IkTargets::default(),
        reach: 0.0,
        occupancy: Occupancy::Pose,
        control: None,
        users: TagFilter::default(),
        group: None,
        queue: None,
    };
    Arc::new(InteractableDef {
        id: RecordId::new("chair"),
        slots: vec![seat],
        verbs: SmallVec::new(),
        material: None,
        destructible: None,
        carry: None,
        adverts: Vec::new(),
    })
}

const CHAIR: PlacementId = PlacementId(7);
const SEAT: SlotRef = SlotRef { placement: CHAIR, slot: 0 };

fn insert_chair(app: &mut App) {
    app.world_mut().write_message(IndexChange::Insert {
        placement: CHAIR,
        def: chair(),
        object: Frame::default(),
        owner: Entity::PLACEHOLDER,
        disabled: SmallVec::new(),
    });
}

fn terms(queue: Option<u8>) -> SlotTerms {
    SlotTerms { group: None, queue, reach: 0.0, users: TagFilter::default(), enabled: true }
}

fn request(user: UserKey, priority: u8, enqueue: bool, queue: Option<u8>) -> ClaimRequest {
    let (slot, terms, tags) = (SEAT, terms(queue), SmallVec::new());
    let op = if enqueue { Op::Enqueue { slot, terms, tags } } else { Op::Claim { slot, terms, tags } };
    ClaimRequest(Request { user, priority, op })
}

fn holder(app: &App) -> Option<UserKey> {
    app.world().resource::<InteractionClaims>().0.claim_on(SEAT).map(|c| c.user)
}

fn seen(app: &App) -> &Seen {
    app.world().resource::<Seen>()
}

#[test]
fn same_tick_claims_resolve_alike_in_any_send_order() {
    let contenders = [(actor(1), 20), (actor(2), 20), (player(1), 30), (actor(3), 30)];
    let orders: [[usize; 4]; 6] = [[0, 1, 2, 3], [3, 2, 1, 0], [1, 3, 0, 2], [2, 0, 3, 1], [3, 1, 2, 0], [1, 0, 3, 2]];
    let mut outcomes = Vec::new();
    for order in orders {
        let mut app = app();
        // Index insert and claims in the same tick: the index changes apply first.
        insert_chair(&mut app);
        for i in order {
            let (user, priority) = contenders[i];
            app.world_mut().write_message(request(user, priority, false, None));
        }
        fixed(&mut app, 1);
        let mut answers: Vec<(UserKey, Result<Granted, Refusal>)> =
            seen(&app).answers.iter().map(|a| (a.request.user, a.result)).collect();
        answers.sort_by_key(|(u, _)| *u);
        outcomes.push((holder(&app), seen(&app).events.clone(), answers));
    }
    // Priority 30 wins; of the two at 30 the smaller `UserKey` (actors first) goes first.
    assert_eq!(outcomes[0].0, Some(actor(3)));
    assert!(outcomes[0].1.iter().any(|e| e.user == actor(3) && e.kind == EventKind::Claimed));
    for o in &outcomes[1..] {
        assert_eq!(o, &outcomes[0]);
    }
}

#[test]
fn a_reserved_lease_expires_after_lease_s_and_promotes_the_queue_head() {
    let mut app = app();
    insert_chair(&mut app);
    app.world_mut().write_message(request(actor(1), 20, false, Some(2)));
    app.world_mut().write_message(request(actor(2), 20, true, Some(2)));
    fixed(&mut app, 1);
    assert_eq!(holder(&app), Some(actor(1)));
    let claimed_at = app.world().resource::<InteractionTick>().0;
    let lease = app.world().resource::<InteractionClaims>().0.params().lease_ticks();
    assert_eq!(lease, 600, "10 s at 60 Hz");
    fixed(&mut app, usize::try_from(lease).unwrap() - 1);
    assert_eq!(holder(&app), Some(actor(1)), "still within the lease");
    fixed(&mut app, 1);
    assert_eq!(holder(&app), Some(actor(2)));
    let at = claimed_at.plus(lease);
    let events = &seen(&app).events;
    assert!(events.iter().any(|e| e.tick == at
        && e.user == actor(1)
        && e.kind == EventKind::Invalidated(InvalidReason::LeaseExpired)));
    assert!(events.iter().any(|e| e.tick == at && e.user == actor(2) && e.kind == EventKind::Promoted));
}

#[test]
fn index_insert_then_claim_works_and_removal_invalidates_with_owner_unloaded() {
    let mut app = app();
    // Not resident yet: refused without reaching the core.
    app.world_mut().write_message(request(actor(1), 20, false, None));
    fixed(&mut app, 1);
    assert_eq!(seen(&app).answers.last().map(|a| a.result), Some(Err(Refusal::Disabled)));
    insert_chair(&mut app);
    fixed(&mut app, 1);
    assert_eq!(app.world().resource::<InteractionIndex>().0.len(), 1);
    app.world_mut().write_message(request(actor(1), 20, false, None));
    fixed(&mut app, 1);
    assert_eq!(seen(&app).answers.last().map(|a| a.result), Some(Ok(Granted::Claimed)));
    assert_eq!(holder(&app), Some(actor(1)));
    // The cell unloads; a claim sent the same tick sees the object gone.
    app.world_mut().write_message(IndexChange::unload(CHAIR));
    app.world_mut().write_message(request(actor(2), 20, false, None));
    fixed(&mut app, 1);
    assert_eq!(holder(&app), None);
    assert!(app.world().resource::<InteractionIndex>().0.is_empty());
    let last = seen(&app).events.last().copied().unwrap();
    assert_eq!((last.user, last.slot, last.kind), (actor(1), SEAT, EventKind::Invalidated(InvalidReason::OwnerUnloaded)));
    assert_eq!(seen(&app).answers.last().map(|a| a.result), Some(Err(Refusal::Disabled)));
}

#[test]
fn a_lease_tunable_change_applies_on_the_next_update() {
    let mut app = app();
    app.world_mut().write_message(SetTunable::new(LEASE_S.name(), "2", SetBy::Console));
    app.update();
    assert!((app.world().resource::<InteractionSettings>().0.lease_s - 2.0).abs() < f32::EPSILON);
    insert_chair(&mut app);
    app.world_mut().write_message(request(actor(1), 20, false, None));
    fixed(&mut app, 1);
    assert_eq!(app.world().resource::<InteractionClaims>().0.params().lease_ticks(), 120);
    fixed(&mut app, 119);
    assert_eq!(holder(&app), Some(actor(1)));
    fixed(&mut app, 1);
    assert_eq!(holder(&app), None, "expired after 2 s of ticks");
}

#[test]
fn tick_hz_follows_the_fixed_timestep() {
    let mut app = app();
    app.insert_resource(Time::<Fixed>::from_hz(30.0));
    fixed(&mut app, 1);
    let hz = app.world().resource::<InteractionClaims>().0.params().tick_hz;
    assert!((hz - 30.0).abs() < 1e-3, "{hz}");
}

#[test]
fn def_defaults_are_the_struct_defaults() {
    let d = InteractParams::default();
    let floats = [
        (&LEASE_S, d.lease_s),
        (&MOVE_TOLERANCE_M, d.move_tolerance_m),
        (&MOVE_TOLERANCE_DEG, d.move_tolerance_deg),
        (&TICKET_LEASE_S, d.ticket_lease_s),
        (&FOCUS_HOLD_S, d.focus_hold_s),
        (&FOCUS_SWITCH_MARGIN, d.focus_switch_margin),
        (&FOCUS_SWITCH_S, d.focus_switch_s),
        (&DOOR_SWING_S, d.door_swing_s),
        (&DOOR_CLOSE_BEHIND_S, d.door_close_behind_s),
        (&PLACE_CLEARANCE_M, d.place_clearance_m),
        (&PLACE_SPREAD_M, d.place_spread_m),
        (&DEBRIS_LOD_M, d.debris_lod_m),
        (&DEBRIS_FADE_S, d.debris_fade_s),
        (&DEBRIS_SHRINK_S, d.debris_shrink_s),
        (&DEBRIS_SPEED_MPS, d.debris_speed_mps),
        (&DEBRIS_SPREAD, d.debris_spread),
        (&RULES_WET_THRESHOLD, d.rules_wet_threshold),
        (&RULES_WET_TTL_S, d.rules_wet_ttl_s),
        (&FIRE_SPREAD_RADIUS_M, d.fire_spread_radius_m),
        (&FIRE_HEAT_PER_S, d.fire_heat_per_s),
        (&FIRE_COOL_PER_S, d.fire_cool_per_s),
        (&FIRE_JITTER, d.fire_jitter),
        (&FIRE_WIND_BIAS, d.fire_wind_bias),
        (&FIRE_WIND_REF_MPS, d.fire_wind_ref_mps),
        (&FIRE_RAIN_DOUSE, d.fire_rain_douse),
        (&FIRE_REGION_M, d.fire_region_m),
        (&IMPACT_SPEED_MIN_MPS, d.impact_speed_min_mps),
        (&IMPACT_IMPULSE_MIN_NS, d.impact_impulse_min_ns),
        (&IMPACT_DAMAGE_PER_NS, d.impact_damage_per_ns),
        (&NOISE_MIN_SPEED_MPS, d.noise_min_speed_mps),
        (&NOISE_REF_SPEED_MPS, d.noise_ref_speed_mps),
        (&NOISE_SPEED_MUL_MAX, d.noise_speed_mul_max),
        (&NOISE_MERGE_S, d.noise_merge_s),
        (&NOISE_REVEAL_M, d.noise_reveal_m),
        (&NOISE_DOOR_REACH_M, d.noise_door_reach_m),
        (&NOISE_BASH_REACH_M, d.noise_bash_reach_m),
        (&NOISE_LOCKPICK_REACH_M, d.noise_lockpick_reach_m),
        (&NOISE_MATERIAL_REF_M, d.noise_material_ref_m),
    ];
    let counts = [
        (&PRIORITY_AMBIENT, d.priority_ambient),
        (&PRIORITY_SCHEDULE, d.priority_schedule),
        (&PRIORITY_QUEST, d.priority_quest),
        (&PRIORITY_PLAYER, d.priority_player),
        (&QUEUE_CAPACITY, d.queue_capacity),
        (&HAND_MAX, d.hand_max),
        (&PLACE_FLOOR_TRIES, d.place_floor_tries),
        (&DEBRIS_BREAKS_PER_TICK, d.debris_breaks_per_tick),
    ];
    assert_eq!(DEBRIS_AWAKE_MAX.default_value(), u32::from(d.debris_awake_max), "interact.debris.awake_max");
    assert_eq!(RULES_EVALS_PER_TICK.default_value(), u32::from(d.rules_evals_per_tick), "interact.rules.evals_per_tick");
    assert_eq!(FIRE_BURNING_MAX.default_value(), u32::from(d.fire_burning_max), "interact.fire.burning_max");
    assert_eq!(NOISE_PER_TICK_MAX.default_value(), u32::from(d.noise_per_tick_max), "interact.noise.per_tick_max");
    assert_eq!(
        FIRE_SPREAD_CHECKS_PER_TICK.default_value(),
        u32::from(d.fire_spread_checks_per_tick),
        "interact.fire.spread_checks_per_tick"
    );
    assert_eq!(floats.len() + counts.len() + 5, TUNABLES.len(), "one def per field but tick_hz");
    for (def, field) in floats {
        assert!((def.default_value() - field).abs() < f32::EPSILON, "{}", def.name());
    }
    for (def, field) in counts {
        assert_eq!(def.default_value(), u32::from(field), "{}", def.name());
    }
    let app = app();
    assert_eq!(app.world().resource::<InteractionSettings>().0, d);
}

// --- Owner moves and re-inserts ---------------------------------------------------------------

/// An object frame `x` m along X, turned `yaw_deg` about Y.
fn at(x: f32, yaw_deg: f32) -> Frame {
    let mut f = Frame::default();
    f.translation.x = x;
    let half = yaw_deg.to_radians() / 2.0;
    f.rotation.y = half.sin();
    f.rotation.w = half.cos();
    f
}

fn move_chair(app: &mut App, object: Frame) {
    app.world_mut().write_message(IndexChange::Move { placement: CHAIR, object });
    fixed(app, 1);
}

/// A two-seat bench at `object`, its `disabled` slots off.
fn insert_bench(app: &mut App, object: Frame, disabled: &[u8]) {
    let mut def = (*chair()).clone();
    def.slots.push(def.slots[0].clone());
    app.world_mut().write_message(IndexChange::Insert {
        placement: CHAIR,
        def: Arc::new(def),
        object,
        owner: Entity::PLACEHOLDER,
        disabled: disabled.iter().copied().collect(),
    });
}

fn claim(app: &mut App, user: UserKey, slot: u8) {
    let op = Op::Claim { slot: SlotRef { placement: CHAIR, slot }, terms: terms(None), tags: SmallVec::new() };
    app.world_mut().write_message(ClaimRequest(Request { user, priority: 20, op }));
}

fn claim_of(app: &App, slot: u8) -> Option<UserKey> {
    app.world().resource::<InteractionClaims>().0.claim_on(SlotRef { placement: CHAIR, slot }).map(|c| c.user)
}

/// The tick's events since `from`, as (user, slot, kind).
fn events_since(app: &App, from: usize) -> Vec<(UserKey, u8, EventKind)> {
    seen(app).events[from..].iter().map(|e| (e.user, e.slot.slot, e.kind)).collect()
}

#[test]
fn owner_moves_add_up_from_where_it_was_claimed() {
    let mut app = app();
    insert_chair(&mut app);
    app.world_mut().write_message(request(actor(1), 20, false, Some(2)));
    app.world_mut().write_message(request(actor(2), 20, true, Some(2)));
    fixed(&mut app, 1);
    assert_eq!(holder(&app), Some(actor(1)));
    assert_eq!(app.world().resource::<InteractionClaims>().0.queue(SEAT).len(), 1);
    assert_eq!(app.world().resource::<InteractionFrames>().anchor(CHAIR), Some(&Frame::default()));
    let mark = seen(&app).events.len();
    // 0.1 m per tick: each step within the 0.25 m tolerance, the third adds up past it.
    move_chair(&mut app, at(0.1, 0.0));
    move_chair(&mut app, at(0.2, 0.0));
    assert_eq!(holder(&app), Some(actor(1)), "within tolerance of the claim's frame");
    assert!(events_since(&app, mark).is_empty());
    move_chair(&mut app, at(0.3, 0.0));
    let moved = EventKind::Invalidated(InvalidReason::OwnerMoved);
    assert_eq!(events_since(&app, mark), vec![(actor(2), 0, moved), (actor(1), 0, moved)]);
    assert_eq!(holder(&app), None);
    assert!(app.world().resource::<InteractionClaims>().0.queue(SEAT).is_empty());
    assert_eq!(app.world().resource::<InteractionFrames>().anchor(CHAIR), None);
    // A fresh claim is measured from where the chair is now.
    app.world_mut().write_message(request(actor(3), 20, false, None));
    fixed(&mut app, 1);
    assert_eq!(holder(&app), Some(actor(3)));
    let mark = seen(&app).events.len();
    move_chair(&mut app, at(0.5, 0.0));
    assert_eq!(holder(&app), Some(actor(3)), "0.2 m from the new claim's frame");
    assert!(events_since(&app, mark).is_empty());
    move_chair(&mut app, at(0.6, 0.0));
    assert_eq!(events_since(&app, mark), vec![(actor(3), 0, moved)]);
}

#[test]
fn a_turn_beyond_the_tolerance_invalidates_without_moving() {
    let mut app = app();
    insert_chair(&mut app);
    app.world_mut().write_message(request(actor(1), 20, false, None));
    fixed(&mut app, 1);
    let mark = seen(&app).events.len();
    move_chair(&mut app, at(0.0, 15.0));
    assert_eq!(holder(&app), Some(actor(1)), "15 deg is within the 20 deg tolerance");
    move_chair(&mut app, at(0.0, 25.0));
    assert_eq!(events_since(&app, mark), vec![(actor(1), 0, EventKind::Invalidated(InvalidReason::OwnerMoved))]);
}

#[test]
fn a_reinsert_ends_holds_on_the_slots_it_switches_off() {
    let mut app = app();
    insert_bench(&mut app, Frame::default(), &[]);
    claim(&mut app, actor(1), 0);
    claim(&mut app, actor(2), 1);
    fixed(&mut app, 1);
    assert_eq!((claim_of(&app, 0), claim_of(&app, 1)), (Some(actor(1)), Some(actor(2))));
    // Slot 0 switched off by the adapter: its claim ends, slot 1's stays.
    let mark = seen(&app).events.len();
    insert_bench(&mut app, Frame::default(), &[0]);
    fixed(&mut app, 1);
    let disabled = EventKind::Invalidated(InvalidReason::SlotDisabled);
    assert_eq!(events_since(&app, mark), vec![(actor(1), 0, disabled)]);
    assert_eq!((claim_of(&app, 0), claim_of(&app, 1)), (None, Some(actor(2))));
    let index = &app.world().resource::<InteractionIndex>().0;
    assert!(index.get(SlotRef { placement: CHAIR, slot: 0 }).is_none());
    // A re-insert elsewhere is a move.
    let mark = seen(&app).events.len();
    insert_bench(&mut app, at(1.0, 0.0), &[]);
    fixed(&mut app, 1);
    assert_eq!(events_since(&app, mark), vec![(actor(2), 1, EventKind::Invalidated(InvalidReason::OwnerMoved))]);
    // A def without slot 1 ends its claim.
    claim(&mut app, actor(3), 1);
    fixed(&mut app, 1);
    assert_eq!(claim_of(&app, 1), Some(actor(3)));
    let mark = seen(&app).events.len();
    app.world_mut().write_message(IndexChange::Insert {
        placement: CHAIR,
        def: chair(),
        object: at(1.0, 0.0),
        owner: Entity::PLACEHOLDER,
        disabled: SmallVec::new(),
    });
    fixed(&mut app, 1);
    assert_eq!(events_since(&app, mark), vec![(actor(3), 1, disabled)]);
    assert_eq!(app.world().resource::<InteractionFrames>().anchor(CHAIR), None);
}
