# wgpu_walk (2026-11-17)

Hand-written: the suite marked every run failed ("DLSS `quality` requested but no DLSS pass was timed": the exe was built
without `--features dlss`, so the runs are TAA at native 2560x1440) and wrote no summary. The runs themselves completed;
per-frame CSVs are in `target/perf/wgpu_walk/`. Means per run of the measured frames, `balmora_walk`, exe
`target/profiling/wgpuwalk.exe` (commit 69ce9bd0 + uncommitted work), interleaved order from the suite, focus clean
(`window/unfocused` 0 in all nine). Use the CPU columns only.

| run | frame | render busy | render graph | encoders | submit | finish wall | encoder thread-ms | finish critical | images | batch sets |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| base-1 | 13.30 | 10.92 | 6.50 | 37.8 | 2.48 | 3.05 | 11.44 | 2.66 | 2551 | 70.2 |
| base-2 | 13.52 | 11.17 | 6.66 | 39.6 | 2.53 | 3.12 | 11.95 | 2.71 | 2558 | 72.9 |
| base-3 | 13.67 | 11.30 | 6.75 | 39.8 | 2.55 | 3.18 | 12.14 | 2.77 | 2558 | 72.7 |
| onetex-1 | 12.00 | 9.11 | 4.66 | 39.6 | 2.33 | 1.28 | 6.75 | 0.90 | 772 | 71.9 |
| onetex-2 | 12.16 | 9.23 | 4.72 | 41.1 | 2.38 | 1.27 | 6.83 | 0.88 | 777 | 74.2 |
| onetex-3 | 12.43 | 9.68 | 4.86 | 39.5 | 2.40 | 1.38 | 7.30 | 0.99 | 778 | 71.8 |
| nomat-1 | 13.14 | 10.82 | 6.33 | 39.6 | 2.56 | 2.77 | 10.31 | 2.39 | 2547 | 72.5 |
| nomat-2 | 13.55 | 11.16 | 6.54 | 39.8 | 2.58 | 2.91 | 10.62 | 2.53 | 2547 | 72.7 |
| nomat-3 | 13.31 | 10.98 | 6.48 | 41.4 | 2.62 | 2.84 | 10.65 | 2.44 | 2556 | 74.0 |

Arms: `base` shipped; `onetex` = `--off texvariety`; `nomat` = `--set render.depth_only.skip_material=on`.
Verdict: ledger perf 2026-11-17 "Texture-walk ceiling".
