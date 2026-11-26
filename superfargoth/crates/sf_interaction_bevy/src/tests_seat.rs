//! `IN2b2` headless tests: the pose-slot default with the action layer (`SfActionPlugin` on flat
//! ground, `SEAT_RULES` with stand-in clips): approval → claim → seat offer → occupy → release.
//! Exact on claim events, answers and seat events.

use std::f32::consts::PI;
use std::sync::Arc;
use std::time::Duration;

use bevy_time::{TimePlugin, TimeUpdateStrategy};
use glam::{Quat, Vec3};
use sf_action::{
    ActionClass, ActionEvent, ActionState, BASE_RULES, CharacterId, CharacterParams, Command, Origin, RootPath, RuleKey,
    RuleSetBuilder, SEAT_RULES, SlotInfo, SlotTable,
};
use sf_action_bevy::{ActionBody, ActionCharacter, ActionRules, ActionSystems, ActionTick, SeatAim, SfActionPlugin};
use sf_anim::RootPose;
use sf_controller::{GroundHit, GroundProbe, HeightSurface, Locomotion, MoverParams, ProbeQuery, TerrainProbe};
use sf_coords::WorldPos;
use sf_ids::{ActorKey, PlayerKey};
use sf_interaction::{
    ClipFamily, Entrance, EventKind, IkTargets, InteractionEvent, InvalidReason, Occupancy, Phase, Refusal, Side, SlotDef,
    SlotKind, SlotRef, TagFilter, TagId, UserKey, VerbDef,
};
use sf_records::RecordId;
use sf_script_bevy::sf_script::Handle;
use sf_script_bevy::{PLAYER, ScriptsPlugin};
use sf_tunables_bevy::TunablesPlugin;
use smallvec::{SmallVec, smallvec};

use super::*;

const CHAIR: PlacementId = PlacementId(7);
const SLOT: SlotRef = SlotRef { placement: CHAIR, slot: 0 };
const NPC_HANDLE: Handle = Handle(500);
const ENTER: &str = "interact.sit.enter";
const LOOP: &str = "interact.sit.loop";
const EXIT: &str = "interact.sit.exit";

fn player() -> UserKey {
    UserKey::Player(PlayerKey(1))
}

fn npc() -> UserKey {
    UserKey::Actor(ActorKey(5))
}

fn sit() -> TagId {
    TagId::of("sit")
}

// --- Ground and clips ---------------------------------------------------------------------------

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

/// A sit-down stand-in: half a metre forward while turning round.
struct SitDown;

impl RootPath for SitDown {
    fn duration(&self) -> f32 {
        0.5
    }
    fn root_at(&self, t: f32) -> RootPose {
        let u = (t / 0.5).clamp(0.0, 1.0);
        RootPose::new(Vec3::new(0.0, 0.0, 0.5) * u, PI * u)
    }
}

/// A stand-up stand-in: half a metre forward.
struct StandUp;

impl RootPath for StandUp {
    fn duration(&self) -> f32 {
        0.4
    }
    fn root_at(&self, t: f32) -> RootPose {
        let u = (t / 0.4).clamp(0.0, 1.0);
        RootPose::new(Vec3::new(0.0, 0.0, 0.5) * u, 0.0)
    }
}

fn rules() -> ActionRules {
    let slots = SlotTable::new()
        .with("loco.move", SlotInfo::looping())
        .with("air.fall", SlotInfo::looping())
        .with("air.jump", SlotInfo::once(0.3))
        .with("air.land.normal", SlotInfo::once(0.25))
        .with("air.land.hard", SlotInfo::once(0.5))
        .with(ENTER, SlotInfo::once(0.5))
        .with(LOOP, SlotInfo::looping())
        .with(EXIT, SlotInfo::once(0.4))
        .with_root(ENTER, Arc::new(SitDown))
        .with_root(EXIT, Arc::new(StandUp));
    let mut b = RuleSetBuilder::new();
    b.add_ron(&Origin::Framework, BASE_RULES).add_ron(&Origin::Framework, SEAT_RULES);
    ActionRules { rules: b.build().expect("base and seat rules"), slots: Box::new(slots) }
}

// --- The chair ----------------------------------------------------------------------------------

/// One seat 0.6 m ahead of the origin, facing back at it; its front entrance 0.55 m in front of it.
fn chair() -> Arc<InteractableDef> {
    let seat = SlotDef {
        kind: SlotKind::Pose { family: TagId::of("seat") },
        local: Frame { translation: Vec3::new(0.1, 0.0, 0.6), rotation: Quat::from_rotation_y(PI) },
        band: 0.45,
        verb: VerbDef { verb: sit(), priority: 0, requires: TagFilter::default() },
        entrances: smallvec![Entrance { side: Side::Front, approach: Vec3::new(0.1, 0.0, 0.05) }],
        clips: ClipFamily("seat.chair".into()),
        ik: IkTargets::default(),
        reach: 0.0,
        occupancy: Occupancy::Pose,
        control: None,
        users: TagFilter::default(),
        group: None,
        queue: None,
    };
    Arc::new(InteractableDef {
        id: RecordId::new("chair"),
        slots: vec![seat],
        verbs: SmallVec::new(),
        destructible: None,
        carry: None,
        adverts: Vec::new(),
    })
}

// --- The app ------------------------------------------------------------------------------------

/// Presses for the next tick's command, by entity.
#[derive(Resource, Default)]
struct Presses(Vec<(Entity, ActionClass)>);

/// What the run saw, by fixed tick.
#[derive(Resource, Default)]
struct Log {
    changed: Vec<InteractionEvent>,
    answered: Vec<ClaimAnswered>,
    seat_events: Vec<(Entity, ActionEvent)>,
    movs: Vec<(Entity, RuleKey)>,
}

fn write_commands(tick: Res<ActionTick>, mut presses: ResMut<Presses>, mut characters: Query<(Entity, &mut ActionCharacter)>) {
    for (e, mut c) in &mut characters {
        let mut cmd = Command::empty(tick.0);
        for (_, class) in presses.0.iter().filter(|(p, _)| *p == e) {
            cmd.buttons.pressed.insert(*class);
            cmd.buttons.held.insert(*class);
        }
        c.command = cmd;
    }
    presses.0.clear();
}

fn record(
    mut log: ResMut<Log>,
    mut changed: MessageReader<ClaimChanged>,
    mut answered: MessageReader<ClaimAnswered>,
    characters: Query<(Entity, &ActionCharacter)>,
) {
    log.changed.extend(changed.read().map(|c| c.0));
    log.answered.extend(answered.read().cloned());
    for (e, c) in &characters {
        let seat = c.output.events.iter().filter(|ev| {
            matches!(
                ev.kind,
                sf_action::EventKind::SeatEntered(_) | sf_action::EventKind::SeatExited(_) | sf_action::EventKind::SeatRefused(_)
            )
        });
        let seen: Vec<(Entity, ActionEvent)> = seat.map(|ev| (e, *ev)).collect();
        log.seat_events.extend(seen);
        log.movs.push((e, c.state.mov));
    }
}

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((TimePlugin, TunablesPlugin, ScriptsPlugin, InteractionPlugin, SfActionPlugin::<Flat>::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
        .insert_resource(Flat(TerrainProbe(Plane)))
        .insert_resource(rules())
        .init_resource::<Presses>()
        .init_resource::<Log>()
        .add_systems(FixedUpdate, write_commands.in_set(ActionSystems::Command))
        .add_systems(FixedUpdate, record.after(SeatSystems::Events));
    app.update();
    let owner = app.world_mut().spawn_empty().id();
    app.world_mut().write_message(IndexChange::Insert {
        placement: CHAIR,
        def: chair(),
        object: Frame::default(),
        owner,
        disabled: SmallVec::new(),
    });
    app
}

/// A character at `at` facing +Z that uses slots as `user`.
fn spawn(app: &mut App, id: u64, at: Vec3, user: UserKey, actor: Handle, priority: u8) -> Entity {
    let character = ActionCharacter::new(CharacterId(id), ActionState::new(RuleKey::of("loco")), CharacterParams::default());
    let body = ActionBody::new(Locomotion::new(WorldPos::from_f64(at.as_dvec3()), 0.0), MoverParams::default(), WorldPos::ORIGIN);
    let user = InteractionUser { user, actor, priority, tags: SmallVec::new() };
    app.world_mut().spawn((character, body, user)).id()
}

fn frames(app: &mut App, n: usize) {
    for _ in 0..n {
        app.update();
    }
}

/// Runs frames until `done` holds (at most `max`); whether it did.
fn until(app: &mut App, max: usize, done: impl Fn(&App) -> bool) -> bool {
    for _ in 0..max {
        if done(app) {
            return true;
        }
        app.update();
    }
    done(app)
}

fn mov(app: &App, e: Entity) -> RuleKey {
    app.world().get::<ActionCharacter>(e).unwrap().state.mov
}

fn aim(app: &App, e: Entity) -> Option<sf_action::SeatSense> {
    app.world().get::<SeatAim>(e).unwrap().0
}

fn clips(app: &App, e: Entity) -> Option<SeatClips> {
    app.world().get::<SeatClips>(e).cloned()
}

/// The chair's clips as offered: its family, 0.45 m as 45 cm, in and out by its one (Front) entrance.
fn chair_clips() -> SeatClips {
    SeatClips { family: ClipFamily("seat.chair".into()), band_cm: 45, enter: Side::Front, exit: Side::Front }
}

fn claim(app: &App) -> Option<(UserKey, Phase)> {
    app.world().resource::<InteractionClaims>().0.claim_on(SLOT).map(|c| (c.user, c.phase))
}

fn kinds(app: &App) -> Vec<(UserKey, EventKind)> {
    app.world().resource::<Log>().changed.iter().filter(|e| e.slot == SLOT).map(|e| (e.user, e.kind)).collect()
}

fn seat_kinds(app: &App, e: Entity) -> Vec<sf_action::EventKind> {
    app.world().resource::<Log>().seat_events.iter().filter(|(x, _)| *x == e).map(|(_, ev)| ev.kind).collect()
}

fn ever(app: &App, e: Entity, rule: &str) -> bool {
    let key = RuleKey::of(rule);
    app.world().resource::<Log>().movs.iter().any(|(x, k)| *x == e && *k == key)
}

/// The player's sit command on the chair (what its focus builds).
fn sit_command() -> ActivateCommand {
    ActivateCommand {
        user: player(),
        actor: PLAYER,
        target: CHAIR,
        verb: Some(sit()),
        slot: Some(0),
        priority: 30,
        tags: SmallVec::new(),
    }
}

/// A grounded player at the origin who sat down on the chair through the activation pipeline.
fn seated() -> (App, Entity) {
    let mut app = app();
    let p = spawn(&mut app, 1, Vec3::ZERO, player(), PLAYER, 30);
    frames(&mut app, 10);
    app.world_mut().write_message(sit_command());
    // Approved in the frame's `Update`; claimed on the next fixed tick.
    frames(&mut app, 2);
    assert_eq!(claim(&app), Some((player(), Phase::Reserved)));
    let offered = aim(&app, p).expect("the claimed seat is offered");
    assert_eq!((offered.target, offered.slot), (seat_target(CHAIR), 0));
    assert_eq!(clips(&app, p), Some(chair_clips()), "the clips come with the offer");
    assert!(until(&mut app, 120, |a| mov(a, p) == RuleKey::of(LOOP)), "sat: {:?}", mov(&app, p));
    frames(&mut app, 2);
    (app, p)
}

// --- Tests --------------------------------------------------------------------------------------

#[test]
fn an_approved_sit_claims_then_the_body_sits_and_the_claim_is_occupied() {
    let (app, p) = seated();
    assert!(ever(&app, p, ENTER));
    assert_eq!(claim(&app), Some((player(), Phase::Occupied)));
    assert_eq!(kinds(&app), vec![(player(), EventKind::Claimed), (player(), EventKind::Entered)]);
    assert_eq!(seat_kinds(&app, p), vec![sf_action::EventKind::SeatEntered(0)]);
    assert_eq!(clips(&app, p), Some(chair_clips()), "seated: unchanged");
    assert_eq!(app.world().resource::<SeatUses>().get(player()).map(|u| u.phase), Some(SeatPhase::Seated));
    assert_eq!(app.world().resource::<SeatUses>().get(player()).and_then(|u| u.entry), Some(Side::Front));
    // The body sits on the slot frame (world), facing its way.
    let body = app.world().get::<ActionBody>(p).unwrap();
    let feet = body.loco.position.to_f64().as_vec3();
    assert!(feet.distance(Vec3::new(0.1, 0.0, 0.6)) < 1e-3, "{feet:?}");
    // The exit sense is the front entrance's approach point, relative to the feet.
    let exit = aim(&app, p).unwrap().exit;
    assert!((exit.position - Vec3::new(0.0, 0.0, -0.55)).length() < 1e-3, "{exit:?}");
}

#[test]
fn a_jump_stands_up_and_releases_the_claim() {
    let (mut app, p) = seated();
    app.world_mut().resource_mut::<Presses>().0.push((p, ActionClass::Jump));
    assert!(until(&mut app, 120, |a| mov(a, p) == RuleKey::of("loco")), "stood: {:?}", mov(&app, p));
    // Where the stand-up landed (the exit has no `stand`, so its speed carries a few cm after).
    let feet = app.world().get::<ActionBody>(p).unwrap().loco.position.to_f64().as_vec3();
    assert!(feet.distance(Vec3::new(0.1, 0.0, 0.05)) < 0.05, "stood at the front entrance: {feet:?}");
    frames(&mut app, 2);
    assert!(ever(&app, p, EXIT));
    assert_eq!(claim(&app), None);
    assert_eq!(
        kinds(&app),
        vec![(player(), EventKind::Claimed), (player(), EventKind::Entered), (player(), EventKind::Released)]
    );
    assert_eq!(seat_kinds(&app, p), vec![sf_action::EventKind::SeatEntered(0), sf_action::EventKind::SeatExited(0)]);
    assert_eq!(aim(&app, p), None);
    assert_eq!(clips(&app, p), None, "gone after the exit");
    assert!(app.world().resource::<SeatUses>().get(player()).is_none());
}

#[test]
fn removing_the_seat_while_seated_stands_the_body_up_and_frees_the_slot() {
    let (mut app, p) = seated();
    app.world_mut().write_message(IndexChange::Remove { placement: CHAIR, reason: InvalidReason::OwnerDestroyed });
    frames(&mut app, 1);
    assert_eq!(aim(&app, p), None, "the aim clears on the invalidating tick");
    assert_eq!(claim(&app), None);
    assert_eq!(app.world().resource::<SeatUses>().get(player()).map(|u| u.phase), Some(SeatPhase::Leaving));
    assert_eq!(clips(&app, p), Some(chair_clips()), "the stand-up still plays this seat's exit");
    assert!(until(&mut app, 120, |a| mov(a, p) == RuleKey::of("loco")), "stood: {:?}", mov(&app, p));
    frames(&mut app, 2);
    assert_eq!(
        kinds(&app),
        vec![
            (player(), EventKind::Claimed),
            (player(), EventKind::Entered),
            (player(), EventKind::Invalidated(InvalidReason::OwnerDestroyed)),
        ]
    );
    assert_eq!(seat_kinds(&app, p), vec![sf_action::EventKind::SeatEntered(0), sf_action::EventKind::SeatExited(0)]);
    assert!(app.world().resource::<SeatUses>().get(player()).is_none());
    assert_eq!(clips(&app, p), None, "gone once the lost seat's stand-up ended");
    // No release follows the exit: the hold ended with the object.
    let releases = app.world().resource::<Log>().answered.iter().filter(|a| matches!(a.request.op, Op::Release)).count();
    assert_eq!(releases, 0);
}

#[test]
fn a_second_user_on_an_occupied_seat_is_refused_and_stays_standing() {
    let (mut app, _p) = seated();
    let n = spawn(&mut app, 2, Vec3::new(1.0, 0.0, 0.0), npc(), NPC_HANDLE, 20);
    frames(&mut app, 10);
    // A script's `Activate` skips validation: the claim refuses it.
    app.world_mut().write_message(ScriptActivate { target: Handle(CHAIR.0), actor: NPC_HANDLE, verb: Some(sit()) });
    frames(&mut app, 30);
    let npc_answers: Vec<_> =
        app.world().resource::<Log>().answered.iter().filter(|a| a.request.user == npc()).map(|a| (a.request.op.clone(), a.result)).collect();
    assert_eq!(npc_answers.len(), 1, "{npc_answers:?}");
    assert!(matches!(npc_answers[0].0, Op::Claim { slot: SLOT, .. }));
    assert_eq!(npc_answers[0].1, Err(Refusal::Taken));
    assert_eq!(claim(&app), Some((player(), Phase::Occupied)));
    assert_eq!(aim(&app, n), None);
    assert_eq!(clips(&app, n), None, "a refused claim offers no clips");
    assert!(!ever(&app, n, ENTER));
    assert!(seat_kinds(&app, n).is_empty());
    assert!(app.world().resource::<SeatUses>().get(npc()).is_none());
}

#[test]
fn an_enter_refused_in_the_air_releases_the_claim_and_clears_the_aim() {
    let mut app = app();
    let p = spawn(&mut app, 1, Vec3::new(0.0, 6.0, 0.0), player(), PLAYER, 30);
    frames(&mut app, 2);
    app.world_mut().write_message(sit_command());
    frames(&mut app, 4);
    assert!(app.world().get::<ActionBody>(p).unwrap().loco.grounded.is_none(), "still falling");
    assert_eq!(seat_kinds(&app, p), vec![sf_action::EventKind::SeatRefused(0)]);
    assert_eq!(kinds(&app), vec![(player(), EventKind::Claimed), (player(), EventKind::Released)]);
    assert_eq!(claim(&app), None);
    assert_eq!(aim(&app, p), None);
    assert_eq!(clips(&app, p), None, "gone after the refused enter");
    assert!(app.world().resource::<SeatUses>().get(player()).is_none());
    assert!(!ever(&app, p, ENTER));
}

#[test]
fn a_claim_lost_before_sitting_removes_the_clips_on_that_tick() {
    let mut app = app();
    let p = spawn(&mut app, 1, Vec3::ZERO, player(), PLAYER, 30);
    frames(&mut app, 10);
    app.world_mut().write_message(sit_command());
    frames(&mut app, 2);
    assert_eq!(clips(&app, p), Some(chair_clips()));
    assert_ne!(app.world().resource::<SeatUses>().get(player()).map(|u| u.phase), Some(SeatPhase::Seated), "still entering");
    app.world_mut().write_message(IndexChange::Remove { placement: CHAIR, reason: InvalidReason::OwnerDestroyed });
    frames(&mut app, 1);
    assert_eq!(aim(&app, p), None);
    assert_eq!(clips(&app, p), None);
    assert!(app.world().resource::<SeatUses>().get(player()).is_none());
}

// --- Seat clip binder (AN26c) -------------------------------------------------------------------

/// The test chair's clip names under the naming contract (family `seat.chair`, band 45 cm), with a
/// second exit side for the latch.
const CHAIR_ENTER: &str = "seat.chair.b45.enter.front";
const CHAIR_LOOP: &str = "seat.chair.b45.loop";
const CHAIR_EXIT: &str = "seat.chair.b45.exit.front";
const CHAIR_EXIT_LEFT: &str = "seat.chair.b45.exit.left";

/// The rules with a slot table that has no `interact.sit.*` entries: only the chair's own clips.
fn aliased_rules() -> ActionRules {
    let slots = SlotTable::new()
        .with("loco.move", SlotInfo::looping())
        .with("air.fall", SlotInfo::looping())
        .with("air.jump", SlotInfo::once(0.3))
        .with("air.land.normal", SlotInfo::once(0.25))
        .with("air.land.hard", SlotInfo::once(0.5))
        .with(CHAIR_ENTER, SlotInfo::once(0.5))
        .with(CHAIR_LOOP, SlotInfo::looping())
        .with(CHAIR_EXIT, SlotInfo::once(0.4))
        .with(CHAIR_EXIT_LEFT, SlotInfo::once(0.4))
        .with_root(CHAIR_ENTER, Arc::new(SitDown))
        .with_root(CHAIR_EXIT, Arc::new(StandUp))
        .with_root(CHAIR_EXIT_LEFT, Arc::new(StandUp));
    let mut b = RuleSetBuilder::new();
    b.add_ron(&Origin::Framework, BASE_RULES).add_ron(&Origin::Framework, SEAT_RULES);
    ActionRules { rules: b.build().expect("base and seat rules"), slots: Box::new(slots) }
}

/// A side forced onto the user's `SeatClips.exit` after the offer (which follows `pick_exit`), for
/// the latch test.
#[derive(Resource, Default)]
struct ForceExit(Option<Side>);

fn force_exit(force: Res<ForceExit>, mut clips: Query<&mut SeatClips>) {
    if let Some(side) = force.0 {
        for mut c in &mut clips {
            if c.exit != side {
                c.exit = side;
            }
        }
    }
}

/// [`app`] with the aliased rules, the chair's clip sets and the exit forcer.
fn aliased_app() -> App {
    let mut app = app();
    app.insert_resource(aliased_rules())
        .insert_resource(SeatClipSets::from_names([CHAIR_ENTER, CHAIR_LOOP, CHAIR_EXIT, CHAIR_EXIT_LEFT], "seat.chair"))
        .init_resource::<ForceExit>()
        .add_systems(FixedUpdate, force_exit.after(SeatSystems::Offer).before(SeatClipSystems));
    app
}

fn alias_of(app: &App, e: Entity, rule: &str) -> Option<sf_action::SlotKey> {
    app.world().get::<sf_action_bevy::ActionAliases>(e).unwrap().0.get(sf_action::SlotKey::of(rule))
}

fn no_aliases(app: &App, e: Entity) -> bool {
    app.world().get::<sf_action_bevy::ActionAliases>(e).unwrap().0.is_empty()
}

fn playing(app: &App, e: Entity) -> Option<sf_action::SlotKey> {
    app.world().get::<ActionCharacter>(e).unwrap().output.slot
}

/// [`seated`] through the aliases: the enter only has a root path through them, so a sit that
/// starts proves the offer's tick bound them (a refused enter is not retried).
fn seated_aliased() -> (App, Entity) {
    let mut app = aliased_app();
    let p = spawn(&mut app, 1, Vec3::ZERO, player(), PLAYER, 30);
    frames(&mut app, 10);
    assert!(no_aliases(&app, p), "nothing bound before a use");
    app.world_mut().write_message(sit_command());
    frames(&mut app, 2);
    assert_eq!(claim(&app), Some((player(), Phase::Reserved)));
    assert_eq!(clips(&app, p), Some(chair_clips()));
    assert_eq!(alias_of(&app, p, ENTER), Some(sf_action::SlotKey::of(CHAIR_ENTER)), "bound on the offer's tick");
    assert!(until(&mut app, 120, |a| mov(a, p) == RuleKey::of(LOOP)), "sat: {:?}", mov(&app, p));
    frames(&mut app, 2);
    (app, p)
}

#[test]
fn an_aliased_sit_claims_then_the_body_sits_and_the_claim_is_occupied() {
    let (app, p) = seated_aliased();
    assert!(ever(&app, p, ENTER));
    assert_eq!(claim(&app), Some((player(), Phase::Occupied)));
    assert_eq!(kinds(&app), vec![(player(), EventKind::Claimed), (player(), EventKind::Entered)]);
    assert_eq!(seat_kinds(&app, p), vec![sf_action::EventKind::SeatEntered(0)]);
    assert_eq!(app.world().resource::<SeatUses>().get(player()).map(|u| u.phase), Some(SeatPhase::Seated));
    let feet = app.world().get::<ActionBody>(p).unwrap().loco.position.to_f64().as_vec3();
    assert!(feet.distance(Vec3::new(0.1, 0.0, 0.6)) < 1e-3, "{feet:?}");
    assert_eq!(alias_of(&app, p, ENTER), Some(sf_action::SlotKey::of(CHAIR_ENTER)));
    assert_eq!(alias_of(&app, p, LOOP), Some(sf_action::SlotKey::of(CHAIR_LOOP)));
    assert_eq!(alias_of(&app, p, EXIT), Some(sf_action::SlotKey::of(CHAIR_EXIT)));
}

#[test]
fn the_aliases_are_gone_the_tick_after_the_stand_up() {
    let (mut app, p) = seated_aliased();
    app.world_mut().resource_mut::<Presses>().0.push((p, ActionClass::Jump));
    assert!(
        until(&mut app, 120, |a| seat_kinds(a, p).contains(&sf_action::EventKind::SeatExited(0))),
        "stood: {:?}",
        mov(&app, p)
    );
    assert!(ever(&app, p, EXIT));
    frames(&mut app, 1);
    assert_eq!(clips(&app, p), None);
    assert!(no_aliases(&app, p), "{:?}", app.world().get::<sf_action_bevy::ActionAliases>(p));
    assert_eq!(claim(&app), None);
}

#[test]
fn the_exit_alias_is_latched_while_the_stand_up_plays() {
    let (mut app, p) = seated_aliased();
    let (front, left) = (Some(sf_action::SlotKey::of(CHAIR_EXIT)), Some(sf_action::SlotKey::of(CHAIR_EXIT_LEFT)));
    // Seated (the loop plays): a new exit side rebinds at once, and back.
    app.world_mut().resource_mut::<ForceExit>().0 = Some(Side::Left);
    frames(&mut app, 1);
    assert_eq!(alias_of(&app, p, EXIT), left);
    app.world_mut().resource_mut::<ForceExit>().0 = None;
    frames(&mut app, 1);
    assert_eq!(alias_of(&app, p, EXIT), front);
    // Stand up; once the exit plays, the side changes under it: the alias keeps the front exit.
    app.world_mut().resource_mut::<Presses>().0.push((p, ActionClass::Jump));
    let exit = sf_action::SlotKey::of(EXIT);
    assert!(until(&mut app, 60, |a| playing(a, p) == Some(exit)), "stand-up: {:?}", mov(&app, p));
    app.world_mut().resource_mut::<ForceExit>().0 = Some(Side::Left);
    let mut ticks = 0;
    loop {
        frames(&mut app, 1);
        if playing(&app, p) != Some(exit) {
            break;
        }
        assert_eq!(clips(&app, p).map(|c| c.exit), Some(Side::Left), "the side did change");
        assert_eq!(alias_of(&app, p, EXIT), front, "rebound mid-move");
        ticks += 1;
        assert!(ticks < 120, "the stand-up never ended");
    }
    assert!(ticks > 1, "the latch was exercised over several ticks: {ticks}");
    assert!(until(&mut app, 120, |a| mov(a, p) == RuleKey::of("loco")), "stood: {:?}", mov(&app, p));
    frames(&mut app, 2);
    assert!(no_aliases(&app, p));
}
