# Dialogue rules: facts, criteria, selection, memory, barks (`sf_rpg::rules`, `sf_rpg::barks`)

Status: **spec (DQ-22, 2026-11-26)**, nothing built. As built today: per topic an ordered response list and "first
response the adapter `accepts`" (`superfargoth/crates/sf_rpg/src/dialogue.rs:88` `select`, `:93` `select_greeting`,
`:112` `DialogueRules`). Lane roadmap: `docs/roadmaps/dialogue-quests.md` Stage 4 (DQ-23..DQ-25, DQ-39), decisions 4,
5, 9, 10. Related: [rpg.md](rpg.md) "Dialogue" (session machine, the session-facing contract), [quests.md](quests.md)
"Quest facts", [ai.md](ai.md) "Interrupt contract" (`BarkCue`, AI30), [ai-memory.md](ai-memory.md) (facts NPCs know,
reputation), [world-reactions.md](world-reactions.md) (`ConcernRules` Bark rung), [crime.md](crime.md) (`OffenceDef.say`),
[tunables.md](tunables.md), [save.md](save.md). Game side: `bevyrrowind/docs/specs/rpg-adapter.md` "Rule compilation
(DQ-24) and Voice barks (DQ-25)".

> **Agent summary.** Line selection is a **rule database** (Ruskin GDC 2012): the caller fires a **query** (a concept
> plus who, plus event facts); rules of the query's concept are matched against **facts** read through one `Facts`
> trait (`speaker.*`, `listener.*`, `player.*`, `world.*`, `quest.*`, `event.*`, `memory.*`, adapter `mw.*`); the
> winner is the highest **priority**, then the most **specific** (sum of criterion weights), then the **least
> recently used**, then a **seeded pick** (hash of host tick, speaker, concept). A winning rule may be **once** or on
> **cooldown** and **writes** only `memory.*` facts; these live in `DialogueMemory` (host, `sf/dialogue` v1). Morrowind
> INFOs compile to rules with a unique priority per INFO in OpenMW order, so classic selection is identical (DQ-24
> equivalence test). Barks: emitters send `BarkCue` (owned by ai.md, AI30); `sf_rpg::barks` gates, dedupes, budgets
> and queries, and hands `BarkLine`s to the `Barks` presenter. Quick replies (DQ-39) and interrupts (DQ-31) are more
> queries on the same engine. Selection is pure, host-only, no RNG state, never per frame.

## Goal

One selection engine for every line an NPC says: conversation topics and greetings, barks (greet, combat, flee,
theft, trespass, idle), ambient reactive lines (quest state, weather, reputation), quick-reply offers and the
fallback line when a conversation is interrupted. Writers add lines as data with no code; a content pack's own
filter language (Morrowind INFO) compiles to the same rules. Non-goals: authored scenes and line IDs (DQ-34),
presentation (DQ-26: presenters, camera, pacing), AI deciding *when* to speak (ai.md, AI30), LLM lines (DQ-38 seam).

## Reuse check (CONTRIBUTING rule 0)

Checked 2026-11-26.

1. **Bevy 0.20** has no dialogue, rule or fact system. Used in the thin plugin layer only: `Message`s carry
   `BarkCue` in and `BarkLine` out, the fixed tick runs the bark queue. The engine is Bevy-free (rpg.md "Reuse").
2. **Crates** (crates.io API 2026-11-26):
   - `yarnspinner` / `bevy_yarnspinner` 0.10.0 (MIT OR Apache-2.0, 2026-11-10, Bevy ^0.20): a story VM; its README
     lists Yarn 2.x compatibility, and a GitHub code search for "saliency" in YarnSpinner-Rust found nothing (DQ-33
     verifies the compiler). Even with Yarn 3 saliency, selection is per node group inside its VM, not a world-wide
     query over 37k rules and other systems' state. Kept for authored scenes (DQ-33/36), not selection.
   - `bevy_fact_rule_event` 0.4.1 (MIT OR Apache-2.0, 2026-04-27, Bevy ^0.18, RON rules via Bevy assets): an
     event -> rule -> fact-write loop over its own central fact database (README). No most-specific pick, priority,
     recency or seeded tie-break; owns state we keep in quests, AI memory and stats; stuck on Bevy 0.18 and coupled
     to its asset loader.
   - Business rule engines (`zen-engine` 2.1.8 MIT, `datalogic-rs` 5.9 JSONLogic, `dataflow-rs` 4.0): decision
     tables or JSON expressions evaluated one rule at a time; no bucketed best-match over thousands of rules, no
     memory, heavy dependency trees. Prolog-style crates (`logru`): wrong model (proof search, not ranking).
   - **Used:** what `sf_rpg` already has (`serde`, `smallvec`, `indexmap`, `thiserror`, `sf_records::RecordId`).
3. **Own:** ~900 lines in `sf_rpg::rules` (keys, rules, index, selection, memory) and ~350 in `sf_rpg::barks`
   (queue, gates, budget). Ruskin's design is small and the cost is in data, not code. Projects: OpenMW has no rule
   engine (first-match INFO lists, adapter reference only); Mudcrab has no dialogue.

## Industry comparison

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Query (concept + fact bag) against a rule DB, most criteria matched wins | Valve response system: L4D, L4D2, Portal 2, TF2 (Ruskin GDC 2012) | `Query`, `RuleDb::select`, specificity = criterion weight sum | **match** |
| Criteria are key + range / equality; rules bucketed by concept, sorted best-first, first full match ends the scan | Ruskin GDC 2012 (sorted buckets, early out) | `Bucket` sorted by rank, speaker sub-bucket, tie-group early out | **match** |
| Rule writes memory facts, optionally expiring ("applycontext" with a duration) | Valve `response_rules.txt` (TF2, L4D) | `FactWrite` into `memory.*`, `expires` | **match** |
| Speak once, respeak delay, follow-up concept (`then`) | Valve response groups (`speakonce`, `respeakdelay`, `then`) | `once`, `cooldown`, `FollowUp` | **match** |
| Per-response random odds | Valve `odds` | emitter side (cue odds: OpenMW `iVoiceHitOdds`); a rule may test `event.roll` | **differ**: odds belong to the trigger, same result |
| Priority tiers above specificity, "play once", random among top eligible | Hades (Supergiant, GDC 2021; memory) | `priority`, `once`, seeded pick | **match** |
| Conditions count as complexity, "random best least recently viewed" | Yarn Spinner 3 saliency (docs.yarnspinner.dev node groups, saliency) | specificity, then least-recent, then seeded | **match** |
| Valve-derived reactive radio dialogue with memory and interruption | Firewatch (Campo Santo, GDC 2017; memory) | rules + `memory.*` + `talk.interrupted` query (DQ-31) | **match** |
| Condition functions per info, first passing in authored order; Random flag among siblings; Say Once | Creation Kit (Skyrim, Fallout 4) | priority = order; equal priority = random group; `once` | **match** |
| First passing INFO in PNAM order; Info Refusal on a disposition-only miss; voice `say` refuses while speaking | Morrowind / OpenMW `mwdialogue/filter.cpp:736-786`, `dialoguemanagerimp.cpp:625-664` (4e718b50) | compiled rules, unique priority, `soft` criteria, one line per actor | **match** (equivalence test) |
| Pick the best speaker among candidates for a group event | L4D (query every candidate, best score speaks; Ruskin) | dedupe cues per concept and area, nearest-listener candidate wins | **gap**: no per-candidate scoring pass; add when crowd barks sound wrong |
| Bark priority and interruption, global concurrency cap | L4D speech priority; RDR2, Cyberpunk ambient lines (memory) | `BarkUrgency`, `barks.max_concurrent`, pre-empt lower urgency (Modern) | **match** |

## Facts

`Facts` is the one read path: every value a criterion tests comes through it, owned by the system that owns the
state (quests, AI memory, stats, the adapter's world view). Keys follow quests.md "Quest facts": lowercase, dotted,
`<namespace>.<path>`, parameters as segments.

```rust
// sf_rpg::rules (Bevy-free). Illustrative.
pub struct FactKey(u32);                      // interned at load by FactTable; criteria compare keys as integers
pub enum FactValue { Missing, Int(i64), Float(f32), Id(RecordId), Ref(TargetRef) }  // TargetRef: quests.md
pub enum FactSpec {                           // parsed once per key at intern time
    Quest(QuestFact), Speaker(Sym), Listener(Sym), Player(Sym), World(Sym), Event(Sym),
    Memory { scope: MemScope, name: Sym }, Ext(u32) /* adapter namespace, resolved by FactResolver */,
}
pub trait Facts { fn get(&self, q: &QueryCtx, key: FactKey, spec: &FactSpec) -> FactValue; }
pub trait FactResolver { fn resolve(&self, text: &str) -> Option<u32>; }  // adapter keys, e.g. `mw.*`
```

| Namespace | Subject | Framework keys (adapters add more under the same prefix) | Owner |
|---|---|---|---|
| `speaker.` | the actor saying the line | `id` (base record), `ref`, `npc` (0/1), `race`, `class`, `faction`, `rank`, `female`, `place`, `health_pct`, `in_combat`, `disposition` (toward the context player), `talked_to` (0/1), `var.<name>` (its script variable), `knows.<fact_kind>` (best confidence 0-255 about the player), `busy` (speaking now), `can_speak` (0 swimming, unconscious, dead) | adapter view; `knows` sf_ai |
| `listener.` | who it is said to (player or actor) | same keys as `speaker.`; `is_player` (0/1) | adapter view |
| `player.` | the context player (`ctx.player`, never a singleton) | `level`, `gold`, `item.<id>` (count), `faction.<id>.rank` (`Missing` when not a member), `stat.<id>`, `crime`, `rep.here` (band -2..2, speaker's home settlement), `rep.faction.<id>` | adapter, sf_stats, sf_ai (AI44) |
| `world.` | host world | `var.<id>` (global variable), `hour`, `day`, `weather`, `dead.<id>` (death count), `raining`, `night` (ai.md `WorldState` keys) | adapter, sky |
| `quest.` | quests.md "Quest facts" | `<id>.stage/.status/.objective.<obj>/.alias.<name>`; `<subject>.alias_in.<id>` as `speaker.alias_in.<id>` | sf_rpg quests |
| `event.` | the query's own context (eager, from the emitter) | `reason`, `target`, `offence`, `damage`, `distance_m`, `roll` (0-99, emitter-seeded), `choice` (pending choice value), `topic` | emitter |
| `memory.` | this engine's writes | `memory.world.<name>`, `memory.speaker.<name>`, `memory.player.<name>` | `DialogueMemory` |

- `event.*` values arrive in the query (`Query.event`, ≤ 8 pairs, Valve's query bag); everything else is read lazily
  and memoised per query (`QueryMemo`: 16 inline slots, then a map), so a rule's cost is the facts it tests.
- `Missing` is a value: each criterion says what it does on `Missing` (Morrowind: a missing global passes, a missing
  local fails). A creature speaker answers `Missing` for NPC-only keys (`race`, `class`, `faction`, `rank`, `female`,
  `disposition`); an NPC without a faction answers `faction` = the empty id and `rank` = -1, so "is X" fails for it
  and passes (on `Missing`) for a creature, as Morrowind's filters do.
- Invalidation is unnecessary: memos live one query; nothing caches across ticks. Facts never come from presentation
  (tracked quest, `JournalSeen`, subtitles): CONTRIBUTING "Net-ready".

## Rules

```rust
pub enum Concept { Topic(RecordId), Greeting, Named(RecordId) }   // `Named`: bark.*, talk.*, reply.*, pack concepts
pub struct Criterion { key: FactKey, test: Test, on_missing: MissingIs /* Pass | Fail */, weight: u8 /* 1 */, soft: bool }
pub enum Test { Cmp(CmpOp, f64) /* = != > >= < <=; ints and floats as f64 */, Is(RecordId), IsNot(RecordId), Present, Absent }
pub struct Rule {
    id: RecordId,                       // stable across loads: memory and saves key on it
    concept: Concept,
    criteria: SmallVec<[Criterion; 6]>, // reordered at insert: id tests, event facts, soft last
    priority: i32,
    order: u32,                         // insertion order, last tie-break only
    once: Option<MemScope>,             // World | Speaker | Player
    cooldown: Option<Cooldown { scope: MemScope, ticks: u32 }>,
    slot: Option<RecordId>,             // ranked queries: one winner per slot (quick replies)
    response: Response,
    writes: SmallVec<[FactWrite; 2]>,
}
pub struct Response { line: Option<LineRef { topic: RecordId, response: RecordId }>, then: Option<FollowUp>, replies: Option<RecordId> }
pub struct FollowUp { concept: RecordId, delay_ticks: u32, who: Speaker | Listener }
pub struct FactWrite { key: FactKey /* memory.* only */, op: Set(i64) | Add(i64) | Clear, expires_ticks: Option<u32> }
```

- **Specificity** = Σ `weight` of the criteria (Ruskin counts criteria; Yarn counts conditions). Computed at insert.
- **Priority bands** (`const`): `PRIORITY_OVERRIDE` = 1 000 000 (packs that replace content lines),
  `PRIORITY_CONTENT` = 0 (framework default: pure Valve, specificity decides), `PRIORITY_FALLBACK` = -1 000 000.
  A compiled ordered list (Morrowind INFOs) takes `CONTENT - index`, so each entry is its own tier (< 1 000 000
  entries per concept).
- **Writes touch only `memory.*`.** Effects on other state (quests, disposition, items, AI) are the response's job:
  the session's `RunResult` (adapter scripts) or the line's commands, so each piece of state keeps one writer.
- **Rule records:** `sf.dialogue.rule` (RON, through `sf_records` / `sf_mods`, mod-patchable by id). Text form of a
  criterion: `"<key> <op> <value>"`, `"<key> is <id>"`, `"<key> present"`, `"<key> absent"`, suffix `?pass` / `?fail` for missing, `^w`
  for weight. Adapters may insert compiled rules directly (`RuleDb::insert`).
- **Validation at load:** unknown namespace, a write outside `memory.*`, a `Named` concept nobody emits (warning, the
  bark census lists emitted concepts), duplicate ids (last mod wins, logged).

## Selection

```rust
pub struct QueryCtx { player: PlayerId, speaker: Option<ActorKey>, speaker_base: Option<RecordId>, listener: Option<Holder> }
pub struct Query { concept: Concept, ctx: QueryCtx, tick: u64 /* host fixed tick */, event: SmallVec<[(FactKey, FactValue); 8]>, mode: Mode }
pub enum Mode { Best, Exists, Ranked { max: u8 } }
pub enum Selection { Picked(SmallVec<[Pick; 1]>), None { near_miss: bool } }
pub struct Pick { rule: RuleIdx, response: Response }
impl RuleDb { pub fn select(&self, q: &Query, facts: &dyn Facts, mem: &DialogueMemory) -> Selection }
```

1. **Bucket.** `RuleDb.buckets: HashMap<Concept, Bucket>`; a `Bucket` holds rules sorted by rank `(priority desc,
   specificity desc, order asc)` in two lists: `by_speaker: HashMap<RecordId, Vec<RuleIdx>>` (rules with an
   `Is` test on `speaker.id`) and `general`. A query walks the speaker's list and `general` merged by rank.
2. **Eligibility.** A rule is skipped without fact reads if `once` is spent for its scope or its cooldown runs
   (`DialogueMemory`, one map probe each). Then criteria in stored order; the first failure rejects (early out).
   A rule whose non-soft criteria pass and a soft one fails sets `near_miss` (Morrowind's Info Refusal).
3. **Stop.** `Best`: the first passing rule fixes the winning rank; the walk continues only through rules of the
   same `(priority, specificity)` (the tie group), then stops. `Exists`: the first pass returns. `Ranked { max }`:
   best rule per `slot` (rules without a slot count as their own), up to `max`, in rank order.
4. **Tie-break** in a tie group of n > 1: least recently fired (`DialogueMemory.last` within
   `dialogue.recency_window_s`; never or forgotten = oldest); among the equally old, index `h % k` with
   `h = splitmix64(tick ^ speaker_key ^ hash(concept))`. Pure function of the query: no RNG state in `sf_rpg`
   (rpg.md "Co-op" rule 1), identical on replay.
5. **Result.** The caller shows the line and, if the rule has `once`, `cooldown`, `writes` or a tie group > 1,
   sends `RpgCommand::RuleFired { rule, ctx, tick }`; Morrowind's compiled rules have none of these, so classic
   conversations write nothing.

**Cost** (standard technique: Ruskin's sorted buckets with early out; rules evaluated on events, never per frame).
Expected, to be printed by DQ-24's measurement test on the real install: a criterion is a memo probe plus a
compare (~5-20 ns); a Morrowind topic bucket averages ~10 rules (23 693 INFOs / 2 358 DIAL in Morrowind.esm,
rpg-adapter "Data"), the greeting bucket a few thousand, but most reject on their first criterion (speaker id,
race, faction, journal), so a greeting query stays under ~50 µs and a topic query under ~5 µs. The topic list
(`Exists` per known topic, ~100-300 topics) is under ~1 ms and runs on conversation open and after a result,
not per frame. Load: compiling ~37k INFOs (with expansions) and interning keys under ~50 ms, ~6 MB of rules.

## Memory, once, cooldown, writes

```rust
pub enum MemScope { World, Speaker(ActorKey), Player(PlayerId) }   // saved as PlayerKey
pub struct DialogueMemory {
    once: BTreeSet<(RecordId, MemScopeKey)>,
    last: BTreeMap<(RecordId, MemScopeKey), u64 /* tick */>,     // recency and cooldown
    facts: BTreeMap<(MemScopeKey, Sym), MemFact { value: i64, expires: Option<u64> }>,
}
```

- `RuleFired` (host, on its tick) marks `once`, stamps `last` for the rule's cooldown scope and recency (speaker
  scope when there is a speaker, else world), applies `writes` in order, and returns `RpgEvent::DialogueMemory {
  rule, keys }`. Expired facts read `Missing` and are dropped on the next write (lazy, no tick work).
- Bounds: `dialogue.memory.max_entries` total, `dialogue.memory.per_speaker` per actor; eviction drops the oldest
  `last` entries first, then expired facts; `once` marks are progress and never evicted (they count toward the cap
  and log a warning when the cap is reached).
- **Co-op:** memory is host world state (decision 9); `Player` scope keys per player, so "this player already heard
  it" stays per player while "the guard already said it" is shared. Clients never select.

## Barks (`sf_rpg::barks`)

**Contract in.** `BarkCue { speaker: ActorKey, concept: RecordId, urgency: BarkUrgency, context: SmallVec<[(FactKey,
FactValue); 4]>, tick }` is **owned by ai.md (AI30)**: sf_ai emits it from Notice rungs (world-reactions.md
`ConcernRules` `Bark`), greetings, combat, flee, companion remarks (followers.md AI42), and crime witnesses with
`OffenceDef.say` as the concept. Its hearing/LOS gate toward a listener is AI's. `BarkUrgency`: `Ambient < Greet <
Reaction < Alarm < Combat` (`const` order).

**Standard concepts** (`sf_rpg::rules::concepts`, `Named`; packs add more): `bark.greet`, `bark.idle`, `bark.attack`,
`bark.hit`, `bark.flee`, `bark.thief` (saw a theft), `bark.intruder` (trespass), `bark.alarm` (calls the guard),
`bark.witness` (default when an offence has no `say`), `bark.remark.*` (companions), `talk.interrupted` (DQ-31),
`reply.*` (DQ-39).

**Per host tick** (`BarkQueue::run(&mut self, facts, db, mem, budget) -> Vec<BarkLine>`):
1. Drop cues older than `barks.cue_ttl_s`. Order by urgency desc, then age.
2. **Dedupe:** same concept within `barks.dedupe_radius_m` and `barks.concept_gap_s` of a playing or chosen line
   collapses to one (the cue whose speaker is nearest a player listener; ties seeded as above). A storm of 20
   witnesses to one theft becomes one or two lines.
3. **Gates:** speaker free (one line per actor: OpenMW `sayActive`); `speaker.can_speak` fact (not swimming,
   unconscious or dead); a player listener within `barks.range_m`; `barks.speaker_gap_s` since its last line;
   fewer than `barks.max_concurrent` lines playing, unless the cue's urgency beats the lowest playing line and
   `barks.preempt` is on (that line is cut).
4. **Query** `Best` on `Concept::Named(concept)` with the cue context as `event.*`; stop when
   `barks.tick_budget_us` or `barks.max_queries_per_tick` is spent (the rest wait for the next tick, inside TTL).
5. Emit `BarkLine { speaker, rule, line: LineRef, then }` (data, replicated to clients in range) and `RuleFired`;
   a `then` follow-up becomes a delayed cue. The `Barks` presenter (DQ-25; DQ-26 presenters) shows the subtitle
   and plays the voice (`DialogueRules::voice(line)`, default none) and reports when the line ends.

**Storm budget.** 20 simultaneous cues: dedupe and gates first (map probes, ~1 µs each), then at most
`max_concurrent` queries (~50 µs each worst case) = well under the 0.2 ms `barks.tick_budget_us`, inside the
8.3 ms frame on the fixed tick. Cues are events, so a quiet tick costs nothing.

## Seams for quick replies (DQ-39) and interrupts (DQ-31)

- **Quick replies:** a bark rule's `Response.replies = Some(set)` asks for a reply offer: the host runs `Ranked {
  max: dialogue.replies.max }` on `Named("reply.<set>")` with the line's facts (one winner per `slot`: defuse, pay,
  retort, ignore). Each reply rule's `then` is the outcome concept; the timeout fires `reply.<set>.ignored`.
  DQ-39 owns the panel, timing and outcomes; this spec owns only `Ranked`, `slot`, `replies`, `then`.
- **Interrupts:** the host's `Interrupt { reason }` (DQ-31) ends the session and queries `talk.interrupted` with
  `event.reason` (`combat`, `damage`, `crime`) for the speaker's break-off line. No new types.

## Co-op, determinism, saves

- Selection runs on the host only, on its fixed tick, from host state; the seed is the host tick and speaker key,
  so a replay or a host migration repeats the pick. Clients receive `DialogueOutput` (rpg.md) and `BarkLine`.
- **`sf/dialogue` v1** (new section next to `sf/rpg` in `sf_save_bevy/src/sections/mod.rs`): `DialogueMemory` with
  ticks saved as **ages** (save tick minus stamp; expiries as remaining ticks) and rebased on load, so no global
  tick has to persist; player scopes as `PlayerKey`. Load: rule ids no rule has any more are kept (dormant mod
  data, save.md) within the caps. A save without the section loads an empty memory. Golden shape + postcard round
  trip.

## Tunables

Prefix `dialogue.` / `barks.`; a game sets defaults by base-mod record, per preset where marked.

| Name | Default | Range | Unit | Net | Source |
|---|---|---|---|---|---|
| `dialogue.recency_window_s` | 900 | 0-14400 | s | Gameplay | Guess (Yarn LRV is unbounded; window keeps memory bounded) |
| `dialogue.memory.max_entries` | 8192 | 256-65536 | count | Gameplay | Guess |
| `dialogue.memory.per_speaker` | 16 | 1-256 | count | Gameplay | Guess |
| `dialogue.replies.max` | 3 | 2-4 | count | Gameplay | Owner(2026-11-25: 2-3 replies) |
| `barks.range_m` | 40 | 5-100 | m | Gameplay | Reference(OpenMW idle voice 3000 units ≈ 43 m, `actors.cpp:442`) |
| `barks.max_concurrent` (preset) | 3 | 0-16 (0 = no cap) | count | Gameplay | Guess; Classic 0 (OpenMW has no cap) |
| `barks.speaker_gap_s` (preset) | 4 | 0-60 | s | Gameplay | Guess; Classic 0 (only `sayActive`) |
| `barks.concept_gap_s` (preset) | 6 | 0-120 | s | Gameplay | Guess (L4D respeak delays); Classic 0 |
| `barks.dedupe_radius_m` (preset) | 15 | 0-100 | m | Gameplay | Guess; Classic 0 (off) |
| `barks.preempt` (preset) | true | bool | none | Gameplay | Reference(L4D speech priority); Classic false (OpenMW refuses while speaking) |
| `barks.cue_ttl_s` | 1.5 | 0.1-10 | s | Gameplay | Guess |
| `barks.tick_budget_us` | 200 | 20-2000 | µs | Gameplay | Guess (frame target 8.3 ms) |
| `barks.max_queries_per_tick` | 8 | 1-64 | count | Gameplay | Guess |

Per-rule `cooldown` and `then.delay_ticks` are content data (rule records), not tunables. `const`: priority
bands, urgency order, save version, memo inline size.

## Presets and the classic seam

- **Classic** (Morrowind Classic): only compiled INFO rules (same pick as `accepts`, DQ-24 test); bark gates as
  OpenMW (`barks.*` Classic values above); no rule packs.
- **Modern** (framework, Modern Morrowind): compiled INFO rules unchanged plus rule packs as data (barks for
  concepts OpenMW never says, e.g. `bark.alarm` on a Report rung; reactive and companion lines; quick replies),
  Modern gates. A pack line overrides content only at `PRIORITY_OVERRIDE`, so Morrowind's own picks stay intact.

## Tests

Exact, synthetic rules: rank order (priority over specificity over order); specificity sums weights; `Missing`
pass/fail; `soft` near miss; tie group least-recent then seeded pick (fixed tick and key give a fixed index; two
ticks differ); `once` per scope (world vs speaker vs player); cooldown expiry; writes only `memory.*` (load error
otherwise); expiring fact reads `Missing`; `Exists` and `Ranked` per slot; speaker sub-bucket merge equals a flat
scan; bark dedupe, gates, budget carry-over, TTL drop, pre-empt; memory caps and eviction; `sf/dialogue` round trip
and age rebase. Measurement (`#[ignore]`): query µs per bucket size on the real install (DQ-24).

## Slices

DQ-23a rule DB + `Facts` + selection (`sf_rpg::rules`), DQ-23b `DialogueMemory` + `RuleFired` + `sf/dialogue` v1,
DQ-24a INFO compiler + equivalence test (`mw_rpg`), DQ-24b session and refusals on rules (`accepts` becomes the test
oracle), DQ-25a `sf_rpg::barks` queue, DQ-25b Morrowind voice on rules + bark triggers. Roadmap table rows.
