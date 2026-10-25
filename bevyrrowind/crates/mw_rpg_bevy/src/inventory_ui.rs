//! The player's inventory screen (ROADMAP 2.2 U5; superfargoth `ui.md` U5): the stock
//! `sf_ui_screens` inventory (icon grid, tabs, tooltips) over the player's `sf_rpg` loadout,
//! the one `inventory.rs` fills from taken items and containers.
//!
//! - **Open / close**: the rebindable `inventory` action (`controls.rs`: I, gamepad North / Y)
//!   opens it when no other menu is open and closes it when it is; Esc / B close it through the
//!   screen stack (the screen handles Back). It is a Menu-layer screen, so `dialogue.rs` pauses
//!   the player's controls while it is open (as for the journal), and E activates nothing.
//! - **Content** ([`ItemCatalog`] for [`ItemRecords`]): display names, weight, value and icon
//!   from the item records (`OpenMW` reads the same fields: `MWClass::*::getName`, `getInventoryIcon`,
//!   apps/openmw/mwclass, 0.49.0); icons are the BSA's `icons/<path>.dds` (the record names a
//!   `.tga`, the shipped file is a DDS, as `Misc::ResourceHelpers::correctIconPath` maps it).
//!   Tabs are Morrowind's (All, Weapon, Apparel, Magic, Misc) with the GMST labels.
//! - Read-only for now: the screen's use / split / merge intents are ignored. Not yet: equip,
//!   drop, weight limit (no capacity: the encumbrance bar shows the carried total only), the
//!   container window, item tooltips' stats.

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use bevy_ui::prelude::*;
use bevy_enhanced_input::prelude::{Action, ActionEvents};
use sf_records::{RecordDb, RecordId};
use sf_rpg::{Attrs, CharacterRpg, Holder, ItemStack, PlayerId, RpgCommand, SlotTable};
use sf_ui::{Layer, UiRegistry};
use sf_ui_bevy::{ScreenStackRes, UiRegistryRes, UiScreenRoot};
use sf_ui_inventory::{InventoryTab, InventoryView, ItemCatalog, ItemInfo, StatDef};
use sf_ui_screens::{InventoryScreen, ScreenSystems};
use tes3::esp::{
    Alchemy, Apparatus, Armor, Book, Clothing, GameSetting, GameSettingValue, Ingredient, Light, Lockpick, MiscItem, Probe,
    RepairItem, Weapon,
};

use mw_controls_bevy::{InventoryAction, inventory_pressed};
use crate::dialogue::Rpg;
use crate::inventory::ItemRecords;

/// The screen's id.
pub const SCREEN: &str = "inventory";

/// The inventory screen record. TODO(HUD slice): with the other screens, in the builtin mod's data.
const SCREENS: &str = r#"
[[define]]
kind = "ui.screen"
id = "inventory"
value = { layer = "Menu", implementation = { Template = "inventory" } }
"#;

/// The player's equipment slots. Empty until equip lands (2.2 follow-up): nothing shows as worn.
#[derive(Resource)]
pub struct PcSlots(pub SlotTable);

impl Default for PcSlots {
    fn default() -> Self {
        Self(SlotTable::new(Vec::<RecordId>::new()).expect("an empty slot table is valid"))
    }
}

/// Dev scaffold `--open-ui inventory`: opens the screen once at start with a few sample items
/// (`DevSampleItems`), so a `--screenshot` run shows it filled. Remove with `--open-ui`.
#[derive(Resource)]
struct OpenAtStart;

/// Sample items of the dev open (Fargoth's ring, a bottle, a dagger, a potion, a shirt).
const DEV_SAMPLE_ITEMS: [(&str, u32); 5] = [
    ("ring_keley", 1),
    ("misc_com_bottle_01", 3),
    ("iron dagger", 1),
    ("p_restore_health_s", 2),
    ("common_shirt_01", 1),
];

/// After `inventory::add` (the item records) and `controls::add` (the inventory action).
/// `open_at_start`: the dev `--open-ui inventory` flag.
pub fn add(app: &mut App, open_at_start: bool) {
    if !app.world().contains_resource::<ItemRecords>() || !app.world().contains_resource::<Rpg>() {
        return;
    }
    let Some(mut registry) = app.world_mut().get_resource_mut::<UiRegistryRes>() else { return };
    registry.0.extend_screens(&screens());
    if open_at_start {
        app.insert_resource(OpenAtStart);
    }
    app.init_resource::<PcSlots>().add_systems(
        Update,
        ((open_at_start_once, toggle_inventory).chain().before(ScreenSystems::Spawn), (centre_inventory.after(ScreenSystems::Spawn), sync_open, close_inventory)),
    );
}

fn screens() -> UiRegistry {
    let mut db = RecordDb::new();
    sf_ui::register_kinds(&mut db).expect("UI record kinds register");
    let base = db.add_source("bevyrrowind", &[]);
    let outcome = sf_records::apply_record_file(&mut db, base, SCREENS, sf_records::RecordFormat::Toml).expect("inventory screen");
    assert!(outcome.errors.is_empty(), "inventory screen: {:?}", outcome.errors);
    UiRegistry::build(&db).0
}

// ============================================================ the catalog

/// Item display data from the records: Morrowind's item classes share name, icon, weight, value.
impl ItemCatalog for ItemRecords {
    fn describe(&self, stack: &ItemStack) -> ItemInfo {
        item_info(&self.0, &stack.item)
    }

    fn stats(&self) -> &[StatDef] {
        &[]
    }
}

/// [`ItemInfo`] of `item`: its record's name (the id when it has none), category (a tab of
/// [`tabs`]), weight, value and icon.
pub fn item_info(db: &RecordDb, item: &RecordId) -> ItemInfo {
    macro_rules! find {
        ($($ty:ty => $category:literal),* $(,)?) => {
            $(if let Some(r) = db.get::<$ty>(item) {
                return info(item, &r.name, &r.icon, $category, r.data.weight, i64::from(r.data.value));
            })*
        };
    }
    find!(
        Weapon => "weapon", Armor => "armor", Clothing => "clothing", Alchemy => "potion", Ingredient => "ingredient",
        Book => "book", MiscItem => "misc", Apparatus => "apparatus", Lockpick => "lockpick", Probe => "probe",
        RepairItem => "repair", Light => "light",
    );
    info(item, "", "", "misc", 0.0, 0)
}

fn info(item: &RecordId, name: &str, icon: &str, category: &str, weight: f32, value: i64) -> ItemInfo {
    let name = name.trim();
    ItemInfo {
        name: if name.is_empty() { item.as_str().to_owned() } else { name.to_owned() },
        category: RecordId::new(category),
        weight,
        value,
        icon: icon_path(icon),
        stats: Vec::new(),
    }
}

/// The VFS image of a record's icon field (`m\misc_com_bottle_01.tga` -> the BSA's
/// `icons/m/misc_com_bottle_01.dds`), through the UI content asset source.
pub fn icon_path(icon: &str) -> Option<String> {
    let icon = icon.trim().trim_matches('\0');
    if icon.is_empty() {
        return None;
    }
    let path = icon.replace('\\', "/").to_ascii_lowercase();
    let stem = path.rsplit_once('.').map_or(path.as_str(), |(stem, _)| stem);
    Some(format!("sf_ui_content://icons/{stem}.dds"))
}

/// Morrowind's inventory filters (`MWGui::ItemModel` categories: All, Weapon, Apparel, Magic,
/// Misc), labelled by their GMSTs. Magic here is potions only (enchanted items: with magic).
pub fn tabs(db: &RecordDb) -> Vec<InventoryTab> {
    let label = |gmst: &str, fallback: &str| match db.get::<GameSetting>(&RecordId::new(gmst)).map(|g| &g.value) {
        Some(GameSettingValue::String(s)) if !s.trim().is_empty() => s.trim().to_owned(),
        _ => fallback.to_owned(),
    };
    let mut all = InventoryTab::all();
    all.label = label("sAllTab", "All");
    vec![
        all,
        InventoryTab::categories("weapon", &label("sWeaponTab", "Weapon"), ["weapon"]),
        InventoryTab::categories("apparel", &label("sApparelTab", "Apparel"), ["armor", "clothing"]),
        InventoryTab::categories("magic", &label("sMagicTab", "Magic"), ["potion"]),
        InventoryTab::categories(
            "misc",
            &label("sMiscTab", "Misc"),
            ["ingredient", "book", "misc", "apparatus", "lockpick", "probe", "repair", "light"],
        ),
    ]
}

/// The player's inventory view, synced with `pc`'s loadout.
pub fn player_view(records: &ItemRecords, slots: &SlotTable, pc: &CharacterRpg) -> InventoryView {
    let mut view = InventoryView::new(Holder::Player(PlayerId::HOST), tabs(&records.0), 1.0, 1.0);
    view.sync(&pc.loadout, slots, records);
    view
}

// ============================================================ the app glue

fn open(commands: &mut Commands, stack: &mut ScreenStackRes, registry: &UiRegistryRes, view: InventoryView) {
    commands.spawn((UiScreenRoot(SCREEN.into()), InventoryScreen::new(view)));
    if let Err(e) = stack.0.push(&registry.0, SCREEN) {
        warn!("inventory: {e:?}");
    }
}

/// The inventory action opens the screen when no other menu is open and closes it when it is.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn toggle_inventory(
    mut commands: Commands,
    actions: Query<&ActionEvents, With<Action<InventoryAction>>>,
    mut stack: ResMut<ScreenStackRes>,
    registry: Res<UiRegistryRes>,
    records: Res<ItemRecords>,
    slots: Res<PcSlots>,
    rpg: Res<Rpg>,
) {
    if !inventory_pressed(&actions) {
        return;
    }
    if stack.0.is_open(SCREEN) {
        stack.0.close(SCREEN);
    } else if stack.0.top(Layer::Menu).is_none() {
        open(&mut commands, &mut stack, &registry, player_view(&records, &slots.0, &rpg.state.pc));
    }
}

/// Dev `--open-ui inventory` (scaffold [`OpenAtStart`]).
#[allow(clippy::needless_pass_by_value)] // system parameters
fn open_at_start_once(
    mut commands: Commands,
    flag: Option<Res<OpenAtStart>>,
    mut stack: ResMut<ScreenStackRes>,
    registry: Res<UiRegistryRes>,
    records: Res<ItemRecords>,
    slots: Res<PcSlots>,
    mut rpg: ResMut<Rpg>,
) {
    if flag.is_none() {
        return;
    }
    commands.remove_resource::<OpenAtStart>();
    for (item, count) in DEV_SAMPLE_ITEMS {
        let holder = Holder::Player(PlayerId::HOST);
        let _ = rpg.state.pc.apply(RpgCommand::AddItem { holder, item: RecordId::new(item), count, attrs: Attrs::new() });
    }
    open(&mut commands, &mut stack, &registry, player_view(&records, &slots.0, &rpg.state.pc));
}

/// An open screen follows the loadout (a dialogue result, a take): unchanged stacks keep their rows.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn sync_open(rpg: Res<Rpg>, records: Res<ItemRecords>, slots: Res<PcSlots>, mut screens: Query<&mut InventoryScreen>) {
    if !rpg.is_changed() {
        return;
    }
    for mut screen in &mut screens {
        screen.view.sync(&rpg.state.pc.loadout, &slots.0, &*records);
    }
}

/// Centres the window once the stock screen gave its root the panel `Node` (it replaces the
/// spawn's): absolute at the middle, offset by half the panel's fixed size.
fn centre_inventory(mut roots: Query<&mut Node, (With<InventoryScreen>, Changed<Node>)>) {
    for mut node in &mut roots {
        if node.position_type == PositionType::Absolute {
            continue;
        }
        let half = |v: Val| match v {
            Val::Px(px) => Val::Px(-px / 2.0),
            _ => Val::Px(0.0),
        };
        node.margin = UiRect { left: half(node.width), top: half(node.height), ..Default::default() };
        node.position_type = PositionType::Absolute;
        node.left = percent(50);
        node.top = percent(50);
    }
}

/// The screen's root despawns once it is off the stack (the action, Esc / Back).
#[allow(clippy::needless_pass_by_value)] // system parameters
fn close_inventory(mut commands: Commands, stack: Res<ScreenStackRes>, roots: Query<(Entity, &UiScreenRoot)>) {
    for (e, root) in &roots {
        if root.0.as_str() == SCREEN && !stack.0.is_open(SCREEN) {
            commands.entity(e).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    //! Real data (skips without a Morrowind install): taking the Census office barrel puts
    //! Fargoth's ring in the inventory screen's view-model under its display name and icon.
    use std::sync::Arc;

    use mw_install::MorrowindInstall;
    use sf_rpg::ActorKey;

    use super::*;
    use crate::inventory::take_all;

    #[test]
    fn the_ring_barrel_lists_fargoths_ring_in_the_inventory_screen() {
        let Ok((install, _)) = MorrowindInstall::detect(None) else {
            eprintln!("no Morrowind install detected; skipping");
            return;
        };
        let vfs = mw_vfs::mount_install(&install, sf_vfs::Vfs::builder()).unwrap().vfs;
        let db = Arc::new(mw_records::load(&vfs, &install.game_files()).unwrap().db);
        let records = ItemRecords(db.clone());
        let slots = PcSlots::default();
        let mut rpg = Rpg::new(db.clone()).unwrap();
        assert!(player_view(&records, &slots.0, &rpg.state.pc).rows().is_empty(), "nothing taken yet");

        let barrel = RecordId::new("chargen barrel fatigue");
        take_all(&mut rpg.state, &db, PlayerId::HOST, ActorKey(7), &barrel);
        let mut view = player_view(&records, &slots.0, &rpg.state.pc);
        let visible = view.visible().to_vec();
        let ring = visible.iter().filter_map(|id| view.rows().get(id)).find(|r| r.stack.item.as_str() == "ring_keley");
        let ring = ring.expect("Fargoth's ring is listed in the All tab");
        assert_eq!(ring.info.name, "Engraved Ring of Healing", "the record name, not the id");
        assert_eq!(ring.stack.count, 1);
        assert_eq!(ring.info.category.as_str(), "clothing");
        let icon = ring.info.icon.as_deref().unwrap_or_default();
        let dds = std::path::Path::new(icon).extension().is_some_and(|e| e == "dds");
        assert!(icon.starts_with("sf_ui_content://icons/") && dds, "{icon}");
        assert_eq!(view.tabs().len(), 5);
        assert_eq!(icon_path(r"m\Misc_Com_Bottle_01.tga").as_deref(), Some("sf_ui_content://icons/m/misc_com_bottle_01.dds"));
    }
}
