# superfargoth

Game-agnostic open-world ARPG framework on Bevy. **GPL-3.0-or-later** with a
section 7 permission for GPU vendor runtimes (`LICENSE-EXCEPTION.md`; owner 2026-10-25).
Its first content pack is Bevyrrowind (`../bevyrrowind`, GPL), which drives
the roadmap: see `../bevyrrowind/docs/ROADMAP.md`.

## Start of every session

Sub-agents with a brief: read only what the brief names, specs by section; skip the rest of this
list (`../bevyrrowind/CLAUDE.md` "Start of every session"). Otherwise:

1. Read `../bevyrrowind/docs/ROADMAP.md` (short: owner decisions, current phase).
2. For structural work, read `docs/ARCHITECTURE.md` (vision, crate plan, build rules §8b, now `docs/build-perf.md`);
   `docs/WISHLIST.md` when you need feature targets or performance numbers.
   `docs/DECISIONS.md` is append-only history: grep it, don't read it whole, or grep `docs/DECISIONS-INDEX.md` (generated: `node tools/decisions-index.mjs`).
3. For the system you touch, read its spec in `docs/specs/`. No spec yet
   for a major system? Write it first.

## End of every session

Update the roadmap (in the bevyrrowind repo: status mark + one "Recent" line) and append
lasting decisions to `docs/DECISIONS.md` (one dated bullet, ≤ 3 lines). Doc budget as in
`../bevyrrowind/CLAUDE.md` ("End of every session").

## Parallel sessions

Several sessions may work in this folder at once. Follow the protocol in
`../bevyrrowind/CLAUDE.md` ("Parallel sessions"): register in
`../.sessions/`, one build at a time, explicit-path commits only when asked,
never revert others' changes.

## Hard rules

1. **No game-specific code.** Nothing here names Morrowind (or any game's)
   formats, IDs, units, or conventions. Litmus test: could another game
   plug in with only a new adapter?
2. **Ported code needs provenance; proprietary SDKs stay out of the binary.**
   Code ported from GPL-3.0-compatible projects (GPL-3.0, GPL-2.0-or-later,
   LGPL, MIT, Apache-2.0, ...) is allowed: cite the source file + commit in the
   module docs and add a `THIRD_PARTY_NOTICES.md` entry. Never port
   GPL-2.0-only or non-free code. Game-specific mechanics (e.g. OpenMW's) still
   belong in the content pack (rule 1). Proprietary vendor SDKs (NVIDIA DLSS)
   are reached only through a separately shipped plugin (`plugins/`), never
   linked into the game.
3. **Clean room for other games** (WISHLIST.md "Reference policy"). In an
   analysis session (REA, game data inspection), write behavior specs only,
   with no code, pseudocode, decompiled names, or raw tables, and commit
   nothing else. In an implementation session, never open decompiled output
   or analysis notes, only `docs/specs/`. Don't mix the two in one session.
4. **Everything is a mod**: VFS, records, and mod manifests are the only content paths.
5. **Industry standard, never kludge**; named scaffolds only.
6. **Compile time is a constraint** (§8b, now build-perf.md): Bevy-free cores, thin Bevy plugins.
7. **Research first, and challenge our docs** (`../CONTRIBUTING.md` "How we decide what to build" 1).
8. **Reuse before building: Bevy, then crates, then our own**, whole projects included
   (`../CONTRIBUTING.md` 2). Crates here must be GPL-3.0-compatible: MIT, Apache-2.0, BSD, zlib,
   GPL-3.0, LGPL; MPL-2.0 only for unmodified dependencies (owner 2026-10-11); never GPL-2.0-only or
   AGPL (`deny.toml`).
9. **Finish before starting** (`../CONTRIBUTING.md` 6).

## Commands

One Cargo workspace for framework and game (root `Cargo.toml`): `--workspace` covers both, and
`deny.toml`, `.config/hakari.toml` and the lock live at the repo root. Run these from the root
(through `bash bevyrrowind/tools/cargo_turn.sh`, `../bevyrrowind/CLAUDE.md` "Parallel sessions"):

```sh
cargo test --workspace --all-features
cargo clippy --workspace --all-features --all-targets -- -D warnings
cargo deny check   # licences, advisories, duplicates (deny.toml; README "Dependency policy")
cargo hakari generate --diff && cargo hakari manage-deps --dry-run   # workspace-hack is current
```

**Adding a crate or a dependency** (`cargo install --locked cargo-hakari`): run
`cargo hakari generate && cargo hakari manage-deps` from the root and commit
`superfargoth/crates/workspace-hack` with the manifests. The hack pins one feature set per
dependency, so `cargo test -p A` and `-p B` share one Bevy build. A Bevy-free crate (framework
`sf_*` core or game `mw_*` core) goes in `final-excludes.workspace-members` of `.config/hakari.toml`
first: the hack carries Bevy, and cores must build cold in seconds (§8b rule 1, now build-perf.md). The game app is
traversal-excluded so its opt-in features (`dev`, `dlss`, `solari`, ...) never reach other builds;
for the same reason no dev-dependency asks for an opt-in Bevy feature (`file_watcher`,
`bevy_solari`): pass it on the command line instead (`--features bevy/file_watcher`).
