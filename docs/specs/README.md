# Specs

One file per major system, written **before** the system is built, and
kept current while it is. Framework systems (`sf_*`) get their spec in
`superfargoth/docs/specs/` instead, so it stays MIT and adapter-free.

A spec is short (one to three pages). Template:

```markdown
# <System>

Status: draft | agreed | implemented (date)
Crates: sf_x, mw_y

## Goal
What it must do, in a few sentences. Non-goals.

## Canonical technique
The industry-standard approach we follow (name it, link papers/docs),
and where we deliberately deviate.

## Industry comparison
What the major engines do for this system (UE, Unity, and the open-world
engines that fit: Bethesda Creation, RAGE, Decima, Frostbite, Anvil,
OpenMW), with citations, and per technique: we match / we differ (why) /
we lack it (gap, priority). Check every scaling cost: what grows with
entity count, view distance, or views (cameras, shadow cascades).
Required since 2026-10-12 (superfargoth `docs/specs/industry-audit.md`).

## Data model
Durable representation first: components, records, assets, save data.

## Interfaces
Public API / traits / plugin boundaries; what adapters provide.

## Verification
How we know it works: tests, the scene/command to run and look at,
benchmark numbers.

## Open questions
```

Index:

- [nif-runtime.md](nif-runtime.md): NIF controllers, particles, billboards, model lights.
- [bake-adapter.md](bake-adapter.md): NIF importer for the bake, VFS/record inputs, cooked models in the viewer.
- [world-adapter.md](world-adapter.md): worldspaces, door links, `WorldSource` and `TerrainSource` for Morrowind (M2).
- [actors-adapter.md](actors-adapter.md): skeletons, KF clips, text keys, body parts → animation/character (draft, M3).
- [animation-sources.md](animation-sources.md): bring-your-own animation libraries (license table, manual download list, import and retarget flow, slot manifest with Morrowind fallback), creature templates (draft, M3).
- [ui-adapter.md](ui-adapter.md): classic theme (menu art, fonts → TrueType), book markup → rich text, topic links, MWScript MessageBox, OpenMW-Lua UI compatibility (draft, M5).
- [weather-adapter.md](weather-adapter.md): REGN → climates, `Morrowind.ini` `[Weather*]` → presets, ash/blight storms (radial from Red Mountain), blizzards, MWScript weather, sounds, interior fog (draft, M6).
- [physics-adapter.md](physics-adapter.md): item classes and weight → mass, `RootCollisionNode` item shapes, script hooks, ragdoll policy per creature, death/knockdown/get-up clips, robe and skirt cloth, test scenes PH-1..PH-5 (draft, M4).
