# Setup: first-run setup screen and bring-your-own downloads

Status: draft (owner idea 2026-10-26), design only; nothing built. Build after the Phase 3 animation lanes
(ROADMAP "Setup screen"). Related: [animation-sources](animation-sources.md) (BYO clip manifest), mods.md
("Mods directory and profiles"), bake.md, `superfargoth/plugins/sf_dlss` (DLSS ships as a separate plugin).

## Goal

One place where a player makes the game ready, mostly in one click:

1. point at their Morrowind install (and optionally an OpenMW/MO2 mod list to import);
2. fetch the optional extras we can't ship (DLSS runtime, animation libraries such as LaFAN1, Quaternius packs);
3. run the bakes (animation retargeting, navmesh, distant land, other `bevyrrowind bake` products) with progress;
4. come back later ("check for new packs", "rebake") from Options.

Non-goals: hosting third-party files, scraping sites whose terms forbid it, an account system, auto-updating the game.

## Industry practice

| Reference | What it does | What we take |
|---|---|---|
| OpenMW launcher + installation wizard | finds or installs the Morrowind data, imports `Morrowind.ini`, manages content files and data paths | install detection and ini import (we already have `mw_install`), content list UI |
| Wabbajack | installs whole mod lists from a manifest of sources + hashes; downloads directly where allowed, opens the page and waits for the user's manual download where not (non-premium Nexus), then verifies by hash | **the download model**: a manifest of sources with hashes, two download kinds, watch the Downloads folder |
| Mod Organizer 2 | profiles, virtual file system over a mods folder | mod list import format (`modlist.txt`, `plugins.txt`); our VFS is `sf_vfs` |
| Steam / console first run | shader and cache builds with a progress screen before the menu | bake progress screen, resumable |

## Design (draft)

- **Inside the game binary, not a separate launcher.** A setup screen built with the UI kit (`sf_ui`), shown before the
  main menu on first run (or when a required step is missing) and reachable from Options. One binary to ship, one
  settings file, same input handling. A thin external launcher stays possible later if a pre-GPU step ever needs it.
- **Steps** are data: each has a check (done / missing / outdated), an action, and a required flag. Required: the
  Morrowind install. Optional: everything else; the game always runs on a plain install (animation slots fall back to
  Morrowind's KF clips, DLSS falls back to TAA).
- **Sources manifest** (extends `mw_graphs/data/animation/sources.ron` into a general one): per source its licence,
  official page, expected files, SHA-256, size, and a download kind:
  - `Direct(url)`: the licence allows fetching the original file (e.g. GitHub releases, Zenodo). One click: download,
    verify hash, unpack into the library folder.
  - `Guided(page)`: the site needs the user's own click (itch.io pay-what-you-want pages, Mixamo with an Adobe account,
    NVIDIA's DLSS terms). The screen opens the page, watches the Downloads folder, and imports the file when one with
    the right hash appears.
  - `Manual`: the user points at a file or folder they already have.
- **Licences are shown before every download**, with "redistributable" or "bring your own" labels. Nothing we bake from
  a non-redistributable source ever leaves the player's machine (animation-sources.md principle 3).
- **Bakes** run as one queue with per-step progress, cancel and resume (the bake cache already makes reruns cheap).
- **Mod list import:** `Morrowind.ini` (built in `mw_install`), `openmw.cfg` content and data lines, MO2 profile
  (`modlist.txt` / `plugins.txt`) mapped onto our mods directory and profiles (mods.md).
- **Framework split:** the step runner, download manager (direct / guided / manual, hash check, folder watch) and the
  screen are game-agnostic (`sf_*`); Morrowind's steps, sources and texts are the adapter's.

## Open questions

1. DLSS: which NVIDIA files may a player fetch directly, and from where (NGX runtime DLL vs SDK)? Today `sdk/` is never
   redistributed and DLSS is a separate plugin package.
2. Direct downloads from GitHub for LaFAN1: allowed (CC BY-NC-ND permits sharing unmodified copies); confirm per source.
3. Where the library folder lives for players (today `BEVYRROWIND_ANIM_SOURCES`, else `<repo>/animation packs`).
4. Do we need a pre-GPU fallback (a player whose GPU fails to start the renderer can't reach an in-game screen)?
