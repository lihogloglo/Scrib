//! Combat C2a (combat.md "C2a", combat-adapter.md "Sparring dummy"): the sparring dummy strikes a
//! player in reach on its schedule through its own `Command` (the chooser, sweeps, contact order
//! and rules any character's swing takes), the player's health drops by the rules' amount and a
//! reaction by the hit's side starts and ends, in both views; a player out of reach is never
//! struck; first person keeps a free look through the reaction. Headless, no clips: synthetic
//! slot lengths and a straight blade track (no animation, so feel is sanity only).

use sf_action::SlotKey;
use sf_action_bevy::ActionTick;
use sf_anim::{SocketTrack, Trs};
use sf_camera::ViewChoice;
use sf_combat::{Side, Tier};
use sf_combat_bevy::{CombatData, CombatSystems, Combatant, CombatantDied, HitLanded};

use super::*;
use crate::combat::PlayerCombat;
use crate::dummy::{DummyKind, Sparring, SparringRules, TrainingDummy};
use crate::tests::Statics;
use crate::tests_combat::{armed_with, look_right};

/// The harness's slot length (every sheet slot, `armed_with`), s.
const SLOT_LEN: f32 = 0.5;

fn ramp(t: f32, a: f32, b: f32, from: f32, to: f32) -> f32 {
    from + (to - from) * ((t - a) / (b - a)).clamp(0.0, 1.0)
}

/// A straight slash across the attacker's front over light1's hit window (0.14-0.28 s): the
/// weapon socket at chest height 0.3 m ahead, sweeping x -1 -> 1, the blade pointing forward
/// (socket +Y turned onto +Z), so it crosses a defender standing 0.7-1.5 m ahead.
pub(crate) fn slash_tracks(app: &mut App) {
    let track = SocketTrack::from_fn(SLOT_LEN, 60.0, |t| Trs {
        translation: Vec3::new(ramp(t, 0.14, 0.28, -1.0, 1.0), 1.2, 0.3),
        rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
        ..Trs::IDENTITY
    });
    app.world_mut()
        .resource_mut::<CombatData>()
        .sockets
        .insert(SlotKey::of("attack.sword_1h.light1"), track);
}

/// Per fixed tick, after the combat step: move starts, landed hits and deaths.
#[derive(Resource, Default, Debug)]
struct Log {
    /// (tick, mover, move) for every move start.
    starts: Vec<(u32, Entity, RuleKey)>,
    hits: Vec<(u32, HitLanded)>,
    died: Vec<Entity>,
    /// Feet heights of every character, every tick.
    heights: Vec<f32>,
}

#[allow(clippy::needless_pass_by_value)] // system parameters
fn record(
    tick: Res<ActionTick>,
    mut hits: MessageReader<HitLanded>,
    mut died: MessageReader<CombatantDied>,
    chars: Query<(Entity, &ActionCharacter, &ActionBody)>,
    mut log: ResMut<Log>,
) {
    let t = tick.0.0;
    for (e, c, body) in &chars {
        if c.output.started {
            log.starts.push((t, e, c.state.mov));
        }
        log.heights
            .push(WorldPos::ORIGIN.delta_f32(body.loco.position).y);
    }
    log.hits.extend(hits.read().map(|h| (t, *h)));
    log.died.extend(died.read().map(|d| d.who));
}

struct Spar {
    app: App,
    player: Entity,
    dummy: Entity,
    cam: Entity,
}

impl Spar {
    /// The armed player (game rules, sheathed: reactions don't need the weapon) with the camera
    /// following it in `view` (`camera`: with the camera plugin, so the look runs through it),
    /// and a sparring dummy `dist` m off at `deg` degrees from the player's facing (+Z).
    fn new(dist: f32, deg: f32, view: ViewChoice, camera: bool) -> Self {
        let (mut app, player) = armed_with(|app| {
            if camera {
                app.add_plugins(SfCameraPlugin::<Statics>::default());
            }
        });
        let sheet = app.world().resource::<PlayerCombat>().sheet.clone();
        app.insert_resource(rules(&[], sheet.as_ref()));
        dummy::systems::<Statics>(&mut app);
        slash_tracks(&mut app);
        app.init_resource::<Log>()
            .add_systems(FixedUpdate, record.after(CombatSystems::Apply));
        for _ in 0..30 {
            app.update();
        }
        let feet = app.world().get::<ActionBody>(player).unwrap().loco.position;
        let (s, c) = deg.to_radians().sin_cos();
        let root = app.world_mut().spawn_empty().id();
        let at = feet.offset(Vec3::new(dist * s, 0.0, dist * c));
        let dummy = dummy::spawn_kind(
            &mut app.world_mut().commands(),
            0,
            DummyKind::Sparring,
            (at, std::f32::consts::PI),
            root,
        );
        let mut cam = PlayerCamera::new(player);
        cam.set_view(view);
        let cam = app.world_mut().spawn((cam, Transform::default())).id();
        app.world_mut().flush();
        Self {
            app,
            player,
            dummy,
            cam,
        }
    }

    fn frames(&mut self, n: usize) {
        for _ in 0..n {
            self.app.update();
        }
    }

    fn key(&self, mov: &str) -> RuleKey {
        self.app
            .world()
            .resource::<PlayerCombat>()
            .sheet
            .as_ref()
            .unwrap()
            .rule_key(mov)
    }

    fn reaction(&self, tier: Tier, side: Side) -> RuleKey {
        self.app
            .world()
            .resource::<PlayerCombat>()
            .sheet
            .as_ref()
            .unwrap()
            .reactions
            .lookup(tier, side)
            .expect("reaction")
    }

    fn log(&self) -> &Log {
        self.app.world().resource::<Log>()
    }

    /// The dummy's light1 starts, in ticks since it began sparring.
    fn strikes(&self) -> Vec<u32> {
        let start = self
            .app
            .world()
            .get::<Sparring>(self.dummy)
            .unwrap()
            .start
            .expect("sparring")
            .0;
        let light1 = self.key("light1");
        self.log()
            .starts
            .iter()
            .filter(|&&(_, e, m)| e == self.dummy && m == light1)
            .map(|&(t, ..)| t - start)
            .collect()
    }

    fn health(&self) -> f32 {
        self.app
            .world()
            .get::<Combatant>(self.player)
            .unwrap()
            .fighter
            .vitals
            .health
            .cur
    }

    /// Sanity: every character's feet stayed finite and on the floor.
    fn on_the_floor(&self) {
        let h = &self.log().heights;
        assert!(
            !h.is_empty() && h.iter().all(|y| y.is_finite() && y.abs() < 0.1),
            "feet left the floor: {h:?}"
        );
    }
}

/// The first `n` strike ticks of the content's schedule.
fn due(app: &App, n: usize) -> Vec<u32> {
    let s = app
        .world()
        .resource::<SparringRules>()
        .0
        .expect("sparring.ron parses");
    (0..10_000).filter(|&t| s.due(t)).take(n).collect()
}

/// Ticks for `secs` at 60 Hz.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn ticks(secs: f32) -> u32 {
    (secs * 60.0).round() as u32
}

/// The sparring dummy 1.2 m ahead strikes on its schedule, each swing lands once on the player
/// inside light1's hit window, the player's health drops by the rules' amount (attack 10 x mv 1.0)
/// per hit, and a front reaction starts on the next tick and ends back in a non-reaction move.
fn strikes_in_reach(view: ViewChoice) {
    let mut s = Spar::new(1.2, 0.0, view, false);
    s.frames(30 + 480);
    let want = due(&s.app, 3);
    assert_eq!(
        &s.strikes()[..3],
        want.as_slice(),
        "strikes on the schedule ({view:?})"
    );
    let start = s
        .app
        .world()
        .get::<Sparring>(s.dummy)
        .unwrap()
        .start
        .unwrap()
        .0;
    let hits: Vec<(u32, HitLanded)> = s
        .log()
        .hits
        .iter()
        .copied()
        .filter(|&(t, _)| t - start <= want[2] + ticks(1.0))
        .collect();
    assert_eq!(hits.len(), 3, "one landing per swing: {hits:?}");
    let react = s.reaction(Tier::Small, Side::Front);
    for (&(t, h), &strike) in hits.iter().zip(&want) {
        assert_eq!(
            (h.attacker, h.defender, h.tier, h.side),
            (s.dummy, s.player, Tier::Small, Side::Front)
        );
        let into = t - start - strike;
        assert!(
            (ticks(0.14)..=ticks(0.28) + 1).contains(&into),
            "landed {into} ticks into light1 (window 0.14-0.28 s)"
        );
        // The reaction starts on the next tick and ends (another move starts) within its clip
        // plus the hit-stop.
        let starts: Vec<_> = s
            .log()
            .starts
            .iter()
            .filter(|&&(tt, e, _)| e == s.player && tt > t)
            .collect();
        assert_eq!(
            (starts[0].0, starts[0].2),
            (t + 1, react),
            "the reaction follows the hit: {starts:?}"
        );
        let ended = starts[1].0 - starts[0].0;
        assert!(
            starts[1].2 != react && (ticks(SLOT_LEN)..=ticks(SLOT_LEN + 0.3)).contains(&ended),
            "reaction lasted {ended} ticks"
        );
    }
    let health = s.health();
    assert!(
        (health - (combat::UNSHEETED_VITALS[0] - 3.0 * mw_combat::DEFAULT_ATTACK)).abs() < 1e-4,
        "health {health}"
    );
    s.on_the_floor();
}

#[test]
fn the_sparring_dummy_strikes_a_player_in_reach_on_schedule_third_person() {
    strikes_in_reach(ViewChoice::ThirdPerson);
}

#[test]
fn the_sparring_dummy_strikes_a_player_in_reach_on_schedule_first_person() {
    strikes_in_reach(ViewChoice::FirstPerson);
}

/// Out of reach (3 m, reach 2 m) the dummy never swings and the player is never hit.
#[test]
fn a_player_out_of_reach_is_never_struck() {
    let mut s = Spar::new(3.0, 0.0, ViewChoice::ThirdPerson, false);
    s.frames(30 + 420);
    assert!(
        s.app
            .world()
            .get::<Sparring>(s.dummy)
            .unwrap()
            .start
            .is_some(),
        "it was sparring"
    );
    assert_eq!(s.strikes(), Vec::<u32>::new());
    assert!(s.log().hits.is_empty(), "{:?}", s.log().hits);
    assert!((s.health() - combat::UNSHEETED_VITALS[0]).abs() < 1e-4);
    s.on_the_floor();
}

/// The hit's side in the player's frame picks the reaction (`sword_1h`: small front, left, right,
/// back). The dummy stands at the player's front, back, left (+X at yaw 0) and right.
#[test]
fn the_reaction_follows_the_side_the_hit_came_from() {
    for (deg, side) in [
        (0.0, Side::Front),
        (180.0, Side::Back),
        (90.0, Side::Left),
        (-90.0, Side::Right),
    ] {
        let mut s = Spar::new(1.2, deg, ViewChoice::ThirdPerson, false);
        s.frames(30 + ticks(2.5) as usize);
        let &(t, h) = s
            .log()
            .hits
            .first()
            .unwrap_or_else(|| panic!("no hit from {deg} deg"));
        assert_eq!((h.tier, h.side), (Tier::Small, side), "from {deg} deg");
        let react = s
            .log()
            .starts
            .iter()
            .find(|&&(tt, e, _)| e == s.player && tt == t + 1)
            .map(|&(.., m)| m);
        assert_eq!(react, Some(s.reaction(Tier::Small, side)), "from {deg} deg");
    }
}

/// First person (camera.md "First person"): a reaction carries no `look` limits, so the mouse
/// turns the view freely through it while the body holds its yaw (the arms follow the body's
/// pose); the view never feeds the reaction.
#[test]
fn first_person_keeps_a_free_look_through_a_reaction() {
    let mut s = Spar::new(1.2, 0.0, ViewChoice::FirstPerson, true);
    let react = s.reaction(Tier::Small, Side::Front);
    let mov = |s: &Spar| {
        s.app
            .world()
            .get::<ActionCharacter>(s.player)
            .unwrap()
            .state
            .mov
    };
    for _ in 0..(30 + ticks(2.5)) {
        s.app.update();
        if mov(&s) == react {
            break;
        }
    }
    assert_eq!(mov(&s), react, "hit and reacting");
    look_right(&mut s.app, 0.05);
    let mut samples = Vec::new();
    while mov(&s) == react && samples.len() < 120 {
        s.app.update();
        let body = s
            .app
            .world()
            .get::<ActionBody>(s.player)
            .unwrap()
            .loco
            .motion
            .yaw;
        let view = s.app.world().get::<PlayerCamera>(s.cam).unwrap().look().yaw;
        samples.push((body, sf_camera::wrap_angle(view - body).to_degrees()));
    }
    assert!(samples.len() > 10, "{samples:?}");
    let off = samples.iter().map(|&(_, o)| o.abs()).fold(0.0, f32::max);
    assert!(
        off > 60.0,
        "free look through the reaction: {off:.1} deg ({samples:?})"
    );
    let body = samples[0].0;
    assert!(
        samples.iter().all(|&(b, _)| (b - body).abs() < 1e-3),
        "the body held its yaw: {samples:?}"
    );
}

/// A sparring bout is non-lethal: a player brought down refills (and the dummy keeps sparring).
#[test]
fn a_player_brought_down_while_sparring_refills() {
    let mut s = Spar::new(1.2, 0.0, ViewChoice::ThirdPerson, false);
    s.frames(30);
    s.app
        .world_mut()
        .get_mut::<Combatant>(s.player)
        .unwrap()
        .fighter
        .vitals
        .health
        .cur = 5.0;
    s.frames(ticks(2.0) as usize);
    assert_eq!(s.log().hits.len(), 1, "{:?}", s.log().hits);
    assert_eq!(s.log().died, [s.player]);
    let health = s
        .app
        .world()
        .get::<Combatant>(s.player)
        .unwrap()
        .fighter
        .vitals
        .health;
    assert!(
        (health.cur - health.max).abs() < 1e-4,
        "refilled: {health:?}"
    );
    s.frames(ticks(2.5) as usize);
    assert_eq!(s.log().hits.len(), 2, "it strikes on: {:?}", s.log().hits);
}

/// The standing training dummy never writes a command (no `Sparring`), whatever the reach.
#[test]
fn a_standing_dummy_never_strikes() {
    let mut s = Spar::new(1.2, 0.0, ViewChoice::ThirdPerson, false);
    let root = s.app.world_mut().spawn_empty().id();
    let feet = s
        .app
        .world()
        .get::<ActionBody>(s.player)
        .unwrap()
        .loco
        .position;
    let standing = dummy::spawn(
        &mut s.app.world_mut().commands(),
        1,
        (feet.offset(Vec3::new(-1.2, 0.0, 0.0)), 0.0),
        root,
    );
    s.app.world_mut().flush();
    s.frames(30 + 420);
    assert!(
        s.app.world().get::<TrainingDummy>(standing).is_some()
            && s.app.world().get::<Sparring>(standing).is_none()
    );
    assert!(
        s.log().hits.iter().all(|(_, h)| h.attacker == s.dummy),
        "{:?}",
        s.log().hits
    );
    assert_eq!(
        s.app
            .world()
            .get::<ActionCharacter>(standing)
            .unwrap()
            .command,
        sf_action::Command::default()
    );
}
