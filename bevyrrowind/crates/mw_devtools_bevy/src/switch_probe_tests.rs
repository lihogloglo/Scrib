//! The probe in a headless app: synthetic frames through the F3 feed, one `FeatureToggled`.

use bevy_app::prelude::*;
use sf_tunables_bevy::{FeatureToggled, TunablesPlugin};

use super::{SwitchProbe, SwitchProbePlugin};
use crate::features_panel::FeaturesPanel;
use crate::perf_panel::graph::FrameSample;
use crate::perf_panel::{PerfFrameFeed, PerfGaugeDemand};

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((TaskPoolPlugin::default(), bevy_time::TimePlugin, TunablesPlugin, SwitchProbePlugin))
        .init_resource::<FeaturesPanel>()
        .init_resource::<PerfFrameFeed>()
        .init_resource::<PerfGaugeDemand>();
    app.update();
    app
}

fn frames(app: &mut App, ms: f32, n: usize) {
    for _ in 0..n {
        let sample = FrameSample { frame_ms: ms, main_ms: Some(ms - 1.0), render_ms: Some(ms - 2.0), gpu_ms: Some(ms - 3.0), ..Default::default() };
        app.world_mut().resource_mut::<PerfFrameFeed>().0 = Some(sample);
        app.update();
    }
}

#[test]
fn a_flip_with_the_panel_open_stores_its_delta() {
    let mut app = app();
    app.world_mut().resource_mut::<FeaturesPanel>().open = true;
    frames(&mut app, 10.0, 250);
    assert!(app.world().resource::<PerfGaugeDemand>().0, "the gauges run while the Features panel is open");
    app.world_mut().write_message(FeatureToggled { name: "sky.froxels.enabled", running: false });
    frames(&mut app, 10.0, 1);
    assert!(app.world().resource::<SwitchProbe>().measuring());
    // 0.5 s settle at 10 ms, then the cheaper frames.
    frames(&mut app, 10.0, 49);
    frames(&mut app, 8.5, 300);
    let probe = app.world().resource::<SwitchProbe>();
    assert!(!probe.measuring());
    let d = probe.get("sky.froxels.enabled").expect("a result for the switch");
    assert!(!d.running);
    assert_eq!(d.frame_ms, Some(-1.5));
    assert_eq!(d.main_ms, Some(-1.5));
    assert_eq!(d.render_ms, Some(-1.5));
    assert_eq!(d.gpu_ms, Some(-1.5));
    assert_eq!(d.before.frame, 200);
    assert!(!d.noise && !d.moved && !d.superseded);
}

#[test]
fn closed_panels_record_nothing() {
    let mut app = app();
    frames(&mut app, 10.0, 250);
    assert!(!app.world().resource::<PerfGaugeDemand>().0, "no gauges with both panels closed");
    app.world_mut().write_message(FeatureToggled { name: "groundcover.enabled", running: true });
    frames(&mut app, 10.0, 1);
    let probe = app.world().resource::<SwitchProbe>();
    assert!(!probe.measuring(), "no before window: nothing waits");
    let d = probe.get("groundcover.enabled").expect("a result, without a delta");
    assert_eq!(d.frame_ms, None);
    assert_eq!(d.before.frame, 0);
}
