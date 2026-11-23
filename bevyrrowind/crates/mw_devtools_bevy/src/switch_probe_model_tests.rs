//! Flip delta statistics on synthetic frames (exact).

use bevy_math::{Quat, Vec3};

use super::{Outcome, Pose, Probe, ProbeConfig, Sample};

const CFG: ProbeConfig = ProbeConfig { window_s: 2.0, settle_s: 0.5, noise_ms: 0.5, still_m: 0.5, still_deg: 2.0 };

fn frame(ms: f32) -> Sample {
    Sample { frame_ms: ms, main_ms: Some(ms - 2.0), render_ms: Some(ms - 3.0), gpu_ms: None, pose: Some(still()) }
}

fn still() -> Pose {
    Pose { translation: Vec3::new(10.0, 2.0, -5.0), rotation: Quat::IDENTITY }
}

/// Feeds `n` frames, returns what came out.
fn feed(p: &mut Probe, s: Sample, n: usize) -> Vec<Outcome> {
    (0..n).flat_map(|_| p.push(s, &CFG)).collect()
}

/// Before 200 frames of 10 ms (2 s), a flip, 0.5 s of settle at 10 ms, then `after` ms frames
/// until the window closes.
fn measure(after: f32) -> Vec<Outcome> {
    let mut p = Probe::default();
    assert!(feed(&mut p, frame(10.0), 400).is_empty(), "nothing pending, nothing out");
    assert!(p.flip(&[("sky.froxels.enabled", false)], &CFG).is_empty());
    assert!(p.is_pending());
    assert!(feed(&mut p, frame(10.0), 50).is_empty(), "settle frames");
    let out = feed(&mut p, frame(after), 1000);
    assert!(!p.is_pending());
    out
}

#[test]
fn ring_keeps_one_window() {
    let mut p = Probe::default();
    feed(&mut p, frame(10.0), 400);
    assert_eq!(p.ring_len(), 200, "2 s of 10 ms frames");
}

#[test]
fn ten_to_eight_and_a_half_is_minus_one_and_a_half() {
    let out = measure(8.5);
    assert_eq!(out.len(), 1);
    let (name, d) = out[0];
    assert_eq!(name, "sky.froxels.enabled");
    assert!(!d.running);
    assert_eq!(d.frame_ms, Some(-1.5));
    assert_eq!(d.main_ms, Some(-1.5));
    assert_eq!(d.render_ms, Some(-1.5));
    assert_eq!(d.gpu_ms, None, "GPU never timed");
    assert_eq!(d.before.frame, 200);
    assert_eq!(d.before.gpu, 0);
    // ceil(2000 / 8.5) frames close the after window.
    assert_eq!(d.after.frame, 236);
    assert!(!d.noise && !d.moved && !d.superseded);
}

#[test]
fn a_small_change_is_noise() {
    let (_, d) = measure(9.75)[0];
    assert_eq!(d.frame_ms, Some(-0.25));
    assert!(d.noise);
    let (_, d) = measure(10.5)[0];
    assert_eq!(d.frame_ms, Some(0.5));
    assert!(!d.noise, "the noise floor itself counts");
}

#[test]
fn settle_frames_are_left_out() {
    let mut p = Probe::default();
    feed(&mut p, frame(10.0), 200);
    p.flip(&[("groundcover.enabled", true)], &CFG);
    // A hitch inside the settle time (exactly 0.5 s: 10 x 50 ms) never reaches the after window.
    assert!(feed(&mut p, frame(50.0), 10).is_empty());
    let out = feed(&mut p, frame(12.0), 1000);
    let (_, d) = out[0];
    assert_eq!(d.frame_ms, Some(2.0));
    assert_eq!(d.after.frame, 167, "ceil(2000 / 12)");
}

#[test]
fn a_camera_move_or_turn_marks_moved() {
    for pose in [
        Pose { translation: still().translation + Vec3::X * 0.6, rotation: Quat::IDENTITY },
        Pose { translation: still().translation, rotation: Quat::from_rotation_y(3f32.to_radians()) },
    ] {
        let mut p = Probe::default();
        feed(&mut p, frame(10.0), 200);
        p.flip(&[("render.ssao", false)], &CFG);
        feed(&mut p, frame(10.0), 60);
        let out = feed(&mut p, Sample { pose: Some(pose), ..frame(10.0) }, 1000);
        assert!(out[0].1.moved);
        assert_eq!(out[0].1.frame_ms, Some(0.0));
    }
    // Under both thresholds: still.
    let mut p = Probe::default();
    feed(&mut p, frame(10.0), 200);
    p.flip(&[("render.ssao", false)], &CFG);
    let small = Pose { translation: still().translation + Vec3::Y * 0.4, rotation: Quat::from_rotation_x(1f32.to_radians()) };
    let out = feed(&mut p, Sample { pose: Some(small), ..frame(10.0) }, 1000);
    assert!(!out[0].1.moved);
}

#[test]
fn a_second_flip_inside_the_window_supersedes_the_first() {
    let mut p = Probe::default();
    feed(&mut p, frame(9.0), 400);
    assert!(p.flip(&[("a", false)], &CFG).is_empty());
    feed(&mut p, frame(9.0), 100);
    let out = p.flip(&[("b", true)], &CFG);
    assert_eq!(out.len(), 1);
    let (name, d) = out[0];
    assert_eq!(name, "a");
    assert!(d.superseded);
    assert_eq!(d.frame_ms, None, "the older measurement is dropped");
    // The second flip measures against the frames just before it (9 ms).
    feed(&mut p, frame(9.0), 55);
    let out = feed(&mut p, frame(7.0), 1000);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].0, "b");
    assert!(!out[0].1.superseded);
    assert_eq!(out[0].1.frame_ms, Some(-2.0));
}

#[test]
fn no_frames_before_no_delta() {
    let mut p = Probe::default();
    let out = p.flip(&[("a", true), ("b", true)], &CFG);
    assert_eq!(out.len(), 2);
    assert!(out.iter().all(|(_, d)| d.frame_ms.is_none() && !d.superseded && d.before.frame == 0));
    assert!(!p.is_pending());
}

// ---- Quick A/B runner (exact) ----

mod quick_ab {
    use super::super::{AbAbort, AbAction, AbConfig, AbInput, AbOutcome, AbRun, Counts, SwitchDelta, median};

    const AB: AbConfig = AbConfig { rounds: 3, noise_ms: 0.5 };

    fn delta(running: bool, frame: f32, moved: bool) -> SwitchDelta {
        SwitchDelta {
            running,
            frame_ms: Some(frame),
            main_ms: Some(frame / 2.0),
            render_ms: Some(frame / 4.0),
            gpu_ms: None,
            before: Counts::default(),
            after: Counts::default(),
            noise: false,
            moved,
            superseded: false,
        }
    }

    /// The fake probe and frame the runner polls.
    struct Sim {
        run: AbRun,
        generation: u64,
        result: Option<SwitchDelta>,
    }

    impl Sim {
        fn new(running: bool) -> Self {
            Self { run: AbRun::new(running, AB), generation: 0, result: None }
        }

        fn input(&self) -> AbInput {
            AbInput { open: true, ready: true, generation: self.generation, result: self.result, ..AbInput::default() }
        }

        /// One whole flip: ready, the flip, its toggle, then the probe result (`frame` ms delta,
        /// `moved`). Returns the action after the result.
        fn flip(&mut self, frame: f32, moved: bool) -> AbAction {
            let AbAction::Flip(target) = self.run.poll(&self.input()) else { panic!("a flip when ready") };
            assert_eq!(self.run.poll(&AbInput { own: Some(target), waiting: true, ..self.input() }), AbAction::Wait);
            assert_eq!(self.run.poll(&AbInput { waiting: true, ..self.input() }), AbAction::Wait, "still measuring");
            self.generation += 1;
            self.result = Some(delta(target, frame, moved));
            self.run.poll(&self.input())
        }

        /// The flip in flight, as far as its toggle.
        fn sent(&mut self) -> bool {
            let AbAction::Flip(target) = self.run.poll(&self.input()) else { panic!("a flip when ready") };
            assert_eq!(self.run.poll(&AbInput { own: Some(target), waiting: true, ..self.input() }), AbAction::Wait);
            target
        }
    }

    fn aborted(why: AbAbort) -> AbAction {
        AbAction::Finish(AbOutcome::Aborted(why))
    }

    #[test]
    fn median_of_mixed_signs() {
        assert_eq!(median(vec![]), None);
        assert_eq!(median(vec![-1.0, 3.0, 0.5]), Some(0.5));
        assert_eq!(median(vec![2.0, -1.0, 1.0, -4.0]), Some(0.0), "mean of the middle two");
    }

    #[test]
    fn three_rounds_give_the_median_cost_and_end_as_they_started() {
        let mut sim = Sim::new(true);
        assert_eq!(sim.run.progress(), (0, 6));
        // On at the start: off (cost = -delta), then on (cost = +delta), three times. Mixed signs:
        // one off flip got cheaper by only 0.2, one on flip read -0.4.
        let flips = [-1.5, 1.4, -0.2, 1.7, -1.6, -0.4];
        let mut last = AbAction::Wait;
        for (i, f) in flips.into_iter().enumerate() {
            last = sim.flip(f, false);
            if i + 1 < flips.len() {
                assert_eq!(last, AbAction::Wait, "flip {i}");
            }
        }
        // Costs 1.5 1.4 0.2 1.7 1.6 -0.4, sorted -0.4 0.2 1.4 1.5 1.6 1.7: median 1.45.
        let AbAction::Finish(AbOutcome::Done(c)) = last else { panic!("done: {last:?}") };
        assert!((c.frame_ms - 1.45).abs() < 1e-6, "{c:?}");
        assert!((c.main_ms.expect("main") - 0.725).abs() < 1e-6);
        assert_eq!((c.min_ms, c.max_ms, c.rounds, c.gpu_ms, c.noise), (-0.4, 1.7, 3, None, false));
        assert_eq!(sim.run.progress(), (6, 6));
        assert!(sim.run.running() && sim.run.original(), "back in its state at the start");
    }

    #[test]
    fn a_moved_round_is_retried_once_then_aborts() {
        let mut sim = Sim::new(false);
        // Off at the start: the first flip turns it on.
        assert_eq!(sim.flip(1.5, true), AbAction::Wait);
        assert_eq!(sim.flip(-1.5, false), AbAction::Wait, "moved round discarded, retried");
        assert_eq!(sim.run.progress(), (2, 8), "a retry adds a pair");
        assert_eq!(sim.flip(1.5, false), AbAction::Wait);
        assert_eq!(sim.flip(-1.5, false), AbAction::Wait, "the retry counts");
        // Round 2 moves, and again in its retry: abort.
        assert_eq!(sim.flip(1.5, false), AbAction::Wait);
        assert_eq!(sim.flip(-1.5, true), AbAction::Wait);
        assert_eq!(sim.flip(1.5, true), AbAction::Wait);
        assert_eq!(sim.flip(-1.5, false), aborted(AbAbort::CameraMoving));
        assert!(!sim.run.running(), "pairs end where they started");
    }

    #[test]
    fn a_retried_round_that_holds_still_finishes() {
        let mut sim = Sim::new(true);
        assert_eq!(sim.flip(-1.0, true), AbAction::Wait);
        assert_eq!(sim.flip(1.0, false), AbAction::Wait);
        for _ in 0..5 {
            assert_eq!(sim.flip(-1.0, false), AbAction::Wait);
            let last = sim.flip(1.0, false);
            if let AbAction::Finish(AbOutcome::Done(c)) = last {
                assert_eq!((c.frame_ms, c.rounds), (1.0, 4), "four pairs run, three kept");
                return;
            }
            assert_eq!(last, AbAction::Wait);
        }
        panic!("never finished");
    }

    #[test]
    fn abort_paths_and_the_state_to_restore() {
        // Panel closed.
        let mut sim = Sim::new(true);
        assert_eq!(sim.run.poll(&AbInput { open: false, ..sim.input() }), aborted(AbAbort::Closed));
        // Another switch flipped while measuring.
        let mut sim = Sim::new(true);
        sim.sent();
        assert_eq!(sim.run.poll(&AbInput { others: true, waiting: true, ..sim.input() }), aborted(AbAbort::Interrupted));
        // This switch flipped from elsewhere, mid-pair (away from the start: the driver puts it back).
        let mut sim = Sim::new(true);
        sim.flip(-1.0, false);
        assert!(!sim.run.running() && sim.run.original());
        assert_eq!(sim.run.poll(&AbInput { own: Some(true), ..sim.input() }), aborted(AbAbort::Interrupted));
        // A superseded result.
        let mut sim = Sim::new(true);
        let t = sim.sent();
        sim.generation += 1;
        sim.result = Some(SwitchDelta { superseded: true, frame_ms: None, ..delta(t, 0.0, false) });
        assert_eq!(sim.run.poll(&sim.input()), aborted(AbAbort::Interrupted));
        // The flip changed nothing: no toggle comes back.
        let mut sim = Sim::new(true);
        assert_eq!(sim.run.poll(&sim.input()), AbAction::Flip(false));
        assert_eq!(sim.run.poll(&sim.input()), AbAction::Wait);
        assert_eq!(sim.run.poll(&sim.input()), AbAction::Wait);
        assert!(matches!(sim.run.poll(&sim.input()), AbAction::Finish(AbOutcome::Aborted(AbAbort::Refused(_)))));
        // Not ready: no flip yet.
        let mut sim = Sim::new(true);
        assert_eq!(sim.run.poll(&AbInput { ready: false, ..sim.input() }), AbAction::Wait);
    }

    #[test]
    fn no_delta_twice_is_no_measurement() {
        let mut sim = Sim::new(true);
        // The probe had no before window: the result lands with the toggle, without a delta.
        let empty = |sim: &mut Sim| {
            let AbAction::Flip(t) = sim.run.poll(&sim.input()) else { panic!("a flip") };
            sim.generation += 1;
            sim.result = Some(SwitchDelta { frame_ms: None, ..delta(t, 0.0, false) });
            sim.run.poll(&AbInput { own: Some(t), ..sim.input() })
        };
        assert_eq!(empty(&mut sim), AbAction::Wait);
        assert_eq!(sim.flip(1.0, false), AbAction::Wait, "round 1 discarded, retried");
        assert_eq!(empty(&mut sim), AbAction::Wait);
        assert_eq!(empty(&mut sim), aborted(AbAbort::NoMeasurement));
    }
}
