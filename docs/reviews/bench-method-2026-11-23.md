# Bench method, 2026-11-23 (design note, nothing run)

`gc_b4` = 18 launches, 1243 s wall (run dir timestamps); a standing run is 35 s wall for 18 s of scenario, a walk
~80 s for 59 s. The headless test (5.5 -> 0.4 ms) and one run's gauge `grass/rebuild_time` (max 0.94 ms) had already
answered; `groundcover.blades.enabled` is a live run condition (`sf_groundcover_bevy/src/blades.rs:326`).

Industry: UE (Gauntlet + CSV profiler / PerfReportTool), Unity Performance Testing (in-process
`WarmupCount` / `MeasurementCount`, many frames, one process) and studio CI (one flythrough per build, budgets per
counter, alert on threshold, bisect) measure one run per build and trend across builds; attribution comes from
counters and captures (Insights), not from relaunching per arm.

## 1. Cheapest source per question

| question | source | runs x time |
|---|---|---|
| still spikes / hitches? | headless `#[ignore = "measurement"]` test, then the gauges of one normal run (gauge max, hitches/min) or the owner's F3 in play | 0-1 x ~1.5 min |
| what does live feature X cost? | in-process ABAB, one launch (§3) | 1 x ~1.5 min |
| what does a restart-only part cost? | `perf suite`, 1 run per arm, same exe | 2 x 35-80 s |
| should X be on by default? | the cost answer + owner look if visual; repeat only near the line (§2) | as above |
| regression since last commit? | one fixed standing view + one walk vs stored baseline (`--bench-baseline`, `mw_devtools_bevy/src/perf.rs:126`) | 2 x ~1 min |
| does it break rendering? | log lines (wgpu validation, shader errors) in a GPU test, then owner look; no bench | 0-1 |

## 2. Are repeats needed? Measured

Script over every `summary.md` in `bevyrrowind/target/perf/*` and `bevyrrowind/benchmarks/runs/*` ("Runs" table;
spread = max - min of run medians within one arm and scene; 405 groups of 2-5 runs, all 2026-11-17+):

| set | groups | median | p90 | p95 | > 1 ms |
|---|---:|---:|---:|---:|---:|
| quiet + focus clean | 91 | 0.20 ms (1.9 %) | 0.67 | 0.73 | 3 |
| all | 405 | 0.20 ms (2.2 %) | 0.77 | 1.43 | 28 |

Runs in an arm do read the same value (`gc_b4`: 0.06-0.23 ms per arm). Every large spread is a discrete fault, not
noise: GC-P0 bistable GPU state (11.5 vs 36.7 ms), `lampsel_new` 10.3 vs 106.7, contaminated suites. A 3x jump shows
in one run: a bug to log, not a reason to average. One paired run in `gc_b4` gave +0.08 to +0.53 ms for a true
~+0.27: one launch per arm settles >= 0.5 ms, not a 0.25 ms cost.

**Rule:** one run per arm. Run a second launch only if the harness flags the run (§4) or the delta is within 0.5 ms
of a decision (default on/off, budget). Costs under 0.5 ms are measured in-process (§3) or from GPU pass timers, never
with more launches. The dirt the 11-16 audit found (blocked arms, unfocused window ~65 vs ~85 fps, other sessions)
is caught by interleaving and hygiene, not repeats.

## 3. In-process A/B (the default A/B tool)

Shape: one launch, scenario warm-up, then the switch flips every N s (A B A B ... >= 4 phases each, 5 s), first
cycle discarded (pipeline compiles on the first flip), per-phase median, p99, hitches, main / render / GPU and GPU
passes; Δ = median of paired (B - A) phases. ABAB cancels thermal drift. ~20 s load + 3 s warm-up + 45 s = ~70 s.

What exists: the switch probe (`mw_devtools_bevy/src/switch_probe.rs`) measures every flip, mean frame / main /
render / GPU over 2 s before vs 2 s after a 0.5 s settle (`WINDOW_S` :39, `SETTLE_S` :46, `NOISE_MS` 0.5 :53), flags a
camera move (`STILL_M` :60); quick A/B = 3 off/on rounds, median cost (`AB_ROUNDS` :74, `start_quick_ab` :319).
Missing: (a) needs the Features panel open and a click (:275), no CLI or scenario entry; (b) feature switches only,
not any `Lifetime::Live` tunable; (c) 2 s means, no p99 / hitches / GPU passes; (d) result only in an `info!` line,
no report file; (e) standing camera only (a walk needs one path lap per phase).

Not live (refused at `switch_probe.rs:337`: `startup_hint`, `Lifetime::Restart | Bake`): `Kind::Removed` parts
(`part_switches.rs:41-81`: groundcover, underwater, farshadows, shadows, gi) and every `part.*` startup row. For these:
`perf suite` 1 run per arm, interleaved, same exe; and make the costly ones live (built, systems skipped by a run
condition, as froxels or blades already are) one slice each.

## 4. Hygiene instead of repeats

Already checked per run (`perf/cli.rs:558`, retry on dirty `:371`): no build / game process, other CPU <= 25 %, GPU
idle before, window focused, power plan and mode recorded (`perf/power.rs`). Add, each refuse (R) or flag (F):
- R: on battery or power mode not best performance.
- F -> one rerun: GPU clock or W drops > 10 % from the run's first 2 s; pipeline compiles in the measure window
  (a warm-up gate, so one warm-up launch per new exe, not per arm); frame > 1.5x the scene's stored baseline
  (catches the bistable fault without peers; today's outlier check needs repeats, `cli.rs:599,669`).

## 5. CONTRIBUTING, proposed (not applied)

```diff
-- **Design a bench session before running it (owner 2026-11-17).** ... (7 lines)
-- **Perf work starts from a measured ceiling and an expected ms**, ... (3+ runs each) ... (11 lines)
+- **Measure the cheapest way (owner 2026-11-23, `docs/reviews/bench-method-2026-11-23.md`).** Perf work starts
+  from a measured ceiling and an expected ms. Hitches: a headless test or one run's gauges. A live switch:
+  `perf ab` (ABAB in one launch). Restart-only parts and regressions: `perf suite`, one run per arm, same exe;
+  switches that touch the same work also as one combined arm. Runs agree within ~0.2 ms, so no repeats: a second
+  run only when the harness flags one or the delta is within 0.5 ms of a decision. Report median, p99,
+  hitches/min and main, render and GPU ms (a cut off the bound thread is "banked"); under ~0.5 ms is noise.
+  Over ~10 min of bench needs the owner's go. Log the result in `docs/ledger/perf.md`.
+- **Verdicts (owner 2026-11-15/16/17).** No win is "inconclusive": it never deletes, freezes or disables code;
+  the change stays behind an opt-in switch, its ledger line says what is left. A win with no look awaiting the
+  owner becomes the default, its off value the A/B arm. Dead code something else replaced is deleted, tagged.
```

## 6. Tooling slices (one each)

1. **`perf ab`**: scenario field `ab: {tunable, phase_s, phases}` (`sf_diag` `scenario.rs:32`) drives the probe for
   any live tunable, no panel; fixes (a)-(d); writes `ab.md`. Live cost: 21 min -> ~1.5 min.
2. **`perf suite` defaults**: `--repeat 1`, one warm-up launch per exe, not per arm; `--repeat` stays opt-in.
   Restart-only cost: 21 min -> ~2-3 min.
3. **Fault and thermal flags without peers** (§4): baseline ratio, clock / W drop, pending compiles, AC refuse.
4. **`perf check`**: standing + walk vs stored baselines, exit non-zero on > 0.5 ms or hitches/min up; ~2.5 min,
   per commit that touches rendering or streaming.
5. **Gauge summary first in every report** (gauges over their budget): "still hitches?" ~36 min -> ~1.5 min or 0.
6. **Live switches for the removed parts** (§3), one slice per part.
