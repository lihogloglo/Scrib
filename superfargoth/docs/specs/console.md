# Dev console

Status: core (2026-10-22); front end, set-by priority, click-select and inspector built 2026-10-23
("As built"); registering the app's switches is a bevyrrowind slice (ROADMAP 1.13)
Crates: `sf_console` (Bevy-free: registry, variables, binding, completion, history, log, input line),
`sf_console_bevy` (thin: resource, messages, run conditions, `App` extension; `overlay` feature:
the bevy_ui front end, click-select, inspector)

<!-- agent-summary -->
> **Agent summary (2026-10-26).** Status: core, bevy_ui overlay, click-select/inspector and MWScript fallback built; ROADMAP 1.13 [~]; CO-1 partial (COC interiors only, no paste, suggestion keys). Left: `set x to y`, exterior `coc`, other `--off` switches as variables. Not built: saved variables/config file, Luau command, cheat policy ("Open questions").
> Crates: `sf_console` (Bevy-free: registry, vars, `Prompt`, history), `sf_console_bevy` (`overlay` feature). Decisions: one namespace, names lowercase ("Design"); `SetBy` priority, a lower source cannot override ("As built"); own `Prompt`, not Bevy `TextInput`, so paste/IME is gap P3; unclaimed lines go to a fallback (MWScript).
> Read next: "As built" first (newest); "Design" to register a variable or command; "Open questions" for policy.
<!-- /agent-summary -->

## Goal

One place to flip any engine switch live, and to run debug commands, without a rebuild or a
restart. First user: every `--off` switch of the Bevyrrowind app becomes a console variable
(`render.shadows off`), so A/B checks happen in one session: SSAO, contact shadows, lamp filter,
fog history/jitter/froxels and the rest (owner 2026-10-21; owner checks SH-16 and SH-19 wait on it). Later users: the in-game overlay
(`sf_devui`), Luau snippets, config files, the command line, test scripts.

Non-goals here: saving variables, and cheat or network policy (below). The front end only draws the
`Log`, edits a `Prompt` and calls `complete`/`History`: another one (egui in `sf_devui`) can reuse all of it.

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

**Built-ins**: `help [name or text]` (`?`; a text that isn't a name searches names and help, as the
Creation Kit's `help <text>`), `set <variable> <value>`, `toggle <on/off variable>`,
`reset [variable]` (no argument: all), `vars [filter]` (`list`; changed ones marked `*`, with who
set them), `history`, `echo <text...>`, `clear` (`cls`). `find` belongs to the overlay (entities).

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
| Priority of who set a value ("set by") | UE `ECVF_SetBy*` (… < ini < command line < code < console) | **match**: default < config < command line < code < console |
| Click-select an object, commands on the selection | Morrowind/Bethesda console (`prid`, `getpos`, `setpos`, `disable`) | **match** |
| Selection highlight | CS/CK selection box; UE editor outline | **partial**: gizmo box, no silhouette (As built) |
| Entity inspector | UE `displayall`/Details, `bevy-inspector-egui` | **match** read-only; editing is `sf_devui`'s |
| Flags: cheat, saved to config, replicated, read-only | Source `FCVAR_CHEAT`/`ARCHIVE`/`REPLICATED`, UE `ECVF_Cheat`/`ReadOnly` | **gap (P2)**: open questions 2-3 |
| Run a file of lines, command-line `+set` | Quake `exec`, `+set`; UE `ConsoleVariables.ini`, `-ExecCmds` | **gap (P2)**: the core has `exec(line)`; files and argv parsing are app wiring |
| Saved history | UE (saved per project), shells | **partial**: `entries`/`load`; the app writes the file |

## Open questions

1. ~~Front end~~: a bevy_ui overlay (2026-10-23, As built); `sf_devui` (egui) can add a second one.
2. Saved variables (`archive` flag) and a config file read at start.
3. Cheat and co-op policy: which commands a client may run, and whether gameplay-changing
   commands need a `cheat` flag.
4. Luau as a command source: a `lua <code...>` command (rest parameter) calling
   `ScriptContext::eval` for a chosen mod context, once a script slot is reachable from `World`.

## As built (2026-10-23)

**Core.** `SetBy` (default < config < command line < code < console): a lower source can't change a
value a higher one set (`SetOutcome::Outranked`, logged as "kept"); `reset` hands the variable back.
`Vars::set` is `Code`; typed lines are `Console`; `Console::exec_as(ctx, line, source)` for config
files and argv. `Prompt` is the input line (cursor, word moves, history, Tab on the word before the
cursor), driven by key-agnostic `Edit`s, so editing is tested without Bevy.

**Fallback (2026-10-25).** `Console::set_fallback(label, |Unclaimed { ctx, text, vars, log }| ..)` runs a
command whose first word no command or variable claims (registered names always win; errors log as
`label: message`); `set_fallback_completion(|first_word| ..)` adds first-word completions. Bevyrrowind
runs MWScript through it, as Bethesda consoles run script lines (`player->AddItem gold_001 5`).

**Plugin.** `ConsoleSubmit::from(source, line)`; `app.set_console_var(id, value, source)`;
`console_var_changed(id)` (true on first run, so the starting value applies once) and
`console_closed` run conditions; `ConsoleOpen` resource. bevyrrowind's `--off` switches:

```rust
let ssao = app.add_console_var(VarSpec::bool("render.ssao", true).help("Screen-space AO"));
app.set_console_var(ssao, !off.contains("ssao"), SetBy::CommandLine);
app.add_systems(Update, apply_ssao.run_if(console_var_changed(ssao))); // reads vars().flag(ssao)
```

**Input line: own `Prompt`, not Bevy 0.20's `TextInput`** (`bevy_ui_widgets`, over parley's
`PlainEditor`). That widget applies edits in `PostUpdate` (a typed line would run a frame late),
takes Up/Down for caret moves (history needs them), and needs focus dispatch, a11y and fonts, so it
can't run in the headless tests. It has IME, clipboard, selection and a blinking caret: **gap (P3)**,
use it as the view over `Prompt` when someone needs paste or IME in the console.

**Overlay** (`overlay` feature, `ConsoleOverlayPlugin`; bevy_ui only, no `sf_ui_css`): `ConsoleKeys`
(Backquote, F1; physical keys), `ConsoleStyle` (font, height, colors). Enter, Tab (+ popup of
candidates), Up/Down, Ctrl+arrows/Backspace/W/L, PageUp/PageDown and wheel, Escape clears then
closes. Input runs in `PreUpdate` `ConsoleInputSystems` (after `InputSystems`, before
`ConsoleSystems`), so a typed switch applies to the same frame's `Update`. While open: `ConsoleOpen`
is true, `ButtonInput<KeyCode/MouseButton>` and mouse motion/scroll are cleared (systems reading
`KeyboardInput` messages must gate on `console_closed`; order `bevy_enhanced_input` after
`ConsoleInputSystems`), and the primary window's cursor is freed and restored on close.

**Pick and inspect.** A left click outside the panels writes `ConsolePick { ray }` from the
`ConsoleCamera` (else the highest-order active 3D camera); bevy_picking's `MeshRayCast` runs once
(no backend raycasting every frame), visible meshes only; the hit selects its nearest `PickRoot`
ancestor (the app marks placed objects) or itself; a miss deselects. Commands: `prid <name>`
(exact `Name`, `PickRoot` first), `find <text>`, `getpos`, `setpos <x> <y> <z>` (world, parent
kept; computed from `Transform`s so `setpos …; getpos` agrees on one line), `moveto` (camera
2.5 radii from the bounds, also `ConsoleMoveTo` for rigs), `disable`/`enable` (`Visibility`),
`delete`. The inspector panel: name, id, transform (+ world), visibility, parent chain, child count,
game rows from `app.add_inspector_rows(|world, e, rows| …)`, then the other components through the
type registry, one line each (compact reflect format, read-only).

**Outline: a gizmo box.** Bevy 0.20 has no silhouette outline (checked: `bevy_pbr` has `Wireframe`,
needs `POLYGON_MODE_LINE` and draws every triangle; `bevy_gizmos` has per-mesh `ShowAabbGizmo`).
`bevy_mod_outline` (MIT/Apache) is at 0.13 for Bevy 0.19, no 0.20 branch (2026-10-23). So: one
gizmo box per selection, in its frame, around all its meshes (the Construction Set's selection box).
Swap to `bevy_mod_outline` when it supports 0.20.

**Tests** (MinimalPlugins): set-by + same-frame apply; keys through completion, submit, history,
popup, input blocking; selection commands and inspector rows; pick ray (nearest, hidden skipped,
root, miss). `examples/console_overlay.rs --autotest` renders under Xvfb + lavapipe and checks a
scripted session (screenshot checked by eye, 2026-10-23). Limits: one window; no caret blink, paste
or mouse selection (P3 above); `find`/`prid` match `Name` only (record ids come with the game's hook).

**No-clip (2026-10-25, `sf_action_bevy::NoClip`):** a body with it steps in `MoverMode::Fly` against a never-hitting probe,
climbing along the host-fed `look`; the game wires it to `TCL` / `ToggleCollision` and `COC` / `CenterOnCell` (door travel).
