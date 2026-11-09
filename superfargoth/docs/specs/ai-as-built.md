# NPC AI as built (`sf_ai`): history moved out of `ai.md`

Moved here to keep [ai.md](ai.md) inside its size budget. Newest slice first; the module docs in
`superfargoth/crates/sf_ai/src/` are the detailed reference.

## AI4: directives (2026-11-09)

`sf_ai::directive` (Bevy-free; new deps `sf_rpg` for `GameTime`, `sf_world` for `Location`/`PlacementId`).
`DirectiveStack` per actor, top first, serde (what AI5 saves: entries, id sequence, clock, serving mode):
`push(spec, now, policy, events) -> DirectiveId`, `finish`, `cancel`, `cancel_where` (a quest stopping: by
`origin`), `serve(Serving)`, `advance(now)`; `status(id)` answers `done(id)` (`serving`/`waiting`/`ended`/`unknown`).
`DirectiveDone { actor, id, end: Completed | Expired | Cancelled, at }`, exactly one per issued id (a push the
policy refuses ends `Cancelled` at once). `StackPolicy { push, on_end }` edits only through `StackEdit` (`insert`,
`push_bottom`, `cancel`, `cancel_where`, `renew`), so ids and events stay exact. `ModernPolicy`: source rank
(record < schedule < memory < script < quest), then priority, newest first among equals (OpenMW's tie rule too);
`repeat` re-pushes a fresh instance on completion or `For` expiry, never on cancel or `Until`. `DirectiveKind`:
`StayNear`, `GoTo`, `Follow`, `Escort`, `Use`, `Fight`, `Custom(RecordId)`. `format_directives`; in
`sf_ai_bevy` the `AiDirectives(DirectiveStack)` component and `ai.directives [id]` (top first, `[serving]`).

Deviations from the illustrative data model, and why (research: OpenMW `aisequence.cpp`/`aiwander.cpp` read for
behaviour; Creation Kit "Package Stack"):
- **Two limits instead of `until`:** OpenMW packages count their remaining duration only while they execute
  (`AiWander` `mRemainingDuration`, game hours), so a quest on top or a fight pauses them; an absolute deadline
  can't express that. `Limit::Until(GameTime)` (schedule windows, served or not) and `Limit::For { minutes }`
  (served minutes, `Directive::served`).
- **`Serving` (`Top` | `Directive(id)` | `Suspended`):** what `For` counts and the drives work on. AI7's
  arbiter sets `Suspended` while a non-directive drive runs (Bethesda: combat suspends package evaluation; OpenMW:
  combat is a package on top) and may name a lower directive when the top can't be served (Bethesda runs the
  topmost package whose conditions hold). A named directive that leaves falls back to `Top`.
- **Exact on one clock:** every mutation takes `now`; `advance` ends expiries in time order at their exact
  deadline (ties top first), so one coarse step equals many fine ones (tested at 1-, 7- and 1000-minute steps):
  tier invariance. Integer game minutes (`GameTime`'s resolution); a clock reading from the past is ignored.
- **Ids are instances:** `DirectiveId` is a per-actor, never-reused sequence (not a load-time index); a repeat
  is a new instance, so `done(old)` holds after one cycle.
- **No `done` flag, no `ParamBag`:** completion is `finish` + the event + `status` (a removed directive's flag
  can't be polled; OpenMW's `mDone` is one sticky flag per sequence, which MA2 keeps from the events). Kinds carry
  their data; OpenMW's per-package options (`canCancel`, `shouldCancelPreviousAi`, ...) are per type, MA2's;
  `origin` names the issuing record. `MAX_REACTIONS` (256) bounds policy cascades and same-instant expiries.
- Not built: the systems that push/advance/finish stacks (AI6/AI7), the Bevy message for `DirectiveDone`,
  filling `TraceEntry::directive` (from `serving()`), `decide`.

## AI3: debug overlay, `sf_ai_bevy` (2026-11-09)

New thin Bevy crate `sf_ai_bevy` (bevy_app/bevy_ecs + `sf_console_bevy`; feature `debug` adds gizmos, camera,
colour, transform and the console overlay). `AiPlugin { timings }` inserts `AiTraces` (the `TraceStore`, tier-2
admission), `AiLibrary` (library + last `LoadReport`), `AiSelected`; `AiActor(ActorKey)` marks an entity the AI
runs. `load_library(world, db, keys, operators)` logs one `error` line per bad record (target `sf_ai`,
`HTN library: sf.ai.method `x`: unknown task `y``, then a count line) and keeps an empty library. Console:
`ai.select [hex|off]` (no id: the console's click selection, `debug`), `ai.trace [n] [hex]` (names through the
library, indices for unknown ids), `ai.tiers` (labelled provisional: last traced step per actor until AI9),
`ai.stats`; `ai.gizmos` (`debug`): ring at the selected actor's feet coloured by the last plan outcome and
`overlay_lines` above its head as Bevy 0.20 stroke-font `Gizmos::text`, facing the active camera. Tier colours,
paths, cones and claims are not drawn (no data until AI6/AI8/AI9/AI14). Enum names print through serde, so
variants `sf_ai` adds later need no change here. Owner check: example `ai_overlay` (AI-1, synthetic trace).

Deviations, and why:
- **BRP without `bevy_remote`:** one `RemotePlugin` per app (the game's `mw_devtools_bevy` owns it) and hakari keeps
  `bevy_remote` out of every other build, so the crate exports `brp_trace(world, params)` /
  `brp_trace_system` for `with_method_main(BRP_TRACE_METHOD = "sf_ai/trace", ..)`.
- **Actor keys as 32-hex strings in BRP**, not `TraceStore::to_json`'s numbers: a 128-bit placement id doesn't fit
  a `serde_json::Value` number or a JavaScript client. The console prints and parses the same hex as `PlacementId`.
- **Not wired into the game yet:** no NPC entity carries `AiActor` and the game app doesn't add `AiPlugin` (needs
  `mw_devtools_bevy`: plugin, `AiActor` from `CharacterPlacement`, the BRP line; a game build, not done here).

## AI2: HTN core (2026-11-09)

`sf_ai::{state, htn}`. `StateKeys` (code-declared, case-folded; same name + kind shares a key, a kind clash
is an error) types every key as `BitKey`/`IntKey`/`RefKey`; `WorldState` is three flat std arrays (no
`SmallVec`/bitset crate: the planner clones the state once per search and undoes effects through a journal).
Records `sf.ai.task` (`Compound` | `Primitive { operator, pre, effects }`) and `sf.ai.method`
(`{ task, priority, pre, subtasks: [Task(..) | Pause] }`) resolve once in `Library::build` against `StateKeys`
and `Operators`; every unknown task, key, operator, kind mismatch or method on a primitive is collected into one
`LoadReport`. Methods sort by priority, ties by record id (load-order independent); a method's rank is its MTR
digit. `decompose` / `resume` / `replan` take `Option<&mut TraceEntry>` (identical result with `None`).

Deviations from the spec's illustrative data model, and why:
- **Effects on primitive tasks, not methods** (Humphreys, Fluid, SHOP): a method effect has no defined time in a
  total-order plan with pause markers. Effects are planning-only (Fluid `PlanOnly`); executors write real state.
- **Full chronological backtracking** (owner 2026-11-09, slice AI2b; SHOP/pyhop depth-first, Humphreys'
  decomposition history, Decima's Prolog-style search; was Fluid's method-local in AI2): every method choice is a
  choice point; a task that can't plan reopens the most recent one with untried methods, even inside an earlier,
  fully planned sibling, restoring world state, plan and method stack through one undo journal. Trace: unwound
  methods `SubtaskFailed(t)`, a reopened planned choice `Backtracked(t)` (`back t` in `format_trace`), one entry
  each. First plan found = best MTR. Worst case exponential: `SEARCH_TRIES` (every backtrack costs a try) cuts it
  off deterministically, tier-invariantly (`Limit`). Opt-out: `MethodDef::cut` (Prolog cut at the method's end;
  `cut` everywhere = method-local). No nogood memo of failed (agenda, state) yet: AIM measures first.
- **MTR rule:** a replan wins only if lexicographically better on the common prefix (equal, or a prefix, is not
  better); the search stops at the first method that can't win (`RejectReason::NotBetter`, Fluid's
  `BeatsLastMTR`). The AI1 open item ("replan discarded") is `TraceEntry::outcome = kept_not_better`, plus
  `TraceEntry::mtr`; `methods`/`plan`/`mtr` describe the plan in force, `rejected` this step's search.
- A pause reached before any step is planned is skipped (nothing to run first). Paused methods are committed:
  `resume` failing means replan from the root. Guards: `SEARCH_DEPTH` 64, `SEARCH_TRIES` 4096
  (`RejectReason::Limit`). Plans record `reads: KeySet` (every key a precondition evaluated, rejected branches
  included) for AI6's dirty-key re-validation; `Library::holds` is the running operator's per-step check.
- Not built: callers set `trigger`; no scratch reuse yet (a search allocates a few small `Vec`s; AIM measures).

## AI1: decision trace (2026-11-08)

`sf_ai::{ids, trace}` (details in the module docs). Entries add a trigger, the served directive,
raw vs. final drive score and `SubtaskFailed` (backtrack) rejections. `TraceStore::record(actor, tick, tier) ->
Option<&mut TraceEntry>` is the off switch, so `decide` takes `Option<&mut TraceEntry>`; the ring recycles entries.
Cap: watched (newest first), then admission order; later actors never evict earlier ones. `budget_us` is `None` with
timings off (reproducible traces). `ai.trace` is registered in AI3; the core exposes `format_trace` and JSON dumps.
