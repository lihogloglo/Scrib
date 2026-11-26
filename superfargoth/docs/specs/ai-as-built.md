# NPC AI as built (`sf_ai`): history moved out of `ai.md`

Moved here to keep [ai.md](ai.md) inside its size budget. Newest slice first; the module docs in
`superfargoth/crates/sf_ai/src/` are the detailed reference.

## AI9d: `sim.cpu_tier = auto` (2026-11-26)

`auto` (default) resolves on every read through `auto_cpu_tier(logical_threads, headless)`: <= 8 Floor, <= 12 Medium,
<= 16 High, > 16 Ultra (`AUTO_TIER_MAX_THREADS`, from the reference rows above), headless Dedicated. Threads from std
`available_parallelism` (1 if unknown); a CPU without SMT reads one row low (explicit choice fixes it). Headless is a
host flag (`AiAllocSettings.host.headless`, default false). An explicit choice wins; `AiAllocStats.cpu_tier` shows the
resolved tier.

## AI9e: tiers on sight and hearing (2026-11-26)

`queue_sight`: Full (and no `AiTier`) checks every `sight_period_s`, Reduced every `tier1_period_s` (also its
`seen_recently`), Logical never (its meter still decays). A tier change restarts the `SightTimer` at that tick; the
first check lands on the timer's own stagger `due(key, tick, period)` (not `first_decide_tick`, which spreads over
the decide period). Hearing skips Logical listeners. AI9f: Reduced listeners hear by straight-line distance (no
`NoisePropagation` query, no door penalty), only in the noise's space (`same_space`: rejects when both the noise's
space and the listener's `AiExecutor.worldspace` are known and differ). `exec.tier` mirrors `AiTier`
(2/1/0) for traces. Tests `tier_tests.rs` (5).

## AI9b: allocator system `sf_ai_bevy::alloc` (2026-11-26)

`AiAllocPlugin` (added by `AiPlugin`), set `AiAllocSystems::Allocate` in `FixedUpdate` before sight and the
controller drive; `allocate_tiers` ranks every `AiActor` with a `Mover` (not `Dormant`) against `Perceivable` players
(`Mover` or `PerceivablePose`; plain distance: `WorldPos` has no worldspace). Writes `AiTier { tier, first_decide_tick }`
(own tick counter, same numbering as `AiPerceptClock`), removes it when the actor leaves the set. Host-written inputs:
`AiInView`, `AiPin(Pin)`. `AiAllocSettings` (cpu tier, caps table, config), `AiAllocStats` (+ `allocate_us` with
timings on). 15 tunables `ALLOC_TUNABLES` (`sim.cpu_tier` choice, `ai.alloc.caps_<tier>_{full,reduced}`, margins,
decide periods), in the game CATALOG. Nothing reads tiers yet (AI9e). Cost: 2000 actors 555 µs mean / 918 µs max per
allocate (release), over the Floor AI budget -> AI9c. AI9c (core only): key-sorted `Vec` + reused buffers, one packed
`u128` rank key per actor, `select_nth_unstable` per band, in-view holds without a sort; 64 µs mean / 155 µs max per
allocate, 161 µs per tick for the whole system (~0.03 µs/actor); a differential test vs the old code (300 runs x 40
ticks) matched exactly (removed after). No allocate period needed.

## AI9a: tier allocator core `sf_ai::alloc` (2026-11-26)

`Allocator::allocate(inputs, caps, config, tick, tick_s) -> Allocation { tiers, changes, removed, stats }` (all in
`ActorKey` order; pure, no clock). Rank: pin, then visible, then distance to the nearest player, then `ActorKey`; an
actor already in a band ranks as if `full_margin_m` (5 m) / `reduced_margin_m` (10 m) nearer (hysteresis; Guess); NaN
ranks farthest; duplicates count once. `CpuTier::caps()` = the table above (`ai.alloc.caps_*`). `TierChange` carries
`first_decide_tick` = `sight::stagger_phase(key) % period` (`full_decide_s` 0.15, `reduced_decide_s` 1.0). Pins exceed
caps (`over_cap_full` counts pins only). AI9a2: Full takes the pins, then the top `cap - pins` by rank (hysteresis
kept, no in-view holds); Reduced keeps visible incumbents (former Full included) past its cap (`held_in_view`,
`over_cap_reduced`); a visible actor never drops to Logical (decision below). Tunables are declared as
bound-later consts (AI9b binds them). Tests `tests_alloc.rs` (9).

**Decision (2026-11-26, industry standard, UE MassLOD / significance manager):** caps are hard between Full and
Reduced: when over cap, the lowest-ranked visible actors drop to Reduced (it only lowers decide rate and foot probes, a
small visible change). "Demotion only out of view" protects the step to Logical (tier 0 has no body): a visible actor
never drops to Logical, so Reduced may exceed its cap by in-view holds (reported).

## AI26: perception gizmos `sf_ai_bevy` (feature `debug`, 2026-11-25)

`perception_gizmos.rs` + tests (7). Console var `ai.perception` (off by default). Every non-dormant observer within 50 m
of the camera: ground arcs at sight and lose range, peripheral + central edges, band colour (grey, yellow, orange,
red-orange, red), text "band N%", LKP line (conf, age), "search held"; LKP sphere (+ ring while held); `AiSelected`
gets a white foot ring and every entry's LKP; targets show "light N%"; `RecentNoises` (2 s, cap 32) draws reach rings
(blue revealing, magenta not). Seam `AiDebugGoal(WorldPos)` (always compiled) for the game's search goal. The app's
`dev` now enables `sf_ai_bevy/debug` (direct dep), so AI3's `ai.gizmos` shows in the game too.

## AI8h1: band reactions core `sf_ai::react` (2026-11-25)

`ReactState::update(cfg, Option<ReactInput>, actor_pos, PerceptClock) -> ReactGoal` from the actor's top awareness
entry (`ReactInput: From<&Awareness>`, plain data). Unaware/Searching/Alerted = `None`; Curious = `Face { toward: LKP }`;
Suspicious = `Investigate { to: investigate_point(from, lkp, step_m), then_face: LKP }` (XZ line, never past the LKP).
Double take: first goal `face_delay_s` after entering from idle; escalation or a target switch adds no delay. Hold: a
band/target change or an LKP move > `retarget_m` re-targets at once, a smaller move after `hold_s`, an unmoved LKP
never; `revision()` bumps per re-target. After escalating to Searching/Alerted on a sighting, the decay back down on
that same sighting (same target, `LastSeen.tick`) gives `None`, so a guard who gave up returns to its routine (Skyrim,
MGS V); a newer sighting re-arms. `ReactGoal::operators` = `SearchGoal::operators` shape (Face; MoveTo + Face). Serde,
no RNG. Tunables `ai.perception.react_*` (4, Guess), bound by AI8h2. 11 exact tests.

## AI8c3: noise over portal paths (2026-11-26)

- **As built (AI8c3, 2026-11-26):** `sf_ai_bevy` `PortalPaths` (default `NoisePropagation`) asks `path_distance` over
  `NavWorldGraph` (≤ `ai.perception.noise_max_hops` = 3 links; links not open in `NavDoorStates` are closed doors) for
  tier-2 listeners, other worldspaces included; the straight-line prefilter runs only in the same space; no graph: other
  spaces unheard. UE hearing is straight-line; this is room/portal propagation (Wwise rooms & portals, OpenMW `heard`).
  A Notify's delay reads the heard distance (path + `closed_door_m` per shut door): a shout behind a shut door lands later.

## AI8c1: hearing system `sf_ai_bevy` (2026-11-25)

`hearing.rs` + `hearing_tests.rs` (9 pass; sf_ai_bevy 37). `NoiseEvent` message (spec fields + `notify`; `space:
Option<WorldspaceId>`, new `sf_ai::NoiseKindId(u16)`; `tick` = emitter stamp, heard at the perception tick).
Propagation seam `NoisePropagation(Box<dyn NoisePaths>)`, `NoisePath { path_m, closed_doors }`, default `StraightLine`
(UE AIPerception). Host `AiWeatherMask` (absent = 1). `hear_noise` in `AiPerceptionSystems::Hearing` (after Sight,
before `ControllerSystems::Drive`): listeners = non-dormant `AiObserver` + `AiAwareness` + `Mover`, ears at `eye_m`,
source skipped; reach = `loudness_m × hearing_mul × mask`, straight-line prefilter, distance = path + `closed_door_m`
(8 m) × doors; `Stimulus::Noise` then `Notify`; a heard noise forces a sight check. Events written before the set are
heard the same tick. Linear scan (few events per tick). Notify relay not yet a behaviour.

## AI8f1: search core `sf_ai::search` (2026-11-25)

`search.rs` + `tests_search.rs` (12 exact; sf_ai 134). One primitive operator `OP_SEARCH` (`register_search_operator`):
the executor (AI8f2) owns a serde `SearchState` and runs each `SearchGoal { GoTo | LookAround | Done }` through the AI6
executors (`SearchGoal::operators`: MoveTo, two Face). First the LKP, then a look; then rings of `ring_points` seeded
points (sight's splitmix, `search_seed(actor, tick)`) at `[inner_frac, 1] × r(t)`, `r = (error_m + widen_mps t) × (1 +
(1 - c)(uncertain_mul - 1))`, looks facing outward; unreachable skipped (≤ 2N+1 queries, else look in place); `Done`
sticky at `start + ttl_ticks`. Sources: TLOU2, Splinter Cell, The Dark Mod. NaN/negative params rejected.

## AI8d2: light exposure system `sf_ai_bevy` (2026-11-25)

`light.rs` + `light_tests.rs` (9 pass; sf_ai_bevy 28 + 1 ignored). Host resource `AiLightInputs { sky, interior,
sources }` (absent = off: no samples, no rays, sight unchanged), target component `AiLightExposure`. `sample_light::<G>`
runs between resolve_profiles and queue_sight: players every tick, actor targets on their staggered sight phase; at the
chest, roof ray up from the head outdoors only; linear `light_search_m` prefilter. LOS = two passes over one batched
`ProbeLos` call (record the rays `light_exposure` asks for, cast, replay in order): same tick, deterministic; the
`RayQueue` didn't fit (deferred, keyed by observer). Light rays are charged to `max_rays_per_tick` first.
`light_only_visibility(light, darkness)` = `apply_darkness` (or 1.0 unsampled) is the seam LW4's concealment replaces.
30 vs 144 Hz: exposure exact, bands equal, meter within 0.05 (3 vs 14 ticks per 0.1 s check). A source inside its
fixture's collider reads occluded: the host places it just outside.

## AI8d1: light exposure core `sf_ai::light` (2026-11-25)

`light.rs` + `tests_light.rs` (14 exact; sf_ai 122 pass). `VisibilityFactors` (Copy, serde; `Stance { Upright,
Crouched, Prone }`, `disguise` 0..1, `modifiers` inline `[Option<(StatId, f32)>; 4]`, neutral default), new
`StatId(u16)` (adapter maps `sf_stats::StatIdx`). `LightSource { pos, radius_m, intensity, on }`;
`light_exposure(at, sky, roof_covered, sources, los, &LightConfig)`: sky (× `roof_sky_mul` when covered; interiors pass
ambient as `sky`) + top-`light_rays` sources (inline top-k, cap 4), each `intensity × saturate(1 - (d/r)^k)²` (Karis
2013 window without 1/d²), LOS-checked strongest first, an occluded one not replaced; clamp 0..1, no allocation.
Non-finite inputs add nothing; NaN light is dark (`is_dark`). `apply_darkness` stays in `sight.rs`.

## AI8b2: sight system `sf_ai_bevy` (2026-11-25)

`perception.rs` (tunables, components, record kinds), `sight.rs` (resources, LOS adapter, plugin, systems),
`sight_tests.rs` (19 pass + 1 measurement; clippy clean, census clean, no new dep). Components: `AiObserver { eye_m }`
(requires `AiAwareness`, `AiSightTimer`), optional `AiPerceptionProfile` / `AiPerceptionStates`, cached
`AiEffectivePerception`, `Perceivable { target, chest_m, head_m }`. Resources: `AiPerceptionSettings` (all 38
`ai.perception.*` / `ai.awareness.*` tunables, `add_perception_tunables`), `AiPerceptClock`, `AiRays`, `AiSightCache`,
`AiPerceptionStateDefs`. `AiSightPlugin<G: GroundProbe>` in `FixedUpdate` (`AiPerceptionSystems::Sight`, before
`ControllerSystems::Drive`): begin_tick → resolve_profiles → queue_sight (stagger, XZ grid cell = lose range, gain,
cache or ray request; merged deferred requests sum their gain) → cast_rays (`ProbeLos`: one batched probe; game =
`sf_physics_bevy::Physics`, characters never block) → apply_sight (`ActorKey` order) + meter update. Records
`sf.ai.perception_profile` / `sf.ai.perception_state` (`register_perception_kinds`, `load_perception_states`). Choices:
"already seen" (switches to `lose_m`) = contact within two check periods; visibility placeholder
`VISIBILITY_UNMODELLED = 1.0` until AI8d; `max_rays_per_tick` bound to Floor 24 (no `sim.cpu_tier` groups yet).
Measured 0.037 ms/tick for 48 observers × 4 static targets, fake LOS, 60 Hz (cache absorbs most rays). Not covered:
worldspace/interior filtering, tier 1 (AI9), `ai.stats` ray counts. Game wiring (bevyrrowind) is its own slice.

## AI8b1: sight maths core `sf_ai::sight` (2026-11-25)

Built to "Sight gain" and "Cost and budget". `sight.rs` + `tests_sight.rs` (14 tests; crate 108 pass, clippy clean,
census unchanged). Types: `SightBudget` (tunables `skip_gain` 0.01, `sight_period_s` 0.1, `tier1_period_s` 1.0,
`max_rays_per_tick` 24 = Floor, `sight_cache_s` 0.25, `cache_move_m` 0.5; Guess), `seconds_to_ticks`, `SightCheck`,
`sight_rate` / `sight_gain`, `apply_darkness`, stagger (`actor_hash` splitmix64, `due`, `SightTimer` with `force`),
`RayKind` (Sight, Light, Concern), `RayRequest`, `LosRay`, `LineOfSight` (batch `cast`, also any `FnMut`),
`LosOutcome`, `RayQueue` (`push` merges same owner/kind/subject, `cancel`, `process` in two batches: oldest requests'
next ray, then head rays of chests just blocked), `RayTickStats`, `SightCache`. Choices where the spec was silent:
`skip_gain` compares gain per second (frame- and period-independent); `near_m` and `peripheral_mul` read from
`SightConfig`; 3D cone angle (UE); a blocked chest out of budget casts only its head next tick; a full queue or cache
refuses (fixed capacity, `prune` frees the cache); `SightTimer::poll` counts at most one period (no burst after a
tier change); results sorted by (owner, kind, subject).

## Perception profile: groggy option and `EffectivePerception` hook (AI8b, AI8g)

Moved from `ai.md` "Perception profile".

- **Groggy (option, decided in AI8b):** AI8a's built-in groggy (`AwarenessMeter::groggy`, `groggy_s`,
  `groggy_alertness`) may become a framework `groggy` state pushed on `wake()` (`alertness` = `groggy_alertness`,
  timed `groggy_s`), so a preset can make heavy sleepers; the effective config then sets `groggy_alertness` = 1 so it is
  never applied twice. Keeping it as built is equally valid: nothing else depends on the choice.
- **Hook: a cached resolved view, not scaling at each call.** `EffectivePerception` (component, `Copy`, ~120 B) holds
  a scaled `AwarenessConfig` copy, `SightParams { range_m, lose_m, central_half_deg, peripheral_half_deg, rate,
  darkness }`, `hearing_mul`, `SearchParams { ttl_s, error_m, widen_mps }` and `next_expiry`. Rebuilt only when the
  profile or states change, `next_expiry ≤ tick` (one compare per actor per tick) or a tunable reloads: no per-tick
  allocation or multiply (UE keeps per-instance sense configs the same way). Actors with neither profile nor states
  have no component and read the global config. Hook points, AI8a unchanged (it already takes `&AwarenessConfig`;
  callers pass `&eff.cfg`): `cfg.decay_curious / decay_suspicious / decay_searching` × `decay` (read by `update` and
  `apply` through `settle`); `cfg.noise_gain` × `alertness` (read by `noise_gain`). AI8b's gain uses `eff.sight.rate`
  (= `rate` × `sight_gain` × `alertness`), its range for the grid prefilter and `f_dist`, its cone for `f_cone`,
  `darkness` on `light` before `DetectionRules::concealment`; `AwarenessMeter::alertness` stays the band/groggy factor
  on top. AI8c passes `HeardNoise.loudness_m` = reach × `eff.hearing_mul` × `weather_mask`, so `noise_gain` and the
  sleeper rule (`asleep_wake_m`) read it unchanged. AI8f reads `eff.search`.

## AI8g: perception profile core `sf_ai::profile` (2026-11-25)

Built to "Perception profile (per individual)" (AI0g). `profile.rs` + `tests_profile.rs` (13 tests; crate 94 pass,
clippy clean, census unchanged). Types: `PerceptionField` (9, `ALL`, `name`, `range`), `PerceptionProfile` (ratios,
default 1.0, missing serde fields read 1.0, unknown fields rejected, `validate` → `PerceptionRangeError` naming every
bad field), `PerceptionState` (optional fields + `duration_s`, `until(clock)`), `StateEntry`, `PushOutcome`
(`Added`/`Refreshed`/`Replaced`/`Full`), `PerceptionStates` (inline 4, `push`/`clear`/`expire`/`next_expiry`),
`SightConfig` / `SearchConfig` (10 base tunables, bound from `ai.perception.*` in sf_ai_bevy later; `skip_gain` left
for AI8b), `SightParams`, `SearchParams`, `EffectivePerception` (`Copy`, `needs_rebuild`) and `resolve`. Choices where
the spec was silent: a full array of conditions refuses the push (`Full`: conditions are never evicted; timed entries
go earliest `until` first, lowest slot on ties); a condition counts as the latest expiry on refresh; unknown state ids
count as 1; entries already due are ignored by `resolve` before `expire` runs; a NaN product falls back to 1.0;
`duration_s` must be finite and ≥ 0. Not done: `sf_records` kind registration (AI8b2 / adapter). Groggy kept as built.

## AI8e: shared knowledge `Notify` (2026-11-25)

Built to "Awareness" (Notify line). `perception.rs` + `tests_notify.rs` (9 tests; crate 81 pass, clippy clean, census
unchanged). Types: `Notify { about, lkp, lkp_tick, confidence, hops }` (+ `relay()` = hops + 1, the caller shouts
again), `Stimulus::Notify { noise, notify }`, `PendingNotify`, `NotifyDelivered`, `StimulusOutcome::Queued { due }`,
`AwarenessConfig::notify_delay_ticks` (`ceil((d / notify_speed_mps + notify_react_s) / tick_s)`, f64),
`AwarenessMeter::update_with(cfg, now, on_delivered)` (`update` delivers and drops the reports). Four config fields
(`notify_speed_mps` 6, `notify_react_s` 0.8, `notify_confidence_mul` 0.7, `notify_max_hops` 3; Guess). Serde added
on the meter and its entries (pending notifies included; the meter is still not in the save). Choices where the spec
was silent: a delivery raises to `min(confidence, Searching)`, never lowers; LKP replaced only by an as-recent
`lkp_tick` (hearsay keeps its age); the delay uses the heard distance (closed doors slow the news); a shout wakes a
sleeper only with `asleep_wake_m` of reach left, the delivery still waits; `hops > max` is `Ignored` but the sound can
still wake; one pending notify per target, at most `max_entries` targets (best confidence, then earlier due, then fewer
hops; a tie keeps the queued one), delivered in (due, target) order; the shout's own sound is a separate
`Stimulus::Noise` the caller sends.

## AI8a: awareness core `sf_ai::perception` (2026-11-25)

Built to "Perception and stimuli" (AI0b). `perception.rs` + `tests_perception.rs` (12 tests; crate 72 pass, clippy
clean, census unchanged). Types: `PerceptTarget`, `AwarenessBand`, `LastSeen`, `Awareness`, `Distraction`,
`PerceptClock`, `HeardNoise`, `Stimulus` (Sight, Noise, Touch, Damage), `StimulusOutcome`, `AwarenessConfig` (spec
defaults, bound from `ai.awareness.*` in sf_ai_bevy later), `AwarenessMeter`, `PlayerDetection` + `player_detection()`.
Serde only on `PlayerDetection` and `AwarenessBand` (awareness is not saved). Choices where the spec was silent:
`PerceptTarget::Actor` holds `ActorKey`; sight arrives with its gain from AI8b; "seen" = last sight/touch/damage and the
Alerted hold counts from it; decay integrates across band changes, Unaware decays at the Curious rate and a 0-level
Unaware entry is forgotten; a big drop lands in the highest band whose stay level still holds (same as many small
steps); the wake test is the noise reach left at the sleeper (loudness - distance), and a waking noise still adds gain;
Touch raises to the Searching floor, damage from a known attacker alerts at once, sourceless damage only wakes; a
sourceless or non-revealing noise fills the distraction slot (replaced if as loud or older than `distraction_ttl_s`);
eviction drops lowest level, then oldest gain, then highest target, after settling all entries; `PlayerDetection.band`
= highest band over counting actors. New tunables (Guess): `distraction_ttl_s` 10, `alert_alertness` 1.5,
`groggy_alertness` 0.5. `Notify`: AI8e above.

## AI6: move executors `MoveTo`, `Wait`, `Face` and HTN `PlanRunner` (2026-11-18)

`sf_ai::exec` (Bevy-free; `sf_coords` + `glam` in `[dependencies]`, 0 Bevy crates): built-in operator vocabulary
(`OP_MOVE_TO = "move_to"`, `OP_WAIT = "wait"`, `OP_FACE = "face"`, `register_move_operators -> MoveOperatorIds`),
`ExecGait`, `FaceTarget { Yaw, Point }`, `OperatorKind` / `MoveAction`, `ExecConfig`, `NavMoveState`, `ExecContext`,
`NavCommand`, `ExecOutput`, `OperatorExec`, and `PlanRunner` (`OperatorResolver`, `OperatorBindings`).
- **`MoveTo`**: commands `NavCommand::MoveTo { goal, gait, speed }`; succeeds (`OperatorStatus::Succeeded`,
  `PathState::None`) on `NavMoveState::Arrived` or initial proximity (`<= arrive_dist_m`); fails immediately with
  `PathState::Failed(PathFailure::NoPath)` on `NavMoveState::Failed`; tracks the actor's closest planar XZ distance to
  `goal.pos`, resetting `no_progress_s` whenever planar distance improves by `>= progress_min_m` (`0.25 m`) and failing
  with `PathState::Failed(PathFailure::NoProgress)` at `stuck_timeout_s` (`3.0 s`, ai.md "Executors").
- **`Wait`**: commands idle locomotion (`NavCommand::Stop`, optional `facing`, optional `idle_variant`) while counting
  down `duration_s` by `dt` each fixed tick; succeeds as soon as `remaining_s <= 0.0`.
- **`Face`**: computes desired yaw (`FaceTarget::Yaw` or `FaceTarget::Point` via `atan2(dx, dz)`), commands
  `NavCommand::Stop` with `facing = Some(desired_yaw)`, and succeeds once `wrap_angle(desired_yaw - ctx.yaw).abs() <=
  tolerance_rad`.
- **`PlanRunner`**: runs either direct/queued operators (`step_direct`) or an HTN root `TaskId` (`step_htn`), monitoring
  dirty read keys (`plan.reads.intersects(&dirty)`) via `htn::replan` (`Replaced` / `KeptNotBetter` / `KeptNoPlan`),
  per-step primitive task preconditions (`lib.holds(step.task, ws)` -> fresh `decompose` on `Trigger::Replan`), and
  partial plan continuations across `Pause` (`htn::resume` -> `PlanOutcome::Continued`, falling back to `decompose` on
  failure), recording `OperatorState` and `PathState` into `TraceEntry`.
- **Bevy shell (`sf_ai_bevy::exec`)**: `AiExecutor` component, `AiExecPlugin`, `AiSystems::Execute` in `FixedUpdate`
  inside `ControllerSystems::Drive` ordered `.before(NavSystems::Corridor)` (replies are already published: `NavSystems::Publish` runs earlier, in `FixedFirst`; the `.after(Publish)` in code is a no-op) (plus
  `apply_idle_intent` `.after(NavSystems::Corridor)` so `Wait`/`Face` `desired_facing` is preserved when
  `NavCorridor::goal` is `None`); live tunables `ai.exec.{stuck_timeout_s 3.0, progress_min_m 0.25,
  face_tolerance_rad 0.05}` (Guess, Dev, Net::Gameplay, Live). `NavQueryPlugin` and `NavCorridorPlugin` added in
  `mw_walkers_bevy::nav::add`. Tests: `sf_ai` `tests_exec.rs` (4), `sf_ai_bevy` `exec_tests.rs` (2 headless Recast room
  integration tests).

## AI7: drives, utility arbitration (2026-11-17)

`sf_ai::drive` (Bevy-free, no new deps). Records `sf.ai.consideration` (`ConsiderationDef { input, range: (at0,
at1), curve }`) and `sf.ai.drive` (`DriveDef { considerations, weight, momentum, commit_ticks, floor?, directive }`),
registered by `register_drive_kinds`; every tunable is a required record field (units and ranges in the type docs),
none has a code default. `Inputs` is the adapter's input vocabulary (`InputId`, case-folded, registration order);
`Drives::build(db, inputs)` resolves names once and collects every unknown input, unknown consideration, bad value
(non-finite or equal bookends, non-finite curve parameter, unsorted or empty piecewise table, negative weight,
momentum or floor) and second directive drive into one `DriveLoadReport` (`AI drives: N error(s)`, AI2 style).
Ids follow **record id order** (not load order), so the tie-break is stable across mod lists. `Curve`: `Polynomial`
(linear at exponent 1), `Logistic`, `Logit`, `Piecewise`; output clamped to 0..=1, NaN gives 0, and a NaN input
(or one past the end of the `inputs` slice) vetoes its drive. Score = IAUS compensated product
(`v + (1 - v)(1 - 1/n) v` per consideration) times the weight. `arbitrate(drives, inputs, prev, tick, trace) ->
Arbitration { drive, reason, state, scores }` is pure: floor lifts an applicable drive (raw > 0), the running drive
gets `x (1 + momentum)`, highest score wins, ties to the lowest id, all zero = idle; a running drive inside
`commit_ticks` holds while its raw > 0. Time is only the logical tick (a coarser tier switches on its first
decision past the same tick). `serving_for(drives, chosen, current)`: a reaction drive gives `Serving::Suspended`;
the directive drive or no drive serves the stack (`Top` after a suspension, a named `Directive(id)` kept).
Trace (AI1, minimally extended): `ConsiderationScore.input` (raw input next to the curve value), `TraceEntry.
drive_reason: Option<DriveReason>` (`best | tie | momentum | commit | floor | idle`, the first rule the winner
needed; skipped in JSON when `None`, so older dumps and `sf_ai_bevy`'s BRP golden keep their shape);
`format_trace` prints `drive d0 (momentum)`. Tests: `tests_drive.rs` (14) + the AI1 goldens updated.

Design, and why (Dave Mark's IAUS, GDC 2013/2015; Game AI Pro 3 ch. 13, Lewis; Guild Wars 2):
- **No per-consideration weight:** IAUS keeps every factor in 0..=1 so the product reads "how true are all of
  these"; a curve's slope and shift already scale it. The drive weight sets the bucket (e.g. 1, 2, 3, 5).
- **Floor only while applicable:** a directive drive whose considerations score 0 (no directive to serve) stays at 0,
  so a weak reaction can win then; with content, a reaction must beat the floor ("clearly beats it").
- **Commit yields to a dead drive:** a committed drive whose own raw score drops to 0 (threat gone) releases at once.
- **Saved by name:** `ArbiterState { running, since }` is serde, but a `DriveId` is a load-time index, so its save
  form is `SavedArbiter { drive: RecordId, since }` (`to_saved` / `resolve`; a drive whose mod is gone resolves to
  none). Not in `sf/ai` yet: that is a v2 step (freeze `ActorSave` in `legacy::v1`, bless `sf.ai.v2.json`).
- Not built: the Bevy system that fills inputs, arbitrates on the AI tick and calls `DirectiveStack::serve`; the
  adapter considerations (Hello, Fight, Flee, Alarm in `mw_ai`); a drive's HTN root task; scratch reuse for the
  `scores` `Vec` (AIM measures first); drive names in `ai.trace` (`TraceNames` has no drive hook yet).

## AI5: the `sf/ai` save section (2026-11-09)

`sf_ai::save`: `AI_SECTION` = `sf/ai`, `AI_VERSION` 1, `AiSave { actors: BTreeMap<ActorKey, ActorSave> }`,
`ActorSave { directives: SavedStack }`, `legacy` (empty: v1 is the first). `SavedStack` (in `directive.rs`) is the
stack minus its actor (the map keys it): entries with served minutes, `next_id`, clock, serving mode;
`DirectiveStack::{to_saved, from_saved, next_id}`. `DirectiveStack` keeps `Serialize` (dumps) but lost
`Deserialize`: the only way in is `from_saved`, which checks the invariants (`StackError`: an entry id the
sequence never issued, a duplicate id, serving an unissued id). `sf_save_bevy::sections::ai(get, set)`: order 350
(after overlay, clock, RPG; before the adapter), `check` refuses a broken stack naming the actor before anything
applies; shape golden `tests/shapes/sf.ai.v1.json`. `sf_ai_bevy`: `AiDirectives` is now a **resource** keyed by
`ActorKey` (was a component on the `AiActor` entity), `ai_save` / `ai_load` are the section's accessors (`None`
when empty, so a world without AI writes no section; load replaces the whole store). Not wired into the game
(no `AiPlugin` there yet). Tests: `sf_ai` `tests_save.rs` (3), `sf_save_bevy` `tests_ai.rs` (3) + the shape
golden, `sf_ai_bevy` accessor test; the four released save goldens now load with `sf/ai` wired.

Design, and why (research: `OpenMW` `esm3/aisequence.cpp` and `mwmechanics/aisequence.cpp` read for behaviour;
UESP Skyrim save format, ACHR change form `CHANGE_ACTOR_EXTRA_PACKAGE_DATA`; Unreal: behaviour-tree state is
transient, games save a high-level AI state and re-run the tree):
- **One world section, not per-placement overlay blobs** (save.md "One path for unload and save" was the
  alternative). Tier 0 keeps every actor logically alive in every worldspace, so the stack is host state that
  never unloads with its cell: one store keyed by stable id, no second copy for streaming to disagree with.
  Blobs would be frozen copies tier 0 can't advance, the stacks of actors with no entity after a load
  (`reload_resident` respawns cells after the apply) would need a holding place anyway, and the overlay's
  one-version-per-owner table refuses older blobs until overlay v2 (C7), while `sf/ai` will step versions often
  (AI10, AI11, AI13, AI14). Actors of a missing mod stay in the store and the section (save.md "Mod-list changes").
- **Saved = what can't be re-derived** (Bethesda's runtime package overrides and package data as change-form
  deltas; `OpenMW` each package's parameters and remaining duration): the stack with its id sequence, so
  `status(id)` answers `done(id)` after a load. `OpenMW` doesn't save `mDone` and loses it; events
  (`DirectiveDone`) aren't saved here either, so AI6/AI7 consumers must read them in the tick they're emitted
  or poll `status`. Not saved (rebuilt): plans, traces, awareness, path requests, wander-like executor state.
- **Base/delta, later:** every stack is saved, record defaults included (~40-60 B per actor before LZ4). When
  MA2 builds stacks from record packages, a stack nothing pushed to since it was built can be skipped and
  rebuilt (as stats' `changed_since_init`); it must not have issued ids beyond the record's, or `done(id)` breaks.
- **Clock:** nothing converts `SkyClock` to `GameTime` yet (the dialogue host passes `GameTime::default()`). Stacks
  ignore readings older than their clock, so a load that put the host clock behind a stack would freeze its
  timers. `sf/clock` and `sf/ai` are snapshotted at the same tick boundary, so this holds if the AI tick's `now`
  is derived from the saved clock and rounded down; which clock wins is the open owner question (three clocks).
  **Owned elsewhere (2026-11-09):** clock consolidation is ROADMAP 1.17, assigned to the mechanics lane; the spec
  lands at [clock.md](clock.md). The AI lane writes no clock conversion: AI4/AI5 directive timers stay blocked on
  1.17, and the AI tick's `now` must come from whatever 1.17 makes authoritative.
- **Keys:** placement and spawn ids are stable (spawn ids come from the saved overlay counter). Flagged:
  `mw_rpg_bevy::dialogue::key` falls back to `Entity` bits for an actor without a placement; such a key must never
  get a stack, or its saved state attaches to nobody (or someone else) after a load.

## AI4: directives (2026-11-09)

`sf_ai::directive` (Bevy-free; new deps `sf_rpg` for `GameTime`, `sf_world` for `Location`/`PlacementId`).
`DirectiveStack` per actor, top first, serde (what AI5 saves: entries, id sequence, clock, serving mode):
`push(spec, now, policy, events) -> DirectiveId`, `finish`, `cancel`, `cancel_where` (a quest stopping: by
`origin`), `serve(Serving)`, `advance(now)`; `status(id)` answers `done(id)` (`serving`/`waiting`/`ended`/`unknown`).
`DirectiveDone { actor, id, end: Completed | Expired | Cancelled, at }`, exactly one per issued id (a push the
policy refuses ends `Cancelled` at once). `StackPolicy { push, on_end }` edits only through `StackEdit` (`insert`,
`push_bottom`, `cancel`, `cancel_where`, `renew`), so ids and events stay exact. `ModernPolicy`: source rank
(record < schedule < memory < script < quest), then priority, newest first among equals (OpenMW's tie rule too);
`repeat` re-pushes a fresh instance on completion or `For` expiry, never on cancel or `Until`. `DirectiveKind`:
`StayNear`, `GoTo`, `Follow`, `Escort`, `Use`, `Fight`, `Custom(RecordId)`. `format_directives`; in
`sf_ai_bevy` the `AiDirectives(DirectiveStack)` component and `ai.directives [id]` (top first, `[serving]`).

Deviations from the illustrative data model, and why (research: OpenMW `aisequence.cpp`/`aiwander.cpp` read for
behaviour; Creation Kit "Package Stack"):
- **Two limits instead of `until`:** OpenMW packages count their remaining duration only while they execute
  (`AiWander` `mRemainingDuration`, game hours), so a quest on top or a fight pauses them; an absolute deadline
  can't express that. `Limit::Until(GameTime)` (schedule windows, served or not) and `Limit::For { minutes }`
  (served minutes, `Directive::served`).
- **`Serving` (`Top` | `Directive(id)` | `Suspended`):** what `For` counts and the drives work on. AI7's
  arbiter sets `Suspended` while a non-directive drive runs (Bethesda: combat suspends package evaluation; OpenMW:
  combat is a package on top) and may name a lower directive when the top can't be served (Bethesda runs the
  topmost package whose conditions hold). A named directive that leaves falls back to `Top`.
- **Exact on one clock:** every mutation takes `now`; `advance` ends expiries in time order at their exact
  deadline (ties top first), so one coarse step equals many fine ones (tested at 1-, 7- and 1000-minute steps):
  tier invariance. Integer game minutes (`GameTime`'s resolution); a clock reading from the past is ignored.
- **Ids are instances:** `DirectiveId` is a per-actor, never-reused sequence (not a load-time index); a repeat
  is a new instance, so `done(old)` holds after one cycle.
- **No `done` flag, no `ParamBag`:** completion is `finish` + the event + `status` (a removed directive's flag
  can't be polled; OpenMW's `mDone` is one sticky flag per sequence, which MA2 keeps from the events). Kinds carry
  their data; OpenMW's per-package options (`canCancel`, `shouldCancelPreviousAi`, ...) are per type, MA2's;
  `origin` names the issuing record. `MAX_REACTIONS` (256) bounds policy cascades and same-instant expiries.
- Not built: the systems that push/advance/finish stacks (AI6/AI7), the Bevy message for `DirectiveDone`,
  filling `TraceEntry::directive` (from `serving()`), `decide`.

## AI3: debug overlay, `sf_ai_bevy` (2026-11-09)

New thin Bevy crate `sf_ai_bevy` (bevy_app/bevy_ecs + `sf_console_bevy`; feature `debug` adds gizmos, camera,
colour, transform and the console overlay). `AiPlugin { timings }` inserts `AiTraces` (the `TraceStore`, tier-2
admission), `AiLibrary` (library + last `LoadReport`), `AiSelected`; `AiActor(ActorKey)` marks an entity the AI
runs. `load_library(world, db, keys, operators)` logs one `error` line per bad record (target `sf_ai`,
`HTN library: sf.ai.method `x`: unknown task `y``, then a count line) and keeps an empty library. Console:
`ai.select [hex|off]` (no id: the console's click selection, `debug`), `ai.trace [n] [hex]` (names through the
library, indices for unknown ids), `ai.tiers` (labelled provisional: last traced step per actor until AI9),
`ai.stats`; `ai.gizmos` (`debug`): ring at the selected actor's feet coloured by the last plan outcome and
`overlay_lines` above its head as Bevy 0.20 stroke-font `Gizmos::text`, facing the active camera. Tier colours,
paths, cones and claims are not drawn (no data until AI6/AI8/AI9/AI14). Enum names print through serde, so
variants `sf_ai` adds later need no change here. Owner check: example `ai_overlay` (AI-1, synthetic trace).

Deviations, and why:
- **BRP without `bevy_remote`:** one `RemotePlugin` per app (the game's `mw_devtools_bevy` owns it) and hakari keeps
  `bevy_remote` out of every other build, so the crate exports `brp_trace(world, params)` /
  `brp_trace_system` for `with_method_main(BRP_TRACE_METHOD = "sf_ai/trace", ..)`.
- **Actor keys as 32-hex strings in BRP**, not `TraceStore::to_json`'s numbers: a 128-bit placement id doesn't fit
  a `serde_json::Value` number or a JavaScript client. The console prints and parses the same hex as `PlacementId`.
- **Not wired into the game yet:** no NPC entity carries `AiActor` and the game app doesn't add `AiPlugin` (needs
  `mw_devtools_bevy`: plugin, `AiActor` from `CharacterPlacement`, the BRP line; a game build, not done here).

## AI2: HTN core (2026-11-09)

`sf_ai::{state, htn}`. `StateKeys` (code-declared, case-folded; same name + kind shares a key, a kind clash
is an error) types every key as `BitKey`/`IntKey`/`RefKey`; `WorldState` is three flat std arrays (no
`SmallVec`/bitset crate: the planner clones the state once per search and undoes effects through a journal).
Records `sf.ai.task` (`Compound` | `Primitive { operator, pre, effects }`) and `sf.ai.method`
(`{ task, priority, pre, subtasks: [Task(..) | Pause] }`) resolve once in `Library::build` against `StateKeys`
and `Operators`; every unknown task, key, operator, kind mismatch or method on a primitive is collected into one
`LoadReport`. Methods sort by priority, ties by record id (load-order independent); a method's rank is its MTR
digit. `decompose` / `resume` / `replan` take `Option<&mut TraceEntry>` (identical result with `None`).

Deviations from the spec's illustrative data model, and why:
- **Effects on primitive tasks, not methods** (Humphreys, Fluid, SHOP): a method effect has no defined time in a
  total-order plan with pause markers. Effects are planning-only (Fluid `PlanOnly`); executors write real state.
- **Full chronological backtracking** (owner 2026-11-09, slice AI2b; SHOP/pyhop depth-first, Humphreys'
  decomposition history, Decima's Prolog-style search; was Fluid's method-local in AI2): every method choice is a
  choice point; a task that can't plan reopens the most recent one with untried methods, even inside an earlier,
  fully planned sibling, restoring world state, plan and method stack through one undo journal. Trace: unwound
  methods `SubtaskFailed(t)`, a reopened planned choice `Backtracked(t)` (`back t` in `format_trace`), one entry
  each. First plan found = best MTR. Worst case exponential: `SEARCH_TRIES` (every backtrack costs a try) cuts it
  off deterministically, tier-invariantly (`Limit`). Opt-out: `MethodDef::cut` (Prolog cut at the method's end;
  `cut` everywhere = method-local). No nogood memo of failed (agenda, state) yet: AIM measures first.
- **MTR rule:** a replan wins only if lexicographically better on the common prefix (equal, or a prefix, is not
  better); the search stops at the first method that can't win (`RejectReason::NotBetter`, Fluid's
  `BeatsLastMTR`). The AI1 open item ("replan discarded") is `TraceEntry::outcome = kept_not_better`, plus
  `TraceEntry::mtr`; `methods`/`plan`/`mtr` describe the plan in force, `rejected` this step's search.
- A pause reached before any step is planned is skipped (nothing to run first). Paused methods are committed:
  `resume` failing means replan from the root. Guards: `SEARCH_DEPTH` 64, `SEARCH_TRIES` 4096
  (`RejectReason::Limit`). Plans record `reads: KeySet` (every key a precondition evaluated, rejected branches
  included) for AI6's dirty-key re-validation; `Library::holds` is the running operator's per-step check.
- Not built: callers set `trigger`; no scratch reuse yet (a search allocates a few small `Vec`s; AIM measures).

## AI1: decision trace (2026-11-08)

`sf_ai::{ids, trace}` (details in the module docs). Entries add a trigger, the served directive,
raw vs. final drive score and `SubtaskFailed` (backtrack) rejections. `TraceStore::record(actor, tick, tier) ->
Option<&mut TraceEntry>` is the off switch, so `decide` takes `Option<&mut TraceEntry>`; the ring recycles entries.
Cap: watched (newest first), then admission order; later actors never evict earlier ones. `budget_us` is `None` with
timings off (reproducible traces). `ai.trace` is registered in AI3; the core exposes `format_trace` and JSON dumps.

## Original build plan, AI0-AI18 (2026-11-08)

Moved from `ai.md` "Build plan"; the roadmap `docs/roadmaps/ai.md` holds the live plan.

| Slice | Deliverable | Test |
|---|---|---|
| **AI0** docs | this spec, nav.md, ai-adapter.md; doc changes after the owner's go | doccheck |
| **AI1** trace | `sf_ai::trace` ring buffer, JSON dump, `ai.trace` (debugger first) | synthetic trace dump exact |
| **AI2** HTN core | records, typed state, decomposition, partial plans, rejections traced | exact decompositions |
| **AI3** overlay | `sf_ai_bevy` gizmos, `ai.select`/`ai.tiers`, BRP | headless smoke; log line |
| **AI4** directives | stack, `StackPolicy`, completion events | exact orders, policy double |
| **AI5** saves | `sf/ai` section | round trip; old saves load |
| **AI6** move executors (**built 2026-11-18**) | `MoveTo`, `Wait`, `Face` over `sf_nav` (then MA3 drops `PathWalkScaffold`) | headless goals reached, no stuck |
| **AI7** drives (**built 2026-11-17**) | arbiter, considerations, curves, momentum | exact scores and switch ticks |
| **AI8** perception | sight, hearing, stimuli, per-player awareness, batched LOS | exact on synthetic scenes |
| **AI9** allocator | tiers 2/1, fixed `sim.cpu_tier` caps, pins; per-actor tier-2 cost in `ai.stats` | exact assignment + hysteresis; cost line logged |
| **AI10** tier 0 | logical state, promotion resolve, `fast_forward` | tier invariance; no in-view promotion |
| **AI11** schedules | records, `ScheduleRules`, quest-safety validator | exact pushes at window edges |
| **AI12** doors | `Traverse` (open, cross, close behind), fade-only "unseen" rule (supersedes seamless SI14; seamless branch on the `sf_portal` core, SI2) | follower through both door modes (seamless on the core double) |
| **AI13** memory | fact store, relations, decay, `MemoryRules`, bounded vendetta encounters, `sf/aimemory` | exact decay, eviction, round trip; vendetta pauses when its target is offline |
| **AI14** smart objects | `UseSlot` + fragments (after the `sf_interaction` spec) | claim conflicts deterministic |
| **AI15** combat | C6 operators, attack tokens, reactions | scripted duel, tokens respected |
| **AI16** net | replicated AI state via `sf_net` (after N2) | loopback sees activity, barks |
| **AI17** Luau | `@sf/ai` (when Lua is unparked) | push/done round trip |
| **AI18** advisor seam | `Advisor`, protocol schema, null advisor, deadline | late answer ignored; saved |
| AI19-AI25, AIM2 | world reactions: concerns, notice drive, discovery by tier, investigate/report, suspicion, tidy, consequences ([world-reactions.md](world-reactions.md)) | there |
| **AIM** measurement | Floor and High cap sweeps ("Tiers and budget"), AI + mover + animation ms together | owner runs; provisional rows filled |

Order: AI1 → AI2 → AI3 → AI4 → AI5 → AI6 (needs NV6, NV7a, NV8; worldspace from `WorldPosOf::location`, DECISIONS 2026-11-17) → AI7 → AI8 → AI9 → AI10 (needs NV12) →
AI11 → AI12 (needs NV11, SI2) → AI13 → AI15 → AI14 → AI16-AI18 later. NV0 runs in parallel with AI1-AI3. The
adapter's schedules pack (MA8) waits for AI10-AI12. AI10, AI12 and NV11 are seamless M9b (they replace SI13-SI15).

## Doc changes for shared files (2026-11-08)

The ready-to-paste DECISIONS bullets and the exact edits for net.md, CONTRIBUTING, streaming.md, action.md,
ARCHITECTURE, ROADMAP (3.9, 3.10) and SYSTEMS.md are in `../research/ai-nav-decisions-2026-11-08.md` "Ready-to-paste
edits for shared docs" (they supersede the list this section held).
