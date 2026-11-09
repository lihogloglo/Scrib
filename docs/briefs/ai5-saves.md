# Brief: slice AI5, the `sf/ai` save section (round trip, old saves load)

Written 2026-11-09 by the AI4 agent for the next agent working on NPC AI. Paste everything below the line into a
fresh session. Compiling on the agent's machine is allowed for this lane (owner, 2026-11-08), in the agent's own
worktree with a private target dir; **the owner launches the game, never the agent.** AI5 is logic: it ends
compiled and unit-tested, no visual check.

---

You are implementing slice **AI5** of the NPC AI spec in Scrib, a Rust/Bevy open-world RPG engine (repo root = your
working directory; framework `superfargoth/` = game-agnostic `sf_*` crates, Morrowind pack `bevyrrowind/` = `mw_*`
crates). One deliverable plus its test. Do not start AI6 (move executors), AI7 (drives) or the adapter's MA2
(Morrowind packages → directives): AI5 makes what AI1-AI4 hold that must survive a save survive it.

## Why this slice now

AI4 built the per-actor `DirectiveStack` (the truth across tiers, ai.md "Directives"); nothing saves it yet. ai.md
"Saves": section `sf/ai` per actor (directive stack, schedule cursor, tier-0 state `(route, progress, until)`,
current activity and claim, drive and running directive id) and `sf/ai/memory` (AI13). Only the stack exists today:
AI5 saves it with a versioned shape that later slices extend by version steps, and proves old saves (without the
section) still load.

## Read (only these; grep, don't read whole files)

1. `CONTRIBUTING.md` (short). `bevyrrowind/CLAUDE.md`: "Parallel sessions", "Hard rules"; `superfargoth/CLAUDE.md`:
   "Hard rules", "Commands".
2. `superfargoth/docs/specs/ai.md`: "Owner decisions", "Directives (the §1b seam)" (+ its "As built (AI4)"),
   "Saves", "Tiers and budget" (tier 0 is logical: the stack must not care which tier saved it), "Build plan" AI5;
   `ai-as-built.md` "AI4".
3. `superfargoth/docs/specs/save.md`: "What is saved (section catalog)", "One path for unload and save",
   "Versioning and migration", "Mod-list changes", "Tests", the "As built" sections (they win on conflicts).
4. `superfargoth/crates/sf_ai/src/directive.rs` (module docs, `DirectiveStack` fields), `lib.rs`.
5. `superfargoth/crates/sf_save_bevy/src/sections/mod.rs` (how framework sections declare key, version, type and
   accessors) and one simple section's tests (grep `sf/rpg` / `RPG_SECTION` in `sf_save_bevy/src/tests*.rs`);
   `sf_rpg/src/service/purse.rs` top (`SERVICES_SECTION`: a section owned by a Bevy-free crate).
6. `superfargoth/crates/sf_ai_bevy/src/lib.rs`: `AiActor`, `AiDirectives` (the component that holds a stack).

## Research of your own (required, bounded: a handful of searches/reads)

The owner wants every agent to check the plan. Before coding: how OpenMW saves `AiSequence` (`aisequence.cpp`
`writeState`/`readState`, `esm3/aisequence.cpp`: which packages, remaining durations, the "last package" marker;
read for behaviour only, GPL, never port into `sf_*`), how Bethesda saves package state (changed-form flags for
`ACHR` AI data; UESP "Save File Format"), and how Unreal/other engines treat AI on save (usually rebuilt, not
saved: what is the minimum that must persist?). Then decide and document:

- **One `sf/ai` section for every actor vs. a per-placement overlay blob (`ai` system in `PlacementOverlay`).**
  save.md "One path for unload and save" says live state of NPCs leaving R0 goes into the overlay on unload, so
  streaming and saving can't disagree. A stack that lives only in an `sf/ai` section would be lost when its cell
  streams out unless something else keeps it. This is the main design question of the slice: check it against
  save.md and ai.md (tier 0 keeps actors logically alive off-screen, so maybe the stack never unloads). Fix
  within AI5 if no owner decision is contradicted; otherwise raise it.
- **The clock.** The stack's clock is `sf_rpg::GameTime` (integer game minutes; re-exported as `sf_ai::GameTime`),
  but the host's saved clock is `SkyClock` (`sf/clock`, `sf_sky`), and `sf_effects` has its own `GameMinutes`.
  Find how the host converts `SkyClock` to `GameTime` (or that nothing does yet) and make sure a load can't put
  the host clock before a stack's clock (the stack ignores readings from the past: its timers would freeze).
- Spawned actors (`ActorKey` from a spawn id) vs. placements: keys must be stable across a save (save.md, net rule).

## Binding owner decisions (never reverse; flag risks only)

ai.md "Owner decisions this spec implements (2026-11-08)" and `superfargoth/docs/research/ai-nav-decisions-2026-11-08.md`:
layering directives → utility → HTN → executors; host-only fixed tick; tiers are a host CPU setting, tier
invariance; vanilla NPCs get schedules behind a seam allowing "no schedules"; GPL-3.0; no MP ceiling; Steam Deck
floor. HTN failure handling is full backtracking (AI2b, owner 2026-11-09). Delirium (`mw_delirium`) stays
core/default. save.md's owner decisions (one-way upgrade, refuse newer versions, golden saves).

## What AI4 left you (APIs and gotchas)

- `DirectiveStack` is serde with **private** fields: `actor: ActorKey`, `entries: Vec<Directive>` (top first;
  `Directive { id, spec: DirectiveSpec, served }`), `next_id: u32` (the never-reused per-actor id sequence: it
  **must** be saved, or `done(id)` answers wrong after a load), `clock: GameTime`, `serving: Serving`. A postcard
  and JSON round trip is tested in `tests_directive.rs`; nothing froze the shape yet (no serde-reflection golden).
  Add the golden; if you need to reshape (e.g. drop the redundant `actor` when the section keys by actor), do it
  now, before version 1 is ever written.
- `DirectiveKind` holds `Location` (`WorldPos` serializes as grid in binary), `PlacementId` (hex in text formats,
  `u128` in binary), `ActorKey` (`u128`), `RecordId`, an `f32` radius. `Limit::{Until(GameTime), For { minutes }}`.
- Events (`DirectiveDone`) are not state: don't save them. Traces, plans, awareness, path requests are not saved
  (ai.md "Saves").
- `sf_ai` is Bevy-free and in hakari's final-excludes: the section key/version constants and any `legacy` types go
  in `sf_ai` (owner crates don't depend on `sf_save`); the `SaveSection` registration goes where the save
  sections of Bevy shells go (check `sf_save_bevy::sections` vs. the owning shell; `sf_ai_bevy` must not pull
  `sf_save_bevy` into builds that don't want it, so consider a feature or the game app).
- `AiDirectives(DirectiveStack)` is a component on the `AiActor` entity; nothing in the game spawns either yet
  (needs `mw_devtools_bevy`/the game: an owner/coordinator call). Your tests are headless.
- `cargo_turn.sh` waits for **any** cargo/rustc on the machine: run long builds in the background. A fresh
  worktree builds Bevy from scratch (tens of minutes on this laptop for `sf_ai_bevy` tests): budget for it, build only the
  crates you touch. Its exit code is always 0 (`cmd start /wait` drops cargo's): grep the output for `error`,
  `warning` and `test result`. Cargo reads sources when it reaches each crate, so don't edit files while a build
  of them runs (AI4's first verification run compiled half-written edits).
- `cargo-deny` is not installed on this machine; say so rather than installing tooling unasked.

## Deliverable

- `sf_ai`: the `sf/ai` save shape (key and version constants, the saved type: per actor at least the directive
  stack; reserved room for schedule cursor, tier-0 state, activity/claim, running drive and directive, which later
  slices add by version steps), serde-reflection golden of the shape.
- The `SaveSection` (or overlay blob, per your research) that writes every actor's stack and restores it onto the
  `AiActor` entities on load; actors present in the world but absent from the save keep their record defaults.
- No AI6/AI7 behaviour; no `mw_*` changes.

## Tests (exact; spec "Build plan" AI5: "round trip; old saves load")

- Round trip through the real section codec: a world with several actors' stacks (every `DirectiveKind`, both
  limits, a named `Serving`, ids with gaps) saves and loads byte-identically and behaves identically afterwards
  (same next id, same expiries).
- Old saves load: a save written without `sf/ai` (a golden from before AI5, or one built without the section)
  loads, and the actors get empty/record-default stacks, no error.
- A section from a newer version is refused naming the section and owner (save.md rule), if the registry doesn't
  already test that generically.
- Shape golden for the saved type. The AI1-AI4 tests still pass.

## Rules that bite here

- Register in `../.sessions/ai5-saves.md` (scope: the `sf_ai` save module + `lib.rs` re-exports, the section
  registration file you choose, tests). Stay out of `htn.rs`/`state.rs`; touch `directive.rs` only for shape
  changes the save needs (say so).
- Every cargo command through `bash bevyrrowind/tools/cargo_turn.sh <args>` from the repo root, one build at a
  time, output filtered to errors/warnings. Build only what you touch.
- Before you report: tests, clippy (pedantic, `-D warnings`) on what you touched; hakari check
  (`cargo hakari generate --diff && cargo hakari manage-deps --dry-run`) if a dependency changed.
- Commit only your paths (`git commit -- <paths>`), message ending with
  `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`. Rebase on `origin/main` before pushing; never force-push.

## Docs at the end

- `ai.md`: Status line; 2-3 lines "As built (AI5)" under "Saves", detail in `ai-as-built.md` (ai.md is at its
  ~26 KB budget: trim, don't grow). `save.md` catalog row for `sf/ai` (one line). One line in
  `bevyrrowind/docs/CHANGELOG.md`. A row in `bevyrrowind/docs/test-pass-2026-11-10.md` (or its successor).
- Delete your `.sessions/` file. Write the AI6 brief (`docs/briefs/ai6-move-executors.md`) in this format.

## Report back (≤ 250 words)

Files changed, test list and results, clippy/deny/hakari status, research findings and plan flaws (fixed vs.
raised), commit hashes, and the AI6 brief path.
