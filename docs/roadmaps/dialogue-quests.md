# Dialogue and quests: lane roadmap

Planned 2026-11-25 from `docs/research/gameplay-2026-11-25/dialogue-quests.md` ("note §n"), DECISIONS 2026-11-25,
the wiki and the four specs. No ledger yet. New slices: **DQ**; visual checks: **DJ-** (dialogue/journal), **MS-** (MWScript).

## 1. Player-facing goal

**Modern Morrowind (default):** every vanilla quest that its own scripts and dialogue can drive works: talk to an NPC,
get orders, the world reacts (doors unlock, items appear, NPCs follow or attack), the journal records it.
Conversations are staged scenes: the camera frames the speaker, who looks at you and gestures, lines pace to reading
time, topics are the choices; combat or a crime breaks it off with a bark. The journal keeps the book and adds a quest
view (active / finished / failed, one tracked quest on the HUD as text). No markers (owner 2026-11-25).

**Classic option keeps:** the OpenMW-look topic window with the camera on the player, the book journal, no markers.
**Framework, other games:** the same core with objectives, a compass with markers, authored scenes (Yarn syntax),
NPC barks and ambient reactive lines, localised line IDs.

## 2. Where we are

- **Built, owner-verified:** `sf_rpg` quest store, journal, topics, `DialogueSession`, linker; `mw_rpg` filters,
  macros, result scripts on the native interpreter (rpg-adapter "As built (W3)"); window + journal on J. Fargoth
  ring flow confirmed (**DJ-1**, 2026-10-25). ROADMAP 2.4 `[~]`.
- **Built, unverified in game:** persuasion in the host + menu (**DJ-2** awaiting owner); stat, faction and crime
  functions in results (`statfn`, stats-s7, unit-tested only); dialogue reads real sheets (stats-s6).
- **Not running:** object (local), global and start scripts. Only dialogue results and console lines execute; no
  runtime use of `StartScript` (wiki "Known limits"). This is the biggest quest blocker (note §5, §7 #1): 1,207 vanilla
  scripts hold 1,468 `GetJournalIndex`, 286 `OnActivate`, 251 `OnDeath`, 321 `GetDistance` call sites
  (scripting-adapter "The corpus").
- **Stubs:** AI packages, combat, death/hit events, Hello/Fight/Flee/Alarm, `ForceGreeting`, sound/music (events
  only), magic, `ShowMap`, `GetPCInJail`.
- **Not started:** objectives, failed state, tracked quest (journal-ui gap P2), rule-based selection, barks,
  cinematic presentation, scene authoring format, line IDs, LLM seam.

## 3. Decisions

1. **Scripts run on the native interpreter, not Luau.** As built (W3) and Luau parked (owner 2026-10-18). The
   scheduling rules stay those of scripting-adapter "Instances, scheduling, state" (globals in start order, locals by
   `PlacementId`, active cells only, edge flags set before ticks), retargeted from Luau contexts to interpreter
   instances. Reference OpenMW `mwscript/*extensions.cpp`, `mwworld/localscripts.cpp` (cite commit per slice).
   Every command goes through one game API (`mw_rpg::api` over sf_* ops) that Luau binds later (DQ-1, done).
2. **Scripts are only as live as the systems they call** (note §5, OpenMW lesson). This lane binds functions whose
   system exists; AI, combat, crime and magic functions are bound by those lanes' slices (ai-adapter MA2/MA4/MA5/MA6)
   or here as soon as the system lands. A measured coverage number (call sites live / total, by family) gates each
   stage instead of an estimate (note §5).
3. **Main-quest trace test as the lane gauge:** a headless real-data test (like `fargoths_ring`) replays the main
   quest's choices and script ticks and prints the furthest stage per quest (QA walkthrough bots). Measurement only.
4. **Selection becomes a rule database** (Ruskin GDC 2012; Yarn Spinner 3 saliency; note §2): facts query, most-
   specific rule wins, recency-demoted random seeded from the host tick. INFO filters compile to criteria with a
   priority equal to today's OpenMW order, so classic selection is unchanged (equivalence test). Departs from rpg.md
   "first passing response wins". A module of `sf_rpg`, not an `sf_rules` crate (DECISIONS: none until Luau unparks).
5. **Presentation is a `DialoguePresenter` choice over one session** (note §3): `Classic` (window), `Cinematic` (auto
   camera, choices panel), `Barks` (subtitle only). Cinematic is the Modern Morrowind default (owner 2026-11-25).
   Auto camera: our own shot solver (two-shot / over-shoulder / close by distance and speaker change, physics ray for
   occlusion); shot length and reading speed are tunables (Witcher 3 generated-then-polished scenes, GDC 2016).
6. **Text-only lines get procedural talk** (jaw/head + gesture + gaze); voiced lines a Rhubarb Lip Sync viseme track
   (MIT, offline, not linked). No runtime ML; Audio2Face only as an external tool (note §3.4).
7. **Quests gain objectives, status and aliases in data** (Creation Kit, Witcher 3, Cyberpunk; note §4), exposed as
   facts so dialogue, AI and UI read one source. Markers setting `None | Compass | Guidance`: Morrowind `None`,
   framework `Compass` (owner 2026-11-25). Morrowind's latest journal entry is its one implicit objective.
8. **Authoring: own `DialogueScene` model + Yarn importer**; Ink via `bladeink` later (note §1). Yarn's runtime only
   if it passes the node-group check and reads our state through `VariableStorage`.
9. **Co-op (net-ready):** quest log, journal and topics stay shared host world state (`WorldRpg`, as built; BG3 /
   Divinity co-op share quest progress). `JournalSeen` and the tracked quest are per player. One player drives a
   conversation (`Conversations` claim, as built); others nearby receive its lines. Timed-choice deadlines and rule
   randomness use host ticks. The research note's "per-player" for quests is not taken.
10. **LLM: seam only, off** (note §6): `LineProvider` returns presentation text with no game effect.

## 4. Stages

### Stage 1: scripts run (main quest as far as scripts allow)

Ends with: the main quest playable from Seyda Neen to Caius Cosades' first orders and every non-combat step after
them; activators, doors, shrines and start scripts work.

| ID | Deliverable | Crates | Depends | Verify |
|---|---|---|---|---|
| DQ-1 | Spec: rewrite scripting-adapter "Decision", "Instances, scheduling, state", M5 table for the interpreter (spec slice) | docs | – | doc review; doccheck |
| DQ-2 | Coverage report over scripts + results by family and status (extends `vanilla_corpus_interprets`) | `mw_script` | DQ-1 | **done 2026-11-25:** 62.8 % bound (results 84.9 %) |
| DQ-3 | Main-quest trace test (decision 3) | `mw_rpg`, `mw_rpg_bevy` | DQ-1 | **done 2026-11-25:** 9 of 355 stages (A1_/A2_/B_/C_); `DQ3_QUEST=<id>` explains one quest |
| DQ-3b | New game without chargen applies the chargen captain's "duties" result (journal `A1_1` 1, Caius' package, gold), not only its topics (`mw_rpg_bevy/src/dialogue.rs:522` `skip_chargen`) | `mw_rpg_bevy` | DQ-3 | **done 2026-11-25** (DQ-3 9 → 12 of 355; A1_1 now stops at 18: a Caius INFO gated on `a2_2_6thhouse >= 5` shadows it, already at 49 in the trace: bot or filter question, next DQ-3c) |
| DQ-3c | Diagnose why `a2_2_6thhouse` is at 49 at the start of the trace and A1_1 stops at 18 (Caius INFO shadowing): trace bot or dialogue filter | `mw_rpg` (tests) | DQ-3b | **done 2026-11-25**: bot artefact (it took Dagoth Gares' ungated `Journal A2_2_6thHouse 49` early; Caius' INFO pick matches vanilla). Bot ranks trials (journal → story-sent → sequence break): 12 → 15 of 355 |
| DQ-3d | Trace bot capabilities: level-ups (`PcLevel` gates, A1_1 18 waits on level 4), world item pick-up and object scripts (A1_2 `dwemer_box`, `misc_dwrv_ark_cube00`), refuse sequence breaks unless stalled | `mw_rpg` (tests) | DQ-3c, DQ-5 for object scripts | DQ-3 total rises |
| DQ-4a | Global instances core: `StartScript`/`StopScript`/`ScriptRunning`, start order, step-limit fault, save by name | `mw_script` | DQ-1 | **done 2026-11-25** (10 exact tests) |
| DQ-4b1a | `mw_rpg::api` (`GameView`, serde `GameOp`, overlay, pure rules) extracted from `RunHost`; results host only binds | `mw_rpg` | DQ-4a | **done 2026-11-25** (DQ-3 still 9 of 355; 7 rule tests) |
| DQ-4b1b | Script host on `mw_rpg::api` implementing `mw_script::InstanceHost` | `mw_rpg` | DQ-4b1a | **done 2026-11-25** (3 fixture tests) |
| DQ-4b2a | Game wiring: public world `Env`, SCPT parse, SSCR start at new game, `FixedUpdate` tick applying ops | `mw_rpg`, `mw_scripts_bevy` | DQ-4b1b | **done 2026-11-25** (starts `main` then SSCR, as OpenMW) |
| DQ-4b2b | Script save section `mw/scripts` (`snapshot`/`load`, load replaces new-game instances) | `mw_saves_bevy`, `mw_rpg_bevy` | DQ-4b2a | **done 2026-11-25** (2 tests; a save without the section restarts the start list) |
| DQ-4b2c | Global locals from outside: `scriptid.var` reads, `SetLocal` writes into instances (step limit from the tunable store: later, with DQ-5) | `mw_rpg_bevy` | DQ-4b2b | **done 2026-11-25** (4 tests; script id wins over actor id, OpenMW `getMemberType`) |
| DQ-5a | Local instances core: keyed by caller key, created with zeroed locals on enter, tick in key order after globals, snapshot/load; answer open Q4 (disabled refs) from OpenMW | `mw_script` | DQ-4a | **done 2026-11-25** (10 tests; order = OpenMW: locals before globals) |
| DQ-5b1 | Local scripts in the game: script-active region (tunable cells), SCRI placements get instances (take on leave, never on `Disable`), locals ticked before globals, self = the placement (`Target` reference variant) | `mw_rpg_bevy`, `mw_rpg` | DQ-5a, DQ-4b2c | **done 2026-11-25** (8 tests; region from the player's cell, `Target::Ref`) |
| DQ-5b2 | `ref.var` reads/writes of local scripts' locals (mirror like `ScriptLocals`, rule 7 for base ids) | `mw_rpg_bevy` | DQ-5b1 | **done 2026-11-25** (5 tests; NPC dialogue locals = its script instance) |
| DQ-5b3 | Inventory-stack instances (holder key mapping, self for OnPCAdd/OnPCEquip) | `mw_rpg_bevy` | DQ-5b2 | **done 2026-11-25** (4 exact tests; mw_rpg_bevy 76 pass) |
| DQ-5b4 | NPC and container inventory stacks in active cells (OpenMW `addCell`); a global started from an item's script targets the stack, not a placed ref of its base (rule 7 today); fresh stack instances reported for DQ-6 edge events | `mw_rpg_bevy` | DQ-5b3 | exact per case |
| DQ-5c | Local script state saved: live + parked `LocalInstances` in `mw/scripts` (v2, migration from v1), load/restart restore instead of clearing | `mw_rpg_bevy`, `mw_saves_bevy` | DQ-5b1 | **done 2026-11-25** (4 tests; `mw/scripts` v2 + v1 migration) |
| DQ-5c2 | Spawned references' local scripts (needs the placement family bound: `PlaceAtPC`/`PlaceAtMe`, 0 % in DQ-2), `active_radius` + step limit from the tunable store | `mw_rpg_bevy` | DQ-5c | headless: a spawned scripted ref runs |
| DQ-5d | Bug (found DQ-5b1): `AddItem` on an unopened container (`Target::Ref`) goes to an empty `npc_store`; opening it later may skip seeding its contents (`inventory.rs` `container_store`) | `mw_rpg_bevy` | DQ-5b1 | **done 2026-11-25** (`container_refs` index; `Rpg::character` seeds via `container_store`) |
| DQ-5e | Rule 7 as the one resolver: `Content::target` `Actor(base)` for `AddItem`/`ModDisposition`/`Stats` uses `by_base` (HashMap order), so `"x".var` and `"x"->ModDisposition` can hit different references; `"chest_id"->AddItem` (base id of a container) does nothing (containers aren't registered actors) | `mw_rpg_bevy` | DQ-5b2 | **done 2026-11-25** (2 exact tests; mw_rpg_bevy 72 pass) |
| DQ-5f1 | `GetItemCount` on a container nobody opened reads its record contents (the `&self` view can't seed `container_store`), same count the seeded store gives later | `mw_rpg_bevy` | DQ-5e | **done 2026-11-25** (exact test; mw_rpg_bevy 77 pass) |
| DQ-5f4 | Levelled container entries (LEVI) rolled at seed, deterministic per placement + entry index (OpenMW `ContainerStore::fill`/`addInitialItem`), so `container_count` before opening = the store after; today both skip them (count 0) | `mw_rpg_bevy` | DQ-5f1 | **done 2026-11-25** (8 tests; mw_rpg 127, mw_rpg_bevy 79 pass) |
| DQ-5f5 | Switch the mw_actors LEVI scaffolds (`held.rs:353` `leveled_item_pick`, `source.rs:647` `leveled_pick`: lowest level, no chance none) to `mw_rpg::levelled_item`; delete them | `mw_actors` | DQ-5f4 | **done 2026-11-25** (`initial_items` + `LevelledSeed`; all callers `BY_RECORD` for now) |
| DQ-5f6 | Per-placement LEVI seed for actors: carry `LevelledSeed` (placement key + player level at spawn) in `CharacterState` (`sf_character` gets a game-opaque seed field) so `recipe` / `held_override` roll per actor and keep the roll across dusk swaps and saves | `sf_character`, `mw_actors`, `mw_actors_bevy` | DQ-5f5 | exact: two guards of one base can differ; same guard same pick after reload |
| DQ-5f2 | Facts/filter reads when rule 7 picks a placement that isn't a spawned actor (NPC in the region, not streamed): answer from the record + kept state, not `None` | `mw_rpg_bevy` | DQ-5e | exact |
| DQ-5f3 | Base targets for doors and unscripted activators (`Lock`/`Enable` by base id) in the rule 7 index | `mw_rpg_bevy` | DQ-5e, DQ-7 | exact |
| DQ-6 | Edge events: `OnActivate` replacing default activation + `Activate`, `OnPCAdd/Drop/Equip` | `mw_scripts_bevy` | DQ-5, IN31 (activation contract; `Activated` exists) | headless exact; owner MS-4, MS-5 |
| DQ-7 | World functions on the host: `Enable/Disable`, `Lock/Unlock`, `GetDistance`, `Position*`/`SetPos`/`GetPos`, `PlaceAtPC/PlaceItem` | `mw_rpg_bevy` | DQ-5, crime A0/A5 (locks) | headless per function |
| DQ-8 | UI functions: modal `MessageBox` + `GetButtonPressed`, `MenuMode`, `GetSecondsPassed`, `FadeIn/Out` | `mw_scripts_bevy`, `mw_ui` | DQ-5 | headless state machine; owner MS-5 |
| DQ-9 | Dialogue from scripts: `ForceGreeting`, `Choice`/`Goodbye` outside results, real `GetDisposition` | `mw_rpg_bevy` | DQ-5 | headless; owner DJ-3 |
| DQ-10 | Faction/rank/reputation results verified on real data (join Blades, `PCRaiseRank`) | `mw_rpg` | DQ-3 | real-install exact test |
| DQ-11 | Live gate: Balmora 5 min with all scripts: 0 runtime errors (or listed), legacy script ms per frame | `mw_scripts_bevy` | DQ-4..9 | log lines; tunable budget (spec: 0.3 ms) |

Owner check **DJ-3** (after DQ-11): new game, Seyda Neen to Balmora, Caius gives orders, journal updates, a scripted
door and a shrine work. DQ-3 is rerun after every slice; its number goes in the ROADMAP line.

### Stage 2: quests that move the world

Ends with: kill, escort, follow and spell quests advance (main-quest Blades tasks, guild quests). Each slice binds the
script functions once the other lane's system exists; the trace test shows the gain.

| ID | Deliverable | Crates | Depends | Verify |
|---|---|---|---|---|
| DQ-12 | Deaths feed `OnDeath`, `GetDeadCount`, `OnMurder`, `OnKnockout`; `OnPCHitMe` from hits | `mw_scripts_bevy` | combat lane death/hit events | headless exact |
| DQ-13 | Magic functions: `AddSpell/RemoveSpell`, `Cast`, `GetEffect`, `GetSpell` | `mw_rpg_bevy` | ROADMAP 2.7 A5b | headless per function |
| DQ-14 | Sound/music functions: `Say` (with subtitle), `PlaySound`, `StreamMusic` events to audio | `mw_scripts_bevy` | audio playback | log lines; owner DJ-4 |
| DQ-15 | `ShowMap`, `GetPCInJail`, `PayFine` hooks routed to map and crime; also `PayFineThief`, the two-call `GoToJail`, and the arrest greeting opening an `ArrestSession` (crime-adapter.md "Guards, pay and jail", 88525511) | `mw_rpg_bevy` | crime lane, map | headless |

AI functions (`AI*` packages, `StartCombat`, Hello/Fight/Flee/Alarm, `GetDetected`) are ai-adapter MA2/MA4/MA5/MA6.

### Stage 3: quest log and tracking

Ends with: a quest view in the journal (active / finished / failed), one tracked quest pinned on the HUD as text;
framework games get objectives and a compass.

| ID | Deliverable | Crates | Depends | Verify |
|---|---|---|---|---|
| DQ-16 | Spec: rpg.md "Quests" (objectives, status, aliases, facts, `QuestChanged`), journal-ui "Tracked quest" | docs | – | doc review |
| DQ-17 | `Objective`, `QuestStatus` (Active/Done/Failed), aliases; commands + events; serde | `sf_rpg` | DQ-16 | unit + postcard round trip |
| DQ-18 | Morrowind mapping: latest entry = implicit objective, QSTF/QSTR finish/restart to status | `mw_rpg` | DQ-17 | real-install exact |
| DQ-19 | Tracked-quest + quest-list view-model (per-player tracked, filters) | `sf_rpg_ui` | DQ-17 | unit |
| DQ-20 | Journal quest tab + HUD tracked line (Modern), book only (Classic) | `sf_ui_screens`, `mw_ui` | DQ-19 | owner DJ-5 |
| DQ-21 | Framework compass strip from `Objective.target` (off in Morrowind) | `sf_ui_screens` | DQ-17 | example scene; owner DJ-6 |

### Stage 4: rule-based selection and barks

Ends with: NPCs bark on hello, combat, flee, theft (Morrowind Voice records) and react to quest state; selection
engine ready for ambient lines.

| ID | Deliverable | Crates | Depends | Verify |
|---|---|---|---|---|
| DQ-22 | Spec: rpg.md "Dialogue" rewrite (facts, criteria, specificity, priority, once/cooldown, writes) | docs | – | doc review |
| DQ-23 | Rule DB + `Facts` trait + selection, indexed by concept | `sf_rpg` | DQ-22 | exact unit; seeded tie-break |
| DQ-24 | INFO filters compiled to rules with OpenMW-order priority | `mw_rpg` | DQ-23 | equivalence test: same INFO as `accepts()` across sampled contexts on the real install |
| DQ-25 | Bark presenter + triggers from AI/combat/crime events, Morrowind Voice DIAL | `sf_rpg`, `mw_rpg_bevy` | DQ-23, DQ-14 | headless trigger tests; owner DJ-7 |
| DQ-39 | Quick replies (owner 2026-11-25, replaces GTA's greet/antagonise button): when an NPC addresses the player (guard warning, "can I help you?", beggar, hawker), 2-3 short timed replies on screen without the dialogue window (defuse/apologise, pay, retort, ignore); the outcome by temperament (AI49) and rule DB | `sf_rpg`, `sf_rpg_ui`, `mw_rpg_bevy` | DQ-23, DQ-25, AI50 | headless: each reply's outcome, timeout = ignore; owner DJ-8 |

### Stage 5: cinematic conversations

Ends with: Modern Morrowind talks in staged scenes; Classic keeps the window; conversations break off on threat.

| ID | Deliverable | Crates | Depends | Verify |
|---|---|---|---|---|
| DQ-26 | Spec: rpg.md + ui "Dialogue presentation" (presenter trait, shot solver, pacing, interrupts) + industry comparison | docs | – | doc review |
| DQ-27 | `DialoguePresenter` trait; Classic presenter = today's window behind it | `sf_rpg_ui` | DQ-26 | unit; DJ-1 unchanged |
| DQ-28 | Shot solver (two-shot / over-shoulder / close, occlusion ray, tunables) | new thin `sf_dialogue_bevy` | DQ-27 | headless sanity (no NaN, camera not inside geometry) |
| DQ-29 | Cinematic presenter: subtitle line, reading-time pacing, topics as a choice panel | `sf_rpg_ui`, `mw_ui` | DQ-28 | owner DJ-8 |
| DQ-30 | Speaker gaze + procedural talk layer (jaw, head, gesture) on text lines | `sf_dialogue_bevy` | anim lane look-at / additive layers | owner DJ-9 |
| DQ-31 | Interrupt: host `Interrupt{reason}` on combat/damage/crime, fallback bark, timed choices (`deadline_ticks`) | `sf_rpg` | DQ-25 | headless exact |
| DQ-32 | Rhubarb viseme bake for voiced lines (offline tool step in bake) | `mw_bake` | DQ-30 | bake log counts; owner DJ-10 |

### Stage 6: authoring for new content (framework)

| ID | Deliverable | Crates | Depends | Verify |
|---|---|---|---|---|
| DQ-33 | Research: `bevy_yarnspinner` 0.10 compiler for node groups/saliency, `VariableStorage` fit (separate slice) | research | – | note |
| DQ-34 | Spec: `DialogueScene` model, stable line IDs, string tables per locale | docs | DQ-33 | doc review |
| DQ-35 | `DialogueScene` + runner on the session | `sf_rpg` | DQ-34 | unit + round trip |
| DQ-36 | Yarn importer (crate or own parser per DQ-33) | new `sf_dialogue_yarn` | DQ-35 | fixture `.yarn` goldens |
| DQ-37 | Line IDs + locale tables; journal re-renders from `source` | `sf_rpg`, `sf_rpg_ui` | DQ-35 | unit |
| DQ-38 | `LineProvider` seam, off by default | `sf_rpg` | DQ-35 | unit: no state writes |

Ink importer (`bladeink`), radiant quest templates (alias `Query` fill) and walk-and-talk scenes: queued after
Stage 6, seam only today.

## 5. Spec changes (spec slices before build slices)

- DQ-1 `scripting-adapter.md`: title/Goal/"Decision" (Luau translator) and the M5 status table contradict the as-built
  interpreter and the wiki; rewrite for the interpreter, Luau later binds the same game API (owner 2026-11-25; done DQ-1); register MS-4/MS-5.
- DQ-16 `rpg.md` "Quests" + "World and character state" (status, objectives, aliases, facts); `journal-ui.md` gap P2.
- DQ-22 `rpg.md` "Dialogue" "Database" + "Adapter hook": `accepts` becomes rule compilation; update the agent summary
  ("first passing response wins") and "Research" (challenges §5.3 again: rules + scenes, not one graph).
- DQ-26 `rpg.md` "Dialogue presentation" (new) and `rpg-adapter.md` Co-op (listeners, interrupts).
- DQ-34 `rpg.md` "Authoring" (new); `rpg.md` Open question 2 resolved by line IDs.
- Wiki `engine/rpg-and-scripting.md` "Known limits" updated by DQ-5/DQ-11; a ledger `docs/ledger/dialogue-quests.md`
  starts with DQ-2's first number.

## 6. Dependencies on other lanes

- **Interaction:** activation contract IN31 (DQ-6). **Crime:** `LockState` (crime-adapter A0/A5) for `Lock`/`Unlock` (DQ-7).
- **AI:** MA2 packages + script AI functions, MA4 Hello/Fight/Flee, MA5 detection, MA6 jail (ai-adapter). Gossip
  spreading reputation (owner 2026-11-25) is AI-owned; it writes reputation facts this lane's rules read (DQ-23).
- **Combat:** death and hit events (DQ-12); interrupts consume combat start (DQ-31).
- **Crime:** fines/jail hooks (DQ-15); witness reports interrupt conversations (DQ-31).
- **Animation:** look-at, additive talk layer, `PlayGroup` slots (DQ-30).
- **Travel:** `PositionCell`/service travel used by quests (DQ-7 covers the script side).
- **Provides:** running object scripts (scripted doors, traps, AI packages from scripts), quest facts +
  `QuestChanged`, the bark presenter, the `Interrupt` command.

## 7. Owner questions

1. In cinematic mode, Morrowind's topics show as a choice panel beside the framed NPC, and keywords inside a line
   stay clickable? **Recommend yes** (keeps Morrowind's topic play inside a modern scene).
2. Combat, damage or a crime nearby ends a conversation with a bark; talking again later resumes with topics and
   history kept? **Recommend yes.**
3. Text-only lines with procedural talking animation, no text-to-speech voices? **Recommend yes.**
4. Co-op: other players near a conversation see its lines, only the player who started it chooses? **Recommend yes.**
5. Morrowind (no markers): the tracked quest's latest journal line shown on the HUD in Modern, hidden in Classic?
   **Recommend yes.**

Owner answers 2026-11-25: all five yes, as recommended.
