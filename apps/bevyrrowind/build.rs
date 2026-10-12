//! DLSS builds (`--features dlss`): copies NVIDIA's runtime DLLs next to the
//! executable, where NVIDIA's loader looks for them (the DLSS SDK may not be
//! redistributed, so they come from the local SDK in `../sdk`, see
//! `.cargo/config.toml` and `superfargoth/docs/specs/sky-lighting.md`).
//! `cargo run` would also find them through `DLSS_SDK`; the copy makes the
//! exe start on its own (perf suite, shortcuts). Other builds: nothing.

use std::path::{Path, PathBuf};

const DLLS: [&str; 2] = [
    "nvngx_dlss.dll",  // Super Resolution
    "nvngx_dlssd.dll", // Ray Reconstruction
];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DLSS_SDK");
    if std::env::var_os("CARGO_FEATURE_DLSS").is_none() {
        return;
    }
    let Some(sdk) = std::env::var_os("DLSS_SDK").map(PathBuf::from) else {
        println!("cargo:warning=DLSS_SDK is not set; copy nvngx_dlss(d).dll next to the executable yourself");
        return;
    };
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if target_os != "windows" {
        // Linux loads libnvidia-ngx-dlss*.so the same way; not set up here.
        return;
    }
    let source = sdk.join("lib").join(format!("Windows_{target_arch}")).join("rel");
    // OUT_DIR is <target>/<profile>/build/<pkg>-<hash>/out; the exe lands in <target>/<profile>.
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("cargo sets OUT_DIR"));
    let Some(exe_dir) = out_dir.ancestors().nth(3) else { return };
    for dll in DLLS {
        let from = source.join(dll);
        println!("cargo:rerun-if-changed={}", from.display());
        copy_if_newer(&from, &exe_dir.join(dll));
        // Examples and tests run from deps/ and examples/.
        for sub in ["deps", "examples"] {
            if exe_dir.join(sub).is_dir() {
                copy_if_newer(&from, &exe_dir.join(sub).join(dll));
            }
        }
    }
}

fn copy_if_newer(from: &Path, to: &Path) {
    let Ok(meta) = std::fs::metadata(from) else {
        println!("cargo:warning=DLSS runtime {} missing (bash ../sdk/fetch.sh); DLSS will be unavailable", from.display());
        return;
    };
    let same = std::fs::metadata(to).is_ok_and(|m| m.len() == meta.len() && m.modified().ok() >= meta.modified().ok());
    if !same && let Err(e) = std::fs::copy(from, to) {
        // A running exe keeps its DLLs locked; the old copy stays usable.
        println!("cargo:warning=could not copy {} to {}: {e}", from.display(), to.display());
    }
}
