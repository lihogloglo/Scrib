# Groundcover adapter (Morrowind)

Status: **spec (G0, 2026-11-22)**, nothing built. Framework spec: `../../../superfargoth/docs/specs/groundcover.md`.
Research: `../../../superfargoth/docs/research/groundcover-2026-11-22.md`.

## What Morrowind gives us

- **No species records.** Morrowind has nothing like Skyrim's GRAS (land texture → grass list), so the rule table is
  ours, shipped as data (`data/groundcover.ron`, loaded as a mod record set so mods can override it).
- **Vanilla plant models** with alpha-card textures (BSA scan 2026-11-22):

  | Model | Texture | Source |
  |---|---|---|
  | `f/flora_grass_01`-`04` | `tx_grass_blade_01` | Morrowind.bsa |
  | `f/flora_grass_05`-`07` | `tx_tall_grass_01` | Tribunal.bsa |
  | `f/flora_bc_grass_01`, `_02` | `tx_bc_grassblade` | Morrowind.bsa |
  | `f/flora_bc_fern_03`, `_04` | `tx_bc_fern_01` | Morrowind.bsa |
  | `f/flora_bm_grass_03`, `_06` | `tx_bm_shrub_grass_03` | Bloodmoon.bsa |

- **The owner's texture packs upgrade them.** MVR PBR v2 ships those textures at 256-2048 px with `_n`/`_nh` and
  `_spec` companions (`tx_grass_blade_01` 256×2048 DXT5, `tx_tall_grass_01` 2048×1024, `tx_bc_grassblade` 512×2048,
  ferns 1024×2048); Tamrielic PBR West Gash ships a grass atlas (`grass/wg_az_atlas.dds`, 4096×2048 BC7, + `_spec`)
  that no vanilla model uses yet. They already go through the texture cook (`textures-adapter.md` §7), including the
  truncated foliage mip chains (T2 `preserve_coverage`). Packs are data profiles, never renderer paths.
- **Land textures** (`LTEX`) are the rule-table input: the terrain already exposes their weights
  (`MwLayers`, splat clipmaps). The grass land textures (`tx_gl_grass_*`, `tx_ai_grass_01`, `tx_bc_grass`,
  `tx_mh_grass_01`, `tx_bm_grass_*`, `tx_grass_01`, `tx_ash_grass_*`, and the `*_dirt_grass_*` mixes) get species;
  roads, rock, sand and water edges get none.
- **OpenMW groundcover mods** (`groundcover=` plugins: Vurt's, Remiros', Aesthesia): ordinary static references.
  Read through `mw_records` as explicit placements for the mesh path (GC-5), keeping their models and textures.

## First species set (GC-2)

Mesh species from the vanilla models above, rule table for the Ascadian Isles and Bitter Coast land textures only:
`tx_ai_grass_01` and `tx_gl_grass_*` → `flora_grass_01`-`04` (+ `05`-`07` sparse); `tx_bc_grass` → `flora_bc_grass_*`
+ `flora_bc_fern_*`. Density, ranges and slope limits are `Guess` tunables, set by the owner's look loop on the test
scene, not by copying another project's numbers. Test scene: Ascadian Isles road near Pelagiad, 08:00 and 14:00;
Bitter Coast south of Seyda Neen, 10:00.

## Units

Morrowind units convert through the existing world scale (`sf_coords`); the framework sees metres only. OpenMW's
groundcover default distance (6144 units, ~88 m) is a reference point, not our value.
