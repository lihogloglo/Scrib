//! `scrib-worldgen`: runs a world recipe (spec `superfargoth/docs/specs/worldgen.md`).
//!
//! ```text
//! scrib-worldgen run <recipe.toml> [--arms A0,A1] [--macro-spacing 25] [--iterations N] [--out DIR] [--no-images]
//! scrib-worldgen import-dem <tile.tif|folder>... --centre LAT,LON --extent-km EW[,NS] --spacing 20 --out DIR
//!     [--fill -30] [--sea-max 0] [--name N] [--source S] [--licence L]
//! scrib-worldgen detail --in GUIDE --out DIR --spacing 5 [--crop X0,Y0,X1,Y1] [--params FILE.toml] [--set NAME=V]...
//! scrib-worldgen stamp-volcano --base GUIDE --params FILE.toml --out DIR [--arms V1,V2]
//! scrib-worldgen measure GUIDE... [--ref GUIDE]... [--base GUIDE] [--mask-sea] [--region X0,Y0,X1,Y1] [--summit auto|X,Y] --out FILE.md|json
//! scrib-worldgen geology --base GUIDE --table GEOLOGY.toml [--set NAME=V]... --out DIR
//! scrib-worldgen place --base GUIDE --geology GEOLOGY.toml [--recipe LANDFORMS.toml] [--set NAME=V]... --out DIR [--world-filter FILTER.toml]
//! scrib-worldgen landforms GUIDE [--names NAMES.json] [--scale S] [--origin X,Y] [--set NAME=V]... --out DIR
//! ```

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use sf_worldgen::cliffs::{CliffRequest, run_cliffs};
use sf_worldgen::dem::{ImportParams, ImportRequest, run_import};
use sf_worldgen::filter::{DetailRequest, apply_overrides_toml, declared_defaults, run_detail, set};
use sf_worldgen::run::{Overrides, run};
use sf_worldgen::stratovolcano::{StampRequest, run_stamp};

#[derive(Parser)]
#[command(
    name = "scrib-worldgen",
    about = "Runs Scrib world recipes (sf_worldgen)"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Runs every arm of a recipe and writes heights, previews, sheets and README.md.
    Run {
        recipe: PathBuf,
        /// Only these arms (comma-separated names).
        #[arg(long, value_delimiter = ',')]
        arms: Option<Vec<String>>,
        /// Macro grid spacing override, metres.
        #[arg(long)]
        macro_spacing: Option<f64>,
        /// Stream power iteration override.
        #[arg(long)]
        iterations: Option<usize>,
        /// Output folder override.
        #[arg(long)]
        out: Option<PathBuf>,
        /// Skip PNG previews.
        #[arg(long)]
        no_images: bool,
    },
    /// Imports georeferenced TIFF elevation tiles (EPSG:4326) as a guide v1 folder on a local metric grid.
    ImportDem {
        /// Georeferenced TIFF tiles, or folders of them.
        #[arg(required = true)]
        tiles: Vec<PathBuf>,
        /// Centre `lat,lon`, degrees (north, east).
        #[arg(long, value_delimiter = ',', num_args = 1, allow_negative_numbers = true)]
        centre: Vec<f64>,
        /// Extent, km: `ew` (square) or `ew,ns`.
        #[arg(long, value_delimiter = ',')]
        extent_km: Vec<f64>,
        /// Output sample spacing, metres.
        #[arg(long)]
        spacing: f64,
        /// Output guide folder.
        #[arg(long)]
        out: PathBuf,
        /// Height written for sea and no data, metres, < 0 (Guess: a flat shelf below the shoreline, as the
        /// Vvardenfell guide's -29.3 m fill).
        #[arg(long, default_value_t = -30.0, allow_negative_numbers = true)]
        fill: f64,
        /// Source heights at or below this are sea, metres (Copernicus GLO-30 open ocean reads 0).
        #[arg(long, default_value_t = 0.0, allow_negative_numbers = true)]
        sea_max: f64,
        /// Place name for the meta.
        #[arg(long)]
        name: Option<String>,
        /// Source line for the meta.
        #[arg(long)]
        source: Option<String>,
        /// Licence or attribution line for the meta.
        #[arg(long)]
        licence: Option<String>,
    },
    /// Runtime-detail preview: resamples a guide to a finer spacing over a crop and applies the erosion filter
    /// (`sf_worldgen::filter`); writes a guide v1 folder.
    Detail {
        /// Base guide folder (v1).
        #[arg(long = "in")]
        input: PathBuf,
        /// Output guide folder.
        #[arg(long)]
        out: PathBuf,
        /// Output sample spacing, metres.
        #[arg(long)]
        spacing: f64,
        /// Crop `x0,y0,x1,y1` in the guide frame, metres (x east, y north); default the whole guide.
        #[arg(long, value_delimiter = ',', num_args = 1, allow_negative_numbers = true)]
        crop: Option<Vec<f64>>,
        /// TOML file of filter value overrides (`name = v` or `name = { v, source }`).
        #[arg(long)]
        params: Option<PathBuf>,
        /// One override `name=value` (source Guess), applied after --params; repeatable.
        #[arg(long = "set")]
        sets: Vec<String>,
    },
    /// Stamps a composable stratovolcano (`sf_worldgen::stratovolcano`) on a base guide, one guide v1 folder per
    /// arm of the params file, plus README.md with timings and the radial-profile fits.
    StampVolcano {
        /// Base guide folder (v1).
        #[arg(long)]
        base: PathBuf,
        /// Params file (TOML: [centre], [reference], [params], [[arm]]).
        #[arg(long)]
        params: PathBuf,
        /// Output folder (one sub-folder per arm).
        #[arg(long)]
        out: PathBuf,
        /// Only these arms (comma-separated names).
        #[arg(long, value_delimiter = ',')]
        arms: Option<Vec<String>>,
    },
    /// Land-only terrain statistics of guide folders at one spacing (`sf_worldgen::measure`): hypsometry, slope,
    /// band roughness, spectrum, drainage density, channel structure, local relief; Markdown + JSON report.
    Measure(Box<MeasureArgs>),
    /// Landform census (`sf_worldgen::landforms`): craters, basins, lakes, lagoons, trenches, gorges, valleys,
    /// peaks, plateaus, bays, headlands, islands; writes features.json, overlay.png and README.md.
    Landforms(LandformsArgs),
    /// Geology map (`sf_worldgen::geology`, LF-1): rock unit, hardness and strata datum per guide sample, written to
    /// `<out>/geology/`.
    Geology {
        /// Base guide folder (v1).
        #[arg(long)]
        base: PathBuf,
        /// Geology table TOML (`[[unit]]`, `[shares]`, `[region."<name>"] shares`, `[values]`).
        #[arg(long)]
        table: PathBuf,
        /// One declared value `name=value`; repeatable.
        #[arg(long = "set")]
        sets: Vec<String>,
        /// Output folder.
        #[arg(long)]
        out: PathBuf,
    },
    /// Placed landforms (`sf_worldgen::placed`, LF-2): fault scarps with talus fans on unit boundaries, thermal
    /// settle, walkability repair ramps; writes a full guide folder (base files copied) + `geology/` +
    /// `features.json` + README.md.
    Place {
        /// Base guide folder (v1).
        #[arg(long)]
        base: PathBuf,
        /// Geology table TOML.
        #[arg(long)]
        geology: PathBuf,
        /// Landforms recipe TOML (`[values]`, `[region."<name>"] weight / face_budget`).
        #[arg(long)]
        recipe: Option<PathBuf>,
        /// One declared value `name=value`; repeatable.
        #[arg(long = "set")]
        sets: Vec<String>,
        /// Output guide folder.
        #[arg(long)]
        out: PathBuf,
        /// Per-world filter file (`steep_fade*` values) copied into the output as `filter.toml`.
        #[arg(long)]
        world_filter: Option<PathBuf>,
    },
    /// Fast steerable macro sharp-landform pass (`sf_worldgen::cliffs`): asymmetric fault scarps, stepped rock
    /// shelves, slope oversteepening, coastal headlands, explicit strokes and hardness-aware thermal talus.
    Cliffs {
        /// Base guide folder (v1).
        #[arg(long)]
        base: PathBuf,
        /// Recipe TOML (`[params]`, `[[stroke]]`, `[[arm]]`).
        #[arg(long)]
        params: PathBuf,
        /// Output folder (one sub-folder per arm, or directly when only one default arm).
        #[arg(long)]
        out: PathBuf,
        /// Optional external cliff-intent mask (`f32` LE grid matching the guide, values in 0..1).
        #[arg(long)]
        mask: Option<PathBuf>,
        /// Only these arms (comma-separated names).
        #[arg(long, value_delimiter = ',')]
        arms: Option<Vec<String>>,
    },
}

#[derive(clap::Args)]
struct LandformsArgs {
    /// Guide folder (v1).
    guide: PathBuf,
    /// Names file (`{format: 1, points: [{name, x_m, y_m, radius_m?, kind_hint?}]}`, guide frame).
    #[arg(long)]
    names: Option<PathBuf>,
    /// Field size over the reference world (sets the declared `scale`).
    #[arg(long)]
    scale: Option<f64>,
    /// Guide-frame metres of sample (0, 0), overriding the guide's `origin_m`.
    #[arg(long, value_delimiter = ',', num_args = 1, allow_negative_numbers = true)]
    origin: Option<Vec<f64>>,
    /// One declared value `name=value` (source Guess); repeatable.
    #[arg(long = "set")]
    sets: Vec<String>,
    /// Output folder.
    #[arg(long)]
    out: PathBuf,
}

fn landforms(a: LandformsArgs) -> sf_worldgen::Result<()> {
    use sf_worldgen::landforms::{LandformsRequest, declared_defaults, run_landforms, set};
    let mut declared = declared_defaults();
    if let Some(s) = a.scale {
        set(&mut declared, "scale", s, Some("Input(--scale)".into()))?;
    }
    for kv in &a.sets {
        let (k, v) = kv
            .split_once('=')
            .ok_or_else(|| sf_worldgen::Error::Invalid(format!("--set {kv}: name=value expected")))?;
        let v: f64 = v
            .trim()
            .parse()
            .map_err(|_| sf_worldgen::Error::Invalid(format!("--set {kv}: not a number")))?;
        set(&mut declared, k.trim(), v, None)?;
    }
    let origin_m = match a.origin.as_deref() {
        None => None,
        Some([x, y]) => Some([*x, *y]),
        Some(_) => return Err(sf_worldgen::Error::Invalid("--origin takes X,Y".into())),
    };
    let command = std::env::args().collect::<Vec<_>>().join(" ").replace('\\', "/");
    let c = run_landforms(&LandformsRequest {
        guide: a.guide,
        names: a.names,
        out: a.out.clone(),
        origin_m,
        declared,
        command,
    })?;
    let named = c.features.iter().filter(|f| f.name.is_some()).count();
    let total: f64 = c.timings.iter().map(|t| t.1).sum();
    println!(
        "{}: {} features ({named} named) in {total:.1} s",
        a.out.display(),
        c.features.len()
    );
    Ok(())
}

#[derive(clap::Args)]
struct MeasureArgs {
    /// Guide folders (v1) to measure.
    dirs: Vec<PathBuf>,
    /// Reference guide folders (measured first; the report's gap section compares the others to their range).
    #[arg(long = "ref")]
    refs: Vec<PathBuf>,
    /// Exclude sea and land within --coast-m of it.
    #[arg(long)]
    mask_sea: bool,
    /// Coastal margin, metres (metric definition, WG-M report: 500).
    #[arg(long, default_value_t = 500.0)]
    coast_m: f64,
    /// Grid-border margin, metres (WG-M report: 1000).
    #[arg(long, default_value_t = 1000.0)]
    edge_m: f64,
    /// Crop `x0,y0,x1,y1` in each guide's frame, metres (x east, y north).
    #[arg(long, value_delimiter = ',', num_args = 1, allow_negative_numbers = true)]
    region: Option<Vec<f64>>,
    /// Measuring spacing, metres (WG-M report: 20).
    #[arg(long, default_value_t = 20.0)]
    spacing: f64,
    /// Cone / lowland split: `auto` (highest valid sample) or `x,y` guide-frame metres.
    #[arg(long, allow_hyphen_values = true)]
    summit: Option<String>,
    /// Cone zone radius around the summit, km (WG-M report: 10).
    #[arg(long, default_value_t = 10.0)]
    cone_km: f64,
    /// Lowland zone: at least this far from the summit, km (WG-M report: 20).
    #[arg(long, default_value_t = 20.0)]
    lowland_km: f64,
    /// Elevation band zone `lo,hi`, metres.
    #[arg(long, value_delimiter = ',', num_args = 1, allow_negative_numbers = true)]
    band: Option<Vec<f64>>,
    /// Spectral patch edge, samples, a power of two (WG-M report: 512).
    #[arg(long, default_value_t = 512)]
    patch: usize,
    /// Walkable slope limit, degrees (declared `max_walk_deg`, default 46, range 20-80).
    #[arg(long)]
    max_walk_deg: Option<f64>,
    /// Base guide folder (LF-0): untouched share (|dh| < 0.5 m) and |dh| percentiles of fields on its samples.
    #[arg(long)]
    base: Option<PathBuf>,
    /// Report file: `.md` (and `.json` beside it) or `.json`.
    #[arg(long)]
    out: PathBuf,
}

fn parse_sets(sets: &[String]) -> sf_worldgen::Result<Vec<(String, f64)>> {
    sets.iter()
        .map(|kv| {
            let (k, v) = kv
                .split_once('=')
                .ok_or_else(|| sf_worldgen::Error::Invalid(format!("--set {kv}: expected name=value")))?;
            let v: f64 = v
                .trim()
                .parse()
                .map_err(|_| sf_worldgen::Error::Invalid(format!("--set {kv}: not a number")))?;
            Ok((k.trim().to_string(), v))
        })
        .collect()
}

fn place(
    base: PathBuf,
    geology: PathBuf,
    recipe: Option<PathBuf>,
    sets: &[String],
    out: PathBuf,
    world_filter: Option<PathBuf>,
) -> sf_worldgen::Result<()> {
    use sf_worldgen::placed::{PlaceRequest, run_place};
    let req = PlaceRequest {
        base,
        geology,
        recipe,
        sets: parse_sets(sets)?,
        out,
        world_filter,
    };
    let (rep, bake_s) = run_place(&req)?;
    let steps: Vec<String> = rep.steps_s.iter().map(|(k, v)| format!("{k} {v:.2}")).collect();
    println!(
        "place: bake {bake_s:.2} s ({}); traces {}, candidates {}, faults {} ({} segments), rejected budget {} \
         spacing {}, water gap {}; throw p50/p90/max {:.0}/{:.0}/{:.0} m; ramps {}; wet/dry {}/{}/{}, frozen moved {}; \
         blake3 {}",
        steps.join(", "),
        rep.traces,
        rep.candidates,
        rep.placed,
        rep.segments,
        rep.rejected_budget,
        rep.rejected_spacing,
        rep.rejected_gap,
        rep.throw_m[0],
        rep.throw_m[1],
        rep.throw_m[2],
        rep.ramps,
        rep.new_wet,
        rep.new_dry,
        rep.sea_moved,
        rep.frozen_moved,
        &rep.hash[..12]
    );
    for r in &rep.regions {
        println!(
            "  {}: faults {} (throw p50/p90/max {:.0}/{:.0}/{:.0} m), face {:.2}/{:.2} km2, cliff {:.2} km2, largest walkable {:.1} -> {:.1} %, ramps {} \
             (failed {})",
            r.region,
            r.faults,
            r.throw_m[0],
            r.throw_m[1],
            r.throw_m[2],
            r.face_km2,
            r.budget_km2,
            r.cliff_km2,
            100.0 * r.route.largest_before,
            100.0 * r.route.largest_after,
            r.route.ramps,
            r.route.failed
        );
    }
    Ok(())
}

fn geology(base: PathBuf, table: PathBuf, sets: &[String], out: PathBuf) -> sf_worldgen::Result<()> {
    use sf_worldgen::geology::{GeologyRequest, run_geology};
    let sets = parse_sets(sets)?;
    run_geology(&GeologyRequest {
        base,
        table,
        sets,
        out,
    })
    .map(|_| ())
}

fn measure(a: MeasureArgs) -> sf_worldgen::Result<()> {
    use sf_worldgen::measure::{MeasureParams, MeasureRequest, Summit, run_measure};
    let bad = |m: &str| sf_worldgen::Error::Invalid(m.into());
    let mut p = MeasureParams::standard(a.spacing);
    p.mask_sea = a.mask_sea;
    p.coast_m = a.coast_m;
    p.edge_m = a.edge_m;
    p.cone_m = a.cone_km * 1000.0;
    p.lowland_m = a.lowland_km * 1000.0;
    p.patch = a.patch;
    if let Some(v) = a.max_walk_deg {
        let d = &sf_worldgen::measure::declared_walk()[0];
        if !(d.min..=d.max).contains(&v) {
            return Err(bad(&format!("--max-walk-deg {v} outside [{}, {}]", d.min, d.max)));
        }
        p.max_walk_deg = v;
    }
    p.region_m = match a.region.as_deref() {
        None => None,
        Some(&[x0, y0, x1, y1]) => Some([x0, y0, x1, y1]),
        Some(_) => return Err(bad("--region takes x0,y0,x1,y1")),
    };
    p.band_m = match a.band.as_deref() {
        None => None,
        Some(&[lo, hi]) => Some([lo, hi]),
        Some(_) => return Err(bad("--band takes lo,hi")),
    };
    p.summit = match a.summit.as_deref() {
        None => None,
        Some("auto") => Some(Summit::Auto),
        Some(xy) => {
            let parts: Vec<f64> = xy.split(',').filter_map(|t| t.trim().parse().ok()).collect();
            let [x, y] = parts[..] else {
                return Err(bad("--summit takes auto or x,y"));
            };
            Some(Summit::At([x, y]))
        }
    };
    run_measure(&MeasureRequest {
        refs: a.refs,
        dirs: a.dirs,
        params: p,
        base: a.base,
        out: a.out,
    })
    .map(|_| ())
}

#[allow(clippy::cast_precision_loss)] // sample count in millions, for the report line
fn detail(
    input: PathBuf,
    out: PathBuf,
    spacing: f64,
    crop: Option<Vec<f64>>,
    params: Option<PathBuf>,
    sets: &[String],
) -> sf_worldgen::Result<()> {
    let crop_m = match crop.as_deref() {
        None => None,
        Some(&[x0, y0, x1, y1]) => Some([x0, y0, x1, y1]),
        Some(_) => {
            return Err(sf_worldgen::Error::Invalid(
                "--crop takes x0,y0,x1,y1".into(),
            ));
        }
    };
    let mut declared = declared_defaults();
    if let Some(p) = &params {
        apply_overrides_toml(&mut declared, &std::fs::read_to_string(p)?)?;
    }
    for kv in sets {
        let (k, v) = kv
            .split_once('=')
            .ok_or_else(|| sf_worldgen::Error::Invalid(format!("--set {kv}: name=value expected")))?;
        let v: f64 = v
            .trim()
            .parse()
            .map_err(|_| sf_worldgen::Error::Invalid(format!("--set {kv}: not a number")))?;
        set(&mut declared, k.trim(), v, None)?;
    }
    let req = DetailRequest {
        input,
        out: out.clone(),
        spacing_m: spacing,
        crop_m,
        declared,
    };
    let r = run_detail(&req)?;
    let s = &r.stats;
    println!(
        "{}: {}x{} @ {} m ({:.2} M samples), origin ({:.1}, {:.1}) m; read {:.2} s, filter {:.2} s, write {:.2} s; |dh| mean {:.3} m, p99 {:.3} m, max {:.3} m, mean dh {:.3} m, changed {:.1} %, new wet {}, new dry {}",
        out.display(),
        r.dims[0],
        r.dims[1],
        r.spacing_m,
        r.samples as f64 * 1e-6,
        r.origin_m[0],
        r.origin_m[1],
        r.seconds_read,
        r.seconds_filter,
        r.seconds_write,
        s.mean_abs_m,
        s.p99_abs_m,
        s.max_abs_m,
        s.mean_m,
        s.changed_share * 100.0,
        s.new_wet,
        s.new_dry
    );
    Ok(())
}

fn stamp_volcano(req: StampRequest) -> ExitCode {
    match run_stamp(&req) {
        Ok(r) => {
            println!(
                "stamp-volcano: {} arms, summit {}",
                r.arms.len(),
                r.centre_rule
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("scrib-worldgen: {e}");
            ExitCode::FAILURE
        }
    }
}

// One arm per subcommand; the dispatch reads best in one place.
#[allow(clippy::too_many_lines)]
fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Landforms(a) => match landforms(a) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("scrib-worldgen: {e}");
                ExitCode::FAILURE
            }
        },
        Cmd::Geology {
            base,
            table,
            sets,
            out,
        } => match geology(base, table, &sets, out) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("scrib-worldgen: {e}");
                ExitCode::FAILURE
            }
        },
        Cmd::Place {
            base,
            geology,
            recipe,
            sets,
            out,
            world_filter,
        } => match place(base, geology, recipe, &sets, out, world_filter) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("scrib-worldgen place: {e}");
                ExitCode::FAILURE
            }
        },
        Cmd::Measure(a) => match measure(*a) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("scrib-worldgen: {e}");
                ExitCode::FAILURE
            }
        },
        Cmd::StampVolcano {
            base,
            params,
            out,
            arms,
        } => stamp_volcano(StampRequest {
            base,
            params,
            out,
            arms,
        }),
        Cmd::Cliffs {
            base,
            params,
            out,
            mask,
            arms,
        } => match run_cliffs(&CliffRequest {
            base,
            params_path: params,
            out,
            mask_path: mask,
            arms,
        }) {
            Ok(summary) => {
                print!("{summary}");
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("scrib-worldgen: {e}");
                ExitCode::FAILURE
            }
        },
        Cmd::Detail {
            input,
            out,
            spacing,
            crop,
            params,
            sets,
        } => match detail(input, out, spacing, crop, params, &sets) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("scrib-worldgen: {e}");
                ExitCode::FAILURE
            }
        },
        Cmd::Run {
            recipe,
            arms,
            macro_spacing,
            iterations,
            out,
            no_images,
        } => {
            let ov = Overrides {
                arms,
                macro_spacing_m: macro_spacing,
                iterations,
                out,
                no_images,
            };
            match run(&recipe, &ov) {
                Ok(_) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("scrib-worldgen: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        Cmd::ImportDem {
            tiles,
            centre,
            extent_km,
            spacing,
            out,
            fill,
            sea_max,
            name,
            source,
            licence,
        } => {
            let (&[lat, lon], &[ew, ref ns @ ..]) = (centre.as_slice(), extent_km.as_slice()) else {
                eprintln!("scrib-worldgen: --centre takes lat,lon and --extent-km takes ew[,ns]");
                return ExitCode::FAILURE;
            };
            let ns = ns.first().copied().unwrap_or(ew);
            let req = ImportRequest {
                inputs: tiles,
                params: ImportParams {
                    centre_lat: lat,
                    centre_lon: lon,
                    extent_m: [ew * 1000.0, ns * 1000.0],
                    spacing_m: spacing,
                    sea_max_m: sea_max,
                    fill_m: fill,
                },
                out: out.clone(),
                name,
                source,
                licence,
            };
            match run_import(&req) {
                Ok(imp) => {
                    let s = &imp.stats;
                    println!(
                        "{}: {}x{} @ {} m, min {:.1} m, max {:.1} m at local ({:.0}, {:.0}) m = ({:.5}, {:.5}), land {:.1} % ({:.1} km2)",
                        out.display(),
                        imp.w,
                        imp.h,
                        imp.spacing_m,
                        s.min_m,
                        s.max_m,
                        s.peak_local_m[0],
                        s.peak_local_m[1],
                        s.peak_lat_lon[0],
                        s.peak_lat_lon[1],
                        s.land_share * 100.0,
                        s.land_km2
                    );
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("scrib-worldgen: {e}");
                    ExitCode::FAILURE
                }
            }
        }
    }
}
