//! Running benchmarks from a driver process: waiting for a quiet machine,
//! sampling the GPU alongside, and summarizing repeated runs.
//!
//! Numbers from a busy machine are noise: [`busy_processes`] lists the
//! processes that must not run during a measurement ([`BusyRules`]: by name,
//! or by where the exe lives), ignoring the driver's own [`family`] (a driver
//! started by `cargo run` doesn't wait for its own cargo). [`PmonSampler`]
//! records which other processes put work on the GPU. [`GpuSampler`] records
//! `nvidia-smi` samples stamped with the wall clock, so they line up with
//! a report's [`Report::wall_start_unix_s`] window.

use std::fmt::Write as _;
use std::io::{self, BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::markdown::big;
use crate::report::Report;
use crate::stats::{Stat, percentile_sorted};

/// One running process.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProcessInfo {
    pub pid: u32,
    pub parent: u32,
    pub name: String,
    /// Full executable path, where the platform reports it (Windows; `None` for protected processes).
    pub path: Option<String>,
}

/// Every running process (pid, parent pid, name, executable path).
pub fn processes() -> io::Result<Vec<ProcessInfo>> {
    let output = if cfg!(windows) {
        Command::new("powershell")
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                // `|` can't appear in a Windows file name or path.
                "Get-CimInstance Win32_Process | ForEach-Object { \"$($_.ProcessId)|$($_.ParentProcessId)|$($_.Name)|$($_.ExecutablePath)\" }",
            ])
            .stderr(Stdio::null())
            .output()?
    } else {
        Command::new("ps")
            .args(["-A", "-o", "pid=,ppid=,comm="])
            .stderr(Stdio::null())
            .output()?
    };
    let text = String::from_utf8_lossy(&output.stdout);
    Ok(text.lines().filter_map(parse_process_line).collect())
}

/// `pid|ppid|name|path` (Windows, path may be empty) or `pid ppid name` (`ps`).
fn parse_process_line(line: &str) -> Option<ProcessInfo> {
    let line = line.trim();
    if line.contains('|') {
        let mut f = line.splitn(4, '|');
        let pid = f.next()?.trim().parse().ok()?;
        let parent = f.next()?.trim().parse().ok()?;
        let name = f.next()?.trim().to_owned();
        let path = f.next().map(str::trim).filter(|p| !p.is_empty()).map(str::to_owned);
        return Some(ProcessInfo { pid, parent, name, path });
    }
    let (pid, rest) = line.split_once([',', ' '])?;
    let rest = rest.trim_start();
    let (parent, name) = rest.split_once([',', ' '])?;
    Some(ProcessInfo {
        pid: pid.trim().parse().ok()?,
        parent: parent.trim().parse().ok()?,
        name: name.trim().to_owned(),
        path: None,
    })
}

/// A process name without directory or `.exe`, lowercase.
fn base_name(name: &str) -> String {
    let file = name.rsplit(['/', '\\']).next().unwrap_or(name);
    let lower = file.to_ascii_lowercase();
    lower
        .strip_suffix(".exe")
        .map_or(lower.clone(), str::to_owned)
}

/// A path for prefix comparison: lowercase, `\` separators, no `\\?\` prefix. `dir` adds a trailing `\`.
pub fn normalize_path(path: &str, dir: bool) -> String {
    let mut p = path.replace('/', "\\").to_ascii_lowercase();
    if let Some(rest) = p.strip_prefix(r"\\?\") {
        p = rest.to_owned();
    }
    if dir && !p.ends_with('\\') {
        p.push('\\');
    }
    p
}

/// What makes another process "busy" for a benchmark: its name (builds, game instances) or where its
/// executable lives (build output and temp copies, whatever the exe is called: `orch-a.exe`, `h1gpucull.exe`).
#[derive(Clone, Debug, Default)]
pub struct BusyRules {
    /// Base names (case-insensitive, `.exe` optional).
    pub names: Vec<String>,
    /// Directories (normalized, trailing `\`) whose executables count.
    pub dirs: Vec<String>,
    /// Path fragments (normalized) that exempt an executable under `dirs`: installed tools such as
    /// `\cargo\bin\` (an MCP server run from a temp toolchain is not a benchmark).
    pub exempt: Vec<String>,
}

impl BusyRules {
    /// Rules for `names` and the directories `dirs` (each kept as given and as canonicalized, which
    /// resolves junctions and 8.3 short names); installed cargo/rustup tools are exempt.
    pub fn new<'a>(names: impl IntoIterator<Item = &'a str>, dirs: impl IntoIterator<Item = std::path::PathBuf>) -> Self {
        let mut rules = Self {
            names: names.into_iter().map(base_name).collect(),
            dirs: Vec::new(),
            exempt: [r"\cargo\bin\", r"\.cargo\bin\", r"\rustup\", r"\.rustup\"].map(str::to_owned).to_vec(),
        };
        for d in dirs {
            let canonical = std::fs::canonicalize(&d).ok();
            for p in std::iter::once(d).chain(canonical) {
                let n = normalize_path(&p.to_string_lossy(), true);
                if n.len() > 3 && !rules.dirs.contains(&n) {
                    rules.dirs.push(n);
                }
            }
        }
        rules
    }

    /// Why `p` is busy (`None` = it isn't).
    pub fn reason(&self, p: &ProcessInfo) -> Option<String> {
        if self.names.contains(&base_name(&p.name)) {
            return Some(p.name.clone());
        }
        let path = normalize_path(p.path.as_deref()?, false);
        let dir = self.dirs.iter().find(|d| path.starts_with(d.as_str()))?;
        if self.exempt.iter().any(|e| path[dir.len() - 1..].contains(e.as_str())) {
            return None;
        }
        Some(format!("{} (under {})", p.name, dir.trim_end_matches('\\')))
    }
}

/// `me`, its ancestors (a driver started by `cargo run` doesn't wait for its own cargo) and its descendants
/// (the game it runs, its `nvidia-smi` samplers).
pub fn family(all: &[ProcessInfo], me: u32) -> Vec<u32> {
    let mut exempt = vec![me];
    let mut cursor = me;
    // Walk up the parent chain (bounded: pid reuse can make cycles).
    for _ in 0..64 {
        let Some(p) = all.iter().find(|p| p.pid == cursor) else {
            break;
        };
        if p.parent == 0 || exempt.contains(&p.parent) {
            break;
        }
        exempt.push(p.parent);
        cursor = p.parent;
    }
    let mut frontier = vec![me];
    while let Some(parent) = frontier.pop() {
        for c in all.iter().filter(|c| c.parent == parent && c.pid != parent) {
            if !exempt.contains(&c.pid) {
                exempt.push(c.pid);
                frontier.push(c.pid);
            }
        }
    }
    exempt
}

/// The running processes [`BusyRules`] flags, except this process's [`family`], each with why.
pub fn busy_processes(rules: &BusyRules) -> io::Result<Vec<(ProcessInfo, String)>> {
    Ok(busy_among(&processes()?, std::process::id(), rules))
}

/// [`busy_processes`] over a given process list.
pub fn busy_among(all: &[ProcessInfo], me: u32, rules: &BusyRules) -> Vec<(ProcessInfo, String)> {
    let exempt = family(all, me);
    all.iter()
        .filter(|p| !exempt.contains(&p.pid))
        .filter_map(|p| rules.reason(p).map(|why| (p.clone(), why)))
        .collect()
}

/// Blocks until [`busy_processes`] finds nothing, checking every `poll`. `on_wait` hears the blockers each
/// time it has to wait.
pub fn wait_until_idle(rules: &BusyRules, poll: Duration, mut on_wait: impl FnMut(&[(ProcessInfo, String)])) -> io::Result<()> {
    loop {
        let busy = busy_processes(rules)?;
        if busy.is_empty() {
            return Ok(());
        }
        on_wait(&busy);
        std::thread::sleep(poll);
    }
}

/// Whole-machine CPU load in percent over about the last second, where the
/// platform reports it (Windows: `Win32_Processor.LoadPercentage`,
/// averaged over packages; elsewhere `None`).
pub fn cpu_load_percent() -> Option<f64> {
    if !cfg!(windows) {
        return None;
    }
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            "(Get-CimInstance Win32_Processor | Measure-Object -Property LoadPercentage -Average).Average",
        ])
        .stderr(Stdio::null())
        .output()
        .ok()?;
    String::from_utf8_lossy(&output.stdout)
        .trim()
        .replace(',', ".")
        .parse()
        .ok()
}

/// Seconds since the Unix epoch.
pub fn unix_now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0.0, |d| d.as_secs_f64())
}

/// One `nvidia-smi` sample.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct GpuSample {
    /// When the line arrived (Unix seconds).
    pub unix_s: f64,
    pub util_pct: f64,
    pub clock_mhz: f64,
    pub mem_clock_mhz: f64,
    pub power_w: f64,
    pub temp_c: f64,
    pub vram_mib: f64,
    /// `clocks_event_reasons.active` bit mask (throttling); 0 = none.
    pub throttle: u64,
}

/// The `--query-gpu` fields, in the order [`parse_smi_line`] reads.
const SMI_QUERY: &str = "utilization.gpu,clocks.gr,clocks.mem,power.draw,temperature.gpu,memory.used,clocks_event_reasons.active";

/// Parses one `--format=csv,noheader,nounits` line of [`SMI_QUERY`].
pub fn parse_smi_line(line: &str, unix_s: f64) -> Option<GpuSample> {
    let f: Vec<&str> = line.split(',').map(str::trim).collect();
    if f.len() < 7 {
        return None;
    }
    let num = |i: usize| f[i].parse::<f64>().ok();
    let hex = f[6].trim_start_matches("0x");
    Some(GpuSample {
        unix_s,
        util_pct: num(0)?,
        clock_mhz: num(1)?,
        mem_clock_mhz: num(2)?,
        // Some laptops report "[N/A]" for power.
        power_w: num(3).unwrap_or(f64::NAN),
        temp_c: num(4)?,
        vram_mib: num(5)?,
        throttle: u64::from_str_radix(hex, 16).unwrap_or(0),
    })
}

/// `nvidia-smi` running in a loop; [`GpuSampler::stop`] returns the samples.
pub struct GpuSampler {
    child: Child,
    reader: JoinHandle<Vec<GpuSample>>,
}

impl GpuSampler {
    /// Starts sampling every `interval_ms`. Fails without `nvidia-smi`.
    pub fn start(interval_ms: u32) -> io::Result<Self> {
        let mut child = Command::new("nvidia-smi")
            .args([
                &format!("--query-gpu={SMI_QUERY}"),
                "--format=csv,noheader,nounits",
                &format!("--loop-ms={interval_ms}"),
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| io::Error::other("no stdout"))?;
        let reader = std::thread::spawn(move || {
            BufReader::new(stdout)
                .lines()
                .map_while(Result::ok)
                .filter_map(|line| parse_smi_line(&line, unix_now()))
                .collect()
        });
        Ok(Self { child, reader })
    }

    pub fn stop(mut self) -> Vec<GpuSample> {
        let _ = self.child.kill();
        let _ = self.child.wait();
        self.reader.join().unwrap_or_default()
    }
}

/// One `nvidia-smi pmon -s u` line: a process with a GPU context and its SM (shader) utilisation.
#[derive(Clone, Debug, PartialEq)]
pub struct GpuProcSample {
    /// When the line arrived (Unix seconds).
    pub unix_s: f64,
    pub pid: u32,
    /// `C` compute, `G` graphics, `C+G` both.
    pub kind: String,
    /// SM utilisation in percent; `None` for `-` (no work in the sample).
    pub sm_pct: Option<f64>,
    /// Process name (truncated to 15 characters by `nvidia-smi`).
    pub name: String,
}

/// Parses one `nvidia-smi pmon -s u` line (`# ...` headers give `None`).
pub fn parse_pmon_line(line: &str, unix_s: f64) -> Option<GpuProcSample> {
    if line.trim_start().starts_with('#') {
        return None;
    }
    // gpu pid type sm mem enc dec [jpg ofa] command
    let f: Vec<&str> = line.split_whitespace().collect();
    if f.len() < 8 {
        return None;
    }
    f[0].parse::<u32>().ok()?;
    let pid = f[1].parse().ok()?;
    let name_at = f.iter().skip(3).position(|c| c.parse::<f64>().is_err() && *c != "-").map_or(f.len(), |i| i + 3);
    Some(GpuProcSample {
        unix_s,
        pid,
        kind: f[2].to_owned(),
        sm_pct: f[3].parse().ok(),
        name: f.get(name_at..).map(|n| n.join(" ")).unwrap_or_default(),
    })
}

/// The processes outside `exempt` whose SM utilisation reached `min_sm_pct` within `[from, to]`, with their
/// peak (one entry per pid, highest first). On Windows (WDDM) every windowed app has a graphics context and
/// is listed at `-`: being listed is not load, the SM share is.
pub fn gpu_contenders(samples: &[GpuProcSample], exempt: &[u32], from: f64, to: f64, min_sm_pct: f64) -> Vec<(u32, String, f64)> {
    let mut out: Vec<(u32, String, f64)> = Vec::new();
    for s in samples.iter().filter(|s| s.unix_s >= from && s.unix_s <= to && !exempt.contains(&s.pid)) {
        let Some(sm) = s.sm_pct.filter(|&sm| sm >= min_sm_pct) else { continue };
        match out.iter_mut().find(|(pid, ..)| *pid == s.pid) {
            Some(e) => e.2 = e.2.max(sm),
            None => out.push((s.pid, s.name.clone(), sm)),
        }
    }
    out.sort_by(|a, b| b.2.total_cmp(&a.2));
    out
}

/// One `nvidia-smi pmon -s u -c 1` snapshot (empty without `nvidia-smi`).
pub fn gpu_processes() -> Vec<GpuProcSample> {
    let Ok(out) = Command::new("nvidia-smi").args(["pmon", "-s", "u", "-c", "1"]).stderr(Stdio::null()).output() else {
        return Vec::new();
    };
    let now = unix_now();
    String::from_utf8_lossy(&out.stdout).lines().filter_map(|l| parse_pmon_line(l, now)).collect()
}

/// `nvidia-smi pmon` running in a loop (one sample a second); [`PmonSampler::stop`] returns the samples.
pub struct PmonSampler {
    child: Child,
    reader: JoinHandle<Vec<GpuProcSample>>,
}

impl PmonSampler {
    /// Starts sampling. Fails without `nvidia-smi`.
    pub fn start() -> io::Result<Self> {
        let mut child = Command::new("nvidia-smi")
            .args(["pmon", "-s", "u", "-d", "1"])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;
        let stdout = child.stdout.take().ok_or_else(|| io::Error::other("no stdout"))?;
        let reader = std::thread::spawn(move || {
            BufReader::new(stdout)
                .lines()
                .map_while(Result::ok)
                .filter_map(|line| parse_pmon_line(&line, unix_now()))
                .collect()
        });
        Ok(Self { child, reader })
    }

    pub fn stop(mut self) -> Vec<GpuProcSample> {
        let _ = self.child.kill();
        let _ = self.child.wait();
        self.reader.join().unwrap_or_default()
    }
}


/// GPU load over a window.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct GpuSummary {
    pub samples: usize,
    pub util_mean: f64,
    pub util_min: f64,
    pub clock_mean_mhz: f64,
    pub clock_min_mhz: f64,
    pub mem_clock_mean_mhz: f64,
    pub power_mean_w: f64,
    pub temp_mean_c: f64,
    pub temp_max_c: f64,
    pub vram_max_mib: f64,
    /// Fraction of samples with a non-zero throttle reason other than idle
    /// (bit 0, "GPU idle").
    pub throttled: f64,
}

impl GpuSummary {
    /// Summary of the samples inside `[from, to]` (Unix seconds).
    pub fn over(samples: &[GpuSample], from: f64, to: f64) -> Option<Self> {
        let window: Vec<&GpuSample> = samples
            .iter()
            .filter(|s| s.unix_s >= from && s.unix_s <= to)
            .collect();
        if window.is_empty() {
            return None;
        }
        #[allow(clippy::cast_precision_loss)] // sample counts are small
        let n = window.len() as f64;
        let mean = |f: fn(&GpuSample) -> f64| {
            let v: Vec<f64> = window
                .iter()
                .map(|s| f(s))
                .filter(|v| v.is_finite())
                .collect();
            #[allow(clippy::cast_precision_loss)]
            let m = if v.is_empty() {
                f64::NAN
            } else {
                v.iter().sum::<f64>() / v.len() as f64
            };
            m
        };
        let min =
            |f: fn(&GpuSample) -> f64| window.iter().map(|s| f(s)).fold(f64::INFINITY, f64::min);
        let max = |f: fn(&GpuSample) -> f64| {
            window
                .iter()
                .map(|s| f(s))
                .fold(f64::NEG_INFINITY, f64::max)
        };
        #[allow(clippy::cast_precision_loss)]
        let throttled = window.iter().filter(|s| s.throttle & !1 != 0).count() as f64 / n;
        Some(Self {
            samples: window.len(),
            util_mean: mean(|s| s.util_pct),
            util_min: min(|s| s.util_pct),
            clock_mean_mhz: mean(|s| s.clock_mhz),
            clock_min_mhz: min(|s| s.clock_mhz),
            mem_clock_mean_mhz: mean(|s| s.mem_clock_mhz),
            power_mean_w: mean(|s| s.power_w),
            temp_mean_c: mean(|s| s.temp_c),
            temp_max_c: max(|s| s.temp_c),
            vram_max_mib: max(|s| s.vram_mib),
            throttled,
        })
    }

    /// One Markdown bullet.
    pub fn to_markdown(&self) -> String {
        format!(
            "- nvidia-smi over the window ({} samples): GPU load **{:.0}%** (min {:.0}%) · {:.0} MHz (min {:.0}) · \
             memory {:.0} MHz · {:.1} W · {:.0} °C (max {:.0}) · VRAM {:.0} MiB · throttled {:.0}% of samples",
            self.samples,
            self.util_mean,
            self.util_min,
            self.clock_mean_mhz,
            self.clock_min_mhz,
            self.mem_clock_mean_mhz,
            self.power_mean_w,
            self.temp_mean_c,
            self.temp_max_c,
            self.vram_max_mib,
            self.throttled * 100.0
        )
    }
}

/// One finished run of a suite.
#[derive(Clone, Debug)]
pub struct SuiteRun {
    /// Scenario name in the suite (`balmora`).
    pub scenario: String,
    /// What else distinguishes the run: resolution, A/B switches.
    pub variant: String,
    /// 1-based repeat index.
    pub repeat: usize,
    pub report: Report,
    pub gpu: Option<GpuSummary>,
}

/// A gauge shown as a column of the suite summary: one per-frame statistic of a counter.
#[derive(Clone, Debug)]
pub struct GaugeColumn {
    pub label: String,
    pub counter: String,
    /// Which per-frame statistic a run shows (the row across repeats takes the median of those).
    pub stat: Stat,
}

impl GaugeColumn {
    /// The counter's per-frame median.
    pub fn median(label: &str, counter: &str) -> Self {
        Self {
            label: label.to_owned(),
            counter: counter.to_owned(),
            stat: Stat::Median,
        }
    }

    /// The counter's per-frame mean (per-thread busy times: a mean adds up over frames, a median hides the
    /// expensive ones).
    pub fn mean(label: &str, counter: &str) -> Self {
        Self {
            label: label.to_owned(),
            counter: counter.to_owned(),
            stat: Stat::Mean,
        }
    }
}

impl SuiteRun {
    /// A gauge column's value in this run.
    pub fn gauge(&self, column: &GaugeColumn) -> Option<f64> {
        self.report
            .counters
            .get(&column.counter)
            .and_then(|c| c.per_frame)
            .map(|s| s.get(column.stat))
    }
}

/// Median of the non-NaN values, if any.
fn median(values: impl IntoIterator<Item = f64>) -> Option<f64> {
    let mut v: Vec<f64> = values.into_iter().filter(|x| !x.is_nan()).collect();
    if v.is_empty() {
        return None;
    }
    v.sort_by(f64::total_cmp);
    Some(percentile_sorted(&v, 50.0))
}

fn cell(v: Option<f64>, decimals: usize) -> String {
    v.map_or("—".to_owned(), |v| format!("{v:.decimals$}"))
}

/// One summary row: frame stats, hitches, GPU load, timed passes, gauges.
struct Row {
    label: String,
    median: Option<f64>,
    p95: Option<f64>,
    p99: Option<f64>,
    worst: Option<f64>,
    hitches: String,
    gpu: [Option<f64>; 4],
    timed: Option<f64>,
    gauges: Vec<Option<f64>>,
}

impl Row {
    fn of_run(r: &SuiteRun, gauges: &[GaugeColumn]) -> Self {
        let f = r.report.frames.as_ref();
        let g = r.gpu;
        Self {
            label: format!("{} {} #{}", r.scenario, r.variant, r.repeat),
            median: f.map(|f| f.stats.median),
            p95: f.map(|f| f.stats.p95),
            p99: f.map(|f| f.stats.p99),
            worst: f.map(|f| f.stats.max),
            hitches: r
                .report
                .hitches
                .as_ref()
                .map_or("—".to_owned(), |h| format!("{}/{}", h.measured, h.warmup)),
            gpu: [
                g.map(|g| g.util_mean),
                g.map(|g| g.clock_mean_mhz),
                g.map(|g| g.power_mean_w),
                g.map(|g| g.temp_mean_c),
            ],
            timed: r.report.timed_gpu_ms(),
            gauges: gauges.iter().map(|c| r.gauge(c)).collect(),
        }
    }

    /// Median of each statistic across `rows`; worst is the max.
    fn across(label: String, rows: &[Self]) -> Self {
        let med = |f: fn(&Self) -> Option<f64>| median(rows.iter().filter_map(f));
        let worst = rows
            .iter()
            .filter_map(|r| r.worst)
            .fold(None, |a: Option<f64>, b| Some(a.map_or(b, |a| a.max(b))));
        let gauges = rows.first().map_or(0, |r| r.gauges.len());
        Self {
            label,
            median: med(|r| r.median),
            p95: med(|r| r.p95),
            p99: med(|r| r.p99),
            worst,
            hitches: rows
                .iter()
                .map(|r| r.hitches.as_str())
                .collect::<Vec<_>>()
                .join(", "),
            gpu: std::array::from_fn(|i| median(rows.iter().filter_map(|r| r.gpu[i]))),
            timed: med(|r| r.timed),
            gauges: (0..gauges)
                .map(|i| median(rows.iter().filter_map(|r| r.gauges[i])))
                .collect(),
        }
    }

    fn write(&self, out: &mut String) {
        let _ = write!(
            out,
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            self.label,
            cell(self.median, 2),
            cell(self.median.map(|m| 1000.0 / m), 0),
            cell(self.p95, 2),
            cell(self.p99, 2),
            cell(self.p99.map(|m| 1000.0 / m), 0),
            cell(self.worst, 1),
            self.hitches,
            cell(self.gpu[0], 0),
            cell(self.gpu[1], 0),
            cell(self.gpu[2], 1),
            cell(self.gpu[3], 0),
            cell(self.timed, 2),
        );
        for g in &self.gauges {
            let _ = write!(out, " {} |", g.map_or("—".to_owned(), big));
        }
        out.push('\n');
    }
}

/// The suite summary: one row per scenario and variant across repeats
/// (median of each run's statistic; worst is the max), then one per run.
pub fn suite_markdown(title: &str, runs: &[SuiteRun], gauges: &[GaugeColumn]) -> String {
    let mut out = format!("# {title}\n\n");
    let mut head = String::from(
        "| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms |",
    );
    for g in gauges {
        let _ = write!(head, " {} |", g.label);
    }
    head.push_str("\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    for _ in gauges {
        head.push_str("---:|");
    }
    head.push('\n');

    let rows: Vec<Row> = runs.iter().map(|r| Row::of_run(r, gauges)).collect();
    let mut groups: Vec<(&str, &str)> = Vec::new();
    for r in runs {
        let key = (r.scenario.as_str(), r.variant.as_str());
        if !groups.contains(&key) {
            groups.push(key);
        }
    }
    let _ = writeln!(out, "## Across repeats (median of runs; worst = max)\n");
    out.push_str(&head);
    for (scenario, variant) in groups {
        let members: Vec<Row> = runs
            .iter()
            .zip(&rows)
            .filter(|(r, _)| r.scenario == scenario && r.variant == variant)
            .map(|(r, _)| Row::of_run(r, gauges))
            .collect();
        Row::across(format!("{scenario} {variant} ×{}", members.len()), &members).write(&mut out);
    }
    let _ = writeln!(out, "\n## Runs\n");
    out.push_str(&head);
    for row in &rows {
        row.write(&mut out);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_lines_parse_on_both_platforms() {
        assert_eq!(
            parse_process_line(r"1234|5678|cargo.exe|C:\Users\me\.cargo\bin\cargo.exe"),
            Some(proc(1234, 5678, "cargo.exe", Some(r"C:\Users\me\.cargo\bin\cargo.exe")))
        );
        assert_eq!(parse_process_line("4|0|System|"), Some(proc(4, 0, "System", None)));
        assert_eq!(parse_process_line("  42     1 /usr/bin/rustc"), Some(proc(42, 1, "/usr/bin/rustc", None)));
        assert_eq!(parse_process_line("garbage"), None);
    }

    fn proc(pid: u32, parent: u32, name: &str, path: Option<&str>) -> ProcessInfo {
        ProcessInfo { pid, parent, name: name.into(), path: path.map(str::to_owned) }
    }

    fn rules() -> BusyRules {
        BusyRules {
            names: ["cargo", "rustc", "game"].map(str::to_owned).to_vec(),
            dirs: vec![normalize_path("C:/cargo-target/scrib", true), normalize_path(r"\\?\C:\Users\me\AppData\Local\Temp", true)],
            exempt: vec![r"\cargo\bin\".into()],
        }
    }

    #[test]
    fn busy_ignores_self_ancestors_and_children() {
        let all = vec![
            proc(1, 0, "explorer.exe", None),
            proc(10, 1, "cargo.exe", None),  // our parent: `cargo run`
            proc(11, 10, "game.exe", None),  // us
            proc(12, 11, "game.exe", None),  // the game we run
            proc(13, 12, "nvidia-smi.exe", None),
            proc(20, 1, "cargo.exe", None),  // someone else's build
            proc(21, 20, "rustc.exe", None), // its compiler
            proc(30, 1, "GAME.EXE", None),   // another game instance
        ];
        let busy = busy_among(&all, 11, &rules());
        let pids: Vec<u32> = busy.iter().map(|(p, _)| p.pid).collect();
        assert_eq!(pids, vec![20, 21, 30]);
        assert_eq!(family(&all, 11), vec![11, 10, 1, 12, 13]);
    }

    #[test]
    fn renamed_bench_exes_are_busy_by_where_they_live() {
        let all = vec![
            proc(11, 1, "perf.exe", None), // us
            proc(40, 1, "orch-a.exe", Some(r"C:\cargo-target\scrib\profiling\orch-a.exe")),
            proc(41, 1, "h1gpucull.exe", Some("c:/CARGO-TARGET/scrib/profiling/h1gpucull.exe")),
            proc(42, 1, "bevyrrowind.exe", Some(r"C:\Users\me\AppData\Local\Temp\bw-owner-1116\bevyrrowind.exe")),
            // An installed tool in a temp toolchain (MCP server) is not a benchmark.
            proc(43, 1, "bevy_brp_mcp.exe", Some(r"C:\Users\me\AppData\Local\Temp\ovid-build-tools\cargo\bin\bevy_brp_mcp.exe")),
            proc(44, 1, "firefox.exe", Some(r"C:\Program Files\Mozilla Firefox\firefox.exe")),
            // A sibling folder with the same prefix is not under the target dir.
            proc(45, 1, "scrib.exe", Some(r"C:\cargo-target\scribble\scrib.exe")),
        ];
        let busy = busy_among(&all, 11, &rules());
        let pids: Vec<u32> = busy.iter().map(|(p, _)| p.pid).collect();
        assert_eq!(pids, vec![40, 41, 42]);
        assert_eq!(busy[0].1, r"orch-a.exe (under c:\cargo-target\scrib)");
    }

    #[test]
    fn pmon_lines_and_contenders() {
        assert_eq!(parse_pmon_line("# gpu         pid   type     sm    mem    enc    dec    jpg    ofa    command ", 0.0), None);
        let idle = parse_pmon_line("    0       1252   C+G      -      -      -      -      -      -    firefox.exe    ", 1.0).unwrap();
        assert_eq!((idle.pid, idle.sm_pct, idle.name.as_str()), (1252, None, "firefox.exe"));
        let ui = parse_pmon_line("    0       8688   C+G      2     28      -      -      -      -    T3 Code (Beta).", 1.0).unwrap();
        assert_eq!((ui.sm_pct, ui.name.as_str()), (Some(2.0), "T3 Code (Beta)."));
        let s = |t: f64, pid: u32, sm: Option<f64>| GpuProcSample { unix_s: t, pid, kind: "C+G".into(), sm_pct: sm, name: format!("p{pid}") };
        let samples = [s(5.0, 7, Some(90.0)), s(10.0, 7, Some(85.0)), s(11.0, 8, Some(40.0)), s(12.0, 8, Some(60.0)), s(12.0, 9, Some(3.0)), s(13.0, 10, None), s(30.0, 11, Some(99.0))];
        // pid 7 is the game (exempt), 9 desktop noise, 10 idle, 11 outside the window.
        assert_eq!(gpu_contenders(&samples, &[7], 8.0, 20.0, 15.0), vec![(8, "p8".to_owned(), 60.0)]);
    }

    #[test]
    fn smi_lines_and_window_summary() {
        let a =
            parse_smi_line("33, 2250, 8001, 29.21, 50, 1608, 0x0000000000000001", 100.0).unwrap();
        assert!((a.util_pct - 33.0).abs() < 1e-9 && a.throttle == 1);
        let b = parse_smi_line("99, 2000, 8001, [N/A], 70, 2000, 0x4", 101.0).unwrap();
        assert!(b.power_w.is_nan());
        let c = parse_smi_line("50, 1000, 8001, 40.0, 60, 1900, 0x0", 200.0).unwrap();
        assert!(parse_smi_line("nonsense", 0.0).is_none());
        let s = GpuSummary::over(&[a, b, c], 99.0, 150.0).unwrap();
        assert_eq!(s.samples, 2);
        assert!((s.util_mean - 66.0).abs() < 1e-9);
        assert!(
            (s.power_mean_w - 29.21).abs() < 1e-9,
            "N/A power is skipped"
        );
        assert!(
            (s.throttled - 0.5).abs() < 1e-9,
            "bit 0 (idle) is not throttling"
        );
        assert!((s.temp_max_c - 70.0).abs() < 1e-9);
        assert!(GpuSummary::over(&[a], 0.0, 1.0).is_none());
    }

    #[test]
    fn suite_summary_aggregates_repeats() {
        use crate::{Counters, Recorder, unit};
        let run = |repeat: usize, ms: f64, meshes: f64| {
            let counters = Counters::new();
            let mut rec = Recorder::new();
            for i in 0..20 {
                counters.set_named("world/meshes", unit::COUNT, meshes);
                rec.commit_frame(if i == 5 { ms * 4.0 } else { ms }, &counters);
            }
            let mut report = Report::build("x", &rec, &[], 1.5, std::collections::BTreeMap::new());
            report.add_hitches(&rec, None, None);
            SuiteRun {
                scenario: "town".into(),
                variant: "1280x720".into(),
                repeat,
                report,
                gpu: None,
            }
        };
        let runs = vec![
            run(1, 10.0, 1000.0),
            run(2, 20.0, 3000.0),
            run(3, 12.0, 2000.0),
        ];
        let gauges = [GaugeColumn::median("meshes", "world/meshes")];
        let md = suite_markdown("t", &runs, &gauges);
        // Median of medians 12 ms (83 fps); worst = max of worsts (80 ms).
        assert!(md.contains("| town 1280x720 ×3 | 12.00 | 83 |"), "{md}");
        assert!(md.contains("| 80.0 | 1/0, 1/0, 1/0 |"), "{md}");
        assert!(md.contains("| 2000 |"), "{md}");
        assert!(md.contains("| town 1280x720 #2 | 20.00 | 50 |"), "{md}");
    }
}
