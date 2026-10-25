//! The test-scene registry's shared half (`scenes.ron`, docs/visual-checks.md): one scene's
//! definition and the scene this process runs. The app's `scenes.rs` loads, lists and runs them.

use std::sync::OnceLock;

use serde::Deserialize;

/// One test scene.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SceneDef {
    /// Short kebab-case name (`scene <name>`).
    pub name: String,
    /// What `view` takes: interior name, exterior name, or grid "x,y".
    #[serde(default)]
    pub cell: Option<String>,
    /// Instead of `cell`: a synthetic test level (`levels/<name>.ron`, `level.rs`).
    #[serde(default)]
    pub level: Option<String>,
    /// `view --camera`: "ex,ey,ez,tx,ty,tz" meters ("+h": above the ground in exteriors).
    #[serde(default)]
    pub camera: Option<String>,
    /// Exteriors: time of day in hours (`view --hour`). The scene's clock starts paused.
    #[serde(default)]
    pub hour: Option<f64>,
    /// More `view` flags, e.g. `["--far-tint"]`, `["--off", "farfield"]`, `["--dlss", "quality"]`.
    #[serde(default)]
    pub flags: Vec<String>,
    /// Cargo features the exe needs (`"dlss"`, `"solari,dlss"`); informational.
    #[serde(default)]
    pub build: Option<String>,
    /// One line: what the scene isolates.
    pub description: String,
    /// What to look for (shown in the window overlay).
    #[serde(default)]
    pub look_for: String,
    /// Ids of the `docs/visual-checks.md` entries this scene serves.
    #[serde(default)]
    pub checks: Vec<String>,
}

impl SceneDef {
    /// The scene places its own camera (`camera`, or `--camera` in its flags): interactive runs
    /// keep it instead of starting in player mode (`player.rs`).
    pub fn fixed_camera(&self) -> bool {
        self.camera.is_some() || self.flags.iter().any(|f| f == "--camera")
    }
}

/// The scene this process runs, for the overlay and screenshot names.
static ACTIVE: OnceLock<SceneDef> = OnceLock::new();

/// The scene this process was started with (`scene <name>`), if any.
pub fn active() -> Option<&'static SceneDef> {
    ACTIVE.get()
}

/// Records the scene this process runs (once; later calls are ignored). `scene <name>` calls it
/// before starting the view.
pub fn set_active(scene: SceneDef) {
    let _ = ACTIVE.set(scene);
}
