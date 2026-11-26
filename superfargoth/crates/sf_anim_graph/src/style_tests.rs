//! Tests of `style` (AN14a): parse, resolve, profile smoothing, chooser.

use crate::style::{
    Bands, ChooseParams, ClassId, ClassName, Cond, Fallback, Field, ModifierScales, MotionProfile, MotionTargets, Side, SmoothRates, StyleError, StyleState,
    StyleTable, choose,
};

/// The spec's example (`animation-style.md` "Class table"), as written.
const SPEC: &str = r#"(
  base_prefix: "loco.",            // the base set's slot prefix; a class prefix replaces it
  default_class: "neutral",
  classes: {
    "neutral": (prefix: None),
    "old":   (prefix: Some("loco.old."), fallback: Some("neutral"), recipe: Some("old"),
              speeds: { "walk": 1.15, "jog": 2.6 },          // no sprint: the mover caps at jog
              bands: (speed_mps: { "walk": (0.9, 1.3) }, stride_m: (0.45, 0.65), spine_pitch_deg: (8, 22),
                      asymmetry: (0.0, 0.06)),
              modifiers: (hunch: 0.3)),                      // the clips already stoop
    "heavy": (prefix: Some("loco.heavy."), fallback: Some("neutral"), recipe: Some("heavy"),
              speeds: { "walk": 1.4, "jog": 3.8, "sprint": 5.8 }),
    "drunk": (prefix: Some("loco.drunk."), fallback: Identity, recipe: Some("drunk"),
              speeds: { "walk": 1.2, "jog": 3.2 }, modifiers: (sway: 0.3)),
    "limp_l": (prefix: Some("loco.limp_l."), fallback: Identity, recipe: Some("wounded"), side: Left,
              speeds: { "walk": 1.0, "jog": 2.2 }, modifiers: (limp: 0.4)),
    "limp_r": (prefix: Some("loco.limp_r."), fallback: Identity, side: Right,   // rows = mirrored candidates
              speeds: { "walk": 1.0, "jog": 2.2 }, modifiers: (limp: 0.4)),
  },
  rules: [                          // the chooser, first match wins (UE Chooser rows)
    (class: "drunk",  when: [(field: Intoxication, enter: 0.6, exit: 0.45)]),
    (class: "limp_l", when: [(field: Injury, enter: 0.5, exit: 0.35)], side: Some(Left)),
    (class: "limp_r", when: [(field: Injury, enter: 0.5, exit: 0.35)], side: Some(Right)),
    (class: "old",    when: [(field: Age, enter: 0.85, exit: 0.8)]),
  ],                                // no row matches: the profile's style_id, else default_class
)"#;

fn spec() -> StyleTable {
    StyleTable::parse(SPEC).expect("spec fixture parses")
}

fn id(t: &StyleTable, name: &str) -> ClassId {
    t.class(&ClassName::new(name)).expect(name)
}

fn s(v: &[(&str, f32)]) -> Vec<(String, f32)> {
    v.iter().map(|(k, x)| ((*k).to_owned(), *x)).collect()
}

// ------------------------------------------------------------------ parse

#[test]
fn style_spec_fixture_parses_to_expected_rows() {
    let t = spec();
    assert_eq!(t.base_prefix, "loco.");
    assert_eq!(t.default_class, ClassName::new("neutral"));
    let names: Vec<&str> = t.classes.iter().map(|r| r.name.as_str()).collect();
    assert_eq!(names, ["drunk", "heavy", "limp_l", "limp_r", "neutral", "old"]);
    for (i, r) in t.classes.iter().enumerate() {
        assert_eq!(t.class(&r.name), Some(ClassId(u16::try_from(i).unwrap())));
    }
    assert_eq!(t.class(&ClassName::new("nobody")), None);

    let row = |n: &str| t.row(id(&t, n)).unwrap();
    let neutral = row("neutral");
    assert_eq!(neutral.prefix, None);
    assert_eq!(neutral.fallback, Fallback::None);
    assert_eq!(neutral.recipe, None);
    assert_eq!(neutral.side, Side::Center);
    assert!(neutral.speeds.is_empty());
    assert_eq!(neutral.bands, Bands::default());
    assert_eq!(neutral.modifiers, ModifierScales::default());

    let old = row("old");
    assert_eq!(old.prefix.as_deref(), Some("loco.old."));
    assert_eq!(old.fallback, Fallback::Class(ClassName::new("neutral")));
    assert_eq!(old.recipe.as_deref(), Some("old"));
    assert_eq!(old.speeds, s(&[("jog", 2.6), ("walk", 1.15)]));
    assert_eq!(
        old.bands,
        Bands {
            speed_mps: vec![("walk".to_owned(), (0.9, 1.3))],
            stride_m: Some((0.45, 0.65)),
            spine_pitch_deg: Some((8.0, 22.0)),
            asymmetry: Some((0.0, 0.06)),
        }
    );
    assert_eq!(old.modifiers, ModifierScales { hunch: 0.3, ..ModifierScales::default() });

    assert_eq!(row("heavy").speeds, s(&[("jog", 3.8), ("sprint", 5.8), ("walk", 1.4)]));
    assert_eq!(row("heavy").fallback, Fallback::Class(ClassName::new("neutral")));
    assert_eq!(row("drunk").fallback, Fallback::Identity);
    assert_eq!(row("drunk").modifiers, ModifierScales { sway: 0.3, ..ModifierScales::default() });
    assert_eq!(row("limp_l").side, Side::Left);
    assert_eq!(row("limp_l").recipe.as_deref(), Some("wounded"));
    assert_eq!(row("limp_r").side, Side::Right);
    assert_eq!(row("limp_r").recipe, None);
    assert_eq!(row("limp_r").modifiers, ModifierScales { limp: 0.4, ..ModifierScales::default() });

    let rules: Vec<(&str, Option<Side>, Vec<Cond>)> = t.rules.iter().map(|r| (r.class.as_str(), r.side, r.when.clone())).collect();
    let c = |field, enter, exit| vec![Cond { field, enter, exit: Some(exit) }];
    assert_eq!(
        rules,
        [
            ("drunk", None, c(Field::Intoxication, 0.6, 0.45)),
            ("limp_l", Some(Side::Left), c(Field::Injury, 0.5, 0.35)),
            ("limp_r", Some(Side::Right), c(Field::Injury, 0.5, 0.35)),
            ("old", None, c(Field::Age, 0.85, 0.8)),
        ]
    );

    // Postcard-safe: a cooked table round-trips.
    let bytes = postcard::to_stdvec(&t).unwrap();
    assert_eq!(postcard::from_bytes::<StyleTable>(&bytes).unwrap(), t);
}

#[test]
fn style_modifier_scales_clamp_to_0_2() {
    let t = StyleTable::parse(r#"(base_prefix: "loco.", default_class: "n", classes: { "n": (modifiers: (hunch: 5.0, sway: -1.0)) })"#).unwrap();
    let m = t.classes[0].modifiers;
    assert_eq!((m.hunch, m.sway, m.limp), (2.0, 0.0, 1.0));
    assert!(t.rules.is_empty());
}

fn refuse(text: &str) -> StyleError {
    StyleTable::parse(text).expect_err("refused")
}

#[test]
fn style_refuses_unknown_default_class() {
    assert_eq!(
        refuse(r#"(base_prefix: "loco.", default_class: "gone", classes: { "n": () })"#),
        StyleError::UnknownDefaultClass("gone".into())
    );
}

#[test]
fn style_refuses_unknown_fallback() {
    assert_eq!(
        refuse(r#"(base_prefix: "loco.", default_class: "n", classes: { "n": (), "a": (fallback: Some("gone")) })"#),
        StyleError::UnknownFallback { class: "a".into(), fallback: "gone".into() }
    );
}

#[test]
fn style_refuses_unknown_rule_class() {
    assert_eq!(
        refuse(r#"(base_prefix: "loco.", default_class: "n", classes: { "n": () }, rules: [(class: "n", when: []), (class: "gone", when: [])])"#),
        StyleError::UnknownRuleClass { rule: 1, class: "gone".into() }
    );
}

#[test]
fn style_refuses_fallback_cycle() {
    let two = refuse(r#"(base_prefix: "loco.", default_class: "n", classes: { "n": (), "a": (fallback: Some("b")), "b": (fallback: Some("a")) })"#);
    assert_eq!(two, StyleError::FallbackCycle("a".into()));
    let own = refuse(r#"(base_prefix: "loco.", default_class: "n", classes: { "n": (fallback: Some("n")) })"#);
    assert_eq!(own, StyleError::FallbackCycle("n".into()));
}

#[test]
fn style_refuses_exit_above_enter() {
    assert_eq!(
        refuse(r#"(base_prefix: "loco.", default_class: "n", classes: { "n": () }, rules: [(class: "n", when: [(field: Load, enter: 0.5, exit: 0.6)])])"#),
        StyleError::ExitAboveEnter { rule: 0, enter: 0.5, exit: 0.6 }
    );
}

#[test]
fn style_refuses_unknown_fields() {
    assert_eq!(
        refuse(r#"(base_prefix: "loco.", default_class: "n", classes: { "n": (prefx: "x") })"#),
        StyleError::UnknownField("prefx".into())
    );
    assert_eq!(
        refuse(r#"(base_prefix: "loco.", default_class: "n", classes: { "n": () }, extra: 1)"#),
        StyleError::UnknownField("extra".into())
    );
    assert_eq!(
        refuse(r#"(base_prefix: "loco.", default_class: "n", classes: { "n": (modifiers: (hunchh: 1.0)) })"#),
        StyleError::UnknownField("hunchh".into())
    );
}

#[test]
fn style_digest_is_stable_and_follows_the_numbers() {
    let (a, b) = (spec(), spec());
    assert_ne!(a.digest, 0);
    assert_eq!(a.digest, b.digest);
    let changed = StyleTable::parse(&SPEC.replace("enter: 0.85", "enter: 0.86")).unwrap();
    assert_ne!(changed.digest, a.digest);
    let speed = StyleTable::parse(&SPEC.replace("\"sprint\": 5.8", "\"sprint\": 5.9")).unwrap();
    assert_ne!(speed.digest, a.digest);
}

// ---------------------------------------------------------------- resolve

const CHAIN: &str = r#"(
  base_prefix: "loco.",
  default_class: "neutral",
  classes: {
    "neutral": (prefix: None),
    "a": (prefix: "loco.a.", fallback: Some("b")),
    "b": (prefix: "loco.b.", fallback: Some("neutral")),
    "c": (prefix: "loco.c.", fallback: Identity),
    "w": (prefix: "loco.w.", fallback: Some("neutral")),
  },
)"#;

fn has(slot: &str) -> bool {
    ["loco.a.walk.cycle", "loco.b.walk.cycle", "loco.b.jog.cycle", "loco.c.idle", "loco.w.walk.cycle"].contains(&slot)
}

#[test]
fn style_resolve_class_slot_present() {
    let t = StyleTable::parse(CHAIN).unwrap();
    assert_eq!(t.resolve(id(&t, "a"), "loco.walk.cycle", has).as_deref(), Some("loco.a.walk.cycle"));
    assert_eq!(t.resolve(id(&t, "c"), "loco.idle", has).as_deref(), Some("loco.c.idle"));
}

#[test]
fn style_resolve_missing_slot_takes_the_class_fallback() {
    let t = StyleTable::parse(CHAIN).unwrap();
    assert_eq!(t.resolve(id(&t, "a"), "loco.jog.cycle", has).as_deref(), Some("loco.b.jog.cycle"));
}

#[test]
fn style_resolve_chain_end_is_none() {
    let t = StyleTable::parse(CHAIN).unwrap();
    assert_eq!(t.resolve(id(&t, "a"), "loco.sprint.cycle", has), None);
    assert_eq!(t.resolve(id(&t, "neutral"), "loco.walk.cycle", has), None);
}

#[test]
fn style_resolve_identity_fallback_is_none() {
    let t = StyleTable::parse(CHAIN).unwrap();
    assert_eq!(t.resolve(id(&t, "c"), "loco.walk.cycle", has), None);
}

#[test]
fn style_resolve_walk_only_class() {
    let t = StyleTable::parse(CHAIN).unwrap();
    assert_eq!(t.resolve(id(&t, "w"), "loco.walk.cycle", has).as_deref(), Some("loco.w.walk.cycle"));
    assert_eq!(t.resolve(id(&t, "w"), "loco.jog.cycle", has), None);
}

#[test]
fn style_resolve_outside_base_prefix_is_none() {
    let t = StyleTable::parse(CHAIN).unwrap();
    assert_eq!(t.resolve(id(&t, "a"), "combat.walk.cycle", |_| true), None);
    assert_eq!(t.resolve(id(&t, "a"), "walk.cycle", |_| true), None);
}

// ---------------------------------------------------------------- profile

fn targets() -> MotionTargets {
    MotionTargets { injury: 1.0, injury_side: Side::Left, fatigue: 0.5, load: 0.8, intoxication: 0.9, mood: -0.5 }
}

fn close(a: f32, b: f32) {
    assert!((a - b).abs() < 1e-6, "{a} != {b}");
}

#[test]
fn style_step_equals_the_formula() {
    let r = SmoothRates::default();
    let dt = 1.0 / 60.0;
    let mut p = MotionProfile::default();
    p.step(&targets(), dt, &r);
    let k = |tau: f32| 1.0 - (-dt / tau).exp();
    close(p.injury, k(r.injury_rise_s));
    close(p.fatigue, 0.5 * k(r.fatigue_s));
    close(p.load, 0.8 * k(r.load_s));
    close(p.intoxication, 0.9 * k(r.intox_rise_s));
    close(p.mood, -0.5 * k(r.mood_s));
}

#[test]
fn style_two_half_steps_equal_one_step() {
    let r = SmoothRates::default();
    let (mut one, mut two) = (MotionProfile::default(), MotionProfile::default());
    one.step(&targets(), 0.1, &r);
    two.step(&targets(), 0.05, &r);
    two.step(&targets(), 0.05, &r);
    for f in [Field::Injury, Field::Fatigue, Field::Load, Field::Intoxication, Field::Mood] {
        close(one.field(f), two.field(f));
    }
}

#[test]
fn style_step_rise_and_fall_times() {
    let r = SmoothRates::default();
    let dt = 0.1;
    let k = |tau: f32| 1.0 - (-dt / tau).exp();
    let mut up = MotionProfile::default();
    up.step(&MotionTargets { injury: 1.0, intoxication: 1.0, ..MotionTargets::default() }, dt, &r);
    close(up.injury, k(r.injury_rise_s));
    close(up.intoxication, k(r.intox_rise_s));
    let mut down = MotionProfile { injury: 1.0, intoxication: 1.0, ..MotionProfile::default() };
    down.step(&MotionTargets::default(), dt, &r);
    close(down.injury, 1.0 - k(r.injury_fall_s));
    close(down.intoxication, 1.0 - k(r.intox_fall_s));
    // A limp sets in faster than it fades.
    assert!(up.injury > 1.0 - down.injury);
}

#[test]
fn style_clamps() {
    let snap_rates = SmoothRates { injury_rise_s: 0.0, injury_fall_s: 0.0, fatigue_s: 0.0, load_s: -1.0, mood_s: 0.0, intox_rise_s: 0.0, intox_fall_s: 0.0 };
    let wild = MotionTargets { injury: 2.0, injury_side: Side::Right, fatigue: f32::NAN, load: -3.0, intoxication: 1.5, mood: -3.0 };
    let mut p = MotionProfile::default();
    p.step(&wild, 1.0 / 60.0, &snap_rates); // tau <= 0 snaps
    assert_eq!((p.injury, p.fatigue, p.load, p.intoxication, p.mood), (1.0, 0.0, 0.0, 1.0, -1.0));
    let mut q = MotionProfile::default();
    q.step(&wild, 100.0, &SmoothRates::default());
    assert!([q.injury, q.fatigue, q.load, q.intoxication].iter().all(|x| (0.0..=1.0).contains(x)));
    assert!((-1.0..=1.0).contains(&q.mood));

    let loaded = MotionProfile { age: 5.0, injury: -1.0, fatigue: f32::NAN, load: 2.0, intoxication: 0.5, mood: 4.0, ..MotionProfile::default() }.clamped();
    assert_eq!((loaded.age, loaded.injury, loaded.fatigue, loaded.load, loaded.intoxication, loaded.mood), (1.0, 0.0, 0.0, 1.0, 0.5, 1.0));
    close(MotionProfile { mood: -4.0, ..MotionProfile::default() }.clamped().mood, -1.0);
}

#[test]
fn style_snap_takes_targets_and_keeps_identity_and_age() {
    let mut p = MotionProfile { style_id: ClassName::new("old"), age: 0.9, ..MotionProfile::default() };
    p.snap(&targets());
    let t = targets();
    assert_eq!((p.injury, p.injury_side, p.fatigue, p.load, p.intoxication, p.mood), (t.injury, t.injury_side, t.fatigue, t.load, t.intoxication, t.mood));
    assert_eq!((p.style_id.as_str(), p.age), ("old", 0.9));
    p.snap(&MotionTargets { injury_side: Side::Right, ..MotionTargets::default() });
    assert_eq!((p.injury, p.injury_side), (0.0, Side::Center));
}

#[test]
fn style_step_keeps_identity_and_age() {
    let mut p = MotionProfile { style_id: ClassName::new("old"), age: 0.9, ..MotionProfile::default() };
    p.step(&targets(), 0.5, &SmoothRates::default());
    assert_eq!((p.style_id.as_str(), p.age), ("old", 0.9));
}

#[test]
fn style_injury_side_rule() {
    let r = SmoothRates::default();
    let mut p = MotionProfile::default();
    p.step(&MotionTargets { injury: 0.8, injury_side: Side::Left, ..MotionTargets::default() }, 0.1, &r);
    assert_eq!(p.injury_side, Side::Left);
    // The injury decays: the limp fades out on the same leg.
    p.step(&MotionTargets::default(), 0.1, &r);
    assert!(p.injury > 0.0);
    assert_eq!(p.injury_side, Side::Left);
    // A new injury on the other leg takes its side.
    p.step(&MotionTargets { injury: 0.5, injury_side: Side::Right, ..MotionTargets::default() }, 0.1, &r);
    assert_eq!(p.injury_side, Side::Right);
}

// ---------------------------------------------------------------- chooser

const NOW: ChooseParams = ChooseParams { hold_ticks: 0, hysteresis: 0.1 };

fn state(t: &StyleTable, active: &str) -> StyleState {
    StyleState { active: id(t, active), identity: id(t, "neutral"), since_tick: 0 }
}

fn active_of(t: &StyleTable, p: &MotionProfile, prev: &str) -> String {
    let next = choose(t, p, &state(t, prev), 10, NOW);
    t.row(next.active).unwrap().name.to_string()
}

fn intox(x: f32) -> MotionProfile {
    MotionProfile { intoxication: x, ..MotionProfile::default() }
}

#[test]
fn style_choose_below_enter_stays_identity() {
    let t = spec();
    assert_eq!(active_of(&t, &intox(0.59), "neutral"), "neutral");
    assert_eq!(active_of(&t, &MotionProfile::default(), "neutral"), "neutral");
}

#[test]
fn style_choose_at_enter_switches() {
    let t = spec();
    assert_eq!(active_of(&t, &intox(0.6), "neutral"), "drunk");
    let next = choose(&t, &intox(0.6), &state(&t, "neutral"), 10, NOW);
    assert_eq!(next.since_tick, 10);
}

#[test]
fn style_choose_between_exit_and_enter_holds_only_when_active() {
    let t = spec();
    assert_eq!(active_of(&t, &intox(0.5), "drunk"), "drunk");
    assert_eq!(active_of(&t, &intox(0.45), "drunk"), "drunk");
    assert_eq!(active_of(&t, &intox(0.5), "neutral"), "neutral");
    assert_eq!(active_of(&t, &intox(0.44), "drunk"), "neutral");
    // No exit given: enter - hysteresis.
    let h = StyleTable::parse(r#"(base_prefix: "loco.", default_class: "n", classes: { "n": (), "f": () }, rules: [(class: "f", when: [(field: Fatigue, enter: 0.6)])])"#).unwrap();
    let tired = |x| MotionProfile { fatigue: x, ..MotionProfile::default() };
    let was_f = StyleState { active: id(&h, "f"), identity: id(&h, "n"), since_tick: 0 };
    let name = |s: StyleState| h.row(s.active).unwrap().name.to_string();
    assert_eq!(name(choose(&h, &tired(0.51), &was_f, 10, NOW)), "f");
    assert_eq!(name(choose(&h, &tired(0.49), &was_f, 10, NOW)), "n");
}

#[test]
fn style_choose_first_row_wins() {
    let t = spec();
    let both = MotionProfile { intoxication: 0.7, age: 0.9, ..MotionProfile::default() };
    assert_eq!(active_of(&t, &both, "neutral"), "drunk");
    assert_eq!(active_of(&t, &MotionProfile { age: 0.9, ..MotionProfile::default() }, "neutral"), "old");
    // An earlier row that enters beats a later one that holds.
    assert_eq!(active_of(&t, &both, "old"), "drunk");
}

#[test]
fn style_choose_side_rows_pick_the_leg() {
    let t = spec();
    let hurt = |side| MotionProfile { injury: 0.6, injury_side: side, ..MotionProfile::default() };
    assert_eq!(active_of(&t, &hurt(Side::Left), "neutral"), "limp_l");
    assert_eq!(active_of(&t, &hurt(Side::Right), "neutral"), "limp_r");
    assert_eq!(active_of(&t, &hurt(Side::Center), "neutral"), "neutral");
}

#[test]
fn style_choose_holds_before_hold_ticks() {
    let t = spec();
    let c = ChooseParams { hold_ticks: 60, hysteresis: 0.1 };
    let prev = t.initial(&MotionProfile::default(), 100);
    assert_eq!(prev, StyleState { active: id(&t, "neutral"), identity: id(&t, "neutral"), since_tick: 100 });
    let early = choose(&t, &intox(0.7), &prev, 159, c);
    assert_eq!(early, prev);
    let at = choose(&t, &intox(0.7), &prev, 160, c);
    assert_eq!(at, StyleState { active: id(&t, "drunk"), identity: id(&t, "neutral"), since_tick: 160 });
}

/// Switch times in seconds for a script: drunk from 3 s to 3.5 s.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss, reason = "small tick counts and rates")]
fn switch_times(rate: u32) -> Vec<(f32, String)> {
    let t = spec();
    let hold_s = crate::style::CHOOSE_HOLD_S.default_value();
    let c = ChooseParams { hold_ticks: (hold_s * rate as f32).round() as u64, hysteresis: 0.1 };
    let mut st = t.initial(&MotionProfile::default(), 0);
    let mut out = Vec::new();
    for tick in 1..=u64::from(rate) * 8 {
        let s = tick as f32 / rate as f32;
        let p = intox(if (3.0..3.5).contains(&s) { 0.7 } else { 0.0 });
        let next = choose(&t, &p, &st, tick, c);
        if next.active != st.active {
            out.push((s, t.row(next.active).unwrap().name.to_string()));
        }
        st = next;
    }
    out
}

#[test]
fn style_choose_same_switch_seconds_at_30_and_60_hz() {
    let a = switch_times(30);
    let b = switch_times(60);
    assert_eq!(a, [(3.0, "drunk".to_owned()), (5.0, "neutral".to_owned())]);
    assert_eq!(a, b);
}

#[test]
fn style_choose_unknown_style_id_falls_to_default_class() {
    let t = spec();
    let nobody = MotionProfile { style_id: ClassName::new("nobody"), ..MotionProfile::default() };
    let next = choose(&t, &nobody, &state(&t, "neutral"), 10, NOW);
    assert_eq!((next.identity, next.active), (id(&t, "neutral"), id(&t, "neutral")));
    let old = MotionProfile { style_id: ClassName::new("old"), ..MotionProfile::default() };
    // The identity updates every call, with no hold; the active class waits for it.
    let held = choose(&t, &old, &state(&t, "neutral"), 10, ChooseParams { hold_ticks: 1000, hysteresis: 0.1 });
    assert_eq!((held.identity, held.active), (id(&t, "old"), id(&t, "neutral")));
    let next = choose(&t, &old, &state(&t, "neutral"), 10, NOW);
    assert_eq!(next.active, id(&t, "old"));
    assert_eq!(t.initial(&nobody, 0).active, id(&t, "neutral"));
}

#[test]
fn style_choose_is_pure() {
    let t = spec();
    let p = MotionProfile { injury: 0.55, injury_side: Side::Left, intoxication: 0.5, age: 0.82, ..MotionProfile::default() };
    let prev = state(&t, "old");
    let c = ChooseParams { hold_ticks: 3, hysteresis: 0.1 };
    assert_eq!(choose(&t, &p, &prev, 7, c), choose(&t, &p, &prev, 7, c));
    assert_eq!(choose(&t, &p, &prev, 7, c), choose(&spec(), &p.clone(), &prev, 7, c));
    assert_eq!(t.initial(&p, 4), t.initial(&p, 4));
}

#[test]
fn style_initial_is_first_entering_row_else_identity() {
    let t = spec();
    let both = MotionProfile { intoxication: 0.7, age: 0.9, ..MotionProfile::default() };
    assert_eq!(t.initial(&both, 9), StyleState { active: id(&t, "drunk"), identity: id(&t, "neutral"), since_tick: 9 });
    // Between exit and enter: no row enters at spawn.
    assert_eq!(t.initial(&intox(0.5), 9).active, id(&t, "neutral"));
}
