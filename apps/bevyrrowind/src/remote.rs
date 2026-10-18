//! Live inspection for agents: the Bevy Remote Protocol (BRP) on
//! 127.0.0.1, driven through the `bevy_brp_mcp` server (`remote` feature,
//! part of `dev`; never in release builds: the endpoint is unauthenticated).
//!
//! Interactive runs only (`view`, `scene`), next to the live A/B keys
//! ([`crate::live_ab`]). Port `BEVYRROWIND_BRP_PORT`, default
//! [`DEFAULT_PORT`] (Ovid uses Bevy's 15702 on the same machine); shown in
//! the window title. Besides Bevy's built-in methods (`world.query`,
//! `world.mutate_components`, ... on reflected types) and `bevy_brp_extras`
//! (screenshot, keys, mouse, frame-time diagnostics), the game publishes
//! these agent tools (`brp_list_agent_tools`, then `brp_execute`):
//!
//! - `bevyrrowind/status`: scene, hour and clock, A/B switches, camera as a
//!   `--camera` string, and whether loading has settled (poll it before a
//!   screenshot).
//! - `bevyrrowind/set`: any of switches, hour, clock, cloud preset, camera;
//!   answers with the new status.
//!
//! Everything else the A/B keys do is reachable with `brp_extras/send_keys`.
//! Workflow and rules: CLAUDE.md "Live inspection (BRP)".

use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraState};
use bevy::prelude::*;
use bevy::remote::{BrpError, BrpResult, RemotePlugin, error_codes};
use bevy_brp_extras::{AgentTool, AppAgentToolExt, BrpExtrasPlugin, PortDisplay};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sf_coords::WorldPos;
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_world_bevy::to_grid;

use crate::live_ab::{LiveAb, Switch, Switches};

/// Bevyrrowind's BRP port unless `BEVYRROWIND_BRP_PORT` says otherwise.
pub const DEFAULT_PORT: u16 = 15712;

const STATUS: &str = "bevyrrowind/status";
const SET: &str = "bevyrrowind/set";

/// Adds BRP, the extras and the game's agent tools.
pub fn add(app: &mut App) {
    let port = std::env::var("BEVYRROWIND_BRP_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(DEFAULT_PORT);
    app.add_plugins(RemotePlugin::default().with_method_main(STATUS, status).with_method_main(SET, set))
        .add_plugins(BrpExtrasPlugin::with_port(port).port_in_title(PortDisplay::Always))
        .register_agent_tool(
            AgentTool::new(
                "bevyrrowind_status",
                STATUS,
                "Scene, hour and clock, A/B switches (null: not in this run), camera as a `--camera` \
                 string (world meters, eye then target), and `ready` (streaming and assets settled; \
                 null when unknown). Poll `ready` before a screenshot.",
            )
            .result_schema_for::<Status>(),
        )
        .register_agent_tool(
            AgentTool::new(
                "bevyrrowind_set",
                SET,
                "Set any of: A/B switches by name, hour (0-24), clock paused, time-lapse, cloud preset \
                 (exterior genus name, e.g. clear, fair, overcast, storm), camera (`--camera` string). \
                 Returns the new status.",
            )
            .params_schema_for::<Set>()
            .result_schema_for::<Status>(),
        );
}

/// `bevyrrowind/status`'s answer.
#[derive(Serialize, JsonSchema)]
struct Status {
    /// The `scenes.ron` entry, if the run is a scene.
    scene: Option<String>,
    /// The scene's "what to look for".
    look_for: Option<String>,
    hour: Option<f32>,
    /// `paused`, `running` or `time-lapse`.
    clock: Option<&'static str>,
    /// Switch name to on/off; null when the switch has nothing to act on here.
    switches: serde_json::Map<String, Value>,
    /// Eye and target in world meters, as `--camera` takes them.
    camera: Option<String>,
    /// Streaming, terrain, models and textures settled.
    ready: Option<bool>,
}

/// `bevyrrowind/set`'s parameters; absent fields stay as they are.
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Set {
    /// Switch name (as in status) to on/off.
    #[serde(default)]
    switches: std::collections::BTreeMap<String, bool>,
    hour: Option<f64>,
    paused: Option<bool>,
    time_lapse: Option<bool>,
    /// Cloud genus name.
    clouds: Option<String>,
    /// `x,y,z,tx,ty,tz`, world meters.
    camera: Option<String>,
}

type CameraQuery<'w, 's> =
    Query<'w, 's, (&'static mut Transform, &'static GlobalTransform, Option<&'static mut CellCoord>, &'static mut FreeCameraState), With<FreeCamera>>;

fn status(
    In(_): In<Option<Value>>,
    state: Res<LiveAb>,
    s: Switches,
    camera: CameraQuery,
    ready: crate::exterior::Readiness,
    interior: Option<Res<crate::view::Loading>>,
) -> BrpResult {
    to_value(&report(&state, &s, &camera, &ready, interior.as_deref()))
}

#[allow(clippy::needless_pass_by_value)]
fn set(
    In(params): In<Option<Value>>,
    real: Res<Time<Real>>,
    mut state: ResMut<LiveAb>,
    mut s: Switches,
    mut camera: CameraQuery,
    ready: crate::exterior::Readiness,
    interior: Option<Res<crate::view::Loading>>,
) -> BrpResult {
    let now = real.elapsed_secs();
    let params: Set = serde_json::from_value(params.unwrap_or(Value::Null)).map_err(invalid)?;
    // Validate everything before changing anything.
    let switches = params
        .switches
        .iter()
        .map(|(name, on)| Switch::from_name(name).map(|sw| (sw, *on)).ok_or_else(|| invalid(format!("unknown switch {name:?}; known: {}", Switch::names()))))
        .collect::<Result<Vec<_>, _>>()?;
    let clouds = params
        .clouds
        .as_deref()
        .map(|name| crate::live_ab::cloud_choice(name).ok_or_else(|| invalid(format!("unknown cloud preset {name:?}"))))
        .transpose()?;
    let eye_target = params.camera.as_deref().map(|c| parse_camera(c).ok_or_else(|| invalid(format!("camera {c:?}: want x,y,z,tx,ty,tz")))).transpose()?;

    for (sw, on) in switches {
        let note = sw.set(&mut state, &mut s, Some(on));
        state.note(note, now);
    }
    if let Some(i) = clouds {
        let note = crate::live_ab::set_clouds(&mut s, i);
        state.note(note, now);
    }
    if let (Some(time), Some(config)) = (s.time.as_mut(), s.config.as_ref()) {
        let calendar = &config.0.calendar;
        if let Some(hour) = params.hour {
            time.0.set_hour(hour.rem_euclid(24.0), calendar);
        }
        if let Some(lapse) = params.time_lapse {
            time.0.time_scale = if lapse { 600.0 } else { 30.0 };
            if lapse {
                time.0.paused = false;
            }
        }
        if let Some(paused) = params.paused {
            time.0.paused = paused;
        }
    } else if params.hour.is_some() || params.paused.is_some() || params.time_lapse.is_some() {
        return Err(invalid("no sky clock in this run"));
    }
    if let Some((eye, target)) = eye_target {
        let Ok((mut transform, _, cell, mut free)) = camera.single_mut() else {
            return Err(BrpError::internal("no free camera in this run"));
        };
        let look = Transform::default().looking_to((target - eye).as_vec3(), Vec3::Y).rotation;
        match cell {
            Some(mut cell) => {
                let (c, t) = to_grid(WorldPos::from_f64(eye));
                *cell = c;
                transform.translation = t.translation;
            }
            None => transform.translation = eye.as_vec3(),
        }
        transform.rotation = look;
        // The controller keeps its own yaw and pitch; without this the next mouse move snaps back.
        let (yaw, pitch, _) = look.to_euler(EulerRot::YXZ);
        free.yaw = yaw;
        free.pitch = pitch;
    }
    // The camera's GlobalTransform updates next frame: report the requested pose.
    let mut out = report(&state, &s, &camera, &ready, interior.as_deref());
    if let Some(c) = params.camera {
        out.camera = Some(c);
    }
    out.ready = out.ready.map(|r| r && eye_target.is_none());
    to_value(&out)
}

fn report(
    state: &LiveAb,
    s: &Switches,
    camera: &CameraQuery,
    ready: &crate::exterior::Readiness,
    interior: Option<&crate::view::Loading>,
) -> Status {
    let scene = crate::scenes::active();
    let (hour, clock) = match (s.time.as_ref(), s.config.as_ref()) {
        (Some(t), Some(c)) => (Some(t.0.hour(&c.0.calendar)), Some(crate::live_ab::clock_label(&t.0))),
        _ => (None, None),
    };
    let switches = Switch::ALL.iter().map(|sw| (sw.name().to_owned(), sw.get(state, s).map_or(Value::Null, Value::Bool))).collect();
    let camera = camera.single().ok().map(|(local, global, cell, _)| crate::live_ab::camera_string(global, local, cell));
    Status {
        scene: scene.map(|s| s.name.clone()),
        look_for: scene.map(|s| s.look_for.clone()).filter(|l| !l.is_empty()),
        hour,
        clock,
        switches,
        camera,
        ready: ready.settled().or_else(|| interior.map(crate::view::Loading::ready)),
    }
}

fn parse_camera(s: &str) -> Option<(bevy::math::DVec3, bevy::math::DVec3)> {
    let v: Vec<f64> = s.split(',').map(|p| p.trim().parse().ok()).collect::<Option<_>>()?;
    (v.len() == 6).then(|| (bevy::math::DVec3::from_slice(&v[..3]), bevy::math::DVec3::from_slice(&v[3..])))
}

fn to_value(v: &impl Serialize) -> BrpResult {
    serde_json::to_value(v).map_err(BrpError::internal)
}

fn invalid(e: impl ToString) -> BrpError {
    BrpError { code: error_codes::INVALID_PARAMS, message: e.to_string(), data: None }
}
