# RPG core: items, equipment, quests, journal, dialogue (`sf_rpg`)

Status: **spec + core implemented** (rpg-core slice 1, 2026-10-21: modules `ids`, `item`, `equip`,
`quest`, `link`, `dialogue`, `state`; 19 unit tests incl. postcard round trips).
**Challenges ARCHITECTURE §5.3** ("dialogue graph"): dialogue is a condition-filtered response
database with a session state machine, not a graph (Research below). Adapter: Bevyrrowind
`docs/specs/rpg-adapter.md` (`mw_rpg`). Roadmap: Phase 2.4.

## Goal

The generic, Bevy-free data and state machines a content pack needs to run a quest from
dialogue: item stacks and inventories, equipment, a quest store, a journal, known topics and a
dialogue session. **No rules and no content format**: formulas (disposition, persuasion,
prices, weight), filter semantics, text macros and scripting live in the adapter (ARCHITECTURE
§5.3: `sf_rpg` primitives, the content pack's rules plugin). Everything is plain serde data so the
same values are saved (`sf_save`, M7) and replicated (net.md).

Out of scope here: stats/skills/magic/factions (M7, separate spec), barter, containers' UI
(ui.md; the dialogue window's view-model is `sf_rpg_ui`, ui.md "Dialogue window view-model"; the journal screen's is `journal-ui.md`), the Bevy plugin (`sf_rpg_bevy`, when the UI slice needs it), the script runtime.

## Research (2026-10-21)

**Industry models.**
- *Condition-filtered response databases* (Morrowind DIAL/INFO, Oblivion/Skyrim topic info
  conditions; Valve's response rules, Ruskin GDC 2012 "AI-driven Dynamic Dialog through Fuzzy
  Pattern Matching"). Dialogue is a table: per topic an ordered list of responses, each with
  conditions on speaker, player and world; the engine picks one. Bethesda picks the **first**
  response whose conditions all pass (authored order is priority); Valve picks the
  **most specific** (most criteria matched). Ordering by specificity at load reduces Valve's rule
  to first-match, so first-match over an adapter-ordered list covers both.
- *Authored graphs* (Yarn Spinner, Ink, Dialogue System for Unity, articy). Conversations are
  node graphs with jumps and inline commands. Good for linear writing; they don't model "any NPC,
  any time, ask about any known topic" without a database layer on top.
- *Keyword hyperlinks* (Morrowind, Ultima VII, Wizardry 8): topic names found in response text
  become clickable and are learned. Standard implementation: a multi-pattern matcher over the
  topic names, case-insensitive, **leftmost-longest**, with word-boundary checks.
- *Quests*: Bethesda's quest = id + current stage index (integer) + finished flag; stages carry
  journal text; the journal is a dated, append-only log grouped by quest. Unreal/Unity quest
  plugins use the same id → stage/objective model.
- *Inventories*: stacks keyed by item definition plus per-instance data (condition, charge,
  enchantment, soul, owner); two stacks merge only when the instance data is equal (Skyrim
  "extra data", OpenMW's `ContainerStore::stacks`). Equipment is a slot map whose slot set and
  occupancy rules (two-handed blocks shield) are game data.

**Co-op (net.md "Quest model").** Quests, journal and known topics are *world* state (host, shared);
inventory and equipment are *character* state (per player). Results go to the context player.
One conversation per NPC (a claim).

## Reuse (Bevy → crates → own)

1. **Bevy** has no RPG, inventory, quest or dialogue features (0.20).
2. **Crates.**
   - `bevy_yarnspinner` (Yarn Spinner, MIT/Apache): authored node graphs; documented for Bevy
     0.13-era releases, no 0.20 release found. Wrong model: our content is ~37k filtered
     responses (Morrowind + expansions), not graphs, and the plugin couples state to Bevy.
   - `bevy_talks` 0.6: graph-of-entities dialogue, "API young and will change", Bevy-coupled.
     Same mismatch.
   - Ink (`bladeink`): story graph runtime; same mismatch, and saves its own state format.
   - Inventory/quest crates on crates.io are small, Bevy-coupled or abandoned; none has per-instance
     stack identity or data-driven slots.
   - **Used:** `aho-corasick` 1.1 (MIT/Unlicense; already in the lock through `regex`) for topic
     linking; `serde`, `smallvec`, `thiserror`, `sf_records::RecordId`.
3. **Own:** the data model and session state machine (~1k lines), Bevy-free so it compiles
   in seconds and is unit-testable without an app.

Projects: OpenMW implements this for Morrowind (GPL: behaviour reference for the adapter only,
never read for this crate). Mudcrab (Skyrim on Bevy) has no dialogue system yet.

## Data model

All ids are `RecordId` (case-folded strings) unless noted. All types derive
`Clone, Debug, PartialEq, Serialize, Deserialize`.

### Players and actors

- `PlayerId(u8)`, 0..`MAX_PLAYERS` (4): the wire/session id (net.md "Players"). `PlayerKey(u128)`:
  the persistent profile key used in saves.
- `ActorKey(u128)`: a stable actor id (the world's `PlacementId` or spawn id; `sf_rpg` does not
  depend on `sf_world`). Never a Bevy `Entity`.
- `Holder`: `Player(PlayerId)` or `Actor(ActorKey)`; who an inventory command targets.

### Items, stacks, inventories

- `ItemStack { id: StackId, item: RecordId, count: u32, attrs: Attrs }`. `StackId(u32)` is unique
  inside one inventory (monotonic) and survives saves; equipment refers to it.
- `Attrs`: a sorted small map `RecordId → AttrValue` (`Int(i64)`, `Float(f32)`, `Id(RecordId)`)
  for per-instance data the adapter defines (`condition`, `charge`, `soul`, `owner`...). Floats
  compare by bit pattern for stack identity, so identity is exact and serde-stable.
- **Stack identity**: two stacks merge iff `item` and `attrs` are equal and neither is held by a
  split-equipped slot (below). `Inventory::add` merges into the first identical stack, else
  appends (insertion order = display order; sorting is the UI's).
- `Inventory::remove(item, n)` takes from **plain stacks (no attrs) first**, then the rest in
  order, and fails without change if fewer than `n` exist (`RemoveError::NotEnough`). Scripted
  removal by id needs no instance choice; UI removal uses `take(StackId, n)`.
- `count(item)`, `stacks()`, `get(StackId)`, `split(StackId, n) -> StackId`. A count past
  `u32::MAX` is an `Overflow` error with no change, never a wrap.

### Equipment

- Slot set is data: `SlotTable` (ordered slot names from the adapter, ≤ 64). `SlotId(u8)` indexes it.
- `EquipRules` (adapter trait): `placement(&ItemStack) -> Option<EquipPlacement>`, where
  `EquipPlacement { slots: SmallVec<[SlotId; 2]>, whole_stack: bool }`: every slot the item
  occupies (a two-handed weapon names weapon + shield) and whether the whole stack equips
  (ammunition, thrown weapons) or one piece is split off (rings, armour).
- `Loadout { inventory, equipped: BTreeMap<SlotId, StackId> }` owns both, so removal and
  equipment can't disagree:
  - `equip(StackId, &rules) -> Result<EquipChange>`: clears every occupied slot (displaced
    stacks are unequipped and merged back where identical), splits one piece off unless
    `whole_stack`, records the stack in each slot. `EquipChange` lists equipped/displaced stacks
    so the adapter can fire its equip events (`OnPCEquip`).
  - `unequip(SlotId)`; `remove`/`take` unequip what they empty. Split-equipped stacks never take
    merges; whole-stack-equipped stacks do (picking up arrows tops up the quiver).
- Requirements (skills, race, beast feet) are rules, checked by the adapter before `equip`.

### Quests

- `QuestLog: BTreeMap<RecordId, QuestState { stage: i32, finished: bool }>`; an absent quest is
  stage 0, not finished.
- `set_stage(quest, stage, flags) -> QuestChange`: stages may go up or down (Bethesda scripts
  set lower stages; the journal never forgets). `StageFlags { finish, restart }`: `finish` sets
  `finished`, `restart` clears it. Returns old/new for events.

### Journal

- `Journal { entries: Vec<JournalEntry>, next_seq }`, append-only.
  `JournalEntry { seq, quest: Option<RecordId>, stage: Option<i32>, source: Option<RecordId>,
  text: String, time: GameTime }`. `source` is the adapter's entry id (an INFO key) so the UI can
  re-read localised text; `text` is the snapshot with macros expanded at write time (the fallback
  when a mod removed the source).
- `GameTime { day: u32, minute: u16 }`: days since the game start and minute of day. The calendar
  (month names, year) is the adapter's.
- Queries: `entries()`, `for_quest(id)`, `quests()` (quests with entries, in first-entry order).
- **Known topics**: `Topics: BTreeMap<RecordId, KnownTopic { learned: u32, heard:
  Vec<HeardLine> }>`; `learned` is the journal sequence clock at learning (stable ordering);
  `heard` holds the responses shown under this topic (`source`, `text`, `speaker` name), the
  journal's "topics" tab.

### World and character state

- `WorldRpg { quests: QuestLog, journal: Journal, topics: Topics }`: shared, host-owned.
- `CharacterRpg { loadout: Loadout }`: one per player character (and per NPC that carries items).
  Stats arrive in M7.

## Commands and events (net rules 2, 5, 8)

State changes go through `RpgCommand`, applied by the host on its tick; each application
returns `RpgEvent`s (data) for UI, replication and scripts.

| Command | Effect | Events |
|---|---|---|
| `SetStage { quest, stage, flags, entry: Option<EntryDraft> }` | quest stage + optional journal entry | `StageChanged`, `JournalAdded` |
| `AddTopic(id)` | learn a topic (idempotent) | `TopicLearned` (first time only) |
| `HearLine { topic, line }` | append a heard response (learns the topic) | `TopicLearned` (first time only) |
| `AddItem { holder, item, count, attrs }` / `RemoveItem { holder, item, count }` | inventory change | `ItemsChanged { holder, item, delta }` |

`WorldRpg::apply` handles world commands; item commands are applied to the target
`CharacterRpg` by the caller, which resolves `Holder` (the Bevy layer owns the holder map).
Script results (MWScript, Lua) and UI requests produce commands; nothing mutates state
directly. Events carry the stable ids (`Holder`, quest, topic), never entities.

## Dialogue

### Database

`DialogueDb` (built by the adapter at load, not saved):
- `topics: IndexMap<RecordId, TopicEntry { kind: TopicKind, name: String, responses: Vec<RecordId> }>`;
  `responses` are in **priority order** (the adapter resolves load order and specificity).
- `TopicKind`: `Conversation` (listed and linkable), `Greeting`, `Service` (persuasion and
  service replies: admire, bribe, refusal), `Bark` (voice, outside the session).
- `greetings: Vec<RecordId>`: greeting topics in evaluation order.

### Adapter hook

```text
trait DialogueRules {
    fn accepts(&self, ctx: &ConversationCtx, topic: &RecordId, response: &RecordId) -> bool;
    fn render(&self, ctx: &ConversationCtx, topic: &RecordId, response: &RecordId) -> String;
    /// Default None. A (topic, response) to show when nothing under `topic` is accepted
    /// (a refusal line); such a topic still counts in `available_topics`. Not for greetings.
    fn fallback(&self, ctx: &ConversationCtx, topic: &RecordId) -> Option<(RecordId, RecordId)>;
}
```

`ConversationCtx { player: PlayerId, speaker: ActorKey, speaker_base: RecordId, choice:
Option<i32> }`. The adapter's implementation holds read access to world and character state
(quests, inventories, globals, its own NPC state) for its filters; `sf_rpg` only orders the
search. `render` expands text macros. Selection: **the first response in the topic's list that
`accepts`**; greetings: the first accepted response across `greetings` in order.

### Session state machine

`DialogueSession` (serde; one per conversation): `Greeting → Talking ⇄ Choosing → Ended`, with a
`Goodbye` sub-state (only "goodbye" allowed).

- `open(db, rules, ctx)` selects a greeting; `ask(topic)` selects a response under a topic;
  `choose(value)` stores the choice in the context and re-asks the **current topic** (the
  Bethesda convention: the choice is a filter input), then clears it; `goodbye()` ends.
- Each step returns `Vec<DialogueOutput>`: `Line { topic, response, text }` (show, append to
  history, `HearLine`), `RunResult { topic, response }` (the host runs the response's result
  script as the speaker, context player = `ctx.player`), `Learned(topic)` (topic names linked in
  the text: emit `AddTopic`), `NoResponse { topic }`.
- Result scripts call back: `offer_choices(Vec<Choice { label, value }>)` → `Choosing`;
  `force_goodbye()` → `Goodbye`. Asking a topic while `Choosing` or `Goodbye` is an error.
- `available_topics(db, rules, known)`: known `Conversation` topics with at least one accepted
  response for this speaker, in name order (the topic list).
- `history`: the shown lines (scrollback), kept for the UI and for net replication of the
  conversation to its player only.

**Co-op:** `Conversations` is the host's claim table: `claim(speaker, player)` fails with `Busy`
if another player talks to that NPC (net.md "Concurrency"); `release` on goodbye/disconnect.
Sessions run on the host; the client sends `ask/choose/goodbye` requests and receives outputs.

### Topic links

`TopicLinker::new(names, LinkRules { word_start: bool, word_end: bool })` builds an Aho-Corasick
automaton over topic display names, ASCII case-insensitive. `link(text) -> Vec<TopicSpan {
range, topic }>`: all matches, filtered by the boundary rules (a boundary = text edge or a
non-alphanumeric char), then **leftmost-longest without overlap**. Built once per database
(learned/unknown filtering is the caller's: the session links against all `Conversation` topics,
learns what it finds and the UI styles spans). Non-ASCII case folding is not applied (content
topics are ASCII; revisit for localised packs).

## Adapter hooks (summary)

| Hook | Supplies |
|---|---|
| `DialogueDb` builder | topics, kinds, names, ordered responses, greeting order |
| `DialogueRules` | filter evaluation (`accepts`) and text macros (`render`) |
| `EquipRules` + `SlotTable` | slot set, item placement, whole-stack rule |
| `LinkRules` | word-boundary policy for topic links |
| attribute keys | what per-instance data exists (`Attrs` keys) |
| result scripts | turn `RunResult` into `RpgCommand`s and session callbacks |

## Co-op (net-ready rules)

1, 4: no clock or RNG inside `sf_rpg`; `GameTime` comes from the caller. 2, 5: commands and
host-applied requests only. 3: plain serde data, no presentation. 6: `PlayerId` in every context;
no singleton player. 7: `ActorKey`, `StackId`, `RecordId`, never `Entity`. 8: `RpgEvent` is data.
`PlayerId` lives in the serde-only `sf_ids` crate (2026-10-21); `sf_rpg` and `sf_action` re-export
it, and `sf_net` (N2) will re-export it rather than own it.

## Tests

Unit tests per module: stack merge identity (attrs, float bits), remove order and failure
atomicity, split, equip/displace/whole-stack/merge-back, quest flags, journal ordering and
queries, topic learning idempotence, linker (longest match, boundaries, overlap, case), session
greeting order/first-match/choice re-ask/goodbye/claim, postcard round-trips of every saved type.

## Open questions

1. (Resolved 2026-10-21: `PlayerId` lives in `sf_ids`.)
2. Journal text snapshot vs re-render on display for localisation (snapshot kept as fallback).
