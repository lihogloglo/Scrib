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
