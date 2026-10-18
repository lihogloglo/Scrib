//! String identifiers. Record-backed ids equal the record id (`sf_records::RecordId`).

use std::borrow::Borrow;
use std::fmt;

use serde::{Deserialize, Serialize};

macro_rules! id_type {
    ($($(#[$meta:meta])* $name:ident;)*) => {$(
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub String);

        impl $name {
            pub fn new(id: impl Into<String>) -> Self {
                Self(id.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<&str> for $name {
            fn from(id: &str) -> Self {
                Self(id.to_owned())
            }
        }

        impl Borrow<str> for $name {
            fn borrow(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    )*};
}

id_type! {
    /// A `ui.screen` record id (`inventory`, `main_menu`).
    ScreenId;
    /// A named extension slot in a screen (`hud.top_right`, `item_tooltip.after_stats`).
    SlotId;
    /// A `ui.theme` record id.
    ThemeId;
    /// A `ui.preset` record id (`pc_dense`, `controller` built in).
    PresetId;
    /// An input context `sf_ui_bevy` maps to the input crate's context type.
    InputContextId;
    /// A keyed list a screen shows (`inventory.items`); `ui.column` records target it.
    ListId;
    /// A `ui.column` record id.
    ColumnId;
    /// A `ui.setting` record id.
    SettingKey;
    /// A Rust screen template registered with `sf_ui_bevy`.
    TemplateId;
    /// A mod id: the `sf_records` source name the mod pipeline gives each mod.
    ModId;
}
