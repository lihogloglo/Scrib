# Crimson Desert: ray-traced lighting, screen-space detail

**Pearl Abyss's BlackSpace Engine (Crimson Desert, March 2026) puts the expensive lighting on ray tracing and the
fine surface detail on screen-space and displacement tricks.** It is one of the owner's three look references, with
RDR2 and Cyberpunk. There is no developer talk yet: everything below comes from Digital Foundry coverage (read
through secondary reports), frame-capture analyses and community breakdowns. Treat it as probable, not confirmed;
the sources disagree on details (marked "disputed").

## Lighting and shadows

- **Diffuse GI is ray traced per pixel**, not from probes (contrasted with UE5 Lumen in Digital Foundry's PC
  preview), and stays on on PS5 Pro. Some sources call it full path tracing; disputed. One frame analysis reports
  about one ray per 16 screen pixels on the base PC setting, so it leans hard on denoising and on DLSS Ray
  Reconstruction / temporal accumulation ([GameGPU][gpu-pt], [Sportskeeda][sk]).
- **Ambient occlusion is ray traced**, replacing SSAO's crevice glow ([GameGPU][gpu-pt]).
- **Reflections are hybrid:** screen-space reflections for what is on screen, a ray-traced fallback for off-screen
  and distant geometry (water in the PC preview, [Digital Trends][dt]). Same idea as our SSPR, with a better fallback.
- **Volumes are lit and shadowed:** fog, smoke and clouds take light as volumes, giving shafts and shadows inside the
  fog ([GameGPU][gpu-pt]).
- **Weak spots:** grain in dark interiors and shadowed areas at lower lighting settings (low sample counts), and a
  raster fallback without RT that is reportedly heavy (dense cascades) and noisier in AO ([GameGPU benchmarks][gpu-bm],
  [Notebookcheck][nbc]).
- Local light shadows: no reliable source on how lamps are shadowed (a forum claim of screen-space shadows is
  partly AI-written; don't cite it).

## Surfaces

- **Displacement at high settings, parallax as the fallback:** tessellated displacement on High, parallax mapping
  substituted on Low; one analyst reads it as parallax with offset limiting to keep silhouettes, another as real
  geometry (disputed, [X: Avasilcutei][x1], [X: Grummz][x2]).
- **A separate "Screen Space Displacement" setting.** Known flaw: surfaces fold in at the screen edges, since a
  screen-space pass has no data past the border ([parallax demo][pm], IGN preview thread).

## What it means for us

1. **Screen-space work is cheap for our bottleneck.** It costs GPU per pixel and adds no render views; our walk is
   CPU-bound on per-view work (46 views, most of them lamp faces, [engine/frame-and-views](../engine/frame-and-views.md)).
   The standard hybrid: a few nearby lamps with real shadow maps, screen-space shadows for the rest (Days Gone's
   open-source Bend screen-space shadows are the usual reference; check point-light support before adopting).
   Limit: no off-screen casters, long lamp shadows can break, so it complements shadow maps. Our contact shadows
   cover sun, moon and lamps on High/Ultra ([engine/shadows](../engine/shadows.md)); ROADMAP 1.23.
2. **Its lighting is our Ultra tier's family.** Solari plus DLSS Ray Reconstruction is the same approach; its noise
   complaints are the risk we would carry there. The raster tier must stand on its own (our frame target is raster).
3. **SSR plus a fallback for off-screen** is the shape to keep for water reflections.
4. **Parallax and displacement are look work (Phase 4).** Vanilla Morrowind textures have no height maps; PBR packs
   often do (OpenMW reads normal-height textures). Prefer parallax with offset limiting or displacement over a
   screen-space pass, to avoid the edge folding.

## Sources

[gpu-pt]: https://en.gamegpu.com/test-gpu/action-fps-tps/trassirovka-puti-v-crimson-desert-tekhnicheskij-razbor-blackspace-engine
[gpu-bm]: https://en.gamegpu.com/test-gpu/action-fps-tps/crimson-desert-benchmarki-videokart-i-protsessorov-na-pk
[sk]: https://tech.sportskeeda.com/gaming-news/crimson-desert-engine-ray-tracing-performance-tech-details
[dt]: https://www.digitaltrends.com/gaming/crimson-desert-delivers-native-4k-ray-tracing-without-upscaling/
[nbc]: https://www.notebookcheck.net/These-Crimson-Desert-graphics-settings-fix-the-game-s-major-issues.1255095.0.html
[x1]: https://x.com/nimlot26/status/2039723169632616911
[x2]: https://x.com/Grummz/status/2039722411059495197
[pm]: https://github.com/mfagerlund/parallax-mapping-demo

- [GameGPU: path tracing in Crimson Desert, BlackSpace Engine analysis][gpu-pt]
- [GameGPU: GPU and CPU benchmarks][gpu-bm]
- [Sportskeeda: engine and ray tracing details (cites Digital Foundry)][sk]
- [Digital Trends: native 4K ray tracing (Digital Foundry PC preview)][dt]
- [Digital Foundry PS5/PS5 Pro tech review (ResetEra thread)](https://www.resetera.com/threads/digital-foundry-crimson-desert-the-big-ps5-ps5-pro-tech-review-every-graphics-mode-tested.1479139/)
- [Notebookcheck: settings that fix the major issues][nbc]
- [X: Avasilcutei, parallax with offset limiting][x1]; [X: Grummz, surface shader is geometry][x2]
- [mfagerlund parallax-mapping demo][pm]
