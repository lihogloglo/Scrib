## Wind

**Field** (`sf_wind`; the CPU and WGSL twins agree to 1e-4, checked by a test that runs the WGSL on
the GPU headless and compares; see "Wind as built" below):
- **Global**: direction + mean speed from `WindState` (weather). Speed eases with `WindTerms`.
- **Gusts**: Megalophobia's two-octave curl-noise field advected downwind. It produces discrete gust
  cells that form, travel, swirl and die, plus a calm bias for patchy stillness and a capped swirl so
  nothing leans upwind. Sampled at `rel + phase_1024` (gust scale ≤ 1 024 m divides the period).
- **Local sources**: up to 32 `WindSource`s in a storage buffer (directional gust, radial burst for
  explosions and spells, vortex, a storm source radial from a world point). Each has a falloff and a
  lifetime. Gameplay adds and removes them.
- **Altitude**: speed × a log profile above ground. Cloud wind uses its own altitude speed from the
  preset.
- **Later (open question 2)**: a Tsushima-style low-resolution 3D wind texture around the camera with
  advection/injection, if the analytic field + sources looks too uniform in practice.

**Consumers.** All read either `sf_wind::sample(rel_pos, t)` in WGSL or `velocity_at` on the CPU:

| Consumer | Use |
|---|---|
| Grass (ARCHITECTURE §7.1) | blade lean = field direction × gust envelope. Bézier bend + flutter. The same function in forward, prepass, shadow and motion-vector passes (`time_prev`), the Megalophobia lesson |
| Trees (cooked L0) | hierarchical sway from baked vertex data (below) |
| Impostors | per-instance trunk bend: the quad's top vertices shear by the field at the instance + a per-model stiffness from the atlas header |
| Cloth (M4) | CPU `velocity_at` per cloth body (aerodynamic force) |
| Precipitation | streak velocity = fall + wind. Snow/ash also flutter |
| Clouds, fog banks | scroll/advection (cloud-altitude speed for clouds) |
| Water (M6 water spec) | wind speed/direction for FFT wave spectra |
| Audio | wind loudness/gust cues |

**Bake-time wind data** (`wind.paint`, a bake step in `sf_wind` (feature `bake`) over `mesh.cook`,
guided by an adapter hook in the style of `FarSource`:
`fn wind_profile(&self, model, summary) -> Option<WindProfile>`):
- For models the adapter marks as foliage (Morrowind: `flora_*`, like the impostor rule): per vertex
  `wind0 = (pivot.xyz relative to the model, stiffness)` and `wind1 = (level 0 trunk/1 branch/2 leaf,
  phase, flutter weight, height fraction)`.
- Derivation without authored data: trunk = the connected component(s) containing the lowest
  vertices. Branches and leaf cards = the other connected components, after welding by position. Pivot
  = the component's vertex nearest the trunk axis. Alpha-masked materials → leaf level. Stiffness from
  the component's size and height (Pivot Painter conventions). Deterministic; keyed by `mesh.cook`
  and the profile's version.
- Stored as a separate product (one vertex stream per cooked LOD) that `sf_model_bevy` appends as two
  optional attributes at load, so `mesh.cook` stays unchanged. **The ray-tracing variant keeps the exact
  position/normal/uv/tangent layout** (Solari rejects other layouts, sky-lighting.md open question
  3): `sf_model_bevy` drops wind attributes from the RT LOD, so ray-traced trees are static.
- Merged far-field meshes (L1-L3) carry no wind. At 400 m+ sway is sub-pixel. Impostors use the
  per-instance bend above.

### Wind as built (WD1 part 1, wd1a, 2026-10-18): `sf_wind`

**Reuse check (rule 8).** Bevy 0.20 has no wind. `bevy_feronia` 0.8 (Bevy 0.18) is grass-only shader
wind with no CPU twin. Noise crates (`noise`, `fastnoise-lite`) are CPU-only and their float hashes
can't be mirrored bit for bit in WGSL. So: our own, porting Megalophobia's `grass_wind.wgsl` (MIT OR
Apache-2.0 by the owner, ARCHITECTURE §10 2026-10-12; provenance in `sf_wind`'s crate docs and the
shader header).

**API** (Bevy-free; deps `glam`, `serde`, `sf_coords`):
- `Wind` (moved from `sf_sky`, re-exported there unchanged: `velocity` at the reference height,
  `gustiness`, `gust_length`, `lerp`, plus a stateless `velocity_at(pos, t)` for constant wind).
- `WindSettings` (swirl 0.9, gust bias 0.42, band 0.3 (Megalophobia's), gust travel 1.0 × mean speed,
  roughness z0 0.1 m, reference height 10 m (WMO; weather-adapter.md maps `Wind Speed` at 10 m), profile
  cap 2).
- `WindState` (saveable): `wind`, clock, the two octaves' drift integrated in f64 and wrapped to
  `PERIOD` (so a speed change never jumps the field), up to `MAX_SOURCES` = 32 `WindSource`s
  (`Directional { direction }`, `Radial`, `Vortex { axis }`; center `WorldPos`, strength, inner/outer
  radius smoothstep falloff, attack/duration/release envelope; `add_source` returns `None` when full;
  expired sources drop in `advance`).
- `WindFrame` = `state.frame(&settings, origin)`: `WindParams` (6 × vec4, 96 B uniform) + `WindSourceGpu`s
  (3 × vec4, 48 B each, ≤ 1.5 KB storage), each with `to_array()` for the upload, and the CPU sampler
  `sample(rel, above_ground)`, `global`, `gust`, `profile`. `state.velocity_at(&settings, WorldPos, h)` builds
  a frame around the point's cell.
- `sf_wind::WESL`: `src/shaders/sf_wind.wesl`, pure functions, no imports, no bindings: `wind_global(params,
  rel, above_ground)`, `wind_gust(params, rel_xz) -> WindGust { dir, curl, gust }`, `wind_profile`,
  `wind_source_count`, `wind_source(source, rel)`. Consumers bind the two buffers and sum the sources.

**Model.** `v = dir × speed × (1 + gustiness·(2·gust − 1)) × profile(h) + Σ sources`. `dir` is the axis
turned by the capped curl (at most `asin(swirl)` off it, so never upwind); `gust` the smoothstepped
two-octave potential. Profile: `ln(h/z0 + 1) / ln(h_ref/z0 + 1)`, 0 at the ground, capped. Sources:
directional `dir·s·fall`, radial `r·s·fall / max(|r|, 0.5 m)`, vortex `(axis × r)·s·fall / max(|axis × r|,
0.5 m)`; sources get no altitude profile and the never-upwind cap applies to the weather wind only.
Grass flutter (Megalophobia's harmonic `wind_press`) is a consumer detail, not part of the field.

**Changes from Megalophobia, and why.**
1. **Integer lattice hash** (PCG, Jarzynski and Olano 2020) instead of `fract(dot(...))`: the float hash
   multiplies values to ~1e4 before `fract`, so one ulp of FMA difference moves the noise by ~1e-3 and the
   twin could never hold 1e-4. PCG on wrapped integer cells is bit-exact on CPU and GPU.
2. **Origin phase.** The lattices tile `PERIOD` = 1 024 m (4 grid cells; gust cells snap to `1024/n`, the
   second octave to `1024/round(n/0.37)`). The CPU computes `(origin mod 1024 − drift) mod 1024` in f64 from
   the origin's cell (exact) and uploads it **split** into whole lattice cells (`cells`, integers, added to the
   cell index) and a fraction, so the shader's float math is only `rel/cell + frac`. A first version added the
   meter offset to `rel` and lost 1.3e-4 m/s across a recenter (ulp of 1 200 m); split, the error is the
   rounding of `rel` itself.
3. **Drift integrated on the CPU** per octave (Megalophobia multiplied time by a fixed speed).

**Twin verification (choice).** `tests/wgsl_twin.rs` runs `WESL` + a small compute harness on the GPU with
headless `wgpu` 30 (Bevy 0.20's version; dev-dependency only, so no library pulls it), default adapter,
falling back to the software adapter (WARP on Windows), skipping with a message if neither exists. Not
"naga + mirrored golden table": naga has no interpreter, and a table generated by the CPU twin is
circular; a table captured from the GPU would go stale with every shader edit. The GPU run is the real
thing and costs ~0.5 s. Golden points: 2 048 scattered over ±600 m and −1 to 400 m above ground, ±4 cells of
lattice boundaries, the source centers. Tolerance 1e-4 **relative above 1 m/s** (`|Δv| / max(|v|, 1)`):
GPUs contract `rel·inv + frac` into an FMA, a 1-ulp difference that the steep gust envelope amplifies.
Measured (RTX 4060 Laptop): worst 1.1e-5 relative (2.4e-4 m/s absolute at a 39 m/s peak), gust envelope
4.3e-6.

**Other tests** (`cargo test -p sf_wind`): never upwind (swirl 0.99, gustiness 1, three directions,
20 steps × 500 points, also the turn angle ≤ asin(swirl)); phase-exact across a ~390 m origin recenter
330 km out (≤ 5e-5 m/s, measured 2.9e-5: the rounding of `rel`); tiles the period; a speed change keeps
the gust pattern; log profile; sources (radial burst envelope and expiry, vortex handedness, directional,
capacity 32); patchiness; `Wind::velocity_at` stays within `mean·(1 ± gustiness)` (the old `sf_sky` test,
still passing through the re-export).

**Open (logged, not built).** Changing `gust_length` across a snap step (`n` changes) swaps the lattice:
crossfade two frames if a weather blend shows it. No eviction when 32 sources are live. Vortex falloff is
spherical (no tornado height). Motion vectors need last frame's `WindParams` (wd1b: upload a `prev` copy or
let consumers pass it). Cloud wind keeps its own altitude speed (clouds read only the direction).
`WindTerms` easing belongs to `sf_weather` (WE1).

### Wind as built (WD1 part 2, wd1b, 2026-10-18): `sf_wind_bevy`

**Reuse check (rule 8).** Bevy 0.20's `ShaderBuffer` asset (`bevy_render::storage`) is the upload: a
same-size rewrite is a `queue.write_buffer` into the existing GPU buffer, so bind groups (and every
material that binds it with `#[storage(n, read_only)]`) stay valid. WESL modules go through
`bevy_shader` as usual. Nothing to build but the glue.

**Plugin** (`WindPlugin { wind, settings }`; deps `bevy_app/asset/ecs/math/platform/render/shader/time`,
`sf_coords`, `sf_wind`; **not** `sf_world_bevy` as planned above: the app writes the origin, like
`sf_atmos_bevy::CloudsOrigin`, which keeps big_space and the streaming graph out of the crate):
- `WindField` resource: `state: WindState` (weather writes `state.wind`, gameplay `add_source`),
  `settings`, and after `WindSystems::Advance` (`PostUpdate`) `frame()` / `prev_frame()` / `sample(rel, h)`
  (the CPU twin for cloth, particles, audio). `WindOrigin(WorldPos)`: written by the app before
  `PostUpdate`'s advance. `WindStats`: CPU ms of the advance + upload.
- Each frame: `state.advance(dt)` (virtual time, so pausing stops the wind), `frame(origin)`, and the
  previous state framed at the **current** origin; pack and rewrite the buffer.
- **One buffer, not uniform + storage** (the handoff's plan): `WindBuffer(Handle<ShaderBuffer>)` holds
  `WindData { params, prev_params, sources: array<WindSource, 32>, prev_sources: array<WindSource, 32> }`,
  3 264 B, fixed size (so "bind at least one element" never arises and the size never changes). One
  binding slot per consumer, and materials get it through `AsBindGroup` with no custom bind group.
- Without rendering (`Assets<Shader>`/`Assets<ShaderBuffer>` absent, a server) only the CPU field runs.

**WESL.** `sf_wind::WESL` is inserted as a UUID-handle `Shader` at path `embedded://sf_wind/field.wesl`;
bevy_shader turns `embedded://a/b.wesl` into module `a::b`, so consumers `import sf_wind::field::...`.
`sf_wind_bevy::wind` (`src/wind.wesl`, `load_shader_library!`) declares
`@group(constants::SF_WIND_GROUP) @binding(constants::SF_WIND_BINDING) var<storage, read> wind: WindData`
and `wind_sample(rel, above_ground)` / `wind_sample_prev(...)` (global + loop over the live sources).
The slot comes from two UInt shader defs (`wind_shader_defs(group, binding)`), which bevy_shader exposes
as `constants::*` (the pattern bevy_pbr uses for `MATERIAL_BIND_GROUP`).

**Motion vectors (choice).** The buffer carries last frame's field (`prev_params`, `prev_sources`);
prepass shaders call `wind_sample_prev` beside `wind_sample`. This is how Bevy handles every other
vertex animation (previous joint matrices for skinning, previous morph weights): a second copy of last
frame's inputs, not "sample again at `t − dt`" (the field is integrated, so last frame can't be
recomputed from a time). Last frame is re-framed at the current origin, so a consumer passes the same
origin-relative position to both and an origin shift adds no false motion.

**Upload cost.** Advance + two `frame()` + pack + buffer write, 32 live sources (worst case), measured
by `cargo test -p sf_wind_bevy upload_cost` (the dev/test profile here is optimized): **0.0025 ms** mean over
2 000 frames (gate 0.02 ms); the `wind` example logs 0.003-0.007 ms per frame live (RTX 4060 laptop).

**Example** `cargo run -p sf_wind_bevy --example wind`: 21×21 CPU-twin arrows (gizmos, 2 m) over a
ground shaded by the GPU `wind_sample` (calm dark blue, mean green, 2× mean yellow); Space radial burst,
V vortex, Up/Down speed, G gustiness; `--screenshot out.png --frames N --burst F`.

**App** (bevyrrowind `exterior.rs`): the weather's `Wind` feeds `WindField.state.wind` (until WE1's
`sf_weather` writes it), the terrain origin feeds `WindOrigin`; fog drift reads the field's mean
velocity; clouds take the field's direction at their own altitude speed (Morrowind `Cloud Speed`).

**Open.** No consumer samples the GPU field in game yet (grass, trees, precipitation: their lanes). Cloud
and fog drift take the mean wind only (no gusts at that scale, by design).

