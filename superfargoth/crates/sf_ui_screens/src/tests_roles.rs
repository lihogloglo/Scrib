//! Role classes (ui.md "Role classes"): each stock screen's main panel is `sf-window`, its inner
//! framed areas `sf-box`, its framed buttons `sf-framed`; bare text buttons carry no role.

use super::*;
use crate::widgets::role;

/// The class lists of every part named `name`.
fn classes_of(ui: &mut Ui, name: &str) -> Vec<String> {
    let mut parts = ui
        .world_mut()
        .query::<(&sf_ui_bevy::UiPart, &sf_ui_css::UiClass)>();
    parts
        .iter(ui.world())
        .filter(|(part, _)| part.0 == name)
        .map(|(_, class)| class.0.clone())
        .collect()
}

/// Every part named `name` exists, keeps its name as first class and has `role`.
fn assert_role(ui: &mut Ui, name: &str, role: &str) {
    let all = classes_of(ui, name);
    assert!(!all.is_empty(), "no part {name}");
    for class in all {
        let mut list = class.split_whitespace();
        assert_eq!(list.next(), Some(name), "{name}: {class}");
        assert!(list.any(|c| c == role), "{name} lacks {role}: {class}");
    }
}

/// Every part named `name` exists and has no role class.
fn assert_bare(ui: &mut Ui, name: &str) {
    let all = classes_of(ui, name);
    assert!(!all.is_empty(), "no part {name}");
    for class in all {
        for role in [role::WINDOW, role::BOX, role::FRAMED] {
            assert!(!class.split_whitespace().any(|c| c == role), "{name} has {role}: {class}");
        }
    }
}

#[test]
fn roles_options() {
    let (mut ui, _) = open_options();
    assert_role(&mut ui, "sf-options", role::WINDOW);
    assert_role(&mut ui, "sf-options-rows", role::BOX);
    assert_role(&mut ui, "sf-options-pane", role::BOX);
    assert_role(&mut ui, "sf-options-back", role::FRAMED);
    assert_role(&mut ui, "sf-options-keybinds", role::FRAMED);
    assert_bare(&mut ui, "sf-options-tab");
    assert_bare(&mut ui, "sf-options-row");
}

#[test]
fn roles_keybinds() {
    let mut ui = Ui::new();
    open_keybinds(&mut ui);
    assert_role(&mut ui, "sf-keybinds", role::WINDOW);
    assert_role(&mut ui, "sf-keybinds-rows", role::BOX);
    assert_role(&mut ui, "sf-keybinds-overlay", role::WINDOW);
    assert_role(&mut ui, "sf-keybinds-reset-all", role::FRAMED);
    assert_role(&mut ui, "sf-keybinds-choice", role::FRAMED);
    assert_bare(&mut ui, "sf-keybinds-slot");
}

#[test]
fn roles_pause_menu_and_its_question() {
    let (mut ui, _) = open_pause(PauseMenuModel::default());
    assert_role(&mut ui, "sf-pause", role::WINDOW);
    assert_role(&mut ui, "sf-pause-confirm", role::WINDOW);
    assert_role(&mut ui, "sf-pause-yes", role::FRAMED);
    assert_role(&mut ui, "sf-pause-no", role::FRAMED);
    // The entries become the main menu's image buttons (look pass L2/L3), not framed.
    assert_bare(&mut ui, "sf-pause-entry");
}

#[test]
fn roles_saveload() {
    let mut ui = Ui::new();
    open_saveload(&mut ui, saves(SaveLoadMode::Load));
    assert_role(&mut ui, "sf-saveload", role::WINDOW);
    assert_role(&mut ui, "sf-saveload-list", role::BOX);
    assert_role(&mut ui, "sf-saveload-primary", role::FRAMED);
    assert_role(&mut ui, "sf-saveload-delete", role::FRAMED);
    assert_role(&mut ui, "sf-saveload-back", role::FRAMED);
    assert_role(&mut ui, "sf-saveload-confirm", role::WINDOW);
    assert_bare(&mut ui, "sf-saveload-row");
}

#[test]
fn roles_loading() {
    let mut ui = Ui::new();
    // The app shows the loading screen outside the screen stack: spawn the root alone.
    ui.world_mut().spawn((
        UiScreenRoot("loading".into()),
        LoadingScreen::new(LoadingModel::default()),
    ));
    ui.frames(2);
    assert_role(&mut ui, "sf-loading-footer", role::WINDOW);
    assert_role(&mut ui, "sf-loading-bar", role::BOX);
    assert_bare(&mut ui, "sf-loading");
}

#[test]
fn roles_dialogue_survive_the_mode_class() {
    let mut ui = Ui::new();
    open_dialogue(&mut ui);
    ui.frames(3);
    // The sync rewrites the root's and Goodbye's classes (mode, disabled): the role stays.
    assert_role(&mut ui, "sf-dialogue", role::WINDOW);
    assert_role(&mut ui, "sf-dialogue-log", role::BOX);
    assert_role(&mut ui, "sf-dialogue-side", role::BOX);
    assert_role(&mut ui, "sf-dialogue-goodbye", role::FRAMED);
    assert_bare(&mut ui, "sf-dialogue-topic");
}

#[test]
fn roles_journal() {
    let mut ui = Ui::new();
    open_journal(&mut ui);
    assert_role(&mut ui, "sf-journal-index", role::WINDOW);
    // The book is drawn by its own art, not a frame.
    assert_bare(&mut ui, "sf-journal");
    assert_bare(&mut ui, "sf-journal-book");
}

#[test]
fn roles_inventory() {
    let mut ui = Ui::new();
    open_inventory(&mut ui, 6);
    assert_role(&mut ui, "sf-inventory", role::WINDOW);
    assert_role(&mut ui, "sf-inventory-grid", role::BOX);
    assert_role(&mut ui, "sf-inventory-split", role::WINDOW);
    assert_role(&mut ui, "sf-inventory-split-ok", role::FRAMED);
    assert_role(&mut ui, "sf-inventory-split-cancel", role::FRAMED);
    assert_bare(&mut ui, "sf-inventory-split-less");
    assert_role(&mut ui, "sf-tooltip", role::BOX);
    assert_bare(&mut ui, "sf-inventory-tab");
}

#[test]
fn roles_container() {
    let (mut ui, _) = open_container();
    assert_role(&mut ui, "sf-container", role::WINDOW);
    assert_role(&mut ui, "sf-inventory-grid", role::BOX);
    assert_role(&mut ui, "sf-container-take-all", role::FRAMED);
}
