//! Headless app test (combat.md build plan C1b): the three-hit light chain on a standing dummy
//! through the whole shell (gate, chooser, mover, combat step) lands four hits, nothing goes NaN,
//! and the dummy stays on the floor.

use std::time::Duration;

use bevy_app::{App, FixedUpdate};
use bevy_math::Vec3;
use bevy_time::{TimePlugin, TimeUpdateStrategy};
use bevy_transform::components::Transform;
use sf_action::{
    ActionClass, ActionState, BASE_RULES, CharacterId, CharacterParams, Command, Origin, RuleKey, RuleSetBuilder, SlotInfo, SlotKey, SlotTable,
};
use sf_action_bevy::{ActionBody, ActionCharacter, ActionRules, ActionSystems, ActionTick, SfActionPlugin};
use sf_anim::{SocketTrack, Trs};
use sf_combat::{
    CombatRules, CompiledSheet, Contact, Defense, Fighter, HurtShape, Offense, Outcome, Pool, PoolDelta, RegenState, StandardRules, Swing,
    SwingPlan, Tier, Vitals,
};
use sf_controller::{GroundHit, GroundProbe, HeightSurface, Locomotion, MoverParams, ProbeQuery, TerrainProbe};
use sf_coords::WorldPos;

use crate::*;

/// Three light attacks and their reactions on synthetic slots (move time, s).
const SHEET: &str = r#"(
    id: "app_sword",
    rig: "test",
    defaults: (turn: 360.0, hit_stop: (hold: 0.06, ease: 0.10), blend_in: 0.10),
    weapon: (socket: "weapon", segment: ((0.0, 0.0, 0.0), (0.0, 0.0, 1.0)), radius: 0.05),
    moves: {
        "light1": (
            category: Attack, classes: [Light], from: [Any], rank: 10,
            slots: ["attack.app.light1"],
            windows: [Accept([Light], 0.10, End), Cancel([Light], 0.45, End), Cancel([Move], 0.80, End), NoTurn(0.15, 0.32)],
            hits: [(window: (0.18, 0.32), mv: 1.0, stamina: 12.0, kind: "slash", tier: Small, stop: 0.05)],
            next: { Light: "light2" },
        ),
        "light2": (
            category: Attack, classes: [Light],
            slots: ["attack.app.light2"],
            windows: [Accept([Light], 0.05, End), Cancel([Light], 0.45, End), Cancel([Move], 0.75, End)],
            hits: [(window: (0.20, 0.34), mv: 1.0, stamina: 12.0, kind: "slash", tier: Small, stop: 0.05)],
            next: { Light: "light3" },
        ),
        "light3": (
            category: Attack, classes: [Light],
            slots: ["attack.app.light3"],
            windows: [Accept([Light], 0.10, End), Cancel([Light], 0.65, End), Cancel([Move], 0.85, End)],
            hits: [
                (window: (0.18, 0.30), mv: 0.8, stamina: 10.0, kind: "slash", tier: Small, stop: 0.05),
                (window: (0.40, 0.52), mv: 1.2, stamina: 10.0, kind: "slash", tier: Medium),
            ],
            next: { Light: "light1" },
        ),
        "roll": (
            category: Dodge, classes: [Dodge], from: [Any], rank: 10,
            slots: ["dodge.app.roll"], rate: 1.25, root_scale: 0.75, square_up: Stick,
            cost: 12.0, iframes: [(0.02, 0.40, All)],
            windows: [Cancel([Move], 0.70, End), NoTurn(0.0, End)],
        ),
    },
    reactions: { (Small, Front): "react.app.small.front", (Medium, Front): "react.app.medium.front" },
    costs: (jump: 10.0, sprint_per_s: 11.0),
)"#;

const CLIPS: &[(&str, f32)] = &[
    ("attack.app.light1", 1.0),
    ("attack.app.light2", 0.9),
    ("attack.app.light3", 1.0),
    ("react.app.small.front", 0.4),
    ("react.app.medium.front", 0.6),
    ("dodge.app.roll", 1.0),
];

/// The roll's root track: 3 m straight ahead over its clip, easing in and out (a roll ends at rest).
struct Line;

impl sf_action::RootPath for Line {
    fn duration(&self) -> f32 {
        1.0
    }

    fn root_at(&self, t: f32) -> sf_anim::RootPose {
        sf_anim::RootPose::new(Vec3::new(0.0, 0.0, 3.0 * smoothstep(t)), 0.0)
    }
}

fn clip_len(slot: &str) -> Option<f32> {
    CLIPS.iter().find(|(s, _)| *s == slot).map(|&(_, l)| l)
}

fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn ramp(t: f32, a: f32, b: f32, from: f32, to: f32) -> f32 {
    from + (to - from) * ((t - a) / (b - a)).clamp(0.0, 1.0)
}

/// Straight-line slashes across x = 0 at chest height, the blade pointing forward (+Z).
fn sockets() -> SocketTracks {
    let mut t = SocketTracks::default();
    let mut add = |slot: &str, f: &dyn Fn(f32) -> f32| {
        let track = SocketTrack::from_fn(clip_len(slot).unwrap(), 60.0, |s| Trs { translation: Vec3::new(f(s), 1.2, 0.3), ..Trs::IDENTITY });
        t.insert(SlotKey::of(slot), track);
    };
    add("attack.app.light1", &|s| ramp(s, 0.18, 0.32, -1.0, 1.0));
    add("attack.app.light2", &|s| ramp(s, 0.20, 0.34, 1.0, -1.0));
    add("attack.app.light3", &|s| if s < 0.35 { ramp(s, 0.18, 0.30, -1.0, 1.0) } else { ramp(s, 0.40, 0.52, 1.0, -1.0) });
    t
}

/// Flat ground at y = 0.
struct Plane;

impl HeightSurface for Plane {
    fn height(&self, _p: WorldPos) -> Option<f32> {
        Some(0.0)
    }
    fn normal(&self, _p: WorldPos) -> Option<Vec3> {
        Some(Vec3::Y)
    }
    fn raycast(&self, origin: WorldPos, dir: Vec3, max: f32) -> Option<(f32, Vec3)> {
        let t = -WorldPos::ORIGIN.delta_f32(origin).y / dir.y;
        (dir.y.abs() > 1e-6 && (0.0..=max).contains(&t)).then_some((t, Vec3::Y))
    }
}

#[derive(Resource)]
struct Flat(TerrainProbe<Plane>);

impl GroundProbe for Flat {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        self.0.probe(queries, out);
    }
}

/// The modern rules as a resource.
#[derive(Resource, Default)]
struct Rules(StandardRules);

impl CombatRules for Rules {
    fn begin_swing(&self, swing: &Swing) -> SwingPlan {
        self.0.begin_swing(swing)
    }
    fn resolve_contact(&self, contact: &Contact, attacker: &Fighter, defender: &Fighter) -> Outcome {
        self.0.resolve_contact(contact, attacker, defender)
    }
    fn can_start(&self, who: &Fighter, class: ActionClass) -> bool {
        self.0.can_start(who, class)
    }
    fn cost(&self, who: &Fighter, base: f32, strength: f32) -> PoolDelta {
        self.0.cost(who, base, strength)
    }
    fn regen(&self, who: &Fighter, state: RegenState) -> PoolDelta {
        self.0.regen(who, state)
    }
}

const ATTACKER: CharacterId = CharacterId(1);
const DUMMY: CharacterId = CharacterId(2);

/// Hits seen through the [`HitLanded`] message.
#[derive(Resource, Default)]
struct Log(Vec<HitLanded>);

/// The attacker mashes Light until `light3` plays, then idles.
#[allow(clippy::needless_pass_by_value)]
fn script(tick: Res<ActionTick>, sheet: Res<Sheet>, mut chars: Query<&mut ActionCharacter>, mut seen: Local<bool>) {
    for mut c in &mut chars {
        let mut cmd = Command::empty(tick.0);
        if c.id == ATTACKER {
            *seen |= c.state.mov == sheet.0.rule_key("light3");
            if !*seen {
                cmd.buttons.pressed.insert(ActionClass::Light);
                cmd.buttons.held.insert(ActionClass::Light);
            }
        }
        c.command = cmd;
    }
}

fn log(mut hits: MessageReader<HitLanded>, mut out: ResMut<Log>) {
    out.0.extend(hits.read().copied());
}

#[derive(Resource)]
struct Sheet(CompiledSheet);

fn app() -> App {
    let sheet = sf_combat::load(SHEET, &clip_len).expect("sheet loads");
    let mut rules = RuleSetBuilder::new();
    rules.add_ron(&Origin::Framework, BASE_RULES);
    sheet.add_rules(&mut rules, &Origin::Mod("test".into()));
    let base = SlotTable::new()
        .with("loco.move", SlotInfo::looping())
        .with("air.fall", SlotInfo::looping())
        .with("air.jump", SlotInfo::once(0.3))
        .with("air.land.normal", SlotInfo::once(0.25))
        .with("air.land.hard", SlotInfo::once(0.5));
    let slots = sheet.fill(base).with_root("dodge.app.roll", std::sync::Arc::new(Line));
    let mut data = CombatData { sockets: sockets(), ..CombatData::default() };
    data.movesets.insert(sheet.clone());
    let mut app = App::new();
    app.add_plugins((TimePlugin, SfActionPlugin::<Flat>::default(), SfCombatPlugin::<Rules>::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
        .insert_resource(Flat(TerrainProbe(Plane)))
        .insert_resource(ActionRules { rules: rules.build().expect("rules build"), slots: Box::new(slots) })
        .insert_resource(data)
        .insert_resource(Sheet(sheet))
        .init_resource::<Rules>()
        .init_resource::<Log>()
        .add_systems(FixedUpdate, script.in_set(ActionSystems::Command))
        .add_systems(FixedUpdate, log.in_set(CombatSystems::Apply));
    app
}

fn spawn(app: &mut App, id: CharacterId, at: Vec3, yaw: f32) -> Entity {
    let key = app.world().resource::<Sheet>().0.key;
    let weapon = app.world().resource::<Sheet>().0.weapon;
    let mut state = ActionState::new(RuleKey::of("loco"));
    state.mode.moveset = Some(key);
    let params = MoverParams::default();
    let character = ActionCharacter::new(id, state, CharacterParams { height: params.height, radius: params.radius, ..CharacterParams::default() });
    let feet = WorldPos::ORIGIN.offset(at);
    let body = ActionBody::new(Locomotion::new(feet, yaw), params, WorldPos::ORIGIN);
    let fighter = Fighter {
        id,
        moveset: key,
        vitals: Vitals { health: Pool::full(100.0), stamina: Pool::full(100.0), poise: None },
        weapon,
        hurt: HurtShape::from_capsule(params.radius, params.height),
        offense: Offense { attack: 10.0 },
        defense: Defense { absorb: 0.0, guard_absorb: 0.0, guard_stability: 0.0 },
        hostile: true,
        min_tier: Tier::Small,
    };
    app.world_mut().spawn((character, body, Transform::default(), Combatant::new(fighter))).id()
}

#[test]
fn the_three_hit_chain_on_a_dummy_lands_four_hits_and_the_dummy_stays_on_the_floor() {
    let mut app = app();
    let attacker = spawn(&mut app, ATTACKER, Vec3::ZERO, 0.0);
    let dummy = spawn(&mut app, DUMMY, Vec3::new(0.0, 0.0, 1.0), std::f32::consts::PI);
    for _ in 0..240 {
        app.update();
        for e in [attacker, dummy] {
            let body = app.world().get::<ActionBody>(e).unwrap();
            let p = WorldPos::ORIGIN.delta_f32(body.loco.position);
            assert!(p.is_finite() && body.loco.motion.yaw.is_finite(), "NaN on {e:?}");
        }
    }
    let hits = &app.world().resource::<Log>().0;
    assert_eq!(hits.len(), 4, "{hits:?}");
    assert!(hits.iter().all(|h| h.attacker == attacker && h.defender == dummy));
    assert_eq!(hits.iter().map(|h| h.tier).collect::<Vec<_>>(), [Tier::Small, Tier::Small, Tier::Small, Tier::Medium]);
    let body = app.world().get::<ActionBody>(dummy).unwrap();
    let feet = WorldPos::ORIGIN.delta_f32(body.loco.position);
    assert!(feet.y.abs() < 0.05 && body.loco.grounded.is_some(), "dummy off the floor at {feet}");
    let c = app.world().get::<Combatant>(dummy).unwrap();
    assert!(c.fighter.vitals.health.cur < 100.0 && c.fighter.vitals.health.cur > 0.0);
    // Stamina was spent on the attacker and comes back.
    let a = app.world().get::<Combatant>(attacker).unwrap();
    assert!(a.fighter.vitals.stamina.cur > 0.0 && a.fighter.vitals.stamina.cur <= 100.0);
}

#[test]
fn a_withheld_class_is_refused_and_flashes() {
    #[derive(Resource, Default)]
    struct Flashes(usize);
    let mut app = app();
    app.init_resource::<Flashes>().add_systems(
        FixedUpdate,
        (|mut m: MessageReader<StaminaFlash>, mut n: ResMut<Flashes>| n.0 += m.read().count()).in_set(CombatSystems::Apply),
    );
    let attacker = spawn(&mut app, ATTACKER, Vec3::ZERO, 0.0);
    app.world_mut().get_mut::<Combatant>(attacker).unwrap().withheld.insert(ActionClass::Light);
    for _ in 0..30 {
        app.update();
    }
    let c = app.world().get::<ActionCharacter>(attacker).unwrap();
    assert_eq!(c.state.mov, RuleKey::of("loco"));
    assert!(app.world().resource::<Flashes>().0 > 0);
}

#[test]
fn a_roll_moves_the_body_by_its_scaled_root_track_on_the_floor() {
    let mut app = app();
    let roller = spawn(&mut app, CharacterId(3), Vec3::ZERO, 0.0);
    app.update();
    // The script writes empty commands for non-attackers; press Dodge once after it.
    app.add_systems(FixedUpdate, press_dodge.in_set(ActionSystems::Command).after(script));
    let mut max_progress = 0.0f32;
    for _ in 0..120 {
        app.update();
        let body = app.world().get::<ActionBody>(roller).unwrap();
        if let Some((_, p)) = body.warp_progress() {
            max_progress = max_progress.max(p);
        }
        let feet = WorldPos::ORIGIN.delta_f32(body.loco.position);
        assert!(feet.is_finite() && feet.y.abs() < 0.05, "roll left the floor: {feet}");
    }
    let body = app.world().get::<ActionBody>(roller).unwrap();
    let feet = WorldPos::ORIGIN.delta_f32(body.loco.position);
    // 3 m of clip x root_scale 0.75 = 2.25 m forward (+Z), sanity bounds only.
    assert!(feet.z > 1.5 && feet.z < 2.6 && feet.x.abs() < 0.1, "rolled to {feet}");
    assert!(max_progress > 0.5, "the clip progress reached the shell: {max_progress}");
    let c = app.world().get::<Combatant>(roller).unwrap();
    assert!(c.fighter.vitals.stamina.cur < 100.0 || c.fight.regen_open);
}

/// Character 3 presses Dodge once, on the first tick it runs.
fn press_dodge(mut chars: Query<&mut ActionCharacter>, mut done: Local<bool>) {
    for mut c in &mut chars {
        if c.id == CharacterId(3) && !*done {
            c.command.buttons.pressed.insert(ActionClass::Dodge);
            c.command.buttons.held.insert(ActionClass::Dodge);
            *done = true;
        }
    }
}
