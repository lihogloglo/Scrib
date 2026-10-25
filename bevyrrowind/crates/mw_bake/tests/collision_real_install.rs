//! Collision sources of known vanilla models (`mw_bake::collision`). Skips
//! (passes) when no Morrowind install is detected. Whole-install numbers:
//! `cargo run --release -p mw_bake --example collision_report`.

use mw_bake::collision::{NoCollision, Source, collision_of};
use mw_install::MorrowindInstall;
use sf_vfs::{Vfs, VfsPath};

#[test]
fn vanilla_models_take_the_expected_collision_source() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let cases = [
        ("meshes/i/in_cave_plant10.nif", Source::Authored),
        ("meshes/x/ex_t_menhir_l_01.nif", Source::RenderLod),
        ("meshes/l/light_com_candle_13.nif", Source::RestPose),
        ("meshes/f/flora_kelp_02.nif", Source::None(NoCollision::Marker)),
        ("meshes/editormarker.nif", Source::None(NoCollision::EditorMarker)),
        ("meshes/r/slaughterfish.nif", Source::None(NoCollision::Actor)),
    ];
    for (path, expected) in cases {
        let bytes = vfs.read(&VfsPath::new(path).unwrap()).unwrap_or_else(|e| panic!("{path}: {e}"));
        let (source, mesh) = collision_of(&mw_nif::load(&bytes).unwrap(), 1e-3);
        assert_eq!(source, expected, "{path}");
        match source {
            Source::RenderLod => assert!(mesh.is_none(), "{path}"),
            Source::None(_) => assert!(mesh.is_some_and(|m| m.is_empty()), "{path}"),
            Source::Authored | Source::RestPose => assert!(mesh.is_some_and(|m| !m.is_empty()), "{path}"),
        }
    }
}
