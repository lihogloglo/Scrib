//! The bench's arm sequence: every arm of `--arms` in turn, in one launch, then exit.
//!
//! Each arm goes through three phases ([`Phase`], [`step`]): `Settle` (the previous arm's batches are gone; a few
//! whole frames so their reservations are released, then the arm's bytes are built and its batches spawned), `Warm`
//! (pipelines compile asynchronously and the first frames are slow; nothing is recorded) and `Measure` (one sample
//! per frame: frame time, main and render thread busy time, and every GPU span's newest timing). The arm's result
//! is printed as `bw2 ceiling |` lines (`key=value`, single spaces) when it ends. An arm's bytes are built only when
//! its turn comes, so at most one arm's bytes exist at a time.
//!
//! GPU total per frame: the top-level spans' timings ([`is_top_level`], the rule of `sf_diag`'s
//! `PassTiming::is_top_level`, so nested spans are not counted twice), each carried forward to frames without a new
//! measurement (the rule of `sf_diag::pacing::derive_work`).

use std::collections::{BTreeMap, HashMap};

use bevy::diagnostic::DiagnosticsStore;
use bevy::platform::time::Instant;
use bevy::prelude::*;
use bevy::render::renderer::{RenderAdapterInfo, RenderDevice};
use bevy::window::PrimaryWindow;
use sf_diag_bevy::DiagCounters;
use sf_diag_bevy::sf_diag::PassTiming;
use sf_diag_bevy::sf_diag::pacing::{MAIN_BUSY, RENDER_BUSY};

use crate::fill::{parse_arm, parse_spec};
use crate::stats::Series;
use crate::upload::CeilingUploads;
use crate::{CeilingAssets, CeilingBatch, spawn_arm};

/// Fixture: whole frames `Settle` lasts before the next arm is spawned (the despawned batches' reservations go).
pub const SETTLE_FRAMES: u32 = 3;
/// Fixture: fewest frames in `Warm` (the first frames compile pipelines and are slow).
pub const WARM_MIN_FRAMES: u32 = 30;
/// Fixture: fewest frames an arm cut short by `max_arm_s` must still have measured.
pub const CAPPED_MIN_FRAMES: u32 = 20;
/// Fixture: bytes per slot of a batch's largest GPU buffer, for the binding-limit warning.
pub const GPU_SLOT_BYTES: u64 = 176;
/// Fixture: the log prefix the orchestrator greps for.
const PREFIX: &str = "bw2 ceiling |";
/// Fixture: Bevy's render diagnostics prefix and GPU timing suffix (`render/<span path>/elapsed_gpu`).
const RENDER_PREFIX: &str = "render/";
const GPU_SUFFIX: &str = "/elapsed_gpu";
/// Fixture: the render thread's whole-schedule counter (`sf_diag_bevy` phases).
const RENDER_THREAD: &str = "render/thread/ms";

/// One arm's phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Settle,
    Warm,
    Measure,
}

/// What [`step`] decides for the current frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Stay,
    Next(Phase),
    /// `Measure` is over; `capped` when `max_arm_s` cut it short.
    ArmDone { capped: bool },
}

/// Phase lengths.
#[derive(Clone, Copy, Debug)]
pub struct BenchConfig {
    pub warm_s: f64,
    pub measure_s: f64,
    pub max_arm_s: f64,
    pub min_frames: u32,
}

/// The phase transition. `secs` and `frames` are the time and the frames spent in `phase`, the current frame
/// included.
pub fn step(phase: Phase, secs: f64, frames: u32, cfg: &BenchConfig) -> Step {
    match phase {
        Phase::Settle if frames > SETTLE_FRAMES => Step::Next(Phase::Warm),
        Phase::Warm if secs >= cfg.warm_s && frames >= WARM_MIN_FRAMES => Step::Next(Phase::Measure),
        Phase::Measure if secs >= cfg.measure_s && frames >= cfg.min_frames => Step::ArmDone { capped: false },
        Phase::Measure if secs >= cfg.max_arm_s && frames >= CAPPED_MIN_FRAMES => Step::ArmDone { capped: true },
        _ => Step::Stay,
    }
}

/// A comma list of arm specs, each validated with [`parse_spec`] and kept as its string (built at its turn).
pub fn parse_arms(list: &str) -> Result<Vec<String>, String> {
    list.split(',')
        .map(|item| {
            let item = item.trim();
            if item.is_empty() {
                return Err(format!("arms `{list}`: empty item"));
            }
            parse_spec(item).map(|_| item.to_owned())
        })
        .collect()
}

/// The span path of a GPU timing diagnostic (`render/<span path>/elapsed_gpu`), or `None` for any other path.
pub fn gpu_span(path: &str) -> Option<&str> {
    path.strip_suffix(GPU_SUFFIX).map(|p| p.strip_prefix(RENDER_PREFIX).unwrap_or(p))
}

/// Whether a span is top level (`sf_diag`'s rule: no `/` in its path below `render/`).
pub fn is_top_level(span: &str) -> bool {
    PassTiming {
        span: span.to_owned(),
        gpu_ms: None,
        gpu_p95_ms: None,
        gpu_frame_ms: None,
        runs_per_frame: None,
        cpu_ms: None,
    }
    .is_top_level()
}

/// Each binding limit a batch of `capacity` slots exceeds, as `(limit name, needed bytes, limit bytes)`.
pub fn limit_overruns(capacity: u32, limits: &AdapterLimits) -> Vec<(&'static str, u64, u64)> {
    let need = u64::from(capacity) * GPU_SLOT_BYTES;
    [("max_storage_buffer_binding_size", limits.max_storage_buffer_binding_size), ("max_buffer_size", limits.max_buffer_size)]
        .into_iter()
        .filter(|&(_, limit)| need > limit)
        .map(|(name, limit)| (name, need, limit))
        .collect()
}

/// A log value: spaces become `_` so lines stay `key=value` separated by single spaces.
pub fn token(s: &str) -> String {
    s.replace(' ', "_")
}

/// The two device limits the bench checks a batch against.
#[derive(Clone, Copy, Debug)]
pub struct AdapterLimits {
    pub max_storage_buffer_binding_size: u64,
    pub max_buffer_size: u64,
}

/// One GPU span's samples in the current arm.
#[derive(Default)]
struct PassRun {
    samples: Series,
    top: bool,
    /// The newest timing, carried forward to frames without a new one.
    last: Option<f64>,
}

/// The arm being run.
struct ArmRun {
    name: String,
    slots: u64,
    frame_ms: Series,
    main_ms: Series,
    render_ms: Series,
    render_thread_ms: Series,
    gpu_ms: Series,
    passes: BTreeMap<String, PassRun>,
    focus_lost: u64,
    measured_s: f64,
}

impl ArmRun {
    fn new(name: String, slots: u64) -> Self {
        Self {
            name,
            slots,
            frame_ms: Series::default(),
            main_ms: Series::default(),
            render_ms: Series::default(),
            render_thread_ms: Series::default(),
            gpu_ms: Series::default(),
            passes: BTreeMap::new(),
            focus_lost: 0,
            measured_s: 0.0,
        }
    }

    fn print(&self, capped: bool, extra: &str) {
        let name = &self.name;
        println!(
            "{PREFIX} arm={name} result slots={} frames={} measured_s={:.3} frame_median_ms={:.3} frame_p99_ms={:.3} \
             main_ms={:.3} render_ms={:.3} render_thread_ms={:.3} gpu_ms={:.3} focus_lost_frames={} capped={capped}{extra}",
            self.slots,
            self.frame_ms.count(),
            self.measured_s,
            self.frame_ms.median(),
            self.frame_ms.p99(),
            self.main_ms.median(),
            self.render_ms.median(),
            self.render_thread_ms.median(),
            self.gpu_ms.median(),
            self.focus_lost,
        );
        for (span, pass) in self.passes.iter().filter(|(_, p)| p.samples.count() > 0) {
            println!(
                "{PREFIX} arm={name} pass={} gpu_median_ms={:.3} gpu_p95_ms={:.3} samples={} top={}",
                token(span),
                pass.samples.median(),
                pass.samples.p95(),
                pass.samples.count(),
                pass.top,
            );
        }
    }
}

/// The run: what is left to do and where it stands.
#[derive(Resource)]
pub struct Bench {
    cfg: BenchConfig,
    max_total_s: f64,
    arms: Vec<String>,
    next: usize,
    phase: Phase,
    phase_start_s: f64,
    phase_frames: u32,
    run_start_s: Option<f64>,
    arm: Option<ArmRun>,
    limits: Option<AdapterLimits>,
    /// Newest measurement time seen per diagnostic path, over the whole run (a pass that stopped running never
    /// counts its old timing again).
    seen: HashMap<String, Instant>,
    arms_run: usize,
    focus_lost_total: u64,
    finished: bool,
}

impl Bench {
    pub fn new(cfg: BenchConfig, max_total_s: f64, arms: Vec<String>) -> Self {
        Self {
            cfg,
            max_total_s,
            arms,
            next: 0,
            phase: Phase::Settle,
            phase_start_s: 0.0,
            phase_frames: 0,
            run_start_s: None,
            arm: None,
            limits: None,
            seen: HashMap::new(),
            arms_run: 0,
            focus_lost_total: 0,
            finished: false,
        }
    }

    fn enter(&mut self, phase: Phase, now_s: f64) {
        self.phase = phase;
        self.phase_start_s = now_s;
        self.phase_frames = 0;
    }

    fn done_line(&self, now_s: f64, extra: &str) {
        let total = now_s - self.run_start_s.unwrap_or(now_s);
        let focus = if self.focus_lost_total == 0 { "kept" } else { "lost" };
        println!(
            "{PREFIX} done arms={} total_s={total:.3} focus_lost_frames={} focus={focus}{extra}",
            self.arms_run, self.focus_lost_total
        );
    }
}

/// Startup: the adapter, its limits and the run's settings.
pub fn print_header(mut bench: ResMut<Bench>, info: Option<Res<RenderAdapterInfo>>, device: Option<Res<RenderDevice>>) {
    let (adapter, backend) =
        info.map_or_else(|| ("unknown".to_owned(), "unknown".to_owned()), |i| (token(&i.name), i.backend.to_string()));
    let limits = device.map(|d| {
        let l = d.limits();
        AdapterLimits { max_storage_buffer_binding_size: l.max_storage_buffer_binding_size, max_buffer_size: l.max_buffer_size }
    });
    let (storage, buffer) = limits.map_or_else(
        || ("unknown".to_owned(), "unknown".to_owned()),
        |l| (l.max_storage_buffer_binding_size.to_string(), l.max_buffer_size.to_string()),
    );
    let cfg = bench.cfg;
    println!(
        "{PREFIX} adapter={adapter} backend={backend} max_storage_buffer_binding_size={storage} max_buffer_size={buffer} \
         arms={} warm_s={} measure_s={} max_arm_s={}",
        bench.arms.join(","),
        cfg.warm_s,
        cfg.measure_s,
        cfg.max_arm_s
    );
    bench.limits = limits;
}

/// Every frame: drains the phase counters (they accumulate until drained), reads each GPU span's newest timing once,
/// and, while measuring, records the frame.
pub fn sample_frame(
    mut bench: ResMut<Bench>,
    time: Res<Time<Real>>,
    store: Res<DiagnosticsStore>,
    counters: Res<DiagCounters>,
) {
    let (mut main, mut render, mut render_thread) = (None, None, None);
    counters.drain(|_, def, value| match def.name.as_str() {
        MAIN_BUSY => main = value,
        RENDER_BUSY => render = value,
        RENDER_THREAD => render_thread = value,
        _ => {}
    });

    let bench = &mut *bench;
    let measuring = bench.phase == Phase::Measure;
    for diagnostic in store.iter() {
        let path = diagnostic.path().as_str();
        let Some(span) = gpu_span(path) else { continue };
        let Some(m) = diagnostic.measurement() else { continue };
        match bench.seen.get_mut(path) {
            Some(last) if *last >= m.time => continue,
            Some(last) => *last = m.time,
            None => {
                bench.seen.insert(path.to_owned(), m.time);
            }
        }
        let Some(arm) = bench.arm.as_mut() else { continue };
        let pass = arm
            .passes
            .entry(span.to_owned())
            .or_insert_with(|| PassRun { top: is_top_level(span), ..PassRun::default() });
        pass.last = Some(m.value);
        if measuring {
            pass.samples.push(m.value);
        }
    }

    let Some(arm) = bench.arm.as_mut().filter(|_| measuring) else { return };
    arm.frame_ms.push(time.delta_secs_f64() * 1000.0);
    for (series, value) in [(&mut arm.main_ms, main), (&mut arm.render_ms, render), (&mut arm.render_thread_ms, render_thread)] {
        if let Some(v) = value {
            series.push(v);
        }
    }
    let top: Vec<f64> = arm.passes.values().filter(|p| p.top).filter_map(|p| p.last).collect();
    if !top.is_empty() {
        arm.gpu_ms.push(top.iter().sum());
    }
}

/// `Warm` and `Measure`: counts frames where the primary window lost focus.
pub fn count_focus(mut bench: ResMut<Bench>, windows: Query<&Window, With<PrimaryWindow>>) {
    if bench.phase == Phase::Settle || windows.single().is_ok_and(|w| w.focused) {
        return;
    }
    bench.focus_lost_total += 1;
    if let Some(arm) = bench.arm.as_mut() {
        arm.focus_lost += 1;
    }
}

/// The phase machine: spawns, ends and switches arms, prints their lines, and exits when every arm ran (or the
/// watchdog fired).
pub fn advance(
    mut commands: Commands,
    mut bench: ResMut<Bench>,
    time: Res<Time<Real>>,
    assets: Res<CeilingAssets>,
    mut uploads: ResMut<CeilingUploads>,
    batches: Query<Entity, With<CeilingBatch>>,
    mut exit: MessageWriter<AppExit>,
) {
    if bench.finished {
        return;
    }
    let now = time.elapsed_secs_f64();
    let start = *bench.run_start_s.get_or_insert(now);
    if now - start > bench.max_total_s {
        if let Some(arm) = bench.arm.as_ref().filter(|a| a.frame_ms.count() > 0) {
            arm.print(true, " aborted=watchdog");
        }
        bench.done_line(now, " aborted=watchdog");
        bench.finished = true;
        exit.write(AppExit::Success);
        return;
    }

    bench.phase_frames += 1;
    let secs = now - bench.phase_start_s;
    match step(bench.phase, secs, bench.phase_frames, &bench.cfg) {
        Step::Stay => {}
        Step::Next(Phase::Measure) => bench.enter(Phase::Measure, now),
        Step::Next(_) => {
            let spec = bench.arms[bench.next].clone();
            bench.next += 1;
            let built = Instant::now();
            let arm = parse_arm(&spec).expect("validated by parse_arms");
            let build_ms = built.elapsed().as_secs_f64() * 1000.0;
            if let Some(limits) = bench.limits {
                for batch in &arm.batches {
                    for (which, need, limit) in limit_overruns(batch.capacity, &limits) {
                        println!(
                            "{PREFIX} arm={} warn=limit capacity={} need_bytes={need} limit={limit} which={which}",
                            arm.name, batch.capacity
                        );
                    }
                }
            }
            let (name, slots, count) = (arm.name.clone(), arm.total_slots(), arm.batches.len());
            spawn_arm(&mut commands, &assets, &mut uploads, arm);
            println!("{PREFIX} arm={name} start slots={slots} batches={count} build_ms={build_ms:.3}");
            bench.arm = Some(ArmRun::new(name, slots));
            bench.enter(Phase::Warm, now);
        }
        Step::ArmDone { capped } => {
            if let Some(mut arm) = bench.arm.take() {
                arm.measured_s = secs;
                arm.print(capped, "");
            }
            bench.arms_run += 1;
            for entity in &batches {
                commands.entity(entity).despawn();
            }
            if bench.next < bench.arms.len() {
                bench.enter(Phase::Settle, now);
            } else {
                bench.done_line(now, "");
                bench.finished = true;
                exit.write(AppExit::Success);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CFG: BenchConfig = BenchConfig { warm_s: 4.0, measure_s: 12.0, max_arm_s: 60.0, min_frames: 120 };

    #[test]
    fn settle_lasts_whole_frames_then_warms() {
        for f in 1..=SETTLE_FRAMES {
            assert_eq!(step(Phase::Settle, 100.0, f, &CFG), Step::Stay, "frame {f}");
        }
        assert_eq!(step(Phase::Settle, 0.0, SETTLE_FRAMES + 1, &CFG), Step::Next(Phase::Warm));
    }

    #[test]
    fn warm_needs_time_and_frames() {
        assert_eq!(step(Phase::Warm, 3.9, 500, &CFG), Step::Stay);
        assert_eq!(step(Phase::Warm, 30.0, WARM_MIN_FRAMES - 1, &CFG), Step::Stay);
        assert_eq!(step(Phase::Warm, 4.0, WARM_MIN_FRAMES, &CFG), Step::Next(Phase::Measure));
    }

    #[test]
    fn measure_needs_time_and_min_frames() {
        assert_eq!(step(Phase::Measure, 11.9, 1000, &CFG), Step::Stay);
        assert_eq!(step(Phase::Measure, 12.0, 119, &CFG), Step::Stay);
        assert_eq!(step(Phase::Measure, 12.0, 120, &CFG), Step::ArmDone { capped: false });
    }

    #[test]
    fn measure_cap_ends_a_slow_arm_with_enough_frames() {
        // 40 frames in 59 s: neither rule yet.
        assert_eq!(step(Phase::Measure, 59.0, 40, &CFG), Step::Stay);
        assert_eq!(step(Phase::Measure, 60.0, 40, &CFG), Step::ArmDone { capped: true });
        // The cap still waits for its own frame minimum.
        assert_eq!(step(Phase::Measure, 90.0, CAPPED_MIN_FRAMES - 1, &CFG), Step::Stay);
        assert_eq!(step(Phase::Measure, 90.0, CAPPED_MIN_FRAMES, &CFG), Step::ArmDone { capped: true });
        // A full arm past the cap is not called capped.
        assert_eq!(step(Phase::Measure, 61.0, 120, &CFG), Step::ArmDone { capped: false });
    }

    #[test]
    fn parse_arms_keeps_strings_and_rejects_bad_items() {
        assert_eq!(
            parse_arms("empty,card:10, tree:5"),
            Ok(vec!["empty".to_owned(), "card:10".to_owned(), "tree:5".to_owned()])
        );
        assert_eq!(parse_arms("tree:1000000"), Ok(vec!["tree:1000000".to_owned()]));
        for junk in ["", "empty,", "card:10,,tree:5", "card:0", "empty,bush:3"] {
            assert!(parse_arms(junk).is_err(), "`{junk}` was accepted");
        }
    }

    #[test]
    fn gpu_spans_and_top_level() {
        assert_eq!(gpu_span("render/main_opaque_pass_3d/elapsed_gpu"), Some("main_opaque_pass_3d"));
        assert_eq!(gpu_span("render/shadows/cascade_0/elapsed_gpu"), Some("shadows/cascade_0"));
        assert_eq!(gpu_span("render/early prepass/elapsed_gpu"), Some("early prepass"));
        assert_eq!(gpu_span("render/shadows/elapsed_cpu"), None);
        assert_eq!(gpu_span("frame_time"), None);
        assert!(is_top_level("main_opaque_pass_3d"));
        assert!(is_top_level("early prepass"));
        assert!(!is_top_level("shadows/cascade_0"));
        assert_eq!(token("early prepass"), "early_prepass");
    }

    #[test]
    fn limit_overruns_name_each_exceeded_limit() {
        let limits = AdapterLimits { max_storage_buffer_binding_size: 128 << 20, max_buffer_size: 256 << 20 };
        assert!(limit_overruns(100_000, &limits).is_empty());
        // 1,000,000 x 176 = 176,000,000 bytes: over 128 MiB, under 256 MiB.
        assert_eq!(
            limit_overruns(1_000_000, &limits),
            vec![("max_storage_buffer_binding_size", 176_000_000, 128 << 20)]
        );
        assert_eq!(limit_overruns(2_000_000, &limits).len(), 2);
    }
}
