//! Settings v2 record fields and the condition evaluator (menus.md "Settings v2").

use sf_records::{RecordDb, RecordFormat, apply_record_file};

use crate::records::{SettingApply, SettingDef, SettingKind, SettingScope, ValueFormat};
use crate::*;

fn load(text: &str, format: RecordFormat) -> UiRegistry {
    let mut db = RecordDb::new();
    register_kinds(&mut db).unwrap();
    let src = db.add_source("m", &[]);
    let outcome = apply_record_file(&mut db, src, text, format).unwrap();
    assert_eq!(outcome.errors, [], "{text}");
    let (reg, conflicts) = UiRegistry::build(&db);
    assert_eq!(conflicts, [], "invalid settings are reported here");
    reg
}

fn full() -> SettingDef {
    SettingDef {
        description: Some("set-up-desc".into()),
        page: Some("graphics".into()),
        group: Some("upscaling".into()),
        apply: SettingApply::Confirm {
            keep_seconds: Some(15),
        },
        scope: SettingScope::Machine,
        enabled_if: vec![
            Condition::Setting {
                setting: "m.upscaler".into(),
                is: SettingValueMatch::AnyOf(vec![
                    SettingValue::Text("dlss".into()),
                    SettingValue::Text("fsr".into()),
                ]),
            },
            Condition::Trait {
                r#trait: "dlss".into(),
            },
        ],
        visible_if: vec![Condition::Phase {
            phase: PHASE_FRONTEND.into(),
        }],
        disabled_reason: Some("set-up-needs-dlss".into()),
        order: Some(-2),
        keywords: Some("set-up-keywords".into()),
        reset: false,
        format: ValueFormat::Times,
        ..SettingDef::new(
            "set-up",
            SettingKind::Float {
                default: 1.0,
                min: 0.5,
                max: 2.0,
                step: Some(0.25),
                log_scale: true,
            },
        )
    }
}

#[test]
fn every_v2_field_round_trips() {
    let def = full();
    let text = toml::to_string(&def).unwrap();
    assert_eq!(toml::from_str::<SettingDef>(&text).unwrap(), def, "{text}");

    let reg = load(
        r#"
[[define]]
kind = "ui.setting"
id = "m.up"
value = { label = "set-up", description = "set-up-desc", page = "graphics", group = "upscaling", kind = { Float = { default = 1.0, min = 0.5, max = 2.0, step = 0.25, log_scale = true } }, apply = { Confirm = { keep_seconds = 15 } }, scope = "Machine", enabled_if = [{ setting = "m.upscaler", is = ["dlss", "fsr"] }, { trait = "dlss" }], visible_if = [{ phase = "frontend" }], disabled_reason = "set-up-needs-dlss", order = -2, keywords = "set-up-keywords", reset = false, format = "x" }

[[define]]
kind = "ui.setting"
id = "m.monitor"
value = { label = "set-monitor", kind = { Choice = { default = "primary", options_from = "display.monitors" } } }

[[define]]
kind = "ui.setting"
id = "m.calibrate"
value = { label = "set-calibrate", kind = "Action" }
"#,
        RecordFormat::Toml,
    );
    assert_eq!(reg.setting("m.up").unwrap().def, def);
    assert_eq!(
        reg.setting("m.monitor").unwrap().def.kind,
        SettingKind::Choice {
            default: "primary".into(),
            options: Vec::new(),
            options_from: Some("display.monitors".into()),
        }
    );
    assert_eq!(reg.setting("m.calibrate").unwrap().def.kind, SettingKind::Action);
    let store = SettingsStore::new(&reg);
    assert_eq!(store.get("m.calibrate"), None, "an action has no value");

    let reg = load(
        r#"[
  define(kind: "ui.setting", id: "m.up", value: (
    label: "set-up", description: Some("set-up-desc"), page: Some("graphics"), group: Some("upscaling"),
    kind: Float(default: 1.0, min: 0.5, max: 2.0, step: Some(0.25), log_scale: true),
    apply: Confirm(keep_seconds: Some(15)), scope: Machine,
    enabled_if: [{ "setting": "m.upscaler", "is": ["dlss", "fsr"] }, { "trait": "dlss" }],
    visible_if: [{ "phase": "frontend" }],
    disabled_reason: Some("set-up-needs-dlss"), order: Some(-2), keywords: Some("set-up-keywords"),
    reset: false, format: x,
  )),
]"#,
        RecordFormat::Ron,
    );
    assert_eq!(reg.setting("m.up").unwrap().def, def, "RON");
}

#[test]
fn v1_records_get_the_defaults() {
    let reg = load(
        r#"
[[define]]
kind = "ui.setting"
id = "m.subtitles"
value = { label = "set-subtitles", kind = { Choice = { default = "on", options = ["on", "off"] } } }

[[define]]
kind = "ui.setting"
id = "m.scale"
value = { label = "set-scale", kind = { Float = { default = 1.0, min = 0.5, max = 2.0 } } }
"#,
        RecordFormat::Toml,
    );
    let sub = &reg.setting("m.subtitles").unwrap().def;
    assert_eq!(
        *sub,
        SettingDef::new(
            "set-subtitles",
            SettingKind::Choice {
                default: "on".into(),
                options: vec!["on".into(), "off".into()],
                options_from: None,
            }
        )
    );
    assert_eq!(
        (sub.apply, sub.scope, sub.reset, sub.format),
        (SettingApply::Live, SettingScope::Profile, true, ValueFormat::Raw)
    );
    assert_eq!(
        reg.setting("m.scale").unwrap().def.kind,
        SettingKind::Float {
            default: 1.0,
            min: 0.5,
            max: 2.0,
            step: None,
            log_scale: false,
        }
    );
}

#[test]
fn conditions_truth_table() {
    let values = |key: &str| match key {
        "m.upscaler" => Some(SettingValue::Text("dlss".into())),
        "m.count" => Some(SettingValue::Int(3)),
        "m.on" => Some(SettingValue::Bool(true)),
        _ => None,
    };
    let traits = |t: &str| t == "dlss";
    let set = |key: &str, is: SettingValueMatch| Condition::Setting {
        setting: key.into(),
        is,
    };
    let text = |s: &str| SettingValue::Text(s.into());
    let tr = |t: &str| Condition::Trait { r#trait: t.into() };
    let ph = |p: &str| Condition::Phase { phase: p.into() };
    let eval = |conds: &[Condition], phase: &str| evaluate(conds, &values, &traits, phase);

    assert!(eval(&[], PHASE_IN_GAME), "empty = always");
    let cases = [
        (set("m.upscaler", SettingValueMatch::Is(text("dlss"))), true),
        (set("m.upscaler", SettingValueMatch::Is(text("fsr"))), false),
        (
            set("m.upscaler", SettingValueMatch::AnyOf(vec![text("fsr"), text("dlss")])),
            true,
        ),
        (
            set("m.upscaler", SettingValueMatch::AnyOf(vec![text("fsr"), text("off")])),
            false,
        ),
        (set("m.upscaler", SettingValueMatch::AnyOf(vec![])), false),
        (set("m.count", SettingValueMatch::Is(SettingValue::Int(3))), true),
        (
            set("m.count", SettingValueMatch::Is(SettingValue::Float(3.0))),
            true,
        ),
        (set("m.count", SettingValueMatch::Is(text("3"))), false),
        (set("m.on", SettingValueMatch::Is(SettingValue::Bool(true))), true),
        (set("m.missing", SettingValueMatch::Is(text("dlss"))), false),
        (tr("dlss"), true),
        (tr("rt"), false),
        (ph(PHASE_IN_GAME), true),
        (ph(PHASE_FRONTEND), false),
    ];
    for (cond, want) in &cases {
        assert_eq!(eval(std::slice::from_ref(cond), PHASE_IN_GAME), *want, "{cond:?}");
    }
    assert!(eval(&[ph(PHASE_FRONTEND)], PHASE_FRONTEND));

    let both = [tr("dlss"), set("m.upscaler", SettingValueMatch::Is(text("dlss")))];
    assert!(eval(&both, PHASE_IN_GAME), "AND, all hold");
    let one_fails = [tr("dlss"), tr("rt"), ph(PHASE_IN_GAME)];
    assert!(!eval(&one_fails, PHASE_IN_GAME), "AND, one fails");
}
