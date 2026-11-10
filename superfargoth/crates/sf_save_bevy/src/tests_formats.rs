//! Format guards (save.md "Versioning and migration"):
//!
//! - **Shape goldens**: every saved type's serde shape (`serde-reflection`) at its section's
//!   current version, in `tests/shapes/<owner>.<family>.v<version>.json`. A shape change without
//!   a version bump fails here: bump the version, freeze the old type in a `legacy` module, add a
//!   typed step, then bless the new golden. Semantic changes with the same shape (units, meanings)
//!   need a bump too; reviewers check.
//! - **Golden saves**: `tests/golden/s2.sfsave`, the canonical played yard written by S2
//!   (`sf/character` v1, `sf/weather` v1), and `tests/golden/character-v2.sfsave` (`sf/weather`
//!   v1), `weather-v2.sfsave` and `character-v3.sfsave` (`sf/character` v2 and v3), must keep
//!   loading (through migration) into the same state; [`CURRENT`], the same yard
//!   at the current versions, must also re-save to the same raw section bytes. Released goldens
//!   are never re-blessed.
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
        &v(sections::AI, sections::AI_VERSION),
        &shape::<sf_ai::AiSave>(
            None,
            &[&traced::<sf_ai::DirectiveKind>, &traced::<sf_ai::DirectiveSource>, &traced::<sf_ai::Limit>, &traced::<sf_ai::Serving>],
        ),
    );
    check_golden(&v(sections::SERVICES, sections::SERVICES_VERSION), &shape::<sf_rpg::service::WorldServices>(None, &[]));
    check_golden(&v(sections::EFFECTS, sections::EFFECTS_VERSION), &shape::<sections::EffectsWorldSave>(None, &[]));
    check_golden(
        &v(sections::CHARACTER, sections::CHARACTER_VERSION),
        &shape::<CharacterSave>(
            None,
            &[
                &traced::<sf_rpg::AttrValue>,
                &traced::<sf_controller::MoverMode>,
                &traced::<sf_action::Mode>,
                &traced::<sf_stats::StatOwner>,
                &traced::<sf_effects::SourceClass>,
                &traced::<sf_effects::ArgRef>,
                &traced::<sf_effects::Phase>,
            ],
        ),
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

/// Frozen older types keep their old shapes: their leaves are still live types, so a change to
/// one (`ActionClock`, `Timers`, `Locomotion`) would silently break the migration's input.
#[test]
fn legacy_shapes_match_their_goldens() {
    check_golden(
        &format!("sf.{}.v1", sections::WEATHER),
        &shape::<sections::weather_legacy::v1::WeatherSave>(None, &[&traced::<sf_weather::Override>, &traced::<sf_wind::WindSourceKind>]),
    );
    check_golden(
        &format!("sf.{}.v1", sections::CHARACTER),
        &shape::<sections::character::legacy::v1::CharacterSave>(
            None,
            &[&traced::<sf_rpg::AttrValue>, &traced::<sf_controller::MoverMode>, &traced::<sf_action::Mode>],
        ),
    );
    check_golden(
        &format!("sf.{}.v2", sections::CHARACTER),
        &shape::<sections::character::legacy::v2::CharacterSave>(
            None,
            &[&traced::<sf_rpg::AttrValue>, &traced::<sf_controller::MoverMode>, &traced::<sf_action::Mode>],
        ),
    );
    check_golden(
        &format!("sf.{}.v3", sections::CHARACTER),
        &shape::<sections::character::legacy::v3::CharacterSave>(
            None,
            &[&traced::<sf_rpg::AttrValue>, &traced::<sf_controller::MoverMode>, &traced::<sf_action::Mode>],
        ),
    );
}

/// The golden save `name`, written from the played yard when blessing and missing.
fn golden_save(name: &str, dir: &Path) -> PathBuf {
    let path = data_dir("golden").join(name);
    if !path.exists() {
        assert!(blessing(), "no golden save {}: bless it with SF_SAVE_BLESS=1", path.display());
        let (mut a, y) = new_game(dir);
        play(&mut a, &y);
        let r = crate::tests::save(&mut a, SaveRequest::to_path(&path));
        assert!(r.durable);
    }
    path
}

fn raw_sections(path: &Path) -> Vec<(String, u32, Vec<u8>)> {
    let f = SaveFile::open(std::fs::read(path).unwrap()).unwrap();
    f.entries().iter().skip(1).map(|e| (e.key.to_string(), e.version, f.raw(&e.key).unwrap())).collect()
}

/// Loads `golden` into a fresh app: it must give the played yard's state and migrate exactly
/// `migrated`. Returns the raw sections of a re-save.
fn load_golden(golden: &Path, dir: &Path, migrated: &[(sf_save::SectionKey, u32, u32)]) -> Vec<(String, u32, Vec<u8>)> {
    // The state the golden was written from.
    let (mut played, y) = new_game(dir);
    play(&mut played, &y);
    let mut b = app(dir, false);
    wire(&mut b);
    let report = load(&mut b, LoadRequest::path(golden)).unwrap();
    assert_eq!(report.migrated, migrated);
    assert!(report.dormant.is_empty(), "{report:?}");
    assert_eq!(state(&b), state(&played));
    let again = crate::tests::save(&mut b, SaveRequest::slot(SaveReason::Manual));
    raw_sections(&again.path)
}

/// The golden save at the current versions (the yard re-written when `sf/character` went to v4).
const CURRENT: &str = "character-v4.sfsave";

fn weather_step() -> (sf_save::SectionKey, u32, u32) {
    (sections::key(sections::WEATHER), 1, sections::WEATHER_VERSION)
}

fn character_step(from: u32) -> (sf_save::SectionKey, u32, u32) {
    (sections::character_key(crate::tests::HERO), from, sections::CHARACTER_VERSION)
}

/// The `sf/weather` v2 golden (`sf/character` v2: a save from before the stats sheet) loads
/// through the character step and re-saves to exactly the current golden's sections.
#[test]
fn the_weather_v2_golden_save_loads_through_the_character_step() {
    let dir = tempfile::tempdir().unwrap();
    let old = data_dir("golden").join("weather-v2.sfsave");
    assert!(old.exists(), "the weather-v2 golden is released: never re-bless it");
    let current = golden_save(CURRENT, dir.path());
    assert_eq!(load_golden(&old, dir.path(), &[character_step(2)]), raw_sections(&current));
}

/// The S2 golden (`sf/character` v1, `sf/weather` v1) loads through both v1 → v2 steps into the
/// same state, and re-saves to exactly the current golden's sections.
#[test]
fn the_s2_golden_save_still_loads_through_migration() {
    let dir = tempfile::tempdir().unwrap();
    let s2 = data_dir("golden").join("s2.sfsave");
    assert!(s2.exists(), "the S2 golden is released: never re-bless it");
    let current = golden_save(CURRENT, dir.path());
    let resaved = load_golden(&s2, dir.path(), &[character_step(1), weather_step()]);
    assert_eq!(resaved, raw_sections(&current));
}

/// The `sf/character` v2 golden (`sf/weather` v1: a save from before surface weather) loads
/// through the weather step and re-saves to exactly the current golden's sections.
#[test]
fn the_character_v2_golden_save_loads_through_the_weather_step() {
    let dir = tempfile::tempdir().unwrap();
    let old = data_dir("golden").join("character-v2.sfsave");
    assert!(old.exists(), "the character-v2 golden is released: never re-bless it");
    let current = golden_save(CURRENT, dir.path());
    assert_eq!(load_golden(&old, dir.path(), &[character_step(2), weather_step()]), raw_sections(&current));
}

/// The `sf/character` v3 golden (a save from before effects) loads through the v3 → v4 step (no
/// effects) and re-saves to exactly the current golden's sections.
#[test]
fn the_character_v3_golden_save_loads_through_the_effects_step() {
    let dir = tempfile::tempdir().unwrap();
    let old = data_dir("golden").join("character-v3.sfsave");
    assert!(old.exists(), "the character-v3 golden is released: never re-bless it");
    let current = golden_save(CURRENT, dir.path());
    assert_eq!(load_golden(&old, dir.path(), &[character_step(3)]), raw_sections(&current));
}

/// The current golden ([`CURRENT`]) loads without migration and re-saves byte for byte.
#[test]
fn the_current_golden_save_loads_and_resaves_identically() {
    let dir = tempfile::tempdir().unwrap();
    let current = golden_save(CURRENT, dir.path());
    assert_eq!(load_golden(&current, dir.path(), &[]), raw_sections(&current));
    let _ = yard();
}
