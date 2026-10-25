//! Checks the baked exterior terrain against Morrowind's own surface (the
//! `LAND` heights on their alternating triangulation, `HeightSampler`):
//! under `GROUNDED` placements (where `constrain` must keep the source
//! surface) and everywhere else (the clamped Catmull-Rom upsample).
//!
//! `cargo run --release -p mw_bake --example ground_check [-- <Morrowind folder>]`
//! (after `bevyrrowind bake`).
#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]

use std::sync::Arc;

use mw_install::MorrowindInstall;
use mw_world::convert::UNIT_METERS_F64;
use mw_world::terrain::HeightSampler;
use mw_world::{MwWorldSource, SourceOptions, ids};
use sf_bake::Store;
use sf_model::steps::ModelInfoStep;
use sf_terrain::bake::BakedTerrain;
use sf_terrain::{ResidentTile, TerrainHeights};
use sf_vfs::Vfs;
use sf_world::PlacementFlags;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let explicit = std::env::args().nth(1).map(std::path::PathBuf::from);
    let (install, _) = MorrowindInstall::detect(explicit.as_deref())?;
    let vfs = mw_vfs::mount_install(&install, Vfs::builder())?.vfs;
    let content = mw_records::load(&vfs, &install.game_files())?;
    let db = &content.db;
    let store = Store::open(mw_bake::default_cache_root().ok_or("no cache root")?)?;
    let manifest = store.load_manifest(&mw_bake::profile_name(&install))?;

    // Model bounds from the bake, as the bake's grounding used them.
    let bounds: std::collections::HashMap<String, ([f32; 3], [f32; 3])> = manifest
        .products
        .iter()
        .filter(|(k, _)| k.step == ModelInfoStep::ID)
        .filter_map(|(k, d)| {
            let info = ModelInfoStep::decode(&store.get(d).ok()?).ok()?;
            (!info.bounds.is_empty()).then(|| (k.subject.clone(), (info.bounds.min, info.bounds.max)))
        })
        .collect();
    let lookup = |m: &str| bounds.get(m).copied();
    let source = MwWorldSource::build(db, &SourceOptions { model_bounds: Some(&lookup), ..SourceOptions::default() });

    let ext = ids::exterior();
    let baked = BakedTerrain::new(store.clone(), &manifest, ext.clone());
    let mut heights = TerrainHeights::default();
    let mut tiles = 0;
    for c in baked.tiles().into_iter().filter(|c| c.level == 0) {
        let product = baked.tile(c).ok_or("tile listed but missing")??;
        heights.insert(Arc::new(ResidentTile { tile: product.height_tile(), bounds: product.bounds }));
        tiles += 1;
    }
    println!("{tiles} level-0 tiles resident");

    let mut sampler = HeightSampler::new(db);
    // (grounded, other): differences baked - source in meters.
    let mut diffs: [Vec<f64>; 2] = [Vec::new(), Vec::new()];
    let mut missing = 0;
    for p in source.all_placements(&ext) {
        let pos = p.transform.pos;
        let w = pos.to_f64();
        let Some(src) = sampler.height((w.x / UNIT_METERS_F64) as f32, (-w.z / UNIT_METERS_F64) as f32) else {
            continue;
        };
        let Some(baked) = heights.height(pos) else {
            missing += 1;
            continue;
        };
        let d = f64::from(baked) - f64::from(src) * UNIT_METERS_F64;
        diffs[usize::from(!p.flags.contains(PlacementFlags::GROUNDED))].push(d);
    }
    println!("placements on LAND without a resident baked height: {missing}");
    for (name, mut d) in [("grounded", std::mem::take(&mut diffs[0])), ("not grounded", std::mem::take(&mut diffs[1]))] {
        d.sort_by(|a, b| a.abs().total_cmp(&b.abs()));
        let n = d.len().max(1);
        let pct = |q: f64| d.get(((n - 1) as f64 * q) as usize).map_or(0.0, |v| v.abs());
        let over = |t: f64| d.iter().filter(|v| v.abs() > t).count();
        let mean = d.iter().sum::<f64>() / n as f64;
        println!(
            "{name:>13}: {} placements; |baked − source| median {:.4} m, p99 {:.4} m, max {:.4} m; mean {mean:+.4} m; > 1 cm: {}, > 10 cm: {}",
            d.len(),
            pct(0.5),
            pct(0.99),
            pct(1.0),
            over(0.01),
            over(0.10)
        );
    }
    Ok(())
}
