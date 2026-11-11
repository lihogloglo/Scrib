# Video playback in a Bevy 0.20 UI node (research, 2026-11-11)

Status: research only, no code. Question: how does a game-agnostic framework play a video (with sound) inside a UI node,
for a menu background, logo clips and cutscenes? Crate facts were read from the crates.io API on 2026-11-11; items marked
"unverified" were not checked against source.

## Recommendation (short)

1. Seam: a Bevy-free `sf_video` crate with `trait VideoSource` (open from a VFS path, `info()` = size, frame rate, audio
   rate/channels; `next_frame()` -> planar YUV 4:2:0 or RGBA plus a presentation time; `next_audio()` -> interleaved f32 PCM;
   `seek_start()` for loops). Decoders are separate crates registered by file extension, the same way `sf_vfs` /
   `sf_texture` loaders plug in. A thin `sf_video_bevy` crate owns the thread, the `Image`, and the audio clock.
2. First decoder: the game's native format, decoded in pure Rust (for Morrowind, Bink 1; see the game note). No DLLs, no
   platform dependence, no bake step.
3. Second decoder (for mods that ship mp4/webm/ogv, and other games): optional, behind a feature, FFmpeg as a sidecar or
   shared LGPL build. Not on the critical path; do not make it a default dependency (compile time, DLLs).
4. Do not build on a Bevy video plugin: none is maintained for 0.20 (below).

## Decoder options (as of Nov 2026)

Licence column: must be GPL-3.0-or-later compatible. "Bevy-free" means it does not depend on Bevy at all (what we want).

| Option | Licence | Maintenance (crates.io) | Windows cost | Audio | Notes |
|---|---|---|---|---|---|
| `ffmpeg-next` 9.0.0 (Aug 2026) | WTFPL (binding); FFmpeg itself LGPL-2.1+ if built without GPL parts | active | needs FFmpeg dev libs + libclang/bindgen at build; DLLs at run | yes (audio decoding in the same API) | every codec and container; heaviest build story |
| `ffmpeg-the-third` 6.0.0 (Aug 2026) | WTFPL | active fork of the above | same | yes | tracks FFmpeg 9 |
| `rsmpeg` 0.18 (Aug 2025) | MIT | slower, FFmpeg 8 | same | yes | lower-level, closer to the C API |
| `video-rs` 0.12 (Sep 2026) | MIT or Apache-2.0 | active | same (wraps ffmpeg-next) | docs mention video only | easy, but a wrapper over the above |
| `ffmpeg-sidecar` 2.7 (Oct 2026) | MIT | active | runs `ffmpeg.exe` as a child process and reads raw frames from a pipe; can auto-download a build; no link step | audio via a second pipe (more work) | fastest to ship, no compile cost; one extra exe (LGPL "essentials"-style builds are 80-100 MB, a trimmed build is smaller: unverified) |
| `dav1d` 0.11 (Nov 2025) | MIT (binding); libdav1d BSD-2 | ok | needs libdav1d (meson/nasm build or vcpkg) | none (video only) | fastest AV1 decoder |
| `rav1d` 1.1 (May 2025) | BSD-2 | quiet since May 2025 | pure Rust (c2rust port of dav1d), no C toolchain | none | roughly 5-10% slower than dav1d (unverified); AV1 only |
| `rav1d-safe` 0.6 | AGPL-3.0-only OR commercial | active | n/a | n/a | rejected: AGPL |
| `openh264` 0.9.10 (Nov 2026) | BSD-2 (wrapper) | very active | Cisco's binary is downloaded at run time (its H.264 patent licence only covers Cisco's binary), or build from source | none | H.264 baseline/main/high decode. Patent status is the reason not to bundle our own build |
| `re_video` 0.42 (Nov 2026, Rerun) | MIT or Apache-2.0 | very active | unverified: wraps dav1d and ffmpeg-sidecar style backends | none | built for a viewer, video only, pulls Rerun's tree; reuse ideas, not the crate |
| `oxideav-theora` 0.0.12 | not checked | new, tiny download count | pure Rust | no | Theora is royalty-free but the crate is immature; unverified quality |
| `bevy_av1` 0.3 (Oct 2025) | MIT or Apache-2.0 | stale, Bevy 0.17 | needs dav1d | no | AV1 in IVF only |
| `bevy_movie_player` 0.7.1 (Jan 2026) | WTFPL or 0BSD | Bevy version unclear | optional ffmpeg feature needs system FFmpeg | unclear | not a base to build on |
| `bevy_video` 0.9, `bevy_video_playing`, `bevy_h264` | n/a | dead, Bevy 0.9-0.10 | | | ignore |
| NihAV (`nihav-rad`: Smacker, Bink) | GPL-3.0+ stated in the author's blog; no licence field in its Cargo.toml | git.nihav.org only, not on crates.io | pure Rust | yes | usable licence-wise (GPL-3.0-or-later), but not packaged, API is NihAV's own; heavier to vendor than a standalone decoder |
| Windows Media Foundation via `windows` crate | n/a (OS) | OS | no DLLs, but H.264/AAC/WMV only; no Bink, Theora, VP9/AV1 without codec packs | yes | Windows only; a second path for Linux/macOS is still needed. Rejected as the base |
| GStreamer (`gstreamer-rs`) | LGPL-2.1 | active | large runtime install, plugin discovery | yes | heavy to ship; rejected |

Container/audio helpers if we ever write our own glue: `symphonia` 0.6.1 is MPL-2.0 (allowed only unmodified), so prefer
what `bevy_audio` already decodes.

## Bink in particular

FFmpeg decodes Bink 1 and 2 video and Bink audio (`libavcodec/bink.c`, `binkaudio.c`, LGPL). Windows Media Foundation and
GStreamer's stock plugins do not. Pure Rust: `infinitier_bik_decoder` 0.1.0 (June to Nov 2026, repo ufoscout/infinitier,
GPL-3.0-or-later) is a Rust port of FFmpeg's bink.c and binkaudio.c (release/6.1), covering BIKb/f/g/h/i/k video (not Bink 2,
"KB2"), DCT and RDFT audio, YUV420 or RGBA output, as a streaming `next_frame()` API. This is the single best fit for Morrowind
(its files are `BIKi`), but it is brand new, tiny, with a single author, so budget a verification step: decode all 17
Morrowind `.bik` files, compare against FFmpeg output once on a developer machine (a measurement, not a CI check), and be
ready to vendor and own it. If it fails, porting `bink.c` ourselves is allowed (LGPL-2.1+ code may be used in GPL-3.0; cite
file and commit and add a THIRD_PARTY_NOTICES entry). Bink 2 would need FFmpeg (no pure-Rust decoder found).

## Putting frames into Bevy

- Decode on a dedicated thread (a plain `std::thread`, or `AsyncComputeTaskPool`); the main thread only swaps the newest
  ready frame in. Keep a small queue (3-4 frames) so decode hiccups do not drop frames.
- Display: an `Image` (`Rgba8UnormSrgb`, `TextureDimension::D2`) shown through the UI `ImageNode`. Create it with
  `RenderAssetUsages::MAIN_WORLD | RENDER_WORLD` (a render-world-only image drops its CPU copy after upload and cannot be
  mutated; unverified for 0.20, check `bevy_image` docs). Each frame: `images.get_mut(&handle)`, overwrite `data`, which
  re-uploads the whole texture. Cost is memory bandwidth only: 640x480 RGBA is 1.2 MB per frame (about 37 MB/s at 30 fps,
  negligible); 1080p is 8 MB (about 250 MB/s, still fine on a laptop, but also do the colour conversion on the GPU).
- Cheaper for larger video: upload Y, U and V as three `R8Unorm` images (1.5 bytes per pixel, no CPU conversion) and draw
  them through a `UiMaterial` shader that does YUV to RGB. Both Bink and H.264/AV1 decode to planar YUV 4:2:0 natively, so this
  is also the "zero conversion" path. Start with RGBA for 640x480 (simple), move to the shader only if profiling asks.
- Aspect ratio: letterbox in the UI node (OpenMW does this with a black parent box). Make "cover", "contain" and
  "stretch" a setting; the menu background wants "cover" for mods authored at 16:9.
- Gamma: video is Rec.601/709 sRGB-ish, so treat the image as sRGB and do not run it through the world tonemapper (UI
  already bypasses it).

## Audio and A/V sync

`sf_audio_bevy` is built on `bevy_audio` (rodio). Bevy plays anything that is a `Decodable` asset. For video, make a
custom `Decodable` whose rodio `Source` pulls PCM from a lock-free ring buffer filled by the decode thread. The standard
approach (FFmpeg's ffplay, OpenMW's `videostate.cpp`) is to make **audio the master clock**: the source counts samples
handed to the device (an atomic counter), the video thread displays a frame when its presentation time is at or before
`samples_played / rate`, drops frames that are more than about one frame late, and waits when early. If a video has no audio
track (the Morrowind menu loop), use the Bevy `Time` clock. Route the audio to a mixer bus (new `Bus::Movie`, or reuse
the music bus), and pause/duck music and ambience while a video with sound plays (OpenMW pauses all sounds except movie
sounds, `windowmanagerimp.cpp` `playVideo`).

## Looping a background video

Loop in the decoder, not the player: at end of stream call `seek_start()` (reopen the file, reset codec state; Bink and
H.264 both start with a keyframe) and keep the queue full so there is no visible gap. Decode one loop ahead on the thread
if the first frame is slow. Do not cache all frames in RAM: 885 frames at 640x480 is about 1 GB as RGBA or 400 MB as
YUV. When the menu is hidden or the window is minimised, pause the thread (OpenMW's `MenuVideo` does this).

## Three approaches compared

(a) **Runtime decode of the native format.** No bake, no conversion loss, mods that replace the file just work, nothing to
ship. Needs a decoder for each format a game uses. Cost: 640x480 Bink 1 is cheap (a C decoder uses a small fraction of one
core; the Rust port is not yet measured here, do that). Chosen path.

(b) **One-time transcode to a format with a light pure-Rust decoder.** Possible for a menu loop (a bake step must stay
under a few minutes, and an AV1 or VP9 encode is far slower than that for a 25 MB intro), loses quality, needs an encoder
(FFmpeg) on the user's machine at bake time anyway, and does not help mod files that appear later. Not recommended.

(c) **Platform decoders** (Media Foundation, AVFoundation, GStreamer). Zero licence worry, hardware decode, but Windows
Media Foundation does not read Bink, and three platform back ends triple the test surface. Keep as a later `VideoSource`
for mp4/H.264 mods if wanted.

What others do: OpenMW decodes everything through FFmpeg (libavformat, libavcodec, swscale to RGBA, audio into OpenAL with the
audio clock as master) in `extern/osg-ffmpeg-videoplayer`; Godot 4 ships only Ogg Theora through libtheora in core (other
formats via the GDExtension ecosystem); Unreal's Media Framework uses its Electra player and platform decoders; Unity's
VideoPlayer uses platform decoders (Media Foundation on Windows, AVFoundation on Apple) plus its own VP8/VP9 path. The common
lesson: each of them supports one or two formats natively and delegates the rest.

## FFmpeg on Windows, what shipping it costs

- LGPL builds (no `--enable-gpl`, no x264/x265) are fine for a GPL-3.0 game. Dynamic linking keeps the LGPL obligations
  simple (ship the licence text and a source offer).
- Shared build: `avcodec`, `avformat`, `avutil`, `swscale`, `swresample` DLLs (tens of MB; exact size depends on the build,
  unverified), found next to the exe. Static linking needs a custom build and our whole binary becomes subject to LGPL
  relinking terms.
- Build: `ffmpeg-next` / `rsmpeg` need FFmpeg headers, import libs, `libclang` for bindgen, and `pkg-config` or vcpkg; this is
  the biggest cost (extra minutes on a clean build, a setup step for every contributor, a CI image). `ffmpeg-sidecar` avoids
  all of that by spawning `ffmpeg.exe` and reading frames from a pipe (the player downloads or the user installs it), at the
  cost of process management, more awkward seeking and a one-process-per-clip start-up delay.
- So FFmpeg is the optional, feature-gated, second back end, not the default.

## Open items for whoever builds this

- Spec first (`superfargoth/docs/specs/video.md`, per CONTRIBUTING); it needs `VideoSource`, `VideoPlayer` component,
  `Bus::Movie`, pause/duck rules, skip-on-key rule for cutscenes.
- Measure (an `#[ignore = "measurement"]` test): ms per frame decoding a 640x480 `BIKi` file with the Rust port; and the
  `Image` upload cost on the target laptop.
- Confirm the `Image` mutate-per-frame path on Bevy 0.20 (usage flags, whether re-upload is per `AssetEvent::Modified`).

## Sources

- crates.io API (`https://crates.io/api/v1/crates/<name>`) for versions, dates and licences listed above.
- Pure-Rust Bink port: https://github.com/ufoscout/infinitier (`src/codecs/bik_decoder/readme.md`)
- NihAV licence discussion and rad crate: https://codecs.multimedia.cx/ ; https://git.nihav.org (nihav-rad Cargo.toml has no licence field)
- Bink format: https://wiki.multimedia.cx/index.php/Bink_Video
- OpenMW video player (FFmpeg, audio-master clock, swscale to RGBA): https://github.com/OpenMW/openmw/tree/97722d48f98f14b323c3880a698e8ce81babf7b3/extern/osg-ffmpeg-videoplayer
- Bevy video crate listings: https://docs.rs/bevy_video , https://docs.rs/bevy_av1 , https://docs.rs/crate/bevy_movie_player/latest/features
