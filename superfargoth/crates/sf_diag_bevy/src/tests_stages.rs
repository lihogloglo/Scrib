//! Stage walls (`stages.rs`) and the encode accumulator (`encode.rs`), cpu-frame-audit M1/M3.

use std::time::Duration;

use bevy_app::{App, TaskPoolPlugin};
use bevy_diagnostic::{DiagnosticsPlugin, FrameCountPlugin};
use bevy_time::{TimePlugin, TimeUpdateStrategy};
use sf_diag::Counters;

use crate::encode::{EncodeFrame, QueueTraffic, encoder_name};
use crate::stages::{MAIN_STAGES, completion_deltas};
use crate::{DiagCounters, DiagSettings, FramePhasePlugin, SfDiagPlugin};

fn drain(counters: &Counters) -> Vec<(String, Option<f64>)> {
    let mut seen = Vec::new();
    counters.drain(|_, def, v| seen.push((def.name.clone(), v)));
    seen
}

fn value(seen: &[(String, Option<f64>)], name: &str) -> Option<f64> {
    seen.iter().find(|(n, _)| n == name).and_then(|(_, v)| *v)
}

#[test]
fn frame_phases_report_main_stage_walls() {
    let mut app = App::new();
    app.add_plugins((TaskPoolPlugin::default(), FrameCountPlugin, TimePlugin, DiagnosticsPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_micros(15_625)))
        .add_plugins((SfDiagPlugin::new(DiagSettings::default()), FramePhasePlugin));
    for _ in 0..4 {
        app.update();
    }
    let seen = drain(&app.world().resource::<DiagCounters>().0);
    for name in MAIN_STAGES.iter().chain(&[sf_diag::pacing::MAIN_BUSY]) {
        let v = value(&seen, name).unwrap_or_else(|| panic!("{name} missing: {seen:?}"));
        assert!(v.is_finite() && v >= 0.0, "{name} = {v}");
    }
}

#[test]
fn completion_deltas_split_the_span_without_going_negative() {
    let d = completion_deltas(&[1.0, 3.0, 2.0, f64::NAN, 5.0]);
    assert_eq!(d, vec![1.0, 2.0, 0.0, 0.0, 2.0]);
    assert!((d.iter().sum::<f64>() - 5.0).abs() < 1e-12);
}

#[test]
fn encoder_names_drop_paths_generics_and_closures() {
    assert_eq!(encoder_name("bevy_pbr::render::light::shadow_pass<bevy_pbr::X>"), "shadow_pass");
    assert_eq!(encoder_name("a::b::main_pass::{{closure}}"), "main_pass");
    assert_eq!(encoder_name("camera_driver_span_end"), "camera_driver_span_end");
    assert_eq!(encoder_name("RenderContextState::flush_encoder"), "flush_encoder");
}

#[test]
fn encode_frame_folds_finishes_per_system_and_resets() {
    let mut frame = EncodeFrame::default();
    frame.finish("a::shadow_pass<A>", 1.0);
    frame.finish("b::shadow_pass<B>", 3.0);
    frame.finish("x::main_pass::{{closure}}", 0.5);
    frame.finish_wall(3.5);
    frame.submit(0.25);
    frame.submit(0.25);
    let counters = Counters::new();
    frame.report(&counters);
    let seen = drain(&counters);
    let expect = [
        ("encode/shadow_pass/sum", 4.0),
        ("encode/shadow_pass/max", 3.0),
        ("encode/shadow_pass/count", 2.0),
        ("encode/main_pass/sum", 0.5),
        ("encode/main_pass/count", 1.0),
        ("encode/finish_critical", 3.0),
        ("encode/finish_wall", 3.5),
        ("encode/submit", 0.5),
    ];
    for (name, v) in expect {
        assert_eq!(value(&seen, name), Some(v), "{name}: {seen:?}");
    }
    // A frame with nothing timed reports nothing; a system missing this frame is not reported.
    frame.report(&counters);
    assert!(drain(&counters).iter().all(|(_, v)| v.is_none()));
    frame.finish("c::main_pass", 2.0);
    frame.report(&counters);
    let seen = drain(&counters);
    assert_eq!(value(&seen, "encode/main_pass/max"), Some(2.0));
    assert_eq!(value(&seen, "encode/shadow_pass/sum"), None);
    assert_eq!(value(&seen, "encode/submit"), Some(0.0));
}

#[test]
fn encode_frame_reports_poll_only_when_polled_and_queue_traffic_per_frame() {
    let mut frame = EncodeFrame::default();
    frame.submit(1.0);
    frame.poll(0.25);
    frame.poll(0.5);
    frame.queue(QueueTraffic { submits: 2, command_buffers: 31, writes: [(90, 4096), (3, 512), (1, 64)] });
    let counters = Counters::new();
    frame.report(&counters);
    let seen = drain(&counters);
    let expect = [
        ("encode/submit", 1.0),
        ("encode/poll", 0.75),
        ("encode/queue/submits", 2.0),
        ("encode/queue/command_buffers", 31.0),
        ("encode/queue/write_buffer/calls", 90.0),
        ("encode/queue/write_buffer/bytes", 4096.0),
        ("encode/queue/write_buffer_with/calls", 3.0),
        ("encode/queue/write_buffer_with/bytes", 512.0),
        ("encode/queue/write_texture/calls", 1.0),
        ("encode/queue/write_texture/bytes", 64.0),
    ];
    for (name, v) in expect {
        assert_eq!(value(&seen, name), Some(v), "{name}: {seen:?}");
    }
    // Next frame: no poll (switch off) = no poll value; queue traffic resets to zero.
    frame.submit(1.0);
    frame.report(&counters);
    let seen = drain(&counters);
    assert_eq!(value(&seen, "encode/poll"), None);
    assert_eq!(value(&seen, "encode/queue/write_buffer/calls"), Some(0.0));
    // Traffic alone (no timed event) still reports.
    frame.queue(QueueTraffic { submits: 1, ..QueueTraffic::default() });
    frame.report(&counters);
    assert_eq!(value(&drain(&counters), "encode/queue/submits"), Some(1.0));
}
