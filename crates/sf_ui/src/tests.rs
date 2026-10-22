use std::collections::BTreeMap;

use sf_records::{RecordDb, RecordFormat, apply_record_file};

use crate::records::{PausePolicy, ScreenImpl, SettingKind};
use crate::*;

const BASE: &str = r#"
[[define]]
kind = "ui.screen"
id = "hud"
value = { layer = "Hud", implementation = { Template = "hud" }, slots = ["hud.top_right"] }

[[define]]
kind = "ui.screen"
id = "inventory"
value = { layer = "Menu", pause = "WhileOpen", implementation = { Template = "inventory" }, slots = ["item_tooltip.after_stats"], presets = ["pc_dense"] }

[[define]]
kind = "ui.screen"
id = "message_box"
value = { layer = "Modal", input = "modal", implementation = { Template = "message_box" } }

[[define]]
kind = "ui.screen"
id = "toast"
value = { layer = "Notification", implementation = { Template = "toast" } }

[[define]]
kind = "ui.slot"
id = "base.compass"
value = { slot = "hud.top_right", priority = 10, content = { Template = "compass" } }

[[define]]
kind = "ui.column"
id = "base.weight"
value = { list = "inventory.items", header = "col-weight", sort = "Number", value = { Field = "weight" }, priority = 20 }

[[define]]
kind = "ui.column"
id = "base.name"
value = { list = "inventory.items", header = "col-name", sort = "Text", value = { Field = "name" } }

[[define]]
kind = "ui.theme"
id = "modern"
value = { name = "theme-modern", stylesheet = "ui/themes/modern/theme.css", fonts = { body = ["Inter", "Noto Sans"] } }

[[define]]
kind = "ui.setting"
id = "base.text_scale"
value = { label = "set-text-scale", kind = { Float = { default = 1.0, min = 1.0, max = 2.0, step = 0.25 } } }

[[define]]
kind = "ui.setting"
id = "base.subtitles"
value = { label = "set-subtitles", kind = { Choice = { default = "on", options = ["off", "on", "speaker"] } } }
"#;

const MOD: &str = r#"
[[define]]
kind = "ui.screen"
id = "inventory"
value = { layer = "Menu", pause = "WhileOpen", implementation = { Bsn = "ui/better/inventory.bsn" }, slots = ["item_tooltip.after_stats"] }

[[define]]
kind = "ui.slot"
id = "better.clock"
value = { slot = "hud.top_right", content = { Template = "clock" } }

[[define]]
kind = "ui.slot"
id = "better.after_compass"
value = { slot = "hud.top_right", priority = 10, content = { Template = "after" } }

[[define]]
kind = "ui.slot"
id = "better.lost"
value = { slot = "hud.nowhere", content = { Template = "lost" } }

[[define]]
kind = "ui.column"
id = "better.value_weight"
value = { list = "inventory.items", header = "col-vw", sort = "Number", value = { Lua = { function = "vw" } }, priority = 20 }

[[define]]
kind = "ui.theme"
id = "parchment"
value = { name = "theme-parchment", stylesheet = "ui/themes/parchment/theme.css", presets = ["tv"] }

[[define]]
kind = "ui.setting"
id = "better.show_clock"
value = { label = "set-clock", kind = { Bool = { default = true } } }

[[define]]
kind = "ui.setting"
id = "better.rows"
value = { label = "set-rows", kind = { Int = { default = 50, min = 10, max = 40 } } }

[[define]]
kind = "ui.setting"
id = "better.density"
value = { label = "set-density", kind = { Int = { default = 2, min = 0, max = 10, step = 2 } } }

[[patch]]
kind = "ui.setting"
id = "base.text_scale"
set = { "label" = "set-text-scale-better" }
"#;

fn db() -> RecordDb {
    let mut db = RecordDb::new();
    register_kinds(&mut db).unwrap();
    let base = db.add_source("base", &[]);
    let better = db.add_source("better", &[base]);
    for (source, text) in [(base, BASE), (better, MOD)] {
        let outcome = apply_record_file(&mut db, source, text, RecordFormat::Toml).unwrap();
        assert_eq!(outcome.errors, [], "{text}");
    }
    db
}

fn registry() -> (UiRegistry, Vec<UiConflict>) {
    UiRegistry::build(&db())
}

#[test]
fn registry_resolves_replacements_and_owners() {
    let (reg, conflicts) = registry();
    let inv = reg.screen("inventory").unwrap();
    assert_eq!(inv.owner.as_str(), "better");
    assert_eq!(
        inv.def.implementation,
        ScreenImpl::Bsn("ui/better/inventory.bsn".into())
    );
    assert!(conflicts.contains(&UiConflict::ScreenReplaced {
        screen: "inventory".into(),
        by: "better".into(),
        replaced: vec!["base".into()],
    }));
    assert_eq!(reg.screens().count(), 4);
    // A patch keeps the owner (the defining mod) and applies the field.
    let scale = reg.setting("base.text_scale").unwrap();
    assert_eq!(
        (scale.owner.as_str(), scale.def.label.as_str()),
        ("base", "set-text-scale-better")
    );
}

#[test]
fn slots_order_by_priority_then_load_order() {
    let (reg, conflicts) = registry();
    let ids: Vec<&str> = reg
        .slot("hud.top_right")
        .iter()
        .map(|d| d.id.as_str())
        .collect();
    assert_eq!(
        ids,
        ["better.clock", "base.compass", "better.after_compass"]
    );
    assert!(reg.slot("no.such.slot").is_empty());
    let unknown: Vec<_> = conflicts
        .iter()
        .filter(|c| matches!(c, UiConflict::UnknownSlot { .. }))
        .collect();
    assert_eq!(unknown.len(), 1);
    assert_eq!(unknown[0].owner().as_str(), "better");
    assert_eq!(unknown[0].severity(), Severity::Warning);
}

#[test]
fn columns_presets_themes_settings() {
    let (reg, conflicts) = registry();
    let cols: Vec<&str> = reg
        .columns("inventory.items")
        .iter()
        .map(|d| d.id.as_str())
        .collect();
    assert_eq!(cols, ["base.name", "base.weight", "better.value_weight"]);

    // Built-in presets exist without records.
    assert_eq!(reg.preset("pc_dense").unwrap().def.root_class, "preset-pc");
    assert_eq!(reg.preset("controller").unwrap().owner.as_str(), "sf");
    assert_eq!(
        reg.theme("modern").unwrap().def.fonts["body"],
        ["Inter", "Noto Sans"]
    );
    assert!(conflicts.contains(&UiConflict::UnknownPreset {
        record: "parchment".into(),
        preset: "tv".into(),
        owner: "better".into()
    }));

    // Invalid default (50 outside 10..=40) is dropped with an error.
    assert!(reg.setting("better.rows").is_none());
    let invalid = conflicts
        .iter()
        .find(|c| matches!(c, UiConflict::InvalidSetting { .. }))
        .unwrap();
    assert_eq!(invalid.severity(), Severity::Error);
    assert!(invalid.to_string().contains("better.rows"), "{invalid}");
    assert_eq!(reg.settings().count(), 4);
}

#[test]
fn unregistered_kinds_build_an_empty_registry() {
    let (reg, conflicts) = UiRegistry::build(&RecordDb::new());
    assert_eq!(
        (
            reg.screens().count(),
            reg.presets().count(),
            conflicts.len()
        ),
        (0, 2, 0)
    );
}

#[test]
fn stack_routes_input_to_top_of_highest_input_layer() {
    let (reg, _) = registry();
    let mut stack = ScreenStack::new();
    assert!(stack.input_target().is_none());
    stack.push(&reg, "hud").unwrap();
    assert_eq!(stack.input_target().unwrap().id.as_str(), "hud");
    assert!(!stack.pauses_game());

    stack.push(&reg, "inventory").unwrap();
    stack.push(&reg, "toast").unwrap(); // notifications never take input
    assert_eq!(stack.input_target().unwrap().id.as_str(), "inventory");
    assert_eq!(
        stack.input_target().unwrap().input.as_str(),
        "inventory",
        "context defaults to screen id"
    );
    assert!(stack.pauses_game());

    let modal = stack.push(&reg, "message_box").unwrap();
    assert_eq!((modal.layer, modal.input.as_str()), (Layer::Modal, "modal"));
    assert_eq!(stack.input_target().unwrap().id.as_str(), "message_box");

    // Closing the modal falls back to the menu, then to the HUD.
    assert_eq!(stack.pop(Layer::Modal).unwrap().id.as_str(), "message_box");
    assert_eq!(stack.input_target().unwrap().id.as_str(), "inventory");
    assert!(stack.close("inventory"));
    assert_eq!(stack.input_target().unwrap().id.as_str(), "hud");
    assert!(!stack.pauses_game());
    assert!(stack.is_open("toast"));

    assert_eq!(
        stack.push(&reg, "nope").unwrap_err(),
        StackError::UnknownScreen("nope".into())
    );
}

#[test]
fn stack_reopen_moves_to_top_and_replace_swaps() {
    let (reg, _) = registry();
    let mut stack = ScreenStack::new();
    stack.push(&reg, "inventory").unwrap();
    let v = stack.version();
    stack.push(&reg, "inventory").unwrap();
    assert_eq!(stack.layer(Layer::Menu).len(), 1);
    assert!(stack.version() > v);

    stack.replace_top(&reg, "message_box").unwrap(); // its own layer (empty): a push
    assert_eq!(stack.top(Layer::Menu).unwrap().id.as_str(), "inventory");
    assert_eq!(stack.top(Layer::Modal).unwrap().pause, PausePolicy::Never);
    stack.clear(Layer::Modal);
    assert!(stack.top(Layer::Modal).is_none());
}

#[derive(Clone, Debug, PartialEq)]
struct Item {
    name: &'static str,
    weight: f32,
    category: &'static str,
}

impl ViewModel for Item {
    const KIND: &'static str = "test.item";

    fn fields() -> &'static [&'static str] {
        &["name", "weight", "category"]
    }

    fn field(&self, name: &str) -> Option<UiValue> {
        match name {
            "name" => Some(self.name.into()),
            "weight" => Some(self.weight.into()),
            "category" => Some(self.category.into()),
            _ => None,
        }
    }
}

fn items() -> ListModel<u32, Item> {
    let mut list = ListModel::new();
    for (k, name, weight, category) in [
        (1, "iron dagger", 3.0, "weapon"),
        (2, "Bread", 0.5, "food"),
        (3, "apple", 0.5, "food"),
        (4, "steel axe", 12.0, "weapon"),
    ] {
        assert!(list.upsert(
            k,
            Item {
                name,
                weight,
                category
            }
        ));
    }
    list
}

#[test]
fn list_sorts_stably_by_field() {
    let mut list = items();
    assert_eq!(list.view(), [1, 2, 3, 4], "insertion order without a sort");
    list.set_sort(Some(SortSpec {
        field: "weight".into(),
        descending: false,
    }));
    assert_eq!(list.view(), [2, 3, 1, 4], "ties keep insertion order");
    list.set_sort(Some(SortSpec {
        field: "weight".into(),
        descending: true,
    }));
    assert_eq!(
        list.view(),
        [4, 1, 2, 3],
        "ties keep insertion order descending too"
    );
    list.set_sort(Some(SortSpec {
        field: "name".into(),
        descending: false,
    }));
    assert_eq!(list.view(), [3, 2, 1, 4], "text sorts case-insensitively");
    list.set_sort(Some(SortSpec {
        field: "missing".into(),
        descending: false,
    }));
    assert_eq!(list.view(), [1, 2, 3, 4]);
}

#[test]
fn list_filters_and_tracks_versions() {
    let mut list = items();
    list.set_filter(ListFilter {
        search: "A".into(),
        search_fields: vec!["name".into()],
        equals: vec![],
        one_of: vec![],
    });
    assert_eq!(list.view(), [1, 2, 3, 4], "every name has an 'a'");
    list.set_filter(ListFilter {
        search: "a".into(),
        search_fields: vec!["name".into()],
        equals: vec![("category".into(), "weapon".into())],
        one_of: vec![],
    });
    assert_eq!(list.view(), [1, 4]);

    let v = list.version();
    assert!(
        !list.upsert(
            4,
            Item {
                name: "steel axe",
                weight: 12.0,
                category: "weapon"
            }
        ),
        "unchanged row"
    );
    assert_eq!(list.version(), v);
    assert!(list.upsert(
        4,
        Item {
            name: "steel axe",
            weight: 11.0,
            category: "weapon"
        }
    ));
    assert!(list.upsert(
        5,
        Item {
            name: "glass axe",
            weight: 6.0,
            category: "weapon"
        }
    ));
    assert_eq!(list.changed_since(v).copied().collect::<Vec<_>>(), [4, 5]);
    assert!(list.row_version(&4).unwrap() > v);
    assert_eq!(list.row_version(&1), Some(1));
    assert_eq!(list.view(), [1, 4, 5]);
    assert_eq!(list.window(1, 10), [4, 5]);
    assert!(list.window(9, 2).is_empty());
    assert_eq!(list.remove(&1).unwrap().name, "iron dagger");
    assert_eq!(list.view(), [4, 5]);
    list.clear();
    assert!(list.is_empty() && list.view().is_empty());
}

#[test]
fn ui_value_sort_order() {
    use std::cmp::Ordering::*;
    assert_eq!(UiValue::Int(2).sort_cmp(&UiValue::Float(2.5)), Less);
    assert_eq!(UiValue::None.sort_cmp(&UiValue::Int(0)), Greater);
    assert_eq!(UiValue::Int(1).sort_cmp(&"a".into()), Less);
    assert_eq!(UiValue::from(Some(3)).to_string(), "3");
}

#[test]
fn settings_defaults_set_and_coerce() {
    let (reg, _) = registry();
    let mut s = SettingsStore::new(&reg);
    assert_eq!(s.get_bool("better.show_clock"), Some(true));
    assert_eq!(s.get_f64("base.text_scale"), Some(1.0));

    assert_eq!(
        s.set("base.text_scale", SettingValue::Float(1.6)).unwrap(),
        &SettingValue::Float(1.5),
        "snapped"
    );
    assert_eq!(
        s.set("base.text_scale", SettingValue::Int(9)).unwrap(),
        &SettingValue::Float(2.0),
        "clamped"
    );
    assert_eq!(
        s.set("better.density", SettingValue::Int(5)).unwrap(),
        &SettingValue::Int(6),
        "rounds half up"
    );
    assert_eq!(
        s.set("better.density", SettingValue::Float(-3.0)).unwrap(),
        &SettingValue::Int(0)
    );
    assert!(matches!(
        s.set("better.show_clock", SettingValue::Int(1)),
        Err(SettingError::WrongType { .. })
    ));
    assert!(matches!(
        s.set("base.subtitles", SettingValue::Text("loud".into())),
        Err(SettingError::NotAnOption { .. })
    ));
    assert_eq!(
        s.set("nope", SettingValue::Bool(true)).unwrap_err(),
        SettingError::Unknown("nope".into())
    );
    assert!(s.reset("better.density"));
    assert_eq!(s.get("better.density"), Some(&SettingValue::Int(2)));
}

#[test]
fn settings_profile_round_trip() {
    let (reg, _) = registry();
    let mut s = SettingsStore::new(&reg);
    let profile: BTreeMap<String, toml::Table> = toml::from_str(
        r#"
        [base]
        text_scale = 1.25
        subtitles = "loud"
        [better]
        show_clock = false
        [gone_mod]
        keep = 7
        "#,
    )
    .unwrap();
    let errors = s.load_profile(&profile);
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert_eq!(
        s.get("base.subtitles"),
        Some(&SettingValue::Text("on".into())),
        "bad value keeps the default"
    );
    assert_eq!(s.get_f64("base.text_scale"), Some(1.25));
    assert_eq!(s.get_bool("better.show_clock"), Some(false));

    let saved = s.to_profile();
    assert_eq!(
        saved["base"].len(),
        1,
        "only non-defaults are saved: {saved:?}"
    );
    assert_eq!(saved["better"]["show_clock"], toml::Value::Boolean(false));
    assert_eq!(
        saved["gone_mod"]["keep"],
        toml::Value::Integer(7),
        "unknown entries pass through"
    );
    assert!(matches!(
        reg.setting("base.subtitles").unwrap().def.kind,
        SettingKind::Choice { .. }
    ));
}

fn src(mod_id: &str, locale: &str, text: &str) -> L10nSource {
    L10nSource {
        mod_id: mod_id.into(),
        locale: locale.parse().unwrap(),
        path: format!("l10n/{locale}/{mod_id}.ftl"),
        text: text.into(),
    }
}

#[test]
fn localizer_overrides_in_load_order_and_falls_back() {
    let sources = [
        src(
            "base",
            "en-US",
            "hello = Hello, { $name }!\nquit = Quit\n    .tooltip = Leave the game\nonly-en = English only\n",
        ),
        src(
            "base",
            "fr",
            "hello = Bonjour, { $name } !\nquit = Quitter\n",
        ),
        src("better", "fr", "quit = Sortir\n"),
    ];
    let chain: Vec<LanguageIdentifier> = vec!["fr".parse().unwrap(), "en-US".parse().unwrap()];
    let (l10n, issues) = Localizer::build(&chain, &sources, false);
    assert!(issues.is_empty(), "{issues:?}");
    assert_eq!(
        l10n.locales().map(ToString::to_string).collect::<Vec<_>>(),
        ["fr", "en-US"]
    );

    let mut args = FluentArgs::new();
    args.set("name", "Nerevar");
    assert_eq!(l10n.format("hello", Some(&args)), "Bonjour, Nerevar !");
    assert_eq!(l10n.format("quit", None), "Sortir", "a later mod overrides");
    assert_eq!(
        l10n.format("quit.tooltip", None),
        "Leave the game",
        "attribute from the fallback locale"
    );
    assert_eq!(l10n.format("only-en", None), "English only");
    assert_eq!(l10n.format("missing-key", None), "missing-key");
    assert!(l10n.has("quit.tooltip") && !l10n.has("quit.nope") && !l10n.has("missing-key"));
}

#[test]
fn localizer_reports_syntax_errors_per_mod() {
    let sources = [src(
        "broken",
        "en-US",
        "good = Fine\nbad line without a value\nalso-good = Yes\n",
    )];
    let (l10n, issues) = Localizer::build(&["en-US".parse().unwrap()], &sources, true);
    assert_eq!(issues.len(), 1, "{issues:?}");
    assert_eq!(
        (issues[0].mod_id.as_str(), issues[0].path.as_str()),
        ("broken", "l10n/en-US/broken.ftl")
    );
    assert_eq!(l10n.format("good", None), "Fine");
    assert_eq!(l10n.format("also-good", None), "Yes");
}

#[test]
fn negotiate_builds_a_fallback_chain() {
    let available: Vec<LanguageIdentifier> = ["en-US", "fr", "de"]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
    let chain = negotiate(&["fr-CA", "de"], &available, &available[0]);
    assert_eq!(
        chain.iter().map(ToString::to_string).collect::<Vec<_>>(),
        ["fr", "de", "en-US"]
    );
}
