use super::*;

#[derive(Clone, Debug, PartialEq)]
struct Item {
    name: String,
    value: u32,
}

fn item(name: &str, value: u32) -> Item {
    Item { name: name.into(), value }
}

fn db() -> RecordDb {
    let mut db = RecordDb::new();
    db.register::<Item>("ITEM").unwrap();
    db
}

#[test]
fn ids_are_case_insensitive() {
    assert_eq!(RecordId::new("Fargoth"), RecordId::new("fARGOTH"));
    assert_eq!(RecordId::from(String::from("Fargoth")).as_str(), "fargoth");
}

#[test]
fn last_definition_wins_and_history_is_kept() {
    let mut db = db();
    let base = db.add_source("base", &[]);
    let modded = db.add_source("mod", &[base]);
    db.define(base, "gold".into(), item("Gold", 1)).unwrap();
    db.define(modded, "GOLD".into(), item("Gold", 5)).unwrap();

    assert_eq!(db.get::<Item>(&"gold".into()), Some(&item("Gold", 5)));
    let entry = db.entry::<Item>(&"gold".into()).unwrap();
    assert_eq!(entry.defined_by(), Some(modded));
    assert_eq!(entry.history.len(), 2);
    assert_eq!(db.len::<Item>(), 1);
}

#[test]
fn patch_requires_existing_record() {
    let mut db = db();
    let s = db.add_source("s", &[]);
    let err = db.patch::<Item>(s, &"nope".into(), |i| i.value = 2).unwrap_err();
    assert!(matches!(err, RecordError::PatchMissing { .. }));

    db.define(s, "a".into(), item("A", 1)).unwrap();
    db.patch::<Item>(s, &"a".into(), |i| i.value = 2).unwrap();
    assert_eq!(db.get::<Item>(&"a".into()).unwrap().value, 2);
    // Define + patch from the same source collapse into one touch.
    assert_eq!(db.entry::<Item>(&"a".into()).unwrap().history.len(), 1);
}

#[test]
fn upsert_defines_then_patches() {
    let mut db = db();
    let a = db.add_source("a", &[]);
    let b = db.add_source("b", &[a]);
    db.upsert(a, "cell".into(), || item("Cell", 1), |_| unreachable!()).unwrap();
    db.upsert(b, "cell".into(), || unreachable!(), |c: &mut Item| c.value += 10).unwrap();
    assert_eq!(db.get::<Item>(&"cell".into()).unwrap().value, 11);
}

#[test]
fn delete_hides_record_and_redefine_restores_it() {
    let mut db = db();
    let a = db.add_source("a", &[]);
    let b = db.add_source("b", &[a]);
    let c = db.add_source("c", &[b]);
    db.define(a, "x".into(), item("X", 1)).unwrap();
    db.delete::<Item>(b, "x".into()).unwrap();
    assert_eq!(db.get::<Item>(&"x".into()), None);
    assert_eq!(db.len::<Item>(), 0);
    assert_eq!(db.iter::<Item>().count(), 0);

    db.define(c, "x".into(), item("X", 3)).unwrap();
    assert_eq!(db.len::<Item>(), 1);
}

#[test]
fn unregistered_kind_errors() {
    let mut db = RecordDb::new();
    let s = db.add_source("s", &[]);
    assert!(matches!(db.define(s, "a".into(), 5u8), Err(RecordError::UnregisteredKind(_))));
    assert_eq!(db.get::<u8>(&"a".into()), None);
}

#[test]
fn duplicate_kind_name_for_other_type_errors() {
    let mut db = db();
    db.register::<Item>("ITEM").unwrap();
    assert!(matches!(db.register::<u8>("ITEM"), Err(RecordError::DuplicateKindName(_))));
}

#[test]
fn iteration_follows_first_definition_order() {
    let mut db = db();
    let a = db.add_source("a", &[]);
    let b = db.add_source("b", &[a]);
    db.define(a, "first".into(), item("1", 1)).unwrap();
    db.define(a, "second".into(), item("2", 2)).unwrap();
    db.define(b, "first".into(), item("1b", 1)).unwrap();
    let names: Vec<&str> = db.iter::<Item>().map(|(_, i)| i.name.as_str()).collect();
    assert_eq!(names, ["1b", "2"]);
}

#[test]
fn overriding_your_masters_is_not_a_conflict() {
    // base <- dlc1 <- dlc2: each layer knowingly overrides the previous.
    let mut db = db();
    let base = db.add_source("base", &[]);
    let dlc1 = db.add_source("dlc1", &[base]);
    let dlc2 = db.add_source("dlc2", &[dlc1]); // transitively depends on base
    db.define(base, "x".into(), item("X", 1)).unwrap();
    db.define(dlc1, "x".into(), item("X", 2)).unwrap();
    db.define(dlc2, "x".into(), item("X", 3)).unwrap();

    let report = db.conflict_report();
    assert_eq!(report.total_conflicts(), 0);
    assert_eq!(report.kinds[0].overridden, 1);
}

#[test]
fn independent_mods_touching_one_record_conflict() {
    let mut db = db();
    let base = db.add_source("base", &[]);
    let mod_a = db.add_source("mod_a", &[base]);
    let mod_b = db.add_source("mod_b", &[base]);
    db.define(base, "x".into(), item("X", 1)).unwrap();
    db.define(base, "y".into(), item("Y", 1)).unwrap();
    db.define(mod_a, "x".into(), item("X", 2)).unwrap();
    db.patch::<Item>(mod_b, &"x".into(), |i| i.value = 3).unwrap();
    db.define(mod_b, "y".into(), item("Y", 2)).unwrap();

    let report = db.conflict_report();
    assert_eq!(report.total_conflicts(), 1);
    let conflict = &report.kinds[0].conflicts[0];
    assert_eq!(conflict.id.as_str(), "x");
    assert_eq!(conflict.history.len(), 3);
    assert!(report.render(&db, 10).contains("mod_a (define) -> mod_b (patch)"));
}

#[test]
#[should_panic(expected = "earlier sources")]
fn sources_cannot_depend_on_later_sources() {
    let mut db = db();
    db.add_source("a", &[SourceId(3)]);
}
