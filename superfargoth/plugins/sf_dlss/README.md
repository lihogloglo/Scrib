# sf_dlss: NVIDIA DLSS plugin

The NGX (DLSS) half of superfargoth's `dlss_wgpu` facade (`../../vendor/dlss_wgpu`). The game
exe links only the facade, which contains no NVIDIA code and builds without NVIDIA's SDK; at
runtime the facade loads this DLL and calls it through the versioned C ABI in
`../../vendor/dlss_wgpu/src/abi.rs` (one file, `#[path]`-included here).

**The built `sf_dlss.dll` statically links NVIDIA's NGX SDK (`nvsdk_ngx_d.lib`).** It is never
part of a game release. It is distributed separately, as the DLSS package below, together with
NVIDIA's `nvngx_dlss.dll` / `nvngx_dlssd.dll` and NVIDIA's licence notice, under NVIDIA's DLSS
SDK terms. The source in this folder is MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`);
its NGX calls are a port of crates.io `dlss_wgpu` 6.0.0 (same licence).

## Build

Needs the NVIDIA DLSS SDK subset, Vulkan headers and libclang in `Bevyrrowind/sdk`
(`bash sdk/fetch.sh`); `.cargo/config.toml` points `DLSS_SDK`, `VULKAN_SDK`, `LIBCLANG_PATH`
there (real environment variables win). Own `[workspace]`, so the game workspaces never build it.
No wgpu or Bevy dependency: a Bevy upgrade doesn't need a plugin rebuild unless `abi.rs` changes.

```sh
bash build-package.sh            # cargo build --release + package in superfargoth/target/sf_dlss/package/dlss
```

## Runtime layout

```
<game exe folder>/
  bevyrrowind.exe
  dlss/                      <- the DLSS package
    sf_dlss.dll
    nvngx_dlss.dll           (Super Resolution)
    nvngx_dlssd.dll          (Ray Reconstruction)
    NVIDIA-DLSS-LICENSE.txt
    README.txt
```

The facade looks in `$SF_DLSS_PLUGIN_DIR` first (dev override), then `<exe dir>/dlss/`. NGX is
told to search the plugin folder, the exe folder and `$DLSS_SDK/lib/<platform>/rel` for its
runtime DLLs. Plugin missing or ABI version mismatch: one info log line, DLSS reported
unsupported, everything else unchanged.

Dev builds: `bevyrrowind/apps/bevyrrowind/build.rs` copies a locally built package into
`target/<profile>/dlss/` when one exists (dev convenience only).
