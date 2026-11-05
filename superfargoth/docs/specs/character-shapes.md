# Character shapes: one per-bone shape set per rig (hurt volumes, physics proxies, held and dead bodies)

Status (2026-11-05): design, nothing built. Owner asked for it after PH-15 ("balls go straight through them" while a
Delirium hit plays) and combat C4 (per-bone hurt capsules). Game side: `../../../bevyrrowind/docs/specs/combat-adapter.md`
"Character shapes (Morrowind)". Builds on [physics.md](physics.md) ("Controller", "Ragdolls and the animation blend",
`sf_ragdoll`), [combat.md](combat.md) ("Hit and hurt volumes", "Reactions, ragdoll and Delirium"), [net.md](net.md)
("Hit authority and lag compensation") and [rig-profile.md](rig-profile.md).

## Goal

A character is one coarse mover capsule today; everything finer than that (an outstretched arm, a lying body, a body
another solver holds) is invisible to physics and to weapons. One shape set per rig, cooked once, serves four users:

1. **Hurt volumes** for weapon sweeps (combat C4): a blade that passes an arm outside the capsule should hit it, one that
   passes beside the capsule but misses every limb should not.
2. **Kinematic proxies** in the physics world while the character animates, so thrown balls and falling props bounce off
   arms and legs, not off an invisible cylinder.
3. **Proxies synced to an external active-ragdoll sim** (Bevyrrowind: Delirium) while it holds the body, so a second,
   third, tenth ball keeps hitting and pushing.
4. **Dead bodies**: props rest on a corpse, a hit or a grab can wake it, corpse drag works again.

Non-goals: per-triangle hit tests, physical weapon-vs-world collision (sword clanging on walls), two-way coupling of
the external sim with props (the external sim's world is its own business; see "Open questions").

## Diagnosis that started this (PH-15, 2026-11-05)

Balls pass through a Delirium-held body because nothing of that body is in the physics world. `sf_physics_bevy`
`character.rs` gives a kinematic `CHARACTER` capsule only to `Mover`s `Without<Dormant>`; a takeover inserts `Held`, which
brings `Dormant`, so `capsules()` removes the capsule at the takeover. Delirium's parts live in delirium-physics, not in
Rapier. No body → no `PhysicsEvent::Hit` → no `ImpactOnCharacter` → the walkers' routing for second hits on held bodies
(`DeliriumReaction { live_only: true }`) never runs. Frozen corpses have the same hole (and no grab target: corpse drag).

## Canonical technique

- **UE Physics Asset** (and Unity's ragdoll colliders, Havok's keyframed bodies, Jolt's
  `Ragdoll::DriveToPoseUsingKinematics`): one primitive body per significant bone, cooked from the skeleton and skin.
  While animated the bodies are kinematic, moved to the bone pose each physics step (UE `PhysicsType Kinematic`,
  `UpdateKinematicBonesToAnim`; Jolt `MoveKinematic`); a ragdoll switches the same bodies to simulated. The capsule
  (UE `Pawn` channel) and the bone bodies (`PhysicsBody` channel) are separate, filtered by collision channels.
- **Hitbox / hurtbox split** (fighting and action games, Souls, For Honor; shooters' per-bone hitboxes): hurt shapes are
  query-only, posed from the animation, tested by the attacker's sweep; never solver bodies. Shooters (CS2, Valorant,
  Overwatch) rewind them for lag compensation and test a cheap bounding volume first, per-bone only on a broad hit.
- **Active ragdoll in another solver** (Euphoria in RAGE keeps its bodies in the game's own physics world; when the sim
  is a separate library, the standard bridge is kinematic shadow bodies driven from the sim's parts every step, with
  contacts on them turned into impulses on the matching part: a one-way coupling at the contact point).
- **Corpses**: a sleeping simulated ragdoll (UE, Skyrim, Fallout: bodies asleep cost nearly nothing, props rest on them, a
  hit or grab wakes them), or for big counts a frozen pose with static proxies.
- **LOD and activation**: full per-bone bodies only where they matter (UE physics asset LOD, "only when relevant" kinematic
  updates; GTA/RDR2 activate ragdoll bounds near the player), capsule only elsewhere.

Industry comparison: we match the Physics Asset / hurtbox split; our one gap versus RAGE is that the active ragdoll is not
in our physics world, closed by the proxy bridge. Gap logged in `industry-audit.md` when S1 lands.

## Design requirements (owner 2026-11-05, "how do we make sure it's optimized")

1. **Two-phase hit tests.** Every test (weapon sweep, prop contact gate) runs against the character's bounding capsule
   first; the per-bone narrowphase runs only on a capsule hit. Weapons: segment vs inflated mover capsule, then segment vs
   each bone capsule. Props: the capsule (or an inflated sensor-free bound) decides activation, the proxies take the contact.
2. **Activation tiers with hysteresis** (below): full per-bone proxies only near the player, in combat, held by a physics
   layer, or recently hit; capsule only otherwise. Enter and leave thresholds differ, plus a minimum hold time.
3. **Weapons are shape-cast queries** over the active frames of a move (combat.md "Sweep"), never simulated bodies: no
   weapon body in the solver, no contact pairs from swords.
4. **Collision groups**: proxies interact with projectiles, props, debris, carried items and ragdoll bodies (the solver's
   `MOVING` set) and with weapon queries; never with each other, the static world, terrain, other characters' capsules or
   proxies, or their own mover (mover sweeps never see them: they are not in `GROUND_LAYERS` nor the push filter).
5. **Kinematic position-based proxies synced once per fixed tick** in one batched system (`FixedUpdate`, before
   `PhysicsSystems::Step`): one pass over all active sets, `set_kinematic_target` per proxy, no per-entity systems, no
   inserts or removals inside the tick (activation changes are applied in `Update`, batched).

## Data model

### Shape set (Bevy-free, `sf_ragdoll`; cooked with `ragdoll.cook`)

```rust
pub struct ShapeSet {
    pub skeleton: SkeletonId,
    pub shapes: Vec<BoneShape>,     // parents before children; 0 = pelvis
    pub bound: (f32, f32),          // bounding capsule radius, half height around the rest pose (broadphase, tier 0)
}
pub struct BoneShape {
    pub joint: u16,                 // skeleton joint it follows
    pub capsule: Capsule,           // radius, half (sphere when half = 0), along its local Y
    pub local: (Vec3, Quat),        // relative to the joint's model frame
    pub region: u8,                 // RagdollDef::regions: "core", "head", "arm_l", ...
    pub hurt_scale: f32,            // hurt radius factor (hit and hurt tuned as a pair, combat.md), default 1
}
pub struct JointShapes(Vec<u16>);   // per skeleton joint: the shape that moves it (own or nearest ancestor's)
```

- **One source**: the shape set is `RagdollDef::bodies` (same capsules, frames, regions), so the limp ragdoll, the hurt
  volumes and the proxies agree on where the body is. `ragdoll.cook` already makes about 15 bodies for a humanoid from the
  `RigProfile` (girdles, at most 3 spine bodies, neck+head, one per limb segment, hands and feet merged).
- **Skin fit** (S1): radii from the skinned mesh at bake (per bone, vertices with dominant weight on it, radius = 90th
  percentile of distance to the segment, clamped by the rig fractions), UE's "fit to skin" auto-generation. Adapter or mod
  override files patch any value, as for ragdolls. Runs inside `ragdoll.cook` (seconds; bake rule).
- Bevy-free so the combat core (`sf_combat`) can test hurt shapes without Bevy (build-perf rule).

### Posing

`pose_shapes(set, model_pose, root: &WorldTransform, out: &mut [WorldTransform])`: each shape at its joint's model frame
times `local`, under the root. Pure math, exact tests. Two pose sources, by user:

| User | Pose source | Why |
|---|---|---|
| Hurt volumes (gameplay hits) | baked joint tracks of the playing slot (`SocketTrack` generalised to the hurt joints) at the tick's clock | deterministic from (move id, clock): rewindable for co-op (net.md), no read of the evaluated pose (action.md rule) |
| Physics proxies | the shown model pose (after IK, hit-react springs, physics-pose copy-back) | what the player sees is what the ball hits; host-side presentation coupling only (see "Co-op") |

Hurt from the evaluated pose instead is a fallback for content without baked tracks (procedural NPC moves), flagged in
the log.

## Activation tiers

| Tier | What exists | Who gets it (enter) | Leaves when |
|---|---|---|---|
| 0 capsule | mover capsule only (today) | everyone | — |
| 1 hurt | tier 0 + hurt narrowphase on demand (no bodies) | any character a weapon sweep's broadphase touches | — (stateless) |
| 2 proxies | tier 0 + kinematic per-bone proxies; the capsule stops making contacts | within `near` (6 m) of a player, or in combat (a fighter target or attacker), or hit in the last `recent` (3 s), or an awake dynamic body inside `bound + reach` (1 m + its speed x 0.25 s) | all reasons gone for `hold` (2 s) and outside `near + 2 m` (hysteresis) |
| 2h held | proxies driven from the holder's shown pose | `Held` by a physics layer that registers as a proxy holder (Delirium) | hand-back |
| 3 corpse | see "Corpses" | death at rest | despawn, cell unload |

- Caps: at most `max_proxied` (16 on High) tier-2 sets; over the cap, nearest to a player first; held and combat sets
  never evicted. Presets scale `near` and the cap (Low 8 / 4 m, Ultra 32 / 10 m).
- Tier changes are batched in `Update` (`ShapeSystems::Activate`): inserts and removals of proxy bodies, capsule layer
  switch. The fixed tick only moves bodies (requirement 5).
- Capsule while proxies are on: its body moves to a layer with no contact pairs (`Layer::CHARACTER` kept for queries,
  contact mask cleared), so a ball never stops on air beside an arm; the mover's sweeps, ORCA and pushes are queries and
  are unaffected.

## Physics integration (`sf_physics` + `sf_physics_bevy`)

- **Layer** `Layer::BODY_PROXY`: `collides_with` = `MOVING` (debris, props, carried items, projectiles, ragdolls,
  powered); not `WORLD`, not `CHARACTER`, not `BODY_PROXY`, never `SENSOR`-triggering by default. On
  `PhysicsParams::hit_layers` (so `Hit` events fire), off `contact_report_layers` (the prop side reports sounds). Not in
  `GROUND_LAYERS`, the push filter (`DEBRIS | PROP`) or the carry pick layers (corpses: below).
- **Bodies**: `BodyKind::Kinematic` (rapier position-based), one collider each, `user` = proxy index. Inserted at the
  posed shapes, CCD stays on the prop side (thrown props have it; S3 tests that a fast ball does not tunnel through a moving thin limb).
- **Sync** (`FixedUpdate`, `ShapeSystems::Sync` before `PhysicsSystems::Step`): for each active set, the target for this
  step's end = the last posed shapes extrapolated by their per-shape velocity to the step's time (the capsule's AN-7
  fix: the pose comes at frame rate, the step at 60 Hz; extrapolating avoids the 0/1/2-frame beat). The kinematic
  velocity this gives is what `impact_impulse` subtracts, so an arm swinging into a resting ball is no hit.
- **Component** (Bevy): `BodyProxies { set: Arc<ShapeSet>, poses: Vec<WorldTransform>, velocities: Vec<Vec3>, holder:
  ProxyHolder }` on the character root, written in `Update` after the shown pose is final (the animator's output; for held
  bodies after the holder's copy-back). `ProxyHolder::{Animation, Layer, Corpse}` only routes impacts.

## Impacts and how they map to parts

- `Hit` on a proxy → `ImpactOnCharacter { entity, impulse, point, direction, shape: Some(u16) }` (new field; `None` from
  the capsule). Per step, one impact per (incoming body, character): the strongest, so a ball touching forearm and chest at
  once pushes once. Every new contact start is a new impact, so rapid balls each count; there is no per-frame dedup per
  character for held bodies (today `knockdown()` keeps one hit per entity per frame: kept for tier 0/2, lifted for 2h).
- Routing, by holder: **Animation** → the game's hook as today (`PhysicsTakeover` gains `joint: Option<u16>`, the shape's
  joint) and the cosmetic `HitReact` op gets `joint: Some` (it already takes one). **Held** → the holder's own entry
  (Bevyrrowind: `Takeover::hit` on the part owning that joint; adapter maps joint → part). **Ragdoll** (sf_ragdoll
  bodies in the solver) → no proxies: its own bodies take contacts. **Corpse** → below.
- The prop side: proxies are kinematic, so the prop bounces (restitution of the prop) or stops; the push on the character
  is the prop's incoming momentum x the game's gain (combat-adapter "Ragdoll routing"). One-way coupling, the standard for
  a shadow-body bridge; two-way (a prop's weight resting on a held body) is an open question.
- Weapons (C4): the move's sweep segment vs the defender's bound capsule (inflated by the weapon radius), then vs each hurt
  shape (`segment_capsule_distance <= r_weapon + r_shape * hurt_scale`); the contact carries `shape` → region (head,
  limbs: rules may scale damage and pick the reaction side), and the point → the takeover's part for physics reactions.

## Corpses

Preferred (S6): the corpse at rest becomes a **sleeping limp ragdoll** in the solver from its frozen pose
(`sf_ragdoll` `Limp`, bodies inserted asleep at the posed shapes, velocities zero): props rest on it, a hit wakes it, the
carry pick includes its bodies (corpse drag works as it did before 3.2b), sleeping bodies cost almost nothing. Fallback
when the owner wants the external sim's look kept: **static proxies** (kinematic bodies with no new targets) at the
frozen pose, `Hit` ignored or (later) waking a limp external takeover. Count cap `max_corpse_sets` (32), oldest frozen to
pose-only (no bodies) past it. Save/streaming persistence of the pose is the existing gap (physics.md "Death": the
quantized `ragdoll` blob) and stays a separate slice.

## Mover capsule: its role next to the shapes

Locomotion collision (collide-and-slide against `STATIC | PROP`, terrain), ORCA radius, pushes (queries), tier-0 contact
body for far characters, the broadphase bound for weapons and for proxy activation, and the hit target for characters
without a shape set (shapeless creatures, LOD crowds). It is never replaced by the proxies, only muted for contacts while
they are on.

## Co-op (net.md)

- Host-authoritative: proxies exist only on the host (it owns NPCs, props and the external sim); clients see replicated
  prop and body state. A client's own prediction never touches proxies.
- Weapon hits: the host tests the attacker's sweep against hurt shapes posed from baked tracks at the rewound tick
  (net.md stores per-tick (`WorldPos`, facing, move id, clock), so per-bone hurt costs no extra history). Held and dead
  bodies have no track: their hurt shapes come from the replicated shown pose ring (host-side, 250 ms of parts at 20 Hz,
  per held body only).
- Proxies read the shown pose: presentation feeding prop physics, allowed because it is host-only and props are not
  predicted; gameplay hits never read it (they use baked tracks).

## Budget and measurement

Target hardware: RTX 4060 laptop, Ryzen 7 7435HS. Budgets are CPU ms per frame at 60 Hz fixed (one step per frame),
median, all shape work together (activation, posing, sync, solver cost added by proxy pairs, hurt narrowphase):

| Preset | Proxied sets (cap) | Budget | Crowd beyond (capsule only) |
|---|---|---|---|
| Low | 8 | ≤ 0.15 ms | 150 |
| High | 16 (20 combat characters: 16 proxied + 4 hurt-only) | ≤ 0.3 ms | 150 |
| Ultra | 32 | ≤ 0.5 ms | 150 |

Rough model to check (S0 measures it): ~15 proxies per set, a kinematic move + broadphase update ~1-2 µs each →
16 x 15 x 1.5 µs ≈ 0.36 ms worst case before pairs, so the hysteresis and the "awake body nearby" reason matter: most
proxied sets in a quiet town have no body in reach and could skip the sync (a set with no `MOVING` body inside
`bound + reach` keeps tier 2 but its sync drops to every 4th tick, targets extrapolated; full rate the tick a body
enters). Hurt narrowphase: ~15 segment-capsule tests per broad hit, negligible (< 5 µs per swing).

- **Bench scene** `level-shapes-bench` (synthetic, `levels/shapes-bench.ron`): 20 walkers in combat range of the player
  (fighting or idling), 150 walkers in a crowd beyond 30 m, a ball launcher throwing a 3 kg ball every 0.25 s at the
  near group, 4 bodies held (Delirium on the game side), 8 corpses. Runs with `--bench` (perf.md "Suite runner"), one
  row per preset.
- **Log-line counters** (once per second, `shapes:` prefix, and in the bench JSON): `sets` by tier (0/1/2/2h/3), `proxies`
  (bodies in the solver), `pairs` (proxy contact pairs this step), `impacts`, `sync_us`, `activate_us`, `hurt_tests`,
  `evicted`. No-picture checks only (CONTRIBUTING "Tests and visual checks").
- **S0 is the measurement slice**: the bench scene + counters + an `#[ignore = "measurement: ..."]` headless bench in
  `sf_physics_bevy` (N kinematic capsules synced, M balls) printing µs per proxy. Every later slice reports its row
  against S0's baseline in perf.md (dated log line) and in its commit message.

## Build plan

One deliverable + its test per slice. Tests: logic exact (posing, routing, tiers), physics sanity (finite, no tunnelling
through the floor, comes to rest), tuned numbers as `#[ignore = "measurement: ..."]`.

| Slice | Deliverable | Test |
|---|---|---|
| **S0** measurement | bench scene `shapes-bench`, `shapes:` counters, headless proxy bench | bench runs headless, counters finite; measurement prints µs/proxy |
| **S1** shape set | `ShapeSet` from `RagdollDef` + skin fit in `ragdoll.cook`, `JointShapes`, overrides | humanoid ~15 shapes, guar set, all finite; exact joint→shape map; measurement: skin coverage % |
| **S2** posing | `pose_shapes` (model pose and baked tracks) | exact transforms vs hand-computed bones |
| **S3** held proxies (PH-15) | `Layer::BODY_PROXY`, `BodyProxies` sync, `ImpactOnCharacter.shape`, holder routing; Delirium registers as holder (game side) | ball at a held body → impact → second push (pelvis moves further along the second hit); proxies within a few cm of the shown pose; gone after hand-back |
| **S4** animated tier 2 | activation tiers + hysteresis + caps, capsule contact mute, joint → `HitReact` / takeover part | ball at an outstretched arm hits; ball beside the capsule but past the limbs passes; tier flips counted, no thrash |
| **S5** hurt volumes (combat C4) | two-phase sweep in `sf_combat` against hurt shapes, region in the contact | blade past the capsule into an arm hits; blade beside the body misses; exact distances |
| **S6** corpses | sleeping limp ragdoll from the frozen pose (or static proxies, owner pick), carry pick | ball rests on a corpse; drag a limb; no explosion at wake |
| **S7** co-op | hurt shapes from rewound tracks; held-body pose ring | replay determinism of hurt contacts |

S3 first: it fixes the owner's PH-15 complaint and proves the bridge; S0 lands with or before it so S3 reports a number.

## Open questions

1. Corpses: sleeping limp `sf_ragdoll` (drag and wake for free, but the look switches solver at rest) or static proxies
   over the frozen Delirium pose (look kept, no drag until a limp Delirium wake exists)?
2. Two-way coupling for held bodies: props pressing on the sim's parts (a crate falling on a lying NPC). Needs the
   external sim to see Rapier bodies (the Delirium "copy the Rapier statics" step); out of scope here.
3. Player's own proxies (third person: balls hitting the player's arms; first person: none?), and the training dummy
   (action character, no mover).
4. `near` / caps per preset are guesses until S0 measures them.

## Owner answers (2026-11-05)

1. Corpses at rest, unconscious and paralysed bodies, and dragged bodies are the existing Rapier ragdoll; Delirium only
   drives responsive bodies (a killing hit plays Delirium's dying window, then hands over to the Rapier ragdoll). S6
   becomes "Delirium → Rapier ragdoll handover"; corpse drag and wake come back with it.
2. Two-way coupling for held bodies: out of scope (impacts still push; only sustained pressure on a living held body is
   Owner: if it ever matters (something heavy resting on a living held body), the way out is to make the character
   unconscious (hand over to the Rapier ragdoll).
   not simulated).
3. Player (recommendation, owner to confirm): hurt volumes when NPCs fight (C6); physics proxies and Delirium on the player stay a later call.
4. Radius and caps: set from S0's measurements.
