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

## Dependency policy

`deny.toml` holds the dependency rules; check them with
[cargo-deny](https://github.com/EmbarkStudios/cargo-deny) (`cargo install cargo-deny --locked`):

```sh
cargo deny check
```

- **Licences:** MIT, Apache-2.0, BSD-2/3-Clause, Zlib, ISC, Unicode-3.0, MIT-0, CC0-1.0.
  MPL-2.0 only for the named, unmodified crates.io crates behind `bevy_flair` (owner decision
  2026-10-11). Never GPL, LGPL or AGPL; a dual-licensed crate passes through its permissive arm.
- **Advisories:** RustSec vulnerabilities, unsound and unmaintained crates fail; yanked ones warn.
- **Bans:** duplicate crate versions (Bevy's above all) warn; wildcard versions fail.
- **Sources:** crates.io only.

There is no CI yet: run it before adding or updating a dependency. A failure is fixed (upgrade,
swap, drop) or raised with the owner, never silenced in `deny.toml`.
