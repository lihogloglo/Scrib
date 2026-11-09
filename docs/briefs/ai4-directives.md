# Brief: slice AI4, directives (stack, `StackPolicy`, completion events) in `sf_ai`

Written 2026-11-09 by the AI3 agent for the next agent working on NPC AI. Paste everything below the line into a
fresh session. Compiling on the agent's machine is allowed for this lane (owner, 2026-11-08); **the owner launches
the game, never the agent.** AI4 is logic: it ends compiled and unit-tested, with no visual check unless you add an
overlay line (then an `AI-` entry "awaiting owner", never a pixel judgement of your own).

---

You are implementing slice **AI4** of the NPC AI spec in Scrib, a Rust/Bevy open-world RPG engine (repo root = your
working directory; framework `superfargoth/` = game-agnostic `sf_*` crates, Morrowind pack `bevyrrowind/` = `mw_*`
crates). One deliverable plus its test. Do not start AI5 (saves), AI6 (executors) or the adapter's MA2 (Morrowind
packages → directives, OpenMW `StackPolicy`): AI4 is the framework seam MA2 plugs into.

## Why this slice now

AI1 (trace), AI2 (HTN core; AI2b: full backtracking, owner 2026-11-09) and AI3 (`sf_ai_bevy` debug overlay) exist,
but nothing tells an NPC *what* to do. Directives are the top layer (ai.md "Architecture": directives → drives →
HTN → executors): content says what (record packages, schedules, scripts, quests), AI decides how. AI4 builds the
per-actor directive stack, the adapter policy seam and the completion signal scripts poll, Bevy-free and exact.

## Read (only these; grep, don't read whole files)

1. `CONTRIBUTING.md` (short). `bevyrrowind/CLAUDE.md`: "Parallel sessions", "Hard rules"; `superfargoth/CLAUDE.md`:
   "Hard rules", "Commands".
2. `superfargoth/docs/specs/ai.md`: "Owner decisions", "Architecture", "Data model" (`Directive`, `DirectiveKind`,
   `StackPolicy`: illustrative), "Directives (the §1b seam)", "Scheduling" (events jump the queue), "Saves" (the
   stack is saved in AI5: keep it serde), "Build plan" row AI4; `ai-as-built.md` (AI1-AI3).
3. `superfargoth/crates/sf_ai/src/`: `lib.rs`, `ids.rs` (`DirectiveId` exists, unused so far), `trace.rs`
   (`TraceEntry::directive`, `Trigger::Event`), `htn.rs` top docs only.
4. `bevyrrowind/docs/specs/ai-adapter.md` "Packages → directives" and "Build plan" MA2: what the Morrowind policy
   will need from your trait (insertion, cancel, repeat; `GetAIPackageDone`). Don't implement it.
5. `superfargoth/docs/ARCHITECTURE.md` §1b (modern rules first, classic behind a seam).
6. `superfargoth/crates/sf_ai_bevy/src/{lib.rs,console.rs}` only if you add a console line (below).

## Research of your own (required, bounded: a handful of searches/reads)

The owner wants every agent to check the plan. Before coding: how Bethesda's AI packages stack and complete
(Creation Kit package stacks, `EvaluatePackage`), OpenMW's `AiSequence` (`apps/openmw/mwmechanics/aisequence.cpp`:
`stack`, `clear`, repeat; read for the *seam's shape* only, the port is MA2's), Unreal's StateTree/Smart Object
"requests", and how Fluid HTN or other HTN games feed goals into the planner. Check whether the illustrative
`Directive` fields (`until: GameTime`, `params: ParamBag`, `Location`) have homes yet (`sf_world::Location`?, a game
clock type?) and don't invent a parallel one. Flaws you find: fix within AI4 if no owner decision is contradicted;
otherwise raise them as questions.

## Binding owner decisions (never reverse; flag risks only)

ai.md "Owner decisions this spec implements (2026-11-08)" and `superfargoth/docs/research/ai-nav-decisions-2026-11-08.md`:
layering directives → utility → HTN → executors; `sf_nav` + `sf_ai` split; host-only fixed tick; tiers are a host
CPU setting, tier invariance (a directive's completion follows one logical clock, never the tier); vanilla NPCs get
schedules behind a seam allowing "no schedules" (default `ScheduleRules::None` until AI10-AI12); GPL-3.0; no MP
ceiling; Steam Deck floor. Delirium (`mw_delirium`) stays core/default. HTN failure handling is full backtracking
(owner 2026-11-09, AI2b): don't touch the planner.

## What AI3 left you (APIs and gotchas)

- `sf_ai_bevy` (thin Bevy shell): `AiPlugin`, resources `AiTraces` (`TraceStore`), `AiLibrary`, `AiSelected`,
  component `AiActor(ActorKey)`, console `ai.select`/`ai.trace`/`ai.tiers`/`ai.stats`/`ai.gizmos`, BRP handler
  `brp_trace` (`sf_ai/trace`). Fill `TraceEntry::directive` when a step serves a directive: `ai.trace` and the
  overlay don't print it yet; adding ` dir <name>` to the overlay (`overlay_lines`) is in scope if you want the
  directive visible (then an `AI-2` check), and an `ai.directives` console command (selected actor's stack, top
  first) is the natural debugger for this slice. Keep the console/overlay bits in `sf_ai_bevy`, the logic in `sf_ai`.
- AI2b (5060d65) added `RejectReason::Backtracked(TaskId)` and `MethodDef`/`Method::cut` (default false).
- Print enum names through serde (`sf_ai_bevy::snake_name` pattern), never exhaustive matches in the shell: `sf_ai`
  enums grow.
- Not wired into the game: the game app doesn't add `AiPlugin` and no NPC has `AiActor` (needs a `mw_devtools_bevy`
  change and a game build: an owner/coordinator call, not yours unless the brief-giver says so).
- Ids: `DirectiveId` is a load-time index; directive *instances* on a stack need a stable per-actor id for
  completion and saves (AI5): decide and document which (record id + sequence?).
- `ActorKey` is `sf_ids::ActorKey(u128)`; actor keys print as 32 hex in the shell.
- Delirium pin: if root `Cargo.toml` still pins `delirium-physics` at `91ee2468...` (gone upstream), bump both `rev`s
  **locally only** to `f4ff84d0df9e8708da3c5fc6ffff8856137c8f9d` to build and restore them before committing (and
  the `Cargo.lock` lines). Never commit a Delirium pin change.
- `cargo_turn.sh` waits for **any** cargo/rustc on the machine (other sessions build big Bevy crates): run long
  builds in the background and wait for the notification. Shared cache: `CARGO_TARGET_DIR=<repo>/target`.
- `cargo-deny` is not installed on this machine; say so rather than installing tooling unasked.

## Deliverable (`superfargoth/crates/sf_ai`, Bevy-free; plus optional debugger lines in `sf_ai_bevy`)

- `directive` module: `Directive` (kind, priority, repeat, until, source, done; serde), the per-actor stack, the
  `StackPolicy` trait (push, on_done; plus whatever the research shows cancel needs) with a **modern default
  policy** (priority order, newest first among equals, `repeat` re-pushes) and a test double; completion events
  (`DirectiveDone { actor, id }` as plain data the shell turns into a Bevy message later).
- `DirectiveKind` vocabulary: only what has a home today; `Custom(RecordId)` for the rest. No executors.
- `decide` stays a stub or takes the stack's top as its input; no drives (AI7).

## Tests (exact; spec "Build plan" AI4: "exact orders, policy double")

- Pushes from every source in mixed order give the exact stack order under the default policy.
- A policy double (records calls, custom order) is honoured: stack and events exact.
- Completion: `done` pops (or re-queues `repeat`), emits the exact event; `until` expiry on the logical clock.
- Serde round trip of a stack (AI5 saves it). The AI1-AI3 tests still pass.

## Rules that bite here

- Register in `.sessions/ai4-directives.md` (scope: `superfargoth/crates/sf_ai/src/directive*.rs` + `lib.rs`
  re-exports, optionally `sf_ai_bevy` console/overlay lines). Stay out of `htn.rs`/`state.rs` (AI2b's).
- Every cargo command through `bash bevyrrowind/tools/cargo_turn.sh <args>` from the repo root, one build at a time,
  output filtered to errors/warnings. Build only `-p sf_ai` (and `-p sf_ai_bevy` if you touch it).
- Before you report: tests, clippy (pedantic, `-D warnings`) on what you touched; hakari check if a dependency
  changed (`cargo hakari generate --diff && cargo hakari manage-deps --dry-run`).
- Commit only your paths (`git commit -- <paths>`), message ending with
  `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`. Rebase on `origin/main` before pushing; never force-push.

## Docs at the end

- `ai.md`: Status line; 2-4 lines "As built (AI4)" under "Directives", detail in `ai-as-built.md` (ai.md is at its
  ~26 KB budget: trim, don't grow). One line in `bevyrrowind/docs/CHANGELOG.md`. If
  `bevyrrowind/docs/test-pass-2026-11-10.md` (or its successor) exists, add an AI4 row (compiled + unit-tested).
- Delete your `.sessions/` file. Write the AI5 brief (`docs/briefs/ai5-saves.md`) in this format.

## Report back (≤ 250 words)

Files changed, test list and results, clippy/deny/hakari status, research findings and plan flaws (fixed vs.
raised), commit hashes, and the AI5 brief path.
