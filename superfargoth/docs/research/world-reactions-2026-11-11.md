# World reactions: noticing, responding, picking up, resetting (research, 2026-11-11)

Companion to [world-interactivity-2026-11-11.md](world-interactivity-2026-11-11.md) (the "use" half: smart objects, carrying,
destruction, mounts). This is the "react" half: NPCs perceive what changed in the world and what others did with objects.
Feeds [specs/ai.md](../specs/ai.md) ("Perception and stimuli", "Memory and relationships", "Tiers and budget"),
[specs/interaction.md](../specs/interaction.md) ("Events"), [crime.md](../specs/crime.md) and
[bevyrrowind/docs/specs/crime-adapter.md](../../../bevyrrowind/docs/specs/crime-adapter.md).
Web search this pass returned press, forums and wikis, almost no GDC primary sources for the reference games (the games'
internals are rarely public). Anything not backed by a source I read is **(unverified)**; the design leans only on
verified points and on our own constraints. Oblivion's Radiant AI is not used as a model (owner); one cautionary note in 4.2.

## 1. Bevy 0.20 (native)

- Has what the glue needs: **observers and entity events** (a `Destroyed`/`Moved` trigger on the object entity, observers
  reacting immediately), buffered **messages** for batch consumers (the 0.17+ Event/Message split; exact 0.20 names
  unverified this pass, check at AI19), **relationships** (`#[relationship]` pairs: `OccupiedBy`, `HeldBy`, `OwnedBy`,
  `HomeOf` are one-line custom relationships with automatic cleanup on despawn), change detection.
- Has not: perception, stimuli, memory, reaction selection, ownership, persistence of world deltas. Verdict: glue only;
  our events stay plain Bevy-free structs in `sf_ai`/`sf_interaction` and the `_bevy` shells translate (compile-time rule).
- Net rule 8 fits: reactions are events + host-side state, never per-tick replicated state.

## 2. Crates (Rust ecosystem, checked 2026-11-11 via the crates.io API)

| Crate | Version / date | Licence | Verdict |
|---|---|---|---|
| `big-brain` | 0.22.0, 2024-11 | Apache | Utility AI, stale. No. |
| `bevy_observed_utility` | 0.2.0, 2025-03 | MIT/Apache | Stale. No. |
| `cranium` | 0.3.0, 2026-07 | MPL-2.0 | New utility-AI crate (92 downloads), Bevy version unchecked. We already have IAUS drives in the spec. No. |
| `bevy_society` | 0.9.0, 2026-10-31 | MIT/Apache | "Living societies: personalities..." young (108 downloads); not checked in depth. Watch only. |
| `mercs2_ai` | 1.0.4, 2026-11-03 | MIT | A Mercenaries 2 reimplementation (percept to goal planner); reference reading, not a dependency. |
| `saddle-ai-fov` | GitHub only, not on crates.io (not found) | ? | FOV/occlusion/stimulus toolkit; version and licence unchecked. Our sight is `portal_query::line_of_sight` anyway. |
| (search for perception/stimulus on Bevy 0.18+) | none found | - | No maintained perception or reaction crate. Expected. |

Nothing fits. Build in `sf_ai` (Bevy-free), reuse the existing perception, fact store and drives.

## 3. Open-source projects

- **OpenMW** (GPL-3, clean port allowed with a notice): the *content* mechanics exist and are already ported in
  crime-adapter.md: `commitCrime` (candidate witnesses within `fAlarmRadius`, LOS + awareness, followers excluded),
  `reportCrime` (per-witness disposition/fight/alarm; guards pursue), stolen-items ledger. Gaps that matter here: crimes
  are **player-only** (our `OffenceAttempt.offender` is the player; NPC crimes not tracked), and nothing reacts to *objects*
  (no moved furniture, no body discovery beyond the crime path). Corpse handling is the shipped reset reference:
  `fCorpseClearDelay` (deletes the dead actor's object) and `fCorpseRespawnDelay`, **both default 72 in-game hours**
  in the data and read by OpenMW's actor code ([forum](https://forum.openmw.org/viewtopic.php?p=69446),
  [commit](https://git.cijber.net/mirror/openmw/commit/ebaeabc96247421a063c832d1c0f74a942ea3e32)). OpenMW items never move.
- **Veloren** (Rust, GPL-3): ECS NPC "rtsim" with per-NPC memory and simulated off-screen NPCs; closest Rust analogue of our
  tier 0. Not read this pass in depth (unverified: it has no object-change awareness).
- **mercs2_ai**, `bevy_society`: see crates; not production references.
- Not checked: Daggerfall Unity, OpenMW Lua mods for NPC reactions (modders add "react to unlocked door" style scripts; unverified).

## 4. Industry

### 4.1 Reactions (events to perception to response to scale)

- **UE5 AI Perception** (verified, [docs](https://dev.epicgames.com/documentation/en-us/unreal-engine/ai-perception-in-unreal-engine)):
  the canonical stimulus architecture. *Senses* (sight, hearing, damage, touch, team, prediction) listen for **stimuli
  sources**; any actor can be registered as a source for chosen senses (their own example: a pickup made visible to Sight),
  events carry `Stimulus { age, strength, location, tag }`, each sense has a **max age** after which the memory is
  forgotten, per-sense detection by affiliation. Lessons: sources are *registered*, not scanned (cost scales with
  registered sources x listeners in range); stimuli age out; "report noise/damage event" is the push path.
- **Hitman (WoA)** (partly verified, [Red Bull guide](https://redbull.com/gb-en/hitman-3-tips-guide), press): NPCs flag
  *behaviour and carried objects* as suspicious (touching restricted things, hiding where one shouldn't, carrying
  curious objects), trespass has a state shown to the player, discarded clothing is explicitly *not* suspicious, witnesses
  call enforcers who investigate, bodies and distractions draw attention. Lesson: a small set of **object/behaviour
  concerns with per-kind severity** and an explicit "not suspicious" allow-list, plus legibility (the player sees the state).
  Internals unverified.
- **Cyberpunk 2077** (partly verified): crowds are density/context driven, civilians flee under pressure, mods add
  clothing/behaviour reactions; **2.0 police respond to wanted level, and press notes there is still no witness-reporting
  system** where NPCs call the police ([PC Gamer](https://www.pcgamer.com/uk/cyberpunk-2077-hotfix-stops-cops-from-teleporting-behind-you-during-roof-crimes),
  [2.0 notes](https://pcgamer.com/cyberpunk-2077-2-0-patch-breakdown)). Lesson (negative): panic/flee crowd layer is cheap
  and reads well; the missing *investigation of change* is exactly what players call shallow. Crowd internals unverified.
- **Kingdom Come: Deliverance 2** (anecdotal, forums/patch notes only): witnesses can *run to call a guard* and can be
  stopped; unwitnessed crimes can still raise suspicion on a stranger; stolen items are marked; reputation per town/faction
  is separate from bounty; Warhorse patch notes list "improved NPC crime reaction". Lesson: **visible, interruptible
  reporting** (the witness is a real actor with a path) is better play than instant bounty. Source quality low; mark
  details **(unverified)**.
- **Red Dead Redemption 2** (secondary only): witnesses alert nearby lawmen, can be silenced or bribed, recognition and
  memory of the player's deeds are a pillar of the NPC system ([press](https://www.gamepressure.com/newsroom/rdr2-can-still-surprise-npc-remembered-the-theft/z31e8e)).
  No primary talk found. Lesson: **witness as a deadline** (the player can still act) and **recognition by identity**
  (face/outfit) rather than a global flag. The GTA VI "rockstar-files" pages that turned up are leak speculation: not used.
- **Dishonored/Thief** (recall, unverified): "stims" on world objects (a knocked-over item, a body, an open door) and
  *alert levels with decay*; guards investigate the stim position and return to the route. **Hitman/Thief/Dishonored all
  converge on: an object change is an investigable *position + kind*, not a global alarm.**
- **UE5 Mass / StateTree** (recall): crowd entities use tags + processors for cheap LOD-based behaviour; a StateTree
  reacts to events by transition. Matches our tiering (tier 0/1 cheap) and event-driven reaction.
- **Ultima VII** (recall, unverified): the ancestor: NPCs own beds, sleep, go to bed at night, react to theft they see.
- **Oblivion Radiant AI (cautionary, owner rejects as a model)**: emergent goal-driven NPCs were scaled back at release
  (NPCs stealing/killing for goals broke quests); do not give NPCs open-ended goals that can invalidate content (our
  quest-safety validator already covers directives).
- **Scaling**: every shipped system above reacts **at event time to the few nearest witnesses** and keeps a *bounded
  memory*, not continuous world scanning.

### 4.2 NPCs picking up loose items

- **Known data points, mostly unverified**: Skyrim/Fallout NPCs re-equip a weapon they dropped (disarm) and loot
  corpses only through scripted/packaged behaviour; Oblivion/Skyrim container and item **ownership** is data (owner + rank,
  already ported). Morrowind NPCs never take floor items. RimWorld pawns **haul** loose items to stockpiles through a
  work-giver pipeline that scores jobs by priority/distance and *forbidden* flags; Dwarf Fortress dwarves haul and "claim"
  items and **dump** clutter (both recall, mechanism well known, no source read). Lesson: hauling is just *jobs over
  smart-object-like item adverts*, priority and role gated, bounded by claim leases (our `Claims` already do this).
- **Who takes what** (design rule, not a source claim): a loose item is a smart object with a `take` slot. Adverts are
  by **role and need**: an unarmed fighter wants a weapon, a shopkeeper wants their own goods back, a scavenger/thief/
  beggar wants valuables, a guard wants evidence/contraband. The rest ignore it. Ownership decides legitimacy.
- **Theft by NPCs**: nobody ships rich NPC-on-NPC crime; Skyrim's thief NPCs are scripted (unverified). Keep it
  data: taking a *free* item is allowed; taking an *owned* item is an offence the owner/guard may witness, handled by the
  same witness path, but only for actors flagged thief-capable.
- **Clutter cleanup**: Bethesda persists dropped items "forever" in a cell (talk-page hearsay, unverified) and is
  famous for save bloat from persisted references; shipped open worlds despawn ephemeral stuff **out of view** (the GTA/RDR
  rule, recall).

### 4.3 Reset and persistence

- **Skyrim**: cell reset when the player leaves, default `iHoursToRespawnCell` = 10 days, "Never Resets" encounter zones
  for safe cells, respawning containers get new loot ([CK wiki](https://ck.uesp.net/wiki/Cell_Reset)). Oblivion's CS wiki
  documents a 72-hour reset ([CS wiki](https://cs.uesp.net/wiki/Cell_Reset)). Both reset **lazily on re-entry**, not by simulating
  the time away, and "player-touched" cells are exempted from reset in some cases (safe cells).
- **Morrowind/OpenMW**: corpse clear and respawn delays 72 h each (section 3). Merchant restock timing: unverified.
- **Nemesis** (press, [PC Gamer](https://www.pcgamer.com/shadow-of-mordors-nemesis-system-inspired-by-multiplayer-sports)):
  early versions had far more personal relationships and were stripped down; persistent memory is *per-character story
  events*, not a log of everything. Internal data model not public (unverified). Lesson: store **few, salient, aggregated**
  facts per NPC (ours: 32 bounded, decaying).
- **KCD**: unwitnessed killed generic NPCs vanish after a while when the body is hidden (forum, anecdotal); named NPCs
  persist. Lesson: **bodies and junk are tiered by importance**.
- **RimWorld/DF**: filth and corpses are first-class; cleaning is a job; both cap by decay/rot (recall).

## 5. What this means for us (adopted: [specs/world-reactions.md](../specs/world-reactions.md))

**Canonical technique**: UE Perception's registered stimulus sources with typed, aging stimuli, driven by object-change
events; the Hitman/Thief/Dishonored rule that a change is an investigable concern (position, kind, maybe a known
culprit), never a global alarm; KCD2/RDR2 visible, interruptible reporting and identity recognition; Nemesis-style few,
salient, aggregated facts; Skyrim/Oblivion/OpenMW lazy reset on cell re-entry plus owner tidying. No world scanning by
NPCs, no new per-tick state. Pipeline, suspicion, NPC culprits, loose items, tidy, reset table and slices: see the spec.

Reset numbers found: OpenMW `fCorpseClearDelay` and `fCorpseRespawnDelay` 72 game h each; Skyrim `iHoursToRespawnCell`
10 days; Oblivion 72 h. Ephemeral junk 24 h and per-cell caps are our tuning (no source).

## 6. Owner questions and answers (2026-11-11)

1. NPC offences against each other and the player? **Yes**: the final universe is more alive and simulated than
   Morrowind (Classic preset keeps player-only).
2. Unwitnessed vandalism? **The victim raises suspicion**; on whom left to the standard (evidence-scored candidates).
3. How long changes stay (the "window")? **It depends**: per category and place, as data.
4. Punishment for repeated vandalism? **Data, not hard-coded** (tunable gameplay).
5. Retrieve a weapon an NPC picked up? **No special rule**: ordinary inventory (loot an unconscious NPC, etc.).

## 7. Couldn't verify

KCD2 witness details (forum anecdotes only), RDR2 and Cyberpunk crowd internals, Hitman internals, Nemesis data model,
Dishonored/Thief stims, Skyrim "dropped items persist forever", NPC weapon re-pickup in Bethesda games, RimWorld/DF
hauling (recall), Morrowind merchant restock timing, Bevy 0.20 exact event/message names, `bevy_society`/`saddle-ai-fov`
Bevy compatibility and licence, Veloren rtsim object awareness. UESP pages returned 403; the Skyrim 10-day and OpenMW 72-hour
numbers come from CK wiki and OpenMW commit/forum search snippets.
