//! The baked texture layer at start (`superfargoth/docs/specs/bake.md` "De-lighting",
//! `superfargoth/docs/specs/textures.md` §5 "Serving"; `mw_bake::cook::CookedTextureLayer`): mounted
//! above every mod, serving the de-lit textures (unless `--off delight`) and the cooked ones, only
//! entries whose source is still the winning file the bake saw.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use mw_bake::VfsInputs;
use mw_bake::cook::CookedTextureLayer;
use mw_install::MorrowindInstall;
use sf_vfs::Vfs;

use crate::bake::cache_root;

/// Which indices the baked texture layer serves.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layers {
    /// De-lit albedos (`texture.delight.index`).
    pub delight: bool,
    /// Cooked textures (`texture.cook.index`). Scaffold until `Part::TexCook` (`--off texcook`) is wired
    /// in the game: [`mount`] always cooks.
    pub cook: bool,
}

/// `vfs` with the baked texture layer on top, de-lit textures when `delight`; prints one
/// `delight:` and one `texture.cook:` line.
pub fn mount(install: &MorrowindInstall, vfs: Vfs, cache: Option<PathBuf>, delight: bool) -> Vfs {
    mount_layers(install, vfs, cache, Layers { delight, cook: true })
}

/// [`mount`] with each index switched on its own.
pub fn mount_layers(install: &MorrowindInstall, vfs: Vfs, cache: Option<PathBuf>, layers: Layers) -> Vfs {
    if !layers.delight {
        println!("delight: off (original textures)");
    }
    if !layers.cook {
        println!("texture.cook: off (textures as shipped)");
    }
    if !layers.delight && !layers.cook {
        return vfs;
    }
    let started = Instant::now();
    let Some((store, root)) = cache_root(cache).ok().and_then(|root| Some((sf_bake::Store::open(&root).ok()?, root))) else {
        println!("textures: no bake cache (original textures)");
        return vfs;
    };
    let manifest = store.load_manifest(&mw_bake::profile_name(install)).ok();
    let delit = manifest.as_ref().filter(|_| layers.delight).and_then(|m| mw_bake::delight::load_index(&store, m));
    let cooked = manifest.as_ref().filter(|_| layers.cook).and_then(|m| mw_bake::cook::load_index(&store, m));
    if layers.delight && delit.is_none() {
        println!("delight: not baked for this profile (run `bevyrrowind bake`; original textures)");
    }
    if layers.cook && cooked.is_none() {
        println!("texture.cook: not baked for this profile (run `bevyrrowind bake`; textures as shipped)");
    }
    if delit.is_none() && cooked.is_none() {
        return vfs;
    }
    let vfs = Arc::new(vfs);
    let inputs = VfsInputs::new(vfs.clone(), install, root.join("filehash.memo"));
    let (layer, stats) = CookedTextureLayer::build(store, delit.as_ref(), cooked.as_ref(), &vfs, &inputs);
    let mounted = vfs.with_top_layer(Arc::new(layer));
    if delit.is_some() {
        println!("delight: layer mounted, {} textures", stats.delit);
    }
    if cooked.is_some() {
        println!("texture.cook: layer mounted, {} cooked, {} in place (as shipped)", stats.cooked, stats.in_place);
    }
    println!("textures: {} stale (source changed since the bake), {:.0?}", stats.stale, started.elapsed());
    mounted
}
