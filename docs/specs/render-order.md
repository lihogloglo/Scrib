# Render order: the frame graph, its rules and its guard

Status: audit + rules (renderorder, 2026-10-19). Crates: every `*_bevy` crate that draws;
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
| 0b | shadow views (CSM, budgeted point/spot) | Bevy | every mesh without `NotShadowCaster`, opaque, masked **and blended** (see RO-1) | material alpha (discard) | shadow maps |
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
| NIF blended (glass, Ghostfence, animated-alpha, foliage on `--leaves blend`) → `FroxelFogMaterial` (app `transparent_fog.rs`) | Transparent3d Sorted | world, AABB centre | test only | none | froxel | **casts, discard only a < 0.05** (RO-1) / receive |
| NIF additive (glows, flames) → `FroxelFogMaterial` | Sorted | world | test only | none | froxel (dim only) | **casts** (RO-1) |
| NIF particles (`mw_assets` `particles.rs`, CPU quads in an emitter-local dynamic mesh) | Sorted | `NoFrustumCulling` ⇒ no AABB ⇒ key = emitter origin (RO-10) | test only | none | froxel (StandardMaterial swap) | **casts** (RO-1) |
| Held props (`held.rs`) | as their material | world (character child) | | std | composite | `NotShadowCaster` for **every** light (RO-11) |
| `TerrainMaterial` (`sf_terrain_bevy`) | Opaque3d | world (`NoFrustumCulling` on some chunks: opaque, no sort issue) | write | custom `terrain_prepass.wesl` with MV | composite | cast / receive + far shadow map |
| Ground blend (`StandardMaterial` + `GroundBlendExt`, forward) | Opaque3d / AlphaMask3d | world | write | std | composite | cast / receive |
| LOD proxies (`StandardMaterial` + `FarShadowExt`) | Opaque3d | world, baked AABB (`NoAutoAabb`) | write | std | composite | cast only L1 with `l1_shadows` / far shadow map |
| `ImpostorMaterial` (`sf_lod_bevy`, `Mask(tint.w)`, forward) | AlphaMask3d | world, baked AABB | write | custom `impostor_prepass.wesl` with MV | composite | per LOD settings, alpha-tested |
| `LightSpriteMaterial` (`sf_lod_bevy`, Add) | Sorted | world, baked AABB; billboard grows in the shader inside it | test only | disabled | **none** (RO-2) | none (`enable_shadows` false) |
| `WaterMaterial` (`StandardMaterial` Blend + `WaterExt`) | Refractive | shader-placed clipmap at the origin, `NoFrustumCulling`, `DrawBand::Refractive` | **write** + test, no culling | none; MV by step 9 | own (froxels, else analytic height fog) | `NotShadowCaster` / receive |
| `PrecipMaterial` (`sf_precip_bevy`, Blend) | Overlay | shader-placed camera-wrapped boxes, `NoFrustumCulling`, `DrawBand::Overlay` | test only | disabled | **none** (RO-3) | none |
| Fade overlay (`sf_world_bevy` `fade.rs`, unlit Blend quad, camera child at 0.2 m) | Sorted | camera-local **without a band** (RO-9) | test only | none | host swap (pointless, unlit) | **casts while fading** (RO-9) |
| Sea plane scaffold (`exterior.rs`) | Opaque3d | world | write | std | composite | `NotShadowCaster` |

## Rules

1. **Every translucent is fogged through the fog path.** A `Blend`/`Premultiplied`/`Add`/`Multiply`
   material fetches the froxel volume (`froxel_fog_apply`; additive: transmittance only), or fogs
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
   explicit shadow cutoff). Camera-local and light-proxy geometry never casts.
5. **Scene-colour and depth reads happen at named points only.** Colour: `snapshot_pass` (step 7).
   Opaque depth: clouds/fog trace and composite (5-6), water snapshot (7), velocity (9). History
   buffers: clouds, froxels (own), DLSS. A new reader names its point here.
6. **Order between the opaque and transparent passes is declared**, in the table above
   (`render_sky` → night → trace → composite → snapshot). The discovery scaffold
   (`SkyOrderScaffold`) makes it depend on plugin add order (RO-8).

## Industry comparison

| Engine | Order | Us |
|---|---|---|
| UE5 (deferred) | depth prepass (+ velocity for movable) → base pass → shadow depths → lighting → volumetric fog (froxels, compute) → sky atmosphere / volumetric clouds → height fog composite on opaque → **Single Layer Water** → translucency (sorted by bounds origin, per-pixel or per-vertex froxel fog; "after DOF" separate translucency) → TSR/DLSS → bloom → tonemap → UI. Translucent shadows are opt-in (opacity-masked or volumetric). | Same order (forward raster instead of deferred); water before translucents; froxels for translucents. Gaps: translucents cast by default (RO-1), no translucent velocity/reactive mask (RO-6). |
| Frostbite (Hillaire 2015, "Physically based and unified volumetric rendering") | depth → G-buffer → lighting → froxel fog shared by opaque (composite) and every transparent (per-vertex/pixel fetch) → refractive surfaces first → sorted transparents → particles (often half-res, opacity shadow maps) | Same structure; particles full-res and not in a separate pass. |
| Unity HDRP | depth prepass → G-buffer → lighting → sky/fog → "before refraction" transparents → colour pyramid snapshot → refractive transparents → "after post" | Our snapshot is before *all* translucents (UE-style); translucents under water are invisible through it (accepted, water.md). |
| Decima (Horizon, Schneider 2015/2017) | clouds raymarched at reduced res with temporal reprojection, composited before translucency | Same (sf_atmos). |

## Violations

Severity: **High** visible every play session; **Medium** visible in common conditions;
**Low** rare or scaffold-only. Each has a proposed slice; none is fixed by this audit (the cheap
ones touch files other sessions own today).

- **RO-1** (High, shadows): blended and additive surfaces cast near-opaque shadows. Bevy 0.20's
  shadow pass queues `Blend`/`Add`/`Premultiplied` with `MAY_DISCARD` and discards only alpha below
  `PREMULTIPLIED_ALPHA_CUTOFF`; nothing sets `NotShadowCaster` on NIF blended/additive meshes or
  particle instances. Flames, glows, the Ghostfence field, glass and smoke quads cast shadows;
  blended foliage casts blobby shadows. *Slice* (after leafmask lands, `mw_assets` + `particles.rs`):
  `Add`/`Multiply` and particles → `NotShadowCaster`; `Blend` → casts with a 0.5 shadow cutoff via a
  prepass fragment on `FroxelFogMaterial`, glass-like (no texture alpha) → no cast; test scene in
  visual-checks (torch flame and window on a sunlit wall).
- **RO-2** (Medium-High): `LightSpriteMaterial` (distant town lights, additive) is not fogged: a
  town shows full-bright points through fog. *Slice*: `light_sprite.wesl` multiplies by
  `froxel_fog_lookup` transmittance (with the analytic far fallback, as water does).
- **RO-3** (Medium): `PrecipMaterial` is not fogged: rain and snow streaks keep full contrast in dense
  fog. *Slice*: `froxel_fog_apply` in `precip.wesl` (camera-local, one fetch per fragment).
- **RO-4** (Medium): fog for translucents lives in the host (`transparent_fog.rs` swap); the
  framework's own generic spawn (`sf_world_bevy` `spawn.rs`) emits blended `StandardMaterial`s that
  any other host draws unfogged. *Slice*: move the swap (it has no game knowledge) into
  `sf_atmos_bevy` as a `FogTranslucentsPlugin`; bevyrrowind keeps only the `--off transfog` A/B.
- **RO-5** (Medium, to verify): interiors have no froxel volume (`FroxelVolume.params.w = 0`), so no
  translucent is fogged there, and opaque interior fog is the clear colour only (`doors.rs`).
  *Slice*: an interior fog path (analytic distance fog in the composite, its parameters in
  `FroxelVolume` for the translucent fallback).
- **RO-6** (Medium): translucents write no motion vectors and no reactive/transparency mask, so
  particles, rain and sprites ghost under DLSS. *Slice*: DLSS bias-current-colour mask from the
  Overlay/Sorted bands if Bevy's DLSS integration exposes it, else motion vectors for precipitation
  via a redraw like water's step 9.
- **RO-7** (Medium, to verify): sky and cloud pixels may carry zero motion vectors (Bevy's prepass
  clears to 0; Bevy's `Skybox` has a motion-vector prepass, `Atmosphere`'s `render_sky` is not known
  to): DLSS would reproject the sky as screen-fixed and smear clouds on rotation. *Slice*: check with
  a BRP capture of the MV texture; if so, a fullscreen far-depth camera-motion pass after step 1.
- **RO-8** (Low-Medium, fragility): `SkyOrderScaffold` is copied in three crates (`night.rs`,
  `sf_atmos_bevy` `render.rs`, `sf_water_bevy` `render.rs`) and the resulting order depends on
  plugin add order (Sky → Clouds → Water); a wrong order silently reorders steps 4-7. *Slice*: one
  helper in `sf_render_order_bevy` plus a startup check that asserts the declared order; delete it
  when Bevy names the `render_sky` set.
- **RO-9** (Low, scaffold): the fade overlay is camera-local without a band (precipitation's Overlay
  band draws over the black fade) and casts a shadow while fading. *Slice*: `NotShadowCaster` +
  `DrawBand::Overlay { order: i16::MAX }`, or a screen-space fade when M8 replaces the scaffold.
- **RO-10** (Low): particles sort by the emitter origin (no AABB). Fine for small emitters; large
  ones (ash storms, spell volumes) can mis-sort against windows and leaves. *Slice*: an AABB from
  the simulation's live bounds each frame (UE: fixed per-emitter bounds) instead of
  `NoFrustumCulling`.
- **RO-11** (Low): held-prop meshes get `NotShadowCaster`, which in Bevy is for every light, so a
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
3. a site declared `Own`/`Froxel` fogged sits in a crate that never references the froxel volume, or
   an unfogged (`Host`/`Missing`) site lacks a violation id listed in this spec.

Adding a translucent class: give it fog (rule 1) and, if shader-placed or camera-local, a band
(rule 2), then add its row here and in `SITES`. Fixing a violation: change its row to the fixed
state, drop the id, delete it here.

**Host-side runtime check (specified, not built).** The app's NIF path builds materials at runtime,
so a source scan can't see it. A `dev`-feature system in bevyrrowind, every 60 frames: warn (once per
entity) when an entity still draws a blended `MeshMaterial3d<StandardMaterial>` two frames after
spawn (the fog swap missed it), when a translucent entity has `NoFrustumCulling` and no `DrawBand`,
and when a blended/additive entity casts shadows without an explicit opt-in (RO-1, after its slice).
