# Specs

One file per major system, written **before** the system is built, and
kept current while it is. Framework systems (`sf_*`) get their spec in
`superfargoth/docs/specs/` instead, so it stays adapter-free (both parts are GPL-3.0-or-later since 2026-10-25).

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


The system index lives in [docs/SYSTEMS.md](../../../docs/SYSTEMS.md): find your system's row (spec sections, adapter, crates, ROADMAP and
visual-check IDs) and read only those. Pre-move index text: [index-2026-10-25.md](index-2026-10-25.md).
