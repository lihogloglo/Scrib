# Dev console

Status: core implemented (2026-10-22), no front end yet; app wiring is a bevyrrowind slice (ROADMAP 1.13)
Crates: `sf_console` (new, Bevy-free: registry, variables, binding, completion, history, log),
`sf_console_bevy` (new, thin: resource, messages, run condition, `App` extension)

## Goal

One place to flip any engine switch live, and to run debug commands, without a rebuild or a
restart. First user: every `--off` switch of the Bevyrrowind app becomes a console variable
(`render.shadows off`), so A/B checks happen in one session. Later users: the in-game overlay
(`sf_devui`), Luau snippets, config files, the command line, test scripts.

Non-goals here: the UI (an egui or bevy_ui overlay comes later and only draws the `Log`, edits a
line, and calls `complete`/`History`), saving variables, and cheat or network policy (below).

## Reuse check (CLAUDE.md rule 8)

1. **Bevy 0.20**: no console. `bevy_dev_tools` has FPS and debug overlays and states, no command line.
2. **Crates** (checked 2026-10-22 on crates.io):
   - `bevy_console` 0.19 (MIT, Bevy 0.20): pulls the full `bevy`, `bevy_egui` and `clap`; a
     command is a Bevy system with a `clap` parser, so it can't be tested without an `App` and
     there are no typed variables. Its UI is the part worth having, and that sits on top of a core
     like ours.
   - `bevy_minibuffer` 0.8 (MIT/Apache, Bevy 0.20): Emacs-style prompt built on `bevy_asky`, full
     `bevy`; key-sequence driven, no variables.
   - `bevy_dev_console`: a name reservation (0.0.0, 2024), nothing to use.
   - `egui_console` 0.4 (MIT) is an egui widget: line edit, scrollback, history; no registry.
   None is Bevy-free (ARCHITECTURE §8b rule 1), and none has typed variables, which are
   the first requirement.
3. **Projects**: Mudcrab's console (MIT, `docs/research/animation/10-mudcrab-borrow.md` item 9)
   has the right shape: a native registry filled by each plugin through an `App` extension,
   case-insensitive names, completion, "did you mean", and parsing as pure functions tested
   without UI. We keep that shape and add typed variables. `sf_script` has
   `ScriptContext::eval` (`=console:<id>` chunks), which becomes one command source (`lua <code>`).

So we write our own core (about 1,600 lines, plus tests), and Bevy only in the thin plugin.

## Design

**One namespace** of commands and variables. Names are case-insensitive and stored lowercase:
a letter, then letters, digits, `_`, `.` or `-`. Dotted prefixes group by system
(`render.shadows`, `time.scale`). A refused registration (bad or duplicate name, default outside
the range, an optional parameter before a required one, a rest parameter not last) is an error
from the core and a panic from the `App` extension, because it's a bug in the registering plugin.

**Variables** (Quake/Source cvars, UE console variables): a type (`Bool`, `Int` with range,
`Float` with range, finite only, `Str`, `Choice` of lowercase words), a default, help text. Every
set validates (integers widen to float when set from code), and setting a variable to its current
value is not a change. Reads are by `VarId` (an index), cheap enough to do every frame. Change
tracking: per-variable `generation`, a global `revision`, and `take_changes()` (each changed
variable once, in order of first change). The Bevy plugin turns that into `ConsoleVarChanged`
messages, so an engine system reacts once instead of polling.

**Commands**: name, aliases, help, typed parameters (`Param::new(name, kind)`, `.optional()`,
`Param::rest(name)` for raw trailing text such as code). Arguments are bound and checked before
the handler runs; the handler gets `Invocation { ctx, args, vars, log }` and returns
`Result<(), String>`. `ctx` is a caller-chosen type: `()` in tests, `World` in the plugin (the
console resource is taken out of the world while a command runs).

**Line syntax**: words split on whitespace; `"quoted text"` with `\"` and `\\`; `;` between
commands (outside quotes). A bare variable name prints it; `name value` sets it. Commands run
left to right and the first error stops the line. Errors name the command, the parameter and the
usage; unknown names get up to three suggestions (edit distance ≤ 1 for names up to 4 characters,
else ≤ 2, against the whole name or its last dotted part, plus names containing the typed text).

**Built-ins**: `help [name]` (`?`), `set <variable> <value>`, `toggle <on/off variable>`,
`reset [variable]` (no argument: all), `vars [filter]` (changed ones marked `*`), `find <text>`
(names and help), `history`, `echo <text...>`, `clear` (`cls`).

**Completion** (cursor at the end of the line): first word from command and variable names
(aliases still run but aren't offered); then by parameter kind: names (filtered: any, variables,
on/off variables), `on`/`off`, choices; a variable's values after `set <variable>` or after a bare
variable name. `Completion::apply` is the Tab key: one match is inserted with a trailing space,
several extend to their common prefix.

**History**: bounded, skips blank lines and repeats of the last line, shell-style `older`/`newer`
browsing that restores the unsent draft; `entries`/`load` for saving between runs (the app owns the
file). `submit` (typed by a user) records and echoes; `exec` (files, command line, scripts) doesn't.

**Log**: bounded lines with a level (`Echo`, `Info`, `Warn`, `Error`) and a sequence number that
keeps counting after old lines drop or `clear`, so a front end draws only `since(seq)`.

**Bevy plugin**: `DevConsole` resource; `ConsoleSubmit { line, typed }` messages run in
`PreUpdate` (`ConsoleSystems`), so a switch flipped this frame applies to this frame's `Update`;
`ConsoleVarChanged { id, name, value }` for every change of the frame, from a line or from code;
`console_flag(id)` run condition; `ConsoleAppExt::{add_console_var, add_console_command}` usable
before or after the plugin is added.

## Co-op (net.md)

Console variables are local developer and presentation state: they are not replicated and never
hold gameplay state (rule 4, state apart from presentation). A command that changes gameplay
must issue gameplay commands through the command path (rule 2), not mutate replicated state
from its handler. Commands and lines are plain data (rule 8), so a host can run lines sent by a
client once a policy exists (open question 3).

## Industry comparison

| Technique | Who does it | Us |
|---|---|---|
| Typed variables with default, range and help, a bare name prints, `name value` sets | Quake 3 / Source `ConVar` (min/max), UE `IConsoleVariable` (`r.Shadow.Enable 0`) | **match** |
| Commands registered by each module, not a central list | Source `CON_COMMAND`, UE `FAutoConsoleCommand`, Mudcrab `App` extension | **match** (`ConsoleAppExt`) |
| `;` chaining, quoted arguments | Quake/Source command buffer | **match** |
| `toggle`, `reset`, list changed values | Source `toggle`/`differences`, Quake `reset`/`cvarlist` | **match** (`toggle`, `reset`, `vars` marks `*`) |
| Search names and help | Source `find`, Creation Kit console `help <text>` | **match** (`find`) |
| Tab completion, per-argument | UE (names + per-command), Source (per-command callback), Quake | **match** for names and typed values; no custom per-command callback (gap P3: add when a command needs one, e.g. record ids) |
| "Did you mean" | Mudcrab, shells | **match** |
| Change callbacks / sinks | UE `SetOnChangedCallback`, console variable sinks; Source change callback | **differ**: change *messages* each frame (ECS style, one reader per system) instead of callbacks into engine code |
| Priority of who set a value ("set by": constructor < config < command line < console) | UE `ECVF_SetBy*` | **gap (P2)**: needed once `--off`, a config file and the console all set one variable; decide with the bevyrrowind wiring slice |
| Flags: cheat, saved to config, replicated, read-only | Source `FCVAR_CHEAT`/`ARCHIVE`/`REPLICATED`, UE `ECVF_Cheat`/`ReadOnly` | **gap (P2)**: open questions 2-3 |
| Run a file of lines, command-line `+set` | Quake `exec`, `+set`; UE `ConsoleVariables.ini`, `-ExecCmds` | **gap (P2)**: the core has `exec(line)`; files and argv parsing are app wiring |
| Saved history | UE (saved per project), shells | **partial**: `entries`/`load`; the app writes the file |

## Open questions

1. Front end: egui (`sf_devui`, ui.md "Dev UI") or a small bevy_ui overlay. Either draws `Log::since`,
   edits one line, Tab = `complete().apply()`, Up/Down = `History`.
2. Saved variables (`archive` flag) and a config file read at start.
3. Cheat and co-op policy: which commands a client may run, and whether gameplay-changing
   commands need a `cheat` flag.
4. Luau as a command source: a `lua <code...>` command (rest parameter) calling
   `ScriptContext::eval` for a chosen mod context, once a script slot is reachable from `World`.
