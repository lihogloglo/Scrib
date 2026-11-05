//! The de-lit texture layer at start (`superfargoth/docs/specs/bake.md` "De-lighting";
//! `mw_bake::delight`): mounted above every mod unless `--off delight`, serving only entries
//! whose source is still the winning file the bake saw.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use mw_bake::VfsInputs;
use mw_bake::delight::{DelightLayer, load_index};
use mw_install::MorrowindInstall;
use sf_vfs::Vfs;

use crate::bake::cache_root;

/// `vfs` with the de-lit layer on top when `enabled` and baked; prints one `delight:` line.
pub fn mount(install: &MorrowindInstall, vfs: Vfs, cache: Option<PathBuf>, enabled: bool) -> Vfs {
    if !enabled {
        println!("delight: off (original textures)");
        return vfs;
    }
    let started = Instant::now();
    let Some((store, root)) = cache_root(cache).ok().and_then(|root| Some((sf_bake::Store::open(&root).ok()?, root))) else {
        println!("delight: no bake cache (original textures)");
        return vfs;
    };
    let index = store.load_manifest(&mw_bake::profile_name(install)).ok().and_then(|m| load_index(&store, &m));
    let Some(index) = index else {
        println!("delight: not baked for this profile (run `bevyrrowind bake`; original textures)");
        return vfs;
    };
    let vfs = Arc::new(vfs);
    let inputs = VfsInputs::new(vfs.clone(), install, root.join("filehash.memo"));
    let (layer, stats) = DelightLayer::build(store, &index, &vfs, &inputs);
    let mounted = vfs.with_top_layer(Arc::new(layer));
    println!(
        "delight: layer mounted, {} textures ({} stale: source changed since the bake), {:.0?}",
        stats.served,
        stats.stale,
        started.elapsed()
    );
    mounted
}
