# Visual checks: lights

Open visual checks: lighting, night exposure, lamp light (LT-*, LM-*). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

## Lighting: night exposure floor, point-light shadows (superfargoth `sky-lighting.md` "Exposure range", "Local-light shadows"; `.sessions/lighting.md`)

Shots in `shots/lighting/` (`before_night.png` / `night_old.png`: the old night; `night2.png`: floor 3.5 + night ambient 1;
`cave_in1*.png`; `torch_stage.png`).

### LT-2: Dark interior keeps its lights (`scene lt-cave`)
- Kind: game location. Run: `cargo dev scene lt-cave` (Addamasartus, mine tunnel).
- Look for: rock texture readable in the torch pools, the far tunnel dark but not black; the wooden beam and post
  shadow the rock (point-light shadows). A/B `--portals`: auto exposure meters this cave at the same level with or
  without the floor (luma 50.8 vs 50.4): the floor never darkens it.
- Status: partial, owner 2026-10-19 (round 9, scene lt-cave): lighting OK (rock readable, far tunnel dark not black); beam/post shadows only when very close, and very grainy (SH-13).

### LT-3: Point-light shadows, torch-bearers (`scene lt-torch-shadows`; stage: `character "imperial guard" --night`)
- Kind: game location + game models.
- Look for: railings, crates and pillars near lamps throw shadows on the street and walls; a torch-bearer within
  ~15 m throws a shadow on the ground (and on a wall beside them) that moves with the torch; walkers passing ~20 m
  lose their shadow by fading, not popping; lantern cages do not stripe the walls (0.25 m shadow near plane). On the
  stage (`.\bevyrrowind character "imperial guard" --night`): a long shadow of the guard on the floor away from the torch.
- A/B: `--point-shadows 0` / `2` / `4`; default by `--quality` (Low 0, Medium 0, High 1, Ultra 4). Cost (1440p, RTX 4060 laptop,
  median frame, N = 0/1/2/4/8): 9.4 / 10.2 / 13.3 / 13.8 / 13.6 ms; render-thread CPU for the casters in each
  light's range is most of it (GPU +0.35 to +1.6 ms). With the High default only the nearest light (a torch
  within 2x the nearest lamp's distance) shadows; `--point-shadows 4` shows the lamps' railings and crates too.
- Status: rejected by owner 2026-10-19 (round 5, Balmora lamps at 22:00): see SH-13.

## Haze and dust (superfargoth `weather/distant-shadows.md` "Aerial perspective and haze" N1/N2; ROADMAP M6 F1; `.sessions/m6-f1.md`)

The `--clouds` weather now also sets the atmosphere's aerosol haze and an absorbing dust/ash term (table in
`weather-adapter.md` "Haze and dust"); `--haze <x>` and `--dust <x>` fix them for A/B. The C key's genera use the
fair-weather look. Near fog, ground fog banks and god rays are F2/F3, not this check.

### LM-2: Leaf side findings (from LM-1, renderfix1)
- With `--off leafmask`, blended leaves vanish entirely at ~100 m+, so the A/B no longer shows "blended as before".
- Leaves (and water) look bright white at night (`shots/renderfix1/night_amb4.png`, Balmora): not traced; hypothesis
  is still-Blend leaves on the forward FroxelFog path.
- 4 textures could not be decoded, so they are never promoted to cutouts: `poxyclouds2`, `tx_banner_temple_04`,
  `tx_eggmine_banner*`, `tx_rope_net_01`.
- Fix (render-opt 2026-10-28): propagated NetImmerse `NiVertexColorProperty::LightingMode` (`Emissive` vs `EmissiveAmbientDiffuse`) and `NiTexturingProperty::ClampMode` through `mw_nif`, `sf_model`, `mw_bake`, and `mw_assets`, and added 16-bit `A1R5G5B5`/`A4R4G4B4`/`B5G6R5` DDS decoding in `mw_bake/src/textures.rs` for cutout promotion. Re-bake `mesh.cook` / `tex.meta` if using an older bake store; check `view Balmora --hour 22`.
- Status: awaiting owner (2026-10-28).
