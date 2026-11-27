//! `IN31b2` headless tests: per-player focus, its verbs and the hold list (interaction.md "Point
//! index and focus"), and the command it gives passing `IN31b1` validation.

use sf_ids::{ActorKey, PlayerKey};
use sf_interaction::{ClipFamily, IkTargets, Occupancy, SlotDef, SlotKind, TagFilter, TagId, UserKey, VerbDef};
use sf_records::RecordId;
use sf_script_bevy::PLAYER;
use sf_script_bevy::sf_script::Handle;
use sf_tunables_bevy::TunablesPlugin;
use smallvec::{SmallVec, smallvec};

use super::*;

const BED: PlacementId = PlacementId(1);
const CHAIR: PlacementId = PlacementId(2);
/// Not indexed: no def (the pack's classes: talk, containers).
const DOOR: PlacementId = PlacementId(3);

const P1: UserKey = UserKey::Player(PlayerKey(1));
const P2: UserKey = UserKey::Player(PlayerKey(2));

fn sleep() -> TagId {
    TagId::of("sleep")
}

fn sit() -> TagId {
    TagId::of("sit")
}

fn examine() -> TagId {
    TagId::of("examine")
}

fn slot(verb: TagId, priority: i16) -> SlotDef {
    SlotDef {
        kind: SlotKind::Pose { family: TagId::of("seat") },
        local: Frame::default(),
        band: 0.45,
        verb: VerbDef { verb, priority, requires: TagFilter::default() },
        entrances: smallvec![],
        clips: ClipFamily("seat".into()),
        ik: IkTargets::default(),
        reach: 0.0,
        occupancy: Occupancy::Pose,
        control: None,
        users: TagFilter::default(),
        group: None,
        queue: None,
    }
}

fn def(id: &str, slots: Vec<SlotDef>, verbs: SmallVec<[VerbDef; 2]>) -> Arc<InteractableDef> {
    Arc::new(InteractableDef {
        id: RecordId::new(id),
        slots,
        verbs,
        material: None,
        destructible: None,
        carry: None,
        adverts: Vec::new(),
    })
}

/// `sleep` through slot 0 (priority 10), then slotless `examine` (priority 0).
fn bed() -> Arc<InteractableDef> {
    let examine = VerbDef { verb: examine(), priority: 0, requires: TagFilter::default() };
    def("bed", vec![slot(sleep(), 10)], smallvec![examine])
}

fn chair() -> Arc<InteractableDef> {
    def("chair", vec![slot(sit(), 0)], SmallVec::new())
}

#[derive(Resource, Default)]
struct Seen(Vec<Activated>);

fn gather(mut seen: ResMut<Seen>, mut activated: MessageReader<Activated>) {
    seen.0.extend(activated.read().copied());
}

/// 60 Hz; the bed and the chair indexed.
fn app() -> App {
    let mut app = App::new();
    app.add_plugins((TunablesPlugin, InteractionPlugin))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .init_resource::<Seen>()
        .add_systems(FixedUpdate, gather.after(InteractionSystems::Activate));
    app.update();
    let owner = app.world_mut().spawn_empty().id();
    for (placement, def) in [(BED, bed()), (CHAIR, chair())] {
        app.world_mut().write_message(IndexChange::Insert {
            placement,
            def,
            object: Frame::default(),
            owner,
            disabled: SmallVec::new(),
        });
    }
    app.world_mut().run_schedule(FixedUpdate);
    app
}

/// The default params at the test's 60 Hz.
fn params() -> InteractParams {
    InteractParams { tick_hz: 60.0, ..InteractParams::default() }
}

fn candidates(player: UserKey, candidates: &[(PlacementId, f32)], held: bool) -> FocusCandidates {
    FocusCandidates { player, candidates: candidates.to_vec(), held, priority: 30, tags: SmallVec::new() }
}

/// Sends one tick's candidates and runs the tick.
fn tick(app: &mut App, messages: Vec<FocusCandidates>) {
    for m in messages {
        app.world_mut().write_message(m);
    }
    app.world_mut().run_schedule(FixedUpdate);
}

fn state(app: &App, player: UserKey) -> FocusState {
    app.world().resource::<PlayerFocus>().get(player).cloned().unwrap_or_default()
}

fn verbs(app: &App, player: UserKey) -> Vec<TagId> {
    state(app, player).verbs().iter().map(|c| c.verb).collect()
}

/// Actor 1 claims and occupies the bed's seat.
fn take_bed(app: &mut App) {
    let terms = sf_interaction::SlotTerms::of(&bed().slots[0], &InteractParams::default());
    let seat = SlotRef { placement: BED, slot: 0 };
    for op in [Op::Claim { slot: seat, terms, tags: SmallVec::new() }, Op::Occupy { slot: seat }] {
        let user = UserKey::Actor(ActorKey(1));
        app.world_mut().write_message(ClaimRequest(Request { user, priority: 20, op }));
        app.world_mut().run_schedule(FixedUpdate);
    }
}

#[test]
fn focus_takes_the_best_at_once_then_a_challenger_needs_margin_and_dwell() {
    let mut app = app();
    let switch = params().focus_switch_ticks();
    assert_eq!(switch, 9, "0.15 s at 60 Hz");
    tick(&mut app, vec![candidates(P1, &[(CHAIR, 0.5), (BED, 1.0)], false)]);
    assert_eq!(state(&app, P1).target(), Some(BED));
    // A lead inside the margin (1.1 < 1.0 * 1.2) never switches.
    for _ in 0..30 {
        tick(&mut app, vec![candidates(P1, &[(CHAIR, 1.1), (BED, 1.0)], false)]);
        assert_eq!(state(&app, P1).target(), Some(BED));
    }
    // Past the margin, the challenger takes focus on its `switch`-th tick in the lead.
    for n in 1..=switch {
        tick(&mut app, vec![candidates(P1, &[(CHAIR, 1.5), (BED, 1.0)], false)]);
        let expect = if n < switch { BED } else { CHAIR };
        assert_eq!(state(&app, P1).target(), Some(expect), "tick {n}");
    }
    assert_eq!(verbs(&app, P1), vec![sit()]);
    // An empty list clears the focus.
    tick(&mut app, vec![candidates(P1, &[], false)]);
    assert_eq!(state(&app, P1).target(), None);
    assert!(state(&app, P1).verbs().is_empty());
}

#[test]
fn while_held_the_target_and_its_verbs_are_frozen_then_update_on_release() {
    let mut app = app();
    tick(&mut app, vec![candidates(P1, &[(BED, 1.0)], false)]);
    assert_eq!(verbs(&app, P1), vec![sleep(), examine()]);
    // Held: a much better target and another user taking the seat change nothing.
    take_bed(&mut app);
    for _ in 0..30 {
        tick(&mut app, vec![candidates(P1, &[(CHAIR, 5.0), (BED, 1.0)], true)]);
        assert_eq!(state(&app, P1).target(), Some(BED));
        assert_eq!(verbs(&app, P1), vec![sleep(), examine()]);
    }
    // Released: the verbs update at once (the seat is taken), the target after the dwell.
    tick(&mut app, vec![candidates(P1, &[(CHAIR, 5.0), (BED, 1.0)], false)]);
    assert_eq!(state(&app, P1).target(), Some(BED));
    assert_eq!(verbs(&app, P1), vec![examine()]);
    for _ in 1..params().focus_switch_ticks() {
        tick(&mut app, vec![candidates(P1, &[(CHAIR, 5.0), (BED, 1.0)], false)]);
    }
    assert_eq!(state(&app, P1).target(), Some(CHAIR));
    assert_eq!(verbs(&app, P1), vec![sit()]);
}

#[test]
fn the_list_opens_only_after_hold_s() {
    let mut app = app();
    let hold = params().focus_hold_ticks();
    assert_eq!(hold, 21, "0.35 s at 60 Hz");
    tick(&mut app, vec![candidates(P1, &[(BED, 1.0)], false)]);
    for n in 1..=hold {
        tick(&mut app, vec![candidates(P1, &[(BED, 1.0)], true)]);
        assert_eq!(state(&app, P1).list_open(), n >= hold, "tick {n}");
    }
    tick(&mut app, vec![candidates(P1, &[(BED, 1.0)], false)]);
    assert!(!state(&app, P1).list_open());
}

#[test]
fn two_players_focus_independently() {
    let mut app = app();
    tick(&mut app, vec![candidates(P2, &[(CHAIR, 1.0)], false), candidates(P1, &[(BED, 1.0)], false)]);
    assert_eq!(state(&app, P1).target(), Some(BED));
    assert_eq!(state(&app, P2).target(), Some(CHAIR));
    // P2 holds on the chair while a better bed shows; P1 swaps to the chair freely.
    for _ in 0..params().focus_switch_ticks() {
        tick(
            &mut app,
            vec![candidates(P1, &[(CHAIR, 5.0), (BED, 1.0)], false), candidates(P2, &[(BED, 5.0), (CHAIR, 1.0)], true)],
        );
    }
    assert_eq!(state(&app, P1).target(), Some(CHAIR));
    assert_eq!(state(&app, P2).target(), Some(CHAIR));
    // No message for P2 keeps it untouched; P1's empty list clears only P1.
    tick(&mut app, vec![candidates(P1, &[], false)]);
    assert_eq!(state(&app, P1).target(), None);
    assert_eq!(state(&app, P2).target(), Some(CHAIR));
    assert_eq!(verbs(&app, P2), vec![sit()]);
}

#[test]
fn a_target_without_a_def_offers_no_verbs_and_activates_its_primary_use() {
    let mut app = app();
    tick(&mut app, vec![candidates(P1, &[(DOOR, 1.0)], false)]);
    let s = state(&app, P1);
    assert_eq!(s.target(), Some(DOOR));
    assert!(s.verbs().is_empty());
    assert_eq!(s.command(P1, PLAYER, Some(0)), None);
    let cmd = s.command(P1, PLAYER, None).unwrap();
    assert_eq!((cmd.target, cmd.verb, cmd.slot), (DOOR, None, None));
    app.world_mut().write_message(cmd);
    app.world_mut().run_schedule(FixedUpdate);
    let a = Activated { target: Handle(DOOR.0), actor: PLAYER, verb: None };
    assert_eq!(app.world().resource::<Seen>().0, vec![a]);
}

#[test]
fn the_command_is_the_best_verb_or_the_hold_list_choice_and_passes_validation() {
    let mut app = app();
    tick(&mut app, vec![candidates(P1, &[(BED, 1.0)], false)]);
    let s = state(&app, P1);
    assert_eq!(state(&app, P2).command(P2, PLAYER, None), None, "no focus, no command");
    let best = s.command(P1, PLAYER, None).unwrap();
    assert_eq!((best.user, best.target, best.verb, best.slot, best.priority), (P1, BED, Some(sleep()), Some(0), 30));
    let listed = s.command(P1, PLAYER, Some(1)).unwrap();
    assert_eq!((listed.verb, listed.slot), (Some(examine()), None));
    assert_eq!(s.command(P1, PLAYER, Some(2)), None);
    app.world_mut().write_message(best);
    app.world_mut().write_message(listed);
    app.world_mut().run_schedule(FixedUpdate);
    let verbs: Vec<Option<u32>> = app.world().resource::<Seen>().0.iter().map(|a| a.verb).collect();
    let mut expect = vec![Some(sleep().0), Some(examine().0)];
    expect.sort_unstable();
    assert_eq!(verbs, expect);
}
