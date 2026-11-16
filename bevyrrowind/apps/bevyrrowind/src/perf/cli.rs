//! `bevyrrowind perf ...`: the suite runner and the trace / Nsight
//! summaries. No Python: everything the Ovid scripts did lives here, on
//! `sf_diag`'s Bevy-free helpers.

use std::fmt::Write as _;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use super::ab::{self as arms, Arm, ArmRun, RunSettings};
use super::sweep::SweepArgs;
use mw_devtools_bevy::perf::FocusTally;
use sf_diag_bevy::sf_diag::suite::{self, GaugeColumn, GpuSampler, GpuSummary, SuiteRun};
use sf_diag_bevy::sf_diag::ab::{RunPoint, interleaved_orders, outlier_reason, outliers};
use sf_diag_bevy::sf_diag::{Report, Scenario, nsys, trace};

/// A named benchmark: `bevyrrowind` arguments (run from the repo root).
pub struct SuiteScenario {
    pub name: &'static str,
    pub about: &'static str,
    pub args: &'static [&'static str],
}

/// The named scenarios. Exterior cameras are world meters (`+h` = h above
/// the ground); fly-through paths live in the scenario files.
pub const SCENARIOS: &[SuiteScenario] = &[
    SuiteScenario {
        name: "balmora",
        about: "Balmora standing: default view, 150 m south-east, 60 m up",
        args: &["view", "Balmora", "--bench", "benchmarks/exterior_balmora.json"],
    },
    SuiteScenario {
        name: "seyda_neen",
        about: "Seyda Neen docks standing: from the water, docks, ship and town",
        args: &["view", "Seyda Neen", "--camera", "-105,+8,1100,-140,+3,1035", "--bench", "benchmarks/exterior_seyda_neen.json"],
    },
    SuiteScenario {
        name: "red_mountain",
        about: "horizon view: 70 m over the hills by Balmora, toward Red Mountain",
        args: &["view", "Balmora", "--camera", "-120,+70,80,293,250,-878", "--bench", "benchmarks/exterior_red_mountain.json"],
    },
    SuiteScenario {
        name: "run",
        about: "run at 7 m/s, 1.7 m above the ground, through Seyda Neen",
        args: &["view", "Seyda Neen", "--bench", "benchmarks/exterior_run.json", "--bench-ground"],
    },
    SuiteScenario {
        name: "balmora_walk",
        about: "walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)",
        args: &["view", "Balmora", "--bench", "benchmarks/exterior_balmora_walk.json"],
    },
    SuiteScenario {
        name: "balmora_night",
        about: "Balmora standing at 22:00, clear: torch-bearing walkers, far light sprites, moon shadows",
        args: &["view", "Balmora", "--hour", "22", "--weather", "clear", "--bench", "benchmarks/exterior_balmora.json"],
    },
    SuiteScenario {
        name: "balmora_walk_night",
        about: "balmora_walk at 22:00, clear: walkers carry lit torches (flame emitters), lamps light the street",
        args: &["view", "Balmora", "--hour", "22", "--weather", "clear", "--bench", "benchmarks/exterior_balmora_walk.json"],
    },
    SuiteScenario {
        name: "flight",
        about: "flight at 60 m/s, 120 m above the ground, Seyda Neen - Balmora - Red Mountain - Vivec",
        args: &["view", "Seyda Neen", "--bench", "benchmarks/exterior_flight.json", "--bench-ground"],
    },
    SuiteScenario {
        name: "census",
        about: "interior: Seyda Neen Census and Excise Office walk-through",
        args: &["view", "Seyda Neen, Census and Excise Office", "--bench", "benchmarks/census_office.json"],
    },
    SuiteScenario {
        name: "light_lab",
        about: "light-cost lab (level light-lab): closed room, 8 lamps, two walkers, fixed `wide` camera; `perf sweep` steps one variable",
        args: &["view", "level:light-lab", "--camera", "wide", "--bench", "benchmarks/light_lab.json"],
    },
];

/// The resolution of a CPU-floor run: small enough that the GPU drops out.
const FLOOR_RESOLUTION: &str = "960x600";

/// Processes that make numbers noise by name: builds, other game instances. Renamed copies are caught by
/// where they live ([`busy_rules`]).
pub(super) const BUSY: &[&str] = &["cargo", "rustc", "bevyrrowind", "ovid"];

/// Another process at or above this SM share (`nvidia-smi pmon`) is GPU work competing with the run: the
/// same 15 % the quiet check allows for the whole GPU before a run. On this laptop's driver (WDDM) every
/// windowed app is listed by `pmon`, idle at `-`, and the desktop UI shows 0-2 %, so being listed is not load.
const GPU_BUSY_PCT: f64 = 15.0;

/// What counts as busy: [`BUSY`] and `--wait-for` names, plus any exe under the cargo target dir (also reached
/// through `target\` junctions) or the temp folder, whatever it is called (`orch-a.exe`, `h1gpucull.exe`, owner
/// copies in `%TEMP%\bw-owner-*`); installed cargo/rustup tools there (MCP servers) are exempt.
pub(super) fn busy_rules(args: &SuiteArgs) -> suite::BusyRules {
    let mut dirs: Vec<PathBuf> = vec![PathBuf::from("target"), PathBuf::from("../target"), std::env::temp_dir()];
    dirs.extend(["CARGO_TARGET_DIR", "TEMP", "TMP"].iter().filter_map(std::env::var_os).map(PathBuf::from));
    if let Some(root) = super::chain::repo_root() {
        dirs.push(root.join("target"));
    }
    dirs.retain(|d| d.exists());
    suite::BusyRules::new(BUSY.iter().copied().chain(args.wait_for.iter().map(String::as_str)), dirs)
}

/// Takes the machine for a chain of runs: the cross-session bench lock and the power plan (`chain.rs`); a dry
/// run takes nothing. Keep the result alive until the summary is written.
pub(super) fn start_chain(args: &SuiteArgs, exe: &Path) -> Result<super::chain::Chain, Box<dyn std::error::Error>> {
    if args.dry_run {
        return Ok(super::chain::Chain::dry());
    }
    let what = std::env::args().map(|a| if a.contains(' ') { format!("\"{a}\"") } else { a }).collect::<Vec<_>>().join(" ");
    super::chain::Chain::begin(&what, exe, args.no_lock)
}

#[derive(clap::Subcommand)]
pub enum PerfCommand {
    /// List the suite's named scenarios.
    List,
    /// Print a scenario's `bevyrrowind` arguments, one per line (for scripts).
    Args { scenario: String },
    /// Step one variable per run over a scenario (the light lab, docs/perf.md "Light lab"):
    /// `perf sweep <tag> light_lab --var point-shadows --values 0,1,2,4,8 --repeat 3`; writes
    /// `sweep.md` and `sweep.csv` (medians over repeats) beside the suite summary.
    Sweep(Box<SweepArgs>),
    /// Run named scenarios (all by default) and summarize them. Run from
    /// the repo root with the profiling build:
    /// `target/profiling/bevyrrowind.exe perf suite <tag> balmora --repeat 3`.
    Suite(Box<SuiteArgs>),
    /// CPU time per span (system) from a `trace_chrome` trace over a window.
    Trace {
        file: PathBuf,
        /// Window start, seconds since the trace began (default: the run's
        /// measured window when the run's report sits next to the trace).
        #[arg(long)]
        from: Option<f64>,
        #[arg(long)]
        to: Option<f64>,
        /// Rows in the self-time table.
        #[arg(long, default_value_t = 60)]
        top: usize,
    },
    /// The long spans behind frame spikes in a `trace_chrome` trace.
    Spikes {
        file: PathBuf,
        #[arg(long)]
        from: Option<f64>,
        #[arg(long)]
        to: Option<f64>,
        /// Spans shorter than this are ignored.
        #[arg(long, default_value_t = 3.0)]
        min_ms: f64,
    },
    /// Per-pass GPU table from Nsight's `vulkan_gpu_marker_sum` CSV
    /// (`tools/perf_nsys.ps1` calls this).
    Nsys {
        csv: PathBuf,
        /// Frames in the capture (default: the most common instance count).
        #[arg(long)]
        frames: Option<u64>,
        /// Also write the table here.
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

#[derive(clap::Args)]
#[expect(clippy::struct_excessive_bools, reason = "independent command-line flags")]
pub struct SuiteArgs {
    /// Name of this suite run: results go to `<out>/<tag>/`.
    pub(super) tag: String,
    /// Scenarios to run (default: all); see `perf list`.
    pub(super) scenarios: Vec<String>,
    /// Runs per scenario and resolution.
    #[arg(long, default_value_t = 1)]
    pub(super) repeat: usize,
    /// Window sizes, `WxH` (repeatable). Default `1280x720`.
    #[arg(long = "resolution")]
    pub(super) resolutions: Vec<String>,
    /// Also run each scenario at 960x600, where the GPU drops out: the CPU floor.
    #[arg(long)]
    floor: bool,
    /// Per-run settings for every run (`--off`, `--env`, `--arg`); an `--arm` adds its own on top.
    #[command(flatten)]
    pub(super) run: RunSettings,
    /// Interleaved A/B arm `NAME:SETTINGS` (repeatable, two or more; the first is the base). SETTINGS are this
    /// command's own `--off=a,b`, `--env K=V`, `--arg=--flag` (whitespace-separated, `base:` for none). Arms run
    /// in counterbalanced ABBA / BAAB order per repeat on the same exe, after one discarded warm-up run per arm;
    /// the summary adds per-arm verdicts vs the first arm (docs/perf.md "Interleaved A/B").
    #[arg(long = "arm")]
    pub(super) arms: Vec<String>,
    /// Seed of the arm order (default: from the clock; printed in the summary to replay the plan).
    #[arg(long)]
    seed: Option<u64>,
    /// Hitch threshold in ms (default 2.5 × median, at least 25 ms).
    #[arg(long)]
    hitch_ms: Option<f64>,
    /// The game executable (default: this one).
    #[arg(long)]
    pub(super) exe: Option<PathBuf>,
    /// Write a Chrome trace per run (`TRACE_CHROME`; needs an exe built with
    /// `--features trace`).
    #[arg(long)]
    trace: bool,
    /// Results root.
    #[arg(long, default_value = "target/perf")]
    pub(super) out: PathBuf,
    /// Also copy the summary and each run's report (md + json) here, e.g.
    /// `benchmarks/runs/<tag>`.
    #[arg(long)]
    pub(super) save: Option<PathBuf>,
    /// More process names to wait for (comma-separated).
    #[arg(long, value_delimiter = ',')]
    pub(super) wait_for: Vec<String>,
    /// Start runs even when the machine is busy (results are flagged).
    #[arg(long)]
    no_wait: bool,
    /// Longest wait for a quiet machine per run, minutes.
    #[arg(long, default_value_t = 120.0)]
    max_wait_min: f64,
    /// Re-run a scenario this many times when other processes were busy during it.
    #[arg(long, default_value_t = 2)]
    retries: usize,
    /// Print the commands without running them (takes no bench lock, leaves the power plan alone).
    #[arg(long)]
    pub(super) dry_run: bool,
    /// Don't take the cross-session bench lock (`.sessions/bench.lock`): debugging the runner only; the
    /// summary says another bench may have overlapped.
    #[arg(long)]
    no_lock: bool,
}

pub fn run(command: PerfCommand) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        PerfCommand::List => {
            for s in SCENARIOS {
                println!("{:<14} {}\n{:<14} bevyrrowind {}", s.name, s.about, "", quoted(s.args));
            }
            Ok(())
        }
        PerfCommand::Args { scenario } => {
            let s = SCENARIOS.iter().find(|s| s.name == scenario).ok_or_else(|| format!("unknown scenario {scenario:?}; see `perf list`"))?;
            for a in s.args {
                println!("{a}");
            }
            Ok(())
        }
        PerfCommand::Suite(args) => run_suite(&args),
        PerfCommand::Sweep(args) => super::sweep::run(&args),
        PerfCommand::Trace { file, from, to, top } => {
            let (from, to) = window(&file, from, to)?;
            let summary = trace::summarize(BufReader::new(File::open(&file)?), from, to)?;
            print!("{}", summary.to_text(top, 160));
            Ok(())
        }
        PerfCommand::Spikes { file, from, to, min_ms } => {
            let (from, to) = window(&file, from, to)?;
            let spans = trace::long_spans(BufReader::new(File::open(&file)?), from, to, min_ms)?;
            println!("spans over {min_ms:.1} ms between {from:.1} and {to:.1} s (by total time)");
            for s in spans {
                println!(
                    "{:5}x  worst {:7.1} ms (ends {:7.2} s)  mean {:6.1} ms  {}",
                    s.count,
                    s.worst_us / 1e3,
                    s.worst_end_s,
                    s.mean_us / 1e3,
                    s.name.chars().take(180).collect::<String>()
                );
            }
            Ok(())
        }
        PerfCommand::Nsys { csv, frames, out } => {
            let rows = nsys::parse_marker_sum(&std::fs::read_to_string(&csv)?)?;
            let md = format!("# GPU passes (Nsight Systems)\n\n`{}`\n\n{}", csv.display(), nsys::marker_markdown(&rows, frames));
            print!("{md}");
            if let Some(out) = out {
                std::fs::write(out, md)?;
            }
            Ok(())
        }
    }
}

fn quoted(args: &[&str]) -> String {
    args.iter().map(|a| if a.contains([' ', ',']) { format!("\"{a}\"") } else { (*a).to_owned() }).collect::<Vec<_>>().join(" ")
}

/// The trace window: explicit, or the measured window of the run whose
/// files sit next to the trace (`run.txt` holds the launch time).
fn window(file: &Path, from: Option<f64>, to: Option<f64>) -> Result<(f64, f64), Box<dyn std::error::Error>> {
    if let (Some(f), Some(t)) = (from, to) {
        return Ok((f, t));
    }
    let dir = file.parent().unwrap_or(Path::new("."));
    let launch = std::fs::read_to_string(dir.join("run.txt"))
        .ok()
        .and_then(|t| t.lines().find_map(|l| l.strip_prefix("launch_unix_s=").and_then(|v| v.trim().parse::<f64>().ok())));
    let report = std::fs::read_dir(dir)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "json") && p != file)
        .find_map(|p| Report::load(&p).ok());
    match (launch, report.as_ref().and_then(|r| r.wall_start_unix_s.zip(r.wall_end_unix_s))) {
        (Some(launch), Some((start, end))) => {
            let (f, t) = (from.unwrap_or(start - launch), to.unwrap_or(end - launch));
            println!("window {f:.1}..{t:.1} s: the measured window (trace time starts near process launch)");
            Ok((f, t))
        }
        _ => Ok((from.unwrap_or(0.0), to.unwrap_or(f64::INFINITY))),
    }
}

/// A finished run: its busy-machine note (the quiet wait gave up), why it should be measured again
/// ([`retry_reason`]), and whether its cleanliness was fully checked (other-process CPU and focus recorded).
pub(super) struct Measured {
    run: SuiteRun,
    note: Option<String>,
    retry: Option<&'static str>,
    checked: bool,
}

type RunResult = Result<Option<Measured>, Box<dyn std::error::Error>>;

/// Why a run's numbers aren't clean and it is worth measuring again: other
/// processes were busy during it, or its window was out of focus or hidden
/// for some measured frames ([`FocusTally`]). `None`: clean, or focus not
/// recorded (an exe older than the guard).
fn retry_reason(busy: bool, focus: Option<FocusTally>) -> Option<&'static str> {
    match (busy, focus.is_some_and(|f| f.is_dirty())) {
        (false, false) => None,
        (true, false) => Some("busy during the run"),
        (false, true) => Some("window out of focus during the run"),
        (true, true) => Some("busy and window out of focus during the run"),
    }
}

/// A run report's focus label, bold when dirty.
fn focus_md(f: FocusTally) -> String {
    if f.is_dirty() { format!("**{}**", f.label()) } else { f.label() }
}

/// A run's name and its focus record, for [`focus_lines`].
pub(super) fn run_focus(r: &SuiteRun) -> (String, Option<FocusTally>) {
    (run_name(&r.scenario, &r.variant, r.repeat), FocusTally::from_report(&r.report))
}

/// Summary lines on window focus: one when every run was clean, else a
/// count, one `**unfocused**` line per dirty run and the runs not recorded.
pub(super) fn focus_lines(runs: &[(String, Option<FocusTally>)]) -> Vec<String> {
    let dirty: Vec<String> = runs
        .iter()
        .filter_map(|(name, f)| f.filter(FocusTally::is_dirty).map(|f| format!("- **unfocused**: {name}: {}", f.label())))
        .collect();
    let unknown: Vec<&str> = runs.iter().filter(|(_, f)| f.is_none()).map(|(name, _)| name.as_str()).collect();
    let clean = runs.len() - dirty.len() - unknown.len();
    if dirty.is_empty() && unknown.is_empty() {
        return vec![format!("- focus: clean, all {} runs (window in front for every measured frame)", runs.len())];
    }
    let mut lines = vec![format!("- focus: {clean} of {} runs clean", runs.len())];
    lines.extend(dirty);
    if !unknown.is_empty() {
        lines.push(format!("- focus: not recorded for {} (exe older than the focus guard)", unknown.join(", ")));
    }
    lines
}

/// Median CPU % of other processes during a run (system minus ours) above
/// which the run counts as measured on a busy machine.
const OTHER_CPU_LIMIT: f64 = 25.0;

/// One planned run.
pub(super) struct Planned {
    pub(super) scenario: &'static SuiteScenario,
    pub(super) resolution: String,
    pub(super) repeat: usize,
    /// `perf sweep`: this run's value of the swept variable.
    pub(super) step: Option<super::sweep::Step>,
    /// Interleaved A/B: the arm (index into the suite's arms) and its settings.
    pub(super) arm: Option<(usize, Arm)>,
    /// A discarded warm-up run (driver and file caches, first launch): never in the summary.
    pub(super) warmup: bool,
}

/// The run plan. Without arms: scenario → resolution → repeat. With arms: one warm-up run per arm (first
/// scenario and resolution), then per scenario and resolution each repeat runs every arm in the counterbalanced
/// order of [`interleaved_orders`] (ABBA / BAAB with two arms).
pub(super) fn plan_runs(chosen: &[&'static SuiteScenario], resolutions: &[String], repeats: usize, arms: &[Arm], seed: u64) -> Vec<Planned> {
    let repeats = repeats.max(1);
    let mut plan = Vec::new();
    if arms.is_empty() {
        for &scenario in chosen {
            for resolution in resolutions {
                for repeat in 1..=repeats {
                    plan.push(Planned { scenario, resolution: resolution.clone(), repeat, step: None, arm: None, warmup: false });
                }
            }
        }
        return plan;
    }
    let orders = interleaved_orders(arms.len(), repeats, seed);
    if let (Some(&scenario), Some(resolution)) = (chosen.first(), resolutions.first()) {
        for &a in &orders[0] {
            plan.push(Planned { scenario, resolution: resolution.clone(), repeat: 0, step: None, arm: Some((a, arms[a].clone())), warmup: true });
        }
    }
    for &scenario in chosen {
        for resolution in resolutions {
            for (r, order) in orders.iter().enumerate() {
                for &a in order {
                    plan.push(Planned { scenario, resolution: resolution.clone(), repeat: r + 1, step: None, arm: Some((a, arms[a].clone())), warmup: false });
                }
            }
        }
    }
    plan
}

fn run_suite(args: &SuiteArgs) -> Result<(), Box<dyn std::error::Error>> {
    let exe = match &args.exe {
        Some(p) => p.clone(),
        None => std::env::current_exe()?,
    };
    if !Path::new("benchmarks").is_dir() {
        return Err("run from the bevyrrowind repo root (scenario paths are relative to it)".into());
    }
    arms::check_save(args.save.as_deref())?;
    args.run.env_pairs()?;
    let arm_list = arms::parse_arms(&args.arms)?;
    let names: Vec<&str> = if args.scenarios.is_empty() { SCENARIOS.iter().map(|s| s.name).collect() } else { args.scenarios.iter().map(String::as_str).collect() };
    let mut chosen = Vec::new();
    for name in names {
        chosen.push(SCENARIOS.iter().find(|s| s.name == name).ok_or_else(|| format!("unknown scenario {name:?}; see `perf list`"))?);
    }
    let mut resolutions = if args.resolutions.is_empty() { vec!["1280x720".to_owned()] } else { args.resolutions.clone() };
    if args.floor && !resolutions.iter().any(|r| r == FLOOR_RESOLUTION) {
        resolutions.push(FLOOR_RESOLUTION.to_owned());
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // any 64 bits of the clock do
    let seed = args.seed.unwrap_or_else(|| (suite::unix_now() * 1e6) as u64);
    if !arm_list.is_empty() {
        println!("A/B: {} arms, seed {seed}", arm_list.len());
        if !args.repeat.max(1).is_multiple_of(arm_list.len()) {
            eprintln!("warning: --repeat {} is not a multiple of {} arms: arm positions are not fully counterbalanced", args.repeat, arm_list.len());
        }
    }
    let plan = plan_runs(&chosen, &resolutions, args.repeat, &arm_list, seed);
    let dir = args.out.join(&args.tag);
    std::fs::create_dir_all(&dir)?;
    let busy = busy_rules(args);
    let profile = exe.parent().and_then(Path::file_name).map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    if profile != "profiling" && profile != "release" {
        eprintln!("warning: {} is a `{profile}` build; measure the profiling build (cargo build --profile profiling -p bevyrrowind)", exe.display());
    }
    println!("{} runs → {}", plan.len(), dir.display());

    // Held until this function returns (after the summary): every arm and repeat under one lock and plan.
    let chain = start_chain(args, &exe)?;
    let (runs, mut ledger) = execute(args, &exe, &dir, &plan, &busy);
    ledger.chain.clone_from(&chain.notes);
    if args.dry_run || runs.is_empty() {
        return Ok(());
    }
    let ab = (!arm_list.is_empty()).then(|| AbInfo {
        seed,
        runs: runs.iter().filter_map(|(i, r)| plan[*i].arm.as_ref().map(|(a, _)| (*a, r.clone()))).collect(),
        arms: arm_list,
    });
    let runs: Vec<SuiteRun> = runs.into_iter().map(|(_, r)| r).collect();
    write_summary(args, &exe, &dir, &runs, &ledger, &busy, ab.as_ref())
}

/// The A/B part of a suite for the summary: arms, seed, and each kept run with its arm.
pub(super) struct AbInfo {
    pub(super) arms: Vec<Arm>,
    pub(super) seed: u64,
    pub(super) runs: Vec<(usize, SuiteRun)>,
}

/// What [`execute`] kept out of the medians, and whether every kept run was checked clean.
#[derive(Default)]
pub(super) struct Ledger {
    /// Busy-machine notes of kept runs (only with `--no-wait`).
    pub(super) notes: Vec<String>,
    /// Runs left out after the last retry, with why.
    pub(super) dropped: Vec<String>,
    /// Measured runs planned (warm-ups excluded).
    pub(super) planned: usize,
    /// Kept runs whose cleanliness could not be fully checked (CPU or focus not recorded).
    pub(super) unchecked: Vec<String>,
    /// Discarded warm-up runs that ran.
    pub(super) warmups: usize,
    /// The chain's hold on the machine: bench lock, power plan, throttling (`chain.rs`).
    pub(super) chain: Vec<String>,
}

impl Ledger {
    /// "machine: quiet" holds only when every kept run was checked clean.
    pub(super) fn quiet(&self) -> bool {
        self.notes.is_empty() && self.unchecked.is_empty()
    }

    /// Summary lines: valid count, dropped runs, the machine state.
    pub(super) fn lines(&self, kept: usize, busy: &suite::BusyRules) -> Vec<String> {
        let mut lines = vec![format!(
            "- valid runs: {kept} of {}{}",
            self.planned,
            if self.warmups > 0 { format!(" ({} warm-up runs discarded)", self.warmups) } else { String::new() }
        )];
        lines.extend(self.dropped.iter().map(|d| format!("- **dropped**: {d}")));
        lines.extend(self.chain.iter().map(|c| format!("- {c}")));
        if self.quiet() {
            let dirs = if busy.dirs.is_empty() { String::new() } else { format!(" nor any exe under {}", busy.dirs.iter().map(|d| format!("`{}`", d.trim_end_matches('\\'))).collect::<Vec<_>>().join(", ")) };
            lines.push(format!(
                "- machine: quiet, every kept run checked (before each run: no {} process{dirs}, no other process ≥ {GPU_BUSY_PCT}% GPU SM, GPU ≤ 15% at ≥ {IDLE_GPU_CLOCK_MHZ} MHz, CPU ≤ 20%; during: other processes ≤ {OTHER_CPU_LIMIT}% CPU and < {GPU_BUSY_PCT}% GPU SM, none of those processes at the end, window in focus)",
                busy.names.join("/")
            ));
        } else {
            lines.extend(self.notes.iter().map(|n| format!("- **busy machine**: {n}")));
            if !self.unchecked.is_empty() {
                lines.push(format!("- machine: not fully checked for {} (CPU or focus not recorded)", self.unchecked.join(", ")));
            }
        }
        lines
    }
}

/// Why a measured run is dirty: busy or unfocused during it, or (unless `--no-wait`) the quiet wait gave up.
fn dirty_reason(m: &Measured, no_wait: bool) -> Option<String> {
    m.retry.map(str::to_owned).or_else(|| m.note.as_ref().filter(|_| !no_wait).map(|n| format!("busy before the run ({n})")))
}

fn point(r: &SuiteRun) -> Option<RunPoint> {
    Some(RunPoint { median_ms: r.report.frames.as_ref()?.stats.median, hitches: r.report.hitches.as_ref().map_or(0, |h| h.measured) })
}

/// The kept runs of the same scenario and variant (an arm's other runs).
fn peers(kept: &[(usize, SuiteRun)], run: &SuiteRun) -> Vec<RunPoint> {
    kept.iter().filter(|(_, k)| k.scenario == run.scenario && k.variant == run.variant).filter_map(|(_, k)| point(k)).collect()
}

/// Measures one planned run until it is clean and no outlier against `kept` (up to `--retries` retries);
/// `None` when it failed or stayed dirty (the ledger says why) or in a dry run.
fn measure(args: &SuiteArgs, exe: &Path, dir: &Path, p: &Planned, busy: &suite::BusyRules, kept: &[(usize, SuiteRun)], ledger: &mut Ledger) -> Option<Measured> {
    let label = || format!("{} @ {} #{}{}", p.scenario.name, p.resolution, p.repeat, p.arm.as_ref().map_or(String::new(), |(_, a)| format!(" [{}]", a.name)));
    for attempt in 0..=args.retries {
        let m = match run_one(args, exe, dir, p, busy) {
            Ok(Some(m)) => m,
            Ok(None) => return None,
            Err(e) => {
                eprintln!("    failed: {e}");
                ledger.dropped.push(format!("{}: failed: {e}", label()));
                return None;
            }
        };
        let why = dirty_reason(&m, args.no_wait).or_else(|| point(&m.run).and_then(|pt| outlier_reason(pt, &peers(kept, &m.run))));
        let Some(why) = why else { return Some(m) };
        if attempt < args.retries {
            println!("    {why}; retry {}/{}", attempt + 1, args.retries);
        } else {
            println!("    {why}; dropped after {} retries", args.retries);
            ledger.dropped.push(format!("{}: {why} (after {} retries)", label(), args.retries));
        }
    }
    None
}

fn print_run(run: &SuiteRun) {
    if let Some(f) = &run.report.frames {
        println!(
            "    median {:.2} ms ({:.0} fps) · p99 {:.2} · worst {:.1} · GPU {}%",
            f.stats.median,
            f.fps_median,
            f.stats.p99,
            f.stats.max,
            run.gpu.map_or("?".to_owned(), |g| format!("{:.0}", g.util_mean))
        );
    }
}

/// Keeps a measured run: prints it, notes it in the ledger.
fn keep(runs: &mut Vec<(usize, SuiteRun)>, ledger: &mut Ledger, i: usize, m: Measured) {
    print_run(&m.run);
    if !m.checked {
        ledger.unchecked.push(run_name(&m.run.scenario, &m.run.variant, m.run.repeat));
    }
    ledger.notes.extend(m.note);
    runs.push((i, m.run));
}

/// Runs the plan (quiet-machine waits, retries of dirty and outlier runs); each kept run with its plan index.
/// Warm-ups run once and are discarded. Dirty runs never enter the result: after the last retry they are dropped
/// (the ledger says "N of M valid"). A last pass re-checks each arm leave-one-out (a garbage first run had no
/// peers yet) and re-measures its outliers.
pub(super) fn execute(args: &SuiteArgs, exe: &Path, dir: &Path, plan: &[Planned], busy: &suite::BusyRules) -> (Vec<(usize, SuiteRun)>, Ledger) {
    let mut runs: Vec<(usize, SuiteRun)> = Vec::new();
    let mut ledger = Ledger::default();
    for (i, p) in plan.iter().enumerate() {
        println!(
            "[{}/{}] {} @ {}{}{} #{}{}",
            i + 1,
            plan.len(),
            p.scenario.name,
            p.resolution,
            p.step.as_ref().map_or(String::new(), |s| format!(" {}", s.label)),
            p.arm.as_ref().map_or(String::new(), |(_, a)| format!(" [{}]", a.name)),
            p.repeat,
            if p.warmup { " (warm-up, discarded)" } else { "" }
        );
        if p.warmup {
            match run_one(args, exe, dir, p, busy) {
                Ok(Some(_)) => ledger.warmups += 1,
                Ok(None) => {}
                Err(e) => eprintln!("    warm-up failed: {e}"),
            }
            continue;
        }
        ledger.planned += 1;
        if let Some(m) = measure(args, exe, dir, p, busy, &runs, &mut ledger) {
            keep(&mut runs, &mut ledger, i, m);
        }
    }
    if args.dry_run {
        return (runs, ledger);
    }
    // Leave-one-out per arm: the outliers an early run escaped.
    let mut groups: Vec<(String, String)> = Vec::new();
    for (_, r) in &runs {
        let key = (r.scenario.clone(), r.variant.clone());
        if !groups.contains(&key) {
            groups.push(key);
        }
    }
    for (scenario, variant) in groups {
        let members: Vec<usize> = (0..runs.len()).filter(|&k| runs[k].1.scenario == scenario && runs[k].1.variant == variant).collect();
        let points: Vec<RunPoint> = members.iter().filter_map(|&k| point(&runs[k].1)).collect();
        if points.len() != members.len() {
            continue;
        }
        let mut flagged: Vec<usize> = outliers(&points).into_iter().map(|(j, why)| {
            let r = &runs[members[j]].1;
            println!("final check: {} is an outlier ({why}); measuring again", run_name(&r.scenario, &r.variant, r.repeat));
            members[j]
        }).collect();
        flagged.sort_unstable_by(|a, b| b.cmp(a));
        for k in flagged {
            let (i, old) = runs.remove(k);
            ledger.unchecked.retain(|u| *u != run_name(&old.scenario, &old.variant, old.repeat));
            if let Some(m) = measure(args, exe, dir, &plan[i], busy, &runs, &mut ledger) {
                keep(&mut runs, &mut ledger, i, m);
            }
        }
    }
    runs.sort_by_key(|(i, _)| *i);
    (runs, ledger)
}

/// `summary.md` across the runs, and the copies under `--save`.
pub(super) fn write_summary(
    args: &SuiteArgs,
    exe: &Path,
    dir: &Path,
    runs: &[SuiteRun],
    ledger: &Ledger,
    busy: &suite::BusyRules,
    ab: Option<&AbInfo>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut gauges: Vec<GaugeColumn> = arms::thread_gauges().into();
    gauges.extend([
        GaugeColumn::median("entities", sf_diag_bevy::ENTITIES),
        GaugeColumn::median("meshes", "world/meshes"),
        GaugeColumn::median("point lights", "world/point_lights"),
        GaugeColumn::median("process CPU %", "process/cpu_usage"),
        GaugeColumn::median("system CPU %", "system/cpu_usage"),
        // Static-scene S0 (sf_diag::views): render-thread work per view, all views summed.
        GaugeColumn::median("views", "views/all/count"),
        GaugeColumn::median("phase items", "views/all/items"),
        GaugeColumn::median("batch sets", "views/all/batch_sets"),
        GaugeColumn::median("work items", "views/all/work_items"),
        GaugeColumn::median("preprocess bind groups", "preprocess/bind_groups"),
        // Static scene (sf_scene_bevy::gauges): "—" in a run without `--scene`.
        GaugeColumn::median("scene cells", "scene/cells/resident"),
        GaugeColumn::median("scene batches", "scene/batches/count"),
        GaugeColumn::median("scene live slots", "scene/slots/batch_live"),
        GaugeColumn::median("scene slot bytes", "scene/slots/batch_capacity_bytes"),
    ]);
    let mut summary = suite::suite_markdown(&format!("Perf suite `{}`", args.tag), runs, &gauges);
    let mut header = String::new();
    let first = &runs[0].report.meta;
    let _ = writeln!(header, "- exe: `{}` ({})", exe.display(), first.get("build").map_or("?", String::as_str));
    for line in arms::provenance_lines(exe, &arms::MachineProbe) {
        let _ = writeln!(header, "{line}");
    }
    let _ = writeln!(header, "- GPU: {}", first.get("gpu").map_or("?", String::as_str));
    let settings = args.run.describe();
    let _ = writeln!(header, "- settings for every run: {} · hitch threshold: {}", if settings.is_empty() { "none".to_owned() } else { settings }, args.hitch_ms.map_or("2.5 × median, ≥ 25 ms".to_owned(), |h| format!("{h} ms")));
    if let Some(ab) = ab {
        let _ = writeln!(header, "- A/B: {} arms interleaved (counterbalanced ABBA/BAAB per repeat, one warm-up per arm discarded), seed {} (`--seed {}` replays the order)", ab.arms.len(), ab.seed, ab.seed);
        for (i, arm) in ab.arms.iter().enumerate() {
            let s = arm.settings.describe();
            let _ = writeln!(header, "  - arm `{}`{}: {}", arm.name, if i == 0 { " (base)" } else { "" }, if s.is_empty() { "shipped defaults".to_owned() } else { s });
        }
    }
    let _ = writeln!(header, "- scenarios: {}", SCENARIOS.iter().filter(|s| runs.iter().any(|r| r.scenario == s.name)).map(|s| format!("`{}` {}", s.name, s.about)).collect::<Vec<_>>().join("; "));
    for line in ledger.lines(runs.len(), busy) {
        let _ = writeln!(header, "{line}");
    }
    for line in focus_lines(&runs.iter().map(run_focus).collect::<Vec<_>>()) {
        let _ = writeln!(header, "{line}");
    }
    let at = summary.find("## ").unwrap_or(summary.len());
    summary.insert_str(at, &format!("{header}\n"));
    if let Some(ab) = ab {
        let mut groups: Vec<(String, String)> = Vec::new();
        for (_, r) in &ab.runs {
            let res = r.variant.split(' ').next().unwrap_or_default().to_owned();
            if !groups.iter().any(|(s, x)| *s == r.scenario && *x == res) {
                groups.push((r.scenario.clone(), res));
            }
        }
        let arm_runs: Vec<ArmRun<'_>> = ab.runs.iter().map(|(arm, run)| ArmRun { arm: *arm, run }).collect();
        let at = summary.find("## Across repeats").unwrap_or(summary.len());
        summary.insert_str(at, &format!("{}\n", arms::verdict_markdown(&ab.arms, &arm_runs, &groups)));
    }
    let path = dir.join("summary.md");
    std::fs::write(&path, &summary)?;
    println!("\nwrote {}", path.display());
    if let Some(save) = &args.save {
        std::fs::create_dir_all(save)?;
        std::fs::write(save.join("summary.md"), &summary)?;
        for r in runs {
            let run_dir = dir.join(run_name(r.scenario.as_str(), &r.variant, r.repeat));
            for entry in std::fs::read_dir(&run_dir)?.filter_map(Result::ok) {
                let p = entry.path();
                if p.extension().is_some_and(|e| e == "md" || e == "json") {
                    let name = format!("{}.{}", run_name(r.scenario.as_str(), &r.variant, r.repeat), p.extension().unwrap_or_default().to_string_lossy());
                    std::fs::copy(&p, save.join(name))?;
                }
            }
        }
        println!("copied to {}", save.display());
    }
    Ok(())
}

fn run_name(scenario: &str, variant: &str, repeat: usize) -> String {
    format!("{scenario}@{}-{repeat}", variant.replace([' ', ',', ':'], "_"))
}

/// Waits for a quiet machine: none of `busy` running, GPU and CPU idle.
/// Returns a note when it gave up waiting.
fn wait_quiet(busy: &suite::BusyRules, max_wait: Duration, no_wait: bool) -> Result<(Option<String>, f64, f64), Box<dyn std::error::Error>> {
    let started = Instant::now();
    let mut said = String::new();
    loop {
        let all = suite::processes()?;
        let procs = suite::busy_among(&all, std::process::id(), busy);
        let contenders = suite::gpu_contenders(&suite::gpu_processes(), &suite::family(&all, std::process::id()), f64::NEG_INFINITY, f64::INFINITY, GPU_BUSY_PCT);
        let gpu = gpu_load(Duration::from_secs(2));
        let cpu = suite::cpu_load_percent().unwrap_or(0.0);
        let mut why = Vec::new();
        if !procs.is_empty() {
            let mut names: Vec<String> = procs.into_iter().map(|(_, why)| why).collect();
            names.sort();
            names.dedup();
            why.push(format!("running: {}", names.join(", ")));
        }
        if !contenders.is_empty() {
            why.push(format!("on the GPU: {}", gpu_list(&contenders)));
        }
        if gpu > 15.0 {
            why.push(format!("GPU {gpu:.0}%"));
        }
        if cpu > 20.0 {
            why.push(format!("CPU {cpu:.0}%"));
        }
        if why.is_empty() {
            return Ok((None, gpu, cpu));
        }
        let reason = why.join(", ");
        if no_wait || started.elapsed() > max_wait {
            return Ok((Some(reason), gpu, cpu));
        }
        if reason != said {
            println!("    waiting for a quiet machine ({reason})");
            said = reason;
        }
        std::thread::sleep(Duration::from_secs(10));
    }
}

/// Below this graphics clock a GPU's utilisation is idle housekeeping, not
/// work: the lock screen and desktop compositor keep 25-40 % "utilisation" at
/// ~210 MHz, while any real 3D load clocks up past 1 GHz (perf.md "Quiet machine").
const IDLE_GPU_CLOCK_MHZ: f64 = 600.0;

/// Mean GPU load over `span` (0 without `nvidia-smi`); samples at an idle
/// clock (below [`IDLE_GPU_CLOCK_MHZ`]) count as 0 %.
fn gpu_load(span: Duration) -> f64 {
    let Ok(gpu) = GpuSampler::start(250) else { return 0.0 };
    std::thread::sleep(span);
    let taken = gpu.stop();
    working_gpu_load(&taken)
}

/// Other work seen during a run, besides CPU share: GPU work of other processes and busy processes still running.
struct OtherWork {
    /// `nvidia-smi pmon` ran.
    pmon_on: bool,
    /// pmon samples inside the measured window.
    in_window: usize,
    /// Other processes at or above [`GPU_BUSY_PCT`] SM in the window.
    gpu: Vec<(u32, String, f64)>,
    /// [`busy_rules`] processes running when the run ended (one that came and went is seen only by its GPU or CPU use).
    busy_at_end: Vec<(suite::ProcessInfo, String)>,
}

impl OtherWork {
    /// Judges the `pmon` samples (`None`: pmon didn't run) in `window` (the runner's family and the game `child`
    /// exempt), takes the end-of-run process snapshot, and writes `gpu-procs.csv` (every sample with SM work: the
    /// evidence).
    fn check(pmon: Option<Vec<suite::GpuProcSample>>, child: u32, busy: &suite::BusyRules, (from, to): (f64, f64), run_dir: &Path) -> std::io::Result<Self> {
        let pmon_on = pmon.is_some();
        let samples = pmon.unwrap_or_default();
        let at_end = suite::processes().unwrap_or_default();
        let mut exempt = suite::family(&at_end, std::process::id());
        exempt.push(child);
        let mut csv = String::from("unix_s,pid,type,sm_pct,name\n");
        for s in samples.iter().filter(|s| s.sm_pct.is_some()) {
            let _ = writeln!(csv, "{:.3},{},{},{},{}", s.unix_s, s.pid, s.kind, s.sm_pct.unwrap_or_default(), s.name.replace(',', " "));
        }
        std::fs::write(run_dir.join("gpu-procs.csv"), csv)?;
        Ok(Self {
            pmon_on,
            in_window: samples.iter().filter(|s| s.unix_s >= from && s.unix_s <= to).count(),
            gpu: suite::gpu_contenders(&samples, &exempt, from, to, GPU_BUSY_PCT),
            busy_at_end: suite::busy_among(&at_end, std::process::id(), busy),
        })
    }

    fn describe(&self) -> String {
        if !self.pmon_on {
            "not checked (no `nvidia-smi pmon`)".to_owned()
        } else if self.gpu.is_empty() {
            format!("none ≥ {GPU_BUSY_PCT}% SM ({} pmon samples)", self.in_window)
        } else {
            gpu_list(&self.gpu)
        }
    }
}

/// Waits for the game to exit; kills it after `limit` (`None` = killed).
fn wait_child(child: &mut std::process::Child, limit: Duration) -> std::io::Result<Option<std::process::ExitStatus>> {
    let deadline = Instant::now() + limit;
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(Some(status));
        }
        if Instant::now() > deadline {
            let _ = child.kill();
            return Ok(None);
        }
        std::thread::sleep(Duration::from_millis(200));
    }
}

/// `gpu.csv`: every `nvidia-smi` sample of the run.
fn write_gpu_csv(samples: &[suite::GpuSample], path: &Path) -> std::io::Result<()> {
    let mut csv = String::from("unix_s,util_pct,clock_mhz,mem_clock_mhz,power_w,temp_c,vram_mib,throttle\n");
    for s in samples {
        let _ = writeln!(csv, "{:.3},{},{},{},{},{},{},{:#x}", s.unix_s, s.util_pct, s.clock_mhz, s.mem_clock_mhz, s.power_w, s.temp_c, s.vram_mib, s.throttle);
    }
    std::fs::write(path, csv)
}

/// `name (pid) 40%, ...` for [`suite::gpu_contenders`].
fn gpu_list(contenders: &[(u32, String, f64)]) -> String {
    contenders.iter().map(|(pid, name, sm)| format!("{name} ({pid}) {sm:.0}% SM")).collect::<Vec<_>>().join(", ")
}

/// What else ran during a measured window: other processes' CPU share above [`OTHER_CPU_LIMIT`], other
/// processes' GPU work (`pmon`, at or above [`GPU_BUSY_PCT`] SM), busy processes still running at the end.
fn during_reasons(other_cpu: Option<f64>, gpu_contenders: &[(u32, String, f64)], busy_at_end: &[(suite::ProcessInfo, String)]) -> Vec<String> {
    let mut why = Vec::new();
    if let Some(o) = other_cpu.filter(|&o| o > OTHER_CPU_LIMIT) {
        why.push(format!("other processes used {o:.0}% CPU"));
    }
    if !gpu_contenders.is_empty() {
        why.push(format!("other GPU work: {}", gpu_list(gpu_contenders)));
    }
    if !busy_at_end.is_empty() {
        let mut names: Vec<&str> = busy_at_end.iter().map(|(_, w)| w.as_str()).collect();
        names.sort_unstable();
        names.dedup();
        why.push(format!("running at the end: {}", names.join(", ")));
    }
    why
}

/// Mean utilisation, idle-clock samples counted as 0 %.
fn working_gpu_load(samples: &[suite::GpuSample]) -> f64 {
    if samples.is_empty() {
        return 0.0;
    }
    #[allow(clippy::cast_precision_loss)] // a few samples
    let n = samples.len() as f64;
    samples.iter().map(|s| if s.clock_mhz < IDLE_GPU_CLOCK_MHZ { 0.0 } else { s.util_pct }).sum::<f64>() / n
}

/// A run's variant label: resolution, then `off=`, extra game flags and the sweep step when present.
fn variant_name(resolution: &str, off: &[String], game_args: &[String], step: Option<&str>) -> String {
    let mut variant = if off.is_empty() { resolution.to_owned() } else { format!("{resolution} off={}", off.join(",")) };
    for extra in game_args.iter().map(String::as_str).chain(step) {
        variant = format!("{variant} {extra}");
    }
    variant
}

fn run_one(args: &SuiteArgs, exe: &Path, dir: &Path, p: &Planned, busy: &suite::BusyRules) -> RunResult {
    // The suite's settings, then the arm's, then the sweep step's.
    let settings = p.arm.as_ref().map_or_else(|| args.run.clone(), |(_, arm)| args.run.with(&arm.settings));
    let mut off = settings.off.clone();
    off.extend(p.step.iter().flat_map(|s| s.off.iter().cloned()));
    let variant = match &p.arm {
        // An arm is named by its name alone: the summary lists each arm's settings.
        Some((_, arm)) => format!("{} arm={}", variant_name(&p.resolution, &args.run.off, &args.run.args, None), arm.name),
        None => variant_name(&p.resolution, &off, &settings.args, p.step.as_ref().map(|s| s.label.as_str())),
    };
    let name = run_name(p.scenario.name, &variant, p.repeat);
    let run_dir = dir.join(&name);
    let mut cmd_args: Vec<String> = p.scenario.args.iter().map(|a| (*a).to_owned()).collect();
    cmd_args.extend(p.step.iter().flat_map(|s| s.args.iter().cloned()).chain(settings.args.iter().cloned()));
    cmd_args.extend(["--resolution".to_owned(), p.resolution.clone(), "--bench-out".to_owned(), run_dir.display().to_string(), "--bench-baseline".to_owned(), "none".to_owned()]);
    if !off.is_empty() {
        cmd_args.extend(["--off".to_owned(), off.join(",")]);
    }
    if let Some(h) = args.hitch_ms {
        cmd_args.extend(["--hitch-ms".to_owned(), h.to_string()]);
    }
    let mut envs = settings.env_pairs()?;
    envs.extend(p.step.iter().flat_map(|s| s.envs.iter().cloned()));
    if args.trace {
        envs.push(("TRACE_CHROME".to_owned(), run_dir.join("trace.json").display().to_string()));
    }
    let shown: String = cmd_args.iter().map(|a| if a.contains(' ') { format!("\"{a}\"") } else { a.clone() }).collect::<Vec<_>>().join(" ");
    if args.dry_run {
        println!("    {} {shown}", exe.display());
        return Ok(None);
    }
    // Stale files from an earlier run would be read as this run's.
    if run_dir.exists() {
        std::fs::remove_dir_all(&run_dir)?;
    }
    std::fs::create_dir_all(&run_dir)?;
    let bench = cmd_args.iter().position(|a| a == "--bench").and_then(|i| cmd_args.get(i + 1)).ok_or("scenario has no --bench")?;
    let scenario = Scenario::from_json(&std::fs::read_to_string(bench)?)?;

    let max_wait = Duration::from_secs_f64(args.max_wait_min * 60.0);
    let (note, gpu_before, cpu_before) = wait_quiet(busy, max_wait, args.no_wait)?;
    let note = note.map(|n| format!("{name}: {n}"));
    if let Some(n) = &note {
        println!("    measuring on a busy machine: {n}");
    }

    let log = File::create(run_dir.join("log.txt"))?;
    let gpu_smi = GpuSampler::start(250).ok();
    let pmon = suite::PmonSampler::start().ok();
    let launch = suite::unix_now();
    let mut child = Command::new(exe)
        .args(&cmd_args)
        .envs(envs.iter().map(|(k, v)| (k, v)))
        .stdout(Stdio::from(log.try_clone()?))
        .stderr(Stdio::from(log))
        .spawn()?;
    let child_pid = child.id();
    let status = wait_child(&mut child, Duration::from_mins(20));
    // Samplers stop before anything can fail (a leaked `nvidia-smi` loop would outlive the suite).
    let samples = gpu_smi.map(GpuSampler::stop).unwrap_or_default();
    let pmon = pmon.map(suite::PmonSampler::stop);
    let ended = suite::unix_now();
    let status = status?;
    let stem = file_stem(&scenario.name);
    let report_path = run_dir.join(format!("{stem}.json"));
    let Ok(report) = Report::load(&report_path) else {
        return Err(format!("no report (exit {status:?}); see {}", run_dir.join("log.txt").display()).into());
    };
    check_dlss(&envs, &cmd_args, &report, &run_dir)?;
    let gpu = report.wall_start_unix_s.zip(report.wall_end_unix_s).and_then(|(a, b)| GpuSummary::over(&samples, a, b));
    // The quiet check only sees the start: other work can begin mid-run.
    let median = |c: &str| report.counters.get(c).and_then(|c| c.per_frame).map(|s| s.median);
    let other_cpu = median("system/cpu_usage").zip(median("process/cpu_usage")).map(|(s, p)| s - p);
    let window = report.wall_start_unix_s.zip(report.wall_end_unix_s).unwrap_or((launch, ended));
    let others = OtherWork::check(pmon, child_pid, busy, window, &run_dir)?;
    let during = during_reasons(other_cpu, &others.gpu, &others.busy_at_end);
    let busy_during = (!during.is_empty()).then(|| format!("{name}: during the run: {}", during.join("; ")));
    let focus = FocusTally::from_report(&report);

    write_gpu_csv(&samples, &run_dir.join("gpu.csv"))?;
    std::fs::write(run_dir.join("run.txt"), format!("launch_unix_s={launch:.3}\nexe={}\nargs={shown}\n", exe.display()))?;

    let mut extra = String::from("\n## Run\n\n");
    let _ = writeln!(extra, "- `{}` {shown}", exe.display());
    if !envs.is_empty() {
        let _ = writeln!(extra, "- env: {}", envs.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(" "));
    }
    let _ = writeln!(extra, "- before the run: GPU {gpu_before:.0}%, CPU {cpu_before:.0}%{}", note.as_ref().map_or(String::new(), |n| format!(" · **busy: {n}**")));
    let _ = writeln!(extra, "- during the run: other processes {}", other_cpu.map_or("?".to_owned(), |o| format!("{o:.0}% CPU (system minus this process)")));
    let _ = writeln!(extra, "- other GPU work during the run: {}", others.describe());
    if let Some(b) = &busy_during {
        let _ = writeln!(extra, "- **busy**: {b}");
    }
    let _ = writeln!(extra, "- focus: {}", focus.map_or_else(|| "not recorded (exe older than the focus guard)".to_owned(), focus_md));
    if let Some(g) = &gpu {
        let _ = writeln!(extra, "{}", g.to_markdown());
    }
    let md_path = run_dir.join(format!("{stem}.md"));
    let mut md = std::fs::read_to_string(&md_path).unwrap_or_default();
    md.push_str(&extra);
    std::fs::write(&md_path, md)?;

    let retry = retry_reason(busy_during.is_some(), focus);
    let note = match (note, busy_during) {
        (Some(a), Some(b)) => Some(format!("{a}; {b}")),
        (a, b) => a.or(b),
    };
    // GPU contention counts as checked when pmon ran with samples in the window, or the machine has no
    // nvidia-smi at all (no GPU samples either).
    let checked = other_cpu.is_some() && focus.is_some() && (others.in_window > 0 || samples.is_empty());
    Ok(Some(Measured { run: SuiteRun { scenario: p.scenario.name.to_owned(), variant, repeat: p.repeat, report, gpu }, note, retry, checked }))
}

/// DLSS asked for but silently absent (an exe without its `dlss/` folder) measures another renderer: fail the run.
/// Not checkable in a run without GPU timers (`BEVYRROWIND_NO_GPU_TIMERS`, meta `gpu_timers: off`): passes.
fn check_dlss(envs: &[(String, String)], cmd_args: &[String], report: &Report, run_dir: &Path) -> Result<(), String> {
    if report.meta.get(mw_devtools_bevy::perf::GPU_TIMERS_META).is_some_and(|v| v == "off") {
        return Ok(());
    }
    match arms::dlss_requested(envs, std::env::var("BEVYRROWIND_DLSS").ok().as_deref(), cmd_args) {
        Some(mode) if !arms::dlss_timed(report.counters.keys()) => Err(format!(
            "DLSS `{mode}` requested but no DLSS pass was timed (no `*dlss_*/elapsed_gpu` counter): the exe ran without DLSS; see {}",
            run_dir.join("log.txt").display()
        )),
        _ => Ok(()),
    }
}

/// As `sf_diag_bevy` names report files.
fn file_stem(name: &str) -> String {
    name.chars().map(|c| if c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-') { c } else { '_' }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idle_clock_gpu_load_counts_as_idle() {
        let at = |util_pct, clock_mhz| suite::GpuSample {
            unix_s: 0.0,
            util_pct,
            clock_mhz,
            mem_clock_mhz: 0.0,
            power_w: 0.0,
            temp_c: 0.0,
            vram_mib: 0.0,
            throttle: 0,
        };
        // The lock screen: 25-40 % at ~210 MHz is idle.
        assert!(working_gpu_load(&[at(40.0, 210.0), at(25.0, 210.0)]) < 1.0);
        // A game: counted.
        assert!((working_gpu_load(&[at(90.0, 1800.0), at(10.0, 210.0)]) - 45.0).abs() < 1e-9);
        assert!(working_gpu_load(&[]) < 1e-9);
    }

    #[test]
    fn unfocused_runs_retry_like_busy_ones() {
        let clean = FocusTally { frames: 100, unfocused: 0, hidden: 0 };
        let dirty = FocusTally { frames: 100, unfocused: 3, hidden: 0 };
        assert_eq!(retry_reason(false, Some(clean)), None);
        // Not recorded (an old exe) doesn't retry on focus.
        assert_eq!(retry_reason(false, None), None);
        assert_eq!(retry_reason(true, Some(clean)), Some("busy during the run"));
        assert_eq!(retry_reason(false, Some(dirty)), Some("window out of focus during the run"));
        assert_eq!(retry_reason(true, Some(dirty)), Some("busy and window out of focus during the run"));
    }

    #[test]
    fn focus_lines_flag_each_dirty_run() {
        let clean = Some(FocusTally { frames: 100, unfocused: 0, hidden: 0 });
        let all_clean = [("a@1280x720-1".to_owned(), clean), ("a@1280x720-2".to_owned(), clean)];
        assert_eq!(focus_lines(&all_clean), ["- focus: clean, all 2 runs (window in front for every measured frame)"]);
        let mixed = [
            ("a@1280x720-1".to_owned(), clean),
            ("a@1280x720-2".to_owned(), Some(FocusTally { frames: 200, unfocused: 50, hidden: 10 })),
            ("b@1280x720-1".to_owned(), None),
        ];
        assert_eq!(
            focus_lines(&mixed),
            [
                "- focus: 1 of 3 runs clean",
                "- **unfocused**: a@1280x720-2: DIRTY: window unfocused 50 of 200 measured frames (25.0%), hidden 10 (5.0%)",
                "- focus: not recorded for b@1280x720-1 (exe older than the focus guard)",
            ]
        );
    }

    /// `--arg` takes a game flag that starts with `--` (the `--scene` A/B), repeatable.
    #[test]
    fn suite_arg_passes_game_flags_through() {
        #[derive(clap::Parser)]
        struct Cli {
            #[command(flatten)]
            suite: SuiteArgs,
        }
        let cli = <Cli as clap::Parser>::try_parse_from(["perf", "t", "balmora_walk", "--arg=--scene", "--arg", "--rings"]).unwrap();
        assert_eq!(cli.suite.run.args, ["--scene", "--rings"]);
        assert_eq!(cli.suite.scenarios, ["balmora_walk"]);
    }
}

#[cfg(test)]
mod ab_tests {
    use super::*;

    fn arm_list(names: &[&str]) -> Vec<Arm> {
        names.iter().map(|n| arms::parse_arm(&format!("{n}:")).unwrap()).collect()
    }

    /// Warm-ups first (one per arm, flagged), then each repeat runs every arm once; positions balance.
    fn check_plan(k: usize, repeats: usize) {
        let names: Vec<String> = (0..k).map(|i| format!("a{i}")).collect();
        let arms = arm_list(&names.iter().map(String::as_str).collect::<Vec<_>>());
        let chosen = [&SCENARIOS[0], &SCENARIOS[4]];
        let plan = plan_runs(&chosen, &["1280x720".to_owned()], repeats, &arms, 9);
        assert_eq!(plan.len(), k + 2 * k * repeats);
        assert!(plan[..k].iter().all(|p| p.warmup && p.repeat == 0 && p.scenario.name == chosen[0].name));
        let mut warm: Vec<usize> = plan[..k].iter().map(|p| p.arm.as_ref().unwrap().0).collect();
        warm.sort_unstable();
        assert_eq!(warm, (0..k).collect::<Vec<_>>(), "one warm-up per arm");
        assert!(plan[k..].iter().all(|p| !p.warmup && p.repeat >= 1));
        for scenario in chosen {
            let measured: Vec<&Planned> = plan[k..].iter().filter(|p| p.scenario.name == scenario.name).collect();
            for (r, block) in measured.chunks(k).enumerate() {
                assert!(block.iter().all(|p| p.repeat == r + 1));
            }
            for pos in 0..k {
                for arm in 0..k {
                    let n = measured.chunks(k).filter(|b| b[pos].arm.as_ref().unwrap().0 == arm).count();
                    assert_eq!(n, repeats / k, "arm {arm} at position {pos}");
                }
            }
        }
    }

    #[test]
    fn interleaved_plans_for_two_three_and_five_arms() {
        check_plan(2, 4);
        check_plan(3, 6);
        check_plan(5, 5);
        // Two arms: ABBA or BAAB per pair of repeats.
        let plan = plan_runs(&[&SCENARIOS[0]], &["1280x720".to_owned()], 4, &arm_list(&["base", "x"]), 3);
        let order: String = plan[2..].iter().map(|p| if p.arm.as_ref().unwrap().0 == 0 { 'A' } else { 'B' }).collect();
        assert!(order.as_bytes().chunks(4).all(|c| c == b"ABBA" || c == b"BAAB"), "{order}");
        // Without arms: unchanged scenario → resolution → repeat, no warm-ups.
        let plan = plan_runs(&[&SCENARIOS[0]], &["1280x720".to_owned(), "960x600".to_owned()], 2, &[], 0);
        let shape: Vec<(String, usize, bool)> = plan.iter().map(|p| (p.resolution.clone(), p.repeat, p.warmup || p.arm.is_some())).collect();
        assert_eq!(
            shape,
            [("1280x720".into(), 1, false), ("1280x720".into(), 2, false), ("960x600".into(), 1, false), ("960x600".into(), 2, false)]
        );
    }

    #[test]
    fn suite_takes_arms_and_seed() {
        #[derive(clap::Parser)]
        struct Cli {
            #[command(flatten)]
            suite: SuiteArgs,
        }
        let cli = <Cli as clap::Parser>::try_parse_from([
            "perf", "t", "balmora_walk", "--repeat", "6", "--seed", "7", "--arm", "base:", "--arm", "scene:--arg=--scene", "--arm=h1:--env BEVYRROWIND_GPU_CULL_STATICS=1",
        ])
        .unwrap();
        assert_eq!(cli.suite.arms, ["base:", "scene:--arg=--scene", "h1:--env BEVYRROWIND_GPU_CULL_STATICS=1"]);
        assert_eq!(cli.suite.seed, Some(7));
        assert_eq!(arms::parse_arms(&cli.suite.arms).unwrap()[2].settings.envs, ["BEVYRROWIND_GPU_CULL_STATICS=1"]);
    }

    fn measured(note: Option<&str>, retry: Option<&'static str>) -> Measured {
        let report = Report::build("x", &sf_diag_bevy::sf_diag::Recorder::new(), &[], 1.5, std::collections::BTreeMap::new());
        Measured { run: SuiteRun { scenario: "s".into(), variant: "v".into(), repeat: 1, report, gpu: None }, note: note.map(str::to_owned), retry, checked: true }
    }

    #[test]
    fn dirty_runs_are_named_and_never_quiet() {
        assert_eq!(dirty_reason(&measured(None, None), false), None);
        assert_eq!(dirty_reason(&measured(None, Some("busy during the run")), false).as_deref(), Some("busy during the run"));
        // The quiet wait gave up: dirty, unless the user asked for `--no-wait`.
        assert_eq!(dirty_reason(&measured(Some("running: cargo"), None), false).as_deref(), Some("busy before the run (running: cargo)"));
        assert_eq!(dirty_reason(&measured(Some("running: cargo"), None), true), None);

        let ledger = Ledger {
            planned: 12,
            warmups: 2,
            dropped: vec!["balmora @ 1280x720 #3 [x]: median 51.70 ms > 1.3 x the arm's 10.20 ms (after 2 retries)".into()],
            ..Ledger::default()
        };
        let rules = suite::BusyRules { names: vec!["cargo".into()], dirs: vec![suite::normalize_path("C:/cargo-target/scrib", true)], exempt: Vec::new() };
        let lines = ledger.lines(11, &rules);
        assert_eq!(lines[0], "- valid runs: 11 of 12 (2 warm-up runs discarded)");
        assert_eq!(lines[1], "- **dropped**: balmora @ 1280x720 #3 [x]: median 51.70 ms > 1.3 x the arm's 10.20 ms (after 2 retries)");
        assert!(lines[2].starts_with("- machine: quiet, every kept run checked (before each run: no cargo process nor any exe under `c:\\cargo-target\\scrib`"), "{}", lines[2]);
        // A kept run not fully checked, or a `--no-wait` note: no "quiet" line.
        let none = suite::BusyRules::default();
        let unchecked = Ledger { planned: 1, unchecked: vec!["a@1280x720-1".into()], ..Ledger::default() };
        assert!(!unchecked.quiet());
        assert!(unchecked.lines(1, &none).iter().all(|l| !l.contains("quiet")));
        let noisy = Ledger { planned: 1, notes: vec!["a: running: cargo".into()], chain: vec!["bench lock: held".into()], ..Ledger::default() };
        assert_eq!(noisy.lines(1, &none), ["- valid runs: 1 of 1", "- bench lock: held", "- **busy machine**: a: running: cargo"]);
    }

    #[test]
    fn other_gpu_work_or_a_bench_started_mid_run_makes_the_run_busy() {
        assert!(during_reasons(Some(10.0), &[], &[]).is_empty());
        let other = suite::ProcessInfo { pid: 7, parent: 1, name: "orch-a.exe".into(), path: None };
        let why = during_reasons(Some(40.0), &[(9, "h1gpucull.exe".into(), 63.0)], &[(other, "orch-a.exe (under c:\\cargo-target\\scrib)".into())]);
        assert_eq!(
            why,
            ["other processes used 40% CPU", "other GPU work: h1gpucull.exe (9) 63% SM", "running at the end: orch-a.exe (under c:\\cargo-target\\scrib)"]
        );
    }

    #[test]
    fn suite_takes_no_lock() {
        #[derive(clap::Parser)]
        struct Cli {
            #[command(flatten)]
            suite: SuiteArgs,
        }
        let cli = <Cli as clap::Parser>::try_parse_from(["perf", "t", "balmora_walk", "--no-lock", "--dry-run"]).unwrap();
        assert!(cli.suite.no_lock && cli.suite.dry_run);
    }
}
