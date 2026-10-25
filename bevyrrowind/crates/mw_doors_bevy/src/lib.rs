//! Door links in the streamed view (ROADMAP M2, streaming.md "Worldspaces at
//! runtime"): E (the use key; Morrowind's was Space) on a door in front of the
//! camera crosses its link with the framework's `FadeTransition`, and each
//! worldspace gets its own look (an interior has no sky, terrain, clouds or
//! sea; its ambient light and fog color come from the cell).
//!
//! Door prefetch is the framework's: the gameplay anchor (the player, which
//! follows the camera) within 24 m of a link end keeps the other side's R0
//! resident, so the fade rarely holds.

use std::collections::{BTreeMap, HashMap};

use bevy_app::{App, PostUpdate, Update};
use bevy_camera::visibility::RenderLayers;
use bevy_camera::{Camera, ClearColorConfig, Exposure};
use bevy_camera_controller::free_camera::FreeCameraState;
use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_input::{ButtonInput, keyboard::KeyCode};
use bevy_light::{AmbientLight, GlobalAmbientLight};
use bevy_log::{info, warn};
use bevy_math::{EulerRot, Quat, Vec3};
use bevy_transform::components::GlobalTransform;
use mw_assets::LightCalibration;
use mw_game_core::exterior::Settled;
use mw_game_core::perf::{Off, Part};
use mw_game_core::player::PlayerBody;
use mw_world::CellEnvironment;
use sf_sky_bevy::SkyCamera;
use sf_terrain_bevy::{TerrainCamera, TerrainPatch};
use sf_world::{DoorLink, LinkEnd, Location, Worldspace, WorldspaceId};
use sf_world_bevy::{
    ActiveWorldspace, DoorLinks, FadeTransition, StartTravel, StreamingSystems, TravelArrived, WorldPosOf,
    WorldspaceRecords, WorldspaceRegistry,
};

// Moved to `mw_game_core` (app-split.md slice 1); the old paths keep working.
pub use mw_game_core::doors::{ARRIVAL_EYE_M, DoorUser};

/// How far in front of the camera a door can be activated, meters
/// (Morrowind's activation reach is about 2.7 m from the eye; the free
/// camera floats, so a little more).
pub const REACH_M: f32 = 4.0;
/// Doors this far are activated whatever the camera faces.
const NEAR_M: f32 = 1.5;
/// Facing: the door must lie within ~60° of the view direction.
const MIN_FACING: f32 = 0.5;

/// Every worldspace record, door link and interior environment of the
/// loaded content.
pub struct WorldLinks {
    pub links: Vec<DoorLink>,
    pub worldspaces: BTreeMap<WorldspaceId, Worldspace>,
    pub environments: HashMap<WorldspaceId, CellEnvironment>,
}

impl WorldLinks {
    pub fn build(db: &sf_records::RecordDb) -> Self {
        let records = mw_world::WorldRecords::build(db);
        let by_cell: HashMap<_, _> = records.environments.into_iter().collect();
        let environments = records
            .worldspaces
            .iter()
            .filter_map(|(id, ws)| Some((id.clone(), by_cell.get(&ws.environment.as_ref()?.id)?.clone())))
            .collect();
        Self {
            links: records.door_links.into_iter().map(|(_, l)| l).collect(),
            worldspaces: records.worldspaces.into_iter().collect(),
            environments,
        }
    }
}

/// Interior environments by worldspace.
#[derive(Resource, Default)]
pub struct Environments(pub HashMap<WorldspaceId, CellEnvironment>);

/// An interior's look (no sky in its cell flags): the cell's ambient light
/// as a camera override and the fog color it clears to. `None`: the
/// worldspace shows the sky (the exterior, or a cell that behaves like one).
pub fn interior_look(env: Option<&CellEnvironment>, calibration: &LightCalibration) -> Option<(AmbientLight, Color)> {
    let env = env.filter(|e| !e.sky)?;
    let (amb, fog) = env.atmosphere.map_or(([0.3; 3], [0.0; 3]), |a| (a.ambient, a.fog_color));
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "clamped to 0..=255")]
    let srgb = |c: [f32; 3]| c.map(|v| (v * 255.0).round().clamp(0.0, 255.0) as u8);
    let global = calibration.ambient(srgb(amb));
    let ambient = AmbientLight {
        color: global.color,
        brightness: global.brightness,
        affects_lightmapped_meshes: global.affects_lightmapped_meshes,
    };
    Some((ambient, Color::srgb(fog[0], fog[1], fog[2])))
}

/// `--use-door`: cross the nearest door once the start has settled.
#[derive(Resource)]
pub struct AutoDoor {
    /// Doors still to walk through.
    pub remaining: u32,
}

/// The camera's look in the exterior, restored when it comes back out.
#[derive(Default)]
pub struct ExteriorLook {
    ambient: Option<GlobalAmbientLight>,
    exposure: Option<Exposure>,
}

pub fn add(app: &mut App, world: WorldLinks, use_door: u32) {
    info!(
        "door links: {} ({} worldspaces, {} interior environments)",
        world.links.len(),
        world.worldspaces.len(),
        world.environments.len()
    );
    app.insert_resource(DoorLinks(world.links))
        .insert_resource(WorldspaceRecords(world.worldspaces))
        .insert_resource(Environments(world.environments))
        .insert_resource(AutoDoor { remaining: use_door })
        .add_systems(Update, report_travel)
        // After the script slot: the activate key's door opens only if no script handled it.
        .add_systems(Update, activate.after(sf_script_bevy::ScriptSlot::Update).before(StreamingSystems::Transition))
        // Before the sky applies: `setup_sky_cameras` must see this frame's
        // `SkyCamera` insert/remove, or a portal crossing renders one frame
        // with the other side's atmosphere and environment light (the
        // crossing flash, portal3: luma 21 -> 177 -> 38 walking in at noon).
        .add_systems(Update, (worldspace_look, sync_free_camera).after(StreamingSystems::Transition).before(sf_sky_bevy::SkySystems::Apply))
        .add_systems(PostUpdate, terrain_layers);
}

/// The nearest link end in front of `here` (looking along `forward`; `None`:
/// any direction) within `reach`, and the end it leads to.
pub fn pick<'a>(
    links: &'a [DoorLink],
    here: &Location,
    forward: Option<Vec3>,
    reach: f32,
) -> Option<(&'a LinkEnd, &'a LinkEnd)> {
    let mut best: Option<(f32, (&LinkEnd, &LinkEnd))> = None;
    for link in links {
        for (from, to) in link.exits() {
            if from.worldspace != here.worldspace {
                continue;
            }
            let d = here.pos.delta_f32(from.frame.pos);
            let dist = d.length();
            if dist > reach {
                continue;
            }
            if let Some(forward) = forward
                && dist > NEAR_M
                && forward.dot(d / dist) < MIN_FACING
            {
                continue;
            }
            if best.is_none_or(|(b, _)| dist < b) {
                best = Some((dist, (from, to)));
            }
        }
    }
    best.map(|(_, pair)| pair)
}

/// The camera's rotation at an arrival marker: the marker's heading, level.
///
/// A door's destination (DODT) is a position and per-axis angles applied X,
/// then Y, then Z (`mw_world::convert::world_transform`). The traveler takes
/// only the Z angle as its heading, as Morrowind and `OpenMW` place the player
/// at a teleport: 0 faces north (+Y in Morrowind, engine -Z), positive turns
/// clockwise seen from above. Markers are not models, so no facing turn
/// applies (unlike rigged actors, `mw_anim::rig::FACING_TURN`). In engine
/// space the Z angle is the leading Y rotation of a Y-Z-X decomposition
/// (`Ry(-z) Rz(y) Rx(-x)`), which any pitch or roll on the marker leaves
/// intact; a flattened forward vector would flip past 90 degrees of pitch.
/// The camera keeps level (`OpenMW` also applies the X angle as pitch; door
/// markers in the vanilla data are level, and arriving level reads better).
pub fn arrival_rotation(rotation: Quat) -> Quat {
    let (yaw, _, _) = rotation.to_euler(EulerRot::YZX);
    Quat::from_rotation_y(yaw)
}

/// Bevy's `FreeCamera` caches yaw and pitch and rebuilds the rotation from
/// them on the next mouse motion, so a camera the fade turned would snap
/// back. Resync the cache (as the controller's own init does, Y-X-Z Euler)
/// and drop the carried velocity when a travel moves the camera. The
/// temporal resolve (TAA/DLSS) starts over: last frame was the other side.
pub fn sync_free_camera(
    mut arrived: MessageReader<TravelArrived>,
    mut cams: Query<&mut FreeCameraState>,
    mut history: Option<ResMut<sf_sky_bevy::TemporalHistory>>,
) {
    for travel in arrived.read() {
        if let Some(h) = history.as_mut() {
            h.reset = true;
        }
        let Ok(mut state) = cams.get_mut(travel.traveler) else { continue };
        let (yaw, pitch, _) = travel.rotation.to_euler(EulerRot::YXZ);
        state.yaw = yaw;
        state.pitch = pitch;
        state.velocity = Vec3::ZERO;
        state.rotation_curve = None;
    }
}

/// Opens a door: the default action of the player's activation of a door
/// placement that no script handled (`ActivationOutcome`, M5 L3; the activate
/// key and crosshair pick are `scripts.rs`'s). Link ends without a door
/// object (teleport points) keep the proximity pick on E, from the player body in player mode
/// (the third-person camera sits metres behind it), else the camera. `--use-door`
/// travels directly, once the start has streamed in.
#[allow(clippy::too_many_arguments)]
fn activate(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    links: Res<DoorLinks>,
    fade: Res<FadeTransition>,
    mut auto: ResMut<AutoDoor>,
    settled: Option<Res<Settled>>,
    positions: WorldPosOf,
    cams: Query<(Entity, &GlobalTransform), With<DoorUser>>,
    bodies: Query<&sf_action_bevy::ActionBody, With<PlayerBody>>,
    mut outcomes: MessageReader<sf_script_bevy::ActivationOutcome>,
    mut travel: MessageWriter<StartTravel>,
) {
    let opened: Vec<_> = outcomes
        .read()
        .filter(|o| !o.handled && o.activation.actor == sf_script_bevy::PLAYER)
        .map(|o| sf_world::PlacementId(o.activation.target.0))
        .collect();
    if fade.is_active() {
        return;
    }
    let automatic = auto.remaining > 0 && settled.is_some();
    let manual = keys.just_pressed(KeyCode::KeyE);
    if !automatic && !manual && opened.is_empty() {
        return;
    }
    let Ok((cam, global)) = cams.single() else { return };
    let Some(mut here) = positions.location(cam) else { return };
    // Player mode: proximity from the body's mid-height (same worldspace: `player::follow_worldspace`).
    if let Ok(body) = bodies.single() {
        here.pos = body.loco.render_position().offset(Vec3::Y * (body.params.height / 2.0));
    }
    // An activated door placement's link end, if any.
    let by_door = links
        .0
        .iter()
        .flat_map(DoorLink::exits)
        .find(|(from, _)| from.worldspace == here.worldspace && from.door.is_some_and(|d| opened.contains(&d)));
    // `--use-door` takes the nearest door around the camera, any direction.
    let reach = if automatic { 3.0 * REACH_M } else { REACH_M };
    let forward = (!automatic).then(|| global.forward().as_vec3());
    let picked = match by_door {
        Some(pair) => Some(pair),
        None if automatic => pick(&links.0, &here, forward, reach),
        // E on a doorless link end (no placement to activate).
        None if manual => pick(&links.0, &here, forward, reach).filter(|(from, _)| from.door.is_none()),
        None => return,
    };
    let Some((from, to)) = picked else {
        if automatic {
            // Say where the nearest one is, for a `--camera` that reaches it.
            let nearest = pick(&links.0, &here, None, f32::INFINITY).map(|(from, _)| from.frame.pos.to_f64());
            warn!("--use-door: no door within {reach} m of {:?} (nearest door at {nearest:?})", here.pos.to_f64());
            auto.remaining = 0;
        }
        return;
    };
    if automatic {
        auto.remaining -= 1;
        // The next door waits for this side to stream in.
        commands.remove_resource::<Settled>();
    }
    info!(
        "door: {} ({:.1} m away) -> {} at {:.1?}",
        from.worldspace.as_str(),
        here.pos.delta_f32(from.frame.pos).length(),
        to.worldspace.as_str(),
        to.arrival.pos.to_f64()
    );
    travel.write(StartTravel {
        traveler: cam,
        to: Location::new(to.worldspace.clone(), to.arrival.pos.offset(Vec3::Y * ARRIVAL_EYE_M)),
        rotation: arrival_rotation(to.arrival.rotation),
        reload: false,
    });
}

/// Logs each crossing's duration (request to the end of the fade-in).
fn report_travel(fade: Res<FadeTransition>, mut last: Local<Option<f32>>) {
    if fade.last_duration != *last {
        *last = fade.last_duration;
        if let Some(secs) = fade.last_duration {
            info!(
                "door travel took {secs:.2} s (fade {:.1} s each way, held {:.2} s at black, {:.2} s of it for gates: lamp shadow captures, pipelines)",
                fade.fade_secs,
                fade.last_hold.unwrap_or_default(),
                fade.last_gate.unwrap_or_default()
            );
        }
    }
}

/// The camera's look follows the active worldspace: an interior (no sky in
/// its cell flags) drops the sky, clouds and terrain from the camera, lights
/// with the cell's ambient color at interior exposure, and clears to its fog
/// color; the exterior (or a cell that behaves like one) gets them back.
/// The listener is outdoors exactly where the sky shows.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn worldspace_look(
    mut commands: Commands,
    active: Res<ActiveWorldspace>,
    envs: Res<Environments>,
    registry: Res<WorldspaceRegistry>,
    calibration: Res<LightCalibration>,
    off: Res<Off>,
    mut ambient: ResMut<GlobalAmbientLight>,
    outdoors: Option<ResMut<sf_audio_bevy::ListenerOutdoors>>,
    mut cams: Query<(Entity, &mut Camera, Option<&Exposure>, Has<SkyCamera>, Has<TerrainCamera>), With<DoorUser>>,
    mut seen: Local<Option<WorldspaceId>>,
    mut saved: Local<ExteriorLook>,
) {
    let Some(ws) = active.0.clone() else { return };
    if seen.as_ref() == Some(&ws) {
        return;
    }
    if registry.get(&ws).is_none() {
        return;
    }
    let Ok((cam, mut camera, exposure, has_sky, has_terrain)) = cams.single_mut() else { return };
    *seen = Some(ws.clone());
    let env = envs.0.get(&ws);
    let sky = env.is_none_or(|e| e.sky);
    // Weather and region sound stay outside (AU-1): the mixer's interior snapshot.
    if let Some(mut outdoors) = outdoors {
        outdoors.0 = sky;
    }
    // Sun and moon light the worldspace that shows the sky, and only that
    // (`exterior::light_layers`; on an interior's layer they lit the office with
    // the noon sun for the frame of a portal crossing, the white flash, portals.md).
    let mut e = commands.entity(cam);
    if let Some((amb, clear)) = interior_look(env, &calibration) {
        if saved.ambient.is_none() {
            saved.ambient = Some(ambient.clone());
            saved.exposure = exposure.copied();
        }
        *ambient = GlobalAmbientLight { color: amb.color, brightness: amb.brightness, ..GlobalAmbientLight::default() };
        // The sky camera left its `AmbientLight` override on the camera,
        // which wins over the global resource: the cell's replaces it.
        e.remove::<(SkyCamera, TerrainCamera, sf_atmos_bevy::CloudCamera)>()
            .insert((Exposure { ev100: mw_assets::SCENE_EV100 }, amb));
        camera.clear_color = ClearColorConfig::Custom(clear);
        info!("worldspace look: interior {}", ws.as_str());
        return;
    }
    // Under the sky again: `SkyCamera` brings its own override back.
    e.remove::<AmbientLight>();
    if let Some(a) = saved.ambient.take() {
        *ambient = a;
    }
    if let Some(x) = saved.exposure.take() {
        e.insert(x);
    }
    camera.clear_color = ClearColorConfig::Default;
    if sky && !has_sky && !off.has(Part::Sky) {
        e.insert(SkyCamera { environment_light: !off.has(Part::Envmap), ..SkyCamera::default() });
        if !off.has(Part::Clouds) || !off.has(Part::Fog) {
            e.insert(sf_atmos_bevy::CloudCamera);
        }
    }
    // The land is the exterior's: another worldspace under the sky (a test
    // level, `level.rs`) has none. With `TerrainCamera` there, the terrain
    // streamed Vvardenfell's patches around the camera and each new patch
    // drew for a frame before `terrain_layers` moved it to the exterior's
    // layer (black flashes in `level-ledges`, owner 2026-10-18).
    if ws == mw_world::ids::exterior() {
        if !has_terrain && !off.has(Part::Terrain) {
            e.insert(TerrainCamera);
        }
    } else if has_terrain {
        e.remove::<TerrainCamera>();
    }
    info!("worldspace look: {}", ws.as_str());
}

/// Terrain patches draw in the exterior's view only.
fn terrain_layers(
    mut commands: Commands,
    registry: Res<WorldspaceRegistry>,
    patches: Query<Entity, (Added<TerrainPatch>, Without<RenderLayers>)>,
) {
    let Some(entry) = registry.get(&mw_world::ids::exterior()) else { return };
    for e in &patches {
        commands.entity(e).try_insert(RenderLayers::layer(entry.layer));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_math::Vec2;
    use sf_coords::{WorldPos, WorldTransform};

    fn end(ws: &str, at: Vec3) -> LinkEnd {
        let frame = WorldTransform::from_pos(WorldPos::from_f64(at.as_dvec3()));
        LinkEnd { worldspace: WorldspaceId::new(ws), frame, arrival: frame, door: None, extent: Vec2::ZERO }
    }

    #[test]
    fn picks_the_nearest_door_in_front() {
        let links = vec![
            DoorLink { a: end("ext", Vec3::new(0.0, 0.0, -3.0)), b: end("hall", Vec3::ZERO), two_way: true },
            DoorLink { a: end("ext", Vec3::new(0.0, 0.0, 3.0)), b: end("shop", Vec3::ZERO), two_way: true },
            DoorLink { a: end("ext", Vec3::new(0.0, 0.0, -3.5)), b: end("inn", Vec3::ZERO), two_way: false },
        ];
        let here = Location::new(WorldspaceId::new("ext"), WorldPos::from_f64(glam::DVec3::ZERO));
        let (_, to) = pick(&links, &here, Some(Vec3::NEG_Z), REACH_M).unwrap();
        assert_eq!(to.worldspace, WorldspaceId::new("hall"), "nearest in front, not the one behind");
        let (_, to) = pick(&links, &here, Some(Vec3::Z), REACH_M).unwrap();
        assert_eq!(to.worldspace, WorldspaceId::new("shop"));
        assert!(pick(&links, &here, Some(Vec3::X), REACH_M).is_none(), "nothing to the side");
        // Inside, the hall's end leads back out.
        let inside = Location::new(WorldspaceId::new("hall"), WorldPos::from_f64(glam::DVec3::new(0.0, 0.0, 1.0)));
        let (_, out) = pick(&links, &inside, Some(Vec3::NEG_Z), REACH_M).unwrap();
        assert_eq!(out.worldspace, WorldspaceId::new("ext"));
        // One-way links only leave from `a`.
        let inn = Location::new(WorldspaceId::new("inn"), WorldPos::from_f64(glam::DVec3::ZERO));
        assert!(pick(&links, &inn, Some(Vec3::NEG_Z), REACH_M).is_none());
    }

    /// The facing at a DODT marker (Morrowind angles through `mw_world`'s
    /// conversion): Z angle 0 faces north, +90 degrees east, 180 south;
    /// X and Y angles (pitch, roll) never change the heading, and the
    /// camera arrives level.
    #[test]
    fn arrival_faces_the_markers_heading() {
        use std::f32::consts::{FRAC_PI_2, PI};
        let forward = |angles: [f32; 3]| {
            let marker = mw_world::convert::world_transform([0.0; 3], angles, 1.0);
            arrival_rotation(marker.rotation) * Vec3::NEG_Z
        };
        let north = Vec3::NEG_Z;
        let east = Vec3::X;
        let close = |a: Vec3, b: Vec3| (a - b).length() < 1e-4;
        assert!(close(forward([0.0, 0.0, 0.0]), north));
        assert!(close(forward([0.0, 0.0, FRAC_PI_2]), east));
        assert!(close(forward([0.0, 0.0, PI]), -north));
        assert!(close(forward([0.0, 0.0, -FRAC_PI_2]), -east));
        for (x, y) in [(0.4, 0.0), (0.0, 0.3), (-0.5, 0.2), (2.5, 0.0)] {
            let f = forward([x, y, FRAC_PI_2]);
            assert!(close(f, east), "pitch {x} roll {y}: {f}");
        }
        // Same heading as the marker's own forward when it is level.
        let marker = mw_world::convert::world_transform([0.0; 3], [0.0, 0.0, 1.0], 1.0);
        assert!(close(arrival_rotation(marker.rotation) * Vec3::NEG_Z, marker.rotation * Vec3::NEG_Z));
    }
}
