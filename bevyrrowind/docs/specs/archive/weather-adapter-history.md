# weather-adapter history

Dated "As built" blocks moved verbatim from [weather-adapter.md](../weather-adapter.md) (doc cleanup S12c, 2026-10-26).

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


## As built: WE2 part 3b (we2c2, 2026-10-19)

- **The weather records are a synthesized mod**, `morrowind-weather` ("Morrowind.ini weather", `mw_mods::WEATHER`),
  not an app call after loading. Record mods apply *inside* the `sf_mods` record step, one source per mod, and
  patches are eager (patching a record that doesn't exist yet fails), so a source defined after the load would come
  after every record mod and none could patch it. `MwAdapter::discover` adds the mod with `load_after` = every
  plugin + `morrowind-data`; `mount` gives it one generated marker file (`weather/morrowind-weather.mwweather`, a
  `MemSource` layer like the builtin mod's), which `apply_records` answers with `MwWeather::build(db so far, ini)`
  + `MwWeather::define_in(db, mod source)`, so the climates see every plugin's `REGN` (the original reads regions
  from the final load order too). `mw_mods::default_profile` lists it after `morrowind-data`; `ensure_weather` adds it
  to profile files that lack it (like `pin_builtin`). Its `stamp` is the adapter version + `Morrowind.ini`'s size
  and mtime, so an ini edit changes the profile fingerprint. `MwWeather::define_into` (own source) stays for tests and
  tools. Known limit: an ESP inside a *folder* mod ordered after `morrowind-weather` changes no climate (its `REGN`
  edits come too late); fix when it bites (order the weather mod after such mods, or rebuild climates in
  `finish_records` for REGN-only changes).
- Real install: `info` lists `weather.climate 16`, `weather.precip 4`, `weather.preset 10`; `mods report --mod
  morrowind-weather` is clean. A test record mod patches `mw_weather/clear` (`climate_ambience`) through the pipeline.
- **`bevyrrowind weather forecast <region> [--days 3] [--seed 0] [--from <game h>]`** (`apps/bevyrrowind/src/weather.rs`):
  loads the profile, builds a `WeatherBook` from the records, prints the climate's weights, change interval and
  transition, then one row per change slot from `WeatherState::forecast` (`day N hh:mm`, weather type name or the
  preset id for mod-added presets, transition hours, `(forced)`), and the per-type counts. The first row is the
  weather already running at `--from`. Example (Bitter Coast, seed 0): Thunderstorm, then Cloudy ×4 over 3 days.
- **Frequency check**: `mw_weather` test `forecast_frequencies_follow_the_regn_chances` (synthetic REGN: Ascadian
  Isles, Red Mountain, Moesring; two seeds; 1,000 slots each; every type within 4.5 binomial σ + 1, nothing outside
  the climate; the forecast lists exactly the slots' presets) and `real_install.rs`
  `real_regions_forecast_their_regn_chances` (all 16 real regions, 1,000 slots; skips without an install). Helpers:
  `mw_weather::{register_kinds, weather_type_of, MwWeather::define_in}`.
- Verify item "ini edit re-runs only `weather.climate`" is moot as built: there is no weather bake step; the records
  are rebuilt from `REGN` + ini at every load (the `fingerprint` field stays for a future cached step).

## As built: WE3 part 1 (we3a, 2026-10-19)

- **App wiring** (`exterior.rs`, `weather.rs`): `WeatherSetup::build(db, eye, hours, force, seed 0)` resolves the
  `WeatherBook` from the loaded `RecordDb` (the `morrowind-weather` mod), maps every exterior `CELL` grid square to
  `mw_region/<region>`'s climate, and falls back to the Bitter Coast climate (synthetic levels, regionless cells).
  `weather_site` writes `WeatherSite` from the exterior camera's grid square (absolute engine position = terrain
  origin cell + camera translation) and altitude; in an interior worldspace it keeps the last climate.
- **Haze, dust and dust tint now come from the presets** (`sf_weather_bevy` drives `sf_sky`); the app's `SkyLook`
  keeps only the fog look (part 2 moves fog and the cloud genus to the frame). The we2b test proves the presets equal
  the removed table at every quarter hour, so `--weather X` looks as `--clouds X` did. `morning_haze` moved to the
  presets' `haze_by_hour` curve. Wind: the field now follows the preset (`1 + 14 w` m/s, wandering) instead of
  `sf_sky::Wind::default()`; the Red Mountain storm source joins in ash/blight weather.
- **Flags:** `--weather <type>` forces one Morrowind weather in every climate (no transition, held for good);
  `--clouds` now defaults to the start weather's genus (forced, or the start cell's forecast at the start hour,
  printed as `weather <Type>`); `--haze`/`--dust` pin the sky terms (`WeatherOverrides`). The live C key still swaps
  cloud genus + fog look only.
- **Bridge until part 2:** clouds and fog are picked once at start from the start weather; haze and wind follow the
  live forecast, so a long run can show e.g. rain haze under the start's clear clouds.

## As built: WE3 part 2a (we3b1, 2026-10-19)

- **Clouds, fog and sky cover follow the live weather** (`sf_weather_bevy` writes `Clouds`/`Fog` from the frame). The
  app's scaffold tables are gone: `CLOUD_CHOICES`/`CloudChoice`, `SkyLook`/`FogLook`/`SKY_*`, `SkyWeather`,
  `drive_sky_weather`, `feed_wind`, `fog_settings`, `ground_fog`. Their values live in the presets
  (`mw_weather::visual`, proven equal by we2b's test). The app keeps only the cloud look (`CloudLook.scaled(0.5)`)
  and `CLOUD_WORLD_SCALE` (now `mw_weather::visual`'s).
- **Flags:** `--clouds <genus>` pins a genus over the weather's clouds (Morrowind weather names moved to
  `--weather`); `--fog <x>` scales the weather's fog densities.
- **Live controls** (owner request): C forces the next weather (REGN order) everywhere over the climate's
  transition; T cycles the clock x1/x4/x20 (game time x30/x120/x600). BRP `bevyrrowind/set`: `weather: "<type>"`
  (or `"auto"`: glide back to the forecast), `transition_hours` (0 skips), `time_scale`, `clouds: "<genus>"`
  (`"weather"` unpins). `bevyrrowind/status.weather`: `current`, `clouds`/`fog` (how far the current weather has
  come in per channel), `forced`, `next` (weather, hours until). A live force is an override in `WorldWeather`
  (`weather::force_live`), so it replays like a script's `ChangeWeather`; `auto` forces the forecast's weather at
  now, released at the next natural change (removing the override would pop).


## As built: WE3 part 2b (we3b2, 2026-10-19)

- **Precipitation:** the app adds `sf_precip_bevy::PrecipPlugin` with the weather (re-exported by `sf_weather_bevy`);
  `weather_site` switches `Precip.enabled` off in interiors (no occlusion map yet, so nothing else stops rain indoors).
  `mw_weather` kinds gain tints: rain grey-blue (0.3 opacity), snow white (0.85), ash grey-brown and blight red-brown
  (0.75), and ash/blight grit grows to 4-10 mm so it reads as blowing streaks in a storm.
- **Region border blend:** `ClimateMap::border_at` takes the camera's square's climate and the nearest square of another
  climate among the 8 around it (distance to that square); `border_blend` with a band of min(256 m, 2 squares). Cells
  without a region use the fallback climate, so coasts next to region-less sea cells blend toward it.
- **Cues:** `WeatherSound` messages (rain/ash loops, thunder) and the frame's region one-shot sets are played by `sf_audio_bevy` (superfargoth audio.md): `SOUN` ids -> VFS files in `mw_weather::sound_bank` (path `Sound/<FNAM>`, wav or mp3 via `resolve_preferring`, volume DATA/255; filled at launch by `WeatherSetup::load_sounds`). Log lines `audio: loop start/stop`, `audio: one-shot` are the headless check; owner listens (visual-checks WE-3).
