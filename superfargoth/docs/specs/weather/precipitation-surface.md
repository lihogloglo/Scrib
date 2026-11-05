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

Superseded (2026-11-05) by [../surface-weather.md](../surface-weather.md): deterministic CPU state in `sf_surface`
(wet + puddle per climate, snow + ash per 32 m cell, saved) instead of a GPU-simulated clipmap; a vertical
shelter map beside the slanted precipitation map; the hook reaches `StandardMaterial` through an own
`bevy_pbr` patch instead of `WorldMaterial = ExtendedMaterial<…>`. Ripples there are procedural, not an atlas.
