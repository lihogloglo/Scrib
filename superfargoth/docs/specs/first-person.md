# First person: body, arms, held items, view model

Status: **FP-1 and FP-1b built (2026-11-10), FP-2..FP-6 planned.** Crates: `sf_character` (`ViewRole`,
merge groups), `sf_character_bevy` (`RoleLayers`, `ShadowLayers`, `PartRole`, module `roles`),
`sf_world_bevy` (`WorldspaceLayers`), `sf_sky_bevy` (traced scene), `sf_camera(_bevy)` (view choice,
eye); game: `mw_actors` (`slots::view_role`), `mw_actors_bevy`, `mw_exterior_bevy` (sky lights,
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
`S` = a worldspace shadow-only layer (FP-1b), `V` = the viewmodel camera's layer (FP-2).

| Role | Third person | FP-1b (built) | + FP-2 (target) |
|---|---|---|---|
| Body | W | W | W |
| Head | W | S (`RoleLayer::SHADOW`) | S |
| Arms | W | W | S + V |
| Held | W | W | S + V |
| Held lights | W + S | W + S | W + S (never role-layered) |

`RoleLayers { roles: [RoleLayer { world, shadow, extra }; 4] }`: part layers = `extra`, plus `W`
when `world`, plus `S` (the root's `ShadowLayers`) when `shadow`. `RoleLayers::world()` is third
person, `RoleLayers::first_person()` the current first-person column. `roles::sync_role_layers` (`PostUpdate`, set `CharacterLayerSystems`, after
the worldspace push, before `CheckVisibility`) re-layers a root's `PartRole` descendants when its
rule, layers, outfit or held props change. Third person still hides the whole avatar while the
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
  (`sf_character_bevy` cannot tell lights from meshes without `bevy_light`); viewers and examples
  without worldspaces have no `S` (a shadow-only role there casts nothing).

## Build plan

1. **FP-1 layer split** (built): roles, merge groups, `RoleLayers`, the avatar's first person shows
   body, arms and held items with the head off. Tests: roles per slot (framework and Morrowind),
   merge groups, the layer rule per role, a headless view switch through the real systems.
2. **FP-1b shadow layer** (built): as above; tests: layer pairs and travel (`sf_world_bevy`), the
   head on `S` alone through the real systems and on travel (`sf_character_bevy`, avatar test),
   sky lights per worldspace (`mw_exterior_bevy`), the traced scene (`sf_sky_bevy`).
3. **FP-2 viewmodel camera**: a second `Camera3d` (higher `order`, `RenderLayers` `V`, own FOV
   from the comfort setting, own depth) or a pass into the main targets; arms/held to `S + V`.
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
| Viewmodel FOV and clipping | OpenMW: per-subgraph projection swap (`OverrideFieldOfViewCallback`) + depth clear bin (`DepthClearCallback`, then a depth-only pass into the opaque depth for post); Bevy example: second camera on its own layer; UE 5.5: per-primitive FOV in the vertex stage | second camera or main-target pass (FP-2) | **planned** |
| Split meshes only where needed | UE merges skeletal meshes per material for crowds, keeps the player modular | split by role only for roots with `RoleLayers` | **match** |

## Reuse

- **Bevy 0.20.0** (crates.io, plus our `vendor/` patches): `RenderLayers` (`bevy_camera`
  `visibility/render_layers.rs`: `layer`, `none`, `from_layers`, `with`, `union`, `intersects`; no
  component = layer 0; empty = invisible) and `examples/camera/first_person_view_model.rs` (world
  camera on layer 0, viewmodel camera `order: 1` on layer 1 with its own FOV, arm `NotShadowCaster`,
  light on both layers). No shadow-only or owner-no-see flag. Hierarchy layer propagation
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
