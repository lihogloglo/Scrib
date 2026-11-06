# Surface weather: as built

Detail moved out of [surface-weather.md](surface-weather.md) (spec budget). The spec keeps the design; this file
keeps the as-built shapes and slice notes; W1-W2, W2b, W5a and W5b moved to [the archive](surface-weather-as-built-archive.md).
Code is the source of truth.

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
- **Origin shift (fix 2026-11-06)**: streak tiles are 256/12 and 256/14 m; hash grids (drops, glints) wrap their cell
  index to `ORIGIN_GRID_M` (`surface_wrap_cell`, UE5's `LWCWrap` idea): nothing the layer samples changes on a shift.
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
- Fix 2026-11-06 (SW-5/SW-6 "resets"): Bevy extracts a mesh only while some view's `ViewVisibility` holds it, so a redraw
  drew only casters in the camera's or the sun cascades' frusta and the cover flipped at each redraw. `mark_casters_visible`
  (`CheckVisibility`, after `SurfaceSystems`) marks the redraw's casters (out of `VisibilityRange` excepted), as a light
  does; `SurfaceSystems` runs after `VisibilityPropagate`/`CalculateBounds`. Open: a one-frame view loses casters whose
  pipeline isn't ready that frame.

## W6a (2026-11-06): trail ring, stamps, refill, prints in the shading (no displacement yet)

- **Ring** (`sf_surface_bevy::trail`, CPU twin `sf_surface::trail`): 1024² `Rgba16Float` (R depression m ≤ 0, G lip m ≥ 0;
  B, A spare: rg16float is not a core storage format) at 6.25 cm over 64 m, toroidal, window = camera's world texel −512
  (whole texels; `window_min`), address = world texel mod 1024. Two textures, ping-pong: the pass reads last frame's,
  writes the other (W6b's previous ring). Mesh-view binding **52** (patch "SW"), sampled bilinear with the repeating
  filtering sampler (47); uniform slots 13 `trail` (render XZ of the first texel's corner, 1/texel, depth) and 14
  `trail_cells` (first texel's address, edge). Depth = the deepest cover look (`snow_layer.y`, 0.15 m today).
- **Pass** (`trail.wgsl`, Core3d before the prepass, once per frame): per texel keep if it held the same world texel
  (else 0: entering rows clear), refill, stamps (Min R / Max G), lip cap (`G ≤ 0.25·D·(1 + R/D)`: no lip in a print).
  Runs only on frames with stamps, a refill quantum or a moved window (~16 MB traffic, est. 0.06 ms). One compute pass
  instead of instanced Min/Max-blended quads: re-centre, refill and stamps all read the old ring anyway.
- **Stamps:** `TrailStamp { at: WorldPos, forward, half_m, depth }` messages; ≤ 40 m from the camera, nearest 64 per
  frame. Profile: flat bottom, soft wall (inner 30 %), quadratic lip 0.25·D high, 0.6 radius wide (ROTTR's rim). Game
  source (`mw_exterior_bevy::trails::foot_stamps`, after `AnimSystems::Apply`): every posed `Animator` with legs (player
  avatar too: it is not a `Mover`; `Mover.loco.grounded` read None outside the controller step), foot = the animator
  entity's transform × the effector's model position; planted while the leg's contact curve > 0.5 (else effector < 12 cm);
  one stamp per foot-down at the plant pose (stamping every planted frame smeared skids in the debug run), 0.16 × 0.09 m
  half-size × body scale, 5 cm toe-ward, facing the body; only within 0.4 m of the terrain height (no print under bridges).
- **Refill:** `6 × (snow rate × snow depth + ash rate × ash depth)` m per game hour from `CurrentWeather` (frame rates,
  `hours` deltas), released in 2 mm quanta (a half-float step at 0.45 m is 0.24 mm, so small steps would round away).
  Morrowind snow (0.5/h) refills a 0.15 m print in 20 game minutes. No fall: prints stay. Mud: none yet (no mud channel).
- **Shading** (`surface_apply_cover`, terrain only): the print cuts the amount (`saturate(amount + h/d)`, h =
  `clamp(R, −d, 0) + G·d/D` per channel depth d: full print = bare ground), and the central-difference gradient of h
  (4 extra taps, only where the pixel's own sample is non-zero) joins the sum, weighted by the uncut blend, fading as a
  pixel grows from 1 to 4 texels; the ring fades out over its last 2 m. `surface debug trails`: prints blue, lips red.
- **Switches:** `SurfaceSettings::trails` (`--off trails`, `Part::Trails`; no live flip: live_ab.rs busy with N1/N2).
- **Debug run (agent, Skaal Village, snow):** `surface debug trails` shows alternating left/right prints with lips behind  the player; in the normal look they are faint (Solstheim's terrain texture is itself snow, so a cut shows snow, and the  overcast light is flat): the gradient carries them until W6b's displacement. Console Enter can't be sent over BRP.
- **Open (logged):** lying bodies don't stamp yet (ragdoll/knockdown capsules live in `mw_delirium_bevy`; a later slice);
  statics never show prints; prints on a rock over terrain within 0.4 m still print the terrain; the pass runs per
  frame while walking (could go dirty-rect with a copy if WM finds it costly).
- **W6b needs:** displacement = `ground + cover·d + h` with the same `surface_trail_height` (move it to a shared
  `trail.wesl` the terrain prepass imports); the previous ring is `TrailGpu.sampled[current ^ 1]` (bind as 53 with the
  previous `trail` slots: keep last frame's uniform values for the motion vectors); bind both before the prepass
  (the pass already runs before it); knee-deep depth 0.45 m = `CoverChannel::default_depth_m` or the record's
  `max_depth_m` (refill quantum and half-float precision checked for 0.45 m).
- **Tests:** twins `window_is_centred_on_whole_texels`, `world_of_covers_the_window_once`,
  `recenter_clears_exactly_the_entering_rows`, `stamp_carves_to_the_ground_with_a_lip_around_it`, `profile_*`,
  `stamps_combine_min_on_depth_max_on_lip`, `lip_never_stands_in_a_print`, `decay_refills_prints_and_flattens_lips`,
  `refill_is_released_in_quanta`, `pass_keeps_moves_and_stamps` (full 1024² CPU pass); plugin
  `trails_take_stamps_and_place_the_ring`; shaders: `trail_pass_compiles_and_matches_the_cpu_twin` (naga, uniform size),
  binding 52 vs the patch, uniform layout, compile per preset ± `SURFACE_WEATHER`.

## W6b (2026-11-06): the cover raised as geometry, motion vectors, knee-deep snow

- **Height:** visual terrain = ground + Σ channels `coverage · amount · max(d + trail_height(R, G, d, ring), 0)`
  (`layer.wesl` `surface_cover_raise`, twin `sf_surface::trail::cover_raise`): amount = cell × slope ramp (ground normal)
  × exposure (the full shelter + canopy tent at the ground point), coverage = the shading's height blend
  (`surface_cover_coverage`, twin `layer::cover_coverage`: drift, the look's height, the surface's lows) at the vertex
  (`surface_raise_coverage`: look and layer heights prefiltered to the vertex spacing, `common::layer_blend_height`), d
  the look's depth; a full print reaches the ground, lips stand on top, ash in a snow-deep ring cuts only its own depth.
  Fix 2026-11-06 (SW-6 paving): the raise was `amount · d`, so a thin cover (0.4) lifted the whole ground 0.18 m; now
  bare ground (coverage 0) stays down. Morrowind's records: snow 0.45 m (knee deep), ash 0.03 m
  (`mw_weather::cover::{SNOW,ASH}_DEPTH_M`, record `max_depth_m`; framework default stays 0.15 / 0.03). The 2 mm refill
  quantum still moves a 0.45 m print in half float (0.24 mm step; test).
- **One function, both passes:** `sf_surface_bevy::view::surface_raise_at(p, n, base, texel_m)` (x now, y now without
  prints, z last frame) under `common::terrain_raise(t)` (`sf_terrain_bevy` `common.wesl`), on LOD ≤ 0, weighted by
  `TerrainVertex::raise_w` = `raise_fade` (twin `CdlodSettings::raise_fade`): 1 to 0.375 × LOD 0's range (18 m), a
  smoothstep to exactly 0 at 0.9 × (43.2 m), by the vertex's 3D distance (equal at shared seam vertices; LOD 1 always lies
  past LOD 0's range). Fix 2026-11-06 (SW-6 ring edge): it was the detail weight `near_w`, a linear 0.45 m drop over LOD
  -1's 3.6 m morph zone (20.4-24 m): 12.5 %, steeper than a standing camera's grazing angle there, so the near snow
  showed a crest following the camera, and the fragment shaded the ramp; now ≤ 2.7 %, and the fragment tilts by the
  unweighted cover slope times the weight (`raised_normal`), so the fade itself never shades. Node bounds grow on LOD ≤ 0.
  The main pass (`SURFACE_WEATHER`) and the prepass (`SURFACE_WEATHER_PREPASS`) both add
  `x` to the vertex Y; positions are `@invariant`, so depth matches exactly (compile test compares the linked bodies).
- **Prepass bindings (patch "SW", W6b):** Bevy's prepass view bind group is one global group (view, globals, previous
  view, visibility ranges); material bindings were the alternative but the ring ping-pongs and its uniform lives in
  the SW buffer. So views with `SurfaceWeatherView` get `MeshPipelineKey::SURFACE_WEATHER` in their prepass key, two
  extra prepass view layouts (+ bindings 43-53) and a per-view `SurfaceWeatherPrepassBindGroups`; every SW binding is
  now vertex-visible. Shadow views never carry the component: shadows stay undisplaced (see Open).
- **Motion vectors:** the prepass outputs `previous_world_position` = ground + `z`: last frame's ring (binding 53,
  `TrailGpu::previous` = the ring bound last frame, the same one when no pass ran) at last frame's placement (uniform
  slots 15 `trail_prev`, 16 `trail_prev_cells`, copied each frame in `update_trails`), with
  `previous_view_uniforms.unjittered_clip_from_world`. Cells and exposure are taken as now (they change over game
  minutes). The ground's own morph is still treated as static (as before W6b).
- **Normals:** the fragment tilts the clipmap normal by the slope of `y` (the cover without prints, interpolated) from
  screen derivatives against world XZ (`terrain.wesl` `raised_normal`); the prints' slope stays per pixel in the
  layer (W6a gradient, 6.25 cm, finer than the 12.5 cm vertices). Per-vertex central differences were rejected: four
  more exposure tents (18 gathers each) per vertex in both passes. Prepass normals (SSAO) stay the ground's.
- **Bounds:** `TerrainSettings::raise_m` (game sets it from the deepest cover look, `mw_exterior_bevy::trails::
  terrain_raise`); `node_margins` grows LOD < 0 node AABBs upward by it, keyed so a change rebuilds them.
- **Characters:** the collider, foot probes and stamps use physics/height-map ground; the raise is GPU only, so feet
  sink into the snow and prints carve to where they stand (RDR2). Nothing lifts the player.
- **Live A/B:** `on|off cover`, `on|off trails` (`mw_devtools_bevy::live_ab`).
- **Open (logged):** cascade-0 shadows undisplaced (prints don't self-shadow; the raised surface sits above its
  shadow caster, so it is lit as the ground would be); POM skipped (vertex grid 12.5 cm + per-pixel print gradient);
  statics and rocks don't get raised snow (it buries their bases); the raise ends at ~24 m (near_w), the cover look
  continues flat beyond; terrain under shallow water near shores rises too.

## W7 (2026-11-06): kick-up puffs in snow and ash

- **Burst pool** (canonical GPU-instanced bursts, no particle crate: `bevy_hanabi` is not on Bevy 0.20): `sf_surface::kickup`
  (Bevy-free twin) + `sf_surface_bevy::kickup` / `kickup.wesl`. A fixed ring of 64 bursts × 12 sprites, one draw of a fixed
  quad mesh (`DrawBand::Overlay { order: -3 }`, under the precipitation boxes); the CPU writes only the bursts (render position,
  age, facing, strength, kind, seed) into a 3.2 KB uniform; hidden while none lives.
- **Source:** the same `TrailStamp` foot-downs as W6 (own `MessageReader`; `mw_exterior_bevy::trails` now stamps while trails
  *or* kick-up is on). A stamp within 30 m throws when its cell's snow or ash (`sf_surface::sample`) is ≥ 0.15 and that look
  is drawn (`SurfaceUniform::cover[c].x > 0`). Strength = cover ramp × pace (`StepClock`: distance to the last print a step
  away over the time between; stand 0.35, sprint 1) × foot size.
- **Sprites (closed form):** radial spray biased along the facing, then the exact linear-drag solution toward the wind under
  gravity (`p0 + v∞t + (v0 - v∞)(1 - e^{-kt})/k`), clamped above the foot, fade in/out, grow over life. Snow 0.45-0.75 s,
  apex ~0.15-0.3 m, white-blue; ash ~1.2 s, light, follows 0.8 of the ground wind (½ the weather's 10 m wind), grey-brown.
- **Light** (not precip's fixed brightness, WE-9 cause 3): per pixel at the sprite centre (the mesh view's light bindings are
  fragment-only): directional lights (sun, moon) over a Lambert-sphere phase with one hardware PCF shadow tap, + the flat
  ambient (night floor) + the view's diffuse environment map (up and toward the camera) × exposure; soft against the depth
  prepass (12 cm); fogged by `TranslucentFog`. Point lights not summed (puffs by a lamp only get the floor; logged).
- **Cost (estimate, not measured):** CPU ~10-20 µs (message read, 64-slot pack, uniform re-upload while live). GPU: 3 k
  vertices, ~0.1-0.3 M px of a ~6-fetch fragment while running in snow, ≈ 0.01-0.03 ms at 1440p + DLSS Q (budget 0.05).
- **A/B:** `--off kickup` (`Part::Kickup` → `KickupSettings::on`). No live flip: `live_ab.rs` was being changed by W6b.
- **Not built / logged:** puddle splashes (the puddle level is a per-pixel function of the terrain's basins; needs a CPU
  twin at a point), point lights, shelter (a stamp under an overhang on a snowy cell still throws; the shelter map is GPU-only).
