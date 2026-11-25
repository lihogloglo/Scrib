# Animation diagnosis: slope lean, arms behind the back, roll lag (2026-11-25)

Diagnosis slice (read-only: no code, build or game run) of the owner's 2026-11-25 report: slope lean with hands
behind the back; slow roll that can't be spammed; context: Kimodo, UAL and attack clips look bad, combat should use
Morrowind's own animations.

## 1. Slope lean and hands behind the back

### Causal chain (code)

1. **Grade feed.** `GraphInstance::update` computes the ground grade along the travel from the foot probes,
   `ground_grade` (`superfargoth/crates/sf_anim_graph/src/procedural.rs:260-282`: least-squares line through the probe
   heights plus normals, clamped to ±1 = 45°), filtered by `GRADE_HALFLIFE` 0.25 s
   (`sf_anim_graph/src/instance.rs:48,892-895`); 30° ramp -> ~0.58. Player probes since P-IK1 (`avatar.rs:213-289`).
2. **Lean op, humanoid values** (`bevyrrowind/crates/mw_graphs/src/build.rs:1085-1099`):
   `Lean(joints: chest 0.45, upperChest 0.4, neck 0.15, gain 0, grade: (up: 0.8, down: 0.075, max: 0.45), halflife 0.3,
   gate "lean")`. Rig names map to `Bip01 Spine1`, `Bip01 Spine2`, `Bip01 Neck` (`mw_graphs/tests/slope_walk.rs:1044-1046`).
   `GradeLean::pitch` = `up * grade`, clamped to `max` (`sf_anim_graph/src/asset.rs:590-607`). So 10° -> 8.1°,
   20° -> 16.7°, 30° -> capped at **25.8° (0.45 rad)**; the `GradeLean` doc says "a few degrees at most"; `max_angle`
   (15°) caps only the turn roll. The `build.rs:1089` comment (+5/+10/+16°) predates the 2026-11-05 retune.
3. **Application** (`procedural.rs:523-535`): each joint gets `Quat::from_rotation_x(pitch * share)` in model space
   through `sf_anim::ik::rotate_joint` (`superfargoth/crates/sf_anim/src/ik.rs:31-43`), which rotates the joint and
   **carries its whole subtree**. Shares sum to 1.0, so the head and everything above Spine1 end up pitched by the
   full angle (+23° head pitch measured at 30° on 2026-11-11; P-IK4 "head stays level" never built).
4. **Why the arms go back.** The arms hang from the clavicles, below the rotated joints (Biped: clavicles are children
   of `Bip01 Neck`; not re-checked in the cooked skeleton). `rotate_joint` keeps their pose relative to the trunk.
   When the trunk tips forward 17-26°, "down along the trunk" points down **and back**, so arms that keep their
   trunk-relative angle swing back toward the hips. A real walker's arms keep hanging with gravity, which relative to a
   leaning trunk means forward. No op counter-rotates the upper arms (the `HandIk` pair after `Lean`,
   `build.rs:1100-1113`, is weight 0 outside mantles; `SpringBones` by default runs only on tails).
5. **The base clip makes it worse.** Free movement now plays the Kimodo gait set (A3, 2026-11-12). Its arms already
   hang behind: Kimodo swing -36..+15° vs Morrowind's own -40..+31° (animation ledger 2026-11-12 arms diagnosis), and
   its trunk is hunched and arched (pose diagnosis: walk.cycle.1 Spine1 -16° vs bind). The lean was tuned on Morrowind
   walk clips (lean 4° back on the flat; AN-21 on Fargoth, 2026-11-05) and now rotates a Kimodo pose instead.
6. **No speed term**: jog and sprint get the full walk lean; NPCs share the humanoid graph.

### Ranked by confidence

| # | Cause | Confidence | Evidence |
|---|---|---|---|
| 1 | Lean up 0.8 rad/grade, cap 25.8°, shares summing to 1.0 (head takes all of it) | high | values in `build.rs:1098`; head +23° measured 2026-11-11 |
| 2 | Spine rotated without counter-rotating the arms: arms follow the trunk, so they point back | high (geometry of `rotate_joint`) | `ik.rs:31-43`, no arm op after `Lean` |
| 3 | Kimodo base: backward arm swing and hunched trunk, lean tuned on Morrowind clips | medium-high | ledger 2026-11-12 arm and pose diagnoses; AN-21 signed off before A3 |
| 4 | Full lean at jog/sprint (no speed term); NPCs share it | medium | op has no speed input |
| 5 | Grade noise on stairs/rough ground (0.25 s filter, then 0.3 s on the angle) | low | not measured on the gait set |

**Not confirmed from code:** the clavicles' parent in the cooked skeleton (Neck vs Spine2); the grade read on real
terrain (only synthetic ramps measured); whether hands-behind shows on the flat too (= clip alone, cause 3). Ruled
out: root tilt (nothing tilts the drawn root to the ground normal).

## 2. Roll: where the lag comes from

Timeline of one roll (player, keyboard; `superfargoth/crates/sf_action`, sheet
`bevyrrowind/crates/mw_combat/data/movesets/sword_1h.ron:153-162`):

| Stage | Where | Value | Effect |
|---|---|---|---|
| Tap fires on **release** | `sf_action/src/command.rs:206-208` `TapHold::DODGE_SPRINT` (ticks 18), sample `:308-328`; binding `mw_player_bevy/src/lib.rs:293-295` | roll = release before 18 ticks (0.3 s) | latency = how long the key is held, ~80-150 ms on a normal tap, up to 0.3 s, + ≤ 1 frame + ≤ 1 tick (16.7 ms). This is the "starts late" of CB-3a. |
| Blend into the clip | `mw_player_bevy/src/avatar.rs:75-76` `CLIP_BLEND_IN` 0.10 (= sheet `defaults.blend_in`), inertialized in the `full` slot (`:375-390`) | 0.10 s | inertialization reaches the clip fast; minor. |
| Clip lead-in | slot `dodge.roll.fwd` = UAL1 `Roll` (`sources.ron:365-372`), 1.47 s, `rate 1.25`, `root_scale 0.75` | move 1.18 s, travel ~3.75 m | how long the clip waits before it drops into the roll is **not measured** (only "99% of travel by clip 0.87 s"). Root `Progress`: the body moves only as fast as the clip's root does. |
| **No input buffer during the roll** | roll windows have only `Cancel`s (`sword_1h.ron:158`); `windows.rs:209-232`: a move with gating windows and no `Accept` accepts nothing; `step.rs:353-360` latches a press only inside `accept` | Dodge/Light presses during the whole roll are **dropped** | mashing Shift during a roll does nothing; the next roll needs a fresh tap released after the window. Attacks, by contrast, `Accept` Dodge from 0.10 s. |
| Recovery lock | `Cancel([Light, Heavy], 0.70)`, `Cancel([Dodge], 0.72)`, `Cancel([Move], 0.74)`, `NoTurn(0.0, End)` | the Dodge cancel at 0.72 s is dead without an `Accept` (nothing is latched); in practice the next roll comes after the hand-off to locomotion at 0.74 s (stick held) or the clip end at 1.18 s (stick released) | roll-to-roll cadence ≥ 0.74 s + the next tap's hold time, ~0.85-0.9 s; 1.2-1.3 s with no stick. |
| Stamina | cost 12; start needs stamina > 0, regen 45/s (`sf_combat/src/rules.rs:148-167`) | ~8 rolls from full | not a cause. |

Also seen: a roll restarted on the very tick the previous one ends keeps the same `rule` and `key`, so
`avatar.rs:340-342` only rewinds the playhead (no new blend): a pop, not lag; medium confidence, rare while presses
are dropped.

Ranked: (1) no `Accept` on the roll, so presses are dropped and rolls can't be chained; (2) roll on release (the
key's hold time is added to every roll); (3) 1.18 s move with recovery opening at 0.72-0.74 s; (4) UAL clip lead-in
(unmeasured). Not confirmed from code: the clip's lead-in and its exact recovery pose; the gamepad tap length
(same `TapHold`).

## 3. Clips in use today (player locomotion and combat)

From `bevyrrowind/crates/mw_graphs/data/animation/sources.ron` and `mw_graphs/src/build.rs`:

| Use | Slot / state | Source |
|---|---|---|
| Free walk / jog / sprint (sheathed, spell stance) | `loco.walk|jog|sprint.cycle` (`sources.ron:120-145`) | Kimodo loco1: `walk.cycle.1` take 05, `run.cycle.3` take 04, `sprint.cycle.1` take 05 (loop cut, root `Match`) |
| Weapon-stance walk/run, sneak, swim, idle, turns, jump, lock-on strafe | graph states (`build.rs`, `walk.none.directional` etc.) | Morrowind `.kf` groups (`WalkForward1h`, `TurnLeft`, `IdleSneak`, `SwimWalk`, `Jump@head`, ...) |
| Draw / sheathe | `upper` slot (`mw_player_bevy/src/combat.rs:374-406`) | Morrowind `<Weapon>@equip` / `@unequip` |
| Light chain 1-4 (+ `_rec`) | `attack.sword_1h.light1..4` | UAL2 `Sword_Light_A..D` (+ `_Rec`); fallback UAL1 `Sword_Attack` |
| Heavy chain 1-4 (+ `_rec`) | `attack.sword_1h.heavy1..4` | UAL2 `Sword_Heavy_A..D` (+ `_Rec`) |
| Roll | `dodge.roll.fwd` | UAL1 `Roll` (no Morrowind roll exists) |
| Guard raise / hold / break | `guard.sword_1h.*` | UAL2 `Sword_Block` (ranges), `Idle_Shield_Break`, fallback `Hit_Knockback` |
| Hit reactions | `react.small.front/left/right/back`, `react.medium/large.front` | UAL1 `Hit_Chest`, `Hit_Shoulder_L/R`, `Hit_Stomach`; UAL2 `Hit_Knockback` |
| Mantles, vault, climb, get-ups | `traverse.*`, `getup.*` | UAL1/UAL2 (`ClimbUp_1m/2m`, `SafetyVault`, `ClimbLedge`, `LayToIdle`, `Crawl_Exit`), LAFAN1 fallbacks |

Morrowind's own attack clips are already in the graph: `WEAPON_SECTIONS` (`build.rs:359-370`) registers
`HandToHand|WeaponOneHand|WeaponTwoHand|WeaponTwoWide@chop|slash|thrust`, bow/crossbow/throw `@shoot`, `SpellCast@*` and
`Shield@block` as `upper` slot clips (masked from the chest up, played over locomotion). The player's sheet does not
use them: its attack slots play UAL clips full body in the `full` slot.

## 4. Fix options (no code)

**Slope lean and arms.** Standing fact (animation ledger): don't retune slope ops on clips about to be replaced. The
owner now rejects the Kimodo walk/run, so the base-clip choice comes first; options 1-3 hold on any base.
1. *Small, speed-aware procedural lean* (UE5 GASP / ALS slope lean: a few degrees, scaled down with speed): `up`
   ~0.25-0.35, `max` ~0.15 rad (≈ 9°), a speed fade. Effect: lean 8-26° -> 4-9°. Tunables only.
2. *Head stabilization* (P-IK4; ALS / Control Rig head look): counter-rotate the head (and most of the neck) by the
   chain's pitch, gaze a few metres ahead on the ground. Removes the tilted head. Needs a small op change.
3. *Lean with arm counter-rotation* (procedural lean + per-limb gravity compensation, ALS "arms in world space"
   blend): after `Lean`, rotate each upper arm back toward gravity by the trunk's added pitch (a share, ~0.7-1.0).
   Hands hang forward of the trunk instead of behind it. Needs an op option (or `Lean` targets with a negative share
   on `leftUpperArm`/`rightUpperArm`, which `rotate_joint` already allows: check after the clavicle parent).
4. *Additive slope poses* (Paragon / Lyra: up/down slope additives blended by grade). Best look; none exist yet.
5. *Fix the base arm swing*: matched retarget offsets or arm IK against the body capsule (ledger 2026-11-12, not built),
   or Morrowind walk/run for the player (Classic set exists). Removes the backward arm bias on the flat too.

**Roll.**
1. *Input buffer* (Souls / Sekiro two-stage buffer, already in `step.rs`): add `Accept([Dodge, Light, Heavy], ~0.35,
   End)` to the roll so a press in its second half is latched and fires at `Cancel([Dodge], 0.72)`. Data only; makes
   rolls chainable at the cancel point. Highest value, lowest cost.
2. *Earlier recovery cancels* (Souls cancel windows): move Dodge/attack cancels to the end of the i-frames + travel
   (measure the clip first), or trim the clip's tail; with a higher `rate` if the roll reads slow.
3. *Press-to-dodge* (Sekiro, Nioh, DMC: dodge on press, sprint on a separate hold or after the dodge) or a shorter tap
   threshold: removes the hold-time latency. Elden Ring itself rolls on release; this is a feel decision for the owner.
4. *Trim the lead-in* (clip `range` start) once a headless test prints root travel over the first 0.2 s.
5. *Restart pop*: start a new inertialized play when `out.started` repeats the same rule (`avatar.rs:340`).
   Roll source: Morrowind has no roll, so it stays a pack or generated clip.

**Combat on Morrowind clips** (owner context). The `upper` slot already has every Morrowind weapon section
(`WEAPON_SECTIONS`). Standard: upper-body attack layer over locomotion (OpenMW, Morrowind's own model), hit and cancel
windows from the clips' text keys (`min hit`, `hit`, `release`, `follow start`). Depends on the sheet playing a slot
in `upper` instead of `full` and on reading the text keys into windows; root motion then comes from the capsule
(Morrowind attacks don't step), so the Elden Ring step-in needs a small warp.

## Missing

Nothing from the brief. Unmeasured: the `Roll` lead-in, the clavicles' parent, the grade on real terrain.
