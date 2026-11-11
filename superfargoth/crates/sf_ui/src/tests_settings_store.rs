//! The store's change log and the `Machine` scope's file (menus.md "Settings v2").

use sf_mods::{ModId, Profile};
use sf_records::{RecordDb, RecordFormat, apply_record_file};

use crate::*;

/// `m.res` (Machine choice), `m.vsync` (Machine bool), `m.subs` (Profile bool).
fn registry() -> UiRegistry {
    let mut db = RecordDb::new();
    register_kinds(&mut db).unwrap();
    let src = db.add_source("m", &[]);
    let text = r#"
[[define]]
kind = "ui.setting"
id = "m.res"
value = { label = "res", kind = { Choice = { default = "1080p", options = ["1080p", "1440p"] } }, scope = "Machine" }

[[define]]
kind = "ui.setting"
id = "m.vsync"
value = { label = "vsync", kind = { Bool = { default = true } }, scope = "Machine" }

[[define]]
kind = "ui.setting"
id = "m.subs"
value = { label = "subs", kind = { Bool = { default = false } } }
"#;
    let outcome = apply_record_file(&mut db, src, text, RecordFormat::Toml).unwrap();
    assert_eq!(outcome.errors, []);
    let (reg, conflicts) = UiRegistry::build(&db);
    assert_eq!(conflicts, []);
    reg
}

fn text(s: &str) -> SettingValue {
    SettingValue::Text(s.into())
}

#[test]
fn a_set_logs_one_change_with_old_and_new() {
    let mut store = SettingsStore::new(&registry());
    assert!(!store.has_changes(), "defaults are not changes");
    store.set("m.res", text("1440p")).unwrap();
    assert_eq!(
        store.take_changes(),
        [SettingChange {
            key: SettingKey("m.res".into()),
            old: Some(text("1080p")),
            new: text("1440p"),
        }]
    );
    assert_eq!(store.take_changes(), [], "drained");

    store.set("m.res", text("1440p")).unwrap();
    assert!(!store.has_changes(), "the same value is no change");
    assert_eq!(store.take_changes(), []);

    store.set("m.subs", SettingValue::Bool(true)).unwrap();
    store.set("m.subs", SettingValue::Bool(false)).unwrap();
    assert_eq!(store.take_changes(), [], "changed back before the drain");

    store.set("m.subs", SettingValue::Bool(true)).unwrap();
    store.reset("m.res");
    assert_eq!(
        store.take_changes(),
        [
            SettingChange {
                key: SettingKey("m.subs".into()),
                old: Some(SettingValue::Bool(false)),
                new: SettingValue::Bool(true),
            },
            SettingChange {
                key: SettingKey("m.res".into()),
                old: Some(text("1440p")),
                new: text("1080p"),
            },
        ],
        "first-write order; a reset is a change"
    );
    assert!(store.set("m.res", text("4k")).is_err());
    assert_eq!(store.take_changes(), [], "a refused value changes nothing");
}

#[test]
fn machine_keys_go_to_the_machine_file_and_profile_keys_to_the_profile() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("user").join("machine.toml");
    let reg = registry();
    let mut store = SettingsStore::new(&reg);
    assert!(
        !store.save_machine(&path).unwrap(),
        "nothing to store, no file"
    );
    assert!(!path.exists());

    store.set("m.res", text("1440p")).unwrap();
    store.set("m.vsync", SettingValue::Bool(false)).unwrap();
    store.set("m.subs", SettingValue::Bool(true)).unwrap();
    let mut profile = Profile::new("p", []);
    assert!(store.save_to(&mut profile));
    assert!(store.save_machine(&path).unwrap());

    let m = ModId::new("m").unwrap();
    assert_eq!(
        profile.settings[&m],
        toml::from_str::<toml::Table>("subs = true").unwrap(),
        "only the Profile key in the profile"
    );
    let file: toml::Table = toml::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(
        file,
        toml::from_str::<toml::Table>("[m]\nres = \"1440p\"\nvsync = false").unwrap(),
        "only the Machine keys in the machine file"
    );

    let mut fresh = SettingsStore::new(&reg);
    assert_eq!(fresh.load_from(&profile), []);
    assert_eq!(
        fresh.get("m.res"),
        Some(&text("1080p")),
        "not from the profile"
    );
    assert_eq!(fresh.get_bool("m.subs"), Some(true));
    assert_eq!(fresh.load_machine(&path).unwrap(), []);
    assert_eq!(fresh.get("m.res"), Some(&text("1440p")));
    assert_eq!(fresh.get_bool("m.vsync"), Some(false));
    assert_eq!(fresh.take_changes().len(), 3, "loads are changes too");

    // A Machine key found in the profile is ignored; a Profile key found in
    // the machine file too.
    let mut stray = Profile::new("p", []);
    stray.settings.insert(
        m.clone(),
        toml::from_str("res = \"1440p\"\nsubs = true").unwrap(),
    );
    let mut other = SettingsStore::new(&reg);
    assert_eq!(other.load_from(&stray), []);
    assert_eq!(other.get("m.res"), Some(&text("1080p")));
    std::fs::write(&path, "[m]\nsubs = true\nres = \"1440p\"\n").unwrap();
    let mut other = SettingsStore::new(&reg);
    assert_eq!(other.load_machine(&path).unwrap(), []);
    assert_eq!(other.get_bool("m.subs"), Some(false));
    assert_eq!(other.get("m.res"), Some(&text("1440p")));

    // Unknown machine entries survive a save; the profile never gets them.
    std::fs::write(&path, "[gone]\nkeep = 7\n").unwrap();
    let mut other = SettingsStore::new(&reg);
    assert_eq!(other.load_machine(&path).unwrap(), []);
    other.set("m.vsync", SettingValue::Bool(false)).unwrap();
    assert!(other.save_machine(&path).unwrap());
    let file: toml::Table = toml::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(
        file,
        toml::from_str::<toml::Table>("[gone]\nkeep = 7\n[m]\nvsync = false").unwrap()
    );
    let mut p = Profile::new("p", []);
    other.save_to(&mut p);
    assert!(p.settings.is_empty());
}

#[test]
fn a_corrupt_machine_file_gives_defaults_and_an_error() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("machine.toml");
    let reg = registry();
    assert_eq!(
        SettingsStore::new(&reg).load_machine(&path).unwrap(),
        [],
        "a missing file is a first run"
    );

    for bad in ["[m\nres = ", "m = 3\n"] {
        std::fs::write(&path, bad).unwrap();
        let mut store = SettingsStore::new(&reg);
        store.set("m.res", text("1440p")).unwrap();
        store.set("m.subs", SettingValue::Bool(true)).unwrap();
        let err = store.load_machine(&path).unwrap_err();
        assert!(matches!(err, MachineFileError::Parse { .. }), "{err}");
        assert_eq!(store.get("m.res"), Some(&text("1080p")), "{bad:?}");
        assert_eq!(store.get_bool("m.vsync"), Some(true));
        assert_eq!(
            store.get_bool("m.subs"),
            Some(true),
            "Profile keys untouched"
        );
    }

    std::fs::write(&path, "[m]\nres = \"8k\"\nvsync = 1\n").unwrap();
    let mut store = SettingsStore::new(&reg);
    let errors = store.load_machine(&path).unwrap();
    assert_eq!(
        errors,
        [
            SettingError::NotAnOption {
                key: "m.res".into(),
                value: "8k".into(),
            },
            SettingError::WrongType {
                key: "m.vsync".into(),
                expected: "a bool",
            },
        ]
    );
    assert_eq!(store.get("m.res"), Some(&text("1080p")));
}
