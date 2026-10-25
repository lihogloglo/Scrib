//! superfargoth patch: finds and loads the `sf_dlss` plugin once per process.
//!
//! Search order: `$SF_DLSS_PLUGIN_DIR` (dev override), then `<exe dir>/dlss/`. A missing plugin
//! or an ABI version mismatch disables DLSS with one info log line; nothing panics.

use crate::abi::{self, ApiV1, AppInfo};
use std::{
    ffi::CString,
    path::{Path, PathBuf},
    sync::OnceLock,
};
use uuid::Uuid;

/// Dev override for the plugin folder.
pub const PLUGIN_DIR_ENV: &str = "SF_DLSS_PLUGIN_DIR";
/// Plugin folder name next to the executable.
pub const PLUGIN_DIR_NAME: &str = "dlss";

pub(crate) struct Plugin {
    pub api: &'static ApiV1,
    dir: CString,
    // Never unloaded: NGX keeps global state and threads inside the plugin.
    _library: libloading::Library,
}

// SAFETY: `api` points at the plugin's immutable static function table and the library is never
// unloaded; the plugin functions are thread-safe per NGX's rules for the calls we make.
unsafe impl Send for Plugin {}
unsafe impl Sync for Plugin {}

static PLUGIN: OnceLock<Option<Plugin>> = OnceLock::new();

/// The loaded plugin, or `None` if DLSS is unavailable.
pub(crate) fn get() -> Option<&'static Plugin> {
    PLUGIN.get_or_init(load).as_ref()
}

fn candidate_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Some(dir) = std::env::var_os(PLUGIN_DIR_ENV) {
        dirs.push(PathBuf::from(dir));
    }
    if let Some(exe_dir) = std::env::current_exe().ok().and_then(|exe| exe.parent().map(Path::to_path_buf)) {
        dirs.push(exe_dir.join(PLUGIN_DIR_NAME));
    }
    dirs
}

fn load() -> Option<Plugin> {
    let dirs = candidate_dirs();
    let Some(dir) = dirs.iter().find(|dir| dir.join(abi::PLUGIN_FILE).is_file()) else {
        tracing::info!(
            "DLSS plugin not found ({} in {:?}); DLSS unavailable",
            abi::PLUGIN_FILE,
            dirs
        );
        return None;
    };
    let path = dir.join(abi::PLUGIN_FILE);
    match open(&path, dir) {
        Ok(plugin) => {
            tracing::info!("DLSS plugin loaded from {}", path.display());
            Some(plugin)
        }
        Err(reason) => {
            tracing::info!("DLSS plugin {} not usable ({reason}); DLSS unavailable", path.display());
            None
        }
    }
}

fn open(path: &Path, dir: &Path) -> Result<Plugin, String> {
    // SAFETY: loading a DLL runs its initializers; the plugin is our own `sf_dlss` crate, whose
    // only initializers are Rust/CRT ones. The ABI version is checked before any other call.
    unsafe {
        let library = libloading::Library::new(path).map_err(|e| e.to_string())?;
        let version = library
            .get::<unsafe extern "C" fn() -> u32>(abi::SYMBOL_ABI_VERSION)
            .map_err(|e| e.to_string())?();
        if version != abi::ABI_VERSION {
            return Err(format!("plugin ABI v{version}, game expects v{}", abi::ABI_VERSION));
        }
        let api = library
            .get::<unsafe extern "C" fn() -> *const ApiV1>(abi::SYMBOL_API_V1)
            .map_err(|e| e.to_string())?();
        // SAFETY: the plugin returns a pointer to a `static` table (lives as long as the library,
        // which we never unload).
        let api = api.as_ref().ok_or("null API table")?;
        let dir = CString::new(dir.to_string_lossy().into_owned()).map_err(|e| e.to_string())?;
        Ok(Plugin { api, dir, _library: library })
    }
}

/// Owned strings behind an [`AppInfo`].
pub(crate) struct App {
    project_id: CString,
    engine_version: CString,
}

impl App {
    pub fn new(project_id: Uuid) -> Self {
        Self {
            project_id: CString::new(project_id.to_string()).expect("UUID has no NUL"),
            engine_version: CString::new(env!("CARGO_PKG_VERSION")).expect("version has no NUL"),
        }
    }

    /// The ABI view; borrows `self` and `plugin`.
    pub fn info(&self, plugin: &Plugin) -> AppInfo {
        AppInfo {
            project_id: self.project_id.as_ptr(),
            engine_version: self.engine_version.as_ptr(),
            plugin_dir: plugin.dir.as_ptr(),
            flags: if cfg!(feature = "debug_overlay") { abi::APP_DEBUG_OVERLAY } else { 0 },
        }
    }
}
