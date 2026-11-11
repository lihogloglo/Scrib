//! The player's combat glue (combat.md "C1b", combat-adapter.md "C1b: game items"): the
//! framework's `sf_combat_bevy` on the player body, with `mw_combat`'s sword sheet.
//!
//! - **Sheet.** [`mw_combat::SWORD_1H`] compiles at startup against the baked slot clips' lengths
//!   ([`crate::PlayerClips`]); its rules and slots join the player's `ActionRules`, its weapon
//!   socket tracks (joint `Weapon Bone`, else `Bip01 R Hand`) and the sheet go into
//!   `CombatData`. A slot without a baked clip is dropped by the sheet (attacks or rolls refuse);
//!   with no sheet at all the player simply has no combat.
//! - **Combatant.** Every player body gets a [`Combatant`] ([`mw_combat::fighter`]: health and
//!   stamina from the character sheet each tick, [`crate::vitals`]; the mover's capsule) and its
//!   moveset in `ActionState::mode.moveset`, kept set (rolls work sheathed).
//! - **Stance.** [`Stance`]: the ready-weapon key (F) draws and sheathes; while sheathed the
//!   attacks and the guard are withheld (`Combatant::withheld`): an attack press does nothing (owner
//!   2026-11-02: first the fighting stance, then the fight; Elden Ring has no sheathed state, but
//!   Morrowind and Skyrim attack only drawn). The avatar's `CharacterReadiness` follows it
//!   ([`show_stance`]): its held weapon (`mw_actors_bevy::held::EquippedHeld`, e.g. a level's
//!   `player_held`) mounts on `Weapon Bone` while drawn and hides while sheathed (the original
//!   shows no sheaths), and the graph's `stance` follows (`mw_actors_bevy` `feed_stance`).
//! - **Draw and sheathe clips.** F starts a [`Stance::Changing`] (`sf_character::ReadyChange`)
//!   timed by the baked clip ([`StanceClips`]: `base_anim`'s `WeaponOneHand` equip / unequip
//!   sections, [`mw_combat::SWORD_1H_DRAW`]): the readiness flips at its `Equip Attach` /
//!   `Unequip Detach` key, not at the press. Gameplay time: [`advance_stance`] runs on the fixed
//!   tick before the combat gate, so attacks stay withheld until a draw has played out and from
//!   the first tick of a sheathe. The avatar plays the clip in the graph's `upper` (masked) slot
//!   over locomotion ([`play_stance`], playhead = the change's elapsed time), so the legs keep
//!   walking. F during a change is ignored. No clip baked: the change is instant (the weapon
//!   pops, as before).
//! - **Lock-on.** `sf_combat_bevy::SfLockOnPlugin` (line of sight through the physics probe): the
//!   player body is a `LockOwner`, the lock key ([`spawn_lock`], `Control::LockOn`: Q, middle
//!   mouse, R3) locks the best `Lockable` (the training dummies) and the third-person camera frames
//!   it; `LockMarkerPlugin` draws the aim cursor on the target's chest (both views): the HUD
//!   crosshair is its template ([`tag_crosshair`]: same look, as themed) and hides meanwhile
//!   (`crate::hud`), so one cursor shows.
//! - **First person** ([`face_view`]): with the player camera in first person the commands carry
//!   `face_view`: attacks square up to the view and their `Turn` windows steer onto the soft-aim
//!   target (`SfLockOnPlugin`'s soft aim: nearest dummy within 3 m and +-30 deg of the view, picked
//!   in the attack's first 1/6 s), and the lock key does nothing (a held lock breaks). Third person
//!   unlocked soft-aims around the stick or facing.
//! - **Guard** (combat.md "C2b"): the guard key (right mouse, L1; `Control::Guard`, held) raises
//!   the sheet's `guard` move while drawn: hits from within 80 deg of the facing are guarded
//!   (damage x 0.35, stamina taken, a flinch instead of a reaction), a guarded hit that empties
//!   stamina breaks it (`guard.break`), regen runs at half rate. First person the body follows the
//!   view at the guard's turn rate (free yaw, attack pitch limits). HUD cue: not yet.
//! - **Rules.** [`ModernRules`]: `sf_combat::StandardRules` as a resource (the classic rules
//!   would replace it, combat-adapter.md "Rules").

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_log::prelude::*;
use bevy_time::prelude::*;
use bevy_time::Fixed;
use sf_action::{ActionClass, ClassSet};
use mw_game_core::player::PlayerAvatar;
use sf_action_bevy::{ActionCharacter, PlayerInput};
use sf_anim::RootMotionMode;
use sf_anim_bevy::Animator;
use sf_anim_graph::{MontageClip, SlotId, SlotRequest};
use sf_character::{Readiness, ReadyChange, ReadyClip};
use sf_character_bevy::{CharacterReadiness, CharacterSystems};
use sf_combat::{
    CombatRules, CompiledSheet, Contact, Fighter, Outcome, PoolDelta, RegenState, SocketTracks, StandardRules, Swing, SwingPlan,
};
use sf_combat_bevy::{
    CombatData, CombatSystems, Combatant, LockMarkerPlugin, LockMarkerTemplate, LockOwner, SfCombatPlugin, SfLockOnPlugin,
};
use sf_camera::ViewChoice;
use sf_camera_bevy::PlayerCamera;
use sf_physics_bevy::Physics;

use crate::avatar::PlayerFeed;
use crate::{PLAYER_CHARACTER, PlayerBody, PlayerClips};

/// A new body's health and fatigue until its sheet links ([`crate::vitals`]: the next tick's
/// refresh overwrites them); an app without stats (tests) keeps them.
pub const UNSHEETED_VITALS: [f32; 2] = [100.0, 100.0];

/// The classes a sheathed weapon withholds: the attacks and the guard (C2b: Morrowind and Skyrim
/// block only with a weapon or shield ready).
const ATTACKS: ClassSet = ClassSet::of(&[ActionClass::Light, ActionClass::Heavy, ActionClass::Guard]);

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

/// The player's compiled sheet (`None`: it didn't compile, the player has no combat). Health and
/// fatigue are the character sheet's ([`crate::vitals`], stats.md S5).
#[derive(Resource, Clone, Debug, Default)]
pub struct PlayerCombat {
    pub sheet: Option<CompiledSheet>,
}

/// Whether the weapon is drawn (the player body; module docs).
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub enum Stance {
    #[default]
    Sheathed,
    Drawn,
    /// Drawing (`to: Drawn`) or sheathing: the clip plays, attacks are withheld.
    Changing(ReadyChange),
}

impl Stance {
    /// The readiness the held weapon shows (its mount).
    #[must_use]
    pub fn shown(self) -> Readiness {
        match self {
            Self::Sheathed => Readiness::Sheathed,
            Self::Drawn => Readiness::Drawn,
            Self::Changing(c) => c.shown(),
        }
    }

    /// The stance at rest for a readiness.
    fn rest(readiness: Readiness) -> Self {
        if readiness == Readiness::Drawn { Self::Drawn } else { Self::Sheathed }
    }
}

/// The draw and sheathe clips' timing ([`mw_combat::SWORD_1H_DRAW`] and `_SHEATHE`, from the
/// baked slot clips' text-key events); [`ReadyClip::INSTANT`] for a clip that isn't baked.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct StanceClips {
    pub draw: ReadyClip,
    pub sheathe: ReadyClip,
}

impl Default for StanceClips {
    fn default() -> Self {
        Self { draw: ReadyClip::INSTANT, sheathe: ReadyClip::INSTANT }
    }
}

impl StanceClips {
    /// From `clips`: each clip's length and its attach (draw) or detach (sheathe) key. A clip
    /// without the key moves the weapon halfway through (logged).
    #[must_use]
    pub fn from_clips(clips: &PlayerClips) -> Self {
        let timing = |slot: &str, event: &str| {
            let Some(c) = clips.slots.iter().find(|c| c.slot == slot) else {
                info!("combat: no {slot} clip baked; the weapon pops in and out");
                return ReadyClip::INSTANT;
            };
            ReadyClip::from_clip(&c.clip, event).unwrap_or_else(|| {
                warn!("combat: the {slot} clip has no {event} key; the weapon moves halfway through it");
                ReadyClip::new(c.clip.duration * 0.5, c.clip.duration)
            })
        };
        let out = Self {
            draw: timing(mw_combat::SWORD_1H_DRAW, mw_anim::events::EQUIP_ATTACH),
            sheathe: timing(mw_combat::SWORD_1H_SHEATHE, mw_anim::events::UNEQUIP_DETACH),
        };
        info!(
            "combat: draw {:.2} s (attach at {:.2}), sheathe {:.2} s (detach at {:.2})",
            out.draw.length, out.draw.key, out.sheathe.length, out.sheathe.key
        );
        out
    }
}

/// The draw or sheathe clip the avatar plays (its `upper` slot and where it heads), on the body.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct StancePlay(Option<(SlotId, Readiness)>);

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
    if !app.is_plugin_added::<LockMarkerPlugin>() {
        app.add_plugins(LockMarkerPlugin);
    }
    app.init_resource::<ModernRules>();
    if let Some(sheet) = &sheet {
        let tracks = sockets(clips, sheet);
        let mut data = app.world_mut().get_resource_or_init::<CombatData>();
        data.sockets = tracks;
        data.movesets.insert(sheet.clone());
        info!("combat: sheet {} with {} slots, {} moves", sheet.id, sheet.slots.len(), sheet.moves.moves.len());
    }
    crate::vitals::add(app);
    app.insert_resource(PlayerCombat { sheet })
        .insert_resource(StanceClips::from_clips(clips))
        .add_systems(Update, ((arm, ready_weapon, show_stance).chain().after(crate::apply_mode).before(CharacterSystems), tag_crosshair))
        .add_systems(RunFixedMainLoop, face_view.in_set(sf_action_bevy::ActionSystems::Gather))
        .add_systems(FixedUpdate, advance_stance.before(CombatSystems::Gate))
        .add_systems(PostUpdate, play_stance.after(crate::avatar::feed::<Physics>).in_set(sf_anim_bevy::AnimSystems::Probe));
    #[cfg(feature = "debug")]
    app.add_systems(Update, toggle_gizmos);
}

/// Frame rate, before the fixed loop: the followed body's commands face the view while its camera
/// is in first person (`Command::face_view`: attacks square up to the view and soft-aim around
/// it, no hard lock-on; camera.md "First person"). The view choice reaches gameplay only as this
/// input bit, as the look does through `Command::view`.
fn face_view(cameras: Query<&PlayerCamera>, mut players: Query<&mut PlayerInput>) {
    for cam in &cameras {
        if let Ok(mut input) = players.get_mut(cam.follow) {
            input.sampler.set_face_view(cam.view() == ViewChoice::FirstPerson);
        }
    }
}

/// The HUD crosshair, once built, is the lock marker's template (the marker copies its look).
pub(crate) fn tag_crosshair(mut commands: Commands, huds: Query<&sf_ui_screens::hud::HudParts, Added<sf_ui_screens::hud::HudParts>>) {
    for parts in &huds {
        commands.entity(parts.crosshair()).insert(LockMarkerTemplate);
    }
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
        let [health, fatigue] = UNSHEETED_VITALS;
        let fighter = mw_combat::fighter(PLAYER_CHARACTER, sheet, health, fatigue, (p.radius, p.height), true);
        let mut combatant = Combatant::new(fighter);
        combatant.withheld = ATTACKS;
        commands.entity(entity).insert((combatant, Stance::Sheathed, StancePlay::default(), LockOwner::default()));
    }
}

/// The lock-on action on the player's `PlayerControls` context `body`, bound from `keymap`
/// (`Control::LockOn`; called with the other bindings, so a rebind respawns it too).
pub(crate) fn spawn_lock(commands: &mut Commands, keymap: &sf_ui::Keymap, body: Entity) {
    let lock = sf_combat_bevy::spawn_lock_action(commands, body);
    mw_controls_bevy::bind(commands, keymap, mw_controls_bevy::Control::LockOn, lock, |_, _| {});
}

/// Withholds the attacks unless drawn (so while drawing and sheathing too).
fn set_stance(stance: &mut Stance, combatant: &mut Combatant, to: Stance) {
    *stance = to;
    combatant.withheld = if to == Stance::Drawn { ClassSet::EMPTY } else { ATTACKS };
}

/// Starts a draw or sheathe of `clip` towards `to`; a clip of no length ends it at once.
fn start_change(stance: &mut Stance, combatant: &mut Combatant, to: Readiness, clip: ReadyClip) {
    let change = ReadyChange::start(to, clip);
    set_stance(stance, combatant, if change.done() { Stance::rest(to) } else { Stance::Changing(change) });
}

/// F draws a sheathed weapon and sheathes a drawn one (ignored while either plays).
#[allow(clippy::needless_pass_by_value)] // system parameters
fn ready_weapon(
    presses: Query<&ActionEvents, With<Action<ReadyWeaponAction>>>,
    clips: Res<StanceClips>,
    mut bodies: Query<(&mut Stance, &mut Combatant), With<PlayerBody>>,
) {
    if !presses.iter().any(|e| e.contains(ActionEvents::START)) {
        return;
    }
    for (mut stance, mut combatant) in &mut bodies {
        match *stance {
            Stance::Sheathed => start_change(&mut stance, &mut combatant, Readiness::Drawn, clips.draw),
            Stance::Drawn => start_change(&mut stance, &mut combatant, Readiness::Sheathed, clips.sheathe),
            Stance::Changing(_) => {}
        }
    }
}

/// Fixed tick, before the combat gate: a draw or sheathe advances on the gameplay clock and
/// ends in its stance (a finished draw lets attacks through on this very tick).
#[allow(clippy::needless_pass_by_value)] // system parameters
pub fn advance_stance(time: Res<Time>, mut bodies: Query<(&mut Stance, &mut Combatant), With<PlayerBody>>) {
    let dt = time.delta_secs();
    for (mut stance, mut combatant) in &mut bodies {
        let Stance::Changing(mut change) = *stance else { continue };
        let to = if change.advance(dt) { Stance::rest(change.to) } else { Stance::Changing(change) };
        set_stance(&mut stance, &mut combatant, to);
    }
}

/// The avatar's readiness follows its body's [`Stance`]: the held weapon mounts while drawn,
/// and from a draw's attach key to a sheathe's detach key.
fn show_stance(mut commands: Commands, avatars: Query<(Entity, &ChildOf, Option<&CharacterReadiness>), With<PlayerAvatar>>, bodies: Query<&Stance>) {
    for (avatar, parent, shown) in &avatars {
        let Ok(stance) = bodies.get(parent.parent()) else { continue };
        let readiness = CharacterReadiness(stance.shown());
        if shown != Some(&readiness) {
            commands.entity(avatar).insert(readiness);
        }
    }
}

/// Inertialization into the draw or sheathe clip and back out, seconds.
const STANCE_BLEND_IN: f32 = 0.12;
const STANCE_BLEND_OUT: f32 = 0.2;

/// Plays the draw or sheathe clip in the avatar's `upper` slot while the body's stance changes,
/// progress-driven by the change's elapsed time (extrapolated by the fixed overstep, clamped to
/// the clip), and stops it when the change ends.
#[allow(clippy::needless_pass_by_value)] // system parameters
pub fn play_stance(
    fixed: Res<Time<Fixed>>,
    mut bodies: Query<(&Stance, &PlayerFeed, &mut StancePlay)>,
    mut animators: Query<&mut Animator>,
) {
    let ahead = fixed.overstep_fraction().clamp(0.0, 1.0) * fixed.timestep().as_secs_f32();
    for (stance, feed, mut play) in &mut bodies {
        let Some(mut a) = feed.animator.and_then(|e| animators.get_mut(e).ok()) else { continue };
        let want = match *stance {
            Stance::Changing(c) => Some((c.to, (c.elapsed + ahead).min(c.clip.length))),
            _ => None,
        };
        let asset = a.asset.clone();
        match (want, play.0) {
            (Some((to, t)), Some((slot, playing))) if playing == to => a.instance.set_slot_progress(slot, t),
            (Some((to, t)), current) => {
                if let Some((slot, _)) = current {
                    a.instance.stop_slot(&asset.binding(), slot, STANCE_BLEND_OUT);
                }
                let name = if to == Readiness::Drawn { mw_combat::SWORD_1H_DRAW } else { mw_combat::SWORD_1H_SHEATHE };
                let graph = &asset.graph;
                let (Some(slot), Some(clip)) = (graph.slot(mw_graphs::build::slots::UPPER), graph.slot_clip(name)) else {
                    bevy_log::warn_once!("player avatar: the graph has no upper slot or no clip for {name}");
                    play.0 = None;
                    continue;
                };
                let req = SlotRequest {
                    root_motion: RootMotionMode::Progress,
                    blend_in: STANCE_BLEND_IN,
                    blend_out: STANCE_BLEND_OUT,
                    ..SlotRequest::new(MontageClip::Bound(clip))
                };
                play.0 = match a.instance.play_slot(&asset.binding(), slot, req) {
                    Ok(_) => {
                        a.instance.set_slot_progress(slot, t);
                        Some((slot, to))
                    }
                    Err(e) => {
                        warn!("player avatar: {name} did not start ({e})");
                        None
                    }
                };
            }
            (None, Some((slot, _))) => {
                a.instance.stop_slot(&asset.binding(), slot, STANCE_BLEND_OUT);
                play.0 = None;
            }
            (None, None) => {}
        }
    }
}
