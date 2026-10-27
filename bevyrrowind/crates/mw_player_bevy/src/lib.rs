//! The player character in the streamed view (ROADMAP 2.1; superfargoth `docs/specs/action.md`
//! A2 slice 2, `camera.md` D1): `sf_action_bevy` moves it on the fixed tick (interpolated,
//! correction spring), `sf_camera_bevy` orbits the view camera around it (collision push-in
//! against the physics world, first/third-person toggle, zoom), and the focus ray picks what
//! E activates, with the HUD's "press to use" prompt ([`hud`]: bars, compass, crosshair).
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
//!   a `PlayerCamera` following it. The capsule is the invisible gameplay shape; the visible
//!   body is the dressed `player` record playing the humanoid graph ([`avatar`]: locomotion
//!   feed, traversal slot, first-person hide). Without characters a capsule mesh is drawn.
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
//! Keyboard defaults: WASD move, Shift dodge / sprint (tap rolls, hold sprints), Ctrl sneak, Space
//! jump (modern default; Morrowind used E), Tab first/third person (Morrowind's default), mouse
//! look, wheel zoom, E use, left mouse attack, F ready weapon ([`combat`]).

pub mod avatar;
pub mod combat;
pub mod dummy;
pub mod feel;
pub mod hud;

use std::collections::HashMap;
use std::sync::Arc;

use bevy_app::prelude::*;
use bevy_asset::prelude::*;
use bevy_camera::prelude::*;
use bevy_camera_controller::free_camera::FreeCameraState;
use bevy_color::prelude::*;
use bevy_ecs::prelude::*;
use bevy_input::prelude::*;
use bevy_log::prelude::*;
use bevy_math::prelude::*;
use bevy_mesh::prelude::*;
use bevy_pbr::prelude::*;
use bevy_shape::{Capsule3d, Cuboid};
use bevy_time::prelude::*;
use bevy_transform::prelude::*;
use bevy_window::{CursorGrabMode, CursorOptions, PrimaryWindow, Window};
use bevy_enhanced_input::prelude::*;
use sf_action::{
    ActionClass, ActionState, BASE_RULES, CharacterId, CharacterParams, Origin, RootPath, RuleKey, RuleSetBuilder, SlotInfo, SlotTable,
    TRAVERSAL_RULES,
};
use sf_action_bevy::{
    ActionBody, ActionCharacter, ActionInputPlugin, ActionRules, ActionSystems, AwaitingGround, PlayerAction, PlayerControls, PlayerId,
    PlayerInput, SfActionPlugin, spawn_player_actions,
};
use sf_anim::{Clip, Skeleton};
use sf_camera::Look;
use sf_camera_bevy::{CameraAction, PlayerCamera, SfCameraPlugin, spawn_camera_actions};
use sf_character_bevy::CharacterPlacement;
use sf_combat::CompiledSheet;
use sf_console_bevy::ConsoleOpen;
use sf_controller::{GroundHit, GroundProbe, Locomotion, MoverParams, ProbeQuery};
use sf_controller_bevy::{AvoidObstacle, Mover};
use sf_coords::{WorldPos, WorldTransform};
use sf_physics::{BodyId, Layer, PhysicsWorld};
use sf_physics_bevy::{Physics, PhysicsStreaming};
use sf_records::{RecordDb, RecordId};
use sf_script::Handle as ScriptHandle;
use sf_script_bevy::{ScriptPlacements, ScriptSlot};
use sf_world::{PlacementId, WorldspaceId};
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_world_bevy::{ActiveWorldspace, FadePhase, FadeTransition, StreamingSystems, TravelArrived, WorldPosOf, from_grid, to_grid};

// Moved to `mw_game_core` (app-split.md slice 1); the old paths keep working.
pub use mw_game_core::player::{FocusUpdate, PlayerBody, PlayerMode};

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
/// After an arrival the body waits until the collision this close around its feet is in
/// (half size of the square, meters): the floor under it plus a step in any direction.
const ARRIVAL_GROUND_MARGIN_M: f32 = 3.0;
/// The body moves anyway after waiting this long for its collision, seconds (a cell that never
/// comes in must not hang the game; the fade's own cap is `FadeTransition::max_hold_secs`).
const ARRIVAL_WAIT_MAX_SECS: f32 = 10.0;
/// Ground placement on an arrival release starts this far above the feet, meters (a marker sunk
/// into the floor still finds its top; `OpenMW` `World::adjustPosition` lifts 20 units).
const PLACE_ABOVE_M: f32 = 1.0;
/// ... and looks this far below the feet, meters (further down, the fall guards take over).
const PLACE_BELOW_M: f32 = 5.0;
/// Placed feet sit this far above the found ground, meters (`OpenMW` `sGroundOffset`, 1 unit).
const GROUND_OFFSET_M: f32 = 0.02;
/// The swept shape's ground stands unless the thin ray's differs by more, meters (`OpenMW`
/// `MovementSolver::traceDown`: 35 units).
const SHAPE_RAY_TOLERANCE_M: f32 = 0.5;
/// A fall this far below an interior's lowest fixed body is a fall into the void: back to the
/// arrival spot, meters (`OpenMW` `Scene::playerMoved` uses 90 units, ~1.3 m).
const INTERIOR_VOID_MARGIN_M: f32 = 2.0;
/// The player's character id on commands (NPCs don't run the action layer yet).
const PLAYER_CHARACTER: CharacterId = CharacterId(1);

/// The traversal slots the player's rules use (`sf_action::TRAVERSAL_RULES`).
pub const TRAVERSAL_SLOTS: &[&str] = &["traverse.mantle.waist", "traverse.mantle.high"];

/// Every slot the player's rules play a baked clip in: [`TRAVERSAL_SLOTS`] and the combat
/// sheet's ([`mw_combat::sheet_slots`]). The host loads their clips from the baked humanoid graph.
#[must_use]
pub fn player_slots() -> Vec<String> {
    let combat = mw_combat::sheet_slots(mw_combat::SWORD_1H).unwrap_or_default();
    TRAVERSAL_SLOTS.iter().map(|s| (*s).to_owned()).chain(combat).collect()
}

/// A baked slot clip: its length and root track (action.md B2a; a cooked clip is a `RootPath`).
/// A traversal slot without one refuses its move (`Refused::NoRootPath`: the mantle falls back to
/// a plain jump); a combat slot without one is dropped by the sheet.
pub struct SlotClip {
    pub slot: String,
    pub clip: Arc<Clip>,
}

/// The clips of [`player_slots`] the host could load, and the skeleton they were cooked for
/// (the weapon socket tracks are sampled on it).
#[derive(Default)]
pub struct PlayerClips {
    pub slots: Vec<SlotClip>,
    pub skeleton: Option<Arc<Skeleton>>,
}

/// `clips`: the player's baked slot clips ([`PlayerClips`]).
pub fn add(app: &mut App, db: Option<Arc<RecordDb>>, start_on: bool, start: PlayerStart, clips: PlayerClips) {
    if !app.is_plugin_added::<EnhancedInputPlugin>() {
        app.add_plugins(EnhancedInputPlugin);
    }
    let reach_m = activate_reach_m(db.as_deref());
    let app_db = db.clone();
    let sheet = combat::load_sheet(&clips);
    mw_controls_bevy::add(app);
    app.add_plugins((SfActionPlugin::<Physics>::default(), ActionInputPlugin, SfCameraPlugin::<Physics>::default()))
        .insert_resource(rules(&clips.slots, sheet.as_ref()))
        .insert_resource(avatar::AvatarSlots::new(player_slots(), clips.slots.iter().map(|c| (c.slot.clone(), c.clip.duration))))
        .insert_resource(PlayerMode { on: start_on, paused: false, body: None, last_ground: None })
        .init_resource::<RestoreBody>()
        .init_resource::<ArrivalSpot>()
        .insert_resource(Activation { db, reach_m })
        .insert_resource(start)
        .init_resource::<Focus>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (toggle, apply_mode, rebind, arrive, follow_worldspace, guard_fall, obstacle, controls_active, grab_cursor).chain(),
        )
        .add_systems(Update, await_ground::<Physics>.after(arrive).before(StreamingSystems::Transition))
        .add_systems(Update, update_focus.in_set(FocusUpdate).before(ScriptSlot::Update))
        .add_systems(RunFixedMainLoop, regrid.in_set(RunFixedMainLoopSystems::AfterFixedMainLoop).after(ActionSystems::Interpolate))
        .add_systems(PostUpdate, (avatar::link, avatar::feed).chain().in_set(sf_anim_bevy::AnimSystems::Probe))
        .add_systems(PostUpdate, avatar::first_person);
    hud::add(app, app_db.as_ref());
    combat::add(app, sheet, &clips);
    dummy::add(app);
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

/// The base, traversal and combat move rules and the slot table the chooser times windows with.
/// Scaffold `PlayerSlots`: fixed clip lengths until B1's animation feed reports the real ones; the
/// traversal slots take their baked clips' lengths, the `sheet`'s slots its timings, and every
/// baked slot its root track.
fn rules(clips: &[SlotClip], sheet: Option<&CompiledSheet>) -> ActionRules {
    let mut slots = SlotTable::new()
        .with("loco.move", SlotInfo::looping())
        .with("air.fall", SlotInfo::looping())
        .with("air.jump", SlotInfo::once(0.3))
        .with("air.land.normal", SlotInfo::once(0.25))
        .with("air.land.hard", SlotInfo::once(0.5));
    if let Some(sheet) = sheet {
        slots = sheet.fill(slots);
    }
    for c in clips {
        if TRAVERSAL_SLOTS.contains(&c.slot.as_str()) {
            slots = slots.with(&c.slot, SlotInfo::once(c.clip.duration));
        }
        let root: Arc<dyn RootPath + Send + Sync> = c.clip.clone();
        slots = slots.with_root(&c.slot, root);
    }
    let mut rules = RuleSetBuilder::new();
    rules.add_ron(&Origin::Framework, BASE_RULES).add_ron(&Origin::Framework, TRAVERSAL_RULES);
    if let Some(sheet) = sheet {
        sheet.add_rules(&mut rules, &Origin::Mod("bevyrrowind".into()));
    }
    ActionRules { rules: rules.build().expect("framework base and traversal rules parse"), slots: Box::new(slots) }
}

/// The capsule mesh with a facing marker, drawn only when the app runs without dressed characters
/// (no `sf_character_bevy::Characters`: headless tests, runs with characters off): the dressed body is [`avatar`].
#[derive(Resource)]
struct PlayerVisual {
    capsule: Handle<Mesh>,
    nose: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let p = feel::mover_params();
    commands.insert_resource(PlayerVisual {
        capsule: meshes.add(Capsule3d::new(p.radius, (p.height - 2.0 * p.radius).max(0.01))),
        nose: meshes.add(Cuboid::new(0.12, 0.12, 0.3)),
        material: materials.add(StandardMaterial { base_color: Color::srgb(0.75, 0.55, 0.3), perceptual_roughness: 0.8, ..Default::default() }),
    });
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
    use mw_controls_bevy::{Control, bind};
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
            PlayerAction::Button(ActionClass::Sneak) => bind(commands, keymap, Control::Sneak, action, plain),
            PlayerAction::Button(ActionClass::Light) => bind(commands, keymap, Control::Attack, action, plain),
            // One button: tap rolls on release, hold sprints (the separate Sprint and Dodge buttons
            // stay unbound).
            PlayerAction::DodgeSprint => bind(commands, keymap, Control::Sprint, action, plain),
            PlayerAction::Button(_) => {}
        }
    }
    let ready = commands.spawn((ActionOf::<PlayerControls>::new(body), Action::<combat::ReadyWeaponAction>::new())).id();
    bind(commands, keymap, Control::ReadyWeapon, ready, plain);
    combat::spawn_lock(commands, keymap, body);
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
/// `dressed`: the `player` record's character is drawn ([`avatar`]), else the capsule mesh.
fn spawn_body(
    commands: &mut Commands,
    visual: &PlayerVisual,
    keymap: &sf_ui::Keymap,
    (feet, yaw): (WorldPos, f32),
    root: Entity,
    dressed: bool,
) -> Entity {
    let (cell, transform) = to_grid(feet);
    let params = feel::mover_params();
    let half = params.height / 2.0;
    let measures =
        CharacterParams { gravity: params.gravity, height: params.height, radius: params.radius, ..CharacterParams::default() };
    let character = ActionCharacter::new(PLAYER_CHARACTER, ActionState::new(RuleKey::of("loco")), measures);
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
            avatar::PlayerFeed::default(),
            cell,
            transform,
            Visibility::default(),
            ChildOf(root),
        ))
        .id();
    spawn_controls(commands, keymap, id);
    if dressed {
        commands.spawn((avatar::avatar_bundle(), ChildOf(id)));
        return id;
    }
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
    pub fn of(world: &mw_doors_bevy::WorldLinks, start: &WorldspaceId) -> Self {
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
        let look = Look::from_dir(mw_doors_bevy::arrival_rotation(marker.rotation) * Vec3::NEG_Z);
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
    mut spot: ResMut<ArrivalSpot>,
    physics: Res<Physics>,
    collision: sf_physics_bevy::CollisionReady,
    settled: Option<Res<mw_game_core::exterior::Settled>>,
    visual: Res<PlayerVisual>,
    characters: Option<Res<sf_character_bevy::Characters>>,
    keymap: Res<sf_ui_bevy::UiKeymap>,
    positions: WorldPosOf,
    mut cams: Query<(Entity, &GlobalTransform, &ChildOf, &mut FreeCameraState), With<mw_doors_bevy::DoorUser>>,
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
                if collision.loading(&here.worldspace, here.pos, 2.0) {
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
            let body = spawn_body(&mut commands, &visual, &keymap.0, (feet, look.yaw), parent.parent(), characters.is_some());
            let mut camera = PlayerCamera::new(body);
            camera.set_look(look);
            commands.entity(cam).insert(camera);
            mode.body = Some(body);
            mode.last_ground = None;
            spot.0 = Some((feet, look.yaw));
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

/// The last arrival pose, feet and yaw (door marker, save-load position or first spawn; the
/// ground placement on release refines it): where the fall guards return the player when no
/// ground is known ([`guard_fall`]).
#[derive(Resource, Default, Clone, Copy, Debug)]
pub struct ArrivalSpot(pub Option<(WorldPos, f32)>);

/// A door moved the camera: the body follows to the arrival marker (a load: the saved body).
fn arrive(
    mut commands: Commands,
    mut arrived: MessageReader<TravelArrived>,
    mut mode: ResMut<PlayerMode>,
    mut spot: ResMut<ArrivalSpot>,
    mut cams: Query<&mut PlayerCamera>,
    mut bodies: Query<(&mut ActionBody, Option<&mut ActionCharacter>), With<PlayerBody>>,
    mut restore: ResMut<RestoreBody>,
) {
    for travel in arrived.read() {
        let Ok(mut camera) = cams.get_mut(travel.traveler) else { continue };
        let Ok((mut body, character)) = bodies.get_mut(camera.follow) else { continue };
        let feet = travel.to.pos.offset(Vec3::NEG_Y * mw_doors_bevy::ARRIVAL_EYE_M);
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
        spot.0 = Some((body.loco.position, body.loco.motion.yaw));
        // The cut re-streams the collision (a load reloads every resident cell): hold the body
        // until the floor under it is in (`await_ground`).
        commands.entity(camera.follow).insert(AwaitingGround);
    }
}

/// Releases the arrived body ([`AwaitingGround`]) once the collision around its feet is in and
/// the fade's destination is ready; keeps the screen black meanwhile, so the frozen frames
/// never show. Gives up after [`ARRIVAL_WAIT_MAX_SECS`]. On release the feet go onto the ground
/// below ([`ground_below`]), which seeds `last_ground` and the [`ArrivalSpot`]; no ground in
/// reach leaves the body as is (the fall guards catch it).
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments, clippy::type_complexity)] // system parameters
fn await_ground<P: GroundProbe + Resource>(
    mut commands: Commands,
    time: Res<Time<Real>>,
    mut waited: Local<f32>,
    collision: sf_physics_bevy::CollisionReady,
    active: Res<ActiveWorldspace>,
    probe: Res<P>,
    mut mode: ResMut<PlayerMode>,
    mut spot: ResMut<ArrivalSpot>,
    mut fade: ResMut<FadeTransition>,
    // The player body (`arrive`, `guard_fall`); a training dummy waits on its own (`dummy.rs`).
    mut bodies: Query<(Entity, &mut ActionBody), (With<AwaitingGround>, With<PlayerBody>)>,
) {
    let Some((entity, mut body)) = bodies.iter_mut().next() else {
        *waited = 0.0;
        return;
    };
    *waited += time.delta_secs();
    let feet = body.loco.position;
    // Not before the fade's destination is ready: until then the worldspace's directory may not
    // be known, and an unknown worldspace reads as "collision in".
    let arriving = matches!(fade.phase(), FadePhase::Hold { ready: None, .. });
    let loading = active.0.as_ref().is_some_and(|ws| collision.loading(ws, feet, ARRIVAL_GROUND_MARGIN_M));
    if arriving || loading {
        if *waited < ARRIVAL_WAIT_MAX_SECS {
            fade.hold();
            return;
        }
        warn!("player: no collision at {:.1?} after {ARRIVAL_WAIT_MAX_SECS} s; moving anyway", feet.to_f64());
    }
    commands.entity(entity).remove::<AwaitingGround>();
    *waited = 0.0;
    let Some(ground) = ground_below(&*probe, feet, &body.params) else { return };
    let mut loco = body.loco.clone();
    loco.position = feet.offset(Vec3::Y * (ground + GROUND_OFFSET_M));
    loco.vertical_speed = 0.0;
    mode.last_ground = Some(loco.position);
    spot.0 = Some((loco.position, loco.motion.yaw));
    body.restore(loco);
}

/// The ground height under `feet`, relative to them (meters, up), within
/// [`PLACE_ABOVE_M`] above and [`PLACE_BELOW_M`] below. Two stages (`OpenMW`
/// `MovementSolver::traceDown`, `movementsolver.cpp:90`, commit 4e718b50): a sphere of the body's
/// radius swept down, then a thin ray; the ray wins when the two disagree by more than
/// [`SHAPE_RAY_TOLERANCE_M`] or the sphere's ground is too steep (door destinations that
/// intersect other geometry once the body's width counts). A cast that starts inside a solid
/// counts as no hit.
pub(crate) fn ground_below(probe: &impl GroundProbe, feet: WorldPos, params: &MoverParams) -> Option<f32> {
    let from = feet.offset(Vec3::Y * PLACE_ABOVE_M);
    let max = PLACE_ABOVE_M + PLACE_BELOW_M;
    let started_out = |h: &GroundHit| h.distance > 1e-3;
    let shape = probe.probe_one(ProbeQuery::sphere(from, Vec3::NEG_Y, max, params.radius)).filter(started_out);
    let ray = probe.probe_one(ProbeQuery::down(from, max)).filter(started_out);
    let height = |h: GroundHit| feet.delta_f32(h.point).y;
    match (shape, ray) {
        (Some(s), Some(r)) if (height(s) - height(r)).abs() > SHAPE_RAY_TOLERANCE_M || !s.walkable(params.min_normal_y()) => Some(height(r)),
        (Some(s), _) => Some(height(s)),
        (None, r) => r.map(height),
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

/// A hole in the collision drops the body; three guards put it back and hold it until the
/// ground there is in ([`AwaitingGround`]): [`FALL_LIMIT_M`] below its last ground (back
/// there); in an interior, [`INTERIOR_VOID_MARGIN_M`] below the cell's lowest fixed body (back
/// to the [`ArrivalSpot`]; `OpenMW` `Scene::playerMoved`, `scene.cpp:575`, commit 4e718b50); in
/// the exterior, under the world's kill height (back to the last ground, else the arrival spot).
/// A grounded frame costs one check; a falling one a world lookup and a comparison.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn guard_fall(
    mut commands: Commands,
    mut mode: ResMut<PlayerMode>,
    spot: Res<ArrivalSpot>,
    physics: Res<Physics>,
    active: Res<ActiveWorldspace>,
    mut bodies: Query<(&mut ActionBody, Has<AwaitingGround>), With<PlayerBody>>,
) {
    let Some(entity) = mode.body else { return };
    let Ok((mut body, waiting)) = bodies.get_mut(entity) else { return };
    let here = body.loco.render_position();
    if body.loco.grounded.is_some() {
        mode.last_ground = Some(here);
        return;
    }
    if waiting {
        return;
    }
    let yaw = body.loco.motion.yaw;
    let last = mode.last_ground.map(|g| (g, yaw));
    let back = if let Some((ground, _)) = last
        && ground.delta_f32(here).y < -FALL_LIMIT_M
    {
        warn!("player: fell {FALL_LIMIT_M} m (no collision below?); back to {:.1?}", ground.to_f64());
        last
    } else {
        let Some((ws, world)) = active.0.as_ref().and_then(|ws| Some((ws, physics.world(ws)?))) else { return };
        let y = here.to_f64().y;
        if ws.as_str() == mw_world::ids::EXTERIOR {
            if !world.kill_height().is_some_and(|kill| y < kill) {
                return;
            }
            warn!("player: fell under the kill height of {}; back to the last ground or arrival", ws.as_str());
            last.or(spot.0)
        } else {
            if !world.lowest_fixed().is_some_and(|low| y < low - f64::from(INTERIOR_VOID_MARGIN_M)) {
                return;
            }
            warn!("player: fell into the void under {}; back to the arrival spot", ws.as_str());
            spot.0.or(last)
        }
    };
    let Some((feet, yaw)) = back else { return };
    body.teleport(feet, yaw);
    body.frame_origin = from_grid(&to_grid(feet).0, &Transform::IDENTITY);
    commands.entity(entity).insert(AwaitingGround);
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

/// The cursor is grabbed and hidden while gameplay owns the mouse ([`wants_grab`]): the player's
/// views and the free camera (F11) alike. While wanted the grab is held every frame, not only set
/// on the edge: other writers drop it, notably Bevy's free camera controller, whose right button
/// grab ends with the cursor freed and shown on the release (`run_freecamera_controller`, before
/// `Update`). Freeing happens once, on the edge, so whoever freed it (a menu, the console's
/// click-select) keeps it.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn grab_cursor(
    mode: Res<PlayerMode>,
    console: Option<Res<ConsoleOpen>>,
    mut windows: Query<(&Window, &mut CursorOptions), With<PrimaryWindow>>,
    mut grabbed: Local<bool>,
) {
    let Ok((window, mut cursor)) = windows.single_mut() else { return };
    let want = wants_grab(&mode, window.focused, console.is_some_and(|c| c.0));
    if want {
        if cursor.grab_mode != GAMEPLAY_GRAB || cursor.visible {
            cursor.grab_mode = GAMEPLAY_GRAB;
            cursor.visible = false;
        }
    } else if *grabbed {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
    }
    *grabbed = want;
}

/// Gameplay owns the mouse: a body is driven or the free camera flies (F11: the player is off),
/// nothing paused it (menus, dialogue), the console is closed, the window has focus. Not while the
/// player is wanted but has no body yet (the spawn waits for the world).
fn wants_grab(mode: &PlayerMode, focused: bool, console: bool) -> bool {
    (mode.body.is_some() || !mode.on) && !mode.paused && !console && focused
}

/// How gameplay grabs the cursor. On Windows, winit's `Locked` clips the cursor to one pixel at
/// wherever it is when the grab lands, which is outside the window when focus came from elsewhere
/// (taskbar, terminal): the cursor stays there, visible, and clicks go to the window behind.
/// `Confined` with the cursor hidden clips it to the window's centre instead (winit
/// `refresh_os_cursor`). Elsewhere `Locked` is right (macOS has no `Confined`; X11 falls back to it).
/// Mouse look reads raw motion, so neither mode affects it.
const GAMEPLAY_GRAB: CursorGrabMode = if cfg!(target_os = "windows") { CursorGrabMode::Confined } else { CursorGrabMode::Locked };

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
    cams: Query<(Entity, &GlobalTransform, Option<&PlayerCamera>), With<mw_doors_bevy::DoorUser>>,
    actors: Query<(&Mover, &CharacterPlacement, &mw_walkers_bevy::walkers::ActorSpace), Without<PlayerBody>>,
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

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_avatar;
#[cfg(test)]
mod tests_feel;
#[cfg(test)]
mod tests_combat;
