# Physics adapter (Morrowind → sf_physics, sf_ragdoll, cloth)

Status: draft (2026-10-12), spec only; the five decisions owner-confirmed 2026-10-12 (ARCHITECTURE §10)
Crates: `mw_physics` (new, Bevy-free: `PhysicsContent`, `RagdollRules`, `ClothRules` impls, weight → mass,
holder limits), `mw_bake` (`BodyRules` for `body.cook`, registration of `body.cook`, `items.check`,
`ragdoll.cook`, `cloth.cook`), `mw_world` (placement classes, unchanged), `mw_actors` and `mw_graphs`
(death, knockdown and get-up states), app wiring in `apps/bevyrrowind` (`physics.rs`, dev keys)
Framework spec: `../../../superfargoth/docs/specs/physics.md`

## Goal

Make Morrowind's loose items physical, its NPCs and creatures ragdoll, and its robes and skirts swing,
with no new art. Morrowind itself has none of this: items never move unless picked up or dropped
(dropped items land at the actor's feet), deaths are clips, and robes are rigid or skinned meshes with
no simulation. OpenMW (Bullet) keeps the same behavior; it uses physics for actor collision only.
This is a modern-behavior change (CLAUDE.md rule 6), kept data-driven so mods can turn parts off.

## Items

**Which placements get bodies** (`mw_world::source::resolve_base`, already `PlacementClass::Dynamic`):
MISC (keys and gold included), WEAP, ARMO, CLOT, BOOK, ALCH, APPA, INGR, LOCK, PROB, REPA, and LEVI once
resolved to an item. Every other class stays static collision (`MwCollisionRules`, actors-adapter.md
"Static collision as built"): STAT, CONT, ACTI, DOOR. LIGH stays `PlacementClass::Light` (its light
entity is unchanged); a light with the *Can Carry* flag also gets a body, inserted `HOLD_UNTIL_GRABBED`,
because most carryable lanterns and candles in vanilla hang or stand where nothing would hold them.

**Mass** (`mw_physics::item_mass`): `kg = weight × KG_PER_WEIGHT` with `KG_PER_WEIGHT = 0.5` (Morrowind
weights read as roughly pounds; a visual calibration constant, not a rule). Weight 0 (gold, many
clutter MISC) uses the shape's volume × 400 kg/m³. The framework clamps simulated mass to
`[0.2, 60]` kg; burden and encumbrance keep the record weight. Layer: `DEBRIS` under 15 kg and 0.6 m,
else `PROP` (framework defaults; anvils, large urns and cuirasses on a rack land in `PROP`).

**Shape** (`BodyRules for MwBodyRules`): the model's `RootCollisionNode` subtree when it has one (its
triangles are hulled per node), else the framework fit (primitive, hull, or decomposition). Unlike
statics, the `NC` marker is ignored for items: an item must be grabbable. Skinned item models (none
expected) fall back to the rest-pose render geometry. Census at implementation: item models with
authored collision, shape kinds chosen, hull counts, bake time (`body_report` example, like
`collision_report`).

**Authored placement** (`items.check` report): vanilla items often sit a few millimeters into tables or
hover above them, because the original engine never simulates them. They stay exactly as placed until
disturbed; the framework's capped depenetration resolves small overlaps when they wake. The report
lists overlaps over 1 cm and unsupported items by cell, so obvious authoring errors can get an
adapter fix list (a RON file of placement-id nudges, like other content fixes).

**Scripts** (MWScript → Luau, M5, needs these hooks): `Disable`/`Enable` remove and reinsert the body;
`SetPos`, `SetAngle`, `Position`, `PositionCell` teleport it (asleep); `PlaceItem`, `PlaceAtPC`, `Drop`
and the player's inventory drop spawn an awake body (an overlay spawn, streaming.md); `GetPos`/`GetAngle`
read the body's pose. Ownership is unchanged: knocking an owned item over is not theft, taking it is
(M10 may add reactions to disturbed property).

**Pick up vs grab** (owner-confirmed 2026-10-12). A tap of Activate picks the item into the inventory, as in the
original; holding Activate grabs it (carry, rotate, drop, throw, or drag when too heavy). On a corpse, a tap opens
its inventory (looting, as in the original) and holding Activate drags it by the limb under the crosshair. Bindings
land with M5 input; M4 uses a dev binding.

**Holder limits from Strength** (`PhysicsContent::holder_strength`; owner request 2026-10-12; constants are tuning,
calibrated by eye in PH-4). `S` is the holder's current Strength attribute (Fortify and Drain Strength apply), so
the same mapping serves the player and, later, NPCs that carry or drag:

| Limit | Formula | S = 30 | S = 50 | S = 100 |
|---|---|---|---|---|
| `carry_force` (grab force cap) | `40 + 4·S` N | 160 N | 240 N | 440 N |
| `carry_mass` (heaviest item lifted) | `5 + 0.2·S` kg | 11 kg | 15 kg | 25 kg |
| `drag_mass` (heaviest object dragged) | `40 + 1.6·S` kg | 88 kg | 120 kg | 200 kg |
| `throw_impulse` | `2 + 0.08·S` N·s | 4.4 | 6 | 10 |

Masses compare against the simulated mass (items: `item_mass` above, clamped at 60 kg) or, for a corpse, the
ragdoll's total mass (75 kg × race factors, "Ragdolls"). So an average character drags any NPC body and carries a
bucket but not an anvil; a weak one (S 30) can drag a light corpse, struggles with a heavy Nord (the force cap makes
it lag), and can't move one past 88 kg. Throw speed is capped by the framework at 15 m/s. Fatigue drain on throws and
drags is M10.

## Ragdolls

**Skeletons.** `base_anim.nif` (humanoid), `base_anim_female.nif`, `base_animkna.nif` (beast races:
digitigrade legs and the tail chain) and every creature skeleton go through `ragdoll.cook` from their
`RigProfile` (actors-adapter.md, rig-profile.md). Override files (RON, by skeleton path) patch limits
and capsules where the automatic fit is off; mods use the same files.

**Policy per creature** (`RagdollRules`, a RON table keyed by CREA id with a default by rig template):

| Case | Policy |
|---|---|
| NPCs, humanoid and biped creatures (dremora, golden saints, skeletons, ash creatures, Dagoth, Vivec) | Ragdoll |
| Flesh creatures (guar, kagouti, alit, nix-hound, rats, mudcrabs, dreugh, cliff racers) | Ragdoll (flyers fall) |
| Incorporeal: ghosts, spectres, will-o-the-wisps (template `Floater` by default) | DeathClip |
| Atronachs, Dwemer centurions, storm atronach rocks | DeathClip in M4 (collapse into rigid pieces is later) |

**Total mass.** NPCs: 75 kg × the RACE record's weight factor × its height factor (per sex); creatures:
the cooked body volume at 1000 kg/m³ unless the table overrides it.

**States** (`mw_graphs` humanoid and creature templates gain `PoseInput` states):
- *Death*: the chosen `Death1-5` clip is the drive target while strength fades to limp over 0.3–0.6 s
  (framework "Death"). Clips stay the fallback for `DeathClip` creatures and beyond the powered budget.
- *Knockdown*: Morrowind's knockdown (a hit while fatigue is low, Hand-to-Hand, some spells) maps to a
  physical knockdown; *knockout* (fatigue below 0) to limp until fatigue recovers. Get-up uses the
  `Knockout@tail` section (the get-up after the loop stop, actors-adapter.md "Graph assets as built")
  for face-up; face-down needs a second clip: a bring-your-own library clip if one is mapped
  (animation-sources.md), else the same clip after a physical roll-over (scaffold `ROLL_TO_BACK`,
  removed when a face-down get-up clip exists).
- *Hit reactions*: `Hit1-5` stay the clip layer; physical hit impulses apply to A0 characters inside the
  powered budget.
- *Placed corpses* (references whose actor is dead at load): settle once as a limp ragdoll from the last
  frame of their death clip when their cell activates (a budgeted queue), then sleep; the settled pose is
  persisted in the `ragdoll` state blob.

## Cloth

**Which parts** (`ClothRules`): body parts of CLOT type *Robe* and *Skirt* below the waist line, attached
to `Bip01 Pelvis`. Default lattice: robes 10 columns × 5 rows (ring), skirts 8 × 3 (ring). Vanilla has
no capes; mods mark parts as cloth (`kind: cape`, attachment joint `Bip01 Spine2`) in the same RON file.
Census at implementation: robe and skirt part models, which are skinned and which rigid (the cook works
on both: rigid parts are already rebound by `smooth_outfit`), vertex counts, cook time.

**Colliders**: thigh, calf and pelvis capsules from the skeleton's `RagdollDef`, plus the tail chain for
beast races (the robe ↔ tail poke-through smooth-bodies.md measured is the same contact).

**Order with smooth bodies**: `cloth.cook` rebinds the hem on the part's rest shape per (part, skeleton);
`smooth_outfit` (smooth-bodies.md) then welds the waist seams of the assembled outfit as today, keeping
the lattice weights below the attachment line.

## Test scenes (for `docs/visual-checks.md`, status "awaiting owner" once built)

| Id | Scene | What to look for |
|---|---|---|
| PH-1 table knock-over | `cargo dev view "Seyda Neen, Arrille's Tradehouse"`, push and throw into the counter clutter; synthetic `physics_table` example | items rest exactly as authored until touched; then topple, roll and settle within 3 s; no jitter, no item through the counter; bottles roll, plates slide |
| PH-2 dead guard | Balmora exterior on the stairs by the guard tower; dev key kills the nearest guard (and a second key knocks one down) | the guard crumples from his death clip into a ragdoll down the stairs, limbs within limits, no stretching, rests; the knocked-down guard gets up on the stairs with feet placed |
| PH-3 robes | `character --walk` with a robe outfit (e.g. `common_robe_01`; verify the id) and `"Balmora, Temple"` priests | hem swings with the stride and lags on turns, no legs through the robe at a run, waist doesn't move |
| PH-4 carry | grab a bucket or bottle in Arrille's Tradehouse, walk up the stairs at 30, 60 and 144 fps (a dev `--fps-cap` flag, added by the carry task) | no vibration of the held item relative to the view; heavy items drag; throw arcs; hold Activate on a dead guard (PH-2) and drag him by a wrist, then by an ankle: the body follows the grabbed limb, no stretching; with Strength lowered (console) the same body barely moves 
| PH-5 persistence | knock items over in PH-1, walk out of the cell and far enough to unload, return | every moved item is where it came to rest |

## Industry comparison

| Technique | Original / OpenMW / later Bethesda games and mods | Us | Verdict |
|---|---|---|---|
| Loose items | Original and OpenMW: static until taken; Oblivion, Skyrim, Fallout: Havok items that wake on contact; MWSE "Perfect Placement" (community mod) lets players place items by hand | Physical, asleep until touched, overlay persistence | **better** than the original (modern behavior, rule 6) |
| Deaths | Original and OpenMW: death clips; Oblivion onward: Havok ragdolls after death | Clip-driven crumple into a ragdoll, persisted pose | **match** later Bethesda, **better** than the original |
| Knockdown | Original: knockdown clip and get-up | Physical knockdown and the original get-up clip | **better** |
| Robes | Original: rigid or skinned, no motion; Skyrim community: HDT-SMP / CBPC cloth physics mods on skinned capes and skirts | Bone-lattice cloth, no new art | **match** the modding standard |

## Open questions

1. `KG_PER_WEIGHT` and the Strength mapping constants: calibrated in PH-1/PH-4 by eye (owner-confirmed approach,
   2026-10-12).
2. Whether NPCs walking through clutter should push `DEBRIS` (framework default yes) or avoid it (M10 AI).
3. Dwemer centurion and atronach collapse into rigid parts (after M4).
