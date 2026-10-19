# Render order: the frame graph, its rules and its guard

Status: audit + rules (renderorder, 2026-10-19); RO-1, RO-2, RO-3 fixed (rofix1, 2026-10-19). Crates: every `*_bevy` crate that draws;
`sf_render_order_bevy` owns the transparent-band policy and the guard test.

**Why this spec.** On 2026-10-19 the owner found four render-order bugs, each fixed on its own:
blended leaves drawn as sorted translucency (leafmask), translucents never fogged (fogfix,
bevyrrowind `transparent_fog.rs`), rain hidden under the ocean (rainwater: `Transparent3d` sorts by
mesh AABB centre and shader-placed meshes sit at the origin; `DrawBand`), and the froxel fog pulse
(shimmer: one jitter for the whole volume). They are one class: nobody owned the whole frame. This
spec is the one picture of what draws when, what each draw reads, how it is fogged, sorted and
shadowed, and the rules that keep the class from coming back. water.md "Render order" stays the
detail for the transparent side; this spec supersedes it where they differ.

## Frame graph (raster path, exterior camera, as built)

Bevy 0.20 runs the camera graph as the `Core3d` schedule in the render world; our passes are
systems ordered in it. A camera with `SkyCamera` (+ `RenderPathCamera`), `CloudCamera` and the
water plugin gets all of it; interiors skip clouds, fog volume and water.

| # | Pass (system / phase) | Owner | What | Reads | Writes |
|---|---|---|---|---|---|
| 0a | `RenderSystems::PhaseSort`: `apply_bands` before `sort_phase_system::<Transparent3d>` | `sf_render_order_bevy` | rewrites sort keys of `DrawBand`-tagged draws | `ExtractedBands` | phase sort info |
| 0b | shadow views (CSM, budgeted point/spot) | Bevy | every mesh without `NotShadowCaster` whose material casts: opaque and masked (alpha-tested); translucent materials disable shadows (rule 4) | material alpha (discard) | shadow maps |
| 0c | `shadow_pass` (after `Prepass`, before `MainPass`) | `sf_atmos_bevy` | cloud shadow map, toroidal, time-sliced | cloud noise | `DirectionalLightTexture` |
| 0d | `fft_pass` (same slot) | `sf_water_bevy` | ocean FFT cascades | spectrum | displacement arrays (current + previous) |
| 1 | `Core3dSystems::Prepass` | Bevy + material prepass shaders | depth (`DepthPrepass`, for `OcclusionCulling`); motion vectors + jitter when DLSS SR is on (`MotionVectorPrepass`, `TemporalJitter`, `MipBias`, `upscale.rs`); deferred G-buffer only on the Solari path (`path.rs`) | — | depth, motion vectors |
| 2 | `main_opaque_pass_3d`: `Opaque3d`, `AlphaMask3d` (binned) | Bevy | NIF statics/actors, terrain, ground blend, LOD proxies, impostors, sea-plane scaffold | shadow maps, cloud shadow, far shadow map | colour, depth |
| 3 | `render_sky` (private set; `SkyOrderScaffold`) | Bevy `Atmosphere` | sky on depth = far | atmosphere LUTs | colour |
| 4 | `night_pass` (after `render_sky`) | `sf_sky_bevy` | stars, moons; no depth write | depth | colour |
| 5 | `trace_pass` (after opaque, before transparent) | `sf_atmos_bevy` | clouds half-res raymarch + temporal resolve; ground-fog bank march; froxel inject + integrate (per-froxel jitter, own history) | opaque depth, LUTs, shadow maps, own histories | cloud/fog textures, `FroxelVolume` |
| 6 | `composite_pass` (after 3-5) | `sf_atmos_bevy` | clouds + height fog + bank fog + froxels on every **opaque** pixel by its depth | opaque depth, step 5 outputs | colour |
| 7 | `snapshot_pass` (after everything before transparent) | `sf_water_bevy` | **the only scene-colour read**: colour + depth copy for refraction/SSPR | colour, depth | snapshot textures |
| 8 | `main_transparent_pass_3d`: `Transparent3d`, three bands | Bevy + `sf_render_order_bevy` | **Refractive** (water, depth-write) → **Sorted** (back to front by AABB centre + `depth_bias`: blended/additive NIF meshes and particles as `FroxelFogMaterial`, light sprites) → **Overlay** (precipitation boxes, far box first) | snapshot (water), `FroxelVolume`, depth (test) | colour (+ depth for water) |
| 9 | `velocity_pass` (after transparent, before `EarlyPostProcess`) | `sf_water_bevy` | water motion vectors (current vs previous displacement) | depth | motion vectors |
| 10 | post | Bevy | DLSS Super Resolution (raster) or DLSS Ray Reconstruction (Solari); tonemapping (camera default); no bloom, SSAO, SSR, DOF components today | colour, depth, motion vectors | output |
| 11 | UI | Bevy UI | HUD, dev overlays | — | swapchain |

Ray-traced path (`--rt`): step 1 becomes Bevy's deferred prepass + Solari lighting (ReSTIR or DLSS
RR), `Msaa::Off`, no occlusion culling; steps 3-9 are unchanged; raster shadow maps are dropped
(`apply_lights`). `portal_spike.rs` (bevyrrowind) renders a second camera to a texture; spike only.

## Material inventory

"Fog" is how a draw gets fog: *composite* = it writes depth before step 6; *froxel* = its fragment
shader calls `froxel_fog_apply` (`sf_atmos_bevy` `FroxelFogMaterial`); *own* = documented in its
spec. "Shadow" = casts / receives / alpha test in the shadow pass.

| Class (crate) | Phase / band | Placement, sort key | Depth | Prepass (depth, MV) | Fog | Shadow |
|---|---|---|---|---|---|---|
| NIF opaque `StandardMaterial` (`mw_assets`, `sf_world_bevy` spawn) | Opaque3d | world, binned | write | std (depth, MV) | composite | cast / receive |
| NIF masked, incl. cutout foliage (leafmask) | AlphaMask3d | world, binned | write | std, alpha discard | composite | cast with the material cutoff / receive |
| NIF blended (glass, Ghostfence, animated-alpha, foliage on `--leaves blend`) → `FroxelFogMaterial` (app `transparent_fog.rs`) | Transparent3d Sorted | world, AABB centre | test only | none | froxel | none (`FroxelFogExt::enable_shadows` false) / receive |
| NIF additive (glows, flames) → `FroxelFogMaterial` | Sorted | world | test only | none | froxel (dim only) | none (as blended) |
| NIF particles (`mw_assets` `particles.rs`, CPU quads in an emitter-local dynamic mesh) | Sorted | `NoFrustumCulling` ⇒ no AABB ⇒ key = emitter origin (RO-10) | test only | none | froxel (StandardMaterial swap) | none (as blended) |
| Held props (`held.rs`) | as their material | world (character child) | | std | composite | `NotShadowCaster` for **every** light (RO-11) |
| `TerrainMaterial` (`sf_terrain_bevy`) | Opaque3d | world (`NoFrustumCulling` on some chunks: opaque, no sort issue) | write | custom `terrain_prepass.wesl` with MV | composite | cast / receive + far shadow map |
| Ground blend (`StandardMaterial` + `GroundBlendExt`, forward) | Opaque3d / AlphaMask3d | world | write | std | composite | cast / receive |
| LOD proxies (`StandardMaterial` + `FarShadowExt`) | Opaque3d | world, baked AABB (`NoAutoAabb`) | write | std | composite | cast only L1 with `l1_shadows` / far shadow map |
| `ImpostorMaterial` (`sf_lod_bevy`, `Mask(tint.w)`, forward) | AlphaMask3d | world, baked AABB | write | custom `impostor_prepass.wesl` with MV | composite | per LOD settings, alpha-tested |
| `LightSpriteMaterial` (`sf_lod_bevy`, Add) | Sorted | world, baked AABB; billboard grows in the shader inside it | test only | disabled | froxel via `TranslucentFog`, transmittance only | none (`enable_shadows` false) |
| `WaterMaterial` (`StandardMaterial` Blend + `WaterExt`) | Refractive | shader-placed clipmap at the origin, `NoFrustumCulling`, `DrawBand::Refractive` | **write** + test, no culling | none; MV by step 9 | own (froxels, else analytic height fog) | `NotShadowCaster` / receive |
| `PrecipMaterial` (`sf_precip_bevy`, Blend) | Overlay | shader-placed camera-wrapped boxes, `NoFrustumCulling`, `DrawBand::Overlay` | test only | disabled | froxel via `TranslucentFog` (blend) | none |
| Fade overlay (`sf_world_bevy` `fade.rs`, unlit Blend quad, camera child at 0.2 m) | Sorted | camera-local **without a band** (RO-9) | test only | none | host swap (pointless, unlit) | none in bevyrrowind (the swap); casts in a host without it (RO-9) |
| Sea plane scaffold (`exterior.rs`) | Opaque3d | world | write | std | composite | `NotShadowCaster` |

## Rules

1. **Every translucent is fogged through the fog path.** A `Blend`/`Premultiplied`/`Add`/`Multiply`
   material binds `sf_render_order_bevy::TranslucentFog` (the fog owner, `sf_atmos_bevy`, writes it;
   without one it is a clear 1³ volume) and its shader imports `sf_render_order_bevy::translucent_fog`
   (`froxel_fog_lookup` + `froxel_fog_apply`; additive: transmittance only), or fogs
   itself as documented in its spec (water), or is a listed violation. The opaque fog is the
   composite (step 6); nothing translucent may rely on it.
2. **Every shader-placed or camera-local translucent has a `DrawBand`.** Bevy's key is the view depth
   of the mesh AABB centre; without a true AABB (`NoFrustumCulling`, geometry from the shader,
   camera child) the key is the entity origin and the order is arbitrary. World-placed translucents
   keep `Sorted` and must have a true AABB. Never order with a per-material `depth_bias`.
3. **Every opaque or masked surface writes depth in the prepass and motion vectors** when
   `MotionVectorPrepass` is on. A custom vertex shader needs a matching prepass shader computing
   the previous position from the previous transform *and* the previous deformation (terrain,
   impostors and water do; wind-animated foliage must when it lands).
4. **Shadows match coverage.** Opaque/masked casters alpha-test in the shadow pass with the same
   cutoff as the colour pass. Translucents cast nothing unless their class opts in (and then with an
   explicit shadow cutoff). Camera-local and light-proxy geometry never casts. Policy (UE, Unity: translucent materials
   don't cast unless "cast as masked" is set): the opt-out is per material *type*
   (`enable_shadows() -> false`): `FroxelFogExt` (every NIF blended/additive surface and particle after
   the host swap), `LightSpriteMaterial`, `PrecipMaterial`. An opt-in class (e.g. blended foliage
   on `--leaves blend`) needs its own material type with a shadow cutoff in a prepass fragment.
5. **Scene-colour and depth reads happen at named points only.** Colour: `snapshot_pass` (step 7).
   Opaque depth: clouds/fog trace and composite (5-6), water snapshot (7), velocity (9). History
   buffers: clouds, froxels (own), DLSS. A new reader names its point here.
6. **Order between the opaque and transparent passes is declared**, in the table above
   (`render_sky` → night → trace → composite → snapshot). The discovery scaffold
   (`SkyOrderScaffold`) makes it depend on plugin add order (RO-8).

## Industry comparison

| Engine | Order | Us |
|---|---|---|
| UE5 (deferred) | depth prepass (+ velocity for movable) → base pass → shadow depths → lighting → volumetric fog (froxels, compute) → sky atmosphere / volumetric clouds → height fog composite on opaque → **Single Layer Water** → translucency (sorted by bounds origin, per-pixel or per-vertex froxel fog; "after DOF" separate translucency) → TSR/DLSS → bloom → tonemap → UI. Translucent shadows are opt-in (opacity-masked or volumetric). | Same order (forward raster instead of deferred); water before translucents; froxels for translucents. Gap: no translucent velocity/reactive mask (RO-6). |
| Frostbite (Hillaire 2015, "Physically based and unified volumetric rendering") | depth → G-buffer → lighting → froxel fog shared by opaque (composite) and every transparent (per-vertex/pixel fetch) → refractive surfaces first → sorted transparents → particles (often half-res, opacity shadow maps) | Same structure; particles full-res and not in a separate pass. |
| Unity HDRP | depth prepass → G-buffer → lighting → sky/fog → "before refraction" transparents → colour pyramid snapshot → refractive transparents → "after post" | Our snapshot is before *all* translucents (UE-style); translucents under water are invisible through it (accepted, water.md). |
| Decima (Horizon, Schneider 2015/2017) | clouds raymarched at reduced res with temporal reprojection, composited before translucency | Same (sf_atmos). |

## Violations

Severity: **High** visible every play session; **Medium** visible in common conditions;
**Low** rare or scaffold-only. Each has a proposed slice. Fixed ones keep a one-line entry.

- **RO-1** fixed (rofix1, 2026-10-19; was High): blended and additive NIF surfaces and particles cast
  near-opaque shadows (Bevy 0.20's shadow pass discards only alpha < 0.05). `FroxelFogExt::enable_shadows`
  is false, so nothing swapped to `FroxelFogMaterial` casts; masked cutouts (leafmask) keep casting with
  their cutoff. Blended foliage on `--leaves blend` now casts nothing (opt-in material type if wanted);
  `--off transfog` restores the old casting too. Visual check RO-A (bevyrrowind visual-checks).
- **RO-2** fixed (rofix1): `light_sprite.wesl` multiplies by the `TranslucentFog` transmittance to the
  light (no in-scatter); past the froxel range `froxel_fog_lookup` extends it as `T_far^(d/far)`.
- **RO-3** fixed (rofix1): `precip.wesl` applies `froxel_fog_apply` (blend) per fragment.
- **RO-4** (FIXED 2026-10-19 ro48: `sf_atmos_bevy::FogTranslucentsPlugin` holds the swap, bevyrrowind adds it unless `--off transfog`; it covers any blended `StandardMaterial` of a host that adds it; was: fog for translucents lives in the host (`transparent_fog.rs` swap); the
  framework's own generic spawn (`sf_world_bevy` `spawn.rs`) emits blended `StandardMaterial`s that
  any other host draws unfogged. *Slice*: move the swap (it has no game knowledge) into
  `sf_atmos_bevy` as a `FogTranslucentsPlugin`; bevyrrowind keeps only the `--off transfog` A/B.)
- **RO-5** (Medium, VERIFIED 2026-10-19 by code: `FroxelVolume.params` doc in `transparent.rs`, `doors.rs` `interior_look` sets only ambient + clear colour; no fog pass runs in interiors): interiors have no froxel volume (`FroxelVolume.params.w = 0`), so no
  translucent is fogged there, and opaque interior fog is the clear colour only (`doors.rs`).
  *Slice*: an interior fog path (analytic distance fog in the composite, its parameters in
  `FroxelVolume` for the translucent fallback).
- **RO-6** (Medium): translucents write no motion vectors and no reactive/transparency mask, so
  particles, rain and sprites ghost under DLSS. *Slice*: DLSS bias-current-colour mask from the
  Overlay/Sorted bands if Bevy's DLSS integration exposes it, else motion vectors for precipitation
  via a redraw like water's step 9.
- **RO-7** (Medium, VERIFIED by code 2026-10-19: no sf_atmos_bevy/sf_sky_bevy pass writes a motion-vector target; sky + cloud resolve write colour only, so MV = prepass clear = 0 on sky pixels; not captured via BRP, DLSS needs the far-depth pass below): sky and cloud pixels may carry zero motion vectors (Bevy's prepass
  clears to 0; Bevy's `Skybox` has a motion-vector prepass, `Atmosphere`'s `render_sky` is not known
  to): DLSS would reproject the sky as screen-fixed and smear clouds on rotation. *Slice*: check with
  a BRP capture of the MV texture; if so, a fullscreen far-depth camera-motion pass after step 1.
- **RO-8** (FIXED 2026-10-19 ro48: `sf_render_order_bevy::add_pass` + `PassStage` {Night, Clouds, Snapshot}, ordered by set, test for all add orders; was: `SkyOrderScaffold` is copied in three crates (`night.rs`,
  `sf_atmos_bevy` `render.rs`, `sf_water_bevy` `render.rs`) and the resulting order depends on
  plugin add order (Sky → Clouds → Water); a wrong order silently reorders steps 4-7. *Slice*: one
  helper in `sf_render_order_bevy` plus a startup check that asserts the declared order; delete it
  when Bevy names the `render_sky` set.)
- **RO-9** (FIXED 2026-10-19 ro-small: quad now has `DrawBand::Overlay { order: i16::MAX }` + `NotShadowCaster`; the host-fog row stays, unlit so pointless; was: the fade overlay is camera-local without a band (precipitation's Overlay
  band draws over the black fade) and casts a shadow while fading. *Slice*: `NotShadowCaster` +
  `DrawBand::Overlay { order: i16::MAX }`, or a screen-space fade when M8 replaces the scaffold.
- **RO-10** (accepted limit 2026-10-19, Bevy has no per-draw sort key beyond the AABB centre/`TransparentSortingInfo3d`; revisit with the AABB slice if an ash storm mis-sorts): particles sort by the emitter origin (no AABB). Fine for small emitters; large
  ones (ash storms, spell volumes) can mis-sort against windows and leaves. *Slice*: an AABB from
  the simulation's live bounds each frame (UE: fixed per-emitter bounds) instead of
  `NoFrustumCulling`.
- **RO-11** (DEFERRED 2026-10-19: cause is that the torch is a shadow-casting point light inside the prop and Bevy has no per-light caster mask; `NotShadowCaster` is the only way to stop self-shadowing, a small fix needs upstream masks or `RenderLayers` light partitioning): held-prop meshes get `NotShadowCaster`, which in Bevy is for every light, so a
  carried torch's prop casts no sun shadow either. *Slice*: per-light caster masks upstream, or limit
  it to the sub-mesh around the flame.
- **RO-12** (accepted): translucents under the water are absent from the refraction snapshot (UE has
  the same limit; water.md).

## Guard

`superfargoth/crates/sf_render_order_bevy/tests/translucent_guard.rs` (plain `std`, runs with
`cargo test -p sf_render_order_bevy`). It scans `crates/*_bevy/src` for translucent alpha modes and
holds a `SITES` table (file, fog treatment, placement, violation id). It fails when:

1. a file gains a translucent site that isn't declared, or a declared site is gone;
2. a shader-placed or camera-local site has no `DrawBand::` and no violation id, or a file spawning
   `NoFrustumCulling` with a translucent is declared world-placed;
3. a `Froxel` site's crate doesn't bind `TranslucentFog` and call `froxel_fog_lookup` from the
   `translucent_fog` module, an `Own` site's crate never references the froxel volume, or an unfogged
   (`Host`/`Missing`) site lacks a violation id listed in this spec;
4. a site's own `Material`, or `sf_atmos_bevy`'s `FroxelFogExt`, doesn't return `false` from
   `enable_shadows` (rule 4).

Adding a translucent class: give it fog (rule 1) and, if shader-placed or camera-local, a band
(rule 2), then add its row here and in `SITES`. Fixing a violation: change its row to the fixed
state, drop the id, delete it here.

**Host-side runtime check (specified, not built).** The app's NIF path builds materials at runtime,
so a source scan can't see it. A `dev`-feature system in bevyrrowind, every 60 frames: warn (once per
entity) when an entity still draws a blended `MeshMaterial3d<StandardMaterial>` two frames after
spawn (the fog swap missed it), when a translucent entity has `NoFrustumCulling` and no `DrawBand`,
and when a blended/additive entity draws a `StandardMaterial` that still casts (the swap missed it, RO-1).
