# Crossreview config (Scrib)

Used by the `crossreview` Claude Code skill (its own repo, installed per machine). One target per run.

## Resolving a target

- A target is a row of `docs/SYSTEMS.md` (match the System column loosely), a crate name, or a path.
- `code`: each crate in the row's Crates column. `sf_*` live in `superfargoth/crates/`, `mw_*` in
  `bevyrrowind/crates/`. `sf_x(_bevy)` or `sf_x (+_bevy)` means both `sf_x` and `sf_x_bevy`. Notes in brackets
  (e.g. `mw_rpg_bevy (inventory_ui)`) mean only that module. Skip `workspace-hack` and planned crates.
- `docs`: the row's main spec(s) and adapter spec(s). Put the row's key section names in `context`.
- `decisions`: `superfargoth/docs/DECISIONS.md` (147 KB: agents grep it) and `superfargoth/docs/DECISIONS-INDEX.md`;
  add `superfargoth/docs/specs/industry-audit.md` (grep for the system).
- `context`: Bevy 0.20 (vendored patches in `superfargoth/vendor/`); frame target High 1440p + DLSS Quality,
  RTX 4060 laptop, <= 11 ms median in exteriors; the row's ROADMAP IDs; the row's key sections.
- Over ~15k lines of Rust: split framework (`sf_*`) and game (`mw_*`) crates into two runs.

## Output and follow-up

- Report: `docs/reviews/<target-slug>-<YYYY-MM-DD>.md` (tracked).
- After the owner reads it, and only with their go: industry gaps go in `superfargoth/docs/specs/industry-audit.md`,
  bugs and leftovers in the ROADMAP backlog or `bevyrrowind/docs/visual-checks.md` ("log bugs, don't chase them").
  Fixes are separate slices the owner picks.
- Never run cargo during a review (other sessions build here; check `.sessions/` if unsure).
