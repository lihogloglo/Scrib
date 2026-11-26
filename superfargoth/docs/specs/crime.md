# Crime, law, stealth, locks and traps (`sf_crime`, `sf_crime_bevy`, `sf_rpg_ui`)

Status: spec written 2026-11-10, revised 2026-11-25 (LW0: AI perception design, walked reports, lock contract;
plan in the [crime roadmap](../../../docs/roadmaps/crime.md), "decision N" below = its §3; locks and traps: [crime-locks.md](crime-locks.md)); nothing built. Builds on [stats.md](stats.md) (`Counter` stats such as bounty,
`StatOp`, `UseSkill`), [rpg.md](rpg.md) (`RpgCommand`, inventory, provider claims), [services.md](services.md)
(`ServiceKind`, `DealVeto`, `Purse`), [effects.md](effects.md) (`SourceDef`: a trap is an effect source),
[ai.md](ai.md) "Perception and stimuli" (`OffenceObserved`, awareness, fact store), [action.md](action.md) (the Sneak
mode flag), [rule-hooks.md](rule-hooks.md) (pure rules), [save.md](save.md), [net.md](net.md) "Net-ready rules". First
user: Bevyrrowind ROADMAP 2.8 ([crime-adapter.md](../../../bevyrrowind/docs/specs/crime-adapter.md)).

<!-- agent-summary -->
> **Agent summary.** Three small systems sharing one vocabulary: **ownership** (who may take or use an object),
> **law** (witnesses react and walk a report to a guard; bounty per jurisdiction lands only on delivery; a guard
> warns, arrests or pursues; jail is a short local scene) and **locks and traps** (lock level and HP, key, trap
> source; pick, probe and force as host requests). Everything is a Bevy-free pure rule over one input struct
> (`sf_crime`), driven by a thin host (`sf_crime_bevy`) from commands: `OffenceAttempt` in, `CrimeEvent` out.
> Witnesses and detection come from the AI lane through two seams (`WitnessQuery`, answered by `sf_ai_bevy`, a fake
> in tests; `DetectionRules`, the concealment formula `sf_ai` consumes), so slices LW1-LW5 are pure and unit-testable. Stolen state is a per-player **ledger** keyed by (item record, owner), not a stack flag, so stacks
> still merge. Bounty is a stat per jurisdiction, per player. Read next: "Design", "Co-op", "Build plan", "Open questions".
<!-- /agent-summary -->

## Goal

A player can take what is not theirs, get seen or not, collect a bounty, be confronted, pay, serve or fight, and clear
the record; pick a lock or disarm a trap with a tool; sneak past an observer. The rules are exact and testable without
AI, rendering or UI; the game supplies the numbers (offence table, jurisdictions, jail place) and the content flags.

Non-goals: perception itself (sight cones, hearing, awareness memory: [ai.md](ai.md), this spec defines two seams),
combat damage and sneak-attack multipliers (combat.md), the dialogue engine (the arrest is a dialogue state the host
opens), werewolf and vampire reveal (game rules that call `OffenceAttempt`), faction membership (stats.md; here only
"is expelled" is written through `StatOp`), shops (services.md; here only the stolen-goods veto and fence offers),
persuasion and bribery of guards (a later `ArrestOption`).

## Canonical technique

Every shipped open-world RPG splits crime into four steps, and so do we:

1. **Ownership is data on objects** (Bethesda: owner NPC or faction + minimum rank, plus a global that lifts it;
   Skyrim adds crime-faction hold ownership; KCD: owned items and "private" areas; Cyberpunk: area zones).
2. **An offence is an event with a kind, a victim, and an amount.** Whether it counts is decided by **witnesses**
   (line of sight plus a detection roll, Skyrim/Oblivion: a hearing radius; KCD: a witness must see and then
   report). Unwitnessed crimes cost nothing (Bethesda), except killing and assault on a victim.
3. **Consequence = bounty in a jurisdiction** (Skyrim: one crime faction per hold, bounty above a threshold sets
   guards on you; Morrowind and Oblivion: one global bounty; KCD: reputation per town plus fines; GTA: wanted level).
4. **Resolution**: pay, serve time (skill loss in Morrowind/Oblivion; Skyrim: a stat-reduction sleep; KCD: jail
   with a stat penalty), flee, or fight; stolen goods are confiscated on pay or jail.

Locks and traps: [crime-locks.md](crime-locks.md). Stealth: concealment versus observer perception; Bethesda rolls per check, modern
stealth games (Skyrim, Dishonored, Thief) fill a meter over time. We offer both behind one rule.

## Reuse (Bevy, then crates, then our own)

Bevy: events, components, `FixedUpdate`; nothing for crime. Crates: none (a game mechanic). Ours, existing:
`sf_stats` (bounty counter, `StatOp::UseSkill`, `CounterSet`), `sf_rpg` (`RpgCommand`, `service::DealVeto`),
`sf_effects` (`Inflict` for a trap), `sf_ids` (seeded rolls, `ActorKey`, `PlayerId`), `sf_records` (offence and
jurisdiction defs as records), the AI lane's fact store and `OffenceObserved`, `sf_save_bevy` sections.
New: `sf_crime` (rules, ledger, lock state) and `sf_crime_bevy` (host). Interaction ("activate this") knows no
locks: crime's gate runs in its `ActivationSystems::Gates` ([crime-locks.md](crime-locks.md)).

## Design

### Crates and layers

- `sf_crime` (Bevy-free): types, ownership, `StolenLedger`, offences and `assess`, `CrimeLedger`, `LockState`,
  `TrapState`, `gate_activation`, rule traits with `Standard*` defaults, settlement as `Vec<RpgCommand>` and
  `Vec<CrimeEffect>`, serde data.
- `sf_crime_bevy`: host systems on the fixed tick, requests and events, wiring to AI, services, stats, saves.
- `sf_rpg_ui`: `InteractPrompt` and `StealthView` view-models; arrest dialogue buttons ride the dialogue view.
- Game adapter crate: rules, records, GMSTs (Morrowind: `mw_crime`, [crime-adapter.md](../../../bevyrrowind/docs/specs/crime-adapter.md)).

### Ownership

```rust
pub enum Owner { None, Actor(ActorKey), Faction { id: RecordId, min_rank: i32 } }
pub struct Ownership { pub owner: Owner, pub lifted_by: Option<GlobalKey> }   // global != 0: nobody owns it
pub struct ObjectKey(/* stable id of a placed object, sf_ids */);
```

`Ownership` is content data on placed objects plus a sparse overlay; **containers pass their owner to their
contents**, an actor owns its inventory. `OwnerRules::owned_for(&OwnerCtx) -> OwnershipVerdict { Free | Owned {
victim, faction } }` is pure over rank, lifting global and own property; `use_allowed(&UseCtx)` adds dead, in-combat,
sleeping or knocked-out victims and adapter `never_take` chests.

**Stolen state is a per-player ledger, not a stack flag** (that would split stacks): `StolenLedger { entries:
BTreeMap<(RecordId item, OwnerKey), Vec<Lot>> }`, `Lot { count: u32, day: GameDay, at: WorldPos }`, oldest first
(decision 11; KCD2 hot goods):

- `record_taken(item, owner, count, day, at)` pushes a lot; skips currency and dead or gone owners (adapter opt-in).
- `reconcile(item, held)` after **every inventory change** clamps lots to what is held, oldest first: sold goods stop
  being stolen (OpenMW never clears, `mechanicsmanagerimp.cpp` 1079-1135, so honest copies turn "stolen"; Classic keeps
  that, crime-adapter.md).
- `cool(day, pos, &Cooling)`, daily and before each sale offer: `Modern { days, distance_m }` drops a lot `days` old
  or `distance_m` from where it was taken; `Permanent` (Classic) never drops.
- `confiscate(held)` removes lots in map order (deterministic); `stolen_from(item, merchant)` matches the merchant or
  its primary faction; `stolen_count(item)` shows "N stolen" on an unsplit stack.
- **Restricted zones** (LW19; Skyrim, KCD2 private rooms): a `Restricted { owner, grace_s }` record gives `Warn` on
  entry; staying past `grace_s` is trespass. **Possession is evidence** (LW20): a frisk (AI23 suspicion, then search)
  that finds lots files a possession offence and confiscates. Tunables: "Reactions, arrest, jail".

### Offences as data

```rust
pub struct OffenceDef {                // a record, kind "crime.offence"
    pub id: RecordId,                  // theft, pickpocket, trespass, assault, murder, ... open set
    pub bounty: BountyRule,            // Flat(i32) | PerValue { mult: f32, min: i32 } | FromArg
    pub witness: WitnessRule,          // Seen | SeenOrVictimAware | VictimAlwaysAware | Heard
    pub provoke: Provocation,          // fight amount for witnesses and victim (rules input)
    pub disposition: DispositionHit,   // for witnesses and victim (rules input)
    pub expels: ExpelRule,             // None | VictimFaction | NamedFaction
    pub say: Option<RecordId>,         // witness bark topic
}
```

`OffenceAttempt { offender: PlayerId, kind: RecordId, victim: Option<ActorKey>, subject: Option<ObjectKey>,
arg: i32, faction: Option<RecordId>, tick }` is the one input, emitted by gameplay code (takes, doors, hits, kills,
pickpocketing, beds, forced locks, spells). A follower acting on its leader's command (`AttackTarget` on a non-hostile,
followers.md) files the attempt with the leader as `offender`; its own unordered acts stay its own (LW10 `Culprit`).
`assess(&OffenceCtx) -> Assessment { per_witness: Vec<WitnessReaction>, reports: ReportPlan }` is pure and adds **no
bounty** in either preset: bounty lands only when a report is delivered ("Reports and recognition"). Departs from the
2026-11-10 text, which added `Assessment.bounty` at assess (decision 2).

`WitnessReaction { witness, reaction: Reaction, disposition_delta, permanent: bool }` with `Reaction = Ignore | Flee |
Fight | Pursue | Say(topic)`. `ReactionRules` (adapter) fills them from the witness's data (alarm, fight, guard flag)
and the offence def. The host forwards `Pursue`/`Fight`/`Flee` to the AI lane as directive requests (ai.md).

### Jurisdictions, bounty and heat

```rust
pub struct Jurisdiction {             // a record, kind "crime.jurisdiction"
    pub id: JurisdictionId,
    pub bounty_stat: StatKey,         // Counter: `bounty` for the default, `bounty.<id>` otherwise (stats.md)
    pub ladder: Vec<LadderStep>,      // LadderStep { from: i32, stance: Stance }, ascending by `from`
    pub kill_on_sight: Option<i32>,   // bounty >= this: Attack, no arrest offered
    pub kill_needs_murder: bool,      // true: Attack also needs a murder on record (Modern, owner); Classic false
    pub decay_per_day: i32,           // 0 = no decay (Classic)
    pub murder_floor: i32,            // decay stops here while a murder is on record
    pub options: Vec<RecordId>,       // ArrestOption records offered here
    pub jail_place: PlaceRef, pub release_place: PlaceRef, pub guard_class: RecordId,
}
pub enum Stance { Ignore, Warn, Arrest, Pursue, Attack }
```

`JurisdictionRules::of(guard | pos) -> JurisdictionId` (adapter): Morrowind, Oblivion one global (owner Q3, open);
Skyrim the hold; a modern open world a region or faction. The adapter fills the record numbers; a game without records gets a default
jurisdiction from the `crime.law.*` tunables.

**Bounty is a debt**: a `Counter` stat per player per jurisdiction that scripts, dialogue and the HUD read directly.
Writers: `CrimeCommand::AddBounty` (only from a delivered report), `Pay`, `Clear` and the daily decay (`-=
decay_per_day`, never below `murder_floor` while a murder is on the `CrimeLedger`), all via `StatOp::CounterSet`.

**Heat is pursuit** (decision 5; GTA, Cyberpunk 2.0, RDR2): AI29's level per (player, jurisdiction) over AI8f's
last-known-position search. Crime only raises it (`HeatRaise { player, jurisdiction, cause: Fled | Resisted |
SeenWanted }`); AI29 cools it while the guards have lost the player and ends the pursuit at zero, or on `Pardon`.
Losing the guards never lowers the bounty.

**The ladder** (`LawRules::stance(&LawCtx { bounty, murder_on_record, wanted_known, jurisdiction }) -> Stance`, pure; `wanted_known` = this guard holds a `Wanted` fact, ai-memory.md, which `SeenWanted` needs): the highest
step with `from <= bounty`, else `Ignore`; `Attack` at `bounty >= kill_on_sight`, and only with a murder on record while `kill_needs_murder` (Modern, owner);
Classic clears it: Morrowind attacks at `iCrimeThreshold` x `iCrimeThresholdMultiplier` (crime-adapter).
`Warn` gives `crime.law.warn_window_s` to comply, then `Arrest`; at `Pursue` guards chase on sight. Fleeing and
resisting raise heat, never the stance.

**Fairness** (decision 3; KCD2 players call unseen bounties unfair): every bounty entry carries `BountyCause {
crime: CrimeId, kind, reporter, place, item, recognition, tick }`, listed in the journal and stats screen (LW17).

`CrimeLedger { next_id, paid_through, causes: Vec<BountyCause> }` per (player, jurisdiction): every offence batch gets
an increasing `CrimeId`; witnesses remember the ids they saw (AI fact `crime.witnessed`); paying sets `paid_through =
next_id - 1` and the host emits `Pardon { player, jurisdiction, through }`; witnesses with ids `<= through` calm down and
restore their disposition. This is OpenMW's calm-down mechanism (`player.cpp` 414-426, `actors.cpp` 1203-1223) made data.

### Witnesses (the AI seam)

```rust
pub trait WitnessQuery {
    /// Who observed this offence, now. Event-time, tier-independent (ai.md "Perception and stimuli").
    fn witnesses(&self, q: &WitnessRequest) -> Vec<Witness>;
}
pub struct WitnessRequest { pub offender: PlayerId, pub at: WorldPos, pub kind: RecordId, pub victim: Option<ActorKey>,
                            pub rule: WitnessRule, pub radius_m: f32, pub tick: Tick }
pub struct Witness { pub actor: ActorKey, pub saw: bool, pub heard: bool, pub is_victim: bool, pub can_report: bool,
                     pub band: Band, pub visibility: f32 }    // ai.md awareness band and sight visibility
```

The AI lane answers with `OffenceObserved { offender, kind, witnesses, at, tick }` (ai.md "Crime, reactions,
adapter"), event-time, from its own LOS and awareness; `band` and `visibility` feed recognition. `can_report` is false
for the offender's allies and followers, another player (Co-op 6), the dead, the `Downed` and actors already fighting
the victim. `WitnessQuery` has one implementation, `sf_ai_bevy`'s event-time query (AI8b, LW9); `sf_crime` tests use a
fake; no stand-in (decision 7). Until LW9, offences are ledgered and logged with no bounty.

### Reports and recognition

A witness never adds bounty: it walks a report to the law (decision 2; RDR2, KCD2; owner 2026-11-25, both presets).

```rust
pub struct ReportPlan(pub Vec<(ActorKey, ReportTarget, Recognition)>);  // one per witness with `can_report`
pub enum ReportTarget { Guard(ActorKey), Post(PlaceRef) }                // ReportRules::target (adapter)
pub struct Report { pub id: ReportId, pub crime: CrimeId, pub offender: PlayerId, pub reporter: ActorKey,
                    pub recognition: Recognition, pub born: Tick, pub state: ReportState }
pub enum ReportState { Pending, Walking(ActorKey /* guard */), Delivered, Cancelled(CancelCause) }
pub enum CancelCause { Dead, Downed, Bribed, Threatened, Timeout }
pub enum Recognition { Unknown, Description, Identified }
```

The host owns reports on the fixed tick. A guard who witnesses is its own target: it delivers on the next tick.
`Pending` becomes `Walking(guard)` when AI22's `Report` directive starts;
arrival sends `ReportDelivered { report, guard }`. Cancelled when the reporter dies, is `Downed` (combat), takes a
bribe or yields to a threat (`ReportRequest::Stop { report, by: Bribe { gold } | Threat }`, from dialogue or an
interaction verb, settled by `ReportRules::stop`, pure, seeded), or misses `crime.report.timeout_s`. A cancelled report
files nothing; shouts on the way raise awareness, never bounty.

On delivery, once per `CrimeId`, the host adds the offence's `BountyRule` sum with its `BountyCause` to the
**guard's** jurisdiction and emits `OffenceCommitted`; a second report of the same crime adds only a cause. Classic:
the adapter maps Morrowind's sums and `reportCrime` rules onto delivery (crime-adapter.md, LW0e). Departs from the old
Classic plan (instant report at event time, as OpenMW does).

**Recognition** (decision 4; RDR2: seen versus identified witnesses, masks lawmen see through; Hitman: disguises
their own staff see through): `RecognitionRules::tier(&RecognitionIn { band, visibility, disguise, is_law,
knows_offender }) -> Recognition`, pure, fixed at sight; `disguise` in [0, 1] is the highest `Disguise` (item data) the
offender wears. Standard: score = `visibility × (1 − disguise)`; `Identified` at `band >= Alerted` and score >=
`identify_at`, `Description` at score >= `describe_at`, else `Unknown`; one tier higher for law and acquaintances (the
adapter says who knows the offender). `Identified` charges the bounty; `Unknown` charges nothing and feeds AI23
suspicion only; what `Description` charges is owner Q2 (recommended: guards learn the look and charge on identifying
the player; until answered, as `Identified`). Classic: always `Identified`.

### Reactions, arrest, jail

- **Notice on sight** is a perception event in the AI lane (AI29), never a scan; crime supplies the stance.
- **Arrest** is a host-owned `ArrestSession { player, guard, jurisdiction, bounty }`, opened when a guard at `Arrest`,
  `Pursue` or an expired `Warn` reaches the player, or when the player surrenders; the dialogue is the screen. Options
  are `ArrestOption { id, kind, available }` records the jurisdiction lists (decision 13; KCD2): `Pay` (the fine,
  `LawRules::fine(bounty)`), `Jail`, `Resist` and `Flee` (heat up, session closes; on `Resist` the guard fights),
  `Surrender` (yield mid-pursuit; never under kill on sight); `Persuade`, `Bribe` later (LW21). `ArrestRequest {
  Choose { session, option } | Surrender }` in, `ArrestEvent` out. Morrowind's guard dialogue drives it (DQ-15 hooks).
- **Pay**: bounty to 0, `StolenLedger::confiscate` into the jurisdiction's evidence store, `Pardon`, drawn weapon
  holstered. **Jail** (decision 12; KCD2): `Sentence { days = LawRules::jail_days(bounty), place, clock: JailClock }`,
  bounty to 0, goods confiscated, `Pardon`, fade to `jail_place`, a short local scene (`crime.jail.scene_s`). One rule,
  clock by session: `PassTime` in solo (days pass as a rest does, owner 2026-11-25), `Credit` in co-op (the shared clock
  never moves; the days count for the jailed player only). Then `JailRules::served(days, &PlayerView) ->
  Vec<RpgCommand>`: Modern = reputation hit (a `Jailed` fact, ai-memory.md), no skill loss (Classic hook keeps Morrowind's); release at
  `release_place`. A saved `Sentence` resumes. **Departs** from the old Co-op 6 (time skipped for one player).
- **Fines on the way out**: `ServiceKind` offers `crime.pay` (thieves' guild style: pay without being arrested)
  and `crime.clear` (script/quest effect: `Clear { jurisdiction }`) reuse services.md; an adapter script can call
  the same `CrimeCommand`s.
- **Net**: reports, heat raises and sessions are host-owned, per player, on the fixed tick; clients see
  `ReportChanged` and send `ReportRequest`/`ArrestRequest`.

**Crime tunables** (tunables.md; `crime.law.*` set the default jurisdiction only; heat cooling is AI29's):

| Tunable | Unit | Range | Default | Source |
|---|---|---|---|---|
| `crime.report.timeout_s` | s (sim) | 30-900 | 180 | Guess |
| `crime.recognition.identify_at` / `describe_at` | score 0-1 | 0-1 | 0.6 / 0.25 | Guess |
| `crime.law.warn_window_s` | s (sim) | 2-30 | 8 | Guess (KCD2, Skyrim warn beat) |
| `crime.heat.on_flee` / `on_resist` / `on_seen_wanted` | heat level 0-1 | 0-1 | 0.5 / 1.0 / 0.25 | Guess |
| `crime.law.warn_from` / `arrest_from` / `pursue_from` | bounty | 0-10000 | 1 / 40 / 1000 | Guess |
| `crime.law.kill_on_sight` / `murder_floor` | bounty | 0-10000 | 1000 / 1000 | Guess |
| `crime.law.decay_per_day` | bounty per game day | 0-1000 | 10 | Guess (RDR2 cools) |
| `crime.jail.bounty_per_day` / `scene_s` | bounty per day / s (real) | 10-1000 / 3-60 | 100 / 12 | Guess |
| `crime.stolen.cool_days` / `cool_distance_m` | game days / m | 1-60 / 0-50000 (0 off) | 7 / 2000 | Guess (KCD2) |
| `crime.fence.fresh_price_mult` | price fraction | 0.1-1 | 0.5 | Guess |
| `crime.trespass.grace_s` | s (sim) | 2-60 | 10 | Guess |

### Fences and stolen goods in services

`StolenVeto` (services' `DealVeto`): selling a lot `stolen_from(item, merchant)` vetoes with `Confiscate` (goods to
the merchant, then a theft `OffenceAttempt`, as Morrowind). Modern: other merchants refuse hot lots, no offence;
cooled goods sell anywhere. Classic: only the victim refuses, forever. `FenceRules::accepts(&Merchant, &Item)` marks
fences (decision 11; Skyrim, KCD2), who buy hot goods at price × `crime.fence.fresh_price_mult`: framework on,
Morrowind content none unless a mod flags a merchant.

### Stealth and detection

Decision 6: crime owns the concealment formula, AI owns detection. The sneak state is the `Sneak` mode flag in
`sf_action::ActionState` (action.md); this spec never moves a character. One pure seam (`sf_crime/src/stealth.rs` is
the source of truth for names):

```rust
pub trait DetectionRules: Send + Sync {
    /// How hidden the target is from the observer right now; larger is better hidden. 0..~200.
    fn concealment(&self, i: &DetectIn) -> f32;
}
pub struct DetectIn { pub target: TargetView, pub observer: ObserverView, pub distance_m: f32,
                      pub angle_rad: f32 /* 0 = in front */, pub factors: VisibilityFactors }
```

- **Inputs.** `factors` is ai.md's `VisibilityFactors`: stance, size, disguise, stat modifiers, and `light` = AI8d's
  `LightExposure` after the observer's `darkness` (ai.md "Light exposure"). Crime never reads light. `TargetView {
  stats, stealth: StealthState { sneaking, moving: Speed } }` and `ObserverView { stats }` carry stats only: duty,
  drink, sleep and mood are the observer's `PerceptionProfile`/`PerceptionStates` (ai.md "Perception profile", AI0g).
- **AI side** (ai.md "Awareness"): `sf_ai` owns the loop, meter, bands, `Awareness` and `PlayerDetection` (max over
  observers that count). Its `DetectionModel` is **Meter** (default; gain × `visibility_from_concealment(c)` =
  `clamp(1 - c/100, 0, 1)`) or **Classic**: one seeded roll per sight check inside `sf_ai`, so no roll enters
  `DetectIn`. Bevyrrowind's Classic rules are MA5, Morrowind's `awarenessCheck` ported as a `DetectionRules`.
- **Default rules.** `StandardDetection` (modern): light, stance (crouched, prone), behind the observer's facing, minus
  a speed reveal above a free speed, clamped. It ignores distance (the meter's `f_dist`), size, disguise and
  modifiers. Upright, still and in front equals the old light-only visibility. Tunables: the 9 `crime.stealth.*` in
  `sf_crime_bevy/src/lib.rs` (`TUNABLES`; Guess, Live; defaults = `StandardDetectionConfig::default()`).
- **Host.** `sf_ai_bevy/src/detection.rs`: `AiDetectionRules(Arc<dyn DetectionRules>)` (default `StandardDetection`,
  replaceable) and the target's `AiVisibilityFactors` (absent = upright); sight builds one `DetectIn` per check on the
  stack. `CrimeStealthPlugin` binds the tunables and swaps the `Arc` on a change, only while the installed rules are
  its own (`Arc::ptr_eq`).
- **Built:** LW4a (seam, `StandardDetection`), LW4b1 (sight seam), LW4b2 (live tunables), MA18 (Bevyrrowind writes the
  player's `AiVisibilityFactors` from `Mode::Sneak`, ai-adapter.md). **Next:** stats and `StealthState` in the views
  (still default), MA5 Classic, MA19 (chameleon, invisibility, size), LW12 eye.
- **Departs** (owner 2026-11-25) from per-NPC indicators: one sneak eye ("Prompt and HUD").
- **Departs** from crime reading light: light arrives through `factors` from AI8d.
- **Departs** from nearest-observer exposure: `PlayerDetection` is the max over observers.

Pickpocket takes the lock seam's shape ([crime-locks.md](crime-locks.md), decision 9): `Timer` (KCD2 hold-to-search, risk from victim awareness, LW18)
or `Roll` = `PickpocketRules::caught(&PickIn, roll) -> bool` (Classic and "auto").

### Locks and traps

Moved to [crime-locks.md](crime-locks.md) (LW0c, decisions 8-10): `LockState`, `TrapState`, the activation gate,
`LockRules`, `LockRequest` / `LockOutcome`, the `LockInteraction` seam, outcome noise, force, traps, their tests.

### Prompt and HUD

`InteractPrompt { target, name, owned: Option<OwnerLabel>, taking_is_crime: bool, stolen_count: u32, lock:
Option<LockView>, trapped: bool, pickpocket: Option<PickChanceView> }` (LW8) is built from the host's own pure functions
(`use_allowed`, `gate_activation`, `LockRules` preview), so the prompt cannot disagree with the outcome. Owned and
crime-if-taken marks the crosshair and the item row; `LockView` shows the tier label. `StealthView { sneaking,
exposure: Hidden | Caution | Detected }` (LW12) drives one sneak eye: exposure = `PlayerDetection.band` (ai.md "Net,
saves, HUD"): Unaware is Hidden; Curious, Suspicious, Searching are Caution; Alerted is Detected. Classic shows
Morrowind's sneak icon only. Screens send intents only; themes style `sf-crime-*` parts.

### Commands and events (net rules 2, 5, 8)

Client to host: `OffenceAttempt` (emitted by host systems; scripts through `CrimeCommand`), `LockRequest`,
`ArrestRequest`, `ReportRequest::Stop`, `Take { item, from }` is the existing inventory command (the host derives the offence).
Host to clients (each with tick, player): `ReportChanged { report, state }`, at delivery `OffenceCommitted { id,
bounty_added, jurisdiction, cause: BountyCause }`,
`BountyChanged { jurisdiction, total }`, `Pardon`, `ArrestOffered/ArrestEnded`, `SentenceStarted/SentenceServed`,
`LockChanged`, `TrapSprung` (fields in crime-locks.md), `StolenChanged { item, count }`, `Refused { reason }`.

## Co-op (net-ready rules)

1. **Fixed tick**: offences, lock attempts and arrests apply on the host tick; rolls seed from (player, object or
   victim, attempt, tick), splitmix64 as services SV1.
2. **Commands only**: screens and scripts send requests; nothing writes bounty, the ledger or lock state directly.
3. **State apart**: all of `sf_crime` is serde data; presentation (prompt, eye, fades) never feeds gameplay.
4. **Pure steps**: ownership, assess, reactions, gate, pick, probe, detection, fine, jail days are pure.
5. **One owner**: host-owned. The arrest session claims the guard (provider claim, rpg.md) so two players cannot
   share one guard's dialogue; lock and ownership state is world state, one copy on the host.
6. **Crime is per player** (decision 15): `OffenceAttempt.offender` is a `PlayerId`; ledger, `CrimeLedger`, bounty
   and sentence are per player. Another player is never a witness, allies never report (as Morrowind), a guard
   pursues the offender only; `SharedBounty` comes later. Co-op jail is `JailClock::Credit`: the others play on and
   the shared clock moves only on a passed party rest vote (net.md "Time, pause and rest").
7. **Stable ids**: `ObjectKey`, `ActorKey`, `PlayerId`, `RecordId`, `CrimeId`.
8. **Events are data**; `StolenChanged` precedes the HUD refresh.

## Saves

- `sf/crime` (owner `sf_crime`, v1, optional): `players: Map<PlayerKey, PlayerLaw { stolen: Vec<(RecordId, OwnerKey,
  Vec<Lot>)>, crime: Vec<(JurisdictionId, CrimeLedger)>, reports: Vec<Report>, sentence: Option<Sentence> }>`;
  `objects: Vec<ObjectLaw { object: ObjectKey, lock: Option<LockState>, trap: Option<Option<RecordId>>, ownership:
  Option<Ownership> }>`, a sparse overlay of changed objects (lock, trap: crime-locks.md "Saves"). Empty parts omitted.
- Elsewhere: bounty is a stat (`sf.character`), witness crime ids are AI facts, heat is AI29's directive, evidence
  stores are containers.
- Arrest sessions are not saved (the guard re-offers on sight). A `Sentence` resumes mid-scene with its `JailClock`
  (LW15). Unknown object keys are dropped and reported (save.md). Shape golden for v1; no migration yet.

## Performance

Event-rate only: an offence is one assess call (<= 64 candidate witnesses: <= 0.1 ms excluding the AI query; the
AI query is budgeted in ai.md, batched LOS <= 2 rays per witness), a lock attempt is arithmetic, a ledger op is
O(log n) over at most a few hundred entries, `reconcile` runs per inventory change (<= 5 microseconds). The
prompt is built for the one focused object (<= 10 microseconds, never per object per frame). **No per-frame scan of
guards**: noticing a wanted player is a perception-driven event, not a loop (OpenMW polls it per actor per frame,
`actors.cpp` 1153-1225).

## Industry comparison

| Game | Technique | Us |
|---|---|---|
| OpenMW / Morrowind | owner or faction + rank, global lifts; witness = LOS + roll; bounty global; alarm 100 reports at once; guards pursue above a threshold, arrest dialogue; jail with skill loss; stolen map by item id | **match** (adapter), except the report walks (owner, Classic too); ledger adds reconcile, jail loss and the prompt marker are hooks |
| Oblivion | same model, bounty global, fences, jail loss | **same rules**, different numbers |
| Skyrim | crime factions per hold, bounty per faction added at once by any witness, trespass areas, witnesses must see or hear, fences, Thieves Guild clears bounty, stolen items marked per stack, one graded sneak eye | **option**: jurisdictions, `crime.clear`, Meter model; **match**: one eye from `PlayerDetection`; per-stack marker rejected (stack splitting) |
| KCD, KCD2 | witnesses walk to guards or confront; fines, jail, flee, talk out; reputation per town; stealth by noise and light; players call unseen bounties unfair | **match**: walked `Report`, `ArrestOption` data, `BountyCause` in the journal, per-town jurisdictions |
| RDR2 | eyewitness runs to the law and can be stopped (threat, pay, beat, kill); seen versus identified; masks fool civilians, not lawmen; bounty cools with time away | **match**: `ReportState` with cancel causes, `Recognition` with the law tier, `decay_per_day` |
| Thief; TLOU2, Hitman | light gem (the player's own light); graded awareness per NPC | **match**: light from `LightExposure` (AI8d), graded bands in `sf_ai`; per-NPC markers rejected (owner: one eye) |
| Hitman | disguises pass most NPCs; staff of the disguise's role see through it | **match**: `Disguise` item data, one tier higher for law and acquaintances |
| GTA, Cyberpunk 2.0 | wanted level or heat: pursuit around the last known position, lost by breaking sight and waiting; escalating units | **match**: heat = AI29 over AI8f LKP, apart from the bounty debt; ladder per jurisdiction |
| Elden Ring / Souls | no crime system (NPC aggro and a world event per murder) | covered by `Reaction::Fight` via the same assess |

Gaps: bribe/persuade options (LW21), trespass zones (LW19) (industry-audit.md).

## Tests

Bevy-free, exact assertions, synthetic records and GMSTs (`sf_crime` tests):

- **Ownership**: actor-owned is `Owned { victim }`; a faction owner with `min_rank 3` is owned at rank 2 and
  free at 3; a non-member is owned; `lifted_by` global nonzero frees both; a container hands its owner to contents.
- **Ledger**: `record_taken` skips currency and dead owners; `reconcile` clamps, oldest lot first; `cool` drops by
  days or distance, never under `Permanent`; `confiscate` follows map order; `stolen_from` matches merchant or faction.
- **Offences**: `BountyRule` Flat, PerValue (minimum 1 when the product floors to 0), FromArg; unknown kind is an
  error; `assess` is total over empty witness lists and never changes bounty.
- **Reports**: delivery applies the bounty once per `CrimeId` to the guard's jurisdiction; each cancel cause files
  nothing; recognition tiers exact (disguise, law, acquaintance; Classic `Identified`).
- **CrimeLedger**: ids increase; `Pardon` through `paid_through` calms exactly the witnesses with ids `<=`.
- **Law flow**: `stance` ladder exactly at each boundary, kill on sight only for murder; `Warn` window then `Arrest`; Flee and Resist raise heat, never the stance; decay stops at the floor (Classic none); arrest options; Pay then Jail refuse a second time;
  `jail_days` floor; `PassTime` moves the clock, `Credit` never; a saved sentence resumes; goods confiscated once.
- **Locks and traps**: [crime-locks.md](crime-locks.md) "Tests".
- **Detection**: `StandardDetection` exact per term (light, stance, behind, speed reveal, clamp); upright, still, in
  front equals light-only visibility (model, roll and meter tests are `sf_ai`'s).
- **Witnesses**: a fake `WitnessQuery`; allies and the dead never report; deterministic order.
- **Co-op** (headless, two `PlayerId`s): A steals, only A's bounty and ledger change; B is not a witness; both open
  the same chest, one `LockChanged`; a second `ArrestRequest` on a claimed guard fails `Busy`.
- **Saves**: `sf/crime` serde round trip, shape golden, sparse overlay equals base after revert, unknown key dropped.
- **View-models**: prompt marker equals `use_allowed`; stolen count; lock row hidden when `level == 0`.

## Build plan

Slices, dependencies and checks: [crime roadmap](../../../docs/roadmaps/crime.md) §4. Stage order: 0 specs LW0a-e;
1 owned things and locks LW1, LW3, A0, A1, A5, LW6, LW7, LW11, LW8; 2 sneak LW4 (built), MA5, LW12, LW13; 3 reports
and law LW2, LW2b, LW5, A2, A3, LW9 (first bounty), LW14, LW15 (jail), LW16, LW17; 4 a thief's trade LW18, A4,
LW19-LW21, LW10, LW22; 5 Classic A8. Rules first: LW1-LW5 need no AI, Bevy or UI.

## Open questions (owner)

All pending owner (roadmap §7); each answer is the recommendation, which the spec follows until then.

1. **Minigame feel?** Lockpick sweet spot (Skyrim, KCD2), pickpocket KCD2 hold-to-search; "auto" and Classic roll.
2. **Can headgear hide who you are?** Yes in Modern, law and acquaintances see through it; a `Description` report
   charges once guards identify the player; Classic off.
3. **One bounty or one per region?** One in Morrowind, locality from heat and gossip; framework per jurisdiction.
4. **Jail without skill loss in Modern?** Yes, plus a reputation hit; Classic keeps skill loss.
5. **Do stolen goods cool off?** Yes in Modern, fences buy hot goods at a discount; Classic permanent.

Answered:

- Solo jail clock (owner 2026-11-25): the days pass as in Morrowind (`JailClock::PassTime`).
- Co-op scope (2026-11-25, decision 15): per player, `SharedBounty` later.
- Stolen state (2026-11-25, decision 11): reconcile plus cool-down, Classic permanent.
- Detection model (2026-11-25, decision 6): Meter default, Classic roll.
- Co-op jail (2026-11-25, decision 12): days credited, shared clock fixed.
- Arrest options (2026-11-25, decision 13): data rows, Persuade and Bribe in LW21.
