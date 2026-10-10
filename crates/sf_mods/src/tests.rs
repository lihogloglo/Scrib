use super::*;

fn id(s: &str) -> ModId {
    ModId::new(s).unwrap()
}

fn manifest(toml_text: &str) -> ModManifest {
    ModManifest::from_toml(toml_text).unwrap()
}

fn simple(name: &str) -> ModManifest {
    ModManifest::synthesized(id(name), name, Version::new(1, 0, 0))
}

fn order_of(installed: &[ModManifest], profile: &[&str]) -> Result<Vec<String>, LoadOrderError> {
    let profile: Vec<ModId> = profile.iter().map(|s| id(s)).collect();
    resolve_load_order(installed, &profile).map(|o| o.iter().map(|m| m.id.to_string()).collect())
}

#[test]
fn parses_full_manifest() {
    let m = manifest(
        r#"
        id = "better-bodies"
        name = "Better Bodies"
        version = "2.2.0"
        api = "^0.1"
        description = "Anatomy."
        load_after = ["tribunal"]

        [dependencies]
        morrowind = ">=1.6"

        [content]
        assets = ["assets", "assets-hd"]
        records = ["records/bodies.ron"]
        scripts = ["scripts/init.luau"]
        "#,
    );
    assert_eq!(m.id.as_str(), "better-bodies");
    assert_eq!(m.version, Version::new(2, 2, 0));
    assert!(m.dependencies[&id("morrowind")].matches(&Version::new(1, 6, 1820)));
    assert_eq!(m.content.assets.len(), 2);
    assert_eq!(m.load_after, [id("tribunal")]);
}

#[test]
fn rejects_unknown_fields_and_bad_ids() {
    assert!(ModManifest::from_toml("id = \"a\"\nname = \"A\"\nversion = \"1.0.0\"\ntypo = 1").is_err());
    assert!(ModManifest::from_toml("id = \"Has Spaces\"\nname = \"A\"\nversion = \"1.0.0\"").is_err());
    assert!(ModId::new("").is_err());
    assert!(ModId::new("UPPER").is_err());
}

#[test]
fn load_dir_reads_mod_toml() {
    let dir = std::env::temp_dir().join(format!("sf_mods_test_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join(MANIFEST_FILE), "id = \"x\"\nname = \"X\"\nversion = \"0.1.0\"").unwrap();
    let m = ModManifest::load_dir(&dir).unwrap();
    assert_eq!(m.root.as_deref(), Some(dir.as_path()));
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn profile_order_is_kept_without_constraints() {
    let installed = [simple("a"), simple("b"), simple("c")];
    assert_eq!(order_of(&installed, &["c", "a", "b"]).unwrap(), ["c", "a", "b"]);
}

#[test]
fn dependencies_load_first_and_disturb_order_minimally() {
    let mut patch = simple("patch");
    patch.dependencies.insert(id("base"), VersionReq::parse("^1").unwrap());
    let installed = [simple("base"), simple("other"), patch];
    // User put the patch first; it must move after base but nothing else moves.
    assert_eq!(order_of(&installed, &["patch", "other", "base"]).unwrap(), ["other", "base", "patch"]);
}

#[test]
fn hints_only_apply_to_enabled_mods() {
    let mut a = simple("a");
    a.load_after.push(id("b"));
    a.load_after.push(id("not-enabled"));
    let mut c = simple("c");
    c.load_before.push(id("a"));
    let installed = [a, simple("b"), c];
    assert_eq!(order_of(&installed, &["a", "b", "c"]).unwrap(), ["b", "c", "a"]);
}

#[test]
fn reports_missing_unknown_and_mismatched() {
    let mut needs = simple("needs");
    needs.dependencies.insert(id("base"), VersionReq::parse("^2").unwrap());
    let installed = [simple("base"), needs];

    assert_eq!(
        order_of(&installed, &["needs"]),
        Err(LoadOrderError::MissingDependency { dependent: id("needs"), dependency: id("base") })
    );
    assert!(matches!(order_of(&installed, &["base", "needs"]), Err(LoadOrderError::VersionMismatch { .. })));
    assert_eq!(order_of(&installed, &["ghost"]), Err(LoadOrderError::UnknownMod(id("ghost"))));
    assert_eq!(order_of(&installed, &["base", "base"]), Err(LoadOrderError::DuplicateInProfile(id("base"))));
    assert_eq!(order_of(&[simple("x"), simple("x")], &["x"]), Err(LoadOrderError::DuplicateMod(id("x"))));
}

#[test]
fn detects_cycles() {
    let mut a = simple("a");
    a.load_after.push(id("b"));
    let mut b = simple("b");
    b.load_after.push(id("a"));
    let installed = [a, b, simple("free")];
    assert_eq!(order_of(&installed, &["a", "b", "free"]), Err(LoadOrderError::Cycle(vec![id("a"), id("b")])));
}
