//! Repository, profile and pipeline tests (mods.md "Verification", A1).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use semver::Version;
use sf_records::{RecordDb, RecordId};
use sf_vfs::{MemSource, VfsPath, VfsSource};

use super::*;

const API: Version = Version::new(0, 1, 0);

fn id(s: &str) -> ModId {
    ModId::new(s).unwrap()
}

/// Writes `<dir>/<folder>/mod.toml` with `manifest` and the given files.
fn write_mod(dir: &Path, folder: &str, manifest: &str, files: &[(&str, &str)]) -> PathBuf {
    let root = dir.join(folder);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join(MANIFEST_FILE), manifest).unwrap();
    for (path, text) in files {
        let path = root.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    root
}

fn basic(mod_id: &str, extra: &str) -> String {
    format!("id = \"{mod_id}\"\nname = \"{mod_id}\"\nversion = \"1.0.0\"\ndescription = \"test\"\n{extra}")
}

fn kinds_for(report: &ModReport, mod_id: &str) -> Vec<IssueKind> {
    let mod_id = id(mod_id);
    report.issues_for(Some(&mod_id)).map(|i| i.kind.clone()).collect()
}

/// Record type of the toy format.
#[derive(Debug, PartialEq)]
struct Num(i64);

/// A toy content adapter: synthesizes mods with in-memory layers and applies
/// `.num` record files (`define <id> <n>`, `add <id> <n>`, `warn`, anything else
/// is an error at that line).
#[derive(Default)]
struct ToyAdapter {
    mods: Vec<ModManifest>,
    layers: HashMap<ModId, Arc<dyn VfsSource>>,
}

impl ToyAdapter {
    fn with_mod(mut self, manifest: ModManifest, layer: Option<MemSource>) -> Self {
        if let Some(layer) = layer {
            self.layers.insert(manifest.id.clone(), Arc::new(layer));
        }
        self.mods.push(manifest);
        self
    }
}

impl ContentAdapter for ToyAdapter {
    fn name(&self) -> &'static str {
        "toy"
    }

    fn discover(&self, _issues: &mut Vec<ModIssue>) -> Vec<ModManifest> {
        self.mods.clone()
    }

    fn mount(&self, manifest: &ModManifest) -> Result<Vec<Arc<dyn VfsSource>>, String> {
        if manifest.id.as_str() == "broken-archive" {
            return Err("archive is corrupt".into());
        }
        Ok(self.layers.get(&manifest.id).cloned().into_iter().collect())
    }

    fn record_extensions(&self) -> &[&str] {
        &["num"]
    }

    fn register_kinds(&self, db: &mut RecordDb) {
        db.register::<Num>("num").unwrap();
    }

    fn apply_records(&self, cx: &mut RecordCx<'_>, file: &VfsPath) -> Result<(), RecordFileError> {
        let text = String::from_utf8(cx.files.read(file).map_err(|e| RecordFileError::new(e.to_string()))?)
            .map_err(|e| RecordFileError::new(e.to_string()))?;
        for (n, line) in text.lines().enumerate() {
            let line_no = u32::try_from(n + 1).unwrap();
            let words: Vec<&str> = line.split_whitespace().collect();
            match words.as_slice() {
                ["define", rid, v] => cx.db.define(cx.source, RecordId::new(rid), Num(v.parse().unwrap())).unwrap(),
                ["add", rid, v] => {
                    let v: i64 = v.parse().unwrap();
                    cx.db
                        .patch::<Num>(cx.source, &RecordId::new(rid), |n| n.0 += v)
                        .map_err(|e| RecordFileError::new(e.to_string()).at(line_no, 1))?;
                }
                ["warn"] => cx.warn(file, "toy warning"),
                _ => return Err(RecordFileError::new(format!("bad line {line:?}")).at(line_no, 1)),
            }
        }
        Ok(())
    }
}

fn load(repo: &ModRepository, enabled: &[&str], adapters: &[&dyn ContentAdapter]) -> LoadedProfile {
    let profile = Profile::new("test", enabled.iter().map(|s| id(s)));
    load_profile(repo, &profile, adapters, &API)
}

fn order(loaded: &LoadedProfile) -> Vec<&str> {
    loaded.mods.iter().map(|m| m.id().as_str()).collect()
}

#[test]
fn manifest_parses_new_sections() {
    let m = ModManifest::from_toml(
        r#"
        id = "lanterns"
        name = "Lanterns"
        version = "1.2.0"
        api = "^0.1"
        [dependencies]
        base = "^1"
        [optional]
        hud = "*"
        [conflicts]
        old_lanterns = "<2.0"
        [content]
        assets = ["assets"]
        records = ["records/items.ron"]
        scripts = ["scripts/lanterns/main.luau"]
        [ui]
        styles = ["ui/lanterns/style.css"]
        "#,
    )
    .unwrap();
    assert!(m.optional[&id("hud")].matches(&Version::new(3, 0, 0)));
    assert!(m.conflicts[&id("old_lanterns")].matches(&Version::new(1, 9, 0)));
    assert_eq!(m.ui.styles, ["ui/lanterns/style.css"]);
    assert!(ModManifest::from_toml("id = \"a\"\nname = \"A\"\nversion = \"1.0.0\"\n[ui]\nstyle = []").is_err());
}

#[test]
fn scan_reports_manifest_and_file_issues() {
    let tmp = tempfile::tempdir().unwrap();
    let mods = tmp.path().join("mods");
    write_mod(&mods, "good", &basic("good", "[content]\nassets = [\"a\"]"), &[("a/textures/x.dds", "x")]);
    std::fs::create_dir_all(mods.join("loose-folder")).unwrap();
    write_mod(&mods, "broken", "id = \"broken\"\nname = 3", &[]);
    write_mod(&mods, "bad-id", "id = \"Bad Id\"\nname = \"B\"\nversion = \"1.0.0\"", &[]);
    write_mod(&mods, "zz-dup", &basic("good", ""), &[]);
    write_mod(&mods, "nodesc", "id = \"nodesc\"\nname = \"N\"\nversion = \"1.0.0\"", &[]);
    write_mod(
        &mods,
        "files",
        &basic(
            "files",
            "[content]\nassets = [\"a\", \"missing\", \"empty\"]\nrecords = [\"records/x.num\", \"../up.num\"]\n\
             scripts = [\"scripts/files/main.luau\", \"scripts/other/main.luau\"]\n[ui]\nstyles = [\"ui/files/s.css\"]",
        ),
        &[("a/readme.txt", "x")],
    );
    std::fs::create_dir_all(mods.join("files/empty")).unwrap();

    let repo = ModRepository::scan(&[mods.clone(), tmp.path().join("no-such-dir")], &[]);
    let folder_level: Vec<&IssueKind> = repo.issues.iter().filter(|i| i.mod_id.is_none()).map(|i| &i.kind).collect();
    assert_eq!(folder_level.len(), 3, "{folder_level:?}"); // no manifest, two parse errors
    assert!(folder_level.iter().any(|k| matches!(k, IssueKind::NoManifest { .. })));
    assert_eq!(folder_level.iter().filter(|k| matches!(k, IssueKind::ManifestParse { .. })).count(), 2);

    let report = ModReport { issues: repo.issues.clone(), mods: Vec::new() };
    assert!(matches!(&kinds_for(&report, "good")[..], [IssueKind::DuplicateId { .. }]));
    assert_eq!(kinds_for(&report, "nodesc"), [IssueKind::MissingDescription]);
    let files = kinds_for(&report, "files");
    let has = |k: &IssueKind| files.contains(k);
    assert!(has(&IssueKind::MissingAssetRoot { root: "missing".into() }));
    assert!(has(&IssueKind::EmptyAssetRoot { root: "empty".into() }));
    assert!(has(&IssueKind::MissingRecordFile { path: "records/x.num".into() }));
    assert!(files.iter().any(|k| matches!(k, IssueKind::InvalidPath { .. })));
    assert!(has(&IssueKind::MissingEntryPoint { path: "scripts/files/main.luau".into() }));
    assert!(has(&IssueKind::EntryPointOutsideNamespace { path: "scripts/other/main.luau".into() }));
    assert!(has(&IssueKind::MissingStyle { path: "ui/files/s.css".into() }));
    assert_eq!(files.len(), 7, "{files:?}");

    assert!(repo.is_valid(&id("good")) && repo.is_valid(&id("nodesc")));
    assert!(!repo.is_valid(&id("files")));
    assert_eq!(repo.manifests().len(), 3);
    // The first `good` (folder order) is kept.
    assert!(repo.get(&id("good")).unwrap().root.as_ref().unwrap().ends_with("good"));

    let loaded = load(&repo, &["files", "good"], &[]);
    assert_eq!(order(&loaded), ["good"]);
    assert!(kinds_for(&loaded.report, "files").contains(&IssueKind::InvalidMod));
}

#[test]
fn profile_round_trips_through_its_file() {
    let tmp = tempfile::tempdir().unwrap();
    let mut profile = Profile::new("default", [id("base"), id("lanterns")]);
    let mut table = toml::Table::new();
    table.insert("radius".into(), toml::Value::Float(6.5));
    table.insert("enabled".into(), toml::Value::Boolean(true));
    profile.settings.insert(id("lanterns"), table);
    profile.limits.memory_mib = Some(128);
    let path = profile.save(tmp.path()).unwrap();
    assert_eq!(path, Profile::path_in(tmp.path(), "default"));

    let back = Profile::load(&path).unwrap();
    assert_eq!(back, profile);
    assert_eq!(Profile::list(tmp.path()), ["default"]);

    let empty = Profile::from_toml("e", "").unwrap();
    assert!(empty.enabled.is_empty() && empty.limits.is_unset());
    assert!(!empty.to_toml().unwrap().contains("limits"));
    assert!(Profile::from_toml("e", "enabld = []").is_err());
}

fn in_memory(manifests: Vec<ModManifest>) -> ModRepository {
    let mut repo = ModRepository::new();
    for m in manifests {
        repo.add(m);
    }
    repo
}

fn synth(mod_id: &str, version: (u64, u64, u64)) -> ModManifest {
    ModManifest::synthesized(id(mod_id), mod_id, Version::new(version.0, version.1, version.2))
}

#[test]
fn optional_dependencies_order_when_enabled_and_inform_when_not() {
    let mut patch = synth("patch", (1, 0, 0));
    patch.optional.insert(id("hud"), "^2".parse().unwrap());
    let repo = in_memory(vec![patch, synth("hud", (2, 1, 0)), synth("other", (1, 0, 0))]);

    let loaded = load(&repo, &["patch", "other", "hud"], &[]);
    assert_eq!(order(&loaded), ["other", "hud", "patch"]);
    assert!(kinds_for(&loaded.report, "patch").is_empty());

    let loaded = load(&repo, &["patch", "other"], &[]);
    assert_eq!(order(&loaded), ["patch", "other"]);
    let info = &loaded.report.issues[0];
    assert_eq!(info.kind, IssueKind::OptionalMissing { dependency: id("hud") });
    assert_eq!(info.severity(), Severity::Info);
}

#[test]
fn optional_version_mismatch_conflicts_and_api_refuse_the_mod() {
    let mut picky = synth("picky", (1, 0, 0));
    picky.optional.insert(id("hud"), "^3".parse().unwrap());
    let mut fussy = synth("fussy", (1, 0, 0));
    fussy.conflicts.insert(id("hud"), "<3".parse().unwrap());
    let mut tolerant = synth("tolerant", (1, 0, 0));
    tolerant.conflicts.insert(id("hud"), ">=3".parse().unwrap());
    let mut future = synth("future", (1, 0, 0));
    future.api = "^0.2".parse().unwrap();
    let repo = in_memory(vec![picky, fussy, tolerant, future, synth("hud", (2, 0, 0))]);

    let loaded = load(&repo, &["hud", "picky", "fussy", "tolerant", "future"], &[]);
    assert_eq!(order(&loaded), ["hud", "tolerant"]);
    assert!(matches!(&kinds_for(&loaded.report, "picky")[..], [IssueKind::OptionalVersionMismatch { .. }]));
    assert!(matches!(&kinds_for(&loaded.report, "fussy")[..], [IssueKind::ConflictPresent { .. }]));
    assert_eq!(
        kinds_for(&loaded.report, "future"),
        [IssueKind::ApiIncompatible { required: "^0.2".parse().unwrap(), provided: API }]
    );
    assert!(loaded.report.issues.iter().all(|i| i.severity() == Severity::Error && i.kind.area() == Area::Order));
}

#[test]
fn broken_mods_drop_out_with_their_dependents() {
    let mut needs_ghost = synth("needs-ghost", (1, 0, 0));
    needs_ghost.dependencies.insert(id("ghost"), "*".parse().unwrap());
    let mut needs_that = synth("needs-that", (1, 0, 0));
    needs_that.dependencies.insert(id("needs-ghost"), "*".parse().unwrap());
    let mut old = synth("wants-new-base", (1, 0, 0));
    old.dependencies.insert(id("base"), "^2".parse().unwrap());
    let mut a = synth("a", (1, 0, 0));
    a.load_after.push(id("b"));
    let mut b = synth("b", (1, 0, 0));
    b.load_after.push(id("a"));
    let mut after_a = synth("after-a", (1, 0, 0));
    after_a.load_after.push(id("a"));
    let repo = in_memory(vec![needs_ghost, needs_that, old, synth("base", (1, 0, 0)), a, b, after_a]);

    let mut profile = Profile::new(
        "p",
        ["needs-that", "needs-ghost", "base", "wants-new-base", "a", "b", "after-a", "base", "unknown"].map(id),
    );
    profile.settings.insert(id("not-enabled"), toml::Table::new());
    let loaded = load_profile(&repo, &profile, &[], &API);
    assert_eq!(order(&loaded), ["base", "after-a"]);

    let r = &loaded.report;
    assert_eq!(
        kinds_for(r, "needs-ghost"),
        [IssueKind::MissingDependency { dependency: id("ghost"), not_loaded: false }]
    );
    assert_eq!(
        kinds_for(r, "needs-that"),
        [IssueKind::MissingDependency { dependency: id("needs-ghost"), not_loaded: true }]
    );
    assert!(matches!(&kinds_for(r, "wants-new-base")[..], [IssueKind::VersionMismatch { .. }]));
    assert_eq!(kinds_for(r, "a"), [IssueKind::Cycle { members: vec![id("a"), id("b")] }]);
    assert_eq!(kinds_for(r, "base"), [IssueKind::DuplicateInProfile]);
    assert_eq!(kinds_for(r, "unknown"), [IssueKind::UnknownMod]);
    assert_eq!(kinds_for(r, "not-enabled"), [IssueKind::SettingsForDisabledMod]);
    assert!(r.has_errors());
    assert_eq!(r.count(Severity::Warning), 2);
}

#[test]
fn strict_resolution_reports_new_errors_and_cycle_members_only() {
    let mut a = synth("a", (1, 0, 0));
    a.load_after.push(id("b"));
    let mut b = synth("b", (1, 0, 0));
    b.load_after.push(id("a"));
    let mut down = synth("down", (1, 0, 0));
    down.load_after.push(id("a"));
    let mut c = synth("c", (1, 0, 0));
    c.conflicts.insert(id("down"), "*".parse().unwrap());
    let installed = [a, b, down, c];
    let ids = |s: &[&str]| s.iter().map(|x| id(x)).collect::<Vec<_>>();
    assert_eq!(
        resolve_load_order(&installed, &ids(&["down", "a", "b"])).unwrap_err(),
        LoadOrderError::Cycle(ids(&["a", "b"]))
    );
    assert!(matches!(resolve_load_order(&installed, &ids(&["c", "down"])), Err(LoadOrderError::Conflict { .. })));
}

#[test]
fn pipeline_mounts_in_load_order_and_reports_overrides() {
    let tmp = tempfile::tempdir().unwrap();
    let mods = tmp.path().join("mods");
    write_mod(
        &mods,
        "textures",
        &basic("textures", "[content]\nassets = [\"data\"]"),
        &[("data/textures/rock.dds", "mod rock"), ("data/textures/new.dds", "new")],
    );
    write_mod(
        &mods,
        "hd",
        &basic("hd", "load_after = [\"textures\"]\n[content]\nassets = [\"a\", \"b\"]"),
        &[("a/textures/rock.dds", "hd a"), ("b/textures/rock.dds", "hd b"), ("b/textures/new.dds", "hd new")],
    );
    let base = ModManifest::synthesized(id("base"), "Base", Version::new(1, 0, 0));
    let archive = MemSource::new("base.archive").with("textures/rock.dds", "base rock").with("meshes/x.nif", "x");
    let toy = ToyAdapter::default().with_mod(base, Some(archive));
    let adapters: [&dyn ContentAdapter; 1] = [&toy];
    let repo = ModRepository::scan(std::slice::from_ref(&mods), &adapters);

    let loaded = load_profile(&repo, &Profile::new("p", ["hd", "base", "textures"].map(id)), &adapters, &API);
    assert_eq!(order(&loaded), ["base", "textures", "hd"]);
    let rock = VfsPath::new("Textures\\Rock.dds").unwrap();
    assert_eq!(loaded.vfs.read(&rock).unwrap(), b"hd b");
    let winner = loaded.owner(loaded.vfs.resolve(&rock).unwrap()).unwrap();
    assert_eq!(winner.id().as_str(), "hd");
    // A mod reads its own copy, not the winner.
    let textures = loaded.get(&id("textures")).unwrap();
    assert_eq!(textures.files(&loaded.vfs).read(&rock).unwrap(), b"mod rock");

    let summary = |m: &str| loaded.report.mods.iter().find(|s| s.id.as_str() == m).unwrap();
    let hd = summary("hd");
    assert_eq!((hd.files, hd.winning, hd.unique), (2, 2, 0));
    assert_eq!(hd.layers, ["hd:a", "hd:b"]);
    assert_eq!(hd.overrides, [(id("base"), 1), (id("textures"), 2)].into_iter().collect());
    let base = summary("base");
    assert_eq!((base.files, base.winning, base.unique), (2, 1, 1));
    assert_eq!(base.overridden_by, [(id("hd"), 1)].into_iter().collect());
    assert_eq!(base.origin, "adapter:toy");
    assert_eq!(summary("textures").overridden_by, [(id("hd"), 2)].into_iter().collect());

    let text = loaded.render_report(None, 5);
    assert!(text.contains("overrides 2 files of textures"), "{text}");
    assert!(loaded.report.render(Some(&id("base"))).contains("1 files overridden by hd"));
    assert!(!loaded.report.has_errors(), "{text}");
}

#[test]
fn records_apply_per_mod_with_dependencies_and_report_errors() {
    let tmp = tempfile::tempdir().unwrap();
    let mods = tmp.path().join("mods");
    write_mod(
        &mods,
        "patch",
        &basic(
            "patch",
            "[dependencies]\nbase = \"*\"\n[content]\nassets = [\"data\"]\n\
             records = [\"records/patch.num\", \"records/bad.num\", \"records/later.ron\"]",
        ),
        &[
            ("data/records/patch.num", "add gold 5\nwarn\ndefine silver 2"),
            ("data/records/bad.num", "add gold 1\nadd nothing 1"),
            ("data/records/later.ron", "[]"),
        ],
    );
    let mut base = ModManifest::synthesized(id("base"), "Base", Version::new(1, 0, 0));
    base.content.records = vec!["base.num".into(), "gone.num".into()];
    // The base's record file lives in the shared data folder (another mod's layer).
    let mut data = ModManifest::synthesized(id("data"), "Data", Version::new(1, 0, 0));
    data.load_before.push(id("base"));
    let shared = MemSource::new("data").with("base.num", "define gold 10");
    let mut orphan = ModManifest::synthesized(id("orphan"), "Orphan", Version::new(1, 0, 0));
    orphan.adapter = Some("elsewhere".into());
    let toy = ToyAdapter::default()
        .with_mod(base, None)
        .with_mod(data, Some(shared))
        .with_mod(synth("broken-archive", (1, 0, 0)), None);
    let adapters: [&dyn ContentAdapter; 1] = [&toy];
    let mut repo = ModRepository::scan(std::slice::from_ref(&mods), &adapters);
    repo.add(orphan);

    let loaded =
        load_profile(&repo, &Profile::new("p", ["base", "patch", "data", "broken-archive", "orphan"].map(id)), &adapters, &API);
    assert_eq!(order(&loaded), ["data", "base", "patch", "broken-archive", "orphan"]);
    // Base defines 10; patch adds 5 + 1 before its failing line.
    assert_eq!(loaded.records.get::<Num>(&RecordId::new("gold")), Some(&Num(16)));
    assert_eq!(loaded.records.get::<Num>(&RecordId::new("silver")), Some(&Num(2)));
    let patch = loaded.get(&id("patch")).unwrap();
    let base = loaded.get(&id("base")).unwrap();
    assert_eq!(loaded.records.source(patch.source).name, "patch");
    assert_eq!(loaded.records.source(patch.source).depends_on, [base.source]);
    // A patch from a dependent source is an intended override, not a conflict.
    assert_eq!(loaded.records.conflict_report().total_conflicts(), 0);

    let r = &loaded.report;
    assert_eq!(kinds_for(r, "base"), [IssueKind::MissingRecordFile { path: "gone.num".into() }]);
    let patch_issues = kinds_for(r, "patch");
    assert_eq!(patch_issues.len(), 3, "{patch_issues:?}");
    assert_eq!(patch_issues[0], IssueKind::RecordWarning { path: "records/patch.num".into(), message: "toy warning".into() });
    assert!(matches!(
        &patch_issues[1],
        IssueKind::RecordFile { path, line: Some(2), column: Some(1), .. } if path == "records/bad.num"
    ));
    assert_eq!(patch_issues[2], IssueKind::UnknownRecordFormat { path: "records/later.ron".into() });
    assert!(matches!(&kinds_for(r, "broken-archive")[..], [IssueKind::Adapter { .. }]));
    assert_eq!(kinds_for(r, "orphan"), [IssueKind::UnknownAdapter { adapter: "elsewhere".into() }]);
    assert!(r.render(None).contains("record file records/bad.num:2:1:"));
}

#[test]
fn script_plans_and_stylesheets_follow_load_order() {
    let tmp = tempfile::tempdir().unwrap();
    let mods = tmp.path().join("mods");
    write_mod(
        &mods,
        "lib",
        &basic("lib", "[content]\nassets = [\".\"]\nscripts = [\"scripts/lib/main.luau\"]"),
        &[("scripts/lib/main.luau", "return 1")],
    );
    write_mod(
        &mods,
        "notes",
        &basic(
            "notes",
            "api = \"^0.1\"\n[dependencies]\nlib = \"*\"\n[optional]\nhud = \"*\"\nmissing = \"*\"\n\
             [content]\nassets = [\"data\"]\nscripts = [\"scripts/notes/main.luau\", \"scripts/notes/extra.luau\"]\n\
             [ui]\nstyles = [\"ui/notes/style.css\"]",
        ),
        &[("data/scripts/notes/main.luau", ""), ("data/scripts/notes/extra.luau", ""), ("data/ui/notes/style.css", "")],
    );
    write_mod(&mods, "hud", &basic("hud", "[ui]\nstyles = []"), &[]);
    let repo = ModRepository::scan(std::slice::from_ref(&mods), &[]);
    let mut profile = Profile::new("p", ["notes", "hud", "lib"].map(id));
    profile.limits.call_timeout_ms = Some(20);
    let loaded = load_profile(&repo, &profile, &[], &API);
    assert_eq!(order(&loaded), ["hud", "lib", "notes"]);

    let plans = loaded.script_contexts();
    assert_eq!(plans.len(), 2);
    assert_eq!(plans[0].id, id("lib"));
    let notes = &plans[1];
    assert_eq!(notes.mods, [id("notes")]);
    assert_eq!(notes.dependencies, [id("lib"), id("hud")]);
    let entry: Vec<&str> = notes.entry_points.iter().map(VfsPath::as_str).collect();
    assert_eq!(entry, ["scripts/notes/main.luau", "scripts/notes/extra.luau"]);
    assert_eq!(notes.api, "^0.1".parse().unwrap());
    assert_eq!(notes.version, Version::new(1, 0, 0));
    assert_eq!(notes.limits.call_timeout_ms, Some(20));
    assert!(loaded.vfs.contains(&notes.entry_points[0]));

    let styles = loaded.stylesheets();
    assert_eq!(styles, [(id("notes"), VfsPath::new("ui/notes/style.css").unwrap())]);
    assert_eq!(kinds_for(&loaded.report, "notes"), [IssueKind::OptionalMissing { dependency: id("missing") }]);
}
