# Lane roadmaps

One file per gameplay system (owner 2026-11-25: "before jumping in, we need proper roadmaps for each"). A lane
roadmap is the ordered plan from today's state to the finished system: what the player gets, in which order, sized
in slices one agent finishes. It sits between the short `bevyrrowind/docs/ROADMAP.md` line (status + pointer) and
the spec (design and as-built detail). Research behind each one: `docs/research/gameplay-2026-11-25/<lane>.md`.

**Status 2026-11-25:** research done for every lane (anim + `anim-diagnosis.md`, combat, ai, nav, dialogue-quests,
crime, interaction, travel) plus `ecswind.md` (the owner's earlier sim, ideas only). Written: [ai.md](ai.md),
[dialogue-quests.md](dialogue-quests.md), [travel.md](travel.md), [interaction.md](interaction.md), [crime.md](crime.md), [anim.md](anim.md) (2026-11-25), [combat.md](combat.md), [nav.md](nav.md) (2026-11-26). Next: order.md.

| Lane | File | Specs it drives |
|---|---|---|
| Animation for gameplay | [anim.md](anim.md) | animation-plan, locomotion, action, animation-sources |
| Combat | [combat.md](combat.md) | combat, combat-adapter, ai.md "Combat AI" |
| NPC AI | [ai.md](ai.md) | ai, ai-adapter, world-reactions |
| Navigation | [nav.md](nav.md) | nav, orca |
| Dialogue and quests | [dialogue-quests.md](dialogue-quests.md) | rpg, journal-ui, rpg-adapter, scripting-adapter |
| Crime and stealth | [crime.md](crime.md) | crime, crime-adapter, world-reactions |
| Interaction | [interaction.md](interaction.md) | interaction, interaction-adapter, world-reactions |
| Travel | [travel.md](travel.md) | services (travel), interaction (c), travel (new) |
| Cross-lane order | [order.md](order.md) | all of the above |

## Format (every lane file, ~8-14 KB)

1. **Player-facing goal**: what the player can do when the lane is done, in plain words (modern preset), and what
   the classic option keeps.
2. **Where we are**: built / verified by the owner / built but unverified / not started (with ROADMAP and
   visual-check IDs). Facts only, from the wiki, spec and ledger.
3. **Decisions**: the design choices this roadmap takes, each with its source (industry standard, Bevy, crate,
   research note section), including where it departs from the current spec or from Morrowind and why.
4. **Stages**: groups of slices that each end in something the player can use. Per slice: ID, one deliverable,
   crate(s) (`sf_*` framework vs `mw_*` adapter), depends on, verify (headless test, log line, or owner check with a
   visual-check ID prefix). One deliverable plus its test per slice; research, spec and build are separate slices.
5. **Spec changes**: which spec sections must be written or rewritten before the build slices (as spec slices).
6. **Dependencies on other lanes** (by slice ID) and what this lane provides to them.
7. **Owner questions**: only design or player-facing ones, each with a recommended answer. Performance, formats,
   budgets and defaults are decided from the industry standard in section 3, never asked.

Slice IDs keep the existing spec prefixes (AI8, NV9, IN1, LW3, C2c...); new slices take the lane's prefix and the
next free number. A slice marked done here is also marked in its spec; the ROADMAP keeps one line per lane.
