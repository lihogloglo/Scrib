# NPC animation variety: age, build, injury, fatigue, mood (research slice, 2026-11-25)

Owner question: "old people don't walk like young girls." Marks: [V] verified at a source, [I] inference/own knowledge.
Research only; no code. Companion to `anim.md` (limited free clip set) and `bevyrrowind/docs/specs/motion-generation.md`.


> **Orchestrator corrections (2026-11-25, against owner decisions; these win over the text below):**
> 1. **100STYLE is rejected** (owner 2026-11-25, DECISIONS: "cartoony"). Ignore every "100STYLE as base" below (§2.2,
>    §2.4, §4 budget): locomotion classes come from Kimodo (and MotionBricks style presets), never a style pack.
> 2. **Kimodo walk/run are to be fixed, not replaced** (owner 2026-11-25): "Kimodo = seeds only" in §0.3 is wrong; the
>    shipping cycles are Kimodo (+ MotionBricks) through the existing gates.
> 3. **Delirium is adopted and on by default** (DECISIONS 2026-11-19: ronokopl/delirium-physics-no-game fork, "adopt
>    upstream, never build our own"). §3's R12 "do not adopt / clean-room into `sf_ragdoll::powered`" is superseded; the
>    cost verdict (rare states, A0/A1, capped) stands, on Delirium.
> 4. **The owner wants a LOT of variation** (2026-11-25). The §4 budget (~200 clips, 5-8 classes) is the first batch,
>    not a ceiling: generation is ~2 h per 200, so the limit is filtering and review, and classes are data.

## 0. Findings

1. **No shipped game stores a full clip set per NPC archetype x state. They stack three cheap things:** a few clipsets per
   *gait class* (old, fat, drunk, injured, scared, ...), *additive/procedural modifiers* driven by continuous scalars
   (limp, hunch, fatigue breathing), and *speed/stride warping*. Content-heavy part = the gait classes; the rest is runtime-cheap. [I, GTA/RDR2/TLOU practice]
2. **Style is mostly a locomotion problem.** Interaction clips (sit, drink, door) read the same for most people; vary them with
   shared clips + modifiers (posture, speed, hesitation), not per-style copies. [I]
3. **Kimodo can make gait-class locomotion only by physical wording, not by "old" labels.** Its own card names age bias as a risk
   and recommends neutral physical text such as "A person walks slowly with shuffled steps". [V HF card] And our own ledger says
   Kimodo locomotion cycles failed the gates (anim.md §0.1), so Kimodo = style *seeds* and ambient clips, not the shipping cycles.
4. **MotionBricks has a newer version, but it adds the human SOMA model, not style authoring we lack.** See §2.3. Verdict: nothing that changes the plan; useful as a runtime/offline in-betweener with six style presets (incl. Injured-Leg, Injured-Torso, Zombie).
5. **Delirium "tint" is the wrong layer for a baseline limp** (cost, one body, provenance); a powered-ragdoll *tint* via our own `sf_ragdoll::powered` is right for rare, strong states (drunk stagger, wounded stumble) on A0/A1 NPCs only. See §3.
6. **Recommended runtime model:** `MotionStyle` clipset id (~6-8 gait classes) + continuous modifiers (`injury`, `fatigue`, `age`, `load`) applied as additive/procedural layers + speed/stride warp; physics tint only for stagger states. Budget in §4.

## 1. Industry: how variants are authored and selected

| Technique | Who | Content cost | Runtime cost | Notes |
|---|---|---|---|---|
| Per-ped movement **clipset** (a named set of walk/run/idle/turn clips) swapped by script or ped metadata | GTA V, RDR2 | High per set (an animator makes each) | Near zero: a table lookup | [V] GTA V peds carry a movement clip set, e.g. `move_m@generic_niko`, `move_m@gangster@a`, stored in `clip_move_*.rpf`; mods swap them and fall back to default transitions when a set lacks a clip ([gta5-mods Niko walk](https://bg.gta5-mods.com/misc/niko-bellic-addon-walk-style), [Franklin walk](https://ca.gta5-mods.com/misc/franklin-gangsta-walk)). Drunk/injured sets exist as clipset names [I]; ped metadata `peds.ymt` assigns the movement set [V, same sources]. |
| Gait/mood layered on a base graph | RDR2, Cyberpunk | Medium | Low | [I] Mood/state tags (calm, alert, scared, tired) pick among clip variants and tweak speed; transitions use shared start/stop clips. |
| **Injured/wounded locomotion** as a full parallel set | TLOU2, Uncharted | High | Zero | [I] Authored limp/hunched sets with their own idle/start/stop; selected by health band (e.g. <30%). The cheapest way to look injured is still a separate gait class, not a filter. |
| **Motion matching with per-style databases** | Ubisoft (For Honor, Assassin's Creed), UE5 PoseSearch | Mocap per style | Search per frame (CPU), more DBs = more memory | [V] UE: a Chooser picks which PoseSearch database to search; normalization sets let several DBs compare costs ([UE forum](https://forums.unrealengine.com/t/5-5-chooser-tables-cannot-chose-databases/2298229), [UPoseSearchNormalizationSet](https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/PoseSearch/UPoseSearchNormalizationSet)); `SetDatabasesToSearch` switches at runtime. Per-style DB is the standard way to do style in MM. |
| **Learned style modelling** | Mason & Starke 2022 (100STYLE, feature-wise transformations, local phases); RSMT 2023 | Mocap per style + training | NN inference per character | [V] 100 styles, 4.06 M frames, ~18.75 min x 60 fps, one actor, Xsens ([paper](https://ar5iv.labs.arxiv.org/html/2201.04439)); Drunk is stochastic there. Research-grade for crowds; not a shipping path for us. |
| **Additive/procedural modifiers** | everywhere | Low (parametric code, 1-3 additive poses) | Very low | [I] Hunch (spine/neck bend), limp (hip drop + shortened stance on one leg + asymmetric stride timing), fatigue (breathing additive, lower stride, slumped shoulders), encumbrance (arm/torso offset, slower speed), tremor. Works for any clip when the rig has IK/ops. Our `FullBodyIk`, `Lean`, stride ops already exist (animation.md). |
| **Physics tint** (Euphoria in GTA IV/RDR2, NaturalMotion) | Rockstar | Engine + behaviour authoring | High per character | [I] Active ragdoll partially follows animation; used for stagger, drunk, hits, not baseline gait. Rockstar kept authored clips for baseline walking. |

Content-heavy: per-class clipsets, per-style MM DBs, learned models. Runtime-cheap: additive/procedural modifiers, warping, clipset id lookup. The practical hybrid is: **a handful of authored gait classes + continuous procedural modifiers**.

## 2. Generation

### 2.1 Kimodo and style
- [V] Kimodo-SOMA-RP-v1: 282 M params, 700 h Bones Rigplay mocap with text, max 300 frames (10 s) at 30 fps; text + constraints (joint pos/rot, 2D heading, 2D root path); licence NVIDIA Open Model License, "ready for commercial use"; best at locomotion, gestures, combat, dancing, everyday activities; "does not always follow the prompt"; roughly equal male/female actors; age bias named as a risk ([HF card](https://huggingface.co/nvidia/Kimodo-SOMA-RP-v1)). Other checkpoints (SMPLX) have a more restrictive licence ([search note](https://huggingface.co/nvidia/Kimodo-SMPLX-RP-v1)); we use SOMA only.
- [V] No dedicated style/age/injury control exists: control = text + kinematic constraints. [I] So style is expressed as **physical description** in text ("walks with a pronounced limp, short step on the left leg, hunched forward, slow") plus **constraints that force the style**: a lower constant root speed (2D root path with slow timing), a bent-forward key pose at frame 0 as full-body constraint, shorter stride via dense foot/root constraints.
- **Family consistency [I]:** Kimodo has no identity/style embedding, so consistency must come from (a) a fixed *style recipe* string reused in every prompt of the family ("an elderly person, hunched, slow shuffling steps, ..." + action), (b) the same key-pose constraint (the family's "posture pose") at start and end of every clip, (c) the same root speed envelope, (d) a seed sweep with an automatic style-score filter (mean speed, stride length, spine pitch, step asymmetry measured on the retargeted clip, must lie in the family's band), (e) a human rank of 3-5 keepers per family. Numeric style filters are the lever: they turn "same person" into measurable bands, on top of the foot-slide gates we already run.
- Measured today: ~40 s per clip, 2000 clips per night target (motion-generation.md Decisions); and Kimodo cycles for locomotion failed gates (anim.md §0).

### 2.2 Cheaper than generating: derive styles from one good clip [I]
For limp, hunch, tired, burdened: do not generate; apply an **offline motion edit** (or runtime additive) to 100STYLE/Neutral cycles: spine pitch offset, stride/speed scale, asymmetric hip hike. Generation is for what edits cannot make (old shuffle with different timing, drunk sway, zombie).

### 2.3 MotionBricks: the new version
- [V] Timeline: arXiv 2604.24833 (27 Apr 2026), SIGGRAPH 2026 ([paper](https://arxiv.org/pdf/2604.24833), [project](https://nvlabs.github.io/motionbricks/)). Early G1-robot preview in GR00T-WholeBodyControl; **official NVlabs/motionbricks, v0.2.0, late Oct 2026** (one source: 22-23 Oct; NVIDIA blog 27 Oct): second pretrained model for the 30-joint **SOMA** human skeleton (`MotionBricks-SOMA-RP-v1`), SafeTensors on HF for the first time, ONNX FP32/mixed FP16 bundled, a new Inbetweener demo; G1 model unchanged ([v0.2.0 write-up](https://www.orcarouter.ai/blog/motionbricks-v0-2-0-soma-standalone-release)).
- [V] Features: one backbone over 350 k clips, "15 000 FPS, 2 ms latency" (marketing; per-character real-time), smart locomotion from velocity/heading/**style** commands, smart objects (proxy keyframes for approach + contact), smart primitives (UE5 authoring). **Styles shown: Zombie, Injured-Leg, Injured-Torso, Skipping, Strafing, Crouch Strafing, plus Freestyle**; Idle/Walk/Jog/Run continuous. "Old" and "drunk" are not listed ([project](https://nvlabs.github.io/motionbricks/)). Per the paper, style is controlled by **keyframes sampled from a short reference clip or hand-authored pose of the desired style** ([search summary of arXiv](https://arxiv.org/pdf/2604.24833)) [partly I: confirm in the paper before building].
- [V] Licence: code Apache 2.0; weights NVIDIA Open Model Agreement (commercial use and redistribution of derivatives permitted subject to its terms); **bundled BONES sample motions: internal-evaluation only** (matches our existing rule: samples never feed shipped output) ([HF](https://huggingface.co/nvidia/motionbricks)).
- [V] Hardware: **NVIDIA GPU mandatory** (TensorRT plans; Warp/Newton backends; no CPU/AMD path). RTX 4060 laptop is NVIDIA, so offline use is fine [I: no VRAM figure published, model is small]. Not a runtime option for a game that must run on all GPUs; offline only (already our decision).
- **Verdict:** v0.2.0 gives us the SOMA model we already planned to use plus an official in-betweener and a *style-by-reference-pose* control. The new thing for this question: **Injured-Leg/Injured-Torso/Zombie style presets and style-by-keyframe could produce a consistent limp family** from one authored key pose, which Kimodo cannot guarantee. Needs a spike (one session): SOMA BVH out, style presets via CLI, check licence text of the Open Model Agreement for redistributing outputs (Kimodo's card says commercial use; MotionBricks' bundled samples are the only restricted part). Not a blocker.

### 2.4 Others
- [V] 100STYLE (CC BY 4.0 per our anim.md §0.1; the paper page states no licence, verify) gives a real-mocap style bank (Drunk, Bent Forward, Lean Back, Drag Right Leg, ...) from a single actor ([paper](https://ar5iv.labs.arxiv.org/html/2201.04439)). Style names for Old/Injured/Tired not confirmed from the paper page [verify in the dataset's style list].
- [I] Other 2025-26 text-to-motion models (MotionGPT-family, MoMask, SMPL-based) are mostly research licences or SMPL-bound (non-commercial); not checked this slice. Not recommended given Kimodo + MotionBricks are both NVIDIA Open Model licensed and on the same SOMA skeleton.

## 3. Delirium tinting

- [V, our R12 verdict] Delirium inverts our model (unanimated, muscle-driven; follow mode is a side message), brings its own solver, handles one 21-part Z-up humanoid, is young (38 commits, one author), and has unclear provenance (reproduces NaturalMotion API; playroom decrypts GTA IV executables). Recommendation: do not adopt as dependency; clean-room the technique into superfargoth; optional bevyrrowind-only `mw_delirium` FOLLOW spike (R9). Already "as built" in-game as opt-in takeover (slices 1, 2a, 2b: takeover of an animated NPC, blend-out) per `12-delirium-physics.md` As-built sections.
- **What an active-ragdoll layer can do for style [I]:** stumble/stagger steps, drunk sway (low-frequency random balance disturbance), fatigue (reduced joint stiffness: sagging arms, heavier head), limp (one leg with lower stiffness / delayed contact), weight shift when carrying a load. It reacts to terrain, pushes and bumps, which clips never do.
- **Cost [I]:** per character a physics body + joint solves every fixed tick, more at lower stiffness (stability), and it breaks foot-lock IK conventions. Our budget: A0 character ≤ 25 µs (animation.md LOD targets); a ragdoll body is far over that. So physics tint can only run on a few visible NPCs (the "stage" tier: A0, near the player, in dialogue or in a stagger state), never as baseline gait for a crowd.
- **Verdict:** the owner's "Delirium can tint a limp" is true but is the expensive path. Baseline limp = additive + clip class (§4); tint = rare states (drunk, wounded stagger, exhausted collapse) via our own `sf_ragdoll::powered` (or Delirium where the owner has it opt-in), gated to A0/A1 and a global cap (e.g. 4-8 concurrent). Keep a cheap authored fallback for all tints.

## 4. Runtime model recommendation

**Style as data.** Each NPC gets a `MotionProfile` (per-individual, like the perception profile decision in DECISIONS):
- `style_id`: a gait class (clipset id), chosen from age/sex/build/personality at spawn (stable, saved).
- continuous scalars, updated by gameplay, smoothed: `injury` 0-1 (+ side L/R), `fatigue` 0-1, `load` 0-1 (encumbrance), `intoxication` 0-1, `mood` (calm/alert/afraid/sad), `age` 0-1 (applied as modifiers on top of the class).
- Selection is a pure function (data table, mod-extensible): `(style_id, state) -> clipset` + modifier values. Same shape as UE Chooser tables / GTA clipset lookup [V/I].

**Layers (in order of cost, cheapest first):**
1. **Clipset swap by gait class (content):** locomotion set = idle, walk, run, start/stop/turn, ~12-16 clips. Switch via inertialized transition (our graph already blends this way) at stand/stop or by crossfade between speeds.
2. **Procedural/additive modifiers (runtime-cheap, all tiers A0-A2):** speed/stride warp; spine/neck hunch; asymmetric stride timing and hip drop for limp; breathing additive and slumped posture for fatigue; arm offset for load; tremor. Parameters = f(scalars), declared as tunables (CONTRIBUTING: no hardcoded tunables).
3. **Hybrid gait mix (cheap):** blend two clipsets (e.g. Neutral and Old) by `age` within the graph, like a blend space axis. Gives continuous age with 2 authored sets.
4. **Motion matching with style tags:** park (anim.md §0.2); a later locomotion-only spike could use per-style databases + a Chooser-like table. Do not plan around it.
5. **Physics tint:** only stagger/drunk/collapse events, A0/A1, capped count (§3).

**Interactions (sit, drink, doors):** one shared clip per verb per furniture band (motion-generation.md), plus: speed scale from `fatigue`/`age` (slower sit-down), additive posture (hunch, tremor), optional **second variant** only for few high-visibility verbs (sit-down: "heavy/old" vs "normal"; getting up from sitting). Do not multiply by style. Per-verb arm clips (drink, door open) stay shared; the additive layers carry style.

**Crowd tiers (LOD, animation.md "LOD and budget"):** A0 all layers; A1 clipset + stride/warp + hunch/limp additive (cheap bone ops); A2 clipset + speed match only (style = class only); A3 no pose (frozen) so style must be invisible there; R1/ambient idles use the class idle only. Rule: **the class carries identity at distance; modifiers are for close range.**

**Combinatorics budget (small team + generator) [I]:**
- Gait classes: 6-8 (neutral-male, neutral-female, old, heavy/large, child if needed, drunk, wounded/limp, afraid/sneaky). Feminine/masculine variants only if recognisably different; start with a shared neutral and a female variant.
- Per class locomotion: ~14 clips (8-dir walk, 4-dir run, idle x2, turn x2, start/stop). 7 classes x 14 = ~100 clips. With 100STYLE as base for some classes and edits for others, generated Kimodo/MotionBricks clips only for old, drunk, wounded: ~42 clips x ~5 candidates each x filter = ~200 generations, one overnight batch (40 s each = ~2.2 h).
- Idle/ambient variants per class: 3 idles + 3 fidgets = ~40 more (generated).
- Interactions: ~12 verbs x ~4 furniture bands = ~50 shared clips (already in the MG plan) + 2 "old/heavy" variants for sit/stand = ~8.
- Total: ~200 authored/accepted clips for class variety, versus a naive 7 styles x 50 interaction clips = 350+. Modifiers multiply look (age x fatigue x injury) at zero clip cost.

## 5. State and what this adds

Already known (do not repeat): MG recipes for activities per furniture band; Kimodo = activities, MotionBricks = links; 100STYLE as the best free locomotion source; park motion matching; additive hit reactions (motion-generation.md Decisions/Industry comparison; anim.md §0, §6). **New here:** (a) variety belongs in a data model (`MotionProfile`), not more clips; (b) a numeric style filter for Kimodo families; (c) MotionBricks v0.2.0 (SOMA, style presets, in-betweener) as a family generator spike; (d) a clipset + modifier layering with LOD rules; (e) physics tint restricted to stagger states.

**Owner answers 2026-11-25:** Q1 as recommended; Q2 yes; Q3 age is a per-NPC value set in code/data, Morrowind has no
"very old" people except chosen obviously old NPCs (override), visual mapping later, low priority (DECISIONS).

## 6. Open questions for the owner (player-facing; recommended answer first)

1. **Visible gait classes at launch?** Recommended: 5 (neutral, old, heavy, drunk, wounded/limp) plus continuous fatigue/load modifiers; add female/child later if the neutral set looks wrong. Alternative: just neutral + modifiers.
2. **Should a wounded NPC (or the player's own character?) visibly limp from combat damage in the game, with healing returning to normal?** Recommended: yes for NPCs and companions, tied to health bands, with the player character left as is until the owner tries it.
3. **Sex/build:** use the Morrowind race/sex record to choose the class, plus an `age` field added by us per NPC (Morrowind has none). Recommended: assign `age`/`style` by deterministic hash of NPC id with overrides in data.

## 7. Unverified

- GTA V/RDR2/TLOU2 internals: from general knowledge [I]; only the clipset mechanism is sourced.
- MotionBricks style control details, VRAM, and exact licence clauses on outputs: read the paper and the Open Model Agreement text before committing.
- 100STYLE licence and style list: verify in the dataset's own page.
