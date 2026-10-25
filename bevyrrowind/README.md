# Bevyrrowind

Morrowind, rebuilt on the [superfargoth](../superfargoth) framework and
[Bevy](https://bevy.org). Not a faithful port: Morrowind if it were made in
2026. You need your own copy of Morrowind; no game data is included.

Status: milestone M0 (data loading). See
[superfargoth/docs/ARCHITECTURE.md](../superfargoth/docs/ARCHITECTURE.md).

## Try it

```sh
# Where does detection look? (Steam, GOG, retail registry, BEVYRROWIND_MORROWIND)
cargo run -p bevyrrowind -- detect

# Mount BSAs + Data Files, load every content file, print timings and conflicts
cargo run --release -p bevyrrowind -- info

# Inspect data
cargo run --release -p bevyrrowind -- find NPC_ fargoth
cargo run --release -p bevyrrowind -- record NPC_ fargoth
cargo run --release -p bevyrrowind -- record CELL "Seyda Neen, Census and Excise Office"
cargo run --release -p bevyrrowind -- ls meshes/x
cargo run --release -p bevyrrowind -- which meshes/x/ex_hlaalu_b_01.nif
```

Pass `--morrowind <path>` (install folder or its `Data Files`) to skip
detection.

```sh
# Render a cell (interior name, or exterior grid "x,y"). Right mouse to
# look, WASD/QE to move, Shift to run.
cargo run -p bevyrrowind -- view "Seyda Neen, Census and Excise Office"

# Automated visual check: wait for assets, save a PNG, exit
cargo run -p bevyrrowind -- view "Seyda Neen, Census and Excise Office" --screenshot shot.png
```

## Crates

| Crate | What it does |
|---|---|
| `mw_install` | Finds the install (env var, registry, Steam libraries), reads `Morrowind.ini` (Windows-1252). |
| `mw_vfs` | BSA archives as VFS sources; mounts `Morrowind.bsa`, ini archives, then loose `Data Files` (loose always wins). |
| `mw_records` | Loads ESM/ESP files into the record database with Morrowind merge rules (cell reference merging, deletions, topic-scoped INFO). |
| `mw_nif` | NIF → engine-neutral model: scene graph, meshes in engine space (Y up, meters), inherited properties, deduplicated materials, visibility rules. |
| `mw_world` | Cells → placements (base record resolution, reference transforms, lights). |
| `mw_assets` | Bevy `AssetLoader` for `.nif` producing a `WorldAsset`; textures resolved through the VFS (`.dds`/modern formats take the slot). |
| `apps/bevyrrowind` | The binary: inspection CLI today, the game later. |

## License

[GPL-3.0-or-later](LICENSE), with an additional permission (GPLv3 section 7)
for GPU vendor runtimes such as NVIDIA DLSS ([LICENSE-EXCEPTION.md](LICENSE-EXCEPTION.md)).
This repo may reuse GPL code (e.g. from OpenMW). The superfargoth framework is
GPL-3.0-or-later too since 2026-10-25. DLSS is an optional plugin package the
player installs separately; the game binary contains no NVIDIA code.
