# Saved perf runs

Each folder is one `perf suite` run (`docs/perf.md` "Suite runner"): `summary.md` (median of runs) and, per run,
the report `.md` + `.json` (the `--save` copy; the `.csv` traces stay in `target/perf/<tag>/`). Newest first.
Read results in `docs/perf.md`; older dated numbers live in `docs/archive/perf-log-2026-10.md`.

| folder | date | build | command | read in |
|---|---|---|---|---|
| `s0_baseline/` | 2026-11-10 | main `018c0f6`, Delirium off, profiling + `dlss` | S0 suite (8 scenarios ×3, 2560x1440, `BEVYRROWIND_DLSS=quality`) | perf.md "S0 baseline results", "Frame budget" |
| `ab_before_static_scene/` | 2026-11-10 | tag `ab/before-static-scene` (`3e219bf`), Delirium off, own bake | same suite; `balmora_night` run 1 lost to a GPU `DeviceLost` (its `log.txt` kept) | perf.md "S0 baseline results" (A/B paragraph) |
