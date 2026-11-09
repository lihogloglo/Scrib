# Brief: slice AI3, the `sf_ai_bevy` debug overlay (gizmos, `ai.trace`/`ai.select`/`ai.tiers`, BRP)

Written 2026-11-09 by the AI2 agent for the next agent working on NPC AI. Paste everything below the line into a
fresh session. Compiling on the agent's machine is allowed for this lane (owner, 2026-11-08); **the owner launches
the game, never the agent.** This slice is **visual**: it ends with `AI-` visual-check entries "awaiting owner" and a
ledger row, never with an agent judging pixels.

---

You are implementing slice **AI3** of the NPC AI spec in Scrib, a Rust/Bevy open-world RPG engine (repo root = your
working directory; framework `superfargoth/` = game-agnostic `sf_*` crates, Morrowind pack `bevyrrowind/` = `mw_*`
crates). One deliverable plus its test. Do not start AI4 (directives) or anything that runs NPCs.

## Why this slice now

AI1 built the decision trace, AI2 the HTN planner that fills it (`sf_ai`, Bevy-free). AI3 is the first Bevy shell
crate, `sf_ai_bevy`, and makes the trace usable in game before any behaviour exists (industry: build the debugger
first; UE Gameplay Debugger, Guerrilla's in-game HTN view): resources holding the `TraceStore` and the `Library`, the
console commands, gizmos and a BRP method, so every later slice (AI4 directives, AI6 executors, AI7 drives) can be
watched the day it lands.

## Read (only these; grep, don't read whole files)

1. `CONTRIBUTING.md` (short). `bevyrrowind/CLAUDE.md`: "Parallel sessions", "Hard rules" (rule 5: the owner's eyes
   only); `superfargoth/CLAUDE.md`: "Hard rules", "Commands".
2. `superfargoth/docs/specs/ai.md`: "Architecture", "Debugging", "Tiers and budget" (only what `ai.tiers` prints),
   "Build plan" row AI3; `ai-as-built.md` (AI1 and AI2 as built).
3. `superfargoth/crates/sf_ai/src/`: `lib.rs` (re-exports), `trace.rs` (`TraceStore`, `format_trace_with`,
   `TraceNames`), `htn.rs` top docs + `Library` (it implements `TraceNames`). Don't re-read the planner internals.
4. `superfargoth/docs/specs/console.md` "Design" + "As built", and how an existing `*_bevy` crate registers console
   commands (grep `register` in `sf_console_bevy` and one user of it). ROADMAP 1.13 (reference picking: the owner
   wants a mouse-picked reference the console acts on; `ai.select` should reuse it if it exists, else take an id).
5. `superfargoth/docs/build-perf.md` rule 1 (thin Bevy crates) and how a recent `*_bevy` crate is wired (workspace
   member, `[workspace.dependencies]`, hakari: a Bevy crate is **not** in `final-excludes`).
6. Gizmos and BRP in Bevy 0.20: the Bevy source/examples of the version in `Cargo.lock` (`bevy_gizmos`,
   `bevy_remote`); grep how other `sf_*_bevy` crates gate debug-only code behind a `debug` feature.
7. `bevyrrowind/docs/visual-checks.md` (grep the index format) and one `visual-checks/<system>.md` entry as a model.
   If `bevyrrowind/docs/test-pass-2026-11-10.md` exists, its row format.

## Research of your own (required, bounded: a handful of searches/reads)

The owner wants every agent to check the plan. Before coding: UE's Gameplay Debugger and Visual Logger (what they
draw per actor, how selection works, the category toggles), Bevy 0.20 `bevy_remote` (custom methods, how other
projects expose game state over BRP), and whether a maintained Bevy-side AI debug crate exists that fits (Bevy 0.20,
licence policy in `deny.toml`) before writing gizmo code. Flaws you find: fix within AI3's scope if no owner decision
is contradicted; otherwise raise them as questions.

## Binding owner decisions (never reverse; flag risks only)

ai.md "Owner decisions this spec implements (2026-11-08)" and `superfargoth/docs/research/ai-nav-decisions-2026-11-08.md`:
layering directives → utility → HTN → executors; `sf_nav` + `sf_ai` split; host-only fixed tick (the overlay only
reads; presentation never feeds gameplay); tiers are a host CPU setting; GPL-3.0; no MP ceiling; Steam Deck floor.
Delirium (`mw_delirium`) stays core/default. No automated visual checks, ever (owner 2026-11-01).

## What AI2 left you (APIs and gotchas)

- `sf_ai` (Bevy-free) exports: `TraceStore` (`record`, `wants`, `watch`/`unwatch`, `set_enabled`, `set_timings`,
  `set_admit`, `to_json`), `Trace`, `TraceEntry` (new at the end: `mtr: Vec<u16>`, `outcome: Option<PlanOutcome>`),
  `format_trace(&Trace, n)` (ids as indices) and `format_trace_with(&Trace, n, &dyn TraceNames)`; pass the
  `Library` to print task/method/operator **names**. Line format: see `format_trace`'s doc comment (`plan <outcome>`
  only when the step planned).
- `htn::Library` is built once at load: `register_kinds(&mut RecordDb)`, then `Library::build(&db, &StateKeys,
  &Operators) -> Result<Library, LoadReport>`. `LoadReport`'s `Display` lists every bad name; the shell should log it
  at load (one line per error) and keep running with an empty library rather than panic. Who declares `StateKeys`
  and `Operators` is open until AI6/AI8 exist: an empty vocabulary is fine for AI3.
- Nothing produces trace entries in game yet (no NPC calls `decompose` until AI4/AI6/AI7). So the headless smoke
  test must push synthetic entries (`TraceStore::push`) and check the console/BRP output text exactly; the visual
  check can only show the overlay's empty/synthetic states. Say so in the visual-check entry.
- `ActorKey` is `sf_ids::ActorKey(u128)` (placement or spawn id, never `Entity`); the shell maps the selected
  entity to its key. Tracing must cost one branch for untraced actors (`TraceStore::wants`).
- The workspace `Cargo.toml` delirium pin (`91ee2468...`) no longer resolves upstream on a cold cache. If it hasn't
  been re-pinned on `origin/main`, bump both `rev`s **locally only** to `f4ff84d0df9e8708da3c5fc6ffff8856137c8f9d`
  to build, restore them before committing, and commit a `Cargo.lock` with only your own changes (AI1/AI2 did this).
  Never commit a Delirium pin change.
- `cargo_turn.sh` waits for **any** cargo/rustc on the machine (other sessions build big Bevy crates): budget for
  waiting, run long builds in the background and wait for the notification, not a sleep loop.
- `cargo-deny` is not installed on this machine; say so rather than installing tooling unasked.

## Deliverable (`superfargoth/crates/sf_ai_bevy`, new thin Bevy crate)

- Resources: the `TraceStore` (enabled, Tier-2 admission by default, timings per the host) and the `Library`
  (+ its load report), filled from `RecordDb` at load.
- Console commands: `ai.trace [n]` (selected or given actor, `format_trace_with` names), `ai.select <ref|id>` (watch
  the actor; reuse 1.13 picking if built), `ai.tiers` (counts per tier; tiers don't exist until AI9, so print what
  the store knows, labelled provisional), `ai.stats` optional (traced count, cap). `ai.facts` waits for AI13.
- Gizmos behind feature `debug`: for the selected actor, its plan text and last outcome; tier colours and paths
  wait for AI6/AI9 (draw nothing for missing data, never fake it).
- BRP: one method returning `TraceStore::to_json` (or one actor's trace).

## Tests (exact; spec "Build plan" AI3: "headless smoke; log line")

- Headless `App` with `MinimalPlugins` + the plugin: push synthetic entries, run `ai.trace`, assert the exact line.
- Load-report logging: a bad method record produces the exact log line naming it; the app keeps running.
- BRP method returns the exact JSON for the synthetic store.
- The AI1/AI2 `sf_ai` tests still pass.

## Visual check and ledger (the owner launches, never you)

- Add `AI-1` (and more if needed) to a `bevyrrowind/docs/visual-checks/<system>.md` (create an `ai.md` system file
  if none fits) with its index line in `visual-checks.md`, status "awaiting owner": exact command line, camera or
  scene, flags, what to type (`ai.select`, `ai.trace`), and concretely what to look for. Entry ≤ ~10 lines.
- If `bevyrrowind/docs/test-pass-2026-11-10.md` exists, add an AI3 row (built + headless-tested; visual awaiting).
- Ask the owner to look; don't claim it works visually.

## Rules that bite here

- Register in `.sessions/ai3-overlay.md` (scope: `superfargoth/crates/sf_ai_bevy/`, root `Cargo.toml`/`Cargo.lock`
  for the new member and deps, `superfargoth/crates/workspace-hack`, the visual-check files). Stay out of other
  sessions' scopes; `sf_ai` itself only if a small API gap blocks you (say so in the commit).
- Every cargo command through `bash bevyrrowind/tools/cargo_turn.sh <args>` from the repo root, one build at a time,
  output filtered to errors/warnings. Build only `-p sf_ai_bevy -p sf_ai`; never the workspace, never run the game.
- Before you report: `cargo test -p sf_ai_bevy -p sf_ai`; clippy (pedantic, `-D warnings`) on both; hakari
  (`cargo hakari generate && cargo hakari manage-deps`, commit the workspace-hack if it changes; `--diff`/`--dry-run`
  clean afterwards); `cargo deny check` if available.
- Commit only your paths (`git commit -- <paths>`), message ending with
  `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`. Rebase on `origin/main` before pushing; never force-push.

## Docs at the end

- `ai.md`: Status line; a 2-4 line "As built (AI3)" under "Debugging", detail in `ai-as-built.md` (ai.md is at its
  26 KB budget: trim, don't grow). `docs/SYSTEMS.md` AI row only if you're sure of the format (it still says "no
  spec yet"; point it at ai.md). One line in `bevyrrowind/docs/CHANGELOG.md`; ROADMAP only if an M7 NPC AI line
  exists (don't create the lane).
- Delete your `.sessions/` file. Write the AI4 brief (`docs/briefs/ai4-directives.md`) in this format.

## Report back (≤ 250 words)

Files changed, test list and results, clippy/deny/hakari status, research findings and plan flaws (fixed vs.
raised), the visual-check IDs awaiting the owner, commit hashes, and the AI4 brief path.
