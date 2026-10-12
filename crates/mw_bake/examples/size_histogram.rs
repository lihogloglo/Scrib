//! Exterior placements by size-based draw distance, per quality preset
//! (industry-audit.md J1): how many placements each preset culls before the
//! ring or far-field handoff that ends them today, from the baked index.
//!
//! `cargo run --release -p mw_bake --example size_histogram [-- <Morrowind folder>]`
//! (after `bevyrrowind bake`).
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use std::collections::BTreeMap;

use mw_install::MorrowindInstall;
use mw_world::ids;
use sf_bake::Store;
use sf_model::LodScreen;
use sf_world::bake::BakedIndex;
use sf_world::{CellDirectory, IndexPage, IndexReader, PlacementClass};

/// (name, `min_pixels`, `min_draw_distance`, `bias`), as the app's presets.
const PRESETS: [(&str, f32, f32, f32); 4] =
    [("low", 4.0, 16.0, 0.75), ("medium", 3.0, 20.0, 0.875), ("high", 2.0, 24.0, 1.0), ("ultra", 1.0, 32.0, 1.0)];
/// The baked far-field handoff `d1` and band (far-field.md), before `bias`.
const D1: f32 = 400.0;
const BAND: f32 = 0.1;
/// Bucket upper bounds, meters; the last bucket is "not culled before its end".
const BUCKETS: [f32; 5] = [32.0, 64.0, 128.0, 256.0, f32::INFINITY];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let explicit = std::env::args().nth(1).map(std::path::PathBuf::from);
    let (install, _) = MorrowindInstall::detect(explicit.as_deref())?;
    let store = Store::open(mw_bake::default_cache_root().ok_or("no cache root")?)?;
    let manifest = store.load_manifest(&mw_bake::profile_name(&install))?;
    let index = BakedIndex::new(store, std::sync::Arc::new(manifest));
    let ext = ids::exterior();
    let dir = CellDirectory::from_bytes(&index.directory(&ext)?.ok_or("no exterior index: run `bevyrrowind bake`")?)?;
    // (class, far eligible, radius)
    let mut placements = Vec::new();
    for page in &dir.pages {
        let Some(bytes) = index.page(&ext, page.page)? else { continue };
        for cell in IndexPage::from_bytes(&bytes)?.cells {
            for p in cell.placements {
                if p.placement.model.is_some() && p.placement.class != PlacementClass::Marker {
                    placements.push((p.placement.class, p.placement.far_eligible(), p.radius));
                }
            }
        }
    }
    println!("{} exterior placements with a model ({} cells)", placements.len(), dir.cells.len());
    let mut by_class: BTreeMap<String, usize> = BTreeMap::new();
    for (class, _, _) in &placements {
        *by_class.entry(format!("{class:?}")).or_default() += 1;
    }
    println!("by class: {by_class:?}");
    let mut radii: Vec<f32> = placements.iter().map(|p| p.2).collect();
    radii.sort_by(f32::total_cmp);
    let q = |f: f32| radii[((radii.len() - 1) as f32 * f) as usize];
    println!(
        "radius m: p5 {:.2} p10 {:.2} p25 {:.2} p50 {:.2} p75 {:.2}; unknown (0, never size-culled) {}",
        q(0.05),
        q(0.1),
        q(0.25),
        q(0.5),
        q(0.75),
        radii.iter().filter(|&&r| r <= 0.0).count()
    );

    println!("\n| preset | s_min | floor | bias | d1 | <32 m | 32-64 | 64-128 | 128-256 | >=256, culled | not culled | culled total | eligible culled, r >= d1/4K (L1-clamp candidates) |");
    println!("|---|---|---|---|---|---|---|---|---|---|---|---|---|");
    for (name, min_pixels, floor, bias) in PRESETS {
        let screen = LodScreen { min_pixels, min_draw_distance: floor, bias, ..LodScreen::default() };
        let d1 = D1 * bias;
        // R1 unload with a far field (sf_lod_bevy::far_streaming_config).
        let r1 = d1 * (1.0 + BAND) + 208.0;
        let min_kept = d1 / 4.0 / screen.focal_px();
        let mut counts = [0usize; BUCKETS.len() + 1];
        let mut clamp = 0usize;
        for &(class, eligible, radius) in &placements {
            let end = if eligible { d1 } else { r1 };
            let exempt = matches!(class, PlacementClass::Actor | PlacementClass::Door) || radius <= 0.0;
            let d = screen.draw_distance(radius);
            if exempt || d >= end {
                counts[BUCKETS.len()] += 1;
                continue;
            }
            counts[BUCKETS.iter().position(|&b| d < b).expect("last bucket is infinite")] += 1;
            clamp += usize::from(eligible && radius >= min_kept);
        }
        let culled: usize = counts[..BUCKETS.len()].iter().sum();
        let pct = |n: usize| 100.0 * n as f32 / placements.len() as f32;
        let cells: Vec<String> = counts.iter().map(|&n| format!("{n} ({:.0} %)", pct(n))).collect();
        println!(
            "| {name} | {min_pixels} | {floor} | {bias} | {d1} | {} | {culled} ({:.0} %) | {clamp} |",
            cells.join(" | "),
            pct(culled)
        );
    }
    Ok(())
}
