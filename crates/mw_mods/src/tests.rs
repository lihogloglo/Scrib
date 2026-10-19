use std::fs;

use sf_mods::{IssueKind, Severity};
use sf_records::RecordId;
use sf_vfs::MemSource;
use tes3::esp::{Header, Light, LightData, Static, TES3Object};

use super::*;

fn p(s: &str) -> VfsPath {
    VfsPath::new(s).unwrap()
}

fn plugin(file_type: FileType, masters: &[&str], records: Vec<TES3Object>) -> Vec<u8> {
    let header = Header { file_type, masters: masters.iter().map(|m| ((*m).to_owned(), 0)).collect(), ..Header::default() };
    Plugin { objects: std::iter::once(header.into()).chain(records).collect() }.save_bytes().unwrap()
}

fn write_bsa(path: &Path, files: &[(&str, &str)]) {
    let mut builder = tes3::bsa::Builder::new();
    for (name, data) in files {
        builder.insert(name.as_bytes().to_vec(), data.as_bytes()).unwrap();
    }
    builder.save_path(path).unwrap();
}

fn write_mod(user: &UserDirs, id: &str, files: &[(&str, &str)]) {
    let root = user.mods().join(id);
    for (path, text) in files {
        let path = root.join("assets").join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }
    let manifest = format!("id = \"{id}\"\nname = \"{id}\"\nversion = \"1.0.0\"\ndescription = \"test\"\n[content]\nassets = [\"assets\"]\n");
    fs::write(root.join("mod.toml"), manifest).unwrap();
}

/// Morrowind.esm (+ .bsa), Tribunal.esm (+ .bsa, master Morrowind), an
/// unclaimed Extra.bsa, a loose texture, and an ini listing them.
fn install(dir: &Path) -> MorrowindInstall {
    let data = dir.join("Data Files");
    fs::create_dir_all(data.join("Textures")).unwrap();
    let lantern = Light { id: "lantern_01".into(), data: LightData { radius: 100, ..LightData::default() }, ..Light::default() };
    let rock = Static { id: "rock".into(), mesh: "r.nif".into(), ..Static::default() };
    fs::write(data.join("Morrowind.esm"), plugin(FileType::Esm, &[], vec![lantern.into(), rock.into()])).unwrap();
    let rock2 = Static { id: "rock".into(), mesh: "r2.nif".into(), ..Static::default() };
    fs::write(data.join("Tribunal.esm"), plugin(FileType::Esm, &["Morrowind.esm"], vec![rock2.into()])).unwrap();
    write_bsa(&data.join("Morrowind.bsa"), &[("textures\\a.dds", "base"), ("textures\\b.dds", "base")]);
    write_bsa(&data.join("Tribunal.bsa"), &[("textures\\b.dds", "tribunal")]);
    write_bsa(&data.join("Extra.bsa"), &[("textures\\e.dds", "extra")]);
    fs::write(data.join("Textures").join("loose.dds"), "loose").unwrap();
    let ini = "[Game Files]\nGameFile0=Tribunal.esm\nGameFile1=Morrowind.esm\n[Archives]\nArchive 0=Tribunal.bsa\nArchive 1=Extra.bsa\n";
    fs::write(dir.join("Morrowind.ini"), ini).unwrap();
    MorrowindInstall::open(dir).unwrap()
}

fn builtin() -> Vec<Arc<dyn VfsSource>> {
    vec![Arc::new(MemSource::new("builtin").with("textures/a.dds", "builtin").with(RUNTIME_PATH, "return {}"))]
}

fn api() -> Version {
    Version::new(0, 1, 0)
}

fn ids(loaded: &LoadedProfile) -> Vec<&str> {
    loaded.mods.iter().map(|m| m.id().as_str()).collect()
}

#[test]
fn default_profile_loads_like_the_install() {
    let dir = tempfile::tempdir().unwrap();
    let install = install(dir.path());
    let user = UserDirs::new(dir.path().join("user"));
    let mods = Mods::open(MwAdapter::new(install, builtin()), user, DEFAULT_PROFILE).unwrap();
    assert!(!mods.profile_from_file);
    let loaded = mods.load(&api());
    assert_eq!(ids(&loaded), ["mw", "morrowind.esm", "tribunal.esm", "morrowind-data", "morrowind-weather"]);
    let errors: Vec<_> = loaded.report.issues.iter().filter(|i| i.severity() == Severity::Error).collect();
    assert!(errors.is_empty(), "{errors:?}");

    // Layers: builtin < Morrowind.bsa < Tribunal.bsa < Extra.bsa < Data Files.
    let labels: Vec<String> = loaded.vfs.stats().into_iter().map(|s| s.label).collect();
    assert_eq!(labels, ["builtin", "Morrowind.bsa", "Tribunal.bsa", "Extra.bsa", "Data Files", "weather"]);
    assert_eq!(loaded.vfs.read(&p("textures/a.dds")).unwrap(), b"base");
    assert_eq!(loaded.vfs.read(&p("textures/b.dds")).unwrap(), b"tribunal");
    assert_eq!(loaded.vfs.read(&p(RUNTIME_PATH)).unwrap(), b"return {}");

    // Records: Tribunal's rock wins; content files are reported in load order.
    let rock = loaded.records.get::<Static>(&RecordId::new("rock")).unwrap();
    assert_eq!(rock.mesh, "r2.nif");
    let (files, _) = mods.adapter.take_content();
    let names: Vec<&str> = files.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(names, ["morrowind.esm", "tribunal.esm"]);
    assert_eq!(files[1].masters, ["Morrowind.esm"]);
}

#[test]
fn mods_patch_records_override_files_and_change_the_fingerprint() {
    let dir = tempfile::tempdir().unwrap();
    let install = install(dir.path());
    let user = UserDirs::new(dir.path().join("user"));
    // A record mod patches the content and the weather mod's generated records alike.
    let patch = r#"[
        patch(kind: "LIGH", id: "lantern_01", set: { "data.radius": 5 }),
        patch(kind: "weather.preset", id: "mw_weather/clear", set: { "climate_ambience": 0.5 }),
    ]"#;
    let root = user.mods().join("dim_lanterns");
    fs::create_dir_all(root.join("assets/records")).unwrap();
    fs::write(root.join("assets/records/lights.ron"), patch).unwrap();
    let manifest = "id = \"dim_lanterns\"\nname = \"Dim\"\nversion = \"1.0.0\"\ndescription = \"test\"\n[content]\nassets = [\"assets\"]\nrecords = [\"records/lights.ron\"]\n";
    fs::write(root.join("mod.toml"), manifest).unwrap();
    write_mod(&user, "textures", &[("textures/a.dds", "mod"), ("textures/m.ktx2", "modern")]);
    write_mod(&user, "legacy", &[("textures/m.dds", "legacy")]);

    let mut profile = default_profile(&install);
    for id in ["dim_lanterns", "textures", "legacy"] {
        profile.enabled.push(ModId::new(id).unwrap());
    }
    profile.save(&user.profiles()).unwrap();
    let mods = Mods::open(MwAdapter::new(install, builtin()), user.clone(), DEFAULT_PROFILE).unwrap();
    assert!(mods.profile_from_file);
    let loaded = mods.load(&api());
    assert_eq!(ids(&loaded)[5..], ["dim_lanterns", "textures", "legacy"]);

    let lantern = loaded.records.get::<Light>(&RecordId::new("lantern_01")).unwrap();
    assert_eq!(lantern.data.radius, 5);
    let report = loaded.render_report(None, 5);
    assert!(report.contains("patch data.radius"), "{report}");
    assert!(report.contains("patch climate_ambience"), "{report}");
    let errors: Vec<_> = loaded.report.issues.iter().filter(|i| i.severity() == Severity::Error).collect();
    assert!(errors.is_empty(), "{errors:?}");
    assert_eq!(loaded.vfs.read(&p("textures/a.dds")).unwrap(), b"mod");
    let shadowed = loaded.report.issues.iter().any(|i| {
        matches!(&i.kind, IssueKind::ShadowedModern { path, by, .. } if path == "textures/m.ktx2" && by.as_str() == "legacy")
            && i.mod_id.as_ref().is_some_and(|m| m.as_str() == "textures")
    });
    assert!(shadowed, "{report}");

    // Same files: same fingerprint; a changed file (size) changes it.
    let first = mods.fingerprint(&loaded);
    assert_eq!(mods.fingerprint(&mods.load(&api())), first);
    fs::write(user.mods().join("textures/assets/textures/a.dds"), "changed").unwrap();
    let rescanned = Mods::open(MwAdapter::new(mods.adapter.install().clone(), builtin()), user, DEFAULT_PROFILE).unwrap();
    assert_ne!(rescanned.fingerprint(&rescanned.load(&api())), first);
}

#[test]
fn builtin_is_pinned_first() {
    let mut profile = Profile::new("p", [data_id(), builtin_id()]);
    assert!(pin_builtin(&mut profile));
    assert_eq!(profile.enabled, [builtin_id(), data_id()]);
    assert!(!pin_builtin(&mut profile));
    let mut empty = Profile::new("p", []);
    assert!(pin_builtin(&mut empty));
    assert_eq!(empty.enabled, [builtin_id()]);
}

#[test]
fn disabled_plugin_drops_its_archive_and_dependents() {
    let dir = tempfile::tempdir().unwrap();
    let install = install(dir.path());
    let user = UserDirs::new(dir.path().join("user"));
    let mut profile = default_profile(&install);
    profile.enabled.retain(|m| m.as_str() != "morrowind.esm");
    profile.save(&user.profiles()).unwrap();
    let mods = Mods::open(MwAdapter::new(install, builtin()), user, DEFAULT_PROFILE).unwrap();
    let loaded = mods.load(&api());
    assert_eq!(ids(&loaded), ["mw", "morrowind-data", "morrowind-weather"]);
    assert!(loaded.report.issues.iter().any(|i| matches!(i.kind, IssueKind::MissingDependency { .. })));
    // Morrowind.bsa went with its plugin; the unclaimed Extra.bsa stays.
    assert_eq!(loaded.vfs.read(&p("textures/a.dds")).unwrap(), b"builtin");
    assert_eq!(loaded.vfs.read(&p("textures/e.dds")).unwrap(), b"extra");
}

#[test]
fn ensure_weather_adds_it_after_the_loose_data() {
    let mut profile = Profile::new("p", [builtin_id(), data_id(), ModId::new("later").unwrap()]);
    assert!(ensure_weather(&mut profile));
    assert_eq!(profile.enabled[2], weather_id());
    assert!(!ensure_weather(&mut profile));
    let mut bare = Profile::new("p", [builtin_id()]);
    assert!(ensure_weather(&mut bare));
    assert_eq!(bare.enabled, [builtin_id(), weather_id()]);
}
