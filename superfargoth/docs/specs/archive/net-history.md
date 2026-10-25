# net: archive

Moved out of [net.md](../net.md) on 2026-10-26 to keep the spec under the doc budget: history, as-built logs, research and comparisons. net.md and the code win where they differ.

## Industry comparison

| Technique | What shipped games and engines do | Us | Verdict |
|---|---|---|---|
| Authority | Source, Overwatch, UE: server-authoritative; Valheim, TES3MP: distributed authority (zone/cell owner) with known lag and desync bugs | host-authoritative listen server | **match** (mainstream) |
| Prediction | Overwatch: local player only, fixed-tick commands, rollback; UE CMC saved moves; Unity Netcode prediction | own character only, fixed tick, replay unacked commands | **match** |
| Interpolation | Source: ~100 ms interpolation of remote entities; lightyear hermite with velocities | ~100 ms, tuned in N3 | **match** |
| Lag compensation | Source: rewind hitboxes to the shooter's view; Overwatch: favour the shooter with limits | rewind NPCs for player attacks; favour the defender for NPC attacks within 150 ms | **match**, defender side tuned for co-op |
| Networked animation | UE: animation not replicated; montage root motion predicted | move id + clock replicated; pose rebuilt locally | **match** |
| Interest management | UE replication graph / net relevancy; replicon visibility | worldspace + distance; overlay per streamed cell | **match** |
| Progression | Enshrouded: world progress + personal character; Skyrim Together: one quest owner | Enshrouded model (owner) | **match** |
| Content identity | TES3MP plugin list check; naia protocol hash | profile + bake input fingerprints | **match** |
| Pause and rest | Elden Ring Seamless: world-wide resets explicit; Minecraft bed rule | no pause, rest vote | **match** |

## Audit of existing specs against the 8 rules (N0)

Scope: every spec in `superfargoth/docs/specs/` and the game's main specs in
`bevyrrowind/docs/specs/`. **Only physics.md "Controller" was changed in N0** (mover on the fixed
tick). Every other required change is listed here and made in the slice named, in its own spec,
when that slice starts. "A2"/"B1" are action.md phases (M4 P11), before N1.

| Spec | Violation (rule) | Required change | When |
|---|---|---|---|
| physics.md "Controller" | mover in `Update` at frame rate (1) | mover on the fixed tick with interpolation; pushes per tick; co-op ownership line | **done in N0** (spec); built in A2 |
| physics.md "Controller" | `ImpactOnCharacter { entity, .. }`: Bevy `Entity`, no tick (7, 8) | carry tick and the character's stable id; host-only event | A2 or N4 |
| physics.md "Stepping" | one active worldspace steps; one gameplay anchor "the player" drives rebase (5, 6) | one physics region per player cluster, each with its own anchor and rebase (net.md "Multi-region") | N5 |
| physics.md "Carry and throw" | holder view recorded per frame from the camera (`RunFixedMainLoop`) (2); grab, drop and throw are local calls (5) | hold point from `Command.view` per tick; grab/drop/throw as host requests; thrown item pre-spawn with provisional id | N5 |
| physics.md "Ragdolls" | no replication shape for ragdoll bodies (5) | host-only ragdolls; per-body poses replicated at a reduced rate; dragging a corpse is a host request | N5 |
| locomotion.md "Bevy integration" | `ControllerSystems` order is per frame in `Update`; `MoveIntent` written by drivers (1, 2) | already logged in "Action-layer changes (R11b)": fixed tick, intents from `Command` via `sf_action` | A2 |
| locomotion.md "Local avoidance" | `AvoidObstacle` "the camera now, the player later" (6) | one non-reactive obstacle per player character | A2 |
| animation.md "Root motion modes" | `Drive` root delta consumed by the mover a frame later; graph steps with frame `dt` (1, 3) | gameplay root from the action layer (`Progress` mode, decided in "Decided shape (R11b)"); `Drive` only for hosts without an action layer | B1 |
| animation.md "Events" | point and window events fire from the evaluated playhead (3, 8) | state that graph events are presentation only (footsteps, VFX); gameplay events are `ActionEvent`s from baked windows on the action clock | B1 (one line) |
| animation.md "Open questions" (crowd variety) | random start phase and rate per character (4) | hash (stable id) so every peer shows the same phase | N2 |
| scripting.md "As built (L2)" (engine facts, `@sf/world`) | `PLAYER = Handle(0)`, `world.player()` (6) | `world.players()`, `player` on event payloads, `context_player()`; old names alias the party leader | N6 (Lua parked; add no new singleton APIs meanwhile) |
| scripting.md "Execution model" (slots) | `Input` slot turns local device input into gameplay script events (2, 5) | in co-op, gameplay-affecting input reaches scripts as host-side facts from commands or requests; local input events only for UI scripts | N6 |
| streaming.md "Worldspaces at runtime" | "Active: the player's worldspace. Only this one simulates" (5, 6) | several active worldspaces on the host, one per player cluster (net.md "Multi-region") | N5 |
| streaming.md "Worldspaces at runtime" (doors) | crossing swaps the global active worldspace and moves "the player" (6) | per-player crossing: only that player's anchors move; host keeps others' regions | N5 |
| streaming.md "Runtime overlay" | spawn ids from a save-persisted counter: fine for one machine, ambiguous with several (7) | only the host allocates; client pre-spawns use a provisional per-player namespace mapped on confirm | N4 |
| sky-lighting.md "Data model" (`SkyClock`) | `SkyClock { paused }`, clock set locally (5) | host-owned clock replicated; `paused` only in single-player | N7 |
| weather-adapter.md "Scripting (MWScript → overrides)" | `GetWindSpeed` and region from "the player's region" (6); overrides set locally (5) | context player's region; `ChangeWeather`/force overrides host-only and replicated (weather evaluation itself complies) | N6 (scripts), N7 (overrides) |
| water.md / water-adapter.md | wave twin time `t` must match between peers (4); buoyancy steps on every peer (5) | `t` from the replicated sim clock; buoyancy host-only, clients interpolate floating props | N5 |
| ui.md "Data model and moddability" (screens) | `PausePolicy` / `pauses_game()` pause the world (5) | in a co-op session screens never pause the world; menu state per player feeds `MenuMode` | N6 |
| scripting-adapter.md "MWScript semantics" / "The `mw` runtime library" | `Player` reference, `Player->`, `GetPCCell`, `PlaceAtPC`, `OnPC*`, `MessageBox` buttons, `MenuMode` all assume one player (6) | context-player table above; button replies as requests | N6 |
| scripting-adapter.md "Instances, scheduling, state" | script-active region "computed from the player's cell" (6) | union over every player's active cells | N5 |
| actors-adapter.md "As built (`mw_ai`)": `wander`, `PathWalkScaffold` | `WanderPlanner::next(.., rng)` with an unspecified RNG; `update(.., dt, ..)` per frame (1, 4) | host-only; RNG seeded by (placement id, tick); NPC commands on the fixed tick through `sf_action` (action.md "NPCs"); replaced by `sf_ai` in M7 | A2 (tick), M7 (`sf_ai`) |
| physics-adapter.md "Items" (drops, holder limits) | "the player's inventory drop", holder limits for "the player" (6) | per holder character; drop as a host request | N5 |
| ui-adapter.md "Dialogue and journal" | topics "the player knows"; `MenuMode` from the one UI (6) | known topics are world state (shared journal); `MenuMode` per player | N6 |
| bake.md / mods.md | no violation; the join check needs a fingerprint that is the same on every machine | handshake compares the profile fingerprint and bake input fingerprints, never machine-dependent outputs | N2 |

The "physics.md is a follow-up" notes in action.md ("Co-op readiness" rule 1, "Follow-ups") and
locomotion.md ("Action-layer changes (R11b)", "Fixed tick") are resolved by N0's physics.md edit;
drop them when those specs are next touched.

**Already compliant** (designed against the rules or presentation-only): action.md, camera.md,
world-coordinates.md (`WorldPos`), weather/state.md core (pure function of seed, clock, region, overrides),
character.md (assembly and equipment are data; equipment changes become host requests with the
inventory in N6), terrain.md, far-field.md, portals.md (per local view), sky-lighting.md apart from
the clock, smooth-bodies.md, rig-profile.md, diag.md (adds net counters in N2), test-levels.md (both
repos; N1 adds a two-player level), industry-audit.md, mods.md, nif-runtime.md, world-adapter.md,
bake-adapter.md, animation-sources.md.

**Biggest findings.**
1. The mover's frame-rate step (physics.md, locomotion.md) was the one hard blocker for prediction;
   fixed in the spec now, built in A2 before any co-op work.
2. "One active worldspace, one gameplay anchor" runs through streaming.md, physics.md "Stepping" and
   the MWScript active region: the host needs several active regions (N5, the largest engine slice).
3. MWScript's single-player vocabulary (`Player`, `GetPCCell`, `PlaceAtPC`, `MenuMode`, `OnPC*`,
   modal message boxes) needs the context-player rules (N6), TES3MP's main pain; scripting.md's Lua
   API has the same `world.player()` singleton.

