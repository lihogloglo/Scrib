# Animation sources: bring-your-own libraries, licenses, import, creatures

Status: first slice built (2026-10-26): manifest, library folder, retarget
bake, slots `traverse.mantle.waist` (LAFAN1 mocap, then UAL2) and `traverse.climb.up` (UAL1); see "As built".
Licenses checked 2026-10-11
Crates: `mw_bake` (source manifest, clip selection, `GraphRules`), on top of
superfargoth `sf_anim` (importers, `anim.cook`, `anim.retarget`,
`rig.profile`), `sf_character` (rig auto-map), `sf_anim_graph` (optional
clips). Framework design: `../../../superfargoth/docs/specs/animation.md`
("Next-gen roadmap", "Optional clips"), `rig-profile.md`, `character.md`.

## Goal

Give vanilla NPCs and creatures a modern clip set without putting any
third-party animation in our repos or releases. The bake retargets
animation libraries the user has on their own machine, exactly as it
treats the Morrowind install: **bring your own (BYO)**. Every library is
optional. A slot with no source falls back to Morrowind's own KF animation,
so the game always runs on a plain install.

Non-goals: shipping any library, scraping any website, motion matching
before M4 (the pose-search database uses the same sources later).

## Principles

1. **Nothing non-redistributable in git or in a release.** Repos hold only
   our manifest (which clip from which file goes in which slot), rig-map
   overrides and our own authored data. Releases hold code.
2. **Downloads are manual.** The manifest lists, per source, the official
   page, the expected files and their SHA-256. `bevyrrowind anim-sources
   check` reports what is present. No automated downloading from
   sites whose terms forbid it (CMU's home page says "please don't crawl";
   Mixamo's FAQ forbids bulk downloading for machine learning) and none
   at all in the first version.
3. **Bake output stays local.** Cooked clips carry their source's license
   class in their provenance. Any export (mod packaging, sharing a bake
   cache) refuses clips whose class forbids redistribution of derivatives.
4. **Morrowind first.** The KF animations from the user's install are the
   baseline and the fallback for every slot.

## Sources and licenses (checked 2026-10-11)

| Source | Content | Format | License (verified where) | Local use | Redistribute derivatives? | Role |
|---|---|---|---|---|---|---|
| Morrowind KF (user's install) | all vanilla NPC and creature clips | NIF/KF | Bethesda EULA | yes | no | baseline, fallback for every slot |
| Quaternius Universal Animation Library 1 and 2 | 45 free + 120+ (Pro, paid) humanoid clips; UAL2 adds melee/armed combos, parkour, zombie locomotion | glTF/GLB, FBX | CC0 1.0 (itch.io page, OpenGameArt) | yes | yes (CC0) | primary humanoid source; framework test rigs already in `sf_character/tests/data` |
| mesh2motion | CC0 rigs and clips incl. non-human rigs (spider and snake added; others to check) | GLB/glTF | assets CC0, app MIT (repo README, `LICENSE-CC0.MD`) | yes | yes | creature reference and test rigs |
| 100STYLE (Ian Mason) | 100 locomotion styles: walk/run fwd/back/side, idles, transitions; 4M+ frames, 28-bone skeleton | BVH, 60 fps | CC BY 4.0 (ianxmason.com/100style) | yes | yes, with attribution | locomotion variety; pose-search database later |
| CMU Graphics Lab Motion Capture Database | ~2,600 takes, broad everyday and sports motion | ASF/AMC, C3D (BVH conversions exist) | "may include in commercially sold products, may not resell directly, even converted"; FAQ: "may be copied, modified, or redistributed"; "please don't crawl" (mocap.cs.cmu.edu) | yes | yes inside a product, never as a data pack | gap filler (idles, gestures, interactions) |
| Mixamo (Adobe) | large humanoid library | FBX | Adobe FAQ (2022, "limited duration technology preview"): royalty-free in projects; no reselling or redistributing raw files as asset packs; no bulk downloads for ML; terms are those in force when used | yes, downloaded with the user's own Adobe account | no (treat as non-redistributable) | optional source; manual download list only |
| LaFAN1 (Ubisoft La Forge) | 77 takes, 4.6 h: locomotion, obstacles, falls and get-ups, fighting, aiming | BVH, 30 fps | CC BY-NC-ND 4.0 (repo `license.txt`) | yes, private adaptation | **no** (ND forbids sharing adaptations) | BYO only: falls/get-ups (M4), traversal |
| Bandai Namco Research Motiondataset 1 and 2 | 1: 17 contents × 15 styles (daily, combat, dance); 2: locomotion and hand actions × 7 styles | BVH | CC BY-NC 4.0 (both datasets' `LICENSE` files) | yes | non-commercially with attribution | BYO; styled locomotion and combat |
| UE Game Animation Sample (Epic) | 500+ AAA clips | uasset | Fab/UE content license: Unreal-only | **no** | no | excluded |
| NVIDIA MotionBricks | generated motion | model | code Apache-2.0, weights NVIDIA Open Model License; Unitree G1 robot checkpoints | n/a | n/a | watch item (animation.md) |

The project is non-commercial, so the NC sources are usable for the
user's own bake. Whether the project ever ships CC0/CC BY-derived cooked
clips (allowed by their licenses, with an attribution file) is an owner
decision; until then everything is BYO for one uniform flow.

## Import profile

**Formats.** glTF/GLB (built in), BVH and FBX (`sf_anim` importer
features `bvh` and `fbx`, built 2026-10-12, animation.md "BVH and FBX
importers": our own BVH parser, FBX via `ufbx` 0.11.5, MIT OR Unlicense,
which needs a C compiler at bake time). `mw_bake` enables both features and
registers a `BvhAnimImporter` whose path-prefix overrides come from the
manifest's `unit_scale`/`up` (an `sf_anim::SourceSpace` per source
directory). CMU's native ASF/AMC is read through a BVH conversion the user
makes or downloads; an ASF/AMC importer is a later option.

**Flow per source file** (all existing or planned bake steps; nothing at
runtime):

1. `skeleton.cook` of the source skeleton (units from the manifest:
   BVH files are in centimeters, inches or arbitrary units, so each source
   declares `unit_scale` and `up` axis (and `forward` if not +Z); FBX units
   and axes come from ufbx). `SkeletonRules::detect_root_motion` picks the
   root-motion joint the importer detected (a moving `root`, else the
   hips / BVH `ROOT`) unless the source names one.
2. `rig.profile` + the humanoid auto-map (`sf_character`), saved for
   review; an override in `data/animation/rigs/<source>.ron` only if the
   auto-map fails (Mixamo `mixamorig:*`, CMU/LaFAN1/100STYLE `Hips,
   LeftUpLeg…`, Quaternius' own names are the expected cases).
3. `anim.cook` on the source skeleton with the manifest's `ClipRules`:
   frame range (mocap takes are long: split by the manifest, e.g.
   100STYLE's `Frame_Cuts.csv`), loop, root motion from the hips projected
   to the ground when the source has no root joint, contacts auto-detected.
4. `anim.retarget` onto `base_anim`, `base_anim_female`, `base_animkna`
   (beast legs keep foot contacts through the IK fix-up) and, for first
   person, `base_anim.1st`.
5. Warp windows, combat windows and footstep events added from the
   manifest (`ClipRules::events`), since mocap carries none.

**Manifest** (`data/animation/sources.ron`, GPL, ours):

```ron
sources: {
  "quaternius_ual2": (license: CC0, page: "https://quaternius.com/...", files: [("UAL2.glb", "sha256:…")], unit_scale: 1.0, up: Y),
  "100style":       (license: CC_BY_4_0, attribution: "Ian Mason, 100STYLE", files: [...], unit_scale: 0.01, up: Y),
  "lafan1":         (license: CC_BY_NC_ND_4_0, private_only: true, files: [...], unit_scale: 0.01, up: Y),
  "mixamo":         (license: MixamoTerms, private_only: true, manual: [("Sword And Shield Slash", "In Place: off, 30 fps, without skin")]),
},
slots: {
  "loco.walk_fwd": (graph: "humanoid", required: false, candidates: [
      ("quaternius_ual2", "Walk_Loop"), ("100style", "Neutral_FW.bvh", frames: (120, 480)), ("mw", "xbase_anim.kf#WalkForward")]),
  "react.getup_back": (graph: "humanoid", candidates: [("lafan1", "fallAndGetUp1_subject1.bvh", frames: (900, 1050))], fallback: drop),
}
```

The user's priority order of sources lives in their local settings, not
the manifest. `GraphRules` (animgraph.compile) picks, per slot, the first
candidate whose files are present; else the Morrowind fallback; else the
slot's `fallback` (`drop` uses the framework's optional-clip rules).
`bevyrrowind anim-sources report` prints which slot came from where.

**Framework side (built 2026-10-12, animation.md "Optional clips as
built").** A slot maps onto graph clip syntax directly: each candidate
becomes an optional clip whose `fallback` is the next candidate, ending in
the Morrowind clip (required) or `Drop`:

```ron
clips: {
  "walk_fwd": (source: "quaternius_ual2:Walk_Loop", optional: true, fallback: Clip("walk_fwd.100style")),
  "walk_fwd.100style": (source: "100style:Neutral_FW.bvh@120-480", optional: true, fallback: Clip("walk_fwd.mw")),
  "walk_fwd.mw": "xbase_anim.kf#WalkForward",
  "getup_back": (source: "lafan1:fallAndGetUp1_subject1.bvh@900-1050", optional: true, fallback: Drop),
}
```

`mw_bake`'s `GraphRules` then implements three hooks: `clip_product`
(source reference → `anim.cook`/`anim.retarget` product), `clip_files`
(the manifest's files for that source, checked and recorded by the bake,
so dropping a library into the folder recompiles exactly the graphs that
asked for it) and `provenance` (`source` = manifest id, `license` = its
license tag, `redistributable` = false for `private_only` sources and the
Morrowind KF). The compiled graph's `resolutions` table is the data of
`anim-sources report`; the export guard refuses any graph whose
`non_redistributable()` is non-empty. The `source:file@frames` reference
form above is a proposal for `mw_bake` (the framework treats references
as opaque strings).

## As built (2026-10-26)

- Manifest: `mw_graphs/data/animation/sources.ron` (next to the graphs; types `mw_graphs::sources`). Candidates are
  `(source, clip, file?, range? in seconds)`; slot cook rules `looping`, `root` (mode, default `Drive`), `vertical`,
  `fallback`. Files carry `sha256` (recorded, not verified: no SHA-256 crate in the tree; `sha2` in the crate batch).
- Graph: the builder chains a slot's candidates as optional clips `slot` → `slot.alt<i>` → fallback (reference
  `source:clip`) and lists the slot in `slot_clips`; no move plays it yet. User-side source priority: not built.
- Folder: `BEVYRROWIND_ANIM_SOURCES`, else `<repo>/animation packs` (compile-time default); `<dir>/<file>` or
  `<dir>/<source>/<file>`. The bake names files `animsrc/<source>/<file>` (`VfsInputs::with_anim_sources`, stat-keyed).
- `mw_bake`: phase 1 cooks a present file's skeleton and lists its clips (`mw.anim.source_clips`); source rig =
  `sf_character::auto_map` + auto-align (UAL1: 52 bones, clean); targets use the humanoid map, legs IK. `GraphRules` v2:
  `source:clip` → `anim.retarget` of an `anim.cook` on the library skeleton, `clip_files` = the library file,
  `provenance` = source id, SPDX tag, redistributable. Missing file → drop; present file without the clip → drop
  (folded into the rules version). `base_anim`, `base_anim_female`, `base_animkna` share the path (no beast-leg fix).
- Report: `bevyrrowind bake` prints a line per slot (source, or fallback and why); `bevyrrowind anim-sources` prints the
  folder, files present/missing and per-slot choice. Tests: `mw_bake/tests/anim_sources_real_install.rs`.
- UAL1 `ClimbLedge_RM` is a 0.63 s top-out: root straight up 2.47 m (2.94 m on `base_anim`), no forward travel.
- LAFAN1 (2026-10-26): BVH importer in `anim_importers()` (per-source `unit_scale`/`up` from the manifest as path
  overrides); source flag `root_from_pelvis` → `SourceRef::retarget_root` → `AnimRules::retarget_root`: cooked without
  extraction, root motion generated on the target hips (framework animation.md "Bake steps"). `lafan1` is
  CC-BY-NC-ND-4.0, `private_only` (graph provenance non-redistributable). `traverse.mantle.waist` candidates:
  `obstacles3_subject4` 123.83–127.17 s, then UAL2 `ClimbUp_1m_RM`. On `base_anim` (measured): 3.34 s, root rises
  1.30 m, forward 2.50 m, side 0.57 m; frame 0 hips over the root (2.6 cm), thighs face 2.3°, toes 5°/−9°; but the
  root yaw turns 0 → 84° by the obstacle and ends at 120° (the actor turns side-on to knee over it, then turns away).
  Tests: `lafan1_mantle_starts_over_its_root_facing_forward`, `measure_mantle_waist_lafan1` (ignored).

## Morrowind creature mapping notes

Creatures keep their own skeletons and KF clips; each gets a rig profile
(auto-detected, overrides in `data/animation/rigs/`). CREA flags pick the
mover mode: `Walks` → `Ground`, `Flies` → `Fly`, `Swims` → `Swim`;
`Biped` creatures with Bip01 humanoid naming may also get a humanoid map
(and so retargeted humanoid clips where their proportions allow). The
original creature models were built with 3ds Max Biped, so many quadrupeds
likely carry Bip01-style names with extra leg joints: verify every
skeleton with `bevyrrowind nif` before writing hints.

| Template | Creatures (verify IDs and skeletons in data) | Notes |
|---|---|---|
| Biped | guar and kagouti (two-legged: guar on digitigrade hind legs, kagouti on its forelimbs; checked 2026-10-12, actors-adapter.md "Rig maps as built"), scamp, dremora, golden saint, ogrim, atronachs, ash ghoul/slave/zombie, corprus stalker, lame corprus, skeletons, bonewalker, lich, daedroth, hunger, clannfear, winged twilight (+ wings), spriggan (Bloodmoon) | humanoid profile when Bip01-like; else biped template without the hub |
| Quadruped | nix-hound, alit, rat, durzog (Tribunal), wolves and bears (Bloodmoon) | front/hind girdles, `SpineBend`, digitigrade checks per leg |
| Multiped | shalk, mudcrab, scrib, centurion spider, kwama warrior | 6–8 support limbs; claws as non-support arms |
| Flyer | cliff racer (wings + tail) | `Fly`; tail via `ChainIk`/springs |
| Floater | netch (betty, bull) | `Fly { hover }`; tentacles as `SpringBones` |
| Swimmer | slaughterfish, dreugh | `Swim` |
| Serpent | kwama forager | spine-only, no limbs |
| Rolling / no feet | centurion sphere, steam centurion (check) | root motion + look-at only |

Modern creature sources are thin (no CC0 quadruped mocap found; MANN's dog
data is research-only): creatures stay on their KF clips, upgraded by
foot placement, warping, look-at, spine bend and springs. Retargeting a
CC0 quadruped library onto guar-like rigs is a later option through the
chain names of rig-profile.md.

## Verification

- `bevyrrowind anim-sources check` on a folder with Quaternius UAL1 free,
  100STYLE and nothing else: reports present/missing per source and per
  slot; the bake succeeds; missing slots resolve to Morrowind clips.
- With no sources at all, the humanoid graph compiles and plays (pure KF).
- Export guard: packaging a test mod that includes a LaFAN1-derived clip
  fails with the clip's provenance.
- Visual: Quaternius walk retargeted onto `base_anim` and `base_animkna`
  next to the source (ROADMAP M3 rig-map item); a guar on stairs with all
  four feet placed.

## Industry comparison

J9 (superfargoth `industry-audit.md`), 2026-10-12, docs only (`.sessions/j9rest.md`).
Retargeting external libraries onto a game rig is standard: UE IK Rig +
IK Retargeter (chains, root and pelvis handling, batch retarget), Unity
Humanoid avatars (muscle space), Mixamo auto-rigging. Ours: bake-time
retargeting through the rig profile (superfargoth `character.md`,
`rig-profile.md`, compared there) with license classes carried in
provenance (**match**; the license gating is ours, engines leave it to the
user). No gap.

## Open questions

1. Ship CC0/CC BY-derived cooked clips in releases (with attributions), or
   keep everything BYO? Owner.
2. Mixamo: allowed as a manual BYO source at all, given the 2022 FAQ still
   calls the service a technology preview? Owner (current default: yes,
   manual, private only).
3. Exact Morrowind creature skeletons and root-motion joints (with
   actors-adapter.md's open question on `Bip01` vs `Bip01 NonAccum`).
