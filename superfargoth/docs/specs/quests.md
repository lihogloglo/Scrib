# Quests: status, objectives, aliases, facts, tracking (`sf_rpg`, `sf_rpg_ui`)

Status: **spec (DQ-16, 2026-11-26)**, nothing built beyond the flat quest store (rpg.md "Quests", as built:
`QuestLog` of stage + finished, `superfargoth/crates/sf_rpg/src/quest.rs:34`). Lane roadmap:
`docs/roadmaps/dialogue-quests.md` Stage 3 (DQ-17..DQ-21), decisions 4, 7, 9. Related: [rpg.md](rpg.md) (journal,
commands, `WorldRpg`), [journal-ui.md](journal-ui.md) "Quest log and tracked quest" (view-models),
[map.md](map.md) "Compass (MAP4)" and "Guidance" (the one compass and the one route), [save.md](save.md),
[tunables.md](tunables.md). Game side: `bevyrrowind/docs/specs/rpg-adapter.md` "Quests (DQ-18)".

> **Agent summary.** The flat stage store stays (Morrowind needs it, it is free) and gains a generic layer in data:
> a quest has a **status** (`Active | Done | Failed`; absent = not started), **objectives** (`Hidden | Active | Done
> | Failed`, optional, with a `TargetRef`) and **aliases** (named slots filled by a fixed ref, a command, later a
> query). Definitions are `sf.quest` records in a `QuestDb` (not saved); state lives in `WorldRpg.quests` (host,
> shared, `sf/rpg` v2). Every change is an `RpgCommand` and comes back as one `RpgEvent::QuestChanged` per changed
> fact; dialogue rules, AI and UI read the same **quest facts** (`quest.<id>.stage/status/objective/alias`). A quest
> with no defined objectives has one **implicit objective**: its latest journal entry. The **tracked quest** is per
> player (`PlayerQuests`, character save), presentation only. Markers follow the `quests.markers` setting (`None |
> Compass | Guidance`): objectives become one more marker source of map.md's compass, never a second strip.

## Goal

Quest progress any pack can author as data and any UI can show: started / active / done / failed, a short list of
what to do next, who or where it is about, one tracked quest per player on the HUD and on the compass. Non-goals:
a quest graph runtime (stage transitions on fact conditions are rules of the rule database, DQ-22/23, or the
adapter's scripts), radiant quest templates (alias `Query` fill is a seam only), scenes (dialogue lane Stage 5),
routes (map.md "Guidance").

## Reuse check (CONTRIBUTING rule 0)

Checked 2026-11-26.

1. **Bevy 0.20** has no quest, objective or journal feature. Its parts we use: messages for `RpgEvent` delivery and
   the UI widgets (in `sf_ui_screens`); the core stays Bevy-free (rpg.md "Reuse").
2. **Crates** (crates.io API, 2026-11-26):
   - `bevy_quests` 0.1.0 (MIT, KBVE monorepo, 2026-07, 48 downloads): typed structs generated from a protobuf schema
     plus a searchable registry; Bevy ^0.19, prost build step, no runtime state machine, objectives, aliases or saves.
     Wrong shape (a registry, not a state model) and Bevy-coupled.
   - `micro_quest` 0.1.0 (2023, abandoned): dialogue + quest structs for an old Bevy.
   - `bevy_fact_rule_event` 0.4.1 (MIT/Apache, Bevy ^0.18): a fact-rule-event loop; relevant to the rule database
     (DQ-22 reuse check), not to quest state; stuck on 0.18.
   - Engine-agnostic quest crates: none maintained with objectives + aliases + serde state.
   - **Used:** what `sf_rpg` already has (`serde`, `smallvec`, `thiserror`, `sf_records::RecordId`, `sf_ids`).
3. **Own:** ~400 lines in `sf_rpg::quest` (status, objectives, aliases, facts, `QuestDb` expansion) and ~300 in
   `sf_rpg_ui` (quest log rows, tracked view, marker list). Projects: OpenMW (Morrowind quests, behaviour reference
   for the adapter only); Mudcrab has no quest system.

## Industry comparison

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Integer stages with log text, stage reached sets state | Creation Kit quest stages; Morrowind journal index | `QuestLog` stage + `StageDef` effects | **match** |
| Objectives shown / completed / failed per stage | CK `SetObjectiveDisplayed/Completed/Failed`; Witcher 3 objectives; Cyberpunk objective sets per phase (GDC 2023) | `ObjectiveStatus`, `SetObjective`, `StageDef.objectives` | **match** |
| Optional objectives | Witcher 3, Cyberpunk, KCD2 | `ObjectiveDef.optional` | **match** |
| Quest success / failure states | Witcher 3 journal (active / success / failed); CK `CompleteQuest`, `FailAllObjectives`; Cyberpunk failed tab | `QuestStatus::{Active, Done, Failed}` | **match** |
| Aliases (named slots: specific ref, forced, find matching) | CK reference aliases (Specific Reference, Forced, Find Matching Reference) | `AliasFill::{Fixed, Forced, Query}`; `Query` waits for the rule DB | **match** (Fixed, Forced) / **gap** (Query: seam) |
| "Stage done" memory (a stage's fragment runs once) | CK `GetStageDone` | effects apply when the stage value changes | **differ**: enough for linear stages; add a done set when content needs it |
| Quest state readable by dialogue conditions | CK `GetStage`, `GetQuestCompleted`, `GetIsAliasRef`; Valve facts (Ruskin GDC 2012) | quest facts (below) | **match** |
| Several quest ids in one log entry | OpenMW quest book groups journal topics by quest name (`apps/openmw/mwgui/journalviewmodel.cpp:155-183`, 4e718b50) | `QuestDef.group` | **match** |
| One tracked quest, objective tracker on HUD | Witcher 3, Cyberpunk (one tracked); Skyrim (several, toast on change) | `PlayerQuests.tracked`, `quests.track.max` (1), HUD `hud.quest` slot | **match** |
| Auto-track new / next quest | Witcher 3, Cyberpunk (memory) | `quests.track.auto_new`, `auto_next` | **match** |
| Compass markers for tracked objectives | Skyrim, Witcher 3, Cyberpunk mappins | objective marker source into map.md MAP4 | **match** |
| Marker points at the door when the target is in another space | Skyrim, Witcher 3 | `TargetPositions` may return a portal proxy; default none (no marker) | **gap**: needs travel/nav portal query |
| No markers, directions in text; guidance instead | Morrowind; KCD2, Elden Ring (memory) | `quests.markers = None / Guidance` | **match** |
| Co-op: shared quest progress, per-player UI | BG3, Divinity: Original Sin 2 (shared journal, memory) | `WorldRpg` shared, `PlayerQuests` per player | **match** |
| Morrowind status from QSTF / QSTR, restart reopens same-name quests | OpenMW `mwdialogue/quest.cpp:69-95`, `journalimp.cpp:81-115` (4e718b50) | rpg-adapter "Quests (DQ-18)" | **match** |

## Data model (`sf_rpg::quest`, Bevy-free)

Definitions (records `sf.quest`, built into a `QuestDb` at load like `DialogueDb`; not saved):

```rust
pub struct QuestDef {
    id: RecordId, name: String,               // display name; localised through JournalLabels::quest_name
    kind: Option<RecordId>,                   // category (main, faction, side...): data the UI groups and sorts by
    group: Option<RecordId>,                  // quests sharing a group show as one quest-log row
    listed: bool,                             // false: never in the quest list (bookkeeping quests)
    objectives: Vec<ObjectiveDef>,            // authored order = display order
    stages: Vec<StageDef>,                    // sorted by stage
    aliases: Vec<AliasDef>,
}
pub struct ObjectiveDef { id: RecordId /* scoped to its quest */, text: String, optional: bool, target: Option<TargetRef> }
pub struct StageDef { stage: i32, status: Option<QuestStatus>, objectives: Vec<(RecordId, ObjectiveStatus)> }
pub struct AliasDef { name: RecordId, fill: AliasFill, optional: bool }
pub enum AliasFill { Fixed(TargetRef), Forced /* by command or script */, Query(RecordId) /* criteria set, DQ-23 */ }
pub enum TargetRef {
    Ref(ActorKey),                            // a placed reference or spawned actor (placement or spawn id)
    Place(RecordId),                          // a map place (map.md `MapPlace`)
    Point { layer: RecordId, pos: [f32; 3] }, // a spot on a map layer (worldspace id)
    Alias(RecordId),                          // this quest's alias; resolves to its fill
}
```

A quest started with no `QuestDef` (flat content, or a def with empty `objectives`) is valid: it has stage, status
and journal only. `QuestDb::get(id)`, `QuestDb::group_members(group)`, `QuestDb::alias_target(world, quest, name)`.

State (in `WorldRpg.quests`, host, saved):

```rust
pub enum QuestStatus { Active, Done, Failed }            // absent from the log = not started
pub enum ObjectiveStatus { Hidden, Active, Done, Failed }  // absent = Hidden
pub struct QuestState {                                    // replaces { stage, finished } (quest.rs:11)
    stage: i32, status: QuestStatus,
    objectives: BTreeMap<RecordId, ObjectiveStatus>,       // only non-Hidden entries
    aliases: BTreeMap<RecordId, TargetRef>,                // filled aliases
    updated: u32,                                          // WorldRpg change clock at the last change ("recent" sort, auto_next)
}
pub struct StageFlags { status: Option<QuestStatus> }      // replaces { finish, restart } (quest.rs:18)
```

- `QuestLog` keeps its map plus a change clock (`u32`, +1 per changed quest, saved) that stamps `updated`;
  `get(id) -> Option<&QuestState>`, `stage(id)` (0 when absent, as built), `status(id) ->
  Option<QuestStatus>`, `objective(id, obj) -> ObjectiveStatus`, `alias(id, name) -> Option<&TargetRef>`,
  `aliases_of(&TargetRef) -> impl Iterator<(quest, alias)>` (reverse lookup for "speaker fills alias" criteria).
  `QuestState` is no longer `Copy`; `QuestChange` (`quest.rs:26`) becomes `QuestDelta` (below).
- **Starting.** The first command that touches an absent quest (`SetStage`, `SetQuestStatus Active`, `SetObjective`,
  `FillAlias`) inserts it `Active` and emits `Started` first. `Fixed` aliases of its def are filled at start.
- **Stage on a closed quest.** `SetStage` changes the stage of a `Done`/`Failed` quest without reopening it unless
  its flags say `Active` (Morrowind restart; journal never forgets, rpg.md "Quests").
- **Closing** leaves objective states as they are (the log shows them as history); markers and the tracked line
  only use `Active` objectives of `Active` quests.
- **Implicit objective.** `objectives(world, db, quest) -> Vec<ObjectiveView>`: the def's non-Hidden objectives in
  def order; when the def has none (or no def), one `ObjectiveView { key: Implicit, text: latest journal entry
  text for the quest, status: Active if the quest is Active else Done/Failed, optional: false, target: None }`, or
  nothing when the quest has no entry. One function, read by the journal, HUD and markers (decision 7).

## Commands and events

Extends rpg.md "Commands and events" (`sf_rpg/src/state.rs:28`, `:72`). Host-applied on its tick (net rules 2, 5).

| Command | Effect | Events (`QuestChanged` delta, in order) |
|---|---|---|
| `SetStage { quest, stage, flags, entry }` (as built, `flags.status` new) | stage; status if `flags.status`; journal entry | `Started`?, `Stage`, `Status`?, then `JournalAdded` |
| `SetQuestStatus { quest, status }` | start, complete, fail, reopen | `Started`?, `Status` |
| `SetObjective { quest, objective, status }` | show, complete, fail, hide one objective | `Started`?, `Objective` |
| `FillAlias { quest, alias, target: Option<TargetRef> }` | fill or clear an alias (`Forced`) | `Started`?, `Alias` |

```rust
RpgEvent::QuestChanged { quest: RecordId, delta: QuestDelta }   // replaces StageChanged (only state.rs emits it today)
pub enum QuestDelta {
    Started,
    Stage { old: i32, new: i32 },
    Status { old: Option<QuestStatus>, new: QuestStatus },
    Objective { id: RecordId, old: ObjectiveStatus, new: ObjectiveStatus },
    Alias { name: RecordId, old: Option<TargetRef>, new: Option<TargetRef> },
}
```

- One event per changed fact; a command that changes nothing emits nothing (idempotent replays, net rule 8).
- **`QuestDb` expansion.** `WorldRpg::apply_quest(&mut self, db: &QuestDb, cmd)` = `QuestDb::expand(world, cmd)`
  then `apply` per command: a `SetStage` that changes the stage of a defined quest with a `StageDef` at the new
  stage appends that def's `SetQuestStatus` and `SetObjective` commands (CK stage objectives); unknown objective or
  alias ids of a defined quest are `RpgError::UnknownObjective` / `UnknownAlias` with no change (atomic per
  expanded batch). `WorldRpg::apply` (no db) stays for flat content and accepts any id.
- Producers: adapter scripts (MWScript `Journal`, Lua later), dialogue results, the rule database's writes (DQ-23),
  and dev commands. Nothing mutates `QuestLog` directly.

## Quest facts (decision 4)

World facts the rule database (DQ-22/23), dialogue filters, AI and UI read; values are integers or a ref so a
criterion is a comparison. Keys are concepts (`QuestFact`), the text form is the authoring syntax:

| Key (text form) | `QuestFact` | Value |
|---|---|---|
| `quest.<id>.stage` | `Stage(quest)` | `i32` (0 when not started) |
| `quest.<id>.status` | `Status(quest)` | 0 not started, 1 active, 2 done, 3 failed |
| `quest.<id>.objective.<obj>` | `Objective(quest, obj)` | 0 hidden, 1 active, 2 done, 3 failed |
| `quest.<id>.alias.<name>` | `Alias(quest, name)` | the filled `TargetRef` or none (equality criteria: "speaker is the alias") |
| `<subject>.alias_in.<id>` | `FillsAlias(quest, who)` | 1 when `who` fills any alias of the quest (`aliases_of`; CK `GetIsAliasRef`) |

`quest_fact(&WorldRpg, &QuestDb, &QuestFact) -> FactValue` is pure; DQ-23's `Facts` trait wraps it, and its fact
invalidation reads `QuestChanged` deltas (each delta names its fact). The tracked quest and `JournalSeen` are **not**
facts: presentation never feeds gameplay (CONTRIBUTING "Net-ready").

## Tracked quest (per player)

```rust
pub struct PlayerQuests { tracked: SmallVec<[RecordId; 1]> }   // quest ids, most recent first, <= quests.track.max
pub enum TrackCommand { Track(RecordId), Untrack(RecordId), Clear }
```

- `PlayerQuests::apply(cmd, world, db, max)`: `Track` puts the quest first (only an `Active`, listed quest; a group
  row tracks its most recently updated `Active` member) and drops the oldest past `max`.
- **Follow-ups** after each `QuestChanged` (`PlayerQuests::on_quest_changed`): a tracked quest that closed is
  dropped; with `quests.track.auto_next`, the player then tracks the most recently updated `Active` listed quest
  (same group first); with `quests.track.auto_new`, a `Started` listed quest is tracked when nothing is.
- Ownership: per-player presentation state (decision 9). The client applies `TrackCommand` at once (no round trip)
  and sends it to the host only so the save carries it; the host never reads it for gameplay. Saved in the
  player's `CharacterSave` (Saves).

## Markers (decision 7)

`quests.markers` (`QuestMarkers::{None, Compass, Guidance}`) picks how objectives reach the world view:

- `None`: no objective markers; directions are the journal text (Morrowind both presets).
- `Compass` (framework default): `objective_markers(world, db, players, &dyn TargetPositions, settings) ->
  Vec<ObjectiveMarker { player, quest, objective, layer, pos, tracked }>` in `sf_rpg_ui`: `Active` objectives with
  a target of the player's tracked quests (all `Active` quests when `quests.markers.tracked_only` is false), capped
  at `quests.markers.max`. map.md's compass (MAP4) draws them as one more marker kind next to places, pins and
  party; this spec adds no strip, no bearing maths and no fade rules of its own.
- `Guidance`: as `Compass`, plus map.md "Guidance" routes to the tracked objective's target (TV20 owns the route).
- `TargetPositions` (adapter/shell hook, client side): `resolve(&TargetRef, player) -> Option<(layer, pos)>`: a
  `Ref` from its live transform or its placement (overlay) position when not streamed, a `Place` from `sf_map`, a
  `Point` as is, an `Alias` through `QuestDb::alias_target`. A target on another layer returns `None` until a
  portal query exists (gap above); the objective still shows in the journal and on the HUD line.
- Cost: resolution once per tick for <= `quests.markers.max` targets per player, under 0.01 ms (inside the HUD
  0.5 ms budget, map.md "Compass").

## Co-op and saves

- **Shared:** `WorldRpg.quests` (status, objectives, aliases) is host world state, replicated as `QuestChanged`
  deltas to every client (BG3 shared journal). **Per player:** `PlayerQuests` (tracked), `JournalSeen` (profile,
  journal-ui.md), markers (computed client-locally from replicated state). No clock or RNG in `sf_rpg`; `updated`
  is a change counter, not time.
- **`sf/rpg` v2** (`sf_save_bevy/src/sections/mod.rs:54`, `RPG_VERSION` 1 -> 2): `QuestState` above. v1 -> v2:
  `finished: true` -> `Done`, else `Active`; no objectives, no aliases; `updated` = the quest's last journal seq,
  else 0. Golden shape test updated (`sf_save_bevy/src/tests_formats.rs:99`). Load: an objective or alias id the
  def no longer has is dropped with one log line (a mod changed the quest); an unknown quest id is kept (dormant
  mod data, save.md).
- **`CharacterSave` v5** (`sections/character.rs:37`, `CHARACTER_VERSION` 4 -> 5, or the next free version when
  built): field `quests: PlayerQuests`; v4 -> v5 tracks nothing (auto_next picks on the next change). A tracked id
  that is no longer `Active` on load is dropped.

## Tunables

Prefix `quests.`; a game sets defaults by base-mod record, per preset where marked. All `Net::Presentation`.

| Name | Default | Range | Unit | Source |
|---|---|---|---|---|
| `quests.markers` (preset) | `Compass` | None/Compass/Guidance | enum | Owner(2026-11-25; Morrowind `None`) |
| `quests.markers.tracked_only` | true | bool | none | Reference(Witcher 3, Cyberpunk: tracked quest only) |
| `quests.markers.max` | 8 | 1-64 | count | Guess |
| `quests.track.max` | 1 | 1-8 | count | Reference(Witcher 3, Cyberpunk: one tracked quest) |
| `quests.track.auto_new` | true | bool | none | Reference(Witcher 3, Cyberpunk auto-track, memory) |
| `quests.track.auto_next` | true | bool | none | Guess |
| `quests.hud.tracked_line` (preset) | `Always` | Off/OnChange/Always | enum | Reference(Witcher 3 always; Skyrim on change) |
| `quests.hud.lines` | 3 | 1-8 | lines | Guess (objective text clamp, ellipsis) |
| `quests.hud.max_objectives` | 3 | 1-8 | count | Reference(Witcher 3, Cyberpunk show 1-3) |
| `quests.hud.flash_s` | 4 | 0-30 | s | Guess (highlight after a change; `OnChange` shows the line this long) |
| `quests.hud.fade_s` | 0.4 | 0-3 | s | Guess |

`const`: status and objective encodings of the facts, save versions.

## Presets and the classic seam

- **Modern** (framework and Modern Morrowind): journal quest tab (list + detail with objectives), tracked line on
  the HUD, markers per `quests.markers` (Morrowind `None`).
- **Classic** (Morrowind Classic): the book journal as built (journal-ui.md, DJ-1), `quests.hud.tracked_line = Off`,
  `quests.markers = None`, no track action. Quest state is the same in both: the seam is presentation only.

## Tests

Exact, synthetic defs: start rules (each command starts an absent quest, `Started` first); `SetStage` on a closed
quest keeps it closed unless flags say `Active`; `StageDef` expansion order and atomic failure on an unknown
objective; one event per changed fact and none for no-ops; implicit objective = latest entry text and follows
status; alias `Fixed` fill at start, `FillAlias` clear, `aliases_of`; facts table values; `PlayerQuests` track/max/
auto_new/auto_next/close; `objective_markers` (tracked only, max, `None` hides all, unresolved target skipped);
postcard round trip of every saved type; `sf/rpg` v1 -> v2 migration.

## Slices

DQ-17 core (`sf_rpg`: types, commands, events, facts, `QuestDb`, `PlayerQuests`, `sf/rpg` v2 and `CharacterSave`
v5, caller updates in `mw_rpg`/`mw_rpg_bevy`/`sf_rpg_ui` for the renamed fields), DQ-18 Morrowind mapping
(rpg-adapter.md), DQ-19 view-models (journal-ui.md), DQ-20 screens, DQ-21a marker source, DQ-21b compass hookup
(needs map.md MAP4). Rows in `docs/roadmaps/dialogue-quests.md` Stage 3.
