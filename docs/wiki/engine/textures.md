# Textures: cook, de-light, normal maps

## What it does

At bake time every texture a referenced model uses becomes block-compressed KTX2 for its role (BC7 colour, BC5 normals,
BC4 masks), albedo painted with baked-in light is de-lit, and normal-map companions from texture packs are converted to
the canonical layout. At runtime a VFS layer serves the KTX2 files above every mod while they are still the winners.
No runtime texture streamer exists yet (residency is specced only).

## How it works

**Framework** (`superfargoth/crates/sf_texture`, Bevy-free, `lib.rs:1-30`): `cook` (role to format: `format_for`,
`cook_levels`, BC4/BC5 encoders, gamma-correct mip chains), `ktx2_out` (`BlockFormat`, `write_ktx2`, BC7 encode), `rewrap`
(shipped DDS or KTX2 blocks into KTX2 unchanged, missing mips filled), `remap` (block-exact Y flip, R/B swap, alpha drop),
`profile` (a pack's conventions as data), `delight` (`shading_grid` `delight.rs:251`, `delight` `:372`: divides the
low-frequency painted light out).

**Morrowind steps** (`bevyrrowind/crates/mw_bake`). Order in `bevyrrowind bake`: de-light, then cook (it skips de-lit files).
- `texture.roles` per model, `texture.delight` per winning file, `texture.delight.index`, `texture.alpha_class` + `.index`
  (cutout or translucent; read by the static-scene bake) (`delight.rs:53-57,480-482`; `bake_delight` `:647`). Strength
  `DEFAULT_STRENGTH` 0.5 (`:61`), `MIN_SIDE` 64 (`:63`). Skipped: non-albedo uses (glow, detail, decal, effects,
  translucent), sky and UI, already-KTX2, several variants of a stem, a same-source normal companion (a PBR pack shipping
  both stays as authored), non 4-aligned sizes.
- `texture.cook` per winning file by role, `.meta`, `.index` (`cook.rs:49-53`; `cook_texture` `:392`, `bake_cook` `:736`).
  Per source: indexed in place (suitable BC with full chain), rewrapped (missing levels or remap), or decoded and encoded
  (BC7 sRGB colour). Alpha kept for blend/glow/detail slots; opaque albedo drops it; cutout kept only if the alpha has holes.
- Pack conventions: `texture_profiles.rs` reads `data/texture_profiles.ron` into `PackProfile`s, default `openmw`
  (`:12`). The framework names no pack.

**Normal maps**: `normal_companion` (`cook.rs:303`) finds `_n`, `_nh`, `_nm` companions, cooked to linear BC5 with the
profile's Y flip (BC1 normals stay BC1). `tangent_rule` (`cook.rs:323`) gives `mesh.cook`, `mesh.rest` and `mesh.rig`
MikkTSpace tangents when the base texture has a companion, depending on file existence via `RunCx::file_exists`.
Runtime material: `mw_assets/src/cooked.rs:296` (`cooked_material_with`).

**Runtime layer**: `CookedTextureLayer` (`cook.rs:781`, mounted by `mw_boot_bevy/src/delight.rs`) serves `<stem>.ktx2`
for an index entry only while the source is still the winner with the fingerprint the bake saw and the only variant of
its stem (`sole_variant`); a newer mod file wins and the next bake cooks it.

## Bevy features used

Bevy's KTX2 image loader reads the products; no custom loader yet. See [content pipeline](content-pipeline.md).

## Switches and counters

`--off delight` (`mw_game_core/src/perf.rs:175`; also drops the cooked layer's de-lit files); bake lines `texture.cook:` and
the de-light report with per-`Skip` counts.

## Known limits

Actors and `scene.chains` drop tangents, so no normal maps there. Terrain layer textures are not de-lit; terrain normals
(T5) and BC7/BC5 layer arrays (T6) are not built. Normal-map look (TX-1) awaits the owner.

## Specs

`superfargoth/docs/specs/textures.md` (sections 5 Part A, 6 Part B residency, 7 Part C normal maps, 9 Slices), `bake.md`
(De-lighting); `bevyrrowind/docs/specs/textures-adapter.md` (Roles, Companions, De-light rule, PBR packs).
