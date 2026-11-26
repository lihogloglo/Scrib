//! The directive stack drives the routine (`docs/specs/ai-adapter.md` "Packages -> directives",
//! slice `MA2d2`): each fixed tick [`drive_directives`] advances every actor's
//! [`DirectiveStack`] on the game clock (`Limit`s expire through [`mw_ai::MorrowindPolicy`]) and lets
//! the directive it serves pick what the routine does.
//!
//! - **`StayNear { radius, .. }`** (`AI_W`): the wander ([`Walker`]) runs as before, around the
//!   directive's home and radius, with the idle table of its package ([`AiPackageExtras`]). The
//!   home is where the actor stands when the directive first serves (`MA2d4`; `OpenMW`
//!   `aiwander.cpp` @ 5f3aafb, 293-297: `mInitialActorPosition`, stored once), not the spec's
//!   point: an `AI_W` under an `AI_T` wanders around the arrival point. It is taken when no
//!   reaction owns the actor, kept by id through repeats (as `OpenMW`'s clone keeps it) and not
//!   saved: after a load or a respawn the first serve takes it again, where the actor stands
//!   then (`OpenMW` saves it; ours is cheap to re-take and a respawn stands at its placement). A
//!   record wander served at spawn gets the spawn point, the planner's home already, so nothing
//!   changes for it; a different one (the next `AI_W`, a script's) re-targets the planner and
//!   re-chooses. An actor without a [`Walker`] (spawned with no `AI_W` distance) gets one, built
//!   as `attach` builds it ([`crate::walkers::new_walker`]); a radius of 0 stands, as at spawn.
//!   The `AI_W` duration is counted once, by the directive's `Limit` (the wander driver keeps no
//!   timer of its own).
//! - **`GoTo(location)`** (`AI_T`): the routine pauses (as under [`Reaction::owns`]) and the
//!   actor walks there with a `MoveTo` operator ([`OpRunner`]) over its [`NavCorridor`]. It is
//!   done (`finish` through the policy: a repeating one goes to the bottom, the next top takes
//!   over) when the corridor arrives, or by `OpenMW`'s end condition (`aitravel.cpp`
//!   `AiTravel::execute` @ 5f3aafb, lines 101-123): planar distance minus half the actor's
//!   height within `max(64 units, speed * dt)` ([`TRAVEL_END_TOLERANCE_M`]) for
//!   `TRAVEL_FINISH_TIME` = 2 s ([`TRAVEL_FINISH_S`]). A destination farther than 7168 units
//!   (`isWithinMaxRange`, lines 23-29; [`TRAVEL_MAX_RANGE_M`]) is not walked: the actor stands,
//!   the directive stays (as `OpenMW`'s package does). A failed walk (no path) retries after
//!   [`TRAVEL_RETRY_S`].
//! - **An empty stack** (`MA2d4`): the walker stands where it is ([`DirectiveDrive::stands`]),
//!   as `OpenMW` runs nothing for an empty `AiSequence` (`aisequence.cpp` @ 5f3aafb, 264-268):
//!   after a `GoTo` the actor stays at its destination, after a wander's `Limit` where it was.
//!   A record-only actor reaches an empty stack only when its `AI_W` duration runs out, where it
//!   stood before too.
//! - **Other kinds** (Follow, Escort, Use, Fight, Custom), a `GoTo` into another worldspace or
//!   off the navmesh: the routine runs as before (one `debug!` per kind; later slices).
//!
//! **Repeats.** A repeating directive comes back under a new id; its extras and home move to it
//! ([`Renewing`]).
//!
//! **Script reads.** Each tick an actor runs, what its stack served is written to
//! [`AiRuns`] (`GetAIPackageDone`, `GetCurrentAIPackage`: [`crate::script_ai`]).
//!
//! **Priority.** The perception reactions ([`crate::reaction`], which run first) keep priority:
//! while one owns the actor this drive writes nothing (the stack still advances); when it hands
//! back, a `GoTo` restarts its walk from where the reaction left the actor.

use std::collections::BTreeSet;

use bevy_app::App;
use bevy_ecs::prelude::*;
use bevy_log::debug;
use bevy_math::Vec3;
use bevy_time::prelude::*;
use mw_ai::Wander;
use mw_ai::packages::PackageExtra;
use sf_ai::{
    DirectiveId, DirectiveKind, DirectiveStack, ExecConfig, ExecContext, ExecGait, FaceTarget, GameTime, NavCommand, OperatorKind,
};
use sf_ai_bevy::{AiActor, AiDirectives, AiExecSettings};
use sf_controller::LocomotionIntent;
use sf_character_bevy::Character;
use sf_controller_bevy::{Avoider, Dormant, MoveIntent, Mover, MoverMode};
use sf_coords::WorldPos;
use sf_nav::DEFAULT_EXTENT;
use sf_nav_bevy::{NavCorridor, NavMeshes, NavQueries};
use sf_sky_bevy::GameClock;
use sf_tunables_bevy::TunablesAppExt;
use sf_tunables_bevy::sf_tunables::{Def, Lifetime, Net, Scope, Source, Unit, def};
use sf_world::Location;

use crate::directives::{AiPackageExtras, Renewing};
use crate::reaction::{OpRunner, Reaction, RunStep};
use crate::script_ai::{AiRuns, Served, record_runs};
use crate::walkers::{
    ActorSpace, WalkerContent, WanderVocab, Walker, clear_corridor_goal, nav_move_state, new_walker, record_walk_speed,
};

/// `mw.ai.travel.end_tolerance_m`: [`TravelSettings::end_tolerance_m`].
pub const TRAVEL_END_TOLERANCE_M: Def<f32> = def!(float "mw.ai.travel.end_tolerance_m" = 0.9144)
    .range(0.0, 10.0)
    .unit(Unit::Meters)
    .help("A GoTo is done once the actor stays this close (planar, minus half its height) for the finish time")
    .source(Source::Reference("OpenMW aitravel.cpp endTolerance max(64 units, speed * dt)"))
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `mw.ai.travel.finish_s`: [`TravelSettings::finish_s`].
pub const TRAVEL_FINISH_S: Def<f32> = def!(float "mw.ai.travel.finish_s" = 2.0)
    .range(0.0, 30.0)
    .unit(Unit::Seconds)
    .help("Seconds within the end tolerance before a GoTo the corridor did not reach counts as done")
    .source(Source::Reference("OpenMW aitravel.cpp TRAVEL_FINISH_TIME"))
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `mw.ai.travel.max_range_m`: [`TravelSettings::max_range_m`].
pub const TRAVEL_MAX_RANGE_M: Def<f32> = def!(float "mw.ai.travel.max_range_m" = 102.4)
    .range(0.0, 10_000.0)
    .unit(Unit::Meters)
    .help("A GoTo destination farther than this is not walked (the actor stands, the directive stays)")
    .source(Source::Reference("OpenMW aitravel.cpp isWithinMaxRange 7168 units"))
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `mw.ai.travel.retry_s`: [`TravelSettings::retry_s`].
pub const TRAVEL_RETRY_S: Def<f32> = def!(float "mw.ai.travel.retry_s" = 2.0)
    .range(0.0, 60.0)
    .unit(Unit::Seconds)
    .help("Seconds a GoTo whose walk failed (no path) stands before it tries again")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// The `GoTo` executor's live values, bound from the `mw.ai.travel.*` tunables.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct TravelSettings {
    pub end_tolerance_m: f32,
    pub finish_s: f32,
    pub max_range_m: f32,
    pub retry_s: f32,
}

impl Default for TravelSettings {
    fn default() -> Self {
        Self {
            end_tolerance_m: TRAVEL_END_TOLERANCE_M.default_value(),
            finish_s: TRAVEL_FINISH_S.default_value(),
            max_range_m: TRAVEL_MAX_RANGE_M.default_value(),
            retry_s: TRAVEL_RETRY_S.default_value(),
        }
    }
}

/// Binds the `mw.ai.travel.*` tunables to [`TravelSettings`].
pub(crate) fn add_travel_tunables(app: &mut App) {
    app.init_resource::<TravelSettings>();
    let _tol = app
        .add_tunable(&TRAVEL_END_TOLERANCE_M)
        .bind_resource(|s: &mut TravelSettings, v| s.end_tolerance_m = v);
    let _finish = app.add_tunable(&TRAVEL_FINISH_S).bind_resource(|s: &mut TravelSettings, v| s.finish_s = v);
    let _range = app.add_tunable(&TRAVEL_MAX_RANGE_M).bind_resource(|s: &mut TravelSettings, v| s.max_range_m = v);
    let _retry = app.add_tunable(&TRAVEL_RETRY_S).bind_resource(|s: &mut TravelSettings, v| s.retry_s = v);
}

/// What an actor's directives did since spawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DirectiveDriveStats {
    /// `GoTo` directives taken up.
    pub goto_started: u32,
    /// `GoTo` directives finished (arrived).
    pub goto_finished: u32,
    /// `GoTo` walks that failed (no path) and wait to retry.
    pub goto_failed_walks: u32,
    /// Times a `StayNear` re-targeted the wander (a home, radius or idle table it did not have).
    pub wander_retargets: u32,
    /// Walkers given to the actor by a `StayNear` (it spawned without one).
    pub walkers_given: u32,
}

/// The `GoTo` being served.
#[derive(Debug)]
struct GoTo {
    id: DirectiveId,
    goal: Location,
    runner: OpRunner,
    /// Whether `runner` holds the walk (false: start it, after `retry_left`).
    running: bool,
    /// Seconds spent within the end tolerance (`OpenMW`'s `mDestinationTimer`, counting up).
    near_s: f32,
    retry_left: f32,
}

/// An actor's directive executor state (module docs). Inserted with the stack's extras when the
/// stack is seeded (`crate::directives::seed_directives`).
#[derive(Component, Debug, Default)]
pub struct DirectiveDrive {
    goto: Option<GoTo>,
    /// Each `StayNear`'s home, by directive id: where the actor stood when it first served.
    homes: Vec<(DirectiveId, WorldPos)>,
    /// The stack is empty: the walker stands.
    stands: bool,
    pub stats: DirectiveDriveStats,
}

impl DirectiveDrive {
    /// Whether a directive's executor drives the actor (the wander stays paused).
    pub fn owns(&self) -> bool {
        self.goto.is_some()
    }

    /// Whether the actor has no directive left: its walker stands where it is (module docs).
    pub fn stands(&self) -> bool {
        self.stands
    }

    /// The home of the `StayNear` `id`, once it served.
    pub fn home(&self, id: DirectiveId) -> Option<WorldPos> {
        self.homes.iter().find(|(d, _)| *d == id).map(|&(_, p)| p)
    }

    /// The home of `id`, taken at `pos` on its first serve; none while a reaction owns the actor
    /// (`owned`) before it has one.
    fn home_at_first_serve(&mut self, id: DirectiveId, pos: WorldPos, owned: bool) -> Option<WorldPos> {
        if let Some(home) = self.home(id) {
            return Some(home);
        }
        if owned {
            return None;
        }
        self.homes.push((id, pos));
        Some(pos)
    }

    /// Moves the homes to each repeat's new id (`renewed`: old, new) and drops those of
    /// directives no longer on `stack`.
    pub(crate) fn follow(&mut self, renewed: &[(DirectiveId, DirectiveId)], stack: &DirectiveStack) {
        for &(old, new) in renewed {
            for (d, _) in self.homes.iter_mut().filter(|(d, _)| *d == old) {
                *d = new;
            }
        }
        self.homes.retain(|(d, _)| stack.get(*d).is_some());
    }
}

/// Moves `extras` and `drive`'s homes to the new ids of what `policy` saw repeat.
pub(crate) fn follow_renewals(
    policy: &Renewing,
    drive: Option<&mut DirectiveDrive>,
    extras: Option<&mut Mut<'_, AiPackageExtras>>,
    stack: &DirectiveStack,
) {
    let renewed = policy.take();
    if !renewed.is_empty()
        && let Some(extras) = extras
    {
        extras.follow(&renewed);
    }
    if let Some(drive) = drive {
        drive.follow(&renewed, stack);
    }
}

/// What one `GoTo` step decided.
enum GoToStep {
    /// The corridor walks toward the goal.
    Walking,
    /// Out of range, or waiting to retry a failed walk.
    Standing,
    /// The walk failed this tick (no path): stands, retries later.
    Failed,
    /// Arrived.
    Done,
}

/// `FixedUpdate`, after the reactions and before the wander drive and [`sf_nav_bevy::NavSystems::Corridor`]:
/// advances every stack and runs its serving directive (module docs).
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments, clippy::too_many_lines, clippy::type_complexity)]
pub(crate) fn drive_directives(
    time: Res<Time>,
    clock: Option<Res<GameClock>>,
    store: Option<ResMut<AiDirectives>>,
    exec_settings: Option<Res<AiExecSettings>>,
    travel: Option<Res<TravelSettings>>,
    vocab: Option<Res<WanderVocab>>,
    meshes: Option<Res<NavMeshes>>,
    mut queries: Option<ResMut<NavQueries>>,
    runs: Option<ResMut<AiRuns>>,
    content: Option<Res<WalkerContent>>,
    mut commands: Commands,
    mut logged: Local<BTreeSet<&'static str>>,
    mut actors: Query<(
        (Entity, &AiActor, &Mover, &ActorSpace, &mut DirectiveDrive, Option<&mut AiPackageExtras>, Has<Dormant>),
        (
            Option<&Reaction>,
            Option<&mut Walker>,
            Option<&mut NavCorridor>,
            &mut MoveIntent,
            Option<&mut Avoider>,
            Option<&Character>,
            Option<&crate::greeting::Greeting>,
        ),
    )>,
) {
    let Some(mut store) = store else {
        return;
    };
    let now = clock.map_or_else(GameTime::default, |c| GameTime::from(c.now));
    let exec_cfg = exec_settings.map_or_else(ExecConfig::default, |s| s.config);
    let travel = travel.map_or_else(TravelSettings::default, |s| *s);
    let ops = vocab.as_ref().map_or_else(|| mw_ai::wander_library().operators, |v| v.0.operators);
    let dt = time.delta_secs().min(0.1);
    let mut events = Vec::new();
    let mut ran = Vec::new();
    let policy = Renewing::default();
    for (
        (entity, actor, mover, space, mut drive, mut extras, dormant),
        (reaction, mut walker, mut nav, mut intent, avoider, character, greeting),
    ) in &mut actors
    {
        let Some(stack) = store.get_mut(actor.0) else {
            continue;
        };
        let served = Served::of(stack);
        stack.advance(now, &policy, &mut events);
        follow_renewals(&policy, Some(&mut drive), extras.as_mut(), stack);
        if dormant {
            continue;
        }
        let stood = drive.stands;
        drive.stands = stack.is_empty();
        if stood && !drive.stands {
            resume_wander(walker.as_deref_mut(), mover.loco.motion.yaw);
        }
        ran.push((actor.0, served));
        let pos = mover.loco.position;
        let yaw = mover.loco.motion.yaw;
        let serving = stack.serving().map(|d| (d.id, d.spec.kind.clone()));
        let walkable_goto = match &serving {
            Some((id, DirectiveKind::GoTo(goal))) => {
                let loc = Location::new(space.0.clone(), pos);
                let on_navmesh = matches!(mover.loco.mode, MoverMode::Ground)
                    && nav.as_ref().is_some_and(|n| {
                        meshes.as_ref().is_some_and(|m| m.0.nearest(&loc, n.class, DEFAULT_EXTENT, &n.filter).is_some())
                    });
                if goal.worldspace == space.0 && on_navmesh {
                    Some((*id, goal.clone()))
                } else {
                    if logged.insert(if on_navmesh { "go_to (other worldspace)" } else { "go_to (off the navmesh)" }) {
                        debug!("directives: a go_to the walkers cannot run yet; the routine keeps the actor");
                    }
                    None
                }
            }
            _ => None,
        };
        // A GoTo that stopped being served (finished, cancelled, out-ranked): the routine resumes.
        if drive.goto.as_ref().is_some_and(|g| walkable_goto.as_ref().is_none_or(|(id, _)| *id != g.id)) {
            drive.goto = None;
            if !reaction.is_some_and(Reaction::owns) {
                if let Some(n) = nav.as_deref_mut() {
                    clear_corridor_goal(n, queries.as_deref_mut());
                }
                resume_wander(walker.as_deref_mut(), yaw);
            }
        }
        if let Some((id, goal)) = walkable_goto {
            let drive = &mut *drive;
            if drive.goto.is_none() {
                drive.stats.goto_started += 1;
                drive.goto = Some(GoTo { id, goal, runner: OpRunner::default(), running: false, near_s: 0.0, retry_left: 0.0 });
            }
            let g = drive.goto.as_mut().expect("set above");
            if let Some(w) = walker.as_deref_mut() {
                // The paused wander's held stand must not overwrite this walk.
                w.release_held_intent();
            }
            if reaction.is_some_and(Reaction::owns) || greeting.is_some_and(crate::greeting::Greeting::holds) {
                // The reaction drives (MA4c2: or a greeting holds the actor still); the walk
                // restarts from wherever it hands back.
                g.running = false;
                g.near_s = 0.0;
                continue;
            }
            let nav = nav.as_deref_mut().expect("a walkable go_to has a corridor");
            let speed = reaction
                .and_then(|r| r.walk_speed)
                .or_else(|| walker.as_ref().map(|w| w.driver.walk_speed))
                .unwrap_or(nav.follower.speed);
            let half_height = 0.5 * mover.params.height;
            let ctx = ExecContext { location: Location::new(space.0.clone(), pos), yaw, dt, nav: nav_move_state(nav), decide: true };
            let step = step_goto(g, &ctx, &exec_cfg, ops, &travel, speed, half_height, nav, &mut intent, queries.as_deref_mut());
            match step {
                GoToStep::Walking => {
                    if let Some(mut avoider) = avoider {
                        avoider.travel = nav
                            .corridor()
                            .and_then(|c| c.corners().first())
                            .map_or(Vec3::ZERO, |c| pos.delta_f32(c.pos).with_y(0.0).normalize_or_zero());
                    }
                }
                GoToStep::Done => {
                    clear_corridor_goal(nav, queries.as_deref_mut());
                    stack.finish(id, now, &policy, &mut events);
                    follow_renewals(&policy, Some(&mut *drive), extras.as_mut(), stack);
                    drive.goto = None;
                    drive.stats.goto_finished += 1;
                    // An empty stack stands here (module docs); otherwise the next top takes over.
                    drive.stands = stack.is_empty();
                    resume_wander(walker.as_deref_mut(), yaw);
                }
                GoToStep::Failed => drive.stats.goto_failed_walks += 1,
                GoToStep::Standing => {}
            }
            continue;
        }
        if let Some((id, kind)) = &serving {
            let owned = reaction.is_some_and(Reaction::owns);
            let home = if matches!(kind, DirectiveKind::StayNear { .. }) {
                drive.home_at_first_serve(*id, pos, owned)
            } else {
                None
            };
            let routine = Routine { id: *id, kind, home, extras: extras.as_deref(), yaw };
            if let Some(wander) = serve_routine(&routine, walker.as_deref_mut(), &mut drive.stats, &mut logged)
                && let Some(home) = home
            {
                // MA2d4: a StayNear on an actor spawned without a wander gets the walker `attach`
                // would have given it (the same speed and idle gate).
                let speed = content
                    .as_deref()
                    .zip(character)
                    .and_then(|(c, ch)| record_walk_speed(c, ch))
                    .or_else(|| reaction.and_then(|r| r.walk_speed))
                    .or_else(|| nav.as_ref().map(|n| n.follower.speed));
                if let Some(speed) = speed {
                    let at = Location::new(space.0.clone(), home);
                    let idle_mult = content.as_ref().map(|c| c.idle_mult);
                    commands.entity(entity).try_insert(new_walker(entity, wander, &at, speed, idle_mult, vocab.as_deref()));
                    drive.stats.walkers_given += 1;
                } else if logged.insert("stay_near (no walk speed)") {
                    debug!("directives: a stay_near on an actor with no walk speed; it stands");
                }
            }
        }
    }
    if let Some(mut runs) = runs {
        record_runs(&mut runs, &ran, &events);
    }
    for e in &events {
        debug!("directive {:?} of actor {:?} ended ({:?})", e.id, e.actor, e.end);
    }
}

/// A served directive the routine runs under.
struct Routine<'a> {
    id: DirectiveId,
    kind: &'a DirectiveKind,
    /// A `StayNear`'s home (none while a reaction owns the actor before its first serve).
    home: Option<WorldPos>,
    extras: Option<&'a AiPackageExtras>,
    yaw: f32,
}

/// The routine under a served directive that does not drive the actor itself: a `StayNear`
/// points the wander at its home and radius, or returns the wander to build a walker from when
/// the actor has none; other kinds have no executor yet (module docs).
fn serve_routine(
    r: &Routine<'_>,
    walker: Option<&mut Walker>,
    stats: &mut DirectiveDriveStats,
    logged: &mut BTreeSet<&'static str>,
) -> Option<Wander> {
    match r.kind {
        DirectiveKind::StayNear { radius, .. } => {
            let home = r.home?;
            let package = match r.extras.and_then(|e| e.get(r.id)) {
                Some(PackageExtra::Wander(wander)) => Some(*wander),
                _ => None,
            };
            if let Some(w) = walker {
                if retarget(w, home, *radius, package.map(|p| p.idle_chances), r.yaw) {
                    stats.wander_retargets += 1;
                }
            } else if *radius > 0.0 {
                // The stack's `Limit` counts the duration; the walker keeps none.
                let wander = package.unwrap_or(Wander {
                    distance: *radius,
                    duration_hours: 0.0,
                    game_hour: 0,
                    idle_chances: [0; 8],
                    repeat: false,
                });
                return Some(Wander { distance: *radius, ..wander });
            }
        }
        _ if logged.insert(r.kind.name()) => {
            debug!("directives: no executor for {} yet; the routine keeps the actor", r.kind.name());
        }
        _ => {}
    }
    None
}

/// One tick of a `GoTo` walk ([`GoToStep`]; arrival: module docs).
#[allow(clippy::too_many_arguments)]
fn step_goto(
    g: &mut GoTo,
    ctx: &ExecContext,
    cfg: &ExecConfig,
    ops: sf_ai::MoveOperatorIds,
    travel: &TravelSettings,
    speed: f32,
    half_height: f32,
    nav: &mut NavCorridor,
    intent: &mut MoveIntent,
    queries: Option<&mut NavQueries>,
) -> GoToStep {
    let to_goal = ctx.location.pos.delta_f32(g.goal.pos);
    if to_goal.length() > travel.max_range_m {
        // OpenMW `isWithinMaxRange`: not walked, the package stays.
        g.running = false;
        stand(nav, intent, ctx.yaw, queries);
        return GoToStep::Standing;
    }
    // OpenMW's end condition: close enough for long enough (the corridor may never arrive in a crowd).
    let tolerance = travel.end_tolerance_m.max(speed * ctx.dt);
    if to_goal.with_y(0.0).length() - half_height <= tolerance {
        g.near_s += ctx.dt;
        if g.near_s >= travel.finish_s {
            return GoToStep::Done;
        }
    } else {
        g.near_s = 0.0;
    }
    if !g.running {
        if g.retry_left > 0.0 {
            g.retry_left -= ctx.dt;
            stand(nav, intent, ctx.yaw, queries);
            return GoToStep::Standing;
        }
        g.runner.start([Some(OperatorKind::MoveTo { goal: g.goal.clone(), gait: ExecGait::Walk, speed }), None]);
        g.running = true;
    }
    match g.runner.step(ctx, cfg, ops) {
        RunStep::Running(out) => {
            if let NavCommand::MoveTo { goal, gait, speed } = &out.nav {
                if nav.goal.as_ref() != Some(goal) || nav.is_arrived() || nav.failed.is_some() {
                    clear_corridor_goal(nav, queries);
                    nav.set_goal(goal.clone());
                }
                nav.follower.gait = sf_ai_bevy::to_controller_gait(*gait);
                nav.follower.speed = *speed;
                GoToStep::Walking
            } else {
                stand(nav, intent, ctx.yaw, queries);
                GoToStep::Standing
            }
        }
        RunStep::Done { failed: false } => GoToStep::Done,
        RunStep::Done { failed: true } => {
            g.running = false;
            g.retry_left = travel.retry_s;
            stand(nav, intent, ctx.yaw, queries);
            GoToStep::Failed
        }
    }
}

/// Stands where the actor is, facing `yaw` (no corridor goal).
fn stand(nav: &mut NavCorridor, intent: &mut MoveIntent, yaw: f32, queries: Option<&mut NavQueries>) {
    clear_corridor_goal(nav, queries);
    intent.0 = LocomotionIntent { desired_facing: Some(yaw), ..LocomotionIntent::IDLE };
}

/// Points the wander at `home` / `radius` (and the idle table, when the directive has one); true
/// if anything changed (the paused step ends and the wander re-chooses).
fn retarget(walker: &mut Walker, home: WorldPos, radius: f32, idles: Option<[u8; 8]>, yaw: f32) -> bool {
    let planner = &mut walker.driver.planner;
    let idles = idles.unwrap_or(planner.wander.idle_chances);
    #[allow(clippy::float_cmp)] // the same package's value: equal unless it is another directive
    let same = planner.home == home && planner.wander.distance == radius && planner.wander.idle_chances == idles;
    if same {
        return false;
    }
    planner.home = home;
    planner.wander.distance = radius;
    planner.wander.idle_chances = idles;
    resume_wander(Some(walker), yaw);
    true
}

/// A paused wander re-chooses from where the actor stands (a turn to the current yaw ends its
/// paused step at once), as after a reaction.
fn resume_wander(walker: Option<&mut Walker>, yaw: f32) {
    if let Some(w) = walker {
        w.driver.face(FaceTarget::Yaw(yaw), 0.0);
    }
}
