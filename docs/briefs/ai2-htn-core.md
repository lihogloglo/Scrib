# Brief: slice AI2, the `sf_ai` HTN core (records, typed state, decomposition, partial plans, traced rejections)

Written 2026-11-08 by the AI1 agent for the next agent working on NPC AI. Paste everything below the line into a
fresh session. Compiling on the agent's machine is allowed for this lane (owner, 2026-11-08); the owner launches the
game, never the agent.

---

You are implementing slice **AI2** of the NPC AI spec in Scrib, a Rust/Bevy open-world RPG engine (repo root = your
working directory; framework `superfargoth/` = game-agnostic `sf_*` crates, Morrowind pack `bevyrrowind/` = `mw_*`
crates). One deliverable plus its test. Do not start AI3 (overlay) or AI4 (directives).

## Why this slice now

AI1 built the debugger first: `sf_ai::trace` records every decision step, including **each rejected method with its
failing precondition**. AI2 is the planner that fills it: the one shared, data-driven HTN method library every NPC
runs (ai.md "Behaviours"), total-order forward decomposition (Humphreys, Game AI Pro ch. 12; Fluid HTN; Decima).

## Read (only these; grep, don't read whole files)

1. `CONTRIBUTING.md` (short; the rules you work under).
2. `bevyrrowind/CLAUDE.md`: "Parallel sessions", "Hard rules"; `superfargoth/CLAUDE.md`: "Hard rules", "Commands".
3. `superfargoth/docs/specs/ai.md`: "Canonical technique", "Reuse", "Data model", "Behaviours", "Executors" (only the
   operator list), "Debugging" (incl. "As built (AI1)"), "Scheduling", "Tests", "Build plan".
4. `superfargoth/crates/sf_ai/src/` (all of it, ~1100 lines with tests): `ids.rs`, `trace.rs`, `tests.rs`. You extend this crate.
5. `superfargoth/crates/sf_records/src/lib.rs` (top docs + `register_serde`) and how `sf_stats` keys records
   (`sf_stats/src/table.rs`: content names resolved to indices at load, `TableError` for load reports). That is the
   house pattern for "names resolve to indices once at load".
6. `superfargoth/docs/research/npc-ai-2026-11-08.md`: grep "HTN", "Fluid", "MTR", "traversal" only.

Do NOT read the nav, seamless or adapter specs; nothing in AI2 needs them.

## Research of your own (required, bounded: a handful of searches/reads)

The owner wants every agent to check the plan. Before coding, look at: Fluid HTN's (MIT, github.com/ptrefall/fluid-hierarchical-task-network)
domain/planner split, partial planning ("PausePlan") and how it compares MTRs; Humphreys' chapter (MTR, plan
monitoring); Guerrilla's "HTN Planning in Decima" (backtracking, branch ordering); the current Rust HTN crates
(`htn`, `whim`, `bevy_bae`: the spec rejected them, recheck in one search that nothing new and Bevy-free appeared).
If code is ported from Fluid HTN, cite the file + commit in module docs and add a `superfargoth/THIRD_PARTY_NOTICES.md`
entry. Flaws you find: fix within AI2's scope if no owner decision is contradicted; otherwise raise them as questions.

## Binding owner decisions (never reverse; flag risks only)

ai.md "Owner decisions this spec implements (2026-11-08)" and `superfargoth/docs/research/ai-nav-decisions-2026-11-08.md`:
layering directives → utility → HTN → executors; `sf_nav` + `sf_ai` split; host-only fixed tick; tiers are a host CPU
setting; Nemesis-style memory; LLM later, out of process; GPL-3.0; no MP ceiling; Steam Deck floor. Delirium
(`mw_delirium`) stays core/default. Grep `superfargoth/docs/DECISIONS.md` if you touch anything overlapping.

## What AI1 left you (APIs and gotchas)

- Ids (`sf_ai::ids`): `TaskId(u32)`, `MethodId(u32)`, `OperatorId(u16)`, `DriveId(u16)`, `ConsiderationId(u16)`,
  `DirectiveId(u32)`; `Copy`, serde-transparent (bare numbers in JSON), `Display` as `t3`, `m7`, `o4`. **AI2 owns the
  task/method/operator registries** that map record names to these indices.
- Trace (`sf_ai::trace`): `TraceStore::record(actor, tick, tier) -> Option<&mut TraceEntry>` is the hook. It returns
  `None` when tracing is off or the actor is not traced, so **`decide` should take `Option<&mut TraceEntry>`** (not the
  spec's illustrative `&mut Trace`) and write nothing when `None`: tracing then costs one branch. The returned entry
  is recycled from the ring (buffers kept), so push into its `Vec`s, don't replace them.
- What AI2 fills: `methods` (chosen decomposition, root first, as `MethodChoice { task, method }`), `rejected` (every
  method tried and skipped, in order: `RejectReason::Precondition(i)` = index of the **first** false precondition in
  the method's `pre` list, checked in order; `RejectReason::SubtaskFailed(task)` = preconditions held, a subtask found
  no method, backtracked), `plan` (`steps` of `PlanStep { task, operator }`, `cursor`, `partial` for a pause marker),
  `trigger` (`Replan` when monitoring invalidated the plan). Leave `drives`, `drive` (AI7), `directive` (AI4), `path`
  (AI6) and `budget_us` (the shell, AI3) alone.
- If a field is missing (e.g. "replan discarded: new MTR not better" as an outcome, or the MTR itself as a rank list),
  add it to `TraceEntry` (and `TraceEntry::reset`), and update the two JSON goldens and the `format_trace` golden in
  `tests.rs`. Field order = declaration order = JSON order; keep new fields at the end of the struct.
- `format_trace(&Trace, n)` prints ids as indices; a name resolver (registries → names) is welcome if cheap, as a
  separate function so the existing golden stays.
- Console `ai.trace` is NOT registered yet (deferred to AI3: actor references and the console context are the
  shell's). Don't add a Bevy shell in AI2.
- Crate deps today: `serde`, `serde_json`, `sf_ids`. AI2 will need `sf_records` (records), probably `smallvec`
  (workspace, `serde` feature) for `WorldState`, maybe a bitset: prefer std or a crate already in `Cargo.lock`
  (`fixedbitset` is there via Bevy/petgraph; check `cargo tree -i`), and `thiserror` for load errors. Every new
  dependency: `cargo deny check` + hakari (below). `sf_ai` is already in hakari's `final-excludes`.
- **Workspace resolution gotcha (2026-11-09):** `origin/main` pins `delirium-physics` at rev `91ee2468...`, which no
  longer exists upstream, so any cargo command fails with "revision not found" on a cold git cache. Another session
  holds an uncommitted bump to `f4ff84d0...` (Delirium's lane; Delirium stays core). If it hasn't landed when you
  start, bump both `rev`s in the root `Cargo.toml` **locally only** to build, then restore them and commit a
  `Cargo.lock` with only your own changes (AI1 did this: `git show HEAD:Cargo.lock` plus the new package stanza).
  Never commit the delirium bump yourself; mention it in your report.
- Redirect `cargo_turn.sh` output to a file and grep it; a backgrounded tool call still dies at its `timeout`.
- **cargo-deny was not installed on the AI1 machine**; if `cargo deny` is missing, say so in your report rather than
  installing tooling unasked.
- Tests live in `src/tests.rs` (one module, `use crate::*`), like `sf_stats`. Golden strings are inline raw strings
  (no golden files: CRLF checkouts on Windows would break `include_str!` comparisons).

## Deliverable (Bevy-free, in `superfargoth/crates/sf_ai`)

- `state` module: `WorldState` as typed slots (bits, small ints, optional target refs), keys from a load-time
  `StateKeys` registry (name → index); unknown names fail at load with a report.
- `htn` module: `Method { task, priority, pre: Vec<Predicate>, subtasks: Vec<TaskRef>, effects: Vec<Effect> }`,
  compound/primitive tasks, a `Library` built from records (`sf.ai.method` and the task/operator vocabulary;
  predicates and effects from a registered vocabulary), `decompose(ws, root, lib, trace: Option<&mut TraceEntry>) ->
  Result<Plan, NoPlan>`: total-order forward decomposition, priority-ordered methods, effects applied to a copy of the
  state, backtracking, partial plans at a pause marker, the MTR recorded so a replan can be compared (ai.md
  "Behaviours": a replan wins only with a higher-priority MTR).
- Not in scope: drives, directives, executors' bodies, perception, tiers, Bevy, saves (plans are never saved).

## Wiring

- `sf_ai` is already a workspace member, a `[workspace.dependencies]` entry and in hakari `final-excludes`. Adding a
  dependency: run `cargo hakari generate && cargo hakari manage-deps` and commit `superfargoth/crates/workspace-hack`
  if it changes.

## Tests (exact; spec "Tests")

- Decomposition: a small synthetic library (3-4 compound tasks, priority-ordered methods) gives the exact plan for
  three world states; the trace entry lists the exact `methods` and `rejected` (with precondition indices and one
  `SubtaskFailed` backtrack).
- Partial plan: a pause marker stops decomposition with `partial: true`; continuing yields the rest exactly.
- MTR: a replan with an equal or lower-priority MTR is not adopted; a higher one is.
- Load: an unknown predicate/task name fails with a report naming it.
- `tracing off`: `decompose(.., None)` gives the same plan as with a trace entry.
- The AI1 tests still pass unchanged (or with goldens updated for fields you added, said in the commit).

## Rules that bite here

- Register in `.sessions/ai2-htn-core.md` (scope: `superfargoth/crates/sf_ai/`, root `Cargo.toml`/`Cargo.lock` only if
  a dependency is added, `superfargoth/crates/workspace-hack`). Read the other session files; stay out of their scope.
- Every cargo command through `bash bevyrrowind/tools/cargo_turn.sh <args>` from the repo root, one build at a time,
  output filtered to errors/warnings. Build only `-p sf_ai`; never the workspace, never the game.
- Before you report: `cargo test -p sf_ai`; `cargo clippy -p sf_ai --all-targets -- -D warnings` (pedantic via
  workspace lints); `cargo deny check`; `cargo hakari generate --diff && cargo hakari manage-deps --dry-run`.
- Commit only your paths (`git add <paths>`, `git commit -- <paths>`), message ending with
  `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`. Rebase on `origin/main` before pushing; never force-push.

## Docs at the end

- `superfargoth/docs/specs/ai.md`: an "As built (AI2)" note of a few lines under "Behaviours"; Status line. The spec
  is at its ~25 KB budget: move the AI1 note (and older as-built text) to `ai-as-built.md` next to it if it grows.
- `bevyrrowind/docs/ROADMAP.md`: mark AI2 only if an M7/3.10 NPC AI line exists (the shared-doc edits that create it
  are pending; don't create the lane). One line in `bevyrrowind/docs/CHANGELOG.md`.
- If `bevyrrowind/docs/test-pass-2026-11-10.md` exists, add an AI2 row (compiled and unit-tested; no visual check).
- Delete your `.sessions/` file. Write the AI3 brief (`docs/briefs/ai3-overlay.md`) in this format.

## Report back (≤ 250 words)

Files changed, the test list and results, clippy/deny/hakari status, research findings and plan flaws (fixed vs.
raised), anything in the spec that turned out wrong, commit hashes, and the AI3 brief path.
