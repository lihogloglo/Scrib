//! Bindgen over NVIDIA's NGX headers and the static NGX link. Port of crates.io `dlss_wgpu`
//! 6.0.0's build.rs (MIT OR Apache-2.0). Needs `DLSS_SDK`, `VULKAN_SDK` and `LIBCLANG_PATH`
//! (`.cargo/config.toml` points them at `../sdk`, see README.md).

use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    println!("cargo:rerun-if-changed=src/wrapper.h");
    println!("cargo:rerun-if-env-changed=DLSS_SDK");
    println!("cargo:rerun-if-env-changed=VULKAN_SDK");

    let dlss_sdk = env::var("DLSS_SDK").expect("DLSS_SDK not set (see README.md)");
    let vulkan_sdk = env::var("VULKAN_SDK").expect("VULKAN_SDK not set (see README.md)");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let platform = if target_os == "windows" {
        format!("Windows_{target_arch}")
    } else {
        format!("Linux_{target_arch}")
    };
    println!("cargo:rustc-env=DLSS_SDK_PLATFORM={platform}");

    let mut lib_dir = format!("{dlss_sdk}/lib/{platform}");
    if target_os == "windows" && target_arch == "x86_64" {
        lib_dir.push_str("/x64");
    }
    assert!(
        Path::new(&lib_dir).is_dir(),
        "DLSS SDK at {dlss_sdk} has no libraries for {platform}"
    );
    println!("cargo:rustc-link-search=native={lib_dir}");
    if target_os == "windows" {
        let crt_static = env::var("CARGO_CFG_TARGET_FEATURE").is_ok_and(|f| f.split(',').any(|f| f == "crt-static"));
        let lib = if crt_static { "nvsdk_ngx_s" } else { "nvsdk_ngx_d" };
        println!("cargo:rustc-link-lib=static={lib}");
        // NGX's own system imports (registry, window lookup). Inside a game exe winit et al.
        // pull these in; a standalone DLL must name them.
        println!("cargo:rustc-link-lib=dylib=advapi32");
        println!("cargo:rustc-link-lib=dylib=user32");
    } else {
        println!("cargo:rustc-link-lib=static=nvsdk_ngx");
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=dylib=dl");
    }

    let vulkan_sdk_include = if target_os == "windows" { "Include" } else { "include" };
    bindgen::Builder::default()
        .header(format!("{}/src/wrapper.h", env!("CARGO_MANIFEST_DIR")))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .wrap_static_fns(true)
        .wrap_static_fns_path(out_dir.join("wrap_static_fns"))
        .clang_arg(format!("-I{dlss_sdk}/include"))
        .clang_arg(format!("-I{vulkan_sdk}/{vulkan_sdk_include}"))
        .allowlist_item(".*NGX.*")
        .blocklist_item("Vk.*")
        .blocklist_item("PFN_vk.*")
        .blocklist_item(".*Cuda.*")
        .blocklist_item(".*CUDA.*")
        .generate()
        .expect("bindgen over the NGX headers")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("write bindings.rs");

    // Static inline NGX helpers (NGX_DLSS_GET_OPTIMAL_SETTINGS, ...) need a C shim.
    cc::Build::new()
        .file(out_dir.join("wrap_static_fns.c"))
        .includes([
            format!("{dlss_sdk}/include"),
            format!("{vulkan_sdk}/{vulkan_sdk_include}"),
        ])
        .compile("wrap_static_fns");
}
