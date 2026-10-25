# superfargoth

Game-agnostic open-world ARPG framework on Bevy. **GPL-3.0-or-later** with a
section 7 permission for GPU vendor runtimes (`LICENSE-EXCEPTION.md`; owner 2026-10-25).
Its first content pack is Bevyrrowind (`../bevyrrowind`, GPL), which drives
the roadmap: see `../bevyrrowind/docs/ROADMAP.md`.

## Start of every session

1. Read `../bevyrrowind/docs/ROADMAP.md` (short: owner decisions, current phase).
2. For structural work, read `docs/ARCHITECTURE.md` (vision, crate plan, build rules §8b);
   `docs/WISHLIST.md` when you need feature targets or performance numbers.
   `docs/DECISIONS.md` is append-only history: grep it, don't read it whole.
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
6. **Compile time is a constraint** (§8b): Bevy-free cores, thin Bevy plugins.
7. **Research first, and challenge our docs.** Before designing or building
   a system, research how the industry does it today: papers, GDC/SIGGRAPH
   talks, engine docs, and the current crate ecosystem (versions, Bevy
   compatibility, maintenance). Our specs and ARCHITECTURE.md are not
   gospel. If research shows a better way, update the spec and log the
   change in ARCHITECTURE.md §10 instead of following a stale doc.
   Prior projects (Godotwind on Godot, ECSwind) are anecdotes, not designs:
   Godot's node tree, GDScript and renderer quirks don't transfer to Bevy,
   so never copy their tiers or constants; calibrate from our own Bevy
   measurements.
8. **Reuse before building, in this order: Bevy, then crates, then our
   own.** First use what Bevy itself provides (check its docs, examples and
   source for the version we're on). Then maintained, license-compatible
   crates and tools (GPL-3.0-compatible: MIT, Apache-2.0, BSD, zlib, GPL-3.0,
   LGPL; MPL-2.0 only for unmodified dependencies, owner decision 2026-10-11;
   never GPL-2.0-only or AGPL; `deny.toml`). Write our
   own only when neither fits (wrong license, abandoned, stuck on an old
   Bevy with no fork path, or below our performance or quality bar), and
   say in the spec why each earlier step didn't.
   **Don't reinvent the wheel: this covers whole projects, not just
   crates.** Before starting a system, look for open-source games, engine
   ports and Bevy projects that already built it (e.g. Mudcrab, a Skyrim
   engine on Bevy, with Luau scripting). If one is building
   what we need and isn't on our critical path, wait for it and adopt or
   port it instead of racing it.
9. **Finish before starting.** Drive started tasks to verified and
   committed before opening new systems (`../bevyrrowind/CLAUDE.md` rule 10).

## Commands

```sh
cargo test --workspace --all-features
cargo clippy --workspace --all-features --all-targets
cargo deny check   # licences, advisories, duplicates (deny.toml; README "Dependency policy")
cargo hakari generate --diff && cargo hakari manage-deps --dry-run   # workspace-hack is current
```

**Adding a crate or a dependency** (`cargo install --locked cargo-hakari`): run
`cargo hakari generate && cargo hakari manage-deps` and commit `crates/workspace-hack` with the
manifests. The hack pins one feature set per dependency, so `cargo test -p A` and `-p B` share one
Bevy build. A Bevy-free crate goes in `final-excludes.workspace-members` of `.config/hakari.toml`
first: the hack carries Bevy, and cores must build cold in seconds (§8b rule 1).
