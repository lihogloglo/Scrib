# Dialogue authoring: what the Rust Yarn Spinner port can do (DQ-33)

Read 2026-11-26. Rust port: github.com/YarnSpinnerTool/YarnSpinner-Rust, `main` at `79e513eb` (2026-11-18), cloned
shallow (paths below are in that tree). Upstream C#: YarnSpinner `66f9f23`, tag v3.3.2 (2026-11-26). Nothing was run.

## Summary

1. **Recommendation: (c) our own parser for a Yarn 3 subset, feeding `DialogueScene`; do not adopt the port's runtime.**
2. The port is on Bevy 0.20 (`bevy_yarnspinner` 0.10.0, 2026-11-10, MIT OR Apache-2.0), so Bevy is not the issue.
3. The port implements the **Yarn Spinner 2.5 language only**. Its parser has 26 rules; upstream 3.x adds `when:`
   headers, `=>` line groups, `once`, `detour`/`return`, enums with parser rules, smart variables, saliency. The port
   has none of them. That fails the node-group check, so (b) is out.
4. (a) compiles only the 2.5 subset, on `antlr-rust =0.3.0-beta` (unmaintained, pinned) with a 337 KB generated
   parser/lexer. The 3.x work (open draft PR #284, 2026-07-19, untouched since) is not merged.
5. `VariableStorage` is usable as a seam but is a map-shaped, `&mut`, `Send + Sync` trait that panics on undeclared
   variables; a read-through over our `Facts` is possible only with shims.
6. The VM has no public save/restore; random IDs and `random()` use the OS RNG, not a seed.
7. Upstream tests (86 `.yarn`, 35 `.testplan`, MIT) are usable as golden fixtures for (c).
8. Main risk of (c): the Yarn lexer (modes, indentation, markup, `{expr}` interpolation) is the hard part; mitigate by
   running upstream fixtures from day one and keeping the subset small.
9. DQ-34 must specify our own line-ID scheme (the port's implicit IDs are position-dependent), string tables and the
   `when:` to rule mapping (section 9).
10. Single deciding check if the owner doubts (c): import the upstream `Tests/*.yarn` set with the subset parser and
    count how many parse; target all files that use only the subset.

## 1. Versions and health

- Crates (workspace members, `Cargo.toml:4-14`): `bevy_yarnspinner`, `yarnspinner` (facade), `yarnspinner_compiler`,
  `yarnspinner_runtime`, `yarnspinner_core`, `yarnspinner_internal_shared`, `yarnspinner_codegen`,
  `bevy_yarnspinner_example_dialogue_view`. All on crates.io at 0.10.0 (internal_shared 0.1.0, codegen 0.1.1).
- Release dates (crates.io API, fetched today): 0.10.0 2026-11-10, 0.9.0 2026-08-07, 0.8.0 2026-03-02, 0.7.0
  2026-01-29, 0.6.x 2025-10-14. Four minor releases in 12 months; 77 commits since 2025-11-26 (GitHub API, one page).
- Bevy per release (tag Cargo.toml): v0.7.0 and v0.8.0 = Bevy 0.18, v0.9.0 = 0.19, 0.10.0 = `bevy ^0.20`
  (`crates/bevy_plugin/Cargo.toml`; crates.io dependency list for 0.10.0). `readme.md:158` table: Bevy 0.20 = crate
  0.10 = **Yarn Spinner 2.5.0**. They follow each Bevy release within weeks.
- Open: 31 issues, 3 PRs (GitHub search). Licence: `LICENSE-MIT` + `LICENSE-APACHE`; upstream YarnSpinner
  `LICENSE.md` is MIT, so the test data (section 7) is MIT. Both are GPL-3.0-or-later compatible.
- Maintainer note: upstream docs call the Rust port a Yarn Labs project, not fully supported (prior note, not rechecked).

## 2. Crate split and weight

- Bevy-free: `yarnspinner_core`, `_compiler`, `_runtime` all have `bevy` as an optional feature, off by default
  (`compiler/Cargo.toml:14-16`, `core/Cargo.toml:15-19`, `runtime/Cargo.toml:21-37`). Exception: runtime depends
  unconditionally on `bevy_platform 0.20.0` (a small std-shim crate, `runtime/Cargo.toml:49`), so it is not
  Bevy-version-free. `core` is `no_std`-leaning (clippy `std_instead_of_core`), PR #295 (2026-11-15).
- The compiler stands alone: `Compiler::new().add_file(File{file_name, source}).compile() -> Result<Compilation>`
  (`compiler.rs:50-101`). `Compilation` has `program`, `string_table: HashMap<LineId, StringInfo>`, `declarations`
  and diagnostics (`output.rs:31-55`). `StringInfo { text, node_name, line_number, file_name, is_implicit_tag,
  metadata }` (`output/string_info.rs`). No runtime needed.
- Direct dependencies. Compiler: `antlr-rust =0.3.0-beta`, `better_any =0.2.0`, `regex`, `annotate-snippets`, `rand`
  0.10, `crc32fast`, `globset`, `walkdir`, plus core and internal_shared. Core: `prost` (protobuf types, derive only),
  `rand`, `hashbrown`, `csv-core`, `thiserror`, `variadics_please`. Runtime: `icu_plurals`, `icu_locale_core`,
  `fixed_decimal`, `unicode-normalization`, `unicode-segmentation`, `regex`, `once_cell`, `log`, `bevy_platform`.
- Generated ANTLR code: `parser/generated/yarnspinnerparser.rs` 284 KB, `yarnspinnerlexer.rs` 53 KB (hand-patched,
  `generated/readme.md`). `antlr-rust` is the unmaintained crate; PR #281 (open since 2026-06-02) swaps it for
  `antlr4rust` 0.5.2. Compile-time cost not measured (no builds allowed).

## 3. Language coverage versus Yarn Spinner 3 (the node-group check)

Evidence: the Rust parser's rule list (`compiler/src/parser/generated/yarnspinnerparser.rs:126-176`, 26 rules, last
is `jump_statement`), lexer tokens (`YarnSpinnerLexer.tokens`: no `ONCE`, `DETOUR`, `RETURN`, `WHEN`,
`LINE_GROUP_ARROW`), and upstream grammar `YarnSpinner.Compiler/Grammars/YarnSpinnerParser.g4` (rules
`when_header`, `line_group_statement`, `once_statement`, `return_statement`, `enum_statement`, `structured_command`).

| Feature (Yarn 3) | Rust port | Evidence |
|---|---|---|
| Node groups (`when:` headers) | absent | no rule/token; issue #273 "Does bevy_yarnspinner support Node Group?": error "More than one node is named", maintainer 2026-01-29 "We didn't update to v3 yet" |
| Saliency strategies | absent | no `ContentSaliency` or `AddSaliencyCandidate` opcode; `OpCode` ends at `RunNode = 16` (`core/src/generated/yarn.rs:113-178`); PR #284 body: "[ ] saliency candidates / node groups" |
| Line groups (`=>`) | absent | PR #284 "[ ] line groups"; no token |
| `once` | absent in main | PR #284 ticks it (draft, unmerged) |
| `detour` / `return` | absent in main | PR #284 ticks them (draft, unmerged) |
| Smart variables | absent | PR #284 "[ ] smart variables" |
| Enums | partial at best | lexer has `COMMAND_ENUM`/`CASE`/`ENDENUM` tokens but the parser has no enum rule and `Type` is `Any/Boolean/Function/Number/String` (`core/src/types/type.rs:33-40`) |
| Shadow lines | absent | no hit for `shadow` in non-generated sources |

PR #284 (draft by a contributor, +14760/-8626, 15 commits, last update 2026-07-19) and tracking issue #278 (migration
checklist) are the only 3.x work; both open. Upstream already ships 3.3.2.

## 4. `VariableStorage`

`runtime/src/variable_storage.rs:20-45`:

```rust
pub trait VariableStorage: Debug + Send + Sync {
    fn clone_shallow(&self) -> Box<dyn VariableStorage>;
    fn set(&mut self, name: String, value: YarnValue) -> Result<()>;
    fn get(&self, name: &str) -> Result<YarnValue>;
    fn contains(&self, name: &str) -> bool { self.get(name).is_ok() }
    fn extend(&mut self, values: HashMap<String, YarnValue>) -> Result<()>;
    fn variables(&self) -> HashMap<String, YarnValue>;
    fn clear(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
```

- Sync, not async. Values: `YarnValue::{Number(f32), String(String), Boolean(bool)}` (`core/src/yarn_value.rs:23-31`);
  our `Int(i64)`/`Id`/`Ref` facts would be lossy (f32).
- `clone_shallow` must share state: the VM registers `visited` and `visited_count` functions built from a clone
  (`dialogue.rs:107-125`), so the storage must be `Arc`-shared (the memory one is `Arc<RwLock<HashMap>>`).
- Read-through over our `Facts` is possible for `get` but awkward: `variables()` and `extend()` must return or take
  whole maps, names must start with `$` (`MemoryVariableStorage::validate_name`), and visit counts are stored as
  ordinary variables in the same storage (`Library::generate_unique_visited_variable_for_node`). Our `get` takes a
  `QueryCtx`; this trait has none, so per-query context needs interior state.
- Undeclared variable: `VariableNotFound` leads to the program's `initial_values`; if absent the VM **panics**
  (`virtual_machine.rs:515-540`). Compiler-side, declarations come from `<<declare>>` or `Compiler::declare_variable`.
  Writes go through `set` on every `<<set>>`; a type mismatch is caught at compile time, not in `set`.

## 5. Runtime as a library

- `Dialogue` (`dialogue.rs:17`) is usable without Bevy: `continue_()` returns `Vec<DialogueEvent>` (`Line`,
  `Options`, `Command`, `NodeStart`, `NodeComplete`, `LineHints`, `DialogueComplete`; `events.rs:23-49`).
  `continue_with_world(&mut World)` exists only with the `bevy` feature.
- Determinism: `random`, `random_range`, `dice` use `SmallRng::try_from_rng(&mut SysRng)` per call
  (`core/src/library.rs:84-98`): OS entropy, no seed. They are library functions, so a host can replace them with a
  tick-seeded version via `library_mut().add_function`. There is no saliency or option shuffling in this port.
  The compiler also tags untagged lines with OS-random IDs (`listeners/untagged_line_listener.rs:42-50`).
- Save/restore: the VM state (`State { program_counter, current_options, stack }`, `virtual_machine/state.rs`) derives
  serde under a feature but is `pub(crate)`; `Dialogue` exposes no snapshot API. Visit counts survive only because
  they live in `VariableStorage`. We could save node plus a re-run, not an exact position, without patching.
- Commands: `<<name args>>` become `DialogueEvent::Command(Command { name, ... })` handed to the host
  (`command.rs:18`); functions are Rust closures in a `Library` (`yarn_fn` module, "emulates Dictionary<string,
  Delegate>"), with Bevy-system-style parameter wrapping.

## 6. Line IDs and localisation

- Explicit `#line:xyz` hashtags (prefix `line:`). Untagged lines: the compile gets an implicit ID
  `line:<crc32(file_name + node_name + string_table_len)>` (`string_table_manager.rs:38-57`). It depends on the
  line's position in the table, so inserting a line earlier changes later IDs. Test: `string_table_generator_visitor.rs`
  asserts `line:1337986088` etc. (`is_implicit_tag: true`).
- Tagger: `add_tags_to_lines` (deprecated) rewrites source text with random `line:<0..0xFFFFFF>` tags from the OS RNG,
  avoiding collisions with existing tags (`untagged_line_listener.rs`, `compiler/add_tags_to_lines.rs:30-60`). The
  compiler itself never writes files; the tagger returns text for the host to save.
- Line metadata: every other hashtag on the line is stored in `StringInfo.metadata` (`string_table_generator_visitor.rs:119-130`).
- String table CSV (bevy plugin, `localization/strings_file/asset.rs`): per-line record with `lock` (first 8 hex chars
  of SHA-256 of the base text, to flag stale translations), `comment` (translator comment built from metadata), file,
  node, line number. Locale is a `Language` set on the `Dialogue` (`set_language_code`) and a `TextProvider` trait
  (`get_text(&LineId)`, `set_language`, `text_provider.rs:18-34`). Voice-over: `audio_assets` feature of the Bevy
  plugin only; none in core crates (not found after searching `runtime/` and `compiler/`).

## 7. Own-parser option (c)

- Size: upstream grammar files total 22.4 KB (`YarnSpinnerLexer.g4` 16.7 KB, `YarnSpinnerParser.g4` 5.7 KB, v3.3.2);
  the parser has about 34 rules, the port's 2.5-era one 26. The lexer is the cost (modes for headers, body, commands,
  expressions, hashtags, plus an indent-aware wrapper `parser/indent_aware_lexer.rs`).
- Minimal RPG subset: nodes with headers (`title`, `tags`, `when`) and `---`/`===`; lines with `Speaker: text`;
  `#line:` and other hashtags; `->` options with `<<if>>` conditions and nested bodies; `<<jump>>`; `<<set>>`,
  `<<declare>>`, `<<if/elseif/else/endif>>`; free commands `<<cmd args>>`; `{expr}` interpolation; comments `//`.
  Expressions: literals, `$var`, `and/or/not`, comparison, `+ - * /`, function calls. Leave out initially: markup
  attributes, `<<enum>>`, smart variables, saliency, `detour`.
- Fixtures: upstream `Tests/` has 86 `.yarn` and 35 `.testplan` files (MIT, `LICENSE.md`), about 7 MB total
  incl. non-Yarn data; testplan grammar `YarnSpinner.Tests/TestPlan/YarnSpinnerTestPlan.g4` (1.3 KB). The Rust port's
  own tests read the same files through its submodule `third-party/YarnSpinner/Tests`
  (`crates/yarnspinner/tests/test_base/paths.rs:18`); the submodule was not checked out in the clone.

## 8. How our session differs

Read `rpg.md:188-257`. Mapping, in the new rule world (`dialogue-rules.md`):
- Yarn node = our scene/response body; a topic or greeting = a node group chosen by rules, not by name.
- A line = `LineRef` rendered by `DialogueRules::render`; speaker comes from `speaker.*` facts.
- `->` options = `offer_choices`/`Choosing`, with `choose(value)` re-asking the topic (filter input), not jumping.
- `<<jump>>` = no counterpart: our flow is rule-selected per `ask(topic)`; inner jumps need a scene runner.
- `<<set>>`/`<<if>>` = writes via `RpgCommand`/`DialogueMemory` and criteria over `Facts`; Yarn has no `Missing`.
- Commands = host result scripts (`RunResult`). No counterpart in Yarn: topic links, `available_topics`, `Conversations`
  claim table, `history`, `Goodbye` sub-state, `event.choice`.

## 9. Recommendation: (c)

Reasons:
1. The port passes none of the Yarn 3 checks that matter (`when:`, `=>`, `once`, `detour`), and its 3.x port is a
   stalled draft, so (a) or (b) would freeze us on Yarn 2.5 and an unmaintained ANTLR crate for 337 KB of generated code.
2. We already own what the runtime would add (selection, state, saves, net). The port's seam does not fit it
   (f32-only values, whole-map methods, panic on undeclared variable, no VM snapshot, OS-RNG), so (b) fights our design.

Main risk: lexer fidelity. Mitigation: keep the subset small and gate on upstream fixtures.

DQ-34 must specify:
- `DialogueScene` as the model, serde, with `when:` mapped to a rule (`Concept` plus criteria), not to Yarn saliency.
- Our own stable line IDs: authored `#line:` or a deterministic ID from (scene id, node, ordinal), never positional
  hashes; a tagger that outputs a patch, run at bake, with a seeded RNG.
- String tables per locale: columns id, base text, lock hash, translator comment, speaker, voice key; stale-lock rule.
- Importer scope table (subset above) and an "unsupported Yarn construct" error list.
- Seeded randomness from the host tick; saving by node id plus variable facts, so no VM snapshot is needed.

## Not verified

- Compile time and size of the Yarn crates (no builds allowed).
- Whether upstream test files use only the subset; counted files, not parsed.
- `Dialogue: Send + Sync` and the `Library` closure bounds (not read).
- Whether PR #284 or #281 have unpublished local progress (only GitHub metadata read).
- Prior note claims (`docs/research/gameplay-2026-11-25/dialogue-quests.md`): confirmed `0.10.0`/Bevy 0.20/MIT OR
  Apache-2.0, README table 2.x, "no evidence of node groups" (now confirmed absent in the grammar); `VariableStorage`
  "2.x API" confirmed. Nothing contradicted.
