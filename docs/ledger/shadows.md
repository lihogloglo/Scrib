# Ledger: shadows (and lamp lights)

Format: `date | tried | result | verdict | commit or doc`. Newest first. Owner checks are SH-/LT- ids in `bevyrrowind/docs/visual-checks.md`.

2026-11-12 | Owner check RO-20 (far-map upload fix P2), Balmora 18:30, clear weather, fly 2-3 km | far shadows follow and update on approach; NEW: subtle shadow swimming on hills while approaching | P2 confirmed; swimming open (suspects: far-map recentre texel shift, cascade handoff; untraced) | visual-checks RO-20
2026-11-12 | Post-mortem of P3 (`NoCpuCulling`): its lamp-list fix lives in `sf_shadow_cache_bevy`, which is **not installed in the game** (`POINT_SHADOW_CACHE = false`, `bevyrrowind/crates/mw_environment_bevy/src/render.rs:37`, since 4c87e1c 2026-10-21; lamp faces use the vendored `bevy_pbr` per-face cache instead) | so `CpuListedPointShadowCasters` was never set and stock Bevy put every GPU-culled static into every point face: ~64k work items per face, 113 stutters/min | verdict: P3 failure explained; any retry must work on the live lamp path (vendored per-face cache), not `sf_shadow_cache_bevy` | wiki engine/frame-and-views.md
2026-11-12 | P3: near statics tagged `NoCpuCulling`; lamp cache lists GPU-culled casters itself (`CpuListedPointShadowCasters`) | Balmora walk 3x, 1440p DLSS Q: median 10.93 on vs 10.32 off, p99 21.98 vs 13.94, stutters 113 vs 13.7 per min | opt-in (`BEVYRROWIND_GPU_CULL_STATICS=1`); SH-24 awaiting owner | 2b63681, `.sessions/perf-push.md`
2026-11-12 | Static-scene fix 5: cached lamp faces see GPU instance batches as statics (`GpuBatchedLightVisibility`, `ShadowCastersChanged`) | tests only (+2 cache, +2 scene), no perf number | kept; owner night check pending | CHANGELOG 2026-11-12
2026-11-12 | Owner re-check of torch and lamp shadows on the player | still flickers hard close to some torches and lamps | open (SH-21) | visual-checks SH-21
2026-11-09 | Sun shadow early-out in the directional loop (skip cascade fetch and contact march where contribution is 0) | naga-validated, never run, no A/B | open, SH-23 awaiting owner | CHANGELOG 2026-11-09, shadows.md "Reset B2"
2026-11-05 | Lamp reach follows shadow state (1.5 x radius unshadowed, 4 x shadowed) | owner: towns and interiors darkened at night | rejected, default back to 4 x (`--range-unshadowed`) | visual-checks LT-reach, lights.md
2026-11-05 | Lamp bodies skip probe GI (`GiSkipEmissive`) so emissive lamps do not light twice | raster and Solari looks unchanged | kept | CHANGELOG 2026-11-05 l-b-lampbody
2026-11-02 | Exterior fps halves after leaving the Census office, shadows suspected | not measured properly | open (SH-22 known bug) | visual-checks SH-22
2026-11-01 | Lamp contact shadows back on for High/Ultra (A1 had removed them: 0.80 ms GPU per shadowed lamp at 1440p native, ray was 0.5 m, now 0.3 m) | owner confirmed in Census office; sun contact shadows seen outdoors only; SH-16 still awaiting | kept | CHANGELOG 2026-11-01 contact-audit
2026-11-01 | Shadow audit fixes: characters follow worldspace `RenderLayers`, budget ranks view-visible lights, Temporal filter parked to Gaussian without jitter, contact/SSR march in DLSS pixel space, sun PCF full receiver slope, #26234 + #26170 | torches now cast; RO-11 ground bands gone (owner 2026-11-01); flicker and cut faces remained | kept; SH-21 partial | `superfargoth/docs/research/shadows/2026-11-01-audit.md`, shadows.md "Shadow fixes as built"
2026-11-01 | SH-20b: lamp faces refresh per light (all dirty faces in one frame), moved lights redraw whole, mover refresh 2 lights per frame | owner: smooth, but flicker and seam-cut shadows on torches remain | partial | shadows.md
2026-10-28 | Clockwork-style world-unit directional PCSS (patch H2), SH-11 far-shadow PostUpdate ordering, SH-20 close-mover refresh within 16 m | SH-11 flashes gone (owner 2026-10-30); sun shadows follow NPCs smoothly; lamp part partial | kept | CHANGELOG 2026-10-28
2026-10-21 | Reset B2: lamps use 2x2 hardware PCF, point/spot shadow early-out (own patch) | -0.09 to -0.10 ms main-pass GPU per shadowed lamp at 1440p; lab 16 lamps 4.83 -> 2.72 ms, census opaque 2.94 -> 2.08 ms; frames render-thread bound so medians barely moved | kept; SH-19 inconclusive | shadows.md, perf.md "Shadow reset B2"
2026-10-21 | Reset B1: vendored Bevy PRs #26149 (slim per-face schedule) + #26161 (skip unchanged views) hoping to replace the lamp cache | walkers and NPCs defeat #26161 (any moving caster redraws the cube): lab 8 lamps 11.4 vs 6.9 ms, census 15.0 vs 9.0 ms against the cache; patch + cache census 9.0 -> 6.5 ms | patch kept, cache kept | shadows.md "Reset B1", DECISIONS 2026-10-21 shadowB1
2026-10-21 | Reset A2: deleted far static sun map, caster cull, `--off shadowtune/shadowsplits`; lamp maps captured while the door fade is black | census gate 0.35 s; first exterior visit still compiling capture pipelines at 1.5 s | kept (deletions) | shadows.md "Reset A2"
2026-10-21 | Reset A1: contact shadows sun-only, sun/moon removed from interior views (shaded per pixel for 0.8 ms), GTAO per preset | GTAO ~0.9 ms at 1440p High; owner could not judge AO or contact shadows without toggles (SH-16) | kept; toggles added 11-01 | shadows.md "Reset A1"
2026-10-21 | Render-audit decision "delete the lamp cube cache in A, #26161 replaces it" | wrong: B1 measured and kept the cache | reverted | DECISIONS 2026-10-26 (docs S6b)
2026-10-20 | V1 lamp cache `sf_shadow_cache_bevy`: static casters cached per light, movers drawn over | first build cached nothing (48 of 48 faces held a "mover"); after 8 fixes census 8 lamps 16.8 -> ~6.3 ms (960p, 0 lamps 4.0); Balmora walk gate not met | kept | shadows.md "V1 as built"
2026-10-20 | Lights1 profiling of one 1024^2 lamp shadow | 3.5 ms frame on the walk, ~2 ms a Bevy 0.20 leak of cube-face views | fixed (`drop_stale_light_views`) | lights.md "Measured"
2026-10-20 | Lamp shadows "only a few metres, grainy, random" and contact-shadow bands at edges (owner) | SH-13 and SH-14 logged | open (known bugs) | visual-checks SH-13, SH-14
2026-10-19 | 8 uncached point shadows indoors (shf6) | owner: flicker, thin chandelier cable | rejected (SH-9) | visual-checks SH-9
2026-10-19 | Cached far static sun map beyond the cascades (shf3 step b) | built off by default, never adopted | reverted (deleted in A2) | DECISIONS 2026-10-19 shf3
2026-10-19 | Sun cascades: PSSM practical split to 250 m replacing the 504 m rule (shf3 step a); 4096^2 maps tried | texels 1.4/3.8/10.5/35 cm vs 1.4/5.2/19/70; GPU about 0 at Seyda Neen; 4096^2 was +1.4 ms | kept 2048^2; presets since cut to 160-280 m | distant-shadows.md "Sun cascades"

## Standing facts

- Stock Bevy shadows plus vendored patches; never inject into Bevy's render world (DECISIONS 2026-10-21).
- The lamp cache is needed: moving casters defeat Bevy's skip rule (B1 measurement).
- Contact shadows are visible outdoors; lamp contact shadows cost about 0.8 ms GPU per lamp at native 1440p.
- Lamp shadow quality (SH-13, SH-21) is the open core problem (hard problem H2); perf work on it is parked.
- Meshes that cast nothing need `NotShadowCaster`, or the cache tracks them and never settles.
- Frames are render-thread bound; GPU-only wins barely move medians.
- High has 8 lamp maps at 512^2 chosen by perf; the owner has not picked the final count.
