# GTA 6 as the NPC-reaction and interactivity benchmark (research, 2026-11-25)

Public sources only: launch reviews (embargo lifted 2026-11-18), the Rob Nelson (Rockstar North) interviews, patents.
No datamining. Tags: **[V]** verified by a review, interview transcript or patent (link given); **[C]** claimed (press
secondary reporting, leaks, fan wikis; do not build on it); **[K]** background knowledge of RDR2 from earlier
published material, not re-fetched in this session; **[I]** my inference.

Source limits, stated plainly: Destructoid, Forbes and the WikiGTAVI guide returned 403/429, so those are cited only
through search snippets. The IGN interview was read through a Best Buy mirror of the transcript. Rockstar published
no technical talk; nothing below about *how* it is built is verified except the patents, whose link to the shipped game
is itself an inference.

## 0. Summary

GTA 6 is RDR2's reaction model with more of everything, not a new paradigm. What is verified: per-NPC temperament and
aggression traits, reactions that depend on what the NPC saw, a witness-required police model with *descriptive*
identification (outfit, vehicle), body-part hit reactions, a simulated off-screen life, and a dedicated writing studio
for pedestrian lines. What the owner will feel as "alive" is the density of small authored reactions plus consistency,
not hidden AI depth. Our plans cover most of the structure; the gaps are mostly *social* (confrontation ladder,
temperament, paired chatter, incidents) rather than systemic.

## 1. Reactions

**What triggers them.** [V] Walking with a shotgun in hands unsettles people; a gun in hands draws more attention than
one slung ([TweakTown quoting Edge](https://www.tweaktown.com/news/114388/gta-6-npcs-get-personality-traits-that-decide-whether-they-flee-or-fight-back/index.html)).
[V] Standing too close to a group makes them turn and say "can I help you?"; following someone too closely draws "oh,
hey", then aggression or leaving; holstered, hold L2 to greet or antagonise, R2 draws
([Nelson, IGN transcript](https://www.bestbuy.com/gamers/gamers-articles/pcmcat1791490206931)). [V] A taunted
convenience-store robber drew and fired; fleeing through a backyard angered the residents; a man hit on Lucia
([Kotaku review](https://kotaku.com/grand-theft-auto-6-the-kotaku-review-2000757842)). [V] Shot placement changes the
reaction: legs stumble, arms/torso recoil, force by weapon; shot drivers lose control
([Gfinity](https://www.gfinityesports.com/article/gta-6-review-worth-every-second-of-the-13-year-wait)). [V]
Weather affects how NPCs behave (Rockstar's Weather Channel partnership, via
[Dot Esports search result](https://dotesports.com/gta/news/gta-6-new-dialogue-system-npc-conversations), thin).
Not found verified: nudity/odd behaviour, filming, bumping reactions. [C] only (fan wiki): rain shelter, umbrellas;
phone-filming. Drop.

**Personality.** [V] Every pedestrian has traits (named: temperament, aggression); reaction depends on "temperament,
aggression, where they are, what they have seen, and how the player has behaved towards them"; Nelson's design
questions: who is aggressive, how many go for a weapon, how many "test me" (Edge via TweakTown, above).

**Escalation.** [V] Observed ladder: notice -> verbal ("can I help you?") -> aggressive or leave; carjack victim panic
or armed fight-back; a robbed pedestrian may attack when your back turns (Edge via TweakTown). Crowds scatter,
some challenge. After a brawl bystanders get up and walk away; incapacitated NPCs are marked and do not get up
(Nelson, IGN). [I] This is a short per-NPC ladder (3-4 rungs) gated by the temperament, not a deep planner.

**Memory.** [V] Gangs "may remember the player's previous actions and seek revenge" (Gfinity). [C] Individual NPCs
recognising you across days: Rockstar has not said so ([GTASixGuide](https://gtasixguide.com/news/npc-memory-system/),
[search summary]). Treat as faction/gang-level memory only.

**Witnesses and police.** [V] A crime must be witnessed (or an alarm trip) before it is reported; dispatch is "very
specific" about what it knows; police know your vehicle and outfit; a new hairstyle does not hide the outfit, a
bought and worn new outfit removes heat; after a car theft police go to the theft site but do not know who you are;
changing vehicle or splitting from your partner lowers perception; staying near the response keeps heat local
(Nelson, IGN). [V] Radio sharing of your description, K-9, air units, spike strips, wounded officers falling back
(Kotaku, Gfinity). So reporting is descriptive and *partial*: the report carries attributes, not an identity.

**Crowd/group.** [V] 100+ distinct NPCs in one tram station, each moving (Kotaku). [V] Panic spreading from a
drive-by through a neighbourhood (Gfinity).

## 2. Object interactions

Verified detail is thin; reviewers praise rather than enumerate. [V] Player: pressure washer with suds, hair gets
wet, shop robberies, car theft minigames, trunk retrieval, kicking open a shop door (dust moves) (Kotaku;
[Sportskeeda trailer note](https://www.sportskeeda.com/gta/gta-6-s-attention-detail-unmatched-trailer-left-fans-mesmerized),
thin). [V] Nelson: animations are retargeted on the fly so differently sized bodies can interact (IGN transcript).
NPC chains (sit, drink, put back) are not described in any verified source; the RDR2 base [K] is scenario points with
conditional entry/exit animations per prop and an NPC picking a scenario by time and role. [I] GTA 6 uses the same
scenario idea plus body-size retargeting.

**Documented flaw worth copying the fix for.** [V] Context-sensitive prompts misfire: the reviewer wanted to loot a
body and Jason tried to enter a nearby car; reload sometimes kicked someone; trunk prompts flicker with gaze (Kotaku).
Lesson: verb arbitration needs stability (priority + hysteresis), our IN31 focus work should state it.

**Interruption.** No verified source. [I] Reactions pre-empt scenarios (a shot interrupts a drink); out of our hands
to confirm, our own interrupt table (AI0c) already defines it.

## 3. Ambient life and dialogue

[V] Locations change with time: clubs quiet by day, busy at dusk; beaches packed by day (Gfinity). [V] Each playable
character follows a daily routine; the off-screen protagonist keeps living (texts, social life) (Gfinity, Kotaku).
[V] Staged world events without the player: a crash with police statements and paramedics loading an ambulance, a gang
drive-by, bank hold-ups heard in the distance, a lightning-started fire, a polar bear escape that spawned news
(Gfinity, Kotaku). [V] Counter-example: every store is open 24 h and a 3 a.m. barbershop is empty but for staff
(Kotaku): schedules are the weak spot even there.
[V] Dialogue: a studio in Los Angeles built from scratch only for NPC lines; "a terrifying amount" of small-talk
dialogue, added continuously (Nelson, IGN). [C] Lines tagged by weather, time, crime seen, recognition (leak, via
[Dot Esports](https://dotesports.com/gta/news/gta-6-new-dialogue-system-npc-conversations)); [V-ish] Extended-look
coverage: ambient lines depend on weather, time and emotional state. Plausible and matches our DQ-23 rule DB.
NPC-to-NPC: not verified beyond RDR2 [K] pair conversations at scenario points.

## 4. Scale and tech

- **Animation count.** [C] "about 600,000 NPC animations" (Edge via TweakTown); one source attributes the figure to
  voice lines instead ([WikiGTAVI via search](https://www.wikigtavi.com/guides/gta-6-npc-ai-living-world)). GTA 5 at
  ~55,000 is also [C]. [I] Any such count is almost certainly clips x variants (body type, context, mood), not 600k
  hand-keyed clips; it does not tell us how many *families* exist.
- **Authoring.** [V] Patent US 11,620,781, "System and Method for Virtual Character Locomotion" (Take-Two; inventor
  Kleanthous, ex-Rockstar AI lead): data-driven animation building blocks taking inputs like direction and desired
  velocity, so game code need not know why a block is used
  ([Justia inventor page](https://patents.justia.com/inventor/tobias-kleanthous);
  [PSU](https://www.psu.com/news/take-two-patents-locomotion-tech-for-gta-6-promises-highly-dynamic-and-realistic-animations/)).
  Link to GTA 6 is [C] (Reddit/LinkedIn). Other Take-Two patents on NPC personality and vehicle navigation
  ([Kotaku](https://kotaku.com/gta-6-patent-grand-theft-auto-1850972511), [GameSpot](https://www.gamespot.com/articles/rockstar-npc-tech-patent-has-people-talking-about-gta-6/1100-6486479/))
  are [C] as to use, and I could not read their claims.
  [I] Likely stack: layered, data-driven locomotion plus authored reaction/scenario clips with on-the-fly
  retargeting to body size; motion matching is **not** confirmed by any source.
- **Behaviour LOD / density.** No verified detail. [V] 100+ unique NPCs in view; frame drops in busy streets and big
  firefights (Kotaku). [I] Distant NPCs surely run reduced simulation, like ours (AI9/AI10 tiers).
- **Team scale.** [V] NPC team "doubled in size" since RDR2, with an LA studio (Nelson, via
  [This Week in Video Games](https://thisweekinvideogames.com/news/gta-vi-is-still-very-much-grand-theft-auto-rockstar-co-studio-head-tells-press/)).
  The lesson for a small team: breadth is content volume, so tooling that makes a new reaction a data row matters
  more than any single AI technique.

## 5. Gap table vs our plans

Our IDs from `docs/roadmaps/{ai,interaction,crime,dialogue-quests}.md`. "Missing" proposes a new slice (GX = gta6
research, to be renumbered by the owner/orchestrator).

| GTA6 behaviour | Our slices | Missing (proposed) |
|---|---|---|
| Weapon drawn unsettles; held more than slung | AI28 `WeaponDrawn`, AI20 rungs | **GX1** weight `WeaponDrawn` by carry state (hand, slung, sheathed, hidden) and distance, data row |
| Per-NPC temperament/aggression decides flee/fight/test | AI0g `PerceptionProfile` (perception only), AI37 morale | **GX2** `Temperament` per individual (flee, freeze, confront, fight, test) as data + role defaults, feeding AI20 response choice and AI37 |
| Standing close or tailing -> "can I help you?" -> aggressive/leave | AI20 Glance/Bark, AI28 `Loitering` | **GX3** proximity/tailing concern: personal-space and follow-too-long kinds, turn-and-speak rung, then leave or escalate |
| Witnessed-only reporting; partial description reports | LW2/LW2b, LW9, AI22, LW16 `Disguise`/descriptions, AI23 | verify LW16 carries *attributes* (outfit, mount/vehicle, companion) not identity; note in LW16, no new slice |
| Change outfit/vehicle or split from partner drops heat | LW16, AI29 | covered if LW16 attributes include mount and companion link; note only |
| Phone/alarm report channel | AI8e `Notify`, AI22 walking reporter | **GX4** report channel as data: runner, shout, bell/horn, magic/script call; Classic = runner only (no phones in Morrowind) |
| Robbed victim attacks when you turn away | LW10, AI25, AI37 | covered once GX2 exists; add a "grudge on turn-away" row in ConsequenceRules, no slice |
| Gangs remember and take revenge | AI13 vendettas, AI25, AI44, MA13 | none |
| Body-part hit reactions (stumble, recoil by weapon) | combat lane (not read for this note) | check combat roadmap; if absent, one slice there, not mine |
| Incapacitated NPCs stay down; brawl bystanders get up and walk off | AI41 downed (companions only) | note: generic non-lethal knock-down and recovery state for any NPC (combat lane) |
| Scenario chains: sit, pick up, drink, put back | AI14 `UseSlot` + fragments, IN8, IN9, AI31, IN2 | none structurally; verify the chain-with-interrupt contract (what happens to a held cup) is in AI0c/IN9 text |
| Interactions retargeted to differing body sizes | IN2 seat clips, IN3 hand IK, IN23 seat IK | **GX5** body-size contact retargeting rule: slot gives contact points (seat height, grip, door handle), anim graph IK-fits them; set before clips are generated |
| Prompt misfires (loot vs enter car) | IN31 focus verbs, LW8 | **GX6** verb arbitration stability: priority per verb class + hysteresis + sticky focus; exact tests |
| Staged world events (crash + paramedics, drive-by, hold-up) | none (AI27 free-time only) | **GX7** ambient incident director: small multi-role scenes (brawl, arrest, bandit ambush, funeral procession, wounded traveller) spawned in range with caps; roles are slots, outcome tier-invariant |
| Chatter by weather, time, mood, crime seen | DQ-23 rule DB, DQ-25 barks, AI30 `BarkCue` | none for single-speaker; see GX8 for paired |
| NPC-to-NPC small talk | AI42 (companions only), AI14 paired slots | **GX8** ambient paired conversation: two NPCs claim facing slots, alternate lines from the rule DB, may reference things they saw |
| Hold to greet / antagonise without a dialogue menu | DQ-14 dialogue, DQ-25 | **GX9** player social verbs: greet, antagonise (and holstered-only gate) as interaction verbs that raise a rule-DB concept; response by temperament |
| Time-of-day venue life | AI11, AI27, MA8 | none; GTA 6 fails at it too (24 h stores), make shop closing/sleep a MA8 validator row |
| Radio-shared description among police | AI8e `Notify`, AI29 | covered (guards share `Notify` with attributes once LW16 lands) |
| Off-screen life of the other protagonist | AI10 tier 0, AI46 | none |
| Weather changes NPC behaviour | AI27 shelter/dusk methods, MA8 storm | none |

New slices proposed: **9** (GX1-GX9). Cheapest and most valuable first: GX2 (temperament), GX3, GX9, GX7.

## 6. Animation list for Kimodo-generated clips

Ordering: what a player sees most often and what sells "this world is used". All as pairs of sides unless noted.
Every family is generated against the contact-point rule of GX5 (one canonical body, IK fits others); variants listed
are the ones that change the *motion*, not the retarget.

1. **Sit/stand family** (IN2, AI14): approach-and-turn-in, sit down, sit idle loops (2-3 postures), stand up; stool,
   chair, bench, floor/cushion; entry from front and both sides; seat heights low/standard/high; arms variants
   (hands on knees, on table). Plus **pull chair out L/R**, **push chair in**, bench scoot.
2. **Seated hand-prop chain** (AI31, IN9): reach to table, pick up cup/bowl/book, drink/eat/read, put down, wipe
   mouth; right-handed first, mirrored for left; table heights 0.7-0.8 m; props: cup, bowl, bottle, spoon, book.
   Each as its own clip so chains recombine (the RDR2 fragment approach).
3. **Doors** (IN33): push and pull, hinge left and right, from both sides; open-and-walk-through, hold-open for
   another, knock, locked rattle, kick/bash (IN37), peek. Heights fixed; no retarget variants needed.
4. **Stand-and-use**: lean on wall/rail (both sides), warm hands at fire, work at bench/stove/anvil loops, sweep,
   draw water, stir pot (IN42 workstations).
5. **Carry and place** (IN35, AI31): pick up from floor/waist/shelf height, carry (one-hand, two-hand, shoulder),
   place on floor/table/shelf, hand-over give/take pair, throw (low priority).
6. **Social-space reactions** (GX3, GX9): turn-to-face (both directions, 45/90/180), "who's there" glance, step back,
   hands-up placate, shrug, point, wave greet, beckon, dismissive wave, shove away, sit-down-in-shock.
7. **Fear/hit reactions** (GX2, combat): flinch, duck-cover, crouch-flee start, trip-stumble, recoil by direction
   (4), knocked down and get-up (front/back), cower loop, hands-over-head.
8. **Bed and rest** (IN3): get in from both sides, sleep loop, wake, sit on bed edge; lean spots.
9. **Ambient idles for GX7/GX8**: talk gestures (3 intensities), argue, laugh, comfort, drink-toast, mourn, kneel
   to treat wounded, carry stretcher (low priority).
10. **Mount/ladder** stay with travel/ladder lanes (IN43, IN10).

Variants that matter: left/right side, entry direction, seat/handle height class, which hand, two intensity levels
for reactions (mild/strong), and a loop-vs-one-shot split with matching blend frames. Variants that do not: body
size (retarget), cosmetic hand prop (swap mesh).

## 7. Owner questions (only those that change player options)

1. **Greet/antagonise verbs (GX9).** Should holstered-weapon players be able to greet or provoke any NPC with a
   hold-button, outside the dialogue menu, as RDR2 and GTA 6 do? *Recommended: yes*, Classic preset keeps dialogue
   only; provocation gated by the same crime offence table.
2. **Civilians fighting back (GX2).** Should ordinary NPCs sometimes attack or retaliate when threatened or robbed,
   rather than only flee or report? *Recommended: yes*, by temperament and role (guards and fighters always,
   merchants rarely, scholars never), Classic off.

Everything else in this note is decided by industry standard and written as a slice proposal.
