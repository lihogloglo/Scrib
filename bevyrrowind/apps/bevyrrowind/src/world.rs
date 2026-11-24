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
//!
//! S3b: a guide carries its own mods (`<guide>/mods/<name>/`, native mod folders such as `worldgen-town`'s): a
//! town's placements only fit the terrain they were planned and graded on, so they load with that guide only, never
//! in a normal run (`guide_towns`). Their record files apply into the loaded content's db under one source each,
//! after the profile, rather than joining the profile's load order: the profile (and its fingerprint, which keys the
//! bake and saves) stays the player's. Their asset roots are not mounted (a town places the content's own models).
//! Every `sf.placement_group` of the exterior they define streams as one `extra` entry `town:<group id>`.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use mw_environment_bevy::water::WaterInputs;
use mw_devtools_bevy::perf::{Off, Part};
use mw_exterior_bevy::worldgen::{CLOUD_BASE_LIFT_M, WORLDGEN_SHARP, WorldgenSetup, WorldgenTiles};
use mw_exterior_bevy::worldgen_roads::{GroundHeight, RoadTextures, WorldgenRoads, is_paved};
use mw_install::MorrowindInstall;
use mw_world::convert::UNIT_METERS_F64;
use mw_worldgen::classify::Class;
use mw_worldgen::town::ROADS_DIR;
use sf_bake::{Manifest, Store};
use sf_records::{RecordDb, RecordFormat, SourceId, apply_record_file};
use sf_world::{Placement, PlacementGroup};
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
    let (vfs, mut content) = mw_boot_bevy::load(install, false)?;
    let vfs = with_guide_mods(flags.delight(install, vfs)?, path).map_err(|e| format!("world:{dir}: {e}"))?;
    println!("content loaded in {:.2} s", started.elapsed().as_secs_f32());
    // The exterior worldspace with the given camera (the `Interior` start only carries them: `exterior_options`
    // would read the content's ground for a place).
    let start = crate::StreamStart::Interior { worldspace: ext, eye, target };
    let mut options = flags.exterior_options(install, &content.db, format!("Bevyrrowind - world {dir}"), start)?;
    // The shore bake reads the content's coast: the generated world has open water only (neutral optics), plus its
    // guide's lakes and rivers (`water.json`, GW-3).
    let guide_water = sf_worldgen::water::GuideWater::read(path).map_err(|e| format!("world:{dir}: water.json: {e}"))?;
    options.water = WaterInputs::default().with_guide_water(guide_water);
    if let Some(w) = &mut options.weather {
        w.load_sounds(&content.db, &vfs);
    }
    let base = with_exterior_layers(tiles, &options.store, &options.manifest, &mw_world::ids::exterior(), dir)?;
    // The shown world starts at the console's `worldgen.sharp` default (0 = sharp operators off, owner 2026-11-22);
    // `base` keeps the full sharp profile, so the live rebuild reaches `worldgen.sharp 1` / `2` as before.
    let tiles = Arc::new(base.with_params(base.params().with_sharp_scale(WORLDGEN_SHARP.default_value())));
    let towns = guide_towns(&mut content.db, path).map_err(|e| format!("world:{dir}: {e}"))?;
    for t in &towns {
        println!("worldgen town: {}: {} placements, {} unresolved bases", t.name, t.placements.len(), t.unresolved);
    }
    let anchors = AnchorTransform::read(path).map_err(|e| format!("world:{dir}: {e}"))?;
    let mut world_content =
        anchors.map(|a| moved_content(&content.db, &a, &tiles, &towns, &options.store, &options.manifest, flora_budget));
    if !towns.is_empty() {
        let c = world_content.get_or_insert_with(Default::default);
        c.extra.extend(towns.into_iter().map(|t| (t.name, t.placements)));
    }
    // GR-1b: the guide's `roads.json` (and S1b: the mods' networks in `ROADS_DIR`, guide mods included) as draped
    // ribbons on the shown surface, unless `--off roads`.
    let roads = if options.perf.off.has(Part::Roads) {
        None
    } else {
        let textures = road_textures(&content.db);
        println!(
            "worldgen road textures: paved {}, unpaved {}",
            textures.paved.as_deref().unwrap_or("none"),
            textures.unpaved.as_deref().unwrap_or("none")
        );
        let ground = tiles.clone();
        let height: GroundHeight = Arc::new(move |x, z| f64::from(ground.height(x, z, 0)));
        Some(WorldgenRoads { guide_dir: path.to_owned(), mod_roads_dir: Some(ROADS_DIR.into()), height, textures })
    };
    options.actors = Some(Arc::new(content.db));
    let report = tiles.clone();
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
        roads,
    });
    mw_exterior_bevy::exterior::run(Arc::new(vfs), options)
}

/// GR-1b: the road ribbons' textures, from the content's road `LTEX` records (`worldgen-site.toml` `[roads]`,
/// `mw_worldgen::site::RoadTable`), first by file name: a paved one (cobble, stone) for paved surfaces, a dirt
/// road one (else a path, else any road) for the rest.
fn road_textures(db: &RecordDb) -> RoadTextures {
    let table = match mw_worldgen::site::SiteTable::builtin() {
        Ok(t) => t.roads,
        Err(e) => {
            eprintln!("warning: worldgen roads: {e}; untextured roads");
            return RoadTextures::default();
        }
    };
    let mut files: Vec<String> = db
        .iter::<tes3::esp::LandscapeTexture>()
        .filter(|(_, l)| table.is_road(&l.id, &l.file_name))
        .map(|(_, l)| l.file_name.to_ascii_lowercase())
        .collect();
    files.sort();
    files.dedup();
    let find = |keys: &[&str], not: &[&str]| {
        files.iter().find(|f| keys.iter().any(|k| f.contains(k)) && !not.iter().any(|k| f.contains(k))).cloned()
    };
    let paved = files.iter().find(|f| is_paved(f)).cloned();
    let unpaved = find(&["road", "dirt"], &["cobble", "stone"]).or_else(|| find(&["path"], &[])).or_else(|| files.first().cloned());
    RoadTextures { paved: paved.map(|f| mw_nif::texture_path(&f)), unpaved: unpaved.map(|f| mw_nif::texture_path(&f)) }
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

/// The content's exterior references moved through `anchors` (`world_content`), with their log lines; the flora pass
/// keeps out of the guide's `towns` too.
fn moved_content(
    db: &RecordDb,
    anchors: &AnchorTransform,
    tiles: &GuideTiles,
    towns: &[GuideTown],
    store: &Store,
    manifest: &Manifest,
    flora_budget: Option<f64>,
) -> mw_exterior_bevy::worldgen::WorldgenContent {
    let started = Instant::now();
    let source = mw_world::MwWorldSource::build(db, &mw_world::SourceOptions::default());
    let ext = mw_world::ids::exterior();
    // The classification only names what's kept inside footprints and what's dropped (the census); it needs the
    // models' bounds for rocks (`mw_worldgen::classify`).
    // The towns' models too: the flora pass excludes their footprints.
    let models: Vec<sf_vfs::VfsPath> = source
        .all_placements(&ext)
        .chain(towns.iter().flat_map(|t| &t.placements))
        .filter_map(|p| p.model.clone())
        .collect();
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
    let refs: Vec<(&Placement, Class)> = source.all_placements(&ext).zip(classes).collect();
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
    let town_sets: Vec<&[Placement]> = towns.iter().map(|t| t.placements.as_slice()).collect();
    let extra = mw_worldgen::flora::extra(db, &source, &bounds, tiles, &town_sets, flora_budget);
    mw_exterior_bevy::worldgen::WorldgenContent { placements, extra }
}

/// A guide's own mods: `<guide>/mods/<name>/mod.toml` (S3b).
pub(crate) const GUIDE_MODS_DIR: &str = "mods";

/// `vfs` with the asset roots of the guide's own mods (`<guide>/mods/*/`, folder-name order; within a mod its
/// `[content] assets` order) mounted on top, labelled `guide:<mod id>:<root>`, as `sf_mods` mounts a mod's roots.
/// No `mods/` folder: `vfs` unchanged.
pub(crate) fn with_guide_mods(mut vfs: sf_vfs::Vfs, guide: &Path) -> Result<sf_vfs::Vfs, String> {
    let root = guide.join(GUIDE_MODS_DIR);
    let read = match std::fs::read_dir(&root) {
        Ok(r) => r,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(vfs),
        Err(e) => return Err(format!("{}: {e}", root.display())),
    };
    let mut dirs: Vec<PathBuf> =
        read.filter_map(Result::ok).map(|e| e.path()).filter(|p| p.join(sf_mods::MANIFEST_FILE).is_file()).collect();
    dirs.sort();
    for dir in &dirs {
        let manifest = sf_mods::ModManifest::load_dir(dir).map_err(|e| e.to_string())?;
        for asset_root in &manifest.content.assets {
            let path = dir.join(asset_root);
            if !path.is_dir() {
                continue;
            }
            let label = format!("guide:{}:{}", manifest.id.as_str(), asset_root.display());
            let source = sf_vfs::DirSource::scan(label, &path).map_err(|e| format!("{}: {e}", path.display()))?;
            vfs = vfs.with_top_layer(Arc::new(source));
        }
    }
    Ok(vfs)
}

/// One town of a guide's mods: its `extra` entry name, its placements as stored, and how many of them name a base
/// record the content lacks.
pub(crate) struct GuideTown {
    pub name: String,
    pub placements: Vec<Placement>,
    pub unresolved: usize,
}

/// Applies the record files of the guide's own mods (`<guide>/mods/*/`, in folder-name order, one source each) into
/// `db` and returns every exterior placement group they define or patch, as `town:<group id>`, sorted by name. No
/// `mods/` folder (or none with a manifest): nothing, and `db` is untouched.
pub(crate) fn guide_towns(db: &mut RecordDb, guide: &Path) -> Result<Vec<GuideTown>, String> {
    let root = guide.join(GUIDE_MODS_DIR);
    let read = match std::fs::read_dir(&root) {
        Ok(r) => r,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(format!("{}: {e}", root.display())),
    };
    let mut dirs: Vec<PathBuf> =
        read.filter_map(Result::ok).map(|e| e.path()).filter(|p| p.join(sf_mods::MANIFEST_FILE).is_file()).collect();
    if dirs.is_empty() {
        return Ok(Vec::new());
    }
    dirs.sort();
    sf_world::register_kinds(db).map_err(|e| e.to_string())?;
    let mut sources: Vec<SourceId> = Vec::with_capacity(dirs.len());
    for dir in &dirs {
        let manifest = sf_mods::ModManifest::load_dir(dir).map_err(|e| e.to_string())?;
        let source = db.add_source(format!("guide:{}", manifest.id.as_str()), &[]);
        sources.push(source);
        for file in &manifest.content.records {
            // Record files are paths inside the mod's asset roots; a later root wins (`sf_mods::Content`).
            let path = manifest.content.assets.iter().rev().map(|a| dir.join(a).join(file)).find(|p| p.is_file());
            let path = path.ok_or_else(|| format!("{}: record file {file} is in none of its asset roots", dir.display()))?;
            let format = path.extension().and_then(|e| e.to_str()).and_then(RecordFormat::from_extension);
            let format = format.ok_or_else(|| format!("{}: not a .ron or .toml record file", path.display()))?;
            let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            let text = text.strip_prefix('\u{feff}').unwrap_or(&text);
            let outcome = apply_record_file(db, source, text, format).map_err(|e| format!("{}: {e}", path.display()))?;
            for e in &outcome.errors {
                eprintln!("warning: {}: {e}", path.display());
            }
        }
    }
    let ext = mw_world::ids::exterior();
    let ours = |id: &sf_records::RecordId| db.entry::<PlacementGroup>(id).is_some_and(|e| e.history.iter().any(|t| sources.contains(&t.source)));
    let mut towns: Vec<GuideTown> = db
        .iter::<PlacementGroup>()
        .filter(|(id, g)| g.worldspace == ext && ours(id))
        .map(|(id, g)| GuideTown {
            name: format!("town:{id}"),
            placements: g.resolve(id).collect(),
            unresolved: g.placements.iter().filter(|p| mw_world::source::resolve_base(db, p.base.id.as_str()).is_none()).count(),
        })
        .collect();
    towns.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(towns)
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

#[cfg(test)]
#[path = "world_tests.rs"]
mod tests;
