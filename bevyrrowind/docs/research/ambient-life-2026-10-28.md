# Ambient NPC life: animation readiness and motion generators (2026-10-28)

Owner question: can our animation system carry NPCs that work, sit, lean, sleep, farm (with good body placement and
stand<->sit transitions), wired to an NPC AI that isn't started; and what MotionBricks, BONES SEED and Kimodo offer.
Research only, nothing built. Web facts were gathered by research agents from page summaries: re-read the primary
licence text before any decision that relies on it.

## Verdict

- **The animation core is mostly ready; the missing pieces are around it.** The hard part of "sit on this chair
  correctly" is placement (warp to a seat slot, pelvis on the seat, feet on the floor, hands on the table). That is
  `Warp` + `FullBodyIk`, which exist. Missing: interaction points, NPCs on the action layer, a seat warp target,
  and the AI that picks activities.
- **Wide catalog** (owner 2026-10-28: "go much wider", Crimson Desert / Cyberpunk / RDR2 scale):
  [ambient-catalog-2026-10-28.md](ambient-catalog-2026-10-28.md), about 140 activities over 16 settings (town, tavern,
  home, farm, mine, port, temple, Great Houses, Ashlanders, guilds, wilderness, Vivec, weather, social, gaits), each
  tagged have (39) / gen (81) / pack (9) / hard (15), plus a 40-item priority order and the "reads as alive" layers.
  Taxonomy notes on Crimson Desert, KCD2, RDR2 and Cyberpunk internals are partly unverified (flagged there).
- **Clips are less of a gap than they look.** The CC0 Quaternius packs we already bundle have sit enter/loop/exit
  with idle variants and talking, ground sit, lie down/get up, five farm actions, fishing, tree chopping, kneeling
  repair, pick-ups, drink, counter idle, rail lean, carry walk, lantern/torch idles. Missing: wall lean, sweep,
  laundry, cooking/stirring, smithing, eating, writing, sleeping in a bed (only floor lie).
- **MotionBricks is a transition generator, not an activity library.** Use it as an **offline** generator for
  transitions and seat-height/approach-angle variants between poses we own. Runtime use conflicts with our
  architecture (below). **Kimodo** (text + constraints, offline) is the candidate for the missing activities.
  **BONES SEED** cannot ship in a public GPL game.

## Our system today (what exists, what doesn't)

| Need | State | Where |
|---|---|---|
| Enter/loop/exit chains, montage slots, inertialized transitions | built | `sf_anim_graph` (`slot`, state machines) |
| Warp a clip onto a target (root path + correction) | built, only `WarpTarget::LedgeTop` | `sf_action::rule` |
| Interaction rules gated on a focus tag | built (`MoveRule.focus`, `Category::Interaction`) | `sf_action::rule`, `step` |
| Full-body IK (XPBD, effectors with rotation) | built | `sf_anim::ik::full_body` |
| Hand IK on contacts (mantle) | built | `mw_graphs` HandIk ops, `sf_action::grip` |
| Look-at | built | `sf_anim` procedural pass |
| Interaction points (seat, bed, lean wall, work spot; slots, claims) | **designed, not built** (`sf_interaction`, M4) | `action-history.md` "Interaction points and edges" |
| NPCs on the action layer | **not yet**: walkers are path walkers with the MW graph (Idle2-9), no `sf_action` | `mw_walkers_bevy`, `mw_ai` |
| AI: schedules / activity choice / claims | **not started** (`mw_ai` = pathgrid + AI_W wander scaffold) | `mw_ai` |
| Props in hand (broom, hoe, cup, book) | partial: combat weapon sockets; no generic prop attach/detach events | `sf_combat`, `sf_character` |
| Crowd cost | anim LOD + budget built; VAT tier planned (`bevy_open_vat`) | ROADMAP Phase 4 |

Morrowind gives no furniture data: chairs, benches and beds are plain statics, and vanilla NPCs have no schedules
(AI_W wander + idle chances only). Seats, beds and lean walls must be **derived at bake** (mesh-name tags plus seat
height and approach sides from collision, like the planned ledge bake) and be patchable by mods (rule 3).

## Industry shape (what to build toward)

UE5 Smart Objects + StateTree/Mass (slots with transform, tag, claim; an activity = approach, enter, loop with
variants, exit); Skyrim furniture/idle markers driven by sandbox packages; RDR2/GTA "scenario points". Our version:

1. **AI layer** (`sf_ai` later): a schedule or needs picker chooses an activity tag near the NPC's home/work
   (`sit.chair`, `work.farm`, `lean.wall`, `sleep.bed`), **claims** an interaction slot, paths to its approach.
2. **Action layer**: the NPC issues `Interact`; a Warp rule with a new `WarpTarget::Slot` aligns the enter clip
   onto the seat; the loop plays with idle variants (sync-free, random picks); exit on interrupt (dialogue, combat,
   schedule change) through the rule's cancellable windows.
3. **Animation**: pelvis effector to seat height, feet to floor probes, hands to table/armrest/handle via
   `FullBodyIk`; look-at to the talker. One enter clip covers a band of seat heights with warp + IK; generated
   variants (below) widen the band.
4. **Data**: activity definitions are records (tag, clips per phase, props, IK effectors, allowed classes/times),
   so mods add activities without code.

A seam for classic AI (owner 2026-10-23) stays: classic = wander + idles, modern = activities on the same goals.
Prior art to check before designing: Morrowind MWSE/OpenMW mods that already add sitting/leaning NPCs and
schedules (names and licences not verified here).

## MotionBricks (NVlabs, SIGGRAPH 2026, arXiv:2604.24833)

- **What it is**: in-betweening. Context = last 4 poses, target = 1-4 poses; the model picks the duration and
  generates 0.6-1.9 s at 30 Hz; longer motion chains generations. No text, no scene input. "Smart Primitives" are
  target poses placed around an object (sit, vault, duck), sampled from clips. Velocity and style control for
  locomotion. Kinematic only; pose/contact artifacts possible.
- **Human checkpoint now exists**: `MotionBricks-SOMA-RP-v1` (30-joint SOMA skeleton), ONNX FP32 758 MB / FP16
  380 MB bundled in `github.com/NVlabs/motionbricks`, SafeTensors on `huggingface.co/nvidia/MotionBricks`. Replaces
  our 2026-10-11 note (G1 robot only).
- **Speed** (their table, RTX 4090, TensorRT plan only): SOMA FP16 1.28 ms at N=1, 1.87 ms at N=16. Each call
  yields about a second of motion per character.
- **Licences**: code Apache-2.0; weights NVIDIA Open Model Agreement ("commercially usable", "NVIDIA claims no
  ownership rights in outputs"; redistribution needs the agreement + a Notice file). **The bundled sample BVHs
  (including the sit clips the sit primitive samples) are under the NVIDIA Sample Data Evaluation License:
  internal evaluation only, no distribution of them or derivatives.** Production target poses must be ours
  (Quaternius CC0 retargeted to SOMA, or MW poses). Training data is Bones RigPlay 1; whether Bones terms reach
  outputs is unverified.
- **Runtime use (rejected for now)**: it would need onnxruntime/TensorRT in or beside the game (TensorRT is
  proprietary: plugin only, like DLSS), a BYO 380 MB weight download, GPU time against the 11 ms frame target, and
  it breaks action.md's rule that gameplay reads **baked** motion (non-deterministic across GPUs; co-op). Revisit
  as a cosmetic-only plugin for distant crowds if clip authoring stays the bottleneck.
- **Offline use (recommended)**: a tool, outside `bevyrrowind bake`, that generates transition clips between our
  own poses: stand -> sit at seat heights 0.35-0.60 m, approach angles, sit -> stand, walk -> lean, idle -> kneel,
  and writes glTF on the SOMA skeleton; `sf_character` retargets it like any pack. Needs: SOMA <-> humanoid map,
  a numpy -> glTF writer, CC0 target poses.

## Kimodo (NVIDIA SIL, arXiv:2603.15546, `github.com/nv-tlabs/kimodo`)

Text-to-motion diffusion with constraints (full-body keyframes, sparse joints, hand/foot end effectors, 2D root
paths). SOMA, G1 and SMPL-X variants; latest `Kimodo-SOMA-RP-v1.2` (2026-10-15). Output NPZ (rotations, joints,
foot contacts, root). Code Apache-2.0; SOMA/G1 weights NVIDIA Open Model (outputs unclaimed); the SMPL-X model uses
the NVIDIA R&D licence (not read: avoid). About 17 GB VRAM with the text encoder on GPU, under 3 GB with it on CPU;
Windows "should work", untested. Role: **offline authoring of the missing activities** (sweep, wall lean, stir a
pot, fold cloth, hammer, eat, write), with end-effector constraints for table height. It can also generate
MotionBricks target poses (their README suggests it).

## BONES SEED (`huggingface.co/datasets/bones-studio/seed`)

142,220 motions (half mirrored), about 288 h, 120 fps, SOMA BVH + G1 CSV, rich labels (Everyday 5,816 clips,
Interactions 14,643, Object Manipulation 11,620). Licence (`bones.studio/info/seed-license`): only academic users
and companies under $1M revenue; no redistribution of the raw dataset; results are the licensee's only if the raw
data can't be extracted from them; mandatory attribution. **Not usable for shipped clips** in a public GPL repo
(retargeted clips are extractable). Only route: ask licensing@bones.studio.

## Tests (2026-10-28, owner approved)

Setup outside the repo: `C:\mbtest\motionbricks` (uv env 3.1 GB, SOMA ONNX 1.1 GB), `C:\mbtest\kimodo` (torch
2.11 cu128, built with pip `cmake`). Scripts and Blender renderer: session scratchpad (`mb_sit_test.py`,
`render_npz.py`, headless Blender 5.2 Workbench capsule mannequin, about 9 s per clip). Outputs `C:\mbtest\out\sit`.

**MotionBricks SOMA FP16 on the RTX 4060 laptop:**
- Runs on Windows. TensorRT plan build 73 s on the first launch, cached after that. One generation (0.6-1.9 s of
  motion) takes **5.4 ms** end to end (inference 4.8 ms); NVIDIA quotes 1.3 ms on a 4090.
- Headless chain from the sample clips (evaluation only): idle -> approach -> sit -> hold 1 s -> stand. Seats at
  0.8/1.5/2.4 m, at 0/45/90/180 degrees, seated facing back or turned 90 degrees. All 8 runs produced sane motion:
  no NaN, the seated hips land exactly on the target (endpoint correction), and the feet stay on the floor
  (-1 to +1.3 cm). The mannequin walks to the box, turns and sits on it.
- Seat height: shifting the seated target by +/-8 cm moves the whole body, so the feet float 9.3 cm or sink 6.7 cm.
  The model does not adapt the legs to the seat height. Seat height must come from our IK (pelvis + feet
  effectors) or from a target pose authored at that height.
- Foot slide while in contact: 6-14 cm/s mean (includes walking frames). Foot locking is needed, which our foot
  placement already has.
- Owner on the first renders: "impressive", but the character makes a jumpy sidestep to get into position and
  often passes through the seat. Cause: the test asked the model to reach a mid-sit pose (with velocity) at the end
  of the walk, and no model sees the scene. Rerun `slot` mode (`C:\mbtest\out\slot`): one generated link (8-16
  tokens) into the first frame of the mocap sit clip, then the mocap sit and stand clips play (NVIDIA's own
  runtime pattern, `generate_clip_transition` + `start_clip`). Collisions stay until a path follower walks to an
  approach slot (`sf_interaction`).
**Kimodo-SOMA-RP-v1.2 on the same laptop** (text encoder `TEXT_ENCODER_MODE=local TEXT_ENCODER_DEVICE=cpu`; it
needs access to the gated `meta-llama/Meta-Llama-3-8B-Instruct`, granted to the owner's HF account 2026-10-28, and
a 16 GB download into the HF cache on C:):
- Runs on Windows natively (no Docker). About 80 s per prompt for 2 variants of 6 s at 30 fps, most of it
  reloading the 8B encoder on the CPU each call. A batch script that keeps the encoder loaded would be much faster.
- Output: 77-joint SOMA (with fingers) NPZ (local/global rotations, joints, foot contacts, root) plus BVH.
- Prompts: "sweeps the floor with a broom", "walks to a wall, turns and leans back against it with arms crossed",
  "stands at a table and folds a piece of cloth". Renders `C:\mbtest\out\kimodo\mp4`; a still sheet shows the
  sweeping arm pose, a backward lean and arms working at table height. There is no broom, wall or table: the
  generator only knows the body.
Next: owner judges the MP4s. Then a slice for the SOMA -> humanoid retarget (glTF writer + `sf_character` map) with
CC0 target poses, and a batch prompt runner with constraints (end effector on a table height, root path).

## Generated clips on the Morrowind rig (owner questions 2026-10-28)

- **Furniture and race scale.** MW chairs, tables and beds are oddly scaled, and race heights differ. So measure,
  don't assume: census the furniture meshes (seat/top height from baked collision, clustered into bands) and the
  race height scales. Then generate one clip set per band, with the target heights as constraints (Kimodo end
  effectors and root, MotionBricks target poses), and let `FullBodyIk` absorb what is left inside a band.
- **Props.** Use the game's own meshes (brooms, tankards, books, quills, picks, baskets) attached to the hand bone
  with a per-prop grip socket (data). The second hand on a broom or rake is a `FullBodyIk` effector on the handle.
  Generated finger poses are generic, so grips need a hand-pose override per prop class.
- **Rig.** Both models output only SOMA. The route is SOMA -> glTF -> our existing retarget path (`sf_character`
  humanoid map -> `base_anim` Bip01), the same as Quaternius UAL. Beast races and the first-person arms need
  checking like any pack.
- **LaFAN1 replacement.** LaFAN1 is CC BY-NC-ND, so it needs a BYO downloader. Kimodo and MotionBricks outputs are
  not claimed by NVIDIA (Open Model Agreement), so generated clips could ship in the repo like the CC0 packs, with
  no downloader. To verify first: whether the Bones RigPlay training-data terms reach outputs, and the copyright
  status of generated motion (likely none, i.e. shippable under our licence). Quality has to be judged by the
  owner on the cases LaFAN1 covers (falls, get-ups, traversal), where physical realism is hardest for kinematic
  models. If adopted, it is a new clip source beyond "packs only" (memory 2026-10-27): owner decision, then a
  DECISIONS line.

## Suggested order (if the owner opens this lane)

1. Spec `sf_interaction` (points, slots, claims, mod records) + `WarpTarget::Slot` + NPCs on `sf_action`.
2. Bake seats/beds/lean walls from MW statics (name tags + collision), mod-patchable.
3. One activity end to end with CC0 clips: an NPC walks to a Seyda Neen bench, sits with IK, idles, stands on
   interrupt.
4. AI spec (`sf_ai`): activity picker + schedule records + classic seam.
5. Generators (MotionBricks transitions, Kimodo gap fill) only once 3 shows which clips are missing.
