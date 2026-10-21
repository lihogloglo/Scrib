# Action layer: archive

Moved out of [action.md](action.md) on 2026-10-21 to keep the spec under the doc budget:
history, background and pre-build sketches. action.md and the `sf_action` code win where
they differ.

## Decisions this spec implements (R11b)

From `11-tradeoffs.md` "Decisions". Each line names where it lands.

| Decision | Here |
|---|---|
| Third person primary; one gameplay body decides hits in both views | "Co-op readiness", camera.md |
| Co-op 2-4, host-authoritative with client prediction; design now | "Co-op readiness" |
| Ledges first (grab, hang, shimmy, mantle, ladders with IK); hold index baked for later | taxonomy, phases B1-B4 |
| Jump at a waist-high wall mantles in normal play with forward input; sneak never triggers traversal; separate up/down intents; assisted/manual setting | "Intents", "Mode profiles" |
| Free-run = hold sprint | "Mode profiles" |
| Missed heavy attack: full recovery, no cancel | "Category default windows" |
| Low stats: smaller magnitudes, all moves available; never timing | "Stat bands" |
| Content-pack first-person clips: optional presentation on the shared clock | camera.md, animation.md |
| NPCs use the same move set; traversal AI later | "NPCs" |
| Clip gaps: packs only, free preferred; no owner mocap; first takedown uses a ragdoll victim | "Clip gaps" |
| Defaults: motion owner per kind, no steering mid-mantle; traversal abortable only in its tail; blending (gap-sized inertialization, pose-matched entry); free camera, soft aim in FP, optional lock-on in TP, framed camera for paired kills; bones-only clips + one moveset line per attack slot; one scale with mode profiles, crowd-scaled hit-stop; one priority list, traversal breaks lock-on, a drawn weapon doesn't block traversal, drop/air kills after the first combat milestone; data move rules in a Rust action layer | throughout |

## Canonical technique: background

**"The decider decides, the graph plays, clip windows gate."** This is the FromSoftware split
(R5 §1): a decider picks every transition, the animation graph only plays and blends, and timed
windows on clips say when a choice is allowed. Our decider is not a script. It is a Rust layer
evaluating **data move rules** on the **fixed tick** (R1 adopt 7, R5 §7), with a bounded settle
loop. UE's Game Animation Sample (GASP) has the same shape for traversal: probes, then a chooser
table, then motion warping (R7 §1.2). Luau never runs inside the per-tick loop (Lua is parked, and
a script slot runs a frame late, which makes controls mushy: R5 §7).

## Pre-build sketches (A1 code supersedes)

### Command

```rust
pub struct Command {
    pub tick: Tick,                    // u32 fixed-tick counter
    pub player: Option<PlayerId>,      // set for people (local or remote); None for AI and scripts
    pub stick: Vec2,                   // move input, already in world yaw (camera-relative resolved by the shell)
    pub view: ViewDir,                 // look direction (yaw, pitch): aim and soft-aim input, never the camera pose
    pub buttons: ButtonEdges,          // pressed / held / released this tick, per action class
    pub hold_ticks: [u16; CLASSES],    // how long each class has been held (charge, sprint, tap vs hold)
    pub lock_request: LockRequest,     // None | Toggle | Next(side)
}
```

Presses between ticks accumulate in the shell and are consumed on the first tick that samples them
(R1 adopt 2). Commands are plain data (`serde`, `postcard`), small, and logged by the flight
recorder.

### ActionState

```rust
pub struct ActionState {
    pub mov: MoveRef,                  // current move rule id + slot chain index
    pub clock: ActionClock,            // ticks since move start, rate (hit-stop), progress for Warp/Docked
    pub mode: ModeFlags,               // Sneak | Normal | FreeRun, plus Combat stance
    pub buffer: InputBuffer,           // latched requests per class (two-stage, below)
    pub timers: Timers,                // coyote, jump buffer, redo cooldowns: all in ticks
    pub targets: ActionTargets,        // stable ids: interaction point, edge + offset, lock-on, partner; warp target as WorldPos
    pub plan: Option<Plan>,            // Warp: start root, target, correction, path; Docked: edge, s, side
}
```

### The chooser, step by step (design; A1 built it, see action.md "As built (A1)")

Runs in `sf_action::step` for each character with an action state, once per fixed tick (60 Hz), in
this order:

1. **Command.** Take this tick's `Command` (none: an empty one).
2. **Clock.** Advance the action clock by one tick × the character's clock scale (hit-stop).
   Collect the windows open now.
3. **Buffer** (two-stage, SK model, R6 §3):
   - a press of a class is **latched** only inside an `accept` window for it (or an unrestricted
     clip); earlier presses are dropped, so early mashing does nothing;
   - a latched request **fires** only inside a `cancel` window for its class;
   - no timer; starting any move clears all latched requests; a `Guard` request fires only if
     still held; `Dodge` fires on release of a tap, holding becomes `Sprint` (one button, if bound
     so).
   - Jump buffer and coyote time are ticks on `Timers` (R1 adopt 3): a Jump latched up to ~6 ticks
     before landing fires on the landing tick; a Jump within ~6 ticks after leaving an edge is a
     ground jump. Values tuned on `flat-slopes`.
4. **Candidates.** Rules whose classes have a firing request, plus forced rules (landing,
   reactions) and automatic rules allowed by the mode. Skip rules on a redo cooldown.
5. **Probes on demand.** Gather the probes the candidates need and run them in one batch
   (`ContextProbes`, state-gated, R5 adopt 7). The player is probed every airborne tick; NPCs only
   when blocked or when a rule asks (locomotion.md "Context probes").
6. **Pick.** Walk the **priority list** below; inside a category, by `rank`. The first rule whose
   entry passes **and** whose validators pass wins. A refusal is logged (rule, failing predicate)
   for the course tests and the debug overlay. No winner while a request is pending: keep it
   latched; for traversal requests, play the no-move feedback (a short additive "can't" reach,
   R3 adopt 2).
7. **Start.** Set the move, clear the buffer, set targets and plan, apply root impulses due on the
   first tick. The **first response** (pose change, sound or camera kick) is emitted this tick
   (R8 adopt 1).
8. **Settle.** If the new move ends or exits on this same tick (zero-length enters, instant
   chains), repeat 4-7, at most 4 rounds (R5 §1's bounded settle).
9. **Move.** Step the gameplay body by the owner: Sim → `sf_controller` mover from the intent;
   Warp → planned path + warped baked root track at the progress; Docked → edge parameter;
   Clip → baked root track with turn-rate and lunge windows; Physics → nothing (the ragdoll owns it).
10. **Output.** Write `ActionOutput` for presentation: move, slot chain, playhead (seconds or
    progress), blend hint (gap-sized), IK targets (lip, rungs, handles), look limits, camera hints,
    `ActionEvent`s (tick, source, target, kind: sounds, VFX, noise, contacts). The graph and camera
    read it; they never write back.

### Input buffering and first response: tests

Scripted-input tests (R6 adopt 7, R8 adopt 1), headless in `sf_action/tests/`:
- every rule: press → first response within 1-3 ticks;
- a press inside `accept` fires exactly on the `cancel` tick; a press before `accept` is lost;
- Guard fires from the buffer only if held; Dodge fires on tap release;
- jump buffer and coyote windows, in ticks;
- same results at 30, 60 and 144 render fps (the fixed tick is the clock).

## Co-op readiness: full text

Owner decision: 2-4 player co-op, host-authoritative with client prediction, no lockstep, no
bit-exact determinism. The R12 co-op research (`../research/coop/README.md` §4) sets eight
net-ready rules for every spec and leaves the crate choice (replicon vs lightyear) to the co-op
milestone (N0-N3). This layer follows all eight:

1. **Gameplay on the fixed tick.** The chooser, the mover, action clocks, buffers, cooldowns and
   hit-stop advance in `FixedUpdate` with a `Tick` counter; rendering interpolates. No frame `dt`
   anywhere in `sf_action`. **Required change:** the kinematic mover runs in `Update` at frame rate
   today (physics.md "Controller", locomotion.md "Movers"); it moves to the fixed tick in phase A2
   (locomotion.md "Action-layer changes (R11b)"). physics.md is not edited by this spec: follow-up.
2. **Input as commands.** `Command` is the only input. Devices and the look input become a command
   in the shell; AI and scripts write the same `Command`. A remote player is just another character
   whose commands arrive over the network.
3. **State apart from presentation.** `ActionState` + `Locomotion` are plain serde (and `Reflect` in
   the Bevy shell) data. The graph, IK, render bodies, first-person arms, the content pack's
   first-person clips, camera, VFX, sound and correction springs read `ActionOutput` and never
   write gameplay state. Hits come from baked weapon socket tracks indexed by (move, tick), never
   from the rendered pose.
4. **Pure steps.** `step(state, command, tick, world queries) → (state, output, events)`. No wall
   clock. Random choices (variant picks, AI-free jitter) hash (seed, character id, tick); never a
   thread RNG. World queries hit static baked data (the same `StaticWorld`/physics statics on every
   peer) plus replicated dynamic state.
5. **One owner per piece of state.** Listed below: predicted (the local character) or host-only.
   Clients change the world (claim a seat, pull a lever, open a door, start a paired move) only
   through requests the host validates; a rule's `Claim` and `PairFits` validators become host
   requests in co-op.
6. **No singleton player.** `sf_action` has no "the player": every character is the same. Commands
   from people carry a `PlayerId`; camera, input shell and UI are per local player; scripts that
   react to moves get a context player.
7. **Stable ids and absolute positions.** `ActionTargets` holds stable ids (character, interaction
   point, edge, lock-on target, partner), never a Bevy `Entity`. Plans and warp targets store
   `WorldPos` (grid cell + local offset); character-space conversion happens inside the step.
8. **Events are data.** `ActionEvent { tick, source, target, kind }` for move start/end, contacts,
   hits, claims and noise, so they can be sent, replayed and de-duplicated.

**What is predicted on the client (own player only):** locomotion and mover state, jump, coyote
and buffers, traversal and docked moves (validated against the same static world), own attack and
dodge starts and clocks, own stamina spend. Mispredictions roll back to the host's acknowledged
tick and replay unacknowledged commands; small errors are hidden by the render-body correction
spring (the same spring locomotion uses).

**What only the host decides:** hits, damage, reactions and deaths; interaction claims (seats,
doors, levers) and door/lever state; paired starts (the attacker's lunge is predicted and masks the
round trip; `grab_check` waits for the host); lock-on target validity; AI.

**What is replicated to the other peers:** per character, a snapshot of the gameplay body:
position, velocity, facing, move id, move start tick and clock rate, mode flags, targets (by id:
interaction point, edge + offset, lock-on, partner). Peers interpolate remote characters and
re-derive everything visual from the move id and clock (presentation is deterministic enough
from those). Events with gameplay meaning (hits, claims) are host messages.

**Left to the co-op milestone (R12 §4, N0-N3):** crate (replicon base; lightyear or our own
one-entity rollback for prediction); tick rate under network (60 Hz assumed); dodge fairness under
latency (host rewind of NPCs vs trusting a client's dodge within a bound); bandwidth for 4 players
and ~200 nearby NPCs; join in progress; saves in co-op.

**Follow-ups outside this spec:** physics.md "Controller" still says frame-rate mover (rule 1);
N0's spec audit should fix it together with locomotion.md.

## Crate choice (ARCHITECTURE §8b)

Section title was: Crate: new `sf_action` (+ `sf_action_bevy`), not a module of `sf_controller`

ARCHITECTURE §8b rules:
- **Rule 5 (split by change frequency).** Move rules and the chooser will change daily through P11
  and M10. `sf_controller` has five dependents (`sf_controller_bevy`, `sf_physics`,
  `sf_physics_bevy`, `sf_level`, and through them the app). A module there would rebuild physics on
  every chooser edit.
- **Rule 3 (wide and shallow).** The chooser needs compiled windows and baked root tracks
  (`sf_anim_graph`, `sf_anim`). `sf_controller` doesn't depend on `sf_anim_graph` today; adding it
  would lengthen `sf_physics`' chain. `sf_action` sits beside `sf_physics` instead.
- **Rule 1-2 (Bevy-free core, thin shell).** `sf_action` depends on `sf_controller`,
  `sf_anim_graph`, `sf_anim`, `sf_records`, later `sf_interaction`; headless course tests use
  `sf_level` as a dev-dependency. `sf_action_bevy` is a leaf plugin over `sf_controller_bevy` and
  `sf_anim_bevy`: command sampling (from `bevy_enhanced_input` actions, ui.md), the fixed-tick
  schedule, the `ActionOutput` → graph feed, gizmos and counters. Nothing depends on it but the app.

## Clip gaps (packs only, free preferred; no owner mocap)

From R9 §4, by phase. "BYO" = usable in the user's own bake, not shippable raw (Mixamo, LaFAN1).

| Phase | Needs | Free sources | Gap |
|---|---|---|---|
| A1-A2 | locomotion, jump, land, roll | UAL1 (CC0), 100STYLE (CC BY) | starts/stops per foot: cut + mirror |
| B2 | step up, vaults, mantles (waist, high) | UAL1 `ClimbLedge`; Mixamo (BYO) | **consistent-style vaults and top-outs**; UAL2 "parkour" (paid) only after checking its clip list |
| B3 | ledge catch, hang, shimmy, climb up, drop to hang | UAL1 climb set; Mixamo braced/free hang, shimmy, hang-to-climb (BYO) | catch from a jump; hang and shimmy as IK on a few base poses (R9 §4) |
| B4 | ladder climb, enter/exit top and bottom | Mixamo ladder (BYO); UAL climb loops | **ladder enter/exit** |
| C1 | sit, stand, door, lever | UAL1 sitting, interact, push | sit by approach angle and doors by side: warp + turn + IK |
| E | guard hits, deflects, posture breaks, directional dodges and staggers | UAL1/UAL2, Mixamo packs, the content pack's own clips | listed; paired takedowns: **none free**, ragdoll victim first |
| D2 | first-person arms | derived from third person; content pack's own FP clips as presentation | quality of derived FP traversal |

## Industry comparison

| Technique | What shipped games and engines do | Us | Verdict |
|---|---|---|---|
| Decider | FromSoftware: per-archetype script decides, behavior graph plays, clip event windows gate (R5 §1); AC1: code + tables (R3 §7) | data rules in Rust on the fixed tick | **match** in shape; moddable without code |
| Traversal choice | UE GASP: traces → chooser table by height/depth/speed → motion warping (R7 §1.2) | probes → rules with bands → validators → warp | **match**, plus AC1 end-pose validation |
| Input buffer | Sekiro: latch in accept, fire in execute, no timer, fixed candidate order; Elden Ring: one slot, newest wins (R6 §3) | Sekiro's model | **match** (ER's slot rejected: a dodge can't wait behind an attack) |
| Commitment | Souls: windows per clip; Ghost of Tsushima: cancellable heavies; DL: free flow (R8 §1.3, R7 §3) | windows per category + moveset; souls heavies | **match** (owner chose souls heavies) |
| Context vs explicit | AC Shadows: up/down intents, auto only while sprinting; DL2: assisted/manual (R7 §3, §2.2) | same, plus Jump-at-wall mantles in Normal | **match** |
| Networked root motion | UE Character Movement: root motion from montages is replicated and predicted; free-form root motion is not network-safe | gameplay root from baked tracks on the action clock; graph presentation-only | **match** |
| Client prediction | host-authoritative prediction + reconciliation (Quake/Source model; UE CMC saved moves) | commands per tick, replay unacked commands, correction spring hides error | **design only**; crate chosen at the co-op milestone (R12 §4) |

Sources: notes R1-R10 in `../research/animation/` (each cites its talks and repos); UE Character
Movement networking and root motion source docs (to be re-checked by R12).

## Build plan: original A1 row

| Slice | Steps | Verify |
|---|---|---|
| **A1** backbone | `sf_action` crate: `Command`, `ActionState`, `MoveRule` RON + validation, priority list, chooser steps 1-10 with Locomotion and Air rules only (jump by height, coyote, jump buffer, landing bands) | headless on `flat-slopes`: first response ≤ 3 ticks; jump buffer and coyote fire on the right tick; identical outcomes at 30/60/144 Hz render; rule load errors listed |

## Later-phase sketches (move back into action.md when the slice starts)

### Stat bands (B5)

Stats scale **magnitudes inside bands, never timing or latency** (decision). The content pack maps
its stats to normalised channels in 0..1; the framework maps each channel to a band per parameter.

```ron
StatBands(
    channels: {
        "jump":     [(param: JumpHeight, band: (0.6, 1.4))],     // m at 0 and 1; mantle classes follow
        "run":      [(param: RunSpeed, band: (0.85, 1.15)), (param: SprintSpeed, band: (0.85, 1.2))],
        "climb":    [(param: DockedSpeed, band: (0.8, 1.2))],
    },
)
```

- Probe classes and warp refusal follow the character's jump height (what is a mantle from a jump
  depends on it).
- Course tests run at both ends of every band.
- Never scaled: windows, buffers, latency, blend times, clip playback rate of committed moves.
- Damage, stamina and reach belong to the combat spec, same rule.
- An optional stat gate per rule exists for content packs; Bevyrrowind uses none (decision: low
  Acrobatics jumps lower, all moves available).

### Interaction points and edges (B4, C1)

Planned crate `sf_interaction` (M4), Bevy-free data:
- **Interaction point**: `{ transform, tag, slots: [(approach: WarpTarget, side)], claim }` on an
  object (seat, door, lever, counter). A rule's `Claim` validator reserves a slot; the approach is
  walked by the path follower (NPCs) or the player; the Warp move aligns the clip; reach IK puts the
  hand on the handle (R9 §4: door by side and hand, lever and sit by approach angle through warp +
  turn + IK on generic interact clips).
- **Edge**: a segment chain with kind (`Ledge`, `Ladder`, later `Beam`, `Hold`) and flags (foot
  hold, look permission, landing offset, recognition cone) (R5 adopt 6). Authored by content packs
  or **baked from collision** per cell (R3 adopt 3): walkable top meeting a wall below, headroom
  check, inside-solid check. Markup filters ("no parkour", "route") are optional tags (R7 §5).
- Until the bake exists, ledges come from live `ContextProbes` (phase B3), as today.

### Paired moves (E3)

**Paired moves** (R6 §9): start check (distance band, angle, target state) → short lunge →
`grab_check` with looser limits → **one** alignment of the victim onto a socket in the attacker's
root frame, eased over the first ~0.5 s → each plays its own root motion → lethal branch onto a
pose-identical death clip (or the ragdoll victim) → `PairFits` sweep before start; wall pushes
bypassed while paired. Air and drop kills come after M10's first combat milestone (decision).
