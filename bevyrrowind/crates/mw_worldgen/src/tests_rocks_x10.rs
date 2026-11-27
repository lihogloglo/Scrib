//! WG-E3m: the rock layer measured on the real 10× guide (`target/worldgen/x10s`, or `SCRIB_X10S`).
// Counts of a few million rocks and quantile indices: well inside f64's exact range.
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::time::Instant;

use sf_worldgen::guide::Guide;
use sf_worldgen::scatter::Params;
use sf_worldgen::scatter_species::RockContext;
use sf_worldgen::tiles::GuideTiles;

use crate::flora::rocks::{RockMesh, RockSource};
use crate::flora::towns::TownMask;
use crate::flora::{self, FloraRun, ProductUse};
use crate::geology::AdapterGeology;
use crate::tests_flora::x10s_copy;

const CONTEXTS: [RockContext; 3] = [RockContext::Talus, RockContext::Outcrop, RockContext::Field];

/// WG-E3m: the rock layer on x10s, copied to a temp folder with its climate product (copied or made) and a geology
/// product made there from the shipped table (`geology.toml` resolved in the guide's palette), so talus reads the
/// mapped cliffs. Three passes: without rocks (product written), with rocks (product written: the first-load seconds
/// difference is the rock layer's cost, the vegetation's competition against rock footprints and the calibration's
/// rock census included), with rocks and no budget (product read: the full rock set). Prints per geology unit its
/// dry land and rocks/ha per context with the pack-model share; per context the count, pack models / library, size
/// quantiles and the fitted size exponent D (truncated Pareto MLE) beside the declared `scatter.rock.size_dim`; the
/// seconds and `rocks.bin` MB. No towns (as the `worldgen-flora` census).
#[test]
#[ignore = "measurement: rocks"]
#[allow(clippy::too_many_lines)]
fn rocks_on_x10s() {
    let Ok((install, _)) = mw_install::MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let src = std::env::var_os("SCRIB_X10S").map_or_else(|| root.join("target/worldgen/x10s"), PathBuf::from);
    if !src.join("heights.f32").exists() {
        eprintln!("no guide at {}; skipping", src.display());
        return;
    }
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("guide");
    std::fs::create_dir_all(&dir).unwrap();
    let (climate, climate_s) = x10s_copy(&src, &dir);

    let geology_s = make_geology(&root, tmp.path(), &dir);

    let t = Instant::now();
    let vfs = mw_vfs::mount_install(&install, sf_vfs::Vfs::builder()).unwrap().vfs;
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    let load_s = t.elapsed().as_secs_f64();
    let cache = std::cell::RefCell::new(HashMap::<String, Option<([f32; 3], [f32; 3])>>::new());
    let bounds = |model: &str| {
        if let Some(b) = cache.borrow().get(model) {
            return *b;
        }
        let b = sf_vfs::VfsPath::new(model)
            .ok()
            .and_then(|path| vfs.read(&path).ok())
            .and_then(|bytes| mw_nif::load(&bytes).ok())
            .and_then(|m| mw_nif::visible_bounds(&m));
        cache.borrow_mut().insert(model.to_owned(), b);
        b
    };
    let tiles = GuideTiles::open(&dir, mw_world::ids::exterior()).unwrap();
    let source = mw_world::MwWorldSource::build(&content.db, &mw_world::SourceOptions::default());
    let prm = Params::default();
    let (file, f, water, clearings) = flora::guide_inputs(&dir, &tiles, &prm).unwrap();
    let rs = RockSource::read(&dir).unwrap();
    let grid = rs.grid.as_ref().expect("the geology product just made");
    let towns = TownMask::default();
    let pass = |prm: Params, rock: Option<&RockSource>| {
        let t = Instant::now();
        let run = flora::scatter_world(&content.db, &source, &bounds, &tiles, &f, &water, &file, clearings.as_ref(), &towns, prm, Some(&dir), rock)
            .unwrap();
        (run, t.elapsed().as_secs_f64())
    };
    let (bare, bare_s) = pass(prm, None);
    let (first, first_s) = pass(prm, Some(&rs));
    let rocks_mb = std::fs::metadata(dir.join("scatter/rocks.bin")).map_or(0, |m| m.len()) as f64 / 1e6;
    let (full, full_s) = pass(Params { budget: f64::from(u32::MAX), ..prm }, Some(&rs));
    assert_eq!(bare.product, ProductUse::Wrote, "a fresh copy has no product");
    assert_eq!(first.product, ProductUse::Wrote, "the rock layer is in the product hash");
    assert_eq!(full.product, ProductUse::Read, "a budget change reads the product");
    assert!(bare.rocks.is_empty(), "no rock source, no rocks");
    assert!(!full.rocks.is_empty(), "the rock layer places rocks on x10s");
    assert!(full.rocks.iter().all(|r| r.pos.iter().all(|v| v.is_finite()) && r.scale.is_finite() && r.scale > 0.0));

    println!(
        "WG-E3m rocks on {} (climate {climate} in {climate_s:.1} s, geology made in {geology_s:.1} s, content {load_s:.1} s)",
        src.display()
    );
    println!("no rocks:   {}", flora::summary_line(&bare));
    println!("rocks:      {}", flora::summary_line(&first));
    println!("no budget:  {}", flora::summary_line(&full));
    println!(
        "first load seconds: no rocks {bare_s:.1}, rocks {first_s:.1} (rock layer +{:.1}), product read {full_s:.1}; rocks.bin {rocks_mb:.1} MB",
        first_s - bare_s
    );
    unit_table(&full, grid, &dir);
    let values = flora::rocks::values_of(&full.learned.file).unwrap();
    context_table(&full, values.size_dim);
}

/// The geology product in `dir`, from the shipped table resolved in the guide's palette (written to `tmp`, outside
/// the guide folder, so the rock layer reads the shipped table as the game does); returns its seconds.
fn make_geology(root: &Path, tmp: &Path, dir: &Path) -> f64 {
    let t = Instant::now();
    let text = std::fs::read_to_string(root.join("bevyrrowind/worldgen").join(crate::geology::TABLE_FILE)).unwrap();
    let adapter = AdapterGeology::from_toml(&text).unwrap();
    let layers = sf_worldgen::layers::GuideLayers::read(dir).unwrap().expect("x10s has layers.json");
    let table = tmp.join("geology.resolved.toml");
    std::fs::write(&table, adapter.resolved_toml(&layers.palette).unwrap()).unwrap();
    let req = sf_worldgen::geology::GeologyRequest { base: dir.to_owned(), table, sets: Vec::new(), out: dir.to_owned() };
    sf_worldgen::geology::run_geology(&req).unwrap();
    t.elapsed().as_secs_f64()
}

/// Library seed of the stand-in libraries [`rock_placements_on_x10s`] builds (the bake's per-class seeds sit behind
/// `sf_rockgen`'s `bake` feature, which this crate does not take): counts do not depend on it, variant bounds do a
/// little.
// tunables: const (fixture seed)
const STAND_IN_SEED: u64 = 1;

/// RG-P6 (rockgen.md "(f)"): the library rocks on x10s as the `view world:` load makes them. One scatter pass with
/// the default parameters and the rock layer over a geology product made from the shipped table (no towns, as the
/// `worldgen-flora` census); libraries of every built-in class at the `rockgen.*` defaults (stand-in seed,
/// [`STAND_IN_SEED`]). Prints the library rocks the scatter marks, the placements resolved and dropped, per class
/// placements and variants used; the content index (`build_pages` + `build_directory` at the default
/// `IndexParams`, encoded bytes = `IndexPage::to_bytes` + `CellDirectory::to_bytes`) over the flora and pack-rock
/// lists without and with the library list (seconds, MB, placements, cells, pages; models other than library
/// variants index as points in both arms: no bake here); library rocks per 1 km ring (0-1, 1-2, 2-3 km, horizontal)
/// around three points: the centres of the densest, the p90 and the p50 occupied square kilometre.
#[test]
#[ignore = "measurement: rock placements"]
#[allow(clippy::too_many_lines)]
fn rock_placements_on_x10s() {
    use sf_rockgen::{BoulderParams, CLASSES, Libraries, LibrarySettings, build_library};
    use sf_world::{IndexParams, Placement, build_directory, build_pages};

    let Ok((install, _)) = mw_install::MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let src = std::env::var_os("SCRIB_X10S").map_or_else(|| root.join("target/worldgen/x10s"), PathBuf::from);
    if !src.join("heights.f32").exists() {
        eprintln!("no guide at {}; skipping", src.display());
        return;
    }
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("guide");
    std::fs::create_dir_all(&dir).unwrap();
    let _ = x10s_copy(&src, &dir);
    make_geology(&root, tmp.path(), &dir);

    let vfs = mw_vfs::mount_install(&install, sf_vfs::Vfs::builder()).unwrap().vfs;
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    let cache = std::cell::RefCell::new(HashMap::<String, Option<([f32; 3], [f32; 3])>>::new());
    let bounds = |model: &str| {
        if let Some(b) = cache.borrow().get(model) {
            return *b;
        }
        let b = sf_vfs::VfsPath::new(model)
            .ok()
            .and_then(|path| vfs.read(&path).ok())
            .and_then(|bytes| mw_nif::load(&bytes).ok())
            .and_then(|m| mw_nif::visible_bounds(&m));
        cache.borrow_mut().insert(model.to_owned(), b);
        b
    };
    let ext = mw_world::ids::exterior();
    let tiles = GuideTiles::open(&dir, ext.clone()).unwrap();
    let source = mw_world::MwWorldSource::build(&content.db, &mw_world::SourceOptions::default());
    let prm = Params::default();
    let (file, f, water, clearings) = flora::guide_inputs(&dir, &tiles, &prm).unwrap();
    let rs = RockSource::read(&dir).unwrap();
    let towns = TownMask::default();
    let t = Instant::now();
    let run = flora::scatter_world(&content.db, &source, &bounds, &tiles, &f, &water, &file, clearings.as_ref(), &towns, prm, Some(&dir), Some(&rs))
        .unwrap();
    let scatter_s = t.elapsed().as_secs_f64();

    let t = Instant::now();
    let libs = Libraries::new(CLASSES.iter().map(|c| {
        (c.name.to_owned(), build_library(&c.params(BoulderParams::default()), &LibrarySettings::default(), STAND_IN_SEED))
    }));
    let libs_s = t.elapsed().as_secs_f64();
    let marked = run.rocks.iter().filter(|r| matches!(r.mesh, RockMesh::Class { .. })).count();
    let t = Instant::now();
    let lib = flora::rocks::library_placements(&run, &libs);
    let resolve_s = t.elapsed().as_secs_f64();
    assert_eq!(lib.placements.len() + lib.dropped, marked, "every marked library rock is placed or dropped");
    assert!(lib.placements.iter().all(|p| p.transform.pos.to_f64().is_finite() && p.transform.scale.is_finite()));

    let base: Vec<Placement> = flora::placements(&run).into_iter().chain(flora::rocks::rock_placements(&run)).collect();
    let models = |m: &sf_vfs::VfsPath| lib.models.get(m).copied();
    let params = IndexParams::default();
    let index = |with: bool| {
        let t = Instant::now();
        let all = base.iter().cloned().chain(lib.placements.iter().filter(|_| with).cloned());
        let (pages, stats) = build_pages(&params, all, &models);
        let directory = build_directory(ext.clone(), &params, &pages);
        let secs = t.elapsed().as_secs_f64();
        let bytes = pages.iter().map(|p| p.to_bytes().len()).sum::<usize>() + directory.to_bytes().len();
        (stats, secs, bytes as f64 / 1e6)
    };
    let (without, without_s, without_mb) = index(false);
    let (with, with_s, with_mb) = index(true);
    assert!(with.duplicate_ids.is_empty(), "library rock ids are unique among the content");
    assert_eq!(with.missing_models, without.missing_models, "every library model indexes with its variant bounds");

    println!(
        "RG-P6 rock placements on {} (scatter {scatter_s:.1} s, {} libraries built in {libs_s:.2} s)",
        src.display(),
        libs.len()
    );
    println!("scatter: {}", flora::summary_line(&run));
    println!(
        "library rocks: {marked} marked, {} placed, {} dropped, {} variants used; resolve {resolve_s:.3} s",
        lib.placements.len(),
        lib.dropped,
        lib.models.len()
    );
    let mut per_class: BTreeMap<&str, (usize, std::collections::BTreeSet<&str>)> = BTreeMap::new();
    for p in &lib.placements {
        let id = p.model.as_ref().unwrap().as_str();
        let (class, _) = sf_rockgen::placed::parse_model_id(id).unwrap();
        let e = per_class.entry(class).or_default();
        e.0 += 1;
        e.1.insert(id);
    }
    println!("| class | placements | variants used |");
    println!("|---|---:|---:|");
    for (class, (n, ids)) in &per_class {
        println!("| {class} | {n} | {} |", ids.len());
    }
    println!("| index | placements | cells | pages | build s | MB |");
    println!("|---|---:|---:|---:|---:|---:|");
    for (name, s, secs, mb) in [("without library", &without, without_s, without_mb), ("with library", &with, with_s, with_mb)] {
        println!("| {name} | {} | {} | {} | {secs:.2} | {mb:.1} |", s.placements, s.cells, s.pages);
    }
    println!("index with library: +{:.2} s, +{:.1} MB", with_s - without_s, with_mb - without_mb);
    rings_table(&lib.placements);
}

/// Library rocks per 1 km ring (horizontal, engine x / z) around the centres of the densest, the p90 and the p50
/// occupied square kilometre.
fn rings_table(placements: &[sf_world::Placement]) {
    // tunables: const (the measurement's ring width, km in m)
    const KM: f64 = 1000.0;
    let xz: Vec<(f64, f64)> = placements.iter().map(|p| p.transform.pos.to_f64()).map(|v| (v.x, v.z)).collect();
    let mut squares: HashMap<(i64, i64), usize> = HashMap::new();
    for &(x, z) in &xz {
        *squares.entry(((x / KM).floor() as i64, (z / KM).floor() as i64)).or_default() += 1;
    }
    let mut occupied: Vec<((i64, i64), usize)> = squares.into_iter().collect();
    if occupied.is_empty() {
        println!("no library rocks: no rings");
        return;
    }
    occupied.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    println!("{} occupied km2", occupied.len());
    println!("| point | centre x / z m | rocks in its km2 | 0-1 km | 1-2 km | 2-3 km |");
    println!("|---|---|---:|---:|---:|---:|");
    for (name, q) in [("densest", 0.0), ("p90", 0.1), ("p50", 0.5)] {
        let ((sx, sz), n) = occupied[((occupied.len() - 1) as f64 * q).round() as usize];
        let (cx, cz) = ((sx as f64 + 0.5) * KM, (sz as f64 + 0.5) * KM);
        let mut rings = [0usize; 3];
        for &(x, z) in &xz {
            if let Some(r) = rings.get_mut(((x - cx).hypot(z - cz) / KM).floor() as usize) {
                *r += 1;
            }
        }
        println!("| {name} | {cx:.0} / {cz:.0} | {n} | {} | {} | {} |", rings[0], rings[1], rings[2]);
    }
}

/// Per geology unit: dry land (guide samples above sea level), rocks/ha per context and all, pack-model share.
fn unit_table(run: &FloraRun, grid: &sf_worldgen::geology::GeologyGrid, dir: &Path) {
    let g = Guide::read(dir).unwrap();
    let cell_ha = g.spacing_m * g.spacing_m / 1e4;
    let n = grid.units.len();
    let mut land = vec![0.0f64; n];
    for (i, &h) in g.heights.iter().enumerate() {
        if f64::from(h) > g.sea_level_m {
            land[usize::from(grid.unit[i]).min(n - 1)] += cell_ha;
        }
    }
    let index: BTreeMap<&str, usize> = grid.units.iter().enumerate().map(|(i, u)| (u.as_str(), i)).collect();
    // [unit (n = none)][context], and pack models per unit.
    let mut count = vec![[0usize; 3]; n + 1];
    let mut models = vec![0usize; n + 1];
    for r in &run.rocks {
        let u = grid.unit_at(r.pos[0], r.pos[1]).and_then(|u| index.get(u).copied()).unwrap_or(n);
        count[u][r.key.context as usize] += 1;
        models[u] += usize::from(matches!(r.mesh, RockMesh::Model(_)));
    }
    println!("| unit | dry land km2 | talus /ha | outcrop /ha | field /ha | all /ha | pack-model share |");
    println!("|---|---:|---:|---:|---:|---:|---:|");
    for u in 0..=n {
        let all: usize = count[u].iter().sum();
        let ha = land.get(u).copied().unwrap_or(0.0);
        if ha == 0.0 && all == 0 {
            continue;
        }
        let name = grid.units.get(u).map_or("outside", String::as_str);
        let per = |c: usize| if ha > 0.0 { c as f64 / ha } else { f64::NAN };
        let share = if all > 0 { 100.0 * models[u] as f64 / all as f64 } else { f64::NAN };
        println!(
            "| {name} | {:.1} | {:.2} | {:.2} | {:.2} | {:.2} | {share:.0} % |",
            ha / 100.0,
            per(count[u][0]),
            per(count[u][1]),
            per(count[u][2]),
            per(all)
        );
    }
}

/// Per context: count, pack models / library, size quantiles, fitted D against the declared one.
fn context_table(run: &FloraRun, declared_d: f64) {
    println!("| context | rocks | pack models | library | size p10 / p50 / p90 / max m | fitted D | declared D |");
    println!("|---|---:|---:|---:|---|---:|---:|");
    for ctx in CONTEXTS {
        let rocks: Vec<_> = run.rocks.iter().filter(|r| r.key.context == ctx).collect();
        if rocks.is_empty() {
            println!("| {ctx:?} | 0 | | | | | {declared_d:.2} |");
            continue;
        }
        let models = rocks.iter().filter(|r| matches!(r.mesh, RockMesh::Model(_))).count();
        let mut sizes: Vec<f64> = rocks.iter().map(|r| r.size_m).collect();
        sizes.sort_by(f64::total_cmp);
        let q = |p: f64| sizes[((sizes.len() - 1) as f64 * p).round() as usize];
        let row = run.learned.file.rocks.get(&ctx);
        let lo = row.map_or(sizes[0], |r| r.size_min_m).min(sizes[0]);
        let hi = row.map_or(sizes[sizes.len() - 1], |r| r.size_max_m).max(sizes[sizes.len() - 1]);
        println!(
            "| {ctx:?} | {} | {models} | {} | {:.2} / {:.2} / {:.2} / {:.2} | {:.2} | {declared_d:.2} |",
            sizes.len(),
            sizes.len() - models,
            q(0.1),
            q(0.5),
            q(0.9),
            sizes[sizes.len() - 1],
            fit_d(&sizes, lo, hi)
        );
    }
}

/// Maximum-likelihood exponent of a Pareto law truncated to `[lo, hi]` (pdf `D lo^D s^(-D-1) / (1 - (lo/hi)^D)`):
/// the root of the score `n/D + n ln lo - sum ln s + n r^D ln r / (1 - r^D)`, `r = lo / hi`, which falls with D.
fn fit_d(sizes: &[f64], lo: f64, hi: f64) -> f64 {
    let count = sizes.len() as f64;
    let sum_ln: f64 = sizes.iter().map(|s| s.ln()).sum();
    let ratio = lo / hi;
    let score = |d: f64| {
        let rd = ratio.powf(d);
        count / d + count * lo.ln() - sum_ln + count * rd * ratio.ln() / (1.0 - rd)
    };
    let (mut below, mut above) = (1e-3, 20.0);
    if score(below) <= 0.0 {
        return below;
    }
    if score(above) >= 0.0 {
        return above;
    }
    for _ in 0..100 {
        let mid = f64::midpoint(below, above);
        if score(mid) > 0.0 {
            below = mid;
        } else {
            above = mid;
        }
    }
    f64::midpoint(below, above)
}

/// The fit recovers the exponent of sizes drawn from the scatter's own law.
#[test]
fn fit_d_recovers_the_law_exponent() {
    let (lo, hi, d) = (0.4, 12.0, 1.8);
    let sizes: Vec<f64> =
        (0..20_000).map(|i| sf_worldgen::scatter_rocks::pareto_size(lo, hi, d, (f64::from(i) + 0.5) / 20_000.0)).collect();
    let fit = fit_d(&sizes, lo, hi);
    assert!((fit - d).abs() < 0.02, "fitted {fit}, drawn {d}");
}
