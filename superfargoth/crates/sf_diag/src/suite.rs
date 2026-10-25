//! Running benchmarks from a driver process: waiting for a quiet machine,
//! sampling the GPU alongside, and summarizing repeated runs.
//!
//! Numbers from a busy machine are noise: [`busy_processes`] lists the
//! processes (by name) that must not run during a measurement, ignoring
//! the driver itself and its ancestors (a driver started by `cargo run`
//! doesn't wait for its own cargo). [`GpuSampler`] records
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
use crate::stats::percentile_sorted;

/// One running process.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProcessInfo {
    pub pid: u32,
    pub parent: u32,
    pub name: String,
}

/// Every running process (name, pid, parent pid).
pub fn processes() -> io::Result<Vec<ProcessInfo>> {
    let output = if cfg!(windows) {
        Command::new("powershell")
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                "Get-CimInstance Win32_Process | ForEach-Object { \"$($_.ProcessId),$($_.ParentProcessId),$($_.Name)\" }",
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

/// `pid,ppid,name` (Windows) or `pid ppid name` (`ps`).
fn parse_process_line(line: &str) -> Option<ProcessInfo> {
    let line = line.trim();
    let (pid, rest) = line.split_once([',', ' '])?;
    let rest = rest.trim_start();
    let (parent, name) = rest.split_once([',', ' '])?;
    Some(ProcessInfo {
        pid: pid.trim().parse().ok()?,
        parent: parent.trim().parse().ok()?,
        name: name.trim().to_owned(),
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

/// The running processes named in `names` (case-insensitive, `.exe`
/// optional), except this process and its ancestors.
pub fn busy_processes(names: &[&str]) -> io::Result<Vec<ProcessInfo>> {
    Ok(busy_among(&processes()?, std::process::id(), names))
}

fn busy_among(all: &[ProcessInfo], me: u32, names: &[&str]) -> Vec<ProcessInfo> {
    let wanted: Vec<String> = names.iter().map(|n| base_name(n)).collect();
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
    all.iter()
        .filter(|p| !exempt.contains(&p.pid) && wanted.contains(&base_name(&p.name)))
        .cloned()
        .collect()
}

/// Blocks until none of `names` runs (see [`busy_processes`]), checking
/// every `poll`. `on_wait` hears the blockers each time it has to wait.
pub fn wait_until_idle(
    names: &[&str],
    poll: Duration,
    mut on_wait: impl FnMut(&[ProcessInfo]),
) -> io::Result<()> {
    loop {
        let busy = busy_processes(names)?;
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

/// A gauge shown as a column of the suite summary (its median).
#[derive(Clone, Debug)]
pub struct GaugeColumn {
    pub label: String,
    pub counter: String,
}

impl SuiteRun {
    fn gauge(&self, counter: &str) -> Option<f64> {
        self.report
            .counters
            .get(counter)
            .and_then(|c| c.per_frame)
            .map(|s| s.median)
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
            gauges: gauges.iter().map(|c| r.gauge(&c.counter)).collect(),
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
            parse_process_line("1234,5678,cargo.exe"),
            Some(ProcessInfo {
                pid: 1234,
                parent: 5678,
                name: "cargo.exe".into()
            })
        );
        assert_eq!(
            parse_process_line("  42     1 /usr/bin/rustc"),
            Some(ProcessInfo {
                pid: 42,
                parent: 1,
                name: "/usr/bin/rustc".into()
            })
        );
        assert_eq!(parse_process_line("garbage"), None);
    }

    #[test]
    fn busy_ignores_self_and_ancestors() {
        let p = |pid, parent, name: &str| ProcessInfo {
            pid,
            parent,
            name: name.into(),
        };
        let all = vec![
            p(1, 0, "explorer.exe"),
            p(10, 1, "cargo.exe"),  // our parent: `cargo run`
            p(11, 10, "game.exe"),  // us
            p(20, 1, "cargo.exe"),  // someone else's build
            p(21, 20, "rustc.exe"), // its compiler
            p(30, 1, "GAME.EXE"),   // another game instance
        ];
        let busy = busy_among(&all, 11, &["cargo", "rustc.exe", "game"]);
        let pids: Vec<u32> = busy.iter().map(|p| p.pid).collect();
        assert_eq!(pids, vec![20, 21, 30]);
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
        let gauges = [GaugeColumn {
            label: "meshes".into(),
            counter: "world/meshes".into(),
        }];
        let md = suite_markdown("t", &runs, &gauges);
        // Median of medians 12 ms (83 fps); worst = max of worsts (80 ms).
        assert!(md.contains("| town 1280x720 ×3 | 12.00 | 83 |"), "{md}");
        assert!(md.contains("| 80.0 | 1/0, 1/0, 1/0 |"), "{md}");
        assert!(md.contains("| 2000 |"), "{md}");
        assert!(md.contains("| town 1280x720 #2 | 20.00 | 50 |"), "{md}");
    }
}
