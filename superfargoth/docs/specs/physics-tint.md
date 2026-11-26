# Physics tint: short physics-shaped motion over animation

Status (2026-11-26): spec (AN39), nothing built. Rows AN38-AN42 in `docs/roadmaps/anim.md` Stage 6 (decision 10). Research:
`docs/research/anim-2026-11-26/physics-tint.md` (R§n). Game side: `../../../bevyrrowind/docs/specs/physics-adapter.md`
"Physics tint on Delirium (AN39)". Related: [animation.md](animation.md) "LOD and budget", [combat.md](combat.md)
"Reactions, ragdoll and Delirium", [physics.md](physics.md) "Ragdolls and the animation blend".

**Agent summary.**
- A tint: for 0.3-3 s the game's active-ragdoll provider follows the clip at lowered strength and the shown pose blends
  toward its bodies per region; root, graph, ops and gameplay stay the animation's. Kinds `Stagger`, `Stumble`, `Sway`,
  `Sag`; a baseline limp or drunk gait is never a tint (decision 10). Medium and Large combat hits ask for a `Stagger`
  over their reaction clip (owner 2026-11-26); creatures and beast races play the fallback (no second provider).
- Framework (`sf_anim::tint`, `sf_anim_bevy::tint`): requests, gate, cap, states, envelopes, fallback. Provider (game):
  bodies and `PhysicsPose`.
- Gate: preset, capable, not held, tier <= A1, distance, cooldown, cap. The cap counts tints plus live takeovers;
  takeovers are never refused and preempt the lowest tint. Refused: the kind's fallback on the same tick; Classic: always.
- Every number is a tunable, most `Guess` until AN42. Measured (AN38a-d): a pinned tinted body costs 0.65 ms per tick
  stepped alone and 0.40 in Delirium's group step, which splits across cores from 6 agents; the cap is 4 (Low 2),
  resting on one run; agents come from a pre-built pool (building one costs 5 ms). The provider must hold the pelvis
  (pose follow alone collapses); a tint that falls is promoted to the takeover. Every kind runs on the pinned pelvis and holds the clip within ~10 degrees mean on the upper body (AN38c,
  "Performance").

## Goal

A guard takes a medium or large blow and gives around it (spine folds, arms swing loose), back on his clip half a second
later; a
wounded man's leg buckles mid-stride; a drunk lurches; an exhausted fighter sags. Each sits on what the character is doing
(a walker keeps walking): no freeze, no pop, no foot slide. Rare: a few bodies near the camera; everyone else shows an
additive version of the same event on the same tick. Non-goals: pushes, knockdowns, deaths (full takeovers); the player,
creatures and beast races (owner answers 2-3, "Open questions"); wall contacts; physics state over the network.

## Today (verified 2026-11-26)

- **Full takeovers only,** through the game resource `RagdollTakeover` (`mw_walkers_bevy/src/physics.rs:341`, kinds
  `:295-309`), filled with Delirium for human walkers. combat.md:341's `sf_action` `PhysicsTakeover` has no type in
  `superfargoth/crates` (grep): no framework seam exists yet.
- **A takeover is not a tint** (R§3): the root goes `Held`/`Dormant` (`mw_delirium_bevy/src/lib.rs:3-7`), the balancer
  stands the body its own way, floors `min_secs` 0.5 s and `blend_out` 0.5 s (`mw_delirium/src/takeover.rs:123-137`) make
  the shortest hand-back ~1.15 s. Delirium's partial drive (per-joint strength, region masks, pose follow) is unused (R§1).
- **Hit bands on a human walker:** kinematic `HitReact` from 1 N s (`HIT_REACT_IMPULSE`, physics.rs:274), Delirium push
  takeover from 10 N s, push + knockdown from 100 N s (`HitRules`, lib.rs:153-174).
- **Step:** `FixedUpdate` 60 Hz, one sequential loop, no cap (lib.rs:626-649); each takeover steps its own agent
  (`mw_delirium/src/takeover.rs:625`), Delirium's `step_group` is never called; gauge `DeliriumStats.step_us`.
- **Physics into animation:** `PhysicsPose { pose, weight, weights }` (`sf_anim_bevy/src/lib.rs:713-735`) goes to
  `GraphInstance::set_physics` (`sf_anim_graph/src/instance.rs:592-602`), blended per joint in local space as the graph's last
  stage, after the ops (`:1032-1049`); history keeps the animated pose (`:996-1006`). `GraphOutput::pose` (`:239-241`) is
  the blended result, so a follow target must be taken before the blend (AN40a adds it).
- **Tiers:** `AnimLod { tier, forced, significance, pixels }` (`sf_anim_bevy/src/lod.rs:111-133`); A1 runs feet but not
  `LookAt` (A0 only).
- **Cost:** 0.50 ms/step (balance) and 0.78 ms/step (takeover) per body (ledger physics.md, 2026-11-19). Tint paths:
  "Performance".

## Reuse check

1. **Bevy 0.20:** no physics, no physical animation (our graph is `sf_anim_graph`). We use its `FixedUpdate`, messages,
   components and observers.
2. **Crates (searched 2026-11-26):** no maintained Bevy active-ragdoll crate (`bevy_animation_graph` lists joint-motor pose
   matching as planned, `bevy_dynanim` is not production ready); `bevy_rapier3d` has joint motors only, which our
   `sf_ragdoll::powered` already turns into stable-PD pose follow (`sf_ragdoll/src/powered.rs:1-13`). Delirium (adopted
   2026-11-19, Euphoria lineage) has per-joint drives, masks, pose follow and disturbances: the provider for humans.
3. **Our own:** the seam (scheduling policy no crate offers) and per-kind profiles, shaped like `sf_ragdoll::react::Timeline`
   (`react.rs:160-198`) without depending on it (`sf_anim` stays physics-free). `sf_ragdoll::powered` is not a planned
   second provider (owner 2026-11-26, answer 3).

## Industry comparison

Canonical technique: **physical animation (powered ragdoll)**: bodies PD-driven toward the keyframed pose, with per-body
strength and a blend weight of the simulated pose over the animation, both ramped by game events (Zordan and Hodgins,
*Motion capture-driven simulations that hit and react*, SCA 2002). The capsule/root stays kinematic.

| Them | Us | Gap |
|---|---|---|
| UE5 Physical Animation profile below a bone + physics blend weight ramped for hits; capsule authoritative [1] | per-kind profile, envelope; root gameplay's | profile is RON |
| Euphoria (GTA IV/V, RDR2): game messages start behaviours, physics leads [2] | same lineage; tint follows, falls go to the takeover | no physics-led tint |
| Uncharted 4 (Mach, GDC 2017): animation drives physics predictably [3] | clip is always the target | - |
| Jedi: Fallen Order (Waszak, GDC 2020): motors over authored melee reactions [4] | `Stagger` over the Medium and Large clips | wall contacts |
| Frostbite (Sachania, GDC 2018): driven ragdolls, cost cuts, networking [5] | cap + gate; local presentation | - |
| Havok powered ragdoll controllers; PhysX articulation implicit-PD drives [6] | Delirium per-joint PD | - |

Not built, deliberately: our own solver; root-moving tints; learned controllers; A2/A3 tints; physics replication; a
second provider for creatures.

[1] dev.epicgames.com/documentation/unreal-engine/physics-driven-animation-in-unreal-engine and .../applying-a-physical-animation-profile-in-unreal-engine;
[2] wccftech.com/rockstar-euphoria-evolved-rdr2; [3] gdcvault.com/play/1024087; [4] 80.lv/articles/gdc-2020-talk-on-physical-animation-in-star-wars-jedi-fallen-order;
[5] gdcvault.com/play/1025210; [6] nvidia-omniverse.github.io/PhysX/physx/5.4.1/docs/Articulations.html.

## Design

**Changes from R§7.** A request names a kind and an intensity; the curve is the kind's profile (UE's named profiles).
The simulated pelvis is held on the animated one and the root is never physics (UE's "simulate below the pelvis"), so no
balancer is needed. R§1's open point (balanced legs under a followed upper body, for `Sway`) is settled by AN38c: the
balancer does not overwrite the followed pose (2.3 degrees mean at 0.5 s) but the body does not hold it (8-10 mean,
27-34 max from 1 s on) and its legs are the balancer's, never the clip's. So every kind uses the pinned pelvis.
**The pelvis hold is a requirement, not an option (AN38a):** whole-body pose follow with neither a pinned pelvis nor a
balancer sinks to the floor (pelvis at 14 % of standing height after 2 s): pose follow drives joint angles only, nothing
carries the weight. Every tint body is either pinned at the pelvis or stands on a balancer, never neither.

### Types (Bevy-free, `sf_anim::tint`)

```rust
pub enum TintKind { Stagger, Stumble, Sway, Sag }
pub struct TintRegions(u8);              // SPINE, NECK, ARM_L, ARM_R, LEG_L, LEG_R; UPPER, LEGS, WHOLE unions
pub enum TintPriority { State, Injury, Hit }   // ascending; takeovers rank above all, outside this enum
pub struct TintImpulse { pub point: WorldPos, pub impulse: Vec3 /* N s, world */, pub joint: Option<u16> }
pub struct TintRequest {
    pub kind: TintKind, pub regions: TintRegions /* empty: the profile's */, pub intensity: f32 /* 0..1 */,
    pub hold_s: Option<f32> /* state kinds, <= max_s */, pub impulse: Option<TintImpulse>,
    pub direction: Vec3 /* lean heading, or zero */, pub priority: TintPriority, pub tick: u64, pub seed: u32,
}
pub struct TintEnvelope { pub attack_s: f32, pub hold_s: f32, pub recover_s: f32, pub release_s: f32, pub weight: f32, pub dip: f32 }
pub struct TintProfile { pub envelope: TintEnvelope, pub regions: TintRegions, pub legs_weight: f32, pub impulse_gain: f32,
                         pub fallback: TintFallback }
pub enum TintFallback { Additive { slot: String }, HitReact { gain: f32 }, None }
pub struct TintSet { pub stagger: TintProfile, pub stumble: TintProfile, pub sway: TintProfile, pub sag: TintProfile } // RON
pub struct TintSample { pub weight: f32 /* physics share */, pub strength: f32 /* 1 = on the clip */ }
pub fn sample(e: &TintEnvelope, intensity: f32, t_s: f32, end: Option<f32>) -> TintSample;   // pure in time
pub enum TintState { Requested, Active, Ending { from: f32, release_s: f32 }, Done(TintEnd), Refused(Refusal) }
pub enum TintEnd { Time, Recovered, Promoted, Preempted, OffScreen, Unloaded, Failed }
pub enum Refusal { Preset, Unsupported, Held, Tier, Distance, Cooldown, Cap }
pub fn gate(req: &TintRequest, g: &GateInput /* "Gate order" fields */, s: &TintSettings) -> Result<(), Refusal>;
impl TintBook {                                    // per root: tint, start tick, cooldown
    pub fn request(&mut self, root: u64, req: TintRequest, g: &GateInput, s: &TintSettings) -> Result<(), Refusal>;
    pub fn takeover_started(&mut self, root: u64, takeovers: usize, s: &TintSettings) -> Vec<(u64, TintEnd)>;
    pub fn end(&mut self, root: u64, why: TintEnd, s: &TintSettings);
    pub fn advance(&mut self, tick: u64, dt: f32, s: &TintSettings) -> Vec<(u64, TintState)>;
}
```

`sample`: strength falls from 1 to `1 - intensity·(1 - dip)` over `attack_s`, holds, rises back to 1 over the last
`recover_s` of the hold; weight rises from 0 to `intensity·weight` over `attack_s`, holds to the hold's end, falls to 0
over `release_s`. Strength is back at 1 before the release starts, so the bodies are back near the clip (within the
provider's tracking error, "Performance") while the weight leaves, and the weight reaches 0 continuously: no pop. Legs
use `weight·legs_weight`; the pelvis joint is always weight 0.

### Bevy layer (`sf_anim_bevy::tint`) and the provider contract

```rust
#[derive(Message)] pub struct TintRequested { pub root: Entity, pub request: TintRequest }
#[derive(Message)] pub struct TintEndRequested { pub root: Entity, pub kind: TintKind }   // the state cleared
#[derive(Message)] pub struct TintFallbackPlay { pub root: Entity, pub kind: TintKind, pub fallback: TintFallback, pub tick: u64, pub intensity: f32 }
#[derive(Component)] pub struct TintCapable;                   // provider: it can tint this character
#[derive(Component)] pub struct ActiveTint { pub kind: TintKind, pub regions: TintRegions, pub sample: TintSample,
                                             pub impulse: Option<TintImpulse>, pub direction: Vec3, pub ending: bool }
#[derive(Component)] pub struct TintTarget(pub Vec<Trs>);       // pre-physics pose after the ops, tinted roots only
#[derive(Message)] pub enum TintFeedback { Ready(Entity), Failed(Entity) }
#[derive(Resource, Default)] pub struct PhysicsBodies { pub takeovers: usize }   // live, stepping, non-corpse
#[derive(SystemSet)] pub enum TintSystems { Admit, Provider, Advance }    // FixedUpdate, chained
pub struct TintPlugin { pub set: Handle<TintSet> }                       // + TintSettings resource ("Tunables")
```

The provider is a plugin, not a trait object (it needs ECS queries). Contract: insert `TintCapable`; in
`TintSystems::Provider` ready bodies for an added `ActiveTint` from its pre-built pool (`Ready` once they step), step them toward `TintTarget` at
`sample.strength`, apply `impulse` once, write `PhysicsBodies.takeovers` every tick, drop bodies when `ActiveTint` goes,
send `Failed` on NaN or blow-up; every frame write the root's `PhysicsPose` (bodies as a local pose under the live root,
interpolated between steps, per-joint `weights` from the regions). It holds its simulated pelvis on the animated pelvis
(required, see above), never moves the root or the mover, never sets `Held`. `Failed` comes from the provider's own check
(NaN, a part below the floor, the pelvis off its target by more than the provider's limit): a balancer's "fallen" flag
is not set on a pinned body (AN38a: it read false on a collapsed one). **A fall is a promotion, never a tint:** when a
tinted body that stands on a balancer (not pinned) goes down, the provider raises the game's takeover claim on that root
the same tick (state table: Done(Promoted)), reusing the built bodies: a body on the ground costs twice a standing one
(AN38b, 0.56 -> 1.1-1.2 ms) and a tint has no get-up. The framework owns the rest.

### States and transitions

| From | Event | To |
|---|---|---|
| Requested | gate passes (same tick) | Active; `ActiveTint` inserted |
| Requested | gate fails | Refused(reason); `TintFallbackPlay` this tick; cooldown starts (not on `Cooldown`) |
| Active | same kind again (a Large hit on a running `Stagger`) | stays Active: impulse re-applied, intensity = the higher of the two, hold extended (total <= `max_s`) |
| Active | other kind, higher priority | stays Active with the new profile, envelope restarted from the current sample |
| Active | hold time over / `TintEndRequested` | Ending (Time / Recovered), release from the current weight |
| Active, Ending | takeover claim on this root (knockdown, death, push band, the provider on a fall) | Done(Promoted): the takeover starts from the shown pose |
| Active, Ending | a takeover elsewhere over the cap (lowest score) | Ending over `preempt_release_s`, Done(Preempted) |
| Active | tier A2; A3 or off-screen | Ending; Done(OffScreen) at once (not visible) |
| any | root despawned or `Animator` removed (streaming unload) | Done(Unloaded); bodies dropped the same tick |
| Active, Ending | `TintFeedback::Failed` | Done(Failed): `inertialize_from` the shown pose over `abort_blend_s` |
| Ending | weight reaches 0 | Done; `ActiveTint` removed; cooldown starts |

Until `Ready` the envelope clock waits (weight 0) and no fallback plays, so a Stagger's first 1-2 frames are the clip alone.

### Gate, cap, ranking

- **Gate order:** `enabled` (`Preset`), `TintCapable` (`Unsupported`), not `Held` (`Held`: the hit goes to the takeover),
  `AnimLod.tier <= max_tier` (`Tier`; A0/A1 imply on screen), camera distance <= `gate.max_distance_m` (`Distance`), no
  cooldown (`Cooldown`), `active + takeovers < cap` (`Cap`).
- **Cap:** tints not Done plus `PhysicsBodies.takeovers`. Takeovers are never refused; one starting over the cap preempts
  the lowest score = (priority, `pixels·significance` of `AnimLod`). A new request never preempts a running tint (no churn).
- **Hysteresis:** a running tint continues while within `max_distance_m + distance_hysteresis_m`; every Done or Refused
  starts `cooldown_s` on that root, so a body never flickers between tint and fallback.

### Hits: bands and combat tiers

A physics impulse (N s) on a capable character: below `band.min_ns` the `HitReact` kick; up to `band.max_ns` a `Stagger`
(refused: the kick); from `band.max_ns` the game's takeover hook (push, its knockdown threshold above). No overlap.
A combat reaction asks by its tier (owner 2026-11-26: Large hits get the tint too, when a slot is free):

| combat.md `Tier` | Tint | What always plays / fallback |
|---|---|---|
| Additive, Small | none | `HitReact` springs; the Small clip |
| Medium | `Stagger`, regions from the struck joint (else UPPER), priority `Hit` | the Medium forced clip with its root motion; refused: + `HitReact` kick |
| Large | `Stagger`, the profile's regions (UPPER), priority `Hit`, intensity set by the game (physics-adapter.md) | the Large forced clip, its root motion and windows; refused: + `HitReact` kick |
| Knockdown, Death | none: takeover; a running tint is Promoted | the takeover |

The reaction clip stays the gameplay move (windows, root motion, its own length) either way: the tint runs its envelope
over the clip's start and never stretches or cuts it. A Medium or Large on a root with a running state or injury tint
takes it over (state table, higher priority); nothing but a takeover ends a tint early. The cap bounds the cost
whatever the tier.

### Pose flow per tick and frame

`FixedUpdate`: host triggers send `TintRequested` -> `Admit` (gate, cap, preemption, fallback) -> `Provider` (build, step,
drop) -> `Advance` (envelopes, ends, cooldowns). Envelope time = fixed ticks since `Ready`, the same on every machine. Per
frame: graph -> inertialization -> ops (`FootPlacement`, `LimbIk`, `LookAt`, `HitReact`) -> copy to `TintTarget` ->
physics blend from `PhysicsPose` -> bones. The bodies follow the post-op pose, so look-at and IK are in the target; legs at
weight 0 keep plants exact; the target arrives one tick late. At Done the weight is 0: no inertialization and no root
move, so no hand-back jump and no `Mover::snap` (ledger physics 2026-11-19).

### Net-ready

A tint is presentation and never feeds gameplay; its trigger is a host event on the fixed tick. Each machine gates locally
(own camera, tiers, cap); a refusal plays the fallback, no gameplay difference. The root is gameplay-authoritative (UE's
capsule; Sachania [5]): knockback and stumble steps are gameplay root motion (reaction clip, mover), never the bodies.
Hurt volumes and gameplay queries read the animated pose or capsule, never the tinted one (AN40a checks; logs a bug if not).

## Tunables

All `anim.tint.*`, in `TintSettings`/`TintSet`, live-editable; `Guess` until the named measurement replaces it.

| Name | Unit | Range | Default | Source / replaced by |
|---|---|---|---|---|
| `enabled` | bool | - | per preset | decision 10 (Classic off) |
| `cap` | bodies | 0-16 | 4 | measured AN38d, one run (group step: 1.61 ms median at N = 4; "Performance"); AN38e and AN42 confirm; also the agent pool's size |
| `max_tier` | tier | A0-A1 | A1 | decision 10 |
| `gate.max_distance_m` | m | 0-40 | 15 | Guess (A0 range, animation.md); owner check |
| `gate.distance_hysteresis_m` | m | 0-10 | 3 | Guess |
| `cooldown_s` | s | 0-5 | 1.0 | Guess |
| `preempt_release_s` | s | 0.05-0.5 | 0.15 | Guess |
| `abort_blend_s` | s | 0.1-0.5 | 0.3 | today's `HAND_BACK_BLEND` (lib.rs:102) |
| `max_s` | s | 0.5-6 | 3 | Guess |
| `step_divisor` | ticks | 1-4 | 1 | Delirium's native 60 Hz (R§4); raised if AN42 is over budget |
| `band.min_ns` | N s | 1-30 | 10 | measured: today's `HitRules::takeover` (lib.rs:154-157) |
| `band.max_ns` | N s | 10-100 | 25 | Guess; AN38: impulse where a push first needs a catch step |

Per kind (`<kind>.attack_s`, `.hold_s`, `.recover_s`, `.release_s` in s, 0-3; `.weight`, `.dip`, `.legs_weight` in 0-1;
`.impulse_gain` x, 0-2), all Guess, replaced by the owner check ANIM-14 (look) and AN38 (stability at the dip):

| Kind | attack | hold | recover | release | weight | dip | legs | impulse_gain |
|---|---|---|---|---|---|---|---|---|
| Stagger | 0.05 | 0.35 | 0.25 | 0.3 | 0.8 | 0.3 | 0 | 0.5 |
| Stumble | 0.1 | 0.6 | 0.3 | 0.4 | 0.7 | 0.4 | 0.3 | 0.3 |
| Sway | 0.3 | 1.0 | 0.4 | 0.6 | 0.6 | 0.5 | 0 | 0 |
| Sag | 0.4 | 1.5 | 0.4 | 0.6 | 0.7 | 0.35 | 0.25 | 0 |

44 framework tunables. Fallback slot aliases (`tint.fallback.<kind>`, an additive layer slot) are content; an unmapped
alias plays the `HitReact` kick.

## Presets and the classic seam

Classic: `enabled` false, every request `Refused(Preset)` with its fallback; no bodies, no cost. Low: `cap` 2; Medium to
Ultra: 4 (the cost is CPU and the same on every preset; measured, "Performance"; a faster CPU raises it through the
tunable). The seam is the request: classic rules may send none.

## Performance

Fixed 60 Hz under a 120 fps target: one tick per ~2 frames, one per worst frame (two on a catch-up). Measured by
`mw_delirium/tests/tint_cost.rs` (release; ledger physics.md "AN38a" to "AN38d"). Tables per state, pose-error numbers
and how Delirium's split works: [history](archive/physics-tint-history.md).

**Budget: tints + takeovers at the cap <= 2.0 ms median and <= 2.7 ms p99 per tick** (2.7 ms = the frame's p99 11 ms
minus its median 8.3 ms; physics share, not the 25 us animation budget).

**Per body, stepped alone (AN38a/b):** the tint's path G (whole-body pose follow, pinned pelvis) costs 0.65 ms, linear
in N (median / p99 1.28 / 2.13 ms at N = 2, 2.54 / 3.62 at 4); pose follow with no pin collapses (pelvis 14 %); a body
on the ground costs twice a standing one (0.56 -> 1.1-1.2 ms), hence the promotion rule.

**Group step (AN38d, one clean run: these numbers rest on it).** G through Delirium's `Agent::step_group`, median at
N = 1 / 2 / 4 / 8: 0.46 / 0.83 / 1.61 / 0.84 ms; p99 at 4: 2.24 in one run, 4.0 in the other; at 8: 1.1. Up to 5 agents
it runs on one thread (0.40 ms per body); from 6 Delirium splits, one task per agent on its own pool. The threshold
is a constant in Delirium's data table, not a setting (history).

**So the provider steps tints and takeovers together through `step_group`** (one shared collision world, each agent in
its own shifted frame, as the test does), and **the cap is 4** (Low 2): 1.61 ms at N = 4 meets the median; its p99 is
unsettled and N = 3, 5, 6, 7 are not timed; N = 5 is ~2.0 ms by the serial cost, at the limit. Past the split 8 agents
cost less than 4, so a cap of 8 needs the counts 3-5 to split too: a provider-side fan-out (one task per agent on
Bevy's `ComputeTaskPool`, each calling the agent's own step; no Delirium change) or a one-constant patch in our pinned
fork. Neither is measured: AN38e times N = 3-7 grouped, the p99 at 4 again and the fan-out at 2-5; AN42 repeats the
cap in the game, where Delirium's pool shares the cores with Bevy's. Rule: the cap is the largest N with every count
up to N inside both limits. Over budget: lower `cap`, then `step_divisor` 2; never delete the feature. Takeovers are
never refused, so the cap only stops tints adding to them.

**Start cost (AN38d): a tint never builds its agent on the frame it starts.** Building one agent costs 5.1-5.4 ms
median (6.9 max), over half a frame; dropping it 0.26-0.32 ms. The provider keeps a pool of `cap` pre-built agents,
built off the frame (a worker task, at load and whenever the pool is short); a tint or a takeover takes one and poses
it, a finished one goes back; a promotion keeps its agent and the pool refills. An empty pool refuses the tint
(`Refused(Cap)`); a takeover then builds as today. Shipped engines do the same: ragdoll bodies exist with the
character and are switched to simulation on the hit (UE's physics asset bodies; Jolt's `Ragdoll::AddToPhysicsSystem`).
AN40b checks that a pooled agent can be posed for any human walker (U) and prints the re-pose cost.

**Pose error (AN38c,** static follow, degrees mean / max): G holds ~10 / ~20 on the upper body and 4.8 / 12.5 on the
legs, steady from 0.5 s to 2 s (undriven: 19-20 / 47). At strength 1 a tinted body is near the clip, not on it: the
weight ramp, not the strength, makes the hand-back popless, and the standing offset is part of the look ANIM-14 (1)
judges. AN40b prints it per kind on a walking clip. Still to time: the catch-step impulse (`band.max_ns`).

## Tests

- **Exact (AN40a, `sf_anim` without Bevy, plugin with a fake provider):** `sample` at key times; every state-table row;
  gate order; 6 requests at cap 4 give 4 Active, 2 `Refused(Cap)` and 2 `TintFallbackPlay` on the same tick; a takeover
  over the cap preempts the lowest score; cooldown; refresh clamp; promotion; unload; Classic refuses all; same inputs on
  the same ticks give the same states.
- **Sanity (AN40b, headless, real install):** per kind no NaN, no part below the floor, the pelvis still at standing
  height 2 s into an undisturbed tint (the AN38a collapse), weight back to 0 and bodies
  dropped within the envelope + 1 s, root untouched. Tracking error is printed (`#[ignore = "measurement: ..."]`). No
  automated visual checks, ever.

## Verification (owner visual checks)

**ANIM-14**, the game's scene `anim-tint` (walking and idle NPCs, console triggers per kind; registered by AN41b, location
in physics-adapter.md). Yes/no: (1) After a medium hit, does the upper body give and return to the clip with no pop?
(2) Does a wounded stumble read as a leg giving way, not a glitch? (3) Does the sag read as tired, and does a further drain
fall into the knockdown with no jump? (4) With six NPCs hit at once (cap 4, so two play the fallback kick), do those
two look acceptable beside the four tinted, with no flicker? (5) In Classic, are the same events there without physics? (6) Do light pushes (10 N s to `band.max_ns`) read
better as a stagger than as today's full push takeover? (`band.max_ns` = 10 is today's behaviour, the A/B arm.) (7) After
a large hit, does the upper body give over the Large clip without fighting it, steps and timing unchanged? (The game's
Large intensity at 0 is the clip alone, the A/B arm.)

## Slices

| ID | Deliverable + test | Crates | Depends | Verify |
|---|---|---|---|---|
| AN38 | measurement test (list in "Performance"); a (A, B, F) 39628390, b (C, D, E, G, N = 2) 37041235 and c (pose error) 8663e3a7 done; d (group step, start cost) measured, one run; e new: N = 3-7 grouped, p99 at 4 again, provider fan-out at 2-5 | `mw_delirium` tests | - | `cargo test -p mw_delirium --release -- --ignored`; ledger |
| AN40a | seam: types, `sample`, `TintBook`, plugin sets, `TintTarget`, fallback messages; exact tests with a fake provider | `sf_anim`, `sf_anim_bevy` | AN39 | `cargo test -p sf_anim -p sf_anim_bevy` (exact cap) |
| AN40b | Delirium provider: pinned pelvis for every kind, one group step, pre-built agent pool; sanity tests | `mw_delirium`, `mw_delirium_bevy` | AN38, AN40a | `cargo test -p mw_delirium -p mw_delirium_bevy` (no NaN, floor, rest) |
| AN41a | hit triggers: Medium and Large tiers and the impulse band -> `Stagger` | `mw_combat`, `mw_walkers_bevy` | AN40b, AN22 | `cargo test -p mw_combat -p mw_walkers_bevy` (exact) |
| AN41b | state triggers: injury, alcohol, fatigue -> `Stumble`, `Sway`, `Sag`; scene `anim-tint` | `mw_walkers_bevy` | AN41a | exact tests; owner ANIM-14 |
| AN41c | `Sway`/`Sag` fallback key-pose clips (data) | `mw_combat` data | AN41b, AN36 | bake loads them; ANIM-14 (5) |
| AN42 | measurement: tint ms at the cap, each kind and mixed with takeovers | `mw_delirium_bevy` tests | AN41b | `#[ignore = "measurement"]`; ledger perf |

AN38c settled R§1's open point ("Design"): AN40b builds no balanced lower body. Later, only once a per-part print
explains C's drift: `Sway` on balanced legs (provider data).

## Open questions

1. (Answered, owner via meta-5, 2026-11-26: yes.) A Large hit also gets the `Stagger` over its clip when a slot is free;
   the clip keeps the timing and stays the fallback; the cap bounds the cost ("Hits").
2. (Answered, owner via meta-5, 2026-11-26: no.) The player's own body gets no tint in third person before AN42 has
   measured cost and control feel; the provider refuses action characters today.
3. (Answered, owner via meta-5, 2026-11-26: fallback only.) No second physics provider for creatures or beast races;
   they play the additive fallback. Quadruped reactions come later through a procedural animation system (WISHLIST 8,
   parked: no spec, no research until the owner says go).
