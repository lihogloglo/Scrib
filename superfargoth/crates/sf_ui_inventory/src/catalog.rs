//! What the content pack tells the inventory screen about items (adapter hooks).

use sf_rpg::{ItemStack, RecordId};

/// Display data of one stack, computed by the adapter from its records and the stack's
/// instance data (condition, charge, enchantment...).
#[derive(Clone, Debug, PartialEq)]
pub struct ItemInfo {
    /// Display name, already localised by the content pack.
    pub name: String,
    /// The game's item category (`weapon`, `armor`, `potion`): tabs filter on it.
    pub category: RecordId,
    /// Weight of one piece, in the game's units.
    pub weight: f32,
    /// Value of one piece, in the game's currency.
    pub value: i64,
    /// Icon path in the VFS.
    pub icon: Option<String>,
    /// Comparable stats of one piece (damage, armour rating, condition), keyed by
    /// [`StatDef::id`]; columns read them as `stat.<id>`.
    pub stats: Vec<(RecordId, f64)>,
}

impl ItemInfo {
    pub fn stat(&self, id: &str) -> Option<f64> {
        self.stats
            .iter()
            .find(|(k, _)| k.as_str().eq_ignore_ascii_case(id))
            .map(|(_, v)| *v)
    }
}

/// Which direction of a stat is an improvement (the colour of a compare delta).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Better {
    #[default]
    Higher,
    Lower,
    /// Neither (a speed, a reach): show the change without judging it.
    Neither,
}

/// A stat the compare panel shows, in display order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatDef {
    pub id: RecordId,
    /// Fluent key of the label.
    pub label: String,
    pub better: Better,
}

/// Adapter hook: item display data and the game's carry rule.
pub trait ItemCatalog {
    /// Display data of a stack. Called only for new or changed stacks.
    fn describe(&self, stack: &ItemStack) -> ItemInfo;

    /// Stats the compare panel shows, in display order.
    fn stats(&self) -> &[StatDef];

    /// Weight a stack adds to the carried total. Default: every piece counts, equipped or
    /// not (Bethesda's rule); a game where worn armour weighs less overrides this.
    fn carried_weight(&self, stack: &ItemStack, info: &ItemInfo, equipped: bool) -> f64 {
        let _ = equipped;
        f64::from(info.weight) * f64::from(stack.count)
    }
}

/// A category tab (the category bar of `SkyUI`; LB/RB tabs in the controller preset).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InventoryTab {
    pub id: RecordId,
    /// Fluent key of the label.
    pub label: String,
    /// Categories the tab shows; empty shows every category.
    pub categories: Vec<RecordId>,
    /// Only equipped stacks.
    pub equipped_only: bool,
}

impl InventoryTab {
    /// The "all items" tab.
    pub fn all() -> Self {
        Self {
            id: RecordId::new("all"),
            label: "inventory-tab-all".into(),
            categories: Vec::new(),
            equipped_only: false,
        }
    }

    /// A tab over some categories.
    pub fn categories(
        id: &str,
        label: &str,
        categories: impl IntoIterator<Item = impl Into<RecordId>>,
    ) -> Self {
        Self {
            id: RecordId::new(id),
            label: label.into(),
            categories: categories.into_iter().map(Into::into).collect(),
            equipped_only: false,
        }
    }

    /// The "equipped" tab.
    pub fn equipped() -> Self {
        Self {
            id: RecordId::new("equipped"),
            label: "inventory-tab-equipped".into(),
            categories: Vec::new(),
            equipped_only: true,
        }
    }
}
