# Brief: slice AI1, the `sf_ai` decision trace (debugger first)

Written 2026-11-08 by the coordinator session for the next agent working on NPC AI. Paste everything below the line
into a fresh session **on the owner's workstation** (it must compile). The owner launches the game, never the agent.

---

You are implementing slice **AI1** of the NPC AI spec in Scrib, a Rust/Bevy open-world RPG engine (repo root =
your working directory; framework `superfargoth/` = game-agnostic `sf_*` crates, Morrowind pack `bevyrrowind/` =
`mw_*` crates). One deliverable plus its test. Do not start AI2.

## Why this slice first

The industry rule (Unreal's Gameplay Debugger and Visual Logger, Guerrilla's in-game HTN view) and the owner's
approved spec agree: build the "why did this NPC do that?" trace before the planner, so every later slice
(HTN core AI2, directives AI4, drives AI7, perception AI8, tiers AI9) is debuggable from its first commit.

## Read (only these; grep, don't read whole files)

1. `CONTRIBUTING.md`: the whole file (short; the rules you work under).
2. `bevyrrowind/CLAUDE.md`: sections "Parallel sessions" and "Hard rules"; `superfargoth/CLAUDE.md`: "Hard rules"
   and "Commands" (adding a crate, hakari, the Bevy-free `final-excludes` list).
3. `superfargoth/docs/specs/ai.md`: "Architecture", "Data model", "Debugging (slice AI1 ...)", "Tests", "Build
   plan". The agent-summary at the top is the one-paragraph overview.
4. `superfargoth/docs/research/ai-nav-decisions-2026-11-08.md`: grep for "trace" and "AI1" only.
5. For the house style of a Bevy-free core crate: `superfargoth/crates/sf_stats/` (Cargo.toml, lib.rs layout,
   where tests live) and `superfargoth/crates/sf_console/` (a Bevy-free console core: see how commands register).
6. `superfargoth/crates/sf_ids/src/lib.rs` (`ActorKey` at line ~53, `PlayerKey`).

Do NOT read the full research docs, the seamless docs or the adapter spec; nothing in AI1 needs them.

## Deliverable

A new **Bevy-free** crate `superfargoth/crates/sf_ai` containing only what AI1 needs:

- `ids` module: small `Copy` + serde newtypes the trace refers to, and that AI2+ will reuse: `DriveId`, `TaskId`,
  `MethodId`, `OperatorId`, `DirectiveId` (u32 or u16 indices into load-time registries; document that names
  resolve to indices at load, spec "Data model"). Keep them minimal; AI2 owns their registries.
- `trace` module, per spec "Debugging":
  - `TraceEntry`: tick (`u64`), tier (`u8`), the drive scores (drive id + score, consideration breakdown as
    `(name index, value)` pairs), the chosen drive, the chosen method path, **each rejected method with the index of
    its first failing precondition** (this is the point of the debugger), the plan (task/operator ids), the
    current operator and its status, path state (an enum: none / pending / following / failed + reason), and the
    budget used (µs as `u32`).
  - `Trace`: a per-actor **ring buffer of 64 entries** (fixed capacity, no allocation per push after creation),
    `push`, `iter` (oldest → newest), `last`, `clear`.
  - `TraceStore`: `ActorKey` → `Trace`, with a cap on traced actors (default: all tier-2 actors; a "selected
    actor" set for full detail is fine), so tracing costs nothing for actors nobody watches. Tracing must be
    switchable off at zero cost (a `bool` check before building an entry).
  - JSON dump: `serde_json` output of one actor's trace and of the whole store, stable field order, for agents and
    for BRP later.
- Console: if `sf_console`'s core lets a Bevy-free crate register a command (check how others do it), add
  `ai.trace <actor>` printing the last N entries in a compact human form (one line per entry: tick, drive, method,
  first rejection). If registration needs the Bevy shell, **don't** create `sf_ai_bevy` (that's AI3): expose a
  `format_trace(&Trace, n) -> String` function and note in the handoff that AI3 wires the command.

Not in scope: HTN, drives, directives, any Bevy code, gizmos, BRP, saves (traces are never saved).

## Wiring (follow `superfargoth/CLAUDE.md` "Commands")

- Add `sf_ai` to the root workspace members and `[workspace.dependencies]` like `sf_stats`.
- Add it to `final-excludes.workspace-members` in `.config/hakari.toml` **before** running hakari (Bevy-free core).
- Run `cargo hakari generate && cargo hakari manage-deps`; commit `superfargoth/crates/workspace-hack` changes.
- Dependencies: `sf_ids`, `serde`, `serde_json`, `thiserror` if needed; nothing else. No Bevy, no Rapier.

## Tests (exact; spec "Tests")

- Ring buffer: push 70 entries, `iter` yields the last 64 in order; `last` is entry 70; `clear` empties.
- Rejections: a synthetic entry with two rejected methods round-trips through JSON with the precondition indices
  intact; the JSON for a fixed synthetic trace equals a checked-in golden string (exact).
- Off switch: with tracing disabled, `TraceStore` stays empty after a synthetic tick loop.
- Cap: tracing 3 actors with a cap of 2 keeps exactly the 2 chosen (document the eviction rule).
- `format_trace` output for the golden trace is exact.

## Rules that bite here

- Register your session in `.sessions/ai1-trace.md` (scope: `superfargoth/crates/sf_ai/`, the root `Cargo.toml`
  members and dependencies lines, `.config/hakari.toml`, `workspace-hack`). Read the other session files and stay out
  of their scope.
- Every cargo command goes through `bash bevyrrowind/tools/cargo_turn.sh <args>` (one build at a time). Filter the
  output to errors and warnings.
- Before you report: `cargo test -p sf_ai`, then clippy (pedantic, `-D warnings`) on `sf_ai`, `cargo deny check`
  and the hakari check (exact commands in `superfargoth/CLAUDE.md` "Commands").
- Never launch the game. Nothing in AI1 is visual, so no visual-check entry is needed.
- Commit only your paths (`git add <paths>` then `git commit -- <paths>`), on `main`, push when green. End the
  commit message with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## Docs at the end

- `superfargoth/docs/specs/ai.md`: add an "As built (AI1)" note of a few lines under "Debugging" (crate, modules,
  the eviction rule, where the console command lives). Keep the spec ≤ ~25 KB.
- `bevyrrowind/docs/ROADMAP.md`: if an AI/navigation lane (M7) exists, mark AI1 there; if not, add one "Recent"
  line only (the shared-doc edits that create the lane are pending, so don't create it yourself).
- One line in `bevyrrowind/docs/CHANGELOG.md`.
- Delete your `.sessions/` file.

## Report back (≤ 200 words)

Files and crate added, the test list and its results, clippy/deny/hakari status, where `ai.trace` lives (core or
deferred to AI3), anything in the spec that turned out wrong, and the commit hash.

## Next slices (for whoever comes after; not you)

- AI2 (HTN core: method records, typed world state, decomposition, partial plans, rejections written to this trace).
- NV0 (`navmesh-rs` spike; research + measurement, runs in parallel with AI1-AI3, brief from `nav.md` "Build plan").
