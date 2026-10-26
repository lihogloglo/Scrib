# Godotwind borrow list (for a dedicated pass)

Owner 2026-10-26: Godotwind (the owner's earlier Morrowind remake, Godot 4, GDScript + C#, MIT) has logic worth taking.
This file is the index for **one dedicated borrow pass**: each entry points at the exact Godotwind code and at our
counterpart. Not part of normal slices; the pass picks entries, ports them as their own slices, and ticks them here.

- Source: `D:\Gamedev\Godotwind\godotwind`, commit `a1654b6` (2026-07-07). The repo has a foreign owner SID: read git
  with `git -c safe.directory='*' ...`.
- Detail tables (one row per candidate, with value and effort): [audit A](godotwind-audit-2026-10-26-a.md) (water,
  character, animation, deformation, interaction) and [audit B](godotwind-audit-2026-10-26-b.md) (world, records,
  NIF, dialogue, UI, console, weather).
- Rules for the pass: take the logic, edge cases, Morrowind data quirks and test ideas; skip Godot node structure and
  Godot workarounds. Godotwind's tuned numbers are starting points to measure in Bevy, never a design (its own audit found
  several wrongly imported from OpenMW). Before porting, check our code and spec: audit B did not read our spec
  summaries, so some of its rows may already exist (door arrival gate, fade hold). Cite the Godotwind file in the module
  docs (MIT; add a `THIRD_PARTY_NOTICES.md` entry if code lands in `superfargoth/`).

## 1. Shore waves: FFT ocean to analytic waves at the shore (owner: "worked well")

What it does: open sea is the FFT ocean; near the coast the FFT displacement fades out and analytic trochoidal shore
waves take over (Outerra's approach), travelling perpendicular to the coastline, phase driven by distance to shore so
they arrive at the beach, with a breaker envelope, a run-up/swash term and along-shore modulation so crests aren't
uniform. Normals get the matching slope; choppiness feeds foam/SSS.

| Part | Godotwind code |
|---|---|
| Shore mask bake: land/water mask, distance by Jump Flooding, RGBA8 = shore factor, gradient dir XZ, water-side distance | `src/core/water/shore_mask_generator.gd` (`generate_from_terrain` :44, `_jfa_pass` :172; layout doc lines 1-12, :105) |
| Vertex: FFT × shore factor, then add shore offset | `src/core/water/shaders/ocean_fft.gdshader` ~45-70 (same pattern in `ocean_fft_projected.gdshader`, `ocean_fft_opaque.gdshader`) |
| Shore wave math | `src/core/water/shaders/ocean_shore_common.gdshaderinc`: envelopes `shore_breaker_envelope` :38, `shore_runup_envelope` :45, `shore_swash_curve` :50, profile `shore_skewed_sine` :57 / `shore_crest_shape` :64 / `shore_profile_slope` :74 / `shore_forward_push` :84, `shore_along_modulation` :89, direction from the mask with gradient fallback `shore_direction_from_mask` :97, phase :118, displacement `water_surface_shore_offset` :122, slope `water_surface_shore_slope` :148 |
| Fragment: shore slope into normals, shallow colour blend, edge foam | `src/core/water/shaders/ocean_fft_common.gdshaderinc` ~428-461, ~665 |

Our counterpart: `superfargoth/docs/specs/water.md` "Shoreline and depth colour" already bakes a signed shore distance
(`water.shore`, exact EDT, plus openness) and attenuates the FFT near shore, with a shoreward foam phase. **Missing: the
analytic shore wave layer** (displacement + slope from distance and gradient). Our baked distance is a better input
than Godotwind's runtime JFA mask; the gradient comes from it. Feeds the Phase 4 water polish entry ("shore swash +
gradual wavelets", WA-1 notes). Effort M: one shader function set + spec section + a visual check at a beach
(Seyda Neen shore).

## 2. Other entries (detail in the audits)

| # | Topic | Godotwind pointer | Our side | Audit |
|---|---|---|---|---|
| 2.1 | Door travel ready = data + visual + transition; refresh desired cells after any teleport; 9 regression tests | `src/core/world/transition/`, its streaming audit in `docs/` | ROADMAP 1.1, physics fix plan arrival gate | B §1 |
| 2.2 | NIF out-of-sync self-test (`\0NiS`), their 64 failing models list, particle-only NIFs as a class | `src/core/nif/`, failure list in `docs/` | `mw_nif`, collision census (`mw_bake`) | B §2 |
| 2.3 | Perf method: standing vs moving cost, worst single work item, instances-per-draw-group census | `EXPERIMENTS.md`, `reports/` | ROADMAP 1.2, 1.9, `perf.md` | B §1 |
| 2.4 | Lazy per-cell ref expansion, first-playable manifest | `src/core/world/`, `src/core/esm/` | ROADMAP 1.2 loading | B §1-2 |
| 2.5 | Book/scroll markup tag checklist; journal test for alternate-ending quests | `src/core/ui/`, `src/core/dialogue/morrowind/` | ROADMAP 2.3, 2.4 | B §3 |
| 2.6 | Interaction edge rules: modal input gate, tap/hold, carry blocks doors/dialogue, carried items across cell unload, grab/release round trip; carry vibration causes (negative list) | `src/core/interaction/`, `docs/systems/interaction_system.md` | ROADMAP 2.1, 3.3 | A §5 |
| 2.7 | Water optics authored by visibility distance; world-anchored underwater motes (4 m grid); splash/wake emitter recipe | `src/core/water/water_optical_profile.gd`, `underwater_particulates.gd`, `water_interaction_sim.gd`, `water_interactor.gd` | ROADMAP 3.6, 3.7 | A §1 |
| 2.8 | Terrain softness table by LTEX texture name (`snow_grass`/`snow_rock` are grass/rock: rule order matters) + footprint stamp-and-recover; also feeds footsteps and wetness | `src/core/deformation/` | missing (Phase 4, next to SU1/SU2) | A §4 |
