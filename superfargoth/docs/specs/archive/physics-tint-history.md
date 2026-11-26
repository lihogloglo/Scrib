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
