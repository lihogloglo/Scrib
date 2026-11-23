//! Features panel: the pure model (exact tabs, rows, badges, delta cells, live means) and a minimal
//! app (open, toggle at the console layer, a not-built row, tabs, reset all, close; a flip's delta
//! through the probe; the live header in place).

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_text::TextColor;
use bevy_ui::prelude::Text;
use bevy_ui_widgets::Activate;
use mw_game_core::player::PlayerMode;
use sf_tunables_bevy::sf_tunables::{Def, Lifetime, def};
use sf_tunables_bevy::{Category, Effect, FeatureSwitch, FeatureSwitches, SetBy, Tunables, TunablesAppExt, TunablesPlugin};

use super::{FeaturesPanel, FeaturesPanelPlugin, FeaturesPanelRoot, PanelAction, PanelState};
use crate::features_panel_model::{self as model, Badge, DeltaCell, LiveMeans, Row};
use crate::perf_panel::graph::FrameSample;
use crate::perf_panel::{PerfFrameFeed, PerfGaugeDemand};
use crate::switch_probe::{AbOutcome, QuickAb, SwitchProbePlugin};
use crate::switch_probe_model::{Counts, SwitchDelta};

const SHADOW: Def<bool> = def!(bool "test.shadow.enabled" = true);
const CONTACT: Def<bool> = def!(bool "test.contact.enabled" = true);
const FOG: Def<bool> = def!(bool "test.fog.enabled" = true).lifetime(Lifetime::Restart);
const CLOUDS: Def<bool> = def!(bool "test.clouds.enabled" = true);
const MODE: Def<&'static str> = def!(choice "test.mode" = "high").choices(&["off", "low", "high"]);

static SHADOW_SW: FeatureSwitch = FeatureSwitch::new(&SHADOW).label("Sun shadows").category(Category::Shadows);
static CONTACT_SW: FeatureSwitch = FeatureSwitch::new(&CONTACT)
    .label("Contact shadows")
    .category(Category::Shadows)
    .effect(Effect::Hides)
    .requires(&["test.shadow.enabled"]);
static FOG_SW: FeatureSwitch = FeatureSwitch::new(&FOG).label("Fog").category(Category::Sky);
static CLOUDS_SW: FeatureSwitch = FeatureSwitch::new(&CLOUDS).label("Clouds").category(Category::Sky);
static MODE_SW: FeatureSwitch = FeatureSwitch::choice(&MODE, "off").label("Mode").category(Category::Sky);

const A: &str = "test.shadow.enabled";
const B: &str = "test.contact.enabled";
const C: &str = "test.fog.enabled";
const D: &str = "test.clouds.enabled";
const E: &str = "test.mode";

fn switches(app: &mut App) {
    app.register_tunables(&[&SHADOW, &CONTACT, &FOG, &CLOUDS, &MODE])
        .add_feature_switch(&SHADOW_SW)
        .add_feature_switch(&CONTACT_SW)
        .add_feature_switch(&FOG_SW)
        .add_feature_switch(&CLOUDS_SW)
        .add_feature_switch(&MODE_SW)
        .mark_feature_not_built(D, "test");
}

fn row(name: &'static str, label: &'static str, on: bool, running: bool, layer: SetBy, badges: Vec<Badge>) -> Row {
    Row { name, label, on, running, layer, badges, delta: DeltaCell::default() }
}

#[test]
fn model_gives_tabs_rows_and_badges() {
    let mut app = App::new();
    app.add_plugins(TunablesPlugin);
    switches(&mut app);
    {
        let mut tunables = app.world_mut().resource_mut::<Tunables>();
        let id = tunables.id(A).expect("registered");
        tunables.set_text_by(id, "off", SetBy::Console).expect("a bool");
    }
    let world = app.world();
    let (sw, store) = (world.resource::<FeatureSwitches>(), world.resource::<Tunables>());

    let tabs = model::tabs(sw);
    assert_eq!(tabs, [Category::Shadows, Category::Sky], "only categories that have switches, panel order");
    assert_eq!(model::select_tab(&tabs, None), Some(Category::Shadows));
    assert_eq!(model::select_tab(&tabs, Some(Category::Sky)), Some(Category::Sky));
    assert_eq!(model::select_tab(&tabs, Some(Category::Water)), Some(Category::Shadows), "a gone tab falls back");
    assert_eq!(model::select_tab(&[], None), None);
    assert_eq!(model::tab_label(Category::ViewDistance), "View distance");
    assert_eq!(model::tab_label(Category::Sky), "Sky");

    assert_eq!(
        model::rows(sw, store, Category::Shadows, |_| DeltaCell::default()),
        [
            row(A, "Sun shadows", false, false, SetBy::Console, vec![]),
            row(B, "Contact shadows", true, false, SetBy::Default, vec![Badge::Hides, Badge::BlockedBy(A)]),
        ]
    );
    let sky = model::rows(sw, store, Category::Sky, |_| DeltaCell::default());
    assert_eq!(
        sky,
        [
            row(C, "Fog", true, true, SetBy::Default, vec![Badge::Restart]),
            row(D, "Clouds", true, false, SetBy::Default, vec![Badge::NotBuilt("test".to_owned())]),
            row(E, "Mode", true, true, SetBy::Default, vec![]),
        ]
    );
    assert_eq!(sky.iter().map(Row::can_toggle).collect::<Vec<_>>(), [true, false, true]);
    assert_eq!(Badge::BlockedBy(A).text(), "blocked by test.shadow.enabled");
    assert_eq!(Badge::NotBuilt("test".to_owned()).text(), "not built (test)");
    assert!(Badge::BlockedBy(A).is_warning() && !Badge::Hides.is_warning());
    assert_eq!(model::console_set(sw, store), [A]);
    assert!(model::rows(sw, store, Category::Water, |_| DeltaCell::default()).is_empty());
}

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((TaskPoolPlugin::default(), bevy_time::TimePlugin, TunablesPlugin, FeaturesPanelPlugin));
    switches(&mut app);
    app.insert_resource(PlayerMode { on: true, paused: false, body: None, last_ground: None });
    app.update();
    app
}

fn button(app: &mut App, action: PanelAction) -> Option<Entity> {
    let mut q = app.world_mut().query::<(Entity, &PanelAction)>();
    q.iter(app.world()).find(|(_, a)| **a == action).map(|(e, _)| e)
}

fn activate(app: &mut App, action: PanelAction) {
    let entity = button(app, action).unwrap_or_else(|| panic!("{action:?} shown"));
    app.world_mut().trigger(Activate { entity });
    app.update();
}

/// The toggles shown, in order.
fn toggles(app: &mut App) -> Vec<&'static str> {
    let mut q = app.world_mut().query::<&PanelAction>();
    q.iter(app.world())
        .filter_map(|a| match a {
            PanelAction::Toggle(name) => Some(*name),
            _ => None,
        })
        .collect()
}

fn toggle_text(app: &mut App, name: &'static str) -> String {
    let e = button(app, PanelAction::Toggle(name)).expect("row shown");
    app.world().get::<Text>(e).map(|t| t.0.clone()).unwrap_or_default()
}

fn value(app: &App, name: &str) -> (String, SetBy) {
    let t = app.world().resource::<Tunables>();
    let var = t.var(t.id(name).expect("registered"));
    (var.value().to_string(), var.set_by())
}

fn note(app: &mut App) -> String {
    let e = app.world().resource::<PanelState>().ui.expect("open").note;
    app.world().get::<Text>(e).map(|t| t.0.clone()).unwrap_or_default()
}

fn roots(app: &mut App) -> usize {
    let mut q = app.world_mut().query_filtered::<(), With<FeaturesPanelRoot>>();
    q.iter(app.world()).count()
}

#[test]
fn panel_toggles_at_the_console_layer_and_closes_clean() {
    let mut app = app();
    assert_eq!(roots(&mut app), 0, "closed at start: no nodes");

    app.world_mut().resource_mut::<FeaturesPanel>().open = true;
    app.update();
    assert_eq!(roots(&mut app), 1);
    assert_eq!(toggles(&mut app), [A, B], "the first tab's rows");
    assert!(button(&mut app, PanelAction::Tab(Category::Sky)).is_some(), "a Sky tab");
    assert!(button(&mut app, PanelAction::Tab(Category::Water)).is_none(), "no tab without switches");
    assert!(app.world().resource::<PlayerMode>().paused, "controls suspended, cursor free");
    assert_eq!(toggle_text(&mut app, A), "on");

    // A click flips it at the console layer; the row follows the FeatureToggled.
    activate(&mut app, PanelAction::Toggle(A));
    app.update();
    assert_eq!(value(&app, A), ("off".to_owned(), SetBy::Console));
    assert_eq!(toggle_text(&mut app, A), "off");
    assert!(note(&mut app).starts_with("test.shadow.enabled: off (console"), "{}", note(&mut app));

    activate(&mut app, PanelAction::Tab(Category::Sky));
    assert_eq!(toggles(&mut app), [C, D, E]);

    // Not built: nothing changes, the note says why.
    activate(&mut app, PanelAction::Toggle(D));
    app.update();
    assert_eq!(value(&app, D), ("on".to_owned(), SetBy::Default));
    assert!(note(&mut app).contains("not built"), "{}", note(&mut app));

    activate(&mut app, PanelAction::Toggle(C));
    activate(&mut app, PanelAction::Toggle(E));
    app.update();
    assert_eq!(value(&app, C), ("off".to_owned(), SetBy::Console));
    assert_eq!(value(&app, E), ("off".to_owned(), SetBy::Console));

    // Reset all peels the console layer: the defaults come back.
    activate(&mut app, PanelAction::ResetAll);
    app.update();
    for name in [A, C] {
        assert_eq!(value(&app, name), ("on".to_owned(), SetBy::Default), "{name}");
    }
    assert_eq!(value(&app, E), ("high".to_owned(), SetBy::Default));
    assert_eq!(toggle_text(&mut app, C), "on");

    activate(&mut app, PanelAction::Close);
    assert_eq!(roots(&mut app), 0);
    assert!(toggles(&mut app).is_empty(), "the tree is gone");
    assert!(!app.world().resource::<PlayerMode>().paused, "gameplay input back as it was");
    assert!(!app.world().resource::<FeaturesPanel>().open);
}

fn delta(frame: Option<f32>, render: Option<f32>, gpu: Option<f32>) -> SwitchDelta {
    SwitchDelta {
        running: false,
        frame_ms: frame,
        main_ms: frame,
        render_ms: render,
        gpu_ms: gpu,
        before: Counts::default(),
        after: Counts::default(),
        noise: false,
        moved: false,
        superseded: false,
    }
}

fn cell(value: &str, detail: &str, flag: &'static str, noise: bool) -> DeltaCell {
    DeltaCell { value: value.to_owned(), detail: detail.to_owned(), flag, noise }
}

#[test]
fn model_formats_flip_deltas_and_live_means() {
    let normal = delta(Some(-1.34), Some(-0.9), Some(-0.4));
    assert_eq!(model::delta_cell(Some(&normal), false), cell("-1.3 ms", "r -0.9 g -0.4", "", false));
    assert_eq!(model::delta_cell(Some(&normal), false).small(), "r -0.9 g -0.4");
    assert_eq!(
        model::delta_cell(Some(&delta(Some(2.06), None, None)), false),
        cell("+2.1 ms", "", "", false),
        "GPU timers off: no detail"
    );
    assert_eq!(model::delta_cell(Some(&delta(Some(-0.04), Some(0.3), None)), false), cell("+0.0 ms", "r +0.3", "", false), "never -0.0");
    let noise = SwitchDelta { noise: true, ..delta(Some(0.2), Some(0.1), Some(-0.1)) };
    assert_eq!(model::delta_cell(Some(&noise), false), cell("+0.2 ms", "r +0.1 g -0.1", "", true));
    let moved = SwitchDelta { moved: true, ..normal };
    let moved = model::delta_cell(Some(&moved), false);
    assert_eq!(moved, cell("-1.3 ms", "r -0.9 g -0.4", "moved", false));
    assert_eq!(moved.small(), "r -0.9 g -0.4 moved");
    let superseded = SwitchDelta { superseded: true, ..delta(None, None, None) };
    let superseded = model::delta_cell(Some(&superseded), false);
    assert_eq!(superseded, cell("", "", "superseded", false));
    assert_eq!(superseded.small(), "superseded");
    assert_eq!(model::delta_cell(Some(&normal), true), cell("", "", "measuring...", false), "measuring hides the old result");
    assert_eq!(model::delta_cell(None, true), cell("", "", "measuring...", false));
    assert_eq!(model::delta_cell(None, false), DeltaCell::default(), "never measured");
    assert_eq!(model::delta_cell(Some(&delta(None, None, None)), false), DeltaCell::default(), "no before window");
    assert_eq!(model::DELTA_HEADING, "frame change when flipped");

    let mut live = LiveMeans::default();
    assert!(!live.due(0.0), "no frame yet");
    assert_eq!(live.text(), "ms: frame n/a  main n/a  render n/a  gpu n/a");
    live.push(100.0, Some(6.0), Some(4.0), None);
    assert!(!live.due(0.25));
    live.push(200.0, Some(8.0), None, None);
    assert!(live.due(0.25), "0.3 s of frames");
    assert_eq!(live.text(), "ms: frame 150.0  main 7.0  render 4.0  gpu n/a");
}

/// The test app with the probe and the F3 feed (no F3 panel: samples are injected).
fn probe_app() -> App {
    let mut app = App::new();
    app.add_plugins((TaskPoolPlugin::default(), bevy_time::TimePlugin, TunablesPlugin, FeaturesPanelPlugin, SwitchProbePlugin))
        .init_resource::<PerfFrameFeed>()
        .init_resource::<PerfGaugeDemand>();
    switches(&mut app);
    app.insert_resource(PlayerMode { on: true, paused: false, body: None, last_ground: None });
    app.update();
    app
}

fn feed(app: &mut App, sample: FrameSample) {
    app.world_mut().resource_mut::<PerfFrameFeed>().0 = Some(sample);
}

/// `n` frames of `ms` (main 1 ms, render 2 ms, GPU 3 ms under it), each fed before its update.
fn frames(app: &mut App, ms: f32, n: usize) {
    for _ in 0..n {
        feed(app, FrameSample { frame_ms: ms, main_ms: Some(ms - 1.0), render_ms: Some(ms - 2.0), gpu_ms: Some(ms - 3.0), ..Default::default() });
        app.update();
    }
}

/// Switch `name`'s delta texts: value and smaller text.
fn delta_texts(app: &App, name: &str) -> (String, String) {
    let state = app.world().resource::<PanelState>();
    let at = state.cells.iter().find(|c| c.name == name).copied().expect("row shown");
    let text = |e| app.world().get::<Text>(e).map(|t| t.0.clone()).expect("a text");
    (text(at.value), text(at.small))
}

fn live_text(app: &App) -> String {
    let e = app.world().resource::<PanelState>().ui.expect("open").live;
    app.world().get::<Text>(e).map(|t| t.0.clone()).unwrap_or_default()
}

#[test]
fn a_flip_shows_its_delta_after_the_window() {
    let mut app = probe_app();
    app.world_mut().resource_mut::<FeaturesPanel>().open = true;
    frames(&mut app, 10.0, 250);
    assert_eq!(delta_texts(&app, A), (String::new(), String::new()), "never measured: blank");

    let entity = button(&mut app, PanelAction::Toggle(A)).expect("row shown");
    feed(&mut app, FrameSample { frame_ms: 10.0, main_ms: Some(9.0), render_ms: Some(8.0), gpu_ms: Some(7.0), ..Default::default() });
    app.world_mut().trigger(Activate { entity });
    app.update();
    frames(&mut app, 10.0, 1);
    assert_eq!(value(&app, A), ("off".to_owned(), SetBy::Console));
    assert_eq!(delta_texts(&app, A).1, "measuring...");

    // 0.5 s settle at 10 ms, then 2 s of cheaper frames.
    frames(&mut app, 10.0, 49);
    frames(&mut app, 8.5, 250);
    assert!(!app.world().resource::<crate::switch_probe::SwitchProbe>().measuring());
    assert_eq!(delta_texts(&app, A), ("-1.5 ms".to_owned(), "r -1.5 g -1.5".to_owned()));
    let state = app.world().resource::<PanelState>();
    let at = state.cells.iter().find(|c| c.name == A).copied().expect("row shown");
    let colour = app.world().get::<TextColor>(at.value).expect("a colour").0;
    assert_ne!(colour, app.world().get::<TextColor>(at.small).expect("a colour").0, "over the noise floor: not greyed");
}

#[test]
fn header_shows_live_ms_in_place() {
    let mut app = probe_app();
    app.world_mut().resource_mut::<FeaturesPanel>().open = true;
    app.update();
    let rows_before = toggles(&mut app);
    let buttons_before = button(&mut app, PanelAction::Toggle(A));
    let header = |ms: f32, gpu: Option<f32>| FrameSample { frame_ms: ms, main_ms: Some(6.0), render_ms: Some(4.0), gpu_ms: gpu, ..Default::default() };
    for _ in 0..30 {
        feed(&mut app, header(10.0, None));
        app.update();
    }
    assert_eq!(live_text(&app), "ms: frame 10.0  main 6.0  render 4.0  gpu n/a", "GPU timers off");
    for _ in 0..30 {
        feed(&mut app, header(20.0, Some(7.0)));
        app.update();
    }
    assert_eq!(live_text(&app), "ms: frame 20.0  main 6.0  render 4.0  gpu 7.0");
    assert_eq!(toggles(&mut app), rows_before);
    assert_eq!(button(&mut app, PanelAction::Toggle(A)), buttons_before, "rows not respawned by the header");
}

fn ab_text(app: &mut App, name: &'static str) -> String {
    let e = button(app, PanelAction::Ab(name)).expect("A/B button shown");
    app.world().get::<Text>(e).map(|t| t.0.clone()).unwrap_or_default()
}

#[test]
fn model_formats_the_ab_button() {
    use crate::switch_probe_model::{AbAbort, AbCost, AbOutcome};
    let cost = AbCost {
        frame_ms: 1.44,
        main_ms: Some(0.3),
        render_ms: Some(0.9),
        gpu_ms: None,
        min_ms: 1.06,
        max_ms: 1.7,
        rounds: 3,
        noise: false,
    };
    let cell = |running, outcome: Option<&AbOutcome>, can| model::ab_cell(A, running, outcome, can);
    assert_eq!(cell(None, None, true), model::AbCell { text: "A/B".to_owned(), enabled: true, noise: false });
    assert_eq!(cell(Some((A, (2, 6))), None, true).text, "A/B 2/6...");
    assert!(!cell(Some((B, (2, 6))), None, true).enabled, "one A/B at a time");
    assert!(!cell(None, None, false).enabled, "not a live row");
    let done = AbOutcome::Done(cost);
    assert_eq!(cell(None, Some(&done), true).text, "cost 1.4 ms (r 0.9, 1.1..1.7)");
    let quiet = AbOutcome::Done(AbCost { frame_ms: -0.04, render_ms: None, noise: true, ..cost });
    assert_eq!(cell(None, Some(&quiet), true), model::AbCell { text: "cost 0.0 ms (1.1..1.7)".to_owned(), enabled: true, noise: true });
    assert_eq!(cell(None, Some(&AbOutcome::Aborted(AbAbort::CameraMoving)), true).text, "A/B: camera moving");
}

#[test]
fn quick_ab_reads_the_cost_and_restores_the_switch() {
    let mut app = probe_app();
    app.world_mut().resource_mut::<FeaturesPanel>().open = true;
    app.update();
    // Restart rows have no A/B.
    activate(&mut app, PanelAction::Tab(Category::Sky));
    activate(&mut app, PanelAction::Ab(C));
    assert!(note(&mut app).contains("not live"), "{}", note(&mut app));
    assert!(app.world().resource::<QuickAb>().running().is_none());
    activate(&mut app, PanelAction::Tab(Category::Shadows));

    // The feature runs: 11.5 ms frames (every click below comes with its frame, so no bare
    // real-time frame lands in a window).
    frames(&mut app, 11.5, 250);
    let click = |app: &mut App, action| {
        feed(app, FrameSample { frame_ms: 11.5, main_ms: Some(10.5), render_ms: Some(9.5), gpu_ms: Some(8.5), ..Default::default() });
        activate(app, action);
    };
    click(&mut app, PanelAction::Ab(A));
    assert_eq!(app.world().resource::<QuickAb>().running(), Some((A, (0, 6))));
    assert!(ab_text(&mut app, A).starts_with("A/B 0/6"), "{}", ab_text(&mut app, A));
    // Nothing else flips while it runs.
    click(&mut app, PanelAction::Toggle(B));
    assert!(note(&mut app).contains("runs"), "{}", note(&mut app));

    // Frames are 1.5 ms slower while the feature runs (main, render, GPU alike).
    let mut flips = 0;
    let mut was = true;
    for _ in 0..5000 {
        if app.world().resource::<QuickAb>().running().is_none() {
            break;
        }
        let on = {
            let world = app.world();
            world.resource::<FeatureSwitches>().running(world.resource::<Tunables>(), A)
        };
        flips += usize::from(on != was);
        was = on;
        frames(&mut app, if on { 11.5 } else { 10.0 }, 1);
    }
    app.update();
    let ab = app.world().resource::<QuickAb>();
    assert!(ab.running().is_none(), "finished");
    let Some(AbOutcome::Done(c)) = ab.get(A) else { panic!("a cost: {:?}", ab.get(A)) };
    assert!((c.frame_ms - 1.5).abs() < 0.01, "{c:?}");
    for v in [c.main_ms, c.render_ms, c.gpu_ms] {
        assert!((v.expect("measured") - 1.5).abs() < 0.01, "{c:?}");
    }
    assert_eq!((c.rounds, c.noise), (3, false));
    assert_eq!(flips, 6, "three off/on pairs");
    assert_eq!(value(&app, A), ("on".to_owned(), SetBy::Default), "back as it was: the console layer peeled");
    assert_eq!(ab_text(&mut app, A), "cost 1.5 ms (r 1.5, 1.5..1.5)");
}

#[test]
fn closing_the_panel_stops_the_ab_and_restores() {
    let mut app = probe_app();
    {
        let mut tunables = app.world_mut().resource_mut::<Tunables>();
        let id = tunables.id(A).expect("registered");
        tunables.set_text_by(id, "on", SetBy::Console).expect("a bool");
    }
    app.world_mut().resource_mut::<FeaturesPanel>().open = true;
    frames(&mut app, 10.0, 250);
    activate(&mut app, PanelAction::Ab(A));
    // Into the first flip's measurement: the switch is off.
    frames(&mut app, 10.0, 20);
    assert_eq!(value(&app, A), ("off".to_owned(), SetBy::Console));
    activate(&mut app, PanelAction::Close);
    frames(&mut app, 10.0, 3);
    let ab = app.world().resource::<QuickAb>();
    assert!(ab.running().is_none());
    assert_eq!(ab.get(A), Some(&AbOutcome::Aborted(crate::switch_probe::AbAbort::Closed)));
    assert_eq!(value(&app, A), ("on".to_owned(), SetBy::Console), "its console value written back");
}
