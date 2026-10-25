# Actors adapter (Morrowind → sf_anim / sf_character)

Status: draft (2026-10-11), notes from the M3 research session; verify each against the data
before implementing. KF import and text-key events built 2026-10-12 ("KF import as built"); rig maps,
masks, creature hints and movers built 2026-10-12 ("Rig maps as built"); static collision rules built
2026-10-12 ("Static collision as built"); `CharacterSource` for NPC_/CREA built 2026-10-12
("`CharacterSource` as built"); graph assets built 2026-10-12 ("Graph assets as built"); held items (Bevy-free) built 2026-10-12 ("Held items as built").
Crates: mw_nif, `mw_anim` (KF/NIF animation → `sf_anim` source clips), `mw_actors` (records →
character recipes), on top of superfargoth `animation.md`, `character.md`, `locomotion.md`.

## Goal

Map Morrowind skeletons, KF animations, text keys and body-part records onto the framework's
humanoid rig, cooked clips, events and character recipes, so vanilla NPCs and creatures animate
with the modern stack. Creatures keep their own skeletons (no retarget), with a creature graph
template.

## Skeletons and clips

- Skeletons: `base_anim.nif` (male), `base_anim_female.nif`, `base_animkna.nif` (beast races:
  digitigrade legs, tail), `base_anim.1st.nif` (first person). Clips come from the matching
  `x*.kf` files. Creatures each have their own skeleton and x-KF.
- KF layout: one long timeline per KF, per-bone keyframe controllers named through
  `NiStringExtraData`. Text keys split it into groups (`Group: Start`/`Stop`, `Loop Start`/
  `Loop Stop`); each group becomes one cooked clip with a loop range. `mw_nif` already collects
  root text keys and normalizes Hermite/TCB/Euler curves; `anim.cook` resamples those.
- Root motion: 3ds Max Biped convention. Check on data whether forward travel is on `Bip01` or
  `Bip01 NonAccum` (the original engine accumulates one, leaves the other). Locomotion uses
  `Match` mode: playback rate = actor speed / clip speed, as the original engine is known to do.
- Masks: four bone groups (lower body, torso, left arm, right arm), so the weapon/spell layer sits
  over locomotion.

## Bip01 → humanoid (verify names in `base_anim`)

| Bip01 | Humanoid |
|---|---|
| Pelvis | hips |
| Spine / Spine1 / Spine2 | spine / chest / upperChest |
| Neck / Head | neck / head |
| L/R Clavicle | shoulders |
| UpperArm / Forearm / Hand | upper arm / lower arm / hand |
| Finger0..4 (+1/+2) | thumb..little |
| Thigh / Calf / Foot / Toe0 | upper leg / lower leg / foot / toes |

Tail, weapon and shield bones go to `extra_chains` and sockets. Check the beast leg chain for an
extra (HorseLink-style) joint. Bip01's rest pose is not a T-pose: use an auto-aligned retarget
pose, checked visually. Source data is Z-up; convert in the importer as for static NIFs.
(2026-10-12: no extra beast leg joint; actors face −Z in engine space and are turned to +Z at
import; see "Rig maps as built".)

## Text keys → events

- `SoundGen: Left/Right/Land/...` → footstep and creature-sound events; `Sound: <id>`.
- `<Chop|Slash|Thrust> Min Attack/Max Attack/Hit/...` → attack windows (they feed M9).
- `Equip Attach`, `Unequip Detach`, `Shoot Attach/Release`, `Self/Touch/Target Start/Release/Stop`.
- Unknown keys stay as generic events.

Moved to [history](archive/actors-adapter-history.md#kf-import-as-built-2026-10-12-mw_anim).

## Body assembly

- NPC_ gives race, gender, head and hair BODY ids; skin parts come from BODY records with matching
  race, gender flag and part index.
- ARMO/CLOT carry part lists (slot index + male/female BODY id) over the 27 part slots (head, hair,
  neck, cuirass, groin, skirt, hands, wrists, forearms, upper arms, feet, ankles, knees, upper legs,
  pauldrons, shield, weapon, tail, ...). Female parts fall back to male. Equipment over clothing
  over skin; robes and skirts hide leg slots.
- Some parts are rigid shapes attached by bone name: they become single-joint skins at cook. Some
  part NIFs may hold both sides or rely on mirroring; inspect with `bevyrrowind nif` first.
- Beast races: no helmets or boots in vanilla; tail slot.
- Scale: race height/weight per gender → root scale (height on the up axis, weight horizontal).
- Heads: 273 files carry `NiGeomMorpherController` → morph targets; talk driven by voice loudness,
  blink by a timer.
- First person: 1st-person skeleton and KF plus 1st-person hand parts (verify naming, e.g. `*.1st.nif`).
- OpenMW behavior may be reused here (GPL repo).

Moved to [history](archive/actors-adapter-history.md#charactersource-as-built-2026-10-12-mw_actors).

## NPC life in M3

`PathWalkScaffold` over PGRD pathgrids; AI_W gives wander distance and Idle2–9 chances.

Moved to [history](archive/actors-adapter-history.md#as-built-2026-10-12-mw_ai-bevy-free).

## Static collision as built (2026-10-12, `mw_bake::collision`)
Moved to [history](archive/actors-adapter-history.md#static-collision-as-built-2026-10-12-mw_bakecollision).

## Verification

See the ROADMAP M3 adapter tasks (each has its own Verify step).

## Industry comparison

J9 (superfargoth `industry-audit.md`), 2026-10-12, docs only (`.sessions/j9rest.md`).
Compared inline: "Graph assets as built" → "Industry check" (UE5 Lyra
layering, OpenMW's controller states and fallbacks: **match**, hits as
machine states **differ** on purpose). The animation, character and rig
techniques are compared in superfargoth `animation.md`, `character.md` and
`rig-profile.md`. No gap at the adapter level.

## Open questions

- ~~Which bone carries root motion?~~ `Bip01` (2026-10-12: `WalkForward` moves it 2.9 m
  forward, `RunForward1h` 3.8 m; `Bip01 NonAccum` doesn't travel).
- ~~Mirrored body parts: confirm per file.~~ Rigid parts on `Left` bones are mirrored by the
  adapter (`PartKind::Rigid::offset`); skinned files hold both sides, split by `node_filter`
  (2026-10-12, "`CharacterSource` as built"). To confirm visually once parts render.
- Actor spawning: add the import half turn (`mw_anim::rig::FACING_TURN`) to each reference's yaw,
  so turned skeletons keep the vanilla world orientation (actor spawning doesn't exist yet).
- Static collision: should pickable items (weapons, armor `_gnd` models) block actors? They do
  now (render LOD1); OpenMW's behavior to check. Footstep surfaces per triangle (texture → surface
  table) are still all default. `bevyrrowind bake` doesn't request `collision.cook` yet (one line:
  add `collision::collision_keys(&paths)` to its keys; the app was busy, 2026-10-12).
- Should `rig::creature_hints` move to data (`data/animation/rigs/<model>.rigprofile.ron`, as
  rig-profile.md plans for overrides) once more creatures need hints? Three need them so far.
