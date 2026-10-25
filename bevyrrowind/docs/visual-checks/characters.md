# Visual checks: characters

Open visual checks: skinned models, characters, smooth bodies, held items, particles on actors (SN, CH, SB, AP, HI). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

## Skinned models (superfargoth `bake.md`, "Bevy mapping"; `.sessions/skinloader.md`)

`Cooked::Skinned` bake products rendered by Bevy's own skinning and morphs
(`sf_model_bevy::spawn_skinned`), bound by joint name to a skeleton. Bind
pose only: no animation runtime yet.

### SN-2: Guar from its cooked skinned product
- Kind: game model. Run: `.\bevyrrowind scene skin-guar` (or `view r\guar.nif --model --skinned`).
- Look for: a whole, textured guar (16 meshes, 48 joints) at the NIF's rest
  pose: no stretched triangles, no parts at the origin, legs and claws in place.
- A/B: K flips to the NIF runtime meshes; nothing should move or change
  (rest pose = bind pose for vanilla skins).
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
- Source: skinloader.

### SN-3: Dark elf head morphs
- Kind: game model. Run: `.\bevyrrowind scene skin-head`.
- Look for: the head matches the NIF (K). M cycles its two geometry morph
  targets (`#1`, `#2`) at weight 1, then none: the face deforms smoothly
  (Bevy morph targets, gathered per mesh from the model-wide weights).
- A/B: K (NIF meshes), M (morph).
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
- Source: skinloader.

## Character animation (superfargoth `animation.md`, "Bevy integration"; `.sessions/animbevy.md`)

`sf_anim_bevy`: the Bevy-free graph runtime evaluated per character
(`par_iter`), local pose written back to rig bone entities from
`spawn_rig`, skins bound to those bones by name. Gizmos draw one green line
per bone (yellow/orange/red at LOD A1/A2/A3).

## Characters (superfargoth `character.md`, "Assembly at runtime"; `.sessions/charbevy.md`)

`sf_character_bevy`: `mw_actors` recipes resolved and merged per material
off the main thread (`sf_character::assemble`), one `SkinnedMesh` per
material (+ one per morphing head) on an `sf_anim_bevy` rig playing the
baked graph's idle; identical looks share one cached outfit. Expected looks
(vanilla Morrowind / OpenMW from memory, not run): guards in Imperial
steel/templar plates, Ajira in a common robe, Huleeya in light armor with
tail, Fargoth's naked body with a loincloth, guar and cliff racer in their
idles.

### CH-1: The M3 lineup (`bevyrrowind character`)
- Update 2026-10-22 (anim-refactor slice 3): characters now start their idles at their own phase and rate (+-5 %), so same-record NPCs no
  longer breathe in lockstep; the isolated check is `scene level-anim-crowd` (AL-7). Status: awaiting owner there.
- Kind: game models. Run: `.\bevyrrowind character` (Imperial guard, Ajira,
  Huleeya, a female dark elf, naked Fargoth, guar, cliff racer), or name
  ids: `.\bevyrrowind character "caius cosades" "sellus gravius:naked"`.
- Look for: everyone dressed and idling (breathing, no T-pose), heads with
  hair, hands and feet present, mirrored parts (left/right pauldrons,
  gauntlets, boots) on the correct sides and not inside-out, beast feet and
  tails on Ajira/Huleeya. The log lists each character's skinned meshes
  (merged per material + morphing head) and the cache.
- A/B: O swaps every NPC between own equipment and naked (CH-2), F fidget,
  G skeleton gizmos.
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
  posed; 13-19 meshes per NPC = their material count + 1 head).
- Source: charbevy.

### CH-3: Balmora street with NPCs standing in place
- Update 2026-10-22 (anim-refactor slice 3): idle phase, rate and AI fidget choice (never the same Idle2-9 twice in a row) vary per NPC;
  check no two neighbours breathe in sync (also `scene level-anim-crowd`). Status: awaiting owner.
- Kind: game location. Run: `.\bevyrrowind scene char-balmora`.
- Look for (with `--off walkers`; otherwise wanderers walk, WK-1): NPCs on the streets and bridges dressed and idling, feet on
  the ground, facing their placed direction (vanilla faces -Z; we add the
  half turn). Leveled creatures show the list's lowest entry (scaffold
  `leveled pick`). Fly down for a close look.
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
  standing in the street).
- Source: charbevy.

### CH-4: Census and Excise Office staff
- Kind: game location. Run: `.\bevyrrowind scene char-census`.
- Look for: Sellus Gravius and the officers dressed (Imperial uniforms) at
  their posts, idling; no part floating off a joint.
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
  the staff now stand on the floor (fallfix confirmed; before, they fell through at load). Their shadows only
  show with the camera very close, as for every object (interior point-light shadows: 1 light, faded out over
  15-25 m); owner asks for a general shadows pass (ROADMAP backlog).
- Source: charbevy.

## Node-animated creatures (superfargoth `character.md`, "Node-animated creatures"; `.sessions/nodecreatures.md`)

Creature bodies are cooked for their rig (`mesh.rig`, `sf_model::cook_rig`):
rigid meshes become single-joint parts of their transform node, so
creatures animated per node (scrib, kwama forager, shalk, bonelord,
centurion sphere) and the rigid parts of mixed
creatures (wolf, bear and goblin heads, kwama antennae, storm atronach
rocks) follow the same rig and graph as skinned bodies.

## Walkers (M3 demo: NPCs walk the streets)

`sf_controller_bevy` (ORCA avoidance → kinematic mover → animation feed) on the characters
`sf_character_bevy` spawns, driven by `mw_ai`'s `PathWalkScaffold` (app `walkers.rs`). Keys in
the streamed view: Shift+J mover gizmos, K foot placement on/off. `--off walkers` turns it all off
(CH-3 then shows everyone standing).

## Actor morphs and faces (superfargoth `character.md`, "Morph playback"; `.sessions/actorfx.md`)

| ID | Scene | What to look for | Status |
|---|---|---|---|
| AF-1 | `cargo dev run -- character "kwama forager" fargoth` (or `cargo run -p bevyrrowind --features dev -- character "kwama forager" fargoth`) | The forager's body swells and squeezes with its idle (geometry morph from its KF clip), no vertex explosions; log `morphs: 9 of 9 targets follow animation curves` with `RUST_LOG=sf_character_bevy=debug` | awaiting owner |
| AF-2 | same lineup, fly close to Fargoth's face | Eyes blink every 2-8 s; press `Y`: the mouth flaps (fake loudness), `Y` again stops; no blink while talking | awaiting owner |
| AF-3 | `character "kwama forager"` placed in an egg mine (any LEVC kwama) via `view` | Leveled kwama placements now wander like CREA ones (walkers report: wanderers > 0 in egg mines) | awaiting owner |
| AF-4 | `cargo dev character ancestor_ghost Heart_Akulakhan dremora clannfear` (shot: `shots/actorfx2/lineup.png`) | Heart of Akulakhan: fire, frost, shock and magicka shields scroll (UV) and blink in and out with its idles; log `actor fx ...: 5 hidden joints, 3 materials, 6 billboards` with `RUST_LOG=bevyrrowind::actor_fx=debug` | awaiting owner |
| AF-5 | same lineup, orbit the heart (right mouse) | The ray sheets around the heart (6 billboard parts) keep facing the camera from every side while the heart beats; nothing sits at the heart's root | awaiting owner |
| AF-6 | same lineup | Ancestor ghost is see-through and its robe alpha follows its idle (not a fixed fade); dremora and clannfear look as before (their effects only play in death / attack groups) | awaiting owner |
| AF-7 | `cargo dev character dremora dremora_lord clannfear --camera 0.6,1.4,3.6,0,1.0,0` (dremora fix, `.sessions/dremora.md`; shots/dremora/before.png, after.png) | Both dremora wear their daedric armor while idling: shoulder pauldrons, upper-arm plates ("biceps"), forearm braces, thigh greaves, calf boots and shoes (before: only the leather body suit). Press X (death): the armor stays on through the fall until the clip's end, then hides as authored | confirmed by owner (2026-10-13) |
