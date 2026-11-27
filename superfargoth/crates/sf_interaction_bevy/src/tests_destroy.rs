//! `IN1b` headless tests: the destruction shell on the fixed tick. One hit crosses two stages ->
//! two [`Destroyed`] in def order with the hit's seed, the same in two runs; breaks over
//! `interact.debris.breaks_per_tick` release next tick keeping the hit's tick; the same hits from
//! two writer systems in either order give the same breaks; hits on a `gone` object and non-script
//! hits on an `ignore_external` one drop; [`DestructionRules`] scales; a [`DestructRest`] clamps and
//! writes nothing; a removing break ends a sitter's claim next tick; a queued break of an unloaded
//! object releases without a removal; one [`DestructChanged`] per object per tick.

use glam::Vec3;
use sf_ids::PlayerKey;
use sf_interaction::{
    BreakAction, ClipFamily, DAMAGE_SCRIPT, DestructState, DestructibleDef, IkTargets, Occupancy, SlotDef, SlotKind,
    SlotTerms, Stage, TagFilter, TagId, UserKey, VerbDef, break_seed,
};
use sf_records::RecordId;
use sf_tunables_bevy::TunablesPlugin;
use smallvec::{SmallVec, smallvec};

use super::*;

const BLUNT: TagId = TagId::of("blunt");
const BLADE: TagId = TagId::of("blade");
const WOOD: TagId = TagId::of("wood");

/// What the fixed tick wrote, gathered after [`DestroySystems::Release`].
#[derive(Resource, Default)]
struct Seen {
    breaks: Vec<sf_interaction::Destroyed>,
    changed: Vec<DestructChanged>,
    /// `IndexChange::Remove { reason: OwnerDestroyed }` placements, in send order.
    destroyed_removes: Vec<PlacementId>,
}

fn gather(
    mut seen: ResMut<Seen>,
    mut breaks: MessageReader<Destroyed>,
    mut changed: MessageReader<DestructChanged>,
    mut index: MessageReader<IndexChange>,
) {
    seen.breaks.extend(breaks.read().map(|d| d.0));
    seen.changed.extend(changed.read().copied());
    for c in index.read() {
        if let IndexChange::Remove { placement, reason: InvalidReason::OwnerDestroyed } = c {
            seen.destroyed_removes.push(*placement);
        }
    }
}

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((TunablesPlugin, InteractionPlugin))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .init_resource::<Seen>()
        .add_systems(FixedUpdate, gather.after(DestroySystems::Release));
    // Binds the tunables (PreUpdate); no `TimePlugin`, so the fixed loop runs only when a test asks.
    app.update();
    app
}

fn fixed(app: &mut App, runs: usize) {
    for _ in 0..runs {
        app.world_mut().run_schedule(FixedUpdate);
    }
}

fn stage(at: f32, model: Option<&str>) -> Stage {
    let on_break = if model.is_none() { BreakAction::SpillContents } else { BreakAction::None };
    Stage { at, model: model.map(RecordId::new), debris: smallvec![], chunks: None, effect: None, on_break }
}

fn def(id: &str, slots: Vec<SlotDef>, destructible: DestructibleDef) -> Arc<InteractableDef> {
    Arc::new(InteractableDef {
        id: RecordId::new(id),
        slots,
        verbs: SmallVec::new(),
        material: Some(WOOD),
        destructible: Some(destructible),
        carry: None,
        adverts: Vec::new(),
    })
}

/// A 40-health wooden crate: cracked at half, splintered at a quarter, gone at zero.
fn crate3() -> Arc<InteractableDef> {
    def(
        "crate3",
        Vec::new(),
        DestructibleDef {
            health: 40.0,
            stages: smallvec![stage(0.5, Some("cracked")), stage(0.25, Some("splintered")), stage(0.0, None)],
            ignore_external: false,
            loot: None,
            salvage: smallvec![],
        },
    )
}

/// A 40-health wooden pot with one stage: gone at zero.
fn pot(ignore_external: bool) -> DestructibleDef {
    DestructibleDef { health: 40.0, stages: smallvec![stage(0.0, None)], ignore_external, loot: None, salvage: smallvec![] }
}

fn seat_slot() -> SlotDef {
    SlotDef {
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
    }
}

fn insert(app: &mut App, placement: PlacementId, def: Arc<InteractableDef>) {
    app.world_mut().write_message(IndexChange::Insert {
        placement,
        def,
        object: Frame::default(),
        owner: Entity::PLACEHOLDER,
        disabled: SmallVec::new(),
    });
}

fn hit(placement: PlacementId, amount: f32, kind: TagId, culprit: u128) -> PropDamage {
    PropDamage {
        placement,
        amount,
        kind,
        point: Vec3::new(0.0, 0.5, 0.0),
        dir: Vec3::NEG_Z,
        culprit: Some(UserKey::Player(PlayerKey(culprit))),
    }
}

fn state(app: &App, placement: PlacementId) -> Option<(DestructState, bool)> {
    app.world().resource::<Destructibles>().get(placement).map(|d| (d.state, d.gone))
}

fn seen(app: &App) -> &Seen {
    app.world().resource::<Seen>()
}

const CRATE: PlacementId = PlacementId(5);

#[test]
fn one_hit_crosses_two_stages_in_def_order_with_the_hits_seed() {
    let run = || {
        let mut app = app();
        insert(&mut app, CRATE, crate3());
        fixed(&mut app, 1);
        // 40 -> 8 (0.2): cracked and splintered.
        app.world_mut().write_message(hit(CRATE, 32.0, BLUNT, 1));
        fixed(&mut app, 1);
        assert_eq!(state(&app, CRATE), Some((DestructState { health: 8.0, stage: 2 }, false)));
        seen(&app).breaks.clone()
    };
    let breaks = run();
    let tick = Tick(2);
    let stages: Vec<u8> = breaks.iter().map(|b| b.stage).collect();
    assert_eq!(stages, vec![0, 1]);
    for b in &breaks {
        assert_eq!((b.tick, b.placement, b.removes), (tick, CRATE, false));
        assert_eq!(b.seed, break_seed(CRATE, tick, b.stage));
        assert_eq!(b.culprit, Some(UserKey::Player(PlayerKey(1))));
        assert!(b.overkill.abs() < f32::EPSILON);
    }
    assert_ne!(breaks[0].seed, breaks[1].seed);
    assert_eq!(run(), breaks);
}

#[test]
fn breaks_over_the_cap_release_next_tick_with_the_hits_tick() {
    let mut app = app();
    let pots: Vec<PlacementId> = (1..=7).map(PlacementId).collect();
    for &p in &pots {
        insert(&mut app, p, def("pot", Vec::new(), pot(false)));
    }
    fixed(&mut app, 1);
    for &p in pots.iter().rev() {
        app.world_mut().write_message(hit(p, 40.0, BLUNT, 1));
    }
    fixed(&mut app, 1);
    // `interact.debris.breaks_per_tick` = 6: the seventh waits; health is already 0 for all seven.
    let released: Vec<PlacementId> = seen(&app).breaks.iter().map(|b| b.placement).collect();
    assert_eq!(released, pots[..6].to_vec());
    assert_eq!(app.world().resource::<PendingBreaks>().0.len(), 1);
    assert_eq!(state(&app, pots[6]), Some((DestructState { health: 0.0, stage: 1 }, true)));
    assert_eq!(seen(&app).changed.len(), 7);
    fixed(&mut app, 1);
    let last = seen(&app).breaks[6];
    assert_eq!((last.placement, last.tick, last.seed), (pots[6], Tick(2), break_seed(pots[6], Tick(2), 0)));
    assert!(last.removes);
    assert_eq!(seen(&app).destroyed_removes, pots);
    assert!(app.world().resource::<PendingBreaks>().0.is_empty());
}

#[derive(Resource, Default)]
struct WriterA(Vec<PropDamage>);

#[derive(Resource, Default)]
struct WriterB(Vec<PropDamage>);

fn write_a(mut hits: ResMut<WriterA>, mut out: MessageWriter<PropDamage>) {
    out.write_batch(hits.0.drain(..));
}

fn write_b(mut hits: ResMut<WriterB>, mut out: MessageWriter<PropDamage>) {
    out.write_batch(hits.0.drain(..));
}

#[test]
fn the_same_hits_from_two_writers_in_either_order_give_the_same_breaks() {
    let run = |a_first: bool| {
        let mut app = app();
        if a_first {
            app.add_systems(FixedUpdate, (write_a, write_b).chain().before(DestroySystems::Damage));
        } else {
            app.add_systems(FixedUpdate, (write_b, write_a).chain().before(DestroySystems::Damage));
        }
        app.init_resource::<WriterA>().init_resource::<WriterB>();
        insert(&mut app, CRATE, crate3());
        fixed(&mut app, 1);
        // 30 + 30 by two culprits: whoever goes first cracks and splinters it, the other destroys it.
        app.world_mut().resource_mut::<WriterA>().0.push(hit(CRATE, 30.0, BLUNT, 2));
        app.world_mut().resource_mut::<WriterB>().0.push(hit(CRATE, 30.0, BLUNT, 1));
        fixed(&mut app, 1);
        (seen(&app).breaks.clone(), seen(&app).changed.clone())
    };
    let (breaks, changed) = run(true);
    assert_eq!(run(false), (breaks.clone(), changed.clone()));
    // The sort puts player 1 first: stages 0 and 1 are theirs, the removing stage player 2's, with
    // the overkill (20 past zero of 40).
    let who: Vec<(u8, Option<UserKey>)> = breaks.iter().map(|b| (b.stage, b.culprit)).collect();
    let (p1, p2) = (Some(UserKey::Player(PlayerKey(1))), Some(UserKey::Player(PlayerKey(2))));
    assert_eq!(who, vec![(0, p1), (1, p1), (2, p2)]);
    assert!((breaks[2].overkill - 0.5).abs() < f32::EPSILON, "{}", breaks[2].overkill);
    assert_eq!(changed, vec![DestructChanged { placement: CRATE, state: DestructState { health: 0.0, stage: 3 } }]);
}

#[test]
fn hits_on_a_gone_object_and_non_script_hits_on_an_ignore_external_one_drop() {
    let mut app = app();
    let (pot_a, quest_pot) = (PlacementId(1), PlacementId(2));
    insert(&mut app, pot_a, def("pot", Vec::new(), pot(false)));
    insert(&mut app, quest_pot, def("quest_pot", Vec::new(), pot(true)));
    fixed(&mut app, 1);
    // Same tick: the first hit destroys the pot, the second finds it gone.
    app.world_mut().write_message(hit(pot_a, 40.0, BLUNT, 1));
    app.world_mut().write_message(hit(pot_a, 40.0, BLUNT, 2));
    app.world_mut().write_message(hit(quest_pot, 40.0, BLUNT, 1));
    fixed(&mut app, 1);
    let released: Vec<(PlacementId, Option<UserKey>)> = seen(&app).breaks.iter().map(|b| (b.placement, b.culprit)).collect();
    assert_eq!(released, vec![(pot_a, Some(UserKey::Player(PlayerKey(1))))]);
    assert_eq!(state(&app, quest_pot), Some((DestructState { health: 40.0, stage: 0 }, false)));
    // Only a script's hit breaks the quest pot.
    app.world_mut().write_message(PropDamage { kind: DAMAGE_SCRIPT, culprit: None, ..hit(quest_pot, 40.0, BLUNT, 1) });
    fixed(&mut app, 1);
    assert_eq!(seen(&app).breaks.last().map(|b| b.placement), Some(quest_pot));
    assert_eq!(seen(&app).changed.len(), 2);
}

#[test]
fn the_rules_factor_scales_raw_damage() {
    let mut app = app();
    {
        let mut rules = app.world_mut().resource_mut::<DestructionRules>();
        rules.set(BLADE, WOOD, 0.5);
        rules.set(TagId::of("hand"), WOOD, 0.0);
    }
    insert(&mut app, CRATE, crate3());
    fixed(&mut app, 1);
    // A blade does half on wood: 40 -> 20 (0.5): cracked only.
    app.world_mut().write_message(hit(CRATE, 40.0, BLADE, 1));
    // A factor of 0 drops the hit.
    app.world_mut().write_message(hit(CRATE, 40.0, TagId::of("hand"), 1));
    fixed(&mut app, 1);
    assert_eq!(state(&app, CRATE), Some((DestructState { health: 20.0, stage: 1 }, false)));
    assert_eq!(seen(&app).breaks.len(), 1);
    // A blunt hit (no rule) does its full amount: 20 -> 10 (0.25): splintered.
    app.world_mut().write_message(hit(CRATE, 10.0, BLUNT, 1));
    fixed(&mut app, 1);
    assert_eq!(state(&app, CRATE), Some((DestructState { health: 10.0, stage: 2 }, false)));
}

#[test]
fn a_rest_clamps_to_the_def_and_writes_no_event() {
    let mut app = app();
    insert(&mut app, CRATE, crate3());
    app.world_mut().write_message(DestructRest { placement: CRATE, state: DestructState { health: 15.0, stage: 1 } });
    fixed(&mut app, 1);
    assert_eq!(state(&app, CRATE), Some((DestructState { health: 15.0, stage: 1 }, false)));
    // A re-insert (a stage swap) keeps the state.
    insert(&mut app, CRATE, crate3());
    fixed(&mut app, 1);
    assert_eq!(state(&app, CRATE), Some((DestructState { health: 15.0, stage: 1 }, false)));
    // An old save past the def: stage to the stage count, health to the def's; the removing stage
    // is reached, so it is gone.
    app.world_mut().write_message(DestructRest { placement: CRATE, state: DestructState { health: 1000.0, stage: 9 } });
    fixed(&mut app, 1);
    assert_eq!(state(&app, CRATE), Some((DestructState { health: 40.0, stage: 3 }, true)));
    assert!(seen(&app).breaks.is_empty());
    assert!(seen(&app).changed.is_empty());
    // A rest for an unregistered placement is dropped.
    app.world_mut().write_message(DestructRest { placement: PlacementId(99), state: DestructState { health: 1.0, stage: 0 } });
    fixed(&mut app, 1);
    assert!(state(&app, PlacementId(99)).is_none());
}

#[test]
fn a_removing_break_ends_the_sitters_claim_next_tick() {
    const STOOL: PlacementId = PlacementId(8);
    let seat = SlotRef { placement: STOOL, slot: 0 };
    let mut app = app();
    insert(&mut app, STOOL, def("stool", vec![seat_slot()], pot(false)));
    let terms = SlotTerms { group: None, queue: None, reach: 0.0, users: TagFilter::default(), enabled: true };
    app.world_mut().write_message(ClaimRequest(Request {
        user: UserKey::Player(PlayerKey(1)),
        priority: 30,
        op: Op::Claim { slot: seat, terms, tags: SmallVec::new() },
    }));
    fixed(&mut app, 1);
    let holder = |app: &App| app.world().resource::<InteractionClaims>().0.claim_on(seat).map(|c| c.user);
    assert_eq!(holder(&app), Some(UserKey::Player(PlayerKey(1))));
    app.world_mut().write_message(hit(STOOL, 40.0, BLUNT, 2));
    fixed(&mut app, 1);
    // Released this tick, with the removal; the claim ends in the next tick's `Apply`.
    assert_eq!(seen(&app).breaks.len(), 1);
    assert_eq!(seen(&app).destroyed_removes, vec![STOOL]);
    assert_eq!(holder(&app), Some(UserKey::Player(PlayerKey(1))));
    fixed(&mut app, 1);
    assert_eq!(holder(&app), None);
    assert!(state(&app, STOOL).is_none());
    assert_eq!(seen(&app).destroyed_removes, vec![STOOL]);
}

#[test]
fn a_queued_break_of_an_unloaded_object_releases_without_a_removal() {
    let mut app = app();
    let pots: Vec<PlacementId> = (1..=7).map(PlacementId).collect();
    for &p in &pots {
        insert(&mut app, p, def("pot", Vec::new(), pot(false)));
    }
    fixed(&mut app, 1);
    for &p in &pots {
        app.world_mut().write_message(hit(p, 40.0, BLUNT, 1));
    }
    fixed(&mut app, 1);
    // The seventh waits; its cell unloads before its release.
    app.world_mut().write_message(IndexChange::unload(pots[6]));
    fixed(&mut app, 1);
    assert_eq!(seen(&app).breaks.last().map(|b| b.placement), Some(pots[6]));
    assert_eq!(seen(&app).breaks.len(), 7);
    assert_eq!(seen(&app).destroyed_removes, pots[..6].to_vec());
}

#[test]
fn two_hits_on_one_object_in_a_tick_write_one_change_with_its_last_state() {
    let mut app = app();
    insert(&mut app, CRATE, crate3());
    fixed(&mut app, 1);
    app.world_mut().write_message(hit(CRATE, 10.0, BLUNT, 1));
    app.world_mut().write_message(hit(CRATE, 15.0, BLUNT, 1));
    fixed(&mut app, 1);
    // 40 -> 15 (0.375): cracked, once.
    assert_eq!(seen(&app).changed, vec![DestructChanged { placement: CRATE, state: DestructState { health: 15.0, stage: 1 } }]);
    assert_eq!(seen(&app).breaks.len(), 1);
}
