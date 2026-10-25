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

### Settings (repo root: one Cargo workspace for framework and game)

| Setting | Where | Why |
|---|---|---|
| `rust-lld` linker | root `.cargo/config.toml` | Linking is the slowest step of an incremental Bevy build; lld is several times faster than `link.exe`. Ships with Rust. |
| `bevy/dynamic_linking` | app `dev` feature, `cargo dev` alias | Bevy becomes a shared library, so a relink no longer re-links Bevy. The biggest incremental win. Never in shipped builds. |
| Deps at `opt-level = 3`, ours at `1` | `[profile.dev]` | Dependencies build once; Bevy at opt-level 0 is too slow to run. |
| `debug = "line-tables-only"`, deps `debug = false` | `[profile.dev]` | Readable backtraces with far less debug info to generate and link. `--profile debugging` for a full debugger build. |
| Optimized build scripts and proc macros | `[profile.dev.build-override]` | Derive macros run on every build of their users. |
| Pinned toolchain | root `rust-toolchain.toml` | Same compiler everywhere; a compiler change invalidates every cache. |
| `profiling` / `dist` profiles | root `Cargo.toml` | Release with symbols for Tracy; thin LTO + 1 codegen unit only for shipping. |
| Feature unification (`cargo hakari`) | root `.config/hakari.toml`, `superfargoth/crates/workspace-hack` | One feature set per dependency, so `cargo test -p A` and `-p B` share one Bevy build. Bevy-free cores (`sf_*` and `mw_*`) and the game app stay out; superfargoth/CLAUDE.md "Commands". |
| Dependency policy (`cargo deny`) | root `deny.toml` | Licences, advisories, duplicates, sources; superfargoth/README.md "Dependency policy". |

Checked against Bevy 0.20's `.cargo/config_fast_builds.toml` (2026-10-12): its Windows section is
exactly `rust-lld.exe` + `rustdocflags`, with every other flag nightly-only. rust-lld is not the
default linker on Windows MSVC (it is on x86_64 Linux), so the config line stays.

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
| No-op build | ≤ 1 s | 0.83 s |
| Touch the app or a leaf plugin crate | ≤ 5 s | 3.3-4.7 s |
| Touch any hot crate (`sf_world_bevy`, `sf_model_bevy`, `mw_nif`, `sf_model`) | ≤ 8 s | 5.2-8.1 s (`sf_model`: 13 dependents, at the limit) |
| Edit a function body in a hot crate | ≤ 10 s | 6.2-10.3 s |
| Clean dev build | ≤ 12 min | 8 min 38 s (M1: 11 min) |
| Cold bake (Morrowind, fresh store) | ≤ 2 min | 47.7 s |
| No-op bake | ≤ 2.5 s | 1.75 s |

Incremental time is cargo's freshness check (~0.8 s) + ~0.5-1 s per crate between the edit and
the app + the app crate (~2 s rustc, 0.73 s link). It grows with chain depth, not crate size: keep
rule 3 (wide and shallow) and rule 5 (split by change frequency) in mind for `sf_model`.

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

