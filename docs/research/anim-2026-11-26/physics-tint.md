# Physics tint on Delirium: what the source can do and what it costs (research half of AN38)

Research only, 2026-11-26. Delirium = `delirium-physics-no-game` at `cb8ccec` (path root `D = C:\Users\leogloglo\.cargo\git\checkouts\
delirium-physics-no-game-1da9a93fe3b92dbc\cb8ccec`). Our crates = `bevyrrowind/crates/...`.

## Summary

1. **Recommendation:** build the seam `PhysicsTint` as a short, strength-ramped *pose-follow* on a Delirium agent, not as
   today's takeover. Today's takeover never follows the clip: it starts `bodyBalance` (or `catchFall`), the balancer
   stands the body its own way, and the root is `Held`. A tint needs a new, lighter path.
2. **Partial drive exists in the engine.** Per joint (20 joints) the effector has desired angle, strength (s^2) and
   damping. The message masks `fb ub ul ur ll lr us un uk uw uc` pick regions, and `setStiffness`, `bodyRelax`,
   `injury` change strength at any tick. `activePose` + the public `Agent.incoming` follow an animation pose.
   Our integration uses none of it (grep: no `activePose`, `incoming`, `setStiffness`, `stumble`, `injury` in `mw_delirium*`).
3. **Precedent for "upper body follows, legs balance":** the `splitBody` extra (balance on legs, `activePoseMask` on
   spine and neck). Whether `bodyBalance` plus `activePose` on arms and spine coexist for a walking pose is not verified.
4. **Disturbance inputs** exist: `applyImpulse`, `leanInDirection`, `stumble`, `staggerFall`, `bodyBalance` with
   `armsOutOnPushTimeout` (the playroom "drunk"), `injury` (limp), `electrocute`. Stand-in numbers are low-confidence.
5. **Cost:** measured 0.50 ms/step (bodyBalance) and 0.78 ms/step (takeover) per body at 60 Hz (ledger 2026-11-19);
   20-30x over the 25 us animation budget. No number exists for pose-follow states. A measurement list is in section 4.
6. **No concurrency cap exists**; one sequential loop in `FixedUpdate`. The tint needs its own cap, shared with deaths
   and knockdowns, which must win.
7. **Start cap 4 concurrent tints (Guess)**, gate A0/A1 plus on-screen plus significance; fallback additive clip.
8. The prior claim (cap 4-8, ragdoll far over budget) is confirmed by a number, but the cost of the *tint states* is
   still unmeasured.

## 1. Partial drive

- **Joint model.** 21 parts, 20 joints (`mw_delirium/src/body.rs:24,32`; `docs/research/.../12-delirium-physics.md:61`).
  Per joint `eff[j]` holds `desired`, `strength = s*s`, `damping = 2*s*d`, muscle stiffness
  (`D/crates/delirium/src/nmrs.rs:492-498`). `Character::set_stiffness(s, d, mask, ms)` writes all joints of a mask
  (`nmrs.rs:703-707`). Mask grammar `fb` whole, `ub` upper, `ul ur` arms, `ll lr` legs, `us` spine, `un` neck, `uk`
  spine+neck, `uw` wrists, `uc` clavicles (`nmrs.rs:648-670`; `docs/how-it-works.md:~213`).
- **Runtime change.** `setStiffness` is a one-shot, usable any tick (`msg.rs:350`, one-shot list `msg.rs:633`).
  `bodyRelax(mask, relaxation 0 stiff..100 limp, damping, holdPose)` (`nm.rs:~510-535`). `injury(part/mask, amount 0..1)`
  weakens a limb, 0 heals (`extras.rs:42`; `docs/modding.md` "Extra messages: general"). So a strength ramp is just a
  message per tick, no restart.
- **Following an animation pose.** `Agent.incoming: Vec<Transform>` is public (`nm.rs:338`), 7 floats per part in the
  C API (`nm_agent_set_incoming_transforms`, `docs/modding.md` "Reading and hitting a character"). The `activePose`
  message copies each masked joint's desired angle from it (`nm.rs:506-513`); it is a one-shot, so it is re-sent every
  tick: the engine's own pose task does `setStiffness` + `activePose` each control step (`nmpose.rs:40-53`). This is a PD
  joint-angle follow only. The pelvis is not driven, so whatever is not balanced must be held up (see 3.).
- **Other follow tools.** `hardConstraint` pins chosen parts to the supplied pose (`extras.rs:33`, `docs/modding.md`
  "Extra messages: general"). `softKeyframe` chases a pose captured at its first tick, not a live clip
  (`extras_held.rs:1138-1148`), with chase strength scaled by joint strength. `incomingTransforms` teleports parts to the pose
  (`nm.rs:515-531`), the hard version.
- **Per-region example (code-backed):** `splitBody` takes `activePoseMask` (default `uk`), `stiffness`, own legs
  balance (`extras_held.rs:1196-1213`; `modding.md` "held, carried, posed").
- **Not verified:** that `bodyBalance` leaves arm and spine `desired` alone while `activePose` writes them for a
  *moving* clip; the balancer owns arms for `armsOutOnPush`. A 30-line headless test settles it (section 4).
- **Our integration today:** `Takeover::start` sends only `bodyBalance` or `catchFall` (`takeover.rs:316-376`), then
  `push`, `knock_down`, `kill`, `relax` (`takeover.rs:388-481`). Rules are hand-back rules, not drive rules.

## 2. Disturbance inputs

| Wanted | Input | Where | Note |
|---|---|---|---|
| Stagger after a medium hit | `applyImpulse` on a part with balancer on | `takeover.rs:388-398`, `ledger/physics.md:24` | in use; >= 10 N s is a push, >= 100 N s a forced fall |
| Stumble steps | `stumble` (legs 12, torso 10, arms 10, `staggerTime` 1.5 s, `leanTowards`, `useArmsBrace`) | `extras_fall.rs:22-46` | arm reduce, wrist, feet, fallMask, grab are "not done" |
| Stagger then fall | `staggerFall` (arm/spine stiffness 12 -> 6/8 over 1 s after 0.5 s, legs 12 -> 3) | `extras_fall.rs:48-64` | the stiffness wind-down is the exhausted-collapse shape |
| Drunk sway | `bodyBalance` + `armsOutOnPushTimeout` + `leanInDirection(leanAmount)` | `delirium-room/src/main.rs:3396`, `learned.rs:171` | timeout is kv slot 190 = 2.2, "low", real value unknown (`kv-own.txt:150`) |
| Sag from exhaustion | lower `setStiffness` / `bodyRelax(mask ub)` | `msg.rs:350`, `nm.rs:510` | cheap, no new behaviour |
| Wounded limp | `injury(mask ll, amount)` | `extras.rs:42` | per-leg weakness |
| Twitch | `electrocute` | `modding.md` | not a goal |
| Lean push | `leanInDirection`, `leanToPosition` | `modding.md` core table | one-shot |

All exist in the fork (`docs/no-game.md:47`: drunk, stagger "hunches and holds, but less deeply", flinch, electrocute
checked by the fork's author). Numbers are estimates (549 stand-in slots; 189 "low"): feel is ours to tune, not the
original's. `stumble` and `staggerFall` carry `used: false` params, so some arms/feet behaviour is simply missing.

## 3. Short tints in our integration

Start (`mw_delirium_bevy/src/lib.rs:428+`, module docs 3-50): a `DeliriumReaction` on the root; **two posed frames**
later the pose becomes parts (`bones_to_parts`), `Takeover::start` builds a `Collision` floor, a `Ragdoll`, an `Agent`,
sets leg stance (`takeover.rs:316-376`), starts the behaviour. The root goes `Held`/`Dormant` (no step, no motion).
Refused when the character has no `Mover` or is not a human skeleton (`lib.rs:462`, docs line 47).
Time floors from `Rules::default()` (`takeover.rs:~120-148`): `min_secs` 0.5 s, recover when calm 0.15 s, `blend_out`
0.5 s, `blend_in` 0.1 s. A tiny push therefore lasts ~0.65 s to the start of blend-out and ~1.15 s to hand-back
(computed from those rules, not measured).
End: `HandBack::Balanced` at weight 0 -> the graph carries on, no inertialization; timeout/blow-up or a fall ->
`inertialize_from` over `HAND_BACK_BLEND` 0.3 s (`lib.rs:101-105,823-864`). State handed back: root moved by the pelvis
floor position, `Mover::snap` (ledger 2026-11-19), `DeliriumHandedBack` message, `knocked_out` flag for falls.
Known artefacts (ledger `physics.md:16-27`): one-frame jump (fixed), knee sink (fixed), crouched landing slide ~1.5 s
(logged), proxies needed for held bodies.

**Fit for 0.5-2 s:** time yes, kind no. The body is `Held`, so a walking NPC freezes in place in the sim and the
shown pose is the balancer's standing, not the clip. A tint must (a) keep the root and graph alive, (b) feed the clip
pose each tick via `incoming` + `activePose` with a strength ramp, (c) show the physics pose as a `PhysicsPose` at a
weight (the same read-back path, `lib.rs:769-806`), (d) end by ramping strength to full follow, then dropping the
agent, so the hand-back has nothing to blend. (b) and the root-follow part are new; the rest reuses the takeover.

## 4. Cost per body

- **Measured:** `bodyBalance` 0.50 ms/step, takeover 0.78 ms/step (ledger `physics.md:17`, 2026-11-19, no-game fork,
  from `tests/real_install.rs:331-347` and `tests/takeover_real_install.rs:229-239`). Older: 0.41-0.57 ms/step, release,
  shared CPU (`12-delirium-physics.md:351`). Fixed tick is 60 Hz (`actors.rs:412`), Delirium's native rate; substeps
  per step = `round(dt*kv27*SUBSTEPS)` clamped 1..8 (`nm.rs:2258-2262`; SUBSTEPS not read). Bodies: 21 parts, 20 joints.
- **Per-frame meaning:** 0.5 ms of a 8.3 ms frame is 6% per body on the sim thread, once per tick. Four bodies are
  ~2 ms per tick. The 25 us A0 animation budget is exceeded 20-30x. The live gauge `DeliriumStats.step_us/peak_step_us`
  (`lib.rs:278-290,628-649`) already sums all bodies per tick.
- **Not measured:** pose-follow states, 4/8 bodies, start and end cost, parallel step. None exist ("not found" in
  the ledgers for `physics.md` and `perf.md`, grep Delirium/ragdoll/takeover).
- **AN38 measurement test (ignored test next to `takeover_step_time`, release, owner not playing):** time
  `Takeover::step` or `Agent::step` over 600 steps after a 60-step warm-up, report mean/median/p99, for: (A) `bodyBalance`
  baseline; (B) balance + `setStiffness(ub, 4)`; (C) `bodyBalance` legs + `activePose` mask `ub` with `incoming` set
  every tick; (D) `stumble`; (E) `staggerFall`; (F) whole-body `setStiffness(12)` + `activePose fb` (the pose task);
  (G) `hardConstraint` on the pelvis plus (F). Each with N = 1, 4, 8 agents stepped in one loop, and N = 8 through
  `Agent::step_group` (`nm.rs:1296`). Also time `Takeover::start`-style construction (`Ragdoll::new`, `Agent::new`) and
  a drop. Also record: fall rate within 2 s, and whether `bodyBalance` overwrites `activePose` angles (C).

## 5. Concurrency today

No cap found after searching `mw_delirium_bevy/src/lib.rs` (`max_`, `cap`, `limit`): only `refused` for non-movers and
non-humans. `step` is one sequential loop over `takeovers.live` (`lib.rs:628-650`), single thread. Many hits mean many
bodies and linear cost; corpses stop stepping once settled (`takeover.rs:~700`, `lib.rs:871-889`). The SDK has a rayon
`par_chunks_mut` multi-agent step (`delirium-sdk/src/lib.rs:1010-1018`) and the engine a `step_group` that we do not
call; thread safety of the `kv`/`nmrs` globals (one asset per process, `physics.md` wiki line 62) is unverified.

## 6. Industry

| Them | What we can do with Delirium | Gap |
|---|---|---|
| Euphoria (GTA IV/V, RDR2): behaviours triggered by game events, physics leads, then clip hand-back; Rockstar says it is "enhanced" and evolved in RDR2 ([wccftech](https://wccftech.com/rockstar-euphoria-evolved-rdr2/)). Concurrent count: not found publicly. | Same engine lineage; shots, stumble, stagger exist. | We have it only as full takeover; no clip-follow driver. |
| UE5 Physical Animation Component: per-body profile (strength, drive), applied "below body"; component strength multiplier fades 0-1; separate mesh physics blend weight ([profile docs](https://dev.epicgames.com/documentation/unreal-engine/applying-a-physical-animation-profile-in-unreal-engine), [component API](https://dev.epicgames.com/documentation/unreal-engine/API/Runtime/Engine/UPhysicalAnimationComponent)). Docs do not state how multiplier and blend weight stack. | masks + `setStiffness` per tick = profile; `PhysicsPose.weight` = blend weight. | We must write the profile table (tunables) ourselves. |
| UE hit reaction recipe: find hit bone, simulate it and children, ramp blend weight up and down ([physics-driven animation](https://dev.epicgames.com/documentation/unreal-engine/physics-driven-animation-in-unreal-engine)). | `injury`/`setStiffness` on the hit region mask. | Region from our `PartTable` pick (`lib.rs:244`). |
| Frostbite powered ragdoll (Sachania, GDC 2018), Zordan 2005 and others (from our research doc `12-delirium-physics.md:79-94`; titles not re-fetched). | clip target + strength dips = our plan. | Recovery pose matching only exists for get-ups. |
| Naughty Dog (Uncharted 4, TLOU2), shipped melee powered ragdolls | **not verified**: web search found no checkable GDC source. | n/a |

## 7. Recommendation for AN39

- **Seam `PhysicsTint` (framework, game-agnostic):** request = `{ kind, region mask, strength curve, duration, optional
  impulse, priority }`; kinds: `Stagger`, `Stumble`, `Sway` (drunk), `Sag` (exhaustion), `Limp` never physics.
  Region is an enum (upper, arms, legs, spine, whole), not Delirium masks. States: `Requested`, `Active`, `Ending`
  (ramp to full follow), `Done`, `Refused(reason)`. Ends by time, by recovery (still, balanced) or by a hit that
  promotes it to a knockdown (hands over to the existing takeover). The game fills it from `mw_delirium_bevy`.
- **Gate:** tier A0 or A1, on screen, within a distance tunable, and significance (target in combat, in dialogue,
  or the player's focus); Classic preset: never. Deaths and knockdowns preempt the oldest tint.
- **Cap:** start with 4 concurrent (Guess), counted together with live non-corpse takeovers; reasoning: at ~0.5-0.8 ms per
  body (measured for balance) four bodies are 2-3 ms per tick, which is the most a 8.3 ms frame can spare; revise from
  the AN38 numbers. Above the cap or refused: the additive clip fallback (authored hit/lean additive) at the same
  start time, so the player sees the same event.
- **Stay "Guess" until measured:** the cap; per-region strength numbers; ramp times; the minimum tint length
  (takeover floors are 0.65 s, a tint may be shorter); distance and significance thresholds; drunk `armsOutOnPushTimeout`
  and lean amount (stand-in slots); tick-rate share (60 Hz fixed vs lower for far tints); whether a pelvis support is needed.

## Not verified

SUBSTEPS constant; thread safety of Delirium globals; that `activePose` coexists with `bodyBalance` on a moving clip;
the build profile of the ledger's 0.50/0.78 ms numbers (the doc says release, shared CPU); any Euphoria concurrency
number; Naughty Dog and Frostbite sources; how UE's strength multiplier and blend weight combine.
