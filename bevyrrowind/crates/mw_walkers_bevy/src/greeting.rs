//! NPCs greet the player (`MA4c2`): every awake AI actor runs `mw_ai`'s exact `OpenMW` greeting
//! machine ([`mw_ai::settings::greeting_step`], `MA4c1`: `actors.cpp` 500-619 and the hello timer
//! @ 5f3aafb) on the fixed tick and acts on it: it says its Hello voice line, stops, turns its body
//! to face the player for `iGreetDuration`, plays `idle2`, and greets again only after the player
//! walked past `fGreetDistanceReset`. This is the game's one greeting machine: the voice director
//! (`mw_audio::voice`) only voices the Hello this module asks for and keeps its idle chatter quiet
//! while [`Greeting::in_progress`].
//!
//! **Facts** ([`mw_ai::settings::GreetInput`]), from the tick state (never the drawn pose):
//! - Hello: the actor's modified [`HELLO`] on its own sheet (`MA4a`, the `ai` group; no sheet: 0,
//!   never greets). Distance and the heading that faces the player: feet to feet
//!   ([`mw_ai::settings::greet_geometry`]), the actor's heading from its mover's yaw
//!   ([`heading_of_yaw`]).
//! - The player: the host's [`PlayerBody`] (no body, e.g. the free camera: nobody greets); dead:
//!   [`Dead`] on it; magically hidden: Invisibility or Chameleon on its sheet
//!   ([`crate::head_track::HeadTrackRules::magically_hidden`], the head tracking's rule and threshold).
//! - `getLOS` and `awarenessCheck`: both are the head tracking's "seen" (sight contact within the
//!   last two sight periods, `sf_ai_bevy::seen_recently`). Our sight check casts the LOS ray and
//!   already applies the crime stealth rules (sneak, light, facing) that `OpenMW`'s
//!   `awarenessCheck` rolls, so no second roll is made.
//! - In combat (`OpenMW` `isInCombat`: the actor actually fights): a `Fight` directive serving.
//!   Not the awareness band: Alerted means fully noticed, not fighting (sight fills a seen
//!   player's band to Alerted within a second and latches it; `AI-13`: reading it as combat kept
//!   every NPC that saw the player from greeting). Named gap: until the fight drive pushes `Fight`
//!   directives (roadmap `MA10` / `AI15`), an NPC that `OpenMW` would already be fighting greets
//!   instead; [`mw_ai::settings::engages_on_sight`] (`MA4d`) is the rule that drive will run.
//!   Swimming: [`MoverMode::Swim`]. Package: the serving directive of the
//!   actor's stack is a `StayNear` (`AI_W`) or a `GoTo` (`AI_T`), or there is none; while a
//!   perception reaction ([`Reaction::owns`]: search, curious or suspicious turn, return) drives
//!   the actor it counts as another package, so the greeting resets and the reaction wins; it
//!   starts counting once the reaction hands the actor back to its routine.
//! - Paralysed: the actor's [`PARALYZE`] magnitude above 0 (named gap: absent from the stat table
//!   until the effects adapter adds it, so never paralysed yet). `movement_forced`: false (named
//!   gap: no `ForceJump`/`ForceSneak` flags exist yet). Smooth movement: on
//!   ([`SMOOTH_MOVEMENT`]: our controller always turns smoothly, `OpenMW`'s default setting).
//!
//! **Saying Hello** (`DialogueManager::say(actor, "hello")`, called when the greeting starts; on
//! failure the greeting goes straight to Done with no facing that frame). Only the voice side can
//! answer it (the actor already speaking, or no voiced Hello INFO passing the filters), so a tick
//! runs in three sets ([`GreetSystems`]): **Decide** steps each machine with "can say" assumed and
//! commits it, unless the step would say Hello: then it asks ([`HelloAsk::Asked`]) and keeps the
//! step's input; the voice plugin (`mw_audio_bevy::voices`, between Decide and Apply) tries the line
//! and answers ([`Greeting::answer_hello`]); **Apply** re-runs the asked steps with the answer
//! (unanswered: cannot say), so the result is exactly `OpenMW`'s. A knocked-down actor never asks
//! (`say` fails for it).
//!
//! Crate edge (`MA4c2` brief option 2): `mw_audio_bevy` depends on this crate under its `dev`
//! feature and reads and writes [`Greeting`]. Option 1 (no new edge) needs a generic bark
//! request/answer type in a crate both already see (`sf_character_bevy` or `sf_controller_bevy`),
//! outside this slice's crates, or the app wiring the order: both cost more than one optional edge.
//!
//! **Voices off, no greeting.** The voice plugin is dev-only until the owner hears it
//! (visual-checks AU-3..AU-5) and is off when the dialogue records fail to build. Then nobody
//! answers, nobody can say Hello and, as in `OpenMW`, no NPC turns to greet: every greeting that
//! starts goes straight to Done. There is deliberately no fallback that turns without a voice.
//!
//! **Acting on it.** Apply runs before the routines ([`crate::directive_drive`], the wander drive):
//! while a greeting holds the actor still ([`Greeting::holds`]) they pause as under a reaction
//! (the wander is not stepped, so its walk keeps its goal and its no-progress timer; a `GoTo`
//! restarts its walk after), and **Hold** (after the corridor, the walkers' held stands and the
//! reactions' stands) writes the stand: `LocomotionIntent { desired_facing, ..IDLE }`, the turn
//! target as an engine yaw ([`yaw_of_heading`]); the controller turns at its own rate (our
//! `smoothTurn`). The machine stops the actor on full updates only; the stop holds through the
//! turn-only ticks between them while the greeting is in progress (`AiWander` keeps a greeting
//! actor idle). A turn that outlives the greeting only sets the facing. `idle2` plays once
//! as the walkers' idle fidget (`IdleVariant` 2) when the greeting starts holding (named gap:
//! `OpenMW` restarts it until the greeting ends).
//!
//! Named gaps: only the host player is greeted; level path actors (`mw_exterior_bevy` level
//! drives, after the reactions' stands) are not ordered against Hold; Dormant actors (outside the
//! AI tiers' processing range) keep their state unstepped, except a knocked-down one (dormant
//! while held), which steps and cannot say.
//!
//! Proof lines (owner check): `greeting: N greeted (M without a voice)`, logged at the first
//! greeting and each time the total doubles; and, on full updates, when it changes (at most once
//! per [`REACH_LOG_PERIOD_S`]), `greeting: N in reach (C in combat, P other package, U not seen,
//! H player hidden or dead), G greeting`: the awake actors within their Hello reach of the player
//! body, and why each one is not counting up, first reason in the machine's order (a swimming
//! actor counts under other package, the same reset; a paralysed one under none of them, never
//! yet: named gap above); G passes every gate (counting up or greeting, not Done).

use std::sync::Arc;

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use bevy_log::info;
use bevy_math::Vec3;
use bevy_time::Time;
use mw_ai::settings::{GreetGmst, GreetInput, HelloTimer, greet_geometry, greeting_step, normalize_angle};
pub use mw_ai::settings::{GreetAction, GreetState, GreetingState};
use mw_game_core::player::{PlayerBody, PlayerCharacter};
use mw_graphs::build::params as gp;
use sf_action_bevy::ActionBody;
use sf_ai::DirectiveKind;
use sf_ai_bevy::{AiActor, AiAwareness, AiDirectives, AiPerceptClock, AiPerceptionSettings};
use sf_anim_bevy::Animator;
use sf_controller::LocomotionIntent;
use sf_controller_bevy::{Avoider, ControllerSystems, Dormant, Held, MoveIntent, Mover, MoverMode};
use sf_records::RecordDb;
use sf_stats_bevy::{StatTableRes, Stats};

use crate::head_track::{HeadTrackRules, HeadTrackSettings, host_hidden_magnitudes, seen_player, sight_window};
use crate::physics::{Dead, KnockedDown};
use crate::reaction::{Reaction, ReactionSystems};

/// The stat table key of the actor's AI Hello (`mw_stats::table::AI_SETTINGS[0]`).
pub const HELLO: &str = "hello";
/// The stat table key of Paralyze's magnitude (the MGEF effect's name, as
/// [`crate::head_track::INVISIBILITY`]; absent until the effects adapter adds magnitude stats).
pub const PARALYZE: &str = "paralyze";
/// `OpenMW`'s smooth movement setting as our controller has it.
pub const SMOOTH_MOVEMENT: bool = true; // tunables: const (a fact of our controller: it always turns smoothly)
/// The reach proof line's floor between two lines, s.
const REACH_LOG_PERIOD_S: f64 = 1.0; // tunables: const (log rate limit)

/// The greeting's order points on the fixed tick, all in [`ControllerSystems::Drive`].
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GreetSystems {
    /// Steps every machine; a greeting that starts asks for its Hello ([`HelloAsk::Asked`]).
    /// After the reactions' step.
    Decide,
    /// Finishes the asked steps with the voice's answer; before the routines.
    Apply,
    /// Writes the greeting's stand and turn into `MoveIntent`, after the corridor and every
    /// routine and reaction stand.
    Hold,
}

/// Where an actor's Hello request stands this tick.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HelloAsk {
    /// Nothing asked.
    #[default]
    None,
    /// The greeting starts and asks the voice to say Hello.
    Asked,
    /// The voice says it.
    Said,
    /// The voice cannot (already speaking, no voiced Hello line).
    Refused,
}

/// What an actor's greetings did since spawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GreetStats {
    /// Greetings with their Hello said.
    pub greeted: u32,
    /// Greetings that went to Done in silence (no voice answered).
    pub silent: u32,
}

/// An actor's greeting (module docs): `OpenMW`'s per-actor state, this tick's action, the Hello
/// request the voice answers.
#[derive(Component, Clone, Debug, Default)]
pub struct Greeting {
    /// The committed machine state.
    pub greet: GreetState,
    /// This tick's Hello request.
    pub hello: HelloAsk,
    /// An asked step's input, finished by Apply.
    pending: Option<GreetInput>,
    action: GreetAction,
    /// Whether `idle2` was started for this hold.
    idle2: bool,
    pub stats: GreetStats,
}

impl Greeting {
    /// Greeting now (Hello said, facing the player): no idle chatter.
    pub fn in_progress(&self) -> bool {
        self.greet.state == GreetingState::InProgress
    }

    /// Holds the actor still this tick: the routines pause.
    pub fn holds(&self) -> bool {
        self.action.hold_still
    }

    /// This tick's action.
    pub fn action(&self) -> GreetAction {
        self.action
    }

    /// The voice's answer to an asked Hello (ignored unless one was asked).
    pub fn answer_hello(&mut self, said: bool) {
        if self.hello == HelloAsk::Asked {
            self.hello = if said { HelloAsk::Said } else { HelloAsk::Refused };
        }
    }
}

/// The greeting's game settings (from the records) and the one hello timer for all actors.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct GreetSettings {
    pub gmst: GreetGmst,
}

/// `Actors::mTimerUpdateHello`.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct GreetClock(pub HelloTimer);

/// The game's greeting counts for the proof log line.
#[derive(Default)]
struct GreetLog {
    greeted: u32,
    silent: u32,
    logged: u32,
}

/// The reach proof line (module docs): `(in reach, in combat, other package, not seen, player
/// hidden or dead, greeting)`.
type ReachCounts = (u32, u32, u32, u32, u32, u32);

/// The reach proof line's last counts and when they were logged.
#[derive(Default)]
struct ReachLog {
    last: Option<ReachCounts>,
    at_s: Option<f64>,
}

impl ReachLog {
    /// Logs `counts` when they changed and the last line is old enough.
    fn offer(&mut self, counts: ReachCounts, now_s: f64) {
        if self.last == Some(counts) || self.at_s.is_some_and(|t| now_s - t < REACH_LOG_PERIOD_S) {
            return;
        }
        self.last = Some(counts);
        self.at_s = Some(now_s);
        let (reach, combat, package, unseen, hidden, greeting) = counts;
        info!(
            "greeting: {reach} in reach ({combat} in combat, {package} other package, {unseen} not seen, \
             {hidden} player hidden or dead), {greeting} greeting"
        );
    }

    /// Adds one in-reach actor's reason (module docs) to `counts`.
    fn count(counts: &mut ReachCounts, input: &GreetInput, state: GreetingState) {
        counts.0 += 1;
        if input.in_combat {
            counts.1 += 1;
        } else if input.swimming || !input.package_allows {
            counts.2 += 1;
        } else if !(input.los && input.aware) {
            counts.3 += 1;
        } else if input.player_dead || input.player_hidden {
            counts.4 += 1;
        } else if !input.paralyzed && state != GreetingState::Done {
            counts.5 += 1;
        }
    }
}

/// The Morrowind heading (`rot[2]`: 0 north, toward east positive) of an engine yaw (about +Y, 0
/// facing +Z, which is south).
#[allow(clippy::cast_possible_truncation, reason = "angles in f32, normalised in f64 as OpenMW does")]
pub fn heading_of_yaw(yaw: f32) -> f32 {
    normalize_angle(std::f64::consts::PI - f64::from(yaw)) as f32
}

/// The engine yaw of a Morrowind heading (the inverse of [`heading_of_yaw`]).
pub fn yaw_of_heading(heading: f32) -> f32 {
    heading_of_yaw(heading)
}

/// Reads the greeting GMSTs from the records (called by `walkers::add`).
pub fn insert_records(app: &mut App, db: &Arc<RecordDb>) {
    app.insert_resource(GreetSettings { gmst: GreetGmst::from_db(db) });
}

/// Registers the greeting's sets and systems (called by the walkers' drive setup).
pub(crate) fn add(app: &mut App) {
    app.init_resource::<GreetSettings>().init_resource::<GreetClock>();
    app.configure_sets(
        FixedUpdate,
        (
            GreetSystems::Decide.in_set(ControllerSystems::Drive).after(ReactionSystems::React),
            GreetSystems::Apply
                .in_set(ControllerSystems::Drive)
                .after(GreetSystems::Decide)
                .before(crate::directive_drive::drive_directives),
            GreetSystems::Hold.in_set(ControllerSystems::Drive).after(ReactionSystems::Apply),
        ),
    )
    .add_systems(
        FixedUpdate,
        (
            (attach, decide).chain().in_set(GreetSystems::Decide),
            apply.in_set(GreetSystems::Apply),
            hold.in_set(GreetSystems::Hold),
        ),
    );
}

/// Every AI actor with a mover greets.
#[allow(clippy::type_complexity)]
fn attach(mut commands: Commands, fresh: Query<Entity, (With<AiActor>, With<Mover>, Without<Greeting>)>) {
    for e in &fresh {
        commands.entity(e).try_insert(Greeting::default());
    }
}

/// The host player's body facts the greeting reads.
struct HostBody {
    feet: sf_coords::WorldPos,
    dead: bool,
    hidden: bool,
}

/// Steps every machine; a greeting that starts asks for its Hello (module docs).
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments, clippy::type_complexity)] // system parameters
fn decide(
    time: Res<Time>,
    settings: Res<GreetSettings>,
    mut clock: ResMut<GreetClock>,
    mut reach_log: Local<ReachLog>,
    head: Option<Res<HeadTrackSettings>>,
    percept: Option<Res<AiPerceptClock>>,
    perception: Option<Res<AiPerceptionSettings>>,
    table: Option<Res<StatTableRes>>,
    directives: Option<Res<AiDirectives>>,
    players: Query<(&ActionBody, Has<Dead>), With<PlayerBody>>,
    sheets: Query<(&PlayerCharacter, &Stats)>,
    mut actors: Query<
        (&AiActor, &Mover, &mut Greeting, Option<&Stats>, Option<&AiAwareness>, Option<&Reaction>, Has<KnockedDown>, Has<Dormant>),
        Without<Dead>,
    >,
) {
    let turn_only = clock.0.frame(time.delta_secs());
    let rules = head.map_or_else(HeadTrackRules::default, |h| h.rules);
    let player = players.iter().next().map(|(body, dead)| {
        let (invisibility, chameleon) = host_hidden_magnitudes(table.as_deref(), sheets.iter());
        HostBody { feet: body.loco.position, dead, hidden: rules.magically_hidden(invisibility, chameleon) }
    });
    let window = sight_window(percept.as_deref(), perception.as_deref());
    let keys = table.as_deref().map(|t| (t.0.find(HELLO), t.0.find(PARALYZE)));
    let mut reach: ReachCounts = (0, 0, 0, 0, 0, 0);
    for (actor, mover, mut greeting, sheet, awareness, reaction, knocked_down, dormant) in &mut actors {
        let greeting = &mut *greeting;
        greeting.pending = None;
        greeting.hello = HelloAsk::None;
        let Some(p) = player.as_ref().filter(|_| !dormant || knocked_down) else {
            greeting.action = GreetAction::default();
            continue;
        };
        let read = table.as_deref().zip(sheet).map(|(t, s)| s.view(&t.0));
        let stat = |idx: Option<sf_stats_bevy::sf_stats::StatIdx>| read.as_ref().zip(idx).map_or(0.0, |(r, i)| r.modified(i));
        let (hello_idx, paralyze_idx) = keys.unwrap_or((None, None));
        let serving = directives.as_deref().and_then(|d| d.get(actor.0)).and_then(|s| s.serving()).map(|d| &d.spec.kind);
        let (distance_units, angle_to_player) = greet_geometry(Vec3::ZERO, mover.loco.position.delta_f32(p.feet));
        let seen = seen_player(awareness, window);
        #[allow(clippy::cast_possible_truncation, reason = "the sheet stores OpenMW's int setting as f32")]
        let hello = stat(hello_idx) as i32;
        let input = GreetInput {
            hello,
            distance_units,
            angle_to_player,
            heading: heading_of_yaw(mover.loco.motion.yaw),
            in_combat: matches!(serving, Some(DirectiveKind::Fight(_))),
            swimming: matches!(mover.loco.mode, MoverMode::Swim { .. }),
            package_allows: !reaction.is_some_and(Reaction::owns)
                && matches!(serving, None | Some(DirectiveKind::StayNear { .. } | DirectiveKind::GoTo(_))),
            player_dead: p.dead,
            paralyzed: stat(paralyze_idx) > 0.0,
            player_hidden: p.hidden,
            los: seen,
            aware: seen,
            // `say` fails for a knocked-down actor; otherwise the voice answers (Apply).
            can_say_hello: !knocked_down,
            movement_forced: false,
            smooth_movement: SMOOTH_MOVEMENT,
            turn_only,
        };
        if !turn_only && !dormant && hello > 0 && distance_units <= settings.gmst.hello_distance(hello) {
            ReachLog::count(&mut reach, &input, greeting.greet.state);
        }
        let mut trial = greeting.greet;
        let mut action = greeting_step(&mut trial, &input, &settings.gmst);
        if turn_only && trial.state == GreetingState::InProgress {
            // `turnActorToFacePlayer` runs on full updates only; its stop holds until the next one
            // (`AiWander` keeps a greeting actor idle while the greeting is in progress).
            action.hold_still |= greeting.action.hold_still;
        }
        if action.say_hello {
            greeting.pending = Some(input);
            greeting.hello = HelloAsk::Asked;
            greeting.action = GreetAction::default();
        } else {
            greeting.greet = trial;
            greeting.action = action;
        }
    }
    if !turn_only && player.is_some() {
        reach_log.offer(reach, time.elapsed_secs_f64());
    }
}

/// Finishes the asked steps with the voice's answer (unanswered: cannot say) and plays `idle2`
/// when a hold starts.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn apply(
    settings: Res<GreetSettings>,
    mut log: Local<GreetLog>,
    mut actors: Query<(&Mover, &mut Greeting)>,
    mut animators: Query<&mut Animator>,
) {
    for (mover, mut greeting) in &mut actors {
        let greeting = &mut *greeting;
        if let Some(mut input) = greeting.pending.take() {
            input.can_say_hello = greeting.hello == HelloAsk::Said;
            greeting.action = greeting_step(&mut greeting.greet, &input, &settings.gmst);
            if greeting.action.say_hello {
                greeting.stats.greeted += 1;
                log.greeted += 1;
            } else {
                greeting.stats.silent += 1;
                log.silent += 1;
            }
            let total = log.greeted + log.silent;
            if total >= log.logged.saturating_mul(2).max(1) {
                log.logged = total;
                info!("greeting: {} greeted ({} without a voice)", log.greeted, log.silent);
            }
        }
        if !greeting.action.hold_still {
            greeting.idle2 = false;
        } else if !greeting.idle2 {
            greeting.idle2 = true;
            if let Some(mut animator) = mover.animator.and_then(|a| animators.get_mut(a).ok()) {
                animator.set_float(gp::IDLE_VARIANT, 2.0);
                animator.fire(gp::FIDGET);
            }
        }
    }
}

/// Writes the greeting's stand and turn (module docs).
#[allow(clippy::type_complexity)]
fn hold(mut actors: Query<(&Greeting, &Mover, &mut MoveIntent, Option<&mut Avoider>), (Without<Dormant>, Without<Held>)>) {
    for (greeting, mover, mut intent, avoider) in &mut actors {
        let action = greeting.action;
        let facing = action.turn_to.map(yaw_of_heading);
        if action.hold_still {
            intent.0 = LocomotionIntent { desired_facing: Some(facing.unwrap_or(mover.loco.motion.yaw)), ..LocomotionIntent::IDLE };
            if let Some(mut avoider) = avoider {
                avoider.travel = Vec3::ZERO;
            }
        } else if facing.is_some() {
            intent.0.desired_facing = facing;
        }
    }
}

#[cfg(test)]
mod pure_tests {
    use std::f32::consts::{FRAC_PI_2, PI};

    use super::*;

    #[test]
    fn yaw_and_heading_convert_both_ways() {
        // Engine yaw 0 faces +Z (south), PI faces -Z (north), PI/2 faces +X (east).
        assert!((heading_of_yaw(0.0).abs() - PI).abs() < 1e-6);
        assert!(heading_of_yaw(PI).abs() < 1e-6);
        assert!((heading_of_yaw(FRAC_PI_2) - FRAC_PI_2).abs() < 1e-6);
        assert!((heading_of_yaw(-FRAC_PI_2) + FRAC_PI_2).abs() < 1e-6);
        for yaw in [0.3_f32, -2.0, 1.4] {
            assert!((yaw_of_heading(heading_of_yaw(yaw)) - yaw).abs() < 1e-5);
        }
        // The heading greet_geometry gives toward a player north of the actor, as a yaw: north.
        let (_, h) = greet_geometry(Vec3::ZERO, Vec3::new(0.0, 0.0, -2.0));
        assert!((yaw_of_heading(h).abs() - PI).abs() < 1e-6);
    }

    #[test]
    fn the_reach_line_counts_the_first_reason_and_logs_changes_at_most_once_a_second() {
        let open = GreetInput {
            hello: 30,
            distance_units: 100.0,
            angle_to_player: 0.0,
            heading: 0.0,
            in_combat: false,
            swimming: false,
            package_allows: true,
            player_dead: false,
            paralyzed: false,
            player_hidden: false,
            los: true,
            aware: true,
            can_say_hello: true,
            movement_forced: false,
            smooth_movement: true,
            turn_only: false,
        };
        let mut c: ReachCounts = (0, 0, 0, 0, 0, 0);
        let none = GreetingState::None;
        ReachLog::count(&mut c, &GreetInput { in_combat: true, package_allows: false, ..open }, none);
        ReachLog::count(&mut c, &GreetInput { swimming: true, ..open }, none);
        ReachLog::count(&mut c, &GreetInput { package_allows: false, los: false, ..open }, none);
        ReachLog::count(&mut c, &GreetInput { aware: false, player_dead: true, ..open }, none);
        ReachLog::count(&mut c, &GreetInput { player_hidden: true, ..open }, none);
        ReachLog::count(&mut c, &open, none);
        ReachLog::count(&mut c, &open, GreetingState::InProgress);
        ReachLog::count(&mut c, &open, GreetingState::Done);
        assert_eq!(c, (8, 1, 2, 1, 1, 2));

        let mut log = ReachLog::default();
        log.offer(c, 10.0);
        assert_eq!((log.last, log.at_s), (Some(c), Some(10.0)));
        let changed = (1, 0, 0, 0, 0, 1);
        log.offer(changed, 10.5);
        assert_eq!(log.last, Some(c), "a change within the second waits");
        log.offer(changed, 11.0);
        assert_eq!((log.last, log.at_s), (Some(changed), Some(11.0)));
        log.offer(changed, 20.0);
        assert_eq!(log.at_s, Some(11.0), "unchanged counts are not logged again");
    }
}
