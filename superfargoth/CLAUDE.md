# superfargoth

Game-agnostic open-world ARPG framework on Bevy; its first content pack is Bevyrrowind (`../bevyrrowind`, same
repo and Cargo workspace), which drives the roadmap. **All working rules are in `../CONTRIBUTING.md`** (the one
source of truth: reading order, building, parallel sessions, tests, docs). This file only adds framework-specific
reference.

- **Litmus test for anything here:** could another game plug in with only a new adapter? Nothing in this folder
  names Morrowind (or any game's) formats, IDs, units or conventions, and nothing depends on `bevyrrowind/`.
- **Crate licences:** framework crates must be GPL-3.0-compatible: MIT, Apache-2.0, BSD, zlib, GPL-3.0, LGPL;
  MPL-2.0 only for unmodified dependencies (owner 2026-10-11); never GPL-2.0-only or AGPL (`../deny.toml`,
  `README.md` "Dependency policy").
- **Where things are:** design docs `docs/ARCHITECTURE.md` (structural work only), `docs/specs/` (by section),
  `docs/build-perf.md` (build settings and gates), `docs/WISHLIST.md` (feature targets, reference policy);
  vendored Bevy patches `vendor/` (`vendor/README.md`); the DLSS plugin `plugins/sf_dlss` (its own workspace).
- Commands (test, clippy, deny, hakari, adding a crate): `../CONTRIBUTING.md` "Building".
