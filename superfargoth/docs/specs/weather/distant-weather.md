## Distant weather (WE4)

Research and design, we4r, 2026-10-19 (docs only, nothing built). Owner want: see weather in other regions
from afar (a storm over the Ashlands seen from Balmora, rain shafts under distant clouds). Today `evaluate`
runs once, at the camera's climates, and the clouds threshold one fixed noise map by **uniform** scalars
("Clouds as built", deviation "no compute pass writes one"). WE4 brings the written weather map back.

### How shipped games do it

- **Horizon Zero Dawn** (Schneider and Vos 2015, [pdf](https://advances.realtimerendering.com/s2015/The%20Real-time%20Volumetric%20Cloudscapes%20of%20Horizon%20-%20Zero%20Dawn%20-%20ARTR.pdf),
  slides 38-45): the weather system writes a 2-D map "over our section of world map": R coverage, G
  precipitation, B type. Precipitation turns a texel into cumulonimbus at 70 % coverage and darkens it
  (absorption up), and **the same signal starts the rain effects** at the player: local rain is the map at
  the player, not a separate state. Clouds draw to 35 km; past 15 km the map is biased toward cumulus so
  the horizon stays interesting. Cutscenes swap in painted maps.
- **Horizon Forbidden West** (Schneider, *Nubis Evolved*, SIGGRAPH 2022,
  [pdf](https://advances.realtimerendering.com/s2022/SIGGRAPH2022-Advances-NubisEvolved-NoVideos.pdf);
  *The Real-Time Volumetric Superstorms of Horizon Forbidden West*, GDC 2022,
  [vault](https://www.gdcvault.com/play/1027688/The-Real-Time-Volumetric-Superstorms)): 2-D "Nubis data
  fields" over 16 × 16 km tiles, decompressed vertically by a profile model. **Lightning**: generate a
  position, play a flash animation, add an in-cloud glow
  `pow(1 − d/radius, 12) × height gradient × (1 − saturate(5 × density))` to the light energy, and a
  **stable mask** so the temporal upscaler doesn't smear or reject the flash.
- **UE5 Volumetric Clouds** ([docs](https://dev.epicgames.com/documentation/unreal-engine/volumetric-cloud-material-in-unreal-engine)):
  a world-placed layout texture ("global pattern", scale in km, offset and rotation) with one cloud type per
  channel (A = nimbostratus, used by the storm mode with lightning), plus a mask texture that adds or removes
  cloud in a region. Spatial weather is texture placement, not per-texel state. Third-party UE weather
  (Ultra Dynamic Weather, [Fab](https://www.fab.com/listings/84fda27a-c79f-49c9-8458-82401fb37cfb)) uses
  **weather override volumes**: a local preset inside a volume, blended at its edge (our border band).
- **Microsoft Flight Simulator 2020/2024**: live weather is a gridded model (Meteoblue, METARs blended in)
  sampled periodically and interpolated into volumetric cloud layers, rain and wind
  ([guide](https://simtuts.com/guides/msfs-2024-live-weather-guide),
  [forum study](https://forums.flightsimulator.com/t/investigating-how-msfs-draws-clouds-for-weather/482387)).
  Community sources only; no Asobo talk found. Lesson: a coarse grid interpolated per sample is enough for
  clouds; sharp convective cells need a cell signal on top (ours: the storm-cell channel).
- **RDR2** (Bauer, SIGGRAPH 2019 Advances, [video](https://www.youtube.com/watch?v=IYxtqlD1-Ac)): as noted in
  "Research summary", only the outline is confirmed (one integrated system, storm and lightning timing). In
  play, regional weather (snow in the Grizzlies, storm cells with lightning over the plains) is visible from
  afar; the mechanism is unconfirmed, so we take nothing specific from it.
- **Far Cry 6** (Weick and Zhou, GDC 2022, [vault](https://gdcvault.com/play/1027725/Simulating-Tropical-Weather-in-Far)):
  arbitrary weather-state transitions in a day cycle, rain and lightning as particles; the abstract says
  nothing spatial. No Wildlands or Far Cry 5 weather talk found. Frostbite (Hillaire 2016) uses the same
  weather-texture model as Nubis.
- **Distant rain shafts**: no primary source found (searched 2026-10-19). Two practices exist: (a) extend the
  cloud medium below the base where the map's precipitation is high (a low-albedo column with fall streaks,
  in the same march), which UE users build in the cloud material; (b) billboard or cylinder **curtain
  impostors** under cells (older flight sims; flat when close). Rain as a participating medium is also
  argued in [this thesis](https://studenttheses.uu.nl/handle/20.500.12932/48068). We take (a), inside the
  far-fog march we already have.

**Common thread.** One 2-D map around the player carries coverage, type and precipitation; the near
effects (rain particles, wetness, sky darkening) read **the same map at the player**, so near and far can't
disagree. Lightning is a positioned event that lights the cloud from inside.

### Design

**1. Per-climate evaluation, not per texel.** `evaluate` is linear in the climate weights for every
continuous channel and sums discrete ones by weight (WE1 "Blending"). So the map never evaluates per texel:
evaluate each climate in range **once at weight 1** into a **climate table** (≤ 64 rows: the cloud scalars,
fog density scale, precipitation intensity and look, lightning rate), then blend rows per texel by the
border weights. Morrowind has ~9 Vvardenfell regions + 6 Solstheim + Mournhold: ≤ 16 evaluates ×
0.0006 ms (release) ≈ 0.01 ms per frame. Upload ≤ 4 KB.

**2. Climate raster.** The `weather.climate` bake (WE3's border grid) already stores per cell its climate,
the nearest other climate and the distance to that border. WE4 makes it the single source: a per-worldspace
raster of `(own, neighbor, distance)`, uploaded once (Morrowind: ~64 × 64 cells, 12 KB). Huge worldspaces
use a toroidal window into it, re-blitted when the map recenters.

**3. Weather map (GPU).** World-anchored, toroidal (like the cloud shadow map), recentered in 1/8-map
steps on the camera, addressed with the origin phase (`CloudsOrigin`, f64 on the CPU), so floating-origin
shifts move nothing.
- Size: 256² texels. Texel ≤ border band / 2, so the band spans ≥ 4 texels: real scale 128 m (32 km,
  covers a 16 km cloud reach), Morrowind 64 m (16 km: all of Vvardenfell from anywhere on it, Red
  Mountain to the sea). Past the map: clamp to the edge texels (the out-of-range rule).
- Textures: A `Rgba16Float` coverage, type, wetness, storm; B `Rgba16Float` base, thickness, density,
  erosion; C `Rgba8Unorm` cirrus, precipitation intensity, fog scale, precipitation look. ~1.3 MB.
- A **resolve compute pass** writes them from raster + table: per texel, `border_blend` weights (the
  `sf_weather` function, WGSL twin) over the two table rows. Runs when the table changed past an epsilon or
  the map recentered (at most every frame); 65 k threads.
- **Uniform fast path**: when every row in range is the same climate state (the common clear day), a
  flag keeps today's uniform scalars and skips the map fetches.

**4. Clouds read the map.** `weather_sample(p)` takes coverage/type/wetness/storm (and base/thickness)
from one bilinear fetch of A (and B) at `p.xz` instead of uniforms; `sf_atmos::weather_at` keeps its math
(the map sample replaces the scalars), so the CPU twin and gameplay "is this in cloud" still agree. The
shell becomes the **envelope** of the map's base/top (a neighbour's cumulonimbus is taller than our fair
deck); `h` is normalized by the local base/thickness and the step comes from the local thickness, so a thin
local deck doesn't pay for the distant tower. The **cloud shadow map** samples the same map (shadows match
the deck), and storm cells keep their own cell channel (MSFS's lesson). Adapter anchors (open question 3,
the Red Mountain plume) become a coverage boost written in the resolve. Clouds drift with the camera's
wind through **fixed** regional weather: a cell drifting out of a stormy region thins over the band (as
Morrowind's per-region weather implies; moving storm systems are owner question 2).

**5. Local = map at the camera.** The camera's `WeatherSite` weights come from the same raster sample and
`border_blend` the GPU uses, on the CPU (`WeatherMap::site_at(pos)`), so `CurrentWeather.frame` (sky
dimming, wind, near rain, cues, `visibility_m`) equals the map at the camera by construction. Crossing a
border is the band crossfade WE3 already plans; nothing pops because there is one code path. Test: the map
texel at the camera equals `evaluate(site)` for every continuous channel along a 2 m border walk.

**6. Precipitation shafts.** In `clouds_trace`, a short march (8-12 steps, quarter res, temporal like the
banks) over the ray's part **below the local cloud base**, out to the far-fog reach (6 km; all of
Vvardenfell), only when the map's per-frame precipitation maximum is above zero. Medium: map C
precipitation at the sample, sheared by the wind with height (slanted curtains), × the vertical profile
(full under the base; a `virga` fraction per preset fades it before the ground) × fall streaks (the detail
volume stretched vertically, scrolled at `PrecipKind.fall_speed`); extinction = intensity ×
`PrecipKind.extinction` (the same number `visibility_m` uses, so a shaft is as opaque as standing in it).
Lit by the key light × cloud shadow map (dark under the storm, bright at its sunlit edge) + sky fill; tint
by look (rain grey-blue, snow white, ash brown). **Hand-off**: weight `ramp(32 m, 128 m, t)`; inside it the
near particle boxes (to 64 m) and the camera's uniform precipitation extinction take over. At the camera
both describe the same intensity, so walking into a storm adds particles in front of a curtain that was
already there.

**7. Far fog by location.** The analytic height fog stays the camera's (closed form, per pixel). The
existing 16-step bank march adds the **difference** `(fog_scale(x) − fog_scale(camera))` × height-fog
density, clamped so the total stays ≥ 0: the Bitter Coast's fog reads from a dry ridge, and the term is
zero at the camera, so near fog is unchanged.

**8. Distant lightning (pure, replicated).** Strikes become **world-anchored** (closes WE1's "flash
position is camera-relative"). Per climate, the Poisson schedule runs at weight 1 with the authored
`per_hour` defined as the rate seen within a reference radius R (default 3 km real, scaled by the adapter),
so the density per km² is `per_hour / (π R²)` and a player inside the region sees the authored rate. Strike
`k` takes a cell from the climate's raster cells by `hash(seed, climate, k)`, jittered, kept if the map's
storm × coverage there is > 0.5 (4 tries, else dropped). Everything is a function of (seed, overrides,
clock, raster), so co-op peers see the same flash at the same place without messages. Render: up to 4
active flashes in a uniform; the trace adds the Nubis 2022 glow term; the resolve skips the history clamp
inside a flash's screen bound (the stable mask) so 0.1-0.3 s flashes neither smear nor vanish; a sky and
ambient bump falls off with distance; thunder delays by distance / 343 m/s and is culled past ~3 R.

**Morrowind adapter notes.** Regions are irregular cell sets: the adapter rasterizes `CELL` region names
(WE2's `ClimateSource`) per 117 m cell into the raster, with a distance transform for the border distance.
Cells without a region (most sea) take the **nearest region** by dilation (owner question 1), so the
coast's storm continues out to sea instead of stopping at a clear ocean. `ChangeWeather` / `ModRegion`
already act per climate, so a scripted storm in a neighbouring region shows from afar for free. Debug: BRP
`weather.force {region, weather}` and `--force-region <id>=<weather>` for the WE-3 check.

### Cost (estimates; GPU ms at 1440p, RTX 4060 Laptop, High)

| Part | Estimate | Note |
|---|---|---|
| CPU: climate table (≤ 16 evaluates) + upload | ≤ 0.02 | inside the 0.05 CPU gate |
| Resolve pass (256², on change) | ≤ 0.01 | inside "cloud shadow map + weather map" 0.05 |
| Trace: 1-2 map fetches per density sample | +0.03-0.08 | 0 on the uniform fast path |
| Trace: envelope shell toward a taller neighbour | +0.05-0.10 | views toward the storm only |
| Shadow map reads the map | < 0.01 | time-sliced |
| Shafts (8-12 steps, rain in view) | +0.03-0.06 | skipped with no precipitation in the map |
| Lightning glow (≤ 4 flashes) | ≤ 0.02 | only while a flash is active |
| **Total, storm in view** | **≈ +0.15-0.25** | **gate +0.15 on High; ≤ 0.02 uniform** |

Clouds on fair already sit at the 0.9 ms High line (S1 cost), so WE4 must be paid for: the fast path
covers uniform days; the levers are a per-ray map lookup on Low/Medium (sample the map at 3 points along
the in-layer segment and lerp), A-only fetches (B every 8 samples), and 8 shaft steps.

### Slices (1-2 steps each; one build agent each)

- **WE4a** (built, we4a 2026-10-19: "WE4a as built" below) `sf_weather::WeatherMap` (Bevy-free): climate raster type over WE3's border grid, per-climate
  table (`evaluate` at weight 1), `site_at(pos)` and `sample(pos)` (CPU twin of the resolve).
  *Verify:* tests: map at the camera = `evaluate(site)` (1e-5, continuous channels) on a 2 m border walk;
  same seed → same table; table cost ≤ 0.02 ms for 16 climates (release). **Needs WE3's border grid (we3b2).**
- **WE4b** `sf_atmos_bevy` weather map: textures, resolve pass, clouds + shadow map read it, envelope
  shell, uniform fast path, plume anchor hook. *Verify:* `clouds` example `--split clear,storm` (two
  climates side by side) screenshot from the clear side; GPU-timer A/B uniform vs split (gate above).
- **WE4c** App: region raster (dilated sea), `site_at` replaces the camera lookup, BRP/flag force per
  region, scene `we3-ridge`, `docs/visual-checks.md` **WE-3**. *Verify (WE-3):* from the ridge above
  Balmora at 14:00, own region Clear, forced Thunderstorm in the Ascadian Isles and Ashstorm in the
  Ashlands: a dark deck over each neighbour, clear sky overhead, no seam at the border band; walk across:
  no pop (screenshots at 400/200/100/0 m).
- **WE4d** (built, we4d 2026-10-19: "WE4d as built" below) Shafts + far fog by location. **Needs WE3 precipitation (we3b2)** for the near hand-off and
  `PrecipKind` looks. *Verify:* WE-3 shots show slanted curtains under the storm, ash brown in the Ashlands;
  the approach sequence shows no double rain at the hand-off; FG-2 Bitter Coast fog seen from a dry ridge;
  shafts ≤ 0.06 ms.
- **WE4e** World-anchored lightning + glow + stable mask. **Needs WE3 lightning/thunder (we3b2).**
  *Verify:* tests: two sites far apart get identical world strikes for one seed; strike rate inside a
  region within 4σ of `per_hour`; WE-3 at 23:00: flashes inside the distant deck, thunder delayed by
  distance.

WE4a-c need only WE3's border grid; d and e wait for we3b2's precipitation and lightning. When WE4b lands,
log the reversal of "Clouds as built"'s "no written weather map" deviation in ARCHITECTURE §10.

### Owner questions (WE4)

1. **Sea weather**: regionless cells (most sea) take the nearest region's weather (recommended), or a
   fixed Clear sea?
2. **Moving storms**: keep weather fixed to regions (Morrowind's model; clouds thin as they drift over a
   border; recommended), or add advected storm cells that cross borders (more realistic, no Morrowind
   basis, later)?
3. **Cost tier**: WE4 adds ≈ 0.15-0.25 ms when a storm is in view, with clouds already at the High line.
   Accept on High (trim elsewhere), or use the per-ray map lookup (cheaper, softer region edges) on High too?

### WE4 owner decisions (2026-10-19)

- **Regions first, moving storms later:** WE4 shows each region's own weather from afar, blended at borders; storm
  cells drifting across borders are a later slice on the same map.
- **Regionless sea cells take the nearest region's weather** (blended), not a default clear sky.
- **Cost:** full weather map on High and Ultra; Medium and Low use the cheaper per-ray lookup (coarser, no rain shafts).

See [history](history.md#we4a-as-built-we4a-2026-10-19-sf_weatherweathermap-cpu-side) (WE4a-e as built).

