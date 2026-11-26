# Dialogue authoring: scenes, line IDs, string tables, Yarn import (DQ-34)

Status: **spec (DQ-34, 2026-11-26)**, nothing built. Roadmap: `docs/roadmaps/dialogue-quests.md` stage 6 (DQ-35..DQ-38),
decisions 8-10. Research: `superfargoth/docs/research/dialogue-authoring-2026-11-26.md` (DQ-33). Related:
[rpg.md](rpg.md) "Dialogue", "Journal"; [dialogue-rules.md](dialogue-rules.md) (facts, rules, selection, memory);
[dialogue-presentation.md](dialogue-presentation.md) "Presenter seam", "Pacing and subtitles"; [ui.md](ui.md)
"localization".

> **Agent summary.** New content (and other games) writes dialogue as **Yarn Spinner 3 subset** files, imported by our
> own Bevy-free parser (`sf_dialogue_yarn`) into a serde **`DialogueScene`** (`sf_rpg::scene`). Node groups (`when:`),
> line groups (`=>`) and jumps to a group become **rules** in the existing `RuleDb`: no second selection mechanism.
> A **`SceneCursor`** in `DialogueSession` steps a node: lines, options (the existing `Choosing`), `<<set>>` writes
> to `memory.*`, host commands. Saved state = the cursor (scene, node, statement path) plus `DialogueMemory`; no VM.
> Every line has a stable **`LineId`** (authored `#line:` or written once by a deterministic tagger patch). Text lives in
> per-locale **CSV string tables** keyed by line ID (lock hash, speaker, voice, comment), per mod, later mods override
> by ID. The journal re-renders from `source` = line ID. `LineProvider` (LLM seam) can only replace display text; off.
> Morrowind's DIAL/INFO keeps its own path.

## Goal

- **Author:** writes `dialogue/<scene>.yarn` in a mod (Yarn's editors work on the subset); headers say where a node
  plays (greeting, topic, named concept) and when (`when:`); runs the tagger before translation; ships
  `strings/<locale>/<mod id>.csv`. Tools may write `sf.dialogue.scene` RON records directly.
- **Player:** branching conversations whose variants follow world state and do not repeat, on the same presenters as
  Morrowind dialogue (subtitles, voice, timed choices), in their language, journal included. Co-op: one player
  drives, others hear the lines.

## Today (verified 2026-11-26)

- `sf_rpg/src/dialogue.rs`: `DialogueSession` `:182`, `SessionState` `:145` (`Talking`, `Choosing(Vec<Choice>)`,
  `Goodbye`, `Ended`), `DialogueOutput` `:127` (`Line { topic, response, text }`, `Learned`, `RunResult`,
  `NoResponse`), `Choice { label, value: i32 }` `:139`. No scene, no line ID, no `rules.rs` (DQ-23a/b unbuilt).
- Journal: `JournalEntry`/`EntryDraft { source: Option<RecordId>, text }` (`sf_rpg/src/quest.rs:109-125`), `HeardLine`
  `:169`; `JournalLabels::entry_text` defaults to the snapshot (`sf_rpg_ui/src/labels.rs:18`, journal-ui.md:47-49).
- UI strings: Fluent, `l10n/<locale>/<mod id>.ftl` with a locale chain (`sf_ui/src/l10n.rs:7`). No dialogue tables.
- Rules: spec only (dialogue-rules.md:108-199). Presenter input (dialogue-presentation.md:82-118): speaker, text,
  `LineAudio` from the voice player, `Deadline` on `Choosing`.

## Reuse check (rule 0, 2026-11-26)

**Bevy** has no dialogue or localisation-table feature (0.20). **Crates**, read from source
(YarnSpinner-Rust `79e513eb`; upstream YarnSpinner v3.3.2 `66f9f23` for the grammar):

- **(A) `yarnspinner` 0.10 as is.** Out: Yarn 2.5 only. The lexer's token file has no `WHEN`, `ONCE`, `DETOUR`,
  `RETURN`, `LINE_GROUP_ARROW` (`compiler/src/parser/generated/YarnSpinnerLexer.tokens`), so node groups fail the
  decision-8 check; the runtime has no save API and OS-random functions (`core/src/library.rs:84-98`).
- **(B) vendored fork in `superfargoth/vendor/yarnspinner`.** Add: regenerate the 3.x grammar (`YarnSpinnerLexer.g4`
  399 lines, 13 modes with semantic predicates, e.g. `{IsInWhenClause()}?` `:233-235`; parser 206 lines) with the
  custom 2020 `antlr4-4.8-2-Rust0.3.0-beta` jar plus six hand patches (`generated/readme.md`): ~385 KB of generated
  Rust again (today 9 566 lines); port `NodeGroupCompiler.cs` (380 lines), the code generator delta (C# 1 404 lines vs
  Rust `code_generation_visitor.rs` 697), saliency opcodes (`OpCode` ends at `RunNode = 16`,
  `core/src/generated/yarn.rs`), the detour stack; publish and serde `State` (`runtime/src/virtual_machine/state.rs`,
  `pub(crate)`: pc, options, value stack); seed `core/src/library.rs:84-98`; retype `VariableStorage`
  (`runtime/src/variable_storage.rs:20-45`: f32/String/bool, whole-map methods, panic on undeclared at
  `virtual_machine.rs:515-540`). Upstream's own 3.x port, draft PR #284, is +14 760 / -8 626 lines, stalled since
  2026-07-19. **Maintain:** ~15 000 lines (compiler 7 488, runtime 4 458, core) + the generated parser + pinned
  `antlr-rust =0.3.0-beta` and `better_any =0.2.0` (unmaintained; PR #281 open), each upstream merge conflicting with
  our 3.x patches. **Compile:** `antlr-rust`, `better_any`, `regex`, `annotate-snippets`, `rand`, `crc32fast`,
  `globset`, `walkdir` (`compiler/Cargo.toml:17-30`), `icu_plurals`, `icu_locale_core`, `fixed_decimal`, `unicode-*`,
  `once_cell`, `bevy_platform` (`runtime/Cargo.toml`). **Bevy:** `bevy_platform` only for `HashMap`/`HashSet`/`Arc`/
  `RwLock` (10 `use` sites in `runtime/src`): swappable, not the blocker. **Net:** deterministic and saveable after
  the patches, but its saliency would be a second selection mechanism beside `RuleDb`, its variables a map shim.
- **(C) own parser for a Yarn 3 subset.** Hand mode lexer + recursive descent, lowered to `DialogueScene`: ~2 500
  lines (lexer with indentation ~700, parser ~800, expressions ~300, lowering + tagger ~500, diagnostics ~200), deps
  `thiserror` + `serde`, Bevy-free, no generated code. The runner (~600 lines, `sf_rpg`) is needed under any option
  (decision 8: our model, rules select). Risk: lexer fidelity; upstream fixtures gate it from the first slice.

**Pick: (C).** Deciding numbers: generated code 385 KB vs 0; maintained lines ~15 000 vs ~2 500; pinned
unmaintained deps 2 vs 0. (B) loses: we would carry a dead parser generator and a VM whose selection, storage and
saving we bypass anyway. (C) still reuses the language, its editor tooling, docs and MIT test corpus.

**`bladeink` 2.1.0** (2026-10-17, Apache-2.0, ink runtime port, reads compiled ink JSON): later, as a second importer
into `DialogueScene`. Not now: ink's knot/stitch weave has its own state and no line IDs, so it needs its own mapping.

## Industry comparison

Canonical technique: **node-based script + content-selected variants + stable line keys + per-locale string tables
with a change hash** (Yarn Spinner 3; Valve/Ruskin rule selection; Unreal String Tables).

| Them | What they do (source) | Us | Gap |
|---|---|---|---|
| Yarn Spinner 3 | `when:` node groups, saliency, `=>`, `once`, `detour`; `#line:` tagger; CSV with `lock` (8 hex SHA-256) (docs.yarnspinner.dev "Node Groups", "Localisation") | the subset; groups via `RuleDb`; same CSV + speaker/voice | no markup, enums, smart variables |
| Ink | knots, weave, inline choices; no line keys (inkle "Writing with ink") | later importer | must tag |
| articy:draft | flow graph export JSON/XML, localisation to Excel by ID (articy docs "Localization") | RON scene = export target | no importer |
| Twine | Twee 3 passages, no line IDs (twinery.org) | — | not a target |
| Unreal / Unity | String Tables keyed namespace + key, gather to PO, changed source flagged (UE "String Tables", "Localization Dashboard"); Unity Localization string tables | line ID = key; per locale per mod; lock = stale | no PO/XLIFF |
| Witcher 3 / Cyberpunk / BG3 | numeric IDs in `.w3strings`, VO by ID (W3 modding wiki); `LocKey` + VO map by ID (WolvenKit wiki); `.loca` handles, VO by handle (BG3 modding wiki) | voice key = line ID by default | — |

Not built: a visual editor (Yarn's tooling is it), Yarn saliency strategies (rules decide), PO/XLIFF (CSV is what
Yarn tools and spreadsheets read), runtime machine translation.

## Design

### Model (`sf_rpg::scene`, Bevy-free, serde; DQ-35a)

```rust
pub struct LineId(Sym);                         // text form "line:<tag>", globally unique across loaded mods
pub struct DialogueScene {
    pub id: RecordId,                           // "<mod>.<file stem>" for imports; record id for RON scenes
    pub cast: Vec<(Sym, SpeakerRef)>,           // Yarn character names -> roles
    pub nodes: Vec<SceneNode>,                  // NodeIdx = index; titles unique except group members
    pub hash: u64,                              // of the lowered content: a stale cursor restarts its node
}
pub enum SpeakerRef { Speaker, Listener, Player, Cast(Sym) /* host resolves, e.g. a quest alias */ }
pub struct SceneNode {
    pub title: Sym,
    pub member: Option<RecordId>,               // group member id ("id:" header, tagger-assigned): its rule id
    pub concept: Option<Concept>,               // "concept:" header: greeting | topic.<id> | named.<id>
    pub when: SmallVec<[Criterion; 4]>,         // from "when:", AND only (rule criteria)
    pub when_once: bool,                        // "when: once"
    pub tags: SmallVec<[Sym; 2]>,
    pub body: Vec<Stmt>,
}
pub enum Stmt {
    Line(SceneLine),
    Options(Vec<SceneOption>),                  // a run of "->"
    LineGroup { concept: RecordId },            // "=>" items are rules on Concept::Named(concept)
    If(Vec<(Cond, Vec<Stmt>)>, Vec<Stmt>),      // if / elseif ... / else
    Once { mark: LineId /* its first line */, body: Vec<Stmt>, otherwise: Vec<Stmt> },
    Set(FactWrite),                             // memory.* only (dialogue-rules "Rules")
    Jump(JumpTarget), Detour(JumpTarget), Return, Stop,
    Command(SceneCommand),
}
pub struct SceneLine { pub id: LineId, pub speaker: SpeakerRef, pub base: String /* "{0}" slots */,
                       pub args: SmallVec<[Expr; 2]>, pub cond: Option<Cond>, pub once: bool,
                       pub meta: SmallVec<[Sym; 2]> /* other hashtags */ }
pub struct SceneOption { pub line: SceneLine, pub body: Vec<Stmt> }
pub enum JumpTarget { Node(NodeIdx), Group(Concept) }   // a title with members is a group: rules pick
pub enum Cond { Crit(Criterion), All(Vec<Cond>), Any(Vec<Cond>), Not(Box<Cond>) }
pub enum Expr { Num(f64), Bool(bool), Fact(FactKey), Call(SceneFn, SmallVec<[Expr; 2]>),
                Bin(BinOp, Box<Expr>, Box<Expr>), Neg(Box<Expr>) }
pub enum SceneFn { Visited, VisitedCount, Random, RandomRange, Dice, Round, Floor, Ceil }
pub struct SceneCommand { pub name: Sym, pub args: SmallVec<[CmdArg; 3]> }   // CmdArg: Text | Expr
impl DialogueScene { pub fn validate(&self, facts: &FactTable) -> Vec<SceneError>; }
```

- Conditions are **rule criteria**, tested by the rules' own criterion code over `Facts`; `Cond` only adds and/or/not
  for body branches. `when:` must be a flat AND (specificity counts it as for a rule).
- Yarn variables: `$name` binds to a fact key. Default `memory.world.<scene>.<name>` (read/write); a mod's
  `dialogue/bindings.ron` maps `$name` to any fact (`$gold` -> `player.gold`), read-only unless it is `memory.*`.
  `<<declare $x = 0>>` gives the type and the value read on `Missing`.
- Records: `sf.dialogue.scene` (RON, `sf_records` / `sf_mods`, mod-patchable by id). Imports produce the same type.

### Selection mapping (no second mechanism)

| Yarn | Becomes |
|---|---|
| node with unique title, no `concept:` | plain node, reached by `<<jump>>` or a rule's scene response |
| node with `concept:` or a group of same-title nodes | one `Rule` per member: `concept` (default `Named("<scene>.<title>")`), `criteria` = `when`, `once` = `when_once` (scope below), `priority` = `PRIORITY_CONTENT`, `id` = `member` |
| `=>` line group | rules on `Named("<scene>.<title>.g<k>")`, one per item; item `<<if>>` -> criteria, `<<once>>` -> rule `once` |
| `<<jump Group>>` | `RuleDb::select(Best)` on the group's concept; `None` -> the node ends |
| `visited("T")`, `visited_count("T")` | fact `memory.world.<scene>.visits.<T>` (written on node exit) |
| `<<once>>` block, `once` line/option | `DialogueMemory.once` mark keyed by the first line's `LineId` (block) or the line's own |

Rules gain one field (DQ-35a, additive to dialogue-rules.md "Rules"): `Response.scene: Option<SceneEntry { scene,
node }>`; a picked greeting or topic rule with `scene` starts the runner. `once` scope: `World` (Yarn's per-save
meaning), `#once:player` / `#once:speaker` tags override (open question 1). The rules' recency-demoted seeded
tie-break is Yarn's "best least recently seen" with a seeded hash in place of file order.

### Runner on the session (`sf_rpg::scene::runner`; DQ-35b)

```rust
pub struct SceneCursor { pub scene: RecordId, pub hash: u64, pub frames: SmallVec<[Frame; 2]> } // detour stack
pub struct Frame { pub node: NodeIdx, pub path: SmallVec<[u16; 4]> /* statement index per nesting level */ }
pub enum SceneStep { Line(LineId), Choose(Vec<Choice>), Command, Ended(SceneEnd) }  // outputs go to `out`
pub enum SceneEnd { NodeDone /* back to topics */, Stop /* goodbye */ }
pub struct SceneEnv<'a> { pub tick: u64, pub facts: &'a dyn Facts, pub memory: &'a DialogueMemory,
                         pub rules: &'a RuleDb, pub scenes: &'a SceneDb, pub step_limit: u32 }
impl SceneCursor {
    pub fn enter(entry: SceneEntry, scenes: &SceneDb) -> Result<Self, SceneError>;
    /// Runs until a line, options, a command or the end; never more than `step_limit` statements.
    pub fn step(&mut self, env: &SceneEnv, q: &QueryCtx, out: &mut Vec<DialogueOutput>) -> SceneStep;
    pub fn choose(&mut self, index: i32) -> Result<(), DialogueError>;
}
```

- `DialogueSession` gains `scene: Option<SceneCursor>`. While set, the new request `advance()` (the presenter's
  "next") steps it and `ask(topic)` errors (`ScenePending`). Options reuse `Choosing` (`value` = option index, its
  `Deadline`, DQ-31b); `choose` resumes the cursor. `NodeDone` -> `Talking`, `Stop` -> `Goodbye`.
- New outputs: `SceneLine { line: LineId, speaker: SpeakerRef, text, voice: Option<VoiceKey> }` (what the presenter
  needs; voice from the table, default the line ID); `SceneWrites(SmallVec<[FactWrite; 2]>)`, applied by the host as
  `RpgCommand::SceneWrite { writes, ctx, tick }` on the `RuleFired` memory path; `SceneCommand(..)`, turned into
  `RpgCommand`s by the adapter's `SceneHost` as `RunResult` is. Within a `step`, pending writes overlay `Facts`.
- Randomness: `splitmix64(tick ^ speaker_key ^ hash(statement))`, pure (dialogue-rules "Selection" 4).
- **Saved:** the cursor (in the session) + `DialogueMemory` (`sf/dialogue`, unchanged); no VM or value stack. A
  cursor whose `hash` differs restarts its top node (logged); a missing scene ends (`NodeDone`).

### Line IDs (DQ-36b tagger, DQ-37a checks)

- **Authored:** `#line:<tag>` on the line (Yarn convention); `<tag>` is `[a-z0-9_.-]+`, globally unique among loaded
  mods (duplicate = load error naming both files, unless the second is an override table row).
- **Untagged:** implicit ID `line:` + 8 lowercase base32 chars of `hash64(scene id, node title, member id, ordinal in
  node)`, flagged `implicit`. Positional, so it is never shipped: tables export only tagged lines, and the load warns
  per untagged scene.
- **Tagger** (`sf_dialogue_yarn::tag`, a tool/bake step, never in play): a text patch per file adding `#line:` to
  untagged lines and `id:` to group members, by the implicit formula with a salt bumped on collision; deterministic
  (Yarn's uses the OS RNG, `untagged_line_listener.rs:42-50`). The author commits the patched files.
- **Edits:** a moved line keeps its tag, so its translations and voice. Changed base text changes its `lock` (first
  8 hex of SHA-256 of the text, `\r\n` -> `\n`; Yarn's format): older translations become **stale**. A deleted line
  orphans its rows (warning, ignored). A copied line copies its tag: the duplicate check catches it.

### String tables per locale (DQ-37a)

Lines are tables, not Fluent: dialogue tools (Yarn, articy, voice studios, CAT tools) exchange per-line tables with
lock, speaker and voice columns; Fluent stays for UI strings (messages with logic, no line metadata).

- `strings/<locale>/<mod id>.csv`, UTF-8, RFC 4180, header `id,text,lock,speaker,voice,comment,scene,node`. The base
  table (authored locale, mod manifest `base_locale`, default `en`) is exported from scenes (`export_base`); a
  translation's `lock` is the base lock it was translated from.
- **Lookup:** active locale, its parent (`fr-CA` -> `fr`), the fallback chain (ui.md setting, shared with Fluent),
  then the scene's base text. Tables load in mod order; per `(locale, id)` the last mod wins.
- **Stale** (lock differs from the current base): still shown (Unreal shows changed-source translations until
  regathered), counted per locale and logged once per table with the IDs.
- `{0}`.. slots are filled from `SceneLine.args` after lookup; a row whose slot count differs from base is rejected.

```rust
pub struct LineTable { /* (locale idx, LineId) -> Row */ }
pub struct Row { pub text: String, pub lock: [u8; 4], pub speaker: Option<Sym>, pub voice: Option<VoiceKey>,
                 pub comment: String }
impl LineTable {
    pub fn load(&mut self, locale: &Locale, mod_id: &str, csv: &str) -> Vec<TableIssue>;
    pub fn text<'a>(&'a self, chain: &[Locale], id: &LineId, base: &'a str) -> Resolved<'a>; // text + stale flag
    pub fn voice(&self, chain: &[Locale], id: &LineId) -> VoiceKey;   // column, else "voice/<locale>/<id>"
    pub fn export_base(scenes: &[DialogueScene]) -> String;             // tagged lines only
}
```

### Journal re-render (DQ-37b)

Journal entries and `HeardLine`s written from a scene store `source` = the line ID and the expanded snapshot.
`sf_rpg_ui` gains `table_entry_text(&LineTable, chain, entry)` for `JournalLabels::entry_text`: a known, slot-free
line re-renders in the active locale, else the snapshot. Morrowind's adapter keeps its own `entry_text`.

### Yarn import (`sf_dialogue_yarn`, Bevy-free; DQ-36a/b)

`pub fn import(files: &[YarnFile], bindings: &Bindings, facts: &FactTable) -> Import { scenes, rules, base_rows,
diagnostics }`; `pub fn tag(files: &[YarnFile]) -> Vec<TextPatch>`. Diagnostics carry file, line, column.

| Construct | Status | Error message when rejected |
|---|---|---|
| headers `title`/`tags`/`concept`/`id`, `---`/`===`, `//`; `Name: text`, escapes, `{expr}`, hashtags | yes | — |
| `->` options (`<<if>>`, `<<once>>`, nested), `if/elseif/else`, `once/else/endonce`, `=>` line groups | yes | — |
| `<<jump T>>`, `<<detour T>>`, `<<return>>`, `<<stop>>`; any other `<<cmd>>` = host command | yes | — |
| `<<set $x to n>>`, `to $x ± n`, `to true/false`; `<<declare $x = literal>>` | memory.* | "only memory.* can be set; use a command" |
| `when:` `always`, `once`, AND of variable-vs-constant comparisons, `visited()` | yes | "when: supports and-ed comparisons with constants" |
| `visited`, `visited_count`, `random`, `random_range`, `dice`, `round`, `floor`, `ceil` | yes | "unknown function <f>" |
| string variables, `<<enum>>`, smart variables, markup (`[b]`, `[plural]`, `[select]`), `#shadow:`, `jump {$expr}` | no | "<construct> is not supported by this importer" |

Upstream fixtures (Tests below) gate every construct; the subset grows only by adding rows here.

### `LineProvider` seam (`sf_rpg::scene::provider`; DQ-38)

```rust
pub struct LineRequest<'a> { pub line: &'a LineId, pub resolved: &'a str, pub speaker: &'a SpeakerRef,
                             pub locale: &'a Locale, pub meta: &'a [Sym] }
pub trait LineProvider: Send + Sync {
    /// Display text replacing `resolved`, or None. Synchronous; an async backend prefetches.
    fn text(&self, req: &LineRequest<'_>) -> Option<String>;
}
```

The host calls it after table lookup, before emitting `SceneLine`, once per line (co-op listeners see the same
text). It gets no `Facts`, `RuleDb` or memory and returns only text: it cannot write state or pick a branch; line ID,
voice and journal `source` stay authored. Default none (setting `dialogue.line_provider` = off).

## Net-ready, saves, split, crates

- Host only: rules, runner, random, writes; clients send `advance/choose/goodbye`, receive outputs (decision 9).
  Presentation never feeds facts. Saved: the cursor in the session (if the save gate allows a mid-conversation save)
  and `DialogueMemory` (`sf/dialogue` v1, unchanged); scenes and tables are mod data.
- `superfargoth/` never names Morrowind; DIAL/INFO stays on its compiled-rule path; scenes serve new content and
  other games and coexist with INFO topics in one session.
- `sf_rpg` (Bevy-free) adds `scene`, `lines`; deps `sha2` 0.10 (in `Cargo.lock`), `csv` 1.x (or `csv-core` if `cargo
  deny` objects). New Bevy-free `sf_dialogue_yarn` (deps `sf_rpg`, `thiserror`, `serde`) goes in
  `final-excludes.workspace-members` of `.config/hakari.toml`. Loading `.yarn` from mods (`sf_vfs` scan -> `import`
  -> `SceneDb`/`RuleDb`/`LineTable`) belongs to the game crate that builds `DialogueDb`, queued when a game ships one.

## Tunables

| Name | Default | Range | Unit | Net | Source |
|---|---|---|---|---|---|
| `dialogue.scene.step_limit` | 10 000 | 100-1 000 000 | statements per `step` | G | Guess (runaway jump loop guard, as the MWScript step limit) |

`const`: line ID format and hash length, lock length (Yarn's 8 hex), CSV columns, detour depth cap 8 (capacity).
Settings, not tunables: `dialogue.line_provider` (off), locale chain (ui.md).

## Tests

- **Round trip (DQ-35a):** every `DialogueScene` type through RON and postcard, equal after; `validate` errors for a
  bad jump, a non-memory write, a duplicate title outside a group.
- **Runner (DQ-35b), exact:** lines in order, options -> `Choosing` -> branch, `if/elseif/else`, set-then-if overlay,
  jump to a group picks the most specific member and demotes the last fired, `once` line and block shown once per
  scope, detour/return, `stop` -> `Goodbye`, step limit ends a `<<jump>>` loop with an error, same tick same `random`;
  save mid-node and reload continues at the same statement; changed `hash` restarts the node.
- **Goldens (DQ-36a/b):** upstream `Tests/TestCases/*.yarn` copied to `sf_dialogue_yarn/tests/upstream/` (MIT,
  YarnSpinner `66f9f23`, `LICENSE.md` copied, `superfargoth/THIRD_PARTY_NOTICES.md` entry). Parse + lower and run
  the `.testplan` (a test-only reader of `line:`/`option:`/`select:`/`command:`/`set:`/`stop`) for Lines,
  ShortcutOptions, IfStatements, Jumps, Detours, Detours-MayDetourToThemselves, Once, LineGroups, NodeGroups,
  NodeGroupVisitTracking, NodeGroupsWithImplicitDeclarations, VisitCount, Visited, VisitTracking, VisitedInWhenHeader,
  Indentation, Commands, Expressions, DecimalNumbers, Smileys; top-level Headers, TaggedLines, Options. Rejected
  goldens (the table's message): Enums*, SmartVariables, ShadowLines, FormatFunctions, Types, InlineExpressions
  (strings), Escaping (markup); `ParseFailures/` fails with a diagnostic. A testplan that depends on Yarn's file-order
  tie-break goes in `tests/divergences.ron` with the reason (count printed; target: none other).
- **ID stability (DQ-36b):** insert, move, edit lines in a tagged fixture: tagged IDs unchanged, only the new line
  gets a tag, the edited line's lock changes; a second tagging is an empty patch; same input, identical patch.
- **Tables (DQ-37a):** `fr-CA` -> `fr` -> base; last mod wins; stale shown and counted; slot mismatch rejected;
  orphan warned. **Journal (DQ-37b):** a line `source` renders in the active locale; unknown or slotted -> snapshot.
- **Provider (DQ-38):** only `SceneLine.text` changes; memory, other outputs, journal `source` identical; off = no call.

## Slices

| ID | Deliverable + test | Crates | Depends | Verify |
|---|---|---|---|---|
| DQ-35a | `DialogueScene` model, `SceneDb`, `validate`, `Response.scene`. Round trip | `sf_rpg` | DQ-34, DQ-23a | `cargo test -p sf_rpg` |
| DQ-35b | `SceneCursor` runner in `DialogueSession`, scene outputs, `SceneWrite`. Exact unit + save mid-node | `sf_rpg` | DQ-35a, DQ-23b, DQ-24b | `cargo test -p sf_rpg` |
| DQ-36a | Own Yarn 3 subset lexer + parser to AST, upstream fixtures + notice. Parse goldens | new `sf_dialogue_yarn` | DQ-34 | `cargo test -p sf_dialogue_yarn` |
| DQ-36b | Lowering to `DialogueScene` + rules, rejection list, deterministic tagger. Testplan goldens + ID stability | `sf_dialogue_yarn` | DQ-36a, DQ-35b | `cargo test -p sf_dialogue_yarn` |
| DQ-37a | `LineTable`: CSV per locale per mod, fallback, override, lock/stale, voice key. Exact unit | `sf_rpg` | DQ-35a | `cargo test -p sf_rpg` |
| DQ-37b | `SceneLine` in the dialogue view-model + journal re-render from `source`. Unit | `sf_rpg_ui` | DQ-37a, DQ-35b | `cargo test -p sf_rpg_ui` |
| DQ-38 | `LineProvider` seam, off by default. Unit: no state writes | `sf_rpg` | DQ-35b | `cargo test -p sf_rpg` |

DQ-36a can run in parallel with DQ-35 (no `sf_rpg` dependency until lowering).

## Open questions (what the author or player can do)

1. **Default `once` scope in co-op:** `World` (a story beat happens once for the group; BG3/Divinity share story
   progress) or `Player`. Recommended: `World`, with `#once:player` for lines each player should hear.
2. **Can a later mod patch another mod's scene structure** (not just its lines)? Recommended: yes, whole-scene
   replacement by record id (as every record); per-node patching waits for a real need.
