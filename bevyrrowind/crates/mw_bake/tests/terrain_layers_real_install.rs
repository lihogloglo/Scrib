//! T6 measurement (textures.md §9): the real install's terrain layers through the `terrain.layers`
//! encode (BC7 albedo + height, BC5 normal, BC5 AO + roughness): layer count, how many carry a
//! normal map / ARM, the array edge the bake picks (largest source, at most `LAYER_ARRAY_SIZE`),
//! encode time at that edge and at 2048² (what HD packs give), and the arrays' memory per preset
//! cap and under the default budget. Vanilla sources only (the mod list is loaded by the game's
//! `bake`, not here). `MORROWIND_DIR` or the detected install.
//!
//! `cargo test --release -p mw_bake --test terrain_layers_real_install -- --ignored --nocapture`
#![allow(clippy::cast_precision_loss)]

use std::time::Instant;

use mw_bake::world::{self, LAYER_ARRAY_SIZE};
use mw_install::MorrowindInstall;
use rayon::prelude::*;
use sf_terrain::bake::{LayerFile, LayerImage, LayerSource, encode_layer, normal_to_array_frame};
use sf_terrain::{LayerArrays, LayerCap};
use sf_vfs::{Vfs, VfsPath};

#[test]
#[ignore = "measurement: needs a Morrowind install; encodes every terrain layer at 2048²"]
fn terrain_layers_over_the_real_install() {
    let explicit = std::env::var_os("MORROWIND_DIR").map(std::path::PathBuf::from);
    let (install, _) = MorrowindInstall::detect(explicit.as_deref()).expect("Morrowind install");
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).expect("vfs").vfs;
    let content = mw_records::load(&vfs, &install.game_files()).expect("records");
    let layers = world::MwLayers::new(&content.db, &vfs);
    let (normals, arms) = layers.companion_counts();
    let decode = |path: &Option<String>, file: LayerFile| -> Option<LayerImage> {
        let path = path.as_ref()?;
        let bytes = vfs.read(&VfsPath::new(path).ok()?).ok()?;
        let mut img = layers.decode(path, &bytes, file).ok()?;
        if file == LayerFile::Normal {
            normal_to_array_frame(&mut img.rgba);
        }
        Some(img)
    };
    let t = Instant::now();
    let maps: Vec<[Option<LayerImage>; 3]> = layers
        .layers()
        .iter()
        .filter_map(|id| layers.layer(id))
        .map(|l| {
            [
                decode(&Some(l.albedo_height.clone()), LayerFile::AlbedoHeight),
                decode(&l.normal, LayerFile::Normal),
                decode(&l.arm, LayerFile::Arm),
            ]
        })
        .collect();
    let decoded = t.elapsed().as_secs_f64();
    let n = maps.len();
    let largest = maps.iter().filter_map(|m| m[0].as_ref()).map(|i| i.width.max(i.height)).max().unwrap_or(1);
    let picked = largest.next_power_of_two().clamp(LayerCap::MIN_EDGE, LAYER_ARRAY_SIZE);
    println!("terrain layers: {n}, {normals} with a normal map, {arms} with ARM; largest albedo {largest}², arrays {picked}²");
    println!("  decode: {decoded:.2} s");
    for edge in [picked, 2048] {
        let t = Instant::now();
        let bytes: usize = maps
            .par_iter()
            .map(|[a, nm, r]| encode_layer(a.as_ref(), nm.as_ref(), r.as_ref(), edge).iter().map(Vec::len).sum::<usize>())
            .sum();
        println!(
            "  encode at {edge}²: {:.2} s on {} threads, {:.1} MiB",
            t.elapsed().as_secs_f64(),
            rayon::current_num_threads(),
            bytes as f64 / 1_048_576.0
        );
    }
    let levels = LAYER_ARRAY_SIZE.trailing_zeros() + 1;
    let budget = Some(1usize << 30);
    for (preset, cap) in [("Low", Some(512)), ("Medium", Some(1024)), ("High", Some(2048)), ("Ultra", None)] {
        let with_budget = LayerCap { max_edge: cap, budget: cap.and(budget) };
        let skip = with_budget.skip(LAYER_ARRAY_SIZE, levels, n);
        let edge = LAYER_ARRAY_SIZE >> skip;
        let mib = 3.0 * n as f64 * LayerArrays::slice_len(edge, levels - skip) as f64 / 1_048_576.0;
        println!("  {preset}: {edge}², {mib:.0} MiB (from {LAYER_ARRAY_SIZE}² arrays)");
    }
}
