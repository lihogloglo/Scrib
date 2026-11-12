# Ledger: sky, atmosphere, clouds, fog, weather

Format: `date | tried | result | verdict | commit or doc`. Newest first. Owner checks are SK-/WE-/FG-/RO-/NS- ids in `bevyrrowind/docs/visual-checks.md`.

2026-11-12 | SKY-1 diagnosis: probe filter passes `lightprobe_irradiance_map` 0.43 + `lightprobe_radiance_map` 0.23 ms looked per-frame despite `EnvMapRefresh` | slicing works (1048 samples in 8418 frames = 1/8, `floor_near/*-2.json`); the report averaged per sample because Bevy spans exist only on frames a pass runs (`sf_diag_bevy/src/lib.rs:408`, NaN skipped `sf_diag/src/stats.rs:33`): real share ~0.09 ms/frame | report fixed: `GPU ms/frame`, `ms/run`, `runs/frame`, timed sum uses the share (test `time_sliced_pass_is_amortised_per_frame`) | uncommitted, `sf_diag/src/markdown.rs`
2026-11-12 | SKY-1 P1b: `atmosphere_luts` 0.36 -> 0.84 ms was per-sample cost, not static LUT recomputes (scale updates are stepped and rate-limited to 4/s, `sf_sky_bevy/src/lib.rs:555`; p95 1.14 vs mean 0.84 shows no spikes): `functions.wesl` summed terms per sample (2 density + per light 1 falloff + 1 phase per scattering term, sun and moon) | combined medium LUTs written on the GPU in the cached static pass (`medium_lut.wesl`), stock single-fetch sampling restored; no CPU bake on weather | expected `atmosphere_luts` ~0.36 ms, awaiting perf run | uncommitted, vendor/README.md P1b
2026-11-12 | Follow-up to P1: cut the GPU `atmosphere_luts` cost (P1b) | not done; stopped on owner request, any partial edits are uncommitted | open | `.sessions/perf-push.md`
2026-11-12 | P1: atmosphere medium never re-bakes on the CPU; weather moves per-term scales (`ScatteringMediumScales`) held in the static LUT key, vendored `bevy_pbr` patch | removed ~4.3 ms render-thread bake x ~51 per 65 s Balmora walk; A/B standing frame 9.53 -> 9.98 ms (GPU bound), `atmosphere_luts` 0.36 -> 0.93 ms (+0.57 ms): each scale change recomputes transmittance and multiscattering | kept; GPU regression open; RO-21 awaiting owner | c080806, vendor/README.md P1 row
2026-11-12 | Far shadow map upload no longer touches every terrain, proxy or impostor material (P2) | stutter fix; bind groups kept (reuse path in vendored gpu_image.rs) | kept | CHANGELOG 2026-11-12
2026-11-06 | Cloud history: 16-sample accumulation, variance clip, STBN blue-noise jitter (WE-10 fix 1) | owner: shimmer stays, storm not over water | failed, WE-10 open | CHANGELOG 2026-11-06 cloud-history
2026-11-06 | WE-10 diagnosis: a local storm seen from outside is a wall at the region border (256 m band, far shafts at in-storm extinction) | 3-slice plan written, no code | open | CHANGELOG 2026-11-06 cloud-edge
2026-11-06 | Night fog fill adds the camera's ambient floor (WE-9 cause 2) | fog fix confirmed 2026-11-06; hour-key wrap (cause 1) and unlit rain at night (cause 3) open | kept | visual-checks WE-9
2026-11-06 | Night lighting: N1 ambient lights diffuse only, N2 airglow added to the sky-view LUT (own patch), N3 moonlight 31.4 lx (2.5 x pi x floor) with Lambert phase | owner: ground better (N1), moonlight OK (N3), night too lit vs OpenMW, Secunda not visible, N2 unconfirmed | kept; RO-16 open | visual-checks RO-16, vendor/README.md N1 and N2 rows
2026-11-06 | Rain lean: drops follow wind at drop height, calm onset, preset tilt cap (was 21-44 degrees, never vertical) | confirmed (Rain about 9-15 degrees, thunderstorm <= about 25) | kept | visual-checks WE-8
2026-11-05 | Weather ambient tint from `Morrowind.ini` by time of day (OpenMW port); terrain reflectance 0.5 -> 0.3 against the night "veil" | ground veil better after fix 1; see RO-16 above | kept | CHANGELOG 2026-11-05 night-ambient, terrain-veil fix 1
2026-11-05 | Lamp reach follows shadow state (1.5 x radius unshadowed, 4 x shadowed) | owner: towns and interiors darkened at night | rejected, default back to 4 x (`--range-unshadowed`) | visual-checks LT-reach, lights.md
2026-11-05 | Lamp bodies skip probe GI (`GiSkipEmissive`) so emissive lamps do not light twice | raster and Solari looks unchanged | kept | CHANGELOG 2026-11-05 l-b-lampbody
2026-11-02 | Fog sky fill as one isotropic per-frame value (sky-dome mean, SH L0) instead of per view azimuth | vertical light pillar gone (FG-9 confirmed); god rays still not visible at sunrise (FG-10) | kept | CHANGELOG 2026-11-02 fg9-fog
2026-10-28 | WE-5 storm occlusion fix, 1.6 km cloud-deck cap, dry `PrecipOcclusion` skip | owner: shelter too generous, cap to a few metres, clouds maybe high | partial | ROADMAP 1.8, visual-checks WE-5
2026-10-21 | Night exposure: 1 nit ambient floor (LT-1) rejected as "can't see anything"; 4 nits at EV100 floor 3.5 | LT-1 confirmed 2026-10-21 | kept | `bevyrrowind/crates/mw_assets/src/lights.rs`, ROADMAP 1.7
2026-10-21 | Volumetric fog black bands on fast turns (FG-1) | not reproduced | open if seen (`--off foghistory` kept) | ROADMAP 1.10
2026-10-20 | Time-sliced sky env map (`EnvMapRefresh`): Bevy refilters ~0.8 ms GPU every frame | refresh every N frames, `--off envslice` | kept | `superfargoth/crates/sf_sky_bevy/src/env_refresh.rs`
2026-10-20 | Local-light budget and light draw distance (lights1) | ~290 unshadowed lights cost nothing measurable; standing in Balmora ~0.5 ms GPU for 210 lights | not built | DECISIONS 2026-10-20 lights1

## Standing facts

- Weather never re-bakes the medium on the CPU; a model change still does.
- GPU `atmosphere_luts` was ~0.9 ms standing in Balmora after P1 (0.4 before); P1b should restore ~0.4 (unmeasured).
- The night is readable through a 4-nit ambient floor and an EV100 floor of 3.5, not through wide exposure.
- Lamp reach 4 x radius is the owner-accepted default; short reach was rejected.
- Weather is a pure function of records, seed, overrides, clock and site (a host can replicate it).
- Known open: WE-6 grainy rain on camera move, WE-7 rain stops over water, WE-10 storm borders, FG-7, FG-8, FG-10.
- Night checks SK-2 and NS-2..4 are deferred; moons and stars are unconfirmed by the owner.
