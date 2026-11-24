//! `bevyrrowind perf ab`: an in-process A/B in ONE launch of a named standing scenario
//! (`docs/reviews/bench-method-2026-11-23.md` §3): the game's `--ab NAME=A,B` flips one live
//! tunable A B A B ... every phase (`mw_devtools_bevy::perf::flip_ab`) and writes `ab.md` / `ab.csv`
//! beside its report (`sf_diag::flip_ab`). The launch goes through the suite's runner (bench lock,
//! quiet-machine wait, busy retries, provenance, summary); `ab.md` is copied to the tag folder and
//! printed.

use std::path::Path;

use sf_diag_bevy::sf_diag::flip_ab::AbSpec;
use sf_diag_bevy::sf_diag::{Measure, Scenario};

use super::cli::{Planned, SCENARIOS, SuiteArgs, SuiteScenario, busy_rules, execute, run_name, start_chain, variant_name, write_summary};
use super::sweep::Step;

#[derive(clap::Args)]
pub struct AbArgs {
    /// The tag, one standing scenario (`perf list`) and the suite's run settings (`--off`, `--set`,
    /// `--env`, `--arg`, `--resolution`, `--exe`, ...). One launch: no `--repeat`, no `--arm`.
    #[command(flatten)]
    suite: SuiteArgs,
    /// The tunable and its two values, `NAME=A,B` (`NAME=A|B` when a value holds commas), A first.
    /// Live tunables only (`bevyrrowind tunables --md` lists them with their lifetime).
    #[arg(long, value_name = "NAME=A,B")]
    ab: String,
    /// Seconds per phase (default `bench.ab.phase_s`).
    #[arg(long)]
    ab_phase_s: Option<f64>,
    /// Measured phases per value; one more A+B cycle runs first and is discarded (default `bench.ab.phases`).
    #[arg(long)]
    ab_phases: Option<u32>,
    /// Seconds left out at the start of each phase (default `bench.ab.settle_s`).
    #[arg(long)]
    ab_settle_s: Option<f64>,
}

/// The game flags of the A/B: `--ab NAME=A,B` and the phase flags that are set.
pub fn game_args(ab: &str, phase_len_s: Option<f64>, per_value: Option<u32>, settle_s: Option<f64>) -> Vec<String> {
    let mut out = vec!["--ab".to_owned(), ab.to_owned()];
    let mut flag = |name: &str, v: Option<String>| {
        if let Some(v) = v {
            out.extend([name.to_owned(), v]);
        }
    };
    flag("--ab-phase-s", phase_len_s.map(|v| v.to_string()));
    flag("--ab-phases", per_value.map(|v| v.to_string()));
    flag("--ab-settle-s", settle_s.map(|v| v.to_string()));
    out
}

/// The one scenario of a `perf ab`, refused when it is not a standing one or when the arguments ask
/// for more than one launch.
fn pick(suite: &SuiteArgs) -> Result<&'static SuiteScenario, String> {
    if suite.repeat != 1 || !suite.arms.is_empty() {
        return Err("perf ab is one launch: --repeat and --arm belong to `perf suite`".to_owned());
    }
    let [name] = suite.scenarios.as_slice() else { return Err("perf ab takes exactly one scenario".to_owned()) };
    SCENARIOS.iter().find(|s| s.name == name).ok_or_else(|| format!("unknown scenario {name:?}; see `perf list`"))
}

/// Refuses a path scenario before the launch (the game refuses it too).
fn check_standing(scenario: &SuiteScenario, json: &str) -> Result<(), String> {
    let parsed = Scenario::from_json(json).map_err(|e| format!("{}: {e}", scenario.name))?;
    if matches!(parsed.measure, Measure::Path { .. }) {
        return Err(format!(
            "perf ab: `{}` is a path scenario; path scenarios are refused for now (a phase would have to span whole laps): use a standing one",
            scenario.name
        ));
    }
    Ok(())
}

pub fn run(args: &AbArgs) -> Result<(), Box<dyn std::error::Error>> {
    let suite = &args.suite;
    let exe = match &suite.exe {
        Some(p) => p.clone(),
        None => std::env::current_exe()?,
    };
    if !Path::new("benchmarks").is_dir() {
        return Err("run from the bevyrrowind repo root (scenario paths are relative to it)".into());
    }
    super::ab::check_save(suite.save.as_deref())?;
    let scenario = pick(suite)?;
    AbSpec::parse(&args.ab)?;
    let bench = scenario.args.iter().position(|a| *a == "--bench").and_then(|i| scenario.args.get(i + 1)).ok_or("scenario has no --bench")?;
    check_standing(scenario, &std::fs::read_to_string(bench)?)?;
    suite.run.check_sets()?;

    let resolution = suite.resolutions.first().cloned().unwrap_or_else(|| "1280x720".to_owned());
    let step = Step { label: "ab".to_owned(), args: game_args(&args.ab, args.ab_phase_s, args.ab_phases, args.ab_settle_s), ..Step::default() };
    let variant = variant_name(&resolution, &suite.run.off, &suite.run.game_args(), Some(&step.label));
    let plan = vec![Planned { scenario, resolution, repeat: 1, step: Some(step), arm: None, warmup: false }];
    let dir = suite.out.join(&suite.tag);
    std::fs::create_dir_all(&dir)?;
    let busy = busy_rules(suite);
    println!("perf ab {} on {}: one launch → {}", args.ab, scenario.name, dir.display());
    let chain = start_chain(suite, &exe)?;
    let (runs, mut ledger) = execute(suite, &exe, &dir, &plan, &busy);
    ledger.chain.clone_from(&chain.notes);
    if suite.dry_run || runs.is_empty() {
        return Ok(());
    }
    let suite_runs: Vec<_> = runs.into_iter().map(|(_, r)| r).collect();
    write_summary(suite, &exe, &dir, &suite_runs, &ledger, &busy, None)?;
    let run_dir = dir.join(run_name(scenario.name, &variant, 1));
    let md = std::fs::read_to_string(run_dir.join("ab.md")).map_err(|e| format!("no ab.md in {}: {e} (see its log.txt)", run_dir.display()))?;
    std::fs::write(dir.join("ab.md"), &md)?;
    if let Ok(csv) = std::fs::read_to_string(run_dir.join("ab.csv")) {
        std::fs::write(dir.join("ab.csv"), &csv)?;
        if let Some(save) = &suite.save {
            std::fs::write(save.join("ab.csv"), &csv)?;
        }
    }
    if let Some(save) = &suite.save {
        std::fs::write(save.join("ab.md"), &md)?;
    }
    print!("\n{md}");
    println!("wrote {}", dir.join("ab.md").display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;

    #[derive(Parser)]
    struct Cli {
        #[command(flatten)]
        ab: AbArgs,
    }

    #[test]
    fn perf_ab_takes_one_standing_scenario_and_passes_the_ab_flags() {
        let cli = Cli::try_parse_from(["ab", "t1", "gc_ascadian", "--ab", "groundcover.blades.enabled=false,true", "--ab-phases", "3"]).unwrap();
        assert_eq!(pick(&cli.ab.suite).unwrap().name, "gc_ascadian");
        assert_eq!(
            game_args(&cli.ab.ab, cli.ab.ab_phase_s, cli.ab.ab_phases, cli.ab.ab_settle_s),
            ["--ab", "groundcover.blades.enabled=false,true", "--ab-phases", "3"]
        );
        assert_eq!(game_args("x=0,1", Some(2.5), None, Some(0.25)), ["--ab", "x=0,1", "--ab-phase-s", "2.5", "--ab-settle-s", "0.25"]);
        assert!(Cli::try_parse_from(["ab", "t1", "gc_ascadian"]).is_err(), "--ab is required");
        let two = Cli::try_parse_from(["ab", "t1", "gc_ascadian", "balmora", "--ab", "x=0,1"]).unwrap();
        assert!(pick(&two.ab.suite).is_err());
        let repeat = Cli::try_parse_from(["ab", "t1", "gc_ascadian", "--ab", "x=0,1", "--repeat", "3"]).unwrap();
        assert!(pick(&repeat.ab.suite).err().is_some_and(|e| e.contains("one launch")));
    }

    #[test]
    fn path_scenarios_are_refused_before_the_launch() {
        let walk = SCENARIOS.iter().find(|s| s.name == "gc_walk").unwrap();
        let path = r#"{"name":"w","measure":{"kind":"path","speed":7,"keyframes":[{"position":[0,0,0]},{"position":[1,0,0]}]}}"#;
        assert!(check_standing(walk, path).unwrap_err().contains("path scenario"));
        let standing = r#"{"name":"s","measure":{"kind":"duration","seconds":15}}"#;
        assert!(check_standing(walk, standing).is_ok());
    }
}
