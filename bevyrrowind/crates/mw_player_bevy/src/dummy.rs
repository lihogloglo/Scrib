//! The arena's dummies (combat-adapter.md "C1b: game items", "Sparring dummy"): a standing target
//! for the player's sword (scaffold `TrainingDummy`, C1b) and a sparring partner that strikes back
//! ([`Sparring`], C2a). One body, two kinds ([`DummyKind`]): the sparring dummy is the training
//! dummy plus a strike schedule, so both share the body, ground, combat and refill rules below.
//!
//! - **Body.** A full action character (the player's mover and capsule, no input of its own), so
//!   a hit's reaction is the same forced Clip move with hit-stop the player gets, played by the
//!   avatar slot player ([`crate::avatar`]).
//! - **Ground.** It spawns [`AwaitingGround`] (frozen) and is released onto the floor below once
//!   the collision around its feet is in; no floor found keeps it waiting (never through it).
//! - **Combat.** A hostile [`Combatant`] on the player's sheet; its vitals are its dressed
//!   character's stat sheet ([`crate::vitals`]), health set to [`DUMMY_HEALTH`]; down, it
//!   refills (the hit already played the largest reaction). `Lockable`: the lock-on's target.
//! - **Sparring** ([`spar`], `ActionSystems::Command`): a sparring dummy writes its own `Command`
//!   each tick from `mw_combat::SPARRING` (`sf_combat::SparSchedule`): it looks at the nearest
//!   player body and presses Light on the schedule while the player stands in reach, so its
//!   light1 runs through the chooser, sweeps, contact order and rules like the player's (NPC
//!   combat, C6, drives characters through `Command` from `sf_ai` the same way). A sparring bout
//!   is non-lethal (KCD's practice bouts end, nobody dies): while a sparring dummy stands, a
//!   player body brought down refills like a dummy (scaffold until the player's death flow).
//! - **Look.** A dressed `NPC_` record ([`DUMMY_RECORD`]) tagged [`DummyAvatar`] (not
//!   `PlayerAvatar`: first person would hide it); a sparring dummy's avatar is also a
//!   [`SparringAvatar`], shown drawn (the level host gives it the level's `player_held` sword).
//! - **Spawn.** The level host calls [`spawn_kind`] for its `dummy*` and `sparring*` spawn points
//!   ([`DummyKind::of_spawn`]; `levels/arena.ron`, `levels/sparring.ron`).

use bevy_app::prelude::*;
use bevy_camera::prelude::Visibility;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use bevy_math::prelude::*;
use bevy_transform::prelude::*;
use sf_action::{ActionState, CharacterId, CharacterParams, RuleKey};
use sf_action_bevy::{ActionBody, ActionCharacter, ActionSystems, ActionTick, AwaitingGround};
use sf_character::Readiness;
use sf_character_bevy::CharacterReadiness;
use sf_combat::{Pool, SparSchedule};
use sf_combat_bevy::{CombatSystems, Combatant, CombatantDied, HitLanded};
use sf_controller::{GroundProbe, Locomotion};
use sf_coords::WorldPos;
use sf_physics_bevy::Physics;
use sf_records::RecordId;
use sf_stats_bevy::sf_stats::{CauseKind, Role, StatOp};
use sf_world_bevy::{ActiveWorldspace, from_grid, to_grid};

use crate::avatar::PlayerFeed;
use crate::combat::PlayerCombat;
use crate::vitals::{CombatSheet, SheetOps, VitalsSystems};
use crate::{ARRIVAL_GROUND_MARGIN_M, GROUND_OFFSET_M, PlayerBody, feel, ground_below};

/// The dummy's dressed record: a plain human NPC (Morrowind.esm).
pub const DUMMY_RECORD: &str = "fargoth";
/// The dummy's health; it refills at 0.
pub const DUMMY_HEALTH: f32 = 100.0;
/// The dummy's fatigue (stamina = fatigue x `mw_combat::STAMINA_PER_FATIGUE`; it never spends any).
const DUMMY_FATIGUE: f32 = 200.0;
/// The first dummy's character id (the player is 1).
const FIRST_DUMMY: u64 = 2;
/// The character kind of `NPC_` records (`mw_actors::source::NPC_KIND`).
const NPC_KIND: &str = "NPC_";

/// A training dummy's body (module docs).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TrainingDummy;

/// A training dummy's dressed character (the avatar link feeds its animator).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct DummyAvatar;

/// A sparring dummy's dressed character (shown with its weapon drawn).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SparringAvatar;

/// What a dummy does.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DummyKind {
    /// Stands and takes hits (C1b).
    #[default]
    Standing,
    /// Strikes the player on the sparring schedule (C2a).
    Sparring,
}

impl DummyKind {
    /// The kind a level spawn point names: `sparring*` spar, `dummy*` stand, anything else is
    /// no dummy (case-insensitive).
    #[must_use]
    pub fn of_spawn(name: &str) -> Option<Self> {
        let name = name.to_ascii_lowercase();
        if name.starts_with("sparring") {
            Some(Self::Sparring)
        } else if name.starts_with("dummy") {
            Some(Self::Standing)
        } else {
            None
        }
    }
}

/// A sparring dummy's schedule state: the tick it started sparring (its first armed, standing
/// tick).
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct Sparring {
    pub start: Option<sf_action::Tick>,
}

/// The sparring schedule (`mw_combat::SPARRING`); `None`: it didn't parse, sparring dummies stand.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct SparringRules(pub Option<SparSchedule>);

impl Default for SparringRules {
    fn default() -> Self {
        Self(mw_combat::sparring(mw_combat::SPARRING).inspect_err(|e| warn!("sparring dummy: {e}; it stands still")).ok())
    }
}

/// The dummy systems. Called from [`crate::add`].
pub(crate) fn add(app: &mut App) {
    systems::<Physics>(app);
}

/// [`add`] with the ground probe `P` (tests use a static level).
pub(crate) fn systems<P: GroundProbe + Resource>(app: &mut App) {
    app.init_resource::<ActiveWorldspace>()
        .init_resource::<SparringRules>()
        .add_systems(Update, (arm, release::<P>).chain().after(crate::apply_mode))
        .add_systems(FixedUpdate, spar.in_set(ActionSystems::Command))
        .add_systems(FixedUpdate, dummy_sheet.after(VitalsSystems::Link).before(CombatSystems::Stats))
        // After the sheet death hook (a sheet death is a `CombatantDied` too) and the vitals
        // differences (the refill must not be undone by this tick's damage going back).
        .add_systems(FixedUpdate, refill.in_set(CombatSystems::Apply).after(VitalsSystems::Deaths));
}

/// Spawns standing dummy `index` ([`spawn_kind`]).
pub fn spawn(commands: &mut Commands, index: u32, at: (WorldPos, f32), root: Entity) -> Entity {
    spawn_kind(commands, index, DummyKind::Standing, at, root)
}

/// Spawns dummy `index` (0-based, over both kinds) of `kind` with its feet at `feet`, facing
/// `yaw` (radians, 0 faces +Z), on the worldspace grid `root`. Its combatant joins once the
/// player's sheet is known ([`arm`]).
pub fn spawn_kind(commands: &mut Commands, index: u32, kind: DummyKind, (feet, yaw): (WorldPos, f32), root: Entity) -> Entity {
    let (cell, transform) = to_grid(feet);
    let params = feel::mover_params();
    let measures =
        CharacterParams { gravity: params.gravity, height: params.height, radius: params.radius, ..CharacterParams::default() };
    let character = ActionCharacter::new(CharacterId(FIRST_DUMMY + u64::from(index)), ActionState::new(RuleKey::of("loco")), measures);
    let body = ActionBody::new(Locomotion::new(feet, yaw), params, from_grid(&cell, &Transform::IDENTITY));
    let id = commands
        .spawn((
            Name::new(format!("training dummy {index}")),
            TrainingDummy,
            // The lock-on's target (`combat.rs`), aimed at the chest.
            sf_combat_bevy::lock::Lockable::default(),
            character,
            body,
            AwaitingGround,
            PlayerFeed::default(),
            cell,
            transform,
            Visibility::default(),
            ChildOf(root),
        ))
        .id();
    let avatar = commands
        .spawn((
            Name::new("training dummy avatar"),
            sf_character_bevy::Character::record(NPC_KIND, RecordId::new(DUMMY_RECORD)),
            DummyAvatar,
            mw_walkers_bevy::walkers::ActionDriven,
            Transform::IDENTITY,
            Visibility::Inherited,
            ChildOf(id),
        ))
        .id();
    if kind == DummyKind::Sparring {
        commands.entity(id).insert((Sparring::default(), Name::new(format!("sparring dummy {index}"))));
        commands.entity(avatar).insert((SparringAvatar, CharacterReadiness(Readiness::Drawn)));
    }
    id
}

/// Fixed tick, `ActionSystems::Command`: each armed, standing, living sparring dummy writes its
/// command from the schedule toward the nearest living player body (module docs, "Sparring").
#[allow(clippy::needless_pass_by_value, clippy::type_complexity)] // system parameters
fn spar(
    tick: Res<ActionTick>,
    rules: Res<SparringRules>,
    players: Query<(&ActionBody, &Combatant), (With<PlayerBody>, Without<TrainingDummy>)>,
    mut dummies: Query<(&mut ActionCharacter, &ActionBody, &Combatant, &mut Sparring), (With<TrainingDummy>, Without<AwaitingGround>)>,
) {
    let Some(schedule) = rules.0 else { return };
    for (mut character, body, combatant, mut sparring) in &mut dummies {
        if combatant.fighter.dead() {
            character.command = sf_action::Command::empty(tick.0);
            continue;
        }
        let start = *sparring.start.get_or_insert(tick.0);
        let feet = body.loco.position;
        let to = players
            .iter()
            .filter(|(_, c)| !c.fighter.dead())
            .map(|(b, _)| feet.delta_f32(b.loco.position))
            .min_by(|a, b| a.length_squared().total_cmp(&b.length_squared()));
        character.command = schedule.command(tick.0, tick.0.0.saturating_sub(start.0), to);
    }
}

/// A new dummy gets its [`Combatant`] and moveset once the player's sheet compiled.
#[allow(clippy::needless_pass_by_value, clippy::type_complexity)] // system parameters
fn arm(
    mut commands: Commands,
    combat: Option<Res<PlayerCombat>>,
    mut dummies: Query<(Entity, &mut ActionCharacter), (With<TrainingDummy>, Without<Combatant>)>,
) {
    let Some(sheet) = combat.as_ref().and_then(|c| c.sheet.as_ref()) else { return };
    for (entity, mut character) in &mut dummies {
        character.state.mode.moveset = Some(sheet.key);
        let p = feel::mover_params();
        let fighter = mw_combat::fighter(character.id, sheet, DUMMY_HEALTH, DUMMY_FATIGUE, (p.radius, p.height), true);
        commands.entity(entity).insert(Combatant::new(fighter));
    }
}

/// Releases a waiting dummy onto the floor below once the collision around its feet is in.
#[allow(clippy::needless_pass_by_value, clippy::type_complexity)] // system parameters
fn release<P: GroundProbe + Resource>(
    mut commands: Commands,
    collision: sf_physics_bevy::CollisionReady,
    active: Res<ActiveWorldspace>,
    probe: Res<P>,
    mut dummies: Query<(Entity, &mut ActionBody), (With<TrainingDummy>, With<AwaitingGround>)>,
) {
    for (entity, mut body) in &mut dummies {
        let feet = body.loco.position;
        if active.0.as_ref().is_some_and(|ws| collision.loading(ws, feet, ARRIVAL_GROUND_MARGIN_M)) {
            continue;
        }
        let Some(ground) = ground_below(&*probe, feet, &body.params) else { continue };
        let mut loco = body.loco.clone();
        loco.position = feet.offset(Vec3::Y * (ground + GROUND_OFFSET_M));
        loco.vertical_speed = 0.0;
        body.restore(loco);
        commands.entity(entity).remove::<AwaitingGround>();
    }
}

/// A dummy's new sheet (its dressed `NPC_` record's) gets [`DUMMY_HEALTH`], full, so the target
/// takes as many hits as before stats (the record's own health is a few hits).
fn dummy_sheet(dummies: Query<&CombatSheet, (With<TrainingDummy>, Added<CombatSheet>)>, mut sheets: SheetOps) {
    for link in &dummies {
        sheets.apply(link.owner, CauseKind::Rules, |s| {
            let Some(health) = s.role(Role::Health) else { return Vec::new() };
            vec![StatOp::SetBase { stat: health, value: DUMMY_HEALTH }, StatOp::PoolSetCurrent { pool: health, value: DUMMY_HEALTH }]
        });
    }
}

/// Logs each hit on a dummy, and each sparring hit on the player (the live check reads them); a
/// dummy down refills: its sheet's health (cause `Rules`) when it has one, and its combat health.
/// While a sparring dummy stands, so does a player body brought down (a bout is non-lethal).
#[allow(clippy::type_complexity)] // system parameters
fn refill(
    mut hits: MessageReader<HitLanded>,
    mut deaths: MessageReader<CombatantDied>,
    sparring: Query<(), With<Sparring>>,
    mut fighters: Query<(&mut Combatant, Option<&CombatSheet>, Has<TrainingDummy>), Or<(With<TrainingDummy>, With<PlayerBody>)>>,
    mut sheets: SheetOps,
) {
    for hit in hits.read() {
        if let Ok((combatant, _, dummy)) = fighters.get(hit.defender) {
            let health = combatant.fighter.vitals.health;
            let who = if dummy { "training dummy" } else { "sparring: the player" };
            info!("{who}: hit {} {:?} from the {:?}, health {:.0}/{:.0}", hit.hit, hit.tier, hit.side, health.cur, health.max);
        }
    }
    let spar = !sparring.is_empty();
    for died in deaths.read() {
        let Ok((mut combatant, link, dummy)) = fighters.get_mut(died.who) else { continue };
        if !dummy && !spar {
            continue;
        }
        let max = combatant.fighter.vitals.health.max;
        combatant.fighter.vitals.health = Pool::full(max);
        if let Some(link) = link {
            sheets.apply(link.owner, CauseKind::Rules, |s| {
                let Some(health) = s.role(Role::Health) else { return Vec::new() };
                vec![StatOp::PoolSetCurrent { pool: health, value: s.max(health) }]
            });
        }
        info!("{}: down, refilled to {max}", if dummy { "training dummy" } else { "sparring: the player" });
    }
}
