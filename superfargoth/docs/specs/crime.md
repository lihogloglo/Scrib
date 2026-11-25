# Crime, law, stealth, locks and traps (`sf_crime`, `sf_crime_bevy`, `sf_rpg_ui`)

Status: spec written 2026-11-10, revised 2026-11-25 (LW0: AI perception design, walked reports, lock contract;
plan in the [crime roadmap](../../../docs/roadmaps/crime.md), "decision N" below = its §3); nothing built. Builds on [stats.md](stats.md) (`Counter` stats such as bounty,
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

Locks follow Bethesda's numbers shape: a lock level, a tool quality, a skill term, a roll, tool wear; traps are an
effect fired at the actor. Stealth: concealment versus observer perception; Bethesda rolls per check, modern
stealth games (Skyrim, Dishonored, Thief) fill a meter over time. We offer both behind one rule.

## Reuse (Bevy, then crates, then our own)

Bevy: events, components, `FixedUpdate`; nothing for crime. Crates: none (a game mechanic). Ours, existing:
`sf_stats` (bounty counter, `StatOp::UseSkill`, `CounterSet`), `sf_rpg` (`RpgCommand`, `service::DealVeto`),
`sf_effects` (`Inflict` for a trap), `sf_ids` (seeded rolls, `ActorKey`, `PlayerId`), `sf_records` (offence and
jurisdiction defs as records), the AI lane's fact store and `OffenceObserved`, `sf_save_bevy` sections.
New: `sf_crime` (rules, ledger, lock state) and `sf_crime_bevy` (host). Interaction ("activate this") has no
`gate_activation` runs before interaction's default on IN31's contract ("Locks and traps").

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

`Ownership` is content data on a placed object (and on containers, doors, beds, actors' inventories) and a sparse
runtime overlay (an adapter script may change it). **Containers pass their owner to what is taken from them**; an
actor's inventory is owned by the actor.

`OwnerRules::owned_for(&OwnerCtx) -> Ownership_Verdict { Free | Owned { victim: Option<ActorKey>, faction: Option<RecordId> } }`
is pure: the actor's standing in the faction (`min_rank`), the lifting global's value and "is this the actor's own
property" are inputs. `use_allowed(&UseCtx) -> bool` adds the special cases (dead actor, actor in combat, sleeping
or knocked-out victim for pickpocket, evidence chest: the adapter names it as `never_take`).

**Stolen state is a ledger, not a stack flag.** `StolenLedger { entries: BTreeMap<(RecordId item, OwnerKey), u32> }`,
one per player (a remote player's goods are theirs alone). Stack attributes would split stacks that merge by
equal attrs and would need an owner on every unit; the ledger keeps one count per (record, owner). Rules:

- `record_taken(item, owner, count)`: skipped for currency; skipped when the owner is an actor who is dead or gone
  (nobody to return it to), unless the adapter opts in.
- `reconcile(item, held)`: after **every inventory change** the host clamps the sum of entries for `item` to what
  the player holds, oldest owner first removed. Sold, dropped, consumed or traded goods stop being stolen (OpenMW
  never does this, `mechanicsmanagerimp.cpp` 1079-1135: stale entries make later honestly bought copies "stolen").
- `confiscate(held) -> Vec<(item, count)>`: removes entries in `BTreeMap` order (deterministic), returns goods to take.
- `stolen_from(item, merchant)`: owner is the merchant actor or the merchant's primary faction.
- `cool_down` (decision 11): Modern, an entry ages out after `cool_days` or `cool_distance_m` from its owner (then
  ordinary merchants buy it; fences buy fresh goods at a discount); Classic, entries are permanent.
- View: `stolen_count(item)` for the HUD; a stack shows "N stolen" without being split.
- **Zones and evidence** (decision 11): `Restricted` zone records warn first, then after a grace the stay is a
  trespass offence (LW19); held stolen goods are evidence: a frisk (AI23 suspicion, then search) that finds ledger
  entries files a possession offence and confiscates (LW20).

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
arg: i32, faction: Option<RecordId>, tick }` is the one input; gameplay code (taking items, opening doors, hits,
kills, picking pockets, sleeping in beds, forcing locks, spells) emits it, nothing else touches bounty.
`assess(&OffenceCtx) -> Assessment { per_witness: Vec<WitnessReaction>, reports: ReportPlan }` is pure and adds **no
bounty** (both presets): bounty applies when a report is delivered ("Reports"). Superseded 2026-11-25: `Assessment.bounty`
at assess (decision 2; owner: no instant report, in Classic too).

`WitnessReaction { witness, reporter: bool, reaction: Reaction, disposition_delta, permanent: bool }` with
`Reaction = Ignore | Flee | Fight | Pursue | Say(topic)`. `ReactionRules` (adapter) fills them from the witness's
data (alarm, fight, guard flag) and the offence def. The host forwards `Pursue`/`Fight`/`Flee` to the AI lane as
directive requests (ai.md) and each reporter's report to AI22's walking `Report` directive.

### Jurisdictions and bounty

`Jurisdiction { id, name, bounty_stat: StatKey, ladder: Vec<(i32, Stance)>, kill_on_sight: Option<i32>,
decay_per_day: i32, murder_floor: i32, jail_place, guard_class... }` is a record; `JurisdictionRules::of(&WitnessCtx) -> JurisdictionId` says which one a witness or position belongs to
(Morrowind: one global; Skyrim: the hold; modern open-world: a region or faction). **Bounty is a `Counter` stat per
player per jurisdiction** (stats.md: `bounty` for the default, `bounty.<id>` otherwise), so MWScript, dialogue
filters and the HUD read it without this crate. Writers: only `CrimeCommand::AddBounty` and `Pay`/`Clear`, both
through `StatOp::CounterSet` (floor 0).

**Bounty and heat are apart** (decision 5; GTA, Cyberpunk 2.0, RDR2): bounty is a debt, kept until paid, served or
decayed (`decay_per_day`, never below `murder_floor` once murder is on record; Classic: no decay); heat is AI29's
pursuit over AI8f's LKP search and cools off when the player is lost. The ladder (data) runs `Warn` → `Arrest` →
`Pursue` → `Attack`; fleeing raises heat, not the stance; kill on sight only for murder above `kill_on_sight`
(owner). **Fairness** (decision 3): each bounty carries its `BountyCause { crime: CrimeId, kind, reporter, place,
item }`, shown in the journal and stats screen (LW17).

`CrimeLedger { next_id, paid_through, causes }` per (player, jurisdiction): every offence batch gets an increasing
`CrimeId`; witnesses remember the ids they saw (AI fact `crime.witnessed`); paying sets `paid_through = next_id - 1`
and the host emits `Pardon { player, jurisdiction, through }`; witnesses with ids `<= through` calm down and restore
their disposition. This is OpenMW's calm-down mechanism (`player.cpp` 414-426, `actors.cpp` 1203-1223) made data.

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
adapter"), event-time, from its own LOS and awareness; `can_report` is false for the offender's allies and followers,
the dead, the unconscious and actors already fighting the victim. `WitnessQuery` has one implementation,
`sf_ai_bevy`'s event-time query (AI8b, LW9); `sf_crime` tests use a fake. Until LW9, offences are ledgered and
logged with no bounty. Superseded 2026-11-25: the `ScaffoldWitnesses` stand-in (decision 7, no interim).

### Reports and recognition

`ReportPlan` names each reporter and its guard or post (`ReportRules`, adapter); each report has `ReportState {
Pending, Walking(guard), Delivered, Cancelled(cause) }`, walked by AI22's `Report` directive (Notify shouts raise
awareness, never bounty). Cancel: reporter dead or knocked out, bribed, threatened, `report_timeout_s` (tunable). On
`ReportDelivered { report, guard }` the bounty goes to the **guard's** jurisdiction, once per `CrimeId`, and
`OffenceCommitted` is emitted; a stopped reporter files nothing. Classic: Morrowind's sums and `reportCrime` rules at
delivery. **Recognition** (decision 4; RDR2, Hitman): `Unknown | Description | Identified` per report from band,
visibility and gear `Disguise` (item data), a tier higher for law and acquaintances (`RecognitionRules`, pure);
`Unknown` feeds AI23 suspicion only. Classic: `Identified`.

### Reactions, arrest, jail

- **Notice on sight** (guards seeing a player over the threshold) is an AI behaviour; this spec supplies
  `LawRules::stance(bounty, jurisdiction) -> Stance { Ignore | Warn | Arrest | Pursue | Attack }` from the ladder, pure.
- **Arrest** is a host-owned `ArrestSession { player, guard, jurisdiction, bounty }` opened when an `Arrest` or
  `Pursue` reaches the player (the dialogue is the screen). `ArrestOption` records (decision 13; KCD2): `Pay` (the
  fine; `LawRules::fine(bounty)` may discount), `Jail`, `Resist` (guard attacks, session closes), `Flee` (heat up),
  `Surrender` (not under kill on sight); `Persuade`, `Bribe` later (LW21). `ArrestRequest { Choose { session, option }
  }` in, `ArrestEvent` out; a game's guard dialogue may drive it (Morrowind: DQ-15).
- **Pay**: bounty to 0, `StolenLedger::confiscate` into the jurisdiction's evidence store, `Pardon`, drawn weapon
  holstered. **Jail** (decision 12): `Sentence { days = LawRules::jail_days(bounty), place }`, bounty to 0, goods
  confiscated, `Pardon`; the host fades, teleports to `jail_place`, plays a short local scene (`jail_scene_s`,
  tunable), **never moves the shared clock**, credits the days via `JailRules::served(days, &PlayerView) ->
  Vec<RpgCommand>` (Modern: reputation hit, no skill loss; skill change is an adapter hook, Classic keeps it), then
  `release_place`. A saved `Sentence` resumes. Superseded 2026-11-25: `PassTime` for the days.
- **Fines on the way out**: `ServiceKind` offers `crime.pay` (thieves' guild style: pay without being arrested)
  and `crime.clear` (script/quest effect: `Clear { jurisdiction }`) reuse services.md; an adapter script can call
  the same `CrimeCommand`s.

### Fences and stolen goods in services

`sf_crime::StolenVeto` implements services' `DealVeto`: a Sell line that is `stolen_from(item, merchant)`
vetoes with `Confiscate` (goods go to the merchant, `OffenceAttempt { kind: theft }` follows, as Morrowind does).
`FenceRules::accepts(&Merchant, &Item) -> bool` marks merchants that buy stolen goods from anyone (modern
thieves' fences; framework default on, Morrowind off: decision 11); the adapter chooses them from content flags. A fenced sale is a normal deal at the fence's
`ServiceRules` price, and `reconcile` clears the ledger entry.

### Stealth and detection

The sneak state is the `Sneak` mode flag in `sf_action::ActionState` (action.md); this spec never moves a character.
`StealthState { sneaking: bool, moving: Speed }` is the read-only view `sf_crime_bevy` builds from it and the
character's stats. One rule seam, pure (decision 6: crime owns the formula, AI owns detection):

```rust
pub trait DetectionRules {
    /// How hidden `target` is from `observer` right now; larger is better hidden. 0..~200.
    fn concealment(&self, i: &DetectIn) -> f32;
}
pub struct DetectIn { pub target: TargetView, pub observer: ObserverView, pub distance_m: f32,
                      pub angle_rad: f32 /* 0 = in front */, pub factors: VisibilityFactors }
```

`VisibilityFactors` is ai.md's (`light` from AI8d's `LightExposure` after the observer's `darkness`); crime never
reads light. `ObserverView` carries stats only, never duty, drink, sleep or mood: those are the observer's
`PerceptionProfile`/`PerceptionStates` (AI0g), scaling sight and hearing in `sf_ai`. `sf_ai` runs the model
(`DetectionModel`, ai.md "Models"): **Meter** (framework default; gain × `1 - concealment / 100`) or **Classic** (one
seeded roll per sight check against concealment), and owns the loop, rolls, `Awareness` and `PlayerDetection`.
Superseded 2026-11-25: `DetectIn.light`/`roll`, Classic as default. Pickpocket takes the lock seam's shape (decision
9): `Timer` (KCD2 hold-to-search, risk from victim awareness, LW18) or `Roll` = `PickpocketRules::caught(&PickIn,
roll) -> bool` (Classic and "auto").

**As built (LW4a, 2026-11-25):** `sf_crime/src/stealth.rs`. `DetectionRules: Send + Sync`; `TargetView { stats: StatView,
stealth: StealthState }`, `ObserverView { stats }` (`StatView`: 8 inline `(StatId, f32)`); `Speed { Still, Walk, Run,
Sprint }` (own data, not `sf_controller::Gait`). `StandardDetection` (modern default) = `light_concealment × (1 − light)` +
stance (crouched 30, prone 50) + `behind_concealment × behind(angle)` (0 within ±`facing_front_rad`, 1 straight behind) −
`min(speed_reveal_per_mps × max(0, speed − speed_free_mps), speed_reveal_max)`, clamped to 0..`max_concealment`; it
ignores distance (sf_ai `f_dist`), size, noise, disguise and `modifiers`. Defaults keep upright/still/in-front equal to
the light-only visibility. `visibility_from_concealment(c) = clamp(1 − c/100, 0, 1)`. Tunables `crime.stealth.*`
(`StandardDetectionConfig`, bound in `sf_crime_bevy`).

**As built (LW4b1, 2026-11-25):** `sf_ai_bevy/src/detection.rs`: `AiDetectionRules(Arc<dyn DetectionRules>)` resource
(default `StandardDetection`, replaceable), optional target component `AiVisibilityFactors { stance, size, disguise,
modifiers }` (absent = upright). `queue_sight` builds one `DetectIn` per check on the stack: light after the observer's
darkness (`VISIBILITY_UNMODELLED` without a sample), speed = |velocity|, distance and angle eye to chest; visibility =
`visibility_from_concealment(concealment)`. Views (stats, `StealthState`) stay default until `sf_crime_bevy` fills them.
`light_only_visibility` removed. Meter only; Classic with MA5.

### Locks and traps

```rust
pub struct LockState { pub level: i32 /* 0 = never locked */, pub locked: bool, pub key: Option<RecordId>,
                       pub hp: u32 /* force damage left */, pub broken: bool, pub impossible: bool /* data flag */ }
pub struct TrapState { pub source: Option<RecordId> }     // an effect SourceDef (effects.md); None = disarmed
```

**Ownership** (decision 8): crime owns everything here plus keys, trespass and the `sf/crime` overlay; interaction
owns the door (IN33's swing slot: Open/Closed, hinge, queue) and reads "Locked" from the gate. On IN31's contract the
crime gate runs before interaction's default and marks `Locked`/`TrapSprung` handled; neither core names the other.
IN37 computes a blow (strength, weapon, contents, noise) and sends `LockRequest::Force`; crime settles `hp`, `broken`
and the offence (vandalism if owned). DQ-7 `Lock`/`Unlock`/`GetLocked` act on `LockState` via `CrimeCommand`.

Unlocking keeps the level (a once-locked object stays recognisable: trespass rule below). Both live on placed
objects with content defaults and a sparse overlay (`sf/crime`). `gate_activation(&ObjectLaw, &ActorView) ->
Gate` is pure and ordered: `Open` | `Locked { pickable }` | `KeyUsed { key, trap_disarmed }` | `TrapSprung { source }`.
A key in the actor's inventory unlocks and disarms; a trap on an openable object fires before it opens.

`LockRequest::{ Pick { object, tool }, Probe { object, tool }, Force { object, damage }, Lock { object, level } }`
in the command stream (host-owned, validated: reach, tool held, tool has uses). `LockRules` (pure; roll seeded from
(player, object, attempt, tick)): `pick_chance(&PickIn) -> Chance { Impossible | Percent(f32) }`, `window(&PickIn)`
(sweet-spot width, jam rate), `tier(level)` (five prompt labels), `probe_chance`, `force(&LockState, damage)`,
`tool_wear(&Tool) -> u32`, `skill_use`. **Seam** (decision 9; Skyrim, KCD2): `LockInteraction { Roll | Minigame }`,
one `LockOutcome`. Minigame: host-seeded sweet spot, `window` from the adapter's ported pick term, break chance from
the tier, client input stream re-simulated by the host (LW11); only `impossible` refuses. Roll: Classic and "auto". Settlement: `LockOutcome { Unlocked | Failed | Jammed | ToolBroken |
Impossible | Disarmed | TrapSprung | Damaged | Forced }` plus `RpgCommand`s (tool use -1, remove at 0) and
`StatOp::UseSkill` on success. **Outcome noise** (decision 10; UE `ReportNoiseEvent`, KCD2): `sf_crime_bevy` emits a
`NoiseEvent` for failed and snapped picks, sprung traps, forced locks (LW13); IN18 keeps physical noise; Open is
quiet. Every attempt (pick, probe, force) emits
`UnlockAttempted { player, object }`, which the crime rules turn into a trespass `OffenceAttempt` if the object is
owned and `level != 0 || locked || trapped` (the Open spell and scripts emit it too). A trap sprung calls effects
`Inflict` from the object at the actor (range rule is an adapter option; classic: the effect targets the actor
if within activation reach, else it detonates at the object).

### Prompt and HUD

`InteractPrompt { target, name, owned: Option<OwnerLabel>, taking_is_crime: bool, stolen_count: u32, lock: Option<LockView>,
trapped: bool, pickpocket: Option<PickChanceView> }` is built from the same pure functions the host uses, so the
prompt cannot disagree with the outcome (`use_allowed`, `gate_activation`, `LockRules` preview). Owned and
crime-if-taken shows a marker on the crosshair and the item row; `LockView` shows the tier label. `StealthView {
sneaking, exposure: Hidden | Caution | Detected }` drives the one sneak eye (owner: Skyrim-style, no per-NPC
markers): exposure = `PlayerDetection.band` (ai.md "Net, saves, HUD": max over observers that count), Unaware →
Hidden, Curious/Suspicious/Searching → Caution, Alerted → Detected; the Classic preset shows only a sneak icon.
Superseded 2026-11-25: "nearest awareness value". Screens send intents only; themes style `sf-crime-*` parts.

### Commands and events (net rules 2, 5, 8)

Client to host: `OffenceAttempt` (emitted by host systems; scripts through `CrimeCommand`), `LockRequest`,
`ArrestRequest::Choose`, `Take { item, from }` is the existing inventory command (the host derives the offence).
Host to clients (each with tick, player): `ReportChanged { report, state }`, at delivery `OffenceCommitted { id, kind,
bounty_added, jurisdiction, reporter, recognition }`,
`BountyChanged { jurisdiction, total }`, `Pardon`, `ArrestOffered/ArrestEnded`, `SentenceStarted/SentenceServed`,
`LockChanged { object, lock, trap }`, `TrapSprung`, `StolenChanged { item, count }`, `Refused { reason }`.

## Co-op (net-ready rules)

1. **Fixed tick**: offences, lock attempts and arrests apply on the host tick; rolls seed from (player, object or
   victim, attempt, tick), splitmix64 as services SV1.
2. **Commands only**: screens and scripts send requests; nothing writes bounty, the ledger or lock state directly.
3. **State apart**: all of `sf_crime` is serde data; presentation (prompt, eye, fades) never feeds gameplay.
4. **Pure steps**: ownership, assess, reactions, gate, pick, probe, detection, fine, jail days are pure.
5. **One owner**: host-owned. The arrest session claims the guard (provider claim, rpg.md) so two players cannot
   share one guard's dialogue; lock and ownership state is world state, one copy on the host.
6. **No singleton player**: `OffenceAttempt.offender` is a `PlayerId`; the ledger, `CrimeLedger`, bounty and
   sentence are per player. **Recommended default: crime is per player.** Another player is not a witness
   (presentation of a remote player is not an observer; no griefing by reporting a friend), a partner never
   reports the offender (allies are exempt as in Morrowind), and a guard pursues the offender only. Option
   `SharedBounty` (party members add to one bounty) is off (later). Jail in co-op (decision 12): the jailed player
   serves the short local scene (`jail_scene_s`), the days are credited to them, the others play on; one player's
   time never advances the shared clock (services' and travel.md's co-op rule; only a passed party rest vote does,
   net.md "Time, pause and rest"). Superseded 2026-11-25: "game time skipped for them only".
7. **Stable ids**: `ObjectKey`, `ActorKey`, `PlayerId`, `RecordId`, `CrimeId`.
8. **Events are data**; `StolenChanged` precedes the HUD refresh.

## Saves

- `sf/crime` (new, owner `sf_crime`, version 1, optional): `players: Map<PlayerKey, PlayerLaw { stolen: Vec<(RecordId,
  OwnerKey, u32)>, crime: Vec<(JurisdictionId, CrimeLedger)>, reports: Vec<(ReportId, ReportState)>, sentence:
  Option<Sentence>, arrest: Option<ArrestPending> }>` (heat is AI29's, saved with its directive); `objects: Vec<ObjectLaw { object: ObjectKey, lock: Option<LockState>, trap:
  Option<Option<RecordId>>, ownership: Option<Ownership> }>` as a sparse overlay on content defaults (only changed
  objects). Empty sections are omitted.
- Bounty is a stat (saved with `sf.character`, stats.md); witness crime ids are AI facts (saved with the AI
  section, ai.md); evidence stores are ordinary containers.
- Open sessions are not saved: saving during an arrest closes the offer (the guard re-offers on next sight).
  A `Sentence` is saved and resumes. Unknown object keys at load are dropped and reported (save.md).
- Shape golden for `sf/crime` v1; no migration yet.

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
| OpenMW / Morrowind | owner or faction + rank, global lifts; witness = LOS + roll; bounty global; alarm 100 reports; guards pursue above a threshold, arrest dialogue; jail with skill loss; stolen map by item id | **match** (adapter); ledger adds reconcile, jail loss and the prompt marker are hooks |
| Oblivion | same model, bounty global, fences, jail loss | **same rules**, different numbers |
| Skyrim | crime factions per hold, bounty per faction, trespass areas, witnesses must see or hear, fences, Thieves Guild clears bounty, stolen items marked per stack, detection meter | **option**: jurisdictions, `crime.clear`, Meter model; per-stack marker rejected (stack splitting) |
| KCD | witnesses report to guards, reputation per town, fines vs jail vs bribe, stealth by noise and light | **seam**: `ArrestOption` extensible, per-town jurisdictions |
| Elden Ring / Souls | no crime system (NPC aggro and a world event per murder) | covered by `Reaction::Fight` via the same assess |

Gaps: bribe/persuade options (LW21), trespass zones (LW19) (industry-audit.md).

## Tests

Bevy-free, exact assertions, synthetic records and GMSTs (`sf_crime` tests):

- **Ownership**: actor-owned is `Owned { victim }`; a faction owner with `min_rank 3` is owned at rank 2 and
  free at 3; a non-member is owned; `lifted_by` global nonzero frees both; a container hands its owner to contents.
- **Ledger**: `record_taken` skips currency and dead owners; two owners kept apart; `reconcile` clamps, oldest first;
  `confiscate` order is the map order and leaves exact remainders; `stolen_from` matches merchant or primary faction.
- **Offences**: `BountyRule` Flat, PerValue (minimum 1 when the product floors to 0), FromArg; unknown kind is an
  error; `assess` is total over empty witness lists and never changes bounty.
- **Reports**: delivery applies the bounty once per `CrimeId` to the guard's jurisdiction; each cancel cause files
  nothing; recognition tiers exact (disguise, law, acquaintance; Classic `Identified`).
- **CrimeLedger**: ids increase; `Pardon` through `paid_through` calms exactly the witnesses with ids `<=`.
- **Law flow**: `stance` ladder exactly at each boundary, kill on sight only for murder; decay stops at the floor; arrest options; Pay then Jail refuse a second time;
  `jail_days` floor; sentence resumes from a saved state; goods confiscated once.
- **Locks**: `gate_activation` order (key unlocks and disarms; trap before open; locked without key); `pick_chance`
  Impossible / Percent; `window` seeded and re-simulated; `force` to 0 hp breaks; tool wear removes at 0; `UnlockAttempted` yields trespass only when owned and (level or locked
  or trapped); an unlocked-but-once-locked owned object is still trespass.
- **Detection**: `concealment` exact for a worked case per term over `VisibilityFactors` (sneak, boots, distance,
  facing, invisibility, light); non-sneaking is always seen (model and roll tests are `sf_ai`'s).
- **Witnesses**: a fake `WitnessQuery`; allies and the dead never report; deterministic order.
- **Co-op** (headless, two `PlayerId`s): A steals, only A's bounty and ledger change; B is not a witness; both open
  the same chest, one `LockChanged`; a second `ArrestRequest` on a claimed guard fails `Busy`.
- **Saves**: `sf/crime` serde round trip, shape golden, sparse overlay equals base after revert, unknown key dropped.
- **View-models**: prompt marker equals `use_allowed`; stolen count; lock row hidden when `level == 0`.

## Build plan (one deliverable per slice; rules-only first: LW1-LW5 need no AI, Bevy or UI)

Slices, dependencies and checks: [crime roadmap](../../../docs/roadmaps/crime.md) §4 (Stage 1 locks and ownership,
2 sneak, 3 reports and law, 4 a thief's trade, 5 Classic). Superseded 2026-11-25: the ten-slice table (scaffold
witnesses dropped, arrest buttons moved to LW14).

## Open questions (owner)

Old questions 1 and 3-7 are answered by roadmap decisions 6, 11-13, 15. Pending, with recommended answers in roadmap
§7: Q1 minigame feel, Q2 disguise (and what a `Description` report charges), Q3 one bounty or per region, Q5 cool-off.

1. **Solo jail and the clock**: **answered (owner 2026-11-25): yes, solo jail passes `days` of game time as Morrowind
   does** (both presets); co-op never moves the shared clock (Co-op 6).