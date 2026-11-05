# First-person arms, hands and held items (research, 2026-11-05)

Status: research only. No web access was used: external facts are from memory and marked (verify) where a
symbol, line or version matters. In-repo facts were grepped.

## 0. What we already decided

`superfargoth/docs/specs/camera.md` "First person" (R7 §2.1, R8 §3) already says: a first-person body (arms,
legs when looking down) with its own FOV and scale toward the camera, head hidden, plus the world body for
shadows and reflections; per action either an arms layer (weapons, spells; a content pack's own 1st-person
clips may play as presentation on the shared clock) or the full body with the baked camera track (traversal,
sitting, paired moves). Bevy has no shadow-only flag (character.md open question 1), so render layers until it
does. `character.md` also lists "true first person" as an option. `mw_actors/src/slots.rs` already has a
`FIRST_PERSON` part-slot table and an `is_arm` fallback (third-person arm parts when no `.1st` part exists).
`avatar.rs` ~l.351 `first_person()` still hides the whole avatar. `sf_character_bevy` already reads
`RenderLayers` on character roots (held.rs, lib.rs). This note confirms, costs, and cuts slices.

## 1. Industry techniques

**A. Viewmodel (separate arms).** Own mesh, skeleton and clips, drawn with its own FOV (about 55-75 deg while
the world is 90+) via a second camera or a depth-range trick; no wall clipping, no wide-FOV stretch. Almost all
shooters; Skyrim/Fallout (separate 1st-person skeleton NIFs, 3rd-person body hidden); Source viewmodel FOV;
Unreal 5.5+ "First Person Rendering" (per-primitive FOV and scale-toward-camera, no second camera) (verify).
- Pros: art control, no head/neck issues, no clipping, cheap, weapon sits where design wants.
- Cons: second animation set (every attack, block, cast, torch idle twice); timing drift vs third person;
  shadows/reflections still need the world body; UAL and pack clips do not map onto a 1st-person rig for free.

**B. True first person / full-body awareness.** Render the world body, hide or shrink head/hair/helmet, camera
at the head bone. Mirror's Edge (arms+legs), Dying Light, Kingdom Come, Star Citizen, Cyberpunk (mostly), Ready or Not.
- Pros: one animation set and IK stack; remote co-op players are the same body; shadows and reflections free;
  traversal (mantle, roll, slide) reads right.
- Cons: arms/weapon posed by third-person clips so aim looks off from the eye; head-bone shake (we bake a
  low-passed eye track); near-plane clipping of shoulders at look-down; FOV distortion of near limbs; arms and
  weapon clip into walls.

**C. Hybrid (where shipped games trend).** Full body for locomotion/traversal/paired moves, viewmodel arms
for weapon work. Cheapest form: one skeleton, two renders: body with the camera FOV (head hidden), and the
same arms plus held items drawn again in a viewmodel pass with a narrower FOV and depth clear. No second clip
set; the pose is the third-person pose, adjusted by arms-layer IK where it must differ.

**Wall clipping**: depth clear in the viewmodel pass, or per-object FOV plus scale toward camera (UE), plus
physical pull-back (lower the weapon near walls) for long weapons.

## 2. Morrowind / OpenMW

- Morrowind: BODY records carry a first-person flag (BYDT flags); the player's race has `.1st` arm/hand parts
  and armor/clothing can list a first-person part (gauntlets, pauldrons). Animations: `xbase_anim.1st.nif`
  (humanoid) and beast variants; a separate arms-only skeleton with its own clip set (technique A).
- OpenMW does technique A: `apps/openmw/mwrender/npcanimation.cpp` (`NpcAnimation::setViewMode`,
  `VM_FirstPerson`, `updateParts()` showing only `.1st` parts, first-person offset for head-bob alignment,
  weapon via the usual attach nodes) and the "first person field of view" setting (Camera category) with a
  separate projection / depth clear for the first-person node in `renderingmanager.cpp` (verify exact symbols
  and line numbers at current master; I could not fetch the repo, so no commit is cited).
- Implication: our `FIRST_PERSON` table mirrors this. Content-pack `.1st` clips are optional presentation; the
  UAL fallback must be the shared-rig path.

## 3. Bevy 0.20-dev

Root `Cargo.toml` uses `bevy = "0.20"` (exact rev not checked).
- Official example `examples/camera/first_person_view_model.rs` (since 0.15): world camera plus a second
  `Camera3d` with higher `order`, `RenderLayers` for the view model, its own perspective FOV, depth cleared so
  arms never clip, view-model meshes `NotShadowCaster`, lights must also include the layer. (verify details)
- Cost for us: (a) a second camera repeats camera-bound passes (prepass, post stack, SSAO, bloom, DLSS) unless
  kept minimal; (b) TAA/DLSS need jitter plus motion vectors per camera, and DLSS history is per camera
  (roughly 0.5-1 ms at 1440p if upscaled separately, unmeasured), so the stable choice is to render the
  viewmodel into the main camera's targets (extra pass after opaque, depth cleared, no second upscale) or
  accept no temporal history on the arms; (c) GI probes and shadows are sampled in world space so lit arms
  are fine, but Solari/RT must see the world body, not the near-camera arms.
- Best final quality, bigger build: UE-style per-material FOV in the vertex stage inside the main camera and
  pass (no second camera; previous-frame projection must use the same adjustment for motion vectors). Needs a
  vendored prepass/material patch (CONTRIBUTING §4, no injection into Bevy).
- Link: https://github.com/bevyengine/bevy/blob/main/examples/camera/first_person_view_model.rs

## 4. Recommendation

1. Hybrid C, one skeleton, shared clips: third-person and UAL/Morrowind clips drive the world body; first
   person shows the same arms and held items in a viewmodel pass. `.1st` clips are an optional per-action
   override (as the spec says), never required.
2. Full body plus baked camera for traversal, sit, paired moves; arms-only viewmodel for weapon, torch, spell
   hands, block (camera.md allocation).
3. World body keeps rendering for shadows, reflections, RT (head hidden via layers); viewmodel arms are
   `NotShadowCaster`.
4. Ship the cheap path first (second camera on a viewmodel `RenderLayers`, depth clear, composited into main
   targets or no TAA history on arms); move to per-material FOV only if the owner dislikes the look.
5. Wall clipping: depth clear, plus weapon pull-back driven by the wall-brace/hand-IK probe.

## 5. Slice plan (one deliverable each)

1. FP-1 layer split: tag head/hair/helmet, arms, held items with `RenderLayers`; first person hides head and
   torso from the main view but keeps the world body for shadows/RT; replaces `avatar.rs::first_person`
   hide-all. Test: exact assertions on layer sets per slot.
2. FP-2 viewmodel camera: second camera or main-target pass, FOV setting (comfort switches), depth clear.
   Log-line checks only; visual-check entry "awaiting owner".
3. FP-3 held items: `held.rs` props, torch mesh in viewmodel while its light stays on the world body, spell
   hand attach for VFX.
4. FP-4 TAA/DLSS/GI: viewmodel motion vectors, upscale decision, NotShadowCaster; printed ms per preset.
5. FP-5 wall pull-back from the wall-brace / hand-IK probes.
6. FP-6 (optional) `.1st` clips baked as a presentation override on the shared clock (needs `bevyrrowind bake`).

Implications of existing systems: `held.rs` attaches props by node, so FP-3 is a second parent mapping, not
new logic; hand IK and combat clips run unchanged because the viewmodel is the same skeleton; co-op remote
players are just the world body, consistent with the net-ready rules.

## Sources

- Bevy example: https://github.com/bevyengine/bevy/blob/main/examples/camera/first_person_view_model.rs
- Unreal First Person Rendering: https://dev.epicgames.com/documentation/en-us/unreal-engine/first-person-rendering (verify)
- OpenMW: https://github.com/OpenMW/openmw/blob/master/apps/openmw/mwrender/npcanimation.cpp and
  https://github.com/OpenMW/openmw/blob/master/apps/openmw/mwrender/renderingmanager.cpp (verify symbols)
- UESP TES3 file format (BODY): https://en.uesp.net/wiki/Tes3Mod:File_Format
