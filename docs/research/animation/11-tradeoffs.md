# R11a: trade-offs for the owner brainstorm (2026-10-19)

Status: **decided** (owner brainstorm 2026-10-19, "Decisions" below). Each section ends in a
default recommendation; the decisions list overrides them. R11b writes the spec from it.
Sources are notes R1-R10 in this folder, cited as "R3 §7". Crates named are what exists today (`sf_anim`, `sf_anim_graph`,
`sf_controller`, `sf_ragdoll`, `sf_character`, `sf_bake`) plus what is only planned:
`sf_interaction` (M4), `sf_ai` (later), a camera spec and a combat spec (both missing).

The owner's situations: sneak behind a guard, roof-to-roof jump, ledge grab / mantle / climb,
sword fight, sit at a bar, door or lever, first and third person.

## Decisions (owner brainstorm, 2026-10-19)

Binding for R11b. Where a decision differs from a section's default below, the decision wins.

- **Primary view: third person.** One gameplay body decides hits in both views; first person
  must be playable and comfortable, traversal there is checked per move, not hand-polished (§5).
- **Multiplayer: yes, small co-op (2-4), host-authoritative with client prediction; design now,
  build later.** No lockstep or bit-exact determinism. R11b is written net-ready: fixed tick,
  input as commands, presentation (camera, first-person bodies, Morrowind arms clips) never
  feeds gameplay, a remote player is just another character. Own research slice and milestone
  after M4-M6 (§11; changes §1, §2, §5 accordingly).
- **Climbing: ledges first** (grab, hang, shimmy, mantle, ladders with IK); hold index baked so
  wall climbing can follow (§12).
- **Jump at a waist-high wall mantles in normal play, with forward input.** Sneak never triggers
  traversal; separate up/down intents; assisted/manual setting (§3).
- **Free-run = hold sprint** (§8).
- **Missed heavy attack: souls-level.** Full recovery, no cancel (§2).
- **Low Acrobatics: jump lower, all moves available.** Stats scale magnitudes in bands, never
  timing or latency (§10).
- **Morrowind first-person clips: optional presentation**, an arms layer on the shared clock;
  default first-person arms come from the shared body (§5).
- **NPCs: same move set as the player; traversal AI later** with `sf_ai` (navmesh until then) (§1).
- **Clip gaps: packs only, free preferred, purchased if needed. No owner mocap** (§7). First
  takedown uses a ragdoll victim.
- **Defaults taken (no owner objection):** §1 motion owner per move kind, no steering mid-mantle;
  §2 traversal abortable only in its tail; §4 blending; §6 free camera, soft aim (no hard lock-on)
  in first person, optional lock-on in third, framed camera allowed for paired kills; §7 bones
  only + one moveset line per attack slot; §8 one scale with mode profiles, crowd-scaled hit-stop;
  §9 one priority list, traversal breaks lock-on, a drawn weapon doesn't block traversal,
  drop/air kills after the first combat milestone; §11 data move rules in a Rust action layer.

---

## 1. Who owns motion

**Conflict.** In a roof-to-roof jump the body must go where momentum and the stick say. In a
mantle or at the bar stool, hands and hips must land exactly on the lip or seat. In a sword
swing, the clip's step forward *is* the weight. One rule can't serve all three.

**Options.**
- **A. The sim drives, the clip follows** (driven playhead). faith-runner (R1 §4, R2 §4); Dying
  Light and Spider-Man (R7 §3). Cost: a driven-playhead clip node in `sf_anim_graph`, clip
  calibration at bake. Breaks: feet slide and hands miss lips unless IK cleans up; faith-runner
  has no ledge IK (R1 §10). Heavy attacks feel floaty.
- **B. The clip's root motion drives the body.** Souls (R5 §2, R6 §5). Cost: every clip needs
  root motion; the bake reconstructs it for in-place clips (R9 §2). Breaks: locomotion feels
  sluggish (Holden, R8 §1.2); mixed-source clips have different step lengths.
- **C. Root motion warped onto a target.** AC1-rs (R3 §1, §5); UE motion warping and GASP
  (R7 §3). Cost: our Layer 1 is single-clip; it needs warps across clip chains, correction
  profiles and a refusal ratio (R3 §11). Breaks: free momentum; GASP reports jitter on release.
- **D. Label regions of each clip with who drives**, and correct the character toward the sim
  with a damped, speed-limited, clamped spring. Holden (R8 §1.2).

**Reconciling design.** A per-move-kind table, which is D applied to moves:
- locomotion, air, sneak: sim owns; the clip catches up within clamps (R8 adopt 2);
- traversal contact moves (vault, mantle, ledge grab, sit, door, lever): warped root motion,
  with the playhead driven by the planned path where the sim plans it (R2 adopt 1, R3 adopt 1);
- attacks, dodges, paired kills: clip root motion, with turn-rate and lunge-scale windows as
  code's say (R6 adapt).

Price: a "motion owner" field in each move rule (R1 adopt 7), a fourth root mode beside
Drive/Match/Ignore ("driven by progress", R2 spec gaps), chain warping in `sf_anim_graph`,
correction springs in `sf_controller`. Moderate; mostly extends what exists.

**Recommendation (default).** Per move kind, as above. Industry reads the same way (R7 §3,
R8 §4).

**Owner questions.**
1. During a mantle or vault, is it fine that you can't steer for about half a second?
2. Should NPCs use the same traversal moves (chases over roofs), or only the player for now?

---

## 2. Commitment vs responsiveness

**Conflict.** A sword fight wants weight: a whiffed heavy should be punishable. A roof run
wants flow, never a "traffic jam" of locked moves (Dying Light, R7 §3). Sneaking wants an
instant stop. A late ledge grab wants forgiveness.

**Options.**
- **A. Souls commitment.** Clip windows gate every cancel; move-out opens last (R6 §2-3).
  Cost: window data per clip, via the moveset sheet (R9 §6). Breaks: traversal feels sticky.
- **B. Free cancels and momentum.** faith-runner (R1 §5); Ghost of Tsushima lets heavies be
  cancelled at any point (R8 §1.3). Breaks: fights lose readability and punishment.
- **C. Uninterruptible, but chosen for you.** AC (R3 §7 gates). Breaks: control; the
  traffic jam.

**Reconciling design.** Split **first response** from **commitment** (R8 §4):
- every press shows something (pose change, sound, camera kick) within 1-3 fixed ticks
  (R8 adopt 1);
- commitment is per-move data in cancel windows, with one window vocabulary for all moves
  (R5 adopt 1);
- the player is faster than enemies; an enemy's first blow stays slower than human reaction
  time (R8 §1.3);
- forgiveness everywhere: jump buffer, coyote time, two-stage input buffer (R1 adopt 3,
  R6 adopt 3).

Price: window-gated interruption in `sf_anim_graph` (R5 spec gaps), a latency test, category
default windows (R9 §6). Low to moderate.

**Recommendation (default).** A spectrum by move kind. Locomotion and sneak: fully responsive.
Traversal: committed until the last contact, then a cancellable tail. Light attacks: medium.
Heavy attacks and paired kills: committed.

**Owner questions.**
1. How punishing is a whiffed heavy: souls-level, or cancellable into block (Ghost of Tsushima)?
2. Can a jump or vault be aborted once started (drop back down)?

---

## 3. Context moves vs explicit buttons (and stealth vs traversal input)

**Conflict.** An assassin running over roofs wants one button that does the right thing. A
player sneaking behind a guard must never vault onto a table or drop off a roof. Morrowind
players expect jump to be jump.

**Options.**
- **A. One context button.** Mirror's Edge (jump = vault, R2); GASP's jump that becomes a
  vault; AC's free run while a button is held (R3 §7). Breaks: accidental parkour.
- **B. Explicit buttons; the engine only searches.** Sekiro (R5 §1, §3). Breaks: flow; many
  buttons on a gamepad.
- **C. Intent split.** AC Shadows: *up* (loud) and *down* (quiet, safe) are separate; automatic
  passovers only while sprinting; stance input never triggers a drop (R7 §3).
- **D. Profiles.** AC1's low / high / free-run (R3 §7), mapped to sneak / normal / assassin
  (R3 §11).

Cost of C+D: input mapping in the action layer; probe gating by speed and time to contact
(R1 adopt 4); parkour noise events for the future `sf_ai` (R7 §5). Low.

**Reconciling design.** C inside D. Jump at an obstacle becomes a vault or mantle (context
within an explicit intent). Automatic passovers only while sprinting. Sneak never triggers
traversal. A comfort setting, "assisted or manual parkour" (Dying Light 2, R7 §2.2).

**Recommendation (default).** C with D's profiles and an assisted/manual setting.

**Owner questions.**
1. In vanilla play, may jump at a waist-high wall become a mantle, or should jump stay a pure
   jump unless free-run is on?
2. Do we spend a button on a separate "drop / go down" intent?

---

## 4. Blending

**Conflict.** Snappy vs smooth. Cancels into a dodge want speed; a run into a vault wants no
seam.

**Options.**
- **A. Instant switch.** sekiro-rs (R5 §7, rejected there); fighting games. No evidence it is
  preferred elsewhere (R7 §3).
- **B. Crossfade from a frozen pose, length sized by the largest hand or foot gap.** AC1-rs
  (R4 §1).
- **C. Inertialization** (ours, built).
- **D. C plus a pose-matched entry frame** for traversal clips. GASP (R7 §3); R4 adopt 2.

**Reconciling design.** D, with gap-sized durations (R4 adopt 1), a short blend-in for combat
cancels (about 0.1 s, R6 adapt), and root impulses on a clip's first frame (R8 §1.1). Price:
small, all in `sf_anim_graph`.

**Recommendation (default).** D. The least contested item; no owner question.

---

## 5. First person

**Conflict.** First person needs hands on the lip, legs underfoot when you look down, a sword
that reads at arm's length, and a calm camera. Our clips are third-person and carry no camera
joint. Sitting at a bar and stealth kills have no published first-person practice (R7 §4, R8 §5).

**Options.**
- **A. Full body, camera authored in every clip.** Mirror's Edge (R2 §5, R7 §2.1). For us:
  bake a virtual camera per clip (R2 §11, R9 §2). Breaks: unproven on third-person mocap; a raw
  head camera causes nausea (ME1). Needs review per move.
- **B. Separate first-person arms rig with its own timing.** Skyrim, Morrowind (R7 §2.1).
  Breaks: the two clip sets drift; no legs or ledge hands; what you see isn't what hits
  (R8 §4).
- **C. One gameplay body, two render bodies.** UE 5.5: first-person arms and legs with their
  own FOV and scale, plus a world body for shadows (R7 §2.1, R8 §3). Morrowind's first-person
  clips survive as presentation on the shared clock (R6 adapt).
- **D. First person only, with third-person cuts** (Cyberpunk; Deus Ex takedowns, R8 §5).
  Previews found the cuts jarring.

**Reconciling design.** C, with the body chosen per action: arms layer for weapons and spells
(Morrowind clips allowed as presentation), full body with the baked camera for traversal,
sitting and paired kills. A third-person cut stays an option per paired kill. Price: **high**.
A camera spec (missing), a rewrite of character.md's first person, a camera track from
`clip.annotate`, render layers in Bevy (Bevy lacks a shadow-only flag; character.md open
question 1), per-move review in visual checks.

**Recommendation (default).** C. One body is the truth in both views.

**Owner questions.**
1. Which view is primary, where most tuning goes: first or third?
2. Must Morrowind's own first-person weapon clips survive, or may first-person arms come from
   the third-person clips?
3. Stealth kill in first person: stay in first person, or cut to third?

---

## 6. Camera

**Conflict.** A sword fight wants lock-on and framing. A roof jump wants free look. Hanging
from a ledge needs look-down limits. In first person, the camera *is* the sword's aim.

**Options.**
- **A. Free look always.** Mordhau, Chivalry (R8 §4); Dying Light releases the camera during
  moves (R7 §3). Breaks: drag swings, spinning, poor readability.
- **B. Lock-on plus camera presets from clips.** Souls (R5 §4, R6 §5, §9).
- **C. Per-move look limits, eased in.** faith-runner (R2 §6).
- **D. Soft aim when unlocked.** God of War auto-orients attacks (R8 §1.3); Sekiro picks a
  target early in a swing (R6 §5).

**Reconciling design.** Free by default; C in first person; optional hard lock-on in combat,
soft aim when not locked; clip presets only for paired kills and similar framed moments;
comfort switches (shake, bob, auto-camera off, FOV; R8 §1.4). Price: a new camera spec and its
code (nothing exists; bevyrrowind has a free fly camera, R2 §11). Moderate.

**Recommendation (default).** As above.

**Owner questions.**
1. In first person: hard lock-on (KCD) or soft aim only?
2. In third person, may a move take the camera (takedown framing), or never?

---

## 7. What clips must carry

**Conflict.** Fights need design windows on every clip; bring-your-own clips from any source
carry nothing but bones. How much can we demand?

**Options.**
- **A. Rich per-clip tracks.** Souls (R5 §1, R6 §1). Breaks: no modder will author them.
- **B. Bones and root only.** AC1-rs (R3 §9). Breaks: no design windows.
- **C. Timing in code by clip name.** faith-runner (R2 §9). Rejected (R2).
- **D. Bones only, then three layers:** inferred facts, category defaults from the slot, and a
  moveset sheet for design numbers (R9 §6).

**Reconciling design.** D. The bake measures everything spatial: root, contacts (feet and
hands), end pose, warp bounds, weapon socket track, camera track. Humans author only design
windows (accept, cancel, i-frames, deflect) and confirm hit windows: about one line per attack
slot. Price: a `clip.annotate` bake step, a moveset sidecar, a review scene (R9 §6). Moderate.
What it doesn't fix: style seams between sources (R9 §5) and traversal coverage (perhaps 20-30
of AC1's 100+ moves, R9 §4).

**Recommendation (default).** D.

**Owner questions.**
1. Is about one line of numbers per attack slot acceptable for modders?
2. For gaps (vaults, top-outs, paired kills), is owner video mocap acceptable, with a ragdoll
   victim as the first takedown?

---

## 8. Pace and scale

**Conflict.** Roof-to-roof free-running wants arcade speeds and long jumps. Souls fights and
RPG weight want realistic timing. Morrowind's towns and run speeds are slow.

**Options.**
- **A. One realistic tuning.** Souls; AC III tuned climbing to real climbers (R7 §3).
- **B. Arcade momentum.** faith-runner build-up to sprint, speed gates for moves (R1 §5).
- **C. One physical scale, per-mode multipliers.** Dying Light 2: realistic run, slightly
  exaggerated jumps, stretched airtime (R7 §3).
- **D. Tuning per encounter in combat.** Hit-stop shrinks with crowd size (R8 §4).

**Reconciling design.** C plus D. One world scale. Modes (sneak, normal, free-run, combat) are
profiles of multipliers in data. Jumps are set by height (R1 §5). Hit-stop scales with enemy
count. Price: tuning profiles in `sf_controller`, a momentum mover mode for the player
(R1 adopt 9), test courses per mode. Moderate tuning effort.

**Recommendation (default).** C plus D.

**Owner questions.**
1. How much faster and further than vanilla Morrowind should a top free-runner go?
2. Is free-run entered by holding sprint, or a toggle?

---

## 9. Combat stance vs traversal (lock-on vs free run)

**Conflict.** Mid-fight, a player wants to hop onto a table or flee over a wall. An assassin
wants to leap from a roof into a kill. Lock-on pins facing; free-run needs facing to follow
the stick.

**Options.**
- **A. Hard modes.** Weapon drawn limits traversal; sprint releases lock-on (R5 §4).
- **B. Everything everywhere**, with attack overlays during traversal: slide kick, vault kick
  (R1 adopt 11); grapple into attack keeps momentum (R5 §3).
- **C. One priority list.** Traversal actions are candidates beside combat ones, in a fixed
  order (Sekiro's candidate order, R6 §3). Starting a traversal breaks lock-on.

**Reconciling design.** C. Locked on, jump stays jump; explicit *up* still mantles. Traversal
to attack only through authored openers (jump attack, drop attack). Price: priority list design
and opener clips (none in our sources). Low to moderate.

**Recommendation (default).** C.

**Owner questions.**
1. Should a drawn weapon slow traversal (no wall run, slower climb)?
2. Are drop and air assassinations in scope for the first combat milestone?

---

## 10. RPG stats vs skill-based action

**Conflict.** In Morrowind, Acrobatics sets jump height, Athletics run speed, weapon skill hit
chance, Sneak detection. The owner wants skill-based feel: WISHLIST §9 already drops "invisible
dice rolls on a hit that visibly connected". The framework can't name Morrowind (rule 1), so
stats reach it only through parameters set by the content pack.

**Options.**
- **A. Stats gate moves.** Low Acrobatics: no wall run or mantle, unlocked by levelling.
  Breaks: no assassin play at level 1.
- **B. Stats scale magnitudes in bounded bands**: jump height, run speed, damage, stamina,
  perhaps reach. Timing and latency stay constant. Breaks: levels must work at both ends of the
  band; probe classes and warp refusal must scale with jump height.
- **C. Stats affect only outcomes after contact** (damage, stagger tier). Breaks: progression
  feels flat for movement.
- **D. Vanilla dice on hits.** Rejected by WISHLIST §9.

**Reconciling design.** B, with an optional gate rule in the content pack for a few moves.
Skill can also pick a clip set (novice vs master sword style) through the clip-set selector
(R5 adapt). Price: parameter hooks in `sf_controller` and the future combat crate; mapping in
bevyrrowind's rules plugin. Low.

**Recommendation (default).** B. Never scale timing windows or input latency by stats.

**Owner questions.**
1. Should a low-Acrobatics character be unable to mantle or climb, or just jump lower?
2. Should weapon skill change timing (faster recovery, wider parry), or only damage and stamina?

---

## 11. Where the decider lives, and how mods add moves

**Conflict.** Something must pick the next move each tick. FromSoftware uses a huge script per
archetype; AC1-rs uses code plus tables; our graph is not a chooser (R3 §7). Our Luau is
parked and runs a frame late, which makes controls mushy (R5 §7). Mods should still add moves.

**Options.**
- **A. A script per archetype.** FromSoftware (R5 §1). Breaks: latency with our Luau;
  unmoddable at that size.
- **B. Code plus tables.** AC1-rs (R3 §7). Breaks: mods need code.
- **C. Data-driven move rules in a Rust action layer**, on the fixed tick, with a bounded
  settle loop. R1 adopt 7, R5 §7. Mods add a rule and a clip slot; Luau gets hooks later.
- **D. Graph transitions with conditions.** Ours today. Fine for locomotion blends only.

**Determinism.** No multiplayer is planned (WISHLIST has none). A fixed tick still pays: scripted
course tests, headless combat demos, AI move sheets (R1 adopt 1-2, R6 §10).

**Reconciling design.** C for actions, D for locomotion blends. Price: a new spec and a module in
`sf_controller` or a new `sf_action` crate; a rule language with validation predicates
(R3 §11). Moderate; it is the backbone for sections 1-3 and 9.

**Recommendation (default).** C, on the fixed tick, Luau never in the per-tick loop.

**Owner questions.**
1. Until Lua returns, are data-only move rules enough for mods?
2. Is co-op or multiplayer ever a goal? If yes, the world body and the sim must be strictly
   deterministic from the start.

---

## 12. Climbing scope: ledges or holds

**Conflict.** AC-style free climbing of any wall needs a hold grid and 100+ consistent clips
(R3 §3, §9). Our sources cover perhaps 20-30 traversal moves from mixed performers (R9 §4).
Morrowind has no climbing markup.

**Options.**
- **A. Climb anything on a hold grid.** AC1 (R3 §3); Uncharted's hundreds of clips (R7 §3).
  Breaks: clip budget; AC itself pulled back from "climb anything" in Shadows (R7 §5).
- **B. Ledges only: grab, hang, shimmy, mantle, ladders, with IK.** GASP; Sekiro's hang and
  shimmy (R5 §3). Breaks: no wall climbing.
- **C. B plus climbable surfaces from a baked hold index, filtered by markup** (R3 adopt 3,
  R7 §5 adapt 3).

**Reconciling design.** B now, with the baked edge and hold index built so C can follow.
Hang, shimmy and ladder steps as IK on a few base poses (R9 §4). Price: hold bake in `sf_bake`,
docked mover mode (R5 adopt 5), `sf_interaction` edges. Moderate.

**Recommendation (default).** B for M4; C later if clips allow.

**Owner question.** Is AC-style wall climbing a must-have for the first release, or are ledges,
mantles, ladders and shimmies enough?

---

## Summary

| # | Conflict | Options | Recommendation (default) | Owner question |
|---|---|---|---|---|
| 1 | Who owns motion | sim / clip / warped / labelled regions | Per move kind: sim for locomotion, warp for contact moves, clip for attacks | OK to lose steering mid-mantle? |
| 2 | Commitment vs responsiveness | souls / free cancel / AC locked | Fast first response; commitment per move in windows; asymmetric timing | How punishing is a whiffed heavy? |
| 3 | Context vs explicit input | one button / explicit / up-down split / profiles | Up-down split, auto only while sprinting, sneak never triggers; assisted setting | May jump become a mantle in vanilla play? |
| 4 | Blending | instant / frozen crossfade / inertialize / + pose-matched entry | Inertialize, gap-sized, pose-matched traversal entry | none |
| 5 | First person | authored camera / arms rig / one body two renders / FP with cuts | One body is truth; FP render body plus world shadow body; baked camera per clip | Which view is primary? |
| 6 | Camera | free / lock-on / look limits / soft aim | Free default, FP look limits, optional lock-on, soft aim | Hard lock-on in first person? |
| 7 | What clips carry | rich tracks / nothing / code by name / bones + inferred + sheet | Bones only; bake infers; one line per attack slot | Owner video mocap for gaps? |
| 8 | Pace and scale | realistic / arcade / one scale + multipliers / per encounter | One scale, mode profiles, crowd-scaled hit-stop | Top free-run speed vs vanilla? |
| 9 | Combat vs traversal | hard modes / everything / one priority list | One priority list; traversal breaks lock-on; openers only | Drop/air kills in scope? |
| 10 | Stats vs skill | gate / scale magnitudes / outcomes only / dice | Scale magnitudes in bands; never timing | Low Acrobatics: no climb, or lower jump? |
| 11 | Decider and mods | script / code+tables / data rules in Rust / graph | Data rules in a Rust action layer on the fixed tick | Multiplayer ever? |
| 12 | Climb scope | climb anything / ledges / ledges + baked holds | Ledges for M4, hold index ready for later | Wall climbing a must for release? |

## Decisions that constrain each other

- **First person (5) → clips (7), camera (6).** One body as truth means hits come from the
  world body's baked socket tracks, and every full-body move needs a reviewed camera track.
  First person as primary view makes per-move look limits a field of every move rule.
- **Who owns motion (1) → decider (11) and clips (7).** A motion owner per move needs the move
  rule schema, and warped moves need root motion and a warp window from the bake.
- **Context input (3) → commitment (2).** If one button does context moves, accidents happen,
  so traversal must stay short and cancellable. Explicit intents allow more commitment.
- **Pace (8) and stats (10) → climb scope (12) and probes.** Jump heights that vary with stats
  or modes mean affordance classes and warp refusal ratios scale with them, and level fixtures
  (Balmora tests) must pass at both ends.
- **Clips (7) → climb scope (12) and blending (4).** Bones-only, mixed-source clips force
  gap-sized blends, pose-matched entry and IK-based hanging; they rule out a hold grid for now.
- **First person (5) + pace (8).** Fast free-running in first person makes comfort settings
  mandatory (Dying Light tuned clip speed and FOV for sickness, R7 §2.2).
- **Combat vs traversal (9) → camera (6).** If traversal breaks lock-on, lock-on must
  re-acquire cleanly on landing, which the camera spec has to define.
- **Multiplayer (11).** A "yes" changes 1, 2 and 5: the world body and fixed-tick sim become
  strictly authoritative, and presentation never feeds gameplay.
