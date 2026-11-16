# Perf verdict audit, 2026-11-16 (adversarial, read-only)

Scope: every performance verdict that changed what runs by default (kept, opt-in, reverted, deleted, frozen), checked
against its own evidence in `bevyrrowind/benchmarks/runs/<tag>/` (summary.md, per-run md/json incl. `wall_start_unix_s`),
the ledgers, DECISIONS and the ROADMAP. No builds, no runs. Method: per verdict, same exe? adjacent or hours apart?
blocked or interleaved? runs vs spread? focus? concurrent benches (overlap of measured windows from the run JSONs)?
DLSS pass present? config = shipped default?

## 1. What the evidence shows (cross-cutting)

1. **Every A/B before 2026-11-16 is blocked, never interleaved.** `run_suite` builds the plan scenario -> resolution ->
   repeat (`apps/bevyrrowind/src/perf/cli.rs:353-359`) and one invocation = one arm, so arms are always AAA then BBB.
   Anything that shifts between blocks (focus, another session, thermal, driver cache) becomes an arm effect with a
   tight within-arm spread, i.e. it looks like a clean result. The ledger itself records the block-to-block drift:
   "the same old path read 10.71, 10.95 and 11.72 ms on the walk" (`docs/ledger/perf.md:76`), ~0.5 ms, larger than
   most of the effects being judged.
2. **Focus was uncontrolled before 11-16 and is invisible in blocked data.** All 3 runs of an arm launch while the
   owner has the same thing in front, so a focus penalty (owner: 65 vs 85 fps = ~3.6 ms) shifts a whole arm.
   Only the `f_*` chain (11-16 00:09-01:12) records `focus: clean`.
3. **Concurrent benches got through the quiet check.** `busy_among` matches exact base names
   (`superfargoth/crates/sf_diag/src/suite.rs:67-99`) against `cargo/rustc/bevyrrowind/ovid` (`cli.rs:83`); renamed
   exes (`orch-*.exe`, `h1gpucull.exe`, `tex-slab-*.exe`, `lampsel_*.exe`) are invisible to it, GPU load is checked
   only before a run (`cli.rs:478-512`) and "busy during" is only other-process CPU > 25 % (`cli.rs:617-619`). From the
   run JSONs, measured windows overlapped on 11-15: `h1_old_nomaps`/`h1_new_nomaps` with `slab_off_contaminated`
   (15:44-15:50), `lampsel_old` with `slab2_on` (16:14), `slab2_off` with `lampsel_new` (16:35); `h1_old` was paused 23
   min while another session's `tc_pbr*` ran (14:53-15:16). Yet all 63 summaries say "machine: quiet", because that line
   is printed whenever no busy note exists (`cli.rs:439-440`).
4. **Garbage runs enter the medians.** `execute` keeps the last retry even if it is still dirty (`cli.rs:384-409`), and
   there is no outlier guard: runs with medians 51.7 (`slab2_off` standing #3), 67.9 (`h1_old_nomaps` #2), 106.7 and
   35.3 (`lampsel_*` night), 79.7 (`slab_off_contaminated`) sit inside "median of 3", which then reports the worse of
   the two good runs.
5. **No provenance.** Summaries carry no date, commit, exe hash or bake id. Ten suites used the mutable
   `target\profiling\bevyrrowind.exe`, which any session's profiling build overwrites (the lean ladder spans 90 min on
   it). The tag folder is overwritten on reuse (`cli.rs:454-466`): the 11-11 `scene_off` control was replaced on 11-15,
   so the first static-scene verdict has lost its control arm.
6. **DLSS silently absent.** `tc_pbr`/`tc_pbrwater` asked for `BEVYRROWIND_DLSS=quality` but have no
   `dlss_super_resolution` pass (exe in a scratchpad without `dlss/`): standing 15.5 ms vs ~10.5 elsewhere.
7. **Frame-only verdicts while bound elsewhere (max() masking).** The frame is ~ max(main, render, GPU). Changes that
   cut the main thread are judged "parity" while render-bound: precombine (main -1.3, frame unchanged, 10-20), the
   scene (11-16 main/busy 8.77-8.88 vs 9.40-9.53), H1 (11-16 main/busy 8.66-8.85 vs 9.40-9.53). The summaries don't
   even show main/busy or render/busy (gauge list `cli.rs:414-431`). The same flaw produced "statics ~1.1 ms of the
   walk" (near ring 3.1 minus NPCs 2.0): subtracting `--off` arms of a max() frame is not additive.
8. **Re-checks were run on a non-shipped base.** `f_on` = F1 encoders + 2 cascades (+ hold, slab cap); `f_3casc` sets
   `BEVYRROWIND_SUN_CASCADES=3:205`. After the chain, the default became 3 cascades + hold, F1 off, a combination that
   was never an arm. `f_scene`, `f_h1`, `f_thr12` are all "vs `f_on`", and H1's 11-15 cost was cascade batch sets.
9. **Power: n=3 detects ~0.6 ms at best.** Within-arm SD of walk run medians is ~0.05-0.3 ms clean, up to 0.9 dirty
   (`lean_base` 13.05-14.87). To detect 0.3 ms (alpha .05, power .8, sd 0.25) needs ~11 runs per arm interleaved; 0.5 ms
   needs ~4. The "< 0.5 ms = noise" rule plus n=3 means a 0.3 ms effect (exactly the main-thread size of the static
   work) has maybe a 30 % chance to be seen, and "not seen" was read as "not there".

## 2. Verdict table

Classes: SOLID = same exe, clean, effect >> spread or deterministic counter/visual reason; SUSPECT = named confound;
UNSUPPORTED = no clean data (or evidence lost).

| verdict | date | source line | decision | test conditions found | confound(s) | class | re-test? |
|---|---|---|---|---|---|---|---|
| F3 far-cascade hold | 11-16 | ledger/perf.md:9 | on | same exe `orch-f123`, focus clean, blocked x3; f_off 12.25-12.63 vs f_on 11.34-11.87 walk; nohold standing 10.60-10.67 vs 10.05-10.32 | blocked; base not shipped | SOLID | no |
| F2 2 cascades | 11-16 | perf.md:9 | opt-in, presets 3 | f_3casc 11.43-11.66 / 9.96-10.06 = f_on | shipped combo (3c+hold, F1 off) never measured | SOLID | confirm shipped (chain 2) |
| F1 encoder per cascade | 11-16 | perf.md:9 | opt-in env | f_noenc 11.21-11.28 vs 11.34-11.87 | measured only with hold + 2 cascades (cascades rarely re-encode) | SUSPECT | yes, chain 2 |
| F4 slab cap | 11-16 | perf.md:9 | on (guard) | f_noslab = f_on | none relevant | SOLID | no |
| Static scene re-check: "parity, under noise" | 11-16 | perf.md:10 | stays frozen [p] 1.16 | clean chain; scene best of 9 arms: walk 11.01-11.08 vs 11.21-12.63, p99 15.1-15.5 vs 15.3-20.7, main/busy 8.8 vs 9.4, standing 9.96-10.01 | verdict understated; 2-cascade base | SUSPECT (verdict, not data) | yes, chain 1 |
| H1 re-check: parity, candidate | 11-16 | perf.md:10 | opt-in, behind F8 | clean; walk 11.05-11.44, main/busy -0.7 ms, render equal | 2-cascade base (H1's cost was cascade batch sets) | SUSPECT | yes, chain 1 |
| 12 compute threads | 11-15/16 | perf.md:18, :10 | opt-in | two tests agree (12.59 vs 12.27 blocked; 11.35 vs 11.42 clean) | - | SOLID | no |
| Pipelining off | 11-15 | perf.md:18 | on | 1 run, 20.48 vs 12.27 | n=1, effect huge | SOLID | no |
| H1 "not a win" | 11-15 | perf.md:24, statics.md:8 | opt-in | same exe, blocked; walk 15.17-16.32 vs 12.97-13.92; render/graph +2.4 (bsets 182 vs 153) | `h1_old` interrupted by `tc_pbr*`; nomaps pair overlapped `slab_off_contaminated` (+67.9 ms run in median); focus unknown; pre-hold | SUSPECT (superseded) | chain 1 |
| B2b flat normal | 11-15 | perf.md:20 | on | same exe `--off flatnormal`, blocked; off walk spread 1.21 ms | ms inside spread, focus unknown | SUSPECT (keep safe: bsets 92->87) | no |
| B2a tangents | 11-15 | textures.md:5 | on | cross-exe + re-bake vs `dkeys_on` | different build and bake | UNSUPPORTED ms (kept for TX-6) | no |
| Depth-only keys | 11-15 | perf.md:22 | on | 2 exe copies, adjacent; 13.15-13.59 vs 12.46-12.68 | build layout, focus unknown | SOLID | no |
| Bindless slab 8192 | 11-15 | perf.md:23 | on | 2 alternating rounds; -1.06 / -0.70 | round 2 overlapped `lampsel_*` (486-hitch run, 51.7 ms run in medians) | SOLID (direction) | no |
| Lamp select SH-21b cost +0.34/+0.69 | 11-15 | shadows.md:5 | kept (correctness) | 2 exes, blocked | overlapped `slab2_*`; night 35.3 / 106.7 ms runs in medians; other CPU 24 % | SUSPECT (cost figure) | yes, chain 2 if `lampsel_old.exe` kept |
| Texture cap 512 "no ms" | 11-15 | perf.md:27 | not used for perf | mutable exe, 90-min blocked ladder | base walk spread 1.8 ms | UNSUPPORTED | chain 2 |
| PBR maps cost 2.7 ms | 11-15 | perf.md:27 | drove fixes A/B1/B2 | same ladder | large; confirmed by later A/Bs | SOLID | no |
| Cumulative ladder "walk flat, CPU bound" | 11-15 | perf.md:28 | informs 1080p rejection, F-plan | blocked 70 min, mutable exe | drift, focus | SUSPECT | no (informational) |
| DLSS Performance no help | 11-15 | perf.md:29 | presets stay DLSS Q | 80 min after base | drift; base spread 1.8 | SUSPECT | chain 2 |
| Maps-off / water attribution | 11-15 | perf.md:26 | feeds F7 (half-rate SSPR) | scratchpad exe | **no DLSS pass**; interleaved with `h1_old` | UNSUPPORTED (at High+DLSS) | chain 2 optional |
| Static scene "parity, delete as queued (D)"; "11 k entities = 0.3 ms" | 11-15 | statics.md:7 | delete queued, F6 estimate cut to 0.3-0.5 | same exe `orch-b4`, adjacent; walk 11.43-11.84 vs 11.47-11.73 | focus unknown, pre-F3, frame-only (main -0.2..-0.3 masked by render +0.3), power ~30 % | SUSPECT | **yes, chain 1** |
| Owner freeze of the static scene | 11-12 | statics.md:10, ROADMAP:77 | frozen [p] | "parity" = `fix4_on` 10:50 vs `fix_off` 00:42 (10 h, other build, mutable exe) | cross-build; "statics ~1.1 ms" by additive subtraction of a max() frame | SUSPECT | chain 1 |
| Scene fix 4 "no gain" (S3a3 parked) | 11-12 | statics.md:13 | WIP parked | vs `fix_on` 10 h earlier, other build | cross-build | UNSUPPORTED | optional |
| Scene fixes 1-3+5, parity vs old | 11-12 | statics.md:15 | kept, opt-in | adjacent 6 min, same exe path; walk +0.39 | focus unknown, blocked | SUSPECT | chain 1 |
| First scene A/B: +2.78 walk | 11-11 | statics.md:17 | old path default | control `scene_off` overwritten 11-15; scene arm coolest (65 °C), lowest power (63.5 W), other CPU 10-15 % | evidence lost; possible throttled run | UNSUPPORTED (magnitude; traced audit did find real costs, since fixed) | no (superseded) |
| Ceiling "statics 3.7 ms" / "~1.1 ms" | 11-12 | perf.md:31, :39 | premise of the freeze | `--off statics` also drops NPCs + ~290 lamps | non-additive decomposition | SUSPECT (method) | chain 1 with per-thread gauges |
| P3 `NoCpuCulling` opt-in | 11-12 | perf.md:33 | opt-in | same exe, adjacent; ms spreads overlap | stutters 113 vs 14/min mechanistic (post-mortem) | SOLID (stutters) | no |
| P4-P6, end-of-day -1.0 | 11-12 | perf.md:30, :34-36 | on | cross-build, 2-3.5 h apart | counters deterministic (bind groups 186->10) | SOLID (keep); ms SUSPECT | no |
| P1 kept with regression | 11-12 | perf.md:37 | on | GPU pass timer 0.36->0.93 | - | SOLID | no |
| GS port always-on "costs nothing" | 11-10 | statics.md:23, perf.md:44 | on | other build + own bake; `ab_before` walk drifts 10.78->12.28 inside one arm | cross-build, drift | SUSPECT (low stakes) | no |
| Size-class slot allocator | 11-10 | statics.md:21 | reverted | synthetic, deterministic | - | SOLID | no |
| Legacy static/mover lamp cache off (`POINT_SHADOW_CACHE = false`, reset B3 `b3bcd987`) | 10-21 | `bevyrrowind/crates/mw_environment_bevy/src/render.rs:37` | off, code kept | **no ledger line, no A/B**; B1 had measured cache+patch best (lab 10.60 vs patch 20.11, census 6.54 vs 15.03) | - | UNSUPPORTED | yes (needs a switch slice) |
| Far static sun map deleted | 10-21 | shadows.md:22 | deleted | never measured ("built off, never adopted") | - | UNSUPPORTED (F3 hold now covers far caching) | no |
| Precombine on, "frame unchanged" | 10-20 | statics.md:25, DECISIONS 2026-11-15 | on | pre-DLSS, pre-PBR; main -1.3 masked | stale | SUSPECT (stale) | chain 1 arm |
| `--prepass` slower (13.9 vs 11.9) | 10-12 | bevyrrowind/docs/perf.md:349 | opt-in | native, pre-DLSS | stale; DLSS adds a prepass anyway | SUSPECT (stale) | optional |
| Caster size cull, anim budget, prepass+crossfade, NoCpuCulling 81 ms | 10-12..10-19 | statics.md:24, :27, :32; perf.md:55-57 | reverted | visual breakage or 7x frame | - | SOLID | no |
| Shadow reset B1/B2, stale light views, terrain blend, far field d1 400, Phase 1 pass | 10-12..10-28 | perf.md:48-53, statics.md:30-31 | on | effects 1-11 ms or GPU timers | - | SOLID | no |
| W2, W4b1, W4b1-throughput | 11-15/16 | perf.md:8, :12, :15 | on | private exe copies, empty vs warm GLCache, counters `after_reveal` | warm runs noisy (ledger says so) | SOLID | no |
| Ultra layer budget | 11-15 | perf.md:19 | on | OOM 3/3 -> 0/3 | - | SOLID | no |

Tally (37 rows, the last grouped rows count once): **SOLID 15, SUSPECT 15, UNSUPPORTED 7.**

## 3. The three decisions most likely wrong

1. **Static scene "parity -> frozen, delete queued".** On the only clean data (11-16), `--scene` is the best of 9 arms
   on the walk median (11.01-11.08), p99 (15.1-15.5) and main thread (-0.6 ms), and the best or tied standing. Every
   earlier "parity" was blocked, focus-unknown, and two were cross-build/10 h apart; the freeze premise ("statics
   worth ~1.1 ms") came from subtracting non-additive `--off` arms. It wins while still carrying known handicaps: no
   LOD or size cull, work items 154 k vs 11 k (dispatch width = capacity x views, S3a3 compaction parked). Its
   pacing win (11-11: 28 -> 0-8 stutters/min; 11-12: 14.6-15.5 -> 5.5-12.8) was never refuted. Do not delete before
   chain 1.
2. **H1 GPU-culled statics "not a win", parked behind F8.** The 11-15 loss looks real for the pre-hold config
   (render/graph +2.4 ms, +29 batch sets) but was measured beside other sessions' benches. On 11-16 it is at frame
   parity while cutting main/busy ~0.7 ms, i.e. it already is part of the F6 main-thread win the roadmap plans to build.
   It was measured on 2 cascades; the shipped 3-cascade + hold config is untested.
3. **Legacy static/mover lamp cache switched off in reset B3 without an A/B.** B1's 4-way favoured cache + patch;
   B3 (same day) turned the cache off for the vendored per-face cache, with no ledger line. SH-21a (11-15) now names
   "static casters cached, movers drawn on top" as the standard fix (400-600 lines) for the torch flicker: that is the
   design of the code that was switched off and is still in the tree.

Runner-up: the "11 k fewer entities = ~0.3 ms" inference (statics.md:7) that cut the F6 estimate from 1.5-2.5 to
0.3-0.5 ms: it came from one confounded A/B whose scene arm added batch sets at the same time.

## 4. What a clean A/B looks like here (industry practice)

Standard practice in perf regression testing (Chromium Pinpoint, Mozilla Perfherder, RMIT of Abedi & Brecht ICPE 2017,
Georges et al. OOPSLA 2007; measurement-bias work of Mytkowicz et al. ASPLOS 2009; UE Gauntlet/CSV profiler on
dedicated machines; Unity Performance Testing warm-up + measurement counts):

- Same exe, arms as runtime switches; if two builds are unavoidable, interleave them and expect +-1-3 % layout bias.
- Arms interleaved (ABBA or randomised blocks) in one invocation, never AAA/BBB; a closing control arm.
- One discarded warm-up run per arm per chain (driver cache, file cache, first launch); thermal soak to a stable
  GPU temperature before the first kept run.
- Fixed power plan and AC power recorded; process power throttling off; window focused for every measured frame.
- Quiet machine enforced by a lock all benches share (not by process names), GPU checked during the run.
- Enough repeats for the effect size (sd ~0.25 ms: ~4 per arm for 0.5 ms, ~11 for 0.3 ms); report spread and a rank
  test or bootstrap CI of the median difference; "inconclusive" is a verdict, "no win" needs power.
- Per-thread times (UE `stat unit`: Game / Draw / GPU) beside the frame time, so a bottleneck elsewhere can't hide a win.
- NVIDIA shader cache controlled: cap raised (NVCP Shader Cache Size 10 GB or unlimited) or a cold/warm protocol
  (W2/W4 already do this right for load time).
- Provenance in every summary: date, commit + dirty flag, exe hash, bake id, DLSS actually active.

## 5. Bench-tool fixes, ranked by dirt removed

1. **Interleaved arms in one invocation** (`cli.rs:335-376`, plan at :353-359): `--arm name:KEY=V,--off=x,--arg=y`
   (repeatable), plan = per repeat ABBA/BAAB order across arms, same exe, plus one discarded warm-up run per arm;
   summary per arm with paired deltas. Removes drift, focus blocks, other-session blocks and thermal trend at once.
2. **Cross-session bench lock + real busy detection** (`cli.rs:83`, `wait_quiet` :478-512, `sf_diag/src/suite.rs:81-99`):
   a lock file held for the whole chain (like `cargo_turn.sh`), busy = any process on the GPU (`nvidia-smi pmon`) or
   any exe under `target\` / `%TEMP%`, and GPU contention checked during the run from the `gpu.csv` samples already
   taken (:584-626).
3. **Dirty or outlier runs never enter medians** (`execute` :384-409): after the last retry, drop the run and write
   "N of M valid"; retry when a run's median is > 1.3x the arm's other runs or measured hitches explode (486/174).
   Print "machine: quiet" only when every run was checked clean (:439-440).
4. **Provenance header** (`write_summary` :433-447): date, `git rev-parse HEAD` + dirty, exe SHA-256 + mtime, bake
   stamp, power plan (`powercfg /getactivescheme`), AC/battery, CPU effective clock; refuse `--save` into an existing
   folder (:454); fail a run when `BEVYRROWIND_DLSS` is set but no `dlss_super_resolution` pass was timed.
5. **Per-thread columns in summary.md** (gauges :414-431): add `main/busy/ms`, `render/busy/ms`, `render/graph/ms`,
   `gpu/timed/ms` means, and a verdict line (median delta, min-max per arm, Mann-Whitney U, "inconclusive" under
   power).
6. **Power throttling and focus**: `unsafe_code = forbid` blocks `SetProcessInformation`; the runner can instead set
   the plan for the chain (`powercfg /setactive` to High performance and restore) or run
   `powercfg /powerthrottling disable /path <exe>` (check whether it needs admin), and wait for "owner not busy" the
   way `bevyrrowind/tools/owner_launch.sh` does.
7. Shader cache: raise the NVCP cap on this machine once; log the GLCache folder size before each run.

## 6. Re-test plan (when the PC is free, ideally at night)

Exe: one private copy of current HEAD named `bevyrrowind.exe` in its own folder (so other runners see it), High,
2560x1440, `BEVYRROWIND_DLSS=quality`, shipped defaults (3 cascades + hold, F1 off). Until fix 1 exists, emulate it
by hand: `perf suite <tag>_<arm>_<k> ... --repeat 1 --exe <copy>` called in ABBA order, one throwaway run per arm
first, per-thread means read from the run md files.

**Chain 1, statics (decides delete vs continue, H1, precombine):** arms base / `--arg=--scene` /
`--env BEVYRROWIND_GPU_CULL_STATICS=1` / `--off precombine`; `balmora_walk` + `balmora`, 6 kept repeats per arm,
ABBA. Plus `balmora_walk_night` for base vs H1 (lamp faces). Optional arm: legacy lamp cache, after a one-line
switch slice exposing `POINT_SHADOW_CACHE` as an env opt-in. Time: 4 arms x 6 x ~2.1 min = 50 min, night walk
2 x 6 x 1.5 = 18 min, warm-ups ~10 min: **~80 min** (+18 min with the lamp-cache arm).

**Chain 2, render knobs:** arms shipped base / `BEVYRROWIND_CASCADE_ENCODERS=1` / `--sun-cascades 2:205` /
`BEVYRROWIND_DLSS=performance` / `--texture-cap 512`; walk + standing, 4 repeats ABBA: 5 x 4 x 2.1 = 42 min + warm-ups
~10: **~50 min**. Optional: `lampsel_old.exe` vs `lampsel_new.exe` (if still on disk) interleaved, walk + walk_night,
6 repeats: ~36 min.

Total: **~2 h 10 min** core, ~3 h with both options. Decision rule to write down before running: a switch wins if the
median delta exceeds the larger arm's min-max range or a rank test gives p < 0.05 with 6 runs, and per-thread means
say which thread moved; anything else is "inconclusive", which keeps code opt-in, never deletes it.
