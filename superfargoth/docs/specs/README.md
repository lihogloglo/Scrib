# Framework specs

Specs for `sf_*` systems. Same template and rules as `../../../bevyrrowind/docs/specs/README.md`,
plus: no game-specific content (both parts are GPL-3.0-or-later since 2026-10-25; the old "no GPL-derived material" rule no longer applies).

**Every spec has an "Industry comparison" section** (rule since 2026-10-12,
after the owner caught a missing basic technique; see
[industry-audit.md](industry-audit.md)). It lists what major engines (UE,
Unity, Bethesda Creation, RAGE, Decima, Frostbite, OpenMW, as relevant) do
for that system, with citations, and says for each technique whether we
match, differ (and why), or lack it (gap + priority). A spec without it is
not "agreed". Existing specs get theirs through roadmap job J9;
industry-audit.md is the cross-system starting point.

**Every spec follows the eight net-ready rules** in [net.md](net.md) (rule since 2026-10-19,
co-op N0): fixed-tick gameplay, commands as the only input, state apart from presentation, pure
steps, one owner per piece of state, no singleton player, stable ids and absolute positions,
events as data. It says how in a "Co-op" section or line, or lists its exceptions; known
violations are in net.md's audit table.

**Doc budget**: a spec over ~25 KB keeps decisions, data model and open questions, and moves its
as-built logs, measurements and superseded options word for word to `archive/<name>-history.md`
(headings kept, so old references resolve there): [ui](archive/ui-history.md),
[physics](archive/physics-history.md), [save](archive/save-history.md); action.md's predates this as [action-archive.md](action-archive.md).


The system index lives in [docs/SYSTEMS.md](../../../docs/SYSTEMS.md): find your system's row (spec sections, crates, ROADMAP and
visual-check IDs) and read only those. Pre-move index text: [index-2026-10-25.md](index-2026-10-25.md).
