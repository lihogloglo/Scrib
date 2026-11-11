# Brief: static scene B5a-2, spawning a promoted placement's entity

Coordinator `scene-coord`, 2026-11-11. One deliverable: gameplay sends `sf_world_bevy::PromoteToEntity { worldspace,
cell, placement }` for a placement the static scene draws; streaming loads its model on demand, spawns its entity
under the cell's near group, and **only then** tells the scene hook, which sends B5a's `PromotePlacement` so the
instance stops drawing. Entity first, deactivation after: no frame where the object is missing (the ISM → actor swap
order UE's Mass representation and foliage-to-actor tools use). Previous slice: B5a `019b13a`.

**Out of scope:** precombine members (a precombined bucket can't hide one member: `log::debug!` and drop; logged as
a gap), uncooked models and spawners with a `prepare` asset (scene-covered placements are cooked statics: drop with
a `debug!`), overlay-removed placements (B5b), counters (B6), budgeting promotions through the scheduler (they are
rare; a per-frame poll in `drive` is enough until measured otherwise).

## Where it stands (`superfargoth/crates/sf_world_bevy/src/`)

- `stream.rs:260-293` `Work`: `placements: Arc<Vec<IndexedPlacement>>`, `members` (precombine ∪ scene-covered: never
  spawned, no model gathered, `:815-817`, `:1316-1319`). `Work` is removed at `reveal` (`:1350-1380`); the near
  `Group { entity, models, _precombine }` (`:296-301`) is what stays, in `CellRuntime.groups` (`:303-307`).
- Model refcounting: `start_assets` (`:950-970`, `users += 1`, `load_queue`), `poll_models` (`:1028-1080`),
  `model_ready` / the `Ready` lookup (`:980-990`), `release_models` (`:1116-1125`). `despawn_item` (`:1416-1455`)
  despawns the group's children and releases `group.models`.
- The spawn of one placement: `:1305-1352` (`spawner_for`, `ends.of(p)`, `SpawnCx`, `origin.delta_f32`, the batch
  entity under the group). `ends(world, request, info)` (`:1192-1207`) takes a `Request` only for its worldspace and
  cell. `layers`: see how `:1278` builds them.
- The drive loop: `drive` (`:500-560`); `drive_streaming` (`:487-497`) runs in `StreamingSystems::Drive`
  (`lib.rs:70-76`).
- The seam: `CellScene` (`scene.rs:45-120`). sf_scene_bevy's `SceneHook` (`sf_scene_bevy/src/residency.rs`, the
  `impl CellScene for SceneHook` block) and B5a's `PromotePlacement` + `apply_promotions` (`residency.rs:669-725`),
  which runs in `SceneSystems::Promote`, after `StreamingSystems::Drive`.

## Read (line ranges only)

The ranges above, plus `superfargoth/docs/specs/scene-runtime-plan.md:653-664` (§7 first paragraph), and for tests
`sf_world_bevy/src/tests.rs:615-790` (`SceneLog`, `TestScene`, `CountSpawner`, `covered_in`, `stream_in`, and
`scene_covered_placements_load_no_models_and_spawn_nothing`, the shape to copy).

## Do

1. The near `Group` keeps the load's `placements` Arc and its scene-covered set (moved from `Work` at `reveal`, no
   copy), plus a `promoted: HashSet<PlacementId>`.
2. `#[derive(Message)] pub struct PromoteToEntity { worldspace, cell, placement }` registered by
   `WorldStreamingPlugin`. `drive` reads them (after `poll_models`): if the cell's near group exists, the placement is
   in its scene-covered set and not yet promoted, acquire its model (`users += 1`, pushed to `group.models` so the
   unload releases it) and park a pending promotion. Each frame, a pending whose model is `Ready` (plus the one
   settle frame `assets_settled` uses) spawns through `spawner_for` + `SpawnCx` exactly as `:1305-1352` does (factor
   that body into one function both call), then calls the new `CellScene::promoted(&self, world: &mut World,
   worldspace, cell, placement)` (default: nothing). A pending whose cell left Near is dropped and its model ref
   released. Anything else (no near group, not covered, precombined, already promoted, model failed): `debug!`, drop.
3. `SceneHook::promoted` writes `PromotePlacement` (`world.write_message`). Update §7's "The caller then spawns" in
   `scene-runtime-plan.md` to one as-built sentence naming `PromoteToEntity`.

## Tests (exact, headless)

In `sf_world_bevy/src/tests.rs`, with `TestScene` covering the cell and `CountSpawner` recording spawns:
1. `promote_to_entity_spawns_once_then_tells_the_scene`: after stream-in, one `PromoteToEntity` for a covered
   placement: exactly one spawn of that placement, under the cell's near group, and `TestScene` logs one `promoted`
   call for it, in a frame after (or the same as) the spawn, never before; a second request spawns nothing more.
2. `promote_to_entity_ignores_what_the_scene_does_not_draw`: an uncovered placement (already an entity), a cell not
   loaded, and an unknown placement id: no spawn, no `promoted` call, no panic.
3. `a_promoted_entity_and_its_model_leave_with_the_cell`: promote, then stream the cell out: the entity is despawned
   and the model slot's `users` falls back (no slot left for that model when nothing else uses it).
Plus in `sf_scene_bevy/src/residency_tests.rs`: one assertion (in an existing B5a test or a 4th small one) that
`SceneHook.promoted(..)` leaves exactly one `PromotePlacement` with the right fields in `Messages<PromotePlacement>`.

## Rules

- **Hand off at ~80k tokens of context**: stop on code that compiles, run what tests you have, and report what's
  left.
- Every cargo command goes through `bash bevyrrowind/tools/cargo_turn.sh <args>` from the repo root. It always exits
  0, so judge by output. Check `tasklist | grep -iE "cargo|rustc"` first (another session may be building; wait).
  Never `--workspace`, never `cargo fmt`. Filter cargo output to errors, warnings and `test result`.
- Other sessions have uncommitted files (menus: `mw_exterior_bevy`, root `Cargo.toml`, workspace-hack; clock:
  `sf_ids`, `sf_rpg`, `sf_ai`, `sf_effects`). Don't touch them. No commit, no game launch.
- Floor: `test -p sf_world_bevy --all-targets` 33 passed (yours adds 3); `test -p sf_scene_bevy --all-targets` 22;
  `clippy -p sf_world_bevy -p sf_scene_bevy --all-targets -- -D warnings` clean.

## Report (≤ 40 lines)

Files with line ranges, the tests and counts, what was wrong in this brief, what B5b inherits, the gaps you logged
(precombine members, pipeline-compile delay on a first-seen material), and the context you had used when you stopped.
