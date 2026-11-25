# Crime, law, stealth, ownership, locks and traps: research note (2026-11-25)

Scope: `superfargoth/docs/specs/crime.md` (LW1-LW10), `bevyrrowind/docs/specs/crime-adapter.md` (A0-A8), `world-reactions.md`,
ROADMAP 2.8. Sibling notes `ai.md` (graded awareness, gossip, LKP search) and `combat.md` are not repeated. Marks: [W] web
source read this session (URL in Sources), [M] from memory, unverified, [P] press/player accounts, not developer statements.

## 0. Verdict

Our spec is a sound **rules core** (ownership as data, offences as data, bounty as Counter per jurisdiction, pure
`LawRules`/`LockRules`/`DetectionRules`, ledger with Pardon). It is the Bethesda 2002-2011 shape with good seams. What the
2018-2026 games add is not more rules but **three physical layers between "crime happens" and "bounty applied"**:
(1) a witness who must *report* (visible, interruptible, delayed), (2) *recognition* (who do they know you are: face,
outfit, mask), (3) a *heat/search* phase (pursuit, last known position, cool-down) separate from the persistent *debt*
(bounty). Our spec has none of the three: `OffenceObserved` adds bounty at event time and `Stance` is a pure function of
bounty. Fixing that is the highest-impact change and costs seams, not a rewrite. Second: stealth HUD and noise; third:
lockpicking as a skill-shaped minigame plus forced entry. No crate exists for any of this (section 6): it stays our code.

## 1. Witnesses, reporting, recognition

**What shipped.**
- RDR2: eyewitnesses have a visible state; the witness runs toward law, the player can chase and "Stop Witness"
  (threaten, pay, beat, kill, lasso, warning shot); killing a witness can create the next witness (chain towards a lawman);
  white-eye icon = saw but does not know you, red-eye = knows who you are; a mask hides you from civilians but lawmen
  recognise through it; bounty payable at post offices [W gamesradar, attackofthefanboy; P] 
- KCD2: witnesses report to guards or confront directly; missing items make locals suspect you; stolen goods carry a
  marker that fades with time or distance from the theft; reputation is per town/faction [W mein-mmo; P]. Players report
  bounties with no visible witness and call it inconsistent [P steam]: the failure mode is **unexplained punishment**.
- Cyberpunk 2.0: five-star heat on a UI scale, patrols give chase on foot or by car, roadblocks, harder to lose heat;
  official statements on witnesses are thin; mods describe civilians reporting after ~10 s unless silenced [W game8, psu; P mods].
- GTA wanted: the canonical "heat" = active pursuit level that decays by breaking line of sight and waiting (search cone around
  last known position) [M].
- Skyrim [M]: crime faction per hold, witness within LOS adds bounty immediately (no report walk), bounty persists.

**Our state.** `WitnessQuery` returns `Witness { saw, heard, can_report }`, bounty is added at event time (`assess`).
`world-reactions.md` already lists "Report is a visible, interruptible directive" (RDR2/KCD2 row) but crime.md's flow does not
consume it: no state between witnessed and reported, no recognition.

**Recommendation (change).**
1. `OffenceObserved` creates a **Report** per reporting witness with `ReportState { Pending, Walking(guard), Delivered,
   Cancelled }`; the AI lane runs it as the visible `Report` directive; bounty applies on `Delivered` (to the guard's
   jurisdiction). Cancelled by: witness killed/knocked out/bribed/intimidated (a `Stop Witness` interaction = dialogue/
   threaten option via `sf_interaction`), or reaching nobody within a timeout. Classic option: instant (today's behaviour).
   Net: report timers on the fixed tick, host-owned.
2. `Recognition { Unknown | Description | Identified }` on each witness record. Mask/hood/helmet/armour set and outfit signature
   (a data `Disguise` stat on equipped items) lower recognition; **law NPCs and acquaintances recognise at a higher tier**
   (RDR2). `Unknown` reports raise a **description** (bounty with `culprit = unknown`, matched later by outfit/face) which
   feeds the existing `world-reactions` suspicion path rather than a new one.
3. Fairness rule (spec-level invariant): every bounty has a retrievable cause (`OffenceCommitted.seen_by`, report log in the
   journal: "reported by X, you were seen stealing Y at Z"). Offscreen reporting is allowed but never unexplained. The HUD shows
   witness state on the NPC (section 3's indicators).
4. Evidence: no new subsystem. "Possession" is the evidence: a guard or owner **frisk/inspect** (`Recognise` hook on the
   `StolenLedger`) converts an unwitnessed theft into an offence on discovery (KCD2: guards search, confiscate). Missing-item
   discovery already routes through world-reactions suspicion.

## 2. Law response

**What shipped.** KCD2: caught options = pay fine, talk out (Speech/Charisma), flee (fugitive state), fight, or alternative
punishment (pillory, beating, branding; execution only after branding for serious crime); repeat offenders get searched more
and guards become more alert and armed; fines scale per offence (e.g. lockpicking 300 Groschen and 10 days jail, pickpocket 60 and
1 day, per one guide) [W mein-mmo; P 3xbz/22wk]; forum reports of bounties fading over days [P]. RDR2: shows bounty, lawmen
warn, you can surrender, pay at post office, bounty cools with time away; Cyberpunk: tiered unit response [W game8].

**Our state.** `Stance { Ignore | Greet | Pursue | Attack }` from thresholds; `ArrestOption { Pay, Jail, Resist }`, Persuade/
Bribe reserved off; no decay; no search state; no warn step; jail loses skills via adapter hook.

**Recommendations.**
- **Escalation ladder (change)**: `Warn` (ask to stop / hand over, a visible short window) -> `Arrest` -> `Pursue` ->
  `Attack`, each a data row per jurisdiction. The warn step is the "fair" beat both KCD2 and Skyrim have [M]; arrests at `Pursue`
  already exist. Resist/flee escalate; fleeing sets **heat** (below), not instantly Attack.
- **Two layers (adopt)**: *bounty* = persistent debt (Counter, per jurisdiction, as today); *heat* = transient pursuit state
  with `Searching(last_known_pos)` -> cool-down, driven by AI8 LKP (ai.md section 2). Guards stop pursuing when heat cools but the debt
  remains. This gives GTA/Cyberpunk escape play without changing the Counter.
- **Bounty decay (add, data)**: per-jurisdiction `decay_per_day`, minimum floor for severe offences (murder never fully
  decays: it stays as `Wanted` record). Classic: none.
- **Arrest options (answers adapter Q7 / spec Q7: add)**: Persuade/Bribe/Surrender/Alternative-punishment as data
  `ArrestOption`s; availability and chance from stats (speech) and offence class. Slice after LW5, not before.
- **Jail (answers Q5, Q6)**: modern default = time + confiscation + reputation hit, **no skill loss**; skill-loss is the
  Classic adapter hook. Co-op: a sentence must not advance the shared world clock for other players; run it as a local
  fast-forward scene and credit days served (revision of spec Q6: "skip time for the jailed player only" is impossible on
  a shared clock).
- **Faction scope**: bounty scope = jurisdiction of the *reporting witness* plus the victim's faction; propagation beyond is
  bounded NPC-speed gossip (ai.md section 5): messengers and guards at gates carry a `Wanted` description. Faction expulsion
  stays a `StatOp` as spec'd. Reputation per town (KCD2) is the same Counter with a different threshold table.

## 3. Stealth

**Model standard (today).** Graded awareness fed by distance, central versus peripheral vision, light, posture and speed, with
noise as a separate stimulus: Thief light gem + noise [M], Dishonored [M], Hitman (suspicion tiers with disguise recognition
by role) [M], Splinter Cell light meter [M], TLOU2 added a "searching" state between unaware and aware (ai.md), AC Shadows
visibility = sliding scale of stance, distance, light level; light sources can be extinguished; white/yellow/red per-enemy
perception meter (react time, investigate, detected) [W pcgamer, gamerant].
Common HUD: a per-observer indicator over the NPC (fill + direction), a player visibility/light gem, body-found alerts; Skyrim's
single eye icon is the classic floor.

**Our state.** `DetectionRules::concealment(DetectIn{ distance, angle, light: Option<f32>, roll })` with two models (Classic
roll, Meter); the Meter is owned by AI8 with `StealthView { exposure: Hidden|Caution|Detected }` on the player.

**Gaps.** (a) `light` is a lone `Option<f32>`: needs a **shared light sample** (our lamp/sun system; same value for player gem
and AI), cached per frame, not per observer (perf). (b) No **noise stimulus** contract: footsteps by surface/posture/speed, picks,
broken locks, thrown objects, combat. Emit `Stimulus::Noise{loudness, pos}` from the action state (ties to nav path-distance in ai.md).
(c) Indicators are per player, not per observer: modern games show per-NPC. (d) No stealth **takedown/non-lethal** rule
(combat.md owns damage) and no **body discovery** stimulus. (e) Disguise (Hitman) is not in `DetectIn`.

**Recommendation.**
- Make **Meter the default** for the framework and the modern preset; Classic roll stays the Morrowind preset (answers Q4 / adapter Q2).
  `concealment` stays the formula seam; add `noise` and `disguise` fields to `DetectIn`.
- Gain rate = f(1 - concealment/100, distance falloff, central/peripheral multiplier, observer alertness) per ai.md; show it
  as an NPC-anchored indicator (white/yellow/red like AC Shadows; direction wedge for off-screen) and a player light gem.
- Light sample: one `LightExposure` value from the lighting system, `sf_crime_bevy` reads; extinguishable lamps already exist in
  the world model, treat them as an interaction (shoot/snuff) with a crime rule only if owned.
- Stealth honesty rule (TLOU2: companions obey stealth) belongs to followers spec.

## 4. Ownership, theft, assault, vandalism

**Standard.** Owner + faction + rank (Bethesda), private/restricted areas with a **trespass warning** before the offence (Skyrim
"leave" warnings, KCD2 private rooms) [M], stolen marker that cools with time/distance (KCD2) [W], fences buy hot goods at a discount [M],
merchants recognise stolen goods in your pack [M], pickpocketing as a read-the-victim minigame (KCD2 hold-to-search timer that
reddens, sleeping/drunk easiest) [W] versus Skyrim's percent roll [M], murder vs assault by victim death/ally.

**Our state.** Strong: `Owner`/`use_allowed`, `StolenLedger` with reconcile and confiscate, `StolenVeto`, `FenceRules`,
`PickpocketRules`, trespass via `UnlockAttempted`, vandalism owner decision, offences as data (assault/murder are rows).

**Gaps and recommendations.**
1. **Area trespass with warning**: zone `Restricted { owner, warn_s, grace }`; entering emits a `Warn` stance first, offence only
   on ignore. Needed for open worlds with private homes, not just owned doors. Add to LW3 data (zone record) and reuse the `Warn` rung.
2. **Hot goods**: `StolenLedger` entry gets `heat_until`/`distance_cleared` so merchants only veto "fresh" goods and fences
   pay by freshness; classic Morrowind keeps permanent flag. This turns the ledger decision (adapter Q1: reconcile) into the
   modern default and answers it: **reconcile, plus cool-down**.
3. **Pickpocket (A4)**: modern version is the KCD2 timer (hold to search, risk meter, victim awareness from the observer
   Meter) instead of one roll; same `PickpocketRules` seam, minigame as client input stream (see 5). Do not build the roll first.
4. **Vandalism**: keep; break owned object = `vandalism` row scaled by object value and noise; classification table stays data.
5. **Fences (adapter Q4)**: answered: framework ships fence offers on; Morrowind content decides which merchants flag (default off
   there, a mod can flip). Cost of building is one flag.

## 5. Locks and traps

**Standard.** Skyrim: rotate lock, sweet-spot pick, skill widens tolerance, pick breaks on jam [M]. KCD2: sweet spot shows gold
when aligned, wrong turn jams and snaps the pick; Stealing skill gives margin; pick snaps are noisy, perks quiet them; door and
lock opening make noise [W mein-mmo]. Fallout/Starfield: similar [M]. Oblivion minigame (tumblers) and Morrowind dice roll are the
older models; modern games also offer **forced entry** (bash with weapon, breakable locks, noise and a vandalism offence) [M], spells and
tools, and **traps visible to perception checks** (passive detection, BG3-style) with disarm skill [M].

**Our state.** `LockRules::pick_chance` + tool wear + `Probe` for traps + Open spell + keys; trap = effect `SourceDef` on the object
fired in `gate_activation`; trespass on every attempt. No forced entry, no noise, no minigame, traps only on activation.

**Recommendations.**
- **Lock interaction behind a `LockInteraction` seam**: `Roll` (Morrowind/Classic, today) or `Minigame` (modern default). The
  minigame only produces the same `LockOutcome`; the *model* is skill-shaped difficulty: skill + tool quality widen the
  sweet-spot window and slow jams; lock tier sets window and break chance; no tier is "impossible" for skill below it, only
  hard, except a data `Impossible` flag (keyed doors). Net: client sends an input stream with a host-derived seed
  (sweet spot from `(object, attempt)` hashed as spec'd); host re-simulates or accepts (2-4 trusted co-op; replay check optional).
  Add an **auto/accessibility** option (chance roll) as a setting, not a separate rule.
- **Noise**: failed pick, snapped pick, forced door, trap spring each emit `Stimulus::Noise`; lock tier and a `quiet` perk change
  loudness. This is the missing link between locks and stealth.
- **Forced entry (add LW3b)**: `LockRequest::Force{ tool|weapon }` damages a lock HP (door/chest `LockState.hp`), loud, and
  raises `vandalism`/trespass if owned; magic Open stays a quiet alternative. Cheap to add with data.
- **Traps (extend, defer)**: add `TrapTrigger { OnActivate | Proximity | Tripwire }` and a **detect** result (perception
  reveals trap, `probe` becomes active disarm); traps affect NPCs and enemies via the same effect call. Slice after LW3, low impact.
- **Tiers**: 5 tiers (Novice..Master) as data labels over `LockState.level`, shown on the prompt with a "can you attempt it?"
  hint, never a raw number (adapter keeps raw levels).

## 6. Crates and open source

crates.io API, 2026-11-25 (name search; no result is a stealth or crime library): nothing for stealth, crime, witnesses or lockpicking
(`lockpick` is a CI tool; `stealth_game`/`bevy_stealth` searches return unrelated crates). AI building blocks only: `big-brain` 0.22.0
(Nov 2024, utility AI, stale for Bevy 0.20), `bevy_behave` 0.5.0 (Feb 2026), `bevior_tree` 0.12.0 (Oct 2026), `seldom_state` 0.16.0
(Apr 2026); licences and Bevy 0.20 support not verified here (AI lane decides in ai.md section 7). `bevy_yarnspinner` 0.10.0 (Nov 2026) is dialogue only,
out of scope. Prior art: OpenMW (GPL-3.0, ported per CONTRIBUTING, `commitCrime/reportCrime/awarenessCheck`, already in the
adapter), Mudcrab (GPL, last audit "nothing to port"). Industry reference only, no code: KCD2, RDR2, Cyberpunk, Thief/Dishonored.
Verdict: build in `sf_crime`; our pure-rule split is the right shape.

## 7. Gaps ranked by player impact and verdicts

| # | Gap vs standard | Impact | Verdict |
|---|---|---|---|
| 1 | Bounty applied at event time; no visible, interruptible report, no recognition/disguise | Very high (the signature feel of RDR2/KCD2) | **Change** LW2/LW9: `ReportState`, `Recognition`; classic = instant |
| 2 | No heat/search phase; stance purely from bounty; no warn step | High | **Add**: heat layer, `Warn` rung, LKP search (needs AI8) |
| 3 | Stealth HUD per player only; no noise; no shared light | High | **Change** LW4/LW8: noise stimulus, `LightExposure`, per-NPC indicators |
| 4 | Lockpicking is a roll only; no forced entry; no noise | High | **Change** LW3: `LockInteraction` seam, minigame default, `Force` |
| 5 | No area trespass warning; no hot-goods freshness | Medium | **Add** to LW3/LW1 data |
| 6 | No bounty decay, gossip bound, faction-propagated wanted | Medium | **Add** decay row + gossip hook (ai.md) |
| 7 | Arrest options limited; no flee/surrender/alt-punishment | Medium | **Add** `ArrestOption` data after LW5 |
| 8 | Jail skill loss, global PassTime in co-op | Medium | **Change**: no skill loss in modern, local scene in co-op |
| 9 | Trap triggers and detection, pickpocket minigame | Low-med | Defer behind LW3/A4 |
| 10 | LW9/LW10, saves, UI slices | n/a | Keep; plan order fine except see 11 |
| 11 | Slice order: AI8 gates LW9 so witness, report, meter are late | Process | Do LW1-LW5 as planned; add **LW2b** (Report/Recognition state types, pure) before AI8 so the state shape is fixed |

Drop: nothing. Keep as-is: ownership, ledger+Pardon, offence data, services veto, trespass-on-attempt, per-player co-op crime.

## 8. Owner questions already answered by the industry (decide ourselves)

| Spec question | Answer (reason) |
|---|---|
| Co-op crime scope per player vs party | **Per player** (host-auth, remote player = another character); hosts may enable "shared heat" later |
| Bounty scope global vs jurisdiction | **Per jurisdiction by default** (Skyrim holds, KCD2 towns); Morrowind preset: one global jurisdiction |
| Stolen ledger reconcile vs classic | **Reconcile + cool-down** (modern); classic map as adapter option |
| Detection classic vs Meter | **Meter default**, classic roll in Morrowind preset |
| Jail skill loss | **None in modern**; classic hook retained |
| Co-op jail | **Local fast-forward scene, no shared clock change** (shared time cannot skip for one) |
| Arrest options | **Pay/Jail/Resist/Flee now; Persuade/Bribe/Surrender data-driven later slice** |
| Fences | **Framework ships fence logic; Morrowind default off** |

## 9. Candidate owner questions (player-facing, recommended answers)

1. **Can the player stop or silence a witness before they report (chase, threaten, kill) in Modern mode?** Recommended: yes, as in RDR2; killing is itself an offence if seen. Classic mode: witnesses report instantly.
2. **Lockpicking: skill-shaped minigame by default with an auto-roll accessibility setting?** Recommended: yes; Morrowind Classic preset uses the dice roll.
3. **Do guards ever refuse to arrest and kill on sight for severe crimes (murder), or is surrender always offered?** Recommended: surrender offered up to a data threshold, kill-on-sight above it for murder only (Morrowind classic: attack at high bounty).
4. **Forced entry (smash chests/doors) legal by default as a loud option?** Recommended: yes, loud, with vandalism offence when owned.
Everything else (heat decay curves, indicator style, unit counts, tiers) decided from the industry standard, not asked.

## Missing

Not verified: KCD2 crime page (Escapist, certificate error), official CDPR 2.0 police numbers, RDR2/Thief/Dishonored/Hitman details
([M]), crate licences and Bevy 0.20 support for `bevior_tree`/`bevy_behave` (ai.md owns it), GDC witness-AI talks, Skyrim fence
mechanics from primary source. No visual or numeric tuning proposed.

## Sources

- KCD2 guide: https://mein-mmo.de/en/kingdom-come-deliverance-2-picking-locks-sneaking-pickpocketing-a-big-guide-for-thieves,1223739/
- KCD2 crime summaries (search): https://www.22wk.com/news/crime-punishment-guide-kingdom-2.html, https://www.3xbz.com/new/crime-punishment-system-kingdom-deliverance-2-explained.html
- RDR2 witness / bounty: https://www.gamesradar.com/red-dead-redemption-2-bounty-and-wanted-level/, https://attackofthefanboy.com/guides/how-to-stop-witnesses-from-reporting-crimes/, https://reddead.fandom.com/wiki/Eyewitness
- Cyberpunk 2.0 police: https://game8.co/games/Cyberpunk-2077/archives/Revamped-Police-System-Wanted-Levels, https://www.psu.com/news/cyberpunk-2077-update-2-0-revamped-police-feature-detailed-and-the-cops-will-show-no-mercy-to-lawbreakers/; mods: https://www.nexusmods.com/cyberpunk2077/mods/21547
- AC Shadows stealth: https://www.pcgamer.com/games/assassins-creed/assassins-creed-shadows-review/, https://gamerant.com/assassins-creed-shadows-enemy-detection-explained/
- crates.io API search, 2026-11-25 (big-brain, bevy_behave, bevior_tree, seldom_state, bevy_yarnspinner)
