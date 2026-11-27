//! `IN37b` headless tests: the bash shell. A press lands one [`Bashed`] at `ceil(windup_s x tick_hz)`
//! ticks with the core's result at the doorway, one revealed bash noise and one [`WorldChanged`]; a
//! second press while pending or in recovery is dropped, one after it lands; a target unlocked
//! before the land, or a user out of reach, lands nothing; a culprit hit on a non-destructible
//! locked door is a blow the same tick (an impact or a culpritless hit is not); a locked
//! destructible also takes [`PropDamage`], which `Hits` never turns into a second blow.
//! `UseApproved` and `PropDamage` are written directly: their producers have their own tests.

use glam::{Quat, Vec3};
use sf_action_bevy::ActionBody;
use sf_controller::{Locomotion, MoverParams};
use sf_coords::WorldPos;
use sf_interaction::{
    BashResult, BashRules, Blow, CHANGE_BASHED, ChangeDetail, DAMAGE_IMPACT, NoiseCase, TagFilter, TagId, Tick,
    UserKey, VERB_BASH, VerbDef, bash, bash_seed,
};
use sf_script_bevy::PLAYER;
use sf_script_bevy::sf_script::Handle;
use smallvec::{SmallVec, smallvec};

use super::tests_noise::{base, door, insert, jar, now, player};
use super::*;

const DOOR: PlacementId = PlacementId(40);
const CHEST: PlacementId = PlacementId(41);
const WOOD: TagId = TagId::of("wood");
const LOCKED: TagId = TagId::of("locked");
const BLUNT: TagId = TagId::of("blunt");
const HAND: TagId = TagId::of("hand");

/// The doors' world frame: 4 m along +X, a quarter turn about +Y.
fn object() -> Frame {
    Frame { translation: Vec3::new(4.0, 0.0, -2.0), rotation: Quat::from_rotation_y(std::f32::consts::FRAC_PI_2) }
}

/// The door slot's doorway (`tests_noise::door`: 1 m up) in the world.
fn doorway() -> Vec3 {
    let o = object();
    o.translation + o.rotation * Vec3::new(0.0, 1.0, 0.0)
}

fn rules() -> BashRules {
    BashRules {
        bare_kind: Some(HAND),
        kind_factor: [(BLUNT, 1.0)].into(),
        threshold: [(WOOD, 10.0)].into(),
        self_per_force: 0.5,
        wear_per_force: 0.1,
        contents_risk_per_force: 0.01,
        contents_risk_max: 0.1,
    }
}

const MACE: Blow = Blow { force: 14.0, kind: BLUNT, bare_handed: false };

/// `def` with a slotless `bash` needing `locked`.
fn bashable(def: &Arc<InteractableDef>) -> Arc<InteractableDef> {
    let mut d = (**def).clone();
    let requires = TagFilter { all: smallvec![LOCKED], ..TagFilter::default() };
    d.verbs.push(VerbDef { verb: VERB_BASH, priority: 0, requires });
    Arc::new(d)
}

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

fn taken<M: Message>(app: &mut App) -> Vec<M> {
    std::mem::take(&mut app.world_mut().resource_mut::<Seen<M>>().0)
}

/// A host with a locked wooden door and a locked wooden chest (100 health), the player at `feet`
/// (no body: `None`) and every user's blow [`MACE`].
fn app(feet: Option<Vec3>) -> App {
    let mut app = base();
    app.init_resource::<Seen<Bashed>>()
        .init_resource::<Seen<PropDamage>>()
        .init_resource::<Seen<InteractionNoise>>()
        .init_resource::<Seen<WorldChanged>>()
        .add_systems(FixedUpdate, see::<Bashed>.after(BashSystems::Land))
        .add_systems(FixedUpdate, see::<PropDamage>.after(BashSystems::Land).before(DestroySystems::Damage))
        .add_systems(FixedUpdate, see::<InteractionNoise>.after(NoiseSystems::Emit))
        .add_systems(FixedUpdate, see::<WorldChanged>.after(WorldChangeSystems))
        .insert_resource(InteractionBashRules(rules()))
        .insert_resource(BashBlows(Box::new(|_| Some(MACE))));
    insert(&mut app, DOOR, bashable(&door(Some(WOOD))), object());
    insert(&mut app, CHEST, bashable(&jar(Some(WOOD), 100.0)), object());
    let mut tags = app.world_mut().resource_mut::<InteractionStateTags>();
    tags.insert(DOOR, LOCKED);
    tags.insert(CHEST, LOCKED);
    let user = InteractionUser { user: player(), actor: PLAYER, priority: 30, tags: SmallVec::new() };
    match feet {
        Some(f) => {
            let body = ActionBody::new(Locomotion::new(WorldPos::from_f64(f.as_dvec3()), 0.0), MoverParams::default(), WorldPos::ORIGIN);
            app.world_mut().spawn((body, user));
        }
        None => {
            app.world_mut().spawn(user);
        }
    }
    clear(&mut app);
    app
}

fn clear(app: &mut App) {
    let _ = taken::<Bashed>(app);
    let _ = taken::<PropDamage>(app);
    let _ = taken::<InteractionNoise>(app);
    let _ = taken::<WorldChanged>(app);
}

/// The player's approved `bash` on `target`, then one frame.
fn press(app: &mut App, target: PlacementId) {
    let activation = Activated { target: Handle(target.0), actor: PLAYER, verb: Some(VERB_BASH.0) };
    app.world_mut().write_message(UseApproved { activation, verb: Some(VERB_BASH), slot: None });
    app.update();
}

fn ticks(app: &mut App, n: u64) {
    for _ in 0..n {
        app.world_mut().run_schedule(FixedUpdate);
    }
}

fn windup(app: &App) -> u64 {
    let mut p = app.world().resource::<InteractionSettings>().0;
    p.tick_hz = 60.0;
    p.bash_windup_ticks()
}

fn bashed(placement: PlacementId, tick: Tick, blow: Blow, point: Vec3) -> Bashed {
    Bashed(sf_interaction::Bashed {
        tick,
        placement,
        culprit: player(),
        kind: blow.kind,
        force: blow.force,
        result: bash(&rules(), &blow, Some(WOOD)),
        point,
        seed: bash_seed(placement, player(), tick),
    })
}

#[test]
fn a_press_lands_one_bashed_at_the_land_tick_with_one_noise_and_one_change() {
    let mut app = app(None);
    let wind = windup(&app);
    assert_eq!(wind, 27, "ceil(0.45 s x 60 Hz)");
    let pressed = now(&app);
    press(&mut app, DOOR);
    ticks(&mut app, wind - 1);
    assert!(taken::<Bashed>(&mut app).is_empty(), "the swing is still winding up");
    ticks(&mut app, 1);
    let landed = now(&app);
    assert_eq!(landed, pressed.plus(27));
    assert_eq!(taken::<Bashed>(&mut app), vec![bashed(DOOR, landed, MACE, doorway())]);
    assert_eq!(
        bash(&rules(), &MACE, Some(WOOD)),
        BashResult { lock_damage: 14, self_damage: 0.0, weapon_wear: 14.0 * 0.1, contents_risk: 0.1 },
    );
    assert!(taken::<PropDamage>(&mut app).is_empty(), "a door is no destructible");

    let params = app.world().resource::<InteractionSettings>().0;
    let heard = taken::<InteractionNoise>(&mut app);
    assert_eq!(heard.len(), 1, "{heard:?}");
    let noise = &heard[0];
    assert_eq!((noise.tick, noise.at, noise.kind, noise.case), (landed, doorway(), NOISE_BASH, NoiseCase::Bash));
    let reach = params.noise_bash_reach_m * 15.0 / params.noise_material_ref_m;
    assert_eq!(noise.loudness_m.to_bits(), reach.to_bits(), "wood's ratio");
    assert_eq!((noise.maker, noise.reveals_maker), (Some(player()), true));

    let changes = taken::<WorldChanged>(&mut app);
    assert_eq!(changes.len(), 1, "{changes:?}");
    let change = &changes[0].0;
    assert_eq!((change.tick, change.kind, change.object, change.at), (landed, CHANGE_BASHED, DOOR, object().translation));
    assert_eq!((change.culprit, change.cause, change.detail), (Some(player()), Some(BLUNT), ChangeDetail::Force(14)));
    assert!(app.world().resource::<BashPending>().blows().is_empty());
}

#[test]
fn a_second_press_while_pending_or_recovering_is_dropped() {
    let mut app = app(None);
    let w = windup(&app);
    press(&mut app, DOOR);
    press(&mut app, DOOR);
    assert_eq!(app.world().resource::<BashPending>().blows().len(), 1, "one pending blow per user");
    ticks(&mut app, w);
    assert_eq!(taken::<Bashed>(&mut app).len(), 1);

    // Recovery: 1 s = 60 ticks from the land.
    press(&mut app, DOOR);
    ticks(&mut app, 59);
    press(&mut app, DOOR);
    assert!(app.world().resource::<BashPending>().blows().is_empty(), "both presses inside recovery");
    ticks(&mut app, w);
    assert!(taken::<Bashed>(&mut app).is_empty());

    press(&mut app, DOOR);
    ticks(&mut app, w);
    assert_eq!(taken::<Bashed>(&mut app).len(), 1, "after recovery a press lands again");
}

#[test]
fn a_target_unlocked_before_the_land_lands_nothing() {
    let mut app = app(None);
    let w = windup(&app);
    press(&mut app, DOOR);
    ticks(&mut app, 3);
    app.world_mut().resource_mut::<InteractionStateTags>().remove(DOOR, LOCKED);
    ticks(&mut app, w);
    assert!(taken::<Bashed>(&mut app).is_empty());
    assert!(taken::<InteractionNoise>(&mut app).is_empty());
    assert!(taken::<WorldChanged>(&mut app).is_empty());
    assert!(app.world().resource::<BashPending>().blows().is_empty(), "the blow is dropped, not kept");
}

#[test]
fn a_user_out_of_reach_at_the_land_lands_nothing() {
    let p = InteractParams::default();
    let far = doorway() + Vec3::new(p.bash_reach_m + 0.5, -1.0, 0.0);
    let mut out = app(Some(far));
    let w = windup(&out);
    press(&mut out, DOOR);
    ticks(&mut out, w);
    assert!(taken::<Bashed>(&mut out).is_empty());
    assert!(taken::<InteractionNoise>(&mut out).is_empty());

    let near = doorway() + Vec3::new(0.0, -1.0, 1.0);
    let mut within = app(Some(near));
    press(&mut within, DOOR);
    ticks(&mut within, w);
    assert_eq!(taken::<Bashed>(&mut within).len(), 1, "within reach it lands");
}

#[test]
fn a_culprit_hit_on_a_locked_door_is_a_blow_the_same_tick() {
    let mut app = app(None);
    let hit = |amount, kind, culprit: Option<UserKey>| PropDamage {
        placement: DOOR,
        amount,
        kind,
        point: doorway(),
        dir: Vec3::X,
        culprit,
    };
    app.world_mut().write_message(hit(9.0, BLUNT, Some(player())));
    app.world_mut().write_message(hit(50.0, DAMAGE_IMPACT, Some(player())));
    app.world_mut().write_message(hit(50.0, BLUNT, None));
    ticks(&mut app, 1);
    let t = now(&app);
    let swing = Blow { force: 9.0, kind: BLUNT, bare_handed: false };
    let got = taken::<Bashed>(&mut app);
    assert_eq!(got, vec![bashed(DOOR, t, swing, doorway())], "an impact or a culpritless hit is no blow");
    assert_eq!(got[0].0.result.lock_damage, 0, "under wood's threshold");

    app.world_mut().write_message(hit(12.0, HAND, Some(player())));
    ticks(&mut app, 1);
    let fist = Blow { force: 12.0, kind: HAND, bare_handed: true };
    let got = taken::<Bashed>(&mut app);
    assert_eq!(got, vec![bashed(DOOR, now(&app), fist, doorway())]);
    assert_eq!(got[0].0.result.self_damage.to_bits(), 6.0f32.to_bits(), "bare-handed");
    assert_eq!(taken::<WorldChanged>(&mut app).len(), 2, "each blow is a change");
}

#[test]
fn a_locked_destructible_also_takes_prop_damage_once() {
    let mut app = app(None);
    let w = windup(&app);
    press(&mut app, CHEST);
    ticks(&mut app, w);
    let t = now(&app);
    let at = object().translation;
    assert_eq!(taken::<Bashed>(&mut app), vec![bashed(CHEST, t, MACE, at)], "no door slot: the origin");
    assert_eq!(
        taken::<PropDamage>(&mut app),
        vec![PropDamage { placement: CHEST, amount: 14.0, kind: BLUNT, point: at, dir: Vec3::ZERO, culprit: Some(player()) }]
    );
    ticks(&mut app, 2);
    assert!(taken::<Bashed>(&mut app).is_empty(), "Hits skips registered destructibles");
}
