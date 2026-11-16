//! Perf panel graphs: the pure rules, exact (history ring, per-frame bound, column mapping with
//! spikes kept, bar scaling, pixels).

use super::{Column, FrameSample, GraphSpec, History, Palette, bar_fraction, bar_rows, column_slots, columns, dim, draw_graph, draw_sparkline, rgba, series_columns};
use crate::perf_panel_model::Bound;

fn frame(ms: f32) -> FrameSample {
    FrameSample { frame_ms: ms, ..FrameSample::default() }
}

#[test]
fn history_is_a_ring_of_the_last_frames() {
    let mut h = History::with_capacity(3);
    for ms in [1.0, 2.0, 3.0, 4.0] {
        h.push(frame(ms));
    }
    assert_eq!(h.iter().map(|s| s.frame_ms).collect::<Vec<_>>(), vec![2.0, 3.0, 4.0]);
    assert!((h.span_s() - 0.009).abs() < 1e-6);
    h.set_capacity(2);
    assert_eq!(h.iter().map(|s| s.frame_ms).collect::<Vec<_>>(), vec![3.0, 4.0]);
    h.set_capacity(5);
    h.push(frame(5.0));
    assert_eq!(h.len(), 3);
    h.clear();
    assert!(h.is_empty());
    let mut none = History::default();
    none.push(frame(1.0));
    assert!(none.is_empty(), "capacity 0 keeps nothing");
}

#[test]
fn each_frame_gets_its_own_bound() {
    let s = FrameSample { frame_ms: 10.0, main_ms: Some(9.5), render_ms: Some(4.0), gpu_ms: None, ..FrameSample::default() };
    assert_eq!(s.bound(0.85), Bound::Main);
    let s = FrameSample { frame_ms: 10.0, main_ms: Some(3.0), render_ms: Some(4.0), gpu_ms: Some(9.0), ..FrameSample::default() };
    assert_eq!(s.bound(0.85), Bound::Gpu);
    let s = FrameSample { frame_ms: 16.7, main_ms: Some(3.0), render_ms: Some(4.0), ..FrameSample::default() };
    assert_eq!(s.bound(0.85), Bound::Wait);
    assert_eq!(frame(8.0).bound(0.85), Bound::Unknown);
}

#[test]
fn column_slots_cover_every_slot_once_or_repeat() {
    // 6 slots on 3 columns: two each.
    assert_eq!((0..3).map(|c| column_slots(c, 3, 6)).collect::<Vec<_>>(), vec![(0, 2), (2, 4), (4, 6)]);
    // 2 slots on 4 columns: each slot twice.
    assert_eq!((0..4).map(|c| column_slots(c, 4, 2)).collect::<Vec<_>>(), vec![(0, 1), (0, 1), (1, 2), (1, 2)]);
    // 5 slots on 2 columns: uneven, nothing lost.
    assert_eq!((0..2).map(|c| column_slots(c, 2, 5)).collect::<Vec<_>>(), vec![(0, 2), (2, 5)]);
    assert_eq!(column_slots(0, 0, 5), (0, 0));
    assert_eq!(column_slots(0, 3, 0), (0, 0));
}

#[test]
fn a_column_keeps_its_worst_frame_so_spikes_never_vanish() {
    let mut h = History::with_capacity(4);
    h.push(FrameSample { frame_ms: 8.0, main_ms: Some(7.5), render_ms: Some(2.0), ..FrameSample::default() });
    h.push(FrameSample { frame_ms: 40.0, main_ms: Some(3.0), render_ms: Some(39.0), ..FrameSample::default() });
    h.push(frame(8.0));
    h.push(frame(9.0));
    let cols = columns(&h, 2, 0.85);
    assert_eq!(
        cols[0],
        Some(Column { frame_ms: 40.0, bound: Bound::Render, main_ms: Some(7.5), render_ms: Some(39.0), gpu_ms: None })
    );
    assert_eq!(cols[1], Some(Column { frame_ms: 9.0, bound: Bound::Unknown, main_ms: None, render_ms: None, gpu_ms: None }));
}

#[test]
fn a_short_history_fills_the_right_hand_columns() {
    let mut h = History::with_capacity(4);
    h.push(frame(5.0));
    let cols = columns(&h, 4, 0.85);
    assert_eq!(cols.iter().map(|c| c.map(|c| c.frame_ms)).collect::<Vec<_>>(), vec![None, None, None, Some(5.0)]);
    let views = series_columns(&h, 2, |s| Some(s.frame_ms * 2.0));
    assert_eq!(views, vec![None, Some(10.0)]);
}

#[test]
fn bars_scale_and_spikes_clip() {
    assert_eq!(bar_rows(12.5, 25.0, 100), (50, false));
    assert_eq!(bar_rows(25.0, 25.0, 100), (100, false));
    assert_eq!(bar_rows(250.0, 25.0, 100), (100, true));
    assert_eq!(bar_rows(f32::NAN, 25.0, 100), (0, false));
    assert_eq!(bar_rows(-1.0, 25.0, 100), (0, false));
    assert_eq!(bar_rows(5.0, 0.0, 100), (0, false));
    assert!((bar_fraction(Some(4.0), Some(8.0)) - 0.5).abs() < f32::EPSILON);
    assert!((bar_fraction(Some(12.0), Some(8.0)) - 1.0).abs() < f32::EPSILON);
    assert!(bar_fraction(None, Some(8.0)).abs() < f32::EPSILON);
    assert!(bar_fraction(Some(4.0), Some(0.0)).abs() < f32::EPSILON);
    assert!(bar_fraction(Some(f64::NAN), Some(8.0)).abs() < f32::EPSILON);
}

#[test]
fn colours_convert_and_dim() {
    assert_eq!(rgba([1.0, 0.5, 0.0], 0.75), [255, 128, 0, 191]);
    assert_eq!(rgba([2.0, -1.0, 0.0], 1.0), [255, 0, 0, 255]);
    assert_eq!(dim([200, 100, 50, 77], 0.5), [100, 50, 25, 77]);
}

fn palette() -> Palette {
    Palette {
        background: [0, 0, 0, 100],
        main: [255, 0, 0, 255],
        render: [0, 0, 255, 255],
        gpu: [0, 255, 0, 255],
        wait: [100, 100, 100, 255],
        target: [255, 255, 255, 255],
        target_p99: [255, 255, 0, 255],
        clip: [255, 0, 255, 255],
    }
}

fn px(out: &[u8], w: u32, x: u32, y: u32) -> [u8; 4] {
    let i = ((y * w + x) * 4) as usize;
    [out[i], out[i + 1], out[i + 2], out[i + 3]]
}

#[test]
fn the_graph_draws_columns_targets_lines_and_clip_marks() {
    let p = palette();
    let spec = GraphSpec { width: 2, height: 10, top_ms: 20.0, targets: [8.0, 12.0], ratio: 0.85 };
    let mut h = History::with_capacity(2);
    h.push(FrameSample { frame_ms: 10.0, main_ms: Some(9.0), ..FrameSample::default() });
    h.push(FrameSample { frame_ms: 100.0, main_ms: Some(2.0), render_ms: Some(98.0), ..FrameSample::default() });
    let mut out = Vec::new();
    draw_graph(&h, &spec, &p, &mut out);
    assert_eq!(out.len(), 2 * 10 * 4);
    // Column 0: main-bound, 10 of 20 ms = rows 5..10, dimmed main; its main line at 9 ms (4.5 rows,
    // rounds to 5) -> row 5, full colour; above the column, background.
    assert_eq!(px(&out, 2, 0, 9), dim(p.main, 0.55));
    assert_eq!(px(&out, 2, 0, 3), p.background);
    assert_eq!(px(&out, 2, 0, 5), p.main, "main line over the column");
    // Targets over the columns: 8 ms -> 4 rows -> row 6; 12 ms -> 6 rows -> row 4.
    assert_eq!(px(&out, 2, 0, 6), p.target);
    assert_eq!(px(&out, 2, 0, 4), p.target_p99);
    assert_eq!(px(&out, 2, 1, 4), p.target_p99);
    // Column 1: render-bound spike off the scale: clip mark on top, the render line clamped to
    // row 0 (no render value in column 0 to join from).
    assert_eq!(px(&out, 2, 1, 0), p.render);
    assert_eq!(px(&out, 2, 1, 1), p.clip);
    assert_eq!(px(&out, 2, 1, 2), dim(p.render, 0.55));
    // The main line joins column 0 (row 5) to column 1 (2 ms -> 1 row -> row 9): rows 5..=9.
    assert_eq!(px(&out, 2, 1, 7), p.main);
    assert_eq!(px(&out, 2, 1, 9), p.main);
}

#[test]
fn an_empty_graph_is_background_and_targets() {
    let p = palette();
    let spec = GraphSpec { width: 8, height: 4, top_ms: 16.0, targets: [8.0, 12.0], ratio: 0.85 };
    let mut out = Vec::new();
    draw_graph(&History::with_capacity(10), &spec, &p, &mut out);
    // 8 ms -> 2 rows -> row 2: dashed (x 0..4 on, 4..6 off, 6..8 on).
    assert_eq!(px(&out, 8, 0, 2), p.target);
    assert_eq!(px(&out, 8, 4, 2), p.background);
    assert_eq!(px(&out, 8, 6, 2), p.target);
    assert_eq!(px(&out, 8, 0, 3), p.background);
}

#[test]
fn the_panel_scales_with_the_screen_and_the_ui_scale() {
    // Auto: 1440p -> 1.33 physical; a 1.5 OS scale divides the logical px.
    let (l, p) = super::panel_scale(0.0, 0.9, 1440, 1.5, 1.0);
    assert!((p - 4.0 / 3.0).abs() < 1e-5 && (l - 8.0 / 9.0).abs() < 1e-5, "{l} {p}");
    // 720p clamps to the minimum.
    assert_eq!(super::panel_scale(0.0, 0.9, 720, 1.0, 1.0), (0.9, 0.9));
    // A set scale wins; UiScale 2 doubles the physical size, not the logical px.
    assert_eq!(super::panel_scale(1.5, 0.9, 720, 1.0, 2.0), (1.5, 3.0));
    assert_eq!(super::panel_scale(0.0, 0.9, 1080, 0.0, 0.0), (1.0, 1.0));
}

#[test]
fn sparklines_scale_to_their_max() {
    let mut out = Vec::new();
    let c = [10, 20, 30, 255];
    let max = draw_sparkline(&[None, Some(1.0), Some(2.0)], 3, 4, c, [0, 0, 0, 0], &mut out);
    assert_eq!(max, Some(2.0));
    assert_eq!(px(&out, 3, 0, 3), [0, 0, 0, 0]);
    assert_eq!(px(&out, 3, 1, 2), c, "1 of 2 -> 2 rows, top row 2");
    assert_eq!(px(&out, 3, 1, 3), dim(c, 0.55));
    assert_eq!(px(&out, 3, 2, 0), c);
    assert_eq!(draw_sparkline(&[None], 3, 4, c, [0, 0, 0, 0], &mut out), None);
    assert_eq!(out.len(), 3 * 4 * 4);
}
