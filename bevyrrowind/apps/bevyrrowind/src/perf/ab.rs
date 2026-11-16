//! Interleaved A/B arms, run provenance and the A/B verdict table of `perf suite`
//! (`docs/reviews/perf-verdict-audit-2026-11-16.md` §5 fixes 1, 4, 5; decision rule in `sf_diag::ab`).

use std::fmt::Write as _;
use std::path::Path;
use std::process::Command;

use sf_diag_bevy::sf_diag::ab::{self, Outcome};
use sf_diag_bevy::sf_diag::pacing::{GPU_TIMED, MAIN_BUSY, RENDER_BUSY};
use sf_diag_bevy::sf_diag::suite::{GaugeColumn, SuiteRun};

/// What a run changes from the shipped game: the per-run settings `perf suite` takes, also inside an `--arm`.
#[derive(clap::Args, Clone, Debug, Default, PartialEq, Eq)]
pub struct RunSettings {
    /// A/B: parts to leave out (see `view --help`, `--off`).
    #[arg(long, value_delimiter = ',')]
    pub off: Vec<String>,
    /// Extra environment for the game, KEY=VALUE (repeatable).
    #[arg(long = "env")]
    pub envs: Vec<String>,
    /// Extra game flag for every run (repeatable; `--arg=--scene`, a flag with a value takes two:
    /// `--arg=--sun-cascades --arg=2:205`): an A/B of a switch that is not an `--off` part. Shown in the variant name.
    #[arg(long = "arg", allow_hyphen_values = true)]
    pub args: Vec<String>,
}

impl RunSettings {
    /// `self` then `more` (an arm's settings on top of the suite's).
    pub fn with(&self, more: &Self) -> Self {
        let cat = |a: &[String], b: &[String]| a.iter().chain(b).cloned().collect();
        Self { off: cat(&self.off, &more.off), envs: cat(&self.envs, &more.envs), args: cat(&self.args, &more.args) }
    }

    /// The `--env` pairs, split (`KEY=VALUE` checked).
    pub fn env_pairs(&self) -> Result<Vec<(String, String)>, String> {
        self.envs
            .iter()
            .map(|pair| pair.split_once('=').map(|(k, v)| (k.to_owned(), v.to_owned())).ok_or_else(|| format!("--env {pair:?}: expected KEY=VALUE")))
            .collect()
    }

    /// The settings as `perf suite` flags (`--off=a,b --env K=V --arg=x`); empty for the shipped game.
    pub fn describe(&self) -> String {
        let mut parts = Vec::new();
        if !self.off.is_empty() {
            parts.push(format!("--off={}", self.off.join(",")));
        }
        parts.extend(self.envs.iter().map(|e| format!("--env {e}")));
        parts.extend(self.args.iter().map(|a| format!("--arg={a}")));
        parts.join(" ")
    }
}

/// One arm of an interleaved A/B: a name and the settings it adds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arm {
    pub name: String,
    pub settings: RunSettings,
}

/// Parses `--arm NAME:SETTINGS`: SETTINGS are `perf suite`'s own per-run flags (`--off`, `--env`, `--arg`),
/// whitespace-separated, parsed by the same definitions; empty for the shipped game (`base:`).
pub fn parse_arm(spec: &str) -> Result<Arm, String> {
    #[derive(clap::Parser)]
    #[command(no_binary_name = true)]
    struct ArmCli {
        #[command(flatten)]
        settings: RunSettings,
    }
    let (name, rest) = spec.split_once(':').ok_or_else(|| format!("--arm {spec:?}: expected NAME:SETTINGS (e.g. `scene:--arg=--scene`, `base:`)"))?;
    if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-')) {
        return Err(format!("--arm {spec:?}: the name must be letters, digits, `_` or `-`"));
    }
    let cli = <ArmCli as clap::Parser>::try_parse_from(rest.split_whitespace())
        .map_err(|e| format!("--arm {name}: {}", e.to_string().lines().next().unwrap_or("bad settings").trim_start_matches("error: ")))?;
    cli.settings.env_pairs().map_err(|e| format!("--arm {name}: {e}"))?;
    Ok(Arm { name: name.to_owned(), settings: cli.settings })
}

/// Parses every `--arm`: none, or at least two with distinct names.
pub fn parse_arms(specs: &[String]) -> Result<Vec<Arm>, String> {
    let arms = specs.iter().map(|s| parse_arm(s)).collect::<Result<Vec<_>, _>>()?;
    if arms.len() == 1 {
        return Err("--arm: an A/B needs at least two arms (the first is the base: `--arm base:`)".into());
    }
    for (i, a) in arms.iter().enumerate() {
        if arms[..i].iter().any(|b| b.name == a.name) {
            return Err(format!("--arm: duplicate arm name {:?}", a.name));
        }
    }
    Ok(arms)
}

/// The DLSS mode a run asks for (`BEVYRROWIND_DLSS` from its env, else inherited, or `--dlss` in its game
/// flags), when it is a DLSS mode; `None` for TAA, off or unset.
pub fn dlss_requested(envs: &[(String, String)], inherited: Option<&str>, game_args: &[String]) -> Option<String> {
    let from_flag = game_args.iter().position(|a| a == "--dlss").and_then(|i| game_args.get(i + 1)).map(String::as_str);
    let from_flag = from_flag.or_else(|| game_args.iter().find_map(|a| a.strip_prefix("--dlss=")));
    let from_env = envs.iter().rev().find(|(k, _)| k == "BEVYRROWIND_DLSS").map(|(_, v)| v.as_str()).or(inherited);
    let mode = from_flag.or(from_env)?.trim().to_ascii_lowercase();
    (!mode.is_empty() && !matches!(mode.as_str(), "off" | "taa" | "none" | "msaa" | "fxaa" | "smaa")).then_some(mode)
}

/// Whether a run report timed a DLSS pass (`render/dlss_super_resolution/elapsed_gpu`, or ray reconstruction).
pub fn dlss_timed(counters: impl IntoIterator<Item = impl AsRef<str>>) -> bool {
    counters.into_iter().any(|c| {
        let c = c.as_ref();
        c.contains("dlss_") && c.ends_with("/elapsed_gpu")
    })
}

/// Where the provenance header reads the machine (injectable: tests run without a shell).
pub trait Probe {
    fn now_unix_s(&self) -> f64;
    /// `git rev-parse HEAD`.
    fn git_head(&self) -> Option<String>;
    /// Tracked files changed (`git status --porcelain --untracked-files=no` lines).
    fn git_changed_files(&self) -> Option<usize>;
    fn sha256(&self, file: &Path) -> Option<String>;
    fn mtime_unix_s(&self, file: &Path) -> Option<f64>;
    /// `powercfg /getactivescheme` output.
    fn power_scheme(&self) -> Option<String>;
    /// `Win32_Battery.BatteryStatus` (`None`: no battery).
    fn battery_status(&self) -> Option<u32>;
    /// The power-mode registry key (`reg query ...\Power\User\PowerSchemes`; `power::parse_overlay`).
    fn power_overlay(&self) -> Option<String>;
    /// The bake cache's newest profile: name and modification time.
    fn bake_profile(&self) -> Option<(String, f64)>;
}

/// [`Probe`] on this machine (git, certutil / sha256sum, powercfg, PowerShell CIM).
pub struct MachineProbe;

fn output(program: &str, args: &[&str]) -> Option<String> {
    let out = Command::new(program).args(args).output().ok()?;
    out.status.success().then(|| String::from_utf8_lossy(&out.stdout).into_owned())
}

impl Probe for MachineProbe {
    fn now_unix_s(&self) -> f64 {
        sf_diag_bevy::sf_diag::suite::unix_now()
    }

    fn git_head(&self) -> Option<String> {
        output("git", &["rev-parse", "HEAD"]).map(|s| s.trim().to_owned()).filter(|s| !s.is_empty())
    }

    fn git_changed_files(&self) -> Option<usize> {
        output("git", &["status", "--porcelain", "--untracked-files=no"]).map(|s| s.lines().filter(|l| !l.trim().is_empty()).count())
    }

    fn sha256(&self, file: &Path) -> Option<String> {
        let path = file.to_string_lossy();
        let out = if cfg!(windows) { output("certutil", &["-hashfile", &path, "SHA256"]) } else { output("sha256sum", &[&path]) };
        out.as_deref().and_then(parse_sha256)
    }

    fn mtime_unix_s(&self, file: &Path) -> Option<f64> {
        let t = std::fs::metadata(file).ok()?.modified().ok()?;
        Some(t.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs_f64())
    }

    fn power_scheme(&self) -> Option<String> {
        output("powercfg", &["/getactivescheme"])
    }

    fn battery_status(&self) -> Option<u32> {
        let out = output("powershell", &["-NoProfile", "-NonInteractive", "-Command", "(Get-CimInstance Win32_Battery).BatteryStatus"])?;
        out.lines().find_map(|l| l.trim().parse().ok())
    }

    fn power_overlay(&self) -> Option<String> {
        super::power::overlay_reg()
    }

    fn bake_profile(&self) -> Option<(String, f64)> {
        let dir = mw_bake::default_cache_root()?.join("profiles");
        std::fs::read_dir(dir)
            .ok()?
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|e| e == "manifest"))
            .filter_map(|p| Some((p.file_stem()?.to_string_lossy().into_owned(), self.mtime_unix_s(&p)?)))
            .max_by(|a, b| a.1.total_cmp(&b.1))
    }
}

/// The hash in `certutil -hashfile` or `sha256sum` output: the first 64-hex-digit word (certutil may space it).
pub fn parse_sha256(out: &str) -> Option<String> {
    out.lines().find_map(|l| {
        let compact: String = l.split_whitespace().take_while(|w| w.chars().all(|c| c.is_ascii_hexdigit())).collect();
        (compact.len() == 64).then(|| compact.to_ascii_lowercase())
    })
}

/// `powercfg /getactivescheme` → `Balanced (381b4222-...)`.
pub fn parse_power_scheme(out: &str) -> Option<String> {
    let rest = out.lines().find_map(|l| l.split_once("GUID:").map(|(_, r)| r.trim()))?;
    let (guid, name) = rest.split_once(char::is_whitespace).map_or((rest, ""), |(g, n)| (g, n.trim()));
    let name = name.trim_start_matches('(').trim_end_matches(')');
    Some(if name.is_empty() { guid.to_owned() } else { format!("{name} ({guid})") })
}

/// AC or battery from `Win32_Battery.BatteryStatus` (1 discharging, 2 on AC, 3 full, 4-5 low/critical, 6-9 charging).
pub fn power_source(status: Option<u32>) -> String {
    match status {
        None => "AC (no battery)".to_owned(),
        Some(2 | 3 | 6..=9) => "AC".to_owned(),
        Some(1 | 4 | 5) => "**battery**".to_owned(),
        Some(s) => format!("unknown (battery status {s})"),
    }
}

/// `YYYY-MM-DD HH:MM:SS UTC` (civil-from-days, Howard Hinnant).
pub fn utc(unix_s: f64) -> String {
    #[allow(clippy::cast_possible_truncation)] // seconds since 1970 fit an i64
    let secs = unix_s.floor() as i64;
    let (days, rem) = (secs.div_euclid(86_400), secs.rem_euclid(86_400));
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02} {:02}:{:02}:{:02} UTC", rem / 3600, rem % 3600 / 60, rem % 60)
}

/// The provenance lines of a summary: when, which commit, which exe, which bake, which power state.
pub fn provenance_lines(exe: &Path, probe: &dyn Probe) -> Vec<String> {
    let unknown = || "?".to_owned();
    let commit = match (probe.git_head(), probe.git_changed_files()) {
        (Some(h), Some(0)) => format!("`{h}` (clean)"),
        (Some(h), Some(n)) => format!("`{h}` (**dirty**: {n} tracked files changed)"),
        (Some(h), None) => format!("`{h}` (dirty: ?)"),
        (None, _) => unknown(),
    };
    let bake = probe.bake_profile().map_or_else(unknown, |(name, t)| format!("profile `{name}`, manifest {}", utc(t)));
    vec![
        format!("- date: {}", utc(probe.now_unix_s())),
        format!("- commit: {commit}"),
        format!(
            "- exe sha256: `{}` · modified {}",
            probe.sha256(exe).unwrap_or_else(unknown),
            probe.mtime_unix_s(exe).map_or_else(unknown, utc)
        ),
        format!("- bake: {bake}"),
        {
            let battery = probe.battery_status();
            let on_battery = matches!(battery, Some(1 | 4 | 5));
            format!(
                "- power: plan {} · mode {} · {}",
                probe.power_scheme().as_deref().and_then(parse_power_scheme).unwrap_or_else(unknown),
                probe.power_overlay().as_deref().and_then(|r| super::power::parse_overlay(r, on_battery)).map_or_else(unknown, |g| super::power::overlay_name(&g)),
                power_source(battery)
            )
        },
    ]
}

/// `--save` must not overwrite an earlier suite: refuse an existing folder with files in it.
pub fn check_save(save: Option<&Path>) -> Result<(), String> {
    let Some(save) = save else { return Ok(()) };
    match std::fs::read_dir(save).map(|mut entries| entries.next().is_some()) {
        Ok(true) => {
            Err(format!("--save {}: the folder already holds results; pick a new tag folder (earlier results are never overwritten)", save.display()))
        }
        _ => Ok(()),
    }
}

/// The per-thread columns, per-frame means (UE `stat unit`: game / draw / GPU beside the frame).
pub fn thread_gauges() -> [GaugeColumn; 4] {
    [
        GaugeColumn::mean("main busy ms (mean)", MAIN_BUSY),
        GaugeColumn::mean("render busy ms (mean)", RENDER_BUSY),
        GaugeColumn::mean("render graph ms (mean)", "render/graph/ms"),
        GaugeColumn::mean("GPU timed ms (mean)", GPU_TIMED),
    ]
}

/// A kept run of an A/B: its arm and repeat.
pub struct ArmRun<'a> {
    pub arm: usize,
    pub run: &'a SuiteRun,
}

fn frame_median(r: &SuiteRun) -> Option<f64> {
    r.report.frames.as_ref().map(|f| f.stats.median)
}

/// The "A/B verdicts" section: per scenario and resolution, each arm against the first (median of run medians,
/// min-max, delta, paired delta, Mann-Whitney, per-thread mean deltas, outcome by [`ab::verdict`]).
pub fn verdict_markdown(arms: &[Arm], runs: &[ArmRun<'_>], groups: &[(String, String)]) -> String {
    let mut out = String::new();
    let Some(base) = arms.first() else { return out };
    let _ = writeln!(
        out,
        "## A/B verdicts vs `{}`\n\nRule (set before the runs): an arm differs when |Δ| > the larger arm's min-max range of run medians, or \
         Mann-Whitney p < {}; else **inconclusive** (keep the code opt-in, never delete on it). Δ = median of the arm's run medians minus the \
         base's; paired Δ = median over repeats of (arm - base) in the same repeat; thread Δ = per-frame means (main busy / render busy / \
         render graph / GPU timed), median over runs.\n",
        base.name,
        ab::ALPHA
    );
    out.push_str("| scenario | arm | runs | median ms | min-max | Δ ms | paired Δ | U | p | main Δ | render Δ | graph Δ | GPU Δ | verdict |\n");
    out.push_str("|---|---|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---|\n");
    let gauges = thread_gauges();
    for (scenario, resolution) in groups {
        let of = |arm: usize| -> Vec<&SuiteRun> {
            runs.iter().filter(|r| r.arm == arm && &r.run.scenario == scenario && r.run.variant.starts_with(resolution.as_str())).map(|r| r.run).collect()
        };
        let medians = |rs: &[&SuiteRun]| -> Vec<f64> { rs.iter().filter_map(|r| frame_median(r)).collect() };
        let thread = |rs: &[&SuiteRun], g: &GaugeColumn| ab::median(&rs.iter().filter_map(|r| r.gauge(g)).collect::<Vec<_>>());
        let base_runs = of(0);
        let base_medians = medians(&base_runs);
        for (i, arm) in arms.iter().enumerate() {
            let arm_runs = of(i);
            let arm_medians = medians(&arm_runs);
            let cell = |v: Option<f64>| v.map_or("—".to_owned(), |v| format!("{v:+.2}"));
            let range = |v: &[f64]| {
                let lo = v.iter().copied().fold(f64::INFINITY, f64::min);
                let hi = v.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                if v.is_empty() { "—".to_owned() } else { format!("{lo:.2}-{hi:.2}") }
            };
            let median = ab::median(&arm_medians).map_or("—".to_owned(), |m| format!("{m:.2}"));
            if i == 0 {
                let _ = writeln!(out, "| {scenario} @ {resolution} | `{}` (base) | {} | {median} | {} | | | | | | | | | |", arm.name, arm_runs.len(), range(&arm_medians));
                continue;
            }
            let paired: Vec<f64> = arm_runs
                .iter()
                .filter_map(|a| {
                    let b = base_runs.iter().find(|b| b.repeat == a.repeat)?;
                    Some(frame_median(a)? - frame_median(b)?)
                })
                .collect();
            let thread_delta: Vec<String> = gauges.iter().map(|g| cell(thread(&arm_runs, g).zip(thread(&base_runs, g)).map(|(a, b)| a - b))).collect();
            let (delta, u, p, outcome) = match ab::verdict(&base_medians, &arm_medians) {
                Some(v) => (
                    format!("{:+.2}", v.delta_ms),
                    format!("{:.1}", v.test.u),
                    format!("{:.3}{}", v.test.p, if v.test.exact { "" } else { " (approx.)" }),
                    match v.outcome {
                        Outcome::Inconclusive => "inconclusive".to_owned(),
                        o => format!("**{o}**"),
                    },
                ),
                None => ("—".into(), "—".into(), "—".into(), "no runs".into()),
            };
            let _ = writeln!(
                out,
                "| {scenario} @ {resolution} | `{}` | {} | {median} | {} | {delta} | {} | {u} | {p} | {} | {outcome} |",
                arm.name,
                arm_runs.len(),
                range(&arm_medians),
                cell(ab::median(&paired)),
                thread_delta.join(" | ")
            );
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn arms_parse_with_the_suite_flags() {
        assert_eq!(parse_arm("base:").unwrap(), Arm { name: "base".into(), settings: RunSettings::default() });
        let a = parse_arm("cas2:--arg=--sun-cascades --arg=2:205 --env BEVYRROWIND_GPU_CULL_STATICS=1 --off=precombine,npcs").unwrap();
        assert_eq!(a.name, "cas2");
        assert_eq!(a.settings.args, ["--sun-cascades", "2:205"]);
        assert_eq!(a.settings.envs, ["BEVYRROWIND_GPU_CULL_STATICS=1"]);
        assert_eq!(a.settings.off, ["precombine", "npcs"]);
        assert_eq!(a.settings.describe(), "--off=precombine,npcs --env BEVYRROWIND_GPU_CULL_STATICS=1 --arg=--sun-cascades --arg=2:205");
        assert_eq!(parse_arm("scene: --arg=--scene ").unwrap().settings.args, ["--scene"]);
        // Errors: no colon, bad name, unknown flag, a bare value, a bad env pair.
        assert!(parse_arm("base").unwrap_err().contains("expected NAME:SETTINGS"));
        assert!(parse_arm(":--off=x").unwrap_err().contains("the name must be"));
        assert!(parse_arm("a b:--off=x").unwrap_err().contains("the name must be"));
        assert!(parse_arm("x:--sun-cascades 2:205").unwrap_err().starts_with("--arm x: "));
        assert!(parse_arm("x:scene").is_err());
        assert!(parse_arm("x:--env NOEQUALS").unwrap_err().contains("expected KEY=VALUE"));
        // The set: none, or two or more distinct names.
        assert!(parse_arms(&[]).unwrap().is_empty());
        assert!(parse_arms(&["base:".into()]).unwrap_err().contains("at least two arms"));
        assert!(parse_arms(&["a:".into(), "a:--off=x".into()]).unwrap_err().contains("duplicate"));
        assert_eq!(parse_arms(&["a:".into(), "b:--off=x".into(), "c:--arg=--scene".into()]).unwrap().len(), 3);
        // Arm settings go on top of the suite's.
        let suite = RunSettings { off: vec!["sky".into()], envs: vec![], args: vec!["--rings".into()] };
        let both = suite.with(&a.settings);
        assert_eq!(both.off, ["sky", "precombine", "npcs"]);
        assert_eq!(both.args, ["--rings", "--sun-cascades", "2:205"]);
    }

    #[test]
    fn dlss_request_and_timing() {
        let env = |v: &str| vec![("BEVYRROWIND_DLSS".to_owned(), v.to_owned())];
        assert_eq!(dlss_requested(&env("quality"), None, &[]).as_deref(), Some("quality"));
        assert_eq!(dlss_requested(&env("off"), None, &[]), None);
        assert_eq!(dlss_requested(&env("TAA"), None, &[]), None);
        assert_eq!(dlss_requested(&[], Some("performance"), &[]).as_deref(), Some("performance"));
        assert_eq!(dlss_requested(&[], None, &[]), None);
        // The run's env wins over the inherited one; a `--dlss` flag over both.
        assert_eq!(dlss_requested(&env("off"), Some("quality"), &[]), None);
        assert_eq!(dlss_requested(&env("off"), None, &["--dlss".into(), "quality".into()]).as_deref(), Some("quality"));
        assert_eq!(dlss_requested(&env("quality"), None, &["--dlss=taa".into()]), None);
        assert!(dlss_timed(["render/main_opaque_pass_3d/elapsed_gpu", "render/dlss_super_resolution/elapsed_gpu"]));
        assert!(!dlss_timed(["render/main_opaque_pass_3d/elapsed_gpu", "render/dlss_super_resolution/elapsed_cpu"]));
    }

    struct FakeProbe {
        dirty: usize,
        battery: Option<u32>,
    }

    impl Probe for FakeProbe {
        fn now_unix_s(&self) -> f64 {
            1_700_000_000.0
        }
        fn git_head(&self) -> Option<String> {
            Some("985a22af0000".into())
        }
        fn git_changed_files(&self) -> Option<usize> {
            Some(self.dirty)
        }
        fn sha256(&self, _: &Path) -> Option<String> {
            Some("ab".repeat(32))
        }
        fn mtime_unix_s(&self, _: &Path) -> Option<f64> {
            Some(951_868_800.0)
        }
        fn power_scheme(&self) -> Option<String> {
            Some("Power Scheme GUID: 381b4222-f694-41f0-9685-ff5bb260df2e  (Balanced)\r\n".into())
        }
        fn battery_status(&self) -> Option<u32> {
            self.battery
        }
        fn power_overlay(&self) -> Option<String> {
            Some("    ActiveOverlayAcPowerScheme    REG_SZ    ded574b5-45a0-4f42-8737-46345c09c238\r\n    ActiveOverlayDcPowerScheme    REG_SZ    00000000-0000-0000-0000-000000000000\r\n".into())
        }
        fn bake_profile(&self) -> Option<(String, f64)> {
            None
        }
    }

    #[test]
    fn provenance_header_names_everything() {
        let exe = PathBuf::from("bench-ab.exe");
        let lines = provenance_lines(&exe, &FakeProbe { dirty: 0, battery: Some(2) });
        assert_eq!(
            lines,
            [
                "- date: 2023-11-14 22:13:20 UTC",
                "- commit: `985a22af0000` (clean)",
                &format!("- exe sha256: `{}` · modified 2000-03-01 00:00:00 UTC", "ab".repeat(32)),
                "- bake: ?",
                "- power: plan Balanced (381b4222-f694-41f0-9685-ff5bb260df2e) · mode best performance · AC",
            ]
        );
        let lines = provenance_lines(&exe, &FakeProbe { dirty: 42, battery: Some(1) });
        assert_eq!(lines[1], "- commit: `985a22af0000` (**dirty**: 42 tracked files changed)");
        assert!(lines[4].ends_with("· mode balanced · **battery**"), "battery reads the DC power mode");
        assert_eq!(power_source(None), "AC (no battery)");
        assert_eq!(utc(0.0), "1970-01-01 00:00:00 UTC");
        // certutil (Windows 10+, and the spaced form of older versions) and sha256sum.
        let hex = "0123456789abcdef".repeat(4);
        assert_eq!(parse_sha256(&format!("SHA256 hash of x.exe:\r\n{hex}\r\nCertUtil: -hashfile command completed successfully.\r\n")).as_deref(), Some(hex.as_str()));
        let spaced: String = hex.as_bytes().chunks(2).map(|c| std::str::from_utf8(c).unwrap()).collect::<Vec<_>>().join(" ");
        assert_eq!(parse_sha256(&format!("SHA256 hash of file x:\n{}\n", spaced.to_uppercase())).as_deref(), Some(hex.as_str()));
        assert_eq!(parse_sha256(&format!("{hex}  x.exe\n")).as_deref(), Some(hex.as_str()));
        assert_eq!(parse_sha256("CertUtil: error"), None);
    }

    fn run(arm_ms: f64, main_ms: f64, repeat: usize) -> SuiteRun {
        use sf_diag_bevy::sf_diag::{Counters, Recorder, Report, unit};
        let counters = Counters::new();
        let mut rec = Recorder::new();
        for _ in 0..10 {
            counters.set_named(MAIN_BUSY, unit::MS, main_ms);
            rec.commit_frame(arm_ms, &counters);
        }
        let report = Report::build("x", &rec, &[], 1.5, std::collections::BTreeMap::new());
        SuiteRun { scenario: "walk".into(), variant: "1280x720 arm=x".into(), repeat, report, gpu: None }
    }

    #[test]
    fn verdict_table_rows_per_arm() {
        let arms = parse_arms(&["base:".into(), "fast:--off=npcs".into(), "same:--arg=--scene".into()]).unwrap();
        let runs: Vec<SuiteRun> = (1..=4)
            .flat_map(|r| {
                #[allow(clippy::cast_precision_loss)]
                let j = r as f64 * 0.1;
                [run(10.0 + j, 9.0, r), run(9.0 + j, 8.0, r), run(10.05 + j, 9.0, r)]
            })
            .collect();
        let arm_runs: Vec<ArmRun<'_>> = runs.iter().enumerate().map(|(i, run)| ArmRun { arm: i % 3, run }).collect();
        let md = verdict_markdown(&arms, &arm_runs, &[("walk".into(), "1280x720".into())]);
        assert!(md.contains("| walk @ 1280x720 | `base` (base) | 4 | 10.25 | 10.10-10.40 |"), "{md}");
        // 4 vs 4 fully separated: U = 0, exact p = 2/70; main thread -1 ms.
        assert!(md.contains("| walk @ 1280x720 | `fast` | 4 | 9.25 | 9.10-9.40 | -1.00 | -1.00 | 0.0 | 0.029 | -1.00 | — | — | — | **faster** |"), "{md}");
        assert!(md.contains("| `same` | 4 | 10.30 | 10.15-10.45 | +0.05 | +0.05 |"), "{md}");
        assert!(md.lines().last().unwrap().ends_with("| inconclusive |"), "{md}");
    }

    #[test]
    fn save_refuses_a_folder_with_results() {
        let root = std::env::temp_dir().join(format!("bench-ab-save-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        assert!(check_save(None).is_ok());
        assert!(check_save(Some(&root)).is_ok(), "a new folder");
        std::fs::create_dir_all(&root).unwrap();
        assert!(check_save(Some(&root)).is_ok(), "an empty folder");
        std::fs::write(root.join("summary.md"), "x").unwrap();
        assert!(check_save(Some(&root)).unwrap_err().contains("already holds results"));
        std::fs::remove_dir_all(&root).unwrap();
    }
}
