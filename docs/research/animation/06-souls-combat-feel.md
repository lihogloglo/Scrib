# R6: souls-like combat feel (ELDEN-RING-Combat-Rewrite, sekiro-rs combat)

Status: done (r6, 2026-10-18).

Sources:
- **ER** = https://github.com/Funny-Bones/ELDEN-RING-Combat-Rewrite (MIT, Bevy 0.19), read in full at
  `177a3b5` (2026-10-09) with its history: `src/sim/{data,player,dummy,mod,tests}.rs`, `rig.rs`, `camera.rs`.
- **SK** = sekiro-rs (Apache-2.0), local clone at `2ad2539` (2026-10-18): `crates/sim/src/combat/*`,
  `hits.rs`, `input.rs`, `character.rs`, `brain/moves.rs`, its combat, deathblow and projectile-guard docs, and
  the `combat_rules`, `combat_polish` and `deathblow_camera` tests.

**Clean room.** Both projects read the commercial games' files, and SK also reads the executable. This note
gives mechanisms and rounded ranges only: no game identifiers, event or param names, formulas, or tables.
SK's hit-filter doc covers map collision groups, not combat.

## 1. Shape: a fixed-tick sim owns time, animation follows

Both run combat as a **fixed 60 Hz simulation with no engine types**. In ER, the sim holds the current action
and its frame, and the rig only looks up that pose. Gameplay never reads the skeleton. Everything gameplay
needs is baked beside the clip: duration, cumulative root motion, the weapon's striking line per frame in
character space, and the windows. SK steps in this order: buffered input → requests, gated by the
*previous* step's clip windows → script and graph pick transitions → this step's clip events → body (root
motion, steering at the clip's turn rate, gravity).

| Thing | Owner (same in both) |
|---|---|
| Startup / active / recovery | **Not authored**: they fall out of the hit and cancel windows |
| Hit windows, i-frames, per-input cancel windows, input-accept start, turn windows, armor, charge, regen, camera, sounds | **Clip event track** (so per weapon moveset) |
| Damage, posture/stamina damage, hit-stop, knockback, reaction tier, guard flags | **Params**, per attack row named by a hit window |
| Which clip follows which, priorities | **Code/script** |
| Default turn rates, guard raise time, buffer rules, hit-stop curve | **Engine constants** |

SK states it outright: the behaviour script only picks animations. The engine classifies each hit before
the script sees it.

## 2. Frame structure of an attack

ER's per-clip windows:
- **Input-accept start**, with a separate one for dodge. Presses before it are *dropped*, not queued, so
  mashing early does nothing (tested).
- **Earliest cancel per input kind**: light, heavy, off-hand, dodge, jump, guard, move. Dodge opens first,
  move last. That late move-out is where weight comes from.
- **Hits**, as a list: each has its own window, damage, stamina cost, hit-stop and capsule radius. Stamina is
  charged **when each hit comes out**, so a cancelled swing costs less.
- **Invulnerability**: a medium roll is about 0.4 s invulnerable out of about 1 s, and travels 3.3 to 4.3 m by
  equip load. A knockdown is invulnerable for about 2 s, and a roll-out opens after about 1.2 s.
- **No-turn and turn-rate windows.** The default inside an action is a few hundred deg/s. Free running turns
  about 3× faster and locked-on strafing about 2× (ER's estimates).
- **Charge window**: releasing inside it swaps to the uncharged clip; holding past it commits.
- Cancel windows can also be **state-conditional**; both projects use only the unconditional ones.

SK adds:
- **Typed i-frames** (general, sweep, thrust, grab). Jumps and the first ~0.27 s of a step let a sweep or a
  grab pass. A blow that meets invulnerability is **not spent**: it can still hit after the i-frames end.
- **Armor windows that clamp the incoming reaction tier** (everything becomes minimum, or only large hits or
  knock-offs get through), plus a no-reaction flag. HP and posture still apply. This is hyper-armor as a
  *reaction filter*, not a poise pool.
- **Regen-percent windows** (0% in attacks and reactions, partial while guarding), and windows that **pin**
  the meter.
- A **fixed-rotation window** on enemy active frames: tracking stops, so a side-step escapes a committed
  swing.
- A **knockback-received scale** on the defender's own clip (0 to ~50% during its combo frames), so combos
  stay in range.

## 3. Input buffering

**ER: one slot, newest wins.** Each press of light, heavy, off-hand or jump overwrites the slot while the clip
listens. Dodge fires on the *release* of a tap shorter than ~0.3 s; held longer, it sprints. The slot has
**no expiry**. It fires on the exact cancel frame for its kind (tested), or at once if the clip ends first.
Presses during hit-stop still register. Guard is not buffered: it is a held state that takes over from its
cancel frame.

**SK: one latched request per action.** Each action has **accept** windows (a press is latched only inside
one) and **execute** windows (a latched request is reported only inside one). There is no timer, and starting
*any* action clears them all. A buffered guard fires only if the button is still held. Unflagged clips
(idle, locomotion) accept and execute everything. When several requests are pending, the **script's fixed
candidate order** wins: reactions first, then item, jump, step, art, attack, guard, movement. A per-stance
mask decides which candidates exist.

In both, the clip decides when input counts. Pressing early still fires on the exact cancel frame, which
feels responsive; early mashing is lost and move-out opens last, which keeps attacks weighty.

## 4. Combos (ER, code side)

Light presses walk the weapon's chain (3 to 6 deep), then restart. **The chain resets once the clip is
left**; there is no combo timer. **Openers** (run, roll, backstep, crouch attacks) continue into the *second*
swing (`4438ecb` fixed a restart). A heavy within ~0.6 s of a block is a **guard counter**. Grip changes play
on an upper-body layer while the legs keep moving.

## 5. Root motion, tracking, lock-on

- **Root motion drives every action**, slid against the world. Jumps follow the authored arc, then gravity.
  A move-cancel keeps a per-action **exit speed**.
- **Tracking is rotation, separate from translation.** The gameplay yaw turns toward the target or the stick
  at the clip's turn rate, except in no-turn windows. ER's dummy stops tracking ~0.3 s before its strike,
  and that gap is what makes a late roll work.
- **Lunge is translation (SK).** Inside a window, root translation is **scaled** toward the target and
  clamped (roughly 0.5× to 1.5×). One slash travels ~1.2 m against a close enemy and ~3 m against one 4 m
  away. With no target, the scale is 1. It deliberately falls short of a far target.
- **Auto-aim when not locked (SK).** In the first ~1/6 s of an attack, the nearest enemy within ~3 m and ±30°
  of the stick becomes the target.
- **Lock-on movement.** The character faces the target and strafes, with slower back and side speeds. Rolls
  and jumps pick one of four directional clips and **square up**: they rotate so that direction points
  exactly where the stick asked, and keep that facing. SK twists the legs toward travel and the spine back,
  so the chest stays on the target with a single clip.
- **Lock-on camera (ER)**: behind the character on the line to the target, eased, at a fixed pitch. It breaks
  at a longer range than it acquires; with no target in range, a lock press recenters.

## 6. Hit detection

- **ER**: the baked weapon line is tested at **5 sub-steps per tick** (a fast tip moves ~0.3 m per tick) as
  a capsule, against the body capsule plus a head sphere. A **bitmask per hit** allows one landing per hit
  per swing, and multi-hit attacks land every hit.
- **SK**: a capsule between two weapon sockets is swept from the previous pose to this one. **Each window
  hits a defender once.** Armoured body parts guard whatever the defender is doing.
- **Failure (ER `88714a9`)**: the first hitbox was a wedge in front of the character, so hits landed before
  the weapon arrived and thrusts hit beside the target. The game's own capsule radius is sized against slim
  per-bone hurt volumes. Against a whole-body capsule it hit early, so ER uses a thin one. **Tune hit and
  hurt volumes as a pair.**

## 7. Resolution, hit-stop, reactions, knockback

**One contact, in order (SK).** First the guard test: the defender must be guarding, and an **arc test**
(half-angle 60 to 90°) gates deflects too, so a deflect while facing away is a hit. Then block or deflect,
posture and HP, break checks, knockback, and the reaction choice. The attacker gets its own result (blocked,
deflected, or hard-deflected), which picks its recoil clip.

**Hit-stop.** SK holds the character's *whole clock* (animation, script, events, root motion, pushes) at
zero for ~60 ms. It then eases back quadratically over the attack's length (~0.1 s for the player's sword),
costing ~0.13 s in total. Blocks and deflects get hit-stop too, and a new one replaces the running one. Each
side has its own length; most enemies take none. ER froze both sides for the whole listed time, found it
"reads as a stall", and halved it (`7474dc5`). SK's curve explains why: most of the listed time is ease-out,
not a freeze.

**Reactions.** ER uses four tiers (flinch, stagger, large stagger, knockdown) × four directions, chosen by
the side the hit came from. Its knockdown throws the character ~4 m back by root motion, invulnerable. ER's
player has no poise, and nothing cancels a stagger early. SK has about ten levels, and **minimum-level guard
hits play additively without leaving the current state**.

**Knockback (SK).** A distance per outcome (hit, block, deflect), cut by the defender's resistance and its
clip's scale window. Speed holds, then falls off linearly; collision stops it. It is a separate push from
root motion.

**Posture (SK).** Posture is stored as what remains. **A deflect never breaks the deflector**, though a later
block or hit can. Recovery slows at low HP and is set per clip.

## 8. Deflect timing and spam punishment

SK has one guard stance. Each press opens a **deflect window** early in the guard clip by raising the
guard's defence above the attack's rating; outside the window the same contact is a block. Attacks rated
above the raised defence are never deflectable. **Rapid re-presses step through guard variants whose windows
shrink: ~200, ~100, ~70 ms, then none.** That punishes mashing, and it is clip data, not a cooldown.

Perilous attacks are guard flags: sweeps (jump them), thrusts (deflect, or a counter-step with a ~0.4 s
window), grabs (dodge). The warning shows 0 to ~0.7 s before the blow. ER's guard needs ~0.13 s to come up
before it blocks.

## 9. Deathblows and stealth kills: paired animation

SK; ripostes, backstabs and enemy grabs all use the same machinery.
1. **Start check.** The defender is broken (or unaware, for stealth). Distance ~1 to 3 m, with a separate near
   variant, and a height band. The attacker is in the front half-angle around the defender (or the back half,
   for stealth), facing within ~35°. The attacker then plays a short **lunge** that turns it to the defender.
2. **Grab check.** At an event in the lunge, looser limits are checked. On a pass, both start **paired
   clips**; otherwise the lunge whiffs.
3. **Alignment.** One character is moved onto a socket in the other's *root* frame: ~1.2 m ahead facing back
   for a front kill, ~0.5 m ahead facing away for a back kill. It happens **once, at the paired clips' first
   frame, eased over ~0.5 s**. After that, each character plays its own root motion.
4. **Branch.** The defender's clip has a branch frame. A lethal blow cuts there to a death clip whose first
   pose is identical. A non-lethal blow plays on into a get-up.
5. **Early release.** The attacker's move cancel opens before the defender's clip ends.
6. **Camera.** Clip events set a camera profile, a distance curve (closing to ~1.5 to 2 m) and a look-at
   on an *animated camera bone*, within limits. The hand-back eases over ~1 to 2 s.

**Failures and fixes (SK).**
- Holding the defender on the attacker's *live* socket made it spin along and pass through walls. Fix: snap
  once, then let each clip's own root motion run.
- An unset variant selector paired a 2 s attacker clip with a 6.6 s defender clip.
- Leftover knockback resumed after the pair.
- A pair queued during hit-stop lost to the victim's stagger. It now waits until the victim has reacted.
- **Walls**: both capsules are pre-swept along the predicted root tracks, the whole pair is shifted until it
  fits, and wall pushes are bypassed while paired.

## 10. Other failures, tests, AI

- **ER `0c3de5f`**: a wrongly paired settle clip made the hand jump 56 cm, and cutting a full-weight layer at
  its last frame snapped 30 to 55 cm. Fix: chain clips whose poses match at the seam, and fade layers out
  across the settle.
- **ER `fec0975`**: a headless self-playing demo, driven through the normal input struct, is a test.
- **SK move sheet**: the AI reads the same clips. Active windows, reach envelope (capsules sampled over root
  travel), first contact and recovery let it pick punishes that land inside the player's recovery.

## 11. Situations

**Sword fight**: §2 to §8, with readable wind-ups (lock-on, third-person camera). **Stealth kill**: §9 with
an unaware defender, the back angle band and the near socket; perception is AI, and neither project has it.
**First person**: neither project has it. Roofs, climbing, sitting and doors: not covered.

## 12. For us

Mapped to `animation.md` (Clip `events`, Slots and montages, Layer 1, Layer 4), WISHLIST §9 and the planned
`docs/specs/combat/` (which doesn't exist yet).

**Adopt**
1. **Combat on our fixed tick, with the clip clock authoritative.** Gameplay reads windows, baked root motion
   and **baked hand/weapon-socket tracks** (root space, over hit windows, at `anim.cook`), never the
   evaluated pose. The socket track is composed with each weapon's striking segment at runtime. This stays
   deterministic and survives anim LOD and culling.
2. **Typed combat windows compiled from `Event`s**, like `parse_warp_window`. Today the event vocabulary is
   undefined.
3. **Buffer**: SK's model. One request per action kind, an accept start and an execute window from the clip,
   no timer, cleared on any action start, and a fixed priority list. Dodge fires on release; guard only if
   still held.
4. **Hit-stop as a per-character clock scale**: hold ~50 to 70 ms, then ease out over ~0.1 s, scaling
   animation, events, root motion, knockback and timers together, with input still buffered. The spec has
   `time_scale` but no curve and no whole-character rule.
5. **Armor as a reaction-tier filter window, plus a knockback-received scale window.** Hyper-armor as data
   only.
6. **Paired-action spec**, from §9: start check → lunge → grab check → one-shot Layer 1 warp onto the
   partner's root-frame socket (~0.5 s) → own root motion → lethal branch onto a pose-identical death clip →
   space reservation and pair collision bypass → camera events. It serves backstabs, ripostes, stealth kills
   and grabs, and two-person interactions outside combat.
7. **Bake-time move metadata** (reach, first contact, recovery) for AI and tests, plus ER-style
   scripted-input tests: fires exactly at the cancel frame, early presses are lost, one landing per hit,
   i-frames leave the window unspent.

**Adapt**
- **Layer 1 is wrong for attacks as written.** Souls attacks split *rotation tracking* (a turn-rate limit
  with no-turn windows, not a warp) from a *clamped scale lunge* with a stand-off that deliberately falls
  short. Add `mode=scale` with min/max clamps and a `standoff`, and exempt clamped windows from the ≤ 2 cm
  check. The target is the lock-on target, else auto-aim (~30° cone, ~3 m, acquired only early in the
  attack).
- **Layer 4 `HitReact`** stays cosmetic. The gameplay reaction is a tiered, directional montage with
  windows. The minimum tier is additive (SK). Knockback is its own mover push, not root motion.
- **Montage sections are not needed.** One clip per combo step, plus a chain table in the moveset. Cancels
  need a short per-request blend-in (~0.1 s, against the 0.2 s default).
- **Morrowind clips** carry only charge, hit and follow keys (actors-adapter). The adapter synthesizes
  defaults: a hit window around the hit key, dodge cancel from follow start, move cancel at follow stop.
  Mods can override them.
- **First person**: windows and hit sweeps always come from the third-person clip on the shared action clock.
  First-person arms are presentation only (R2). Lock-on becomes soft aim, and deathblow cameras use R2's
  camera bone.

**Reject**: parsing game formats (rule 3); ER's single-slot buffer (a dodge can't wait behind an attack
press); full-length hit-stop freezes.

**Where our docs are wrong, thin or missing**
- *Wrong*: WISHLIST §9 lists "startup, active, recovery" as authored phases. They should be derived from hit
  and cancel windows. The roadmap's "root rotation tracks the target" during wind-ups is a gameplay-yaw
  turn rate, not warping.
- *Thin*: §9's "hitstop" and "poise" have no model behind them. Also absent: buffer rules, reaction tiers,
  knockback, deflect-window shrink, typed i-frames.
- *Missing*: paired animation, combat camera events, lock-on strafing (four-way dodges, square-up, leg/spine
  twist), the AI move sheet, and `docs/specs/combat/` itself. Proposed files: `windows.md`, `input.md`,
  `hits.md` (detection, resolution order, hit-stop, knockback), `reactions.md` (tiers, armor, posture),
  `paired.md`, `lockon-camera.md`.

**Minimal attack data model** (one clip plus):
- **Identity**: clip, moveset tag, kind. The next step comes from the moveset's chain table, not the clip.
- **Root**: drive the clip's root motion. Optional lunge window (from, to, min/max scale, stand-off). Exit
  speed on a move-cancel.
- **Turning**: no-turn windows, turn-rate windows, auto-aim acquire window.
- **Input**: accept start per input kind.
- **Cancels**: earliest time per input kind (attack, heavy, dodge, jump, guard, move, item), optionally
  state-gated.
- **Hits** (a list), each with:
  - a window;
  - a shape (weapon segment or socket pair, plus radius);
  - damage and posture multipliers, and a stamina cost charged at window start;
  - a reaction tier, knockback per outcome, and hit-stop per side;
  - flags: unblockable, undeflectable, sweep, thrust, grab.
- **Defence**: invulnerability windows with a type mask, armor windows (the highest tier let through),
  knockback-received scale, regen-percent windows, and the deflect window on guard clips.
- **Charge**: a window plus a release clip.
- **Presentation**: sound, VFX, camera and warning events.
- **Baked**: reach envelope, first-contact time, recovery time.
