# Interaction (d): world rules (`sf_interaction`)

Status: design 2026-11-25 (IN28). Part (d) of the interaction spec: its shared core (defs, slot kinds, events, tick
and net rules), "Budgets" and "Build slices" (IN4, IN18, IN36, IN19, IN17, IN39-IN41, IN44) stay in
[interaction.md](interaction.md); destruction, which fire damages, is [interaction-destruction.md](interaction-destruction.md).
Lane roadmap: [interaction](../../../docs/roadmaps/interaction.md) decisions 6-8. Research:
[gameplay-2026-11-25/interaction](../../../docs/research/gameplay-2026-11-25/interaction.md) §1, §5.

<!-- agent-summary -->
> **Agent summary.** One rules table (`InteractionRule` records: source tags × target material + state → state tags
> with TTL, damage through `DestructionRules`, effect, `WorldChange`, noise), evaluated host-side on `FixedUpdate`.
> Fire is a deterministic heat model on props plus a coarse ground burn grid, spread budgeted per region, biased by the
> weather's wind, blocked by `Wet` (surface wetness); settlement buildings never catch from spread (authored stages
> only). `Lit` is a state tag: snuff/relight, the light follows, `LightChanged`; the shell's `GameplayLights` list is
> what the host copies into the AI's light inputs. Interaction noise is the shell's own `InteractionNoise` message
> (loudness = material reach × impact speed); the host maps it to `sf_ai_bevy`'s `NoiseEvent`. No core names `sf_ai`.
<!-- /agent-summary -->

## Scope

Owner (2026-11-25): budgeted fire spread, grass regrows fast; candles and lamps snuffed and relit, NPCs react to
darkness; thrown and smashed objects draw attention; towns burn only through authored stages. In: the rules layer and
its states (`Burning`, `Wet`, `Lit`, `Charred`), fire on props and ground, light as state, interaction noise. Later
rows of the same table: electricity, ice, surfaces (oil, pools). Out: building fire beyond authored stages (tier 3
parked, (b)), fluid fire or smoke simulation, rendered light feeding gameplay (net rule 3).

## Rules layer (IN17)

```rust
// Illustrative, not compiled. Records `sf.interact.rule` and `sf.interact.material`, mod-patchable.
pub struct InteractionRule { pub id: RecordId, pub source: TagFilter /* element, tool, projectile, impact, spell */,
                             pub target: TagFilter /* material + required / forbidden states */, pub result: RuleResult }
pub struct RuleResult {
    pub add: SmallVec<[(TagId, Ttl); 2]>,   // Ttl::Secs(f32) | Ttl::UntilRemoved
    pub remove: SmallVec<[TagId; 2]>,       // water on Burning: remove Burning, add Wet
    pub damage: Option<(TagId, f32)>,       // damage kind + amount (per hit, or per s while held) → DestructionRules
    pub stage: Option<u8>,                  // force a destruction stage (Charred), (b)
    pub effect: Option<TagId>,              // sf_effects id, seeded per client
    pub change: Option<TagId>,              // WorldChange kind
    pub noise: Option<TagId>,               // noise kind ("Noise")
}
pub struct MaterialDef { pub id: TagId /* wood, cloth, straw, glass, stone, grass */, pub flammable: Option<Flammable>,
                         pub noise: TagId, pub impact_reach_m: f32 /* reach at interact.noise.ref_speed_mps */ }
pub struct Flammable { pub ignite_heat: f32, pub fuel_s: f32, pub burnt_stage: Option<u8>, pub spread: bool }
```

- **One material per object.** IN17 moves `material` from `DestructibleDef` up to `InteractableDef` (a banner burns
  without being destructible). Ground cells get theirs from the adapter (terrain texture, groundcover species).
- **States are `TagId`s with TTL** (Gameplay Tags with duration): a core `ObjectStates` table by `PlacementId`, a few
  `(tag, expires_tick)` per object. Verbs gate on them (`VerbDef::requires`: snuff needs `Lit`).
- **Evaluation** (host only, rule 5; pure core `step_rules(&mut ObjectStates, causes, &Rules, tick)`): the shell
  gathers the tick's causes (hits, effect areas with source tags, fire heat, rain, water contact); the core matches
  them in record order (first match per cause and target; mods reorder by patching), applies results, expires TTLs,
  returns events. At most `interact.rules.evals_per_tick` matches per tick, the rest wait (oldest first). Same inputs,
  same result at any frame rate. Visuals (flames, smoke, char) are seeded per client from `hash(placement, state,
  tick)` (rule 4); only state changes replicate.

**As built (IN17a, 2026-11-27).** `InteractableDef::material: Option<TagId>` (`#[serde(default)]`: older data loads
`None`), gone from `DestructibleDef`. `sf_interaction/src/states.rs`: `ObjectStates` (per placement a tag-sorted list
plus a `(tick, placement, tag)` deadline set), `add(placement, tag, Ttl, now, params) -> bool` (Secs = whole ticks at
`interact.tick_hz`, at least one; refresh keeps the later expiry, `UntilRemoved` wins), `remove`, `has`, `tags`, `expire(now)`
(`expires <= now`, sorted). 4 exact tests (`tests_states.rs`). Rules, save codec and replication: IN17b on.

**As built (IN17b, 2026-11-27).** `sf_interaction/src/rules.rs`: `InteractionRule`, `RuleResult` (serde-default
fields), `MaterialDef`, `Flammable`, `Rules` (record order), `Cause { placement, material, source }` queued in a
`RuleQueue` and `step_rules(&mut ObjectStates, &mut RuleQueue, &Rules, now, &InteractParams) -> Vec<RuleEvent>`:
expiries first, then causes in (queued tick, placement, material, sorted source) order; the target filter sees the
cause's material plus the states; results apply remove, add, damage, stage, effect, change, noise; state events only
for real changes. The budget counts evaluated causes, matched or not (`interact.rules.evals_per_tick`, the scan is the
cost). `wet_threshold` / `wet_ttl_s` are bound for the shell's wetness causes (IN39). 5 exact tests (`tests_rules.rs`,
incl. all 120 orders of 5 causes). No system gathers causes yet; save codec and replication follow with IN39.

## Fire (IN17 core, IN39 in game, IN41 adapter)

- **Ignition.** A cause matching a rule that adds `Burning` to a flammable material: a torch or fire-spell hit, a
  neighbour's heat, a lamp knocked onto straw. The adapter's rows decide what ignites; Classic ignites nothing.
- **Heat, not dice.** Burning objects and cells heat flammable targets within `interact.fire.spread_radius_m`:
  `heat += interact.fire.heat_per_s × falloff(d) × wind(θ) × (1 ± jitter)` per tick, jitter seeded by `hash(target,
  second)` (`interact.fire.jitter`); ignition at the material's `ignite_heat`; heat decays at `interact.fire.cool_per_s`
  with no source near. Burning lasts `fuel_s`, deals its rule's damage per second (`DestructionRules`), then swaps to
  `burnt_stage` (`Charred`) or the last destruction stage.
- **Wind.** `wind(θ) = max(0, 1 + interact.fire.wind_bias × cos θ × min(|w| / interact.fire.wind_ref_mps, 1))`, `w`
  the weather's mean wind (`sf_weather`, pure from seed and clock; gusts are presentation), θ the angle to the source →
  target direction. Fire runs downwind, creeps upwind.
- **Wet blocks it.** Outdoors a target gets `Wet` (TTL `interact.rules.wet_ttl_s`, refreshed) while the surface
  wetness at it (`sf_surface::sample`, SU1) passes `interact.rules.wet_threshold`, and on water contact; `Wet` takes no
  heat, and a burning object past `interact.fire.rain_douse` goes out (rain stops it: IN17's verify). Indoors (the
  host's interior flag) rain never wets; outdoors a roof is one upward ray per fire candidate, cached while it rests.
- **Extinguish.** Rule rows (water, frost, smother, the adapter's spells) remove `Burning` and add `Wet`.
- **Budget by region** (`interact.fire.region_m` squares): at most `interact.fire.burning_max` burning objects (more
  ignitions queue) and `interact.fire.lights_max` fire lights (the hottest; the rest are emissive flames). Heat checks
  (pairs from `PointIndex` grid buckets and cell neighbourhoods) are capped at `interact.fire.spread_checks_per_tick`,
  round-robin; a skipped pair is checked later with the elapsed time, so the cap slows fire, never redirects it.
- **Settlements (owner 2026-11-25).** The adapter gives settlement zones and marks defs or materials `structure`. In a
  zone a `structure` never takes heat or `Burning` from spread: it burns only through an authored stage (quest or
  script, `Destroyed`/`WorldChange` as (b)). Furniture, crates, banners, hay and grass in towns burn; the wild burns
  freely.
- **Static vegetation** (trees, shrubs) is ground-cell fuel, not a fire carrier: a burnt cell tints, and swaps its
  instances to a charred variant through the static-scene overlay when the adapter gives one.

**As built (IN17c1, 2026-11-27).** `sf_interaction/src/fire.rs`: `step_fire(&mut Fire, &mut ObjectStates, &[FireBody
{ placement, pos, flammable, wetness }], &FireEnv { wind }, now, params)`: douses (wetness >= `rain_douse`: `Burning` off,
`Wet` for `wet_ttl_s`), burn-outs (`fuel_s` from the tick first seen `Burning`, then `Charred`, `Burnt { stage }`), then
heat from the start-of-tick burning set: `falloff(d) = 1 - d / spread_radius_m` (linear; our choice), jitter `j` from
`splitmix64` over `(placement, second)`, cooling with no source in reach; ignitions apply last. Wet, burning and charred
bodies take no heat. Bodies sorted by placement, so arrival order is irrelevant. 7 `interact.fire.*` tunables bound.
5 exact tests (`tests_fire.rs`). Budgets IN17c2; damage per second and wetness -> `Wet` are the host's (IN39).

**As built (IN17c2, 2026-11-27).** `fire.rs`: `fire_region(pos)` (`region_m` squares, XZ); burning bodies per region
(rule-lit too) capped at `burning_max`, heat ignitions past it queue (`Fire::queued()`, drained oldest first before the
tick's new ignitions; `Wet`/`Charred`/rule-lit/unsent leave it, queued bodies take no heat). Heat checks = source -> target
distance tests from 3 x 3 buckets of `spread_radius_m`, at most `spread_checks_per_tick` (at least one target), round-robin
in placement order from a cursor; a target catches up with its elapsed ticks, each source clamped to the ticks it heated,
so with room for every check the result is bit-exact IN17c1. `Flammable::structure` (serde default) +
`FireBody::in_settlement` (host's zone lookup) = immune to spread. 3 exact tests in `tests_fire.rs`.

### Rules and fire shell (IN39, spec 2026-11-27)

**Checks.** Bevy 0.20: `FixedUpdate`, messages, resources; nothing fire-shaped. Our crates: `sf_effects` is the
actors' magic-effect step (`sf_effects/src/lib.rs:1`), not flames; `sf_surface_bevy`'s shelter is a GPU depth capture
(`lib.rs:8`), no CPU query; `sf_weather` gives the mean wind (`map.rs:305` `wind()`); the shell depends on none of
them (render stacks; the `DeformGround` closure pattern, `sf_surface_bevy/src/deform.rs:178`, instead). Crates:
`bevy_hanabi` (MIT/Apache GPU particles) shows no Bevy 0.20 release (README table ends at 0.18, read 2026-11-27, U),
so the flame presenter starts from the pack's flame meshes and is its own row's reuse check. Industry: Far Cry 2's
fire ([Lévesque interview](https://gamedeveloper.com/game-platforms/interview-how-i-far-cry-2-i-s-fire-fuels-spreads):
a year and a half on it, fire as an area "fear spell" AI flees; wind-driven spread through bushes,
[PC Gamer](https://www.pcgamer.com/great-moments-in-pc-gaming-watching-fire-spread-in-far-cry-2)), plus an
unattributed [write-up mirror](https://symmetricaldatasecurity.blogspot.com/2020/12/far-cry-how-fire-burns-and-spreads.html)
(original not found, U): cells with hit points that burning neighbours drain, more downwind, faster uphill, emitters
kept in front of the camera. Ours: heat = their hit points, the same wind bias, AI reads burning objects as lights and
`Ignited` changes, the presenter picks lights near the camera. Uphill bias is not modelled (gap from the U source,
industry-audit.md when IN39b lands).

**Host inputs** (resources the game fills; the shell never names weather, surface or physics):
`InteractionRules(Rules)`; `FireWeather { wind: Vec2, indoors: bool }` (mean wind at the player, the active space's
interior flag); `FireWetness`, `FireShelter`, `FireZones`: `Option<Arc<dyn Fn(Vec3) -> f32 | bool + Send + Sync>>`
(`sf_surface::sample`'s wet channel; one upward ray on the host's physics probe; the adapter's settlement zones).
Unset closures = dry, open sky, wild. Materials: `InteractionMaterials` (IN18b, shared). **No new registry** (review):
a placement's def and frame are already `InteractionDefs` (`activate.rs:57`) and `InteractionFrames::current`
(`lib.rs:458`), both kept by `step` in `Apply` from every `IndexChange`. The fire shell keeps only `FireCandidates`
(placements whose def material is `flammable` in `InteractionMaterials`, from `IndexChange`, rebuilt when the materials
change) with their shelter and zone answers, dropped on `Insert`/`Move` ("cached while it rests").

**IN39a rules shell.** `InteractionObjectStates(ObjectStates)` + `RuleCause { placement, source, culprit:
Option<UserKey> }` (hits, spell areas, water contact, a torch; any writer), material from `InteractionDefs`
(unindexed: dropped), into the core `RuleQueue`; `step_rules` in `RuleSystems`. **Core changes (review):** (1) the
core `Cause` carries `culprit: Option<UserKey>` (part of its sort key) and `RuleEvent` gains `culprit` and `source` (the
cause's sorted tags): the queue sorts and defers causes past `evals_per_tick`, so the shell cannot map an event back to
its cause today (`rules.rs:181` has only placement + rule). (2) **Host-owned tags:** `LIT` (and later door tags) live
in `Lights` / the door registry, not in `ObjectStates`, so a rule needing `Lit` never matched and `remove: [Lit]`
emitted nothing (`states.remove` false). `Cause::host` = the placement's host-owned tags now set (the shell copies them
from `InteractionStateTags`), `step_rules(.., host_owned: &[TagId])`: the target filter sees states + material + host;
a host-owned tag is never stored: `Removed(t)` iff it is in `host`, `Added(t)` iff not. Every event is one `RuleApplied
{ tick, event }` (culprit, source inside); routes: Added/Removed/Expired of rule tags mirror into
`InteractionStateTags` (verbs gate on `Burning`, `Wet`); `LIT` on a registered light -> `LightState::set` (`light.rs:128`)
+ the light registry's `sync` (tag, `GameplayLights`) + `LightChanged` (cause: the source's first tag in id order, a
stable but arbitrary pick: rules wanting a named cause set `change`); `Damage` -> one `PropDamage` (per evaluation; a
held source re-sends its cause each tick); `Stage(i)` -> `PropStage`; `Change(k)` -> the change inbox (below); `Noise(k)`
-> `NoiseInbox` (material reach, maker the culprit, not revealed); `Effect(e)` -> `RuleEffect { tick, placement,
effect, at }` for the host's presentation.
**`PropStage { placement, stage, kind, culprit, point }`** (`stage` = index into the def's `stages`, as `Destroyed::stage`
and `Flammable::burnt_stage`): a new core `force_stage(def, state, index) -> Hit`: `stage := max(state.stage, index + 1)`,
`health := min(health, def.health x stages[index].at)`, `crossed = from..to`, overkill 0; the shell queues the crossed
stages as a hit's. Not `apply_damage` with `health - def.health x at`: `fraction <= at` after that float subtraction can
miss the stage by one ulp (review). Already reached, gone, unknown, or `ignore_external` (but `DAMAGE_SCRIPT`): nothing.
**As built (IN39a1, 2026-11-27).** `rules.rs`: `Cause { .., culprit, host }` (both in the sort key; `push` sorts and
dedups `host` too), `RuleEvent { .., culprit, source }` (expiries: `None`, empty), `step_rules(.., host_owned)`; the
core `force_stage` takes the hit's `kind` too (`force_stage(def, state, index, kind)`), so the `ignore_external` gate
is the core's; "gone" = a reached stage has no model. Tests: `tests_rules.rs` deferred culprit/source, host-owned
`Lit`; `tests_destroy.rs` a 37-health def where `37 - 33.3` leaves the fraction above 0.1 and `force_stage` reaches it.
**As built (IN39a2, 2026-11-27).** `sf_interaction_bevy/src/rules.rs`: `gather -> step -> route` chained in `RuleSystems`
(`HOST_OWNED = [LIT]`; door tags later), `InteractionRuleQueue` keeps deferred causes; the shell `LightChanged` gained a
second field, the rule cause (`None` for a use), so `from_light` gets it; `PropStage` runs after the tick's `PropDamage`
in `DestroySystems::Damage`; damage/stage kind = the source's first tag (`RULE_SOURCE` without one), rule noise =
`NoiseCase::Impact` at the material's impact reach; `ChangeInbox` drained after doors, lights, breaks. 5 headless tests.

**Order (review)**, one chain in `FixedUpdate`: `InteractionSystems::Apply` -> `RuleSystems` -> `FireSystems` ->
`DestroySystems::Damage` (reads the tick's rule and fire `PropDamage` / `PropStage`) -> `Release` ->
`WorldChangeSystems`. Rule and fire changes go into a `ChangeInbox` the world-change system drains with the door,
light and destroy ones, so one stream per tick is sorted and folded (`fold_tick`), not two.

**IN39b fire step.** Bodies = `FireCandidates` at `InteractionFrames::current`'s translation, in `FireSystems`, on the
same `ObjectStates`. Wetness 0 indoors or sheltered, else `FireWetness(pos)`; at `>= interact.rules.wet_threshold` the
shell refreshes `Wet` (`wet_ttl_s`) before `step_fire` (`FireEnv { wind }`). `step_fire` changes states without state
events, so the shell diffs `ObjectStates::tags` before/after for each placement with a refresh or a `FireEvent` and
mirrors the diff into `InteractionStateTags`. Culprits (`FireCulprits`, placement -> (user, cause tag)): a rule-lit body
takes its `RuleApplied` culprit and source tag; a heat ignition inherits the nearest `spread` body burning at the tick's
start within `interact.fire.spread_radius_m` (the ones that could heat it; distance, then placement; none in reach: no
culprit). `Ignited` (heat, or `RuleApplied` Added(`BURNING`), built as a `FireEvent`) and `Burnt` -> `from_fire` (site from
`ChangeOwners`, at the body) into the change inbox; Doused and Burnt forget the entry after their event.

**IN39c fire outputs.** Each burning destructible takes `PropDamage { kind: DAMAGE_FIRE ("fire", new core const),
amount: interact.fire.damage_per_s / tick_hz, point: pos, dir: +Y, culprit }` (the adapter's `DestructionRules (fire,
material)` scales it). `Burnt` -> `PropStage` (`burnt_stage`, else the def's last stage); a `Destroyed` of kind fire on
a `Charred` placement maps to no `WorldChange` (its `Burnt` said it). `BurningObjects` (placement -> at, since, region,
`light`): `light` on the `interact.fire.lights_max` earliest-started per region (then placement; a light keeps its slot
until it goes out, no flicker); `FireChanged { tick, placement, output, at }` for the presenter; a burning placement
that is not a registered light joins `GameplayLights` (`interact.fire.light_radius_m`, intensity 1) while it burns
and leaves it on Doused, Burnt or its `Remove` (the light registry's `drop_light` covers only registered lights).

**IN39d save.** Core `ObjectStates::rest_bytes(placement, now)` / `from_rest_bytes(.., now)` (version, then per state
tag id + remaining ticks, `u32::MAX` = until removed) and `Fire::rest_bytes` / `restore` (version, heat bits, elapsed
burn ticks or none (restores `since` and `heats_from`, equal outside a restore), queued flag; `checked` restarts at the
load tick and the round-robin cursor is not saved: a restored target misses no more than one tick of catch-up): remaining time, never absolute ticks, so a save loads exact at any tick count. Shell:
`ObjectStatesChanged { placement, bytes }` on change (the host's overlay blob, (b) "Save"), `ObjectStatesRest` applies
one; an unloaded placement drops its states and fire after its last write. No blob = fresh. Fire runs on ticks while
resident; a clock jump (rest, travel) advances nothing (Far Cry 2 burns only near the player too).

**Later rows (game, `mw_interaction_bevy`).** IN39e: the host fills `FireWeather`, `FireWetness`, `FireShelter`,
`FireZones` (sf_weather, sf_surface, its probe, IN41's zones). IN39f: the presenter (flame model + smoke per
`BurningObjects` within `interact.fire.effect_lod_m`, shadowless point lights on the `light` ones, char from the stage
swap), owner check IN-39. **New tunables** (rows below): `interact.fire.damage_per_s`; `lights_max`, `light_radius_m`
(IN39c) and `effect_lod_m` (IN39f) get declared with their rows.

### Ground burn grid (IN40)

Per region a coarse grid (`interact.burn.cell_m` cells, `interact.burn.region_cells` a side; one byte of burn, one of
fuel): fuel from the adapter (groundcover density, terrain material); Fresh → Burning (`interact.burn.cell_burn_s`) →
Burnt → regrown to Fresh over `interact.burn.regrow_h` game hours (owner: fast; closed form in elapsed hours, so clock
jumps cost nothing). Burning cells heat neighbours and props above them like any source; at most
`interact.burn.updates_per_tick` cell updates per tick (dirty list). **Presentation:** an R8 burn texture per resident
region, dirty rows uploaded at `interact.burn.upload_hz`; the terrain shader darkens albedo by `burn ×
interact.burn.tint`, groundcover shortens and darkens blades and culls them above `interact.burn.cull` (one texture
fetch, the surface-wetness pattern; no per-blade state). Saved as a sparse list of non-Fresh cells with their burn tick.

## Light as state (IN19)

- **Def.** The `Light` slot kind carries `LightDef { radius_m, intensity, lit_default }` (adapter data). Verbs snuff
  and relight, gated on `Lit`, one user at a time (claim). Rules toggle it too (water snuffs a candle, fire lights a
  torch).
- **State.** `Lit` is a saved tag without TTL, set from `lit_default` on first load, then from the overlay. A change
  emits **`LightChanged { tick, placement, lit, at, culprit: Option<UserKey> }`** (rule 8) and a `WorldChange`
  (`Snuffed` / `Relit`) for AI28 `Darkened` and AI27 dusk lamps.
- **Light follows.** The shell spawns no render light: the host's light hook turns the placement's light, flame and
  glow on or off on `LightChanged`.
- **AI light, never from the GPU.** The shell keeps **`GameplayLights`**: per light-bearing placement in resident cells
  `{ placement, pos, space, radius_m, intensity, lit }` (moving owners and held props follow; burning fire lights join),
  updated on load, unload, `LightChanged` and moves. Its fields match `sf_ai::LightSource` (`sf_ai/src/light.rs:110`),
  but the type is the shell's, so `sf_interaction(_bevy)` never names `sf_ai`. **The host wires it:** the system that
  writes `AiLightInputs` (`sf_ai_bevy/src/light.rs`; Bevyrrowind `MA16`, `mw_walkers_bevy/src/light_inputs.rs`) copies
  the active space's entries into `sources` in place of its own placed-light list and record "on" flag; equipped
  lights (equipment, not interaction props) stay the host's. `LightChanged` reaches the darkness concern the same way.

**As built (IN19a, 2026-11-27).** `sf_interaction/src/light.rs`: `SlotKind::Light(LightDef)`, `LIT`, `VERB_SNUFF` /
`VERB_RELIGHT`, `LightState::{initial, tag, use_verb, set}` (`use_verb` refuses `AlreadyUnlit` / `AlreadyLit` /
`NotALightVerb` and changes nothing; `set` for rules emits only on a change), `LightChanged`, the one-byte rest state
(`LIGHT_STATE_OWNER` = `light`, bytes 0/1, others rejected). `at` is the world-frame `Vec3` of the core. Tests:
`tests_light.rs` (6 exact). Next: the shell (IN19b).

**As built (IN19b, 2026-11-27).** `sf_interaction_bevy/src/light.rs`: `Lights` (first `Light` slot; a re-insert keeps
the state, else `lit_default`), the `Lit` tag and `GameplayLights { placement, pos, radius_m, intensity, lit }` kept in
the same system on insert, move, remove, use and `LightRest` (a restore, no event). Default after
`ActivationSystems::Default`: snuff / relight (no verb toggles), culprit = the actor's `InteractionUser`. The state gate is
the verbs' `requires` (snuff `all: [Lit]`, relight `none: [Lit]`); the core refuses what passes none (a script's). **No
`space` field:** the index holds only the active space's resident ring, so the host copies every entry. `WorldChange`
waits for its type (AI28). Tests: `tests_light.rs` (6, whole pipeline).

## Noise (IN18; impacts IN4, people IN36)

| Case | Trigger (shell) | `at` | Reveals the maker |
|---|---|---|---|
| Impact | `PhysicsEvent::Contact` at `speed` >= `interact.noise.min_speed_mps` (IN4 reads the same reports) | contact point | no |
| Thrown landing | first impact of a thrown prop | landing point | no |
| Drop | a released held prop landing | landing point | if the dropper is within `interact.noise.reveal_m` |
| Break | `Destroyed` (b) | object | no (yes for a melee break) |
| Door | `DoorChanged` (IN33) | doorway | yes |
| Bash | each lock or door blow (IN37) | lock | yes |
| Lock pick | an attempt through the activation gate (crime LW13) | lock | yes |
| Fire | an object or cell igniting | object | no |

- **Loudness.** `loudness_m = material.impact_reach_m × clamp(v / interact.noise.ref_speed_mps, 0,
  interact.noise.speed_mul_max)`, `v` the normal impact speed, nothing under `interact.noise.min_speed_mps` (reach ∝
  sound pressure ∝ impact speed, inverse-distance law). Doors, bashes and picks use their reach tunables × the
  material's reach ratio. One object's impacts within `interact.noise.merge_s` merge into the loudest; at most
  `interact.noise.per_tick_max` per tick (loudest kept).
- **The message.** `sf_interaction_bevy` writes **`InteractionNoise { tick, at, space, kind: TagId, loudness_m,
  maker: Option<UserKey>, reveals_maker, case }`** on `FixedUpdate`, before the host's perception step.
- **Decision: the host reports it to perception** (UE `ReportNoiseEvent`: gameplay code that makes a noise reports it
  to the perception system, which owns the stimulus type). Our perception vocabulary is `sf_ai_bevy::NoiseEvent`
  (`hearing.rs:51`), and the host already reports gait and landing noises with it (AI8c2,
  `mw_walkers_bevy/src/noise_emitters.rs`); its emitter maps `InteractionNoise` one to one (`TagId` → its
  `NoiseKindId` table, `maker` → `PerceptTarget`, fields copied). Rejected: the shell writing `NoiseEvent` itself (a
  shell dependency on `sf_ai_bevy`, a cycle once `sf_ai_bevy` runs `UseSlot` on interaction claims, IN8, and AI compile
  weight on every interaction build); a shared vocabulary crate (moves `NoiseKindId`, `PerceptTarget` out of `sf_ai`
  for one field copy).
- **Distraction, not blame.** A thrown or dropped prop's noise names the thrower as `maker` (an actor never hears its
  own noise) with `reveals_maker = false`: listeners fill their `Distraction` slot (ai.md), never the thrower's entry.
  The landing also emits `WorldChange::Landed { culprit }`: a witness who saw the throw (crime `WitnessQuery`,
  world-reactions.md) raises `Nuisance` (AI28); IN36 makes a seen hit on a person an assault.

**As built (IN18a, 2026-11-27).** `sf_interaction/src/noise.rs`: `loudness_m(reach, v, params) -> Option<f32>` (none
under `min_speed_mps`, for a silent material or non-finite input; `max`/`min`, so a NaN tunable is silent, never a
panic); `Noise { source, at, kind, loudness_m, maker, reveals_maker, case: NoiseCase }` (the shell adds tick + space);
`NoiseMerge::step(tick, noises)`: one per object a tick (its loudest), inside the object's `merge_s` window (from its
first heard noise, at least one tick) only a louder one is heard (nothing is held back, the loudest is never missed),
then the `per_tick_max` loudest; a cut noise opens no window; a total order on every field. The 9 `interact.noise.*`
on `InteractParams`, bound in `bind_rules`. 5 exact tests (`tests_noise.rs`). The material reach ratio for doors,
bashes and picks is the shell's (IN18b).

**As built (IN18b, 2026-11-27).** `sf_interaction_bevy/src/noise.rs`: `InteractionMaterials` (host-filled
`MaterialDef`s by id; a material's ratio = its `impact_reach_m` / `interact.noise.material_ref_m`, the reference
material the door, bash and pick reaches are set for; none or unknown = 1), `NoiseInbox` (the tick's `Noise`s + the
merge; later bash and pick rows `push`), `InteractionNoise` written in `NoiseSystems::Emit` (after `DoorSystems::Commit`,
`DestroySystems::Release`, `ImpactSystems`). Door: at the doorway, door reach x ratio, kind `noise.door`, revealed. Break:
at the object, material reach x `speed_mul_max`, kind `noise.break`, culprit as maker, not revealed (melee reveal waits for
the damage kind on `Destroyed`). Impact (`physics`): one noise per contact, the louder side of indexed placements with a
known material, kind its `noise`; a body in `Thrown` gives `ThrownLanding` naming the thrower; debris pieces,
`PROJECTILE`, `HELD` silent; a changed active world clears the windows. `space` is `None` (the active worldspace). Not
yet: the `Drop` case (dropper within `reveal_m`). 4 headless exact tests (`tests_noise.rs`).

**As built (IN18c, 2026-11-27).** `mw_interaction_bevy/src/noise.rs` (`sf_ai_bevy` has no interaction edge, so no
cycle): each `InteractionNoise` -> one `NoiseEvent` (`at` as `WorldPos`, `space`, `loudness_m`, `reveals_maker` copied;
`maker` -> `PerceptTarget`, same keys; no `notify`), stamped with the perception clock's tick (the stamp every `NoiseEvent`
carries), kinds from `InteractionNoiseKinds` (`noise.door`, `noise.break`, `noise.object` from id 256, disjoint from the
player's 0..; a material's tag `register`ed; unknown = `noise.object`) until `NoiseKind` records exist. Order: after
`NoiseSystems::Emit` and `AiPerceptionSystems::Hearing` (heard next perception tick, deterministic; same tick would need the
door/destroy/impact steps before hearing, which the physics order does not promise). 2 headless exact tests.

## Reuse check (CONTRIBUTING rule 0)

- **Bevy 0.20:** `FixedUpdate`, messages and observers, `Image` assets for the burn texture; no chemistry, fire or
  gameplay light. Glue only. **Rapier:** contacts come through `sf_physics` `PhysicsEvent::Contact` (solver impulse + normal speed change), not
  `ContactForceEvent` (fires every step a resting load passes its force threshold; destruction spec "Impacts (IN4)").
- **Our crates:** `sf_surface` (wetness), `sf_weather` (mean wind), fire presentation is the host's (IN39f; `sf_effects` is the actors' magic-effect step, not flames), the terrain
  and groundcover shaders (burn lookup), `sf_ai_bevy` `NoiseEvent` and `AiLightInputs` (host-mapped).
- **Crates:** no chemistry or fire-spread crate (research §7); `bevy_hanabi` for particles is the presenter's check (IN39f; no Bevy 0.20 release seen 2026-11-27).
- **Projects:** OpenMW has no fire spread or snuffing (lights are records with an off-by-default flag); nothing to
  port. So: **our own** rules in `sf_interaction`, thin systems in `sf_interaction_bevy`.

## Industry comparison

[M] = from memory, unverified; [S] = secondary source (research §1).

| System | What it does | What we take |
|---|---|---|
| BotW/TotK chemistry [S] | elements change materials' state by rules, elements change each other, materials never directly; proximity spread | the table shape: elements = source tags, materials = target tags, no per-object scripts |
| Divinity OS2 / BG3 surfaces [S] | ground grid of surface tags with a transform table | the burn grid as a coarse tag carrier; more surfaces later on it |
| UE Gameplay Tags [M] | hierarchical tags as predicates, effects add tags with duration | states = `TagId` + TTL; filters gate verbs and rules |
| UE AIPerception [M] | gameplay calls `ReportNoiseEvent(loudness, location, instigator)` | host reports `InteractionNoise` as `NoiseEvent`; thrower kept, not revealed |
| Far Cry 2 fire [M] | grid propagation over vegetation, wind-driven, fuel burns out, capped | heat on a coarse grid, wind bias, fuel time, region caps; deterministic for net |
| Thief / Dishonored [M] | light level from game lights (light gem), lamps snuffed, NPCs relight | `Lit`, gameplay lights for the AI, darkness concern, relight method |
| Skyrim [M] | placed candles and torches, no snuffing; fire spells ignite only oil | we go further; Classic burns nothing |

Gaps go in [industry-audit.md](industry-audit.md) when built (none known).

## Tunables (tunables.md; framework values, a pack's base mod may override)

| Name | Unit | Default | Range | Source | Net |
|---|---|---|---|---|---|
| `interact.rules.evals_per_tick` | count | 64 | 8-1024 | Guess | Gameplay |
| `interact.rules.wet_threshold` | ratio | 0.3 | 0-1 | Guess (sf_surface wetness scale) | Gameplay |
| `interact.rules.wet_ttl_s` | s | 120 | 5-900 | Guess | Gameplay |
| `interact.fire.region_m` | m | 64 | 16-256 | Guess | Gameplay |
| `interact.fire.burning_max` | count | 24 | 2-128 | Guess (Industry: Far Cry 2 capped fires [M]) | Gameplay |
| `interact.fire.lights_max` | count | 4 | 0-16 | Guess (shadowless, lights.md) | Presentation |
| `interact.fire.spread_checks_per_tick` | count | 48 | 4-512 | Guess | Gameplay |
| `interact.fire.spread_radius_m` | m | 2.5 | 0.5-10 | Guess | Gameplay |
| `interact.fire.heat_per_s` | ratio/s | 0.5 | 0.05-5 | Guess | Gameplay |
| `interact.fire.cool_per_s` | ratio/s | 0.2 | 0-5 | Guess | Gameplay |
| `interact.fire.jitter` | ratio | 0.25 | 0-1 | Guess | Gameplay |
| `interact.fire.wind_bias` | ratio | 0.6 | 0-1 | Guess (Industry: Far Cry 2 wind-driven [M]) | Gameplay |
| `interact.fire.wind_ref_mps` | m/s | 8 | 1-30 | Guess | Gameplay |
| `interact.fire.rain_douse` | ratio | 0.6 | 0-1 | Guess | Gameplay |
| `interact.fire.light_radius_m` | m | 6 | 1-20 | Guess (also the AI's fire light reach, IN39c) | Gameplay |
| `interact.fire.damage_per_s` | 1/s (raw damage) | 5 | 0-100 | Guess (the adapter's `DestructionRules (fire, material)` scales it) | Gameplay |
| `interact.fire.effect_lod_m` | m | 60 | 10-300 | Guess | Presentation |
| `interact.burn.cell_m` | m | 2 | 0.5-8 | Guess (Industry: Divinity surface cells [S]) | Gameplay |
| `interact.burn.region_cells` | count | 256 | 32-1024 | Guess (= 512 m at 2 m) | Gameplay |
| `interact.burn.cell_burn_s` | s | 8 | 1-60 | Guess | Gameplay |
| `interact.burn.regrow_h` | h (game) | 48 | 1-720 | Guess (owner 2026-11-25: regrows fast) | Gameplay |
| `interact.burn.updates_per_tick` | count | 256 | 16-4096 | Guess | Gameplay |
| `interact.burn.upload_hz` | Hz | 4 | 1-30 | Guess | Presentation |
| `interact.burn.tint` | ratio | 0.85 | 0-1 | Guess | Presentation |
| `interact.burn.cull` | ratio | 0.7 | 0-1 | Guess | Presentation |
| `interact.noise.min_speed_mps` | m/s | 1 | 0-5 | Guess | Gameplay |
| `interact.noise.ref_speed_mps` | m/s | 5 | 1-20 | Guess | Gameplay |
| `interact.noise.speed_mul_max` | ratio | 2 | 1-4 | Guess | Gameplay |
| `interact.noise.merge_s` | s | 0.25 | 0-2 | Guess (one rattle, one noise) | Gameplay |
| `interact.noise.per_tick_max` | count | 16 | 1-128 | Guess | Gameplay |
| `interact.noise.reveal_m` | m | 1.5 | 0-5 | Guess | Gameplay |
| `interact.noise.door_reach_m` | m | 8 | 0-40 | Guess | Gameplay |
| `interact.noise.bash_reach_m` | m | 20 | 0-60 | Guess | Gameplay |
| `interact.noise.lockpick_reach_m` | m | 3 | 0-20 | Guess | Gameplay |
| `interact.noise.material_ref_m` | m | 10 | 1-60 | Guess (wood: the reach the door, bash, pick values are set for) | Gameplay |

All `Scope::Dev`, `Lifetime::Live` except `interact.burn.cell_m` and `interact.burn.region_cells` (`Lifetime::Restart`:
they size the grid). Material values, rule rows and `LightDef` are mod data, not tunables.

## Budgets (High, 1440p + DLSS Quality, RTX 4060 laptop, ≤ 8.3 ms median, ≤ 11 ms p99; targets for IN44)

- **Rules:** ≤ `interact.rules.evals_per_tick` matches plus TTL expiry from a sorted list: < 0.05 ms per tick.
- **Fire:** ≤ `interact.fire.burning_max` burning objects per region, ≤ `interact.fire.spread_checks_per_tick` heat
  checks (grid buckets, no physics queries): < 0.1 ms per tick at a full blaze. Beyond `interact.fire.effect_lod_m` an
  emissive card; ≤ `interact.fire.lights_max` shadowless fire lights per region (lights.md's caster budget untouched);
  GPU < 0.3 ms for flames, smoke and lights in the worst resident blaze.
- **Burn grid:** ≤ `interact.burn.updates_per_tick` cells per tick (< 0.05 ms); dirty-row uploads (≤ 64 KB per region)
  at `interact.burn.upload_hz`; one texture fetch in the terrain and groundcover shaders.
- **Lights, noise:** `GameplayLights` on change only; ≤ `interact.noise.per_tick_max` noises, each a field copy.
- **IN44** measures rules, spread and burn grid (main and GPU ms) on High in a synthetic blaze scene; results in
  [perf.md](../../../docs/ledger/perf.md).

## Save and net

TTL states save their remaining time (IN39d codec), `Lit` and `Charred` as tags, burn cells as a sparse list (the `interaction`
overlay blob, (b) "Save"); old saves load Fresh with lights at `lit_default`. Host-authoritative: clients receive
state changes, `LightChanged`, burn-cell deltas (batched at `interact.burn.upload_hz`) and noises; flames and char
are seeded per client.

## For IN29 (adapter) and IN30 (world reactions)

- **IN29 / IN41 (`mw_interaction`):** material rows (pack objects, terrain textures and groundcover species →
  material, `flammable`, `noise`, `impact_reach_m`); ignition sources as rule rows (torches, fire spells, fire
  damage); `LightDef` from the pack's light records (`lit_default` from its off-by-default flag); settlement zones and
  `structure` marks (town rule); Classic switches spread and snuffing off; the host's `AiLightInputs` writer reads
  `GameplayLights`; the host noise emitter maps `InteractionNoise` and owns the `TagId` → `NoiseKindId` table.
- **IN30 (world-reactions.md):** `WorldChange` kinds `Ignited`, `Burnt`, `Snuffed`, `Relit`, `Landed` (with culprit)
  feeding AI28's `Fire`, `Darkened` and `Nuisance`; a `burnt` reset class (IN15) for charred objects.
