# Tunables census

Status: generated 2026-11-16 (spec `tunables.md`, slice T0). **Grep this file, never read it whole.** Every row is a
*candidate* found by name; the class is a first pass by rules, not a decision. A migration slice (tunables.md
"Migration") re-reads its system's rows, applies tunables.md "What stays a const", and fills provenance.

## How it is made

The generator is the script in the appendix (Node ≥ 18, no dependencies). From the repo root:

```sh
node tools/tunables-census.mjs          # totals per system
node tools/tunables-census.mjs --md     # the tables below ("Generated tables")
node tools/tunables-census.mjs --json   # raw rows (file, line, name, type, value, class, system)
```

Until slice T0 commits it as `tools/tunables-census.mjs`, copy the appendix there. Cross-checks with plain ripgrep
(the script's three sources):

```sh
rg -n -g '*.rs' -g '!*tests*' '^\s*(pub(\([a-z]+\))?\s+)?const\s+[A-Z][A-Z0-9_]*\s*:\s*(bool|f32|f64|u\d+|i\d+|usize|isize|Duration|Vec[234]|\[f32;)' superfargoth/crates bevyrrowind/crates bevyrrowind/apps
rg -n -g '*.rs' 'impl Default for \w+' superfargoth/crates bevyrrowind/crates bevyrrowind/apps     # then literal fields inside
rg -n -g '*.wesl' '^\s*const\s+[A-Z][A-Z0-9_]*' superfargoth/crates bevyrrowind/crates
rg -o -N -I 'BEVYRROWIND_[A-Z0-9_]+' -g '*.rs' bevyrrowind superfargoth | sort -u                 # env switches
```

**What it scans:** non-test `.rs`, `.wesl` and `.wgsl` under `superfargoth/crates`, `bevyrrowind/crates`,
`bevyrrowind/apps`; skips `tests/`, `examples/`, `benches/`, `vendor/`, `*_tests.rs`, `tests*.rs` and the rest of a file
after `#[cfg(test)] mod`. Rows: (1) `const NAME: <number, bool, tuple, array, Duration, Vec>` ; (2) every literal field
of an `impl Default for X` (one per line, or inside a one-line `Self { .. }`), container defaults (`Vec::new()` ...)
dropped; (3) WESL/WGSL `const`. **Not scanned:** literals inline in expressions (`if masked > 0.5`), preset `match`
arms and values built in game code (hand-listed in "Hand-reviewed" below), `.ron`/`.toml` data (already data).

**Class rules (first pass, in order):** C if the file is a test scene or lab (`level/`, `*_lab.rs`, viewers); C if a
`Default` field is runtime state (`elapsed`, `timer`, `predicted_us` ...); C for sentinels/derived (`::MAX`, `::ALL`,
`size_of`, hex); C for units and formats (`UNITS_PER_*`, `CELL_UNITS`); C for layout, maths and ids (`VERSION`,
`EPS`, `BYTES`, `STRIDE`, `MASK`, `PI`, `FLAG_*`, `DEBUG_*`, `NO_*` ...); C when the value is an expression of other
consts; C for small `u` integers in shaders without a tuning word (enum codes); **T** when the name carries a tuning
word (distance, range, radius, fade, budget, LOD, cascade, shadow, hysteresis, threshold, frames, seconds, speed,
gain, bias, scale, max/min, pixels, count, size, ...) or is a field of a `*Settings/*Config/*Params/*Budget/*Lod...`
default; C for a `usize` under 64 without one (indices); a file with 8+ unsure plain integers is an id table (C);
else **?**. Expect noise both ways (`MAX_RAYS` is T but is also a GPU array bound: see tunables.md "Capacities").

## Totals (2026-11-16)

Sources: 1,187 consts, 789 `Default` fields, 192 shader consts (102 of them T: each needs a uniform before it can
be live). Env switches: 30 `BEVYRROWIND_*` names read at 32 sites, plus 9 `SF_*` (bake/test tooling); `--off` has 69
parts; 9 live console variables are registered (`mw_devtools_bevy/src/console.rs`).

Perf/look systems (shadows-lights-gi, streaming-exterior, statics-lod-farfield, animation-characters,
sky-weather-surface, terrain-water, textures): **1,286 candidates, T 738, C 285, ? 263.**

## Hand-reviewed: what the scan can't see

### Preset tables in code (the "Scalability.ini" we have, as `match` arms)

These are the values the `Quality` preset (`--quality`, `BEVYRROWIND_QUALITY`) writes. They become preset group rows
(tunables.md "Presets"), one tunable per column. Provenance today: the doc comments cite Bevy defaults, `OpenMW`
(2 px cutoff) and ledger lines for some; the rest is "guess".

| Where | Tunable (proposed name) | Low | Medium | High | Ultra | Note |
|---|---|---|---|---|---|---|
| mw_exterior_bevy/src/exterior.rs:1620 `params` | `lod.bias` | 0.75 | 0.875 | 1.0 | 1.0 | view-distance bias, scales rings + far handoff |
| same | `lod.min_pixels` | 4 | 3 | 2 | 1 | size cull, px at 1440p (High = OpenMW 2 px) |
| same | `lod.min_draw_distance_m` | 16 | 20 | 24 | 32 | size-cull floor |
| exterior.rs:1635 `point_shadows` | `shadow.lamp.count` / `shadow.lamp.map_size` | 0 / 512 | 6 / 512 | 8 / 512 | 12 / 1024 | High from `POINT_SHADOWS_DEFAULT` |
| exterior.rs:1650 `contact_shadows` | `shadow.contact.steps` | off | 8 | 16 | 16 | length 0.3 m = Bevy default |
| exterior.rs:1662 `lamp_contact_shadows` | `shadow.lamp.contact` | off | off | on | on | env `BEVYRROWIND_LAMP_CONTACT` |
| exterior.rs:1669 `ssao` | `render.ssao.quality` | off | low | medium | high | Bevy GTAO levels |
| exterior.rs:1680 `bloom` | `render.bloom` | 0 | 0.1 | 0.1 | 0.1 | `BLOOM_INTENSITY` |
| exterior.rs:1695 `sun_cascades` | `shadow.sun.cascades` | 3 | 3 | 3 | 4 | ledger perf 2026-11-16 (F2) |
| same | `shadow.sun.max_distance_m` | 160 | 190 | 205 | 280 | |
| same | `shadow.sun.lambda` | 0.82 | 0.80 | 0.80 | 0.85 | PSSM split blend |
| same | `shadow.sun.overlap` | 0.15 | 0.15 | 0.16 | 0.2 | |
| exterior.rs:1709 `soft_shadow_size` | `shadow.sun.pcss_size` | off | off | 10 | 10 | `SOFT_SHADOW_SIZE` |
| exterior.rs:1723 `shadow_filter` | `shadow.filter` | hardware | gaussian/temporal | same | same | env `BEVYRROWIND_SHADOW_FILTER` |
| mw_game_core/src/exterior.rs:68 `cloud_map_per_sample` | `sky.clouds.map_per_sample` | off | off | on | on | |
| mw_game_core/src/exterior.rs:75 `texture_cap` | `texture.max_size` | 512 | 1024 | 2048 | none | UE `MaxLODSize` analogue |
| mw_game_core/src/exterior.rs:89 `parallax` | `surface.parallax.layers` / `.relief_steps` | off | 16 / none | 32 / 5 | 64 / 8 | |
| mw_environment_bevy/src/water.rs:135 | `water.quality` | low | medium | high | ultra | maps to `WaterQuality` |

### Two truths for one value (fix in the system's migration slice)

| Value | Framework default | Game override | Note |
|---|---|---|---|
| lamp shadow fade, m | `PointShadowBudget` 15 / 25 (sf_sky_bevy/src/shadow_budget.rs:84) | `LAMP_SHADOW_FADE` 35 / 45 (mw_environment_bevy/src/render.rs:32) | the game value is the live one |
| lamp shadow map size | `PointShadowBudget.map_size` 1024 | `POINT_SHADOWS_DEFAULT` 512 (render.rs:28) | |
| High LOD screen | `LodScreen::default()` 1440 px, 1 px, bias 1, 2 px, 24 m (sf_model/src/lod.rs:32) | `params(High)` (exterior.rs:1620) | same numbers, two places |
| far handoff `d1` | `FarLevels.handoff` 400 / 2000 / 8000 m (sf_lod/src/levels.rs:72) | scaled by `lod.bias` | bake key: a change re-bakes (tunables.md "Lifetimes") |
| lamp shadow cache | `ShadowCacheSettings` (sf_shadow_cache_bevy/src/lib.rs:119) | `POINT_SHADOW_CACHE = false` (render.rs:37) | the legacy cache is off; its settings are dead weight until deleted |

### Env switches (30 `BEVYRROWIND_*`)

| Switch | Read at | Fold into | Kind |
|---|---|---|---|
| `QUALITY` | mw_game_core/src/exterior.rs:48 | preset driver `sf.graphics_preset` | player |
| `OFF` | mw_game_core/src/perf.rs:7 | `--off` stays; each part becomes a bool tunable `<system>.<part>` | dev |
| `POINT_SHADOWS` | mw_environment_bevy/src/render.rs:19 | `shadow.lamp.count`, `shadow.lamp.map_size` | player (preset) |
| `SUN_CASCADES` | render.rs:22 | `shadow.sun.cascades`, `shadow.sun.max_distance_m` | player (preset) |
| `SHADOW_FILTER` | mw_exterior_bevy/src/exterior.rs:597 | `shadow.filter` | player (preset) |
| `SSAO` | exterior.rs:588 | `render.ssao.quality` | player (preset) |
| `LAMP_CONTACT` | exterior.rs:379 | `shadow.lamp.contact` | player (preset) |
| `DLSS` | render.rs:40 | `render.upscaling` | player |
| `GPU_CULL_STATICS` | exterior.rs:296 | `stream.gpu_cull_statics` | dev |
| `COMPILE_THREADS` | exterior.rs:300 | `pipeline.compile_threads` | dev (startup) |
| `COMPUTE_THREADS` | exterior.rs:312 | `task.compute_threads` | dev (startup) |
| `NO_PIPELINED` | exterior.rs:319 | `render.pipelined` | dev (startup) |
| `CASCADE_ENCODERS` | apps/bevyrrowind/src/main.rs:629 | `shadow.sun.parallel_encoders` | dev (startup) |
| `LOADING_MESHES` | exterior.rs:1385 | `stream.max_loading_meshes` | dev |
| `AI_RADIUS` | mw_walkers_bevy/src/walkers.rs:82 | `ai.wake_radius_m` (+ sleep) | gameplay (host) |
| `STRENGTH` | mw_walkers_bevy/src/physics.rs:492 | `physics.body_strength` | gameplay (host) |
| `FPS_CAP` | physics.rs:591 | `app.fps_cap` | player |
| `HITCH_MS` | mw_devtools_bevy/src/perf.rs:35 | `diag.hitch_ms` | dev |
| `PRESENT` | perf.rs:374 | `app.present_mode` | player |
| `WINDOW` | perf.rs:391 | `app.window_mode` | player |
| `RENDER_SIZE` | perf.rs:396 | `diag.render_size` | dev |
| `AUTO` | physics.rs:886 | stays env (test driver script) | tooling |
| `STRIDE_TRACE`, `PIPELINE_DEFS`, `RAGDOLL_REPORT` | walkers.rs:783, pipeline_watch.rs:38, mw_boot_bevy/src/bake.rs:341 | bool `diag.*` tunables | dev |
| `BAKE_SKIP_NAV` | mw_boot_bevy/src/bake.rs:75 | stays env (bake tooling) | tooling |
| `MORROWIND`, `USER_DATA`, `ANIM_SOURCES` | mw_install/src/lib.rs:19, mw_mods/src/lib.rs:65, mw_bake/src/sources.rs:23 | stay env (paths, not values) | tooling |
| `BRP_PORT` | mw_devtools_bevy/src/remote.rs:47 | stays env (process wiring) | tooling |

So 21 of 30 fold into tunables (most become plain `--set`/`--arm` values, the env name kept one release as an alias
that sets the tunable with `SetBy::CommandLine`), 9 stay env (paths, ports, tooling drivers).

### Inline literals worth a look first (not counted)

Found while reviewing; the migration slice of each system greps its own files for more:
`sf_lod/src/policy.rs:125` (`masked_area > 0.5`), `sf_world_bevy/src/precombine.rs:67` (half band), the sun cascade
`minimum_distance: 0.1, split_near: 1.0` (exterior.rs:1702), `Bloom::NATURAL` fields.

## Generated tables

Per system, then per crate: the T and ? rows in full, the C rows as one line. Paths are relative to the crate.

| System | Candidates | T | C | ? |
|---|---:|---:|---:|---:|
| ai-nav | 52 | 23 | 14 | 15 |
| animation-characters | 196 | 127 | 54 | 15 |
| app-diag | 54 | 35 | 7 | 12 |
| audio | 43 | 36 | 1 | 6 |
| effects | 56 | 11 | 44 | 1 |
| feel-camera-combat | 208 | 153 | 13 | 42 |
| other | 238 | 73 | 100 | 65 |
| physics | 147 | 126 | 8 | 13 |
| shadows-lights-gi | 50 | 35 | 12 | 3 |
| sky-weather-surface | 604 | 325 | 102 | 177 |
| statics-lod-farfield | 102 | 55 | 34 | 13 |
| streaming-exterior | 103 | 71 | 24 | 8 |
| terrain-water | 210 | 115 | 53 | 42 |
| textures | 21 | 10 | 6 | 5 |
| ui | 84 | 50 | 7 | 27 |
| **total** | **2168** | **1245** | **479** | **444** |

## ai-nav

### mw_ai (11: T 5, C 2, ? 4)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/scaffold.rs:17 | WALK_SLACK | `5.0` | T |  |
| src/scaffold.rs:21 | STUCK_SECONDS | `3.0` | ? |  |
| src/scaffold.rs:23 | PROGRESS | `0.25` | ? |  |
| src/scaffold.rs:25 | SAME_GOAL | `0.5` | ? |  |
| src/wander.rs:22 | IDLE_CHANCE_MULTIPLIER | `0.75` | T |  |
| src/wander.rs:24 | MIN_WALK_SPEED | `100.0` | T |  |
| src/wander.rs:25 | MAX_WALK_SPEED | `200.0` | T |  |
| src/wander.rs:27 | DEFAULT_IDLE_SECONDS | `5.0` | ? |  |
| src/wander.rs:34 | START_RADIUS | `1.0` | T |  |

Stays const: GOAL_TRIES (small index), BLOCKED_EDGES (small index).

### sf_ai (6: T 3, C 2, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/directive.rs:45 | MAX_REACTIONS | `256` | T |  |
| src/htn.rs:62 | SEARCH_DEPTH | `64` | T |  |
| src/htn.rs:66 | SEARCH_TRIES | `4096` | ? |  |
| src/trace.rs:37 | DEFAULT_TRACE_CAP | `256` | T |  |

Stays const: AI_VERSION (layout/math/id), TRACE_LEN (layout/math/id).

### sf_ai_bevy (6: T 3, C 1, ? 2)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/gizmos.rs:32 | TEXT_SIZE | `0.12` | T |  |
| src/gizmos.rs:34 | LINE_STEP | `0.17` | T |  |
| src/gizmos.rs:36 | TEXT_LIFT | `2.1` | ? |  |
| src/gizmos.rs:37 | RING_RADIUS | `0.55` | T |  |
| src/lib.rs:67 | AiPlugin.timings | `true` | ? | bool |

Stays const: DEFAULT_TRACE_LINES (small index).

### sf_nav (8: T 3, C 4, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/grid.rs:35 | MAX_TILE_CELLS | `4096` | T |  |
| src/query.rs:54 | DEFAULT_EXTENT | `Vec3::new(2.0, 4.0, 2.0)` | ? |  |
| src/types.rs:13 | MAX_CLASSES | `8` | T |  |
| src/types.rs:16 | MAX_AREAS | `64` | T |  |

Stays const: BLOB_VERSION (layout/math/id), EPS (layout/math/id), FIELD_BITS (layout/math/id), FIELD_MASK (layout/math/id).

### sf_nav_bake (1: T 0, C 1, ? 0)

Stays const: JOIN_EPS (layout/math/id).

### sf_nav_bevy (3: T 0, C 0, ? 3)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/gizmos.rs:21 | LIFT | `0.05` | ? |  |
| src/gizmos.rs:23 | PATH_LIFT | `0.12` | ? |  |
| src/gizmos.rs:25 | TICK | `0.3` | ? |  |

### sf_nav_recast (17: T 9, C 4, ? 4)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/build.rs:24 | MAX_EDGE_LEN_M | `12.0` | T |  |
| src/build.rs:25 | MAX_SIMPLIFICATION_ERROR | `1.3` | T |  |
| src/build.rs:26 | MIN_REGION_AREA | `8 * 8` | T |  |
| src/build.rs:27 | MERGE_REGION_AREA | `20 * 20` | T |  |
| src/build.rs:28 | VERTS_PER_POLY | `6` | ? |  |
| src/build.rs:29 | DETAIL_SAMPLE_CELLS | `6.0` | ? |  |
| src/build.rs:30 | DETAIL_SAMPLE_MAX_ERROR_CELLS | `1.0` | T |  |
| src/lib.rs:62 | MAX_TILES_PER_MESH | `1 << 13` | T |  |
| src/lib.rs:65 | MAX_MESHES | `1 << refs::SLOT_BITS` | T |  |
| src/lib.rs:68 | MAX_POLYS_PER_TILE | `1 << 20` | T |  |
| src/refs.rs:11 | SLOT_SHIFT | `33` | ? |  |
| src/refs.rs:16 | OURS | `((1 << (SLOT_BITS + GEN_BITS)) - 1) <...` | ? |  |
| src/search.rs:35 | AWAY_RADIUS | `32_000.0` | T |  |

Stays const: QUERY_POOL (small index), SLOT_BITS (layout/math/id), GEN_SHIFT (derived), GEN_BITS (layout/math/id).

## animation-characters

### mw_actors (33: T 2, C 29, ? 2)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/equip.rs:42 | SLOT_COUNT | `17` | T |  |
| src/held.rs:247 | DayCycle.sunrise | `6.0` | ? |  |
| src/held.rs:247 | DayCycle.night_start | `20.0` | ? |  |
| src/slots.rs:26 | PRT_COUNT | `27` | T |  |

Stays const: HEAD (small index), HAIR (small index), NECK (small index), CUIRASS (small index), GROIN (small index), SKIRT (small index), R_HAND (small index), L_HAND (small index), R_WRIST (small index), L_WRIST (small index), SHIELD (small index), R_FOREARM (small index), L_FOREARM (small index), R_UPPER_ARM (small index), L_UPPER_ARM (small index), R_FOOT (small index), L_FOOT (small index), R_ANKLE (small index), L_ANKLE (small index), R_KNEE (small index), L_KNEE (small index), R_LEG (small index), L_LEG (small index), R_PAULDRON (small index), L_PAULDRON (small index), WEAPON (small index), TAIL (small index), FIRST_PERSON (layout/math/id), ActorConfig.first_person_fov_y_deg (layout/math/id).

### mw_actors_bevy (5: T 5, C 0, ? 0)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/actor_particles.rs:44 | ACTOR_PARTICLE_RANGE_M | `60.0` | T |  |
| src/held.rs:70 | HeldLightBudget.max | `12` | T | settings default |
| src/held.rs:78 | TORCH_SHADOW_PRIORITY | `1.0` | T |  |
| src/held.rs:120 | HeldLightTuning.clearance_m | `0.7` | T |  |
| src/held.rs:120 | HeldLightTuning.source_radius_m | `0.12` | T |  |

### mw_anim (10: T 3, C 5, ? 2)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/convert.rs:23 | EULER_RATE | `60.0` | T |  |
| src/convert.rs:29 | SNAP_DURATION | `0.1` | T |  |
| src/convert.rs:30 | SNAP_CHORD | `0.05` | ? |  |
| src/convert.rs:31 | SNAP_SPEED_RATIO | `4.0` | T |  |
| src/convert.rs:35 | TAINT | `0.25` | ? |  |

Stays const: EPSILON (layout/math/id), BIPED (sentinel/derived), SWIMS (sentinel/derived), FLIES (sentinel/derived), WALKS (sentinel/derived).

### mw_walkers_bevy (18: T 15, C 1, ? 2)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/physics.rs:145 | DEATH_HANDOFF | `0.4` | T |  |
| src/physics.rs:266 | KNOCKDOWN_IMPULSE | `15.0` | T |  |
| src/physics.rs:271 | HIT_REACT_IMPULSE | `1.0` | T |  |
| src/physics.rs:274 | KNOCKDOWN_HANDOFF | `0.05` | T |  |
| src/physics.rs:532 | TURN | `0.26` | ? |  |
| src/physics.rs:688 | BALL_RADIUS | `0.12` | T |  |
| src/physics.rs:689 | BALL_MASS | `3.0` | T |  |
| src/physics.rs:690 | BALL_SPEED | `12.0` | T |  |
| src/physics.rs:775 | STEP_BUDGET_MS | `0.3` | T |  |
| src/physics.rs:776 | INSERT_BUDGET_MS | `0.5` | T |  |
| src/physics.rs:858 | AUTO_FRAME_DELAY | `0.5` | T |  |
| src/walkers.rs:62 | TIMESCALE | `30.0` | T |  |
| src/walkers.rs:64 | FLY_HOVER | `1.5` | ? |  |
| src/walkers.rs:68 | PROCESS_MARGIN | `4.0` | T |  |
| src/walkers.rs:70 | DORMANT_SIGNIFICANCE | `0.25` | T |  |
| src/walkers.rs:75 | AI_WAKE_RADIUS_M | `110.0` | T |  |
| src/walkers.rs:77 | AI_SLEEP_RADIUS_M | `122.0` | T |  |

Stays const: BALLS_KEPT (small index).

### sf_anim (67: T 53, C 6, ? 8)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/annotate.rs:39 | CONTACT_SPEED | `0.35` | T |  |
| src/annotate.rs:42 | CONTACT_RELEASE | `2.0` | T |  |
| src/annotate.rs:45 | MIN_CONTACT | `0.1` | T |  |
| src/annotate.rs:49 | HAND_BRACE | `0.1` | ? |  |
| src/annotate.rs:53 | HAND_SPEED | `2.0` | T |  |
| src/annotate.rs:57 | HAND_PIVOT | `0.5` | ? |  |
| src/annotate.rs:60 | HAND_PIVOT_RELEASE | `0.8` | T |  |
| src/clip.rs:189 | RAW | `32` | ? |  |
| src/cloth.rs:44 | CLOTH_SUBSTEP | `1.0 / 240.0` | T |  |
| src/cloth.rs:46 | CLOTH_MAX_SUBSTEPS | `16` | T |  |
| src/cloth.rs:162 | GridSpec.rows | `5` | T | settings default |
| src/cloth.rs:162 | GridSpec.cols | `10` | T | settings default |
| src/cloth.rs:162 | GridSpec.ring | `true` | T |  |
| src/cloth.rs:162 | GridSpec.mass | `1.5` | T |  |
| src/cloth.rs:162 | GridSpec.max_distance_scale | `0.6` | T |  |
| src/cloth.rs:293 | ClothParams.damping | `1.0` | T |  |
| src/cloth.rs:294 | ClothParams.gravity | `Vec3::new(0.0, -9.81, 0.0)` | T |  |
| src/cloth.rs:295 | ClothParams.inertia_linear | `0.5` | T |  |
| src/cloth.rs:296 | ClothParams.inertia_angular | `0.3` | T | settings default |
| src/cloth.rs:297 | ClothParams.tether_scale | `1.0` | T |  |
| src/cloth.rs:298 | ClothParams.thickness | `0.015` | T | settings default |
| src/cloth.rs:299 | ClothParams.teleport_distance | `1.0` | T |  |
| src/cloth.rs:300 | ClothParams.teleport_angle | `1.0` | T |  |
| src/cook.rs:53 | CookSettings.error_bound | `1e-4` | T |  |
| src/cook.rs:53 | CookSettings.shell_distance | `1.0` | T |  |
| src/cook.rs:53 | CookSettings.min_rate | `30.0` | T |  |
| src/cook.rs:53 | CookSettings.max_rate | `120.0` | T |  |
| src/cook.rs:105 | RootMotionRules.extract | `true` | T | settings default |
| src/cook.rs:105 | RootMotionRules.horizontal | `true` | T | settings default |
| src/cook.rs:105 | RootMotionRules.vertical | `false` | T | settings default |
| src/cook.rs:105 | RootMotionRules.yaw | `true` | T | settings default |
| src/cook.rs:105 | RootMotionRules.attitude | `false` | T | settings default |
| src/cook.rs:105 | RootMotionRules.anchor_end | `false` | T | settings default |
| src/cook.rs:160 | ContactRules.enabled | `true` | T | settings default |
| src/cook.rs:162 | ContactRules.height | `0.06` | T |  |
| src/cook.rs:163 | ContactRules.speed | `0.35` | T |  |
| src/cook.rs:164 | ContactRules.release | `2.0` | T |  |
| src/cook.rs:165 | ContactRules.lift | `0.06` | T | settings default |
| src/cook.rs:182 | FALLBACK_LEG_LENGTH | `0.85` | ? |  |
| src/foot_metrics.rs:47 | MeterRules.planted | `0.5` | T | settings default |
| src/foot_metrics.rs:47 | MeterRules.settle | `0.1` | T |  |
| src/foot_metrics.rs:47 | MeterRules.sole | `0.0` | T | settings default |
| src/ik.rs:69 | TwoBone.max_extension | `0.9999` | T |  |
| src/ik.rs:69 | TwoBone.weight | `1.0` | T |  |
| src/ik.rs:78 | STRAIGHT | `(0.03, 0.2)` | ? |  |
| src/ik.rs:360 | AnkleShare.share | `0.5` | ? |  |
| src/ik/full_body.rs:49 | RELAXATION | `0.5` | ? |  |
| src/ik/full_body.rs:174 | FullBodySettings.root_compliance | `1.0` | T | settings default |
| src/ik/full_body.rs:175 | FullBodySettings.reach | `0.95` | T |  |
| src/import.rs:52 | SourceSpace.unit_scale | `1.0` | T |  |
| src/import.rs:52 | SourceSpace.right_handed | `true` | ? | bool |
| src/loop_cut.rs:71 | LoopCutRules.min_strides | `1` | T |  |
| src/loop_cut.rs:71 | LoopCutRules.max_strides | `2` | T |  |
| src/loop_cut.rs:71 | LoopCutRules.plateau | `0.9` | T | settings default |
| src/loop_cut.rs:71 | LoopCutRules.blend | `0.12` | T |  |
| src/loop_cut.rs:71 | LoopCutRules.refine | `0.1` | T | settings default |
| src/loop_cut.rs:132 | VELOCITY_WEIGHT | `0.01` | T |  |
| src/loop_cut.rs:134 | IN_PLACE_SPEED | `0.1` | T |  |
| src/retarget.rs:154 | CONTACT_BLEND | `0.15` | T |  |
| src/skeleton.rs:119 | SkeletonRules.detect_root_motion | `false` | T | settings default |
| src/skeleton.rs:119 | SkeletonRules.guess_tags | `true` | T | settings default |

Stays const: CLIP_FORMAT (layout/math/id), NO_JOINT (sentinel/derived), RIG_PROFILE_FORMAT (layout/math/id), NO_PARENT (sentinel/derived), SKELETON_FORMAT (layout/math/id), LN2 (layout/math/id).

### sf_anim_bevy (8: T 6, C 2, ? 0)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lod.rs:58 | A2_INTERVAL | `2` | T |  |
| src/lod.rs:80 | AnimLodSettings.pixels | `[120.0, 50.0, 20.0]` | T |  |
| src/lod.rs:81 | AnimLodSettings.hysteresis | `0.15` | T |  |
| src/lod.rs:83 | AnimLodSettings.cull_offscreen | `true` | T | settings default |
| src/lod.rs:84 | AnimLodSettings.reference_height | `1440.0` | T |  |
| src/lod.rs:176 | AnimBudget.cost_us | `[25.0, 15.0, 8.0, 1.0]` | T |  |

Stays const: AnimBudget.predicted_us (runtime state), AnimBudget.demoted (runtime state).

### sf_anim_graph (24: T 18, C 6, ? 0)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/asset.rs:539 | FootGuards.max_step | `0.5` | T |  |
| src/asset.rs:540 | FootGuards.max_drop | `0.4` | T |  |
| src/asset.rs:541 | FootGuards.max_rise | `0.5` | T |  |
| src/asset.rs:542 | FootGuards.unplant | `0.3` | T | settings default |
| src/asset.rs:544 | FootGuards.max_speed | `8.0` | T |  |
| src/asset.rs:545 | FootGuards.blend_halflife | `0.1` | T |  |
| src/asset.rs:546 | FootGuards.foot_halflife | `0.04` | T |  |
| src/compiled.rs:71 | Provenance.redistributable | `true` | T |  |
| src/instance.rs:48 | GRADE_HALFLIFE | `0.25` | T |  |
| src/procedural.rs:226 | FADE_CUT | `5e-3` | T |  |
| src/procedural.rs:253 | NORMAL_WEIGHT | `0.01` | T |  |
| src/procedural.rs:591 | STEP_EDGE | `0.02` | T |  |
| src/secondary.rs:34 | SPRING_SUBSTEP | `1.0 / 120.0` | T |  |
| src/secondary.rs:36 | SPRING_MAX_SUBSTEPS | `12` | T |  |
| src/secondary.rs:254 | CLOTH_LEG_FOLLOW | `0.7` | T |  |
| src/secondary.rs:257 | CLOTH_MAX_LOD | `1` | T |  |
| src/variety.rs:20 | DEFAULT_RATE_JITTER | `0.05` | T |  |
| src/variety.rs:58 | Variety.rate | `1.0` | T |  |

Stays const: EPS (layout/math/id), GRAPH_FORMAT (layout/math/id), EPS (layout/math/id), LATE_CLIP (sentinel/derived), LOOP_FOREVER (sentinel/derived), Variety.phase (runtime state).

### sf_character (25: T 20, C 4, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/align.rs:79 | UPRIGHT_COS | `std::f32::consts::FRAC_1_SQRT_2` | T |  |
| src/align.rs:83 | DEGENERATE_COS | `0.866` | T |  |
| src/crowd.rs:7 | C0_PIXELS | `20.0` | T |  |
| src/crowd.rs:9 | C1_PIXELS | `4.0` | T |  |
| src/crowd.rs:12 | HYSTERESIS | `0.15` | T |  |
| src/rig.rs:68 | RetargetOptions.ik_legs | `true` | T | settings default |
| src/smooth.rs:155 | SmoothSettings.weld | `0.002` | T | settings default |
| src/smooth.rs:155 | SmoothSettings.link | `0.04` | T | settings default |
| src/smooth.rs:155 | SmoothSettings.overlap | `0.08` | T | settings default |
| src/smooth.rs:155 | SmoothSettings.max_hops | `3` | T |  |
| src/smooth.rs:155 | SmoothSettings.band | `0.08` | T | settings default |
| src/smooth.rs:155 | SmoothSettings.keep_authored | `true` | T | settings default |
| src/smooth.rs:155 | SmoothSettings.iterations | `4000` | T |  |
| src/smooth.rs:155 | SmoothSettings.tolerance | `1e-5` | T |  |
| src/smooth.rs:423 | TransferSettings.max_distance | `0.10` | T |  |
| src/smooth.rs:423 | TransferSettings.max_angle | `35.0` | T |  |
| src/smooth.rs:423 | TransferSettings.weld | `0.002` | T | settings default |
| src/smooth.rs:423 | TransferSettings.iterations | `4000` | T |  |
| src/smooth.rs:423 | TransferSettings.tolerance | `1e-5` | T |  |
| src/smooth_outfit.rs:64 | OutfitSmoothing.guard_tolerance | `0.0005` | T |  |
| src/smooth_outfit.rs:64 | OutfitSmoothing.seam_normals | `true` | ? | bool |

Stays const: BEVY_MORPH_BYTES (layout/math/id), BEVY_JOINT_BYTES (layout/math/id), ClothSpec.attach_offset (layout/math/id), COUNT (sentinel/derived).

### sf_character_bevy (6: T 5, C 1, ? 0)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/cloth.rs:111 | ClothSettings.simulate | `true` | T | settings default |
| src/lib.rs:227 | AnimVarietySettings.enabled | `true` | T | settings default |
| src/lib.rs:395 | SETTLE_FRAMES | `2` | T |  |
| src/view_model.rs:53 | ViewModelSettings.fov_scale | `1.0` | T |  |
| src/view_model.rs:54 | ViewModelSettings.depth | `0.0` | T |  |

Stays const: OutfitCache.capacity (layout/math/id).

## app-diag

### bevyrrowind (3: T 2, C 0, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/perf/cli.rs:91 | GPU_BUSY_PCT | `15.0` | T |  |
| src/perf/cli.rs:369 | OTHER_CPU_LIMIT | `25.0` | T |  |
| src/perf/cli.rs:794 | IDLE_GPU_CLOCK_MHZ | `600.0` | ? |  |

### mw_devtools_bevy (7: T 4, C 0, ? 3)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/live_ab.rs:85 | LiveAb.help | `true` | ? | bool |
| src/live_ab.rs:85 | LiveAb.sun_shadows | `true` | T |  |
| src/live_ab.rs:85 | LiveAb.mesh_wire | `false` | ? | bool |
| src/live_ab.rs:520 | TIME_SCALES | `[30.0, 120.0, 600.0]` | T |  |
| src/perf.rs:32 | DEFAULT_RESOLUTION | `(1280, 720)` | T |  |
| src/pipeline_watch.rs:26 | FLUSH_SECS | `0.5` | T |  |
| src/remote.rs:40 | DEFAULT_PORT | `15712` | ? |  |

### mw_game_core (8: T 5, C 0, ? 3)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/doors.rs:13 | ARRIVAL_EYE_M | `0.5` | T |  |
| src/exterior.rs:9 | CAPTURE_STABLE_FRAMES | `10` | T |  |
| src/exterior.rs:11 | CAPTURE_TIMEOUT_SECS | `120.0` | T |  |
| src/exterior.rs:24 | CLOUD_WORLD_SCALE | `mw_weather::visual::CLOUD_WORLD_SCALE` | T |  |
| src/player.rs:42 | LocoAb.speed_match | `true` | T |  |
| src/player.rs:42 | LocoAb.stride_warp | `true` | ? | bool |
| src/player.rs:42 | LocoAb.lean | `true` | ? | bool |
| src/player.rs:42 | LocoAb.side_lanes | `true` | ? | bool |

### sf_diag (21: T 14, C 4, ? 3)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/ab.rs:89 | OUTLIER_RATIO | `1.3` | T |  |
| src/ab.rs:91 | HITCH_RATIO | `3.0` | T |  |
| src/ab.rs:92 | HITCH_SLACK | `10.0` | T |  |
| src/ab.rs:146 | EXACT_MAX | `20` | T |  |
| src/ab.rs:266 | ALPHA | `0.05` | ? |  |
| src/gate.rs:14 | STREAMING_STUTTERS_PER_MIN | `1.0` | T |  |
| src/path.rs:16 | SAMPLES_PER_SEGMENT | `256` | T |  |
| src/path.rs:19 | MIN_KEY_SPACING | `1e-6` | T |  |
| src/report.rs:20 | MAX_SPIKES | `32` | T |  |
| src/report.rs:23 | MAX_CULPRITS | `5` | T |  |
| src/report.rs:100 | Tolerance.relative | `0.10` | ? |  |
| src/report.rs:101 | Tolerance.absolute_ms | `0.25` | T |  |
| src/report.rs:169 | MAX_HITCHES | `64` | T |  |
| src/report.rs:172 | MAX_CHURN | `12` | T |  |
| src/scenario.rs:23 | Warmup.seconds | `2.0` | ? |  |
| src/scenario.rs:24 | Warmup.frames | `30` | T |  |
| src/views.rs:71 | PHASE_METRIC_COUNT | `9` | T |  |

Stays const: UNSET (sentinel/derived), STUTTER_WINDOW (small index), STUTTER_MIN_REFERENCE (small index), REPORT_FORMAT (layout/math/id).

### sf_diag_bevy (15: T 10, C 3, ? 2)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:138 | DiagSettings.update_baseline | `false` | T | settings default |
| src/lib.rs:139 | DiagSettings.exit_on_finish | `false` | T | settings default |
| src/lib.rs:140 | DiagSettings.apply_transform | `true` | T | settings default |
| src/lib.rs:144 | DiagSettings.markdown | `true` | T | settings default |
| src/pipeline_watch.rs:57 | BURST_SECS | `2.0` | T |  |
| src/stages.rs:131 | CLEANUP | `SETS.len() - 1` | ? |  |
| src/textures.rs:47 | SCAN_SECS | `4.0` | T |  |
| src/textures.rs:49 | SAMPLE_LAG_SECS | `0.5` | T |  |
| src/textures.rs:51 | MIN_PRINT_SECS | `10.0` | T |  |
| src/textures.rs:53 | CHANGE_SHARE | `0.05` | ? |  |
| src/warmup.rs:28 | AFTER_REVEAL_SECS | `10.0` | T |  |
| src/warmup.rs:280 | COMPILE_BUCKETS_MS | `[1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0...` | T |  |

Stays const: EXAMPLES (small index), UNSET (sentinel/derived), SLOTS (layout/math/id).

## audio

### mw_audio (9: T 8, C 0, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:34 | UNIT_M | `0.9144 / 64.0` | T |  |
| src/lib.rs:58 | RangeScale.min_mult | `20.0` | T |  |
| src/lib.rs:58 | RangeScale.max_mult | `50.0` | T |  |
| src/lib.rs:58 | RangeScale.default_min | `5.0` | T |  |
| src/lib.rs:58 | RangeScale.default_max | `40.0` | T |  |
| src/lib.rs:366 | MusicMood.battle_hold_s | `5.0` | T |  |
| src/voice.rs:237 | VoiceRules.greet_duration_s | `4.0` | T |  |
| src/voice.rs:238 | VoiceRules.idle_odds | `10.0` | T | settings default |
| src/voice.rs:379 | ENVELOPE_HZ | `20.0` | ? |  |

### mw_audio_bevy (6: T 5, C 0, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:48 | STEP_RANGE_M | `15.0` | T |  |
| src/lib.rs:50 | STEP_JUMP_M | `3.0` | T |  |
| src/lib.rs:425 | START_GRACE_S | `0.5` | T |  |
| src/lib.rs:427 | IDLE_SPEED_MPS | `0.2` | T |  |
| src/lib.rs:429 | EYE_M | `1.6` | T |  |
| src/lib.rs:431 | ENVELOPE_CACHE | `256` | ? |  |

### sf_audio (26: T 21, C 1, ? 4)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/attenuation.rs:27 | Attenuation.min_m | `1.0` | T |  |
| src/attenuation.rs:27 | Attenuation.max_m | `40.0` | T |  |
| src/group.rs:42 | SoundGroup.volume | `[1.0, 1.0]` | T |  |
| src/group.rs:43 | SoundGroup.pitch | `[1.0, 1.0]` | ? |  |
| src/group.rs:44 | SoundGroup.max_instances | `0` | T |  |
| src/impacts.rs:146 | ImpactParams.min_speed | `0.5` | T |  |
| src/impacts.rs:147 | ImpactParams.full_speed | `6.0` | T |  |
| src/impacts.rs:148 | ImpactParams.min_volume | `0.15` | T |  |
| src/impacts.rs:149 | ImpactParams.cooldown_s | `0.1` | T |  |
| src/impacts.rs:150 | ImpactParams.override_ratio | `2.0` | T |  |
| src/impacts.rs:151 | ImpactParams.max_per_tick | `8` | T |  |
| src/impacts.rs:152 | ImpactParams.slide_min | `0.4` | T |  |
| src/impacts.rs:153 | ImpactParams.slide_full | `3.0` | T | settings default |
| src/impacts.rs:154 | ImpactParams.slide_min_level | `0.3` | T | settings default |
| src/impacts.rs:155 | ImpactParams.slide_hold_s | `0.12` | T |  |
| src/impacts.rs:156 | ImpactParams.slide_fade_s | `0.1` | T |  |
| src/lib.rs:48 | COUNT | `8` | T |  |
| src/lib.rs:97 | Mixer.outdoor | `1.0` | ? |  |
| src/lib.rs:148 | FADE_S | `2.0` | T |  |
| src/lib.rs:150 | OUTDOOR_FADE_S | `0.5` | T |  |
| src/music.rs:10 | MUSIC_FADE_S | `3.0` | T |  |
| src/volume.rs:12 | VOLUME_RANGE_DB | `60.0` | T |  |
| src/volume.rs:15 | VOLUME_TAPER | `0.1` | T |  |
| src/volume.rs:60 | BusVolume.level | `1.0` | ? |  |
| src/volume.rs:60 | BusVolume.muted | `false` | ? | bool |

Stays const: SoundGroup.no_repeat (layout/math/id).

### sf_audio_bevy (2: T 2, C 0, ? 0)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/contacts.rs:37 | MIN_LEVEL | `1e-4` | T |  |
| src/lib.rs:47 | MAX_ONE_SHOTS | `64` | T |  |

## effects

### mw_effects (49: T 5, C 44, ? 0)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/mgef.rs:64 | EFFECT_COUNT | `143` | T |  |
| src/mgef.rs:220 | FIRE_DAMAGE | `14` | T |  |
| src/mgef.rs:221 | SHOCK_DAMAGE | `15` | T |  |
| src/mgef.rs:222 | FROST_DAMAGE | `16` | T |  |
| src/source.rs:26 | FOOT_M | `0.3048` | T |  |

Stays const: TARGET_SKILL (sentinel/derived), TARGET_ATTRIBUTE (sentinel/derived), NO_DURATION (sentinel/derived), NO_MAGNITUDE (sentinel/derived), HARMFUL (sentinel/derived), CONTINUOUS_VFX (sentinel/derived), CAST_SELF (sentinel/derived), CAST_TOUCH (sentinel/derived), CAST_TARGET (sentinel/derived), ALLOW_SPELLMAKING (sentinel/derived), ALLOW_ENCHANTING (sentinel/derived), NEGATIVE_LIGHT (sentinel/derived), APPLIED_ONCE (sentinel/derived), STEALTH (sentinel/derived), NON_RECASTABLE (sentinel/derived), ILLEGAL_DAEDRA (sentinel/derived), UNREFLECTABLE (sentinel/derived), CASTER_LINKED (sentinel/derived), RECORD_MASK (layout/math/id), FIRE_SHIELD (small index), LIGHTNING_SHIELD (small index), FROST_SHIELD (small index), POISON (small index), WEAKNESS_TO_FIRE (small index), WEAKNESS_TO_FROST (small index), WEAKNESS_TO_SHOCK (small index), WEAKNESS_TO_MAGICKA (small index), WEAKNESS_TO_COMMON_DISEASE (small index), WEAKNESS_TO_CORPRUS_DISEASE (small index), WEAKNESS_TO_POISON (small index), SANCTUARY (small index), PARALYZE (small index), SILENCE (small index), SOUND (small index), RESIST_FIRE (id table), RESIST_FROST (id table), RESIST_SHOCK (id table), RESIST_MAGICKA (id table), RESIST_COMMON_DISEASE (id table), RESIST_CORPRUS_DISEASE (id table), RESIST_POISON (id table), RESIST_PARALYSIS (id table), CORPRUS (id table), VAMPIRISM (id table).

### sf_effects (7: T 6, C 0, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/active.rs:15 | MAX_EFFECTS_PER_SOURCE | `64` | T |  |
| src/active.rs:37 | EffectIdAlloc.next | `1` | ? |  |
| src/standard.rs:39 | CostConsts.scale | `0.1` | T |  |
| src/standard.rs:39 | CostConsts.area_unit | `5.0` | T |  |
| src/standard.rs:39 | CostConsts.skill_discount | `0.005` | T |  |
| src/standard.rs:39 | CostConsts.skill_cap | `100.0` | T |  |
| src/standard.rs:218 | StandardEffectRules.item_uses | `10.0` | T | settings default |

## feel-camera-combat

### mw_combat (4: T 0, C 0, ? 4)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:37 | STAMINA_PER_FATIGUE | `0.5` | ? |  |
| src/lib.rs:44 | DEFAULT_ATTACK | `10.0` | ? |  |
| src/lib.rs:48 | GUARD_ABSORB | `0.65` | ? |  |
| src/lib.rs:51 | GUARD_STABILITY | `1.0` | ? |  |

### mw_player_bevy (37: T 31, C 1, ? 5)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/avatar.rs:73 | TRAVERSAL_BLEND_IN | `0.12` | T |  |
| src/avatar.rs:74 | TRAVERSAL_BLEND_OUT | `0.25` | T |  |
| src/avatar.rs:76 | CLIP_BLEND_IN | `0.10` | T |  |
| src/avatar.rs:79 | TELEPORT_M | `3.0` | T |  |
| src/avatar.rs:83 | HIDE_FADE | `0.5` | T |  |
| src/avatar.rs:85 | AIR_GRACE | `0.1` | T |  |
| src/avatar.rs:87 | FALL_SPEED | `-2.0` | T |  |
| src/combat.rs:76 | UNSHEETED_VITALS | `[100.0, 100.0]` | ? |  |
| src/combat.rs:371 | STANCE_BLEND_IN | `0.12` | T |  |
| src/combat.rs:372 | STANCE_BLEND_OUT | `0.2` | T |  |
| src/dummy.rs:54 | DUMMY_HEALTH | `100.0` | ? |  |
| src/dummy.rs:56 | DUMMY_FATIGUE | `200.0` | ? |  |
| src/feel.rs:19 | ACCEL_HALFLIFE | `0.07` | T |  |
| src/feel.rs:21 | BRAKE_HALFLIFE | `0.05` | T |  |
| src/feel.rs:23 | TURN_WALK_DEG | `720.0` | T |  |
| src/feel.rs:24 | TURN_RUN_DEG | `1080.0` | T |  |
| src/feel.rs:25 | TURN_SPRINT_DEG | `480.0` | T |  |
| src/feel.rs:26 | TURN_SNEAK_DEG | `540.0` | T |  |
| src/feel.rs:29 | FACING_HALFLIFE | `0.1` | T |  |
| src/feel.rs:33 | MEASURE_HALFLIFE | `0.025` | T |  |
| src/feel.rs:38 | GRAVITY | `18.0` | T |  |
| src/lib.rs:103 | LOOK_RAD_PER_COUNT | `0.0025` | T |  |
| src/lib.rs:105 | STICK_LOOK_RAD_PER_S | `2.6` | T |  |
| src/lib.rs:107 | DEFAULT_ACTIVATE_UNITS | `192.0` | ? |  |
| src/lib.rs:110 | FALL_LIMIT_M | `200.0` | T |  |
| src/lib.rs:113 | ARRIVAL_GROUND_MARGIN_M | `3.0` | T |  |
| src/lib.rs:116 | ARRIVAL_WAIT_MAX_SECS | `10.0` | T |  |
| src/lib.rs:119 | PLACE_ABOVE_M | `1.0` | T |  |
| src/lib.rs:121 | PLACE_BELOW_M | `5.0` | T |  |
| src/lib.rs:123 | GROUND_OFFSET_M | `0.02` | T |  |
| src/lib.rs:126 | SHAPE_RAY_TOLERANCE_M | `0.5` | T |  |
| src/lib.rs:129 | INTERIOR_VOID_MARGIN_M | `2.0` | T |  |
| src/lib.rs:974 | FLOOR_SEARCH_M | `200.0` | T |  |
| src/lib.rs:976 | CEILING_M | `60.0` | T |  |
| src/lib.rs:983 | STEP_M | `0.05` | T |  |
| src/vitals.rs:248 | FATIGUE_REGEN | `10.0` | ? |  |

Stays const: FIRST_DUMMY (layout/math/id).

### sf_action (47: T 20, C 5, ? 22)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/brace.rs:51 | BRACE_FAN | `[0.0, 0.61, 1.22, std::f32::consts::F...` | ? |  |
| src/brace.rs:53 | BRACE_PALM_GAP | `0.03` | ? |  |
| src/brace.rs:55 | BRACE_BEND | `0.9` | ? |  |
| src/brace.rs:57 | BRACE_STRAIGHT | `0.98` | ? |  |
| src/brace.rs:59 | BRACE_ELBOW_ROOM | `0.02` | ? |  |
| src/brace.rs:61 | BRACE_LOOKAHEAD | `0.25` | T |  |
| src/brace.rs:63 | BRACE_BAND | `0.1` | ? |  |
| src/brace.rs:65 | BRACE_LIGHT | `0.5` | ? |  |
| src/brace.rs:67 | BRACE_FIRM_SPEED | `2.5` | T |  |
| src/brace.rs:69 | BRACE_MIN_SPEED | `0.4` | T |  |
| src/brace.rs:71 | BRACE_RISE | `6.0` | ? |  |
| src/brace.rs:72 | BRACE_FALL | `2.0` | ? |  |
| src/brace.rs:74 | BRACE_HALFLIFE | `0.05` | T |  |
| src/brace.rs:77 | BRACE_LEAD | `(1.5, 1.6)` | T |  |
| src/brace.rs:84 | BRACE_HEAD_ON | `(0.4, 0.9)` | ? |  |
| src/brace.rs:86 | BRACE_SPREAD | `0.68` | T |  |
| src/brace.rs:88 | BRACE_CHEST | `0.92` | ? |  |
| src/brace.rs:91 | BRACE_TUCK | `0.55` | ? |  |
| src/brace.rs:93 | BRACE_CONFIRM | `(0.2, 0.08)` | ? |  |
| src/brace.rs:95 | BRACE_MAX_NORMAL_Y | `0.3` | ? |  |
| src/brace.rs:99 | PREFER | `Vec3::new(0.8, -1.0, 0.3)` | ? |  |
| src/command.rs:352 | REPEAT_MISSING_TICKS | `6` | ? |  |
| src/grip.rs:21 | GRIP_LATERAL | `(0.18, 0.32)` | ? |  |
| src/grip.rs:23 | GRIP_INSET | `0.04` | ? |  |
| src/grip.rs:25 | GRIP_RISE | `0.03` | ? |  |
| src/grip.rs:27 | GRIP_MAX_GAP | `0.5` | ? |  |
| src/grip.rs:29 | GRIP_RAMP | `0.10` | ? |  |
| src/ids.rs:6 | TICK_HZ | `60` | ? |  |
| src/state.rs:143 | ModeProfile.speed_scale | `1.0` | T |  |
| src/state.rs:143 | ModeProfile.jump_scale | `1.0` | T |  |
| src/state.rs:205 | CharacterParams.gravity | `9.81` | T |  |
| src/state.rs:206 | CharacterParams.jump_height | `1.0` | T |  |
| src/state.rs:207 | CharacterParams.coyote_ticks | `6` | T | settings default |
| src/state.rs:208 | CharacterParams.jump_buffer_ticks | `6` | T | settings default |
| src/state.rs:209 | CharacterParams.deadzone | `0.15` | T | settings default |
| src/state.rs:210 | CharacterParams.run_threshold | `0.6` | T |  |
| src/state.rs:211 | CharacterParams.height | `1.8` | T |  |
| src/state.rs:212 | CharacterParams.radius | `0.3` | T |  |
| src/step.rs:22 | MAX_SETTLE | `4` | T |  |
| src/step.rs:25 | FORWARD_COS | `0.5` | T |  |
| src/step.rs:31 | TRAVERSE_INPUT_COS | `0.422_618_27` | T |  |
| src/step.rs:756 | VAULT_REACH_SLACK | `0.3` | T |  |

Stays const: BRACE_FINGERS_UP (layout/math/id), CLASSES (small index), LEFT (small index), RIGHT (layout/math/id), RATE_ONE (layout/math/id).

### sf_action_bevy (2: T 2, C 0, ? 0)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:297 | OBSTACLE_RANGE_M | `1.5` | T |  |
| src/lib.rs:338 | SAME_FACE_COS | `0.985` | T |  |

### sf_camera (39: T 34, C 1, ? 4)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/collision.rs:28 | CollisionParams.radius | `0.2` | T |  |
| src/collision.rs:28 | CollisionParams.margin | `0.05` | T |  |
| src/collision.rs:28 | CollisionParams.out_halflife | `0.35` | T |  |
| src/collision.rs:28 | CollisionParams.fade_start | `0.9` | T |  |
| src/collision.rs:28 | CollisionParams.fade_end | `0.45` | T |  |
| src/comfort.rs:47 | ComfortSettings.shake | `true` | T | settings default |
| src/comfort.rs:48 | ComfortSettings.shake_strength | `1.0` | T |  |
| src/comfort.rs:49 | ComfortSettings.head_bob | `false` | T | settings default |
| src/comfort.rs:50 | ComfortSettings.auto_camera | `false` | T | settings default |
| src/comfort.rs:51 | ComfortSettings.fov_world | `60.0` | T | settings default |
| src/comfort.rs:52 | ComfortSettings.fov_body | `50.0` | T | settings default |
| src/comfort.rs:53 | ComfortSettings.view_model | `false` | T | settings default |
| src/comfort.rs:54 | ComfortSettings.view_model_depth | `0.8` | T |  |
| src/comfort.rs:55 | ComfortSettings.centre_dot | `false` | T | settings default |
| src/comfort.rs:56 | ComfortSettings.motion_blur | `false` | T | settings default |
| src/comfort.rs:89 | ShakeParams.frequency | `18.0` | T | settings default |
| src/comfort.rs:89 | ShakeParams.decay | `1.6` | T | settings default |
| src/lockon.rs:47 | LockParams.acquire_range | `15.0` | T |  |
| src/lockon.rs:49 | LockParams.break_range | `20.0` | T |  |
| src/lockon.rs:50 | LockParams.los_timeout | `1.5` | T |  |
| src/lockon.rs:51 | LockParams.reacquire_window | `1.0` | T | settings default |
| src/lockon.rs:52 | LockParams.angle_weight | `0.6` | T |  |
| src/lockon.rs:56 | LockParams.framing_halflife | `0.12` | T |  |
| src/lockon.rs:101 | SoftAimParams.reach | `3.0` | T |  |
| src/orbit.rs:34 | ModeDistances.normal | `3.2` | ? |  |
| src/orbit.rs:34 | ModeDistances.sneak | `2.4` | ? |  |
| src/orbit.rs:34 | ModeDistances.free_run | `4.0` | ? |  |
| src/orbit.rs:34 | ModeDistances.locked | `3.6` | ? |  |
| src/orbit.rs:93 | OrbitParams.pivot_height | `1.45` | T |  |
| src/orbit.rs:98 | OrbitParams.distance_halflife | `0.25` | T |  |
| src/orbit.rs:99 | OrbitParams.pivot_halflife_xz | `0.06` | T |  |
| src/orbit.rs:100 | OrbitParams.pivot_halflife_y | `0.12` | T |  |
| src/orbit.rs:101 | OrbitParams.max_lag | `2.0` | T |  |
| src/orbit.rs:102 | OrbitParams.swap_halflife | `0.12` | T |  |
| src/orbit.rs:222 | RecentreParams.delay | `1.5` | T |  |
| src/orbit.rs:222 | RecentreParams.min_speed | `1.0` | T |  |
| src/orbit.rs:222 | RecentreParams.halflife | `0.8` | T |  |
| src/view_model.rs:50 | MAX_DEPTH | `0.99` | T |  |

Stays const: OrbitParams.shoulder_offset (layout/math/id).

### sf_camera_bevy (6: T 5, C 1, ? 0)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:114 | Lockable.aim_height | `1.3` | T |  |
| src/lib.rs:131 | FirstPersonParams.eye_height | `1.62` | T |  |
| src/lib.rs:146 | ZoomParams.min | `0.5` | T | settings default |
| src/lib.rs:146 | ZoomParams.max | `1.5` | T | settings default |
| src/lib.rs:146 | ZoomParams.step | `0.1` | T |  |

Stays const: FirstPersonParams.eye_forward (layout/math/id).

### sf_combat (13: T 10, C 0, ? 3)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/fighter.rs:116 | SPRINT_RESUME | `0.15` | ? |  |
| src/rules.rs:167 | StandardRules.regen_per_s | `45.0` | T |  |
| src/rules.rs:167 | StandardRules.guard_regen | `0.5` | T | settings default |
| src/rules.rs:175 | MIN_STABILITY | `0.1` | T |  |
| src/shape.rs:10 | SWEEP_STEPS | `5` | T |  |
| src/shape.rs:33 | RADIUS_SCALE | `0.8` | T |  |
| src/shape.rs:34 | HEAD_RADIUS | `0.12` | T |  |
| src/shape.rs:36 | HEAD_HEIGHT | `0.93` | T |  |
| src/sheet.rs:177 | HitStopDef.hold | `0.06` | T |  |
| src/sheet.rs:177 | HitStopDef.ease | `0.10` | ? |  |
| src/sheet.rs:193 | Defaults.turn | `360.0` | ? |  |
| src/sheet.rs:193 | Defaults.blend_in | `0.10` | T |  |
| src/step.rs:268 | HOSTILE_RANGE | `20.0` | T |  |

### sf_combat_bevy (4: T 3, C 0, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lock.rs:66 | LockConfig.eye_height | `1.5` | T |  |
| src/lock.rs:66 | LockConfig.los_radius | `0.1` | T |  |
| src/marker.rs:34 | MARKER_SIZE | `8.0` | T |  |
| src/marker.rs:36 | MARKER_RIM | `1.0` | ? |  |

### sf_controller (44: T 37, C 4, ? 3)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/avoid.rs:99 | AvoidParams.time_horizon | `2.0` | T | settings default |
| src/avoid.rs:100 | AvoidParams.neighbour_dist | `5.0` | T |  |
| src/avoid.rs:101 | AvoidParams.max_neighbours | `8` | T |  |
| src/avoid.rs:102 | AvoidParams.vertical_gap | `1.5` | T | settings default |
| src/avoid.rs:103 | AvoidParams.yield_weight | `9.0` | T |  |
| src/avoid.rs:104 | AvoidParams.idle_speed | `0.1` | T |  |
| src/avoid.rs:105 | AvoidParams.side_bias | `0.15` | T |  |
| src/avoid.rs:106 | AvoidParams.margin | `0.05` | T |  |
| src/context.rs:203 | DROP_SAMPLES | `4` | T |  |
| src/context.rs:205 | NUDGE | `0.05` | ? |  |
| src/context.rs:207 | RUNOUT_LIFT | `0.1` | ? |  |
| src/correction.rs:38 | CorrectionParams.halflife | `0.1` | T |  |
| src/correction.rs:39 | CorrectionParams.yaw_halflife | `0.2` | T |  |
| src/correction.rs:40 | CorrectionParams.speed_ratio | `0.5` | T |  |
| src/correction.rs:41 | CorrectionParams.min_speed | `0.3` | T |  |
| src/correction.rs:42 | CorrectionParams.max_yaw_rate | `3.0` | T |  |
| src/correction.rs:43 | CorrectionParams.max_distance | `0.5` | T |  |
| src/motion.rs:86 | SpringParams.halflife_velocity | `0.27` | T |  |
| src/motion.rs:86 | SpringParams.halflife_facing | `0.27` | T |  |
| src/motion.rs:134 | STILL_SPEED | `1e-3` | T |  |
| src/motion.rs:138 | PREDICT_STEP | `1.0 / 60.0` | T |  |
| src/mover.rs:26 | DEFLECT_FACING | `0.1` | T |  |
| src/mover.rs:29 | SLIDE_STOP_S | `0.1` | T |  |
| src/mover.rs:70 | MoverParams.halflife_height | `0.08` | T |  |
| src/mover.rs:71 | MoverParams.radius | `0.3` | T |  |
| src/mover.rs:72 | MoverParams.height | `1.8` | T |  |
| src/mover.rs:74 | MoverParams.step_down | `0.45` | T |  |
| src/mover.rs:75 | MoverParams.max_slope_deg | `46.0` | T |  |
| src/mover.rs:76 | MoverParams.skin | `0.01` | T | settings default |
| src/mover.rs:77 | MoverParams.wall_stop_deg | `30.0` | T |  |
| src/mover.rs:78 | MoverParams.gravity | `9.81` | T |  |
| src/mover.rs:79 | MoverParams.max_fall_speed | `50.0` | T |  |
| src/mover.rs:80 | MoverParams.ground_radius | `0.0` | T |  |
| src/mover.rs:81 | MoverParams.swim_depth | `1.35` | T |  |
| src/path.rs:40 | ArrivalParams.walk_deceleration | `6.0` | T |  |
| src/path.rs:40 | ArrivalParams.run_deceleration | `3.5` | T |  |
| src/path.rs:40 | ArrivalParams.cutoff_speed | `0.1` | T |  |
| src/steps.rs:41 | CollisionSettings.lod | `1` | T |  |
| src/steps.rs:41 | CollisionSettings.weld | `1e-3` | T | settings default |
| src/trajectory.rs:13 | TRAJECTORY_TIMES | `[0.2, 0.4, 0.6, 0.8, 1.0]` | ? |  |

Stays const: EPS (layout/math/id), COLUMN (small index), MoverParams.step_up (layout/math/id), COLLISION_FORMAT (layout/math/id).

### sf_controller_bevy (12: T 11, C 1, ? 0)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:151 | MEASURE_HALFLIFE | `0.05` | T |  |
| src/lib.rs:204 | Avoider.priority | `0.0` | T |  |
| src/lib.rs:206 | Avoider.max_speed | `0.0` | T |  |
| src/lib.rs:296 | ControllerSettings.enabled | `true` | T | settings default |
| src/lib.rs:297 | ControllerSettings.avoidance | `true` | T | settings default |
| src/lib.rs:299 | ControllerSettings.max_dt | `0.1` | T |  |
| src/lib.rs:300 | ControllerSettings.moving_speed | `0.3` | T |  |
| src/lib.rs:301 | ControllerSettings.idle_speed | `0.15` | T |  |
| src/lib.rs:302 | ControllerSettings.stop_speed | `0.25` | T |  |
| src/lib.rs:306 | ControllerSettings.foot_probe_down | `0.75` | T |  |
| src/lib.rs:307 | ControllerSettings.foot_probes | `true` | T |  |

Stays const: ControllerSettings.foot_probe_up (layout/math/id).

## other

### mw_assets (23: T 12, C 1, ? 10)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/animation.rs:41 | MATERIAL_ANIM_HZ | `30.0` | ? |  |
| src/cooked.rs:30 | RAY_TRACING_LOD | `0` | T |  |
| src/cooked.rs:75 | CookedLodSettings.max_distance | `2000.0` | T |  |
| src/cooked.rs:75 | CookedLodSettings.tint_levels | `false` | T | settings default |
| src/lib.rs:51 | SCENE_EV100 | `7.0` | ? |  |
| src/lib.rs:466 | LEAF_DIFFUSE_TRANSMISSION | `0.5` | ? |  |
| src/lib.rs:472 | LEAF_THICKNESS | `0.05` | ? |  |
| src/lights.rs:77 | FULL_MOONLIGHT_FLOOR_RATIO | `2.5` | T |  |
| src/lights.rs:81 | LightCalibration.lumens_per_radius_sq | `8_000.0` | T |  |
| src/lights.rs:81 | LightCalibration.range_scale | `4.0` | T |  |
| src/lights.rs:81 | LightCalibration.unshadowed_range_scale | `4.0` | T |  |
| src/lights.rs:81 | LightCalibration.ambient_nits | `300.0` | ? |  |
| src/lights.rs:81 | LightCalibration.exposure_floor_ev100 | `3.5` | T |  |
| src/lights.rs:81 | LightCalibration.night_ambient_nits | `4.0` | ? |  |
| src/lights.rs:144 | SHADOW_NEAR_Z | `0.25` | T |  |
| src/map_ab.rs:29 | PLAIN_METALLIC | `0.0` | ? |  |
| src/map_ab.rs:31 | PLAIN_ROUGHNESS | `0.85` | ? |  |
| src/map_ab.rs:33 | PLAIN_REFLECTANCE | `0.3` | ? |  |
| src/map_ab.rs:40 | PARALLAX_DEPTH_SCALE | `0.04` | T |  |
| src/particles.rs:108 | BUFFER_CHUNK | `1024` | ? |  |
| src/particles.rs:432 | CARRY_MAX_STEP_M | `4.0` | T |  |
| src/particles.rs:573 | PARTICLE_REACH_M | `12.0` | T |  |

Stays const: UP (layout/math/id).

### mw_bake (53: T 21, C 21, ? 11)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/census.rs:29 | MIN_EXTENT | `0.25` | T |  |
| src/census.rs:31 | MAX_BANDS | `4` | T |  |
| src/census.rs:33 | BIN | `0.05` | ? |  |
| src/census.rs:35 | MERGE_GAP | `0.10` | ? |  |
| src/census.rs:37 | MAX_SPAN | `0.15` | T |  |
| src/census.rs:40 | MIN_FILL | `0.3` | T |  |
| src/census.rs:42 | MIN_HEIGHT | `0.1` | T |  |
| src/census.rs:44 | MIN_SHARE | `0.03` | T |  |
| src/census.rs:46 | FLAT | `0.9` | ? |  |
| src/census.rs:48 | SURFACE_BIN | `0.02` | ? |  |
| src/cook.rs:83 | DEPTH_MAX_SIZE | `1024` | T |  |
| src/delight.rs:62 | DEFAULT_STRENGTH | `delight::DEFAULT_STRENGTH` | T |  |
| src/delight.rs:64 | MIN_SIDE | `64` | T |  |
| src/far.rs:36 | IMPOSTOR_MIN_HEIGHT | `3.0` | T |  |
| src/importer.rs:43 | DROP_FRACTION | `0.02` | T |  |
| src/nav.rs:63 | NAV_CELL | `64.0` | ? |  |
| src/nav.rs:65 | INTERIOR_PAD | `2.0` | ? |  |
| src/nav.rs:67 | DOOR_HALF_DEPTH | `0.75` | T |  |
| src/nav.rs:69 | DOOR_FLOOR_SLACK | `0.5` | T |  |
| src/nav.rs:71 | APPROACH_RADIUS | `6.0` | T |  |
| src/nav.rs:73 | DEFAULT_SWIM_HEIGHT_SCALE | `0.9` | T |  |
| src/nav.rs:75 | TERRAIN_SPACING | `sf_terrain::BASE_SPACING` | T |  |
| src/nav.rs:432 | HEIGHT_BUDGET | `128 << 20` | T |  |
| src/nav.rs:433 | MESH_BUDGET | `128 << 20` | T |  |
| src/nav.rs:434 | CELL_BUDGET | `32 << 20` | T |  |
| src/nav.rs:550 | GeometryCache.cells | `Decoded::new(CELL_BUDGET)` | ? |  |
| src/nav.rs:551 | GeometryCache.meshes | `Decoded::new(MESH_BUDGET)` | ? |  |
| src/nav.rs:552 | GeometryCache.heights | `Decoded::new(HEIGHT_BUDGET)` | T |  |
| src/textures.rs:54 | DDS_DATA | `128` | ? |  |
| src/textures.rs:56 | DX10_DATA | `148` | ? |  |
| src/textures.rs:149 | D3DFMT_A16B16G16R16 | `36` | ? |  |
| src/world.rs:45 | LAYER_ARRAY_SIZE | `2048` | T |  |

Stays const: GRAPH_RULES_VERSION (layout/math/id), VERSION (layout/math/id), VERSION (layout/math/id), RULES_VERSION (layout/math/id), VERSION (layout/math/id), DEPTH_VERSION (layout/math/id), VERSION (layout/math/id), ALGORITHM_VERSION (layout/math/id), ALPHA_CLASSIFIER_VERSION (layout/math/id), RULE_VERSION (layout/math/id), RULES_VERSION (layout/math/id), VERSION (layout/math/id), RULES_VERSION (layout/math/id), RULES_VERSION (layout/math/id), PALETTE_LEN (layout/math/id), DDPF_ALPHAPIXELS (sentinel/derived), DDPF_ALPHA (sentinel/derived), DDPF_FOURCC (sentinel/derived), DDPF_PALETTEINDEXED8 (sentinel/derived), DDPF_RGB (sentinel/derived), DDPF_LUMINANCE (sentinel/derived).

### mw_graphs (9: T 8, C 0, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/build.rs:173 | RATE_CLAMP | `(0.8, 1.25)` | T |  |
| src/build.rs:179 | SPEED_SMOOTHING | `0.05` | T |  |
| src/build.rs:181 | LOCK_BLEND | `0.1` | T |  |
| src/build.rs:255 | TURN_THRESHOLD | `0.5` | T |  |
| src/build.rs:258 | UNPLANT_ANGLE_DEG | `20.0` | T |  |
| src/build.rs:383 | DIAG | `std::f32::consts::FRAC_1_SQRT_2` | ? |  |
| src/build.rs:421 | GAIT_BLEND | `0.15` | T |  |
| src/build.rs:424 | DIRECTION_SMOOTHING | `0.05` | T |  |
| src/script.rs:24 | SCRIPT_PRIORITY | `100` | T |  |

### mw_nif (23: T 6, C 10, ? 7)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/anim.rs:43 | Timing.frequency | `1.0` | ? |  |
| src/anim.rs:43 | Timing.start | `0.0` | ? |  |
| src/anim.rs:43 | Timing.stop | `0.0` | ? |  |
| src/material.rs:80 | Material.glossiness | `0.0` | ? |  |
| src/material.rs:81 | Material.alpha | `1.0` | ? |  |
| src/material.rs:85 | Material.unlit_lighting | `false` | ? | bool |
| src/material.rs:87 | Material.double_sided | `false` | ? | bool |
| src/material.rs:88 | Material.depth_test | `true` | T |  |
| src/material.rs:89 | Material.depth_write | `true` | T |  |
| src/material.rs:164 | CUTOUT_CUTOFF | `0.5` | T |  |
| src/particles.rs:24 | GRAVITY_SCALE | `1.6` | T |  |
| src/particles.rs:201 | WINDOW_SLACK | `0.05` | T |  |
| src/particles.rs:276 | STEP | `1.0 / 30.0` | T |  |

Stays const: Timing.phase (runtime state), UNIT_METERS (unit/format), ANIM_FLAG_AUTO_PLAY (sentinel/derived), ANIM_FLAG_NOT_RANDOM (sentinel/derived), PARTICLE_FLAG_LOCAL_SPACE (sentinel/derived), BSP_ARRAY_FLAG_AT_NODE (sentinel/derived), BASE (small index), DARK (small index), DETAIL (small index), GLOW (small index).

### mw_rpg (14: T 3, C 7, ? 4)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/barter.rs:65 | MERCANTILE_SUCCESS | `0` | ? |  |
| src/barter.rs:433 | BarterOptions.cap_75 | `true` | T | settings default |
| src/filter.rs:44 | ActorFacts.npc | `false` | ? | bool |
| src/filter.rs:51 | ActorFacts.rank | `-1` | ? |  |
| src/filter.rs:52 | ActorFacts.female | `false` | ? | bool |
| src/training.rs:46 | TRAINING_HOURS | `2` | T |  |
| src/training.rs:61 | TrainingOptions.base_skill | `true` | T | settings default |

Stays const: MERCANTILE (small index), PERSONALITY (small index), ActorFacts.id (layout/math/id), MERCANTILE (small index), SPEECHCRAFT (small index), SALT (sentinel/derived), TRAINER_SKILLS (small index).

### mw_rpg_bevy (7: T 0, C 3, ? 4)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/cast.rs:42 | CAST_SEED_SALT | `1 << 31` | ? |  |
| src/dialogue/service_list_screen.rs:51 | ROW_EXTENT | `1.0` | ? |  |
| src/dialogue/service_list_screen.rs:52 | VIEWPORT_ROWS | `8.0` | ? |  |
| src/effects.rs:342 | ATTRIBUTE | `40` | ? |  |

Stays const: MERCANTILE (small index), EFFECTS_BLOB_VERSION (layout/math/id), STATS_BLOB_VERSION (layout/math/id).

### mw_saves_bevy (4: T 0, C 2, ? 2)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/menu.rs:210 | MONTH_DAYS | `[31, 28, 31, 30, 31, 30, 31, 31, 30, ...` | ? |  |
| src/menu.rs:212 | START | `(16, 7)` | ? |  |

Stays const: RPG_VERSION (layout/math/id), PLAYER_EXT_VERSION (layout/math/id).

### mw_script (2: T 1, C 1, ? 0)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/interp.rs:27 | DEFAULT_STEP_LIMIT | `100_000` | T |  |

Stays const: EMITTER_VERSION (layout/math/id).

### mw_stats (14: T 3, C 8, ? 3)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/gmst.rs:52 | VANILLA_LEVEL_UP_MULT | `[2.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4...` | T |  |
| src/progression.rs:29 | MAX_PICKS | `3` | T |  |
| src/table.rs:126 | STRENGTH | `0` | T |  |
| src/table.rs:136 | MISC | `0.0` | ? |  |
| src/table.rs:137 | MINOR | `1.0` | ? |  |
| src/table.rs:138 | MAJOR | `2.0` | ? |  |

Stays const: RANKS (small index), SECURITY (small index), SNEAK (small index), INTELLIGENCE (small index), WILLPOWER (small index), AGILITY (small index), ENDURANCE (small index), LUCK (small index).

### mw_viewers_bevy (12: T 0, C 12, ? 0)

Stays const: ORBIT_PERIOD (test-scene fixture), SETTLE_FRAMES (test-scene fixture), LOAD_LIMIT_FRAMES (test-scene fixture), SAVE_LIMIT_FRAMES (test-scene fixture), FLOOR_PERIOD (test-scene fixture), BROWSE_BLEND_IN (test-scene fixture), BROWSE_REPLAY_LEAD (test-scene fixture), OrbitCam.yaw (test-scene fixture), OrbitCam.pitch (test-scene fixture), OrbitCam.distance (test-scene fixture), ORBIT_SPEED (test-scene fixture), ZOOM_STEP (test-scene fixture).

### sf_app_bevy (1: T 0, C 0, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:49 | ALLOW_SAME_STATE_TRANSITIONS | `false` | ? | bool |

### sf_bake (5: T 0, C 3, ? 2)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/store.rs:42 | TAG_RAW | `0` | ? |  |
| src/store.rs:43 | TAG_LZ4 | `1` | ? |  |

Stays const: RECORD_HEADER (layout/math/id), RECORD_CHECK (small index), TRACE_FORMAT (layout/math/id).

### sf_coords (3: T 0, C 0, ? 3)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:23 | GRID_CELL | `256.0` | ? |  |
| src/lib.rs:25 | HALF_CELL | `GRID_CELL / 2.0` | ? |  |
| src/lib.rs:26 | CELL_F64 | `GRID_CELL as f64` | ? |  |

### sf_ids (5: T 1, C 0, ? 4)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/clock.rs:17 | MS_PER_SECOND | `1_000` | ? |  |
| src/clock.rs:18 | MS_PER_MINUTE | `60 * MS_PER_SECOND` | ? |  |
| src/clock.rs:19 | MS_PER_HOUR | `60 * MS_PER_MINUTE` | ? |  |
| src/clock.rs:21 | MS_PER_HOUR_F64 | `3_600_000.0` | ? |  |
| src/lib.rs:16 | MAX_PLAYERS | `4` | T |  |

### sf_level (1: T 0, C 0, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/mesh.rs:131 | SEGMENTS | `24` | ? |  |

### sf_rpg (2: T 0, C 2, ? 0)

Stays const: MAX_SLOTS (layout/math/id), SERVICES_VERSION (layout/math/id).

### sf_save (21: T 8, C 7, ? 6)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/container.rs:34 | LZ4_MIN | `128` | T |  |
| src/container.rs:36 | LZ4_MAX_RATIO | `255` | T |  |
| src/container.rs:130 | Limits.max_sections | `4096` | T |  |
| src/fs.rs:163 | RENAME_ATTEMPTS | `5` | T |  |
| src/fs.rs:165 | RENAME_BACKOFF | `Duration::from_millis(10)` | ? |  |
| src/meta.rs:115 | SaveMeta.sequence | `0` | ? |  |
| src/meta.rs:117 | SaveMeta.wall_clock | `0` | ? |  |
| src/meta.rs:118 | SaveMeta.game_days | `0.0` | ? |  |
| src/meta.rs:119 | SaveMeta.play_time_ms | `0` | T |  |
| src/meta.rs:120 | SaveMeta.sim_tick | `0` | ? |  |
| src/meta.rs:128 | SaveMeta.grid_cell | `0.0` | ? |  |
| src/slots.rs:52 | RotationPolicy.auto | `3` | T | settings default |
| src/slots.rs:52 | RotationPolicy.quick | `2` | T | settings default |
| src/slots.rs:52 | RotationPolicy.exit | `1` | T | settings default |

Stays const: CONTAINER_VERSION (layout/math/id), PREAMBLE_LEN (layout/math/id), MAX_OWNER_LEN (layout/math/id), MAX_NAME_LEN (layout/math/id), META_VERSION (layout/math/id), SaveMeta.world_id (layout/math/id), SaveMeta.save_id (layout/math/id).

### sf_save_bevy (25: T 2, C 22, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:94 | SaveConfig.background | `true` | T | settings default |
| src/lib.rs:786 | AUTOSAVE_RETRY | `Duration::from_secs(1)` | ? |  |
| src/sections/mod.rs:127 | V1_DAY_MS | `86_400_000` | T |  |

Stays const: CLOCK_VERSION (layout/math/id), WEATHER_VERSION (layout/math/id), RECORDS_VERSION (layout/math/id), OVERLAY_VERSION (layout/math/id), RPG_VERSION (layout/math/id), AI_VERSION (layout/math/id), SERVICES_VERSION (layout/math/id), EFFECTS_VERSION (layout/math/id), CHARACTER_VERSION (layout/math/id), SCRIPT_VERSION (layout/math/id), RECORDS (id table), OVERLAY (id table), CLOCK (id table), WEATHER (id table), RPG (id table), SERVICES (id table), EFFECTS (id table), AI (id table), PARTY (id table), CHARACTERS (id table), ADAPTER (id table), SCRIPTS (id table).

### sf_script (5: T 3, C 0, ? 2)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/fault.rs:31 | FaultPolicy.max_aborts | `3` | T |  |
| src/fault.rs:32 | FaultPolicy.max_errors | `30` | T |  |
| src/fault.rs:33 | FaultPolicy.window | `Duration::from_secs(60)` | T | settings default |
| src/timers.rs:90 | MUL | `6_364_136_223_846_793_005` | ? |  |
| src/watchdog.rs:31 | CHECK_EVERY | `64` | ? |  |

### sf_script_bevy (1: T 0, C 0, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:112 | RECENT | `64` | ? |  |

### sf_stats (1: T 1, C 0, ? 0)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/ops.rs:153 | MAX_DEPTH | `4` | T |  |

### sf_vfs (1: T 0, C 0, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/watch.rs:33 | DEFAULT_DEBOUNCE | `Duration::from_millis(150)` | ? |  |

### sf_video (1: T 0, C 1, ? 0)

Stays const: PROBE_LEN (layout/math/id).

### sf_video_bevy (5: T 4, C 0, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/audio.rs:25 | RING_SECONDS | `4` | T |  |
| src/control.rs:32 | VIDEO_DUCK_DB | `60.0` | ? |  |
| src/control.rs:35 | DUCK_ATTACK_S | `0.25` | T |  |
| src/control.rs:36 | DUCK_RELEASE_S | `1.0` | T |  |
| src/lib.rs:89 | QUEUE_FRAMES | `3` | T |  |

## physics

### mw_delirium (30: T 26, C 1, ? 3)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/body.rs:77 | Options.arm_out_deg | `55.0` | T |  |
| src/body.rs:77 | Options.torque | `1.0` | T | settings default |
| src/body.rs:77 | Options.body_kg | `80.0` | T | settings default |
| src/sim.rs:14 | DT | `1.0 / 60.0` | ? |  |
| src/takeover.rs:121 | Rules.still_speed | `0.05` | T |  |
| src/takeover.rs:122 | Rules.still_secs | `0.5` | T |  |
| src/takeover.rs:123 | Rules.min_secs | `0.5` | T |  |
| src/takeover.rs:124 | Rules.fallen_ratio | `0.5` | T |  |
| src/takeover.rs:125 | Rules.down_secs | `(0.5, 2.5)` | T |  |
| src/takeover.rs:126 | Rules.rest | `(0.1, 0.3)` | T | settings default |
| src/takeover.rs:128 | Rules.relax | `(50.0, 3.0)` | T | settings default |
| src/takeover.rs:129 | Rules.dead_relax | `(90.0, 2.0)` | T | settings default |
| src/takeover.rs:130 | Rules.corpse_secs | `(1.0, 20.0)` | T |  |
| src/takeover.rs:131 | Rules.max_secs | `15.0` | T |  |
| src/takeover.rs:132 | Rules.falling_speed | `1.0` | T |  |
| src/takeover.rs:133 | Rules.max_velocity | `(8.0, 15.0)` | T |  |
| src/takeover.rs:134 | Rules.recover_speed | `0.25` | T |  |
| src/takeover.rs:135 | Rules.recover_secs | `0.15` | T |  |
| src/takeover.rs:136 | Rules.blend_out | `0.5` | T |  |
| src/takeover.rs:137 | Rules.blend_in | `0.1` | T |  |
| src/takeover.rs:139 | Rules.roll_speed | `2.5` | T |  |
| src/takeover.rs:140 | Rules.roll_gain | `4.0` | T |  |
| src/takeover.rs:141 | Rules.roll_secs | `3.0` | T |  |
| src/takeover.rs:144 | Rules.knock_down_after | `0.3` | T | settings default |
| src/takeover.rs:145 | Rules.dying | `(0.6, 4.0)` | T | settings default |
| src/takeover.rs:219 | TORSO | `[0, 7, 8, 9, 10]` | ? |  |
| src/takeover.rs:258 | RollProgress.since | `0.0` | ? |  |
| src/takeover.rs:578 | ROLL_RADIUS | `0.25` | T |  |
| src/takeover.rs:579 | SETTLED | `1.0` | T |  |

Stays const: RIGHT (layout/math/id).

### mw_delirium_bevy (8: T 7, C 0, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:102 | HAND_BACK_BLEND | `0.3` | T |  |
| src/lib.rs:105 | BLENDED_OUT | `0.1` | T |  |
| src/lib.rs:108 | GROUND_RAY | `4.0` | ? |  |
| src/lib.rs:111 | CLICK_IMPULSE | `[20.0, 45.0, 80.0]` | T |  |
| src/lib.rs:172 | HitRules.takeover | `10.0` | T | settings default |
| src/lib.rs:172 | HitRules.certain_fall | `100.0` | T | settings default |
| src/lib.rs:172 | HitRules.gain | `1.0` | T |  |
| src/lib.rs:930 | SIZE | `4.0` | T |  |

### mw_physics (5: T 5, C 0, ? 0)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:25 | KG_PER_WEIGHT | `0.5` | T |  |
| src/lib.rs:27 | WEIGHTLESS_DENSITY | `400.0` | T |  |
| src/lib.rs:29 | MASS_RANGE | `(0.2, 60.0)` | T |  |
| src/lib.rs:40 | DEFAULT_STRENGTH | `50.0` | T |  |
| src/lib.rs:118 | HUMANOID_MASS | `75.0` | T |  |

### sf_physics (46: T 41, C 2, ? 3)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/body.rs:46 | ITEM_DAMPING | `(0.1, 1.5)` | T |  |
| src/body.rs:202 | PhysicsParams.gravity | `Vec3::new(0.0, -9.81, 0.0)` | T |  |
| src/body.rs:203 | PhysicsParams.rebase_distance | `512.0` | T |  |
| src/body.rs:204 | PhysicsParams.mass_range | `(0.2, 60.0)` | T |  |
| src/body.rs:205 | PhysicsParams.density | `1000.0` | T |  |
| src/body.rs:206 | PhysicsParams.max_corrective_velocity | `0.5` | T |  |
| src/body.rs:207 | PhysicsParams.thin_extent | `0.02` | T | settings default |
| src/body.rs:208 | PhysicsParams.thin_skin | `0.004` | T | settings default |
| src/body.rs:210 | PhysicsParams.contact_force_threshold | `200.0` | T |  |
| src/body.rs:213 | PhysicsParams.contact_report_speed | `0.3` | T |  |
| src/body.rs:215 | PhysicsParams.kill_margin | `10.0` | T |  |
| src/body.rs:216 | PhysicsParams.rolling_resistance | `20.0` | T | settings default |
| src/carry.rs:58 | HolderLimits.carry_force | `240.0` | T |  |
| src/carry.rs:58 | HolderLimits.carry_mass | `15.0` | T |  |
| src/carry.rs:58 | HolderLimits.drag_mass | `120.0` | T |  |
| src/carry.rs:58 | HolderLimits.throw_impulse | `6.0` | T |  |
| src/carry.rs:107 | CarryParams.reach | `2.5` | T |  |
| src/carry.rs:108 | CarryParams.hold_distance | `0.6` | T |  |
| src/carry.rs:109 | CarryParams.settle_time | `0.15` | T |  |
| src/carry.rs:110 | CarryParams.break_distance | `1.0` | T |  |
| src/carry.rs:111 | CarryParams.break_time | `0.3` | T | settings default |
| src/carry.rs:112 | CarryParams.max_throw_speed | `15.0` | T |  |
| src/carry.rs:113 | CarryParams.drag_friction | `0.6` | T |  |
| src/carry.rs:114 | CarryParams.torque_arm | `0.3` | T | settings default |
| src/carry.rs:115 | CarryParams.extra_iterations | `4` | T |  |
| src/carry.rs:118 | CarryParams.max_extrapolation | `0.1` | T |  |
| src/character.rs:45 | PushParams.push_force | `400.0` | T |  |
| src/character.rs:45 | PushParams.reach | `0.04` | T |  |
| src/character.rs:45 | PushParams.capsule_inset | `0.03` | T | settings default |
| src/character.rs:109 | INCOMING | `0.25` | ? |  |
| src/cook.rs:55 | FitSettings.primitive_tolerance | `0.15` | T |  |
| src/cook.rs:56 | FitSettings.hull_tolerance | `0.25` | T |  |
| src/cook.rs:57 | FitSettings.max_hulls | `8` | T |  |
| src/cook.rs:58 | FitSettings.max_hull_vertices | `32` | T |  |
| src/cook.rs:59 | FitSettings.min_extent | `0.01` | T |  |
| src/cook.rs:60 | FitSettings.resolution | `64` | T |  |
| src/debug.rs:96 | CROSS | `0.04` | ? |  |
| src/steps.rs:186 | CheckSettings.settle_time | `0.5` | T |  |
| src/steps.rs:186 | CheckSettings.support_tolerance | `0.05` | T |  |
| src/steps.rs:186 | CheckSettings.reach | `1.0` | T |  |
| src/steps.rs:186 | CheckSettings.terrain_patch | `2.0` | T | settings default |
| src/steps.rs:186 | CheckSettings.overlap_allowance | `0.01` | T | settings default |
| src/steps.rs:344 | MIN_SHRINK | `0.8` | T |  |
| src/world.rs:568 | STANDING | `0.05` | ? |  |

Stays const: BODY_FORMAT (layout/math/id), ITEMS_FORMAT (layout/math/id).

### sf_physics_bevy (26: T 23, C 2, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/character.rs:40 | CharacterPhysics.impact_threshold | `1.0` | T |  |
| src/character.rs:111 | TERRAIN_KILL_DEPTH | `2.0` | T |  |
| src/debug.rs:27 | PhysicsDebug.enabled | `false` | ? | bool |
| src/debug.rs:27 | PhysicsDebug.radius | `40.0` | T |  |
| src/debug.rs:27 | PhysicsDebug.max_lines | `400_000` | T |  |
| src/lib.rs:102 | PhysicsSettings.hz | `60.0` | T | settings default |
| src/lib.rs:103 | PhysicsSettings.max_steps_per_frame | `3` | T |  |
| src/lib.rs:104 | PhysicsSettings.insert_budget_ms | `0.5` | T |  |
| src/lib.rs:105 | PhysicsSettings.insert_batch | `16` | T |  |
| src/lib.rs:106 | PhysicsSettings.item_batch | `32` | T |  |
| src/lib.rs:107 | PhysicsSettings.debris_mass | `15.0` | T |  |
| src/lib.rs:108 | PhysicsSettings.debris_extent | `0.6` | T | settings default |
| src/lib.rs:109 | PhysicsSettings.terrain_chunk | `64.0` | T | settings default |
| src/lib.rs:110 | PhysicsSettings.terrain_spacing | `0.5` | T |  |
| src/lib.rs:111 | PhysicsSettings.terrain | `true` | T | settings default |
| src/proxies.rs:45 | MAX_LEAD | `0.1` | T |  |
| src/proxies.rs:176 | SECS | `1.0` | T |  |
| src/ragdoll.rs:92 | RagdollConfig.fade | `0.1` | T |  |
| src/ragdoll.rs:94 | RagdollConfig.get_up_blend | `0.3` | T |  |
| src/ragdoll.rs:95 | RagdollConfig.max_powered | `8` | T |  |
| src/ragdoll.rs:472 | GET_UP_PROBE | `(1.0, 2.0)` | T |  |
| src/stream.rs:415 | COVERAGE_PERIOD_S | `0.25` | T |  |
| src/stream.rs:416 | COVERAGE_MARGIN | `1.0` | T |  |
| src/stream.rs:504 | INSERT_PRIOR_S | `15e-6` | T |  |

Stays const: LYING_REST_GET_UP (layout/math/id), STATE_VERSION (layout/math/id).

### sf_ragdoll (32: T 24, C 3, ? 5)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/cook.rs:75 | CookParams.density | `1000.0` | T |  |
| src/cook.rs:76 | CookParams.max_mass_ratio | `4.0` | T |  |
| src/cook.rs:77 | CookParams.max_spine_bodies | `3` | T |  |
| src/cook.rs:78 | CookParams.max_chain_bodies | `4` | T |  |
| src/cook.rs:79 | CookParams.limb_radius | `0.2` | T |  |
| src/cook.rs:80 | CookParams.effector_radius | `0.3` | T |  |
| src/cook.rs:81 | CookParams.wing_radius | `0.08` | T |  |
| src/cook.rs:82 | CookParams.chain_radius | `0.2` | T |  |
| src/cook.rs:83 | CookParams.min_extra_chain | `0.15` | T |  |
| src/cook.rs:122 | SPINE_SWING_DEG | `15.0` | T |  |
| src/cook.rs:123 | SPINE_TWIST_DEG | `15.0` | T |  |
| src/cook.rs:145 | HYPEREXTENSION | `10.0` | ? |  |
| src/instance.rs:48 | INERTIA_FLOOR | `1.0` | ? |  |
| src/instance.rs:55 | RagdollSettings.extra_iterations | `16` | T |  |
| src/instance.rs:56 | RagdollSettings.damping | `(0.05, 0.6)` | T |  |
| src/instance.rs:58 | RagdollSettings.user | `0` | T | settings default |
| src/instance.rs:60 | RagdollSettings.ccd | `true` | T | settings default |
| src/instance.rs:62 | RagdollSettings.end_inertia_floor | `0.4` | T | settings default |
| src/instance.rs:63 | RagdollSettings.head_inertia_floor | `0.3` | T | settings default |
| src/instance.rs:64 | RagdollSettings.axial_inertia | `0.5` | T | settings default |
| src/powered.rs:145 | DriveParams.omega | `160.0` | T | settings default |
| src/powered.rs:145 | DriveParams.kinematic_gain | `4.0` | T |  |
| src/powered.rs:145 | DriveParams.pelvis_omega | `12.0` | T | settings default |
| src/powered.rs:145 | DriveParams.gravity_compensation | `1.0` | T |  |
| src/powered.rs:145 | DriveParams.powered_iterations | `8` | T |  |
| src/react.rs:295 | Sensed.pelvis_speed | `0.0` | T |  |
| src/react.rs:295 | Sensed.upright | `1.0` | ? |  |
| src/react.rs:295 | Sensed.grounded | `true` | ? | bool |
| src/react.rs:295 | Sensed.host | `true` | ? | bool |

Stays const: RAGDOLL_FORMAT (layout/math/id), EPS (layout/math/id), Sensed.can_get_up (layout/math/id).

## shadows-lights-gi

### sf_gi (20: T 15, C 4, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/ddgi.rs:18 | IRRADIANCE_TEXELS | `6` | T |  |
| src/ddgi.rs:20 | VISIBILITY_TEXELS | `14` | T |  |
| src/ddgi.rs:22 | DEPTH_SHARPNESS | `50.0` | T |  |
| src/ddgi.rs:24 | SELF_SHADOW_BIAS | `0.3` | T |  |
| src/fit.rs:22 | VolumeBudget.max_dims | `UVec3::new(32, 16, 32)` | T |  |
| src/fit.rs:22 | VolumeBudget.min_spacing | `1.0` | T |  |
| src/lib.rs:32 | MAX_RAYS | `64` | T |  |
| src/lib.rs:55 | GiSettings.dims | `UVec3::new(16, 8, 16)` | T | settings default |
| src/lib.rs:55 | GiSettings.spacing | `1.0` | T |  |
| src/lib.rs:55 | GiSettings.rays | `64` | T |  |
| src/lib.rs:55 | GiSettings.probes_per_frame | `512` | T |  |
| src/lib.rs:55 | GiSettings.hysteresis | `0.95` | T |  |
| src/lib.rs:55 | GiSettings.changed_hysteresis | `0.8` | T |  |
| src/lib.rs:55 | GiSettings.max_lights | `64` | T |  |
| src/lights.rs:18 | MOVED_DISTANCE | `0.05` | T |  |
| src/lights.rs:21 | POWER_CHANGE | `0.1` | ? |  |

Stays const: EPSILON (layout/math/id), GPU_LIGHT_WORDS (layout/math/id), FLAG_SPOT (layout/math/id), FLAG_CHANGED (layout/math/id).

### sf_gi_bevy (19: T 10, C 8, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:74 | READINESS_TAIL_FRAMES | `2` | T |  |
| src/lib.rs:113 | GiVolume.intensity | `1.0` | T |  |
| src/lib.rs:113 | GiVolume.bounce | `true` | ? | bool |
| src/lib.rs:113 | GiVolume.ambient_scale | `0.0` | T |  |
| src/probe_update.wesl:66 | MAX_RAYS | `64u` | T | shader const: needs a uniform |
| src/probe_update.wesl:68 | SURFACE_BIAS | `0.01` | T | shader const: needs a uniform |
| src/probe_update.wesl:70 | IRR_TEXELS | `6u` | T | shader const: needs a uniform |
| src/probe_update.wesl:71 | VIS_TEXELS | `14u` | T | shader const: needs a uniform |
| src/probe_update.wesl:72 | IRR_COUNT | `36u` | T | shader const: needs a uniform |
| src/probe_update.wesl:73 | VIS_COUNT | `196u` | T | shader const: needs a uniform |
| src/probe_update.wesl:74 | DEPTH_SHARPNESS | `50.0` | T | shader const: needs a uniform |

Stays const: PI (layout/math/id), FLAG_SUN (layout/math/id), FLAG_BOUNCE (layout/math/id), LIGHT_SPOT (shader enum), LIGHT_CHANGED (shader enum), UNIFORM_BYTES (layout/math/id), FLAG_SUN (layout/math/id), FLAG_BOUNCE (layout/math/id).

### sf_shadow_cache_bevy (11: T 10, C 0, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:81 | MAX_CACHED_LIGHTS | `42` | T |  |
| src/lib.rs:119 | ShadowCacheSettings.enabled | `true` | T | settings default |
| src/lib.rs:120 | ShadowCacheSettings.max_lights | `24` | T |  |
| src/lib.rs:121 | ShadowCacheSettings.settle_time | `1.0` | T |  |
| src/lib.rs:122 | ShadowCacheSettings.move_threshold | `0.01` | T |  |
| src/lib.rs:123 | ShadowCacheSettings.reuse_layers | `true` | T | settings default |
| src/lib.rs:124 | ShadowCacheSettings.captures_per_frame | `2` | T |  |
| src/lib.rs:125 | ShadowCacheSettings.recapture_interval | `2.0` | T |  |
| src/lib.rs:126 | ShadowCacheSettings.warmup | `false` | T |  |
| src/lib.rs:127 | ShadowCacheSettings.warmup_settle_time | `0.2` | T |  |
| src/render.rs:470 | CAPTURE_SUBVIEW | `6` | ? |  |

## sky-weather-surface

### mw_environment_bevy (9: T 6, C 1, ? 2)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/night_sky.rs:18 | DEG_PER_SIZE | `0.1` | T |  |
| src/night_sky.rs:20 | DAYS_PER_PHASE | `3.0` | ? |  |
| src/render.rs:28 | POINT_SHADOWS_DEFAULT | `(8, 512)` | T |  |
| src/render.rs:32 | LAMP_SHADOW_FADE | `(35.0, 45.0)` | T |  |
| src/render.rs:37 | POINT_SHADOW_CACHE | `false` | T |  |
| src/water.rs:78 | DEFAULT_WIND | `7.0` | ? |  |
| src/weather.rs:206 | RASTER_MARGIN | `4` | T |  |
| src/weather.rs:218 | LIVE_HOLD_HOURS | `1.0e6` | T |  |

Stays const: SEED (sentinel/derived).

### mw_weather (23: T 17, C 1, ? 5)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/ambient.rs:44 | WEATHER_AMBIENT_BRIGHTNESS | `0.0` | ? |  |
| src/ambient.rs:48 | FADE_STEPS | `96` | T |  |
| src/cover.rs:38 | TILING_M | `512.0 * UNIT_METERS` | T |  |
| src/cover.rs:42 | SNOW_DEPTH_M | `0.45` | T |  |
| src/cover.rs:44 | ASH_DEPTH_M | `0.03` | T |  |
| src/lib.rs:46 | DEFAULT_TIMESCALE | `30.0` | T |  |
| src/lib.rs:358 | REFERENCE_RAINDROPS | `650.0` | ? |  |
| src/lib.rs:361 | REFERENCE_SNOWFLAKES | `1500.0` | ? |  |
| src/visual.rs:26 | CLOUD_WORLD_SCALE | `0.5` | T |  |
| src/visual.rs:31 | MAX_CLOUD_THICKNESS_M | `1_600.0` | T |  |
| src/visual.rs:43 | CLOUD_SPEED_MPS | `10.0` | T |  |
| src/visual.rs:46 | BANK_DENSITY | `0.03` | T |  |
| src/visual.rs:48 | BANK_FALLOFF | `12.0` | T |  |
| src/visual.rs:52 | WIND_SPEED_RANGE | `[1.0, 15.0]` | T |  |
| src/visual.rs:58 | RAIN_TILT_MAX_DEG | `20.0` | T |  |
| src/visual.rs:60 | STORM_TILT_MAX_DEG | `35.0` | T |  |
| src/visual.rs:62 | BLOWING_TILT_MAX_DEG | `sf_weather::UNCAPPED_DEG` | T |  |
| src/visual.rs:75 | RED_MOUNTAIN_SUMMIT | `[29_568.0, 73_728.0, 18_952.0]` | ? |  |
| src/visual.rs:79 | STORM_STRENGTH | `6.0` | T |  |
| src/visual.rs:80 | STORM_INNER_M | `300.0` | T |  |
| src/visual.rs:81 | STORM_RADIUS_M | `3_000.0` | T |  |
| src/visual.rs:124 | ASH_ALBEDO | `Vec3::new(0.55, 0.47, 0.38)` | ? |  |

Stays const: UNIT_METERS (unit/format).

### sf_atmos (79: T 34, C 2, ? 43)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/bank_edges.rs:21 | LOG_STEPS | `32.0` | T |  |
| src/bank_edges.rs:23 | OCTAVES | `24.0` | ? |  |
| src/bank_edges.rs:26 | DEPTH_SIGMA | `0.25` | T |  |
| src/bank_edges.rs:29 | DEPTH_FLOOR | `1e-4` | T |  |
| src/density.rs:50 | CB_TOP_MIN | `0.35` | T |  |
| src/density.rs:53 | CB_TOP_EXP | `1.5` | ? |  |
| src/density.rs:56 | CB_CELL_SHIFT | `0.8` | ? |  |
| src/density.rs:59 | CB_ANVIL_WIDEN | `0.15` | ? |  |
| src/density.rs:61 | CB_ANVIL_LO | `0.72` | ? |  |
| src/density.rs:62 | CB_ANVIL_RISE | `0.08` | ? |  |
| src/density.rs:64 | CB_ANVIL_COVER | `0.6` | ? |  |
| src/density.rs:66 | CB_RAG_H | `0.1` | ? |  |
| src/density.rs:68 | CB_RAG_BOOST | `1.0` | T |  |
| src/density.rs:142 | COVERAGE_EDGE | `0.2` | T |  |
| src/density.rs:175 | EVOLVE_MIX | `0.35` | ? |  |
| src/fog.rs:28 | KOSCHMIEDER | `3.912` | ? |  |
| src/fog.rs:49 | FogLayer.density | `0.0` | T |  |
| src/fog.rs:49 | FogLayer.falloff | `100.0` | T |  |
| src/fog.rs:49 | FogLayer.base | `0.0` | ? |  |
| src/fog.rs:55 | MAX_EXPONENT | `2.0` | T |  |
| src/fog.rs:112 | FogBanks.coverage | `0.42` | T |  |
| src/fog.rs:113 | FogBanks.sharpness | `0.18` | T |  |
| src/fog.rs:116 | FogBanks.tile | `1_024.0` | ? |  |
| src/fog.rs:117 | FogBanks.detail_tile | `256.0` | ? |  |
| src/fog.rs:118 | FogBanks.warp_tile | `2_048.0` | ? |  |
| src/fog.rs:119 | FogBanks.warp_amount | `220.0` | ? |  |
| src/fog.rs:127 | NOISE_MID | `0.357` | T |  |
| src/fog.rs:128 | NOISE_GAIN | `6.0` | T |  |
| src/fog.rs:171 | FogSettings.phase_g | `0.6` | T | settings default |
| src/fog.rs:172 | FogSettings.ambient | `1.0` | T | settings default |
| src/fog.rs:173 | FogSettings.far | `6_000.0` | T |  |
| src/fog.rs:174 | FogSettings.sky_distance | `20_000.0` | T |  |
| src/fog.rs:175 | FogSettings.wind | `Vec2::new(2.0, 0.5)` | T | settings default |
| src/fog.rs:218 | WARP_DRIFT | `0.35` | ? |  |
| src/fog.rs:220 | DETAIL_DRIFT | `1.35` | ? |  |
| src/fog.rs:256 | SKY_FILL_SAMPLES | `64` | T |  |
| src/fog.rs:259 | GOLDEN_ANGLE | `2.399_963_2` | T |  |
| src/fog.rs:302 | VIEW_AMBIENT_IN_FOG | `1.0` | ? |  |
| src/froxel.rs:79 | G | `1.220_744_084_605_759_5` | ? |  |
| src/froxel.rs:123 | HISTORY_CLAMPED | `0.95` | ? |  |
| src/froxel.rs:126 | HISTORY_UNCLAMPED | `0.9` | ? |  |
| src/froxel.rs:176 | MAX_FOG_LIGHTS | `32` | T |  |
| src/noise.rs:16 | SHAPE_SIZE | `128` | T |  |
| src/noise.rs:18 | DETAIL_SIZE | `32` | T |  |
| src/noise.rs:20 | WEATHER_SIZE | `512` | T |  |
| src/noise.rs:28 | STORM_PERIOD | `3` | ? |  |
| src/noise.rs:32 | CIRRUS_STRETCH | `6.0` | ? |  |
| src/scroll.rs:24 | SCROLL_PERIOD | `16_384.0` | ? |  |
| src/scroll.rs:28 | CIRRUS_WIND | `2.0` | ? |  |
| src/scroll.rs:32 | RISE_RATE | `0.5` | T |  |
| src/settings.rs:259 | CloudLook.extinction | `0.015` | ? |  |
| src/settings.rs:260 | CloudLook.ambient | `1.0` | ? |  |
| src/settings.rs:261 | CloudLook.ground_bounce | `1.0` | ? |  |
| src/settings.rs:262 | CloudLook.powder | `0.7` | ? |  |
| src/settings.rs:263 | CloudLook.phase_g | `0.6` | ? |  |
| src/settings.rs:264 | CloudLook.ms_falloff | `0.5` | T |  |
| src/settings.rs:267 | CloudLook.weather_tile | `8_192.0` | ? |  |
| src/settings.rs:268 | CloudLook.shape_tile | `2_048.0` | ? |  |
| src/settings.rs:269 | CloudLook.detail_tile | `256.0` | ? |  |
| src/settings.rs:270 | CloudLook.cirrus_tile | `16_384.0` | ? |  |
| src/settings.rs:271 | CloudLook.cirrus_depth | `0.6` | T |  |
| src/settings.rs:272 | CloudLook.evolve | `0.35` | ? |  |
| src/settings.rs:273 | CloudLook.far | `50_000.0` | T |  |
| src/settings.rs:274 | CloudLook.shadow_strength | `0.9` | T |  |
| src/shadow.rs:23 | MAX_DRIFT_M | `4.0` | T |  |
| src/storm.rs:17 | STORM_SEGMENTS | `16` | ? |  |
| src/storm.rs:19 | CORE_FIELD | `0.4` | ? |  |
| src/storm.rs:21 | CORE_RAMP | `0.25` | ? |  |
| src/storm.rs:23 | STORM_T0 | `32.0` | ? |  |
| src/storm.rs:26 | NEAR_SHARP | `(128.0, 256.0)` | T |  |
| src/storm.rs:45 | StormShape.enabled | `true` | ? | bool |
| src/storm.rs:45 | StormShape.edge_m | `64.0` | T |  |
| src/storm.rs:45 | StormShape.edge_noise_m | `48.0` | T |  |
| src/storm.rs:45 | StormShape.ground_fraction | `0.35` | T |  |
| src/storm.rs:324 | HI | `0.15` | ? |  |
| src/temporal.rs:172 | HALF | `[8, 8]` | ? |  |
| src/temporal.rs:173 | FULL | `[16, 16]` | ? |  |

Stays const: INJECT_GROUP (layout/math/id), INJECT_TILE (derived).

### sf_atmos_bevy (40: T 22, C 6, ? 12)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:75 | SHADOW_HALF_EXTENT | `8_192.0` | T |  |
| src/lib.rs:79 | SHADOW_ANCHOR_CELL | `2_048.0` | T |  |
| src/render.rs:452 | FOG_LIGHT_TEXELS | `3` | T |  |
| src/render.rs:455 | FOG_STEPS | `16` | T |  |
| src/shaders/bank_edges.wesl:6 | LOG_STEPS | `32.0` | T | shader const: needs a uniform |
| src/shaders/bank_edges.wesl:7 | OCTAVES | `24.0` | ? |  |
| src/shaders/bank_edges.wesl:8 | DEPTH_SIGMA | `0.25` | T | shader const: needs a uniform |
| src/shaders/bank_edges.wesl:9 | DEPTH_FLOOR | `1e-4` | T | shader const: needs a uniform |
| src/shaders/cloud_common.wesl:130 | CB_TOP_MIN | `0.35` | T | shader const: needs a uniform |
| src/shaders/cloud_common.wesl:131 | CB_TOP_EXP | `1.5` | ? |  |
| src/shaders/cloud_common.wesl:132 | CB_CELL_SHIFT | `0.8` | ? |  |
| src/shaders/cloud_common.wesl:133 | CB_ANVIL_WIDEN | `0.15` | ? |  |
| src/shaders/cloud_common.wesl:134 | CB_ANVIL_LO | `0.72` | ? |  |
| src/shaders/cloud_common.wesl:135 | CB_ANVIL_RISE | `0.08` | ? |  |
| src/shaders/cloud_common.wesl:136 | CB_ANVIL_COVER | `0.6` | ? |  |
| src/shaders/cloud_common.wesl:137 | CB_RAG_H | `0.1` | ? |  |
| src/shaders/cloud_common.wesl:138 | CB_RAG_BOOST | `1.0` | T | shader const: needs a uniform |
| src/shaders/cloud_trace.wesl:157 | FAR_STEPS | `8u` | T | shader const: needs a uniform |
| src/shaders/cloud_trace.wesl:159 | FAR_SHEAR | `0.35` | T | shader const: needs a uniform |
| src/shaders/cloud_trace.wesl:254 | CORE_FIELD | `0.4` | ? |  |
| src/shaders/cloud_trace.wesl:255 | CORE_RAMP | `0.25` | ? |  |
| src/shaders/cloud_trace.wesl:256 | STORM_T0 | `32.0` | ? |  |
| src/shaders/cloud_trace.wesl:258 | STORM_NEAR_SHARP | `vec2(128.0, 256.0)` | T | shader const: needs a uniform |
| src/shaders/cloud_trace.wesl:260 | EDGE_NOISE_TILES | `24.0` | T | shader const: needs a uniform |
| src/shaders/fog.wesl:40 | MAX_EXPONENT | `2.0` | T | shader const: needs a uniform |
| src/shaders/fog_light.wesl:27 | SKY_FILL_SAMPLES | `64u` | T | shader const: needs a uniform |
| src/shaders/fog_light.wesl:28 | GOLDEN_ANGLE | `2.3999632` | T | shader const: needs a uniform |
| src/shaders/froxel_common.wesl:19 | MAX_FOG_LIGHTS | `32u` | T | shader const: needs a uniform |
| src/shaders/froxel_inject.wesl:33 | FAR_PENUMBRA | `0.012` | T | shader const: needs a uniform |
| src/shaders/froxel_inject.wesl:34 | FAR_PENUMBRA_MIN | `2.0` | T | shader const: needs a uniform |
| src/shaders/froxel_inject.wesl:35 | FAR_BIAS | `6.0` | T | shader const: needs a uniform |
| src/shaders/froxel_inject.wesl:36 | FAR_BIAS_PER_M | `0.0015` | T | shader const: needs a uniform |
| src/shaders/froxel_inject.wesl:37 | FAR_FADE_START | `0.85` | T | shader const: needs a uniform |
| src/shaders/froxel_inject.wesl:126 | TILE | `GROUP - 2u` | ? |  |

Stays const: GLOWS (small index), PI (layout/math/id), STORM_SEGMENTS (shader enum), FLAG_PER_FROXEL (layout/math/id), FLAG_CLAMP (layout/math/id), GROUP (layout/math/id).

### sf_precip_bevy (22: T 16, C 2, ? 4)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:56 | STREAK_S | `0.035` | T |  |
| src/lib.rs:90 | PrecipSlot.intensity | `0.0` | T |  |
| src/lib.rs:91 | PrecipSlot.fall_speed | `7.0` | T |  |
| src/lib.rs:92 | PrecipSlot.size | `[0.002, 0.004]` | T |  |
| src/lib.rs:93 | PrecipSlot.color | `Vec4::new(0.55, 0.6, 0.65, 0.3)` | ? |  |
| src/lib.rs:94 | PrecipSlot.flutter | `0.0` | ? |  |
| src/lib.rs:95 | PrecipSlot.wind_follow | `1.0` | T |  |
| src/lib.rs:115 | Precip.enabled | `true` | ? | bool |
| src/lib.rs:118 | Precip.brightness | `1.0` | ? |  |
| src/occlusion.rs:55 | MAP_SIZE | `512` | T |  |
| src/occlusion.rs:58 | HALF_M | `48.0` | T |  |
| src/occlusion.rs:61 | TEXEL_M | `2.0 * HALF_M / MAP_SIZE as f32` | T |  |
| src/occlusion.rs:64 | UPWIND_M | `100.0` | T |  |
| src/occlusion.rs:65 | DOWNWIND_M | `60.0` | T |  |
| src/occlusion.rs:67 | MIN_ELEVATION | `0.261_799_4` | T |  |
| src/occlusion.rs:69 | RECENTER_M | `6.0` | T |  |
| src/occlusion.rs:70 | RECENTER_Y_M | `8.0` | T |  |
| src/occlusion.rs:72 | REDIRECT_DEG | `3.0` | T |  |
| src/occlusion.rs:74 | REFRESH_FRAMES | `30` | T |  |
| src/occlusion.rs:76 | REDIRECT_FRAMES | `8` | T |  |

Stays const: SLOTS (layout/math/id), SUBVIEW (sentinel/derived).

### sf_sky (54: T 31, C 1, ? 22)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/atmosphere.rs:90 | MediumScales.haze | `1.0` | ? |  |
| src/atmosphere.rs:91 | MediumScales.dust | `0.0` | ? |  |
| src/cascade_hold.rs:56 | CascadeHoldConfig.enabled | `true` | T | settings default |
| src/cascade_hold.rs:56 | CascadeHoldConfig.hold_from | `1` | T |  |
| src/cascade_hold.rs:56 | CascadeHoldConfig.move_threshold | `4.0` | T |  |
| src/cascade_hold.rs:56 | CascadeHoldConfig.turn_threshold_deg | `3.0` | T |  |
| src/cascade_hold.rs:56 | CascadeHoldConfig.sun_threshold_deg | `0.1` | T |  |
| src/cascade_hold.rs:56 | CascadeHoldConfig.max_age | `4` | T |  |
| src/celestial.rs:42 | Orbit.latitude_deg | `45.0` | T |  |
| src/celestial.rs:43 | Orbit.axial_tilt_deg | `23.44` | T |  |
| src/celestial.rs:45 | Orbit.spring_equinox_day | `79.0` | T |  |
| src/celestial.rs:46 | Orbit.north_heading_deg | `0.0` | T |  |
| src/celestial.rs:47 | Orbit.lunar_month_days | `29.530_59` | ? |  |
| src/celestial.rs:48 | Orbit.moon_phase_at_epoch | `0.0` | ? |  |
| src/clock.rs:21 | Calendar.hours_per_day | `24.0` | T |  |
| src/clock.rs:22 | Calendar.days_per_year | `365.2422` | ? |  |
| src/clock.rs:49 | SkyClock.days | `0.5` | ? |  |
| src/night.rs:21 | MAX_MOONS | `4` | T |  |
| src/night.rs:82 | Moon.angular_size | `0.009_04` | T |  |
| src/night.rs:83 | Moon.phase_period_days | `29.530_59` | ? |  |
| src/night.rs:84 | Moon.phase_at_epoch | `0.0` | ? |  |
| src/night.rs:85 | Moon.phase_steps | `0` | T |  |
| src/night.rs:87 | Moon.illuminance | `1.0` | ? |  |
| src/night.rs:88 | Moon.tint | `[0.9, 0.93, 1.0]` | ? |  |
| src/night.rs:90 | Moon.occluder | `1.0` | ? |  |
| src/night.rs:116 | Moonlight.full_lux | `1.0` | T |  |
| src/night.rs:116 | Moonlight.artistic | `false` | ? | bool |
| src/night.rs:237 | StarField.enabled | `true` | ? | bool |
| src/night.rs:238 | StarField.brightest_lux | `2.0e-5` | T |  |
| src/night.rs:239 | StarField.density | `1.0` | T |  |
| src/night.rs:241 | StarField.twinkle | `0.3` | ? |  |
| src/night.rs:270 | SunDiskLook.intensity | `1.0` | T |  |
| src/night.rs:271 | SunDiskLook.glare | `1.2e-3` | ? |  |
| src/night.rs:299 | NIGHT_SKY_GLOW_RATIO | `0.1` | T |  |
| src/night.rs:304 | NIGHT_SKY_HORIZON_RATIO | `3.0` | T |  |
| src/sky.rs:68 | SkySettings.observer_altitude | `0.0` | T | settings default |
| src/sky.rs:69 | SkySettings.sun_illuminance | `130_000.0` | T | settings default |
| src/sky.rs:70 | SkySettings.sun_angular_size | `0.009_308_42` | T |  |
| src/sky.rs:71 | SkySettings.moon_illuminance | `1.0` | T | settings default |
| src/sky.rs:72 | SkySettings.moon_angular_size | `0.009_04` | T |  |
| src/sky.rs:90 | SkySettings.sun_shadow_min_elevation_deg | `-1.0` | T |  |
| src/sky.rs:91 | SkySettings.moon_shadow_min_elevation_deg | `3.0` | T |  |
| src/sky.rs:109 | ExposureRange.min_ev100 | `-10.0` | T |  |
| src/sky.rs:109 | ExposureRange.max_ev100 | `20.0` | T |  |
| src/sky.rs:155 | SkyLight.elevation | `0.0` | ? |  |
| src/sky.rs:156 | SkyLight.illuminance | `0.0` | ? |  |
| src/sky.rs:158 | SkyLight.casts_shadows | `false` | T |  |
| src/sky.rs:216 | TRANSMITTANCE_SAMPLES | `48` | T |  |
| src/weather.rs:53 | Weather.cloud_cover | `0.0` | ? |  |
| src/weather.rs:54 | Weather.overcast_dimming | `0.8` | ? |  |
| src/weather.rs:55 | Weather.haze | `1.0` | ? |  |
| src/weather.rs:56 | Weather.dust | `0.0` | ? |  |
| src/weather.rs:58 | Weather.sun_glare | `1.0` | ? |  |

Stays const: EPS (layout/math/id).

### sf_sky_bevy (31: T 26, C 2, ? 3)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/gpu_culled_casters.rs:40 | ALL_FACES | `0b11_1111` | ? |  |
| src/lib.rs:175 | SkyRenderOptions.sea_level | `0.0` | T | settings default |
| src/lib.rs:177 | SkyRenderOptions.num_cascades | `4` | T |  |
| src/lib.rs:178 | SkyRenderOptions.minimum_distance | `0.1` | T |  |
| src/lib.rs:179 | SkyRenderOptions.split_near | `1.0` | T |  |
| src/lib.rs:180 | SkyRenderOptions.maximum_distance | `250.0` | T |  |
| src/lib.rs:181 | SkyRenderOptions.lambda | `0.9` | T | settings default |
| src/lib.rs:182 | SkyRenderOptions.overlap_proportion | `0.2` | T | settings default |
| src/lib.rs:188 | SkyRenderOptions.weather_ambient_tint | `true` | T | settings default |
| src/lib.rs:189 | SkyRenderOptions.night_sky_glow | `true` | T | settings default |
| src/lib.rs:190 | SkyRenderOptions.medium_resolution | `256` | T |  |
| src/lib.rs:191 | SkyRenderOptions.medium_min_interval | `0.25` | T |  |
| src/lib.rs:192 | SkyRenderOptions.auto_light_proxies | `true` | T | settings default |
| src/lib.rs:193 | SkyRenderOptions.default_proxy_radius | `0.08` | T |  |
| src/lib.rs:194 | SkyRenderOptions.auto_ray_tracing_meshes | `true` | T | settings default |
| src/lib.rs:196 | SkyRenderOptions.ray_tracing_denoiser | `true` | T |  |
| src/lib.rs:224 | SunShadow.contact_shadows | `false` | T |  |
| src/lib.rs:250 | SkyCamera.exposure | `true` | T |  |
| src/lib.rs:251 | SkyCamera.ambient | `true` | ? | bool |
| src/lib.rs:252 | SkyCamera.environment_light | `true` | ? | bool |
| src/lib.rs:384 | MEDIUM_UPDATE_STEPS | `Vec3::new(0.05, 0.02, 0.02)` | T |  |
| src/shadow_budget.rs:84 | PointShadowBudget.max | `0` | T | settings default |
| src/shadow_budget.rs:84 | PointShadowBudget.fade_start | `15.0` | T |  |
| src/shadow_budget.rs:84 | PointShadowBudget.fade_end | `25.0` | T |  |
| src/shadow_budget.rs:84 | PointShadowBudget.fade_time | `0.3` | T |  |
| src/shadow_budget.rs:84 | PointShadowBudget.hysteresis | `0.8` | T |  |
| src/shadow_budget.rs:84 | PointShadowBudget.map_size | `1024` | T |  |
| src/shadow_budget.rs:84 | PointShadowBudget.contact_shadows | `false` | T |  |
| src/shadow_budget.rs:130 | BudgetedShadow.priority | `1.0` | T |  |

Stays const: T_ROWS (small index), PI (layout/math/id).

### sf_surface (90: T 58, C 3, ? 29)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/grid.rs:10 | CELL_M | `32.0` | T |  |
| src/grid.rs:13 | SNOW_LINE_BLEND_M | `50.0` | T |  |
| src/kickup.rs:26 | BURSTS | `64` | T |  |
| src/kickup.rs:28 | SPRITES | `12` | ? |  |
| src/kickup.rs:30 | RANGE_M | `30.0` | T |  |
| src/kickup.rs:32 | MIN_COVER | `0.15` | T |  |
| src/kickup.rs:34 | FLOOR_BELOW_M | `0.05` | T |  |
| src/kickup.rs:37 | FOOT_REF_M | `0.14` | T |  |
| src/kickup.rs:234 | STEP_M | `[0.2, 2.6]` | T |  |
| src/kickup.rs:236 | STEP_S | `[0.12, 1.2]` | T |  |
| src/kickup.rs:289 | SEED_STEP | `Vec3::new(0.618_034, 0.754_877_7, 0.5...` | T |  |
| src/layer.rs:34 | POROSITY_ROUGH_LOW | `0.3` | ? |  |
| src/layer.rs:35 | POROSITY_ROUGH_SPAN | `0.5` | ? |  |
| src/layer.rs:36 | WET_DARK_POROUS | `0.2` | ? |  |
| src/layer.rs:37 | WET_SATURATION | `0.3` | T |  |
| src/layer.rs:38 | WALL_WET | `0.6` | ? |  |
| src/layer.rs:39 | FILM_START | `0.35` | ? |  |
| src/layer.rs:40 | FILM_POROUS_HOLD | `0.6` | T |  |
| src/layer.rs:41 | FILM_ROUGHNESS | `0.25` | ? |  |
| src/layer.rs:43 | WATER_REFLECTANCE | `0.354` | ? |  |
| src/layer.rs:50 | PUDDLE_REACH | `0.3` | T |  |
| src/layer.rs:51 | PUDDLE_DEPTH | `0.1` | T |  |
| src/layer.rs:52 | PUDDLE_EDGE | `0.1` | ? |  |
| src/layer.rs:53 | PUDDLE_FLAT_LOW | `0.994_522` | ? |  |
| src/layer.rs:54 | PUDDLE_FLAT_HIGH | `0.998_630` | ? |  |
| src/layer.rs:55 | BASIN_FULL_M | `0.1` | T |  |
| src/layer.rs:56 | BASIN_POOL | `0.35` | ? |  |
| src/layer.rs:57 | PUDDLE_ROUGHNESS | `0.06` | ? |  |
| src/layer.rs:58 | POOL_TILE_M | `64.0` | T |  |
| src/layer.rs:60 | HEIGHT_FROM_LUMA | `2.0` | T |  |
| src/layer.rs:65 | RIPPLE_CELL_M | `1.6` | T |  |
| src/layer.rs:66 | RIPPLE_RADIUS | `0.2` | T |  |
| src/layer.rs:67 | RIPPLE_RING | `0.166_666_67` | T |  |
| src/layer.rs:68 | RIPPLE_RATE | `1.2` | T |  |
| src/layer.rs:69 | RIPPLE_TILT | `0.5` | ? |  |
| src/layer.rs:70 | RIPPLE_FADE_NEAR_M | `0.02` | T |  |
| src/layer.rs:71 | RIPPLE_FADE_FAR_M | `0.06` | T |  |
| src/layer.rs:72 | RIPPLE_TIME_WRAP_S | `1000.0` | T |  |
| src/layer.rs:82 | COVER_UP_LOW | `0.45` | ? |  |
| src/layer.rs:83 | COVER_UP_HIGH | `0.8` | ? |  |
| src/layer.rs:86 | COVER_SHADING_SHARE | `0.25` | ? |  |
| src/layer.rs:87 | COVER_EDGE | `0.08` | ? |  |
| src/layer.rs:88 | COVER_HEIGHT_MIX | `0.5` | T |  |
| src/layer.rs:89 | COVER_DRIFT | `0.5` | ? |  |
| src/layer.rs:90 | COVER_DRIFT_TILE_M | `64.0` | T |  |
| src/layer.rs:91 | COVER_NORMAL | `1.0` | ? |  |
| src/layer.rs:93 | COVER_TOP_MAX | `0.999` | T |  |
| src/layer.rs:95 | ASH_REFLECTANCE | `0.5` | ? |  |
| src/layer.rs:98 | SPARKLE_CELL_M | `0.02` | T |  |
| src/layer.rs:99 | SPARKLE_DENSITY | `0.03` | T |  |
| src/layer.rs:100 | SPARKLE_TILT | `0.35` | ? |  |
| src/layer.rs:101 | SPARKLE_ROUGHNESS | `0.1` | ? |  |
| src/layer.rs:102 | SPARKLE_FADE_NEAR_M | `0.006` | T |  |
| src/layer.rs:103 | SPARKLE_FADE_FAR_M | `0.02` | T |  |
| src/layer.rs:107 | ORIGIN_GRID_M | `256.0` | T |  |
| src/layer.rs:161 | LUMA | `[0.2126, 0.7152, 0.0722]` | ? |  |
| src/lib.rs:86 | SurfaceTuning.catch_up_h | `72.0` | T | settings default |
| src/lib.rs:87 | SurfaceTuning.dry_h | `2.0` | T | settings default |
| src/lib.rs:88 | SurfaceTuning.puddle_dry_mult | `0.5` | T |  |
| src/lib.rs:90 | SurfaceTuning.ash_clear_mult | `0.25` | T |  |
| src/lib.rs:91 | SurfaceTuning.puddle_from_wet | `0.7` | T | settings default |
| src/lib.rs:93 | SurfaceTuning.ash_wash | `2.0` | T | settings default |
| src/lib.rs:94 | SurfaceTuning.storm_wet_rate | `2.0` | T |  |
| src/save.rs:20 | QUANTUM | `1.0 / 65535.0` | ? |  |
| src/trail.rs:30 | RING_SIZE | `1024` | T |  |
| src/trail.rs:32 | RING_TEXEL_M | `0.0625` | T |  |
| src/trail.rs:34 | STAMP_RANGE_M | `40.0` | T |  |
| src/trail.rs:36 | MAX_STAMPS | `64` | T |  |
| src/trail.rs:38 | LIP_HEIGHT | `0.25` | T |  |
| src/trail.rs:40 | LIP_WIDTH | `0.6` | T |  |
| src/trail.rs:42 | WALL | `0.3` | ? |  |
| src/trail.rs:45 | REFILL | `6.0` | ? |  |
| src/trail.rs:47 | REFILL_QUANTUM_M | `0.002` | T |  |
| src/trail.rs:50 | FOOT_HALF_M | `Vec2::new(0.16, 0.09)` | T |  |
| src/view.rs:13 | WINDOW_CELLS | `128` | ? |  |
| src/view.rs:213 | SHELTER_SIZE | `1024` | T |  |
| src/view.rs:215 | SHELTER_HALF_M | `64.0` | T |  |
| src/view.rs:217 | SHELTER_TEXEL_M | `2.0 * SHELTER_HALF_M / 1024.0` | T |  |
| src/view.rs:219 | SHELTER_UP_M | `200.0` | T |  |
| src/view.rs:220 | SHELTER_DOWN_M | `150.0` | T |  |
| src/view.rs:222 | SHELTER_RECENTER_M | `16.0` | T |  |
| src/view.rs:223 | SHELTER_RECENTER_Y_M | `8.0` | T |  |
| src/view.rs:225 | SHELTER_REFRESH_FRAMES | `30` | T |  |
| src/view.rs:230 | SHELTER_LEAF_PASS | `0.4` | ? |  |
| src/view.rs:357 | EXPOSURE_TAU_S | `0.8` | T |  |
| src/view.rs:360 | EXPOSURE_RUN_S | `5.0 * EXPOSURE_TAU_S` | T |  |
| src/view.rs:494 | NOISE_SIZE | `256` | T |  |

Stays const: KEEP (small index), RIPPLE_GRIDS (small index), PARAM_SLOTS (layout/math/id).

### sf_surface_bevy (123: T 56, C 41, ? 26)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/cover.rs:36 | COVER_SIZE | `512` | T |  |
| src/cover.rs:38 | SLICES_PER_CHANNEL | `2` | ? |  |
| src/cover.rs:76 | DERIVED_NORMAL_STRENGTH | `3.0` | T |  |
| src/cover.rs:78 | DERIVED_HEIGHT_PERCENTILES | `(0.02, 0.98)` | T |  |
| src/exposure.wgsl:30 | SHELTER_NEAR_M | `0.12` | T | shader const: needs a uniform |
| src/exposure.wgsl:31 | SHELTER_FULL_M | `0.35` | T | shader const: needs a uniform |
| src/exposure.wgsl:32 | SHELTER_FADE_START | `0.875` | T | shader const: needs a uniform |
| src/kickup.rs:53 | GROUND_WIND | `0.5` | ? |  |
| src/kickup.rs:63 | KickupSettings.on | `true` | T | settings default |
| src/kickup.wesl:28 | PUFF_REFLECTANCE | `0.2` | ? |  |
| src/kickup.wesl:30 | SEED_STEP | `vec3<f32>(0.618034, 0.7548777, 0.5698...` | T | shader const: needs a uniform |
| src/kickup.wesl:32 | FLOOR_BELOW_M | `0.05` | T | shader const: needs a uniform |
| src/kickup.wesl:34 | RANGE_M | `30.0` | T | shader const: needs a uniform |
| src/kickup.wesl:36 | SOFT_M | `0.12` | T | shader const: needs a uniform |
| src/layer.wesl:88 | CLASS_FAR | `3u` | T | shader const: needs a uniform |
| src/layer.wesl:92 | SHELTER_NORMAL_OFFSET_M | `0.35` | T | shader const: needs a uniform |
| src/layer.wesl:95 | SHELTER_NEAR_M | `0.12` | T | shader const: needs a uniform |
| src/layer.wesl:96 | SHELTER_FULL_M | `0.35` | T | shader const: needs a uniform |
| src/layer.wesl:98 | SHELTER_FADE_START | `0.875` | T | shader const: needs a uniform |
| src/layer.wesl:101 | SHELTER_LAG_TOP_M | `1.0` | T | shader const: needs a uniform |
| src/layer.wesl:104 | POROSITY_ROUGH_LOW | `0.3` | ? |  |
| src/layer.wesl:105 | POROSITY_ROUGH_SPAN | `0.5` | ? |  |
| src/layer.wesl:107 | WET_DARK_POROUS | `0.2` | ? |  |
| src/layer.wesl:109 | WET_SATURATION | `0.3` | T | shader const: needs a uniform |
| src/layer.wesl:111 | WALL_WET | `0.6` | ? |  |
| src/layer.wesl:114 | FILM_START | `0.35` | ? |  |
| src/layer.wesl:115 | FILM_POROUS_HOLD | `0.6` | T | shader const: needs a uniform |
| src/layer.wesl:116 | FILM_ROUGHNESS | `0.25` | ? |  |
| src/layer.wesl:117 | WATER_REFLECTANCE | `0.354` | ? |  |
| src/layer.wesl:120 | PATCH_TILE_M | `32.0` | T | shader const: needs a uniform |
| src/layer.wesl:121 | STREAK_TILE_U_M | `21.333334` | T | shader const: needs a uniform |
| src/layer.wesl:122 | STREAK_TILE_V_M | `18.285715` | T | shader const: needs a uniform |
| src/layer.wesl:127 | PUDDLE_REACH | `0.3` | T | shader const: needs a uniform |
| src/layer.wesl:128 | PUDDLE_DEPTH | `0.1` | T | shader const: needs a uniform |
| src/layer.wesl:129 | PUDDLE_EDGE | `0.1` | ? |  |
| src/layer.wesl:130 | PUDDLE_FLAT_LOW | `0.994522` | ? |  |
| src/layer.wesl:131 | PUDDLE_FLAT_HIGH | `0.998630` | ? |  |
| src/layer.wesl:132 | BASIN_FULL_M | `0.1` | T | shader const: needs a uniform |
| src/layer.wesl:133 | BASIN_POOL | `0.35` | ? |  |
| src/layer.wesl:134 | PUDDLE_ROUGHNESS | `0.06` | ? |  |
| src/layer.wesl:135 | POOL_TILE_M | `64.0` | T | shader const: needs a uniform |
| src/layer.wesl:136 | HEIGHT_FROM_LUMA | `2.0` | T | shader const: needs a uniform |
| src/layer.wesl:139 | RIPPLE_CELL_M | `1.6` | T | shader const: needs a uniform |
| src/layer.wesl:140 | RIPPLE_RADIUS | `0.2` | T | shader const: needs a uniform |
| src/layer.wesl:141 | RIPPLE_RING | `0.16666667` | T | shader const: needs a uniform |
| src/layer.wesl:142 | RIPPLE_RATE | `1.2` | T | shader const: needs a uniform |
| src/layer.wesl:143 | RIPPLE_TILT | `0.5` | ? |  |
| src/layer.wesl:144 | RIPPLE_FADE_NEAR_M | `0.02` | T | shader const: needs a uniform |
| src/layer.wesl:145 | RIPPLE_FADE_FAR_M | `0.06` | T | shader const: needs a uniform |
| src/layer.wesl:146 | RIPPLE_TIME_WRAP_S | `1000.0` | T | shader const: needs a uniform |
| src/layer.wesl:147 | RIPPLE_SCALE_0 | `1.0` | T | shader const: needs a uniform |
| src/layer.wesl:148 | RIPPLE_SCALE_1 | `1.25` | T | shader const: needs a uniform |
| src/layer.wesl:149 | RIPPLE_SCALE_2 | `0.8` | T | shader const: needs a uniform |
| src/layer.wesl:150 | RIPPLE_SCALE_3 | `0.625` | T | shader const: needs a uniform |
| src/layer.wesl:154 | COVER_UP_LOW | `0.45` | ? |  |
| src/layer.wesl:155 | COVER_UP_HIGH | `0.8` | ? |  |
| src/layer.wesl:156 | COVER_SHADING_SHARE | `0.25` | ? |  |
| src/layer.wesl:157 | COVER_EDGE | `0.08` | ? |  |
| src/layer.wesl:158 | COVER_HEIGHT_MIX | `0.5` | T | shader const: needs a uniform |
| src/layer.wesl:159 | COVER_DRIFT | `0.5` | ? |  |
| src/layer.wesl:160 | COVER_DRIFT_TILE_M | `64.0` | T | shader const: needs a uniform |
| src/layer.wesl:161 | COVER_NORMAL | `1.0` | ? |  |
| src/layer.wesl:162 | COVER_TOP_MAX | `0.999` | T | shader const: needs a uniform |
| src/layer.wesl:163 | ASH_REFLECTANCE | `0.5` | ? |  |
| src/layer.wesl:166 | SPARKLE_CELL_M | `0.02` | T | shader const: needs a uniform |
| src/layer.wesl:169 | ORIGIN_GRID_M | `256.0` | T | shader const: needs a uniform |
| src/layer.wesl:170 | SPARKLE_DENSITY | `0.03` | T | shader const: needs a uniform |
| src/layer.wesl:171 | SPARKLE_TILT | `0.35` | ? |  |
| src/layer.wesl:172 | SPARKLE_ROUGHNESS | `0.1` | ? |  |
| src/layer.wesl:173 | SPARKLE_FADE_NEAR_M | `0.006` | T | shader const: needs a uniform |
| src/layer.wesl:174 | SPARKLE_FADE_FAR_M | `0.02` | T | shader const: needs a uniform |
| src/layer.wesl:181 | TRAIL_EDGE_FADE_M | `2.0` | T | shader const: needs a uniform |
| src/layer.wesl:182 | TRAIL_LIP_HEIGHT | `0.25` | T | shader const: needs a uniform |
| src/layer.wesl:184 | LUMA | `vec3(0.2126, 0.7152, 0.0722)` | ? |  |
| src/lib.rs:108 | SurfaceSettings.layer | `true` | T | settings default |
| src/lib.rs:109 | SurfaceSettings.wet | `true` | T | settings default |
| src/lib.rs:110 | SurfaceSettings.puddles | `true` | T | settings default |
| src/lib.rs:111 | SurfaceSettings.shelter | `true` | T | settings default |
| src/lib.rs:112 | SurfaceSettings.cover | `true` | T | settings default |
| src/lib.rs:113 | SurfaceSettings.trails | `true` | T | settings default |
| src/lib.rs:114 | SurfaceSettings.raise | `true` | T | settings default |
| src/uniform.rs:22 | U_TRAIL_PREV | `15` | T |  |

Stays const: PARAM_BYTES (layout/math/id), MODE_FRESH (layout/math/id), MODE_CARRY (layout/math/id), MODE_EASE (layout/math/id), TAU (layout/math/id), FLAG_WET (layout/math/id), FLAG_PUDDLE (layout/math/id), FLAG_SNOW (layout/math/id), FLAG_ASH (layout/math/id), FLAG_RAISE (layout/math/id), DEBUG_OFF (layout/math/id), DEBUG_EXPOSURE (layout/math/id), DEBUG_CELLS (layout/math/id), DEBUG_POROSITY (layout/math/id), DEBUG_TRAILS (layout/math/id), CLASS_STATIC (shader enum), CLASS_TERRAIN (shader enum), CLASS_DYNAMIC (shader enum), COVER_SNOW (shader enum), COVER_ASH (shader enum), SHELTER_SUBVIEW (sentinel/derived), CANOPY_SUBVIEW (sentinel/derived), U_SHELTER_CLIP (small index), U_WINDOW (small index), U_CELLS (small index), U_SHELTER (small index), U_MISC (small index), U_TIME (small index), U_CANOPY (small index), U_COVER (small index), U_COVER_UV (small index), U_TRAIL (small index), U_TRAIL_CELLS (small index), U_TRAIL_PREV_CELLS (small index), U_SLOTS (layout/math/id), FLAG_WET (layout/math/id), FLAG_PUDDLE (layout/math/id), FLAG_SNOW (layout/math/id), FLAG_ASH (layout/math/id), FLAG_RAISE (layout/math/id), SurfaceUniform.flags (layout/math/id).

### sf_weather (99: T 42, C 36, ? 21)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/frame.rs:22 | LIGHTNING_CAP_PER_HOUR | `120.0` | ? |  |
| src/frame.rs:24 | LIGHTNING_BUCKET_HOURS | `1.0 / 30.0` | T |  |
| src/frame.rs:26 | LIGHTNING_MAX_SPAN_HOURS | `1.0` | T |  |
| src/frame.rs:28 | BORDER_BAND_M | `256.0` | T |  |
| src/frame.rs:30 | MAX_VISIBILITY_M | `100_000.0` | T |  |
| src/frame.rs:92 | SPEED_OF_SOUND | `343.0` | T |  |
| src/frame.rs:171 | WeatherFrame.lightning_per_hour | `0.0` | ? |  |
| src/frame.rs:564 | WORLD_STRIKE_CELL_M | `2000.0` | T |  |
| src/frame.rs:566 | WORLD_STRIKE_RADIUS_M | `16_000.0` | T |  |
| src/hash.rs:55 | PHASE | `1` | ? |  |
| src/hash.rs:56 | JITTER | `2` | T |  |
| src/hash.rs:57 | PICK | `3` | ? |  |
| src/hash.rs:58 | WIND_SPEED | `4` | T |  |
| src/hash.rs:59 | WIND_DIR | `5` | ? |  |
| src/hash.rs:60 | STRIKE_COUNT | `6` | T |  |
| src/hash.rs:61 | STRIKE | `7` | ? |  |
| src/map.rs:182 | COVERAGE | `0` | T |  |
| src/map.rs:184 | DENSITY | `2` | T |  |
| src/map.rs:191 | CLOUD_SPEED | `8` | T |  |
| src/map.rs:193 | CIRRUS_HEIGHT | `10` | T |  |
| src/map.rs:203 | FOG_DENSITY | `17` | T |  |
| src/map.rs:204 | FOG_FALLOFF | `18` | T |  |
| src/map.rs:206 | BANK_DENSITY | `20` | T |  |
| src/map.rs:207 | BANK_FALLOFF | `21` | T |  |
| src/map.rs:208 | BANK_COVERAGE | `22` | T |  |
| src/map.rs:363 | MAP_SIZE | `256` | T |  |
| src/map.rs:365 | MAP_LAYERS | `4` | ? |  |
| src/precip_wind.rs:25 | CALM_MPS | `1.5` | T |  |
| src/precip_wind.rs:28 | DIRECTION_TAU_S | `30.0` | T |  |
| src/precip_wind.rs:30 | SPEED_TAU_S | `8.0` | T |  |
| src/precip_wind.rs:32 | TURN_MIN_MPS | `0.25` | T |  |
| src/precip_wind.rs:35 | DROP_HEIGHT_M | `3.0` | T |  |
| src/precip_wind.rs:37 | UNCAPPED_DEG | `90.0` | T |  |
| src/precip_wind.rs:73 | PrecipWindFilter.speed | `0.0` | T |  |
| src/records.rs:70 | TransitionTiming.clouds | `Window::new(0.0, 0.6)` | ? |  |
| src/records.rs:71 | TransitionTiming.sky | `Window::new(0.0, 0.6)` | ? |  |
| src/records.rs:72 | TransitionTiming.wind | `Window::new(0.2, 0.8)` | ? |  |
| src/records.rs:73 | TransitionTiming.fog | `Window::new(0.4, 1.0)` | ? |  |
| src/records.rs:74 | TransitionTiming.precipitation | `Window::new(0.4, 1.0)` | ? |  |
| src/records.rs:75 | TransitionTiming.precipitation_out | `Window::new(0.0, 0.6)` | ? |  |
| src/records.rs:76 | TransitionTiming.cues | `Window::new(0.4, 1.0)` | ? |  |
| src/records.rs:77 | TransitionTiming.cues_out | `Window::new(0.0, 0.6)` | ? |  |
| src/records.rs:239 | FogTerms.height_density | `0.0` | T |  |
| src/records.rs:242 | FogTerms.bank_density | `0.0` | T |  |
| src/records.rs:309 | WANDER_HOURS | `3.0` | T |  |
| src/records.rs:316 | WindTerms.speed | `[1.5, 3.5]` | T |  |
| src/records.rs:319 | WindTerms.direction_deg | `15.0` | T |  |
| src/records.rs:320 | WindTerms.direction_jitter_deg | `60.0` | T |  |
| src/records.rs:321 | WindTerms.storm | `0.0` | ? |  |
| src/records.rs:358 | Lightning.per_hour | `12.0` | ? |  |
| src/records.rs:358 | Lightning.flash | `1.0` | ? |  |
| src/records.rs:358 | Lightning.fade_s | `0.25` | T |  |
| src/records.rs:358 | Lightning.distance_m | `[300.0, 6_000.0]` | T |  |
| src/records.rs:392 | OneShotSet.interval_s | `[1.0, 5.0]` | T |  |
| src/records.rs:392 | OneShotSet.volume | `1.0` | T |  |
| src/records.rs:451 | WeatherPreset.climate_ambience | `1.0` | ? |  |
| src/records.rs:480 | Climate.change_interval_hours | `(8.0, 16.0)` | T |  |
| src/records.rs:481 | Climate.transition_hours | `1.5` | T |  |
| src/records.rs:530 | PrecipKind.fall_speed | `7.0` | T |  |
| src/records.rs:531 | PrecipKind.size | `[0.002, 0.004]` | T |  |
| src/records.rs:535 | PrecipKind.extinction | `2e-3` | ? |  |
| src/records.rs:536 | PrecipKind.tint | `[0.55, 0.6, 0.65, 0.3]` | ? |  |
| src/state.rs:43 | MAX_DEPTH | `6` | T |  |

Stays const: CLOUD_TYPE (small index), WETNESS (small index), STORM (small index), EROSION (small index), BASE (small index), THICKNESS (small index), CIRRUS (small index), DIMMING (small index), HAZE (small index), DUST (small index), WIND_X (small index), WIND_Z (small index), GUSTINESS (small index), FOG_BASE (small index), PRECIP (small index), PRECIP_EXTINCTION (small index), LIGHTNING (small index), WET (small index), PUDDLE (small index), SNOW (small index), ASH (small index), DRYING (small index), PRECIP_TINT_R (small index), PRECIP_TINT_G (small index), PRECIP_TINT_B (small index), LEN (layout/math/id), MAP_EPSILON (layout/math/id), PrecipWindFilter.started (runtime state), CHANNELS (layout/math/id), CLOUDS (small index), SKY (small index), WIND (small index), FOG (small index), PRECIP (small index), CUES (small index), EPS (layout/math/id).

### sf_weather_bevy (10: T 8, C 0, ? 2)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/fx.rs:37 | LIGHTNING_LUX | `40_000.0` | T |  |
| src/fx.rs:39 | LIGHTNING_CLOUD_GAIN | `6.0` | T |  |
| src/fx.rs:42 | LIGHTNING_ELEVATION | `0.95` | ? |  |
| src/fx.rs:44 | LIGHTNING_GLOW | `30_000.0` | ? |  |
| src/fx.rs:46 | LIGHTNING_NEAR_M | `[3_000.0, 9_000.0]` | T |  |
| src/fx.rs:48 | LIGHTNING_GLOW_HEIGHT | `0.35` | T |  |
| src/fx.rs:56 | CUE_STEP | `0.02` | T |  |
| src/fx.rs:124 | PrecipWind.slant_cap | `true` | T |  |
| src/lib.rs:70 | MAX_STEP_HOURS | `0.25` | T |  |
| src/lib.rs:124 | MAP_TEXEL_M | `64.0` | T |  |

### sf_wind (19: T 9, C 2, ? 8)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/field.rs:13 | SOURCE_CORE | `0.5` | ? |  |
| src/lib.rs:47 | PERIOD | `1024.0` | ? |  |
| src/shaders/sf_wind.wesl:65 | SOURCE_CORE | `0.5` | ? |  |
| src/source.rs:8 | MAX_SOURCES | `32` | T |  |
| src/state.rs:15 | OCTAVE1_SCALE | `0.37` | T |  |
| src/state.rs:16 | OCTAVE1_DRIFT | `Vec2::new(1.9, 0.6)` | ? |  |
| src/state.rs:19 | PERIOD_CELLS | `(PERIOD / GRID_CELL as f64) as i32` | ? |  |
| src/state.rs:48 | WindSettings.swirl | `0.9` | T | settings default |
| src/state.rs:49 | WindSettings.gust_bias | `0.42` | T |  |
| src/state.rs:50 | WindSettings.gust_band | `0.3` | T | settings default |
| src/state.rs:51 | WindSettings.gust_travel | `1.0` | T | settings default |
| src/state.rs:52 | WindSettings.roughness | `0.1` | T | settings default |
| src/state.rs:53 | WindSettings.reference_height | `10.0` | T |  |
| src/state.rs:54 | WindSettings.max_profile | `2.0` | T |  |
| src/wind.rs:28 | Wind.velocity | `Vec2::new(2.0, 0.5)` | ? |  |
| src/wind.rs:29 | Wind.gustiness | `0.3` | ? |  |
| src/wind.rs:30 | Wind.gust_length | `60.0` | ? |  |

Stays const: OCTAVE1_SEED (sentinel/derived), OCTAVE1_SEED (sentinel/derived).

### sf_wind_bevy (5: T 0, C 5, ? 0)

Stays const: PARAMS_VEC4S (layout/math/id), SOURCE_VEC4S (layout/math/id), DATA_VEC4S (layout/math/id), DATA_BYTES (layout/math/id), WIND_MAX_SOURCES (shader enum).

## statics-lod-farfield

### sf_lod (17: T 12, C 3, ? 2)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/build.rs:39 | FarBakeSettings.light_merge | `[0.0, 12.0, 48.0]` | T | settings default |
| src/cluster.rs:34 | SimplifySettings.crease_cos | `0.77` | T |  |
| src/cluster.rs:34 | SimplifySettings.color_weight | `0.5` | T |  |
| src/cluster.rs:34 | SimplifySettings.prune | `true` | T | settings default |
| src/impostor.rs:49 | ImpostorSettings.frames | `8` | T |  |
| src/impostor.rs:49 | ImpostorSettings.frame_px | `32` | T |  |
| src/impostor.rs:49 | ImpostorSettings.supersample | `2` | T | settings default |
| src/levels.rs:72 | FarLevels.handoff | `[400.0, 2000.0, 8000.0]` | T |  |
| src/levels.rs:72 | FarLevels.band | `0.1` | T | settings default |
| src/levels.rs:72 | FarLevels.min_pixels | `2.0` | T |  |
| src/near.rs:25 | BUCKET_EDGES | `[0.5, 2.0, 8.0]` | ? |  |
| src/policy.rs:83 | FarLight.color | `[1.0, 0.72, 0.42]` | ? |  |
| src/policy.rs:83 | FarLight.intensity | `1.0` | T |  |
| src/policy.rs:83 | FarLight.radius | `6.0` | T |  |

Stays const: FOREVER (layout/math/id), FIRST_LEVEL (layout/math/id), LAST_LEVEL (layout/math/id).

### sf_lod_bevy (8: T 8, C 0, ? 0)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:75 | FarFieldSettings.l1_shadows | `true` | T |  |
| src/lib.rs:75 | FarFieldSettings.light_min_pixels | `2.5` | T |  |
| src/lib.rs:75 | FarFieldSettings.proxy_depth_offset_px | `2.0` | T |  |
| src/shaders/far_shadow_map.wesl:15 | FAR_FADE_START | `0.85` | T | shader const: needs a uniform |
| src/shaders/far_shadow_map.wesl:18 | FAR_PENUMBRA | `0.012` | T | shader const: needs a uniform |
| src/shaders/far_shadow_map.wesl:19 | FAR_PENUMBRA_MIN | `2.0` | T | shader const: needs a uniform |
| src/shaders/far_shadow_map.wesl:22 | FAR_BIAS | `6.0` | T | shader const: needs a uniform |
| src/shaders/far_shadow_map.wesl:23 | FAR_BIAS_PER_M | `0.0015` | T | shader const: needs a uniform |

### sf_model (31: T 20, C 7, ? 4)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/cook.rs:45 | CookSettings.max_lods | `5` | T |  |
| src/cook.rs:46 | CookSettings.min_triangles | `64` | T |  |
| src/cook.rs:47 | CookSettings.min_reduction | `0.25` | T |  |
| src/cook.rs:48 | CookSettings.base_error | `0.01` | T |  |
| src/cook.rs:49 | CookSettings.normal_weight | `0.5` | T |  |
| src/cook.rs:50 | CookSettings.uv_weight | `0.5` | T |  |
| src/cook.rs:51 | CookSettings.color_weight | `0.25` | T |  |
| src/cook.rs:52 | CookSettings.lock_borders | `false` | T | settings default |
| src/cook.rs:53 | CookSettings.overdraw_threshold | `1.05` | T |  |
| src/cooked.rs:36 | RETIRED_FORMAT_VERSIONS | `[]` | ? |  |
| src/lib.rs:217 | Material.double_sided | `false` | ? | bool |
| src/lib.rs:218 | Material.unlit | `false` | ? | bool |
| src/lod.rs:32 | LodScreen.height_px | `1440.0` | T |  |
| src/lod.rs:32 | LodScreen.pixel_error | `1.0` | T |  |
| src/lod.rs:32 | LodScreen.bias | `1.0` | T |  |
| src/lod.rs:32 | LodScreen.crossfade | `0.1` | T |  |
| src/lod.rs:32 | LodScreen.min_pixels | `2.0` | T |  |
| src/lod.rs:32 | LodScreen.min_draw_distance | `24.0` | T |  |
| src/skin.rs:38 | MAX_INFLUENCES | `4` | T |  |
| src/skin.rs:45 | RUNTIME_MORPH_LODS | `2` | T |  |
| src/skin.rs:58 | JOINT_CENTER_WEIGHT | `0.5` | T |  |
| src/skin.rs:61 | MORPH_WEIGHT | `1.0` | T |  |
| src/uv_density.rs:35 | SMALL_AREA | `1e-8` | T |  |
| src/uv_density.rs:37 | TAIL | `0.1` | ? |  |

Stays const: CATALOG_FORMAT (layout/math/id), ATTRS (small index), FORMAT_VERSION (layout/math/id), MIN_FORMAT_VERSION (layout/math/id), KEY_FORMAT (layout/math/id), MORPH_EPSILON (layout/math/id), RULE_VERSION (layout/math/id).

### sf_model_bevy (4: T 3, C 1, ? 0)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/loader.rs:92 | SfMeshLoaderSettings.compress | `true` | T | settings default |
| src/loader.rs:93 | SfMeshLoaderSettings.keep_cpu_copy | `false` | T | settings default |
| src/skinned.rs:18 | MAX_JOINTS | `256` | T |  |

Stays const: MAX_MORPH_TEXELS (sentinel/derived).

### sf_render_order_bevy (6: T 3, C 2, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:58 | OVERLAY_BASE | `1.0e6` | ? |  |
| src/translucent_fog.wesl:40 | FOG_BLEND | `0u` | T | shader const: needs a uniform |
| src/translucent_fog.wesl:41 | FOG_PREMULTIPLIED | `1u` | T | shader const: needs a uniform |
| src/translucent_fog.wesl:43 | FOG_MULTIPLY | `3u` | T | shader const: needs a uniform |

Stays const: NO_FOG (layout/math/id), FOG_ADD (shader enum).

### sf_scene (31: T 8, C 18, ? 5)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/alloc.rs:211 | BLOCK_GRAIN | `64` | ? |  |
| src/bake.rs:281 | SceneSettings.layers | `1` | T | settings default |
| src/bake.rs:281 | SceneSettings.casts_shadow | `true` | T |  |
| src/cull.rs:466 | HorizonParams.margin | `3.0` | T |  |
| src/cull.rs:466 | HorizonParams.min_cells | `4.0` | T |  |
| src/demand.rs:37 | MAX_CLUSTER_RADIUS | `16.0` | T |  |
| src/demand.rs:108 | CLUSTER | `16 + 8` | ? |  |
| src/space.rs:83 | M1_LEVEL | `0` | ? |  |
| src/space.rs:367 | SpaceConfig.space | `0` | T | settings default |
| src/space.rs:369 | SpaceConfig.frames_in_flight | `3` | T |  |
| src/table.rs:96 | MAX_LOD_LEVELS | `16` | T |  |
| src/table.rs:101 | INSTANCE | `80` | ? |  |
| src/table.rs:107 | INSTANCE_CHAIN | `68` | ? |  |

Stays const: CLASSES (small index), MATERIAL_KEY_VERSION (layout/math/id), LAYOUT_VERSION (layout/math/id), DEMAND_FORMAT (layout/math/id), CLUSTER_RULE (layout/math/id), OFFSET (sentinel/derived), PRIME (sentinel/derived), HEADER (layout/math/id), TEXTURE (small index), NO_SLOT (sentinel/derived), DEFAULT_CHAIN_CAPACITY (layout/math/id), CHAIN_GROUP (sentinel/derived), FORMAT_VERSION (layout/math/id), MAX_MODELS (sentinel/derived), HEADER (layout/math/id), CHAIN (small index), LEVEL (small index), MODEL_OFFSET (layout/math/id).

### sf_scene_bevy (5: T 1, C 3, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/residency.rs:70 | SLAB_WAIT_FRAMES | `120` | T |  |
| src/slabs.rs:77 | MATERIAL_RECHECK_EXTRACTS | `120` | ? |  |

Stays const: NON_INDEXED (sentinel/derived), SLOT_BYTES (layout/math/id), SLOT_LEN (sentinel/derived).

## streaming-exterior

### mw_doors_bevy (3: T 3, C 0, ? 0)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:42 | REACH_M | `4.0` | T |  |
| src/lib.rs:44 | NEAR_M | `1.5` | T |  |
| src/lib.rs:46 | MIN_FACING | `0.5` | T |  |

### mw_exterior_bevy (23: T 9, C 12, ? 2)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/exterior.rs:1412 | SOFT_SHADOW_SIZE | `10.0` | T |  |
| src/exterior.rs:1415 | CONTACT_LENGTH | `0.3` | ? |  |
| src/exterior.rs:1425 | BLOOM_INTENSITY | `0.1` | T |  |
| src/exterior.rs:1436 | SUN_DEPTH_BIAS | `0.02` | T |  |
| src/exterior.rs:1438 | SUN_NORMAL_BIAS | `0.8` | T |  |
| src/gi_volume.rs:49 | INTERIOR_AMBIENT_SCALE | `0.1` | T |  |
| src/level.rs:366 | SIZE | `128` | T |  |
| src/trails.rs:28 | PLANTED | `0.5` | ? |  |
| src/trails.rs:31 | DOWN_M | `0.12` | T |  |
| src/trails.rs:33 | ON_TERRAIN_M | `0.4` | T |  |
| src/trails.rs:35 | TOE_SHIFT_M | `0.05` | T |  |

Stays const: MARKER_SPACING (test-scene fixture), SWATCH_SPACING (test-scene fixture), PANEL (test-scene fixture), SPHERE (test-scene fixture), CUBE (test-scene fixture), MODEL_SPACING (test-scene fixture), MODEL_Z (test-scene fixture), TURNTABLE (test-scene fixture), LAMP_ORBIT (test-scene fixture), LAMP_HEIGHT (test-scene fixture), LAMP_RATE (test-scene fixture), LAMP_RADIUS (test-scene fixture).

### mw_world (20: T 14, C 4, ? 2)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/portal.rs:39 | SLAB_MAX_THICKNESS_RATIO | `0.35` | T |  |
| src/portal.rs:41 | HORIZONTAL_NORMAL_Y | `0.7` | ? |  |
| src/portal.rs:44 | READY_MAX_RATIO | `1.15` | T |  |
| src/portal.rs:45 | ADJUST_MAX_RATIO | `2.0` | T |  |
| src/portal.rs:441 | FLOOR_PROBE_HEIGHT_M | `1.2` | T |  |
| src/portal.rs:443 | FLOOR_PROBE_DEPTH_M | `2.0` | T |  |
| src/portal.rs:445 | FLOOR_PROBES_M | `[0.3, 0.6]` | T |  |
| src/portal.rs:447 | FLOOR_MAX_OFFSET_M | `0.6` | T |  |
| src/portal.rs:449 | FLOOR_REACH_M | `30.0` | T |  |
| src/source.rs:95 | SourceOptions.ground_tolerance_m | `0.5` | T |  |
| src/source.rs:95 | SourceOptions.pivot_tolerance_m | `1.0` | T |  |
| src/terrain.rs:46 | LAND_VERTS | `65` | ? |  |
| src/terrain.rs:48 | LAND_TEXELS | `16` | T |  |
| src/terrain.rs:54 | HEIGHT_SCALE | `8.0` | T |  |
| src/terrain.rs:57 | DEFAULT_HEIGHT_UNITS | `-2048.0` | T |  |
| src/worldspaces.rs:58 | PAIR_DISTANCE_M | `8.0` | T |  |

Stays const: UNIT_METERS_F64 (unit/format), CELL_UNITS (unit/format), VERTEX_UNITS (unit/format), TEXEL_UNITS (unit/format).

### sf_world (25: T 21, C 3, ? 1)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/index.rs:198 | IndexParams.far_min_radius | `4.0` | T |  |
| src/reader.rs:82 | DEFAULT_CAP | `256 * 1024 * 1024` | T |  |
| src/rings.rs:84 | RingConfig.active | `RingRadii::new(192.0, 256.0)` | T | settings default |
| src/rings.rs:85 | RingConfig.near | `RingRadii::new(768.0, 896.0)` | T |  |
| src/rings.rs:86 | RingConfig.far | `RingRadii::new(3000.0, 3250.0)` | T |  |
| src/rings.rs:87 | RingConfig.horizon | `8192.0` | T | settings default |
| src/rings.rs:88 | RingConfig.far_min_radius | `4.0` | T |  |
| src/scheduler.rs:159 | Budgets.spawn_ms | `1.0` | T |  |
| src/scheduler.rs:160 | Budgets.activate_ms | `0.5` | T |  |
| src/scheduler.rs:161 | Budgets.deactivate_ms | `0.5` | T |  |
| src/scheduler.rs:162 | Budgets.despawn_ms | `0.5` | T |  |
| src/scheduler.rs:164 | Budgets.fetches_in_flight | `8` | T | settings default |
| src/scheduler.rs:165 | Budgets.assets_in_flight | `64` | T | settings default |
| src/scheduler.rs:166 | Budgets.burst_factor | `20.0` | T |  |
| src/scheduler.rs:233 | SchedulerConfig.door_prefetch | `RingRadii::new(24.0, 32.0)` | T | settings default |
| src/scheduler.rs:234 | SchedulerConfig.lookahead_min | `0.5` | T |  |
| src/scheduler.rs:235 | SchedulerConfig.lookahead_max | `4.0` | T |  |
| src/scheduler.rs:236 | SchedulerConfig.view_weight | `0.5` | T |  |
| src/scheduler.rs:237 | SchedulerConfig.initial_latency | `0.25` | T | settings default |
| src/scheduler.rs:238 | SchedulerConfig.far_by_page | `false` | T |  |
| src/scheduler.rs:239 | SchedulerConfig.linger_secs | `5.0` | T |  |
| src/scheduler.rs:939 | LINGER_CLASS | `3` | ? |  |

Stays const: VERSION (layout/math/id), PLACEMENT_BYTES (layout/math/id), PAGE_SHIFT (layout/math/id).

### sf_world_bevy (32: T 24, C 5, ? 3)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/coords.rs:22 | GRID_SWITCH | `32.0` | ? |  |
| src/debug.rs:30 | StreamingDebug.enabled | `false` | ? | bool |
| src/debug.rs:30 | StreamingDebug.requests | `true` | ? | bool |
| src/debug.rs:30 | StreamingDebug.rings | `true` | T |  |
| src/fade.rs:113 | MIN_HOLD_FRAMES | `3` | T |  |
| src/fade.rs:145 | FadeTransition.fade_secs | `0.3` | T |  |
| src/fade.rs:146 | FadeTransition.max_hold_secs | `10.0` | T |  |
| src/fade.rs:147 | FadeTransition.ready_margin_m | `16.0` | T |  |
| src/fade.rs:154 | FadeTransition.gated | `false` | T | settings default |
| src/matclass.rs:116 | FoldRules.flat_normal_map | `true` | T | settings default |
| src/mesh_slabs.rs:55 | MeshSlabsPlugin.raise_cap | `true` | T |  |
| src/reveal.rs:58 | SETTLE_FRAMES | `3` | T |  |
| src/reveal.rs:81 | RevealGate.max_secs | `10.0` | T |  |
| src/reveal.rs:81 | RevealGate.gates_max_secs | `1.5` | T |  |
| src/reveal.rs:81 | RevealGate.radius_m | `48.0` | T |  |
| src/reveal.rs:81 | RevealGate.stepped | `false` | T |  |
| src/spawn.rs:213 | MAX_SUBTREE_DEPTH | `48` | T |  |
| src/spawn.rs:448 | RANGE_STEPS_PER_OCTAVE | `32.0` | T |  |
| src/stream.rs:54 | MAX_GUARDED_FRAMES | `6` | T |  |
| src/stream.rs:58 | SHIFT_COOLDOWN_FRAMES | `2` | T |  |
| src/stream.rs:129 | StreamingConfig.placements_per_item | `16` | T | settings default |
| src/stream.rs:133 | StreamingConfig.origin_shift_guard_s | `0.05` | T |  |
| src/stream.rs:134 | StreamingConfig.max_loading_meshes | `64` | T |  |
| src/stream.rs:135 | StreamingConfig.precombine | `true` | T | settings default |
| src/stream.rs:136 | StreamingConfig.material_classes | `true` | T | settings default |
| src/stream.rs:137 | StreamingConfig.flat_normal_map | `true` | T | settings default |
| src/stream.rs:138 | StreamingConfig.gpu_cull_statics | `false` | T | settings default |

Stays const: FadeTransition.elapsed (runtime state), FadeTransition.queued_blackout (runtime state), TARGET_MAX_SLAB_BYTES (layout/math/id), MIB (layout/math/id), INLINE_PAIRS (small index).

## terrain-water

### mw_water (17: T 9, C 1, ? 7)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:20 | SEA_FETCH_M | `20_000.0` | T |  |
| src/lib.rs:204 | WaterIni.underwater_color | `[12, 30, 37]` | ? |  |
| src/lib.rs:205 | WaterIni.underwater_color_weight | `0.85` | T |  |
| src/lib.rs:206 | WaterIni.sunrise_fog | `3.0` | ? |  |
| src/lib.rs:207 | WaterIni.day_fog | `2.5` | ? |  |
| src/lib.rs:208 | WaterIni.sunset_fog | `3.0` | ? |  |
| src/lib.rs:209 | WaterIni.night_fog | `4.0` | ? |  |
| src/lib.rs:210 | WaterIni.indoor_fog | `3.0` | ? |  |
| src/lib.rs:211 | WaterIni.ripple_lifetime | `3.0` | T |  |
| src/lib.rs:212 | WaterIni.max_ripples | `75` | T |  |
| src/lib.rs:213 | WaterIni.near_water_radius | `1000.0` | T |  |
| src/lib.rs:214 | WaterIni.near_water_points | `8` | T |  |
| src/lib.rs:215 | WaterIni.near_water_underwater_freq | `0.3` | T |  |
| src/lib.rs:216 | WaterIni.near_water_underwater_volume | `0.9` | T |  |
| src/lib.rs:283 | UNIT_M | `0.9144 / 64.0` | T |  |
| src/lib.rs:286 | LAND_VERTS | `65` | ? |  |

Stays const: CELL_UNITS (unit/format).

### sf_terrain (43: T 19, C 14, ? 10)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/bake.rs:275 | TerrainBakeConfig.fill_height | `0.0` | T |  |
| src/bake.rs:278 | TerrainBakeConfig.read_margin | `32.0` | T |  |
| src/bake.rs:279 | TerrainBakeConfig.min_top_level | `9` | T |  |
| src/cdlod.rs:25 | PATCH_QUADS | `32` | ? |  |
| src/cdlod.rs:27 | MIN_LOD | `-2` | T |  |
| src/cdlod.rs:29 | MAX_LOD | `9` | T |  |
| src/cdlod.rs:76 | RAISE_FADE_START | `8.0` | T |  |
| src/cdlod.rs:79 | RAISE_FADE_END | `12.0` | T |  |
| src/cdlod.rs:82 | RAISE_MAX_LOD | `4` | T |  |
| src/cdlod.rs:86 | RAISE_TEXEL_M | `0.5` | T |  |
| src/cdlod.rs:102 | CdlodSettings.lod0_range | `48.0` | T |  |
| src/cdlod.rs:102 | CdlodSettings.morph_fraction | `0.3` | T |  |
| src/clipmap.rs:19 | CLIPMAP_SIZE | `512` | T |  |
| src/constrain.rs:15 | FOOTPRINT_FEATHER | `1.5` | ? |  |
| src/edits.rs:19 | BRICK_SAMPLES | `64` | T |  |
| src/edits.rs:20 | BRICK_I64 | `64` | ? |  |
| src/product.rs:349 | MIN_EDGE | `4` | T |  |
| src/pyramid.rs:91 | MAX_QUANT_STEP | `0.02` | T |  |
| src/shadow.rs:17 | UNOCCLUDED | `-1.0e6` | ? |  |
| src/shadow.rs:40 | ShadowMarch.steps | `40` | T |  |
| src/shadow.rs:40 | ShadowMarch.first | `4.0` | ? |  |
| src/shadow.rs:40 | ShadowMarch.reach | `6000.0` | T |  |
| src/source.rs:122 | PHASE_QUANTUM | `1e-4` | ? |  |
| src/source.rs:123 | PHASE_TOLERANCE | `5e-4` | T |  |
| src/source.rs:215 | BUCKET | `64` | ? |  |
| src/tile.rs:10 | TILE_QUADS | `512` | ? |  |
| src/tile.rs:14 | QUADS_I64 | `512` | ? |  |
| src/tile.rs:15 | QUADS_F64 | `512.0` | ? |  |
| src/tile.rs:18 | BASE_SPACING | `0.5` | T |  |

Stays const: TerrainBakeConfig.rt_stride (layout/math/id), NO_LAYER_ALBEDO (layout/math/id), STRIP_BLOCK_ROWS (small index), FORMAT (layout/math/id), FORMAT (layout/math/id), FORMAT (layout/math/id), FORMAT (layout/math/id), BLOCK_BYTES (layout/math/id), FORMAT (layout/math/id), TILE_SAMPLES (derived), BLOCK_QUADS (small index), BLOCKS (derived), LAYER_SAMPLES (derived), TINT_SAMPLES (derived).

### sf_terrain_bevy (63: T 32, C 29, ? 2)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/far_map.rs:24 | FAR_MAP_SIZE | `384` | T |  |
| src/far_map.rs:26 | FAR_MAP_TEXEL | `24.0` | T |  |
| src/far_map.rs:32 | REFRESH_COS | `0.999_990_5` | T |  |
| src/far_map.rs:33 | REFRESH_MOVE | `1000.0` | ? |  |
| src/far_map.rs:34 | REFRESH_AGE | `Duration::from_secs(8)` | T |  |
| src/lib.rs:70 | DEFAULT_REFLECTANCE | `0.3` | ? |  |
| src/lib.rs:75 | DEFAULT_LAYER_BUDGET | `1 << 30` | T |  |
| src/lib.rs:116 | TerrainSettings.max_loads | `16` | T |  |
| src/lib.rs:117 | TerrainSettings.snap | `16` | T | settings default |
| src/lib.rs:118 | TerrainSettings.splat_range | `1000.0` | T |  |
| src/lib.rs:119 | TerrainSettings.blend_depth | `0.2` | T |  |
| src/lib.rs:123 | TerrainSettings.raise_m | `0.0` | T |  |
| src/lib.rs:771 | SHIFT_SETTLE_FRAMES | `2` | T |  |
| src/lib.rs:779 | SLACK_M | `0.05` | T |  |
| src/params.rs:12 | P_HEIGHT | `16` | T |  |
| src/params.rs:18 | MAX_LAYERS | `256` | T |  |
| src/params.rs:24 | HEIGHT_LEVELS | `10` | T |  |
| src/params.rs:27 | MORPH_LODS | `12` | T |  |
| src/shaders/common.wesl:43 | MIN_LOD | `-2` | T | shader const: needs a uniform |
| src/shaders/common.wesl:44 | HEIGHT_LEVELS | `10u` | T | shader const: needs a uniform |
| src/shaders/common.wesl:46 | SIZE | `512` | T | shader const: needs a uniform |
| src/shaders/common.wesl:364 | RAISE_FADE_START | `8.0` | T | shader const: needs a uniform |
| src/shaders/common.wesl:365 | RAISE_FADE_END | `12.0` | T | shader const: needs a uniform |
| src/shaders/common.wesl:367 | RAISE_MAX_LOD | `4` | T | shader const: needs a uniform |
| src/shaders/common.wesl:370 | RAISE_TEXEL_M | `0.5` | T | shader const: needs a uniform |
| src/shaders/far_shadow.wesl:34 | FAR_STEPS | `14u` | T | shader const: needs a uniform |
| src/shaders/far_shadow.wesl:37 | FAR_REACH | `5000.0` | T | shader const: needs a uniform |
| src/shaders/far_shadow.wesl:40 | FAR_PENUMBRA | `0.012` | T | shader const: needs a uniform |
| src/shaders/far_shadow.wesl:42 | FAR_FADE_START | `0.85` | T | shader const: needs a uniform |
| src/shaders/far_shadow.wesl:45 | FAR_BIAS_PER_M | `0.0015` | T | shader const: needs a uniform |
| src/shaders/far_shadow.wesl:46 | FAR_FIRST_PER_M | `0.01` | T | shader const: needs a uniform |
| src/shaders/far_shadow.wesl:49 | FAR_MARCH_END | `700.0` | T | shader const: needs a uniform |
| src/shaders/far_shadow.wesl:51 | FAR_MAP_BIAS | `6.0` | T | shader const: needs a uniform |
| src/shaders/far_shadow.wesl:52 | FAR_MAP_PENUMBRA_MIN | `2.0` | T | shader const: needs a uniform |

Stays const: P_CAMERA (small index), P_ORIGIN (small index), P_MISC (small index), P_VALID (small index), P_MORPH (small index), P_LAYER (small index), P_MACRO (small index), P_DETAIL (small index), P_LAYERS (small index), P_FAR (derived), SLOTS (layout/math/id), LAYER_LEVELS (small index), MACRO_LEVELS (small index), DEBUG_LOD (layout/math/id), DEBUG_WIREFRAME (layout/math/id), DEBUG_NO_FAR_SHADOWS (layout/math/id), DEBUG_NO_SPECULAR (layout/math/id), DEBUG_NO_INDIRECT (layout/math/id), DEBUG_RAW_LAYERS (layout/math/id), DEBUG_NO_ENV_SPECULAR (layout/math/id), LAYER_LEVELS (shader enum), DEBUG_LOD (layout/math/id), DEBUG_WIREFRAME (layout/math/id), DEBUG_NO_FAR_SHADOWS (layout/math/id), DEBUG_NO_SPECULAR (layout/math/id), DEBUG_NO_INDIRECT (layout/math/id), DEBUG_RAW_LAYERS (layout/math/id), DEBUG_NO_ENV_SPECULAR (layout/math/id), MACRO_LEVELS (shader enum).

### sf_water (55: T 37, C 3, ? 15)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:39 | TWIN_RESOLUTION | `64` | T |  |
| src/records.rs:157 | WaterOptics.absorption | `[0.45, 0.065, 0.02]` | ? |  |
| src/records.rs:158 | WaterOptics.scattering | `[0.003, 0.004, 0.005]` | T |  |
| src/records.rs:159 | WaterOptics.phase_g | `0.8` | ? |  |
| src/records.rs:160 | WaterOptics.roughness | `0.03` | ? |  |
| src/records.rs:162 | WaterOptics.whitecaps | `1.0` | ? |  |
| src/records.rs:163 | WaterOptics.shore | `1.0` | ? |  |
| src/records.rs:164 | WaterOptics.contact | `1.0` | ? |  |
| src/records.rs:166 | WaterOptics.refraction_strength | `0.3` | T |  |
| src/records.rs:167 | WaterOptics.caustics | `1.0` | ? |  |
| src/sea_wind.rs:45 | SeaWindSettings.response_s | `30.0` | T |  |
| src/sea_wind.rs:46 | SeaWindSettings.blend_s | `6.0` | T |  |
| src/sea_wind.rs:47 | SeaWindSettings.speed_step | `0.03` | T |  |
| src/sea_wind.rs:48 | SeaWindSettings.min_speed_step | `0.15` | T |  |
| src/sea_wind.rs:49 | SeaWindSettings.angle_step_deg | `3.0` | T |  |
| src/sea_wind.rs:50 | SeaWindSettings.calm | `0.5` | T | settings default |
| src/shore.rs:47 | ShoreSettings.range | `256.0` | T |  |
| src/shore.rs:48 | ShoreSettings.open_core | `200.0` | T | settings default |
| src/shore.rs:49 | ShoreSettings.open_falloff | `800.0` | T |  |
| src/shore.rs:50 | ShoreSettings.enclosed_fetch | `2000.0` | T | settings default |
| src/shore.rs:51 | ShoreSettings.enclosed_max | `0.35` | T |  |
| src/shore.rs:52 | ShoreSettings.swell_ramp | `40.0` | T | settings default |
| src/shore.rs:53 | ShoreSettings.chop_ramp | `3.0` | T | settings default |
| src/shore.rs:54 | ShoreSettings.chop_floor | `0.25` | T | settings default |
| src/shore.rs:55 | ShoreSettings.normal_floor | `0.5` | T | settings default |
| src/shore.rs:56 | ShoreSettings.beach_slope | `0.05` | T | settings default |
| src/shore.rs:57 | ShoreSettings.shore_waves | `1.0` | T | settings default |
| src/shore.rs:127 | BIG | `1e20` | ? |  |
| src/shore.rs:132 | BED_SLOPE_MIN | `0.02` | T |  |
| src/shore.rs:133 | BED_SLOPE_MAX | `0.3` | T |  |
| src/shore_waves.rs:62 | MICHE | `0.142 * PI` | ? |  |
| src/shore_waves.rs:64 | STEEPNESS_LIMIT | `0.75` | T |  |
| src/shore_waves.rs:68 | ORBIT_LIMIT | `0.4` | T |  |
| src/shore_waves.rs:70 | MIN_DEPTH | `0.1` | T |  |
| src/shore_waves.rs:72 | NOISE_PERIOD | `1024.0` | T |  |
| src/shore_waves.rs:73 | PHASE_CELLS | `16` | ? |  |
| src/shore_waves.rs:74 | SET_CELLS | `32` | ? |  |
| src/shore_waves.rs:75 | PHASE_NOISE | `0.8` | T |  |
| src/shore_waves.rs:76 | SET_FLOOR | `0.6` | ? |  |
| src/shore_waves.rs:77 | SWASH_SKEW | `0.5` | ? |  |
| src/shore_waves.rs:79 | SWASH_ZONE | `0.3` | ? |  |
| src/shore_waves.rs:81 | ECKART_LINEAR | `4.0` | T |  |
| src/spectrum.rs:29 | GRAVITY | `9.81` | T |  |
| src/spectrum.rs:36 | REPEAT_PERIOD | `256.0` | ? |  |
| src/spectrum.rs:67 | SpectrumParams.speed | `8.0` | T |  |
| src/spectrum.rs:70 | SpectrumParams.fetch | `100_000.0` | T | settings default |
| src/spectrum.rs:72 | SpectrumParams.amplitude | `1.0` | T | settings default |
| src/spectrum.rs:73 | SpectrumParams.choppiness | `0.8` | T | settings default |
| src/spectrum.rs:218 | DEFAULT_CASCADES | `[256, 64, 16, 4]` | T |  |
| src/spectrum.rs:221 | DEFAULT_SPLIT | `2` | T |  |
| src/sspr.rs:28 | MIN_HEIGHT | `0.01` | T |  |
| src/twin.rs:31 | INVERSION_ITERATIONS | `3` | T |  |

Stays const: LEAF (small index), TRAINS (small index), BREAKER_INDEX (layout/math/id).

### sf_water_bevy (32: T 18, C 6, ? 8)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:49 | RESOLUTION | `256` | T |  |
| src/lib.rs:51 | MAX_CASCADES | `4` | T |  |
| src/lib.rs:53 | MAX_LOOKS | `8` | T |  |
| src/lib.rs:161 | WaterReflections.sspr | `true` | ? | bool |
| src/lib.rs:161 | WaterReflections.wave_distortion | `0.6` | T |  |
| src/lib.rs:254 | ClipmapSettings.base_spacing | `0.25` | T |  |
| src/lib.rs:254 | ClipmapSettings.quads | `128` | T | settings default |
| src/lib.rs:254 | ClipmapSettings.rings | `7` | T |  |
| src/lib.rs:254 | ClipmapSettings.skirt | `32_000.0` | T | settings default |
| src/lib.rs:254 | ClipmapSettings.displacement_range | `900.0` | T |  |
| src/shaders/water_common.wesl:71 | SW_SHARES | `vec3(0.5, 0.3, 0.2)` | T | shader const: needs a uniform |
| src/shaders/water_common.wesl:72 | SW_PHASE_SPREAD | `vec3(1.0, -0.7, 1.3)` | T | shader const: needs a uniform |
| src/shaders/water_common.wesl:73 | SW_BREAKER | `0.78` | ? |  |
| src/shaders/water_common.wesl:74 | SW_MICHE | `0.44610616` | ? |  |
| src/shaders/water_common.wesl:75 | SW_STEEPNESS | `0.75` | ? |  |
| src/shaders/water_common.wesl:76 | SW_ORBIT | `0.4` | T | shader const: needs a uniform |
| src/shaders/water_common.wesl:77 | SW_MIN_DEPTH | `0.1` | T | shader const: needs a uniform |
| src/shaders/water_common.wesl:78 | SW_PHASE_NOISE | `0.8` | T | shader const: needs a uniform |
| src/shaders/water_common.wesl:79 | SW_SET_FLOOR | `0.6` | ? |  |
| src/shaders/water_common.wesl:80 | SW_SKEW | `0.5` | ? |  |
| src/shaders/water_common.wesl:81 | SW_ZONE | `0.3` | ? |  |
| src/shaders/water_common.wesl:82 | SW_ECKART_LINEAR | `4.0` | T | shader const: needs a uniform |
| src/shaders/water_common.wesl:83 | SW_GRAVITY | `9.81` | T | shader const: needs a uniform |
| src/shaders/water_common.wesl:84 | SW_REPEAT | `256.0` | ? |  |
| src/shaders/water_fft.wesl:42 | MAX_N | `256u` | T | shader const: needs a uniform |
| src/shaders/water_sspr.wesl:27 | MIN_HEIGHT | `0.01` | T | shader const: needs a uniform |

Stays const: OceanSpectrum.seed (runtime state), KEEP (small index), SSPR_GROUP (layout/math/id), SW_TAU (layout/math/id), LANES (shader enum), TAU (layout/math/id).

## textures

### sf_texture (18: T 7, C 6, ? 5)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/delight.rs:51 | FLOOR | `0.5` | ? |  |
| src/delight.rs:53 | CEILING | `2.0` | ? |  |
| src/delight.rs:57 | LUMINANCE_WEIGHTS | `[0.2126, 0.7152, 0.0722]` | T |  |
| src/delight.rs:59 | DEFAULT_STRENGTH | `0.5` | T |  |
| src/ktx2_out.rs:108 | CLEAR_WEIGHT | `1.0 / 4096.0` | T |  |
| src/ktx2_out.rs:208 | COVERAGE_STEPS | `32` | T |  |
| src/residency.rs:42 | STREAMABLE_ABOVE | `512` | ? |  |
| src/residency.rs:44 | TAIL_SIZE | `128` | T |  |
| src/residency.rs:46 | UPGRADE_STABLE_S | `0.5` | T |  |
| src/residency.rs:48 | DOWNGRADE_WAIT_S | `5.0` | T |  |
| src/residency.rs:50 | FALLBACK_RHO | `1.0` | ? |  |
| src/rewrap.rs:153 | DDS_DATA | `128` | ? |  |

Stays const: EXTENT (small index), CELLS (derived), BLUR_PASSES (small index), BC7_BLOCK_BYTES (layout/math/id), HEADER_LEN (layout/math/id), DX10_LEN (layout/math/id).

### sf_texture_bevy (1: T 1, C 0, ? 0)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:218 | SETTLE | `Duration::from_secs(2)` | T |  |

### sf_texview_bevy (2: T 2, C 0, ? 0)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/lib.rs:44 | TARGET_MAX | `1024` | T |  |
| src/lib.rs:46 | VIEW_HEIGHT_VH | `45.0` | T |  |

## ui

### mw_ui (10: T 5, C 1, ? 4)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/book.rs:100 | IMG_SCHEME_DEFAULT_SIZE | `50.0` | T |  |
| src/book.rs:109 | LEGACY_SIZES | `[0.75, 8.0 / 9.0, 1.0, 1.2, 1.5, 2.0,...` | T |  |
| src/book.rs:121 | BookOptions.trim_after_last_break | `true` | T | settings default |
| src/message.rs:70 | MAX_BUTTONS | `10` | T |  |
| src/message.rs:73 | MAX_FIELD | `1024` | T |  |
| src/message.rs:134 | POSITIVE_SPACE | `1` | ? |  |
| src/message.rs:135 | POSITIVE_SIGN | `2` | ? |  |
| src/message.rs:136 | ALIGN_LEFT | `4` | ? |  |
| src/message.rs:138 | ALTERNATE_FORM | `16` | ? |  |

Stays const: PREPEND_ZERO (layout/math/id).

### sf_console (2: T 0, C 2, ? 0)

Stays const: HISTORY_CAPACITY (layout/math/id), LOG_CAPACITY (layout/math/id).

### sf_console_bevy (9: T 6, C 0, ? 3)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/overlay/inspect.rs:54 | REFRESH_FRAMES | `10` | T |  |
| src/overlay/inspect.rs:56 | MAX_VALUE | `160` | T |  |
| src/overlay/inspect.rs:239 | MAX_ITEMS | `6` | T |  |
| src/overlay/mod.rs:130 | ConsoleStyle.height_percent | `45.0` | T |  |
| src/overlay/mod.rs:131 | ConsoleStyle.lines | `40` | ? |  |
| src/overlay/mod.rs:132 | ConsoleStyle.popup_lines | `8` | ? |  |
| src/overlay/mod.rs:140 | ConsoleStyle.block_game_input | `true` | ? | bool |
| src/overlay/mod.rs:141 | ConsoleStyle.release_cursor | `true` | T |  |
| src/overlay/select.rs:67 | FIND_MAX | `50` | T |  |

### sf_rpg_ui (5: T 1, C 0, ? 4)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/book.rs:114 | PageMetrics.height | `600.0` | T |  |
| src/book.rs:115 | PageMetrics.block_gap | `0.0` | ? |  |
| src/book.rs:116 | PageMetrics.orphans | `2` | ? |  |
| src/book.rs:117 | PageMetrics.widows | `2` | ? |  |
| src/dialogue.rs:29 | DEFAULT_MAX_ENTRIES | `256` | ? |  |

### sf_ui (19: T 13, C 2, ? 4)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/notify.rs:179 | ToastConfig.base | `1.5` | T | settings default |
| src/notify.rs:180 | ToastConfig.per_char | `0.06` | T | settings default |
| src/notify.rs:181 | ToastConfig.min | `2.5` | T | settings default |
| src/notify.rs:182 | ToastConfig.max | `10.0` | T | settings default |
| src/notify.rs:183 | ToastConfig.scale | `1.0` | T |  |
| src/notify.rs:184 | ToastConfig.fade_in | `0.15` | T |  |
| src/notify.rs:185 | ToastConfig.fade_out | `0.5` | T |  |
| src/notify.rs:186 | ToastConfig.max_visible | `4` | T |  |
| src/notify.rs:187 | ToastConfig.max_queued | `32` | T |  |
| src/notify.rs:188 | ToastConfig.merge_repeats | `true` | T | settings default |
| src/rebind.rs:26 | CAPTURE_TIMEOUT | `5.0` | T |  |
| src/rebind.rs:79 | Rebinder.draining | `false` | ? | bool |
| src/rich_text.rs:82 | RunStyle.size | `1.0` | T |  |
| src/rich_text.rs:84 | RunStyle.bold | `false` | ? | bool |
| src/rich_text.rs:85 | RunStyle.italic | `false` | ? | bool |
| src/rich_text.rs:86 | RunStyle.underline | `false` | ? | bool |
| src/settings.rs:50 | AUDIO_VOLUME_STEP | `0.05` | T |  |

Stays const: FIT_EPSILON (layout/math/id), DEFAULT_OVERSCAN (small index).

### sf_ui_bevy (26: T 18, C 1, ? 7)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/device.rs:25 | AXIS_USE | `0.5` | ? |  |
| src/device.rs:27 | MOUSE_USE | `4.0` | ? |  |
| src/keybinds.rs:23 | STICK_PRESS | `0.5` | ? |  |
| src/lib.rs:178 | UiSettings.text_scale | `1.0` | T |  |
| src/lib.rs:179 | UiSettings.hud_scale | `1.0` | T |  |
| src/lib.rs:180 | UiSettings.menu_scale | `1.0` | T |  |
| src/lib.rs:181 | UiSettings.high_contrast | `false` | T |  |
| src/lib.rs:182 | UiSettings.reduce_motion | `false` | T | settings default |
| src/lib.rs:183 | UiSettings.classic_windows | `false` | T | settings default |
| src/nav.rs:47 | STICK_THRESHOLD | `0.5` | T |  |
| src/nav.rs:64 | NavRepeat.delay | `0.4` | T |  |
| src/nav.rs:65 | NavRepeat.interval | `0.08` | T |  |
| src/nav.rs:68 | NavRepeat.next | `0.0` | ? |  |
| src/notify.rs:105 | MESSAGE_BOX_Z | `1 << 20` | T |  |
| src/notify.rs:106 | TOAST_Z | `(1 << 20) + 1` | ? |  |
| src/reader.rs:148 | WHEEL_LINE | `40.0` | ? |  |
| src/router.rs:24 | UI_CONTEXT_PRIORITY | `1 << 16` | T |  |
| src/router.rs:149 | SCREEN_Z_STEP | `1 << 12` | T |  |
| src/text_measure.rs:69 | RichTextTheme.font_size | `18.0` | T |  |
| src/text_measure.rs:70 | RichTextTheme.line_height | `1.25` | T |  |
| src/text_measure.rs:72 | RichTextTheme.block_gap | `0.5` | ? |  |
| src/text_measure.rs:73 | RichTextTheme.image_scale | `1.0` | T |  |
| src/text_measure.rs:183 | MeasureScale.scale_factor | `1.0` | T |  |
| src/text_measure.rs:184 | MeasureScale.text_scale | `1.0` | T |  |
| src/text_measure.rs:227 | CACHE_LIMIT | `8192` | T |  |

Stays const: NavRepeat.elapsed (runtime state).

### sf_ui_inventory (1: T 0, C 1, ? 0)

Stays const: EPSILON (layout/math/id).

### sf_ui_screens (12: T 7, C 0, ? 5)

| Where | Name | Value | Cls | Note |
|---|---|---|---|---|
| src/inventory.rs:60 | InventoryGridStyle.cell | `Vec2::new(64.0, 64.0)` | ? |  |
| src/inventory.rs:61 | InventoryGridStyle.gap | `4.0` | ? |  |
| src/keybinds.rs:87 | LABEL_WIDTH | `200.0` | T |  |
| src/keybinds.rs:88 | CELL_WIDTH | `140.0` | T |  |
| src/keybinds.rs:89 | CELL_GAP | `8.0` | ? |  |
| src/loading.rs:33 | SWEEP_SECONDS | `1.5` | ? |  |
| src/loading.rs:35 | SWEEP_WIDTH | `25.0` | T |  |
| src/options.rs:64 | DEFAULT_FLOAT_STEPS | `20.0` | T |  |
| src/options.rs:67 | THUMB_WIDTH | `10.0` | T |  |
| src/options.rs:70 | PANE_WIDTH | `280.0` | T |  |
| src/saveload.rs:244 | THUMB | `(128.0, 72.0)` | ? |  |
| src/widgets.rs:98 | ScreenColors.font_size | `18.0` | T |  |


## Appendix: the generator (`tools/tunables-census.mjs`)

```js
// Tunables census (superfargoth/docs/specs/tunables-census.md). Node >= 18, no deps.
// From the repo root: node census.mjs            -> totals per system
//                     node census.mjs --md       -> the census tables (markdown)
//                     node census.mjs --json     -> raw rows
// Lists numeric/bool consts (Rust + WESL) and literal fields of every `impl Default for X`, skipping
// tests, examples, benches, vendor and in-file `#[cfg(test)] mod` blocks, and classifies each by its
// name and value: T tunable, C stays const, ? unsure. The classification is a first pass (rules below);
// tunables.md "What stays a const" is the rule a human applies when migrating.
import fs from 'node:fs';
import path from 'node:path';

const ROOTS = ['superfargoth/crates', 'bevyrrowind/crates', 'bevyrrowind/apps'];
const SKIP_DIR = new Set(['target', 'tests', 'examples', 'benches', 'vendor', 'workspace-hack', 'node_modules']);
const files = [];
function walk(d) {
  for (const e of fs.readdirSync(d, { withFileTypes: true })) {
    const p = path.join(d, e.name);
    if (e.isDirectory()) { if (!SKIP_DIR.has(e.name)) walk(p); }
    else if (/\.(rs|wesl|wgsl)$/.test(e.name) && !/(^tests?[_.]|_tests?\.rs$|^test_)/.test(e.name)) files.push(p);
  }
}
ROOTS.forEach(walk);
files.sort();

const NUM_T = '(bool|\\((?:[a-z0-9]+, *)+[a-z0-9]+\\)|\\[(?:f32|f64|u32|usize); *\\d+\\]|f32|f64|u8|u16|u32|u64|usize|i8|i16|i32|i64|isize|Duration|Vec2|Vec3|Vec4|UVec2|UVec3|IVec2)';
const RS_CONST = new RegExp(`^\\s*(?:pub(?:\\([a-z]+\\))?\\s+)?const\\s+([A-Z][A-Z0-9_]*)\\s*:\\s*${NUM_T}\\s*=\\s*(.+?);?\\s*$`);
const WESL_CONST = /^\s*const\s+([A-Z][A-Z0-9_]*)\s*(?::\s*([\w<>]+))?\s*=\s*(.+?);\s*$/;
const DEFAULT_IMPL = /impl Default for (\w+)\b/;
const LIT = '-?[\\d_.]+(?:e-?\\d+)?(?:_?f32|_?f64|_?u32|_?usize)?|Duration::from_\\w+\\([^)]*\\)|\\w+::new\\([^()]*\\)|\\[[\\d_., -]+\\]|\\([\\d_., -]+\\)|true|false';
const FIELD_LIT = new RegExp(`^\\s+([a-z_][a-z0-9_]*)\\s*:\\s*(${LIT})\\s*,?\\s*(?:\\/\\/.*)?$`);
const INLINE_FIELD = new RegExp(`\\b([a-z_][a-z0-9_]*)\\s*:\\s*(${LIT})\\s*(?=,|\\s*\\})`, 'g');

const CONST_RE = /(^|_)(VERSION|MAGIC|SEED|EPS|EPSILON|PI|TAU|SQRT2?|LN2|MIB|KIB|GIB|BYTES|STRIDE|ALIGN|ALIGNMENT|MASK|BITS|TAG|HEADER|FORMAT|LAYOUT|LEN|VEC4S|WORDS|INDEX|IDX|ID|KIND|FLAG|FLAGS|SENTINEL|NONE|INVALID|UNSET|UP|FORWARD|RIGHT|ZERO|ONE|BINDING|GROUP|LOCATION|WORKGROUP|WORKGROUP_SIZE|SLOT|SLOTS|OFFSET|PAGE_SHIFT|PAGE_SIZE|PRIME|CHANNELS|RULE|CAPACITY)$|^(FLAG|DEBUG|NO|FIRST|LAST|MODE)_|FOREVER/;
const UNIT_RE = /(UNITS?_PER_|_PER_UNIT|_TO_M$|^M_PER|UNIT_METERS|GAME_UNITS|CELL_SIZE|CELL_UNITS|VERTEX_UNITS|TEXEL_UNITS|INCH|FEET)/;
const TUNE_RE = /(DIST|RANGE|RADIUS|REACH|FADE|BUDGET|NEAR|FAR|RING|LOD|CASCADE|SHADOW|HYST|THRESH|COOLDOWN|DELAY|TIMEOUT|HOLD|FRAMES|SECS|_S$|_MS$|_US$|_SEC$|HOURS|TAU_|SPEED|MPS|GAIN|INTENSITY|BIAS|SCALE|RATE|LIMIT|CAP$|^MAX_|^MIN_|_MAX$|_MIN$|PER_FRAME|PER_TICK|_M$|_M2$|METERS|LUX|EV$|EXPOSURE|STRENGTH|WEIGHT|DAMP|STIFF|ACCEL|MARGIN|SLACK|TOLERANCE|ANGLE|_DEG$|_RAD$|PX$|PIXELS|RES$|RESOLUTION|SAMPLES|STEPS|ITERS?$|QUEUE|CONCURRENT|THREADS|INTERVAL|DURATION|BLEND|SMOOTH|FACTOR|MULT|BOOST|DENSITY|SIZE|SPACING|HEIGHT|DEPTH|WIDTH|COUNT|BURST|BATCH|SPLIT|ATTEN|FALLOFF|CUTOFF|SOFT|SHARP|CONTRAST|SATURATION|JITTER|NOISE|SPREAD|PCT|PERCENT|RATIO|FRACTION|LEAD|LAG|AHEAD|GRACE|WARM|SETTLE|STEP|AREA|VOLUME|TEMP|MASS|FORCE|IMPULSE|FRICTION|GRAVITY|LIFETIME|TTL|AGE|HALFLIFE|HANDOFF|SIGNIFICANCE|RELEASE|FOLLOW|COS$|PRIORITY|TIMESCALE|PENUMBRA|SUBSTEP|REDUCTION|ERROR|TRIANGLES|FACING|PROBE|OFFSET_M|ORBIT|TEXELS|RAYS)/;
const STATE_FIELD = /^(elapsed|timer|age|current|state|phase|frame|frames_left|seed|predicted_us|demoted|queued_\w+|last_\w+|next_\w+|count|generation|revision|dirty|started|done)$/;
const CONFIG_STRUCT = /(Settings|Config|Params|Budget|Budgets|Tuning|Options|Lod|Ranges|Limits|Rings|Quality|Policy|Tiers|Thresholds|Fade|Transition|Spec|Gate|Guards|Rules|Levels|Profile)$/;
const FIXTURE_FILE = /\/(level|levels|lab|labs)\/|_lab\.rs$|\/viewers?\b|mw_viewers_bevy/;

function system(file) {
  const c = file.split('/')[2];
  const m = [
    [/^(sf_shadow_cache_bevy|sf_gi)/, 'shadows-lights-gi'],
    [/^(sf_world|mw_exterior_bevy|mw_doors_bevy|mw_world)/, 'streaming-exterior'],
    [/^(sf_scene|sf_lod|sf_render_order_bevy|sf_model)/, 'statics-lod-farfield'],
    [/^(sf_anim|sf_character|mw_anim|mw_actors|mw_walkers)/, 'animation-characters'],
    [/^(sf_weather|sf_surface|sf_precip|sf_atmos|sf_sky|sf_wind|mw_weather|mw_environment)/, 'sky-weather-surface'],
    [/^(sf_terrain|sf_water|mw_water)/, 'terrain-water'],
    [/^(sf_texture|sf_texview)/, 'textures'],
    [/^(sf_action|sf_controller|sf_camera|sf_combat|mw_player|mw_combat|mw_controls)/, 'feel-camera-combat'],
    [/^(sf_physics|sf_ragdoll|mw_physics|mw_delirium)/, 'physics'],
    [/^(sf_ai|sf_nav|mw_ai)/, 'ai-nav'],
    [/^(sf_audio|mw_audio)/, 'audio'],
    [/^(sf_ui|sf_rpg_ui|mw_ui|sf_console)/, 'ui'],
    [/^(sf_diag|bevyrrowind$|mw_devtools|mw_game_core|mw_boot)/, 'app-diag'],
    [/^(sf_effects|mw_effects)/, 'effects'],
  ].find(([re]) => re.test(c));
  return m ? m[1] : 'other';
}

function classify(r) {
  const field = r.src === 'default';
  const base = field ? r.name.split('.')[1] : r.name;
  const N = base.toUpperCase();
  const v = r.value;
  if (FIXTURE_FILE.test(r.file)) return ['C', 'test-scene fixture'];
  if (field && STATE_FIELD.test(base)) return ['C', 'runtime state'];
  if (/::MAX|::MIN|::ALL|size_of|^0x/.test(v)) return ['C', 'sentinel/derived'];
  if (UNIT_RE.test(N)) return ['C', 'unit/format'];
  if (CONST_RE.test(N)) return ['C', 'layout/math/id'];
  if (/^[A-Z_][A-Z0-9_]*(\s*[-+*\/]\s*[A-Z0-9_.() ]+)+$/.test(v) && !/\d\.\d/.test(v) && /[A-Z]/.test(v)) return ['C', 'derived'];
  if (r.src === 'wesl' && /^\d+u$/.test(v) && !TUNE_RE.test(N)) return ['C', 'shader enum'];
  if (TUNE_RE.test(N)) return ['T', r.src === 'wesl' ? 'shader const: needs a uniform' : ''];
  if (field && CONFIG_STRUCT.test(r.name.split('.')[0])) return ['T', 'settings default'];
  if (/^(true|false)$/.test(v)) return ['?', 'bool'];
  if (r.ty === 'usize' && /^\d+$/.test(v) && +v < 64) return ['C', 'small index'];
  return ['?', ''];
}

const rows = [];
for (const f of files) {
  const rel = f.replace(/\\/g, '/');
  const lines = fs.readFileSync(f, 'utf8').split(/\r?\n/);
  let inDefault = null, depth = 0;
  for (let i = 0; i < lines.length; i++) {
    const l = lines[i];
    if (/^\s*#\[cfg\(test\)\]/.test(l) && /^\s*(pub\s+)?mod\s+\w+\s*\{/.test(lines[i + 1] || '')) break;
    if (rel.endsWith('.rs')) {
      const m = RS_CONST.exec(l);
      if (m) rows.push({ file: rel, line: i + 1, name: m[1], ty: m[2], value: m[3].trim(), src: 'const' });
      const d = DEFAULT_IMPL.exec(l);
      if (d) { inDefault = d[1]; depth = 0; }
      if (inDefault) {
        depth += (l.match(/\{/g) || []).length - (l.match(/\}/g) || []).length;
        const fm = FIELD_LIT.exec(l);
        if (fm) rows.push({ file: rel, line: i + 1, name: `${inDefault}.${fm[1]}`, ty: 'field', value: fm[2], src: 'default' });
        else if (/Self\s*\{.*\}/.test(l)) for (const x of l.matchAll(INLINE_FIELD))
          rows.push({ file: rel, line: i + 1, name: `${inDefault}.${x[1]}`, ty: 'field', value: x[2], src: 'default' });
        if (depth <= 0 && /\}/.test(l)) inDefault = null;
      }
    } else {
      const m = WESL_CONST.exec(l);
      if (m && !/^(mat\w*|array)</.test(m[3])) rows.push({ file: rel, line: i + 1, name: m[1], ty: m[2] || 'wesl', value: m[3].trim(), src: 'wesl' });
    }
  }
}
const CONTAINER = /^(Vec|BTreeMap|HashMap|HashSet|BTreeSet|String|Rng|SmallVec|Entity)::/;
for (let i = rows.length - 1; i >= 0; i--) if (CONTAINER.test(rows[i].value)) rows.splice(i, 1);
for (const r of rows) { [r.cls, r.why] = classify(r); r.sys = system(r.file); }
// A file with 8+ unsure plain-integer consts is an id/enum table (record codes, section ids): stays const.
const intUnsure = {};
for (const r of rows) if (r.cls === '?' && r.src === 'const' && /^-?\d[\d_]*$/.test(r.value)) (intUnsure[r.file] ??= []).push(r);
for (const xs of Object.values(intUnsure)) if (xs.length >= 8) for (const r of xs) [r.cls, r.why] = ['C', 'id table'];

if (process.argv.includes('--json')) { console.log(JSON.stringify(rows)); process.exit(0); }
const count = (xs, c) => xs.filter(r => r.cls === c).length;
const systems = [...new Set(rows.map(r => r.sys))].sort();
if (!process.argv.includes('--md')) {
  console.log(`rows ${rows.length} T ${count(rows, 'T')} C ${count(rows, 'C')} ? ${count(rows, '?')}`);
  for (const s of systems) { const xs = rows.filter(r => r.sys === s); console.log(s, xs.length, 'T', count(xs, 'T'), 'C', count(xs, 'C'), '?', count(xs, '?')); }
  process.exit(0);
}
const esc = (v) => (v.length > 40 ? v.slice(0, 37) + '...' : v).replace(/\|/g, '\\|').replace(/`/g, "'");
const out = [];
out.push('| System | Candidates | T | C | ? |', '|---|---:|---:|---:|---:|');
for (const s of systems) { const xs = rows.filter(r => r.sys === s); out.push(`| ${s} | ${xs.length} | ${count(xs, 'T')} | ${count(xs, 'C')} | ${count(xs, '?')} |`); }
out.push(`| **total** | **${rows.length}** | **${count(rows, 'T')}** | **${count(rows, 'C')}** | **${count(rows, '?')}** |`, '');
for (const s of systems) {
  out.push(`## ${s}`, '');
  const crates = [...new Set(rows.filter(r => r.sys === s).map(r => r.file.split('/').slice(0, 3).join('/')))];
  for (const c of crates) {
    const xs = rows.filter(r => r.file.startsWith(c + '/'));
    const keep = xs.filter(r => r.cls !== 'C'), consts = xs.filter(r => r.cls === 'C');
    out.push(`### ${c.split('/')[2]} (${xs.length}: T ${count(xs, 'T')}, C ${consts.length}, ? ${count(xs, '?')})`, '');
    if (keep.length) {
      out.push('| Where | Name | Value | Cls | Note |', '|---|---|---|---|---|');
      for (const r of keep) out.push(`| ${r.file.slice(c.length + 1)}:${r.line} | ${r.name} | \`${esc(r.value)}\` | ${r.cls} | ${r.why} |`);
      out.push('');
    }
    if (consts.length) out.push(`Stays const: ${consts.map(r => `${r.name} (${r.why})`).join(', ')}.`, '');
  }
}
console.log(out.join('\n'));
```
