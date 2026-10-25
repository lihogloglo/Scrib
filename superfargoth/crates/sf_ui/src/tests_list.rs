//! List model extensions (sort chains, value sets, word search) and the virtual list.

use crate::{ListFilter, ListModel, SortSpec, UiValue, ViewModel, VirtualList};

#[derive(Clone, Debug, PartialEq)]
struct Row {
    name: String,
    weight: f64,
    kind: &'static str,
}

impl ViewModel for Row {
    const KIND: &'static str = "test.row";

    fn fields() -> &'static [&'static str] {
        &["name", "weight", "kind"]
    }

    fn field(&self, name: &str) -> Option<UiValue> {
        match name {
            "name" => Some(self.name.as_str().into()),
            "weight" => Some(self.weight.into()),
            "kind" => Some(self.kind.into()),
            _ => None,
        }
    }
}

fn row(name: &str, weight: f64, kind: &'static str) -> Row {
    Row {
        name: name.into(),
        weight,
        kind,
    }
}

fn spec(field: &str, descending: bool) -> SortSpec {
    SortSpec {
        field: field.into(),
        descending,
    }
}

fn model() -> ListModel<u32, Row> {
    let mut list = ListModel::new();
    for (k, r) in [
        (1, row("Iron Longsword", 20.0, "weapon")),
        (2, row("bread", 0.5, "food")),
        (3, row("Apple", 0.5, "food")),
        (4, row("iron helm", 5.0, "armor")),
        (5, row("Steel Sword", 20.0, "weapon")),
    ] {
        list.upsert(k, r);
    }
    list
}

#[test]
fn sort_chain_breaks_ties() {
    let mut list = model();
    list.set_sort_keys(vec![spec("weight", true), spec("name", false)]);
    assert_eq!(list.view(), [1, 5, 4, 3, 2], "heaviest first, ties by name");
    assert_eq!(list.sort(), Some(&spec("weight", true)));
    list.set_sort_keys(vec![spec("weight", false), spec("name", true)]);
    assert_eq!(list.view(), [2, 3, 4, 5, 1]);
    let v = list.version();
    list.set_sort_keys(vec![spec("weight", false), spec("name", true)]);
    assert_eq!(list.version(), v, "same keys: no change");
    list.set_sort(None);
    assert_eq!(list.view(), [1, 2, 3, 4, 5]);
    assert_eq!(list.position(&4), Some(3));
}

#[test]
fn filter_words_and_value_sets() {
    let mut list = model();
    list.set_filter(ListFilter {
        search: "  IRON sw ".into(),
        search_fields: vec!["name".into()],
        ..ListFilter::default()
    });
    assert_eq!(list.view(), [1], "every word, any order, case-insensitive");
    list.set_filter(ListFilter {
        search: "sw iron".into(),
        search_fields: vec!["name".into()],
        ..ListFilter::default()
    });
    assert_eq!(list.view(), [1]);
    list.set_filter(ListFilter {
        one_of: vec![("kind".into(), vec!["armor".into(), "weapon".into()])],
        ..ListFilter::default()
    });
    assert_eq!(list.view(), [1, 4, 5]);
    list.set_filter(ListFilter {
        search: "iron".into(),
        search_fields: vec!["name".into()],
        one_of: vec![("kind".into(), vec!["armor".into(), "weapon".into()])],
        equals: vec![("kind".into(), "armor".into())],
    });
    assert_eq!(list.view(), [4]);
}

#[test]
fn reorder_and_cached_keys_stay_correct() {
    let mut list = model();
    list.set_sort_keys(vec![spec("weight", false)]);
    assert_eq!(list.view(), [2, 3, 4, 1, 5]);
    // Reordering moves ties (insertion order) without touching row versions.
    let versions: Vec<_> = (1..=5).map(|k| list.row_version(&k)).collect();
    assert!(list.reorder(&[5, 4, 3, 2, 1]));
    assert!(!list.reorder(&[5, 4, 3, 2, 1]), "already in that order");
    assert_eq!(list.view(), [3, 2, 4, 5, 1]);
    assert_eq!(
        (1..=5).map(|k| list.row_version(&k)).collect::<Vec<_>>(),
        versions
    );
    assert_eq!(
        list.iter().map(|(k, _)| *k).collect::<Vec<_>>(),
        [5, 4, 3, 2, 1]
    );
    // Unnamed rows go last, keeping their order.
    assert!(list.reorder(&[1]));
    assert_eq!(
        list.iter().map(|(k, _)| *k).collect::<Vec<_>>(),
        [1, 5, 4, 3, 2]
    );

    // A changed row's cached sort key is recomputed.
    list.upsert(2, row("bread", 50.0, "food"));
    assert_eq!(list.view().last(), Some(&2));
    // Cached search text follows the searched fields and row changes.
    list.set_filter(ListFilter {
        search: "food".into(),
        search_fields: vec!["name".into()],
        ..ListFilter::default()
    });
    assert!(list.view().is_empty());
    list.set_filter(ListFilter {
        search: "food".into(),
        search_fields: vec!["kind".into()],
        ..ListFilter::default()
    });
    assert_eq!(list.view(), [3, 2]);
    list.upsert(3, row("Apple", 0.5, "fruit"));
    assert_eq!(list.view(), [2]);
}

fn big(n: u32) -> ListModel<u32, Row> {
    let mut list = ListModel::new();
    for k in 0..n {
        list.upsert(k, row(&format!("item {k:05}"), f64::from(k), "misc"));
    }
    list
}

#[test]
fn virtual_window_binds_only_visible_rows() {
    let mut list = big(1000);
    // 10 px rows, 95 px viewport: 10 rows visible (the last partly), 2 overscan below.
    let mut v = VirtualList::new(10.0, 95.0);
    let frame = v.update(&mut list);
    assert_eq!(frame.len, 1000);
    assert_eq!(frame.rows.len(), 12);
    assert!(frame.rows.iter().all(|r| r.rebind));
    assert_eq!(frame.rows[0].key, 0);
    assert_eq!(v.pool_len(), 12);
    assert!((frame.content_extent - 10_000.0).abs() < 1e-3);
    let (start, len) = frame.thumb.unwrap();
    assert!(start == 0.0 && (len - 0.0095).abs() < 1e-6);

    // Nothing changed: no rebinds.
    let frame = v.update(&mut list);
    assert!(frame.rows.iter().all(|r| !r.rebind));

    // One row down: the rows that stay keep their slot; only entering rows rebind.
    let slot_of_5 = frame.rows.iter().find(|r| r.key == 5).unwrap().slot;
    v.scroll_rows(1);
    let frame = v.update(&mut list);
    assert_eq!(frame.rows.iter().filter(|r| r.rebind).count(), 1);
    let r5 = frame.rows.iter().find(|r| r.key == 5).unwrap();
    assert_eq!((r5.slot, r5.offset, r5.rebind), (slot_of_5, 40.0, false));
    assert!(v.pool_len() <= 15, "the pool stays at window size");

    // A changed row's data rebinds that row only.
    list.upsert(6, row("renamed", 6.0, "misc"));
    let frame = v.update(&mut list);
    let rebound: Vec<u32> = frame
        .rows
        .iter()
        .filter(|r| r.rebind)
        .map(|r| r.key)
        .collect();
    assert_eq!(rebound, [6]);

    // A jump far away reuses the released slots; the pool only grows to the full window
    // (10.5 rows visible + 2 overscan each side).
    v.scroll_to(5000.0);
    let frame = v.update(&mut list);
    assert_eq!(frame.rows[0].key, 498);
    assert_eq!(frame.rows.len(), 14);
    assert!(frame.released.is_empty(), "every released slot was reused");
    assert_eq!(v.pool_len(), 14);
    v.scroll_by(6.0);
    assert_eq!(v.update(&mut list).rows.len(), 15, "11 rows partly visible");
    assert_eq!(v.pool_len(), 15);
    // Clamped at the end.
    v.scroll_to(1.0e9);
    let frame = v.update(&mut list);
    assert!((frame.scroll - 9905.0).abs() < 1e-3);
    assert_eq!(frame.rows.last().unwrap().key, 999);
    assert_eq!(v.index_at(94.0), Some(999));
    assert_eq!(v.index_at(95.0), None);
}

#[test]
fn virtual_focus_follows_keys_through_sort_and_filter() {
    let mut list = big(100);
    let mut v = VirtualList::new(10.0, 50.0);
    v.move_focus(&mut list, 1);
    assert_eq!(v.focus(), Some(&0), "first move focuses the first row");
    v.move_focus(&mut list, 7);
    let frame = v.update(&mut list);
    assert_eq!(frame.focus, Some((7, 7)));
    assert!(
        (frame.scroll - 30.0).abs() < 1e-3,
        "focus scrolled into view at the bottom"
    );
    v.page(&mut list, 1);
    assert_eq!(v.focus(), Some(&11), "a page is visible rows minus one");
    v.move_focus(&mut list, -1000);
    assert_eq!(v.focus(), Some(&0), "clamped at the top");

    // Re-sorting keeps the focused key and scrolls to its new place.
    v.set_focus(Some(10));
    list.set_sort(Some(spec("weight", true)));
    let frame = v.update(&mut list);
    assert_eq!(frame.focus, Some((89, 10)));
    let r = frame.rows.iter().find(|r| r.key == 10).unwrap();
    assert!(
        r.offset >= 0.0 && r.offset + 10.0 <= 50.0,
        "focused row visible"
    );

    // Filtering the focused row out hands focus to the row now at its index.
    list.set_filter(ListFilter {
        search: "item 0001".into(),
        search_fields: vec!["name".into()],
        ..ListFilter::default()
    });
    let frame = v.update(&mut list);
    // 00001 and 00010..00019, heaviest first.
    assert_eq!(frame.len, 11);
    assert_eq!(frame.focus, Some((9, 10)), "item 00010 still matches");
    list.remove(&10);
    let frame = v.update(&mut list);
    assert_eq!(
        frame.focus,
        Some((9, 1)),
        "the row that moved into its place"
    );
    list.clear();
    let frame = v.update(&mut list);
    assert_eq!(
        (frame.focus, frame.thumb, frame.rows.len()),
        (None, None, 0)
    );
    assert_eq!(v.focus(), None);
}

#[test]
fn virtual_scale_change_keeps_rows_in_view() {
    let mut list = big(100);
    let mut v = VirtualList::new(10.0, 50.0);
    v.scroll_to(200.0);
    assert_eq!(v.update(&mut list).rows[2].key, 20);
    v.set_row_extent(20.0);
    let frame = v.update(&mut list);
    assert!((frame.scroll - 400.0).abs() < 1e-3);
    assert_eq!(frame.rows[2].key, 20);
    v.invalidate();
    assert!(v.update(&mut list).rows.iter().all(|r| r.rebind));
}
