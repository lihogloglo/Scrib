# superfargoth

An open-world action-RPG framework on [Bevy](https://bevy.org). Game-agnostic:
content packs (such as [Bevyrrowind](../bevyrrowind), Morrowind on
superfargoth) plug in through adapters, and every piece of content, including
a base game, is a mod.

Status: early foundation work. See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
for the vision, design, and milestones.

## Crates

| Crate | What it does |
|---|---|
| `sf_vfs` | Layered virtual filesystem: loose folders, archives (via adapter sources), in-memory files. Last mounted wins, provenance per file, modern formats can take a legacy file's slot (`foo.glb` over `foo.nif`). Optional Bevy `AssetSource` (`bevy` feature). |
| `sf_records` | Record database: typed records per kind, define/patch/delete from ordered sources, full provenance, dependency-aware conflict report. |
| `sf_mods` | `mod.toml` manifests and load-order resolution (dependencies, version requirements, ordering hints, cycle detection) that stays as close to the user's profile as possible. |

## Rules

- **No game-specific code.** If it names Morrowind, it belongs in an adapter repo.
- **No GPL-derived code.** Nothing copied or translated from OpenMW, MWSE,
  `nif.xml`, or other GPL sources. Specs come from public documentation,
  papers, and observation.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at
your option.
