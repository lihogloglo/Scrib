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
use sf_combat::{GuardResult, Side, Tier};
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

// ---------- Guard (combat.md "C2b") ----------

/// A guarded light1's stamina damage on the player (`guard_damage` 24 x 0.55 / stability 1).
fn guard_cost(s: &Spar) -> f32 {
    let sheet = s
        .app
        .world()
        .resource::<PlayerCombat>()
        .sheet
        .clone()
        .unwrap();
    let row = sheet.moves.moves[&sheet.rule_key("light1")].hits[0];
    let player = &s.app.world().get::<Combatant>(s.player).unwrap().fighter;
    sf_combat::StandardRules::default().guard_stamina_damage(row.guard_damage, player)
}

/// Draws the weapon (F: instant without a draw clip) and holds the guard key from now on.
fn draw_and_guard(s: &mut Spar) {
    tests::hold_key(&mut s.app, KeyCode::KeyF, true);
    s.frames(2);
    tests::hold_key(&mut s.app, KeyCode::KeyF, false);
    s.frames(2);
    let guard = tests::action(&mut s.app, PlayerAction::Button(ActionClass::Guard));
    tests::mock(&mut s.app, guard, true);
    s.frames(2);
    assert_eq!(
        s.mov(),
        s.key("guard"),
        "drawn, the held key raises the guard"
    );
}

impl Spar {
    fn mov(&self) -> RuleKey {
        self.app
            .world()
            .get::<ActionCharacter>(self.player)
            .unwrap()
            .state
            .mov
    }

    fn stamina(&self) -> sf_combat::Pool {
        self.app
            .world()
            .get::<Combatant>(self.player)
            .unwrap()
            .fighter
            .vitals
            .stamina
    }

    /// Runs frames (one tick each) until the log holds `n` hits; returns the stamina at each hit's
    /// tick.
    fn until_hits(&mut self, n: usize, max: usize) -> Vec<f32> {
        let mut at = Vec::new();
        for _ in 0..max {
            let before = self.log().hits.len();
            self.app.update();
            if self.log().hits.len() > before {
                at.push(self.stamina().cur);
            }
            if self.log().hits.len() >= n {
                return at;
            }
        }
        panic!(
            "{} hits after {max} frames: {:?}",
            self.log().hits.len(),
            self.log().hits
        );
    }
}

/// The player holds the guard facing the sparring dummy: every strike is guarded (tier
/// `Additive`, no reaction: the guard stays up), the health drops by the guarded amount (attack 10
/// x 0.35) and the stamina by the guard cost at each hit, then refills at the guarding rate before
/// the next strike (2.5 s apart).
fn guard_holds_the_strikes(view: ViewChoice) {
    let mut s = Spar::new(1.2, 0.0, view, false);
    draw_and_guard(&mut s);
    let cost = guard_cost(&s);
    let at = s.until_hits(3, 30 + 480);
    let max = s.stamina().max;
    for (&(_, h), &stamina) in s.log().hits.iter().zip(&at) {
        assert_eq!(
            (h.attacker, h.defender, h.tier, h.guard),
            (s.dummy, s.player, Tier::Additive, GuardResult::Held),
            "{view:?}"
        );
        assert!(
            (stamina - (max - cost)).abs() < 1e-3,
            "stamina {stamina} at the hit ({view:?}): full {max} - {cost}"
        );
    }
    let guard = s.key("guard");
    let first = s.log().hits[0].0;
    assert!(
        s.log()
            .starts
            .iter()
            .all(|&(t, e, m)| e != s.player || t < first || m == guard),
        "no reaction while the guard holds: {:?}",
        s.log().starts
    );
    assert_eq!(s.mov(), guard);
    let want = combat::UNSHEETED_VITALS[0]
        - 3.0 * mw_combat::DEFAULT_ATTACK * (1.0 - mw_combat::GUARD_ABSORB);
    assert!(
        (s.health() - want).abs() < 1e-3,
        "health {} vs {want}",
        s.health()
    );
    s.frames(60);
    assert!(
        (s.stamina().cur - max).abs() < 1e-3,
        "refilled while guarding: {:?}",
        s.stamina()
    );
    s.on_the_floor();
}

#[test]
fn a_held_guard_takes_the_sparring_dummys_strikes_third_person() {
    guard_holds_the_strikes(ViewChoice::ThirdPerson);
}

#[test]
fn a_held_guard_takes_the_sparring_dummys_strikes_first_person() {
    guard_holds_the_strikes(ViewChoice::FirstPerson);
}

/// With regeneration off and a strike every 1.25 s, the guard breaks on the first guarded hit
/// the stamina can't pay: the player's 50 holds three (13.2 each), the fourth breaks it (tier
/// `Large`, stamina 0), the guard-break move starts on the next tick, and once it ends the still
/// held key raises the guard again.
#[test]
#[allow(
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)] // a handful of hits; 0 is exact (clamped)
fn the_guard_breaks_when_a_strike_empties_the_stamina() {
    let mut s = Spar::new(1.2, 0.0, ViewChoice::ThirdPerson, false);
    s.app
        .insert_resource(combat::ModernRules(sf_combat::StandardRules {
            regen_per_s: 0.0,
            ..sf_combat::StandardRules::default()
        }));
    s.app
        .world_mut()
        .resource_mut::<SparringRules>()
        .0
        .as_mut()
        .expect("schedule")
        .period = 1.25;
    draw_and_guard(&mut s);
    let (full, cost) = (s.stamina().cur, guard_cost(&s));
    let n = (full / cost).ceil() as usize;
    assert_eq!(n, 4, "50 / 13.2");
    let at = s.until_hits(n, 30 + 600);
    let hits: Vec<_> = s
        .log()
        .hits
        .iter()
        .map(|&(t, h)| (t, h.tier, h.guard))
        .collect();
    for (k, &(_, tier, guard)) in hits.iter().enumerate().take(n - 1) {
        assert_eq!(
            (tier, guard),
            (Tier::Additive, GuardResult::Held),
            "hit {k}"
        );
        assert!(
            (at[k] - (full - cost * (k as f32 + 1.0))).abs() < 1e-3,
            "hit {k}: {}",
            at[k]
        );
    }
    let (t, tier, guard) = hits[n - 1];
    assert_eq!((tier, guard), (Tier::Large, GuardResult::Broken));
    assert_eq!(at[n - 1], 0.0);
    s.frames(1);
    let brk = s.key("guard.break");
    assert!(
        s.log()
            .starts
            .iter()
            .any(|&(tt, e, m)| (tt, e, m) == (t + 1, s.player, brk)),
        "{:?}",
        s.log().starts
    );
    let want = combat::UNSHEETED_VITALS[0]
        - n as f32 * mw_combat::DEFAULT_ATTACK * (1.0 - mw_combat::GUARD_ABSORB);
    assert!(
        (s.health() - want).abs() < 1e-3,
        "guarded damage only: {}",
        s.health()
    );
    s.frames(ticks(SLOT_LEN + 0.3) as usize);
    assert_eq!(
        s.mov(),
        s.key("guard"),
        "the held key raises the guard again after the break"
    );
    s.on_the_floor();
}

/// First person: the held guard's body follows the view at the guard's turn rate (360 deg/s, 6
/// deg a tick) however fast the mouse turns, the view is free in yaw (the guard's `look` is
/// pitch-only), and once the mouse stops the body catches up and squares on the view.
#[test]
fn first_person_the_guard_follows_the_view_at_its_turn_rate() {
    let mut s = Spar::new(6.0, 0.0, ViewChoice::FirstPerson, true);
    draw_and_guard(&mut s);
    let want = sf_camera::LookLimits::from_degrees(180.0, (-45.0, 20.0), 0.10);
    let yaws = |s: &Spar| {
        let body = s
            .app
            .world()
            .get::<ActionBody>(s.player)
            .unwrap()
            .loco
            .motion
            .yaw;
        let cam = s.app.world().get::<PlayerCamera>(s.cam).unwrap();
        (body, cam.look().yaw, cam.look_limits)
    };
    look_right(&mut s.app, 0.2);
    let mut last = yaws(&s).0;
    let mut ahead = 0.0f32;
    for _ in 0..20 {
        s.app.update();
        let (body, view, limits) = yaws(&s);
        assert_eq!(limits, Some(want), "the guard's look rule holds");
        let step = sf_camera::wrap_angle(body - last).to_degrees().abs();
        assert!(
            step <= 6.0 + 1e-3,
            "the body turned {step:.3} deg in a tick"
        );
        ahead = ahead.max(sf_camera::wrap_angle(view - body).to_degrees().abs());
        last = body;
    }
    assert!(
        ahead > 45.0,
        "the view runs ahead of the body, free in yaw: {ahead:.1} deg"
    );
    look_right(&mut s.app, 0.0);
    s.frames(90);
    let (body, view, _) = yaws(&s);
    assert!(
        sf_camera::wrap_angle(view - body).abs() < 1e-3,
        "squared on the view: body {body}, view {view}"
    );
    assert_eq!(s.mov(), s.key("guard"));
}
