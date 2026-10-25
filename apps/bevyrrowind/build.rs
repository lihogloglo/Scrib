//! DEV CONVENIENCE ONLY (`--features dlss`): copies a locally built DLSS package into
//! `target/<profile>/dlss/`, where the `dlss_wgpu` facade looks for it at runtime.
//!
//! The game exe contains no NVIDIA code and builds without NVIDIA's SDK; DLSS comes from the
//! separately distributed package (`sf_dlss.dll` + NVIDIA's `nvngx_dlss(d).dll` + licence),
//! built by `../superfargoth/plugins/sf_dlss/build-package.sh`. This copy only saves a manual
//! step in dev; the `dlss/` folder must never go into a game release. No package: a warning,
//! and the game reports DLSS unsupported. Other builds: nothing.

use std::path::{Path, PathBuf};

/// Where `build-package.sh` assembles the package (relative to this crate).
const PACKAGE: &str = "../../../superfargoth/target/sf_dlss/package/dlss";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if std::env::var_os("CARGO_FEATURE_DLSS").is_none() {
        return;
    }
    let manifest = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR"));
    let package = manifest.join(PACKAGE);
    println!("cargo:rerun-if-changed={}", package.display());
    let Ok(entries) = std::fs::read_dir(&package) else {
        println!(
            "cargo:warning=no local DLSS package at {} (bash ../superfargoth/plugins/sf_dlss/build-package.sh); DLSS will be unavailable",
            package.display()
        );
        return;
    };
    // OUT_DIR is <target>/<profile>/build/<pkg>-<hash>/out; the exe lands in <target>/<profile>.
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("cargo sets OUT_DIR"));
    let Some(exe_dir) = out_dir.ancestors().nth(3) else { return };
    let dest = exe_dir.join("dlss");
    if let Err(e) = std::fs::create_dir_all(&dest) {
        println!("cargo:warning=could not create {}: {e}", dest.display());
        return;
    }
    for entry in entries.flatten() {
        let from = entry.path();
        if from.is_file() {
            println!("cargo:rerun-if-changed={}", from.display());
            copy_if_newer(&from, &dest.join(entry.file_name()));
        }
    }
}

fn copy_if_newer(from: &Path, to: &Path) {
    let Ok(meta) = std::fs::metadata(from) else { return };
    let same = std::fs::metadata(to).is_ok_and(|m| m.len() == meta.len() && m.modified().ok() >= meta.modified().ok());
    if !same && let Err(e) = std::fs::copy(from, to) {
        // A running exe keeps its DLLs locked; the old copy stays usable.
        println!("cargo:warning=could not copy {} to {}: {e}", from.display(), to.display());
    }
}
