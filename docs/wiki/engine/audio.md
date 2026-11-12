# Audio

## What it does
Sound is a mixer of buses over `bevy_audio`. The framework core holds buses, attenuation, cues, music, footsteps,
impacts and ambience logic (testable without a device); the Morrowind adapter turns SOUN/SNDG/DOOR records, music
folders and voice INFOs into that data; the game glue turns facts into sounds. Agents cannot hear: every start and stop
is an event and an `info!` line.

## How it works
- **Core** (`superfargoth/crates/sf_audio`, Bevy-free): `Bus` (`lib.rs:34`: Master, Ambience, Weather, Sfx, Music, Ui,
  Voice, Movie), `Mixer` (`:85`), `SoundBank`/`SoundFile`, `OneShotScheduler`, `attenuation.rs`, sound groups (cues
  with variants), `music.rs` player, `footsteps.rs`, `impacts.rs` (physics impacts and slides), `ambience.rs` layers,
  `volume.rs` (`VolumeSettings`, perceptual slider curve, mutes, ducking rules: speech ducks music).
- **Bevy plugin** (`sf_audio_bevy`): `SfAudioPlugin` (`lib.rs:197`): bus loops and one-shots (`BusSound`),
  random sets (`OneShotSets`), cue one-shots 2D or positional, looping emitters, ambience beds, music. Positional
  voices take their curve from `sf_audio::Attenuation`; `SpatialListener` only pans. Features: `physics`
  (`PhysicsAudioPlugin`, `physics.rs:19`), `weather` (`WeatherAudioPlugin`, `weather.rs:15`: beds, thunder),
  `settings` (`SettingsAudioPlugin`, `settings.rs:17`: volumes from the settings store, persisted in the profile).
  `AudioVolumes` and `AudioDucking` write the mixer each frame; `AudioEvent` messages for every start/stop.
- **Morrowind content** (`mw_audio`, Bevy-free): every `SOUN` is a bank entry and a one-variant positional cue
  (range bytes in metres via `fAudio*` GMSTs); `SNDG` creature cues `<creature>/<kind>`; door sounds; loop emitters
  from lights with a sound and the vanilla `PlayLoopSound3D` activator idiom (`loop_script`, a named scaffold until
  the script runtime plays them); footsteps by boots weight class; `Music/Explore` and `Music/Battle` playlists via
  the VFS; NPC voices (`voice.rs`): INFO selection, greeting and idle timing, lip-sync envelope.
- **Game glue** (`mw_audio_bevy::add`, `lib.rs:79`, called at `mw_exterior_bevy/src/exterior.rs:226`): listener on the
  door-using camera; door open sound travels with the player through a load; `LoopEmitter` children spawned with the
  cell's near level and unloaded with it (`emitter_census` logs counts); explore/battle music; footsteps from clip
  `SoundGen` text keys (graph `footstep` events), else a `StrideTimer`.
- **Dev-feature only** (`mw_audio_bevy` `dev`, `VoicePlugin` `lib.rs:487`): NPC voices (Hello, idle chatter, `SayVoice`,
  `LipSync`), NPC boots, swinging door sounds. Gated until the owner hears them (visual-checks AU-3..AU-5).

## Bevy features used
`bevy_audio` (default features off in the workspace, `Cargo.toml:24`), `SpatialListener`, messages for audio events.

## Switches and counters
Volume sliders via the options screen (audio page). Log lines per start/stop; `emitter_census`. No `--off` part for
audio.

## Known limits
`CombatMusic` is never set (no combat yet), so battle music does not play. Interior `AmbienceLayers` "cell" beds
for the modern preset are a follow-up; vanilla interiors rely on sound activators. Voices are dev-only. No reverb or
occlusion in the code reviewed here.

## Specs
`superfargoth/docs/specs/audio.md`, `video.md` (movie bus).
