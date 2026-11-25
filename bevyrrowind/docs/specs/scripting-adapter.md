# Scripting adapter (MWScript interpreter)

Status: design rewritten 2026-11-25 (DQ-1, docs only) for the interpreter we have. Built: front end (W1),
native interpreter for dialogue results and console lines (W3, [rpg-adapter.md](rpg-adapter.md) "As built (W3)").
Not running yet: object (local), global and start scripts (DQ-4..DQ-11, `docs/roadmaps/dialogue-quests.md` Stage 1).
The October design (translate to Luau at bake time) is superseded:
[archive/scripting-adapter-luau-translation-2026-10.md](archive/scripting-adapter-luau-translation-2026-10.md);
W1/W2 as-built notes: [archive/scripting-adapter-history.md](archive/scripting-adapter-history.md).

Crates: `mw_script` (Bevy-free, no dependencies: lexer, lenient parser, resolver, function table, interpreter,
`Host` trait, instance and scheduling data), `mw_rpg` (Bevy-free: the Morrowind game API `mw_rpg::api`, both hosts),
`mw_scripts_bevy` (thin: active cells, the script step, edge events, applying ops, saves), `mw_rpg_bevy` (live
world reads for the hosts). Framework: `sf_rpg`, `sf_world`, `sf_stats`, `sf_ai`, `sf_ui` own the game-agnostic
operations; `../../../superfargoth/docs/specs/scripting.md` (Luau, parked). Related: `ui-adapter.md` "MWScript UI
calls", `physics-adapter.md` "Scripts", `weather-adapter.md` "Scripting", wiki `docs/wiki/engine/rpg-and-scripting.md`.

## Goal

Run the original game's scripts (SCPT records, start scripts, dialogue result text, console lines) on a native
MWScript interpreter, so quests, activators, doors, shrines and start scripts behave as the content expects. Every
command the interpreter runs calls one shared Rust game API, which the parked Luau layer will later bind as well,
so nothing is written twice and native mods read and write the same state.

Non-goals: MWSE-Lua (patches `Morrowind.exe`, ARCHITECTURE §6.3), bug-for-bug emulation of the original compiler's
undefined behaviour, a Luau translation of MWScript (superseded, above). OpenMW-Lua mods: an option, not a promise
("The Luau layer later").

## The corpus (measured 2026-10-13 on the owner's Steam install)

Counted from the ESMs' raw subrecords (`SCTX` script text, `BNAM` INFO result text) with a
throwaway PowerShell reader in the session scratchpad; record counts from `bevyrrowind info`.

| | Morrowind | Tribunal | Bloodmoon | Total |
|---|---|---|---|---|
| Script records (SCTX) | 632 | 263 | 336 | 1,231 (1,207 live after overrides) |
| Script source | 514 KB | 214 KB | 252 KB | 981 KB, 49,093 lines |
| Largest script | 6.1 KB | 13.0 KB | 8.6 KB | |

- Script length: 51 under 10 lines, 589 of 10–29, 510 of 30–99, 79 of 100–299, 2 of 300+.
  Mostly small state machines.
- Start scripts (SSCR): 4. Global variables (GLOB): 143.
- Dialogue result scripts: 17,144 INFO records carry result text (776 KB, 44,206 lines),
  mostly one-liners.
- Locals declared: ~2,120 `short`, ~380 `float`, 14 `long`.

Most-used constructs (case-insensitive token counts; comments included, so approximate):

| Construct | In scripts | In dialogue results |
|---|---|---|
| `GetJournalIndex` | 1,468 | 0 |
| `Journal` (incl. the above) | 749 | 5,138 |
| `Disable` / `Enable` | 1,434 / 694 | 74 / 136 |
| `MessageBox` / `GetButtonPressed` | 573 / 84 | 13 / 0 |
| `MenuMode` | 321 | 0 |
| `GetDistance` | 321 | 0 |
| `GetItemCount` / `AddItem` / `RemoveItem` | 296 / 160 / 255 | 0 / 1,678 / 1,040 |
| `OnActivate` / `OnDeath` / `OnPCEquip` / `OnPCHitMe` | 286 / 251 / 97 / 92 | 0 |
| `GetSecondsPassed` | 267 | 0 |
| `GetDeadCount` | 255 | 0 |
| `StartCombat` / `StopCombat` | 216 / 127 | 363 / 15 |
| `AddSpell` / `RemoveSpell` / `Cast` | 176 / 82 / 123 | 59 / 9 / 9 |
| `Say` / `ForceGreeting` | 186 / 144 | 7 / 28 |
| `StopScript` / `StartScript` | 152 / 62 | 3 / 200 |
| `AIWander` / `AITravel` / `AIFollow` | 143 / 95 / 29 | 125 / 7 / 118 |
| `PlaySound` / `PlayGroup` | 138 / 112 | 3 / 0 |
| `GetPos` / `Rotate` / `PositionCell` | 128 / 126 / 107 | 0 / 0 / 20 |
| `Lock` / `Unlock` | 106 / 47 | 32 / 24 |
| `Goodbye` / `Choice` / `ModDisposition` / `ModPCFacRep` / `ShowMap` | 0 / 7 / 11 / 3 / 7 | 1,889 / 1,708 / 1,809 / 575 / 536 |
| `->` (explicit reference) | 3,000 | 2,996 |

Mods multiply this (Tamriel Rebuilt alone ships thousands of scripts), so the design must
handle tens of thousands of scripts, not 1,207.

## MWScript semantics we must reproduce

From public documentation (UESP "Morrowind Mod" pages, "Morrowind Scripting for Dummies") and
in-game observation; OpenMW used only as a black-box oracle.

1. **Polling.** A running script executes top to bottom **every frame**; `Return` ends this
   frame's run. There are no functions, no gotos; `If/ElseIf/Else/EndIf`, `While/EndWhile`.
2. **Global vs local.** Global scripts run while started (`StartScript`, start scripts, or
   targeted `ref->StartScript`); local scripts are attached to a base record (`SCRI`) and run
   per placed instance **while the instance is in an active cell** (exterior: the 3×3 cells
   around the player; interior: the current cell), not while merely rendered.
3. **Variables.** `short` (16-bit), `long` (32-bit), `float` (32-bit) locals per instance,
   zero-initialized, persisted in saves; globals from GLOB records; other objects' locals read
   as `id.var`; dialogue filters (`Local`, `NotLocal`) read locals too.
4. **Edge-triggered functions.** `OnActivate`, `OnDeath`, `OnPCEquip`, `OnPCHitMe`,
   `OnKnockout`, `OnMurder`, `OnPCAdd`, `OnPCDrop`, `OnRepair`, `OnPCSoulGemUse` return 1 once
   after the event and then 0. **A script that contains `OnActivate` replaces the object's
   default activation**; it must call `Activate` to perform it.
   Correction 2026-11-25 (UESP, verify in DQ-6): `OnPCEquip`, `OnPCDrop`, `OnPCSoulGemUse`, `OnPCRepair` are mostly declared
   as `short` locals that the engine sets to 1 and the script resets (W1's `ShadowedName` hits),
   and a `PCSkipEquip` local blocks equipping; the host sets them as variables, not edge flags.
5. **Message boxes.** `MessageBox "fmt" args... "button"...`: OpenMW `std::format` substitution (`ui-adapter.md` "As built (MessageBox formatting)"; lowering written 2026-11-08, UNTESTED);
   without buttons a timed notification, with buttons a modal; `GetButtonPressed` returns −1
   until chosen, then the index once.
6. **`MenuMode`** is 1 while a menu is open; local scripts keep running in menus, so most begin
   with a `MenuMode` guard. **`GetSecondsPassed`** is the frame's delta, used to build timers.
7. **References.** Implicit reference = the instance the script runs on (or the speaker for
   dialogue results); explicit `id->Function` targets the object with that ID (for non-unique
   IDs, one arbitrary instance: an inherent ambiguity we resolve deterministically, nearest
   active instance first, then lowest `PlacementId`).
8. **Journal.** `Journal id n` sets the quest index and adds the entry; `SetJournalIndex` sets
   without an entry; `GetJournalIndex` returns the current index.
9. **Dialogue results** run once when the response is shown, as the speaker.
10. **Lenient syntax.** The original compiler accepted much malformed text (missing spaces
    around `->` and operators, commas between arguments, stray tokens after statements,
    unbalanced parentheses, quoted and unquoted IDs with odd characters, `End` followed by a
    name). Scripts that never compiled in the original compiler are also present in mods.

## Decision: interpret MWScript, every command through one game API

**Interpret** (owner 2026-10-18 parked Luau; owner 2026-11-25 condition; roadmap decision 1). MWScript runs on
`mw_script`'s tree-walking interpreter (`interp.rs`: `pub trait Host` at `:99`, `Interpreter` at `:208`,
`Interpreter::run(script, locals, host)`), as OpenMW runs it on its own C++ interpreter. The front end is measured:
1,207 live scripts and 17,144 results parse with 0 unrecoverable, parse + resolve ≈ 0.1 s single-threaded (W1);
every vanilla result runs against a neutral host (`real_install::vanilla_corpus_interprets`). Scripts are parsed
once at content load (cached by text digest), never baked: there is no translation product.

**One game API (owner 2026-11-25, the design rule).** Every interpreter command is one call into a shared Rust
game API: queries return values, operations are typed data the host applies on its tick. A `Host` method converts
arguments (MWScript `Ident`, `Target`, `f64`) and calls the API; it holds no game logic, no state and no branching
beyond that conversion. Morrowind semantics (reference resolution, journal raising, item clamps, units) live once,
in the API; the later Luau layer binds the same API, and an OpenMW-Lua compatibility layer would map onto that.
This follows owner 2026-11-08 ("everything starts in Rust, Luau rewrites specific bits later"; rules stay pure
`fn(&In) -> Out`, rule-hooks.md).

**Who owns the API surface.** Game-agnostic operations stay in the framework system that owns the state:
`sf_rpg` (`RpgCommand`, `state.rs:28`: quest stages, topics, items; reads `QuestLog`, inventory counts),
`sf_world` (`PlacementOverlay`, `overlay.rs:20`: enable/disable, move, spawn, remove, per-placement state blobs),
`sf_stats` (`StatOp`), `sf_ai` (package and combat commands, the same `Command` players use), `sf_ui` (notifications,
modal requests, fades). The Morrowind layer is **`mw_rpg::api`** (Bevy-free): `GameView` (queries, grown from today's
`MwWorld`, `runner.rs:35`, `MwView` and `MwSheets`) and `GameOp` (operations, grown from today's `ResultEffect`,
`result.rs:29`), in Morrowind terms (record ids, game units, MWScript targets), lowering each op to framework
commands. `mw_script` stays dependency-free and never names the API: its `Host` is the language boundary.

Superseded and parked code: the Luau emitter (`emit.rs`), the bake step `mwscript.translate` (`mw_bake/src/script.rs`)
and the `mw.legacy` context in `mw_scripts_bevy`. The interpreter takes its numeric rules from `emit.rs` "Numeric
semantics" today; that text moves to `interp.rs` before the emitter goes. Removing them is its own slice (tagged,
CONTRIBUTING "Verdicts": dead code something else replaced).

## Design

### Layers

```
MWScript text -> parse/resolve (mw_script) -> Interpreter -> Host (binding only) --.
@mw/* Luau module (parked) -------------------------------------------------------+
OpenMW-Lua compat (option) -> our Luau API --------------------------------------+
  v
mw_rpg::api: GameView (queries) + GameOp (ops), Morrowind semantics as pure fns
  v lowers to
sf_rpg RpgCommand | sf_world overlay ops | sf_stats StatOp | sf_ai commands | sf_ui requests
  v applied by
mw_rpg_bevy / mw_scripts_bevy appliers on the host tick (the only ECS writers)
```

### The `Host` boundary (`mw_script`)

- Today: typed methods for 13 `Builtin`s (journal, topics, `Choice`, `Goodbye`, items, disposition, `MessageBox`,
  `PlaySound`), `get`/`set` for variables outside the instance, `stat` for the ~140 stat/faction/crime functions
  (`StatCall`, `statfn.rs`), `unhandled` for the rest (neutral value from the function table, reported once).
- Growth follows the `stat` precedent: one method per family taking a family call enum parsed by `mw_script`
  (`scripts`, `world`, `ui`, `dialogue`, `ai`, `magic`, `weather`), each defaulting to `None` (then `unhandled`).
  The trait stays bounded while the 233-row function table fills in. `Random` becomes a builtin (today it reads 0).
- Two hosts implement it in `mw_rpg`: the results host (`RunHost`, `runner.rs:317`, as built) and the script host
  (DQ-4). The first slice that adds the script host moves `RunHost`'s semantics (item clamps, overlay reads,
  journal rule) into `mw_rpg::api`, so both hosts are bindings.

### The game API (`mw_rpg::api`)

As built (DQ-4b1a, 2026-11-25): `api.rs` holds `GameView` (base reads), `Overlay`, `GameRun` (reads see the run's
writes; `ops()`), serde `GameOp` (`ResultEffect` is an alias), `Target { Speaker, Player, Actor }` (= `SelfRef`,
`Player`, `Id`; the rename is mechanical, 29 uses in `mw_rpg_bevy`), and the pure rules; `RunHost` only forwards.
`Target::Speaker` resolves through `ConversationCtx.speaker`: the script host gives a local script a ctx whose
speaker is its instance.

As built (DQ-4b1b, 2026-11-25): `mw_rpg::ScriptHost` (`script_host.rs`) implements `Host` + `InstanceHost` over
one `GameRun` per tick (each script sees the committed writes of the ones before it); `begin_run` takes a
`Checkpoint`, `end_run(_, false)` rolls back that run's ops and overlay writes. A targeted global's self is
`Target::Actor(target id)` (or `Player`), never `Speaker`: a tick's runs share one op list, so Speaker would be
ambiguous. A self call in a plain global, `Choice`/`Goodbye` outside dialogue: `ScriptReports` (once each), no op.
Start/Stop/ScriptRunning stay in `GlobalInstances`. DQ-4b2 needs: a public world `Env` (`runner.rs` `WorldEnv` is
private) for `ScriptSet::parse_insert`, a pre-tick snapshot of global locals for `scriptid.var` reads, a
`SetLocal{Actor(scriptid)}` applier through `GlobalInstances::get_mut`; exact-reference targets need a `Target`
reference variant (semantics rule 7).

As built (DQ-4b2a, 2026-11-25): `mw_rpg_bevy/src/dialogue/global_scripts.rs`, resource `GlobalScripts` (every SCPT
parsed once at new game with `mw_rpg::WorldEnv`, `GlobalInstances`, one interpreter, reports logged once). New game
starts `main` then each SSCR script in record order (OpenMW `GlobalScripts::addStartup`, 0.49.0), right after
`skip_chargen`. `tick_global_scripts` (`FixedUpdate`, every tick, menus included) runs one `ScriptHost` and applies
its ops through `Rpg::apply(None, ..)`, the console's applier. Step limit = the tunable's default (not the store yet).
Save (DQ-4b2b): section `mw/scripts` v1 (`mw_saves_bevy`), `Option<GlobalsSnapshot>`; load replaces the instances
wholesale (`LoadReport`s logged once); a save without the section (older saves) restarts the start list.
Locals (DQ-4b2c): `Rpg` keeps a `ScriptLocals` copy (declared slots, values, queued writes) read by `View`; an id naming
a script record is that global even if an actor shares it (OpenMW `compilercontext.cpp` `getMemberType`,
`interpretercontext.cpp` `getMemberLocals` @ 4e718b50); never-started reads 0, a write creates it stopped. In a tick,
globals see others' tick-start locals plus earlier `set id.var` writes; console/dialogue writes are seen at once and
reach the instance at the next tick start; `snapshot` includes queued writes.

- **Shape.** `GameView` answers reads; a run produces `GameOp`s in call order and mutates nothing; an overlay
  makes later reads in the same run see its writes (as built for results, as in the original engine). Ops are
  serde data, so a run is replayable and the host is authoritative (net rule: presentation never feeds gameplay).
- **Targets.** `SelfRef` (the running instance: speaker for results, the placement for local scripts, the
  targeted reference of `ref->StartScript`, none for plain globals), `Player`, `Id(record)`. Resolution to a live
  reference is one function with semantics rule 7 (nearest active instance, then lowest `PlacementId`).
- **Units.** Game units and degrees, as MWScript and OpenMW Lua speak; lowering converts through `sf_coords` and
  `mw_world`'s unit scale. The framework commands are in metres.
- **Morrowind rules** inside the API are pure `fn(&In) -> Out` (journal raising, `RemoveItem` takes at most what
  is held, disposition clamps, faction rank rules in `mw_rpg::script_stats`).

| Family | MWScript | API | Framework target | Slice |
|---|---|---|---|---|
| Variables | locals, globals, `id.var` | `var`/`global`, `SetLocal`/`SetGlobal` | script store (below), GLOB values | built for results; DQ-4 |
| Journal | `Journal`, `SetJournalIndex`, `GetJournalIndex`, `AddTopic` | `quest_stage`, `Journal`, `SetJournalIndex`, `AddTopic` | `RpgCommand::SetStage`/`AddTopic` | built for results |
| Items | `AddItem`, `RemoveItem`, `GetItemCount` | `item_count`, `AddItem`, `RemoveItem` | `RpgCommand` | built for results |
| Scripts | `StartScript`, `StopScript`, `ScriptRunning`, SSCR | `script_running`, `StartScript { script, target }`, `StopScript` | script store | DQ-4 |
| Placement | `Enable`/`Disable`, `Position*`, `Set/GetPos`, `Set/GetAngle`, `Rotate`, `PlaceAt*`, `PlaceItem`, `GetDistance` | `position`, `distance`, `enabled`, `SetEnabled`, `MoveTo`, `Spawn` | `sf_world` overlay ops (Bevy-free data; `sf_script_bevy` `WorldCommand`, `world.rs:57`, maps onto them) | DQ-7 |
| Locks | `Lock`, `Unlock`, `GetLocked` | `lock`, `SetLock` | per-placement state in the overlay (lock system's blob) | DQ-7 |
| Activation | `OnActivate`, `Activate` | edge flag, `Activate` (default action) | the activation pipeline (`mw_scripts_bevy`, doors) | DQ-6 |
| UI and time | `MessageBox`, `GetButtonPressed`, `MenuMode`, `GetSecondsPassed`, `Fade*`, `Enable*Menu` | `button_pressed`, `menu_mode`, `seconds_passed`, `MessageBox`, `Fade` | `sf_ui` requests, `mw_ui` formatter, tick dt | DQ-8 |
| Dialogue | `ForceGreeting`, `Choice`/`Goodbye` outside results, `GetDisposition` | `ForceGreeting`, `disposition` | `sf_rpg` `DialogueSession` | DQ-9 |
| Stats, factions, crime | `statfn` set | `Host::stat` → `mw_rpg::script_stats` | `sf_stats` `StatOp` | built, unit-tested only; DQ-10 |
| AI and combat | `AI*` packages, `Start/StopCombat`, `OnDeath`, `OnPCHitMe`, `OnMurder`, `OnKnockout` | package and combat ops, edge flags | `sf_ai` | ai-adapter MA2/MA4-MA6 (Stage 2) |
| Magic, weather, sound | `Cast`/`AddSpell`/`GetEffect`, `ChangeWeather`/`ModRegion`, `PlaySound`/`Say`/`StreamMusic` | ops per family | `sf_effects`, `sf_weather`, audio events | their lanes; sound is events only |

### Instances, scheduling, state

The instance data and ordering are Bevy-free in `mw_script` (keyed by a caller-supplied id); `mw_scripts_bevy`
schedules them. Reference OpenMW `apps/openmw/mwscript/globalscripts.cpp`, `mwworld/localscripts.cpp`,
`mwscript/*extensions.cpp` (each slice cites the commit it read).

- **Script set.** Every live SCPT (winning plugin) is parsed once at content load; inert scripts never run and
  are reported. Dev hot reload of an edited ESP script re-parses it and remaps its instances' locals by name.
- **Global instances**, keyed by script id: running flag, locals, optional target (`ref->StartScript`), start
  sequence number. `StartScript` on a running script does nothing; on a stopped one it restarts with its locals
  kept; `StopScript` stops after the current run (OpenMW `GlobalScripts`, verify both in DQ-4). Start scripts
  (SSCR) start on a new game; a loaded save restores the running set and starts SSCR entries it lacks (a mod added
  since).
- **As built (DQ-4a, 2026-11-25, `mw_script::instances`):** `ScriptSet` (lower-cased ids, inert scripts listed with
  their diagnostics) and `GlobalInstances` (`start`/`stop`/`is_running`, `tick(set, interp, host)` in start order,
  `take_reports`, `snapshot`/`load` by name and type with `LoadReport`s). `StartScript`/`StopScript`/`ScriptRunning`
  are builtins through `Host::script(target, ScriptOp, id)`; inside a run they queue and apply after it (dropped on a
  fault), `ScriptRunning` sees the queue. Hosts that run instances implement `InstanceHost` (`begin_run` seeds
  `Random` and the implicit target; `end_run(committed = false)` drops the buffered game ops on a step-limit fault).
  Step limit = tunable `mw.script.step_limit` (100k, 1k-10M, Guess). Checked against OpenMW `globalscripts.cpp` @
  `c31b4efc`: same no-op start, restart keeps locals and takes the new target, stop lets the run finish, failed run
  stops, load drops unknown scripts. Departures (deterministic, net-ready): OpenMW runs globals in hash-map order and
  applies Start/Stop at the call; ours run in start order, a restart takes a new sequence number (moves to the end).
- **Local instances**, one per placed reference whose base record has a script, keyed by `PlacementId`; also
  spawned references and items in the player's inventory (key `(holder, StackId)`; OnPCAdd/OnPCEquip need them).
  Created when the reference first enters the script-active region, with zeroed locals.
- **Script-active region** (semantics rule 2): exterior, the cells within `script_active_radius` Morrowind cells
  of the player's cell (tunable, cells, 1-3, default 1 = 3×3, source OpenMW setting "exterior cell load distance");
  interior, the current cell. Computed from the player's cell, independent of render streaming rings. Co-op: the
  union around every player; the host runs all scripts.
- **Per tick** (`FixedUpdate`, host only): (1) latch edge events from the previous tick's messages; (2) globals in
  start order; (3) locals by key order (`PlacementId`, then inventory stacks). Each run's ops are applied before
  the next run starts, so later scripts see earlier writes (sequential, as in the original engine); entity spawns
  and despawns reach the ECS at the end of the step. Scripts started this tick first run next tick. Order of (2)
  and (3) is checked against OpenMW's frame loop in DQ-4 and follows it if it differs.
- **Decided (DQ-5a, 2026-11-25):** OpenMW runs locals before globals (`engine.cpp:283-286` @ 4e718b50), so ours does:
  (1) edges, (2) locals by key, (3) globals in start order. A global a local starts this tick therefore runs in step (3)
  of the same tick, as in OpenMW; every other start first runs next tick. Disabled references keep running their
  local scripts (Q4: `World::disable` never touches `LocalScripts`; only Lua's `pauseLocalScript` flag skips them).
- **As built (DQ-5a):** `mw_script::LocalInstances` (`locals.rs`), `LocalKey { Placement(u64), Stack { holder, stack }
  }` (derived order = tick order), `insert` (zeroed; same script keeps locals), `take`/`restore` by name (for the
  overlay, DQ-5c), `set_paused` (the Lua-only flag, saved; never wired to `Disable`), `tick(set, interp, &mut
  GlobalInstances, host)`, `snapshot`/`load`. `InstanceHost` gains `begin_local_run(script, key)` and
  `local_self(key)`. Departures: key order (OpenMW: cell-add order), a faulted instance is paused in a saved flag,
  new locals start at zero.
- **As built (DQ-5b1):** `mw_rpg_bevy/src/dialogue/local_scripts.rs`: `ScriptRefs` indexes every CELL reference whose
  base (17 kinds with a script field) has a script, keyed by `mw_world::ids::placement_id`, grouped by exterior grid or
  interior name; `ScriptRegion` (player body's cell) and `LocalScripts::sync` re-diff only when centre or radius
  changes (take = parked with locals, return = restore). `GlobalScripts::tick` syncs, ticks locals then globals through
  one `ScriptHost`. Self = `Target::Ref(u128)` (exact placement; `Content::target` maps it to `ActorKey(p)`); a global
  started by a local without `ref->` gets the base id. Tunable `mw.script.active_radius` (1-3, default 1; store not
  bound yet). Not yet: `ref.var` of locals (DQ-5b2), inventory stacks (DQ-5b3).
- **Save (DQ-5c):** `mw/scripts` v2 = `ScriptsSave { globals, locals, parked }` (parked = taken instances, key order);
  v1 → v2 typed step (globals kept, locals empty). Load re-syncs the region next tick: live-outside gets parked,
  parked-inside resumes, nothing zeroed; a key in both lists = live wins; unknown scripts: one report per instance.
- **Object locals (DQ-5b2):** `ObjectLocals` (`local_scripts.rs`) beside the globals' copy in `ScriptLocals`: `Speaker`/
  `Ref(p)` read the instance's locals, `Actor(base)` by rule 7 (nearest live to the player by CELL position, then
  lowest `PlacementId`; none live: lowest id). One store: a scripted placement's (NPC's) locals live only in its
  instance; `state.locals` remains for unscripted keys; a new instance is seeded from chargen locals then moved-out
  `state.locals` entries. Parked instances read and take writes (OpenMW keeps locals in `RefData`); a write to a
  never-run reference creates it parked. Same-tick visibility as DQ-4b2c.
- **Time and menus** (rule 6). `GetSecondsPassed` = the fixed-tick dt. The step also runs while a menu is open
  (`MenuMode` 1) even when game time is paused; what `GetSecondsPassed` returns then is checked in DQ-8.
- **Randomness.** `Random` draws from a generator seeded by host tick and instance key (deterministic, net-ready).
- **Edge events** (rule 4): `Activated`, item added to the player, and later died/hit/knocked out/murdered set a
  flag on the target's instance before the tick; the function returns 1 once (cleared on read). The `OnPC*`
  special locals (equip, drop, soul gem use, repair) are set as variables instead. Sources: the interaction lane's
  `Activated`, `sf_rpg` commands applied for the player, combat and AI lanes (Stage 2; until then never fire).
- **Activation** (DQ-6). The resolver marks scripts that contain `OnActivate`; for those the default action is
  suppressed and the edge set; `Activate` runs the default action (door travel, pick-up) through the same
  pipeline that Luau mod handlers use (`handled` blocks the default, scripting.md L3). Scripts without
  `OnActivate` leave the default action alone.
- **Message boxes** (DQ-8): a `MessageBox` with buttons opens a modal; the pending choice lives with the
  instance, so `GetButtonPressed` reads −1 until chosen, then the index once, and survives a save.
- **Saves.** Global instances (id, running, target, locals), GLOB values, pending choices, and inventory instances
  go in the script save section; local placement instances store their locals in the placement overlay's
  per-placement state (blob `mw.script`), so they survive cell unloads and saves. Locals are saved **by name and
  type**, remapped by name on load (a mod that changed the script keeps what still matches; the rest start at 0,
  reported). Edge flags are not saved.
- **Faults.** A step-limit stop (`RunError::StepLimit`, 100k steps; the limit becomes a tunable in DQ-4) drops
  that run's ops, stops the instance until reload or the next `StartScript`, and is reported once with script id
  and line. Unhandled functions read their neutral value and are reported once per function.
- **Budget.** Legacy script time ≤ 0.3 ms per frame in Balmora with all scripts (tunable
  `legacy_script_budget_ms`, Guess, October spec), measured by DQ-11. If it misses: compile the AST to a flat
  bytecode (OpenMW's form).

### Dialogue results and the console (as built)

Results run once when the response is shown, as the speaker (rule 9), through `mw_rpg::MwResults` and the results
host; the console runs one line as the selected actor or the player (`mw_rpg_bevy::dialogue::run_console`). Once
local scripts run, the speaker's locals that results and filters (`Local`, `NotLocal`) read are that speaker's
local instance: one store, not a copy.

## The Luau layer later (parked), and OpenMW Lua mods (option)

- **Luau** (`superfargoth/docs/specs/scripting.md`, parked after L3) binds the framework operations as `@sf/*` and
  `mw_rpg::api` as `@mw/*`. A Luau mod reads and writes MWScript locals, globals, journal and items through the
  same ops, so a mix of MWScript and Luau content shares one state. Luau later rewrites specific bits (events,
  rule overrides), never re-implements an op.
- **Option, not a promise: OpenMW Lua mods.** A compatibility layer could map OpenMW's Lua API (`openmw.*`
  packages: `core`, `world`, `self`, `nearby`, `types`, `ui`, ...; global, local and player scripts talking by
  events) onto our Luau API; the API speaks game units and record ids partly for this. Gaps to weigh then: LuaJIT
  (Lua 5.1 plus `goto` etc.) versus Luau, asynchronous events, `omwscripts` attachment. No slice: waits for Luau
  to unpark and the owner's go.

## Function status (replaces "What M5 needs from M7")

Stubs return neutral values so scripts take their "not applicable" branch (the temple shrine asks the player's
Temple rank; the neutral −1 charges the non-member donation and proceeds). `functions.txt`'s `status` column
(`live`/`stub`/`later`) is the October Luau plan, not what runs (W2's 55.9 % "live" counted it): DQ-2 adds what the
interpreter binds and prints call sites bound / total by family. **Measured 2026-11-25 (DQ-2):** 16776 of 26704 call
sites bound (62.8 %; the old `live` column claims 81.7 %): results 13026 / 15342 (84.9 %, the part that runs today),
scripts 3750 / 11362 (static: object and global scripts run from DQ-4/DQ-5). The bound set is `interp::Binding::of`
(the dispatch itself; stat functions count as bound through `Host::stat`, 1475 sites). Top unbound: `Disable`,
`Enable`, `SetFight`, `StartCombat`, `ShowMap`, `GetDisabled`, `GetDistance`, `MenuMode`, `OnActivate`, `AIWander`.
Today, by family:

| Family | Today | Next |
|---|---|---|
| Control flow, locals, globals, `Return` | live in results and console | object/global scripts: DQ-4/DQ-5 |
| `Random`, `MenuMode`, `GetSecondsPassed`, `GetPCCell`, `GetInterior` | not bound (neutral) | DQ-4, DQ-8 |
| `StartScript`/`StopScript`/`ScriptRunning`, start scripts | **not live**: no runtime use | DQ-4 |
| Activators: `OnActivate`, `Activate`, `Lock`/`Unlock`/`GetLocked` | **not live**: doors use the default action only | DQ-6, DQ-7 |
| `Enable`/`Disable`, positions, `PlaceAtPC`/`PlaceItem`, `GetDistance` | not bound | DQ-7 |
| `Journal`/`SetJournalIndex`/`GetJournalIndex`/`AddTopic` | live in results (DJ-1 verified) | scripts: DQ-4 |
| `AddItem`/`RemoveItem`/`GetItemCount` | live in results and console | scripts: DQ-4 |
| `MessageBox` | results: formatted, buttons parsed (modal untested); `GetButtonPressed` not bound | DQ-8 |
| `Choice`, `Goodbye`, `ModDisposition`/`SetDisposition` | live in results | outside results, `GetDisposition`: DQ-9 |
| Stats, skills, factions, crime (`statfn`) | bound, unit-tested only | DQ-10 on real data |
| `PlaySound`, `Say`, `StreamMusic` | events only | audio lane |
| AI packages, combat, death/hit events | stub, never fire | ai-adapter MA* (Stage 2) |
| Magic, weather, `ShowMap`, `ForceGreeting` | stub | their lanes; `ForceGreeting` DQ-9 |

## Verification

- **Corpus (real install, as built)**: every vanilla script and result parses (0 unrecoverable, counts by rule);
  every result runs once against a neutral host. Counts only: vanilla text is never committed (Bethesda content);
  goldens use our own fixtures.
- **Coverage (DQ-2, as built)**: `vanilla_corpus_interpreter_coverage` (`mw_script/tests/real_install.rs`, ignored
  measurement) prints call sites bound / total by family and origin, the old `live` share and the top unbound
  functions; `Coverage::bound_report`. Each run's total goes in `docs/ledger/dialogue-quests.md`.
- **Headless fixtures, exact**: start order, `StopScript`, restart keeps locals, save round trip by name (DQ-4);
  ticks only in the active region, locals survive unload and save (DQ-5); `OnActivate` once, default suppression,
  `Activate` passthrough (DQ-6); each world op (DQ-7); message-box state machine, `GetSecondsPassed` timers (DQ-8).
- **Binding rule**: each slice's review checks that `Host` impl methods only convert and call `mw_rpg::api`.
- **Main-quest trace (DQ-3)**: furthest stage per quest, rerun after every slice (measurement).
- **Live gate (DQ-11)**: Balmora 5 min with all scripts: 0 runtime errors (or each listed with a reason),
  legacy script ms per frame against `legacy_script_budget_ms`.
- **Owner scenes**: MS-4 (shrine of St. Rilm, `Balmora, Temple`: donation, blessing), MS-5 (a door with a
  confirmation script: message box, Yes travels, No stays), DJ-3 (new game to Caius' orders).

## Industry comparison

2026-11-25; "unverified" marks memory not re-read this session.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Legacy script execution | OpenMW: compiles MWScript to its own bytecode (`components/compiler`) and runs it on its interpreter (`components/interpreter`); `--script-all` reports failures | tree-walking interpreter over the lenient AST, parsed at load, step-limited | **match** (interpreted); AST walker instead of bytecode until DQ-11 measures a need |
| Command dispatch | OpenMW: one opcode class per function in `mwscript/*extensions.cpp`, calling engine subsystems | `Host` method → one `mw_rpg::api` op → framework command | **match** in shape; ours are data (replay, host authority) |
| Modern scripting beside legacy | OpenMW: Lua for new mods and for mechanics moved out of C++ into built-in Lua scripts; MWScript stays supported and interpreted; Lua reads mwscript variables (`world.mwscript`, unverified) | Luau (parked) binds the same API; MWScript stays interpreted | **match** direction |
| One native API for scripts | Papyrus (Skyrim, Fallout 4): scripts call engine "native" functions; SKSE adds natives to the same table (unverified) | one game API for MWScript, Luau and any compat layer | **match** |
| Fault isolation | Papyrus logs errors and continues (unverified) | step limit, run dropped, instance stopped, reported once | **match** |
| Active-cell scripting | Morrowind/OpenMW: local scripts run in active cells only | same rule, independent of render streaming | **match** |

## Open questions

1. Integer division and comparison quirks between `short`/`float`: settle with the OpenMW oracle and record the
   rule here (the interpreter follows `emit.rs` "Numeric semantics" today).
2. Non-unique explicit references (`"id"->Disable` with several instances): nearest active first is our rule;
   confirm against the oracle on a known vanilla case.
3. Order of global versus local scripts within a frame, and whether a script started this frame runs at once
   (DQ-4, from OpenMW's frame loop).
4. ~~Do disabled references keep running their local scripts?~~ Yes (DQ-5a, OpenMW `worldimp.cpp:772`).
5. Item scripts: do locals move with an item between the world and an inventory? DQ-6.
6. `GetSecondsPassed` while a menu is open (DQ-8).
7. Resolved: `AIWander` through the M3 scaffold (old question 3) is left to the AI lane (roadmap decision 2).
