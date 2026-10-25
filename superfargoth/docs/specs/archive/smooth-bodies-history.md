# Smooth bodies history

Dated history blocks moved verbatim from [smooth-bodies.md](../smooth-bodies.md) (doc cleanup S10b, 2026-10-26).

## Survey (smoothbody2, 2026-10-12)

*Superseded*: measured on misplaced rigid parts (NIF root transforms were dropped). Corrected numbers
are under "Survey rerun and guard" below.

`bevyrrowind/crates/mw_actors/examples/smooth_survey.rs`, output `shots/smoothbody2/metrics.txt`.
One NPC per playable race × sex (first in load order), naked; plus eight outfits on a male dark
elf, a female Imperial and a male Argonian: netch leather and glass (light), bonemold (medium),
iron (no iron boots in the data) and daedric (heavy), a common robe, a skirt with shirt, shirt
and pants. 44 cases, 7 poses: elbows 90°, knees 90°, shoulders raised 57°, `WalkForward` and
`RunForward` at 25% and 75% (female and beast clips from their own `.kf`; the female rig has no
run frames).

Metrics. **Opening**: how far each seam vertex moves away from its closest point on the other
part, relative to rest (the closest point is found at rest and carried by its triangle). Max over
seams and poses; the median is 0.0 mm in every case (most seams don't open at all). Seams between
two authored skins are left out: the step doesn't touch them. **Pokes**: vertices of a part that
were inside a linked part at rest and are outside it in the pose (> 1 mm), worst pose.

Naked, max opening vanilla → smooth (pose), pokes vanilla → smooth:

| Race | Male | Female |
|---|---|---|
| Argonian (digitigrade, tail) | 183 (run) → **16** (run), 12 → 5 | same body: 183 → **16**, 12 → 5 |
| Breton | 53 (run) → **5**, 24 → 6 | 264 (knee) → **32** (knee), 33 → 12 |
| Dark elf | 55 (run) → **9**, 30 → 8 | 264 (knee) → **42** (knee), 34 → 10 |
| High elf | 55 (run) → **9**, 28 → 11 | 41 (elbow) → **1**, 27 → 7 |
| Imperial | 53 (run) → **5**, 23 → 6 | 264 (knee) → **32** (knee), 32 → 11 |
| Khajiit (digitigrade, tail) | 177 (run) → **5**, 11 → 4 | 177 (run) → **3**, 9 → 3 |
| Nord | 99 (run) → **5**, 13 → 5 | 264 (knee) → **32** (knee), 7 → 7 |
| Orc | 85 (run) → **9**, 21 → 8 | 53 (knee) → **4**, 10 → 3 |
| Redguard | 82 (run) → **6**, 29 → 9 | 264 (knee) → **32** (knee), 32 → 11 |
| Wood elf | 82 (run) → **13**, 31 → 11 | 264 (knee) → **42** (knee), 31 → 10 |

Outfits (male dark elf / female Imperial / male Argonian), max opening vanilla → smooth:

| Outfit | Dark elf m | Imperial f | Argonian m |
|---|---|---|---|
| netch leather | 63 → 22 | 46 → 6 | 93 → 44 |
| glass | 98 → 22 | 85 → 22 | 183 → 21 |
| bonemold | 51 → 6 | 25 → 10 | 104 → 35 |
| iron | 112 → 8 | 106 → 8 | 104 → 44 |
| daedric | 110 → 22 | 113 → 14 | 104 → 35 |
| robe | 14 → 13 | 9 → 14 | 3 → 13 |
| skirt + shirt | 68 → 2 | 63 → 3 | 7 → 1 |
| shirt + pants | 83 → 17 | 267 → 65 | 193 → 60 |

Pokes fall or stay equal in 43 of 44 cases (typically 30 → 8); they rise only for iron on the Imperial female (19 → 23,
all ≤ 8.3 mm deep). **Determinism**: all 44 cases bit-identical on a second thread. **Already
skinned content**: unit test `authored_skins_pass_through` (a seamless skinned body and two
authored pieces welded at a seam come out unchanged); on real content, smoothing the smoothed
result changes nothing in 42 of 44 cases (bonemold on two bodies changes 20 vertices: a part left
exactly rigid by the first pass is re-solved against its now-authored neighbors). **Cost**:
`smooth_body` 3–23 ms per case, mean 10 ms, 450 ms for all 44 (release, BelowNormal priority with
the owner on the machine: indicative).

**Remaining failures** (the full list with part names is in `metrics.txt`, "Failures"):
- *Systematic, female legs* (all female humanoid races except High elf and Orc): the knee at 90°
  still opens 32–42 mm at the upper-leg ↔ knee-piece overlap (vanilla 264 mm). The female upper
  leg wraps far around the knee piece; the thigh's rim is the parent side and stays nearly rigid,
  so the knee piece bends just under it. Fix candidates: let the shell's rim blend too (a
  two-sided overlap band), or a per-joint band from bone length (Next steps).
- *Systematic, armor wrists and pauldrons*: bracer ↔ forearm-guard (`fa`/`w`) and pauldron
  shoulder ↔ upper-arm pieces (`cl`/`ua`) open 13–22 mm in elbow and run poses (vanilla 50–110).
- *Systematic, beast tails with clothing or armor*: tail (authored) ↔ greaves / pants open
  35–60 mm in run frames (vanilla ~100–190); the naked beast bodies are fine (≤ 16 mm). Tail and
  pelvis pieces have no common rest seam; candidates: exclude links to tail joints, or pin the
  tail root ring.
- *Robe ↔ bare wrist*: 12–14 mm at elbow 90° where vanilla is 3–14: the wrist rim is welded to
  the authored hand, so it can't also follow the authored robe sleeve. One-off per outfit type.

## Survey rerun and guard (smoothbody3, 2026-10-12)

**Input fix first.** `mw_nif` used to drop each NIF's root transform. Rigid body parts need it (pants
ankle 0.37 m, knee 7 cm, groin 12 cm), so every earlier survey number, vanilla and smooth, was measured
on misplaced parts (fix by feetfix, `.sessions/feetfix.md`). On corrected input the **female knee
failure goes away** (vanilla 46-53 mm, smoothbody2 algorithm 3.5 mm). The 264 mm "knee" came from the
upper-leg shell sitting 19 cm too low. A pivot-plane "overhang" rule built for it never fires on
corrected input and was removed. All numbers below are on corrected input, in
`shots/smoothbody3/metrics.txt` (smoothbody2's algorithm on the same input: `shots/sb3_sb2base/`).

Changes in `sf_character::smooth`:
1. **Per-seam guard** (`guard_seams`): never worse than the original, seam by seam. For each seam (part
   pair), the widest opening over caller-supplied probe poses (`seam_openings`, the survey's metric)
   is compared with the original weights. If smooth is wider by more than `tolerance` (survey 0.5 mm),
   weights are restored in widening steps until no seam is worse: (1) the changed vertices nearest that
   seam, (2) the vertices of the offending links and the triangles they measure against, (3) every changed
   vertex within `radius` (survey: `band`), (4) both whole parts, positions included.
   **Trigger rate: 25 of 474 seams (5%) in 44 cases.** The triggers are robe ↔ bare wrist (all 6), glass
   forearm guard ↔ bracer (5), gauntlet ↔ forearm (3), armor knee guard / ankle / foot pieces (11),
   cuirass ↔ greaves pelvis (1). After the guard, **0 seams are worse than vanilla**. smoothbody2 had 28.
2. **Sibling overlap order**: between pieces on equally deep joints (two pieces hung on one bone), the
   hidden side follows the visible one: a rim inside the other piece follows it, a shell rim leads.
   Elsewhere the child still follows the parent.
3. **Sibling-branch rule** (`hops`): the shallower side must be the common ancestor or a *leaf* joint
   right below it (a piece hung on the parent bone). This drops tail ↔ trouser-leg links.
4. **Authored rims meet their nearest part or nothing**: an authored rim (tail root, hand) whose nearest
   part is not skeleton-adjacent passes through it (a tail through trousers) instead of linking to the
   next part over.

Max seam opening, vanilla → smoothbody2 algorithm → smoothbody3 (mm, corrected input):

| Group / case | vanilla | sb2 | **sb3** |
|---|---|---|---|
| Female knee (all races) | 46-53 | 3.5-4.0 | **3.5-4.0** (input bug, see above) |
| Robe ↔ bare wrist (dark elf m / imperial f / argonian m) | 13.9 / 8.5 / 2.9 | 12.5 / 14.4 / 12.6 | **1.4 / 0.1 / 2.9** (guard: vanilla weights at the wrist) |
| Glass bracer `fa` ↔ `w` | 0 on that seam | worse than vanilla | **vanilla** (guard); case max 3.9 / 2.1 / 10.8 |
| Pauldron `cl` ↔ `ua` (daedric dark elf m / imperial f) | 98.8 / 68 | 21.5 / 13.5 | **21.5 / 13.5** (unchanged) |
| Tail: netch / clothes on argonian m | 93 / 193 | 44.2 / 60.2 | **17.9 / 14.4** |
| Tail: iron on argonian m | 104 | 44.1 | **44.1** (tail ↔ greaves pelvis) |
| Naked argonian m/f (tail ↔ groin) | 67 on that seam | 16.3 | **46.5** (regression, under vanilla) |
| bonemold on argonian m | 74 | 34.7 | **39.0** (regression) |
| iron on imperial f | 106 | 8.0 | **18.3** (guard trade-off) |
| daedric on argonian m | 91 | 35.4 | **66.4** (guard trade-off; 32.8 before the guard) |

The other 31 cases are bit-identical to smoothbody2's algorithm. Determinism: 44/44 bit-identical on a
second thread. Idempotence: 39/44 (it was 44/44 on corrected input without the guard; the guard reverts
against the already-smoothed input). Cost: mean 12.9 ms per case including the guard (indicative).

**Trade-offs**:
- *Guard*: iron on imperial f and daedric on argonian m get worse case maxima. A sibling seam (knee guard ↔ ankle
  piece, vanilla 0 mm) opened a few mm, and restoring it reopened the knee or tail seam beside it, up to
  that seam's vanilla value. This follows the owner rule (never worse than vanilla, per seam). A softer
  rule (a relative tolerance, or keeping a seam when its revert worsens the case maximum) is an owner call.
- *Tail rules*: a clothed beast is far more common than a naked one. Netch and clothes go from 44-60 to
  14-18 mm; naked Argonians go from 16 to 46 mm at tail ↔ groin (vanilla on that seam: 67).

**Still open**:
- Tail root ↔ groin/greaves pelvis: 44-66 mm in run25. The tail root ring is 100% on the first tail
  joint, and the rigid pelvis piece follows it only at the nearest triangle corner. Candidate: make all
  corners of the linked triangle follow an authored rim, or give the tail root a falloff.
- Pauldrons `cl` ↔ `ua`: 13-22 mm, not addressed.

## Live (smoothlive, 2026-10-12)

Wired into `sf_character_bevy`; on by default, A/B in the app.

- **`sf_character::smooth_outfit`** (feature `smooth`, module `smooth_outfit`): the assembly step. Per
  visible part `SkinSurface::from_cooked` plus the same submeshes decoded (`DecodedPart::decode`, node
  filter applied identically, so vertex order matches one to one), then one `smooth_body` over the set,
  then `guard_seams` over the rig's probe poses (tolerance 0.5 mm, radius = band). Parts the step changed
  come back as `DecodedPart`s in rest space: positions and normals in skeleton model space, weights on a
  palette of the joints they use with inverse bind `rest⁻¹` (`bind_palette`). Unchanged parts stay cooked
  (no palette growth). Left out, as authored: parts with morph targets (heads keep their own mesh and
  bind-space deltas), hair (`CharacterRecipe::hair`: stays on its joint; `transfer_weights` is not used
  live, it suits only skinned cloth, which vanilla content lacks), parts with a joint the skeleton lacks.
  Fewer than two parts, or no probe poses: unchanged.
- **`assemble`** takes pre-decoded parts: `AssemblyPart::decoded: Option<&DecodedPart>` (palette and
  submeshes replace the cooked ones; materials and morph infos still from the cooked part).
  `AssemblyPart::new(cooked, body_part)` for the plain case.
- **`sf_character_bevy`**: `SmoothBodies::{Smooth (default), Vanilla}` in `CharacterSettings` (resource)
  and in `OutfitCacheKey::bodies`; the build task smooths before `assemble`. `CharacterContent::smooth_probes`
  (default empty: outfits stay as authored, with a warning) gives model-space probe poses per skeleton;
  `SmoothRig` (rest, parents, skinning matrices) is cached per skeleton. Changing `CharacterSettings`
  marks every `Character` changed, so outfits rebuild through the no-pop swap (rig and animator kept); both
  looks stay cached. `CharacterStats::{smoothed, smoothed_parts, smooth_reverted, smooth_ms}`.
- **Adapter** (`mw_actors::probes::smooth_probes`): the survey's poses from the skeleton and its cooked
  clips (elbows/knees 90°, shoulders 57°, `WalkForward`/`RunForward` at 25%/75%, root motion removed).
- **App**: `--bodies vanilla|smooth` on `view`/`scene`/`character`, **B** toggles live (overlay line in the
  F1 help), `character --walk` (1 walks in place, 2 runs), `character "id+item+item"` dresses an NPC in
  exactly those items. Checks SB-1..SB-5 in `bevyrrowind/docs/visual-checks.md`, scene `sb-balmora`.
- **Interactions checked**: node filters (decode and surface use the same filter); mirrored rigid parts
  (winding flipped by the cook, the mirror is in the part's inverse bind, so rest positions and the volume
  sign are right); rigid part root transforms (in the offset since feetfix); morph-bearing heads (left
  out); actor effects (actorfx2: per-character material copies match `material_name`, unchanged;
  visibility joints exist only on creatures, whose single-model bodies have no cross-part seams, so the
  step leaves them unchanged).
- **Cost** (release, 7-character lineup built in parallel on the task pool with the owner on the machine:
  indicative only): 4.5-116 ms per outfit, mean 38 ms; two armored NPCs alone: mean 21 ms. Above the
  survey's single-thread 13 ms because the builds share cores. A miss shows the outfit a frame or so later;
  a hit costs nothing. The `outfit.smooth` bake pre-warm (next) removes it for NPC outfits.
- First run (lineup of 7, `shots/smoothlive/`): 66 of 112 parts rewritten, 0-40 welded vertices per
  outfit, guard reverted 2 seams (robe outfit); `armor_{smooth,vanilla}.png` close-ups.

## Seam shading (seams, 2026-10-12)

**Owner decision (2026-10-13):** seam shading fix confirmed (SB-6); colour seams stay as authored (no
colour levelling); replacer bodies remain the route for different skin art.

Owner check CH-1b failed: seams visible on every character with B on and off. Diagnosis
(`bevyrrowind/crates/mw_actors/examples/seam_report.rs`, naked Fargoth, 20 parts; close-ups in
`shots/seams/`):

- **The seams are not vertex loops.** Only forearm ↔ wrist and knee ↔ ankle share rim vertices
  (10 and 7). The upper arm, the hands and the forearm's elbow end are **closed shells** (0 open
  edges) pushed into the next piece; neck, chest, groin, thigh and foot rims lie over or under the
  next piece with no vertex within 3 mm. The visible seam is where two surfaces cross, so the
  2 mm weld (the only normal fix so far) touched two seam types out of nine.
- **Shading:** across the seams the two sides' normals differ 9-17° where rims meet (shin, wrist:
  a bright line, gone in smooth mode since the weld averages them) and 30-95° at overlaps
  (elbow rim highlight, knee, neck, chin).
- **Colour (not fixable without art or a colour step):** every piece has its own texture
  (`ua`, `la`, `ul`, `ll`, `n`, `c`, hand `h*`, foot `uf/lf`, head `h02`), so colour continuity is
  whatever the painter matched: the knee bulb (`ll`) is lighter than the thigh (`ul`), the neck
  (`n`) darker than chest and head, the hands lighter than the wrist. No vertex colours, identical
  materials (white, same flags), so no per-material lighting differences.
- **Silhouette:** zigzag rims (thigh over the knee bulb, forearm top at the elbow) and the bulb
  itself are authored geometry; they show in bind pose in both modes.

**Fix (shading): proximity normal blending** (`blend_seam_normals`, run by `smooth_outfit` after
the guard, Smooth mode only so B still shows the authored look). Every vertex within `link`
(4 cm) of a skeleton-adjacent part's surface whose normal faces the same way moves toward the
average of its own normal and that surface's interpolated normal: fully at the surface, not at all
at 4 cm (smoothstep). Both sides do it, so they meet at one normal where they cross. This is the
normal transfer modular-character tools apply at joins (Blender Data Transfer "custom normals" with
a falloff, Maya Transfer Attributes) and the mesh-to-terrain normal blending of current engines;
Bevy and the crate ecosystem have nothing for it (Bevy only computes flat/smooth normals per mesh),
and it reuses `smooth`'s BVH. Heads (morph targets) join for shading only: their weights and
positions stay, and the new normals go back into their own bind space. Fargoth: 137 normals on 18
of 20 parts; rim-to-neighbour angle at the knee 32 → 10°, neck → chest 48 → 20°, groin 33 → 12°,
shin and wrist 13 → 0°. Cost: one closest-point query per vertex per part (parry BVH), well under
the step's 13 ms. Test `seam_normals_agree_across_seams`.

**Left (owner decision; art or a new step):**
1. **Colour seams** dominate what is left. Options, cheapest first: (a) **seam colour levelling**
   (Lempitsky & Ivanov 2007, the local colour adjustment of Waechter et al. 2014 "Let There Be
   Color!"): sample both textures along each seam once per part pair, solve a smooth per-vertex
   multiplicative tint (harmonic, like the weight band) that makes both sides match at the seam
   and fades to 1 inside, store it in the vertex colour attribute (the cook already carries it;
   `ambient_diffuse` multiplies). No art change, no runtime cost; needs CPU texture decode at
   assembly or a bake per part pair. (b) Skin-tone matching per race (one tint per texture from
   its mean colour near the seams): cruder. (c) Replacer bodies (Better Bodies, Robert's Bodies:
   skinned, one texture set), already supported as data.
2. **Silhouette:** shrink-wrap rims that lie over a neighbour within ~1 cm onto its surface (the
   Shrinkwrap-on-border-loop technique) would flatten the zigzag lips; it moves geometry, so it has
   to run before the guard and be measured by the survey. The knee bulb is the shape the art has.
