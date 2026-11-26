# Visual checks: anim-lab

Open visual checks: animation lab and animation fixes (AL-*, AN-*). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

## Animation lab (`levels/anim-lab.ron`, test-levels.md; `.sessions/anim-refactor.md`, slice 1)

One greybox for the animation checks, six areas apart. Open the scenes below (or `scene level-anim-overview --camera <name>`; cameras are in the
level file). NPCs only run their scripts near the camera (processing range), so fly to an area first. All entries: kind synthetic level, status
awaiting owner. Known issue ids this feeds: SN-2/3 (guar, beasts), CH-1/3/4 (dressed NPCs), SB-1/2/3/5 (spring bones, tails), HI-3, AP-2/3 (not
here), PH-2/9/10 (ragdoll, knockdown, hit reaction).


### AL-2: Walk loop, turns (SB-1, CH-1)
- `scene level-anim-loop` (cameras `loop`, `loop-close`). Fargoth walks a 12 m square, Elone runs a 24 m square.
- Look for: foot sliding or skating at the four corners, a smooth turn (no snap, body leads), cycle speed matching the move speed at walk and run,
  spring bones and tails settling after turns. Status: awaiting owner.
- Slice 4b (headless, 2026-10-22): sidestep and backpedal no longer slide (walk right 30, run back 55, run strafes 32-36 cm per stance,
  now 0). Strafe (A/D) and back-pedal (S) in a free-walk `view`, at walk and run: feet planted, no skating. The price: speed matching
  now plays strafe clips up to 2.2x (was 1.5x), so a run-strafe has a brisk cadence. If it looks frantic, lower SpeedMatch `max` in
  `mw_humanoid.animgraph.ron` (and `build.rs`) and accept some slide. Status: awaiting owner.

### AL-3: Ramps and stairs (feet on slopes, SB-1)
- `scene level-anim-stairs` (cameras `slopes`, `stairs`, `stairs-close`). Runner on 10 deg, walkers on 20 and 30 deg and on 0.15 m x 12 and
  0.25 m x 8 stairs, all to a 2 m plateau and back.
- Look for: feet on the surface (no floating or sinking, no knees through treads), lean with the slope, no pelvis jitter on the steps, a
  steep (30 deg, 0.25 m) climb still plausible. K toggles foot placement (A/B).
- Slice 5 (headless, 2026-10-22): ramps within 2 cm (toe no longer sinks on 30 deg); comfy stairs going up: toes lift onto the next
  tread (was 16 cm into it, now about 5; heel may hang over the nosing). Known bad: the steep 0.25 m flight at walk speed (feet sink or
  float up to ~0.6 m; needs shorter steps, not IK). Status: awaiting owner.


### AL-8: Waist mantle, player (action.md B2a; body: A2 slice 2)
- Needs a bake after 2026-10-26 with the UAL2 library (log: "player traversal: 1 of 1 slot clips baked").
- `scene level-anim-ledges`, F11; walk W into the yellow 1 m box, then W+Space (or Space while walking).
- Look for: the body rises onto the top (~1 m past the lip) in ~0.7 s playing the climb clip in step with the move
  (playhead = the warp's progress), then blends back into walk and drops off the back normally; red 2 m box: a plain
  jump (now the high mantle: AL-10); Ctrl (sneak) + Space: never a mantle. The orange 1.5 m box refuses from a stand at the face (feel/tuning is
  the owner's call). Hands on the lip and feet on top are not IK-fitted yet (B2b). Owner 2026-10-27: mantle alright;
  body clipping into the box (clip, B2b IK: stays logged); camera too close on the bigger box.
- Camera fix 2026-10-27 (camera.md "As built (traversal pivot)"): the pivot rode the climb root through the lip, so on the
  1.5 m box jumped from close the arm collapsed into the chest; now the pivot leads up to the top's height. Look for:
  walk-jump onto the orange box from close and from ~1.5 m out: the camera rises smoothly ahead of the body, keeps its
  distance, no pop in. Walls still pull it in. Status: camera confirmed (owner 2026-10-27: "solved"); hands on the lip
  and the body clipping: B2b (next).





### AL-5: Props push (PH-1, physics-adapter.md)
- `scene level-anim-props` (cameras `props`, `props-close`); F11 (player under the camera), push crates, boxes and the table; `V` throws.
- Look for: the NPC brushing the crate row without sliding through it, light boxes shoved with a lean or step (not a stuck walk cycle), the
  300 kg block unmoved, crates not jittering. Status: awaiting owner.


### AL-7: Idles, reactions, other rigs (SN-2/3, CH-3, SB-3, PH-10)
- `scene level-anim-idles` (cameras `idles`, `idles-close`). Left to right: Fargoth, a Khajiit (Ajira), an Argonian (Huleeya) fidgeting with
  different idle variants; Elone hit-reacting every 4 s; a dummy knocked down every 12 s; a guar; a mudcrab.
- Look for: distinct fidget clips (no T-pose, no sliding), flinch and recovery on the hit, Khajiit and Argonian legs and tails bending
  correctly, creatures standing idle (no graph fidget for them yet). Status: awaiting owner.
- Variety (CH-1/CH-3 crowd): `scene level-anim-crowd` (camera `idles-crowd`): four identical Fargoths, same idle and fidget set. No two neighbours
  breathe in sync, fidgets begin at different times and vary, none repeats back to back (`@fidget`: Idle2/3/4 at 50/30/20, every 3-8 s).
  Status: awaiting owner.

### AN-7: Capsule steady under a walker (owner pass 2026-10-22, slice R3-1)
- `scene level-anim-loop` (camera `loop-close`), `G` for collider wireframes.
- Look for: the orange capsule glides with the walker and the runner, a constant hair behind them (one physics
  step, ~2 cm walking, ~7 cm running), no shaking or stutter. Was: target from the last frame's feet, beating against
  the 60 Hz step. Status: awaiting owner.


### AN-3: Footsteps on the feet (slice R3-1)
- `scene level-anim-loop` (camera `loop-close`), sound on; then `level-anim-props` for the walker's stops.
- Look for: each step sound as a foot lands (left/right alternate), at walk and run, none while standing. Sounds come
  from the clips' `SoundGen` keys now (were one per 0.7 m of travel). Status: awaiting owner.

### AN-2: Walkers stop within a step (slice R3-2)
- `scene level-anim-stairs` (camera `stairs`): the ramp/stair walkers and the runner turn round at each end after
  a 1 s wait; `level-anim-overview` shows the same area from afar.
- Look for: a walker slows over its last step (about 0.5 m, under 1 s) and stands, the runner over 2-3 steps
  (about 2 m, 1.5 s); no long glide, no slow-motion walk at the end, idle blends in as it settles. Was: braking
  from 2 m out plus a 1.5 s creep at 0.25 m/s (3 s). Status: awaiting owner.




### AN-8 Creature capsules fit the creature (awaiting owner)
Scene `level-anim-idles` (or `-overview`, camera `idles-close`), F7 + G on. Look for: the mudcrab and the guar (last two
on the row) have a physics shape over their whole body (an upright cylinder, orange, as wide as the shell or body and as
high as the back), not the old 0.4 m column inside the shell; people keep their capsule. Throw a ball (`V`) at the crab:
it should hit the shell's outline. Known: the shape is round, so a long body (guar) is covered with some slack at the sides.

### AN-9 Planted foot holds through the heel roll (awaiting owner)
Scene `level-anim-loop` (camera `loop-close`). Look for: a planted foot stays put from heel-strike until the toe leaves the
ground; at weight transfer (the other foot landing) the heel rises and the foot rolls over a fixed toe, with no forward
slide. Any correction happens while the foot is in the air. Fix: cooked `roll.<foot>` curves (heel-off to toe-off) pin the toe.
Note (run 2): the male/female skeletons have no toe bone (only the beast races have `Toe0`); the cook and compiler use the foot's
socket child as the toe, so the roll works on humans too. Gated: `slope_walk` forward walk toe slide 2 cm per heel-off (14-16 cm
without the roll curves). Strafes not gated (lead: walk right 7/15 cm, likely swing-foot skim, check in the scene).

### AN-10, AN-13, AN-14 Get-ups (awaiting owner)
Scene `level-anim-idles`, camera `idles-close`. AN-13: throw a ball (`V`) at the guar: it falls, gets up once and stands
idle (it used to loop its knockout clip forever). AN-10: knock dummies down until one lands face down: it rolls onto its back
while rising, with no fast spin about the vertical. AN-14: knock someone down, press `I` on them while down: they stay dead.


### DE-1 Delirium takeover on a Morrowind NPC (ROADMAP 3.2b slices 2a-2b; confirmed 2026-10-27)
`cargo build -p bevyrrowind --features dev`, `powershell -File tools/run_dev.ps1 scene level-anim-idles` (camera `idles`).
Press F11 (free camera) first, then left-click a human NPC (Fargoth far left, Elone centre; no push in player mode): plain 45, Ctrl 20, Shift 80 N s at the clicked part. Recovery
confirmed 2026-10-27 (catch steps, fade into the idle). Fall: once down and slowing the body goes limp holding its pose
(`bodyRelax`: no balancer fidgeting; worst part speed lying still 0.66 -> 0.42 m/s, 6.9 -> 2.2 rad/s), gets up after
0.3 s at rest (down 0.8-2.6 s, was 2.5-4.3 s). Owner 2026-10-27 (first live LaFAN1 look): arms twisted, feet flipped,
walks off after standing, Morrowind clip never used; vibration nearly gone. Fixed: LaFAN1 retargets from its frame-0 T-pose
(twist error 121 -> 3-4 deg); candidates by how it lies: `Knockout@tail` (side, wins within 20 deg of the best),
UAL2 `LayToIdle` (back), UAL1 `Crawl_Exit` (front), LaFAN1 side_right (trimmed to planted feet); LaFAN1 side_left
dropped (walks 2.5 m). Look for: varied get-ups, no twisted limbs, ends standing in place. Log: `gets up with <slot>`
plus the gaps. Headless 8 falls: Knockout 5, LayToIdle 1, Crawl_Exit 1, side_right 1; push-to-standing 4.1-4.3 s.
Owner 2026-10-27 (after the re-bake): animations work well, "nice". Nitpick: the body sometimes slides a bit on the
ground before getting up, which delays the get-up (lying `still` check waits on it). Fixed 2026-10-27 (afternoon): rest
check on COM displacement + `roll_stall` timeout (head-right roll 3.0 -> 0.6 s); click-to-push F11 only, never on action
characters (player, dummy). Owner confirmed live: no floor slide, capsule back under the body after the get-up (hidden
while stumbling by design: Dormant). Status: confirmed. Open (later): a body landing crouched sinks/slides ~1.5 s first.
Re-check 2026-11-19 (confirmed): engine numbers now the no-game fork's stand-ins, not GTA IV's (build
`--features dev,delirium_push`). Look for: same as above, plus whether he feels stiffer or looser than before (headless:
80-120 N s chest pushes mostly ridden out by stepping, 160 N s floors him from the side). Owner 2026-11-19: a jump/flicker
back to the capsule at the end; fixed (`Mover::snap` at the hand-back); owner: solved.


### AL-TRUNK Matched retarget pose for the trunk (retarget-trunk 2026-11-12, browser confirmed by owner; in-game part open)
- Needs `bevyrrowind bake` (anim.retarget v5). Run (from `bevyrrowind/`): `cargo dev clip-preview --browse "../animation packs/generated/loco1"`
  (walk/run cycles), then a UAL clip (e.g. `Walk_Loop`, `Idle_Loop`); then `scene level-anim-ledges` (F11, mantle `ClimbUp_2m`) and `scene level-arena` (attacks).
- Look for: trunk more upright than before (no arched lower back, neck not jutting forward, head level at rest); limbs, feet and
  hands as before. What is left is the clip's own style (Kimodo walk arches ~8 deg, clavicles low; UAL walk leans its upper spine).
  The game's retargeted mantle and combat clips no worse (hands still on the lip, no new twist). Measured: `mw_bake/tests/it/generated_pose.rs`.
- Also (retarget-shoulders, anim.retarget v6): arms clear the chest in Kimodo walks (shoulders not pulled in, elbows out
  like MW's own WalkForward); UAL arms ~4 deg wider, shoulders not shrugged in runs. Measured: `mw_bake/tests/it/generated_arms.rs`.

