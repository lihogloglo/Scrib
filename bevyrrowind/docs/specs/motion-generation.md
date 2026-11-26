# Motion generation: offline clip factory for ambient life (MG)

Status: spec 2026-10-28 (owner: "make a proper doc, then implement straight away"). Research and first tests:
[ambient-life-2026-10-28](../research/ambient-life-2026-10-28.md), activity list:
[ambient-catalog-2026-10-28](../research/ambient-catalog-2026-10-28.md). ROADMAP 3.8.

<!-- agent-summary -->
> **Agent summary.** An offline pipeline that turns activity recipes (prompt + constraints + furniture band + props)
> into many clips: NVIDIA Kimodo (text + constraints) for activities, MotionBricks (in-betweening) for links, SOMA BVH
> out, our existing BVH import + `sf_character` retarget onto `base_anim`, and a review renderer that plays each clip on
> a real Morrowind NPC with the real furniture and props, frame-stepped into MP4s. Slices MG1-MG8 below.
> Read: "Pipeline", then your slice. Tools: [scrib-motiongen](https://github.com/scrib-tools/scrib-motiongen) (Python, its own repo since 2026-11-23), Rust in `mw_bake`, app `clip-preview`.
<!-- /agent-summary -->

## Goal

NPCs that sit, lean, sweep, farm, drink, sleep and talk, in the hundreds of variants an open world needs
(catalog: about 140 activities × phases × variants × furniture bands, so **thousands of clips**). The clips must fit
Morrowind's own furniture heights and race sizes, hold Morrowind props, and play on the Morrowind rig. The owner
judges every clip visually on a real NPC, so review must be fast and in context.

Non-goals here: the runtime activity system (interaction points, claims, schedules, `sf_interaction`, `sf_ai`:
separate specs), runtime generation (rejected, research note), shipping weights.

## Decisions

- **Generators are offline tools, never in the game or the bake.** Python, outside Cargo, run on the owner's GPU.
  Weights live in the user's HF cache. The bake only sees the BVH files they write (rule 3: a pack like any other).
- **Kimodo makes activities, MotionBricks makes links** (research note). Kimodo: `Kimodo-SOMA-RP-v1.2`, text +
  constraints, SOMA-77 BVH. MotionBricks: `MotionBricks-SOMA-RP-v1`, transitions between our own poses (enter/exit
  links, approach to a slot); its sample BVHs are evaluation only and never feed shipped output.
- **Measure, don't assume.** Furniture heights come from the user's install (collision census, MG4), clustered
  into bands; race heights from `RACE` records. Recipes are generated once per band. `FullBodyIk` absorbs the rest
  at runtime.
- **SOMA BVH is the interchange.** Kimodo writes it natively (`--bvh`); MotionBricks output is written with the same
  hierarchy. Our BVH importer + `auto_map` + retarget do the rest, like Quaternius UAL and LAFAN1.
- **Review on the real rig.** The review renderer is the game itself (dev command `clip-preview`): real NPC body,
  real furniture static, real props, real retarget, fixed-step capture to MP4. No dummy, no second render path.
- **Repo policy until the licence check (owner decision pending).** Generated BVH, provenance and MP4s stay local in
  `<repo>/animation packs/generated/` (untracked, the BYO folder). Every clip has a provenance sidecar (model,
  version, prompt, seed, constraints, date) so a later decision can ship them like the CC0 packs, or regenerate.
- **Throughput target:** ≤ 10 s of wall time per accepted 6 s clip on the RTX 4060 laptop, so 2,000 clips fit a
  night. Measured baseline 2026-10-28: about 40 s per clip (80 s per prompt for 2 samples, mostly reloading the 8B
  text encoder on the CPU per call).

## Pipeline

```
recipes/*.toml ──> motiongen plan ──> jobs (recipe × band × variant)
                       │                     │
furniture bands <── MG4 census              ├─ kimodo worker (persistent; encoder + model loaded once,
race heights                                │   prompt embedding cache, batched samples)  ──> SOMA BVH + .prov.json
                                            └─ motionbricks worker (links between clip end/start poses) ─> SOMA BVH
                                                     │
                     animation packs/generated/<activity>/<phase>/<id>.bvh (+ .prov.json, sources fragment)
                                                     │
            MG3: BVH import (source `generated`, unit/up), auto_map SOMA -> humanoid -> base_anim (mw_bake)
                                                     │
            MG5-7: clip-preview: NPC + furniture + props, fixed-step capture ──> <id>.mp4 + review.html
                                                     │
                     owner verdicts (keep / reject / note) ──> verdicts.toml ──> sources.ron candidates (later)
```

### Recipes (MG1)

TOML, one file per setting (scrib-motiongen `motiongen/recipes/tavern.toml`), rows from the catalog:

```toml
[[activity]]
id = "sit.chair.drink"            # slot-style id: kind.family.variant
point = "seat"                     # interaction point kind (catalog column); selects the furniture band list
props = ["tankard"]                # prop classes (MG7 table)
phases = ["enter", "loop", "exit"]
variants = 4                       # samples per (phase, band)
duration = { enter = 3.0, loop = 8.0, exit = 3.0 }
prompt.enter = "A person standing in front of a chair turns and sits down on it."
prompt.loop  = "A person sitting on a chair drinks from a mug held in the right hand, then rests it on the knee."
prompt.exit  = "A person sitting on a chair stands up."
constraints.loop = { pelvis_height = "band.seat", root = "fixed" }
```

Constraint vocabulary (mapped to Kimodo constraints by the planner): `pelvis_height` (root height keyframes),
`root = fixed | path:<points>` (2D root path), `hand_l/hand_r = { height = "band.top", forward = 0.35 }`
(end-effector keyframes), `start_pose = "<clip id>:end"` (full-body keyframe, for chains), heights in meters or
`band.*` references. Planner scales SOMA-space targets by `soma_height / race_height` per band.
MG-C0 (2026-11-05): `pelvis_height` and `hand_*` also take a list of `{ ..., at = [s] }` tables (one value per keyframe
group), `root = "path:x,z@t;..."` times its points (seconds), and hands may combine with a path (the hand key's root XZ
is the path there). `python -m motiongen stats <bvh|dir>` prints skate/flight/root/hand/jerk numbers for any BVH.

### Generation workers (MG1, MG8)

- `python -m motiongen run <recipes...> --out <dir>`: one process keeps Kimodo and the LLM2Vec encoder loaded (encoder
  on CPU by default, 32 GB RAM), caches prompt embeddings on disk (hash of prompt + encoder version), runs
  `num_samples` per job in one GPU batch, writes SOMA-77 BVH + `.prov.json` per sample and a `jobs.jsonl` log
  (time per stage). Resumable: a job with all outputs present is skipped.
- Speed levers, measured in MG1: embedding cache, batch size, `diffusion_steps` (100 default; try 50), FP16, and
  keeping the encoder resident. Print a measurement line per run; no pass/fail on speed (tests rule).
- MotionBricks worker (MG8): links `A.end -> B.start` (approach slot -> enter, loop end -> loop start for seamless
  loops, exit -> idle), using the TensorRT engine (cached plan), written as SOMA BVH with the same provenance.

### Import onto the Morrowind rig (MG3)

- New source in `mw_graphs/data/animation/sources.ron`: `generated` (folder `animation packs/generated`,
  BVH, unit/up from the Kimodo export; SPDX `LicenseRef-generated-nvidia-open-model`, `private_only` until the
  owner decides). `auto_map` must map SOMA-77 names (Hips, Spine1, Spine2, Chest, Neck1/2, Head, Left/RightShoulder,
  Arm, ForeArm, Hand, fingers, Left/RightLeg = thigh, ...) to the humanoid map; fingers map when the target has
  them, else drop. Root motion from the pelvis (`root_from_pelvis`, like LAFAN1).
- Verification: a generated BVH cooks and retargets onto `base_anim`, `base_anim_female`, `base_animkna` without
  NaN; feet within a few cm of the floor in a standing clip (measurement test, printed).

### Furniture and race census (MG4)

- `bevyrrowind furniture-census [--json <path>]`: every `STAT`/`ACTI`/`CONT`/`FURN`-like record whose id or mesh
  matches the seat/table/bed/bench/stool/counter/shelf/bar patterns (list in the tool), with the baked collision's
  top surfaces: seat height (highest horizontal surface ≤ 0.8 m with a ≥ 0.25 m square footprint), table/counter top,
  bed top, footprint, mesh scale in refs (`XSCL`) distribution, ref counts per cell type. Clustered into bands
  (k-means or fixed 5 cm bins merged under ref-count weight). Plus `RACE` heights (male/female).
- Output `animation packs/generated/bands.json`, read by the planner. The census prints its tables (measurement).

### Review renderer (MG5-MG7)

`bevyrrowind clip-preview --clip <bvh> [--npc <id>] [--furniture <record id> [--at x,y,z,yaw]] [--prop
<class>=<record id>] [--camera orbit|front|side] [--out <dir>] [--fps 30]` (dev feature):
- MG5 **capture**: a minimal level (flat floor, noon light, no streaming), one NPC (default a Dunmer commoner,
  `--npc` any `NPC_`), the clip cooked and retargeted **in process** (the same Bevy-free cook + retarget calls the
  bake uses, no bake round trip), played in the full-body slot with root motion; `Time<Virtual>` stepped by a fixed
  1/fps per frame, a `Screenshot` per frame to PNG, then `ffmpeg` to MP4 (H.264). Exit when the clip ends. A
  `--batch <list>` mode renders many clips in one process (one load, one window).
- MG6 **furniture**: place a real `STAT` mesh by record id at the band's reference position (the seat under the
  clip's seated pelvis, from the provenance constraints), with its collision drawn on F-key debug only.
- MG7 **props**: a prop table (scrib-motiongen `motiongen/props.toml` → `mw` record ids: tankard `misc_com_tankard_01`,
  broom `misc_com_broom_01`, book, quill, pick, basket, cloth...) with a grip socket per class (bone, offset,
  rotation, two-handed second-hand point). Attach = a child entity on the hand bone; the second hand is a
  `FullBodyIk` effector (only in preview until the runtime activity system lands). Spawn/despawn at phase events.
- `review.html` in the output folder: a grid of MP4s per activity/phase/band with id, prompt, seed; the owner writes
  verdicts in `verdicts.toml` (`id = "keep" | "maybe" | "reject" | "note: ..."`, id = the clip's file stem), which MG9
  (later) turns into sources.ron candidates. `clip-preview --browse` writes it in the browsed folder: Y keep, N reject,
  M maybe (each advances), Backspace clear, U next unrated; sorted keys, existing entries kept, atomic save.

#### Browse review: diagnosis (AN9, 2026-11-26)

No timed owner session was possible (owner away); the evidence is the owner's notes (DECISIONS 2026-11-25: "the clip
picker tool is weak"; ~2 h per 200 clips, about 35 s a clip; rates by elimination: reject sure, maybe usable, keep
relatively best, then keep+maybe ranked by numbers, small batches), the code, and what `grades.json` already holds.

**Now.** One clip at a time: sorted by path, grouped by first folder (`browse.rs:84-111`), the clip cooked on demand and
looped (`live.rs:156-205`). The overlay shows activity, path, band, prompt and tallies (`live.rs:109-130`); keys Y/N/M
rate and advance, Backspace clears, U next unrated, L loop cut, arrows move (`live.rs:77-107`, `browse.rs:174-190`).
Nothing reads `grades.json` (gates `skate`/`jerk`/`turn_mag`, `mirror_candidate`, metrics, `_groups` triage).

**Gaps, most time first.**
1. **Gated clips are judged by eye.** Of 175 rated loco1 clips the gates caught 26 of the rejects at the cost of 6
   keep/maybe; each costs a full look. Fix: hide or sort gated clips last, show gates and metrics in the overlay
   (skate cm/s, jerk), jump past a triaged group. **AG2.**
2. **No comparison, only memory.** "Relatively best" needs two takes side by side, but takes of one recipe job are
   spread through the list (stems differ only by take number) and each is seen alone. Fix: view a prompt group
   (`_groups`) as 2-4 synced actors, reject drops one, the last get keep/maybe. **AN10** (groups from AG2).
3. **No second pass over keep+maybe.** After elimination the owner must rank by numbers, but the tool has no filter
   (unrated, maybe, keep) and no ordering by metric. Fix: view filter and sort key (gate count, skate, jerk, prompt
   group), an "unrated only" mode as the default queue. **AG2** (sort, filter); the keep/maybe ranking = **AG3** score.
4. **A mistaken key is hard to fix.** Y/N/M advance first, so Backspace clears the *next* clip (`browse.rs:174-190`);
   the fix is Left, Backspace, Right. Fix: Z undoes the last verdict and returns to it. **New** (small, with AN10).
5. **Judging contact needs slow motion.** Skate and sliding feet are what rejects hinge on, yet there is no pause, frame
   step, speed or foot-contact marker; only a fixed loop. Fix: Space pause, `,` `.` step, `[` `]` speed, optional
   contact-lock markers from the grader. **New** (with AN10: sync control is shared).
6. **Wrong-way turns look like rejects.** `mirror_candidate` clips are usable relabels but show as plain bad takes.
   Fix: show the flag and preview the mirror. **AG2** (flag), mirror preview **new**.
7. **A clip is seen on one body, free-standing.** Skate or foot trouble that only shows at slot speed or on another
   rig is missed until in game. Fix: **AN11.**
8. **Notes need a text editor.** `note: ...` verdicts cannot be entered in the tool. Fix: a number key tags a reason
   (skate, slide, pop, pose) into the note. **New** (small); these tags feed AG3 calibration.

**Industry form.** Dailies tools (ShotGrid/Tweak RV, KeyFlow) review versions with synced A/B, wipe and tiled compare,
frame step and per-version status keys, never one clip at a time from memory. Mocap pipelines pick a **circle take**
from several takes of one move, then pass only circled takes on, with notes. Circle-take (AI-take dailies room) is the
closest match: takes organised per shot, auto quality gates first, compare, circle winners, export prompts and seeds. We
follow that: gates remove the sure rejects before any look (gap 1), groups are compared in a row (gap 2), the winner is
circled (keep). Sources: [RV on ShotGrid](https://shotgrid-rv.software.informer.com/),
[KeyFlow](https://keyflow.tools/), [circle-take](https://github.com/wassermanproductions/circle-take),
[Animost mocap take selection](https://animost.com/tutorials/motion-capture-for-games-planning-and-direction-part-3/).

**Roadmap.** Gaps 4, 5 and 8 have no row: add **AN12** (review controls: undo, pause/step/speed, reason tags;
`mw_viewers_bevy`, depends AN10, exact list state). For the 246 loco2 clips: run `motiongen grade` first, and until
AG2 lands review the ungated clips first (`grades.json` lists the gated ones).

**As built (AG2, 2026-11-26):** `--browse` reads `<folder>/grades.json` (absent or broken: unchanged behaviour): inside each
activity ungated clips first, then gated, then failing groups; **C** candidates only (gated hidden), **V** view all → unrated →
maybe → keep (arrows, Y/N/M advance and U stay inside it); overlay grade line (gates, skate, jerk p99, steps, group gated/total,
mirror candidate). `browse.rs`, `live.rs`; 6 exact tests.

### Interaction pieces and hub poses (AN43, owner 2026-11-26)

**Rule.** Interaction chains (sit, reach the cup, drink, put it down, stand up) are **assembled at runtime from
pieces, never generated as one take**. Every piece starts and ends on a listed **hub pose**, so any piece that ends on
hub H can be followed by any piece that starts on H. Loops play at one hub (start = end = H). Slot clip ids stay the
interaction spec's (`<kind>.<family>.{enter.<side>,loop,exit.<side>}`, `interaction.md` "Definitions"); hubs are
metadata on those clips, not a second naming scheme.

- **Hub = body hub + hand state.** Body hubs: `stand.idle`, `sit.<seat>` (stool, chair, bench, floor, bed edge),
  `lie.bed`, `lean.wall`, `kneel` ...; each hand is `empty` or holds a prop class (`cup`, `book`, `bowl` ...), so
  "seated holding a cup in the right hand" is `sit.chair+R:cup`. A piece that doesn't use a hand keeps its state
  (upper-arm-down mask), so hand states don't multiply body pieces (The Sims' posture + carry split).
- **Hub list per family is data** (`recipes/hubs.toml` in scrib-motiongen, copied beside the output as
  `generated/hubs.toml`; Scrib reads it, files only): `id`, family, body hub, hands, and its **source**: one frame of
  a kept SOMA take (clip id + frame), never hand-typed angles. The MW-rig hub is that frame through the normal retarget
  (MG3), so generation and bake share one definition. Style classes may add their own body hub
  (`stand.idle@old`); missing ones fall back to the neutral hub, inertialization absorbs the gap.
- **Piece metadata:** each generated piece's provenance names `from_hub` and `to_hub`; the bake copies them into the
  cooked clip (`sources.ron` row tags). A chain is valid when consecutive pieces share a hub (load error otherwise).
- **Generation:** hub poses go to the generator as start/end constraints (Kimodo keyframe constraints, ARDY
  keyframes, MotionBricks in-betweening between two given poses); AN44 measures which hits hubs best. Unconstrained
  takes are allowed only through the bake's hub snap and gate (AN45: blend the last/first frames into the exact hub,
  reject above a tunable hub error, beside the foot-slide gate).
- **Runtime** (no new system: existing `sf_anim_graph` and `sf_action` parts): the enter piece is warped onto the slot
  frame (`Interaction` move rule's Warp, action.md); prop pieces play as layers with masks over the seated or standing
  body; IK pins hands and feet to the slot's contact points (`IkTargets`, two-bone / `FullBodyIk`); every piece
  boundary is an inertialized switch (the residual after the snap is millimetres and degrees, so it never pops).
- **Hub error** (the gate's measure and AN44's table): per joint local rotation error (deg, max and mean) and hand/
  foot/pelvis position error in the character frame (cm), at the piece's first and last frame.

Reuse check: Bevy 0.20's `AnimationGraph` has blends and masks but no posture/transition contract; our own
`sf_anim_graph` already has slots, masks, inertialization and warp. No crate implements a posture graph. Industry:
hubs are Snap-Together Motion's "hub nodes" (Gleicher et al., SIGGRAPH 2003: clips snapped to shared poses so any
order plays); The Sims' posture graph (postures and carry as separate tracks); RDR2/GTA scenario clip sets (enter,
idle, exit per scenario, base idle as the junction); UE5 Smart Objects + Contextual Animation + Motion Warping for the
slot alignment; inertialization at the junctions (Bollo, GDC 2018). AN25 fills the per-family hub table; this
section fixes the rule and the format.

### Interaction families (AN25, 2026-11-26)

The contract AN26 (sit/stand), AN28 (doors), AN30 (beds, lean) and AN31 (reactions) are briefed from. Names are the
interaction spec's (`superfargoth/docs/specs/interaction.md` "Definitions": `<kind>.<family>.{enter.<side>,loop,exit.<side>}`;
sides Front/Back/Left/Right, ladder Bottom/Top) and AN43's hubs; only the hub ids marked **new** are added. A **piece**
is one clip, a **family** one slot clip set. A hub is shared across families: a chair `exit` ends on `stand.idle`, the
hub a door `enter` starts on.

**Conventions.** Slot frame = the def's (root on the floor, +Z the way the user faces, seat/lean at the band height;
`interaction-adapter.md` IN32a); every contact below is in that frame. A one-shot family (door, lever, reactions)
carries only `enter.<side>`; held poses add `loop` and `exit`. Loops play at one hub. Approach and departure are MG8
link pieces and the locomotion graph, never part of a family.

#### Families, pieces, hubs

| Slot clip family | Generated pieces (mirrored side derived) | Hubs | Morrowind side |
|---|---|---|---|
| `seat.stool`, `seat.chair`, `seat.bench`, `seat.floor`, `seat.bed_edge` | `enter.front`, `enter.left` (`.right` mirrored), `loop` x2 (rest, fidget), `exit.front`, `exit.left` (`.right` mirrored) = 6 | enter `stand.idle` -> `sit.<seat>`; loop at `sit.<seat>`; exit back to `stand.idle` | 32 seat records, bands 0.14 / 0.48 / 0.54 / 0.64 m; bed edge from the bed records |
| `bed.lie` | `enter.front`, `enter.back`, `loop` x2 (sleep, restless), `exit.front`, `exit.back` = 6 | `stand.idle` -> `lie.bed` -> `stand.idle` | 46 beds, bands 0.22 / 0.52 / 0.64 / 0.74 m (bedroll, bunk, hammock, bed) |
| `lean.wall` | `enter`, `loop` x2, `exit` = 4 (shoulder side mirrored) | `stand.idle` -> `lean.wall` -> `stand.idle` | no wall def in IN32: framework family, Morrowind spots need a cook rule |
| `counter.table`, `counter.bar` (lean stations) | `enter`, `loop` x2, `exit` = 4 | `stand.idle` -> `lean.counter` -> `stand.idle` | 28 tables 0.84 / 0.98 m, 12 counters 0.94 m; stations by `counter_spacing_m` |
| `door.push`, `door.pull` | `enter.front`, `enter.back` for hinge L (hinge R mirrored) = 2 per family | `stand.idle` -> `stand.idle` (stop, open, step through) | vanilla doors: hinge at the origin, 90 degrees, one way; travel doors are not slots |
| `door.knock`, `door.rattle` (locked), `door.bash` (IN37) | one `enter.<side>` each, usable from both sides = 3 | `stand.idle` -> `stand.idle` | "locked" is crime's state, read at the gate |
| `lever.pull` | `enter.up`, `enter.down` (the two positions) = 2 | `stand.idle` -> `stand.idle` | scripted activators stay scripts; framework family |
| `ladder` | `enter.bottom`, `enter.top`, `loop.up`, `loop.down`, `exit.bottom`, `exit.top` = 6 | `stand.idle` -> `climb.ladder` (**new**) -> `stand.idle` | none in the IN32 census; framework family, later |
| `workstation.stand` | `enter`, `exit`, one `loop` per job (first batch: anvil, alchemy bench, cooking pot) = 5 | `stand.idle` -> `stand.idle` (tool by layer) | anvils, alchemy benches, pots (IN42); jobs are mod data |
| Reactions (AN31, **not slots**: no claim, any NPC) | `turn.l90` (+ mirror), `turn.180`, `placate` in/loop/out, `cower` in/loop/out, `flee.start`, `surrender` in/loop/out, `talk` x4 loops (upper-body mask) = 16 | `stand.idle` -> `stand.idle`; held ones end on `crouch.cower`, `kneel.surrender` (**new**) | played by AI and dialogue |

Chain example: `stand.idle` -> `seat.chair.enter.front` -> prop pieces (AN27, at `sit.chair+R:cup`) ->
`seat.chair.exit.front` -> `stand.idle`. Beds go `stand.idle` -> `lie.bed` in one piece; a route through `sit.bed_edge`
is two existing families chained, not a new piece.

#### Contact constraints (slot frame)

| Family | Contacts | When | To the generator | At runtime |
|---|---|---|---|---|
| seat | pelvis at band + hub offset (`sit.chair`: 0.53 m pelvis at the 0.48 band, so +0.05) over the slot centre; feet flat on the floor; hands on thighs, or on the table edge when `reach` > 0 | enter: settle frame (last ~20 %, tunable) to the end; loop and exit start: all frames | Kimodo `fullbody` keyframes at first and last frame (the hubs), `pelvis_height` (`root_y_pos`) at the settle frames, foot end-effectors on seated frames, `root2d` path for the entrance side | enter warped onto the slot frame (translation + yaw); `IkTargets` pins feet, hands to the table; vertical IK absorbs the band residual (tunable, Guess 3 cm) |
| bed | pelvis and back on the top at band + offset, head at the slot's +Z end | enter: from the lie-down frame; loop: all | as seat, `fullbody` on `lie.bed`; one set per band group | warp as seat; no hand pins |
| lean | wall: back (and heel) on the wall plane; counter: forearms or hands on the top at the counter band; feet planted | loop: all; enter: from the contact frame | `fullbody` at both ends; hand end-effectors at the counter band | hands pinned to the top within `reach` |
| door | one hand on the handle at the contact event (a clip event, like the lever's), handle height from the placement; knock: fist at the leaf; bash: shoulder at the leaf | one frame, then the swing | right-hand end-effector at that frame (the hand constraint is weak, MG1 gaps: IK fixes the last cm) | enter warped to the handle point (Motion Warping idiom); the leaf swing starts at the event |
| lever | hand on the handle from contact to the end of the stroke | the stroke | as door | hand pinned; lever position driven by the event |
| ladder | both hands and both feet on rungs, every cycle (rung spacing of the def) | loop: each cycle | `fullbody` hubs; cycle length = whole rungs, set by duration and prompt | `Docked` mover along the edge chain; hands/feet IK to the rungs |
| workstation | hands on the surface or tool at the job's contact frame; feet planted | each loop step | `fullbody` hubs, hand end-effectors for the contact | the def's `IkTargets` |
| reactions | none (no object) | n/a | hubs only | mover turns the yaw for `turn.*`; no pins |

Bands come from `bands.json` (MG4): one piece set per band cluster, picked by the slot's `band`; the cook's band is the
truth, not the family name. MotionBricks (ARDY note §5) takes up to 4 target poses and fills between them: the same hub
and contact frames serve it as keyframes when AN44 picks it.

#### Variant axes and cost

| Axis | Coverage | Cost |
|---|---|---|
| Side (L/R) | generate Left, mirror Right when the piece is symmetric (`mirror_candidate`, AG1); handed ones (lever, door handle side) generated per side | x1 to x2 |
| Band | one set per census band cluster; vertical IK absorbs the residual | seat x4, bed x3 groups, counter x2 |
| Seat type | one family each; bench places (0.55 m) reuse `seat.bench` | in the table |
| Style class (AN18: old, heavy, ...) | own hubs `stand.idle@old`, `sit.chair@old`; a missing class falls back to neutral, inertialization absorbs the gap | x2 per class per family-band |
| Body (races, female rig) | band and race heights (MG4) pick the set; same pieces retargeted per rig (MG3); female neutral is an AN18 style class | x1, no new generation |
| Hand state | layer + upper-arm mask over the body piece; hand-state hubs only where a prop chain starts or ends (`sit.chair+R:cup`, AN27) | not multiplied |

**First batch, AN26: 36 generated pieces + 12 mirrored.** Stool, chair (2 bands), bench, floor = 5 family-bands x 6 = 30,
plus the old chair at one band = 6; mirrored right-side enter/exit 2 x 6 family-bands = 12. At 8 seeds: 288 takes to
grade. AN28 doors: 7 generated + 4 mirrored. AN30: beds 6 x 3 band groups = 18, lean wall 4, counter 4 x 2 bands = 8.
AN31: 16. Lever 2, ladder 6, workstation 5 come after.

#### Hubs to seed (`recipes/hubs.toml`; the generation slices fill sources from kept takes)

| id | family | body hub | hands | slice |
|---|---|---|---|---|
| `stand.idle`, `sit.chair` | any, chair | | empty | exists |
| `sit.stool`, `sit.bench`, `sit.floor`, `sit.bed_edge` | stool, bench, floor, bed_edge | `sit.<seat>` | empty | AN26 |
| `stand.idle@old`, `sit.chair@old` | any, chair | style `old` | empty | AN26 |
| `lie.bed` | bed | `lie.bed` | empty | AN30 |
| `lean.wall`, `lean.counter` | lean, counter | `lean.wall`, `lean.counter` | empty | AN30 |
| `climb.ladder` | ladder | `climb.ladder` | on rungs (not a prop) | later |
| `crouch.cower`, `kneel.surrender` | reaction | same | empty | AN31 |
| `sit.chair+R:cup`, `+R:bowl`, `+R:book` | chair | `sit.chair` | R holds the prop | AN27 |
| `stand.idle+R:tool` | workstation | `stand.idle` | R holds the tool | workstation |

Doors, levers, turns and talk use `stand.idle` only.

Reuse check and industry comparison: as in the paragraph above (Sims posture graph, RDR2/GTA scenario clip sets, UE5
Smart Objects, Contextual Animation, Motion Warping). Per family: seats and beds are posture hubs (Snap-Together
Motion); doors, levers and workstations warp one contact frame onto the placed handle or tool (the Motion Warping
idiom) with the contact stored as a clip event; the ladder is a looped cycle docked to an edge chain. Bevy 0.20 has no
equivalent; our slot, IK and warp parts already cover the runtime.

## Slices (one deliverable each)

| Slice | Deliverable | Needs | Status |
|---|---|---|---|
| MG1 | `motiongen` Kimodo batch worker + recipe/plan format + speed measurement | Kimodo env | done (uncommitted; see As built) |
| MG2 | 10 seed recipes from catalog Tier 1-2 (sit, lean, sweep, drink, farm, talk, sleep...) | MG1 format | todo |
| MG3 | SOMA BVH source + auto_map onto `base_anim` (+ tests) | a Kimodo BVH | done (uncommitted) |
| MG4 | furniture + race census CLI → `bands.json` | install | done (uncommitted; see As built) |
| MG5 | `clip-preview`: NPC + clip in process + fixed-step capture → MP4 (+ `--batch`) | MG3 (start with a UAL clip) | done |
| MG6 | furniture static placement in `clip-preview` | MG5, MG4 | done (hand-placed; band auto-align later) |
| MG7 | props table + hand attach + second-hand IK in preview | MG5 | MG7a done (one prop, hand attach); MG7b grip table + IK todo |
| MG8 | MotionBricks link worker | MG1 | todo |
| MG9 | verdicts → sources.ron candidates + review.html | MG5 | later |
| MG10a | interactive browse: `clip-preview --browse <dir>` (keys, loop, orbit, prov overlay) | MG5 | done |
| MG-C0 | combat/athletic feasibility run (`recipes/combat-feasibility.toml`, `stats`), 2026-11-05 | MG1 | done; results in research/combat-anim-generation-2026-11-05.md |
| AG1 | `motiongen grade <dir>`: physical gates (skate, jerk, turn_mag, no_step; mirror_candidate flag) + prompt-group triage -> `grades.json` (design: research/animation/17-autograde) | MG1 | done 2026-11-12 (selftest fixtures) |

Order: MG1, MG3, MG5 in parallel (Python / bake / app), then MG4, MG2, MG6, MG7, MG8.

## Verification

Sanity only (tests rule): generated BVH parses, cooks, retargets without NaN on the three skeletons; the preview
renders N frames for an N-frame clip and writes an MP4; the census finds chairs/tables/beds in Vanilla. Measurements
(printed, `#[ignore = "measurement: ..."]` or tool output): seconds per clip by stage, feet height error, census
tables. Quality is the owner's verdict in `review.html`.

## Industry comparison

Studios use text-to-motion and in-betweening to block out and fill large ambient sets, then clean the keepers
(Ubisoft La Forge's learned in-betweening; NVIDIA's MotionBricks UE5 demo; Kimodo/ARDY for authoring). Ambient sets
are scenario/workspot libraries placed on furniture markers (GTA scenarios, Cyberpunk workspots, Skyrim furniture
markers, UE Smart Objects). We generate per furniture band instead of hand-fitting each mesh, because Morrowind's
furniture is irregular and we have no animators.

## Open questions

- Licence: do Bones RigPlay terms reach model outputs? Copyright status of generated motion. Until answered,
  generated clips are local only.
- Fingers: SOMA-77 has them, `base_anim` hands are coarse; do we keep generated finger curves or per-prop hand poses?
- Beast races (`base_animkna`): digitigrade legs on a plantigrade source; check in MG3.

## As built

MG1, MG3, MG4, MG5, MG6, MG7a and MG10a (2026-10-28) moved to [archive/motion-generation-as-built-2026-10-28.md](archive/motion-generation-as-built-2026-10-28.md) (sections keep their "As built (MGn)" titles).
