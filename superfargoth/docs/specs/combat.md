# Combat: feel layer, moveset sheets, rules seam

Status (2026-10-27): C1a, C1b and C1c (lock-on parts 1-4) built; next C1c-2b strafe clips, then C2. ROADMAP 3.5. Game side:
`../../../bevyrrowind/docs/specs/combat-adapter.md`. Built on [action.md](action.md) (commands, windows, chooser, motion
owners, co-op rules: referenced, not repeated); owns its build-plan row E1-E3. Slices **combat C1..C7** are not action.md's
C1 (interactions).

## Goal

An Elden Ring feel in third person (owner 2026-10-27): committed attacks with clip windows, a stamina economy, rolls with
i-frames, weapon sweeps with hit-stop, tiered reactions, lock-on. Game-agnostic: any content pack supplies moveset sheets,
clips and a rules implementation. The same framework runs classic rules (swing then roll to hit, fatigue, no i-frames) behind
one seam (ARCHITECTURE §1b), and drives NPCs and creatures with per-rig sheets.

Non-goals (first cuts): parry/deflect (guard only), player poise and hyper-armor (every unguarded hit staggers), weapon
skills, ranged and magic combat, stealth/paired kills (C6), first-person melee presentation (camera.md soft aim; later), AI
(`sf_ai`, M7), damage numbers.

Owner decisions (2026-10-27, DECISIONS.md): one button, tap = roll on release, hold = sprint; clips from free packs
(Quaternius UAL1/UAL2) plus the content's own animations, LaFAN1 optional with a non-LaFAN1 fallback per slot. Defaults
taken: one weapon class first (one-handed sword), laid out so a second class is only data; per-rig sheets for NPCs and
creatures (C6); co-op host-authoritative.

## Canonical technique

**FromSoftware's split: a fixed-tick decider with per-clip windows, weapon sweeps against hurt volumes, a per-character
hit-stop clock, and a rules layer behind the hit event.** Sources: `../research/combat/01-elden-ring-reference.md` (R1: the
ELDEN-RING-Combat-Rewrite reference, MIT, constants and slices), `../research/animation/06-souls-combat-feel.md` (R6: ER and
Sekiro code, windows, buffer, hit-stop curve, typed i-frames, armor windows), `../research/animation/08-industry-combat-responsiveness.md`
(R8: latency budget, asymmetric timing, hit-stop, shake), the classic requirements in
`../../../bevyrrowind/docs/research/combat-classic-openmw.md`. Industry: UE's Gameplay Ability System keeps attributes and
effects apart from animation notifies, as our feel layer / rules seam does.

Reuse (rule 8): Bevy has no combat layer and no maintained crate covers clip windows, weapon sweeps and a rules seam. Hits
are segment-capsule math on baked socket tracks, not physics shape casts (deterministic, host-side, LOD-proof). The ER
rewrite is a reference, not a dependency (a monolithic sim with a one-slot buffer; we have the chooser and per-class buffer):
we take its constants (with attribution, R1 flags) and its tests' shape. Lock-on math exists (`sf_camera::lockon`), and
framing and trauma shake (`sf_camera_bevy`).

Numbers adopted (R1 unless noted): per-class accept/cancel, no timer (R6 §3, built); dodge on release if held < 0.3 s,
else sprint; 5 sweep sub-steps; one landing per (hit, defender); hit-stop hold ~60 ms + quadratic ease ~0.1 s per side
(R6 §7, not ER's flat freeze); stamina roll 12, backstep 8, jump 10, regen 45/s, x0.5 guarding, sprint 11/s near
hostiles; turn inside actions 360 deg/s; guard raise 4 frames, arc 80 deg, stamina taken x0.55 (C2); lock 15 m / break
20 m (`LockParams`); first visible response within 1-3 ticks (R8 adopt 1).

## Architecture

Three layers, each testable alone: **feel** (framework, fixed tick: windows, motion, vitals, sweeps, contact order,
reactions, hit-stop, lock-on; owns timing, never computes damage), **rules** (the `CombatRules` seam: damage, costs, hit or
miss, regen; modern `StandardRules` in `sf_combat`, classic in the content pack, C5), **content** (sheets, clips, weapon
mapping, stats).

### Crates: new `sf_combat` (Bevy-free) + `sf_combat_bevy` (thin), not inside `sf_action`

`sf_action` is upstream of every character and edited by traversal and movement slices; combat changes daily through
C1-C7. Inside `sf_action`, every combat edit would rebuild `sf_action_bevy` and all its dependents (build-perf.md rules 3
and 5). As a leaf off `sf_action`, `sf_combat` rebuilds only itself, its shell and the game crates that fight; a classic
rules crate depends on `sf_combat` alone.

- **`sf_action` gets only the vocabulary its chooser must evaluate**: window kinds `Turn(rate)`, `NoTurn`, `Hit(n)`,
  `IFrames(mask)` (C1; `Charge`, `Armor`, `Lunge`, `KbScale`, `Deflect` later); the `Clip` motion owner; entry predicates
  `moveset` and `from`; a forced-move request; a start gate (`Sense::refused`); tap/hold sampling. All generic (action.md
  "Windows" already names them).
- **`sf_combat`** (deps `sf_action`, `sf_anim`, `sf_ids`, glam, serde, ron, thiserror; Bevy-free, in hakari's
  `final-excludes`): sheet loading and compilation, `Fighter` and vitals, hit/hurt shapes, sweeps, contact order,
  `CombatRules` + `StandardRules`, reaction pick, hit-stop curve, the combat step.
- **`sf_combat_bevy`**: components, `CombatSystems` sets in `FixedUpdate` after `ActionSystems::Move`, socket-track build at
  load, lock-on shell (`sf_camera::lockon` + `sf_camera_bevy::LockOn`), gizmos, presentation messages. Plugin
  `SfCombatPlugin<R: CombatRules + Resource>` (one monomorphization, in the game).

## Data model

### Moveset sheet (RON, record kind `Moveset`)

One file per weapon class and rig. Patchable like `MoveRule` (mods.md); C1 may load the builtin file the way
`TRAVERSAL_RULES` loads. The loader compiles each move into a generated `MoveRule` (id `<sheet>.<move>`, category, entry
classes, `moveset` and `from` predicates, `owner: Clip`, exits), the slot windows (`SlotInfo`) and a combat `MoveTable` entry
(costs, hits, i-frames). A second weapon class is a second file; nothing else changes.

```ron
(
    id: "sword_1h",
    rig: "humanoid",                         // the graph family whose slots it names
    defaults: (turn: 360.0, hit_stop: (hold: 0.06, ease: 0.10), blend_in: 0.10),
    weapon: (socket: "weapon", segment: ((0.0, 0.08, 0.0), (0.0, 0.95, 0.0)), radius: 0.08), // socket space, m
    moves: {
        "light1": (
            category: Attack, classes: [Light], from: [Any], rank: 10,
            slots: ["attack.sword_1h.light1", "attack.sword_1h.light1_rec"],
            windows: [                       // move time, s (after `rate`)
                Accept([Light, Heavy, Dodge], 0.10, End),
                Cancel([Dodge], 0.35, End), Cancel([Light, Heavy], 0.45, End), Cancel([Move], 0.80, End),
                Turn(360.0, 0.0, 0.15), NoTurn(0.15, 0.32),
            ],
            hits: [(window: (0.18, 0.32), mv: 1.0, stamina: 12.0, kind: "slash", tier: Small, stop: 0.05)],
            next: { Light: "light2" },
        ),
        "roll": (
            category: Dodge, classes: [Dodge], from: [Any], rank: 10,
            slots: ["dodge.roll.fwd"], rate: 1.25, root_scale: 0.75, square_up: Stick,
            cost: 12.0, iframes: [(0.02, 0.40, All)],
            windows: [Cancel([Light, Heavy], 0.75, End), Cancel([Dodge], 0.80, End), Cancel([Move], 0.95, End), NoTurn(0.0, End)],
            locked: { Left: "roll_left", Right: "roll_right" },   // 4-way when locked on
        ),
    },
    reactions: { (Small, Front): "react.small.front", (Medium, Front): "react.medium.front" /* ... */ },
    costs: (jump: 10.0, sprint_per_s: 11.0),
)
```

- **Chains**: `next` per class becomes `from: ["<sheet>.<this>"]` on the target move at a higher rank than the opener, so the
  chooser's priority list picks it (no combo timer; leaving the move resets the chain, R6 §4). Past the last light, `next`
  wraps to `light1`. Openers (run, roll, backstep attacks) are moves with `from: [...]` states (C3).
- **Windows** are authored against the slot's first candidate clip. Every other candidate brings its own (manifest `events`
  or the content pack's text-key synthesis); one that brings none gets the sheet windows scaled by the length ratio, with a
  load warning. Startup/active/recovery are derived from hit and cancel windows (R6 §12), never authored.
- **Hit row**: window, `mv` (motion value), `stamina` (cost, charged when the window opens), `kind` (content tag for the
  rules), `tier` (unguarded reaction), `stop` (hit-stop hold, s); later `poise`, `knockback`, `guard_damage`, flags
  `Unblockable | Sweep | Thrust | Grab`, `source: Sweep | Cone` (classic resolves by reach cone).
- **Load errors**: unknown slot, class, tier or `next` target; overlapping hit windows with one index; `Hit(n)` with no row.

### Fighter and vitals

`sf_combat::Fighter` (component `Combatant` wraps it): `id: CharacterId`, `moveset`, `vitals`, `weapon: WeaponShape`,
`hurt: HurtShape`, `offense { attack }`, `defense { absorb, guard_absorb, guard_stability }` (numbers the content fills;
classic ignores them), `hostile`, `min_tier` (player `Small`: every unguarded hit staggers). `Vitals { health, stamina,
poise: Option<_> }` of `Pool { cur, max }`; other pools stay content-side (`Outcome::events`). `FightState` (serde, beside
`ActionState`): the swing (move, start tick, landed `(hit, defender)` pairs, hits charged), hit-stop timer, regen open.

### Hit and hurt volumes

- **Weapon**: a segment and radius in socket space, from the sheet default or the content (weapon mesh length). Composed at
  runtime with the **baked socket track** of the playing slot: `sf_anim::SocketTrack` (socket transform in root space at
  60 Hz over the clip). C1 builds tracks at load from the cooked clip and skeleton (sf_anim pose sampling); a bake product
  (`anim.cook` `sockets`) later if load time asks for it. Gameplay never reads the evaluated pose (action.md).
- **Hurt**: one capsule from the character's mover capsule, radius x0.8, plus a head sphere (r 0.12 m at 0.93 x height); per-bone
  capsules from the ragdoll cook later (C4). Hit and hurt sizes are tuned as a pair (R6 §6, ER `88714a9`).

## The per-tick pipeline (fixed tick, 60 Hz)

In `FixedUpdate`, `CombatSystems::Gate` runs before `ActionSystems::Step`; the other sets after `ActionSystems::Move`.

1. **Gate**: classes with `rules.can_start(fighter, class) == false` go into `Sense::refused` (`sf_action_bevy::ActionGate`);
   a refused press is dropped (`Refused::Gate`, a `StaminaFlash` message).
2. **Chooser** (`sf_action::step`): the forced reaction first, then the priority list; combat moves are ordinary rules.
   Clip owner: yaw turns toward the lock target, else the stick, at the open `Turn` rate, never in `NoTurn`; translation is
   the slot root track's delta over the clock interval x `root_scale`, rotated by the current yaw. `ActionOutput` gains
   `clip: Option<ClipOutput { delta, yaw, progress }>` and the clock interval `(t0, t1)`.
3. **Move**: the shell applies the Clip delta through the mover's collide-and-slide, gravity on.
4. **Vitals**: a combat move start calls `rules.begin_swing` and charges its start cost; a hit window opening charges its
   row's stamina; `rules.regen` while regen is open (in `Loco`, or past the move's first `Cancel(Move)`).
5. **Sweep**: each open `Hit(n)` in `(t0, t1]`: the weapon segment at 5 sub-steps between the track samples at `t0` and `t1`,
   on the two ticks' root poses, against hurt shapes within the move's reach (segment-capsule distance). A first touch of
   `(n, defender)` is a `Contact`. Positions are relative to the attacker (WorldPos deltas, never `Entity`).
6. **Contact order** (SK, R6 §7): dead → skip; defender `IFrames` open for the row's flags → no landing, pair unspent;
   guard raised and attacker in the guard arc → `guarded` (C2); then `rules.resolve_contact`.
7. **Outcome**: pool deltas on both sides; tier = `max(rules hint or row tier, defender.min_tier)` (armor clamps it, C3),
   side from the hit direction in the defender's frame; death at health 0.
8. **Apply**: the reaction becomes `ActionState::forced` for the next `Step`; hit-stop starts on both sides; presentation
   messages (`HitLanded { point, kind, tier }`, `CameraShake`, sound tag).

Hit-stop: `ActionClock::rate` follows the curve (0 for `hold`, then `1 - (1 - u)^2` over `ease`), so windows, root motion,
slot progress and sweeps freeze together while input still latches. A new stop replaces a running one; Sim-owned
characters scale their intent by the rate; crowds shorten it later (R8 adopt 5).

## The rules seam

```rust
pub trait CombatRules: Send + Sync {
    /// Before the move's first hit window (classic decides hit or miss here). Modern: connects, strength 1.
    fn begin_swing(&self, swing: &Swing) -> SwingPlan;
    /// A sweep (or cone) met a defender, after the i-frame and guard tests.
    fn resolve_contact(&self, contact: &Contact, attacker: &Fighter, defender: &Fighter) -> Outcome;
    /// May `who` start a move of `class` now? Modern: stamina > 0 for costed classes. Classic: always.
    fn can_start(&self, who: &Fighter, class: ActionClass) -> bool;
    /// Pool cost of a move start or a hit window opening (`base` from the sheet, `strength` 0..1).
    fn cost(&self, who: &Fighter, base: f32, strength: f32) -> PoolDelta;
    /// Per-tick regeneration while regen is open.
    fn regen(&self, who: &Fighter, state: RegenState) -> PoolDelta;
}
```

- `Swing { attacker, move_key, strength, candidates: &[CharacterId] }` (candidates: a cone query for `source: Cone`, else
  empty); `SwingPlan { connects, strength, target: Option<CharacterId> }`.
- `Contact { attacker, defender, move_key, hit: u8, strength, kind: TagId, mv, flags, point, direction, side,
  defender_state { guarded, down, unaware, in_reaction } }`.
- `Outcome { landed: bool, defender: PoolDelta, attacker: PoolDelta, guard_break: bool, tier_hint: Option<Tier>,
  events: SmallVec<[TagId; 2]> }`; `PoolDelta { health, stamina, poise }`.
- Stats are the implementor's: the content's rules resource keeps its own per-character table keyed by `CharacterId`,
  refreshed in `CombatSystems::Stats` before resolve. The framework never reads stats. Random picks hash (seed, character,
  tick) (`sf_combat::roll`).
- **`StandardRules`** (modern, `sf_combat`): no hit roll; damage `attack x mv x strength x (1 - absorb)`; guarded: `x (1 -
  guard_absorb)` and stamina damage `guard_damage x 0.55 / guard_stability`, guard break at 0 (C2); `can_start` = stamina
  > 0 for Light/Heavy/Offhand/Dodge (the last action may overdraw, ER); costs = base; regen 45/s, x0.5 guarding. Content
  wraps or replaces it (stat bands, C5).

## Stamina economy

Spend per hit at window open (attacks), at start (roll, backstep, jump while a hostile is near), sprint per second while a
hostile fighter is within 20 m (calm-world sprinting is free, ER). Start only while stamina > 0. Regen has no delay timer:
it opens at `Loco` or the move's first `Cancel(Move)`, so long recoveries also delay it (ER). Guard (C2): regen x0.5,
blocked hits take stamina, 0 = guard break. Numbers come from content and rules; behaviour at 0 is the framework's.
Classic: `can_start` always true; fatigue scales effectiveness inside its rules.

## Lock-on and camera

The target is gameplay state (`ActionState::lock`, a stable id). A lock press acquires with `sf_camera::lockon::acquire`
over `Lockable` hostiles (the shell gathers candidates and line of sight), else recentres; `LockTimers::check` breaks it
(20 m, 1.5 s without line of sight, death), as does any traversal (re-offered on touchdown). The shell mirrors it to
`sf_camera_bevy::LockOn`, which already frames it. Locked locomotion sets `LocomotionIntent::desired_facing` to the
target, capped by the mover's own turn rates (`SpringParams::turn`, the player-feel retune): no second turn system. Clip
moves aim `Turn` windows at the target; locked rolls use the sheet's `locked` moves, unlocked rolls square up to the stick.
C4: switching by stick flick, soft aim without lock (~3 m, +-30 deg, first ~1/6 s of an attack, R6 §5), combat shake.
The Classic mode profile turns lock-on and hit-stop off (action.md "Mode profiles").

## Reactions, ragdoll and Delirium

`Tier { Additive, Small, Medium, Large, Knockdown, Death }` x `Side { Front, Back, Left, Right }` (ER four tiers plus SK's
additive minimum, R6 §7). The sheet's `reactions` table maps (tier, side) to a forced move; a missing side falls back to
Front, a missing tier to the next lower.
- **Additive**: no state change; presentation kicks the graph's cosmetic `HitReact` springs (`sf_anim_bevy::Hit`). Guard
  hits at minimum (C2), armor-clamped hits (C3).
- **Small / Medium / Large**: forced `Clip` moves, not cancellable (action.md category `Forced`), with their own root motion
  (`root_scale` keeps the push short) and windows (`Cancel(Dodge)` late in Large for a roll-out).
- **Knockdown / Death**: a forced move with `Owner::Physics`: `sf_action` emits `PhysicsTakeover { impulse, point }` and
  holds the move until the shell reports the hand-back; invulnerable while down, a roll-out window on the get-up. The
  content routes it to its ragdoll path (Bevyrrowind: the walkers' powered-ragdoll knockdown or the Delirium takeover,
  combat-adapter.md); framework code names neither. Today those react to thrown bodies (`sf_physics::PhysicsEvent::Hit`);
  weapon hits become a second source.

## Animation integration

- Clips play in the graph's full-body montage slot, `Progress` root mode, playhead = chain progress on the action clock
  (`set_slot_progress`), as the mantle does (action.md "As built (A2 slice 2)"); cancels blend in ~0.1 s (R6 adapt).
  Classic content may use an upper-body slot over locomotion.
- Slots `kind.family.variant`: `attack.<sheet>.<move>[_rec]`, `dodge.roll.{fwd,left,right}`, `react.<tier>.<side>`,
  `guard.<sheet>.{raise,idle,hit,break}`.
- **Sources**: candidates in the content's animation-sources manifest, first present wins, then the fallback
  (`ClipFallback::Clip` = the content's own clip, or `Drop`). Dropped slots leave the move's chain at compile (today an
  unknown slot holds the chain: fix in C1a); a move with every slot dropped is not loaded.
- **LaFAN1-optional rule**: a LaFAN1 (or any private-only) candidate needs a non-LaFAN1 candidate or a content fallback
  after it, checked at load. No C1 slot uses LaFAN1; later: falls, get-ups, stumbles.

## Co-op

Host-authoritative (action.md "Co-op readiness"). Client-predicted: own move starts, clocks, stamina costs, i-frames. Host
only: sweeps, contacts, rules, pools, reactions, deaths, lock validity. `FightState` replicates beside `ActionState`.

## Tests

Logic keeps exact assertions; feel tests assert sanity and print measurements (`#[ignore = "measurement: ..."]`). Scripted
`Command` streams run through `sf_action::step` + `sf_combat::step` headless (ER `fec0975`).
- Chain: a Light inside `Accept` starts `light2` exactly on the `Cancel(Light)` tick; a press before `Accept` is lost; leaving
  the move resets the chain; `light3` wraps to `light1`.
- Tap/hold: release within 17 ticks = `Dodge` on the release tick; held 30 ticks = `Sprint` from tick 18, no `Dodge`.
- Sweeps: one landing per (hit, defender); `light3` lands twice; out of reach never hits; open i-frames leave the pair
  unspent and it lands after they close inside the same window.
- Stamina: zero refuses Light and Dodge (`Refused::Gate`); cancelled before its hit window = no cost; regen only after
  `Cancel(Move)`; overdraw clamps at 0.
- Hit-stop: both clocks hold then ease; latched input fires after; a second hit replaces the stop.
- Seam: an always-miss double changes no pool and starts no hit-stop; an always-`can_start` double never refuses.
- Determinism: one command stream twice = byte-identical `ActionState` + `FightState` (postcard). Latency: first response
  on the press tick from `Loco` (R8 adopt 1).
- Measurements: first contact time vs dummy distance (0.8-2.5 m), stamina per chain, roll distance, i-frame share, hit-stop
  total.

## Build plan

| Slice | Deliverable | Test | Visual check |
|---|---|---|---|
| **C1a** core | `sf_action` vocabulary + `Clip` owner; `sf_combat` sheet, vitals, sweeps, contact order, `StandardRules`, hit-stop | Tests list, headless | none |
| **C1b** player vs dummy | `sf_combat_bevy`; game glue (sheet, clips, player, training dummy, level `arena`, stamina bar) | 3-hit chain on the dummy lands 4 hits; no NaN; dummy on the floor | CB-1 chain, roll, stamina, hit-stop |
| **C1c** lock-on | built 2026-10-27 (parts 1-4, "As built (C1c)"; strafe clips C1c-2b next): acquire/break, locked strafe, 4-way rolls, framing | breaks past 20 m and on a mantle | CB-2 strafe and roll around the dummy |
| **C2** guard, reactions | sparring dummy that strikes; player reactions by tier and side; guard; backstep; knockdown via `Physics` | contact-order unit tests; guard break at 0 | CB-3 block, guard break, knockdown, get-up |
| **C3** heavy, openers, poise | heavy + charge; run/roll/backstep/jump attacks; armor windows; poise; lunge; typed i-frames; second weapon class | second class adds no code; charge release swaps clip | CB-4 heavy and openers |
| **C4** targeting | switching, soft aim, combat shake, per-bone hurt capsules | switch picks the flicked side | CB-5 |
| **C5** classic rules | classic `CombatRules` (game side), cone contacts, Classic profile | formula tests against the ported source | CB-6 classic swing |
| **C6** NPC combat | NPCs and creatures through `Command` from `sf_ai`; per-rig sheets; paired takedown, ragdoll victim | AI uses baked reach and recovery | CB-7 |
| **C7** net | host-authoritative hits, own-start prediction (co-op phase F) | replay determinism | none |

action.md E1 = combat C1-C3 (windows, sheet, light/heavy/dodge/guard), E2 = C1c + C4 (lock-on, soft aim), E3 = C6.

## C1 brief

Order: C1a, then C1b, then C1c, one agent each. C1b and C1c touch `sf_controller` and `mw_player_bevy`: start them after
the `player-feel` session lands (`.sessions/player-feel.md`).

### C1b: player vs training dummy (Bevy + game)

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

### C1c: lock-on

**As built (C1c, 2026-10-27).** Four commits (a2e6d20, 3a54f4b, ddc252d, 2709b71); check CB-2.
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

## Open questions

1. Co-op hit-stop on a client's own attacker: wait for the host's `HitLanded`, or predict? Decide at C7.
2. Clips for heavy, charged, stagger tiers by side, guard-hit recoil, backstep, running and jump attacks are missing from the
   free packs (R1 §5): buy a pack, prototype with Mixamo, or use the content's own clips? Needed by C2/C3, not C1.
3. Player poise and hyper-armor numbers (C3): ER-like per-weapon armor windows, or a single poise pool first?
