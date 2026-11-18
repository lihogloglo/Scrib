# Visual checks: worldgen

Open visual checks for world generation (WG-*). Index and how-to: [`../visual-checks.md`](../visual-checks.md).
Specs: `superfargoth/docs/specs/worldgen.md`, `../specs/worldgen-adapter.md`.

### WG-S2: 10x Vvardenfell erosion look test (images; `target/release/scrib-worldgen.exe run bevyrrowind/worldgen/vvardenfell-s2.toml`)

Status: awaiting owner (2026-11-18). Open `target/worldgen/s2/sheet-*.png` (overview, red-mountain, ashlands,
bitter-coast, west-gash; 12 × 12 km crops, 20 m grid, light 315°/45°, true vertical scale). Arms left to right:
**A0** vanilla ×10 no erosion · **A1** stream power, uniform rock · **A2** per-region rock + Red Mountain as a
concave stratovolcano to ~3.3 km · **A3** A2 + talus/scree. Numbers and crop centres: `target/worldgen/s2/README.md`.
Look for: does any arm read as real land (branching valleys, sharp ridges, a real volcano) rather than blobs? Rate by
elimination (reject / maybe / keep). Is A2's Red Mountain shape right? Does per-region rock (A1 vs A2) or talus
(A2 vs A3) matter? Too little erosion overall? Known: only the macro stage (20 m); no mid-scale hydraulic detail
(WG-3) or runtime detail yet, so close-up texture is missing by design.
