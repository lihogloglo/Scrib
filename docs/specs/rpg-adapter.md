# Morrowind RPG adapter (`mw_rpg`)

Status: **implemented** (rpg-core slice 2, 2026-10-21; result scripts W3, 2026-10-23, see
"As built (W3)"). Framework spec: `../superfargoth/docs/specs/rpg.md` (`sf_rpg`). Roadmap: Phase 2.4.
Behaviour reference: OpenMW (GPL, fine in this repo) and UESP's Morrowind modding pages. The
"verify" marks below are resolved in "OpenMW findings" at the end; where they disagree, the
findings (and the code) win.

## Goal

Map Morrowind's dialogue database (DIAL/INFO), journal topics, items and equipment onto `sf_rpg`,
evaluate INFO filters, expand text macros, and hand result scripts to the script runtime, so the
Phase 2 acceptance flow runs: talk to Fargoth → his ring → journal entry → save/load.

## Data (verified against the Steam install, 2026-10-21)

| File | DIAL | INFO | Topic / Voice / Greeting / Persuasion / Journal | QSTN / QSTF / QSTR |
|---|---|---|---|---|
| Morrowind.esm | 2,358 | 23,693 | 1,698 / 8 / 10 / 10 / 632 | **0 / 0 / 0** |
| Tribunal.esm | 893 | 6,757 | 264 / 4 / 5 / 0 / 620 | 615 / 813 / 4 |
| Bloodmoon.esm | 860 | 6,504 | 220 / 5 / 5 / 1 / 629 | 623 / 831 / 3 |

**Finding:** Morrowind.esm alone has no quest names or finished flags; the expansions re-ship the
base game's journal topics with a quest-name INFO (index 0, `QSTN`) and `QSTF` on final stages
(e.g. `MS_FargothRing`). With Morrowind.esm only, the journal falls back to a flat dated log
and quests show their id (as the original 1.0 journal did).

Filter (SCVR) type counts in Morrowind.esm: Function 5,486, Global 4,746, Local 358,
Journal 8,212, Item 1,048, Dead 1,258, NotId 427, NotFaction 139, NotClass 237, NotRace 189,
NotCell 142, NotLocal 2,593.

`mw_records` already loads DIAL and INFO through `tes3` (typed `DialogueInfo` with `filters:
Vec<Filter>`, `quest_state`, speaker fields, `script_text`), keyed `<topic>\u{1f}<info id>`.
Response order (PNAM/NNAM linked lists across plugins) was left to this system (DECISIONS §11).

## Building the `DialogueDb`

1. **Order.** Per topic, rebuild the INFO order by load order: each plugin's INFO is inserted
   after its `prev_id` (PNAM) if that exists in the merged list, else before its `next_id`
   (NNAM), else appended; a re-defined INFO keeps its position unless its PNAM changed
   (verify against OpenMW's `Dialogue::addInfo`). Deleted INFOs drop out.
2. **Kinds.** DIAL type Topic → `Conversation`; Greeting → `Greeting`; Persuasion → `Service`;
   Voice → `Bark`; Journal → not in the db (journal table below).
3. **Greeting order:** `Greeting 0` … `Greeting 9` (ids by name; verify modded extras are
   ignored).
4. **Journal table** (`MwJournalDb`): per journal topic, index → INFO (text, `QSTF`, `QSTR`), and
   the `QSTN` INFO's text as the quest name.

## Filter evaluation (`MwDialogueRules::accepts`)

All checks must pass; cheap rejects first (OpenMW `Filter::testActor`, `testPlayer`,
`testSelectStructs`, `testDisposition` order):

1. **Speaker**: `speaker_id` equals the actor's base id; `speaker_race`, `speaker_class`,
   `speaker_faction` equal the NPC's (any of them set fails for creatures); `speaker_rank`: the
   NPC's rank in its faction ≥ value; `speaker_sex`; `speaker_cell`: the speaker's cell name
   **starts with** the value, case-insensitive (verify prefix vs equality).
2. **Player**: `player_faction` (player is a member, rank ≥ `player_rank`); `player_rank` with no
   faction uses the speaker's faction. The player is `ctx.player` (net.md context player).
3. **SCVR conditions** (≤ 6), each `value OP literal` with `= != > >= < <=`:
   - `Function`: the function table (PC stats/skills, `Choice`, `PCReputation`, `SameFaction`,
     `PCGold`, `Health %`, `PCSex`, `Detected`, `Alarmed`, `Weather`, `Level`, `Talked to PC`,
     `Reaction`, `Rank Requirement`, `Creature Target`, `Friend Hit`, …). Slice 2 implements
     `Choice` and returns neutral values (0) for the rest, logged once per function.
   - `Global`: the global's value; `Local`/`NotLocal`: the speaker's script local (missing
     script or variable: `Local` fails, `NotLocal` passes; verify).
   - `Journal`: the quest's stage (0 if unset). `Item`: count of that item in the **player's**
     inventory. `Dead`: the base id's death count.
   - `NotId`, `NotFaction`, `NotClass`, `NotRace`, `NotCell`: speaker is not that (value compared
     as 0/1, as stored).
4. **Disposition**: the speaker's disposition ≥ INFO `disposition` (Topic and Greeting kinds only).

**Info refusal:** if no INFO under a topic passes but one passes ignoring step 4, show the first
passing response of the `Info Refusal` persuasion topic instead (OpenMW's fallback; verify for
greetings).

**Topic list:** known topics for which some INFO passes steps 1-3 for this speaker (disposition
ignored, so the topic shows and the refusal applies; verify).

## Text (`MwDialogueRules::render`)

- Macros (case-insensitive, longest name first): `%PCName`, `%PCRace`, `%PCClass`, `%PCRank`,
  `%PCNextRank`, `%PCCrimeLevel`, `%Name`, `%Race`, `%Class`, `%Faction`, `%Rank`, `%NextRank`,
  `%Cell`, `%Global` names (`%<global id>`; verify the full list and rounding).
- Topic links: `TopicLinker` over all `Conversation` topic names, `LinkRules { word_start: true,
  word_end: false }` (verify against OpenMW's `KeywordSearch`: links inside longer words and
  possessives), leftmost-longest. Every linked topic is learned (`AddTopic`; verify whether the
  search set is all topics or only the speaker's). Explicit `@topic#` markers: **none** in
  Morrowind.esm dialogue text (counted 2026-10-21), so no marker pass; ui-adapter.md's "(verify)"
  is resolved. Persuasion topics: `Admire/Intimidate/Taunt/Bribe Success|Fail`, `Info Refusal`,
  `Service Refusal`.
- Journal entries render with the same macros at write time (snapshot in `JournalEntry.text`).

## Result scripts

After a `Line` output the host runs the INFO's `script_text` as the speaker (scripting-adapter.md
rule 9), context player = the talking player. Results run in `mw_script`'s native interpreter
over the W1 AST (Luau translation W1/W2 exists, 17,144 results compile; parked with Luau, owner
2026-10-18); the runner binds these functions to `sf_rpg` commands and session callbacks:

| MWScript | `sf_rpg` |
|---|---|
| `Journal id n` | `SetStage { quest: id, stage: n, flags from INFO n (QSTF/QSTR), entry: INFO n text }` |
| `SetJournalIndex id n` / `GetJournalIndex id` | `SetStage` without entry / `QuestLog::stage` |
| `AddTopic t` | `AddTopic(t)` |
| `AddItem i n` / `RemoveItem i n` (`Player->` = context player, else speaker) | `AddItem` / `RemoveItem { holder }` |
| `GetItemCount i` | `Inventory::count` |
| `Choice "a" 1 "b" 2 …` | `session.offer_choices` |
| `Goodbye` | `session.force_goodbye` |
| `ModDisposition n` / `SetDisposition` | `mw_rpg` NPC state: disposition modifier per `ActorKey` (world state, serde) |
| `ModPCFacRep`, `PCRaiseRank`, `ModReputation`, … | not interpreted yet: neutral value, reported once (M7 factions) |
| `set <local> to v` / `set <global> to v` | speaker's script locals / globals (host, through `MwWorld`) |
| `MessageBox "fmt" v… "button"…` / `PlaySound s` | UI / audio events |

Selection rule for `Journal` text: the INFO under the journal topic whose index equals `n`; if
none, the stage is set with no entry (vanilla behaviour, verify). Duplicate `Journal` calls at the
current index: verify whether an entry is added again; default: skip if the last entry for that
quest has the same index.

## Items and equipment

- Items are records of kinds `ALCH APPA ARMO BOOK CLOT INGR LIGH LOCK MISC PROB REPA WEAP`;
  `Attrs` keys: `condition` (Int), `charge` (Float), `soul` (Id), `owner` (Id) (M7 adds more).
- `SlotTable` (Morrowind): helmet, cuirass, greaves, left pauldron, right pauldron, left gauntlet,
  right gauntlet, boots, shirt, pants, skirt, robe, left ring, right ring, amulet, belt, carried
  right (weapon), carried left (shield/torch), ammunition.
- `EquipRules`: armour/clothing part → slot; rings go to the free ring slot (right first, verify);
  two-handed weapons occupy carried left too; ammunition and thrown weapons equip whole stacks;
  lights go carried left. Beast-race restrictions are checked before `equip` (rules).

## Acceptance flow: Fargoth's ring (verified IDs)

Records (Morrowind.esm; quest name and `QSTF` from Tribunal/Bloodmoon):
- NPC `fargoth`, script `LocalState` (local `state`). Item `ring_keley` (CLOT, "Engraved Ring of
  Healing").
- `Greeting 5`, INFO `31191105114705102`: speaker fargoth, `Journal MS_FargothRing = 0` → "…I
  swear they took my rin[g]…": links and learns `ring`.
- Topic `ring` (speaker fargoth), in order:
  1. `12942106023022017709` `Local state = -1` → "Yes, thank you for finding it!"
  2. `156171272864323892` `Choice = 1` → "Darn. Well if you see it let me know…"
  3. `158791326429969956` `Choice = 2`, `Item ring_keley = 0` → "That's not it!…"; `ModDisposition -5`
  4. `3245423813243151497` `Choice = 2`, `Item ring_keley > 0` → "You found it!…"; `Journal
     MS_FargothRing 100`, `AddItem "ring_keley" 1`, `player->RemoveItem "ring_keley" 1`, `set state
     to -1`, `ModDisposition 50`, `"arrille"->ModDisposition 40`
  5. `2507536232473510507` no filters → "I swear one of the Guards has it…"; `Journal
     MS_FargothRing 10`, `Choice "Uhhh. No, sorry. No ring." 1 "Yes, I found it! Here take it." 2`
- Journal `MS_FargothRing`: 0 `QSTN` "Fargoth's Ring", 10 "I have met a Bosmer named Fargoth…",
  100 `QSTF` "I have returned the Engraved Ring of Healing to Fargoth…".

Headless test (slice 2, `mw_rpg/tests/fargoth.rs`, skips without an install): open with Fargoth →
greeting `31191105114705102`, `ring` learned → ask `ring` → response 5, stage 10, journal entry 1,
choices offered → choose 2 without the ring → response 3 → give the player `ring_keley` → ask
`ring`, choose 2 → response 4: stage 100 finished (with expansions), ring moved to Fargoth,
`state = -1`, journal has two entries → ask `ring` → response 1 → postcard round-trip of
`WorldRpg` + `CharacterRpg` + NPC state equals the original.

## Co-op

Filters read the context player (`ctx.player`): `Item`, PC functions, player faction. Quest,
journal, topics and dispositions are world state; inventories per character. One conversation
per NPC (`Conversations` claim).

## OpenMW findings (checked 2026-10-21, OpenMW master)

- **Topic links** (`mwdialogue/keywordsearch.cpp`): case-insensitive; a match must **start**
  after a separator (text start, space, tab, newline, `'`, `"`, `(`, `[`), has **no end check**
  ("ring" links in "rings", "ring's", not in "bring"); overlaps resolve longest-first. We use
  `LinkRules { word_start: true, word_end: false }`; differences: our start boundary is any
  non-alphanumeric (so "x.ring" links for us, not in OpenMW) and overlaps resolve leftmost-longest.
  OpenMW scans the raw text (before macros); we link the rendered text.
- **Learning linked topics** (`DialogueManager::addTopicsFromText`): the search set is all
  topics, but a topic is learned only if the **speaker answers it** (a passing INFO, Info
  Refusal fallback included), when the line is shown (after its result script). The host
  filters `DialogueOutput::Learned` with `MwDialogueRules::speaker_answers`. The window only
  hyperlinks known topics.
- **Repeated `Journal`** (`Journal::addEntry`): no new entry if the journal already has that
  quest's INFO at that index; the stage only **rises** through `Journal` (a lower index is
  ignored). An index with no INFO just raises the stage, silently. `SetJournalIndex` sets
  (up or down) without an entry. `QSTF` sets finished, `QSTR` clears it (OpenMW also reopens
  other finished quests with the same `QSTN` name: not done, M7).
- **INFO order** (`InfoOrder::insertInfo`): only `PNAM` counts (NNAM ignored): empty → front,
  found → right after it, missing → end; a redefinition with the same `PNAM` stays in place,
  otherwise moves. Deleted INFOs keep their chain slot until all plugins load.
- **Filters** (`mwdialogue/filter.cpp`): the INFO cell (`ANAM`) is the **player's** cell,
  case-insensitive prefix; `NotCell` is the speaker's cell, prefix. `Not*` kinds ignore the
  comparison and value. `Local` fails without the script or variable; `NotLocal` passes then.
  A missing global makes the filter pass. Creatures pass race/class/faction/rank, but fail
  INFOs with no speaker id, and skip the disposition check. Player rank without `DNAM` uses
  the speaker's faction. `Choice` is -1 when no choice is pending.
- **Info Refusal**: when no INFO passes but one failed only on disposition, the first passing
  `Info Refusal` response shows; **not for greetings**. The topic list uses the same search
  (disposition included, refusal counts as an answer). Greetings: `Greeting 0..9` by id order.
- **Macros** (`fixDefinesDialog`): `%` or `^`; `PCName PCRace PCClass PCRank NextPCRank
  PCNextRank Name Race Class Rank Faction Cell PCCrimeLevel` (case-insensitive prefix), then
  globals (longest id prefix; shorts/longs as integers, floats as C++ `<<`, 6 significant
  digits). Unknown macros stay. `Action*` key-binding macros are not done (UI work).
- **Equipping** (`ActionEquip`, `Clothing::getEquipmentSlots`): rings go left first, then
  right; both full → the left ring comes off, right moves left, new goes right. Two-handers
  use the weapon slot only; the shield stays equipped (hidden while the weapon is drawn).
  Ammunition and thrown weapons equip whole stacks.

## As built (slice 2)

- `MwDialogue::from_records(&RecordDb)`: `DialogueDb` (kinds as above, greetings, linker),
  `MwJournalDb` (quest → `QSTN` name + index → text/flags), and the INFO records keyed by
  `response_id(topic, info)` (the `mw_records` key). INFO order: OpenMW's insert rule over the
  merged records in first-definition order (an override that moves an INFO is placed at its
  first definition's turn; deleted INFOs are gone from the merged db, so a later INFO whose
  `PNAM` names one lands at the end). `from_topics` builds from ordered lists (tests).
- `MwDialogueRules::new(&MwDialogue, &dyn MwView)`: `accepts`/`render`/`fallback` (sf_rpg
  hook added for Info Refusal), `speaker_answers`, `expand` (macros for journal text),
  `neutral_functions` (filter functions evaluated as 0 because the view returned `None`; the
  host logs them). `MwView` is the host's read access: speaker/player `ActorFacts`
  (`ActorFacts::from_npc` resolves names from records), player faction rank, globals, speaker
  locals, quest stage, item count, dead count, derived disposition, rank names, functions.
- `MwDialogue::journal_command` / `set_journal_index`: the rules above as `RpgCommand`s.
- `NpcStates`: per-`ActorKey` disposition modifier (serde, world state).
- `MwEquipRules` (+ `slot_table`, `MwSlot`, `MwEquipKind`): kinds from ARMO/CLOT/WEAP/LIGH
  (carryable)/LOCK/PROB; `MwEquipRules::equip(&mut Loadout, stack)` applies the ring rule
  (`EquipRules::placement` alone uses the first candidate slot).
- **Result scripts**: `ResultRunner::run(ResultCall) -> Vec<ResultEffect>` is the seam; the
  host applies effects (`Journal` through `journal_command`, items as `sf_rpg` commands with
  `Target` → `Holder`, `Choices`/`Goodbye` on the session, dispositions in `NpcStates`, locals
  in the script runtime). Slice 2 tested it with a test double; W3 (below) is the real runner.

## As built (W3, 2026-10-23)

**Reuse.** Bevy has no scripting VM for legacy languages; no `MWScript` interpreter crate exists
(crates.io, 2026-10-23); OpenMW's compiler + interpreter is GPL C++ bound to its engine (behaviour
reference only). The Luau path (`sf_script`) is parked by the owner (2026-10-18), so the slice
runs a small tree-walking interpreter over the AST `mw_script` already builds; the emitter and
the interpreter share the parser, the function table and the numeric rules.

- **`mw_script::interp`**: `Interpreter::run(script, locals, host)` runs a resolved `Script` once
  (`If`/`ElseIf`/`Else`, `While`, `Return`). Numbers as in `emit.rs` "Numeric semantics":
  `f64` evaluation, exact comparisons, conversion on assignment only (`store`: short/long
  truncate and wrap, float through `f32`, non-finite → 0). Locals live in `Locals` (by slot);
  everything else goes to the `Host` trait: `get`/`set` for globals, `id.var` and speaker
  locals, typed methods for `Journal`, `SetJournalIndex`, `GetJournalIndex`, `AddTopic`,
  `Choice`, `Goodbye`, `AddItem`, `RemoveItem`, `GetItemCount`, `SetDisposition`,
  `ModDisposition`, `MessageBox`, `PlaySound`, and `unhandled` for every other function (its
  numeric arguments are evaluated; the call reads `Status::neutral`). `MessageBox` formats
  `%f %.Nf %g %d %%` in order (missing values 0), strings after the text are buttons. A run
  stops at `step_limit` (100k statements, `RunError::StepLimit`). Inert scripts never run (the
  caller checks `Parsed::is_inert`).
- **`mw_rpg::MwResults`** (game runner, one per session): parses each result once per
  response id (re-parsed when its text changes), resolving bare names against `MwWorld`
  globals, else as speaker locals. `runner(ResultState { world, player, env })` gives a
  `ResultRunner` reading `sf_rpg` state (`WorldRpg` quests, the context player's
  `CharacterRpg`) and `MwWorld` for the rest: globals, script variables (`ScriptVar { ty,
  value }` of the speaker, `id.var`, `id->`), items held by non-player targets. A run returns
  `ResultEffect`s in call order and mutates nothing; an overlay makes later reads in the
  same run see its writes (journal stage, item counts, variables), as in the original engine.
- **Effects** added: `SetGlobal`, `MessageBox { text, buttons }`, `PlaySound(id)`; `SetLocal`
  and `SetGlobal` carry the converted value; writes to undeclared variables are dropped.
  `Stub` removed. Targets: implicit = `Speaker`, `player->` = `Player`, `id->` = `Actor(id)`.
- **Semantics** (verify against the OpenMW oracle): `Journal` only raises the overlay stage
  (as `journal_command`); `RemoveItem` takes at most what the holder has (0 held: no
  effect); `AddItem` with a count ≤ 0 does nothing; `Choice` pairs a label with the next
  value (a label without one is dropped).
- **Reports** (`drain_reports`, each once per `MwResults`; the host logs them):
  `Unhandled(function)`, `Inert { response, reason }` (the result returns no effects),
  `UnknownVariable`. A step-limit stop is `ResultError::Failed`; the host drops that run.
- **Sounds and message boxes are events**, not `MwWorld` calls, so a run stays replayable
  and the host routes them to audio and UI on its tick (net rule 8).
- Tests: `mw_script` `interp_tests` (each builtin, each control-flow form, conversions,
  formatting, step limit); `mw_rpg` `runner_tests` (effect mapping, overlay reads, clamped
  removal, globals and members, report-once, inert, re-parse, step limit);
  `tests/fargoth.rs`: `fargoths_ring` (real data, through the real results; skips without an
  install) and `synthetic_ring_quest` (the same flow on our own DIAL/INFO fixture, runs
  everywhere); `mw_script` `real_install::vanilla_corpus_interprets` runs every vanilla result
  once against a neutral host and lists the functions not interpreted yet by calls.
