# Combat and athletic animation: generate, buy or hybrid? (research, 2026-11-05)

Owner 2026-11-05: the UAL2 `Sword_Light_A-D` attacks "are really not good"; jump and mantle (`ClimbUp_2m`) need
better clips. Research only, no code, no game launch. Items marked **(verify)** come from memory or a snippet
that was not read in full. Pipeline context: [motion-generation spec](../specs/motion-generation.md), [ambient
research](ambient-life-2026-10-28.md), combat.md "Animation integration" and "Moveset sheet", action.md mantle.

## Verdict (short)

1. **Do not make Kimodo the primary source for the sword chain.** It lists combat as a strength but is a kinematic
   diffusion model with known foot skating, no object awareness, inexact constraint hits and a 10 s cap. A fast,
   weapon-held, anticipation-heavy swing is its hardest case (no source shows a good one). Use it as a cheap
   draft/variation tool and for in-betweening, not as the final hero attack.
2. **Best sword source: a mocap sword pack** (free Mocap Online "T.C. Sword" as the trial, then a paid pack such as
   Mocap Online / Kubold / RamsterZ class), retargeted by our existing BVH/FBX path, then fixed with procedural layers
   (retime to sheet windows, hit-stop, trails, IK).
3. **Jumps, rolls, vault, mantle: packs first (UAL2 parkour set, then a paid parkour pack), Kimodo/MotionBricks for
   gaps** (takeoff/landing links, height variants). Mantle height is already handled at runtime by `Warp` root-track
   warping plus IK; the clip needs the right shape, not the right height.
4. **Hybrid is the practical path**: mocap/keyed base + procedural layers. Generated clips multiply content; they do
   not replace hero animation (the industry still anchors hero animation in captured or keyed data).

## 1. Are Kimodo and MotionBricks good at combat and athletics?

What the sources say:

- Kimodo best-practices: covers "locomotion, gestures, everyday activities, object interactions, combat, dancing"
  and styles; **sport-specific actions "will likely give poor results"**. Prompts start "A person...", one or two
  behaviours, medium detail; under ~20 keyframes per constraint type; **max 10 s per prompt**; conflicting
  constraints cause artifacts; foot skating can occur and constraints are not hit exactly; post-processing helps
  contacts. [limitations](https://research.nvidia.com/labs/sil/projects/kimodo/docs/key_concepts/limitations.html)
- SOMA-SEED model card: "best at locomotion, gestures, combat, dancing, everyday activities"; known foot skating and
  prompt mismatch; realistic motion only; **unaware of scene objects** (no weapon, no ledge geometry, only keyframe
  constraints); jump and climb are not listed. [model card](https://huggingface.co/nvidia/Kimodo-SOMA-SEED-v1.1);
  paper [arXiv 2603.15546](https://arxiv.org/abs/2603.15546).
- Our audit (ambient note): SOMA-RP trained on Bones RigPlay; weights NVIDIA Open Model, outputs unclaimed; bundled
  sample BVHs evaluation-only; MotionBricks is in-betweening (links between poses we supply). Measured ~40 s per
  clip on the 4060.

No public evidence found for fast sword swings; the table is inference. **Verify with the MG-C0 test below.**

| Aspect | Expectation |
|---|---|
| Wrist/hand trajectory, grip | Weak: no weapon in the model; the hand goes wherever the arm goes. A hand end-effector constraint (<=20 keys) can pin a path, but blade plane and edge alignment are not controllable. |
| Anticipation / follow-through | Diffusion regresses toward mean motion: fast moves come out mushy, and holds and sharp accents (windup then snap) are the usual casualty. Text can ask for them; reliability unknown. |
| Root travel / lunges | Root 2D path and pelvis-height constraints exist; lunge foot contacts will skate without post-processing. |
| Jumps | Plausible (athletic locomotion is in the data) but takeoff/flight/land are not named capabilities; ballistic arc approximate. Use full-body keyframes for takeoff and landing poses. |
| Mantle at given height | Pelvis-height + hand constraints can express "hands on a 1.0 m ledge"; hands may slide. Fine as a base clip since `Warp` retimes the root and IK pins hands. |
| Jitter / skate | Known; use Kimodo post-processing, then our foot-lock/IK. |

MotionBricks ([arXiv 2604.24833](https://arxiv.org/abs/2604.24833)): good for links (idle to attack-ready, land to
idle) between poses we supply; it does not invent a good attack.

## 2. Alternatives (late 2026 snapshot)

Search coverage was thin. **All quality claims are the authors' own or from memory (verify).**

| Option | What | Combat/athletic quality | Licence | Hardware | Integration |
|---|---|---|---|---|---|
| **Paid mocap packs** (Mocap Online sword/combat, Kubold, RamsterZ, Kevin Iglesias Human Melee, others) | Captured, polished swings with real anticipation | **Best** for sword chains | Per-store licence; commercial use usually OK, **no redistribution of raw files**; Mocap Online also requires an "AI usage" permit for AI use (verify per pack). Fits our `private_only`/BYO pattern, not a GPL data repo. | none | Low: FBX/BVH to `auto_map` to retarget, same as UAL |
| **Mocap Online free "T.C. Sword"** ([store](https://mocaponline.com/collections/free-mocap-animations/products/tc-sword), [itch](https://mocaponline.itch.io/free-3d-sword-animation-pack)) | 1H sword attacks, blocks, moves | Mocap quality; good trial | free-pack terms not read **(verify)** | none | Low |
| **Rokoko free fight/weapon pack** ([page](https://rokoko.com/resources/motion-library-10-free-fight-and-weapon-animations)) | 10 clips | Mocap, small | not read **(verify)** | none | Low |
| **UAL2 (owned)** | CC0, 130+ clips, melee/armed combos, parkour set ([page](https://quaternius.com/packs/universalanimationlibrary2.html)) | Owner rates sword attacks poor; parkour set not yet judged | CC0 | none | Done |
| **Mixamo** | Big fight/jump/climb library | Mid, dated | Commercial use in a product OK, no redistribution of raw content, AI/ML restriction ([Adobe FAQ](https://community.adobe.com/questions-696/mixamo-faq-licensing-royalties-ownership-eula-and-tos-589400)); our rule: prototyping only | none | Low (FBX) |
| **LaFAN1** | Ubisoft, includes fight clips | Good mocap | **CC BY-NC-ND 4.0**, private_only (already an optional source); cannot ship | none | Done |
| **100STYLE / Motorica** | locomotion styles / dance | Not combat | not verified | none | n/a |
| **BONES-SEED** | 288 h SOMA mocap with text | Strong, includes combat | Academic or <$1M companies only, no redistribution (ambient note); ask licensing@bones.studio | none | Needs a grant |
| **AMASS subsets / CMU** | Research mocap, some fencing/martial arts | Raw, noisy, SMPL | AMASS non-commercial per dataset; CMU wording verify | none | Medium (SMPL to BVH) |
| **HY-Motion 1.0** (Tencent, [summary](https://www.emergentmind.com/papers/2512.23464)) | text-to-motion, 3000 h incl. game animation assets | Authors claim beating baselines; sword unverified | Tencent licence with territorial limits (verify) | Likely >8 GB for the full model (verify) | Medium |
| **MoMask / MotionLCM / MotionGPT family** | HumanML3D academic models | Low fidelity, 22 joints, no fingers, skate | Research datasets, non-commercial | OK | **Skip**: dominated by Kimodo |
| **TextOp / FRoM-W1** (2026, robot-oriented) | text to humanoid whole-body control | Robot-style, not game polish | open source (verify) | OK | Skip |
| **Physics controllers** ([ProtoMotions](https://github.com/NVlabs/ProtoMotions), [MaskedMimic](https://research.nvidia.com/labs/par/project/maskedmimic.html), ASE sword-and-shield, DeepMimic) | RL policy tracks a reference | Good plausibility (balance, contacts); needs a reference clip and hours of RL; physical, not stylised | NVIDIA/Apache code (verify) | Training on 8 GB tight; cloud realistic | **Park**; only a polish pass for skate |
| **Morrowind attack clips, restyled** | Retarget/modify `base_anim` slash/chop/thrust | Authentic to MW weapons, 2000s stiffness | Bethesda data, BYO | none | Low; already the `ClipFallback::Clip` path |

## 3. What shipped games do

- FromSoftware/Souls style (talks and frame-data community; **verify specifics**): attacks hand-keyed or
  mocap-then-heavily-keyed, long anticipation, a clear hold on the active frames, big authored root travel, a
  recovery cancellable by dodge. Timing is the design; frame data is read off the clip.
- Industry norm (UE5 melee, Bethesda, Ubisoft): mocap fights cleaned by animators; generative tools for filler, NPC
  variation and previs. 2026 vendor guides repeat that generated motion is still "floaty" and hero animation stays
  captured ([overview](https://app.cinevva.com/guides/ai-animation-generators-3d),
  [comparison](https://metavert.io/compare/mocap-vs-generative-animation)).
- Procedural layers we own or planned: moveset windows, `Progress` root mode, hit-stop, FullBodyIk, foot lock,
  weapon trails. They retime phases to the sheet and pin contacts; they cannot invent missing anticipation.
- So a generated clip can be a convincing draft and fine NPC variation, not an anticipation-and-snap hero swing.

## 4. Recommendation per category

Fallback rule: every slot keeps the current UAL2 clip or a Morrowind clip as its last candidate.

| Category | Best source | Second | Notes |
|---|---|---|---|
| Sword light chain (4 hits) | Mocap 1H sword pack (T.C. Sword trial, then a paid pack) | Kimodo drafts with hand/root constraints, owner picks | Retime to sheet windows; each hit with its own recovery |
| Sword heavy / charged | Same pack's heavy/overhead clips | Kimodo "heavy two-handed overhead slam" + MotionBricks to stitch a windup hold | The anticipation hold is the part to get right |
| Roll / dodge | UAL2 or paid parkour roll | Kimodo forward roll | Short, root-travel heavy; i-frames come from the sheet |
| Jump start / loop / land | UAL2 parkour set, else Kimodo with takeoff/landing pose keys | MotionBricks land-to-idle link | Three slots; the air loop is a near-static pose |
| Mantle 1 m / 2 m | Paid parkour pack top-out clips | Kimodo with pelvis-height + hand constraints at two heights; `ClimbUp_2m` stays fallback | Height by warp + hand IK, not by generation |
| Vault | Paid parkour pack | Kimodo "vaults over a low wall" (low confidence) | Needs a hand-plant contact marker |

## Slice plan (one deliverable each)

| Slice | Deliverable | Verify / owner step |
|---|---|---|
| **MG-C0** | Feasibility run: 12 Kimodo clips (4 sword swings, 2 roll, 2 jump, 2 mantle, 2 vault) with constraints and post-processing; printed skate and hand-height numbers | Owner browses via `clip-preview --browse`; go/no-go for Kimodo-based sword |
| **MG-C1** | Sourcing: download T.C. Sword and shortlist 2-3 paid packs, licence text read and filed (AI-use clauses) | Owner approves purchase |
| **MG-C2** | Import + slot mapping + bake of the chosen sword pack onto `attack.<sheet>.light1..4`, UAL2 kept as fallback | `bevyrrowind bake`; owner signs off in visual-checks |
| **MG-C3** | Recipe set for roll/jump/mantle/vault gaps (Kimodo/MotionBricks), BVH in `animation packs/generated/` with provenance | Owner picks keepers; verdicts.toml |
| **MG-C4** | Timing pass: sheet windows and root travel tuned to new clips; hit-stop and trail hooks per move | Tests sanity-only; owner judges feel |
| **MG-C5** (parked) | ProtoMotions cleanup of skate on selected generated clips | Only if MG-C0 shows skate is the blocker; likely cloud |

Open questions for the owner: budget for a paid pack; whether generated clips may ship (output claim unverified);
whether to ask Bones about SEED licensing.

## MG-C0 results (2026-11-05)

Run: `tools/motiongen/recipes/combat-feasibility.toml`, 12 prompts + 4 text-only sword controls, 4 samples each (64
clips, steps 50, fp32, post-process on): GPU 70 s (1.1 s/clip), wall 4.8 min (175 s CPU prompt encoding). Numbers:
`python -m motiongen stats` (median of 4; skate = horizontal foot travel while a foot point is within 1.5 cm of its
resting height, ledge as a second floor for mantles). Mocap baseline, same metric (LaFAN1 jumps/fight): skate 7-13 cm
per contact-second, wrist peak 9-11 m/s. All 64 cook onto `base_anim`/female/kna (finite; `measure_generated_dir`).

| Clip | Skate cm (cm/s) | Flight s | Root m | Pelvis min-max m | R wrist max m, peak m/s | Constraint err m |
|---|---|---|---|---|---|---|
| sword diag R-L / L-R / horiz / overhead | 9/2/2/23 (0.5-5) | 0 | 0.03 (0.40 overhead) | 0.80-0.94 | 1.56-1.71, 3.8-4.5 | hand 0.68-0.73 |
| same, text only | 18-41 (5-9) | 0 | 0.35-0.76 | 0.79-0.92 | 1.44-1.58, 4.0-5.6 | - |
| roll forward / side | 44/56 (24/30) | 0.6/0.5 | 2.92/1.94 | 0.12-0.95 | 1.10-1.36 | pelvis 0.04/0.11 |
| jump standing / running | 6/55 (2/33) | 0.25/0.24 | 1.01/10.06 | 0.85-1.08 | 1.03-1.25 | pelvis 0.14/0.45 |
| mantle 1 m / 2 m | 79/214 (52) | 1.2/1.4 | 1.19/1.00 | top 2.10/2.88 (asked 2.0/3.0) | 2.00/2.83 | hands 0.2/0.5 |
| vault run 1-hand / walk 2-hand | 26/16 (17/8) | 0.2/1.7 | 7.0/2.1 | max 1.00/2.23 | 1.29/2.16 | pelvis 0.34/0.12, hands 0.2-0.5 |

Verdict per category (numbers only; the owner's look decides):

- **Sword chain: not usable** (NPC draft at most). Wrist peak ~4 m/s vs ~10 in mocap, windup barely above the
  shoulder; hand keys missed by ~0.7 m (they do stop the drift). Confirms section 1: mocap pack (MG-C1).
- **Rolls: usable as base with IK + timing.** Real ground rolls (pelvis to 0.12 m, 2-3 m travel, pelvis keys held);
  skate is partly the metric (feet near the floor mid-roll); needs foot lock and a retime to the i-frame window.
- **Jumps: standing = weak base** (1 m forward, apex +0.2 m instead of +0.45, 0.25 s flight); **running = not usable**
  (no leap: flight 0.24 s is a stride, apex key ignored, skate 33 cm/s).
- **Mantles: usable as base with IK + warp** (1 m good: top reached, hands on the lip at the first key, then drift);
  2 m weaker (hands 0.5 m off). Skate number is mostly feet crossing the ledge height.
- **Vaults: not usable.** The one-hand run never clears the wall (pelvis max 1.00 m); the two-hand one climbs on top
  (pelvis 2.23 m, 1.7 s off the floor) instead of vaulting.

Findings for MG1b/MG-C3: our position-only hand keys are weak and Kimodo's post-process ignores them (it handles only
its own end-effector sets, which need a full reference pose); pelvis keys hold for slow rises, not against the text
(jump apex, vault); the importer's pelvis-yaw root unwinds ~360 degrees through a roll (roll slots need another root
rule); root motion is planar, so a mantle previews as climbing onto air.

Owner check (from `bevyrrowind/`): `cargo dev clip-preview --browse "../animation packs/generated/mg-c0" --prop
"iron longsword"` (Up/Down = activity, Left/Right = sample; `sword.free.*` = text-only controls).

## Gen2 (simple prompts, 2026-11-05)

After the owner's MG-C0 verdict (over-described, ambiguous prompts; "climb up" read as foot-by-foot climbing): recipe
`tools/motiongen/recipes/combat-gen2.toml`, 55 prompts × 7 samples = 385 clips into `animation packs/generated/gen2/`
(36 attacks: light ×2 + heavy for sword, claymore, foil, axe, battleaxe, mace, spear, dagger; jump / run / roll
attacks for sword, claymore, spear, dagger; 19 traversal / dodge prompts, 2-3 phrasings each). Rules: one plain
action, Kimodo's "A person ..." style, one style word (katana-style, french foil, big claymore, berserker); no hand
keys, no root keys; only an end pelvis height on mantles. Wall 12.7 min at BelowNormal: 337 s CPU prompt encoding,
GPU 1.03 s/clip. All 385 cook (`measure_generated_dir`). Ranked top 2 per id: `gen2/shortlist.md`.

Numbers (median per group; the owner's look decides): light/heavy swings wrist 3.8 m/s (best 7.9; mocap 9-11),
skate 2-4 cm/s, root drift 0.05 m without any root key. Jump/roll attacks are faster (6.2/6.5 m/s); jump attacks
only leave the ground for spear and claymore (flight 0.5 s). Rolls are real ground rolls (pelvis 0.12-0.15 m, 2.5-3.5
m). Running jump 1 ("runs and jumps over a gap") reaches 0.6 s flight, 8 m (MG-C0: a stride); standing jumps still
weak (0.2-0.33 s). Vaults 1 and 3 ("vaults over a low wall", "parkour vault") lift the pelvis to 1.3-1.5 m with
4-7 cm/s skate (MG-C0 never cleared); "runs and vaults over a fence" skates (23 cm/s). Wording matters for keys:
the mantle end key is followed by "grabs the top of a high wall and pulls themselves up" (err 0.13-0.41 m) but
ignored by "pulls themselves up onto a high ledge" (0.8-1.3 m). Simple prompts fixed the ambiguity, not the swing
speed: fast swings may still need a retime or the mocap pack.

Owner verdict on gen2 (2026-11-05): "a mixed bag"; claymore clips are worthless (the word is probably unknown to the
model: next round use plain wording such as "a big two-handed sword"). Picks pending.
