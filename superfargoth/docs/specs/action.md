# Action layer: moves, move rules, chooser, input, co-op readiness

Status: **owner answers recorded (2026-10-19, R11b); A1 and A2 slice 1 built.** Slice A1 built 2026-10-21 (`sf_action`,
see "As built (A1)" at the end, which wins where it differs from the design text above).
History, background and pre-build sketches: [archive/action-history.md](archive/action-history.md).
Crates: `sf_action` (Bevy-free core, new), `sf_action_bevy` (thin plugin, new).
Sources: the animation study `../research/animation/` (notes R1-R10, cited "R3 §7"), and its decision
list in `11-tradeoffs.md` "Decisions (owner brainstorm, 2026-10-19)", which this spec implements,
and the net-ready rules of the R12 co-op research (`../research/coop/README.md` §4).
Companions: [camera.md](camera.md) (new), [animation.md](animation.md) "Next-gen roadmap: decided
shape (R11b)", [locomotion.md](locomotion.md) "Action-layer changes (R11b)", [test-levels.md](test-levels.md).

<!-- agent-summary -->
> **Agent summary (2026-10-26).** Status: A1 (`sf_action` backbone) and A2 slice 1 (`sf_action_bevy` fixed tick) and slice 2 (animated player body) built; ROADMAP 2.1 [~] (PL-1 awaiting owner), 3.1 [ ] open (B1-F). Designed, not built: warp mantle/vault, ledges, ladders, free-run, interactions, combat, co-op.
> Crates / key types: `sf_action` (Bevy-free: `Command`, `InputSampler`, `MoveRule`/`RuleSet`, `ActionState`, `step`, `BASE_RULES`), `sf_action_bevy`. Decisions: rules decide on the fixed tick, Luau never per tick; five motion owners Sim/Warp/Docked/Clip/Physics ("Canonical technique"); gameplay reads baked data, never the pose; Up = Jump, Down = Sneak at an edge ("Owner answers"); net-ready ("Co-op readiness").
> Read next: "As built (A1)" first (wins over design text); "Build plan" for slices; "The chooser" for rule changes.
<!-- /agent-summary -->

## Goal

One system picks what every character does next, for the player and NPCs alike: sneak behind a
guard, jump roof to roof, grab a ledge and mantle, fight with a sword, sit at a bar, open a door,
pull a lever, in third and first person. It must feel snappy (a press shows something within 1-3
ticks), keep fights weighty (commitment is data per move), let mods add moves without code, and be
ready for 2-4 player co-op without a rewrite.

Non-goals here: the combat rules themselves (damage, posture, stamina: the M10 combat spec), AI
(M7 `sf_ai`), network transport (R12 co-op research), wall climbing on holds (after ledges).

## Decisions this spec implements

The owner's R11b decisions (`11-tradeoffs.md` "Decisions"): third person primary, one gameplay
body in both views; co-op-ready; ledges first; Jump+forward mantles, separate Up/Down intents,
Sneak never traverses; hold Sprint to free-run; missed heavies fully recover; stats scale
magnitudes, never timing; packs-only clips. Table of where each lands:
[archive](archive/action-history.md#decisions-this-spec-implements-r11b).

## Canonical technique

**"The decider decides, the graph plays, clip windows gate"** (FromSoftware's split, R5 §1; UE
GASP's probes → chooser → warping for traversal, R7 §1.2): data move rules evaluated in Rust on
the fixed tick with a bounded settle loop; Luau never runs per tick. Background:
[archive](archive/action-history.md#canonical-technique-background).

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

`sf_action::Command`: tick, optional `PlayerId` (people only), stick already in world yaw,
`view` (yaw, pitch: aim and soft-aim input, never the camera pose), button edges and hold
ticks per action class, lock request. Presses between ticks accumulate in the shell
(`InputSampler`); commands are serde/postcard data logged by the flight recorder. Pre-build
sketch: [archive](archive/action-history.md#command).

**Action classes** (one vocabulary for buttons, buffers and windows): `Light`, `Heavy`, `Offhand`,
`Dodge`, `Jump`, `Up`, `Down`, `Guard`, `Interact`, `Item`, `Move`, `Sprint`, `Sneak`.

**Intents.** `Jump` is a jump; with forward input at an obstacle it becomes the matching vault or
mantle in Normal and Free-run (decision). `Up` is the explicit traversal intent: mantle, climb up,
ledge grab, ladder up; it works locked on. `Down` is the quiet intent: drop to hang, let go, ladder
down, slide off. Sneak never triggers traversal, and stance input never triggers a drop (AC Shadows'
split, R7 §5). Default bindings are the content pack's (`sf_mods::Profile::bindings`); open
question 1.

### Character action state (gameplay, saved, replicated)

`sf_action::ActionState`: current move and slot chain index, action clock (integer, rate for
hit-stop), mode flags (Sneak | Normal | FreeRun, plus Combat stance), the two-stage input
buffer, tick timers (coyote, jump buffer, redo), targets by stable id (interaction point,
edge + offset, lock-on, partner) and, from B1, a Warp/Docked plan.

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

Per-category defaults from R9 §6 (light, heavy, dodge, guard, ...; a moveset sheet line overrides
any of them): [archive](archive/action-history.md#category-default-windows-e1), back here when E1 starts.

### Mode profiles

One world scale; modes are multipliers in data (R7 §3, DL2's approach), tuned on test courses.

| Profile | Entered by | Changes |
|---|---|---|
| `Sneak` | Sneak held (hold or toggle is the player's keymap setting, applied by the input layer: `sf_ui::PressMode`) | slow gait, quiet noise; no traversal rule fires (all traversal rules exclude Sneak), no automatic moves, stops instantly |
| `Normal` | default | Jump at an obstacle with forward input vaults or mantles; Up and Down explicit; no automatic passovers |
| `FreeRun` | **hold Sprint** (decision) | momentum mover mode, longer jumps by height, automatic passovers (`auto_in: [FreeRun]`), ledge catch from runs and jumps, loud noise |
| `Combat` (stance) | weapon drawn | combat rules on; lock-on allowed; traversal still allowed (a drawn weapon doesn't block it); hit-stop on |

`Combat` layers over the other three. Sprint cancels Sneak. **Assisted/manual setting** (DL2,
R7 §2.2): Manual drops `auto_in` and the Jump-into-mantle context, leaving Up/Down explicit.

Each profile is a record: speed and acceleration scales, jump height, turn-rate scale, noise level,
allowed automatic rules, hit-stop scale. Defaults are tuned, not copied from any game.

### Stat bands

Stats scale **magnitudes inside bands, never timing or latency** (decision): the content pack
maps stats to channels in 0..1, the framework maps each channel to a band per parameter (jump
height, run and sprint speed, docked speed). Sketch: [archive](archive/action-history.md#stat-bands-b5).

## The chooser (fixed tick)

Per character, once per fixed tick (60 Hz), in `sf_action::step`: take the command; advance the
action clock (hit-stop rate) and collect open windows; run the two-stage buffer (a press is
latched only inside `accept`, fires only inside `cancel`, no timer: Sekiro's model, R6 §3; jump
buffer and coyote as ticks); gather candidates (forced, requested, automatic; redo cooldowns
skip); probe on demand in one batch (B2+); pick by the priority list then validators, logging
refusals; start (first response this tick); settle at most 4 rounds; write `ActionOutput`.
Moving the body is the shell's ("As built (A1)"). Step-by-step design and the scripted-input
test list: [archive](archive/action-history.md#the-chooser-step-by-step-design-a1-built-it-see-actionmd-as-built-a1).

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

**Paired moves** (E3, R6 §9): one alignment of the victim onto the attacker's root frame, then
each plays its own root motion; `PairFits` before start. Sketch: [archive](archive/action-history.md#paired-moves-e3).

## Interaction points and edges

Planned Bevy-free `sf_interaction` (M4): interaction points (seat, door, lever; claimable approach
slots) and edges (ledge, ladder; authored or baked from collision per cell). Until the bake,
ledges come from live `ContextProbes`. Sketch: [archive](archive/action-history.md#interaction-points-and-edges-b4-c1).

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

Host-authoritative 2-4 player co-op with client prediction, no lockstep (owner decision). The eight
net-ready rules of R12 (`../research/coop/README.md` §4) hold here: gameplay on the fixed tick (no
frame `dt` in `sf_action`; movers move to the fixed tick in A2); `Command` is the only input (AI,
scripts and remote players write the same one); `ActionState` + `Locomotion` are serde data, and
presentation (graph, IK, camera, first-person bodies, VFX, sound) reads `ActionOutput` and never
writes back, with hits from baked socket tracks; pure steps, no wall clock, random picks hash
(seed, character, tick); one owner per state (client world changes such as `Claim` and `PairFits`
are host-validated requests); no singleton player (`PlayerId` on commands; camera, input and UI
per local player); stable ids and `WorldPos`, never `Entity`; events are data.
- **Predicted** (own player): locomotion, jump, coyote and buffers, traversal and docked moves, own
  attack and dodge starts and clocks, own stamina; mispredictions replay unacknowledged commands,
  small errors hidden by the render-body correction spring.
- **Host only**: hits, damage, reactions, deaths; interaction claims and door/lever state; paired
  starts (`grab_check` waits for the host); lock-on target validity; AI.
- **Replicated**: a snapshot per character (position, velocity, facing, move id, start tick, clock
  rate, mode flags, targets by id); peers re-derive presentation from move id and clock.

Full text, items left to the co-op milestone and follow-ups: [archive](archive/action-history.md#co-op-readiness-full-text).

## Crate: new `sf_action` (+ `sf_action_bevy`), not a module of `sf_controller`

It changes daily through P11 and M10 and needs compiled windows and baked root tracks, so a module
in `sf_controller` would rebuild physics on every chooser edit (§8b rules 3 and 5). `sf_action_bevy`
is a leaf plugin (command sampling from `bevy_enhanced_input`, the fixed-tick schedule, the
graph feed, gizmos). Rationale: [archive](archive/action-history.md#crate-choice-architecture-8b).

## Build plan (phases)

Slices of 1-2 steps each. Each ends verified and committed before the next starts. Courses are
`sf_level` levels (test-levels.md); new levels are added in the slice that needs them. Phase B is
ROADMAP P11's first part, and A1-A2 are its prerequisites, so P11 can start with A1.

| Slice | Steps | Verify |
|---|---|---|
| **A1** backbone | done 2026-10-21: see "As built (A1)" | `cargo test -p sf_action` |
| **A2** fixed tick in Bevy | slice 1 done 2026-10-22 (see "As built (A2 slice 1)"); slice 2 (animated player body) built 2026-10-26, PL-2 awaiting owner; `sf_action_bevy`: per-local-player command sampling (`PlayerId`), movers on the fixed tick with transform interpolation (required, R12 rule 1; locomotion.md change), render-body correction spring | `level-flat-slopes` walk/jump at FPS caps 30/60/144 with no jitter; `wk-balmora` mover stage time within 10 % of before; owner visual check added |
| **B1** progress + chain warp | done 2026-10-26 (see "As built (B1)"; `sf_action` call site left for B2): `sf_anim_graph`: `Progress` root mode, chain warping with `Linear`/`Late` profiles and refusal, warp fn callable from `sf_action` | graph tests: 2-clip chain ends ≤ 2 cm / 2° from target; refusal above ratio; playhead equals action progress each tick |
| **B2** mantle and vault | B2a (waist mantle, player) done 2026-10-26 on the capsule player (see "As built (B2a)"); the clip on the body comes with the animated player body (A2 slice 2). Next B2b. Warp-owned step up, vault (thin), mantle (waist, high) from `Jump`+forward and `Up`; validators `EndPoseFree`, `PathOverTop`, `HandsOnLip`; `tail` cancel; minimal `clip.annotate` (hand/foot contacts, contact-bounded warp window, end pose) | course `ledges`: boxes 0.5-1.6 m mantle with feet ≤ 2 cm on top; thin 0.9/1.2 m walls vault, deep block mantles; Sneak never triggers; jump band ends both pass; refusal log on the 3.5 m wall |
| **B2c** wall brace (owner 2026-10-27) | Procedural `WallBrace` op after B2b's hand IK: probe ahead/beside (animation-layers.md "wall beside"); a free hand plants on a wall within arm's reach, arm slightly bent, weight by approach speed and distance (firm when running in, light when walking past, trailing hand along side walls); busy hands skipped (weapon, torch, carried item: off-hand only with a one-hand weapon). Refs: R2 "wall-braced arm", R4 "arms out of walls" | course `ledges`/`anim-lab` walls: run into the 3 m wall, palm ≤ 2 cm off the surface, no arm through the wall; walk past: light trail; sword drawn: off-hand only |
| **B3** ledge catch and hang | Docked mover mode; ledge catch from jump and fall, hang, shimmy, climb up, drop to hang (`Down`), let go; hand IK latched on the lip | course `ledges`: 2.0/2.5 m boxes and the 3.5 m wall: catch, shimmy to the end, climb up; hands ≤ 1 cm off the lip; jolt test (R2 adopt 3) |
| **B4** edge index + ladders | `sf_interaction` edges baked from collision per cell; ladders as edges with rung IK, enter/exit top and bottom | new level `ladders` (2 ladders, one to a roof); rungs ≤ 1 cm; edge census over Balmora rooftops (count, bake time ≤ 5 % of the cold bake) |
| **B5** free-run | FreeRun profile on Sprint hold, momentum mover mode, automatic passovers, time-to-contact gating, ledge catch across gaps, noise events, assisted/manual setting | course `ledges`: 1/2/3/4 m gaps and the beam, scripted roof run; Manual setting disables automatic moves |
| **C1** interactions | `sf_interaction` points: seat (sit/stand by approach angle), door (push/pull by side), lever; claims; reach IK | new level `seats-doors` (stools, bench, two doors, lever); sit from 4 approach angles, hand on handle ≤ 2 cm; two NPCs never claim one seat |
| **D1** camera | `sf_camera` third-person rig, first-person eye from the baked camera track, per-move look limits, comfort switches (camera.md) | camera.md tests; visual checks CAM-1/CAM-2 |
| **D2** first-person bodies | first-person render body + world shadow body; optional first-person arms clips on the shared clock | same command stream gives identical gameplay ticks in both views; visual check |
| **E1-E3** combat (M10) | E1 windows + moveset sheet + light/heavy/dodge/guard; E2 lock-on and soft aim; E3 stealth takedown with ragdoll victim | scripted-input combat tests (R6 adopt 7); heavy whiff has no cancel; takedown `PairFits` at walls |
| | E1-E3 now live in [combat.md](combat.md) "Build plan" (2026-10-27): E1 = combat C1-C3, E2 = C1c + C4, E3 = C6. | |
| **F** co-op (after M4-M6) | R12 research, then snapshots, then prediction and reconciliation | R12 defines |

## Clip gaps and industry comparison

Moved to [archive/action-history.md](archive/action-history.md#clip-gaps-packs-only-free-preferred-no-owner-mocap) (clip sources by phase; shipped-practice table, all **match**).

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

Changes against the design (`step` decides and the shell moves; windows through `SlotSource`;
integer clocks and FNV-1a ids; requests wait for windows; jump buffer and coyote in ticks; the A1
rule vocabulary; candidates; the "press to use" hook; net serde): [archive](archive/action-history.md#as-built-a1-changes-against-the-design).

Moved to [history](archive/action-history.md#as-built-a2-slice-1-2026-10-22-sf_action_bevy).

## As built (B1, 2026-10-26)

The warp function is `sf_anim::WarpPlan` (re-exported by `sf_anim_graph`; in `sf_anim` because
`sf_action` already reaches it through `sf_controller`): `WarpPlan::try_new(&RootChain, start,
target, Linear | Late, refuse_ratio)` fixes the correction at start, `root_at(&chain, t)` gives
`start ⊕ baked(t) + correction × profile(t)` inside the carrier clip's Layer 1 `warp` window (else
its span). `RootMotionMode::Progress` montages take their playhead from
`GraphInstance::set_slot_progress(slot, chain_secs)`, queued ones back to back. Layer 1's per-update
skew warp stays for `Drive` (attack tracking). Our form is UE's Adjustment Blend Warp / AC1's
additive correction, one target per plan (GASP uses several; B2 chains plans). No `sf_action` call
site yet (needs the `sf_anim` dep and a lock line). API, tests, B2 notes:
[history](archive/action-history.md#as-built-b1-2026-10-26-chain-warp-and-progress-montages).


## As built (B2a part 1, 2026-10-26)

`Owner::Warp { target: LedgeTop { inset }, profile, refuse_ratio, steer }`, entry predicate
`probe: Obstacle(classes, height)` (character heights, `CharacterParams::height`), `Sense::obstacle`
(the shell's `sf_controller` context probe, relative to the feet), `SlotSource::root` (slot root
paths; `SlotTable::with_root`). At start the chooser plans an `sf_anim::WarpPlan` over the slot
chain (start at the feet facing into the face, target on top `inset` past the lip, a radius inside a
shallower top); no root path, no obstacle or `MaxCorrection` refuse into the next candidate (the plain
jump). `ActionState::warp` keeps the plan (not serialized yet); each tick `ActionOutput::warp` gives
the root at the end of the tick, relative to the start-tick feet, and the chain progress for
`set_slot_progress`. Rule `traverse.mantle.waist` in `TRAVERSAL_RULES`. Slot clip: UAL2
`ClimbUp_1m_RM`. Part 2: the shell probes `ObstacleAhead` on Jump/Up and places Warp bodies directly; the game loads the slot clip (history).
[history](archive/action-history.md#as-built-b2a-part-1-2026-10-26-warp-owner-and-the-waist-mantle-clip)

## As built (A2 slice 2, 2026-10-26)
Animated player body: the dressed `player` record under the body, fed from the drawn (interpolated, spring-corrected)
root; the Warp's chain progress drives the traversal slot clip (`Progress`). The clip's playhead is lerped by the
same `overstep_fraction` from the same two ticks as the body (`ActionBody::warp_progress` beside `poses`; 0 before the
move's first tick), so it stays on the drawn body with several ticks per frame (audit F6, 2026-10-26). [history](archive/action-history.md#as-built-a2-slice-2-2026-10-26-animated-player-body)
