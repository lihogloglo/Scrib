# Scrib (one repository since 2026-10-25)

This folder is the `scrib` git repo: the framework and the Morrowind game, both GPL-3.0-or-later (+ GPU vendor
runtime permission, `LICENSE-EXCEPTION.md`). Each part's instructions are the source of truth:

- `bevyrrowind/CLAUDE.md`: Morrowind content pack and game. Start here; roadmap in `bevyrrowind/docs/ROADMAP.md`.
- `superfargoth/CLAUDE.md`: game-agnostic framework. Architecture in `superfargoth/docs/ARCHITECTURE.md`.
- Hard problems reserved for the owner's strongest agent: `bevyrrowind/docs/hard-problems.md`.

Git: one repo at this root (`git` works from either subfolder; commit with explicit paths). The two parts keep their
names, paths and crate prefixes (`sf_*` framework, `mw_*` game); the framework/game split is architectural (no
Morrowind code in `superfargoth/`), not a licence wall. Old standalone repos: `D:\Gamedev\_old-git\` (history is
merged here) and github.com/lihogloglo/{superfargoth,bevyrrowind} (frozen; never push to them).
Not tracked: `sdk/` (NVIDIA SDK, never redistribute), `shots/`, `procgen/`, `cloud-prompts/`, `.sessions/`.

Cargo: ONE workspace at the root (`Cargo.toml`, `Cargo.lock`, `.cargo/config.toml`, `rust-toolchain.toml`,
`deny.toml`, `.config/hakari.toml`); cargo works from any subfolder, and `--workspace` means both parts.
`superfargoth/plugins/sf_dlss` is its own workspace (links NVIDIA; built separately). GitHub: `origin` =
github.com/lihogloglo/Scrib.
Build folders: `target`, `bevyrrowind/target` and `superfargoth/target` are directory junctions to one
folder, `C:\cargo-target\scrib` (D: is nearly full; owner, 2026-10-12; merged 2026-10-25). Paths like
`bevyrrowind/target/release/...` work unchanged; don't replace the junctions with real folders. Remove a junction
only with `cmd /c rmdir <path>` (never `rm -rf`: it deletes the cache behind it). `C:\cargo-target\superfargoth`
is the old framework cache, deletable.

Parallel sessions register in `.sessions/<name>.md` (scope, status) and run
one cargo build at a time. Full protocol: `bevyrrowind/CLAUDE.md`, "Parallel sessions".
