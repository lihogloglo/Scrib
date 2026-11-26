//! `IN2c2` / owner check ANIM-11 measurement (ignored): the player's Sit on the real seats of
//! "Seyda Neen, Census and Excise Office", replayed headless with the game's real rules
//! (`mw_player_bevy::rules`), baked clips (`exterior::player_clips`), seat clip sets and furniture
//! defs (`mw_interaction_bevy::load_defs`). Per seat and stance it prints the `SeatReport` steps the
//! use reaches, whether the body ran the sit rules, and the enter warp's correction ratio computed
//! from the clip the alias binds. It asserts only that every use ends in a terminal report.
#![allow(clippy::default_trait_access, reason = "empty tag and disabled lists: this crate has no smallvec dependency")]

use std::sync::Arc;
use std::time::Duration;

use bevy::app::{App, FixedUpdate, Last};
use bevy::ecs::prelude::*;
use bevy::time::{TimePlugin, TimeUpdateStrategy};
use glam::{Quat, Vec3};
use mw_interaction_bevy::input::PLAYER_USER;
use mw_player_bevy::PlayerClips;
use sf_action::{ActionState, CharacterId, CharacterParams, Command, RootPath, RuleKey, SlotKey};
use sf_action_bevy::{ActionBody, ActionCharacter, ActionSystems, ActionTick, SfActionPlugin};
use sf_anim::{RootPose, WarpPlan, WarpProfile};
use sf_controller::{GroundHit, GroundProbe, HeightSurface, Locomotion, ProbeQuery, TerrainProbe};
use sf_coords::{WorldPos, WorldTransform};
use sf_interaction_bevy::sf_interaction::{Frame, InteractableDef, Side, SlotDef, SlotKind, compose, pick_entrance, to_local};
use sf_interaction_bevy::{
    ActivateCommand, FocusCandidates, IndexChange, InteractionPlugin, InteractionSettings, InteractionUser, PlayerFocus,
    SeatClipSets, SeatReport, SeatStep, SeatSystems,
};
use sf_records::RecordId;
use sf_script_bevy::{PLAYER, ScriptsPlugin};
use sf_tunables_bevy::TunablesPlugin;
use sf_world::PlacementId;

const CELL: &str = "Seyda Neen, Census and Excise Office";
/// The seats the owner pressed E on (ANIM-11, 2026-11-26).
const SEATS: [&str; 3] = ["furn_com_rm_chair_03", "furn_com_r_chair_01", "furn_com_rm_bench_02"];
/// The placement id the test indexes the seat under (any id: one placement per app).
const PLACEMENT: PlacementId = PlacementId(9_001);
/// Fixed ticks a use runs after the Sit command.
const TICKS: usize = 180;
/// The seat fallback family (`mw_player_bevy` `SEAT_FALLBACK_FAMILY`, private there).
const SEAT_FALLBACK_FAMILY: &str = "seat.chair";
const ENTER: &str = "interact.sit.enter";
const LOOP: &str = "interact.sit.loop";

// --- Flat ground (copied from `sf_interaction_bevy/src/tests_seat.rs`) -------------------------

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

// --- Copies of private one-liners --------------------------------------------------------------

/// `mw_interaction_bevy/src/index.rs` `frame_of`.
fn frame_of(t: &WorldTransform) -> Frame {
    Frame { translation: t.pos.to_f64().as_vec3(), rotation: t.rotation }
}

/// `sf_interaction_bevy/src/seat.rs` `yaw_of`: facing of a world rotation (yaw 0 faces +Z).
fn yaw_of(rotation: Quat) -> f32 {
    let f = rotation * Vec3::Z;
    f.x.atan2(f.z)
}

/// `sf_interaction_bevy/src/seat.rs` `band_cm`.
fn band_cm(band: f32) -> u16 {
    let cm = (band * 100.0).round();
    if cm.is_nan() || cm <= 0.0 {
        0
    } else if cm >= f32::from(u16::MAX) {
        u16::MAX
    } else {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "in range and whole")]
        let v = cm as u16;
        v
    }
}

/// `mw_interaction_bevy/src/input.rs` `score`: a focus candidate's score from its distance.
fn score(distance: f32) -> f32 {
    1.0 / (1.0 + distance.max(0.0))
}

// --- Real data ---------------------------------------------------------------------------------

struct Seat {
    record: &'static str,
    def: Arc<InteractableDef>,
    /// The object frame, translated so the stance slot's root sits at the world origin (y = 0 is
    /// its floor, the flat ground).
    object: Frame,
    /// The slot the stances are measured from: the def's first pose slot.
    slot: u8,
    /// The reference as placed (world, m), its yaw and scale, for the print.
    placed: (Vec3, f32, f32),
}

struct Real {
    clips: PlayerClips,
    sets: SeatClipSets,
    seats: Vec<Seat>,
}

/// The install, bake, defs, clips and the three seats; `None` (with a message) when one is missing.
fn load() -> Option<Real> {
    let Ok((install, _)) = mw_install::MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let vfs = mw_vfs::mount_install(&install, sf_vfs::Vfs::builder()).ok()?.vfs;
    let content = mw_records::load(&vfs, &install.game_files()).ok()?;
    let Some(root) = mw_bake::default_cache_root() else {
        eprintln!("no bake cache root; skipping");
        return None;
    };
    let Ok(store) = sf_bake::Store::open(root) else {
        eprintln!("bake cache does not open; skipping");
        return None;
    };
    let Ok(manifest) = store.load_manifest(&mw_bake::profile_name(&install)) else {
        eprintln!("no bake manifest for this install (run `bevyrrowind bake`); skipping");
        return None;
    };
    let defs = mw_interaction_bevy::load_defs(&store, &manifest);
    let clips = player_clips(&store, &manifest);
    let clip_sets = SeatClipSets::from_names(clips.slots.iter().map(|c| c.slot.as_str()), SEAT_FALLBACK_FAMILY);
    let Some(cell) = mw_world::find_cell(&content.db, CELL) else {
        eprintln!("no cell {CELL:?}; skipping");
        return None;
    };
    let mut keys: Vec<(u32, u32)> = cell.references.keys().copied().collect();
    keys.sort_unstable();
    let mut seats = Vec::new();
    for record in SEATS {
        let Some(def) = defs.get(&RecordId::new(record)).cloned() else {
            eprintln!("no furniture def for {record} (bake older than the defs product?); skipping");
            return None;
        };
        let Some(r) = keys.iter().map(|k| &cell.references[k]).find(|r| r.id.eq_ignore_ascii_case(record)) else {
            eprintln!("no reference of {record} in {CELL:?}; skipping");
            return None;
        };
        let scale = r.scale.unwrap_or(1.0);
        let t = mw_world::convert::world_transform(r.translation, r.rotation, scale);
        let mut object = frame_of(&t);
        let Some(slot) = def.slots.iter().position(|s| matches!(s.kind, SlotKind::Pose { .. })) else {
            eprintln!("{record}: no pose slot in its def; skipping");
            return None;
        };
        let placed = (object.translation, yaw_of(object.rotation), scale);
        object.translation -= compose(&object, &def.slots[slot].local).translation;
        seats.push(Seat { record, def, object, slot: u8::try_from(slot).ok()?, placed });
    }
    Some(Real { clips, sets: clip_sets, seats })
}

/// A copy of `exterior.rs` `player_clips` (private to that module, so unreachable from here; logs
/// left out): the player's slots, then every baked seat clip, from the baked humanoid graph.
fn player_clips(store: &sf_bake::Store, manifest: &sf_bake::Manifest) -> PlayerClips {
    let library = mw_bake::anim::AnimLibrary::new(store.clone(), manifest.clone());
    let skeleton_name = mw_graphs::HUMANOID_SETS[0].skeleton;
    let Ok(graph) = library.graph(mw_graphs::Template::Humanoid.path(), skeleton_name) else {
        eprintln!("player clips: no humanoid graph baked");
        return PlayerClips::default();
    };
    let mut wanted = mw_player_bevy::player_slots();
    for (name, _) in &graph.slot_clips {
        if mw_player_bevy::is_seat_clip(name) && !wanted.contains(name) {
            wanted.push(name.clone());
        }
    }
    let slots = wanted
        .iter()
        .filter_map(|slot| {
            let product = &graph.clips.get(usize::from(graph.slot_clip(slot)?))?.product;
            let clip = library.clip(product).ok()?;
            Some(mw_player_bevy::SlotClip { slot: slot.clone(), clip: Arc::new(clip) })
        })
        .collect();
    let skeleton = library.skeleton(skeleton_name).ok().map(Arc::new);
    PlayerClips { slots, skeleton }
}

/// The slot clip named by `key`.
fn clip_name(clips: &PlayerClips, key: Option<SlotKey>) -> Option<&str> {
    let key = key?;
    clips.slots.iter().find(|c| SlotKey::of(&c.slot) == key).map(|c| c.slot.as_str())
}

fn v(p: Vec3) -> String {
    format!("({:.2}, {:.2}, {:.2})", p.x, p.y, p.z)
}

fn print_seat(seat: &Seat, real: &Real) {
    let (at, yaw, scale) = seat.placed;
    println!("seat {}: placed at {} yaw {yaw:.2} scale {scale:.2}; {} slots", seat.record, v(at), seat.def.slots.len());
    for (i, s) in seat.def.slots.iter().enumerate() {
        let entrances: Vec<String> = s.entrances.iter().map(|e| format!("{:?} {}", e.side, v(e.approach))).collect();
        println!(
            "  slot {i}: {:?} local {} yaw {:.2} band {:.2} family {} entrances [{}]",
            s.kind,
            v(s.local.translation),
            yaw_of(s.local.rotation),
            s.band,
            s.clips.0,
            entrances.join(", ")
        );
        if matches!(s.kind, SlotKind::Pose { .. }) {
            let found = real.sets.resolve(&s.clips.0, band_cm(s.band));
            let names = found.map(|set| {
                let side = s.entrances.first().map_or(Side::Front, |e| e.side);
                (clip_name(&real.clips, set.enter(side)), clip_name(&real.clips, set.seated()), clip_name(&real.clips, set.exit(side)))
            });
            println!("    resolve({}, {}) -> {names:?} (enter, loop, exit)", s.clips.0, band_cm(s.band));
        }
    }
}

// --- The app (after `sf_interaction_bevy/src/tests_seat.rs` `app`, `spawn`) ---------------------

/// What a run saw.
#[derive(Resource, Default)]
struct Log {
    movs: Vec<RuleKey>,
    reports: Vec<SeatReport>,
}

/// The tick's command: nothing pressed (the Sit is the activation pipeline's, not a button).
fn write_commands(tick: Res<ActionTick>, mut characters: Query<&mut ActionCharacter>) {
    for mut c in &mut characters {
        c.command = Command::empty(tick.0);
    }
}

fn record(mut log: ResMut<Log>, characters: Query<&ActionCharacter>) {
    for c in &characters {
        log.movs.push(c.state.mov);
    }
}

fn record_reports(mut log: ResMut<Log>, mut reports: MessageReader<SeatReport>) {
    log.reports.extend(reports.read().cloned());
}

fn app(real: &Real, seat: &Seat) -> App {
    let mut app = App::new();
    app.add_plugins((TimePlugin, TunablesPlugin, ScriptsPlugin, InteractionPlugin, SfActionPlugin::<Flat>::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
        .insert_resource(Flat(TerrainProbe(Plane)))
        .insert_resource(mw_player_bevy::rules(&real.clips.slots, None))
        .insert_resource(real.sets.clone())
        .init_resource::<Log>()
        .add_systems(FixedUpdate, write_commands.in_set(ActionSystems::Command))
        .add_systems(FixedUpdate, record.after(SeatSystems::Events))
        .add_systems(Last, record_reports);
    app.update();
    let owner = app.world_mut().spawn_empty().id();
    app.world_mut().write_message(IndexChange::Insert {
        placement: PLACEMENT,
        def: Arc::clone(&seat.def),
        object: seat.object,
        owner,
        disabled: Default::default(),
    });
    app
}

/// The player body at `feet` facing `yaw`, as `mw_player_bevy` `spawn_body` builds it, and the
/// interaction user `mw_interaction_bevy` `player_user` makes of it.
fn spawn(app: &mut App, feet: Vec3, yaw: f32, priority: u8) -> Entity {
    let params = mw_player_bevy::feel::mover_params();
    let measures =
        CharacterParams { gravity: params.gravity, height: params.height, radius: params.radius, ..CharacterParams::default() };
    let character = ActionCharacter::new(CharacterId(1), ActionState::new(RuleKey::of("loco")), measures);
    let body = ActionBody::new(Locomotion::new(WorldPos::from_f64(feet.as_dvec3()), yaw), params, WorldPos::ORIGIN);
    let user = InteractionUser { user: PLAYER_USER, actor: PLAYER, priority, tags: Default::default() };
    app.world_mut().spawn((character, body, user)).id()
}

/// One frame with the seat focused (what the player's focus ray writes every frame).
fn frame(app: &mut App, feet: Vec3, priority: u8) {
    let candidates = vec![(PLACEMENT, score(feet.length()))];
    app.world_mut().write_message(FocusCandidates { player: PLAYER_USER, candidates, held: false, priority, tags: Default::default() });
    app.update();
}

// --- Stances -----------------------------------------------------------------------------------

/// Yaw that faces from `from` to the world origin (the stance slot's root).
fn facing_seat(from: Vec3) -> f32 {
    (-from.x).atan2(-from.z)
}

/// The stances (name, feet, yaw), relative to the stance slot at the origin.
fn stances(seat: &Seat) -> Vec<(&'static str, Vec3, f32)> {
    let def: &SlotDef = &seat.def.slots[usize::from(seat.slot)];
    let world = compose(&seat.object, &def.local);
    let flat = |d: Vec3| Vec3::new(d.x, 0.0, d.z).normalize_or_zero();
    let forward = flat(world.rotation * Vec3::Z);
    let left = flat(world.rotation * Vec3::X);
    let slot_yaw = yaw_of(world.rotation);
    let mut out = Vec::new();
    for (name, d) in [("A 0.9 front", 0.9), ("B 1.2 front", 1.2), ("C 1.5 front", 1.5)] {
        out.push((name, forward * d, facing_seat(forward * d)));
    }
    out.push(("D 1.2 left", left * 1.2, facing_seat(left * 1.2)));
    let from_b = forward * 1.2;
    if let Some(e) = pick_entrance(def, to_local(&seat.object, from_b), |_| true) {
        let mut at = compose(&seat.object, &Frame { translation: e.approach, rotation: Quat::IDENTITY }).translation;
        at.y = 0.0;
        out.push(("E approach, facing seat", at, facing_seat(at)));
        out.push(("F approach, slot yaw", at, slot_yaw));
    }
    out.push(("G on slot, slot yaw", Vec3::ZERO, slot_yaw));
    out
}

/// Whether `step` ends a use attempt (anything but the steps on the way).
fn terminal(step: &SeatStep) -> bool {
    !matches!(step, SeatStep::Asked | SeatStep::Held | SeatStep::Offered { .. })
}

/// `interact.sit.enter`'s `refuse_ratio` from the framework's seat rules text.
fn rule_refuse_ratio() -> Option<f32> {
    let rules = sf_action::SEAT_RULES;
    let rest = &rules[rules.find(&format!("id: \"{ENTER}\""))?..];
    let tail = &rest[rest.find("refuse_ratio:")? + "refuse_ratio:".len()..];
    tail.trim_start().split(|c: char| !(c.is_ascii_digit() || c == '.')).next()?.parse().ok()
}

/// The enter warp as `sf_action` `plan_slot` plans it (feet at their yaw onto the sensed enter
/// pose), from the clip the seat's clip set binds for `side`.
fn warp_line(real: &Real, seat: &Seat, slot: u8, side: Side, (feet, yaw): (Vec3, f32)) -> String {
    let def = &seat.def.slots[usize::from(slot)];
    let world = compose(&seat.object, &def.local);
    let key = real.sets.resolve(&def.clips.0, band_cm(def.band)).and_then(|s| s.enter(side));
    let Some(clip) = key.and_then(|k| real.clips.slots.iter().find(|c| SlotKey::of(&c.slot) == k)) else {
        return format!("no enter clip for {} b{} {side:?}", def.clips.0, band_cm(def.band));
    };
    let path: &dyn RootPath = &*clip.clip;
    let end = path.root_at(path.duration());
    let start = RootPose::new(Vec3::ZERO, yaw);
    let target = RootPose::new(world.translation - feet, yaw_of(world.rotation));
    let plan = WarpPlan::new(path, start, target, WarpProfile::Linear);
    format!(
        "clip {} travel {:.2} m end yaw {:.2} | corr {:.2} m yaw corr {:.2} ratio {:.2} (refuse {:?})",
        clip.slot,
        end.position.length(),
        end.yaw,
        plan.correction.length(),
        plan.yaw_correction,
        plan.ratio(),
        rule_refuse_ratio()
    )
}

/// One stance's use: its line, and whether it ended in a terminal report.
fn run(real: &Real, seat: &Seat, (name, feet, yaw): (&str, Vec3, f32)) -> (String, bool) {
    let mut app = app(real, seat);
    let priority = app.world().resource::<InteractionSettings>().0.priority_player;
    let e = spawn(&mut app, feet, yaw, priority);
    let mut command: Option<ActivateCommand> = None;
    for _ in 0..40 {
        frame(&mut app, feet, priority);
        let focused = app.world().resource::<PlayerFocus>().get(PLAYER_USER).and_then(|s| s.command(PLAYER_USER, PLAYER, None));
        if let Some(c) = focused.filter(|c| c.verb.is_some()) {
            command = Some(c);
            break;
        }
    }
    let Some(command) = command else {
        return (format!("{} {name}: the focus offers no verb (no Sit prompt)", seat.record), false);
    };
    let body = app.world().get::<ActionBody>(e).expect("the body");
    let settled = (body.loco.position.to_f64().as_vec3(), yaw);
    app.world_mut().resource_mut::<Log>().reports.clear();
    let ticks0 = app.world().resource::<Log>().movs.len();
    app.world_mut().write_message(command.clone());
    while app.world().resource::<Log>().movs.len() < ticks0 + TICKS {
        frame(&mut app, feet, priority);
    }
    let log = app.world().resource::<Log>();
    let steps: Vec<&SeatStep> = log.reports.iter().filter(|r| r.user == PLAYER_USER && r.entity == e).map(|r| &r.step).collect();
    let last = steps.iter().rev().find(|s| terminal(s)).copied();
    let movs = &log.movs[ticks0..];
    let ran = |rule: &str| movs.iter().any(|k| *k == RuleKey::of(rule));
    let side = steps.iter().find_map(|s| match s {
        SeatStep::Offered { entry, .. } => Some(entry.unwrap_or(Side::Front)),
        _ => None,
    });
    let slot = command.slot.unwrap_or(seat.slot);
    let warp = warp_line(real, seat, slot, side.unwrap_or(Side::Front), settled);
    let line = format!(
        "{} {name}: feet {} yaw {yaw:.2} | verb {:?} slot {slot} | steps {steps:.2?} | terminal {last:.2?} | ran enter {} loop {} | {warp}",
        seat.record,
        v(settled.0),
        command.verb,
        ran(ENTER),
        ran(LOOP),
    );
    (line, last.is_some())
}

#[test]
#[ignore = "measurement: needs a Morrowind install and the game's bake; which step a player's Sit reaches per stance"]
fn sit_repro_real_seats_per_stance() {
    let Some(real) = load() else { return };
    let seat_clips: Vec<&str> = real.clips.slots.iter().map(|c| c.slot.as_str()).filter(|s| mw_player_bevy::is_seat_clip(s)).collect();
    println!("baked seat clips ({}): {seat_clips:?}", seat_clips.len());
    println!("interact.sit.enter refuse_ratio: {:?}", rule_refuse_ratio());
    for seat in &real.seats {
        print_seat(seat, &real);
    }
    let mut silent = Vec::new();
    for seat in &real.seats {
        for stance in stances(seat) {
            let (line, ended) = run(&real, seat, stance);
            println!("{line}");
            if !ended {
                silent.push(format!("{} {}", seat.record, stance.0));
            }
        }
    }
    assert!(silent.is_empty(), "uses that ended in no terminal seat report: {silent:?}");
}
