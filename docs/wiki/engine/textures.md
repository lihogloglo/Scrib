# Textures: cook, de-light, normal maps

## What it does

At bake time every texture a referenced model uses becomes block-compressed KTX2 for its role (BC7 colour, BC5 normals,
BC4 masks), albedo painted with baked-in light is de-lit, and normal-map companions from texture packs are converted to
the canonical layout. At runtime a VFS layer serves the KTX2 files above every mod while they are still the winners.
At load, `.ktx2` files start at the first mip level within the quality preset's size cap (S1); an image replaced under its handle re-binds the materials using it (S2,
`sf_texture_bevy/src/rebind.rs`, line `texture.rebind:`; detail in the spec §9); no residency streamer yet: the planner is `sf_texture/src/residency.rs` (S3a), demand data is baked per cell (`texture.demand`, UV density `mesh.uv_density`; S3b, `superfargoth/crates/sf_scene/src/demand.rs`), the streamer is S4.

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
Runtime material: `mw_assets/src/cooked.rs:296` (`cooked_material_with`). `_nh` height → BC4 `depth_map` (parallax per
preset, `cook_depth` / `texture.cook.depth`, `map_ab::Parallax`; T4d, textures.md §9).

**Per-source profiles** (TS-1a, 2026-11-13): `mw_bake::texture_profiles::TextureSources` gives each VFS source its pack
profile from its `mod.toml` `[texture]` (`profile`, `normal_y`, `suffixes` renames) or, for sources without one (loose
Data Files, BSAs), `<user data>/textures.toml` (`[<mod id>]` tables; `mw_mods` `load_textures`); none = `openmw`. A
companion is read with the profile of the source its winning file comes from. Bake logs `texture profile: <src> -> <p>`.

**Source formats** (TS-1b, 2026-11-13; table in textures-adapter.md §9): `decode_rgba` (`mw_bake/src/textures.rs:62`),
used by the cook's decode path, de-light, terrain and far bakes, picks the container by its bytes; block files go through
the rewrap's reader (`:80`, so BC4/BC5/BC7 and KTX2 decode too), legacy uncompressed DDS by its bit masks plus P8
(`:153`), DX10 and KTX2 uncompressed by table (`:180`, `:201`), TGA/BMP/PNG through `image`. Not read: HDR, signed,
Basis, cubemaps, JPEG. Matrix test over BSA, loose and mod sources: `mw_bake/tests/texture_sources.rs`.

**ORM / `_spec` maps** (TS-2, 2026-11-13; textures-adapter.md §10): `plan_orms` (`cook.rs:358`) finds a base texture's
`_spec` and reads it with its own source's profile: `wareya-pbr` = packed ORM, rewrapped through the block-exact R↔B swap
(BC1 stays BC1, DXT5 → BC1, BC7 swaps fields) by `cook_orm` (`:496`); `openmw` = OpenMW's specular map, converted by
`cook_specular` (`:605`: roughness `(2/(n+2))^¼` from the Blinn-Phong exponent `n = 255·A`, `sf_texture::specular_to_orm`,
no occlusion, dielectric, colour dropped) to linear BC1. The index publishes base → companion, served as
`cooked/texture_orms.postcard` (`read_orms` `:1139`). The material builder (`mw_assets/src/lib.rs:483`) binds it as
`metallic_roughness_texture` (and `occlusion_texture` for a packed ORM), `metallic` and `perceptual_roughness` 1.0,
`reflectance` 0.5; materials without one keep 0 / 0.85 / 0.3. Pack-defect repair (owner question 7): profile quirk
`FillEmptyLevel0(role: Orm, channel: B)` in `data/texture_profiles.ron`, **off**; rebuilds an empty level-0 metal channel
from level 1 (`sf_texture::fill_empty_level0`, BC1 only), counted `level 0 rebuilt` in the `texture.cook:` line.

**Runtime layer**: `CookedTextureLayer` (`cook.rs:781`, mounted by `mw_boot_bevy/src/delight.rs`) serves `<stem>.ktx2`
for an index entry only while the source is still the winner with the fingerprint the bake saw and the only variant of
its stem (`sole_variant`); a newer mod file wins and the next bake cooks it.

## Bevy features used

`.ktx2`: our `CappedKtx2Loader` (S1, `superfargoth/crates/sf_texture_bevy/src/lib.rs:146`), registered after Bevy's `ImageLoader`
(last registered wins per extension, `bevy_asset` `server/loaders.rs` `find`; one expected startup `warn!` `Duplicate
AssetLoader preregistered ... ["ktx2"]`: Bevy 0.20 can't replace a loader, and `tonemapping_luts` keeps `bevy_image/ktx2` on).
It reads the header + level index, plans the cap (`sf_texture::level_cap` `plan` `:134`, `first_level` `:201`: first level
with larger side ≤ cap, backed off to a whole 4x4 block for BCn; BasisLZ/3D/unknown formats served whole), reads only the
file prefix the kept levels need (levels are stored smallest first), rewrites header/index (`trim` `:223`, level bytes
unchanged) and hands it to Bevy's `ImageLoader` (same `ImageLoaderSettings`: sRGB from the settings, sampler). Cap:
`Quality::texture_cap` (`mw_game_core/src/exterior.rs:75`; Low 512, Medium 1024, High 2048, Ultra none), `--texture-cap <px>`
(`0` = none) overrides (app `main.rs:601`), plugin added in `mw_exterior_bevy/src/exterior.rs:308` (game runs only; viewers
uncapped). DDS/TGA still load through Bevy's loaders, uncapped. See [content pipeline](content-pipeline.md).

## Switches and counters

Load-time cap (S1): `texture.cap: <cap>, N ktx2 loaded, M capped, U uncappable, X MiB loaded, Y MiB skipped` (GPU bytes;
logged once each loading burst has been still 2 s; resource `sf_texture_bevy::TextureCapStats`). `--off delight` (`mw_game_core/src/perf.rs:175`; also drops the cooked layer's de-lit files); bake lines `texture.cook:` and
the de-light report with per-`Skip` counts. At startup `texture.cook: layer mounted, ... N normal companions, M ORM companions`
(`mw_boot_bevy/src/delight.rs:69`). Live (TX-DIAG, `sf_diag_bevy/src/textures.rs`, every interactive run via
`mw_devtools_bevy::pipeline_watch::add`): `textures.live:` (images, GPU MiB per BC family from the texture descriptors,
normal maps in an sRGB format) and `materials.live:` (StandardMaterials with normal / metallic_roughness / occlusion / depth
maps; shown normal-mapped entities whose mesh lacks tangents or UV0, which Bevy silently ignores; `unread` = RENDER_WORLD
mesh not prepared yet), logged at each reveal (+0.5 s) and on a >5 % change (at most every 10 s); gauges `textures/gpu_mib`,
`textures/images`, `materials/normal_no_tangent`, `materials/normal_no_uv0` when `DiagCounters` record.
`--off normalmaps` / `--off orm` and console `flip normalmaps` / `flip orm` (F1 HUD line "Textures"; TX-lab,
`mw_assets/src/map_ab.rs`): every StandardMaterial loses its normal map / ORM slots (2002 factors 0 / 0.85 / 0.3) and
gets them back field for field; `--off parallax` / `flip parallax` the same for depth maps. Asset viewer: `scene level-texture-lab` (test-levels.md "Texture
lab"), per-swatch info panel and `texture-lab:` log lines.

## Known limits

Actors and `scene.chains` drop tangents, so no normal maps there; actors (`mw_actors_bevy` `cooked_material`) bind no
ORM either yet (no companions passed). A pack shipping wareya-style `_spec` must be assigned `wareya-pbr` (§8), else its
`_spec` reads as OpenMW's specular map. Terrain layer textures are not de-lit; terrain layers get normal + ARM
from the same companions (T5, `mw_bake/src/world/layer_maps.rs`) but stay RGBA8 256² (BC7/BC5 and size per preset: T6;
the `_nh` height isn't moved into the albedo's alpha). Tangent handedness (fixed 2026-11-13, tx-tangent): cooked
tangents store Bevy's sign, the negated MikkTSpace one, like `Mesh::generate_tangents` (`bevy_mesh` `mikktspace.rs:126`;
`sf_model/src/tangent.rs` `bevy_handed`), so +Y-up normals read with green up and cooked meshes agree with the loader's
fallback; mirrored rigid parts flip `w` (`skin.rs` `rigid`). Bakes before `KEY_FORMAT` 4 had green inverted: re-bake.
Normal-map look (TX-1) awaits the owner. Sizes that aren't whole 4x4
blocks are served as shipped (`CookSkip::Unaligned`; a raw unaligned BC DDS can't load), and a stem in two extensions
across sources (BSA `.dds`, mod `.tga`) is `Ambiguous`, served as shipped (textures-adapter.md §9).

## Specs

`superfargoth/docs/specs/textures.md` (sections 5 Part A, 6 Part B residency, 7 Part C normal maps, 9 Slices), `bake.md`
(De-lighting); `bevyrrowind/docs/specs/textures-adapter.md` (Roles, Companions, De-light rule, PBR packs).
