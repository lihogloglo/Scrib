# Action layer: moves, move rules, chooser, input, co-op readiness

Status: **draft, awaiting owner review (R11b, 2026-10-19).** Slice A1 built 2026-10-21 (`sf_action`,
see "As built (A1)" at the end, which wins where it differs from the design text above).
Crates: `sf_action` (Bevy-free core, new), `sf_action_bevy` (thin plugin, new).
Sources: the animation study `../research/animation/` (notes R1-R10, cited "R3 §7"), and its decision
list in `11-tradeoffs.md` "Decisions (owner brainstorm, 2026-10-19)", which this spec implements,
and the net-ready rules of the R12 co-op research (`../research/coop/README.md` §4).
Companions: [camera.md](camera.md) (new), [animation.md](animation.md) "Next-gen roadmap: decided
shape (R11b)", [locomotion.md](locomotion.md) "Action-layer changes (R11b)", [test-levels.md](test-levels.md).

## Goal

One system picks what every character does next, for the player and NPCs alike: sneak behind a
guard, jump roof to roof, grab a ledge and mantle, fight with a sword, sit at a bar, open a door,
pull a lever, in third and first person. It must feel snappy (a press shows something within 1-3
ticks), keep fights weighty (commitment is data per move), let mods add moves without code, and be
ready for 2-4 player co-op without a rewrite.

Non-goals here: the combat rules themselves (damage, posture, stamina: the M10 combat spec), AI
(M7 `sf_ai`), network transport (R12 co-op research), wall climbing on holds (after ledges).

## Decisions this spec implements

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

## Canonical technique

**"The decider decides, the graph plays, clip windows gate."** This is the FromSoftware split
(R5 §1): a decider picks every transition, the animation graph only plays and blends, and timed
windows on clips say when a choice is allowed. Our decider is not a script. It is a Rust layer
evaluating **data move rules** on the **fixed tick** (R1 adopt 7, R5 §7), with a bounded settle
loop. UE's Game Animation Sample (GASP) has the same shape for traversal: probes, then a chooser
table, then motion warping (R7 §1.2). Luau never runs inside the per-tick loop (Lua is parked, and
a script slot runs a frame late, which makes controls mushy: R5 §7).

Five motion owners, one per move kind (Holden's "label regions of the data with the displacement
you want", R8 §1.2):

| Owner | Who moves the gameplay body | Clip | Used by |
|---|---|---|---|
| `Sim` | the mover, from intent (springs or momentum) | follows (`Match`); render body corrected toward the capsule | locomotion, sneak, air |
| `Warp` | the baked root track, warped onto a target, playhead from planned progress | `Progress` root mode | vault, mantle, ledge catch, climb up, ladder ends, sit, door, lever |
| `Docked` | the mover, along an edge or ladder | `Progress` or cycle by edge distance | hang, shimmy, ladder climb |
| `Clip` | the baked root track at the action clock, with turn-rate and lunge windows | `Progress` (seconds) | attacks, dodges, reactions, paired moves |
| `Physics` | `sf_ragdoll` | `PoseInput` | knockdown, death, ragdoll victim |

Gameplay never reads the evaluated pose. It reads **baked** data sampled on the action clock: root
tracks, windows, contact times, end poses, weapon socket tracks (R6 adopt 1). That is what makes
the layer survive animation LOD, culling and first person, and what makes it net-ready.

## Move taxonomy

Slot names are the clip-slot categories of R9 §6 (`kind.family.variant`). A slot can hold a chain
of clips (enter, loop, exit).

| Kind | Moves (first build in **bold**) | Owner | Default windows |
|---|---|---|---|
| Locomotion | **idle, walk, run, sprint, turn in place, starts/stops** | Sim | unrestricted |
| Sneak | **sneak idle/move** | Sim | unrestricted |
| Air | **jump (by height), fall, land (normal, roll, hard)**, coyote jump | Sim | land: hard landing commits |
| Traversal contact | **step up, vault (low/thin), mantle (waist, high), ledge catch (from jump or fall), climb up, drop to hang**, ladder enter/exit (top, bottom), later wall run | Warp | committed until the last contact, then a cancellable tail |
| Docked | **hang, shimmy, ladder climb up/down** | Docked | unrestricted for Up/Down/Move along the edge |
| Combat | light chain, heavy (charged), guard (held), deflect, dodge/step/roll, backstep, jump attack, drop attack (after M10's first milestone), hit reactions by tier | Clip | category defaults below |
| Paired | stealth takedown (first: ragdoll victim), backstab, riposte, grab | Clip, after one alignment | committed; attacker's move cancel opens before the victim ends |
| Interaction | **sit/stand (seat), door (push/pull by side), lever**, pick up, later talk-facing | Warp to an interaction slot | committed until the contact ends |
| Forced | stagger, knockdown, get-up, death | Clip or Physics | not cancellable except get-up's tail |

Climbing on a hold grid is out of scope. The edge/hold index (phase B4) is built so it can follow.

## Data model

### Commands (the only input)

Every character is driven by one `Command` per fixed tick. The player's comes from input, an NPC's
from AI or scripts, a remote player's from the network. Nothing else feeds the chooser.

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

**Action classes** (one vocabulary for buttons, buffers and windows): `Light`, `Heavy`, `Offhand`,
`Dodge`, `Jump`, `Up`, `Down`, `Guard`, `Interact`, `Item`, `Move`, `Sprint`, `Sneak`.

**Intents.** `Jump` is a jump; with forward input at an obstacle it becomes the matching vault or
mantle in Normal and Free-run (decision). `Up` is the explicit traversal intent: mantle, climb up,
ledge grab, ladder up; it works locked on. `Down` is the quiet intent: drop to hang, let go, ladder
down, slide off. Sneak never triggers traversal, and stance input never triggers a drop (AC Shadows'
split, R7 §5). Default bindings are the content pack's (`sf_mods::Profile::bindings`); open
question 1.

### Character action state (gameplay, saved, replicated)

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

Together with `sf_controller::Locomotion` this is the whole gameplay body. No other state may
change what a character does. It lives in `sf_action`, not in Bevy systems.

### Move rule (data)

One RON record per move, of record kind `MoveRule` (patchable like other records, mods.md).

```ron
(
    id: "traverse.mantle.waist",
    kind: Traversal,
    category: Traversal,                 // places it in the priority list
    rank: 200,                           // order inside the category
    slots: ["traverse.mantle.waist"],    // clip chain; Optional slots fall back (animation.md "Optional clips")
    owner: Warp(target: LedgeTop, profile: Late, refuse_ratio: 0.35, steer: false),
    entry: (
        classes: [Jump, Up],             // what request starts it; [] = automatic
        need_forward: true,              // Jump only with forward input (decision)
        modes: [Normal, FreeRun],        // never Sneak (decision)
        states: [Grounded, AirRising],
        probe: Obstacle(classes: [Mantle], height: (0.7, 1.5)),   // in character heights (rig measures)
        speed: (0.0, 12.0), vspeed: (-1.0, 8.0),
        time_to_contact: 0.4,            // s; probe reach scales with speed (R1 adopt 4)
        facing_cone: 50.0,               // deg between the face normal and stick or view
        auto_in: [FreeRun],              // also fires with no press in these modes (passovers)
    ),
    validate: [EndPoseFree, PathOverTop, HandsOnLip],
    windows: Category,                   // category defaults, moveset sheet overrides
    exits: [(on: End, to: Loco), (on: Tail(Move), to: Loco), (on: Tail(Jump), to: Rule("air.jump"))],
    exit_velocity: KeepPlanar(0.6),
    redo_ticks: 12,
    look: (yaw: 70.0, pitch: (-45.0, 35.0), ease: 0.15),        // first-person limits around entry yaw
    noise: Loud,                         // perception event for sf_ai
    breaks_lock: true,
)
```

**Vocabulary, not expressions.** Rules combine fixed predicates; there is no expression language.
- Entry predicates: action classes, `need_forward`, modes, stance, states (`Grounded`, `AirRising`,
  `AirFalling`, `Docked`, `InMove(kind)`), probe class + height/depth band (character heights),
  speed and vertical-speed bands, time to contact, facing cone, target state (`Unaware`, `Broken`,
  `Behind`, `Front`), interaction tag, optional stat gate (content pack only).
- Validators (all must pass, else the next candidate; AC1's end-pose acceptance, R3 §5):
  `EndPoseFree` (capsule at the chain's end pose fits), `PathOverTop` (swept capsule over the top
  and down: R1 adopt 5), `HandsOnLip` / `HandsOnHolds` (hands within reach of real edges, spacing
  as the clip's hands), `FeetSupport`, `Claim(point)` (interaction slot free and claimed),
  `PairFits` (both capsules swept along their predicted root tracks, R6 §9), `MaxCorrection`
  (warp correction / root distance ≤ `refuse_ratio`).
- Exits: `End`, `Tail(class)`, `Touchdown`, `EdgeEnd`, `Released(class)`, `Window(name)`, each
  to a rule id or `Loco`.

Unknown predicates, slots or rule ids are load errors with the mod named (mods.md reports).

### Windows (one vocabulary for all moves)

Typed windows are compiled from clip `Event` windows by `animgraph.compile` (animation.md),
filled from three layers (R9 §6): inferred by `clip.annotate`, category defaults by slot, and the
moveset sheet. The action layer reads them on the action clock.

| Window | Meaning |
|---|---|
| `accept(class)` | a press of `class` is latched from here (earlier presses are dropped) |
| `cancel(class)` | a latched `class` request may start its move here (execute) |
| `tail` | traversal: the last contact ended; `cancel(Move, Jump, Dodge, Down)` open |
| `turn(rate)` / `no_turn` | gameplay yaw tracks stick or target at `rate`; none inside `no_turn` |
| `lunge(min, max, standoff)` | root translation scaled toward the target, clamped (R6 §5) |
| `warp(target, profile)` | Warp owner: where the correction is applied |
| `contact(hand/foot, side)` | inferred contacts: IK latching, validation, `tail` start |
| `launch(velocity, air_control)` / `gravity_off` | air and traversal phases |
| `iframes(mask)` / `armor(tier)` / `kb_scale` | defence (M10) |
| `hit(n)` / `deflect` / `charge` | offence (M10); hit shapes come from the baked weapon socket track |
| `grab_check` / `branch` | paired: grab test point; lethal branch frame |
| `look(limits)` / `ignore_look` / `camera(preset)` | presentation hints, camera.md |

**The restricted rule** (R5 adopt 1): a clip with no `accept`/`cancel` windows accepts and executes
every class. Locomotion and idle loops need no authoring.

### Category default windows

From R9 §6, changed by the decisions. A moveset sheet line overrides any of them.

| Category | Defaults |
|---|---|
| Loops (locomotion, sneak, hang, ladder) | unrestricted |
| Traversal contact | `accept(all)` from start; no `cancel` until `tail` (last inferred contact end); no steering (decision) |
| Interaction | as traversal; seated idle unrestricted for `Down`, `Interact`, `Move` (stand up) |
| Light attack | `accept(all)` from 0.2 of the clip; `cancel(Light, Heavy, Dodge)` at recovery start (after the last hit window); `cancel(Move)` later |
| Heavy attack | **no cancel windows** (decision: a missed heavy is a full recovery); exits at `End` only. Whether a connected heavy opens a chain is moveset data (default: no) |
| Dodge | i-frames over a set fraction (moveset row); `cancel(Light)` after i-frames (dodge attack opener); `cancel(Move)` late |
| Guard | held state; enters at its own `cancel(Guard)`; fires from the buffer only if still held |
| Reactions | none until the tier's recovery; then `cancel(Dodge)` |
| Paired | none; the attacker's `cancel(Move)` opens before the victim's clip ends |

### Mode profiles

One world scale; modes are multipliers in data (R7 §3, DL2's approach), tuned on test courses.

| Profile | Entered by | Changes |
|---|---|---|
| `Sneak` | Sneak toggle | slow gait, quiet noise; no traversal rule fires (all traversal rules exclude Sneak), no automatic moves, stops instantly |
| `Normal` | default | Jump at an obstacle with forward input vaults or mantles; Up and Down explicit; no automatic passovers |
| `FreeRun` | **hold Sprint** (decision) | momentum mover mode, longer jumps by height, automatic passovers (`auto_in: [FreeRun]`), ledge catch from runs and jumps, loud noise |
| `Combat` (stance) | weapon drawn | combat rules on; lock-on allowed; traversal still allowed (a drawn weapon doesn't block it); hit-stop on |

`Combat` layers over the other three. Sprint cancels Sneak. **Assisted/manual setting** (DL2,
R7 §2.2): Manual drops `auto_in` and the Jump-into-mantle context, leaving Up/Down explicit.

Each profile is a record: speed and acceleration scales, jump height, turn-rate scale, noise level,
allowed automatic rules, hit-stop scale. Defaults are tuned, not copied from any game.

### Stat bands

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

## The chooser (fixed tick)

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

### Priority list (one list for every character)

Highest first. Sekiro's fixed candidate order, adapted (R6 §3, R11a §9).

1. **Forced**: death, reactions, knockdown, get-up (not requested; from hits and physics).
2. **Landing** by fall-speed band (forced on touchdown).
3. **Air catch**: ledge catch while falling or jumping (automatic in Normal and FreeRun when
   moving toward a ledge within its cone; never in Sneak).
4. **Paired starts**: takedown, backstab, riposte (on `Light` with a valid target state).
5. **Dodge**.
6. **Attacks**: heavy, light, openers (jump attack, drop attack later).
7. **Guard**.
8. **Explicit traversal**: `Up` (mantle, climb up, ladder up, ledge grab), `Down` (drop to hang,
   let go, ladder down).
9. **Jump**, whose context candidates (vault, mantle with forward input) are tried before the
   plain jump. Locked on, Jump stays a plain jump; `Up` still mantles.
10. **Interact**: seats, doors, levers, pick up.
11. **Automatic passovers** (FreeRun only, no press).
12. **Locomotion** (always matches).

Starting any traversal or interaction **breaks lock-on**; camera.md re-acquires on landing.

## Motion owners in detail

- **Sim.** Locomotion intent from the stick, gait from the mode, `Match` root mode. The render body
  is corrected toward the capsule by a damped, velocity-limited, clamped spring (locomotion.md
  "Action-layer changes (R11b)"). In FreeRun, the player uses the momentum mover mode.
- **Warp.** At start, the plan fixes: start root, target (lip, top, landing, seat slot), the chain's
  total root motion, the correction (target minus chain end), and the correction profile
  (`Linear`, or `Late` = cubic, rise first and go over the lip late: R3 §5). Progress advances
  with the action clock (or with airtime for ballistic phases). The gameplay root is
  `start + baked_root(progress) + correction × profile(progress)`, computed by the same Bevy-free
  warp function the graph uses (animation.md "Decided shape (R11b)", item 2). No steering until `tail`.
- **Docked** (R5 adopt 5): the mover is on an edge chain or ladder; position = edge parameter;
  gravity and pushes off; side input in a cone moves along the edge at the docked speed; edge ends
  and corners are probed ahead. The hang offset (hand vs root) comes from the clip at bake.
- **Clip.** The baked root track at the action clock, with yaw separate from translation: turn
  toward stick or target at the window's rate, none in `no_turn`; `lunge` scales translation toward
  the target within clamps; the target is the lock-on target, else soft aim (camera.md). Exit speed
  on a move cancel keeps momentum.
- **Physics.** Ragdoll and powered ragdoll (physics.md, animation.md layer 4). The first takedown's
  victim is a short reaction then a ragdoll (decision), so no paired clip is needed for it.

**Paired moves** (R6 §9): start check (distance band, angle, target state) → short lunge →
`grab_check` with looser limits → **one** alignment of the victim onto a socket in the attacker's
root frame, eased over the first ~0.5 s → each plays its own root motion → lethal branch onto a
pose-identical death clip (or the ragdoll victim) → `PairFits` sweep before start; wall pushes
bypassed while paired. Air and drop kills come after M10's first combat milestone (decision).

## Interaction points and edges

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

## Input buffering and first response: tests

Scripted-input tests (R6 adopt 7, R8 adopt 1), headless in `sf_action/tests/`:
- every rule: press → first response within 1-3 ticks;
- a press inside `accept` fires exactly on the `cancel` tick; a press before `accept` is lost;
- Guard fires from the buffer only if held; Dodge fires on tap release;
- jump buffer and coyote windows, in ticks;
- same results at 30, 60 and 144 render fps (the fixed tick is the clock).

## How mods add moves

A mod ships, with no code:
1. **Move rules**: `MoveRule` records (new or patches of existing ones, field-level, mods.md).
2. **Clips** in its animation-sources manifest, each filling a **slot** (`traverse.vault.low`,
   `attack.light.1`). The slot's category gives default windows.
3. **Moveset sheet rows** (RON sidecar keyed by slot) for design numbers: one line per attack slot
   (R9 §6), paired offsets and branch frames, i-frames.

Rules: a mod rule cannot enter categories above `Paired starts` (forced, landing, air catch are the
framework's). `rank` orders inside a category; ties break by load order. Load-time validation
lists every unknown slot, predicate and exit. Luau, when it returns, gets **hooks** on move start
and end (next script slot), never the per-tick decision.

## NPCs

NPCs run the same rules (decision). Their commands come from `sf_ai` later (M7); until then from
the path follower and scripts (`PathWalkScaffold`), which only produce `Move` and `Interact`.
Traversal AI (choosing to mantle or jump in a chase) waits for `sf_ai`; NPCs path on the navmesh
until then. NPC probes stay gated (A0, blocked or asked).

## Co-op readiness (designed now, built after M4-M6)

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

## Crate: new `sf_action` (+ `sf_action_bevy`), not a module of `sf_controller`

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

## Build plan (phases)

Slices of 1-2 steps each. Each ends verified and committed before the next starts. Courses are
`sf_level` levels (test-levels.md); new levels are added in the slice that needs them. Phase B is
ROADMAP P11's first part, and A1-A2 are its prerequisites, so P11 can start with A1.

| Slice | Steps | Verify |
|---|---|---|
| **A1** backbone | `sf_action` crate: `Command`, `ActionState`, `MoveRule` RON + validation, priority list, chooser steps 1-10 with Locomotion and Air rules only (jump by height, coyote, jump buffer, landing bands) | headless on `flat-slopes`: first response ≤ 3 ticks; jump buffer and coyote fire on the right tick; identical outcomes at 30/60/144 Hz render; rule load errors listed |
| **A2** fixed tick in Bevy | `sf_action_bevy`: per-local-player command sampling (`PlayerId`), movers on the fixed tick with transform interpolation (required, R12 rule 1; locomotion.md change), render-body correction spring | `level-flat-slopes` walk/jump at FPS caps 30/60/144 with no jitter; `wk-balmora` mover stage time within 10 % of before; owner visual check added |
| **B1** progress + chain warp | `sf_anim_graph`: `Progress` root mode, chain warping with `Linear`/`Late` profiles and refusal, warp fn callable from `sf_action` | graph tests: 2-clip chain ends ≤ 2 cm / 2° from target; refusal above ratio; playhead equals action progress each tick |
| **B2** mantle and vault | Warp-owned step up, vault (thin), mantle (waist, high) from `Jump`+forward and `Up`; validators `EndPoseFree`, `PathOverTop`, `HandsOnLip`; `tail` cancel; minimal `clip.annotate` (hand/foot contacts, contact-bounded warp window, end pose) | course `ledges`: boxes 0.5-1.6 m mantle with feet ≤ 2 cm on top; thin 0.9/1.2 m walls vault, deep block mantles; Sneak never triggers; jump band ends both pass; refusal log on the 3.5 m wall |
| **B3** ledge catch and hang | Docked mover mode; ledge catch from jump and fall, hang, shimmy, climb up, drop to hang (`Down`), let go; hand IK latched on the lip | course `ledges`: 2.0/2.5 m boxes and the 3.5 m wall: catch, shimmy to the end, climb up; hands ≤ 1 cm off the lip; jolt test (R2 adopt 3) |
| **B4** edge index + ladders | `sf_interaction` edges baked from collision per cell; ladders as edges with rung IK, enter/exit top and bottom | new level `ladders` (2 ladders, one to a roof); rungs ≤ 1 cm; edge census over Balmora rooftops (count, bake time ≤ 5 % of the cold bake) |
| **B5** free-run | FreeRun profile on Sprint hold, momentum mover mode, automatic passovers, time-to-contact gating, ledge catch across gaps, noise events, assisted/manual setting | course `ledges`: 1/2/3/4 m gaps and the beam, scripted roof run; Manual setting disables automatic moves |
| **C1** interactions | `sf_interaction` points: seat (sit/stand by approach angle), door (push/pull by side), lever; claims; reach IK | new level `seats-doors` (stools, bench, two doors, lever); sit from 4 approach angles, hand on handle ≤ 2 cm; two NPCs never claim one seat |
| **D1** camera | `sf_camera` third-person rig, first-person eye from the baked camera track, per-move look limits, comfort switches (camera.md) | camera.md tests; visual checks CAM-1/CAM-2 |
| **D2** first-person bodies | first-person render body + world shadow body; optional first-person arms clips on the shared clock | same command stream gives identical gameplay ticks in both views; visual check |
| **E1-E3** combat (M10) | E1 windows + moveset sheet + light/heavy/dodge/guard; E2 lock-on and soft aim; E3 stealth takedown with ragdoll victim | scripted-input combat tests (R6 adopt 7); heavy whiff has no cancel; takedown `PairFits` at walls |
| **F** co-op (after M4-M6) | R12 research, then snapshots, then prediction and reconciliation | R12 defines |

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

## Owner answers (2026-10-19)

1. **`Up` = Jump (context), `Down` = Sneak/Crouch pressed at an edge** (drop to a hang, climb
   down, let go while hanging). No dedicated Down button. Sneak held while moving still never
   triggers traversal; only the deliberate press at an edge does.
2. **Clips: free sources first** (UAL1 and other free packs; Mixamo for prototyping only, not
   shipped raw). Buy UAL2 only after its clip list shows it covers the vault/top-out gaps.
3. **First-person stealth takedown stays in first person**, framed by look limits; no camera cut.

Not a question any more: movers on the fixed tick (R12 rule 1, required; phase A2).

## As built (A1, 2026-10-21)

`crates/sf_action` (Bevy-free; deps `sf_controller`, glam, serde, ron, thiserror). Public API:
`Command` / `InputSampler` / `CommandFeed`; `MoveRule` → `RuleSetBuilder` → `RuleSet`;
`SlotSource` / `SlotTable` / `Window`; `ActionState`; `step(&StepCtx, &mut ActionState,
&Command, &Sense, &mut ActionOutput)`; `BASE_RULES` (framework rules: `loco`, `air.fall`,
`air.jump`, `land.normal`, `land.hard`, `interact.use`). Tests: `src/tests.rs`.

Reuse: no crate covers a gameplay input buffer or move chooser (`leafwing-input-manager` and
`bevy_enhanced_input` map devices to actions; ggrs/lightyear buffer netcode inputs). The A2
shell feeds `InputSampler` from `bevy_enhanced_input` actions.

Changes against the design above:
- **`step` decides, the shell moves.** Chooser step 9 is not inside `step`: it emits the Sim
  owner's `LocomotionIntent` plus `launch` (vertical m/s) in `ActionOutput`, and the shell runs
  one batched `KinematicMover::step_batch` for all characters (the mover batches probes across
  characters). The mover result is next tick's `Sense` (grounded, vertical and planar speed,
  yaw, interaction focus). `step` mutates state in place; same inputs, same bytes.
- **Windows come through `SlotSource`** (slot length + typed windows on the action clock), so
  A1 needs neither `sf_anim` nor `sf_anim_graph`; B1 implements it over `animgraph.compile`.
  A slot chain plays each finite slot then the next; a looping or unknown slot holds it; an
  empty chain is an instant move. Rule field `windows: Category | Unrestricted | Committed` sets
  the policy when the slot authors no windows (A1 defaults: Landing, Jump, Locomotion,
  Passover unrestricted; the rest committed until E1 adds combat defaults).
- **Integer clocks.** `ClockTime` is thousandths of a tick (1 s = 60 000); the clock rate is
  per mille (hit-stop). Ids on the wire are FNV-1a 32 keys of the content ids (`RuleKey`,
  `SlotKey`, `TagId`, collisions are load errors).
- **Requests wait for windows, not predicates.** A latched request that could fire now but that
  no rule takes is dropped, with the refusal logged ("keep it latched" above made a refused
  Interact or Jump fire seconds later). Traversal's "can't" feedback hangs off the refusal (B2).
- **Jump buffer and coyote are ticks stored as ticks:** a Jump pressed in the air up to
  `jump_buffer_ticks` (6) before the landing tick fires on it, in the settle round after the
  forced landing; a Jump on the first `coyote_ticks` (6) airborne ticks after walking off an
  edge is a ground jump (`Phase::Coyote`). A committed (hard) landing lets the buffer lapse,
  but a press during it latches and fires on its end tick.
- **Vocabulary in A1:** entry `classes, need_forward (60° cone), modes, states (Grounded,
  Coyote, AirRising, AirFalling, Landed), speed, vspeed (the impact speed on the Landed tick),
  auto_in, focus`; exits `End, Touchdown, Airborne` to `Loco` or `Rule(id)`; `launch:
  JumpHeight | Speed(v)`; `steer` (false stops a Sim move); `redo_ticks`. Validators, probes,
  targets, plans and the other owners arrive with B1-B3. Unknown fields are load errors.
- **Candidates:** forced (framework category, no classes) always; requested when a latched
  class is in `cancel`; automatic (no classes) when the current move is over (ended, exited,
  or an automatic move whose own entry stopped holding) or `cancel(Move)` is open with the
  stick active. The current move restarts only once it is over. Mod rules in `Forced`,
  `Landing` or `AirCatch` are load errors; a later rule with the same id replaces the earlier
  (field-level patches come from `sf_records` once the shell registers `MoveRule` as a kind).
- **"Press to use" hook:** the shell passes `Sense.focus` (target id + tag) from its view cone;
  `ActionOutput.prompt` names the rule a press would start and whether it's accepted now;
  `interact.use` (instant, `focus: Some([])`) emits `EventKind::Activate(tag)` with the target,
  a host-validated request in co-op. C1's warped interactions are more `focus` rules.
- **Net:** `Command`, `ActionState`, `ActionEvent` are serde (postcard round-trip tested; a
  state snapshot is < 64 bytes). `CommandFeed` orders and de-duplicates a player's commands on
  the host and repeats stick, view and held buttons (no new presses) for 6 missing ticks.

Found for A2 (in `sf_controller`, not changed here): the mover has no launch, so a rising
body re-snaps to the ground within `step_up` (needs a `launch` that skips the ground cast
while rising), and `Locomotion` is not serde yet (net rule 3).
