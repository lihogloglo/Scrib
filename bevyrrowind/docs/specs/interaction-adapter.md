# Interaction adapter: Morrowind furniture, crates, beds, mounts and travel on `sf_interaction`

Status: design 2026-11-11 (ROADMAP 3.11), nothing built. Framework spec:
[interaction.md](../../../superfargoth/docs/specs/interaction.md); research:
[world-interactivity-2026-11-11](../../../superfargoth/docs/research/world-interactivity-2026-11-11.md). Related:
[motion-generation.md](motion-generation.md) (MG4 census, clips), [ai-adapter.md](ai-adapter.md) (AIWander, anchors),
[physics-adapter.md](physics-adapter.md) (pick vs grab, ownership), [crime-adapter.md](crime-adapter.md) (offences),
[services-adapter.md](services-adapter.md) (travel).

## Morrowind has none of this

Vanilla Morrowind and OpenMW have no furniture use, no destruction and no mounts: chairs and beds are statics or
activators, crates are containers, silt striders, boats and gondolas are travel services that fade and advance time.
Everything here is **modern** (owner: modern first, classic behind a seam). The **Classic preset** keeps vanilla:
furniture inert, nothing breaks, travel fades; each feature is an adapter switch (`mw.rules.interaction.*`: `sit`,
`break`, `move_furniture`, `mounts`, `ride_travel`), so a player who chooses classic sees Morrowind.

## Which records become interactable

**Furniture** (from the MG4 census, `mw_bake::census`, motion-generation.md "As built (MG4)"): `STAT`/`ACTI`/`CONT`
whose `furn_*`/`active_*` id or model names a bed/bedroll/hammock/bunk, counter/bar, table/desk, chair/stool/bench/
throne or shelf. Vanilla + expansions: 32 seat, 28 table, 12 counter, 46 bed, 28 shelf base records, ~10.3k
placements, 97 % at scale 1.00. Measured bands: seat 0.14 / 0.48 / 0.54 / 0.64 m, table 0.84 / 0.98, counter 0.94, bed
0.22 / 0.52 / 0.64 / 0.74 (Morrowind furniture runs 15-30 % above real-world heights; plan clips against these).

- `InteractionContent::def_for(base, model)` maps the census kind to an `InteractableDef`: seat → one `seat` slot
  (benches: one per 0.55 m of seat length, exclusive `group` for overlapping ones), bed → `bed` slot (+ `seat` on
  its edge), table/counter → `counter` slots along usable edges (stand and lean), shelf → none (later: `browse`).
  Slot frames, bands and entrance sides come from `interact.cook` on the baked collision (the census top-surface code
  promoted to a cook step); a side blocked by a wall in a placement is dropped at runtime by `EndPoseFree`.
- Mods add or patch `sf.interact.def` records keyed by base id (a mod chair with a better seat frame wins over the
  cook), so content stays a mod (rule 3).
- **Sitting needs no promotion**: slots are derived from the placement, the character attaches; the static stays in
  the static scene. Only moving or breaking a static promotes it (static-scene.md §1).

**Activation order** (scripting-adapter.md "MWScript semantics" 4): a script containing `OnActivate` replaces the
object's default activation and must call `Activate` to perform it, so a scripted seat sits only when its script
allows it. Unscripted seats: tap Activate = sit. Statics have no name and no
tooltip in vanilla (`hasToolTip`); the adapter gives seat slots a verb-only prompt ("Sit"), off in Classic.

## Beds and resting

- Beds that are `ACTI` with a rest script (vanilla `Bed_Standard`-style scripts; the census lists `active_*` beds;
  verify the script set at IN3) keep their behaviour: activation opens the rest menu, now after a lie-down clip.
  Sleeping in an owned bed is `OT_SleepingInOwnedBed` (crime-adapter.md "Offences"), unchanged.
- Static `furn_*` beds: lie down only; resting follows the normal "can rest here" rules (`GetPCSleep`, illegal-rest
  cells), not the bed.
- NPCs: `AIWander` idles become slot activities when slots are near (ai-adapter.md mapping table); the `home` anchor
  is an owned bed (AI14); NPCs sleep in their own or their faction's beds (ownership on the placement).

## Crates, barrels and other breakables (IN1)

- **First set**: `CONT` placements whose model is a wooden crate, barrel, clay urn or sack (`contain_crate_*`,
  `barrel_*`, `contain_urn_*`, `contain_sack_*`-style ids; the list is a data table in `mw_interaction`, checked
  against the install at cook time). Organic containers (plants, `flora_*`), chests, closets, desks and corpses do not
  break. Materials: wood (crate, barrel), clay (urn), cloth (sack).
- **Damage** (`DestructionRules`): blunt weapons full, blades ×0.5, arrows ×0.25 (a barrel takes a few arrows), hand to
  hand ×0.25, fire spells on wood ×1.5; impacts from thrown and falling objects by the physics threshold. Numbers are
  feel tuning (modern side), printed by measurement tests.
- **On break**: the container's contents (its resolved levelled lists, owner kept on each item) spill as placed items
  around the break point (physics items, asleep after settling), or into a loot bag (owner question). A locked crate
  breaks as well: breaking it open is a lock bypass.
- **Crime**: taking a spilled owned item is theft as today (items keep their owner). Breaking an owned container in
  view is an offence: the adapter maps it to `OT_Trespassing` through the call OpenMW makes for an attempt on an owned
  lock (`unlockAttempted`, MM 954-966, crime-adapter.md "Locks and traps"), unless the owner wants a vandalism offence.
- **Scripts**: a container with an `OnActivate` or `OnDeath`-like script, or named by any script (`mw_script` index),
  is unbreakable by default (quest containers must not vanish); the adapter's eligibility table reports them.
- Debris: generic wood planks, clay shards and cloth scraps (pack meshes, cosmetic).

## Moving furniture (IN5, IN6, IN9)

- **Pull-out**: chairs and stools (not benches, thrones, prayer stools) get `pre_move: Slide` along their forward axis
  (0.35 m) when the entrance is behind a table edge (table found by the cook's neighbour search).
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
  arrival, same time cost). Classic: fade only (default for now).
- Routes: authored splines per destination pair (pack data, `mw.travel.route` records), a straight fallback only over
  water; no route = fade. Silt strider placements are animated statics (`a_siltstrider`-style meshes; check NIF
  controllers at IN7); the ride moves a promoted copy, the docked one is hidden for the trip.
- Walking on a boat's deck during a ride needs moving platforms (IN11, with 3.7 buoyancy).

## Saves and co-op

Nothing adapter-specific beyond the framework (`interaction` overlay blob: stage, health, route progress; seats in
`CharacterSave`/`sf/ai`). Co-op: a mount belongs to its buyer's `PlayerKey`; ride-along travel is party-wide (all
players in the party board, host decides; net.md "Time, pause and rest" for the time skip).

## Owner questions (adapter)

1. Breakables: only crates, barrels, urns and sacks, or also pottery items (redware, `misc_com_*` plates) and doors?
2. Breaking owned containers: `OT_Trespassing` (like picking an owned lock) or a new vandalism offence?
3. Guar mounts: buy from herders, quest reward, or both? Pack-guar cargo in the first mount slice?
4. Ride-along travel for striders and boats on by default in the modern preset, or opt-in?
5. Show a "Sit" prompt on furniture statics (vanilla has no tooltip for statics)?
