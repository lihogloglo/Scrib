//! Perf panel: the pure rules (exact) and a minimal-app toggle (no panic, the node and the
//! counters follow the mode).

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;
use sf_diag_bevy::DiagCounters;

use super::{PanelText, PerfPanel, PerfPanelPlugin};
use crate::perf_panel_model::{Bound, PanelMode, Series, Snapshot, Windows, bound_by, format, gpu_frame, gpu_span, top_n};

#[test]
fn series_window_mean_max_percentile() {
    let mut s = Series::default();
    for (i, v) in [10.0, 20.0, 30.0, 40.0].into_iter().enumerate() {
        #[allow(clippy::cast_precision_loss)]
        s.push(i as f64 * 0.5, v, 1.0);
    }
    // t = 0, 0.5, 1.0, 1.5 with a 1 s window: t = 0 dropped at t = 1.5.
    assert_eq!(s.len(), 3);
    assert_eq!(s.mean(), Some(30.0));
    assert_eq!(s.max(), Some(40.0));
    assert_eq!(s.percentile(99.0), Some(40.0));
    assert_eq!(s.percentile(50.0), Some(30.0));
    assert_eq!(s.percentile(0.0), Some(20.0));
    assert_eq!(Series::default().mean(), None);
    assert_eq!(Series::default().percentile(99.0), None);
}

#[test]
fn p99_of_120_frames_is_the_second_largest() {
    let mut s = Series::default();
    for i in 0..120 {
        s.push(f64::from(i) * 0.001, f64::from(i), 10.0);
    }
    // Nearest rank: ceil(0.99 * 120) = 119th smallest = 118.
    assert_eq!(s.percentile(99.0), Some(118.0));
    assert_eq!(s.max(), Some(119.0));
}

#[test]
fn windows_smooth_by_name() {
    let mut w = Windows::default();
    w.push("main/busy/ms", 0.0, 4.0, 1.0);
    w.push("main/busy/ms", 0.1, 6.0, 1.0);
    w.push("render/busy/ms", 0.1, 3.0, 1.0);
    assert_eq!(w.mean("main/busy/ms"), Some(5.0));
    assert_eq!(w.mean("render/busy/ms"), Some(3.0));
    assert_eq!(w.mean("missing"), None);
    let mut kept: Vec<(&str, f64)> = w.means(|n| n.starts_with("main/")).collect();
    kept.sort_by(|a, b| a.0.cmp(b.0));
    assert_eq!(kept, vec![("main/busy/ms", 5.0)]);
}

#[test]
fn bound_by_picks_the_busiest_close_to_the_frame() {
    // GPU-bound: 7.9 of 8.3 ms (>= 0.85 x frame).
    assert_eq!(bound_by(Some(8.3), Some(4.0), Some(5.0), Some(7.9), 0.85), Bound::Gpu);
    assert_eq!(bound_by(Some(10.0), Some(9.5), Some(5.0), Some(7.0), 0.85), Bound::Main);
    assert_eq!(bound_by(Some(10.0), Some(5.0), Some(9.0), None, 0.85), Bound::Render);
    // Nothing near the frame: vsync, a cap, or untimed GPU.
    assert_eq!(bound_by(Some(16.7), Some(4.0), Some(5.0), Some(6.0), 0.85), Bound::Wait);
    // Ties: GPU, then render, then main.
    assert_eq!(bound_by(Some(10.0), Some(9.0), Some(9.0), Some(9.0), 0.85), Bound::Gpu);
    assert_eq!(bound_by(Some(10.0), Some(9.0), Some(9.0), None, 0.85), Bound::Render);
    assert_eq!(bound_by(None, Some(9.0), None, None, 0.85), Bound::Unknown);
    assert_eq!(bound_by(Some(10.0), None, None, None, 0.85), Bound::Unknown);
    assert_eq!(bound_by(Some(10.0), Some(f64::NAN), None, Some(9.0), 0.85), Bound::Gpu);
}

#[test]
fn top_n_largest_first_ties_by_name() {
    let items = [("b", 1.0), ("a", 3.0), ("c", 3.0), ("d", 0.5), ("nan", f64::NAN)];
    assert_eq!(top_n(items, 3), vec![("a", 3.0), ("c", 3.0), ("b", 1.0)]);
    assert_eq!(top_n(items, 10).len(), 4);
    assert!(top_n(items, 0).is_empty());
}

#[test]
fn gpu_spans_and_frame_sum_top_level_only() {
    assert_eq!(gpu_span("render/main_opaque_pass_3d/elapsed_gpu"), Some("main_opaque_pass_3d"));
    assert_eq!(gpu_span("render/main_opaque_pass_3d/elapsed_cpu"), None);
    assert_eq!(gpu_span("frame_time"), None);
    let spans = [("main_opaque_pass_3d", 2.0), ("auxiliary_views", 1.5), ("auxiliary_views/shadow", 1.0)];
    assert_eq!(gpu_frame(spans), Some(3.5));
    assert_eq!(gpu_frame([]), None);
}

#[test]
fn modes_cycle() {
    assert_eq!(PanelMode::Off.next(), PanelMode::Compact);
    assert_eq!(PanelMode::Compact.next(), PanelMode::Full);
    assert_eq!(PanelMode::Full.next(), PanelMode::Off);
}

#[test]
fn text_compact_and_full() {
    let s = Snapshot {
        frame_ms: Some(8.0),
        frame_p99: Some(11.0),
        frame_max: Some(12.5),
        main_ms: Some(3.0),
        render_ms: Some(7.5),
        gpu_ms: None,
        bound: Some(Bound::Render),
        gpu_note: Some("GPU: diag.panel.gpu_timers off"),
        tunables: vec![("diag.panel.window_s".to_owned(), "2".to_owned(), "console")],
        ..Snapshot::default()
    };
    assert_eq!(
        format(&s, false),
        "frame 8.00 ms (125 fps)  p99 11.00  max 12.50\n\
         main 3.00  render 7.50  GPU n/a  (GPU: diag.panel.gpu_timers off)\n\
         bound by: RENDER thread"
    );
    let full = format(&s, true);
    assert!(full.contains("\nGPU passes: n/a"));
    assert!(full.contains("\nviews: n/a"));
    assert!(full.contains("\nencoders: n/a"));
    assert!(full.contains("\npreset: n/a"));
    assert!(full.ends_with("\ntunables (non-default):\n  diag.panel.window_s = 2 [console]"));
}

/// `sf_diag_bevy::set_gauges_live` is process-wide: the app tests take turns.
static GAUGES_FLAG: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn panel_text(app: &mut App) -> Option<String> {
    let mut q = app.world_mut().query_filtered::<&Text, With<PanelText>>();
    q.iter(app.world()).next().map(|t| t.0.clone())
}

#[test]
fn toggling_builds_and_hides_the_panel() {
    let _one = GAUGES_FLAG.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut app = App::new();
    app.add_plugins((TaskPoolPlugin::default(), bevy_time::TimePlugin, PerfPanelPlugin));
    app.finish();
    app.cleanup();
    app.update();
    // Hidden at start: no node, the counters out of the main world, gauges off.
    assert!(panel_text(&mut app).is_none());
    assert!(!app.world().contains_resource::<DiagCounters>());
    assert!(!sf_diag_bevy::gauges_live());

    for mode in [PanelMode::Compact, PanelMode::Full] {
        app.world_mut().resource_mut::<PerfPanel>().mode = mode;
        for _ in 0..6 {
            app.update();
        }
        let text = panel_text(&mut app).expect("panel shown");
        assert!(text.starts_with(&format!("perf ({}", mode.label())), "{text}");
        assert!(app.world().contains_resource::<DiagCounters>());
        assert!(sf_diag_bevy::gauges_live());
    }

    app.world_mut().resource_mut::<PerfPanel>().mode = PanelMode::Off;
    app.update();
    app.update();
    assert!(panel_text(&mut app).is_none());
    assert!(!app.world().contains_resource::<DiagCounters>());
    assert!(!sf_diag_bevy::gauges_live());
}

#[test]
fn gauges_off_shows_frame_only() {
    let _one = GAUGES_FLAG.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut app = App::new();
    app.add_plugins((TaskPoolPlugin::default(), bevy_time::TimePlugin, PerfPanelPlugin));
    app.finish();
    app.cleanup();
    {
        let mut tunables = app.world_mut().resource_mut::<sf_tunables_bevy::Tunables>();
        let id = tunables.id("diag.panel.gauges").expect("registered");
        tunables.set_text(id, "off").expect("a bool");
    }
    app.world_mut().resource_mut::<PerfPanel>().mode = PanelMode::Compact;
    for _ in 0..4 {
        app.update();
    }
    let text = panel_text(&mut app).expect("panel shown");
    assert!(text.contains("diag.panel.gauges off"), "{text}");
    assert!(!app.world().contains_resource::<DiagCounters>());
}
