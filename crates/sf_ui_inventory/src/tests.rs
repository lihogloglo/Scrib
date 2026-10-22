//! Synthetic catalog and equip rules; no game data.

use std::time::Instant;

use sf_rpg::{
    AttrValue, Attrs, CharacterRpg, EquipPlacement, EquipRules, Holder, InventoryError, ItemStack,
    Loadout, PlayerId, RecordId, RpgCommand, SlotTable, StackId,
};
use sf_ui::records::{ColumnDef, SortKind, ValueSource};
use sf_ui::{UiValue, ViewModel};

use crate::{
    Better, Comparison, InventorySort, InventoryTab, InventoryView, ItemCatalog, ItemInfo, ItemRow,
    QuantityPicker, StatDef, Verdict,
};

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

struct Catalog {
    stats: Vec<StatDef>,
    /// Worn items weigh nothing (to test the carry hook).
    worn_weightless: bool,
}

impl Catalog {
    fn new() -> Self {
        let stat = |s: &str, better| StatDef {
            id: id(s),
            label: format!("stat-{s}"),
            better,
        };
        Self {
            stats: vec![
                stat("damage", Better::Higher),
                stat("armor", Better::Higher),
                stat("speed", Better::Neither),
                stat("noise", Better::Lower),
            ],
            worn_weightless: false,
        }
    }
}

impl ItemCatalog for Catalog {
    fn describe(&self, stack: &ItemStack) -> ItemInfo {
        let (name, category, weight, value, stats): (&str, &str, f32, i64, &[(&str, f64)]) =
            match stack.item.as_str() {
                "sword" => (
                    "Iron Sword",
                    "weapon",
                    10.0,
                    50,
                    &[("damage", 8.0), ("speed", 1.0)],
                ),
                "fine_sword" => (
                    "Fine Iron Sword",
                    "weapon",
                    9.0,
                    120,
                    &[("damage", 11.0), ("speed", 1.2)],
                ),
                "claymore" => (
                    "Claymore",
                    "weapon",
                    25.0,
                    90,
                    &[("damage", 20.0), ("speed", 0.8), ("noise", 3.0)],
                ),
                "shield" => (
                    "Wooden Shield",
                    "armor",
                    8.0,
                    20,
                    &[("armor", 6.0), ("noise", 1.0)],
                ),
                "helm" => ("Iron Helm", "armor", 5.0, 30, &[("armor", 4.0)]),
                "ring" => ("Copper Ring", "clothing", 0.1, 15, &[]),
                "arrow" => ("Arrow", "ammo", 0.1, 1, &[("damage", 2.0)]),
                "bread" => ("bread", "food", 0.5, 2, &[]),
                "gold" => ("Gold", "money", 0.0, 1, &[]),
                other => (other, "misc", 1.0, 1, &[]),
            };
        let condition = match stack.attrs.get(&id("condition")) {
            Some(AttrValue::Int(c)) => format!(" ({c}%)"),
            _ => String::new(),
        };
        ItemInfo {
            name: format!("{name}{condition}"),
            category: id(category),
            weight,
            value,
            icon: Some(format!("icons/{}.dds", stack.item)),
            stats: stats.iter().map(|(k, v)| (id(k), *v)).collect(),
        }
    }

    fn stats(&self) -> &[StatDef] {
        &self.stats
    }

    fn carried_weight(&self, stack: &ItemStack, info: &ItemInfo, equipped: bool) -> f64 {
        if equipped && self.worn_weightless {
            return 0.0;
        }
        f64::from(info.weight) * f64::from(stack.count)
    }
}

struct Rules {
    table: SlotTable,
}

impl EquipRules for Rules {
    fn placement(&self, stack: &ItemStack) -> Option<EquipPlacement> {
        let slot = |n: &str| self.table.id(n).unwrap();
        let (slots, whole_stack) = match stack.item.as_str() {
            "sword" | "fine_sword" => (vec![slot("right")], false),
            "claymore" => (vec![slot("right"), slot("left")], false),
            "shield" => (vec![slot("left")], false),
            "helm" => (vec![slot("head")], false),
            "ring" => (vec![slot("finger")], false),
            "arrow" => (vec![slot("ammo")], true),
            _ => return None,
        };
        Some(EquipPlacement {
            slots: slots.into_iter().collect(),
            whole_stack,
        })
    }
}

fn rules() -> Rules {
    Rules {
        table: SlotTable::new(["right", "left", "head", "finger", "ammo"]).unwrap(),
    }
}

const PLAYER: Holder = Holder::Player(PlayerId::HOST);

fn tabs() -> Vec<InventoryTab> {
    vec![
        InventoryTab::all(),
        InventoryTab::categories("gear", "inventory-tab-gear", ["weapon", "armor", "ammo"]),
        InventoryTab::categories("food", "inventory-tab-food", ["food"]),
        InventoryTab::equipped(),
    ]
}

struct Fixture {
    lo: Loadout,
    rules: Rules,
    catalog: Catalog,
    view: InventoryView,
}

impl Fixture {
    fn new() -> Self {
        let mut lo = Loadout::new();
        let rules = rules();
        for (item, n) in [
            ("sword", 1),
            ("fine_sword", 1),
            ("claymore", 1),
            ("shield", 1),
            ("helm", 1),
            ("ring", 2),
            ("arrow", 40),
            ("bread", 3),
            ("gold", 250),
        ] {
            lo.add(id(item), n, Attrs::new()).unwrap();
        }
        lo.add(
            id("sword"),
            1,
            Attrs::new().with("condition", AttrValue::Int(40)),
        )
        .unwrap();
        let sword = stack_of(&lo, "sword");
        lo.equip(sword, &rules).unwrap();
        lo.equip(stack_of(&lo, "shield"), &rules).unwrap();
        lo.equip(stack_of(&lo, "arrow"), &rules).unwrap();
        let mut f = Self {
            lo,
            rules,
            catalog: Catalog::new(),
            view: InventoryView::new(PLAYER, tabs(), 10.0, 100.0),
        };
        f.sync();
        f
    }

    fn sync(&mut self) -> crate::SyncReport {
        self.view.sync(&self.lo, &self.rules.table, &self.catalog)
    }

    fn names(&mut self) -> Vec<String> {
        let ids = self.view.visible().to_vec();
        ids.iter()
            .map(|s| self.view.row(*s).unwrap().info.name.clone())
            .collect()
    }

    /// Applies a command as the host would, then follows its events.
    fn apply(&mut self, cmd: RpgCommand) {
        let mut pc = CharacterRpg {
            loadout: std::mem::take(&mut self.lo),
        };
        let events = pc.apply(cmd).unwrap();
        self.lo = pc.loadout;
        for e in &events {
            if self.view.on_event(e) {
                self.sync();
            }
        }
    }
}

fn stack_of(lo: &Loadout, item: &str) -> StackId {
    lo.inventory()
        .stacks()
        .iter()
        .find(|s| s.item == id(item) && s.attrs.is_empty())
        .unwrap()
        .id
}

fn column(field: &str, sort: SortKind) -> ColumnDef {
    ColumnDef {
        list: crate::INVENTORY_LIST.into(),
        header: format!("col-{field}"),
        width: None,
        sort,
        value: ValueSource::Field(field.into()),
        priority: 0,
    }
}

#[test]
fn rows_markers_and_totals() {
    let mut f = Fixture::new();
    assert_eq!(f.view.rows().len(), 10);
    let sword = stack_of(&f.lo, "sword");
    let row = f.view.row(sword).unwrap();
    assert_eq!(row.field("equipped"), Some(UiValue::Bool(true)));
    assert_eq!(row.field("slots"), Some("right".into()));
    assert_eq!(row.field("stat.damage"), Some(UiValue::Float(8.0)));
    assert_eq!(row.field("stat.armor"), Some(UiValue::None));
    assert_eq!(row.field("icon"), Some("icons/sword.dds".into()));
    assert_eq!(row.field("nope"), None);
    let gold = f.view.row(stack_of(&f.lo, "gold")).unwrap();
    assert_eq!(gold.field("value_per_weight"), Some(UiValue::None));
    assert_eq!(gold.field("total_value"), Some(UiValue::Int(250)));
    let arrows = f.view.row(stack_of(&f.lo, "arrow")).unwrap();
    assert_eq!(arrows.field("slots"), Some("ammo".into()));
    // Split-equipped ring? Not equipped: the plain ring stack has no slots.
    let rings = f.view.row(stack_of(&f.lo, "ring")).unwrap();
    assert!(!rings.is_equipped());
    assert!(ItemRow::fields().contains(&"value_per_weight"));

    let t = f.view.totals();
    // 10 + 9 + 25 + 8 + 5 + 0.2 + 4 + 1.5 + 0 + 10 (the worn-out sword)
    assert!((t.encumbrance.carried - 72.7).abs() < 1e-4, "{t:?}");
    assert_eq!(t.value, 50 + 120 + 90 + 20 + 30 + 30 + 40 + 6 + 250 + 50);
    assert_eq!((t.stacks, t.pieces), (10, 301));
    f.view.set_capacity(Some(70.0));
    let e = f.view.encumbrance();
    assert!(e.is_over());
    assert!((e.remaining().unwrap() + 2.7).abs() < 1e-4);
    assert!(!e.after(-10.0).is_over());
    assert!((e.ratio().unwrap() - 72.7 / 70.0).abs() < 1e-9);
    assert_eq!(crate::Encumbrance::default().ratio(), None);

    // The carry hook decides what worn items weigh.
    f.catalog.worn_weightless = true;
    f.view.refresh(&f.lo, &f.rules.table, &f.catalog);
    assert!((f.view.totals().encumbrance.carried - (72.7 - 10.0 - 8.0 - 4.0)).abs() < 1e-4);
    assert_eq!(
        f.view.encumbrance().capacity,
        Some(70.0),
        "capacity survives a sync"
    );
}

#[test]
fn sync_only_describes_what_changed() {
    let mut f = Fixture::new();
    let r = f.sync();
    assert_eq!((r.described, r.updated, r.removed), (0, 0, 0));
    let v = f.view.rows().version();

    f.lo.add(id("bread"), 2, Attrs::new()).unwrap();
    let r = f.sync();
    assert_eq!((r.described, r.updated, r.removed), (1, 0, 0));
    assert_eq!(
        f.view.rows().changed_since(v).copied().collect::<Vec<_>>(),
        [stack_of(&f.lo, "bread")]
    );

    // Equipping the helm changes its marker only: no describe.
    f.lo.equip(stack_of(&f.lo, "helm"), &f.rules).unwrap();
    let r = f.sync();
    assert_eq!((r.described, r.updated), (0, 1));
    assert!(f.view.row(stack_of(&f.lo, "helm")).unwrap().is_equipped());

    // Dropping the gold removes its row only.
    f.lo.take(stack_of(&f.lo, "gold"), 250).unwrap();
    let r = f.sync();
    assert_eq!((r.described, r.updated, r.removed), (0, 0, 1));
    assert_eq!(f.view.rows().len(), 9);
}

#[test]
fn tabs_search_and_counts() {
    let mut f = Fixture::new();
    assert_eq!(f.view.tab_counts(), [10, 7, 1, 3]);
    f.view.set_tab(2);
    assert_eq!(f.names(), ["bread"]);
    f.view.cycle_tab(1);
    assert_eq!(f.view.tab(), 3);
    assert_eq!(f.names(), ["Iron Sword", "Wooden Shield", "Arrow"]);
    f.view.cycle_tab(1);
    assert_eq!(f.view.tab(), 0, "wraps");
    f.view.cycle_tab(-1);
    assert_eq!(f.view.tab(), 3, "wraps backwards");
    f.view.set_tab(99);
    assert_eq!(f.view.tab(), 3, "clamped");

    f.view.set_tab(1);
    f.view.set_search("iron");
    assert_eq!(
        f.names(),
        [
            "Iron Sword",
            "Fine Iron Sword",
            "Iron Helm",
            "Iron Sword (40%)"
        ]
    );
    f.view.set_search("SWORD iron");
    assert_eq!(
        f.names(),
        ["Iron Sword", "Fine Iron Sword", "Iron Sword (40%)"]
    );
    assert_eq!(
        f.view.tab_counts(),
        [10, 7, 1, 3],
        "counts ignore the search"
    );
    f.view.set_search("weapon");
    assert!(f.names().is_empty(), "the search looks at names only");
    f.view
        .set_search_fields(vec!["name".into(), "category".into()]);
    assert_eq!(f.names().len(), 4);
}

#[test]
fn sort_columns_tie_breaks_and_pinning() {
    let mut f = Fixture::new();
    f.view.set_tab(1);
    assert_eq!(
        f.names(),
        [
            "Iron Sword",
            "Fine Iron Sword",
            "Claymore",
            "Wooden Shield",
            "Iron Helm",
            "Arrow",
            "Iron Sword (40%)"
        ],
        "unsorted = inventory order"
    );
    // Numbers sort high to low first; ties by name.
    assert!(f.view.sort_by_column(&column("weight", SortKind::Number)));
    assert_eq!(
        f.names(),
        [
            "Claymore",
            "Iron Sword",
            "Iron Sword (40%)",
            "Fine Iron Sword",
            "Wooden Shield",
            "Iron Helm",
            "Arrow"
        ]
    );
    // Same column again flips; the name tie-break stays A→Z.
    assert!(f.view.sort_by_column(&column("weight", SortKind::Number)));
    assert_eq!(f.names()[..3], ["Arrow", "Iron Helm", "Wooden Shield"]);
    assert_eq!(
        f.view.sort(),
        Some(&InventorySort {
            field: "weight".into(),
            descending: false
        })
    );
    // Text sorts A→Z first; stats sort through `stat.<id>`, missing values last.
    assert!(f.view.sort_by_column(&column("name", SortKind::Text)));
    assert_eq!(f.names()[0], "Arrow");
    assert!(
        f.view
            .sort_by_column(&column("stat.damage", SortKind::Number))
    );
    assert_eq!(
        f.names(),
        [
            "Claymore",
            "Fine Iron Sword",
            "Iron Sword",
            "Iron Sword (40%)",
            "Arrow",
            "Iron Helm",
            "Wooden Shield"
        ]
    );
    f.view.set_pin_equipped(true);
    assert_eq!(f.names()[..3], ["Iron Sword", "Arrow", "Wooden Shield"]);
    // Columns that don't sort are refused and keep the current sort.
    assert!(!f.view.sort_by_column(&column("icon", SortKind::None)));
    let lua = ColumnDef {
        value: ValueSource::Lua {
            function: "vw".into(),
        },
        ..column("x", SortKind::Number)
    };
    assert!(!f.view.sort_by_column(&lua));
    assert_eq!(f.view.sort().unwrap().field, "stat.damage");
    f.view.set_sort(None);
    f.view.set_pin_equipped(false);
    assert_eq!(f.names()[0], "Iron Sword");
}

fn delta(c: &Comparison, stat: &str) -> (Option<f64>, Option<f64>, f64, Verdict) {
    let d = c.stats.iter().find(|d| d.stat == id(stat)).unwrap();
    (d.candidate, d.current, d.delta, d.verdict)
}

#[test]
fn compare_with_what_equipping_displaces() {
    let f = Fixture::new();
    let cmp = |item: &str| {
        f.view
            .compare(stack_of(&f.lo, item), &f.lo, &f.rules, &f.catalog)
    };
    // One-handed upgrade: against the worn sword only.
    let c = cmp("fine_sword").unwrap();
    assert!(!c.worn);
    assert_eq!(c.replaces, [stack_of(&f.lo, "sword")]);
    assert_eq!(c.stats.len(), 2, "only stats either side has");
    let (cand, cur, d, v) = delta(&c, "damage");
    assert_eq!((cand, cur, v), (Some(11.0), Some(8.0), Verdict::Better));
    assert!((d - 3.0).abs() < 1e-9);
    assert_eq!(delta(&c, "speed").3, Verdict::Changed);

    // Two-handed: against sword and shield together, summed per stat.
    let c = cmp("claymore").unwrap();
    assert_eq!(c.replaces.len(), 2);
    assert_eq!(
        c.stats.iter().map(|d| d.stat.as_str()).collect::<Vec<_>>(),
        ["damage", "armor", "speed", "noise"],
        "catalog order"
    );
    assert_eq!(delta(&c, "damage").3, Verdict::Better);
    assert_eq!(delta(&c, "armor"), (None, Some(6.0), -6.0, Verdict::Worse));
    // Lower noise is better: 3 against the shield's 1 is worse.
    assert_eq!(
        delta(&c, "noise"),
        (Some(3.0), Some(1.0), 2.0, Verdict::Worse)
    );

    // An empty slot: everything the candidate has is a gain.
    let c = cmp("helm").unwrap();
    assert!(c.replaces.is_empty());
    assert_eq!(delta(&c, "armor"), (Some(4.0), None, 4.0, Verdict::Better));

    // Already worn: compared with itself.
    let c = cmp("shield").unwrap();
    assert!(c.worn);
    assert!(c.stats.iter().all(|d| d.verdict == Verdict::Same));

    // Not equippable or not a stack: no panel.
    assert_eq!(cmp("bread"), None);
    assert_eq!(
        f.view.compare(StackId(999), &f.lo, &f.rules, &f.catalog),
        None
    );
}

#[test]
fn quantity_picker() {
    assert_eq!(QuantityPicker::for_split(1), None);
    assert_eq!(QuantityPicker::for_split(0), None);
    let mut p = QuantityPicker::for_split(40).unwrap();
    assert_eq!((p.min(), p.max(), p.value()), (1, 39, 20));
    p.step(-100);
    assert_eq!(p.value(), 1);
    p.step(10);
    assert_eq!(p.value(), 11);
    p.set(1000);
    assert_eq!(p.value(), 39);
    assert!((p.fraction() - 1.0).abs() < 1e-6);
    p.set_fraction(0.5);
    assert_eq!(p.value(), 20);
    let one = QuantityPicker::for_split(2).unwrap();
    assert_eq!((one.value(), one.fraction()), (1, 1.0));
    assert_eq!(QuantityPicker::new(5, 4, 4), None);
}

#[test]
fn split_and_merge_commands_round_trip_through_the_host() {
    let mut f = Fixture::new();
    let arrows = stack_of(&f.lo, "arrow");
    let actions = f.view.actions(&f.lo, arrows);
    assert!(actions.split);
    assert_eq!(actions.merge_into, None);

    let picker = f.view.split_picker(&f.lo, arrows).unwrap();
    let cmd = f.view.split_command(&f.lo, arrows, picker.value()).unwrap();
    assert_eq!(
        cmd,
        RpgCommand::SplitStack {
            holder: PLAYER,
            stack: arrows,
            count: 20
        }
    );
    f.view.set_focus(Some(arrows));
    f.apply(cmd);
    // The view synced and focused the new stack.
    let new = f.view.focus().unwrap();
    assert_ne!(new, arrows);
    assert_eq!(f.view.row(new).unwrap().stack.count, 20);
    assert!(!f.view.row(new).unwrap().is_equipped());
    assert!(f.view.row(arrows).unwrap().is_equipped());
    assert_eq!(f.view.rows().len(), 11);

    // Merge: the loose arrows go back into the quiver, which stays equipped.
    assert_eq!(f.view.actions(&f.lo, new).merge_into, Some(arrows));
    let cmd = f.view.merge_command(&f.lo, new, None).unwrap();
    f.apply(cmd);
    assert_eq!(f.view.focus(), Some(arrows), "focus follows the merge");
    assert_eq!(f.view.row(arrows).unwrap().stack.count, 40);
    assert!(f.view.row(arrows).unwrap().is_equipped());
    assert_eq!(f.view.rows().len(), 10);

    // What the screen refuses (the host would too).
    let sword = stack_of(&f.lo, "sword");
    let bread = stack_of(&f.lo, "bread");
    assert_eq!(f.view.split_picker(&f.lo, sword), None, "one piece");
    assert!(matches!(
        f.view.split_command(&f.lo, bread, 3),
        Err(InventoryError::NotEnough { .. })
    ));
    assert_eq!(
        f.view.split_command(&f.lo, bread, 0),
        Err(InventoryError::ZeroCount)
    );
    assert_eq!(
        f.view.merge_command(&f.lo, bread, None),
        Err(InventoryError::NoMergeTarget(bread))
    );
    assert_eq!(
        f.view.merge_command(&f.lo, bread, Some(sword)),
        Err(InventoryError::NotSameKind(bread, sword))
    );
    // The worn ring piece is locked.
    let rings = stack_of(&f.lo, "ring");
    let worn = f.lo.equip(rings, &f.rules).unwrap().equipped;
    f.sync();
    assert_eq!(f.view.split_picker(&f.lo, worn), None);
    assert_eq!(
        f.view.merge_command(&f.lo, worn, None),
        Err(InventoryError::Equipped(worn))
    );
    assert_eq!(
        f.view.split_command(&f.lo, worn, 1),
        Err(InventoryError::Equipped(worn))
    );
    assert_eq!(f.view.actions(&f.lo, rings), crate::StackActions::default());

    // Events for another holder are ignored.
    let other = Holder::Player(PlayerId::new(1).unwrap());
    assert!(!f.view.on_event(&sf_rpg::RpgEvent::ItemsChanged {
        holder: other,
        item: id("x"),
        delta: 1
    }));
}

/// A large inventory: 5,000 stacks over 8 categories.
fn hoard(n: u32) -> Loadout {
    let mut lo = Loadout::new();
    for i in 0..n {
        let item = id(&format!("junk_{:04}", i % 997));
        let attrs = Attrs::new().with("condition", AttrValue::Int(i64::from(i)));
        lo.add(item, 1 + i % 7, attrs).unwrap();
    }
    lo
}

struct HoardCatalog;

impl ItemCatalog for HoardCatalog {
    fn describe(&self, stack: &ItemStack) -> ItemInfo {
        let Some(AttrValue::Int(c)) = stack.attrs.get(&id("condition")) else {
            unreachable!()
        };
        let n = u32::try_from(*c).unwrap_or(0);
        ItemInfo {
            name: format!("{} mk {}", stack.item, n % 13),
            category: id(&format!("cat{}", n % 8)),
            #[allow(clippy::cast_precision_loss)]
            weight: (n % 50) as f32 * 0.5,
            value: i64::from(n % 300),
            icon: None,
            stats: vec![(id("damage"), f64::from(n % 40))],
        }
    }

    fn stats(&self) -> &[StatDef] {
        &[]
    }
}

fn hoard_view(n: u32) -> (Loadout, InventoryView) {
    let lo = hoard(n);
    let tabs = (0..8)
        .map(|c| InventoryTab::categories(&format!("t{c}"), "tab", [format!("cat{c}")]))
        .chain([InventoryTab::all()])
        .collect();
    let mut view = InventoryView::new(PLAYER, tabs, 24.0, 960.0);
    view.set_tab(8);
    view.sync(&lo, &SlotTable::default(), &HoardCatalog);
    (lo, view)
}

#[test]
fn thousands_of_stacks_stay_virtual() {
    let (mut lo, mut view) = hoard_view(5000);
    assert_eq!(view.rows().len(), 5000);
    let frame = view.frame();
    // 40 rows visible + 2 overscan below at the top.
    assert_eq!((frame.len, frame.rows.len()), (5000, 42));
    view.move_focus(1);
    view.page(10);
    let focus = view.focus().unwrap();
    let frame = view.frame();
    assert_eq!(frame.focus.unwrap().0, 390);
    assert!(frame.rows.len() <= 45);
    assert!(frame.rows.iter().any(|r| r.key == focus));

    // Sorting keeps the focused stack in view.
    view.sort_by_column(&column("value", SortKind::Number));
    let frame = view.frame();
    assert_eq!(frame.focus.unwrap().1, focus);
    assert!(frame.rows.iter().any(|r| r.key == focus && r.offset >= 0.0));

    // A tab and a search narrow the view; the pool doesn't grow.
    let pool = view.nav().pool_len();
    view.set_tab(3);
    view.set_search("mk 7");
    let frame = view.frame();
    assert!(frame.len > 0 && frame.len < 5000);
    let visible = view.visible().to_vec();
    assert!(visible.iter().all(|s| {
        let r = view.row(*s).unwrap();
        r.info.category == id("cat3") && r.info.name.contains("mk") && r.info.name.contains('7')
    }));
    assert_eq!(view.nav().pool_len(), pool);

    // Taking one stack removes one row; nothing else is described again.
    let first = lo.inventory().stacks()[0].id;
    lo.take(first, 1).unwrap();
    let r = view.sync(&lo, &SlotTable::default(), &HoardCatalog);
    assert_eq!((r.described, r.removed), (0, 1));
}

/// Timing on a large inventory: `cargo test -p sf_ui_inventory --release -- --ignored --nocapture`.
#[test]
#[ignore = "timing report, run on demand"]
fn timing_large_inventory() {
    for n in [1000, 5000, 20_000] {
        let lo = hoard(n);
        let mut view = InventoryView::new(PLAYER, vec![], 24.0, 960.0);
        let t = Instant::now();
        view.sync(&lo, &SlotTable::default(), &HoardCatalog);
        let first_sync = t.elapsed();
        let t = Instant::now();
        view.sync(&lo, &SlotTable::default(), &HoardCatalog);
        let resync = t.elapsed();
        let t = Instant::now();
        view.set_sort(Some(InventorySort {
            field: "value".into(),
            descending: true,
        }));
        view.frame();
        let sort = t.elapsed();
        let mut lo = lo;
        let first = lo.inventory().stacks()[0].id;
        lo.split(first, 1).ok();
        lo.add(
            id("junk_0001"),
            1,
            Attrs::new().with("condition", AttrValue::Int(-1)),
        )
        .unwrap();
        let t = Instant::now();
        view.sync(&lo, &SlotTable::default(), &HoardCatalog);
        view.frame();
        let one_change = t.elapsed();
        let t = Instant::now();
        view.set_search("mk 7");
        view.frame();
        let search = t.elapsed();
        view.set_search("");
        view.frame();
        let t = Instant::now();
        for _ in 0..100 {
            view.nav().scroll_by(24.0);
            view.frame();
        }
        let scroll = t.elapsed() / 100;
        println!(
            "{n:>6} stacks: first sync {first_sync:?}, re-sync {resync:?}, sort+frame {sort:?}, \
             one new stack sync+frame {one_change:?}, search+frame {search:?}, scroll frame {scroll:?}"
        );
    }
}
