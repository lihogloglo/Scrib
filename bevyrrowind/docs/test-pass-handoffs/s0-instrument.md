# s0-instrument (2026-11-08): handoff, written without compiling

**Update 2026-11-08 (orchestrator): compiled and tested on the laptop** (Rust 1.98.1 + VS 2026 Build Tools installed); see the static-scene.md Slices row. Fixes applied after compiling are noted there.

Scope: static-scene slice S0, per-view render counters in the perf report. Nothing committed. This PC has no Rust
toolchain: no cargo command was run (none built, tested or linted). First compile happens on the main workstation.

## Files
- New: `superfargoth/crates/sf_diag/src/views.rs` (Bevy-free: `ViewKind`, `PhaseCounts`, `ViewCounts`, `ViewFrame::record`
  → gauges, `ViewTable` → `Report::views`; 4 unit tests), `superfargoth/crates/sf_diag_bevy/src/views.rs`
  (`RenderViewsPlugin`, feature `views`: render-world `count_views` in `RenderSystems::Render` after `render_system`).
- Changed: `sf_diag` lib.rs (`pub mod views`), report.rs (`Report::views`, serde default/skip-empty);
  `sf_diag_bevy` Cargo.toml (feature `views` = render + optional bevy_core_pipeline, bevy_pbr), lib.rs (`DiagViews`
  shared slot, `DiagScenario::views`, `frame_boundary` takes it on measured frames, `finish` fills `report.views`),
  tests.rs (`render_view_counts_reach_the_report`, synthetic frames, sanity only);
  vendor `bevy_render/src/render_phase/mod.rs` (`bin_count`, `batch_set_count`), `bevy_pbr/src/render/gpu_preprocess.rs`
  (`PreprocessBindGroupCount`), vendor README row; `mw_devtools_bevy` Cargo.toml (feature `views`) + perf.rs (plugin
  added); `apps/bevyrrowind/src/perf/cli.rs` (5 summary columns).
- Docs: diag.md "Render views", static-scene.md S0 row, perf.md "Static scene baseline (S0)", CHANGELOG.

## Owner: run on the workstation, in order
1. `bash bevyrrowind/tools/cargo_turn.sh test -p sf_diag`
2. `bash bevyrrowind/tools/cargo_turn.sh test -p sf_diag_bevy` and `... check -p sf_diag_bevy --features views`
3. `bash bevyrrowind/tools/cargo_turn.sh clippy -p sf_diag -p sf_diag_bevy -p mw_devtools_bevy --all-targets --features sf_diag_bevy/views -- -D warnings`
4. `bash bevyrrowind/tools/cargo_turn.sh hakari generate --diff` and `... hakari manage-deps --dry-run` (new optional deps)
5. `bash bevyrrowind/tools/cargo_turn.sh build -p bevyrrowind --profile profiling --features dlss`
6. Baseline: perf.md "Static scene baseline (S0)" (two `perf suite` commands).

## APIs I was unsure of (check first if it fails to compile)
- `#[derive(SystemParam)] struct Phases<'w>` with only `'w` (if rejected: add `'s` + `PhantomData`).
- `bevy_render::render_resource::CachedRenderPipelineId` being `Hash + Eq + Copy` (it is a field of `Hash` keys).
- `RawBufferVec::len` / `PartialBufferVec::len` (usize) on `PreprocessWorkItemBuffers` variants.
- Tuple-field coercion `&mut Recorder` → `&Recorder` in `finish(run.scenario(), (recorder, warmup), ...)`.
- `Option::is_none_or` (Rust 1.82+, toolchain 1.98 ok).

## Not counted
- Per-entity re-specialisation (no common tick in the per-view specialisation caches): "specialised this frame" is
  `pipelines/created` from `PipelineChurnPlugin`.
- Late (occlusion) work items: they re-read the early items, so only the early pass is counted.
