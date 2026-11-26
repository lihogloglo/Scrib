# Cross-lane order (all lane roadmaps)

One order over the eight lane files in this folder: anim, combat, ai, nav, interaction, crime, dialogue-quests, travel.
Slice IDs are each lane's own; this file only sequences them, says which lanes may build at the same time, and names the
cross-lane chains. Lane files stay the source of each slice's deliverable, crates and checks. Revised when a wave ends.

## 1. How the order is decided

1. **Finish what is started** (CONTRIBUTING rule 6): stages already in flight end before new stages open.
2. **Unblock the most play**: among ready slices, the one whose chain opens the most player-facing play across lanes
   goes first (section 3). No demo milestones (owner 2026-11-25): waves are dependency groups, not showcases.
3. **At most three build lanes at once** (owner cap: 3 build agents, plus 2 research-only). Docs and research slices run
   any time in the docs track and never take a build slot.
4. **No two build lanes in one crate** (section 4). A slice that writes another lane's crate runs in that lane's slot,
   queued by that lane's orchestrator, or waits.
5. **Owner checks batch**: each wave ends with its owner checks (visual-check IDs in the lane files), launched one scene
   at a time by the session talking to the owner.

## 2. Where the lanes stand (2026-11-26)

| Lane | State | In flight |
|---|---|---|
| AI (hub) | Stage 1 mostly done (perception, hearing, light, reactions; MA2/MA4 splits); Stage 2 allocator AI9a-f done | AI8c3, then MA21, MA2c3/MA2d3/MA2d4, MA4c2/MA4e, MA20, AI9g (gameplay-roadmaps-7) |
| Anim | Stage 1 half done (AN1, AN1b, AN4a/b, AN5, B1-B2b, AN8/AN8b); roll CB-6 confirmed | AN1c, AN1d, AN2... (anim-2) |
| Interaction | Stage 0 done; IN0a/b, IN31a done; IN20 + IN11a-c2 done (travel Stage 3 base) | IN31a-fix, IN31b, IN32, IN33 (interaction-2) |
| Dialogue-quests | Stage 1 largely done (DQ-2..DQ-5f5) | none: DQ-5b4, DQ-5c2, DQ-5f2/3/6, DQ-6..DQ-11 wait |
| Crime | Stage 2 sneak core done (LW4a/b1/b2) | none: Stage 0 specs LW0a-e first |
| Nav | NV0-NV8, NV11a-d1 built; NV-1, NV-2 confirmed | none (NV11 consumed by AI8c3, MA21) |
| Combat | C1-C2b, C4 soft aim built; CB-1/2/3/3a/6 confirmed; answers in | none: Stage 0 specs first |
| Travel | Stage 0 research partly done; TV-R2 parked (mounts) | none |

## 3. Cross-lane chains (the spines)

| Chain | Slices, in order | Opens |
|---|---|---|
| A. Day in town | MA21 → NV12a → NV12c → AI10 → AI12 → AI11; AN13 → MG8 → AN26 (sit clips) → IN2 → IN3 → AI14 (= IN8) → AI27 → MA7, MA8 → AIM | AI Stage 2: Balmora runs dawn to night, NPCs sit, sleep, light lamps, walk routes out of view |
| B. Use the world | IN31b → IN32 → IN33 → DQ-6, DQ-7, LW6 | activation verbs, doors, locks; scripts on activation |
| C. Fight anyone | C0a → AN32 → AN33 → AN34 → anim C4 → C2b2, C2c → C17 → MA15, DQ-12, C16 | sword fight on Morrowind clips; every NPC can be hit and killed; death feeds quests and crime |
| D. Break, throw, hear | C2c + C3a → IN1 → IN4 → IN18, IN14 → AI19, AI28 | props break and make noise; NPCs notice (AI Stage 3) |
| E. Seen and wanted | LW0a → LW2 → LW2b → LW5, LW9 → AI22 → AI29; DQ-22 → DQ-23 → DQ-25 → AI30 | crimes reported, pursuit, barks (AI Stage 3, crime Stage 3) |
| F. Enemies that fight | C0b, C-R1 → C6a-c → C6e (duelist) → AI15 → AI36, AI37, MA10 → TV9 | tokens, circling, skill (AI Stage 6); road ambushes |
| G. Over walls and through the air | NV0c → NV9a → AI33 (+ AL-10, AL-12) → IN43; NV13a-c → AI34, AI35 → MA12 | AI Stage 5 traversal, fliers, swimmers |
| H. Rides and roads | IN11 rest → IN7 → IN24 → TV4 → TV6, TV8 → TV13; NV12b/d → AI46, TV19 → TV21 | real strider and boat rides, long journeys |

Two findings from the chains, now in the lane files: **NV12a/NV12c are pulled forward** (AI10 needs them, nav.md Stage 4
note), and **C17 gives NPCs health and death without AI**, so MA15, DQ-12 and C16 no longer wait for MA10 (combat.md
Stage 1; ai.md MA15's dependency line is the AI hub's edit). Chain A's longest leg is the sit clips (AN13 → MG8 → AN26):
IN2's headless logic can build as soon as IN31 lands, with owner check IN-2 waiting for AN26 (interaction lane's call).

## 4. Crates: who touches what, and which lanes can build together

**Owners** (the lane whose orchestrator queues every slice in the crate):

| Lane | Owns | Writes in others' crates |
|---|---|---|
| AI | `sf_ai`, `sf_ai_bevy`, `mw_ai`, **`mw_walkers_bevy`** | `mw_rpg(_bevy)`, `mw_script` (MA2c, MA4b, MA4e); `sf_interaction` (AI14, AI31); `sf_nav` (AI8c3) |
| Anim | `sf_anim`, `sf_anim_graph`, `sf_anim_bevy`, `mw_graphs`, `mw_viewers_bevy`, scrib-motiongen, `mw_bake` anim files | `sf_controller` (B2b), `mw_walkers_bevy` (AN8b, AN21, AN22, AN41), `mw_combat` (AN33-AN37) |
| Combat | `sf_combat`, `sf_combat_bevy`, `mw_combat` (after anim Stage 5), `mw_player_bevy`, `mw_rules` | `sf_action` (C3c, C11), `mw_walkers_bevy` (C2c, C17), `sf_interaction` (C10), `sf_camera_bevy` (C4c) |
| Nav | `sf_nav`, `sf_nav_recast`, `sf_nav_bake`, `sf_nav_bevy`, `mw_bake` nav files | `sf_controller` avoid (NV7b), `mw_walkers_bevy` (NV5a) |
| Interaction | `sf_interaction(_bevy)`, `mw_interaction`, **`sf_controller`** (IN11 owner) | `sf_action` (IN2, IN25), `sf_combat` (IN36), `sf_anim_graph` (IN23, IN26), `sf_nav` (IN22), `sf_physics(_bevy)` |
| Crime | `sf_crime`, `sf_crime_bevy`, `mw_crime` | `sf_ai_bevy` (LW4b1 done, LW9), UI crates (LW8, LW11, LW17) |
| Dialogue | `mw_script`, `mw_rpg`, `mw_rpg_bevy`, `mw_scripts_bevy`, `sf_rpg`, `sf_rpg_ui`, new `sf_dialogue_bevy` | `mw_saves_bevy`, `mw_actors`, `sf_ui_screens` |
| Travel | `sf_travel(_bevy)`, `sf_map` | `mw_rpg(_bevy)` (TV5, A4, TV13), `sf_ai` (TV17, TV21), `sf_nav_bevy` (TV25) |

**Hot crates** (three or more lanes write them): `mw_walkers_bevy` (AI, anim, combat, nav), `sf_controller` (interaction,
anim, nav, travel), `mw_rpg(_bevy)` (dialogue, AI, travel), `sf_action` (combat, interaction), `mw_combat` (combat, anim).
Rule: a hot crate has one writer at a time; the owner's orchestrator queues the guests. `mw_bake` splits by file (anim,
nav, travel, map bakes) and may take two writers on different files.

**Pairs that share no crate** (safe to build together): combat ‖ nav, combat ‖ dialogue, combat ‖ travel, crime ‖ nav,
crime ‖ travel, crime ‖ anim, dialogue ‖ anim, dialogue ‖ interaction, dialogue ‖ nav. Conditional: combat core
(`sf_combat` only) ‖ AI; nav core (`sf_nav*` without `mw_walkers_bevy`) ‖ AI once AI8c3 is in; travel Stages 1-2 ‖ all
but dialogue. Always sharing: AI with anim/combat/nav game sides (`mw_walkers_bevy`), interaction with travel Stages 3/6.

## 5. The order (waves)

A wave ends when its stages' slices are done and their owner checks are queued. Lanes are listed in slot priority: the
first three take the build slots, the next one moves up when a slot frees.

### Wave 0 (in flight, finish first)

AI Stage 1 tail (AI8c3, MA21, MA2c3, MA2d3/MA2d4, MA4c2, MA4e) and MA20, AI9g · anim Stage 1 · interaction Stage 1
(IN31a-fix, IN31b, IN32, IN33). Docs track: combat C0a-C0e, nav NV0b-NV0d, crime LW0a-LW0e, research C-R1, C-R2, NV-R1.

### Wave 1: a day in town, a world you can use

1. **AI + nav (game side)**: NV12a, NV12c → AI10 → AI12 → AI11 → MA7 (nav slices queued by the AI hub: `mw_walkers_bevy`).
2. **Interaction**: IN2 (headless), IN3, then AI14 = IN8 with the AI hub (`sf_ai` + `sf_interaction`), AI27 → MA8, AIM.
3. **Anim**: Stage 2 review tool (AG2, AN10, AN46, AN11, MG9) and the sit chain AN13 → MG8 → AN26, AN30 (beds), AN28 (doors).
4. Next up when a slot frees: **dialogue** DQ-5b4, DQ-5c2, DQ-5f2/3/6, DQ-6 (needs IN31), DQ-8, DQ-9, DQ-10, DQ-11, then
   DQ-7 after crime A0/A5 (disjoint from wave 1 except `mw_rpg_bevy`, free once MA2c3/MA4e land).

Owner checks: AI Stage 2 (Balmora day), IN-2, IN-3, ANIM-9, anim Stage 1 checks.

### Wave 2: the fight on Morrowind clips, locks and sneaking

1. **Combat** (runs anim Stage 5 in its slot): AN32, AN33, AN34, AN35, AN36, AN37, anim C4 → C2b2, C2c, C17, C3a, C3b, C3c,
   C3d, C4c.
2. **Crime**: Stage 1 (LW1, LW3, A0, A1, A5, LW6, LW7, LW8, LW11) → Stage 2 rest (MA5 with the AI hub, LW12, LW13).
3. **Dialogue** (if not finished in wave 1): Stage 1 end; then Stage 3 quest log (DQ-16..DQ-21, `sf_rpg*` only).
4. Next: **nav core** NV14, NV5a, NV7b (`sf_controller` avoid, after anim's B-slices), NVM.

Owner checks: CB-1 re-check, CB-3b, CB-3c, CB-4, CB-4b, CB-5c, CB-10, ANIM-12, ANIM-13; crime LW-1..LW-3.

### Wave 3: the world notices

1. **AI** Stage 3 (AI19, AI20, AI28, AI21, AI22, AI29, AI23, AI30, MWR1, AI49, AI50, AIM2).
2. **Crime + dialogue**: crime Stage 3 (LW2, LW2b, LW5, A2, A3, LW9 in the AI hub's queue, LW14-LW17); dialogue Stage 2
   (DQ-12 on C15/C17, DQ-13 after effects A5b, DQ-14, DQ-15) and Stage 4 barks (DQ-22..DQ-25, DQ-39, DQ-31).
3. **Interaction** Stage 2 (IN35, IN6, IN1, IN38, IN4, IN18, IN37, IN14, IN15, IN36) with **combat** Stage 2 in the same
   slot sequence where they touch (C2d, C2e, C9, C10 after IN28 data, C11, C15, C16).
4. Next: **travel** Stages 1-2 (TV1, TV5, A4, TV10, TV26, TV14, TV15, MAP0-MAP5, TV16): disjoint but for `mw_rpg`.

Owner checks: AI-5, LW-4, CB-8, CB-8b, CB-9, CB-9b, CB-5d, IN-1, IN-4, IN-36, DJ-4, DJ-7, MAP-1..MAP-4.

### Wave 4: enemies that fight, NPCs that climb and fly

1. **Combat** Stages 3-4 (C12, C13, S1, S2, S5, C6a-C6f) → **AI** Stage 6 (AI15, AI36, AI37, MA10) in the AI slot.
2. **Nav** Stages 2-3 (NV9a-NV9e, NV13a-NV13d) → **AI** Stage 5 (AI33 with anim AL-10/AL-12, AI34, AI35, MA12); IN43, IN34.
3. **AI + interaction** Stage 4 (AI31, AI32, AI24, MA11; IN9, IN16, IN42) after MG7b and AN29 (carry clips).
4. Next: **interaction** Stage 6 rest (IN11c2g, IN11c2c3, IN11c2d, IN11c2e, IN11c3, IN21, IN7, IN23, IN24, IN22).

Owner checks: CB-7, CB-11, AI-6, AI-7, AI-8, NV-3, IN-43, ANIM-10.

### Wave 5: bows and spells, rides, roads, followers

1. **Combat** Stage 5 (C8a-C8e, with effects A5b) and Stage 6 (C14).
2. **Travel** Stages 4-5 (TV2, TV3, TV4, TV6, TV8, TV12, TV13, TV7, TV9, TV11) on interaction Stage 6.
3. **AI** Stage 7 followers (AI38-AI42, MA9) and Stage 9 (AI45; AI46 on NV12b/NV12d).
4. Next: **nav** Stage 5 (NV10a-NV10c, NV15), **interaction** Stage 3 light and fire (IN19, IN17, IN39-IN41, IN44).

Owner checks: CB-12, CB-13, TV-2, TV-3, TV-5, AI-9, AI-11, NV-4, NV-5, IN-19, IN-39, IN-40.

### Wave 6: memory, a thief's trade, styled people, staged talk

AI Stage 8 (AI13, AI43, AI44, AI25, MA13) · crime Stage 4 (LW18, A4, LW19-LW22, LW10) · anim Stage 3 (AN12-AN24) and
Stage 6 (AN38-AN42) · dialogue Stage 5 (DQ-26..DQ-32). Owner checks: AI-10, ANIM-8, ANIM-14, DJ-8, DJ-9.

### Later (not scheduled)

Classic presets (crime Stage 5 A8, combat C5); co-op (combat C7, travel Stage 7, character-shapes S7, net N1-N8);
mounts (travel Stage 6, waits for TV-R2, owner-parked); dialogue Stage 6 authoring; each lane's parked list.

## 6. What each wave unlocks for the player (by lane stage)

| Wave | Player-facing |
|---|---|
| 1 | NPCs live a day (routes, doors, sitting, beds, lamps); everything activatable answers; scripts react |
| 2 | Morrowind-clip sword fight with poise, heavy and charged attacks; NPCs can be killed; locks, traps, owned things, sneak |
| 3 | Crimes are seen, reported, pursued; barks; props break and make noise; parry, riposte, kick into hazards; maps, instant travel |
| 4 | Enemies take turns and read you; creatures with their own moves; NPCs over walls, fliers and fish on real nav; errands |
| 5 | Bows and spells; real strider and boat rides with risk on the road; followers; travellers on roads; a world that changes |
| 6 | Gossip and reputation; pickpocketing and trespass; style variety; physical stumbles; staged conversations |

## 7. Owner questions

None: the order follows from dependencies and the CONTRIBUTING rules. The owner may swap any two waves whose lanes share
no crate (section 4) without breaking a chain.
