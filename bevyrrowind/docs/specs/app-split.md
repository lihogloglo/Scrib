# App split: plugin crates out of `apps/bevyrrowind` (ROADMAP 1.11)

Plan only (2026-10-25, lane `appsplit-plan`). Rules: ARCHITECTURE §8b (now build-perf.md) 3 (wide, shallow), 4 (thin binary), 5 (split by
change frequency), 6 (minimal Bevy features per crate). Coupling measured by `crate::` greps over
`apps/bevyrrowind/src` (18.3k lines, 40 modules).

## Naming

Of the 18 existing `mw_*` crates only `mw_assets` depends on Bevy; the rest are Bevy-free cores (`mw_water`,
`mw_weather`, `mw_audio`, `mw_rpg`, ...). New Bevy plugin crates use the framework suffix: **`mw_<topic>_bevy`**
(like `sf_*_bevy`), so "Bevy or not" is readable from the name and `mw_audio` / `mw_audio_bevy` don't collide.
One exception: `mw_game_core` (Bevy, but tiny). Each new crate is a plugin (`pub fn add(app)` or `impl Plugin`), no
`main`. Per crate: root workspace member, `[lints] workspace = true`, `bevy = { default-features = false, features = [..] }`
with only what it uses, then `cargo hakari generate && cargo hakari manage-deps` (Bevy crates stay IN the hack; only
Bevy-free crates go in `final-excludes`).

## Module graph today (`crate::X` uses)

The hub is **`exterior.rs`**: it calls 23 modules' `add(...)` (legitimate wiring) AND is imported back by 9 of them
for a few shared items. Those back-edges are the cycles.

| Module | Uses | Used by |
|---|---|---|
| exterior | actors, audio, dialogue, doors, hot_reload, inventory(_ui), level, live_ab, night_sky, perf, physics, pipeline_watch, player, remote, render, saves, scenes, scripts, ui_theme, walkers, water, weather | doors, physics, water, walkers, console, live_ab, remote, player, view |
| doors | exterior (`Settled`), player (`PlayerBody`), perf (`Off`,`Part`) | exterior, player (5), audio, console, saves, scripts, level |
| player | controls, doors (5), walkers (`ActorSpace`), exterior (`Settled`) | doors, console, dialogue, saves, scripts |
| walkers | physics, exterior (`ExteriorCamera`) | exterior, player, level/actors |
| physics | exterior (`Settled`) | walkers, exterior |
| dialogue | player (`PlayerMode`), ui_theme | inventory(_ui), console, saves |
| inventory / inventory_ui | dialogue, scripts (`ActivePlacements`), controls | exterior, saves |
| scripts | controls, doors, mods, player | exterior, inventory |
| saves | controls, dialogue, doors, inventory, mods, player | exterior |
| water | exterior (`ExteriorCamera`,`Quality`) | exterior |
| weather | `crate::load` (main.rs) | exterior, live_ab, remote |
| actors | actor_fx, actor_particles, faces, held | exterior, character_view, held |
| mods | bake, hot_reload, ui_theme | bake, controls, saves, scripts |
| live_ab | console, remote, scenes, weather, exterior (`pinned_clouds`), perf | console, remote, view, exterior, scenes |
| console | dialogue (`Rpg`,`key`), doors, exterior, live_ab, perf, player | live_ab |
| remote | exterior (`Readiness`), live_ab (7), scenes, view (`Loading`), weather | live_ab, character_view |
| scenes | level, live_ab, `crate::run` (main.rs) | exterior, live_ab, remote |
| level (+actors) | doors, exterior (`run`), `crate::load`, walkers | exterior, scenes |
| view | anim_view, skinned_view, exterior consts, live_ab, perf, pipeline_watch, render, `shipped_errors` | main |
| character_view | actors, held, remote, render, `shipped_errors` | main |
| perf mod / perf::cli / sweep.rs | none (cli and sweep: only each other) | many (`Off`,`Part`,`Perf`) |
| leaves | controls (->mods), audio (->doors), faces, actor_fx, actor_particles, hot_reload, night_sky, ui_theme, pipeline_watch, render, anim_view, skinned_view | |

### Cycles and how to break each

1. **exterior <-> {doors, player, physics, walkers, water, console, live_ab, remote}** (the big one). Back-imports are
   only `Settled`, `ExteriorCamera`, `Quality`, `Readiness`, `CAPTURE_*`, `pinned_clouds`/`CLOUD_WORLD_SCALE`.
   Fix: markers/enums to `mw_game_core`; `Readiness` to `mw_devtools_bevy` (it needs only `doors::AutoDoor`,
   `pipeline_watch::PendingDraws`); exterior then only calls `add()` downward.
2. **doors <-> player**: doors needs only the `PlayerBody` marker; player needs `WorldLinks`, `DoorUser`,
   `arrival_rotation`, `ARRIVAL_EYE_M`. Fix: `PlayerBody`, `PlayerMode`, `Focus`, `FocusUpdate` to `mw_game_core`; doors
   stays below player.
3. **mods <-> bake** (+ mods -> hot_reload, ui_theme): same crate (`mw_boot_bevy`). The CLI half of mods.rs
   (`ModsCommand`, `run`, `open`) stays in the binary and calls the crate's `select`, `bindings_profile`, `load_content`.
4. **live_ab <-> console <-> remote** (3-cycle) and **remote -> view -> live_ab -> remote**: one crate
   (`mw_devtools_bevy`); `view::Loading` (interior-load flag read by remote) moves into it so `view` sits above.
5. **exterior -> level -> exterior::run; scenes -> level -> exterior -> scenes; scenes::run -> main::run; level, weather -> main::load**:
   scene registry (`SceneDef`, `active()`, ~200 lines) to `mw_game_core` (live_ab, remote read `active()`);
   `scenes::run` stays in the binary; `main::load`/`mount` move to `mw_boot_bevy`; `level` + `exterior` share a crate.
6. **`crate::shipped_errors` (main.rs) used by exterior, view, character_view**: 12 lines, into `mw_game_core`.
7. No cycle: dialogue -> inventory -> scripts -> player, and saves above all of them, form a DAG. walkers -> physics is one-way.

## Target crates

| Crate (layer) | Modules moved in (lines) | Deps on other new crates | Bevy |
|---|---|---|---|
| `mw_game_core` (L0) | `Settled`, `ExteriorCamera`, `Quality`, `CAPTURE_*`, cloud consts/`pinned_clouds`, `PlayerBody`, `PlayerMode`, `Focus`, `FocusUpdate`, `DoorUser`, `ARRIVAL_EYE_M`, `perf::{Off,Part}`, scene registry, `shipped_errors` (~500) | none | yes, minimal features; types only, no systems, must rarely change |
| `mw_boot_bevy` (L1) | mods (lib half), bake, hot_reload, ui_theme, `load`/`mount` (~900) | core | yes |
| `mw_controls_bevy` (L1) | controls (376) | boot | yes (+ bevy_enhanced_input) |
| `mw_doors_bevy` (L1) | doors (457) | core | yes |
| `mw_walkers_bevy` (L1) | walkers, physics (1937) | core | yes |
| `mw_actors_bevy` (L1) | actors, actor_fx, actor_particles, faces, held (1705) | core | yes |
| `mw_environment_bevy` (L2) | water, weather, night_sky, render (962) | core, boot | yes |
| `mw_audio_bevy` (L2) | audio (822) | doors | yes |
| `mw_player_bevy` (L2) | player + tests (1137) | core, controls, doors, walkers | yes |
| `mw_scripts_bevy` (L3) | scripts (369) | core, controls, doors, boot, player | yes |
| `mw_rpg_bevy` (L4) | dialogue, inventory, inventory_ui (1810) | core, boot, controls, scripts | yes |
| `mw_saves_bevy` (L5) | saves (425) | rpg, player, doors, controls, boot | yes |
| `mw_devtools_bevy` (L6) | console, live_ab, remote, pipeline_watch, `Readiness`, `view::Loading`, perf/mod: Perf plugin, flags, `render_size` (~3100) | core, doors, player, rpg, environment | yes (`remote` feature forwarded) |
| `mw_viewers_bevy` (L7) | view, anim_view, skinned_view, character_view (~1200) | devtools, actors, environment, core | yes |
| `mw_exterior_bevy` (L7) | exterior, level, level/actors (~2200) | every plugin crate above (composition crate) | yes |

**Stays in `apps/bevyrrowind` (~2.2k lines)**: `main.rs` (clap, info/ls/which/nif/record/find, `run`), mods CLI half,
`perf/cli.rs` + `perf/sweep.rs` (shell-out benchmark suite, 776), `sweep.rs` (nif-sweep), `scenes::run`. Binary
features (`dev`, `remote`, `solari`, `dlss`, `trace`, `tracy`) forward to the crates that need them. A later `mw_cli`
Bevy-free library for the record/find/nif commands is optional.

Notes. `mw_devtools_bevy` is dev-flavoured but linked always today (`live_ab::add` is unconditional); gating it behind
a feature is a separate decision. Critical path of depth is core -> doors -> player -> scripts -> rpg -> saves ->
devtools -> exterior -> bin (8 deep). To shorten: devtools uses `Rpg` only in two console commands
(`console.rs:142-151`); a small trait/event in core removes the rpg dependency (devtools becomes L3 and can go early).
`character_view` can sit beside actors if a lane edits it daily. Split `exterior` further later (sun/clouds/streaming
config, capture) only if it stays the most-edited file.

## What the workspace merge changes

One workspace at the repo root (`merge-ws1`): new crates are plain `crates/mw_*_bevy` members using `workspace = true`
deps, no `path = "../../crates/..."` entries, one `Cargo.lock`, one hakari run, one target dir. §8b (now build-perf.md)'s "path dependency
built from bevyrrowind gets `[profile.dev.package."*"]` opt-level 3" stops applying: re-check profile overrides so the
new game crates build at the game's opt-level 1 (not 3), or every edit in them rebuilds optimised. Do not start slice 1
until `merge-ws1` has landed the root `Cargo.toml`; crates created in the old layout would be redone.

## Extraction slices (each compiles alone; the app keeps `pub use mw_x::foo;` shims so `crate::foo::..` paths still resolve)

Per slice: move the module(s) into the crate, widen `pub(crate)` to `pub`, add the dep + shim, verify
`cargo check -p <new> -p bevyrrowind` and `cargo test -p <new>`, one `cargo dev` smoke (`view --screenshot`), note the
touch-build time. One extraction per session; the lane owning a module edits it elsewhere only after its move lands.

1. [x] **`mw_game_core`** (done 2026-10-25; `Focus` stays in the app, `Quality` render settings stay as `exterior::QualityRender`): types only, no systems move. Breaks cycles 1, 2, 6. Lowest risk. Needed by every later slice.
   (If `Quality` carries a clap `ValueEnum` derive, keep the parse wrapper in the binary.)
2. [x] **`mw_boot_bevy`** (done 2026-10-26; mods lib half incl. `open`, which `load_content` needs; UI theme assets moved with it; no `mw_game_core` dep needed) (mods lib, bake, hot_reload, ui_theme, load/mount). Breaks cycles 3 and the `crate::load` part of 5.
   Unblocks the content/mods/bake lane and controls/environment.
3. **`mw_doors_bevy` + `mw_controls_bevy`** (833 lines, two small leaves, can be two parallel sessions).
4. `mw_walkers_bevy` (walkers, physics): AI/movement/combat lane.
5. `mw_actors_bevy` (actors, fx, particles, faces, held): character/animation lane. Parallel with 4.
6. `mw_environment_bevy` (water, weather, night_sky, render) and `mw_audio_bevy`: rendering, weather and audio lanes.
7. `mw_player_bevy`, 8. `mw_scripts_bevy`, 9. `mw_rpg_bevy`, 10. `mw_saves_bevy`: strict order (each depends on the last).
11. `mw_devtools_bevy` (breaks cycle 4; unblocks perf/rendering-bug lanes that edit live_ab/remote/console). Can move up
    to slice 4 if the Rpg trait cut above is done first.
12. `mw_viewers_bevy`, 13. `mw_exterior_bevy`: the app is now ~2.2k lines.
14. Remove shims; update CLAUDE.md module map and ARCHITECTURE §8b (now build-perf.md) crate list; re-measure the §8b gates (touch a leaf
    plugin crate <= 5 s, no-op <= 1 s, clean <= 12 min, since each crate adds a link unit) into `docs/perf.md`; tick ROADMAP 1.11.

Parallelism: after slice 2, slices 3, 4, 5 and 6 touch disjoint modules and can run as separate lanes (respect the
3-build-agent cap); 7-10 are serial; 11 needs 3, 7 and 9 (or the trait cut).
