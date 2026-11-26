# Morrowind RPG adapter: as-built history

Moved from [../rpg-adapter.md](../rpg-adapter.md) on 2026-11-26 (size). Current design lives there; these are the
slice notes as they were written.

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

## As built (persuasion in the host, 2026-11-08, compiled + unit-tested 2026-11-09: UNTESTED in game (owner test pass 2026-11-10))

- `Rpg::persuade(talk, kind, sheets, tick)` (`mw_rpg_bevy/src/dialogue/persuasion.rs`) ports `DialogueManager::persuade`,
  `goodbyeSelected` and `updateOriginalDisposition` (`dialoguemanagerimp.cpp` @`ca5e2576`): the roll is
  `mw_rpg::persuasion_roll`, `splitmix64` over (speaker key, player index, the host's tick, the session's attempt count);
  `persuasion_change` decides; the temporary change goes on the speaker's base at once (`NpcStates`), and goodbye or the
  window closing (`Rpg::end_talk`) leaves original + permanent changes, clamped so the derived value stays in 0..=100
  (`mw_rpg::PersuasionSession`, per `Talk`). Speechcraft use 0/1 as a stat command; a successful bribe moves `gold_001`
  from the context player to the speaker (refused when the player can't pay). Returns the reply DIAL ("admire success",
  ...); `persuade_and_reply` asks it as a service topic. Fight/Flee nudges stay in an unsaved per-actor map until AI
  settings exist (TODO). Tests: `mw_rpg` `persuasion::session_tests` (exact), `mw_rpg_bevy`
  `dialogue::persuasion::tests` (headless fallback; real-data Fargoth flow against the pure functions).
- Menu (2026-11-08, UNTESTED): `Rpg::refresh_actions` (after every step) gives an NPC's window one `persuasion` action whose six options are labelled from `sAdmire`, `sIntimidate`, `sTaunt`, `sBribe 10/100/1000 Gold` (button `sPersuasion`), bribes disabled below their price (`PersuasionDialog::onOpen`, `mwgui/dialogue.cpp` @`ca5e2576`).
- `Rpg::request_at(.., tick)` (the game passes `StatTick`) routes `DialogueRequest::Action` through `Rpg::route_action`: a Persuasion option → `persuade_and_reply`; other ids (services) are a logged no-op until SV3. Headless tests: options exact, bribes follow gold, requests reach `persuade`.
