//! The player character in the streamed view (ROADMAP 2.1; superfargoth `docs/specs/action.md`
//! A2 slice 2, `camera.md` D1): `sf_action_bevy` moves it on the fixed tick (interpolated,
//! correction spring), `sf_camera_bevy` orbits the view camera around it (collision push-in
//! against the physics world, first/third-person toggle, zoom), and the focus ray picks what
//! E activates, with a "press to use" prompt.
//!
//! - **Modes.** F11 switches between the player and the free (fly) camera, which stays for dev
//!   work. Interactive runs start in player mode once the start has streamed in
//!   (`exterior::Settled`); `--bench` and `--screenshot` runs keep the free camera, so every
//!   existing scene and benchmark is unchanged. Turning the player on drops it in under the
//!   camera; turning it off despawns it and hands the camera back to `FreeCamera`.
//! - **Entities.** The body ([`PlayerBody`]: `ActionCharacter`, `ActionBody`, `PlayerInput`,
//!   the `PlayerControls` context with the bindings below) is a `big_space` child of the active
//!   worldspace grid; after the interpolation [`regrid`] keeps its `CellCoord` canonical and
//!   `frame_origin` on its cell. The view camera (the streaming camera, `doors::DoorUser`) gets
//!   a `PlayerCamera` following it. A capsule with a facing marker stands in for the body
//!   (scaffold `PlayerCapsule`: removal path, the dressed player character from
//!   `sf_character_bevy` with the A2 animation feed).
//! - **Doors.** The camera stays the traveler (`doors.rs`, scripts, sky and streaming key on
//!   it); on `TravelArrived` the body is teleported to the arrival marker and reparented to the
//!   new worldspace grid, the look turned to the marker's heading.
//! - **Activation.** [`Focus`] is `OpenMW`'s focus ray (`World::getFocusObject`): a camera ray
//!   through the screen centre, as long as `iMaxActivateDist` plus the camera's arm length, the
//!   arm subtracted from the hit distance; what has a name is activatable ([`activatable`]).
//!   `scripts.rs` writes `sf_script_bevy::Activated { target, actor: PLAYER }` for it on E;
//!   unhandled outcomes run the default action (`doors.rs` opens doors; dialogue consumes NPCs).
//! - **Seam.** [`PlayerMode::paused`] suspends the controls and frees the cursor (menus,
//!   dialogue). Movement and camera feel are modern (`sf_action` rules); a classic
//!   `mw_rules` controller would replace the rules and params, not this module.
//!
//! Bindings (keyboard and mouse; gamepad later with the keybind settings): WASD move, Shift
//! sprint, Ctrl sneak, Space jump (modern default; Morrowind used E), Tab first/third person (Morrowind's
//! default), mouse look, wheel zoom, E use (`scripts.rs`; Morrowind used Space).

use std::collections::HashMap;
use std::sync::Arc;

use bevy::camera_controller::free_camera::FreeCameraState;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use bevy_enhanced_input::prelude::*;
use sf_action::{ActionClass, ActionState, BASE_RULES, CharacterId, CharacterParams, RuleKey, RuleSet, SlotInfo, SlotTable};
use sf_action_bevy::{
    ActionBody, ActionCharacter, ActionInputPlugin, ActionRules, ActionSystems, PlayerAction, PlayerControls, PlayerId,
    PlayerInput, SfActionPlugin, spawn_player_actions,
};
use sf_camera::Look;
use sf_camera_bevy::{CameraAction, PlayerCamera, SfCameraPlugin, spawn_camera_actions};
use sf_controller::{Locomotion, MoverParams};
use sf_controller_bevy::AvoidObstacle;
use sf_coords::WorldPos;
use sf_physics_bevy::Physics;
use sf_records::{RecordDb, RecordId};
use sf_script::Handle as ScriptHandle;
use sf_script_bevy::{ScriptPlacements, ScriptSlot};
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_world_bevy::{FadeTransition, TravelArrived, WorldPosOf, from_grid, to_grid};

/// Switches between the player and the free camera.
pub const TOGGLE_KEY: KeyCode = KeyCode::F11;
/// Mouse look, radians per mouse count.
const LOOK_RAD_PER_COUNT: f32 = 0.0025;
/// `iMaxActivateDist` when the GMST is missing, game units (Morrowind.esm's value).
const DEFAULT_ACTIVATE_UNITS: f32 = 192.0;
/// A fall this far below the last ground puts the player back there, meters (a hole in the
/// streamed collision, not gameplay: Vvardenfell has no drop this deep).
const FALL_LIMIT_M: f32 = 200.0;
/// The player's character id on commands (NPCs don't run the action layer yet).
const PLAYER_CHARACTER: CharacterId = CharacterId(1);

pub fn add(app: &mut App, db: Option<Arc<RecordDb>>, start_on: bool) {
    if !app.is_plugin_added::<EnhancedInputPlugin>() {
        app.add_plugins(EnhancedInputPlugin);
    }
    let reach_m = activate_reach_m(db.as_deref());
    app.add_plugins((SfActionPlugin::<Physics>::default(), ActionInputPlugin, SfCameraPlugin::<Physics>::default()))
        .insert_resource(rules())
        .insert_resource(PlayerMode { on: start_on, paused: false, body: None, last_ground: None })
        .insert_resource(Activation { db, reach_m })
        .init_resource::<Focus>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (toggle, apply_mode, arrive, follow_worldspace, guard_fall, obstacle, controls_active, grab_cursor).chain(),
        )
        .add_systems(Update, (update_focus, prompt).chain().before(ScriptSlot::Update))
        .add_systems(RunFixedMainLoop, regrid.in_set(RunFixedMainLoopSystems::AfterFixedMainLoop).after(ActionSystems::Interpolate));
}

/// `iMaxActivateDist` in meters. `OpenMW` `apps/openmw/mwworld/worldimp.cpp`
/// `World::getMaxActivationDistance` (commit 4e718b50) reads the GMST as an integer, in game
/// units.
fn activate_reach_m(db: Option<&RecordDb>) -> f32 {
    let units = db.map_or(DEFAULT_ACTIVATE_UNITS, |db| mw_ai::gmst_float(db, "iMaxActivateDist", DEFAULT_ACTIVATE_UNITS));
    #[allow(clippy::cast_possible_truncation, reason = "a few meters")]
    let m = (f64::from(units) * mw_world::convert::UNIT_METERS_F64) as f32;
    m
}

/// The base move rules and the slot table the chooser times windows with. Scaffold
/// `PlayerSlots`: fixed clip lengths until B1's animation feed reports the real ones.
fn rules() -> ActionRules {
    let slots = SlotTable::new()
        .with("loco.move", SlotInfo::looping())
        .with("air.fall", SlotInfo::looping())
        .with("air.jump", SlotInfo::once(0.3))
        .with("air.land.normal", SlotInfo::once(0.25))
        .with("air.land.hard", SlotInfo::once(0.5));
    ActionRules { rules: RuleSet::from_ron(BASE_RULES).expect("framework base rules parse"), slots: Box::new(slots) }
}

/// Player or free camera.
#[derive(Resource, Debug)]
pub struct PlayerMode {
    /// The player is wanted (it spawns once the start has streamed in).
    pub on: bool,
    /// Controls suspended and the cursor free (menus, dialogue): the body stands.
    pub paused: bool,
    body: Option<Entity>,
    last_ground: Option<WorldPos>,
}

/// The player character's body.
#[derive(Component, Debug)]
pub struct PlayerBody;

/// Scaffold `PlayerCapsule`: the stand-in mesh (see the module docs).
#[derive(Resource)]
struct PlayerVisual {
    capsule: Handle<Mesh>,
    nose: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

/// The prompt's text node.
#[derive(Component)]
struct PromptText;

/// The centre dot (player mode).
#[derive(Component)]
struct Crosshair;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let p = MoverParams::default();
    commands.insert_resource(PlayerVisual {
        capsule: meshes.add(Capsule3d::new(p.radius, (p.height - 2.0 * p.radius).max(0.01))),
        nose: meshes.add(Cuboid::new(0.12, 0.12, 0.3)),
        material: materials.add(StandardMaterial { base_color: Color::srgb(0.75, 0.55, 0.3), perceptual_roughness: 0.8, ..default() }),
    });
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: percent(22),
            width: percent(100),
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_child((PromptText, Text::new(""), TextFont::from_font_size(20.0), TextColor(Color::srgb(0.95, 0.88, 0.7))));
    commands
        .spawn((
            Crosshair,
            Node {
                position_type: PositionType::Absolute,
                left: percent(50),
                top: percent(50),
                width: px(4),
                height: px(4),
                margin: UiRect::all(px(-2)),
                ..default()
            },
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
            Visibility::Hidden,
        ));
}

// --- bindings ---------------------------------------------------------------------------------

fn bind(commands: &mut Commands, action: Entity, binding: impl Into<Binding>, modifiers: impl Bundle) {
    commands.spawn((BindingOf(action), binding.into(), modifiers));
}

/// The player's actions on `body` (its `PlayerControls` context) with the default bindings.
fn spawn_controls(commands: &mut Commands, body: Entity) {
    for (tag, action) in spawn_player_actions(commands, body) {
        match tag {
            PlayerAction::Stick => {
                // `Cardinal` WASD: x right, y forward.
                bind(commands, action, KeyCode::KeyD, ());
                bind(commands, action, KeyCode::KeyA, Negate::all());
                bind(commands, action, KeyCode::KeyW, SwizzleAxis::YXZ);
                bind(commands, action, KeyCode::KeyS, (Negate::all(), SwizzleAxis::YXZ));
            }
            PlayerAction::Button(ActionClass::Jump) => bind(commands, action, KeyCode::Space, ()),
            PlayerAction::Button(ActionClass::Sprint) => bind(commands, action, KeyCode::ShiftLeft, ()),
            PlayerAction::Button(ActionClass::Sneak) => bind(commands, action, KeyCode::ControlLeft, ()),
            PlayerAction::Button(_) => {}
        }
    }
    for (tag, action) in spawn_camera_actions(commands, body) {
        match tag {
            CameraAction::Look => bind(commands, action, Binding::mouse_motion(), (Scale::splat(LOOK_RAD_PER_COUNT), Negate::y())),
            CameraAction::Zoom => bind(commands, action, Binding::mouse_wheel(), SwizzleAxis::YXZ),
            CameraAction::ToggleView => bind(commands, action, KeyCode::Tab, ()),
            CameraAction::SwapShoulder => {}
        }
    }
}

// --- mode ---------------------------------------------------------------------------------------

/// The player body bundle at `feet`, facing `yaw` (0 faces +Z), on the worldspace grid `root`.
fn spawn_body(commands: &mut Commands, visual: &PlayerVisual, feet: WorldPos, yaw: f32, root: Entity) -> Entity {
    let (cell, transform) = to_grid(feet);
    let params = MoverParams::default();
    let half = params.height / 2.0;
    let character = ActionCharacter::new(PLAYER_CHARACTER, ActionState::new(RuleKey::of("loco")), CharacterParams::default());
    let body = ActionBody::new(Locomotion::new(feet, yaw), params, from_grid(&cell, &Transform::IDENTITY));
    let player = PlayerId::new(0).expect("player 0 is valid");
    let id = commands
        .spawn((
            Name::new("player"),
            PlayerBody,
            character,
            body,
            PlayerControls,
            PlayerInput::new(player),
            AvoidObstacle { position: feet, velocity: Vec3::ZERO, radius: params.radius },
            cell,
            transform,
            Visibility::default(),
            ChildOf(root),
        ))
        .id();
    spawn_controls(commands, id);
    let material = MeshMaterial3d(visual.material.clone());
    commands.spawn((Mesh3d(visual.capsule.clone()), material.clone(), Transform::from_xyz(0.0, half, 0.0), ChildOf(id)));
    // Facing marker at head height (yaw 0 faces +Z).
    commands.spawn((Mesh3d(visual.nose.clone()), material, Transform::from_xyz(0.0, params.height - 0.2, params.radius), ChildOf(id)));
    id
}

fn toggle(keys: Res<ButtonInput<KeyCode>>, mut mode: ResMut<PlayerMode>) {
    if keys.just_pressed(TOGGLE_KEY) {
        mode.on = !mode.on;
        info!("player: {}", if mode.on { "on (F11: free camera)" } else { "off: free camera (F11: player)" });
    }
}

#[allow(clippy::needless_pass_by_value)] // system parameters
fn apply_mode(
    mut commands: Commands,
    mut mode: ResMut<PlayerMode>,
    settled: Option<Res<crate::exterior::Settled>>,
    visual: Res<PlayerVisual>,
    positions: WorldPosOf,
    mut cams: Query<(Entity, &GlobalTransform, &ChildOf, &mut FreeCameraState), With<crate::doors::DoorUser>>,
) {
    let Ok((cam, global, parent, mut free)) = cams.single_mut() else { return };
    match (mode.on, mode.body) {
        (true, None) => {
            let Some(here) = settled.and_then(|_| positions.location(cam)) else { return };
            let look = Look::from_dir(global.forward().as_vec3());
            // Feet at the eye: the body drops onto whatever is below.
            let body = spawn_body(&mut commands, &visual, here.pos, look.yaw, parent.parent());
            let mut camera = PlayerCamera::new(body);
            camera.set_look(look);
            commands.entity(cam).insert(camera);
            mode.body = Some(body);
            mode.last_ground = None;
            free.enabled = false;
            info!("player: spawned at {:.1?} in {}", here.pos.to_f64(), here.worldspace.as_str());
        }
        (true, Some(_)) => free.enabled = false,
        (false, Some(body)) => {
            commands.entity(body).despawn();
            commands.entity(cam).remove::<PlayerCamera>();
            mode.body = None;
            // Hand the camera back without a snap (as `doors::sync_free_camera`).
            let (yaw, pitch, _) = global.rotation().to_euler(EulerRot::YXZ);
            free.yaw = yaw;
            free.pitch = pitch;
            free.velocity = Vec3::ZERO;
            free.enabled = true;
        }
        (false, None) => {}
    }
}

/// A door moved the camera: the body follows to the arrival marker.
fn arrive(
    mut arrived: MessageReader<TravelArrived>,
    mut mode: ResMut<PlayerMode>,
    mut cams: Query<&mut PlayerCamera>,
    mut bodies: Query<&mut ActionBody, With<PlayerBody>>,
) {
    for travel in arrived.read() {
        let Ok(mut camera) = cams.get_mut(travel.traveler) else { continue };
        let Ok(mut body) = bodies.get_mut(camera.follow) else { continue };
        let feet = travel.to.pos.offset(Vec3::NEG_Y * crate::doors::ARRIVAL_EYE_M);
        let look = Look::from_dir(travel.rotation * Vec3::NEG_Z);
        body.teleport(feet, look.yaw);
        let (cell, _) = to_grid(feet);
        body.frame_origin = from_grid(&cell, &Transform::IDENTITY);
        camera.set_look(Look::new(look.yaw, 0.0));
        mode.last_ground = None;
    }
}

/// The body lives on the camera's worldspace grid (doors move the camera between grids).
fn follow_worldspace(
    mut commands: Commands,
    cams: Query<(&PlayerCamera, &ChildOf)>,
    bodies: Query<&ChildOf, With<PlayerBody>>,
) {
    for (camera, parent) in &cams {
        if let Ok(body_parent) = bodies.get(camera.follow)
            && body_parent.parent() != parent.parent()
        {
            commands.entity(camera.follow).insert(ChildOf(parent.parent()));
        }
    }
}

/// After the interpolation: the drawn root back into canonical grid coordinates.
fn regrid(mut bodies: Query<(&mut ActionBody, &mut CellCoord, &mut Transform), With<PlayerBody>>) {
    for (mut body, mut cell, mut transform) in &mut bodies {
        let world = body.frame_origin.offset(transform.translation);
        let (c, local) = to_grid(world);
        if *cell != c {
            *cell = c;
        }
        transform.translation = local.translation;
        body.frame_origin = from_grid(&c, &Transform::IDENTITY);
    }
}

/// A hole in the streamed collision drops the body: put it back on its last ground.
fn guard_fall(mut mode: ResMut<PlayerMode>, mut bodies: Query<&mut ActionBody, With<PlayerBody>>) {
    let Some(entity) = mode.body else { return };
    let Ok(mut body) = bodies.get_mut(entity) else { return };
    let here = body.loco.render_position();
    if body.loco.grounded.is_some() {
        mode.last_ground = Some(here);
        return;
    }
    let Some(ground) = mode.last_ground else { return };
    if ground.delta_f32(here).y < -FALL_LIMIT_M {
        warn!("player: fell {FALL_LIMIT_M} m (no collision below?); back to {:.1?}", ground.to_f64());
        let yaw = body.loco.motion.yaw;
        body.teleport(ground, yaw);
    }
}

/// NPCs steer around the player (ORCA obstacle, `walkers.rs`).
fn obstacle(mut bodies: Query<(&ActionBody, &mut AvoidObstacle), With<PlayerBody>>) {
    for (body, mut o) in &mut bodies {
        o.position = body.loco.render_position();
        o.velocity = body.loco.motion.velocity;
    }
}

/// [`PlayerMode::paused`] switches the player's input context off.
fn controls_active(mode: Res<PlayerMode>, mut commands: Commands, bodies: Query<Entity, With<PlayerBody>>) {
    if !mode.is_changed() {
        return;
    }
    for body in &bodies {
        commands.entity(body).insert(ContextActivity::<PlayerControls>::new(!mode.paused));
    }
}

/// The cursor is locked and hidden while the player is driven (mouse look).
fn grab_cursor(mode: Res<PlayerMode>, mut windows: Query<(&Window, &mut CursorOptions), With<PrimaryWindow>>, mut grabbed: Local<bool>) {
    let Ok((window, mut cursor)) = windows.single_mut() else { return };
    let want = mode.body.is_some() && !mode.paused && window.focused;
    if want != *grabbed {
        *grabbed = want;
        cursor.grab_mode = if want { CursorGrabMode::Locked } else { CursorGrabMode::None };
        cursor.visible = !want;
    }
}

// --- activation ---------------------------------------------------------------------------------

/// The activation rules' data: records and the reach.
#[derive(Resource)]
struct Activation {
    db: Option<Arc<RecordDb>>,
    /// `iMaxActivateDist`, meters.
    reach_m: f32,
}

/// What E activates now (`scripts.rs` writes `Activated` for it).
#[derive(Resource, Default, Debug)]
pub struct Focus {
    pub target: Option<FocusTarget>,
}

#[derive(Clone, Debug)]
pub struct FocusTarget {
    pub handle: ScriptHandle,
    /// Base record id.
    pub record: String,
    /// The record's display name.
    pub name: String,
    /// The prompt's verb (`None`: the name only, as Morrowind's tooltip).
    pub verb: Option<&'static str>,
    /// From the player's side (the camera arm subtracted), meters.
    pub distance: f32,
}

/// A record's display name and its prompt verb.
pub type Activatable = (String, Option<&'static str>);

/// Whether a record can be activated, with its name and the prompt's verb. `OpenMW` focuses what
/// has a tooltip (`MWClass::*::hasToolTip`: a non-empty name; statics have none,
/// `apps/openmw/mwclass/*.cpp`, commit 4e718b50); activation then is the class's
/// `activate`: NPCs talk, doors and containers open, books read, items are taken, carryable
/// lights too; a living creature or a fixed light shows its name only.
pub fn activatable(db: &RecordDb, record: &str) -> Option<Activatable> {
    use tes3::esp::{
        Activator, Alchemy, Apparatus, Armor, Book, Clothing, Container, Creature, Door, Ingredient, Light, LightFlags, Lockpick,
        MiscItem, Npc, Probe, RepairItem, Weapon,
    };
    let id = RecordId::new(record);
    let named = |name: &str, verb| (!name.is_empty()).then(|| (name.to_owned(), verb));
    macro_rules! classes {
        ($($ty:ty => $verb:expr),* $(,)?) => {
            $(if let Some(r) = db.get::<$ty>(&id) { return named(&r.name, $verb); })*
        };
    }
    classes! {
        Npc => Some("Talk"),
        Door => Some("Open"),
        Container => Some("Open"),
        Activator => Some("Use"),
        Book => Some("Read"),
        Creature => None,
        Weapon => Some("Take"),
        Armor => Some("Take"),
        Clothing => Some("Take"),
        Ingredient => Some("Take"),
        MiscItem => Some("Take"),
        Alchemy => Some("Take"),
        Apparatus => Some("Take"),
        Lockpick => Some("Take"),
        Probe => Some("Take"),
        RepairItem => Some("Take"),
    }
    if let Some(l) = db.get::<Light>(&id) {
        return named(&l.name, l.data.flags.contains(LightFlags::CAN_CARRY).then_some("Take"));
    }
    None
}

/// `OpenMW`'s focus ray (`World::getFocusObject`, `apps/openmw/mwworld/worldimp.cpp`, commit
/// 4e718b50): from the camera through the screen centre, `iMaxActivateDist` plus the camera
/// distance long; the hit distance minus the camera distance must be within `iMaxActivateDist`.
/// Terrain doesn't block (`OpenMW`'s default `terrain obstructs focus` is off); our pick is the
/// bounding-sphere scaffold `CrosshairPick` (`scripts.rs`), statics skipped.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)] // system parameters
fn update_focus(
    mut focus: ResMut<Focus>,
    activation: Res<Activation>,
    placements: Res<ScriptPlacements>,
    fade: Res<FadeTransition>,
    mode: Res<PlayerMode>,
    cams: Query<(Entity, &GlobalTransform, Option<&PlayerCamera>), With<crate::doors::DoorUser>>,
    positions: WorldPosOf,
    mut names: Local<HashMap<String, Option<Activatable>>>,
) {
    focus.target = None;
    let Some(db) = activation.db.as_deref() else { return };
    if fade.is_active() || mode.paused {
        return;
    }
    let Ok((cam, global, camera)) = cams.single() else { return };
    let Some(here) = positions.location(cam) else { return };
    let arm = camera.and_then(PlayerCamera::pose).map_or(0.0, |p| p.arm_length);
    for p in placements.loaded.values() {
        if !names.contains_key(&p.record) {
            names.insert(p.record.clone(), activatable(db, &p.record));
        }
    }
    let eye = here.pos.to_f64();
    let reach = activation.reach_m;
    let accept = |p: &sf_script_bevy::Placed| names.get(&p.record).is_some_and(Option::is_some);
    let picked = crate::scripts::pick(&placements, here.worldspace.as_str(), [eye.x, eye.y, eye.z], global.forward().as_vec3(), reach + arm, accept);
    let Some((handle, t)) = picked else { return };
    let distance = t - arm;
    let Some(placed) = placements.loaded.get(&handle) else { return };
    let Some(Some((name, verb))) = names.get(&placed.record) else { return };
    if distance <= reach {
        focus.target = Some(FocusTarget { handle, record: placed.record.clone(), name: name.clone(), verb: *verb, distance });
    }
}

/// The "press to use" prompt and the centre dot.
fn prompt(
    focus: Res<Focus>,
    mode: Res<PlayerMode>,
    mut texts: Query<&mut Text, With<PromptText>>,
    mut dots: Query<&mut Visibility, With<Crosshair>>,
) {
    if let Ok(mut text) = texts.single_mut() {
        let want = match &focus.target {
            Some(FocusTarget { name, verb: Some(verb), .. }) => format!("[E] {verb}  {name}"),
            Some(FocusTarget { name, verb: None, .. }) => name.clone(),
            None => String::new(),
        };
        if text.0 != want {
            text.0 = want;
        }
    }
    if let Ok(mut v) = dots.single_mut() {
        let want = if mode.body.is_some() && !mode.paused { Visibility::Inherited } else { Visibility::Hidden };
        v.set_if_neq(want);
    }
}

#[cfg(test)]
mod tests;
