# Build speed research BS2 (2026-11-11)

Research only; nothing was built or measured here. Environment: Bevy 0.20 (released 2026-10-08), Windows 11 MSVC,
Rust 1.98.1 stable, rust-lld, ~120 crates, one shared target dir. Tags: **[src]** = read at the URL, **[known]** =
background knowledge, not re-verified today (verify before relying), **[infer]** = my reasoning.
Our current state: `superfargoth/docs/build-perf.md` (dynamic_linking in the `dev` feature, alias `cargo dev`,
dev profile opt-level 1 / deps 3, nightly flags "not adopted").

Search was shallow (web search returned mostly mirrors); unconfirmed items are marked "unverified".

## Q1. Unify tests and `cargo dev` on one Bevy

**Fact base**
- Bevy setup docs: dynamic linking is "the most impactful compilation time decrease"; on Windows you must also
  enable the fast-build optimisations or hit "too many exported symbols"; `cargo test --doc` needs
  `rustc --print target-libdir` on PATH. [src] https://bevy.org/learn/quick-start/getting-started/setup/
- Bevy `config_fast_builds.toml`: share-generics must be `n` with dynamic linking on Windows (65k symbol limit).
  [src] https://raw.githubusercontent.com/bevyengine/bevy/main/.cargo/config_fast_builds.toml
- Bevy cargo features: `dynamic_linking`, `serialize`, `debug`, `async-io`, `hotpatching` all exist on main.
  [src] https://raw.githubusercontent.com/bevyengine/bevy/main/docs/cargo_features.md
- rust-lld + dynamic linking crash on Windows 11 (STATUS_STACK_BUFFER_OVERRUN) in an old Bevy issue; unverified
  for 0.20/1.98. https://github.com/bevyengine/bevy/issues/2921 We already run both with `cargo dev`, so it works today.
- TheBevyFlock `bevy_new_2d` uses a `dev` cargo feature and passes hotpatching as a CLI feature; its README shows no
  dynamic_linking details. [src] https://github.com/TheBevyFlock/bevy_new_2d . I found no published postmortem on
  dynamic_linking in workspace tests (unverified either way).

**Key observation [infer; check with `cargo tree -e features -i bevy`]:** in Bevy, `dynamic_linking` only adds the
`bevy_dylib` dependency to the top `bevy` crate, which re-exports `bevy_internal`. The `bevy_*` rlibs (ecs, render,
...) are the same compiled artifacts if their feature sets match. So the rebuild between test and `cargo dev` comes
from the *feature set* (`serialize`, `debug`, `async-io`, `bevy/dev` tools), not from dynamic linking itself.
Fix features first; dynamic linking is then a cheap last step.

**Recommendations**
1. Put `serialize`, `debug`, `async-io` (what BRP needs) into the one shared Bevy feature list, always on, also in
   release. Cost: `serialize` adds serde impls on Bevy types (small compile and binary cost, no runtime cost)
   [infer]; `debug` adds debug names (tiny). If the owner objects for shipping, gate only `bevy/dev` (dev_tools,
   file_watcher) behind `dev` and accept that it differs. Gain: removes a duplicate Bevy build when alternating
   `cargo test` and `cargo dev` (minutes). Risk low. Effort small.
2. Do NOT run workspace tests with dynamic_linking by default. Pitfalls [known]: test exes need
   `bevy_dylib-<hash>.dll` and the sysroot std DLL on PATH (cargo adds `target/debug/deps` and the sysroot libdir
   for `cargo test`/`run`, but doctests and exes started outside cargo fail with DLL-not-found); the 65k export limit
   applies to bevy_dylib only. Gain is link time of Bevy-using test exes (Q5). If wanted, apply narrowly with an
   alias such as `ctest = "test -p <crate> --features dev"`; in a workspace `--features bevy/dynamic_linking` only
   resolves for packages selected with `-p`.
3. Keeping dynamic_linking out of release/profiling/dist is already by construction (reachable only via the app's
   `dev` feature and the `cargo dev` alias; profiles cannot toggle features, so a feature or alias is the standard,
   as in bevy_new_2d). Nothing to change.
4. Clippy's check-mode Bevy is separate by design (metadata artifacts); it cannot be unified, but identical feature
   sets (rec. 1) let `check` and `clippy` share it.

## Q2. Hotpatching (subsecond)

- Bevy 0.17 notes: feature `hotpatching`, uses Dioxus `subsecond` via `dx`; run
  `BEVY_ASSET_ROOT="." dx serve --hot-patch --features "bevy/hotpatching"`. Applies to **ECS systems** only. Limits:
  only the binary crate is patched, no Wasm, a system whose parameters change is not reloaded, "may be sensitive to
  Rust and linker configuration". [src] https://bevy.org/news/bevy-0-17/
- Example header: `dx serve --hot-patch --example hotpatching_systems --features hotpatching`. [src]
  https://github.com/bevyengine/bevy/blob/main/examples/ecs/hotpatching_systems.rs
- Still in the 0.20 feature list. The 0.19-to-0.20 migration guide (first 100k chars read) and the 0.20 news page
  show no hotpatching change. [src] https://bevy.org/news/bevy-0-20/
- Dioxus 0.7.0 claims subsecond works on Windows, patches only `subsecond::call` sites, and does not migrate state
  when struct layouts change. [src] https://github.com/DioxusLabs/dioxus/releases/tag/v0.7.0
- **Windows blocker:** Dioxus issue #4911 (open; labels bug, windows, subsecond): `dx serve --hot-patch` fails with
  rust-lld "could not open kernel32.lib" unless run from the VS x64 Native Tools prompt, because `dx` replays the
  link with a cleared environment (no `LIB`). Fix PR #5850 open. [src] https://github.com/DioxusLabs/dioxus/issues/4911
- Community crates `bevy_simple_subsecond_system` / `bevy_hotpatching_experiments` (Bevy 0.18, `#[hot]`) extend it;
  Local state resets each frame. [src] https://docs.rs/crate/bevy_hotpatching_experiments/latest
- Fit with our setup: dynamic_linking + hotpatching is not documented by Bevy, assume incompatible until tried
  (`dx` drives its own link step) [infer]. Only the binary crate patches, but our gameplay systems live in
  `mw_*_bevy` library crates, so most edits would still be a full relink [src for "binary crate only"; consequence
  is my inference].
- **What works today on Windows:** system-body edits in the app crate, launched from a Native Tools prompt, with a
  fragile linker setup. Not usable as the main loop for our crate layout. Recommendation: 1-hour spike at most;
  otherwise wait for patching across crates and the #4911 fix. Gain if it worked: sub-second edit-to-effect for app
  crate systems. Risk high. Effort medium.

## Q3. Stable vs nightly (sources up to ~April 2026; later status unverified)

| Item | Status | Value for us |
|---|---|---|
| `-Zthreads` | Still nightly. 2026 goal: stabilization report not yet filed; "parallel frontend by default" PR rust-lang/rust#146237 open, rebased Apr 2026. [src] https://rust-lang.github.io/rust-project-goals/2026/parallel-front-end.html | Bevy fast-builds config recommends `-Zthreads=0` on nightly. Helps the big app crate and clean builds. No Bevy-specific numbers found; Rust blog reports large frontend gains on its benchmarks [known] https://blog.rust-lang.org/2023/11/09/parallel-rustc/ |
| Cranelift | Nightly component, Windows x86_64 dist exists; no unwinding on Windows (`panic=abort`). [src, via search summary of the README; not confirmed on the repo page] | Our deps build at opt-level 3 with LLVM; cranelift cannot help there; `should_panic` tests break. Low value, medium risk. |
| `-Zshare-generics` | Nightly; Bevy says keep `n` with dynamic linking on Windows | Skip while dynamic_linking is used. |
| fine-grain-locking, build-dir-new-layout | Unstable in our 1.98.1. Cargo split `.cargo-lock` in two so `check` and `build` can run in parallel with fine-grain-locking; layout stabilization intended "this year"; lock follow-up PR #16463 closed (different direction). [src] https://rust-lang.github.io/rust-project-goals/2025h2/cargo-build-dir-layout.html | Exactly the "two sessions in one target dir" need. Not stable; check each Rust release's cargo notes. |
| `-Zfeature-unification` | Unstable (RFC 3692, cargo#14774); `workspace` and `package` modes implemented. [src] https://doc.rust-lang.org/stable/cargo/reference/unstable.html | Could replace hakari later, needs nightly cargo. Keep hakari. |

A dev-only nightly: only `-Zthreads=0` is a likely win and it is unmeasured for us. A different rustc rebuilds
everything, so it would need its own `CARGO_TARGET_DIR`. Not recommended until `-Zthreads` stabilizes.

## Q4. Dev Drive (ReFS) and Defender performance mode

- Microsoft: up to 30% faster overall build times vs default Windows 11 22H2 with Defender on (Surface Laptop 4);
  combines ReFS and Defender async "performance mode". [src]
  https://blogs.windows.com/windowsdeveloper/2023/06/01/dev-drive-performance-security-and-control-for-developers/
- No independent cargo benchmark separating the two effects found. Dioxus CI sets up a Dev Drive citing AV scanning
  cost (a comment, not a measurement): https://code.morphllm.com/DioxusLabs/dioxus/src/branch/main/.github/workflows/setup-dev-drive.ps1
- .NET restore: NTFS vs Dev Drive about 846 ms vs 434 ms, 1.29 s vs 730 ms (metadata-heavy IO, like cargo). [src]
  https://blog.maartenballiauw.be/post/2023/11/22/test-driving-windows-11-dev-drive-for-dotnet.html
- Caveats [src]: performance mode needs real-time protection on; a drive moved from another machine is untrusted.
- [known, verify] Needs Windows 11 22H2+, ReFS volume of at least 50 GB (Settings, System, Storage, Disks & volumes,
  "Create Dev Drive", or `Format-Volume -DevDrive`), check with `fsutil devdrv query X:`. Moving only the target dir
  gets the build-IO gain; put `CARGO_HOME` there too. Junctions into ReFS work, so the existing `target` junctions
  can point at the Dev Drive. Gated by free disk (our incremental cache once hit 660 GB).
- **Cheap alternative now:** a Defender exclusion for `C:\cargo-target\scrib`, `~/.cargo` and
  `rustc.exe`/`cargo.exe`/`rust-lld.exe` [known]. Captures the Defender share of the gain; a security tradeoff the
  owner accepts on a dev box. Gain 10-30% is the vendor claim, unmeasured here. Measure with `cargo build --timings`.

## Q5. Linkers and link time

- Wild: Windows (PE/COFF) output is on the roadmap but "pretty far down"; the 0.9.0 "Windows" item is building wild
  on Windows. Not usable. [src] https://github.com/wild-linker/wild/discussions/734 ,
  https://newreleases.io/project/github/wild-linker/wild/release/0.9.0
- rust-lld is Bevy's own recommendation for MSVC. Knobs [known, untested]: PDB generation is a major link cost, so
  `debug = 0` for test builds removes most of it (we use `line-tables-only` in dev); `-Clink-arg=/DEBUG:NONE` for
  exes nobody debugs; `/OPT:NOREF,NOICF` skips dead-code stripping (faster link, bigger exe). Try only if
  `--timings` shows link dominating. `-Csplit-debuginfo=packed` is already the Windows default.
- Many test executables: each `tests/*.rs` is its own binary linking all deps. Standard fix: one integration binary
  per crate (`tests/it/main.rs`); Cargo's own suite got ~3x less compile time and ~5x less disk, one project went
  20 s to 13 s. [src] https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html
  For Bevy-free crates prefer unit tests in `src/`. Count ours first: `ls */crates/*/tests/*.rs | wc -l`.
  `cargo nextest` parallelises running across binaries and tests but does not cut linking [known].

## Q6. sccache

Little value [known]: it cannot cache incremental artifacts (it disables incremental) and our dev loop relies on
incremental for our own crates; the shared target dir already is a local cache. It helps CI or clean builds
across machines, not this loop. Skip.

## Ranked actions

1. **Unify Bevy feature set** (serialize/debug/async-io always on; only dynamic_linking and `bevy/dev` differ).
   Gain: removes a duplicate Bevy build when switching test/dev (minutes per switch). Risk low. Effort small.
2. **Defender exclusions for target dir, CARGO_HOME, toolchain** (Dev Drive later if disk allows). Gain 10-30%
   vendor claim, unmeasured. Risk low (security tradeoff). Effort minutes. Measure with `--timings`.
3. **Consolidate integration tests to one binary per crate; `debug = 0` for test builds.** Gain: fewer links
   (up to ~3x compile in matklad's case). Risk low. Effort medium (mechanical). Count binaries first.
4. **`cargo nextest`** for `cargo test --workspace` run time. Gain: faster test phase only. Risk low. Effort small.
5. **Hotpatching spike** (1 hour, VS Native Tools prompt, app crate only). Gain: sub-second for app-crate systems
   only. Risk high (Dioxus #4911, binary-crate-only). Effort medium.
6. **Watch, do not adopt:** `-Zthreads` (revisit at stabilization), fine-grain-locking / new build-dir (would allow
   concurrent sessions in one target dir), `-Zfeature-unification` (could retire hakari), cranelift, wild (no
   Windows), sccache.
