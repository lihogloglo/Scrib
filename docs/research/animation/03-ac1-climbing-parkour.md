# R3: AC1-rs climbing and parkour

Status: done (r3, 2026-10-18).

Source: https://github.com/Goku436/AC1-rs (MIT), read at `6d935e0` (2026-10-08): `docs/PARKOUR.md`,
`docs/NOTES.md`, `crates/game/src/{climb,climb_table,jump,move_blend,level,city,character}.rs`, 91 commits.

**Clean room.** AC1-rs plays Assassin's Creed's own clips and move graph from the user's install;
much of `PARKOUR.md` was traced from the executable. This note gives mechanisms and rounded value
*ranges* only: no game identifiers, clip names, tables or flags. AC1-rs's own modules are named.

## 1. Shape and root ownership

The character is in **one movement context** at a time (ground, air, ledge hang, climb, wall run,
beam/post, ladder, bar, hay); a switch is immediate, a setup record (entry kind, target, first move)
filling the new one. AC1-rs keeps nearly all of it in one 5,200-line `climb.rs` state holding **a
queue of clips played back to back**, each with a target state, a playback rate and a root correction.
- **Ground:** root motion of one 17-clip blend (5 gaits × straight/lean/bank left/right, plus
  slow-down and take-off), weighted from one 0..1 speed value in four bands, all on one normalised
  clock, so speed is the blend's own (`move_blend.rs`).
- **Discrete moves** (climb, hang, mantle, stop, turn, landing): clip root motion **plus a correction
  spread over the clip** so it ends on the computed target (§5).
- **Jumps:** clip displacement + correction, or a ballistic arc with the flight clip time-scaled (§4).
- **Falls:** gravity; walls stop horizontal speed.

## 2. Where holds come from

**Authored first, geometry as fallback** (`level.rs`, `city.rs`).
- **Authored markup:** grab edges with a top normal and a wall normal, ladders (vertical edges),
  poles (bars). Edges steeper than ~1:3 are stairs or roof slopes, not holds.
- **Geometry detection:** (1) every boundary edge of an up-facing triangle that no other top face
  shares, level and ≥ ~0.3 m long, is a **lip candidate** with an outward direction; (2) validated
  against collision at three points (2 of 3 must pass): a wall facing out under it at two depths
  (~0.3 and ~0.9 m down: rejects kerbs and risers) within ~0.25 m of the lip, the top level just
  behind, and **room for the hands** in front.
- **Perches:** open stretches of beam tops (headroom above, not inside a solid) and **narrow wall
  tops ≤ ~0.35 m deep**. At 0.6 m a city got >10,000 perches and running along roofs flipped in and
  out of balancing at every parapet (`a1ba106`).
- **Inside-solid test:** closed outward-wound meshes, so most of four level rays hitting back faces
  means inside. Rejects beams inside walls and hangs whose body would be in a wall (`de2984c`).
- **Top-out needs room to stand:** ~1.7 m headroom ~0.5 m in, no wall just past the edge.
- Hay, benches, ladders, bars without markup: found by mesh name, fitted against collision.

**Edge classification for jumps:** probes from just above and outside each candidate edge measure
**top depth** (none: the wall goes on up / narrow < ~0.3 m / medium < ~1 m / wide), **what is under**
(step < 0.5 m / low < 2 m / wall under within ~0.7 m / **nothing: free hang**), then strike moves from
{step onto, run onto, wall hang, free hang}: narrow tops can't be stepped onto, only wide ones run on,
hangs need ~2–2.5 m of drop in front, a wall hang needs a wall under. Last, a **clear line from the
chest** to the landing or the hang's chest.

## 3. Climbing: a hand-configuration graph on a hold grid

- **Stance = hand configuration:** hands level, left higher, right higher, each narrow or wide (six
  wait loops). Separate states: **wall hang** (feet braced; hands open/closed), **free hang**, ladder, bar.
- **Grid:** cells ~0.75 m across × ~0.6 m up around the feet (~7 × 8). A hand cell holds when an edge
  lies within ~±0.3 m along, ~±0.15 m vertically and ~1 m in/out of the wall plane; feet use the
  cell two rows under each hand.
- **Input is wall-relative and quantised:** stick up = up the wall, whatever the camera. Ten
  directions: up and down split by which side of vertical (that picks the hand), diagonals
  (~22–68°), sideways. A **(stance, direction) table** gives end stance, moving hand and grid delta,
  or redirects. A **long** move (~1.2 m hand over hand) is tried first past half stick.
- **Candidates in fixed order, first valid wins** (`take_first`): table move → corners (inside, then
  outside) → into the wall hang if no footholds → swing into a free hang. From a hang sideways: onto
  climbing holds lower, level, shimmy, higher.
- **Fallbacks** (`try_move`): plain moves → top-out → **leap** to the next hold if out of reach →
  bring hands level, retry → step off at the bottom (down, or sideways past the last hold) → else a
  **look-around** toward the stick (~0.3 s fade), so a held stick always gives feedback.
- **Atomic moves, no buffering:** input is read only between moves; a held stick chains them.

## 4. Jumps and target selection

**AC1 always jumps at a target**; a press with none does nothing (free running off an edge excepted).
1. **Search volume** along the wanted direction from mid-body, from ~0.5 m ahead, ~±1 m across:
   ~3 m up, ~5 m down, up to ~9 m out; a short narrow box for waist-high tops. Edges must face the
   jumper within 45°.
2. **Classify** (§2).
3. **Score per kind:** **tops, posts, beams: the nearest** in the cone; **hangs: the highest**, unless
   the nearest is within ~2.5 m of it; more than ~3 m down excluded. A hang beats a top when at least
   as high and nearer, or much higher.
AC1-rs's reduced version (`jump_target`): reach ~1–4.6 m, ≤ ~1 m off the line, holds only 1.3–2.6 m up
(lower edges are tops), roof edges landed ~0.45 m in. Every target needs a real gap, a clear way at
knee, chest and over-the-top heights, landing room and headroom.

**Execution** (`jump.rs`, `jump_aimed`):
- **Blend space + correction:** takeoff and flight are mixes of variants by height (up / level /
  down / deep) and distance bands (~2.5 / 5 / 7–8 m), takeoffs also by direction group off perches.
  The residual error is spread over the **flight**. **Over ~50 % of the distance: refuse**, use the arc.
- **Planned arc:** flight time ≈ distance / run speed (≥ ~0.4 s), a minimum up speed, raised to
  clear a higher edge; the flight clip is **time-scaled to the airtime**. Too close to the edge for
  the takeoff's run-up, **the takeoff starts partway in**.
- **Arrival:** reception by target kind (top, beam, post, wall hang mixed by the wall's lean, swing).
  It is **cut where its step slows to run speed**, handing velocity to locomotion (`899a243`).
- **Mid-air catch:** hand box ~0.4 × ±0.3 m, edges within ~70°; the stick > 20° to a side looks for a
  wall there and turns to it. Walking off an edge catches only with the legs held.

## 5. End-pose validation and synchronisation

The core (`end_pose`, `chain_end`, `take_first`). Per candidate:
1. **Sample the last frame** (chains: accumulate root motion and yaw) from the current root: world
   wrists, ankles, root and yaw.
2. Holds for **both hands**: plain moves in the grid box round each hand; leaps (~0.75 m reach, also
   0.6/1.2 m lower for side and down leaps) and corners (~0.6 m) snap to the nearest. **An up-leap
   must land higher.**
3. Holds as **far apart as the clip's hands** (± ~0.35 m): no catching across a gap.
4. **Feet:** the move declares wall or free; rays from each foot and between them. A wall > ~0.32 m
   behind the hold line is no footing. Stances also need **a foothold ~0.5–1.7 m under each hand**,
   else the move ends in the wall hang (`bb29a24`: climbing on air).
5. **Sideways body clearance** against a wall meeting this one.
6. **Correction** = mean hand error (+ hang offsets), carried by the chain's clip with the most root motion.

**Applying:** root = start + root motion(t) + correction × k, k **linear** in normalised time, or
**cubic** for pull-ups from the waist (rise first, go over the lip late). Leaps between a short and a
long variant are **mixed toward the target** first. Moves from the ground **ease in**: start offset
folded into the correction, start yaw eased out by smoothstep over ~0.25 s. Ground-sliding moves stop
at walls and at drops > ~0.6 m and follow ground height. Hangs **square up** to the wall after a move.

**Blending:** fade from the last shown pose (smoothstep), lasting the graph's blend time, else **sized
by how far hands and feet jump** (~0.05–0.25 s seams, up to ~0.3 s into loops). Fades start from the
pose before IK, so IK isn't counted twice; root yaw fades the way that stays continuous.

## 6. IK (interface; R4 covers solvers)

- Climbing IK weight rises ~4/s, falls ~5/s (**~0.25 s in, ~0.2 s out**), on only while hands are on
  the wall (off for corners, top-outs, leaps). Instant switching jumped feet 10–13 cm (`6638335`).
- **Hands:** weight from **distance to the hold**: full within ~6 cm, none beyond ~20 cm, so IK is on
  at contact and off in transit **with no contact annotations**. Each hand **latches one hold** until
  its weight fades; weight changes ≤ ~6/s. Without this a long reach was yanked onto a passing hold
  and released in two frames (`372e2dc`). Grip a few cm out from and under the edge.
- **Reach assist:** a hold out of arm's length rotates clavicle and upper spine toward it.
- **Feet:** ray into the wall, distance exponentially smoothed, weight ≤ 0.8, off beyond ~0.3 m.

## 7. Profiles, modes, gates

- **Low profile:** walk; no automation; at a > 2 m drop the walk halts and looks down after ~0.25 s;
  legs pulls down into a hang.
- **High profile:** run; a legs press (buffered ~0.3 s) jumps only at a target; into a wall it grabs
  (holds ~1.8–2.4 m up with footholds → climb; a ledge ~0.5–3 m → standing jump to kneel or hang).
- **Free run = high + legs held:** sprint, and it acts by itself: runs up or grabs any wide wall
  (≥ ~0.7 m) it's steered into, steps (≤ ~0.85 m) or jumps onto low obstacles, jumps off drops
  ≥ ~0.8 m when a target is ahead, mounts beams. **Wall run is tested before jumps and grabs.**
- **Wall run:** sprint, stick within ~45°, flush wide wall; entry + one step, then the highest
  reachable ending (hold, kneel on a top, free hang, over a thin wall, ladder) or push off and fall
  (still catching). **Rebound** by the camera-relative stick snapped to back/left/right. No side wall
  run: no clips, and a procedural one ran through walls (`bb29a24`).
- **Per-clip gates** say *when* a move may be interrupted: locked; may leave for standing (per
  profile); may leave for moving; clip turns the body; and **leading foot before/after**. Smoothness
  comes from playing to a gate plus per-item blend times (0–0.3 s). Locomotion resumes **at the
  stride phase matching the pose**, on the foot the last clip names.
- **The graph is not the chooser:** code + tables pick jumps, climb moves, landings; the graph gives
  follow-ups, blend times, gates, and a **lint**: off-graph clip steps are logged by the test suite.

## 8. Failure modes and tooling

- Jump into an awning, stop mid-air, drop back, repeat → clear way at three heights + landing room.
- Re-catching the ledge just leapt from → up-leaps must end higher.
- Hopping off kerbs into planters → no jumps off drops < ~0.8 m or into a knee-high wall.
- Mount/dismount every frame where a beam meets a floor → don't mount where floor continues.
- Hangs in walls → inside-solid test; side moves → body clearance.
- Free-hang jump flew through an arch → aim the **root for the end state's hang** (~2 m under a free
  hang's hold, ~1 m under a wall hang's), not the hands (`cede5af`).
- City holds off the clips' steps → vertical tolerance ~0.35 m for up/down, depth correction for
  set-back storeys (6 → 9 of 20 city walls topped, `470fe00`).

**Tooling:** ~66 scripted scenarios tagged by system, 6 games in parallel (~2 min); a 10 s **flight
recorder** (root, clip, limbs, IK targets per frame); a **pops** detector (bones moving much faster
than last frame, by clip); a **climb probe** over the 20 nearest city walls; a "why" log of every
rejected candidate and by how much.

## 9. Clips and annotations

**No warp windows, no contact curves.** Per clip: root motion and yaw (the end pose is the ground
truth), leading foot at start and end, gates, blend-in time, end stance, and blend-variant parameter
(height, distance, angle, lean). Families: gaits by speed with lean/bank; starts, stops, pivots,
90/180° turns per foot and profile; six stance loops with ~10-direction short and long moves, corners;
hang loops, shimmies, hang↔climb, look-arounds; top-outs per hang (into a stand **or straight into
walk/jog** with the stick held); two-length leaps as chains (wind-up, flight, catch, settle); takeoffs
× direction × height × distance, flights by target kind, receptions, landings (soft/hard ×
straight/forward × into gait, damage, roll); wall-run entries and endings, rebounds; beam crouch
walk/jog, turns, 30/90° side entries; ladder, bar swing, hay, bench sit by side and angle. Gaps are
filled by **mirroring** or the other foot.

## 10. Situations

- **Sneak:** low-profile walk and a crowd "blend" walk (upper-body layer); no stealth crouch.
- **Roof to roof:** the strongest part (§4), plus leap of faith into hay.
- **Ledge grab / mantle / climb:** grab, wall run, mid-air catch, pull-down; grid climb, hangs,
  corners, leaps, top-outs. Low obstacles stepped or jumped onto; no vault proper.
- **Sit:** benches only, turn-and-sit by approach angle. **Door/lever:** none. **Sword:** removed.
- **First person:** none; orbit camera, left free on walls since climbing input ignores it.

## 11. For us

**Adopt**
1. **End-pose validation as every traversal action's acceptance test**, with all its rules (hands in
   box, spacing kept, feet wall/free, foothold under each hand, side clearance, inside-solid) and a
   **max-correction refusal**. `sf_anim` must sample a clip chain's end pose from a candidate root
   cheaply. M4 traversal chooser (`sf_controller` + `sf_anim_graph`).
2. **Ordered candidates with explicit fallbacks** ending in a no-move feedback clip.
3. **Holds as a persistent spatial set, baked from mesh** (lip candidates + collision validation +
   perches + inside-solid) per cell, since Morrowind has no markup; adapters may add markup.
4. **Jump target search** with per-kind scoring, cone, arc clearance, landing room; aim the root for
   the end state.
5. **Proximity-driven IK weights** with hold latching, rate limits, global fades, gating by move class.
6. **Exit-velocity hand-off** and gait resume at matching foot and phase.
7. **Test tooling:** tagged scenarios, flight recorder, pops detector, rejection log, a climb probe
   over a real town (Balmora as fixture).

**Adapt:** low / high / free run → sneak / normal / an opt-in "assassin" free-run (Morrowind keeps
manual jumping). The grid exists because AC1's clips step a fixed distance; with mixed-source clips,
measure each move's hand displacement at bake and validate against that. Six stances cost ~60
moves; start with two (level, offset) plus full-body IK reach.

**Reject:** string states and clip names as the move graph; fixed thresholds (ours scale with rig
measures); prop detection by mesh name (use interaction points).

**Where our spec is wrong or thin**
- **animation.md "Prior art"** says jump targets go "highest first": wrong; nearest for tops, posts
  and beams, highest only among hangs. It implies moves are pure data; in AC1 the **chooser is code +
  tables** and the graph gives gates, follow-ups, blend times and a lint. "A mod adds a move with a
  clip and a rule" needs that rule language (candidate order, validation predicates) specified.
- **Layer 1 (warping)** is single-clip and window-based. Missing: **warp across a multi-clip chain**
  (one clip carries it), **correction profiles** (linear, late/cubic), **refusal above a ratio**,
  **variant mixing before warping** (so the residual stays small), **time warping** to physics airtime,
  **start alignment** and **partial start** near an edge. Mixed-source clips will lack windows: we
  need a default whole-clip window with these profiles.
- **Layer 2 (IK)** gives the solver, not the **policy**: target choice, latching, proximity weights
  vs clip curves, rate limits, gating, foot smoothing, shoulder/spine reach assist. `IkTarget` needs
  the surface normal.
- **Layer 3 / "Context probes"** handle one obstacle ahead (≤ 4 queries). Climbing needs a
  **persistent hold index** queried round the hands each move; jumps need a **cone search**.
  `Affordance` lacks perch/beam, wall vs **free hang**, thin wall to go over, ladder, bar, top-depth
  classes; ledge checks lack hand room and inside-solid; there's no **arc clearance** test.
- **Missing:** per-clip **gates** and **leading-foot tags** outside combat; a no-move feedback state;
  landing damage by drop; mid-air catch; input conventions (wall-relative climbing, camera-relative
  rebound).
- **Clips are the bottleneck:** AC1-rs relies on 100+ purpose-made traversal clips with consistent
  hand spacing and step lengths; R9 must budget §9 against our sources.
