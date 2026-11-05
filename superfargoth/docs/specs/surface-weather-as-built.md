# Surface weather: as built

Detail moved out of [surface-weather.md](surface-weather.md) (spec budget). The spec keeps the design; this file
keeps the as-built shapes and slice notes. Code is the source of truth.

## W1 (2026-11-05): CPU state (`sf_surface`)

```rust
pub struct SurfaceRates { wet, puddle, snow, ash, drying: f32 }   // exists in sf_weather (per game hour)
pub struct SurfaceTuning {  // record-backed; start values from Clockwork, re-calibrated by the owner's look
    step_h: f64, dry_h: f32,  // 1/6 h (10 game min) fixed step; 2.0 h full wet -> dry (Clockwork 120 min)
    puddle_dry_mult: f32, snow_melt_mult: f32, ash_clear_mult: f32, // 0.5, 1/3 (snow 3x slower), 1/4 (dust 4x)
    puddle_from_wet: f32,     // 0.7 (owner 2026-11-06, was 0.9): puddles rise only on soaked ground
}
pub struct ClimateSurface { wet: f32, puddle: f32 }               // per ClimateIx
pub struct CellSurface { snow: f32, ash: f32 }                    // per 32 m cell, sparse
pub struct SurfaceState {
    t_h: f64,                                         // game hours of the last step
    climates: Vec<ClimateSurface>,                    // indexed by ClimateIx
    cells: HashMap<(WorldspaceId, IVec2), CellSurface>, // only non-zero cells
}
pub fn advance(state: &mut SurfaceState, to_h: f64, rates: &dyn RateSource, tuning: &SurfaceTuning);
pub fn sample(state: &SurfaceState, raster: &ClimateRaster, ws: &WorldspaceId, p: DVec2) -> SurfaceSample; // CPU query
```

- **Step** (pure, fixed `step_h` on a grid from hour 0, sub-stepped for rest/wait; catch-up capped at 72 h, a new
  state warms up over the same 72 h; values under half a save quantum snap to 0): per climate,
  `wet += (rates.wet − d)·Δ` with `d = rates.drying` (blended presets; `1/dry_h` where no preset sets it,
  0 while `rates.wet > 0`), clamped to [0, 1]; `puddle` rises at `rates.puddle` only while
  `wet ≥ puddle_from_wet`, dries at `puddle_dry_mult·d`. Per cell, `snow`/`ash` coverage add
  `rates.snow/ash × altitude factor` and decay at `snow_melt_mult·d` / `ash_clear_mult·d` while nothing falls. Cell rates come from `ClimateRaster::site_at(cell centre)` weights × `ClimateRow`s, the same
  border blend the WE4 weather map uses (map.rs), so cover agrees with the far weather.
- **RateSource** is `WeatherState::blend` per climate at the step's mid time: weather is a pure function of
  (seed, overrides, clock), so any peer computes the same rates.
- **Altitude factor** (snow only): `smoothstep(snow_line − 50 m, snow_line + 50 m, cell height)`, `snow_line`
  per climate (default −∞: snow everywhere it snows); as built a `CellGrid::set_altitude` hook (snow line, cell height), no
  record field until a game needs one (Morrowind: off).
- **Cell membership:** a cell becomes active when its rate is positive; it is dropped when both channels
  reach 0. Vvardenfell at 32 m is ~30k cells; only the snowy/ashy part is ever stored.
- Rates = the presets' `surface` × PRECIP blend weight (`WeatherRates`); cells over the map's raster
  (`WorldWeatherMap::with_worldspace`). `sf_weather_bevy` steps it on game time; console `surface set/dump/clear`.

## W2 (2026-11-05): GPU resources (`sf_surface_bevy`) and the terrain hook

- **Bevy-free logic** in `sf_surface::view`: `CellWindow` (toroidal 128² window, `recenter` returns the entering
  column/row strips, disjoint; a jump ≥ the window rewrites all; `texel_rects` splits at the wrap), `cell_texel` /
  `fill_texels` (R wet, G puddle from each cell's `CellGrid` site, else the camera site; B snow, A ash; per-channel
  maxima → layer flags), `shelter_center` (snapped to 0.125 m), `shelter_view` (orthographic, −Y, −Z up, reverse Z,
  200 m above to 150 m below the centre), `ShelterTrigger` (16 m / 8 m / streamed every 30 frames), `noise_atlas`
  (256² RGBA8 tileable value noise R/G, streaks B/A), `DebugView`.
- **Plugin** `SurfaceWeatherPlugin` (`PostUpdate`, after transforms and the terrain): `SurfaceGpu` (uniform
  `ShaderBuffer` of 9 `vec4<u32>` slots, cells `Rgba8Unorm` 128² linear/repeat, shelter `Depth32Float` 1024²,
  noise), `SurfaceUniform` (CPU mirror, uploaded only when it changes), `SurfaceStats`, `CellWrites` (strips written
  through `RenderQueue::write_texture`, as the terrain clipmaps). The window's render-space corner comes from
  `TerrainOrigin` (f64), so the shader works in render space and needs no absolute world position.
- **Change cue:** `WorldSurface::generation()` (`sf_weather_bevy`), bumped by a step, a new grid, `surface
  set/clear` and `restore`; code writing `state` directly calls `touch()`. A change rewrites the window (worst case
  0.73 ms CPU for 128² cells with every cell covered, release, `measure_full_window_fill`); a recenter writes one
  128-texel strip.
- **Shelter:** subview `0xFC` of `sf_sky_bevy::sun_view` (precipitation is `0xFB`); casters are visible shadow
  casters without `SkinnedMesh` or `NoSurfaceShelter` (terrain patches get it: a heightfield shelters nothing from
  straight above). Drawn only while a layer flag is set or a debug view shows; redrawn on an origin shift. No
  shadow-casting directional light (night, interiors): no redraw, the last map stays.
- **View:** `SurfaceWeatherView` follows `TerrainCamera` (W2) and is mirrored onto the render entity for W3.
- **Terrain hook (scaffold, removed by W3):** `TerrainMaterial` bindings 113-118 (uniform storage, cells + sampler,
  shelter as unfilterable float, noise + sampler) and `TerrainMaterialKey { surface }` → shader def
  `SURFACE_WEATHER`; `sf_surface_bevy` hooks every terrain material. `terrain.wesl` calls `apply_surface_layer`
  under `@if(SURFACE_WEATHER)`; the old no-op `surface_state.wesl` is gone.
- **WESL** `sf_surface_bevy::layer` (`src/layer.wesl`): binding-agnostic (uniform and textures passed as
  arguments, like `translucent_fog`), `surface_exposure` (3×3 tent of loads, lookup 0.35 m out along the normal,
  bias 5 cm + 1.5 texels × slope, fade to open sky over the outer 1/8), `surface_cells_at` (bilinear, repeat),
  `surface_cell_load`. W2 changes the look only through `surface debug exposure|cells`. `SurfaceInput.class` is
  `surface_class` (`class` is a reserved WGSL word; caught by the naga test).
- **Tests:** `sf_surface` `view_tests` (recenter strips exact, wrap split, texels vs `sample`, shelter matrix,
  triggers, noise tiling); `sf_surface_bevy` (uniform ↔ WGSL struct and constants, naga parse + validate of
  `layer.wesl` with its import stubbed, headless plugin smoke: window corner, strips, casters, hook, no NaN).

## W3 (2026-11-05): wetness on every lit material, `bevy_pbr` patch "SW"

- **Patch "SW"** (vendored `bevy_pbr`, `superfargoth patch (own, SW)`; README row): `render/surface_weather.rs`
  (`SurfaceWeatherView(SurfaceWeatherBindings { uniform, cells, shelter, noise, sampler, shelter_sampler })`, binding
  consts 43-48), the view key/def in `render/mesh.rs`, layout + bind group entries in `render/mesh_view_bindings.rs`
  (the uniform is `uniform_buffer_sized(false, None)`: bevy_pbr knows nothing of its layout), and in `render/pbr.wesl`
  an `@if(SURFACE_WEATHER)` import of `sf_surface_bevy::view::apply_surface_weather_standard`, called after
  `apply_decals`, before lighting. The provider declares the WGSL bindings (`view.wesl`; a test checks the numbers
  against the consts). The def is pushed by `MeshPipeline::specialize` only, so prepass and shadow pipelines never see
  it; the deferred lighting pipeline just carries the bindings in its layout.
- **Render world** (`render.rs`): one uniform buffer (`U_SLOTS` × 16 B, written when the slots change), a repeat-linear
  sampler (cells, noise), a clamp-nearest one (shelter gather); each frame the render entities of cameras marked
  `SurfaceWeatherCamera` get the component once the three GPU images exist, and lose it when unmarked (interiors: no def,
  zero cost). The W2 `ShaderBuffer` uniform is gone.
- **Scaffold removed:** `TerrainMaterial` bindings 113-118, `TerrainMaterialKey`, `hook_terrain`, the `@if` bindings in
  `terrain.wesl`, `tag_terrain`, and `sf_surface_bevy`'s dependency on `sf_terrain_bevy`. The game glue
  (`mw_exterior_bevy` `surface_glue`, `no_surface_shelter_terrain`) copies `TerrainOrigin` into `SurfaceOrigin`, mirrors
  `TerrainCamera` as `SurfaceWeatherCamera` and tags `TerrainPatch` with `NoSurfaceShelter`.
- **Layer** (`layer.wesl`, CPU twin `sf_surface::layer`):
  - Exposure: Clockwork `surfaceExposure` shape: nine `textureGather`s cover a 6×6-texel tent (0.75 m), cover ramps from
    0.12 m to 0.35 m above the point (ROOF_NEAR/FULL re-calibrated) plus the slope's rise over two texels; lookup
    0.35 m out along the normal (W2). Replaces W2's 3×3 hard compare with 5 cm bias. Leaf hold and the `radius > 3` far
    path not ported (W2b).
  - Wet amount: cell R × facing (ground 1, walls `WALL_WET` 0.6 × (0.3 + 0.7 streak), world-plane streaks from noise B/A
    over 22 m × 18 m tiles, faces turned down dry) × drying patches (noise R, 32 m tiles: `smoothstep(0.6n, 0.6n + 0.4,
    wet)`) × exposure (not FAR). DYNAMIC (skinned) uses `instance_wet` only: dry until W8.
  - Material (Lagarde 2013): porosity `(1 − metallic)·saturate((rough − 0.3)/0.5)`; albedo × lerp(1, 0.2, porosity·wet)
    with saturation +0.3·wet·porosity; gloss `lerp(1, gloss, lerp(1, factor, 0.5·wet))`; film past saturation
    `wet·smoothstep(0.35, 1, wet)·(1 − 0.6·porosity)` takes roughness to ≤ 0.25 and dielectric reflectance to 0.354
    (F0 0.02).
  - Normals: the normal map's surface gradient against the geometric normal (Mikkelsen 2020) × (1 − film), one resolve
    (W4-W5 add their gradients before it). FAR (proxies, impostors): albedo and gloss only.
  - Debug view `porosity` added (`surface debug porosity`).
- **Switches:** `SurfaceSettings { layer, wet, shelter }` mask the flags (`--off surface`, `--off wet`) or skip the
  shelter map (`--off shelter`, and the Low preset); the hook stays compiled in.
- **Tests:** `sf_surface` layer twins (porosity heuristic, Lagarde identities, monotonic wetting, facing);
  `sf_surface_bevy` `shader_tests`: `wesl` links the real modules (vendored `bevy_pbr`/`bevy_render`/`bevy_core_pipeline`
  sources, our crates' WESL; Bevy's `canonical_path` rule) for the StandardMaterial fragment (static, skinned), terrain,
  proxy and impostor at Low/High/Ultra defs, with and without `SURFACE_WEATHER`, and naga validates each (30 compiles,
  ~2 s); binding numbers vs the patch; WGSL wet constants vs the twin; headless plugin (A/B switches, interior).
- **Why a patch, not `WorldMaterial = ExtendedMaterial<…>`** (the earlier design; moved from the spec at W2b): the data
  is per view, not per material; one patch reaches every `StandardMaterial` user (~30 spawn sites, characters included)
  with no type churn in `MaterialMapper`, `CookedMaterials`, fades or skinning; it repeats the GI patch. Binding count:
  group 0 holds at most 19 textures and 12 samplers with every feature on (W2b's canopy included); Bevy requests the
  adapter's limits, far above that on desktop (WebGPU's default 16 would not fit; not a target).

## W2b (2026-11-05): rain wash, light-independent shelter views, partial leaf shelter

Owner answers 2026-11-05 (DECISIONS): rain washes snow away faster and ash much faster; the shelter map draws at night;
foliage shelters partially (~40% through), roofs fully.
- **Rain wash** (`sf_surface`): `SurfaceTuning::snow_wash` 1/3 and `ash_wash` 2, cover cleared per hour per unit of the
  cell's blended wetting rate (`SurfaceRates::wet`, the rain's intensity: Morrowind rain 2 × raindrops/thunderstorm
  raindrops). `cover()` subtracts `wash` whether or not the channel falls (sleet on snow, rain through an ash fall); drying
  stays 0 while rain falls. A storm (wet 2/h): snow 2/3 per hour (4× the dry 1/6, Clockwork `surfaceweather.cpp`:
  melt `(1 + 3·rain)/(sSnowLife·dry)`), ash 4 per hour, gone in 15 game minutes (Clockwork `sWashRate` 0.25/min
  exponential: 95% in 12 min), 32× the dry 1/8. Linear like the rest of our model; no save change (tuning isn't saved).
  `grid::blend` returns `CellRates { snow, ash, rain, drying }`.
- **Own views** (`sf_sky_bevy::sun_view`): `DepthViewAnchor` + `depth_view_anchor()` (synced to the render world).
  `extract_anchors` (installed once) gives the anchor's render entity the marker and empty
  `RenderShadowMapVisibleEntities` / `RenderExtractedShadowMapVisibleEntities`; `inject_casters` and `spawn_view` accept an
  anchor or a shadow-enabled directional light (`owner_enabled`). Why it is stock: Bevy 0.20 has no depth-only view API;
  its shadow pass is the facility that draws mesh casters (alpha mask, GPU preprocessing) into a depth target, and
  `queue_shadows` / `specialize_shadows` read a view's casters only from `get_shadow_map_visible_entities` on the entity
  its `LightEntity` names (`Directional` picks the orthographic, unclipped-depth key); `prepare_lights` never sees our
  views (spawned after it). Visibility collection already treats any entity with the tables as a shadow owner. Rejected:
  a second `Camera3d` rendering to a texture (full main pass and its own sun cascades on every redraw, depth not readable
  without a copy node); a dummy `DirectionalLight` (cascades redrawn every frame, a light in every pixel's loop).
- **Canopy** (`sf_surface_bevy`): `ShelterFoliage` marker; `update_shelter` splits casters into the shelter map
  (`SHELTER_SUBVIEW` 0xFC) and the canopy map (`CANOPY_SUBVIEW` 0xFD, `SurfaceGpu::canopy`, same view, same frame:
  `ShelterFrame` + `CanopyFrame` sources); `Added<ShelterFoliage>` counts as streamed casters. Uniform slot 9
  `canopy` (x valid, y `SHELTER_LEAF_PASS` 0.4). Patch "SW" binding 49 `surface_canopy` (unfilterable float; the shelter
  sampler). `surface_exposure` gathers the canopy map at the same nine tent positions (only while `canopy.x > 0`, a
  uniform branch) and returns `open · (1 − (1 − pass) · leaf cover)`. Why two maps, not Clockwork's single map with leaves
  on even texels (`depthclipped.frag` `roofMap`, `surface.glsl` `roofSolidLanes`/`roofLeaves`): it needs a foliage flag
  in the depth shader (Bevy's prepass shader has none; a patch), halves roof resolution on a checkerboard, and needs a
  B-spline lattice to read cover back; a split caster list is CPU-side and game-agnostic.
- **Game** (`mw_exterior_bevy` `shelter_foliage`): a mesh whose `StandardMaterial` has diffuse transmission and an
  alpha mask (the leaf rule of `mw_assets` `Surface::foliage`: cutout leaf cards, Clockwork's "flora" alpha parts) gets
  `ShelterFoliage`. Limit: `--off foliagetransmission` drops the mark (leaves then shelter fully).
- **Cost:** CPU per redraw one caster pass as before (split into two lists; 16 m / 8 m / ≤ 1 per 30 frames, so not
  measured separately); GPU: a second depth view on redraw frames with only leaf casters; in the layer, nine more
  gathers per lit pixel while wet (exposure runs only when the wet flag is on).
- **Tests:** `sf_surface` `rain_washes_ash_faster_than_snow_and_both_faster_than_dry_decay` (ordering, closed forms,
  ash gone in 2 steps, snow at 1.5 h, step-grid bit equality); `sf_sky_bevy` `an_anchor_owns_views_without_a_light`;
  `sf_surface_bevy` headless plugin with no light at all: requests owned by the anchor, leaves only in the canopy
  request, same matrix, uniform `canopy`; binding 49 vs the patch; shader compiles per preset.

## W4 (2026-11-05): puddles and rain rings

- **Technique** (research: Lagarde "Water drop 2b - Dynamic rain and its effects", 2013; Clockwork `surfaceWater`,
  `rain_ripples.glsl`): Remember Me paints puddle sites in vertex colour and fills cracks from the height map against a
  rising flood level, then lerps gloss to 1, F0 to 0.02 and the normal to flat; ripples are a 256² ripple texture in
  4 layers enabled per quarter of rain intensity (mips were "unsatisfying"). Open worlds have no painted sites, so ours
  takes the site from a pool score (Clockwork's noise, plus terrain concavity) and keeps Lagarde's level/height
  shoreline and water surface; rings are procedural (no texture, binding or mip problem) with Lagarde's layering.
- **Layer** (`layer.wesl`, twin `sf_surface::layer`): runs when the wet or puddle flag is on. Puddle state `p` = cell G
  where the true surface is level (`smoothstep(cos 6°, cos 3°, level_up)`, owner 2026-11-06; was 4°/2°), times exposure (none under roofs; under leaves
  smaller puddles and fewer rings). Pool = noise atlas G at a 64 m tile + `0.35·saturate(basin / 0.1 m)`. Water depth
  `(pool − 1 + 0.3·p)/0.1 − height`, puddle = `smoothstep(0, 0.1, depth)`: lows (terrain layer height, object albedo
  luminance × 2) fill first and the shoreline follows them. Then wet = max(wet, puddle) for Lagarde's darkening;
  roughness → 0.06 (Bevy clamps 0.089), reflectance → 0.354 (F0 0.02), metallic → 0; the normal-map gradient × (1 −
  film)(1 − puddle) (flat water on the geometric normal), plus the rings' surface gradient × puddle × footprint fade
  (rings gone by 6 cm per pixel). One resolve. Sky/probe reflection is Bevy's env map at that roughness (no SSR).
- **Rings**: up to 4 drop grids (1.6 m cells × 1, 1.25, 0.8, 0.625: all divide the 256 m origin grid), grid k on over
  rain k/4..(k+1)/4; per cell one drop a cycle (1.2 cycles/s, own start phase, random spot by an integer hash of cell
  and cycle) with two trailing rings (a − b/2 + c/4); ring profile `blip`/`circle` ported; tilt 0.5.
- **Uniform slot 8** `time`: x animation seconds (`Time`, wrapped at 1000 s = 1200 whole cycles), y rain at the camera
  = `CurrentWeather.frame.surface.wet / SurfaceTuning::storm_wet_rate` (2), only while the puddle flag is on;
  `ripple_clock` holds x while no rings can show (no uniform write on dry frames). Rain 0 stops rings at once.
- **Inputs**: `SurfaceInput.footprint` (new: m per pixel from `dpdx/dpdy` of the world position, taken at the top of
  `apply_surface_weather_standard` in uniform control flow; terrain from its own derivatives; far proxies 0).
  `height` from albedo for StandardMaterial (`surface_height_of`), `TerrainSurface.height` (blended layer heights) for
  terrain; terrain `basin` = mean `height_at` 2 m out along ±X, ±Z minus the centre, ≥ 0, computed only while
  `surface_puddles_on()` and `N.y > 0.9`.
- **Switches**: `SurfaceSettings.puddles` (`--off puddles`, off on Low) masks `FLAG_PUDDLE`. No new bindings or patch
  hunks (SW unchanged); no shader def per layer yet (runtime flag only).
- **Tests**: `sf_surface` `puddles_fill_from_the_lows_and_dry_back`, `standing_water_is_a_mirror_with_water_f0`,
  `rings_follow_rain_and_stay_in_their_band` (incl. origin-grid divisibility, whole cycles at the wrap);
  `sf_surface_bevy` `puddle_switch_and_ring_clock` (mask, rain, clock holds), constants twin (31 consts), preset
  compiles now also require `surface_ripples` (and `terrain_ground` for terrain, `surface_face_up` for pbr) with
  the def on.
- **Known**: W3's streak tiles (22/18 m) don't divide the 256 m origin grid (streaks jump on an origin shift; logged).
  Ring hash cycle jumps once per 1000 s wrap (one drop moves mid-cycle).
- **Fix 2026-11-05 (owner SW-4: puddles on slopes)**: the gate was Clockwork's `smoothstep(0.94, 0.99, N.y)`, which
  is still ~0.97 at 10° and ~0.5 at 15° (zero only past ~20°), and it read the shading normal (smoothed mesh vertex
  normals; the filtered 8-bit clipmap normal on terrain). Now `SurfaceInput.level_up`, the true surface's up, gated
  none from 6°, full from 3° (2026-11-06): meshes the triangle face normal from `dpdx/dpdy` of the position
  (`surface_face_up`); terrain the steeper of the clipmap normal and the height field's central differences over 4 m
  (`terrain_ground`, R32F heights, same 5 fetches as the basin). Test `no_puddles_on_slopes` (5/10/20° planes and
  faces dry, a level hollow full). Morrowind's 8-unit height steps over 128 units are ~3.6°: such near-level
  terrain now gets little or no water (owner to judge).

## W2b fix (2026-11-06): shelter views draw the camera's render layers (SW-2)

- `sf_sky_bevy::sun_view` `spawn_view`: an anchor's view takes the owning camera's render-world `RenderLayers` (Bevy's
  `extract_cameras` copies them), a light's view the light's (`view_layers`, test `an_anchor_view_draws_the_camera_layers`);
  before, an anchor view got layer 0 and `queue_shadows` dropped every caster on a worldspace layer (maps empty, exposure 1).
  `sf_surface_bevy` `split_casters` keeps only casters whose layers meet the camera's (`SurfaceStats.shelter_casters` honest).
- Open (logged, not fixed): `SurfaceSystems` isn't ordered after `VisibilitySystems::VisibilityPropagate`/`CalculateBounds`
  (a caster streamed this frame can miss its redraw); a one-frame view loses casters whose pipeline isn't ready that frame.

## W5a (2026-11-06): cover looks, records and the layer-array binding (no shader cover yet)

- **Records** (`sf_surface::cover`): kind `surface.layer` = `SurfaceLayer { channel: snow|ash, albedo_height, normal?,
  arm?, tiling_m (4), max_depth_m? (channel default 0.15/0.03), roughness (0.8), sparkle (0), contrast (1) }` (VFS path
  strings, serde defaults in brackets; `normal` is optional: flat). `ActiveLayers { default, worldspaces }`: the
  adapter's per-worldspace, per-channel pick; `resolve` gives none for a missing record or one of the other channel.
- **GPU** (`sf_surface_bevy::cover`): the host inserts `CoverLayers([Option<CoverSource>; 2])` (record + decoded RGBA8;
  this crate reads no files); on change `pack` builds `Rgba8Unorm` 512² × 4 slices (2c albedo, sRGB bytes: decode after
  sampling; 2c+1 normal XY, roughness, AO) with `sf_terrain::bake::mip_chain` (now public, the terrain layers' filter),
  else a 1×1 one-slice neutral placeholder. Patch "SW" bindings 50 (`texture_2d_array`, filterable) and 51 (repeat,
  trilinear, aniso 16). Uniform slots 10-11 `snow_layer`/`ash_layer`: 1/tiling (0 = no look), depth, sparkle, contrast.
- **Morrowind adapter** (`mw_weather::cover`, records in the synthesized weather source): `mw_surface/snow` from `LTEX`
  `Tx_BM_snow_01.dds` (Bloodmoon), `mw_surface/ash` from `AL_ash_01` (`Tx_ashlands_08`), preference lists then the
  lowest plain keyword texture; tiling one land texel (7.3 m), snow roughness 0.65 sparkle 1, ash roughness 0.9.
  Picked for every worldspace. `WeatherSetup.cover` resolves them; `mw_exterior_bevy::surface_cover` decodes the
  original `.dds`/`.tga` (not the de-lit KTX2: no BC7 decoder), prints `surface cover <channel>: <path> (ok|missing)`.
- **Gap (logged):** Morrowind land textures have no height or normal map and no bake step derives one (the terrain
  arrays use the same flat defaults): cover height is the albedo alpha (opaque = flat) and the normal is flat. W5b may
  use the albedo luminance as height, as objects do for puddles; a derived normal is open.
- **Not yet:** the array is filled once at startup for the exterior (no worldspace switch repack); nothing samples it.
- **Tests:** record file parsing, defaults, patches, bad channel, pick (`sf_surface` `cover_tests`); pack layout, the
  placeholder, uniform slots, plugin repack (`sf_surface_bevy` `cover_array_packs_layers_and_placeholder`); binding
  numbers vs the patch, the view module unstripped (every binding declared and valid); adapter choice exact
  (`mw_weather` `cover_layers_pick_land_textures`) and on the real install (`real_cover_layers_name_install_textures`).

## W5b (2026-11-06): the cover drawn (`layer.wesl`, CPU twin `sf_surface::layer`)

- **Where:** `apply_surface_layer` step 3, before wetness and puddles, on class STATIC and TERRAIN only (characters are
  DYNAMIC under `SKINNED`, proxies/impostors FAR: never covered), gated per channel on `FLAG_SNOW`/`FLAG_ASH` and the look's
  `1/tiling > 0`. Snow first, ash over it (open question 1 default).
- **Amount:** cell B (snow) / A (ash) × `smoothstep(0.45, 0.8, N.y)` on the shading normal (base normal map included: tops of
  bumps first) × the shelter exposure (one exposure gather shared with wet/puddles).
- **Height blend** (the height lerp of layered snow/terrain materials): compared height
  `saturate(mix(SurfaceInput.height, 1 − look height, 0.5) + (drift − 0.5)·0.5)`, drift = noise G over 64 m; the cover's
  level `amount·(1 + 2e) − e` against it, `smoothstep(−e, e, ·)` with `e = 0.08 / contrast` (exact 0 at amount 0, 1 at 1).
- **Material:** look albedo (sRGB decoded), roughness, AO (× `diffuse_occlusion`), metallic 0, F0 ice (snow, 0.354) or 0.04
  (ash); covered porosity → 1 (rain then soaks it). Normal: the look's tangent normal (top-down: +X = world X, +Y = world −Z)
  as a volume gradient, projected (Mikkelsen 2020), replacing the surface's gradient by the blend in the one sum.
- **Sparkle** (`sparkle` > 0, snow): 2 cm world cells, `sparkle × 3 %` glint (hash), tilt ±0.35 and roughness 0.1, fading
  from 6 mm to 2 cm pixel footprint (gone past ~20 m at 1440p).
- **Sampling:** `textureSampleGrad` with the caller's `dpdx/dpdy` (new `SurfaceInput` fields: StandardMaterial and terrain
  pass theirs, `surface_input_of` zero), so trilinear + aniso in any control flow. UV = render XZ / tiling + `cover_uv`
  (new uniform slot 12, `fract(origin.xz / tiling)` in f64 per channel; `cover::cover_uv`): world-aligned and exact.
- **Plain looks** (`cover::with_height`, `normal_from_height`, pack time): all-opaque alpha → height = linear luminance
  auto-levelled between its 2nd/98th percentiles; no normal map → central-difference normal of that height (strength 3).
  Records with a real height/normal keep theirs.
- **Switches:** `SurfaceSettings::cover` (`--off cover`, `Part::Cover`) masks the snow/ash flags; `surface set snow|ash v`
  forces cells. No live `flip cover` (live_ab.rs was busy with two other sessions); `flip surface` remains.
- **Checked:** cell state reaches the texture (each `advance` step bumps `WorldSurface.generation` → full window rewrite).
- **Open (logged):** no worldspace-change repack (the Morrowind adapter picks one look for every worldspace, so nothing to
  repack today; needs the host to re-resolve `ActiveLayers` on `WorldWeatherMap.worldspace` change); cover ends at the
  cell window edge (~2 km, the window's hard zero, as wetness); FAR proxies stay uncovered (pop at the proxy distance in deep
  snow); prepass normals unmodified (spec "Prepass and SSAO"); Morrowind terrain layers have flat alpha, so the terrain's
  height is 1 near and 0.5 past the splat range: partial cover shifts with distance (fix: `with_height`'s luminance
  fallback in the terrain bake, which also gives puddles real terrain lows; needs a re-bake and an SW-4 re-check).
- **Tests:** twins `cover_endpoints_and_order`, `cover_amount_by_slope_and_shelter`, `cover_height_mixes_surface_cover_and_drift`,
  `sparkle_fades_with_distance`; constants vs WGSL (45); uniform layout; derived height/normal, cover UV, `--off cover` mask
  (`cover_derives_height_normal_and_uv`); compile per preset ± `SURFACE_WEATHER` now also requires `surface_apply_cover`.
