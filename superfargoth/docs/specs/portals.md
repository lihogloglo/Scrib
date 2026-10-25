# Portals: seamless door links (M8)

> **2026-10-19:** the app-side spike (`bevyrrowind/apps/bevyrrowind/src/portal_spike.rs`, `--portals`) was deleted
> in the phase-0 cleanup (owner go); recover it from git history. This spec and its findings stay the design for M9.

Status: **spike done (2026-10-12)**, owner findings fixed (PT-2..PT-6, awaiting owner); **parked until M8**: seamless
portals are an optional setting, off by default (`--portals` or `BEVYRROWIND_PORTALS=1`; classic fade doors stay the
default, owner decision 2026-10-12). Production design below. The spike is
bevyrrowind's scaffold `portal_spike` (`--portals`, `apps/bevyrrowind/src/portal_spike.rs`);
this spec is what M8 builds in the framework.

Goal (WISHLIST.md §6): an interior stays its own worldspace; its door is a
portal to the exterior door it links to. Each side renders the other through
the opening, both ways, and walking through re-parents the player through
the link transform (`DoorLink::cross`) with no fade. Spaces never overlap,
so "bigger on the inside" just works.

## Industry check

- **Valve, Portal / Portal 2** (Kircher & Lawrence, "Portal Problems",
  GDC 2008; Valve's later portal-rendering notes): recursive stencil
  portals rendered in the main pass, an oblique near plane on the exit
  plane (Lengyel 2005), the teleport when the player's center crosses the
  plane, and special care for the camera's near plane crossing the portal
  (they clip the player's view against the portal and push the camera).
  They compare stencil vs render-to-texture: texture is simpler but costs a
  full extra scene per portal; stencil is cheaper for nesting.
- **Render-to-texture kits** (UE `SceneCaptureComponent2D` + render target,
  Unity seamless-portal assets, Sebastian Lague's "Coding Adventure:
  Portals" 2020): a second camera at the crossed pose renders to a texture,
  the portal surface samples it in screen space, oblique clip at the exit
  plane, a thickened portal mesh so the near plane never cuts the opening,
  teleport a little before the plane. Known UE pitfalls: the capture runs
  outside the main view's TAA/motion-vector history unless its state
  persists ("always persist rendering state"), the crossing frame is a
  camera cut for temporal effects, and capture cost is a full scene render.
- **Prey (2017) / Dishonored**: one continuous station/city, but transit
  doors still load sections; no see-through portals between spaces.
  Bethesda (Morrowind to Starfield) keeps load doors. Seamless open worlds
  (Witcher 3, Cyberpunk) put interiors inside the exterior partition,
  which Morrowind's larger-inside interiors rule out.
- **Bevy**: no portal feature, but Bevy 0.20 ships every piece: render
  layers per camera, `RenderTarget::Image`, `PerspectiveProjection::near_clip_plane`
  (oblique clip, Lengyel 2005, added for portals and mirrors) and the
  `mirror` example (second camera, screen-space sampled material, oblique
  plane). **Crate**: `bevy_basic_portals` 0.5.0 (April 2024, Bevy 0.13 era,
  last release; NVPL v7+ license, not OSI): stale and license-incompatible
  with superfargoth. Reuse order result: Bevy's pieces, our own thin plugin.

Moved to [history](archive/portals-history.md#spike-findings-census-office-2026-10-12).

## Last fix pass (portal3, 2026-10-12) and parking

Owner pass on portal2's build: (1) from inside, no terrain through the door; (2) still a flash frame at the
crossing; (3) exposure compensation "too intense but doing its job".

**(1) Terrain from inside.** The main camera drops `TerrainCamera` indoors and the terrain's render origin followed
the *active* worldspace. Now the portal camera carries `TerrainCamera` while it looks out at the exterior from another
worldspace, and the app's `sync_origin` takes the origin of the root the terrain camera stands in (the exterior's,
held by its `OriginProxy` while the player is inside), else the active one's; the sea scaffold follows that camera.

**(2) Crossing flash.** `portal trace` now also logs the main camera's render-space position, terrain/cloud
camera, portal camera and box, a few frames *before* each crossing, and the presented frame's mean sRGB luma with
white/black shares (`portal luma`, from a screenshot readback, so it covers raster and Solari alike). Raster walk at
noon, walking in: luma 21.8 (through the portal), **177.5 on the crossing frame**, 38.5 after. Cause: the sky's
`setup_sky_cameras` (in `SkySystems::Apply`, Update) was unordered with `doors::worldspace_look`, so it saw the
`SkyCamera` removal a frame late: the crossing frame drew the office with the noon atmosphere and its environment
light. Fix: `worldspace_look` runs before `SkySystems::Apply` (also helps fade doors). After: 21.8, 38.8, 39.0 (no
flash frame). Walking out the luma goes 44.7, 148, 148, 235 (bright to brighter: the new sky camera's environment
map fills in over two frames, then the office-adapted eye sees the noon street blown out and adapts).

**(3) Softer adaptation, live strength.** The compensation curve is scaled by a strength `s` about the lit
interior's point: exposure correction `E_s = E_int + s (E - E_int)`, `E = compensation - x` (Bevy's target).
`s = 1` is portal2's curve; **default 0.7**; speeds 6/4 -> **3/2 EV/s** (brighten/darken). Lower strength = a
smaller doorway swing at the cost of brighter days and darker nights than the sky schedule (the trade UE's min/max
EV limits make). Keys `-` / `=` change it by 0.1 (0-1.5) and print the resulting steady-state EV100 of a night
street, a lit interior and noon; F1 shows it. The owner picks the default.

**Parked for M8** (not fixed in the spike): motion vectors in the opening (TAA/DLSS reproject it by the box);
the void when turning just past the door (the 0.13-0.26 m band behind the far plane: thick portal or stencil
sub-view); a ~1.3 EV level step at the crossing looking in (luma 21.8 through the portal vs 38.8 direct, same
exposure: the portal camera's office renders darker than the main camera's; suspects: per-view effects the portal
camera lacks or a different ambient/fog path); the new sky camera's environment map built from scratch (share the
portal camera's); cost of the portal view (exterior side unmeasured) and of grounding/auto exposure; the real wall
hole from the frame geometry; `AutoExposure` for all views; Solari's acceleration structure ignores render layers
(other worldspaces' geometry may shadow/light in ray-traced mode, unverified).

## M8 design (recommended)

1. **Bake**: fill `LinkEnd.frame`/`extent` (kept door, swing) for
   portal-ready and adjust links (`mw_world::portal`, 0.02 s for all
   links): the grounded common opening (`floor_offset` + `common_opening`,
   the spike runs them lazily); later the real wall hole from the frame
   geometry.
2. **`sf_portal_bevy`** (framework, Bevy-only thin plugin): `PortalView`
   pool (1 by default, quality preset up to 2), pick by view direction and
   screen area; camera per view as above (HDR, no tonemapping, oblique
   clip, far root's floating origin, the viewer's base exposure); opening
   box at the door's mid-plane; the position-only `Gate` crossing rule
   (symmetric trigger, no cooldown); `TravelArrived { seamless }`; history
   reset at the crossing; a thick portal or stencil sub-view for the band
   just behind the far plane.
3. **Per-view look** hooks for adapters (ambient, fog, sky) instead of the
   app's single-camera `worldspace_look`.
4. **Environment blending** (WISHLIST §6): ambient/fog/reverb volumes
   interpolate across the threshold; exposure is the viewer's: one base
   EV100 for every camera and `AutoExposure` (compensation curve) on the
   player camera, for every worldspace (not only portals).
5. Keep `FadeTransition` for fade-only links (one-way, trapdoors).

## Industry comparison

J9 (industry-audit.md), 2026-10-12, docs research only (`.sessions/j9rest.md`).
"Industry check" above is the research; this gives verdicts and adds what
it didn't cover. "Unverified" marks claims from memory not re-read.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| See-through rendering | Valve stencil portals (nesting) vs render-to-texture (UE `SceneCapture`, Unity kits) | render-to-texture view pool, oblique clip (Bevy `near_clip_plane`) | **match** (RTT; stencil sub-views are an upstream wish) |
| Crossing | teleport at the plane, thickened portal mesh, camera cut for temporal effects | position-only `Gate`, history reset, thick portal band | **match** |
| Light confinement per space | separate scenes/levels per capture; UE lighting channels (unverified) | render layers; light probes leak (upstream) | **partial**, known (upstream wishlist) |
| Exposure across spaces | UE/Frostbite pre-exposure + histogram auto exposure | base EV100 + `AutoExposure` compensation curve | **match** in effect; pre-exposure is an upstream wish |
| Seamless interiors in open worlds | Witcher 3, Cyberpunk, RDR2: interiors inside the exterior partition (no separate space); Bethesda: load doors | separate worldspaces joined by portals | **differ**, forced by content (interiors larger than outside) |
| Interior visibility (rooms + portals culling) | Source/id PVS, Unreal/Bethesda room bounds + portals (Fallout 4 room bounds, unverified), UE precomputed visibility | n/a now (interiors small, separate) | already audit row 31 (P2, M8) |
| **Sound through the opening** | Wwise Spatial Audio "Rooms and Portals" (sound from another room reaches the listener through portal openings, with diffraction and room reverb); Steam Audio, UE Audio Gameplay Volumes / submix sends (unverified detail) | not covered: per-space reverb blending only (M8 design 4) | **GAP (P2, M8)**: with a door open, the far space's ambience (rain, wind, a tavern) should be audible, attenuated, from the opening's position. Item in the M8 design to add when the audio spec exists: one emitter per portal that plays the far space's ambience bed at the opening, gain by opening solid angle |

**Sources:** Audiokinetic, Wwise Spatial Audio, Rooms and Portals:
https://www.audiokinetic.com/en/library/edge/?source=SDK&id=spatial_audio_roomsportals.html
(not re-read; unverified URL); the rest is cited in "Industry check".

## Upstream wishlist (Bevy)

- `RenderLayers` on light probes.
- Per-view timing labels for secondary cameras (diagnostics).
- Portal/stencil sub-views sharing the main view's target, depth and
  temporal history; or a way to feed a secondary view's motion vectors
  into the main prepass.
- Shared directional shadow maps across views of the same worldspace.
- Pre-exposure for `AutoExposure` (scale the HDR target by last frame's
  exposure, as UE and Frostbite) so a fixed base EV100 isn't needed for
  fp16 range; a compensation curve indexed by absolute scene EV.
