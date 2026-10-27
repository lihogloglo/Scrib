<p align="center"><img src="logo.png" alt="Scrib" width="256"></p>

# Scrib

A modern open-world action RPG engine on [Bevy](https://bevy.org), and Morrowind running on it.

| Part | What it is | Start here |
|---|---|---|
| [`superfargoth/`](superfargoth) | The game-agnostic framework (`sf_*` crates): streaming, records and mods, rendering, weather, animation, physics, UI, saves | [ARCHITECTURE](superfargoth/docs/ARCHITECTURE.md) |
| [`bevyrrowind/`](bevyrrowind) | The Morrowind content pack and game binary (`mw_*` crates, `apps/bevyrrowind`) | [ROADMAP](bevyrrowind/docs/ROADMAP.md) |

Morrowind's data is not included: point the game at your own install.

## License

[GPL-3.0-or-later](LICENSE), with an additional permission (GPLv3 section 7) for GPU vendor runtimes such as
NVIDIA DLSS ([LICENSE-EXCEPTION.md](LICENSE-EXCEPTION.md)). The game binary contains no NVIDIA code: DLSS is an
optional plugin package (`superfargoth/plugins/sf_dlss`) installed separately. Vendored crates under
`superfargoth/vendor/` keep their own licences. Bundled third-party assets: the Quaternius Universal Animation
Library 1 and 2 (CC0 1.0, by Quaternius; Git LFS) in `bevyrrowind/assets/animation/quaternius/` (`PROVENANCE.md`).

Until 2026-10-25 the framework and the game were two repositories (`superfargoth`, MIT OR Apache-2.0 then, and
`bevyrrowind`); both histories are merged here.
