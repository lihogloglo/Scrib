# Animation study: character action system (owner, 2026-10-18)

The owner wants a snappy, general character animation system that works in many situations:
sneaking slowly behind a guard, jumping roof to roof, climbing, mantling, parkour for players who
play an assassin, fighting with a sword, sitting in a bar, all in first and third person. Few games
achieve this, even AAA ones; Elden Ring / Sekiro, Mirror's Edge and Assassin's Creed are the rare
successes people imitate. `../../specs/animation.md` ("Next-gen roadmap", "Prior art") was written from
READMEs only and is still too generic. This study reads how the working systems actually work, in
detail, then rewrites the spec. The hardest part is expected to be the clips: making animations from
any source (Morrowind KF, UAL, 100STYLE, LaFAN1, Mixamo, mocap) work inside these systems.

## Rules for every study note

- **Clean room** (superfargoth CLAUDE.md rule 3, WISHLIST "Reference policy"). Several projects were
  reverse-engineered from game executables or read the original game's data. Notes describe
  behavior and technique: mechanisms, state machines in prose, timing windows and value *ranges*,
  data shapes in our own words. **Never** code, pseudocode, decompiled identifiers, file-format
  structures, or raw tables or constants extracted from a game. MIT/Apache project code may be
  read; name its own (non-game) modules when that helps a reader find the idea.
- **Detail over summary.** Answer *how*, not *that*: what runs each frame, who owns the root
  (capsule-driven vs root motion vs warped), how input becomes an action (buffers, cancel windows,
  priorities), how transitions blend, where IK enters, what the clip must carry (contacts, events,
  warp windows, sync markers), how the camera and first-person body work.
- **Situations.** Each note says what its project does (or can't do) for the owner's situations:
  sneak behind a guard, roof-to-roof jump, ledge grab / mantle / climb, sword fight (attack, block,
  parry, dodge, hit reaction), sit on a chair at a bar, door/lever, first person and third person.
- **Failure modes** the project hit and how it fixed them (look in commit messages and docs).
- **For us:** map each idea to our crates (`sf_anim`, `sf_anim_graph`, `sf_controller`,
  `sf_interaction`, `sf_ragdoll`) and to gaps in animation.md / locomotion.md; say adopt / adapt /
  reject and why. Prefer reuse of MIT/Apache code where it fits (CLAUDE.md rule 8).
- One note per slice, `NN-<topic>.md` in this folder, ≤ ~2,500 words, sources linked with commit
  hashes. No code changes, no builds.

## Slices

| # | Note | Scope | Status |
|---|---|---|---|
| R1 | `01-faith-runner-movement.md` | faith-runner `faith_move`: controller, move set, vault/climb/wallrun/springboard, tuning, world trait, course tests | done 2026-10-18 |
| R2 | `02-faith-runner-animation.md` | faith-runner `faith_anim` driver/rig/sound, `look`/`camera`: how motion drives clips, first-person body | done 2026-10-18 |
| R3 | `03-ac1-climbing-parkour.md` | AC1-rs `climb`, `climb_table`, `docs/PARKOUR.md`, `docs/NOTES.md`: holds, move graph, jump targets, end-pose validation | done 2026-10-18 |
| R4 | `04-ac1-animation-ik.md` | AC1-rs `animation`, `move_blend`, `ik` crate, `docs/PIKVR.md`: blending, IK placement, mirroring, cloth, ragdoll, VR body | done 2026-10-18 |
| R5 | `05-sekiro-mobility-behavior.md` | sekiro-rs `docs/MOBILITY.md`, `sim/behavior`, behavior-graph loading, root motion: grapple, ledges, how a behavior graph runs | queued |
| R6 | `06-souls-combat-feel.md` | ELDEN-RING-Combat-Rewrite (all) + sekiro-rs combat (posture, deflect, hit, knockback, `COMBAT-RULES`, `HKS-*`): windows, buffering, root motion, hit-stop, lock-on | queued |
| R7 | `07-industry-traversal-fp.md` | published talks/papers: AC Unity/Syndicate parkour, Mirror's Edge (full-body first person), Dying Light 1/2, Uncharted/TLOU, BotW climbing, Spider-Man, UE GASP traversal internals, Overgrowth procedural; first vs third person body strategies | queued |
| R8 | `08-industry-combat-responsiveness.md` | talks and community frame data (not RE): souls-like windows, input buffering, animation cancelling, responsiveness vs weight, hit-stop, root motion vs capsule control, God of War 2018, For Honor, Ghost of Tsushima | queued |
| R9 | `09-clip-pipeline.md` | making any clip work: retargeting (UE IK Retargeter, Unity Humanoid), auto-annotation (contacts, sync markers, warp windows), style/timing normalisation, coverage of the move list by our sources (animation-sources.md), first-person arms; owner's local `D:\Gamedev\Animation morrowind` (UAL Standard + Source, ARP retarget blends) | queued |
| R10 | `10-mudcrab-borrow.md` | Mudcrab (Apache-2.0) whole project vs ours: what to adopt or port (converters, LOD, launcher/mods, colour pipeline, profiling, Luau) | queued |
| R11 | spec rewrite | synthesis of R1-R10 into a concrete spec (move taxonomy, action graph and chooser, data per clip, first/third person, combat timing, test courses), owner review | after R1-R10 |

Two research agents run at a time beside the build lanes (owner, 2026-10-18).
