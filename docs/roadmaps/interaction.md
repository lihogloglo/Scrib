# Lane roadmap: Interaction (smart objects, carry, destruction, fire and light, moving things)

Specs: `superfargoth/docs/specs/{interaction,world-reactions}.md`, `bevyrrowind/docs/specs/interaction-adapter.md`.
Research: `docs/research/gameplay-2026-11-25/interaction.md` (§ = its sections). No wiki page or ledger yet: the first
build slice creates `docs/ledger/interaction.md`. **This lane owns every IN row**, including IN17-IN19 (from ai.md)
and IN20-IN26 + TV0b (from travel.md), with the same IDs and meanings.

## 1. Player-facing goal

Modern preset, lane done (stage in brackets): sit, lie, lean, open, close and lock swing doors, each with a verb prompt
and a hold list of alternatives; Morrowind's scripted objects behave as before [1]. Carry **one thing in the hands**
(chair, crate, body), slower, no sprint or roll; throw it; smash crates, barrels and pottery that spill; bash locks;
every impact, door and drop is a noise NPCs investigate [2]. Snuff and relight lamps to darken a room; fire spreads
within a budget, pushed by wind, stopped by rain; scorched grass regrows fast [3]. NPCs sit, sleep, work at forges,
carry and tidy through the same objects [4]. Levers, ladders [5]. Stand, sit and walk on moving decks [6]; ride a guar
that steers like an RDR2 horse [7]. **Classic keeps** vanilla: furniture inert, nothing breaks or burns, lights fixed,
travel fades (adapter switches `mw.rules.interaction.*`: `sit`, `break`, `move_furniture`, `mounts`, `ride_travel`,
new `fire`, `snuff`).

## 2. Where we are (2026-11-25)

- **Built:** player carry/drag/throw (`sf_physics` `carry.rs`, `HolderLimits`); ledger physics 2026-10-26: drag
  slow-down only on the free camera, thrown items pass through the player (slice 11 held). Activation:
  `Activated`/`ActivationOutcome` (`sf_script_bevy/src/lib.rs:66`, a script may suppress the default); player `Focus`
  with a prompt verb (`mw_player_bevy/src/lib.rs:773`, OpenMW `hasToolTip`): NPCs talk, load doors fade
  (`mw_doors_bevy`), containers, books, items. Non-load doors: a dev-only sound, no state or motion. MG4 furniture
  census (~10.3k placements). AI8a awareness with `Distraction`, `Stimulus::Noise` (`sf_ai`).
- **Not started:** crates `sf_interaction(_bevy)`, `mw_interaction` (none exist), every IN slice; ROADMAP 3.11 `[ ]`;
  no IN- visual check for this lane.
- **Owner:** 2026-11-11 answers (interaction.md "Owner questions": all breakables break, spill with damage, vandalism
  offence, buildings parked, mounts = climb on and drive, no free sitting, IN5 into IN9, Kimodo player clips, "Sit"
  prompt); DECISIONS 2026-11-25: budgeted fire spread, fast grass regrowth, snuff/relight, NPCs react to darkness,
  thrown/smashed objects draw attention, one carried thing, slower; bashing locks/doors by strength is a crime if
  reported; travel Q2-Q6.

## 3. Decisions

1. **One slot model for player and NPC** (UE Smart Objects, Bethesda furniture markers, RDR2 scenarios, Cyberpunk
   workspots; §1, §3): spec defs, `PointIndex`, host `Claims` kept. **Adds:** claim `priority` (UE: a higher user takes
   a Reserved claim, never an Occupied one; player/quest > schedule > ambient) and a **queue ticket** per door or
   workstation slot.
2. **Slot kinds widen** (§3; spec has none): `door` (Open/Closed/Locked, kinematic hinge, doorway claim + queue),
   `lever`, `ladder` (edge chain for locomotion.md's `Docked` mover, entrances both ends, NV9 link), `workstation`
   (NPC activity loop, adapter menu for the player), `light`. Load doors keep the fade (`mw_doors_bevy`, AI12).
3. **Verbs and activation** (RDR2, Cyberpunk prompts): a slot def carries `verb` + `priority`; focus shows the best,
   hold lists the rest. A use is an `Activate(tag)` command (net rule 2) the host turns into the existing `Activated`;
   handled suppresses the default. `OnActivate`/`Activate` are **DQ-6**: this lane gives the contract and defaults.
4. **Carry: one link, two presentations** (RDR2/TLOU attach + clip; Skyrim/HL2 spring; §2): third person uses the
   spec's `HeldProp` for the player as for NPCs; first person keeps the built spring. One thing held (owner); carry
   class data: speed multiplier, `can_sprint`, `can_roll`. **Departs** from the spec (player = spring only). Physics
   stays rapier (§0); `avian_pickup` is a design reference.
5. **Destruction tiers** (Bethesda `DEST`, UE Chaos, Teardown; §4): stage swap first; later chunks with Chaos
   **cluster thresholds**; debris **distance LOD** (tunable, ~25 m: swap + effect only); glass = baked shard sets;
   loot-on-break and `salvage` as data. Runtime fracture crates rejected; buildings parked (owner).
6. **World rules layer** (BotW chemistry, Divinity surfaces, Gameplay Tags; §1, §5): `InteractionRules` records,
   source tags × target material + state → state with TTL, `DestructionRules` damage, effect, `WorldChange`, noise.
   States are `TagId`s with TTL (`Burning`, `Wet`). Fire spreads on the fixed tick under a per-region budget,
   wind-biased, blocked by `Wet` (weather wetness); ground burn is a coarse per-cell grid tinting groundcover and
   terrain, regrowing fast (owner). Electric, ice, surfaces: later rows. **Departs** (the spec is silent).
7. **Light as state** (Thief, Dishonored, Skyrim): `Lit` tag, snuff/relight verb, light component follows;
   `LightChanged` feeds ai.md `LightExposure`, AI28 `Darkened`, AI27 dusk lamps. Never from the GPU.
8. **Interactions make noise** (UE `ReportNoiseEvent`, Dishonored; §5): impacts over `ContactForceEvent` thresholds,
   `Destroyed`, doors, throws, drops, bashes; loudness = material `NoiseKind` × impact speed; `NoiseEvent` shape per
   ai.md "Perception". A thrown prop fills `Distraction`, not the thrower's entry; landing in view raises `Nuisance`.
9. **Persistence** (§6): IN15 adds at-rest snapping, holder link wins on stream-out, a `burnt` class.
10. **Moving things follow travel.md decisions 3-6** (based movement, seats = attach + IK, RDR2 mounts); IN7's rail
    half dropped (TV4).
11. **Tick, net, data:** claims, spread, breaks on `FixedUpdate`, host-authoritative, seeded visuals per client; all
    content via `sf_records`; Bevy-free cores; budgets against 8.3 ms (the spec still says 11 ms).

## 4. Stages

Stages 2 and 3 run in parallel after Stage 1; Stage 6 can start any time (IN20 needs nothing). Owner checks are IN-<n>.

### Stage 0: specs (docs only)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| IN27 | `interaction.md` core, (a), (b): decisions 1-5, 9, 11; build table | docs | - | **done 2026-11-25** (core 41 KB + `interaction-carry.md`, `-destruction.md`, `-vehicles.md`; adds hand claims for seated chains, stable focus, contact points from gta6.md) |
| IN28 | `interaction.md` new "(d) World rules": decisions 6-8, caps, reuse, industry | docs | IN27 | **done 2026-11-26** (`interaction-world-rules.md`, 19.8 KB; noise via `InteractionNoise` → host → `NoiseEvent`; `Lit` → `GameplayLights` → MA16; IN17 moves `material` to `InteractableDef`) |
| IN29 | `interaction-adapter.md`: activation per class, doors, lights, breakables, ignition, switches | docs | IN28 | **done 2026-11-26** (17.2 KB: 15-class activation table with OpenMW 4e718b50 cites, census defs, world-rules mapping, breakables, switches) |
| IN30 | `world-reactions.md`: IN14 kinds (door, lever, light, fire), IN15 additions | docs | IN28 | **done 2026-11-26** (14-kind `WorldChange` table with concerns fed, `burnt` + door/lever/light reset rows, at-rest save, holder link wins) |
| TV0b | `interaction.md` (c) rewrite (travel decisions 3-6; IN7, IN10, IN11, IN20-IN26) | docs | TV0 | **done 2026-11-26** (`interaction-vehicles.md`; IN11c3a/b in travel.md) |

### Stage 1: use the world

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| IN0a | Core crate + claims: def types (serde), `Claims` (priority, preempt Reserved only, queue, lease, groups, hand claims, invalidation, sorted batch), `InteractParams` defaults | `sf_interaction` | IN27 | **done 2026-11-25** (26 exact tests; spec "As built (IN0a)") |
| IN0b | Core queries: `PointIndex` + `find_slots`, `rank_verbs` + focus stability, `pick_entrance`/`pick_exit`, `interact.cook` stub | `sf_interaction` | IN0a | **done 2026-11-26** (13 exact tests, crate 39; spec "As built (IN0b)") |
| IN31a | Shell part 1: new `sf_interaction_bevy`, `Claims` + `PointIndex` resources, claim/index request messages applied as one sorted batch on `FixedUpdate`, lease ticks, `InteractionEvent` out, live `interact.*` binding | `sf_interaction_bevy` | IN0a, IN0b | **done 2026-11-26** (6 headless tests; spec "As built (IN31a)") |
| IN31b | Shell part 2: per-player `FocusStabilizer`, best verb + hold list, `Activate` → `Activated` (+ verb) → gates → default | `sf_interaction_bevy` | IN31a | **done 2026-11-26** (b1 activation pipeline, b2 per-player focus; 12 tests; spec "As built (IN31b1)", "(IN31b2)") |
| IN32 | Adapter defs (census) + each existing class's activation as a default verb, Classic switches | `mw_interaction` | IN31, IN29 | exact per class, unchanged; def counts; **[x] IN32a/b done 2026-11-26** (`mw_interaction` core + `measure()`; bake step `mw.interact.defs`: 31 seat, 28 table, 12 counter, 46 bed defs; spec "As built (IN32a/b1/b2)"); IN32c1 + c0 done 2026-11-26 (`mw_interaction_bevy`: defs indexed, player focus + press through the contract, seat families + `SeatClips`); IN32c2 done 2026-11-26 (defaults on `UseApproved`, `runs_default`); IN32c2b audio door sounds done; owner check IN-32 awaiting |
| IN2 | Player sits/stands: Interaction move, `Seated`, side entrances, Kimodo clips | `sf_interaction`, `sf_action` | IN31 | **headless done 2026-11-26** (IN2a moves, IN2b1 seat offer, IN2b2 shell default; spec "As built (IN2a)", "(IN2b2)"); owner check = anim's ANIM-11 (AN26c2 508a628a; IN-2 has no scene of its own, the "Sit" prompt is IN-32 question 4): **FAILED 2026-11-26** (E on "Sit" did nothing). Sit fix built 2026-11-26: IN2c1-c3 (reports, log lines, repro), IN2c4 approach walk, IN2c5 `Approaching` report, IN2c5w claim given back at a loss, IN2c4b the walk brakes into its target, IN2c6 real-data asserts (21 of 21 stances sit); spec "As built (IN2c4b + IN2c6"; ANIM-11 to be re-asked |
| IN3 | Beds, lean spots, rest hook, hand IK | `sf_interaction_bevy`, `mw_interaction` | IN2 | hook once; owner IN-3. **IN3a done 2026-11-27** (`VERB_SLEEP`, `RestRequested` default, static beds' `sleep` in the bake, logged); IN3b/c wait on anim AN30c-e, IN3d open |
| IN33 | Swing doors (doors, containers; "Locked" read from crime's `LockState`, crime.md decision 8), hinge, doorway queue, `WorldChange` | `sf_interaction(_bevy)`, `mw_interaction` | IN31 | exact transitions; owner IN-33. **IN33a core done 2026-11-26** (`door.rs`: state, toggle, swing sign, close-behind); **IN33b1a/b done** (physics: body overlap query, kinematic placements); **IN33b2a done** (shell `door.rs`: `Doors` registry, default toggle on `open` / `close`, `DoorChanged`, `door.open` tag, two tunables); **IN33b2b done** (swing on the fixed tick: `DoorSystems::{Plan, Commit}`, `DoorSweeps` with the host's `blocked` flag, `DoorMoved`, close-behind); **IN33b2c done** (rest state: three bytes in the core, `DoorRest` restore in the shell; the game stores it as a placement-overlay state blob); **IN33c1 done** (`mw_interaction::door_def`: hinge at the origin, axis up, 90 degrees one way, close-behind, slotless `open` / `close`, an NPC-only `pass` doorway slot; the two swing rows Offered); **IN33c2a done** (`mw_interaction_bevy::DoorHook` on the Door class: a swing door's visuals under a `DoorPivot`, in R0 indexed with `door_def`, listed kinematic, rest state restored from the overlay; **IN33c2b1 done** (`door_body.rs`: the kinematic body takes each `DoorMoved` as its target, a restored rest or a late body is teleported, the pivot follows the body's interpolated pose each frame); **IN33c2b2 done** (`block_sweeps`: a step whose leaf would overlap a character's capsule is held, the door pauses in their path and resumes); **IN33c2b3 done** (`save_rest`: each toggle writes the door's rest state to the placement overlay, so an open door is open again after its cell reloads and in a save); **IN33c2c done** (`prompt`: an open swing door prompts "Close", the string from its row); **IN33c2d done** (outside R0 the pivot stands at the rest state: a door saved open spawns open, one leaving R0 mid-swing ends at its target); **IN33c3 done** (`mw_audio_bevy`: each toggle plays the door's open or close sound, every build; the AU-5 scaffold is deleted); **IN33d done** (`mw_rpg_bevy::container_door`: a container window's open and close send `DoorChanged`, no leaf); all IN33 slices built, owner check IN-33 awaiting |

### Stage 2: carry, throw, break, and the world hears it

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| IN35 | Player 3P `HeldProp`: attach, grip IK, carry class gates; 1P spring kept | `sf_interaction_bevy`, `sf_physics` | IN0, MG7b | exact gates; owner IN-35 |
| IN6 | Player moves furniture: promotion, carry/drag, slots follow, persistence | `sf_interaction_bevy` | IN2, IN35 | overlay round trip; owner IN-6 |
| IN1 | Crates/barrels break: prop targets, stages, spill, debris + LOD, `Destroyed`, blob | `sf_interaction(_bevy)` | IN0, combat C2c/C3 | stage math, round trip; owner IN-1. **IN1a + IN1b done 2026-11-27** (core maths, shell: registry, damage, paced `Destroyed`); **IN1c done 2026-11-27** (seeded debris bodies, cap, fade, LOD; game sets + mesh hook not yet) |
| IN38 | Adapter breakables (crates, barrels, urns, sacks, pottery), materials, `NoiseKind` | `mw_interaction` | IN1, IN29 | exact rows on game data. **IN38a done 2026-11-27** (pure table: classes, refusals, defs, damage rows, Classic off); **IN38b1 done 2026-11-27** (game wiring: records table, defs on the index, rules, overlay save/restore, dev key); **IN38b2 done 2026-11-27** (live removal of broken containers and pottery; spill split to IN38b3); **IN38b3-S spec done 2026-11-27** (spill: queued spawn, motion blob, game spill = b3a/b/c); **IN38b3a done 2026-11-27** (framework queued spawn `Streaming::spawn_when_ready`); **IN38b3b done 2026-11-27** (items come back awake from an awake `physics` blob, `ItemMotion`); **IN38b3c done 2026-11-27** (game spill: contents land as loose items, loss + wear from overkill, `mw.loose` take); **IN38c-S spec done 2026-11-27** (static urns: scene hide, game removal, precombine veto = c1/c2/c3); **IN38c1 done 2026-11-27** (framework `Streaming::hide_covered` + `CellScene::hidden`); **IN38c2 done 2026-11-27** (game: listed statics live via `LiveStatics`, break = `remove_live` + `hide_covered`); **IN38c3 done 2026-11-27** (breakable statics vetoed from the near precombine, far fingerprint v4); owner IN-38 |
| IN45 | `Give`: hand a `HeldProp` from one holder to another (errand `Give(ActorRef)`, ai.md AI0f/AI31), refused past `ai.errand.give_wait_s` | `sf_interaction(_bevy)` | IN35 | exact hand-over; one holder at a time |
| IN46 | Pure `pick_spot(anchor, filter, occupancy, ActorKey)` over the `PointIndex` (ai.md AI0f), so tier 0 places a prop where tier 2 would; nearest valid floor past `ai.errand.spot_radius_m` | `sf_interaction` | IN0b | exact: same spot both tiers. **IN46a done 2026-11-27** (`SlotKind::Surface`); **IN46b done 2026-11-27** (`spot.rs` `pick_spot`; AI32 calls it) |
| IN4 | Impact damage from `PhysicsEvent::Contact` impulse thresholds (spec "Impacts (IN4)") | `sf_interaction_bevy` | IN1 | exact map; owner IN-4. **IN4-S spec done 2026-11-27** (`Contact` impulse + speed gate, same raw amount on both sides, debris-shell pieces/projectile/held filtered, culprit gap IN4c; rows IN4a/b; **IN4-REV review done 2026-11-27**: loose items are `DEBRIS` bodies, so the filter is the `DebrisPiece` set, not the layer); **IN4a done 2026-11-27** (`impact.rs` `impact_damage`, `DAMAGE_IMPACT`, 3 tunables); **IN4b done 2026-11-27** (shell `ImpactSystems`, contacts -> `PropDamage`, on in the game); **IN4c done 2026-11-27** (thrower culprit: `Thrown` table from `CarryOutcome::Thrown { body }`) |
| IN18 | `NoiseEvent` emitters: impacts, breaks, doors, throws, drops, bashes (pick, trap and forced-lock noise is crime's, `sf_crime_bevy`, crime-locks.md) | `sf_interaction_bevy` | IN4, IN33, AI8a | exact loudness per case. **IN18a done 2026-11-27** (core `noise.rs`: `loudness_m`, `NoiseMerge`, 9 tunables); **IN18b done 2026-11-27** (shell `noise.rs`: doors, breaks, contacts -> `InteractionNoise`); **IN18c done 2026-11-27** (game `mw_interaction_bevy` `noise.rs`: -> `NoiseEvent`) |
| IN37 | Bash locks/doors: strength-scaled lock damage, contents risk, offence hook (blow scale matches crime's lock `max_hp` = `mw.crime.lock_hp_per_level`, Guess 2; vandalism offence "as Theft" per crime-adapter: check against interaction-adapter MWR1) | `sf_interaction`, `mw_interaction` | IN1, IN33 | exact thresholds. **IN37-S spec done 2026-11-27** (destruction "Bashing (IN37)": rows IN37a-d); **IN37-REV review done 2026-11-27** (Hits + Land split, user by actor, `bare_kind`, OpenMW `adjustWeaponDamage`, 8 blows); **IN37a core done 2026-11-27** (`bash.rs`, `from_bashed`); **IN37b shell done 2026-11-27** (press, hits, land, 3 tunables) |
| IN14 | `WorldChange` vocabulary + placement diff | `sf_interaction` | IN1, IN6, IN33 | one event per transition. **IN14a done 2026-11-27** (core `world_change.rs`: kinds, `WorldChange`, mappers, `fold_tick`); **IN14b done 2026-11-27** (shell `WorldChanged` from doors, lights, breaks; `ChangeOwners`) |
| IN15 | `ResetRules`, classes, ttl, cap, lazy reconcile, at-rest snap, holder link | `sf_interaction`, `sf/overlay` | IN1, IN30 | expiry, eviction, old saves |
| IN36 | Thrown props hit people: damage/stagger by mass × speed, assault when seen | `sf_interaction_bevy`, `sf_combat` | IN4, Q2 | exact curve; owner IN-36 |

### Stage 3: light and fire

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| IN19 | `Lit`, snuff/relight verb, light follows, `LightChanged` | `sf_interaction(_bevy)`, `mw_interaction` | IN31 | exact events; owner IN-19. **IN19a done 2026-11-27** (core `LightState`, `LightChanged`, rest byte); **IN19b done 2026-11-27** (shell `Lights`, `GameplayLights`, snuff/relight default); **IN19c done 2026-11-27** (adapter defs, game light hook, saved state; owner IN-19 awaiting) |
| IN17 | Rules core: `InteractionRules`, states + TTL, `Wet`, budgeted wind spread | `sf_interaction` | IN28, IN1 | exact on a synthetic grid; rain stops it. **IN17a done 2026-11-27** (material on the def, `ObjectStates` + TTL); **IN17b done 2026-11-27** (`step_rules`); **IN17c1 done 2026-11-27** (`step_fire` heat core); **IN17c2 done 2026-11-27** (region cap + queue, round-robin checks, settlement structures) |
| IN39 | Fire in game: effects (capped lights), charred stage, damage, extinguish | `sf_interaction_bevy`, `mw_interaction_bevy` | IN17 | sanity; owner IN-39. **IN39-S spec done 2026-11-27** (world-rules "Rules and fire shell (IN39)": host-filled weather/wetness/shelter/zone inputs, rules shell, fire step, outputs, save; rows IN39a-f; reviewed 2026-11-27: core `Cause`/`RuleEvent` carry culprit + source, host-owned tags, `force_stage`, no new registry); **IN39a1 rules core done 2026-11-27** (culprit + source on events, host-owned tags, `force_stage`); **IN39a2 rules shell done 2026-11-27** (`RuleCause` -> `RuleSystems`, routes to lights, `PropDamage`, `PropStage`, `ChangeInbox`, noise, effects); **IN39b fire step done 2026-11-27** (`FireCandidates`, host wetness/shelter/zone closures, `Wet` refresh, culprits along the spread, `Ignited`/`Burnt` changes) |
| IN40 | Ground burn grid: groundcover/terrain tint, fast regrow | `sf_interaction_bevy` | IN17 | exact regrow; owner IN-40 |
| IN41 | Adapter: ignition sources, flammables, town rule (Q1), Classic off | `mw_interaction` | IN39, IN40 | exact rows; Classic ignites nothing |
| IN44 | Measurement: claims, spread, debris, burn grid ms, High, 4060 laptop | tests | IN39, IN40 | `#[ignore = "measurement"]`; perf ledger |

### Stage 4: NPCs share the objects (with the AI lane)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| IN8 | = **AI14** (AI lane builds): `UseSlot`, `find_slots`, adverts, priority | `sf_ai`, `sf_interaction` | IN0, IN2, IN3 | deterministic; owner IN-8 |
| IN9 | NPC held props + `Relocate` (pull-out folded in) | `sf_interaction`, `sf_ai` | IN8, IN35, MG7b | headless plan; owner IN-9 |
| IN16 | Loose-item `take` slots, ownership on take, `LootRules` | `sf_interaction` | IN8, LW10 | ownership and ledger |
| IN42 | Workstations: NPC loop, player menu (Q4), queue; jobs as TOML mod data (no recompile) | `sf_interaction`, `mw_interaction` | IN8 | exact queue; a new job file loads without a rebuild; owner IN-42 |

AI consumers: AI31 and AI24 (IN9), AI27 (IN19), AI28 (IN18, IN19), AI8c (IN18), AI19 (IN14).

### Stage 5: mechanisms and ladders

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| IN34 | Levers, buttons, switches: toggle, linked target, saved (scripted activators stay scripts) | `sf_interaction(_bevy)` | IN31, IN14 | exact; round trip |
| IN43 | Ladders: edge-chain slot, both entrances, `Docked` enter/exit, NV9 link | `sf_interaction`, `sf_controller` | IN0, `Docked`, NV9 | exact; NPC crosses (AI33); owner IN-43 |

### Stage 6: standing, sitting, walking on moving things (travel.md Stage 3; TV4, TV6, TV8, TV13 consume it)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| IN20 | `GroundHit` carries the ground body and point velocity; kinematic velocity from the tick pose delta | `sf_physics`, `sf_controller` | - | **done 2026-11-25** (4 exact tests, 30/144 Hz) |
| IN11 | Based movement on any moving body (lifts, platforms, cart beds, decks) | `sf_controller` | IN20 | as travel.md |
| IN21 | Deck presentation: drawn pose, foot IK, camera interpolated base-local | `sf_controller_bevy`, `sf_camera` | IN11c3a | owner IN-11 |
| IN7 | Seat links on a moving owner: create/persist/delete, seat modes, dismount spot, jump off (rail half dropped) | `sf_interaction` | IN2, IN20 | exact; navmesh |
| IN23 | Seat IK: pelvis to seat, feet to footrest/stirrup/deck, hands to rail/reins | `sf_anim_graph` | IN7 | owner IN-23 |
| IN24 | Passenger state `{ vehicle, seat or base-local pose }`: save, `Boarded`/`Left`, base-relative replication | `sf_interaction`, `sf_save` | IN7, IN11c3b | mid-ride save/load |
| IN22 | Deck-local nav: navmesh in the vehicle frame; NPCs walk a moving deck | `sf_nav(_bevy)` | IN11c3a, NV9 | bow to stern |

### Stage 7: mounts (travel.md Stage 6; TV17, TV18 follow)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| IN25 | Mount locomotion profile (RDR2): speed, inertia, turn rate by speed, gait ladder | `sf_controller`, `sf_action` | TV-R2 | exact |
| IN26 | Procedural quadruped legs: N-leg foot placement, pelvis/spine pitch-roll, stride warp | `sf_anim_graph` | TV-R2 | owner IN-26 |
| IN10 | Creature mount: mount/dismount by side, `RiderToMount`, astride seat, mounted gaits | `sf_interaction` | IN7, IN23, IN25, IN26 | owner IN-10 |

**Later / parked:** IN12 chunks (cluster thresholds), IN13 buildings (owner), placement mode (Q5), electric/ice/
surfaces, physics vehicles, mounted combat. **Dropped:** IN5 (into IN9, owner 2026-11-11).

## 5. Spec changes (as spec slices, before build)

- IN27 `interaction.md`: core (priority, queue, kinds, verbs, activation contract linking DQ-6), (a) (player
  `HeldProp`, carry class), (b) (LOD, clusters, salvage, bash), build table (IN5 out, IN31-IN44 in), 8.3 ms.
- IN28 new "(d) World rules": rules, states, spread, burn grid, noise, `Lit`; it settles where `NoiseEvent` lives
  (read by `sf_interaction_bevy` and `sf_ai_bevy`; `sf_interaction` never names `sf_ai`).
- IN29 adapter (pottery breaks; doors are bashed open, not destroyed). IN30 world-reactions. TV0b "(c)".

## 6. Dependencies on other lanes

| Lane | Needs | Provides |
|---|---|---|
| AI | AI8a (built); AI14 builds IN8 | IN0, IN2, IN3, IN9, IN14, IN16-IN19 |
| Travel | TV0, TV-R2 | Stage 6 to TV4/TV6/TV8/TV13; Stage 7 to TV17/TV18 |
| Dialogue-quests | DQ-6 owns `OnActivate` | IN31 contract; IN33 lock state (DQ-7) |
| Crime | vandalism (MWR1), LW10 | IN37 bash hook, IN14, owned breaks |
| Combat | C2c/C3 prop targets, poise (IN36) | prop targets, fire damage |
| Anim | Kimodo sit/lie/carry/astride clips, MG7b, `Docked` | slot frames, bands, IK targets |
| Nav | NV9, NV10 door state, IN22 review | door state, ladder slots |
| Physics, weather, lights, groundcover | contact events, wetness, wind, tint hook | `Wet`, `Burning`, `Lit`, burn grid |
| Net | N1-N5 (parked) | host claims, seeded events, links |

## 7. Owner questions (each with a recommended answer)

**Answered 2026-11-25 (owner): all five recommendations taken.** Home decorating is for much later. Jobs
(workstation activities) grow later with batch-generated animations, so they are **data**: a job is a mod file (TOML,
like Magna Urbis cultures) naming its slot kind, steps, clips, props and outputs, loaded through `sf_records` /
`sf_mods` and hot-reloaded in dev; adding a job never recompiles (IN42 builds the loader with the first jobs).

1. **Can spreading fire set town buildings alight?** Recommended: no; buildings burn only through authored stages
   (quests, sieges); furniture, crates, banners and grass in towns burn; the wild burns freely.
2. **Do thrown objects hurt and stagger people?** Recommended: yes, by mass × speed with a low cap; heavy objects
   stagger; a hit seen by others is assault; Classic off.
3. **Can you hold a door shut, wedge or barricade it?** Recommended: open, close, lock and bash first (NPCs open, queue
   and close behind them); wedge and barricade later as an option.
4. **Do NPCs visibly work at forges, alchemy tables and cooking pots, and does the player use the same object?**
   Recommended: yes; the player gets its Morrowind function (alchemy, repair), no new crafting in this lane.
5. **Home decorating (ghost preview and snap, Skyrim/Fallout)?** Recommended: later; carry and drop-to-rest for now.
