# Weather adapter (Morrowind → sf_weather, sf_wind, sf_atmos, sf_surface)

Status: draft (2026-10-11), spec only
Crates: `mw_weather` (new, Bevy-free adapter: records → presets/climates, hooks), app wiring in
`apps/bevyrrowind`; near-term fixes touch `mw_assets`, `mw_bake` and the app (weather.md
"Near-term visual fixes")
Framework spec: `../../../superfargoth/docs/specs/weather.md`

## Goal

Express Morrowind's weather in the framework's terms, so that the original data drives the modern
system and mods (plugins, ini edits, our record mods) keep working:

- regions (`REGN`) → `Climate` records (weather chances, change interval, region sounds);
- the ten weather types and the `[Weather*]` settings in `Morrowind.ini` → `WeatherPreset` records;
- ash storms, blight storms and blizzards, including their storm direction and gameplay side effects;
- MWScript weather functions → `sf_weather` overrides;
- sound ids → cues; sky textures → optional reference only.

Not a 1:1 port (rule 6). Morrowind fades a 2D cloud texture and flat fog colors. We take its *intent*
(which weather where, how often, how it sounds, what it does to gameplay) and render it with
volumetric clouds, froxel fog and surface state. The original colors are calibration references, not
outputs.

## Canonical technique

The framework side is weather.md (RDR2/UE-style integrated atmosphere, a weather state machine with
blended presets as records). The adapter follows the original engine's data semantics. OpenMW
(GPL, fine in this repo) is the reference for behavior: `apps/openmw/mwworld/weather.cpp` (weather
selection per region, transition timing, storm direction, thunder), `components/esm3/loadregn.cpp`
(REGN layout), and the `fallback=Weather_*` keys `openmw-iniimporter` produces from `Morrowind.ini`
(the full key list). No crate covers this; `tes3` parses `REGN`.

## Data model

### Weather types → presets

Morrowind has ten weather types, indexed in this order in `REGN`'s weather chances: Clear, Cloudy,
Foggy, Overcast, Rain, Thunder, Ash, Blight, then Snow and Blizzard (Bloodmoon). Each becomes one
`WeatherPreset` record, `mw_weather/<type>`, generated in a **synthetic mod layer** ("Morrowind.ini
weather") that sits right after the master files. Our own record mods and other mods patch it field by
field, as with any record (rule 3).

Mapping from a `[Weather <Type>]` section (keys as `openmw-iniimporter` lists them):

| Ini keys | Preset fields | How |
|---|---|---|
| `Clouds Maximum Percent`, `Cloud Texture` | `clouds` genus preset (coverage, type, layer heights, cirrus) | Weather → genus (`sf_atmos::CloudPreset`, S1 2026-10-13): Clear → clear (a few humilis + cirrus trace), Cloudy → fair cumulus, Foggy → altostratus, Overcast/Rain/Snow/Blizzard → overcast, Thunderstorm → storm (cumulonimbus cells), Ashstorm/Blight → stratocumulus. The texture name picks the genus; as built in `exterior.rs` `CLOUD_CHOICES` until the weather adapter reads the ini. |
| `Land Fog Day Depth`, `Land Fog Night Depth` | `fog.height_fog.density` KeyCurve (day/night) | Morrowind's fog depth is a fraction of view distance. Converted through Koschmieder to an extinction at ground level (visibility ≈ 3.9/σ), calibrated on the Foggy and Ash screenshots. |
| `Fog * Color`, `Sky * Color`, `Ambient * Color`, `Sun * Color` (sunrise/day/sunset/night) | none directly; `sky.tint` curves only if a mod opts in | Calibration reference: with the physical atmosphere they come out of the medium. The adapter keeps them in a `MwWeatherLook` record so a "classic look" mod can blend them in as tints. |
| `Sun Disc Sunset Color`, `Glare View` | `SunDiskLook::glare` (per weather, M6 blend) | `Glare View` (1 clear and fair, 0.25 cloudy, 0 overcast and worse) scales the base glare once weather presets blend (M6); until then the clear value is used and cloud transmittance hides the glare. The drawn disk is 2.5° (`--sun-size`; physical 0.53°). `Sun Disc Sunset Color` is calibration only (the atmosphere reddens the disk). |
| `Wind Speed` | `wind.speed` | Morrowind's value is unitless (≈ 0.1-0.9). Mapped linearly to 1-15 m/s at 10 m above ground, then tuned on grass and tree motion. |
| `Cloud Speed` | `clouds.wind` | Cloud-altitude wind = preset direction × `Cloud Speed` × 10 m/s (`CLOUD_SPEED_MPS`; ini defaults Clear 1.25, Cloudy 2, Foggy 1.25, Overcast 1.5, Rain 2, Thunderstorm 3, Ashstorm 7, Blight 9, Snow 1.5, Blizzard 7.5). Real-world speeds read frozen beside the 30x game clock. Shapes also evolve (`CloudLook::evolve`). |
| `Transition Delta` | `transition` | Morrowind's per-frame blend rate becomes a duration in game hours (one weather change ≈ 1-2 game hours; OpenMW's interpretation is the reference) plus channel lead/lag defaults. |
| `Using Precip`, `Rain Diameter`, `Rain Height Min/Max`, `Rain Threshold`, `Rain Entrance Speed`, `Max Raindrops` | `precipitation` (kind `mw_precip/rain`, intensity) | `Max Raindrops` sets intensity relative to Thunderstorm. `Rain Threshold` sets when rain starts during a transition (lag). Diameter and heights don't apply (our volumes are camera-wrapped). |
| `Snow *` (Bloodmoon: diameter, heights, entrance speed, `Max Snowflakes`, threshold) | `precipitation` (kind `mw_precip/snow`) | Same as rain. |
| `Thunder Sound ID 0..3`, `Thunder Frequency`, `Thunder Threshold`, `Flash Decrement` | `lightning` | Frequency becomes strikes per minute. Flash decrement sets the flash decay. Sounds become the thunder cue set. |
| `Ambient Loop Sound ID`, `Rain Loop Sound ID` | `cues` | `SOUN` record refs. `None` means no cue. |
| `Disease Chance` (Blight) | `tags: [(mw_tag/blight_exposure, chance)]` | The rules plugin reads the tag (below). |
| `Storm Threshold` (Ash, Blight, Blizzard) | `transition` (storm particles and wind lag) | When the storm look starts during a transition. |

Global `[Weather]` keys:

| Ini keys | Target |
|---|---|
| `Hours Between Weather Changes` | `Climate.change_interval_hours` default for every region |
| `Sunrise/Sunset Time`, `Sunrise/Sunset Duration` | already in `SkySettings` (sky-lighting.md); used to calibrate, not to override the ephemeris |
| `Precip Gravity`, `Snow Gravity Scale` | `PrecipKind.fall_speed` |
| `Rain Ripples`, `Rain Ripple Radius/Scale/Speed`, `Snow Ripples` | ripple atlas settings in `PrecipKind` (puddles and water) |
| `Minimum/Maximum Time Between Environmental Sounds` | region sound cue timing (below) |
| `Stars *` fading, `Timescale Clouds` | sky-lighting `StarField::visibility` (default −4° → −14° sun elevation ≈ Morrowind's 1 h post-sunset start + 2 h fade at Vvardenfell's summer latitude) and `clouds.wind_factor` behavior |
| `[Moons]` (Masser/Secunda sizes, phases, colors) | sky-lighting `Moon`s (not weather), app `night_sky.rs` | `Size` × 0.1° (Masser 9.4°, Secunda 4°, quad size; the disk is ~85 % of it); `Daily Increment` (h) → `Circuit { circuit_days: 1 + inc/24 }` (rises that much later each day); `Axis Offset` → path tilt; 8 phases × 3 days (both start full, observed), textures `tx_{masser,secunda}_{new,one_wax,half_wax,three_wax,full,three_wan,half_wan,one_wan}.dds`. Not mapped: `Fade In/Out Start/Finish` hours and `Fade Start/End Angle` (the day sky's brightness hides the moons physically), `Speed`, `Moon Shadow Early Fade Angle`, `Script Color` (blood moon, M5 scripting). Full illuminance 0.8 + 0.2 lx (the old single 1 lx moon). `tx_mooncircle_*` (Morrowind's star blockers) are replaced by the analytic `occluder` disk. |

Where the ini comes from: `mw_install` already finds the install. The adapter reads `Morrowind.ini`
from it; OpenMW installs carry the same values as `fallback=` lines in `openmw.cfg`. Missing keys
fall back to vanilla values compiled into `mw_weather` (from the vanilla ini; numbers are data, not
code). The ini is hashed into the climate/preset fingerprint, so editing it re-runs only the cheap
`weather.climate` bake step.

### Precipitation kinds

`mw_precip/rain`, `mw_precip/snow` (Bloodmoon) and `mw_precip/ash` (dark grey-brown dust, low fall
speed, strong wind drift, accumulates `ash`). `mw_precip/blight` is ash tinted red with a reddish
fog albedo. Blizzard is snow at high intensity + high wind + snow mist in the fog. Textures are ours
(generated or from an asset mod), not Morrowind's `tx_raindrop`/`tx_ash` sprites, which are
too small for streaks. A "classic" mod can point `PrecipKind.texture` at them.

### Regions → climates

Each `REGN` record becomes `Climate mw_region/<id>`:
- `weights` = its weather chances (u8 percentages, 8 in Morrowind.esm and 10 with Bloodmoon), mapped to
  the presets above. A zero chance means the type never occurs there.
- `change_interval_hours` = the global `Hours Between Weather Changes` (± a jitter we add, so regions
  don't all change in the same frame).
- **Region sounds** (`REGN` sound list with chances) become ambient cues on the climate, played at the
  ini's environmental sound interval when the weather has no ambient loop of its own.
- `REGN`'s sleep creature list is not weather (it goes to the rest system later).

`ClimateSource::climate_at(ws, pos)`: the exterior cell's region name (`CELL` region field). Cells
without a region use the worldspace default (Clear only). Interiors return none. Interiors with the
"behave like exterior" flag (Mournhold's plaza, Tribunal) get the region their `CELL` names, or Clear.
`weather.climate` rasterizes this per cell (8192 units ≈ 117 m), so the border band (weather.md,
256 m) blends over about two cells.

### Ash and blight storms, blizzards

- **Storm direction.** In the original game (and OpenMW's `weather.cpp`), ash and blight storms blow
  *away from Red Mountain*. The adapter sets each Red-Mountain-adjacent ash/blight climate's
  `storm_source = WindSource::Radial { center: Red Mountain summit, strength, falloff }`. When an ash
  or blight preset is active, `sf_weather` adds that source, so the global wind turns radial around
  the mountain, and ash streaks, fog banks, clouds and grass all agree. Blizzards (Solstheim) use the
  preset's wind direction (no radial source).
- **Red Mountain region**: its `REGN` weights decide the mix (ash and blight dominate in vanilla data).
  After the main quest, scripts change the region's weights (`ModRegion`, below), and the climate follows
  through the runtime overlay. The plume over the summit is a coverage anchor in the weather map
  (weather.md "Driving from weather"), strongest during ash and blight weather.
- **Ash accumulation**: ash presets have `surface.ash_rate > 0`. Ash settles on up-facing surfaces in
  exposed areas and blows off at high wind. Ashlands terrain layers already look ashy. The surface
  channel adds fresh grey dust on rooftops and rocks during and after storms.
- **Gameplay tags** (`mw_rules`, M10): `blight_exposure` with the ini's `Disease Chance`. The rules
  plugin rolls blight diseases for exposed actors (not occluded by the precipitation map, not in an
  interior), at an interval matching the original. `visibility_m` feeds AI perception.
  Morrowind NPCs shield their faces during storms (an animation hook later: `storm_intensity` in
  `WeatherFrame`).

### Scripting (MWScript → overrides)

| MWScript | `sf_weather` call |
|---|---|
| `ChangeWeather "<region>" <type>` | `force(mw_region/<id>, mw_weather/<type>, transition: preset default)` |
| `ModRegion "<region>" c0 c1 … c9` | runtime overlay patch of `Climate.weights` (saved) |
| `GetCurrentWeather` | current dominant preset of the player's climate → type index |
| `GetWindSpeed` | `WindState` speed at the player, converted back to Morrowind units |

They go through the MWScript → Luau layer (M5), which calls the curated weather API (`weather.force`,
`weather.set_climate_weights`, `weather.current`, `weather.wind_at`). OpenMW-Lua's `core.weather`
(where present) maps to the same calls.

### Sounds

Cues carry `SOUN` record refs: weather ambient loops (rain, heavy rain, ash storm, blight, blizzard),
thunder sets (delayed by distance), region ambient sounds. The audio system (later) owns playback.
Interiors attenuate exterior loops via the occlusion/environment volumes (M8). Footsteps pick a wet
variant when the surface wetness under the foot exceeds a threshold (`sf_surface` query).

### Sky textures (optional reference only)

`Tx_Sky_*.dds` cloud textures and `tx_sun_*` are not rendered by default (the moons' `tx_masser_*`/`tx_secunda_*` phase textures are, see `[Moons]`; `tx_stars_*` dome textures are not: the framework star field replaces them). Uses:
- calibration: the original cloud texture per weather tells which cloud type to default to;
- a "classic sky" mod can map them onto the high 2D cirrus layer (weather.md "Geometry"), which takes
  a texture.
Replacer mods for these textures therefore have no effect unless that mod is enabled. The mod report
says so.

### Interior fog

`CELL` ambient data (fog color and fog density) becomes a fog-only environment preset for that
interior: height-independent froxel density + color, or Bevy `DistanceFog` until environment volumes
exist (M8). Underwater cells and the "has water" interiors keep their water fog (water spec).

### Morrowind-specific calibration

- **Scale**: Vvardenfell is compressed (Red Mountain stands a few hundred meters above sea level).
  Cloud bases default to 600-900 m above sea level for low cumulus, 400 m for ash-storm decks, high
  cirrus at 5 km. They are tuned on the screenshot set, not taken from Earth defaults (weather.md open
  question 8).
- **Haze**: Morrowind's look is hazy: weather-driven haze, and the dust term in ash weathers (below).
  The near-term N1/N2 steps calibrate this before `mw_weather` exists.
- **Haze and dust** (M6 F1, as built; app scaffold table `exterior.rs` `SKY_*` until `mw_weather` presets carry
  `SkyTerms`): haze (1 = clean Earth) / dust / dust tint per weather: clear 20, cloudy 24 (and the cloud genera; overcast and storm genera 32),
  overcast 32, rain 48, thunderstorm 48, foggy 60, snow 48, blizzard 80; ash storm haze 24 + dust 4 (grey-brown);
  blight the same with dust absorption tint (0.6, 1.4, 1.9) (rusty red). Haze follows a morning curve (×1.5 at 6:30,
  ramps 4:00-6:30 and 6:30-10:00); dust does not. `--haze`/`--dust` fix them. Ashlands-region dust on non-storm days
  and Bitter Coast mornings by region wait for climates (WE2). Calibrated for 1-5 km views (weather.md N1 "As built").
- **Wind and trees**: `wind_profile` marks `meshes/f/flora_*` (the same rule as `mw_bake::far::is_flora`)
  as foliage. Kelp and bushes get leaf-level flutter only. Emperor parasols are stiff (trunk bend only).

## Interfaces

```rust
// mw_weather (Bevy-free)
pub struct MwWeather { /* presets, climates, sound map, ini fingerprint */ }
impl MwWeather {
    pub fn build(db: &RecordDb, ini: Option<&MorrowindIni>) -> Self;
    pub fn records(&self) -> impl Iterator<Item = WeatherRecord>; // presets, climates, precip kinds
}
impl ClimateSource for MwClimates { fn climate_at(&self, ws: &WorldspaceId, pos: WorldPos) -> Option<RecordRef>; }
impl WindProfileSource for MwFlora { fn wind_profile(&self, model: &str, s: &ModelSummary) -> Option<WindProfile>; }
```

App: `WeatherPlugin` from `sf_weather_bevy` with `MwWeather`'s records loaded into `sf_records` (as a
mod layer), `MwClimates` as the climate source, `--weather <type>` and `--time` flags for screenshots.

## Verification

- `cargo test -p mw_weather`: REGN weights map to the right presets (index order, Bloodmoon's two
  extra); ini parsing with missing keys falls back to vanilla; `ChangeWeather`/`ModRegion` produce the
  expected overrides; storm source points away from Red Mountain from four sample positions; the
  fingerprint changes when the ini changes.
- Screenshots (weather.md "Verification"): Bitter Coast rainstorm (Seyda Neen), ash storm near
  Ghostgate, sunrise god rays in Balmora, foggy Bitter Coast morning. Plus a region crossing
  (Ascadian Isles → Ashlands) as a time-lapse to show the border blend.
- A forecast dump (`bevyrrowind weather forecast <region> --days 3`): sequence frequencies match
  `REGN` weights within sampling error over 1,000 changes.

## Industry comparison

J9 (superfargoth `industry-audit.md`), 2026-10-12, docs only (`.sessions/j9rest.md`).
The rendering techniques are compared in superfargoth `weather.md`. Data
semantics: Bethesda's engines (Morrowind's `[Weather*]` INI keys and REGN
chances; later Creation weather records with per-weather fog, colors,
sounds, precipitation, and region weather lists) and OpenMW (reimplements
the vanilla selection, transitions and storms from the INI fallback keys)
all keep weather as data per region with timed transitions. Ours: the same
data mapped to presets and climates (**match**), rendered with a modern
integrated atmosphere (**differ**, by design: the look is the framework's,
the timing and selection are the original's). No gap at the adapter level.

## Open questions

1. Transition timing: Morrowind's `Transition Delta` semantics (per-frame vs per-hour) need checking
   against OpenMW's `weather.cpp` before tuning. Note the result here.
2. Tribunal's Mournhold: confirm which of its cells behave like exteriors and what weather they get.
3. Blight disease interval and exposure rule: take OpenMW's behavior as the reference when the rules
   plugin is written (M10).
