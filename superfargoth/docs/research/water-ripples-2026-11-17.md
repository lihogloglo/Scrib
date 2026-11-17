# Water ripples: rain, bodies, splashes (research, 2026-11-17)

Research only (no builds run). Owner ask: "Rain should make ripples, people and objects going in the water too."
Scope: (a) rain ripples on water bodies, (b) wakes and rings from characters, creatures, thrown items and boats,
(c) the hook for entry splashes (the particles are PR2, another system). Budget: <= 0.15 ms GPU for all ripples
on the RTX 4060 laptop, 1440p + DLSS Quality. Claims I did not check in a source are marked *(recall)*.

## Summary and recommendation

The spec already names the right design (`specs/water.md` L73, L320, L563): rain ripple normals in the water pass,
plus an iWave-style grid around the camera that actors inject into. Nothing of either is built; the water shader
(`water_material.wesl` ~L200-230) builds its normal only from cascade slopes `s` plus shore waves.

Key finding: **rain ripples on water need no new technique.** `sf_surface_bevy/src/layer.wesl` already has a
procedural, atlas-free ring function, `surface_ripples(xz, time, rain)` (L522; four drop grids switched on by rain
quarter, three trailing rings per drop, footprint fade `surface_ripple_fade`, origin-wrapped hash so it survives
floating-origin shifts), with a CPU twin in `sf_surface::layer` (`ripple_weights`, `ring`). Ported from OpenMW
Clockwork (`rain_ripples.glsl`, commit c5e5a9a6, GPL-3.0, already in THIRD_PARTY_NOTICES). The water pass can
add its slope to `s.xy` for about 0.03-0.06 ms. The spec's "ripple atlas" is not needed.

Do NOT inject raindrops into the dynamic grid (spec L563 says drops inject): thousands of splats per second at
0.125 m texels, and the grid only covers 32 m. Analytic rings are cheaper and cover the whole view.

Staged slices (one deliverable each, smallest first):

| # | Slice | Needs | Expected GPU | A/B switch |
|---|---|---|---|---|
| R1 | Rain rings on water | shared WESL ring module; `WaterRain` input resource | 0.03-0.06 ms (water pixels only) | `--off water_rain` |
| R2 | Dynamic ripple grid: compute + water-pass normals, fed by a test emitter | clipmap body mask; `WaterDisturbances` resource | 0.05-0.08 ms High | `--off ripples` (spec already lists it) |
| R3 | Producers: wading characters/creatures, floaters (boats, items), entry impacts | `WaterQuery::immersion`, floater pontoons, fixed tick | +0.01 ms CPU/GPU for emitters | `water.ripples.emitters` |
| R4 | Splash hook + foam from the grid (optional) | `WaterSplash` message; foam accumulate input | +0.02 ms | `water.ripples.foam` |

R1 and R2 are independent. R3 needs R2. R4 last (PR2 owns the particles).

## Rain ripples (R1)

**What exists.** Puddle rings on ground: `surface_ripples` returns the slope along X/Z at a render XZ; the surface
shader multiplies by `RIPPLE_TILT` (0.5) and a fade. Constants in `sf_surface::layer` (`RIPPLE_CELL_M` 1.6 m,
`RIPPLE_RADIUS` 0.2 cell, `RIPPLE_RATE` 1.2 /s, `RIPPLE_SCALES` 1, 1.25, 0.8, 0.625). Rain intensity: the surface
plugin derives it in `ripple_clock` (`rain_of(frame.surface.wet, storm_wet_rate)`), the weather frame has
`PrecipFrame { kind, intensity }` (`sf_weather/src/frame.rs:50`). The water plugin reads no rain today (grep of
`sf_water_bevy/src/lib.rs`: nothing).

**Plan.**
1. Share the code. The ring functions (`surface_blip`, `surface_ring`, `surface_drops`, `surface_ripples`) read
   constants that come from the surface crate. Move them into one small WESL module both crates import, with the
   constants passed as a struct (or a shared `sf_ripple` constants block). Keep the CPU twin in `sf_surface`.
   Duplicating the shader is the fallback; sharing is better (one twin, one test). The framework crates stay
   game-free (this is generic rain).
2. Feed it. Add a host input `WaterRain { intensity: f32 }` beside `WaterFrame`/`WaterLook` (same adapter pattern:
   the host system writes it from `CurrentWeather`; the water crate never depends on `sf_weather_bevy` or
   `sf_surface_bevy`). Time: the same wrapped animation clock as `ripple_clock` (`Time`, wrapped on whole ring
   cycles), not the water time, so a paused water clock cannot freeze rings.
3. Use it. In `water_material.wesl` add `surface_ripples(in.world_position.xz, t, rain)` (rest-plane XZ, not the
   Lagrangian point: rings are not carried by swell) times a tunable strength to `s.xy` before the normal is built.
   Weight by `surface_ripple_fade(footprint)` (the water pass already computes `footprint`). Beyond the fade
   (about 40 m looking down at 1440p) add a small roughness term instead (rain roughens water at distance, as for
   `lost` slope variance in the cascades). Fade the rings in with shore distance not at all: rain falls on all water.
4. Interiors and shelter. Interior pools: rain 0 (no sky). Under bridges, docks, eaves: OpenMW has this open bug
   (issue 7157, occluded water still rings). We have `sf_surface_bevy/src/shelter.rs` (top-down view of cover,
   used for wet masks and ripples); sample it per pixel in a later pass. First cut: ungated outdoors, logged in
   the ROADMAP as a known leak.
5. Cost. 4 grids x 3 rings x a few ALU ops per water pixel; the existing puddle use is the same math. Water pixels
   are about 30% of a coastal frame; estimate 0.03-0.06 ms. Light rain turns on one grid only (loop `break`s on
   `w <= 0`), so drizzle is cheaper.
6. Extras that cost nothing: raise slope variance and a little foam sparkle at heavy rain; damp the shortest
   cascade a little in rain (rain damps capillary waves). Both are tunables, off at first.

**Look risks (owner checks, not agent-judged).** Ring scale vs the 0.5 m near clipmap texel: normals only, so
no mesh aliasing. Ring slope sign (puddle code adds it as a tilt; confirm direction on water). TAA/DLSS shimmer:
the fade handles it on puddles; same function here.

## Dynamic ripples and wakes (R2, R3)

### Canonical technique

A camera-centred 2D height field solved on the GPU, bodies write into it, the surface shader reads its slope
(and optionally height). Two families:

- **iWave** (Tessendorf, GPG4 2004): convolution kernel (13x13 in the paper) over a height + previous-height
  pair, gives correct dispersion (long waves faster than short) and a natural Kelvin-like wake behind a moving
  obstacle; unconditionally stable. Ran a 128x128 grid at 30+ fps in 2004.
- **Crest "Dynamic Waves"** (MIT): plain wave equation on a multi-resolution LOD stack around the camera, 60 Hz
  by default, damping 0.05, Courant 0.7, inputs from `SphereWaterInteraction` components (several spheres
  approximate a hull), `VelocityOffset` to hide input latency; sits on top of the FFT waves. Cheaper per texel
  (5-point stencil) but non-dispersive, so rings look "too clean"; Crest compensates with damping.

A Bevy project already does iWave with three camera-following cascades, 13x13 kernels fitted to Airy dispersion
(error <= 1.5% over 3-12 texels), bodies as ellipsoid contacts, Kelvin wakes, shore reflection and foam from
breaking: Ladvien/bevy_elements PR 4, 0.29 ms at High with 8 bodies (0.23 ms still water). Licence not visible:
check before reading the source; use only as a cross-check of our numbers.

### Recommendation

iWave kernel (matches the spec, gives wakes for free, stable), one grid, not cascades, first:

- Grid 256x256 at 0.125 m = 32 m (High); 128x128 at 0.25 m on Medium; off on Low (spec table row).
  Texture: RG16F (height, previous height), ping-pong; 2 x 256 KB. A 16F slope texture (RG16F, dh/dx, dh/dz)
  written by the same pass so the water shader does one bilinear fetch.
- Toroidal and camera-centred like the water-level clipmap: scroll by whole texels, zero the strips that wrap.
  Anchor in absolute world XZ (wrapped by the origin grid) so floating-origin shifts do not move ripples.
- Solve at a fixed 60 Hz tick (net-ready: pure presentation, host never needs it, a remote player is just
  another emitter source). Cap 2-4 substeps per frame; skip the solve entirely when no emitters are live and the
  grid has decayed to zero (a "sleep" flag), which is the common dry-land frame.
- Kernel in a workgroup-shared-memory tile (16x16 plus a 6-texel apron): 169 taps x 65k texels = 11M MACs per
  step, order 0.02-0.04 ms on a 4060. Use a precomputed 13x13 weight table in a uniform buffer.
- Damping and a boundary mask: read the water-level clipmap's body id and shore distance. Where there is no
  water (id 0), clamp height to 0 and damp heavily near shore (< 1 m), so waves do not climb the beach and
  ripples do not leak between bodies at different heights. Pools and interiors: same grid, nothing else needed.
- Several bodies at different levels: the grid stores a perturbation relative to each body's rest surface (so
  the same grid serves a river under a bridge and a lake beside it); an emitter writes only if its water sample's
  body matches the texel's body id.

### Coupling to our water

- **Normals only first.** Add `(dh/dx, dh/dz)` of the grid to `s.xy` (scaled by a tunable) next to cascades and shore
  waves. Needs no mesh change, gets SSPR and refraction distortion for free, no motion-vector change. Edge fade:
  smoothstep over the outer 8 m of the grid so the 32 m border has no pop.
- **Displacement later** (spec: High): adding height to vertices needs a mesh finer than the grid (0.125 m) to
  avoid aliasing; the near clipmap ring is coarser. Park it until the clipmap density is measured. If added,
  displace by `h` only (no horizontal term).
- **Foam** (R4): feed `|h| > threshold` or `|grad h|` into the existing foam accumulate for wakes and splashes
  (Crest does foam from dynamic waves; bevy_elements foams breaking waves at ak 0.44).
- Costs on the water pass itself: one bilinear RG16F fetch, a few ALU ops. Included in the 0.05-0.08 ms of
  the spec's "water-level clipmap + dynamic ripples" row.

### Emitters (R3)

One resource, `WaterDisturbances`, a `Vec` rewritten each fixed tick by producers, uploaded to a small
storage buffer (cap 64; nearest to the camera win). Each entry: render XZ centre, radius, signed source
strength, horizontal velocity, kind (`Wade`, `Hull`, `Impact`). The compute pass splats entries as smooth
bumps added to the height before the solve (stamp along the segment from last tick's position when speed x dt
exceeds a texel, so fast movers leave a line, not dots).

| Source | How it is fed | Strength |
|---|---|---|
| Wading person / creature | `WaterQuery::immersion(capsule)` (exists, spec L530): depth, submerged fraction, surface point; one `Wade` entry per leg or one for the capsule | speed x submerged width; zero when standing still (a standing person makes no ring, only motion does; plus a small periodic term for breathing/idle is optional, off) |
| Floating items and boats | the floaters' pontoon spheres (spec L560): each submerged sphere = one `Hull` entry (Crest's SphereWaterInteraction model) | dV/dt of submerged volume + drag wake `k x |v_h| x submerged cross-section`; `VelocityOffset`-style lead of half a tick |
| Thrown item, dropped body, jump in | impact when a body's submerged fraction goes from 0 up above a speed; one-tick `Impact` entry, a negative-centre/positive-ring bump (pressure pulse) | proportional to entry speed x radius^2; clamp |
| Swimmer | same as wade with the torso capsule, larger radius | |

All producers read fixed-tick state; the water crate holds no gameplay state and nothing reads the grid back
(CPU buoyancy keeps using `WaterQuery` and the twin; spec L141 leaves short ripples out of buoyancy on purpose).
Nothing in `superfargoth` names Morrowind: the host/adapter decides which actors emit.

## Splash hook (R4)

The water side emits one message per entry/exit and leaves the look to PR2 (particles) and audio (cues):

`WaterSplash { pos, normal(up), speed, radius, volume, body_id, kind: Enter|Exit|Wade|Rain }`.

Spec L567 already has `splash { size }`; extend it with speed, radius and kind. Source: the same impact detector
as R3 (submerged fraction crosses 0 with speed above a tunable). Rain splash sprites (spec L577) are a different
path: the precipitation system spawns them from rain intensity and a water-or-ground test using the
water-level clipmap (spec L434), not this message. The water side also exposes the surface height at the point
(`WaterQuery::sample`) so particles do not spawn at the wrong level.

## Reuse check

| Item | Verdict | Licence / note |
|---|---|---|
| Bevy 0.20 | Nothing native: no water, no height-field sim; its `water_material` example scrolls normal maps (the only "ripples"). Compute and storage textures are available, so the sim is plain `sf_water_bevy` work | MIT/Apache |
| Our `surface_ripples` (Clockwork `rain_ripples.glsl`) | Reuse for R1 (share the module) | GPL-3.0, notice exists |
| OpenMW `ripplesimulation.cpp` | Not a simulation: a flipbook sprite (cycle 0.3 s) or particle ring emitted when an actor is in water (every update, or each 1.5 s / 10 units moved); no ping-pong solve, no licence header in the file; rain rings are in the shader (`rain()` summed over scales). Nothing to port beyond what R1 has; its emission rule (only when partly submerged) is a good gate | OpenMW GPL-3.0 |
| Crest Dynamic Waves | Reference for settings (damping 0.05, Courant 0.7, 60 Hz, sphere inputs, velocity offset). Unity code, not portable; concepts only | MIT (repo licence checked) |
| bevy_water (Neopallium), bevy-aqua | Analytic/FFT waves, no body-driven ripples found | check before any use; not needed |
| bevy_elements PR 4 (Ladvien) | Closest Bevy prior art (iWave cascades, 0.29 ms). Cross-check numbers; licence unknown | do not copy until checked |
| Unity/Godot ripple addons | Mostly 2D height-field or particle decals; nothing we need | skip |
| iWave paper (Tessendorf 2004) | Our own implementation from the paper; the paper is a published method, no code licence issue | n/a |

No crate fits; our own is justified (nothing in the Bevy ecosystem is a maintained, licence-clear iWave grid
that plugs into our clipmap + FFT + shore field).

## Industry comparison

| Game / engine | Rain on water | Bodies / wakes | Source |
|---|---|---|---|
| Lagarde (Remember Me / Frostbite talks) | Flipbook ring normals layered by rain intensity (4 layers) | n/a | Water drop 2b (cited in our `layer.wesl`) |
| OpenMW | Shader `rain()` rings; flipbook sprite or particle at actors | no solve | OpenMW source (`ripplesimulation.cpp`, `water_fragment.glsl`) |
| Crest | n/a (textures) | Wave-equation LOD stack, sphere inputs, foam | crest docs |
| UE5 Water / Niagara *(recall)* | Ripple normals or Niagara grid | Niagara Grid2D shallow water or ripples to a render target | UE docs, unchecked |
| Tessendorf iWave | n/a | Height grid + kernel; Kelvin wake emerges | the paper |
| Crysis 2/3, Sea of Thieves, Uncharted 4, RDR2, TLOU2, Ghost of Tsushima, BotW/TotK *(recall)* | Mostly ring flipbooks/procedural normals | Mostly camera-local height grids or authored wake meshes and decals; BotW/TotK use ring sprites on footsteps | unchecked: not used for decisions |
| Us (proposed) | Procedural rings (as Lagarde/OpenMW/Clockwork) | iWave grid 32 m, emitters, normals first | this note |

Where we would differ: one grid instead of cascades (so far), ripples as presentation on a fixed tick, one
emitter list for characters and objects. Gap to log in `industry-audit.md`: no wake foam trail or hull cutout
yet (R4 and later).

## Tunables to declare (`specs/tunables.md`; `Guess` allowed)

- `water.rain_ripple.strength` (slope scale, 0-2, start 0.5 like `RIPPLE_TILT`), `water.rain_ripple.fade_near_m`,
  `fade_far_m` (reuse `RIPPLE_FADE_*`), `water.rain_ripple.far_roughness` (0-0.2).
- `water.rain_ripple.min_rain` (intensity below which no grid, 0-0.25).
- `water.ripples.grid_texels` (128/256 per preset), `texel_m` (0.125 / 0.25), `tick_hz` (60), `max_substeps` (4).
- `water.ripples.damping` (per s), `kernel_gain`, `shore_damp_m` (1.0), `edge_fade_m` (8).
- `water.ripples.normal_strength` (0-4), `water.ripples.displace` (off / on, High only, parked).
- `water.ripples.emitters_max` (64), `wade.min_speed` (m/s), `wade.strength`, `hull.drag_k`, `hull.lead_ticks` (0.5),
  `impact.min_speed` (m/s), `impact.strength`, `impact.clamp`.
- `water.splash.min_speed`, `water.splash.cooldown_s`.

## Open points

- Confirm the ring-slope sign and whether `surface_ripples` output is slope or tilt on water (owner visual check
  scene: a still lake in heavy rain, camera 1.5 m above, noon; one in drizzle; look for ring density per intensity,
  no shimmer, no rings under roofs).
- Clipmap vertex density near the camera decides whether displacement (not just normals) is allowed.
- Add `WaterRain` and `WaterDisturbances` to the wiki page `engine/terrain-and-water.md` when built; ledger line
  per slice (`docs/ledger/water.md`); no ripple lines exist there today.

## Sources

- Tessendorf, Interactive Water Surfaces (iWave): https://jtessen.people.clemson.edu/reports/papers_files/Interactive_Water_Surfaces.pdf
- Lagarde, Water drop 2b, Dynamic rain and its effects (2013): https://seblagarde.wordpress.com/2013/01/03/water-drop-2b-dynamic-rain-and-its-effects/
- Lagarde, Water drop 3b, wet surfaces: https://seblagarde.wordpress.com/2013/04/14/water-drop-3b-physically-based-wet-surfaces/
- Crest repo (MIT): https://github.com/wave-harmonic/crest ; waves docs https://crest.readthedocs.io/en/stable/user/waves.html ; sphere inputs https://docs.crest.waveharmonic.com/API/WaveHarmonic.Crest/SphereWaterInteraction.html
- OpenMW `ripplesimulation.cpp`: https://github.com/OpenMW/openmw/blob/master/apps/openmw/mwrender/ripplesimulation.cpp ; rain shader https://gitlab.com/rhtucker/openmw/-/blob/master/files/shaders/water_fragment.glsl ; occlusion issue https://gitlab.com/OpenMW/openmw/-/work_items/7157
- bevy_elements PR 4 (iWave cascades in Bevy): https://github.com/Ladvien/bevy_elements/pull/4
- bevy_water: https://github.com/Neopallium/bevy_water ; bevy-aqua: https://lib.rs/crates/bevy-aqua
- In repo: `superfargoth/crates/sf_surface_bevy/src/layer.wesl` L449-540, `sf_surface/src/layer.rs` L65-76, `sf_surface_bevy/src/lib.rs` L439, `sf_water_bevy/src/shaders/water_material.wesl` ~L200-230, `specs/water.md` L73, L320, L563-570, L618-623.
