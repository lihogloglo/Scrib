//! Lists how flora/tree NIF materials draw (LM-1 diagnosis): the authored
//! alpha mode, the cutout class, and the base texture's alpha histogram
//! (holes / solid / soft, as `textures::alpha_is_cutout` reads it).
//!
//! `cargo run --release -p mw_bake --example foliage_report [-- <filter> [<Morrowind folder>]]`
//! (filter: substring of the mesh path, default `flora_`).
#![allow(clippy::cast_precision_loss)]

use mw_install::MorrowindInstall;
use sf_vfs::{Vfs, VfsPath};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filter = std::env::args().nth(1).unwrap_or_else(|| "flora_".into()).to_ascii_lowercase();
    let explicit = std::env::args().nth(2).map(std::path::PathBuf::from);
    let (install, _) = MorrowindInstall::detect(explicit.as_deref())?;
    let vfs = mw_vfs::mount_install(&install, Vfs::builder())?.vfs;
    let is_nif = |p: &VfsPath| std::path::Path::new(p.as_str()).extension().is_some_and(|e| e.eq_ignore_ascii_case("nif"));
    let mut meshes: Vec<VfsPath> = vfs.paths().filter(|p| is_nif(p) && p.as_str().contains(&filter)).cloned().collect();
    meshes.sort_by(|a, b| a.as_str().cmp(b.as_str()));
    for mesh in meshes {
        let Ok(model) = vfs.read(&mesh).map_err(|e| e.to_string()).and_then(|b| mw_nif::load(&b).map_err(|e| e.to_string()))
        else {
            continue;
        };
        for m in model.materials.iter().filter(|m| m.alpha_mode != mw_nif::AlphaMode::Opaque) {
            let tex = m.base_texture.as_deref().unwrap_or("-");
            let stats = VfsPath::new(tex)
                .ok()
                .and_then(|p| vfs.resolve_preferring(&p, &["dds", "tga"]))
                .and_then(|(r, _)| mw_bake::textures::decode_rgba(r.as_str(), &vfs.read(&r).ok()?).ok())
                .map_or_else(
                    || "undecodable".to_owned(),
                    |img| {
                        let (mut holes, mut solid, mut soft) = (0usize, 0usize, 0usize);
                        for a in img.rgba.as_chunks::<4>().0.iter().map(|p| p[3]) {
                            match a {
                                0..=25 => holes += 1,
                                230..=255 => solid += 1,
                                _ => soft += 1,
                            }
                        }
                        let n = (holes + solid + soft) as f32 / 100.0;
                        format!(
                            "holes {:.0}% solid {:.0}% soft {:.0}% cutout={}",
                            holes as f32 / n,
                            solid as f32 / n,
                            soft as f32 / n,
                            mw_bake::textures::alpha_is_cutout(&img)
                        )
                    },
                );
            println!("{}\t{:?}\t{:?}\talpha {:.2}\t{tex}\t{stats}", mesh.as_str(), m.alpha_mode, m.cutout, m.alpha);
        }
    }
    Ok(())
}
