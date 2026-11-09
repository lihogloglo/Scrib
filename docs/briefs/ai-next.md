# NPC AI lane: status after AI5, and what blocks AI6

Written 2026-11-09 by the AI5 agent for the coordinator and the owner. Not a brief: AI6 can't start yet, so no AI6
brief was written. Spec: `superfargoth/docs/specs/ai.md` (design), `ai-as-built.md` (what each slice built and why).

## What AI1-AI5 left

| Slice | Built | Where |
|---|---|---|
| AI1 trace | `sf_ai::trace`: per-actor decision trace ring, JSON dump, `format_trace` | `ai-as-built.md` "AI1" |
| AI2 / AI2b HTN | typed `WorldState`, record-built method `Library`, planner with MTR and full backtracking (`SEARCH_TRIES` 4096) | "AI2" |
| AI3 overlay | `sf_ai_bevy`: `ai.*` console, `debug` gizmos, BRP handler `sf_ai/trace`; example `ai_overlay` (visual check AI-1, awaiting owner) | "AI3" |
| AI4 directives | `DirectiveStack`, `StackPolicy` / `ModernPolicy`, `DirectiveDone`, `Limit::{Until, For}`, `Serving` | "AI4" |
| AI5 saves | `sf/ai` v1 (`sf_ai::save`, `sf_save_bevy::sections::ai`); `AiDirectives` became a resource keyed by `ActorKey` | "AI5" |

Everything is headless and unit-tested; **nothing is in the game**: no NPC carries `AiActor`, the game app doesn't add
`AiPlugin`, the game doesn't register `sections::ai`, and nothing pushes or advances stacks (AI6/AI7). Nothing decides yet.

## Open owner questions

1. **Game clock consolidation.** Three clocks exist: `sf_sky::SkyClock` (saved as `sf/clock`, `f64` days),
   `sf_rpg::GameTime` (integer minutes; the AI's clock and the dialogue/journal's), `sf_effects::GameMinutes`.
   Nothing converts `SkyClock` to `GameTime` yet (the dialogue host passes `GameTime::default()`). Whichever wins,
   the AI tick's `now` must come from the saved clock and round down, or a load can freeze stack timers
   (`ai-as-built.md` "AI5" "Clock"). AI4/AI5 used `GameTime` and did not consolidate.
2. **`SEARCH_TRIES` on the Steam Deck.** 4096 is a guess; the planner's worst case is exponential with full
   backtracking. Set it once AIM measures the Floor tier (ai.md "Tiers and budget").
3. **Wiring `sf_ai_bevy` into the game** (`mw_devtools_bevy` or the game app): `AiPlugin`, `AiActor` from
   `CharacterPlacement` (placement or spawn id; never `Entity` bits: `mw_rpg_bevy::dialogue::key` falls back to
   them), the BRP line (`with_method_main(BRP_TRACE_METHOD, brp_trace_system)`), and
   `app.add_save_section(sections::ai(sf_ai_bevy::ai_save, sf_ai_bevy::ai_load))` in `mw_saves_bevy`.
   A game build and the owner's launch; no agent did it.

## What AI6 (move executors `MoveTo`, `Wait`, `Face` over `sf_nav`) is blocked on

ai.md "Build plan" order: "AI6 (needs NV7a, NV8, seamless SI3)". None of these is built (nav.md status: draft,
docs only; nothing of `sf_nav` exists).

| Needed | What | Spec location |
|---|---|---|
| NV7a fixed tick | NPC drive (path follower, ORCA, mover) in `FixedUpdate` + render interpolation; "NV7a lands before AI6" | `superfargoth/docs/specs/nav.md` "Build plan" |
| NV8 corridor | `PathCorridor` feeding `PathFollower`, repair after a push | nav.md "Build plan" |
| SI3 | `InWorldspace` + per-camera `ViewerWorldspace` (every nav query takes the actor's worldspace) | `superfargoth/docs/research/seamless-interiors-2026-11-08.md` "Build plan" (SI3 row; "SI3 before nav.md NV5 and ai.md AI6") |

Before those: nav.md order NV0 (the `navmesh-rs` spike, research) → NV1 → NV2 → NV3 → NV4 → SI3 → NV5 → NV6 → NV7a →
NV7b → NV8. So the shortest path to AI6 is the nav lane from NV0, with SI3 as a shared prerequisite (the risky
cross-system refactor). Later AI slices also wait on nav: AI10 on NV12, AI12 on NV11 and SI2.

## Notes for whoever resumes the AI lane

- AI7 (drives) needs no nav: it could run before AI6 if the owner wants the lane to keep moving (an ordering call,
  not decided here). It sets `Serving` and must consume `DirectiveDone` within the tick or poll `status(id)`
  (events aren't saved).
- `sf/ai` grows by version steps: freeze `ActorSave` in `sf_ai::save::legacy::v1`, add a typed step in
  `sections::ai`, bless `sf.ai.v2.json`, keep `sf.ai.v1.json` under the legacy shapes test.
- MA2 (Morrowind packages → directives) can skip saving stacks that nothing changed since they were built from
  records (base/delta), as long as they issued no ids beyond the record's.
