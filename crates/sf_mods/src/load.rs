//! `load_profile`: pipeline steps 2-4 (resolve, mount, records) with a report
//! (mods.md "Pipeline (startup)").

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::ops::Range;
use std::sync::Arc;
use std::time::{Duration, Instant};

use semver::{Version, VersionReq};
use sf_records::{RecordDb, SourceId};
use sf_vfs::{FileStat, SourceIndex, Vfs, VfsBuilder, VfsPath, VfsSource};

use crate::repo::{declared_files, is_folder_mod, origin};
use crate::{
    ContentAdapter, IssueKind, LoadOrderError, ModFiles, ModId, ModIssue, ModManifest, ModReport, ModRepository,
    ModSummary, Profile, RecordCx, ScriptLimits, resolve_load_order,
};

/// One mod as loaded.
pub struct LoadedMod {
    pub manifest: ModManifest,
    /// Its VFS layers (source indices), lowest first; empty when it has no files.
    pub layers: Range<u16>,
    /// Its record source.
    pub source: SourceId,
}

impl LoadedMod {
    pub fn id(&self) -> &ModId {
        &self.manifest.id
    }

    /// The mod's own view of `vfs`.
    pub fn files<'a>(&'a self, vfs: &'a Vfs) -> ModFiles<'a> {
        ModFiles { vfs, layers: &self.layers, shared: self.manifest.adapter.is_some() }
    }
}

/// Wall time per pipeline step.
#[derive(Clone, Copy, Debug, Default)]
pub struct LoadTimings {
    pub resolve: Duration,
    pub mount: Duration,
    pub records: Duration,
}

/// A profile after steps 2-4: load order, VFS, records, and the report.
pub struct LoadedProfile {
    pub profile: Profile,
    /// Loaded mods in load order.
    pub mods: Vec<LoadedMod>,
    pub vfs: Arc<Vfs>,
    pub records: RecordDb,
    pub report: ModReport,
    pub timings: LoadTimings,
}

/// What a script context needs from the pipeline; maps one to one onto
/// `sf_script::ContextConfig` (which this crate doesn't depend on).
#[derive(Clone, Debug, PartialEq)]
pub struct ScriptPlan {
    /// Context id (the mod id); also the namespace of its events.
    pub id: ModId,
    /// Mods whose `scripts/<mod>/` the context runs.
    pub mods: Vec<ModId>,
    /// Loaded dependencies (required, and optional ones that are enabled):
    /// their library files are `require`-able as `@<dep>/...`.
    pub dependencies: Vec<ModId>,
    /// Entry points (VFS paths), run in order at start.
    pub entry_points: Vec<VfsPath>,
    pub api: VersionReq,
    pub version: Version,
    /// The profile's limits (unset values: runtime defaults).
    pub limits: ScriptLimits,
}

impl LoadedProfile {
    pub fn get(&self, id: &ModId) -> Option<&LoadedMod> {
        self.mods.iter().find(|m| m.id() == id)
    }

    /// The mod whose layer `source` is.
    pub fn owner(&self, source: SourceIndex) -> Option<&LoadedMod> {
        self.mods.iter().find(|m| m.layers.contains(&source.0))
    }

    /// Script contexts to create, in load order (one per mod with entry points).
    pub fn script_contexts(&self) -> Vec<ScriptPlan> {
        let loaded: BTreeSet<&ModId> = self.mods.iter().map(LoadedMod::id).collect();
        self.mods
            .iter()
            .filter(|m| !m.manifest.content.scripts.is_empty())
            .map(|m| {
                let manifest = &m.manifest;
                let optional = manifest.optional.keys().filter(|d| loaded.contains(d));
                ScriptPlan {
                    id: manifest.id.clone(),
                    mods: vec![manifest.id.clone()],
                    dependencies: manifest.dependencies.keys().chain(optional).cloned().collect(),
                    entry_points: manifest.content.scripts.iter().filter_map(|p| VfsPath::new(p).ok()).collect(),
                    api: manifest.api.clone(),
                    version: manifest.version.clone(),
                    limits: self.profile.limits,
                }
            })
            .collect()
    }

    /// Stylesheets in load order, for the UI composer's `@layer mods.<id>`.
    pub fn stylesheets(&self) -> Vec<(ModId, VfsPath)> {
        self.mods
            .iter()
            .flat_map(|m| m.manifest.ui.styles.iter().filter_map(|p| Some((m.id().clone(), VfsPath::new(p).ok()?))))
            .collect()
    }

    /// The mod report followed by the record conflict section.
    pub fn render_report(&self, only: Option<&ModId>, max_conflicts: usize) -> String {
        let mut out = self.report.render(only);
        out.push_str(&self.records.conflict_report().render(&self.records, max_conflicts));
        out
    }

    /// The profile fingerprint (mods.md "Bake cache interaction"): blake3 over
    /// the load-ordered mods (id, version, manifest), each synthesized mod's
    /// adapter [`stamp`](ContentAdapter::stamp), and the stat key (path, size,
    /// modification time) of every file on disk in every layer. Reads stats
    /// only, never file contents, so a warm start can compare it cheaply with
    /// the one the last bake stored: equal means nothing a bake reads changed.
    pub fn fingerprint(&self, adapters: &[&dyn ContentAdapter]) -> [u8; 32] {
        fn field(h: &mut blake3::Hasher, bytes: &[u8]) {
            h.update(&(bytes.len() as u64).to_le_bytes());
            h.update(bytes);
        }
        let mut h = blake3::Hasher::new();
        h.update(b"sf_mods.profile.v1");
        let mut files: Vec<VfsPath> = Vec::new();
        for m in &self.mods {
            let manifest = &m.manifest;
            field(&mut h, manifest.id.as_str().as_bytes());
            field(&mut h, manifest.version.to_string().as_bytes());
            field(&mut h, format!("{manifest:?}").as_bytes());
            if let Some(name) = &manifest.adapter {
                let stamp = adapters.iter().find(|a| a.name() == name).map(|a| a.stamp(manifest)).unwrap_or_default();
                field(&mut h, &stamp);
            }
            for layer in m.layers.clone() {
                let source = self.vfs.source(SourceIndex(layer));
                field(&mut h, source.label().as_bytes());
                files.clear();
                source.visit_files(&mut |p| files.push(p));
                files.sort_unstable();
                for path in &files {
                    // As indexed (directory sources), else from disk.
                    let stat = match source.stat(path) {
                        Some(stat) => stat,
                        None => match source.real_path(path) {
                            Some(real) => std::fs::metadata(real).map_or(FileStat { size: u64::MAX, mtime_ns: 0 }, |m| FileStat::of(&m)),
                            None => continue,
                        },
                    };
                    let (size, mtime) = (stat.size, stat.mtime_ns);
                    field(&mut h, path.as_str().as_bytes());
                    h.update(&size.to_le_bytes());
                    h.update(&mtime.to_le_bytes());
                }
            }
        }
        *h.finalize().as_bytes()
    }
}

/// Runs pipeline steps 2-4 for `profile`: resolves the load order (dropping
/// mods that can't load, and their dependents, with an error each), mounts the
/// VFS in load order, and applies every mod's record files.
///
/// `api` is the framework API version mods' `api` requirements are checked
/// against (`sf_script::API_VERSION`).
pub fn load_profile(
    repo: &ModRepository,
    profile: &Profile,
    adapters: &[&dyn ContentAdapter],
    api: &Version,
) -> LoadedProfile {
    let mut issues = repo.issues.clone();
    let mut timings = LoadTimings::default();

    let start = Instant::now();
    let order = resolve(repo, profile, api, &mut issues);
    timings.resolve = start.elapsed();

    let start = Instant::now();
    let (vfs, layers) = mount(repo, &order, adapters, &mut issues);
    timings.mount = start.elapsed();

    let start = Instant::now();
    let mut records = RecordDb::new();
    let mut mods = Vec::with_capacity(order.len());
    let mut sources: HashMap<ModId, SourceId> = HashMap::new();
    for adapter in adapters {
        adapter.register_kinds(&mut records);
    }
    for (manifest, layers) in order.into_iter().zip(layers) {
        let mut depends_on: Vec<SourceId> =
            manifest.dependencies.keys().chain(manifest.optional.keys()).filter_map(|d| sources.get(d).copied()).collect();
        depends_on.sort_unstable();
        let source = records.add_source(manifest.id.as_str(), &depends_on);
        sources.insert(manifest.id.clone(), source);
        let loaded = LoadedMod { manifest: manifest.clone(), layers, source };
        let files = loaded.files(&vfs);
        if !is_folder_mod(manifest) {
            check_mounted_files(manifest, files, &mut issues);
        }
        for path in &manifest.content.records {
            let Ok(file) = VfsPath::new(path) else { continue };
            if !files.contains(&file) {
                continue; // reported by validation
            }
            let ext = file.extension().unwrap_or_default();
            let Some(adapter) = adapters.iter().find(|a| a.record_extensions().contains(&ext)) else {
                let kind = IssueKind::UnknownRecordFormat { path: path.clone() };
                issues.push(ModIssue::new(Some(&manifest.id), kind));
                continue;
            };
            let mut cx = RecordCx { manifest, source, db: &mut records, files, sources: &sources, issues: &mut issues };
            if let Err(e) = adapter.apply_records(&mut cx, &file) {
                let kind =
                    IssueKind::RecordFile { path: path.clone(), line: e.line, column: e.column, message: e.message };
                issues.push(ModIssue::new(Some(&manifest.id), kind));
            }
        }
        mods.push(loaded);
    }
    for adapter in adapters {
        adapter.finish_records(&mut records);
    }
    record_issues(&records, &sources, &mut issues);
    timings.records = start.elapsed();
    shadow_issues(&vfs, &mods, adapters, &mut issues);

    let report = ModReport { issues, mods: summarize(&vfs, &mods) };
    LoadedProfile { profile: profile.clone(), mods, vfs: Arc::new(vfs), records, report, timings }
}

/// Modern files shadowed by a later mod's legacy file (the `resolve_preferring`
/// edge: a later legacy file beats an earlier modern one), on the modern file's mod.
fn shadow_issues(vfs: &Vfs, mods: &[LoadedMod], adapters: &[&dyn ContentAdapter], issues: &mut Vec<ModIssue>) {
    let alternates: Vec<(&str, &[&str])> = adapters.iter().flat_map(|a| a.modern_alternates().iter().copied()).collect();
    if alternates.is_empty() {
        return;
    }
    let owner = |s: SourceIndex| mods.iter().find(|m| m.layers.contains(&s.0));
    for path in vfs.paths() {
        let Some(ext) = path.extension() else { continue };
        for &(modern, legacy_exts) in &alternates {
            if ext != modern {
                continue;
            }
            let Some(source) = vfs.resolve(path) else { continue };
            for legacy in legacy_exts {
                let legacy_path = path.with_extension(legacy);
                let Some(legacy_source) = vfs.resolve(&legacy_path).filter(|s| s.0 > source.0) else { continue };
                let (Some(m), Some(by)) = (owner(source), owner(legacy_source)) else { continue };
                if m.id() != by.id() {
                    let kind =
                        IssueKind::ShadowedModern { path: path.to_string(), legacy: legacy_path.to_string(), by: by.id().clone() };
                    issues.push(ModIssue::new(Some(m.id()), kind));
                }
            }
        }
    }
}

/// Record conflicts and merges (the xEdit conflict view), one issue per record,
/// on the mod whose change won (none for adapter-derived sources).
fn record_issues(records: &RecordDb, sources: &HashMap<ModId, SourceId>, issues: &mut Vec<ModIssue>) {
    let owner: HashMap<SourceId, &ModId> = sources.iter().map(|(id, source)| (*source, id)).collect();
    let report = records.conflict_report();
    for kind in &report.kinds {
        let lists = [(&kind.conflicts, true), (&kind.merges, false)];
        for (list, conflict) in lists {
            for c in list {
                let touches: Vec<(String, String)> = c
                    .history
                    .iter()
                    .map(|t| (records.source(t.source).name.clone(), sf_records::describe(t)))
                    .collect();
                let (kind_name, id) = (kind.kind.clone(), c.id.to_string());
                let issue = if conflict {
                    let fields = c.fields.iter().map(ToString::to_string).collect();
                    IssueKind::RecordConflict { kind: kind_name, id, fields, touches }
                } else {
                    IssueKind::RecordMerge { kind: kind_name, id, touches }
                };
                let winner = c.history.last().and_then(|t| owner.get(&t.source).copied());
                issues.push(ModIssue::new(winner, issue));
            }
        }
    }
}

/// Step 2: the profile's mods in load order. Mods that can't load are dropped
/// with an error, and the order is re-resolved without them (their dependents
/// then fail in turn), so one broken mod never blocks the rest. Factorio and
/// MO2 refuse or disable the same cases; Factorio also disables a mod whose
/// enabled optional dependency has the wrong version.
fn resolve<'a>(repo: &'a ModRepository, profile: &Profile, api: &Version, issues: &mut Vec<ModIssue>) -> Vec<&'a ModManifest> {
    let mut issue = |id: &ModId, kind| issues.push(ModIssue::new(Some(id), kind));
    let mut enabled: Vec<ModId> = Vec::with_capacity(profile.enabled.len());
    for id in &profile.enabled {
        if enabled.contains(id) {
            issue(id, IssueKind::DuplicateInProfile);
        } else if let Some(manifest) = repo.get(id) {
            if !repo.is_valid(id) {
                issue(id, IssueKind::InvalidMod);
            } else if !manifest.api.matches(api) {
                issue(id, IssueKind::ApiIncompatible { required: manifest.api.clone(), provided: api.clone() });
            } else {
                enabled.push(id.clone());
            }
        } else {
            issue(id, IssueKind::UnknownMod);
        }
    }
    for id in profile.settings.keys() {
        if !profile.enabled.contains(id) {
            issue(id, IssueKind::SettingsForDisabledMod);
        }
    }

    let order = loop {
        let error = match resolve_load_order(repo.manifests(), &enabled) {
            Ok(order) => break order,
            Err(e) => e,
        };
        let not_loaded = |d: &ModId| profile.enabled.contains(d);
        let dropped: Vec<ModId> = match error {
            LoadOrderError::MissingDependency { dependent, dependency } => {
                let not_loaded = not_loaded(&dependency);
                issue(&dependent, IssueKind::MissingDependency { dependency, not_loaded });
                vec![dependent]
            }
            LoadOrderError::VersionMismatch { dependent, dependency, required, found } => {
                issue(&dependent, IssueKind::VersionMismatch { dependency, required, found });
                vec![dependent]
            }
            LoadOrderError::OptionalVersionMismatch { dependent, dependency, required, found } => {
                issue(&dependent, IssueKind::OptionalVersionMismatch { dependency, required, found });
                vec![dependent]
            }
            LoadOrderError::Conflict { dependent, other, declared, found } => {
                issue(&dependent, IssueKind::ConflictPresent { other, declared, found });
                vec![dependent]
            }
            LoadOrderError::Cycle(members) => {
                for id in &members {
                    issue(id, IssueKind::Cycle { members: members.clone() });
                }
                members
            }
            // Filtered above; drop defensively so the loop always ends.
            LoadOrderError::UnknownMod(id) | LoadOrderError::DuplicateInProfile(id) | LoadOrderError::DuplicateMod(id) => {
                issue(&id, IssueKind::UnknownMod);
                vec![id]
            }
        };
        enabled.retain(|id| !dropped.contains(id));
    };

    let loaded: BTreeSet<&ModId> = order.iter().map(|m| &m.id).collect();
    for manifest in &order {
        for dependency in manifest.optional.keys().filter(|d| !loaded.contains(d)) {
            issue(&manifest.id, IssueKind::OptionalMissing { dependency: dependency.clone() });
        }
    }
    order
}

/// Step 3: each mod's adapter layers, then its asset roots, in load order.
fn mount(
    repo: &ModRepository,
    order: &[&ModManifest],
    adapters: &[&dyn ContentAdapter],
    issues: &mut Vec<ModIssue>,
) -> (Vfs, Vec<Range<u16>>) {
    let mut builder = VfsBuilder::new();
    let mut count: u16 = 0;
    let mut ranges = Vec::with_capacity(order.len());
    for manifest in order {
        let start = count;
        let mut layers: Vec<Arc<dyn VfsSource>> = Vec::new();
        if let Some(name) = &manifest.adapter {
            match adapters.iter().find(|a| a.name() == name) {
                None => issues.push(ModIssue::new(Some(&manifest.id), IssueKind::UnknownAdapter { adapter: name.clone() })),
                Some(adapter) => match adapter.mount(manifest) {
                    Ok(mut sources) => layers.append(&mut sources),
                    Err(message) => {
                        let kind = IssueKind::Adapter { adapter: name.clone(), message };
                        issues.push(ModIssue::new(Some(&manifest.id), kind));
                    }
                },
            }
        }
        layers.extend(repo.roots(&manifest.id).iter().cloned());
        for layer in layers {
            builder = builder.mount_shared(layer);
            count = count.checked_add(1).expect("too many VFS layers");
        }
        ranges.push(start..count);
    }
    (builder.build(), ranges)
}

/// File checks for synthesized mods, whose layers exist only after mounting.
fn check_mounted_files(manifest: &ModManifest, files: ModFiles<'_>, issues: &mut Vec<ModIssue>) {
    for (path, kind) in declared_files(manifest) {
        // Invalid paths were reported by validation.
        if VfsPath::new(path).is_ok_and(|p| !files.contains(&p)) {
            issues.push(ModIssue::new(Some(&manifest.id), kind.missing(path)));
        }
    }
}

/// Per-mod VFS contribution: files, winners, and who overrides whom.
fn summarize(vfs: &Vfs, mods: &[LoadedMod]) -> Vec<ModSummary> {
    let mut owner: Vec<usize> = Vec::new();
    for (i, m) in mods.iter().enumerate() {
        owner.resize(usize::from(m.layers.end), i);
    }
    let mut out: Vec<ModSummary> = mods
        .iter()
        .map(|m| {
            let manifest = &m.manifest;
            ModSummary {
                id: manifest.id.clone(),
                version: manifest.version.clone(),
                origin: origin(manifest),
                layers: m.layers.clone().map(|s| vfs.source(SourceIndex(s)).label().to_owned()).collect(),
                files: 0,
                winning: 0,
                unique: 0,
                overrides: BTreeMap::new(),
                overridden_by: BTreeMap::new(),
                record_files: manifest.content.records.len(),
                script_entry_points: manifest.content.scripts.len(),
                styles: manifest.ui.styles.len(),
            }
        })
        .collect();
    let mut providers: Vec<usize> = Vec::new();
    for path in vfs.paths() {
        providers.clear();
        for source in vfs.providers(path) {
            let m = owner[usize::from(source.0)];
            if !providers.contains(&m) {
                providers.push(m);
            }
        }
        let Some(&winner) = providers.last() else { continue };
        out[winner].winning += 1;
        if providers.len() == 1 {
            out[winner].unique += 1;
        }
        for &m in &providers {
            out[m].files += 1;
            if m != winner {
                *out[winner].overrides.entry(mods[m].manifest.id.clone()).or_default() += 1;
                *out[m].overridden_by.entry(mods[winner].manifest.id.clone()).or_default() += 1;
            }
        }
    }
    out
}
