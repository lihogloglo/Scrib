//! Scaffold `TrainingDummy` (combat-adapter.md "C1b: game items"; removed when C2's sparring
//! dummy lands): a standing target for the player's sword.
//!
//! - **Body.** A full action character (the player's mover and capsule, no input: it never moves
//!   or attacks on its own), so a hit's reaction is the same forced Clip move with hit-stop the
//!   player gets, played by the avatar slot player ([`crate::avatar`]).
//! - **Ground.** It spawns [`AwaitingGround`] (frozen) and is released onto the floor below once
//!   the collision around its feet is in; no floor found keeps it waiting (never through it).
//! - **Combat.** A hostile [`Combatant`] on the player's sheet, health [`DUMMY_HEALTH`]; at 0 it
//!   refills (the hit already played the largest reaction).
//! - **Look.** A dressed `NPC_` record ([`DUMMY_RECORD`]) tagged [`DummyAvatar`] (not
//!   `PlayerAvatar`: first person would hide it).
//! - **Spawn.** The level host calls [`spawn`] for its `dummy*` spawn points (`levels/arena.ron`).

use bevy_app::prelude::*;
use bevy_camera::prelude::Visibility;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use bevy_math::prelude::*;
use bevy_transform::prelude::*;
use sf_action::{ActionState, CharacterId, CharacterParams, RuleKey};
use sf_action_bevy::{ActionBody, ActionCharacter, AwaitingGround};
use sf_combat::Pool;
use sf_combat_bevy::{CombatSystems, Combatant, CombatantDied, HitLanded};
use sf_controller::{GroundProbe, Locomotion};
use sf_coords::WorldPos;
use sf_physics_bevy::Physics;
use sf_records::RecordId;
use sf_world_bevy::{ActiveWorldspace, from_grid, to_grid};

use crate::avatar::PlayerFeed;
use crate::combat::PlayerCombat;
use crate::{ARRIVAL_GROUND_MARGIN_M, GROUND_OFFSET_M, feel, ground_below};

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

/// The dummy systems. Called from [`crate::add`].
pub(crate) fn add(app: &mut App) {
    systems::<Physics>(app);
}

/// [`add`] with the ground probe `P` (tests use a static level).
pub(crate) fn systems<P: GroundProbe + Resource>(app: &mut App) {
    app.init_resource::<ActiveWorldspace>()
        .add_systems(Update, (arm, release::<P>).chain().after(crate::apply_mode))
        .add_systems(FixedUpdate, refill.in_set(CombatSystems::Apply));
}

/// Spawns dummy `index` (0-based) with its feet at `feet`, facing `yaw` (radians, 0 faces +Z), on
/// the worldspace grid `root`. Its combatant joins once the player's sheet is known ([`arm`]).
pub fn spawn(commands: &mut Commands, index: u32, (feet, yaw): (WorldPos, f32), root: Entity) -> Entity {
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
    commands.spawn((
        Name::new("training dummy avatar"),
        sf_character_bevy::Character::record(NPC_KIND, RecordId::new(DUMMY_RECORD)),
        DummyAvatar,
        mw_walkers_bevy::walkers::ActionDriven,
        Transform::IDENTITY,
        Visibility::Inherited,
        ChildOf(id),
    ));
    id
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

/// Logs each hit on a dummy (the live check reads them); a dummy at 0 health refills.
fn refill(
    mut hits: MessageReader<HitLanded>,
    mut deaths: MessageReader<CombatantDied>,
    mut dummies: Query<&mut Combatant, With<TrainingDummy>>,
) {
    for hit in hits.read() {
        if let Ok(combatant) = dummies.get(hit.defender) {
            let health = combatant.fighter.vitals.health;
            info!("training dummy: hit {} {:?} from the {:?}, health {:.0}/{:.0}", hit.hit, hit.tier, hit.side, health.cur, health.max);
        }
    }
    for died in deaths.read() {
        if let Ok(mut combatant) = dummies.get_mut(died.who) {
            let max = combatant.fighter.vitals.health.max;
            combatant.fighter.vitals.health = Pool::full(max);
            info!("training dummy: down, refilled to {max}");
        }
    }
}
