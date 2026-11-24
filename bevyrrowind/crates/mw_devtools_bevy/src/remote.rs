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
//! - `bevyrrowind/status`: scene, hour, calendar date and clock, A/B switches, camera as a
//!   `--camera` string, and whether loading has settled (poll it before a
//!   screenshot).
//! - `bevyrrowind/set`: any of switches, hour, whole days, clock, cloud preset, camera;
//!   answers with the new status.
//! - `bevyrrowind/console`: runs a dev-console line (as a script would: not echoed, not in
//!   the history) and answers with the console log's tail; a command's output shows on the
//!   next call.
//!
//! Everything else the A/B keys do is reachable with `brp_extras/send_keys`.
//! Workflow and rules: CLAUDE.md "Live inspection (BRP)".

use bevy_camera_controller::free_camera::{FreeCamera, FreeCameraState};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_transform::prelude::*;
use bevy_time::prelude::*;
use bevy_remote::{BrpError, BrpResult, RemotePlugin, error_codes};
use bevy_brp_extras::{AgentTool, AppAgentToolExt, BrpExtrasPlugin, PortDisplay};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sf_calendar::CalendarView;
use sf_coords::WorldPos;
use sf_sky_bevy::{GameClock, SkyConfig};
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_world_bevy::to_grid;

use crate::live_ab::{LiveAb, Switch, Switches};

/// Bevyrrowind's BRP port unless `BEVYRROWIND_BRP_PORT` says otherwise.
pub const DEFAULT_PORT: u16 = 15712;

const STATUS: &str = "bevyrrowind/status";
const SET: &str = "bevyrrowind/set";
const CONSOLE: &str = "bevyrrowind/console";

/// Adds BRP, the extras and the game's agent tools.
pub fn add(app: &mut App) {
    let port = std::env::var("BEVYRROWIND_BRP_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(DEFAULT_PORT);
    app.add_plugins(RemotePlugin::default().with_method_main(STATUS, status).with_method_main(SET, set).with_method_main(CONSOLE, console))
        .add_plugins(BrpExtrasPlugin::with_port(port).port_in_title(PortDisplay::Always))
        .register_agent_tool(
            AgentTool::new(
                "bevyrrowind_status",
                STATUS,
                "Scene, hour, calendar date and clock, A/B switches (null: not in this run), camera as a `--camera` \
                 string (world meters, eye then target), and `ready` (streaming and assets settled; \
                 null when unknown). Poll `ready` before a screenshot.",
            )
            .result_schema_for::<Status>(),
        )
        .register_agent_tool(
            AgentTool::new(
                "bevyrrowind_set",
                SET,
                "Set any of: A/B switches by name, hour (0-24), days (whole days to move the clock, keeping                  the hour; negative goes back, stopping at day 0), clock paused, time-lapse, cloud preset \
                 (exterior genus name, e.g. clear, fair, overcast, storm), camera (`--camera` string). \
                 Returns the new status.",
            )
            .params_schema_for::<Set>()
            .result_schema_for::<Status>(),
        )
        .register_agent_tool(
            AgentTool::new(
                "bevyrrowind_console",
                CONSOLE,
                "Run a dev-console line (optional) and return the last `tail` lines of the console log \
                 (default 40). The line runs on the next frame: call again without `line` to read its output.",
            )
            .params_schema_for::<ConsoleCall>()
            .result_schema_for::<ConsoleTail>(),
        );
}

/// `bevyrrowind/console`'s parameters.
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ConsoleCall {
    /// A console line to run (as `ConsoleSubmit::exec`: not echoed, not in the history).
    line: Option<String>,
    /// How many of the log's last lines to return (default 40).
    tail: Option<usize>,
}

/// `bevyrrowind/console`'s answer.
#[derive(Serialize, JsonSchema)]
struct ConsoleTail {
    /// The console log's last lines, oldest first (as they stood before `line` ran).
    lines: Vec<String>,
}

/// Console log lines returned when `tail` is absent.
// tunables: const (a reply size, not a trade-off)
const CONSOLE_TAIL: usize = 40;

fn console(
    In(params): In<Option<Value>>,
    log: Option<Res<sf_console_bevy::DevConsole>>,
    mut submit: MessageWriter<sf_console_bevy::ConsoleSubmit>,
) -> BrpResult {
    let params: ConsoleCall = serde_json::from_value(params.unwrap_or(Value::Null)).map_err(invalid)?;
    let Some(log) = log else { return Err(invalid("no dev console in this run")) };
    let all: Vec<String> = log.log().lines().map(|l| l.text.clone()).collect();
    let tail = params.tail.unwrap_or(CONSOLE_TAIL).min(all.len());
    let lines = all[all.len() - tail..].to_vec();
    if let Some(line) = params.line {
        submit.write(sf_console_bevy::ConsoleSubmit::exec(line));
    }
    to_value(&ConsoleTail { lines })
}

/// `bevyrrowind/status`'s answer.
#[derive(Serialize, JsonSchema)]
struct Status {
    /// The `scenes.ron` entry, if the run is a scene.
    scene: Option<String>,
    /// The scene's "what to look for".
    look_for: Option<String>,
    hour: Option<f32>,
    /// The calendar date and time (`day` in the console), e.g. `16 Last Seed 427 3E, 09:00`.
    date: Option<String>,
    /// `paused`, `running` or `time-lapse`.
    clock: Option<&'static str>,
    /// Switch name to on/off; null when the switch has nothing to act on here.
    switches: serde_json::Map<String, Value>,
    /// Eye and target in world meters, as `--camera` takes them.
    camera: Option<String>,
    /// Streaming, terrain, models and textures settled.
    ready: Option<bool>,
    /// The weather at the camera's climate: current, how far it has come in
    /// (clouds lead, fog follows), forced or not, the next change.
    weather: Option<mw_environment_bevy::weather::LiveWeather>,
}

/// `bevyrrowind/set`'s parameters; absent fields stay as they are.
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Set {
    /// Switch name (as in status) to on/off.
    #[serde(default)]
    switches: std::collections::BTreeMap<String, bool>,
    hour: Option<f64>,
    /// Whole days to move the clock, keeping the hour (negative goes back; before the epoch it
    /// stops at day 0). Applied before `hour`.
    days: Option<i64>,
    paused: Option<bool>,
    time_lapse: Option<bool>,
    /// Pin a cloud genus (clear, fair, cumulus, stratocumulus, altocumulus, altostratus,
    /// cirrus, overcast, storm) over the weather's clouds; "weather" unpins.
    clouds: Option<String>,
    /// Force a weather everywhere from now (clear, cloudy, foggy, overcast, rain,
    /// thunderstorm, ashstorm, blight, snow, blizzard, windy, or a mod's preset record id)
    /// over the climate's transition; "auto" glides back to the forecast.
    weather: Option<String>,
    /// With `weather`: only this region's climate (a `REGN` id, e.g. "Ashlands Region";
    /// to see a neighbour's weather from afar); "auto" returns just that region.
    region: Option<String>,
    /// With `weather`: the transition, game hours (0 skips it).
    transition_hours: Option<f32>,
    /// Clock speed, game seconds per real second (30 is Morrowind's; 120 shows a
    /// weather transition in ~10-25 s).
    time_scale: Option<f32>,
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
    ready: crate::ready::Readiness,
    interior: Option<Res<crate::ready::Loading>>,
    calendar: Option<Res<CalendarView>>,
) -> BrpResult {
    to_value(&report(&state, &s, &camera, &ready, interior.as_deref(), calendar.as_deref()))
}

#[allow(clippy::needless_pass_by_value)]
#[expect(clippy::too_many_arguments, reason = "a Bevy system's parameters")]
fn set(
    In(params): In<Option<Value>>,
    real: Res<Time<Real>>,
    mut state: ResMut<LiveAb>,
    mut s: Switches,
    mut camera: CameraQuery,
    ready: crate::ready::Readiness,
    interior: Option<Res<crate::ready::Loading>>,
    calendar: Option<Res<CalendarView>>,
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
        .map(|name| match name {
            "weather" | "auto" => Ok(None),
            _ => sf_atmos_bevy::sf_atmos::CloudPreset::parse(name).map(Some).ok_or_else(|| invalid(format!("unknown cloud genus {name:?}"))),
        })
        .transpose()?;
    let weather = params
        .weather
        .as_deref()
        .map(|name| if name.eq_ignore_ascii_case("auto") { Ok(None) } else { mw_environment_bevy::weather::parse_weather(name).map(Some).map_err(invalid) })
        .transpose()?;
    if params.transition_hours.is_some_and(|h| h.is_nan() || h < 0.0) {
        return Err(invalid("transition_hours: want >= 0"));
    }
    let eye_target = params.camera.as_deref().map(|c| parse_camera(c).ok_or_else(|| invalid(format!("camera {c:?}: want x,y,z,tx,ty,tz")))).transpose()?;

    for (sw, on) in switches {
        let note = sw.set(&mut state, &mut s, Some(on));
        state.note(note, now);
    }
    if let Some(genus) = clouds {
        let note = crate::live_ab::set_clouds(&mut s, genus);
        state.note(note, now);
    }
    if let Some(ty) = weather {
        let note = crate::live_ab::set_weather(&mut s, ty.as_ref(), params.transition_hours, params.region.as_deref());
        if !note.starts_with("weather ->") {
            return Err(invalid(note));
        }
        state.note(note, now);
    }
    if let (Some(time), Some(config)) = (s.time.as_mut(), s.config.as_ref()) {
        set_day_hour(&params, time, config, calendar.as_deref())?;
        if let Some(lapse) = params.time_lapse {
            time.time_scale = if lapse { 600.0 } else { 30.0 };
            if lapse {
                time.paused = false;
            }
        }
        if let Some(scale) = params.time_scale.filter(|s| *s > 0.0) {
            time.time_scale = scale;
            time.paused = false;
        }
        if let Some(paused) = params.paused {
            time.paused = paused;
        }
    } else if params.hour.is_some() || params.days.is_some() || params.paused.is_some() || params.time_lapse.is_some() || params.time_scale.is_some() {
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
    let mut out = report(&state, &s, &camera, &ready, interior.as_deref(), calendar.as_deref());
    if let Some(c) = params.camera {
        out.camera = Some(c);
    }
    out.ready = out.ready.map(|r| r && eye_target.is_none());
    to_value(&out)
}

/// `set`'s `days` then `hour`, on the calendar's day (the sky's without one).
fn set_day_hour(params: &Set, time: &mut GameClock, config: &SkyConfig, calendar: Option<&CalendarView>) -> Result<(), BrpError> {
    let day_ms = calendar.map_or_else(|| config.day_ms(), CalendarView::day_ms);
    if let Some(days) = params.days {
        crate::console::shift_days(time, days, day_ms).map_err(invalid)?;
    }
    if let Some(hour) = params.hour {
        time.set_hour(hour.rem_euclid(f64::from(config.0.calendar.hours_per_day)), day_ms);
    }
    Ok(())
}

fn report(
    state: &LiveAb,
    s: &Switches,
    camera: &CameraQuery,
    ready: &crate::ready::Readiness,
    interior: Option<&crate::ready::Loading>,
    calendar: Option<&CalendarView>,
) -> Status {
    let scene = mw_game_core::scenes::active();
    let (hour, clock) = match (s.time.as_ref(), s.config.as_ref()) {
        (Some(t), Some(c)) => (Some(c.sky_clock(t).hour(&c.0.calendar)), Some(crate::live_ab::clock_label(t))),
        _ => (None, None),
    };
    let switches = Switch::ALL.iter().map(|sw| (sw.name().to_owned(), sw.get(state, s).map_or(Value::Null, Value::Bool))).collect();
    let camera = camera.single().ok().map(|(local, global, cell, _)| crate::live_ab::camera_string(global, local, cell));
    Status {
        scene: scene.map(|s| s.name.clone()),
        look_for: scene.map(|s| s.look_for.clone()).filter(|l| !l.is_empty()),
        hour,
        date: s.time.as_ref().zip(calendar).map(|(t, view)| crate::console::date_label(view, t.now)),
        clock,
        switches,
        camera,
        ready: ready.settled().or_else(|| interior.map(crate::ready::Loading::ready)),
        weather: crate::live_ab::live_weather(s),
    }
}

fn parse_camera(s: &str) -> Option<(bevy_math::DVec3, bevy_math::DVec3)> {
    let v: Vec<f64> = s.split(',').map(|p| p.trim().parse().ok()).collect::<Option<_>>()?;
    (v.len() == 6).then(|| (bevy_math::DVec3::from_slice(&v[..3]), bevy_math::DVec3::from_slice(&v[3..])))
}

fn to_value(v: &impl Serialize) -> BrpResult {
    serde_json::to_value(v).map_err(BrpError::internal)
}

fn invalid(e: impl ToString) -> BrpError {
    BrpError { code: error_codes::INVALID_PARAMS, message: e.to_string(), data: None }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// C3c-2: `days` moves the clock by whole days keeping the hour (before `hour`), and stops at
    /// day 0 going back past the epoch.
    #[test]
    fn set_days_moves_the_clock_by_whole_days() {
        let view = crate::console::tests::tamrielic();
        let config = SkyConfig(sf_sky_bevy::sf_sky::SkySettings::default());
        let start = view.start_instant();
        let mut clock = GameClock::new(start, 30.0);
        let params = |v: Value| serde_json::from_value::<Set>(v).unwrap();
        set_day_hour(&params(serde_json::json!({ "days": 2 })), &mut clock, &config, Some(&view)).unwrap();
        assert_eq!(clock.now.0, start.0 + 2 * view.day_ms());
        assert_eq!(crate::console::date_label(&view, clock.now), "18 Last Seed 427 3E, 09:00");
        set_day_hour(&params(serde_json::json!({ "days": -1, "hour": 21.5 })), &mut clock, &config, Some(&view)).unwrap();
        assert_eq!(crate::console::date_label(&view, clock.now), "17 Last Seed 427 3E, 21:30");
        set_day_hour(&params(serde_json::json!({ "days": -5 })), &mut clock, &config, Some(&view)).unwrap();
        assert_eq!(crate::console::date_label(&view, clock.now), "16 Last Seed 427 3E, 21:30", "stops at day 0");
        assert!(serde_json::from_value::<Set>(serde_json::json!({ "days": 1.5 })).is_err(), "whole days only");
    }
}
