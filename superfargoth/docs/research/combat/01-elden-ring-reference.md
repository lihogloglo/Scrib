# Combat R1: Elden Ring reference (ELDEN-RING-Combat-Rewrite) and the shape of our combat layer

Research for the combat spec (ROADMAP 3.5, action.md E1-E3). Owner 2026-10-27: "implement combat, taking Elden Ring as a
reference ... a generic combat system that can also work for [classic rules], but the ultimate goal is an Elden Ring feel in
third person". Overlaps and extends `../animation/06-souls-combat-feel.md` (R6, cited "R6 §n"); this file adds the constants,
the slice plan and the seam. Classic rules: `bevyrrowind/docs/research/combat-classic-openmw.md` (game side).

Reference: github.com/Funny-Bones/ELDEN-RING-Combat-Rewrite, MIT code, HEAD `177a3b5` (pulled 2026-10-27), Bevy 0.19. Paths
below are relative to `C:\cargo-target\refs\elden-ring-combat-rewrite\`. Its data (`src/sim/extracted.rs`, animations) is
generated from the owner's own game and is not in the repo, so per-clip frame numbers cannot be read here; only the constants,
the extractor (`tools/extract.py`, which shows where each number comes from) and tests that quote some. Flags:
**[X]** extracted from the game files (README.md "Read from the game's files"), **[E]** estimate (README.md "Still estimated",
or marked `ESTIMATE`), **[T]** value seen only in a test assertion.

## 1. What the reference does, system by system

**Shape.** The whole game is a pure 60 Hz state machine with no engine types (`src/sim/`, README.md "Layout"). Animation
frames are 30 fps, one tick is half a frame, `DF = 0.5` (`src/sim/data.rs:20-23`). Same split as our action.md: the sim decides,
the rig follows. Player state is `Ground | Act | Air | Dead`; `Player::step` is `player.rs:407-455`.

**Action data layout** (`data.rs:384-418` `ActionDef`, one per animation, read from TAE events by `tools/extract.py:405-490`):
`total` length; `input_from`, `input_dodge_from` (accept windows); `cancel_{light,heavy,dodge,jump,guard,move,left}` (earliest
frame per input class); `iframes (from,to)`; `jump_frames` (low attacks pass under); `hits[]`; `charge (from,to)`;
`no_turn[]`; `turn[(from,to,deg/s)]`; `motion[]` (cumulative root motion per frame). All **[X]**. Each `Hit`
(`data.rs:353-375`): window, `mv` motion value, `guard_damage`, `stamina`, `stop` (hit-stop seconds), radius, `blade[]` (two
weapon points per frame in character space). Hit numbers are **[X]** (attack param, `extract.py:379-383`); the blade points and
the 0.1 m radius are **[E]** (stand-in models, `extract.py:48-75`). Event ids: `extract.py:129-135`. 24 weapon classes, each
with 1h and 2h movesets (`extract.py:35-62`; attack ids `ATTACKS`, 2h = +2000).

**Chains** (`player.rs:988-1027`). Light: `Light1..Light6`, length per weapon, past the end it wraps to `Light1`. Openers (run,
roll, backstep, crouch attack) continue into `Light2`, not `Light1` (`player.rs:991-995`; README says this is inferred from the
game's script). Heavy: `Heavy1Charge` -> `Heavy2Charge`; a tap releases into `Heavy1`, holding past the charge window commits
to the charged hit. Opener choice from state (`player.rs:513-560`): sprint + light = `RunLight`, sprint + heavy = `RunHeavy`,
after a roll = `RollAttack`, after a backstep = `BackstepAttack`, crouched = `CrouchAttack`, guard counter ready + heavy =
`GuardCounter`. Jump attack: once per jump, from a window the jump clip opens (`try_air_attack`, `player.rs:819`). Left-hand and
paired (power stance) chains have the same shape on the left button (`player.rs:1006`).

**Windows and buffering.**
- Accept: a press before `input_from` is dropped, not queued (`read_buttons`, `player.rs:457-500`; test `tests.rs:154`).
- One buffer slot, newest press wins (`player.rs:200`; `Req` 60-69; comment at 483). A buffered request fires on the first tick
  its class's `cancel_*` frame is reached (`act`, `player.rs:711-742`; test `tests.rs:167`). R6 rejects the single slot (a
  dodge cannot wait behind an attack); our `Accept(ClassSet)` is per class.
- Dodge fires on release, only if held less than `SPRINT_HOLD_FRAMES = 10` (`data.rs:63`, **[E]**); held longer is a sprint.
- Move cancel is the latest window: that is where weight comes from (R6 §2). Stamina regen runs only after `cancel_move`
  (`player.rs:744,761`).
- Charge: releasing inside the `charge` window swaps to the uncharged clip (`player.rs:660-675`). Tap heavy mv 1.25, held 1.60;
  stamina 20 vs 30 **[T]** (`tests.rs:253-281`).

**Stamina** (`data.rs:27-45`). Roll 12, backstep 8, jump 10 **[X]** ("shared script constants"). Attack cost is per hit, taken
when that hit's window opens, so a cancelled swing costs less (`player.rs:678-683`) **[X]**. Regen 45/s **[E]**, only in free
states and x0.5 while guarding **[E]**; sprint drain 11/s, only while an enemy is hostile **[E]**; no sprint in a calm world
(`tests.rs:332`). An action starts only if `stamina > 0` (`player.rs:519,722`); costs clamp at 0, so the last action may
overdraw. There is no regen delay timer: regen simply starts at `cancel_move`. Max HP and stamina come from the Vagabond class
through the game's stat curves **[X]** (`extract.py:221-243`).

**Poise / stance: none for the player.** Any unguarded hit that lands plays `Hurt(level, side)`; nothing cancels it early
(`player.rs:1058-1078`, test `tests.rs:707`). Poise exists only on the dummy (`dummy.rs:17,107-120`: pool 60,
`take_hit(damage, guard_damage*5)`, break gives a 1.6 s stagger, refill after 5 s unhit), **[E]**, invented. Reaction tiers
`HurtLevel {Small, Middle, Large, Knockdown}` x `Dir {Front, Back, Left, Right}` chosen by the side the hit came from
(`data.rs:121-127`, `player.rs:1064-1078`). Knockdown throws the player back by root motion, invulnerable while down, roll out
early **[X]** (README "Combat"). The big gap against real Elden Ring (poise, hyper-armor): take SK's armor windows (R6 §2).

**I-frames.** `iframes (from,to)` per animation from the dodge flag **[X]**. Medium roll: 13 frames = 26 ticks, 3.65 m **[T]**
(`tests.rs:101-114`). Roll type by `Load {Light, Medium, Heavy}`, chosen by key 1/2/3 in the sandbox, not computed from weight;
heavy-roll exit speed 0.4 x run **[E]** (`data.rs:480-505`). Rolls go four ways only when locked on; free rolls face the roll
direction (`square_up`, `player.rs:949-967`). Backstep = dodge without stick. Jump i-frames: `jump_frames` lets low sweeps pass
at clearance 0.35 m **[E]** (`data.rs:77`, `player.rs:1037`). No typed i-frames (SK has general/sweep/thrust/grab, R6 §2).

**Guard and counter** (`receive_hit`, `player.rs:1029-1080`). Order: dead, i-frames (`Dodged`), low sweep vs airborne
(`Jumped`), guard test, damage. Guard counts when raised (`GUARD_RAISE_FRAMES = 4`) and the attacker is within an 80 degree
half-arc (`GUARD_ARC_DEG`); stamina taken is `hit.stamina x 0.55`; `<= 0` is `GuardBreak`, else `GuardHit` plus a 20-frame
`GUARD_COUNTER_WINDOW` in which heavy becomes `GuardCounter` (`data.rs:84-92`, all **[E]**). No parry (README "Not included").

**Hit detection** (`mod.rs:158-188`, `player.rs:333-395`). The baked weapon line (two points per frame, linearly interpolated)
is swept over the tick in `SWEEP_STEPS = 5` sub-steps (`player.rs:176,374`) as a capsule (0.1 m **[E]**; the game's is
0.3-0.4) against the target's body segment and a head sphere (`mod.rs:163-174`). One landing per hit through a bitmask
(`landed`, `player.rs:83,397`); multi-hit attacks land each hit with its own stamina and damage. Lesson (R6 §6): the first
version used a wedge in front of the character and hit early; tune hit and hurt volumes as a pair. Damage is
`weapon attack x mv` (`mod.rs:176`): no scaling, armor or two-hand bonus (README "Not included").

**Hit-stop** (`mod.rs:180-182`, `player.rs:416-420`). A landed hit freezes both sides for `hit.stop x HIT_STOP_SCALE`; inputs
are still read into the buffer, nothing else steps. `stop` is per-hit param time **[X]**; the 0.5 scale is **[E]** ("the full
time reads as a stall", `data.rs:82-86`). A flat freeze with no ease-out; R6 §7 says SK's curve (60 ms hold, ~0.1 s ease) is
the better model.

**Root motion and rotation in attacks.** Baked root motion is applied as the per-tick delta and moved with a collision slide
(`act`, `player.rs:680-700`). `no_turn` windows stop yaw, `turn` windows set a rate, default inside an action 360 deg/s;
run 1080, sprint 480, locked 720 deg/s, all **[E]** (`data.rs:55-59`). The turn goal is the lock-on target, else the stick
(`player.rs:688-698`). Locked-on dodges keep the launch facing. No lunge scaling toward the target (R6 §5 adds it).

**Lock-on and camera.** Lock with a target within 15 m; break at 22 m, on target death or player death (`data.rs:94-95`,
`mod.rs:84-102`, **[E]**). A press with nothing to lock recentres the camera. Locked: camera yaw eases to the line to the target
at rate 8/s, pitch to -0.24 rad (`camera.rs:73-88`); distance 4.3 m, pivot 1.5 m **[E]** (`camera.rs:10-16`). One target only:
no scoring or switching. Locked locomotion and jumps strafe four ways, facing the target (README "Movement").

**AI.** Only the sparring dummy: Idle, Windup 0.9 s (tracks to 0.6 s), Strike 0.1 s, Recover 1.2 s, cycling four swings (slam,
low sweep, harder slam, knockdown) with made-up damage 120-240 and stamina 30-60 (`dummy.rs:17-66,124-203`) **[E]**. Useful
as a telegraph-and-punish test dummy, not as AI.

**Not in the reference:** parry/deflect, weapon skills, player poise, stat scaling, armor, launch, paired/stealth attacks,
enemy variety, damage numbers, camera shake, target switching. R6 covers paired moves and SK's deflect timing.

## 2. What we already have that combat plugs into

- `sf_action` (Bevy-free): `ActionClass {Light, Heavy, Offhand, Dodge, Jump, Up, Down, Guard, ...}` (`command.rs:18-47`; Guard
  fires from the buffer only while held, `:59`), `Command`, `MoveRule` data, the chooser `step`, the priority list, the two-stage
  buffer, the integer action clock. Windows implemented today: `Accept`, `Cancel`, `Tail` only (`windows.rs:16-24`). Specified
  but not built: `turn`, `no_turn`, `lunge`, `iframes`, `armor`, `kb_scale`, `hit(n)`, `deflect`, `charge` (action.md "Windows").
  The `Combat` mode profile and slices E1-E3 are open (action.md "Mode profiles", "Build plan").
- Motion owners `Clip` (baked root track on the action clock, turn-rate and lunge windows) and `Physics` (ragdoll) are what
  attacks and knockdowns use (action.md "Canonical technique"). `sf_anim::WarpPlan`, the `sf_action_bevy` fixed tick and the
  animated player body (A2 slice 2) exist; a combat clip plays through the same slot and `Progress` path as the mantle.
- Reaction machinery: `sf_physics::PhysicsEvent::Hit { target, by, impulse, point, direction }` (`sf_physics/src/body.rs:132`);
  walker knockdown at impulse >= 15 N s and cosmetic `HitReact` springs below it, get-up from a clip's first frame
  (`mw_walkers_bevy/src/physics.rs:31-36,213-272`); `sf_anim_bevy::Hit`; `mw_delirium_bevy` takeover of animated NPCs with
  hand-back by inertialization or knockout get-up, and a dev click-to-push. These respond to thrown bodies today, not weapon hits.
- Stats and items (content side, for the seam): `mw_player_bevy::hud::PlayerVitals` (static health/magicka/fatigue, no damage
  or regen, `hud.rs:64-104`); `mw_actors::held` weapon type, two-handed, `Stance` (`held.rs:118-124`); `mw_rpg` records. There is
  no `Health` or `Damage` component for combat; `Killable` in `physics.rs:165` is only a query alias for the dev kill key.
- Missing: weapon socket tracks baked per clip (R6 adopt 1), hit shape registry, per-character hurtbox, lock-on (camera.md has a
  soft-aim hook), stamina/poise state, combat clips bound to slots. No `sf_ai` yet.

## 3. Proposed architecture split

Three layers, each testable alone: **feel** (framework, game-agnostic, fixed tick), **rules** (a seam with a modern and a
classic implementation), **content** (moveset sheets and clips). Industry basis: section 7.

**Framework feel layer** (finish the `sf_action` windows, add a Bevy-free `sf_combat` and a thin `sf_combat_bevy`; crate split
to confirm):
1. Windows compiled from clip events onto the action clock: `turn`, `no_turn`, `lunge`, `iframes(mask)`, `armor(tier)`,
   `kb_scale`, `hit(n)`, `deflect`, `charge`, on the existing accept/cancel machinery. Startup/active/recovery are derived
   from hit and cancel windows (R6 §12), not authored.
2. Buffering: keep `sf_action`'s per-class accept and cancel latch (better than the reference's one slot). Dodge on release
   with a hold threshold, if the owner wants one button for sprint and roll (question 2).
3. Resources: stamina (spend at hit-window open or action start; regen after a delay; zero means cannot start; guard break at
   zero) and poise (pool per character, per-hit poise damage, per-clip armor window, break forces a reaction). The numbers come
   from the sheet and the rules seam; what happens at zero is framework behaviour.
4. Hit detection: sweeps of baked weapon-socket segments at 5 sub-steps per tick against a hurtbox set; one landing per
   (hit window, defender); multi-hit lists; a `HitEvent` out. Capsule hurtbox first, per-bone later. Host-authoritative
   (action.md "Co-op readiness"): only the host resolves hits; clients predict own starts, clocks and stamina.
5. Contact order (SK, R6 §7): typed i-frames, guard/deflect arc test, **rules seam**, poise/armor filter, reaction pick (tier x
   side), knockback, hit-stop. The reaction is a `Forced` rule at the top of the chooser's priority list; knockdown is a
   `Physics` takeover (the existing Delirium path).
6. Hit-stop as a per-character clock scale (hold ~50-70 ms, ease ~0.1 s, per side, inputs still buffered). `ActionState`
   already carries a clock rate (action.md "Character action state").
7. Lock-on and soft aim: target by stable id in `ActionState`; `turn` windows aim at it; camera follows (camera.md); scoring by
   angle and distance, switch on stick flick, break on range, death or traversal.
8. Presentation events (damage number, VFX, sound, shake) are data read by presentation, never written back.

**Rules seam** (framework trait; content implements it; ARCHITECTURE §1b "an attack connected: how much damage?"):
```
resolve_contact(attacker: &Stats, defender: &Stats, contact: &Contact) -> Outcome
Contact { move_id, hit_index, strength 0..1 (charge or wind-up), kind (slash, thrust, blunt..), weapon,
          point, direction, defender_state (guarding, down, unaware) }
Outcome { hit: bool, health, stamina, poise, other pools, reaction_hint: Option<Tier>, blocked, events }
```
plus `swing_cost(move, stats)`, `regen(stats, state)`, `can_start(move, stats)`. Modern implementation: motion value x weapon x
stat bands, no hit roll, stamina and poise from the sheet. Classic (`mw_rules`): hit chance at swing time, min/max by charge,
fatigue, armor (classic note). Both read the same `Contact` the feel layer builds, so a classic mod can keep swing-then-roll or
drop it. Stats map to bands, never to timing (action.md "Stat bands"). Random picks hash (seed, character, tick) for net-readiness.

**Data vs code.**
- Data (RON moveset sheet, patchable, keyed by slot id; action.md "How mods add moves"): chain table (next slot per class and
  state), window overrides, per-hit rows (window, socket pair and radius, motion value, stamina, poise damage, tier, hit-stop per
  side, knockback, flags unblockable/undeflectable/sweep/thrust/grab), defence windows, costs, charge window, lunge and turn
  windows, exit speed, roll type by load; reactions and i-frames per slot; bake-time reach, first contact, recovery.
- Code: chooser and buffer, window evaluation, hit sweeps, contact order, resource behaviours, hit-stop clock, lock-on,
  forced-reaction rules, the `Rules` trait. Which moveset a weapon uses lives in the content crate.
- Authoring cost: free packs carry no events, so windows are annotated once per clip (`clip.annotate` plus a sidecar).

## 4. First slice and the following

**C1, the smallest thing the owner can feel.** One weapon, one sheet. Player: a 3-hit light chain (accept/cancel windows, root
motion, turn-rate and no-turn windows), roll on Dodge with i-frames and a stamina cost, stamina bar with regen delay, sprint.
One dummy (capsule hurtbox) that reacts: hit-stop on both, additive flinch, health, a poise break into a stagger; it does not hit
back. Lock-on on it with four-way strafe and camera. Scripted-input tests (R6 adopt 7): fires exactly at the cancel frame, early
presses lost, one landing per hit, i-frames leave the window unspent, zero stamina refuses. A stub `Rules` holding the modern
formula. Owner visual check (new CB-1) in a new test level. Clips: three sword swings, one roll, one hit reaction (all in UAL,
section 5), plus the socket-track bake.

**C2, the dummy hits back, plus guard.** Telegraph/strike/recover dummy (the reference's shape); player damage and tiered,
directional reactions; guard with arc and stamina cost; guard break; deflect window; guard counter; backstep; knockdown through
the Delirium Physics path. Unit tests on the contact order.
**C3, heavy and openers.** Heavy with charge window; running, rolling, backstep, jump attacks; armor windows (hyper-armor as
data); lunge toward the target; typed i-frames; a second weapon class to prove the data layout.
**C4, targeting and camera.** Scoring and switching; soft aim without lock; combat camera events; shake.
**C5, rules seam proper.** `mw_rules` classic implementation behind the trait (game side); stat-band mapping.
**C6, NPC combat.** The same moves driven by `Command` from `sf_ai` (M7); paired takedown with a ragdoll victim (E3).
**C7, net.** Host-authoritative hits and own-start prediction (co-op phase F).

## 5. Clip needs

Clip names read from the packs' glTF on 2026-10-27. UAL1 = `AnimationLibrary_Godot.glb` (CC0), UAL2 = `UAL2_Standard.glb`
(CC0), LAFAN1 BYO and private only. `bevyrrowind/crates/mw_graphs/data/animation/sources.ron` declares only `traverse.*` slots
so far; no combat slot exists.

| Need | UAL1 | UAL2 | Verdict |
|---|---|---|---|
| Light chain (3+) | `Sword_Attack`, `Sword_Attack_RM`, `Sword_Attack_Standing` | `Sword_Regular_A/B/C`, `_A_Rec`, `_B_Rec`, `Sword_Regular_Combo` | 3-hit chain OK; windows must be annotated |
| Stance, draw | `Sword_Idle`, `Sword_Enter`, `Sword_Exit` | `Idle_Shield_Loop` | OK |
| Roll, dodge | `Roll`, `Roll_RM`, `Dodge_Left/Right` (+`_RM`), `BackFlip` | `Sword_Dash_RM`, `Shield_Dash_RM` | forward and side OK; back only `BackFlip` (acrobatic); no equip-load variants |
| Hit reactions | `Hit_Chest`, `Hit_Head`, `Hit_Shoulder_L/R`, `Hit_Stomach` | `Hit_Knockback`, `_RM` | flinch OK; knockdown start OK; stagger and large stagger missing |
| Death, get-up | `Death01/02` | none | knockdown ground clips and get-up missing (LAFAN1 falls and get-ups, BYO) |
| Guard | none | `Sword_Block`, `Idle_Shield_Loop`, `Idle_Shield_Break`, `Shield_OneShot` | guard idle and break plausible; guard-hit recoil missing |
| Heavy, charged | none | `Melee_Hook`, `_Rec`, `Zombie_Scratch` | only an unarmed hook; overhead two-hander and charge loop missing |
| Running, rolling, backstep, jump attack | none | none | missing (`Slide_*`, `Sword_Dash_RM` could seed a running attack) |
| Parry, riposte, takedown | none | none | missing; LAFAN1 fight takes are unarmed |
| Unarmed | `Punch_Jab`, `Punch_Cross`, `Kick`, `PunchKick_*` | | enough for a fist moveset |

The free packs cover C1 and most of C2. Heavy, running/rolling/jump attacks, parry and stagger tiers are gaps for C3 (options:
UAL Pro, another purchased melee pack, Mixamo for prototyping only, retargeted LAFAN1 takes: question 4). The content pack's own
animation file (attack groups with Min Attack / Max Attack / Hit / Follow text keys, block, hit, knockdown/knockout) is a full
classic-look set but swing-only, with no roll or dodge, so a modern feel needs pack clips either way; the text-key kinds are
already parsed (`bevyrrowind/crates/mw_anim/src/text_keys.rs:32-53`). Retargeting UAL clips onto the game rig is what A2 slice 2
already does; root motion comes from `_RM` tracks or the pelvis extractor.

## 6. Open questions for the owner

1. Player poise: Elden Ring has it (the reference omits it). Poise and hyper-armor from C3, or "every unguarded hit staggers"
   for the first cuts?
2. Dodge and sprint on one button (tap = roll on release, hold = sprint, as in the reference) or two? The roll then fires on
   release, which adds input latency.
3. Parry/deflect: in scope for the first cut, or guard only? It needs clips we do not have.
4. Clips for heavy/charged, stagger tiers, running and jump attacks: buy a pack, prototype with Mixamo, retarget LAFAN1, or
   reuse the content pack's own attack clips for these?
5. First moveset sheet: one sword only, or a sword plus a second class to prove the data layout?
6. Creatures (full-body attack clips, non-humanoid rigs) and NPCs: same framework with per-rig sheets, deferred to C6?

## 7. Industry practice

Souls-likes make fights a commitment economy: every action spends stamina and holds the character in a clip until a cancel
window opens (FromSoftware's per-clip event windows, cancel earliest for dodge and latest for move: R6 §2, §12). Input is
buffered only after an accept frame so mashing is lost, i-frames are clip data and the dodge costs stamina, and hits come from
weapon volumes swept against hurt volumes with hit-stop to sell contact (Sakurai on hit-stop:
sourcegaming.info/2015/11/11/thoughts-on-hitstop-sakurais-famitsu-column-vol-490-1/; the GDC talks catalogued in R8, e.g.
gdcvault.com/play/1026423 on God of War combat). Modern action RPGs keep the feel layer (state machine, windows, buffer, hit
detection, hit-stop, camera) apart from the rules layer (damage formulas, stats, effects): Unreal's Gameplay Ability System
separates abilities, effects and attributes from animation notifies
(dev.epicgames.com/documentation/en-us/unreal-engine/gameplay-ability-system-for-unreal-engine), and the reference here leaves
damage as `attack x motion value` behind its hit event. That is the seam in ARCHITECTURE §1b: a classic formula is another
implementation of the contact resolver. R8 (`../animation/08-industry-combat-responsiveness.md`) holds the responsiveness
numbers (buffer, latency, hit-stop) the spec should adopt.
