# Visual checks: anim-lab

Open visual checks: animation lab and animation fixes (AL-*, AN-*). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

## Animation lab (`levels/anim-lab.ron`, test-levels.md; `.sessions/anim-refactor.md`, slice 1)

One greybox for the animation checks, six areas apart. Open the scenes below (or `scene level-anim-overview --camera <name>`; cameras are in the
level file). NPCs only run their scripts near the camera (processing range), so fly to an area first. All entries: kind synthetic level, status
awaiting owner. Known issue ids this feeds: SN-2/3 (guar, beasts), CH-1/3/4 (dressed NPCs), SB-1/2/3/5 (spring bones, tails), HI-3, AP-2/3 (not
here), PH-2/9/10 (ragdoll, knockdown, hit reaction).

### AL-1: Animation lab overview
- `.\bevyrrowind scene level-anim-overview`. Look for: six areas apart (ledges + props near the origin, loop south, slopes/stairs west-north,
  drops north-east, idles south-west), nothing sunk or missing, every NPC dressed. Status: awaiting owner.
- Owner 2026-10-30: works perfectly. Status: confirmed.

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

### AL-4: Traversal heights (future vault / mantle / climb)
- `scene level-anim-ledges` (cameras `ledges`, `ledges-close`); F11 for the player, walk into each box and the 3 m wall.
- Look for: the capsule stops at each ledge and the body does not clip into the box (no vault, mantle or climb exists yet: this is the
  baseline for them); heights readable against the 1 m grid. Status: awaiting owner.
- 2026-10-30: retired (written before mantles existed; AL-10 and AL-11 cover traversal now). Status: retired.

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

### AL-10: High mantle, player (action.md B2 high mantle)
- Needs a bake after 2026-10-27 (`traverse.mantle.high` on UAL2 `ClimbUp_2m`). `scene level-anim-ledges`, F11; walk W into a box, W+Space.
- Look for: the orange h1.60 and h2.00 boxes play the high climb (hands above the head, body over the lip onto the top); the yellow
  h1.20 box keeps the waist mantle; the red h2.50 box and the 3.5 m wall refuse into a plain jump; Ctrl + Space never mantles.
- Owner 2026-10-27 (copy d): works, hands go above the head. Issues: the hands aim at the box centre, not the lip (B2b-2 hand IK, after
  the B2b-2a contacts); hard to trigger: the body slides along the face and doesn't face it (B2-T approach tolerance + yaw align, in
  progress).
- Owner 2026-10-30, after B2-T (99f6466) and B2b-2b/2c (130bf38, e5224f2): the mantle triggers and works; the hands reach the lip
  but the hand IK needs tuning. Status: confirmed (hand IK tuning logged on ROADMAP 3.1).

### AL-11: Wall brace, player (action.md B2c)
- No bake needed (runtime feed only). `scene level-anim-ledges`, F11; the grey 3 m wall behind the boxes (14 m long, z -7).
- Run (W) straight into the wall: both hands rise before the impact and plant on the face, arms slightly bent, palms on
  the wall (not floating more than ~2 cm, no fingers or forearm through it); release W: the hands let go over ~0.5 s.
- Walk along the wall about 0.5 m off it (turn parallel, W): only the wall-side hand trails lightly on the face, behind
  the shoulder; walking away: it lets go. Standing still: no brace. Near the wall's end the hand drops past the edge.
- F (sword drawn): only the left (off) hand braces. Mantles: the lip grip still wins (AL-10 unchanged).
- Known: the hand keeps its animated rotation (no palm frame yet), so fingers may angle oddly. Status: awaiting owner.
- Owner 2026-10-30 (release copy, 13:08 build): fails. Running into the wall, the character starts walking sideways along it,
  doesn't stop, and the hands never rise. F never draws a sword (never did), so the drawn case is untested.
- Fix 2026-10-30 (wall-brace-fix): the brace never ran (its gate waited for "no action slot", but plain locomotion always
  has one); now gated on no Clip/Warp move. Live dev build: both weights reach 1 at the wall, wrists on the face. The sideways
  walk is older and not the brace: pushing W at an angle into a wall, the mover keeps the along-wall part (collide-and-slide)
  while the body faces the stick, so the graph plays a side step (bug logged here; the fix is a mover/facing slice). Run in straight (camera
  square to the wall) to judge the hands. Needs a new exe copy, no bake. Status: awaiting owner.
- Owner 2026-10-30 (dev3 copy, after the gate fix): the brace now shows, but (1) running straight at the wall, the body
  starts moving sideways as soon as it hits: wanted a grazing-angle rule: below a threshold angle to the wall's
  normal the mover stops instead of sliding (mover `sf_controller` `mover.rs`; B2-T noted the same); (2) the side-wall
  trail pose is wrong: people feeling a wall reach the hand ahead of them, palm on the wall, thumb pointing down
  (needs the palm frame from the wall normal, the known rotation gap). Status: partial.
- Owner 2026-10-30: the wall brace needs a big fix; find where the sideways slide comes from first (diagnosis slice),
  then fix the stop and the trail pose.

### AL-9: Gait bands, pelvis and feet (pose audit S3, feed audit F4)
- Humanoid op data 2026-10-26: `StrideWarp` max 1.6 -> 1.2, `SpeedMatch` rate_share 0.3 -> 0.5, foot IK gated on `on_ground`.
- `scene level-anim-loop`: NPCs walk on flat ground. Then `scene level-anim-overview`, F11: player walk, run and sprint.
- Look for: pelvis steady, no bob at step frequency; no deep crouch when running or sprinting (was ~95 mm at 4.5-7 m/s, now
  under 1 mm headless); feet don't slide; feet let go of the ground in a jump. Sprint plays the run cycle at 2.2x
  (4.75 steps/s): if it looks frantic, a sprint cycle is the fix, not a longer stride. Run strafes still skate (clip issue).
  Numbers: `cargo test -p mw_graphs --test slope_walk gait_bands -- --ignored --nocapture`. Status: confirmed (owner 2026-10-27: "so much better"; movement feel follow-up under PL-2).

### AL-5: Props push (PH-1, physics-adapter.md)
- `scene level-anim-props` (cameras `props`, `props-close`); F11 (player under the camera), push crates, boxes and the table; `V` throws.
- Look for: the NPC brushing the crate row without sliding through it, light boxes shoved with a lean or step (not a stuck walk cycle), the
  300 kg block unmoved, crates not jittering. Status: awaiting owner.

### AL-6: Falls and ragdoll (PH-2, PH-9)
- `scene level-anim-drop` (cameras `drop`, `drop-side`, `drop-close`); `G` shows collider wireframes, `I` kills the NPC under the crosshair, `V` throws.
- Look for: the walker stepping off the 3 m edge (a fall animation or a snap? it must not freeze), the dummy knocked down every 10 s
  (`@impact` 25 N*s: ragdoll, then get-up), with `G` the capsules following the mesh, no pop into the ragdoll, no spin on the 15 deg slope.
  No 6 m fall exists for actors (no way to place one on the platform): throw or fly. Status: awaiting owner.
- 2026-10-30: retired with PH-2 (owner: the clip+limp ragdoll is deprecated, delirium from now on). Status: retired.

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

### AN-1: Planted feet stay put on distant walkers (slice R3-1)
- `scene level-anim-overview` (camera `overview`, all walkers at A2 LOD there), then `level-anim-props`/`loop`.
- Look for: on the props walker (beside the orange crates) and the loop walker, the standing foot doesn't drift with
  the body during its stance, far and near. Was: A2 updates got half the body motion. Status: awaiting owner.
- Owner 2026-10-30 (overview): works perfectly. Status: confirmed.

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

### AN-4 Knockdowns always come back
Scene `level-anim-overview` (or `level-anim-drop`), G on. Throw `V` balls at walkers, the knockdown dummy, the guar
and the mudcrab, hard and soft. Look for: every knocked-down character gets up within ~6 s, then has its capsule
(G) and walks/idles again; a creature without a ragdoll logs "knockdown cancelled" and keeps its capsule. Never a
character frozen lying or standing without a capsule.
- 2026-10-30: retired with PH-2 (delirium replaces knockdowns). Status: retired.

### AN-5 Get-up starts from the lying pose
Scene `level-anim-overview`, the `@impact` knockdown dummy (idles row, every 12 s). Look for: at the moment the
ragdoll hands back, no snap: the limbs flow from where they lie into the stand-up over ~0.3 s, the pelvis doesn't
jump, and the body doesn't rise or sink to meet the ground.
- 2026-10-30: retired with PH-2 (delirium replaces knockdowns). Status: retired.

### AN-6 Get-up ends on the capsule
Same scene, G on. Look for: when the get-up starts the capsule appears under the lying pelvis (not ~1 m away), the
character stands up onto it, and nothing slides when the clip ends and idle takes over.
- 2026-10-30: retired with PH-2 (delirium replaces knockdowns). Status: retired.

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
