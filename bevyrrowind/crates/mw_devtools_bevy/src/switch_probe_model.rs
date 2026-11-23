//! The flip delta probe's statistics, without Bevy (spec `superfargoth/docs/specs/feature-switches.md`
//! "Cost beside the switch"): the mean frame, main, render and GPU ms over a window before a
//! feature switch flips, against the same window after a settle time.
//!
//! Time is the sum of the samples' own frame times (each sample is one frame, its `frame_ms` the
//! real time it took), so a window of `window_s` is the last frames whose times add up to it, and
//! a run fed synthetic frames is exact.

use std::collections::VecDeque;

use bevy_math::{Quat, Vec3};

/// Where the view camera was on one frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pose {
    pub translation: Vec3,
    pub rotation: Quat,
}

/// One frame's times (ms) and the camera pose; `None` for a thread or GPU time nobody measured.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Sample {
    pub frame_ms: f32,
    pub main_ms: Option<f32>,
    pub render_ms: Option<f32>,
    pub gpu_ms: Option<f32>,
    pub pose: Option<Pose>,
}

/// The probe's tunables (`diag.switch.*`), read each frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProbeConfig {
    pub window_s: f32,
    pub settle_s: f32,
    pub noise_ms: f32,
    pub still_m: f32,
    pub still_deg: f32,
}

/// Frames counted in a window, per value (a thread or GPU time can be missing on some frames).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Counts {
    pub frame: u32,
    pub main: u32,
    pub render: u32,
    pub gpu: u32,
}

/// One switch's measured flip: after minus before (ms; negative = the flip made it cheaper).
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(clippy::struct_excessive_bools, reason = "independent flags a view shows side by side")]
pub struct SwitchDelta {
    /// The switch's running state after the flip.
    pub running: bool,
    /// `None`: no frames on one side (nothing was recorded before the flip, or superseded).
    pub frame_ms: Option<f32>,
    pub main_ms: Option<f32>,
    pub render_ms: Option<f32>,
    pub gpu_ms: Option<f32>,
    pub before: Counts,
    pub after: Counts,
    /// |frame delta| under `diag.switch.noise_ms`.
    pub noise: bool,
    /// The camera moved or turned past `diag.switch.still_m` / `still_deg` during the windows.
    pub moved: bool,
    /// Another switch flipped before the after window closed: no delta.
    pub superseded: bool,
}

impl SwitchDelta {
    fn empty(running: bool) -> Self {
        Self {
            running,
            frame_ms: None,
            main_ms: None,
            render_ms: None,
            gpu_ms: None,
            before: Counts::default(),
            after: Counts::default(),
            noise: false,
            moved: false,
            superseded: false,
        }
    }
}

/// A switch's result as the probe hands it out.
pub type Outcome = (&'static str, SwitchDelta);

/// Running sums over one window.
#[derive(Clone, Copy, Debug, Default)]
struct Sums {
    frame: f64,
    main: f64,
    render: f64,
    gpu: f64,
    counts: Counts,
}

impl Sums {
    fn add(&mut self, s: &Sample) {
        self.frame += f64::from(s.frame_ms);
        self.counts.frame += 1;
        if let Some(v) = s.main_ms {
            self.main += f64::from(v);
            self.counts.main += 1;
        }
        if let Some(v) = s.render_ms {
            self.render += f64::from(v);
            self.counts.render += 1;
        }
        if let Some(v) = s.gpu_ms {
            self.gpu += f64::from(v);
            self.counts.gpu += 1;
        }
    }

    fn means(&self) -> [Option<f64>; 4] {
        let mean = |sum: f64, n: u32| (n > 0).then(|| sum / f64::from(n));
        let c = self.counts;
        [mean(self.frame, c.frame), mean(self.main, c.main), mean(self.render, c.render), mean(self.gpu, c.gpu)]
    }
}

/// The camera's largest move and turn from where it stood when the before window opened.
#[derive(Clone, Copy, Debug, Default)]
struct Motion {
    reference: Option<Pose>,
    moved: bool,
}

impl Motion {
    fn see(&mut self, pose: Option<Pose>, cfg: &ProbeConfig) {
        let Some(pose) = pose else { return };
        let Some(r) = self.reference else {
            self.reference = Some(pose);
            return;
        };
        let far = r.translation.distance(pose.translation) > cfg.still_m;
        let turned = r.rotation.angle_between(pose.rotation).to_degrees() > cfg.still_deg;
        self.moved |= far || turned;
    }
}

/// A flip waiting for its after window.
#[derive(Clone, Debug)]
struct Pending {
    switches: Vec<(&'static str, bool)>,
    before: Sums,
    after: Sums,
    /// Real time since the flip (ms), settle included.
    elapsed_ms: f64,
    /// Time inside the after window (ms).
    after_ms: f64,
    motion: Motion,
}

/// The ring of recent frames and the flip being measured.
#[derive(Clone, Debug, Default)]
pub struct Probe {
    ring: VecDeque<Sample>,
    ring_ms: f64,
    pending: Option<Pending>,
}

impl Probe {
    /// A flip is waiting for its after window.
    pub fn is_pending(&self) -> bool {
        self.pending.is_some()
    }

    /// Frames in the ring (the next flip's before window).
    pub fn ring_len(&self) -> usize {
        self.ring.len()
    }

    /// The ring holds a full `window_s` of frames: the next flip has a whole before window.
    pub fn ring_full(&self, cfg: &ProbeConfig) -> bool {
        self.ring_ms >= f64::from(cfg.window_s) * 1000.0
    }

    /// The switches the waiting flip measures (none when nothing waits).
    pub fn waiting(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.pending.iter().flat_map(|p| p.switches.iter().map(|&(name, _)| name))
    }

    /// Forgets the ring (recording stopped: the frames are no longer the last ones).
    pub fn clear_ring(&mut self) {
        self.ring.clear();
        self.ring_ms = 0.0;
    }

    /// Switches flipped this frame (after the frames pushed so far). The flip still waiting is
    /// superseded (returned, no delta); with an empty ring there is no before window: the switches
    /// come back at once with no delta and nothing waits.
    pub fn flip(&mut self, switches: &[(&'static str, bool)], cfg: &ProbeConfig) -> Vec<Outcome> {
        let mut out = Vec::new();
        if let Some(old) = self.pending.take() {
            out.extend(old.switches.iter().map(|&(name, running)| {
                let mut d = SwitchDelta::empty(running);
                d.before = old.before.counts;
                d.after = old.after.counts;
                d.moved = old.motion.moved;
                d.superseded = true;
                (name, d)
            }));
        }
        if switches.is_empty() {
            return out;
        }
        if self.ring.is_empty() {
            out.extend(switches.iter().map(|&(name, running)| (name, SwitchDelta::empty(running))));
            return out;
        }
        let mut before = Sums::default();
        let mut motion = Motion::default();
        for s in &self.ring {
            before.add(s);
            motion.see(s.pose, cfg);
        }
        self.pending = Some(Pending {
            switches: switches.to_vec(),
            before,
            after: Sums::default(),
            elapsed_ms: 0.0,
            after_ms: 0.0,
            motion,
        });
        out
    }

    /// One frame: into the ring, and into the waiting flip's settle or after window. Returns the
    /// flip's results once its after window is full.
    pub fn push(&mut self, s: Sample, cfg: &ProbeConfig) -> Vec<Outcome> {
        let window_ms = f64::from(cfg.window_s) * 1000.0;
        if s.frame_ms > 0.0 {
            self.ring.push_back(s);
            self.ring_ms += f64::from(s.frame_ms);
            while let Some(front) = self.ring.front()
                && self.ring_ms - f64::from(front.frame_ms) >= window_ms
            {
                self.ring_ms -= f64::from(front.frame_ms);
                self.ring.pop_front();
            }
        }
        let Some(p) = &mut self.pending else { return Vec::new() };
        if s.frame_ms <= 0.0 {
            return Vec::new();
        }
        p.motion.see(s.pose, cfg);
        p.elapsed_ms += f64::from(s.frame_ms);
        if p.elapsed_ms <= f64::from(cfg.settle_s) * 1000.0 {
            return Vec::new();
        }
        p.after.add(&s);
        p.after_ms += f64::from(s.frame_ms);
        if p.after_ms < window_ms {
            return Vec::new();
        }
        let p = self.pending.take().expect("pending");
        finish(&p, cfg)
    }
}

#[allow(clippy::cast_possible_truncation, reason = "ms deltas to f32")]
fn finish(p: &Pending, cfg: &ProbeConfig) -> Vec<Outcome> {
    let (b, a) = (p.before.means(), p.after.means());
    let delta = |i: usize| b[i].zip(a[i]).map(|(b, a)| (a - b) as f32);
    p.switches
        .iter()
        .map(|&(name, running)| {
            let frame_ms = delta(0);
            let d = SwitchDelta {
                running,
                frame_ms,
                main_ms: delta(1),
                render_ms: delta(2),
                gpu_ms: delta(3),
                before: p.before.counts,
                after: p.after.counts,
                noise: frame_ms.is_some_and(|f| f.abs() < cfg.noise_ms),
                moved: p.motion.moved,
                superseded: false,
            };
            (name, d)
        })
        .collect()
}

// ---- Quick A/B (slice FS-C1b) ----

/// Polls a flip may take to come back as its switch's `FeatureToggled` (the `SetTunable` queued
/// in `Update` applies in the next frame's `PreUpdate`), with one frame of slack.
const FLIP_POLLS: u32 = 3; // tunables: const (schedule latency, not a trade-off)

/// The quick A/B's settings (`diag.switch.ab_rounds`, `diag.switch.noise_ms`), read at its start.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AbConfig {
    /// Off/on pairs measured.
    pub rounds: u32,
    pub noise_ms: f32,
}

/// Why a quick A/B stopped before its last round.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AbAbort {
    /// A round's camera moved, and again in its retry.
    CameraMoving,
    /// A round had no delta (no before window), and again in its retry.
    NoMeasurement,
    /// A switch was flipped from elsewhere while it ran.
    Interrupted,
    /// The Features panel closed.
    Closed,
    /// The flip was refused, or changed nothing (the answer line).
    Refused(String),
}

impl AbAbort {
    /// `camera moving`, `interrupted`, ...
    pub fn text(&self) -> String {
        match self {
            AbAbort::CameraMoving => "camera moving".to_owned(),
            AbAbort::NoMeasurement => "no measurement".to_owned(),
            AbAbort::Interrupted => "interrupted".to_owned(),
            AbAbort::Closed => "panel closed".to_owned(),
            AbAbort::Refused(why) => why.clone(),
        }
    }
}

/// One flip's cost of the feature (ms): the delta when the flip turned it on, minus the delta when
/// it turned it off, so a feature that costs 1.5 ms gives +1.5 both ways.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cost {
    pub frame_ms: f32,
    pub main_ms: Option<f32>,
    pub render_ms: Option<f32>,
    pub gpu_ms: Option<f32>,
}

impl Cost {
    /// The cost a measured flip gives; `None` without a frame delta.
    pub fn of(d: &SwitchDelta) -> Option<Self> {
        let sign = if d.running { 1.0 } else { -1.0 };
        let signed = |v: Option<f32>| v.map(|v| sign * v);
        Some(Self {
            frame_ms: sign * d.frame_ms?,
            main_ms: signed(d.main_ms),
            render_ms: signed(d.render_ms),
            gpu_ms: signed(d.gpu_ms),
        })
    }
}

/// A finished quick A/B: the medians over its flips' costs (ms; positive = the feature costs that
/// much), the frame cost's spread, and the rounds it took.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AbCost {
    pub frame_ms: f32,
    pub main_ms: Option<f32>,
    pub render_ms: Option<f32>,
    pub gpu_ms: Option<f32>,
    /// The flips' frame costs, lowest and highest.
    pub min_ms: f32,
    pub max_ms: f32,
    /// Off/on pairs run, discarded ones included.
    pub rounds: u32,
    /// |median frame cost| under `diag.switch.noise_ms`.
    pub noise: bool,
}

/// How a quick A/B ended.
#[derive(Clone, Debug, PartialEq)]
pub enum AbOutcome {
    Done(AbCost),
    Aborted(AbAbort),
}

/// What the frame says, given to [`AbRun::poll`] once per frame.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[allow(clippy::struct_excessive_bools, reason = "independent facts of one frame")]
pub struct AbInput {
    /// The Features panel is open.
    pub open: bool,
    /// The probe has a full before window and waits on no flip.
    pub ready: bool,
    /// This switch's `FeatureToggled` this frame: its running state after.
    pub own: Option<bool>,
    /// Another switch toggled this frame, not alongside this one (a requirement's dependants
    /// toggle with it).
    pub others: bool,
    /// The probe's result generation.
    pub generation: u64,
    /// The probe still waits on this switch's flip.
    pub waiting: bool,
    /// The probe's last result for this switch.
    pub result: Option<SwitchDelta>,
}

/// What the driver does after a poll.
#[derive(Clone, Debug, PartialEq)]
pub enum AbAction {
    Wait,
    /// Set the switch on (`true`) or off at the console layer.
    Flip(bool),
    /// Done or aborted: put the switch back as it was before the run.
    Finish(AbOutcome),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    /// Waiting for the probe to be ready for the next flip.
    Ready,
    /// The flip was sent; waiting for its toggle.
    Sent { target: bool, polls: u32, generation: u64 },
    /// The probe measures the flip.
    Measuring { target: bool, generation: u64 },
}

/// A spoiled flip: its round is discarded.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Spoil {
    Moved,
    Empty,
}

fn aborted(why: AbAbort) -> AbAction {
    AbAction::Finish(AbOutcome::Aborted(why))
}

/// A quick A/B of one switch: `rounds` pairs of flips (away from its state at the start and
/// back), each flip measured by the probe; a round whose camera moved (or that had no delta) is
/// discarded and retried once. Every round ends with the switch back in its state at the start.
#[derive(Clone, Debug, PartialEq)]
pub struct AbRun {
    cfg: AbConfig,
    original: bool,
    running: bool,
    phase: Phase,
    pair: Vec<Cost>,
    pair_flips: u32,
    spoil: Option<Spoil>,
    retried: bool,
    costs: Vec<Cost>,
    rounds_done: u32,
    pairs_run: u32,
    flips_done: u32,
    extra_pairs: u32,
}

impl AbRun {
    /// A run of the switch, now running (`true`) or not.
    pub fn new(running: bool, cfg: AbConfig) -> Self {
        Self {
            cfg: AbConfig { rounds: cfg.rounds.max(1), ..cfg },
            original: running,
            running,
            phase: Phase::Ready,
            pair: Vec::new(),
            pair_flips: 0,
            spoil: None,
            retried: false,
            costs: Vec::new(),
            rounds_done: 0,
            pairs_run: 0,
            flips_done: 0,
            extra_pairs: 0,
        }
    }

    /// The switch's state at the start.
    pub fn original(&self) -> bool {
        self.original
    }

    /// The switch's state as of the last measured flip.
    pub fn running(&self) -> bool {
        self.running
    }

    /// Flips measured, of the flips planned (a retried round adds two).
    pub fn progress(&self) -> (u32, u32) {
        (self.flips_done, 2 * (self.cfg.rounds + self.extra_pairs))
    }

    /// One frame.
    pub fn poll(&mut self, input: &AbInput) -> AbAction {
        if !input.open {
            return aborted(AbAbort::Closed);
        }
        if input.others {
            return aborted(AbAbort::Interrupted);
        }
        match self.phase {
            Phase::Ready => {
                if input.own.is_some() {
                    return aborted(AbAbort::Interrupted);
                }
                if !input.ready {
                    return AbAction::Wait;
                }
                let target = !self.running;
                self.phase = Phase::Sent { target, polls: 0, generation: input.generation };
                AbAction::Flip(target)
            }
            Phase::Sent { target, polls, generation } => match input.own {
                Some(r) if r == target => {
                    self.phase = Phase::Measuring { target, generation };
                    self.measured(input, target, generation)
                }
                Some(_) => aborted(AbAbort::Interrupted),
                None if polls + 1 >= FLIP_POLLS => aborted(AbAbort::Refused("the flip changed nothing".to_owned())),
                None => {
                    self.phase = Phase::Sent { target, polls: polls + 1, generation };
                    AbAction::Wait
                }
            },
            Phase::Measuring { target, generation } => {
                if input.own.is_some() {
                    return aborted(AbAbort::Interrupted);
                }
                self.measured(input, target, generation)
            }
        }
    }

    /// The flip's result, once the probe stored it.
    fn measured(&mut self, input: &AbInput, target: bool, generation: u64) -> AbAction {
        if input.generation <= generation || input.waiting {
            return AbAction::Wait;
        }
        let cost = match input.result {
            Some(d) if d.superseded => return aborted(AbAbort::Interrupted),
            Some(d) if d.moved => Err(Spoil::Moved),
            Some(d) => Cost::of(&d).ok_or(Spoil::Empty),
            None => Err(Spoil::Empty),
        };
        match cost {
            Ok(c) => self.pair.push(c),
            // A moved flip names the abort over an empty one.
            Err(s) => self.spoil = Some(if self.spoil == Some(Spoil::Moved) { Spoil::Moved } else { s }),
        }
        self.running = target;
        self.flips_done += 1;
        self.pair_flips += 1;
        self.phase = Phase::Ready;
        if self.pair_flips < 2 {
            return AbAction::Wait;
        }
        self.end_pair()
    }

    fn end_pair(&mut self) -> AbAction {
        self.pairs_run += 1;
        self.pair_flips = 0;
        let pair = std::mem::take(&mut self.pair);
        if let Some(spoil) = self.spoil.take() {
            if self.retried {
                return aborted(match spoil {
                    Spoil::Moved => AbAbort::CameraMoving,
                    Spoil::Empty => AbAbort::NoMeasurement,
                });
            }
            self.retried = true;
            self.extra_pairs += 1;
            return AbAction::Wait;
        }
        self.retried = false;
        self.costs.extend(pair);
        self.rounds_done += 1;
        if self.rounds_done < self.cfg.rounds {
            return AbAction::Wait;
        }
        AbAction::Finish(AbOutcome::Done(self.cost()))
    }

    /// The medians over the kept flips.
    fn cost(&self) -> AbCost {
        let pick = |f: fn(&Cost) -> Option<f32>| median(self.costs.iter().filter_map(f).collect());
        let frames: Vec<f32> = self.costs.iter().map(|c| c.frame_ms).collect();
        let frame_ms = median(frames.clone()).unwrap_or(0.0);
        AbCost {
            frame_ms,
            main_ms: pick(|c| c.main_ms),
            render_ms: pick(|c| c.render_ms),
            gpu_ms: pick(|c| c.gpu_ms),
            min_ms: frames.iter().copied().fold(f32::INFINITY, f32::min),
            max_ms: frames.iter().copied().fold(f32::NEG_INFINITY, f32::max),
            rounds: self.pairs_run,
            noise: frame_ms.abs() < self.cfg.noise_ms,
        }
    }
}

/// The median (the mean of the middle two for an even count); `None` when empty.
pub fn median(mut v: Vec<f32>) -> Option<f32> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(f32::total_cmp);
    let mid = v.len() / 2;
    Some(if v.len().is_multiple_of(2) { f32::midpoint(v[mid - 1], v[mid]) } else { v[mid] })
}

#[cfg(test)]
#[path = "switch_probe_model_tests.rs"]
mod tests;
