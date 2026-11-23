# Saved perf runs

Each folder is one `perf suite` run (`docs/perf.md` "Suite runner"): `summary.md` (median of runs) and, per run,
the report `.md` + `.json` (the `--save` copy; the `.csv` traces stay in `target/perf/<tag>/`). Newest first.
Read results in `docs/perf.md`; older dated numbers live in `docs/archive/perf-log-2026-10.md`.

| folder | date | build | command | read in |
|---|---|---|---|---|
| `gc_p0b/` | 2026-11-23 | main `75f496d8` + the groundcover lane (GC-M1, GC-M2), profiling + `dlss` | `perf suite gc_p0b gc_ascadian balmora` ×3, arms on / worst (`groundcover.render.reserve_worst_case=true`) / `--off groundcover`, 2560x1440, `BEVYRROWIND_DLSS=quality` | `docs/ledger/groundcover.md`, `docs/ledger/perf.md` 2026-11-23 |
| `gc_p0/` | 2026-11-23 | main `75f496d8` + GC-M2, profiling + `dlss` | `perf suite gc_p0 gc_ascadian balmora` ×3, arms on / `--off groundcover` / empty (`groundcover.density_scale=0`); the worst-case slot reserve and its bistable fault | same |
| `s0_baseline/` | 2026-11-10 | main `018c0f6`, Delirium off, profiling + `dlss` | S0 suite (8 scenarios ×3, 2560x1440, `BEVYRROWIND_DLSS=quality`) | perf.md "S0 baseline results", "Frame budget" |
| `ab_before_static_scene/` | 2026-11-10 | tag `ab/before-static-scene` (`3e219bf`), Delirium off, own bake | same suite; `balmora_night` run 1 lost to a GPU `DeviceLost` (its `log.txt` kept) | perf.md "S0 baseline results" (A/B paragraph) |
