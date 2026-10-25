## Precipitation

**Technique** (Tatarchuk 2006; RDR2 and Watch Dogs follow the same structure):
- **Volumes**: 3 nested camera-centered boxes (near 8 m, mid 24 m, far 64 m). Each is one instanced
  draw of N quads. A drop's position is closed-form: `p = wrap(hash(id) · size + v·t, camera box)`,
  with `v = fall + wind`. No simulation, no CPU work, no spawn/despawn. The far layer uses thinner, more
  transparent streaks, and the fog provides the far "rain curtain".
- **Looks** (`PrecipKind`): rain streaks (stretched along screen-space velocity, refraction-free
  luminance from the sky ambient + nearby lights, lit by lightning), snow/ash flakes (flutter from the
  wind field, soft round sprites, ash in a dark grey-brown with the sun scattering forward),
  dust (dense, small).
- **Occlusion**: a top-down depth map along the precipitation direction (default 1024² over 128 m,
  R32F). It comes from an **occlusion camera**: an orthographic `Camera3d` rendering opaque casters
  into a small target, no atmosphere or post, refreshed when the camera moves 1/8 of the coverage or
  every 8 frames (amortized). A render-graph node copies its depth into the persistent texture.
  Streaks below the occluder depth are discarded in the vertex shader. The same map decides which
  surfaces get wet, snowy or ashy (below).
- **Splashes**: N splash sprites per frame at random XZ around the camera, placed at the occlusion
  depth (the top surface there), with an animated sprite sheet. Ripples on puddles and water: a
  tiling animated ripple normal atlas (Lagarde 2a) that the surface hook samples.
- **Drips, wet camera**: out of scope until a camera FX spec.
- Rate = `PrecipTerm.intensity` × the cloud shadow map's local cover (no rain under a clear hole when
  the weather is "showers").

## Surface state

WISHLIST §5 and early decision 5, built on the hook `sf_terrain_bevy` already calls
(`apply_surface_state(&pbr, world_xz)`, a no-op today).

**Data.**
- **Surface-state clipmap** (GPU, RGBA8, 2 levels: 512² at 0.25 m = 128 m, and 512² at 2 m = 1 km,
  toroidal like the height clipmap): R wetness, G puddle water level, B snow depth, A ash depth.
- **Exposure**: a surface point is exposed if its height ≥ the occlusion depth − ε (open sky) or it
  faces up under the open sky (objects: per-pixel check in the hook. The clipmap stores the terrain/top
  surface).
- **Update compute** each frame (a few µs): `wet += rate·exposed·dt − drying(sun, wind, temperature)·dt`,
  puddle level rises after wetness saturates, snow/ash add when their channel's precipitation falls
  and melt/blow off per preset. Rates come from `WeatherFrame.surface_rates` (game hours).
- **Coarse persistent grid** (`sf_surface`, CPU, 32 m cells over the worldspace, in the save): fed by
  the same model at low rate. It seeds the clipmap when regions stream in, so you don't walk into a dry
  town after a storm.
- **Permeability per surface** (`TerrainLayer.surface` record, terrain.md): porosity (how dark and how
  fast), puddle allowed, snow/ash adherence. Objects get it from a material extras key or a default.

**Material response** (Lagarde 3a/3b), in `surface_state.wesl`:
- wet porous: albedo × `mix(1, albedo-dependent darkening, wet·porosity)`, roughness → `mix(r, 0.1, wet)`,
  specular F0 unchanged;
- puddles: where `puddle_level > height_signal` (terrain: the layer height channel already used for
  height blending; objects: an up-facing mask × a world-space noise), a water layer: albedo darkens,
  roughness 0.02, normal → flat + ripple normals × rain intensity, F0 0.02;
- snow/ash: on `N·up > threshold`, height-blended with the layer height (terrain.md height blend),
  albedo/roughness/normal toward the snow/ash material (`PrecipKind` points at its layer). Terrain also
  displaces by depth through the deformation stack (terrain.md layer 3);
- far field: proxies and impostors get the wetness darkening only (cheap, sub-pixel puddles).

**Which materials get the hook.** Terrain (has it), impostors and proxies (`sf_lod_bevy` materials),
and **all cooked world statics**. The streaming `MaterialMapper` (streaming.md) maps cooked materials to
`WorldMaterial = ExtendedMaterial<StandardMaterial, SurfaceExt>` instead of `StandardMaterial`. Bindless
and batching are kept (one extension type). Its fragment builds `PbrInput` the way Bevy's does, calls
the hook, and lights with `apply_pbr_lighting`. The hook is cheap (one clipmap fetch + ALU) and
compiled out per quality preset.

**Reflections.** Wet looks need reflections. The environment map (`AtmosphereEnvironmentMapLight`)
gives the sky everywhere. Puddles that mirror buildings need SSR, which in Bevy 0.20 requires depth +
deferred prepasses. The exterior camera has no depth prepass (far-field.md pitfall 2). Whether the
deferred prepass honors the `VisibilityRange` dither is untested (it has a fragment stage, unlike the
depth-only prepass). Open question 4. The Solari path reflects by ray tracing anyway.

**Characters** (built with characters): per-character wetness uniform: rain exposure (occlusion map at
the head), immersion height from the water query API (WISHLIST §4), drying per hour. The character
material darkens below the waterline height and with exposure.

