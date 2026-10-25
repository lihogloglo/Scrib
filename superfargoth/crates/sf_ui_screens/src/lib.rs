//! Stock game screens (ui.md "Screens", ROADMAP 2.2 U3-U5): the Bevy side of the Bevy-free
//! view-models, built from a small widget kit with CSS class names on every part.
//!
//! - [`widgets`]: parts (`Name` + [`sf_ui_bevy::UiPart`] + [`sf_ui_css::UiClass`]), buttons,
//!   panels, value bars, fallback colours ([`ScreenColors`]); fixed strings in
//!   [`ScreenLabels`].
//! - [`keyed`]: [`KeyedChildren`], keyed reconciliation of a view-model's rows into child
//!   entities: add, remove and reorder without rebuilding the rest.
//! - [`dialogue`]: the dialogue window over `sf_rpg_ui::DialogueView`: scrollback with
//!   clickable topic links (span picking, plus the gamepad link cursor), topic list, choices,
//!   goodbye. Requests go out as [`DialogueRequestEvent`]s for the host.
//! - [`journal`]: the journal over `sf_rpg_ui::JournalScreen`: the book (pages measured on
//!   Bevy's own text layout, drawn like `sf_ui_bevy`'s book reader), quest and topic indexes.
//! - [`inventory`]: the inventory over `sf_ui_inventory`: virtual icon grid, drag and drop,
//!   the split dialog and tooltips with compare-with-equipped. Intents go out as
//!   [`InventoryEvent`]s.
//! - [`keybinds`]: the keybinds screen over `sf_ui_bevy::UiKeymap`: capture flow, conflict
//!   question, glyphs for the last-used pad.
//! - [`saveload`]: the save/load screen over an app-filled [`SaveLoadModel`]: saves newest
//!   first, new save, overwrite and delete with a confirmation. Choices go out as
//!   [`SaveLoadEvent`]s.
//! - [`pause_menu`]: the pause (escape) menu over an app-filled [`PauseMenuModel`]: resume,
//!   save, load, options, quit (asks first). Choices go out as [`PauseMenuEvent`]s.
//! - [`hud`]: value bars, the compass slot, crosshair and the "press to use" prompt.
//!
//! A game spawns a screen as a `UiScreenRoot` entity carrying the screen's component (for
//! example `DialogueScreen::new(view)`) and pushes its id on the screen stack; the router
//! gives it input while it is on top, so gameplay sees none of the inputs it binds. Screens
//! whose view-model owns focus (dialogue, inventory, keybinds, save/load, pause) carry `UiOwnsFocus` and move
//! it themselves; the journal uses Bevy's directional navigation.
//!
//! Nothing here names a game: content packs bring names, icons, themes and the host logic.

pub mod dialogue;
pub mod hud;
pub mod inventory;
pub mod journal;
pub mod keybinds;
pub mod keyed;
pub mod labels;
pub mod pause_menu;
pub mod saveload;
pub mod widgets;

#[cfg(feature = "demo")]
pub mod demo;

#[cfg(test)]
mod tests;

use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use sf_ui_bevy::{ScreenStackRes, UiScreenRoot, UiSystems, UiTextSystems};

pub use dialogue::{DialogueRequestEvent, DialogueScreen};
pub use hud::{HudBar, HudScreen, HudSlot, HudView, PromptLabels, UsePrompt};
pub use inventory::{InventoryEvent, InventoryGridStyle, InventoryIntent, InventoryScreen};
pub use journal::{JournalSeenChanged, JournalSource, JournalUi, JournalUiPlugin};
pub use keybinds::{KeybindsFocus, KeybindsScreen};
pub use keyed::{KeyedChildren, KeyedSync};
pub use labels::ScreenLabels;
pub use pause_menu::{
    PauseChoice, PauseEntry, PauseMenuEvent, PauseMenuFocus, PauseMenuModel, PauseMenuParts,
    PauseMenuScreen,
};
pub use saveload::{
    SaveEntryView, SaveLoadChoice, SaveLoadEvent, SaveLoadFocus, SaveLoadMode, SaveLoadModel,
    SaveLoadRow, SaveLoadScreen,
};
pub use widgets::{ScreenColors, StatBar};

/// Ordering of the screens' systems in `Update`, after the router's dispatch.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenSystems {
    /// Build new screens' parts.
    Spawn,
    /// UI actions, directions and pointer intents → view-models and requests.
    Input,
    /// View-models → entity trees (keyed), focus.
    Sync,
}

/// The stock screens (not the journal, which needs the game's [`JournalSource`]: add
/// [`JournalUiPlugin`] for it). Needs `SfUiPlugin` and `UiTextPlugin`.
pub struct SfScreensPlugin;

impl Plugin for SfScreensPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<bevy_ui_widgets::ScrollAreaPlugin>() {
            app.add_plugins(bevy_ui_widgets::ScrollAreaPlugin);
        }
        app.init_resource::<ScreenColors>()
            .init_resource::<ScreenLabels>()
            .init_resource::<InventoryGridStyle>()
            .add_message::<DialogueRequestEvent>()
            .add_message::<InventoryEvent>()
            .add_message::<SaveLoadEvent>()
            .add_message::<PauseMenuEvent>()
            .add_observer(dialogue::on_dialogue_button)
            .add_observer(dialogue::on_link_click)
            .add_observer(dialogue::on_dialogue_hover)
            .add_observer(keybinds::on_keybind_cell)
            .add_observer(saveload::on_saveload_part)
            .add_observer(pause_menu::on_pause_menu_part)
            .add_observer(inventory::on_inventory_button)
            .add_observer(inventory::on_cell_over)
            .add_observer(inventory::on_cell_drag_start)
            .add_observer(inventory::on_cell_drag)
            .add_observer(inventory::on_cell_drag_end)
            .add_observer(inventory::on_cell_drop)
            .add_observer(inventory::on_inventory_removed)
            .configure_sets(
                Update,
                (
                    ScreenSystems::Spawn,
                    ScreenSystems::Input,
                    ScreenSystems::Sync,
                )
                    .chain()
                    .after(UiSystems::Dispatch)
                    .before(UiTextSystems::Input),
            )
            .add_systems(
                Update,
                (
                    dialogue::spawn_dialogue,
                    hud::spawn_hud,
                    keybinds::spawn_keybinds,
                    inventory::spawn_inventory,
                    saveload::spawn_saveload,
                    pause_menu::spawn_pause_menu,
                )
                    .in_set(ScreenSystems::Spawn),
            )
            .add_systems(
                Update,
                (
                    dialogue::dialogue_input,
                    keybinds::keybinds_input,
                    inventory::inventory_input,
                    saveload::saveload_input,
                    pause_menu::pause_menu_input,
                )
                    .in_set(ScreenSystems::Input),
            )
            .add_systems(
                Update,
                (
                    dialogue::sync_dialogue,
                    dialogue::scroll_dialogue,
                    hud::sync_hud,
                    keybinds::sync_keybinds,
                    inventory::sync_inventory,
                    saveload::sync_saveload,
                    pause_menu::sync_pause_menu,
                    widgets::sync_bars,
                    widgets::focus_ring,
                    widgets::scroll_to_focus,
                )
                    .chain()
                    .in_set(ScreenSystems::Sync),
            );
    }
}

/// The nearest ancestor of `entity` (or itself) that `is_screen` accepts.
pub(crate) fn screen_of(
    entity: Entity,
    parents: &Query<&ChildOf>,
    is_screen: impl Fn(Entity) -> bool,
) -> Option<Entity> {
    let mut current = entity;
    loop {
        if is_screen(current) {
            return Some(current);
        }
        current = parents.get(current).ok()?.parent();
    }
}

/// The screen is the input target (top of the highest input-taking layer).
pub(crate) fn is_target(stack: &ScreenStackRes, root: &UiScreenRoot) -> bool {
    stack.0.input_target().is_some_and(|t| t.id == root.0)
}
