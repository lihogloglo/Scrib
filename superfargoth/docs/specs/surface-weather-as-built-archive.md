# Surface weather: as built, archive (W1-W2)

Older slice notes moved out of [surface-weather-as-built.md](surface-weather-as-built.md) (budget, 2026-11-06,
W6a). Code is the source of truth.

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

