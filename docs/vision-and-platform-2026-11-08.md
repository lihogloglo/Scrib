# Vision and platform: assessment and proposal (2026-11-08)

Status: advisory, docs only. Nothing was built or run (this machine can't compile). Inputs: README, CONTRIBUTING,
both CLAUDE.md files, ARCHITECTURE, WISHLIST, SYSTEMS, ROADMAP, hard-problems, `next-gen-pbr-gi-rt-plan.md`,
`research/coop/README.md`, `specs/net.md` + `archive/net-history.md`, greps of DECISIONS, the vendor README,
THIRD_PARTY_NOTICES, crate `lib.rs` headers, and web sources (end of file). Where this doc disagrees with a spec,
the spec stands until the owner decides. "Proposed doc changes" (§10) lists the edits; no other doc was touched.

<!-- agent-summary -->
> **Agent summary.** Owner goal: Scrib as a *platform* (multiplayer fantasy worlds that feel real, on lots of
> hardware, built to last), with Morrowind as the first content pack. Verdict: the engineering discipline is already
> platform-grade (adapter split, everything-is-a-mod, Bevy-free cores, net-ready rules that the code actually
> follows). The gaps are platform-level, not system-level: licence model, no CI or mechanical guardrails, the quality
> presets live in the game, no second content pack proving the split, a deep `bevy_pbr` fork, and a multiplayer
> design that nothing tests yet. Read §1 (philosophy), §8 (roadmap), §9 (owner decisions). All §9 items are now
> decided (1-3 by the owner, 4-10 delegated: `docs/platform-decisions-2026-11-08.md`).
<!-- /agent-summary -->

## 0. Where we stand (facts)

- About 293k lines of Rust in 106 crates (69 `sf_*`, 37 `mw_*`; ~207k in the framework), written in about four
  weeks (DECISIONS starts 2026-10-10), mostly by agents. Bevy 0.20 (2026-10-08), Rust 1.98.1, wgpu 30.
- The framework/game split holds in code: no `sf_*` manifest names an `mw_*` crate, and Morrowind appears in
  framework sources only in comments and test fixtures.
- Net-ready rules are followed in code: `sf_action::Command` is the only input, gameplay crates run on the fixed
  tick (`sf_physics_bevy`, `sf_action_bevy`, `sf_combat_bevy`, `sf_stats_bevy`), `sf_ids::PlayerId` is used in 34
  files, and there is no `rand::random` or `thread_rng` in gameplay code.
- Data contracts are already well chosen: `mod.toml` with `api_version`, RON/TOML records with field patches and
  provenance (`sf_records`), owner-keyed versioned save sections with migrations (`sf_save`), and a
  content-addressed bake (`sf_bake`).
- No CI: the repo has no `.github/` and no other runner. Every gate (`cargo test`, clippy, deny, hakari) depends on
  an agent remembering to run it.
- `superfargoth/vendor/bevy_pbr` carries about 10 of our own features (DDGI sampling, surface-weather view
  bindings, world-unit PCSS, mover shadow refresh, atmosphere LUT cache, airglow, ambient-specular toggle, ...) on
  top of 8 upstream PRs. This is the largest upgrade liability in the repo.

## 1. Philosophy

**Scrib is an open, moddable engine for persistent, systemic, shared open worlds. Every game on it is a content
pack, every content pack is a mod, and every machine is a peer.**

### Principles

1. **Content is data; games are packs.** A game is records + assets + scripts + rules plugins loaded through
   `sf_vfs`/`sf_records`/`sf_mods`. The engine never names a game (exists: hard rule 1).
2. **The world is the simulation; the picture is a view of it.** Gameplay state is plain serde data on a fixed
   tick. Rendering, animation, audio and UI only read it. This one rule gives co-op, saves, replays, headless
   servers and hardware scaling together (exists as the net-ready rules; promote it to *the* architectural rule).
3. **Multiplayer is the default topology.** Single-player is a host with no guests, on the same code path (net.md).
4. **Scale by tiers, not forks.** One renderer with data-driven scalability groups, from a handheld up to path
   tracing. A feature that can't scale down has a fallback, or it stays opt-in.
5. **Open formats at the edges, our formats only in the middle.** Authoring and exchange use glTF, KTX2, Ogg/Opus,
   Fluent, RON/TOML. Our own binaries (`.sfmesh`, `.sfsave`, the bake store) are either regenerable or versioned
   with migrations.
6. **Thin over Bevy, not around it.** Logic stays in Bevy-free cores; Bevy plugins stay thin; fixes go upstream.
7. **Industry standard, measured.** Name the canonical technique, measure per tier, no kludges (exists).
8. **Guardrails over goodwill.** With agents writing most of the code, any rule that matters is enforced by CI, a
   lint or a test, not by a doc paragraph.

### Non-goals (proposed, for the owner to confirm)

- An MMO (thousands of players, sharded persistence). We don't rule out dedicated servers for tens of players (§4).
- A general-purpose engine for every genre. Scrib targets 3D open worlds with characters. 2D, RTS and racing are
  not goals.
- Bit-exact determinism or lockstep (owner, 2026-10-19).
- Native code mods (dylibs) (ARCHITECTURE §6.1, correct).
- Our own editor before Bevy ships one (§3.2).
- Consoles, unless the licence decision (§2.3) allows them.

## 2. Tensions

### 2.1 State-of-the-art visuals vs lots of hardware

The current target is one machine (RTX 4060 laptop, 1440p + DLSS Quality, ≤ 11 ms). Every reference number,
gate and agent instruction points at it. Risks:

- **DLSS is the default upscaler**, so AMD/Intel GPUs, Steam Deck and Steam Machine (RDNA 3, 28 CU, 8 GB) fall back
  to TAA at a higher internal resolution. They'd miss the budget the gates were tuned for.
- **Raster GI needs ray queries** (gi.md "Tiers"). Without them, Low gets "off", which is a lighting *cliff*: the
  same interior looks like a different game. The no-RT fallback is named (cone tracing, Clockwork "light return")
  but not planned.
- **H1 is a render-thread (CPU) problem.** That's the axis that hurts handhelds most (Steam Deck: 4 Zen 2 cores).
- The vendored-shader approach ties visuals to Bevy internals (§6.1).

Resolution: keep "looks as good as possible" for the top tier, and add a **floor tier with its own gate** (§5).
The tie-breaker in CONTRIBUTING should read "fast on the floor tier, beautiful on the top tier".

### 2.2 Morrowind fidelity vs a general platform

The split is right, and §1b (modern first, classic behind seams) already keeps Morrowind's 2002 rules out of the
framework. Remaining pulls toward Morrowind:

- Some framework vocabulary comes from TES (worldspaces + door links, placements, the `Player` script context,
  dialogue as a condition-filtered INFO database in `sf_rpg`). These are fine defaults, but they are genre kits,
  not engine core (§3.1).
- **No second consumer exists.** The litmus test ("could another game plug in with only an adapter?") is checked by
  reading, never by running. Until a second pack runs, Morrowind-shaped assumptions leak in silently.
- The fidelity work (OpenMW ports, NIF quirks, MWScript) is the long tail. It shouldn't set the platform's pace.

### 2.3 GPL-3.0 vs platform adoption (the decision with the longest reach)

**What is true today.**

- The framework went from MIT OR Apache-2.0 to GPL-3.0-or-later on 2026-10-25 (DECISIONS L1514), so that GPL code
  (OpenMW, Clockwork, Delirium, openmw-rtx) could be reused directly. The owner was the sole author, so this was
  free to do.
- Third-party GPL code is now **inside framework crates**: Clockwork surface weather (`sf_surface`,
  `sf_surface_bevy`), openmw-rtx de-lighting (`sf_texture::delight`), OpenMW stat semantics (`sf_stats::ops`), and
  Clockwork PCSS in **vendored `bevy_pbr`** (`shadow_sampling.wesl`). The last one contradicts ARCHITECTURE §3
  ("Vendored Bevy patches stay MIT OR Apache-2.0 with no GPL-derived lines, so they can go upstream"): that patch
  can no longer go upstream as written.
- Default game builds link Delirium (GPL-3.0-only), so release binaries are GPL-3.0-only. Delirium is also flagged
  "accept the provenance risk, remove if challenged" (ROADMAP 3.2b).
- `LICENSE-EXCEPTION.md` covers GPU vendor runtimes only. **The planned Steam transport (net.md N8) would link the
  proprietary Steamworks SDK into the GPL binary, and nothing permits that today.** The same applies to EOS,
  Discord, FMOD/Wwise and console SDKs.

**What GPL-3.0 means for a platform**, read neutrally:

| Question | Under GPL-3.0-or-later |
|---|---|
| Can someone ship a commercial game on Scrib? | Yes. They must offer the corresponding source of the engine and their game code under GPL-3.0. Their art, audio, levels and text can stay proprietary if they are separate data, as Morrowind's own data is. |
| Do Luau mods have to be GPL? | Usually not for plain scripts. The FSF FAQ says code an interpreter runs is generally not bound by the interpreter's licence, but bindings to the host's facilities can make it "effectively linked". Our curated API is such a binding, so this is a grey zone. A clear statement in the API docs would settle it for modders. |
| Consoles? | Effectively no. Console SDKs are under NDA and the platform code can't be published (Godot, MIT-licensed, still can't ship official ports for exactly this reason). GPLv3 §6 also requires "Installation Information" for User Products, which a locked console can't provide. Only copyright holders can add an exception, and third-party GPL code in the tree means the owner alone can't grant one. |
| Steam, itch, GOG? | Fine to distribute on. Proprietary SDKs (Steamworks, achievements, overlay) need an added §7 permission or a separate process. |
| Bevy ecosystem contributors? | The Bevy ecosystem is MIT/Apache. Our crates can't be used in permissive projects, and our improvements can't go back except as separately written patches. This narrows the contributor pool. |
| What GPL buys | Direct reuse of the largest body of TES-engine knowledge (OpenMW and its forks). No proprietary forks of the engine. It matches the OpenMW/modding culture. Precedents: Veloren (Rust, GPL-3.0, multiplayer RPG), OpenMW, Luanti (engine LGPL-2.1, games licensed separately). |

**Options.**

| Option | Shape | Pros | Cons |
|---|---|---|---|
| A. Status quo | Everything GPL-3.0-or-later | Max reuse of GPL code. Simple. | No consoles, proprietary SDKs need permissions one by one, small engine adoption outside FOSS |
| B. Split | `sf_*` core MIT/Apache (or MPL-2.0); content packs GPL; third-party GPL code only in packs or quarantined `*_gpl` crates | Platform-friendly core. Morrowind keeps OpenMW reuse. Bevy-compatible | Clockwork/openmw-rtx/OpenMW-derived framework code (above) must move out or be clean-room rewritten. Discipline needed. |
| C. MPL-2.0 framework | File-level copyleft | Changes to our files stay open, and proprietary games can link (it works with Rust static linking, unlike LGPL's relinking clause) | Can't absorb GPL code into framework files. Same rewrite as B. |
| D. GPL + commercial dual licence | Owner sells exceptions | Funds the project, keeps openness | Needs sole ownership (CLA) and zero third-party GPL in the framework |
| E. GPL + broad §7 exception | e.g. "may link proprietary platform SDKs and middleware" | Opens Steam and middleware, maybe consoles | Covers only the owner's code, so third-party GPL parts block it (as with DLSS + Delirium today) |

**Assessment.** A suits a Morrowind fan engine. B suits the stated platform vision best. **Time matters more than
the choice:** every GPL port into `sf_*` makes B, C, D and E costlier, and the window where the owner is the sole
author closes with the first outside contributor. Recommendation: decide within weeks. Until then, adopt a
**quarantine rule**: third-party GPL code goes only into `bevyrrowind/` or into crates marked `*_gpl` (or listed as
GPL-derived), never into general framework crates or vendored Bevy. Also pick DCO or CLA before the first outside
PR.

### 2.4 Agent velocity vs maintainability

About 290k lines in a month is a big asset and a big liability. Signals: DECISIONS is 157 KB (1,629 lines), many
specs are over budget (ROADMAP 0.4), ARCHITECTURE has drifted (§10), there's no CI, and two GI plans disagree (§5.5).
Agents follow written rules unevenly. The fix is mechanical: CI, lints that encode rules, and fewer, truer docs.

### 2.5 Small co-op vs "a universe playable in multiplayer"

net.md targets 2-4 players with a listen server and no dedicated server. That's the right *first* product, and the
design is good (§4). The owner's wording ("vibrant fantasy universe playable in multiplayer") suggests more. The
cheap decisions that keep 8-32 players and dedicated servers possible should be taken now, even if they're built
later (§4.3).

## 3. Platform architecture

### 3.1 Is the `sf_*`/`mw_*` split the right shape?

Yes for the boundary. It's incomplete as a platform because "framework" mixes four layers. Proposed layering (a
documentation and dependency rule, no renames needed now):

```
L0 Engine core     sf_coords sf_ids sf_vfs sf_records sf_mods sf_save sf_script sf_bake sf_diag sf_console
                   sf_world sf_net(planned) sf_scalability(new)              (no genre, no rendering policy)
L1 World sim       sf_physics sf_terrain sf_water sf_weather sf_wind sf_atmos sf_surface sf_ai(planned)
                   sf_interaction(planned)                                     (systemic world, net-ready)
L2 Presentation    sf_*_bevy render/anim/audio/ui plugins, sf_gi, sf_lod, sf_sky, sf_ui*
L3 Genre kits      sf_rpg sf_stats sf_combat sf_action sf_character sf_rpg_ui sf_ui_inventory  (ARPG kit)
Packs              bevyrrowind (mw_*), future packs                          (depend on any layer)
```

Rules: a layer depends only on layers below it (L2 may read L1 state, never the reverse). L0/L1 cores never depend
on Bevy rendering. A new game may replace a whole L3 kit. CI checks this from `cargo metadata` (§8 Now-2).

### 3.2 What a NEW game needs that's missing

| Need | Today | Gap | Proposal |
|---|---|---|---|
| Native content path | `sf_world::NativeWorldSource`, glTF import in `sf_model`/`sf_anim`, RON records | Never exercised end to end without an adapter | **A small CC0 sample pack** (`samples/` or a third top-level part): one greybox valley, glTF props, native records, one Luau quest, weather, saves. It runs in CI. This is the litmus test made executable. |
| Asset formats | NIF/DDS via adapter; glTF/KTX2 win their VFS slot | No native scene/level format | glTF 2.0 (+ `extras`) for meshes and scenes now; Bevy `.bsn` for scenes when it ships (0.21 milestone). OpenUSD: watch only, it's too heavy for a solo project today. |
| Level authoring | none (Morrowind uses its ESM; test levels are RON) | No editor | Don't build one. Use **Blender as the level editor** (glTF export with components in `extras`; Bevy tools such as Skein exist), the in-game console + BRP inspector for live edits, and adopt Bevy's editor when it lands (0.20 says Feathers + BSN are its building blocks). Write a short "authoring a level" guide. |
| Record schemas | Rust types registered by adapters; native RON/TOML files | Modders get no schema or validation outside the game | Generate **JSON Schema** per record kind from `Reflect`, ship it with the API docs, so editors (VS Code, Taplo) validate RON/TOML. Same for `mod.toml`. |
| New record kinds without Rust | Not possible | A data-only pack can't add a kind | A generic "script-owned record kind" (schema declared in data, values as `Value` trees) for mods. Engine kinds stay Rust. |
| Scripting reach | Luau API v0.1, **parked since 2026-10-18** (L3) | Systems built since (stats, combat, saves, weather) have no script surface | **API-per-system rule**: each new framework system ships its minimal `@sf/<system>` module and `.d.luau` with it. Unpark L4 in slices. |
| Plugin ABI | Rust crates. No dylibs (correct) | No stability promise | Stability tiers per crate (`stable` / `unstable` / `internal`, in the crate doc header). The real ABI is data: mod manifest, record schemas, save sections, Lua API version, net protocol. Version those strictly. |
| Versioning | Every crate `0.1.0` via the workspace | Fine for now | Don't publish yet. When a second project depends on Scrib: publish `sf_*` crates with **major versions tied to Bevy's** (as the ecosystem does), plus a CHANGELOG per stable crate. |
| Docs and onboarding | Excellent for agents (budgets, summaries); none for humans | No "build a game on superfargoth" path | A `superfargoth/docs/guide/` (5 pages: concepts, first pack, records, scripts, levels) plus rustdoc for stable crates. Write it from the sample pack. |

### 3.3 Scripting options (for the record; Luau stays)

| Option | Sandbox | Speed | Modder reach | Fit |
|---|---|---|---|---|
| **Luau (mlua)**, chosen | Strong (no io/os, per-mod state, interrupts) | Good interpreter, optional codegen | Large (Roblox, Lua culture), type checker, LSP | **Keep** for content and mods |
| Lua 5.4 / LuaJIT | Must be built by us; LuaJIT unmaintained-ish | LuaJIT fastest | OpenMW/MWSE-Lua culture | No: Luau is the safer Lua |
| WASM (wasmtime, component model, WIT) | Strongest (capabilities by import) | Near native | Any language (Rust, C#, Zig, AssemblyScript) | **Later tier** for heavy mods (AI, procedural generation, sims). Wasmtime has LTS releases. Cost: binary size, compile time, API mirroring |
| Rhai, Mun, Rust hot reload | Weak to n/a | varies | small | No |

Decision to take now: keep the **API registry language-neutral** (it already generates `.d.luau`), so a WIT
generator can come later without redesigning the API. That's cheap now and expensive to retrofit.

## 4. Multiplayer-first

### 4.1 What's already decided, and is right

net.md and R12 are strong and match industry practice (net-history "Industry comparison"): a listen server, the
host simulating everything, prediction of the local character only (Overwatch model), snapshot interpolation, lag
compensation with "favour the defender", animation not replicated (Unreal's rule), stable ids instead of `Entity`,
absolute `WorldPos`, a content-fingerprint handshake (TES3MP/naia), the Enshrouded quest model, scripts only on the
host (avoids TES3MP's main failure), a context player for MWScript, and a rest vote. The eight net-ready rules
already bind every spec, and the code follows them (§0).

### 4.2 Critique

1. **Nothing tests it.** N1-N5 wait until after M6 and N6-N8 until after M7. Until then, net-readiness is checked
   only by spec audits. Agents add single-player shortcuts that look harmless, like the 2026-10-25 decision "the
   streaming camera stays the door traveller and script player". **Pull a headless loopback test forward**: two
   peers in one process (lightyear and replicon both offer in-memory transports), one capsule each, commands on the
   tick, run in CI. It costs one slice, and it turns rules 1-8 from prose into a failing test.
2. **The crate choice can be made now.** Lightyear 0.31 supports Bevy 0.20 (README table) and is built on
   replicon; the R12 lag worry has resolved for this release. Keep the N1 criteria, but run the spike with the
   loopback test.
3. **Save and replication are designed twice.** `sf_save` sections and replicated components are separate
   registries. Unreal marks properties `Replicated` and `SaveGame` on one declaration. Proposal: **one state
   registry** where each gameplay component declares `saved`, `replicated` (with visibility class) and `predicted`,
   and where the save section and the replication rule are generated from that one declaration. Snapshots for join,
   saves and debugging dumps then share one serializer and one schema version.
4. **Gameplay radius vs quality preset.** streaming.md: "ring radii scale with the quality preset", and actors
   simulate only inside R0. If R0 scales too, a Low-preset host simulates fewer NPCs than a High one, so gameplay
   depends on graphics settings. Rule: R0 (simulation and collision) is fixed per game; only visual rings scale.
   Verify against the code.
5. **Headless host.** net.md says remote regions on the host skip the animation graph. Take the next step and make
   "the simulation runs with no renderer" a CI target (a `--headless` app build without `bevy_render`). That is what
   a dedicated server is, and it also gives fast gameplay tests.
6. **Interest management at scale.** Worldspace + distance with per-cell overlay deltas is right for 4 players. For
   more, plan for replicon-style visibility per *cell group* (like Unreal's replication graph or Iris: spatial grid,
   dormancy, per-connection priority) and per-component rates. That's a later build, but the cell-keyed overlay
   already points the right way.
7. **Licence blocks the transport plan.** See §2.3 (Steamworks).

### 4.3 Decisions to take now (cheap now, a rewrite later)

| Decision | Recommended | Why now |
|---|---|---|
| Authority | Server-authoritative; the host is a server that also plays; a dedicated server is the same binary, headless | Keeps tens of players and persistent servers possible |
| Determinism | None beyond "same binary, same input, close result" (as decided) | Already right |
| Simulation/presentation split | Enforced by crate layering (L1 never depends on L2) + headless CI build | Rules erode without enforcement |
| Entity identity | `PlacementId` (content hash), host-allocated spawn ids, `PlayerKey`; **namespaced by world id** so several worlds or servers never collide | Persistent servers and world transfers need it |
| State registry | One declaration drives save + replication + prediction (§4.2.3) | Every new system otherwise invents its own |
| Wire/save schema | Versioned per component family, as save sections already are; the protocol version is in the handshake | Cross-version play stays a non-goal, but saves must survive |
| Player count | A setting with a default of 4, and no hard-coded arrays or masks sized 4 (`sf_ids::PlayerId(u8)` already allows 255; net.md calls it "0-3") | Cheap now |
| Mods in MP | Content fingerprint must match (decided); later, server-pushed mod sets (Factorio's model) | Shapes `sf_mods` profile APIs |

### 4.4 Prior art in one table

| Project | Model | Lesson for us |
|---|---|---|
| TES3MP (OpenMW) | Server + Lua server scripts; cell authority on clients | Client authority and client-side scripts caused most bugs. We avoid both. |
| Skyrim Together | Party-leader quest owner | Single-player quest scripts need one owner, or a context player (we have the latter) |
| Valheim | Zone owners simulate | Distributed authority spreads lag. Host-only is right at our scale. |
| Enshrouded | World progress + personal character | Our quest model |
| Unreal (replication graph, Iris) | Server-authoritative, property flags, relevancy graph, dormancy | One property declaration for replication and saves; spatial relevancy at scale |
| lightyear / replicon | Replicon replication + lightyear prediction, host-client mode | Our stack; on Bevy 0.20 now |
| Veloren (Rust, GPL) | Dedicated server, ECS, many players | Proof that Rust ECS + server authority scales past co-op |

## 5. Scalability across hardware

### 5.1 Reference tiers (proposed)

| Tier | Reference device | Target | Path |
|---|---|---|---|
| Floor ("Low") | Steam Deck (RDNA 2, 8 CU, 4c Zen 2), Intel Xe/Arc iGPU, Apple M1 | 800p-1080p, 30-40 fps, TAA/FSR-class upscaling | Raster, no RT, baked or cheap GI fallback, short rings, half-rate effects |
| Mainstream ("Medium") | Steam Machine / RX 7600 class (8 GB), RTX 3060 | 1080p-1440p, 60 fps, upscaled | Raster, optional RT probes at half rate |
| Reference ("High") | RTX 4060 laptop (today's target) | 1440p + DLSS Q, ≤ 11 ms | Raster + RT probe GI (gi.md Tier A) |
| Top ("Ultra") | RTX 4080+ | 1440p-4K, 60 fps | Solari path tracing + DLSS-RR |
| Not targeted | Phones, WebGL2 | none | Web/WebGPU only for viewers and demos, not the full game |

The floor is the gate that protects "runs on a lot of hardware". It needs its own benchmark JSON and a device in
the owner's reach (a Steam Deck is the obvious one; Valve keeps improving RADV ray-query performance, Mesa 26).
The **minimum API floor** is Vulkan 1.2 / DX12 / Metal 3 with compute and indirect draw. That's what Bevy's
GPU-driven path needs, and it rules out WebGL2 and old mobile.

### 5.2 Backend implications (Bevy/wgpu)

- Vulkan, DX12 and Metal are first-class. Solari now runs on macOS too (Bevy 0.20).
- **Upscalers:** DLSS is the only one wired. Add an `sf_upscale` seam with TAA, FSR (FidelityFX is MIT-licensed,
  so no plugin package needed), XeSS, and MetalFX later; pick per vendor at startup.
- WebGPU lacks ray queries and has tight binding limits, and mesh shaders are unsupported on web (0.20 notes).
  That's acceptable for a viewer tier only.
- **Ray tracing as a feature flag, never a requirement:** gi.md already gates on
  `SolariPlugins::required_wgpu_features()`. Keep that pattern for every RT feature.

### 5.3 Scalability settings architecture (missing at framework level)

Today the presets are `mw_game_core::exterior::Quality` (game crate), with per-system enums (`CloudPreset`,
`WaterQuality`, `UpscaleQuality`) and the `--off` switches. A second game would have to rebuild all of it. Proposal
(**`sf_scalability`**, modelled on Unreal's scalability groups):

- **Groups** (view distance, shadows, GI, reflections, effects, foliage, post, textures, clouds/weather, animation
  LOD, crowd). Each has levels 0-4, and each level is a set of **typed console variables** (`sf_console` already
  has typed variables that every plugin registers).
- **Presets are records** (RON, moddable, overridable per game), not Rust enums. The Options screen (ROADMAP 2.2b)
  renders the groups.
- **Auto-detect**: GPU features (RT, bindless), VRAM, CPU cores, plus an optional 10 s benchmark scene. The result
  is written to the profile.
- **Dynamic resolution** under a frame-time target, as the main runtime lever.
- **Simulation scalability stays separate** and isn't player-facing in MP (AI LOD, crowd tiers, physics substeps
  are server settings) (§4.2.4).

### 5.4 Perf budgets

Express budgets **per tier, CPU and GPU separately**. Today's single "≤ 11 ms" hides the render-thread bound (H1),
which matters most on the floor tier. Add a per-system budget table (render thread, main thread, GPU per pass) to
perf.md "Frame budget" for High and Low, and gate both.

### 5.5 How the GI/RT plan fits

- `superfargoth/docs/specs/gi.md` (dynamic ray-query probe volumes, no bake, Tier A on RT hardware, "off" without)
  and `docs/next-gen-pbr-gi-rt-plan.md` (baked `IrradianceVolume` per cell, SSR, SSGI, and "automated ReadyGate
  visual regression screenshots") disagree. The second also breaks the owner rule "no automated visual checks".
  Proposal: gi.md is the source of truth. Keep the next-gen plan's **Track 1 (PBR materials and tangents)**,
  which every tier needs. Mark Tracks 2-3 superseded where they conflict.
- **Fill the no-RT cliff**: one-time baked probe irradiance (the gi-2026-11-01 research's own first
  recommendation) or Clockwork-style fill as the Low/Medium fallback. The probe-volume data model can be shared, so
  only the *update* source differs (rays vs bake). That keeps the look consistent across tiers.
- Probe density, update budget and cascade count are natural scalability-group knobs.

## 6. Futureproofing

### 6.1 Bevy dependency

Bevy breaks its API every release, roughly every 4-5 months (0.18 in January, 0.19 in June, 0.20 in October 2026),
with no 1.0 date. Our exposure:

- **Low** in Bevy-free cores (most logic), which is the main insulation and should stay the rule.
- **High** in the vendored `bevy_pbr`/`bevy_render`/`bevy_core_pipeline`/`bevy_solari` forks. Each upgrade means
  rebasing ~10 of our own render features plus dropping merged PRs.

Mitigations, in order:

1. **Upstream the two generic hooks** that most own patches need: a view-level material extension point (global
   view bindings + a fragment hook for every `StandardMaterial`; surface weather, GI and ambient toggles hang off
   it), and a pluggable diffuse-GI provider. With those upstream, most vendored hunks become plain plugin code.
2. **Upgrade cadence policy:** adopt each Bevy release within ~4-6 weeks, never skip one, in a dedicated
   upgrade lane with a checklist (vendor rebase, crate lag table, perf gates, visual-check batch).
3. **No blanket adapter layer over Bevy.** It would cost more than the churn and fight the ECS. Use targeted seams
   only where we already have them: render features, assets, input.
4. Keep the `[patch]` list short: each entry has an owner and a "drop when".

### 6.2 Rust ecosystem risks

| Dependency | Risk | Exit |
|---|---|---|
| `mlua` + Luau | One maintainer for the bindings | Luau's C API is stable; bindings are replaceable |
| `rapier3d` | Dimforge's pace; we use it Bevy-free (good) | `sf_physics` hides it; avian is the fallback |
| `big_space` (rc pin + own patch) | Pre-release, patched | Our own `WorldPos`, so it can be replaced |
| `tes3` (git pin) | Single author | Game-only; fork if needed |
| `bevy_flair`, `bevy_enhanced_input`, lightyear | Bevy-lag | Already in the lag tracker; keep it current |

Add a "bus factor + exit" column to ARCHITECTURE §4.

### 6.3 Data format longevity

Good: RON/TOML records and manifests (human-readable), glTF/KTX2 intake, blake3-keyed regenerable bake products,
and versioned save sections with stepwise migrations. To add: a written **format registry** (one page listing
every on-disk format, its version, owner, spec and whether it's regenerable), golden files for every save-section
version in tests, and Fluent for all player-facing text.

### 6.4 AI-assisted workflow and solo maintainability

- **CI is the top maintainability item.** Run test, clippy, deny, hakari, a boundary check, and a doc-link check
  (`tools/doccheck.sh` exists) on every push. Agents can't skip what CI enforces.
- **Encode rules as lints.** `clippy.toml` `disallowed-methods`/`disallowed-types` for `rand::random`,
  `thread_rng`, `Time<Real>` in L0/L1 crates, and `Single<…Player…>`. A `cargo metadata` test for the layer
  rules (§3.1) and "no `mw_` in `sf_`".
- **Docs diet, second pass.** Fix ARCHITECTURE drift (§10). Split DECISIONS by year or topic once it passes
  ~200 KB. Keep the index generated.
- **Human-readable code review.** The owner reviews visuals in batches. Code gets cross-reviews per system
  (`docs/reviews/`). Schedule one per system per phase, starting with the L0 crates other projects would depend on.
- **Agents as a platform feature.** BRP + `bevy_brp_mcp` live inspection, the console and the generated
  `.d.luau` already make Scrib agent-operable. Document it as a supported workflow for pack authors.

## 7. "World feels real": the systemic layer

Weather is the model to copy: a pure function of (seed, clock, region, overrides), shared by every consumer
(clouds, wind, grass, surfaces, audio), local on every peer, persisted by overrides only. Other systemic layers
should follow the same shape: **state as data on the tick, rules as data, stimuli as events, host-authoritative,
saved through one registry.**

| Abstraction (framework, L1) | What it gives every game | Prior art | Status |
|---|---|---|---|
| **World state and persistence** | One persistent-change model (extends `PlacementOverlay`): moved/removed/spawned, per-entity state blobs, reset policies per cell (never, timed, on event) | Bethesda cell reset; Enshrouded world layer | Overlay built; policies missing |
| **Simulation LOD and off-screen sim** | Actors and systems at tiers: full (R0), coarse (schedule advanced analytically), statistical (population counts). An NPC is where their schedule says when you arrive. | S.T.A.L.K.E.R. A-Life online/offline; RDR2 | Missing; today actors are dormant outside R0 |
| **Interaction / smart objects** | Affordances on objects (sit, use, work, loot, climb), claims (net.md has them), animation hooks | Unreal Smart Objects; The Sims advertisements | `sf_interaction` planned, not specced |
| **Perception and stimuli** | Sight/sound/smell stimuli as events with tick + source; per-player detection | Unreal AI Perception | Missing |
| **AI decision layer** | Goals from content (the §1b seam) → utility scoring + HTN plans, budgeted per tier, host-only | Utility AI (Dave Mark), HTN (Horizon) | `sf_ai` planned; no spec ("AI" row in SYSTEMS) |
| **Schedules and calendar** | Daily/weekly routines as records, tied to the world clock, readable by scripts | Oblivion Radiant AI (and its lessons) | Missing |
| **Material and element rules** | Entity properties (wet, burning, frozen, conductive) + interaction rules as data, fed by `sf_surface` and weather | BotW "chemistry engine" (GDC 2017, "Breaking Conventions with BotW") | Surface state exists for rendering only |
| **Economy and ecology kits** | Optional L3 kits: resource stocks and flows per settlement, spawn ecology per region | Mount & Blade, Kenshi | Later |
| **Memory and relationships** | Per-NPC memory of players (the parked "Nemesis-like" idea) | Shadow of Mordor Nemesis | Parked idea; needs the state registry |

Order: persistence policies → schedules + sim LOD → interaction → perception → AI decisions. Each comes with its
Luau module (API-per-system) and its net-ready "Co-op" section. Lesson from Radiant AI: systemic *but bounded*.
Authors need knobs to keep emergent behaviour from breaking quests.

## 8. Roadmap of strategic recommendations

### Now (next 4-6 weeks; mostly decisions and guardrails, fits around Phases 1-3)

0. **Delirium out of the default build and the workspace** (decision 8): first, because its pinned rev can't be
   fetched and a fresh clone (CI included) can't build the default game. Deaths and knockdowns back on `sf_ragdoll`.
1. **Licence: decided** (GPL stays; quarantine dropped). Left: DCO in CONTRIBUTING plus a CI sign-off check
   (decision 1b). Steam is decided as a separate helper process (decision 9), so nothing to do before N8.
2. **CI with mechanical guardrails** (§6.4): GitHub Actions running the four existing gates, plus the boundary and
   layer check (L0/L1 never depend on L3, decision 4), plus `disallowed-methods` for the net-ready rules. One slice,
   and every later slice benefits.
3. **Sample pack S0 + loopback co-op test** (§4.2.1, decision 5): `samples/greybox` with an `sf_*`-only headless app
   in CI, then two peers in one process on lightyear 0.31 on top of it. It keeps the framework split and
   net-readiness honest while gameplay is being written, and it seeds the dedicated build.
3b. **Format registry** (`superfargoth/docs/formats.md`, decision 10): one page listing every on-disk and wire
   format, its version and trigger; golden files for every save-section version.
4. **`sf_scalability` + tier matrix** (§5): move presets into the framework as records + console variables. Add the
   floor tier (Steam Deck class) with its own benchmark gate. Add an FSR/TAA path beside DLSS.
5. **Adopt this philosophy and fix doc drift** (§1, §10): principles and non-goals into ARCHITECTURE §1-§2, layers
   into §5, and reconcile the two GI plans.

### Next (after Phase 2 done, alongside Phase 3; ~2-4 months)

6. **Bevy 0.21 upgrade lane** (decision 6): start at the 0.21 RC, ship within 6 weeks of release; move the vendored
   Bevy crates to a git fork branch on that rebase; one upstream slice for the view material hook (then GI provider).
7. **Sample pack S1 after the vertical slice** (§3.2, decision 5): the rendered CC0 greybox valley, Blender → glTF
   levels, JSON Schemas for records and manifests, the human guide written from it.
8. **One state registry** for save + replication + prediction (§4.2.3), before more systems add sections.
9. **Scripting stays parked; systems are script-ready** (decision 7): each spec gets a "Script surface" (events,
   commands, formula structs as plain data). Unpark on the first of: S1's quest, Phase 2.7 formulas built, mod API
   1.0 needed; then `@sf/records`, `@sf/rules`, events, from a JSON API dump.
9b. **Clean-room active ragdoll spec** for `sf_ragdoll` (decision 8), research first, never reading Delirium code.
10. **Specs for `sf_ai`, `sf_interaction`, persistence policies, sim LOD** (§7), research first (rule 1).
11. **No-RT GI fallback** sharing the probe data model (§5.5).

### Later (after M6/M7)

12. Co-op N2-N8 plus the headless dedicated-server build (owner answer 2: no ceiling). N8 Steam = the separate
    `scrib-platform-steam` helper over `sf_platform_ipc` (decision 9); the first Steam release is SDK-free.
13. Publish `sf_*` crates only once an outside project depends on them: `0.N` per Bevy minor with a compatibility
    table, no promise before Bevy 1.0 (decision 10). Saves are promised from the first public build; records and the
    Luau API from mod API 1.0.
14. WASM mod tier (wasmtime component model) after Luau API 1.0, generated from the same API dump (decision 7).
15. Editor: adopt Bevy's when it ships; until then, Blender + in-game tools.
16. Viewer/demo builds on WebGPU; console ports only if the licence allows.

## 9. Owner decisions needed

1. **Licence model**: A (status quo GPL), B (permissive core + GPL packs), C (MPL core), D (dual licence), or E (GPL
   + broad exception). Plus the quarantine rule meanwhile, and DCO or CLA.
2. **Multiplayer ceiling**: 2-4 co-op only, or keep 8-32 players and dedicated servers possible (affects
   §4.3: headless build, id namespaces, interest management)?
3. **Hardware floor**: is Steam Deck class (or iGPU, or Apple M1) a supported tier with a gate? Web and mobile as
   non-goals?
4. **Platform scope**: "open-world ARPG framework" (today's wording) or "engine for systemic open worlds" with the
   ARPG as one kit (§3.1)?
5. **Second content pack timing**: a small CC0 sample now (recommended), or after the Morrowind vertical slice?
6. **Bevy policy**: track every release within N weeks, and spend effort on upstream hooks?
7. **Scripting**: unpark Luau under the API-per-system rule? Keep the registry language-neutral for a later WASM
   tier?
8. **Delirium as a default feature**: keep it (GPL-3.0-only binaries, provenance risk), or make it opt-in?
9. **Steamworks**: extend `LICENSE-EXCEPTION.md` (only possible for our own code, see 1) or run Steam in a separate
   process?
10. **Stability promise**: when do `sf_*` APIs and data formats start carrying compatibility guarantees?

### Owner answers (2026-11-08)

- **1 Licence:** stay GPL-3.0-or-later (option A). Owner's view: with AI-assisted code, licence walls matter less
  over time; the GPL is not a reason to change course. The quarantine rule is dropped; provenance notices still apply.
- **2 Multiplayer:** no ceiling. Design for server authority, a headless/dedicated build, id namespaces and
  interest management (§4.3); co-op 2-4 is the first milestone, not the limit. CONTRIBUTING's "small co-op" wording
  needs updating.
- **3 Hardware:** Steam Deck is a supported floor tier with its own gate. Web (WebGPU) is a "beefy machine" bonus,
  not a target. Mobile is a non-goal.
- **Seamless interiors:** an option (per door / per pack setting), not mandatory.
- **AI:** the `sf_ai` spec comes before the seamless NPC work; research first (`superfargoth/docs/research/npc-ai-2026-11-08.md`).
- **4 Scope (delegated to agent):** an engine for systemic, persistent 3D open worlds with characters; the ARPG is
  the first L3 kit; L0/L1 never depend on L3 (CI check). High confidence.
- **5 Sample pack (delegated to agent):** S0 now, headless and minimal (`samples/greybox`, an `sf_*`-only app in CI,
  host of the loopback co-op test); S1 rendered CC0 valley after the vertical slice. Medium-high.
- **6 Bevy (delegated to agent):** adopt every minor, never skip; lane from the RC, ship ≤ 6 weeks (8 max); vendored
  Bevy crates move to a git fork branch; ≥ 1 upstream slice per cycle (material hook, then GI provider). High/medium.
- **7 Scripting (delegated to agent):** Luau stays parked (owner rule); specs get a "Script surface"; on unpark, a JSON
  API dump generates `.d.luau`/docs/WIT; WASM after Luau 1.0; gameplay scripts host-only in MP. Medium-high.
- **8 Delirium (delegated to agent):** out of the default build and the workspace now: the pinned rev can't be fetched
  and upstream says it was rewritten from GTA IV's program code; `sf_ragdoll` default, clean-room active ragdoll later. High.
- **9 Steam (delegated to agent):** SDK-free Steam release first; N8 via a separate permissive helper process over a
  generic IPC; no LICENSE-EXCEPTION extension (void with third-party GPL in the binary). High (legal line medium).
- **10 Stability (delegated to agent):** promises go to data, not Rust: saves from the first public build, records/
  `mod.toml`/Luau API from mod API 1.0; `sf_*` crates `0.N`, unpromised before Bevy 1.0; a format registry. High.
- **1b DCO/CLA (delegated to agent):** DCO with inbound = outbound, plus consent to future §7 permissions. High.
- Full reasoning, evidence and ready-to-paste edits: `docs/platform-decisions-2026-11-08.md`.

## 10. Proposed doc changes

| Doc | Change |
|---|---|
| `superfargoth/docs/ARCHITECTURE.md` §1 | Add the philosophy and non-goals (§1 here) above the Morrowind vision; keep "Morrowind if Bethesda made it in 2026" as the first pack's vision |
| ARCHITECTURE §2 principle 2 | Stale: "the framework lives in its own Cargo workspace and repo". Now one workspace: "the boundary is enforced by CI (`cargo metadata` check)" |
| ARCHITECTURE §3 | The claim "vendored Bevy patches ... no GPL-derived lines" is false since the Clockwork PCSS hunk. Fix it (GPL-derived H2 hunk, fork branch) and record the licence decision (GPL stays, no quarantine) and the Steam helper-process rule (decisions 6, 9) |
| ARCHITECTURE §4 | Refresh the lag tracker (lightyear 0.31 on 0.20) and add "bus factor / exit" |
| ARCHITECTURE §5 | Drift: `sf_foliage`, `sf_interaction`, `sf_ai`, `sf_viewer`, `sf_devui`, `sf_ui_lua` don't exist, while `sf_surface`, `sf_gi`, `sf_combat`, `sf_stats`, `sf_action`, `sf_ids` aren't listed. Replace with the L0-L3 layering table (§3.1) generated or checked against `superfargoth/crates/` |
| ARCHITECTURE §6.1 | "Possibly WASM later" → "WASM tier after Luau API 1.0, from the same API dump"; host-only gameplay scripts in MP (decision 7) |
| ARCHITECTURE new § | Scalability (§5.3) and the tier matrix (§5.1) |
| `superfargoth/docs/DECISIONS.md` | Append one dated bullet per owner decision in §9 when taken; nothing before |
| `bevyrrowind/docs/ROADMAP.md` | A short "Platform" lane: P0 Delirium out of default + workspace, P1 CI + DCO + layer check, P2 sample pack S0 + loopback co-op test, P3 `sf_scalability` + floor gate, P4 format registry; 3.2/3.2b Delirium lines (decisions 5, 8, 10) |
| `CONTRIBUTING.md` | Tie-breaker per tier (§2.1); "Net-ready" points to the lints once they exist; ~~licence quarantine rule~~ (dropped); scope wording (decision 4); "Script-ready" rule (7); DCO + no game-derived code (1b, 8); frame target becomes the tier matrix |
| `superfargoth/docs/specs/net.md` | Non-goals: reword "dedicated servers" per decision 2; player count not fixed at 4; one state registry; R0 not scaled by preset; Steam via the separate helper process under "Topology and transport" (decision 9) |
| `superfargoth/docs/specs/streaming.md` | "Ring radii scale with the quality preset": exclude R0 (simulation) |
| `superfargoth/docs/specs/gi.md` + `docs/next-gen-pbr-gi-rt-plan.md` | gi.md is canonical. Mark next-gen Tracks 2-3 superseded where they conflict (baked volumes vs dynamic probes, automated screenshot checks); add the no-RT fallback tier |
| `superfargoth/docs/specs/scripting.md` | ~~The API-per-system rule~~ (superseded by decision 7): deprecation window, JSON API dump as the registry's truth on unpark, the unpark trigger, host-only gameplay scripts in MP. Exact text in `platform-decisions-2026-11-08.md` |
| `docs/SYSTEMS.md` | Rows for Scalability, Interaction, Persistence/sim LOD; the AI row points to the planned spec |
| `superfargoth/vendor/README.md` | Mark the GPL-derived hunk (H2); the move to a git fork branch; the upgrade cadence (decision 6). Exact text in `platform-decisions-2026-11-08.md` |
| **Decisions 4-10 (2026-11-08)** | Ready-to-paste old→new text for DECISIONS (8 bullets), ARCHITECTURE §1/§3/§4/§6.1, CONTRIBUTING (scope wording, net-ready, script-ready, licence + DCO), ROADMAP (3.2, 3.2b, decision 5 + P-lane, co-op line), scripting.md, net.md (Steam helper), `LICENSE-EXCEPTION.md` (Delirium paragraph) and the vendor README: end of `docs/platform-decisions-2026-11-08.md`. Not applied |
| `LICENSE-EXCEPTION.md` | Delirium scope paragraph → third-party GPL code + "platform SDKs never enter the game process" (decisions 8, 9). No Steamworks extension |
| new `superfargoth/docs/formats.md` | Format registry (decision 10): every on-disk/wire format, version, owner, spec, regenerable, guarantee trigger |

## Sources

- Bevy 0.20 release notes (Solari on macOS, BSN, Feathers as editor groundwork, WESL, no 1.0 statement):
  https://bevy.org/news/bevy-0-20/
- Bevy 0.19 (June 2026) and 0.18 (January 2026) releases:
  https://alternativeto.net/news/2026/6/bevy-0-19-brings-next-gen-scenes-faster-rendering-contact-shadows-and-new-feathers-widgets/ ,
  https://alternativeto.net/news/2026/1/bevy-0-18-brings-atmospheric-occlusion-pbr-shading-solari-improvements-and-font-variations
- lightyear README (0.31 ↔ Bevy 0.20, built on replicon, host-client mode): https://github.com/cBournhonesque/lightyear
- bevy_replicon: https://github.com/simgine/bevy_replicon (versions per `superfargoth/docs/research/coop/README.md`)
- TES3MP: https://github.com/TES3MP/TES3MP ; OpenMW 0.50 (Nov 2025, Lua expansion, single-player):
  https://alternativeto.net/news/2025/11/openmw-0-50-0-updates-gamepad-controls-expands-lua-scripting-and-adds-polish-support
- Unreal Iris replication: https://dev.epicgames.com/documentation/en-us/unreal-engine/iris-replication-system-in-unreal-engine ;
  Unreal scalability: https://dev.epicgames.com/documentation/en-us/unreal-engine/scalability-reference-for-unreal-engine ;
  Smart Objects: https://dev.epicgames.com/documentation/en-us/unreal-engine/smart-objects-in-unreal-engine
- Godot on console ports (NDA, can't open-source platform code): https://godotengine.org/article/about-official-console-ports/
- GPLv3 text (§6 Installation Information, §7 additional permissions): https://www.gnu.org/licenses/gpl-3.0.html ;
  FSF FAQ, interpreters and bindings: https://www.gnu.org/licenses/gpl-faq.html#IfInterpreterIsGPL
- MPL-2.0 FAQ (file-level copyleft, combining with proprietary code): https://www.mozilla.org/en-US/MPL/2.0/FAQ/
- Veloren (Rust, GPL-3.0, multiplayer): https://gitlab.com/veloren/veloren ; Luanti (engine LGPL-2.1):
  https://github.com/luanti-org/luanti
- Steam Machine specs (RDNA 3, 28 CU, 8 GB): https://www.pcgamer.com/hardware/gaming-pcs/steam-machine-specs-availability
- RADV ray-tracing improvements (Mesa 26.0, Steam Deck):
  https://www.gamingonlinux.com/2026/02/mesa-26-0-is-out-bringing-ray-tracing-performance-improvements-for-amd-radv ,
  https://frvr.com/blog/valve-helps-amd-gpus-gain-significant-ray-tracing-and-vulkan-improvements-on-steam-deck-and-more/
- Wasmtime stability and LTS: https://docs.wasmtime.dev/stability-release.html ; WASM 2026 status (secondary):
  https://devnewsletter.com/p/state-of-webassembly-2026/
- Luau: https://luau.org ; clippy `disallowed_methods`: https://rust-lang.github.io/rust-clippy/master/index.html#disallowed_methods
- Comparable engines: Godot GDExtension https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/what_is_gdextension.html ,
  O3DE https://o3de.org , Fyrox https://fyrox.rs , Stride https://stride3d.net (all permissive and all ship editors,
  the gap Scrib fills with Blender + Bevy's coming editor)
