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

As built; DQ-24 compiles these checks to rules ("Rule compilation (DQ-24) and Voice barks (DQ-25)" below) and keeps
`accepts` as the equivalence test's oracle.

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
over the W1 AST (the Luau translation W1/W2 is superseded by the interpreter, DQ-1 2026-11-25: scripting-adapter.md); the runner binds these functions to `sf_rpg` commands and session callbacks:

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

## Quests (DQ-18)

Maps journal topics onto the framework quest model (`superfargoth/docs/specs/quests.md`). Behaviour from OpenMW
`apps/openmw/mwdialogue/quest.cpp`, `journalimp.cpp`, `apps/openmw/mwgui/journalviewmodel.cpp` at 4e718b50.

- **`QuestDb` from `MwJournalDb`** (`mw_rpg/src/db.rs:46`): one `QuestDef` per journal topic. `name` = its `QSTN`
  INFO text; `group` = the case-folded `QSTN` name (OpenMW's quest book identifies a quest by name, so several
  topics share one row: `journalviewmodel.cpp:155-183`); `listed` = has a `QSTN` name (nameless topics stay out of
  the quest book, same lines). When no loaded file has any `QSTN` (Morrowind.esm alone), every quest with entries
  lists under its id (the as-built fallback, "Data" above). No objectives, stages or aliases: the latest journal
  entry is the one implicit objective (dialogue-quests decision 7).
- **Status.** A journal INFO's `QSTF` -> `StageFlags { status: Some(Done) }`, `QSTR` -> `Some(Active)`, carried by
  `journal_command` (`db.rs:167`) exactly where `finish`/`restart` are today. The flag applies even when the index is
  below the current stage (OpenMW sets finished before comparing indexes, `quest.cpp:80-84`); a repeated `Journal`
  for an entry already in the journal only raises the stage and applies no flag (`journalimp.cpp:81-94`).
  `SetJournalIndex` never changes status (`journalimp.cpp:118`, `quest.cpp:52`). The first `Journal` or
  `SetJournalIndex` on a quest starts it `Active` (OpenMW `getOrStartQuest`).
- **Restart reopens the group.** A `QSTR` entry also emits `SetQuestStatus { Active }` for every `Done` quest of the
  same group (OpenMW reopens finished quests with the same name, `journalimp.cpp:98-108`).
- **`Failed`** never comes from Morrowind data (failures are entry text); nothing infers it.
- **Facts.** The INFO `Journal` filter reads `quest.<id>.stage` (the value `QuestLog::stage` gives today).
- **Settings:** `quests.markers = None` in both presets (owner 2026-11-25); Classic `quests.hud.tracked_line = Off`
  (book only, vanilla has no tracker); Modern `Always`, `quests.track.auto_new = true`.
- **Test (real install, exact):** `MS_FargothRing` lists as "Fargoth's Ring", `Active` after index 10 with entry 10
  as its objective, `Done` after 100 (`QSTF`); quest-book group count equals the distinct `QSTN` names among started
  quests; one `QSTR` quest from the data reopens its finished group members.

## Rule compilation (DQ-24) and Voice barks (DQ-25)

Maps INFO filters onto the framework rule database (`superfargoth/docs/specs/dialogue-rules.md`). Behaviour from
OpenMW `apps/openmw/mwdialogue/filter.cpp` (`testActor` :83, `testPlayer` :166, `testSelectStructs` :209,
`testDisposition` :218, `list` :736-786) and `dialoguemanagerimp.cpp` (greeting :168-175, topic list :352-358,
service refusal :597-603, `say` :625-664) at 4e718b50.

- **Where.** `mw_rpg::compile` at `MwDialogue::from_records` (`mw_rpg/src/db.rs:103`), after `order_infos`
  (`db.rs:223`). Rule id = the response key (`<topic>\u{1f}<info id>`), `line` = (topic, response).
- **Concepts.** Topic and Persuasion DIAL -> `Concept::Topic(id)`; `Greeting 0..9` -> one `Concept::Greeting`, index
  running across the ten topics in order (the same pick as `select_greeting`'s `find_map`, `dialogue.rs:93`); Voice
  -> `Named(bark.*)` (table below); `Service Refusal` also -> `Named("mw.service_refusal")` with the inverted
  disposition test; Journal -> none.
- **Priority** = `PRIORITY_CONTENT - index` in OpenMW INFO order: every INFO its own tier, so specificity and the
  seeded pick never apply and the first passing INFO wins, as `list(.., searchAll = false)` returns. No `once`,
  cooldown or writes: classic dialogue writes no rule memory.
- **Criteria** (stored in OpenMW's test order: actor, player, select structs, disposition):

| INFO field / SCVR | Criterion | On `Missing` |
|---|---|---|
| speaker id (`ONAM`) | `speaker.id is X` (also files the rule in the speaker sub-bucket) | fail |
| no speaker id | `speaker.npc = 1` (creatures fail INFOs that don't name them) | fail |
| race / class / faction (`RNAM` `CNAM` `FNAM`) | `speaker.race/class/faction is X` | pass (creature) |
| speaker rank (≠ -1) | `speaker.rank >= r` (NPC without faction: -1) | pass (creature) |
| sex | `speaker.female = 0/1` | pass (creature) |
| player faction (`DNAM`) + rank | `player.faction.<f>.rank >= r` | fail (not a member) |
| player rank, no `DNAM` | `mw.pc_rank_speaker_faction >= r` | fail |
| cell (`ANAM`, the player's cell, prefix) | `mw.player_cell_prefix.<s> = 1` | - |
| Function | `mw.fn.<function>` compare (unbound: 0, recorded in `neutral_functions` as today) | - |
| Function `Choice` | `event.choice` compare | fail (no choice pending) |
| Global / Local / NotLocal | `world.var.<id>` compare / `speaker.var.<id>` compare / `speaker.var.<id> absent` | pass / fail / - |
| Journal / Item / Dead | `quest.<id>.stage` / `player.item.<id>` / `world.dead.<id>` compare | - |
| NotId / NotFaction / NotClass / NotRace | `speaker.id/faction/class/race isnot X` | pass |
| NotCell (speaker's cell, prefix) | `mw.speaker_cell_prefix.<s> = 0` | - |
| disposition | `speaker.disposition >= d`, **soft** | pass (creature) |
| disposition, service refusal bucket | `speaker.disposition < d`, not soft; none when d = 0 | pass (creature) |

- **Facts impl.** `MwFacts` over the existing `MwView` (`mw_rpg/src/filter.rs:105`): each key answers from the call
  `passes_*` makes today (`filter.rs:161-312`), so equivalence holds by construction; `mw.*` keys resolve through
  `FactResolver`.
- **Refusals.** `fallback(ctx, topic, near_miss)`: near miss on a Conversation topic -> `Best` on `Topic("info
  refusal")` (no fallback for greetings, `startDialogue` :175). `service_refusal` (`filter.rs:190`) -> `Best` on
  `mw.service_refusal` with `event.choice` = the service type. Topic list and link learning (`speaker_answers`,
  `filter.rs:151`) -> `Exists`, or a near miss with a fallback.
- **Equivalence test (DQ-24a, real install, exact):** every NPC and creature base of the three ESMs as speaker x player
  states (new game, DQ-3 trace snapshots, factions joined, choices 1-9 pending) x every Conversation topic, the
  greeting, Voice topics and service refusal: the rule pick and `near_miss` equal `accepts` + `fallback`. Prints rule
  and criteria counts and µs per query by bucket (measurement).

**Voice barks.** DIAL type Voice (Morrowind.esm 8, Tribunal 4, Bloodmoon 5; "Data" above). OpenMW's `say` searches the
topic with `Filter(actor, choice 0, talkedToPlayer)` and no Info Refusal, refuses while the actor already speaks,
for NPCs swimming and for knocked-down actors, shows the subtitle, plays the INFO sound and runs its result script as
the actor (:625-664). As built: `mw_audio/src/voice.rs` `select` (first INFO through `MwDialogueRules`, no choice
pending) and `VoiceDirector::begin` (one line per actor). DQ-25b: `select` becomes a `Best` query with
`event.choice = 0` (OpenMW; the test counts INFOs whose pick changes), the voice INFO's result script runs as the
speaker, and `VoiceDirector` stays the one-line gate behind `sf_rpg::barks`.

| Voice DIAL | Concept | OpenMW trigger (4e718b50) | Our emitter |
|---|---|---|---|
| Hello | `bark.greet` | greeting machine, `mwmechanics/actors.cpp:549` | `mw_walkers_bevy` greeting (MA4c2) |
| Idle | `bark.idle` | `fVoiceIdleOdds`, 3000 units, LOS, `actors.cpp:440-452` | `mw_audio` `VoiceDirector` idle timer |
| Attack | `bark.attack` | combat start, `aicombat.cpp:693`, `mechanicsmanagerimp.cpp:1690`, `:1752` | AI30 combat cue |
| Hit | `bark.hit` | `iVoiceHitOdds` on damage, `mwclass/npc.cpp:776`; `combat.cpp:802`; `actors.cpp:1829` | combat hit event (DQ-12) |
| Flee | `bark.flee` | `aicombat.cpp:274`, `:353` | AI Flee drive cue |
| Thief | `bark.thief` | each witness of theft or pickpocket, `mechanicsmanagerimp.cpp:1182` | crime `OffenceDef.say` (theft, pickpocket) |
| Intruder | `bark.intruder` | trespass witness with Alarm ≥ 100, `mechanicsmanagerimp.cpp:1326` | crime `OffenceDef.say` (trespass) |
| Alarm | `bark.alarm` | never said by OpenMW | Classic none; Modern a Report rung (rule pack) |

Classic keeps OpenMW's triggers and gates (`barks.*` Classic values: no dedupe, cap or gaps; no pre-empt). Modern adds
dedupe, caps and rule packs as data; compiled Voice rules are the same in both.

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

## As built

Slice notes (slice 2 `MwDialogue`/`MwDialogueRules`, W3 interpreter, persuasion in the host 2026-11-08) live in
[archive/rpg-adapter-as-built.md](archive/rpg-adapter-as-built.md).
