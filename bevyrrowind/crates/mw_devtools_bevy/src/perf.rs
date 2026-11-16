//! Performance measurement in the viewer: benchmark runs (`--bench`), the
//! window size they render at, A/B switches (`--off`), and the `perf`
//! subcommands' shared settings. Workflow: `docs/perf.md`; framework side:
//! `superfargoth/docs/specs/diag.md`. Moved from the app (app-split.md slice 12a); the
//! `perf` subcommands (suite runner, sweep, trace and Nsight summaries) stay in the app's `perf/`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use bevy_app::prelude::*;
use bevy_diagnostic::SystemInformationDiagnosticsPlugin;
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_light::DirectionalLight;
use bevy_mesh::Mesh;
use bevy_pbr::diagnostic::MaterialAllocatorDiagnosticPlugin;
use bevy_pbr::StandardMaterial;
use bevy_render::diagnostic::{MeshAllocatorDiagnosticPlugin, RenderDiagnosticsPlugin};
use bevy_render::renderer::RenderAdapterInfo;
use bevy_window::{MonitorSelection, PresentMode, PrimaryWindow, Window, WindowMode, WindowOccluded, WindowPosition, WindowResolution};
use bevy_winit::{UpdateMode, WinitSettings};
use bevy_world_serialization::WorldAsset;
use sf_diag_bevy::sf_diag::{Phase, Report, Scenario, unit};
use sf_diag_bevy::{AssetChurnPlugin, ChurnPlugin, DiagCounters, DiagScenario, DiagSettings, SfDiagPlugin};
use sf_world_bevy::{PlacementSpawner, PlacementSpawners, SpawnCx};
use sf_world::{IndexedPlacement, Level, PlacementClass};
// Moved to `mw_game_core` (app-split.md slice 1); the old paths keep working.
pub use mw_game_core::perf::{Off, Part};

/// Default benchmark window, physical pixels (the stored baselines).
pub const DEFAULT_RESOLUTION: (u32, u32) = (1280, 720);

/// Environment variables read as defaults for the flags of the same name (`BEVYRROWIND_OFF`: `mw_game_core::perf`).
const ENV_HITCH_MS: &str = "BEVYRROWIND_HITCH_MS";

/// Measurement flags shared by interiors and exteriors.
#[derive(clap::Args, Clone, Debug, Default)]
pub struct PerfFlags {
    /// Window size in physical pixels, `WxH` (benchmarks: `1280x720` unless set).
    #[arg(long, value_parser = parse_resolution)]
    pub resolution: Option<(u32, u32)>,
    /// A/B: leave parts out, comma-separated: terrain, near (R1 placements,
    /// alias statics), far (R2 placements), sky (atmosphere, sun and moon),
    /// envmap (sky environment light only), shadows, lights, particles, nif
    /// (models on the runtime NIF path), farfield (L1-L3 proxies, alias lod),
    /// denoiser (DLSS Ray Reconstruction on `--rt`).
    /// Env: `BEVYRROWIND_OFF`.
    #[arg(long, value_delimiter = ',')]
    pub off: Vec<String>,
    /// Benchmark reports go here (default `target/diag`).
    #[arg(long, requires = "bench")]
    pub bench_out: Option<PathBuf>,
    /// Baseline to compare against: a path, or `none`. Default
    /// `baselines/<file>` next to the scenario (1280x720) or
    /// `baselines/<W>x<H>/<file>` at other resolutions.
    #[arg(long, requires = "bench")]
    pub bench_baseline: Option<String>,
    /// Measure this many seconds instead of the scenario's duration (standing
    /// scenarios; a long window for Nsight captures).
    #[arg(long, requires = "bench")]
    pub bench_seconds: Option<f64>,
    /// Frames longer than this (ms) are hitches. Default 2.5 × the median
    /// frame, at least 25 ms. Env: `BEVYRROWIND_HITCH_MS`.
    #[arg(long)]
    pub hitch_ms: Option<f64>,
}

fn parse_resolution(raw: &str) -> Result<(u32, u32), String> {
    let (w, h) = raw.split_once(['x', 'X']).ok_or("expected WxH, e.g. 2560x1440")?;
    let w: u32 = w.trim().parse().map_err(|e| format!("{w:?}: {e}"))?;
    let h: u32 = h.trim().parse().map_err(|e| format!("{h:?}: {e}"))?;
    if w == 0 || h == 0 {
        return Err("resolution must be positive".into());
    }
    Ok((w, h))
}

/// Window and benchmark settings resolved from the flags.
pub struct Perf {
    pub off: Off,
    /// Physical window size; `None` keeps the viewer's default window.
    pub resolution: Option<(u32, u32)>,
    pub bench: Option<(Scenario, DiagSettings)>,
}

impl Perf {
    /// Resolves the flags. `meta` goes into the report (cell, build...).
    pub fn new(
        flags: &PerfFlags,
        bench: Option<&Path>,
        update_baseline: bool,
        mut meta: BTreeMap<String, String>,
        adjust: impl FnOnce(&mut Scenario),
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let off = Off::from_flags(&flags.off)?;
        let resolution = flags.resolution.or(bench.map(|_| DEFAULT_RESOLUTION));
        let bench = match bench {
            Some(path) => {
                let mut scenario = Scenario::from_json(&std::fs::read_to_string(path)?)?;
                adjust(&mut scenario);
                if let Some(seconds) = flags.bench_seconds {
                    match &mut scenario.measure {
                        sf_diag_bevy::sf_diag::Measure::Duration { seconds: s } => *s = seconds,
                        _ => return Err("--bench-seconds needs a duration scenario".into()),
                    }
                }
                let file = path.file_name().ok_or("scenario path has no file name")?;
                let baselines = path.parent().unwrap_or(Path::new(".")).join("baselines");
                let baseline = match flags.bench_baseline.as_deref() {
                    Some("none") => None,
                    Some(p) => Some(PathBuf::from(p)),
                    None => Some(match resolution {
                        Some(r) if r != DEFAULT_RESOLUTION => baselines.join(format!("{}x{}", r.0, r.1)).join(file),
                        _ => baselines.join(file),
                    }),
                };
                let hitch_ms = flags
                    .hitch_ms
                    .or_else(|| std::env::var(ENV_HITCH_MS).ok().and_then(|v| v.parse().ok()));
                meta.insert("build".to_owned(), build_label().to_owned());
                meta.insert("off".to_owned(), off.label());
                let settings = DiagSettings {
                    output_dir: flags.bench_out.clone().unwrap_or_else(|| PathBuf::from("target/diag")),
                    baseline,
                    update_baseline,
                    exit_on_finish: true,
                    hitch_ms,
                    meta,
                    ..DiagSettings::default()
                };
                Some((scenario, settings))
            }
            None => None,
        };
        Ok(Self { off, resolution, bench })
    }

    /// The primary window: the requested size at scale factor 1 (physical
    /// pixels, whatever the desktop scaling), vsync off for benchmarks.
    /// Interactive runs without `--resolution` open borderless fullscreen on
    /// the primary monitor (owner 2026-10-30); with it, a window of that size
    /// centred on the primary monitor.
    pub fn window(&self, title: String, default_size: Option<(u32, u32)>) -> Window {
        let mut window = Window { title, ..Window::default() };
        if let Some((w, h)) = self.resolution.or(default_size) {
            window.resolution = WindowResolution::new(w, h).with_scale_factor_override(1.0);
        }
        if self.bench.is_none() {
            if self.resolution.is_some() {
                window.position = WindowPosition::Centered(MonitorSelection::Primary);
            } else {
                window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
            }
        } else {
            window.present_mode = bench_present_mode();
            if std::env::var(ENV_WINDOW).is_ok_and(|v| v.eq_ignore_ascii_case("borderless")) {
                window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
            }
        }
        window
    }

    /// Adds the benchmark plugins (with a scenario) and the A/B systems.
    pub fn add_plugins(self, app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
        let poll = env_flag(std::env::var(ENV_POLL_BEFORE_SUBMIT).ok().as_deref());
        sf_diag_bevy::set_poll_before_submit(poll);
        if let Some((scenario, mut settings)) = self.bench {
            let gpu_timers = !env_flag(std::env::var(ENV_NO_GPU_TIMERS).ok().as_deref());
            if gpu_timers {
                app.add_plugins(RenderDiagnosticsPlugin);
            } else {
                settings.meta.insert(GPU_TIMERS_META.to_owned(), "off".to_owned());
            }
            if poll {
                settings.meta.insert(POLL_META.to_owned(), "on".to_owned());
            }
            app.add_plugins((
                MeshAllocatorDiagnosticPlugin,
                // StandardMaterial bindless slabs + their resource count (tex-slab, 2026-11-15).
                MaterialAllocatorDiagnosticPlugin::<StandardMaterial>::default(),
                SystemInformationDiagnosticsPlugin,
                SfDiagPlugin::new(settings).with_scenario(scenario)?,
                ChurnPlugin,
                sf_diag_bevy::FramePhasePlugin,
                sf_diag_bevy::RenderViewsPlugin,
                AssetChurnPlugin::<Mesh>::default(),
                AssetChurnPlugin::<Image>::default(),
                AssetChurnPlugin::<StandardMaterial>::default(),
                AssetChurnPlugin::<WorldAsset>::default(),
            ))
            // Bevy's default throttles an unfocused window to 60 Hz
            // (`WinitSettings::game`): a benchmark must run flat out
            // whether or not it has focus.
            .insert_resource(WinitSettings { focused_mode: UpdateMode::Continuous, unfocused_mode: UpdateMode::Continuous })
            .add_systems(Update, (record_meta, record_focus));
        }
        if self.off.has(Part::Shadows) {
            app.add_systems(PostUpdate, no_shadows);
        }
        if self.off.has(Part::Particles) {
            app.add_observer(no_particles);
        }
        app.insert_resource(self.off);
        Ok(())
    }
}

pub fn build_label() -> &'static str {
    if cfg!(debug_assertions) { "dev" } else { "release" }
}

/// Puts the GPU and the actual window size into the report's metadata
/// (the OS may refuse a window larger than the screen).
fn record_meta(
    adapter: Option<Res<RenderAdapterInfo>>,
    windows: Query<&Window>,
    settings: Option<ResMut<DiagSettings>>,
    mut last: Local<Option<(String, u32, u32)>>,
) {
    let (Some(mut settings), Ok(window)) = (settings, windows.single()) else { return };
    let gpu = adapter.map(|a| format!("{} ({:?}, {})", a.name, a.backend, a.driver_info)).unwrap_or_default();
    let now = (gpu, window.physical_width(), window.physical_height());
    if last.as_ref() == Some(&now) {
        return;
    }
    settings.meta.insert("gpu".to_owned(), now.0.clone());
    settings.meta.insert("resolution".to_owned(), format!("{}x{}", now.1, now.2));
    settings.meta.insert("present".to_owned(), format!("{:?}", window.present_mode));
    *last = Some(now);
}

/// Sum counter: 1 for each frame the primary window didn't have keyboard
/// focus (another app in front), else 0; its run total counts those frames.
pub const UNFOCUSED: &str = "window/unfocused";
/// Sum counter: 1 for each frame the primary window was hidden (occluded, or
/// minimized: a 0-pixel window), else 0.
pub const HIDDEN: &str = "window/hidden";
/// Report metadata key: [`FocusTally::label`] over the measured frames.
pub const FOCUS_META: &str = "focus";

/// Measured frames spent with the window out of the foreground. Windows
/// gives a background window less CPU (no foreground boost, power
/// throttling) and composites it instead of flipping it, so such frames
/// aren't comparable with the rest: a run with any is dirty.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FocusTally {
    pub frames: u64,
    pub unfocused: u64,
    pub hidden: u64,
}

impl FocusTally {
    /// Counts one measured frame.
    pub fn add(&mut self, focused: bool, hidden: bool) {
        self.frames += 1;
        self.unfocused += u64::from(!focused);
        self.hidden += u64::from(hidden);
    }

    pub fn is_dirty(&self) -> bool {
        self.unfocused > 0 || self.hidden > 0
    }

    /// From a report's [`UNFOCUSED`] / [`HIDDEN`] totals and frame count;
    /// `None` for a report without them (an exe older than the guard).
    pub fn from_report(report: &Report) -> Option<Self> {
        let total = |name: &str| report.counters.get(name).and_then(|c| c.total);
        let (unfocused, hidden) = (total(UNFOCUSED)?, total(HIDDEN)?);
        let frames = report.frames.as_ref().map_or(0, |f| f.stats.count);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "frame counts: whole, non-negative, far below 2^52")]
        Some(Self { frames: frames as u64, unfocused: unfocused.round() as u64, hidden: hidden.round() as u64 })
    }

    /// `clean`, or `DIRTY: ` and the share of measured frames unfocused and hidden.
    pub fn label(&self) -> String {
        if !self.is_dirty() {
            return "clean".to_owned();
        }
        #[allow(clippy::cast_precision_loss, reason = "frame counts far below 2^52")]
        let pct = |n: u64| if self.frames == 0 { 0.0 } else { 100.0 * n as f64 / self.frames as f64 };
        format!(
            "DIRTY: window unfocused {} of {} measured frames ({:.1}%), hidden {} ({:.1}%)",
            self.unfocused,
            self.frames,
            pct(self.unfocused),
            self.hidden,
            pct(self.hidden)
        )
    }
}

/// Counts the frames the primary window spent unfocused or hidden
/// ([`UNFOCUSED`], [`HIDDEN`]) and labels the measured ones in the report's
/// metadata ([`FOCUS_META`]). winit reports occlusion on some platforms only
/// (not Windows); there a minimized window shows as 0 × 0 pixels.
fn record_focus(
    windows: Query<(Entity, &Window), With<PrimaryWindow>>,
    mut occlusion: MessageReader<WindowOccluded>,
    counters: Res<DiagCounters>,
    scenario: Option<Res<DiagScenario>>,
    settings: Option<ResMut<DiagSettings>>,
    mut occluded: Local<bool>,
    mut tally: Local<FocusTally>,
) {
    let Ok((entity, window)) = windows.single() else { return };
    if let Some(m) = occlusion.read().filter(|m| m.window == entity).last() {
        *occluded = m.occluded;
    }
    let hidden = *occluded || window.physical_width() == 0 || window.physical_height() == 0;
    counters.add_named(UNFOCUSED, unit::COUNT, if window.focused { 0.0 } else { 1.0 });
    counters.add_named(HIDDEN, unit::COUNT, if hidden { 1.0 } else { 0.0 });
    let (Some(scenario), Some(mut settings)) = (scenario, settings) else { return };
    if scenario.phase() != Phase::Measure {
        return;
    }
    tally.add(window.focused, hidden);
    // A clean run writes its label once; a dirty one keeps the counts current.
    if tally.is_dirty() || !settings.meta.contains_key(FOCUS_META) {
        settings.meta.insert(FOCUS_META.to_owned(), tally.label());
    }
}

/// `--off shadows`: the sky turns the sun's shadow maps on in `Update`;
/// this turns them off again before extraction.
fn no_shadows(mut lights: Query<&mut DirectionalLight>) {
    for mut light in &mut lights {
        if light.shadow_maps_enabled {
            light.shadow_maps_enabled = false;
        }
    }
}

/// `--off particles`: NIF particle emitters never start.
fn no_particles(add: On<Add<mw_assets::ParticleEmitter>>, mut commands: Commands) {
    commands.entity(add.entity).try_remove::<mw_assets::ParticleEmitter>();
}

/// `--off near` / `--off far` / `--off nif`: spawns nothing for the
/// placements of the ring levels left out.
struct OffSpawner {
    inner: Arc<dyn PlacementSpawner>,
    near: bool,
    far: bool,
}

impl PlacementSpawner for OffSpawner {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        let skip = match cx.level {
            Level::Near => self.near,
            Level::Far => self.far,
            _ => false,
        };
        if skip { 0 } else { self.inner.spawn(world, cx, placement) }
    }

    fn needs_model(&self, placement: &IndexedPlacement) -> bool {
        self.inner.needs_model(placement)
    }
}

/// Applies the ring and NIF switches to the exterior's spawners.
pub fn apply_off(spawners: &mut PlacementSpawners, off: &Off) {
    if off.has(Part::Nif) {
        spawners.set_uncooked(None);
    }
    let (near, far) = (off.has(Part::Near), off.has(Part::Far));
    if !near && !far {
        return;
    }
    let wrap = |inner: &Arc<dyn PlacementSpawner>| -> Arc<dyn PlacementSpawner> { Arc::new(OffSpawner { inner: inner.clone(), near, far }) };
    for class in [PlacementClass::Static, PlacementClass::Dynamic, PlacementClass::Actor, PlacementClass::Light, PlacementClass::Door] {
        if let Some(s) = spawners.for_class(class).map(wrap) {
            spawners.set(class, s);
        }
    }
    let default = spawners.for_class(PlacementClass::Custom(u16::MAX)).map(wrap);
    let uncooked = spawners.uncooked().map(wrap);
    spawners.set_default(default).set_uncooked(uncooked);
}

/// Env var choosing a benchmark's present mode: `immediate`, `mailbox`, `fifo`
/// or `auto-novsync` (default). An A/B switch for frame pacing (perf.md).
pub const ENV_PRESENT: &str = "BEVYRROWIND_PRESENT";

fn bench_present_mode() -> PresentMode {
    match std::env::var(ENV_PRESENT).unwrap_or_default().to_ascii_lowercase().as_str() {
        "immediate" => PresentMode::Immediate,
        "mailbox" => PresentMode::Mailbox,
        "fifo" | "vsync" => PresentMode::Fifo,
        _ => PresentMode::AutoNoVsync,
    }
}


/// Env var `1`: poll the wgpu device just before the main submit (`sf_diag_bevy::set_poll_before_submit`),
/// so wgpu's deferred destruction is timed as `encode/poll` instead of inside `encode/submit`. A submit-cost
/// A/B (CW-5b, `docs/reviews/submit-cost-diagnosis-2026-11-16.md` §7); the work moves, it is not removed.
pub const ENV_POLL_BEFORE_SUBMIT: &str = "BEVYRROWIND_POLL_BEFORE_SUBMIT";

/// Env var `1`: a benchmark without `RenderDiagnosticsPlugin` (no GPU pass timers or their query
/// resolves), CPU gauges kept. Every `render/*/elapsed_gpu` / `elapsed_cpu` column is then empty and
/// the report's meta says `gpu_timers: off` (CW-5b: does the plugin cost submit time?).
pub const ENV_NO_GPU_TIMERS: &str = "BEVYRROWIND_NO_GPU_TIMERS";

/// Report metadata key, `off` when [`ENV_NO_GPU_TIMERS`] dropped the GPU timers.
pub const GPU_TIMERS_META: &str = "gpu_timers";
/// Report metadata key, `on` when [`ENV_POLL_BEFORE_SUBMIT`] is set.
pub const POLL_META: &str = "poll_before_submit";

/// An on/off env switch: `1`, `true`, `on` or `yes` (any case) is on; unset or anything else is off.
pub fn env_flag(value: Option<&str>) -> bool {
    value.is_some_and(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "on" | "yes"))
}

/// Env var choosing a benchmark's window: `borderless` (borderless
/// fullscreen on the primary monitor, at its resolution: a window that fits
/// the display) or anything else for a normal window of `--resolution`
/// (diag.md "Frame pacing": a window larger than the display adds
/// compositor present jitter).
pub const ENV_WINDOW: &str = "BEVYRROWIND_WINDOW";

/// Env var `WxH`: render the exterior camera into an offscreen image of
/// this size and show it scaled to the window. Separates pixel count from
/// the window's present path in a benchmark (perf.md "Frame pacing").
pub const ENV_RENDER_SIZE: &str = "BEVYRROWIND_RENDER_SIZE";

/// The offscreen render size from [`ENV_RENDER_SIZE`], if set and valid.
pub fn render_size() -> Option<(u32, u32)> {
    parse_resolution(&std::env::var(ENV_RENDER_SIZE).ok()?).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_flags_are_on_only_for_explicit_yes_values() {
        for on in ["1", "true", "ON", " yes "] {
            assert!(env_flag(Some(on)), "{on}");
        }
        for off in [None, Some(""), Some("0"), Some("off"), Some("false"), Some("2")] {
            assert!(!env_flag(off), "{off:?}");
        }
    }

    #[test]
    fn focus_tally_is_clean_only_with_every_frame_in_front() {
        let mut t = FocusTally::default();
        for _ in 0..3 {
            t.add(true, false);
        }
        assert!(!t.is_dirty());
        assert_eq!(t.label(), "clean");
        t.add(false, false);
        assert!(t.is_dirty());
        assert_eq!(t, FocusTally { frames: 4, unfocused: 1, hidden: 0 });
        assert_eq!(t.label(), "DIRTY: window unfocused 1 of 4 measured frames (25.0%), hidden 0 (0.0%)");
        // Hidden while focused (minimized as the active app) is dirty too.
        let mut h = FocusTally::default();
        h.add(true, true);
        assert!(h.is_dirty());
        assert_eq!(h.label(), "DIRTY: window unfocused 0 of 1 measured frames (0.0%), hidden 1 (100.0%)");
    }
}
