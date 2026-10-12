//! Render settings shared by interiors and exteriors: ray-traced lighting
//! (`--rt`, Bevy Solari) and temporal upscaling (`--dlss`, Bevy's DLSS).
//! The framework side is `sf_sky_bevy` (`RenderPathSetting`,
//! `UpscalingSetting`); spec `superfargoth/docs/specs/sky-lighting.md`.

use bevy::prelude::*;
use sf_sky_bevy::sf_sky::{RenderPath, Upscaling};
use sf_sky_bevy::{SkyRenderOptions, UpscalingSetting};

/// Default for `--dlss` when the flag isn't given (perf suite: `--env BEVYRROWIND_DLSS=quality`).
const ENV_DLSS: &str = "BEVYRROWIND_DLSS";

/// Upscaling without `--dlss` or `BEVYRROWIND_DLSS`: DLSS `auto` (Quality
/// at our resolutions) in builds with DLSS, which falls back to native on
/// GPUs without it; off in builds without (owner decision, ARCHITECTURE.md
/// §10, 2026-10-12). `--dlss off` turns it off.
pub fn default_upscaling() -> Upscaling {
    if cfg!(feature = "dlss") { Upscaling::parse("auto").unwrap_or(Upscaling::Off) } else { Upscaling::Off }
}

/// The render options of a view.
#[derive(Clone, Copy, Debug, Default)]
pub struct RenderSettings {
    pub path: RenderPath,
    pub upscaling: Upscaling,
    /// DLSS Ray Reconstruction on the ray-traced path (`--off denoiser` turns it off).
    pub denoiser: bool,
}

impl RenderSettings {
    /// From the flags; `--dlss` falls back to `BEVYRROWIND_DLSS`, then to
    /// [`default_upscaling`].
    pub fn new(ray_traced: bool, dlss: Option<Upscaling>, denoiser: bool) -> Result<Self, String> {
        let upscaling = match dlss {
            Some(u) => u,
            None => match std::env::var(ENV_DLSS) {
                Ok(raw) if !raw.trim().is_empty() => Upscaling::parse(&raw)?,
                _ => default_upscaling(),
            },
        };
        if ray_traced && !cfg!(feature = "solari") {
            eprintln!("--rt needs a build with `--features solari`; rendering raster");
        }
        if upscaling != Upscaling::Off && !cfg!(feature = "dlss") {
            eprintln!("--dlss needs a build with `--features dlss`; upscaling off");
        }
        let path = if ray_traced { RenderPath::RayTraced } else { RenderPath::Raster };
        Ok(Self { path, upscaling, denoiser })
    }

    /// Whether the view needs the sky plugin's render-path management.
    pub fn managed(self) -> bool {
        self.path == RenderPath::RayTraced || self.upscaling != Upscaling::Off
    }

    /// For reports: `raster`, `rt`, plus the upscaling (`raster dlss=quality`).
    pub fn label(self) -> String {
        let path = match self.path {
            RenderPath::Raster => "raster",
            RenderPath::RayTraced if self.denoiser => "rt",
            RenderPath::RayTraced => "rt nodenoiser",
        };
        match self.upscaling {
            Upscaling::Off => path.to_owned(),
            u @ Upscaling::Dlss(_) => format!("{path} dlss={}", u.label()),
        }
    }

    /// Before `DefaultPlugins`: DLSS identifies the application to NVIDIA's
    /// runtime (Bevy's DLSS init plugin reads it while creating the device).
    pub fn init(app: &mut App) {
        #[cfg(feature = "dlss")]
        app.insert_resource(bevy::anti_alias::dlss::DlssProjectId(bevy::asset::uuid::Uuid::from_u128(
            0x6b65_7272_6f77_696e_642d_6276_7972_726f,
        )));
        #[cfg(not(feature = "dlss"))]
        let _ = app;
    }

    /// Before the sky plugin: its settings resources.
    pub fn insert(self, app: &mut App) {
        app.insert_resource(UpscalingSetting(self.upscaling));
    }

    /// After the sky plugin (it inits `SkyRenderOptions`): the denoiser switch.
    pub fn apply_options(self, options: &mut SkyRenderOptions) {
        options.ray_tracing_denoiser = self.denoiser;
    }
}
