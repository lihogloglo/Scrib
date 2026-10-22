# Borrow audit, 2026-10: Mudcrab delta, bevy_atmos, Clockwork comparison

Research note, 2026-10-22. Nothing built or run. Sources: Mudcrab
`origin/main` `805777d` vs the `3129b57` audit ([animation/10-mudcrab-borrow.md](animation/10-mudcrab-borrow.md)),
bevy_atmos `a435d89` (v0.2.0), and OpenMW `clockwork-alpha` **behaviour only** (the branch's settings docs under
`docs/source/reference/modding/settings/`, read through the GitLab raw URLs; no source files, same method as
[shadows/README.md](shadows/README.md) §2). Clean-room rules hold: techniques are described, not copied.

## Verdict

**Mudcrab changed licence on 2026-10-21 (commit `88b25a2`, MIT/Apache to GPL-3.0-or-later), so from now on nothing
of theirs can go into superfargoth as code**; R10's "both projects are MIT/Apache, contribution is licence-free"
and its "port" items no longer hold for anything after that commit. 659 commits landed since `3129b57`, mostly
converter coverage (Skyrim-specific), a settings/grading UI, merged-terrain submission for Metal, streaming
admission and diagnostics. It is still Bevy 0.19, still has no weather, wet surfaces or animation, and offers no
0.20 migration help. Only *ideas* for diagnostics and streaming control are worth taking. **bevy_atmos**
(MIT OR Apache-2.0, Bevy 0.19, one author, v0.2.0 on 2026-10-21) is a young sky/cloud/weather/water plugin. We
are ahead on almost every render system, but it holds three licence-clean ideas: a compact **wet-surface and puddle
shader** (what Phase 4 needs), a **live look editor with hold-to-compare**, and small tricks (AO on direct light,
cloud cube into reflections). Clockwork's wet look is a global wetness scalar plus a cheap sky-reflecting puddle,
the "cheap" bar the owner asked for; our weather spec designs a heavier version (clipmap, persistent grid, SSR)
that we should **cut down for the first slice** (last section).

## Licence facts that change plans

- **Mudcrab**: commit `88b25a2` removed `LICENSE-MIT/-APACHE`, added GPLv3, set every crate to `GPL-3.0-or-later`.
  The converter also vendors a GPL NIF parser. Treat all Mudcrab code as GPL: portable into **bevyrrowind**,
  clean-room re-implement for **superfargoth**. *Owner question:* do we rely on the pre-relicense snapshot? I say no.
- **bevy_atmos**: MIT OR Apache-2.0, only `bevy` and `indexmap` as dependencies; portable into either repo with
  attribution. Bevy 0.19.1: its render-graph hooks need a 0.20 port, and its fog pass finds a Bevy system by name.
- **Clockwork**: GPL, docs only. Few millisecond figures; wet/rain internals undocumented (inference is marked).

## Candidate table

Costs: bevy_atmos's are theirs (720p, RX 7800 XT); Clockwork's theirs (RTX 4090, resolution as stated); ours from
our specs (1440p, 4060 Laptop). "Est." = my scaling, unmeasured. Effort S <0.5 d, M 1-2 d, L 3+ d.

| # | Source | What it does | Licence / fit | How Clockwork does it | Cost | Effort | ROADMAP | Rec. |
|---|---|---|---|---|---|---|---|---|
| 1 | bevy_atmos `wet.wgsl` | Wet material: soaked albedo x0.55, roughness to 0.18, puddles from domain-warped 4-octave gradient noise on up-facing pixels (`n > 0.65`, soft edge by `fwidth`), puddle roughness 0.03, flat normal, F0 up | MIT/Apache, 109 lines; our own `WorldMaterial` ext in weather.md | `surface weather` (0-2, default 1): wets outdoor surfaces in rain, darkened reflective ground, puddles, streaks on walls; interiors untouched; zero "compiles nothing" | ALU only; noise is ~6 gradient-noise calls per pixel (use a tiled texture instead). Clockwork: not stated | M | Phase 4 SU1/SU2 | **Borrow technique** (see puddles section) |
| 2 | bevy_atmos `shelter.rs` | Top-down depth capture, 512x512 at 0.5 m, redone every 64 m; merges water heights; PNG debug dump | MIT/Apache; we have the same (`sf_precip_bevy/occlusion.rs`) | `weather particle occlusion`: extra pass, default **off**, "experimental" | Nothing between recaptures (ours too) | S | Phase 4 PR2 | **Skip**; steal the PNG dump and "merge water" |
| 3 | bevy_atmos `ssr.rs` | Forward SSR: copy lit colour after transparents, march depth prepass, 1 frame old, sky/probe fallback | MIT/Apache; needs a depth prepass, which our exterior camera lacks (far-field pitfall 2) | `puddle reflections` = `sky` (default) or `screen` (traces each puddle through depth+colour, copies opaque colour while ground is wet) | atmos: ~1.2 ms of the +1.5 ms for a river filling the view at 720p; est. >3 ms at 1440p. Clockwork: not stated | L | water.md, SU2 | **Skip** now; reopen only if a depth prepass lands for other reasons |
| 4 | bevy_atmos clouds (`clouds.wgsl`) | Per-pixel raymarch of a shell over a round earth, Perlin-Worley + curl, Beer/powder/2-lobe phase, 1/4 of pixels per frame + reprojection; **also draws the same clouds into a 256-px cube, one face per frame, for reflections and ambient** | MIT/Apache; same family as ours (Nubis) | Cloud *shading* is part of `upgrades`; lightning lights clouds near the bolt top and holds ~1.5 s glow (`lightning strength`). Mechanism undocumented | atmos 0.36-0.48 ms Medium, scaling with pixels; est. 2-4 ms at 1440p. Ours: +0.6-1.0 ms measured | S to check | SH-12 | **Skip** the march; **check** whether our probe/env map shows clouds (cube-per-frame trick) |
| 5 | bevy_atmos sky/fog/shafts | Rayleigh/Mie sky cube (0.05 ms), tinted exponential fog, shafts via Bevy volumetric fog | MIT/Apache | Volumetric fog on sun shadow maps + local lights, 160x90x64 froxels; ~0.24 ms (4090, 1080p, high) | Ours: froxels ~0.16 ms (4060, 1440p) | n/a | FG-1 | **Skip**; we are ahead |
| 6 | bevy_atmos water | Gerstner waves, foam, rain rings, river flow, auto-find by mesh name | MIT/Apache; our FFT + SSPR is stronger | Planar RTT reflection 512-2048, `reflection detail` 0-5, refraction, `rain ripple detail` 0-2, caustics | atmos ~1.5 ms (720p). Ours `water_fft` 0.14 ms | n/a | M6 water | **Skip**; reuse Clockwork's 3 ripple tiers |
| 7 | bevy_atmos look editor (`editor.rs`, `knobs.rs`) | F9 panel: presets, per-slider reset, **hold to compare with original**, saves only changed values as a diffable text file per level | MIT/Apache, 1.2k lines of `bevy_ui` on 0.19 | none (INI settings + in-game Visual Upgrades toggles, each master switch also "classic vs modern") | CPU/UI only | M | Phase 1 live A/B keys (ROADMAP 122) | **Borrow technique**: hold-to-compare and changed-only save, built on our A/B keys; no code needed |
| 8 | bevy_atmos `ao_direct.wgsl` + shadows tab | AO also darkens sunlit corners ("occlusion in sunlight"), 18 lines; PCSS behind Bevy's `experimental_pbr_pcss` | MIT/Apache | AO 0.10 ms (4090, 1080p; 0.04 ms in Vivec), half-res; PCSS-like sun softness | atmos 0.015 ms on top of SSAO | S | Phase 1 image quality | **Borrow technique**, low priority; ties to shadows/README F5 |
| 9 | Mudcrab `frame_budget.rs` | Real-clock frame times (not Bevy's clamped virtual clock); p99, p99.9, worst, count over budget and over 2x budget; **which side owned each late frame** (main, render, upload) | GPL now; re-implement for sf_diag | n/a | CPU only | M | 1.2 frame budget + gates | **Borrow technique** (not code) |
| 10 | Mudcrab `route.rs`, `capture_frame.rs`, matched-route doc | Deterministic route at fixed step; shots tied to the *extraction* of a screenshot entity, serialized, pose frozen through readback; receipts with LOD/pending state; no `dt` in movement | GPL now; extends R10 item 2 (shot batches) | n/a | none | M | 1.2, visual-checks batches | **Borrow technique**; one agent-run regression image set |
| 11 | Mudcrab adaptive streaming (`streaming/admission,priority,reservations,control.rs`, `research/adaptive-streaming.md`) | Backpressure on downstream queue (256), memory reservations from a converter-emitted per-asset cost file, headroom (2 GiB), spatial priority, startup/cruise/recovery budgets | GPL now; compare with our streaming.md rings/budgets | "loading bar", faster blends, `lod factor`, composite maps; no controller documented | CPU only | L | Phase 1 streaming | **Borrow technique**: per-asset cost estimates from the bake + memory-reservation admission, only if our bursts show memory spikes |
| 12 | Mudcrab `visibility_optimization.rs` | Structural nodes (no mesh, no bounds) on Bevy's `NoCpuCulling` change-driven path | GPL; ROADMAP already has "NoCpuCulling A/B (J2)" | n/a | CPU, measured by them on Metal | S | ROADMAP 107 (J2) | **Already planned**; do the A/B |
| 13 | Mudcrab merged-terrain batching (`terrain_batching.rs`) | Merged per-cell terrain mesh, atomic swap after the render world acks the upload; fixes Metal's per-indirect-record CPU loop | GPL; Metal-specific; we have CDLOD | Composite maps, `lod factor` | n/a on Vulkan | n/a | n/a | **Skip** (remember the swap-after-upload-ack rule) |
| 14 | Mudcrab bug-report, settings/grading UI, palettes, doors, environment DB | Bounded diagnostics bundle with privacy redaction; the rest is Skyrim-specific or UI | GPL | n/a | n/a | n/a | none | **Skip** (bug-report: revisit for external playtests) |

R10 follow-ups: item 1 (DX12 indirect-count workaround) is a public wgpu issue, ~20 lines; write it ourselves.
Items 2-4 (shots, UASTC encoder, MO2 reader) now need clean-room re-implementation for superfargoth, or a port into
bevyrrowind only. Mudcrab's docs say to "decide any Bevy 0.20 migration separately": nothing to learn there.

## Clockwork behaviour, summarised (docs only)

Each area has an `upgrades` master switch ("false draws as the OpenGL entry does") and every feature's 0 "compiles
nothing"; we already do this with `--off` and tiers. Stated costs, all RTX 4090: AO 0.10 ms (1080p), specular AA
0.06 ms (1440p), volumetric fog 0.24 ms (1080p, high), terrain horizon map under 30 us; none for wet, clouds or sky
light. Expect roughly 4-5x on a 4060 laptop (est.). Its rain-occlusion pass is off by default and "experimental".

## Rain puddles and wet surfaces (ROADMAP Phase 4, SU1/SU2)

**What Clockwork does (docs only).** Rain wets *outdoor* surfaces: ground gets darker and reflective, puddles form,
walls get streaks; interiors never change. Strength `surface weather` 0-2 (default 1). Wetness is **not saved**.
It dries over `surface drying time`, default 120 game minutes (10-1440); snow melts over 3x that, storm dust over
4x; accumulation and drying keep running even when the visual upgrades are off, so state stays correct when the
player toggles. Reflections: `sky` by default (sky and sun glint only); `screen` also traces nearby objects through
depth and colour and falls back to the sky, Vulkan only, costing a trace per puddle plus a colour copy while wet.
Cost: none published. Inference: a global scalar plus a per-pixel up-facing/outdoors test, since the docs mention
no per-area map.

**What bevy_atmos does.** A `StandardMaterial` extension with one scalar `wetness` (rises over 60 s of rain, falls
over 180 s), a per-material `allow` (0 under roofs) and a `puddles` flag. Puddles are a **world-space xz noise
threshold** on up-facing pixels (`world_normal.y > 0.85`), growing with wetness, edge softened by `fwidth`. The
shelter map keeps rain out from under roofs. No terrain height.

**What we have.** `WeatherFrame.surface` carries `wet, puddle, snow, ash, drying` (`sf_weather/src/frame.rs:341`) and
the occlusion map exists. Not built: `sf_surface`, `surface_state.wesl`, `WorldMaterial`. weather.md's design
(two-level clipmap, 32 m persistent grid in saves, SSR) is the heavy end; Clockwork and bevy_atmos ship without it.

**Cheap approach for slice 1 (proposal, needs owner go; it narrows weather.md "Surface state").**

1. **State = two scalars on the CPU**, not a clipmap: `wet` and `puddle` in `WeatherFrame.surface`, with rise and
   drying rates in game minutes (default drying 120 game min as a starting point; Morrowind's timescale is
   configurable, so keep it in game time). **Do not persist it** (Clockwork doesn't; a save loads dry or at the
   region's current weather). Drop weather.md's coarse persistent grid and clipmap from slice 1; they stay as an
   upgrade if per-place wetness (under a bridge for hours) is wanted later.
2. **Exposure = the occlusion map we already have.** Per pixel: `exposed = world_y >= occlusion_depth - eps`
   (open sky) fetched once; interiors skip it with a cell flag. One texture fetch, no new pass.
3. **Shader (WESL, in the `surface_state` hook on `WorldMaterial`)**:
   - soaked: `albedo *= mix(1, 0.55, wet*porosity*exposed)`, `roughness = mix(r, 0.18, ...)` (values from bevy_atmos,
     Lagarde-consistent with weather.md);
   - puddle: `up = N.y > 0.85`; `field` from one **tiled noise texture** (two taps, warped), not 6 hashed
     gradient-noise calls; threshold moves with `puddle`; edge width from `fwidth`; inside it albedo x0.6, roughness
     0.03, flat normal, F0 up to 0.04-ish, ripple normals scaled by rain intensity;
   - **terrain**: keep weather.md's "puddle level above layer height" rule (no noise), since it already has the
     height channel;
   - **far field**: wet darkening only (as spec'd).
4. **Reflections: sky and probe only** (Clockwork's default, free). Our environment map and light probes already
   reflect the sky; Solari's path reflects by ray tracing for free. **No SSR** (item 3). If the owner wants objects in
   puddles later, that is the only line item with a real cost.
5. **Tiers** (same shape as Clockwork's ripple detail 0-2): Low darken+gloss only; Medium + puddles; High + ripple
   normals and splashes (PR2). Off compiles to nothing.
6. **Budget and test.** Target <= 0.3 ms at 1440p High, 4060 laptop (est.; unmeasured). Add `--off surface`
   and a "stormy road" test scene to `visual-checks.md`; measure on/off before claiming a number.

Owner questions: (a) persist wetness in saves? (b) mirror puddles that show buildings (needs SSR, +ms), or
sky/probe only for slice 1?

## Recommendations (ranked)

1. **Wet/puddles: borrow bevy_atmos's technique, not its architecture** (rows 1, 2, 3), with the slice above. It is the
   one real gap in our weather, and the cheap form matches what Clockwork ships.
2. **Treat Mudcrab as GPL from now on**: re-implement rows 9-11 (frame-budget tail stats with frame-owner attribution,
   route/shot receipts, admission with memory reservations) from their descriptions in `sf_diag`/`streaming.md`;
   correct R10's licence text and its "port" items. Row 12 is already on our list.
3. **Hold-to-compare live tuning** (row 7) layered on the A/B keys already planned (ROADMAP 122), so FG-1/WE-5-style
   bisects happen at the owner's view instead of in agent screenshots.

## Sources

Mudcrab `git log 3129b57..origin/main`, `88b25a2`, `crates/engine/src/{frame_budget,route,capture_frame,
visibility_optimization,streaming/*,terrain_merge/*}.rs`, `docs/research/{adaptive-streaming,matched-route-driver,
metal-performance-fixes-20261008}.md`. bevy_atmos `README.md`, `CHANGELOG.md`, `src/{wet,shelter,ssr,clouds}.*`.
Clockwork (GPL, behaviour only): `docs/source/reference/modding/settings/{weather,shaders,water,fog,terrain,
postprocessing}.rst`, branch `clockwork-alpha`, <https://gitlab.com/OursCodeur/openmw>. Ours: weather.md, water.md,
`sf_precip_bevy/occlusion.rs`, `sf_weather/src/frame.rs`, ROADMAP 55, 107, 122, 168-172. Web searches found no
forum, video or release-note coverage of Clockwork's wet look, and no cost figure for it.
