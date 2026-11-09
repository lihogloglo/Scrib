# messagebox-lowering: MessageBox lowering (mw_script -> host -> mw_ui::message)

Status: DONE (2026-11-08), written without compiling: UNTESTED (owner test pass 2026-11-10).
Scope: bevyrrowind/crates/mw_script/ (parser, interp, tests), mw_rpg/src/runner.rs (+ runner_tests.rs),
mw_rpg/Cargo.toml. mw_scripts_bevy: nothing to change (it has no `Host` impl / MessageBox hook). mw_ui read-only.

## Files touched
- mw_script/src/ast.rs: new `Arg::Word(String)` (MessageBox bare word as written); `Arg::Str` doc.
- mw_script/src/parser.rs: `parse_call` dispatches `MessageBox` to new `parse_message_box` (format text,
  one value per placeholder: `%s` = quoted string or bare word as text, others `unary` expression; then every
  quoted string / bare word to line end as `Arg::Str`/`Arg::Word`; missing values -> one `MissingArgument` +
  `Arg::Missing`); helpers `skip_comma`, `take_word` (glued Word/Int/Float/`-`/`.` run: `-2`, `.todd`);
  free fn `pub(crate) message_placeholders(&str) -> Vec<bool>` (port of the mw_ui/OpenMW scan, true = `%s`).
- mw_script/src/interp.rs: new `pub enum MessageArg { Num(f64), Str(String), Word(String) }`;
  `Host::message_box(&mut self, format: &str, args: &[MessageArg])`; MessageBox builtin evaluates Expr args to
  `Num` and passes everything raw; `format_message` + `format_g` DELETED; `use std::fmt::Write` removed.
- mw_script/src/lib.rs: re-export `MessageArg` (`pub use interp::{..., MessageArg, ...}`).
- mw_script/src/emit.rs: `Arg::Str(s) | Arg::Word(s) => lua_string(s)`.
- mw_script/src/interp_tests.rs: Recorder logs `message_box {format:?} {args:?}`; replaced
  `message_box_formats_and_buttons` + `message_format_rules` with `message_box_hands_values_and_buttons_to_the_host`,
  `message_box_keeps_bare_words_as_words`, `message_box_text_placeholders_take_strings_and_words`.
- mw_script/src/tests.rs: new `message_box_args_split_by_placeholders` (placeholder scan + parsed args + MissingArgument).
- mw_script/tests/real_install.rs: NeutralHost `message_box` signature.
- mw_rpg/src/runner.rs: `message_box` converts to `mw_ui::message::MessageArg` and calls `message_box_from_args`;
  `ResultEffect::MessageBox { text, buttons }` unchanged (result.rs not touched).
- mw_rpg/src/runner_tests.rs: new `message_box_formats_as_openmw`; `ui_session_and_sound_effects` still expects "25 drakes".
- mw_rpg/Cargo.toml: `mw_ui.workspace = true` (root workspace entry already exists, line 271).
- bevyrrowind/docs/specs/ui-adapter.md: the "Not done" bullet under "As built (MessageBox formatting)" -> one "Lowering" line.

## Owner test commands
    bash bevyrrowind/tools/cargo_turn.sh test -p mw_script --lib
    bash bevyrrowind/tools/cargo_turn.sh test -p mw_script --test emit
    bash bevyrrowind/tools/cargo_turn.sh test -p mw_rpg --lib runner
    bash bevyrrowind/tools/cargo_turn.sh clippy -p mw_script -p mw_rpg --all-targets -- -D warnings
    bash bevyrrowind/tools/cargo_turn.sh test -p mw_script --test real_install -- --ignored   (if gated; needs install; inert must stay 0)
    cargo hakari generate && cargo hakari verify   (mw_rpg gained mw_ui -> sf_ui deps; both in final-excludes, likely no diff)

## Unverified guesses / risk spots
- Lexer: `3.14pi` lexes as Word("3") + StrayCharacter('.') + Word("14pi") glued -> `take_word` gives "314pi"
  (OpenMW: "3.14pi"). Not tested; fixing needs a lexer change (keep `.` inside a word started by digits).
- Numbers in bare words print their value, not the source (`2.50` -> "2.5", `.5` -> "0.5", `007` -> "7").
- Bare words after the values now never resolve as variables (OpenMW: they are names). Fewer UndeclaredVariable
  warnings on vanilla; real_install counts asserted there are script/result totals only.
- Keywords after MessageBox values (`set`, `endif`...) are swallowed as button words up to the line end (as OpenMW).
- Number values still parse with `unary` (one term, as before): `MessageBox "%g" a + b` -> value a, then the
  `+` stops the args (TrailingTokens). Same as the old parser.
- `Arg::Missing` for missing values is skipped by the interpreter (mw_ui reads them 0/"" and counts them).
- Luau emitter: `Arg::Word` emits as a string, so the parked Luau `mw.message_box` cannot tell quoted from bare.
- runner drops `Message::report` (no logger there); text defines (`^PCName`) still unexpanded (mw_ui docs: caller's job; not done anywhere that I checked).
- Closure-free code; `Function::name == "MessageBox"` string compare per call parse.

## For the orchestrator
- CHANGELOG: `2026-11-08 MessageBox lowering: mw_script parser splits MessageBox args by the format's placeholders (%s text, bare words kept as Arg::Word), Host::message_box gets (fmt, &[MessageArg]) raw, mw_rpg runner formats with mw_ui::message::message_box_from_args (OpenMW formatter + issue-8129 buttons); mw_script's format_message deleted; 5 new tests; written without compiling, UNTESTED (owner pass 2026-11-10).`
- Root Cargo.toml: no change (`mw_ui` workspace entry exists). hakari: run `cargo hakari generate`/`verify` once (likely no diff).
- mw_rpg/src/lib.rs: no re-export needed.
- test-pass-2026-11-10.md: the owner commands above.
- scripting-adapter.md semantics 5 still says "`printf`-style"; could point to ui-adapter.md "As built (MessageBox formatting)" (not edited: outside the one-line brief).
