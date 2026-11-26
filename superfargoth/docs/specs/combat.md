# Combat: feel layer, moveset sheets, rules seam

Status (2026-11-26): C1a-C1c (lock-on parts 1-4), C4 soft aim, C4b, C2a, C2b built; design updated to `docs/roadmaps/combat.md` (C0a); C0b-e still pending. ROADMAP 3.5. Game side:
`../../../bevyrrowind/docs/specs/combat-adapter.md`. Built on [action.md](action.md) (commands, windows, chooser, motion
owners, co-op rules: referenced, not repeated); owns its build-plan row E1-E3. Slices **combat C1..C7** are not action.md's
C1 (interactions).

## Goal

Third person feels like Elden Ring, first person like Dark Messiah, on one fighter, one sheet set and one rules core for
every view and actor ("Views"): committed attacks with clip windows, stamina, a spammable roll with i-frames, weapon sweeps
with hit-stop, symmetric poise, parry and riposte, warped finishers, a typed damage packet, kicks into hazards, lock-on in
third person. Game-agnostic: content supplies sheets, clips and rules; classic rules (roll to hit, fatigue, no i-frames,
lock-on or hit-stop) sit behind one seam (ARCHITECTURE §1b). Order and slices: [combat roadmap](../../../docs/roadmaps/combat.md).

Non-goals: *whether* and *whom* to fight (`sf_ai`, AI15; C6 only decides how an engaged fighter fights, C0b), view-aware
AI, paired clips and cutscene kills, damage numbers, weapon skills (later data), dismemberment and mounted combat (parked).
Ranged and magic are in scope, designed in C0c ("Delivery (C8)").

Owner direction: Elden Ring in third person (2026-10-27); first person after Dark Messiah / Faithkeeper (kicks, directional
and charged attacks, parries, environmental kills, finishers; 2026-11-15); swing direction from the movement keys (KCD2,
2026-11-12); one button, tap = roll, hold = sprint (2026-10-27). Clips are Morrowind's own, retimed (roadmap decision 6); the
roll stays UAL `Roll`. Reference:
[Brutal Morrowind](https://github.com/incident201/brutal-morrowind) (GPL-3.0 OpenMW 0.51.0 fork; diff before porting).

## Canonical technique

**FromSoftware's split: a fixed-tick decider with per-clip windows, weapon sweeps against hurt volumes, a per-character
hit-stop clock, and a rules layer behind the hit event.** Sources: R1 `../research/combat/01-elden-ring-reference.md` (ER
Combat-Rewrite, MIT), R6 `../research/animation/06-souls-combat-feel.md`, R8 `../research/animation/08-industry-combat-responsiveness.md`,
classic `../../../bevyrrowind/docs/research/combat-classic-openmw.md`. UE GAS keeps attributes apart from notifies, as our rules seam does.

Reuse (rule 8): Bevy has no combat layer and no maintained crate covers clip windows, weapon sweeps and a rules seam. Hits
are segment-capsule math on baked socket tracks, not physics shape casts (deterministic, host-side, LOD-proof). The ER
rewrite is a reference, not a dependency (a monolithic sim with a one-slot buffer; we have the chooser and per-class buffer):
we take its constants (with attribution, R1 flags) and its tests' shape. Lock-on math exists (`sf_camera::lockon`), and
framing and trauma shake (`sf_camera_bevy`).

Numbers adopted (R1, R6, R8; roll 12 stamina, regen 45/s, hit-stop ~60 ms + ~0.1 s ease, guard 4 ticks and 80 deg, lock 15 m /
break 20 m): [history](archive/combat-history.md) "Canonical technique: sources and adopted numbers"; live values in the sheet.

## Reuse check

C0a parts (2026-11-26): Bevy 0.20 has no combat layer; no maintained crate does (survey 2026-11-25, roadmap decision 1); the
ER Combat-Rewrite (MIT, Bevy 0.19) has no parry, poise or AI: timing reference only. So: our own code in `sf_combat`, each
part a field on `Contact`, `Outcome` or `MoveSpec` or a new `source`, never a second pipeline. Reused: the `Warp` owner
(action.md), `PhysicsTakeover` + the content's ragdoll, `sf_interaction` volumes, `sf_effects` status ticks, `sf_camera_bevy`.

## Industry comparison

| Part | Industry | Ours |
|---|---|---|
| Poise | ER stance (hidden pool, break = stagger, bosses kneel) + hyper-armour windows [V eldenring.wiki.gg] | one pool for all; `Armor` windows |
| Parry | Lies of P perfect guard (first frames of the raise) [M]; Mordhau parry + riposte [V pcgamer]; Sekiro payoff = posture break | window on raise, attacker poise damage, no HUD bar |
| Criticals | ER backstab/riposte x2-3.5, paired clips [M]; UE Motion Warping aligns a montage to a target | rules multiplier; two ordinary clips aligned by `Warp` |
| Damage | ER types, per-type defence, status buildup [M]; UE GAS attributes apart from notifies | packet ≤ 3 types + ≤ 2 buildups, ids only |
| Push | Dark Messiah kick, hazard kills [M] | `Kick` class, `Environment` contacts with credit |
| Views | Avowed one timing in both views [V gamedeveloper]; Chivalry 2, Mordhau: 3P is a camera option, feint/morph for all [V gamepressure] | one rules core, per-view profile |
| Unseen hits | Doom/Arkham token caps, Arkham attacker icon [V Sulman]; Dead Island 2 cuts unseen damage | tokens + cues; no view-aware rule (owner Q9) |

## Architecture

Three layers, each testable alone: **feel** (framework, fixed tick: windows, motion, vitals, sweeps, contact order,
reactions, hit-stop, lock-on; owns timing, never computes damage), **rules** (the `CombatRules` seam: damage, costs, hit or
miss, regen; modern `StandardRules` in `sf_combat`, classic in the content pack, C5), **content** (sheets, clips, weapon
mapping, stats).

### Crates: new `sf_combat` (Bevy-free) + `sf_combat_bevy` (thin), not inside `sf_action`

Why a leaf crate (rebuild cost): [history](archive/combat-history.md) "Crate split rationale".

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
Live sheet: `bevyrrowind/crates/mw_combat/data/movesets/sword_1h.ron`; the C1 shape: [history](archive/combat-history.md) "Moveset sheet example (the C1 shape)".

- **Chains**: `next` per class becomes `from: ["<sheet>.<this>"]` on the target move at a higher rank than the opener, so the
  chooser's priority list picks it (no combo timer; leaving the move resets the chain, R6 §4). Past the last light, `next`
  wraps to `light1`. Openers (run, roll, backstep attacks) are moves with `from: [...]` states (C3).
- **Windows** are authored on the slot's first candidate clip; others bring their own (manifest `events`, text-key
  synthesis) or get the sheet windows scaled by length, with a warning. Startup/active/recovery derive from hit and cancel
  windows (R6 §12).
- **Hit row**: window, `mv` (motion value), `stamina` (charged when the window opens), `kind` (content tag), `tier` (reaction
  on a poise break, or always without a pool), `stop` (hit-stop hold, s), `guard_damage` (default `stamina`), `poise`,
  `damage` (typed split), `push`; flags `Unblockable | NoParry | Critical | Sweep | Thrust | Grab`; `source: Sweep | Cone`
  (classic resolves by reach cone).
- **Guard** (C2b; parry C2d): a move with `classes: [Guard]` is a hold move (it lasts while held; its last slot loops);
  `guard: (raise, arc, parry)` makes it guard (raise and parry window s, rounded to ticks; half-arc deg). Optional slots
  `guard_hit`, `guard_break`, `guard_parry` (defender) and `parried` (attacker).
- **Load errors**: unknown slot, class, tier or `next` target; overlapping hit windows with one index; `Hit(n)` with no row; a
  `guard` on a non-hold move, a guard arc outside (0, 180].

### Fighter and vitals

`sf_combat::Fighter` (component `Combatant` wraps it): `id: CharacterId`, `moveset`, `vitals`, `weapon: WeaponShape`,
`hurt: HurtShape`, `offense { attack }`, `defense { absorb, guard_absorb, guard_stability }` (numbers the content fills;
classic ignores them), `poise: Option<PoiseSpec>`, `hostile`, `min_tier` (reaction floor, default `Small`). `Vitals {
health, stamina, poise: Option<_> }` of `Pool { cur, max }`; other pools stay content-side (`Outcome::events`). `FightState`
(serde, beside `ActionState`): the swing (move, start tick, landed `(hit, defender)` pairs, hits charged), hit-stop timer,
regen open, last poise damage tick, `poise_broken_until`, last guard raise, `riposte_until`, `pushed_by: (CharacterId, until)`.

### Hit and hurt volumes

- **Weapon**: a segment and radius in socket space (sheet default or content), composed with the playing slot's **baked
  socket track** (`sf_anim::SocketTrack`, root space, 60 Hz, built at load). Gameplay never reads the evaluated pose (action.md).
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
   row's stamina; `rules.regen` while regen is open (in `Loco`, or past the move's first `Cancel(Move)`; `Guarding` throughout
   a guard move).
5. **Sweep**: each open `Hit(n)` in `(t0, t1]`: the weapon segment at 5 sub-steps between the track samples at `t0` and `t1`,
   on the two ticks' root poses, against hurt shapes within the move's reach (segment-capsule distance). A first touch of
   `(n, defender)` is a `Contact`. Positions are relative to the attacker (WorldPos deltas, never `Entity`).
6. **Contact order** (SK, R6 §7), the same for every `source`: dead → skip; defender `IFrames` open for the row's flags → no
   landing, pair unspent; guard move running, attacker within its arc, row not `Unblockable`: parry window open and row not
   `NoParry` → `defender_state.parry` ("Parry"), else raised (clock before this tick >= `raise`) → `defender_state.guarded`
   (C2b); critical eligibility → `Contact.critical`; then `rules.resolve_contact`.
7. **Outcome**: pool deltas on both sides; side from the hit direction in the defender's frame; death at health 0.
   `parried`: the defender's `guard_parry`, the attacker's `parried`. `guarded`: tier `Additive` and `guard_hit` (none: the
   guard holds); `guard_break`: tier `Large` and `guard_break` (else the Large reaction). Otherwise poise picks the tier
   ("Poise"); no pool: `max(rules hint or row tier, defender.min_tier)`.
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
- `Contact { attacker, defender, source, credit: Option<CharacterId>, move_key, hit: u8, strength, kind: TagId, mv, flags,
  guard_damage, poise, damage, push, critical: Option<CritKind>, point, direction, side, defender_state { guarded, parry,
  down, unaware, in_reaction, poise_broken } }`; `source: Sweep | Cone | Projectile | Beam | Area | Environment`
  (Projectile, Beam, Area: C0c "Delivery (C8)").
- `Outcome { landed: bool, defender: PoolDelta, attacker: PoolDelta, guarded: bool, guard_break: bool, parried: bool,
  critical: Option<CritKind>, damage: SmallVec<[(DamageType, f32); 3]>, buildup: SmallVec<[(StatusId, f32); 2]>, tier_hint:
  Option<Tier>, events: SmallVec<[TagId; 2]> }`; `PoolDelta { health, stamina, poise }`. `guarded` and `parried` are the
  rules' answers (their reactions play), so a classic rule can block with no guard move.
- Stats are the implementor's: the content's rules resource keeps its own per-character table keyed by `CharacterId`,
  refreshed in `CombatSystems::Stats` before resolve. The framework never reads stats. Random picks hash (seed, character,
  tick) (`sf_combat::roll`).
- **`StandardRules`** (modern, `sf_combat`): no hit roll; damage `attack x mv x strength x (1 - absorb)`; guarded: `x (1 -
  guard_absorb)` and stamina damage `guard_damage x 0.55 / guard_stability` (stability floor 0.1), guard break when that
  reaches 0 (C2b); `can_start` = stamina
  > 0 for Light/Heavy/Offhand/Dodge (the last action may overdraw, ER); costs = base; regen 45/s, x0.5 guarding. Content
  wraps or replaces it (stat bands, C5).
- `StandardRules` (C0a parts): poise damage = row `poise` x `Armor` scale; parry granted when offered; `combat.crit.*`
  multipliers; one damage entry of the content's default type, no buildup.

## Stamina economy

Spend per hit at window open (attacks), at start (roll, backstep, jump while a hostile is near), sprint per second while a
hostile fighter is within 20 m (calm-world sprinting is free, ER). Start only while stamina > 0. Regen has no delay timer:
it opens at `Loco` or the move's first `Cancel(Move)`, so long recoveries also delay it (ER). Guard (C2b): regen x0.5,
blocked hits take stamina, 0 = guard break; raising a guard costs nothing and works at 0 (the next block breaks it, ER). Numbers come from content and rules; behaviour at 0 is the framework's.
Classic: `can_start` always true; fatigue scales effectiveness inside its rules.

## Poise

C3a, decision 2, owner Q2 (heavy armour shrugs off light hits). One system for player, NPCs, creatures and bosses; it
replaces "every unguarded hit staggers".
- **Data**: `PoiseSpec { break_tier: Option<Tier>, regen_delay, regen_per_s }`; pool `Vitals.poise` (max from content, C0d);
  row `poise` (default `mv x combat.poise.per_mv`); window `Armor(scale)`: poise damage x `scale` (hyper-armour, cast armour).
- **Unguarded contact**: the rules return `defender.poise`. Holds (cur > 0): tier `Additive`. Breaks: `break_tier`, else
  `max(row tier, min_tier)` one step up, `Knockdown` past `combat.poise.knockdown_overflow` x max; refill to max;
  `poise_broken` for `combat.poise.broken_s` (riposte-eligible). No pool (dummy, classic): row tier, as built.
- **Regen** after `regen_delay` from the last poise damage, at `regen_per_s`, through `rules.regen`.
- Bosses: large max, `break_tier: Knockdown` (ER stance break). Guarded hits cost stamina, not poise. A break inside a
  cast's `Armor` window cancels the cast (C8d, C0c).
- Tunables: `combat.poise.per_mv` (poise/mv, 0-100), `.regen_delay_s` (s, 0.5-5), `.regen_per_s` (max/s, 0.05-1),
  `.broken_s` (s, 0.3-3), `.knockdown_overflow` (x max, 0-2); all Guess.

## Parry

C2d, decision 3, owner Q1 (a tight window on raising the guard, riposte, both views). No posture bar.
- **Data**: `GuardSpec { raise, arc_cos, parry_ticks }` (default `combat.parry.window_s`); row flag `NoParry`; slots
  `guard_parry` (defender, its windows hold `Accept([Riposte], ..)`) and `parried` (attacker).
- **Window**: guard clock `[0, parry_ticks)` from the press, overlapping the raise (Lies of P); none if the last raise was
  under `combat.parry.rearm_s` ago (no mashing). Tested before guarded (step 6).
- **Parried**: no damage of any kind to the defender; the attacker takes `combat.parry.poise` x its max poise, plays
  `parried` (a break escalates it) and is riposte-eligible for `combat.parry.riposte_s`. Classic: no window, never `parried`.
- Tunables: `combat.parry.window_s` (s, 0.05-0.3, Guess 0.13), `.rearm_s` (s, 0-1), `.poise` (x max, 0-2), `.riposte_s` (s,
  0.3-2); Guess.

## Criticals and finishers

C2e, decision 4, owner Q5 (heavy damage, a short warped finisher, never a cutscene).
- **Eligibility** (before `Step`, into `Sense.critical: Option<(CritKind, CharacterId)>`): `Backstab` = within
  `combat.crit.reach_m` inside the target's back cone (`.back_half_angle_deg`), or target `unaware` (content fills it from
  perception, adapter hook); `Riposte` = target `poise_broken` or parried, in reach, in front. Never down, dead or mid-finisher.
- **Move**: Light with `Sense.critical` set requests class `Riposte`; sheet `backstab` / `riposte` moves (`classes:
  [Riposte]`, predicate `critical`, rank over `light1`) have `Critical` rows.
- **Alignment**: the attacker is `Warp`-owned onto a victim-relative slot (`finisher.offset`), linear snap over
  `combat.crit.snap_s` (~0.2 s), then held; the victim plays a forced `(Critical(kind), side)` react or death clip,
  `Warp`-locked to the attacker. Both hold `IFrames(All)` while locked (ER [M]). A snap over `.max_snap_m` or a blocked end
  pose falls back to the normal attack.
- **Damage**: `Contact.critical`; the rules multiply. The victim ends in `PhysicsTakeover` (death or knockdown).
  Crime's sneak-attack multiplier is the content's backstab multiplier; stealth takedowns are backstabs tagging
  `Knockout` (C15, C16).
- Tunables: `combat.crit.reach_m` (m, 0.5-3, Guess 1.5), `.back_half_angle_deg` (deg, 20-90, Guess 45), `.snap_s` (s,
  0.1-0.4, Reference ER), `.max_snap_m` (m, 0.3-2, Guess), `.backstab_mult` / `.riposte_mult` (1-5, Reference ER 2-3.5).

## Damage packet

C12, decision 5, owner Q6. Fields on `Contact` and `Outcome`, no second pipeline.
- `DamageType`, `StatusId`: content ids (`sf_ids`). Row `damage: [(DamageType, ratio)]` splits `mv`; absent, the default type.
- `Outcome.damage` is after the rules' armour and resistances (for `HitLanded`, sound, VFX); `defender.health` is its sum.
  `Outcome.buildup` goes to `sf_effects` as `(target, status, amount, credit)`; meters, procs and ticks are its own.
- The adapter owns resistances (C0d, C13); Morrowind ships no buildup at first. 3 and 2 are capacities (consts); more types
  keep the largest, with a load warning.

## Push and environment contacts

C9, C10, decision 12 (owner 2026-11-12 "dedicated kick with physics pushes"; Dark Messiah).
- **Push**: `Kick` moves and shield bash carry row `push: (impulse, up)` (N s along the hit, upward tilt). Landing sets the
  defender's `pushed_by` for `combat.push.credit_s`; Knockdown → `PhysicsTakeover { impulse, point }` (C2c), lesser tiers
  scale the reaction's root motion (cap `combat.push.max_shove_m`).
- **Hazards**: `sf_interaction` data (IN28) `Hazard { kind, damage: [(DamageType, f32); ≤3], every: Once | Interval(s),
  lethal }`. A body inside one (walking or ragdolled) gets `Contact { source: Environment, credit: pushed_by }` through the
  same order (attacker = a stand-in `Fighter`); i-frames apply, guard and parry don't. Thrown bodies and props (IN36) above
  `combat.push.body_hit_mps` are Environment contacts credited to the thrower.
- **Credit** reaches `HitLanded` and `Died { killer }` (C15, C16); a fall death reports `pushed_by` as the killer.
- Tunables: `combat.push.credit_s` (s, 1-10, Guess 3), `.max_shove_m` (m, 0.2-3), `.body_hit_mps` (m/s, 2-15); Guess.

## Views

Decision 19 (owner 2026-11-26, Q7-9; research `gameplay-2026-11-25/combat-views.md` §3).
- **View-independent** (identical for player, NPC, remote player): `ActionState`, `FightState`, moves, clocks, windows,
  i-frames, hit-stop, root motion, contacts, poise, parry, criticals, damage, stamina, tokens, the body's clips. `sf_combat`
  and `sf_ai` have no view input.
- **Per-view profile** (input and presentation data, `sf_action_bevy` and `sf_camera_bevy`, never `sf_combat`): targeting
  (third person lock-on + flick; first person soft aim 3 m / ±30 deg and the 45 deg look limit, built), facing source
  (`face_view` in first person), the dodge's presentation, camera (FOV, shake, comfort) and feedback (trails, VFX, haptics).
- **One dodge** (C18, Q7): one `MoveSpec` (timing, i-frames, cost; CB-6) in both views; the body plays the roll clip, so a
  partner sees a roll; the first-person camera follows a side-dash path along its root motion, no tumble
  (`camera.fp_dodge.{lateral_m, dip_m, roll_deg}`, Guess). Locked 4-way rolls exist only in third person.
- **Switching mid-fight** (Q8): instant; the running move keeps its clock, windows and yaw; only presentation and the next
  steering input change. Entering first person drops the lock that tick (contacts never read it).
- **No view-aware AI** (Q9: fragile in multiplayer): first-person fairness = cues every player gets (C19: audio telegraph
  at windup start, hit-direction indicator from `HitLanded.direction`) + the view-independent token cap (C6a, C0b).
- **Feint and morph** (decision 7, C11): sheet data in both views, gated by input scheme: feint = a windup `Cancel` closing
  `combat.feint.cutoff_s` before the first hit window (s, 0-0.2, Reference Mordhau); morph = chain `next` by look delta.
- Co-op PvE allows mixed views; a host "force first person" option only if PvP ever exists.

## Lock-on and camera

The third-person profile of "Views"; contacts never read the lock.
The target is gameplay state (`ActionState::lock`, a stable id): a lock press acquires (`sf_camera::lockon::acquire` over
`Lockable` hostiles in line of sight), else recentres; `LockTimers::check` breaks it (20 m, 1.5 s unseen, death, any
traversal); `sf_camera_bevy::LockOn` frames it. Locked locomotion faces the target under the mover's own turn rates (no
second turn system), `Turn` windows aim at it, locked rolls use the sheet's `locked` moves; unlocked rolls square up to the
stick and attacks soft-aim (~3 m, +-30 deg, first ~1/6 s, R6 §5). Left: C4c. The Classic profile turns lock-on and hit-stop
off (action.md "Mode profiles").

## Reactions, ragdoll and Delirium

`Tier { Additive, Small, Medium, Large, Knockdown, Death }` x `Side { Front, Back, Left, Right }` (ER four tiers plus SK's
additive minimum, R6 §7). The sheet's `reactions` table maps (tier, side) to a forced move; a missing side falls back to
Front, a missing tier to the next lower.
- **Additive**: no state change; presentation kicks the graph's cosmetic `HitReact` springs (`sf_anim_bevy::Hit`). Guard
  hits at minimum (C2), armor-clamped hits (C3).
- **Small / Medium / Large**: forced `Clip` moves, not cancellable (action.md category `Forced`), with their own root motion
  (`root_scale` keeps the push short) and windows (`Cancel(Dodge)` late in Large for a roll-out).
- **Knockdown / Death**: a forced move with `Owner::Physics`: `sf_action` emits `PhysicsTakeover { impulse, point }` and
  holds the move until the shell's hand-back; invulnerable while down, a roll-out window on the get-up. The content routes it
  to its ragdoll (Bevyrrowind: walkers' powered ragdoll or Delirium, combat-adapter.md).

## Animation integration

- Clips play in the graph's full-body montage slot, `Progress` root mode, playhead = chain progress on the action clock
  (`set_slot_progress`), as the mantle does (action.md "As built (A2 slice 2)"); cancels blend in ~0.1 s (R6 adapt).
  Classic content may use an upper-body slot over locomotion.
- **Draw / sheathe** (built 2026-11-05; [history](archive/combat-history.md) "Draw / sheathe as built"): an upper-body clip
  over locomotion, timed on the fixed tick from the baked attach key (`sf_character::ReadyChange`).
- Slots `kind.family.variant`: `attack.<sheet>.<move>[_rec]`, `dodge.roll.{fwd,left,right}`, `react.<tier>.<side>`,
  `guard.<sheet>.{raise,idle,hit,break}`.
- **Sources**: the content's animation-sources manifest, first present wins, then `ClipFallback::Clip` or `Drop`; dropped
  slots leave the chain, a move with every slot dropped is not loaded.
- **LaFAN1-optional rule**: a LaFAN1 (or any private-only) candidate needs a non-LaFAN1 candidate or a content fallback
  after it, checked at load. No C1 slot uses LaFAN1; later: falls, get-ups, stumbles.

## Co-op

Host-authoritative (action.md "Co-op readiness"). Client-predicted: own move starts, clocks, stamina costs, i-frames. Host
only: sweeps, contacts, rules, pools, reactions, deaths, lock validity. `FightState` replicates beside `ActionState`.
Poise, parry, criticals, buildup and push credit are host state. Hit authority: net.md "Hit authority" (C0e, decision 15).

## Tests

Logic keeps exact assertions; feel tests assert sanity and print measurements (`#[ignore = "measurement: ..."]`). Scripted
`Command` streams run through `sf_action::step` + `sf_combat::step` headless (ER `fec0975`).
The C1 test list (chain, tap/hold, sweeps, stamina, hit-stop, seam, determinism, latency, measurements): [history](archive/combat-history.md)
"C1 test list"; each slice's tests are in its as-built notes.

## Owner checks (CB-*)

In `bevyrrowind/docs/visual-checks/combat.md`. The spec's old "CB-6 classic swing" is CB-14; CB-6 is the confirmed roll.

| ID | Checks (slice) |
|---|---|
| CB-1, CB-2, CB-3, CB-3a, CB-6 | confirmed: light chain + roll + hit-stop (C1b; re-check on Morrowind clips), lock-on (C1c), draw/sheathe, sparring reactions (C2a), spammable roll |
| CB-3b, CB-3c | guard + still pose + break (C2b, C2b2; partial); backstep, knockdown, get-up (C2c) |
| CB-4, CB-4b | heavy, charge, openers (C3b, C3c); greatsword, spear, fist (C3d) |
| CB-5, CB-5b, CB-5c | first-person soft aim, look limits (C4, C4b); lock flick switch, shake (C4c) |
| CB-5d, CB-5e | feint and morph in both views (C11); first-person side dash (C18) |
| CB-7, CB-7b | duelist (C6e); audio telegraph, hit-direction indicator (C19) |
| CB-8, CB-8b | parry and riposte (C2d); backstab and riposte finishers (C2e) |
| CB-9, CB-9b | kick (C9); kick into a hazard, credited kill (C10) |
| CB-10, CB-11 | poise (C3a); creature fighters, boss phases (C6f) |
| CB-12, CB-13, CB-14 | bow (C8b); cast over locomotion (C8d); classic swing (C5) |

## Build plan

| Slice | Deliverable | Test | Visual check |
|---|---|---|---|
| **C1a** core | `sf_action` vocabulary, `Clip` owner; `sf_combat` sheet, vitals, sweeps, contact order, `StandardRules`, hit-stop | headless | none |
| **C1b** player vs dummy | `sf_combat_bevy`, game glue, dummy, level `arena`, stamina bar | 3-hit chain lands 4 hits | CB-1 |
| **C1c** lock-on | built (parts 1-4); strafe clips C1c-2b | breaks past 20 m, on a mantle | CB-2 |
| **C2** guard, reactions | **C2a**, **C2b** built 2026-11-10; **C2c** backstep, knockdown via `Physics` | contact order; guard break at 0 | CB-3a, CB-3b, CB-3c |
| **C3** heavy, openers, poise | C3a-d (poise, heavy + charge, openers, second classes) | second class adds no code | CB-10, CB-4, CB-4b |
| **C4** targeting | soft aim, C4b built; C4c flick switch, shake; per-bone hurt shapes (S5) | flicked side; soft-aim pick | CB-5, CB-5b, CB-5c |
| **C5** classic rules | classic `CombatRules` (game side), cone contacts, Classic profile | formula tests | CB-14 |
| **C6** NPC combat | tokens, picker, skill, telegraphs (C0b); per-rig sheets | baked reach and recovery | CB-7 |
| **C7** net | host-authoritative hits, own-start prediction (co-op phase F) | replay determinism | none |

action.md E1 = combat C1-C3 (windows, sheet, light/heavy/dodge/guard), E2 = C1c + C4 (lock-on, soft aim), E3 = C6.

Lane slices, order = `docs/roadmaps/combat.md` stages (depends and verify there): C2b2 still guard + regen delay; C2c
backstep, knockdown; C2d parry; C2e criticals; C3a poise; C3b heavy + charge; C3c openers; C3d second classes; C4c
third-person profile; C8a-e delivery (C0c); C9 kick; C10 hazards; C11 feint and morph; C12 damage packet; C13 Morrowind
damage mapping; C14 prop targets; C15 hit and death events; C16 offences; C17 NPC fighters without AI; C18 first-person
dodge; C19 threat cues.

## As built by slice

Built: C1a, C1b, C1c parts 1-4, C4 soft aim, C4b, C2a, C2b. As built, tests and what is not built: [history](archive/combat-history.md)
"As built by slice (moved from combat.md 2026-11-26)" and the entries above it.

## Open questions

1. Co-op hit-stop on a client's own attacker: wait for the host's `HitLanded`, or predict? Decide at C7.
2. Clips for heavy, charged, stagger tiers by side, guard-hit recoil (C2b: none; a guard walk too), backstep, running and jump
   attacks are missing from the free packs (R1 §5): buy a pack, prototype with Mixamo, or use the content's own clips? Needed by C2/C3, not C1.
   Answered 2026-11-26 (decision 6): Morrowind's own clips; missing moves as key-pose clips (AN37).
3. Player poise and hyper-armor numbers (C3): ER-like per-weapon armor windows, or a single poise pool first?
   Answered 2026-11-26 (decision 2): one symmetric pool plus `Armor` windows ("Poise"); numbers in C0d.
