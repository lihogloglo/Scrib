# Kimodo prompting and constraints for game animation (2026-11-12)

Research only. Sources: Kimodo repo at `C:\mbtest\kimodo` (commit e1c219c9, docs in `docs/source/`), the [paper](https://arxiv.org/html/2603.15546),
[best practices](https://research.nvidia.com/labs/sil/projects/kimodo/docs/key_concepts/limitations.html), the [BONES-SEED card](https://huggingface.co/datasets/bones-studio/seed),
and our 229 labels (`animation packs/generated/loco1/verdicts.toml`: 38 keep, 61 maybe, 130 reject) with sidecars and `grades.json`.
Tags: [V] measured here, [D] NVIDIA docs/paper/code, [K] memory only, [H] hypothesis, untested.
Complements [18](18-kimodo-community-2026-11-12.md) (tips) and [17](17-autograde-2026-11-12.md) (grader).

## 1. What the model saw and how it is steered

- Training text: Rigplay clips have a whole-clip overview plus per-atomic-action captions, which an LLM (Qwen3-32B) paraphrases into "A [subject] ..." at varying detail [D, paper]. BONES-SEED: up to 4 natural descriptions (first three about 6 words, the 4th a longer sentence), 1 technical, 2 short lowercase labels; present tense, third person, subject "person / character / individual" [D, HF card]. So present-tense "A person ..." sentences of 6-15 words match both.
- Locomotion is SEED's largest category (74k clips: walking, jogging, jumping, climbing, crawling, turning, transitions) [D, card]. Trained styles: tired, angry, happy, sad, scared, drunk, injured, stealthy, old, childlike; the subject can carry one ("An old person...", "A drunk person...") [D, best practices].
- Text encoder: LLM2Vec (Llama-3 8B), one 4096-d vector per prompt. Unknown nouns degrade silently: "claymore" was worthless, "a big two-handed sword" is the fix [V, gen2 verdict in the [combat research](../../../bevyrrowind/docs/research/combat-anim-generation-2026-11-05.md)].
- CFG is `separated`, defaults text 2.0 and constraint 2.0 [D, `docs/source/user_guide/configuration.md`, paper]. We run the defaults [V, sidecars] at 50 steps (NVIDIA examples use 100; 50-100 is "slightly lower quality") [D]. Never swept by us. NVIDIA tried gradient guidance and dropped it [D, paper].
- Post-processing: foot locking, IK against skate, and a short optimisation that makes constraints hit exactly (root margin 0.04 m) [D, `kimodo/postprocess.py`]. The paper's metrics are without it; pelvis deviation from a root path is about 9.7 cm p95 before it [D].
- Duration is only `num_frames` at 30 fps; training clips cap at 10 s, evaluation 3-9 s [D]. No documented speed or duration trick [K]; speed comes from text plus root constraints.
- The "smoothed root" is a heavily smoothed pelvis XZ path; root keys constrain that signal, not the pelvis directly. Without it the paper saw straight walks become sneaking/old-person style [D, paper, `constraints.md`].
- Training constraints: sparse keys ramp 1 to 20, biased to few; dense paths exist for the root only; sparse and dense mixed 25 %, no constraints 10 % [D, paper].

## 2. What our labels say [V]

K/M/R = keep/maybe/reject of 6 takes per prompt.

| Group | Path | Reject | Notes |
|---|---|---|---|
| walk cycle, 3 prompts | 1.6 m/s | 16/18 | same prompt without path: R 1/6, K 4/6 |
| run cycle | 4.5 | 6/18 (K5 M7) | free: R 2/6. "runs forward at a steady pace" R 0/6 |
| sprint cycle | 7.0 | 14/18 | free also R 4/6: bad either way |
| walk start f / l90 / b180 | yes | 5/12, 9/12, 11/12 | turning starts worst |
| run start f / l90 / b180 | yes | 6/12, 2/12, 5/12 | "standing turns left and starts running" R 0/6 |
| walk stop | yes | 11/18 | |
| run stop | yes | 7/18 | "...comes to a quick stop, plants a foot and stands still" R 0/6; "jogs forward and stops quickly" R 6/6; "comes to a sudden stop" R 1/6 |
| pivots | yes | run 10/12, walk 9/12 | only 5 non-rejects |
| turn in place | fixed_xz | l90 5/12, l180 7/12 | "turns 90 degrees to the left on the spot" R 1/6 vs "turns to the left" R 4/6; "turns around to the left on the spot" R 2/6 vs "turns around" R 5/6 |

(Doc 17 quoted run path 13/18 on an earlier 175-take snapshot; with all 229 labels it is 6/18.)

Speed of the free takes (Hips XZ length / time) [V]: walk 1.23-1.74 m/s (kept ones 1.58-1.74), jog 1.96-2.42, sprint 2.42-3.12.
Forced: walk 1.62 (inside the free range), jog 4.53 (about 2x free), sprint 7.06 (about 2.5x).
Forced run cycles: flight 1.6 s vs 0.14 free, skate 87 vs 16 cm/s, jerk p99 3300 vs 970; forced sprint skate 206 vs 8 cm/s.

Reading: specific, in-style wording wins (direction + angle + "on the spot", an explicit end state like "plants a foot and stands still", "steady pace"); terse "stops quickly" and "turns around" lose. n = 6 per prompt, so treat as leaning, not proof.

## 3. Why the path-forced walks failed

Not the speed: 1.62 m/s sits inside the free walk range, and the free walk was kept 4/6. Candidates, by evidence:

1. **Our path is 7 keys at 1 Hz, linearly interpolated, then hit exactly by post-processing** [V, `recipe.py:430`, sidecar frames 0,30,...,179]. NVIDIA's own root-path example keys every frame (300 of 300, walking about 1.18 m/s, prompt "casually walking forward slowly") [V, `assets/demo/examples/kimodo-soma-rp/05_root_path`], and sparse keys are the under-20 regime meant for waypoints [D]. A piecewise-linear path demands constant pelvis speed; real gait surges per step. [H] The exact-hit step flattens that, giving the stiff, bouncy look (bob 0.07 vs 0.04-0.05 m, skate 27 vs 12-20 cm/s; doc 17).
2. **Full speed from frame 0.** The model must already be at gait speed at t = 0, unlike free takes, which ramp from idle [H].
3. **Speed out of distribution on run/sprint**: forced 2-2.5x above what those words produce alone (jog 2.2, sprint 2.7). Flight and skate numbers above are the signature [V].
4. **Prompt fights path**: "walks briskly" with a fixed slow-ish path was R 6/6; [D] says avoid contradictions.

Not the cause: heading (unconstrained on paths) [V, `kimodo_backend.py` build_constraints].

## 4. Prompting guide

Do
- Present tense, "A person ..." (or "An old person ..."), one or two behaviours, 6-15 words [D].
- Turns: direction, angle, place: "A person standing turns 90 degrees to the left on the spot." [V]
- Stops: name the end state: "...comes to a stop and stands still", runs add "plants a foot" [V].
- Use trained verbs and styles (section 1); plain object names.
- 6-12 samples per prompt, 2-3 phrasings, owner picks by elimination [V, practice].
- Keep post-processing on [D].

Don't
- Contradict text and constraints [D].
- Use rare nouns or non-trained activities (claymore, baseball) [D, V].
- Describe body parts at length or chain actions with "then"; use timeline prompts, each self-contained [D].
- Use words for exact speed. Words shift the distribution (jog about 2.2, sprint about 2.7 m/s) but not to a number [V].
- Use "tired" for upright cycles (a trained style, slouches) [K].

## 5. Constraint recipe per move type (proposed)

Rule: any speed-forced take uses dense per-frame root keys (NVIDIA's pattern) and a speed the free takes can reach; sparse 1 Hz keys are the control.

| Move | Prompt | Root constraint | Numbers |
|---|---|---|---|
| walk cycle | "A person walks forward at a relaxed pace." | none; plus a dense variant at 1.4 | 6 s; accept 1.2-1.75; retime to the gait table by up to 1.15x at bake |
| jog/run cycle | "A person runs forward at a steady pace." | none; dense variants at 3.0 and 3.5 | free jog about 2.2; key 4.5 only if 3.5 passes, else retime (up to 1.3x) |
| sprint | "A person sprints forward as fast as possible." | none; dense at 3.5 | free about 2.7; 7 m/s needs a mocap pack or a retime limit (owner call) |
| start | "A person standing still starts walking forward." | dense: hold 0.5 s, ramp over 1.5 s, end at the free speed | never key past what the free cycle reaches |
| stop | "A person walking forward comes to a stop and stands still." (run: add "plants a foot") | dense: steady, decelerate 0.8-1.0 s walk / 0.5-0.7 s run, hold at least 0.8 s | |
| turn in place | "A person standing turns 90 degrees to the left on the spot." | `fixed_xz` plus an end heading key [cos, sin] (untested) | 2.0 s (90), 2.5 s (180) |
| pivot | "A person runs forward, turns sharply to the left and runs back." | none first; then a dense U-turn, radius about 1 m | prove free first |
| loop | no guidance exists; cut a same-foot window at bake [18] | | at least 5 strides |

Alternative to forcing speed: generate free, measure speed (`motiongen grade`), retime at bake. A 10-15 % time scale hides well; 2x does not [K].

## 6. Batch-2 outline (loco set, 6 samples each)

- A. Speed/path sweep (answers section 3): walk prompt free; dense 1.4; dense 1.6; sparse 1.6 at 0.5 s keys; dense 1.6 with constraint weight 1 and 3 (needs `cfg_weight` plumbing, the one backend change). Jog at free, dense 2.5, 3.5, 4.5.
- B. Prompt A/B: stops and turns with the winning forms; drop "stops quickly", "turns around".
- C. Starts: paths end at the free speed; straight first, turning starts second (R 9-11/12 now), with "turns to the left and starts walking" and an end heading key.
- D. Steps 50 vs 100 on two prompts.
Grade with `motiongen grade`, owner rates, log in `docs/ledger/animation.md`.

## 7. Style sets (phrasing, untested [H])

Change only the subject or adverb, keep the recipe: "An old person walks forward." (old, encumbered feel), "An injured person walks forward with a limp.", "A drunk person walks forward.", "A scared person walks forward carefully." / "A stealthy person ..." (timid), "An angry person walks forward." (aggressive), "A person walks forward carrying a heavy object." (encumbered; the docs use "carrying an object" [D]), "A sick person ..." (not in the trained list: use tired or injured). Male/female is not a documented label and the card warns of stereotype bias [K]; take it from the skeleton and retarget, not the prompt. Style words change speed (old, tired, injured slower), so generate styles free; a dense path at base speed will fight them.

## Open

- Not read: NVIDIA GitHub issues, the HF discussion, the full tech-report PDF (dense-path sampling details); paper HTML gave summaries only.
- Section 3 causes 1-2 and all of sections 5-7 are hypothesis until batch 2A.
