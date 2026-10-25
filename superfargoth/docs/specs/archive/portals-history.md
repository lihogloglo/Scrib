# Portals history

Dated history blocks moved verbatim from [portals.md](../portals.md) (doc cleanup S10b, 2026-10-26).

## Spike findings (Census Office, 2026-10-12)

Pair: interior `seyda neen, census and excise office` door at sill
(16.73, 2.28, 10.74), opening 2.08 × 3.57 m, ↔ exterior `chargendoorjournal`
at (−147.07, 2.10, 1023.54), 1.64 × 2.81 m (ratio 1.27). The office has a
second exterior door (sill z 3.44) and a warehouse door 3.6 m from the main
door outside: a portal pick must use the view direction, not distance alone.
Screenshots: `target/shots/portal/{outside_in,inside_out}.png`.

What works: looking in from the street shows the lit office through the
door (its own ambient, lights, exposure, fog color); looking out shows
Seyda Neen under the sky; the smaller slab hides, the bigger swings (when
its origin sits at an edge) or hides; crossing re-parents the camera with
no fade, keeping its heading and velocity. Frames for every link are
filled at startup: **1,612 of 1,828 links** get frames (portal-ready +
adjust) in **0.02 s** (door bounds load fast), so the bake step is cheap.

### 1. Rendering approach

Render-to-texture, one portal camera per open portal: `Camera3d`,
`order −1`, `Hdr`, `Tonemapping::None`, `DebandDither::Disabled`,
`RenderTarget::Image` (Rgba16Float, the main camera's target size), render
layers = the far worldspace's layer only, pose = `DoorLink::cross(near,
far, main pose)`, projection = the main camera's with `near_clip_plane`
on the far door plane (normal = far frame +Z in view space, w = −n·q, 2 cm
bias toward the camera so the frame trim stays). The opening is a quad on
the slab's front face with a material that samples the image at
`(frag.xy − viewport.xy) / viewport.zw`, so it matches the main view's
pixel grid, also under DLSS (the material runs at the render resolution).

Stencil vs texture: stencil (render the far worldspace into the main pass
inside the opening's stencil mask) avoids the second target and shares the
main view's TAA/DLSS history, but Bevy 0.20 has no per-view stencil-masked
sub-pass, no way to draw a second set of render layers inside one view,
and its lights, clusters, environment maps and exposure are per view, not
per mesh. Texture is what Bevy supports today. Upstream would need: a
"portal view" concept (sub-view sharing the main view's target and depth,
masked by stencil, with its own layers, lights, clusters, exposure).

**Finding: Morrowind door slabs lie on solid walls.** There is no hole
behind the door model (the exterior shell's wall is right behind the
slab), so a portal surface behind the door plane is hidden by the wall.
The quad goes on the slab's front face (`thickness/2`, capped at 0.15 m);
the crossing happens 0.15 m in front of it, before the main camera's near
plane (0.1 m) reaches the quad. After the crossing, the camera stands just
behind the far plane, where the far walls' backs are culled (one-sided).

Oblique clip: works as documented. Bevy's frustum culling derives from the
projection, so the oblique plane also culls what's behind the far door.
Degenerate when the virtual camera reaches the plane: the spike falls back
to the plain near plane for that frame.

### 2. Light confinement (read from Bevy 0.20 source, confirmed visually)

| Feature | Per worldspace? | How |
|---|---|---|
| Directional lights | yes | `prepare_lights` keeps only lights whose `RenderLayers` intersect the view's (`bevy_pbr/src/render/light.rs`); shadow casters filtered by the light's layers too |
| Point/spot lights | yes | clustering skips lights whose layers don't intersect the view (`bevy_light/src/cluster/assign.rs`) |
| Ambient | yes | `AmbientLight` component on the camera overrides `GlobalAmbientLight` |
| Environment map / atmosphere | yes | per camera (`AtmosphereEnvironmentMapLight`, `EnvironmentMapLight` on the camera) |
| Fog, exposure, tonemapping | yes | per camera components |
| Light probes (reflection probes, irradiance volumes) | **no** | world-space volumes with no layer check: a probe in an interior would light the exterior at the same coordinates. Upstream: `RenderLayers` on light probes (we don't place probes yet) |

Pitfall: a light on the shared layer 0 lights every camera (the view-light
test is an intersection), so the sun must be on the exterior's layer
only. The spike does that (`sun_layers`). Found and fixed on the way: the
sky leaves an `AmbientLight` override on the camera, so interiors used the
sky's ambient instead of the cell's (`doors.rs` now inserts the cell's).

### 3. Exposure, tonemapping, temporal effects

Each side renders with its own exposure into linear HDR and the main
camera tonemaps once; seams don't show. This "per-space exposure" reads
well but isn't physical: a viewer standing in a dark office should see the
street blown out. M8: exposure belongs to the viewer: the portal camera
copies the main camera's `Exposure`, and Bevy's `AutoExposure` on the main
camera (metering the final HDR target, which contains the portal pixels)
adapts as you approach; per-space exposure only as a clamp/bias.

Temporal: the opening's motion vectors are the quad's, not the far scene's,
so TAA/DLSS reprojection inside the opening is wrong while moving (ghosting,
soft). The portal camera itself has no history (it gets DLSS too when it is
a `SkyCamera`, a second DLSS context). The crossing frame is a camera cut:
the spike doesn't reset history yet (`TemporalHistory::reset` exists, as
for origin shifts). M8: write the portal pass's own motion vectors (far
camera's previous view-projection) into the main prepass inside the
opening, or render the portal camera at the main camera's render resolution
with jitter and let DLSS see the composite; reset history at the crossing.

### 4. Cost (1440p, RTX 4060 Laptop, raster + DLSS auto, release)

Street camera 5 m from the Census door looking in (portal view = the
interior), interleaved A/B, two pairs: **median 8.41 → 9.91 / 10.06 ms
(+1.5 to +1.65 ms)**; render-graph CPU +0.5 to +1.4 ms, main-world CPU
≤ +1.4 ms. Bevy's pass timings don't list the second view's passes (the
same timed total), so GPU cost shows as frame time and acquire wait. The
first measurement pair (19.7 → 23.7 ms) ran while another session built
and is discarded. Looking out from inside (portal view = the exterior with
sky, atmosphere LUTs, environment map, shadows) was not measured: the
benchmark mode holds the warm-up forever in a start interior. Expect it to
cost roughly a full exterior frame minus DLSS; the atmosphere environment
light per camera is the item to share.

Cost levers for M8: render the portal view only when the opening is on
screen and its screen area is > a threshold; clamp its viewport to the
opening's screen rectangle (`Camera::viewport` + `SubCameraView`); share
the sun's shadow maps between views (Bevy re-renders cascades per view);
lower its resolution with distance; one portal view at a time.

### 5. Crossing

`DoorLink::cross` with sill-origin frames maps the pose exactly; with each
end keeping its own opening, the 1.27 ratio shows as extra margin (from
inside, the larger opening shows the street around the small exterior
door: walls are one-sided, so nothing occludes) rather than a clamped
hole. Crossing within the larger opening but outside the smaller lands in
the far wall: the M8 collider must block the margin (or clamp the opening
to the shared minimum, as the frames proposal says).

Camera continuity: the spike rotates `FreeCameraState`'s yaw/pitch and its
velocity with `cross_direction` directly; `TravelArrived` (fade path) zeroes
velocity, so M8 needs `TravelArrived { seamless: true }` or a velocity
transform in the message. The `Player` anchor follows the camera into the
other worldspace on the next frame (`follow_player`).

Floating origin: each worldspace root keeps its own origin; the portal
camera lives in the far root next to its `OriginProxy` (Census sits within
1 km of the origin, so precision was fine). M8: the portal camera takes the
far root's `FloatingOrigin` (world-coordinates.md), swapped with the main
camera at the crossing in one command flush. The terrain's render origin
follows the *active* worldspace's origin cell, so terrain seen through a
portal from inside is only right when both origins share a cell: M8 makes
`TerrainOrigin` the exterior root's origin, not the active worldspace's.

Floors: from the street the opening's bottom ~0.3 m shows the interior's
clear color (the interior floor sits above the far door's sill). The bake
should ground the sill on the floor (raycast) rather than the slab bottom.

## Owner findings and fixes (PT-1 → PT-2..PT-5, 2026-10-12)

The owner played PT-1 and reported: (a) a gap with a little void, the two
sides not to scale or aligned; (b) a big white flash at the crossing;
(c) exposure must be physical: from the street at noon the office is dark,
from inside the street is bright, different again at night; (d) once
inside, you can't go back out through the same door, especially walking
backwards. Root causes and fixes (scaffold `portal_spike`, `--portals`):

**(a) Gap and void.** Two causes. *Scale/margin*: each end kept its own
opening (office 2.08 × 3.57 m, street 1.64 × 2.81 m, ratio 1.27) under a
rigid transform, so the bigger side showed a margin of the other world
around the smaller door, and the smaller side cropped the bigger one.
*Void*: the office door slab reaches below the floor, so the frame's sill
(slab bottom) sits under the office floor; rays through the bottom of the
street-side opening went under the far floor and hit nothing (the fog
colored strip, ~0.3 m). Fix: **grounded common opening**
(`mw_world::portal::{floor_offset, common_opening}`): each end's sill is
raised onto the floor found by two downward rays 0.3 and 0.6 m in front of
the door against the visible triangles of the cell's statics, activators
and containers within 30 m (accepted within ±0.6 m of the slab's sill;
none found keeps the slab sill, e.g. a door on terrain); then both ends
get the per-dimension minimum opening, centered on the slab and standing
on the floor. With equal openings and a rigid crossing, each opening's
corners map exactly onto the other's (unit test
`floor_probe_finds_a_raised_floor_and_the_common_opening_aligns`: corner
error < 0.1 mm, so the projected corners coincide on screen from both
sides). The opening's box moved from the slab front (≤ 0.15 m out) to
1 cm in front of the mid-plane, still in front of the wall face (half a
slab back), which shrinks the parallax gap at its edges and lets the
crossing plane move to 0.13 m. In the spike the grounding runs once, when
a portal first opens (logged with its floor offsets and time); M8 bakes it.
The office shell has a real hole the size of its (bigger) slab, so a swung
slab left a void ring around the smaller common opening (seen in the
second screenshot): a slab bigger than the common opening now *stays
closed* (`DoorMove::Stay`), the opening's box sits on its front face, the
portal view is clipped just in front of that face from the other side,
and the slab hides while the camera stands behind it (right after
arriving). Doors the opening's size swing; the smaller slab hides. Next
step: the real hole from the frame geometry (the wall's opening edges)
instead of the slab bounds.

**(b) White flash.** Found from the systems' frame order (command flushes);
the new `portal trace` log (6 frames after each crossing: active
worldspace, main camera EV100, sky camera, auto exposure, ambient, clear
color, sun illuminance and layers) shows the fixed crossings clean: 4
scripted crossings, EV100 7 and the sun on the exterior layer only in
every traced frame. Two one-frame bugs: (1) walking *in*,
`doors::worldspace_look` queued the sun and moon onto the *office's*
render layer (it did so for any worldspace, "the worldspace that shows the
sky"); the spike's `sun_layers` corrected them only on the next frame, so
the crossing frame lit the office with the ~100 klx noon sun at EV100 7:
white. (2) walking *out*, the camera became a `SkyCamera` again and
`sf_sky_bevy::setup_sky_cameras` inserted `Exposure::default()` (EV100
9.7) after `apply_cameras` had written the schedule (EV100 ~14 at noon):
one frame 4+ stops over: white. Fixes: the sun moves only to worldspaces
that show the sky; a new sky camera gets the scheduled EV100 at once
(framework fix, all doors); and with viewer exposure (c) no per-space
exposure switches at a crossing at all. The crossing also resets the
temporal upscaler's history (`TemporalHistory::reset`: a camera cut).
Not changed: a camera that becomes a sky camera builds its atmosphere
environment map from scratch (the first frame's may be dark): a portal
camera already renders the exterior then, M8 should share it.

**(c) Exposure: the viewer's, one physical light scale.** Calibration
check: the content already shares one scale. Sun 130 klx above the
atmosphere (~100 klx on the ground at noon, EV100 schedule ~14), interiors
lit to "fully lit" white at EV100 7 (`LightCalibration`: 8,000 lm per m²
of radius, ambient 300 nits for a white cell ambient), night EV100 ~0.5.
Real-world EV100 tables: sunlit 15, home interiors 5–7, offices 7–8, full
moon −3 to −2. So interior vs noon differs by 7 stops (real 7–10) and our
nights are ~3 stops brighter than physical (the moon at 1 lx is a
deliberate readability choice, sky-lighting.md). No calibration change
was needed; what was wrong is that each camera had its own exposure, so
the office through the door looked as bright as the street. Now
(default with `--portals`; `--scheduled-exposure` keeps the old behavior
for A/B): both cameras render at one fixed base exposure, EV100 7 (the
portal camera's HDR image is linear and on the same scale, no own
exposure or tonemapping), and the main camera carries Bevy 0.20's
`AutoExposure` (histogram of the final HDR frame, opening included; the
darkest and brightest 10% ignored) which applies the correction in its
tonemapping pass. Settings: histogram range −16..10 (log2 of the
base-exposed average; scene EV = x + 7, so EV −9..17 is the min/max
adaptation), compensation curve (UE's "exposure compensation curve" by
average scene EV) −4.5 EV for a night street (x ≤ −11), −3.6 for a lit
interior (x = −3; −3 read ~0.6 EV bright at noon), −2.3 (mid grey) in daylight (x ≥ 2): partial adaptation,
so nights stay dark and interiors land on the old EV100 7, noon on ~14.
Speeds: 6 EV/s into a brighter scene, 4 EV/s into a darker one
(exponential near the target): a noon doorway's 7 stops adapt in ~1.5 s.
Why a fixed base and not the schedule: Bevy's `AutoExposure` meters the
HDR target already scaled by `Exposure`, and its compensation curve is
indexed by that metered value, so the base must not vary (or the curve
would mean different scene levels at different hours); EV100 7 keeps the
noon street (~200 in linear HDR) and moonlit nights (~1e-4) inside fp16
precision. UE and Frostbite solve the same range problem with
*pre-exposure* (scaling the scene buffer by last frame's exposure); Bevy
0.20 has none: upstream wish. Expected look: from the street at noon the
office is dark (7 stops under, plus the 10% dark cut), stepping in it
brightens over ~1.5 s; from inside at noon the street through the door
blows out; at night the office glows from the street and the street is
dark from inside. Owner checks PT-2..PT-4.

**(d) Can't get back out.** Three causes: (1) the "near" rule that
opens a portal whatever the camera faces measured the distance to the
*sill* (≥ 1.6 m for an eye), never under its 1 m threshold, so walking
backwards (facing away) never opened the return portal; (2) the camera
crossed 0.3 m in front of the near opening and so arrived 0.3 m *behind*
the far door's plane, where the return portal needed the camera in front
(z > 0) to open and to cross from: stepping straight back moved it further
behind, a trap; (3) the trigger was not symmetric between the two sides.
Fix: `Gate`, the crossing rule by position only (not facing or velocity):
the camera crosses when its local z in the open end's frame goes from at
or above the trigger to below it inside the opening; the trigger is
0.12 m in front of the end's box on an approach (0.13 m at the street
door, 0.26 m at the office's closed slab) and, right after a crossing,
the same physical plane seen from the other side (negated), so the arrival is never past
the return trigger and stepping back crosses back; it returns to 0.13 m
once the camera stands beyond. No cooldown. Near portals open within 2 m
of the opening's rectangle whatever the camera faces. Unit test
`crossings_by_position_both_ways_forward_backward_and_strafing` (forward
in, backwards out, through and straight back three times, strafing in and
out, passing beside the opening and along the plane never cross), and
`--portal-walk` walks through and back twice in the app (test scene
`portal-walk`, PT-5). Known gap: in the 0.13-0.26 m band just behind the far
door's plane after arriving, the return opening is hidden; turning
round inside that band shows the far world behind its door wall. A thick
portal (or stencil sub-view, M8) closes it.

Still open from the spike: motion vectors inside the opening (TAA/DLSS
reproject the opening by the box's motion, not the far scene's); cost of
the portal view with the exterior on the far side (the spike's bench
hangs in a start interior); both are M8 items, measured when the machine
is free.

