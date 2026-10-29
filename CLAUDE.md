# Scrib (one repository since 2026-10-25)

This folder is the `scrib` git repo: the framework and the Morrowind game, both GPL-3.0-or-later (+ GPU vendor
runtime permission, `LICENSE-EXCEPTION.md`). Each part's instructions are the source of truth:

- `bevyrrowind/CLAUDE.md`: Morrowind content pack and game. Start here; roadmap in `bevyrrowind/docs/ROADMAP.md`.
- `superfargoth/CLAUDE.md`: game-agnostic framework. Architecture in `superfargoth/docs/ARCHITECTURE.md`.
- Hard problems reserved for the owner's strongest agent: `bevyrrowind/docs/hard-problems.md`.

Git: one repo at this root (`git` works from either subfolder; commit with explicit paths). The two parts keep their
names, paths and crate prefixes (`sf_*` framework, `mw_*` game); the framework/game split is architectural (no
Morrowind code in `superfargoth/`), not a licence wall. Never commit `sdk/` (NVIDIA's SDK may not be
redistributed). Machine-specific notes (build folders, old repos) live in `CLAUDE.local.md`, untracked.

Cargo: ONE workspace at the root (`Cargo.toml`, `Cargo.lock`, `.cargo/config.toml`, `rust-toolchain.toml`,
`deny.toml`, `.config/hakari.toml`); cargo works from any subfolder, and `--workspace` means both parts.
`superfargoth/plugins/sf_dlss` is its own workspace (links NVIDIA; built separately). GitHub: `origin` =
github.com/lihogloglo/Scrib.

Parallel sessions register in `.sessions/<name>.md` (scope, status) and run
one cargo build at a time. Full protocol: `bevyrrowind/CLAUDE.md`, "Parallel sessions".

Working rules for everyone (research first, reuse order, finish before starting, tests, docs, agents):
@CONTRIBUTING.md

System map: `docs/SYSTEMS.md` (find your system's row: specs and sections, crates, ROADMAP and visual-check IDs; read only those).
