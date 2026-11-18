# NPC AI as built (`sf_ai`): history moved out of `ai.md`

Moved here to keep [ai.md](ai.md) inside its size budget. Newest slice first; the module docs in
`superfargoth/crates/sf_ai/src/` are the detailed reference.

## AI6: move executors `MoveTo`, `Wait`, `Face` and HTN `PlanRunner` (2026-11-18)

`sf_ai::exec` (Bevy-free; `sf_coords` + `glam` in `[dependencies]`, 0 Bevy crates): built-in operator vocabulary
(`OP_MOVE_TO = "move_to"`, `OP_WAIT = "wait"`, `OP_FACE = "face"`, `register_move_operators -> MoveOperatorIds`),
`ExecGait`, `FaceTarget { Yaw, Point }`, `OperatorKind` / `MoveAction`, `ExecConfig`, `NavMoveState`, `ExecContext`,
`NavCommand`, `ExecOutput`, `OperatorExec`, and `PlanRunner` (`OperatorResolver`, `OperatorBindings`).
- **`MoveTo`**: commands `NavCommand::MoveTo { goal, gait, speed }`; succeeds (`OperatorStatus::Succeeded`,
  `PathState::None`) on `NavMoveState::Arrived` or initial proximity (`<= arrive_dist_m`); fails immediately with
  `PathState::Failed(PathFailure::NoPath)` on `NavMoveState::Failed`; tracks the actor's closest planar XZ distance to
  `goal.pos`, resetting `no_progress_s` whenever planar distance improves by `>= progress_min_m` (`0.25 m`) and failing
  with `PathState::Failed(PathFailure::NoProgress)` at `stuck_timeout_s` (`3.0 s`, ai.md "Executors").
- **`Wait`**: commands idle locomotion (`NavCommand::Stop`, optional `facing`, optional `idle_variant`) while counting
  down `duration_s` by `dt` each fixed tick; succeeds as soon as `remaining_s <= 0.0`.
- **`Face`**: computes desired yaw (`FaceTarget::Yaw` or `FaceTarget::Point` via `atan2(dx, dz)`), commands
  `NavCommand::Stop` with `facing = Some(desired_yaw)`, and succeeds once `wrap_angle(desired_yaw - ctx.yaw).abs() <=
  tolerance_rad`.
- **`PlanRunner`**: runs either direct/queued operators (`step_direct`) or an HTN root `TaskId` (`step_htn`), monitoring
  dirty read keys (`plan.reads.intersects(&dirty)`) via `htn::replan` (`Replaced` / `KeptNotBetter` / `KeptNoPlan`),
  per-step primitive task preconditions (`lib.holds(step.task, ws)` -> fresh `decompose` on `Trigger::Replan`), and
  partial plan continuations across `Pause` (`htn::resume` -> `PlanOutcome::Continued`, falling back to `decompose` on
  failure), recording `OperatorState` and `PathState` into `TraceEntry`.
- **Bevy shell (`sf_ai_bevy::exec`)**: `AiExecutor` component, `AiExecPlugin`, `AiSystems::Execute` in `FixedUpdate`
  inside `ControllerSystems::Drive` ordered `.before(NavSystems::Corridor)` (replies are already published: `NavSystems::Publish` runs earlier, in `FixedFirst`; the `.after(Publish)` in code is a no-op) (plus
  `apply_idle_intent` `.after(NavSystems::Corridor)` so `Wait`/`Face` `desired_facing` is preserved when
  `NavCorridor::goal` is `None`); live tunables `ai.exec.{stuck_timeout_s 3.0, progress_min_m 0.25,
  face_tolerance_rad 0.05}` (Guess, Dev, Net::Gameplay, Live). `NavQueryPlugin` and `NavCorridorPlugin` added in
  `mw_walkers_bevy::nav::add`. Tests: `sf_ai` `tests_exec.rs` (4), `sf_ai_bevy` `exec_tests.rs` (2 headless Recast room
  integration tests).

## AI7: drives, utility arbitration (2026-11-17)

`sf_ai::drive` (Bevy-free, no new deps). Records `sf.ai.consideration` (`ConsiderationDef { input, range: (at0,
at1), curve }`) and `sf.ai.drive` (`DriveDef { considerations, weight, momentum, commit_ticks, floor?, directive }`),
registered by `register_drive_kinds`; every tunable is a required record field (units and ranges in the type docs),
none has a code default. `Inputs` is the adapter's input vocabulary (`InputId`, case-folded, registration order);
`Drives::build(db, inputs)` resolves names once and collects every unknown input, unknown consideration, bad value
(non-finite or equal bookends, non-finite curve parameter, unsorted or empty piecewise table, negative weight,
momentum or floor) and second directive drive into one `DriveLoadReport` (`AI drives: N error(s)`, AI2 style).
Ids follow **record id order** (not load order), so the tie-break is stable across mod lists. `Curve`: `Polynomial`
(linear at exponent 1), `Logistic`, `Logit`, `Piecewise`; output clamped to 0..=1, NaN gives 0, and a NaN input
(or one past the end of the `inputs` slice) vetoes its drive. Score = IAUS compensated product
(`v + (1 - v)(1 - 1/n) v` per consideration) times the weight. `arbitrate(drives, inputs, prev, tick, trace) ->
Arbitration { drive, reason, state, scores }` is pure: floor lifts an applicable drive (raw > 0), the running drive
gets `x (1 + momentum)`, highest score wins, ties to the lowest id, all zero = idle; a running drive inside
`commit_ticks` holds while its raw > 0. Time is only the logical tick (a coarser tier switches on its first
decision past the same tick). `serving_for(drives, chosen, current)`: a reaction drive gives `Serving::Suspended`;
the directive drive or no drive serves the stack (`Top` after a suspension, a named `Directive(id)` kept).
Trace (AI1, minimally extended): `ConsiderationScore.input` (raw input next to the curve value), `TraceEntry.
drive_reason: Option<DriveReason>` (`best | tie | momentum | commit | floor | idle`, the first rule the winner
needed; skipped in JSON when `None`, so older dumps and `sf_ai_bevy`'s BRP golden keep their shape);
`format_trace` prints `drive d0 (momentum)`. Tests: `tests_drive.rs` (14) + the AI1 goldens updated.

Design, and why (Dave Mark's IAUS, GDC 2013/2015; Game AI Pro 3 ch. 13, Lewis; Guild Wars 2):
- **No per-consideration weight:** IAUS keeps every factor in 0..=1 so the product reads "how true are all of
  these"; a curve's slope and shift already scale it. The drive weight sets the bucket (e.g. 1, 2, 3, 5).
- **Floor only while applicable:** a directive drive whose considerations score 0 (no directive to serve) stays at 0,
  so a weak reaction can win then; with content, a reaction must beat the floor ("clearly beats it").
- **Commit yields to a dead drive:** a committed drive whose own raw score drops to 0 (threat gone) releases at once.
- **Saved by name:** `ArbiterState { running, since }` is serde, but a `DriveId` is a load-time index, so its save
  form is `SavedArbiter { drive: RecordId, since }` (`to_saved` / `resolve`; a drive whose mod is gone resolves to
  none). Not in `sf/ai` yet: that is a v2 step (freeze `ActorSave` in `legacy::v1`, bless `sf.ai.v2.json`).
- Not built: the Bevy system that fills inputs, arbitrates on the AI tick and calls `DirectiveStack::serve`; the
  adapter considerations (Hello, Fight, Flee, Alarm in `mw_ai`); a drive's HTN root task; scratch reuse for the
  `scores` `Vec` (AIM measures first); drive names in `ai.trace` (`TraceNames` has no drive hook yet).

## AI5: the `sf/ai` save section (2026-11-09)

`sf_ai::save`: `AI_SECTION` = `sf/ai`, `AI_VERSION` 1, `AiSave { actors: BTreeMap<ActorKey, ActorSave> }`,
`ActorSave { directives: SavedStack }`, `legacy` (empty: v1 is the first). `SavedStack` (in `directive.rs`) is the
stack minus its actor (the map keys it): entries with served minutes, `next_id`, clock, serving mode;
`DirectiveStack::{to_saved, from_saved, next_id}`. `DirectiveStack` keeps `Serialize` (dumps) but lost
`Deserialize`: the only way in is `from_saved`, which checks the invariants (`StackError`: an entry id the
sequence never issued, a duplicate id, serving an unissued id). `sf_save_bevy::sections::ai(get, set)`: order 350
(after overlay, clock, RPG; before the adapter), `check` refuses a broken stack naming the actor before anything
applies; shape golden `tests/shapes/sf.ai.v1.json`. `sf_ai_bevy`: `AiDirectives` is now a **resource** keyed by
`ActorKey` (was a component on the `AiActor` entity), `ai_save` / `ai_load` are the section's accessors (`None`
when empty, so a world without AI writes no section; load replaces the whole store). Not wired into the game
(no `AiPlugin` there yet). Tests: `sf_ai` `tests_save.rs` (3), `sf_save_bevy` `tests_ai.rs` (3) + the shape
golden, `sf_ai_bevy` accessor test; the four released save goldens now load with `sf/ai` wired.

Design, and why (research: `OpenMW` `esm3/aisequence.cpp` and `mwmechanics/aisequence.cpp` read for behaviour;
UESP Skyrim save format, ACHR change form `CHANGE_ACTOR_EXTRA_PACKAGE_DATA`; Unreal: behaviour-tree state is
transient, games save a high-level AI state and re-run the tree):
- **One world section, not per-placement overlay blobs** (save.md "One path for unload and save" was the
  alternative). Tier 0 keeps every actor logically alive in every worldspace, so the stack is host state that
  never unloads with its cell: one store keyed by stable id, no second copy for streaming to disagree with.
  Blobs would be frozen copies tier 0 can't advance, the stacks of actors with no entity after a load
  (`reload_resident` respawns cells after the apply) would need a holding place anyway, and the overlay's
  one-version-per-owner table refuses older blobs until overlay v2 (C7), while `sf/ai` will step versions often
  (AI10, AI11, AI13, AI14). Actors of a missing mod stay in the store and the section (save.md "Mod-list changes").
- **Saved = what can't be re-derived** (Bethesda's runtime package overrides and package data as change-form
  deltas; `OpenMW` each package's parameters and remaining duration): the stack with its id sequence, so
  `status(id)` answers `done(id)` after a load. `OpenMW` doesn't save `mDone` and loses it; events
  (`DirectiveDone`) aren't saved here either, so AI6/AI7 consumers must read them in the tick they're emitted
  or poll `status`. Not saved (rebuilt): plans, traces, awareness, path requests, wander-like executor state.
- **Base/delta, later:** every stack is saved, record defaults included (~40-60 B per actor before LZ4). When
  MA2 builds stacks from record packages, a stack nothing pushed to since it was built can be skipped and
  rebuilt (as stats' `changed_since_init`); it must not have issued ids beyond the record's, or `done(id)` breaks.
- **Clock:** nothing converts `SkyClock` to `GameTime` yet (the dialogue host passes `GameTime::default()`). Stacks
  ignore readings older than their clock, so a load that put the host clock behind a stack would freeze its
  timers. `sf/clock` and `sf/ai` are snapshotted at the same tick boundary, so this holds if the AI tick's `now`
  is derived from the saved clock and rounded down; which clock wins is the open owner question (three clocks).
  **Owned elsewhere (2026-11-09):** clock consolidation is ROADMAP 1.17, assigned to the mechanics lane; the spec
  lands at [clock.md](clock.md). The AI lane writes no clock conversion: AI4/AI5 directive timers stay blocked on
  1.17, and the AI tick's `now` must come from whatever 1.17 makes authoritative.
- **Keys:** placement and spawn ids are stable (spawn ids come from the saved overlay counter). Flagged:
  `mw_rpg_bevy::dialogue::key` falls back to `Entity` bits for an actor without a placement; such a key must never
  get a stack, or its saved state attaches to nobody (or someone else) after a load.

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
