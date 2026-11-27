# Interaction (smart objects)

## What it does

Furniture, doors, beds and the census classes are smart objects: each carries a def (slots, verbs, entrances),
its slots go in a point index while resident, users claim slots through one host table, and every use (player
input, NPC, script) takes one activation path. Built today: seats (approach then sit), swing doors, focus and
prompts, the rest hook, snuff / relight of fire lights (core, shell and the game's light hook), the destruction
core's maths (no shell yet). Carry and world rules are spec only.

## How it works

Three layers: the Bevy-free core `sf_interaction`, the shell `sf_interaction_bevy` (resources, messages,
systems), the Morrowind adapter `mw_interaction` (defs from records and meshes) plus the game wiring
`mw_interaction_bevy`.

- **Defs.** `InteractableDef` (`superfargoth/crates/sf_interaction/src/def.rs:69`) holds `SlotDef`s
  (`def.rs:97`, kind `SlotKind` `def.rs:142`: pose, door, lever, ..., and `Surface`, a put-down point for carried
  props that is claimed, never used) and `VerbDef`s (`def.rs:133`).
  The bake builds them once: `mw_bake::interact::interact_defs` (`bevyrrowind/crates/mw_bake/src/interact.rs:131`,
  product `mw.interact.defs`, `VERSION` 3 at `interact.rs:188`) classifies census furniture by id and model
  (`mw_interaction/src/furniture.rs:54`), measures its mesh (`measure.rs:79`) and calls `def_for`
  (`defs.rs:229`); census containers and activators get a slotless class verb (`class_verb`, `defs.rs:264`),
  static beds a slotless `sleep` (`bed_verb`, `defs.rs:279`). Swing doors get `door_def` at spawn
  (`defs.rs:296`). The game reads the product once at startup (`mw_interaction_bevy/src/lib.rs:93`).
- **Index.** A placement-spawner hook (`mw_interaction_bevy/src/index.rs:110`) writes `IndexChange::Insert` when
  a placement with a def enters R0 and `Remove` when it leaves. The shell keeps `InteractionIndex`, a
  `PointIndex<Entity>` (`sf_interaction_bevy/src/lib.rs:218`; core `sf_interaction/src/index.rs:122`, grid
  buckets); `find_slots` (`index.rs:233`) returns slots in radius by `advert x falloff`, ties by `SlotRef`.
- **Errand spots.** `pick_spot` (`sf_interaction/src/spot.rs`, IN46b) answers "where does this carried prop go"
  for both AI tiers: the nearest slot in radius passing the errand's filter (`spot_candidates`: distance, then
  `SlotRef`), skipping moving or hand-held props, other users' claims and slots a resting prop covers
  (`interact.place.clearance_m`); none: the first clear point of a golden-angle floor spiral the caller projects
  (`interact.place.spread_m`, `floor_tries`), else the anchor. Pure and order-independent; the AI claims the result.
- **Destruction core.** `apply_damage` (`sf_interaction/src/destroy.rs`, IN1a): health floors at 0; each stage whose
  `at` the health fraction reaches is crossed once, in def order, several per hit; `overkill` (damage past zero over
  full health) feeds the adapter's spill curve. `debris_lod` (`interact.debris.lod_m`), `BreakQueue` (at most
  `interact.debris.breaks_per_tick` per tick, the rest next tick) and the `destruct` rest state (stage + health).
- **Destruction shell.** `Destructibles` (`sf_interaction_bevy/src/destroy.rs`, IN1b) registers defs with a
  `destructible` from `IndexChange`; `DestroySystems::Damage` (after Apply) sorts the tick's `PropDamage` by a total
  key, scales by `DestructionRules` (kind x material), applies health at once (one `DestructChanged` per object per
  tick) and queues each crossed stage; `Release` sends at most `breaks_per_tick` `Destroyed` (hit tick, `break_seed`)
  and `Remove { OwnerDestroyed }` for a removing stage of a resident object (claims end next tick). `DestructRest`
  restores clamped, no event. Drops: gone, factor <= 0, non-script kinds on `ignore_external`.
- **Game breakables.** `mw_interaction_bevy/src/breakable.rs` (IN38b1): `BreakablesTable` (CONT + MISC + STAT records,
  `classify` with script facts: own script with an `OnActivate`-like block, or named by any script/dialogue token,
  refuses; STAT urns and pots listed since IN38c2, their ids made live by the host as `mw_scripts_bevy::LiveStatics`:
  they join `ActivePlacements` and an entity-drawn one spawns under a `LiveRoot`; a removing break runs `remove_live`
  then `Streaming::hide_covered`, `mw_rpg_bevy` `breakable.rs`); the index hook (`index.rs`, now also on `Dynamic`) attaches
  `destructible_def` + material to the furniture def or a slotless def (kept indexed with sitting off), then sends
  `DestructRest` from overlay owner `destruct`; `DestructionRules` = `damage_rules(BreakSettings)` live; standing
  `DestructChanged` saved to the overlay (a removing state writes none). Dev build: `Delete` breaks the focus.
- **Breaking out.** `mw_rpg_bevy/src/breakable.rs` (IN38b2) on each removing `Destroyed` after `Release`:
  `remove_broken` -> `ActivePlacements::remove_live` (`mw_scripts_bevy/src/lib.rs`: hooks, overlay removal, the
  container's `LiveRoot` visuals; containers are live there since IN38b2) plus an item's `ItemPivot`, or overlay
  only for an unloaded cell. **Spill** (IN38b3c): a `SpillContents` stage lands the container's store (seeded if
  never opened, taken out of `RpgState::inventories`) as one spawned overlay placement per kept stack: loss and
  weapon/armour wear from the overkill (`mw_interaction` `breakable.rs` `spill_draw`/`kept_count`/`health_loss`,
  `break_spill_*` tunables, gold whole), scattered in the footprint, `physics` blob awake (falls, settles), `mw.loose`
  blob (count, owned, condition) the take reads; live via `Streaming::spawn_when_ready`, unloaded via the overlay.
  An unmoved container whose cell left R0 before the release keeps its store (no transform known). No loot yet; debris
  bodies are framework-only (IN1c, below), the game fills no `DebrisSets` yet.
- **Claims.** `Claims` (`sf_interaction/src/claims.rs:225`): priority with preemption of Reserved claims only,
  queue tickets, leases, exclusion groups, one body hold per user plus hand claims, typed `Refusal`
  (`claims.rs:114`), invalidation reasons. Requests are applied as one sorted batch per tick
  (`apply_batch`, `claims.rs:601`) in `InteractionSystems::Apply` (`FixedUpdate`, `lib.rs:412`); results go out
  as `ClaimAnswered` / `ClaimChanged`.
- **Focus and verbs.** The game turns the focus ray's first hit into one `FocusCandidates` per fixed tick
  (`mw_interaction_bevy/src/input.rs:106`). `InteractionSystems::Focus` (`sf_interaction_bevy/src/focus.rs:139`)
  keeps each player's `FocusState` with a `FocusStabilizer` (`sf_interaction/src/verbs.rs:65`: switch margin +
  dwell, frozen while interact is held). Offered verbs come from `offered_verbs`
  (`sf_interaction_bevy/src/activate.rs:216`, `rank_verbs` in `verbs.rs:32`, which skips `Surface` slots), the same check validation uses,
  so the prompt never offers a refused use. The prompt text comes from the adapter's rows
  (`mw_interaction/src/activation.rs:218` `ROWS`; `input.rs:75`).
- **Activation pipeline** (module docs `sf_interaction_bevy/src/activate.rs:1`): `ActivateCommand` (the game's
  Use press, `input.rs:132`) is validated in `InteractionSystems::Activate` (`activate.rs:269`) and becomes a
  script `Activated`; scripts may answer "handled"; `ActivationSystems::Collect / Gates / Default` (`Update`,
  after the script slot, `lib.rs:401`) gather the rest, let gate systems refuse, and write one `UseApproved`
  or `UseRefused` each (`activate.rs:317`). A script's own `Activate` enters at the gates, never the scripts.
  Verb defaults read `UseApproved` after `ActivationSystems::Default`.
- **Seats.** `seat_requests` (`seat.rs:388`, `Update`) turns an approved pose use into a claim request;
  `seat_offers` (`seat.rs:426`, `FixedUpdate`) turns the grant into the action body's seat aim (approach rule
  `interact.sit.approach`, then enter; `seat_clips.rs:33`); `seat_events` (`seat.rs:545`) occupies or releases
  from the body's seat events. Every step writes a `SeatReport` (`seat.rs:268`), logged by the game
  (`mw_interaction_bevy/src/report.rs:90`).
- **Swing doors.** Core `DoorState` (`sf_interaction/src/door.rs:42`: toggle, smoothstep leaf, close-behind poll).
  Shell `Doors` (`sf_interaction_bevy/src/door.rs:82`): `door_default` (`door.rs:243`) toggles on
  `open` / `close`, `plan` and `commit` (`door.rs:279`, `door.rs:320`) step the leaf on the fixed tick and write
  `DoorChanged` / `DoorMoved`. The game moves the kinematic body (`mw_interaction_bevy/src/door_body.rs:80`),
  holds a step that would overlap a character (`door_body.rs:52`), and saves the rest state to the placement
  overlay (`door.rs:248`, owner `door`).
- **Rest hook.** An approved `VERB_SLEEP` (`sf_interaction/src/rest.rs:10`) use writes one `RestRequested`
  (`sf_interaction_bevy/src/rest.rs:28`, `rest_default` `rest.rs:45`). The game only logs it
  (`report.rs:85`): no rest UI exists yet.
- **Light state.** `LightState` (`sf_interaction/src/light.rs:51`): snuff / relight via `use_verb` (typed refusals),
  rule `set`, `LightChanged`, one rest byte (owner `light`). Shell (`sf_interaction_bevy/src/light.rs`): `Lights`
  (`:77`) registers objects with a `Light` slot from `IndexChange` (`register` `:191`, after the door registry, so
  validation sees `Lit` the same tick), keeps the `Lit` state tag and `GameplayLights` (`:117`, the AI's lights,
  never the GPU's); `light_default` (`:240`) runs an approved snuff / relight (no verb toggles) and writes
  `LightChanged`; `LightRest` (`:149`) restores a saved state with no event. Game (`mw_interaction_bevy/src/light.rs`,
  IN19c): `LightHook` spawns a fire light's visuals under a `LightAnchor` (lit from the overlay byte, else lit),
  indexes `mw_interaction::light_interactable` in R0 (slot disabled when `mw.rules.interaction.snuff` is off),
  `follow_changes` sets the anchor and saves the byte, `apply_anchors` hides the point light and swaps glow
  materials to a shared dark copy. Prompts "Snuff" / "Relight" verb-only; a carryable keeps "Take".
- **Object states (IN17a, core only).** `ObjectStates` (`sf_interaction/src/states.rs`): `TagId` states with a TTL
  per placement (`Ttl::Secs` in ticks, refresh keeps the later expiry, `UntilRemoved`), `expire(now)` in (tick,
  placement, tag) order. Materials: `InteractableDef::material`.
- **Rules (IN17b, core only).** `step_rules` (`sf_interaction/src/rules.rs`): expires TTLs, then evaluates up to
  `interact.rules.evals_per_tick` queued `Cause`s oldest first in a canonical order (arrival order never matters);
  the first `InteractionRule` in record order whose `source` passes the cause's tags and `target` the material +
  states applies (remove, add, then damage/stage/effect/change/noise as `RuleEvent`s). No shell system feeds it yet.
- **Fire (IN17c1, core only).** `step_fire` (`sf_interaction/src/fire.rs`): rain at `interact.fire.rain_douse` puts a
  burning body out (`Wet`), `fuel_s` after its first `Burning` tick it is `Charred` (`Burnt { stage }`), then flammable
  bodies not burning, `Wet` or `Charred` gain heat from burning spreaders within `interact.fire.spread_radius_m` (linear
  falloff x `wind_factor` x seeded per-second jitter), cool without one, ignite at `ignite_heat`. Budgets (IN17c2):
  at most `interact.fire.burning_max` burning per `interact.fire.region_m` square (more ignitions queue, oldest first),
  at most `interact.fire.spread_checks_per_tick` heat checks per tick round-robin with elapsed time (slower, same
  set); a `structure` material with `FireBody::in_settlement` never catches from spread. No shell system yet (IN39).
- **Impacts (IN4a core, IN4b shell).** `impact_damage(impulse_ns, speed_mps, params)` (`sf_interaction/src/impact.rs`):
  0 under `interact.impact.speed_min_mps` or within the `interact.impact.impulse_min_ns` dead zone, else
  `interact.impact.damage_per_ns x (impulse - dead zone)`; non-finite in -> 0, overflow saturates. Kind `DAMAGE_IMPACT`
  (`destroy.rs`). Shell (`sf_interaction_bevy/src/impact.rs`, `physics` feature, on in the game): `ImpactSystems`
  after `PhysicsSystems::Step`, before `DestroySystems::Damage`, reads this step's `PhysicsEvent::Contact`s
  (`ImpactCursor` into `PhysicsEvents`, reset in `First`), maps bodies to live `Destructibles` through
  `PhysicsStreaming::body`, skips pairs with a `DebrisPiece`, `PROJECTILE` or `HELD` body (loose `DEBRIS`-layer items
  count) and sends the same raw `PropDamage` to each registered side, `dir` into that side; `culprit` the user who threw either body
  (`Thrown`, fed from `CarryOutcome::Thrown { body }` of an `InteractionUser` holder, kept until the body sleeps, is held
  again or leaves; the other body's thrower first), else `None` (IN4c, `impact.rs`).
- **Noise (IN18a, core only).** `loudness_m(reach, v, params)` (`sf_interaction/src/noise.rs`): the material's
  `impact_reach_m` x `clamp(v / interact.noise.ref_speed_mps, 0, speed_mul_max)`, none under `min_speed_mps`.
  `NoiseMerge::step` thins one tick's `Noise`s: an object's loudest, inside its `interact.noise.merge_s` window only a
  louder one, at most `per_tick_max` loudest first (a cut noise opens no window), any arrival order alike.
- **Noise shell (IN18b, `sf_interaction_bevy/src/noise.rs`).** `NoiseSystems::Collect` (after door commit, break release,
  impacts) gathers `DoorChanged` of registered doors (doorway, `door_reach_m` x material ratio = `impact_reach_m` /
  `material_ref_m` from `InteractionMaterials`, revealed), `Destroyed` of registered destructibles (object, reach x
  `speed_mul_max`, culprit, not revealed) and, with `physics`, this step's contacts (`NoiseCursor`; louder side of indexed
  placements with a known material; `Thrown` body -> `ThrownLanding` + thrower; debris pieces, `PROJECTILE`, `HELD` silent)
  into `NoiseInbox`; `NoiseSystems::Emit` writes the merged `InteractionNoise` (`space: None`). No `Drop` case yet.
- **Noise to NPC hearing (IN18c, `mw_interaction_bevy/src/noise.rs`).** The game copies each `InteractionNoise` into one
  `sf_ai_bevy::NoiseEvent` after `NoiseSystems::Emit` and `AiPerceptionSystems::Hearing` (heard on the next perception
  tick), stamped with `AiPerceptClock::tick`; `maker` -> `source` (same keys), kind from `InteractionNoiseKinds` (ids from
  256: door, break, object; material tags `register`ed). Skipped while either message type is missing.

## Bevy features used

`Message`s for every hand-off, `SystemSet` chains on `FixedUpdate` (claims, focus, activation, doors, seats) and
`Update` (gates, defaults), an `On<Remove>` observer (`mw_interaction_bevy/src/door.rs:154`). See
[schedules, transforms, ECS](../bevy/schedules-transforms-ecs.md).

## Switches and counters

- `mw.rules.interaction.sit` (`mw_interaction/src/tunables.rs:19`, default on; Classic off): off indexes no
  furniture, flipping it live removes or re-inserts the resident furniture (`index.rs:134`).
- Tunables `interact.claim.*`, `interact.queue.*`, `interact.focus.*`, `interact.door.*`
  (`sf_interaction_bevy/src/lib.rs:188` `TUNABLES`) and `mw.interact.*` (measure, furniture) follow the
  console live.
- Log lines: `interact: indexed N furniture placements in <worldspace> (x, y, z)`, `interact: N swing doors indexed ...`,
  `seat: ...` per `SeatReport`, `interact: rest requested <placement>`.

## Known limits

- No rest UI: `RestRequested` is only logged. Lying down (IN3b) and lean (IN3c) wait for baked bed and lean clips.
- `interact.cook` is a stub (`sf_interaction/src/cook.rs:16`); defs are cooked by the game's bake step only.
- Carry and world rules are spec only; breaking removes the object and spills a container's contents; debris is framework-only (`sf_interaction_bevy` `debris.rs`, `physics` feature: seeded pieces within `interact.debris.lod_m` of `PhysicsFocus`, `awake_max` cap fading the oldest, fade on sleep or `fade_s`, shrink, despawn; IN1c) and the game has no sets or mesh hook yet; breakable `STAT` urns stay out of the near precombine (`--off scene` arm, `mw_bake` `far.rs` `precombine_exclude`, IN38c3), so they break there too after a re-bake.
- Lights: off-by-default lights get no def; the loop sound and the AI's light copy don't follow `LightChanged` yet.
- History of each slice: [ledger](../../ledger/interaction.md).

## Specs

[interaction](../../../superfargoth/docs/specs/interaction.md) (core, with carry, destruction, vehicles, world
rules specs), [interaction-adapter](../../../bevyrrowind/docs/specs/interaction-adapter.md),
[world-reactions](../../../superfargoth/docs/specs/world-reactions.md).
