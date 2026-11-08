# Platform decisions (2026-11-08, delegated to agent)

Status: decided by an agent, as the owner delegated: "decide based on the industry-standard way, and really think this
through." It covers questions 4-10 of `docs/vision-and-platform-2026-11-08.md` §9, plus DCO or CLA from question 1.
Docs only: nothing was built or run, and no shared doc was edited. The shared-doc edits are at the end, ready to paste.
Binding inputs are the owner answers of 2026-11-08 (GPL-3.0-or-later stays, no multiplayer ceiling, Steam Deck floor,
web is a bonus, mobile is a non-goal, seamless interiors optional, `sf_ai` first), CONTRIBUTING, and the owner's
same-day rule "everything starts in Rust; Luau comes later" (DECISIONS 2026-11-08).

<!-- agent-summary -->
> **Agent summary.** 4: an engine for systemic 3D open worlds, with the ARPG as its first kit (high confidence).
> 5: a sample pack now, but headless and minimal (S0, in CI); the rendered CC0 greybox comes after the vertical slice
> (medium-high). 6: adopt every Bevy minor within 6 weeks, starting at the RC; move the vendored crates to a git fork
> branch; upstream work goes to the two hooks (high / medium). 7: Luau stays parked; each system is "script-ready"
> instead (events, commands and formula structs); the API registry becomes a JSON API dump; mods run on the server only
> (medium-high). 8: **OWNER OVERRULED (2026-11-08): Delirium stays a default, core part of the game.** Remaining
> action: re-pin it, because the pinned commit can't be fetched any more (see Q8). 9: Steam runs through a separate, permissively licensed
> helper process; LICENSE-EXCEPTION is not extended (high). 10: guarantees start with the data contracts, not the Rust
> API: saves at the first public build, the mod API at Luau 1.0 (high). DCO, not a CLA (high).
<!-- /agent-summary -->

## 0. How the decisions interact

Seven questions decided one at a time would contradict each other. Three couplings shape everything below:

1. **The platform contract is data plus scripts, never Rust.** Roblox, Factorio, Luanti and Minecraft data packs
   promise stability on what modders touch (script API, data formats). Their engine internals churn freely. Bevy
   breaks its API every 3.5-5 months, and we carry a `bevy_pbr` fork, so a Rust API promise before Bevy 1.0 would be
   a promise we can't keep. So (Q10) the guarantees go to saves, records, `mod.toml`, the Luau API and the net
   handshake. The Rust crates stay `0.x` and are unpromised. This frees the Bevy policy (Q6) to optimise for our own
   upgrade cost only. It also makes the Luau API (Q7) the thing to get right before 1.0, not the thing to ship early.
2. **Licence × Steam × Delirium.** With GPL kept and the quarantine rule dropped, third-party GPL code (OpenMW ports in
   `mw_*`, Clockwork in `sf_surface` and vendored `bevy_pbr`, openmw-rtx, skyrim-delirium) is in every game binary.
   Only its authors could permit linking it with Steamworks. That forces the helper process (Q9). Steam's distribution
   agreement also makes us warrant that we hold all necessary rights. We can't do that for code that its own author
   says was "rewritten in Rust from the game's own program code" (Q8). The owner accepts that risk for Delirium
   (2026-10-22 stance, confirmed 2026-11-08); it bears on a Steam release, not on development.
3. **Sample pack × layering × multiplayer × stability.** The headless sample pack (Q5) is one artifact with four
   uses. It is the executable litmus test for the platform scope (Q4) and the seed of the headless and dedicated
   build (owner answer 2). It holds the golden files for the data contracts (Q10). And it is where the "script-ready"
   surfaces (Q7) first get a consumer that isn't Morrowind.

## Q4. Platform scope

**Question.** Is Scrib an "open-world ARPG framework" (today's wording), or an "engine for systemic open worlds" with
the ARPG as one kit?

**Options.** (a) An ARPG framework: genre baked into the core. (b) An engine for systemic 3D open worlds with
characters, where the ARPG is the first first-party L3 kit. (c) A general-purpose engine (Godot, Unity, O3DE, Fyrox).

**Evidence.**
- *General engines need teams and editors.* Godot, O3DE, Fyrox and Stride all ship editors and broad genre coverage.
  O3DE has a foundation behind it and still struggles for adoption. A solo-plus-agents project can't compete there.
  Bevy itself is the general engine in our stack, so (c) would duplicate the layer we build on.
- *Genre-shaped engines that widened along the content axis did well.* Luanti (Minetest) went from a game to "a
  voxel game engine". Minetest Game stopped being the default, and many games now run on it. The engine stayed
  voxel-shaped, though: it widened in content, not in genre. Bethesda's Creation Engine is in practice an "open-world
  RPG engine" whose kits (dialogue, quests, radiant AI) shipped Oblivion, Fallout and Starfield. RPG Maker and Ren'Py
  win because they are narrow.
- *Veloren* is a game with an ECS core and no engine ambitions. It proves the technical shape (Rust, server
  authority, many players), not a platform.
- *Our code already has shape (b).* The vision doc §3.1 layering (L0 core, L1 world sim, L2 presentation, L3 genre
  kits) matches the crates: `sf_rpg`, `sf_combat`, `sf_stats` and `sf_action` are kits, while `sf_weather`,
  `sf_surface`, `sf_physics` and the planned `sf_ai` are systemic world layers.

**Weighing.** The owner's goal ("many games, e.g. a vibrant multiplayer fantasy universe") isn't served by a
genre-locked core. Option (b) costs almost nothing today: it's a dependency rule and a wording change. It also stops
ARPG assumptions (combat stats in the save core, a `Player` script context in L0) from leaking down. The risk is scope
creep, so (b) needs explicit non-goals: no 2D, no RTS, no racing, no generic editor, and no second kit until a second
game needs one.

**Decision.** **(b): "an engine for systemic, persistent 3D open worlds with characters".** The ARPG kit is the first
first-party kit, and Morrowind is the first pack. L0/L1 must not depend on L3, checked by `cargo metadata` in CI. The
non-goals from §1 of the vision doc stand ("our own editor before Bevy ships one" included).
**Confidence: high.**

**Revisit if** a second game wants a non-character genre (then refuse or fork), or if keeping L1 free of L3 costs
more than one adapter seam per system (that would mean the "systemic world" layer is really ARPG-shaped).

## Q5. Second content pack

**Question.** A small CC0 sample pack now, or after the Morrowind vertical slice? If now, what is its minimal scope?

**Options.** (a) A full rendered sample game now. (b) Nothing until the vertical slice (Phase 2) is done. (c) A
headless, minimal pack now, run in CI, with the rendered sample after the slice.

**Evidence.**
- Engines prove their split with a second consumer early. Bevy's own `examples/` run in CI, and Godot ships demo
  projects. Unreal's Lyra sample was built *as* the reference for its modular gameplay framework. Luanti only became
  an engine once games other than Minetest Game ran on it.
- Godotwind (the owner's previous project) checked its framework boundary with ratchet tests and still carried
  "known debt" bridges (ARCHITECTURE §2). A boundary that is only read, never run, drifts.
- The framework already has the parts: `sf_level` (greybox levels), `sf_world::NativeWorldSource`, glTF intake in
  `sf_model`, RON records and `mod.toml` in `sf_records`/`sf_mods`, and `sf_save`. What's missing is a *game
  composition* outside `mw_*`. Today the app shell, presets and boot path (`mw_boot_bevy`, `mw_game_core`) are
  game-side. A second app is the real test.
- Cost now: the vertical slice is the owner's priority, and art for a rendered pack competes with it. Headless needs
  no art.

**Weighing.** (b) leaves the platform claim untested through Phases 2-3, exactly while most framework APIs get
written. (a) competes with the slice for the owner's eyes (visual checks), which are the scarcest resource. (c) gives
the litmus test, the headless build (owner answer 2: dedicated servers) and the golden files (Q10) without any
visual-check cost. The previous recommendation ("CC0 sample pack in CI now") was right about timing and wrong about
size: a rendered greybox valley with weather and a Luau quest is a second game, not a test.

**Decision.** **(c).** Two stages:
- **S0 now (Phase 1-2, ~2-3 slices):** `samples/greybox/`, a pack with `mod.toml`, native RON records (one actor
  kind, one item kind) and one `sf_level` greybox level in its native file format. Add an `sf_*`-only headless app
  (`samples/apps/greybox_headless`, no `mw_*` dependency, no `bevy_render`) that boots the pack through
  `sf_vfs`/`sf_records`/`sf_mods`, streams the level, spawns two capsule characters driven by `sf_action::Command`s
  on the fixed tick, applies one stat change, saves, loads, and asserts the state round-trips. It runs in CI. It is
  also the host for the loopback co-op test (vision §8 Now-3): two peers in one process, same pack.
- **S1 after the vertical slice:** the rendered CC0 greybox valley (glTF from Blender, CC0 assets such as Kenney,
  Quaternius, Poly Haven, ambientCG), weather, and one scripted quest once Luau is unparked (Q7). It becomes the
  basis of the human guide.

Pack data (records, levels) is CC0, so people can copy it as templates. Pack Rust code is GPL-3.0-or-later like the
rest. **Confidence: medium-high** (S0 is high; the S1 timing depends on Phase 2's end).

**Revisit if** S0 needs framework changes larger than one adapter seam each (fix the seams first: that's the point),
or if no outside game shows interest by the first public build (then S1 can wait for one).

## Q6. Bevy policy

**Question.** What upgrade cadence? How do we handle the vendored `bevy_pbr`/`bevy_render`/`bevy_core_pipeline`/
`bevy_solari` patches? How much effort goes into upstream PRs?

**Options (cadence).** (a) Track every minor within N weeks. (b) Skip releases and upgrade every second one. (c) Pin
until Bevy 1.0. (d) Track `main`.

**Options (patches).** (i) Keep copying registry folders and re-applying diffs (today). (ii) A git fork of
`bevyengine/bevy`, one branch per Bevy release, one commit per patch, `[patch.crates-io]` with `git`/`rev`. (iii) Move
our render features out of Bevy into our own render plugins.

**Evidence.**
- Bevy releases: 0.17 on 2025-09-30, 0.18 on 2026-01-13, 0.19 on 2026-06-19, 0.20 on 2026-10-08 (bevy.org/news).
  That's a 3.5-5 month cycle with release candidates first (our `big_space` pin is `0.14.0-rc.0` "for 0.20-rc"). The
  0.20 post says BSN syntax "should largely be nailed down" and that the editor is not here yet. No 1.0 date exists.
- The ecosystem ports fast: for 0.19, avian, tnua, hanabi and the input crates ported within about 2 weeks
  (ARCHITECTURE §4). Skipping a release doubles the migration: Bevy's migration guides are per release, and two
  stacked guides interact.
- Pinning (c) is what large Bevy users without a renderer fork do near shipping, but it freezes us out of the Bevy
  features we rely on (Solari, BSN, GPU-driven paths). Tiny Glade avoided the problem by writing its own renderer on
  Bevy's ECS. That's option (iii) taken to its end, and too heavy for us.
- Engine forks in industry: Unreal licensees keep their engine as a git branch and merge each upstream release.
  Cargo's own documented way to carry an unpublished fix is `[patch]` pointing at a git fork. Embark's Rust work
  used exactly that for long-lived crate forks. Copying folders (i) loses history, makes each hunk's provenance a
  comment convention, and makes "is this PR merged yet?" a manual diff.
- Upstream review bandwidth: our stack has PRs open since 2026-10 (#26149, #26161, #26235-26237, #26321). One
  (#26234) merged within days. Render PRs that add extension points take longer, so planning around fast merges is
  unsafe.
- Our own `bevy_pbr` hunks number about 10 (vendor README). One (H2 PCSS) is GPL-derived (Clockwork), so it can't go
  upstream as written. ARCHITECTURE §3 still claims the opposite.

**Weighing.** The patch stack is the upgrade cost, and it grows with every render feature. Cadence (a) keeps each
rebase small. (ii) turns the rebase into `git rebase --onto v0.21.0` with conflicts shown hunk by hunk, and each
commit is already an upstream PR candidate. (iii) is right for features that don't need to touch every
`StandardMaterial`: GI provider plumbing and surface weather would move out *once* Bevy has hooks, which is what the
upstream effort buys.

**Decision.**
1. **Cadence: adopt every Bevy minor, never skip one.** Start the upgrade lane at Bevy's first RC (rebase the fork
   branch, list lagging crates). Ship on the new minor **within 6 weeks of its release** (8 weeks at most, when a
   critical crate lags: then fork-patch that crate or wait). Take patch releases (0.x.y) within a week. Critical
   crates are the ones in our `[patch]` list plus `lightyear` (once N1 starts), `big_space`, `bevy_enhanced_input`
   and `bevy_flair`.
2. **Patches: option (ii).** A fork `scrib/bevy` with branch `scrib-v0.20`: upstream PR commits cherry-picked as-is,
   our own patches one commit each, with the vendor README's marker in the message. `[patch.crates-io]` takes
   `{ git = ..., rev = ... }` for the five crates. `superfargoth/vendor/` keeps only the non-Bevy crates
   (`big_space`, `dlss_wgpu`). Every own patch gets an "upstream route" line: a PR number, "needs hook X", or
   "GPL-derived: rewrite clean before upstreaming".
3. **Upstream effort:** at least one upstream slice per Bevy cycle, about 10% of render work. In priority order:
   (1) a view-level material hook (global view bindings plus a fragment extension point for every
   `StandardMaterial`), which surface weather, ambient specular and GI sampling would use; (2) a pluggable diffuse-GI
   provider on the raster path. Upstream patches are written fresh under MIT/Apache, never from the GPL-derived
   hunk. Where Bevy already has a working group or an open design for the same hook, we join it instead of opening a
   competing PR.
4. **Patch budget:** every upgrade report states own-patch hunks and lines before and after. A new own patch needs a
   spec line saying why a plugin can't do it (CONTRIBUTING rule 4 already prefers patches over injection; this adds a
   ceiling to watch).

**Confidence: high** on cadence; **medium** on the git-fork mechanics (they are standard Cargo practice, but a git
dependency on all of Bevy makes cold fetches heavier. Measure it on the first rebase).

**Revisit if** Bevy announces 1.0 or an LTS line (then pin to it), if one upgrade costs more than ~2 weeks of lane
time twice in a row (then move the heaviest features to option (iii) even without hooks), or if the two hooks merge
(then drop most own patches).

## Q7. Scripting

**Question.** Do we unpark Luau under the "each system ships its script API" rule? Do we keep the registry
language-neutral for a later WASM tier? How are mods sandboxed on multiplayer servers?

**Options.** (a) Unpark Luau now, with an `@sf/<system>` module per system (the previous recommendation). (b) Keep
Luau parked, and make each system "script-ready" (events, commands and formula inputs as plain serde data) with no
script code yet. (c) Skip Luau and go straight to WASM. (d) Make the registry an IDL source now and generate both.

**Evidence.**
- *Luau is the right language* (unchanged from the vision doc §3.3): Roblox's sandbox, a type checker and LSP, an
  open-source MIT runtime, and mlua bindings (we're on mlua 0.12 with `luau`). L1-L3 are built and verified.
- *Language-neutral registries in shipped engines.* Godot generates every GDExtension binding from one
  `extension_api.json` dump of the engine's reflection data. Roblox publishes a JSON API dump that its tooling
  (luau-lsp definitions, docs) is generated from. Both are a structured dump, not hand-written type text. Our
  `ApiModule::types()` returns a hand-written Luau declaration string, which is **not** language-neutral: a WIT or
  C# generator could not read it.
- *WASM tier.* Wasmtime has LTS releases (stability policy). WASI 0.3 (native async) shipped on 2026-06-11, and
  Wasmtime 46+ runs 0.3 components by default (wasi.dev). Most toolchains still target 0.2. Veloren already loads
  WASM plugins through wasmtime's component model from one WIT file (`plugin/wit/veloren.wit`, with separate
  `server-plugin` and `animation-plugin` worlds). It's a working precedent, but Veloren's plugin ecosystem stays
  small next to Lua-modded games. Modders pick the easy language.
- *Multiplayer mod trust models.* Minecraft servers: plugins run server-side only, and clients stay vanilla. Luanti:
  mods run on the server, clients download media, and client-side mods are limited by the server's
  `csm_restriction_flags`. Roblox: server scripts are authoritative, and client scripts are untrusted
  (FilteringEnabled). Factorio: the server pushes the mod set and every peer runs it (lockstep). Garry's Mod sends
  Lua to clients, the classic exploit and malware vector. TES3MP: server-side Lua.
- *The owner's latest call* (DECISIONS 2026-11-08): everything starts in Rust, Luau later, and no `@sf/rules` until
  the owner unparks Luau.

**Weighing.** (a) writes Luau modules against APIs that are still moving (stats, combat, saves, AI), so each one gets
rewritten, and it contradicts a same-day owner rule. (c) gives modders a harder language and more tooling for a
small project. Doing nothing risks APIs that can't be exposed later (Bevy `Entity` in events, closures in formulas).
(b) captures what matters for a later Luau or WASM tier at almost no cost: what a system *emits* and *accepts* as
plain data is exactly what both a Luau module and a WIT interface need. (d) is right, but only when Luau unparks:
building a schema system for a parked feature is waste.

**Decision.**
1. **Luau stays parked** (owner rule kept). It is replaced by a **script-ready rule**: each system's spec has a
   "Script surface" section listing its events (Bevy messages of plain serde data, stable ids, never `Entity`), the
   commands it accepts (messages), and its formula inputs (`fn(&In) -> Out` structs, already the rule-hooks shape).
   No Luau code and no hand-written `.d.luau`.
2. **When Luau unparks, the registry's source of truth becomes a structured API dump** (`sf_api.json`: modules,
   functions, types, `since`/`deprecated` versions), on the Godot `extension_api.json` / Roblox API dump pattern.
   `.d.luau`, the docs, the JSON Schemas (vision §3.2) and a later WIT world are generated from it.
   `ApiModule::types()` is replaced by structured metadata then, not now.
3. **Unpark trigger:** the first of (i) S1 of the sample pack needs its quest, (ii) the Phase 2.7 mechanics
   (formulas behind seams) are built, or (iii) the stability promise (Q10) needs mod API 1.0. Unparking starts with
   `@sf/records`, `@sf/rules` and events, in that order.
4. **WASM: later, from the same dump.** Not before Luau API 1.0. Its target is heavy server mods (AI, procedural
   generation, simulation), with wasmtime LTS and a component-model world per role (Veloren's split).
5. **Multiplayer sandbox:** gameplay scripts run **on the host or server only** (as net.md already says for MWScript).
   Clients get the server's content set (fingerprint-matched; later server-pushed and hash-checked, the Luanti and
   Factorio model). Client-side scripts are limited to a **presentation context** (UI, HUD, cosmetic effects) with
   no gameplay writes. Their only way to affect the world is `sf_action::Command`s the server validates (Roblox
   FilteringEnabled). The server's per-mod memory and CPU limits (scripting.md) are enforced and reported per mod. A
   server operator can set them, and a mod over budget is suspended, not the server. Mod-shipped bytecode stays
   forbidden.

**Confidence: medium-high** (the sandbox model is high; the unpark trigger is a judgement call).

**Revisit if** a modder community forms before Luau 1.0 (unpark earlier), if mlua's Luau build fails on the web
target (it compiles C++; check before promising scripting on web), or if WASI 0.3 tooling reaches parity in Rust, C#
and AssemblyScript (WASM could then come before Luau 1.0 for server mods).

## Q8. Delirium as a default feature

**Question.** GPL-3.0-only binaries are fine under the owner's licence choice. What about the remaining provenance
and maintenance risk?

**Options.** (a) Keep it default-on (today, owner 2026-11-05). (b) Make it opt-in and keep it in the workspace.
(c) Remove it from the workspace and keep it as a local, owner-only experiment, with the default path on
`sf_ragdoll` and a clean-room active ragdoll as the long-term answer (R12 option B).

**Evidence (checked 2026-11-08).**
- **The pinned commit no longer exists upstream.** `git fetch https://github.com/PVCPC/delirium-physics
  91ee2468820d434e123d4c09e89760ce38d165f5` fails with "upload-pack: not our ref", and the GitHub commit page
  returns 404. The remote now carries `main` (v0.7.0, `f4ff84d`), a `gta-free` branch and tags v0.2.5-v0.7.0: the
  history was rewritten. **A fresh clone, CI or any contributor can't build the default game.** Only machines with
  the old git cache can. Cargo generally needs a git source to resolve the workspace lock even with the
  feature off (confirm on a clean clone in the first CI run).
- **Upstream's own provenance statement changed.** The README now calls Delirium a "reverse-engineered port of
  Euphoria ... as it runs in GTA IV", "rewritten in Rust from the game's own program code and checked against the
  running game". It says an earlier claim that behaviours were written from scratch "was wrong". It also says the
  engine reads its numbers from the player's own GTA IV, and that neither the playroom nor the SDK runs without it.
  So upstream no longer offers a usable library for us, and the code we pinned is, by its author's account, derived
  from Rockstar and NaturalMotion's program code.
- **Precedent.** Take-Two's DMCA takedowns (and 2021 lawsuit) against the re3/reVC GTA decompilations. A GPL licence
  does not cure third-party copyright (R12 already said so).
- **Distribution.** The Steam Distribution Agreement makes the developer warrant they hold all necessary rights
  (Steamworks "Distributing Open Source"). We can't warrant that for code its own author says was rewritten from a
  commercial game's binary. `mw_delirium` also ports skyrim-delirium, which is downstream of the same work.
- **Maintenance.** One author, rewritten history, a 21-part 1.8 m humanoid only, and process-global state ("one
  character per process", `mw_delirium` docs). R12 called it "6 days old" when adopted.

**Weighing.** The owner's 2026-10-22 stance ("fan project: accept the provenance risk, remove if challenged") was
reasonable for a private experiment. Three facts since then change it: the platform goal (other games, Steam, outside
contributors), the build no longer being reproducible, and the author's own account of where the code came from. (b)
fixes nothing: the git source still breaks resolution, and "opt-in" still ships in any build someone makes. The
`RagdollTakeover` seam means the default path can fall back to `sf_ragdoll` (Rapier) without touching callers.

**Agent's proposed decision (c), OVERRULED by the owner 2026-11-08.** The agent proposed removing Delirium from the
default build and the workspace. That reversed explicit owner decisions (2026-10-22 "accept the provenance risk",
2026-10-30 "replaces the clip+limp ragdoll", 2026-11-05 "default feature") that were never delegated, so it is void.

**Owner decision (2026-11-08): (a). Delirium is part of the main game, on by default; not optional.** The
`RagdollTakeover` path, deaths and knockdowns on Delirium, and `sf_ragdoll` for beasts/corpses stay as they are.

**What remains from the evidence (build hygiene only):**
- **Re-pin.** Rev `91ee246` is gone from `PVCPC/delirium-physics` (and isn't in `YoucefNabil/skyrim-delirium`
  either). On the owner's workstation, move the pin to a ref that exists (a release tag such as `v0.7.0`, or the
  `gta-free` branch head if it matches what `mw_delirium` needs), rebuild, and test; check `mw_delirium`'s data path
  against the new version (skyrim-delirium ships its own character, no GTA IV data).
- **Protect against further history rewrites** (standard practice for git dependencies): fork the upstream repo
  under the project's GitHub account and pin to that fork, so a force-push upstream can't break the build again.

**Revisit if** upstream is challenged (the owner's 2026-10-22 rule: "remove if challenged"), or before a Steam
release (the rights warranty in Q9).

## Q9. Steamworks

**Question.** Do we extend `LICENSE-EXCEPTION.md` to cover Steamworks, or run Steam in a separate process?

**Options.** (a) Add Steamworks (and EOS, Galaxy, Discord, middleware) to the §7 permission. (b) A separate
helper process that links Steamworks and talks to the game over local IPC. (c) Ship on Steam without the SDK at all.
(d) Re-quarantine the third-party GPL code so (a) becomes valid.

**Evidence.**
- Valve: "The Steamworks SDK is incompatible with some open source licenses". Copyleft "will be problematic when
  combining code with the Steamworks SDK". GPL apps appear only when "the author of the code ... has given the
  permission". If incompatible third-party code is present, "YOU MUST NOT DISTRIBUTE YOUR APPLICATION VIA STEAM".
- GPLv3 §7 additional permissions can only be granted by copyright holders. The FSF FAQ (`#GPLIncompatibleLibs`)
  says that when a program includes others' GPL code you can't add the exception without their agreement. Our game
  binary includes OpenMW-derived content mechanics, Clockwork surface weather and PCSS, openmw-rtx de-lighting, and
  (today) skyrim-delirium. So (a) is void for the binary that matters. The owner dropped the quarantine rule, so (d)
  is closed.
- Separate programs: the FSF FAQ (`#MereAggregation`) treats pipes, sockets and command-line arguments as normally
  separate programs, unless the communication is "intimate" enough to exchange complex internal data structures.
  Ryan Gordon's **steamshim** (zlib) is the known pattern: a small C++ parent links Steamworks and launches the GPL
  game, which talks to it over pipes. It was built for exactly this case (stats and achievements, extensible).
- GPL titles on Steam without the SDK: Blender ships on Steam with no Steamworks integration. OpenTTD shipped on Steam
  in 2021 under GPL-2.0. Mindustry (GPL-3.0) ships achievements and the Workshop as its *sole author*, the case we
  can't claim.
- What needs no SDK: the Steam overlay and Steam Input are provided by the Steam client to any game it launches. Steam
  Cloud has Auto-Cloud configured in the partner site. Steam Deck Verified requires good controller and text-input
  behaviour, which our own UI keyboard can meet. So **the Steam Deck floor needs no Steamworks at all**.
- Networking: Valve's GameNetworkingSockets is BSD-3-Clause (GPL-compatible) and can sit in-process. The Steam
  Datagram Relay itself needs Steam, which lightyear's Steam path (aeronet/steamworks-rs in-process) would link into
  the game.

**Weighing.** (a) is legally void while the binary carries others' GPL code. (d) is closed by the owner. (c) is
enough for launch and for the Deck. (b) adds what players expect (achievements, rich presence, invites, lobbies)
with a generic, narrow IPC (no shared internal data structures), which is the defensible side of the FAQ's line. The
helper must contain **no GPL code**, so it and its IPC crate are owner-written and permissively licensed. Agent-written
code has thin copyright, but the owner holds what there is.

**Decision.** **(c) first, then (b) for platform services; no LICENSE-EXCEPTION change for Steamworks.**
- The Steam release starts SDK-free (overlay, Input, Auto-Cloud, Deck).
- N8 becomes **`scrib-platform-steam`**: a separate executable (MIT OR Apache-2.0, owner-written, no GPL dependency)
  linking Steamworks. The game talks to it through a small, versioned, generic message protocol (`sf_platform_ipc`,
  MIT OR Apache-2.0) over a local pipe or socket: achievements, stats, rich presence, lobby create/join/invite,
  overlay-to-invite, auth tickets. One trait seam (`PlatformServices`) in the game takes Steam, EOS, GOG Galaxy or
  none. The game runs fully without the helper.
- Gameplay transport stays our UDP stack. Steam relay (SDR) through the helper, by relaying datagrams over IPC, is
  optional and only if NAT traversal through our own relay proves insufficient.
- Consoles stay out: helper processes don't help there (NDA code in-process, GPLv3 §6 installation information).

**Confidence: high** on the direction, **medium** on the legal line (the separate-process reading is the
widely-used one, not a court ruling). The owner may want one hour with a licensing lawyer before the first Steam
upload (owner-only, below).

**Revisit if** the FSF or a court narrows the separate-process reading, if Valve publishes a GPL-specific route, or if
lightyear gains a Steam transport that can run out-of-process.

## Q10. Stability promise

**Question.** When and how do `sf_*` APIs and the save, record and mod data formats start carrying compatibility
guarantees? This covers the versioning scheme, deprecation policy and format versioning.

**Options.** (a) No promises until 1.0 of everything. (b) Promise the Rust crates (semver) once published. (c) Promise
the **data contracts** first, each at its own trigger, and leave Rust unpromised until Bevy 1.0.

**Evidence.**
- *Script and data APIs, not engine internals, carry the promises in modding platforms.* Roblox keeps deprecated
  APIs working indefinitely. Luanti has kept its Lua API backward-compatible for over a decade, with deprecation
  warnings, and its network protocol accepts a version range. Factorio breaks its mod API only at major versions (1.1
  → 2.0); mods declare `factorio_version`, and old saves load in newer versions. Minecraft versions data packs with an
  integer `pack_format`. RimWorld mods declare `supportedVersions`, and every 1.x breaks some of them.
  Bethesda/SKSE: native plugins break on every runtime patch, which is the failure mode of promising nothing at the
  binary layer while modders depend on it.
- *Engines' code-API policies.* Unreal deprecates with `UE_DEPRECATED` and removes a few releases later. Unity marks
  `[Obsolete]` and ships an API Updater that rewrites user scripts. Godot promises GDExtension forward compatibility
  within 4.x ("compatibility methods"). Bevy promises nothing and ships a migration guide per release, and the Rust
  ecosystem around it versions crates `0.N` per Bevy release with a compatibility table. `cargo-semver-checks`
  enforces semver mechanically.
- *Our formats are already versioned:* `mod.toml` `api_version`, `sf_save` owner-keyed sections with stepwise
  migrations and refused unknown versions (DECISIONS 2026-11-01), the net handshake's protocol version and
  fingerprint, and the content-addressed, regenerable bake.

**Weighing.** Rust promises before Bevy 1.0 would cost us the fork and the cadence (Q6) for no user, since no outside
project depends on `sf_*`. Saves are the format players lose sleep over, and the cheapest to promise because the
migration machinery exists. The mod API is worth promising only once it exists (Luau unparked) and has been used by
the sample pack and the Morrowind adapter.

**Decision.** **(c).** One format registry (`superfargoth/docs/formats.md`) lists every on-disk and wire format:
version, owner crate, spec, regenerable or not, and the trigger below.

| Contract | Version scheme | Guarantee starts | Guarantee |
|---|---|---|---|
| Saves (`.sfsave`, sections) | Integer version per section (exists) | **First public build** (any build given to players) | Saves from any public build load in every later build (stepwise migrations, golden file per version in tests). Never the reverse. A break needs an owner decision and a converter. |
| Records (RON/TOML) and `mod.toml` | Integer `format` per file kind (Minecraft `pack_format` style), plus `api_version` | **Mod API 1.0** | Old files load. Breaking schema changes ship an automatic upgrader (`scrib mod upgrade`, the Unity API Updater and Godot 3→4 converter pattern), and old formats stay readable for at least one major. JSON Schemas published per kind. |
| Luau API (`@sf/*`) | Semver, `api` in the manifest (exists) | **1.0 at unpark + M7 RPG modules** (scripting.md's target) | Minor versions only add. Deprecated bindings log once per mod and are kept for ≥ 1 major *and* ≥ 12 months. Removal only at a major. |
| Net protocol | Integer protocol version in the handshake | First public multiplayer build | Exact match required; cross-version play stays a non-goal (net.md). |
| Bake store | Content-addressed and regenerable | Never promised | Rebuilt on mismatch. |
| `sf_*` Rust crates | `0.N`, one breaking release per Bevy minor | **No promise before Bevy 1.0**; published once an outside project depends on them | Migration notes per release (Bevy's practice) and a Bevy compatibility table. `#[deprecated]` for one release where cheap. `cargo-semver-checks` in CI for published crates. |
| Engine and game releases | Semver `0.y.z` until the first stable game release | n/a | The changelog names every format version bump. |

Stability tiers in crate doc headers (the previous recommendation) are deferred to publication: with no outside users
they would be noise. **Confidence: high.**

**Revisit if** an outside project wants to depend on `sf_*` before Bevy 1.0 (publish with the compatibility table,
still unpromised), if Bevy reaches 1.0 (start Rust semver on L0 crates), or if a save break is ever proposed (owner
decision, never an agent's).

## DCO or CLA (from Q1)

**Question.** Now that GPL stays: DCO, CLA or nothing?

**Options.** (a) Nothing (inbound = outbound by convention; GitHub's terms of service §D.6 apply inbound = outbound
to public repos). (b) DCO sign-off (Linux; GitLab switched from a CLA to the DCO in 2017). (c) A CLA (Qt, Canonical,
MongoDB; used to relicense or dual-license). (d) Copyright assignment (FSF; GCC dropped the requirement in 2021).

**Evidence.**
- GPL engines and games mostly use (a): Blender, Godot (MIT), OpenMW, Veloren, Luanti. CLAs are tied to dual
  licensing and relicensing, the path the owner rejected (option D in vision §2.3), and they deter contributors.
- Relicensing without a CLA is possible but costly. Bevy asked every contributor to approve its MIT OR Apache-2.0
  move (bevy#2373), and VideoLAN contacted hundreds of authors to move libVLC to LGPL. We wouldn't need that unless the
  licence changes, which the owner has ruled out.
- AI-era code: the US Copyright Office (2025) holds that purely AI-generated material isn't copyrightable. A CLA
  can't assign rights that don't exist, while a DCO certifies what matters: the contributor has the right to submit.

**Decision.** **(b) DCO**, with `Signed-off-by` on every commit, checked by CI. CONTRIBUTING states inbound =
outbound: contributions are licensed GPL-3.0-or-later with the additional permissions in `LICENSE-EXCEPTION.md`, as
the project's own code. Also, one sentence of consent that the maintainers may add further GPLv3 §7 *additional
permissions* (only ever widening rights, never restricting) to contributions for platform SDKs and vendor runtimes.
That keeps the owner's options open at no cost to contributors' freedom. AI-assisted commits name the tool in a
trailer (`Co-Authored-By`, as our agents already add). **Confidence: high** (DCO), **medium** (the §7 consent
sentence; it is uncommon, so drop it if a contributor objects).

**Revisit if** the owner ever wants dual licensing (then a CLA, before the first outside PR).

## Owner-only items (not decidable by an agent)

- **Money:** a Steam Deck as the floor-tier gate device. Steam Direct's per-app fee, and whether Scrib or only the
  game goes on Steam. An optional hour with a licensing lawyer on the helper-process reading before the first Steam
  upload.
- **Time and relationships:** whether the owner wants to engage personally with Bevy's rendering maintainers (the
  upstream hooks move faster with a known face in the working group); whether and when to open the repo to outside
  contributors (DCO goes live then).
- **Delirium:** settled by the owner (stays default, see Q8); only the re-pin is left.

## Sources

- Bevy releases and 0.20 notes (BSN syntax "largely nailed down", editor building blocks): https://bevy.org/news/ ,
  https://bevy.org/news/bevy-0-20/
- Bevy relicensing approvals: https://github.com/bevyengine/bevy/issues/2373
- Cargo `[patch]` with git sources: https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html
- Godot GDExtension compatibility and `extension_api.json`:
  https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/what_is_gdextension.html
- Unity API Updater: https://docs.unity3d.com/Manual/APIUpdater.html ; Unreal coding standard (deprecation):
  https://dev.epicgames.com/documentation/en-us/unreal-engine/epic-cplusplus-coding-standard-for-unreal-engine
- Minecraft `pack_format`: https://minecraft.wiki/w/Pack_format ; Factorio mod `info.json`:
  https://wiki.factorio.com/Tutorial:Mod_structure ; Luanti (client-side mod restrictions, Lua API):
  https://github.com/luanti-org/luanti
- Luau: https://luau.org ; mlua: https://github.com/mlua-rs/mlua
- Wasmtime stability and LTS: https://docs.wasmtime.dev/stability-release.html ; WASI 0.3 (2026-06-11, Wasmtime 46):
  https://wasi.dev/releases/wasi-p3 ; Veloren WIT plugin host:
  https://docs.veloren.net/src/veloren_common_state/plugin/module.rs.html
- Steamworks, distributing open source: https://partner.steamgames.com/doc/sdk/uploading/distributing_opensource
- steamshim (zlib, GPL game ↔ Steamworks over pipes): https://github.com/icculus/steamshim
- GameNetworkingSockets (BSD-3-Clause): https://github.com/ValveSoftware/GameNetworkingSockets
- GPL FAQ, `#MereAggregation` and `#GPLIncompatibleLibs`: https://www.gnu.org/licenses/gpl-faq.html ; GPLv3 §7:
  https://www.gnu.org/licenses/gpl-3.0.html
- OpenTTD on Steam (2021): https://www.gamingonlinux.com/2021/02/classic-open-source-simulation-game-openttd-is-coming-to-steam
- Mindustry on Steam (sole author, GPL-3.0): https://gamingonlinux.com/2019/09/mindustry-the-factorio-like-tower-defense-game-is-now-on-steam/page=1
- delirium-physics README and refs (checked 2026-11-08 with `git ls-remote` and `git fetch <rev>`):
  https://github.com/PVCPC/delirium-physics ; GitHub DMCA notices (Take-Two vs re3, 2021): https://github.com/github/dmca
- DCO: https://developercertificate.org ; GitLab's switch to the DCO (2017):
  https://about.gitlab.com/blog/2017/11/01/gitlab-switches-to-dco-license/ ; GitHub ToS, contributions under the
  repository licence: https://docs.github.com/en/site-policy/github-terms/github-terms-of-service
- US Copyright Office, Copyright and AI, Part 2 (2025): https://www.copyright.gov/ai/

## Ready-to-paste edits for shared docs

None of these were applied. Each is old text → new text, exact as of 2026-11-08.

### `superfargoth/docs/DECISIONS.md` (append)

```
- 2026-11-08 (agent, delegated by owner; `docs/platform-decisions-2026-11-08.md`): **scope**: an engine for systemic,
  persistent 3D open worlds with characters; the ARPG is the first L3 kit, Morrowind the first pack. L0/L1 never
  depend on L3 (CI `cargo metadata` check). No 2D/RTS/racing, no own editor.
- 2026-11-08 (agent, delegated): **sample pack** S0 now: `samples/greybox` (CC0 data) + an `sf_*`-only headless app in
  CI (boot, stream, two command-driven capsules, stat change, save/load round trip; hosts the loopback co-op test).
  S1 (rendered CC0 valley, scripted quest) after the vertical slice.
- 2026-11-08 (agent, delegated): **Bevy**: adopt every minor, never skip; lane starts at the RC, ship within 6 weeks
  (8 max). Vendored Bevy crates move to a git fork branch (one commit per patch, upstream route per patch). ≥ 1
  upstream slice per cycle: view-level material hook first, diffuse-GI provider second; never from GPL-derived hunks.
- 2026-11-08 (agent, delegated): **scripting**: Luau stays parked (owner rule); each spec gets a "Script surface"
  (events, commands, formula structs as plain serde data). On unpark the registry's truth is a JSON API dump that
  generates `.d.luau`/docs/WIT. MP: gameplay scripts host-only; clients get a presentation-only context.
- 2026-11-08 (owner): **Delirium stays a default, core part of the game** (an agent proposal to remove it is void).
  Pinned rev `91ee246` is gone upstream: re-pin to an existing tag/branch on a project-owned fork.
- 2026-11-08 (agent, delegated): **Steam**: release SDK-free first (overlay, Input, Auto-Cloud, Deck); N8 becomes a
  separate permissive `scrib-platform-steam` helper over a generic IPC (`sf_platform_ipc`, MIT/Apache). No
  LICENSE-EXCEPTION change: third-party GPL code in the binary makes a Steamworks exception void.
- 2026-11-08 (agent, delegated): **stability**: promises go to data, not Rust. Saves load forward from the first
  public build; records/`mod.toml` (integer `format`) and the Luau API (semver, deprecations ≥ 1 major and 12 months)
  from mod API 1.0; `sf_*` crates `0.N` per Bevy minor, unpromised until Bevy 1.0. Registry: `docs/formats.md`.
- 2026-11-08 (agent, delegated): **DCO**, not a CLA: `Signed-off-by` checked by CI; inbound = outbound
  (GPL-3.0-or-later + LICENSE-EXCEPTION), plus consent to future §7 additional permissions for platform SDKs.
```

### `superfargoth/docs/ARCHITECTURE.md`

§1, old:
```
An open-world action-RPG framework on Bevy, with Morrowind as its first and
flagship content pack. Not a faithful port: "Morrowind if Bethesda made it in
2026". Morrowind's 2002 feature set is a floor, not a ceiling.
```
new:
```
An engine on Bevy for systemic, persistent 3D open worlds with characters
(`docs/platform-decisions-2026-11-08.md` Q4). The action-RPG kit (`sf_rpg`,
`sf_stats`, `sf_combat`, `sf_action`) is its first genre kit, and Morrowind is
its first and flagship content pack: "Morrowind if Bethesda made it in 2026".
Morrowind's 2002 feature set is a floor, not a ceiling. Engine layers (L0 core,
L1 world simulation) never depend on a genre kit (L3).
```

§3, old:
```
- Vendored Bevy patches stay MIT OR Apache-2.0 with no GPL-derived lines, so
  they can go upstream. MIT and Apache-2.0 dependencies are GPLv3-compatible.
```
new:
```
- Vendored Bevy patches are commits on our Bevy fork branch, each with an
  upstream route. One own patch is GPL-derived (H2 PCSS, Clockwork) and must be
  rewritten clean before it goes upstream; upstream PRs are always written fresh
  under MIT/Apache. MIT and Apache-2.0 dependencies are GPLv3-compatible.
- Proprietary platform SDKs (Steamworks, EOS, Galaxy) never enter the game
  process: they live in separate, permissively licensed helper executables over
  a generic IPC (`sf_platform_ipc`), because third-party GPL code in the binary
  can't carry a §7 exception (`docs/platform-decisions-2026-11-08.md` Q9).
```

§4, old:
```
**Lag policy:** we track the latest Bevy. Crates that lag go behind
`[patch]` forks until upstream catches up. The first milestones (VFS,
records, NIF loading) need only Bevy itself.
```
new:
```
**Lag policy:** we adopt every Bevy minor and never skip one: the upgrade lane
starts at Bevy's first RC and ships within 6 weeks of the release (8 at most
when a critical crate lags); patch releases within a week. Crates that lag go
behind `[patch]` forks until upstream catches up. No Rust API stability is
promised before Bevy 1.0; the platform contract is data (see the format
registry, `docs/formats.md`).
```

§6.1, old:
```
No native code mods (dylibs): Rust has no stable ABI, and native mods break
sandboxing. Possibly WASM later, if Lua proves too slow for some mods.
```
new:
```
No native code mods (dylibs): Rust has no stable ABI, and native mods break
sandboxing. A WASM tier (wasmtime component model) may come after Luau API 1.0
for heavy server mods, generated from the same API dump as the Luau bindings.
In multiplayer, gameplay scripts run on the host or server only; clients run
presentation-only scripts.
```

### `CONTRIBUTING.md`

Old:
```
Scrib is two parts in one repo and one Cargo workspace: `superfargoth/`, a game-agnostic open-world ARPG
framework on Bevy (`sf_*` crates), and `bevyrrowind/`, the Morrowind content pack and game (`mw_*` crates).
```
new:
```
Scrib is two parts in one repo and one Cargo workspace: `superfargoth/`, a game-agnostic engine on Bevy for
systemic open worlds, with an action-RPG kit (`sf_*` crates), and `bevyrrowind/`, the Morrowind content pack
and game (`mw_*` crates).
```

Old:
```
- **Net-ready:** small co-op (2-4 players, host-authoritative) comes later, so new specs use a fixed tick,
  input as commands, presentation that never feeds gameplay, and a remote player is just another character.
```
new:
```
- **Net-ready:** multiplayer has no fixed ceiling (co-op 2-4 is the first milestone; server-authoritative,
  headless/dedicated builds later), so new specs use a fixed tick, input as commands, presentation that never
  feeds gameplay, and a remote player is just another character. Gameplay scripts run on the host only.
- **Script-ready:** each system's spec has a "Script surface" section: the events it emits and the commands it
  accepts (plain serde data, stable ids, never `Entity`) and its formula inputs (`fn(&In) -> Out`). No Luau
  code while Luau is parked.
```

Old:
```
GPL-3.0-or-later with a GPU vendor runtime permission (`LICENSE-EXCEPTION.md`). Ported code cites its
source file + commit in the module docs and gets a `superfargoth/THIRD_PARTY_NOTICES.md` entry; never
GPL-2.0-only, AGPL or non-free code. Proprietary SDKs (NVIDIA DLSS) are reached only through a separately
shipped plugin and are never committed. Other games' data follows the clean-room rule
(`superfargoth/CLAUDE.md` rule 3).
```
new:
```
GPL-3.0-or-later with a GPU vendor runtime permission (`LICENSE-EXCEPTION.md`). Ported code cites its
source file + commit in the module docs and gets a `superfargoth/THIRD_PARTY_NOTICES.md` entry; never
GPL-2.0-only, AGPL or non-free code, and never code derived from a commercial game's program code or data,
whatever its licence says. Proprietary SDKs (NVIDIA DLSS) are reached only through a separately shipped
plugin and are never committed; platform SDKs (Steamworks and others) only through a separate helper process.
Other games' data follows the clean-room rule (`superfargoth/CLAUDE.md` rule 3).

Contributions use the DCO (https://developercertificate.org): every commit carries `Signed-off-by`. You
license your contribution under GPL-3.0-or-later with the additional permissions in `LICENSE-EXCEPTION.md`,
and you agree that the maintainers may add further GPLv3 section 7 additional permissions to it for platform
SDKs and vendor runtimes. AI-assisted commits name the tool in a trailer (`Co-Authored-By`).
```

### `bevyrrowind/docs/ROADMAP.md`

Lines 3.2 and 3.2b: unchanged (owner 2026-11-08: Delirium stays default). Add one backlog line:
```
- [ ] Re-pin Delirium: rev 91ee246 is gone upstream; pin an existing tag/branch on a project-owned fork (owner workstation, build + test).
```

Add after the "Owner decisions" block, before "Still open":
```
Decided 2026-11-08 (owner + delegated, `docs/vision-and-platform-2026-11-08.md` §9, `docs/platform-decisions-2026-11-08.md`):
5. **Platform:** GPL stays; no MP ceiling; Steam Deck floor tier with its own gate; engine for systemic open worlds,
   ARPG as the first kit. P-lane: P0 Delirium re-pin on a project fork, P1 CI + DCO + layer check, P2 sample pack S0
   (headless, CI) + loopback co-op test, P3 `sf_scalability` + floor gate, P4 format registry (`docs/formats.md`).
```

Phase 4 line, old:
```
- [p] Co-op N1–N8 (`specs/net.md`; N1–N5 after this phase, N6–N8 after M7).
```
new:
```
- [p] Co-op N1–N8 (`specs/net.md`; N1 loopback test pulled forward into P2; N2–N5 after this phase, N6–N8 after M7;
  N8 = Steam through the separate `scrib-platform-steam` helper, platform decisions Q9).
```

### `superfargoth/docs/specs/scripting.md`

Old:
```
- **Framework API** (`@sf/*`) is one semver line (`api` in the manifest, already in
  `sf_mods::ModManifest`). Minor versions only add; a removal or semantic change is a major
  bump with the old behaviour kept for one major where feasible. Bindings newer than the mod's
  requirement are still visible (additive), deprecated ones log once per mod. Until the first
  public mod release it stays `0.x` (no promises); target `1.0` once M7's RPG modules exist.
```
new:
```
- **Framework API** (`@sf/*`) is one semver line (`api` in the manifest, already in
  `sf_mods::ModManifest`). Minor versions only add; a removal or semantic change is a major
  bump. Deprecated bindings log once per mod and are kept for at least one major and 12 months.
  Bindings newer than the mod's requirement are still visible (additive). Until the first
  public mod release it stays `0.x` (no promises); target `1.0` once Luau is unparked and M7's
  RPG modules exist (`docs/platform-decisions-2026-11-08.md` Q10).
- **Registry source of truth (on unpark):** a structured API dump (`sf_api.json`: modules,
  functions, types, `since`/`deprecated`), the Godot `extension_api.json` / Roblox API dump
  pattern. `.d.luau`, docs, JSON Schemas and a later WIT world are generated from it;
  `ApiModule::types()` (hand-written Luau text) is replaced then. While parked, systems are
  only script-ready (CONTRIBUTING "Script-ready").
- **Multiplayer:** gameplay contexts exist on the host or server only. Clients may run a
  presentation context (UI, HUD, cosmetic) with no gameplay writes; its only path to the
  world is `sf_action::Command`s the host validates. Per-mod limits are server settings.
```

Old:
```
Later: `@sf/rules` (formula overrides, [rule-hooks.md](rule-hooks.md); unparks first if the owner agrees),
```
new:
```
Unpark trigger (platform decisions Q7): the first of sample pack S1's quest, the Phase 2.7
formulas built, or mod API 1.0 needed; then `@sf/records`, `@sf/rules`, events, in that order.
Later: `@sf/rules` (formula overrides, [rule-hooks.md](rule-hooks.md)),
```

### `superfargoth/docs/specs/net.md`

Old:
```
  (N2 decides). **Steam** (lobbies, relay, invites) is N8, an opt-in feature of the game repo:
  the networking crates are MIT/Apache, the Steamworks SDK is proprietary, so it never enters
  superfargoth.
```
new:
```
  (N2 decides). **Steam** (lobbies, invites, presence; relay optional) is N8, through a separate,
  permissively licensed helper executable (`scrib-platform-steam`) over a generic IPC
  (`sf_platform_ipc`): the game binary carries third-party GPL code, so Steamworks can never be
  linked into it (`docs/platform-decisions-2026-11-08.md` Q9). Gameplay packets stay on our UDP stack.
```

### `LICENSE-EXCEPTION.md`

Old:
```
- In particular the Delirium active ragdoll (`mw_delirium`: delirium-physics and code ported from skyrim-delirium,
  GPL-3.0-only), which default game builds link (feature `delirium`; such binaries are GPL-3.0-only), is not covered.
  We never bundle NVIDIA's DLSS runtime: the player fetches it (setup screen, `bevyrrowind/docs/specs/setup.md`),
  so default builds are fine to distribute. Only a package that bundles a vendor runtime with the game must be built
  without Delirium (`--no-default-features`).
```
new:
```
- Code ported from other projects (OpenMW, Clockwork, openmw-rtx, the Delirium active ragdoll (`mw_delirium`,
  GPL-3.0-only, on by default) and others in `superfargoth/THIRD_PARTY_NOTICES.md`) is in every game binary, so this
  permission never makes a whole game binary combinable with a proprietary library.
  We never bundle NVIDIA's DLSS runtime: the player fetches it (setup screen, `bevyrrowind/docs/specs/setup.md`).
  Platform SDKs (Steamworks and others) are not covered and never enter the game process; they run in a separate
  helper program.
```

### `superfargoth/vendor/README.md`

Old:
```
Patched copies of crates.io crates, wired with `[patch.crates-io]` in **both**
`superfargoth/Cargo.toml` and `bevyrrowind/Cargo.toml`, excluded from the superfargoth
workspace (not ours to lint or test). Every changed hunk carries a `superfargoth patch`
comment; `grep -rn "superfargoth patch" vendor/<crate>` lists them.
```
new:
```
Patched copies of crates.io crates, wired with `[patch.crates-io]` in the root `Cargo.toml`,
excluded from the workspace (not ours to lint or test). Every changed hunk carries a
`superfargoth patch` comment; `grep -rn "superfargoth patch" vendor/<crate>` lists them.

**Moving (platform decisions Q6, 2026-11-08):** the Bevy crates go to a git fork of
bevyengine/bevy (branch `scrib-v<minor>`), one commit per row below (upstream PRs
cherry-picked as-is, own patches with their marker in the message), wired with
`{ git, rev }` in `[patch.crates-io]`. Only `big_space` and `dlss_wgpu` stay here. Each own
patch states its upstream route: a PR, "needs hook X", or **GPL-derived** (H2 PCSS: adapted from
Clockwork, GPL-3.0; rewrite clean from papers before upstreaming). Each upgrade report states own-patch
hunks and lines before and after.
```

Old:
```
## Rebase or drop on a Bevy upgrade
```
new:
```
## Rebase or drop on a Bevy upgrade

Cadence: the lane starts at Bevy's first RC and ships within 6 weeks of the release (ARCHITECTURE §4).
Once the fork exists: `git rebase --onto v<new> v<old> scrib-v<old>` into `scrib-v<new>`, drop the
commits of merged PRs, bump `rev`. Until then, the folder procedure below.
```
