//! Lists the animation groups, sections and events of a Morrowind `.kf` /
//! `.nif` file. Stand-in for `bevyrrowind anim <kf>` until the app gets the
//! subcommand (a thin wrapper over [`mw_anim::report`]).
//!
//! `cargo run -p mw_anim --example anim_report -- meshes/xbase_anim.kf [--events] [--group <name>]`
//!
//! The path is read from disk if it exists there, else looked up in the
//! detected Morrowind install's VFS.

use mw_anim::{Animation, ReportOptions, report};
use mw_install::MorrowindInstall;
use sf_vfs::{Vfs, VfsPath};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    // The path: the first argument that is neither a flag nor the value of `--group`.
    let path = args.iter().enumerate().find(|(i, a)| !a.starts_with("--") && (*i == 0 || args[i - 1] != "--group")).map(|(_, a)| a);
    let Some(path) = path else {
        eprintln!("usage: anim_report <path.kf|path.nif> [--events] [--group <name>]");
        std::process::exit(2);
    };
    let options = ReportOptions {
        events: args.iter().any(|a| a == "--events"),
        group: args.iter().position(|a| a == "--group").and_then(|i| args.get(i + 1)).cloned(),
    };
    let animation = Animation::from_file(path, &read(path)).unwrap_or_else(|e| fail(path, e));
    print!("{}", report(path, &animation, &options));
}

fn fail(path: &str, error: impl std::fmt::Display) -> ! {
    eprintln!("{path}: {error}");
    std::process::exit(1);
}

fn read(path: &str) -> Vec<u8> {
    if let Ok(bytes) = std::fs::read(path) {
        return bytes;
    }
    let Ok((install, _)) = MorrowindInstall::detect(None) else { fail(path, "not a file, and no Morrowind install detected") };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap_or_else(|e| fail(path, e)).vfs;
    let vfs_path = VfsPath::new(path).unwrap_or_else(|e| fail(path, e));
    vfs.read(&vfs_path).unwrap_or_else(|e| fail(path, e))
}
