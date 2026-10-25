# Scripting: history

Moved out of [scripting.md](../scripting.md) on 2026-10-26 (doc diet S12a), word for word. Headings are unchanged. scripting.md and the code win where they differ.

## Research findings (checked 2026-10-13)

### R1. Runtime candidates (crates.io API, 2026-10-13)

| Option | Version | Bevy / license | Verdict |
|---|---|---|---|
| **`mlua` + Luau** | 0.12.2 (2026-10-03); Luau 0.740 via `luau0-src` 0.22.0 (2026-09-27); MSRV 1.88 | Bevy-free, MIT | **chosen** |
| `mlua` + LuaJIT | same crate | MIT | fast interpreter + JIT, but no sandbox mode or back-edge interrupt, `ffi` must be stripped by hand, no type checker, slow upstream releases |
| `mlua` + Lua 5.4/5.5 | same crate | MIT | no type checker, sandbox by hand, no interrupt at back-edges (only count hooks, slower) |
| `bevy_mod_scripting` | 0.22.0 (2026-10-11), **on Bevy 0.20**, uses `mlua ^0.11` | MIT/Apache | not adopted: reflection-first (any `Reflect` type is scriptable), the opposite of §6.2's curated API; README says "work in progress, expect significant API changes"; no sandbox or limit story; one more layer between us and mlua |
| `bevy_scriptum` | 0.11.0, Bevy 0.18, `mlua ^0.9.8` | MIT/Apache | stale (two Bevy versions behind) |
| `piccolo` (pure-Rust Lua) | 0.3.3 (2024-06) | MIT | stackless and safe by design, but unmaintained for 2 years and slower |
| `rhai` | 1.26.1 | MIT/Apache | Rust-native and sandboxable, but a niche language with no modding ecosystem, slower than Luau |
| WASM (`wasmtime`) | 49.0.2 | Apache-2.0 + LLVM exception | strongest isolation and multi-language, but heavy (compile time, Cranelift), poor for small event handlers and UI glue; keep as §6.1's "possibly later" for compute-heavy mods |

**Why Luau.** Built for exactly this (Roblox runs untrusted user code at scale): `sandbox` mode
(read-only globals and libraries, per-thread environments), an `interrupt` callback for
watchdogs, a per-state memory limit, an interpreter faster than PUC Lua and close to LuaJIT's
interpreter, optional native code generation (x64 incl. Windows, arm64), gradual typing with a
type checker and `luau-lsp` for mod authors, Lua 5.1 syntax compatibility, and require-by-string
with aliases. mlua exposes all of it: `Lua::sandbox`, `set_interrupt`, `set_memory_limit`,
`used_memory`, `gc_set_mode`, `set_compiler` (optimization level, type info), `Compiler` for
ahead-of-time bytecode, `create_require_function` (custom `Require`), `scope`,
`set_app_data`, serde conversion (`serialize` feature).

### R2. mlua features

| Feature | Use | Why |
|---|---|---|
| `luau` | yes | the runtime |
| `serialize` (= `serde`) | **no** (dropped in L3) | mlua's serde bridge (`LuaSerdeExt`) is used by no module: state, payloads and interface arguments all go through our own walker (`StateValue`), because mlua's bridge follows Lua's hash order (no canonical bytes, so no determinism hash) and stops at the first bad value without a path |
| `send` | **yes** (L1: `sf_script` feature, default on) | makes `Lua` `Send + Sync` so states live in ordinary resources and mods can later run on the compute pool. Measured against `NonSend`: calls within noise (raw call 66 vs 60 ns, guarded 148 vs 144 ns, handler dispatch 206 vs 200 ns), bulk state I/O ~30 % slower (rare path) |
| `async` | **no** | Rust futures inside scripts aren't saveable; waits are named timers |
| `luau-jit` (native codegen) | **off** (L1: `sf_script` feature `jit`) | measured: call-heavy Luau loop 2.3× faster (30 → 13 ns/iter), a `math.sqrt` loop unchanged (70 → 66 ns), API calls unchanged, context start 2.4× slower (50 ctx: 15 → 36 ms, code generated at load), +32 KiB heap per state, +~80 s clean build. API-bound handlers don't benefit; revisit per mod (`--!native`) if a measured mod needs it |
| `macros`, `module`, `vendored` | no | proc-macro compile cost; we embed, not extend; Luau always builds from source |

Build cost: `luau0-src` compiles Luau's C++ with `cc` (MSVC on Windows: already required by
`ufbx` for FBX bakes). Only `sf_script` depends on mlua, so it builds once, in parallel, and is
untouched by game edits (§8b rules 1 and 5). **Measured (L1, -j 8, BelowNormal):** the
`mlua-sys` build script (Luau C++) takes 66 s of one job; the whole new dependency set
(mlua, mlua-sys, luau0-src, bstr, parking_lot, erased-serde, ...) cleans-builds in 82 s wall,
off the critical path of the game crates. `jit` adds the codegen C++ (~+80 s).

### Spike results (L1, 2026-10-13)

Ryzen 7 7435HS laptop, release (`bench` profile), machine otherwise idle (no other build or game
running; the bench build itself overlapped another session's build). Medians; the watchdog rows
are the best of 5 interleaved rounds because the clock speed drifts between runs.

| Measure | `send` | `NonSend` | `send` + `jit` |
|---|---|---|---|
| Rust → Luau, empty function, raw | 66 ns | 60 ns | 67 ns |
| Rust → Luau, guarded entry | 148 ns | 144 ns | 151 ns |
| Luau → Rust, getter | 38 ns | 39 ns | 34 ns |
| Luau → Rust, number → number | 48 ns | 46 ns | 48 ns |
| Empty context heap (after GC) | 394 KiB | 394 KiB | 426 KiB |
| Start 50 contexts × 20 handlers | 15 ms | 14 ms | 36 ms |
| Dispatch to 1000 handlers (50 × 20), per handler | 206 ns | 200 ns | 203 ns |
| … with a 3-field payload incl. 2 handles | 272 ns | 253 ns | 266 ns |
| Event nobody handles, 50 contexts | 3.6 µs | 3.1 µs | 3.6 µs |
| Call-heavy loop: no interrupt / empty callback / clock every safepoint / shipped watchdog | 30 / 51 / 154 / 55 ns | 27 / 57 / 139 / 57 ns | 13 / 38 / 134 / 41 ns |
| World guard, 1 API call per slot: `scope` / app data | 731 / 232 ns | 711 / 198 ns | 656 / 170 ns |
| World guard, 100 API calls per slot: `scope` / app data | 6.5 / 6.9 µs | 6.7 / 7.2 µs | 4.7 / 5.1 µs |
| Numeric loop (`math.sqrt`, mul, mod), per iteration | 70 ns | 66 ns | 66 ns |
| Snapshot / restore, 1.4 MiB storage (140k values) | 33 / 52 ms/MiB | 24 / 31 ms/MiB | 27 / 34 ms/MiB |
| Hot reload of that mod | 133 ms | 80 ms | 83 ms |

Assumptions checked:

- **Decision 1 (Luau via mlua, state per mod): confirmed.** Call costs are 2-4× inside the
  budget, 50 sandboxed states start in 15 ms and fit in ~20 MiB, isolation, limits, require,
  timers and reload all work through mlua's safe API (the workspace keeps
  `unsafe_code = "forbid"`). Nothing measured favours a shared state: dispatching to a context
  with no subscriber is ~70 ns (a Rust-side set lookup).
- **`send`: keep.** Same call costs as `NonSend` within noise; only bulk state I/O is ~30 %
  slower, which is a save/reload path.
- **World guard: app-data guard** (above).
- **`luau-jit`: off** (R2 row).
- **Wrong in the draft, corrected here:** baseline memory (256 → 512 KiB budget), save/restore
  (5 → 40 ms/MiB), the watchdog's cost model (mlua trampoline per safepoint, not a clock read),
  and the need for the watchdog latch and for lifting the memory limit during host-side
  conversion.

### As built (L2, 2026-10-13)

`sf_script_bevy` (deps: `bevy_app`, `bevy_ecs`, `bevy_time`, `sf_diag_bevy`, `sf_script`; no
render or input crate):

- **`Scripts`** resource: the `ScriptRuntime` (`pub runtime`) plus the slot cell, the declared
  slot resources, per-context totals for counters and the last 64 diagnostics (`recent()`,
  `stopped()`). `Scripts::new` registers `@sf/world`; `start`/`reload` keep their diagnostics
  for the next slot. `ScriptsPlugin` inserts an empty one unless the app inserted its own.
- **Slots** (`ScriptSlot` system sets, one exclusive system each): `Input` in `PreUpdate`
  (`ScriptFact`s for that slot, e.g. `ScriptFact::input_action`), `Update` in `Update`
  (`Activated` first, then `ScriptFact`s, then `ScriptRuntime::update(dt)`: queued script
  events, due timers, `update` handlers; `dt` from `Time` (virtual)), `Ui` in `PostUpdate`
  (`ScriptFact`s for that slot; U7 grows it). The app orders input/focus before `Input`,
  fact-emitting gameplay before `Update`, default actions after it. A slot with no contexts
  costs one resource check.
- **Engine facts**: `Activated { target, actor }` (handles; `PLAYER` = `Handle(0)` for the
  player) → event `"activated"` `{ target, actor }`; every activation gets an
  `ActivationOutcome { activation, handled }` after the slot, `handled` when any handler
  returned `"handled"`. Other facts are `ScriptFact { slot, name, payload }` (the app maps
  `CellLoaded` → `"cell_loaded"`). Engine names have no colon; mod events stay `<mod>:<name>`.
- **Handles and `@sf/world` (L2 subset)**: `ScriptPlacements` (handle → record, worldspace,
  f64 position, radius; plus the player) is the loaded-placement map, a slot resource; the
  content pack fills it from its R0 activation hooks (Bevyrrowind: a `PlacementSpawner` wrapper).
  `world.player()`, `is_loaded(h)`, `record(h)`, `worldspace(h)`, `position(h)` → `x, y, z`
  (doubles: a Luau `vector` is f32). Handle methods (`h:position()`) need the handle's userdata
  type to dispatch to registered modules: an `sf_script` hook for L3.
- **Diagnostics**: per slot, `take_diagnostics()` → `ScriptDiagnostics` message + `recent()`;
  counters `script/<slot>/ms` (slot wall time) and per context `script/ctx/<id>/ms`, `calls`,
  `errors`, `aborts` (per-slot deltas of `ContextStats`), gauge `script/ctx/<id>/mib`.
- **Not in L2** (moved on): entity scripts with state in the placement overlay and the
  "target's entity scripts first" half of the activation pipeline (L3 `world.attach`, W3 for
  legacy locals), spawn/despawn (L3), the dev Lua console (needs U1 input; `ScriptContext::eval`
  exists), `Died` (M7), hot reload wiring (A4 calls `Scripts::reload`).

### As built (L3, 2026-10-13): API v0.1

Modules (`sf_script` registers the first four, `sf_script_bevy::Scripts::new` adds `@sf/world`);
the types are in the generated `docs/api/sf.d.luau`:

| Module | v0.1 surface |
|---|---|
| `@sf/core` (also global `sf`) | `api_version`, `mod_id`, `version`, `log`, `random`, `storage()`, `settings()` (copy of `ContextConfig::settings`), `time()` (sim seconds), `game_time()` (hours, host-set with `ScriptRuntime::set_game_time`, nil without a clock), `on_load(fn)` |
| `@sf/events` | `on(name, fn)`, `emit(name, payload)` (unchanged) |
| `@sf/timers` | `register`, `after`, `every`, `cancel` (unchanged) |
| `@sf/interfaces` | `export(name, functions)` at start, `get(mod_id, name)` → proxy or nil |
| `@sf/world` | `player`, `is_loaded`, `record`, `worldspace`, `position`, `nearby(at, r)`, `spawn(record, at, yaw?)`, `despawn(h)`, `define_script`, `attach`, `detach`, `state`, `script_of`; handle methods `h:record()`, `h:position()`, `h:despawn()`, `h:attach(..)`, `h:state()`, ... |

- **`on_load` and world access (decision).** Entry points (a script's top level) run with the slot
  closed, so world reads there fail ("world access outside a script slot"): top level is for
  registration only (handlers, timers, entity scripts, exports), as in Factorio (`on_load` may
  not touch `game`) and OpenMW (scripts get the world in handlers). `core.on_load` handlers are
  **queued** by start, hot reload (`old_version` = the snapshot's) and restore, and run once at
  the start of the next `Update` slot (`ScriptRuntime::run_loads`, before activations), where the
  slot is open and world reads work. Rejected: opening a slot around `Scripts::start` (start runs
  at app build, before streaming has filled `ScriptPlacements`, so reads would see an empty
  world), and running `on_load` inside `reload` (A4 reloads from a watcher system that has no
  slot either). Headless users of `sf_script` get the same order from `ScriptRuntime::update`.
- **Handle methods**: `Handle`'s `__index` falls back to a per-context method table that API
  modules fill at install (`sf_script::handle_methods`), read-only after install; unknown method
  names raise. Modules declare their methods' types with `ApiModule::handle_types`.
- **Spawn/despawn**: `@sf/world` queues `WorldCommand::{Spawn, Despawn}` in the `ScriptWorld` slot
  resource; the slot runner flushes them as Bevy messages when the slot closes. Spawned handles are
  `FNV-1a-128("sf.script.spawn", context, n)` with the top bit set (deterministic per run; the
  counter persists with the overlay in M7) and are in `ScriptPlacements` at once (scripts read
  their own writes). The content pack applies the commands (Bevyrrowind: placement overlay + the
  `LiveSpawn` scaffold in `scripts.rs`, see its removal path).
- **Activation pipeline**: `ScriptRuntime::activate(target, payload)` offers `"activated"` to the
  target's entity scripts (contexts in load order), then to global subscribers; `handled` if any
  returned `"handled"`. Bevyrrowind's doors now open from `ActivationOutcome { handled: false }`
  of a door placement (`doors.rs`), so a script can lock a door.
- **Interfaces**: synchronous, Factorio `remote.call` style. The callee runs in its own state,
  under its own watchdog and call limit; arguments and results are copied as `StateValue` (no
  handles to functions or tables cross states); A → B → A cycles raise "re-entrant interface
  call"; exports are published only after a successful start or reload (a failed reload keeps
  the old ones). Not counted in the callee's stats or fault log yet (errors surface in the
  caller, who may `pcall`).
- **`.d.luau`**: `ScriptRuntime::type_definitions` emits one definitions file: `declare class
  Handle` (every module's handle methods) and each module as a typed global `sf_<name>`
  (`@sf/world` → `sf_world`) with its `type` aliases (`LoadInfo`, `ActivatedPayload`,
  `Position`). Golden: `superfargoth/docs/api/sf.d.luau`, kept in sync by
  `sf_script_bevy` test `golden_type_definitions` (`SF_BLESS=1` regenerates). Not yet checked
  with `luau-analyze` (not installed here); mapping `require("@sf/world")` to `sf_world` for
  `luau-lsp` needs per-module stub files (follow-up with the mod-author docs).
- **Example mod**: `superfargoth/examples/mods/api_tour` (every module once; entity script in
  `lamp.luau`), run headless by `sf_script_bevy` test `example_mod_api_tour_runs_headless`.
- mlua `serialize` dropped (no module used it).
- **Not in L3** (moved on): `@sf/records`, `@sf/input`, `@sf/anim`, `@sf/physics`; entity scripts
  from a record's `script` field; `teleport`/`set_enabled`; timers on the game clock
  (`at_game_time`); profile settings into `ContextConfig::settings` (U1/U6 settings registry).

## Industry comparison

2026-10-13. "Unverified" marks facts from memory not re-read this session. OpenMW is a black-box
API-shape reference only (GPL; no code read or copied).

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Language / VM | Roblox: Luau (sandbox, typed, native codegen); Factorio: modified Lua 5.2 (unverified version); WoW: Lua 5.1; OpenMW: LuaJIT (unverified); Skyrim/Fallout 4: Papyrus (compiled, own VM); Bevy crates: bevy_mod_scripting (Lua family + Rhai) | Luau via mlua | **match** (Roblox) |
| Isolation unit | Factorio: one Lua state per mod, `remote.call` copies values (unverified detail); OpenMW: sandboxed environment per script (docs); Roblox: shared VM per actor, per-script environments | state per mod; shared context only for legacy translated content | **match** (Factorio) |
| Script kinds | OpenMW: global, local (per object, may modify only its object), player, menu (docs); Papyrus: scripts attached to forms/references; Factorio: one control script per mod | global handlers per mod + entity scripts attached to placements | **match**. We don't restrict entity scripts to modifying their own object (OpenMW's rule exists for its multiplayer plan); writes are sequential in one exclusive slot, so it isn't needed |
| Event delivery | OpenMW: events delivered next frame, handlers in reverse load order, `false` stops propagation (docs); Factorio: synchronous dispatch per event, mod order (unverified) | script-to-script events next slot, load order; engine facts dispatched in slot; `"handled"` stops default activation | **match** |
| Waiting / timers | OpenMW: saved timers store only a registered callback name (docs); Papyrus: latent functions (`Utility.Wait`) suspend stacks that are saved; Factorio: `on_nth_tick` | named timers with plain-data payloads | **match** (OpenMW); deliberately unlike Papyrus |
| Persistence | Factorio: `storage` table per mod, plain data (renamed from `global` in 2.0, unverified date); OpenMW: `onSave`/`onLoad` plain serializable data, `storage` sections (docs); Papyrus: whole VM state in saves (orphan-script bloat) | per-mod `storage` + entity state + timers, plain data, migration hook | **match** |
| CPU limits | Roblox: script timeout (exhausted allowed execution time, unverified default 10 s); WoW: "script ran too long" in protected contexts, per-addon CPU profiling (unverified); Papyrus: per-frame VM time budget (`fUpdateBudgetMS`, unverified default 1.2 ms) and time slicing; Factorio: none (determinism) | per-call hard limit + per-frame soft budget + per-mod stats | **match** (Roblox + WoW profiling). No time slicing (Papyrus) because it makes script timing nondeterministic |
| Memory limits | Roblox: per-place memory categories; WoW: per-addon memory reporting; OpenMW: setting for Lua memory (unverified) | per-context allocator limit | **match** |
| Hot reload | OpenMW `reloadlua` (onSave → restart → onLoad, loose files only; docs); Roblox Studio: restart play; Factorio: `/c` console + `on_load`; Papyrus: none at runtime | per-mod automatic reload through the save path, old state kept on failure | **match**, better granularity |
| API shape | Factorio: curated, documented, versioned per game version; Roblox: curated instance API with capability security; bevy_mod_scripting: generic reflection over any `Reflect` type | curated, semver'd, type definitions generated | **match** (Factorio/Roblox); **differ** from bevy_mod_scripting on purpose (§6.2) |
| Tooling for authors | Roblox: Luau type checker, Studio; Factorio: generated API docs + community `.d.lua` (sumneko) | generated `.d.luau` for luau-lsp, generated docs | **match** |
| Determinism | Factorio: required for lockstep multiplayer; Roblox: not deterministic | deterministic by construction (order, seeded RNG, no clock) | **match** (Factorio), not needed for networking |
| Parallel scripts | Roblox Parallel Luau (actors, `task.desynchronize`) | sequential exclusive slots; states are `Send` so a parallel read-only phase can be added | **gap (P3)**: only if the budget fails with many mods |

**Sources** (checked 2026-10-13): crates.io API for every version in R1/R2; mlua 0.12.2 docs
(`docs.rs/mlua/0.12.2/mlua/struct.Lua.html`: `sandbox`, `set_interrupt`, `set_memory_limit`,
`create_require_function`, `set_compiler`, `scope`); bevy_mod_scripting README
(github.com/makspll/bevy_mod_scripting: languages incl. Luau, Bevy 0.20 row for 0.22, "work in
progress"); OpenMW Lua overview (openmw.readthedocs.io/en/latest/reference/lua-scripting/
overview.html: script kinds, next-frame events, onSave/onLoad, named timer callbacks,
`reloadlua`, `.omwscripts` flags). Luau: luau.org (sandboxing, performance, typecheck, require
RFC; not re-read this session).

