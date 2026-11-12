# NPC AI ecosystem survey (2026-11-06)

Research only, no code. Goal: pick what to reuse before the `sf_ai` spec. Our Bevy is 0.20 (root `Cargo.toml`). Facts below
come from crates.io API, GitHub pages and docs.rs fetched 2026-11-06; "unverified" marks what I could not check. Last-activity
dates are crates.io `updated_at` unless stated (GitHub pages did not show commit dates in the fetch).

## Verdict

- **Behaviour layer:** the crate ecosystem is thin and mostly code-defined. Nothing gives us mod-loadable, deterministic,
  Bevy-free AI out of the box. Best fits for a *thin adapter* are `bevior_tree` 0.12 (Bevy 0.20, serde/typetag feature) and
  `bevy_bae` (HTN as entities, but Bevy 0.18). Both are small; we should expect to own the executor.
- **Recommended plan:** build a small Bevy-free `sf_ai` core ourselves (goal requests, HTN-style task decomposition or utility
  scoring, data from records), and treat crates as design references. Reason: net-ready fixed tick, content-supplied goals and a
  classic-package seam do not match any crate's model. Revisit `bevior_tree` as the BT executor if we want trees.
- **Do not adopt:** `big-brain` (Bevy 0.15 pin, repo archived 2025-10-07), `dogoap` (Bevy-reflect 0.16, last release 2025-05).
- **C# libraries (Fluid BT / Fluid HTN):** both MIT, good design references, no Rust port found. Port the *design* (Fluid HTN's
  domain/planner/effect-type model) if we pick HTN; do not port the code.
- **Nemesis:** I could not find the Rust reverse-engineered rewrite the owner saw (see section). Patent US 10,926,179 runs to
  ~2035-2036. Clean-room rule blocks code from it regardless. Behaviour-spec-only reuse is the only route.
- **Navigation:** adopt `bevy_rerecast` (0.6, Bevy 0.20) + `bevy_landmass` (0.13, Bevy 0.20), or `vleue_navigator` 0.17. Decide
  after a streamed-tile spike.
- **Perception / AI LOD:** nothing usable found; build ourselves. Local avoidance: `bevy_orca` is small and current.
- **Open questions for owner:** (1) link to the Nemesis Rust repo? (2) BT vs HTN vs utility as the primary model (Q below).
  (3) Appetite for emergent "story" layer (memory/relationships) as framework vs Morrowind-only? (4) OK to take Veloren
  `rtsim` as a design reference (GPL-3, code portable)?

## 1. Behaviour layer crates

| crate | approach | licence | Bevy | last activity | Bevy-free core | data-driven | fit notes |
|---|---|---|---|---|---|---|---|
| bevior_tree 0.12 | behaviour tree plugin, nodes as entities/components | see repo (unverified; macros crates) | 0.20 | 2026-10-15 | no (needs `bevy`) | optional `serde` + `typetag` features, `ron` dev-dep; README does not describe loading | tracks Bevy every release; 61 stars, 194 commits; best BT candidate for us |
| bevy_behave 0.5 | BT with dynamically spawned task entities | MIT OR Apache-2.0 | 0.18 (README table: 0.6 = Bevy 0.19, not on crates.io list I saw) | 2026-02-06 | no | no: author says asset loading "PRs welcome" | 131 stars; behind our Bevy |
| bevy_bae 0.1 | HTN planner, "behaviour as entities", properties via `bevy_mod_props` | MIT OR Apache-2.0 | crate deps 0.18 (README table says 0.17) | 2026-01-29 | no | tasks are Bevy bundles in code; properties are data | 52 stars; closest to "goal then plan"; needs port to 0.20 |
| beet 0.0.9-rc.1 | whole app framework with behaviour (BT) | MIT OR Apache-2.0 | 0.18 | 2026-02-02 | no | n/a | pre-release, wide scope; not a drop-in |
| big-brain 0.22 | utility AI (scorers, actions, thinker) | see repo (unverified) | 0.15 | 2024-11-30 | no | no, code builders | GitHub repo archived 2025-10-07, moved to Codeberg; Bevy 0.16 per README |
| bevy_observed_utility 0.2 | utility AI via observers | MIT OR Apache-2.0 | unverified | 2025-03-05 | no | no | stale |
| bevy_thinker 0.4 | utility-scored behaviour trees | unverified | 0.20.1 | 2026-11-05 | no | unverified | brand new (163 downloads); watch only |
| seldom_state 0.16 | component-based state machine | MIT OR Apache-2.0 | 0.18 | 2026-04-02 | no | no | good for low-level animation-ish states, not goals |
| moonshine-behavior 0.6 | minimal state machine with history stack | unverified | 0.20 | 2026-10-11 | no | no | tracks Bevy; stack of behaviours maps to interrupt/resume |
| dogoap 0.5 | GOAP | MIT | bevy_reflect 0.16 | 2025-05-21 | planner mostly (reflect only) | no | stale for 0.20; pure planner could be forked |
| bonsai-bt 0.14 | behaviour tree, no ECS | MIT | none | 2026-09-22 | yes | no (code) | 119k downloads; usable as logic core, no mod data |
| htn 0.2.1 | HTN for game AI | Apache-2.0 | none | 2026-10-06 | yes | unverified | 26 downloads, new, unproven |
| emergent 1.9 | BT, utility, FSM, planner toolset, ECS-free | MIT OR Apache-2.0 | none | 2026-06-20 | yes | unverified | most complete Bevy-free toolkit; 32k downloads; single maintainer |
| cranium-core 0.3 | modular game AI, `no_std`, action-set loader | MPL-2.0 | plugin crate separate | 2026-07-20 | yes | has actionset loader (unverified depth) | MPL only unmodified; 871 downloads |

Observations:
- No crate is deterministic by design (none document it). Determinism is mostly about our schedule (fixed tick, ordered
  queries, seeded RNG) and avoiding entity-spawn-per-task executors with unordered observers.
- Data-driven is the gap everywhere. For mod-supplied behaviour the practical choices are (a) a data schema we define
  (HTN domains/utility curves as records) interpreted by our core, or (b) serde-able trees (`bevior_tree` feature).
- Code-defined trees fit "content asks for goals": goals come from records and a fixed set of Rust-side task primitives
  implement them.

## 2. The owner's two C# libraries

- **fluid-behavior-tree** (ashblue): MIT, 1.2k stars, 354 commits. Builder-pattern, code-driven BT; Sequence, Selector,
  Parallel, decorators, `Splice()` for reuse, runtime visualiser. No data format. Nothing a Rust BT crate lacks; fits only as
  API inspiration. No Rust port found.
- **fluid-hierarchical-task-network** (ptrefall): MIT, 459 stars, 264 commits. Total-order forward-decomposition HTN; domain
  builder; partial planning/pause; domain splicing; run-time domain slots (good for mods); effect types PlanOnly /
  PlanAndExecute / Permanent (relevant for predicting world-state changes). Derived ports exist in JS, C++, Lua, Godot 4;
  no Rust port found on the page. `bevy_bae` and `htn` crates are the Rust neighbours (not verified as ports).
- Verdict: neither fits as code. The HTN design (domain slots + effect types + replanning on world-state change) fits our
  "goal request, AI fulfils it" model and is worth reimplementing in our core. Plain BT adds little over `bevior_tree`.

## 3. Nemesis-like systems

- **Rust rewrite: not found.** Searches of GitHub, crates.io ("nemesis", 21 results, none relevant), and web returned only
  Minecraft mods (ToroCraft/NemesisSystem, Oromist-Nemesis-Orcs), `marti368/Nemesis` (pirate simulator), a Skyrim mod, a
  Cyberpunk mod (NightCity Vendetta: trust/fear/respect/hostility profiles), RDR2 mod, and the itch.io jam "Frenemy"
  open-source parody. The owner's link is needed to assess repo, licence and quality.
- **Provenance risk (a), patent:** US 10,926,179, "Nemesis characters, nemesis forts, social vendettas and followers in
  computer games", Warner Bros. Entertainment, filed March 2015, granted 2021-02-23. Patent Arcade says valid until 2035;
  TechRadar's headline says 2036. Exact date unverified (check USPTO/Google Patents; term adjustments possible). Claim 1 as
  summarised: a processor detects a predefined event involving the player avatar and a first NPC, and changes parameters of a
  *second* NPC. No legal conclusion here; owner may want counsel before building anything labelled Nemesis-like.
- **Risk (b), clean room:** `superfargoth/CLAUDE.md` rule 3 forbids porting from decompiled or reverse-engineered game code,
  so a repo described as reverse engineered cannot be a code source.
- **Behaviour-spec-only reuse:** write our own spec from public descriptions (GDC talks, interviews, postmortems): NPCs
  with persistent memory of encounters, trait/role slots, rank hierarchy, events that propagate to *other* NPCs. Generic
  designs (grudges, gossip, factions) predate the patent (Dwarf Fortress, Crusader Kings, Morrowind disposition/factions).
  Veloren `rtsim` (below) is a legal GPL-3 reference for sentiments and factions.
- **Other open-source memory/relationship projects found:** none in Rust; only mods above. Design write-up:
  bommyknocker.bearblog.dev "Hack the Nemesis System into your RPG" (not reviewed beyond the search snippet).

## 4. Navigation

| crate | approach | licence | Bevy | last activity | Bevy-free core | data-driven | fit notes |
|---|---|---|---|---|---|---|---|
| rerecast 0.5 / bevy_rerecast 0.6 | Rust port of Recast; Bevy integration; avian backend | MIT OR Apache-2.0 | 0.20 | 2026-10-26 | yes (`bevy_rerecast_core`, `no_std` mention) | navmesh as asset | 176 stars; generates and partially regenerates tiles; best generation candidate. Verify tile streaming and detail-mesh output |
| bevy_landmass 0.13 (+landmass 0.9) | pathing, steering, local avoidance over supplied navmeshes | MIT OR Apache-2.0 | 0.20 | 2026-10-27 | yes (`landmass`) | meshes from rerecast via `landmass_rerecast` | 117 stars; local avoidance exists per author docs (unverified on page); good pair with rerecast |
| vleue_navigator 0.17 | Polyanya any-angle pathing + navmesh builder | MIT OR Apache-2.0 | 0.20 | 2026-10-27 | polyanya separate (unverified) | glTF / mesh / obstacle-built | 504 stars; runtime update from obstacles; tiles/3D layers not described |
| oxidized_navigation 0.12 | Recast-like tile generation, Rapier/Avian | unverified | 0.15 | 2024-12-25 | no | no | stale for us |
| navmesh-rs 0.2.2 | pure-Rust Recast + Detour + DetourCrowd + TileCache | MIT OR Apache-2.0 (from zlib recast) | none | 2026-11-06 | yes, `no_std`+alloc | no | 1 star, 59 commits: brand new, AI-era risk; only full Detour crowd/tile cache port |
| bevy_orca 0.2 | ORCA local avoidance (RVO2), no ECS | MIT OR Apache-2.0 | 0.20 | 2026-10-30 | yes (bevy_math) | n/a | README states agent-written code, no line-by-line human audit |

Notes: a streamed-tile runtime pipeline (generate per terrain tile, stitch borders, drop with the tile) is the key
requirement; I could not confirm it for any crate from the pages fetched. A spike must test it. Navmesh bake time during play
conflicts with "nothing bakes during play": tile generation at stream-in is runtime work and needs the owner's explicit
OK, or prebaked per-tile navmesh.

## 5. Perception and AI LOD

- Perception: `saddle-ai-fov` (FOV/LOS toolkit with optional stealth adapter) found via docs site; crates.io API lookup under
  that name returned 404, so licence, version and Bevy support are unverified. No hearing/stealth crate otherwise.
- AI LOD / process levels: none found. Veloren `rtsim` is the only open-source reference I know for simulating NPCs off-screen
  at reduced fidelity (see below). Plan to build ourselves, driven by the same streaming rings as terrain.

## 6. Whole projects

- **Veloren** (GitLab, GPL-3.0-or-later per project page; Rust, 18.5k commits, since 2019): contains `rtsim`, an NPC
  simulation module with factions, sentiments, personalities and LOD simulation (topics listed on the repo tree page; I did
  not read the code). Licence is compatible with ours. Most promising source to study for off-screen NPC life, long-term
  state and reactions. Engine is its own (not confirmed Bevy). Provide a follow-up research slice reading `rtsim` design.
- **OpenMW**: reference for classic Morrowind AI packages (wander, travel, escort, follow, pursue); already our plan for the
  classic executor behind the seam. Not re-researched here.
- **Skyrim "Give me another fight" / NightCity Vendetta mods**: design ideas for memory profiles only; not code sources.
- No notable Bevy game with open-source NPC AI found in the single search run; coverage here is weak.

## Gaps and unverified

Licence of `bevior_tree`, `big-brain`, `moonshine-behavior`, `oxidized_navigation`, `bevy_thinker`, `saddle-ai-fov`;
commit dates on GitHub pages (not shown by the fetch tool); `bevy_behave` 0.6 on crates.io; Bevy-free status of
`vleue_navigator`; exact patent expiry; the Nemesis Rust repo; Veloren `rtsim` code quality.

## Sources (all accessed 2026-11-06)

- https://crates.io/api/v1/crates?q=bevy%20behavior&per_page=20&sort=relevance
- https://crates.io/api/v1/crates?q=nemesis&per_page=15
- https://crates.io/api/v1/crates?q=htn&per_page=10
- https://crates.io/api/v1/crates?q=utility%20ai&per_page=15
- https://crates.io/api/v1/crates?q=navmesh&per_page=20
- https://crates.io/api/v1/crates/bevior_tree/0.12.0/dependencies
- https://crates.io/api/v1/crates/bevy_bae and /bevy_bae/0.1.0/dependencies
- https://crates.io/api/v1/crates/bevy_behave and /bevy_behave/0.5.0/dependencies
- https://crates.io/api/v1/crates/beet and /beet/0.0.9-rc.1/dependencies
- https://crates.io/api/v1/crates/seldom_state and /seldom_state/0.16.0/dependencies
- https://crates.io/api/v1/crates/big-brain/0.22.0/dependencies
- https://crates.io/api/v1/crates/moonshine-behavior/0.6.0/dependencies
- https://crates.io/api/v1/crates/vleue_navigator/0.17.0/dependencies
- https://crates.io/api/v1/crates/bevy_rerecast/0.6.0/dependencies
- https://crates.io/api/v1/crates/bevy_landmass and /bevy_landmass/0.13.0/dependencies
- https://crates.io/api/v1/crates/oxidized_navigation/0.12.0/dependencies
- https://crates.io/api/v1/crates/dogoap and /dogoap/0.5.0/dependencies
- https://crates.io/api/v1/crates/bonsai-bt
- https://crates.io/api/v1/crates/bevy_observed_utility
- https://crates.io/api/v1/crates/htn
- https://crates.io/api/v1/crates/cranium-core and /cranium-bevy-plugin/0.3.0/dependencies
- https://crates.io/api/v1/crates/bevy_thinker/0.4.0/dependencies
- https://crates.io/api/v1/crates/emergent
- https://crates.io/api/v1/crates/saddle-ai-fov (404)
- https://github.com/ashblue/fluid-behavior-tree
- https://github.com/ptrefall/fluid-hierarchical-task-network
- https://github.com/janhohenheim/bevy_bae
- https://github.com/hyranno/bevior_tree
- https://github.com/zkat/big-brain
- https://github.com/RJ/bevy_behave
- https://github.com/andriyDev/landmass
- https://github.com/janhohenheim/rerecast
- https://github.com/vleue/vleue_navigator
- https://github.com/Tavrin/navmesh-rs
- https://docs.rs/crate/bevy_orca/latest
- https://gitlab.com/veloren/veloren
- https://gitlab.com/veloren/veloren/-/tree/master/rtsim
- https://www.patentarcade.com/video-game-patent/US10926179/nemesis-characters-nemesis-forts-social-vendettas-and-followers-in-computer-games
- https://www.techradar.com/gaming/shadow-of-mordors-nemesis-system-will-remain-warner-bros-property-until-2036
- Web searches (Nemesis Rust, patent, perception/crowd, GOAP/Bevy games): results incl. https://github.com/ToroCraft/NemesisSystem,
  https://github.com/marti368/Nemesis, https://bommyknocker.bearblog.dev/nemesis/, https://julien-blanchon.github.io/saddle-ai-fov/,
  https://itch.io/jam/jamesis, https://www.nexusmods.com/cyberpunk2077/mods/36307, https://github.com/splashdust/dogoap
