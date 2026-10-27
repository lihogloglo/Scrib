//! The player's combat glue (combat.md "C1b", combat-adapter.md "C1b: game items"): the
//! framework's `sf_combat_bevy` on the player body, with `mw_combat`'s sword sheet.
//!
//! - **Sheet.** [`mw_combat::SWORD_1H`] compiles at startup against the baked slot clips' lengths
//!   ([`crate::PlayerClips`]); its rules and slots join the player's `ActionRules`, its weapon
//!   socket tracks (joint `Weapon Bone`, else `Bip01 R Hand`) and the sheet go into
//!   `CombatData`. A slot without a baked clip is dropped by the sheet (attacks or rolls refuse);
//!   with no sheet at all the player simply has no combat.
//! - **Combatant.** Every player body gets a [`Combatant`] ([`mw_combat::fighter`]: health and
//!   stamina from the HUD's record vitals, the mover's capsule) and its moveset in
//!   `ActionState::mode.moveset`, kept set (rolls work sheathed).
//! - **Stance.** [`Stance`]: the ready-weapon key (F) draws and sheathes; while sheathed the
//!   attacks are withheld (`Combatant::withheld`), and an attack press draws instead (the press
//!   itself is refused that tick). Gameplay only: no weapon mesh is drawn yet.
//! - **Lock-on.** `sf_combat_bevy::SfLockOnPlugin` (line of sight through the physics probe): the
//!   player body is a `LockOwner`, the lock key ([`spawn_lock`], `Control::LockOn`: Q, middle
//!   mouse, R3) locks the best `Lockable` (the training dummies) and the third-person camera frames
//!   it.
//! - **Rules.** [`ModernRules`]: `sf_combat::StandardRules` as a resource (the classic rules
//!   would replace it, combat-adapter.md "Rules").

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_log::prelude::*;
use sf_action::{ActionClass, ClassSet};
use sf_action_bevy::{ActionCharacter, ActionSystems};
use sf_combat::{
    CombatRules, CompiledSheet, Contact, Fighter, Outcome, PoolDelta, RegenState, SocketTracks, StandardRules, Swing, SwingPlan,
};
use sf_combat_bevy::{CombatData, CombatSystems, Combatant, LockOwner, SfCombatPlugin, SfLockOnPlugin};
use sf_physics_bevy::Physics;

use crate::hud::PlayerVitals;
use crate::{PLAYER_CHARACTER, PlayerBody, PlayerClips};

/// The classes a sheathed weapon withholds.
const ATTACKS: ClassSet = ClassSet::of(&[ActionClass::Light, ActionClass::Heavy]);

/// The modern combat rules as a resource (`sf_combat::StandardRules`).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct ModernRules(pub StandardRules);

impl CombatRules for ModernRules {
    fn begin_swing(&self, swing: &Swing) -> SwingPlan {
        self.0.begin_swing(swing)
    }
    fn resolve_contact(&self, contact: &Contact, attacker: &Fighter, defender: &Fighter) -> Outcome {
        self.0.resolve_contact(contact, attacker, defender)
    }
    fn can_start(&self, who: &Fighter, class: ActionClass) -> bool {
        self.0.can_start(who, class)
    }
    fn cost(&self, who: &Fighter, base: f32, strength: f32) -> PoolDelta {
        self.0.cost(who, base, strength)
    }
    fn regen(&self, who: &Fighter, state: RegenState) -> PoolDelta {
        self.0.regen(who, state)
    }
}

/// The player's compiled sheet (`None`: it didn't compile, the player has no combat), and the
/// record's health and fatigue a new body starts with.
#[derive(Resource, Clone, Debug, Default)]
pub struct PlayerCombat {
    pub sheet: Option<CompiledSheet>,
    pub health: f32,
    pub fatigue: f32,
}

/// Whether the weapon is drawn (the player body; module docs).
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Stance {
    #[default]
    Sheathed,
    Drawn,
}

/// Ready / sheathe the weapon (on the player's `PlayerControls` context).
#[derive(InputAction)]
#[action_output(bool)]
pub struct ReadyWeaponAction;

/// Compiles the sword sheet against `clips`' lengths (warnings and errors logged).
pub fn load_sheet(clips: &PlayerClips) -> Option<CompiledSheet> {
    let len = |slot: &str| clips.slots.iter().find(|c| c.slot == slot).map(|c| c.clip.duration);
    match sf_combat::load(mw_combat::SWORD_1H, &len) {
        Ok(sheet) => {
            for w in &sheet.warnings {
                warn!("combat sheet {}: {w}", sheet.id);
            }
            Some(sheet)
        }
        Err(errors) => {
            for e in errors {
                warn!("combat sheet {} {:?}: {}; the player has no combat", e.sheet, e.mov, e.message);
            }
            None
        }
    }
}

/// The weapon socket tracks of `sheet`'s slot clips on the clips' skeleton (empty without one or
/// without a weapon joint: no hits land).
fn sockets(clips: &PlayerClips, sheet: &CompiledSheet) -> SocketTracks {
    let Some(skeleton) = clips.skeleton.as_deref() else { return SocketTracks::default() };
    let Some(bone) = mw_combat::WEAPON_BONES.iter().find_map(|b| skeleton.find(b)) else {
        warn!("combat: the humanoid skeleton has no weapon joint ({:?}); no hits land", mw_combat::WEAPON_BONES);
        return SocketTracks::default();
    };
    let slots = clips.slots.iter().filter(|c| sheet.slots.iter().any(|(s, _)| *s == c.slot));
    sf_combat_bevy::socket_tracks(skeleton, bone, slots.map(|c| (sf_action::SlotKey::of(&c.slot), &*c.clip)))
}

/// The combat plugin, data and the player's glue systems. Called from [`crate::add`].
pub(crate) fn add(app: &mut App, sheet: Option<CompiledSheet>, clips: &PlayerClips) {
    if !app.is_plugin_added::<SfCombatPlugin<ModernRules>>() {
        app.add_plugins(SfCombatPlugin::<ModernRules>::default());
    }
    if !app.is_plugin_added::<SfLockOnPlugin<Physics>>() {
        app.add_plugins(SfLockOnPlugin::<Physics>::default());
    }
    app.init_resource::<ModernRules>();
    if let Some(sheet) = &sheet {
        let tracks = sockets(clips, sheet);
        let mut data = app.world_mut().get_resource_or_init::<CombatData>();
        data.sockets = tracks;
        data.movesets.insert(sheet.clone());
        info!("combat: sheet {} with {} slots, {} moves", sheet.id, sheet.slots.len(), sheet.moves.moves.len());
    }
    // The HUD's record vitals, before it starts showing the combat ones.
    let record = app.world().get_resource::<PlayerVitals>().map_or(PlayerVitals::PLACEHOLDER, |v| [v.health.max, v.magicka.max, v.fatigue.max]);
    app.insert_resource(PlayerCombat { sheet, health: record[0], fatigue: record[2] })
        .add_systems(Update, (arm, ready_weapon).chain().after(crate::apply_mode))
        .add_systems(FixedUpdate, draw_on_attack.after(ActionSystems::Step).before(CombatSystems::Stats));
    #[cfg(feature = "debug")]
    app.add_systems(Update, toggle_gizmos);
}

/// Dev key for the combat gizmos (`sf_combat_bevy::CombatGizmos`; G is the physics colliders).
#[cfg(feature = "debug")]
pub const GIZMOS_KEY: bevy_input::keyboard::KeyCode = bevy_input::keyboard::KeyCode::KeyO;

/// [`GIZMOS_KEY`] toggles the combat gizmos.
#[cfg(feature = "debug")]
#[allow(clippy::needless_pass_by_value)] // system parameters
fn toggle_gizmos(keys: Option<Res<bevy_input::ButtonInput<bevy_input::keyboard::KeyCode>>>, gizmos: Option<ResMut<sf_combat_bevy::CombatGizmos>>) {
    let (Some(keys), Some(mut gizmos)) = (keys, gizmos) else { return };
    if keys.just_pressed(GIZMOS_KEY) {
        gizmos.enabled = !gizmos.enabled;
        info!("combat gizmos {} (O toggles)", if gizmos.enabled { "on" } else { "off" });
    }
}

/// A new player body gets its [`Combatant`] and [`Stance`]; the moveset stays set (a restored
/// action state may lack it).
#[allow(clippy::needless_pass_by_value)] // system parameters
fn arm(
    mut commands: Commands,
    combat: Res<PlayerCombat>,
    mut bodies: Query<(Entity, &mut ActionCharacter, Has<Combatant>), With<PlayerBody>>,
) {
    let Some(sheet) = &combat.sheet else { return };
    for (entity, mut character, armed) in &mut bodies {
        if character.state.mode.moveset != Some(sheet.key) {
            character.state.mode.moveset = Some(sheet.key);
        }
        if armed {
            continue;
        }
        let p = crate::feel::mover_params();
        let fighter = mw_combat::fighter(PLAYER_CHARACTER, sheet, combat.health, combat.fatigue, (p.radius, p.height), true);
        let mut combatant = Combatant::new(fighter);
        combatant.withheld = ATTACKS;
        commands.entity(entity).insert((combatant, Stance::Sheathed, LockOwner::default()));
    }
}

/// The lock-on action on the player's `PlayerControls` context `body`, bound from `keymap`
/// (`Control::LockOn`; called with the other bindings, so a rebind respawns it too).
pub(crate) fn spawn_lock(commands: &mut Commands, keymap: &sf_ui::Keymap, body: Entity) {
    let lock = sf_combat_bevy::spawn_lock_action(commands, body);
    mw_controls_bevy::bind(commands, keymap, mw_controls_bevy::Control::LockOn, lock, |_, _| {});
}

/// Withholds the attacks while sheathed.
fn set_stance(stance: &mut Stance, combatant: &mut Combatant, to: Stance) {
    *stance = to;
    combatant.withheld = if to == Stance::Drawn { ClassSet::EMPTY } else { ATTACKS };
}

/// F toggles the stance.
fn ready_weapon(
    presses: Query<&ActionEvents, With<Action<ReadyWeaponAction>>>,
    mut bodies: Query<(&mut Stance, &mut Combatant), With<PlayerBody>>,
) {
    if !presses.iter().any(|e| e.contains(ActionEvents::START)) {
        return;
    }
    for (mut stance, mut combatant) in &mut bodies {
        let to = if *stance == Stance::Drawn { Stance::Sheathed } else { Stance::Drawn };
        set_stance(&mut stance, &mut combatant, to);
    }
}

/// An attack press while sheathed draws (after the step, which refused it: no attack this tick).
fn draw_on_attack(mut bodies: Query<(&ActionCharacter, &mut Stance, &mut Combatant), With<PlayerBody>>) {
    for (character, mut stance, mut combatant) in &mut bodies {
        if *stance == Stance::Sheathed && character.command.pressed(ActionClass::Light) {
            set_stance(&mut stance, &mut combatant, Stance::Drawn);
        }
    }
}
