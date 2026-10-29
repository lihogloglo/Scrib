# Contributing to Scrib

Scrib is two parts in one repo and one Cargo workspace: `superfargoth/`, a game-agnostic open-world ARPG
framework on Bevy (`sf_*` crates), and `bevyrrowind/`, the Morrowind content pack and game (`mw_*` crates).
These are the working rules for everyone, people and coding agents alike (`AGENTS.md` and the `CLAUDE.md`
files point here). "The owner" in our docs is the project maintainer; owner decisions live in the ROADMAP
and `superfargoth/docs/DECISIONS.md`.

## Find your way

- `docs/SYSTEMS.md`: one row per system (specs and sections, crates, ROADMAP and visual-check IDs). Find
  your row and read only those docs.
- `bevyrrowind/docs/ROADMAP.md`: the current phase and owner decisions. Work comes from the current phase;
  parked `[p]` items wait for the owner's go.
- `superfargoth/docs/ARCHITECTURE.md` for structural work; the spec in `*/docs/specs/` for the system you
  touch. No spec yet for a major system? Write it first.
- Part rules and commands: `bevyrrowind/CLAUDE.md` and `superfargoth/CLAUDE.md`. Despite the name they
  apply to everyone.
- `bevyrrowind/docs/hard-problems.md` lists problems the owner handles separately; ask before taking one on.
- Morrowind's data is not included: point the game at your own install (`bevyrrowind/README.md`).
- Don't read these whole, grep them: `superfargoth/docs/DECISIONS.md` (or `DECISIONS-INDEX.md`),
  `bevyrrowind/docs/visual-checks.md`, `bevyrrowind/docs/perf.md`, any `docs/archive/`.

## How we decide what to build

1. **Research first, and challenge our docs.** Before designing or building a system, research how the
   industry does it today: papers, GDC/SIGGRAPH talks, engine docs, OpenMW and the modding community, and
   the current crate ecosystem (versions, Bevy compatibility, maintenance). Our specs and ARCHITECTURE.md
   are not gospel: if research shows a better way, update the spec and log the change in ARCHITECTURE.md §10.
2. **Reuse before building: Bevy, then crates, then our own.** First what Bevy itself provides (docs,
   examples and source of the version we're on), then maintained, licence-compatible crates, then our own
   code only when neither fits (wrong licence, abandoned, stuck on an old Bevy with no fork path, or below
   our performance or quality bar), with the spec saying why each earlier step didn't fit. This covers whole projects too: look for
   open-source games, engine ports and Bevy projects that already built it (e.g. Mudcrab; OpenMW for
   Morrowind behaviour). If one is building what we need off our critical path, wait and adopt or port it.
3. **Industry standard, never kludge.** Name the canonical technique first and check the system against
   what shipped engines do (UE, Unity, Bethesda, OpenMW, Decima, RAGE), not just against our spec. Specs
   carry an "Industry comparison" section; gaps go in `superfargoth/docs/specs/industry-audit.md`. On a
   perf problem, ask "what's the standard technique?" before inventing one. Temporary scaffolds are named,
   isolated and come with a removal path. Prior projects (Godotwind on Godot, ECSwind) are anecdotes, not
   designs: never copy their tiers or constants; calibrate from our own Bevy measurements.
4. **Patch Bevy, don't inject into it.** When Bevy falls short: use it as-is, then a vendored patch
   carrying upstream PRs (`superfargoth/vendor/`) or a small honest fork, never code that reaches into
   Bevy's private render-world state or ordering.
5. **Diagnose before optimizing.** If a feature looks broken, fix the visible failure before any perf work
   on it. Don't stack extra layers on a weak base; prefer something simpler that works.
6. **Finish before starting.** Drive started tasks (`[~]`) to verified and committed before opening new
   systems. Ask the owner (open an issue) before starting a new system or lane.
7. **Roadmap over polish; log bugs, don't chase them.** The next roadmap task beats tuning what exists.
   Regressions, leftovers and bugs found along the way go in the ROADMAP or visual-checks with whatever
   evidence you have, unless they block your task; the owner picks what gets fixed next.

## Design constraints

- **Tie-breaker:** runs as fast as possible, looks as good as possible. Baking is acceptable only as a
  one-time step under a few minutes; nothing bakes during play.
- **Frame target:** High preset, 1440p + DLSS Quality on an RTX 4060 laptop, ≤ 11 ms median in exteriors
  (stretch 7 ms). Raster must meet it; ray tracing (Solari) is an Ultra tier that may cost more. Perf work
  reports ms per preset.
- **Not a 1:1 port; modern first.** Morrowind content mechanics (MWScript, dialogue, magic effects,
  levelled lists, factions, crime) are ported from OpenMW into `bevyrrowind/` (cite file + commit). Feel
  systems (combat, movement, progression, AI, formulas, UI flow) are modern first, each behind a seam so a
  classic-rules version can come later; never build the classic side early. Framework systems stay
  format-agnostic: other games' formats must plug in too.
- **Combat and movement** aim for an Elden Ring-like third-person feel. Clips come from animation packs
  (free first) and Morrowind's own animations, never mocap; optional packs (e.g. LaFAN1) always need a
  fallback.
- **Net-ready:** small co-op (2-4 players, host-authoritative) comes later, so new specs use a fixed tick,
  input as commands, presentation that never feeds gameplay, and a remote player is just another character.
- **Everything is a mod:** content flows through `sf_vfs` / `sf_records` / `sf_mods`, no privileged paths.
- **Framework/game split:** nothing in `superfargoth/` names Morrowind (or any game's) formats, IDs or
  conventions; add an adapter hook instead.
- **Compile time is a constraint:** logic in Bevy-free crates, thin Bevy plugin crates
  (`superfargoth/docs/build-perf.md`).
- **Dependencies:** licence policy in `deny.toml` and `superfargoth/README.md` "Dependency policy". Crate
  upgrades happen in one batched "update all" pass (ROADMAP backlog), not one by one. Adding a crate needs
  `cargo hakari` (`superfargoth/CLAUDE.md` "Commands").

## Tests and visual checks

- Logic and data (saves, quests, scripts, parsers): exact assertions. Look and feel (physics, animation,
  rendering, UI look): sanity only (no NaN or explosion, nothing through the floor, comes to rest, no
  crash); tuned numbers are printed by `#[ignore = "measurement: ..."]` tests, never pass/fail limits.
- Rendering work isn't done until it ran and was seen. A few screenshots for sanity (at most ~3 iterations
  per issue), then register a test scene in `bevyrrowind/docs/visual-checks/<system>.md` plus its index
  line in `visual-checks.md` (status "awaiting owner"), saying concretely what to look for and when. The
  owner signs off visuals in batches.
- Changed animation slots or sources? Re-run `bevyrrowind bake` before checking in the live game: headless
  tests bake their own cache, the game reads the last bake.
- Before a PR: `cargo test --workspace`, clippy (pedantic, `-D warnings`), `cargo deny check` and the
  hakari check (`superfargoth/CLAUDE.md` "Commands").

## Docs

- End of a task: your task's status mark in the ROADMAP (edit only its line) and one line in
  `bevyrrowind/docs/CHANGELOG.md`. As-built detail goes in the spec and the commit message, never the
  roadmap. Decisions with lasting effect: one dated bullet (≤ 3 lines) appended to `DECISIONS.md`.
- Budgets (agents load these files, so size is context): ROADMAP ≤ ~250 lines, a spec ≤ ~25 KB (move
  "As built" history to an archive file next to it), a visual-check entry ≤ ~10 lines.

## Licence and provenance

GPL-3.0-or-later with a GPU vendor runtime permission (`LICENSE-EXCEPTION.md`). Ported code cites its
source file + commit in the module docs and gets a `superfargoth/THIRD_PARTY_NOTICES.md` entry; never
GPL-2.0-only, AGPL or non-free code. Proprietary SDKs (NVIDIA DLSS) are reached only through a separately
shipped plugin and are never committed. Other games' data follows the clean-room rule
(`superfargoth/CLAUDE.md` rule 3).

## Working with coding agents

- One deliverable plus its test per slice; if it needs "and", it's two slices. Research or diagnosis and
  building are separate slices; so are measurement sweeps and docs.
- A brief is prepared: exact files, functions and line ranges, the previous handoff, what's ruled out, how
  to verify. Never add scope to a running agent; queue a new slice.
- Read lean: grep first, read only the lines needed; filter cargo output to errors and warnings.
- Hand off at ~80-100k tokens of context: stop at a compiling state, write the handoff, and let a fresh
  agent continue.
- Use a strong model for anything that writes or fixes code; lighter models for research and docs only.
- Windows: don't round-trip text files through PowerShell 5.1 `Get-Content`/`Set-Content` (it reads UTF-8
  as ANSI and writes BOMs). Use your editor or `sed`; `grep -rlE "Â|â€"` finds the damage.
