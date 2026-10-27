# Quaternius Universal Animation Library 1 + 2 (full)

| File | What | License |
|---|---|---|
| `UAL1_RM.glb` | Universal Animation Library, 120 clips, root motion baked into every clip | CC0 1.0 |
| `UAL2_RM.glb` | Universal Animation Library 2, 134 clips, root motion baked into every clip | CC0 1.0 |
| `License.txt`, `README.txt` | The packs' own files, unchanged | CC0 1.0 |

- Author: Quaternius (https://quaternius.com, https://www.patreon.com/quaternius). Store pages:
  https://quaternius.itch.io/universal-animation-library and https://quaternius.itch.io/universal-animation-library-2.
- Downloaded 2026-10-27 from the CC0 mirror https://github.com/DyingStar-game/DyingStar
  (`assets/Universal Animation Library/`). Unmodified. Tracked with Git LFS (root `.gitattributes`).
- Only the files the bake reads are here: the root-motion (`_RM`) exports. The plain exports (`UAL1.glb`, `UAL2.glb`)
  are the same clips with the root bone held still; clips without root travel are identical in both, and every slot
  that travels wants the `_RM` take (the cook extracts root motion from the root bone). No `.blend`, no Unity `.fbx`.
- The bake finds them through `bevyrrowind/crates/mw_graphs/data/animation/sources.ron` (sources `quaternius_ual1`,
  `quaternius_ual2`); a copy in the user's library folder (`BEVYRROWIND_ANIM_SOURCES`) takes precedence.
