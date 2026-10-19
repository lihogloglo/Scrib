# Long robes and skirts over walking legs: the cheapest correct path

Research note p10r, 2026-10-19. Research only: no code changed, nothing built or run. Scope set by the owner
mid-task: full robe simulation is overkill for a small detail, so find the cheapest path that never shows a missing
leg. Context: PH-3 FAILED (owner 2026-10-18; `bevyrrowind/docs/visual-checks.md` PH-3, ROADMAP M4 P10).

## 1. What shipped games do

Most games ship long robes and skirts as an ordinary **skinned mesh**: the garment's vertices are weighted to the
pelvis, both thighs and both calves, with the weights blended across the gap between the legs (the "dress" weight
preset), and the body under the garment is hidden or removed. Simulation, if any, is an optional extra on top of
that skinned base, never the thing that keeps the legs covered.

- **The skinned base comes first, and the sim needs it.** Magica Cloth 2's backstop page: a skirt "skinned to the
  left and right legs" follows them, and "it is important that the Transform or Mesh follows the animation". Its
  sample skirt (UnityChan KAGURA) "is not skinned against the legs", so the legs pass through it, and Magica's fix
  is to skin the proxy to the leg bones ("Custom Skinning"). ([Magica Cloth 2: Backstop](https://magicasoft.jp/en/mc2_backstop_setup/))
- **When skirts do get spring bones, the legs drive them.** Kawaii Physics (UE, MIT) adds *SyncBone* to "sync
  animation-driven bones (e.g. legs) into the simulation so cloth follows the body", and *BoneConstraint* to keep
  hem bones off the legs. ([KawaiiPhysics README](https://github.com/pafuhana1213/KawaiiPhysics))
- **Tools ship a "dress" weight preset.** In Reallusion Character Creator, the fix for a clipping skirt is to transfer
  the skin weights again in T-pose using the "dress" preset, which blends the weights across both legs.
  ([Reallusion forum](https://forum.reallusion.com/PrintTopic547137.aspx))
- **The body under the garment is hidden by a mask, and loose garments are where masks fail.** MetaHuman outfits
  ship a "Body Hidden Face Map" ([Fab listing](https://www.fab.com/listings/445b4519-a04f-4517-a580-498ce208a782)).
  Second Life clothing uses alpha layers ([forum](https://community.secondlife.com/forums/topic/309008-short-skirt-rigging-issue)).
  Character Creator's automatic "Delete Hidden Faces" leaves "huge holes or missing parts" on loose clothing
  ([Reallusion discussions](https://discussions.reallusion.com/t/cc4-unreal-delete-hidden-faces/12712)). That is
  our PH-3 failure: hidden legs under a garment that no longer follows them.

Morrowind already uses this pattern. A robe is one Chest part skinned from shoulders to ankles. Its part rules hide
the groin, upper legs, knees and cuirass (`actors-adapter.md`), and the robe's authored skin moves with the thighs
and calves, so the hidden legs stay covered. V1 replaced that authored skin below the pelvis with a
pelvis-hung lattice, which is why legs went missing.

## 2. Does our kinematic mode (`H`) already give vanilla coverage?

**No. But the vanilla path is already in the code, one hook away.**

From the code (no runs):
- The `H` toggle (`apps/bevyrrowind/src/actors.rs` `toggle_cloth`) only sets `ClothSettings::simulate`. The robe
  stays cooked and **rebound to the lattice**. In `sf_anim_graph::secondary::ClothRun::update` with `simulate`
  off, the lattice joints sit on their *animated targets*. `ClothBinding::animated` computes each target as
  `ride.lerp(skin, follow)`, with `follow = CLOTH_LEG_FOLLOW (0.7) × row fraction`. So the hem row follows the legs
  at only 70 % and rides the pelvis at 30 %, and the rows above follow even less.
- Below lattice row 1, the vertices carry bilinear weights over a 10 × 5 lattice (`SkinRebind`), not the authored
  per-vertex thigh and calf weights. The split between the legs is smeared across about 10 columns.
- So kinematic mode looks closer to vanilla than simulated mode (the p10d `robe-run-kinematic.png` follows the
  legs), but it cannot guarantee coverage at a full stride.
- **The vanilla path:** in `sf_character_bevy::Shared::cloth_part`, if `CharacterContent::cloth_spec` returns
  `None`, nothing is cooked or rebound, no lattice joints are added and no op runs. The part is "drawn as authored",
  exactly as before P10. Two parts already take this path today because their cook fails (`werewolfskin`,
  `c_skirt_extravagant_1`). In bevyrrowind, `MwCharacters::cloth_spec` (`actors.rs`) forwards to
  `mw_physics::MwClothRules`, which marks every robe Chest part and skirt Skirt part.

## 3. Recommendation

**Robes and skirts default to their authored skin: `cloth_spec` returns `None` for them, so no simulation and no
rebind.** This matches vanilla Morrowind and most shipped games, and costs **0 µs per robe** (the cloth op ran at
20-26 µs per robe against a 30 µs gate). It also removes the ~9 ms per-part cook at load and the lattice joints
in the palette. Coverage is guaranteed the same way the original guarantees it: the authored weights follow the
hidden legs.

A cheap hem swing is **optional and parked**, not part of this fix (S2 below now uses a painted max distance on the hem vertices instead; the spring-bone idea is kept for reference). If the owner wants one later, use 4 spring
bones at the hem (front, back, left, right) on the existing `sf_anim_graph::secondary::SpringBones`, blended into
the hem vertices at a small weight *on top of* the authored skin, never in place of it. Leg follow stays at 1, so
coverage holds. Estimated cost is a few µs per robe.

**Keep the XPBD cloth for capes, banners and tents** (the "mesh cloth" item after M4). The solver (`sf_anim::cloth`:
tethers, max distance, swept capsules, character-space inertia, teleport handling), the `ClothRun` op,
`ClothRules`, `cloth_colliders` and the `H` A/B switch are all reusable there. A cape doesn't hide body parts, so
the coverage problem doesn't arise.

### Remove or park from V1

| Item | Action |
|---|---|
| `MwClothRules` marking robe Chest and skirt Skirt parts as cloth | **Change:** off by default; keep the rules behind an opt-in (a setting or `--cloth robes`) for A/B and mods |
| `CLOTH_LEG_FOLLOW` 0.7 and its tuning, the hem-width census fix (+13..+133 %) | **Park** (only matters for simulated robes) |
| `cloth.cook` bake step (removes the `ClothCookAtLoad` scaffold) | **Park** until a default path cooks cloth (capes) |
| PH-3 as written (simulated hem swing) | **Rewrite:** robes skinned as authored, no missing legs at walk and run |
| Solver, op, `cook_cloth`, `SkinRebind`, colliders, `H` key, `cloth:` log | **Keep** (capes and mesh cloth; open-lattice `ring: false` cook is still unbuilt) |
| Smooth bodies on cloth parts (p10d leftover) | **Drop** for robes (no rebind any more) |

Nothing needs deleting. The switch is one rule default plus docs.

### Slices

1. **Robes and skirts as authored** (bevyrrowind only). Make `MwClothRules` empty by default with an opt-in. Update
   ROADMAP P10 (robes: authored skin by owner decision; XPBD kept for mesh cloth), physics-adapter.md "Cloth",
   physics.md "Cloth" (one decision line plus an ARCHITECTURE.md §10 entry), and visual-checks PH-3.
   *Verify:* a unit test that `MwClothRules` returns `None` for `common_robe_01`'s Chest part by default and a spec
   with the opt-in. `character "fargoth+common_robe_01" "@dark elf,,f+common_robe_02" --walk`, keys 1/2: the log
   shows no `cloth:` line, and screenshots at walk and run show no missing leg. Then the owner re-checks PH-3 in
   `scene level-robe-walk` (cameras `priest`, `course`) and `scene ph3-balmora-temple`.
   *Done (p10s, 2026-10-19):* `mw_physics::ClothMode` (`Authored` default, `Robes` opt-in = `--cloth robes`).
2. **(Low priority, owner idea 2026-10-19; replaces the hem-spring idea) hem-only drape on the authored skin.**
   - *What is simulated:* only the bottom rows of hem vertices. Every vertex keeps its authored Morrowind skin as
     its target position; the hem vertices are particles pulled toward that skinned position (gravity, inertia, the
     leg capsules), the rest are drawn exactly as authored.
   - *Constraint:* each simulated vertex is tethered to its own skinned position by a **max distance painted per
     vertex**, 0 at the waist and on every unsimulated row, ramping to a few cm at the hem: the standard "painted max
     distance" of Chaos Cloth (UE) and NvCloth (and Magica Cloth's max distance). No lattice, no rebind: the
     authored weights stay.
   - *Coverage guarantee:* no vertex can stray more than its radius from its skinned position, and the skinned
     position already covers the hidden legs, so the legs stay covered at any stride; the worst case is S1.
   - *Cost target:* ≤ 5 µs per robe (`cloth:` log), only a few dozen hem vertices per robe.
   - *Verify:* the slice 1 shots unchanged at a dead stop; the hem settles a little behind on a stop or turn.

### Owner questions

1. Is the vanilla look (authored skin, no swing) enough for robes, or do you want slice 2's hem springs as well?
2. Should the simulated robe stay as an opt-in (`--cloth robes`, for mods), or be dropped from the game's settings?
