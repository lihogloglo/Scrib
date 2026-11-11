# World reactions: noticing, suspecting, responding, picking up, resetting (`sf_ai` + `sf_interaction`)

Status: design 2026-11-11 (ROADMAP 3.11), nothing built. Research:
[world-reactions-2026-11-11](../research/world-reactions-2026-11-11.md). The "use" half (slots, claims, carrying,
destruction, mounts) is [interaction.md](interaction.md); this is the "react" half. Touches [ai.md](ai.md) "Perception
and stimuli", "Memory and relationships", "Drives", "Tiers and budget"; [crime.md](crime.md) `OffenceAttempt`,
`WitnessQuery`, `StolenLedger`; [save.md](save.md) `sf/overlay`; [net.md](net.md).

<!-- agent-summary -->
> **Agent summary.** Interaction and combat emit `WorldChange` events (destroyed, displaced, occupied, opened, dropped,
> taken, killed). Witnesses are found at event time; everything else becomes a bounded per-cell **concern** that NPCs
> discover by tier (sight for tier 2, room pass for tier 1, schedule edge for tier 0). A `Notice` drive picks a
> response from data (`ConcernRules`): ignore, glance, bark, investigate, confront, report, tidy, flee. Unwitnessed
> harm raises **suspicion** on candidates the victim's evidence points to. NPCs are full culprits too (theft, looting,
> vandalism). Memory is aggregated facts; consequences, suspicion weights and reset timings are **data records**, never
> code. The world heals by owner tidying and a lazy per-category reset on cell load. Slices below.
<!-- /agent-summary -->

## Goal

Owner 2026-11-11: the world must feel real; things are broken, moved and used by NPCs as well as the player, and NPCs
notice. The final game is not only Morrowind: it is a more alive, simulated universe, so NPCs are full participants
(they steal, loot, vandalise, get suspected) and every gameplay consequence is tunable data. References (owner):
Cyberpunk 2077, Kingdom Come: Deliverance 2, Red Dead Redemption 2, the Nemesis system. Oblivion's Radiant AI is not a
model (its only lesson kept: quest-safe NPC goals, already in ai.md "Schedules").

## Owner answers (2026-11-11)

1. **NPC offences: yes.** NPCs commit offences against each other and the player. Classic preset: player-only (seam).
2. **Unwitnessed harm raises suspicion** in the victim; on whom is decided by the standard (below, "Suspicion").
3. **Reset timing "depends"**: per category and per place, data (below, "Reset and persistence").
4. **Consequences of repeated harm: data**, not hard-coded (`ConsequenceRules`).
5. **Retrieving items an NPC took: no special rule.** It is ordinary inventory: loot them dead or unconscious,
   pickpocket, buy, ask or persuade. Whether the NPC took it lawfully follows ownership like any other take.

## Reuse check (CONTRIBUTING rule 0)

- **Bevy 0.20**: observers and entity events, messages, relationships (`OccupiedBy`, `HeldBy`). Glue only; the cores
  stay Bevy-free in `sf_ai` / `sf_interaction`, shells translate.
- **Crates** (checked 2026-11-11): no maintained perception, stimulus or reaction crate (`big-brain`,
  `bevy_observed_utility` stale; `cranium` MPL and unproven; `bevy_society` young, watch). Build on our own perception,
  fact store and drives, which already exist in the AI design.
- **Open source**: OpenMW has witness and crime content (ported, crime-adapter.md) but player-only crime and no
  reaction to objects; its corpse clear and respawn delays (72 game hours) are our reset reference.

## Industry comparison

| Concern | Industry | Ours |
|---|---|---|
| Stimulus sources | UE AI Perception: registered sources, typed stimuli with age and max age | `Concern` board: registered, bounded, aging |
| Object change as signal | Hitman, Thief, Dishonored: a body, open door or moved item is an investigable position + kind | `WorldChange` → `Concern { kind, at }` |
| Reporting | KCD2, RDR2: the witness physically runs to the law and can be stopped | `Report` is a visible, interruptible directive |
| Recognition, suspicion | RDR2 identity recognition; KCD2 suspicion of strangers (unverified detail) | evidence-scored `SuspicionRules` |
| Memory | Nemesis: few salient per-character facts | aggregated facts (one per culprit, kind, owner) |
| Crowd panic | Cyberpunk 2077 crowd flee layer (no witness reporting: press) | `Flee` rung, tier-cheap |
| Reset | Skyrim cell reset 10 days, Oblivion 72 h, OpenMW corpses 72 h, all lazy on re-entry | lazy reconcile on cell load, per category |
| Hauling, clutter | RimWorld, Dwarf Fortress: jobs over item adverts | loose items are `take` slots with adverts |

Gaps kept on purpose: no global alarm state; no open-ended NPC goals that can break quests.

## Pipeline

1. **Emit** (`sf_interaction`, IN14). `WorldChange { tick, kind, object, at, culprit: Option<Culprit>, owner,
   home: Option<PlacementId> }`, `Culprit = Player(PlayerKey) | Actor(ActorRef)`. Kinds (records, adapter-extendable):
   `Destroyed`, `Displaced` (diff against the authored placement, never a scan), `Occupied` (an owned seat or bed in use
   by a non-owner), `Opened`, `LockBroken`, `Dropped`, `Spilled`, `Taken`, `Killed` (a body exists). Sources: IN1
   breaking, IN2/IN3 seats and beds, IN6/IN9 moving, lock rules, combat deaths, inventory drops.
2. **Attribute at event time.** The crime `WitnessQuery` (LOS + awareness, tier-independent, ai.md) runs once. Witnesses
   get the concern with the culprit known; if the change is an offence, crime gets an `OffenceAttempt` as today.
3. **Concern board** (host, per cell; AI19). `Concern { kind, object, at, culprit: Known(Culprit) | Unknown,
   severity, born, ttl, count, claimed_by }`. Cap 16 per cell (UE-style bounded sources): same-object and same-room
   concerns merge (`count`), lowest severity evicted. NPCs check concerns, never the object population. Not saved:
   rebuilt on load from overlay deltas (a displaced placement and a destroyed stage are the evidence) and facts.
4. **Discover by tier** (AI21). Tier 2: sight/hearing test of nearby concerns at the decision rate (batched LOS, ≤ 2
   rays per check). Tier 1: about 1 Hz, noticed when the actor is in the concern's room. Tier 0: resolved logically at a
   schedule edge (the owner arriving home or on shift discovers every pending concern of their places in one step).
   Same outcome on every CPU tier (tier invariance, the co-op rule).
5. **Choose** (AI20). Noticing feeds a `Notice` drive: considerations over severity, relation to the owner (self,
   kin, employer, faction, stranger, guard), culprit known and who, disposition, fear, role, directive floor (content
   wins unless the reaction beats it). `ConcernRules` (records) map `(kind, relation, culprit_known)` to a response
   ladder, cheapest first: `Ignore` → `Glance` (look-at, idle) → `Bark` (rate-limited) → `Investigate` (go, look,
   search N s, return) → `Confront` (dialogue demand: "put that back") → `Report` (run to a guard or bell; interruptible;
   on arrival an `OffenceObserved`) → `Tidy` → `Flee`. Heavy rungs are directives on the stack, so saves and tiers
   already cover them.
6. **Remember** (AI25). One `Fact` per `(culprit, kind, owner)` with a counter and `last_at` (ai.md fact store), so
   repeated vandalism over weeks is one fact whose value climbs; saves stay bounded.
7. **Consequences** (AI25). `ConsequenceRules` (records): fact thresholds → effects (disposition delta, price
   modifier, refuse service, ban from premises, grudge directive Nemesis-style, bounty only through crime). Shipped
   defaults are data and tuned in play.

## Suspicion (owner answer 2)

When a victim discovers an unwitnessed harm (culprit `Unknown`), they raise suspicion (AI23). The standard shape
(Hitman's suspicious-person state, RDR2 recognition, KCD2 stranger suspicion) is an evidence score, not a guess:

- **Candidates** come only from what the victim, their household or nearby witnesses actually perceived: anyone in
  their `Awareness` entries (ai.md: `last_seen` location and tick) seen in or near the place within a time window
  before discovery; anyone later seen carrying a taken item (`StolenLedger`); known prior culprits (facts).
- **Score** per candidate from `SuspicionRules` (records): presence near the place and time, stranger vs resident,
  prior facts against them, carrying the item, faction relations. Highest candidate above a threshold gets a
  `Suspects` fact (low salience, decays); none above it gives a generic `Unsettled` state (caution, guards investigate
  the place).
- **Effects** are consequences, not crime: cooler disposition, an accusing dialogue line, a guard who questions or
  searches (later slice), more vigilant owners. **Never a bounty on suspicion alone**; evidence found later (the item in
  your pack, a witness) becomes a normal offence.
- Wrong suspicion is allowed and is part of the fiction (an innocent NPC can be suspected and lose standing).

## NPC culprits (owner answer 1)

- `OffenceAttempt.offender` becomes `Culprit` (crime.md, slice LW10). Witnessing, reporting, ledger and reactions use
  the same paths for NPC and player culprits. Guards pursue NPC culprits through AI directives (arrest of NPCs later).
- **Who offends** is data: role and personality traits (adapter) make takes of owned items, looting of bodies and
  vandalism attractive in drives (thief, scavenger, desperate, drunk, rival). Nobody offends by default.
- **Quest safety**: the existing validator (ai.md "Schedules") forbids NPC offences against quest-flagged items,
  actors and places.
- Classic preset: NPC offences off; NPC takes only of unowned items.

## Loose items (IN16)

- A loose item (dropped, spilled, a body's weapon) carries a `take` slot (the player's pick-up uses the same point).
  NPCs claim it with a lease (IN0 claims). Adverts are drive inputs: `want_weapon` (unarmed fighter), `own_goods`
  (owner, displaced), `greed` (role flag), `evidence` (guards). The adapter decides roles; the framework names none.
- **Ownership**: dropped items keep their owner (the player's dropped sword is still the player's); an unowned item
  becomes the taker's; taking an owned item is `Taken` plus an `OffenceAttempt` that needs a witness, like the player's.
  Items NPCs hold are ordinary inventory (owner answer 5).
- Out of view: tier 1 and 0 actors only resolve own-goods returns at schedule edges; clutter goes by reset policy.

## Tidy and repair (AI24)

`Tidy` (approach, carry, place, release) reuses IN9 `Relocate`: put displaced owned objects back, clear a spill into its
container, replace a destroyed object with a new one (a restocked placement) after a delay. Per role, from
`ConcernRules`. In view it is performed; out of view the same outcome is applied logically at the schedule edge.

## Reset and persistence (IN15; owner answer 3)

"Window" = how long a change stays before the world restores it by itself. Principle: **save deltas, not state**;
each delta has a class and an expiry; reconcile **lazily on cell load**, never by simulating the absent world; never
remove anything in a player's view. `ResetRules` (records) set the timings **per class and per place kind** (settled,
owned, wild, dungeon, pinned), so "it depends" is data. Defaults, from shipped games:

| Class | Settled / owned places | Wild, dungeons | Source |
|---|---|---|---|
| Cosmetic (debris, blood) | not saved; fade (debris 8 s) | same | interaction.md |
| Ephemeral junk | 24 game h, out of view | 24 h | tuned |
| Bodies (non-named) | guards or kin remove at their next shift (≤ 24 h) | 72 h | OpenMW `fCorpseClearDelay` 72 h |
| Respawn (spawn records) | 72 h | 72 h | OpenMW `fCorpseRespawnDelay` |
| Broken / moved objects | owner tidies at the next schedule edge | 10 days (cell reset) | Skyrim `iHoursToRespawnCell` |
| Owned stock | restock window (adapter) | - | adapter |
| Pinned | never: player-stored, quest items, named bodies, player homes, "never reset" places | never | Skyrim "Never Resets" |

- Reconcile on cell load: `elapsed = now - last_seen`; per delta: expired → drop; owner tidy due → apply "put back";
  else keep. Cost proportional to deltas (< 1 ms for hundreds).
- Save growth: `persistence` + `born` on spawned `sf/overlay` entries; per-cell cap 256 deltas (ephemeral first, then
  oldest); facts aggregated; concerns not saved.
- Co-op: host game time; any player present in a cell blocks its reset.

## Data records (all mod-patchable, adapter ships defaults)

`ConcernRules` (kind × relation → ladder, severities), `SuspicionRules` (weights, window, threshold, decay),
`ConsequenceRules` (fact thresholds → effects), `ResetRules` (class × place kind → ttl), `LootRules` (roles → adverts,
offending traits). The framework ships the mechanism and neutral defaults; numbers are tuned in play.

## Net, save, cost

- Host-only: concerns, suspicion, facts, claims. Clients see results (barks, look-ats, a reporter running). A remote
  player is just another `Culprit`. Reactions run on the fixed tick from events; no presentation feeds back.
- Save: deltas and facts as above; nothing per-tick.
- Cost (targets, measured by AIM2): concern checks are O(nearby concerns × tier-2 listeners) at 5-10 Hz with ≤ 16
  concerns per cell; event-time witness query as crime today. Expected well under 0.1 ms per frame.

## Build slices (cheapest first; one deliverable plus its test each)

| Slice | Deliverable | Depends on | Verify |
|---|---|---|---|
| IN14 | `WorldChange` vocabulary + authored-placement diff, emitted by interaction, lock, combat and drop paths | IN0, IN1, IN6 | exact: one event per transition; displaced = diff |
| IN15 | `ResetRules`, persistence classes, `born`/ttl on overlay entries, per-cell cap, lazy reconcile | IN1, `sf/overlay` | exact: expiry math, eviction order, round trip, old saves load |
| LW10 | `Culprit` in `OffenceAttempt` / `OffenceObserved`, NPC culprits through witness, ledger, reactions | LW2, LW6 | exact: NPC theft witnessed and not, ledger rows |
| AI19 | `ConcernBoard`: kinds, merge, cap, severity, ttl, per-cell index, fed by IN14 + witness query | AI8, IN14 | exact: merge, eviction, attribution |
| AI20 | `Notice` drive + `ConcernRules` + Glance/Bark rungs | AI7, AI19 | exact scores, rate limit, tier invariance |
| AI21 | Discovery by tier (sight, room pass, schedule edge) | AI9, AI10, AI11, AI19 | tier invariance exact |
| AI22 | `Investigate`, `Confront`, `Report` (interruptible reporter, `OffenceObserved` on arrival) | AI6, AI4, AI20, LW9 | headless: reporter reaches guard; stopped reporter files nothing |
| AI23 | Suspicion: candidates from awareness and ledger, `SuspicionRules`, `Suspects` fact | AI8, AI13, AI19 | exact: scoring, threshold, decay, no bounty |
| IN16 | Loose-item `take` slot, adverts, ownership on take, `LootRules` | IN8, LW10 | exact: ownership and ledger after take; claim conflicts |
| AI24 | `Tidy` / replace over IN9 `Relocate`, logical outcome out of view | IN9, AI14, AI21 | headless plan; tier invariance |
| AI25 | Aggregated reaction facts + `ConsequenceRules` | AI13, AI20 | exact counters and decay; save size bounded |
| AIM2 | Concern-check ms per tier (concerns × listeners) | AI21 | `#[ignore = "measurement: ..."]`, owner runs |
| MWR1 | Adapter defaults: Morrowind `ConcernRules`/`SuspicionRules`/`ConsequenceRules`/`ResetRules`/`LootRules`, vandalism offence, Classic preset off | the above | exact table rows |

IN14, IN15 and LW10 need no AI and can follow IN1. The AI slices wait for 3.10 being wired into the game. Visual
checks (owner): prefix `WR-` in `bevyrrowind/docs/visual-checks/interaction.md` (smash a crate in a shop, move a shop
chair, sleep in an owned bed, leave a body, drop a weapon in town); agents build and hand the scene names over.

## Open questions

None for the owner: the answers above settle design; timings, weights and formats are decided by the standard and
live in records.
