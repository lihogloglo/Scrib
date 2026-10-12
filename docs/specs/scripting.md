# Scripting (Luau)

Status: draft (2026-10-13, m5-spec session, docs only). Built in M5.
Crates (proposed): `sf_script` (Bevy-free: runtime, sandbox, API registry, state, timers, events),
`sf_script_bevy` (plugin: schedule slots, world access, entity handles, hot reload, diagnostics),
API modules next to the systems they expose (`sf_ui_lua` per ui.md; small modules may live in
`sf_script_bevy` behind features). Mod pipeline: [mods.md](mods.md). First adapter:
`bevyrrowind/docs/specs/scripting-adapter.md` (legacy script translation).

## Goal

Mods add mechanics, UI and content logic in **Luau**, through a **curated, versioned API**, with
the safety players expect from Roblox or Factorio: a broken or hostile mod can't touch the
file system, can't freeze the game, can't take other mods down with it, and its state survives
saves and hot reloads. Adapters translate legacy script languages into the same runtime
(ARCHITECTURE §6.3), so one VM runs everything.

Requirements, in priority order:

1. **Sandboxed**: no `io`, `os`, `debug`, native loading, `loadstring`/`load` of text or
   bytecode from mods, or network. Read-only standard libraries.
2. **Bounded**: per-mod memory limit, per-call CPU limit, per-frame CPU budget, all reported.
3. **Isolated**: an error stops one handler, a repeat offender suspends one mod, never the game.
4. **Deterministic**: same build, same inputs, same results (scheduling order, seeded RNG, no
   wall clock), so bugs reproduce and saves replay.
5. **Persistent**: script state is plain data, serialized into saves (`sf_save`, M7) and
   carried across hot reloads with the same code path.
6. **Fast**: ≤ 1 ms of script time per frame for a typical load (budget below).
7. **Hot reload** of one mod's scripts in dev builds, keeping its state.

Non-goals: native code mods (§6.1: no stable ABI, no sandbox); a raw `Reflect` world API for
mods (allowed only behind a dev flag, §6.2); multiplayer lockstep (determinism is for bugs and
replays, not networking); a visual scripting editor.

## Canonical technique

**Embedded, sandboxed Lua-family VM with a curated host API and event-driven handlers**, the
model of Roblox (Luau), Factorio (Lua 5.2 fork), WoW (Lua 5.1) and OpenMW (LuaJIT). Specifics we
follow:

- **One VM state per mod** (Factorio: every mod has its own Lua state; cross-mod calls go through
  `remote` interfaces that copy values). Gives per-mod memory limits (a Luau allocator limit is
  per state), per-mod CPU accounting, per-mod hot reload (drop and rebuild one state) and a clean
  path to running mods in parallel later.
- **Handlers, not threads.** Scripts register callbacks for engine events, named timers and a
  per-frame update; the host calls them at fixed schedule points (Factorio `script.on_event`,
  OpenMW engine handlers, WoW frame events). No free-running coroutines across frames: a
  suspended coroutine can't be saved (Papyrus saves suspended stacks; orphaned stacks are the
  classic Skyrim save-bloat bug), so waits are **named timers with plain-data payloads**
  (OpenMW `registerTimerCallback`). Coroutines inside one handler call are fine.
- **State is plain data** declared by the script (Factorio `storage`, OpenMW `onSave`/`onLoad`):
  no functions, metatables, or cycles. Saves and hot reload use the same serialization.
- **Capability sandbox** (Roblox, Luau `sandbox` mode): globals and libraries read-only, every
  host capability is an explicit API function, unknown globals are errors.
- **Interrupt-based watchdog** (Roblox "script timeout", Luau `interrupt` callback): the VM
  checks a deadline at calls and loop back-edges.

Where we deviate: Roblox shares one VM per actor among many scripts; OpenMW runs every script of
a kind in shared Lua state with per-script environments. We pick the Factorio split (state per
mod) for the reasons above, with one exception: **adapters may declare a shared context** for
translated legacy content, because legacy scripts address each other by ID across plugins
(scripting-adapter.md).

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
| `serialize` (= `serde`) | yes | script state ↔ serde (saves, hot reload, event payloads) without hand-written converters |
| `send` | yes, confirm in spike L1 | makes `Lua` `Send + Sync` so states live in ordinary resources and mods can later run on the compute pool; costs a lock per entry. Spike measures it against `NonSend` storage |
| `async` | **no** | Rust futures inside scripts aren't saveable; waits are named timers |
| `luau-jit` (native codegen) | off at M5; A/B in L1 | Luau codegen pays off on numeric loops, little on API-bound handlers; adds code memory. Turn on per mod (`--!native`, Roblox style) if a measured mod needs it |
| `macros`, `module`, `vendored` | no | proc-macro compile cost; we embed, not extend; Luau always builds from source |

Build cost: `luau0-src` compiles Luau's C++ with `cc` (MSVC on Windows: already required by
`ufbx` for FBX bakes). Only `sf_script` depends on mlua, so it builds once, in parallel, and is
untouched by game edits (§8b rules 1 and 5). L1 records the clean-build delta.

## Design

### Contexts

A **context** is one Luau state (`mlua::Lua`) plus its bookkeeping:

```rust
pub struct ScriptContext {
    pub id: ContextId,               // usually the mod id
    pub mods: SmallVec<[ModId; 1]>,  // one mod, or an adapter's shared legacy context
    pub api: ApiVersion,             // resolved from the manifest's `api` requirement
    lua: mlua::Lua,
    limits: Limits,                  // memory bytes, per-call ms, per-frame soft ms
    stats: ContextStats,             // time per slot, calls, errors, memory high-water
    status: Status,                  // Running | Suspended { reason } | Failed { error }
}
```

Creation: a Luau state with the safe standard libraries only, minus what the sandbox forbids
(`loadstring`, `getfenv`/`setfenv`, `newproxy`, `string.dump`, `collectgarbage` beyond
`count` (Luau sandbox default), all of `os`, all of `debug` except `traceback`), install API modules for the context's `api` version,
replace `print` with the logger, replace `math.random` with a seeded per-context PCG (state
saved), install `require`, then `lua.sandbox(true)` (globals read-only; each loaded chunk gets
its own environment). Memory limit from the manifest-independent player setting (default
64 MiB per mod context, 256 MiB for shared legacy contexts; tune in L1).

Contexts are created in load order and always iterated in load order, so every observable
ordering is deterministic.

### Modules and `require`

Luau require-by-string (RFC "require by string with aliases"), implemented with mlua's
`Require` over `sf_vfs`:

- `require("./util")`, `require("../lib/x")`: files of the same mod (`scripts/<mod_id>/...`).
- `require("@sf/ui")`, `require("@sf/world")`, ...: framework API modules.
- `require("@<adapter>/...")`: content-pack API modules registered by the adapter
  (Bevyrrowind: `@mw/...`), versioned with the content pack.
- `require("@<mod_id>/...")`: a **library file** of a declared dependency, loaded into the
  requiring context (code sharing, not state sharing).

Only source files are loadable from mods. **Mod-shipped bytecode is never loaded** (Luau does
not verify bytecode; malformed bytecode can corrupt the VM). Bytecode the engine compiled
itself (bake outputs, scripting-adapter.md) may be cached and loaded.

An entry point is a file listed in the manifest's `content.scripts`; it runs once at context
start and registers handlers. Globals written by a script live in its own environment
(sandbox), so mods can't monkey-patch each other or the API.

### Execution model and threading with Bevy's ECS

Scripts run in **exclusive systems at fixed schedule slots**, with a synchronous API:

| Slot (`ScriptSlot`) | Schedule position | Runs |
|---|---|---|
| `Input` | `PreUpdate`, after input and UI focus | input-action and UI events (button pressed, screen opened); synchronous so UI never lags a frame (OpenMW #9233 lesson, ui.md) |
| `Update` | `Update`, after gameplay systems that emit facts (activation, death, cell load) and before physics write-back | engine events → due timers → `on_update(dt)` per context → entity scripts' `tick` |
| `Ui` | `UiSystems::ApplyScriptCommands` (ui.md) | view-model writes, slot contributions, `ui.*` handle setters |

Inside a slot the system has `&mut World`. A Rust API function reaches it through a
**world guard** valid only while the slot runs: the slot stores an access token in the
context's app data before calling into Luau and clears it after, so a function captured and
called outside a slot raises a Lua error instead of touching freed state. Mechanism (mlua
`scope` vs a guarded pointer in app data) is decided in spike L1 by cost; both keep the API
safe Rust at the boundary.

**Writes are immediate** for component state the API owns (enable/disable, teleport, inventory,
variables), so scripts read their own writes (Factorio semantics). **Structural changes**
(spawn, despawn) return a handle immediately and flush through `Commands` at the end of the
handler. **Events between scripts are delivered in the next `Update` slot** (OpenMW: one frame
delay), in load order, so a handler never re-enters another mod mid-call and the order is
deterministic. The cost is that exclusive slots don't run in parallel with other systems; the
per-frame budget below keeps that bounded. A read-only parallel phase (Roblox Parallel Luau:
`task.desynchronize`) is possible later because states are `Send`; not in M5.

**Engine → script facts** are Bevy messages (`Activated { actor, target }`, `Died`,
`CellLoaded`, `InputAction`, ...) that `sf_script_bevy` drains in the slot and dispatches to
subscribed contexts. **Script → engine** requests that belong to another system (play a
montage, apply an impulse, force weather) are messages that system consumes, so `sf_script`
never depends on gameplay crates; the API module of each system lives with that system.

**Activation pipeline** (generic): `Activated` is offered to the target's entity scripts and
then to global subscribers in load order; any handler may return `"handled"` to suppress the
default action (open door, pick up item); otherwise the default runs after the slot. Adapters
can mark a script as handling activation statically (legacy semantics, scripting-adapter.md).

### Entity handles and entity scripts

- A script never holds a Bevy `Entity`. **Handles** are userdata wrapping a `PlacementId`
  (`sf_world`: 128-bit, stable across load orders and saves) or a runtime-spawned id from the
  placement overlay. `handle:is_loaded()` says whether it has a live entity; resolving goes
  through a `PlacementId → Entity` map maintained by streaming. Calls needing a loaded entity
  on an unloaded one return `nil, "unloaded"` (or raise, per function doc); data-only calls
  (record, stored state, position from the overlay) work while unloaded.
- **Entity scripts**: records may attach a script to objects (`script` field on a base record,
  or `world.attach(handle, "script_id")`). Each instance gets a plain-data state table, stored
  in the placement overlay's per-placement state under `"sf.script/<context>"` (saved with the
  overlay, survives cell unloads). The script module exports handlers (`tick`, `on_activate`,
  `on_load`, ...); `tick` runs every `Update` while the entity is loaded and active, in
  `PlacementId` order.

### State, timers and saves

Per context, everything that persists:

- `storage`: a plain-data table per mod (`local s = sf.storage()`), like Factorio's `storage`.
- entity script states (above);
- **named timers**: `timers.after(seconds, "name", payload)`, `timers.every(...)`,
  `timers.at_game_time(...)`; the callback is looked up by name in `timers.register("name", fn)`
  at context start. Simulation-time and game-time clocks (game time comes from `sf_sky`'s
  clock). Unregistered names are errors at schedule time, not at fire time;
- RNG state; pending events.

Serialization: mlua `serialize` → `serde` → postcard, with a per-context header (mod id,
version, api version, schema hash of registered timers). Rules checked on write: no functions,
userdata other than handles (handles serialize as `PlacementId`), metatables, cycles or shared
references; violations are errors naming the path (`storage.quests[3].cb is a function`).
`sf_save` (M7) stores the blobs; M5 uses them for hot reload and tests.

Migration: `on_load(old_state, old_version)` lets a mod upgrade its own data when its version
changed (Factorio `on_configuration_changed`).

### Curated API and versioning

The API is a **registry of modules**. Each binding is declared once with its Luau type
signature and the API version that introduced it:

```rust
pub trait ApiModule: Send + Sync + 'static {
    fn name(&self) -> &'static str;                 // "@sf/world", "@mw/script"
    fn version(&self) -> semver::Version;           // the module's own version
    fn install(&self, lua: &mlua::Lua, cx: &InstallCx) -> mlua::Result<mlua::Table>;
    fn types(&self) -> &'static str;                // Luau declarations, `.d.luau`
}
pub struct ScriptRuntime { /* contexts, registry, timers, stats */ }
impl ScriptRuntime {
    pub fn register_module(&mut self, m: Box<dyn ApiModule>);
    pub fn start(&mut self, mods: &LoadOrder, vfs: &Vfs) -> Vec<ScriptDiagnostic>;
    pub fn reload(&mut self, ctx: ContextId, vfs: &Vfs) -> Result<(), ScriptError>; // save → rebuild → load
}
```

- **Framework API** (`@sf/*`) is one semver line (`api` in the manifest, already in
  `sf_mods::ModManifest`). Minor versions only add; a removal or semantic change is a major
  bump with the old behaviour kept for one major where feasible. Bindings newer than the mod's
  requirement are still visible (additive), deprecated ones log once per mod. Until the first
  public mod release it stays `0.x` (no promises); target `1.0` once M7's RPG modules exist.
- **Content-pack APIs** (`@mw/*`) version with the content pack; a mod expresses the need with an
  ordinary dependency on the content-pack mod (`dependencies = { bevyrrowind = "^1.0" }`).
- **Type definitions**: the registry emits `sf.d.luau` (and each adapter its own), so authors
  get completion and checking in `luau-lsp`; docs are generated from the same declarations.

M5 module set (each small; names provisional):

| Module | Contents | Depends on |
|---|---|---|
| `@sf/core` | `api_version`, `mod_id`, `log`, `storage()`, `random`, `time` (sim and game clock), `settings` (read this mod's settings) | — |
| `@sf/events` | `on(name, fn)`, `emit(name, payload)`, engine event names; mod events namespaced `<mod_id>:<name>` | — |
| `@sf/timers` | `register`, `after`, `every`, `cancel` | — |
| `@sf/interfaces` | `export(name, table_of_functions)`, `get(mod_id, name)`; arguments and results are copied plain data (cross-state) | — |
| `@sf/world` | handles: `get(placement or record query)`, `nearby(pos, radius, filter)`, `h:record()`, `h:position()`, `h:set_enabled()`, `h:teleport()`, `spawn(record, at)`, `h:despawn()`, `attach(h, script)`, `h:state()`; `player()` | `sf_world`, streaming |
| `@sf/records` | read-only views of record kinds that adapters expose (`records.get(kind, id)`, lazily converted, cached per frame) | `sf_records` |
| `@sf/input` | action state, mod-defined actions with default bindings (rebindable in settings) | input layer |
| `@sf/ui` | ui.md "Luau surface" (slots, columns, screens, settings pages, message boxes, notifications) | `sf_ui_lua` |
| `@sf/anim` | play a montage in a slot (animation.md "Slots and montages") | `sf_anim_bevy` |
| `@sf/physics` | impulse, raycast (physics.md) | `sf_physics_bevy` |

Later: `@sf/rpg` (M7: inventory rules, stats, effects, dialogue, quests), `@sf/weather` (M6),
`@sf/audio` (with the audio system), `@sf/save` hooks (M7).

### Hot reload

Dev builds (and a player toggle in a "mod author" setting), driven by the VFS watcher
(mods.md "Hot reload"):

1. A changed `.luau`/`.lua` file maps to its owning mod → its context.
2. `on_save` the context (same serializer as saves), keep handles and timers as data.
3. Build a new state, run entry points, `on_load(state)`; on any error **keep the old state
   running** and report the error (Erlang/Elixir-style "upgrade or keep the old code").
4. Shared legacy contexts reload the changed script module only (its instances keep their
   variable tables).

OpenMW's `reloadlua` (`onSave` → restart → `onLoad`) and Factorio's `on_load` are the same
shape; ours is per mod and automatic.

### Error isolation and limits

| Fault | Response |
|---|---|
| Lua error in a handler | caught (mlua `Result`), logged with mod id, handler and traceback; other handlers continue; counted in `sf_diag` |
| Handler exceeds the hard per-call limit (interrupt deadline; default 50 ms release, 500 ms dev) | aborted with a timeout error; same as above |
| 3 aborts or 30 errors within 60 s | context **suspended** for the session (handlers skipped, state kept); player notified (dev: on-screen; release: mod manager badge) |
| Memory limit hit | allocation fails → Lua error; the context is suspended (its state may be inconsistent) |
| Rust panic in an API function | API functions must not panic (clippy `unwrap_used`/`expect_used`/`panic` denied in API crates); each slot call is wrapped in `catch_unwind` as a last resort → context failed |
| Per-frame soft budget exceeded (all contexts > 1 ms, or one > 0.5 ms averaged over 1 s) | warning + `sf_diag` counter; no throttling in M5 (Papyrus-style time slicing changes semantics; revisit if needed) |

The interrupt callback reads a deadline set per call, so its cost is one clock read every N
interrupts (Luau calls it at function entries and loop back-edges; L1 measures the overhead).

### Determinism

Same build and same inputs give the same script results:

- Contexts in load order; entity scripts in `PlacementId` order; events in emission order.
- No wall clock: `os` is absent; `sf.time` gives simulation and game time from the engine.
- `math.random` and `sf.random` are per-context seeded PCG streams, saved.
- API functions that return collections return arrays in a defined order (never hash order).
- Handler timeouts are wall-clock based and therefore not deterministic, but they only occur on
  failure paths.

### Performance budget

Reference laptop (Ryzen 7 7435HS), release build, calibrate in L1/L2 with `sf_diag` spans:

| Item | Budget |
|---|---|
| All script slots per frame, typical load (vanilla content-pack scripts in a city + 10 native mods) | ≤ 1.0 ms |
| HUD Lua (ui.md) | ≤ 0.2 ms |
| Rust → Luau handler call | ≤ 0.3 µs median |
| Luau → Rust API call (simple getter) | ≤ 0.15 µs median |
| Empty context baseline memory | ≤ 256 KiB (measured in L1) |
| Context start, 50 mods (no scripts heavier than 1 ms) | ≤ 200 ms at game start |
| Hot reload of one mod | ≤ 100 ms |
| Save/restore of all script state | ≤ 5 ms per MiB |

Levers if a budget fails: native codegen per mod (`--!native`), handler throttling for
`on_update` (`every_n_frames`), API batching (`world.nearby` returning arrays rather than
iterators), caching record views per frame.

## Verification

- **Unit (Bevy-free, `sf_script`)**: sandbox escape list (`io`, `os`, `debug.getinfo`,
  `loadstring`, `getfenv`, `string.dump`, writing `_G`, writing library tables, requiring a
  non-dependency, loading bytecode) all fail; memory limit fires; interrupt aborts `while true
  do end` within the limit; an error in one context leaves others running; suspension after the
  thresholds; state round trip (all plain types, handles, nested tables) and rejection of
  functions/cycles with paths; timer save/restore fires once at the right time; determinism: two
  runs of a scripted scenario produce identical state hashes; API gating by version; generated
  `.d.luau` golden test.
- **Integration (headless `sf_script_bevy`)**: an app with a test mod that listens for
  `Activated`, spawns a placement, sets a timer, edits its storage; hot reload of the mod keeps
  storage and timers; a suspended mod doesn't block others.
- **Bench** (`cargo bench -p sf_script` and an `sf_diag` scenario): the budget table; Balmora
  with the content pack's legacy scripts + 10 synthetic mods.
- **Owner scenes**: ROADMAP "M5" MS-2 (Lua mod + hot reload) and MS-4/MS-5 (legacy scripts).

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

## Open questions

1. `send` feature vs `NonSend` storage, and the world-guard mechanism (`scope` vs guarded
   pointer): decided by numbers in L1.
2. Native codegen default: off; per-mod opt-in (`--!native`) if a measured mod benefits.
3. Whether entity `tick` should support a declared rate (`tick_hz`) to cut cost for mods that
   poll; legacy scripts must tick every frame (scripting-adapter.md).
4. Player-facing controls for limits (per-mod memory override in the mod manager): M5 has
   profile-level settings only.
5. Shared library mods loaded into many contexts duplicate their code per state; measure with a
   large library before considering a shared read-only module cache.
