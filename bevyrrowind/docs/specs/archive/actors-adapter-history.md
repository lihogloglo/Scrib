# actors-adapter history

Dated "As built" blocks moved verbatim from [actors-adapter.md](../actors-adapter.md) (doc cleanup S10a, 2026-10-26).

## KF import as built (2026-10-12, `mw_anim`)

Crate `bevyrrowind/crates/mw_anim` (Bevy-free; deps `mw_nif`, `sf_anim` without features, glam).
`mw_nif` gained only `kf` (`load_kf` → `KeyframeFile { text_keys, tracks }`) and `Curve::keys()`.

- **Reading.** A `.kf` root is an `NiSequenceStreamHelper`: its extra-data chain holds the text
  keys, then one `NiStringExtraData` per bone; its controller chain holds the matching
  `NiKeyframeController`s in the same order (pair by position, as the original engine does).
  Animated `.nif`s go through `mw_nif::load` (`Animation::from_nif`): their `Target::Transform`
  controllers, named by node. Curves are engine space (meters, Y up), like `mw_nif` models.
- **Groups** (`mw_anim::groups`): one per text-key prefix other than `SoundGen`/`Sound`.
  Range `Start`..first later `Stop`; groups without those keys (weapon groups, `SpellCast`,
  `Shield`: only sub-action keys) span their first..last key. Case-insensitive; the data's
  variants `Stop.`, `Start Loop`/`Stop Loop` are folded. `Loop Start`/`Loop Stop` → loop range.
  Default looping (`Group::looping`): loop keys, else whole-clip for `idle*`, `walk*`, `run*`,
  `sneak*`, `turn*`, `swimwalk*`/`swimrun*`/`swimturn*`, else once.
- **Clips.** Each group → one `SourceClip` named after it, times from the group start. Hermite
  translation/scale stay `CubicSpline` (NIF per-segment tangents ÷ segment length = per-second;
  cuts add a key with the value and derivative there, so the cut curve is the same cubic);
  quaternion keys stay linear (slerp in both crates); Euler tracks are sampled at key times and
  ≥ 60 Hz. `KfAnimImporter` (`AnimImporter`, extension `kf`) returns all groups;
  `AnimSource::root_motion` = `Bip01`. `clip_rules(group)` gives the group's `Looping`.
- **Sections.** Sub-actions inside a group (`chop`, `slash`, `thrust`, `shoot`, `equip`,
  `unequip`, `self`, `touch`, `target`, `block`) become `Section { name, start, stop }`
  (stop = last `Stop` or follow-through stop); `section_rules(section)` cuts a clip to it. The
  humanoid graph plays these (Morrowind plays weapon groups by sub-range, not by group).
- **Events** (names in `mw_anim::events`, payload lowercase unless an id):

| Text key | Event | Payload |
|---|---|---|
| `SoundGen: Left/Right/SwimLeft/SwimRight` | `footstep` | `left`, `right`, `swim_left`, `swim_right` |
| `SoundGen: Land` | `land` | |
| `SoundGen: Moan/Roar/Scream` | `sound_gen` | the kind |
| `Sound: <id>` | `sound` | the id, as written |
| `<a> Min Attack`..`<a> Max Attack` | window `attack.charge` | `chop`/`slash`/`thrust`/`shoot` |
| `<a> Min Hit`, `<a> Hit`, bare `Hit` (creatures) | `attack.min_hit`, `attack.hit` | attack (empty for bare) |
| `<a> [Small/Medium/Large] Follow Start`..`Stop` | window `attack.follow` | `chop small`, ..., `shoot` |
| `Shoot Attach`, `Shoot Release`, `Shoot Follow Attach` | `attack.attach`, `attack.release`, `attack.follow_attach` | `shoot` |
| `Equip Attach`, `Unequip Detach` | `equip.attach`, `unequip.detach` | |
| `Self/Touch/Target Release` | `cast.release` | `self`/`touch`/`target` |
| `Block Hit` | `block.hit` | |
| anything else | `text_key` | the whole line |

  `SoundGen`/`Sound` keys belong to every group containing their time (shared boundaries
  included); group-prefixed keys only to their group.
- **Data** (vanilla + expansions, `cargo test -p mw_anim`): 162 `.kf` files → 2330 clips, every
  file has groups, every text key maps (no `text_key` left). `xbase_anim.kf`: 34 bones, 841 key
  lines, 135 groups (Idle..Idle9, movement × hh/1h/2c/2w stances, Hit1-5, Death1-5, weapon groups
  with 5 sections each, `SpellCast` equip/unequip/self/target/touch, `Shield` block); keys every
  1/15 s. `xbase_anim.1st.kf` 54 bones/103 groups, `xbase_animkna.kf` 40/133,
  `xbase_anim_female.kf` 33/3 (female overrides of Idle4 etc., layered over the male file).
  `base_anim.nif` carries 34 keyframe controllers but no text keys: the KFs are the source.
- **Inspect:** `cargo run -p mw_anim --example anim_report -- meshes/xbase_anim.kf [--events]
  [--group <name>]` (`mw_anim::report`). The app subcommand `bevyrrowind anim <kf>` is a
  follow-up thin wrapper over `report` (the app was being edited by another session).
- Not here: skins, morphs (need `sf_model` skin data), the bake step wiring
  (`mw_bake` registering `KfAnimImporter`, female/beast/1st-person layering of KFs).


## Rig maps as built (2026-10-12, `mw_anim::rig`)

Code: `bevyrrowind/crates/mw_anim/src/rig.rs` (deps gained `sf_character`). Inspect with
`cargo run -p mw_anim --example rig_report -- meshes/base_anim.nif --humanoid [--tree]` or
`-- meshes/r/guar.nif`; `-- --creatures <id substrings>` lists CREA ids, models and flags.

- **Skeleton** (`rig::skeleton(&Model)`): transform nodes only (no geometry, particles, collision).
  When the root holds `Bip01` (or `Root Bone`: cliff racer, centurion spider), only that subtree
  is kept under the root: the root's other children are meshes, shadow boxes, `Bounding Box` and
  editor dummies (the guar's `Dummy15` sits 0.5 m below the ground and broke support detection).
  Body-part sockets inside it stay. Duplicate names get ` #n`. Root motion joint: the bones root,
  `Bip01`, else `Root Bone` (`rig::root_motion_joint`; also the KF importers' `root_motion`).
- **Root motion per creature (as built, rootmotion 2026-10-12).** Census
  `mw_bake/tests/creature_root_motion_real_install.rs` (507 walk/run/swim clips, 89 CREA skeletons):
  74 skeletons travel on `Bip01`, 15 on `Root Bone` (babelfish, bonelord, cave mudcrab, cliff racer,
  dreugh, centurion spider, kwama forager, scrib, netch betty/bull, shalk, slaughterfish, sphere
  centurion…). Before, only `Bip01` was a root-motion joint, so the `Root Bone` cycles kept 0.3-82 m
  of travel in the bones: in the lineup they walked off and snapped back at each loop while the
  `Bip01` creatures stepped in place (owner note on NC-1). Now every clip's travel is extracted (0
  flagged; the census also cooks one clip per skeleton without extraction as a control). The rule,
  for every actor: locomotion clips are `Match`; the travel lives on the root-motion track, never in
  the bones. The world's mover drives and the track only sets the playback rate (SpeedMatch); the
  `character` lineup stores it on `AnimRootMotion` without applying it (key 3 applies it: each actor
  walks off at its clip's speed). Odd authored speeds, not fixed: cliff racer 43 m/s walk, dreugh
  6.6 m/s (Match clamps the rate to 0.75-1.3); netch bull `WalkForward` and nixhound `RunForward`
  have loop seams in some limbs (0.57 / 0.25 m median drift, not travel).
- **Facing.** Morrowind actors face −Z in engine space (data: toes and thumbs at −Z). The framework
  faces +Z (character.md: "an adapter whose native clips face elsewhere turns its skeleton at
  import"), so the root joint's rest gains a 180° Y turn (`rig::FACING_TURN`). KF tracks animate
  `Bip01` and below in the root's frame and root motion is extracted in model space, so clips and
  root motion face +Z with no clip change. **Actor spawning must add the same half turn** to a
  reference's yaw. Auto-align then needs no facing fix (0.0°), and `sf_anim::rig` detection reads
  sides and front/back correctly (before the turn, every limb's side was flipped).
- **Humanoid** (`rig::humanoid`): `sf_character::auto_map` on the skeleton's `Bip01*` joints only
  (`rig::bone_skeleton`, same model-space rest; names unchanged so the map holds for the full
  skeleton), root motion forced to `Bip01` (auto-map picks the hips), then `auto_align`. That is
  the whole hint table (`rig::humanoid_hints`): sockets (`Head`, `Neck`, `Chest`, `Groin`,
  `Left Hand`, `Right Upper Leg`, `Tail`, ...), `Weapon Bone`/`Shield Bone`, the first-person
  `Camera` and an unnamed node under it are not bones. Mapped with them, they became one-joint
  extra chains (one named `head`, colliding with the head chain), the base skeleton's `Left Foot`
  socket was taken as toes, and the beast tail chain stopped at its `Tail` socket.
- **Result** (all four clean: no warnings, errors or missing required bones; align residual
  ≤ 0.02°): `base_anim` and `base_anim_female` 32 bones (Pelvis → hips; Spine/Spine1/Spine2 →
  spine/chest/upperChest; Neck, Head; Clavicle → shoulder; UpperArm/Forearm/Hand; Finger0..2 with
  two segments each; Thigh/Calf/Foot; no toes). `base_animkna` 34: adds Toe0 → toes and extra
  chain `tail` (`Bip01 Tail`..`Tail3`). **No HorseLink in the beast skeleton**: `Bip01 L Foot` is the
  raised ankle (0.24 m up) and `Toe0` the ground contact, so the profile leg is the plantigrade
  `TwoBone` thigh–calf–foot with toe Toe0. `base_anim.1st` 52: Finger0..4 with three segments
  (Finger0 → thumb metacarpal), Toe0.
- **Profiles** (`RigMap::profile`): Biped, support legs `leftLeg`/`rightLeg`, arms from the
  clavicle; the leg girdle is `Bip01 Spine` (3ds Max hangs the thighs there).
- **Masks** (`rig::masks(&Skeleton, &RigMap)` → four `JointMask`s, `BodyGroup`): torso = the chest
  bone's subtree (`Bip01 Spine1`), left/right arm = the shoulder's subtree (`Bip01 L/R Clavicle`,
  with fingers, `Shield Bone`/`Weapon Bone` and arm sockets), lower body = the rest (root, `Bip01`,
  pelvis, `Bip01 Spine`, legs, tail, `Groin`). Same roots as the original engine's blend groups
  (OpenMW: `Bip01 Spine1`, `Bip01 L Clavicle`, `Bip01 R Clavicle`). They partition every
  skeleton (test). Graph assets can write the same rules as `JointRef::Rig("chest")` etc.
- **Retarget round trip** (Quaternius Universal rig → each Bip01 skeleton → back, the
  sf_character test pose): body bones ≤ 0.07°, hips/left hand/right foot < 1 cm; fingers 2.8°
  on the two-segment skeletons (Quaternius has three: the distal bend can't survive a 3 → 2 chain),
  0.07° on first person.

**Creatures** (`rig::creature_profile(&Skeleton, model)`: `sf_anim::detect_profile` with
`rig::creature_hints(model)`, keyed by the model's lowercase file stem):

| Creature (model) | Profile | Hints |
|---|---|---|
| guar (`r\Guar.NIF`) | Biped: it walks on its hind legs; legs Thigh–Calf–HorseLink–Foot, Digitigrade `TwoBoneAnkle`; arms; tail | none |
| kagouti (`r\LeastKagouti.NIF`) | Biped, correctly: the creature has two legs, rigged on the Biped arm bones (Clavicle–UpperArm–Forearm–Hand, Digitigrade `TwoBoneAnkle`, three fingers on the ground); its `Thigh`..`Toe0` are a 4 cm stub in the body (rechecked rigfix 2026-10-12). Gap: the girdle is `Bip01 Neck`, and detection only reads branches between the body root and the head, so `Bip01 Tail` (off `Bip01 Spine`, above the girdle) is not found as a tail | none |
| nix-hound (`r\NixHound.NIF`) | Quadruped; four Digitigrade `TwoBoneAnkle` legs (front from the clavicle to the middle finger, hind Thigh–Calf–Foot–Toe1); neck Neck1–Neck2; tail | the four legs (mid-stride rest pose: one front leg read as plantigrade, hind legs stopped at the 0.7 m-high `Foot`) |
| cliff racer (`r\CliffRacer.NIF`) | Flyer; wings `L/R Wing`..`WingCap` (5 joints, Fabrik); `rudder` Fin; tail `TailRoot`..`TailCap`; head | wings (they branch at every joint, so detection kept one joint; the right spar's third joint is `R Wing_4`) and the rudder (else an arm) |
| centurion spider (`r\G_CenturionSpider.NIF`) | Multiped; six `TwoBone` legs `L/R1..3_A`–`_B`–tip on girdle `Bone00`; no head | none (the six-leg hint went once `sf_anim` stopped the leg walk at a joint carrying legs on both sides, rigfix 2026-10-12) |

Movers (`rig::CreatureMover::from_crea_flags`): `Flies` (0x20) → Fly (cliff racer, netches);
`Swims` (0x10) without `Walks` (0x40) → Swim (slaughterfish); else Ground, including biped
creatures that carry only `Biped` (dremora 0x0d) and amphibious `Walks | Swims`. Every vanilla
CREA also sets 0x08. The controller glue maps it onto `sf_controller::MoverMode`.


## `CharacterSource` as built (2026-10-12, `mw_actors`)

Crate `bevyrrowind/crates/mw_actors` (Bevy-free; deps `mw_anim`, `mw_nif`, `mw_world`, `sf_character`,
`sf_records`, `sf_vfs`, tes3). `MwActors::new(Arc<RecordDb>, Arc<Vfs>)` implements `CharacterSource`
(kinds `NPC_`, `CREA`) and `PartLookup`; resolve with `resolve_recipe(&actors, actors.slots(), &recipe, view)`.
Rules follow the original engine as OpenMW documents it (`NpcAnimation`, `InventoryStore::autoEquipArmor`,
`Armor::getEffectiveArmorRating`, `autoCalculateSkills`; GPL reference, fine here).

- **Framework hook** (character.md "Part selection and socket offsets"): `BodyPart::node_filter` and
  `PartKind::Rigid { joint, offset }`. Skinned parts carry the slot's filter (skin NIFs hold both sides:
  `Tri Left Forearm`/`Tri Right Forearm`; hair uses filter `Hair` on bone `Head`); rigid parts are
  bolted to the slot's bone and mirrored (`offset` = scale(−1, 1, 1)) when the bone name contains `Left`,
  as the original renderer does. NIF X stays X under the Z-up → Y-up conversion, so the mirror axis
  is unchanged in engine space.
- **Slots** (`mw_actors::slots`): humanoid defaults plus `skirt`, `left/right_wrist`, `left/right_ankle`,
  `shield`, `weapon`, `body` (a creature's whole model). PRT → slot: Head, Hair, Neck, Cuirass→chest,
  Groin→pelvis, Skirt→skirt, Hand/Forearm/UpperArm/Foot/UpperLeg→defaults, Knee→`*_lower_leg` (the
  knee part is the lower leg), Pauldron→`*_shoulder`, Wrist/Ankle→new slots, Shield, Weapon, Tail.
  Attach bones from the original part list (`Head`, `Neck`, `Chest`, `Groin`, `Right Hand`, ...,
  `Right Clavicle` for pauldrons, `Shield Bone`, `Weapon Bone`, `Tail`).
- **Parts**, id `"<BODY id>#<slot>"`: one per (BODY, slot) the BODY's mesh part or an item's part list
  allows (2,300 in vanilla + expansions), built on first use through a skinned/rigid probe
  (`mw_nif::load(..).had_skin`; unreadable → skinned, the cook reports it). Creature bodies:
  `"<model path>#body"`, `Skinned` (the model is its own skeleton).
- **Skin** per (race, sex, view): BODY records of type skin, playable, not vampire, in id order, last match
  wins; female bodies fall back to male parts; first person uses `.1st` parts, else third-person arm parts
  (hand, wrist, forearm, upper arm; same sex first). Head and hair come from the NPC record.
- **Wearables**, id `"<item>#m"`/`"<item>#f"` (1,978): parts from the item's list, female BODY if any else
  male; first-person parts from `<BODY>.1st`, else the third-person BODY if it is an arm part. Priorities
  → layers: clothing 2 → `CLOTHING`, armor 3 → `ARMOR`, skirt 8 → `Layer(175)`, robe 24 → `OUTER`. `hides`:
  slots with no BODY found (reserved), the robe's reserve list (groin, skirt, upper legs, knees, upper arms,
  forearms, cuirass), the skirt's (groin, upper legs), and the hair under anything filling or reserving the
  head (open helmets fill the hair slot with their own part and leave the face). Equal priorities: the
  earlier equipment slot wins (robe, skirt, helmet, cuirass, greaves, pauldrons, boots, gauntlets, shirt,
  pants), so recipes list worn items in reverse slot order (resolve lets the later one win a tie).
- **Auto-equip** (`mw_actors::equip`, used when `CharacterState::worn` is `None`): ARMO, then CLOT, in
  inventory order. Armor is skipped at effective rating ≤ unarmored (`fUnarmoredBase1·s × fUnarmoredBase2·s`);
  effective rating = rating × skill / `iBaseArmorSkill` by weight class (`i*Weight` × `fLightMaxMod` /
  `fMedMaxMod`, weight 0 → raw rating). In a taken slot: lower armor type kept (gauntlet over bracer), same
  type → higher rating; armor replaces clothing, clothing never replaces armor, clothing vs clothing by
  value. Beast races refuse items with a head or foot part. Skills: stored, or auto-calculated (5, +25 major
  / +10 minor, race bonus, +5 specialization, (level − 1) × (1 class skill | 0.1, +0.5 specialization), ≤ 100).
  `CharacterState::worn = Some(ids)` accepts item ids or wearable ids and is re-ordered the same way.
- **Recipe**: skeleton = NPC_ model if set (only the four `anim_dancinggirl` NPCs), else `base_animkna`
  (beast) / `base_anim_female` / `base_anim`; `RigRef::Humanoid(skeleton)`; scale (weight, height, weight)
  from the race per sex; first person `base_anim[_female|kna].1st.nif` when present, else `base_anim.1st`.
  `rig_maps()` = `mw_anim::rig::humanoid` on the six humanoid skeletons found. Creatures: skeleton and body
  = the model, `RigRef::Profile(model)`, scale = CREA scale; `creature_mover(id)` from the flags. Graph
  paths are placeholders in `ActorConfig` until the M3 graph assets exist.
- **Measured** (vanilla + expansions, `tests/real_install.rs`, 0.6 s): 3,041 NPC_ and 396 CREA, all give
  recipes; skeletons 1,706 male, 1,058 female, 273 beast, 4 custom. Every visible part of the named actors
  exists in the VFS and every filtered part keeps geometry. Imperial guard (`chargen dock guard`): imperial
  chain cuirass, greaves, pauldrons, gauntlets, boots, helmet in the hair slot over the face, shirt
  forearms between pauldron and gauntlet; 16 rigid parts (helmet, neck, groin, limbs), mirrored exactly on
  `Left` bones. Ajira (Khajiit, female): beast skeleton, tail, robe hides legs and arms; naked: 14 female
  parts, 8 male fallbacks (arms). Huleeya (Argonian): newtscale cuirass, netch greaves, cloth bracers, tail.
  Fargoth naked: skin in every slot. Guar, cliff racer (Fly), centurion spider recipes.
- **Equipment audit** (2026-10-12, `tests/equipment_audit.rs`, missing boots and armor): every NPC_'s
  auto-equipped items against the original renderer's slot rules re-run independently (slot list order,
  priorities 2/3/8/24, robe/skirt reserves, BODY-less references reserve, skin at 1, hair under a head
  part), then every distinct visible part (1,765: all NPC outfits + every wearable, both sexes) built,
  cooked as the bake does, decoded through its node filter and placed on the rest skeleton. Item part
  references (3,041 NPCs): 35,145 shown; legit drops: 5,088 outranked by another item, 2,041 references
  without a BODY (reserved, as the original), 1,036 under a robe/skirt, 13 items beast races refuse, 3
  vanilla models that show nothing (Ten Pace Boots' `Tri Foot` matches neither foot filter, UESP's known
  mesh bug; empty werewolf hair). Slot-by-slot mismatches against the original rules: 0. Bugs fixed:
  (1) **18 rigid part models (36 parts) never rendered**: their root carries a one-key, non-auto-play
  transform controller, so `mesh.cook` skipped them ("animated node"): iron/glass/bonemold/adamantium/
  orcish boots, common shoes 1 and 2, some pauldrons and pants knees, 392 NPCs' default outfits. The bake
  now also cooks them as `mesh.rig` (`mw_bake::anim::rigid_part_needs_rig`, `ActorSets::rigid_parts` in
  `body_rig_keys`), which `MwCharacters::cooked` already prefers; the original never runs controllers on
  body parts other than the head, so the rest pose is what shows. Needs a rebake. (2) Auto-equip used a
  fractional unarmored rating; the original truncates it (52 armor pieces now kept). (3) Leveled-list
  inventory entries are worn through the scaffold `held::leveled_item_pick` (252 items now worn, e.g.
  `random_armor_chitin` → chitin boots). The lineup's bare-legged Nord was a typo in the visual check
  (`iron_boots`; the record is `iron boots`): `CharacterState::worn` skips unknown ids silently.
- **Not here** (follow-ups): held items (now "Held items as built"), vampire heads, werewolves, creature equipment (biped creatures), skin/hair tints,
  rigid parts' `BoneOffset` nodes (weapons/shields), the cook honouring `node_filter`/`offset`, and the
  visual check against OpenMW (needs `sf_character_bevy`).

## Held items as built (2026-10-12, `mw_actors::held`, Bevy-free)

Framework model (character.md "Held items"): `HeldItem { item, model, hand, drawn: Option<Mount>,
sheathed: Option<Mount>, light: Option<HeldLight> }`, `Mount { socket, offset }`, `Readiness` (drawn or
sheathed: runtime, not in the recipe). `MwActors::held(kind, id, HeldContext { dark, in_combat })` returns
them for `NPC_`, `CREA`, `LEVC`; recipes without `CharacterState::held` use the daylight default (no torch),
so the host re-asks at dusk, dawn and cell changes and passes `CharacterState { held: Some(..) }`.
`MwActors::{npc_loadout, creature_loadout}` give the records (`Loadout { weapon, ammo, carried_left }`).
Rules as OpenMW documents them (`InventoryStore::autoEquipWeapon`/`autoEquipShield`,
`Actors::updateEquippedLight`, `World::isDark`, `NpcAnimation::showCarriedLeft`, `getShieldMesh`,
`Animation::updateCarriedLeftVisible`, `SceneUtil::attach`; GPL reference, read from memory of its sources):

- **Weapon** (`Weapon Bone`, main hand): weapon skills from the highest (0 never), ties in the order long
  blade, axe, spear, short blade, marksman, blunt; within the skill the weapon with the highest maximum
  chop/slash/thrust (ties: later item). Bows and crossbows need arrows/bolts in the inventory, else the next
  skill. Ammo: highest max chop (ties: later). Thrown weapons need none; ammo is never wielded.
- **Shield** (`Shield Bone`, off hand): none with a two-handed weapon (bows and crossbows count); best
  condition (`health`, ties: earlier). Model: the shield `BODY` of its part list (female, else male), else the
  ground model.
- **Torch** (`Shield Bone`: it takes the carried-left slot): when dark, the first `LIGH` with `Can Carry`.
  Out of combat it replaces the shield; in combat a shield keeps the slot, a torch fills an empty one. A
  two-handed weapon doesn't stop it (the left slot is free). Light at the model's `AttachLight` node; color
  and radius from `mw_world::light_info`, ignoring `Off by default` (vanilla `torch_infinite_time` has it and
  still shines on NPCs).
- **Darkness** (`held::is_dark(cell, hour, DayCycle)`): exteriors from 19:00 to 6:00 (hour < `Sunrise Time`
  or > `Sunset Time` + `Sunset Duration` − 1; Morrowind.ini, defaults 6/18/2); interiors when ambient
  R + G + B ≤ 201 unless resting is illegal there (shops, homes stay lit). Our choice: interiors that behave
  like exteriors follow the clock. OpenMW tests the player's cell; we test the actor's (same in practice).
- **Visibility.** The original shows **no sheathed weapons**: no scabbards, no quivers, no back-slung
  weapons; a weapon shows only while drawn (`sheathed: None`). The arrow appears on the bow's `ArrowBone`
  during an attack (animation-driven, not a held item). The carried-left item (shield, torch) always shows,
  except while a two-handed weapon is drawn (`drawn: None`). OpenMW's optional weapon/shield sheathing (off
  by default) needs mod content (`Bip01 <weapon type>` sheath bones in `xbase_anim_sh`, `_sh` models):
  not implemented; the `sheathed` mount is where it would go. The spell stance shows the sheathed mounts.
- **Torch arm** (torchfix 2026-10-12). Black box (OpenMW's character controller): while a lit light is the
  carried-left item and shows, the `Torch` group plays from its start to stop keys, looped, on the left-arm
  blend mask (`Bip01 L Clavicle` subtree), at a priority above weapon actions and below scripted groups;
  it stops when the light goes or a two-hander hides it. Ours: `mw_graphs` humanoid/first-person graphs
  have a `torch` float parameter weighting a `torch_layer` (Override, mask `left_arm` = `leftShoulder`
  subtree) between the `upper` slot and the `full` slot; the `Torch` clip is optional (dropped where a
  skeleton lacks it). `held.rs` feeds `torch` with `stance` (`carries_light`). Before: the left arm kept
  the locomotion pose, "locked in front" (owner, HI-4).
- **Offsets.** The model's root transform (as rigid parts), under a `BoneOffset` node's model-space position
  turned −90° about X when the model has one (OpenMW's `SceneUtil::attach`). Neither bone is a `Left` bone:
  nothing is mirrored. Vanilla skeletons have no `Weapon Bone Left` (bows go on `Weapon Bone`).
- **Leveled lists, scaffold** (`held::leveled_item_pick`): a `LEVI` inventory entry holds its lowest-level
  item (ties: first, nested lists followed, `chance_none` ignored), for held items only (guards' `Random
  Weapon`/`Random Shield`). Removal path: `sf_rpg` resolves inventories at instantiation (player level,
  seeded roll). Worn items still skip lists (changing outfits under the live smooth-body A/B was avoided).
- **Creatures**: only with the `Weapon and Shield` flag; every weapon skill = the CREA combat skill (our
  choice; the skill order decides). Same bones (`Weapon Bone`/`Shield Bone` in their models).
- **Stance glue** (`mw_graphs::Stance::of_weapon(readiness, weapon_type)`): sheathed → `none`; drawn
  without a weapon → `hh`; short blade, one-handed long blade/blunt/axe, bow, thrown → `1h`; two-handed
  long blade/blunt/axe → `2c`; staff, spear → `2w`; crossbow → `crossbow`; a readied spell is `Spell`
  whatever the weapon (the host sets it).
- **First person** (notes only): `base_anim.1st.nif` has the same `Weapon Bone`/`Shield Bone`, so the same
  mounts apply to the view model; held items attach to the active view's skeleton.
- **Measured** (`tests/held_install.rs`, 0.5 s): 3,041 NPC_ drawn stances `1h` 1,572, `hh` 1,060, `2c` 261,
  `2w` 137, `crossbow` 11; 82 class-Guard NPCs, 80 armed (long blade 30, blunt 12, short blade 11, two-handed
  long blade 11, axe 8, spear 5, bow 3; the two unarmed carry no weapon); 315 shields by day; 47 NPCs light a
  torch at night (Imperial guards: `torch_infinite_time`, 3.66 m, at `AttachLight`), none by day; Redoran
  shield (Volene Llervu, chitin shield); every chosen model is in the VFS; 152 held models: 1 with a root
  transform, 2 with `BoneOffset`; 53 of 58 weapon-flag creatures hold a weapon (skeletons, dremora);
  576 of 1,323 interiors are dark.
- **Industry check.** Engines attach held props to named sockets with a relative transform (UE skeletal
  mesh sockets, Unity bone children/attach points, Godot `BoneAttachment3D`), with separate holster
  sockets per weapon type when sheathing is shown (Skyrim's `WeaponSword`/`WeaponBack` nodes, OpenMW's
  sheathing mod bones). Our `Mount` per readiness is that model; lights as child lights of the prop.


### As built (2026-10-12, `mw_ai`, Bevy-free)

A new crate rather than `mw_world` additions, so the app-facing API doesn't churn. Reuse: `petgraph`
0.8.3 (`DiGraph` + `algo::astar`) and `fastrand` 2.5 (seeded per actor), both already in the tree via
Bevy; nothing in Bevy itself covers waypoint graphs. Behavior follows OpenMW's `AiWander` and pathgrid
handling (GPL reference, fine here).

- **`pathgrid`**: `PathGraph::from_record(grid, &CellKey)`: points converted once to engine
  `WorldPos` (f64, `(x, z, -y) · UNIT_METERS`); exterior points are cell-local, so `grid · 8192` units are
  added first (OpenMW `CoordinateConverter`). Connections are directed as stored (point `i` owns the next
  `connection_count` entries); self-loops and duplicates are dropped; a count mismatch or a missing target
  is a `PathGridError` (record skipped). Edge weight = length in m. Queries: `nearest` (3D, the A* start),
  `within(center, r)` (3D), `path(a, b)` (A*, straight-line heuristic), `one_way_edges`.
  `PathGrids::build(db)` keys grids by `CellKey::{Exterior(x, y), Interior(lowercase name)}`: interior when
  an interior `CELL` has the grid's name, else its DATA grid. Empty grids are skipped (OpenMW #6209).
  One grid per cell, as in OpenMW: an exterior wanderer uses its current cell's grid only.
- **`wander`**: `Wander` from `AI_W` (distance → m, duration in game hours, Idle2–9 chances, `reset` →
  `repeat`; `game_hour` kept but inert, as in OpenMW). `roll_idle`: per idle, `r ∈ 0..100/mult`; it wins if
  `r < mult · chance` and beats the best `r` so far (`fIdleChanceMultiplier`, vanilla 0.75, via
  `gmst_float`). `WanderPlanner::next(graph, pos, rng)`: idle roll first (`Idle { variant 2..=9, seconds }`,
  seconds per variant from the caller, default 5 s, the wiring fills clip lengths); distance 0 → stand
  (`variant 0`, 2–5 s); else up to 4 random goals among the nodes within `distance` of home, A* from the
  node nearest the actor, dropping the first node when the actor is already past it; no grid or no node in
  range → one straight leg to a random point in the disc (`via_grid: false`; a scaffold: the original keeps
  those actors in place). Give-ups (`replan_around`, walkers3/5): a blocked pathgrid edge is remembered both
  ways and A* routes around it; a blocked *first* leg (actor → its nearest node) skips that node as a start
  while the actor stands within 1 m of where it failed (`blocked_starts`), starting at the next-nearest; an
  off-grid leg's heading is avoided (±45°) from about there when rolling the next point. `walk_speed(speed attr)` =
  `fMinWalkSpeed + speed% · (fMaxWalkSpeed − fMinWalkSpeed)` (100/200 units/s; encumbrance not yet).
- **`scaffold`**: **`PathWalkScaffold`** (named scaffold, CLAUDE.md rule 4). States Choosing → Walking
  (`sf_controller::PathFollower`, `PathMode::Once`) / Idling. `update(graph, pos, dt, game_hours)` →
  `ScaffoldOutput { intent: LocomotionIntent, idle_variant, walking, expired }`. Duration counts game
  hours (0 = forever; `repeat` restarts it; otherwise `expired`, the caller pops the package). A walk
  longer than 2× its nominal time + 5 s counts as stuck and replans. **Removal path**: delete
  `mw_ai::scaffold` and its wiring when `sf_ai` (M10) writes the intent; `pathgrid`/`wander` remain as
  adapter data.

Measured (vanilla + expansions, `tests/real_install.rs`): 1,386 pathgrids, 0 empty, 0 errors. Balmora:
46 grids (4 exterior), 1,089 points, 2,956 edges, 0 one-way; all exterior points inside their cell
square. 53 Balmora NPCs wander (distance > 0); 41 have grid nodes in range and 40 of those get an A*
grid walk on the first decision (the other found no usable goal in its 4 tries and stands); exterior NPCs stand a median 3.3 m
from their nearest node (13 NPCs), which confirms the exterior offset rule. Synthetic: idle roll at
chance 100 hits 74/133; goals cover exactly the allowed nodes; the `sf_controller` mover on flat ground
walks 10 scaffold paths in 90 s, at most 0.33 m off the polyline (corner cutting) and stopping within
0.15 m of each goal.

Left: `sf_controller_bevy` wiring (per-actor scaffold component, cell lookup via `CellKey::exterior_at`,
idle seconds from `mw_anim` clip lengths, game clock) and the visual check "Balmora NPCs walk pathgrid
edges".


## Static collision as built (2026-10-12, `mw_bake::collision`)

The Morrowind rules (`MwCollisionRules`, version 1) of the framework's `collision.cook`
(locomotion.md "Ground probes"), feeding `StaticProbe`. Registered by `mw_bake::model_baker`; keys from
`collision::collision_keys(models)`. Per model, first match wins (OpenMW `BulletNifLoader` semantics,
GPL reference read for behavior only):

| Case | Collision |
|---|---|
| skinned (actors, body parts) | none: actors are capsules |
| root string marker starting `NC` (any case: `NCO`, `NCC`; `mw_nif::no_collision_marker`) | none (`NCC` is camera-only in the original) |
| `RootCollisionNode` present | its subtree's triangles (hidden ones too), minus keyframe-moved nodes; empty → none (camera-only in the original) |
| editor marker model (`MRK`) without one | none (`Tri EditorMarker` nodes never render, so never reach the fallback) |
| animated parts (`mesh.cook` skips) | NIF render geometry in the rest pose, minus moving, billboard and particle subtrees |
| else | the `mesh.cook` LOD1 (framework default) |

The rest-pose row departs from locomotion.md's "skipped models get an empty mesh": candles, fires and
light fixtures (630 placements for one candle) would otherwise be walk-through. It is full-detail, but
only 150 models. Differences from OpenMW: the fallback is LOD1, not every triangle, and it leaves out
hidden (app-culled) geometry, as the render cook does. Surfaces are all `SurfaceId::DEFAULT` for now.

Measured (2026-10-12, vanilla + expansions, 16 threads, release; `collision_report` example): 6,609
models (10 missing files, as for `mesh.cook`), 759k triangles, 8.7 MiB.

| Source | Models | Placements | Triangles |
|---|---|---|---|
| authored `RootCollisionNode` | 3,168 | 160k | 372k (mean 118) |
| render LOD1 | 2,497 (4 empty) | 97k | 373k (mean 150) |
| rest pose | 150 (100 empty: moving item parts) | 1.8k | 14k |
| none: `NC` marker (kelp, grass, effects) | 374 | 77k | |
| none: skinned | 410 | 1.9k | |
| none: editor marker / empty collision node | 6 / 4 | 3.4k / 5 | |

Added bake time: cold 8.5 s wall (24.9 s summed over threads; it re-parses each NIF, about 4 ms),
`mesh.cook` already cached; warm 0.12 s. Against the 32 s full first bake that is about 40 s, well
inside the 2 min gate.

## Graph assets as built (2026-10-12, `mw_graphs`)

Crate `bevyrrowind/crates/mw_graphs` (Bevy-free; `sf_anim_graph` with `bake`, `mw_anim`, `ron`).
Templates are built in code (`mw_graphs::build`) and shipped as RON under
`crates/mw_graphs/data/animation/graphs/` (`mw_graphs::BUILTIN`, paths as `mw_actors::ActorConfig`
names them), so mods replace or patch them like any graph; a unit test keeps the files equal to
the builder (`MW_GRAPHS_BLESS=1 cargo test -p mw_graphs --lib` rewrites them).

- **Templates.** `mw_humanoid` (NPC skeletons and biped creatures), `mw_first_person` (the same
  without idle fidgets), `mw_creature` (every other creature; mover modes are parameters).
  `mw_graphs::creature_template(flags)`: CREA `Biped` → humanoid, as the original engine plays
  `xbase_anim` under biped creatures' own groups (`creature_animations` layers it; dremora, golden
  saints, liches, skeletons, Vivec have 0-5 groups of their own). `mw_actors` recipes pick the graph
  the same way.
- **Clip references** are group names resolved per skeleton by `AnimSet` (a `CompileSource`):
  `Group` (whole group, its loop range), `Group@section` (a sub-action: `WeaponOneHand@chop`),
  `Group@tail` (after the loop stop: the get-up of `Knockout`). Files layer by priority: female
  `xbase_anim_female.kf` over the male file; creatures `x<model>.kf`, then the model's own keys.
  Every clip but `Idle` is optional with a fallback chain: stance set → base set
  (`WalkForward1h` → `WalkForward`), `Run<Dir>` → `Walk<Dir>`, `SwimRun<Dir>` → `SwimWalk<Dir>` →
  `Walk<Dir>` (fish and fliers move with their walk cycle), `IdleSwim`/`IdleSneak` → `Idle`;
  variants (`Idle2-9`, `Hit1-5`, `Death1-5`) drop.
- **Base layer: `locomotion` machine.** Movement states `idle`, `walk`, `run`, `turn_left/right`,
  `sneak`, `sneak_idle`, `jump`, `swim_idle`, `swim_walk`, `swim_run`, `swim_turn_left/right`, each
  picked by exclusive conditions on `moving`, `running`, `sneaking`, `swimming`, `on_ground`,
  `turn` (unit test: exactly one matches every combination). Walk, run, sneak, swim are 2D blend
  spaces on `move` (x right, y forward) over the four directional groups, `Match` root motion.
  Reactions take priority and return to whichever movement the parameters ask for: `hit1-5`
  (trigger `hit` + `hit_variant`, re-hit restarts), `knockdown` (trigger), `knockout` (bool, loops
  the loop range) → `knockout_end` (`Knockout@tail`), `death1-5` (`dead` + `death_variant`, any
  variant missing falls to `death1`), swim variants; reactions are `Drive` (the original
  accumulates `Bip01` travel in them). Idle fidgets: `fidget` trigger + `idle_variant` 2-9 plays
  `Idle<n>` once; the number comes from the AI_W idle roll (`mw_ai`), so the graph stays
  deterministic. Jump (B2, 2026-10-26; OpenMW, Lyra JumpStart → Land): `jump` plays `Jump@head` once,
  holding its last pose while airborne; touchdown → `land` (`Jump@tail`, land key; inert 0.1 s) → movement
  when it finishes, `run` cuts it, airborne again re-jumps; a `Jump` without loop point plays whole (`land` drops).
  Fall (B3a; Lyra FallLoop, UE `IsJumping`/`IsFalling`): bool `jumping` picks `jump` vs `fall` on leaving the ground;
  `fall` holds `Jump@head`'s last frame (clip `start: 1.0`, not looping; `hold:` clip names, dropped without a loop
  point, then a fall enters `jump`), inert 0.2 s in, → `land` on touchdown; no jump ↔ fall in the air.
- **Weapon stance** (`stance` = `mw_graphs::Stance`): each movement state is a nested machine over
  none / hh / 1h / 2c / 2w / crossbow / spell. The first five play their own movement set; crossbow
  plays `1h` movement under `IdleCrossbow` and spell the base set under `IdleSpell` on the `upper`
  mask (chest subtree = the original's torso + arm groups); `SpellTurnLeft/Right` for the spell turn.
  Adapter rule: bow and thrown → `1h`, two-handed melee → `2c` / `2w` (OpenMW's
  `fallbackShortWeaponGroup`: real weapons fall back to the long-blade sets, crossbows on the lower
  body only).
- **Slots.** Humanoids: `upper` (masked, group `upper`) over locomotion, then `full` (group `full`);
  creatures: `full`. `slot_clips`: every weapon sub-action (`HandToHand`, `WeaponOneHand`,
  `WeaponTwoHand`, `WeaponTwoWide`, `BowAndArrow`, `Crossbow`, `ThrowWeapon`, `SpellCast`, `Shield`),
  `Torch`; creatures `Attack1-3`. Holding an attack at `Max Attack` is gameplay (M9).
- **PlayGroup / LoopGroup** (`mw_graphs::script`): `GroupRequest::{play, looped}` →
  `SlotRequest` per animation.md "Slots and montages" (flag 0 `Queue`, 1 and 2 `Interrupt`, 2 from
  the loop start, `loops` = count − 1), in slot `full` at priority 100 (OpenMW's "Scripted"
  priority), so gameplay montages can't interrupt; `bound_group` finds the graph's clip, else the
  host binds the cooked group late (`LateClip`).
- **Verified** (`cargo test -p mw_graphs`; real install, ~20 s): male / female / beast compile with
  no fallback (beast drops `Death4-5`), first person keeps 21 states (no swim or death groups);
  89 CREA models all compile (83 creature template, 6 humanoid), 4 stationary ones have no walk
  state (Yagrum, Akulakhan's heart, kwama queen, raven); headless simulation on the male skeleton
  drives every state (each stance's movement set or overlay, four directions, turns, sneak, jump
  with its `land` key, swim, `Idle2-9`, `Hit1-5` with Hit2's roar, knockdown, knockout,
  `Death1-5`), a one-hand chop in the `upper` slot while walking (charge window, min hit, hit; legs
  keep walking), `LoopGroup Idle3 2 1` ending after one duration plus one loop, and a gameplay
  montage refused over it; guar walks with footsteps and attacks, the cliff racer flies on its walk
  cycle, the slaughterfish swims on its walk cycle.
- **Industry check.** Same layering as UE5 Lyra (full-body locomotion machine; weapon-specific
  sets as linked anim layers per item; upper-body actions through a layered blend per bone, i.e.
  our masked slot; montages for one-shots) and OpenMW's controller (movement and idle states
  separate; missing weapon movement falls back to a base or long-blade set on the lower body with
  the weapon idle on the upper; idle chain swim/sneak idle → weapon idle → idle). Differ: hits are
  machine states, not montages, because Morrowind picks one of five by number and they share the
  reaction priority with knockdown and death; a montage-based hit react is an adapter swap.
  Sources (2026-10-12): https://dev.epicgames.com/documentation/en-us/unreal-engine/animation-in-lyra-sample-game-in-unreal-engine ;
  https://www.unrealengine.com/tech-blog/adapting-lyra-animation-to-your-ue5-game ; OpenMW
  `character.cpp` history (short-group fallback, separate movement/idle states):
  https://git.cijber.net/mirror/openmw/commit/6a32a4aed054e354404cbc8c321404841de36d2a ,
  https://git.cijber.net/mirror/openmw/commit/123ca175c8172e0cfaec09cc24658f480eab5d84 (commit
  messages read, current master not re-checked).
- **Left.** The viewer verify (parameter sliders on a dressed NPC: heavy agent);
  procedural ops beyond `SpeedMatch` (foot placement needs ground probes from the host);
  directional creature clips beyond forward (most creatures have only `WalkForward`: the blend
  space plays it for every direction, as the original does); the bake step (`animgraph.compile`
  with `GraphRules` mapping these references to `anim.cook` products) and mounting `BUILTIN` in
  the VFS.


