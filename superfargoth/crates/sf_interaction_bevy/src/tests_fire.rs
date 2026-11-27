//! `IN39b` headless tests: the fire step shell. A torch rule lights a rug, its heat lights the
//! neighbouring rug with the torch-bearer as culprit, and both burn out (`Ignited` x2, then `Burnt`
//! at exactly the fuel time); wet ground (the wetness closure) keeps the neighbour `Wet` and unlit;
//! indoors the same closure is ignored and no shelter is asked; a candidate asks shelter once while
//! it rests and again after a move.
//!
//! `IN39c` (fire outputs): a burning destructible takes `damage_per_s / tick_hz` fire damage every
//! burning tick with the fire's culprit; its burn-out forces the last stage once and reports
//! `Burnt`, not `Destroyed`; three fires in one region with `lights_max` 2 light the first two, the
//! first going out hands its slot to the third; each burning object is a gameplay light until it
//! burns out or unloads.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use glam::{Quat, Vec3};
use sf_interaction::{
    BURNING, BreakAction, CHANGE_BURNT, CHANGE_IGNITED, CHARRED, ChangeDetail, DAMAGE_FIRE, DestructibleDef,
    FireOutput, Flammable, InteractableDef, InteractionRule, MaterialDef, RuleResult, Rules, Stage, TagFilter, TagId,
    Ttl, WET, WorldChange,
};
use sf_records::RecordId;
use smallvec::{SmallVec, smallvec};

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

// --- IN39c fire outputs --------------------------------------------------------------------------

const RUG3: PlacementId = PlacementId(43);

/// A cloth rug: with `health`, a destructible whose one stage at 0 keeps a model (the charred rug).
fn rug(health: Option<f32>) -> Arc<InteractableDef> {
    let charred = Stage {
        at: 0.0,
        model: Some(RecordId::new("rug_charred")),
        debris: smallvec![],
        chunks: None,
        effect: None,
        on_break: BreakAction::None,
    };
    let destructible = health.map(|health| DestructibleDef {
        health,
        stages: smallvec![charred],
        ignore_external: false,
        loot: None,
        salvage: smallvec![],
    });
    Arc::new(InteractableDef {
        id: RecordId::new("rug"),
        slots: Vec::new(),
        verbs: SmallVec::new(),
        material: Some(CLOTH),
        destructible,
        carry: None,
        adverts: Vec::new(),
    })
}

/// The fire's outputs seen after the world-change step.
#[derive(Resource, Default)]
struct Outputs {
    damage: Vec<PropDamage>,
    stages: Vec<PropStage>,
    broken: Vec<sf_interaction::Destroyed>,
    fire: Vec<FireChanged>,
}

fn watch(
    mut damage: MessageReader<PropDamage>,
    mut stages: MessageReader<PropStage>,
    mut broken: MessageReader<Destroyed>,
    mut fire: MessageReader<FireChanged>,
    mut out: ResMut<Outputs>,
) {
    out.damage.extend(damage.read().copied());
    out.stages.extend(stages.read().copied());
    out.broken.extend(broken.read().map(|d| d.0));
    out.fire.extend(fire.read().copied());
}

fn outputs_app() -> App {
    let mut app = app();
    app.init_resource::<Outputs>().add_systems(FixedUpdate, watch.after(WorldChangeSystems));
    app
}

fn light(app: &mut App, placement: PlacementId) {
    app.world_mut().write_message(RuleCause { placement, source: smallvec![TORCH], culprit: Some(player()) });
}

#[test]
fn a_burning_destructible_takes_damage_per_s_over_tick_hz_each_burning_tick() {
    let mut app = outputs_app();
    insert(&mut app, RUG, rug(Some(1000.0)), at(0.0));
    light(&mut app, RUG);
    run(&mut app, 30);

    let world = app.world();
    let p = effective_params(world.resource::<InteractionSettings>(), Some(world.resource::<Time<Fixed>>()));
    let amount = p.fire_damage_per_s / p.tick_hz;
    let out = world.resource::<Outputs>();
    assert_eq!(out.damage.len(), 30, "one hit per burning tick, the lighting tick included");
    for d in &out.damage {
        let want = PropDamage {
            placement: RUG,
            amount,
            kind: DAMAGE_FIRE,
            point: at(0.0).translation,
            dir: Vec3::Y,
            culprit: Some(player()),
        };
        assert_eq!(*d, want);
    }
    let mut health = 1000.0_f32;
    for _ in 0..30 {
        health = (health - amount).max(0.0);
    }
    let state = world.resource::<Destructibles>().get(RUG).map(|d| d.state);
    assert_eq!(state.map(|s| (s.health, s.stage)), Some((health, 0)));
}

#[test]
fn a_burn_out_forces_the_last_stage_once_and_reports_burnt_not_destroyed() {
    let mut app = outputs_app();
    insert(&mut app, RUG, rug(Some(1000.0)), at(0.0));
    light(&mut app, RUG);
    run(&mut app, 200);

    let pl = Some(player());
    assert_eq!(
        summary(&changes(&mut app)),
        vec![(CHANGE_IGNITED, RUG, pl, Some(TORCH)), (CHANGE_BURNT, RUG, pl, Some(TORCH))]
    );
    let world = app.world();
    let out = world.resource::<Outputs>();
    assert_eq!(out.damage.len(), 60, "fuel_s x 60 Hz burning ticks");
    let forced =
        PropStage { placement: RUG, stage: 0, kind: DAMAGE_FIRE, culprit: pl, point: at(0.0).translation };
    assert_eq!(out.stages, vec![forced]);
    let broken: Vec<_> = out.broken.iter().map(|b| (b.placement, b.stage, b.kind, b.removes, b.culprit)).collect();
    assert_eq!(broken, vec![(RUG, 0, DAMAGE_FIRE, false, pl)]);
    let state = world.resource::<Destructibles>().get(RUG).map(|d| d.state.stage);
    assert_eq!(state, Some(1), "swapped to the charred stage, still resident");
    let fire: Vec<_> = out.fire.iter().map(|f| (f.placement, f.output)).collect();
    assert_eq!(fire, vec![(RUG, FireOutput::Ignited), (RUG, FireOutput::Burnt { stage: None })]);
}

/// The `light` flag of each burning object, in placement order.
fn lit(app: &App) -> Vec<(PlacementId, bool)> {
    app.world().resource::<BurningObjects>().iter().map(|(p, o)| (p, o.light)).collect()
}

fn gameplay(app: &App) -> Vec<(PlacementId, Vec3, f32, f32, bool)> {
    let lights = app.world().resource::<GameplayLights>();
    lights.iter().map(|l| (l.placement, l.pos, l.radius_m, l.intensity, l.lit)).collect()
}

#[test]
fn fire_lights_go_earliest_first_per_region_and_a_fire_going_out_frees_its_slot() {
    let mut app = outputs_app();
    app.world_mut().resource_mut::<InteractionSettings>().0.fire_lights_max = 2;
    // 10 m apart: out of each other's heat, inside one 64 m fire region.
    insert(&mut app, RUG, rug(None), at(0.0));
    insert(&mut app, RUG2, rug(None), at(10.0));
    insert(&mut app, RUG3, rug(None), at(20.0));
    for p in [RUG, RUG2, RUG3] {
        light(&mut app, p);
        run(&mut app, 1);
    }
    let first = app.world().resource::<BurningObjects>().get(RUG).map(|o| (o.since, o.region));
    let third = app.world().resource::<BurningObjects>().get(RUG3).map(|o| (o.since, o.region));
    assert_eq!(first.map(|f| f.0.plus(2)), third.map(|t| t.0));
    assert_eq!(first.map(|f| f.1), third.map(|t| t.1));
    assert_eq!(lit(&app), vec![(RUG, true), (RUG2, true), (RUG3, false)]);
    let r = 6.0;
    assert_eq!(
        gameplay(&app),
        vec![
            (RUG, at(0.0).translation, r, 1.0, true),
            (RUG2, at(10.0).translation, r, 1.0, true),
            (RUG3, at(20.0).translation, r, 1.0, true),
        ]
    );

    // The first lit burns 60 ticks: still out of a slot one tick before, given it on the tick.
    run(&mut app, 57);
    assert_eq!(lit(&app), vec![(RUG, true), (RUG2, true), (RUG3, false)]);
    run(&mut app, 1);
    assert_eq!(lit(&app), vec![(RUG2, true), (RUG3, true)]);
    assert_eq!(gameplay(&app).iter().map(|g| g.0).collect::<Vec<_>>(), vec![RUG2, RUG3]);

    // The third unloads while burning: its entries go; the second burns out the same tick.
    app.world_mut().write_message(IndexChange::unload(RUG3));
    run(&mut app, 1);
    assert!(lit(&app).is_empty());
    assert!(gameplay(&app).is_empty());
    let fire: Vec<_> = app.world().resource::<Outputs>().fire.iter().map(|f| (f.placement, f.output)).collect();
    let burnt = FireOutput::Burnt { stage: None };
    assert_eq!(
        fire,
        vec![
            (RUG, FireOutput::Ignited),
            (RUG2, FireOutput::Ignited),
            (RUG3, FireOutput::Ignited),
            (RUG, burnt),
            (RUG2, burnt),
        ]
    );
}
