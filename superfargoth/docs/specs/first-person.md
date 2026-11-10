# First person: body, arms, held items, view model

Status: **FP-1, FP-1b, FP-2a and FP-2b built (2026-11-10; FP-2 awaiting the owner's check, view model off by default); FP-2c..FP-6 planned.** Crates: `sf_character` (`ViewRole`,
merge groups), `sf_character_bevy` (`RoleLayers`, `ShadowLayers`, `PartRole`, module `roles`),
`sf_world_bevy` (`WorldspaceLayers`), `sf_sky_bevy` (traced scene), `sf_camera(_bevy)` (view choice,
eye, `view_model` projection math), `sf_character_bevy::view_model` (FP-2b material); game: `mw_actors` (`slots::view_role`), `mw_actors_bevy`, `mw_exterior_bevy` (sky lights,
lamps), `mw_player_bevy` (avatar).
Design source: [camera.md](camera.md) "First person" (render bodies, look limits, soft aim); research:
`../research/first-person-arms-2026-11-05.md` (techniques, OpenMW, Bevy) and
`../research/animation/08-industry-combat-responsiveness.md` §2. ROADMAP 3.4b; visual checks FP-.

## Rules

- **Presentation only** (camera.md "Rules"): the view choice and these layers never write gameplay
  state; a view switch rebuilds nothing and changes no tick. Aim stays `Command.view`, and the
  view choice reaches gameplay only as the input bit `Command.face_view` (camera.md "Rules").
- **One skeleton, one clip set.** The world body's pose (third-person, UAL and pack clips, IK) is the
  pose first person shows; a content pack's own first-person clips are an optional presentation
  override (FP-6), never required.
- **The world body keeps rendering** for shadows, reflections and ray tracing; first person only
  changes which views draw which parts.
- **Framework/game split.** The framework names the roles and the layer plan; an adapter maps its
  body-part slots onto roles (`CharacterContent::view_role`). Nothing here names a game's slots.

## Technique: hybrid (C)

Industry options (research note §1): A, a separate viewmodel rig and clip set (most shooters,
Skyrim/Fallout, OpenMW); B, true first person: the world body with the head hidden (Mirror's Edge,
Dying Light, Kingdom Come); C, hybrid: B for the body plus the same skeleton's arms and held items
drawn again in a viewmodel pass with their own FOV and a depth clear (where shipped games trend; UE
5.5 "First Person Rendering" is C inside one pass). We build **C, cheapest form first**: FP-1 is B
(one render, head off the main view); FP-2 adds the viewmodel pass. A is rejected: a second clip set
for every attack, block, cast and torch idle, with timing drift against the co-op body.

## Roles

`sf_character::ViewRole`, one per part:

| Role | What | Default slots (by region, `ViewRole::of_region`) |
|---|---|---|
| `Head` | head, face, hair, neck, helmets, amulets | regions covering neck, head, eyes or jaw |
| `Arms` | shoulders to fingertips, pauldrons, gauntlets | regions covering a shoulder, arm, hand or finger |
| `Body` | torso, pelvis, legs, feet, tail, accessories | everything else (empty regions included) |
| `Held` | weapons, shields, torches as props (`held.rs`) | none; every held-item mesh is `Held` |

The adapter overrides per slot. **Morrowind** (`mw_actors::slots::view_role`, by `BipedObjectType`):
head, hair, neck: `Head` (OpenMW adds neither head nor hair in first person,
`apps/openmw/mwrender/npcanimation.cpp` `updateParts` ~l.651, commit ca5e2576); the
`FIRST_PERSON` slots (hands, wrists, forearms, upper arms, pauldrons, shield): `Arms`, the parts
OpenMW's first-person model shows (`addPartGroup` ~l.871: `.1st` parts, else the third-person
hand/wrist/forearm/upper-arm part); weapon: `Held`; the rest `Body`. Framework slots no part slot
fills take the region default.

## Assembly

Outfits merge per material (character.md "Assembly at runtime"), so one skin material holds head,
arms and legs together. A character whose root carries `RoleLayers` is assembled with **merge
groups** (`AssemblyPart::group` = role): one mesh per (material, role), each mesh entity tagged
`PartRole`; the cache key carries `roles: true`. Only such characters split (the local player):
NPCs and remote co-op players keep one mesh per material, since a split costs +2-4 skinned meshes
(draws and palette copies) per character. `RoleLayers` is read at build time: insert it with the
`Character`.

## Layer plan

Bevy 0.20 has no shadow-only flag (`bevy_light` pairs casters with a light by `RenderLayers`
intersection, `lib.rs` ~l.437/611/703; a `Visibility::Hidden` entity casts nothing), so roles are
moved between layers. `W` = the root's own layers (its worldspace's, `world::follow_worldspace_layers`),
`S` = a worldspace shadow-only layer (FP-1b). FP-2 adds no layer: the view copy ("View model") is drawn
by the main camera on `W` with the view-model projection, `NotShadowCaster`.

| Role | Third person | FP-1b (built) | + FP-2 (target) |
|---|---|---|---|
| Body | W | W | W |
| Head | W | S (`RoleLayer::SHADOW`) | S |
| Arms | W | W | S, plus a view copy on W |
| Held | W | W | S, plus a view copy on W |
| Held lights | W + S | W + S | W + S (never role-layered) |

`RoleLayers { roles: [RoleLayer { world, shadow, extra, view_copy }; 4] }`: part layers = `extra`
(a host's own layers), plus `W` when `world`, plus `S` (the root's `ShadowLayers`) when `shadow`;
`view_copy` adds a `ViewCopy` per part on `W` ("View model", as built). `RoleLayers::world()` is
third person, `RoleLayers::first_person(view_model)` the FP-1b column (`false`) or the FP-2 one
(`true`: arms and held `RoleLayer::VIEW_MODEL`). `roles::sync_role_layers` (`PostUpdate`, set `CharacterLayerSystems`, after
the worldspace push, before `CheckVisibility`) re-layers a root's `PartRole` descendants, and spawns or despawns their view copies, when
its rule, layers, outfit or held props change. Third person still hides the whole avatar while the
camera's collision fade is past 0.5 (eye inside the body), as before.

**FP-1b, the shadow layer** (built 2026-11-10). One framework allocation: `WorldspaceRegistry`
hands out layer pairs, `WorldspaceLayers { world, shadow: world + 1 }` (1/2, 3/4, …; layer 0 stays
shared), with the sets per kind: `content()` `[W]`, `camera()` `[0, W]`, `light()` `[W, S]`,
`sky_light()` `[0, W, S]`, `shadow_only()` `[S]`. Writers: cameras at spawn and on travel
(`fade.rs` `Cut::travel`, `camera()`, never `S`); sky lights (`mw_exterior_bevy::light_layers`,
`sky_light()` of the active worldspace, none without sky); placement and script lamps
(`NifSpawner::spawn_light`, `light()`), the lab rig; character lights:
`world::follow_worldspace_layers` writes the root's `RenderLayers` `[W]` and `ShadowLayers([S])`,
pushes `[W]` to parts and `light()` to `HeldLightSlot`s and everything under them, and new slots
take `ShadowLayers::light` (`held.rs`), which `mw_actors_bevy` copies onto the torch.
`sync_role_layers` re-layers on a `ShadowLayers` change, so `S` follows travel.
Bevy side, unchanged and public: `check_dir_light_mesh_visibility` / point / spot checks pair
casters with a light's layers and mark them `ViewVisibility` (so an `S`-only mesh is extracted and
skinned), and the shadow views carry the light's layers (`queue_shadows` `matches_layers`); cameras
never list it. Checked consumers:
- **Ray traced scene and DDGI**: `sf_sky_bevy::path::scene_layers` traces the cameras' layers plus
  the layers of every light they see, the raster rule's union, so `S` is in the BLAS/TLAS (Solari
  shadows, GI, reflections) and `sf_gi_bevy` probes trace it, as the third-person body is (Solari
  0.20 traces static BLASes only, rendering.md DL-3: a known gap, not FP's); GI's
  light list already intersects lights with the cameras' layers (`[W, S]` still meets `[0, W]`).
- **Sun cascades, point/spot cubes, shadow cache**: Bevy's own views use the light's layers; the
  cache treats skinned meshes as movers, drawn by Bevy's face views each frame.
- **Sun views**: a light's view (`sf_precip_bevy` rain occlusion) takes the light's layers, so `S`
  occludes rain as in third person; an anchor's view (`sf_surface_bevy` shelter, canopy) keeps the
  camera's layers, and its hosts skip skinned meshes, so `S` (character parts only) changes nothing
  there. The far shadow map is terrain heights: no meshes.
- Not covered: a light a host hangs on a character outside a `HeldLightSlot` gets `[W]` only
  (`sf_character_bevy` does not tell lights from meshes); viewers and examples
  without worldspaces have no `S` (a shadow-only role there casts nothing).

## View model (FP-2 decision, 2026-11-10)

**Decision: one render, the view copy drawn by the main camera with its own projection (UE 5.5
"First Person Rendering", Source's viewmodel, OpenMW's projection swap), not a second camera.**
Research against the Bevy 0.20.0 source built here (`bevy` examples, `bevy_render`, `bevy_pbr`,
`bevy_light`, `bevy_core_pipeline`, vendored copies):

**A second `Camera3d`** (`examples/camera/first_person_view_model.rs`: `order: 1`, its own layer and
FOV, light on both layers; the example's arm is `NotShadowCaster`, its only light a point light):
- *Sun shadows twice.* `bevy_light::build_directional_light_cascades` fits cascades for every active
  camera, and `bevy_pbr` `prepare_lights` (vendor `render/light.rs` ~l.2147, ~l.2580) makes cascade
  shadow views per (camera x directional light whose layers meet the camera's). A sun carrying `V`
  re-renders all its cascades (4) for the view model, frusta nearly equal to the main camera's;
  without `V` the sun does not light the arms at all (sun-lit world, dark arms). Point and
  spot maps are per light (shared), but every light needs `V`, and `V` is not per worldspace.
- *Every per-view input duplicated to match the world:* `Exposure`, `Tonemapping`/`ColorGrading`,
  `AtmosphereEnvironmentMapLight` (a per-view cubemap), the DDGI `GiProbeVolumeView`, surface
  weather, fog, ambient; plus the per-view CPU visibility pass, clusters and light assignment.
- *Compositing.* Cameras on one target share main textures when target, usages, format and MSAA
  match (`view/mod.rs` `prepare_view_targets`, `MainTextureKey`); each runs the whole `Core3d`
  schedule. Shared with `ClearColorConfig::None`: the view model loads the main camera's already
  tonemapped texture and tonemaps it again (`tonemapping/node.rs` only checks `hdr`), unless the main
  camera gives up tonemapping and bloom while first person is on. Not shared (MSAA differs): it
  clears its own texture, tonemaps only the arms, and `upscaling` alpha-blends it over the output
  (`sorted_camera_index_for_target > 0`, `upscaling/mod.rs`), so post runs twice and bloom splits.
  Its own depth texture (`Camera3dDepthLoadOp::Clear(0.0)`) is why arms never clip into walls.
- *DLSS/TAA.* It has no `MainPassResolutionOverride`: it renders at output resolution after the
  main camera's DLSS, outside its history (no ghosting, but no temporal AA: MSAA at full res, and
  the arms look sharper than the world). Bevy UI's default camera is the highest-order one.
- Cost: a second 4-cascade sun pass (perf.md cascade counters) + ~0.3-0.6 ms of full-res clear,
  resolve, tonemap and blit at 1440p, per frame in first person. Removing the sun cost needs a Bevy
  patch (a view borrowing another view's lights and shadow maps: no upstream concept). Rejected.

**Single pass** (chosen): a material extension (`ExtendedMaterial<StandardMaterial, ViewModel>`,
public API: `MaterialExtension::{vertex_shader, prepass_vertex_shader}`; `sf_water_bevy`'s
`WaterExt` is our extension precedent, `sf_lod_bevy`/`sf_terrain_bevy` our custom vertex stages) whose vertex stage copies Bevy's `render/mesh.wesl` / `prepass.wesl`
(skinning, morph) and changes only the clip position:
- *FOV:* view-space x and y scaled by `k = tan(fov_world/2) / tan(fov_view/2)` before the camera's
  own `clip_from_view`, which keeps aspect, near plane and the TAA/DLSS jitter.
- *No wall clipping:* clip depth remapped into the nearest slice, `z' = R w + (1 - R) z` (reverse Z:
  `ndc' = R + (1 - R) ndc`, `R` ~0.8): only geometry closer than `near / R` can cover the arms
  (the classic viewmodel depth range; UE does it by scaling toward the eye).
- *Lighting stays true:* `world_position` (lighting, shadows, GI, fog, surface weather) is the real
  one; only `position` changes, so the arms are lit exactly like the body, with no duplicated state.
- *Prepass and motion vectors:* the prepass vertex stage applies the same clip so depth-prepass
  equality holds, and outputs pseudo-world positions `world_from_view * S_k * view_from_world * p`
  (previous frame: `previous_view_uniforms`), so Bevy's stock prepass fragment writes the right
  motion vectors: the arms go through DLSS/TAA with the world (FP-4 then checks, not builds).
- *Shadows:* shadow views use the prepass vertex shader too, and Bevy 0.20 has no shader def that
  tells a shadow view from the main prepass (`light.rs` ~l.3124 `light_key` = `DEPTH_PREPASS` +
  projection, as a depth-only camera prepass). So the arms are **drawn twice**: the role part stays
  on `S` with its `StandardMaterial` (shadow only, as the head), and a **view copy** (same mesh, same
  `SkinnedMesh` joints, the view-model material, `NotShadowCaster`, on `W`) is what the camera sees.
  Draw count equals third person; +1 joint palette per copy.
- Cost: vertex ALU on the arm and held meshes only, no extra view: expected < 0.05 ms per preset.
- Known costs of the depth slice (to watch in the visual check): screen-space effects read the
  remapped depth (SSAO halo around the arm silhouette, contact shadows on the arms march the wrong
  pixels); the FOV change moves the arms against the body on `W` (a shoulder seam looking down,
  open question 4). The copies stay out of the traced scene (skinned meshes are not traced, DL-3; a
  marker keeps it so) and out of other cameras' views.

Setting: `ComfortSettings::fov_body` (vertical degrees, default 50 against `fov_world` 60, Bevy's
example ratio), `view_model` (on/off, default off until the owner's FP-2 check) and `view_model_depth` (`R`, 0.8).
`V` and the light-layer change from FP-1b's handoff are dropped.

**As built, FP-2a (2026-11-10)** (detail: [archive](archive/first-person-history.md)): the math in
`sf_camera::view_model` (`fov_scale`, `ViewModelProjection { k, depth }`,
`ComfortSettings::view_model_projection`; exact tests, and the WESL mirror lines in its docs) and
the copies in `sf_character_bevy::roles`: `RoleLayer::view_copy` (`VIEW_MODEL`: the part on `S`
only plus a `ViewCopy { source }` child on `W`, `NotShadowCaster`, `NoRayTracing`, the part's mesh,
material, `SkinnedMesh` joints, morph weights and bounds), spawned, re-layered and despawned by
`sync_role_layers`, kept on the part's mesh, material, skin and bounds by `follow_view_copies`.

**As built, FP-2b (2026-11-10): the material.** `sf_character_bevy::view_model`:
`ViewModelMaterial = ExtendedMaterial<StandardMaterial, ViewModel>`, `ViewModel { fov_scale, depth }`
(`k`, `R`); `ViewModelPlugin` (embedded shaders, `MaterialPlugin`, the systems below).
- *Stages* (`src/shaders/`): `view_model.wesl` (main pass) and `view_model_prepass.wesl` (prepass;
  also the deferred vertex shader) are copies of the vendored Bevy 0.20.0 `render/mesh.wesl` and
  `prepass/prepass.wesl` vertex stages, each with a re-sync header; `view_model_common.wesl` holds
  the bindings and the two helpers (`view_model_world`, `view_model_clip`). Main pass: the lit
  `world_position` is the real one, `position = view_model_clip(position_world_to_clip(
  view_model_world(world, view.view_from_world, k)), R)`. Prepass: `world_position` is the pseudo
  position, `position` the same clip, `previous_world_position` the previous frame's pseudo position
  through `previous_view_uniforms.view_from_world`, so Bevy's stock prepass fragments write the
  drawn motion. Public `MaterialExtension` API only; no Bevy patch.
- *Bindless:* the extension is bindless like `StandardMaterial` (Bevy's `extended_material_bindless`
  layout: data index 100, index table at binding 101, data array at 102; a uniform at 100 when
  bindless is off), so `ExtendedMaterial` stays bindless and the copies batch; the vertex stage
  reads its slot from `mesh[instance].material_and_lightmap_bind_group_slot`.
- *Derivation:* with the plugin, a copy never carries a `StandardMaterial` component: `roles` writes
  `ViewCopyMaterial(part handle)` on spawn and on the part's handle swap, and
  `derive_view_model_materials` gives it the material derived from that handle (one per source
  asset id, `ViewModelMaterials`; other material swappers such as `sf_atmos_bevy`'s fog on blended
  materials never see a copy). Derived base = the source with `OpaqueRendererMethod::Forward` (a
  deferred G-buffer would relight the copy from its remapped depth: the RT path, which defaults to
  deferred, lights the copies forward). In-place edits of a source (`AssetEvent::Modified`, same
  handle) are copied over; a removed source is forgotten. Copies get `NoFrustumCulling` (drawn with
  another projection than their bounds; with `fov_body > fov_world` real bounds would cull a copy
  still on screen). Without the plugin copies keep the stock material (FP-2a).
- *Parameters:* `ViewModelSettings { fov_scale, depth }` (default identity), written by the game
  (`mw_player_bevy::avatar::first_person`) from the player camera's
  `ComfortSettings::view_model_projection()`; a change rewrites every derived material.
- *Decided:* (a) **clustered lights near the edge: accepted.** The fragment picks its cluster from
  the drawn pixel and the real view z; the real position is off the drawn ray by `(k - 1)` times its
  view-space x, y (< ~10 cm for arms within 1 m at `k` 1.24), so a light is missed only when its
  range ends within that distance of the arm, where Bevy's windowed falloff `(1 - (d/r)^4)^2` is
  ~`16 (0.1/r)^2` (2 % at a 3 m light, Morrowind lamps 2-7 m). A fix needs the whole PBR fragment
  copied; FP-2c's lamp check decides. (b) **Occlusion culling** (exteriors) tests the real bounds
  against last frame's depth, which holds the copy's own near-slice depth: a copy culled in the
  early phase is redrawn in Bevy's late phase (no flicker, a little cost; FP-4 measures).
  (c) **Default off** (`ComfortSettings::view_model`) until the owner's FP-2 check: dev console
  `on`/`off`/`flip viewmodel` (`mw_devtools_bevy`) A/Bs it in one run.
- Tests (headless): naga validation of both stages for 60 def sets (plain, `SKINNED`,
  `MORPH_TARGETS`, both, compressed vertices, colours/UV1/dither; prepass depth, normal, motion
  vectors with `HAS_PREVIOUS_SKIN`/`HAS_PREVIOUS_MORPH`; each bindless or not); re-sync guards
  (each copy equals the vendored stage except the documented lines); the helpers contain
  `sf_camera`'s mirror lines; uniform and bindless layout; derivation (per handle, handle swap,
  in-place edit, settings change); the avatar's copy carries the camera's projection and follows a
  settings change; the console switch. Pipeline creation on a GPU is the owner's log-line check
  (visual check FP-2).

## Build plan

1. **FP-1 layer split** (built): roles, merge groups, `RoleLayers`, the avatar's first person shows
   body, arms and held items with the head off. Tests: roles per slot (framework and Morrowind),
   merge groups, the layer rule per role, a headless view switch through the real systems.
2. **FP-1b shadow layer** (built): as above; tests: layer pairs and travel (`sf_world_bevy`), the
   head on `S` alone through the real systems and on travel (`sf_character_bevy`, avatar test),
   sky lights per worldspace (`mw_exterior_bevy`), the traced scene (`sf_sky_bevy`).
3. **FP-2 view model** ("View model"), three slices: **FP-2a** (built 2026-11-10, "As built,
   FP-2a") the projection as Bevy-free math in
   `sf_camera` (`k` from `fov_world`/`fov_body`, the depth slice, pseudo-world positions; exact unit
   tests) and the view-copy plan in `sf_character_bevy` (`RoleLayers` gains a per-role `view_copy`;
   copies spawned, re-synced and despawned with the role, sharing mesh and joints, `NotShadowCaster`,
   marker; headless tests: copies exist only in first person, on `W` only, base parts on `S` only,
   travel); **FP-2b** (built 2026-11-10, "As built, FP-2b") the `ViewModel` material extension and
   its two shaders (copies of Bevy's mesh and prepass vertex stages), the copy's material derived
   from the part's, params from the setting; **FP-2c** visual check FP-2 (owner: shader-compile log
   line, walls, lighting, DLSS on/off), then the default.
4. **FP-3 held items**: `held.rs` props in the viewmodel, torch light kept on the world body,
   spell-hand attach for VFX.
5. **FP-4 TAA/DLSS/GI**: viewmodel motion vectors and jitter, the upscale decision, printed ms
   per preset.
6. **FP-5 wall pull-back** from the wall-brace / hand-IK probes.
7. **FP-6** (optional) first-person clips as a presentation override (`bevyrrowind bake`).
First-person combat gameplay (soft aim, no hard lock, attacks face the view) is built ahead of
FP-2 (combat.md "C4 soft aim", 2026-11-10), and so are its attack look limits (combat.md "C4b": the view
held within a move's `look` yaw of the body until `Cancel(Move)`); the arms in the view model ride on FP-2.

## Industry comparison

| Technique | Shipped practice | Us | Verdict |
|---|---|---|---|
| Hide own head, keep its shadow | UE `bOwnerNoSee` + `bCastHiddenShadow`; Unity `ShadowCastingMode.ShadowsOnly` | head to a per-worldspace layer only lights (and the traced scene) see | **match** (FP-1b) |
| First-person arms | separate viewmodel rig (Skyrim, OpenMW `xbase_anim.1st`, shooters) or the world body's arms (Mirror's Edge) | world body's arms (FP-1), redrawn in a viewmodel (FP-2) | **match** (hybrid C) |
| Viewmodel FOV and clipping | OpenMW: per-subgraph projection swap (`OverrideFieldOfViewCallback`) + depth clear bin (`DepthClearCallback`, then a depth-only pass into the opaque depth for post); Bevy example: second camera on its own layer; UE 5.5: per-primitive FOV in the vertex stage | single pass: view copy with own FOV and a near depth slice in the vertex stage, true world position for lighting (FP-2) | **match** (UE 5.5 shape; built FP-2b, awaiting owner) |
| Split meshes only where needed | UE merges skeletal meshes per material for crowds, keeps the player modular | split by role only for roots with `RoleLayers` | **match** |

## Reuse

- **Bevy 0.20.0** (crates.io, plus our `vendor/` patches): `RenderLayers` (`bevy_camera`
  `visibility/render_layers.rs`: `layer`, `none`, `from_layers`, `with`, `union`, `intersects`; no
  component = layer 0; empty = invisible) and `examples/camera/first_person_view_model.rs` (world
  camera on layer 0, viewmodel camera `order: 1` on layer 1 with its own FOV, arm `NotShadowCaster`,
  light on both layers; rejected for FP-2, "View model"). No shadow-only or owner-no-see flag. Hierarchy layer propagation
  (`HierarchyPropagatePlugin<RenderLayers>`) is not used, as world.rs explains.
- **Crates**: none covers character part roles.
- **OpenMW** (ca5e2576): technique A, separate first-person skeleton and parts; cited above.

## Open questions

1. The neck is `Head` (hidden): looking down shows the collar line. Torso and legs stay; is the
   look-down view right, or should the upper torso hide too (most full-body-awareness games keep it)?
2. ~~FP-1b own slice?~~ Done as its own slice (2026-11-10).
3. On the ray-traced path the head is in the traced scene, so it can show in reflections (a
   mirror shows your face). Keep, or mask `S` out of reflection rays if Solari ever offers per-ray
   masks?
4. FP-2's FOV override moves the arms against the body (the torso is drawn at `fov_world`): with
   `fov_body` 50 vs 60 the arms are ~24% larger on screen. Accept the shoulder seam looking down,
   blend `k` toward 1 near the shoulders, or default `fov_body = fov_world`? Owner, at FP-2c.
