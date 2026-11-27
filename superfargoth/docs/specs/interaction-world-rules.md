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
| Impact | rapier `ContactForceEvent` over the material's threshold (IN4) | contact point | no |
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

## Reuse check (CONTRIBUTING rule 0)

- **Bevy 0.20:** `FixedUpdate`, messages and observers, `Image` assets for the burn texture; no chemistry, fire or
  gameplay light. Glue only. **Rapier:** `ContactForceEvent` with per-collider force thresholds is the impact trigger.
- **Our crates:** `sf_surface` (wetness), `sf_weather` (mean wind), `sf_effects` (flames, capped lights), the terrain
  and groundcover shaders (burn lookup), `sf_ai_bevy` `NoiseEvent` and `AiLightInputs` (host-mapped).
- **Crates:** no chemistry or fire-spread crate (research §7); particles are `sf_effects`' business.
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
| `interact.fire.light_radius_m` | m | 6 | 1-20 | Guess | Presentation |
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

TTL states save their remaining time, `Lit` and `Charred` as tags, burn cells as a sparse list (the `interaction`
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
