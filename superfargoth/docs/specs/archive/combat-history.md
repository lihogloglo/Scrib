# Combat: archive

Moved out of [combat.md](../combat.md) on 2026-10-27 to keep the spec under the doc budget: the C1a and C1b briefs
and their as-built notes. combat.md and the `sf_combat` code win where they differ.

## C1 briefs and as built (moved 2026-10-27)

### C1a: framework core (Bevy-free)

`sf_action` (extend; keep each change generic):
- `windows.rs`: `WindowKind::{Turn(u16 deg/s), NoTurn, Hit(u8), IFrames(Mask)}`; `pub fn open_windows(rule, slots, t) ->
  impl Iterator<Item = &Window>`; `locate` skips dropped slots (absent from the source) instead of holding the chain.
- `rule.rs`: `Owner::Clip { root_scale: f32, square_up: SquareUp }`; `Entry::{moveset: Option<String>, from: Vec<Prev>}`
  (`Prev::Any | Loco | Rule(id)`; not `From`, which shadows the std trait); `RuleSetBuilder::add` accepts generated rules from `sf_combat`.
- `state.rs`: `ModeState::moveset: Option<MovesetKey>`; `ActionState::{forced: Option<Forced { rule: RuleKey, side: u8 }>,
  lock: Option<TargetId>}`.
- `step.rs`: forced first; `Sense::{refused: ClassSet, lock_target: Option<Vec3>}`; `Refused::Gate`; the Clip owner (yaw by
  `Turn`/`NoTurn`, root delta x `root_scale`); `ActionOutput::{clip, clock: (ClockTime, ClockTime)}`.
- `command.rs`: `TapHold { tap: Dodge, hold: Sprint, ticks: 18 }` in `InputSampler` (release under 18 ticks = a `Dodge`
  press edge on the release tick; from tick 18 `Sprint` held).
- `SlotSource::root` already serves root tracks; Clip moves use it.

`sf_combat` (new; `lib.rs`, `sheet.rs` (RON types, compile to rules + windows + `MoveTable`), `fighter.rs` (`Fighter`,
`Vitals`, `Pool`, `FightState`), `shape.rs` (`WeaponShape`, `HurtShape`, segment-capsule distance, 5 sub-steps),
`rules.rs` (trait, `Swing`, `Contact`, `Outcome`, `StandardRules`, hashed `roll`), `react.rs` (`Tier`, `Side`, table
lookup), `hitstop.rs` (curve to `ActionClock::rate`), `step.rs` (`pub fn step(ctx, fighters, actions, hurt, rules, out)`,
pipeline items 4-8), `tests/` with `data/test_sword.ron` on synthetic slots (`SlotTable` + straight-line socket tracks).
Register in hakari `final-excludes`. `sf_anim`: `SocketTrack` type and `sample_socket_track(skeleton, clip, bone, hz)`.

**As built (C1a, 2026-10-27).** Built as above; tests in `sf_combat/tests/combat.rs` (26 + 4 measurements). Deviations:
`Refused::{Prev, Moveset}` beside `Gate` (gate checked after entry and `from`); `CombatEvent::{Evaded, Missed}`; reactions
are `kind: Forced` rules (`CompiledRule::force_only`) started only through `ActionState::forced`; `ActionOutput::clock` is
(start, end) of the tick (end-of-tick convention like Warp); `start()` keeps the clock rate so a hit-stop spans the forced
reaction; `Owner::Clip` gained `rate` (clip s per move s: root tracks and `ClipOutput::progress` are clip time); chain
targets rank above their sources and every `Any`/`Loco` opener of the class, transitively; `At` reads bare `End` through an
untagged repr (RON drops a bare identifier's name in `deserialize_any`) and sheets load with RON `implicit_some`
(`stop: 0.05`); hit-stop ease ticks sit at `(e+1)/(ease+1)` (hold is exactly `hold` ticks); jump (a launch start) and sprint
(FreeRun with intent, outside combat moves) cost the sheet `costs` only within `HOSTILE_RANGE` 20 m of a living hostile, and
a draining tick doesn't regenerate. No `sf_ids` dep. **C1b must know:** no reach prefilter yet (every unit is tested at
every sub-step: add a per-track bound when tracks are built at load); stamina 0 doesn't stop a sprint yet; a sweep crossing
a slot boundary samples through `MoveSpec::slot_at`; the host order is `gate` -> `sf_action::step` -> mover -> `step`.

### C1b: player vs training dummy (Bevy + game)

- `sf_combat_bevy`: `SfCombatPlugin<R>`, `Combatant(Fighter)`, `CombatSystems::{Gate, Stats, Vitals, Sweep, Resolve, Apply}`,
  `ActionGate` write, socket tracks from cooked clips at load, gizmos (weapon segment, hurt capsules, open windows) under
  the dev overlay, messages `HitLanded`, `StaminaFlash`, `CombatantDied`.
- Game glue per `combat-adapter.md`: sheet `sword_1h.ron`, sources slots, player `Combatant`, `TrainingDummy`, level `arena`.
- **First sheet values** (starting points, tuned at CB-1; times in move seconds; clip peaks measured from the packs' right
  hand speed, 2026-10-27):

| Move | Slots (first candidate) | Length | Hits (window, mv, stamina, tier, stop) | Cancels Dodge / Light / Move |
|---|---|---|---|---|
| `light1` | UAL2 `Sword_Regular_A` (0.43 s, peak 0.25) + `Sword_Regular_A_Rec` (0.97) | 1.40 | 0.18-0.32, 1.0, 12, Small, 0.05 | 0.35 / 0.45 / 0.80 |
| `light2` | UAL2 `Sword_Regular_B` (0.53, peak 0.27) + `Sword_Regular_B_Rec` (1.03) | 1.56 | 0.20-0.34, 1.0, 12, Small, 0.05 | 0.38 / 0.55 / 0.95 |
| `light3` | UAL2 `Sword_Regular_C` (2.00, peaks 0.25 and 0.65) | 2.00 | 0.18-0.32, 0.8, 8, Small, 0.05; 0.58-0.72, 1.1, 8, Medium, 0.07 | 0.80 / 1.10 / 1.30 |
| `roll` | UAL1 `Roll_RM` (1.47, 5.0 m) at rate 1.25, root x0.75 | 1.18 | i-frames 0.02-0.40; cost 12 | 0.80 / 0.75 / 0.95 |
| `roll_left`/`_right` | UAL1 `Dodge_Left_RM`/`Dodge_Right_RM` (1.30, 2.0 m) at rate 1.25 | 1.04 | i-frames 0.02-0.36; cost 12 | 0.70 / 0.65 / 0.85 |

  All light moves: `Accept([Light, Heavy, Dodge])` from 0.10, `Turn(360)` until 0.15, `NoTurn` 0.15 to the end of the last
  hit window. Fallbacks: `light*` → UAL1 `Sword_Attack` (peak 0.40) → the content's own one-hand slash; `_rec` slots →
  `Drop`; rolls have no content fallback (the content pack has no roll): a missing roll clip refuses the move. The UAL2
  in-place attacks still move their root bone (A 0.82 m, C 1.56 m): keep it as the step-in, check it after the cook.
- Reaction slots, the training dummy, weapon draw and bindings: game side, `combat-adapter.md` "C1b".

**As built (C1b, 2026-10-27).** Framework: `SfCombatPlugin<R>` sets `Gate` / `Stats` / `Step` / `Apply` (one `Step` set runs the
core's vitals, sweep and resolve); `Combatant { fighter, fight, withheld }` requires `ActionGate` (-> `Sense::refused`); messages
`HitLanded` / `StaminaFlash` / `CombatantDied`; `socket_tracks` at 60 Hz; feature `debug`: `CombatGizmos`. Core additions:
`FightState::winded` (Sprint gated until 15 % stamina), reach prefilter (`SocketTracks` max reach). `sf_action_bevy`: Clip moves
drive the mover through `RootMotion`, `warp_progress` covers Clip chains, `PlayerAction::DodgeSprint` (tap roll, hold sprint).
Game (`mw_combat` Bevy-free sheet `sword_1h.ron`; `mw_player_bevy` `combat.rs`, `dummy.rs`): 14 slots on UAL1/UAL2 clips
(`sources.ron`), `ModernRules(StandardRules)`, F stance (attacks withheld sheathed; an attack press draws), avatar plays Warp and
Clip slots, HUD stamina from the `Combatant`; scaffold `TrainingDummy` (full action body, own ground wait, refills at 0) from
`dummy*` spawns of `levels/arena.ron`. Gaps: capsules don't collide (root motion walks into the dummy), no weapon mesh, medium
and large reactions share one clip at full travel. Check CB-1 (`visual-checks/combat.md`).

### C1c: lock-on

`sf_combat_bevy::lock` (candidates, `acquire`, `LockTimers`, `LockOn` mirror), `Sense::lock_target`, locked
`desired_facing`, `locked` roll moves. Test: acquire within 15 m and the cone, break past 20 m and on a mantle, re-offer on
touchdown.

### C1c as built (moved from combat.md 2026-11-10)

Four commits (a2e6d20, 3a54f4b, ddc252d, 2709b71); check CB-2.
- **Part 1, lock** (`sf_combat::update_lock` over `sf_camera::lockon`; shell `sf_combat_bevy::SfLockOnPlugin`, `LockOwner`,
  `Lockable { aim_height: 1.3 }`): a press acquires within 15 m and a 35 deg cone around the view yaw with line of sight (thin
  sphere cast through the ground probe: statics and terrain), else recentres the camera (`PlayerCamera::recentre`); a press
  while locked releases. Holds to 20 m; breaks on 1.5 s without line of sight, death or any traversal, re-offered for 1 s after
  touchdown (a jump is not a traversal). `ActionState::lock` is the truth; `LockAim` -> `Sense::lock_target`; the existing
  `sf_camera_bevy::LockOn` frames it (third person). Bound to Q / middle mouse / pad R3 (`Control::LockOn`).
- **Part 2a, facing and strafe** (`sf_action` step): locked and not sprinting, desired facing = the target's yaw through the
  mover's own turn caps; gait speed is taken against that facing (side/back speeds), so sideways circles the target and back
  backs away facing it; standing still turns to it; sprint faces its run. Locomotion still plays the forward cycles.
- **Part 3, locked rolls**: `Entry::lock` (`StickSide`, front/back +-45 deg), `Refused::Lock`, `SquareUp::Lock` (faces the
  target on the first tick); the sheet compiles a move's `locked` map into ranked variants (load errors for unknown targets).
  Left/right pick `roll_left`/`roll_right`; front/back keep the forward roll squared to the stick.
- **Part 4, marker** (`sf_combat_bevy::LockMarkerPlugin`, owner request): one UI node (8 px white dot, 1 px dark rim) at the
  target's root + `aim_height` projected through the player camera (PostUpdate after propagation); hidden unlocked, behind the
  camera or off screen.
- Left: strafe and back-walk clips in the graph (C1c-2b; UAL2 `Walk_Fwd/Bwd/L/R`); back-roll / backstep clip missing (back input
  rolls away); target switching by stick flick (C4: `LockRequest::Next` already switches); the Classic profile hiding lock-on.

### C1b sheet (moved from combat.md 2026-11-10)

Briefs and as-built notes for C1a and C1b (first sheet values on the UAL2 `Sword_Regular_*` 3-hit chain):
[history](archive/combat-history.md). The sheet as it stands:

| Move | Slots (first candidate) | rate / root_scale | Hits (window, mv, stamina, tier, stop) | Cancels Dodge / Light / Move |
|---|---|---|---|---|
| `light1` | UAL2 `Sword_Light_A` + `_A_Rec` | 1 / 1.0 | 0.14-0.28, 1.0, 12, Small, 0.05 | 0.31 / 0.37 / 0.57 |
| `light2` | UAL2 `Sword_Light_B` + `_B_Rec` | 1 / 1.0 | 0.18-0.32, 1.0, 12, Small, 0.05 | 0.36 / 0.43 / 0.66 |
| `light3` | UAL2 `Sword_Light_C` + `SwordLight_C_Rec` | 1 / 0.25 | 0.38-0.52, 1.0, 12, Small, 0.05 | 0.56 / 0.70 / 1.15 |
| `light4` | UAL2 `Sword_Light_D` (finisher; `next` wraps to `light1`) | 1 / 0.22 | 0.23-0.37, 1.2, 14, Medium, 0.07 | 0.50 / 1.05 / 1.25 |
| `roll` | UAL1 `Roll` (`_RM` pack) | 1.25 / 0.75 | i-frames 0.02-0.40; cost 12 | 0.72 / 0.70 / 0.74 |
| `roll_left`/`_right` | UAL1 `Dodge_Left`/`_Right`, only through `locked`, `square_up: Lock` | 1.4 / 1.0 | i-frames 0.02-0.30; cost 12 | 0.34 / 0.36 / 0.38 |

Light hit windows sit on the right hand's speed peak +-0.07 s; light1/light2 `Cancel(Light)` at the strike clip's end (its pose
is the next strike's first frame). `root_scale` gives ~0.4 m per swing (baked travel A 0.37, B 0.43, C 1.69, D 2.03 m). The
roll cancels open where its travel ends (no settle tail, owner). Fallbacks: `light*` -> UAL1 `Sword_Attack` -> content slash;
`_rec` and rolls -> `Drop`. Watch: light3 end -> `C_Rec` pose jump (hips +0.25 m); light3 -> light4 pose mismatch.

### C4 soft aim as built (moved from combat.md 2026-11-10)

- **Pick**: `sf_camera::lockon::soft_aim` (pure): the nearest candidate (horizontal distance) within `SoftAimParams { reach 3 m,
  cone 30 deg, window 1/6 s }` with line of sight; ties to the smaller angle, then the smaller id.
- **Tick**: `sf_combat::update_soft_aim` keeps the target on the swing (`SwingState::aim`): acquired while the attack's move time
  is inside the window, then held for that swing (a chained light picks again); a lock wins. Axis `soft_aim_axis`: the view
  yaw with `Command::face_view`, else the stick, else the facing (third person unlocked gets it too).
- **Shell**: `sf_combat_bevy::lock` (same pass as the lock): candidates are the other `Lockable`s, line of sight cast only for
  those in reach and only while acquiring; `sf_action_bevy::SoftAim` -> `Sense::soft_target`. `LockConfig::soft` tunes it.
- **Steer**: `sf_action` Clip owner: `Turn` aims at the lock, else an Attack's soft target, else the stick; with
  `Command::face_view` an Attack squares up to the view on its first tick and its free aim is the view. Rolls keep the stick.
- **First person** = `Command::face_view` (the input shell sets it from the view choice; the game's `mw_player_bevy::combat`
  `face_view` system): `update_lock` breaks a held lock (`BreakReason::View`) and ignores presses. The camera never turns.
- Tests: `sf_camera` pick (cone, reach, nearest, line of sight, ties, window); `sf_combat` tests `combat.rs` (square-up, turn in
  the window only, late targets, chains, lock wins, third person, rolls) and `lock.rs` (no lock facing the view); `mw_player_bevy`
  headless (dummy 20 deg off: turned onto it in light1's window, not after; lock breaks entering first person).

### C4b as built (from combat.md 2026-11-10)

**As built (2026-11-10)**, check CB-5b. Industry (R8 §2): true first-person melee bodies turn with the view, so the view never
leaves the weapon, and the swing's turn rate is capped while it is released (Mordhau's per-weapon "turn cap", near 360 deg for
high values; Chivalry 2's "torque"); Skyrim's and Dying Light's arms are the view model, so nothing to bound. Ours holds the body
still in `NoTurn` (no dragging), so the bound is an angle box around the body instead of a rate cap.
- **Data**: `sf_action::LookRule { yaw, pitch, ease }` (degrees, s) as `MoveRule.look`; a sheet move's `look` (bare in RON) is
  copied onto its generated rule, validated (yaw in (0, 180], pitch min <= max inside +-90, ease >= 0). `CompiledRule::look_at`:
  the rule's `look` until a `Cancel(Move)` window opens or the chain ends (an unrestricted move never limits).
- **Shell**: `sf_camera_bevy::follow_move_look` (`CameraSystems::Look`, before `update_look`): each player camera's
  `look_limits` from its followed character's move, `limits_yaw` = the body's gameplay yaw (where the arms are: the square-up
  yaw, plus what the `Turn` window turned). `update_look` applies them in first person only (`sf_camera::FirstPersonLook`,
  eased in and out over `ease`, input past a limit is dropped), and the look it leaves is what the sampler puts in
  `Command.view`: the clamp sits where the view is produced, so gameplay and a co-op host see the view the player has.
- **Content**: `sword_1h` attacks `look: (yaw: 45, pitch: (-45, 20), ease: 0.10)`; rolls free. 45 deg, not the suggested 60:
  half the horizontal FOV at the default 60 deg vertical on 16:9 is 45.7 deg, so the swing in front of the body stays on screen.
- Tests: `sf_action` (held until `Cancel(Move)`, none without `look` or unrestricted, load errors), `sf_combat` (sheet to rule),
  `mw_combat` (every attack has `look`), `sf_camera` (recentred limits keep their weight, ease in again after release),
  `mw_player_bevy` headless (first person: light1 holds the view within 45 deg of the body, `Command.view` equals it, free after
  `Cancel(Move)`; third person and the roll free).
