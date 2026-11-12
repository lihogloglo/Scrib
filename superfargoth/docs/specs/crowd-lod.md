# Crowd LOD: distance tiers for characters (CROWD, design 2026-11-12)

Owner go 2026-11-12: NPCs get a distance tier. Target: Cyberpunk 2077 / RDR2 crowds. Presentation only.
Rows: [characters](../../../docs/wiki/engine/characters.md), [animation](../../../docs/wiki/engine/animation.md);
cost: `../research/npc-cpu-cost-2026-11-12.md`, `docs/research/architecture-vs-bevy-2026-11-12.md` §2.

## Problem
Balmora walk: 568 full rigs (~24k entities), 566 at A3 (frozen), ~20 evaluated; `--off actors` = ~2.0 ms (before
P4-P6), mostly frozen far rigs (skin extract/upload, joint visibility, +1200 main / +1100 shadow items).

## Tiers
Size = reference pixels at 1440 px, 70° vertical FOV, from the projected bound, not the frustum, so turning the
camera never swaps representation. 1.8 m figure: 20 px ≈ 93 m, 4 px ≈ 460 m.

| Tier | When | Keeps | Animation | Shadows | Physics / AI |
|---|---|---|---|---|---|
| C0 Rig | ≥ 20 px, or pinned | rig, joints, skinned meshes (today) | A0-A2 by size, A3 off-screen (unchanged) | skinned; per-frame within 16 m, root grid beyond (SH-20) | untouched |
| C1 Proxy | 20 → 4 px | root + one rigid mesh per material, lowest cooked LOD, pose frozen at demotion; held props reparented to root | `Animator` on root runs A3 logic (clocks, events, root motion); no bones | rigid static caster | untouched |
| C2 Hidden | < 4 px or past the actor ring | root only, hidden | A3 logic | none | untouched |

- **Pins** force C0: player, dialogue, combat, lock-on target, scripted scene, ragdoll (`PhysicsPose`), anim
  significance flags. Remote co-op players are characters: same rules, on each client's own camera.
- **Hysteresis 15%** (our A-tier value; UE Mass LOD uses 10% on distance): C0→C1 below 20 px, back above 23 px
  (≈ 81 m); C1→C2 below 4 px, back above 4.6 px. Minimum dwell 0.5 s per tier.
- **Swap budget** per frame: ≤ 4 promotions (nearest first), ≤ 8 demotions (farthest first); a waiting character
  keeps its form. Demotion only from A3, so the pose is already frozen: C1 looks like today's far NPCs, and no
  one freezes earlier than today (the reverted 2026-10-19 budget freeze is not repeated).

## Far representation (decision)

| Option | Bevy 0.20 mechanism | Cost | Verdict |
|---|---|---|---|
| (a) Rig kept, joints despawned | none: `SkinnedMesh.joints` must be live entities (`bevy_mesh/src/skinning.rs:16`) | n/a | impossible stock |
| (a') Rig kept, frozen skins not re-extracted | vendored `skin.rs` skips held blocks | keeps entities, items | C0 only (CROWD-3) |
| (b) Pose baked to a rigid mesh | CPU linear blend skinning (Bevy has none; ours, Bevy-free) → `Mesh` asset, `Mesh3d` + same `MeshMaterial3d`; `VisibilityRange` fade at C2 | ~40-80 µs/NPC on `AsyncComputeTaskPool`; VRAM ~50-150 KB/proxy at lowest LOD (gauge) | **C1 now** |
| (c) Animation-texture impostor | `bevy_open_vat` (vertex VAT, per mesh) or a bone texture per skeleton in a `MaterialExtension` vertex shader on custom joint attributes | GPU only, no CPU per NPC; own main/prepass/shadow vertex paths | **C1 later** (CROWD-4) |

(b) first: smallest change that removes rigs, same look as today. Character meshes upload with
`RenderAssetUsages::RENDER_WORLD` (`sf_character_bevy/src/lib.rs:711`), so the `OutfitCache` entry keeps an `Arc`
of the Bevy-free assembled proxy-LOD mesh while referenced. Expression morphs bake at weight 0.
- **Demote** (PostUpdate after propagation): palette = joint `GlobalTransform` × inverse bindpose, root-relative;
  a task skins the proxy LOD. On completion, in one command flush: swap `Mesh3d` on the existing part entities,
  remove `SkinnedMesh` + `DynamicSkinnedMeshBounds`, refit `Aabb`, despawn the joint hierarchy (P5 block
  refcount drops). Root, `Character`, `RenderLayers`, roles survive.
- **Promote**: respawn the rig hidden from the `OutfitCache` hit; the `Animator` (clocks kept running) poses it that
  frame; next frame show it and drop the proxy (UE Mass: actor spawns, then the instance hides). No bind-pose frame.
- (c): bevy_open_vat bakes per mesh and our outfits are unique (`../research/crates-2026-10-25.md` §2), so it fits
  creatures; humanoids take a bone texture per skeleton (UE AnimToTexture bone mode, Unity Animation Instancing).

## Net-ready
Crowd tier is client presentation from the local camera: not saved, not replicated, never read by AI, physics,
scripts or combat. The root entity is the character in every tier; gameplay never reads joints of a non-C0
character (hit volumes are physics shapes; sockets fall back to root + height). AI tiers (`ai.md`) and anim tiers
stay separate axes.

## Reuse check
- Bevy 0.20: `VisibilityRange` (`bevy_camera/src/visibility/range.rs:80`, fade margins, `use_aabb`) for C2;
  automatic instancing of identical mesh + material; `AsyncComputeTaskPool`. None: CPU skinning, skeletal-to-
  static LOD, crowd tier; `AnimationGraph` has no LOD (wiki bevy/skinning-and-animation). P5 blocks reused.
- Crates: `bevy_open_vat` 0.20 (MIT/Apache; owner adopt 2026-10-25, for creatures); no Bevy crate for skeletal
  crowd LOD or bone-animation textures (crates-2026-10-25). Own: tier selection, CPU skin, swap.

## Industry comparison
- **UE Mass** representation: high-res actor, low-res actor, static mesh instance, none; distance bands, 10%
  hysteresis, per-LOD caps, spawn queues; City Sample far crowds use AnimToTexture. C0/C1/C2 map to it.
- **UE Significance Manager / URO / Budget Allocator** (rate cap, interpolation, 1.0 ms): our A-tiers, `AnimBudget`.
- **CP2077** (Tremblay, GDC 2024): instancing for standing crowds, no update when occluded beyond 5 m. C1 is that
  tier. Gap: occlusion-driven anim skip (we use off-screen) → `industry-audit.md`.
- **AC Unity**: 40 AIs, 120 high-res, ~10k crowd. **OpenMW**: no actors past 7168 units (C2).

## Build plan (one deliverable + test per slice; hand off at ~80k tokens)

| Slice | Deliverable | Files | Expected | Verification |
|---|---|---|---|---|
| CROWD-1 | Ceiling: `--off farnpcs` (characters past 81 m not visual) + `crowd/*` gauges (per size band; meshes, joints per NPC) | `bevyrrowind/crates/mw_game_core/src/perf.rs:154`, `sf_character_bevy/src/lib.rs:360` | measurement: NPC cost after P4-P6 (est. 1.0-1.5 ms), far share (est. ≥ 70%) | walk + standing, 3 runs vs base and `--off actors`; ledger line |
| CROWD-2 | C0/C1/C2 with CPU pose bake | new `sf_character/src/crowd.rs` (tiers, hysteresis, budget), `sf_anim/src/skin_cpu.rs` (LBS), `sf_character_bevy/src/crowd.rs` (swap), `held.rs`; `--off crowdlod` | 0.7-1.2 ms of 2.0; build only if CROWD-1 far share ≥ 0.5 ms | unit: `hysteresis_holds_in_band`, `pins_force_rig`, `swap_budget_caps_per_frame`, `promote_nearest_first`, `lbs_identity_palette_is_exact`, `lbs_two_joint_reference`; headless: `demote_despawns_rig_and_frees_joint_block`, `promote_has_no_bind_pose_frame`, `held_prop_survives_demotion`, `tier_invariance_gameplay_hash` (all-C0 vs default, same AI/physics state); A/B `--off crowdlod`; owner CH-6 |
| CROWD-3 | Pose-held C0 skins: no joint walk or upload for unposed blocks; dirty ranges | vendored `bevy_pbr/src/render/skin.rs`, `sf_anim_bevy/src/lib.rs` | 0.1-0.3 ms | `held_block_not_rewritten`; CH-5 recheck |
| CROWD-4 [p] | Animated C1: bone texture per skeleton (idle, walk, run loops from `bevyrrowind bake`), instanced | `sf_anim` bake step, `sf_character_bevy` material extension + WGSL | CPU ~0 vs CROWD-2; GPU +0.05-0.15 ms (measure) | bake round trip (texture frame = pose runtime ±1e-4); owner CH-6b |

**Owner scene CH-6** (`bevyrrowind/docs/visual-checks/characters.md` + index line): Balmora, noon, standing spot then
the walk; look down long streets at NPCs 70-120 m away, A/B `--off crowdlod`. Look for: pops or flicker as NPCs
cross ~85-95 m, T-pose frames, far NPC shadows present, far poses no worse than today.

## Open question (owner, scope)
Far walkers stay frozen (as today) until CROWD-4: animated far crowds in Phase 1, or Phase 4 with the parked VAT item?
