//! `sf/services` (services.md "Saves"), through the real section codec: merchant purses save and
//! load byte-identically; a save from before the section loads with no purses (every merchant
//! restocks on its next open); a newer `sf/services` is refused, naming the section, before
//! anything applies. The shape golden is `tests/shapes/sf.services.v1.json` (`tests_formats.rs`).

use sf_rpg::service::{Purse, SERVICES_SECTION, WorldServices};
use sf_rpg::{ActorKey, GameTime};
use sf_save::{SaveFile, SectionFlags};

use crate::sections;
use crate::tests::{Services, app, load, new_game, play, save, state, wire, with_extra};
use crate::{LoadError, LoadRequest, MigrateError, SaveRequest};

const ARRILLE: ActorKey = ActorKey(0x0bad_cafe_0000_0000_0000_0000_0000_0002);
const CREEPER: ActorKey = ActorKey(0x0bad_cafe_0000_0000_0000_0000_0000_0003);

/// Two purses: one spent below its base after a restock on day 2, one above it (the player sold).
fn purses() -> WorldServices {
    let mut services = WorldServices::default();
    services.purses.insert(
        ARRILLE,
        Purse {
            gold: 137,
            last_restock: GameTime {
                day: 2,
                minute: 615,
            },
        },
    );
    services.purses.insert(
        CREEPER,
        Purse {
            gold: 6_500,
            last_restock: GameTime { day: 0, minute: 1 },
        },
    );
    services
}

#[test]
fn merchant_purses_round_trip_through_the_section_codec() {
    let dir = tempfile::tempdir().unwrap();
    let (mut a, y) = new_game(dir.path());
    play(&mut a, &y);
    a.world_mut().resource_mut::<Services>().0 = purses();
    let first = save(&mut a, SaveRequest::quick());
    let f = SaveFile::open(std::fs::read(&first.path).unwrap()).unwrap();
    let key = sections::key(sections::SERVICES);
    assert_eq!(key.to_string(), SERVICES_SECTION);
    assert_eq!(f.entry(&key).unwrap().version, sections::SERVICES_VERSION);

    // A fresh game loads the same purses, and re-saves the same section bytes.
    let (mut b, _) = new_game(dir.path());
    load(&mut b, LoadRequest::path(&first.path)).unwrap();
    assert_eq!(b.world().resource::<Services>().0, purses());
    assert_eq!(state(&b), state(&a));
    let again = save(&mut b, SaveRequest::quick());
    let raw = |p: &std::path::Path| {
        SaveFile::open(std::fs::read(p).unwrap())
            .unwrap()
            .raw(&key)
            .unwrap()
    };
    assert_eq!(raw(&again.path), raw(&first.path));

    // The loaded purse restocks on the same clock: due 72 h after day 2 10:15.
    let mut loaded = b.world().resource::<Services>().0.clone();
    let rule = sf_rpg::service::PurseRule::Pool {
        base: 500,
        reset_hours: 72,
    };
    assert_eq!(
        loaded.open(
            ARRILLE,
            rule,
            GameTime {
                day: 5,
                minute: 614
            }
        ),
        None
    );
    assert_eq!(
        loaded.open(
            ARRILLE,
            rule,
            GameTime {
                day: 5,
                minute: 615
            }
        ),
        Some(500)
    );
}

#[test]
fn a_save_from_before_the_section_loads_with_no_purses() {
    let dir = tempfile::tempdir().unwrap();
    // The released S2 golden (written before `sf/services` existed).
    let s2 = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden/s2.sfsave");
    let f = SaveFile::open(std::fs::read(&s2).unwrap()).unwrap();
    assert!(f.entry(&sections::key(sections::SERVICES)).is_none());
    let mut b = app(dir.path(), false);
    wire(&mut b);
    b.world_mut().resource_mut::<Services>().0 = purses();
    let report = load(&mut b, LoadRequest::path(&s2)).unwrap();
    assert!(report.dormant.is_empty(), "{report:?}");
    // Full replace: no purses, so each merchant starts from its base on its next open.
    assert_eq!(b.world().resource::<Services>().0, WorldServices::default());
    // And a world without purses writes no `sf/services`.
    let again = save(&mut b, SaveRequest::quick());
    assert!(
        SaveFile::open(std::fs::read(&again.path).unwrap())
            .unwrap()
            .entry(&sections::key(sections::SERVICES))
            .is_none()
    );
}

#[test]
fn a_newer_sf_services_is_refused_before_anything_applies() {
    let dir = tempfile::tempdir().unwrap();
    let (mut a, _) = new_game(dir.path());
    let base = save(&mut a, SaveRequest::quick());
    let (mut b, _) = new_game(dir.path());
    b.world_mut().resource_mut::<Services>().0 = purses();
    let before = state(&b);
    let key = sections::key(sections::SERVICES);
    let newer = with_extra(
        &base.path,
        &[(
            key.clone(),
            sections::SERVICES_VERSION + 1,
            SectionFlags::NONE,
            vec![0],
        )],
    );
    let err = load(&mut b, LoadRequest::path(&newer)).unwrap_err();
    assert_eq!(
        err,
        LoadError::Migrate(MigrateError::TooNew {
            key,
            version: sections::SERVICES_VERSION + 1,
            current: sections::SERVICES_VERSION
        })
    );
    assert!(err.to_string().contains("sf/services"), "{err}");
    assert_eq!(state(&b), before);
}
