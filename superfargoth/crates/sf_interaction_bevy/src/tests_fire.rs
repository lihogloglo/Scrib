//! `IN39b` headless tests: the fire step shell. A torch rule lights a rug, its heat lights the
//! neighbouring rug with the torch-bearer as culprit, and both burn out (`Ignited` x2, then `Burnt`
//! at exactly the fuel time); wet ground (the wetness closure) keeps the neighbour `Wet` and unlit;
//! indoors the same closure is ignored and no shelter is asked; a candidate asks shelter once while
//! it rests and again after a move.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use glam::{Quat, Vec3};
use sf_interaction::{
    BURNING, CHANGE_BURNT, CHANGE_IGNITED, CHARRED, ChangeDetail, Flammable, InteractionRule, MaterialDef, RuleResult,
    Rules, TagFilter, TagId, Ttl, WET, WorldChange,
};
use sf_records::RecordId;
use smallvec::smallvec;

use super::tests_noise::{base, insert, jar, now, player};
use super::*;

const RUG: PlacementId = PlacementId(40);
const RUG2: PlacementId = PlacementId(41);
const CLOTH: TagId = TagId::of("cloth");
const TORCH: TagId = TagId::of("torch");
/// The rug's fuel, s (60 ticks at 60 Hz).
const FUEL_S: f32 = 1.0;

#[derive(Resource, Default)]
struct Changes(Vec<WorldChange>);

fn see(mut read: MessageReader<WorldChanged>, mut seen: ResMut<Changes>) {
    seen.0.extend(read.read().map(|c| c.0));
}

fn app() -> App {
    let mut app = base();
    app.init_resource::<Changes>().add_systems(FixedUpdate, see.after(WorldChangeSystems));
    let torch = InteractionRule {
        id: RecordId::new("torch"),
        source: TagFilter { all: smallvec![TORCH], ..TagFilter::default() },
        target: TagFilter::default(),
        result: RuleResult { add: smallvec![(BURNING, Ttl::UntilRemoved)], ..RuleResult::default() },
    };
    app.world_mut().resource_mut::<InteractionRules>().0 = Rules::new(vec![torch]);
    let flammable = Flammable { ignite_heat: 0.1, fuel_s: FUEL_S, burnt_stage: None, spread: true, structure: false };
    app.world_mut().resource_mut::<InteractionMaterials>().insert(MaterialDef {
        id: CLOTH,
        flammable: Some(flammable),
        noise: TagId::of("impact.cloth"),
        impact_reach_m: 4.0,
    });
    app
}

fn at(x: f32) -> Frame {
    Frame { translation: Vec3::new(x, 0.0, 0.0), rotation: Quat::IDENTITY }
}

/// Two cloth rugs 1 m apart.
fn rugs(app: &mut App) {
    insert(app, RUG, jar(Some(CLOTH), 10.0), at(0.0));
    insert(app, RUG2, jar(Some(CLOTH), 10.0), at(1.0));
}

fn torch(app: &mut App) {
    app.world_mut().write_message(RuleCause { placement: RUG, source: smallvec![TORCH], culprit: Some(player()) });
}

fn run(app: &mut App, ticks: usize) {
    for _ in 0..ticks {
        app.world_mut().run_schedule(FixedUpdate);
    }
}

fn changes(app: &mut App) -> Vec<WorldChange> {
    std::mem::take(&mut app.world_mut().resource_mut::<Changes>().0)
}

/// `(kind, object, culprit, cause)` of each change.
fn summary(c: &[WorldChange]) -> Vec<(TagId, PlacementId, Option<sf_interaction::UserKey>, Option<TagId>)> {
    c.iter().map(|c| (c.kind, c.object, c.culprit, c.cause)).collect()
}

fn wet_beyond_half_a_metre(app: &mut App) {
    let wet: Arc<SampleAt> = Arc::new(|p: Vec3| if p.x > 0.5 { 0.5 } else { 0.0 });
    app.world_mut().resource_mut::<FireWetness>().0 = Some(wet);
}

#[test]
fn a_torch_lights_a_rug_its_heat_lights_the_neighbour_with_the_culprit_and_both_burn_out() {
    let mut app = app();
    rugs(&mut app);
    assert_eq!(app.world().resource::<FireCandidates>().len(), 2);
    torch(&mut app);
    app.world_mut().run_schedule(FixedUpdate);
    let lit = now(&app);
    run(&mut app, 200);

    let c = changes(&mut app);
    let p = Some(player());
    assert_eq!(
        summary(&c),
        vec![
            (CHANGE_IGNITED, RUG, p, Some(TORCH)),
            (CHANGE_IGNITED, RUG2, p, Some(TORCH)),
            (CHANGE_BURNT, RUG, p, Some(TORCH)),
            (CHANGE_BURNT, RUG2, p, Some(TORCH)),
        ]
    );
    assert_eq!(c[0].tick, lit);
    assert_eq!(c[2].tick, lit.plus(60), "burns for exactly fuel_s");
    assert_eq!(c[2].detail, ChangeDetail::Stage(None));
    assert_eq!(c[1].at, at(1.0).translation);
    let world = app.world();
    let tags = world.resource::<InteractionStateTags>();
    assert_eq!(tags.tags(RUG), &[CHARRED]);
    assert_eq!(tags.tags(RUG2), &[CHARRED]);
    let culprits = world.resource::<FireCulprits>();
    assert!(culprits.get(RUG).is_none() && culprits.get(RUG2).is_none(), "forgotten after the burn-out");
}

#[test]
fn wet_ground_keeps_the_neighbour_wet_and_unlit() {
    let mut app = app();
    wet_beyond_half_a_metre(&mut app);
    rugs(&mut app);
    torch(&mut app);
    run(&mut app, 200);

    let p = Some(player());
    assert_eq!(
        summary(&changes(&mut app)),
        vec![(CHANGE_IGNITED, RUG, p, Some(TORCH)), (CHANGE_BURNT, RUG, p, Some(TORCH))]
    );
    let world = app.world();
    assert_eq!(world.resource::<InteractionStateTags>().tags(RUG2), &[WET]);
    let states = &world.resource::<InteractionObjectStates>().0;
    assert!(states.has(RUG2, WET) && !states.has(RUG2, BURNING) && !states.has(RUG2, CHARRED));
}

#[test]
fn indoors_the_wetness_closure_is_ignored_and_no_shelter_is_asked() {
    let mut app = app();
    wet_beyond_half_a_metre(&mut app);
    let asked = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&asked);
    let roof: Arc<ProbeAt> = Arc::new(move |_| {
        counter.fetch_add(1, Ordering::Relaxed);
        false
    });
    app.world_mut().resource_mut::<FireShelter>().0 = Some(roof);
    app.world_mut().resource_mut::<FireWeather>().indoors = true;
    rugs(&mut app);
    torch(&mut app);
    run(&mut app, 200);

    let kinds: Vec<_> = summary(&changes(&mut app)).into_iter().map(|(k, o, _, _)| (k, o)).collect();
    assert_eq!(
        kinds,
        vec![(CHANGE_IGNITED, RUG), (CHANGE_IGNITED, RUG2), (CHANGE_BURNT, RUG), (CHANGE_BURNT, RUG2)]
    );
    assert_eq!(asked.load(Ordering::Relaxed), 0);
    assert!(!app.world().resource::<InteractionObjectStates>().0.has(RUG2, WET));
}

#[test]
fn a_candidate_asks_shelter_once_at_rest_and_again_after_a_move() {
    let mut app = app();
    let dry: Arc<SampleAt> = Arc::new(|_| 0.0);
    app.world_mut().resource_mut::<FireWetness>().0 = Some(dry);
    let asked = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&asked);
    let roof: Arc<ProbeAt> = Arc::new(move |_| {
        counter.fetch_add(1, Ordering::Relaxed);
        true
    });
    app.world_mut().resource_mut::<FireShelter>().0 = Some(roof);
    insert(&mut app, RUG, jar(Some(CLOTH), 10.0), at(0.0));
    // Not flammable: no candidate.
    insert(&mut app, PlacementId(42), jar(None, 10.0), at(3.0));
    run(&mut app, 5);
    assert_eq!(app.world().resource::<FireCandidates>().len(), 1);
    assert_eq!(asked.load(Ordering::Relaxed), 1);
    assert_eq!(app.world().resource::<FireCandidates>().get(RUG).and_then(|c| c.sheltered), Some(true));

    app.world_mut().write_message(IndexChange::Move { placement: RUG, object: at(0.5) });
    run(&mut app, 5);
    assert_eq!(asked.load(Ordering::Relaxed), 2);

    app.world_mut().write_message(IndexChange::unload(RUG));
    run(&mut app, 1);
    assert!(app.world().resource::<FireCandidates>().is_empty());
}
