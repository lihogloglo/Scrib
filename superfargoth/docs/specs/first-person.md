# First person: body, arms, held items, view model

Status: **FP-1, FP-1b and FP-2a built (2026-11-10); FP-2 designed (single-pass view model, "View model" below), FP-2b..FP-6 planned.** Crates: `sf_character` (`ViewRole`,
merge groups), `sf_character_bevy` (`RoleLayers`, `ShadowLayers`, `PartRole`, module `roles`),
`sf_world_bevy` (`WorldspaceLayers`), `sf_sky_bevy` (traced scene), `sf_camera(_bevy)` (view choice,
eye, `view_model` projection math); game: `mw_actors` (`slots::view_role`), `mw_actors_bevy`, `mw_exterior_bevy` (sky lights,
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
example ratio), `view_model` (on/off, default off until FP-2b) and `view_model_depth` (`R`, 0.8).
`V` and the light-layer change from FP-1b's handoff are dropped.

**As built, FP-2a (2026-11-10).** Math: `sf_camera::view_model` (`fov_scale`,
`ViewModelProjection { k, depth }`, `ComfortSettings::view_model_projection`), exact tests in
`sf_camera/src/tests_view_model.rs`. The formulas both WESL stages mirror, line for line
(`view_from_world` rigid, as Bevy's camera views are; `right`/`up` = its rows 0 and 1):

```text
fn view_model_world(p, view_from_world, k) -> vec3:       // pseudo-world position
    let v = view_from_world * vec4(p, 1.0);
    let right = vec3(view_from_world[0].x, view_from_world[1].x, view_from_world[2].x);
    let up = vec3(view_from_world[0].y, view_from_world[1].y, view_from_world[2].y);
    return p + (k - 1.0) * (v.x * right + v.y * up);    // = world_from_view S_k view_from_world p
fn view_model_clip(clip, r) -> vec4:                      // depth slice
    return vec4(clip.xy, r * clip.w + (1.0 - r) * clip.z, clip.w);
position = view_model_clip(position_world_to_clip(view_model_world(world, view.view_from_world, k)), r)
```

The rows form needs no `world_from_view`, which `PreviousViewUniforms` lacks. Tested: in view
space x, y scale by `k` and depth is kept (equal to the inverse-matrix form); through Bevy's jittered
`perspective_infinite_reverse` the result is the `fov_body` projection with the same jitter;
`ndc' = R + (1 - R) ndc` keeps order and the near plane, and only geometry inside `near / R` can
cover; Bevy's stock motion-vector formula over the pseudo positions of two frames equals the drawn
motion (zero for an arm still on screen). **FP-2b's two stages** (vendor `bevy_pbr`):
`render/mesh.wesl` `vertex` (l.34-98): keep l.67 `world_position`, replace l.69 `position` by the
line above. `prepass/prepass.wesl` `vertex` (l.70-174): l.92 `world_position` = the pseudo position
(the prepass fragment reads it only for motion vectors: l.187, StandardMaterial's
`pbr_prepass.wesl` l.148 / `pbr_prepass_functions.wesl` l.93-97), l.93 `position` as above; l.157-160
`previous_world_position` = `view_model_world(prev, previous_view_uniforms.view_from_world, k)`;
l.94-97 (shadow-view ortho clamp) never runs, the copy casting nothing.

Copies: `sf_character_bevy::roles`. `RoleLayer::view_copy` (+ `VIEW_MODEL`: `S` only plus a copy)
gives every `PartRole` part of the role a `ViewCopy { source }` child (`Name` "view copy", `W`,
`NotShadowCaster`, `NoRayTracing`, the part's `Mesh3d`, `MeshMaterial3d<StandardMaterial>`,
`SkinnedMesh` with the **same joint entities**, morph weights, `Aabb`/`NoAutoAabb` or
`DynamicSkinnedMeshBounds`). Being the part's child, it despawns with it (outfit swap, held change)
and inherits its visibility; `sync_role_layers` spawns, re-layers and despawns copies with the rule,
the root's layers (travel: the worldspace push writes `W` to copies too) and the outfit;
`follow_view_copies` (after it) re-copies the part's mesh, material, skin, morph weights and bounds
on change, so a host's material swap (`mw_actors_bevy` `held.rs` self-lit torches, which also
reaches the copy itself as a descendant of the prop) needs no hook. Bevy 0.20 skins every mesh
from its own `SkinnedMesh::joints` (one palette per mesh), and the outfit already shares one rig's
joints between meshes, so a copy is one more palette, no second rig. Traced scene and GI:
`sf_sky_bevy` `mirror_meshes` mirrors every visible `StandardMaterial` mesh in the traced layers,
skinned or not, so a copy on `W` would be mirrored: `NoRayTracing` keeps it out (its
`Without<NoRayTracing>` filter), and DDGI traces only `RaytracingMesh3d`. No `sf_sky_bevy` change.
**Gate:** the avatar asks for `first_person(ComfortSettings::view_model)`, off by default: with the
stock material a copy draws exactly where the part did (the part is off `W`, so nothing draws
twice), at one extra draw and palette for nothing, so FP-2a changes nothing in game until FP-2b.
Tests: `sf_character_bevy` (copies only with the view model, arms and held only, on `W`, parts on
`S`, shared mesh/material/joints, material swap, held change, outfit swap, back to third person,
travel with feature `world`), `mw_player_bevy` avatar (both states of the setting through the real
camera and layer systems).

FP-2b must know: (a) Bevy's clustered lighting indexes the cluster by `frag_coord.xy` with the real
view z (`pbr_functions.wesl` l.457, `clustered_forward.wesl` l.57): the copy's drawn pixel is not its
lit position's, so near the screen edge a clustered light can be missed (fix in the extension's
fragment or accept; check in FP-2c); (b) the material is the part's, swapped by hosts (torches):
derive the extension per part handle and follow `follow_view_copies`' changes; asset mutation in
place does not change the handle; (c) `StandardMaterial` is bindless in 0.20: the extension's
`k`/`R` uniform must fit that layout or the copy's pipeline is non-bindless (one bind group per
material); (d) a headless check is possible: link the stages with `wesl` against the vendored
modules per def set and naga-validate (pattern: `sf_sky_bevy/src/shader_tests.rs`,
`sf_surface_bevy`'s), defs at least `SKINNED`, `MORPH_TARGETS`, `MOTION_VECTOR_PREPASS`,
`HAS_PREVIOUS_SKIN`, `HAS_PREVIOUS_MORPH`, `VERTEX_TANGENTS`; pipeline creation on a GPU stays an
owner log-line check.

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
   travel); **FP-2b** the `ViewModel` material extension and its two shaders (copy Bevy's mesh and
   prepass vertex stages), the copy's material derived from the part's, params from the setting;
   needs a GPU run for the shader-compile log line; **FP-2c** visual check FP-2 (DLSS on/off).
4. **FP-3 held items**: `held.rs` props in the viewmodel, torch light kept on the world body,
   spell-hand attach for VFX.
5. **FP-4 TAA/DLSS/GI**: viewmodel motion vectors and jitter, the upscale decision, printed ms
   per preset.
6. **FP-5 wall pull-back** from the wall-brace / hand-IK probes.
7. **FP-6** (optional) first-person clips as a presentation override (`bevyrrowind bake`).
First-person combat gameplay (soft aim, no hard lock, attacks face the view) is built ahead of
FP-2 (combat.md "C4 soft aim", 2026-11-10); its presentation (attack look limits, arms in the view
model) rides on FP-2.

## Industry comparison

| Technique | Shipped practice | Us | Verdict |
|---|---|---|---|
| Hide own head, keep its shadow | UE `bOwnerNoSee` + `bCastHiddenShadow`; Unity `ShadowCastingMode.ShadowsOnly` | head to a per-worldspace layer only lights (and the traced scene) see | **match** (FP-1b) |
| First-person arms | separate viewmodel rig (Skyrim, OpenMW `xbase_anim.1st`, shooters) or the world body's arms (Mirror's Edge) | world body's arms (FP-1), redrawn in a viewmodel (FP-2) | **match** (hybrid C) |
| Viewmodel FOV and clipping | OpenMW: per-subgraph projection swap (`OverrideFieldOfViewCallback`) + depth clear bin (`DepthClearCallback`, then a depth-only pass into the opaque depth for post); Bevy example: second camera on its own layer; UE 5.5: per-primitive FOV in the vertex stage | single pass: view copy with own FOV and a near depth slice in the vertex stage, true world position for lighting (FP-2) | **planned** (UE 5.5 shape) |
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
