# Interaction research: world interactivity for player and NPCs (2026-11-25)

Scope: `superfargoth/docs/specs/interaction.md` (IN0-IN13), `world-reactions.md` (IN14-IN16, AI19-25),
`bevyrrowind/docs/specs/interaction-adapter.md`. Tags: [P] primary or official doc fetched, [S] secondary press or wiki,
[M] from memory, unverified. Research only, nothing built.

## 0. Correction and verdict in one paragraph

- **Physics is not Avian here.** `Cargo.toml` pins `rapier3d` 0.36.1 under our own `sf_physics`; avian stays "fallback" (physics.md
  line ~64). Anything Avian-only below is a design reference, not a dependency.
- The spec is strong on **slots, claims, carry, stage-swap destruction, persistence classes** (all on par with UE Smart Objects,
  Skyrim `DEST`, Chaos-lite). It is **silent on the second half of "systemic"**: material x element rules (fire, wet, electric,
  light), noise from interactions, doors/levers/ladders/stations as slot kinds, prompt/verb UX. Greps of both specs find zero hits
  for ladder, lever, workstation, lamp/candle, priority, noise, burn, wet. Those are the gaps that change what the player can do.

## 1. Systemic interaction models

| Source | Model | Lesson for us |
|---|---|---|
| BotW/TotK chemistry [S: [GamesBeat](https://gamesbeat.com/the-legend-of-zelda-breath-of-the-wild-makes-chemistry-just-as-important-as-physics/), [Thumbsticks](https://www.thumbsticks.com/gdc-17-breath-of-the-wild-science-lies); GDC 2017 talk itself not reached] | Physics = rule-based movement, chemistry = rule-based **state** calculator. **Elements** (fire, water, ice, wind, electricity: no constant state) change **materials** (wood, metal, grass, player); elements change each other (water puts out fire); materials never change each other directly. | One small rule table `(element, material tag) -> state change / damage / spawn`, evaluated locally per entity, spreads by proximity. Designers add content by tagging, not scripting. This is the "interaction rules layer". |
| Divinity OS2 / BG3 surfaces [S: [fextralife](https://divinityoriginalsin.wiki.fextralife.com/Ground+surface+effects), [gamepressure](https://www.gamepressure.com/originalsinii/environmental-effects-and-combinations/zea274); Larian design doc not found] | Ground **surface** layers (water, oil, blood, fire, poison) as a grid; element on surface -> new surface or cloud (water+fire -> steam, water conducts electricity, oil burns). | Surfaces are a per-cell grid of tags with a transform table; same engine as chemistry, different carrier. For us a coarse grid per loaded cell is enough (no turn-based exactness). |
| Dishonored / Prey [M] | Few verbs, many tools: lights shot out or snuffed, bodies hidden, doors/vents as routes, objects thrown for noise; NPCs react to every one via shared concern queue. | Our `WorldChange` -> `ConcernBoard` (AI19) is the right carrier; interactions must **emit** into it (section 5). |
| Half-Life Alyx / Boneworks [S: [Road to VR](https://www.roadtovr.com/these-details-make-half-life-alyx-unlike-any-other-vr-game-inside-xr-design/), [UploadVR](https://uploadvr.com/half-life-alyx-boneworks-physics/)] | Alyx: "gravity gloves" pull on an arc, simulated, intent-anticipating; physics is not a showcase. Boneworks: fully simulated body/hand physics, high authoring cost and jank. | Assist over simulation: grab targeting with intent bias (section 2). Do not simulate hands. |
| Kenshi [M] | Everything has a body part or HP state, NPCs haul, loot, build; no smart objects, emergent via job queues. | Confirms NPC hauling as plan (our `Relocate`), but Kenshi's cost model needs far fewer NPCs awake. |
| UE5 Smart Objects [P: [docs](https://dev.epicgames.com/documentation/en-us/unreal-engine/smart-objects-in-unreal-engine---overview)] | Definition = slots + tags + behaviours; query by Gameplay Tags / tag query; **Claim -> Use -> Release**; claim has **priority** (higher can take a lower claim unless already in use); user-required tags filter who may use a slot. | We match claim/use/release. Two things to check against IN0: **claim priority** (player may bump an NPC's mere reservation; not an occupant) and **user tags** (size, species, armed). Neither is in the spec's `Claim`. |
| Gameplay Tags / Abilities [M] | Hierarchical tags as the universal predicate/blocking language; effects add/remove tags with duration. | Our `TagId` already plays this role. Reuse it for element and state: `Burning`, `Wet`, `Conductive` are tags with TTL, not enums. |
| Unity [M] | No built-in smart objects; assets (Behavior Designer, Opsive) copy UE or Sims adverts. | Nothing to adopt. |

**Recommended shape (framework, data-driven):** `InteractionRules` record, mod-patchable, three columns: *source* (tag set: element,
tool, projectile, impact class), *target* (material tag + current state tags), *result* (state tag add/remove with TTL, damage
kind into `DestructionRules`, spawn effect/surface, emit `WorldChange` + noise). The existing `DestructibleDef.material: TagId`
becomes the first consumer; fire/wet/light become further consumers. No new simulation core: a per-entity state-tag
component, a per-cell surface grid (tags, low-res), and a spread step on the fixed tick with a budget.

## 2. Physical handling (grab, carry, drag, throw, stack, place)

- Our physics.md already carries the classic answer (soft 6-DOF joint from a kinematic hand, force cap, drag when too heavy, throw
  impulse, marked **match** against Skyrim/HL2/Amnesia). Skyrim-style spring-to-point is the baseline [M]; HL2/Alyx adds **pull
  to hand on an arc** and **intent bias** (the thing you reach toward is preferred) [S, Road to VR/Engadget via search].
- `avian_pickup` 0.7.0 (MIT OR Apache-2.0; Avian 0.8 + Bevy 0.20) [P: [docs.rs](https://docs.rs/avian_pickup)] is a design reference
  only (we use rapier): one held object per actor, a held prop cannot be stolen by another puller, input as messages, output as
  messages for sound/VFX, works for players and AI, **untested over a network**, props need transform interpolation to look
  smooth in front of the camera, no mass limits. We already exceed it (holder limits, net).
- **First vs third person (gap).** First person: physics grab (spring) is natural. Third person (our Elden Ring camera): the standard
  is **attach to hand + carry clip** (RDR2 carry, Last of Us shelves [M]); a floating spring-held object beside a visible body
  looks wrong. The spec's `HeldProp` (kinematic attach, grips, carry clips by class) is already that and is NPC-only. **Recommend:
  the player uses the same `HeldProp` link in third person** and the spring grab only in first person: one link type for saves,
  net and AI reactions, two presentations. (Spec change, small.)
- **Two-handed and weight:** class-driven (`carry.one_hand/two_hand/shoulder/drag`) is the industry pattern (RDR2 bodies and sacks,
  KCD2 [M]). Add speed multiplier and "no sprint, no roll while holding" as data on the carry class (not in spec).
- **Stack and place:** the standard is not free physics stacking but **placement mode** (ghost preview, snap to surface or slot,
  rotate; Skyrim/Fallout shops, Valheim [M]). Drop-and-sleep (spec) is enough for rough placing; a placement preview is a
  presentation feature for homes and shops, later.
- **NPC reactions to thrown objects:** `OffenceObserved`/concern path covers harm; missing is the **nuisance** case (object thrown
  that hits nobody). Recommend a rule: a thrown/dropped prop produces a noise event and, if it lands within an NPC's view cone
  within 1 s, a `Glance`-rung concern (AI20 already has the rung). Data, no new system.

## 3. Use slots (sit, lean, sleep, eat, work, doors, ladders, stations)

- Our slot model equals UE Smart Objects plus Bethesda furniture markers; Sims "adverts" feed utility (spec). RDR2/GTA scenarios
  and Cyberpunk workspots are the same idea on prop models [M].
- **Missing slot kinds** (add as def data, one path for player and NPC): **door** (open/close/lock; Morrowind doors exist but the
  point should be a slot so NPCs claim a doorway and queue), **lever/button/switch** (state toggle + `WorldChange` + linked target),
  **ladder** (vertical traversal slot with entrance both ends; nav link; needs a controller mode, not a clip), **work station**
  (forge, alchemy, cooking pot, loom: slot + a `crafting.md` recipe set + NPC activity tag; the player opens a menu, the NPC loops
  a work clip), **container/ shop counter**. Doors and ladders are the ones with new engine needs (nav link, controller).
- **Verb prompts:** owner answer 9 gives a "Sit" prompt. For objects with several uses (bed: sleep / sit / pick lock), the
  standard is a **context-verb wheel/list with a primary verb** (RDR2 prompt + hold for alternatives, Cyberpunk) [M].
  Spec has a point index and focus, not the multi-verb arbitration. Add: slot def carries `verb` + `priority`; focus shows the best,
  hold or key cycles.
- **Claims:** keep host-only leases; add priority (above) and a **crowd rule** (a queue ticket per workstation so five NPCs do not
  stand on one stool: UE Smart Objects leave this to the user [M]).
- **Free sitting:** owner said no for the player (answer 7). Keep. NPCs may still use authored ground/ledge slots (placed by pack
  data), no extra work.

## 4. Destruction

| Tier | Industry | Ours | Verdict |
|---|---|---|---|
| Stage swap + debris | Bethesda `DEST` [M], RDR2 props [M] | IN1 | on par; right first slice |
| Pre-fractured chunks | UE5 Chaos Geometry Collections: **damage thresholds per cluster level**, clusters break top-down, clustering cuts collision count [P: [Epic cluster guide](https://dev.epicgames.com/documentation/unreal-engine/cluster-geometry-collections-user-guide-in-unreal-engine?lang=en-US), [SideFX](https://sidefx.com/docs/houdini/unreal/chaosintegration.html)] | IN12 | adopt the **cluster-level threshold** idea: chunks are not free bodies at once; the top cluster releases first, children stay welded until hit again. One integer stage still saves it. Offline fracture (Blender Cell Fracture) stays the content path |
| Runtime voxel/structural | Teardown: voxel grid, no structural stability ("a building may hang on one voxel"), network sync by determinism, not by data [S: [80.lv](https://80.lv/articles/teardown-developer-breaks-down-multiplayer-and-voxel-destruction-tech)]; Red Faction, The Finals [M] | IN13 parked | agrees with our park: event + seed (we do that), authored stages for buildings. Teardown confirms the cheap network rule |
| Runtime fracture crates | `bevy_carnage` (mesh fracture via convex decomposition, Bevy 0.19 per docs.rs, git only) [S: search]; `bevy_shatter` (Voronoi glass, Bevy 0.16) [S]; none touch rapier | not needed | spec already rejects; keep. Glass is the one case where **runtime Voronoi** earns it (windows, bottles): bake several shard sets offline per size instead |

- **Contents spill and damage scaling:** owner answer 1 covers it. Standard adds **loot-in-container rolls on break** (Skyrim
  `LeveledItem`, adapter) and a "salvage" tag (a barrel yields planks). Data only.
- **Debris budget:** spec cap 48 awake bodies/region, 6 breaks/tick: fine. Add **distance LOD**: beyond ~25 m spawn no bodies, only
  the stage swap and an effect [M, standard in UE/Chaos "debris removal"]. Tunable.
- **Persistence and nav:** overlay blob + `Destroyed` event + authored nav link flip: standard. Add: a destroyed prop in a **pinned**
  or quest scene must respect pin class (IN15 covers it).

## 5. Environment systems (what is entirely missing)

None of fire, wet-as-gameplay, light interaction or interaction noise exists in any spec (greps: no `ignit`, `flammab`, `fire
spread`; weather.md has surface wetness designed, not built).

- **Fire.** Standard (BotW, Far Cry, Divinity) is state-tag spread: a `Burning` entity ignites neighbours of flammable material tag
  within radius at a rate, consumes fuel (HP) and ends in a stage swap (ash, charred). Wind biases the spread direction (we have
  wind). Water/rain/`Wet` blocks it. Cost control: fire cells, not per-triangle; cap burning entities and particle lights per
  region; vegetation burning is a groundcover/terrain tint, not per-blade. **Recommend a minimal chemistry slice (IN17)**: states
  `Burning`, `Wet`, `Conductive?` (skip electricity for now), a rules table, spread with a budget, one effect set. Opt-in per
  pack: Morrowind has fire spells and torches but not fire spread; the adapter decides ignition sources.
- **Wet.** weather.md surface wetness already tracks it per surface; the missing hook is entity `Wet` tag (rain exposure, water
  contact) that blocks ignition and amplifies shock. Fed from the same weather query; cheap.
- **Wind on objects.** Lightweight props (cloth, banners, hanging signs) react via a wind-force term on kinematic/sleeping
  bodies, or pure shader sway (weather WD2/WD3 designed). Prefer shader; not gameplay.
- **Light sources (high player impact).** Pick up a lamp or torch, snuff a candle, shoot out a lamp are the Dishonored/Thief/
  Skyrim torch staples. Our lights.md has "carried lights get a separate" path (line 64) for the player; missing: a **light
  slot verb** (extinguish/relight = state tag `Lit` on the entity, light component follows), NPCs notice darkness (concern
  kind) and relight at dusk (a schedule activity: ai.md schedules). Cheap and very visible; recommend for the adapter first.
- **Sound of interactions (stealth).** ai.md:146 and action.md `noise: Loud` already emit noise for combat. Interactions must too:
  a **`NoiseEvent { pos, loudness, kind }` emitted by** impacts over `ContactForceEvent` thresholds, `Destroyed`, doors, thrown
  props, dropped items (loudness from material and impact speed), consumed by the same `portal_query::path_distance`
  attenuation. This one hook makes Dishonored-style distraction possible and is nearly free (data on material tags).
- **Electric/ice:** skip until a pack wants them; the rule table makes them additive content.

## 6. World persistence

- IN14/IN15 (delta classes, lazy reconcile on cell load, caps) is the modern standard in spirit and exceeds Bethesda's
  (Skyrim `iHoursToRespawnCell`, persistent refs, "Never Resets") [M]. Open worlds like BotW reset by class (grass/enemies on
  blood moon, props mostly persistent only in memory) [M]; Teardown saves per-voxel but is a small map [S].
- Gaps to add as one line each to IN15: (a) **a dynamic body is saved only at rest** (a moving physics object at save time is
  snapped to its sleeping pose or its home; avoids saves with items mid-air); (b) **home-distance tidy rule**: a moved prop far
  from its authored placement in a settled place counts toward the owner's tidy (AI24 covers NPC action; the delta-expiry covers
  the absent case); (c) **streaming-out of a held or in-flight prop**: the holder's link wins (the prop travels with the actor;
  an orphan is placed at the holder's last position); (d) **per-cell moved-prop cap** already 256 deltas: fine.

## 7. Crates and open source (Bevy 0.20, GPL-3.0-or-later compatible)

| Crate | Version / Bevy | Licence | Use |
|---|---|---|---|
| `avian_pickup` | 0.7.0, Avian 0.8, Bevy 0.20 [P] | MIT OR Apache-2.0 | design reference (rapier is ours); one-held-object rule, messages for SFX |
| `saddle-physics-object-interaction` | Bevy 0.18, Avian 0.6 [S] | not checked | reference only |
| `bevy_carnage` | Bevy 0.19, git [S] | not checked | mesh fracture for gore, not props |
| `bevy_shatter` | Bevy 0.16 [S] | not checked | stale |
| `bevy_picking_rapier` | 0.20.0 listed [S] | not checked | raycast picking backend only, no grab; our interaction focus uses our own point index |
| Smart objects / chemistry crates | none found | | confirms "our own" verdict in the spec |

No dependency recommended. Licence of `bevy_carnage` and `saddle-*` not verified: check before any port.

## 8. Gaps ranked by player impact, with recommendations

| # | Gap vs standard | Rec | Why |
|---|---|---|---|
| 1 | No systemic rules layer: fire/wet/light/electric as tag states with a rule table (BotW, Divinity) | **Add** IN17 `InteractionRules` + state tags + budgeted spread; fire and `Wet` first | The biggest "interactive, prettier, more alive" win; reuses `material` tag and weather wetness |
| 2 | Interactions emit no noise; stealth/AI cannot hear a smashed crate or thrown cup | **Add** `NoiseEvent` hook to IN1/IN4/IN6/doors | Nearly free; feeds AI19; Dishonored-style play |
| 3 | Light interactions (carry, snuff, relight, NPC relight at dusk) | **Add** `Lit` state + slot verb; adapter first | Very visible, Morrowind torches/lamps |
| 4 | Missing slot kinds: door, lever, ladder, work station | **Add** to IN0 def vocabulary; ladder and door get own slices (controller mode, nav link) | Doors/ladders block traversal parity with modern ARPGs |
| 5 | Player third-person carry should be `HeldProp` (attach + clip), spring grab only in first person | **Change** (merge player path into IN9's link) | One link for save/net/AI; looks right in third person |
| 6 | Claim priority + user tags (UE) not in `Claim` | **Check/Change** IN0 before it is built | Cheap now, a migration later |
| 7 | Multi-verb prompt arbitration (primary verb + alternatives) | **Add** to IN0 focus (slot `verb`, `priority`) | Beds/tables/doors have several uses |
| 8 | Chunk release by cluster threshold, debris distance LOD | **Change** IN12; tunables | Perf and look; Chaos standard |
| 9 | Persistence: at-rest snapping, holder-link on stream-out | **Add** one line each to IN15 | Avoids save bugs seen in Bethesda mid-air items [M] |
| 10 | Nuisance reaction for thrown/dropped props that hit no one | **Add** as `ConcernRules` data | Cheap, sells "NPCs notice" |
| 11 | NPC relocation (`Relocate`, IN9) has no shipped precedent (spec itself says so) | **Keep, stay late** | Pull-out clip (cheap) first, as already decided |
| 12 | Runtime fracture crates, voxel destruction, structural collapse | **Drop** for now (stay parked, IN13) | Teardown says even the best skips structure; cost huge |
| 13 | Placement mode (ghost, snap) | **Park** (homes/shops later) | Not core; drop-and-sleep suffices |

**Adopt:** none (no crate). **Change:** 5, 6, 8, 9. **Add:** 1-4, 7, 10. **Park/drop:** 11-13.

### Candidate owner questions (player-facing; none repeat answered questions)

1. Should fire spread through the world (wooden houses, grass, props) and be fought with water, or only burn what a spell
   or torch directly hits? Recommend **spread, with a clear limit** (settlements only burn under authored/quest or magical
   fire; grass and props burn freely in the wild), because spread is the signature systemic moment and the cost is bounded.
2. Can the player put out and relight light sources (candles, lamps, torches) and do NPCs react to darkness and to you doing it?
   Recommend **yes**, with NPCs relighting at dusk.
3. Do thrown objects and smashed crates draw NPC attention (distraction, stealth tool)? Recommend **yes**: loudness by
   material and speed; owned-object vandalism offence already settled.
4. In third person, may the player carry only one thing at a time in the hands (bodies, crates, chairs), with a speed penalty
   and no roll/sprint? Recommend **yes**, to match NPCs; small items go in inventory as today.
5. Do doors and ladders take part in the same interaction (NPCs can be blocked by you standing in a doorway, doors can be
   wedged, kicked, or barricaded)? Recommend **open/close/lock only first**; wedge/barricade as a later pack option.
6. Do workstations (forge, loom, cooking pot) show NPCs actually working at them and let the player use the same object?
   Recommend **yes**: same slot, different menu; NPC work is an activity loop.

## 9. Missing from this note

- Primary-source GDC (BotW chemistry) slides and the Alyx/Boneworks physics writeups were not reached; BotW and surface notes
  are from press and wikis. RDR2, KCD2, Cyberpunk, Kenshi, Skyrim details are [M].
- Licences of `saddle-physics-object-interaction`, `bevy_carnage`, `bevy_shatter` not checked.
- No perf numbers for fire spread or surface grids (to be measured per the bench-method note when built).
