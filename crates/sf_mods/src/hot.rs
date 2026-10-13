//! Hot-reload routing (mods.md "Hot reload"): what a batch of winning VFS
//! changes means for a loaded profile.

use sf_vfs::{VfsChange, VfsPath};

use crate::{LoadedProfile, ModId};

/// Script file extensions (`scripts/<mod>/...`).
const SCRIPT_EXTENSIONS: [&str; 2] = ["luau", "lua"];

/// A batch of winning changes ([`sf_vfs::Vfs::rebuild`]) sorted by consumer.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HotReloadPlan {
    /// Script files under `scripts/`: the script runtime reloads every context
    /// that runs them or `require`s them as a dependency library.
    pub scripts: Vec<VfsPath>,
    /// Record files of loaded mods: the record step re-runs
    /// ([`LoadedProfile::reload_records`]).
    pub records: Vec<(ModId, VfsPath)>,
    /// Stylesheets (`ui.styles`): the UI composer re-imports them.
    pub styles: Vec<(ModId, VfsPath)>,
    /// Everything else: assets loaded by path (the asset server reloads
    /// them from its source events) and bake inputs (the host decides).
    pub assets: Vec<VfsChange>,
}

impl HotReloadPlan {
    pub fn is_empty(&self) -> bool {
        self.scripts.is_empty() && self.records.is_empty() && self.styles.is_empty() && self.assets.is_empty()
    }
}

impl LoadedProfile {
    /// Routes winning changes to their consumers.
    pub fn route(&self, changes: &[VfsChange]) -> HotReloadPlan {
        let declared = |pick: fn(&crate::ModManifest) -> &Vec<String>, path: &VfsPath| {
            self.mods.iter().find_map(|m| {
                pick(&m.manifest).iter().any(|p| VfsPath::new(p).is_ok_and(|p| p == *path)).then(|| m.id().clone())
            })
        };
        let mut plan = HotReloadPlan::default();
        for change in changes {
            let path = &change.path;
            if let Some(owner) = declared(|m| &m.content.records, path) {
                plan.records.push((owner, path.clone()));
            } else if let Some(owner) = declared(|m| &m.ui.styles, path) {
                plan.styles.push((owner, path.clone()));
            } else if path.as_str().starts_with("scripts/")
                && path.extension().is_some_and(|e| SCRIPT_EXTENSIONS.contains(&e))
            {
                plan.scripts.push(path.clone());
            } else {
                plan.assets.push(change.clone());
            }
        }
        plan
    }
}
