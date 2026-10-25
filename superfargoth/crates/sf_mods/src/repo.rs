//! `ModRepository`: every installed mod, validated (mods.md pipeline step 1, "Discover").

use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use sf_vfs::{DirSource, VfsPath, VfsSource};

use crate::{ContentAdapter, IssueKind, MANIFEST_FILE, ManifestError, ModId, ModIssue, ModManifest};

/// Installed mods: folder mods from the mods directories plus the adapters'
/// synthesized mods. Asset roots are indexed once here and reused by every
/// profile load.
#[derive(Default)]
pub struct ModRepository {
    manifests: Vec<ModManifest>,
    /// Pre-scanned asset roots per manifest (same index), lowest first.
    roots: Vec<Vec<Arc<dyn VfsSource>>>,
    by_id: HashMap<ModId, usize>,
    /// Mods with errors; enabling one reports it and skips it.
    invalid: BTreeSet<ModId>,
    /// Discovery and validation issues.
    pub issues: Vec<ModIssue>,
    /// Time spent scanning (step 1).
    pub scan_time: Duration,
}

impl ModRepository {
    pub fn new() -> Self {
        Self::default()
    }

    /// Discovers every mod: first the adapters' synthesized mods (in adapter
    /// order), then each folder with a `mod.toml` under `mod_dirs` (in the
    /// given order, folders sorted by name). A missing mods directory is empty.
    pub fn scan(mod_dirs: &[PathBuf], adapters: &[&dyn ContentAdapter]) -> Self {
        let start = Instant::now();
        let mut repo = Self::new();
        for adapter in adapters {
            let mut issues = Vec::new();
            let found = adapter.discover(&mut issues);
            repo.issues.append(&mut issues);
            for mut manifest in found {
                manifest.adapter = Some(adapter.name().to_owned());
                repo.add(manifest);
            }
        }
        for dir in mod_dirs {
            let Ok(entries) = std::fs::read_dir(dir) else { continue };
            let mut folders: Vec<PathBuf> =
                entries.filter_map(Result::ok).map(|e| e.path()).filter(|p| p.is_dir()).collect();
            folders.sort();
            for folder in folders {
                repo.add_folder(&folder);
            }
        }
        repo.scan_time = start.elapsed();
        repo
    }

    /// Loads and adds the mod in `folder`.
    pub fn add_folder(&mut self, folder: &Path) {
        if !folder.join(MANIFEST_FILE).is_file() {
            self.issues.push(ModIssue::new(None, IssueKind::NoManifest { path: folder.to_path_buf() }));
            return;
        }
        match ModManifest::load_dir(folder) {
            Ok(manifest) => self.add(manifest),
            Err(ManifestError::Parse { path, source }) => {
                let message = source.to_string().trim_end().to_owned();
                self.issues.push(ModIssue::new(None, IssueKind::ManifestParse { path, message }));
            }
            Err(e) => {
                let kind = IssueKind::ManifestParse { path: folder.join(MANIFEST_FILE), message: e.to_string() };
                self.issues.push(ModIssue::new(None, kind));
            }
        }
    }

    /// Adds a manifest: rejects duplicate ids, indexes its asset roots (when
    /// it has a `root`) and checks its declared files.
    pub fn add(&mut self, manifest: ModManifest) {
        let id = manifest.id.clone();
        if let Some(&first) = self.by_id.get(&id) {
            let kind = IssueKind::DuplicateId { first: origin(&self.manifests[first]), ignored: origin(&manifest) };
            self.issues.push(ModIssue::new(Some(&id), kind));
            return;
        }
        let errors_before = self.error_count(&id);
        let roots = self.index_roots(&manifest);
        self.validate(&manifest, &roots);
        if self.error_count(&id) > errors_before {
            self.invalid.insert(id.clone());
        }
        self.by_id.insert(id, self.manifests.len());
        self.manifests.push(manifest);
        self.roots.push(roots);
    }

    fn error_count(&self, id: &ModId) -> usize {
        self.issues.iter().filter(|i| i.mod_id.as_ref() == Some(id) && i.severity() == crate::Severity::Error).count()
    }

    fn index_roots(&mut self, manifest: &ModManifest) -> Vec<Arc<dyn VfsSource>> {
        let Some(root) = &manifest.root else { return Vec::new() };
        let mut sources: Vec<Arc<dyn VfsSource>> = Vec::new();
        for asset_root in &manifest.content.assets {
            let dir = root.join(asset_root);
            let issue = |kind| ModIssue::new(Some(&manifest.id), kind);
            if !dir.is_dir() {
                self.issues.push(issue(IssueKind::MissingAssetRoot { root: asset_root.clone() }));
                continue;
            }
            let label = format!("{}:{}", manifest.id, asset_root.display());
            match DirSource::scan(label, dir) {
                Ok(source) => {
                    let mut files = 0usize;
                    source.visit_files(&mut |_| files += 1);
                    if files == 0 {
                        self.issues.push(issue(IssueKind::EmptyAssetRoot { root: asset_root.clone() }));
                    }
                    sources.push(Arc::new(source));
                }
                Err(e) => {
                    let kind = IssueKind::UnreadableAssetRoot { root: asset_root.clone(), message: e.to_string() };
                    self.issues.push(issue(kind));
                }
            }
        }
        sources
    }

    /// Manifest-level checks. File presence is checked here for folder mods;
    /// a synthesized mod's files are checked after its adapter mounts it.
    fn validate(&mut self, manifest: &ModManifest, roots: &[Arc<dyn VfsSource>]) {
        let folder_mod = is_folder_mod(manifest);
        let mut present: HashSet<VfsPath> = HashSet::new();
        for source in roots {
            source.visit_files(&mut |path| {
                present.insert(path);
            });
        }
        let mut issues = Vec::new();
        if folder_mod && manifest.description.trim().is_empty() {
            issues.push(IssueKind::MissingDescription);
        }
        for (path, kind) in declared_files(manifest) {
            match VfsPath::new(path) {
                Err(e) => issues.push(IssueKind::InvalidPath { path: path.clone(), message: e.to_string() }),
                Ok(vpath) => {
                    if kind == FileKind::Script && !in_script_namespace(&manifest.id, &vpath) {
                        issues.push(IssueKind::EntryPointOutsideNamespace { path: path.clone() });
                    } else if folder_mod && !present.contains(&vpath) {
                        issues.push(kind.missing(path));
                    }
                }
            }
        }
        self.issues.extend(issues.into_iter().map(|kind| ModIssue::new(Some(&manifest.id), kind)));
    }

    /// Every installed manifest (valid or not), in discovery order.
    pub fn manifests(&self) -> &[ModManifest] {
        &self.manifests
    }

    pub fn get(&self, id: &ModId) -> Option<&ModManifest> {
        self.by_id.get(id).map(|&i| &self.manifests[i])
    }

    /// Whether the mod has errors that keep it from loading.
    pub fn is_valid(&self, id: &ModId) -> bool {
        self.by_id.contains_key(id) && !self.invalid.contains(id)
    }

    /// The pre-scanned asset-root layers of an installed mod.
    pub(crate) fn roots(&self, id: &ModId) -> &[Arc<dyn VfsSource>] {
        self.by_id.get(id).map_or(&[], |&i| self.roots[i].as_slice())
    }
}

/// Where a mod came from, for reports.
pub(crate) fn origin(manifest: &ModManifest) -> String {
    match (&manifest.adapter, &manifest.root) {
        (Some(adapter), _) => format!("adapter:{adapter}"),
        (None, Some(root)) => root.display().to_string(),
        (None, None) => "in memory".to_owned(),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum FileKind {
    Record,
    Script,
    Style,
}

impl FileKind {
    pub(crate) fn missing(self, path: &str) -> IssueKind {
        let path = path.to_owned();
        match self {
            Self::Record => IssueKind::MissingRecordFile { path },
            Self::Script => IssueKind::MissingEntryPoint { path },
            Self::Style => IssueKind::MissingStyle { path },
        }
    }
}

/// Every file path a manifest declares, with its kind.
pub(crate) fn declared_files(manifest: &ModManifest) -> impl Iterator<Item = (&String, FileKind)> {
    let records = manifest.content.records.iter().map(|p| (p, FileKind::Record));
    let scripts = manifest.content.scripts.iter().map(|p| (p, FileKind::Script));
    let styles = manifest.ui.styles.iter().map(|p| (p, FileKind::Style));
    records.chain(scripts).chain(styles)
}

/// `scripts/<mod id>/...` (`sf_script`'s per-mod namespace; ids are lowercase like VFS paths).
fn in_script_namespace(id: &ModId, path: &VfsPath) -> bool {
    path.as_str().strip_prefix("scripts/").and_then(|rest| rest.strip_prefix(id.as_str())).is_some_and(|rest| {
        rest.starts_with('/')
    })
}

/// A mod read from a folder (as opposed to one an adapter or a test built):
/// its declared files are checked at scan time, against its asset roots.
pub(crate) fn is_folder_mod(manifest: &ModManifest) -> bool {
    manifest.adapter.is_none() && manifest.root.is_some()
}
