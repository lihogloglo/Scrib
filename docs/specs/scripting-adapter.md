# Scripting adapter (MWScript → Luau)

Status: draft (2026-10-13, m5-spec session, docs only). Built in M5; W1 front end built 2026-10-13 (m5-w1, see "As built (W1)").
Crates (proposed): `mw_script` (Bevy-free: lexer, lenient parser, AST, function table,
semantic checks, Luau emitter, diagnostics), a `mwscript.translate` step in `mw_bake`,
`mw_script_bevy` (thin: the `@mw/script` API module over the framework APIs, legacy instance
bookkeeping), and the Luau half of the `mw` runtime library as files in Bevyrrowind's builtin
mod. Framework specs: `../../../superfargoth/docs/specs/scripting.md` (runtime),
`../../../superfargoth/docs/specs/mods.md` (pipeline). Related: `ui-adapter.md` "MWScript UI
calls", `physics-adapter.md` "Scripts", `weather-adapter.md` "Scripting".

## Goal

Make the original game's scripts (SCPT records and dialogue result text) run on the framework's
Luau runtime, so quests, activators, doors, shrines and start scripts behave as the content
expects, and so native Luau mods can read and write the same state. ARCHITECTURE §6.3 chose a
transpiler; this spec confirms that with evidence, specifies it, and scopes what can run in M5
before the RPG core (M7) exists.

Non-goals: MWSE-Lua (patches `Morrowind.exe`, §6.3), OpenMW-Lua compatibility (`mw_openmw_lua`,
after the core game), bug-for-bug emulation of the original compiler's undefined behaviour.

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
5. **Message boxes.** `MessageBox "fmt" args... "button"...`: `printf`-style substitution;
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

## Decision: translate to Luau at bake time (not an interpreter)

| Criterion | Translator (MWScript → Luau source → Luau bytecode) | Runtime interpreter (own bytecode VM in Rust) |
|---|---|---|
| Runtime machinery (sandbox, limits, error isolation, save state, hot reload, diagnostics, debugger story) | inherited from `sf_script` | all built a second time |
| Interop with native Luau mods (read/write locals, globals, journal) | same tables in the same VM family | a bridge layer |
| Speed | Luau-compiled ticks (register VM, constant folding) | a hand-written interpreter must beat Luau's to matter; no reason it would |
| Fidelity | the parser and function table carry the semantics; integer wrap / float32 rounding are emitted as helper calls where they can differ | same parser and function table needed anyway |
| Cost per load | translate once per script fingerprint (bake product); load = bytecode | compile at every start (OpenMW's approach) |
| Debugging | generated Luau with `-- L<n>` source-line comments; errors report the MWScript line | needs its own tooling |

Evidence it fits: the control flow maps 1:1 onto Luau; per-frame polling maps onto an entity
`tick`; edge-triggered functions map onto per-instance flags set by engine messages and cleared
when read; locals are plain data, which is exactly what `sf_script` persists. The corpus is
small (1.76 MB of source including dialogue), so translating all of it is a fraction of a
second, then cached. OpenMW (black box) compiles all scripts at startup to its own bytecode and
interprets them; it offers compile-everything switches (`--script-all`,
`--script-all-dialogue`) that report scripts failing to compile and keeps running when one
fails. We take the observable lesson (compile everything up front, report, never crash) and
cache the result instead of recompiling each start.

The same `mw_script` crate is a library, so it also translates **live** where a bake makes no
sense: the dev console (one line → snippet → run as the selected object) and dev hot reload of
an edited ESP script.

## Design

### Pipeline

1. **Lex/parse** (lenient): tokens are case-insensitive; every leniency is a **named rule**
   (`MissingSpaceAroundArrow`, `CommaBetweenArgs`, `TrailingTokens`, ...) that records a
   diagnostic, so the corpus test can count how often each fires. Unrecoverable scripts are
   reported and **inert** (never crash, never half-run).
2. **Resolve**: locals and types, globals (GLOB), explicit references (record IDs, `Player`),
   function calls against the **function table**, other scripts' variables (`id.var`), journal
   and topic IDs (warnings for unknown IDs, as they may come from later plugins at runtime).
3. **Emit Luau**: one module per script, one function per dialogue result (grouped per topic).
   Emitted code keeps all state in module tables, never in globals: in Luau sandbox mode a state
   has one writable environment shared by every chunk it loads (m5-l1 finding, scripting.md
   "Spike results (L1)"), so globals would leak between translated scripts in the shared state.
4. **Compile**: Luau bytecode via mlua's `Compiler` (optimization level 2), stored as a bake
   product. Only engine-compiled bytecode is ever loaded (scripting.md "Modules").

### Generated shape (illustration, not vanilla text)

```lua
-- generated by mw_script v1 from SCPT "example_door" (example.esp), do not edit
return function(mw)
  return mw.script {
    id = "example_door",
    vars = { button = "short", asked = "short" },
    activation = "script",                 -- reads OnActivate: default activation suppressed
    tick = function(self, v)               -- self: handle; v: this instance's variables
      if mw.menu_mode() == 1 then return end                     -- L4
      if self:on_activate() == 1 then                            -- L7
        mw.message_box(self, "Go through?", nil, { "Yes", "No" }) -- L8
        v.asked = 1
      end
      if v.asked == 1 then                                       -- L11
        v.button = mw.short(self:get_button_pressed())           -- L12
        if v.button >= 0 then v.asked = 0 end
        if v.button == 0 then self:activate() end                -- L15
      end
    end,
  }
end
```

- `mw.short`/`mw.long` wrap assignments to integer locals (two's-complement wrap, truncation
  toward zero); floats are Luau doubles rounded through `f32` on assignment
  (`mw.float`), matching the original's storage.
- Comparisons and arithmetic follow MWScript's numeric rules (integer division semantics where
  both sides are integers is checked against the oracle in W1 and emitted as a helper if it
  differs).
- `id->Fn args` becomes `mw.ref("id"):fn(args)`; references are cached per instance.

### The `mw` runtime library

- **Function table** (data, one row per MWScript function, written from UESP's function list):
  name, argument kinds, return kind, explicit-reference allowed, and **status** (`live`,
  `stub(neutral value)`, `later(Mx)`). It drives the parser (arity, argument kinds), the emitter,
  the generated `@mw/script` type definitions and the coverage report.
- **Luau half** (`scripts/mw/*.luau` in the builtin mod): `mw.script`, numeric helpers,
  message-box state machine, edge flags, timers. Editable and hot-reloadable like any mod code.
- **Rust half** (`@mw/script` in `mw_script_bevy`): functions that need the world, written only
  against framework APIs (`@sf/world`, `@sf/ui`, `@sf/anim`, `@sf/physics`, records) plus the
  small M5 data stores below. It never reaches into Bevy components the framework doesn't
  expose, so the framework's API stays the contract.

### Instances, scheduling, state

- All translated scripts run in **one shared context** (`mw.legacy`, owned by the builtin mod):
  scripts are records, so the winning plugin's version runs, and scripts address each other by
  ID across plugins (scripting.md "Contexts").
- **Global instances** keyed by script ID; **local instances** are framework entity scripts
  keyed by `PlacementId`, with variables stored in the placement overlay (saved, survive cell
  unloads).
- **Script-active region**: local scripts tick only for instances inside the MWScript active
  cells (rule 2), computed by the adapter from the player's cell, independent of the render
  streaming rings (which are much larger). Globals tick always.
- Order per frame: globals in start order, then locals by `PlacementId`; deterministic.
- Edge flags (`OnActivate`, `OnDeath`, ...) are set by engine messages (`Activated`, `Died`,
  `Equipped`, `Hit`) in the `Update` slot before ticks; cleared on read or after the next tick.
- Activation: scripts statically marked `activation = "script"` consume `Activated`; their
  `Activate` call runs the default action (door travel, pick-up) afterwards.
- `GetButtonPressed`: the pending choice per instance lives in its variables' hidden slot, so
  it survives saves and hot reloads (ui-adapter.md).

### Dialogue results

Translated and compiled for the whole corpus in M5 (compile coverage). They execute only when
`mw_dialogue` exists (M7); in M5 the dev console can run one against an actor for testing.

## What M5 needs from M7, and the M5 slice

Each function gets a status. **Stubs return neutral values** so scripts take their
"not applicable" branch (example: the temple shrine script asks the player's Temple rank; the
stub returns −1, so it charges the non-member donation and proceeds).

| Family | M5 status | Needs |
|---|---|---|
| Control flow, locals, globals, `Return`, `Random` | live | — |
| `MenuMode`, `GetSecondsPassed`, `GetPCCell`, `GetInterior` | live | UI layer state, clock |
| `OnActivate`, `Activate`, door activation, `Lock`/`Unlock`/`GetLocked` | live | generic `Activated` message (from the M4 P11 interaction work or a small `sf_interaction` task in M5); lock state in the placement overlay |
| `Enable`/`Disable`/`GetDisabled`, `Position`/`PositionCell`/`SetPos`/`GetPos`/`SetAngle`/`GetAngle`/`Rotate`, `PlaceAtPC`/`PlaceItem`, `GetDistance` | live | placement overlay + physics hooks (physics-adapter.md "Scripts") |
| `MessageBox`, `GetButtonPressed`, `FadeIn`/`FadeOut`, `Enable*Menu` flags | live | UI slice message box, fade overlay |
| `StartScript`/`StopScript`/`ScriptRunning`, start scripts | live | — |
| `Journal`/`SetJournalIndex`/`GetJournalIndex`/`AddTopic` | live (state only) | **minimal quest store** (`sf_rpg`: quest → index, entries list, known topics; plain data, saved). Journal UI is M7 |
| `AddItem`/`RemoveItem`/`GetItemCount` | live | **minimal inventory stacks** (`sf_rpg`: record id → count per actor/container). The M5 inventory screen shows the player's real stacks |
| `PlayGroup`/`LoopGroup` | live | animation slots (animation.md "Slots and montages") |
| `GetDeadCount` | live (returns 0 until deaths exist) | death counter store; fed by M4 ragdoll death if present |
| `PlaySound`, `Say`, `StreamMusic` | stub (logged) | audio system (not scheduled) |
| `AIWander` | optional live via `mw_ai` `PathWalkScaffold` | — |
| `AITravel`/`AIFollow`/`AIEscort`/`StartCombat`/`StopCombat`, `OnDeath`, `OnPCHitMe`, `OnKnockout`, `OnMurder` | stub / never fires | M7 AI, M10 combat |
| Stats, skills, `ModHealth` etc., magic (`Cast`, `AddSpell`, `GetEffect`, disease checks), factions (`GetPCRank` → −1), disposition, reputation, crime | stub (neutral) | M7 |
| `ForceGreeting`, `Choice`, `Goodbye`, `ShowMap` | stub | M7 dialogue, map |
| Weather (`ChangeWeather`, `ModRegion`, `GetCurrentWeather`) | live if `sf_weather` exists, else stub | M6 (weather-adapter.md) |

The minimal quest store and inventory stacks are the first durable pieces of `sf_rpg`'s data
model (ARCHITECTURE §5.3): plain records-and-overlay data with no rules. M7 extends them; it
doesn't replace them. Everything else waits for M7.

**Coverage report** (W2 output): per function, call sites in the corpus and status, so the
roadmap shows how much of the content's script logic is live (by call sites, the live set above
covers the largest families: journal, enable/disable, message boxes, menu guards, distances,
items, activation, timers).

**Demonstrable without M7**: activators, doors, shrines, levers and start-script logic that only
use the live families; quest *state* advancing from scripts (visible in a dev journal-state
overlay, not the journal UI).

## As built (W1, 2026-10-13, m5-w1)

Crate `mw_script` (Bevy-free, no dependencies): `lexer` → line parser → `resolve`, plus the
function table. API: `parse(text, SourceKind, &FunctionTable)`, `parse_and_resolve(.., &dyn Env)`
→ `Parsed { script, diagnostics }`, `Parsed::is_inert()`.

**Reuse check.** No MWScript parser exists on crates.io (searched `mwscript`, `morrowind`,
`tes3`); Greatness7's `tes3` (already a dependency) reads SCPT/INFO but has no compiler; OpenMW's
compiler is C++ and bound to its interpreter. Parser generators (`chumsky`, `lalrpop`) don't fit:
MWScript calls have no parentheses, so argument count and kinds come from a runtime table, and
recovery is per line. Hand-written recursive descent with per-line recovery is the usual choice for
lenient legacy front ends. OpenMW's compiler source was not read; two behaviours below are noted
as "OpenMW-like" from memory of its warnings (unverified), pending the W2/W3 oracle runs.

**Lexer.** Case-insensitive words keep their source spelling (`Ident { key, text }`); newline
tokens (statements are lines); `;` comments; tokens carry `glued` (no whitespace before) so bare
IDs like `Ald-ruhn` can be rejoined in ID positions. Comparison operators are normalised from any
mix of `= < > !` (`=`, `=<`, `<==`, `<>`, `> =`).

**Parser.** One statement per line; leftovers on a line are a named repair, never a cascade.
`Begin`/`End`, `Short`/`Long`/`Float` (locals are script-wide; a bare word that is a declared local
is a variable even before its declaration line), `Set`, `If`/`ElseIf`/`Else`/`EndIf`,
`While`/`EndWhile`, `Return`, calls. Calls take their arguments from the **function table**
(`src/functions.txt`, 452 functions after `{$LIST}` expansion: name, aliases, argument kinds
`id|text|num|axis|any` with `?`/`*`, return `void|int|float`, status family, acts-on-reference).
Numeric arguments are unary expressions (`Random 100 < 50` is `Random(100) < 50`). At a line start
a function wins over a local of the same name; inside expressions the local wins.

**AST for the emitter.** Every variable is a `VarRef` naming where its state lives: `Local(slot)`,
`Global`, `Member { object, var }`, `Speaker` (dialogue results), `Undeclared` (reads 0, writes
dropped). Nothing maps to a Luau global (m5-l1 finding). Statements carry source lines for
`-- L<n>` maps; calls carry `FnId` and typed `Arg`s (`Id`, `Str`, `Expr`, `Axis`, `Missing`).

**Resolution.** `Env` (globals, object IDs, `object.var` via the object's script or a script ID).
Script names that aren't locals or globals → `UndeclaredVariable` (warning); dialogue-result names
that aren't globals → speaker locals; unknown explicit references → `UnknownReference` (warning).
Not yet checked: kinds of ID arguments (journal, topic, cell, sound IDs): W2, with the coverage
report.

**Policy.** Severities: `Accepted` (the original accepted it; counted), `Repaired` (defined
repair, script runs), `Warning` (resolution), `Error` (inert). Only an unknown function at a
statement head, an unparsable expression or a `Set` without a target are errors: an
MWSE-dependent mod script is then inert rather than half-run.

**Corpus result** (real install, `cargo test -p mw_script --release --test real_install --
--nocapture`): 1,231 script texts (1,207 live) and 17,144 dialogue results, **0 unrecoverable**;
400 scripts and 16,710 results with no diagnostics at all; explicit references all resolve.
Parse + resolve of everything ≈ 0.1 s single-threaded (release).

| Rule | Severity | Scripts (hits / texts) | Results (hits / texts) | What happens |
|---|---|---|---|---|
| `CommaBetweenArgs` | accepted | 1,287 / 261 | 1,020 / 255 | `,` is an optional separator |
| `EndWithName` | accepted | 591 / 591 | | `End name`, name unchecked |
| `ShadowedName` | accepted | 113 / 113 | | local named like a function or global |
| `HyphenatedId` | accepted | 2 / 2 | 12 / 12 | `ShowMap Ald-ruhn`, `Begin a-b` |
| `LateDeclaration` / `DuplicateLocal` | accepted | 8 / 4, 1 / 1 | | locals are script-wide; first wins |
| `NonstandardOperator` | repaired | 80 / 52 | | `=` (79) and `<==` as comparisons |
| `ParenthesizedCall` | repaired | 33 / 2 | | `ref->( GetItemCount x ) == 1` |
| `UnmatchedBlockEnd` | repaired | 30 / 26 | | stray `EndIf` (mostly a commented-out `If`), ignored (OpenMW-like) |
| `TrailingTokens` | repaired | 11 / 6 | | junk after `Else` (`else set x to 8`) ignored (OpenMW-like) |
| `ExtraArgument` | repaired | 7 / 7 | 6 / 6 | `GetDisabled "id" == 0`, `Disable "id"`: value skipped, operator kept |
| `ExtraEndIf` | repaired | 4 / 4 | | `ElseIf` right after an `EndIf`: that `EndIf` was extra, block reopened |
| `TextAfterEnd` | repaired | 3 / 3 | | ignored |
| `StrayCharacter` | repaired | 4 / 1 | 162 / 162 | `[ ]` around a condition; `Choice:` |
| `UnbalancedParens` | repaired | 1 / 1 | | |
| `UnknownStatement` | repaired | 2 / 2 | | a bare `( cond )` line, ignored |
| `ReferenceOnVariable` | repaired | 1 / 1 | | `Player->localVar` reads the local |
| `UnknownFunctionInExpression` | repaired | 1 / 1 | | `player->GetStat Level` reads 0 (Tribunal `Plague_Gedna`; inert would break a quest) |
| `DuplicateBegin` | repaired | 1 / 1 | | |
| `UnterminatedString` | repaired | | 1 / 1 | string runs to the line end |
| `UndeclaredVariable` | warning | 13 / 10 | | reads 0 (typos, journal IDs used as variables, a cell name in a numeric slot) |

Rules defined but not fired by vanilla (kept for mods): `SpacedOperator`, `MissingEnd`,
`MissingBegin`, `StatementsBeforeBegin`, `MissingEndIf`, `MismatchedBlockEnd`, `ElseIfAfterElse`,
`ElseIfAsTwoWords`, `MissingCondition`, `SetWithoutTo`, `EmptyDeclaration`, `MissingArgument`,
`ReferenceOnKeyword`, `UnknownMember(Object)`, `UnknownReference`, and the three errors.

**Handoff to W2 (emitter).**
- Input: `Parsed` from `parse_and_resolve` with an `Env` built from the record DB (the corpus test's
  `CorpusEnv` is the model: GLOB types, all IDs, object → script, live scripts' locals). Inert
  scripts emit a stub module and a report line.
- Emit `VarRef` to module tables (`v[slot]` or `v.<key>`, `mw.globals.<key>`, `mw.member(obj,var)`,
  speaker table), never Luau globals; `Arg::Missing` reads 0 / "".
- `Stmt::line` → `-- L<n>`; `FnId` → `functions.txt` row: add a `status` column (live / stub /
  later) next to `family` for the coverage report. Call sites by family (vanilla, scripts + results):
  journal 6,198, inventory 3,560, dialogue 3,546, placement 3,200, disposition 2,220, ai 1,568,
  faction 810, combat 759, magic 659, ui 604, audio 549, map 543, stats 454, activation 428,
  scripts 403, menu 308, events 268, deaths 255, clock 248, the rest < 120 each.
- Still open: integer division and short/float comparison quirks (open question 1) need the oracle;
  ID-kind checks of arguments; `ElseIf` after an extra `EndIf` and stray `EndIf` repairs should be
  confirmed against OpenMW's behaviour on the same vanilla scripts (`HentusTravel`, `GlenmorilRaven`).

## As built (W2, 2026-10-13, m5-w2 + m5-w2b)

**Emitter** (`mw_script::emit`): `emit_script(id, origin, &Parsed, table, env)` → one module per
SCPT; `emit_dialogue(topic, origin, &[(info id, &Parsed)], ..)` → one module per topic with
`results[<info id>] = function(self) ... end` (run as the speaker; a result's own locals live
in a `local v = {..}` table). A module is `return function(mw) ... return mw.script({ id, vars,
activation, tick }) end`; `vars` lists the declared locals and types, `activation = "script"` is
set when the script reads `OnActivate`. **No Luau globals**: locals `v.<name>`, GLOB globals
`G.<name>` (`mw.globals`), unknown-type globals `mw.get_global/set_global`, `id.var` and speaker
locals `mw.get_var/set_var(target, name)`, undeclared names read `0` and writes go through
`mw.discard(x)` (calls inside `x` still run). Names that aren't Luau identifiers or are keywords
become `t["name"]`. Calls: reference functions are handle methods (`self:get_pos("z")`,
`ref("id"):disable()`), the rest `mw.<snake_case>(self, ...)`; a void function used as a value reads
`(call or 0)`; `Arg::Missing` reads `0` (numeric slot) or `""`. Every statement ends `-- L<n>`.
Inert texts: scripts become a stub module (`inert = "<first error>"`, empty `tick`, variables
kept so `id.var` reads see zeros); inert results are listed in the topic module's `inert` table;
both are listed in the bake report. `EMITTER_VERSION` = 1.

**Numeric semantics** (open question 1, still pending the oracle): expressions evaluate in
floating point (the original's documented float evaluation), so `/` between integers is float
division and comparisons compare values; only assignment converts (`mw.short`/`mw.long`: truncate
toward zero, two's-complement wrap, non-finite → 0; `mw.float`: round through `f32`).
`mw_script::oracle_flags` counts the spots a different oracle answer would change: vanilla has
**0 integer divisions** (both operands integer-typed) and **44 comparisons** between a
non-literal integer operand and a `float` (integer literals compare exactly and aren't counted).

**Function table status** (`functions.txt` 6th column, `live` / `stub` / `stub=<n>` / `later`):
63 rows live, 106 stub, 59 later (rows before `{$LIST}` expansion); `Status::neutral()` is what the
runtime returns while a function isn't live (`GetPCRank` −1, `GetHealth` 100, `SayDone` 1).
Luau names are unique `snake_case` (`GetPCCell` → `get_pc_cell`), checked by a test.

**Runtime skeleton** (`luau/mw.luau`, `emit::RUNTIME`; `emit::runtime_functions` generates the
status table): `Runtime.new(host, functions)` returns `mw` with the conversions, globals table,
reference handles (`mw.ref`, cached), `get_var/set_var` over host-provided variable tables,
`mw.script`/`mw.dialogue` constructors, `new_vars`, `Random` and `GetSquareRoot`; every other
function is a host function or a counted stand-in returning its neutral value
(`mw.fallback_calls`). W3 supplies `host.functions` / `host.methods` / `host.vars_of` /
`host.handle`.

**Bake** (`mw_bake::script`): `RecordEnv` (GLOB types, all record IDs + `player`, object → script,
every live script's locals) is the resolution `Env`; its digest is a parameter of every product.
`mwscript.translate` subjects `script/<id>` and `topic/<topic>` store the module, its bytecode
(mlua `Compiler`, level 2), inert texts, call sites, rule hits and oracle counts (postcard);
`mwscript.coverage` (subject `corpus`) sums them. `bevyrrowind bake` prints one `scripts:` line,
the coverage split and the oracle counts; compile errors fail the bake, inert texts don't.
Products hold game text, so they live in the bake store only. Release bake on the owner's machine:
2,840 translate products (1,207 scripts + 1,633 live topics, 15,852 live results: the merged DB has
fewer INFOs than the per-file corpus) in 1.4 s CPU summed over threads, 5.6 s wall cold (mostly
writing 2,841 small products to the store; a batched or packed product would cut it, not needed
for the gate), **0.19 s no-op**. Coverage over the live DB: 25,449 call sites, 57.1 % live.

**Corpus (real install, `cargo test -p mw_script --release --test real_install -- --nocapture`)**:
1,207 live scripts + 17,144 results → 2,922 modules (1,207 + 1,715 per-file topics), **0 inert,
0 compile errors, 0 run errors** (each script's first tick and every result run once in one
sandboxed `sf_script` context with no host); 3.6 MB Luau, 3.5 MB bytecode, 15 MB context memory.
Single-threaded release: translate 136 ms, compile 245 ms, load + run 292 ms (gate ≤ 2 s).

**Coverage** (live call sites only, scripts + results): 26,704 call sites, **14,934 live (55.9 %)**,
5,729 stub, 6,041 later. By family: journal 6,105 live; dialogue 3,546 later (`Goodbye`, `Choice`,
`ForceGreeting`: M7); inventory 3,490 (3,354 live); placement 3,122 (3,106 live); disposition 2,211
stub; ai 1,557 (896 stub, 661 later); faction 807 (91 stub, 716 later); combat 748 (180 stub, 568
later); magic 652 (281 stub, 371 later); ui 596 (540 live); map 543 stub; audio 499 stub; stats
436 (420 stub); activation 426 live; scripts 394 live; menu 297 live; events 258 stub; deaths
247 live; clock 238 live; the rest < 120 each. Live functions the vanilla corpus reaches on a
first tick, i.e. W3's host to-do list by hits: `journal`, `add_item`, `remove_item`,
`get_journal_index`, `disable`, `get_disabled`, `menu_mode`, `on_activate`, `start_script`,
`add_topic`, `enable`, `cell_changed`, `get_item_count`, `get_dead_count`, `get_distance`, then
the long tail (positions, locks, player controls, animation groups, message boxes).

**Handoff to W3**: implement `host` for `Runtime.new` in `mw_script_bevy` against the M5 stores
and framework APIs (live rows of `functions.txt`); mount `RUNTIME` + `runtime_functions` in the
builtin mod (A3); load translate products' bytecode in the shared `mw.legacy` context; instance
tables from `mw.new_vars(def)` stored in the placement overlay; dialogue results stay dormant until
M7 (console can run one). Still open: the oracle fixtures (numeric semantics, `ElseIf` repairs).

## Verification

- **Corpus (real install)**: every vanilla script and dialogue result parses (failures listed by
  rule with counts; target 0 unrecoverable), translates, and compiles under Luau; leniency rule
  counts reported. Asserts counts only: vanilla text and translations are never committed
  (Bethesda content), goldens use our own fixtures.
- **Translation speed**: the whole corpus (scripts + dialogue) translated and compiled ≤ 2 s on
  the reference laptop across cores (bake gate share), single script ≤ 1 ms.
- **Behaviour (fixtures + fake world)**: `OnActivate` once and default suppression, `Activate`
  passthrough, message-box state machine with `GetButtonPressed`, `GetSecondsPassed` timers,
  `short` wrap and `float` rounding, `id.var` access, `StartScript`/`StopScript`, save/restore
  of instance variables, hot reload of one script keeping variables.
- **Black-box oracle**: a handful of fixtures run in OpenMW and ours with the same inputs,
  comparing variable traces (no code read).
- **In game**: Balmora for 5 minutes with all vanilla start scripts and local scripts active:
  0 runtime errors (or each listed with a reason), legacy script time ≤ 0.3 ms per frame.
- **Owner scenes**: MS-4 (shrine of St. Rilm, `Balmora, Temple`), MS-5 (a door with a
  confirmation script), ROADMAP "M5".

## Industry comparison

2026-10-13; "unverified" marks memory not re-read this session.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Legacy script execution | OpenMW: own compiler to bytecode at load + interpreter; compile-all switches report failures (black box); MWSE: extends the original engine's interpreter (unverified) | translator to Luau, cached bake product, one VM | **differ**: same observable lessons (compile all, report, never crash), less machinery |
| Script language → bytecode VM | Skyrim/Fallout 4 Papyrus: source compiled to `.pex` bytecode ahead of time, run by the Papyrus VM | MWScript compiled ahead of time to Luau bytecode | **match** (ahead-of-time compile) |
| Source-to-source translation of a scripting layer at cook time | UE4 Blueprint nativization (Blueprint → C++ at cook; removed in UE5, unverified reason: maintenance and edge-case bugs) | MWScript → Luau, but targeting the same sandboxed VM, not native code, and reversible (source maps) | **match** in shape; avoids nativization's problem (two semantics) because Luau is the only runtime |
| Interop between legacy and modern scripts | OpenMW Lua can read/write mwscript variables of objects (unverified API name); MWSE-Lua shares the original engine's variables | native Luau mods read/write MWScript locals and globals through `@mw/script` | **match** |
| Active-cell scripting | Morrowind/OpenMW: local scripts run in active cells only | same rule, independent of render streaming | **match** |

## Open questions

1. Integer division and comparison quirks between `short`/`float`: settle with the oracle in W1
   and record the rule here.
2. Non-unique explicit references (`"id"->Disable` with several instances): nearest active
   first is our rule; confirm against the oracle on a known vanilla case.
3. Whether to make `AIWander` live through the M3 scaffold in M5 or leave AI entirely to M7.
4. Scripts on objects in the player's inventory (equip scripts, `OnPCEquip`) need inventory
   instances with state; M5 keeps them inert unless the inventory slice needs one.
