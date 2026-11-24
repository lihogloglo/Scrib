# Grass blade density, shipped games (2026-11-23)

Finding: **no shipped game publishes near-camera blades/m2.** Studios publish budgets and LOD tricks, not density.
Coverage = density x width (m of blade width per m2 = optical depth per metre of grazing sight line).

| Game / source | blades/m2 near | width | coverage | LOD falloff | source |
|---|---|---|---|---|---|
| Ours (bevy_grass MEADOW) | 60 | 0.02 m | 1.2 | lod1/lod2/fade | repo |
| Ghost of Tsushima (Sucker Punch, GDC 2021) | not published | not published | - | "twice as wide for half as many blades" with distance (secondary notes); per-tile compute placement, 15-vert high / 7-vert low blade | https://publish.obsidian.md/xybre/3.+Reference/Creative/Development/Gamedev/Visuals/Shaders+-+Grass+and+Foliage ; GDC Vault 1027214 |
| GoT total (secondary) | ~83,000 blades on screen, ~2.5 ms (PS4) | - | - | - | https://hashnode.com/posts/what-we-can-learn-from-grass-in-ghost-of-tsushima-renders/69e130b2ffbb787634eb9ef4 |
| UE Landscape Grass Type | Density = instances per 10 m x 10 m (100 m2): sample 400 = 4/m2; forum-reported cap 1000 = 10/m2. Instances are meshes/clump cards, not single blades | mesh-defined | - | Start/End cull distance per variety | https://dev.epicgames.com/documentation/unreal-engine/grass-quick-start-in-unreal-engine ; https://forums.unrealengine.com/t/newlandscapegrasstype-grass-density-locked-after-1000/629376 (units: my reading, unverified) |
| Horizon ZD/FW | not published (talk members-only); grass = few-vertex cards, not blades (secondary) | - | - | - | GDC Vault 1025530 |
| Far Cry 5, Witcher 3, BotW | not published | - | - | - | none found |
| Jahrmann/Wimmer I3D 2017 | not read (PDF >10 MB, fetch failed) | - | - | culling removes up to 75% of blades unnoticed (2016 poster) | https://www.cg.tuwien.ac.at/research/publications/2016/JAHRMANN-2016-IGR/JAHRMANN-2016-IGR-poster.pdf |
| Real lawn (reference) | ~32,000 (3,000/ft2, secondary) | ~3 mm | ~100 | - | https://www.gardenguides.com/129203-many-blades-grass-square-foot.html |

Not obtained: GoT tile size, blades per tile, LOD distances (need the GDC Vault video or slides); HZD, Acerola, Unity HDRP numbers.

## Recommendation (judgement, not sourced)
1. Metric is coverage, not blades/m2: at a grazing view what matters is blades hit per metre of sight line. 60/m2 x 2 cm = 1.2; "dense carpet" at 2 cm blades: raise near density to 150-250/m2 (coverage 3-5), tunable, owner picks by eye.
2. Width should grow with distance, not count only: follow GoT's trade (half the blades, twice the width keeps coverage constant), so LOD tiers hold coverage and only blade count/vertices drop.
3. Keep the near tier short (about 10-15 m); beyond it the fade/thinning absorbs the extra cost.
