# Local avoidance (ORCA)

Moved verbatim out of [locomotion.md](locomotion.md) on 2026-10-26 (doc budget, S11); current design, the section below is unchanged.

## Local avoidance (ORCA; orca, 2026-10-12)

Industry-audit gap G2, job M3-b. `sf_controller::avoid` (`Avoidance`, `AvoidAgent`, `AvoidParams`,
`AvoidRole`), Bevy-free, about 500 lines with docs.

**Canonical technique.** Optimal reciprocal collision avoidance (ORCA: van den Berg, Guy, Lin,
Manocha, "Reciprocal n-Body Collision Avoidance", ISRR 2009, Springer STAR 70, 2011), the
velocity-level method behind RVO2, UE's Character Movement RVO and, in sampled form, Detour
Crowd. Each neighbour within `neighbour_dist` (5 m, closest 8) contributes a half-plane of
velocities that stay collision-free for `time_horizon` (2 s) if both sides take their share; a
2D incremental linear program picks the velocity closest to the preferred one inside the
speed disc, and when the half-planes leave no room a 3D program minimizes the largest
violation (paper section 5). Written from the paper; no OpenMW code.

**Where it runs.** Per frame, after intents are written (player input, `PathFollower`,
scripts) and before `KinematicMover::step_batch`: `Avoidance::solve(&agents, &params, dt, &mut
desired)` takes each character's feet, current velocity (`MotionState::velocity`, what the
others see), preferred velocity (the intent's `desired_velocity`), radius, max speed, role,
priority and direction of travel, and writes back the desired velocity (vertical part kept for
flyers and swimmers). Avoidance steers the intent and the springs smooth it; walls stay the
mover's collide-and-slide, and `separate` remains the last-resort push for overlaps ORCA
can't prevent (it works on velocities only, so a crowd pinned against a door frame can overlap
by centimeters). UE does the same: RVO runs inside the movement component on the requested velocity.

**Rules on top of the paper** (each tested):
- *Player non-reactive* (`AvoidRole::NonReactive`): its velocity passes through unchanged and
  every other agent takes the full avoidance against it (RVO2's and UE's avoidance weights).
- *Give-way* for idle characters (preferred speed under `idle_speed`, 0.1 m/s): against a
  moving agent the idle side takes `yield_weight` (9) to the walker's 1, so standing NPCs step
  aside and walkers keep their line, as OpenMW's "NPCs give way" behaves (from observation).
- *Slack split.* The paper and RVO2 apply one responsibility share to the whole ORCA line, so a
  pair that is *not* on a collision course also splits its slack by that share: a walker with a
  10 % share against a standing NPC was capped at 0.36 m/s from 5 m away. Here the share applies
  to a required change and the slack is split the other way round; each pair's two parts still
  sum to the full `u`, so the reciprocal guarantee holds.
- *Doorway and corridor deadlocks* (ORCA's known pitfall: symmetric opposing streams meet in a
  choke point and stall). Of two agents walking against each other (directions of travel more
  than 90° apart), one yields (`yield_weight`): the lower `priority`, or for equal priorities a
  fixed heading rule (the one travelling toward +X when the pair runs mostly along X, else toward
  +Z), a traffic-rule convention, so a whole stream yields together. The rule needs the route's
  direction (`AvoidAgent::travel`, the path segment): the steering direction toward a doorway
  waypoint cuts across the flow and flips the rule mid-door (measured: deadlock). Priorities must
  be coarse ranks (escort, guard, citizen); per-character values (an id hash) mix yielders into
  both streams and jam (measured). Plus a *keep-right bias* (0.15 rad turn of the preferred
  velocity with an oncoming neighbour), the RVO2 examples' random perturbation made
  deterministic, so open-space streams form lanes.
- *Determinism.* No randomness (dodgy picks a random direction for exactly coincident agents; we
  split by position, else +X). Neighbours are chosen by (distance, index), so `solve_one` (brute
  force) equals the batched result bit for bit.
- *Low-speed jitter* is left to the velocity springs (half-life 0.27 s). An idle dead zone was
  tried and removed: an idle agent only gets a nonzero result when standing still would collide,
  so a dead zone only suppressed needed sidesteps.
- *Known limit*: ORCA assumes agents reach the chosen velocity at once; our springs lag (UE RVO
  has the same issue with acceleration). The tests run through the real springs and pass; the
  acceleration-aware variant (AVO, van den Berg et al. 2011) is not needed so far.

**Industry check.** RVO2 (the paper's reference library) and `dodgy_2d`: one responsibility
share per line, random tie-breaks, obstacle lines for static polygons. UE: RVO in the Character
Movement Component (avoidance weight, groups to ignore) or the Detour Crowd manager (Recast's
crowd: adaptive velocity sampling around the preferred velocity plus path-corridor
optimization and wall segments from the navmesh), one or the other. Unity: `NavMeshAgent`
avoidance at a chosen quality level, with `avoidancePriority` (0–99, an avoiding agent ignores
lower-priority agents): the same idea as our ranks, used the same way (coarse classes, not ids).
Known ORCA pitfalls and how they are handled here: deadlocks in corridors and doorways
(yielding by priority or heading rule, keep-right bias; measured above), jitter at low speed
(springs), reciprocity broken by acceleration limits (tests run through the springs), and
agents pushed into walls the solver can't see (mover slides; navmesh paths in M10).

**Neighbours.** A per-frame spatial hash (Teschner et al. 2003) of the agents by counting sort,
cells of half `neighbour_dist`, searched ring by ring with an exact early-out once the 8th
closest is nearer than the next ring: dense crowds look at 3 × 3 cells, sparse ones at 5 × 5.
The static query world isn't used (it holds no dynamic agents).

**Measured** (`cargo test -p sf_controller --all-features avoid`, real mover on flat ground,
60 Hz, radius 0.3 m, walk 1.5 m/s):
- Head-on, two agents 10 m apart: pass with a 4.3 cm minimum gap, arrive in 7.15 s; without
  avoidance they overlap (gap < −0.5 m). Offsets and crossing angles (0–1.2 rad) pass too.
- Player head-on: the player's line drifts 0 mm, the NPC goes round (gap 4.2 cm).
- Give-way: a walker passing a standing NPC swerves 7.5 cm, the NPC steps aside 0.74 m, gap 4.9 cm.
- Doorway, 1.5 m wide in a wall, 4 + 4 agents swapping sides: all arrive in 13.2 s (equal
  priorities, heading rule) and 13.1 s (one side ranked higher); 3.4 cm worst overlap at the
  door frame. Without yielding, or with interleaved per-character priorities: no finish in 20 s.
  Without the keep-right bias: 12.0 s (yielding, not the bias, clears doorways).
- Batched = single on 150 scattered agents (sparse, dense, a 3 m cluster).
- Time (`avoidance_timing_150_agents`, ignored; `cargo test -p sf_controller --release
  avoidance_timing -- --ignored --nocapture`; one thread, battery, three quiet runs): 150 agents over 40 × 40 m
  0.054–0.056 ms median (≤ 0.068 p95); 150 in a 12 × 12 m square 0.087–0.089 ms median (≤ 0.112
  p95). Budget 0.1 ms; the test asserts the median, so it fails while another build loads the machine.

**Reuse order** (superfargoth CLAUDE.md rule 8). Bevy 0.20 has no avoidance. Crates: `dodgy_2d`
0.5.5 (MIT/Apache, a tested ORCA port of RVO2 with obstacles) was evaluated and not taken: it
pins glam 0.29 and rand 0.9 (we are on glam 0.33 and rand 0.10: two extra crate families in both
workspaces, against the compile-time rule §8b), calls `rand::random()` for coincident agents
(non-deterministic: breaks batched = single and replays), allocates its line lists per call,
takes neighbours as a `Cow<Agent>` slice per agent, and applies responsibility to the slack as
RVO2 does (the give-way problem above). Patching it means forking most of its agent code, and
its static-obstacle half isn't needed while walls are the mover's. `landmass` (navigation with
avoidance through dodgy) stays an M10 candidate. So: our own, from the paper, about the size of
dodgy's agent part. No new dependency.

**Plugging in.**
- `sf_controller_bevy` (to build): one system between "intents written" and the mover, gathering
  `AvoidAgent`s from the active ring's characters (`Locomotion`, `MoverParams`, intent); the
  player's controller marks itself `NonReactive`. One `Avoidance` per thread if it is ever
  chunked (0.05–0.09 ms single-threaded for 150, so not yet). Counters: agents, neighbours per
  agent, 3D-program fallbacks. Gizmos: the chosen velocity next to the intent's.
- `PathWalkScaffold` (`mw_ai`): `travel` = the current path edge's direction (node to node, not
  toward the steering point), `priority` 0 for ordinary NPCs, `max_speed` the gait's speed.
  Waiting at nodes makes an NPC idle, so it gives way; it doesn't walk back to its spot
  afterwards (the follower's next stop does). The scaffold's stuck timeout stays the last
  resort. When `sf_ai` (M10) owns intents, it fills the same fields.
- M4: the physics controller keeps this stage unchanged. ORCA obstacle lines for walls (the
  paper's static obstacles) are not planned: navmesh paths (M10) keep agents off walls.
