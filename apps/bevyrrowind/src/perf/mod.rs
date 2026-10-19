//! Performance measurement in the viewer: benchmark runs (`--bench`), the
//! window size they render at, A/B switches (`--off`), and the `perf`
//! subcommands (suite runner, trace and Nsight summaries). Workflow:
//! `docs/perf.md`; framework side: `superfargoth/docs/specs/diag.md`.

pub mod cli;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use bevy::diagnostic::SystemInformationDiagnosticsPlugin;
use bevy::light::DirectionalLight;
use bevy::prelude::*;
use bevy::render::diagnostic::{MeshAllocatorDiagnosticPlugin, RenderDiagnosticsPlugin};
use bevy::render::renderer::RenderAdapterInfo;
use bevy::window::{MonitorSelection, PresentMode, WindowMode, WindowResolution};
use bevy::winit::{UpdateMode, WinitSettings};
use bevy::world_serialization::WorldAsset;
use sf_diag_bevy::sf_diag::Scenario;
use sf_diag_bevy::{AssetChurnPlugin, ChurnPlugin, DiagSettings, SfDiagPlugin};
use sf_world_bevy::{PlacementSpawner, PlacementSpawners, SpawnCx};
use sf_world::{IndexedPlacement, Level, PlacementClass};

/// Default benchmark window, physical pixels (the stored baselines).
pub const DEFAULT_RESOLUTION: (u32, u32) = (1280, 720);

/// Environment variables read as defaults for the flags of the same name.
const ENV_OFF: &str = "BEVYRROWIND_OFF";
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

/// A part of the frame an A/B run can leave out.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Part {
    Terrain,
    Near,
    Far,
    Sky,
    Envmap,
    Shadows,
    Lights,
    Particles,
    Nif,
    FarField,
    Denoiser,
    FarShadows,
    ShadowTune,
    /// Soft contact-hardening sun shadows (shadows F5, Bevy PCSS).
    SoftShadows,
    /// Screen-space contact shadows (shadows F1).
    ContactShadows,
    /// Shadows F3 sun cascades (practical split to ~250 m); off = the A1 504 m layout.
    ShadowSplits,
    /// Cached point-light shadow maps (shadows F6 slice 2, `sf_shadow_cache_bevy`; parked: off anyway).
    ShadowCache,
    Clouds,
    /// Size-based draw distance (industry-audit.md J1).
    SizeCull,
    /// Actor placements (NPCs, creatures) as characters.
    Actors,
    /// NPCs walking (controller, avoidance, wander scaffold; `walkers.rs`).
    Walkers,
    /// Object-terrain blending (rocks, roots, foundations into the ground;
    /// superfargoth terrain.md "Object-terrain blending").
    TerrainBlend,
    /// Rigid-body physics (`physics.rs`, M4): no physics worlds, no colliders.
    Physics,
    /// Height fog, ground-fog banks and the far fog (weather.md "Fog", F2).
    Fog,
    /// The froxel volume near the camera (god rays, lit fog; weather.md "Fog", F3):
    /// off, the analytic layers fog everything (the Low preset).
    Froxels,
    /// Per-froxel sample rotation (weather.md "Temporal sampling"): off, one Halton
    /// offset for the whole volume, history read at the jittered point (the old
    /// pulsing fog, an A/B).
    FogDither,
    /// The froxels' temporal jitter: off, every froxel samples its centre (diagnosis).
    FogJitter,
    /// Fog on blended and additive NIF materials (`sf_atmos_bevy::FogTranslucentsPlugin`: they draw
    /// after the fog composite); off, they skip the fog (the old look, an A/B).
    TransFog,
    /// Foliage cutouts (`mw_nif::Cutout`): off, authored blend +
    /// "alpha > 0" surfaces draw blended as before (the old look, an A/B).
    LeafMask,
    /// Camera motion vectors for sky and cloud pixels (`sf_render_order_bevy::SkyMotion`, RO-7); off,
    /// they carry none and DLSS/TAA smear the sky on camera turns (the old look, an A/B).
    SkyMotion,
}

impl Part {
    const ALL: [(&'static str, Part); 39] = [
        ("terrain", Part::Terrain),
        ("near", Part::Near),
        ("statics", Part::Near),
        ("r1", Part::Near),
        ("far", Part::Far),
        ("r2", Part::Far),
        ("sky", Part::Sky),
        ("atmosphere", Part::Sky),
        ("envmap", Part::Envmap),
        ("shadows", Part::Shadows),
        ("lights", Part::Lights),
        ("particles", Part::Particles),
        ("nif", Part::Nif),
        ("farfield", Part::FarField),
        ("lod", Part::FarField),
        ("denoiser", Part::Denoiser),
        ("farshadows", Part::FarShadows),
        ("shadowtune", Part::ShadowTune),
        ("softshadows", Part::SoftShadows),
        ("contactshadows", Part::ContactShadows),
        ("shadowsplits", Part::ShadowSplits),
        ("shadowcache", Part::ShadowCache),

        ("clouds", Part::Clouds),
        ("sizecull", Part::SizeCull),
        ("actors", Part::Actors),
        ("npcs", Part::Actors),
        ("walkers", Part::Walkers),
        ("ai", Part::Walkers),
        ("terrainblend", Part::TerrainBlend),
        ("groundblend", Part::TerrainBlend),
        ("physics", Part::Physics),
        ("fog", Part::Fog),
        ("froxels", Part::Froxels),
        ("godrays", Part::Froxels),
        ("fogdither", Part::FogDither),
        ("fogjitter", Part::FogJitter),
        ("transfog", Part::TransFog),
        ("leafmask", Part::LeafMask),
        ("skymotion", Part::SkyMotion),
    ];

    fn name(self) -> &'static str {
        Self::ALL.iter().find(|(_, p)| *p == self).map_or("?", |(n, _)| n)
    }
}

/// The parts left out of this run.
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct Off(Vec<Part>);

impl Off {
    /// From `--off` and, when that is empty, `BEVYRROWIND_OFF`.
    pub fn from_flags(flags: &[String]) -> Result<Self, String> {
        let env = std::env::var(ENV_OFF).unwrap_or_default();
        let names: Vec<&str> = if flags.is_empty() {
            env.split(',').collect()
        } else {
            flags.iter().map(String::as_str).collect()
        };
        let mut parts = Vec::new();
        for name in names.into_iter().map(str::trim).filter(|n| !n.is_empty()) {
            let part = Part::ALL
                .iter()
                .find(|(n, _)| n.eq_ignore_ascii_case(name))
                .map(|(_, p)| *p)
                .ok_or_else(|| {
                    let known: Vec<&str> = Part::ALL.iter().map(|(n, _)| *n).collect();
                    format!("unknown --off part {name:?}; known: {}", known.join(", "))
                })?;
            if !parts.contains(&part) {
                parts.push(part);
            }
        }
        parts.sort();
        Ok(Self(parts))
    }

    pub fn has(&self, part: Part) -> bool {
        self.0.contains(&part)
    }

    /// `terrain,shadows`, or `none`.
    pub fn label(&self) -> String {
        if self.0.is_empty() {
            "none".to_owned()
        } else {
            self.0.iter().map(|p| p.name()).collect::<Vec<_>>().join(",")
        }
    }
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
    pub fn window(&self, title: String, default_size: Option<(u32, u32)>) -> Window {
        let mut window = Window { title, ..Window::default() };
        if let Some((w, h)) = self.resolution.or(default_size) {
            window.resolution = WindowResolution::new(w, h).with_scale_factor_override(1.0);
        }
        if self.bench.is_some() {
            window.present_mode = bench_present_mode();
            if std::env::var(ENV_WINDOW).is_ok_and(|v| v.eq_ignore_ascii_case("borderless")) {
                window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
            }
        }
        window
    }

    /// Adds the benchmark plugins (with a scenario) and the A/B systems.
    pub fn add_plugins(self, app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
        if let Some((scenario, settings)) = self.bench {
            app.add_plugins((
                RenderDiagnosticsPlugin,
                MeshAllocatorDiagnosticPlugin,
                SystemInformationDiagnosticsPlugin,
                SfDiagPlugin::new(settings).with_scenario(scenario)?,
                ChurnPlugin,
                sf_diag_bevy::FramePhasePlugin,
                AssetChurnPlugin::<Mesh>::default(),
                AssetChurnPlugin::<Image>::default(),
                AssetChurnPlugin::<StandardMaterial>::default(),
                AssetChurnPlugin::<WorldAsset>::default(),
            ))
            // Bevy's default throttles an unfocused window to 60 Hz
            // (`WinitSettings::game`): a benchmark must run flat out
            // whether or not it has focus.
            .insert_resource(WinitSettings { focused_mode: UpdateMode::Continuous, unfocused_mode: UpdateMode::Continuous })
            .add_systems(Update, record_meta);
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
