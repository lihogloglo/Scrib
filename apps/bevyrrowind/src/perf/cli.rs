//! `bevyrrowind perf ...`: the suite runner and the trace / Nsight
//! summaries. No Python: everything the Ovid scripts did lives here, on
//! `sf_diag`'s Bevy-free helpers.

use std::fmt::Write as _;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use sf_diag_bevy::sf_diag::suite::{self, GaugeColumn, GpuSampler, GpuSummary, SuiteRun};
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
        name: "flight",
        about: "flight at 60 m/s, 120 m above the ground, Seyda Neen - Balmora - Red Mountain - Vivec",
        args: &["view", "Seyda Neen", "--bench", "benchmarks/exterior_flight.json", "--bench-ground"],
    },
    SuiteScenario {
        name: "census",
        about: "interior: Seyda Neen Census and Excise Office walk-through",
        args: &["view", "Seyda Neen, Census and Excise Office", "--bench", "benchmarks/census_office.json"],
    },
];

/// The resolution of a CPU-floor run: small enough that the GPU drops out.
const FLOOR_RESOLUTION: &str = "960x600";

/// Processes that make numbers noise: builds, other game instances.
const BUSY: &[&str] = &["cargo", "rustc", "bevyrrowind", "ovid"];

#[derive(clap::Subcommand)]
pub enum PerfCommand {
    /// List the suite's named scenarios.
    List,
    /// Print a scenario's `bevyrrowind` arguments, one per line (for scripts).
    Args { scenario: String },
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
    tag: String,
    /// Scenarios to run (default: all); see `perf list`.
    scenarios: Vec<String>,
    /// Runs per scenario and resolution.
    #[arg(long, default_value_t = 1)]
    repeat: usize,
    /// Window sizes, `WxH` (repeatable). Default `1280x720`.
    #[arg(long = "resolution")]
    resolutions: Vec<String>,
    /// Also run each scenario at 960x600, where the GPU drops out: the CPU floor.
    #[arg(long)]
    floor: bool,
    /// A/B: parts to leave out (see `view --help`, `--off`).
    #[arg(long, value_delimiter = ',')]
    off: Vec<String>,
    /// Hitch threshold in ms (default 2.5 × median, at least 25 ms).
    #[arg(long)]
    hitch_ms: Option<f64>,
    /// Extra environment for the game, KEY=VALUE (repeatable).
    #[arg(long = "env")]
    envs: Vec<String>,
    /// The game executable (default: this one).
    #[arg(long)]
    exe: Option<PathBuf>,
    /// Write a Chrome trace per run (`TRACE_CHROME`; needs an exe built with
    /// `--features trace`).
    #[arg(long)]
    trace: bool,
    /// Results root.
    #[arg(long, default_value = "target/perf")]
    out: PathBuf,
    /// Also copy the summary and each run's report (md + json) here, e.g.
    /// `benchmarks/runs/<tag>`.
    #[arg(long)]
    save: Option<PathBuf>,
    /// More process names to wait for (comma-separated).
    #[arg(long, value_delimiter = ',')]
    wait_for: Vec<String>,
    /// Start runs even when the machine is busy (results are flagged).
    #[arg(long)]
    no_wait: bool,
    /// Longest wait for a quiet machine per run, minutes.
    #[arg(long, default_value_t = 120.0)]
    max_wait_min: f64,
    /// Re-run a scenario this many times when other processes were busy during it.
    #[arg(long, default_value_t = 2)]
    retries: usize,
    /// Print the commands without running them.
    #[arg(long)]
    dry_run: bool,
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

/// A finished run and, when the machine was busy, why.
type RunResult = Result<Option<(SuiteRun, Option<String>, bool)>, Box<dyn std::error::Error>>;

/// Median CPU % of other processes during a run (system minus ours) above
/// which the run counts as measured on a busy machine.
const OTHER_CPU_LIMIT: f64 = 25.0;

/// One planned run.
struct Planned {
    scenario: &'static SuiteScenario,
    resolution: String,
    repeat: usize,
}

fn run_suite(args: &SuiteArgs) -> Result<(), Box<dyn std::error::Error>> {
    let exe = match &args.exe {
        Some(p) => p.clone(),
        None => std::env::current_exe()?,
    };
    if !Path::new("benchmarks").is_dir() {
        return Err("run from the bevyrrowind repo root (scenario paths are relative to it)".into());
    }
    let names: Vec<&str> = if args.scenarios.is_empty() { SCENARIOS.iter().map(|s| s.name).collect() } else { args.scenarios.iter().map(String::as_str).collect() };
    let mut chosen = Vec::new();
    for name in names {
        chosen.push(SCENARIOS.iter().find(|s| s.name == name).ok_or_else(|| format!("unknown scenario {name:?}; see `perf list`"))?);
    }
    let mut resolutions = if args.resolutions.is_empty() { vec!["1280x720".to_owned()] } else { args.resolutions.clone() };
    if args.floor && !resolutions.iter().any(|r| r == FLOOR_RESOLUTION) {
        resolutions.push(FLOOR_RESOLUTION.to_owned());
    }
    let mut plan = Vec::new();
    for scenario in chosen {
        for resolution in &resolutions {
            for repeat in 1..=args.repeat.max(1) {
                plan.push(Planned { scenario, resolution: resolution.clone(), repeat });
            }
        }
    }
    let dir = args.out.join(&args.tag);
    std::fs::create_dir_all(&dir)?;
    let mut busy: Vec<&str> = BUSY.to_vec();
    busy.extend(args.wait_for.iter().map(String::as_str));
    let profile = exe.parent().and_then(Path::file_name).map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    if profile != "profiling" && profile != "release" {
        eprintln!("warning: {} is a `{profile}` build; measure the profiling build (cargo build --profile profiling -p bevyrrowind)", exe.display());
    }
    println!("{} runs → {}", plan.len(), dir.display());

    let mut runs = Vec::new();
    let mut notes = Vec::new();
    for (i, p) in plan.iter().enumerate() {
        println!("[{}/{}] {} @ {} #{}", i + 1, plan.len(), p.scenario.name, p.resolution, p.repeat);
        let mut result = run_one(args, &exe, &dir, p, &busy);
        for retry in 1..=args.retries {
            if !matches!(result, Ok(Some((_, _, true)))) {
                break;
            }
            println!("    busy during the run; retry {retry}/{}", args.retries);
            result = run_one(args, &exe, &dir, p, &busy);
        }
        match result {
            Ok(Some((run, note, _))) => {
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
                notes.extend(note);
                runs.push(run);
            }
            Ok(None) => {}
            Err(e) => eprintln!("    failed: {e}"),
        }
    }
    if args.dry_run || runs.is_empty() {
        return Ok(());
    }
    write_summary(args, &exe, &dir, &runs, &notes, &busy)
}

/// `summary.md` across the runs, and the copies under `--save`.
fn write_summary(args: &SuiteArgs, exe: &Path, dir: &Path, runs: &[SuiteRun], notes: &[String], busy: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let gauges = [
        GaugeColumn { label: "entities".into(), counter: sf_diag_bevy::ENTITIES.into() },
        GaugeColumn { label: "meshes".into(), counter: "world/meshes".into() },
        GaugeColumn { label: "point lights".into(), counter: "world/point_lights".into() },
        GaugeColumn { label: "process CPU %".into(), counter: "process/cpu_usage".into() },
        GaugeColumn { label: "system CPU %".into(), counter: "system/cpu_usage".into() },
    ];
    let mut summary = suite::suite_markdown(&format!("Perf suite `{}`", args.tag), runs, &gauges);
    let mut header = String::new();
    let first = &runs[0].report.meta;
    let _ = writeln!(header, "- exe: `{}` ({})", exe.display(), first.get("build").map_or("?", String::as_str));
    let _ = writeln!(header, "- GPU: {}", first.get("gpu").map_or("?", String::as_str));
    let _ = writeln!(header, "- A/B off: {} · hitch threshold: {}", if args.off.is_empty() { "none".to_owned() } else { args.off.join(",") }, args.hitch_ms.map_or("2.5 × median, ≥ 25 ms".to_owned(), |h| format!("{h} ms")));
    let _ = writeln!(header, "- scenarios: {}", SCENARIOS.iter().filter(|s| runs.iter().any(|r| r.scenario == s.name)).map(|s| format!("`{}` {}", s.name, s.about)).collect::<Vec<_>>().join("; "));
    if notes.is_empty() {
        let _ = writeln!(header, "- machine: quiet (before each run: no {} process, GPU ≤ 15%, CPU ≤ 20%; during: other processes ≤ {OTHER_CPU_LIMIT}% CPU)", busy.join("/"));
    } else {
        for n in notes {
            let _ = writeln!(header, "- **busy machine**: {n}");
        }
    }
    let at = summary.find("## ").unwrap_or(summary.len());
    summary.insert_str(at, &format!("{header}\n"));
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
    format!("{scenario}@{}-{repeat}", variant.replace([' ', ','], "_"))
}

/// Waits for a quiet machine: none of `busy` running, GPU and CPU idle.
/// Returns a note when it gave up waiting.
fn wait_quiet(busy: &[&str], max_wait: Duration, no_wait: bool) -> Result<(Option<String>, f64, f64), Box<dyn std::error::Error>> {
    let started = Instant::now();
    let mut said = String::new();
    loop {
        let procs = suite::busy_processes(busy)?;
        let gpu = gpu_load(Duration::from_secs(2));
        let cpu = suite::cpu_load_percent().unwrap_or(0.0);
        let mut why = Vec::new();
        if !procs.is_empty() {
            let mut names: Vec<String> = procs.iter().map(|p| p.name.clone()).collect();
            names.sort();
            names.dedup();
            why.push(format!("running: {}", names.join(", ")));
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

/// Mean GPU load over `span` (0 without `nvidia-smi`).
fn gpu_load(span: Duration) -> f64 {
    let Ok(gpu) = GpuSampler::start(250) else { return 0.0 };
    std::thread::sleep(span);
    let taken = gpu.stop();
    GpuSummary::over(&taken, 0.0, f64::INFINITY).map_or(0.0, |s| s.util_mean)
}

fn run_one(args: &SuiteArgs, exe: &Path, dir: &Path, p: &Planned, busy: &[&str]) -> RunResult {
    let variant = if args.off.is_empty() { p.resolution.clone() } else { format!("{} off={}", p.resolution, args.off.join(",")) };
    let name = run_name(p.scenario.name, &variant, p.repeat);
    let run_dir = dir.join(&name);
    let mut cmd_args: Vec<String> = p.scenario.args.iter().map(|a| (*a).to_owned()).collect();
    cmd_args.extend(["--resolution".to_owned(), p.resolution.clone(), "--bench-out".to_owned(), run_dir.display().to_string(), "--bench-baseline".to_owned(), "none".to_owned()]);
    if !args.off.is_empty() {
        cmd_args.extend(["--off".to_owned(), args.off.join(",")]);
    }
    if let Some(h) = args.hitch_ms {
        cmd_args.extend(["--hitch-ms".to_owned(), h.to_string()]);
    }
    let mut envs: Vec<(String, String)> = Vec::new();
    for pair in &args.envs {
        let (k, v) = pair.split_once('=').ok_or_else(|| format!("--env {pair:?}: expected KEY=VALUE"))?;
        envs.push((k.to_owned(), v.to_owned()));
    }
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
    let launch = suite::unix_now();
    let mut child = Command::new(exe)
        .args(&cmd_args)
        .envs(envs.iter().map(|(k, v)| (k, v)))
        .stdout(Stdio::from(log.try_clone()?))
        .stderr(Stdio::from(log))
        .spawn()?;
    let deadline = Instant::now() + Duration::from_mins(20);
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break Some(status);
        }
        if Instant::now() > deadline {
            let _ = child.kill();
            break None;
        }
        std::thread::sleep(Duration::from_millis(200));
    };
    let samples = gpu_smi.map(GpuSampler::stop).unwrap_or_default();
    let stem = file_stem(&scenario.name);
    let report_path = run_dir.join(format!("{stem}.json"));
    let Ok(report) = Report::load(&report_path) else {
        return Err(format!("no report (exit {status:?}); see {}", run_dir.join("log.txt").display()).into());
    };
    let gpu = report.wall_start_unix_s.zip(report.wall_end_unix_s).and_then(|(a, b)| GpuSummary::over(&samples, a, b));
    // The quiet check only sees the start: other work can begin mid-run.
    let median = |c: &str| report.counters.get(c).and_then(|c| c.per_frame).map(|s| s.median);
    let other_cpu = median("system/cpu_usage").zip(median("process/cpu_usage")).map(|(s, p)| s - p);
    let busy_during = other_cpu.filter(|&o| o > OTHER_CPU_LIMIT).map(|o| format!("{name}: other processes used {o:.0}% CPU during the run"));

    let mut csv = String::from("unix_s,util_pct,clock_mhz,mem_clock_mhz,power_w,temp_c,vram_mib,throttle\n");
    for s in &samples {
        let _ = writeln!(csv, "{:.3},{},{},{},{},{},{},{:#x}", s.unix_s, s.util_pct, s.clock_mhz, s.mem_clock_mhz, s.power_w, s.temp_c, s.vram_mib, s.throttle);
    }
    std::fs::write(run_dir.join("gpu.csv"), csv)?;
    std::fs::write(run_dir.join("run.txt"), format!("launch_unix_s={launch:.3}\nexe={}\nargs={shown}\n", exe.display()))?;

    let mut extra = String::from("\n## Run\n\n");
    let _ = writeln!(extra, "- `{}` {shown}", exe.display());
    if !envs.is_empty() {
        let _ = writeln!(extra, "- env: {}", envs.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(" "));
    }
    let _ = writeln!(extra, "- before the run: GPU {gpu_before:.0}%, CPU {cpu_before:.0}%{}", note.as_ref().map_or(String::new(), |n| format!(" · **busy: {n}**")));
    let _ = writeln!(extra, "- during the run: other processes {}", other_cpu.map_or("?".to_owned(), |o| format!("{o:.0}% CPU (system minus this process){}", if busy_during.is_some() { " · **busy**" } else { "" })));
    if let Some(g) = &gpu {
        let _ = writeln!(extra, "{}", g.to_markdown());
    }
    let md_path = run_dir.join(format!("{stem}.md"));
    let mut md = std::fs::read_to_string(&md_path).unwrap_or_default();
    md.push_str(&extra);
    std::fs::write(&md_path, md)?;

    let busy = busy_during.is_some();
    let note = match (note, busy_during) {
        (Some(a), Some(b)) => Some(format!("{a}; {b}")),
        (a, b) => a.or(b),
    };
    Ok(Some((SuiteRun { scenario: p.scenario.name.to_owned(), variant, repeat: p.repeat, report, gpu }, note, busy)))
}

/// As `sf_diag_bevy` names report files.
fn file_stem(name: &str) -> String {
    name.chars().map(|c| if c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-') { c } else { '_' }).collect()
}
