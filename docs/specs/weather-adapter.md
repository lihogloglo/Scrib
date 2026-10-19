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
- **Region sounds** (`REGN` sound list with chances) become one `OneShotSet` in `Climate.one_shots`:
  every `Minimum/Maximum Time Between Environmental Sounds` real seconds (vanilla 1-5 s) one draw, each
  sound with its chance / 100 per draw, the rest silence. Weathers with an ambient loop of their own
  (`Ambient Loop Sound ID`: ash, blight, blizzard) have `climate_ambience = 0`, which fades the region's
  ambience out on the cue channel.
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

| MWScript | `mw_weather::script` | `sf_weather` call |
|---|---|---|
| `ChangeWeather "<region>" <type>` | `change_weather(state, region, type, now)` | `WeatherState::force(mw_region/<id>, mw_weather/<type>, now, None, None)`: the climate's transition, held until the first slot start one minimum interval later; a type outside 0-9 does nothing |
| `ModRegion "<region>" c0 … c9` | `mod_region(state, region, chances, now)` | `WeatherState::set_weights(mw_region/<id>, now, …)`: slots starting from `now` draw from the new chances (the weather in progress stays, the past doesn't change); 8 values (pre-Bloodmoon) leave Snow/Blizzard at 0; all zero = Clear |
| `GetCurrentWeather` | `current_weather(state, book, region, now)` → `WeatherType` (`.index()`) | `WeatherState::current`: the preset the climate is in **or turning into**, so a script sees its own `ChangeWeather` at once (the original reports the old weather until the transition ends) |
| `GetWindSpeed` | `wind_speed(frame)` | `WeatherFrame::wind()` speed mapped back through `WIND_SPEED_RANGE` to the ini's `Wind Speed` 0..1 scale (wander and blends show through) |
| `GetMasserPhase`, `GetSecundaPhase`, `TurnMoonRed`, `TurnMoonWhite` | none | sky (moons, `night_sky.rs`), not weather |

These are the only weather functions in `mw_script`'s function table (`functions.txt`, "weather, sky").
They go through the MWScript → Luau layer (M5), which calls the curated weather API (`weather.force`,
`weather.set_climate_weights`, `weather.current`, `weather.wind_at`). OpenMW-Lua's `core.weather`
(where present) maps to the same calls. The region is the script's argument; for `GetCurrentWeather` and
`GetWindSpeed` the caller passes the player's region / the frame at the player.

### Sounds

Cues carry `SOUN` record refs: weather ambient loops (rain, heavy rain, ash storm, blight, blizzard),
thunder sets (delayed by distance); region ambient sounds are one-shot sets (`OneShotSet`, above).
The audio system (later) owns playback and schedules the one-shot draws.
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
- **Fog** (M6 F2, as built; same scaffold table, `SkyLook::fog` = `FogLook`, until `mw_weather` reads `Land Fog Day/Night
  Depth` through Koschmieder): height-fog visibility at sea level / falloff height per weather: clear 40 km / 100 m and fair
  30 km / 120 m (F3: a thin valley haze so dawn sun shafts have a medium), the other cloud genera none; overcast 8 km / 300 m; rain 3 km / 250 m (+ banks 0.25); thunderstorm 2 km / 250 m (+ banks 0.3);
  foggy 3 km / 150 m + banks 1.0 at dawn, a third by day; snow as rain; blizzard 300 m / 400 m; ash storm 450 m /
  500 m with ash albedo (0.55, 0.47, 0.38), blight the same with a red albedo (0.62, 0.42, 0.36). Ground-fog banks:
  0.03 /m at sea level (≈ 130 m visibility inside one), 12 m falloff, × `ground_fog(hour)` (1 from 4:30 to 8:30, gone
  by 11:00, half from 19:00 through the night). Height fog × (1 + 0.5 × the same curve). `--fog <x>` scales both,
  `--off fog` drops them. Sea level is the fog's base (Bitter Coast swamps sit in the banks; Balmora's valley gets
  the height fog). Banks by region (Bitter Coast, West Gash mornings) wait for climates (WE2/WE3).
  F3 (froxels, god rays): the same terms fill the froxel volume near the camera (High, 384 m), lit by the sun
  through the cascades, the terrain shadow-height map and the cloud shadow, and by the 32 nearest `LIGH` point
  lights; `--off froxels` (alias `godrays`) is the analytic-only A/B. The app's transparent NIF and particle
  materials still draw `StandardMaterial` (unfogged): switching blended ones to `FroxelFogMaterial` is a follow-up.
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

## As built: WE2 part 1 (we2a, 2026-10-18)

`crates/mw_weather` (Bevy-free; deps `mw_install`, `sf_records`, `sf_weather`, `tes3`).
`MwWeather::build(&db, ini: Option<&MorrowindIni>)` → `presets` (10, `mw_weather/<type>`, `WeatherType` = REGN index
order, also MWScript's), `climates` (one per live `REGN`, `mw_region/<region id>`), `precip_kinds`
(`mw_precip/{rain,snow,ash,blight}`), `timescale` (the `timescale` global, default 30) and `fingerprint` (FNV-1a of every
ini value read, vanilla where absent, plus the time scale: unrelated ini edits leave it unchanged).
`define_into(&mut db, depends_on)` registers the `weather.*` kinds and defines everything in a new source
**"Morrowind.ini weather"**; call it after the content files and before record mods (a test patches a climate from a
later source). `mw_install::MorrowindIni` now keeps every `key=value` (`values`, `get(section, key)`, case-insensitive).

Mapped so far:
- **Climate**: weights = REGN chances as-is (zero chances dropped; all-zero → Clear); `change_interval_hours` =
  `Hours Between Weather Changes` × (0.75, 1.25); `transition_hours` = chance-weighted mean of the presets' durations.
  `Transition Delta` is read as OpenMW does (`weather.cpp`: "how quickly ... in Hz ... real time", the factor drops by
  `delta × real seconds`), so a transition lasts `1/delta` real seconds = `timescale / (delta × 3600)` game hours
  (0.015 → 0.56 h, Blight 0.040 → 0.21 h). This answers open question 1. sf_weather has one duration per climate
  (Morrowind's is per target weather): hence the weighted mean.
- **Preset**: precipitation kind by type (Rain/Thunderstorm when `Using Precip`; Snow, Blizzard → snow; Ashstorm → ash;
  Blight → blight); intensity = `Max Raindrops` / Thunderstorm's (450/650 for Rain), snow `Max Snowflakes` / 1500
  (Blizzard 1); `transition.precipitation` starts at `Rain/Snow/Storm Threshold` (our reading: the transition fraction
  where it begins). Surface rates from the kind (rain wet 2 × i, puddles 0.5 × i; snow 0.5 × i; ash 0.25 × i; drying
  0.25/h without precipitation). `Ambient Loop Sound ID` and `Rain Loop Sound ID` → cues (`None` skipped). Lightning when
  `Thunder Frequency` > 0: `per_hour = f × 60 / timescale` (f strikes per real minute; vanilla 0.4 → 0.8/game h),
  thunder = `Thunder Sound ID 0..3`. `Disease Chance` → tag `mw_tag/blight_exposure`.
- **Precip kinds**: `Precip Gravity` read as the rain fall speed in units/s (575 → 8.2 m/s), snow × `Snow Gravity
  Scale`; ash and blight a slow dust (extinction 8e-3/m ≈ 450 m visibility, the F2 ash fog), accumulating ash.
- Real install: 16 climates (Vvardenfell, Solstheim, Mournhold); Red Mountain is Blight 100 before the main quest.

## As built: WE2 part 2 (we2b, 2026-10-18)

`mw_weather::visual` fills each preset's visual fields with the values the app's scaffold tables (`exterior.rs`
`CLOUD_CHOICES`, `SKY_*`, `FogLook`, `morning_haze`, `ground_fog`, `BANK_*`) hard-code, so WE3's switch to the presets
is a no-op on screen (test `presets_match_the_app_tables`, every quarter hour over 24 h, 1 % tolerance).

- **Clouds**: `CloudTerms::from_weather(CloudWeather::preset(genus).scaled_heights(0.5))`, genus per type as in the
  "Clouds" row; `speed = Cloud Speed × 10` m/s (ini, vanilla fallbacks). The app also scales the `CloudLook` (noise
  scales) by 0.5: that is renderer look, not weather, and stays with WE3's cloud plugin.
- **Sky**: haze, dust, `dust_tint` (blight 0.6/1.4/1.9) per type; `haze_by_hour` = morning haze (× 1.5 at 6:30).
- **Fog**: height fog = `KOSCHMIEDER / visibility` at sea level with the type's falloff, × (1 + ½ ground fog) by hour
  (`height_by_hour`); banks `0.03 × banks` with falloff 12 m and `banks_by_hour = banks_day + (1 − banks_day) × ground
  fog` (Foggy, Rain, Snow, Thunderstorm); ash albedo 0.55/0.47/0.38, blight 0.62/0.42/0.36. Hour curves are linear
  `KeyCurve`s sampled every 15 min, collinear keys dropped. `Land Fog Day/Night Depth` is **not read**: its depths are
  fractions of the original's short view range, and our fog is calibrated on 1-5 km views (F2); an ini fog edit
  changes nothing yet.
- **Wind**: `Wind Speed` w (vanilla 0-0.9) → mean `1 + 14 w` m/s, range ± 25 %; Ashstorm and Blight `wind.storm = 1`.
  Every climate with an ash or blight chance gets `storm_source = WindSource::radial(summit, 6 m/s, 3 km)` with a
  300 m inner radius; the summit is the highest `LAND` vertex (units 29568, 73728, 18952; cell 3,9; 271 m; the
  real-install test re-finds it), so far regions with ash (mods) get the preset wind only. Test: the source blows
  outward 1 km N/E/S/W of the summit.
- **Lightning**: `Flash Decrement` d (vanilla 4) → `Lightning::fade_s = 1/d` real seconds (new field in sf_weather,
  also on `LightningStrike`). sf_weather now re-exports `sf_atmos`, `sf_sky`, `sf_wind` for adapters.

## As built: WE2 part 3a (we2c1, 2026-10-19)

- **Region sounds → one-shot sets, in sf_weather** (not a mw record). Random one-shot ambience is a generic,
  industry-standard construct: FMOD Studio's scatterer instrument (playlist, spawn interval, randomization), Wwise
  random containers on a trigger rate, UE sound cues with Random + Delay nodes, and Bethesda's later region records
  (sound + chance, gated by weather class). So `sf_weather::OneShotSet { sounds: [(id, chance per draw)], interval_s:
  [min, max], volume }` (chances summing over 1 normalize; the rest is silence) lives on `Climate.one_shots`.
  **Intervals are real seconds** (ambience density is perceptual, it mustn't follow the time scale), so the weather
  core doesn't schedule draws: `WeatherFrame.one_shots: Vec<OneShotFrame { climate, set, volume }>` says which sets are
  on and how loud (set volume × spatial climate weight × ambience), and the audio layer draws with
  `OneShotSet::next_interval_s(u)` / `pick(u)` and reads the set via `WeatherBook::climate_record`. New
  `WeatherPreset.climate_ambience` (default 1) is the weather-class gate as a number: the climate's loops and
  one-shots play at `max(0, w_climate − Σ w_cue·(1 − ambience))`, so a storm fades them out with its cue window.
  The adapter: chance % / 100, empty or 0 % sounds dropped, interval from the ini (vanilla 1-5 s), `climate_ambience
  = 0` where `Ambient Loop Sound ID` is set. Tests: draw frequencies (stratified 10k draws), normalization,
  interval range, frame fade monotone with loops and one-shots together, half volume on a border; REGN sounds map,
  ash/blight/blizzard hush.
- **MWScript** (`mw_weather::script`, table above): `change_weather`, `mod_region`, `current_weather`, `wind_speed`,
  tested on `WeatherState` (invalid types ignored, forced type held 10 h, unknown region `None`, wind inside the
  preset's range, `ModRegion` keeps the past and every later change Clear for 100 %, all zero = Clear).
- Not done: no positional one-shots (distance ring, as FMOD's scatterer min/max distance) and no per-sound weather
  filter beyond `climate_ambience`; the audio layer (later) may add both.

Not yet (we2c2, in order): (1) `bevyrrowind weather forecast <region> --days 3` (`WeatherState::forecast`) and the
1,000-slot frequency check; (2) the app calls `define_into` after content load, before record mods.
