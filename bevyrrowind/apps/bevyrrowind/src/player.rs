//! The player character in the streamed view (ROADMAP 2.1; superfargoth `docs/specs/action.md`
//! A2 slice 2, `camera.md` D1): `sf_action_bevy` moves it on the fixed tick (interpolated,
//! correction spring), `sf_camera_bevy` orbits the view camera around it (collision push-in
//! against the physics world, first/third-person toggle, zoom), and the focus ray picks what
//! E activates, with a "press to use" prompt.
//!
//! - **Modes.** F11 switches between the player and the free (fly) camera, which stays for dev
//!   work. Interactive runs start in player mode once the start has streamed in
//!   (`exterior::Settled`); `--bench` and `--screenshot` runs, and test scenes that place their
//!   own camera (`scenes::SceneDef::fixed_camera`), keep the free camera, so every existing
//!   scene and benchmark is unchanged. Turning the player on drops it in under the
//!   camera; turning it off despawns it and hands the camera back to `FreeCamera`. The first
//!   spawn of an interior start ([`PlayerStart`]) goes to a door's arrival marker into the
//!   cell, else to the floor under the camera ([`floor_below`]), not onto the roof.
//! - **Entities.** The body ([`PlayerBody`]: `ActionCharacter`, `ActionBody`, `PlayerInput`,
//!   the `PlayerControls` context with the bindings below) is a `big_space` child of the active
//!   worldspace grid; after the interpolation [`regrid`] keeps its `CellCoord` canonical and
//!   `frame_origin` on its cell. The view camera (the streaming camera, `doors::DoorUser`) gets
//!   a `PlayerCamera` following it. A capsule with a facing marker stands in for the body
//!   (scaffold `PlayerCapsule`: removal path, the dressed player character from
//!   `sf_character_bevy` with the A2 animation feed).
//! - **Doors.** The camera stays the traveler (`doors.rs`, scripts, sky and streaming key on
//!   it); on `TravelArrived` the body is teleported to the arrival marker and reparented to the
//!   new worldspace grid, the look turned to the marker's heading. Doorless link ends
//!   (teleport points) are picked from the body, not the camera behind it.
//! - **Activation.** [`Focus`] is `OpenMW`'s focus ray (`World::getFocusObject`): a camera ray
//!   through the screen centre, as long as `iMaxActivateDist` plus the camera's arm length, the
//!   arm subtracted from the hit distance; what has a name is activatable ([`activatable`]).
//!   The ray is cast on the physics world ([`focus_ray`]: statics, doors, items, the hit body
//!   mapped back to its placement) plus the actors' capsules ([`ActorShape`]); the nearest hit
//!   is the focus. `scripts.rs` writes `sf_script_bevy::Activated { target, actor: PLAYER }` for
//!   it on E, never while paused;
//!   unhandled outcomes run the default action (`doors.rs` opens doors; dialogue consumes NPCs).
//! - **Seam.** [`PlayerMode::paused`] suspends the controls and frees the cursor (menus,
//!   dialogue). Movement and camera feel are modern (`sf_action` rules); a classic
//!   `mw_rules` controller would replace the rules and params, not this module.
//!
//! Bindings come from the player's keymap (`controls.rs`: defaults, gamepad, hold / toggle,
//! profile, F10 keybinds screen); a keymap change respawns the body's bindings ([`rebind`]).
//! Keyboard defaults: WASD move, Shift sprint, Ctrl sneak, Space jump (modern default; Morrowind
//! used E), Tab first/third person (Morrowind's default), mouse look, wheel zoom, E use.

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
use sf_character_bevy::CharacterPlacement;
use sf_controller::{Locomotion, MoverParams, ProbeQuery};
use sf_controller_bevy::{AvoidObstacle, Mover};
use sf_coords::{WorldPos, WorldTransform};
use sf_physics::{BodyId, Layer, PhysicsWorld};
use sf_physics_bevy::{Physics, PhysicsStreaming};
use sf_records::{RecordDb, RecordId};
use sf_script::Handle as ScriptHandle;
use sf_script_bevy::{ScriptPlacements, ScriptSlot};
use sf_world::{PlacementId, WorldspaceId};
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_world_bevy::{FadeTransition, TravelArrived, WorldPosOf, from_grid, to_grid};

/// Switches between the player and the free camera.
pub const TOGGLE_KEY: KeyCode = KeyCode::F11;
/// Mouse look, radians per mouse count.
const LOOK_RAD_PER_COUNT: f32 = 0.0025;
/// Stick look at full deflection, radians per second (~150 deg/s, a common console default).
const STICK_LOOK_RAD_PER_S: f32 = 2.6;
/// `iMaxActivateDist` when the GMST is missing, game units (Morrowind.esm's value).
const DEFAULT_ACTIVATE_UNITS: f32 = 192.0;
/// A fall this far below the last ground puts the player back there, meters (a hole in the
/// streamed collision, not gameplay: Vvardenfell has no drop this deep).
const FALL_LIMIT_M: f32 = 200.0;
/// The player's character id on commands (NPCs don't run the action layer yet).
const PLAYER_CHARACTER: CharacterId = CharacterId(1);

pub fn add(app: &mut App, db: Option<Arc<RecordDb>>, start_on: bool, start: PlayerStart) {
    if !app.is_plugin_added::<EnhancedInputPlugin>() {
        app.add_plugins(EnhancedInputPlugin);
    }
    let reach_m = activate_reach_m(db.as_deref());
    crate::controls::add(app);
    app.add_plugins((SfActionPlugin::<Physics>::default(), ActionInputPlugin, SfCameraPlugin::<Physics>::default()))
        .insert_resource(rules())
        .insert_resource(PlayerMode { on: start_on, paused: false, body: None, last_ground: None })
        .init_resource::<RestoreBody>()
        .insert_resource(Activation { db, reach_m })
        .insert_resource(start)
        .init_resource::<Focus>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (toggle, apply_mode, rebind, arrive, follow_worldspace, guard_fall, obstacle, controls_active, grab_cursor).chain(),
        )
        .add_systems(Update, (update_focus.in_set(FocusUpdate), prompt).chain().before(ScriptSlot::Update))
        .add_systems(RunFixedMainLoop, regrid.in_set(RunFixedMainLoopSystems::AfterFixedMainLoop).after(ActionSystems::Interpolate));
}

/// Where [`Focus`] is updated: E's activation (`scripts.rs`) runs after it.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FocusUpdate;

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

/// A stick axis binding: a dead zone, and the vertical axis feeds Y.
fn stick_axis(binding: &Binding, entity: &mut EntityCommands<'_>) {
    entity.insert(DeadZone::default());
    if sf_ui_bevy::is_vertical_axis(binding) {
        entity.insert(SwizzleAxis::YXZ);
    }
}

/// No modifiers.
fn plain(_: &Binding, _: &mut EntityCommands<'_>) {}

/// The player's actions on `body` (its `PlayerControls` context), bound from `keymap`
/// (`controls.rs`).
fn spawn_controls(commands: &mut Commands, keymap: &sf_ui::Keymap, body: Entity) {
    use crate::controls::{Control, bind};
    for (tag, action) in spawn_player_actions(commands, body) {
        match tag {
            PlayerAction::Stick => {
                // `Cardinal` keys: x right, y forward; the pad stick with a dead zone.
                bind(commands, keymap, Control::MoveRight, action, plain);
                bind(commands, keymap, Control::MoveLeft, action, |_, e| {
                    e.insert(Negate::all());
                });
                bind(commands, keymap, Control::MoveForward, action, |_, e| {
                    e.insert(SwizzleAxis::YXZ);
                });
                bind(commands, keymap, Control::MoveBack, action, |_, e| {
                    e.insert((Negate::all(), SwizzleAxis::YXZ));
                });
                bind(commands, keymap, Control::Move, action, stick_axis);
            }
            PlayerAction::Button(ActionClass::Jump) => bind(commands, keymap, Control::Jump, action, plain),
            PlayerAction::Button(ActionClass::Sprint) => bind(commands, keymap, Control::Sprint, action, plain),
            PlayerAction::Button(ActionClass::Sneak) => bind(commands, keymap, Control::Sneak, action, plain),
            PlayerAction::Button(_) => {}
        }
    }
    for (tag, action) in spawn_camera_actions(commands, body) {
        match tag {
            // Mouse counts are a per-frame delta already; a stick is a rate (times delta time).
            CameraAction::Look => bind(commands, keymap, Control::Look, action, |b, e| {
                if matches!(b, Binding::MouseMotion { .. }) {
                    e.insert((Scale::splat(LOOK_RAD_PER_COUNT), Negate::y()));
                } else {
                    stick_axis(b, e);
                    e.insert((DeltaScale::default(), Scale::splat(STICK_LOOK_RAD_PER_S)));
                }
            }),
            CameraAction::Zoom => bind(commands, keymap, Control::Zoom, action, |_, e| {
                e.insert(SwizzleAxis::YXZ);
            }),
            CameraAction::ToggleView => bind(commands, keymap, Control::ToggleView, action, plain),
            CameraAction::SwapShoulder => {}
        }
    }
}

/// A keymap change (rebind, hold / toggle) respawns the body's actions with the new bindings.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn rebind(
    mut commands: Commands,
    keymap: Res<sf_ui_bevy::UiKeymap>,
    mode: Res<PlayerMode>,
    mut seen: Local<Option<u64>>,
    actions: Query<&Actions<PlayerControls>>,
) {
    let version = keymap.0.version();
    if seen.replace(version).is_none_or(|v| v == version) {
        return;
    }
    let Some(body) = mode.body else { return };
    let Ok(existing) = actions.get(body) else { return };
    for action in existing {
        commands.entity(action).despawn();
    }
    spawn_controls(&mut commands, &keymap.0, body);
}

// --- mode ---------------------------------------------------------------------------------------

/// The player body bundle at `feet`, facing `yaw` (0 faces +Z), on the worldspace grid `root`.
fn spawn_body(commands: &mut Commands, visual: &PlayerVisual, keymap: &sf_ui::Keymap, feet: WorldPos, yaw: f32, root: Entity) -> Entity {
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
    spawn_controls(commands, keymap, id);
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

/// Where an interior `view` start puts the player: the free camera frames the cell from above
/// (`main.rs` `interior_view`), so feet at its eye would land on the roof.
#[derive(Resource, Debug, Default)]
pub struct PlayerStart {
    /// The interior the run starts in; cleared once the player has spawned there.
    worldspace: Option<WorldspaceId>,
    /// Arrival markers of the doors into it.
    arrivals: Vec<WorldTransform>,
}

impl PlayerStart {
    /// The arrival markers into `start` when it is an interior (it has a cell environment);
    /// exterior starts keep the drop under the camera.
    pub fn of(world: &crate::doors::WorldLinks, start: &WorldspaceId) -> Self {
        if !world.environments.contains_key(start) {
            return Self::default();
        }
        let arrivals = world
            .links
            .iter()
            .flat_map(|l| [l.two_way.then_some(&l.a), Some(&l.b)])
            .flatten()
            .filter(|end| &end.worldspace == start)
            .map(|end| end.arrival)
            .collect();
        Self { worldspace: Some(start.clone()), arrivals }
    }
}

/// The first player spawn in an interior start: at the arrival marker nearest the camera (feet
/// on the marker, facing its heading, as door travel: `doors.rs`, [`arrive`]), else on the
/// floor under the camera ([`floor_below`]), else `None` (the eye).
fn interior_spawn(arrivals: &[WorldTransform], world: Option<&PhysicsWorld>, eye: WorldPos) -> Option<(WorldPos, Option<f32>)> {
    let nearest = arrivals.iter().min_by(|a, b| eye.delta_f32(a.pos).length().total_cmp(&eye.delta_f32(b.pos).length()));
    if let Some(marker) = nearest {
        let look = Look::from_dir(crate::doors::arrival_rotation(marker.rotation) * Vec3::NEG_Z);
        return Some((marker.pos, Some(look.yaw)));
    }
    let floor = floor_below(world?, eye, MoverParams::default().height)?;
    Some((floor.offset(Vec3::Y * 0.05), None))
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)] // system parameters
fn apply_mode(
    mut commands: Commands,
    mut mode: ResMut<PlayerMode>,
    mut start: ResMut<PlayerStart>,
    physics: Res<Physics>,
    streaming: Option<Res<PhysicsStreaming>>,
    settled: Option<Res<crate::exterior::Settled>>,
    visual: Res<PlayerVisual>,
    keymap: Res<sf_ui_bevy::UiKeymap>,
    positions: WorldPosOf,
    mut cams: Query<(Entity, &GlobalTransform, &ChildOf, &mut FreeCameraState), With<crate::doors::DoorUser>>,
) {
    let Ok((cam, global, parent, mut free)) = cams.single_mut() else { return };
    match (mode.on, mode.body) {
        (true, None) => {
            let Some(here) = settled.and_then(|_| positions.location(cam)) else { return };
            let mut look = Look::from_dir(global.forward().as_vec3());
            // Feet at the eye: the body drops onto whatever is below.
            let mut feet = here.pos;
            if start.worldspace.as_ref() == Some(&here.worldspace) {
                // Wait for the collision under the eye before looking for the floor.
                if streaming.as_deref().is_some_and(|s| s.collision_loading(&here.worldspace, here.pos, 2.0)) {
                    return;
                }
                start.worldspace = None;
                if let Some((at, yaw)) = interior_spawn(&start.arrivals, physics.world(&here.worldspace), here.pos) {
                    feet = at;
                    if let Some(yaw) = yaw {
                        look = Look::new(yaw, 0.0);
                    }
                } else {
                    warn!("player: no arrival marker or floor under the camera; spawning at the eye");
                }
            }
            let body = spawn_body(&mut commands, &visual, &keymap.0, feet, look.yaw, parent.parent());
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

/// A saved body (action and mover state) for the next arrival: a save load puts it back
/// after its travel (`saves.rs`).
#[derive(Resource, Default)]
pub struct RestoreBody(pub Option<(ActionState, Locomotion)>);

/// A door moved the camera: the body follows to the arrival marker (a load: the saved body).
fn arrive(
    mut arrived: MessageReader<TravelArrived>,
    mut mode: ResMut<PlayerMode>,
    mut cams: Query<&mut PlayerCamera>,
    mut bodies: Query<(&mut ActionBody, Option<&mut ActionCharacter>), With<PlayerBody>>,
    mut restore: ResMut<RestoreBody>,
) {
    for travel in arrived.read() {
        let Ok(mut camera) = cams.get_mut(travel.traveler) else { continue };
        let Ok((mut body, character)) = bodies.get_mut(camera.follow) else { continue };
        let feet = travel.to.pos.offset(Vec3::NEG_Y * crate::doors::ARRIVAL_EYE_M);
        let look = Look::from_dir(travel.rotation * Vec3::NEG_Z);
        match restore.0.take() {
            // The saved mover state is at the saved feet (`saves.rs` travels there).
            Some((state, loco)) => {
                body.restore(loco);
                if let Some(mut character) = character {
                    character.state = state;
                }
            }
            None => body.teleport(feet, look.yaw),
        }
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
/// The first thing the ray hits is the focus ([`focus_ray`]): a wall or a static in front
/// blocks, and what it hits shows a prompt only if it is [`activatable`]. Terrain doesn't
/// block (`OpenMW`'s default `terrain obstructs focus` is off).
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)] // system parameters
fn update_focus(
    mut focus: ResMut<Focus>,
    activation: Res<Activation>,
    placements: Res<ScriptPlacements>,
    fade: Res<FadeTransition>,
    mode: Res<PlayerMode>,
    physics: Res<Physics>,
    streaming: Option<Res<PhysicsStreaming>>,
    cams: Query<(Entity, &GlobalTransform, Option<&PlayerCamera>), With<crate::doors::DoorUser>>,
    actors: Query<(&Mover, &CharacterPlacement, &crate::walkers::ActorSpace), Without<PlayerBody>>,
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
    let reach = activation.reach_m;
    let ws = here.worldspace.as_str();
    let near = reach + arm + 2.0;
    let shapes: Vec<ActorShape> = actors
        .iter()
        .filter(|(m, _, space)| space.0 == here.worldspace && here.pos.delta_f32(m.loco.render_position()).length() < near)
        .map(|(m, p, _)| ActorShape {
            handle: ScriptHandle(p.0.0),
            feet: m.loco.render_position(),
            radius: m.params.radius,
            height: m.params.height,
        })
        .collect();
    // The hit body's placement (statics, doors, items: `PhysicsStreaming` keeps placement -> body).
    let owner = |body: BodyId| {
        let s = streaming.as_deref()?;
        placements
            .loaded
            .iter()
            .find(|(h, p)| p.worldspace == ws && s.body(&here.worldspace, PlacementId(h.0)) == Some(body))
            .map(|(h, _)| *h)
    };
    let world = physics.world(&here.worldspace);
    let picked = focus_ray(world, here.pos, global.forward().as_vec3(), reach + arm, owner, &shapes);
    let Some((handle, t)) = picked else { return };
    let distance = t - arm;
    let Some(placed) = placements.loaded.get(&handle) else { return };
    let cached = names.entry(placed.record.clone()).or_insert_with(|| activatable(db, &placed.record));
    let Some((name, verb)) = cached else { return };
    if distance <= reach {
        focus.target = Some(FocusTarget { handle, record: placed.record.clone(), name: name.clone(), verb: *verb, distance });
    }
}

/// What the focus ray hits in the physics world: placed statics, doors and items. Terrain is
/// left out (see [`update_focus`]), characters are tested as [`ActorShape`]s, and a held item
/// (`Layer::HELD`) doesn't hide what is behind it.
const FOCUS_LAYERS: Layer = Layer::STATIC.union(Layer::PROP).union(Layer::DEBRIS);

/// An actor as the focus ray sees it: an upright capsule at the mover's feet. Actors are tested
/// analytically, not through the physics world: a character's kinematic capsule exists only
/// while its mover simulates (`sf_physics_bevy` skips `Dormant` movers), a standing NPC out
/// of processing range must still be talked to, and the player's own capsule must not catch
/// the third-person ray.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ActorShape {
    pub handle: ScriptHandle,
    pub feet: WorldPos,
    pub radius: f32,
    pub height: f32,
}

/// The nearest thing a ray from `origin` along `dir` (normalized) hits within `max` meters:
/// a physics body of `world` ([`FOCUS_LAYERS`], mapped to its placement by `owner`; a body
/// with no placement blocks) or an actor. Returns the hit's handle and distance.
pub(crate) fn focus_ray(
    world: Option<&PhysicsWorld>,
    origin: WorldPos,
    dir: Vec3,
    max: f32,
    owner: impl Fn(BodyId) -> Option<ScriptHandle>,
    actors: &[ActorShape],
) -> Option<(ScriptHandle, f32)> {
    let mut best: Option<(Option<ScriptHandle>, f32)> = world
        .and_then(|w| w.probe(FOCUS_LAYERS).cast(&ProbeQuery::ray(origin, dir, max)))
        .map(|(body, hit)| (owner(body), hit.distance));
    for a in actors {
        let base = origin.delta_f32(a.feet);
        let r = a.radius.max(0.05);
        let lo = base + Vec3::Y * r;
        let hi = base + Vec3::Y * (a.height - r).max(r);
        if let Some(t) = ray_capsule(dir, lo, hi, r)
            && t <= max
            && best.is_none_or(|(_, bt)| t < bt)
        {
            best = Some((Some(a.handle), t));
        }
    }
    best.and_then(|(h, t)| Some((h?, t)))
}

/// Distance along `dir` (normalized, from the origin) to the capsule around the segment
/// `lo`-`hi` of radius `r`: the cylinder side and the end spheres, nearest first. `None` when
/// the ray misses or starts inside.
fn ray_capsule(dir: Vec3, lo: Vec3, hi: Vec3, r: f32) -> Option<f32> {
    let sphere = |center: Vec3| {
        let along = center.dot(dir);
        let disc = along * along - (center.length_squared() - r * r);
        (disc >= 0.0).then(|| along - disc.sqrt()).filter(|t| *t >= 0.0)
    };
    let mut best = sphere(lo).into_iter().chain(sphere(hi)).reduce(f32::min);
    let len = (hi - lo).length();
    if len > 1e-4 {
        // The infinite cylinder around the axis, in the plane perpendicular to it.
        let axis = (hi - lo) / len;
        let flat_dir = dir - axis * dir.dot(axis);
        let flat_origin = -lo + axis * lo.dot(axis);
        let qa = flat_dir.length_squared();
        let qb = flat_dir.dot(flat_origin);
        let disc = qb * qb - qa * (flat_origin.length_squared() - r * r);
        if qa > 1e-8 && disc >= 0.0 {
            let t = (-qb - disc.sqrt()) / qa;
            let along_axis = (dir * t - lo).dot(axis);
            if t >= 0.0 && (0.0..=len).contains(&along_axis) {
                best = Some(best.map_or(t, |s| s.min(t)));
            }
        }
    }
    best
}

/// How far down [`floor_below`] looks, meters.
const FLOOR_SEARCH_M: f32 = 200.0;
/// The highest ceiling over an interior floor, meters (Morrowind's halls stay well under it).
const CEILING_M: f32 = 60.0;

/// The floor under `eye` in an interior: down from the eye, the first surface with at least
/// `head_room` free above it and a ceiling within [`CEILING_M`]. A start above the room
/// (`main.rs` `interior_view` frames the cell from above) passes the roof's top (nothing
/// above it) and the roof's slab (no head room) on the way down.
pub(crate) fn floor_below(world: &PhysicsWorld, eye: WorldPos, head_room: f32) -> Option<WorldPos> {
    const STEP_M: f32 = 0.05;
    let probe = world.probe(Layer::STATIC);
    let mut from = eye;
    let mut fallen = 0.0;
    for _ in 0..64 {
        let (_, down) = probe.cast(&ProbeQuery::ray(from, Vec3::NEG_Y, FLOOR_SEARCH_M - fallen))?;
        if down.distance <= 1e-3 {
            // Inside a solid primitive: step through it.
            from = from.offset(Vec3::NEG_Y * (2.0 * STEP_M));
            fallen += 2.0 * STEP_M;
            continue;
        }
        let up = probe.cast(&ProbeQuery::ray(down.point.offset(Vec3::Y * STEP_M), Vec3::Y, CEILING_M));
        if up.is_some_and(|(_, h)| h.distance + STEP_M >= head_room) {
            return Some(down.point);
        }
        fallen += down.distance + STEP_M;
        from = down.point.offset(Vec3::NEG_Y * STEP_M);
    }
    None
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
