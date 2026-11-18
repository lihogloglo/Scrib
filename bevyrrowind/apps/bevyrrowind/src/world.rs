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

use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use mw_environment_bevy::water::WaterInputs;
use mw_devtools_bevy::perf::{Off, Part};
use mw_exterior_bevy::worldgen::{CLOUD_BASE_LIFT_M, WorldgenSetup, WorldgenTiles};
use mw_install::MorrowindInstall;
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
    options.actors = Some(Arc::new(content.db));
    let report = tiles.clone();
    let base = tiles.clone();
    let rebuild = move |octaves: u32| -> WorldgenTiles {
        let t = Arc::new(base.with_params(FilterParams { octaves, ..*base.params() }));
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
    });
    mw_exterior_bevy::exterior::run(Arc::new(vfs), options)
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
