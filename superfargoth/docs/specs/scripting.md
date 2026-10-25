# Scripting (Luau)

Status: draft (2026-10-13, m5-spec session, docs only). Built in M5. L1 (`sf_script` core + spike)
done 2026-10-13 (m5-l1): numbers in "Spike results (L1)"; decision 1 confirmed. L2 (`sf_script_bevy`)
done 2026-10-13 (m5-l2): "As built (L2)". L3 (API v0.1: core, events, timers, interfaces, world with
spawn/despawn and entity scripts, `on_load`, generated `.d.luau`) done 2026-10-13 (m5-l3): "As built (L3)";
verified in the game app 2026-10-18 (spawn/despawn, entity script state and tick, `handled` blocks the
default action, unhandled door activation still travels). `@sf/records`, `@sf/input`, `@sf/anim`,
`@sf/physics` and MS-2: parked (owner 2026-10-18, Lua parked after L3).
Crates (proposed): `sf_script` (Bevy-free: runtime, sandbox, API registry, state, timers, events),
`sf_script_bevy` (plugin: schedule slots, world access, entity handles, hot reload, diagnostics),
API modules next to the systems they expose (`sf_ui_lua` per ui.md; small modules may live in
`sf_script_bevy` behind features). Mod pipeline: [mods.md](mods.md). First adapter:
`bevyrrowind/docs/specs/scripting-adapter.md` (legacy script translation).

<!-- agent-summary -->
> **Agent summary (2026-10-26).** Status: PARKED after L3 (owner 2026-10-18). Built: L1 `sf_script` core (Luau, spike), L2 `sf_script_bevy`, L3 API v0.1 (core, events, timers, interfaces, world spawn/despawn, entity scripts, `on_load`, `.d.luau`), verified in game 2026-10-18. Parked: `@sf/records`, `@sf/input`, `@sf/anim`, `@sf/physics`, MS-2.
> Crates / key types: `sf_script`, `sf_script_bevy`, `ScriptContext`, `ScriptSlot`. Decisions: one sandboxed VM state per mod, adapters may share a context ("Canonical technique"); handlers and named timers, no saved coroutines; state is plain data; scripts run in exclusive systems at fixed slots, synchronous API ("Execution model"); watchdog interrupt ("Error isolation and limits").
> Read next: "Contexts" and "Curated API and versioning" for API work; "Hot reload"; "Open questions" (all open items are minor).
<!-- /agent-summary -->

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

Moved to [history](archive/scripting-history.md#research-findings-checked-2026-10-13).

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
saved), install `require`, then `lua.sandbox(true)` (globals and library tables read-only;
script globals go to a writable proxy environment of the state). Memory limit from the
manifest-independent player setting (default 64 MiB per mod context, 256 MiB for shared legacy
contexts; L1 found no reason to change them).

L1 findings (`sf_script::ScriptContext::build`):

- mlua installs `loadstring` and a **filesystem `require`** (`FsRequirer`) on every Luau state;
  both are removed before sandboxing, with `getfenv`, `setfenv`, `newproxy` and `gcinfo`.
  `os` is never opened; `debug` is reduced to `traceback`. The escape list is a unit test.
- `sandbox(true)` gives the *state* one writable environment, not one per chunk: all files of a
  mod share it (fine, state per mod). In a shared legacy context, translated scripts share it too,
  so the adapter's generated code must keep its variables in module tables, never globals
  (scripting-adapter.md).
- API module tables are installed into a registry table and made read-only
  (`Table::set_readonly`), so `require("@sf/core").x = ...` fails.

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

L1 implementation (`ModRequirer`, mlua `Require` over a `ScriptSource` trait implemented by
`sf_vfs::Vfs`): `..` never leaves `scripts/`, and the segment below it must be one of the
context's mods or declared dependencies (so `../other/x` fails like `@other/x`); `.luau` then
`.lua`, and `init.luau` for directories, with both present being an "ambiguous" error; no
`.luaurc` configuration files from mods; chunks load in `ChunkMode::Text`, so a bytecode file is
a syntax error, never executed (tested). The dev console's chunks (`=console:<id>`) may require
by alias only.

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
called outside a slot raises a Lua error instead of touching freed state. **L1 decision: an
app-data guard, not `scope`.** Measured per slot and context: `scope` costs ~0.5 µs per scoped
function it creates (a slot with one API call: 730 ns; an API of 20 functions would cost ~10 µs
per context per slot, ~1.5 ms a frame for 50 mods × 3 slots), while the guard costs ~0.23 µs per
slot; per API call both cost ~65 ns. L2 picks how the slot's access reaches the guard (data the
API needs moved in for the slot, or one scoped userdata per slot as the fallback); API functions
look it up in app data and fail with "outside a script slot" when it's empty.

**L2 decision: slot resources moved into one shared cell** (`sf_script_bevy::access`). API
modules declare the resources they read or write (`Scripts::register_slot_resource::<T>()`, any
`Resource + Default`); every context's app data holds an `Arc` to the runtime's one
`SlotAccess` cell from creation (the `@sf/world` module installs it, so restarts and reloads
get it too). Opening a slot moves each declared resource out of the `World` into the cell
(`mem::take` without change detection), closing moves it back; API functions borrow it with
`with_resource::<T>(lua, |t| ..)`, which fails with "world access outside a script slot" while
the cell is closed and with "re-entrant" instead of deadlocking. Cost: one move per declared
resource per slot, **independent of the number of contexts** (per-context guard installation
would be 0.23 µs × contexts × slots, 35 µs a frame at 50 mods), plus a mutex and a type lookup
per API call. Rejected: a `World` pointer (needs `unsafe`, forbidden), `scope` (L1 cost),
swapping the whole `World` out (breaks the running schedule's world id). Writes are immediate;
modules that write call `mark_changed::<T>` so change-detecting systems see them.

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
  later) or `world.attach(handle, "name")`, where `name` was defined at context start with
  `world.define_script(name, handlers)` (looked up by name like timers, so attachments are plain
  data). Each attachment gets a plain-data state table. **L3 change:** the states live in the
  owning context and travel in its snapshot (one blob per mod for saves and hot reload, survives
  cell unloads because the context holds it), not in the placement overlay as drafted: the
  overlay would split one mod's state across two stores with two save paths. Handlers:
  `activated(h, payload, state)` (before the mod's global handlers) and `tick(h, dt, state)`
  every `Update` while the placement is loaded, contexts in load order, handles in order.

### State, timers and saves

Per context, everything that persists:

- `storage`: a plain-data table per mod (`local s = sf.storage()`), like Factorio's `storage`.
- entity script states (above);
- **named timers**: `timers.after(seconds, "name", payload)`, `timers.every(...)`,
  `timers.at_game_time(...)`; the callback is looked up by name in `timers.register("name", fn)`
  at context start. Simulation-time and game-time clocks (game time comes from `sf_sky`'s
  clock). Unregistered names are errors at schedule time, not at fire time;
- RNG state; pending events.

Serialization: `StateValue` (our walker, L1) → `serde` → postcard, with a per-context header (mod id,
version, api version, schema hash of registered timers). Rules checked on write: no functions,
userdata other than handles (handles serialize as `PlacementId`), metatables, cycles or shared
references; violations are errors naming the path (`storage.quests[3].cb is a function`).
`sf_save` (M7) stores the blobs; M5 uses them for hot reload and tests.

Migration: `core.on_load(function(info) ... end)` lets a mod upgrade its own data when its version
changed (Factorio `on_configuration_changed`): `info = { reason = "start" | "reload" | "restore",
version, old_version? }`, with storage already restored, so the handler migrates `sf.storage()` in
place (L3; the draft's `on_load(old_state, old_version)` would have copied the state twice).

L1 implementation: `StateValue` (nil, bool, number, string bytes, vector, handle, table with
sorted keys; keys are strings, integral numbers or booleans) is walked from Lua by our own
converter, which reports the path of the first violation and sorts keys, so equal states give
equal bytes (the determinism test compares snapshot bytes). Numbers are Luau doubles (mlua reports
integral ones as `Integer`; both store as numbers). `ContextSnapshot` = header (context, mod
version, API version, FNV-1a of the registered timer names) + clock + RNG + storage + timers,
postcard-encoded. Restoring writes storage **into the existing table** (scripts that captured
`sf.storage()` stay valid) and drops, with a diagnostic, timers whose callback the current code
no longer registers.

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
3. Build a new state, restore storage/clock/RNG, run entry points (so start code sees its data),
   restore timers (their callbacks are registered by the entry points), `on_load(state)`; on any
   error **keep the old state running** and report the error (Erlang/Elixir-style "upgrade or
   keep the old code"). L1 builds steps 2-3 as `ScriptRuntime::reload` (tested: count kept, new
   code live, broken edit keeps the old code); `on_load` arrives with the `@sf/core` API (L3).
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

The interrupt callback reads a deadline set per **host entry** (L1: one deadline and one pair of
clock reads per context per event, timer batch or console call; each handler's error is still
caught and counted on its own; a timeout or memory fault ends the entry). The assumption that its
cost is "one clock read every N interrupts" was **wrong**: L1 measured that the cost is mlua's
callback trampoline at every safepoint (function calls and loop back-edges), ~21-30 ns, paid even
by a callback that returns at once; our check (an atomic counter, the clock every 64th safepoint)
adds ~4 ns on top. A clock read at every safepoint would cost +400 % on a call-heavy loop
(`Instant::now` is 28 ns on the reference laptop). So: a call-heavy pure-Luau loop runs ~1.8×
slower under the watchdog (30 → 55 ns/iteration); API-bound handlers barely notice (handler
dispatch, watchdog included, is ~200 ns per handler). Lever if a measured mod suffers: a raw C
interrupt reading the atomic deadline (a few ns; needs one named, reviewed exception to
`unsafe_code = "forbid"`), or upstream a cheaper mlua hook.

Once the deadline fires, the watchdog **latches**: every later safepoint raises until the host
disarms it. Without the latch, `while true do pcall(function() while true do end end) end` ran for
minutes, because the 64-safepoint sampling always landed inside the inner `pcall` (found by the
L1 test).

With a memory limit set, mlua wraps every allocating call from Rust in a protected call (an
allocation failure must not unwind through Rust frames): building a 3-field payload with two
handles costs 2.0 µs instead of 0.77 µs. Host-side bulk work (payloads, restores, snapshot walks)
therefore runs with the limit lifted and re-applied after (`unlimited`); host data is bounded and
trusted, and a state left over its limit fails at its next script allocation (suspension).

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

| Item | Budget | L1 measured (`cargo bench -p sf_script`, release, `send`) |
|---|---|---|
| All script slots per frame, typical load (vanilla content-pack scripts in a city + 10 native mods) | ≤ 1.0 ms | L2/W3 (needs the Bevy slots) |
| HUD Lua (ui.md) | ≤ 0.2 ms | U7 |
| Rust → Luau handler call | ≤ 0.3 µs median | 66 ns raw, 148 ns guarded (watchdog, stats, `catch_unwind`); ~200 ns per handler in a dispatch incl. payload: **met** |
| Luau → Rust API call (simple getter) | ≤ 0.15 µs median | 38 ns (getter), 48 ns (number → number): **met** |
| Empty context baseline memory | ~~≤ 256 KiB~~ **≤ 512 KiB** | 394 KiB after full GC; a bare mlua Luau state is already 378 KiB, so 256 KiB was below the floor (budget corrected; 50 mods ≈ 20-25 MiB) |
| Context start, 50 mods (no scripts heavier than 1 ms) | ≤ 200 ms at game start | 15 ms (20 handlers each): **met** |
| Hot reload of one mod | ≤ 100 ms (mods with ≤ 1 MiB of state) | 80-133 ms with 1.4 MiB of storage (dominated by snapshot + restore); a few ms for a small mod |
| Save/restore of all script state | ~~≤ 5 ms per MiB~~ **≤ 40 ms per MiB each way** | snapshot 25-33 ms/MiB (Lua walk ~220-280 ns per value, postcard 3-6 ms per 1.4 MiB), restore 31-52 ms/MiB: the old budget was 7× too tight for mlua's per-value cost. Typical mod state is KiB-scale; a save with 1 MiB of script state costs ~70 ms. Lever (M7, if saves need it): a Luau-side encoder/decoder over a `buffer` (one call instead of one mlua op per value), postcard off-thread |

Levers if a budget fails: native codegen per mod (`--!native`), handler throttling for
`on_update` (`every_n_frames`), API batching (`world.nearby` returning arrays rather than
iterators), caching record views per frame.

Moved to [history](archive/scripting-history.md#spike-results-l1-2026-10-13).

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

Moved to [history](archive/scripting-history.md#industry-comparison).

## Open questions

1. ~~`send` feature vs `NonSend` storage, and the world-guard mechanism~~: decided in L1
   (`send`; app-data guard); L2 decided the hand-off: declared slot resources moved into one shared cell.
2. Native codegen default: off (L1 numbers); per-mod opt-in (`--!native`) if a measured mod benefits.
3. Whether entity `tick` should support a declared rate (`tick_hz`) to cut cost for mods that
   poll; legacy scripts must tick every frame (scripting-adapter.md).
4. Player-facing controls for limits (per-mod memory override in the mod manager): M5 has
   profile-level settings only.
5. Shared library mods loaded into many contexts duplicate their code per state; measure with a
   large library before considering a shared read-only module cache.
