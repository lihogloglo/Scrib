//! The options footer action bar (menus.md §5): Reset page asks first, then puts the page's
//! `reset` settings back to their defaults; Back closes. Keyboard and pad reach both, and the
//! window keeps its rect throughout.

use sf_records::{RecordDb, RecordFormat, apply_record_file};
use sf_ui::{SettingValue, SettingsStore, UiRegistry, register_kinds};

use super::options_pane::window_rect;
use super::*;

/// Page `p`: two resettable settings and a language-like one that Reset page leaves alone.
const SETTINGS: &str = r#"
[[define]]
kind = "ui.setting"
id = "t.vol"
value = { label = "set-vol", page = "p", kind = { Float = { default = 0.5, min = 0.0, max = 1.0, step = 0.1 } } }

[[define]]
kind = "ui.setting"
id = "t.sub"
value = { label = "set-sub", page = "p", kind = { Bool = { default = true } } }

[[define]]
kind = "ui.setting"
id = "t.lang"
value = { label = "set-lang", page = "p", kind = { Choice = { default = "en", options = ["en", "fr"] } }, reset = false }
"#;

fn registry() -> UiRegistry {
    let mut db = RecordDb::new();
    register_kinds(&mut db).expect("UI record kinds register");
    let base = db.add_source("t", &[]);
    let outcome = apply_record_file(&mut db, base, SETTINGS, RecordFormat::Toml).expect("parse");
    assert!(outcome.errors.is_empty(), "{:?}", outcome.errors);
    let (registry, errors) = UiRegistry::build(&db);
    assert!(errors.is_empty(), "{errors:?}");
    registry
}

/// The store with all three settings of page `p` changed.
fn changed_store(registry: &UiRegistry) -> SettingsStore {
    let mut store = SettingsStore::new(registry);
    store.set("t.vol", SettingValue::Float(0.9)).unwrap();
    store.set("t.sub", SettingValue::Bool(false)).unwrap();
    store.set("t.lang", SettingValue::Text("fr".into())).unwrap();
    store
}

/// The options screen on page `p`, built from `store`; its window rect once laid out.
fn open_on_p(registry: &UiRegistry, store: Option<&SettingsStore>) -> (Ui, Entity, (Vec2, Vec2)) {
    let model = OptionsModel::from_settings(registry, store, None);
    let (mut ui, root) = open_options_with(model);
    show_options_page(&mut ui, root, "p");
    ui.frames(2);
    let rect = window_rect(&ui, root);
    (ui, root, rect)
}

fn screen(ui: &Ui, root: Entity) -> &OptionsScreen {
    ui.world().get::<OptionsScreen>(root).unwrap()
}

fn row_value(ui: &Ui, root: Entity, key: &str) -> SettingValue {
    screen(ui, root).model.row(key).unwrap().value.clone()
}

fn disabled(ui: &Ui, entity: Entity) -> bool {
    ui.world()
        .get::<bevy_ui::InteractionDisabled>(entity)
        .is_some()
}

fn overlay_shown(ui: &Ui, root: Entity) -> bool {
    let overlay = options_parts(ui, root).overlay();
    ui.world().get::<Node>(overlay).unwrap().display != Display::None
}

/// Down from the first row to the footer: past the three rows to Key bindings.
fn to_footer(ui: &mut Ui) {
    for _ in 0..3 {
        tap_key(ui, KeyCode::ArrowDown);
    }
}

#[test]
fn options_footer_is_reached_by_keyboard_and_pad_and_reset_skips_while_nothing_differs() {
    let registry = registry();
    let store = changed_store(&registry);
    let (mut ui, root, _) = open_on_p(&registry, Some(&store));
    let parts = options_parts(&ui, root);
    let (keybinds, reset, back) = (parts.keybinds(), parts.reset_page(), parts.back());
    assert_eq!(ui.text_of(reset), "Reset page");
    assert!(screen(&ui, root).can_reset());
    assert!(!disabled(&ui, reset));

    // Keyboard: rows, Key bindings, Reset page, Back (clamped), and back up.
    to_footer(&mut ui);
    assert_eq!(ui.focus(), Some(keybinds));
    tap_key(&mut ui, KeyCode::ArrowDown);
    assert_eq!(ui.focus(), Some(reset));
    tap_key(&mut ui, KeyCode::ArrowDown);
    assert_eq!(ui.focus(), Some(back));
    tap_key(&mut ui, KeyCode::ArrowDown);
    assert_eq!(ui.focus(), Some(back), "no wrap");
    tap_key(&mut ui, KeyCode::ArrowUp);
    assert_eq!(ui.focus(), Some(reset));
    // Pad: the same stops.
    ui.tap(GamepadButton::DPadDown);
    assert_eq!(ui.focus(), Some(back));
    ui.tap(GamepadButton::DPadUp);
    ui.tap(GamepadButton::DPadUp);
    assert_eq!(ui.focus(), Some(keybinds));

    // At defaults there is nothing to reset: the button is disabled and skipped, a click
    // asks nothing, and Back is next after Key bindings.
    let (mut ui, root, rect) = open_on_p(&registry, None);
    let parts = options_parts(&ui, root);
    let (keybinds, reset, back) = (parts.keybinds(), parts.reset_page(), parts.back());
    assert!(!screen(&ui, root).can_reset());
    assert!(disabled(&ui, reset));
    to_footer(&mut ui);
    assert_eq!(ui.focus(), Some(keybinds));
    tap_key(&mut ui, KeyCode::ArrowDown);
    assert_eq!(ui.focus(), Some(back), "a disabled Reset page is skipped");
    ui.world_mut().trigger(Activate { entity: reset });
    ui.frames(1);
    assert!(!screen(&ui, root).asking_reset());
    assert!(take_options(&mut ui).is_empty());
    assert_eq!(window_rect(&ui, root), rect);

    // Back from the footer closes, as Esc does.
    tap_key(&mut ui, KeyCode::Enter);
    assert_eq!(take_options(&mut ui), [OptionsChange::Back]);
    assert!(!options_open(&ui));
}

#[test]
fn options_reset_page_no_and_esc_change_nothing() {
    let registry = registry();
    let store = changed_store(&registry);
    let (mut ui, root, rect) = open_on_p(&registry, Some(&store));
    let reset = options_parts(&ui, root).reset_page();
    let [_, no] = options_parts(&ui, root).confirm_buttons();
    to_footer(&mut ui);
    tap_key(&mut ui, KeyCode::ArrowDown);
    assert_eq!(ui.focus(), Some(reset));

    // Asks, No focused; the window stays where it was.
    tap_key(&mut ui, KeyCode::Enter);
    assert!(screen(&ui, root).asking_reset());
    assert!(overlay_shown(&ui, root));
    assert!(ui.text_of(options_parts(&ui, root).overlay()).contains("defaults"));
    assert_eq!(ui.focus(), Some(no));
    assert_eq!(window_rect(&ui, root), rect, "the question moved the window");
    // Modal: up and down stay on the answers.
    tap_key(&mut ui, KeyCode::ArrowUp);
    assert_eq!(ui.focus(), Some(no));

    // No: nothing sent, nothing changed, back on Reset page.
    tap_key(&mut ui, KeyCode::Enter);
    assert!(!screen(&ui, root).asking_reset());
    assert!(!overlay_shown(&ui, root));
    assert!(take_options(&mut ui).is_empty());
    assert_eq!(ui.focus(), Some(reset));
    assert_eq!(row_value(&ui, root, "t.vol"), SettingValue::Float(0.9));
    assert_eq!(row_value(&ui, root, "t.sub"), SettingValue::Bool(false));

    // Esc answers no and keeps the screen open.
    tap_key(&mut ui, KeyCode::Enter);
    assert!(screen(&ui, root).asking_reset());
    tap_key(&mut ui, KeyCode::Escape);
    assert!(!screen(&ui, root).asking_reset());
    assert!(take_options(&mut ui).is_empty());
    assert!(options_open(&ui));
    assert_eq!(ui.focus(), Some(reset));
    assert_eq!(row_value(&ui, root, "t.vol"), SettingValue::Float(0.9));
    assert_eq!(window_rect(&ui, root), rect);
}

#[test]
fn options_reset_page_yes_resets_only_the_resettable_settings_through_the_store() {
    let registry = registry();
    let mut store = changed_store(&registry);
    let (mut ui, root, rect) = open_on_p(&registry, Some(&store));
    let parts = options_parts(&ui, root);
    let (reset, back) = (parts.reset_page(), parts.back());
    let [yes, _] = parts.confirm_buttons();
    to_footer(&mut ui);
    tap_key(&mut ui, KeyCode::ArrowDown);
    tap_key(&mut ui, KeyCode::Enter);
    tap_key(&mut ui, KeyCode::ArrowLeft);
    assert_eq!(ui.focus(), Some(yes));
    tap_key(&mut ui, KeyCode::Enter);

    let changes = take_options(&mut ui);
    assert_eq!(
        changes,
        [OptionsChange::Reset(vec!["t.vol".into(), "t.sub".into()])]
    );
    // The app resets each key in the store, as the game's options host does.
    let _ = store.take_changes();
    let OptionsChange::Reset(keys) = &changes[0] else {
        unreachable!()
    };
    for key in keys {
        assert!(store.reset(key));
    }
    assert_eq!(store.get("t.vol"), Some(&SettingValue::Float(0.5)));
    assert_eq!(store.get("t.sub"), Some(&SettingValue::Bool(true)));
    assert_eq!(store.get("t.lang"), Some(&SettingValue::Text("fr".into())));
    let fired: Vec<String> = store
        .take_changes()
        .into_iter()
        .map(|c| c.key.to_string())
        .collect();
    assert_eq!(fired, ["t.vol", "t.sub"], "a change per reset setting");

    // The screen shows the defaults at once; the language row keeps its value.
    assert_eq!(row_value(&ui, root, "t.vol"), SettingValue::Float(0.5));
    assert_eq!(row_value(&ui, root, "t.sub"), SettingValue::Bool(true));
    assert_eq!(row_value(&ui, root, "t.lang"), SettingValue::Text("fr".into()));
    assert_eq!(option_value(&ui, root, "t.vol"), "50%");
    // Nothing left to reset: disabled, focus on Back, the question gone, the window unmoved.
    assert!(!screen(&ui, root).asking_reset());
    assert!(!overlay_shown(&ui, root));
    assert!(disabled(&ui, reset));
    assert_eq!(ui.focus(), Some(back));
    assert_eq!(window_rect(&ui, root), rect, "the reset moved the window");
}
