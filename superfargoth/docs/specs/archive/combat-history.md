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
