# Audio (`sf_audio`, `sf_audio_bevy`)

Status: draft, minimal build (owner scope 2026-10-19: enough for weather sound, not a full audio system).
Milestone: M6 WE3 leftover; positional audio, music and UI sound are later.

## Goal

Play what the simulation asks for: weather loops that fade with the weather, thunder after its
distance delay, region one-shot ambience. Volumes follow a bus mixer. Nothing here is gameplay.

## Model

- **Buses** (`sf_audio::Bus`): `Master` (parent of all), `Ambience`, `Weather`, `Sfx`, `Music`, `Ui`.
  Each has a linear volume (default 1) and a mute; a voice's gain = voice level x sound's own
  volume x bus x master (`Mixer::gain`, pure). Only the buses the build uses are wired; the enum
  is the contract so later systems pick a bus instead of inventing one.
- **Loops**: a voice per looping sound with a `level` that eases toward a target volume at
  `1 / FADE_S` per second (`fade_toward`, pure, linear, never overshoots). Two loops
  fading in and out at once is the crossfade; the voice despawns when its level reaches 0 with
  target 0. Looped weather beds are noise-like, so linear gain fades do not dip audibly; an
  equal-power law is a one-line change in `fade_gain` if a tonal loop needs it.
- **One-shots**: thunder arrives as a message already delayed by the weather (distance / 343 s).
  Region ambience (`OneShotSet`: sounds with chances, interval, volume) is drawn by
  `sf_audio::OneShotScheduler` (pure, deterministic xorshift, interval uniform in range, chances
  normalized) from the frame's `one_shots`; the plugin plays the draw on the Ambience bus.
- **Positional (later)**: thunder carries `azimuth` and `distance_m`; the minimal build plays it
  non-spatially (distance already shapes `thunder_volume`). Spatial = `PlaybackSettings::with_spatial`
  plus a `SpatialListener` on the camera when a system needs it.
- **Streaming vs decoded**: Bevy's `AudioSource` decodes on playback from the loaded bytes (loops and
  short effects). Long music would want a streaming decoder; none of the minimal build needs it
  (Morrowind's ambient loops are small). Revisit with music.
- **Loading**: sounds are ordinary assets from the `sf_vfs` asset source (`hot_reload::asset_source`),
  so a mod's file replaces a vanilla one. The adapter owns id -> file: `SoundBank`
  (`RecordId` -> VFS path + base volume); the framework never names `SOUN`.

## Crate decision

| Option | Bevy 0.20 | Licence | Notes |
|---|---|---|---|
| `bevy_audio` 0.20 (built in) | yes, it is Bevy | MIT/Apache | rodio 0.22 on cpal; wav (hound) and mp3 are Bevy features; per-sink `set_volume`, loops, spatial; no buses, no fades |
| `bevy_kira_audio` 0.27.1 (2026-10-13) | yes (`^0.20`) | MIT/Apache | adds kira 0.12: tracks, tweens, crossfades built in; a second audio stack beside the one the app already links |
| `bevy_seedling` 0.8.0 (2026-08-09) | no (`^0.19`) | MIT/Apache | firewheel graph (best mixer story); blocked until it ships for 0.20 |

Pick: **`bevy_audio`**. Reuse order is Bevy first, and what it lacks (bus gains, volume easing)
is about forty lines of pure math with tests; the app already enables Bevy's `audio` feature, so this
adds only `wav`/`mp3` codecs. Kira would give tweens we do not need yet at the cost of a second
backend and compile time; seedling does not support our Bevy. Revisit when music or DSP (reverb,
occlusion filters) arrives: seedling on 0.20 or kira are the upgrade paths, and the `Mixer`/
`SoundBank` layer stays (only the voice-playing system changes). No new crate in the dependency
batch rule's sense.

## Co-op (net.md)

Audio is presentation: it reads replicated state (weather frame, strikes) and never writes
simulation state; no sound has gameplay effect, so each peer plays its own and may differ in timing.

## Industry comparison

- Wwise/FMOD: buses with snapshot-driven gains, scatterer/random containers for ambience, 3D
  attenuation curves. We match the bus gain and scatterer ideas (small), lack snapshots, DSP,
  occlusion and attenuation curves (gap, low until positional audio).
- UE: Sound Classes/Submixes, Ambient Sound with fade in/out, Sound Cues Random+Delay. Same coverage.
- Creation/OpenMW: OpenMW fades weather loops and plays region sounds on a random timer
  (our scatterer); both stream music. Match for weather; streaming music is a gap (later).

## Tests

Bevy-free: `fade_toward`, `Mixer::gain`, the scheduler's draw/interval/normalization and
determinism; adapter: SOUN -> VFS file resolution (case, `\`, wav/mp3 alternates). Bevy: a mini app
turns `WeatherSound` into voices and logs start/stop (`playback: false`). Agents cannot hear, so audible
checks are owner passes (docs/visual-checks.md).
