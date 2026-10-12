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
//! F5 far-field tint, F6 terrain LOD colors, F7 terrain wireframe,
//! F8 streaming rings, F11 object-terrain blend, `[` / `]` hour -/+ 0.5 (Shift: 0.1), P pause clock,
//! T time-lapse, F9 copy camera (`--camera` string), F10 clouds on/off,
//! C cycle cloud preset (clear, fair, overcast, storm), F12 screenshot,
//! B smooth ↔ vanilla character bodies (handled in `actors.rs`).

use std::fmt::Write as _;

use bevy::camera_controller::free_camera::FreeCamera;
use bevy::light::DirectionalLight;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use sf_coords::WorldPos;
use sf_sky_bevy::sf_sky::{UpscaleQuality, Upscaling};
use sf_sky_bevy::{ActiveUpscaling, SkyConfig, SkyTime, UpscalingSetting};
use sf_world_bevy::big_space::prelude::CellCoord;

/// Adds the keys and overlay when the run is interactive.
pub fn add(app: &mut App, interactive: bool) {
    if interactive {
        app.add_plugins(LiveAbPlugin);
    }
}

pub struct LiveAbPlugin;

impl Plugin for LiveAbPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LiveAb>()
            .add_systems(Startup, (spawn_overlay, pause_scene_clock))
            .add_systems(Update, (keys, overlay).chain())
            .add_systems(PostUpdate, sun_shadows_off);
    }
}

/// Switch state that has no framework resource of its own.
#[derive(Resource)]
struct LiveAb {
    help: bool,
    sun_shadows: bool,
    /// Last action, shown for a few seconds.
    note: Option<(String, f32)>,
}

impl Default for LiveAb {
    fn default() -> Self {
        Self { help: true, sun_shadows: true, note: None }
    }
}

#[derive(Component)]
struct OverlayText;

/// DLSS modes F4 cycles through.
/// Presets the C key cycles through.
const CLOUD_CYCLE: [sf_atmos_bevy::sf_atmos::CloudPreset; 4] = [
    sf_atmos_bevy::sf_atmos::CloudPreset::Clear,
    sf_atmos_bevy::sf_atmos::CloudPreset::Fair,
    sf_atmos_bevy::sf_atmos::CloudPreset::Overcast,
    sf_atmos_bevy::sf_atmos::CloudPreset::Storm,
];

const DLSS_CYCLE: [Upscaling; 5] = [
    Upscaling::Off,
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
    if let (Some(mut time), Some(_)) = (time, crate::scenes::active()) {
        time.0.paused = true;
    }
}

/// The debug resources the keys flip (each optional: interiors have fewer).
#[derive(bevy::ecs::system::SystemParam)]
struct Switches<'w> {
    terrain: Option<ResMut<'w, sf_terrain_bevy::TerrainDebug>>,
    far: Option<ResMut<'w, sf_lod_bevy::FarFieldDebug>>,
    rings: Option<ResMut<'w, sf_world_bevy::StreamingDebug>>,
    upscaling: Option<ResMut<'w, UpscalingSetting>>,
    time: Option<ResMut<'w, SkyTime>>,
    config: Option<Res<'w, SkyConfig>>,
    clouds: Option<ResMut<'w, sf_atmos_bevy::Clouds>>,
    /// Characters: smooth or vanilla bodies (B, `actors.rs`).
    bodies: Option<Res<'w, sf_character_bevy::CharacterSettings>>,
    /// Seamless portals (`--portals`): the exposure strength when on.
    portals: Option<Res<'w, crate::portal_spike::ExposureStrength>>,
}

fn on_off(v: bool) -> &'static str {
    if v { "on" } else { "off" }
}

#[allow(clippy::too_many_lines, clippy::too_many_arguments)]
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
    let mut note = |text: String| {
        println!("[A/B] {text}");
        state.note = Some((text, now));
    };
    if input.just_pressed(KeyCode::F3) {
        if let Some(t) = s.terrain.as_mut() {
            t.no_far_shadows = !t.no_far_shadows;
            note(format!("terrain far shadows {}", on_off(!t.no_far_shadows)));
        } else {
            note("terrain far shadows: no terrain here".to_owned());
        }
    }
    if input.just_pressed(KeyCode::F11) {
        if let Some(t) = s.terrain.as_mut() {
            t.no_ground_blend = !t.no_ground_blend;
            note(format!("object-terrain blend {}", on_off(!t.no_ground_blend)));
        } else {
            note("object-terrain blend: no terrain here".to_owned());
        }
    }
    if input.just_pressed(KeyCode::F4) {
        if let Some(u) = s.upscaling.as_mut() {
            let i = DLSS_CYCLE.iter().position(|m| *m == u.0).map_or(0, |i| (i + 1) % DLSS_CYCLE.len());
            u.0 = DLSS_CYCLE[i];
            let built = if cfg!(feature = "dlss") { "" } else { " (this exe has no dlss feature: stays off)" };
            note(format!("DLSS requested {}{built}", u.0.label()));
        } else {
            note("DLSS: start with --dlss <mode> in interiors to make it switchable".to_owned());
        }
    }
    if input.just_pressed(KeyCode::F10) {
        if let Some(c) = s.clouds.as_mut() {
            c.enabled = !c.enabled;
            note(format!("clouds {}", on_off(c.enabled)));
        } else {
            note("clouds: not in this run (interior or --off clouds)".to_owned());
        }
    }
    if input.just_pressed(KeyCode::KeyC)
        && let Some(c) = s.clouds.as_mut()
    {
        *cloud_preset = (*cloud_preset + 1) % CLOUD_CYCLE.len();
        let preset = CLOUD_CYCLE[*cloud_preset];
        c.settings = crate::exterior::cloud_settings(preset);
        note(format!("clouds preset {}", preset.name()));
    }
    if input.just_pressed(KeyCode::F5) {
        if let Some(f) = s.far.as_mut() {
            f.level_tint = !f.level_tint;
            note(format!("far-field tint {}", on_off(f.level_tint)));
        } else {
            note("far-field tint: no far field here".to_owned());
        }
    }
    if input.just_pressed(KeyCode::F6)
        && let Some(t) = s.terrain.as_mut()
    {
        t.lod_colors = !t.lod_colors;
        note(format!("terrain LOD colors {}", on_off(t.lod_colors)));
    }
    if input.just_pressed(KeyCode::F7)
        && let Some(t) = s.terrain.as_mut()
    {
        t.wireframe = !t.wireframe;
        note(format!("terrain wireframe {}", on_off(t.wireframe)));
    }
    if input.just_pressed(KeyCode::F8)
        && let Some(r) = s.rings.as_mut()
    {
        r.enabled = !r.enabled;
        note(format!("streaming rings {}", on_off(r.enabled)));
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
            note(format!("hour {:.2}", time.0.hour(calendar)));
        }
        if input.just_pressed(KeyCode::KeyP) {
            time.0.paused = !time.0.paused;
            note(format!("clock {}", if time.0.paused { "paused" } else { "running" }));
        }
        if input.just_pressed(KeyCode::KeyT) {
            time.0.time_scale = if time.0.time_scale < 100.0 { 600.0 } else { 30.0 };
            time.0.paused = false;
            note(format!("time-lapse {} (x{})", on_off(time.0.time_scale > 100.0), time.0.time_scale));
        }
    }
    if input.just_pressed(KeyCode::F9)
        && let Ok((global, local, cell)) = camera.single()
    {
        // Exteriors: grid cell + local offset to world meters; interiors: plain transform.
        let eye = match cell {
            Some(c) => WorldPos::new(IVec3::new(c.x, c.y, c.z), local.translation).to_f64(),
            None => global.translation().as_dvec3(),
        };
        let target = eye + global.forward().as_dvec3() * 50.0;
        let raw = format!("{:.1},{:.1},{:.1},{:.1},{:.1},{:.1}", eye.x, eye.y, eye.z, target.x, target.y, target.z);
        let hour = match (s.time.as_ref(), s.config.as_ref()) {
            (Some(t), Some(c)) => format!(" --hour {:.2}", t.0.hour(&c.0.calendar)),
            _ => String::new(),
        };
        let line = format!("--camera \"{raw}\"{hour}");
        let copied = clipboard.and_then(|mut c| c.set_text(line.clone()).ok()).is_some();
        note(format!("{line}{}", if copied { "  (copied)" } else { "" }));
    }
    if input.just_pressed(KeyCode::F12) {
        let name = crate::scenes::active().map_or("view", |s| s.name.as_str());
        let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
        let path = std::path::PathBuf::from("shots").join(format!("{name}-{secs}.png"));
        let _ = std::fs::create_dir_all("shots");
        note(format!("screenshot {}", path.display()));
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
    }
    // Last: `note` borrows `state` until its final use above.
    if input.just_pressed(KeyCode::F1) {
        state.help = !state.help;
    }
    if input.just_pressed(KeyCode::F2) {
        state.sun_shadows = !state.sun_shadows;
        let text = format!("sun shadows {}", on_off(state.sun_shadows));
        println!("[A/B] {text}");
        state.note = Some((text, now));
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

#[allow(clippy::needless_pass_by_value)]
fn overlay(
    real: Res<Time<Real>>,
    state: Res<LiveAb>,
    s: Switches,
    active: Option<Res<ActiveUpscaling>>,
    off: Option<Res<crate::perf::Off>>,
    mut text: Query<&mut Text, With<OverlayText>>,
    mut last: Local<String>,
) {
    let Ok(mut text) = text.single_mut() else { return };
    let scene = crate::scenes::active();
    let title = scene.map_or_else(|| "view".to_owned(), |s| format!("scene {}", s.name));
    let mut out = format!("{title}  |  F1 {}", if state.help { "hide" } else { "help" });
    if let (Some(t), Some(c)) = (s.time.as_ref(), s.config.as_ref()) {
        let clock = if t.0.paused { "paused" } else if t.0.time_scale > 100.0 { "time-lapse" } else { "running" };
        let _ = write!(out, "  |  {:.2} h ({clock})", t.0.hour(&c.0.calendar));
    }
    if state.help {
        let opt = |v: Option<bool>| v.map_or("n/a", on_off);
        let dlss = match (s.upscaling.as_ref(), active.as_ref()) {
            (Some(u), Some(a)) if u.0 == a.0 => u.0.label().to_owned(),
            (Some(u), Some(a)) => format!("{} (running {})", u.0.label(), a.0.label()),
            _ => "n/a".to_owned(),
        };
        let terrain = s.terrain.as_ref();
        let flags_off = off.map_or_else(|| "none".to_owned(), |o| o.label());
        let bodies = s.bodies.as_ref().map_or_else(|| "n/a".to_owned(), |b| format!("{:?}", b.bodies).to_lowercase());
        let _ = write!(
            out,
            "\nF2 sun shadows {}  F3 terrain far shadows {}  F4 DLSS {dlss}\n\
             F5 far tint {}  F6 terrain LOD colors {}  F7 wireframe {}  F8 rings {}\n\
             F10 clouds {}  F11 ground blend {}  C cloud preset  [ ] hour -/+0.5 (Shift 0.1)  P pause  T time-lapse  F9 copy camera  F12 screenshot  B bodies {bodies}  |  --off {flags_off}",
            on_off(state.sun_shadows),
            opt(terrain.map(|t| !t.no_far_shadows)),
            opt(s.far.as_ref().map(|f| f.level_tint)),
            opt(terrain.map(|t| t.lod_colors)),
            opt(terrain.map(|t| t.wireframe)),
            opt(s.rings.as_ref().map(|r| r.enabled)),
            opt(s.clouds.as_ref().map(|c| c.enabled)),
            opt(terrain.map(|t| !t.no_ground_blend)),
        );
        match s.portals.as_ref() {
            Some(p) => {
                let _ = write!(out, "\nseamless portals on (--portals): - = exposure strength {:.1}", p.0);
            }
            None => out.push_str("\nseamless portals off (default; --portals or BEVYRROWIND_PORTALS=1)"),
        }
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
