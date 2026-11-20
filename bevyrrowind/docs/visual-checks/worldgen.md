# Visual checks: worldgen

Open visual checks for world generation (WG-*). Index and how-to: [`../visual-checks.md`](../visual-checks.md).
Specs: `superfargoth/docs/specs/worldgen.md`, `../specs/worldgen-adapter.md`.

### WG-S2: 10x Vvardenfell erosion look test (images; `target/release/scrib-worldgen.exe run bevyrrowind/worldgen/vvardenfell-s2.toml`)

Status: partial (owner 2026-11-18): A1 and A2 show many parallel straight lines ("feels like a motherboard": D8 single-flow channels on smooth upscaled slopes); hillshades are hard to judge, owner asked for a 3D scene → WG-S2b routing fix + WG-V viewer. Open `target/worldgen/s2/sheet-*.png` (overview, red-mountain, ashlands,
bitter-coast, west-gash; 12 × 12 km crops, 20 m grid, light 315°/45°, true vertical scale). Arms left to right:
**A0** vanilla ×10 no erosion · **A1** stream power, uniform rock · **A2** per-region rock + Red Mountain as a
concave stratovolcano to ~3.3 km · **A3** A2 + talus/scree. Numbers and crop centres: `target/worldgen/s2/README.md`.
Look for: does any arm read as real land (branching valleys, sharp ridges, a real volcano) rather than blobs? Rate by
elimination (reject / maybe / keep). Is A2's Red Mountain shape right? Does per-region rock (A1 vs A2) or talus
(A2 vs A3) matter? Too little erosion overall? Known: only the macro stage (20 m); no mid-scale hydraulic detail
(WG-3) or runtime detail yet, so close-up texture is missing by design.

### WG-V: 3D terrain viewer (`owner_launch.sh worldgen-view <arm dirs>`; built 2026-11-18, `mw_viewers_bevy::worldgen_view`)

Status: viewer works (owner 2026-11-18, 144 fps); terrain verdict: A2/A3 rejected: "mountain pulled upward (vertices)", many evenly spaced grooves "like a bad definition problem", scale hard to read; "this ain't it at all, rethink entirely". Screenshot: owner ShareX bevyrrowind_c7Yd8shYSQ.jpg. Keys: WASD + mouse, Space/Q up-down,
Shift ×5, wheel = fly speed, **G walk** (1.7 m eye), **1-9 switch arm, same camera**, `[` `]` sun azimuth, `,` `.` sun
elevation, H HUD, F10 quit. Starts 15 km south of the peak at 500 m. Look for: the WG-S2 questions in 3D (real land or
blobs, the volcano's shape, parallel channels); also whether the viewer itself is good enough to judge terrain on
(shadows, fog, LOD pop at 6 / 24 km, no cracks). FPS is vsync-capped; fly mode keeps the camera above the ground, so
switching to a higher arm lifts it.
- Owner 2026-11-18 (viewer with real Etna/Teide/Iceland vs S2b B3, `--plain-terrain`; contour shader path bug: invisible
  terrain, fix running): real terrain has **less mid-detail** than ours; Morrowind-derived B3 is "more rugged (more gullies,
  valleys) for the same scale". → less dissection (wider valley spacing, higher channel threshold, finite time), volcanic
  regions shaped by lava-flow landforms, and real-DEM statistics as tuning targets (WG-M).
- Owner 2026-11-18 (contours fixed; real places + filter pairs + S4 C2/C3): Iceland/Hekla "definitely not"; Etna and
  Teide feel **VAST** (wide views, long slope from the sea); Vvardenfell = flat lowlands with the volcano surging out
  of them, **more vertical/dramatic than real life** (Etna the better ref for the pop); Teide's barrancos/fractures =
  foyadas. Erosion filter: etna 5 "looks amazing", 7 > 6. C2 parametric volcano "great great start": less
  cone-symmetric, gullies too simple/radial, higher, steeper, sub-peaks. Decisions: gentle apron coast → base,
  **summit ~5.5 km**.
- Owner 2026-11-18 (WG-VO V1-V4 vs C2, Etna, Teide): "very promising". Gripes, fine for now: the cones feel like
  "different cones merged together" (artificial); the littoral is very smooth and undeveloped; relief stops being
  interesting down the volcano's slopes. Next: original settlements in place + trees, for a screenshot.

### WG-G: generated island in the game renderer (G1 2026-11-18, G2 2026-11-19)

Launch: `bash bevyrrowind/tools/owner_launch.sh view "world:D:/Gamedev/Bevyrrowind/target/worldgen/vo/V3"` (camera
11 km south of the summit, 400 m up). Look for: gullies and ridges down to ~1-2 m when walking; console
`worldgen.filter false/true` shows what the filter adds; hitches while flying vs the `--off gputerrain` arm; clouds
(base ~3.1 km) with the 5.5 km summit through them. Placeholders: flat height/slope colours, no shore foam.
- Owner 2026-11-19 (G1, CPU tiles): runs, fps good, a lot of hitches, erosion not visible. → G2 awaiting owner.

### WG-X: Vvardenfell 10x, same map (2026-11-19, spec `worldgen-adapter.md` "Same map at 10x")

Launch: `bash bevyrrowind/tools/owner_launch.sh view "world:D:/Gamedev/Bevyrrowind/target/worldgen/x10v/<arm>"` (arms s55,
s45, s55c; bare base `target/worldgen/x10`). Look for: Red Mountain's height and prominence, the apron (gentle rise from
the lowlands), Morrowind's crater and foyadas still recognisable on top; north side is steep by geometry (coast 10 km).
- Owner 2026-11-19 (bare x10 base, no volcano, no content): "I think it works"; wants the volcano higher with a gentle slope → X1 arms.
- Owner 2026-11-19 (s55): "very nice like that"; s55 is the same-map base. Later: the blobbiness inherited from Morrowind's coarse heightmap (not now).
- Owner 2026-11-19 (x10s Balmora, all of WG-X1-X5, 200k flora): "alright"; textures super repeated, world extremely empty → more
  flora/rocks (`--set scatter.budget=N`, full density 7.2 M), anti-tiling + VCLR tint later. Log: 66-108 fps over 10 s
  windows once loaded, first ~25 s near 8 fps; 890 pending pipelines at the 14.7 s reveal cap (alpha-blend PBR pipelines
  up to 5 s each in the driver), 23,028 shown entities, 707 StandardMaterials, 2.07 GB of textures.
- Owner 2026-11-19 (x10s, flora 1M then full 7.2 M, end of session): 26 fps; still empty; no proper forest (trees not
  snug together), rocks all one size, "sprayed some assets over the terrain like little bits on a donut"; relief still
  smooth and blobby (next big thing); some settlements misplaced (Balmora must sit on both banks of its river).
  Next: study the Chlorophyll engine for vegetation at scale.

### WG-X7a: Terrain ground texture anti-tiling (Mikkelsen 2022 hex-tiling, 2026-11-20)

Status: **confirmed (owner 2026-11-20)**: "world of difference", all live toggles (`terrain.hex_tiling`, `.rot_strength`, `.contrast`, `.scale`) work. Kept on by default.
Launch: `bash bevyrrowind/tools/owner_launch.sh view "world:D:/Gamedev/Bevyrrowind/target/worldgen/x10s" --hour 12` (also applies to classic Seyda Neen / Balmora).
Live A/B in console (`~`): `set terrain.hex_tiling off` vs `set terrain.hex_tiling on`; also live: `set terrain.hex_tiling.rot_strength 0` (offset-only vs full rotation `1`), `set terrain.hex_tiling.contrast 7`, `set terrain.hex_tiling.scale 1`.
Look for: repeating tile grid gone across large 10x single-layer expanses (hillsides around Balmora, Ashlands, Red Mountain apron); lighting on bumpy rock/dirt normals stays consistent (no inverted or wobbling highlights from rotated tiles); no seams at hex borders or when crossing 1024 m floating-origin shifts; height-blended layer transitions stay crisp. Check cobblestone/paved roads in Balmora under `rot_strength 1` vs `0`.

### WG-SHARP: Sharp landforms + cliff rock shading + Option A (WG-X7b/c + Phase 3a/3b, 2026-11-20)

Launch on `target/worldgen/x10cs` (dramatic) or `target/worldgen/x10c-sharp` (sharp) and compare vs `target/worldgen/x10s` (smooth base).
- Owner 2026-11-20 (v1 `x10cs` / `x10c-sharp`): "both look nice", spotted seams between terrain tiles / near range → fixed clipmap edge fades (`layer_edge_fade`, morph-blended `terrain_curvature`, `terrain_normal`/`macro_albedo` border cross-fades) and added Option A (per-region geology, routed volcanic lava lobes, `VCLR` macro tint `layers.tint`).
Live console (`~`) A/B toggles:
- Shader cliff rock & crevices (`WG-X7b`): `set terrain.cliff.enabled off` / `on`, `set terrain.cliff.slope_start_deg 28`, `set terrain.cliff.slope_full_deg 45`, `set terrain.cliff.crevice_strength 0.45`.
- Runtime sharp strata/Voronoi/ridges (Phase 3a): `set worldgen.sharp 0` (old gully-only) vs `1` (default) vs `2` (aggressive); `set worldgen.filter false` / `true`.
Look for:
- **Clipmap / range seams gone:** fly 300–1200 m above terrain and sweep across the ~900–1000 m near-splat ring and clipmap level borders — no hard square lines in normals, shading, crevices, or near-to-far albedo transition.
- **Per-region geology:** Bitter Coast stays low/marshy (quiet scarps), Ascadian Isles has gentle terraced rolling hills, Grazelands reads as flat-topped stepped mesas, West Gash has towering canyon escarpments, Azura's Coast / Sheogorad have sheer rocky sea-cliff headlands.
- **Volcanic lava lobes:** Molag Amur / Ashlands / Red Mountain slopes show downhill-routed dark rocky lava ribbons with raised lateral levees and steep aa fronts.
- **Morrowind `VCLR` macro tint (`WG-X7c`):** subtle regional soil/road/shadow colour variation carried onto both far macro and near ground splats.
- Settlements still sit cleanly on their stamps without floating/burying; no CDLOD culling holes or wet/dry sea glitches.
