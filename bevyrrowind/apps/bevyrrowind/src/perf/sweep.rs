//! `bevyrrowind perf sweep`: one variable stepped per run over a fixed
//! scenario (the light lab, `docs/perf.md` "Light lab"). Every step runs
//! through the suite's runner (quiet-machine waits, busy retries, one report
//! per run); the sweep adds a table of medians over the repeats per value:
//! frame median and p95, timed GPU, the main opaque pass, the shadow views
//! (`auxiliary_views`) and the render thread.

use std::fmt::Write as _;
use std::path::Path;

use sf_diag_bevy::sf_diag::Report;

use super::cli::{BUSY, Planned, SCENARIOS, SuiteArgs, execute, write_summary};

#[derive(clap::Args)]
pub struct SweepArgs {
    #[command(flatten)]
    suite: SuiteArgs,
    /// The variable: a view flag without its dashes (`point-shadows`, `lab-lights`, `range`,
    /// `lab-radius`, `lab-orbit`, ...), `off` (an `--off` part; `none` for the baseline),
    /// `a+b` (flags stepped together, values `1+2`), `resolution` (`WxH`), or `env:KEY` (an
    /// environment variable, e.g.
    /// `env:BEVYRROWIND_SHADOW_FILTER`).
    #[arg(long)]
    var: String,
    /// The values, in table order.
    #[arg(long, value_delimiter = ',', required = true)]
    values: Vec<String>,
}

/// One value of the swept variable: what it adds to a run.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Step {
    /// `var=value`, in run names and tables.
    pub label: String,
    pub args: Vec<String>,
    pub off: Vec<String>,
    pub envs: Vec<(String, String)>,
}

impl Step {
    fn new(var: &str, value: &str) -> Self {
        let mut step = Self { label: format!("{var}={value}"), ..Self::default() };
        if let Some(key) = var.strip_prefix("env:") {
            step.envs.push((key.to_owned(), value.to_owned()));
        } else if var == "off" {
            step.off.extend(value.split('+').filter(|v| *v != "none").map(str::to_owned));
        } else if var.contains('+') {
            // Flags stepped together (`point-shadows+lab-lights` = `16+16`), value by value.
            for (flag, value) in var.split('+').zip(value.split('+')) {
                step.args.extend([format!("--{}", flag.trim_start_matches('-')), value.to_owned()]);
            }
        } else if var != "resolution" {
            step.args.extend([format!("--{}", var.trim_start_matches('-')), value.to_owned()]);
        }
        step
    }
}

/// The table's columns: (header, how to read it from a run's report).
/// A table column: header, and how to read it from a run's report.
type Column = (&'static str, fn(&Report) -> Option<f64>);

const COLUMNS: &[Column] = &[
    ("frame median ms", |r| r.frames.as_ref().map(|f| f.stats.median)),
    ("frame p95 ms", |r| r.frames.as_ref().map(|f| f.stats.p95)),
    ("timed GPU ms", Report::timed_gpu_ms),
    ("main opaque GPU ms", |r| counter(r, "render/main_opaque_pass_3d/elapsed_gpu")),
    ("shadow views GPU ms", |r| counter(r, "render/auxiliary_views/elapsed_gpu")),
    ("render busy ms", |r| counter(r, "render/busy/ms")),
    ("render thread ms", |r| counter(r, "render/thread/ms")),
    ("point lights", |r| counter(r, "world/point_lights")),
];

fn counter(report: &Report, name: &str) -> Option<f64> {
    report.counters.get(name).and_then(|c| c.per_frame).map(|s| s.median)
}

fn median(mut values: Vec<f64>) -> Option<f64> {
    values.retain(|v| v.is_finite());
    values.sort_by(f64::total_cmp);
    let n = values.len();
    (n > 0).then(|| if n % 2 == 1 { values[n / 2] } else { f64::midpoint(values[n / 2 - 1], values[n / 2]) })
}

pub fn run(args: &SweepArgs) -> Result<(), Box<dyn std::error::Error>> {
    let suite = &args.suite;
    let exe = match &suite.exe {
        Some(p) => p.clone(),
        None => std::env::current_exe()?,
    };
    if !Path::new("benchmarks").is_dir() {
        return Err("run from the bevyrrowind repo root (scenario paths are relative to it)".into());
    }
    let [name] = suite.scenarios.as_slice() else { return Err("a sweep takes exactly one scenario".into()) };
    let scenario = SCENARIOS.iter().find(|s| s.name == name).ok_or_else(|| format!("unknown scenario {name:?}; see `perf list`"))?;
    let base_resolution = suite.resolutions.first().cloned().unwrap_or_else(|| "1280x720".to_owned());
    // Repeats interleave (value 1, 2, ..., then again): drift (heat, clocks) spreads over every value.
    let mut plan = Vec::new();
    for repeat in 1..=suite.repeat.max(1) {
        for value in &args.values {
            let resolution = if args.var == "resolution" { value.clone() } else { base_resolution.clone() };
            plan.push(Planned { scenario, resolution, repeat, step: Some(Step::new(&args.var, value)) });
        }
    }
    let dir = suite.out.join(&suite.tag);
    std::fs::create_dir_all(&dir)?;
    let mut busy: Vec<&str> = BUSY.to_vec();
    busy.extend(suite.wait_for.iter().map(String::as_str));
    println!("sweep {} over {:?}: {} runs → {}", args.var, args.values, plan.len(), dir.display());
    let (runs, notes) = execute(suite, &exe, &dir, &plan, &busy);
    if suite.dry_run || runs.is_empty() {
        return Ok(());
    }

    let mut md = format!("# Sweep `{}`: `{}` over {}\n\n", suite.tag, args.var, scenario.name);
    let _ = writeln!(md, "- `bevyrrowind {}` at {base_resolution}; off: {}", scenario.args.join(" "), if suite.off.is_empty() { "none".to_owned() } else { suite.off.join(",") });
    let _ = writeln!(md, "- medians over repeats of per-run medians (frame stats: per-run median / p95); {} runs", runs.len());
    if notes.is_empty() {
        let _ = writeln!(md, "- machine: quiet for every run");
    }
    for n in &notes {
        let _ = writeln!(md, "- **busy machine**: {n}");
    }
    let header: Vec<&str> = COLUMNS.iter().map(|c| c.0).collect();
    let _ = writeln!(md, "\n| {} | runs | {} |\n|---|---:|{}", args.var, header.join(" | "), "---:|".repeat(COLUMNS.len()));
    let mut csv = format!("{},runs,{}\n", args.var, header.iter().map(|h| h.replace(' ', "_")).collect::<Vec<_>>().join(","));
    for value in &args.values {
        let label = Step::new(&args.var, value).label;
        let reports: Vec<&Report> = runs.iter().filter(|(i, _)| plan[*i].step.as_ref().is_some_and(|s| s.label == label)).map(|(_, r)| &r.report).collect();
        let cells: Vec<Option<f64>> = COLUMNS.iter().map(|(_, read)| median(reports.iter().filter_map(|r| read(r)).collect())).collect();
        let fmt = |c: &Option<f64>, sep: &str| c.map_or(sep.to_owned(), |v| format!("{v:.2}"));
        let _ = writeln!(md, "| {value} | {} | {} |", reports.len(), cells.iter().map(|c| fmt(c, "?")).collect::<Vec<_>>().join(" | "));
        let _ = writeln!(csv, "{value},{},{}", reports.len(), cells.iter().map(|c| fmt(c, "")).collect::<Vec<_>>().join(","));
    }
    std::fs::write(dir.join("sweep.md"), &md)?;
    std::fs::write(dir.join("sweep.csv"), &csv)?;
    print!("\n{md}");
    let suite_runs: Vec<_> = runs.into_iter().map(|(_, r)| r).collect();
    write_summary(suite, &exe, &dir, &suite_runs, &notes, &busy)?;
    if let Some(save) = &suite.save {
        std::fs::write(save.join("sweep.md"), &md)?;
        std::fs::write(save.join("sweep.csv"), &csv)?;
    }
    println!("wrote {}", dir.join("sweep.md").display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps_map_to_flags_off_parts_and_env() {
        assert_eq!(Step::new("point-shadows", "4").args, ["--point-shadows", "4"]);
        assert_eq!(Step::new("point-shadows+lab-lights", "16+16").args, ["--point-shadows", "16", "--lab-lights", "16"]);
        assert_eq!(Step::new("off", "none").off, Vec::<String>::new());
        assert_eq!(Step::new("off", "contactshadows+shadowcache").off, ["contactshadows", "shadowcache"]);
        assert_eq!(Step::new("env:BEVYRROWIND_SHADOW_FILTER", "hardware").envs, [("BEVYRROWIND_SHADOW_FILTER".to_owned(), "hardware".to_owned())]);
        assert!(Step::new("resolution", "960x600").args.is_empty());
        assert_eq!(median(vec![3.0, 1.0, 2.0, 10.0]), Some(2.5));
    }
}
