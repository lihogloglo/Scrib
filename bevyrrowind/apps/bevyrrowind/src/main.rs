//! Bevyrrowind command line.
//!
//! Today: inspection tools over the user's Morrowind install. Later, running
//! without a subcommand will launch the game.

mod level;
mod mods;
mod perf;
mod scenes;
mod sweep;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use mw_boot_bevy::bake::cache_root;
use mw_boot_bevy::{load, mount};
use mw_devtools_bevy::perf::{Off, Part, Perf, PerfFlags};
use mw_environment_bevy::render::{RenderSettings, parse_sun_cascades};
use mw_environment_bevy::water::{WaterInputs, WaterMode};
use mw_environment_bevy::weather::{WeatherCommand, WeatherSetup, parse_weather_type};
use mw_exterior_bevy::exterior::{ExteriorOptions, Quality};
use mw_install::{MorrowindInstall, Origin};
use mw_records::KindVisitor;
use mw_viewers_bevy::view::{Baked, SkinnedView, ViewOptions};
use sf_records::{RecordDb, RecordId};
use sf_vfs::{DirEntry, Vfs, VfsPath};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Morrowind install folder (or its Data Files). Overrides detection,
    /// including the `BEVYRROWIND_MORROWIND` environment variable.
    #[arg(long, global = true)]
    morrowind: Option<PathBuf>,
    /// Mod profile to load (`<user data>/profiles/<name>.toml`; `default`
    /// falls back to the install's own content when the file doesn't exist).
    #[arg(long, global = true, default_value = mw_mods::DEFAULT_PROFILE)]
    profile: String,
    /// User data folder holding `mods/` and `profiles/` (default: the
    /// `BEVYRROWIND_USER_DATA` environment variable, else `<local data dir>/bevyrrowind`).
    #[arg(long, global = true)]
    user_data: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant, reason = "parsed once at startup; the view flags are the big variant")]
enum Command {
    /// Show where detection looks and what it finds.
    Detect,
    /// Mount the VFS, load all content, and print a summary.
    Info {
        /// Conflicts to list per record kind.
        #[arg(long, default_value_t = 5)]
        conflicts: usize,
    },
    /// Print one record and the files that touched it.
    Record { kind: String, id: String },
    /// List record IDs of a kind, optionally containing a substring.
    Find {
        kind: String,
        pattern: Option<String>,
        #[arg(long, default_value_t = 50)]
        limit: usize,
    },
    /// List a VFS directory.
    Ls { dir: Option<String> },
    /// List cells that place a base record (most uses first).
    Uses {
        id: String,
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
    /// Copy a file out of the VFS.
    Extract { path: String, out: PathBuf },
    /// Show which sources provide a file, winner last.
    Which { path: String },
    /// Print a NIF model's converted node tree and materials.
    Nif { path: String },
    /// Print a KF (or animated NIF) file's animation groups, sections, events
    /// and root travel (`mw_anim::report`).
    Anim {
        /// VFS path, e.g. `meshes/xbase_anim.kf` (`meshes/` is optional).
        path: String,
        /// One line per text-key event, not only per group.
        #[arg(long)]
        events: bool,
        /// Only this group (case-insensitive).
        #[arg(long)]
        group: Option<String>,
    },
    /// Convert every NIF in the VFS; report failures and unsupported features.
    NifSweep {
        /// Only NIFs whose path starts with this (e.g. "meshes/l/").
        #[arg(long, default_value = "")]
        prefix: String,
        /// Example files listed per unsupported type.
        #[arg(long, default_value_t = 3)]
        examples: usize,
        /// Also list fully supported types, with examples for every type.
        #[arg(long)]
        all: bool,
    },
    /// Weather records: forecasts per region (`weather.rs`).
    Weather {
        #[command(subcommand)]
        command: WeatherCommand,
    },
    /// Cook every model the records reference (meshes + LODs) into the bake cache.
    Bake {
        /// Cache root (default: `<local data dir>/bevyrrowind/bake`).
        #[arg(long)]
        cache: Option<PathBuf>,
    },
    /// Mods and profiles: list, load order, report, enable/disable (`mods.md`).
    Mods {
        #[command(subcommand)]
        command: mods::ModsCommand,
    },
    /// Benchmarks: the suite runner, trace and Nsight summaries (`docs/perf.md`).
    Perf {
        #[command(subcommand)]
        command: perf::cli::PerfCommand,
    },
    /// Open a named test scene (`scenes.ron`, `docs/visual-checks.md`) in the
    /// interactive viewer; without a name (or with `--list`), list them.
    Scene {
        /// Scene name (or a unique prefix).
        name: Option<String>,
        /// List the scenes.
        #[arg(long)]
        list: bool,
        /// Scene registry to read instead of `scenes.ron` (cwd, then next to the exe, then built in).
        #[arg(long)]
        file: Option<PathBuf>,
        /// More `view` flags after the scene's own, e.g. `--off farfield`, `--dlss quality`.
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        extra: Vec<String>,
    },
    /// Render a cell (interior name, or exterior grid as "x,y").
    // A repeated flag replaces the earlier one: `scene <name> --camera ...`
    // overrides the scene's own camera instead of a clap error.
    #[command(args_override_self = true)]
    View {
        /// Interior name, exterior grid "x,y", or with `--model` a NIF path.
        /// Hyphen values allowed so negative grids like "-4,-3" aren't read as flags.
        #[arg(allow_hyphen_values = true)]
        cell: String,
        /// Show a single model (at the origin) instead of a cell.
        #[arg(long)]
        model: bool,
        #[command(flatten)]
        flags: ViewFlags,
    },
    /// A lineup of dressed, idling actors (NPC_ / CREA ids; default: the M3 lineup).
    /// `id:naked` drops equipment, `@race,class[,f]` picks the first matching NPC.
    Character {
        ids: Vec<String>,
        /// Save a PNG once every character is dressed, then exit.
        #[arg(long)]
        screenshot: Option<PathBuf>,
        #[arg(long, default_value_t = 2.0)]
        settle: f32,
        /// Eye and target, "ex,ey,ez,tx,ty,tz" (meters).
        #[arg(long)]
        camera: Option<String>,
        #[arg(long)]
        cache: Option<PathBuf>,
        /// `vanilla` (default) or `smooth` bodies; B toggles live. Smooth is opt-in (owner 2026-10-22).
        #[arg(long, default_value = "vanilla", value_parser = parse_bodies)]
        bodies: sf_character_bevy::SmoothBodies,
        /// Robes and skirts: `authored` (default) or `robes` (simulated, opt-in).
        #[arg(long, default_value = "authored")]
        cloth: mw_physics::ClothMode,
        /// Walk in place from the start (1 toggles, 2 runs).
        #[arg(long)]
        walk: bool,
        /// Weapons drawn from the start (R toggles).
        #[arg(long)]
        drawn: bool,
        /// Night from the start: dark stage, torches lit (N toggles).
        #[arg(long)]
        night: bool,
    },
}

#[derive(clap::Args)]
#[expect(clippy::struct_excessive_bools, reason = "independent command-line flags")]
struct ViewFlags {
    /// Save a screenshot here once loaded, then exit.
    #[arg(long)]
    screenshot: Option<PathBuf>,
    /// Take the screenshot even if the readiness gate times out (CPU loading or
    /// GPU pipelines still pending); by default a timeout logs an error and exits 1.
    #[arg(long)]
    screenshot_anyway: bool,
    /// Seconds to render after loading before the screenshot.
    #[arg(long, default_value_t = 2.0)]
    settle: f32,
    /// Camera eye and target in meters: "ex,ey,ez,tx,ty,tz". In exteriors a
    /// height written "+h" is h meters above the ground (or the sea).
    #[arg(long, allow_hyphen_values = true)]
    camera: Option<String>,
    /// Exteriors: pin a cloud genus (clear, fair, cumulus, stratocumulus, altocumulus,
    /// altostratus, cirrus, overcast, storm) over the weather's clouds (their wind stays the
    /// weather's). Default: the weather's. `--off clouds` drops them.
    #[arg(long, value_parser = parse_clouds)]
    clouds: Option<sf_atmos_bevy::sf_atmos::CloudPreset>,
    /// Exteriors: force a Morrowind weather everywhere (clear, cloudy, foggy, overcast, rain,
    /// thunderstorm, ashstorm, blight, snow, blizzard) instead of the regions' forecast
    /// (M6 WE3, `sf_weather_bevy`).
    #[arg(long, value_parser = parse_weather_type)]
    weather: Option<mw_weather::WeatherType>,
    /// Exteriors: fixed aerosol haze (`sf_sky::Weather::haze`; 1 is a pristine Earth sky)
    /// instead of the weather's, without the morning curve (weather.md N1).
    #[arg(long)]
    haze: Option<f32>,
    /// Exteriors: fixed dust/ash density (0 none, ~6 an ash storm) instead of the weather's
    /// (weather.md N2).
    #[arg(long)]
    dust: Option<f32>,
    /// Exteriors: multiplier on the weather's fog (height fog and ground-fog banks;
    /// 0 none, 2 twice as dense), weather.md "Fog". `--off fog` drops the fog terms.
    #[arg(long)]
    fog: Option<f32>,
    /// Exteriors: the sea (M6 W2): `on` (FFT ocean), `scaffold` (old flat sea, A/B), `off`.
    #[arg(long, value_enum)]
    water: Option<WaterMode>,
    /// Exteriors: 10 m wind speed for the ocean, m/s (scaffold until `sf_wind`; default 7).
    #[arg(long)]
    water_wind: Option<f32>,
    /// Exteriors: time of day in hours.
    #[arg(long, default_value_t = 10.0)]
    hour: f64,
    /// Exteriors: game day (days since the sky's epoch; moon phases advance one step every 3 days).
    #[arg(long, default_value_t = 172.0)]
    day: f64,
    /// Exteriors: drawn sun disk diameter in degrees (Morrowind default 2.5; physical 0.53).
    #[arg(long)]
    sun_size: Option<f32>,
    /// Exteriors: quality preset: low, medium, high (default), ultra: view-distance bias
    /// (LOD, size culling, far-field handoff) and the size-culling cutoff (env `BEVYRROWIND_QUALITY`).
    #[arg(long, value_parser = parse_quality)]
    quality: Option<Quality>,
    /// Exteriors: color terrain patches by LOD.
    #[arg(long)]
    terrain_lod: bool,
    /// Exteriors: overlay the terrain's triangle edges.
    #[arg(long)]
    wire: bool,
    /// Exteriors: draw the streaming rings and resident cells.
    #[arg(long)]
    rings: bool,
    /// Exteriors and streamed interiors: draw the physics colliders around the camera (`G` toggles).
    #[arg(long)]
    physics_debug: bool,
    /// Exteriors: the benchmark path's heights are above the ground (or sea).
    #[arg(long, requires = "bench")]
    bench_ground: bool,
    /// Exteriors: R1 (near ring) load radius in meters, for calibration runs.
    #[arg(long)]
    near_ring: Option<f32>,
    /// Exteriors: R2 (far ring) load radius in meters, for calibration runs.
    #[arg(long)]
    far_ring: Option<f32>,
    /// Exteriors: spawn no point lights (calibration runs).
    #[arg(long)]
    no_lights: bool,
    /// Exteriors: tint far-field proxies by level (L1 green, L2 blue, L3 purple).
    #[arg(long)]
    far_tint: bool,
    /// Exteriors: depth + motion-vector prepass with occlusion culling (A/B
    /// experiment). The default has none: a depth-only prepass ignores the
    /// LOD crossfade dither in Bevy 0.20; motion vectors give it the fragment
    /// stage that applies it. `--dlss` adds the same prepass.
    #[arg(long)]
    prepass: bool,
    /// Exteriors: once the start has streamed in, walk through the nearest
    /// door in front of the camera (as pressing Space, the activate key);
    /// `--screenshot` then captures the other side. `--use-door 2` comes back out.
    #[arg(long, num_args = 0..=1, default_missing_value = "1")]
    use_door: Option<u32>,
    /// Exteriors and streamed interiors: once placements have streamed in,
    /// activate the placement under the crosshair N times, one a second (as
    /// pressing Space; mod scripts receive `activated`, M5 L2).
    #[arg(long, num_args = 0..=1, default_missing_value = "1")]
    activate: Option<u32>,
    /// Dev: open a UI screen at start for `--screenshot` checks of the theme (`journal`, or
    /// `message` for a sample message box, `inventory` with a few dev sample items). Interactive
    /// runs open them with J / I / scripts.
    #[arg(long)]
    open_ui: Option<String>,
    /// Characters: `vanilla` (default, owner 2026-10-22; segmented parts as authored) or `smooth`
    /// (opt-in: seams welded, weights blended across joints). B toggles live.
    #[arg(long, default_value = "vanilla", value_parser = parse_bodies)]
    bodies: sf_character_bevy::SmoothBodies,
    /// Robes and skirts: `authored` (default: their authored skin, as vanilla)
    /// or `robes` (simulated cloth, the opt-in; physics-adapter.md "Cloth").
    #[arg(long, default_value = "authored")]
    cloth: mw_physics::ClothMode,
    /// Interiors: the single-cell viewer (one cell, no streaming, no doors)
    /// instead of the streamed view. `--rt`, `--no-baked` and `--skinned`
    /// choose it too: the streamed view doesn't support them yet.
    #[arg(long)]
    single_cell: bool,
    #[command(flatten)]
    lighting: Box<Lighting>,
    #[command(flatten)]
    lab: Box<LabFlags>,
    /// Load every model from its NIF, ignoring cooked meshes.
    #[arg(long)]
    no_baked: bool,
    /// Bake cache root (default: `<local data dir>/bevyrrowind/bake`).
    #[arg(long)]
    cache: Option<PathBuf>,
    /// Tint cooked models by LOD level (green LOD0, yellow, orange, red, magenta).
    #[arg(long)]
    lod_tint: bool,
    /// Load skinned models (actors, creatures, body parts) from their cooked
    /// `Cooked::Skinned` product, bound by joint name to the NIF's skeleton
    /// (scaffold until actor spawning; keys: K cooked/NIF, M cycle a morph).
    #[arg(long)]
    skinned: bool,
    /// With `--skinned`: the NIF whose nodes are the skeleton (default: the
    /// model itself; body parts use `meshes/base_anim.nif`).
    #[arg(long)]
    skeleton: Option<String>,
    /// With `--skinned`: play the baked animation graph on an `sf_anim_bevy`
    /// rig instead of binding to the NIF skeleton (keys: see `anim_view`).
    #[arg(long, requires = "skinned")]
    animate: bool,
    /// Ray-traced lighting (Bevy Solari; build with `--features solari`).
    /// Falls back to raster on GPUs without ray tracing.
    #[arg(long)]
    rt: bool,
    /// Temporal resolve: `taa` (Bevy TAA, native resolution), DLSS (build with
    /// `--features dlss`; NVIDIA RTX, TAA elsewhere): auto, native (alias dlaa),
    /// quality, balanced, performance, ultra-performance; or `off` (MSAA). With
    /// `--rt` it sets the denoiser's (Ray Reconstruction) quality. Default on
    /// Medium and up: auto in builds with DLSS, taa otherwise; Low: off.
    /// Env: `BEVYRROWIND_DLSS`.
    #[arg(long, value_parser = sf_sky_bevy::sf_sky::Upscaling::parse)]
    dlss: Option<sf_sky_bevy::sf_sky::Upscaling>,
    /// Run an `sf_diag` benchmark scenario (JSON), then exit with its verdict.
    /// Reports go to `target/diag/`; the baseline is `baselines/<file>` next
    /// to the scenario.
    #[arg(long)]
    bench: Option<PathBuf>,
    /// Record this run as the scenario's baseline instead of comparing.
    #[arg(long, requires = "bench")]
    bench_update_baseline: bool,
    #[command(flatten)]
    perf: Box<PerfFlags>,
}

impl ViewFlags {
    fn options(
        self,
        install: &MorrowindInstall,
        title: String,
        placements: Vec<mw_world::Placement>,
        ambient: Option<[u8; 4]>,
    ) -> Result<ViewOptions> {
        let render = self.render()?;
        let baked = if self.no_baked {
            None
        } else {
            let store = sf_bake::Store::open(cache_root(self.cache)?)?;
            let manifest = store.load_manifest(&mw_bake::profile_name(install))?;
            if manifest.products.is_empty() {
                println!("no baked products for this install (run `bevyrrowind bake`); loading NIFs");
            }
            Some(Baked { store, manifest })
        };
        let meta = std::collections::BTreeMap::from([("cell".to_owned(), title.clone()), ("render".to_owned(), render.label())]);
        let perf = Perf::new(&self.perf, self.bench.as_deref(), self.bench_update_baseline, meta, |_| {})?;
        mw_assets::set_cutout_foliage(!perf.off.has(Part::LeafMask));
        Ok(ViewOptions {
            title,
            placements,
            ambient,
            screenshot: self.screenshot,
            screenshot_anyway: self.screenshot_anyway,
            settle: self.settle,
            camera: self.camera.as_deref().map(parse_camera).transpose()?,
            calibration: self.lighting.calibration(),
            baked,
            lod_tint: self.lod_tint,
            skinned: self.skinned.then(|| SkinnedView {
                skeleton: self.skeleton.as_deref().and_then(mw_world::model_path),
                animate: self.animate,
            }),
            render,
            perf,
        })
    }

    /// Whether an interior opens in the single-cell viewer.
    fn wants_single_cell(&self) -> bool {
        self.single_cell || self.rt || self.no_baked || self.skinned
    }

    /// Options for the streamed view starting at `start`.
    fn exterior_options(
        self,
        install: &MorrowindInstall,
        db: &RecordDb,
        title: String,
        start: StreamStart,
    ) -> Result<ExteriorOptions> {
        mw_environment_bevy::water::set_mode(self.water.unwrap_or_default(), self.water_wind);
        if self.no_baked {
            return Err("the exterior streams from bake products; --no-baked is not available".into());
        }
        let render = self.render()?;
        let store = sf_bake::Store::open(cache_root(self.cache)?)?;
        let manifest = store.load_manifest(&mw_bake::profile_name(install))?;
        let mut heights = mw_world::terrain::HeightSampler::new(db);
        let interior = matches!(start, StreamStart::Interior { .. });
        // Interiors have no terrain: "+h" heights are above 0.
        let mut ground = |x: f64, z: f64| if interior { 0.0 } else { ground_m(&mut heights, x, z) };
        let (eye, target) = match (&self.camera, &start) {
            (Some(raw), _) => parse_exterior_camera(raw, &mut ground)?,
            (None, StreamStart::Exterior(center)) => {
                // Default: 150 m south-east of the place, 60 m up, looking at it.
                let target = glam::DVec3::new(center.x, ground(center.x, center.y), center.y);
                let (ex, ez) = (center.x + 90.0, center.y + 120.0);
                (glam::DVec3::new(ex, ground(ex, ez).max(target.y) + 60.0, ez), target)
            }
            (None, StreamStart::Interior { eye, target, .. }) => (*eye, *target),
        };
        let worldspace = match start {
            StreamStart::Interior { worldspace, .. } => worldspace,
            StreamStart::Exterior(_) => mw_world::ids::exterior(),
        };
        let meta = std::collections::BTreeMap::from([
            ("cell".to_owned(), title.clone()),
            ("camera".to_owned(), format!("{eye:.1} -> {target:.1}")),
            ("bench_ground".to_owned(), self.bench_ground.to_string()),
            ("rings".to_owned(), format!("near {:?} far {:?}", self.near_ring, self.far_ring)),
            ("render".to_owned(), render.label()),
            ("prepass".to_owned(), self.prepass.to_string()),
        ]);
        let bench_ground = self.bench_ground;
        let mut perf = Perf::new(&self.perf, self.bench.as_deref(), self.bench_update_baseline, meta, |scenario| {
            if bench_ground && let sf_diag_bevy::sf_diag::Measure::Path { keyframes, .. } = &mut scenario.measure {
                for k in keyframes {
                    k.position.y += ground(k.position.x, k.position.z);
                }
            }
        })?;
        mw_assets::set_cutout_foliage(!perf.off.has(Part::LeafMask));
        if let Some((_, settings)) = &mut perf.bench {
            // big_space: the app converts the pose to grid coordinates.
            settings.apply_transform = false;
        }
        println!("camera {eye:.1} -> {target:.1}");
        let weather = WeatherSetup::build(db, eye, self.day * 24.0 + self.hour, self.weather, 0);
        if let Some(t) = weather.start {
            println!("weather {}{}", t.name(), if self.weather.is_some() { " (forced)" } else { "" });
        }
        Ok(ExteriorOptions {
            title,
            eye,
            target,
            screenshot: self.screenshot,
            screenshot_anyway: self.screenshot_anyway,
            settle: self.settle,
            store,
            manifest,
            lod_tint: self.lod_tint,
            terrain_lod: self.terrain_lod,
            wireframe: self.wire,
            rings: self.rings,
            physics_debug: self.physics_debug,
            near_ring: self.near_ring,
            far_ring: self.far_ring,
            no_lights: self.no_lights || perf.off.has(Part::Lights),
            far_tint: self.far_tint,
            prepass: self.prepass,
            hour: self.hour,
            day: self.day,
            sun_size: self.sun_size,
            clouds: self.clouds,
            haze: self.haze,
            dust: self.dust,
            fog: self.fog,
            weather: Some(weather),
            quality: Quality::resolve(self.quality),
            perf,
            lights: light_infos(db),
            world: mw_doors_bevy::WorldLinks::build(db),
            water: WaterInputs::build(db),
            use_door: self.use_door.unwrap_or(0),
            activate: self.activate.unwrap_or(0),
            open_ui: self.open_ui.clone(),
            actors: None,
            level: None,
            bodies: self.bodies,
            cloth: self.cloth,
            worldspace,
            calibration: self.lighting.calibration(),
            render,
        })
    }

    /// `--rt`, `--dlss`, `--off denoiser` and `--off shadowcache`.
    fn render(&self) -> Result<RenderSettings> {
        let off = Off::from_flags(&self.perf.off)?;
        let low = Quality::resolve(self.quality) == Quality::Low;
        Ok(RenderSettings::new(self.rt, self.dlss, !off.has(Part::Denoiser), low)?.with_point_shadows(self.lighting.point_shadows).with_sun_cascades(self.lighting.sun_cascades)
            .with_shadow_cache(!off.has(Part::ShadowCache)))
    }
}

/// Where the streamed view starts.
enum StreamStart {
    /// An exterior place: its center (x, z) in world meters.
    Exterior(glam::DVec2),
    /// An interior worldspace, with the default eye and target.
    Interior { worldspace: sf_world::WorldspaceId, eye: glam::DVec3, target: glam::DVec3 },
}

/// The single-cell viewer's default framing of an interior (eye, target):
/// above and south of the center of its placements.
fn interior_view(placements: &[mw_world::Placement]) -> (glam::DVec3, glam::DVec3) {
    let points = placements.iter().filter(|p| p.model.is_some()).map(|p| glam::Vec3::from_array(p.transform.translation).as_dvec3());
    let (lo, hi) = points.fold((glam::DVec3::INFINITY, glam::DVec3::NEG_INFINITY), |(lo, hi), p| (lo.min(p), hi.max(p)));
    if !lo.x.is_finite() {
        return (glam::DVec3::new(0.0, 2.0, 5.0), glam::DVec3::ZERO);
    }
    let center = (lo + hi) / 2.0;
    let extent = (hi - lo).max_element().max(4.0);
    (center + glam::DVec3::new(0.0, extent * 0.45, extent * 0.55), center)
}

/// Ground height (meters) at a world position: the terrain, or the sea
/// where the terrain is under water.
fn ground_m(heights: &mut mw_world::terrain::HeightSampler<'_>, x: f64, z: f64) -> f64 {
    let u = mw_world::convert::UNIT_METERS_F64;
    #[allow(clippy::cast_possible_truncation)]
    let h = heights.height((x / u) as f32, (-z / u) as f32).map_or(0.0, |h| f64::from(h) * u);
    h.max(0.0)
}

/// "ex,ey,ez,tx,ty,tz" in meters, where a y written "+h" is h above the ground.
fn parse_exterior_camera(
    raw: &str,
    ground: &mut dyn FnMut(f64, f64) -> f64,
) -> std::result::Result<(glam::DVec3, glam::DVec3), String> {
    let parts: Vec<&str> = raw.split(',').map(str::trim).collect();
    let [ex, ey, ez, tx, ty, tz] = parts[..] else { return Err("expected six comma-separated values".into()) };
    let num = |v: &str| v.parse::<f64>().map_err(|e| format!("{v:?}: {e}"));
    let mut point = |x: &str, y: &str, z: &str| -> std::result::Result<glam::DVec3, String> {
        let (x, z) = (num(x)?, num(z)?);
        let y = match y.strip_prefix('+') {
            Some(h) => ground(x, z) + num(h)?,
            None => num(y)?,
        };
        Ok(glam::DVec3::new(x, y, z))
    };
    Ok((point(ex, ey, ez)?, point(tx, ty, tz)?))
}

/// Light parameters of every `LIGH` record, by lowercase id.
fn light_infos(db: &RecordDb) -> std::collections::HashMap<String, mw_world::LightInfo> {
    db.iter::<tes3::esp::Light>()
        .filter_map(|(id, _)| {
            let (_, _, light) = mw_world::resolve_base(db, id.as_str())?;
            Some((id.as_str().to_ascii_lowercase(), light?))
        })
        .collect()
}

/// An exterior place: grid "x,y" (its center) or the name of exterior
/// cells (the center of all cells with that name), in world meters (x, z).
fn exterior_place(db: &RecordDb, name: &str) -> Option<(String, glam::DVec2)> {
    let trimmed = name.trim().trim_start_matches('@');
    let grids: Vec<(i32, i32)> = if let Some((x, y)) = trimmed.split_once(',')
        && let (Ok(x), Ok(y)) = (x.trim().parse::<i32>(), y.trim().parse::<i32>())
    {
        vec![(x, y)]
    } else {
        db.iter::<tes3::esp::Cell>()
            .filter(|(_, c)| !c.is_interior() && c.name.eq_ignore_ascii_case(trimmed))
            .map(|(_, c)| c.data.grid)
            .collect()
    };
    if grids.is_empty() {
        return None;
    }
    #[allow(clippy::cast_precision_loss)]
    let n = grids.len() as f64;
    let (sx, sy) = grids.iter().fold((0.0, 0.0), |(a, b), &(x, y)| (a + f64::from(x) + 0.5, b + f64::from(y) + 0.5));
    let cell = mw_world::convert::CELL_UNITS * mw_world::convert::UNIT_METERS_F64;
    let label = if grids.len() == 1 { format!("{trimmed} {:?}", grids[0]) } else { format!("{trimmed} ({} cells)", grids.len()) };
    Some((label, glam::DVec2::new(sx / n * cell, -(sy / n) * cell)))
}

fn parse_camera(raw: &str) -> std::result::Result<(bevy::math::Vec3, bevy::math::Vec3), String> {
    let values: Vec<f32> =
        raw.split(',').map(|v| v.trim().parse::<f32>().map_err(|e| e.to_string())).collect::<std::result::Result<_, _>>()?;
    match values[..] {
        [ex, ey, ez, tx, ty, tz] => Ok((bevy::math::Vec3::new(ex, ey, ez), bevy::math::Vec3::new(tx, ty, tz))),
        _ => Err("expected six comma-separated numbers".into()),
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn run(cli: Cli) -> Result {
    mw_boot_bevy::mods::select(mw_boot_bevy::mods::Selection { user_data: cli.user_data, profile: cli.profile });
    let command = match cli.command {
        Command::Detect => return detect(cli.morrowind.as_deref()),
        Command::Scene { name, list, file, extra } => {
            return scenes::run(cli.morrowind, name.as_deref(), list, file.as_deref(), extra);
        }
        Command::Perf { command } => return perf::cli::run(command),
        other => other,
    };
    let (install, origin) = MorrowindInstall::detect(cli.morrowind.as_deref())?;
    match command {
        Command::Detect | Command::Perf { .. } | Command::Scene { .. } => unreachable!(),
        Command::Info { conflicts } => info(&install, &origin, conflicts),
        Command::Bake { cache } => mw_boot_bevy::bake::run(&install, cache),
        Command::Weather { command } => mw_environment_bevy::weather::run(&install, command),
        Command::Mods { command } => mods::run(&install, command),
        Command::Ls { dir } => {
            let vfs = mount(&install, false)?;
            ls(&vfs, dir.as_deref().unwrap_or(""));
            Ok(())
        }
        Command::Uses { id, limit } => {
            let (_, content) = load(&install, false)?;
            for (cell, count) in mw_world::cells_using(&content.db, &id).into_iter().take(limit) {
                let name = mw_world::cell_label(cell);
                println!("{count:>4}  {name}");
            }
            Ok(())
        }
        Command::Extract { path, out } => {
            let vfs = mount(&install, false)?;
            std::fs::write(&out, vfs.read(&VfsPath::new(&path)?)?)?;
            Ok(())
        }
        Command::Which { path } => {
            let vfs = mount(&install, false)?;
            which(&vfs, &path)
        }
        Command::Nif { path } => {
            let vfs = mount(&install, false)?;
            nif(&vfs, &path)
        }
        Command::Anim { path, events, group } => {
            let vfs = mount(&install, false)?;
            let normalized = path.to_ascii_lowercase().replace('\\', "/");
            let path = if normalized.starts_with("meshes/") { normalized } else { format!("meshes/{normalized}") };
            let animation = mw_anim::Animation::from_file(&path, &vfs.read(&VfsPath::new(&path)?)?)?;
            print!("{}", mw_anim::report(&path, &animation, &mw_anim::ReportOptions { events, group }));
            Ok(())
        }
        Command::NifSweep { prefix, examples, all } => {
            let vfs = mount(&install, false)?;
            sweep::run(&vfs, &prefix, examples, all);
            Ok(())
        }
        Command::Record { kind, id } => {
            let (_, content) = load(&install, false)?;
            record(&content.db, &kind, &id)
        }
        Command::Find { kind, pattern, limit } => {
            let (_, content) = load(&install, false)?;
            find(&content.db, &kind, pattern.as_deref(), limit)
        }
        Command::View { cell, model: true, flags } => {
            let vfs = mount(&install, false)?;
            let path = mw_world::model_path(&cell).ok_or("empty model path")?;
            let placement = mw_world::Placement {
                base_id: path.clone(),
                kind: mw_world::BaseKind::Static,
                model: Some(path.clone()),
                transform: mw_nif::coords::Trs::IDENTITY,
                light: None,
                editor_only: false,
            };
            let options =
                flags.options(&install, format!("Bevyrrowind - {path}"), vec![placement], Some([160, 160, 160, 255]))?;
            mw_viewers_bevy::view::run(std::sync::Arc::new(vfs), options)
        }
        Command::View { cell, model: false, flags } => {
            view_cell(&install, &cell, flags)
        }
        Command::Character { ids, screenshot, settle, camera, cache, bodies, cloth, walk, drawn, night } => {
            let (vfs, content) = load(&install, false)?;
            let store = sf_bake::Store::open(cache_root(cache)?)?;
            let manifest = store.load_manifest(&mw_bake::profile_name(&install))?;
            let camera = camera.as_deref().map(parse_camera).transpose()?;
            let options = mw_viewers_bevy::character_view::CharacterViewOptions { ids, screenshot, settle, camera, bodies, cloth, walk, drawn, night };
            mw_viewers_bevy::character_view::run(std::sync::Arc::new(vfs), std::sync::Arc::new(content.db), store, manifest, options);
            Ok(())
        }
    }
}

/// `view <cell>`: an exterior place or an interior, streamed; or an interior
/// in the single-cell viewer.
fn view_cell(install: &MorrowindInstall, cell: &str, flags: ViewFlags) -> Result {
    // Synthetic test levels (TS1): `level:<name>`.
    if let Some(name) = cell.strip_prefix(mw_exterior_bevy::level::PREFIX) {
        return level::view(install, name, flags);
    }
    let (vfs, content) = load(install, false)?;
    // Exterior cells (by grid "x,y" or by name) stream the whole exterior.
    let interior = mw_world::find_cell(&content.db, cell).filter(|c| c.is_interior());
    if interior.is_none()
        && let Some((label, center)) = exterior_place(&content.db, cell)
    {
        println!("exterior at {label}: ({:.0}, {:.0}) m", center.x, center.y);
        let mut options =
            flags.exterior_options(install, &content.db, format!("Bevyrrowind - {label}"), StreamStart::Exterior(center))?;
        if let Some(w) = &mut options.weather {
            w.load_sounds(&content.db, &vfs);
        }
        options.actors = Some(std::sync::Arc::new(content.db));
        return mw_exterior_bevy::exterior::run(std::sync::Arc::new(vfs), options);
    }
    let found = interior.ok_or_else(|| format!("no cell {cell:?}"))?;
    let (placements, skipped) = mw_world::cell_placements(&content.db, found);
    // Interiors stream like the exterior (doors work); the single-cell
    // viewer remains for what the streamed view lacks.
    if !flags.wants_single_cell() {
        let (eye, target) = interior_view(&placements);
        let start = StreamStart::Interior { worldspace: mw_world::ids::worldspace_of(found), eye, target };
        println!("interior {}: streamed view (--single-cell for the single-cell viewer)", found.name);
        let mut options = flags.exterior_options(install, &content.db, format!("Bevyrrowind - {}", found.name), start)?;
        if let Some(w) = &mut options.weather {
            w.load_sounds(&content.db, &vfs);
        }
        options.actors = Some(std::sync::Arc::new(content.db));
        return mw_exterior_bevy::exterior::run(std::sync::Arc::new(vfs), options);
    }
    if !skipped.is_empty() {
        eprintln!("warning: {} references with unknown base records", skipped.len());
    }
    println!("{}: {} placements", found.name, placements.len());
    let ambient = found.atmosphere_data.as_ref().map(|a| a.ambient_color);
    let options = flags.options(install, format!("Bevyrrowind - {}", found.name), placements, ambient)?;
    mw_viewers_bevy::view::run(std::sync::Arc::new(vfs), options)
}

fn detect(explicit: Option<&std::path::Path>) -> Result {
    for candidate in mw_install::candidates(explicit) {
        let verdict = match MorrowindInstall::open(&candidate.path) {
            Ok(_) => "OK",
            Err(_) => "-",
        };
        println!("{verdict:>3}  {:<40} {}", format!("{:?}", candidate.origin), candidate.path.display());
    }
    match MorrowindInstall::detect(explicit) {
        Ok((install, origin)) => {
            println!("\nusing {} ({origin:?})", install.root.display());
            println!("game files: {}", install.game_files().join(", "));
            println!("archives:   {}", install.archives().join(", "));
            Ok(())
        }
        Err(error) => Err(error.into()),
    }
}

fn info(install: &MorrowindInstall, origin: &Origin, conflicts: usize) -> Result {
    println!("install: {} ({origin:?})", install.root.display());
    let (_, content) = load(install, true)?;
    let report = content.db.conflict_report();
    println!("\nrecords: {} live\n", report.total_live());
    print!("{}", report.render(&content.db, conflicts));
    Ok(())
}

fn ls(vfs: &Vfs, dir: &str) {
    for entry in vfs.read_dir(dir) {
        match entry {
            DirEntry::Dir(name) => println!("{name}/"),
            DirEntry::File(path) => {
                let winner = vfs.resolve(&path).map(|s| vfs.source(s).label().to_owned()).unwrap_or_default();
                println!("{:<48} {winner}", path.file_name());
            }
        }
    }
}

fn which(vfs: &Vfs, raw: &str) -> Result {
    let path = VfsPath::new(raw)?;
    let providers = vfs.providers(&path);
    if providers.is_empty() {
        return Err(format!("{path} is not in the VFS").into());
    }
    for source in providers {
        println!("{}", vfs.source(*source).label());
    }
    Ok(())
}

fn nif(vfs: &Vfs, raw: &str) -> Result {
    let path = match mw_world::model_path(raw) {
        Some(p) if !raw.to_ascii_lowercase().replace('\\', "/").starts_with("meshes/") => p,
        _ => raw.to_owned(),
    };
    let model = mw_nif::load(&vfs.read(&VfsPath::new(&path)?)?)?;
    println!("{path}: marker={} no_collision={} had_skin={}", model.marker, model.no_collision, model.had_skin);
    if let Some((lo, hi)) = mw_nif::visible_bounds(&model) {
        println!("bounds (engine meters): {lo:.2?} .. {hi:.2?}");
    }
    for (i, node) in model.nodes.iter().enumerate() {
        let depth = std::iter::successors(node.parent, |&p| model.nodes[p].parent).count();
        let mesh = node.mesh.map_or(String::new(), |m| {
            let mesh = &model.meshes[m];
            format!("  mesh#{m} {} tris, material#{}", mesh.indices.len() / 3, mesh.material)
        });
        let hidden = if node.visible { "" } else { "  [hidden]" };
        let t = node.transform;
        let transform = if t == mw_nif::coords::Trs::IDENTITY {
            String::new()
        } else {
            format!("  t={:.2?} r={:.2?} s={:.2}", t.translation, t.rotation, t.scale)
        };
        println!("{i:>3} {}{:?} {:?}{mesh}{hidden}{transform}", "  ".repeat(depth), node.kind, node.name);
    }
    for (i, m) in model.materials.iter().enumerate() {
        println!(
            "material#{i}: {:?} {:?} {:?} opacity={} diffuse={:?} emissive={:?} vertex_colors={:?}",
            m.base_texture, m.glow_texture, m.alpha_mode, m.alpha, m.diffuse, m.emissive, m.vertex_colors
        );
    }
    if !model.text_keys.is_empty() {
        println!("text keys: {:?}", model.text_keys);
    }
    for c in &model.controllers {
        let target = match &c.target {
            mw_nif::anim::Target::Transform { node, track } => format!(
                "transform node#{node} t={} r={} s={}",
                track.translation.is_some(),
                track.rotation.is_some(),
                track.scale.is_some()
            ),
            mw_nif::anim::Target::Visibility { node, .. } => format!("visibility node#{node}"),
            mw_nif::anim::Target::Alpha { material, .. } => format!("alpha material#{material}"),
            mw_nif::anim::Target::MaterialColor { material, slot, .. } => format!("{slot:?} color material#{material}"),
            mw_nif::anim::Target::Uv { material, track } => {
                format!("uv material#{material} at 0/0.25/0.5: {:?} {:?} {:?}", track.sample(0.0), track.sample(0.25), track.sample(0.5))
            }
        };
        let t = c.timing;
        println!(
            "controller: {target}  [{:.2}..{:.2}] x{} {:?} auto_play={} random_start={}",
            t.start, t.stop, t.frequency, t.cycle, c.auto_play, c.random_start
        );
    }
    for p in &model.particles {
        println!(
            "particles: node#{} emitter#{} material#{} max={} rate={:.1}/s life={:.2}+{:.2}s speed={:.2}m/s radius={:.3}m color={:?} auto_play={} modifiers={:?}",
            p.node,
            p.emitter,
            p.material,
            p.max_particles,
            p.birth_rate,
            p.lifespan,
            p.lifespan_variation,
            p.speed,
            p.initial_size,
            p.initial_color.to_array(),
            p.auto_play,
            p.modifiers.iter().map(|m| match m { mw_nif::particles::Modifier::Color(_) => "Color".to_owned(), other => format!("{other:?}") }).collect::<Vec<_>>()
        );
    }
    Ok(())
}

fn unknown_kind(kind: &str) -> Box<dyn std::error::Error> {
    format!("unknown record kind {kind:?}; known kinds: {}", mw_records::KINDS.join(" ")).into()
}

fn record(db: &RecordDb, kind: &str, id: &str) -> Result {
    struct Show<'a>(&'a RecordDb, RecordId);
    impl KindVisitor for Show<'_> {
        type Output = Option<String>;
        fn visit<T: std::fmt::Debug + Send + Sync + 'static>(self) -> Option<String> {
            let entry = self.0.entry::<T>(&self.1)?;
            let history: Vec<String> =
                entry.history.iter().map(|t| format!("{} ({})", self.0.source(t.source).name, sf_records::describe(t))).collect();
            let value = entry.value.as_ref().map_or_else(|| "<deleted>".to_owned(), |v| format!("{v:#?}"));
            Some(format!("{value}\n\nhistory: {}", history.join(" -> ")))
        }
    }
    let shown = mw_records::visit_kind(kind, Show(db, RecordId::new(id))).ok_or_else(|| unknown_kind(kind))?;
    println!("{}", shown.ok_or_else(|| format!("no {kind} record {id:?}"))?);
    Ok(())
}

fn find(db: &RecordDb, kind: &str, pattern: Option<&str>, limit: usize) -> Result {
    struct Find<'a>(&'a RecordDb, Option<String>, usize);
    impl KindVisitor for Find<'_> {
        type Output = (Vec<String>, usize);
        fn visit<T: std::fmt::Debug + Send + Sync + 'static>(self) -> (Vec<String>, usize) {
            let matches: Vec<String> = self
                .0
                .iter::<T>()
                .map(|(id, _)| id.to_string())
                .filter(|id| self.1.as_deref().is_none_or(|p| id.contains(p)))
                .collect();
            let total = matches.len();
            (matches.into_iter().take(self.2).collect(), total)
        }
    }
    let pattern = pattern.map(str::to_ascii_lowercase);
    let (ids, total) = mw_records::visit_kind(kind, Find(db, pattern, limit)).ok_or_else(|| unknown_kind(kind))?;
    for id in &ids {
        println!("{}", id.replace('\u{1f}', " / "));
    }
    if total > ids.len() {
        println!("... {} more (use --limit)", total - ids.len());
    }
    Ok(())
}

/// Lighting calibration overrides (see `mw_assets::LightCalibration`).
#[derive(clap::Args)]
struct Lighting {
    /// Light power, lumens per square meter of light radius.
    #[arg(long)]
    lumens: Option<f32>,
    /// Light range cutoff as a multiple of its radius.
    #[arg(long)]
    range: Option<f32>,
    /// Ambient luminance (nits) for a white cell ambient.
    #[arg(long)]
    ambient: Option<f32>,
    /// Night exposure floor, EV100: the darkest exposure a view may take.
    #[arg(long)]
    exposure_floor: Option<f32>,
    /// Exterior ambient floor at night, nits.
    #[arg(long)]
    night_ambient: Option<f32>,
    /// Point lights casting shadows (the most important, `sf_sky_bevy::shadow_importance`; default by `--quality`;
    /// env `BEVYRROWIND_POINT_SHADOWS`).
    #[arg(long)]
    point_shadows: Option<usize>,
    /// Sun cascades `COUNT:METERS` instead of the preset's (A/B; env `BEVYRROWIND_SUN_CASCADES`).
    #[arg(long, value_parser = parse_sun_cascades)]
    sun_cascades: Option<(usize, f32)>,
}

/// Light-lab knobs (`view level:light-lab`, docs/specs/test-levels.md "light-lab"): they edit the
/// level's parametric lights, so a perf sweep (`perf sweep`) steps one per run.
#[derive(clap::Args)]
struct LabFlags {
    /// Levels with `lights`: this many lights instead of the level's count.
    #[arg(long = "lab-lights")]
    lights: Option<u32>,
    /// Levels with `lights`: lay them on a ring of this radius (m) around the rig center.
    #[arg(long = "lab-ring")]
    ring: Option<f32>,
    /// Levels with `lights`: the lights' radius, m (range = radius x `--range`).
    #[arg(long = "lab-radius")]
    radius: Option<f32>,
    /// Levels with `lights`: orbit the rig center at this rate, deg/s (moving lights).
    #[arg(long = "lab-orbit")]
    orbit: Option<f32>,
}

impl Lighting {
    fn calibration(&self) -> mw_assets::LightCalibration {
        let mut calibration = mw_assets::LightCalibration::default();
        calibration.lumens_per_radius_sq = self.lumens.unwrap_or(calibration.lumens_per_radius_sq);
        calibration.range_scale = self.range.unwrap_or(calibration.range_scale);
        calibration.ambient_nits = self.ambient.unwrap_or(calibration.ambient_nits);
        calibration.exposure_floor_ev100 = self.exposure_floor.unwrap_or(calibration.exposure_floor_ev100);
        calibration.night_ambient_nits = self.night_ambient.unwrap_or(calibration.night_ambient_nits);
        calibration
    }
}

/// `--quality` values.
fn parse_quality(s: &str) -> std::result::Result<Quality, String> {
    Quality::parse(s).ok_or_else(|| format!("unknown quality {s:?}: low, medium, high, ultra"))
}

/// `--clouds` values (the `sf_atmos` cloud genera).
fn parse_clouds(s: &str) -> std::result::Result<sf_atmos_bevy::sf_atmos::CloudPreset, String> {
    sf_atmos_bevy::sf_atmos::CloudPreset::parse(s).ok_or_else(|| {
        let names: Vec<_> = sf_atmos_bevy::sf_atmos::CloudPreset::ALL.iter().map(|c| c.0).collect();
        format!("unknown clouds {s:?}: {}", names.join(", "))
    })
}

fn parse_bodies(s: &str) -> std::result::Result<sf_character_bevy::SmoothBodies, String> {
    match s.to_ascii_lowercase().as_str() {
        "smooth" => Ok(sf_character_bevy::SmoothBodies::Smooth),
        "vanilla" => Ok(sf_character_bevy::SmoothBodies::Vanilla),
        other => Err(format!("bodies {other:?}: smooth or vanilla")),
    }
}
