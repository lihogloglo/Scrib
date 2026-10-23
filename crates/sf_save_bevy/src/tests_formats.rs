//! Format guards (save.md "Versioning and migration"):
//!
//! - **Shape goldens**: every saved type's serde shape (`serde-reflection`) at its section's
//!   current version, in `tests/shapes/<owner>.<family>.v<version>.json`. A shape change without
//!   a version bump fails here: bump the version, freeze the old type in a `legacy` module, add a
//!   typed step, then bless the new golden. Semantic changes with the same shape (units, meanings)
//!   need a bump too; reviewers check.
//! - **Golden save**: `tests/golden/s2.sfsave`, the canonical played yard written by S2. It must
//!   keep loading into the same state and re-save to the same raw section bytes.
//!
//! Bless (write missing or changed goldens) with `SF_SAVE_BLESS=1`.

use std::path::{Path, PathBuf};

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_reflection::{Samples, Tracer, TracerConfig};
use sf_save::{SaveFile, SaveMeta};

use crate::sections::{self, CharacterSave, ClockSave, OverlaySave, RecordOverlay, WeatherSave};
use sf_rpg::WorldRpg;

use crate::tests::{Overlay, app, load, new_game, play, state, wire, yard};
use crate::{LoadRequest, SaveReason, SaveRequest};

fn blessing() -> bool {
    std::env::var_os("SF_SAVE_BLESS").is_some()
}

fn data_dir(sub: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join(sub)
}

/// The serde shape of `T`, traced by deserialization so every field and every variant of the
/// enums named in `enums` is explored. A type whose deserializer needs real data (the overlay's
/// embedded codec) is traced from `sample` by serialization instead: it has no enums, and the
/// sample must fill every `Option` and collection.
fn shape<T: Serialize + DeserializeOwned>(sample: Option<&T>, enums: &[&TraceEnum]) -> String {
    let mut tracer = Tracer::new(TracerConfig::default());
    let mut samples = Samples::new();
    for e in enums {
        e(&mut tracer, &samples);
    }
    match sample {
        Some(sample) => drop(tracer.trace_value(&mut samples, sample).unwrap()),
        None => drop(tracer.trace_type::<T>(&samples).unwrap()),
    }
    let registry = tracer.registry().unwrap();
    serde_json::to_string_pretty(&registry).unwrap() + "\n"
}

/// Traces one enum type (every variant).
type TraceEnum = dyn Fn(&mut Tracer, &Samples);

fn traced<E: DeserializeOwned>(t: &mut Tracer, s: &Samples) {
    t.trace_type::<E>(s).unwrap();
}

fn check_golden(name: &str, text: &str) {
    let path = data_dir("shapes").join(format!("{name}.json"));
    match std::fs::read_to_string(&path) {
        Ok(golden) if golden == text => {}
        _ if blessing() => {
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, text).unwrap();
        }
        Ok(_) => panic!(
            "the saved shape of {name} changed. Bump the section version, freeze the old type, add a migration step, then bless with SF_SAVE_BLESS=1. Diff: {} vs the new shape:\n{text}",
            path.display()
        ),
        Err(e) => panic!("no shape golden {} ({e}): bless it with SF_SAVE_BLESS=1", path.display()),
    }
}

#[test]
fn saved_shapes_match_their_goldens() {
    let dir = tempfile::tempdir().unwrap();
    let (mut a, y) = new_game(dir.path());
    play(&mut a, &y);
    let v = |name: &str, version: u32| format!("sf.{name}.v{version}");

    check_golden(&format!("sf.meta.v{}", sf_save::META_VERSION), &shape::<SaveMeta>(None, &[&traced::<SaveReason>]));
    check_golden(&v(sections::CLOCK, sections::CLOCK_VERSION), &shape::<ClockSave>(None, &[]));
    check_golden(
        &v(sections::WEATHER, sections::WEATHER_VERSION),
        &shape::<WeatherSave>(None, &[&traced::<sf_weather::Override>, &traced::<sf_wind::WindSourceKind>]),
    );
    check_golden(
        &v(sections::RECORDS, sections::RECORDS_VERSION),
        &shape::<RecordOverlay>(None, &[&traced::<sections::SavedValue>, &traced::<sections::SavedOp>]),
    );
    let overlay = OverlaySave { overlay: a.world().resource::<Overlay>().0.clone(), blob_versions: [("actor".to_owned(), 1)].into() };
    check_golden(&v(sections::OVERLAY, sections::OVERLAY_VERSION), &shape(Some(&overlay), &[]));
    check_golden(&v(sections::RPG, sections::RPG_VERSION), &shape::<WorldRpg>(None, &[]));
    check_golden(
        &v(sections::CHARACTER, sections::CHARACTER_VERSION),
        &shape::<CharacterSave>(None, &[&traced::<sf_rpg::AttrValue>, &traced::<sf_controller::MoverMode>, &traced::<sf_action::Mode>]),
    );
    // `<mod id>/script`: the runtime's snapshot bytes are `ContextSnapshot` (any owner).
    #[cfg(feature = "script")]
    {
        use sf_script_bevy::sf_script::{ContextSnapshot, StateKey, StateValue};
        check_golden(
            &format!("any.{}.v{}", sections::SCRIPT, sections::SCRIPT_VERSION),
            &shape::<ContextSnapshot>(None, &[&traced::<StateValue>, &traced::<StateKey>]),
        );
    }
}

#[test]
fn the_s2_golden_save_still_loads_and_resaves_identically() {
    let path = data_dir("golden").join("s2.sfsave");
    let dir = tempfile::tempdir().unwrap();
    if !path.exists() {
        assert!(blessing(), "no golden save {}: bless it with SF_SAVE_BLESS=1", path.display());
        let (mut a, y) = new_game(dir.path());
        play(&mut a, &y);
        let r = crate::tests::save(&mut a, SaveRequest::to_path(&path));
        assert!(r.durable);
    }
    // The state the golden was written from.
    let (mut played, y) = new_game(dir.path());
    play(&mut played, &y);

    let mut b = app(dir.path(), false);
    wire(&mut b);
    let report = load(&mut b, LoadRequest::path(&path)).unwrap();
    assert!(report.migrated.is_empty() && report.dormant.is_empty(), "{report:?}");
    assert_eq!(state(&b), state(&played));
    let again = crate::tests::save(&mut b, SaveRequest::slot(SaveReason::Manual));
    let golden = SaveFile::open(std::fs::read(&path).unwrap()).unwrap();
    let resaved = SaveFile::open(std::fs::read(&again.path).unwrap()).unwrap();
    let raw = |f: &SaveFile| -> Vec<(String, u32, Vec<u8>)> {
        f.entries().iter().skip(1).map(|e| (e.key.to_string(), e.version, f.raw(&e.key).unwrap())).collect()
    };
    assert_eq!(raw(&golden), raw(&resaved));
    let _ = yard();
}
