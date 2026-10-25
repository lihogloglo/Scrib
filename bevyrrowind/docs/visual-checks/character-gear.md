# Visual checks: character-gear

Open visual checks: smooth bodies, held items, particles on actors (SB, HI, AP). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

## Smooth bodies (superfargoth `smooth-bodies.md`; `.sessions/smoothlive.md`)

Segmented bodies (rigid limb pieces), armor and clothing are smoothed per
outfit at assembly, off the main thread (`sf_character::smooth_outfit`):
seams welded, weights blended across joints, then a per-seam guard over
probe poses (elbows/knees 90°, shoulders raised, walk/run frames) puts back
the authored weights wherever a seam would open wider than vanilla. Heads
(morphs) and hair stay as authored. On by default; `--bodies vanilla` starts
segmented, **B** toggles live (every outfit rebuilds through the no-pop swap;
both looks stay cached). In `character`: **1** walks in place, **2** runs.
The lineup log prints "smooth bodies: N outfits, parts rewritten, guard
reverted, ms per outfit (indicative)".

Known remaining failures (survey, `smooth-bodies.md` "Survey rerun and
guard"), so look there first: **tail root ↔ groin / greaves pelvis opens
44-66 mm** in run frames (naked Argonians 46 mm, iron/daedric on Argonians);
**pauldron ↔ upper-arm pieces 13-22 mm** in elbow and run poses; **guard
cascades**: reverting one seam reopens a neighbor up to its vanilla width
(iron on an Imperial female 8 → 18 mm, daedric on an Argonian 33 → 66 mm).
None of these is wider than vanilla.

### SB-1: Lineup close-up at elbows and knees, walking
- Kind: game models. Run: `.\bevyrrowind character "@imperial,guard" ajira huleeya "@dark elf,,f" fargoth "@nord,+iron_cuirass+iron_greaves+iron_pauldron_left+iron_pauldron_right+iron boots+iron_gauntlet_left+iron_gauntlet_right" "@high elf,+common_robe_01+common_shoes_01" --walk`
  (Imperial guard, Khajiit, Argonian, female dark elf, Fargoth in his pants,
  a Nord in iron, a High elf in a robe). Fly close (free camera).
- Look for: elbows, knees, wrists, ankles and the neck bend without cracks,
  see-through slits or pieces sliding over each other; no piece is
  stretched or collapsed (a stretched plate is the step blending too far).
  Press B (vanilla): the classic cracks open at the same joints; B again.
  2 runs (larger bends).
  Since animfix (2026-10-12) the lineup is a treadmill: the graph is told the
  cycle's own speed, so walk (1) and run (2) play at their natural cadence
  (before: 0.75x, slow-motion), and the feet play the clip (no foot locks
  without ground: before, a planted foot stuck forward while the clip swept).
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
- Source: smoothlive, animfix.

### SB-2: Armored NPC
- Kind: game models. Run: `.\bevyrrowind character "@nord,+iron_cuirass+iron_greaves+iron_pauldron_left+iron_pauldron_right+iron boots+iron_gauntlet_left+iron_gauntlet_right" "@imperial,guard" --walk --camera "0.8,1.5,1.6,0,1.1,0"`.
- Look for: cuirass ↔ greaves, greaves ↔ knee guard ↔ boots and gauntlet ↔
  bare forearm stay closed as the legs and arms swing; plates stay rigid
  in their middle. Pauldrons are the known weak spot (13-22 mm opening at
  the shoulder ↔ upper arm, vanilla 68-99 mm). In Balmora (`scene sb-balmora`)
  the guards walk.
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
- Source: smoothlive.

### SB-3: Beast tails with greaves
- Kind: game models. Run: `.\bevyrrowind character huleeya "huleeya+iron_cuirass+iron_greaves" "ajira+netch_leather_greaves+netch_leather_cuirass" "huleeya:naked" --walk --camera "0.5,1.0,-2.2,0,0.8,0"`
  (camera behind them; 2 runs).
- Look for: the tail root against the greaves' pelvis piece and against the
  naked groin. **Known failure**: this seam still opens 44-66 mm in run
  frames (vanilla wider); the rest of the tail and legs should be closed.
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
- Source: smoothlive.

### SB-5: Walkers in Balmora
- Kind: game location. Run: `.\bevyrrowind scene sb-balmora` (`--bodies vanilla` to start segmented).
- Look for: walking NPCs' elbows and knees without cracks; press B for
  vanilla and back: no NPC disappears for a frame, no hitch beyond the first
  build of each look. Since animfix: feet planted while walking (no sliding),
  no NPC with legs stretched behind and the pelvis on the ground (WK-7).
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
- Source: smoothlive, animfix.

## Held items (superfargoth `character.md`, "Held items at runtime"; `.sessions/heldrender.md`)

Weapons, shields and torches as rigid props on `Weapon Bone` / `Shield Bone`
(`sf_character_bevy::held`), mounted by readiness (vanilla: no sheathed
weapons), torches lit when dark (placed-light calibration + flicker, the 12
nearest lit), the graph's weapon stance fed from readiness + weapon type.
Compare with OpenMW (same NPCs, same hour).

### HI-3: Torch-bearer at night
- Kind: game models. Run: `.\bevyrrowind character "imperial guard" "volene llervu" "@imperial,guard" --night` (N toggles day/night).
- Look for: the Imperial guard holds a torch in the left hand (its shield
  replaced, as out of combat in vanilla); warm flickering light on the floor
  and on the neighbours; by day (N) the torch goes and the shield comes back
  (props swap, outfit meshes stay). No flame yet (particles not rendered).
  The carrier itself looks overexposed near the torch (inverse-square near
  field; calibration follow-up).
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
  torch in hand, orange light pool on the floor and the lineup).
- Source: heldrender.

## Particles on actors, bows, carried-light near field (nif-runtime.md "Particles on actors"; `.sessions/actorparticles.md`)

Torch flames and creature particles are NIF emitters anchored at a skeleton
joint or the prop root (world-space particles trail a moving joint); bows are
skinned props on their own small skeleton (rest pose); the carried torch
light keeps 0.7 m off the carrier, dimmed to read "fully lit" at a third of its radius (torchfix). Compare with OpenMW.

### AP-2: Flame atronach and ancestor ghost particles
- Kind: game models. Run: `.\bevyrrowind character atronach_flame ancestor_ghost --night`;
  `RUST_LOG=bevyrrowind::actor_particles=debug` logs each emitter's joint and live emitter joint.
- Look for: the atronach's fire (4 emitters: one from `Bip01 Pelvis`, three
  from the `Weapon` hand joint) burning on the body and following it while it
  idles; the ghost's mist sprays (4 emitters at its root) under it. Particles
  hide beyond 60 m and pause off view.
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
  shows only the pelvis glow; the hand emitters may sit outside their emission
  window (they run on a free clock, not the animation time) or need
  `NiBSPArrayController` vertex emission: check against OpenMW. particles2:
  the hand flames are moment-keyed (cast) and wait for `emit:` curves; the
  body flames now cover the skeleton (PA-1). particles3: the hand flames
  emit in `Attack1`-`3` only, the ghost's sprays in its deaths only, so an
  idling lineup shows neither (PA-3).
- Source: actorparticles.

### AP-3: Archers hold bows
- Kind: game models. Run: `.\bevyrrowind character "imperial archer" "@wood elf,archer" "@dark elf,archer" --drawn`.
- Look for: a curved bow in hand (long bow, chitin short bow, steel longbow:
  skinned props, 1 joint each), not missing; held still at rest (no string
  pull yet). Log with `RUST_LOG=sf_character_bevy=debug`: `... is a skinned prop`.
- Status: deferred 2026-10-19 (phase0): characters/animation/physics rework in Phase 3; recheck there.
  the Imperial archer's curved bow).
- Source: actorparticles.
