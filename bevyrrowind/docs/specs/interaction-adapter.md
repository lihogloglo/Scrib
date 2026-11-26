# Interaction adapter: Morrowind furniture, crates, beds, mounts and travel on `sf_interaction`

Status: design 2026-11-11 (ROADMAP 3.11), revised 2026-11-26 (IN29); IN32a core built 2026-11-26 (`mw_interaction`:
kinds, defs, activation table; not wired yet). Framework:
[interaction.md](../../../superfargoth/docs/specs/interaction.md),
[world rules](../../../superfargoth/docs/specs/interaction-world-rules.md),
[destruction](../../../superfargoth/docs/specs/interaction-destruction.md); lane:
[interaction roadmap](../../../docs/roadmaps/interaction.md); research:
[world-interactivity-2026-11-11](../../../superfargoth/docs/research/world-interactivity-2026-11-11.md). Related:
[motion-generation.md](motion-generation.md) (MG4 census, clips), [ai-adapter.md](ai-adapter.md) (AIWander, anchors),
[physics-adapter.md](physics-adapter.md) (pick vs grab, ownership), [crime-adapter.md](crime-adapter.md) (offences),
[services-adapter.md](services-adapter.md) (travel), [scripting-adapter.md](scripting-adapter.md) (DQ-6),
[groundcover-adapter.md](groundcover-adapter.md). OpenMW citations: commit 4e718b50, `apps/openmw/`.

## Morrowind has none of this

Vanilla Morrowind and OpenMW have no furniture use, no destruction and no mounts: chairs and beds are statics or
activators, crates are containers, silt striders, boats and gondolas are travel services that fade and advance time.
Everything here is **modern** (owner: modern first, classic behind a seam). The **Classic preset** keeps vanilla:
furniture inert, nothing breaks, nothing burns, lamps stay as placed, travel fades. Each feature is an adapter switch
(`mw.rules.interaction.*`: `sit`, `break`, `bash`, `move_furniture`, `mounts`, `ride_travel`, `snuff`, `fire`,
`throw_hurt`); a switched-off kind or verb is not indexed (interaction.md "Definitions"), so Classic sees Morrowind.

## Activation per record class (IN32 registers these as default verbs)

Today `mw_player_bevy::activatable` (`src/lib.rs:798`) gives the focus a name and prompt verb, the key writes
`Activated` (`sf_script_bevy/src/lib.rs:66`), and default actions read the unhandled `ActivationOutcome` (doors
`mw_doors_bevy`, talk `mw_rpg_bevy/src/dialogue.rs`, take/open `mw_rpg_bevy/src/inventory.rs:235`). IN32 moves each
onto the contract (interaction.md "Activation contract"), behaviour unchanged. Crime's lock and trap gate runs first
(crime.md decision 8; a key unlocks and disarms, a trap fires: `door.cpp:143-180`, `container.cpp:152-200`); OpenMW's
werewolf refusal is a later gate. The gate refuses only `open` (Locked, TrapSprung); `pick` and `probe` are crime
verbs that pass it (crime-locks.md): a readied lockpick (probe) + activate on a locked (trapped) `DOOR`/`CONT` maps
to `pick` (`probe`), IN33. Rows in focus order (files in `mwclass/` unless named):

| Record | Verb | Slot | Default action | Vanilla | Classic |
|---|---|---|---|---|---|
| `NPC_` living | talk; pickpocket sneaking; search if knocked out | none | dialogue; crime A4 | `npc.cpp:798-845` | same |
| `NPC_`/`CREA` dead | search | none | loot (`ContainerOpened`) | `npc.cpp:813-821`, `creature.cpp:441-450` | same |
| `CREA` living | talk if it has dialogue | none | dialogue | `creature.cpp:452-458` | same |
| `CONT` | open (organic harvest later) | none | `TakeRequest` → `ContainerOpened` | `container.cpp:140-200` | same |
| `DOOR` with destination | open | none (not a door slot) | fade travel (`mw_doors_bevy`); NPC use AI12 | `door.cpp:118-124, 183-199` | same |
| `DOOR` without | open / close | `Door` (IN33) | swing; "Locked" from crime's `LockState` | `door.cpp:202-229`, `mwworld/worldimp.cpp:1427-1440` | same |
| `BOOK` | read | none | reader (UI lane; until then take) | `book.cpp:55-62` | same |
| `ALCH INGR MISC WEAP ARMO CLOT APPA LOCK PROB REPA` | take | none (NPC: IN16) | `TakeRequest`, theft by owner | `misc.cpp:63-66` | same |
| `LIGH` carryable | take; snuff / relight | `Light` | take; `Lit` | `light.cpp:93-103` | take |
| `LIGH` fixed flame | snuff / relight (verb-only prompt) | `Light` | `Lit` (IN19) | none, no tooltip (`light.cpp:150`) | inert |
| `ACTI` with `OnActivate` | use | none | the script (DQ-6) | scripting-adapter.md | same |
| `ACTI` without | use (as today) | none | none | `activator.cpp:116-123` | same |
| Census furniture | sit / lie / lean | `Pose` | "Using a slot" | none | off |
| Beds | sleep; sit on the edge | `Pose` | lie-down clip, rest hook (IN3) | crime-adapter.md "Sleeping in a bed" | rest menu |
| Locked `DOOR`/`CONT` | bash (hold list, after pick) | none | IN37 blow → crime `LockRequest::Force` | none | off |

- **Scripts win.** A script containing `OnActivate` suppresses the default and calls `Activate` to run it
  (scripting-adapter.md "Activation"), so a scripted seat sits only when its script allows it; `Activate` re-enters at
  the gates (contract step 7).
- **Prompts.** Statics have no name and no tooltip in vanilla (`hasToolTip`); the adapter gives seat slots a
  verb-only prompt ("Sit", owner Q5), off in Classic. Several uses of one object are verbs in the hold list.

## Defs from the census (IN32)

**Furniture** (from the MG4 census, `mw_bake::census`, archive/motion-generation-as-built-2026-10-28.md "As built (MG4)"): `STAT`/`ACTI`/`CONT`
whose `furn_*`/`active_*` id or model names a bed/bedroll/hammock/bunk, counter/bar, table/desk, chair/stool/bench/
throne or shelf. Vanilla + expansions: 32 seat, 28 table, 12 counter, 46 bed, 28 shelf base records, ~10.3k
placements, 97 % at scale 1.00. Measured bands: seat 0.14 / 0.48 / 0.54 / 0.64 m, table 0.84 / 0.98, counter 0.94, bed
0.22 / 0.52 / 0.64 / 0.74 (Morrowind furniture runs 15-30 % above real-world heights; plan clips against these).

- `InteractionContent::def_for(base, model)` maps the census kind to an `InteractableDef`: seat → `Pose { seat }`
  (benches: one per `mw.interact.bench_seat_m`, `Guess` 0.55 m, exclusive `group` for overlaps), bed → `Pose { bed }`
  + `seat` on its edge, table/counter → `counter` slots along usable edges (stand and lean) whose `reach` covers the
  top, for hand claims on cups and plates (interaction.md "Claims"); shelf → none (later `browse`). Workstations
  (IN42: anvils, alchemy benches, cooking pots; the player gets Morrowind's repair or alchemy menu) join the table.
- Slot frames, bands and entrance sides come from `interact.cook` on the baked collision (the census top-surface code
  promoted to a cook step); a side blocked by a wall in a placement is dropped at runtime by `EndPoseFree`.
- **Door hinge from the placement, not the bounds.** OpenMW turns a door about its reference's vertical axis, from
  the placed angle to +90° at 90°/s (`worldimp.cpp:1427-1440`): vanilla door meshes pivot at their origin. `DoorDef` =
  hinge at the origin, axis up, 90°, one way, `close_behind` for NPCs; the bounds fallback only for mod doors whose
  origin is off the bounds' edge (the cook lists them).
- Each def carries its material (world rules "Rules layer"), from the table below.
- Mods add or patch `sf.interact.def` records keyed by base id (a mod chair with a better seat frame wins over the
  cook), so content stays a mod (rule 3).
- **Sitting needs no promotion**: slots are derived from the placement, the character attaches; the static stays in
  the static scene. Only moving or breaking a static promotes it (static-scene.md §1).
- IN32's test: exact defs on synthetic records; def counts per kind printed on the install.

### As built (IN32a, 2026-11-26)

Bevy-free crate `bevyrrowind/crates/mw_interaction` (21 exact tests), nothing wired yet:
- `furniture.rs`: `Kind`, `classify` (the one definition; `mw_bake::census` re-exports them since IN32b2).
- `defs.rs`: `def_for(base, kind, &Measured { center, footprint, top, front: Facing }, &FurnitureParams)`. Slot frame =
  root on the floor, +Z the way the user faces; `band` = `top`. Seat: places of `bench_seat_m` across the front, plus
  the places between them for 2-5 places (slot `k` has mask bits `k - 1`, `k`), Front entrance (`seat.chair` /
  `seat.bench`). Bed (frame fixed in IN32b1, anim contract AN25): `lie` (`bed.lie`) with +Z to the head
  (`Measured::head`), body along +Z, entrances Left and Right (the lier's own sides, right = `Y x head`), plus one edge
  `sit` (`seat.bed_edge`) per long side facing out (Front entrance); all three share bit 0. Table (4
  edges) / counter (front, back): `lean` stations per `counter_spacing_m`, facing the top, entered from Back, `reach` to
  the centre line (`counter.table` / `counter.bar`). Shelf, degenerate shapes, `sit` off: `None`.
- `activation.rs`: `ROWS` (record class, condition, verb tag, prompt, default action, Classic, switch, availability);
  `primary`/`offered` give today's `activatable` prompts exactly (tested), plus furniture sit/lie/lean; unbuilt actions
  (search, pickpocket, creature talk, swing, bash, snuff/relight, sleep, browse) are `NotYet(<slice>)`, never offered.
- `tunables.rs`: `mw.rules.interaction.sit` (bool) and `mw.interact.{bench_seat_m, approach_m, edge_seat_inset_m,
  counter_spacing_m, counter_stand_m}` (`Guess`), read through `FurnitureParams`.
- Left: IN32b (split: IN32b1 measuring as pure code, below; IN32b2 the bake calls it and prints def counts per kind
  on the install); IN32c, the shell registers `TUNABLES`, indexes defs and registers the offered rows as default
  verbs (replacing `activatable`'s verb strings). Hand IK targets and adverts are not set yet.

### As built (IN32b1, 2026-11-26)

`mw_interaction::measure(kind, &[[Vec3; 3]], top, &MeasureParams) -> Option<Measured>` (`measure.rs`, 30 crate tests).
IN32b2's bake calls it once per base record with the census triangles (collision first, render fallback, the same soup
`census::shape` reads) and `Shape::top_below(kind.cap())`, then `def_for`. Model frame, engine axes, Y up, floor = the
lowest point; `center`/`footprint` = the x/z bounds of all triangles. Triangles are clipped exactly against the
measuring planes (area-weighted, so density and long panels don't bias). Rules (thresholds `mw.interact.measure.*`,
`Guess`, lifetime `Bake`):
- **Seat front** = away from the backrest: the area above `top + backrest_above_m` (0.2 m), if at least
  `upright_min_area` (0.1) of the footprint area, whose centroid is at least `backrest_offset` (0.3) of the half
  footprint off-centre (larger of x, z); symmetric arms cancel. **Fallback** (stools, backless benches, a centred
  post): aspect >= `bench_aspect` (1.5) faces the + side of the short axis, else +Z.
- **Bed head** = the end zone (outer `bed_end_share`, 0.25, of the long axis) holding at least `head_share` (0.6) of
  the area above `top + headboard_above_m` (0.15 m) in both zones, that area >= `upright_min_area` of the footprint.
  **Fallback** (bedrolls, hammocks, equal posts): + end of the long axis. `front` = `head` for beds.
- **Table, counter, shelf front** = + side of the short axis (def stations use both long sides / all edges, so the
  sign is not measured). `head` = `front` for every non-bed kind. Square footprints count as long along x.
- `None`: no triangles, any non-finite point, zero extent in x or z, top not positive finite.
- Floor offset: closed in IN32b2 (below).

### As built (IN32b2, 2026-11-26)

Bake step `mw.interact.defs` (`bevyrrowind/crates/mw_bake/src/interact.rs`, version 1, one product, subject
`furniture`), run by `bevyrrowind bake` after the navmeshes; nothing registered at runtime (IN32c reads it with
`decode_interact_defs`). The census now `pub use`s `mw_interaction::furniture::{Kind, classify}` and shares two
helpers: `furniture_records(db)` (the census record scan) and `model_soups(nif)` (collision + render triangles).
- Per census base record: census top (`Shape::top_below(kind.cap())`) on the collision soup, else the render soup;
  that soup and top to `measure()`, then `def_for` with the `FurnitureParams` defaults. Shelves, records without
  a usable top or a loadable model: no def (counted). Product = `InteractDefs { defs (sorted by base RecordId),
  counts per kind, failed models }`, postcard.
- Plan parameters: the record list (`id kind model` lines) and both parameter sets (`Debug`); every model is a
  tracked file read, so a mod editing a furniture record or its mesh re-bakes the defs.
- **Floor offset rule:** `Measured::floor` = the measured soup's lowest y (model frame); `def_for` stands every slot
  root and approach point at that y (`band` stays the height above it). Vanilla furniture is authored centred, not
  floor-rooted: 133 of 143 measured records (every table, counter and bed, 30 of 31 seats) sit 0.18-1.34 m above
  their floor (beds ~0.45, `pr` beds 0.65, bunks 1.0, bars 0.47); without the offset every slot would float.
- `Measured::basis` records which rule set the facing (`Upright` = backrest/headboard found, `LongSide`, `PlusZ`,
  `LongEnd`); the bake report counts it.
- **On the install** (vanilla + expansions, `tests/interact_real_install.rs`, measurement): seat 32 records, 31 defs,
  83 slots (backrest 10, long-side fallback 12, +Z fallback 9, 1 unmeasured, 6 on render geometry); table 28 / 28 /
  266 slots; counter 12 / 12 / 24; bed 46 / 46 / 138 (headboard 12, long-end fallback 34, 6 render); shelf 28 records,
  no defs; 0 models failed. Open: 34 of 46 beds take the head fallback (a head at the foot is possible; the owner
  check in IN32c shows it); 266 table stations (~9.5 per table) may want a coarser `counter_spacing_m`.

### As built (IN32c1 + IN32c0, 2026-11-26)

New thin crate `bevyrrowind/crates/mw_interaction_bevy` (`add(app, store, manifest)`, called by `mw_exterior_bevy`
after the player; registers `mw_interaction::TUNABLES` and `sf_interaction_bevy::InteractionPlugin`).
- **Defs:** `mw.interact.defs` decoded once at startup into `FurnitureDefs` (base `RecordId` -> `Arc<InteractableDef>`);
  missing or undecodable: one warning, no defs, the game runs as before. Log `interact: N furniture defs loaded (S slots)`.
- **Index:** `FurnitureHook` wraps the STAT/ACTI/CONT spawners: R0 activation of a def'd base writes
  `IndexChange::Insert` (placement world frame, scale ignored; owner = cell entity) while `mw.rules.interaction.sit` is
  on, deactivation or discard `IndexChange::unload`; `ResidentFurniture` makes the switch live (off removes, on
  re-inserts). Log once per cell: `interact: indexed M furniture placements in <ws> (x, y, z)`. The index has no
  worldspace key (one worldspace resident at a time).
- **Player:** `InteractionUser` (live `priority_player`, no tags) on `PlayerBody`; `sf_action::SEAT_RULES` in the
  player's rules. `mw_player_bevy::Focus::hit` (the ray's first hit in reach, activatable or not) is the single
  `FocusCandidates` entry per fixed tick (score `1/(1+d)`, `held: false`: no hold list yet). Each Use press (or
  `--activate`) is latched in `UsePresses` (`mw_scripts_bevy::activate`) and becomes exactly one
  `FocusState::command` between `InteractionSystems::Focus` and `::Activate` (none lost on a zero-tick frame, none
  doubled on two). Def-less targets: `verb: None` as before. Prompt: a pose verb shows verb-only from the rows
  (Sit / Lie down / Lean), anything else keeps `activatable`'s name + verb.
- **Class verb (IN32c0):** a def'd census `CONT`/`ACTI` keeps its class's primary use (`class_verb`: open / use) as a
  slotless verb at `CLASS_VERB_PRIORITY` 1, above every pose verb (0), so its best verb and prompt are unchanged; its
  pose verbs follow (hold list later). Install: ACTI 55 defs (use kept), CONT 0.
- **Seat families (IN32c0, anim contract `<family>[.<style>].b<band cm>.{enter.<side>, loop, loop.fidget,
  exit.<side>}`, provisional until ANIM-9):** top under `mw.interact.floor_seat_band_m` (0.3 m, Guess, strict) ->
  `seat.floor` (wins); else one place with a backrest (`Basis::Upright`) `seat.chair`, one without `seat.stool`, more
  `seat.bench`. Bake `mw.interact.defs` v2 (report counts CONT/ACTI defs).
- Still default handlers on `ActivationOutcome` (IN32c2 moves them to `UseApproved`). The player's sit is refused
  (`NoRootPath`) until the slot table has the `interact.sit.*` clips (anim AN26c reads `SeatClips`).
- Open: ACTI placements are not in `ScriptPlacements` (`mw_scripts_bevy` hook skips ACTIVATOR), so an activator is
  never focused (pre-existing; its 55 furniture defs are indexed but unreachable by the player).

### As built (IN32c2, 2026-11-26)

The pack's defaults run on `UseApproved` (contract step 6), so a gate in `ActivationSystems::Gates` can refuse them:
travel doors (`mw_doors_bevy::activate`), talk (`mw_rpg_bevy::dialogue::talk_on_activate` via pure `talk_target`),
take and open (`mw_rpg_bevy::inventory::request_takes`), each `.after(ActivationSystems::Default)` the same frame,
other ordering kept. One verb rule, `mw_interaction::runs_default(DefaultAction, verb)`: the primary use
(`verb: None`) or the tag of an offered row with that action (door/container `open`, `talk`, `take`/`read`); a pose
verb on a def'd chest or activator never opens it. `mw_player_bevy::activatable` takes its prompts from the rows
(`primary(class, ..).prompt`; a light's `Carryable` is the only condition known there), names and tooltip rules
unchanged. A script's `Activate` (`ScriptActivate`) now runs these defaults too (the contract; nothing writes it
yet). **IN32c2b:** `mw_audio_bevy`'s `door_sounds` and `swing_door_sounds` play on `UseApproved` with `runs_default(TravelDoor, verb)`, `.after(ActivationSystems::Default)`. Only `mw_scripts_bevy`'s dev `report` still reads `ActivationOutcome`.

### As built (IN33c1, 2026-11-26)

`mw_interaction::door_def(base, doorway)` is the def of a swing door (a `DOOR` reference without a destination), built
per placement, not baked: `doorway` is the closed leaf's centre in the object's frame (the model bounds' centre, from
the hook). One `Door` slot at the doorway: hinge at the origin, axis +Y, 90 degrees, one way, `close_behind` (OpenMW
`apps/openmw/mwworld/worldimp.cpp:1427-1440` is the behaviour source); its verb `pass` takes users tagged `npc`
(`USER_NPC`; AI14 gives NPC users the tag), so the player, whose user has no tags, never claims a doorway. The player
toggles through two slotless verbs at `CLASS_VERB_PRIORITY`: `open` (not `door.open`) and `close` (`door.open`). The
angle is a content convention (`// tunables: const`), not a tunable. The two `SwingDoor` rows of the activation table
are Offered; without conditions the door's primary row is still the travel row, so `class_verb(Door)` and the def-less
prompt stay "Open". `runs_default(_, open)` is true for both door actions: `mw_doors_bevy` acts only on a door in
`DoorLinks`, the framework's door default only on one registered in `Doors`.

## World rules mapping (IN41, IN38, IN19)

**Material rows** (`sf.interact.material` + a pattern table in `mw_interaction`, checked against the install at cook,
unknown models default to inert stone and are listed; `impact_reach_m` are `Guess` tunables until IN18 measures):

| Source | Patterns (examples) | Material | Burns | Noise kind |
|---|---|---|---|---|
| Objects | `contain_crate*`, `barrel*`, wooden `furn_*`, docks, carts | wood | yes | `impact.wood` |
| Objects | rugs, banners, tapestries, sacks, bedrolls | cloth | fast | `impact.cloth` |
| Objects | hay, straw, baskets | straw | fastest | `impact.straw` |
| Objects | redware, urns, pots, plates / bottles | clay / glass | no | `impact.clay` / `.glass` |
| Objects | dwemer, silverware, iron | metal | no | `impact.metal` |
| Objects | `ex_*`, `in_*` architecture, rocks | stone, `structure` | no | `impact.stone` |
| `LTEX` | `tx_*grass*` and grass mixes / dirt, sand, rock, ash | grass (fuel) / inert | yes / no | step kinds |
| Groundcover | `flora_grass_*`, `flora_bc_*`, `flora_bm_grass_*` | grass, fuel by density | yes | none |
| Static flora | `flora_tree_*`, shrubs | ground-cell fuel, charred variant if any | via cell | none |

Bitter Coast swamp and Solstheim snow are often `Wet` through surface wetness (SU1): no special case.

**Ignition sources** (rule rows; Classic ignites nothing):

- Fire magic: any `FireDamage` effect (MGEF 14) on hit, touch, target or area (spells, enchantments, creatures);
  `FrostDamage` (MGEF 16) and water remove `Burning` and add `Wet`.
- Torches: an equipped `LIGH` with `FIRE` and `CAN_CARRY` that hits or is held against a flammable target.
- Placed fires (`LIGH` with `FIRE`: candles, braziers, fire pits) and lava (`*lava*` models, lava `LTEX` if any)
  ignite **by contact only** (a rug pushed into a brazier), never as radius heat, so a lamp beside a tapestry never
  burns a town at load.

**Settlements** (owner 2026-11-25: towns burn only through authored stages): zones are every interior cell and every
named exterior cell (`CELL` name set; later worldgen settlements give theirs, Magna Urbis data). Architecture (`ex_*`,
`in_*`, mod kits by the same table) is `structure`, never caught by spread in a zone. Furniture, crates, banners, hay and grass in towns burn.

**Lights** (`Light` slot, IN19):

- `LightDef` from `LIGH`: `radius_m` as `light_record` converts it (`mw_walkers_bevy/src/light_inputs.rs:164`;
  `NEGATIVE` and radius 0 give none), `intensity` = `mw.ai.light.lamp_intensity`, `lit_default` = `OFF_BY_DEFAULT`
  clear (OpenMW inserts such a light's model unlit and without its loop sound, `light.cpp:45, 56`).
- Snuff and relight only for `FIRE` lights; glow and magic lights get no verb. `LightChanged` also toggles the loop
  sound (`mw_audio_bevy`).
- **AI light (MA16):** `write_light_inputs` (`light_inputs.rs:307`) today copies `PlacedLights` (`LightHook`, `on` =
  the record flag); after IN19 it copies the active space's `GameplayLights`, so a snuffed lamp darkens stealth sight.
  Equipped torches stay the host's; a dropped lit torch is a light-bearing placement.

**Noise** (IN18): the host emitter (`mw_walkers_bevy/src/noise_emitters.rs`) maps `InteractionNoise` one to one onto
`NoiseEvent` (`maker` → `PerceptTarget`) and owns the `TagId` → `NoiseKindId` table, beside `KIND_NAMES` /
`PlayerNoiseKinds`: `impact.<material>`, `drop`, `throw`, `break.<material>`, `door`, `bash`, `fire` (pick, trap and forced-lock
noise: crime emits it, `sf_crime_bevy`, crime-locks.md).

## Breakables (IN38; bash IN37)

- **Set** (owner: everything breakable breaks, pottery too): `CONT` crates, barrels, clay urns and sacks
  (`contain_crate_*`, `contain_barrel_*`, `contain_urn_*`, `contain_sack_*`), `MISC` pottery and glass (redware, pots,
  plates, bottles: baked shard sets), decorative `STAT` urns and pots (promoted on the break). A data table in
  `mw_interaction`, checked at cook (IN38 prints counts). Organic containers, chests, closets, desks, corpses, metal
  and stone do not break.
- **Damage** (`DestructionRules`): blunt weapons full, blades ×0.5, arrows ×0.25 (a barrel takes a few arrows), hand to
  hand ×0.25, fire on wood ×1.5; thrown and falling impacts by the physics threshold. `Guess` tunables.
- **On break**: the contents (resolved levelled lists, owner kept on each item) spill as placed items, damaged by the
  overkill (owner: a vaporised crate leaves little). A locked breakable breaks like any other, lock included.
- **Crime**: taking a spilled owned item is theft as today (items keep their owner). Breaking an owned object in view
  is the new **vandalism** offence (owner 2026-11-11): crime's row (MWR1), `arg` = the value destroyed (object and
  lost contents), bounty like theft.
- **Doors and locks are bashed open, never destroyed** (IN37): the adapter turns Strength and weapon (or fists) into
  the blow force; crime settles `hp`, `Broken` and the offence.
- **Scripts**: anything with an `OnActivate` or `OnDeath`-like script, or named by any script (`mw_script` index), is
  unbreakable by default (quest containers must not vanish); the eligibility table reports them.
- Debris: generic wood planks, clay shards and cloth scraps (pack meshes, cosmetic).

## Switches (IN34)

- Every working vanilla lever, button or pull chain is an `ACTI` whose script does the work (`OnActivate` then
  `Lock`/`Unlock`, `SetAngle`, `PlayGroup`, `Enable`): **scripts win** (DQ-6), no `Lever` def. An unscripted
  lever-like `ACTI` is decoration (nothing to link): none either.
- Plain levers are data: an `sf.interact.def` `Lever` keyed by base id or placement with its links (target, signal:
  open a door, toggle a light), for mods and worldgen. IN32 prints `ACTI` counts (scripted / lever-like unscripted).

## Beds and resting

- Beds that are `ACTI` with a rest script (vanilla `Bed_Standard`-style scripts; the census lists `active_*` beds;
  verify the script set at IN3) keep their behaviour: activation opens the rest menu, now after a lie-down clip.
  Sleeping in an owned bed is `OT_SleepingInOwnedBed` (crime-adapter.md "Offences"), unchanged.
- Static `furn_*` beds: lie down only; resting follows the normal "can rest here" rules (`GetPCSleep`, illegal-rest
  cells), not the bed.
- NPCs: `AIWander` idles become slot activities when slots are near (ai-adapter.md mapping table); the `home` anchor
  is an owned bed (AI14); NPCs sleep in their own or their faction's beds (ownership on the placement).

## Moving furniture (IN9, IN6)

- **Pull-out** (IN9; IN5 folded in, owner 2026-11-11): chairs and stools (not benches, thrones, prayer stools) get
  `pre_move: Slide` along their forward axis (`mw.interact.pull_out_m`, `Guess` 0.35 m) when the entrance is behind a
  table edge (table found by the cook's neighbour search).
- **Player grab**: chairs, stools, small tables and barrels are `carry` objects; benches, beds, counters and shelves are
  drag-only or fixed (mass from the physics adapter's weight rule; fixed for anything built into walls). Moving is not
  theft (physics-adapter.md "Pick up vs grab").
- **Static-scene eligibility**: a moved or broken `STAT` placement must load as an entity after a save (the overlay
  names it): `mw`'s eligibility hook reads the overlay (static-scene.md §1, "no record or script names it"); confirm
  the hook sees the overlay with scene-coord before IN6.

## Mounts (IN10)

- **The guar** is the first mount (Morrowind's beast of burden; no horses in Vvardenfell). Mount base: a new pack
  creature record (`mw_guar_mount`) on the vanilla guar mesh and skeleton with its own walk/run/idle clips; saddle
  slot frame measured on the guar skeleton's spine bones at cook; rider clips generated (Kimodo, seated astride,
  bands from the saddle height), legs by IK to the flanks.
- Acquisition: bought from a guar herder (a new service on the travel/merchant path) or a quest; whistle summon within
  a radius; follows when unridden (`sf_ai` `Follow`); can carry cargo (pack-guar inventory) later. Owner questions.
- Mounted combat: not in the first slice.

## Travel vehicles (IN7, IN11)

- **Silt striders, boats, gondolas** stay services (services-adapter.md: `DODT` destinations, prices, time). Modern
  option **ride-along**: the player sits in a seat on the vehicle (strider howdah, deck bench, gondola seat) and the
  vehicle follows a route spline at travel speed through the loaded world, skippable at any time (fade to the classic
  arrival, same time cost). Ride-along is on in the modern preset (owner 2026-11-11) once the moving-deck and seat
  slices land (travel.md Stage 3-4); until then every preset is an instant teleport (fade, game time spent; owner
  2026-11-25); Classic: fade only.
- Routes: authored splines per destination pair (pack data, `mw.travel.route` records), a straight fallback only over
  water; no route = fade. Silt strider placements are animated statics (`a_siltstrider`-style meshes; check NIF
  controllers at IN7); the ride moves a promoted copy, the docked one is hidden for the trip.
- Walking on a boat's deck during a ride needs moving platforms (IN11, with 3.7 buoyancy).

## Saves and co-op

Nothing adapter-specific beyond the framework (`interaction` overlay blob: stage, health, door, lever, `Lit`, route
progress; seats in `CharacterSave`/`sf/ai`). Co-op: a mount belongs to its buyer's `PlayerKey`; ride-along travel is
party-wide (all players in the party board, host decides; net.md "Time, pause and rest" for the time skip).

## Owner questions (adapter)

1. Breakables: only crates, barrels, urns and sacks, or also pottery items (redware, `misc_com_*` plates) and doors?
2. Breaking owned containers: `OT_Trespassing` (like picking an owned lock) or a new vandalism offence?
3. Guar mounts: buy from herders, quest reward, or both? Pack-guar cargo in the first mount slice?
4. Ride-along travel for striders and boats on by default in the modern preset, or opt-in?
5. Show a "Sit" prompt on furniture statics (vanilla has no tooltip for statics)?

Answers (DECISIONS 2026-11-11, 2026-11-25): (1) everything breakable breaks, pottery included; doors are bashed
open, never destroyed. (2) A new vandalism offence. (3) Open.
(4) On in the modern preset, fade in Classic. (5) Yes. Mount first slice: climb on, then drive (IN10).
