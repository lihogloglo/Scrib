# Session: coord-s3b (started 2026-11-10)

**Session id:** `scrib-4f` — the *same* session as `coord-static-render.md` (new context window, so that file is my own handoff, not another lane's). No other Scrib session is live: this session owns the checkout and the build slot.

**Role:** coordinator for **S3b — `sf_scene_bevy`**, the next and last unbuilt half of the static
scene (`superfargoth/docs/specs/static-scene.md`, hard problem H1). Takes over from `coord-static-render`
(scrib-4f), whose handoff is the "Next, in order" list in `.sessions/coord-static-render.md`.
Does not write production code itself: briefs, dispatches and reviews agents, merges only what compiles.

## Machine and standing facts (confirmed this session)

- **ThinkPad X13** (Ryzen 5 PRO 4650U, 6c/12t, 15.2 GB, integrated Radeon, model 20UGS10W09).
  Rust 1.98.1. **The game and its data are not on this machine and cannot be launched.** No visual
  judgement by any agent, anywhere.
- `repo/target` is **18 GB and still warm** (4022 fingerprints, vendored `bevy_pbr` rlib from
  2026-11-10 03:13), and C: has **111 GB free** — so the 2026-11-09 constraint ("~40 GB free, no
  second target dir") no longer binds. Incremental Bevy builds are viable; each still costs 20-40 min.
- **Owner decisions, this session:** (1) compile here as we go — `main` only gets code that compiles;
  (2) the 2026-11-10 test pass is **not** happening today, so S3b proceeds on the plan's default
  batch granularity (`PerClass`) and the S0 baseline stays an open owner run; (3) extra scope
  approved beyond S3b: the static-scene spec pass, the S1c format follow-ups, and the
  `prepare_preprocess_bind_groups` cache.
- One build at a time, always `-p <crate>`, never `--workspace`, always through
  `bash bevyrrowind/tools/cargo_turn.sh` — which **always exits 0**: judge every run by its output.
- `repo/`'s working tree is dirty with the mechanics lane's uncommitted work (`mw_rpg`, `mw_script`,
  `mw_stats`, `sf_effects`, `superfargoth/docs/specs/clock.md`), preserved on
  `origin/backup/mechanics-wip-2026-11-09`. Nobody touches it.

## State on arrival

`main` = `origin/main` = `e3112be`. Everything static-scene is merged: S0, S1a-S1c, S2, the sun shadow
early-out, **patch GS complete** (slices A, B-i, B-ii), and **`sf_scene` core A1-A6** (format v3,
`ChainRegistry`, `ClassAllocator`, `SceneSpace`/`batch_ranges`, the `scene.chains` bake step). Every
other branch is a stale tip whose content is in `main` or superseded. Nothing has ever been baked:
`bake_scene` is still not called (step C1).

**What is left of S3b:** steps **B1-B6 + C1/C2** of `superfargoth/docs/specs/scene-runtime-plan.md`
§10 — the `sf_scene_bevy` crate, which does not exist yet.

## Agents dispatched (2026-11-10)

| Agent | Slice | Branch | Writes |
|---|---|---|---|
| R1 | B1/B2 readiness: verify plan §5/§6 against the code, write the brief | `static-scene/brief-b1-b2` | `docs/briefs/scene-bevy-b1-b2.md` |
| R2 | B3/B4 readiness: verify the GS port's main-world API and A5's as-built API, write the brief | `static-scene/brief-b3-b4` | `docs/briefs/scene-bevy-b3-b4.md` |
| D1 | Static-scene spec pass: the 6 requested corrections, §5's wrong third axis, decision 8's restructure | `static-scene/spec-pass-2026-11-10` | `specs/static-scene.md`, `specs/archive/static-scene-history.md`, ROADMAP 1.16, SYSTEMS row |
| P1 | `prepare_preprocess_bind_groups` research + work order | `perf/preprocess-bind-groups` | `research/preprocess-bind-groups-2026-11-10.md`, `docs/briefs/perf-bind-group-cache.md` |

None of them runs cargo: this session owns the build slot. `scene-runtime-plan.md`, `gs-patch-plan.md`,
`DECISIONS.md`, `CHANGELOG.md` and the test-pass ledger are **coordinator-owned** this session —
agents report lines for them rather than editing them, so four worktrees do not collide in one file.

## Queued, not yet dispatched

1. **S1c-a, the format v3 contract close** — drop `ALPHA_MASKED`/`WIND` from the instance flags (pure
   functions of the chain) and define the `material_override` target. Bevy-free, seconds to test.
   **Time-critical: the format is free only until the first bake, and C1 is what starts baking.**
   Held until R1/R2 land, because it moves `sf_scene/src/table.rs` line numbers the briefs cite.
2. **B1+B2**, then **B3+B4**, then **B5+B6+C1/C2** — on the briefs R1 and R2 are writing.
3. **S1c-b** — decode LOD0 only for eligible models.
4. The owner's open questions, carried forward unanswered: is **S3a3** worth building at all (SPUD, the
   shipped reference for this data model, has no GPU culling whatsoever); **interiors** in or out
   (default: out); **`NO_LOD`** placements (barred from the table today).

## Regression baseline, measured on this machine at `main` = `e3112be`

    bash bevyrrowind/tools/cargo_turn.sh test -p sf_scene --features bake --all-targets -j 6
    -> Finished in 38.96s; running 107 tests; 105 passed; 0 failed; 2 ignored

A5/A6 took this from the 85 the A5/A6 brief quoted to 105. **No later slice may land with fewer passing
or with a failure.** `sf_scene` is Bevy-free, so the whole run is 39 s cold-ish and seconds warm — there
is never a reason to defer it. Two ignored are the deliberate `#[ignore = "measurement"]` ones
(`class_allocator_high_water_curve` and the real-install Vvardenfell measurement).

Remember the standing caveat when reading any green run here: **there is no Morrowind install on this
machine**, and ~100 install-gated tests across `bevyrrowind/crates` pass having asserted nothing. None of
`sf_scene`'s own 105 are install-gated, but no ledger row may ever say "N tests pass" without saying how
many asserted nothing.
