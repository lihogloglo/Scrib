# Cloud upsampling at silhouettes (SH-12): diagnosis and proposed design

Research note, 2026-10-19. Nothing here is applied. It covers bevyrrowind `docs/visual-checks.md` SH-12 and the
uncommitted depth-aware resolve and composite in `crates/sf_atmos_bevy/src/shaders/cloud_resolve.wesl` and
`cloud_composite.wesl`. The owner's screenshots show two things: `Clipboard_10-19-2026_01.jpg` (the bright,
stair-stepped fringe inside the tower and hill, before the patch) and `outline.jpg` (the dotted, lighter halo
above every ridge line, after the patch).

## 1. The root cause both artefacts share

The trace mixes two kinds of texel in one buffer and one distance channel:

- `cloud_trace.wesl:242-247` reads **one** full-res depth texel per traced texel. The texel it reads,
  `uv * full_size`, is the bottom-right pixel of the half-res texel. The depth buffer is jittered by DLSS/TAA,
  and in checkerboard mode one traced texel stands for a 2×2 half-res block (4×4 full-res pixels).
- `:256`: the march ends at that depth (`end = min(seg.y, min(scene_t, far))`). When the sampled pixel is a
  tower 150 m away, the cloud shell is never entered, so `has_low` is false and the texel is cloud-free.
- `:268` and `:392`: the distance channel then holds the **geometry** distance (`min(scene_t, far)`). For sky
  texels the same channel holds a **cloud** distance (transmittance-weighted mean, shell exit, cirrus `ci.x`, or
  far-weather distance).

So at every silhouette the half-res buffer holds two kinds of texel: "sky-layer cloud" and "cut by an occluder,
cloud-free". The only label is a distance that also varies widely inside the sky. Every later pass has to guess
which kind a tap is from that distance, and with jitter and the 4-frame checkerboard rotation the guess flips
from frame to frame.

### 1a. Original fringe inside the meshes (before the patch)

At a half-res texel G on the geometry side of an edge, the resolve's 3×3 clamp box was built from trace texels
that include bright sky (original `lo/hi` loop, now `cloud_resolve.wesl:53-66`). Its bilinear history fetch at
`prev_uv` also blended sky texels. G therefore kept bright, forward-scattered cloud. On fresh frames its
distance was set to `current_dist`, the geometry distance (`:104`). In the composite the per-tap occlusion test
`d.x > scene_t * 1.02 + 10` (`cloud_composite.wesl:62`) then saw a distance equal to the tower's and did
**not** drop the tap. The result was bright cloud painted 1-2 half-res texels into the mesh, re-chosen every
frame by jitter and the checkerboard. That gave the blocky, shimmering fringe.

### 1b. New stippled halo outside the meshes (the patch)

1. Checkerboard: `cloud_resolve.wesl:28-35` takes `current` from `tp = px / 2`. That is the one texel of the
   2×2 block traced this frame, which for 3 of every 4 pixels is a **block-mate**. At an edge, the block-mate
   of a sky pixel S is often a geometry texel G (cloud-free, distance 150 m).
2. `:45` sets `tol` from that geometry distance (`0.1·150 + 5 = 20 m`). Every history tap around S holds a cloud
   distance in kilometres, so `:84-95` rejects all four, `history_w = 0`, and `valid` is false.
3. `:109-113` falls back to `current`, which is G's cloud-free sample. It also writes G's 150 m into S's history
   distance, so the next frame rejects S's own history as well.
4. Which pixels of a block get this treatment follows `sub_pixel(frame)` (`cloud_common.wesl:207`). The result
   is a checker or stipple of cloud-free sky one half-res texel wide, cycling over 4 frames: the halo.
5. The composite adds a second, thinner halo. `cloud_composite.wesl:52` sets `pixel_dist = min(scene_t, far)`,
   which is `far` for every sky pixel, but cloud texels sit at a few km. So `near` (`:66`) is false for almost
   every tap, the bilateral collapses to plain bilinear, and a sky pixel takes 25-50 % of the neighbouring
   cloud-free geometry texel.

### 1c. The sky "wobble"

Ranked by likelihood. The A/B steps in section 4 settle the order.

1. **New, from the patch: distance rejection inside the sky.** The tolerance (`:45`, `:57`, `:89`) compares
   cloud distances, and those legitimately differ by far more than 10 % between neighbours: cloud front versus
   empty sky (shell exit), cumulus versus cirrus, far weather. At every cloud edge the history is thrown away and
   pixels that were not traced this frame show a block-mate's single noisy sample (IGN jitter not averaged).
   Camera motion makes `prev_uv` cross more of these edges. Cloud edges then turn into 2×2 blocks that crawl
   while the camera moves.
2. **Older, independent of the patch: jitter mismatch in the reprojection.** The rays come from
   `uv_to_ray_direction`, which uses `view.view_from_clip`. That matrix is **jittered**: `bevy_render`
   `view/mod.rs:1218-1222` applies `TemporalJitter::jitter_projection` before inverting. The stored
   `prev_clip_from_world`, however, is built from `ExtractedView::clip_from_view`, which is **unjittered**
   (`render.rs:721`). Each frame's history is therefore fetched off by last frame's jitter (±0.5 render px,
   about ±0.75 output px at DLSS Quality). The error random-walks through the 3-of-4 history-only frames. The
   clouds swim by a pixel or so against the full-res sun disk and silhouettes, whether or not the camera moves,
   and the repeated sub-texel bilinear fetches soften them.
3. **Unlikely: RO-7 sky motion vectors** (`sf_render_order_bevy/src/sky_motion.wesl`). These vectors cover
   rotation only, at infinity. For clouds 3 km away, a 5 m/s strafe moves them about 0.05 px per frame at 1440p
   and 60 fps, and 15 m/s of wind about 0.13 px per frame, both under DLSS's clamp. This is the
   industry-standard choice for the sky. `--off skymotion` should change nothing.

## 2. What the industry does

- **Clouds as a sky layer, composited by full-res depth.** Horizon Zero Dawn's Nubis (Schneider and Vos,
  SIGGRAPH 2015) renders the cloudscape into a low-resolution buffer: quarter resolution, one pixel of each 4×4
  block updated per frame, the rest reprojected. It runs as part of the sky, behind the scene, so the low-res
  buffer holds only cloud, and the full-res opaque pass cuts the silhouettes exactly. (This is from memory of the
  talk; the PDF slides could not be extracted here.) Toft et al. 2016 summarise the scheme the same way. Frostbite
  (Hillaire 2016) traces at half resolution (640×360 for 720p) with temporal reprojection, and uses one
  transmittance-weighted mean depth per pixel for aerial perspective and reprojection. We already do the same.
- **UE5 Volumetric Cloud keeps two depths per low-res texel.** `r.VolumetricRenderTarget.Mode 0` traces at 1/4
  resolution, reconstructs at 1/2, then upsamples to full resolution, and it "can trace up to a certain depth and
  keep that information for the reconstruction" (Hillaire). Its `VolumeFrontDepth` stores **.x = cloud front
  depth and .y = the scene depth used when tracing**. The bilateral upsample (`UpsamplingMode 4`) runs only when
  the four taps' depths differ by more than a threshold. Hillaire calls that threshold "an empirical fudge
  factor", and a reported 4-pixel "brick" artefact at near/far boundaries comes from the same branch. Mode 2,
  which traces 1 pixel of each 4×4 and reprojects the rest, "would leak after reprojection over opaque meshes":
  the same failure as our original fringe. UE 5.2 improved "depth edge reconstruction, less holes, less trails
  during reprojection". Other cvars, recalled from engine source (please verify):
  `MinimumDistanceKmToEnableReprojection` and `MinimumDistanceKmToDisableDisoclusion`. Clouds beyond a few km
  skip disocclusion handling entirely, because nothing that far can be disoccluded by near geometry.
- **Downsample depth by the furthest value for clouds.** Low-res cloud passes take the **max distance** of the
  footprint, so a texel that touches sky traces the full ray. ChaliceUnity's cloud post-process "downsamples
  depth by keeping the greatest value", and UE's checkerboard min/max half-res depth serves the same purpose.
  The full-res composite then removes cloud from the geometry pixels.
- **Reject history by kind of texel, not by cloud distance.** Every source reprojects with a single
  representative cloud depth. Disocclusion tests apply to **geometry**-limited samples only, never to sky.

**Conclusion.** The march should not stop at a single depth sample. Stop at the footprint's furthest depth, so
any texel touching sky becomes pure sky-layer cloud. Record the depth the march was cut at as a separate value,
and let the full-res composite occlude by that pixel's own depth.

## 3. Proposed design

Formats: widen `DIST` (`render.rs:423`) from `Rg32Float` to `Rgba32Float`. The channels become **R** = cloud
distance (as now), **G** = bank modulation (as now), and **B** = **cut depth**, the scene distance that limited
the march, or the sentinel `clouds.far` when nothing did (an "open" texel). A stays spare. This adds about
15 MB per texture at 1440p half resolution, for 3 textures (trace plus 2 history). No new passes and no new
bindings, only one more channel to write and read.

### Trace (`cloud_trace.wesl`)

- Replace the single depth tap (`:242-247`) with the **furthest** distance over a 6×6 full-res window: the
  half-res texel's 2×2 pixels plus 2 pixels on each side, covering the composite's bilinear reach (±2 px) and the
  jitter (±0.5 px). In reverse-Z that is the minimum depth value, with 0 meaning sky (infinite). Use 9
  `textureGather`s or 36 `textureLoad`s. Either way that is about 2.25 depth loads per full-res pixel, which is
  negligible next to 32-80 march steps.
- `d_max` = that distance. `end = min(seg.y, far, d_max)`. Also use `d_max` in place of `scene_t` for the bank
  march (`:263`), cirrus (`:407`) and far weather (`:427`).
- `B = select(d_max, clouds.far, d_max >= min(seg.y, clouds.far))`. Any texel within about 2 px of sky is
  **open** and holds the full sky-layer cloud. Texels fully inside a near tower stay cut and cloud-free, as today,
  so the march cost goes up only on silhouette texels.
- R stays as it is. For cut texels with no cloud it stays `min(d_max, far)`.

### Resolve (`cloud_resolve.wesl`)

Define `open(t) = t.B >= clouds.far` and `same(a, b) = (open(a) && open(b)) || (!open(a) && !open(b) &&
abs(a.B - b.B) <= 0.1 * b.B + 5.0)`.

- `current` = this frame's traced texel at `tp`, as now. Because of the 6×6 window, a block-mate of an open
  pixel is open too.
- Clamp box: include a 3×3 trace tap only when `same(tap, current)`. This replaces the tolerance on cloud
  distance at `:57`.
- Reproject by R (cloud distance) as now (`:70-73`). **Use a jittered `prev_clip_from_world`** (render.rs, below).
- History: the 4 bilinear taps at `prev_uv`, each weight times `same(tap, current)`, then renormalised.
  Disocclusion is now handled for the right population. A newly revealed sky texel that was inside the tower
  last frame (cut) is rejected, which removes the trail behind moving silhouettes. Open taps are never rejected
  for having a different cloud distance, which removes the wobble described in 1c(1).
- Fallback when no tap is accepted: `current` (same population, by construction). Write `out.dist = (history
  R or current R, bank, current.B)`. B always comes from the current trace, so a texel's population can't be
  poisoned.
- Optional, as in UE: skip the `same` test for history taps whose R is above a few km (clouds that far cannot
  be disoccluded by near geometry).

### Composite (`cloud_composite.wesl`)

Per bilinear tap i:

- Occlusion as today (`:62`): if `R_i > scene_t·1.02 + 10`, the tap's cloud is behind this pixel's geometry,
  so it contributes `(0, 0, 0, 1)`.
- **Sky pixel** (`depth == 0`): weight times `open_i`, renormalised. Fall back to plain bilinear if the sum is 0.
- **Geometry pixel**: prefer taps cut at this pixel's depth (`!open_i && abs(B_i - scene_t) <= 0.1·scene_t + 5`).
  If there are none, use the open taps with the occlusion test (nearest-depth fallback). For a mountain inside
  cloud this over-fogs at most one texel at the edge, which UE accepts too.
- Bank: same weights as the colour, so the bilateral on `pixel_dist` (`:52`, `:66-67`) is removed.

### CPU (`render.rs`)

- `render.rs:721`: build `clip_from_world` from the **jittered** projection. Take `Option<&TemporalJitter>` and
  call `jitter_projection(&mut proj, full.as_vec2())` with the main-pass (render-resolution) size, exactly as
  `bevy_render` `view/mod.rs:1218-1222` does. Store that as next frame's `prev_clip_from_world`. The shader is
  unchanged: the current rays are already jittered, and the composite samples the half-res buffer in the same
  jittered grid, which is what DLSS/TAA expect.
- Allocate `DIST` as `Rgba32Float` and update the trace and resolve target formats (`render.rs:435-436`).
- Spec: update weather/clouds.md "Clouds", Rendering 4-5 and the "Passes" paragraph. The march stops at the
  footprint's furthest depth, not "at the main depth buffer".

### What this keeps and drops from the uncommitted patch

Keep the 4-tap manual history fetch and the "nearest current on no history" fallback. Drop every use of cloud
distance as a rejection key: resolve `:45`, `:57`, `:89`, and composite `:52`, `:66-67`. Better: revert the
patch and apply the design above as one change.

## 4. How to verify

**A/B to attribute the wobble** (owner, Balmora 17:30 toward the sun, plus the evening ridge from
`outline.jpg`):

| Run | Expect if section 1c is right |
|---|---|
| Patch reverted (`git stash`) | Fringe inside the meshes is back, cloud-edge "crawl" is gone (1c(1)); a slight swim remains (1c(2)) |
| Patch plus `--off skymotion` | No change (1c(3) ruled out) |
| Checkerboard off (`quality.checkerboard = false`) | The stippled halo becomes a solid 1-texel band, confirming the block-mate cause (1b) |
| New design | No fringe, no halo; ridge lines sharp; clouds still at rest relative to the sun disk when standing still |

**What the owner should look at:** slow pans and strafes past a tower against a bright low sun; a ridge against an
evening sky; standing still for 2 s (any swimming points to the jitter); a fast turn (trails behind silhouettes
point to disocclusion).

**Debug view (cheap; proposed):** a `clouds.debug` mode in the composite that tints open taps blue, cut taps
red, and pixels where the resolve accepted no history yellow. A correct build shows red only inside meshes,
blue in the sky, and yellow only in a 1-texel ring on fast motion.

**Synthetic test (optional; pure Rust, no GPU):** a CPU model of the per-tap rules in `sf_atmos`. Use a 64×32
half-res grid with a vertical tower edge, cloud value 1 in the sky, the 4-frame checkerboard and a Halton ±0.5 px
jitter, run over 16 frames with a 1 px/frame pan. Assert that sky pixels within 3 px of the edge stay at or above
0.95 of the cloud value, and that full-res geometry pixels get exactly 0 cloud. This is only worth it if the WGSL
rules are kept in step with it. Otherwise rely on the visual check.

## Sources

- Schneider and Vos, "The Real-time Volumetric Cloudscapes of Horizon Zero Dawn", SIGGRAPH 2015 Advances:
  <https://www.guerrilla-games.com/read/the-real-time-volumetric-cloudscapes-of-horizon-zero-dawn>,
  [slides](https://www.advances.realtimerendering.com/s2015/The%20Real-time%20Volumetric%20Cloudscapes%20of%20Horizon%20-%20Zero%20Dawn%20-%20ARTR.pdf)
- Schneider, "Nubis, Evolved" (Horizon Forbidden West): <https://www.guerrilla-games.com/read/nubis-evolved>
- Hillaire, "Physically Based Sky, Atmosphere and Cloud Rendering in Frostbite", SIGGRAPH 2016:
  <https://www.ea.com/frostbite/news/physically-based-sky-atmosphere-and-cloud-rendering>,
  [slides](https://www.slideshare.net/DICEStudio/physically-based-sky-atmosphere-and-cloud-rendering-in-frostbite)
- UE5 volumetric render target modes, Hillaire on the Epic forums: <https://forums.unrealengine.com/t/volumetriccloud/688516>
- UE5 bilateral upsampling, `VolumeFrontDepth` (.x cloud, .y scene), threshold "fudge factor", UE-388559:
  <https://forums.unrealengine.com/t/brick-pattern-artifact-in-volumetric-clouds/2835963>,
  <https://issues.unrealengine.com/issue/UE-388559>
- UE Volumetric Cloud component properties (tracing distances):
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/volumetric-cloud-component-properties-in-unreal-engine>
- Toft, Bowles and Zimmermann, "Optimisations for Real-Time Volumetric Cloudscapes" (2016): <https://arxiv.org/abs/1609.05344>
- ChaliceUnity (cloud post-process, furthest-depth downsample): <https://github.com/spencer-melnick/ChaliceUnity>
- Unigine clouds, depth-based reconstruction (cloud depth used for upsampling and reprojection):
  <https://developer.unigine.com/docs/latest/editor2/settings/render_settings/clouds/>
- Bevy 0.20 source: `bevy_render/src/view/mod.rs:1218-1222` (jitter applied to the view uniform only),
  `bevy_render/src/camera.rs:799` (`jitter_projection`), `bevy_pbr/src/atmosphere/functions.wesl:335`
  (`uv_to_ray_direction` via `view.view_from_clip`).
