# NPC memory, gossip and reputation (`sf_ai`)

Status: draft (AI0e, 2026-11-26). Slice status lives in [docs/roadmaps/ai.md](../../../docs/roadmaps/ai.md) §4
(stage 8). Moved from [ai.md](ai.md) "Memory and relationships" and extended by AI lane decision 8. Crates: `sf_ai`
(Bevy-free), `sf_ai_bevy` (host systems, saves). Matches
[crime.md](crime.md) (`Wanted`, jail), [world-reactions.md](world-reactions.md) (AI25), [stats.md](stats.md) groups
([stats-as-built.md](stats-as-built.md) "Groups"), [followers.md](followers.md). Game side:
[ai-adapter](../../../bevyrrowind/docs/specs/ai-adapter.md) "Memory".

<!-- agent-summary -->
> **Agent summary (2026-11-26).** Typed, immutable facts; each NPC holds bounded `Known` entries (`heard_from`, `hops`
> ≤ 3, confidence down per hop). Gossip swaps top-N facts at logical meetings (shared scenes, NPC talks; tier 0 at
> window edges, same result). Travellers carry facts between settlements. Reputation per settlement is an aggregate of
> what residents know; per faction it is the `sf_stats` group reputation, moved once per fact. Classic: no gossip.
<!-- /agent-summary -->

## Goal

Your deeds travel at NPC speed: the people who saw them know, the people they talk to hear, a caravaner carries the
news to the next town. What a place knows sets greetings, prices and how guards treat you. Named enemies remember you
(Nemesis). Knowledge is typed and exact: it can fade, be retracted or stay unheard, but never mutates.
Non-goals: rumour text generation, lies and distorted rumours, an LLM, a global morality meter in Modern.

## Reuse check

- **Bevy 0.20:** nothing social; plain components, `Message`s for fact events, the fixed tick. The store is Bevy-free.
- **Crates:** crates.io "gossip" crates (`foca`, SWIM) solve cluster membership, not knowledge; utility AI crates
  (`big-brain`, archived) have no memory. None fits, so our own code over `SmallVec` and sorted vectors.
- **Industry** ("Industry comparison"): Nemesis relations, KCD2 and Mount & Blade per-place reputation, RDR2 and
  Skyrim witnesses, Dwarf Fortress told-by chains, ECSwind's "typed facts, no rumour mutation".

## Fact store

```rust
// sf_ai (Bevy-free). Illustrative, not compiled.
pub struct Fact {                      // immutable content, host-wide table, refcounted by holders
    pub id: FactId, pub kind: FactKindId,
    pub subject: Agent,                // who did it: Player(PlayerKey) | Actor(ActorKey) | Group(GroupId)
    pub about: FactObject,             // Player | Actor | Group | Placement | Settlement | Item(RecordId) | None
    pub place: Option<SettlementId>, pub at: GameTime,
    pub value: i32, pub last_at: GameTime,   // aggregated kinds count repeats (AI25): one fact per (subject, kind, about)
    pub learned_by: SmallVec<[GroupId; 2]>,  // factions this fact already moved (once each)
    pub retracted: bool,
}
pub struct Known {                     // one holder's view of one fact
    pub fact: FactId, pub heard_from: Option<ActorKey>, // None + hops 0 = witnessed; None + hops > 0 = told by script or quest
    pub hops: u8, pub confidence: u8 /* q8, 255 = certain */, pub value: i32 /* as heard */, pub learned_at: GameTime,
}
pub struct Memory { pub known: SmallVec<[Known; 8]>, pub relations: SmallVec<[Relation; 4]> } // per actor, capped
pub struct FactKind {                  // record `sf.ai.fact_kind`, mod-patchable
    pub id: FactKindId, pub shareable: bool, pub max_hops: u8, pub salience: f32, pub half_life_days: f32,
    pub settlement_weight: i32, pub faction_weight: i32, pub value_cap: i32, pub aggregate: bool,
}
```

- **Witnessed vs heard.** A witness gets `hops = 0`, confidence from crime's recognition: `Identified` = 255,
  `Description` = `ai.memory.describe_confidence`, `Unknown` none (AI23 `Suspects` instead). Each hop adds 1 to `hops`
  and multiplies confidence by `notify_confidence_mul` (ai.md perception: the rule shouts use).
- **No rumour mutation.** Content lives once in `Fact`; holders hold references, and a swap copies the reference, hops,
  lowered confidence and value as heard. A newer value (higher `last_at`) replaces an older one; nothing else changes.
  Dialogue may say "I saw" (`hops = 0`) or "they say".
- **Retraction.** The owning system retracts a fact when the truth changes (crime's `Pardon`, a quest). Holders drop it
  lazily on their next read: no scan.
- **Decay.** Salience is a pure function: `kind.salience × 0.5^((now - learned_at) / half_life_days)`. Nothing ticks;
  reads compute it. Below `ai.memory.forget_below` the entry is dropped on the next write to that memory.
- **Bounds.** At most `ai.memory.per_npc` entries per actor; a new one evicts the lowest `salience × confidence` (ties:
  lowest `FactId`). Archetype `remembers: false` keeps none (creatures). A fact nobody holds or owns is freed;
  `ai.memory.facts_max` caps the table. World facts (faction relations) are compacted.
- **Writers** are events, never polls: `OffenceObserved`, combat outcomes, AI25 reaction facts, quest and script
  `AddFact` / `RetractFact`, deaths. Dialogue filters and Luau `@sf/ai` read `knows(holder, kind, about, min_conf)`.

## Relations and vendettas (Nemesis-style; owner decision 7)

- `Relation { a, b: ActorRef | PlayerKey, affinity, fear, rivalry, history: SmallVec<FactId> }`, updated by fact-kind
  rules, at most `ai.memory.relations_per_npc` per actor (lowest `|affinity| + fear + rivalry` evicted). Players are
  keyed by `PlayerKey` (the character), so memory survives sessions and works for any number of players.
- **`MemoryRules`** (content): encounters (defeated by, fled from, spared), rivalry escalating to a vendetta (a
  `Memory` directive: seek, ambush), survivor returns, promotions via `sf_stats` groups. The framework ships store,
  decay, queries and events.
- **Vendettas may cross worldspaces, bounded** (decisions (g); Nemesis ambushes, Skyrim's hired thugs): the host picks
  an encounter point near the target player (a link end or route node out of every player's view) within `max_hops`
  and `ttl`; the actor travels on tier 0 and is promoted out of view, then seeks or ambushes. Only actors the
  quest-safety validator allows; an offline target `PlayerKey` pauses the directive (never retargets); expiry returns
  the actor via its stack. Other memory goals stay local.

## Gossip: bounded propagation (AI lane decision 8)

- **Meetings, not proximity.** Swaps happen only at logical meetings: two actors whose served directives point at the
  same scene (a tavern table's slots, a paired chatter slot, an operator and its customer, NPC-to-NPC `Converse`).
  Swap instants are `window_start + k × ai.gossip.swap_period_s` while both stay served there, whatever the physical
  arrival, so every tier gets the same instants; tier 0 runs them at the window edge in time order: **tier-invariant.**
  NPCs passing in a street do not swap.
- **The swap** (`swap(&Memory, &Memory, &Kinds, now) -> SwapResult`, pure, no RNG): each side offers its top
  `ai.gossip.swap_n × archetype.gossip_mul` entries the other lacks or holds with a lower confidence or older value,
  ranked by `salience × confidence`, then `FactId`. Offered entries must be `shareable`, have `hops < min(kind.max_hops,
  notify_max_hops)` and confidence ≥ `ai.gossip.min_share_confidence`. Innkeepers and caravaners: higher `gossip_mul`.
- **Travellers carry facts.** Road travellers (tier 0 on NV12, roadmap decision 15) and operators keep their memory on
  the road. Their stops are scenes in the destination settlement (an NV12 `Anchor` node), so the general rule spreads
  what they carry. Reach follows from hops: witness (0) → local (1) → traveller (2) → next town (3). A pack wanting wider
  reach raises the ceiling (`notify_max_hops`, range 0-8).
- **Speed is emergent:** walking time plus swap period. "Gossip speed" in crime.md means this. `notify_speed_mps`
  stays the speed of live shouts (ai.md "Perception"); gossip adds no speed tunable.
- **Load cap:** a time-ordered queue runs at most `ai.gossip.swaps_per_tick` swaps per tick; a deferred swap keeps its
  logical instant, so the result is unchanged.

## Reputation per settlement and per faction

- **Settlements.** `SettlementRules::home(actor) -> Option<SettlementId>` (adapter; generated worlds: Magna Urbis
  output). Each keeps a derived index of what residents know, `PoolEntry { fact, holders: u16, min_hops,
  max_confidence }`, updated on learn and forget events, capped at `ai.rep.settlement_facts` (top `holders ×
  max_confidence`; pure, rebuilt on load).
- **Settlement score** per (player, settlement), recomputed only when an entry about that player changes:
  `Σ kind.settlement_weight × min(value, kind.value_cap) × max_confidence/255 × min(1, holders / ai.rep.spread_full_holders)`,
  clamped to `±ai.rep.max`. As facts fade and holders forget, the score drifts back (KCD2 towns forgive).
- **Factions extend `sf_stats`, no second number:** faction reputation is `Membership.reputation` (stats-as-built
  "Groups"). The first time a member holds a fact at confidence ≥ `ai.rep.faction_learn_confidence`,
  `ReputationRules::faction_delta(kind, value)` emits one `StatOp` on the player's group reputation and adds the
  faction to `learned_by`: once per fact per faction, no decay (stats owns it).
- **Bands and effects.** `ReputationRules::band(score) -> RepBand` (Hated, Disliked, Neutral, Liked, Admired; thresholds
  `ai.rep.bands`, hysteresis `ai.rep.band_hysteresis`). `ReputationView { settlement, faction }` per player feeds
  greetings (bark and dialogue categories), prices (a service price modifier) and crime: guards read the band for watch
  and warn behaviour; **a band never adds bounty** (bounty comes only from delivered reports, crime.md).
- **Crime matches.** A delivered report creates a `Wanted` fact (`subject` player, `about` jurisdiction, value =
  bounty), held by the receiving guard at `hops = 0`; it spreads by the rule above; `Pardon` retracts it. A guard who
  holds it raises `HeatRaise { cause: SeenWanted }` on sight. Modern jail's "reputation hit" (`JailRules::served`) is a
  `Jailed` fact seeded at `hops = 0` into the jail's guards: the settlement score drops through it, and its
  `faction_weight` moves the jurisdiction's faction once.
- **Classic** (adapter MA13): no swaps, no settlement score (`GossipRules::None`); Morrowind's global fame and faction
  reputation, moved by scripts only, as vanilla.

## Consequences (`ConsequenceRules`, AI25)

world-reactions.md decision 7 on this store: one aggregated `Fact` per (culprit, kind, owner), `value` counting
repeats, so weeks of vandalism stay one entry. `ConsequenceRules` (records) map a holder's own facts over thresholds to
effects (disposition, price, refused service, ban, a Nemesis-style grudge directive; bounty only through crime), run on
fact events, never per frame. Lasting effects are facts (`Banned`), so saves stay bounded. Personal consequences read
the holder; reputation reads the settlement.

## Co-op and net

- Facts are about a `PlayerKey`; reputation is per player per settlement and per faction. A deed two players are seen
  doing gives each their own fact. A commanded follower's deed is its leader's (crime.md, followers.md).
- Host only; never replicated except a player's own `RepBand`s, if the pack shows them. `FactId`s are stable in saves.

## Saves (`sf/aimemory`)

One world section (ai.md "Saves" names it), v1 = referenced unretracted facts and each actor's non-empty `Memory`.
Nothing derived: indexes, scores and the swap queue rebuild on load. Bound: `facts_max × ~40 B + actors × per_npc ×
~16 B + relations`, checked by a test that fills every cap.

## Cost

Event-rate only: facts on events, swaps from a queue, scores on dirty entries; no per-frame or per-tick scan. A swap
merges two sorted lists of ≤ `per_npc` entries. Budget: ai.md's "facts 0.05 ms" per tick; a measurement test prints ms.

## Tunables

| Tunable | Default | Unit, range | Source |
|---|---|---|---|
| `ai.memory.per_npc` / `relations_per_npc` / `facts_max` | 32 / 8 / 16384 | count, 4-256 / 0-64 / 1024-262144 | Guess |
| `ai.memory.forget_below` | 0.05 | salience, 0-1 | Guess |
| `ai.memory.describe_confidence` | 160 | q8, 0-255 | Guess |
| `ai.gossip.swap_period_s` | 600 | game s, 10-7200 | Guess (one exchange per 10 game minutes at a table) |
| `ai.gossip.swap_n` / `swaps_per_tick` | 2 / 32 | count, 0-16 / 1-1024 | Guess |
| `ai.gossip.min_share_confidence` | 64 | q8, 0-255 | Guess |
| `ai.rep.settlement_facts` / `spread_full_holders` | 64 / 8 | count, 8-1024 / 1-256 | Guess |
| `ai.rep.faction_learn_confidence` | 128 | q8, 0-255 | Guess |
| `ai.rep.max` / `ai.rep.bands` / `ai.rep.band_hysteresis` | 100 / -60, -25, 25, 60 / 5 | score, 10-10000 | Guess |

Reused from ai.md's perception table: `notify_confidence_mul` (per hop), `notify_max_hops` (ceiling). Per-kind
numbers are record data with load-time range checks.

## Tests

Exact and seeded (no RNG in swaps; seeds only in the scripted scenes):
- decay and eviction: salience at given times, lowest entry evicted, ties by `FactId`;
- swap: set difference, top-N order, hops + 1, confidence × mul, stop at the hop ceiling, unshareable never passes,
  fact content unchanged (no mutation), newer value replaces older;
- **tier invariance:** one scripted tavern day with 6 NPCs at cap 48 and at cap 400 gives the same `Known` sets at every
  window edge; a deferred swap under `swaps_per_tick = 1` gives the same sets;
- travellers: a caravaner told by town A's witness brings the fact to town B's index only after its logical
  arrival, at `hops = 2`;
- reputation: given index entries give the exact score and band, with hysteresis; a faction delta applies once;
- crime: `Wanted` spreads, `Pardon` retracts it everywhere on read; `Jailed` lowers the jail settlement's score;
- co-op: two `PlayerKey`s keep separate facts and scores; saves round trip, filled to every cap, within the bound.
Measurement: `#[ignore = "measurement: gossip spread"]` prints hours until half of a synthetic 40-resident town knows a deed.

## Industry comparison

| Game | What it does | Ours | Verdict |
|---|---|---|---|
| Shadow of Mordor / War (Nemesis) | orcs remember encounters, rivalry, promotion, survivor returns, ambush | relations + `MemoryRules`, bounded vendettas | **match** |
| RDR2 | witnesses must reach the law (stoppable); bounty per state; honor is one global moral scalar | walked reports (crime), facts per witness; no global meter in Modern | **match** (witness); **differ** (honor) |
| KCD, KCD2 | reputation per town, set by crimes and deeds, moves prices and dialogue, recovers over time | settlement score from residents' facts, decays | **match** |
| Mount & Blade | relation per village, town and lord; drives prices and recruits | per settlement + faction reputation | **match** |
| Skyrim, Oblivion | fame and infamy counters, global; bounty per hold known hold-wide at once | Classic keeps Morrowind's global fame; Modern spreads by meetings | **differ** (by design) |
| Dwarf Fortress (adventure mode) | rumours carry who told whom, spread by conversation and travellers | `heard_from`, hops, travellers | **match** (no mutation in ours) |
| Watch Dogs: Legion | NPCs remember harm to their relatives (press only) | relations by fact rules | **match** (press only) |

Sources: UESP (fame, infamy, crime); Warhorse KCD2 codex and GDC 2025 (press); Monolith Nemesis talks (press); Bay 12
DF v0.40 notes (rumours); `docs/research/gameplay-2026-11-25/ai.md` §5 and `ecswind.md`.

## Build plan

Order and status: [docs/roadmaps/ai.md](../../../docs/roadmaps/ai.md) §4, stage 8.
- **AI13** fact store, relations, decay, vendettas, `sf/aimemory` (exact decay, round trip).
- **AI43** gossip swaps: meetings, hops, confidence, tier-0 edges (exact spread, tier invariance).
- **AI44** reputation per faction and settlement (exact thresholds).
- **AI25** `ConsequenceRules` on aggregated facts (exact, save bounded).
- **MA13** Morrowind fact kinds, reputation to disposition, Classic = fame (owner AI-10).

## Open questions

1. **Can the player silence gossip?** Recommended: yes in Modern, a bribe or intimidate option makes one holder drop
   one fact (people already told keep it); no planted lies (no mutation). **Answered (owner 2026-11-26): yes, as recommended.**
