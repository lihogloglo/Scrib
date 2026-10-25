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

**As built (P3, 2026-10-13).** `mw_bake::body`: `MwBodyRules` (every mesh under a `RootCollisionNode`, any
visibility, one hull each; `NC` ignored; skinned models fit the render mesh), `MwItemsRules` (items =
`PlacementClass::Dynamic` + lights with *Can Carry*; carryable lights `always_hold`; terrain in the exterior; the
carryable set's digest is a key parameter), `item_mass` (moves to `mw_physics` with P4), `bake_bodies` /
`check_items` (both run in `bevyrrowind bake`, which prints their reports), example `body_report` (body census;
`items.check` needs the world index, so its report comes from `bevyrrowind bake`). Census: see ROADMAP M4 P3.

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
it lag), and can't move one past 88 kg. Throw speed is capped by the framework at 15 m/s. The framework caps the *drag* force at what slides the drag mass (`max(carry_force, 0.6·g·drag_mass)`, physics.md P6 as built): the `carry_force` column is the carry grip only. Fatigue drain on throws and
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

**As built (P7, 2026-10-13).** `mw_physics::ragdoll_params(skeleton, humanoid)`: humanoid skeletons (the actor sets'
`humanoid` flag: NPC races, biped creatures such as dremora, skeletons, liches, saints) weigh `HUMANOID_MASS` 75 kg
at scale 1; creatures keep their capsules' volume × 1000 kg/m³ (no skin extents yet: see the census outliers in
ROADMAP M4 P7); models naming `ghost`, `spect`, `wraith` or `wisp` keep the death clip (`RagdollPolicy::DeathClip`).
`mw_bake::ragdoll` (`MwRagdollRules`, `bake_ragdolls`) cooks every actor skeleton but first-person ones inside
`bevyrrowind bake`'s actor phase (census line `ragdolls:`; `BEVYRROWIND_RAGDOLL_REPORT=1` lists every skeleton). The
app's debug driver: `I` kills the character nearest the crosshair (graph `dead` flag, mover `Dormant`, `Ragdoll`
after a 0.4 s death clip); corpses drag by the grabbed limb through the P6 carry driver.

## Cloth

**Decision (owner 2026-10-19, p10s; research `../superfargoth/docs/research/cloth/long-robes.md`): robes and skirts
are drawn with their authored skin by default.** PH-3 V1 failed because the lattice replaced the robe's authored
thigh/calf weights below the pelvis, so the hidden legs could step out of it. Shipped games and vanilla Morrowind
skin long garments to the legs and hide the body under them; simulation is at most an extra on top. So
`mw_physics::MwClothRules` defaults to `ClothMode::Authored`: `ClothRules::cloth` answers `None` for every part, and
`sf_character_bevy` takes its "drawn as authored" path (no cook, no rebind, no lattice joints, no op: 0 µs per robe).
**Opt-in: `--cloth robes`** (`ClothMode::Robes`, on `view`/`scene`/`level` and `character`) restores the V1
simulated robes below, for A/B and mods; **H** then toggles simulated ↔ kinematic as before. `kind()`/`parts()` and
the census still list the cloth parts. Parked (only matter for simulated robes): leg-follow tuning
(`CLOTH_LEG_FOLLOW` 0.7), hem widening for a run stride, the `cloth.cook` bake step (keeps scaffold
`ClothCookAtLoad` until a default path cooks cloth, i.e. capes and mesh cloth). The solver, op, colliders and `H`
stay for capes, banners and tents. Next slice (S2, owner idea, low priority): hem-only drape on the authored skin
(ROADMAP P10, long-robes.md S2). The V1 text below describes the opt-in.

**Which parts** (`ClothRules`): body parts of CLOT type *Robe* and *Skirt* below the waist line, attached
to `Bip01 Pelvis`. Default lattice: robes 10 columns × 5 rows (ring), skirts 8 × 3 (ring). Vanilla has
no capes; mods mark parts as cloth (`kind: cape`, attachment joint `Bip01 Spine2`) in the same RON file.
Census at implementation: robe and skirt part models, which are skinned and which rigid (the cook works
on both: rigid parts are already rebound by `smooth_outfit`), vertex counts, cook time.

**As built (p10b, 2026-10-18): `mw_physics::MwClothRules`** (`cloth.rs`) and the census
(`cargo run --release -p mw_actors --example cloth_census`). The census corrected the rule above: **robes don't use
the Skirt slot.** All 64 robe items (Morrowind + Tribunal + Bloodmoon) are one skinned *Chest* part from the
shoulders to the ankles plus two ankle parts, so a robe's cloth is its Chest part below `Bip01 Pelvis` (vertices
above keep their skin). Skirts: 20 items, 14 with a *Skirt* part, 5 with only a *Groin* part (left as is), 6 with
neither. Rules: `(BODY, slot)` pairs, robe Chest → 10 × 5 ring, skirt Skirt → 8 × 3 ring, `ClothParams` defaults,
attachment `Bip01 Pelvis` at its rest height. 50 cloth parts, **all skinned** (0 rigid): robes 527–1701 vertices,
skirts 145–412; 48 cook (`c_skirt_extravagant_1` is a rigid mesh on a node named after its file, `werewolfskin`
needs `Bip01 Tail`: both fail cleanly); cook 9 ms median, 68 ms max per part including the NIF import. Hems:
robes 0.0–0.33 m above the ground (most at the ankles), skirts 0.10–0.53 m. **Every hem is narrower than a run
stride needs** (+13 % to +133 % stretch, physics.md "As built (p10b)"): p10c blends the lower rows toward the
authored leg skin. Census line items to recheck in PH-3: female robe parts are only 2 (most robes reuse the male
part on female bodies), and beast races wear the same parts on `base_animkna.nif` (tail collider).

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
| PH-3 robes | `character "fargoth+common_robe_01" "@dark elf,,f+common_robe_02" --walk`, `scene level-robe-walk`, `scene ph3-balmora-temple` | robes drawn as authored (default): legs always covered at walk and run, the robe follows the stride, no seam at the waist |
| PH-4 carry | grab a bucket or bottle in Arrille's Tradehouse, walk up the stairs at 30, 60 and 144 fps (a dev `--fps-cap` flag, added by the carry task) | no vibration of the held item relative to the view; heavy items drag; throw arcs; hold Activate on a dead guard (PH-2) and drag him by a wrist, then by an ankle: the body follows the grabbed limb, no stretching; with Strength lowered (console) the same body barely moves 
| PH-5 persistence | knock items over in PH-1, walk out of the cell and far enough to unload, return | every moved item is where it came to rest |

## Industry comparison

| Technique | Original / OpenMW / later Bethesda games and mods | Us | Verdict |
|---|---|---|---|
| Loose items | Original and OpenMW: static until taken; Oblivion, Skyrim, Fallout: Havok items that wake on contact; MWSE "Perfect Placement" (community mod) lets players place items by hand | Physical, asleep until touched, overlay persistence | **better** than the original (modern behavior, rule 6) |
| Deaths | Original and OpenMW: death clips; Oblivion onward: Havok ragdolls after death | Clip-driven crumple into a ragdoll, persisted pose | **match** later Bethesda, **better** than the original |
| Knockdown | Original: knockdown clip and get-up | Physical knockdown and the original get-up clip | **better** |
| Robes | Original: rigid or skinned, no motion; Skyrim community: HDT-SMP / CBPC cloth physics mods on skinned capes and skirts | Authored skin by default (owner 2026-10-19); bone-lattice cloth as the `--cloth robes` opt-in | **match** the original and shipped games; opt-in matches the modding standard |

## Open questions

1. `KG_PER_WEIGHT` and the Strength mapping constants: calibrated in PH-1/PH-4 by eye (owner-confirmed approach,
   2026-10-12).
2. Whether NPCs walking through clutter should push `DEBRIS` (framework default yes) or avoid it (M10 AI).
3. Dwemer centurion and atronach collapse into rigid parts (after M4).
