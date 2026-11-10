# Static scene: state at the end of 2026-11-10, and the next slices

Written by session `coord-s3b` (`scrib-4f`) at the end of a laptop coordinator session. `main` = `5acb8eb`,
pushed. **Everything is on `origin`**; nothing of this session exists only on the laptop. The owner moves
to another machine next, so this file plus the two briefs it names are the whole handoff.

## What landed today (all merged to `main`, all pushed)

| | What | Verified |
|---|---|---|
| **B1** | `sf_scene_bevy` exists: the baked `.sfscene` cell table as a validating asset + loader, `SceneConfig`, a `ScenePlugin` nobody adds yet | compiles, 2 tests, clippy clean |
| **B2** | the `CellScene`/`SceneCells` seam in `sf_world_bevy`, **seven** `stream.rs` call sites, one idempotent `release` | 32 tests (29 before), clippy clean |
| docs | the static-scene spec pass (19 queued corrections), the plan-correction pass, two coordinator handoffs out of untracked `.sessions/` | — |
| research | `docs/briefs/scene-bevy-b1-b2.md` (650 lines), `docs/briefs/scene-bevy-b3-b4.md` (891 lines), the bind-group verdict | — |

Regression baselines measured on the laptop at `main`, none of which a later slice may fall below:

    cargo_turn.sh test   -p sf_scene --features bake --all-targets  -> 105 passed, 0 failed, 2 ignored
    cargo_turn.sh test   -p sf_world_bevy                           -> 32 passed, 0 failed
    cargo_turn.sh test   -p sf_scene_bevy                           -> 2 passed, 0 failed
    cargo_turn.sh clippy -p sf_scene --features bake --all-targets -- -D warnings  -> Finished, clean

**Nothing is verified in game.** There is no game and no Morrowind data on the laptop, the owner's test pass
has not run, and **nothing has ever been baked** (`bake_scene` is still not called — that is step C1).
`SceneCells::default()` holds no hook, so nothing streams differently yet either.

## The one thing that must be decided before B3 starts

`covered` is answered in the **Fetch** stage, but `plan_in` runs later. So by the time a chain load fails or
a batch answers `AllocError::Exhausted`, those placements are **already in `work.members` and their models
were never gathered** — which means the plan's §5.3 rows 3-4 and §4.4's "overflowed placements are left out
of `covered`" are **not deliverable as written**. Three candidate shapes are named in
`scene-runtime-plan.md` §5.3; none was designed, deliberately, because it is a coordinator/owner call.

This went from theoretical to central today: the owner chose to **keep per-batch capacity at 4096 and test
the entity fallback hard**, so the `Exhausted` path (row 4) is the hot one and needs the ruling first.

## Owner decisions taken today (binding)

1. Compile on the laptop as we go; `main` only gets code that compiles. (Honoured: every merge above was built.)
2. The 2026-11-10 test pass did **not** happen. S3b proceeds on the plan's default `PerClass` granularity, and
   the **S0 perf baseline is still an open owner run** — it is the one measurement a design decision waits on.
3. Extra scope approved and delivered: the spec pass, the plan corrections, the bind-group research.
4. **S3a3 stays parked.** Finish S3b, then decide with real numbers rather than arithmetic. Accepted
   consequence: S3b's first in-game numbers will look worse than the design deserves, because per-view output
   is sized by *reserved* capacity until S3a3 lands. Say so when reporting them.
5. **Per-batch capacity stays 4096**, with `Exhausted` → entity fallback a first-class tested path. If the
   measured class × slab-pair × bind-group fan-out exceeds ~16 batches, the fix is a **total** slot budget in
   `SpaceConfig` with per-batch capacity derived — an `sf_scene` (A5) change, still awaiting the owner.

## Next slices, in order

1. **The `covered`/`plan_in` ruling** above. Docs-only, but it blocks B3.
2. **B3** — `SceneSpaces`, chain asset loading, batch entities + `ranges`, materials, `poll_cell_loads`, the
   hook impl. Brief: `docs/briefs/scene-bevy-b3-b4.md`. Two additions the B1/B2 agent asks for: the
   `far_min_radius` harness trap (the shared fixture's near step is empty unless the radius is raised — use
   `streaming_app_radius`), and the fact that **no test yet proves `acquire`'s handle holds a fetch open**
   (`TestScene::acquire` returns `None`, so that call site short-circuits); B3 must add it once a real table
   loads per cell. B3 also needs the **render→main slab snapshot** through `ResMut<MainWorld>` in
   `ExtractSchedule` — novel in this repo, since both existing uses carry main→render.
3. **B4** — the render world: `extract_scene_uploads`, `prepare_scene_buffers`, `publish_scene`. The research
   agent recommends B3 and B4 as **two** slices, because B4's tests are near-vacuous without non-empty batches.
4. **B5+B6** — promotion, the `PreprocessingOnly` fallback and warning, `suppress_scene_placements`, counters.
5. **C1/C2** — wire `bake_scene` + `bake_chains` into `bevyrrowind bake`, then `--scene`, `Part::Scene`
   (`Part::ALL` → 55) and the precombine interlock. **C1 closes the format window**: format v3 is free to
   change only until something is baked, so the queued S1c follow-ups below must land *before* C1.
6. **S1c-a, queued and never dispatched**: drop `ALPHA_MASKED`/`WIND` from the instance flags (both are pure
   functions of the chain) and define the `material_override` target. Bevy-free, seconds to test. Then
   **S1c-b**: decode LOD0 only for eligible models.
7. **The bind-group cache** (`docs/briefs/perf-bind-group-cache.md`), after C2, and only after the owner runs
   `perf trace` on `balmora` and `balmora_walk` — the 2.3 ms that motivated it is a stale 2026-10-19 number.

## Open owner questions, carried forward

1. The `covered`/`plan_in` ruling (above) — the only one that blocks work.
2. The **total slot budget** change to `SpaceConfig`, if measured fan-out exceeds ~16 batches.
3. **Interiors** in or out of the static scene (default taken: out).
4. **`NO_LOD`** placements, barred from the table today; the scene could pin level 0 instead. Decide before S9.
5. `SceneConfig::worldspaces` defaults to `None` = "every worldspace with a `scene.cells` product", and
   nothing consults it yet (that is B3). If an explicit opt-in list is wanted, now is the cheap moment.
6. **Is S3a3 worth building at all**, once S3b's numbers exist? SPUD, the shipped reference for this exact
   data model, has no GPU culling whatsoever.

## Two loose findings worth a line each

- **Our own GS materialize cache may be under-keyed**: `materialize.rs:265` keys four buffers but binds five —
  `job.metadata` is unkeyed and rewritten every frame. Probably safe by an unstated invariant; **unverified**.
  Worth settling before anyone trusts that cache.
- **`scene-runtime-plan.md` is now 72 KB**, roughly three times a spec budget, because every correction had to
  carry its evidence. It wants a split (the plan, plus a corrections appendix, or the measurements moved into
  the briefs it already cites). Nobody should read it whole; read the section your slice names.

## Process notes for whoever picks this up

- A worktree at a different path gets different cargo package ids, so **any** separate worktree pays a full
  cold vendored-Bevy rebuild. Only `repo/` can use the warm target: run Bevy builds there, and give
  docs/research agents worktrees.
- `bash bevyrrowind/tools/cargo_turn.sh` **always exits 0** — judge every run by its output (`^error`,
  `could not compile`, `test result:`, `Finished`), never by `$?`. Never `--workspace`; always `--all-targets`
  on clippy; never `cargo fmt` (the repo is not fmt-clean and fmt is not in the gate).
- `repo/`'s working tree still carries the mechanics lane's four uncommitted files plus untracked
  `superfargoth/docs/specs/clock.md`, preserved on `origin/backup/mechanics-wip-2026-11-09`. Leave them alone.
- Brief agents from **verified** briefs, not from the plan: three times now, an agent sent at the plan found it
  stale, and the one brief written *from* the plan was found unbuildable. Point them at section ranges and tell
  them the brief outranks the plan.
