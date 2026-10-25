# Audio (`sf_audio`, `sf_audio_bevy`)

Status: framework built for ROADMAP 2.6 (sound pass slice 1, 2026-10-21): buses, cues (sound groups),
positional one-shots and loop emitters, ambience beds, music player, footstep hook. Volume mixer
(per-bus sliders and mutes in the settings, perceptual curve, ducking) 2026-10-22. Content wiring
(Bevyrrowind records, music folders, footsteps, doors/UI) is slice 2. Earlier: weather sound (M6 WE3),
interior snapshot (AU-1).

<!-- agent-summary -->
> **Agent summary (2026-10-26).** Status: framework built for ROADMAP 2.6 [~] (sound pass slice 1 plus volume mixer); content wiring is slice 2. Built: buses, cues, positional one-shots and loop emitters, ambience beds, music player, footstep hook, weather feeder, volume mixer with ducking. Designed, not built: DSP (occlusion low-pass, reverb), HRTF, sample-accurate music sync.
> Crates / key types: `sf_audio` (Bevy-free: `Bus`, `Mixer`, `CueBank`, `Attenuation`), `sf_audio_bevy` (`PlaySound`, `LoopEmitter`, `weather` feature). Visual checks: AU-3, AU-4, AU-5 awaiting owner.
> Decisions that matter: stay on `bevy_audio`, gaps are ours ("Backend"); attenuation curve is ours via `SpatialScale = 1/max_m`; Bevy `GlobalVolume` unused ("Model"); audio never writes sim state ("Co-op").
> Read next: "Model" for API; "Backend" for DSP upgrade path (`bevy_seedling`/`bevy_kira_audio`).
<!-- /agent-summary -->

## Goal

Play what the simulation and presentation ask for: weather loops and thunder, region one-shots,
gameplay and UI sounds (positional or 2D), looping world emitters, interior/exterior beds, and mood
music with crossfades. Volumes follow a bus mixer. Nothing here is gameplay.

## Model

- **Buses** (`sf_audio::Bus`), a tree (`Bus::parent`): `Master` over all, `Ambience`, `Weather`
  (under `Ambience`, 2026-10-22), `Sfx`, `Music`, `Ui`, `Voice` (speech). Each has a linear volume
  (default 1), a mute and a duck gain; a voice's gain = level x sound's own volume x the product of
  volume x duck along its bus chain, 0 if any bus on it is muted (`Mixer::gain`, pure). The volumes
  come from the player's settings (`AudioVolumes`, "Volume mixer" below); voices follow within a
  frame (`fade_voices` re-applies gains to every sink). Bevy's `GlobalVolume` is not used (it does
  not reach already-playing sinks).
- **Cues** (`sf_audio::CueBank`, `SoundGroup`): gameplay asks for a cue id; the group picks a variant
  (weighted, no immediate repeat: the Wwise random container / FMOD multi-instrument / UE Sound Cue
  Random) and rolls a volume and pitch in its ranges. A group carries its bus, its attenuation
  (`None` = 2D) and a max-instances cap (new plays over it are dropped). Ids that are not groups play
  as one sound under `CueBank::default_group` (Sfx, default curve), so every bank sound is a cue.
- **Positional** (`sf_audio::Attenuation`): `min_m` (full volume inside), `max_m` (silent from, and
  never started beyond), `Rolloff::Inverse` (default: `min/d`, offset to reach 0 exactly at `max_m`)
  or `Linear`. The curve is ours; the backend only pans (see "Backend"). `PlaySound { cue, at }` with
  `SoundAt::{Listener, Point, Entity, Offset}`; an `Entity` voice is a child so it follows its source
  (`Offset(e, v)`: a child at `v` in `e`'s local space, e.g. a door sound beside a player who travels).
  `LoopEmitter { cue }` on an entity adds a looping positional child voice (fires, water, machines).
- **Bus feeders** (2026-10-21): systems that keep their own sound state send `BusSound::{Loop,
  OneShot} { sound, volume, bus }` (a loop's message sets its target level; 0 fades it out) and fill
  `OneShotSets` (keyed random one-shot sets drawn on the Ambience bus outdoors). The weather is one
  such feeder: `sf_audio_bevy::weather::WeatherAudioPlugin` behind the `weather` cargo feature maps
  `WeatherSound` and the frame's region sets onto them, so `sf_audio_bevy` itself builds and tests
  without `sf_weather_bevy`'s render crates (ARCHITECTURE section 8b rule 6). Feeders run before
  `AudioSystems`.
  The host puts Bevy's `SpatialListener` on the camera (one listener; co-op is one listener per peer).
- **Voice budget**: at most `MAX_ONE_SHOTS` (64) cue one-shots at once, plus per-cue caps; plays
  beyond `max_m` are culled before they start. Loop emitters keep playing at gain 0 out of range
  (cheap at our counts); a virtual-voice pass (stop beyond range, resume on approach) comes with
  content that has many emitters (slice 2 measures).
- **Loops** (weather, beds, emitters): a voice with a `level` easing toward a target at `1 / FADE_S`
  per second (`fade_toward`, pure, linear, never overshoots). Two loops fading opposite ways is the
  crossfade; a loop despawns once silent with target 0. Noise-like beds use the linear law.
- **Ambience** (`sf_audio::AmbienceLayers`): named layers (host tags such as "region", "cell") of
  `(sound, volume, Space::{Outdoor, Indoor, Any})`. Wanted volume = volume x the mixer's eased
  outdoor exposure (Outdoor) or its complement (Indoor); a sound in two layers plays once at its
  loudest. `reconcile_loops` (pure) diffs wanted vs playing into start/retarget/stop, so walking
  through a door crossfades the exterior bed into the interior one.
- **One-shot scatterer**: region ambience (`OneShotSet`: sounds with chances, interval, volume) is
  drawn by `OneShotScheduler` (pure, deterministic `Rng`, interval uniform in range, chances
  normalized) from the weather frame's `one_shots`; the plugin plays draws on the Ambience bus.
  Thunder arrives already delayed by the weather (distance / 343 s).
- **Snapshots** (AU-1): the mixer's `outdoor` exposure (0..1) scales the outdoor buses
  (`Mixer::is_outdoor`: Weather) and weighs the ambience layers. The host sets `ListenerOutdoors`;
  `sf_audio_bevy` eases it over `OUTDOOR_FADE_S` (0.5 s) and draws no region one-shots below 0.5.
  A muffled "rain on the roof" layer (low-pass instead of silence) waits for DSP.
- **Volume mixer** (2026-10-22, `sf_audio::VolumeSettings`, `Ducker`): the player sees six sliders
  with mutes, `Bus::USER` = master, music, sfx, voice, ambience, UI (weather follows ambience: players
  hear rain as ambience, and a seventh slider only splits hairs). A slider position s (0..1) maps
  to gain through `slider_to_gain`: linear in dB over `VOLUME_RANGE_DB` (60 dB: -60 x (1 - s) dB,
  half travel = -30 dB), and below `VOLUME_TAPER` (0.1) a straight line to true silence at 0 (a
  pure dB curve never reaches 0). Loudness is heard logarithmically, so equal steps sound equal; an
  amplitude-linear slider crams the audible range into its last few percent. 60 dB is the usual
  control range (the exp(6.908 s) x 0.001 taper of Citra's volume slider and Cherry Audio's forum;
  KVR's DSP forum likewise suggests limiting the range to about 60 dB); `gain_to_slider` is the
  inverse. Mute is separate from the
  level, so unmuting restores it. `sf_audio_bevy::AudioVolumes` holds the settings and writes the
  mixer when they change (so `AudioMixer` volumes are owned by it, not set directly).
  **Settings model**: `sf_ui` ships the matching entries as built-in settings (owner `sf`, page
  `audio`, group `volume`): `sf.audio_<bus>_volume` (float 0..1, default 1, step 0.05: 20 gamepad
  presses) and `sf.audio_<bus>_muted` (bool), labels `set-audio-<bus>-volume`/`-muted` from the
  content's strings. They persist like every setting (`[settings.sf]` in the profile, non-defaults
  only, saved on leaving the menus by `sf_ui_bevy`). A record with the same id retunes one (a pack's
  music default of 0.7) and its value stays in `[settings.sf]`. The `settings` cargo feature's
  `SettingsAudioPlugin` copies the store into `AudioVolumes` on change, so a slider is heard within
  the frame; behind a feature (like `weather`) so audio builds without the UI stack.
  **Ducking** (`DuckRule { trigger, target, depth_db, attack_s, release_s }`, the Wwise
  auto-ducking parameters: duck volume, fade out, fade in): a trigger is a bus with a sounding voice
  (`DuckTrigger::Bus`, e.g. speech ducking music by 6-10 dB) or a named host hold
  (`DuckTrigger::Hold`, `Ducker::set_hold("menu", true)` for a pause menu or a cutscene: the hook
  for game state). The envelope moves linearly in dB; rules on one bus do not stack (deepest wins,
  Wwise caps stacking with a maximum ducking volume); a duck on a bus ducks its children. The
  framework ships no rules; the content adds them (`AudioDucking`). Activity is read from voice
  levels, not output gains, so a duck can never trigger itself.
- **Music** (`sf_audio::MusicPlayer`): playlists keyed by generic mood tags (the content names them,
  e.g. explore and battle); `request(Some(tag))` each frame from game state. A mood change
  crossfades over `crossfade_s` (default `MUSIC_FADE_S` = 3 s) with the equal-power law
  (`equal_power`, tonal material would dip with a linear one); a finished track starts the next from
  a shuffle bag (each track once per bag, no repeat across bags); `min_hold_s` stops mood flicker
  (debouncing combat start/end is the game's rule, e.g. hold battle a few seconds after the last
  hostile). Pure: the plugin plays `MusicCommand::{Start, Stop}`, sets gains from `voices()`, and
  reports a drained sink with `finished`. Music runs on real time (plays in menus).
- **Footsteps** (`sf_audio::FootstepMap`, `StrideTimer`): (animation event, surface tag) -> cue,
  falling back to the event's any-surface entry; event names come from animation text keys /
  notifies, surface tags from the host's ground query; both are content strings. `StrideTimer` is
  the fallback trigger for rigs without step events: a step per stride of ground travel (the first
  at half a stride), alternating feet; the host resets it when airborne or still.
- **Impacts** (2026-10-22; `sf_audio::ImpactAudio`, `ImpactMap`): physics contact reports become
  hit one-shots and slide loops. Input per report (`ContactSample`): the normal impulse (N·s), the
  normal speed change it caused (m/s, the impact speed independent of mass), the slide speed at the
  contact, both surfaces (`SurfaceId` values: per collision triangle on meshes, the ground probe's
  on terrain, `ItemContent::surface` on items) and which sides are moving bodies. A cue is picked
  by surface pair (exact pair, then the moving body's surface against any, then the other's, then
  a catch-all) and impulse band (`(min impulse, cue)` per entry: heavy bodies get heavy samples, a
  hit under the lowest band is silent); its volume ramps linearly with impact speed from
  `min_volume` (0.15) at `min_speed` (0.5 m/s) to 1 at `full_speed` (6 m/s). No machine-gun:
  (1) the speed gate (holding a body up against gravity changes its speed by `g·dt` = 0.16 m/s per
  step, so resting and rolling contacts never pass; physics already drops reports under 0.3 m/s),
  (2) one hit per moving body per `cooldown_s` (0.1 s) unless at least `override_ratio` (2x) harder
  than its last sound (world geometry is never limited: two balls on one floor both sound),
  (3) at most `max_per_tick` (8) hits per frame, loudest first. Slides: one loop per moving body
  (its fastest contact's pair), level from slide speed (0.4..3 m/s -> 0.3..1), eased over
  `slide_fade_s` (0.1 s), held `slide_hold_s` (0.12 s) after the last report, then faded and stopped;
  a change of surface swaps the loop. Rolling makes no slide (the contact point does not slip);
  a rolling loop from angular speed is future work. Industry: Havok/PhysX contact listeners with
  separating-velocity thresholds, Source's per-object impact-sound delay and scrape sounds, UE
  Chaos hit/slide events, Unity `relativeVelocity`/impulse recipes: same gates. Bevy side:
  `ContactSound` messages (positions in the listener's frame) -> `PlaySoundScaled` hits at the
  contact point and `SlideVoice` loops that follow it; the `physics` feature's
  `PhysicsAudioPlugin` writes them from `PhysicsEvents`. Pitch by speed and mass waits for content.
- **Streaming vs decoded**: `bevy_audio` keeps the encoded bytes in the asset and decodes while
  playing (mp3/wav), so a 5 MB music track costs 5 MB of RAM, not its PCM size. True disk streaming
  is not needed at our sizes.
- **Loading**: sounds are ordinary assets from the `sf_vfs` asset source, so a mod's file replaces a
  vanilla one. The adapter owns id -> file (`SoundBank`: id -> VFS path + base volume) and id ->
  group (`AudioCues`); the framework never names a format's records.

## Backend (reuse: Bevy, then crates, then ours)

Re-evaluated 2026-10-21 for the full sound pass (positional SFX, buses, music, many one-shots).

| Option | Bevy 0.20 | Licence | Fit |
|---|---|---|---|
| `bevy_audio` 0.20 (built in, rodio 0.22) | yes | MIT/Apache | spatial panning via `SpatialListener` + `PlaybackSettings::with_spatial`; per-sink volume and speed; loops; one rodio player per voice into one mixer. No buses, tweens, attenuation curves or DSP |
| `bevy_kira_audio` 0.27.1 (2026-10-13) | yes | MIT/Apache | kira 0.12: tracks (buses), tweens, spatial emitters with distance curves, clock-synced music. A second audio stack beside Bevy's, more compile time |
| `bevy_seedling` 0.8.0 (2026-08-09, Firewheel) | no (0.19 only; no 0.20 release as of 2026-10-21) | MIT/Apache | best mixer graph (sample pools, buses, filters, HRTF option) |

Pick: **stay on `bevy_audio`.** Each gap is small, pure and tested in `sf_audio`: buses are gain
products, fades and crossfades are `fade_toward`/`equal_power`, the attenuation curve is ours.
The one real limit was rodio's fixed distance law: it multiplies each ear by `min(1, 1/d^2)` in
scaled units (amplitude falls as intensity should, -12 dB per doubling, no max distance). We set each
spatial voice's `SpatialScale` to `1 / max_m`, which puts the whole audible range inside one unit
where rodio's factor is 1 (ears are scaled too, so panning is unchanged), and apply
`Attenuation::gain` as the sink volume. What `bevy_audio` cannot do: DSP (low-pass for occlusion and
the muffled interior weather, reverb zones), sample-accurate music sync, HRTF. Upgrade path: when a
slice needs DSP, move the voice-playing systems to `bevy_seedling` (once it ships for our Bevy) or
`bevy_kira_audio`; `Mixer`, `CueBank`, `Attenuation`, `MusicPlayer`, `AmbienceLayers` and the
messages stay, only `sf_audio_bevy`'s spawn/gain code changes.

## Co-op (net.md)

Audio is presentation: it reads replicated state and events (weather frame, strikes, hits, door
opens, animation events) and never writes simulation state. Each peer plays its own sounds with its
own listener; draws (variants, pitch, region one-shots, track order) may differ between peers.

## Industry comparison

- Wwise/FMOD: buses + snapshots, random/sequence containers, 3D attenuation curves, voice limits,
  music transitions on beat, auto-ducking / sidechain. We match buses, one snapshot (interior),
  random containers, curves (two shapes), instance caps, timed crossfades and volume ducking (bus
  activity or host holds; a level-following sidechain compressor needs DSP); we lack DSP,
  occlusion, beat-synced transitions.
- UE: Sound Classes/Submixes, Sound Cues (Random, Modulator for pitch/volume), Attenuation assets,
  concurrency groups, Ambient Sound with fades. Same coverage as above.
- Creation engine/OpenMW: weather loops faded, region sounds on a random timer, music from explore
  and battle folders with a fade on change, footsteps from animation text keys. All covered; the
  content mapping is slice 2.

## Tests

Bevy-free (`sf_audio`): fades, mixer gains, bus tree, slider curve (dB points, taper continuity,
monotonic, inverse), volume settings and mutes, ducking (attack/release timing, no stacking, holds,
children), outdoor exposure, scheduler draw/interval/determinism,
attenuation (bounds, monotonic, shapes), group picks (no repeat, weights, empty), cue resolution and
fallback, music (crossfade levels, shuffle bag without repeats, hold, silence), footstep map fallback
and stride cadence, ambience exposure weights and loop reconciliation, impacts (pair fallback, bands,
speed gate and volume, cooldown and override, bounces, per-tick cap, slide start/follow/stop and
surface switch). `sf_physics`: contact reports (landing speed, rolling and resting silent, slide
speed, per-triangle surfaces). Bevy (`sf_audio_bevy`, mini
app with `playback: false`): weather loops/thunder, indoor easing, cue caps/variants/distance cull,
distance gain, ambience beds through a door, music start/fade, contacts (scaled hits, a slide loop
that follows and stops), volumes to the mixer, speech ducking
music and releasing; with `settings`: built-in settings exist for every menu bus, a store change
reaches the mixer within the frame. `sf_ui`: built-ins registered, retunable, persisted under
`[settings.sf]`. Agents cannot hear: audible checks
are owner passes (docs/visual-checks.md).
