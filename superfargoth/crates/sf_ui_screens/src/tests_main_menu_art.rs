//! Main menu entry art (look pass L2): an entry with images shows its image part instead of its
//! label and swaps it by state; an entry without shows its label; the version text slot.

use bevy_input_focus::InputFocusVisible;
use bevy_picking::hover::Hovered;
use bevy_ui::Pressed;

use super::*;

fn display(ui: &Ui, entity: Entity) -> Display {
    ui.world().get::<Node>(entity).unwrap().display
}

/// The image the entry's image part draws.
fn drawn(ui: &Ui, entry: Entity) -> Option<Handle<Image>> {
    let art = ui.world().get::<MainMenuEntryArt>(entry).unwrap();
    ui.world()
        .get::<ImageNode>(art.image())
        .map(|i| i.image.clone())
}

fn label_of(ui: &Ui, entry: Entity) -> Entity {
    ui.world().get::<Children>(entry).unwrap()[0]
}

/// Three distinct images.
fn art(ui: &mut Ui) -> MainMenuImages {
    let mut images = ui.world_mut().resource_mut::<Assets<Image>>();
    let [normal, over, pressed] = [(); 3].map(|()| images.add(Image::default()));
    MainMenuImages::new(normal, over, pressed)
}

/// The demo menu with art on New game (the first enabled entry, so focused) and Options.
fn open_with_art() -> (Ui, Entity, MainMenuImages) {
    let (mut ui, root) = open_main_menu(demo::main_menu());
    let images = art(&mut ui);
    {
        let mut screen = ui.world_mut().get_mut::<MainMenuScreen>(root).unwrap();
        for entry in &mut screen.model.entries {
            if entry.id == "new" || entry.id == "options" {
                entry.images = Some(images.clone());
            }
        }
    }
    ui.frames(2);
    (ui, root, images)
}

#[test]
fn an_entry_with_art_shows_its_image_instead_of_its_label() {
    let (ui, root, images) = open_with_art();
    let parts = main_menu_parts(&ui, root);
    for id in ["new", "options"] {
        let entry = parts.entry(id).unwrap();
        let image = ui.world().get::<MainMenuEntryArt>(entry).unwrap().image();
        assert_eq!(display(&ui, image), Display::Flex, "{id}: image shown");
        assert_eq!(display(&ui, label_of(&ui, entry)), Display::None, "{id}: label hidden");
        assert!(ui.class_of(entry).split_whitespace().any(|c| c == "image"));
        assert_eq!(ui.text_of(entry), ui.text_of(label_of(&ui, entry)), "the label stays (fallback)");
    }
    let options = parts.entry("options").unwrap();
    assert_eq!(drawn(&ui, options), Some(images.normal.clone()), "not focused: normal");
    // An entry without art: its label, no image.
    let load = parts.entry("load").unwrap();
    let image = ui.world().get::<MainMenuEntryArt>(load).unwrap().image();
    assert_eq!(display(&ui, image), Display::None);
    assert_eq!(display(&ui, label_of(&ui, load)), Display::Flex);
    assert_eq!(ui.text_of(load), "Load game");
    assert!(!ui.class_of(load).split_whitespace().any(|c| c == "image"));
    assert!(ui.world().get::<Hovered>(load).is_some(), "every entry tracks hover");
}

#[test]
fn art_follows_visible_focus_and_the_press() {
    let (mut ui, root, images) = open_with_art();
    let new = main_menu_parts(&ui, root).entry("new").unwrap();
    assert_eq!(ui.focus(), Some(new));
    ui.world_mut().insert_resource(InputFocusVisible(false));
    ui.frames(1);
    assert_eq!(drawn(&ui, new), Some(images.normal.clone()), "focus not visible (mouse): normal");
    ui.world_mut().insert_resource(InputFocusVisible(true));
    ui.frames(1);
    assert_eq!(drawn(&ui, new), Some(images.over.clone()), "pad or key focus: over");
    ui.world_mut().entity_mut(new).insert(Pressed);
    ui.frames(1);
    assert_eq!(drawn(&ui, new), Some(images.pressed.clone()), "held: pressed");
    ui.world_mut().entity_mut(new).remove::<Pressed>();
    ui.world_mut().insert_resource(InputFocusVisible(false));
    ui.frames(1);
    assert_eq!(drawn(&ui, new), Some(images.normal.clone()));

    // Art taken away by the app: the label again, same entity.
    ui.world_mut()
        .get_mut::<MainMenuScreen>(root)
        .unwrap()
        .model
        .entries[1]
        .images = None;
    ui.frames(1);
    assert_eq!(main_menu_parts(&ui, root).entry("new"), Some(new));
    assert_eq!(display(&ui, label_of(&ui, new)), Display::Flex);
    let image = ui.world().get::<MainMenuEntryArt>(new).unwrap().image();
    assert_eq!(display(&ui, image), Display::None);
}

#[test]
fn hover_picks_over_and_the_press_wins() {
    let images = MainMenuImages::new(
        Handle::default(),
        Handle::Uuid(bevy::asset::uuid::Uuid::from_u128(1), default()),
        Handle::Uuid(bevy::asset::uuid::Uuid::from_u128(2), default()),
    );
    assert_eq!(images.pick(false, false), &images.normal);
    assert_eq!(images.pick(true, false), &images.over, "hovered or visibly focused");
    assert_eq!(images.pick(true, true), &images.pressed);
    assert_eq!(images.pick(false, true), &images.pressed);
}

#[test]
fn the_version_text_shows_when_set() {
    let (mut ui, root) = open_main_menu(demo::main_menu());
    let version = main_menu_parts(&ui, root).version();
    assert_eq!(display(&ui, version), Display::None, "empty: hidden");
    ui.world_mut()
        .get_mut::<MainMenuScreen>(root)
        .unwrap()
        .model
        .version = "Demo 1.2.3".into();
    ui.frames(1);
    assert_eq!(display(&ui, version), Display::Flex);
    assert_eq!(ui.text_of(version), "Demo 1.2.3");
    ui.world_mut()
        .get_mut::<MainMenuScreen>(root)
        .unwrap()
        .model
        .version
        .clear();
    ui.frames(1);
    assert_eq!(display(&ui, version), Display::None);
}
