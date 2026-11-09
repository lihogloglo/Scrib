# Session: book-markup
Scope: new crate bevyrrowind/crates/mw_ui (module `book`: Morrowind BOOK markup -> sf_ui rich_text IR), plus one line in
root Cargo.toml [workspace.dependencies] (`mw_ui`, after mw_stats) and `"mw_ui"` in .config/hakari.toml final-excludes.
Status: DONE (code written, 2026-11-08), NOT COMPILED (no cargo on this machine). Nothing committed.

## Files
- bevyrrowind/crates/mw_ui/Cargo.toml (deps sf_ui, sf_vfs; dev mw_install, tes3[esp])
- bevyrrowind/crates/mw_ui/src/lib.rs, src/book.rs (parser, docs = mapping + OpenMW provenance), src/tests_book.rs (27 tests)
- bevyrrowind/crates/mw_ui/tests/real_install.rs (`#[ignore = "real install"]` survey of every BOOK record)
- Cargo.toml line 269, .config/hakari.toml line 95

## Owner to do
- `cargo hakari generate && cargo hakari manage-deps`, then `cargo test -p mw_ui`, clippy, and
  `cargo test -p mw_ui --test real_install -- --ignored --nocapture`.
- THIRD_PARTY_NOTICES.md entry "OpenMW (book markup)": formatting.cpp, resourcehelpers.cpp, fontloader.cpp @ ca5e2576.

## Unverified (not compiled)
- tes3 `Plugin::load_bytes_filtered` closure form copied from mw_script tests; whether filtering out all but TES3/BOOK is OK.
- `TES3Object::Book(x)` + `x.text`/`x.id` (read from tes3 libs/esp/src/types/book.rs @ e61ad140).
- OpenMW details not read in source: VFS::Path::Normalized::changeExtension, MyGUI parseValue trailing-char rule.
