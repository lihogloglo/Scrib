//! The reveal gate (pipeline-warmup.md §5.4, W1): exact, GPU-free. The render world's terms
//! are fed as [`PendingSample`]s, as `sf_diag_bevy::PendingDrawsPlugin` would store them.

#![allow(clippy::float_cmp)] // exact: sums of DT (1/8 s) are exact in binary

use bevy::prelude::*;
use sf_diag_bevy::{PendingDraws, PendingSample, Reveal, RevealProbe};

use super::*;

/// Exact in binary: sums of it are exact, so the asserted seconds are too.
const DT: f32 = 0.125;

fn ready() -> RevealInputs {
    RevealInputs { cells: true, textures: true, ..RevealInputs::default() }
}

/// Steps `gate` with `inputs` until it opens (at most `frames`); the frame count and the reveal.
fn run(gate: &mut RevealGate, inputs: &RevealInputs, frames: u32) -> Option<(u32, Reveal, Option<String>)> {
    (1..=frames).find_map(|n| match gate.step("door", "Census", DT, inputs) {
        RevealStep::Holding => None,
        RevealStep::Open { reveal, warning } => Some((n, reveal, warning)),
    })
}

fn terms(r: &Reveal) -> Vec<(&'static str, f32)> {
    r.terms.iter().map(|t| (t.name, t.secs.unwrap_or(f32::NAN))).collect()
}

/// Each pending term on its own keeps the gate closed: a near mesh whose material (or its
/// textures) is unprepared, a visible mesh with no pipeline yet, a main-pass draw whose pipeline
/// is queued, a pipeline compiling, and a shadow-view-only pending pipeline.
#[test]
fn each_pending_term_keeps_the_gate_closed() {
    let cases = [
        ("near material unprepared", PendingSample { near_unprepared: 1, ..PendingSample::default() }),
        ("visible, no pipeline yet", PendingSample { unprepared: 1, ..PendingSample::default() }),
        ("main-pass pipeline queued", PendingSample { skipped: 1, waiting: 1, ..PendingSample::default() }),
        ("pipeline compiling", PendingSample { waiting: 1, ..PendingSample::default() }),
        ("shadow view only", PendingSample { skipped_shadow: 1, ..PendingSample::default() }),
    ];
    for (what, sample) in cases {
        let mut gate = RevealGate::default();
        let pending = RevealInputs { sample, ..ready() };
        assert!(run(&mut gate, &pending, 50).is_none(), "{what}: the gate stays closed");
        assert!(gate.is_holding(), "{what}");
        assert!(gate.cpu_ready(), "{what}: cells and textures are in");
    }
}

/// Opens once every term has stayed clear for `SETTLE_FRAMES`; the hold's time goes to the first
/// term still pending each frame (cells, textures, assets, pipelines, then gates/settling).
#[test]
fn opens_when_all_terms_clear_and_attributes_the_hold() {
    let mut gate = RevealGate::default();
    let not_yet = RevealInputs::default();
    assert!(run(&mut gate, &not_yet, 2).is_none()); // cells 0.25 s
    let cells = RevealInputs { cells: true, ..RevealInputs::default() };
    assert!(run(&mut gate, &cells, 1).is_none()); // textures 0.125 s
    let assets = RevealInputs { sample: PendingSample { near_unprepared: 2, waiting: 1, ..PendingSample::default() }, ..ready() };
    assert!(run(&mut gate, &assets, 3).is_none()); // assets 0.375 s
    let pipelines = RevealInputs { sample: PendingSample { skipped_shadow: 1, ..PendingSample::default() }, ..ready() };
    assert!(run(&mut gate, &pipelines, 4).is_none()); // pipelines 0.5 s
    let (n, reveal, warning) = run(&mut gate, &ready(), 10).expect("opens");
    assert_eq!(n, SETTLE_FRAMES, "clear for SETTLE_FRAMES frames");
    assert_eq!((reveal.kind, reveal.place.as_str(), reveal.capped, warning), ("door", "Census", None, None));
    assert_eq!(reveal.held_secs, 1.625);
    assert_eq!(terms(&reveal), [("cells", 0.25), ("textures", 0.125), ("assets", 0.375), ("pipelines", 0.5), ("gates", 0.375)]);
    assert!(!gate.is_holding());
    assert_eq!(gate.last_reveal(), Some(&reveal));
}

/// A term that comes back resets the settle count: the reveal waits for a clear run.
#[test]
fn a_late_pending_term_restarts_the_settle() {
    let mut gate = RevealGate::default();
    assert!(run(&mut gate, &ready(), SETTLE_FRAMES - 1).is_none());
    let late = RevealInputs { sample: PendingSample { near_unprepared: 1, ..PendingSample::default() }, ..ready() };
    assert!(run(&mut gate, &late, 1).is_none());
    let (n, _, _) = run(&mut gate, &ready(), 10).expect("opens");
    assert_eq!(n, SETTLE_FRAMES);
}

/// The cap (10 s after cells and textures) opens a gate that never clears, with a warning that
/// names the pending terms and counts.
#[test]
fn the_cap_opens_with_a_warning_naming_what_is_pending() {
    let mut gate = RevealGate::default();
    assert!((gate.max_secs - 10.0).abs() < f32::EPSILON, "owner 2026-11-11: 10 s");
    // Cells take 0.25 s: the cap counts from the CPU terms, not from the start of the hold.
    assert!(run(&mut gate, &RevealInputs::default(), 2).is_none());
    let stuck = PendingSample { near_unprepared: 2, unprepared: 1, skipped: 1, skipped_shadow: 3, waiting: 4, ..PendingSample::default() };
    let (n, reveal, warning) = run(&mut gate, &RevealInputs { sample: stuck, ..ready() }, 200).expect("capped");
    assert_eq!(n, 81, "0 s at the first CPU-ready frame, 10 s eighty frames later");
    assert_eq!((reveal.capped, reveal.held_secs), (Some("reveal_cap"), 10.375));
    assert_eq!(
        warning.as_deref(),
        Some("not ready after 10.4 s (reveal_cap): pending assets 3 (near 2, visible 1), pipelines 8 (draws 1, shadow/prepass 3, compiling 4)")
    );
}

/// The client's own cap (the fade's `max_hold_secs`) opens at once, naming the CPU terms too.
#[test]
fn giving_up_opens_with_max_hold() {
    let mut gate = RevealGate::default();
    assert!(run(&mut gate, &RevealInputs::default(), 6).is_none());
    let give_up = RevealInputs { give_up: true, ..RevealInputs::default() };
    let (_, reveal, warning) = run(&mut gate, &give_up, 1).expect("gave up");
    assert_eq!((reveal.capped, reveal.held_secs), (Some("max_hold"), 0.875));
    assert_eq!(warning.as_deref(), Some("not ready after 0.9 s (max_hold): pending cells, textures"));
}

/// External gates (shadow captures, ...) hold for at most `gates_max_secs` after cells and
/// textures, as the fade's gates did before W1.
#[test]
fn external_gates_hold_up_to_their_cap() {
    let mut gate = RevealGate::default();
    let gated = RevealInputs { gated: true, ..ready() };
    let (n, reveal, warning) = run(&mut gate, &gated, 100).expect("gate capped");
    assert_eq!(n, 13, "the gate is honoured until 1.5 s after the CPU terms (frame 1 is 0 s)");
    assert_eq!(reveal.capped, Some("gate"));
    assert!(warning.is_some_and(|w| w.contains("held past 1.5 s")));
}

/// Progress: a quarter for cells, a quarter for textures, then the share of the peak assets and
/// pipelines done; never backwards.
#[test]
fn progress_counts_terms_and_never_goes_back() {
    let mut gate = RevealGate::default();
    assert_eq!(gate.progress(), None);
    let _ = gate.step("loading", "x", DT, &RevealInputs::default());
    assert_eq!(gate.progress(), Some(0.0));
    let _ = gate.step("loading", "x", DT, &RevealInputs { cells: true, ..RevealInputs::default() });
    assert_eq!(gate.progress(), Some(0.25));
    let busy = |assets, waiting| RevealInputs {
        sample: PendingSample { near_unprepared: assets, waiting, ..PendingSample::default() },
        ..ready()
    };
    let _ = gate.step("loading", "x", DT, &busy(4, 4));
    assert_eq!(gate.progress(), Some(0.5));
    let _ = gate.step("loading", "x", DT, &busy(2, 0));
    assert_eq!(gate.progress(), Some(0.875));
    let _ = gate.step("loading", "x", DT, &busy(4, 4));
    assert_eq!(gate.progress(), Some(0.875), "never backwards");
}

#[derive(Resource, Default)]
struct Statuses(Vec<RevealStatus>);

#[derive(Resource)]
struct Ask(bool);

fn client(mut reveal: RevealCheck, ask: Res<Ask>, mut seen: ResMut<Statuses>) {
    if ask.0 {
        let status = reveal.step(RevealAsk { kind: "loading", place: "Seyda Neen", focus: None, cells: true, gated: false, give_up: false });
        seen.0.push(status);
    }
}

fn gate_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<RevealGate>()
        .init_resource::<PendingDraws>()
        .init_resource::<Statuses>()
        .insert_resource(Ask(true))
        .add_message::<Reveal>()
        .add_systems(Update, client)
        .add_systems(Last, end_reveal_frame);
    app.world_mut().spawn((Camera3d::default(), Transform::from_xyz(1.0, 2.0, 3.0), GlobalTransform::from_xyz(1.0, 2.0, 3.0)));
    app
}

/// A client (the app's `Loading` → `InGame`, as `mw_exterior_bevy` `phase.rs` drives it) waits
/// while the render world reports a near material unprepared, with the probe set on its focus
/// (the active 3D camera); once it clears, the gate opens, writes the `Reveal` and clears the probe.
#[test]
fn a_client_waits_on_the_render_world_and_the_probe_follows_the_hold() {
    let mut app = gate_app();
    let pending = app.world().resource::<PendingDraws>().clone();
    pending.store(PendingSample { near_unprepared: 1, ..PendingSample::default() });
    for _ in 0..10 {
        app.update();
    }
    assert!(app.world().resource::<Statuses>().0.iter().all(|s| !s.open && s.cpu_ready));
    assert_eq!(pending.probe(), Some(RevealProbe { center: [1.0, 2.0, 3.0], radius: 48.0 }));
    assert!(app.world().resource::<RevealGate>().is_holding());

    pending.store(PendingSample::default());
    for _ in 0..SETTLE_FRAMES {
        app.update();
    }
    let opened = app.world().resource::<Statuses>().0.iter().filter(|s| s.open).count();
    assert_eq!(opened, 1, "opens once, after SETTLE_FRAMES clear frames");
    assert_eq!(pending.probe(), None);
    let reveals: Vec<Reveal> = app.world_mut().resource_mut::<Messages<Reveal>>().drain().collect();
    assert_eq!(reveals.len(), 1);
    assert_eq!((reveals[0].kind, reveals[0].place.as_str(), reveals[0].capped), ("loading", "Seyda Neen", None));
}

/// A hold no client steps any more (its cover ended another way) is dropped, and the probe cleared.
#[test]
fn an_abandoned_hold_is_dropped() {
    let mut app = gate_app();
    let pending = app.world().resource::<PendingDraws>().clone();
    pending.store(PendingSample { waiting: 1, ..PendingSample::default() });
    app.update();
    assert!(pending.probe().is_some());
    app.world_mut().resource_mut::<Ask>().0 = false;
    app.update();
    assert!(!app.world().resource::<RevealGate>().is_holding());
    assert_eq!(pending.probe(), None);
}

/// Textures that never load (neither in `Assets<Image>` nor failed) are capped too: the cap's
/// clock starts at the cells, so a `Loading` (no client cap) can never hang on them.
#[test]
fn textures_that_never_load_are_capped() {
    let mut gate = RevealGate::default();
    assert!(run(&mut gate, &RevealInputs::default(), 2).is_none());
    let no_textures = RevealInputs { cells: true, ..RevealInputs::default() };
    let (n, reveal, warning) = run(&mut gate, &no_textures, 200).expect("capped");
    assert_eq!((n, reveal.capped), (81, Some("reveal_cap")));
    assert_eq!(warning.as_deref(), Some("not ready after 10.4 s (reveal_cap): pending textures"));
}
