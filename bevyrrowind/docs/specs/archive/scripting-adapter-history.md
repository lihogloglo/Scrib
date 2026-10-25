# scripting-adapter history

Dated "As built" blocks moved verbatim from [scripting-adapter.md](../scripting-adapter.md) (doc cleanup S10a, 2026-10-26).

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


