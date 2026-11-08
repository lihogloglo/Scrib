# messagebox: MessageBox formatting slice (mw_ui::message)

Status: DONE (2026-11-08), written without compiling: UNTESTED (owner test pass 2026-11-10).

## Files touched
- NEW bevyrrowind/crates/mw_ui/src/message.rs: `format_message`, `message_box`, `message_box_from_args`,
  `placeholders`; types `MessageArg {Num, Str, Word}`, `Placeholder`, `Message`, `MessageReport`; consts
  `MAX_BUTTONS` (10), `MAX_FIELD` (1024).
- NEW bevyrrowind/crates/mw_ui/src/tests_message.rs (16 tests).
- bevyrrowind/crates/mw_ui/src/lib.rs (`pub mod message;`, test module).
- bevyrrowind/crates/mw_ui/Cargo.toml (description only; no new deps).
- bevyrrowind/docs/specs/ui-adapter.md (line 5 status, MessageBox bullet, "### As built (MessageBox formatting)").
- mw_script NOT edited.

## Source (OpenMW ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f)
components/misc/messageformatparser.cpp 7-19, 26-97; components/interpreter/miscopcodes.hpp 17-160;
components/compiler/lineparser.cpp 145-180, 461-473, 501-519; components/compiler/generator.cpp 436-447;
test vectors apps/openmw_tests/mwscript/testscripts.cpp 133-158 + 633-660 (format), 512-520 + 1031-1043 (issue 8129).

## Test commands for the owner
    bash bevyrrowind/tools/cargo_turn.sh test -p mw_ui --lib tests_message
    bash bevyrrowind/tools/cargo_turn.sh clippy -p mw_ui --all-targets -- -D warnings

## Unverified guesses / risk spots
- Rust `{:.N}` / `{:.Ne}` on f32 rounds exact ties to even (believed since Rust 1.67). Test `rounding_ties_to_even`
  isolates it; if it fails, the formatter needs its own tie rounding.
- std::format semantics taken from the C++20 spec, not run: `{:g}` without precision = 6 digits; `0` flag ignored
  for inf/nan and when left-aligned; `{:a}` = to_chars hex (no 0x; subnormal `0.000002p-126`; rounding with
  carry `2.0p+0`). Hex subnormal and rounding are the least sure (tests `hex_notation`).
- NaN sign: prints `nan`/`-nan` (MSVC to_chars may print `-nan(ind)`).
- Clippy: pedantic `many_single_char_names` avoided by renaming; `cast_possible_truncation` allowed on the two
  f64->f32 / f64->i32 casts with comments. `manual_range_contains` handled.
- Format inline args `{a:.p$}`, `{frac:0wanted$x}`, `{value:.prec$e}` with `prec = ...` assumed fine.

## For the lowering slice (mw_script, later)
- mw_script's `interp.rs` `format_message` differs from OpenMW: keeps trailing `%` and unknown `%x` as written,
  no flags/width/`%s`/`%e`/`%a`, f64 not f32, and every string after the text becomes a button. OpenMW: the first
  `placeholders(fmt).len()` args are values (a quoted string can be a `%s` value), buttons = count of quoted args
  after them, labels from the front incl. unquoted words (issue 8129). Lowering should call
  `mw_ui::message::message_box_from_args`, passing bare words as `MessageArg::Word` (parser currently makes them
  Expr) and `%s` positions as text.

## For the orchestrator
- CHANGELOG: `2026-11-08 mw_ui::message: MessageBox formatting ported from OpenMW (std::format flags/width/precision, s d i f e g a, f32 values, issue-8129 button rule), 16 tests; written without compiling, UNTESTED (owner pass 2026-11-10).`
- THIRD_PARTY_NOTICES (extend the OpenMW entry): `bevyrrowind/crates/mw_ui/src/message.rs: ported from OpenMW (GPL-3.0) commit ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f, components/misc/messageformatparser.cpp, components/interpreter/miscopcodes.hpp, components/compiler/lineparser.cpp; test vectors from apps/openmw_tests/mwscript/testscripts.cpp.`
- test-pass-2026-11-10.md: the two commands above.
- SYSTEMS.md: UI adapter row gains `mw_ui::message`.
