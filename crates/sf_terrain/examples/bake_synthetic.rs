//! Cold-bakes the synthetic world through the terrain steps and prints
//! per-step timings (the bake budget check without a renderer).
//!
//! `cargo run -p sf_terrain --features bake --release --example bake_synthetic -- [size_km]`

mod synthetic;

fn main() {
    let size_km = std::env::args().nth(1).map_or(16.0, |s| s.parse().expect("size in km"));
    let (baked, _) = synthetic::bake(size_km, true);
    println!("{} tiles baked", baked.tiles().len());
}
