//! The row view-model (`inventory.item`).

use sf_rpg::{ItemStack, RecordId, StackId};
use sf_ui::{UiValue, ViewModel};

use crate::catalog::ItemInfo;

/// One inventory row: a stack and what the adapter said about it.
#[derive(Clone, Debug, PartialEq)]
pub struct ItemRow {
    /// The stack as last synced (id, item, count, instance data).
    pub stack: ItemStack,
    pub info: ItemInfo,
    /// Slot names it is equipped in (table order); empty when not equipped.
    pub slots: Vec<RecordId>,
    /// Weight it adds to the carried total ([`ItemCatalog::carried_weight`](crate::ItemCatalog::carried_weight)).
    pub carried: f64,
}

impl ItemRow {
    pub fn id(&self) -> StackId {
        self.stack.id
    }

    pub fn is_equipped(&self) -> bool {
        !self.slots.is_empty()
    }

    pub fn total_value(&self) -> i64 {
        self.info.value.saturating_mul(i64::from(self.stack.count))
    }
}

/// Field names a column or binding can use; `stat.<id>` reads [`ItemInfo::stats`].
pub const ROW_FIELDS: &[&str] = &[
    "name",
    "category",
    "count",
    "weight",
    "total_weight",
    "value",
    "total_value",
    "value_per_weight",
    "equipped",
    "slots",
    "icon",
];

/// Prefix of stat fields (`stat.armor_rating`).
pub const STAT_PREFIX: &str = "stat.";

impl ViewModel for ItemRow {
    const KIND: &'static str = "inventory.item";

    fn fields() -> &'static [&'static str] {
        ROW_FIELDS
    }

    #[allow(clippy::cast_precision_loss)] // display values; i64 beyond 2^53 is not a price
    fn field(&self, name: &str) -> Option<UiValue> {
        let info = &self.info;
        Some(match name {
            "name" => info.name.as_str().into(),
            "category" => info.category.as_str().into(),
            "count" => self.stack.count.into(),
            "weight" => info.weight.into(),
            "total_weight" => (f64::from(info.weight) * f64::from(self.stack.count)).into(),
            "value" => info.value.into(),
            "total_value" => self.total_value().into(),
            // Weightless items have no ratio (sorted last, like any missing value).
            "value_per_weight" => (info.weight > 0.0)
                .then(|| info.value as f64 / f64::from(info.weight))
                .into(),
            "equipped" => self.is_equipped().into(),
            "slots" => self
                .slots
                .iter()
                .map(RecordId::as_str)
                .collect::<Vec<_>>()
                .join(", ")
                .into(),
            "icon" => info.icon.as_deref().into(),
            _ => return name.strip_prefix(STAT_PREFIX).map(|s| info.stat(s).into()),
        })
    }
}
