# Physics tint: archive

Moved out of [physics-tint.md](../physics-tint.md) to keep the spec under the doc budget. The spec and the ledger
(`docs/ledger/physics.md`) win where they differ.

## AN38a/b measured cost table (moved from physics-tint.md "Performance" 2026-11-26)

**Measured (AN38a/b, 2026-11-26;** `mw_delirium/tests/tint_cost.rs`, release, 60 warm-up + 600 timed ticks, N agents in
one loop on one thread; ledger physics.md "AN38a", "AN38b"), median / p99 ms per tick:

| State | N = 1 | N = 2 | N = 4 | N = 8 | ms per body | At 2 s |
|---|---|---|---|---|---|---|
| A balancer (a takeover's standing phase) | 0.50 / 0.88 | - | 1.78 / 3.00 | 4.21 / 8.50 | 0.44-0.53 | stands |
| B balancer + soft upper body | 0.67 / 0.98 | - | 2.73 / 4.52 | 5.47 / 8.63 | 0.67-0.68 | stands |
| C balancer + followed upper body (`Sway` option) | 0.63 / 0.90 | 1.27 / 1.96 | 2.65 / 4.43 | 5.29 / 9.10 | 0.63-0.66 | stands |
| D stumble after a 45 N s chest push | 0.54 / 0.89 | 1.04 / 1.57 | 2.05 / 3.13 | 4.10 / 6.37 | 0.51-0.54 | recovers, stands |
| E stagger-fall after the same push | 1.17 / 1.90 | 2.24 / 3.34 | 4.54 / 7.68 | 8.59 / 13.60 | 1.1-1.2 down, 0.56 standing | fallen |
| F whole-body pose follow, no balancer, no pin | 0.86 / 1.49 | - | 3.32 / 5.23 | 6.71 / 10.35 | 0.83-0.86 | collapsed (pelvis 14 %) |
| **G = F + pinned pelvis (the tint's path)** | 0.65 / 1.04 | **1.28 / 2.13** | 2.54 / 3.62 | 5.19 / 8.79 | 0.64-0.65 | stands (100 %) |

Cost is linear in N (no shared cost); p99 is 1.5-2x the median. The pin makes pose follow stand and is cheaper than F (F
is a body collapsing onto the floor). A body on the ground costs about twice a standing one (E). C at N = 4 once ran a
slow phase (4.26 / 4.54).

## AN38c pose error by state (2026-11-26; ledger physics.md "AN38c")

`mw_delirium/tests/tint_cost.rs`, ignored test `tint_pose_error`: one agent starts in the bind pose and follows the
asset's zero pose; error = part rotation relative to the pelvis, sim vs target, degrees mean / max, at 0.5 / 1 / 2 s.

| State | Upper body | Legs | Pelvis height |
|---|---|---|---|
| a body that does not move | 19.1 / 46.9 | 2.9 / 10.6 | - |
| A balancer, no pose drive | 20.5 / 46.5, 20.2 / 47.1, 19.8 / 47.1 | 8.6-9.1 / 13.1-13.5 | 98 % |
| C balancer + followed upper body | 2.3 / 4.1, 10.0 / 33.5, 8.0 / 26.8 | equal to A's | 98 % |
| F whole-body follow, no pin, no balancer | 7.7 / 15.1, 9.8 / 17.5, 9.9 / 17.1 | 2.5 / 4.1, 4.5 / 8.0, 9.4 / 29.4 | 14 % at 2 s |
| G = F + pinned pelvis (the tint's path) | 8.5 / 17.2, 11.2 / 20.5, 10.9 / 19.8 | 4.7-4.8 / 12.3-12.5 | 101 % |

The balancer does not overwrite the followed pose (C reaches it within 0.5 s) but C does not hold it; the test does not
show which part drifts or why (a per-part print would).

## Group step: where Delirium's split starts (AN38d; moved from physics-tint.md "Performance" 2026-11-26)

**Group step (AN38d, 2026-11-26, one clean run:** every number in this paragraph rests on it; test `tint_group_cost`,
ledger physics.md "AN38d"). G through Delirium's `Agent::step_group`, median ms per tick at N = 1 / 2 / 4 / 8: 0.46 /
0.83 / 1.61 / 0.84; p99 at N = 4: 2.24 in one run, 4.0 in the other (one 12.9 ms tick); at N = 8: 1.1. Up to 5 agents
the group runs on one thread (0.40 ms per body); from 6 Delirium splits it, one task per agent, on its own thread pool
(0.105 ms per body at 8). The threshold is not a setting: `par_each` (Delirium `cb8ccec`, `delirium/src/ragdoll.rs:
4901-4909`, crate-private) splits when `min(pool threads, N / 3) >= 2`; the 3 is slot 0 of Delirium's constant table
(`delirium-data/src/kv.map`, stored as is, not reachable through `unlock_own`); the pool has one thread per core that
shares the L3 cache (`:4811-4818`).
Chunk size is `ceil(N / (4 x cores))` (slot 1 = 4), so at N = 8 each agent is its own task.

## "Performance" as written for the per-agent loop (cap 2; superseded by the group step 2026-11-26)

**Measured (AN38a/b, 2026-11-26;** `mw_delirium/tests/tint_cost.rs`, release, 600 timed ticks, N agents in one loop on
one thread; ledger physics.md "AN38a", "AN38b"; the full table, states A-G at N = 1-8:
[history](archive/physics-tint-history.md)), ms per body per tick: G, the tint's path (whole-body pose follow with a
pinned pelvis), 0.64-0.65, stands at 100 % pelvis height; C (balancer + followed upper body, the `Sway` option)
0.63-0.66; F (pose follow with neither pin nor balancer) 0.83-0.86 and collapsed (pelvis 14 %); E (stagger-fall after a
45 N s chest push) 0.56 standing, 1.1-1.2 once down. Cost is linear in N (no shared cost); p99 is 1.5-2x the median. A
body on the ground costs about twice a standing one, hence the promotion rule in the provider contract.

**Pose error (AN38c,** one body following a pose from the bind pose, no motion; degrees mean / max; numbers per state:
[history](archive/physics-tint-history.md)): the tint's path G holds ~10 / ~20 on the upper body and 4.8 / 12.5 on the
legs, steady from 0.5 s to 2 s (an undriven body: 19-20 / 47). So at strength 1 a tinted body is near the clip, not on
it: the weight ramp, not the strength, makes the hand-back popless, and the standing offset is part of the look
ANIM-14 (1) judges. The provider's stiffness trades it against give; AN40b prints it per kind on a walking clip.

**Budget: tints + takeovers at the cap <= 2.0 ms median and <= 2.7 ms p99 per tick** (2.7 ms = the frame's p99 11 ms
minus its median 8.3 ms; physics share, not the 25 us animation budget). **Cap 2 holds:** G at N = 2 is 1.28 / 2.13 ms
(first 2 s 1.25 / 1.71), C 1.27 / 1.96; N = 3 is ~1.9 ms median by the linear cost with p99 over the limit; N = 4 fails
(2.54 / 3.62); 8 never. So `cap` = 2 (Low 1), replacing the Guess of 4. AN42 checks it in the game with takeovers mixed
in. Takeovers are never refused, so the cap only stops tints adding to them: four takeovers alone cost 2.0-3.1 ms a
tick today, cap or not.

**AN38 still to time** (d): N = 1-8 through `step_group` (what a grouped provider would gain: the game steps each
agent on its own today); body construction and drop; the impulse at which a push first takes a catch step
(`band.max_ns`). Over budget: lower `cap`, then `step_divisor` 2; never
delete the feature. A grouped or parallel step that cuts the per-body cost raises the cap by the same rule; it waits on
the provider's thread-safety check (R§5).
