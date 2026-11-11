# Video playback

Status: design (2026-11-11), owner answer at the end. Uses: menu backdrop loop, intro/logo videos, later
cutscenes ([menus.md](menus.md)). Research: [video-playback-2026-11-11](../research/video-playback-2026-11-11.md).

<!-- agent-summary -->
> **Agent summary (2026-11-11).** Status: design, not built. Bevy-free `sf_video` (`VideoSource` trait, decoders
> registered per format by header probe) + thin `sf_video_bevy` (decode thread, frame `Image` on a UI
> `ImageNode`, audio-master A/V sync, `Bus::Movie`, loop, skip). First decoder: Bink 1 in pure Rust
> (`infinitier_bik_decoder`, GPL-3.0-or-later), gated on a measurement; FFmpeg optional later for mod formats.
<!-- /agent-summary -->

## Goal

Play a video into a UI node with its sound, looped or once, skippable, in sync, cheap on the CPU, with formats
plugged in per game (Morrowind ships Bink 1; another game might ship AV1, H.264 or Theora). No bake needed.

## Canonical technique

A decode thread fills a small frame queue; the main thread shows the newest frame whose presentation time has
come; **audio is the master clock** (samples handed to the device), video drops frames over one frame late and
waits when early (FFmpeg's ffplay, OpenMW `videostate.cpp`). Loops seek to start inside the decoder (OpenMW
`MenuVideo`). Engines: OpenMW (FFmpeg), Godot 4 (Theora), UE Media Framework, Unity VideoPlayer (platform).

## Reuse

- No Bevy video plugin supports 0.20 (research). Frames go through a plain `Image` (`MAIN_WORLD |
  RENDER_WORLD`, data overwritten per frame; 640×480 RGBA ≈ 37 MB/s at 30 fps); a YUV `UiMaterial` path only
  if profiling asks.
- Audio through `sf_audio_bevy` (bevy_audio/rodio): a custom `Decodable` whose source pulls PCM from a
  lock-free ring the decode thread fills and counts samples played.
- Decoders: **Bink 1 via `infinitier_bik_decoder`** (crates.io 0.1.0, 2026-11-06, GPL-3.0-or-later, Rust port
  of FFmpeg's Bink video + audio, part of the GPL Infinity Engine project `ufoscout/infinitier`; young, one
  author). Fallback if it fails the gate: port FFmpeg `libavcodec/bink.c` + `binkaudio.c` (LGPL-2.1+, cite file
  + commit). **FFmpeg** (`ffmpeg-sidecar` MIT or an LGPL shared build) only as an optional backend for
  mp4/webm mod videos, behind a feature, later: it costs DLLs, libclang/bindgen and contributor setup.
  Rejected: Windows Media Foundation (no Bink), `rav1d-safe` (AGPL).

## Design

- `sf_video` (Bevy-free): `trait VideoSource { fn info() -> VideoInfo (w, h, fps, audio: Option<rate, ch>,
  frames); fn next_frame(&mut self, out: &mut Frame) -> Result<Option<Pts>>; fn next_audio(...);
  fn seek_start(); }`; `VideoDecoders` registry: `probe(header: &[u8]) -> Option<DecoderId>` per registered
  decoder (probe the header, not the extension; mods ship Bink under other names). Reads through `sf_vfs`.
- `sf_video_bevy`: `VideoPlayer { source: path, looped, skippable, bus }` component on an `ImageNode`
  entity; spawns the decode thread (3-4 frame queue), swaps the newest due frame, emits `VideoFinished` /
  `VideoSkipped`; pauses its thread when hidden or the window is minimised; no-audio videos clock on `Time`.
- `Bus::Movie` in `sf_audio` (volume row on the audio page); while a sounded video plays, music, ambience and
  SFX pause or duck (OpenMW pauses all but movie sound).
- Skip: Confirm, Back, Escape or click, through the UI router (one context while a skippable video plays).

## Build plan

- **V1** `sf_video` trait + registry + the Bink decoder behind it; `#[ignore = "measurement"]` test decoding
  every `.bik` of the install (ms/frame, failures). Gate: all Morrowind files decode, ≤ ~4 ms/frame at 640×480
  on the laptop; else the fallback port.
- **V2** `sf_video_bevy` player: frames on an `ImageNode`, loop, no-audio clock. Test: headless play of a
  generated test stream, frame count and loop sanity.
- **V3** audio track, A/V sync, `Bus::Movie`, ducking, skip. Test: sync drift stays bounded (printed measurement).

## Owner answers (2026-11-11)

1. **Chosen:** start with `infinitier_bik_decoder` (pure Rust, GPL-3.0-or-later, young single-author crate),
   measured in V1 with the FFmpeg-port fallback; FFmpeg later for mod formats only.
