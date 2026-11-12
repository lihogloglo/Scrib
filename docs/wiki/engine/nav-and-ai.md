# Navmesh and AI

## What it does
The framework has a navmesh core with a Recast backend, a bake and a debug overlay, plus an AI core (HTN planner,
directive stacks, decision traces). **None of it runs in the game yet.** Today NPCs wander on Morrowind's PGRD
pathgrids through a named scaffold; the navmesh is only baked to disk.

## How it works
- **Navmesh core** (`sf_nav`, Bevy-free): agent classes, `AreaTable`, `QueryFilter`, sliced `PathJob`, and the
  `NavBackend` trait (`sf_nav/src/backend.rs:253`: bake tile, residency, nearest, sliced `find_path`, raycast).
- **Backend** (`sf_nav_recast`): `RecastBackend` over the pure-Rust `navmesh-rs` (Recast + Detour). One Detour mesh per
  (worldspace, agent class, region); tiles are DNAV v7; region-local f32 meters; stale refs fail by salt.
- **Bake** (`sf_nav_bake`): `nav.tiles` (per tile, reads recorded as dynamic inputs so a moved wall re-bakes only its
  tiles; door volumes get own polygons) and `nav.links` (off-mesh links). Backend-agnostic.
- **Morrowind adapter** (`mw_bake/src/nav.rs`): `MwNavSource` feeds the cooked collision and baked terrain heights;
  `bake_nav` (`:956`) runs inside `bevyrrowind bake` (`mw_boot_bevy/src/bake.rs:70`). Water is a flat swim-level quad,
  as in OpenMW. Scaffold: `BEVYRROWIND_BAKE_SKIP_NAV=1` skips it (`bake.rs:65`).
- **Nav runtime shell** (`sf_nav_bevy`): `NavMeshes` resource, `nav.stats`, `nav.draw tiles|paths` debug layers. The
  game does not add this plugin (no use in `bevyrrowind/`); no streaming of baked tiles, no query service.
- **AI core** (`sf_ai`, Bevy-free): ids, decision `trace`, typed `WorldState`, HTN `Library` and planner (`htn`,
  MTR compare, bounded backtracking), per-actor `DirectiveStack` with `ModernPolicy` (`directive`), `AiSave` shape for
  the `sf/ai` section (`save`). No drives, no executors yet.
- **AI shell** (`sf_ai_bevy`): `AiPlugin`, trace store, `AiLibrary` loaded from records, `ai.select`, optional gizmos.
  Not added by the game; the `sf/ai` save section exists in `sf_save_bevy/src/sections/mod.rs:282` but nothing fills it.
- **What NPCs actually do** (`mw_ai`, Bevy-free; `mw_walkers_bevy/src/walkers.rs`, added at `exterior.rs:229`):
  PGRD pathgrids as per-cell graphs with A* (`pathgrid`), the `AI_W` wander package and OpenMW's idle roll (`wander`),
  and `PathWalkScaffold` (`scaffold.rs:57`) turning wander steps into `sf_controller` `PathFollower` walks. It is
  named a scaffold "until `sf_ai` exists". No combat AI, schedules, reactions, perception or following.

## Bevy features used
Resources and console commands only; gizmos behind feature `debug`. No Bevy navmesh crate (surveyed in the spec).

## Switches and counters
`nav.stats`, `nav.draw <layer>`, `ai.select` (console, only if the plugins are added). Walkers' physics follows
`--off physics` (`exterior.rs:229`). No nav or AI `--off` part.

## Known limits
Baked navmeshes are not loaded by the game; cold nav bake time is not yet within the "few minutes" gate (scaffold
above). Walkers ignore the navmesh and the HTN. See `docs/ROADMAP.md` for the AI and nav slices.

## Specs
`superfargoth/docs/specs/nav.md`, `ai.md`, `ai-as-built.md`, `orca.md`; `bevyrrowind/docs/specs/ai-adapter.md`,
`actors-adapter.md`.
