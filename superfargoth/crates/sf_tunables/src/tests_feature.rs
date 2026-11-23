//! Feature switches (slice FS-1a): on/off, layer, availability, registration.

use crate::{
    AnyDef, Availability, Category, Cost, Def, Effect, FeatureSwitch, FeatureSwitches, Group,
    OffValue, SetBy, SwitchError, Tunables, Value, def,
};

const FOG: Def<bool> = def!(bool "t.fog.enabled" = true).group(Group::Sky);
const FROXELS_ON: Def<bool> = def!(bool "t.froxels.enabled" = true).group(Group::Sky);
const SHAFTS_ON: Def<bool> = def!(bool "t.shafts.enabled" = true);
const BLOOM: Def<&str> = def!(choice "t.bloom" = "high").choices(&["off", "low", "high"]);
const LEVEL: Def<u32> = def!(count "t.level" = 1).range(0, 4);
const DEFS: &[&dyn AnyDef] = &[&FOG, &FROXELS_ON, &SHAFTS_ON, &BLOOM, &LEVEL];

const FOG_SW: FeatureSwitch = FeatureSwitch::new(&FOG).label("Fog");
const FROXELS: FeatureSwitch = FeatureSwitch::new(&FROXELS_ON)
    .label("Volumetric fog")
    .effect(Effect::Skips)
    .cost(&[Cost::Gpu("froxel"), Cost::Cpu("froxel_inject")])
    .requires(&["t.fog.enabled"])
    .at(file!(), line!());
const SHAFTS: FeatureSwitch = FeatureSwitch::new(&SHAFTS_ON)
    .requires(&["t.froxels.enabled"])
    .category(Category::Debug)
    .effect(Effect::Hides);
const BLOOM_SW: FeatureSwitch = FeatureSwitch::choice(&BLOOM, "off").category(Category::Post);

fn store() -> Tunables {
    let mut t = Tunables::default();
    t.register_all(DEFS).unwrap();
    t
}

fn registry(t: &Tunables) -> FeatureSwitches {
    let mut f = FeatureSwitches::default();
    for s in [&FOG_SW, &FROXELS, &SHAFTS, &BLOOM_SW] {
        f.register(s, t).unwrap();
    }
    f
}

fn set(t: &mut Tunables, name: &str, value: Value, layer: SetBy) {
    let id = t.id(name).unwrap();
    t.set_by(id, value, layer).unwrap();
}

#[test]
fn builder_fields() {
    assert_eq!(FROXELS.name(), "t.froxels.enabled");
    assert_eq!(FROXELS.label_text(), "Volumetric fog");
    assert_eq!(SHAFTS.label_text(), "t.shafts.enabled");
    assert_eq!(FROXELS.category_of(), Category::Sky); // from the tunable's group
    assert_eq!(SHAFTS.category_of(), Category::Debug);
    assert_eq!(FeatureSwitch::new(&SHAFTS_ON).category_of(), Category::World);
    assert_eq!(FROXELS.off_value(), OffValue::Bool(false));
    assert_eq!(BLOOM_SW.off_value(), OffValue::Choice("off"));
    assert_eq!(SHAFTS.effect_of(), Effect::Hides);
    assert_eq!(
        FROXELS.costs(),
        &[Cost::Gpu("froxel"), Cost::Cpu("froxel_inject")]
    );
    assert_eq!(FROXELS.required(), &["t.fog.enabled"]);
    assert_eq!(FROXELS.location().map(|l| l.0), Some(file!()));
    assert_eq!(Category::from_group(Group::Foliage), Category::Foliage);
    assert_eq!(Category::World.group(), None);
    assert_eq!(Category::Sky.group(), Some(Group::Sky));
    for c in Category::ALL {
        assert_eq!(c.group().map_or(c, Category::from_group), c);
    }
}

#[test]
fn bool_switch_on_and_off() {
    let mut t = store();
    let f = registry(&t);
    let s = f.state(&t, "t.fog.enabled").unwrap();
    assert!(s.on);
    assert_eq!(s.layer, SetBy::Default);
    assert_eq!(s.availability, Availability::Available);
    assert!(s.running());
    set(&mut t, "t.fog.enabled", Value::Bool(false), SetBy::Console);
    let s = f.state(&t, "T.Fog.Enabled").unwrap();
    assert!(!s.on);
    assert!(!s.running());
    assert!(!f.running(&t, "t.fog.enabled"));
}

#[test]
fn choice_switch_on_and_off() {
    let mut t = store();
    let f = registry(&t);
    assert!(f.state(&t, "t.bloom").unwrap().on);
    set(&mut t, "t.bloom", Value::Str("low".into()), SetBy::User);
    assert!(f.state(&t, "t.bloom").unwrap().on);
    set(&mut t, "t.bloom", Value::Str("off".into()), SetBy::Console);
    let s = f.state(&t, "t.bloom").unwrap();
    assert!(!s.on);
    assert_eq!(s.layer, SetBy::Console);
}

#[test]
fn layer_after_console_and_preset_writes() {
    let mut t = store();
    let f = registry(&t);
    set(&mut t, "t.fog.enabled", Value::Bool(false), SetBy::Preset);
    let s = f.state(&t, "t.fog.enabled").unwrap();
    assert_eq!((s.on, s.layer), (false, SetBy::Preset));
    set(&mut t, "t.fog.enabled", Value::Bool(true), SetBy::Console);
    let s = f.state(&t, "t.fog.enabled").unwrap();
    assert_eq!((s.on, s.layer), (true, SetBy::Console));
    // A later preset write stays under the console's value.
    set(&mut t, "t.fog.enabled", Value::Bool(false), SetBy::Preset);
    let s = f.state(&t, "t.fog.enabled").unwrap();
    assert_eq!((s.on, s.layer), (true, SetBy::Console));
    let id = t.id("t.fog.enabled").unwrap();
    assert!(t.clear(id, SetBy::Console));
    let s = f.state(&t, "t.fog.enabled").unwrap();
    assert_eq!((s.on, s.layer), (false, SetBy::Preset));
}

#[test]
fn blocked_by_required_switch_and_back() {
    let mut t = store();
    let f = registry(&t);
    assert_eq!(
        f.state(&t, "t.froxels.enabled").unwrap().availability,
        Availability::Available
    );
    set(&mut t, "t.fog.enabled", Value::Bool(false), SetBy::Console);
    let s = f.state(&t, "t.froxels.enabled").unwrap();
    assert!(s.on);
    assert_eq!(
        s.availability,
        Availability::Blocked {
            by: "t.fog.enabled"
        }
    );
    assert!(!s.running());
    // Transitive: shafts need froxels, which are on but blocked.
    assert_eq!(
        f.state(&t, "t.shafts.enabled").unwrap().availability,
        Availability::Blocked {
            by: "t.froxels.enabled"
        }
    );
    set(&mut t, "t.fog.enabled", Value::Bool(true), SetBy::Console);
    assert_eq!(
        f.state(&t, "t.froxels.enabled").unwrap().availability,
        Availability::Available
    );
    assert!(f.running(&t, "t.shafts.enabled"));
}

#[test]
fn not_built() {
    let t = store();
    let mut f = registry(&t);
    f.mark_not_built("t.fog.enabled", "--off fog").unwrap();
    let s = f.state(&t, "t.fog.enabled").unwrap();
    assert!(s.on);
    assert_eq!(
        s.availability,
        Availability::NotBuilt {
            reason: "--off fog"
        }
    );
    assert!(!s.running());
    // What requires it is blocked.
    assert_eq!(
        f.state(&t, "t.froxels.enabled").unwrap().availability,
        Availability::Blocked {
            by: "t.fog.enabled"
        }
    );
    assert_eq!(
        f.mark_not_built("t.nope", "x"),
        Err(SwitchError::UnknownSwitch("t.nope".into()))
    );
}

#[test]
fn duplicate_unknown_and_wrong_kind_refused() {
    const GHOST: FeatureSwitch = FeatureSwitch::named("t.ghost", OffValue::Bool(false));
    const NUMBER: FeatureSwitch = FeatureSwitch::named("t.level", OffValue::Bool(false));
    const BAD_OFF: FeatureSwitch = FeatureSwitch::choice(&BLOOM, "none");
    let t = store();
    let mut f = registry(&t);
    assert_eq!(
        f.register(&FOG_SW, &t),
        Err(SwitchError::Duplicate("t.fog.enabled".into()))
    );
    assert_eq!(
        f.register(&GHOST, &t),
        Err(SwitchError::UnknownTunable("t.ghost".into()))
    );
    assert!(matches!(
        f.register(&NUMBER, &t),
        Err(SwitchError::WrongKind { .. })
    ));
    let mut g = FeatureSwitches::default();
    assert!(matches!(
        g.register(&BAD_OFF, &t),
        Err(SwitchError::WrongKind { .. })
    ));
    assert_eq!(f.len(), 4);
    assert!(f.state(&t, "t.ghost").is_none());
}

#[test]
fn registration_order_and_categories() {
    let t = store();
    let f = registry(&t);
    let names: Vec<_> = f.iter().map(FeatureSwitch::name).collect();
    assert_eq!(
        names,
        [
            "t.fog.enabled",
            "t.froxels.enabled",
            "t.shafts.enabled",
            "t.bloom"
        ]
    );
    let sky: Vec<_> = f.by_category(Category::Sky).map(FeatureSwitch::name).collect();
    assert_eq!(sky, ["t.fog.enabled", "t.froxels.enabled"]);
    assert_eq!(f.by_category(Category::Water).count(), 0);
    assert_eq!(f.get("t.bloom").map(FeatureSwitch::name), Some("t.bloom"));
    assert_eq!(f.tunable("t.bloom"), t.id("t.bloom"));
}

#[test]
fn unresolved_and_cyclic_requires_block() {
    const A_ON: Def<bool> = def!(bool "t.a" = true);
    const B_ON: Def<bool> = def!(bool "t.b" = true);
    const A: FeatureSwitch = FeatureSwitch::new(&A_ON).requires(&["t.b"]);
    const B: FeatureSwitch = FeatureSwitch::new(&B_ON).requires(&["t.a", "t.missing"]);
    let mut t = Tunables::default();
    t.register_all(&[&A_ON, &B_ON]).unwrap();
    let mut f = FeatureSwitches::default();
    f.register(&A, &t).unwrap();
    f.register(&B, &t).unwrap();
    assert_eq!(f.unresolved_requires(), [("t.b", "t.missing")]);
    // A cycle terminates and reads as blocked.
    assert!(matches!(
        f.state(&t, "t.a").unwrap().availability,
        Availability::Blocked { .. }
    ));
    assert!(!f.running(&t, "t.b"));
}
