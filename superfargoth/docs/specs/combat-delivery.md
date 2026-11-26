# Combat: delivery (projectiles, beams, areas, aim modes, cast slot; C8, C14)

Status (2026-11-26): spec C0c, nothing built. Split from [combat.md](combat.md) (size); it adds three `source`s and a few
fields to that spec's pipeline, never a second one. Order and slices: [combat roadmap](../../../docs/roadmaps/combat.md)
Stage 5 (decisions 1, 5, 11, 13, 15, 17, 19; owner answers Q3, Q4). Effects contract: [effects.md](effects.md) "Casting",
"Events", E5. Hurt shapes: [character-shapes.md](character-shapes.md). Research: `../../../docs/research/gameplay-2026-11-25/combat.md`
§3. Ledger `../../../docs/ledger/combat-feel.md` (nothing ranged built or tried).

## Goal

Bows, thrown weapons and spells fly, arc, stick, burst and channel through the same contact order, rules and `Outcome` as a
sword: delivery is the framework's (`sf_combat::delivery`, Bevy-free), effects are the adapter's (`sf_effects` and its
content). Aiming is a per-view input profile; the core sees an aim direction and never the view. Host-authoritative and
deterministic; clients draw a cosmetic copy.

Non-goals: physics-simulated arrows (no rigid bodies in flight, no Avian: one physics, rapier queries only); VFX and spell
visuals (`bevy_hanabi` 0.20, effects lane); effect semantics, magnitudes, resistances (effects.md, adapters); ammo
inventory (adapter); homing and piercing (open questions); view-aware AI (decision 19).

## Sources

A hit row's `source` (combat.md "Moveset sheet") grows from `Sweep | Cone` to `Sweep | Cone | Projectile(ProjectileKey) |
Beam(BeamKey) | Area(AreaKey)`; the row's window is the **release** window (first tick = release tick). Each source makes
`Contact`s with the same fields; step 6 ("The per-tick pipeline") is unchanged:

| Source | Makes a `Contact` when | `point`, `direction` | `strength` |
|---|---|---|---|
| `Projectile` | its swept segment first touches a hurt shape this tick | touch point, flight direction | release strength (draw charge) |
| `Beam` | a pulse ray's first hurt-shape hit | hit point, beam direction | 1 per pulse (row `mv` is per pulse) |
| `Area` | a candidate is inside the radius with line of sight | nearest hurt-shape point, centre → target | `falloff(d / radius)` |

**Order** (new set `CombatSystems::Delivery`, after Sweep, before step 6): spawn this tick's releases and launches, step
live projectiles by id, then bursts, ground areas and beam pulses; all contacts of the tick go to step 6 sorted by
`(source rank: Sweep/Cone 0, Projectile 1, Beam 2, Area 3, Environment 4; origin id; defender id)`. Projectiles carry the
attacker as `Contact.attacker` (stand-in `Fighter` if the owner is gone, like Environment) and `credit` = owner.

## Projectiles (C8a)

- **State** (host, serde, `ProjectileSet { next: ProjectileId(u32), live: BTreeMap<ProjectileId, Projectile> }`):
  `Projectile { id, spec: ProjectileKey, owner: CharacterId, credit: Option<CharacterId>, pos: WorldPos, vel: Vec3,
  born: Tick, row: HitPayload, payload: Option<Delivery>, passed: SmallVec<[CharacterId; 2]>, bounces: u8 }`.
  `HitPayload` is the release row resolved at release (move key, hit index, strength, `mv`, `kind`, flags, `poise`,
  `damage`, `push`, `buildup`), so later sheet patches never change a flying arrow. `payload` = an effects `Delivery`
  (spells, enchanted ammo). Bevy entities exist only for visuals, spawned from the spawn event on host and clients alike
  (one presentation path).
- **Spec** (content record kind `Projectile`): `ProjectileSpec { radius, speed: (min, max) /* by strength */,
  gravity_scale, drag /* 1/m, quadratic */, life_s, on_impact: Stick | Bounce { restitution, max } | Break | Burst(AreaKey),
  surfaces: Vec<(SurfaceId, Impact)>, on_block: Break | Drop, stick_depth_m, visual: TagId }`.
- **Integrate** per fixed tick (dt = 1/60), semi-implicit Euler in this order: `vel += (g × gravity_scale − drag × |vel| ×
  vel) × dt; p1 = p0 + vel × dt`. One segment per tick is exact enough: a parabola's sagitta over one tick is `g dt² / 8`
  = 0.34 mm, and the segment covers the full displacement, so nothing tunnels at any speed (100 m/s = 1.67 m per tick).
- **Characters**: broadphase = segment against each fighter's `ShapeSet.bound` (+ radius), linear scan, skipping the owner
  for `owner_grace_s` and `passed`; narrowphase = segment (radius) against the posed hurt shapes (S5, the same
  `HurtShape::touch` primitive as blades), in the target's frame: `p0` against its pose at t0, `p1` at t1 (relative motion,
  as melee sweeps). Earliest touch along the segment wins; ties to the lower `CharacterId`.
- **World**: `PhysicsWorld::probe(STATIC | TERRAIN | PROP | RAGDOLL).cast(&ProbeQuery::sphere(p0, dir, len, radius))`,
  composed with `TerrainProbe` through `Nearest` (physics.md "Queries: `PhysicsProbe`"); `GroundHit.surface` picks the
  impact, `GroundHit.body` names a prop or a corpse. No new query API.
- **Fate** (read in step 8 from the contact's `Outcome`): landed → `on_impact` (Stick to the hit shape's joint at
  `(joint, local offset)`, Break, Burst); `guarded` → `on_block`; i-framed → the defender joins `passed`, the projectile flies
  on and may still take a world hit behind it this tick; world hit → the surface's impact (stone breaks, wood sticks, the
  content's table), a prop also gets a C14 contact and an impulse through `sf_physics` (host). Out of `life_s`, beyond the
  active physics world or its cell unloaded → `Expired`.
- **Stuck**: a stuck projectile leaves `live` for `StuckSet` (`(anchor: Joint { who, joint, local } | World { pos, normal }
  | Body { body, local }, spec, since)`). Joint anchors follow the hurt shape, and a ragdoll follows the same shapes
  (character-shapes "one source"), so an arrow stays in the corpse. Caps: `max_stuck` per world and `max_stuck_per_body`,
  oldest removed first; `stuck_life_s`; despawned with its cell; not saved. The adapter may turn a stuck or dropped
  projectile into a pickup (`StuckEnded` event; Morrowind recovery is the adapter's).
- **Events**: `ProjectileSpawned { id, tick, owner, spec, origin, vel, strength }`, `ProjectileEnded { id, tick, at, reason:
  Hit | Stuck | Broke | Burst | Blocked | Expired }`, `StuckEnded { id, reason }`.
- **Determinism**: integer ids from the host counter, `BTreeMap` order, sorted contacts, no `Entity` and no thread RNG;
  same command stream, same projectile bytes.

## Ranged sheet (C8b)

- **Draw / charge** on the move (a `Heavy`-like hold, combat.md C3b): `MoveSpec.draw: Option<Draw { min_s, full_s, release:
  Hit(n) }>`; holding `Light` past `min_s` nocks, releasing fires with `strength = clamp((held − min_s) / (full_s − min_s))`
  (speed and damage scale); released before `min_s`: no shot, ammo kept (Skyrim, ER). `Guard` or `Dodge` cancels the draw.
- **Muzzle**: the release socket's baked track at the release tick (deterministic, like blades), never the evaluated pose.
- **Aim**: the core reads one aim direction from the command: `Command.aim: Option<WorldPos>` (new input field, C8b), the
  point to hit; direction = point − muzzle; `None` = `view`. The profile (`sf_action_bevy` / `sf_camera_bevy`, never
  `sf_combat`) fills it:
  - **third person, locked**: the lock target's core shape, led by the lead solver (C8e) × `aim.lock_lead_frac`;
  - **third person, over the shoulder** (aim held or drawing unlocked): shoulder camera + reticle; the point is a camera
    trace (`ProbeQuery::ray` from the camera through the reticle, `aim.trace_max_m`), so the muzzle shoots at what the
    reticle covers (camera-to-focus, as Lyra); a point closer than `aim.min_point_m` falls back to the camera direction;
  - **first person**: the same camera trace from the crosshair.
- **Moving while drawing** (owner Q3): `MoveSpec.move_scale` multiplies locomotion while the upper-body move runs (bows and
  small spells `draw.walk_scale`; big spells 0 = rooted); no sprint while `move_scale < 1`.
- **Ammo** is the adapter's: `can_start` refuses without ammo (combat.md rules seam), `Released { owner, move_key, tick }`
  tells the adapter to spend one; ammo damage and on-strike enchantments come from its rules at resolve and its `payload`.

## Area and Beam (C8c)

- **Area** `AreaSpec { radius, falloff: None | Linear { inner } | Smooth { inner }, los: bool, hits_owner: bool, row:
  HitPayload, placement: AtImpact | AtPoint { range_max } | OnSelf }`. `AtImpact` = a projectile's `Burst`; `AtPoint` =
  ground reticle: the aim point clamped to `range_max` (no hit inside it: a down ray at the clamped point); `OnSelf` = nova.
- **Overlap**: candidates whose `ShapeSet.bound` lies within `radius` of the centre, capped at `area.max_candidates`, sorted
  by `CharacterId` (effects.md "Area"); `d` = distance to the nearest posed hurt shape; with `los`, a `ProbeQuery::ray` over
  `STATIC | TERRAIN | PROP` from the centre lifted `area.los_lift_m` to the target's core shape must be clear.
  `falloff(u = d / radius)`: None 1; Linear 1 up to `inner`, then `1 − s` with `s = (u − inner) / (1 − inner)`; Smooth `1 − s²(3 − 2s)`. Each passing
  candidate gets one `Contact { source: Area, strength: falloff }`; i-frames and guard (direction from the centre) apply.
- **Beam** `BeamSpec { range_m, radius, rate_hz, row }`: while its hold move runs, every `60 / rate_hz` ticks (phase from
  the move start) one `ProbeQuery::sphere` along the aim plus the hurt-shape test (as a zero-time projectile); first hit
  makes a contact; `BeamPulse { owner, tick }` lets the content charge its pool per pulse and end the beam (a `Cancel`).

## Cast action (C8d)

- **Clips**: upper-body `cast.<sheet>.<move>` over locomotion (as draw/sheathe), timed on the action clock; Morrowind's
  `SpellCast` groups (target, touch, self) are the adapter's slots. New class `ActionClass::Cast` (sf_action vocabulary,
  C8d); the key and the readied-spell stance are effects A5b's.
- **Release tick** (the cast move's release window): touch spells first resolve their target (a `touch.reach_m` sphere
  probe along the aim against hurt shapes), then the shell queues `EffectOp::Cast { source, target, rolls }` (A5a host
  path). The same tick, `EffectSystems` (after combat) answers: self entries land; `Touch { target, delivery }` lands on
  the touch target; `EffectRequest::Launch(Delivery)` comes to `sf_combat_bevy`, which asks the content
  `launch_spec(&Delivery) -> Option<LaunchSpec { projectile: Option<ProjectileKey>, area: Option<AreaKey>, beam:
  Option<BeamKey> }>` and spawns at the next tick's `Delivery` set from the stored release muzzle and aim (one tick of
  latency, effects.md "answers return as ops next tick"). `CastFailed` (fizzle) spawns nothing.
- **Small vs big** (owner Q3): `MoveSpec.move_scale` (small `cast.small_walk_scale`, big 0); which spells are big is the
  adapter's (combat-adapter.md, C0d).
- **Interruption**: the cast move's windup carries `Armor(scale)` (cast armour, combat.md "Poise"); a poise break inside it
  cancels the cast before release: no `Cast` op, no cost (DD2: stagger interrupts the chant); after release it is spent.

## The effects contract (`EffectRequest::Launch`)

As built in `sf_effects` (E3): `EffectRequest::Launch(Delivery { source, class, caster, entries, rolls })` carries the
cast's target-range entries; `Delivery::inflict()` makes the `EffectOp::Inflict` for one target.
- **Flight is combat's**: the `Delivery` rides `Projectile.payload` (or the area or beam) until it lands; effects never
  move anything.
- **When `Inflict` runs**: once per target whose contact has `Outcome.deliver` (new field: default `landed && !parried
  && !guarded`; the rules may override, e.g. classic spells ignore guards). Queued as an `EffectCommand` in the contact's
  tick; `EffectSystems` run after combat, so it lands that tick.
- **Areas**: entries with `area > 0` land on the area's passing candidates within that entry's own radius (effects.md
  "Area": candidates sorted by stable id). **Falloff scales the combat packet only** (`Contact.strength`); effect entries
  land at their rolled magnitude (OpenMW behaviour). A magnitude scale on `Inflict` is E5's to add if content wants it.
- **Buildup**: `Outcome.buildup` goes out as `(target, status, amount, credit)` (combat.md "Damage packet"); projectile
  credit = its owner.
- **Reflect**: effects' `Reflected` re-inflicts on the caster directly; no projectile flies back (OpenMW). On-strike
  enchanted ammo: `ItemCast::OnStrike` from the landed outcome (E5).

## Friendly fire (owner Q4)

Per-source policy data, `FriendlyFire { spare_allies, spare_owner }`; `Contact.defender_state.allied` is filled by the
content (companions, faction, party) like `unaware`; a spared contact is skipped before i-frames, the pair stays unspent
(a blade or arrow passes through).

| Source | Allies | Owner |
|---|---|---|
| `Sweep`, `Cone` (melee) | spared | n/a |
| `Projectile`, `Beam` | hit | spared for `owner_grace_s`, then hit (bounces) |
| `Area` (explosions, fire) | hit | hit (`hits_owner`, default on) |
| `Environment` | hit | hit |

**Co-op switch** `combat.delivery.coop_friendly_fire` (host setting, default off, owner Q4): off skips every contact whose
attacker and defender are both players; Environment still hurts, but a player's credit on another player is dropped.
Classic content may set melee `spare_allies: false`.

## Ranged fighters (C8e)

- **Band**: ranged picker rows (combat-npc.md "Move picker") use `band: (min_m, max_m)` from the projectile's reach
  envelope; inside `min_m` the row is illegal and the fighter backs off (`Wait { ring_m: min_m + ring_margin_m }`) or a
  melee row wins.
- **Pool**: bows, thrown weapons and bolt spells share `TokenKind::Ranged`; bursts with `radius >= big_area_m`, beams and
  charged shots take `Charged`. Reason: the cap limits incoming threats per target, and to the defender an arrow and a
  fire bolt are the same threat (dodge or block a projectile); separate pools would let a mixed group double the volleys.
- **LOS**: a ray from the muzzle to the target's core every `ranged.los_every_ticks` (staggered by id); none, or an ally's
  bound within `ranged.ally_clear_m` of the line, makes the row illegal (`Close`: AI15/AI36 find a slot with a view).
- **Lead solver** `lead::solve(muzzle, speed, g, drag, target_pos, target_vel) -> Option<(dir, t)>`: `t` from distance
  over speed, aim point `target + v t`, low-arc launch angle in closed form, then `ranged.lead_iters` corrections of `t`
  with our own integrator (drag); `None` = out of range (`Close`). NPCs read target position and velocity from the C6c
  delayed `TargetTrace`, lead by `lerp(ranged.lead_min_frac, 1, skill)` and add the C6c aim error, so a fast strafe still
  beats a poor archer. The result is the command's `aim`, like a player's.

## Prop targets (C14)

Projectiles, bursts and sweeps that hit a `PROP` body (`GroundHit.body`) make a prop contact: material × damage type →
prop damage and break stages is the adapter's table (interaction IN1); impulses go through `sf_physics` on the host.
Designed in C14, not here.

## Data model and interfaces

- Hit row: `source` as above; `MoveSpec { …, draw: Option<Draw>, move_scale: f32 /* 1 */ }`; window kind `Release`.
- `Contact.defender_state.allied: bool`; `Outcome.deliver: bool`; `Command.aim: Option<WorldPos>`; `ActionClass::Cast`.
- Content records: `ProjectileSpec`, `AreaSpec`, `BeamSpec` (RON, patchable), keyed `ProjectileKey`, `AreaKey`, `BeamKey`.
- Host state: `ProjectileSet`, `StuckSet`, `BeamRun { owner, spec, start, next_pulse }`, `PendingLaunch { delivery,
  muzzle, aim, tick }`.
- Functions: `delivery::step(&DeliveryCtx, &mut ProjectileSet, &mut StuckSet, probe: &dyn GroundProbe, shapes:
  &PosedShapes, out: &mut DeliveryOut)`, `delivery::area(&AreaSpec, centre, …) -> SmallVec<[Contact; 8]>`,
  `delivery::beam_pulse(…)`, `lead::solve(…)`; `DeliveryOut { contacts, spawned, ended, stuck_ended, pulses }`.
- Hooks (content): `launch_spec(&Delivery)`, `allied(a, b)` (fills the flag), surface table in `ProjectileSpec`.

## Tunables (`combat.delivery.*`, net `Gameplay` unless noted)

All `Guess` unless noted; projectile speeds, drag, radii and areas are content data on the specs.
- Projectiles: `.projectile.max_live` (count, 16-1024, 256), `.projectile.max_stuck` (count, 0-512, 128),
  `.projectile.max_stuck_per_body` (count, 0-32, 8), `.projectile.stuck_life_s` (s, 5-600, 120), `.projectile.owner_grace_s`
  (s, 0-0.5, 0.1).
- Moving: `.draw.walk_scale` (×, 0.2-1, 0.5; Owner 2026-11-26 Q3 for "slowed", value Guess), `.cast.small_walk_scale` (×,
  0.2-1, 0.6).
- Aim (net `Presentation`, input side): `.aim.lock_lead_frac` (0-1, 1), `.aim.trace_max_m` (m, 50-500, 200),
  `.aim.min_point_m` (m, 0-5, 1.5).
- Area and beam: `.area.los_lift_m` (m, 0-1, 0.3), `.area.max_candidates` (count, 8-128, 32), `.touch.reach_m` (m, 0.5-3,
  1.5), `.beam.rate_hz` (Hz, 2-30, 10; specs may override).
- Ranged AI: `.ranged.los_every_ticks` (ticks, 1-30, 6), `.ranged.lead_iters` (count, 1-5, 3), `.ranged.lead_min_frac` (0-1,
  0.3), `.ranged.ally_clear_m` (m, 0-1, 0.4), `.ranged.big_area_m` (m, 1-10, 4).
- `.coop_friendly_fire` (bool, false; Owner 2026-11-26 Q4).

## Net / co-op

Projectiles, stuck sets, beams, areas, contacts and fates are host state (decision 15); not attacker-claimed. Clients
predict their own draw, cast clip and release (own move starts, combat.md "Co-op") and spawn a cosmetic copy at their
release, matched to the host's `ProjectileSpawned` by `(owner, release tick)`; other actors' copies start from the spawn
event, fast-forwarded by the tick difference. Copies run the same integrator and stop visually on local static geometry,
never make contacts, and end on `ProjectileEnded`. Fast-forwarding a client-fired projectile on the host (Overwatch-style,
capped) is net.md "Hit authority"'s call (C0e).

## Budgets (High, target laptop; all Guess until C8a's `#[ignore = "measurement"]` tests)

Per fixed tick: typical fight 8 live projectiles ≈ 0.02 ms; stress 64 live × (one sphere cast ~3 µs + bound scan of 150
fighters + 1-2 narrowphase sets) ≈ 0.25 ms; an area burst with 32 candidates and LOS rays ≈ 0.06 ms once; a beam pulse
< 0.01 ms. Over 0.1 ms in the bound scan → a uniform grid. Stuck projectiles cost only their visuals (instanced, render lane).

## Tests

Exact, Bevy-free (`sf_combat`), synthetic fixtures; probe = `StaticWorld` oracle (physics.md):
- **No tunnelling**: 100 m/s and 300 m/s at a 2 cm-thick wall and a 5 cm limb capsule, 1 000 seeded offsets: every one
  hits on the right tick, the touch point within radius + 1 mm.
- **Integrator**: gravity-only arc matches the closed form within 1 mm at 50 m; drag halves speed at the expected tick.
- **Stuck arrow persists**: joint anchor follows the posed shape over 300 ticks and after death (ragdoll pose); world anchor
  outlives `life_s`; cap evicts the oldest; cell unload ends it.
- **Fate**: i-frames → `passed` and a wall hit behind; guarded → `on_block`; surface table picks Stick vs Break; `Burst`
  makes exactly one area.
- **Falloff**: Linear `inner` 0.5, radius 4: d = 1 → 1, d = 3 → 0.5, d = 4 → 0; Smooth at d = 3 → 0.5; LOS blocked by a wall
  → no contact; candidates in id order.
- **FF matrix**: every source × {ally, owner in grace, owner after grace, co-op player with switch off/on} gives the table.
- **Charge release**: release at `min_s` → strength 0, at `full_s` and later → 1, before `min_s` → no projectile, ammo event
  not sent; muzzle = baked socket at the release tick.
- **Cast**: poise break in cast armour → no `Cast` op; release → `Launch` → projectile next tick → `Inflict` on hit, same
  tick as the contact; fizzle → nothing spawned.
- **Lead**: a target at constant velocity is hit by a perfect-skill shot (no error) at 10-60 m; out of range → `None`.
- **Determinism**: shuffled spawn order within a tick → identical `ProjectileSet` bytes; serde round trip.

## Reuse check

Bevy 0.20: no projectile, ballistics or delivery system (gameplay is ours; Bevy gives `FixedUpdate` and messages).
Crates (crates.io 2026-11-26): `bevy_ballistics` 0.1.0 (Avian 0.7, Bevy 0.19, 2026-07: Avian rigid bodies, our one-physics
rule rejects it); `bevy_ballistic` 0.8.0 (MIT, `bevy_math` ^0.20, 2026-10-19: launch-angle math only, no drag or moving
target; its formula is the textbook one we reuse, no dependency for ~40 lines); `bevy_diesel` 0.2.0 (ability framework,
Bevy 0.19); `ballistics-engine` 0.50.1 (firearm ballistics, heavy deps, not a game delivery layer). `bevy_rapier3d`: not
used; we query our own `sf_physics` world (`PhysicsProbe`, rapier3d 0.36.1), which the movers, activation and crosshair
already share. So: our own Bevy-free module in `sf_combat`, on existing query and hurt-shape primitives.

## Industry comparison

| Technique | Industry | Ours |
|---|---|---|
| Delivery × effect | Skyrim CK: delivery Self / Contact / Aimed / Target Actor / Target Location; projectile types Missile, Lobber, Beam, Flame, Cone, Arrow [CK]; ER, DD2 the same split [M] | match (`Sweep/Cone` touch, `Projectile`, `Beam`, `Area` at impact, point or self; effects separate) |
| Projectile movement | UE `UProjectileMovementComponent`: speed, gravity scale, bounce, homing, sub-stepped sweeps of its collider [Epic] | match (one exact segment per fixed tick); homing behind (open) |
| Ability carries the effect | UE GAS: ability spawns a projectile holding a `GameplayEffectSpec`, applied on hit [Epic] | match (`Delivery` payload, `Inflict` on landing) |
| Camera-to-focus aim | UE Lyra ranged weapons trace from the camera, fire from the muzzle [M] | match (over-shoulder, 1P) |
| Stuck arrows | Skyrim: arrows stick in actors and world, recoverable [M] | match (bone anchors, caps; recovery adapter's) |
| Cast over locomotion, interrupt | ER, Skyrim: upper-body casts, slowed; DD2: stagger breaks the chant [M] | match (owner Q3; cast armour) |
| Morrowind reference | OpenMW `apps/openmw/mwworld/projectilemanager.cpp`: magic bolts and fired/thrown projectiles, speeds from GMSTs (`fProjectileMinSpeed`/`MaxSpeed`, `fThrownWeapon*`, `fTargetSpellMaxSpeed`) [M] | match for the classic mapping (adapter, cite file + commit at C8a; effects.md pins `ca5e2576`) |
| AI lead | Doom/Halo-style lead with accuracy by skill and target speed [M] | match (delayed trace, C6c error) |

## Build plan

Rows, depends and verification: combat roadmap Stage 5 and C14 in Stage 6.
- **C8a** projectiles (state, integrator, segment vs hurt shapes and `PhysicsProbe`, fates, stuck set, events), after S5.
- **C8b** ranged sheet (draw, `Release`, `move_scale`, `Command.aim`, the three aim profiles, ammo events).
- **C8c** areas (placement, falloff, LOS) and beams (pulses).
- **C8d** cast action (`ActionClass::Cast`, cast clips, release → `Cast` → `Launch` → C8a/C8c, cast armour), after A5b.
- **C8e** ranged fighters (band, shared `Ranged` pool, LOS, lead solver).
- **C14** prop targets in sweeps and projectiles (adapter material × damage type table).

## Open questions

1. Homing and piercing projectiles (Skyrim-style seekers, ER piercing bolts): fields on `ProjectileSpec` when content asks.
2. Deflecting projectiles with a parry (Sekiro): projectile rows default `NoParry`; a deflect fate later.
3. Saving stuck and flying projectiles: not saved (dropped on load, Skyrim resets cells); revisit if players complain.
4. Magnitude falloff for area effects: needs an `Inflict` scale in `sf_effects` (E5), not taken by default (OpenMW).
5. Which Morrowind spells are "big" (rooted), ammo recovery chance, bolt speeds per school: C0d (combat-adapter.md).
6. Host fast-forward of client-fired projectiles and its cap: C0e (net.md "Hit authority").
   Answered 2026-11-26 (C0e): fast-forward by the client's half RTT, capped at `net.combat.projectile_ff_max_ms` (150 ms,
   Guess); net.md "Hit authority".
