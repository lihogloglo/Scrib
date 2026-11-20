//! `view world:<guide dir>` (worldgen slice G1, `superfargoth/docs/specs/worldgen.md`): a generated guide
//! (format v1, e.g. `target/worldgen/vo/V3`) as the exterior, in the game renderer. Its terrain tiles are
//! generated on demand (`sf_worldgen::tiles::GuideTiles`: the guide's Catmull-Rom surface plus the runtime erosion
//! filter, band-limited per level, down to 0.5 m), nothing of the content streams in
//! (`mw_exterior_bevy::worldgen`), the sky, clouds (raised), weather and water are the exterior's. The camera
//! starts south-east of the guide's summit looking at it; `--camera ex,ey,ez,tx,ty,tz` (heights `+h` above the
//! generated ground) overrides it.
//!
//! Slice G2: the clipmaps fill on the GPU from the provider's field and its CPU tiles only answer height queries
//! (generated on a small pool of their own, `twin_threads`); `--off gputerrain` is the G1 path (every tile on the
//! CPU, rayon's global pool). The filter is live (`worldgen.filter`, `worldgen.octaves`, `mw_exterior_bevy::worldgen`).
//!
//! WG-X4: a guide with `anchors.json` (the same map at 10×, `worldgen-x10`) streams the content's settlements at
//! their new places: anchor members and what lies inside stamped footprints, moved through the anchor transform
//! (`world_content`); the rest of the content's exterior references stay out.

use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use mw_environment_bevy::water::WaterInputs;
use mw_devtools_bevy::perf::{Off, Part};
use mw_exterior_bevy::worldgen::{CLOUD_BASE_LIFT_M, WorldgenSetup, WorldgenTiles};
use mw_install::MorrowindInstall;
use mw_world::convert::UNIT_METERS_F64;
use mw_worldgen::classify::Class;
use sf_bake::{Manifest, Store};
use sf_records::RecordDb;
use sf_worldgen::anchors::AnchorTransform;
use sf_worldgen::filter::FilterParams;
use sf_worldgen::tiles::{GuideTiles, LevelTiming};

/// `view world:<guide dir>`.
pub(crate) const PREFIX: &str = "world:";

/// The default view of the summit: the eye this far east and south of it, meters.
// tunables: const (a framing default: where a debug view starts; `--camera` overrides it)
const EYE_OFFSET_M: [f64; 2] = [3000.0, 11000.0];
/// ... this high above the ground under it (and above the sea), meters.
// tunables: const (a framing default: where a debug view starts; `--camera` overrides it)
const EYE_HEIGHT_M: f64 = 400.0;

pub(crate) fn view(install: &MorrowindInstall, dir: &str, mut flags: crate::ViewFlags) -> crate::Result {
    let started = Instant::now();
    let path = Path::new(dir);
    let ext = mw_world::ids::exterior();
    // The GPU fill (default) keeps the CPU tiles to the twin's own small pool; the G1 arm uses rayon's.
    let gpu = !Off::from_flags(&flags.perf.off)?.has(Part::GpuTerrain);
    let mut opened = GuideTiles::open(path, ext.clone()).map_err(|e| format!("world:{dir}: {e}"))?;
    if gpu {
        opened = opened.with_twin_pool().map_err(|e| format!("world:{dir}: {e}"))?;
    }
    let tiles = Arc::new(opened);
    let g = tiles.guide();
    if g.sea_level_m != 0.0 {
        eprintln!("warning: world:{dir}: guide sea level {} m, the game's sea is at 0 m", g.sea_level_m);
    }
    let [sx, sh, sz] = tiles.summit();
    let [x0, z0, x1, z1] = tiles.extent_m();
    let prm = *tiles.params();
    let lift = CLOUD_BASE_LIFT_M.default_value();
    let label = format!(
        "worldgen world: {dir}, {}x{} @ {} m, x {x0:.0}..{x1:.0} m, z {z0:.0}..{z1:.0} m, sea {} m, summit {sh:.0} m at \
         ({sx:.0}, {sz:.0}); runtime filter {} octaves {}..{} m, max {} m, band {:?} samples/wavelength; clouds +{lift:.0} m; \
         terrain fill {}",
        g.w,
        g.h,
        g.spacing_m,
        g.sea_level_m,
        prm.octaves,
        prm.wavelength_m,
        prm.wavelength_m / prm.lacunarity.powi(i32::try_from(prm.octaves).unwrap_or(1) - 1),
        prm.max_amplitude_m,
        tiles.settings().band_samples,
        if gpu { format!("GPU (CPU twin on {} threads)", tiles.settings().twin_threads) } else { "CPU tiles (G1 arm)".into() },
    );
    println!("{label}");
    let mut ground = |x: f64, z: f64| f64::from(tiles.height(x, z, 0));
    let (eye, target) = if let Some(raw) = flags.camera.take() {
        crate::parse_exterior_camera(&raw, &mut ground)?
    } else {
        let (ex, ez) = (sx + EYE_OFFSET_M[0], sz + EYE_OFFSET_M[1]);
        let eye = glam::DVec3::new(ex, ground(ex, ez).max(0.0) + EYE_HEIGHT_M, ez);
        (eye, glam::DVec3::new(sx, sh * 0.8, sz))
    };
    // `--set scatter.budget=<count>`: the flora pass's budget (WG-X5), read here because the pass runs before the app.
    let flora_budget = flags.perf.set.iter().find_map(|s| s.strip_prefix("scatter.budget=")).map(str::parse::<f64>).transpose()
        .map_err(|e| format!("--set scatter.budget: {e}"))?;
    // A load-time value, not an app tunable: the app's `--set` check would reject the name.
    flags.perf.set.retain(|s| !s.starts_with("scatter.budget="));
    let (vfs, content) = mw_boot_bevy::load(install, false)?;
    let vfs = flags.delight(install, vfs)?;
    println!("content loaded in {:.2} s", started.elapsed().as_secs_f32());
    // The exterior worldspace with the given camera (the `Interior` start only carries them: `exterior_options`
    // would read the content's ground for a place).
    let start = crate::StreamStart::Interior { worldspace: ext, eye, target };
    let mut options = flags.exterior_options(install, &content.db, format!("Bevyrrowind - world {dir}"), start)?;
    // The shore bake reads the content's coast: the generated world has open water only (neutral optics).
    options.water = WaterInputs::default();
    if let Some(w) = &mut options.weather {
        w.load_sounds(&content.db, &vfs);
    }
    let tiles = with_exterior_layers(tiles, &options.store, &options.manifest, &mw_world::ids::exterior(), dir)?;
    let anchors = AnchorTransform::read(path).map_err(|e| format!("world:{dir}: {e}"))?;
    let world_content = anchors.map(|a| moved_content(&content.db, &a, &tiles, &options.store, &options.manifest, flora_budget));
    options.actors = Some(Arc::new(content.db));
    let report = tiles.clone();
    let base = tiles.clone();
    let rebuild = move |octaves: u32, sharp: f32| -> WorldgenTiles {
        let prm = FilterParams { octaves, ..*base.params() }.with_sharp_scale(sharp);
        let t = Arc::new(base.with_params(prm));
        let report = t.clone();
        (t, Arc::new(move || summary(&report.timings())))
    };
    options.worldgen = Some(WorldgenSetup {
        tiles,
        label,
        cloud_base_lift_m: lift,
        timings: Arc::new(move || summary(&report.timings())),
        octaves: prm.octaves,
        rebuild: Arc::new(rebuild),
        content: world_content,
    });
    mw_exterior_bevy::exterior::run(Arc::new(vfs), options)
}

/// WG-X3: the generated world draws the exterior bake's layer arrays (a guide's `layers.json` names the same layer
/// records, `worldgen-layers`), and its macro look comes from their mean albedos. Without a baked layer table the
/// tiles stay as they are (default layer, fallback look).
#[allow(clippy::cast_precision_loss)] // MiB for a log line
fn with_exterior_layers(
    tiles: Arc<GuideTiles>,
    store: &Store,
    manifest: &Manifest,
    ext: &sf_world::WorldspaceId,
    dir: &str,
) -> Result<Arc<GuideTiles>, String> {
    let baked = Arc::new(sf_terrain::bake::BakedTerrain::new(store.clone(), manifest, ext.clone()));
    let table = match baked.layer_table() {
        Some(Ok(t)) => t,
        Some(Err(e)) => {
            eprintln!("warning: world:{dir}: baked terrain layer table: {e}; no ground layers");
            return Ok(tiles);
        }
        None => {
            eprintln!("warning: world:{dir}: no baked terrain layer table (run `bevyrrowind bake`); no ground layers");
            return Ok(tiles);
        }
    };
    let arrays: sf_worldgen::tiles::LayerArraysFn = Arc::new(move |size, cap| baked.layers_capped(size, cap));
    let (t, missing) = tiles.with_params(*tiles.params()).with_layer_arrays(&table, arrays).map_err(|e| format!("world:{dir}: {e}"))?;
    match t.layers() {
        Some(l) => {
            let g = &l.layers().global;
            println!(
                "worldgen layers: {} palette layers ({} not in the bake: grey), global {}x{} @ {:.2} m, {} stamp grids, \
                 layer table {} layers; GPU field {:.1} MiB",
                l.ids().len(),
                missing.len(),
                g.dims[0],
                g.dims[1],
                g.spacing_m,
                l.layers().stamps.len(),
                table.layers.len(),
                t.gpu_data_bytes() as f64 / f64::from(1u32 << 20),
            );
            if !missing.is_empty() {
                eprintln!("warning: world:{dir}: layers not in the bake's table: {missing:?}");
            }
        }
        None => println!("worldgen layers: none in {dir} (run `bevyrrowind worldgen-layers --guide {dir}`); fallback look"),
    }
    Ok(Arc::new(t))
}

/// The content's exterior references moved through `anchors` (`world_content`), with their log lines.
fn moved_content(
    db: &RecordDb,
    anchors: &AnchorTransform,
    tiles: &GuideTiles,
    store: &Store,
    manifest: &Manifest,
    flora_budget: Option<f64>,
) -> mw_exterior_bevy::worldgen::WorldgenContent {
    let started = Instant::now();
    let source = mw_world::MwWorldSource::build(db, &mw_world::SourceOptions::default());
    let ext = mw_world::ids::exterior();
    // The classification only names what's kept inside footprints and what's dropped (the census); it needs the
    // models' bounds for rocks (`mw_worldgen::classify`).
    let models: Vec<sf_vfs::VfsPath> = source.all_placements(&ext).filter_map(|p| p.model.clone()).collect();
    let infos = mw_exterior_bevy::worldgen::model_infos(store, manifest, &models);
    let infos: std::collections::HashMap<String, ([f32; 3], [f32; 3])> =
        infos.into_iter().map(|(m, i)| (m.as_str().to_owned(), (i.min.to_array(), i.max.to_array()))).collect();
    let bounds = |m: &str| infos.get(m).copied();
    let refs = mw_worldgen::x10::exterior_refs(&source, &bounds);
    let rock_min_m = mw_worldgen::anchor::Params::default().rock_min_m;
    let classes = refs.iter().map(|r| mw_worldgen::classify::classify(r, rock_min_m).0);
    let mut heights = mw_world::terrain::HeightSampler::new(db);
    let mut source_ground = |x: f64, y: f64| {
        #[allow(clippy::cast_possible_truncation)] // Morrowind units of the source map fit f32
        let units = |m: f64| (m / UNIT_METERS_F64) as f32;
        heights.height(units(x), units(y)).map(|h| f64::from(h) * UNIT_METERS_F64)
    };
    let ground = |x: f64, y: f64| f64::from(tiles.height(x, -y, 0));
    let refs: Vec<(&sf_world::Placement, Class)> = source.all_placements(&ext).zip(classes).collect();
    let (placements, census) = crate::world_content::remap(refs, anchors, &ground, &mut source_ground);
    println!(
        "worldgen anchors: {} anchors ({} stamped) at scale {}; {} placements kept in {:.2} s",
        anchors.anchors.len(),
        anchors.anchors.iter().filter(|a| a.stamp.is_some()).count(),
        anchors.scale,
        placements.len(),
        started.elapsed().as_secs_f32()
    );
    println!("worldgen anchors: {}", census.line());
    let extra = mw_worldgen::flora::extra(db, &source, &bounds, tiles, flora_budget);
    mw_exterior_bevy::worldgen::WorldgenContent { placements, extra }
}

/// `L0 12 x 31.2 ms (max 45.0) + macro 2.1 ms | L1 ...`: tiles per level, mean and max ms each.
fn summary(levels: &[LevelTiming]) -> String {
    #[allow(clippy::cast_precision_loss)]
    let per = |ms: f64, n: u64| if n == 0 { 0.0 } else { ms / n as f64 };
    levels
        .iter()
        .map(|t| {
            format!(
                "L{} {} x {:.1} ms (max {:.1}) + macro {:.1} ms",
                t.level,
                t.tiles,
                per(t.total_ms, t.tiles),
                t.max_ms,
                per(t.macro_ms, t.macros)
            )
        })
        .collect::<Vec<_>>()
        .join(" | ")
}
