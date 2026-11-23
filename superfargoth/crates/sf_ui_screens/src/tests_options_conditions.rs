//! Options rows honour their records (menus.md §4 "Settings v2"): `enabled_if` with its
//! `disabled_reason`, `visible_if`, `order` and `format`.

use sf_records::{RecordDb, RecordFormat, apply_record_file};
use sf_ui::{SettingValue, UiRegistry, register_kinds};

use super::*;

const SETTINGS: &str = r#"
[[define]]
kind = "ui.setting"
id = "t.a"
value = { label = "set-a", page = "p", kind = { Choice = { default = "off", options = ["off", "on"] } } }

[[define]]
kind = "ui.setting"
id = "t.b"
value = { label = "set-b", page = "p", kind = { Bool = { default = true } }, enabled_if = [{ setting = "t.a", is = "on" }], disabled_reason = "set-b-why" }

[[define]]
kind = "ui.setting"
id = "t.c"
value = { label = "set-c", page = "p", kind = { Bool = { default = true } }, visible_if = [{ setting = "t.a", is = "on" }] }

[[define]]
kind = "ui.setting"
id = "t.d"
value = { label = "set-d", page = "p", kind = { Bool = { default = true } }, order = -1 }

[[define]]
kind = "ui.setting"
id = "t.e"
value = { label = "set-e", page = "p", kind = { Bool = { default = true } }, order = 1, visible_if = [{ phase = "frontend" }], enabled_if = [{ trait = "dlss" }] }

[[define]]
kind = "ui.setting"
id = "t.only"
value = { label = "set-only", page = "q", kind = { Bool = { default = true } }, visible_if = [{ setting = "t.a", is = "on" }] }

[[define]]
kind = "ui.setting"
id = "t.pct"
value = { label = "set-pct", page = "f", kind = { Float = { default = 0.25, min = 0.0, max = 1.0, step = 0.05 } }, format = "percent" }

[[define]]
kind = "ui.setting"
id = "t.ipct"
value = { label = "set-ipct", page = "f", kind = { Int = { default = 40, min = 0, max = 100, step = 5 } }, format = "percent" }

[[define]]
kind = "ui.setting"
id = "t.ms"
value = { label = "set-ms", page = "f", kind = { Float = { default = 16.5, min = 0.0, max = 50.0, step = 0.5 } }, format = "ms" }

[[define]]
kind = "ui.setting"
id = "t.ims"
value = { label = "set-ims", page = "f", kind = { Int = { default = 16, min = 0, max = 50, step = 1 } }, format = "ms" }

[[define]]
kind = "ui.setting"
id = "t.x"
value = { label = "set-x", page = "f", kind = { Float = { default = 1.5, min = 0.5, max = 2.0, step = 0.5 } }, format = "x" }

[[define]]
kind = "ui.setting"
id = "t.raw"
value = { label = "set-raw", page = "f", kind = { Float = { default = 1.25, min = 0.5, max = 2.0, step = 0.25 } }, format = "raw" }

[[define]]
kind = "ui.setting"
id = "t.unit"
value = { label = "set-unit", page = "f", kind = { Float = { default = 0.5, min = 0.0, max = 1.0 } } }

[[define]]
kind = "ui.setting"
id = "t.int"
value = { label = "set-int", page = "f", kind = { Int = { default = 7, min = 0, max = 10, step = 1 } } }
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

fn localizer() -> sf_ui::Localizer {
    let source = sf_ui::L10nSource {
        mod_id: "t".into(),
        locale: "en".parse().unwrap(),
        path: "l10n/en/t.ftl".into(),
        text: "set-b-why = Needs A on.\n".into(),
    };
    sf_ui::Localizer::build(&["en".parse().unwrap()], &[source], false).0
}

fn model() -> OptionsModel {
    OptionsModel::from_settings(&registry(), None, Some(&localizer()))
}

fn keys(model: &OptionsModel, page: &str) -> Vec<String> {
    model
        .pages
        .iter()
        .find(|p| p.id == page)
        .map(|p| p.rows.iter().map(|r| r.key.clone()).collect())
        .unwrap_or_default()
}

/// The test's pages in order (the built-in audio and interface pages come first).
fn page_ids(model: &OptionsModel) -> Vec<String> {
    model
        .pages
        .iter()
        .map(|p| p.id.clone())
        .filter(|id| ["p", "q", "f"].contains(&id.as_str()))
        .collect()
}

fn set(model: &mut OptionsModel, key: &str, value: &str) {
    let row = model
        .pages
        .iter_mut()
        .flat_map(|p| &mut p.rows)
        .find(|r| r.key == key)
        .unwrap();
    row.value = SettingValue::Text(value.into());
}

#[test]
fn enabled_if_disables_with_its_reason_until_the_setting_matches() {
    let mut model = model();
    let b = model.row("t.b").unwrap();
    assert!(!b.enabled, "t.a is off");
    assert_eq!(b.disabled_reason.as_deref(), Some("Needs A on."));
    set(&mut model, "t.a", "on");
    assert!(model.refresh());
    assert!(model.row("t.b").unwrap().enabled);
    assert!(!model.refresh(), "a second refresh changes nothing");
    set(&mut model, "t.a", "off");
    assert!(model.refresh());
    assert!(!model.row("t.b").unwrap().enabled);
}

#[test]
fn visible_if_hides_and_shows_rows_and_pages() {
    let mut model = model();
    assert_eq!(keys(&model, "p"), ["t.d", "t.a", "t.b", "t.e"]);
    assert_eq!(page_ids(&model), ["p", "f"], "page q has no visible row");
    assert!(model.row("t.c").is_none());
    set(&mut model, "t.a", "on");
    model.refresh();
    assert_eq!(keys(&model, "p"), ["t.d", "t.a", "t.b", "t.c", "t.e"], "back in its place");
    assert_eq!(page_ids(&model), ["p", "q", "f"], "page q back in its place");
    set(&mut model, "t.a", "off");
    model.refresh();
    assert_eq!(keys(&model, "p"), ["t.d", "t.a", "t.b", "t.e"]);
    assert_eq!(page_ids(&model), ["p", "f"]);
}

#[test]
fn phase_and_trait_conditions_hold_until_the_context_is_known() {
    let mut model = model();
    assert!(model.row("t.e").unwrap().enabled, "unknown traits hold");
    model.set_condition_context(Some(sf_ui::PHASE_IN_GAME), Some(&[][..]));
    assert!(model.row("t.e").is_none(), "frontend only");
    model.set_condition_context(Some(sf_ui::PHASE_FRONTEND), Some(&[][..]));
    assert!(!model.row("t.e").unwrap().enabled, "no dlss");
    model.set_condition_context(Some(sf_ui::PHASE_FRONTEND), Some(&["dlss"][..]));
    assert!(model.row("t.e").unwrap().enabled);
}

#[test]
fn order_sorts_rows_within_the_page_stable_on_ties() {
    // Load order a, b, c (hidden), d (order -1), e (order 1): d first, e last, the rest (0) in
    // load order.
    let model = model();
    assert_eq!(keys(&model, "p"), ["t.d", "t.a", "t.b", "t.e"]);
}

#[test]
fn format_renders_each_value_as_its_record_says() {
    let model = model();
    let labels = ScreenLabels::default();
    let text = |key: &str| model.row(key).unwrap().value_text(&labels);
    assert_eq!(text("t.pct"), "25%");
    assert_eq!(text("t.ipct"), "40%");
    assert_eq!(text("t.ms"), "16.5 ms");
    assert_eq!(text("t.ims"), "16 ms");
    assert_eq!(text("t.x"), "1.5x");
    assert_eq!(text("t.raw"), "1.25");
    assert_eq!(text("t.unit"), "50%", "raw 0..1 float reads as a percentage, as before");
    assert_eq!(text("t.int"), "7");
    assert_eq!(model.row("t.ms").unwrap().default_text(&labels), "16.5 ms");
}

/// The screen: a click cycling `t.a` enables `t.b` (widget state and pane) and shows `t.c` the
/// same way it would any model change, then settles: no respawned row, no change every frame.
#[test]
fn options_screen_follows_conditions_when_a_value_changes() {
    let (mut ui, root) = open_options_with(model());
    show_options_page(&mut ui, root, "p");
    let b_row = options_parts(&ui, root).row("t.b").unwrap();
    assert!(ui.world().get::<bevy_ui::InteractionDisabled>(b_row).is_some());
    assert!(options_parts(&ui, root).row("t.c").is_none());
    {
        let mut screen = ui.world_mut().get_mut::<OptionsScreen>(root).unwrap();
        screen.set_hovered(Some("t.b".into()));
    }
    ui.frames(1);
    assert_eq!(ui.text_of(options_parts(&ui, root).pane().reason), "Needs A on.");

    let a_row = options_parts(&ui, root).row("t.a").unwrap();
    ui.world_mut().trigger(Activate { entity: a_row });
    ui.frames(2);
    assert_eq!(
        take_options(&mut ui),
        [OptionsChange::Set("t.a".into(), SettingValue::Text("on".into()))]
    );
    let parts = options_parts(&ui, root);
    assert_eq!(parts.row("t.b"), Some(b_row), "the row is updated, not respawned");
    assert!(ui.world().get::<bevy_ui::InteractionDisabled>(b_row).is_none());
    assert!(parts.row("t.c").is_some(), "t.c shown");
    assert_eq!(
        parts.row_keys(),
        ["t.d", "t.a", "t.b", "t.c", "t.e"],
        "t.c in its sorted place"
    );
    assert_eq!(ui.text_of(parts.pane().reason), "", "no reason while enabled");

    let tick = |ui: &Ui| {
        ui.world()
            .entity(root)
            .get_ref::<OptionsScreen>()
            .unwrap()
            .last_changed()
    };
    let settled = tick(&ui);
    ui.frames(3);
    assert_eq!(tick(&ui), settled, "no refresh loop");
}
