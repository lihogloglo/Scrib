//! World adapter report on the real install: worldspaces, door links,
//! placements, the exterior index, and terrain.
//!
//! `cargo run --release -p mw_world --example world_report [-- <Morrowind folder>]`
#![allow(clippy::cast_precision_loss, clippy::too_many_lines, clippy::many_single_char_names)]

use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

use mw_install::MorrowindInstall;
use mw_world::{MwTerrainSource, MwWorldSource, SourceOptions, WorldRecords, ids, terrain};
use rayon::prelude::*;
use sf_terrain::{LayerId, SourceMosaic, TerrainSource};
use sf_vfs::{Vfs, VfsPath};
use sf_world::{IndexParams, ModelInfo, Partition, PlacementFlags, WorldSource, build_index};

type Bounds = HashMap<String, ([f32; 3], [f32; 3], u64)>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let explicit = std::env::args().nth(1).map(std::path::PathBuf::from);
    let (install, _) = MorrowindInstall::detect(explicit.as_deref())?;
    let t = Instant::now();
    let vfs = mw_vfs::mount_install(&install, Vfs::builder())?.vfs;
    let content = mw_records::load(&vfs, &install.game_files())?;
    println!("loaded {} files in {:.2?}", content.files.len(), t.elapsed());
    for f in &content.files {
        println!("  {}", f.name);
    }
    for w in &content.warnings {
        if matches!(w, mw_records::LoadWarning::MovedReferences { .. } | mw_records::LoadWarning::BadMasterIndex { .. }) {
            println!("  warning: {w:?}");
        }
    }
    let db = &content.db;

    // Worldspaces and door links.
    let t = Instant::now();
    let records = WorldRecords::build(db);
    println!("\nworld records in {:.2?}", t.elapsed());
    println!("  worldspaces: {} (1 exterior + {} interiors)", records.worldspaces.len(), records.worldspaces.len() - 1);
    let d = &records.doors;
    println!("  door references with a destination: {}", d.doors);
    println!("  two-way links: {} ({} doors), one-way links: {}", d.paired, d.paired * 2, d.one_way);
    println!("  dangling destinations: {}", d.dangling.len());
    for (cell, dest) in d.dangling.iter().take(10) {
        println!("    {cell} -> {dest:?}");
    }
    let (ends, _) = mw_world::worldspaces::door_ends(db);
    println!("  one-way links (door cell -> destination: nearest back-pointing door, distances door↔arrival):");
    for (_, link) in records.door_links.iter().filter(|(_, l)| !l.two_way) {
        let a = ends.iter().find(|e| Some(e.id) == link.a.door).unwrap();
        let best = ends
            .iter()
            .filter(|b| b.worldspace == a.destination && b.destination == a.worldspace && b.id != a.id)
            .map(|b| (b.pose.pos.delta(a.arrival.pos).length(), a.pose.pos.delta(b.arrival.pos).length(), &b.cell))
            .min_by(|x, y| (x.0 + x.1).total_cmp(&(y.0 + y.1)));
        match best {
            Some((d1, d2, cell)) => println!("    {} -> {}: {cell} at {d1:.1} m / {d2:.1} m", a.cell, a.destination),
            None => println!("    {} -> {}: none", a.cell, a.destination),
        }
    }
    let edges = [0.5, 1.0, 2.0, 3.0, 4.0, 6.0, 8.0, 12.0, 16.0, 32.0, 64.0, f32::INFINITY];
    let mut hist = vec![0usize; edges.len()];
    for s in &d.best_scores {
        hist[edges.iter().position(|e| s <= e).unwrap()] += 1;
    }
    println!("  best back-pointing candidate, sum of both distances (doors with any candidate: {}):", d.best_scores.len());
    for (e, n) in edges.iter().zip(&hist) {
        println!("    <= {e:>5} m: {n}");
    }

    // Model bounds (cheap pass over every placed model).
    let t = Instant::now();
    let source0 = MwWorldSource::build(db, &SourceOptions::default());
    let first_build = t.elapsed();
    let mut models: Vec<String> = Vec::new();
    for ws in source0.worldspaces() {
        models.extend(source0.all_placements(ws).filter_map(|p| p.model.as_ref().map(|m| m.as_str().to_owned())));
    }
    models.sort();
    models.dedup();
    let t = Instant::now();
    let bounds: Bounds = models
        .par_iter()
        .filter_map(|m| {
            let path = VfsPath::new(m).ok()?;
            let bytes = vfs.read(&path).ok()?;
            let model = mw_nif::load(&bytes).ok()?;
            let (min, max) = mw_nif::visible_bounds(&model)?;
            Some((m.clone(), (min, max, bytes.len() as u64)))
        })
        .collect();
    println!("\nmodel bounds: {} of {} models in {:.2?}", bounds.len(), models.len(), t.elapsed());

    // Placements.
    let lookup = |m: &str| bounds.get(m).map(|(a, b, _)| (*a, *b));
    let options = SourceOptions { model_bounds: Some(&lookup), ..SourceOptions::default() };
    let t = Instant::now();
    let source = MwWorldSource::build(db, &options);
    println!("\nplacements: built in {:.2?} with bounds ({first_build:.2?} without)", t.elapsed());
    let s = source.stats();
    println!("  worldspaces {}, regions {}, placements {}", s.worldspaces, s.regions, s.placements);
    for (class, n) in &s.by_class {
        println!("    {class:<12} {n}");
    }
    println!("  grounded {} (pivot rule: {}), editor-only {}", s.grounded, source0.stats().grounded, s.editor_only);
    println!("  unknown bases {}", s.unknown_bases.len());
    let mut unknown: BTreeMap<&str, usize> = BTreeMap::new();
    for (_, b) in &s.unknown_bases {
        *unknown.entry(b.as_str()).or_default() += 1;
    }
    let mut unknown: Vec<_> = unknown.into_iter().collect();
    unknown.sort_by_key(|u| std::cmp::Reverse(u.1));
    for (b, n) in unknown.iter().take(8) {
        println!("    {b}: {n}");
    }
    println!(
        "  moved (MVRF): {} applied, {} to grids without a CELL, {} positioned outside their destination; {} in interiors",
        s.moved, s.moved_to_missing_cell, s.moved_outside_destination, s.moved_interior
    );
    println!("  exterior refs positioned outside their own cell: {}", s.outside_own_cell);

    // Precision: WorldPos vs the f64 conversion, every placement.
    let ext = ids::exterior();
    let mut worst = 0.0f64;
    let mut ids_seen = std::collections::HashSet::new();
    let mut dup = 0;
    for ws in source.worldspaces() {
        for p in source.all_placements(ws) {
            if !ids_seen.insert(p.id) {
                dup += 1;
            }
            let back = p.transform.pos.to_f64();
            let again = sf_coords::WorldPos::from_f64(back).to_f64();
            worst = worst.max((back - again).abs().max_element());
        }
    }
    println!("  duplicate ids across all worldspaces: {dup}; WorldPos round-trip worst {worst:e} m");

    // Index sanity: the exterior.
    let info = |path: &VfsPath| {
        bounds.get(path.as_str()).map(|(min, max, bytes)| ModelInfo {
            min: glam::Vec3::from_array(*min),
            max: glam::Vec3::from_array(*max),
            bytes: *bytes,
            lod_distance: 0.0,
        })
    };
    let t = Instant::now();
    let index = build_index(&ext, &IndexParams::default(), &source, &info);
    let build = t.elapsed();
    let t = Instant::now();
    let page_bytes: usize = index.pages.iter().map(|p| p.to_bytes().len()).sum();
    let dir_bytes = index.directory.to_bytes().len();
    println!("\nexterior index: built in {build:.2?}, encoded in {:.2?}", t.elapsed());
    let st = &index.stats;
    println!(
        "  placements {}, cells {} (128 m), pages {}, missing models {}, duplicate ids {}",
        st.placements,
        st.cells,
        st.pages,
        st.missing_models.len(),
        st.duplicate_ids.len()
    );
    println!(
        "  bytes: pages {page_bytes} ({:.1} per placement), directory {dir_bytes}",
        page_bytes as f64 / st.placements as f64
    );
    let mut cells: Vec<_> = index.directory.cells.iter().collect();
    cells.sort_by_key(|c| std::cmp::Reverse(c.placements));
    println!("  largest cells by placements:");
    for c in cells.iter().take(6) {
        println!(
            "    {:?}: {} placements ({} far), {:.1} MiB est., bounds {:?}..{:?}",
            c.cell.to_array(),
            c.placements,
            c.far_placements,
            c.bytes_all as f64 / 1_048_576.0,
            c.bounds.min.to_array().map(f32::round),
            c.bounds.max.to_array().map(f32::round)
        );
    }
    let mut by_size: Vec<_> = index.directory.cells.iter().collect();
    by_size.sort_by(|a, b| (b.bounds.max - b.bounds.min).max_element().total_cmp(&(a.bounds.max - a.bounds.min).max_element()));
    println!("  largest content AABBs:");
    for c in by_size.iter().take(4) {
        println!("    {:?}: extent {:?} m", c.cell.to_array(), (c.bounds.max - c.bounds.min).to_array().map(f32::round));
    }

    // Interiors: every one indexed as a single cell.
    let t = Instant::now();
    let params = IndexParams { partition: Partition::Single, ..IndexParams::default() };
    let mut interiors = Vec::new();
    for ws in source.worldspaces().filter(|w| **w != ext) {
        let index = build_index(ws, &params, &source, &info);
        let extent = index.directory.cells.first().map_or(0.0, |c| (c.bounds.max - c.bounds.min).max_element());
        interiors.push((extent, index.stats.placements, ws.clone()));
    }
    println!("\ninteriors: {} indexed in {:.2?}", interiors.len(), t.elapsed());
    interiors.sort_by(|a, b| b.0.total_cmp(&a.0));
    for (extent, n, ws) in interiors.iter().take(6) {
        println!("    {ws}: {extent:.0} m across, {n} placements");
    }

    // Terrain.
    let t = Instant::now();
    let terrain = MwTerrainSource::new(db);
    println!("\nterrain: source built in {:.2?}", t.elapsed());
    println!("  {:?}", terrain.stats());
    let t = Instant::now();
    let mut tiles = Vec::new();
    terrain.tiles(&ext, &mut |info| tiles.push(info.id));
    let read: Vec<_> = tiles.iter().filter_map(|id| Some((*id, terrain.read(&ext, *id)?))).collect();
    let read_time = t.elapsed();
    let mut layer_ids: HashMap<sf_records::RecordId, LayerId> = HashMap::new();
    let t = Instant::now();
    let mosaic = SourceMosaic::new(read, terrain::fill_height(), &mut |id| {
        let next = LayerId(u32::try_from(layer_ids.len()).unwrap());
        *layer_ids.entry(id.clone()).or_insert(next)
    });
    println!("  read {} tiles in {read_time:.2?}; mosaic in {:.2?}: {}", tiles.len(), t.elapsed(), match &mosaic {
        Ok(_) => "ok".to_owned(),
        Err(e) => format!("error {e}"),
    });
    let layers = terrain::terrain_layers(db, Some(&vfs));
    let present = layers.iter().filter(|(_, l)| vfs.contains(&l.albedo_height)).count();
    println!("  layers: {} defined, {} textures present, {} used", layers.len(), present, layer_ids.len());
    let mut surfaces: BTreeMap<&str, usize> = BTreeMap::new();
    for (_, l) in &layers {
        *surfaces.entry(l.surface.id.as_str()).or_default() += 1;
    }
    println!("  surfaces: {surfaces:?}");

    // Shared borders: Morrowind's neighbors should agree on edge heights.
    let mut edges = 0;
    let mut mismatched = 0;
    let mut worst_edge = 0.0f32;
    for info in terrain.infos() {
        let (x, y) = terrain::tile_grid(info.id);
        let (Some(a), Some(b)) = (terrain.land((x, y)), terrain.land((x + 1, y))) else { continue };
        edges += 1;
        let mut bad = false;
        for j in 0..terrain::LAND_VERTS {
            let d = (a.heights[j * 65 + 64] - b.heights[j * 65]).abs();
            worst_edge = worst_edge.max(d);
            bad |= d > 0.0;
        }
        mismatched += usize::from(bad);
    }
    println!("  east-west shared edges: {edges}, {mismatched} with differing heights (worst {worst_edge} units)");

    // Grounded flags vs the source surface, by class.
    let mut grounded: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    for p in source.all_placements(&ext) {
        let e = grounded.entry(format!("{:?}", p.class)).or_default();
        e.0 += usize::from(p.flags.contains(PlacementFlags::GROUNDED));
        e.1 += 1;
    }
    println!("  grounded by class (exterior): {grounded:?}");

    // The fingerprint pass that a bake runs per check.
    let t = Instant::now();
    let mut regions = 0;
    for ws in source.worldspaces() {
        source.regions(ws, &mut |_, _| regions += 1);
    }
    println!("\nregions() over every worldspace: {regions} regions in {:.2?}", t.elapsed());
    Ok(())
}
