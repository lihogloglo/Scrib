//! Warm-up measurement arithmetic (pipeline-warmup.md W0): exact, GPU-free.

use bevy_app::App;
use bevy_ecs::message::Messages;

use crate::warmup::{
    AFTER_REVEAL_SECS, AfterReveal, CompileStats, PendingDraws, PendingSample, Reveal, RevealProbe, RevealTerm,
    RevealWatchPlugin,
    describe_key_change,
};
use crate::{DiagCounters, GpuPending};

fn sample(created: usize, skipped: usize, visible: usize, unprepared: usize) -> PendingSample {
    PendingSample { skipped, visible, unprepared, created, ..PendingSample::default() }
}

#[test]
fn pending_draws_round_trip() {
    let p = PendingDraws::default();
    let s = PendingSample { skipped: 3, visible: 40, unprepared: 2, assets: 5, waiting: 7, created: 90, skipped_shadow: 4, near_unprepared: 1 };
    p.clone().store(s);
    assert_eq!(p.sample(), s, "clones share one Arc (render world writes, main world reads)");
    assert_eq!(p.get(), 3);
    assert_eq!(p.gpu(), GpuPending { pipelines: 7, draws: 3 });
    let probe = RevealProbe { center: [1.0, 2.0, 3.0], radius: 48.0 };
    p.clone().set_probe(Some(probe));
    assert_eq!(p.probe(), Some(probe), "the main world sets the probe, the render world reads it");
    p.set_probe(None);
    assert_eq!(p.probe(), None);
}

#[test]
fn after_reveal_counts_only_inside_the_window() {
    let mut w = AfterReveal::default();
    // Before any reveal: nothing counts, but the created baseline is kept.
    assert_eq!(w.frame(0.0, &sample(100, 9, 50, 0)).pipelines, 0);
    assert!(w.reveal(1.0, "door", "Census").is_none());
    let f = w.frame(1.0, &sample(104, 6, 60, 2));
    assert_eq!((f.pipelines, f.skipped), (4, 6), "pipelines since the last frame, draws this frame");
    let f = w.frame(5.0, &sample(105, 0, 60, 5));
    assert_eq!((f.pipelines, f.skipped), (1, 0));
    let f = w.frame(1.0 + AFTER_REVEAL_SECS, &sample(107, 3, 80, 1));
    assert_eq!((f.pipelines, f.skipped), (2, 3), "the window's last instant still counts");
    // Past the window: closes with the totals, this frame not counted.
    let f = w.frame(1.0 + AFTER_REVEAL_SECS + 0.1, &sample(120, 50, 80, 0));
    assert_eq!((f.pipelines, f.skipped), (0, 0));
    let s = f.closed.expect("closed");
    assert_eq!((s.kind, s.place.as_str()), ("door", "Census"));
    assert_eq!((s.pipelines, s.skipped, s.skipped_frames, s.skipped_peak), (7, 9, 2, 6));
    assert_eq!((s.visible, s.unprepared_peak), (200, 5));
    assert!((s.skipped_percent() - 4.5).abs() < 1e-9);
    assert!((s.secs - (AFTER_REVEAL_SECS + 0.1)).abs() < 1e-9);
    assert!(!w.is_open());
    assert_eq!(w.frame(20.0, &sample(130, 1, 1, 0)), crate::warmup::LateFrame::default());
}

#[test]
fn a_new_reveal_cuts_the_open_window() {
    let mut w = AfterReveal::default();
    w.reveal(0.0, "loading", "Seyda Neen");
    w.frame(0.5, &sample(10, 1, 10, 0));
    w.frame(1.0, &sample(13, 0, 10, 0));
    let cut = w.reveal(3.0, "door", "Census").expect("cut short");
    assert_eq!((cut.kind, cut.pipelines, cut.skipped), ("loading", 3, 1));
    assert!((cut.secs - 3.0).abs() < 1e-9);
    assert_eq!(w.frame(3.0, &sample(15, 0, 10, 0)).pipelines, 2, "the new window counts from its reveal");
}

#[test]
fn reveal_line_names_terms_caps_and_gpu_state() {
    let r = Reveal {
        kind: "door",
        place: "Census".into(),
        held_secs: 1.234,
        terms: vec![
            RevealTerm { name: "cells", secs: Some(0.4) },
            RevealTerm { name: "assets", secs: None },
        ],
        capped: Some("gate"),
    };
    let at = PendingSample { skipped: 2, unprepared: 1, assets: 4, waiting: 3, skipped_shadow: 5, near_unprepared: 1, ..PendingSample::default() };
    assert_eq!(
        r.line(&at),
        "reveal: door held 1.23 s (cells 0.40 s, assets n/a), capped: gate, \"Census\"; at reveal: 3 pipelines \
         compiling, 2 visible draws skipped (5 shadow/prepass), 1 meshes unprepared (1 near), 4 assets pending"
    );
}

#[test]
fn compile_histogram_buckets_and_percentiles() {
    let mut c = CompileStats::default();
    c.record("a", 0.5, 0.2, 0.3); // 0.5 ms -> <1
    c.record("b", 0.0, 1.0, 2.0); // 3 ms -> <4
    c.record("c", 1.0, 10.0, 20.0); // 30 ms -> <32
    c.record("d", 0.0, 100.0, 600.0); // 700 ms -> >=512
    assert_eq!(c.count, 4);
    assert_eq!(c.buckets, [1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1]);
    assert_eq!(c.slowest, Some(("d".to_owned(), 700.0)));
    assert!((c.front_ms - 111.2).abs() < 1e-9 && (c.driver_ms - 622.3).abs() < 1e-9 && (c.wait_ms - 1.5).abs() < 1e-9);
    assert!((c.percentile_ms(0.5) - 4.0).abs() < f64::EPSILON);
    assert!((c.percentile_ms(0.75) - 32.0).abs() < f64::EPSILON);
    assert!((c.percentile_ms(1.0) - 700.0).abs() < f64::EPSILON, "open bucket: the slowest");
    assert!(CompileStats::default().percentile_ms(0.5).abs() < f64::EPSILON);
    assert_eq!(
        c.line(),
        "compiled 4 (front 0.11 s, driver 0.62 s, waiting 0.00 s; p50 <4 ms, p95 <700 ms, slowest 700 ms \"d\"; \
         ms <1:1 <4:1 <32:1 >=512:1)"
    );
}

#[test]
fn view_key_change_names_flags_fields_and_rest() {
    let flags = [("DEPTH_PREPASS", 1 << 2), ("SSR", 1 << 16), ("IN_FIELD", 1 << 30)];
    let fields = [("TONEMAP", 0b1111 << 28)];
    let old = (1 << 2) | (6 << 28);
    let new = (1 << 16) | (4 << 28) | (1 << 40);
    assert_eq!(describe_key_change(old, new, &flags, &fields), "TONEMAP 6->4 -DEPTH_PREPASS +SSR bits 0x10000000000");
    assert_eq!(describe_key_change(old, old, &flags, &fields), "none");
}

#[test]
fn reveal_watch_counts_into_diag_counters() {
    let mut app = App::new();
    app.add_plugins(bevy_time::TimePlugin).init_resource::<DiagCounters>().add_plugins(RevealWatchPlugin);
    let pending = app.world().resource::<PendingDraws>().clone();
    pending.store(sample(10, 0, 0, 0));
    app.update();
    app.world_mut().resource_mut::<Messages<Reveal>>().write(Reveal {
        kind: "door",
        place: "x".into(),
        held_secs: 0.5,
        terms: Vec::new(),
        capped: None,
    });
    pending.store(sample(13, 4, 20, 0));
    app.update();
    let counters = app.world().resource::<DiagCounters>();
    let mut seen = Vec::new();
    counters.drain(|_, def, v| seen.push((def.name.clone(), v)));
    seen.sort_by(|a, b| a.0.cmp(&b.0));
    assert_eq!(seen, [("draws/skipped_late".to_owned(), Some(4.0)), ("pipelines/after_reveal".to_owned(), Some(3.0))]);
}
