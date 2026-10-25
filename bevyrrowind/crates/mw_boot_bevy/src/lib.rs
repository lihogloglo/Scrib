//! The game's boot layer (`bevyrrowind/docs/specs/app-split.md` slice 2): the mod profile every
//! command loads through ([`mods`]), the `bake` command ([`bake`]), dev hot reload
//! ([`hot_reload`]), the Morrowind UI theme ([`ui_theme`]), and the app's [`mount`] / [`load`]
//! entry points. The `bevyrrowind mods ...` CLI stays in the binary.

pub mod bake;
pub mod hot_reload;
pub mod mods;
pub mod ui_theme;

use mw_install::MorrowindInstall;
use mw_records::LoadedContent;
use sf_vfs::Vfs;

/// The app's error type: any error, boxed.
pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

/// The selected mod profile's VFS ([`mods`]): builtin graphs and runtime,
/// then archives, loose `Data Files`, and enabled mods, in load order.
pub fn mount(install: &MorrowindInstall, verbose: bool) -> Result<Vfs> {
    mods::mount(install, verbose)
}

/// The selected mod profile's VFS and records ([`mods`]).
pub fn load(install: &MorrowindInstall, verbose: bool) -> Result<(Vfs, LoadedContent)> {
    let (vfs, content, _) = mods::load_content(install, verbose)?;
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for warning in &content.warnings {
        let name = format!("{warning:?}");
        let variant = name.split([' ', '{', '(']).next().unwrap_or_default().to_owned();
        *counts.entry(variant).or_default() += 1;
    }
    for (variant, count) in counts {
        eprintln!("warning: {count} x {variant}");
    }
    Ok((vfs, content))
}
