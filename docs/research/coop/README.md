# Co-op multiplayer: research (R12)

Status: research, 2026-10-19. Feeds a future `docs/specs/net.md` and the co-op milestone.
Owner decision (2026-10-19, `research/animation/11-tradeoffs.md` "Decisions"): **small co-op, 2-4 players,
one player hosts, host-authoritative with client prediction. Design now, build after M4-M6. No lockstep.**

Bevy 0.20 itself has no networking. The Bevy parts we use are `FixedMain` / `Time<Fixed>` (the tick),
`Reflect` + serde (state), and observers/events. Everything else is a crate or ours (rule 8).

---

## 1. Crate survey

Checked 2026-10-19 on the crates.io API and GitHub. Bevy 0.20 shipped 2026-10-08.

| Crate | Latest (date) | Bevy | Licence | What it is |
|---|---|---|---|---|
| [`bevy_replicon`](https://github.com/simgine/bevy_replicon) | 0.45.2 (2026-10-15) | **0.20** | MIT/Apache | Server-authoritative ECS replication. No I/O, no prediction |
| [`lightyear`](https://github.com/cBournhonesque/lightyear) | 0.30.1 (2026-09-16) | 0.19 (0.20 not yet) | MIT/Apache | Full stack: ticks, inputs, prediction/rollback, interpolation, transports. **Built on replicon since 0.27** |
| [`renet`](https://github.com/lucaspoffo/renet) / `bevy_renet` | 2.0.0 / 5.0.0 (2026-06-20) | 0.19 | MIT/Apache | Transport: netcode.io UDP, channels; `renet_steam` 3.0 |
| [`renet2`](https://github.com/UkoeHB/renet2) / `bevy_replicon_renet2` | 1.0.0 (2026-10-16) | **0.20** | MIT/Apache | renet fork: UDP, memory, WebTransport, WebSocket; `renet2_steam` |
| [`aeronet`](https://github.com/aecsocket/aeronet) (+`_steam`, `_webtransport`, `_replicon`) | 0.22.0-rc.1 (2026-09-23) | 0.20-rc | MIT/Apache | Transport layer; lightyear's Steam and WebTransport path |
| [`bevy_quinnet`](https://github.com/Henauxg/bevy_quinnet) / `bevy_replicon_quinnet` | 0.22.0 / 0.21.0 (2026-10-13) | **0.20** | MIT/Apache | QUIC transport |
| [`naia`](https://github.com/naia-lib/naia) | 0.25.0 (2026-05-12) | 0.18 | MIT/Apache | Own replication + rooms + tick-buffered input + authority delegation; native UDP, browser WebRTC |
| [`bevy_ggrs`](https://github.com/gschup/bevy_ggrs) / `ggrs` | 0.22.0 / 0.13.0 (2026-06-26) | 0.19 | MIT/Apache | GGPO-style P2P rollback: **rejected** (below) |
| `bevy_replicon_snap` | 0.2.6 (2024-12) | 0.15 | MIT | Interpolation/prediction addon, listed unmaintained by replicon |
| [`bevy_rewind`](https://github.com/NiseVoid/bevy_rewind) | 0.0.1 (2025-03) | none | MIT/Apache | Rollback for replicon; last push 2026-01, early |
| `bevy_matchbox` | 0.14.0 (2026-02) | 0.18 | MIT/Apache | WebRTC P2P sockets |

**lightyear** ([README](https://github.com/cBournhonesque/lightyear), [0.27-0.30 releases](https://github.com/cBournhonesque/lightyear/releases)).
- Replication: through `bevy_replicon` since 0.27 (June 2026). Lightyear adds prediction, interpolation,
  authority metadata, hierarchy and pre-spawning on top.
- Prediction/rollback: per-component, "one-line" opt-in. Inputs are buffered per tick and redundantly sent.
  The input for tick N runs on tick N on the server. It has input delay, correction easing (0.30) and
  runtime switching between predicted and interpolated timelines (0.30, like Unity Netcode's
  [prediction switching](https://docs.unity3d.com/Packages/com.unity.netcode@1.3/manual/prediction-switching.html)).
  It also has lag compensation.
- Interpolation: snapshot interpolation; hermite with velocities (0.30).
- Interest management: replicon visibility filters, global `RoomId`s.
- Inputs: native, `leafwing-input-manager` and **`bevy_enhanced_input`**. BEI is our pick (§10, u1a).
- Topologies: client-server, **host-client (a client acts as server)**, deterministic P2P (0.29).
- Transports: UDP, crossbeam (in-process), WebSocket, WebTransport, **Steam** (via aeronet, `lightyear_steam`).
- Avian: `lightyear_avian3d` corrects position, rotation and velocities. **We use `rapier3d` directly**
  (physics.md), so this integration does not apply; see §3.
- Maintenance: one main author, very active (last push 2026-10-16, 1.2k stars, 63 open issues).
  Bevy lag was 9 days for 0.18 and 3 days for 0.19. For 0.20 it is 11 days and counting; dependabot
  bump PRs are open. The issue tracker shows prediction edge cases still being fixed (#1780, a visual
  correction that trails, open).
- Size: a large workspace (io, connection, inputs, replication, prediction, interpolation, transport).
  This matters for our compile-time rule (§8b).

**bevy_replicon** ([README](https://github.com/simgine/bevy_replicon)).
- Server-to-client replication, remote events and triggers, authorization, and per-client entity and
  component visibility.
- One code path for singleplayer, client, dedicated server and **listen server**.
- I/O comes from a backend (renet, renet2, quinnet, aeronet, matchbox).
- On Bevy 0.20 four days after release, with a release every one to two weeks.
- **No prediction or interpolation**: the addons for those are unmaintained or early. It is now the
  shared base of the ecosystem, since lightyear is built on it.

**naia** follows the Tribes 2 model, with rooms, tick-buffered channels and authority delegation.
- It has its own replication (not replicon) and no built-in prediction systems.
- It is on Bevy 0.18. Before 0.25 it went 17 months without a release.
- Viable, but outside the main line.

**GGRS (rejected).** GGPO rollback re-simulates the whole game for every peer and needs bit-exact
determinism ([GGRS](https://github.com/gschup/ggrs)). An open world doesn't fit that: streaming,
Rapier, AI, scripts and hundreds of NPCs would all need to be bit-exact and re-run on every
misprediction. The owner ruled out lockstep for the same reason.

**Verdict.** The ecosystem has converged: **replicon is the replication layer and lightyear is the
prediction layer on top**. Transports are interchangeable.

---

## 2. Prior art: open-world RPG co-op

**TES3MP (Morrowind on OpenMW).** Server plus Lua server scripts; any player can run a server.
- **What it syncs.** Over years it grew to cover players, NPC movement, combat, spells, equipment,
  journal, kill counts, faction state, object enable/disable, containers, time of day, weather, map
  exploration, custom records and script variables ([changelog](https://github.com/TES3MP/TES3MP/blob/master/tes3mp-changelog.md)).
- **NPC authority.** AI runs on one client per cell, the *cell authority*. That is the first visitor,
  taken over by a lower-ping visitor, and passed to the lowest-ping visitor when the authority leaves
  ([CoreScripts `logicHandler.lua`](https://github.com/TES3MP/CoreScripts/blob/master/scripts/logicHandler.lua)).
  What broke because of it:
  - late authority packets caused desyncs;
  - followers couldn't follow non-authority players through doors;
  - only the authority could make actors teleport. Others got a message box saying the NPC's "AI is
    running on another player's client" ([commit](https://git.cijber.net/mirror/openmw-tes3mp/commit/528bd26a3b494e20192372c8313769edc93d9b9f)).
- **Scripts.** MWScript runs on every client. Only variables of scripts the server whitelists are synced
  (0.8.0: "based on which clientside scripts are set to be synchronized by the server").
  - Most quests work. "Quests relying on complex scripting may not work", and mod compatibility "is
    not guaranteed" ([UESP](https://en.m.uesp.net/wiki/Tes3Mod:TES3MP)).
  - The project names AI and client-side script variables as its main remaining problems.
- **Saves.** Players and world state are stored on the server. The plugin list must match on every client.
- **Lesson.** Client-side authority and client-side scripts were the source of most bugs. Every
  single-player assumption (one `Player`, menus that pause) needed its own packet type.

**Skyrim Together Reborn.**
- **Quests and dialogue.** Only the *party leader* progresses quests, talks to quest NPCs and picks up
  quest items. Others hear the leader's dialogue and can fight and solve puzzles. Developers advise one
  leader per playthrough, because switching breaks quests. Random encounters may not appear for everyone
  ([guide](https://www.ggrecon.com/guides/skyrim-together-reborn-how-to-download-install-play/),
  [host docs](https://wabbanode.com/help/skyrim-together/understanding-parties-in-skyrim-together-reborn)).
- **Lesson.** If quest scripts assume one player, the honest fix is one quest owner.

**Elden Ring Seamless Co-op (mod).**
- Players join the host's world; guests' progress events also count in their own worlds.
- **Resting at a grace resets the world for everyone**, "to prevent enemy desync".
- It uses a separate save file (`.co2`), and every player needs the same mod version
  ([Prima](https://primagames.com/tips/how-to-download-and-use-the-elden-ring-seamless-co-op-mod),
  [eXputer](https://exputer.com/guides/elden-ring-seamless-co-op-mod/)).
- **Lesson.** Time-skip and reset actions must be world-wide events with a rule (vote or all-reset).
  Co-op saves stay apart from single-player saves.

**Valheim.**
- **What it does.** The world is split into 64 m zones. The first player in a zone becomes the *zone
  owner* and simulates its physics and monster AI. The dedicated server mostly relays.
- **What broke.** One player's bad connection lags everyone in that zone, and the fix is "leave and
  re-enter". Mods move simulation back to the server
  ([host guide](https://xgamingserver.com/blog/how-to-fix-lag-desync-in-valheim-dedicated-server/),
  [Dedicated Simulation mod](https://thunderstore.io/c/valheim/p/Sarkastic_gg/Dedicated_Simulation/)).
- Characters are saved on the client and the world on the server.
- **Lesson.** Distributing authority saves server CPU but spreads lag and desync. With 2-4 players and a
  host who is playing, host-only simulation is the better trade.

**Enshrouded.**
- **Two progress layers.** The *character* keeps level, gear and inventory and travels between worlds.
  The *world* keeps quest steps, rescued NPCs and the flame level, and stays with the host or server.
- Quest steps are world-wide, so one player finding the blacksmith unlocks him for all.
- Rewards (skill points, XP) are personal and need presence. Players who missed sessions find finished
  quests in their journal, and a "personal progression" mode is a frequent request
  ([guide](https://www.itechguides.com/how-progression-works-in-enshrouded-multiplayer/),
  [host wiki](https://supercraft.host/wiki/enshrouded/quest-progression-explained/)).
- **Lesson.** This is the cleanest model for us: Morrowind's quest state is already global.

**General netcode references.**
- Overwatch: ECS netcode with predicted, tick-based player commands and rollback of the local player only
  ([Ford, GDC 2017](https://www.gdcvault.com/play/1024001/-Overwatch-Gameplay-Architecture-and)).
- Valve Source: interpolation and lag compensation
  ([networking](https://developer.valvesoftware.com/wiki/Source_Multiplayer_Networking),
  [lag compensation](https://developer.valvesoftware.com/wiki/Lag_Compensation)).
- Gambetta: [prediction and reconciliation](https://www.gabrielgambetta.com/client-side-prediction-server-reconciliation.html).
- Fiedler: [state synchronization](https://gafferongames.com/post/state_synchronization/).
- Epic: Unreal does not replicate animation. Only montages, which have a rewindable timeline, carry root
  motion across the network ([Epic staff](https://forums.unrealengine.com/t/root-motion-in-multiplayer-without-montages/349465)).
  Motion warping on the owning client jitters when the replicated transform fights local root motion
  ([forum](https://forums.unrealengine.com/t/why-is-root-motion-animation-jittery-on-originating-client/592746/2)).

---

## 3. What co-op demands from each system

Model: **the host simulates the whole world.** Each client predicts only its own character (and items
it holds), and sees every other entity interpolated about 100 ms in the past.

**Action and animation (R11b, `sf_controller`, `sf_anim_graph`).**
- *Predicted moves.* The local player's move state (move id, start tick, phase, warp target, root
  velocity) is gameplay state, advanced on the fixed tick from commands. On a correction the client
  rewinds to the host's state and replays the buffered commands. The *pose* is never sent: remote
  clients rebuild it from the replicated move state and the shared clip clock (Epic's rule).
- *Root motion under prediction.* Root motion must be a pure function of (clip, ticks elapsed, warp)
  read from the baked root track, so a replay gives the same path. A clip's rendered playback must not
  drive it.
- *Warps.* Warps onto **static** targets (ledges, seats, doors from the bake) predict well, because
  every machine has the same static world. Warps onto **moving** targets need the target's host state:
  - paired kills and grabs on NPCs are *host-started*: the client predicts the wind-up only;
  - shoves and lunges toward an NPC use the interpolated target and accept a small correction.
- *Hit authority.* The host decides every hit.
  - **Player → NPC:** the host checks the player's swing (from baked socket tracks at the command tick)
    against NPC hitboxes rewound to what that client saw (lag compensation, a ring buffer of NPC poses
    about 250 ms deep). Lightyear ships this.
  - **NPC → player:** the host honours a client's dodge or parry that started at an earlier command
    tick, within a cap (about 150 ms). This is "favour the defender", acceptable among friends, and
    avoids "I dodged and still got hit".
  - Hit-stop and reactions are presentation, applied locally when the host's hit event arrives (or
    predicted on the attacker's side and confirmed).
- *Decisions that already fit.* One world body is truth, presentation never feeds gameplay, and
  first-person arms are presentation (11-tradeoffs "Decisions").

**Physics (`sf_physics`, Rapier).**
- *Determinism.* Not needed beyond "same binary, same input, close result". Rapier's serial solver
  already gives that, and corrections are smoothed. `enhanced-determinism` stays off.
- *Who simulates props.* The host simulates every dynamic body. Clients interpolate props and never
  roll them back.
  - A **held item** is kinematic in holder space (physics.md already does this), so it predicts with
    its holder.
  - A **thrown item** is pre-spawned on the thrower's client and handed to the host body when the host
    confirms it (lightyear pre-spawning).
- *Client collision.* The client's predicted kinematic mover collides with its own static world from
  the local bake, and with props at their last interpolated pose.
- *Conflict with our docs.* physics.md (2026-10-12) keeps the kinematic mover in `Update` at frame
  rate. Prediction needs it **on the fixed tick**: it must replay N ticks in one frame, and host and
  client must agree tick for tick. That decision needs revisiting when R11b moves actions to the
  fixed tick.
- *Large worlds.* physics.md runs one f32 frame around **one** anchor cell. With players far apart,
  the host needs one physics region (its own Rapier world and origin) per cluster of players, or a
  Rapier f64 build. Rapier supports several independent worlds; `sf_physics` already hides Rapier
  types.

**World streaming (`sf_world`, streaming.md, world-coordinates.md).**
- *Host.* R0 (physics, scripts, AI) is kept active around **every** player. streaming.md already allows
  several `Gameplay` anchors. Host CPU grows with the number of separate player regions: up to 4× R0
  in the worst case.
- *Client.* Each client streams its own visual rings and R0 collision locally from its own content copy.
  It also has its own floating origin.
- *Network coordinates.* Positions go over the network as absolute (grid cell, local offset), never
  origin-relative.
- *Interest management.* Replicon visibility is set by worldspace plus distance (about the R0/R1 radius).
  A client only receives NPCs and props near itself. Interiors are separate worldspaces, so they filter
  for free.
- *Content identity.* A content identity check runs at join: same mod set, load order and bake hash
  (TES3MP and naia both refuse mismatches). Deterministic generated content, such as per-tile-seed
  placement and wind, is computed locally, never sent.

**AI and NPCs (`sf_ai`, M7).**
- Host-only. NPCs reach clients as interpolated state: transform, locomotion values, move state,
  health, equipment.
- Perception, threat and dialogue targets must handle **several players**: nearest or most threatening,
  and sneak detection per player.
- Followers follow *a* player id, not "the player" (TES3MP's follower bug).
- NPC attacks use the same move rules as players, run only on the host.

**MWScript and Lua (`sf_script`, scripting-adapter).**
- **Scripts run only on the host**, which avoids TES3MP's main failure. Their effects replicate as
  ordinary state: enable/disable, position, inventory, globals, journal.
- **Effects that target one player are messages to that player:** MessageBox with buttons (reply comes
  back as a request), sounds and UI, journal popups, and `Player->` commands.
- **"Player" needs a context:**
  - the activating, talking or triggering player where there is one;
  - otherwise a host rule, such as nearest player for distance checks or party leader for `GetPCCell`.
- `MenuMode` is per player. The world doesn't pause in co-op (Elden Ring, Valheim). Single-player
  keeps pause.
- Lua `on_update(dt)` stays on the host clock. Luau stays out of the per-tick prediction loop, as
  already decided.

**Saves (`sf_save`, M7).**
- **Host world save:** record overlay, entities, script state, quest and journal state, game clock.
- **Per-player character save:** stats, skills, inventory, spells, appearance, position. It travels
  with the player, as in Enshrouded.
- The host also stores the last state of each guest's character in the world save for reconnects. A
  co-op world save is separate from single-player saves (Elden Ring's `.co2`).
- Joining mid-game is a full state snapshot, which replicon does natively.

**Weather and time (`sf_weather`, `sf_wind`).**
- The host owns the game clock (game time, time scale) and replicates it.
- Weather evaluation is already a fast pure function of (seed, time, region), and wind is a
  deterministic lattice hash (WE1, WD1). So clients compute both locally from the clock. Only
  overrides (script `ChangeWeather`, region transitions) are sent.
- **Rest and wait** advance the shared clock, so they need a rule: everyone agrees, or everyone must be
  resting (Minecraft's bed rule). Elden Ring shows that any world-wide reset must be explicit.

---

## 4. Recommendation

**Crate choice: decide at milestone time. Lean towards lightyear, and use replicon's vocabulary now.**
- Both paths share replicon: its components, replication rules, visibility and listen-server mode.
  Choosing replicon as the replication base is low-risk today.
- Prediction is the open choice. In co-op only the local character (and held items) is predicted, so
  our own rollback for one entity on replicon is a real fallback. Lightyear covers more (input
  buffering, timeline sync, lag compensation, Steam).
- Criteria at milestone time:
  1. Bevy version lag (lightyear: 3-9 days before, 11+ for 0.20);
  2. host-client mode quality;
  3. prediction of a **non-Avian** kinematic mover and move state;
  4. clean build-time cost against §8b;
  5. bandwidth for 4 players and about 200 nearby NPCs;
  6. a Steam transport.
- The networking crates are MIT/Apache. The Steamworks SDK itself is proprietary, so Steam stays an
  opt-in feature in the game repo.

**Net-ready rules (every spec from now on):**
1. **Gameplay on the fixed tick.** Movement, actions, combat, timers and status effects advance in
   `FixedUpdate` with a tick counter. Rendering interpolates. No frame `dt` in gameplay.
2. **Input as commands.** Devices and the camera become a per-tick command (move vector, look direction,
   action intents). Gameplay reads only commands. AI and scripts write the same intents.
3. **State apart from presentation.** Gameplay state is plain `Reflect` + serde components. Pose, camera,
   VFX, sound, UI and first-person arms read it and never write it. Hits come from baked tracks indexed
   by (move, tick), never from the rendered pose.
4. **Pure steps.** The next state comes from (state, command, tick, static world). There is no wall
   clock. RNG is seeded by (entity, tick), never `rand::random`.
5. **One owner per piece of state.** Each system says whether it is host-only (AI, scripts,
   spawn/despawn, damage, inventory, quests, clock, weather overrides) or predicted (local character).
   Clients change the world only through requests the host validates.
6. **No singleton player.** No `Single<Player>` in gameplay. Use a `PlayerId`. Scripts get a context
   player. Camera, UI and streaming anchors are per local player.
7. **Stable ids and absolute positions.** Anything that matters has a stable id (reference or spawn id),
   not a Bevy `Entity`. Positions are stored as grid cell plus local offset.
8. **Events are data.** Hits, deaths and activations carry tick, source and target, so they can be
   sent, replayed and de-duplicated.

**Milestone outline** (after M4-M6, each slice 1-2 steps):

| Slice | Content | Done when |
|---|---|---|
| N0 | `specs/net.md` and an audit of existing specs against the 8 rules (locomotion and physics mover tick, scripting `Player`) | Audit committed; conflicts logged in §10 |
| N1 | Spike in `sf_level` test levels: two processes, a capsule, lightyear vs replicon plus own prediction. Measure clean build time, bandwidth, and feel at 100 ms / 1% loss | Crate picked, §10 entry |
| N2 | `sf_net` thin plugin: host/join over UDP, content-hash handshake, remote players interpolated | 2 players walk Balmora together |
| N3 | Predicted local locomotion, jump and swim, with reconciliation and a net debug overlay | No visible correction at 100 ms RTT |
| N4 | Predicted actions: attacks, dodge, traversal with warps; host hit authority and lag compensation | Two players fight one NPC; mantles stay correct |
| N5 | World: several R0 regions on the host, physics regions, NPC interest management, props, carry and throw | Players 5 km apart, host frame within budget |
| N6 | Scripts and RPG: host-only MWScript/Lua, context player, routed MessageBox and dialogue, container requests | A scripted main-quest chain completes in co-op |
| N7 | Saves (host world plus per-player character), join in progress, reconnect, clock, rest rule | Save, quit, resume with a guest |
| N8 | Steam transport (lobby, relay, invites) as an opt-in game feature | Invite a friend through Steam |

**Top risks.**
1. Single-player assumptions in MWScript and mods (`Player`, `MenuMode`, pause), which was TES3MP's
   main pain.
2. Host CPU and one-origin physics when players spread out.
3. Predicted root motion and warps, and dodge-vs-hit fairness.
4. Lightyear's Bevy lag and compile time.

**Owner questions (max 3).**
1. **Quest progress:** a shared world with personal characters (Enshrouded; recommended, since
   Morrowind quests are global), or a single quest owner (Skyrim Together)?
2. **When:** right after M6 with the engine-only slices N0-N5, or after M7 (RPG core, saves, AI), since
   N6-N7 need those? Recommended: N0 now (docs only), N1-N5 after M6, N6-N8 after M7.
3. **Transport:** is Steam (lobby, relay, no port forwarding) the target for release, or is direct
   IP/LAN enough at first?
