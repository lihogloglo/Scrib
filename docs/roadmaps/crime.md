# Lane roadmap: Crime and stealth (ownership, locks and traps, witnesses, law, sneaking)

Specs: `superfargoth/docs/specs/{crime,world-reactions}.md`, `bevyrrowind/docs/specs/crime-adapter.md` (slices A0-A8),
ai.md "Perception and stimuli". Research: `docs/research/gameplay-2026-11-25/crime.md` (§ = its sections). The first
build slice creates `docs/ledger/crime.md`. New slices start at LW11; AI22, AI29, MA5, MA6 are the AI lane's, IN33,
IN37 the interaction lane's.

## 1. Player-facing goal

Modern preset, lane done: pick locks with a skill-shaped minigame, probe traps, use keys and spells, or **bash**
loudly; the prompt shows owned items, lock tier, traps; shops refuse fresh stolen goods [1]. **Sneak**: one
Skyrim-style eye shows how noticed you are; light, stance, speed and each person's senses decide; failed picks are
heard [2]. A seen crime is **walked to the law** by a witness you can chase, pay or silence; a guard warns, arrests
(pay, jail, resist, flee), pursues, searches, cools off; the bounty stays and the journal says who reported what [3].
Pick pockets under a risk meter, get warned off private rooms, hide your face, get frisked, fence hot goods [4].
**Classic keeps** Morrowind's dice (locks, pickpocket, one seeded roll per sight check), permanent stolen flag, jail
skill loss, one bounty; witnesses walk to report there too (owner 2026-11-25) [5].

## 2. Where we are (2026-11-25)

- **Not started:** `sf_crime`, `sf_crime_bevy`, `mw_crime` (no crates), every LW and A slice; ROADMAP 2.8 `[ ]`; no
  LW- visual check, no ledger.
- **Built that crime uses:** bounty `Counter` (`mw_stats/src/table.rs:253`), `Get/Set/ModPCCrimeLevel` live (no
  `recordCrimeId` at 0); `GetPCSneaking`, `Lock`/`Unlock`/`GetLocked` live in `mw_script/src/functions.txt` but no
  lock, key, trap or owner field loaded on references (A0); `PayFine`, `GoToJail` later, `GetPCInJail`, `GetDetected`
  stubs; `Sneak` mode flag (`sf_action`); `fDispCrimeMod` in `derived_disposition`; AI8a meter + `PlayerDetection`,
  AI8e `Notify` (headless); specs AI0b, AI0g done.
- **Owner (DECISIONS 2026-11-25):** witnesses walk and can be stopped, both presets; surrender except kill-on-sight
  for murder above a threshold; lockpick minigame TBD; bashing by strength, a crime if reported (vandalism if owned);
  sneak eye, no per-NPC markers; per-individual `PerceptionProfile`.

## 3. Decisions

1. **Keep the pure rules core** (§0, §6: no crate fits; OpenMW ported): `sf_crime` Bevy-free, thin `sf_crime_bevy`,
   `mw_crime`; ownership, offence records, bounty `Counter` per jurisdiction, `CrimeLedger` + `Pardon` as spec'd.
2. **Reports sit between "seen" and "wanted"** (RDR2, KCD2; §1; owner): `OffenceObserved` stays event-time (ai.md);
   `assess` returns reactions and a `ReportPlan`, **no bounty**; each reporter gets `ReportState { Pending,
   Walking(guard), Delivered, Cancelled }`, walked by AI22; bounty applies on `ReportDelivered` to the guard's
   jurisdiction, both presets (Classic: Morrowind's sums and `reportCrime` rules, at delivery). Cancel: death,
   knock-out, bribe, threat, timeout. **Departs** from crime.md (bounty at `assess`) and §1.1 (Classic instant).
3. **Fairness** (§1.3): every bounty carries its cause (reporter, place, item), shown in the journal.
4. **Recognition** (RDR2, Hitman; §1.2): `Unknown | Description | Identified` per report from band, visibility, gear
   `Disguise` (item data), higher tier for law and acquaintances; `Unknown` feeds AI23 suspicion. Classic: Identified.
5. **Bounty and heat apart** (GTA, Cyberpunk 2.0, RDR2; §2): bounty = debt; heat = AI29 pursuit on AI8f LKP search,
   cools off. Per-jurisdiction ladder (data) `Warn` → `Arrest` → `Pursue` → `Attack`; fleeing raises heat; kill on
   sight only for murder above a threshold (owner); `decay_per_day` with a murder floor (Classic none).
6. **Crime owns the formula, AI owns detection** (ai.md "Perception", AI0b, AI0g): `concealment` reads
   `VisibilityFactors`; meter, Classic roll (one seeded roll per sight check), `LightExposure` (AI8d) and
   `PlayerDetection` (max over observers) are `sf_ai`'s; `StealthView.exposure` = its band; one sneak eye, no per-NPC
   markers (owner). Duty, drink, sleep come from `PerceptionProfile`/states, never crime code. Meter default.
   **Departs** from §3 (per-NPC indicators, crime reading light) and crime.md (`light: Option<f32>`, nearest observer).
7. **No witness scaffold** (CONTRIBUTING "right from the start"; AI8a built, AI8b next): `WitnessQuery` is implemented
   once, by `sf_ai_bevy`'s event-time query; tests use a fake; `ScaffoldWitnesses` dropped. Until Stage 3 offences are
   ledgered and logged, no bounty.
8. **Locks are crime's, doors are interaction's** (settles crime.md vs IN33, IN37, DQ-7): crime owns `LockState`
   (+ `hp`), `TrapState`, `gate_activation`, `LockRules`, `LockRequest`/`LockOutcome`, keys, trespass and the `sf/crime`
   overlay. IN33 owns the swing slot (Open/Closed, hinge, queue) and reads "Locked" from the gate. They meet on IN31's
   contract: the crime gate runs before interaction's default and marks Locked/TrapSprung handled; cores never name
   each other. IN37 computes the blow (strength, weapon, contents, impact noise) and sends `LockRequest::Force`; crime
   settles HP, `Broken` and the offence. DQ-7 `Lock`/`Unlock`/`GetLocked` act on `LockState`.
9. **Lock seam** (Skyrim, KCD2; §5): `LockInteraction { Roll | Minigame }`, same `LockOutcome`; the ported `pickLock`
   term (A5) sets the sweet-spot window and jam rate, tier the break chance; only a data `Impossible` flag refuses;
   five tier labels on the prompt. Host-seeded sweet spot, client input stream, host re-simulates. Roll = Classic and
   an "auto" setting. Pickpocket takes the same shape (KCD2 timer, risk from victim awareness; Classic roll).
10. **Outcome noise** (UE `ReportNoiseEvent`, KCD2; §3b): `sf_crime_bevy` emits `NoiseEvent` for failed and snapped
    picks, sprung traps, forced locks; IN18 keeps physical noise (impacts, doors). Open spell is quiet.
11. **Ownership extras** (KCD2, Skyrim; §4): ledger reconcile **plus cool-down** in Modern, permanent in Classic;
    fences framework on, Morrowind off; `Restricted` zones warn first; possession is evidence (frisk).
12. **Jail** (§2): Modern = days, confiscation, reputation, no skill loss (Classic hook kept); a short local scene that
    never moves the shared clock, days credited. **Departs** from crime.md Co-op 6 (time skipped for one player).
13. **Arrest options are data** (KCD2): Pay, Jail, Resist, Flee, Surrender; Persuade, Bribe later. Morrowind's guard
    dialogue drives its arrest through DQ-15 hooks.
14. **Adapter split**: MA5 (`mw_ai`) owns the `awarenessCheck` port (crime-adapter A6 folds in); `mw_crime` owns A2
    offences and A3 law and jail; MA6 is only the glue from stance to guard directives.
15. **Net, data, cost**: host-authoritative, fixed tick, commands; crime per player (a partner never witnesses);
    offences, jurisdictions, tiers, zones are records; event-rate cost only, no per-frame guard scan.

## 4. Stages

Owner checks `LW-n` go in `bevyrrowind/docs/visual-checks/crime.md` (new). `sf_crime_bevy` = `scb`, `sf_rpg_ui` = `ui`.

### Stage 0: specs (docs only)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| LW0a | crime.md "Witnesses", "Reactions": decisions 2-5, 13 | docs | - | doccheck |
| LW0b | crime.md "Stealth", "Prompt and HUD": decision 6 | docs | AI0b, AI0g | doccheck |
| LW0c | crime.md "Locks and traps": decisions 8-10 | docs | - | doccheck |
| LW0d | crime.md ownership, jail, build plan, open questions | docs | LW0a-c | doccheck |
| LW0e | crime-adapter.md: decision 14, A2 at delivery, options | docs | LW0a, LW0b | doccheck |

### Stage 1: locks, keys, traps, owned things

Ends with: in Seyda Neen, pick the Census office chest, probe a trapped one, open a door by key, bash a crate lock;
the prompt shows owned, stolen, tier, trapped; Arrille refuses the stolen goods.

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| LW1 | Ownership, `use_allowed`, ledger (reconcile, cool-down, confiscate), `StolenVeto`, fences | `sf_crime` | LW0d | exact |
| LW3 | `LockState`(+hp), `TrapState`, gate, `LockRules` pick/probe/force, `Roll`, trespass | `sf_crime` | LW0c | exact, seeded |
| A0 | Reference owner/lock/key/trap fields, load rule, `MwCrimeGmst` | `mw_crime`, `mw_world` | LW3 | exact; real-install counts |
| A1 | `isOwned`, `isAllowedToUse`, Classic stolen map, services veto | `mw_crime` | LW1, A0 | exact vs OpenMW |
| A5 | `pickLock`/`probeTrap` terms, keys, traps, Open spell, `Lock`/`Unlock` (DQ-7) | `mw_crime` | LW3, A0 | exact vs `security.cpp` |
| LW6 | Host: requests/events, gate before IN31 default, take, veto, `LockChanged` | `scb` | LW1, LW3, IN31 | headless, two players |
| LW7 | `sf/crime` save: lock/trap overlay, ledger | `scb`, `sf_save_bevy` | LW6 | round trip, goldens |
| LW11 | Lock minigame: seeded sweet spot, window from `PickIn`, jam/snap, input stream | `sf_crime`, `ui` | LW6, Q1 | re-simulation; owner LW-2 |
| LW8 | `InteractPrompt`: owned, stolen count, tier label, trapped | `ui` | LW6, A5 | exact vs rules; owner LW-1 |

IN37 (interaction) lands the bash on `LockRequest::Force` once LW6 exists.

### Stage 2: sneak (with AI Stage 1)

Ends with: a Balmora guard loses you in shadow, finds you in lamp light; the eye fills and empties; a snapped pick
brings someone to look; a drunk notices nothing.

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| LW4a | New Bevy-free crate `sf_crime` (hakari excludes): `DetectionRules`, `DetectIn`, `StealthState`, `StandardDetection` (modern default: stance, speed, light; tunables) | `sf_crime` | LW0b (spec section written) | **done 2026-11-25** (10 exact tests; ledger crime) |
| LW4b1 | Sight seam: `sf_ai_bevy` sight builds `DetectIn` per check and takes visibility from an `AiDetectionRules` resource (default `StandardDetection`) instead of `light_only_visibility`; target `AiVisibilityFactors` component (stance, modifiers) the game writes; Meter only (Classic with MA5) | `sf_ai_bevy` | LW4a | **done 2026-11-25** (4 headless tests; upright exact, crouched less, 100 hides) |
| LW4b2 | `sf_crime_bevy` (new): plugin binds `crime.stealth.*` tunables live and installs the rules into `AiDetectionRules` | `sf_crime_bevy` | LW4b1 | headless: a tunable change reaches the rules |
| MA5 | (AI lane) `awarenessCheck` as Morrowind `DetectionRules`, Classic roll | `mw_ai` | AI8a, LW4 | exact vs formula |
| LW12 | `StealthView` from `PlayerDetection.band`, sneak eye (Classic: icon) | `scb`, `ui` | LW4, AI8b, AI8d | exact; owner LW-3 |
| LW13 | Outcome noises: failed/snapped pick, trap, forced lock; `NoiseKind` rows | `scb`, `mw_crime` | LW6, AI8c | exact loudness |

### Stage 3: seen, reported, wanted (with AI Stage 3)

Ends with: steal in a Balmora shop; the clerk walks to a guard (stop her or not); the guard warns, arrests (pay, jail,
resist, flee), pursues, searches, cools off, bounty kept; the journal names the reporter.

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| LW2 | `OffenceDef`, `assess` → reactions + `ReportPlan`, `CrimeLedger`, `Pardon`, `WitnessQuery` | `sf_crime` | LW0a | exact; no bounty at assess |
| LW2b | `ReportState`, `Recognition`, heat record, pure transitions | `sf_crime` | LW2 | exact, cancel causes |
| LW5 | Ladder, murder threshold, `ArrestSession`, options, `Sentence`, `JailRules`, decay | `sf_crime` | LW2 | exact, fake clock |
| A2 | Offence table, `commitCrime`, `reportCrime` at delivery, `recordCrimeId` | `mw_crime` | LW2, LW0e | exact vs OpenMW |
| A3 | Thresholds, `PayFine`/`PayFineThief`/`GoToJail`, Classic skill loss, bed rule | `mw_crime` | LW5, A2 | exact vs OpenMW |
| LW9 | `WitnessQuery` from AI8b, reports to AI22, bounty on delivery, `Pardon` | `scb`, `sf_ai_bevy` | LW2b, LW6, AI8b | stopped reporter files nothing |
| LW14 | Arrest: guard claim, dialogue options, flee/surrender into AI29 heat | `scb`, `ui` | LW5, LW9, DQ-15 | headless per option |
| LW15 | Jail: local scene, confiscation, days credited, release; sentence + reports saved | `scb` | LW14, LW7 | resume; clock unchanged |
| LW16 | Recognition: item `Disguise`, law/acquaintance tier, descriptions → AI23 | `scb`, `mw_crime` | LW9, AI23, Q2 | exact tiers |
| LW17 | Crime record: each bounty's cause in journal and stats screen | `ui` | LW9 | exact; owner LW-4 (with AI-5) |

AI22, AI29, MA6 (AI lane), DQ-15, DQ-31 (dialogue lane) complete it.

### Stage 4: a thief's trade

Ends with: lift a purse at the Balmora market; get told to leave a private room; a suspicious guard frisks you.

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| LW18 | Pickpocket: hold-to-search timer, risk from victim awareness, input stream | `sf_crime`, `scb`, `ui` | LW9, Q1 | re-simulation; owner LW-5 |
| A4 | Morrowind pickpocket rule (Classic roll), window path | `mw_crime` | LW18 | exact vs OpenMW |
| LW19 | Trespass zones: `Restricted` records, `Warn`, grace, offence on ignore | `sf_crime`, `scb` | LW5, AI28 | exact timings |
| LW20 | Frisk: suspicion → search → possession offence, confiscation | `scb` | LW9, AI23 | unwitnessed theft found |
| LW21 | Arrest options Persuade, Bribe (speech stats) | `sf_crime`, `mw_crime` | LW14 | exact chances |
| LW10 | NPC culprits: `Culprit` in offences and reports | `sf_crime`, `scb` | LW9 | NPC theft seen and not |
| LW22 | Traps: proximity/tripwire triggers, detection reveals, NPCs trigger | `sf_crime`, `scb` | LW6, AI8b | exact triggers |

AI43 (`Wanted` spread at gossip speed) and AI44 reputation complete it.

### Stage 5: Classic preset

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| A8 | Options UI for `MwCrimeOptions` (detection, locks, pickpocket, ledger, jail, jurisdictions) | `mw_crime`, `mw_ui` | Stage 4 | exact rows; owner LW-6 |

**Later:** werewolf bounty (werewolf lane), `SharedBounty` co-op option, stealth takedowns (combat lane).
**Dropped:** `ScaffoldWitnesses` (decision 7), A6 (into MA5), LW8's arrest buttons (into LW14).

## 5. Spec changes (spec slices before build)

- **LW0a** crime.md: `Assessment { per_witness, reports }` without `bounty`; `ReportState`, `ReportDelivered`,
  `Recognition`, heat, ladder, kill-on-sight, decay, fairness; `OffenceCommitted` at delivery (fix 4: owner, no instant
  report in Classic either).
- **LW0b** crime.md: (1) `StealthView.exposure` = `PlayerDetection.band` (max over observers, owner Q1 eye), not
  "nearest awareness"; (2) `DetectIn.factors: VisibilityFactors` replaces `light: Option<f32>` (ai.md "Light
  exposure", AI8d); (3) Classic = one seeded roll per sight check in `sf_ai`, `roll` leaves `DetectIn`; (5) observer
  differences come from `PerceptionProfile`/states (AI0g), `ObserverView` carries no duty or mood.
- **LW0c** "Locks and traps": decisions 8-10 with the IN33/IN37/DQ-7 contract. **LW0d**: decisions 7, 11, 12, build
  plan, answered questions. **LW0e** crime-adapter: A2 at delivery, A6 into MA5, options table, questions 1, 2, 4.
- Other lanes: world-reactions AI22 on `ReportState` (AI lane); IN27/IN29 adopt decision 8.

## 6. Dependencies on other lanes

| Lane | Crime needs | Crime provides |
|---|---|---|
| AI | AI8b (sight + event-time query), AI8c, AI8d, AI8f, MA5, AI22, AI29, AI23, AI28, AI43, MA6 | LW2b types, LW4 formula, LW5 stance and ladder, LW10, offence rows for MWR1 |
| Interaction | IN31 activation contract, IN33 doors, IN37 bash blow, IN18 physical noise, IN14 | lock model and gate (IN33), Force settlement (IN37), `use_allowed` (IN16), vandalism row |
| Dialogue-quests | DQ-15 fine/jail hooks, DQ-31 interrupt, DQ-25 witness barks, journal | `LockState` for DQ-7, `PayFine`/`GoToJail`/`GetPCInJail` targets, crime events |
| Combat | assault/murder `OffenceAttempt` on hits, sneak-attack multiplier, takedowns (IDs TBD) | witness and stance reactions |
| Stats, services, save, anim | `Counter`, `DealVeto`, `sf_save_bevy`; Kimodo pick/search/surrender clips | `StolenVeto`, `sf/crime` |

## 7. Owner questions (each with a recommended answer)

1. **Lockpicking and pickpocketing minigames: which feel?** Recommended: lockpick = rotate to a sweet spot (Skyrim,
   KCD2), skill and pick quality widen it, a wrong turn jams and snaps the pick loudly; pickpocket = KCD2 hold to
   search with a risk meter; an "auto" setting rolls instead; Classic rolls Morrowind's dice.
2. **Can a helmet, hood or mask hide who you are**, so a civilian reports "a stranger in a Netch helm" and guards
   search for that look? Recommended: yes in Modern, guards and people who know you see through it; Classic off.
3. **One bounty for all Vvardenfell, or one per region?** Recommended: one island-wide bounty in Morrowind (its
   dialogue and scripts read one value); locality comes from heat and gossip: far guards learn only through `Wanted`
   spread at gossip speed. The framework keeps per-jurisdiction bounty for other games.
4. **Jail: time and confiscation without skill loss in Modern?** Recommended: yes, plus a reputation hit; Classic
   keeps Morrowind's skill loss; in co-op the jailed player serves a short scene while the world goes on.
5. **Do stolen goods cool off?** Recommended: yes in Modern, after days or distance ordinary merchants buy them,
   fences buy fresh ones at a discount (none in Morrowind content unless a mod adds them); Classic stays stolen forever.
