## Weather: state machine and presets as data

**Records** (`sf_records` kinds; adapters generate them, mods patch them field by field):

```rust
pub struct WeatherPreset {
    pub id: RecordId,
    pub clouds: CloudLayers,          // per layer: coverage, type (stratus 0 → cumulus 1), density,
                                      // base/top (m above sea level), erosion, wind factor
    pub sky: SkyTerms,                // haze multiplier, dust/ash absorption tint, overcast dimming,
                                      // optional sun/ambient tint KeyCurves over sun elevation
    pub fog: FogTerms,                // height fog: density, falloff (m), base height, albedo, phase g;
                                      // ground banks: density, height, noise scale; KeyCurves over
                                      // time of day (morning fog)
    pub precipitation: Vec<PrecipTerm>, // kind: RecordRef<PrecipKind> (rain, snow, ash, …), intensity
    pub wind: WindTerms,              // speed range, gustiness, gust length, direction bias/lock
    pub surface: SurfaceRates,        // wet/puddle/snow/ash accumulation and drying per hour
    pub lightning: Option<Lightning>, // strikes per minute, flash intensity, sound set
    pub cues: Vec<Cue>,               // ambient loops, precipitation loops (RecordRef to sound records)
    pub tags: Vec<(RecordRef, f32)>,  // gameplay tags with strength (e.g. a disease-exposure tag)
    pub transition: TransitionTiming, // per-channel lead/lag: clouds first, precipitation last
}
pub struct Climate {                  // "region weather"
    pub id: RecordId,
    pub weights: Vec<(RecordRef<WeatherPreset>, f32)>,
    pub change_interval_hours: (f32, f32),
    pub transition_hours: f32,
    pub storm_source: Option<WindSource>, // e.g. radial wind away from a volcano
}
pub struct PrecipKind { pub id: RecordId, pub look: PrecipLook /* streak | flake | dust */,
    pub fall_speed: f32, pub size: [f32; 2], pub texture: VfsPath, pub splash: Option<VfsPath>,
    pub accumulates: SurfaceChannel /* wet | snow | ash | none */ }
```

Durations are in **game hours** on `SkyClock`, so time scale and sleeping work.

**Which climate applies.** An adapter hook `ClimateSource::climate_at(ws, WorldPos) -> RecordRef`
(Morrowind: the cell's region). A bake step `weather.climate` rasterizes it into a coarse per-worldspace
grid with distance to the nearest border. The camera's weather is the blend of the climates within a
**border band** (default 256 m), so crossing a region border is a spatial crossfade, not a pop. RDR2 and
UE weather volumes behave the same way.

**State and forecast.** Per climate: `ClimateWeather { current, next, change_at, transition_start }`.
The next preset is a **pure function** of `(world seed, climate id, change index)` (weighted pick from
a counter-based hash), as in bevy_weather's driver. So saves store only the seed plus explicit
overrides, and a forecast can be queried. Scripted overrides (`force(climate, preset, hours)`,
weight edits from quests) are records in the runtime overlay, so saves keep them.

**Blending.** `WeatherFrame = blend(presets, weights, t)`: continuous fields interpolate per channel
with the preset's lead/lag (clouds 0-60% of the transition, wind 20-80%, precipitation and fog 40-100%).
Discrete kinds (precipitation, sound loops) crossfade by intensity. Rain fades out while snow fades
in; they never morph. Surface rates add.

**Outputs.** `WeatherFrame { sky: sf_sky::Weather, wind: WindState, clouds, fog, precip, surface_rates,
lightning_events, cues, tags, visibility_m }`. `visibility_m` (Koschmieder from fog + precipitation)
feeds AI perception and the dev HUD.

**Interiors** have no weather. Their fog comes from an adapter-supplied fog-only preset per interior
(an environment volume, M8). Interiors marked as exterior-like get their climate's weather.

**Lightning**: the strike schedule is a Poisson process from the hash, so it is deterministic and
forecastable. A strike = a 0.1-0.3 s flash (a temporary point light up in the cloud + cloud emission +
the directional sky term) and a thunder cue delayed by distance / 343 m/s.

### Weather core as built (WE1, we1, 2026-10-18): `sf_weather`

**Reuse check (rule 8).** Bevy 0.20 has no weather. No maintained crate does a forecastable weather state
machine (crates.io/web, 2026-10-18: Bevy state-machine crates `seldom_state`, `moonshine_behavior` are
entity FSMs with stored state; `bevy_weather` 0.2 is on Bevy 0.19 and its driver was the reference
above). The core is ~1 000 lines of plain Rust, so: ours. Randomness is a counter-based hash
(SplitMix64 finalizer over `(seed, record key, salt, counter)`), not a `rand` RNG: stateless, O(1) per
slot, nothing to save. Record keys are FNV-1a of the folded id, so load order and added mods never
reshuffle a forecast.

**Crate** (Bevy-free; deps `glam`, `serde`, `smallvec`, `sf_records`, `sf_sky`, `sf_wind`, and
**`sf_atmos`**, not in the plan above: the frame carries the renderer's `CloudWeather` and `FogSettings`
directly instead of mirror structs, and `visibility_m` uses `FogSettings::extinction_at`; `sf_atmos` is
`glam` + `rayon` only). `sf_records::RecordId` gained serde (its folded string) for record references.

**Records** (kinds `weather.preset`, `weather.climate`, `weather.precip`; `register_kinds`; all
`#[serde(default)]`, so a record file or patch names only the fields it sets). As the sketch above,
with these differences: the record id is the database key (no `id` field); references are `RecordId`
(no typed `RecordRef<T>` exists yet); `CloudTerms` is `CloudWeather` as data with a cloud-altitude
`speed` (the direction is the ground wind's); `FogTerms` holds the height fog and bank layer terms plus
`height_by_hour` / `banks_by_hour` multiplier curves, `SkyTerms` a `haze_by_hour` curve (empty curve = 1);
`WindTerms { speed: [min, max], gustiness, gust_length, direction_deg, direction_jitter_deg, storm }`;
`PrecipKind` has an `extinction` (1/m at intensity 1) for visibility; `Climate` has `cues` (region
ambient loops) and `one_shots` (`OneShotSet`, random one-shot ambience: sounds with a chance per draw, rest silence,
interval in **real seconds**; the audio layer schedules the draws, a "scatterer" as in FMOD Studio), and
`WeatherPreset.climate_ambience` (0..1, cue channel) scales the climate's loops and one-shots, so storms hush
birds (we2c1). `WeatherBook::from_db` resolves everything into indices once (rebuild when weather
records change); unknown references become `warnings`, not panics.

**State machine.** Per climate, slot `k` starts at `phase + k·mean + jitter·(2u_k − 1)` with `mean` the
middle of `change_interval_hours` and `jitter` a quarter of its range, so every interval stays inside the
range and the slot containing any time is found in O(1) (no summing of past intervals). Slot `k`'s preset
is a weighted draw from `hash(seed, climate, k)`. `WeatherState { seed, overrides }` is the save: an
`Override::Force { climate, preset, start, transition_hours, until }` (default: until the first slot start
one minimum interval later) or `Override::Weights { climate, from, weights }` (slots starting at or after
`from`). Overrides are **timestamped entries in the state, not record patches**: a patch has no "from
when", so editing weights through the record overlay would rewrite the past and the current weather.
A **segment** is one transition (at a slot start, a forced change, or a force's end) from the blend just
before it (recursively, so a change interrupting a change starts where it was) to one preset.
`forecast(climate, from, n)` lists segments; `current` is the preset being turned into.

**Blending.** Six channels with windows from `TransitionTiming` (fractions of the transition, smoothstep
inside): clouds 0-60 %, sky 0-60 %, wind 20-80 %, fog 40-100 %; precipitation (also lightning rate,
surface rates, tags) in 40-100 % and **out 0-60 %**, cues the same. Continuous channels use the incoming
preset's window (weights sum to 1); discrete channels fade each outgoing preset by its own `*_out` window
and the incoming one by its in-window, summed per precipitation kind or sound, so rain stops before the
sky clears, and rain → snow is two crossfading entries, never a mixed kind. Per preset the wind wanders
inside its speed range and ±jitter direction by smooth value noise (3 h lattice) keyed by climate and
preset; presets blend as velocity vectors; `gust_length` is discrete (the heaviest preset's, one swap at
mid-transition, since `sf_wind` snaps it to lattice divisors).

**Border band.** `border_weight(d, band)` = ½ on the border, 1 from `band/2` inside (smoothstep);
`border_blend(own, neighbor, d, band)` returns the climate weights `evaluate` takes. The grid lookup
(`weather.climate` bake, `ClimateSource`) is WE2/WE3.

**Lightning.** A non-homogeneous Poisson process by thinning (Lewis and Shedler 1979): candidates at the
cap rate (`LIGHTNING_CAP_PER_HOUR` = 120 per game hour) are drawn per 1/30 h bucket (Poisson count by CDF
inversion, uniform times, all hashed), each kept with probability `rate(t) · climate weight / cap`, where
`rate(t)` is the precipitation-channel blend of `Lightning::per_hour`. Strikes carry time, azimuth,
distance, flash, and a thunder sound. Deterministic and independent of frame boundaries; steps over 1 h
(sleep, skips) report none. **Rates are per game hour** (every weather duration is game time); the adapter
converts Morrowind's real-time frequency with its time scale.

**Frame** (`evaluate` / `evaluate_into`, which reuses allocations): `sky: sf_sky::Weather` (cloud cover
= `CloudWeather::mean_cover`, haze, dust, dimming, and **`sky.wind`, the wind target** for
`WindField.state.wind`; the transition windows are its easing), `clouds: CloudWeather` (moving with the
wind direction at the cloud speed), `fog: FogSettings` (weather fields; look fields default, the plugin
keeps its own; banks drift with the ground wind), `precip`, `surface`, `lightning` (strikes in
`[prev_hours, hours)`), `lightning_per_hour`, `cues` (weather and region loops), `one_shots` (climate sets on, with volume), `tags`, `storms` (a
climate's `storm_source` with strength = the wind-channel blend of `WindTerms::storm` × climate weight;
WE3 adds/updates the `WindSource`), `visibility_m` = min(3.912 / (fog outside banks at the camera
altitude + Σ precipitation intensity × extinction), 100 km). No climate (interiors): calm defaults.

**Verified** (`cargo test -p sf_weather`, 10 tests): forecast purity (same seed same slots, load order
irrelevant, intervals in range, `slot_at` inverts `slot_start`), draw frequencies within 4σ of the
weights over 8 000 slots, save/restore (RON) with a force and a weight edit gives identical frames over
220 h, lead/lag (clouds half way at 30 %, no rain before 40 %, rain gone at 60 % of the way out,
continuous weights sum to 1), rain → snow crossfade (only the two kinds, monotone, no coverage pops),
border walk (2 m steps, no pops in coverage, snow or wind), lightning (count within 4σ of 30/h over
400 h, exponential gaps (CV ≈ 1), identical strikes at 30 s and 8.6 min steps), wind easing and
visibility, records through `sf_records` with a mod patch. **`evaluate` cost: 0.0006 ms** (release, 2
climates mid-transition, lightning on; gate 0.05 ms).

**Open (logged).** Overrides are never pruned (a long game with many scripted changes grows the save;
prune entries older than one interval + transition, keeping the last weight edit). `gust_length` swaps
once mid-transition (crossfade two wind frames if it shows). Lightning flash position is camera-relative
(no world anchor). Interior fog-only presets (M8). `storm_intensity` for NPC animation: derive from
`storms` when needed.

### WE3 as built (part 1, we3a, 2026-10-19): `sf_weather_bevy`

**Reuse check (rule 8).** Bevy has no weather system; the crate is glue between `sf_weather` (logic), `sf_sky_bevy`
(clock, sky medium) and `sf_wind_bevy` (field). Deps: `bevy_app/ecs/platform`, `sf_diag_bevy` (counters),
`sf_sky_bevy`, `sf_weather`, `sf_wind_bevy`, `smallvec`. Not on `sf_atmos_bevy`: clouds and fog read the frame from
their own consumers (part 2).

- `WeatherPlugin { book: Arc<WeatherBook>, state: WeatherState }` inserts `WeatherRecords`, `WorldWeather` (the save
  data: seed + overrides; scripts and debug flags `force` through it), `WeatherSite` (camera climates with spatial
  weights + altitude, **written by the app**: only it knows its regions), `CurrentWeather { frame, hours, valid }`,
  `WeatherOverrides { haze, dust }` (debug pins), `WeatherStats`.
- `WeatherSystems::{Evaluate, Apply}`, chained in `Update` after `SkySystems::Clock`, before `SkySystems::Apply`.
  Evaluate: game hours = `SkyTime.days × hours_per_day`; lightning window `(prev, now]` clamped to
  `MAX_STEP_HOURS` (0.25 h, a clock jump skips strikes) and empty on the first frame or when the clock goes back;
  an empty site skips (keeps the last weather, e.g. interiors). Apply: sky `haze`/`dust`/`dust_tint` written only on
  change (the sky's rate-limited rebuild sees no churn); `WindField.state.wind` = `frame.wind()`; one radial source
  per storming climate, added once and retuned in place (new `WindState::source_mut`) so its attack never restarts.
- **Not driven yet:** the sky's `cloud_cover` / `overcast_dimming` (they change the lit look; part 2 with the cloud
  genus blend), clouds, fog, precipitation, lightning light, cues, border blend.
- **Replication (co-op, net.md):** the frame is a pure function of (records, `WorldWeather`, clock, site); a host
  replicates the seed + overrides and the clock, each peer evaluates its own site.
- Counters: `weather/eval/ms`, `weather/apply/ms`, `weather/strikes`. Cost: evaluate 0.0027 ms mean, whole
  `app.update` with the plugin 0.005 ms (test `cost_per_frame`, opt-level 1; gate 0.05 ms).

### WE3 as built (part 2a, we3b1, 2026-10-19): clouds, fog, sky cover

- `sf_weather_bevy` now also depends on `sf_atmos_bevy` (the `Clouds` and `Fog` resources are the consumers; a
  separate glue crate would be one more node for three small systems). `WeatherSystems::Apply` runs `apply_sky`,
  `apply_clouds`, `apply_fog`, `apply_wind`; all write only on change, and each is a no-op without its resource.
- **Clouds:** `Clouds.settings.weather` = `frame.clouds` (the per-channel genus blend: coverage, type, density,
  wetness, heights, cirrus, and the wind at cloud altitude = ground wind direction × the presets' blended `speed`).
  The look (`CloudLook`, noise tiles) stays the app's, so its `scaled(0.5)` halving survives. No pops: every term is a
  smoothstep blend on the clouds window (0-60 %), and the scroll integrates the wind (`CloudScroll` travel), so a speed
  change never jumps positions.
- **Sky:** `cloud_cover` (= the drawn layer's `mean_cover`) and `overcast_dimming` join haze/dust/tint.
- **Fog:** `fog_settings(frame.fog, scale, look)`: height fog, bank layer + coverage, albedo, phase, ambient and wind
  from the frame (hour curves already applied by `evaluate`), the renderer's look fields (reach, sky distance, bank
  noise tiles/warp) kept. Fog follows on its own window (40-100 %): clouds lead, fog lags.
- `WeatherOverrides` gains `clouds: Option<CloudWeather>` (a pinned genus; its wind stays the weather's) and
  `fog_scale: Option<f32>` (density multiplier).
- Tests (6 green): `frame_drives_clouds_and_fog` (clouds = frame, fog ×2 scale, look fields kept, cover/dimming, pin
  keeps the wind); `transition_glides_clouds_lead` (clear → overcast over 2 h in 400 steps: max per-step change within a
  smoothstep's bound, clouds pass half-way before the fog). Cost unchanged (gate 0.05 ms).


### WE3 as built (part 2b, we3b2, 2026-10-19): precipitation, lightning, cues, border blend

- **New crate `sf_precip_bevy`** (thin; `PrecipPlugin`, `Precip` resource, `PrecipMaterial`, `precip.wesl`). The
  technique of "Precipitation" below minus occlusion: three nested camera-centered boxes (8 / 24 / 64 m, 3000 / 6000 /
  8000 drops at intensity 1), one draw each per kind slot (2 slots, so a crossfade draws both; 6 entities). Drop
  position closed-form in the vertex shader: `cam + (fract((seed·L + offset − cam) / L) − ½)·L`, the offset being
  velocity (fall + wind × follow) integrated on the CPU and wrapped to `[0, L)` (a wind change never jumps drops).
  A drop shows while its rank (evenly spread, shuffled) < intensity. Streaks stretch along the velocity
  (`STREAK_S` 0.035 s of travel) and face the camera; flakes are soft discs that flutter. Minimum width 1 px
  (flakes 1.5) with alpha dimmed to keep energy; faded at the box faces, inside the nested box, and within 1 m of
  the camera. Far boxes are more transparent. Alpha blended, no prepass, no shadows, `NoFrustumCulling`.
  **No particle crate:** `bevy_hanabi` is not on Bevy 0.20 (crate table above) and the closed form needs no simulator.
- `PrecipKind.tint` (linear rgb + opacity) is new. Looks: `Streak` = rain (wind follow 0.8); `Flake` = snow
  (flutter 0.15 m); `Dust` = blowing grit **drawn as wind streaks** (flutter 0.3 m, follow 1): ash storms at 10-12 m/s
  read as near-horizontal blowing ash (owner, Next up 6a), not specks.
- `sf_weather_bevy::fx`: `apply_precip` maps `frame.precip` into `Precip` slots (a kind keeps its slot while it falls:
  `assign_slots`), wind = `frame.wind().velocity`, brightness = `precip_brightness(sun elevation, overcast dimming,
  flash)` (night floor 0.04). Written only on change.
- **Lightning:** `LightningStrike::{flash_at, thunder_delay_s, thunder_volume}` in `sf_weather` (pure: decay over
  `fade_s` + two re-strokes placed by the strike time; delay = distance / `SPEED_OF_SOUND` 343). `apply_lightning`:
  the latest strike restarts `LightningFlash` (real time); a shadowless `DirectionalLight` (`LightningLight`, spawned on
  the first strike, hidden between flashes) from the strike's azimuth at 0.95 rad elevation, `LIGHTNING_LUX` 40 000 ×
  flash; **cloud emission stand-in:** the clouds' `look.ambient` × (1 + 6 × flash), restored after (a true in-cloud
  emission at the strike position is still open). Rain brightens with the flash.
- **Cues:** `WeatherSound` message: `Loop { sound, volume, region }` on start / stop (volume 0) / change ≥ 0.02;
  `OneShot { sound, volume, azimuth, distance_m }` for thunder when its delay has passed. Played by `sf_audio_bevy` (audio.md; WE3 audio, 2026-10-19):
  loops fade over 2 s on the Weather bus, thunder plays on arrival, and the frame's `one_shots` are drawn by `sf_audio::OneShotScheduler` on the Ambience bus.
- **Border blend (app):** `weather_site` finds the nearest grid square of another climate among the 8 around the
  camera (distance to that square), and writes `border_blend(own, neighbor, d, band)`, band = min(`BORDER_BAND_M`,
  2 squares) ≈ 234 m in Morrowind; written only on a weight change ≥ 1e-3. Interiors switch `Precip.enabled` off.
- Tests: sf_precip_bevy 3 (mesh/ranks, wrap, nesting/light), sf_weather `thunder_delay_and_flash_are_pure`,
  sf_weather_bevy `frame_drives_precip_cues_and_lightning`, `slots_keep_their_kind_through_a_crossfade`.
- **Occlusion map (as built, rainocc 2026-10-19):** `sf_precip_bevy/src/occlusion.rs`, a top-down ortho depth map (512², 80 m square, 100 m depth, reverse Z) drawn as one more shadow view of the strongest shadow-casting directional light (subview 0xFB, no pass of our own), redrawn when the camera moves 6 m (8 m vertically) or casters stream in (30-frame throttle). The vertex shader kills a drop with a caster above it (0.3 m slack). Hosts add `NoPrecipOcclusion` to terrain and far proxies (done in exterior.rs). `PrecipOcclusionConfig(false)` turns it off. Render side: `sf_sky_bevy::sun_view` (no view when the light has no shadow maps by render time; see "Shared sun-view mechanism"). Since it rides on the sun's shadows, no occlusion when the sun casts none (overcast weather, if the sky turns them off). Not yet seen under a roof/tree (owner check). Splashes not built. **Along the travel direction (storm-occlusion, WE-5, 2026-10-25):** the view now looks along the strongest slot's velocity (`view_dir`, never shallower than 15° below the horizon, so a lee shelter is ~3.7 obstacle heights, not 15:1 behind every bump), eye 100 m upstream, 160 m depth, 96 m square; falling rain is the top-down case. Redrawn on a 3° turn (8-frame throttle). Terrain is now an occluder (hills shelter their lee; exterior.rs no longer opts it out); far proxies stay out. Common rain occlusion maps are top-down and miss blown precipitation; this is the directional-shadow-map form of the same map.
- **Not built:** splashes, ripples, fog on the streaks (transparent
  fog path, fogfix), textures (`PrecipKind.texture` unused: procedural shapes), cloud shadow-map rate modulation,
  in-cloud emission at the strike.


