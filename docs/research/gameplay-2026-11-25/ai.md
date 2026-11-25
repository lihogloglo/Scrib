# NPC AI for a living open world: research note (2026-11-25)

Scope excludes combat tactics and pathfinding. Crate data: crates.io API, 2026-11-25. Tags: **[P]** primary page seen (URL in Sources); **[M]** memory or press, not
re-verified. GDC Vault talks are paywalled: only abstracts were read.

## 0. Verdict

1. **Keep the stack** (directive stack, utility drives, HTN, operators): Guerrilla/Decima shape plus Bethesda-style
   authored packages. Nothing found in 2025-26 argues for replacing it.
2. **The risk is order of work:** perception (AI8), tier 0 (AI10), schedules (AI11/MA8), smart objects (AI14) make the
   world look alive, but sit behind a long chain, with combat AI15 ahead of smart objects.
3. **Absent from the plan:** follower commands, a bark/chatter system, gossip, weather/time reactions, crowd extras.
   All are cheap on top of facts, concerns and smart objects.
4. **No Bevy 0.20 AI crate should be the core**; two 0.20-ready crates exist for side uses (section 7).
5. **LLM:** keep the validated-menu advisor; add a cosmetic chatter channel and an intent-to-validated-action path.
   Do not plan local LLM inference inside the 8.3 ms GPU budget.

## 1. Decision architecture

| Game / engine | Decision layer | Evidence |
|---|---|---|
| Horizon, Decima | HTN planner, used since Killzone 2 | [P] Guerrilla |
| KCD / KCD2 | Modular behaviour trees; AI LOD with a LOD-unaware scripting interface; "Long Distance Move" node | [P] GDC listing, GamesRadar |
| UE5 | StateTree (Mass-integrated); Smart Objects = reservation, no logic; Mass LOD tick rates | [P] Epic |
| Hitman 2016 | per-NPC sensors plus shared knowledge base (player status, last seen); guards and civilians differ | [P] Game Developer |
| Cyberpunk 2077 | no primary source; 1000+ routine NPCs, reaction "volumes", crowd flee, launch AI criticised | [M] press |
| RDR2 | no primary source; scripted routines (work, saloon, sleep) | [M] press |

- **Fix in `ai.md` industry table:** "GOAP: ... KCD II" is not supported (KCD uses behaviour trees). Dying Light 2 and
  Transformers HTN claims: nothing found, tag unverified.
- The field converges on layers, not one algorithm: authored routine layer, reactive interrupts, cheap logical
  simulation off-screen, smart objects as the world's verbs. We have all four on paper.
- HTN's win is plan monitoring and method libraries as data; its cost is authoring/debug, mitigated by our trace-first
  AI1. Utility (IAUS, AI7) picks "what", HTN decides "how" [M, Dave Mark]. GOAP stays a non-goal.
- Epic's smart-object pattern (state "use object" with children "reach", "use") equals our `UseSlot` fragment.

**Recommendation: keep.** Add one table to `ai.md`: the interrupt contract (which drive pre-empts which, how a
pre-empted plan resumes; `Library::holds` and `Pause` exist) since weather, greeting, follow and chatter are all interrupts.

## 2. Perception

AI8 is unbuilt; spec = cone + range + batched LOS, hearing by `path_distance`, per-player awareness. Sound base.
Missing versus the standard:
1. **Graded awareness.** TLOU2 added a state between "knows where you are" and "unaware" [P GameSpot]; Hitman
   Absolution shows a rising suspicion circle, disguises identified by same-outfit NPCs at close range [P PC Gamer].
   Make `Awareness.level` a 0..1 meter (unaware, curious, suspicious, searching, alerted) with gain = f(distance,
   central/peripheral cone, target visibility, NPC alertness) and decay. OpenMW `awarenessCheck` becomes the adapter's
   gain function (the seam).
2. **Last known position (LKP)** as a fact: `LastSeen{pos, tick, velocity}`; a framework HTN *search* method (go to LKP,
   look, widen over navmesh, give up on ttl). Needed by guards, hostiles, companions.
3. **Shared knowledge, not telepathy.** Hitman's shared knowledge service [P]; alerts spread by events (shout bark,
   alarm, someone running) at finite speed through doors, passing the LKP with reduced confidence. Build as
   `Stimulus::Notify` emitted by a witness, consumed by listeners' perception.
4. **Light/shadow:** read the lighting we already compute (sky/lamp irradiance at the target, one cached sample per
   player per tick) plus stance, speed, armour noise as a framework `VisibilityFactors`, adapter-weighted.
5. **Cost:** spatial-grid prefilter (range, cone, only then rays), 1-2 target sample points, stagger by actor id,
   cache N ticks; tier 1 uses distance + same-space only (specced).
6. **Companions obey stealth rules** (TLOU2 fixed invisible companions [P]).

## 3. Daily life

KCD2: every NPC has a routine and events change it [P Wikipedia, Wccftech]; patch 1.2 still lists "improved daily
schedules" [P Dexerto], so schedules stay a tuning burden. Our plan (AI11 window-edge pushes, anchors, tier 0, smart
objects) is the right approach, wrongly ordered:
- **Reorder:** AI14 (smart-object fragments) before AI15; slice a thin "day loop" first: one worldspace, three anchors,
  tier-0 route fast-forward, bed/chair/bar slots, doors via the out-of-view fade rule. Combat AI is not what makes the
  world look alive.
- **Needs system: do not build.** KCD and RDR2 rely on schedules plus object variety; Sims-style needs are another genre
  [M]. Use 2-3 drive considerations (tired at night, hungry at meal windows) to bias which smart object a free actor picks.
- **Ambient behaviours:** tagged smart objects weighted by time and role (sweep, lean, pray, smoke); paired slots for
  two-NPC activities (these trigger pair chatter, section 5).
- **Weather/time:** clock and weather publish `WorldState` keys (`raining`, `storm`, `night`, `festival`); methods and
  schedules condition on them (seek roofed slots, lamps at dusk, stalls close). Add an `Ambient` concern kind (no
  object, a world-state trigger) to world-reactions; no new machinery.
- **Reacting to the player** (greeting, staring, "put that away", trespass, sneaking, running): the `Notice` drive and
  `ConcernRules` ladder already fit. Add kinds `WeaponDrawn`, `Trespass`, `Sneaking`, `Loitering`; Morrowind's Hello
  radius is one rule row. A few data rows buy much of the Skyrim/KCD2 feel.
- **Crowds:** `crowd-lod.md` is presentation only. Ambient extras (lane pedestrians spawned and despawned with streaming,
  a flee layer as Cyberpunk, already noted in world-reactions) differ from persistent NPCs. Not needed for Morrowind;
  needed for a generic big-city game. Optional `sf_crowd` after the day loop (owner Q1).

## 4. Simulation LOD and tiers

Our 3 tiers, global cap by CPU tier, pins over caps, scripts seeing one actor, promotion resolve and tier invariance
match the evidence: KCD2 has ~2400 NPCs, about half in one city, scripting unaware of LOD, "thousands of NPCs at
60 FPS" [P GDC listing]; UE Mass uses per-LOD tick intervals, spread first update, visible/off-screen settings and
hysteresis [P Epic, X157]. Fixed counts (not time-adaptive) is a good determinism choice. Small additions:
(a) **stagger the first update** after cell load; (b) tier 0 never runs the planner: a state machine over
`(route, progress, activity, until)`; (c) **queue promotion resolve** (nav snap, slot claim) nearest-first with a
per-tick cap; (d) per-tier ms in `ai.stats` for AIM. Numbers unchanged.

## 5. Social systems

- **Gossip** is a non-goal ("rumour kits") but knowledge travelling at NPC speed is the cheap layer that makes reputation
  feel real. **Recommend bounded propagation as a rule:** facts carry `heard_from` and `hops`; NPCs sharing a smart-object
  scene (tavern table, well) or a conversation swap top-N shareable facts; hops <= 3, confidence drops per hop,
  salience eviction (already specced); at tier 0 resolved at schedule edges between co-located actors. Classic keeps
  the instant faction bounty (adapter).
- **Reputation:** per faction and settlement scalar from fact-kind rules; shown by greetings, prices, guards. Link `crime.md`.
- **Nemesis rivalry:** specced (decision 7, patent ruled out by owner); late (AI13); no change.
- **Companions/followers:** only follow-through-doors is specced (MA9). Standard: `Follow{leader, slot, rules}` with
  personal space (stay out of the player's path, doors, ladders), rotating side/rear slots, catch-up teleport only out of
  view, a small command set (follow, wait, hold fire/engage, go to), combat help under attack tokens (combat lane),
  stealth-rule obedience, companion barks. Commands via `sf_interaction` focus or a radial. **Add a followers spec**
  (framework directive, slots, commands; adapter: Morrowind escort/follow).
- **Barks and chatter:** no spec. Standard is rule-based dynamic dialogue (Valve response rules, fuzzy matching [M]): a
  bark = record with conditions on state/facts, priority, per-speaker and per-category cooldowns, hearing/LOS to
  listener, no repeats until all played. Concern `Bark` rung, greetings, witness shouts, paired-slot chatter and companion
  remarks become categories of one `sf_barks`. Host picks, clients play. Write a short spec before AI20.

## 6. LLM NPCs: seam only

- **On-device SLMs ship now:** KRAFTON PUBG Ally on NVIDIA ACE (ASR, 2B SLM, TTS; reflex actions never wait on the
  model) [P]; ACE Game Agent SDK 2026 with a 4B Qwen model, Total War advisor playtest 2026 [P]. Inworld seems to have
  moved to runtime/voice infrastructure [M, single guide].
- **Mantella** (Skyrim, Fallout 4: STT, LLM, TTS): conversation-triggered actions, NPC-to-NPC talk, groups of five,
  0.14 (Apr 2026) adds vanilla-dialogue integration and nearby-NPC awareness [P project; version per secondary article].
- **Failure mode:** Where Winds Meet's chat NPCs were exploited by echoing their last words and forging emote brackets
  to skip quest conditions [P PCGamesN, Outlook]. Confirms our rule: the model picks among host-validated options and
  never writes state.

Keep seams for: (1) validated-menu advisor (specced); (2) **display-only chatter channel**, text tagged `cosmetic`
that can never satisfy a condition, with the authored bark as offline fallback; (3) **intent to action**: player
free text/voice mapped to one currently valid verb, same menu mechanism; (4) context export (persona + top facts +
awareness + activity as JSON, specced); (5) authoring-time generation needs no runtime seam. Not realistic: a local
2-8B model sharing the 4060 laptop GPU within 8.3 ms (VRAM, DLSS/RT contention). Keep out-of-process, optional, off by
default, choices saved, host-run in co-op.

## 7. Crates and open source (crates.io API, 2026-11-25)

| Crate | Ver, date | Bevy | Licence | Verdict |
|---|---|---|---|---|
| big-brain | 0.22.0, Nov 2024, archived | 0.15 | Apache-2.0 | dead |
| bevy_observed_utility | 0.2.0, Mar 2025 | 0.15 | MIT/Apache | stale |
| dogoap (GOAP) | 0.5.0, May 2025 | none listed | MIT | non-goal |
| bevy_bae (HTN) | 0.1.0, Jan 2026 | 0.18 | MIT/Apache | string state; no |
| beet | 0.0.9-rc.1, Feb 2026 | 0.18 | MIT/Apache | pre-release |
| bevy_behave | 0.5.0, Feb 2026 | 0.18 | MIT/Apache | behind 0.20 |
| **bevior_tree** | 0.12.0, Oct 2026 | **0.20** | MIT/Apache | side use, mod logic |
| **moonshine-behavior** | 0.6.0, Oct 2026 | **0.20** | MIT | stack state machine, props |
| **bevy_gearbox** | 0.10.0, Nov 2026 | **0.20.1** | MIT/Apache | statecharts (ability/anim states) |
| bonsai-bt, behave | 0.14.0 Sep 2026; 0.9.2 Mar 2026 | none | MIT; Apache-2.0 | Bevy-free BTs; we have HTN |

Conclusion unchanged: own small Bevy-free `sf_ai` core (design from Fluid HTN MIT, Humphreys, Decima). `bevior_tree`
could be an optional documented route for non-NPC entities, never wired into `sf_ai`. OpenMW (GPL) stays the Morrowind
port reference (`AiSequence`, `awarenessCheck`, `getRandomIdle` at b8285cc9). UE StateTree/Smart Objects are design
references only.

## 8. Gaps by player impact

| # | Gap vs standard | Recommendation |
|---|---|---|
| 1 | AI8 lacks graded awareness, LKP search, shared knowledge, light factor | **Change spec** (section 2); AI8 right after AI7 |
| 2 | Living chain long; smart objects after combat | **Reorder** AI14 before AI15; thin day loop first |
| 3 | No bark/chatter system | **Add** `sf_barks` spec before AI20 |
| 4 | Player/weather/time reactions not enumerated | **Add** concern kinds plus `Ambient` kind |
| 5 | Followers unspecified beyond doors | **Add** followers spec |
| 6 | Gossip a non-goal | **Change** to bounded propagation; Classic instant bounty |
| 7 | Crowd extras absent | **Park** optional `sf_crowd` |
| 8 | Needs system | **Drop**; 2-3 considerations |
| 9 | LLM protocol lacks cosmetic/intent paths | **Add** to protocol only |
| 10 | Industry table errors (KCD GOAP, DL2/Transformers) | **Fix** |
| 11 | No stagger/queued promotion in tier spec | **Add** (section 4) |

Adopt: no crate. Change: 1, 2, 6. Drop: needs subsystem.

### Candidate owner questions (player-facing; recommended answer)

1. Optional ambient crowd extras (generic pedestrians) in big cities? Yes, later, off in Classic.
2. Companion command wheel (follow, wait, hold fire, engage) and companions knocked down rather than killed by
   default (data flag)? Yes to both.
3. Gossip (reputation spreads NPC to NPC) versus instant faction bounty? Gossip in Modern, instant bounty in Classic.
4. On-screen suspicion/alert meter over NPCs? Yes as a Modern option, off in Classic.
5. Free-text/voice NPC talk via an optional plugin? Seam only, off by default, mod-provided, never alters quests or rewards.

## Missing

Cyberpunk and RDR2 internals (no primary talk found); KCD2 talk body and slides; TLOU2 perception, Ghost of Tsushima and
Splinter Cell detection talks (not found, uncited); Dying Light 2 and Transformers HTN (nothing found).

## Sources

- Guerrilla: https://guerrilla-games.com/read/htn-planning-in-decima , https://www.guerrilla-games.com/read/the-ai-of-horizon-zero-dawn
- KCD2 GDC: https://schedule.gdconf.com/session/supporting-thousands-of-npcs-in-kingdom-come-deliverance-kingdom-come-deliverance-ii/915120 , https://gamesradar.com/games/rpg/kingdom-come-deliverance-had-to-add-complex-pathfinding-for-just-2-npcs-that-owned-horses-if-you-want-to-do-this-feature-you-have-to-support-it
- KCD2 routines: https://wccftech.com/kingdom-come-deliverance-2-dev-talks-about-dynamic-npc-routines-and-emergent-storytelling-for-side-quests/ , https://www.dexerto.com/gaming/kingdom-come-deliverance-2-1-2-patch-notes-3161815/
- Cyberpunk: https://pcgamer.com/as-cdpr-resurrected-cyberpunk-2077-it-focused-on-one-word-believability ; RDR2: https://pcgamer.com/red-dead-redemption-2-npcs-lead-interesting-lives
- Hitman: https://gamedeveloper.com/design/the-ai-of-hitman-2016- ; TLOU2: https://www.gamespot.com/articles/the-last-of-us-2s-upgraded-enemy-ai-will-make-for-/1100-6470119/
- UE: https://dev.epicgames.com/documentation/en-us/unreal-engine/overview-of-state-tree-in-unreal-engine , https://dev.epicgames.com/documentation/unreal-engine/smart-objects-in-unreal-engine---overview , https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/MassLOD , https://x157.github.io/UE5/Mass/LOD
- LLM: https://www.nvidia.com/en-eu/geforce/news/nvidia-ace-autonomous-ai-companions-pubg-naraka-bladepoint/ , https://developer.nvidia.com/blog/build-on-device-ai-companions-with-the-nvidia-ace-game-agent-sdk-and-unreal-engine-5-plugins/ , https://github.com/art-from-the-machine/Mantella , https://www.pcgamesn.com/where-winds-meet/reception-ai-chatbots , https://respawn.outlookindia.com/gaming/gaming-news/where-winds-meet-ai-exploit-lets-players-skip-quests
