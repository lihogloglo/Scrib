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
- **Ported code carries provenance.** Code from other GPL-3.0-compatible
  projects is welcome with its source, commit and licence recorded in
  [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
- **No proprietary SDK in the binary.** NVIDIA DLSS is an optional,
  separately shipped plugin (`plugins/`).

## License

[GPL-3.0-or-later](LICENSE), with an additional permission (GPLv3 section 7)
for linking GPU vendor runtimes such as NVIDIA DLSS: see
[LICENSE-EXCEPTION.md](LICENSE-EXCEPTION.md). Vendored crates in `vendor/`
keep their own licences (Bevy patches stay MIT OR Apache-2.0 for upstream).
Before 2026-10-25 the framework was MIT OR Apache-2.0; copies taken before
then keep those terms.

## Dependency policy

The repo root's `deny.toml` holds the dependency rules for the whole workspace (framework and
game); check them from the repo root with
[cargo-deny](https://github.com/EmbarkStudios/cargo-deny) (`cargo install cargo-deny --locked`):

```sh
cargo deny check
```

- **Licences:** GPL-3.0-compatible only: MIT, Apache-2.0, BSD-2/3-Clause, Zlib, ISC, Unicode-3.0,
  MIT-0, CC0-1.0, GPL-3.0, GPL-2.0-or-later, LGPL-2.1-or-later/3.0. MPL-2.0 only for the named,
  unmodified crates.io crates behind `bevy_flair` (owner decision 2026-10-11) and, in the game,
  `dirs` (option-ext) and the MP3 decoder (Symphonia). Never GPL-2.0-only or AGPL.
- **Advisories:** RustSec vulnerabilities, unsound and unmaintained crates fail; yanked ones warn.
- **Bans:** duplicate crate versions (Bevy's above all) warn; wildcard versions fail.
- **Sources:** crates.io, plus the game's `tes3` git dependency (listed in `allow-git`).

There is no CI yet: run it before adding or updating a dependency. A failure is fixed (upgrade,
swap, drop) or raised with the owner, never silenced in `deny.toml`.
