#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use std::sync::Arc;

use super::*;

fn controller() -> ReactionController {
    ReactionController::new(Arc::new(ReactionSet::default()))
}

fn request(class: ReactionClass, impulse: f32, seed: u64) -> ReactionRequest {
    ReactionRequest { class, body: 0, point: WorldPos::default(), impulse: Vec3::new(0.0, 0.0, impulse), tick: 7, seed }
}

/// Steps `seconds` at `fps`, returning every output.
fn run(c: &mut ReactionController, fps: f32, seconds: f32, sensed: &Sensed) -> Vec<ReactionOutput> {
    let steps = (seconds * fps).round() as usize;
    (0..steps).map(|_| c.step(1.0 / fps, sensed)).collect()
}

fn events(outs: &[ReactionOutput]) -> Vec<ReactionEvent> {
    outs.iter().flat_map(|o| o.events.iter().copied()).collect()
}

#[test]
fn default_set_validates_and_round_trips_ron() {
    let set = ReactionSet::default();
    set.validate().expect("valid");
    let again = ReactionSet::from_ron(&set.to_ron()).expect("parses");
    assert_eq!(set, again);
    assert_eq!(set.stagger.strength.curve(Region::Legs).eval(0.08), 0.6);
    assert_eq!(set.stagger.strength.curve(Region::Arms).eval(0.08), 0.35);
}

#[test]
fn validate_rejects_a_discontinuous_stagger() {
    let mut set = ReactionSet::default();
    set.stagger.strength.all = Curve(vec![(0.0, 1.0), (0.2, 0.3), (0.5, 0.8)]);
    assert!(set.validate().is_err());
    let mut set = ReactionSet::default();
    set.get_up.strength.all = Curve(vec![(0.0, 0.5), (0.3, 1.0)]);
    assert!(set.validate().unwrap_err().contains("get_up"));
    let mut set = ReactionSet::default();
    set.knockdown.strength.all = Curve(vec![(0.5, 0.2), (0.1, 0.0)]);
    assert!(set.validate().is_err());
}

#[test]
fn tracking_is_full_strength_and_clip() {
    let mut c = controller();
    let out = c.step(1.0 / 60.0, &Sensed::default());
    assert_eq!(out.state, ReactionState::Tracking);
    assert_eq!(out.region_strength, [1.0; 4]);
    assert_eq!(out.region_physics_weight, [0.0; 4]);
    assert!(out.events.is_empty());
}

#[test]
fn stagger_dips_then_returns_to_tracking_continuously() {
    let mut c = controller();
    assert!(c.request(request(ReactionClass::Stagger, 40.0, 1)));
    let outs = run(&mut c, 60.0, 1.0, &Sensed::default());
    assert_eq!(outs[0].state, ReactionState::Stagger);
    let low = outs.iter().map(|o| o.strength(Region::Spine)).fold(1.0, f32::min);
    assert!(low < 0.45, "the dip reaches {low}");
    let legs = outs.iter().map(|o| o.strength(Region::Legs)).fold(1.0, f32::min);
    assert!(legs > low, "legs dip less: {legs}");
    let back = outs.iter().position(|o| o.state == ReactionState::Tracking).expect("returns to tracking");
    assert!((0.4..0.65).contains(&(back as f32 / 60.0)));
    // Strength is continuous through the whole reaction, including the hand-back.
    let mut prev = 1.0;
    for o in &outs {
        for r in Region::ALL {
            let s = o.strength(r);
            if r == Region::Spine {
                assert!((s - prev).abs() < 0.2, "jump {prev} -> {s}");
                prev = s;
            }
        }
    }
    let last = outs.last().unwrap();
    assert_eq!(last.region_strength, [1.0; 4]);
    assert_eq!(last.region_physics_weight, [0.0; 4]);
    assert!(events(&outs).is_empty(), "a stagger raises no events");
}

#[test]
fn stagger_escalates_on_impulse() {
    let mut c = controller();
    assert!(c.request(request(ReactionClass::Stagger, 120.0, 1)));
    let out = c.step(1.0 / 60.0, &Sensed::default());
    assert_eq!(out.state, ReactionState::Falling);
    assert_eq!(out.events, vec![ReactionEvent::Fell]);
}

#[test]
fn stagger_falls_on_lost_balance_only_on_the_host() {
    let tilted = Sensed { upright: 0.3, ..Sensed::default() };
    let mut c = controller();
    c.request(request(ReactionClass::Stagger, 40.0, 1));
    let out = c.step(1.0 / 60.0, &tilted);
    assert_eq!(out.state, ReactionState::Falling);
    assert_eq!(out.events, vec![ReactionEvent::Fell]);

    let mut client = controller();
    client.request(request(ReactionClass::Stagger, 40.0, 1));
    let out = client.step(1.0 / 60.0, &Sensed { host: false, ..tilted });
    assert_eq!(out.state, ReactionState::Stagger);
    // Tracking never falls by balance alone.
    let mut idle = controller();
    assert_eq!(idle.step(1.0 / 60.0, &tilted).state, ReactionState::Tracking);
}

#[test]
fn a_re_hit_during_a_stagger_deepens_it_without_a_pop() {
    let mut c = controller();
    c.request(request(ReactionClass::Stagger, 40.0, 1));
    let mid = run(&mut c, 60.0, 0.3, &Sensed::default());
    let before = mid.last().unwrap().strength(Region::Spine);
    c.request(request(ReactionClass::Stagger, 40.0, 2));
    let after = c.step(1.0 / 60.0, &Sensed::default()).strength(Region::Spine);
    assert!(after < before + 0.1, "no recovery jump: {before} -> {after}");
    assert_eq!(c.state(), ReactionState::Stagger);
}

#[test]
fn knockdown_reproduces_the_brace_constants() {
    // Brace 0.25 fading linearly to limp over 0.8 s, all bodies simulated.
    let mut c = controller();
    assert!(c.request(request(ReactionClass::Knockdown, 120.0, 1)));
    let fast = Sensed { pelvis_speed: 3.0, ..Sensed::default() };
    let outs = run(&mut c, 60.0, 1.2, &fast);
    assert_eq!(events(&outs), vec![ReactionEvent::Fell]);
    for (i, o) in outs.iter().enumerate() {
        let t = (i + 1) as f32 / 60.0;
        let want = 0.25 * (1.0 - t / 0.8).max(0.0);
        for r in Region::ALL {
            assert!((o.strength(r) - want).abs() < 1e-4, "t {t}: {} vs {want}", o.strength(r));
            assert_eq!(o.physics_weight(r), 1.0);
        }
        assert_eq!(o.state, ReactionState::Falling);
    }
}

#[test]
fn knockdown_cycle_settles_gets_up_and_tracks() {
    let mut c = controller();
    c.request(request(ReactionClass::Knockdown, 120.0, 1));
    let moving = Sensed { pelvis_speed: 2.0, ..Sensed::default() };
    let rest = Sensed { pelvis_speed: 0.05, ..Sensed::default() };
    // Still moving: stays falling, even past the curve's end.
    assert!(run(&mut c, 60.0, 2.0, &moving).iter().all(|o| o.state == ReactionState::Falling));
    // At rest for rest_time: settles, then gets up (the host's clip), then tracks.
    let outs = run(&mut c, 60.0, 1.5, &rest);
    assert_eq!(events(&outs), vec![ReactionEvent::Settled, ReactionEvent::GettingUp, ReactionEvent::GotUp]);
    let settle = outs.iter().position(|o| o.events.contains(&ReactionEvent::Settled)).unwrap();
    assert!(((settle + 1) as f32 / 60.0 - 0.5).abs() < 0.03, "settled after rest_time: step {settle}");
    // Getting up: strength rises 0 -> 1 and weight falls 1 -> 0, continuously.
    let up: Vec<_> = outs.iter().filter(|o| o.state == ReactionState::GettingUp).collect();
    assert!(up.windows(2).all(|w| w[1].strength(Region::Spine) >= w[0].strength(Region::Spine)));
    assert!(up.windows(2).all(|w| w[1].physics_weight(Region::Spine) <= w[0].physics_weight(Region::Spine)));
    let last = outs.last().unwrap();
    assert_eq!((last.state, last.region_strength, last.region_physics_weight), (ReactionState::Tracking, [1.0; 4], [0.0; 4]));
}

#[test]
fn airborne_or_held_down_does_not_progress() {
    let mut c = controller();
    c.request(request(ReactionClass::Knockdown, 120.0, 1));
    let airborne = Sensed { pelvis_speed: 0.0, grounded: false, ..Sensed::default() };
    assert!(run(&mut c, 60.0, 2.0, &airborne).iter().all(|o| o.state == ReactionState::Falling));
    let knocked_out = Sensed { can_get_up: false, ..Sensed::default() };
    let outs = run(&mut c, 60.0, 3.0, &knocked_out);
    assert_eq!(outs.last().unwrap().state, ReactionState::Down);
    assert_eq!(events(&outs), vec![ReactionEvent::Settled]);
    // Down holds the fall's last values (limp, all bodies simulated).
    assert_eq!(outs.last().unwrap().region_strength, [0.0; 4]);
    assert_eq!(outs.last().unwrap().region_physics_weight, [1.0; 4]);
    // Allowed again: gets up.
    let outs = run(&mut c, 60.0, 1.0, &Sensed::default());
    assert_eq!(events(&outs), vec![ReactionEvent::GettingUp, ReactionEvent::GotUp]);
}

#[test]
fn request_acceptance_by_state() {
    let mut c = controller();
    assert!(c.request(request(ReactionClass::Knockdown, 100.0, 1)));
    assert!(!c.request(request(ReactionClass::Knockdown, 100.0, 2)), "already falling");
    assert!(!c.request(request(ReactionClass::Stagger, 10.0, 3)), "a stagger doesn't interrupt a fall");
    // A knockdown while getting up falls again.
    let mut c = controller();
    c.request(request(ReactionClass::Knockdown, 100.0, 1));
    // Rest from the start: curve end 0.8 s, then settle and the get-up begin.
    run(&mut c, 60.0, 0.85, &Sensed::default());
    assert_eq!(c.state(), ReactionState::GettingUp);
    assert!(!c.request(request(ReactionClass::Stagger, 10.0, 3)));
    assert!(c.request(request(ReactionClass::Knockdown, 100.0, 4)));
    assert_eq!(c.step(1.0 / 60.0, &Sensed::default()).state, ReactionState::Falling);
}

#[test]
fn death_crumples_and_is_terminal() {
    let mut c = controller();
    c.request(request(ReactionClass::Stagger, 20.0, 1));
    assert!(c.request(request(ReactionClass::Death, 50.0, 2)));
    let outs = run(&mut c, 60.0, 1.0, &Sensed::default());
    assert_eq!(events(&outs), vec![ReactionEvent::Died]);
    for (i, o) in outs.iter().enumerate() {
        let t = (i + 1) as f32 / 60.0;
        let want = (1.0 - t / 0.45).max(0.0);
        assert!((o.strength(Region::Spine) - want).abs() < 1e-4);
        assert_eq!(o.state, ReactionState::Dead);
        assert_eq!(o.physics_weight(Region::Head), 1.0);
    }
    for class in [ReactionClass::Stagger, ReactionClass::Knockdown, ReactionClass::Death] {
        assert!(!c.request(request(class, 500.0, 9)));
    }
    let more = run(&mut c, 60.0, 5.0, &Sensed { pelvis_speed: 0.0, ..Sensed::default() });
    assert!(events(&more).is_empty());
    assert!(more.iter().all(|o| o.state == ReactionState::Dead && o.max_strength() == 0.0));
}

#[test]
fn timelines_are_deterministic_per_seed() {
    let trace = |seed| {
        let mut c = controller();
        c.request(request(ReactionClass::Stagger, 40.0, seed));
        run(&mut c, 60.0, 0.7, &Sensed::default())
    };
    assert_eq!(trace(5), trace(5));
    assert_ne!(trace(5), trace(6), "the seed varies the stagger");
    // Variation is small: within the set's ±10 % of time and depth.
    let (a, b) = (trace(5), trace(6));
    let end = |t: &[ReactionOutput]| t.iter().position(|o| o.state == ReactionState::Tracking).unwrap();
    assert!((end(&a) as f32 / end(&b) as f32 - 1.0).abs() < 0.25);
    // Variation off: every seed gives the same knockdown.
    let knock = |seed| {
        let mut c = controller();
        c.request(request(ReactionClass::Knockdown, 100.0, seed));
        run(&mut c, 60.0, 1.0, &Sensed::default())
    };
    assert_eq!(knock(1), knock(2));
}

#[test]
fn curves_do_not_depend_on_the_frame_rate() {
    // Common sample times: multiples of 1/6 s are whole steps at 30, 60 and 144 fps.
    let sample = |class, fps: f32| {
        let mut c = controller();
        c.request(request(class, 40.0, 3));
        let per = (fps / 6.0).round() as usize;
        let mut got = Vec::new();
        for k in 1..=6 {
            let mut out = None;
            for _ in 0..per {
                out = Some(c.step(1.0 / fps, &Sensed { pelvis_speed: 2.0, ..Sensed::default() }));
            }
            let out = out.unwrap();
            got.push((out.state, out.region_strength, out.region_physics_weight, k));
        }
        got
    };
    for class in [ReactionClass::Stagger, ReactionClass::Knockdown, ReactionClass::Death] {
        let reference = sample(class, 60.0);
        for fps in [30.0, 144.0] {
            for (a, b) in reference.iter().zip(sample(class, fps)) {
                assert_eq!(a.0, b.0, "{class:?} at {fps} fps, sample {}", a.3);
                for i in 0..4 {
                    assert!((a.1[i] - b.1[i]).abs() < 2e-3, "{class:?} {fps} fps strength {} vs {}", a.1[i], b.1[i]);
                    assert!((a.2[i] - b.2[i]).abs() < 2e-3, "{class:?} {fps} fps weight");
                }
            }
        }
    }
}

#[test]
fn settling_is_within_a_frame_across_frame_rates() {
    let settle_time = |fps: f32| {
        let mut c = controller();
        c.request(request(ReactionClass::Knockdown, 100.0, 1));
        let sensed = Sensed { can_get_up: false, ..Sensed::default() };
        for i in 1..=(3.0 * fps) as usize {
            if c.step(1.0 / fps, &sensed).events.contains(&ReactionEvent::Settled) {
                return i as f32 / fps;
            }
        }
        panic!("never settled");
    };
    let reference = settle_time(144.0);
    for fps in [30.0, 60.0] {
        assert!((settle_time(fps) - reference).abs() <= 1.0 / fps + 1e-3);
    }
}

#[test]
fn region_names_map_to_controller_regions() {
    for (name, region) in [("core", Region::Spine), ("head", Region::Head), ("arm_l", Region::Arms), ("wing_r", Region::Arms), ("leg_r", Region::Legs), ("hind_paw", Region::Legs), ("tail", Region::Spine)] {
        assert_eq!(Region::of_name(name), region, "{name}");
    }
}

#[test]
fn curves_interpolate_and_hold() {
    let c = Curve(vec![(0.0, 1.0), (1.0, 0.0), (3.0, 1.0)]);
    assert_eq!(c.eval(-1.0), 1.0);
    assert!((c.eval(0.5) - 0.5).abs() < 1e-6);
    assert!((c.eval(2.0) - 0.5).abs() < 1e-6);
    assert_eq!(c.eval(9.0), 1.0);
    assert_eq!(Curve(Vec::new()).eval(1.0), 0.0);
}
