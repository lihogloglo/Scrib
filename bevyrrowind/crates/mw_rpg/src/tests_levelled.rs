//! Levelled item lists (DQ-5f4): `getLevelledItem` and `addInitialItem` semantics over many seeds.

use std::collections::BTreeSet;

use sf_records::{RecordDb, RecordId};
use tes3::esp::{LeveledItem, LeveledItemFlags, MiscItem};

use crate::{LevelledRoll, initial_items, levelled_item};

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

/// A test list: id, flags, chance none, (item, level) entries.
type List<'a> = (&'a str, LeveledItemFlags, u8, &'a [(&'a str, u16)]);

/// `a1` (level 1), `a5`, `b5` (level 5), `a10` (level 10) as MISC; lists added per test.
fn db(lists: &[List<'_>]) -> RecordDb {
    let mut db = RecordDb::new();
    db.register::<MiscItem>("MISC").unwrap();
    db.register::<LeveledItem>("LEVI").unwrap();
    let src = db.add_source("test", &[]);
    for item in ["a1", "a5", "b5", "a10"] {
        db.define(src, id(item), MiscItem { id: item.into(), ..Default::default() }).unwrap();
    }
    for (list, flags, chance_none, items) in lists {
        let items = items.iter().map(|(i, l)| ((*i).to_string(), *l)).collect();
        let record = LeveledItem { id: (*list).into(), leveled_item_flags: *flags, chance_none: *chance_none, items, ..Default::default() };
        db.define(src, id(list), record).unwrap();
    }
    db
}

const ITEMS: &[(&str, u16)] = &[("a1", 1), ("a5", 5), ("b5", 5), ("a10", 10)];

/// Every pick of `list` at `level` over 200 seeds.
fn picks(db: &RecordDb, list: &str, level: u16) -> BTreeSet<Option<String>> {
    (0..200u64).map(|s| levelled_item(db, &id(list), level, &mut LevelledRoll::new(&[s])).map(|r| r.as_str().to_owned())).collect()
}

fn set(items: &[Option<&str>]) -> BTreeSet<Option<String>> {
    items.iter().map(|i| i.map(str::to_owned)).collect()
}

#[test]
fn chance_none_100_gives_nothing_and_0_always_gives() {
    let db = db(&[("never", LeveledItemFlags::empty(), 100, ITEMS), ("always", LeveledItemFlags::empty(), 0, &[("a1", 1)])]);
    assert_eq!(picks(&db, "never", 20), set(&[None]));
    assert_eq!(picks(&db, "always", 20), set(&[Some("a1")]));
    let mut roll = LevelledRoll::new(&[7]);
    assert!(initial_items(&db, &id("never"), 5, 20, &mut roll).is_empty(), "a 100% chance none entry adds nothing");
}

#[test]
fn from_all_levels_picks_every_entry_at_or_below_the_player() {
    let db = db(&[("all", LeveledItemFlags::CALCULATE_FROM_ALL_LEVELS, 0, ITEMS)]);
    assert_eq!(picks(&db, "all", 5), set(&[Some("a1"), Some("a5"), Some("b5")]), "level 10 is not eligible at 5");
    assert_eq!(picks(&db, "all", 4), set(&[Some("a1")]));
    assert_eq!(picks(&db, "all", 0), set(&[None]), "nothing at or below level 0");
}

#[test]
fn without_the_flag_only_the_highest_eligible_level() {
    let db = db(&[("top", LeveledItemFlags::empty(), 0, ITEMS)]);
    assert_eq!(picks(&db, "top", 7), set(&[Some("a5"), Some("b5")]));
    assert_eq!(picks(&db, "top", 30), set(&[Some("a10")]));
}

#[test]
fn a_nested_list_resolves_with_its_own_rules() {
    let db = db(&[("inner", LeveledItemFlags::empty(), 0, &[("a5", 5)]), ("outer", LeveledItemFlags::empty(), 0, &[("inner", 1)])]);
    assert_eq!(picks(&db, "outer", 3), set(&[None]), "the inner list has nothing at level 3");
    assert_eq!(picks(&db, "outer", 5), set(&[Some("a5")]));
    let looping = self::db(&[("loop", LeveledItemFlags::empty(), 0, &[("loop", 1)])]);
    assert_eq!(picks(&looping, "loop", 5), set(&[None]), "a list naming itself gives nothing");
}

#[test]
fn counts_each_flag_and_plain_ids() {
    let all = LeveledItemFlags::CALCULATE_FROM_ALL_LEVELS;
    let db = db(&[("once", all, 0, ITEMS), ("each", all | LeveledItemFlags::CALCULATE_FOR_EACH_ITEM, 0, ITEMS)]);
    let fill = |item: &str, count: i32, seed: u64| initial_items(&db, &id(item), count, 5, &mut LevelledRoll::new(&[seed]));
    assert_eq!(fill("a10", -3, 0), [(id("a10"), 3)], "a plain id, restocking count by magnitude");
    assert!(fill("a10", 0, 0).is_empty(), "count 0 adds nothing");
    for seed in 0..50 {
        let whole = fill("once", 4, seed);
        assert_eq!(whole.len(), 1, "one roll for the whole count");
        assert_eq!(whole[0].1, 4);
        let pieces = fill("each", -4, seed);
        assert_eq!(pieces.len(), 4, "one roll per piece");
        assert!(pieces.iter().all(|(_, n)| *n == 1));
    }
    assert!((0..50).any(|s| fill("each", 4, s).iter().map(|(i, _)| i).collect::<BTreeSet<_>>().len() > 1), "per-piece rolls differ");
}

#[test]
fn the_same_seed_rolls_the_same() {
    let db = db(&[("all", LeveledItemFlags::CALCULATE_FROM_ALL_LEVELS | LeveledItemFlags::CALCULATE_FOR_EACH_ITEM, 10, ITEMS)]);
    for seed in 0..50 {
        let a = initial_items(&db, &id("all"), 6, 10, &mut LevelledRoll::new(&[seed, 3]));
        let b = initial_items(&db, &id("all"), 6, 10, &mut LevelledRoll::new(&[seed, 3]));
        assert_eq!(a, b);
    }
}
