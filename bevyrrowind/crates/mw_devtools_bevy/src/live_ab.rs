//! Runtime A/B keys and a one-line overlay for the interactive viewer
//! (`view` and `scene`; never in screenshot or benchmark runs), so the owner
//! can flip a feature while looking at it (`docs/visual-checks.md`).
//!
//! Only switches that are cheap to flip live are here: each one writes a
//! settings resource the framework already reads every frame. The rest
//! (far field, terrain, sky, near/far rings, LOD tint) are decided at
//! startup: use the `--off`/`--lod-tint` flags and compare two runs.
//!
//! Keys: F1 help, F2 sun shadows, F3 terrain far shadows, F4 DLSS mode,
//! Shift+F6 far-field tint (F5 / F9 are quicksave / quickload, `saves.rs`), F6 terrain LOD colors, F7 wireframe (terrain and skinned meshes; G shows the physics colliders, ragdoll capsules and joints, `physics.rs`),
//! F8 streaming rings, `[` / `]` hour -/+ 0.5 (Shift: 0.1), P pause clock,
//! T clock speed (x1, x4, x20), Shift+F12 copy camera (`--camera` string), Shift+C clouds on/off (also console
//! `flip clouds`; F10 is the keybinds screen, `controls.rs`),
//! C next weather (forced everywhere with its transition), F12 screenshot,
//! B smooth ↔ vanilla character bodies (handled in `actors.rs`).
//! Agents reach the same switches over BRP ([`crate::remote`], `remote` feature), the
//! dev console (backquote) as `on`/`off`/`flip <name>` ([`crate::console`]).

use std::fmt::Write as _;

use bevy_camera_controller::free_camera::FreeCamera;
use bevy_light::DirectionalLight;
use bevy_mesh::morph::MeshMorphWeights;
use bevy_mesh::skinning::SkinnedMesh;
use bevy_pbr::wireframe::{Wireframe, WireframeColor, WireframePlugin};
use bevy_app::prelude::*;
use bevy_clipboard::prelude::*;
use bevy_ui::prelude::*;
use bevy_text::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_transform::prelude::*;
use bevy_time::prelude::*;
use bevy_input::prelude::*;
use bevy_color::prelude::*;
use bevy_render::view::screenshot::{Screenshot, save_to_disk};
use sf_coords::WorldPos;
use sf_sky_bevy::sf_sky::{SkyClock, UpscaleQuality, Upscaling};
use sf_sky_bevy::{ActiveUpscaling, SkyConfig, SkyTime, UpscalingSetting};
use sf_world_bevy::big_space::prelude::CellCoord;

/// Adds the keys and overlay (and BRP, with the `remote` feature) when the run is interactive.
pub fn add(app: &mut App, interactive: bool) {
    if interactive {
        app.add_plugins((LiveAbPlugin, crate::console::ConsoleWiring));
        #[cfg(feature = "remote")]
        crate::remote::add(app);
    }
}

pub struct LiveAbPlugin;

impl Plugin for LiveAbPlugin {
    fn build(&self, app: &mut App) {
        // F7 on skinned meshes (is the mesh following the ragdoll?).
        if !app.is_plugin_added::<WireframePlugin>() {
            app.add_plugins(WireframePlugin::default());
        }
        app.init_resource::<LiveAb>()
            .add_systems(Startup, (spawn_overlay, pause_scene_clock))
            .add_systems(Update, (keys, overlay, skinned_wireframe).chain())
            .add_systems(PostUpdate, sun_shadows_off);
    }
}

/// Switch state that has no framework resource of its own.
#[derive(Resource)]
pub struct LiveAb {
    help: bool,
    sun_shadows: bool,
    /// F7: wireframe over the skinned meshes (the terrain has its own flag).
    mesh_wire: bool,
    /// Last action, shown for a few seconds.
    note: Option<(String, f32)>,
}

impl Default for LiveAb {
    fn default() -> Self {
        Self { help: true, sun_shadows: true, mesh_wire: false, note: None }
    }
}

impl LiveAb {
    /// Logs an action and shows it in the overlay for a few seconds.
    pub fn note(&mut self, text: String, now: f32) {
        println!("[A/B] {text}");
        self.note = Some((text, now));
    }
}

#[derive(Component)]
struct OverlayText;

/// Temporal resolve modes F4 cycles through (`off` = MSAA).
const DLSS_CYCLE: [Upscaling; 6] = [
    Upscaling::Off,
    Upscaling::Taa,
    Upscaling::Dlss(UpscaleQuality::Quality),
    Upscaling::Dlss(UpscaleQuality::Balanced),
    Upscaling::Dlss(UpscaleQuality::Performance),
    Upscaling::Dlss(UpscaleQuality::Native),
];

fn spawn_overlay(mut commands: Commands) {
    commands.spawn((
        OverlayText,
        Text::new(""),
        TextFont::from_font_size(14.0),
        TextColor(Color::srgb(0.95, 0.95, 0.85)),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(6.0),
            left: Val::Px(6.0),
            padding: UiRect::all(Val::Px(6.0)),
            ..Node::default()
        },
    ));
}

/// A scene is checked at its hour: its clock starts paused (P resumes).
fn pause_scene_clock(time: Option<ResMut<SkyTime>>) {
    if let (Some(mut time), Some(_)) = (time, mw_game_core::scenes::active()) {
        time.0.paused = true;
    }
}

/// The debug resources the keys flip (each optional: interiors have fewer).
#[derive(bevy_ecs::system::SystemParam)]
pub struct Switches<'w> {
    terrain: Option<ResMut<'w, sf_terrain_bevy::TerrainDebug>>,
    far: Option<ResMut<'w, sf_lod_bevy::FarFieldDebug>>,
    rings: Option<ResMut<'w, sf_world_bevy::StreamingDebug>>,
    upscaling: Option<ResMut<'w, UpscalingSetting>>,
    pub time: Option<ResMut<'w, SkyTime>>,
    pub config: Option<Res<'w, SkyConfig>>,
    clouds: Option<ResMut<'w, sf_atmos_bevy::Clouds>>,
    /// The live weather (C key, BRP `weather`): the world state, records, the grid's
    /// climates, the camera's site and the pins.
    weather: Option<ResMut<'w, sf_weather_bevy::WorldWeather>>,
    weather_records: Option<Res<'w, sf_weather_bevy::WeatherRecords>>,
    climates: Option<Res<'w, sf_weather_bevy::WorldWeatherMap>>,
    #[cfg_attr(not(feature = "remote"), allow(dead_code, reason = "read by BRP `bevyrrowind/status` only"))]
    site: Option<Res<'w, sf_weather_bevy::WeatherSite>>,
    #[cfg_attr(not(feature = "remote"), allow(dead_code, reason = "BRP `clouds` only"))]
    weather_overrides: Option<ResMut<'w, sf_weather_bevy::WeatherOverrides>>,
    /// Characters: smooth or vanilla bodies (B, `actors.rs`).
    bodies: Option<Res<'w, sf_character_bevy::CharacterSettings>>,
}

/// An on/off switch: its key toggles it, BRP sets it by name.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Switch {
    SunShadows,
    TerrainFarShadows,
    Clouds,
    FarTint,
    TerrainLodColors,
    Wireframe,
    Rings,
}

/// Names for BRP ([`crate::remote`]) and the console's `on`/`off` ([`crate::console`]).
impl Switch {
    pub const ALL: [Self; 7] = [
        Self::SunShadows,
        Self::TerrainFarShadows,
        Self::Clouds,
        Self::FarTint,
        Self::TerrainLodColors,
        Self::Wireframe,
        Self::Rings,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            Self::SunShadows => "sun_shadows",
            Self::TerrainFarShadows => "terrain_far_shadows",
            Self::Clouds => "clouds",
            Self::FarTint => "far_tint",
            Self::TerrainLodColors => "terrain_lod_colors",
            Self::Wireframe => "wireframe",
            Self::Rings => "rings",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|s| s.name() == name)
    }

    pub fn names() -> String {
        Self::ALL.map(Self::name).join(", ")
    }

    const fn label(self) -> &'static str {
        match self {
            Self::SunShadows => "sun shadows",
            Self::TerrainFarShadows => "terrain far shadows",
            Self::Clouds => "clouds",
            Self::FarTint => "far-field tint",
            Self::TerrainLodColors => "terrain LOD colors",
            Self::Wireframe => "wireframe",
            Self::Rings => "streaming rings",
        }
    }


    /// On or off; `None` when this run has nothing for it to act on.
    pub fn get(self, state: &LiveAb, s: &Switches) -> Option<bool> {
        let terrain = s.terrain.as_ref();
        match self {
            Self::SunShadows => Some(state.sun_shadows),
            Self::TerrainFarShadows => terrain.map(|t| !t.no_far_shadows),
            Self::Clouds => s.clouds.as_ref().map(|c| c.enabled),
            Self::FarTint => s.far.as_ref().map(|f| f.level_tint),
            Self::TerrainLodColors => terrain.map(|t| t.lod_colors),
            Self::Wireframe => Some(state.mesh_wire),
            Self::Rings => s.rings.as_ref().map(|r| r.enabled),
        }
    }

    /// Sets it (`None`: toggles) and says what happened.
    pub fn set(self, state: &mut LiveAb, s: &mut Switches, on: Option<bool>) -> String {
        let Some(current) = self.get(state, s) else {
            let why = match self {
                Self::Clouds => "not in this run (interior or --off clouds)",
                Self::FarTint => "no far field here",
                Self::Rings => "no streaming here",
                _ => "no terrain here",
            };
            return format!("{}: {why}", self.label());
        };
        let v = on.unwrap_or(!current);
        match self {
            Self::SunShadows => state.sun_shadows = v,
            Self::Wireframe => {
                state.mesh_wire = v;
                if let Some(t) = s.terrain.as_mut() {
                    t.wireframe = v;
                }
            }
            Self::Clouds => {
                if let Some(c) = s.clouds.as_mut() {
                    c.enabled = v;
                }
            }
            Self::FarTint => {
                if let Some(f) = s.far.as_mut() {
                    f.level_tint = v;
                }
            }
            Self::Rings => {
                if let Some(r) = s.rings.as_mut() {
                    r.enabled = v;
                }
            }
            Self::TerrainFarShadows | Self::TerrainLodColors => {
                if let Some(t) = s.terrain.as_mut() {
                    match self {
                        Self::TerrainFarShadows => t.no_far_shadows = !v,
                        _ => t.lod_colors = v,
                    }
                }
            }
        }
        format!("{} {}", self.label(), on_off(v))
    }
}

/// Pins a cloud genus over the weather's clouds (`None`: the weather's).
#[cfg(feature = "remote")]
pub fn set_clouds(s: &mut Switches, genus: Option<sf_atmos_bevy::sf_atmos::CloudPreset>) -> String {
    let Some(o) = s.weather_overrides.as_mut() else {
        return "clouds: no weather in this run (interior)".to_owned();
    };
    o.clouds = genus.map(mw_game_core::exterior::pinned_clouds);
    genus.map_or_else(|| "clouds follow the weather".to_owned(), |g| format!("clouds pinned {}", g.name()))
}

/// Game hours since the clock's epoch (the weather's time).
fn weather_now(s: &Switches) -> Option<f64> {
    let (time, config) = (s.time.as_ref()?, s.config.as_ref()?);
    Some(time.0.days * f64::from(config.0.calendar.hours_per_day))
}

/// Forces a weather from now over `transition_hours` (`None`: the climate's;
/// 0 skips), or with `None` glides back to the forecast: every climate, or
/// only `region`'s (a `REGN` id or name, e.g. "Ashlands Region"), for distant weather.
pub fn set_weather(s: &mut Switches, ty: Option<mw_weather::WeatherType>, transition_hours: Option<f32>, region: Option<&str>) -> String {
    let Some(now) = weather_now(s) else {
        return "weather: no clock in this run".to_owned();
    };
    let (Some(world), Some(records), Some(map)) = (s.weather.as_mut(), s.weather_records.as_ref(), s.climates.as_ref()) else {
        return "weather: not in this run (interior or no weather records)".to_owned();
    };
    let climates = match region {
        None => map.climates().to_vec(),
        Some(r) => match records.0.climate(&mw_weather::climate_id(r)) {
            Some(c) => vec![c],
            None => return format!("weather: unknown region {r:?}"),
        },
    };
    mw_environment_bevy::weather::force_live(&records.0, &mut world.0, &climates, ty, now, transition_hours);
    let name = ty.map_or("auto (forecast)", mw_weather::WeatherType::name);
    region.map_or_else(|| format!("weather -> {name}"), |r| format!("weather -> {name} in {r}"))
}

/// The weather at the camera's climate.
#[cfg(feature = "remote")]
pub fn live_weather(s: &Switches) -> Option<mw_environment_bevy::weather::LiveWeather> {
    let now = weather_now(s)?;
    let climate = s.site.as_ref()?.climates.first()?.0;
    mw_environment_bevy::weather::live_weather(&s.weather_records.as_ref()?.0, &s.weather.as_ref()?.0, climate, now)
}

/// The T key's clock speeds: normal (Morrowind's 30x), x4 (a weather
/// transition in ~10-25 s), x20 (a day in ~2.4 min).
pub const TIME_SCALES: [f32; 3] = [30.0, 120.0, 600.0];

pub fn clock_label(clock: &SkyClock) -> &'static str {
    if clock.paused {
        "paused"
    } else if clock.time_scale > 100.0 {
        "time-lapse"
    } else {
        "running"
    }
}

/// The camera as a `--camera` string: eye and a target 50 m ahead, world meters.
pub fn camera_string(global: &GlobalTransform, local: &Transform, cell: Option<&CellCoord>) -> String {
    // Exteriors: grid cell + local offset to world meters; interiors: plain transform.
    let eye = match cell {
        Some(c) => WorldPos::new(IVec3::new(c.x, c.y, c.z), local.translation).to_f64(),
        None => global.translation().as_dvec3(),
    };
    let target = eye + global.forward().as_dvec3() * 50.0;
    format!("{:.1},{:.1},{:.1},{:.1},{:.1},{:.1}", eye.x, eye.y, eye.z, target.x, target.y, target.z)
}

fn on_off(v: bool) -> &'static str {
    if v { "on" } else { "off" }
}

/// Keys that toggle a [`Switch`] (Shift+F6: far-field tint, in [`keys`]).
const SWITCH_KEYS: [(KeyCode, Switch); 5] = [
    (KeyCode::F2, Switch::SunShadows),
    (KeyCode::F3, Switch::TerrainFarShadows),
    (KeyCode::F6, Switch::TerrainLodColors),
    (KeyCode::F7, Switch::Wireframe),
    (KeyCode::F8, Switch::Rings),
];

#[allow(clippy::too_many_arguments)]
fn keys(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    real: Res<Time<Real>>,
    mut state: ResMut<LiveAb>,
    mut s: Switches,
    camera: Query<(&GlobalTransform, &Transform, Option<&CellCoord>), With<FreeCamera>>,
    clipboard: Option<ResMut<Clipboard>>,
    mut cloud_preset: Local<usize>,
) {
    let now = real.elapsed_secs();
    let shifted = input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    for (key, switch) in SWITCH_KEYS {
        if input.just_pressed(key) && !(shifted && key == KeyCode::F6) {
            let text = switch.set(&mut state, &mut s, None);
            state.note(text, now);
        }
    }
    if shifted && input.just_pressed(KeyCode::F6) {
        let text = Switch::FarTint.set(&mut state, &mut s, None);
        state.note(text, now);
    }
    if input.just_pressed(KeyCode::F4) {
        let text = if let Some(u) = s.upscaling.as_mut() {
            let i = DLSS_CYCLE.iter().position(|m| *m == u.0).map_or(0, |i| (i + 1) % DLSS_CYCLE.len());
            u.0 = DLSS_CYCLE[i];
            let built = if cfg!(feature = "dlss") || !matches!(u.0, Upscaling::Dlss(_)) { "" } else { " (this exe has no dlss feature: TAA)" };
            format!("temporal resolve requested {}{built}", u.0.label())
        } else {
            "temporal resolve: start with --dlss <mode> in interiors to make it switchable".to_owned()
        };
        state.note(text, now);
    }
    let shift = input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    if input.just_pressed(KeyCode::KeyC) && shift {
        let text = Switch::Clouds.set(&mut state, &mut s, None);
        state.note(text, now);
    } else if input.just_pressed(KeyCode::KeyC) && s.weather.is_some() {
        // The Morrowind weathers in `REGN` order, each over the climate's transition.
        *cloud_preset = (*cloud_preset + 1) % mw_weather::WeatherType::ALL.len();
        let text = set_weather(&mut s, Some(mw_weather::WeatherType::ALL[*cloud_preset]), None, None);
        state.note(text, now);
    }
    if let (Some(time), Some(config)) = (s.time.as_mut(), s.config.as_ref()) {
        let calendar = &config.0.calendar;
        let step = if input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) { 0.1 } else { 0.5 };
        let delta = if input.just_pressed(KeyCode::BracketRight) {
            step
        } else if input.just_pressed(KeyCode::BracketLeft) {
            -step
        } else {
            0.0
        };
        if delta != 0.0 {
            let hour = f64::from(time.0.hour(calendar)) + delta;
            time.0.set_hour(hour, calendar);
            state.note(format!("hour {:.2}", time.0.hour(calendar)), now);
        }
        if input.just_pressed(KeyCode::KeyP) {
            time.0.paused = !time.0.paused;
            state.note(format!("clock {}", if time.0.paused { "paused" } else { "running" }), now);
        }
        if input.just_pressed(KeyCode::KeyT) {
            let i = TIME_SCALES.iter().position(|s| *s >= time.0.time_scale).map_or(0, |i| (i + 1) % TIME_SCALES.len());
            time.0.time_scale = TIME_SCALES[i];
            time.0.paused = false;
            state.note(format!("clock x{} (game time x{})", time.0.time_scale / TIME_SCALES[0], time.0.time_scale), now);
        }
    }
    if shifted && input.just_pressed(KeyCode::F12)
        && let Ok((global, local, cell)) = camera.single()
    {
        let raw = camera_string(global, local, cell);
        let hour = match (s.time.as_ref(), s.config.as_ref()) {
            (Some(t), Some(c)) => format!(" --hour {:.2}", t.0.hour(&c.0.calendar)),
            _ => String::new(),
        };
        let line = format!("--camera \"{raw}\"{hour}");
        let copied = clipboard.and_then(|mut c| c.set_text(line.clone()).ok()).is_some();
        state.note(format!("{line}{}", if copied { "  (copied)" } else { "" }), now);
    }
    if !shifted && input.just_pressed(KeyCode::F12) {
        let name = mw_game_core::scenes::active().map_or("view", |s| s.name.as_str());
        let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
        let path = std::path::PathBuf::from("shots").join(format!("{name}-{secs}.png"));
        let _ = std::fs::create_dir_all("shots");
        state.note(format!("screenshot {}", path.display()), now);
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
    }
    if input.just_pressed(KeyCode::F1) {
        state.help = !state.help;
    }
}

/// F7: wireframe over every skinned mesh (characters), so the mesh can be judged against the
/// physics capsules G draws (PH-2). Meshes dressed later pick it up on the next frame.
/// Meshes with morph targets (heads) are skipped: Bevy 0.20's wireframe pipeline has no morph bind
/// group layout, so drawing them is a validation error that quits the app.
#[allow(clippy::type_complexity)]
fn skinned_wireframe(
    state: Res<LiveAb>,
    mut commands: Commands,
    plain: Query<Entity, (With<SkinnedMesh>, Without<Wireframe>, Without<MeshMorphWeights>)>,
    wired: Query<Entity, (With<SkinnedMesh>, With<Wireframe>)>,
) {
    if state.mesh_wire {
        for e in &plain {
            commands.entity(e).try_insert((Wireframe, WireframeColor { color: Color::srgb(0.95, 0.95, 0.2) }));
        }
    } else {
        for e in &wired {
            commands.entity(e).try_remove::<(Wireframe, WireframeColor)>();
        }
    }
}

/// F2 off: the sky re-enables the sun's shadow maps in `Update` each frame
/// it wants them; this turns them off again before extraction.
fn sun_shadows_off(state: Res<LiveAb>, mut lights: Query<&mut DirectionalLight>) {
    if state.sun_shadows {
        return;
    }
    for mut light in &mut lights {
        if light.shadow_maps_enabled {
            light.shadow_maps_enabled = false;
        }
    }
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn overlay(
    real: Res<Time<Real>>,
    state: Res<LiveAb>,
    s: Switches,
    active: Option<Res<ActiveUpscaling>>,
    off: Option<Res<mw_game_core::perf::Off>>,
    colliders: Option<Res<sf_physics_bevy::PhysicsDebug>>,
    mut text: Query<&mut Text, With<OverlayText>>,
    mut last: Local<String>,
) {
    let Ok(mut text) = text.single_mut() else { return };
    let scene = mw_game_core::scenes::active();
    let title = scene.map_or_else(|| "view".to_owned(), |s| format!("scene {}", s.name));
    let mut out = format!("{title}  |  F1 {}", if state.help { "hide" } else { "help" });
    if let (Some(t), Some(c)) = (s.time.as_ref(), s.config.as_ref()) {
        let _ = write!(out, "  |  {:.2} h ({})", t.0.hour(&c.0.calendar), clock_label(&t.0));
    }
    if state.help {
        let opt = |sw: Switch| sw.get(&state, &s).map_or("n/a", on_off);
        let dlss = match (s.upscaling.as_ref(), active.as_ref()) {
            (Some(u), Some(a)) if u.0 == a.0 => u.0.label().to_owned(),
            (Some(u), Some(a)) => format!("{} (running {})", u.0.label(), a.0.label()),
            _ => "n/a".to_owned(),
        };
        let colliders = colliders.map_or("n/a", |d| on_off(d.enabled));
        let flags_off = off.map_or_else(|| "none".to_owned(), |o| o.label());
        let bodies = s.bodies.as_ref().map_or_else(|| "n/a".to_owned(), |b| format!("{:?}", b.bodies).to_lowercase());
        let _ = write!(
            out,
            "\nF2 sun shadows {}  F3 terrain far shadows {}  F4 DLSS {dlss}\n\
             Shift+F6 far tint {}  F6 terrain LOD colors {}  F7 wireframe (terrain, skinned meshes) {}  F8 rings {}\n\
             G physics colliders (ragdoll capsules, joints) {colliders}\n\
             F11 player / free camera (player: WASD, Space jump, E use, Shift sprint, Ctrl sneak, Tab 1st/3rd person, wheel zoom)\n\
             Shift+C clouds {}  C next weather  [ ] hour -/+0.5 (Shift 0.1)  P pause  T clock x1/x4/x20  Shift+F12 copy camera  F12 screenshot  F5/F9 quicksave/load  B bodies {bodies}  |  --off {flags_off}",
            opt(Switch::SunShadows),
            opt(Switch::TerrainFarShadows),
            opt(Switch::FarTint),
            opt(Switch::TerrainLodColors),
            opt(Switch::Wireframe),
            opt(Switch::Rings),
            opt(Switch::Clouds),
        );
        if let Some(look) = scene.map(|s| s.look_for.as_str()).filter(|l| !l.is_empty()) {
            let _ = write!(out, "\nlook for: {look}");
        }
    }
    if let Some((note, at)) = &state.note
        && real.elapsed_secs() - at < 4.0
    {
        let _ = write!(out, "\n> {note}");
    }
    if *last != out {
        text.0.clone_from(&out);
        *last = out;
    }
}
