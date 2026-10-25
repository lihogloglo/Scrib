# Scrib (one repository since 2026-10-25)

This folder is the `scrib` git repo: the framework and the Morrowind game, both GPL-3.0-or-later (+ GPU vendor
runtime permission, `LICENSE-EXCEPTION.md`). Each part's instructions are the source of truth:

- `bevyrrowind/CLAUDE.md`: Morrowind content pack and game. Start here; roadmap in `bevyrrowind/docs/ROADMAP.md`.
- `superfargoth/CLAUDE.md`: game-agnostic framework. Architecture in `superfargoth/docs/ARCHITECTURE.md`.
- Hard problems reserved for the owner's strongest agent: `bevyrrowind/docs/hard-problems.md`.

Git: one repo at this root (`git` works from either subfolder; commit with explicit paths). The two parts keep their
names, paths and crate prefixes (`sf_*` framework, `mw_*` game); the framework/game split is architectural (no
Morrowind code in `superfargoth/`), not a licence wall. Old standalone repos: `D:\Gamedev\_old-git\` (history is
merged here) and github.com/lihogloglo/{superfargoth,bevyrrowind} (frozen; don't push this repo to them).
Not tracked: `sdk/` (NVIDIA SDK, never redistribute), `shots/`, `procgen/`, `cloud-prompts/`, `.sessions/`.

Cargo: still two workspaces (`superfargoth/`, `bevyrrowind/`) until the `merge-ws` slice makes one at the root.
Build folders: `bevyrrowind/target` and `superfargoth/target` are directory junctions
to `C:\cargo-target\{bevyrrowind,superfargoth}` (D: is nearly full; owner, 2026-10-12).
Paths like `target/release/...` work unchanged; don't replace the junctions with real folders.

Parallel sessions register in `.sessions/<name>.md` (scope, status) and run
one cargo build at a time. Full protocol: `bevyrrowind/CLAUDE.md`, "Parallel sessions".
