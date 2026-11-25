# Dialogue, quests, journal: research slice (2026-11-25)

Scope: a generic dialogue + quest + journal system for `superfargoth/` with Morrowind as an adapter. Research only.
Our state read: wiki `engine/rpg-and-scripting.md`; `sf_rpg` spec (Goal, Quests, Journal, Dialogue); `journal-ui.md`;
`rpg-adapter.md` (goal, as built); `scripting-adapter.md` (M5 table, coverage), `mw_script/src/functions.txt`.
Items marked (memory) are from my own knowledge, not re-fetched; the rest cite a URL.

## 0. Where we are (one paragraph)

`sf_rpg` already is a Bethesda-shaped core: `QuestLog` = quest id -> `{stage:i32, finished}` (no objectives),
append-only dated `Journal`, `Topics` (known topic list), `DialogueSession` state machine (Greeting/Talking/Choosing/
Goodbye) over a `DialogueDb` (topic -> ordered responses; first response whose adapter `accepts()` wins), text-in-
window with Aho-Corasick topic hyperlinks. Everything is serde, host-authoritative, per-player. `mw_rpg` ports DIAL/INFO
filters, macros, result scripts (native interpreter), persuasion; the Fargoth ring flow is confirmed. The framework
is therefore a **first-match ordered filter list with a topic-menu UI**. That is Morrowind's model, not the 2018-2026
standard (choice wheel / dialogue tree, cinematic staging, objective-based quests with markers, reactive barks).
The good news: the state machine, journal and saves are format-neutral; the gaps are in authoring format, selection
semantics, presentation, quest objectives, and un-run object/global scripts.

## 1. Authoring/runtime systems as a generic base

| System | Fit | Status (checked) |
|---|---|---|
| **Yarn Spinner (Rust + Bevy)** | Closest fit. Node-based screenplay text, line IDs for localisation + voice, commands/functions, Bevy plugin. | `yarnspinner` 0.10.0 and `bevy_yarnspinner` 0.10.0, both published 2026-11-10, MIT OR Apache-2.0; `bevy_yarnspinner` depends on `bevy ^0.20` (crates.io API: yarnspinner, bevy_yarnspinner/0.10.0/dependencies). Repo README compat table lists Yarn Spinner **2.x only** (2.3-2.5); docs call the Rust port a Yarn Labs project, "not fully or officially supported" (docs.yarnspinner.dev, Bevy FAQ). **Storylets/node groups/line groups/saliency are Yarn Spinner 3 features**; I found no evidence the Rust port has them (unverified: grep its compiler before committing). |
| **Ink / bladeink** | Great narrative scripting (knots/stitches, weave, state), but a story-flow language, not a world-state matcher; output is a linear story + choices. | `bladeink` 2.1.0, 2026-10-17, Apache-2.0, pure Rust, no Bevy dependency (crates.io API). Ink itself MIT (memory). |
| articy:draft | Flow editor + DB for writers; proprietary tool, export JSON. Cyberpunk-class pipelines use similar in-house tools. Not adoptable as a format. | proprietary (memory) |
| Dialogue System for Unity (PixelCrushers) | Unity asset, closed; its model (conversation DB, conditions/scripts per entry, quest machine) is the reference feature list only. | commercial (memory) |
| UE5 | No first-party dialogue graph; studios build on Sequencer + data tables + Gameplay Tags; Epic's 2024 "Narrative Tools"/marketplace plug-ins are the norm. Lesson: dialogue = asset graph + tag-conditions + sequencer scene. | (memory) |
| Valve rule DB (section 2) | The selection layer. | GDC 2012 |

**Recommendation: do not adopt one language as the framework's format.** Two different jobs are being conflated:
(a) *selection* of which line/scene plays (needs a world-state query: section 2) and (b) *flow* inside an authored
scene (lines, options, jumps, commands). Make (b) a small framework-owned data model (`DialogueScene { nodes: lines,
options, jumps, commands }`, serde, stable line IDs, per-line speaker/tags/conditions), and offer **Yarn syntax as one
importer** (`bevy_yarnspinner` 0.10 is on Bevy 0.20, dual-licensed; use its compiler/runtime only if it passes the
node-group check, otherwise import `.yarn` to our model with our own thin parser; Yarn's grammar is open and small).
Ink gets an optional importer later via bladeink (Apache-2.0 is GPL-3 compatible). Reasons: Yarn's strengths (line IDs,
localisation string tables, voice-over linking, commands) are exactly our missing pieces; a runtime that owns its own
variable store would fight `QuestLog`/globals/saves/net (host authoritative, plain serde), so any adopted runtime must
read/write our state through a trait (Yarn's `VariableStorage` is such a seam, 2.x API).

**Morrowind adapter mapping.** DIAL/INFO already maps to the generic model with no loss:
- DIAL topic = a *topic node group*; each INFO = a candidate with a **rule** (filters) and a result script. Greeting
  DIALs = a group evaluated on open. Today's first-accepted-wins ordering stays as the `MwDialogueRules` tiebreak
  (OpenMW order), implemented as a priority on the rule, not as a special case in `sf_rpg`.
- Choice() results -> generic `options` node; Goodbye/ForceGreeting -> commands.
- Hyperlinked keywords/topic list = a **presentation mode** (`TopicList`), not a data model. A generic game uses
  `Wheel` or `Tree`; Morrowind classic keeps `TopicList` (owner directive: classic stays reachable).
- Text macros (`%PCName`, `%Faction`) stay adapter `render`.

## 2. Valve's dynamic dialogue and successors

Ruskin, GDC 2012, "AI-driven Dynamic Dialog through Fuzzy Pattern Matching" (gdcvault.com/play/1015528;
slides cdn.fastly.steamstatic.com/apps/valve/2012/GDC2012_Ruskin_Elan_DynamicDialog.pdf): game code fires a *query*
(a bag of key/value facts: concept, who, health, nearby enemies, recent events), a **rule database** of thousands of
rules, each a list of criteria (key, range/equality/enum), is matched; the rule with the **most satisfied criteria**
(most specific) wins, ties broken randomly / by recency; the winning rule runs a *response* (a line, a scene, an
animation, a fact write, a follow-up concept) and writes facts back (memory: "player already heard X"). Writers author
rules in a tool with no code changes (summary: Game Developer, "Creating Behaviours with Fuzzy Pattern Matching";
emshort.blog 2012-03-16). Used in L4D (barks), Portal 2, TF2, and copied widely.

Why it is the right generalisation: Morrowind's INFO filter list is a *hand-ordered first-match* special case of it
(OpenMW orders INFO by load order, which is a poor proxy for specificity). Yarn Spinner 3's node/line groups with
`when:` + saliency "best" strategy (most conditions passed wins, recency demotion, random tie) are the same idea inside
one tool (docs.yarnspinner.dev/write-yarn-scripts/advanced-scripting/node-groups, /saliency). Successors: Firewatch
(Campo Santo, Valve-derived system, GDC 2017 talk by Sean Vanaman/Jake Rodkin, memory), Hades (priority requirements
lists + "play once", Supergiant GDC 2021, memory), Cyberpunk/RDR2 ambient conversation systems (condition-gated,
time-of-day, with interruption; memory; no primary source fetched).

**Design for us (`sf_rules`-style, Bevy-free):**
- `FactStore` view: read-only query trait `Facts::get(key) -> Value` over world state (quest stage, flags, time, weather,
  distance, relationship, location tags, last-heard timestamps). Queries are built by the *caller* (a bark trigger, a
  talk start, an ambient scheduler).
- `Rule { criteria: Vec<Criterion>, weight, once/cooldown, response: ResponseRef, writes: Vec<FactWrite> }`; selection =
  max criteria count, then recency-demoted random (seeded from the host tick for net determinism).
- Same engine serves: dialogue entry selection, barks (combat noises, "see player"), ambient banter (NPC-NPC),
  reactive lines (weather, crime, quest state), scene selection for radiant content.
- Speed: index rules by concept + one hot key (Valve used a sorted-bucket scan; fine at tens of thousands of rules,
  evaluated on events, never per frame). This is a perf-neutral choice.
- Morrowind adapter: INFO filter -> `Criterion` list (the adapter already enumerates ~dozen filter types), INFO order ->
  tiebreak priority; so `MwDialogueRules::accepts` becomes `compile_to_rules` at DB build. Barks (`Say`, hit/flee/hello
  voice records) get a real home instead of the current `TopicKind::Bark` stub (`Say`/`PlaySound` are logged stubs).

## 3. Presentation

Industry (2018-2026): dialogue is a **scene**, not a window.
- **Witcher 3**: dialogue scenes were generated by an editor-time algorithm (camera cuts, blocking, gesture/anim
  selection by line, from the dialogue script), then hand-polished by designers; Tomsinski, GDC 2016, "Behind the Scenes
  of Cinematic Dialogues in The Witcher 3" (gdcvault.com/play/1022988). CDPR disputes "algorithm did most of it":
  starting point, not final.
- **Cyberpunk 2077**: first-person throughout; dialogue camera remains the player's eyes, NPCs perform, choices as a
  radial/list overlay with a **timer** on some, NPCs can interrupt; (memory).
- **BG3**: scripted cinematic cameras via Larian's dialogue tool, large timeline; **KCD2**: hybrid first-person with
  cut-aways; **Starfield**: over-the-shoulder auto-camera with a list; **Mass Effect**: wheel + interrupts (memory).
- Constants across all: shot-reverse-shot from line speaker, auto head/eye look-at, idle + gesture layers on talk,
  choice summaries (paraphrase vs full line), timed choices only where tension is the point, **walk-and-talk** as a
  scripted escort/follow node, **interruptible**: combat/damage/crime event cancels scene with a fallback bark.

**Our design:**
1. **Staging data per line**, all optional, defaults from heuristics: `speaker`, `listener`, `camera: Auto|Shot(id)|None`,
   `gesture/emotion tag`, `look_at`. Auto camera = our own small solver (two-shot/over-shoulder/close selection by
   distance, speaker change, ~3-5 s shot length; occlusion check via `sf_physics` ray). Not a research problem.
2. **Three presentation modes behind one `DialoguePresenter` trait** (the framework's session stays the same):
   `Classic` (window, topic list, camera stays on player; Morrowind), `Cinematic` (auto camera, wheel or list, timed
   options), `Barks` (subtitle only, no input).
3. **Timed choices + interruption** = fields on `Choosing { deadline_ticks, default }` and a host cancel command
   (`Interrupt{reason}`); deadline is a host tick count so it is net-deterministic.
4. **Lip sync / facial from audio**: voiced lines are the minority in Morrowind (voice files exist for vanilla greetings
   and race barks but most INFOs have text only). Options:
   - Offline bake at pack-build: **Rhubarb Lip Sync** v1.14.0 (2025-04-03, MIT, 2D visemes from audio + optional
     text; GitHub DanielSWolf/rhubarb-lip-sync) outputs a viseme timeline (A-H,X) we map to blendshapes/jaw. Cheap,
     deterministic, English-centric (PocketSphinx).
   - **NVIDIA Audio2Face-3D** models + SDK open-sourced 2025 (developer.nvidia.com/blog/nvidia-open-sources-audio2face-
     animation-model); the models are under the *NVIDIA Open Model License* (HF model card), **not** an OSI licence:
     treat as optional external tool, never linked, never shipped (same rule as DLSS). SDK licence unverified.
   - Oculus/OVR LipSync: proprietary, Unity/UE plug-in; exclude.
   - Text-only lines: no audio to drive. Standard solution (Skyrim/Fallout 4 used procedural jaw from audio; for
     silent text): **talk-loop** (jaw/head noise layer driven by text length and punctuation, like early Bethesda/
     Hades), text-to-viseme from syllable count (cheap), plus eye/head gaze. Recommend: procedural *talk gesture + jaw
     flap* on text-only lines, pause for reading time = f(characters) (typewriter 30-60 cps, then "continue"
     prompt), and Rhubarb viseme tracks only for voiced lines, baked once. No runtime ML.
   - Note `docs/research` clip rules: no mocap; gestures from free packs / Kimodo clips (matches owner policy).

## 4. Quests

Reference models:
- **Skyrim/Creation Kit** (memory): Quest = list of **stages** (int) with log text + result script; **objectives**
  (display targets, tracked), **aliases** (named slots filled by a reference/find-conditions: "the bandit chief", scene
  actors), **scenes** (phases of actions with dialogue packages), and **radiant** quests via templates + alias
  fill-in. Our `QuestLog{stage,finished}` is Morrowind's flat form (journal index), no aliases/objectives.
- **Witcher 3**: quest graph (blocks, conditions, scene calls), objectives with map markers, journal states
  (active/failed/success). **Cyberpunk**: quests as *phases* of a graph with objective sets, markers via "mappins", GDC
  2023 takeaways (gamedeveloper.com/gdc2023/10-key-takeaways-from-the-quest-design-of-cyberpunk-2077-and-the-witcher-3).
  **KCD2**: objective-led with optional/branch tracking and map hints; (memory).
- **Failure/exclusion**: explicit `Failed` state plus `abandoned`; conflicting quests set each other's state through
  facts, not special cases.

**Design (framework):**
- Keep stage + journal as is (needed for Morrowind and free to keep). Add a generic layer on top, in data, not code:
  - `Quest { id, stages, objectives: Vec<Objective{id, text_ref, state: Hidden|Active|Done|Failed, optional, target:
    Option<TargetRef>}>, aliases: Vec<Alias{name, fill: Fixed|Query(criteria)}> }`.
  - Quest state changes are *facts* (`quest.<id>.stage`, `.objective.<id>`) so the rule DB, dialogue, barks, AI and the
    UI all read one source. Writes only through host commands (net rule), emitted as events (`QuestChanged`).
  - Quest logic = a small **graph or rule set in data** (stage transitions on fact conditions), run by the host on
    fact changes; MWScript is the adapter's way to author the same transitions.
- **Radiant**: template quest + alias fill by `Query` + a rule-selected scene. Industry benchmark (Skyrim Radiant Story,
  Shadow of Mordor Nemesis) is shallow-by-design; scope = **seam only now** (aliases + queries give 80 %).
- **Markers vs directions**: standard is a three-level option: `None` (Morrowind text directions only; the journal text
  carries them), `Compass` (distance-faded markers on a compass strip, Skyrim/Witcher/Cyberpunk), `Guidance` (KCD2/
  Elden Ring: hints and grace/trail rather than a quest arrow). Make it a per-game *setting* driven by `Objective.target`
  (data exists but marker display is optional). Morrowind default = None; `journal-ui.md` marks this as gap P2.
- **Journal UX**: keep the book (Morrowind classic) and add the objective-centric "quest log" view: active/failed/
  completed filter, tracked quest, per-quest log of past stages (already have), notes. `JournalSeen` new markers already
  match the industry. Missing: tracked quest + objective list view-model.

## 5. MWScript coverage

Facts available (no per-call-site percentage is recorded in the as-built notes; I did not build or run anything):
- `functions.txt`: 233 function rows; status field is M5-era: ~99 `live`, ~83 `stub`. Stat, faction and crime functions
  (~140 names via `mw_script::statfn`) were flipped to live on 2026-11-08 (CHANGELOG, "stats-s7"; written on a machine
  that could not compile; unit-tested only).
- **Biggest blocker, not a function family: object and global scripts do not run.** Wiki "Known limits": only dialogue
  result scripts and console lines execute; `StartScript` has no runtime use. The M5 table lists "live" for activators
  and start scripts, but the wiki (newer) says they do not run in game. This means any quest driven by an object/global
  script (most non-trivial ones: Tribunal/Bloodmoon, guild quests, many "if OnActivate"/GetDistance triggers) is dead
  regardless of the 99 live functions.
- Still stubs/`never`: `AITravel/AIFollow/AIEscort/StartCombat/StopCombat`, `Get/Set/Mod{Hello,Fight,Flee,Alarm}`, `Get/Set/
  ClearForce*`, `GetWeaponDrawn`, `GetTarget/GetDetected/GetAttacked`, `OnDeath/OnPCHitMe/OnKnockout/OnMurder`,
  `GetDisposition` (stub=50; persuasion exists in host untested), faction reaction functions, `GetPCInJail`, `ForceGreeting`,
  `Choice`/`Goodbye` in non-dialogue contexts, `Say`/`PlaySound`/`StreamMusic` (events only), magic (`Cast`, `AddSpell`,
  `GetEffect`), `ShowMap`.
- Quest-blocking families, ranked by quest count (my estimate from memory of the vanilla content; **verify with the
  W2 coverage report**): (1) object/global script execution itself, (2) AI packages (escort/follow/travel quests),
  (3) combat/death events (kill quests: `OnDeath`, `GetDeadCount` is only live as 0 until deaths feed it), (4) faction/rank/
  reputation (now flipped live, untested), (5) disposition, (6) magic and spell/effect tests, (7) sound/music (cosmetic).
- **OpenMW** (memory): implements essentially the full vanilla + Tribunal + Bloodmoon function set natively in C++
  (`apps/openmw/mwscript/*extensions.cpp`), runs *every* script (global, local per reference, start scripts) on a
  per-frame scheduler with the actual engine systems behind it (AI packages, combat, magic, factions, crime); its
  dialogue filter order and `Choice`/`Goodbye` semantics are what we ported. OpenMW's lesson: scripts are only as live as
  the engine hooks they call; fidelity comes from the AI/combat/magic systems, not the interpreter.
- Implication: the cheapest high-leverage MWScript task is **run global/local/start scripts on the interpreter**
  (the interpreter and `Host` trait already exist and are step-limited); then wire AI/combat events as the AI and combat
  lanes deliver them. Add a **coverage gate**: corpus call-site percentage by family printed by an `#[ignore]`
  measurement test, so the roadmap states a number.

## 6. LLM dialogue seam

Mantella/Herika (Skyrim mods), Inworld, NVIDIA ACE (NPCs with LLM + TTS + A2F): all external services, cloud or local
models, latency 0.5-3 s, non-deterministic, safety/moderation cost; shipped AAA use is nil (memory; no URL fetched).
Realistic scope: **seam only**: a `LineProvider` trait returning a line for (speaker, facts, utterance) that the host
calls when a scene is flagged `freeform`; the result is *presentation text with no game effect* (no quest writes, no
facts, no results scripts) so the net/replay/determinism model is untouched; off by default; no model code in the
framework; a mod can implement it. Never gate quests on it.

## 7. Gaps vs the standard (ranked by player impact) and recommendations

| # | Gap | Impact | Recommendation |
|---|---|---|---|
| 1 | Object/global/start MWScripts do not run | Most vanilla quests/guilds dead | **Adopt** as top MW task: scheduler for local+global scripts on the existing interpreter; then coverage test by family |
| 2 | Selection = first-accepted ordered list | Dialogue feels canned; no barks/ambient/reactive | **Change**: add rule DB (criteria count, recency, once, writes); MW filters compile to it |
| 3 | Presentation = window + topic list only | Looks dated; no staging | **Add** `DialoguePresenter` modes (Classic/Cinematic/Barks) + auto camera; Classic is default for MW |
| 4 | Quests have stage but no objectives/aliases/markers | No tracking, no failure state, no radiant | **Add** objectives + aliases + fact-backed state; marker setting None/Compass/Guidance |
| 5 | No authoring format for new content | Modders/other games write Rust-free data? | **Adopt** own scene model; Yarn syntax importer (bevy_yarnspinner 0.10 or own parser); Ink via bladeink optional |
| 6 | No voice/lip-sync path; text-only lines unstaged | Dead faces | Procedural talk gesture + reading-time rule; Rhubarb bake for voiced lines; A2F only as external tool |
| 7 | Localisation: text snapshot in journal, no line IDs | Blocks non-English packs | Stable line IDs on every line (Yarn model); string table per locale |
| 8 | Timed choices / interruption | Tension, missing in TES | Host tick deadlines; cancel command |
| 9 | LLM freeform | Optional | Seam only (section 6) |
| 10 | Journal UI lacks tracked quest view | Medium | View-model addition |

**Candidate owner questions (player-facing only; recommended answer first):**
1. Cinematic dialogue (camera, wheel) is the default for new games; Morrowind keeps the classic topic window as its
   default and offers Cinematic as a setting? Recommend **yes** (classic stays reachable, modern default for non-MW).
2. Quest markers: default None for Morrowind (directions in text), Compass for the generic framework, a setting to
   switch? Recommend **yes**.
3. Should NPCs interrupt/cancel conversations on combat/crime and resume with a bark? Recommend **yes**.
4. Voiced-lines policy: ship text-only with procedural talking animation, no TTS in the framework? Recommend **yes**.

Not owner questions (decided): formats (own scene model + Yarn importer), rule-DB perf and indexing, LLM seam default off,
lip-sync tool choice, line-ID/localisation scheme.

## Missing / unverified

- bevy_yarnspinner 0.10 feature coverage (Yarn 3 node groups/saliency): unverified; README compat table says Yarn 2.x.
- Per-call-site MWScript coverage percentage: not in notes; needs the W2 report run.
- Cyberpunk/RDR2/KCD2/Starfield/BG3 presentation details are from memory, no primary source fetched.
- Firewatch and Hades GDC talk URLs not fetched.
- Ruskin slides read only through secondary summaries; criteria matching details above are from memory of the talk.
