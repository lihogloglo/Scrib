//! `IN39a2` headless tests: [`RuleCause`]s through the rules shell. Water on a burning jar is
//! `Removed(Burning)` + `Added(Wet)` with the culprit and the tags mirrored (an unindexed placement:
//! nothing); a snuff rule on a lit registered light turns it off through the light registry with one
//! [`LightChanged`] carrying the rule cause (`Lit` never stored; the unlit light then matches no
//! rule); a damage rule is one [`PropDamage`] at the object; a stage rule is one [`Destroyed`] at
//! that stage; a change rule folds with the tick's door change into one [`WorldChanged`].

use glam::{Quat, Vec3};
use sf_interaction::{
    BURNING, CHANGE_OPENED, ChangeDetail, InteractionRule, LIT, RuleEvent, RuleOutput, RuleResult, Rules, Stage,
    BreakAction, DestructibleDef, TagFilter, TagId, Ttl, WET, WorldChange,
};
use sf_records::RecordId;
use smallvec::smallvec;

use super::tests_light::lamp;
use super::tests_noise::{base, door, insert, jar, now, player};
use super::*;

const JAR: PlacementId = PlacementId(30);
const LAMP: PlacementId = PlacementId(31);
const DOOR: PlacementId = PlacementId(32);
const WATER: TagId = TagId::of("water");
const TORCH: TagId = TagId::of("torch");
const WIND: TagId = TagId::of("wind");
const FIRE: TagId = TagId::of("fire");

#[derive(Resource)]
struct Seen<M: Message>(Vec<M>);

impl<M: Message> Default for Seen<M> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

fn see<M: Message + Clone>(mut read: MessageReader<M>, mut seen: ResMut<Seen<M>>) {
    seen.0.extend(read.read().cloned());
}

fn app(rules: Vec<InteractionRule>) -> App {
    let mut app = base();
    app.init_resource::<Seen<RuleApplied>>()
        .init_resource::<Seen<LightChanged>>()
        .init_resource::<Seen<PropDamage>>()
        .init_resource::<Seen<Destroyed>>()
        .init_resource::<Seen<WorldChanged>>()
        .add_systems(FixedUpdate, see::<RuleApplied>.after(RuleSystems))
        .add_systems(FixedUpdate, see::<LightChanged>.after(RuleSystems))
        .add_systems(FixedUpdate, see::<PropDamage>.after(RuleSystems).before(DestroySystems::Damage))
        .add_systems(FixedUpdate, see::<Destroyed>.after(DestroySystems::Release))
        .add_systems(FixedUpdate, see::<WorldChanged>.after(WorldChangeSystems));
    app.world_mut().resource_mut::<InteractionRules>().0 = Rules::new(rules);
    app
}

fn taken<M: Message>(app: &mut App) -> Vec<M> {
    std::mem::take(&mut app.world_mut().resource_mut::<Seen<M>>().0)
}

fn rule(id: &str, source: TagId, target: TagFilter, result: RuleResult) -> InteractionRule {
    InteractionRule { id: RecordId::new(id), source: TagFilter { all: smallvec![source], ..TagFilter::default() }, target, result }
}

fn cause(app: &mut App, placement: PlacementId, source: TagId, culprit: Option<sf_interaction::UserKey>) {
    app.world_mut().write_message(RuleCause { placement, source: smallvec![source], culprit });
}

fn at(x: f32) -> Frame {
    Frame { translation: Vec3::new(x, 0.5, -1.0), rotation: Quat::IDENTITY }
}

#[test]
fn water_on_a_burning_jar_removes_burning_adds_wet_with_the_culprit_and_mirrors_the_tags() {
    let water = rule(
        "water",
        WATER,
        TagFilter { all: smallvec![BURNING], ..TagFilter::default() },
        RuleResult { remove: smallvec![BURNING], add: smallvec![(WET, Ttl::Secs(10.0))], ..RuleResult::default() },
    );
    let mut app = app(vec![water]);
    insert(&mut app, JAR, jar(None, 10.0), at(1.0));
    let t = now(&app);
    let params = app.world().resource::<InteractionSettings>().0;
    app.world_mut().resource_mut::<InteractionObjectStates>().0.add(JAR, BURNING, Ttl::UntilRemoved, t, &params);
    app.world_mut().resource_mut::<InteractionStateTags>().insert(JAR, BURNING);
    let _ = taken::<RuleApplied>(&mut app);

    cause(&mut app, JAR, WATER, Some(player()));
    // An unindexed placement's cause is dropped.
    cause(&mut app, PlacementId(99), WATER, None);
    app.world_mut().run_schedule(FixedUpdate);
    let t = now(&app);
    let event = |output| RuleApplied {
        tick: t,
        event: RuleEvent {
            placement: JAR,
            rule: Some(RecordId::new("water")),
            culprit: Some(player()),
            source: smallvec![WATER],
            output,
        },
    };
    assert_eq!(taken::<RuleApplied>(&mut app), vec![event(RuleOutput::Removed(BURNING)), event(RuleOutput::Added(WET))]);
    assert_eq!(app.world().resource::<InteractionStateTags>().tags(JAR), &[WET]);
    let states = &app.world().resource::<InteractionObjectStates>().0;
    assert!(states.has(JAR, WET) && !states.has(JAR, BURNING));
}

#[test]
fn a_snuff_rule_turns_a_lit_light_off_through_the_light_registry() {
    let snuff = rule(
        "douse-lamp",
        WATER,
        TagFilter { all: smallvec![LIT], ..TagFilter::default() },
        RuleResult { remove: smallvec![LIT], ..RuleResult::default() },
    );
    let mut app = app(vec![snuff]);
    insert(&mut app, LAMP, lamp("lamp", true), at(2.0));
    assert!(app.world().resource::<Lights>().get(LAMP).is_some_and(|l| l.state.lit));
    let flame = app.world().resource::<Lights>().get(LAMP).map(|l| l.at).unwrap_or_default();
    let _ = taken::<LightChanged>(&mut app);

    cause(&mut app, LAMP, WATER, Some(player()));
    app.world_mut().run_schedule(FixedUpdate);
    let t = now(&app);
    assert_eq!(
        taken::<LightChanged>(&mut app),
        vec![LightChanged(
            sf_interaction::LightChanged { tick: t, placement: LAMP, lit: false, at: flame, culprit: Some(player()) },
            Some(WATER)
        )]
    );
    let world = app.world();
    assert!(world.resource::<Lights>().get(LAMP).is_some_and(|l| !l.state.lit));
    assert!(world.resource::<GameplayLights>().get(LAMP).is_some_and(|l| !l.lit));
    assert!(!world.resource::<InteractionStateTags>().tags(LAMP).contains(&LIT));
    assert!(world.resource::<InteractionObjectStates>().0.is_empty(), "Lit is never stored");

    // Unlit now: the rule's target filter fails, nothing happens.
    cause(&mut app, LAMP, WATER, Some(player()));
    app.world_mut().run_schedule(FixedUpdate);
    assert_eq!(taken::<RuleApplied>(&mut app).len(), 1, "only the first tick's Removed(Lit)");
    assert_eq!(taken::<LightChanged>(&mut app), vec![]);
}

#[test]
fn a_damage_rule_is_one_prop_damage_at_the_object() {
    let burn = rule(
        "torch-burns",
        TORCH,
        TagFilter::default(),
        RuleResult { damage: Some((FIRE, 4.0)), ..RuleResult::default() },
    );
    let mut app = app(vec![burn]);
    insert(&mut app, JAR, jar(None, 10.0), at(3.0));
    cause(&mut app, JAR, TORCH, Some(player()));
    app.world_mut().run_schedule(FixedUpdate);
    assert_eq!(
        taken::<PropDamage>(&mut app),
        vec![PropDamage {
            placement: JAR,
            amount: 4.0,
            kind: FIRE,
            point: at(3.0).translation,
            dir: Vec3::Y,
            culprit: Some(player()),
        }]
    );
    assert_eq!(app.world().resource::<Destructibles>().get(JAR).map(|d| d.state.health), Some(6.0));
}

/// A crate of 10 health: stage 0 (cracked, keeps a model) at half health, stage 1 removes it.
fn crate_def() -> Arc<InteractableDef> {
    let stage = |at: f32, model: Option<&str>| Stage {
        at,
        model: model.map(RecordId::new),
        debris: smallvec![],
        chunks: None,
        effect: None,
        on_break: BreakAction::None,
    };
    let mut def = (*jar(None, 10.0)).clone();
    def.destructible = Some(DestructibleDef {
        health: 10.0,
        stages: smallvec![stage(0.5, Some("crate.cracked")), stage(0.0, None)],
        ignore_external: false,
        loot: None,
        salvage: smallvec![],
    });
    Arc::new(def)
}

#[test]
fn a_stage_rule_is_one_destroyed_at_that_stage() {
    let char_it = rule("char", FIRE, TagFilter::default(), RuleResult { stage: Some(0), ..RuleResult::default() });
    let mut app = app(vec![char_it]);
    insert(&mut app, JAR, crate_def(), at(4.0));
    cause(&mut app, JAR, FIRE, None);
    app.world_mut().run_schedule(FixedUpdate);
    let destroyed = taken::<Destroyed>(&mut app);
    assert_eq!(
        destroyed.iter().map(|d| (d.0.placement, d.0.stage, d.0.kind, d.0.culprit, d.0.removes, d.0.point)).collect::<Vec<_>>(),
        vec![(JAR, 0, FIRE, None, false, at(4.0).translation)]
    );
    let state = app.world().resource::<Destructibles>().get(JAR).map(|d| (d.state.stage, d.state.health, d.gone));
    assert_eq!(state, Some((1, 5.0, false)));
}

#[test]
fn a_change_rule_folds_with_the_ticks_door_change_into_one_world_changed() {
    let gust = rule("gust", WIND, TagFilter::default(), RuleResult { change: Some(CHANGE_OPENED), ..RuleResult::default() });
    let mut app = app(vec![gust]);
    insert(&mut app, DOOR, door(None), at(5.0));
    let _ = taken::<WorldChanged>(&mut app);
    // The door opened on the tick the rule runs (the tick advances first).
    let t = now(&app).plus(1);
    app.world_mut().write_message(DoorChanged(sf_interaction::DoorChanged {
        placement: DOOR,
        open: true,
        user: Some(player()),
        tick: t,
    }));
    cause(&mut app, DOOR, WIND, None);
    app.world_mut().run_schedule(FixedUpdate);
    // One stream: the pair's last change of the tick (the rule's, gathered after the door's) stands.
    assert_eq!(
        taken::<WorldChanged>(&mut app),
        vec![WorldChanged(WorldChange {
            tick: t,
            kind: CHANGE_OPENED,
            object: DOOR,
            at: at(5.0).translation,
            culprit: None,
            owner: None,
            home: None,
            cause: Some(WIND),
            detail: ChangeDetail::None,
        })]
    );
}
