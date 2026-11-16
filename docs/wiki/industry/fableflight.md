# FableFlight: long view, clouds, streaming (code study)

**Most of its view distance comes from the setting and the data, not the engine.** FableFlight
([github.com/Rolandjg/fableflight](https://github.com/Rolandjg/fableflight), MIT, raw wgpu, studied at `c5f30f5`
2026-11-15) is a flight sim: the camera sits high with little near content, and the world is streamed from data
that already comes as LOD pyramids (DEM, satellite imagery, OSM, Google 3D Tiles within 9 km). Past ~4 km the
world *is* the satellite photo on the terrain. Its engine is simpler than ours (CPU culling only, no occlusion
culling, no shadow caching, no upscaler, no GPU timers so no measured numbers, no terrain LOD morph). What it
teaches is which choices make distance nearly free. MIT is GPL-compatible: a port cites file + commit and gets a
`THIRD_PARTY_NOTICES.md` entry. Paths below are in that repo.

## Takeaways for us, ranked
| # | Idea | Their code | Us now |
|---|---|---|---|
| 1 | **Bake a top-down "satellite" albedo of our own world** (statics, forest canopy) into the far terrain colour: past the last impostor ring, towns and forests stay readable for one fetch (MSFS, Far Cry do the same) | `sources.rs:219`, `terrain.wgsl:40-56` | `terrain.macro` 257² is terrain-only ([terrain](../engine/terrain-and-water.md)) |
| 2 | Share-the-cost refresh: redraw a map in slices over N frames, cross-fade old to new, aim it at the future sun | `render.rs:168-300`, `csmix.wgsl` | env map refilters ~0.7 ms on 1 frame in 8, not spread ([sky](../engine/sky-atmosphere-weather.md)) |
| 3 | God rays at every distance: aerial perspective is a per-pixel 12-step jittered march (quadratic spacing) that reads the cloud shadow map | `composite.wgsl:593-598`, `common.wgsl:238-246` | froxel shafts near the camera only, far fog analytic; FG-10 open |
| 4 | Clouds to 160 km on spherical shells, step `clamp(0.016·t, 40, 2500)` m, x4 stretch in clear air | `clouds.wgsl:163-225` | `CloudLook::far` 50 km, fades over the last quarter (`sf_atmos/src/settings.rs:273`) |
| 5 | Cloud shadow map stores optical depth in 3 height bands + a closed-deck flag, two cascades (±40 km, ±200 km), texel-snapped centres | `clouds.wgsl:465-521`, `common.wgsl:581-645` | one 1024² toroidal map, ground only |
| 6 | LOD by screen-space error, not fixed distances (3D Tiles rule `error·px/dist > sse_px`) | `tiles3d.rs:1221,1276` | far field handoffs 400/2000/8000 m x preset bias (`sf_lod/src/levels.rs:72`) |
| 7 | Parent draws until **all** children are ready; cross-fade one level at a time (never 3 overlapping) | `world.rs:1196-1224`, `tiles3d.rs:1244,1562` | check against `sf_lod_bevy` `residency_pass` |
| 8 | Per-instance ±12% LOD distance jitter so a forest changes a tree at a time, not along a line | `trees.rs:24-25` | dithered ranges, no per-instance jitter found |
| 9 | Per-cascade caster sets: far cascade = stand-ins + tall buildings, mid = one tree model, undergrowth near only | `render.rs:2576,2630,2736` | compare with our cascade caster set ([shadows](../engine/shadows.md)) |
| 10 | Upload byte cap per frame (24 MiB, 8 MiB chunks) + small allocator blocks: a non-ReBAR staging slow path (~9 MB/s) gave ~1 fps over cities | `world.rs:83-89`, `render.rs:1123-1131` | check Bevy `WgpuSettings::memory_hints` on the 4060 laptop |
| 11 | Town glow on clouds and haze: night-light map blurred at 4 heights, lights cloud undersides (eye candy, Phase 4) | `skyglow.rs`, `common.wgsl:271-346` | none |
| 12 | Storm cells as a CPU list (nearest 16 on a jittered 40 km grid); a per-ray mask marches to the storm ceiling only near a cell | `weather.rs:147-197`, `common.wgsl:402` | storm from the weather map scalar; WE-10 open |
| Later | Real planet: f64 positions, horizon cull `acos(R/(R+alt))+acos(R/(R+9000))`. At 100 km a flat world is ~785 m off | `geo.rs:6`, `world.rs:1166` | flat world on `big_space` |

## Clouds and atmosphere
- **Clouds** (`clouds.wgsl`, `common.wgsl:437-565`): one marched layer plus a 6-step alto slab, an analytic cirrus
  sheet at 8.8 km and storm towers to ~11 km. No weather texture: coverage, type and top come from one 128³ noise
  texture at 1700/173/29 km scales. Half-res; texels alternate nearest/farthest depth of their 2x2 block, store scene
  distance, joint-bilateral upsample. History reprojected through the opacity-weighted mean cloud distance, clamped
  to same-surface neighbours (mean ± 1.6 σ), 10% current. Same structure as ours.
- **Lighting:** 6-7 light steps (36 m x1.85), 4 multi-scatter octaves (a 0.55, b 0.38, c 0.5) plus a diffusion tail,
  4-lobe HG phase (0.86, 0.5, -0.25, 0.97 diffraction spike), post-march silver rim `tau·e^-tau`, sun colour from
  the transmittance LUT, own aerial perspective to the cloud's mean distance.
- **Sky:** Hillaire 2020: transmittance 256x64 and multi-scatter 32² rebaked only on a haze change, sky-view
  256x128 every frame. No froxel aerial-perspective volume: the per-pixel march (#3) does it, plus a 1.35 "artistic
  lift". Weather from METAR: visibility -> haze by inverting Koschmieder, 14 s exponential easing.

## Streaming and far world
- **Terrain:** slippy-map quadtree from z2, 64x64 tiles, split when `width/dist > lod_k` (0.95..0.5 by preset),
  skirts, no morph (pops). Near-camera tessellation with no tessellation stage: each edge snaps to a coarser lattice
  from its own midpoint (`terrain.wgsl:537-545`). Detail: 8 + 5 hex-tiled photo-scan layers tinted by the satellite
  colour, faded by 1 km. Reversed-Z, infinite far, f64 camera-relative (Bevy already gives us the depth part).
- **Queues:** 14 workers, wanted list rebuilt and sorted every frame (`dist/width`, 3D tiles `1/sse`), unstarted
  jobs drained and requeued, an epoch drops stale results; in-flight 28 (tiles: 64 queued); LRU by last draw.
- **Vegetation and buildings:** trees 6 LODs at 260/600/1200/2200 m, impostors from 120 m (64 hemi-octahedral views
  x 128 px; ours 8x8 x 32 px), tree reach 1.4-4.5 km by preset. Buildings dissolve to a box stand-in at 1-1.3 km
  (x1.5-3 for towers).
- **3D Tiles:** Google photogrammetry de-lit per texel and relit with the game sun (`relight.rs`); a top-down
  coverage mask lets it replace the engine's own world, dithered over a 1.6 km band.

## Frame and performance
One encoder, single-threaded render; forward shading; 4x MSAA G-buffer + TAA (Halton 8, YCoCg variance clip,
separate rigid reprojection for the aircraft); GTAO full-res; 3 sun cascades ±260/1500/8500 m, no caching; bloom,
AgX. Presets scale render scale (0.7-1.5), shadow map (1024-6144), cloud step (1.8x-0.7x), tree reach and LOD k
(`graphics.rs`). Only CPU timings (`--fps-log`).

## Don't copy
Its terrain LOD (no morph, skirts), MSAA + TAA together, forward lighting without clustering, CPU-only culling, and
eyeballed constants (1.35 haze lift, `MS_GAIN` 2.6): we calibrate from our own measurements.
