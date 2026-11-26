# Build performance

Moved verbatim from [ARCHITECTURE.md](ARCHITECTURE.md) section 8b (doc cleanup S14, 2026-10-26); still cited as "ARCHITECTURE §8b".

## 8b. Build performance

Compile time is a design constraint, like frame time. The rules:

### Crate graph

1. **Bevy-free cores.** Logic that doesn't need the engine lives in crates
   that don't depend on Bevy: formats, records, mod resolution, rules,
   conversion (`sf_records`, `sf_mods`, `mw_install`, `mw_records`, `mw_nif`,
   `mw_world`). They build in seconds, their tests skip Bevy's link, and
   most logic changes only touch them.
2. **Thin Bevy shells.** A Bevy crate is a plugin: components, systems,
   asset loaders, gluing a core to the engine (`mw_assets` over `mw_nif`,
   `sf_vfs`'s `bevy` feature over `sf_vfs`). Keep heavy code out of them.
3. **Wide and shallow, not tall.** Many sibling crates build in parallel;
   a chain A → B → C → D builds serially. Avoid a "common" crate that
   everything depends on and that changes often: editing it rebuilds the
   world. Shared types that change rarely are fine there.
4. **Thin binary.** `apps/*` only wires plugins together. Gameplay never
   lives in the binary crate.
5. **Split by change frequency, not just by topic.** A crate that is
   edited daily should have few dependents.
6. **Minimal Bevy features per crate.** Libraries depend on
   `bevy = { default-features = false, features = [...] }` with only what
   they use; the app enables the full set. Disabled features are crates
   that never compile.
7. **Generics stay local.** Heavily generic code is monomorphized in every
   crate that uses it. Expose concrete or `dyn` APIs at crate boundaries
   where performance allows (e.g. `KindVisitor` instead of generic
   callbacks everywhere).

Game plugin crates (`bevyrrowind/crates`, app split 2026-10-26; the binary `apps/bevyrrowind` keeps only the CLI; spec `bevyrrowind/docs/specs/app-split.md`):

| Crate | Holds |
|---|---|
| `mw_game_core` | Shared types: markers, enums, constants, scene registry, shipped_errors |
| `mw_boot_bevy` | Mods lib half, bake, hot_reload, ui_theme, load/mount |
| `mw_controls_bevy` | Keymap, controls, l10n strings |
| `mw_doors_bevy` | Doors |
| `mw_environment_bevy` | Water, weather, night_sky, render |
| `mw_audio_bevy` | Audio and voices |
| `mw_walkers_bevy` | Walkers, physics |
| `mw_actors_bevy` | Actors, actor_fx, actor_particles, faces, held |
| `mw_player_bevy` | Player and its tests |
| `mw_scripts_bevy` | Script runtime glue |
| `mw_rpg_bevy` | Dialogue, inventory, inventory_ui |
| `mw_saves_bevy` | Saves |
| `mw_devtools_bevy` | Console, live_ab, remote, pipeline_watch, perf, ready (Loading, settled, Readiness) |
| `mw_viewers_bevy` | view, anim_view, skinned_view, character_view |
| `mw_exterior_bevy` | Exterior, level (+ level/actors), pause_menu |

### Settings (repo root: one Cargo workspace for framework and game)

| Setting | Where | Why |
|---|---|---|
| `rust-lld` linker | root `.cargo/config.toml` | Linking is the slowest step of an incremental Bevy build; lld is several times faster than `link.exe`. Ships with Rust. |
| `bevy/dynamic_linking` | app `dev` feature, `cargo dev` alias | Bevy becomes a shared library, so a relink no longer re-links Bevy. The biggest incremental win. Never in shipped builds. Not only the top crate: it also sets `bevy_diagnostic/dynamic_linking` (drops sysinfo diagnostics), so `dev` is its own full Bevy ("Feature unification"). |
| Deps at `opt-level = 3`, ours at `1` | `[profile.dev]` | Dependencies build once; Bevy at opt-level 0 is too slow to run. |
| `debug = "line-tables-only"`, deps `debug = false` | `[profile.dev]` | Readable backtraces with far less debug info to generate and link. `--profile debugging` for a full debugger build. |
| Optimized build scripts and proc macros | `[profile.dev.build-override]` | Derive macros run on every build of their users. |
| Pinned toolchain | root `rust-toolchain.toml` | Same compiler everywhere; a compiler change invalidates every cache. |
| `profiling` / `dist` profiles | root `Cargo.toml` | Release with symbols for Tracy; thin LTO + 1 codegen unit only for shipping. |
| Feature unification (`cargo hakari`) | root `.config/hakari.toml`, `superfargoth/crates/workspace-hack` | One feature set per dependency, so any `cargo test -p A`, `-p B`, `--workspace` and `build -p bevyrrowind` share one Bevy build ("Feature unification" below). Bevy-free cores (`sf_*` and `mw_*`) and the game app stay out of the hack; the tes3 crates are traversed but never named in it; CONTRIBUTING.md "Building". |
| Dependency policy (`cargo deny`) | root `deny.toml` | Licences, advisories, duplicates, sources; superfargoth/README.md "Dependency policy". |

Checked against Bevy 0.20's `.cargo/config_fast_builds.toml` (2026-10-12): its Windows section is
exactly `rust-lld.exe` + `rustdocflags`, with every other flag nightly-only. rust-lld is not the
default linker on Windows MSVC (it is on x86_64 Linux), so the config line stays.

**Feature unification** (build-speed BS1, 2026-11-11). Cargo resolves features per command, so a
selection that turns on one more feature anywhere in Bevy's dependency closure (`half/nightly`,
`slotmap/default`, an audio codec) rebuilds all of Bevy; 9 Bevy rebuilds a day were measured. Rules:
every crate that pulls a feature into Bevy's closure is traversed by hakari (the tes3 crates sit in
`final-excludes`, not `traversal-excludes`); the app (traversal-excluded for its opt-ins) asks for no
Bevy feature its libraries don't (audio: `mw_exterior_bevy`, codecs: `mw_audio_bevy`, PCSS:
`mw_exterior_bevy`). `cargo hakari verify` never passed here (the final-excluded cores build Bevy
crates with fewer features, by design); the check is a per-selection `cargo tree -f '{p}|{f}'` diff over
`bevy_internal`'s closure. Measured: `test -p sf_ui_bevy --no-run`, then `test -p mw_exterior_bevy
--no-run`, then `build -p bevyrrowind`: 54 s, 91 s, 42 s wall; bevy_render/wgpu/naga `Fresh` in all
three, and the 2nd and 3rd compiled no third-party crate at all. Left by design: each app opt-in
(`dev`, `dlss`, `solari`, `trace`) is its own Bevy, and clippy/check keeps one check-mode Bevy.
`dev` (BS1b) forks from Bevy's root and cannot be unified: `remote` turns on `bevy_platform`/`bevy_tasks`
`async-io` (worker threads and `TaskPool::scope` then block on async-io's IOCP reactor: a hot-path change,
no for shipped builds) and `windows-sys` networking, `dynamic_linking` drops sysinfo (benchmarks record
it), `file_watcher` forks `bevy_asset`. The variants coexist in `target/` (different hashes): measured, ext
test, `build --features dev`, ui test: Bevy all `Fresh` after one build of each; the cost is per Bevy change.
Our crates (BS1c): hakari skips workspace members, so a Bevy crate's game features are set once on its root
`[workspace.dependencies]` entry (members use `.workspace = true`, not `path`), no `default` list feeds only
`--workspace`, and no feature exists only for dev-deps. ext test -> app build recompiled 18 of our crates, now 2
(app-only `mw_viewers_bevy`, the app); ws/viewers/devtools match the app exactly. Left: core features (`sf_vfs`/`sf_ids` `bevy`, bake ones; up to 22 crates under `-p mw_boot_bevy`).

Since the merge into one workspace (2026-10-25), framework crates are workspace members like the
game's, so `[profile.dev.package."*"]` no longer applies to them: they build at opt-level 1 with
line tables, and `--profile debugging` covers them. Before, as path dependencies of bevyrrowind,
they built at opt-level 3; measured then (health, 2026-10-12), forcing opt-level 1 changed no touch
or body-edit time beyond noise. Not re-measured: framework hot paths may run slower in `cargo dev`.

### Gates (dev loop)

Measured 2026-10-12 on the reference laptop (Ryzen 7 7435HS, 16 threads), idle, bevyrrowind
`cargo dev` path; method and breakdown in `bevyrrowind/docs/perf.md` "Dev loop". Re-measure when a
crate is added to the app's graph or Bevy is upgraded; a gate miss is a bug.

| Gate | Limit | Measured |
|---|---|---|
| No-op build | ≤ 1 s | 0.83 s; 2026-11-11: 1.08 s |
| Touch the app or a leaf plugin crate | ≤ 5 s | 3.3-4.7 s; 2026-11-11 app: 6.1 s (miss) |
| Touch any hot crate (`sf_world_bevy`, `sf_model_bevy`, `mw_nif`, `sf_model`) | ≤ 8 s | 5.2-8.1 s (`sf_model`: 13 dependents, at the limit); 2026-11-11 (warm, after the incremental wipe): `sf_world_bevy` 15.5 s, `mw_doors_bevy` 12.2 s, `sf_model` 29.4 s (now 111 dependents): misses, see ROADMAP 1.14 |
| Edit a function body in a hot crate | ≤ 10 s | 6.2-10.3 s |
| Clean dev build | ≤ 12 min | 8 min 38 s (M1: 11 min) |
| Cold bake (Morrowind, fresh store) | ≤ 2 min | 47.7 s |
| No-op bake | ≤ 2.5 s | 1.75 s |

The gates are single measurements; the real workday is in the build log: every `cargo_turn.sh` command appends
its queue wait (by reason), cargo run time, exit and compiled units / Bevy crates to `.sessions/build-log.tsv`
(untracked), and `bash bevyrrowind/tools/cargo_turn.sh --stats [days]` sums it (since 2026-11-17).

Incremental time is cargo's freshness check (~0.8 s) + ~0.5-1 s per crate between the edit and
the app + the app crate (~2 s rustc, 0.73 s link). It grows with chain depth, not crate size: keep
rule 3 (wide and shallow) and rule 5 (split by change frequency) in mind for `sf_model`.

### Queue, CI and worktrees (build-speed BS2/BS3, 2026-11-26)

The build log (7 days to 2026-11-26) gave 1022 min of cargo and 578 min waiting, 495 of them behind other tickets;
one `test --workspace` with nothing to compile still ran 290 s. Three changes, results in `docs/ledger/build.md`:

- **Tests run outside the queue.** `cargo_turn.sh` splits `cargo test` into `test --no-run` (holds the ticket) and
  `cargo test-run` (an alias set through `CARGO_ALIAS_TEST_RUN`, which `busy()` ignores; a `--config` alias doesn't
  work, cargo ignores aliases from it). Cargo releases its target lock before running the binaries (checked: a build
  finished in 3 s while a 20 s test ran). rustdoc.exe now counts as busy: doctests compile in the run phase.
- **CI** (`.github/workflows/ci.yml`): test, both clippy runs, and a no-compile policy job (deny, hakari, tunables
  census, all blocking; the census passes again since ec45d48e) on push to `main`, `ci/**`, `wt/**` and PRs; docs-only
  pushes skip it. On `ci/**`, `wt/**` and PRs a newer push cancels the running one; on `main` a running run is never
  cancelled and only the newest pending one waits behind it, so a verdict covers the batch of commits pushed during
  the previous run and a red one lists its suspects (a run per commit was tried on 2026-11-26: 21 runs in progress
  or queued within 40 min, at GitHub's 20-job limit). Windows runners for parity with the dev machine; LFS clips from the Actions cache
  (free LFS bandwidth ~1 GB/month). `tools/ci_push.sh` pushes HEAD + given paths (with gh's credential).
- **CI cache** (ci-2, `.github/workflows/cargo_cache.mjs`): the whole `target` + registry, one entry per job and
  main commit, restored by every branch (newest main entry). `Swatinem/rust-cache` was dropped: it keeps only packages outside the
  repo root, so our 146 crates and the 9 vendored ones (wgpu-core, bevy_render...) rebuilt on every run. Two helpers
  make the cached path crates usable: `mtimes` gives every file unchanged since the cached commit (git diff) back the
  exact mtime that build saw (recorded in the cache; equality matters for build scripts without rerun-if-changed),
  changed files keep the checkout's; `prune` keeps only the units this run's cargo JSON listed (examples and bins found
  by their fingerprint file) and stamps the commit. Measured: cold 54 min, no-change floor 2.9 min build (`clippy`
  42 s); a change low in the graph (sf_ids) still rebuilds every local crate (~45 min). A red step names its crates and,
  on main, the suspect commits since the last green ancestor (`ci_report.mjs`, `gh run view <id>`).
- **CI test shards** (ci-3, `.github/workflows/test_groups.mjs`): the test job runs as three jobs, each
  `cargo test -p <its crates>` with its own cache (`test-v2-...-s<i>of3-<sha>`). Crates are ordered by how many
  workspace libs they need and cut where the slowest shard is shortest (weights = source bytes, no list to keep);
  each shard passes the features the whole workspace resolves for its crates, because nine framework crates get
  features only from game crates and would otherwise be tested bare. Measured after a change low in the graph
  (sf_ids): one job 23 min of build, slowest shard 13. The floor is the shard holding the app (every lib, ~15 min
  when all of them rebuild), so more runners stop paying after three or four. A job saves a cache entry only when
  it compiled something (three shards + clippy are ~6.9 GB of GitHub's 10 GB when all save).
- **One integration test binary per crate** (ROADMAP 1.14, ci-4; first `mw_bake` 29 files, `mw_graphs` 6): a crate with
  more than two integration test files keeps them in `tests/it/<file>.rs`, each a module of `tests/it/main.rs` (a new
  file needs its `mod` line there), so cargo links one test executable where it linked one per file (no manifest
  change: cargo finds `tests/<dir>/main.rs` by itself). One file: `cargo test -p <crate> --test it <file>::`;
  measurements: `... --test it <file>:: -- --ignored --nocapture`. All the crate's integration tests now share one
  process and its thread pool: a test that needs a fixed temp path, an environment variable or the working directory
  for itself must not assume it runs alone.
- **Worktrees** (`tools/worktree.sh`): own target per checkout, `C:\cargo-target\scrib-wt\<name>` via junctions.
  A shared target is unsafe: cargo hashes a path crate by its path relative to the workspace root and checks
  freshness by mtime, so two checkouts would reuse each other's artifacts. Not adopted yet: sccache to share the
  registry crates between targets (the vendored Bevy patches are path crates, built incrementally, so uncacheable).
  Not adopted: cargo-nextest (process per test; several suites share one loaded fixture through statics).
- **Cheap queue scans** (idle-1 L1): one `ps -W` (~0.12 s) lists every Windows process; PowerShell runs only when a
  cargo.exe or bevyrrowind.exe needs its command line; waiters poll every 1 s. Overhead per command 1.9 s -> 0.45 s.
  Edit `cargo_turn.sh` on a copy and `mv` it in: bash reads a running script by offset, so an in-place edit can
  derail commands already queued.
- **Check lane** (idle-1 L2): `clippy`/`check` build check-mode units (rmeta only), which share nothing with test or
  build units but took the same target-dir lock, so a 5 s clippy waited behind a test build (54 min/day) and tests
  behind clippy (18 min/day). They now build in `<target>-check` (`C:\cargo-target\scrib-check`, a worktree's
  `<name>-check`) in queue lane `c`, beside lane `b`; lanes merge while the owner plays. Not for sf_dlss, a checkout
  without a target junction, or a caller-set `CARGO_TARGET_DIR`. Same fix as rust-analyzer's `cargo.targetDir`; the
  proper one, cargo's `-Zfine-grain-locking` (cargo#16155), is still unstable on 1.98.1.

### Machine setup (Windows)

- **Exclude the `target/` folders (and `~/.cargo`) from Windows Defender
  real-time scanning.** Scanning every object file and rlib is often the
  single largest overhead on Windows. Needs admin; do it once.
- Use a Dev Drive (ReFS) for the repos and cargo home if available.

### Not adopted (yet)

- **Cranelift backend, `-Zshare-generics`, the parallel frontend
  (`-Zthreads`)**: real wins but nightly-only. Revisit if they reach stable.
  Owner decision (health, 2026-10-12; not measured, no nightly is installed): a nightly toolchain
  for dev builds only. Bevy's fast-builds config lists `-Zthreads=0` and `-Zcache-proc-macros`
  for Windows and warns that `-Zshare-generics` must stay off (`=n`) with dynamic linking there
  (65k export limit). Cranelift ships for Windows but without panic unwinding (`panic=abort`).
  Where it would help: the app crate's ~2 s of rustc per edit and the clean build's Bevy chain.

