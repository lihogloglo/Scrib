# Co-op networking: host-authoritative 2-4 player co-op

Status: **spec, docs only (N0, 2026-10-19).** Nothing built. Crate choice at N1, code from N2.
Crates (planned): `sf_net` (thin Bevy plugin, N2: session, handshake, replication rules, prediction
glue), on a replication crate picked in N1. Game side: Steam transport as an opt-in feature in the
game repo (N8).
Sources: the R12 co-op research (`../research/coop/README.md`, crate survey, prior art, "Owner
decisions (2026-10-19)"), [action.md](action.md) "Co-op readiness" (R11b), [camera.md](camera.md),
ARCHITECTURE.md §10.

<!-- agent-summary -->
> **Agent summary (2026-10-26).** Status: spec only (N0); nothing built, `sf_net` does not exist. Timing: N1-N5 after the current phase, N6-N8 after M7 (ROADMAP "[p] Co-op N1-N8"); design-only until after Phase 3. Net-ready rules bind every spec.
> Planned: `sf_net` (session, handshake, replication rules, prediction glue) on lightyear, or replicon plus a transport plus our own one-entity rollback, picked in the N1 spike. Decisions: 2-4 players, host-authoritative with client prediction, no lockstep; shared world, own characters; direct IP/LAN first, Steam later as opt-in ("Owner decisions", "Topology and transport", "Quest model").
> Read next: "Net-ready rules" before writing any system spec; "Ownership per system" and "Replication model" for state; "Prediction" for own character; "Crate decision (N1 spike)"; "Open questions (owner)".
<!-- /agent-summary -->

## Goal

Two to four friends play one world together: walk Balmora side by side, fight the same NPCs, split
up across Vvardenfell, finish the main quest together, save and resume. Each keeps their own
character. It must feel like single-player for your own character (no input lag on moving, jumping,
mantling, attacking) and stay correct for the world (one truth for NPCs, quests, items).

Non-goals: competitive PvP, anti-cheat, dedicated servers, more than 4 players, lockstep or
bit-exact determinism (owner decision; GGRS rejected in R12 §1), cross-version play.

## Owner decisions this spec implements

| Decision (2026-10-19) | Here |
|---|---|
| Small co-op, 2-4 players; one player hosts; host-authoritative with client prediction; no lockstep | "Topology", "Prediction" |
| **Shared world, own characters** (Enshrouded model): world and quest state shared, each player keeps their own character, inventory and skills; anyone in the party can advance a quest | "Quest model", "Saves" |
| **Timing**: N0 now (this spec + audit); N1-N5 after M6; N6-N8 after M7 | "Milestone" |
| **Transport**: direct IP / LAN first (relay if needed); Steam lobbies later, opt-in game feature | "Topology and transport" |

## Canonical technique

**Listen server with client-side prediction of the local character and snapshot interpolation of
everything else** (Quake/Source model: Valve's "Source Multiplayer Networking"; Gambetta's
prediction and reconciliation; Overwatch's ECS netcode, Ford GDC 2017, which rolls back only the
local player). The host runs the whole simulation. Each client predicts only its own character
(and the item it holds) from its own commands, and shows every other entity about 100 ms in the
past, interpolated. Progress follows Enshrouded's two layers: world state stays with the host's
world, character state travels with the player.

Rejected: distributed authority per zone or cell (Valheim zone owners, TES3MP cell authority; both
spread lag and desync, R12 §2), scripts on every client (TES3MP's main failure), GGPO rollback of
the whole world (needs bit-exact determinism of streaming, Rapier, AI and scripts).

## Net-ready rules (binding for every spec)

From R12 §4. **Every spec follows these from now on**, and says in a "Co-op" section (or a line in
its data model) how it does, or lists its exceptions. A spec that breaks one goes in the audit
table below with its required change and the slice that makes it.

1. **Gameplay on the fixed tick.** Movement, actions, combat, timers and status effects advance in
   `FixedUpdate` (60 Hz) with a tick counter. Rendering interpolates. No frame `dt` in gameplay.
2. **Input as commands.** Devices and the camera become a per-tick command (move vector, look
   direction, action intents: action.md `Command`). Gameplay reads only commands. AI and scripts
   write the same intents.
3. **State apart from presentation.** Gameplay state is plain `Reflect` + serde components. Pose,
   camera, VFX, sound, UI and first-person arms read it and never write it. Hits come from baked
   tracks indexed by (move, tick), never from the rendered pose.
4. **Pure steps.** The next state comes from (state, command, tick, static world). No wall clock.
   RNG is seeded by (stable id, tick), never `rand::random` or a thread RNG.
5. **One owner per piece of state.** Each system says whether its state is host-only or predicted
   (local character only). Clients change the world only through requests the host validates.
6. **No singleton player.** No `Single<Player>` in gameplay. Use a `PlayerId`. Scripts get a context
   player. Camera, UI and streaming anchors are per local player.
7. **Stable ids and absolute positions.** Anything that matters has a stable id (`PlacementId`,
   spawn id, `PlayerId`, record id), never a Bevy `Entity` on the wire or in saved state. Positions
   are `WorldPos` (grid cell + local offset, world-coordinates.md), never origin-relative.
8. **Events are data.** Hits, deaths, activations and claims carry tick, source and target (stable
   ids), so they can be sent, replayed and de-duplicated.

**Allowed exceptions** (host-only systems whose cadence doesn't affect prediction): MWScript keeps
its per-frame polling on the host (scripting-adapter.md "MWScript semantics" 1; `GetSecondsPassed` is the
host's frame delta), and Lua `on_update(dt)` runs on the host clock (R12 §3). Neither runs on
clients, so neither needs replay. Presentation (graph, cloth, particles, camera, UI) runs at frame
rate everywhere.

## Topology and transport

- **Listen server.** One player's game is the host; every peer runs the same binary and content.
  Single-player is a host with no clients, on the same code path (replicon's model), so co-op code
  is never a separate game mode. Up to 4 players (setting, default 4).
- **Tick.** 60 Hz, the physics and action rate. One `Tick` counter (u32), host-authoritative.
  Clients run their input timeline ahead of the host by about half the round trip plus a small
  jitter buffer, so a command for tick N arrives before the host simulates N; they render remote
  entities on an interpolation timeline about 100 ms behind (tuned in N3).
- **Transport (owner).** Direct IP and LAN over UDP first: host listens on a port, guests join by
  address; LAN discovery by broadcast on the local subnet. A relay only if NAT blocks testers
  (N2 decides). **Steam** (lobbies, relay, invites) is N8, an opt-in feature of the game repo:
  the networking crates are MIT/Apache, the Steamworks SDK is proprietary, so it never enters
  superfargoth.
- **Channels.** Unreliable for snapshots and commands (commands sent redundantly: each packet
  carries the last several unacknowledged ticks); reliable ordered for gameplay events, requests
  and replies; reliable for the join snapshot.
- **Join handshake.** Protocol version, game build id, the mod **profile fingerprint** (mods.md,
  already the bake's "bake needed" key) and the bake manifest's input fingerprint. Any mismatch
  refuses the join with a report naming the differing mods (TES3MP and naia refuse mismatches too).
  Fingerprints compare inputs, not GPU- or machine-dependent bake outputs.

## Players

- **`PlayerId`**: a small index (0-3) assigned by the host at join, used on the wire and in events,
  plus a persistent **player key** (random 128-bit, created once per player profile) used in saves
  to find a returning player's character. The host is a player like the others (usually id 0).
- A player's character is an ordinary character entity with `PlayerCharacter(PlayerId)`. Nothing in
  gameplay queries for "the" player: systems that need people iterate player characters, or take a
  `PlayerId` from a command, event or script context.
- **Per local player** on each machine: input shell (command sampling), camera, UI stack, streaming
  anchors (`Gameplay` anchor + camera anchor), floating origin, audio listener. One local player per
  machine for now; the per-local-player shape keeps split-screen possible.
- **Party leader**: the host's player by default, transferable by the host. Used only where a rule
  needs one person and no context exists (scripts, below).

## Ownership per system

"Host" = only the host simulates and decides; clients receive it. "Predicted" = the owning client
simulates ahead and the host corrects. "Local" = each machine computes its own, never sent.

| System / state | Owner | Clients | On the wire |
|---|---|---|---|
| Own character body (`ActionState` + `Locomotion`, mover, jump, buffers, traversal and docked moves, own attack and dodge starts, own stamina spend) | **predicted** by its player, host-authoritative | simulate own; rewind and replay on correction | commands up; snapshot + ack tick down |
| Other players' characters | host | interpolate | snapshot (action.md "What is replicated") |
| Held item (carry) | predicted with its holder (kinematic in holder space) | simulate own | holder's snapshot + item id |
| Thrown item | host after confirm | thrower pre-spawns, hands over on confirm | request up, body state down |
| NPC AI, perception, threat, dialogue targets, NPC moves | host | interpolate | snapshot: `WorldPos`, facing, move id, start tick, clock rate, locomotion values, health, equipment |
| Hits, damage, reactions, deaths, status effects | host (player melee contacts are attacker-claimed, host-checked: "Hit authority") | claim own melee touches, predict own hit-stop; play reactions when the host event arrives | claims up; events down |
| Dynamic props, debris, ragdolls, buoyancy | host (Rapier) | interpolate, never roll back | body state (`WorldPos`, rotation, velocity, asleep); ragdolls: per-body poses at a reduced rate |
| Spawn, despawn, enable/disable, placement overlay | host | apply | overlay deltas per cell; live spawns |
| Doors, levers, locks, seat and interaction claims, paired starts (`grab_check`) | host | request; predicted wind-up only | request/reply + state |
| Inventory, containers, equipment, barter, pick-up | host | request | per-player inventory to its owner; equipment to all |
| Character stats, skills, spells, factions, bounty (per player) | host during the session | read own | own sheet to its owner; visible bits to all |
| Quests, journal, known topics, globals, script variables | host (shared world) | read | world-state deltas, reliable |
| MWScript and Lua gameplay scripts | host | none (UI scripts run locally) | effects as state; per-player effects as messages |
| Game clock, time scale, rest/wait, weather overrides | host | compute weather, wind, sky, water waves locally from clock + seed + overrides | clock + overrides |
| Streaming rings, floating origin, render, LOD | local | own anchors | nothing |
| Static world, records, bake, generated content (grass, scatter, wind lattice) | local, identical (handshake) | compute | nothing |
| Animation graph, IK, cloth, smooth bodies, VFX, sound, camera, first-person arms, UI | local presentation | derive from replicated state + clip clock | nothing |

## Replication model

- **Base: replicon's vocabulary** (both crate paths share it, R12 §4): replicated components and
  rules per entity kind, server-authoritative replication, per-client visibility, remote events
  and triggers, listen-server mode. Gameplay components are already plain serde data (rule 3), so
  a replication rule is a registration, not a rewrite.
- **Identity.** Placed objects use their `PlacementId` (streaming.md: blake3 of the content key).
  Runtime spawns use the overlay's spawn ids (streaming.md "Runtime overlay": hash of a
  save-persisted counter), **allocated only by the host**; a client pre-spawn (thrown item, predicted
  projectile) carries a provisional id in its player's namespace that the host maps on confirm.
  Players use `PlayerId`. Bevy `Entity` never crosses the wire; each peer keeps a stable-id → `Entity`
  map, as scripts already do (scripting.md "Entity handles").
- **Positions** go out as absolute `WorldPos` (cell + offset), quantised (1 mm, rotation as a
  smallest-three quaternion). Each peer converts to its own floating origin.
- **Two kinds of world state, two rules.**
  - *Live entities* (characters, NPCs, awake props, ragdolls) replicate only inside the client's
    **interest set**: same worldspace, within about the R1 radius of the client's player (tuned in
    N5). Interiors are separate worldspaces, so they filter for free.
  - *Persistent changes* (placement overlay: removed, moved, spawned, state blobs; lock and door
    state) replicate **per cell when the client's streaming reaches the cell** (a per-cell delta on
    load, then live updates while resident), so a looted shop or a disabled wall shows correctly at
    any distance, without streaming NPCs there.
- **World-level state** (globals, quest indices, journal, topics, clock, weather overrides, party
  list) goes to every client, reliable.
- **Events** (`ActionEvent`, hits, deaths, journal updates) are host messages carrying tick, source
  and target; clients de-duplicate by (tick, source, kind).
- **Requests** (client → host): interact, claim, pick up, drop, container and barter operations,
  equip, dialogue choice, MessageBox button, rest vote, throw. The host validates (distance, line of
  sight from its own state, claims, inventory, the player's menu state) and replies accept or
  refuse; a refusal undoes any predicted presentation.
- **Join in progress**: the host sends world-level state, the client's own character, and the
  snapshot of its interest set; persistent per-cell deltas follow as its streaming loads cells.
  replicon does the snapshot natively.
- **Bandwidth target** (to measure in N1, tune in N5): 4 players with about 200 NPCs near them,
  ≤ 256 kbit/s down per client typical, ≤ 64 kbit/s up. Levers: interest radius, snapshot rate
  by distance (near 30 Hz, far 10 Hz), delta compression, sleeping bodies send nothing.

## Prediction (own character only)

- **Command flow.** Each tick the input shell samples one `Command` (action.md: tick, `PlayerId`,
  stick in world yaw, view yaw/pitch, button edges, hold ticks, lock request), applies it locally,
  stores it with the predicted state, and sends it with the last unacknowledged ones.
- **Host.** Applies each player's command on its tick. A missing command repeats the last stick and
  view for up to 6 ticks with no new presses, then becomes empty (no stuck attacks).
- **Reconciliation.** The host's snapshot of a player's own character carries the last applied
  command tick. The client compares it with its stored prediction for that tick; beyond a small
  tolerance (position 1 cm, move id or clock differing), it rewinds to the host state and replays the
  unacknowledged commands in the same frame. This is why the mover and the chooser must run on the
  fixed tick (physics.md "Controller", revised in N0). The render body's correction spring
  (locomotion.md) hides the visual error; it never feeds back.
- **What the client collides with while predicting**: its local static world (identical by the
  handshake), props and other characters at their interpolated poses. It never pushes props.
- **Root motion and warps** (R12 §3): root motion is a pure function of (clip, ticks elapsed, warp)
  read from baked root tracks, so a replay repeats the path. Warps onto static targets (ledges,
  seats from the bake) predict; paired moves onto NPCs are host-started (the client predicts only
  the wind-up); lunges toward an NPC aim at its interpolated pose and accept a small correction.
- **Not predicted**: hits and damage, NPCs, props, claims, inventory, anything another player owns.

## Hit authority and lag compensation

Adopted 2026-11-26 (combat decision 15, spec slice C0e; built in combat C7, net N4): **attacker-claimed hits with a host
plausibility check** for player melee, host sweeps for the rest. The N0 rewind-first design is in the
[archive](archive/net-history.md#hit-authority-n0-replaced-2026-11-26-by-c0e); its rewind buffer is the fallback below.

- **Player melee: claimed.** The attacking client sweeps (combat.md "The per-tick pipeline" 5-6) on its own predicted
  state against defenders as it sees them and sends each first touch on that tick's command packet (re-sent with the
  unacked ones): `HitClaim { tick, attacker: PlayerId, defender (stable id), swing (move start tick), move_key, hit_row,
  point (attacker-relative), region }`. The client predicts
  only hit-stop and spark (combat.md open question 1); damage, reactions and deaths wait for the host.
- **Host check**, run when the host applies that player's command for `tick` (a later claim, within
  `claim_window_ms`, reads a short history of that player's move, swing, windows and root pose): (1) the tick is
  within the window; (2) on the host's state of that player, `move_key` and `swing` are the running move and `hit_row` is
  open at `tick`; (3) `point` lies within `reach_slack_m` of the host's weapon segment and within `range_slack_m` of the
  defender's host hurt shapes, and the attacker-to-defender bearing within `angle_slack_deg` of the claim's; (4) it is the
  first claim of `(attacker, swing, hit_row, defender)`, combat.md's per-pair rule; (5) the defender has no `IFrames` for
  the row's flags at any host tick in `tick ± iframe_slack_ms` (doubt favours the defender). Pass: the host fills
  `defender_state` (guard, parry, critical) from its own state, applies friendly fire (combat-delivery.md "Friendly
  fire"; co-op switch off by default: player-on-player claims drop), runs `rules.resolve_contact`, applies the `Outcome`
  and the attacker's hit-stop at `tick` (an accepted claim never corrects) and broadcasts it. Fail:
  `ClaimRefused { tick, swing, hit_row, defender, reason }`; the predicted hit-stop reconciles like any misprediction
  ("Prediction"); refusals counted per reason.
- **NPC → player: host sweep, defender favoured.** An NPC contact on a player at host tick H waits that player's view
  lag (half RTT + interpolation delay), capped at `defend_favour_ms`, and resolves against the player's state at H plus
  that lag: a dodge, guard or parry started in answer to the swing they saw counts. (PvE: only the human can be
  treated unfairly.) NPC → NPC and environment contacts: host, no wait.
- **Projectiles: host entities, fast-forwarded.** A client's release command spawns the projectile on the host from its
  release state at the command tick (origin, aim, strength), advanced at once by that client's half RTT capped at
  `projectile_ff_max_ms`, each skipped tick swept against current host state; past the cap the shot starts late instead
  of reaching further back. Client copies: combat-delivery.md "Net / co-op". Beams, areas and touch casts
  start on the host when the command arrives; NPC shots are never fast-forwarded.
- **Host state only:** pools, poise, buildup, tokens (combat-npc.md `TokenBook`), criticals, reactions, deaths, fates.
- **Rewind fallback** (`net.combat.rewind`, off). Only if C7 measures the slacks refusing fair hits or
  accepting visible misses. Check (3) then becomes an exact re-sweep of the claimed segment against the defender
  rewound to what that client saw: a ring buffer of each NPC's (`WorldPos`, facing, move id, clock) per tick,
  `rewind_depth_ms` deep. Hit shapes come from baked tracks too, so rewinding stores state, not poses.
- **Cheating stance.** PvE co-op among friends; anti-cheat is a non-goal ("Goal"). The checks stop desync and obvious
  cheats (reach, wrong move, repeats), not a determined cheater. PvP would need host sweeps with rewind (note only).

**Tunables** (`net.combat.*`, net `Gameplay`, host settings; all `Guess` until C7 measures them under the N1 link
conditioning): `.claim_window_ms` (ms, 0-250, 100), `.reach_slack_m` (m, 0-0.3, 0.05: the attacker's state is the host's
own), `.range_slack_m` (m, 0-2, 0.75: about 150 ms of view lag at a 5 m/s run), `.angle_slack_deg` (deg, 0-90, 30),
`.iframe_slack_ms` (ms, 0-100, 50), `.defend_favour_ms` (ms, 0-250, 150: R12's cap, kept), `.projectile_ff_max_ms` (ms,
0-250, 150), `.rewind` (bool, false), `.rewind_depth_ms` (ms, 100-500, 250).

Industry comparison (FromSoft, Destiny/Warframe, Overwatch, Source): [archive](archive/net-history.md#industry-comparison).

**Reuse.** Bevy 0.20: nothing for hits. `lightyear` 0.31.0 (2026-10-21, Bevy 0.20): lag compensation only in
`lightyear_avian3d` (`LagCompensationHistory`, `LagCompensationSpatialQuery` over Avian colliders), unusable with our
Rapier-direct mover and baked-track hit shapes. `bevy_replicon` 0.45.8 (2026-11-24, Bevy 0.20): replication only, no
rewind. So `HitClaim` and a pure `check_claim(history, claim, slacks) -> Result<(), Refusal>` in `sf_combat` are ours.

**Tests (C7).** `check_claim` exact cases: on time accepts; late tick (`Late`), wrong move id or swing
(`NotRunning`), row closed (`Window`), defender beyond the range slack (`Range`), bearing off (`Angle`), a second claim of
the same (swing, row, defender) (`Duplicate`, while the next row accepts), defender rolling (`IFramed`), player defender
with the co-op switch off (`Friendly`). Defender favour: a dodge inside the lag window i-frames the hit, one past the cap
does not. Fast-forward: the host shot equals a spawn at the client tick advanced k ticks; the cap clamps k. Replay
determinism: the same command and claim streams, arrival shuffled within a tick, give identical `Outcome`s and pools;
serde round trip of `HitClaim` and `ClaimRefused`.

## Multi-region host simulation

- The host keeps R0 (collision, physics, scripts, AI) active around **every** player: each player
  has a `Gameplay` streaming anchor on the host (streaming.md already allows several anchors). The
  host renders only around its own local player; remote regions run without visuals. Gameplay
  never needs evaluated poses (action.md), so remote regions skip the animation graph except for
  powered ragdolls.
- **Physics regions.** One Rapier world with its own anchor and rebase per **cluster** of players:
  players whose R0 areas overlap (same worldspace, within about 1 km) share a region; clusters merge
  and split with hysteresis (merge under 1 km, split over 1.5 km). Each interior a player is in is
  its own region. `sf_physics` already hides Rapier types and Rapier supports independent worlds;
  a Rapier f64 build is the fallback if region handover proves worse (N5 measures both).
- **Script-active region**: the union of each player's MWScript active cells (3×3 around each
  player's exterior cell, or each player's interior).
- **Budget**: the host's frame stays within budget with 4 players in 4 separate regions (N5's
  done-when). Levers: AI LOD and actor tiers per region, far-tier movers every 2nd or 4th tick.
- **Door crossing** is per player: a player entering an interior moves only their own anchor; the
  host never swaps a global "active worldspace".

## Time, pause and rest

- The host owns the game clock (`SkyClock`: days, time scale) and replicates it. Weather, wind, sky
  and water waves are pure functions of (seed, clock, region, overrides) and run locally (weather/state.md,
  water.md); only overrides (`ChangeWeather`, forced presets) are sent. Real-second ambience timers
  are local presentation.
- **No world pause in co-op** (Elden Ring, Valheim). Menus are per player; a player in a menu stays
  in the world (a guest could be hit while in the inventory). Single-player keeps pause.
- **Rest and wait** advance the shared clock, so they are a party vote: a player requests a rest of
  N hours, every other player accepts in their own UI or is already resting (Minecraft's bed rule);
  the host then advances the clock for everyone. Any world-wide reset is an explicit, announced
  event (Elden Ring's lesson).

## Scripts and the context player

- **Gameplay scripts run only on the host** (MWScript and Lua). Effects replicate as ordinary state:
  enable/disable, position, inventory, globals, journal. UI scripts (the `Ui` slot) run locally.
- **Context player.** Every script run has an optional context `PlayerId`:

| Situation | Context player |
|---|---|
| Activation (`OnActivate`, default activation), pick-up, `OnPCAdd`, `OnPCDrop`, `OnPCEquip`, `OnPCHitMe`, `OnPCSoulGemUse`, `OnRepair` | the acting player |
| Dialogue filters and results, barter, training, persuasion | the talking player |
| Trigger volumes, Lua events with an actor | the player who caused it |
| Local script, no event (distance checks, `GetDistance Player`, `PlaceAtPC`, `GetPCCell`) | the **nearest player** to the script's object (same worldspace first) |
| Global script, no event | the **party leader** |

- **`Player` resolves to the context player.** `Player->X`, `GetPC*` and `ModPC*` functions,
  `AddItem`/`RemoveItem` on `Player`, faction joins and rank changes act on that player's character.
- **Per-player effects are messages to that player**: `MessageBox` (a timed notification goes to
  the context player; a modal with buttons goes to the context player, its reply comes back as a
  request and `GetButtonPressed` reads it), sounds and music cues, fades, `Enable*Menu`, journal
  popups (journal popups go to every player, since the journal is shared).
- **`MenuMode`** is true only if the context player has a menu open; with no context it is false
  (the world doesn't pause).
- **Lua**: `world.player()` and the `PLAYER` handle become `world.players()` plus a `player` field
  on event payloads and an explicit `context_player()`; the API is versioned (scripting.md), so the
  single-player names stay as aliases for the party leader with a deprecation warning.
- Luau stays out of the per-tick prediction loop (action.md).

## Quest model (owner: shared world, own characters)

- **World state** (host's world, shared by all): journal entries and indices, quest stages, known
  dialogue topics, globals, script variables, NPC states (dead, disabled, moved, dispositions'
  base), containers, the placement overlay. **Anyone in the party can advance a quest**: their
  activation or dialogue runs the same host script; the journal update goes to every connected
  player. A player who was away finds the journal as it is.
- **Character state** (per player, travels with them): attributes, skills, level, health and
  magicka, spells, inventory and equipment, gold, faction memberships and rank, reputation, bounty,
  diseases and vampirism, appearance, known recipes later.
- **Rewards** go to the context player (the one who talked or acted): `AddItem Player`, gold,
  skill rewards, faction advancement. Sharing loot is the players' business (drop and pick up).
- **Faction-gated quests** check the talking player's rank (dialogue filters read the context
  player); a quest step done by one member counts for the world.
- **Concurrency**: one conversation per NPC at a time (a claim; others see the NPC busy); the host
  applies requests in arrival order on its tick, so two players can't both take a unique reward.
- Personal-progression mode (Enshrouded's frequent request) is not planned.

## Saves

- **Host world save** (`sf_save`, M7): placement overlay, entities, script state, quests, journal,
  topics, globals, clock, weather seed and overrides, NPC states, containers, and the **last known
  character state of every guest** (keyed by player key) for reconnects.
- **Per-player character save**: the character state above plus position (worldspace + `WorldPos`)
  in that world, stored on the player's machine, keyed by (world id, player key). The host's copy
  wins inside its world (one truth per world); the guest's copy is the backup if the host has none.
- Co-op world saves live apart from single-player saves (Elden Ring's `.co2`).
- The host autosaves on a timer and when a guest leaves; the leaving guest gets their character
  file. Join in progress and reconnect restore the guest's character from the host's copy.
- A new guest creates a character in the host's world (character creation runs as for a new game,
  placed near the party leader). Importing a character from another world: see open question 1.

## Crate decision (N1 spike)

Spike pending: lightyear (on replicon) versus replicon + a transport (renet2 or quinnet) + our own one-entity rollback.
Rule: lightyear if it passes Bevy lag, listen-server quality, non-Avian prediction and build cost; otherwise the second.
Criteria, setup and link conditioning: [history](archive/net-history.md#crate-decision-n1-spike-moved-2026-11-26).

Moved to [history](archive/net-history.md#industry-comparison).

## Milestone (co-op N slices)

Each slice is 1-2 steps, verified and committed before the next (R12 §4 outline, owner timing).

| Slice | When | Content | Done when |
|---|---|---|---|
| **N0** | now | this spec and the audit; physics.md "Controller" on the fixed tick | audit committed; §10 entry |
| N1 | after M6 | crate spike (above) | crate picked, §10 entry |
| N2 | after M6 | `sf_net`: host/join over UDP (direct IP, LAN discovery), handshake, `PlayerId`, remote players interpolated, net counters | 2 players walk Balmora together |
| N3 | after M6 | predicted locomotion, jump, swim; reconciliation; net debug overlay | no visible correction at 100 ms RTT |
| N4 | after M6 | predicted actions (attacks, dodge, traversal with warps); attacker-claimed hits with host checks, projectile fast-forward (combat C7); host spawn ids | two players fight one NPC; mantles stay correct |
| N5 | after M6 | multi-region host, physics regions, interest management, overlay per cell, props, carry and throw, ragdolls | players 5 km apart, host frame within budget |
| N6 | after M7 | host-only MWScript and Lua with the context player; routed MessageBox and dialogue; container and barter requests; per-player menus | a scripted main-quest chain completes in co-op |
| N7 | after M7 | host world save + per-player character, join in progress, reconnect, clock, rest vote | save, quit, resume with a guest |
| N8 | after M7 | Steam transport (lobby, relay, invites), opt-in game feature | invite a friend through Steam |

## Open questions (owner)

1. **Bringing characters in**: may a guest import their single-player character (stats, inventory,
   no quests) into a co-op world, or only create one there (default here; avoids a level-50 guest
   in a new world)?
