# How Morrowind engines keep towns fast (OpenMW, Godotwind, MGE XE)

Research note, 2026-10-19. Research only: no code was read into our framework and none is quoted here.

**Clean-room rule.** OpenMW is GPL-3.0. This note describes behaviour, user-facing settings, defaults
and documented reasoning only: no code, no pseudocode, no identifiers or tables copied from source.
Setting names are quoted from OpenMW's public settings reference (user documentation). Godotwind is
MIT, but it is an OpenMW-inspired Godot port, so we treat it the same way and take only behaviour and
lessons. Nothing here may be pasted into `superfargoth`. Any implementation has to be written from the
engine-neutral rules in the "For us" sections.

Units: Morrowind uses about 70 units per metre. One exterior cell is 8192 units, about 117 m.

Our baseline (`docs/perf.md`, "Frame budget"): the target is 11 ms at High, 1440p with DLSS Quality,
on a laptop RTX 4060. Balmora runs 14-15 ms standing and about 25 ms walking at eye height. The
render thread is the bottleneck, with 44-52 k mesh entities. Sun cascades cost 4.8 ms of CPU
(12.6 k caster entries over 4 cascades), actors cost 3 ms on the main thread, and lights and
particles cost almost nothing. The main thread also runs 16 visibility passes.

---

## 1. Static objects in towns: draw calls and CPU

### What OpenMW does

- **Active grid.** OpenMW simulates and fully loads a 3x3 block of exterior cells around the player:
  grid radius 1, so about 350 m across. Without distant terrain, the default `viewing distance` of
  7168 units (about 102 m) is chosen to stay inside that grid: the docs give the rule
  "cell size x grid radius - 1024" so geometry never pops in at the grid edge. Wide fields of view
  "can require reducing this by 25 % or more". With distant terrain on, the slider goes up to 81920
  units (about 1.2 km). Source: [camera settings](https://openmw.readthedocs.io/en/latest/reference/modding/settings/camera.html).
- **Object paging (merging statics into chunks).** On by default, together with distant terrain.
  Objects outside the active grid are loaded through the terrain's quadtree chunks. Within a chunk,
  OpenMW merges objects "when their merging benefit outweighs their merging cost". The spotlight
  article gives the reason: draw calls are "the main bottleneck if you want to display statics". The
  merge runs at runtime, in background chunk builds, and keeps no separate LOD meshes ("no system yet
  to generate lower-detail meshes"; vanilla assets are already low-poly). Sources:
  [terrain settings](https://openmw.readthedocs.io/en/latest/reference/modding/settings/terrain.html),
  [spotlight "Turning the pages"](https://openmw.org/2020/openmw-spotlight-turning-the-pages/).
- **Active grid object paging.** This is the same merging applied *inside* the 3x3 active cells, for
  statics that cannot move. It is on by default now (it was off in 2020). The docs say: "Active grid
  paging significantly improves the framerate when your setup is CPU-limited." The spotlight article
  says the gain is largest "in draw-heavy areas like cities and towns". Forum users with Balmora
  slowdowns (60 dropping to 25-40 fps) are told to turn it on and to reduce shadows and water
  reflections ([forum](https://forum.openmw.org/viewtopic.php?p=70035)). Its known costs:
  - Merged chunks hit per-object light limits, so the docs advise raising "max lights".
  - A 2024+ issue reports frame-time gaps that grow as more cells get paged, worse with active-grid
    paging ([#8918](https://gitlab.com/OpenMW/openmw/-/issues/8918)).
- **Merge tuning (defaults):**
  - `object paging merge factor` 250: higher means more complex objects get merged.
  - `object paging min size` 0.01: the smallest apparent size (object size divided by distance)
    that is still drawn. A 1 m object disappears beyond about 100 m.
  - `object paging min size merge factor` 0.3: cheap objects get a second chance and are drawn from
    farther away even below the min size.
  - `object paging min size cost multiplier` 25: larger values drop cheap objects more readily.
  - `debug chunks` tints each merged batch in its own colour, which is handy for checking the
    batching.
  - Community tuning raises min size to 0.023 on strong GPUs and the Steam Deck
    ([modding-openmw performance tips](https://modding-openmw.com/tips/performance/)).
- **Small-feature culling.** On by default, with `small feature culling pixel size` 2: anything that
  covers about 2 px or less on screen is skipped. This is a camera-cull rule and works on top of
  paging's size rule.
- **Terrain.**
  - The terrain is a quadtree of chunks with distance LOD. Settings: `lod factor` 1.0 and
    `vertex lod mod` 0 (each step doubles or halves the vertex count).
  - Far chunks use a pre-blended "composite map" texture (512 px) instead of per-layer splatting.
    `max composite geometry size` 4 cells sets the chunk size; the docs note the trade-off:
    "smaller = more draw calls, larger = more overdraw".
  - The docs warn that fewer chunks render faster but cause rebuild hitches when you move fast.
  - `water culling` skips water that lies under visible terrain chunks.
- **Content-side help.** Community mods matter as much as engine work:
  - Morrowind Optimization Patch cleans meshes and collapses nodes.
  - Project Atlas re-UVs the "highest-strain" town meshes onto shared texture atlases. Its reason:
    "each shape, even within one mesh file, requires its own draw call", and some vanilla meshes
    have more than 75 shapes ([Project Atlas](https://github.com/MelchiorDahrk/Project-Atlas),
    [atlased meshes tip](https://modding-openmw.com/tips/atlased-meshes)).
- **Draw calls in a Balmora view.** OpenMW publishes no number. Godotwind measured its own engine
  before its shadow fixes: Balmora "dense" took about 3,580 draws, of which about 2,770 were shadow
  passes, and Seyda Neen about 1,560 to 2,200 draws
  (`D:\Gamedev\Godotwind\godotwind\docs\archive\plans\shadow_pass_optimization_2026_04_22.md`). That
  is the right order of magnitude for an unmerged Morrowind town.

### For us (engine-neutral)

1. **Our cost is per entity, not per triangle. Cut entity count inside the active grid, not only in
   the far field.** OSG pays per scene-graph node in cull and draw. Bevy pays per mesh entity in
   extract, visibility, queue and batching. That is the same failure, and OpenMW fixed it with
   active-grid merging. Bake-time merging of immovable statics (no script, no collision change, not
   pickable) into per-cell or per-chunk meshes, grouped by material, is the matching lever. Our own
   numbers show the far field already "makes towns affordable" (37 ms without it). The near field is
   where the 46-52 k entities live.
2. **Merge only when it pays.** OpenMW weighs merge cost against benefit and does not merge
   everything. Rough rules:
   - Merge many small, cheap, same-material pieces.
   - Leave large unique meshes alone.
   - Keep anything interactive as its own entity.
   - Keep merged chunks spatially compact so frustum culling still works.
3. **Use one apparent-size rule per tier.** That means size divided by distance, with OpenMW's 0.01
   as a sane default for the far field, plus a 2 px screen-size cull near the camera. It is unit-free:
   do not "convert" it to metres (Godotwind got this wrong, see §4).
4. **Atlas or array the textures of the worst town meshes at bake time.** Material count, not mesh
   count, limits how far merging can go.
5. **Merged chunks need a lighting path without per-object light limits.** Bevy's clustered forward
   lighting already has none (see §3).

---

## 2. Actors

### What OpenMW does

- **`actors processing range`**: default 7168 units (about 102 m), allowed range 3584 to 7168, in the
  in-game Prefs menu. Inside it, the actor's "AI, animations, and physics" are updated. Actors near
  the boundary fade in and out instead of popping. Actors on Travel AI packages always stay active,
  so patrols and caravans keep working. Vanilla Morrowind had the same idea as its "AI distance"
  slider. Source: [game settings](https://openmw.readthedocs.io/en/latest/reference/modding/settings/game.html).
- **Animation and skinning.** From public commit titles and headers (behaviour only):
  - OpenMW skins on the CPU, into double-buffered vertex data so the draw thread can render the
    previous result while the next one is computed.
  - Bone matrices are computed during the cull traversal, so only actors that some camera actually
    keeps are re-skinned. Off-screen actors are not skinned ("Don't update off-screen animations").
  - Actors beyond the processing distance have their animation controller deactivated and their
    skinning updates stopped after the first frame ("Disable skinning updates for actors beyond the
    AI processing distance").
  - Sources: commit mirror at git.cijber.net (searches for riggeometry, skeleton). We found no GPU
    skinning in OpenMW.
- **Physics.** Actor movement runs on a background thread (`async num threads` 1, default since
  0.48). Line-of-sight queries can be cached across frames. Physics outside the processing range is
  not stepped. Source: [physics settings](https://openmw.readthedocs.io/en/latest/reference/modding/settings/physics.html).
- **Shadows from actors** are off by default (§3).
- **How many actors animate in Balmora.** No published number. By the rules above, it is every actor
  within about 102 m (usually a few dozen in Balmora). Only the visible subset is skinned each frame.
  The community stress test is "`placeatpc` 20 imperial guards in the Balmora Mages Guild"
  ([performance tips](https://modding-openmw.com/tips/performance/)).

### For us

1. **Use two independent gates.**
   - Processing range (about 100 m, with Morrowind's 51-102 m as the user range) gates AI, animation
     graph and physics.
   - Visibility gates the pose evaluation and skinning: if no view (main camera or a shadow cascade
     that draws actors) needs the actor this frame, skip it.
2. **Freeze actors at the edge of the processing range, don't despawn them.** Keep the last pose,
   fade at the boundary, and exempt actors that must keep moving (travel, follow).
3. **Add update-rate LOD on top** (industry standard; OpenMW does not have it): full rate near the
   camera, then 30, 15 and 5 Hz with distance. Godotwind's tiers are in §4.
4. **Actors cast shadows only near the camera.** Leave them out of the far cascades (§3).
5. **Skinned bounds:** compute them per actor from a padded bind-pose box. Per-bone bounds every
   frame cost more than they save.

---

## 3. Shadows and lights

### OpenMW shadow defaults ([shadow settings](https://openmw.readthedocs.io/en/latest/reference/modding/settings/shadows.html))

| setting | default | notes from the docs |
|---|---|---|
| enable shadows | off | "may reduce performance" |
| number of shadow maps | 3 (1-8) | |
| maximum shadow map distance | 8192 units (about 117 m) | 0 means no limit |
| shadow fade start | 0.9 | fraction of the max distance |
| shadow map resolution | 1024 | "increase GPU load" |
| actor / player / terrain / object shadows | all off | each "may reduce performance" |
| enable indoor shadows | off | only actors cast shadows indoors (no ceiling shadows), and shadows can bleed through floors |
| compute scene bounds | bounds | "primitives" is better quality at more CPU, "none" turns it off |
| allow shadow map overlap | on | overlapping cascades (CSM). Off gives non-overlapping splits (PSSM) |
| split point uniform logarithmic ratio | 0.5 | |
| minimum lispsm near far ratio | 0.25 | LiSPSM warping per map |
| use front face culling | off | performance gain, more peter-panning |
| polygon offset factor / units, normal offset | 1.1 / 4.0, 1.0 | |

- **Community preset** ([modding-openmw shadows](https://modding-openmw.com/mods/shadows/)): 4 maps
  ("only a little FPS"), max distance 4096 units (about 58 m, half the default, "improves FPS"),
  resolution 4096 (drop to 2048 or 1024 if needed), all four caster types on, indoor shadows off.
  Its advice: if FPS is short, turning shadows off beats bad shadows. The performance tips page also
  says to halve the distance to 4096.
- **Design reading.**
  - OpenMW ships shadows off. With shadows on, every caster class is off until the user enables it.
  - The shadow range is short (about 58-117 m), even with a far view distance.
  - Beyond the range, objects cast no real-time shadows at all. Distant land is never in a cascade.
  - Statics are mostly drawn through merged paging chunks, so the caster count per cascade stays low.
- **Godotwind's measurements** (Godot, 4-split PSSM, 200 m max distance):
  - Shadow passes were 69-77 % of all draws, and each mesh drew into 2.5-4 cascades.
  - Turning off shadow casting for small objects by apparent size and distance, decided per
    batch from its nearest instance with 5 m hysteresis, cut shadow draws by about 36-40 %. In the
    worst near-only scene it raised FPS from 21 to 138.
  - Its caster cutoff: distance = object size x 200, clamped to 40-200 m
    (`...\archive\plans\shadow_pass_optimization_2026_04_22.md`).

### OpenMW lights

- **0.47 removed the fixed-function 8-lights-per-object limit**
  ([MR !618 title](https://gitlab.com/OpenMW/openmw/-/merge_requests/618)). `lighting method` offers
  "shaders compatibility" (the default) and "shaders" (modern GPUs, more lights).
- **Per-object light assignment:**
  - `max lights` is 8 (2-64) per object ([stable shader settings](https://openmw.readthedocs.io/en/stable/reference/modding/settings/shaders.html)).
  - `light bounds multiplier` is 1.65. In the latest docs this setting is `light radius multiplier`,
    1.75.
  - `maximum light distance` is 8192 units, and `light fade start` is 0.85.
  - Lights are assigned to each object by overlap, CPU-side, during culling. Lights fade out past the
    distance limit.
- **Latest builds add `clustered lighting`** (default off). It assigns lights to screen-space
  clusters instead of to objects, which removes per-object limits; the docs recommend it together
  with per-pixel lighting ([latest shader settings](https://openmw.readthedocs.io/en/latest/reference/modding/settings/shaders.html)).
- **Particles:** `particle point lighting` can be turned off "in particle-dense scenes".

### For us

1. **Each caster class gets its own distance cap.** The shadow distance is far shorter than the
   view distance (OpenMW: 58-117 m). Actors and small clutter cast only in the near cascades.
   Terrain and the far field cast from cheap proxies or not at all.
2. **Cull casters per cascade by apparent size.** A 2 m rock does not belong in a cascade that covers
   80 m texels. Godotwind's evidence (-36 % shadow draws) is stronger than our first caster rule
   (-8 % entries). Make the rule depend on cascade texel size and distance, not only on mesh size.
3. **Fewer, cheaper cascades.** OpenMW defaults to 3 maps. 4 is the enthusiast preset. Combine this
   with caching static casters in the far cascades and refreshing them at a lower rate: that matches
   our perf.md plan, and it is standard practice (Unreal's cached shadows, Frostbite).
4. **Draw shadow casters from the merged chunks.** That is the biggest structural win, because each
   chunk entity replaces dozens of caster entries in every cascade.
5. **Lights.** We already use clustered forward lighting, which is the endpoint OpenMW is moving to.
   Our measurement (lights about free) agrees. Keep a light distance fade (OpenMW: 0.85 of max) for
   the visual cut-off. Per-object limits are not a thing to copy.
6. **Interiors.** OpenMW's default (no indoor sun shadows) says sun shadows indoors are not worth
   their cost.

---

## 4. Godotwind (anecdote: Godot's renderer is not Bevy's)

Sources: `D:\Gamedev\Godotwind\godotwind\docs\` (`systems/distance_rendering.md`,
`audit/distant_rendering_perf_audit_2026_07_05.md`,
`reference/hlod_openmw_industry_comparison_2026_05_02_hlod.md`,
`archive/plans/shadow_pass_optimization_2026_04_22.md`) and
`src/core/animation/animation_lod_controller.gd` (MIT; behaviour only).

**Its tiers:**

| tier | range | content |
|---|---|---|
| near gameplay | 0-150 m | nodes, physics, interactive objects |
| static visuals | 150-400 m | cell-local instanced draw groups, engine mesh LOD |
| baked chunk proxy | 400-1200 m | offline per-cell merged and simplified mesh |
| impostors | 1200-5000 m | octahedral, in spatial pages |

Promotion to near happens at 250 m and demotion at 280 m (20 m hysteresis).

**What it hit and how it reacted:**

1. **Shadow passes were most of the draws** (69-77 %). Fixed by the size and distance rule from §3.
   A flat "under 1 m tall" rule was rejected because it misjudges close objects. A pure visibility
   range rule was rejected because it emptied the scene near the player.
2. **Runtime merge without simplification ("HLOD") was a net loss.** FPS went from 57 to 44, with
   861 ms stalls and a crash class. The verdict was to replace it with an offline bake per cell,
   merged and simplified with LODs, that the runtime only loads and publishes, then delete the
   runtime merger.
3. **Instancing did not pay for Morrowind content.** Groups averaged 2.3 instances per draw, so the
   about 3,000 lines of clustering machinery bought almost nothing. Morrowind cells have too much
   variety for automatic instancing. Merging works; instancing only suits grass and repeated
   clutter.
4. **Halving draw calls (649 to 328) did not move FPS** in Godot (34.2 to 34.4). Godot's Vulkan
   back end is not draw-bound there. The bottleneck was world assembly and teardown on the main
   thread during movement: per-frame budgets overran 2-30x because single work items took tens of
   ms. Its rules: make streaming work preemptible at fine grain, or move it off-thread, and prebake
   per-cell payloads. This matches our 25 ms walking against 14-19 ms standing, so check streaming
   spikes, not only the render thread.
5. **It got OpenMW's constant wrong.** A misremembered "0.14" was "corrected" to 0.003, about 3x more
   permissive than OpenMW's real 0.01, so far too many small objects survived. Lesson: take constants
   from the docs, and remember the ratio has no units.
6. **Animation LOD** by distance, with frustum culling done first:
   - IK only within 5 m (30 Hz);
   - simple blending up to 60 m (15 Hz);
   - a single clip at low rate up to 150 m (5 Hz);
   - beyond 150 m, or off-screen, no update at all.

   The frustum check uses chest height, not feet, so actors are not culled when the camera looks up.
   An "importance" bias lets key NPCs keep a higher tier.
7. **Measure the split before optimising.** Its audit found that six benchmark harnesses never
   captured the render CPU/GPU split. We already measure it.

**Engine-independent ideas worth keeping:**

- bake merging offline;
- cull shadows by apparent size per batch, with hysteresis;
- animation LOD tiers plus off-screen freeze;
- hysteresis at every tier boundary;
- preemptible streaming work;
- don't build instancing machinery for content that doesn't repeat.

---

## 5. MGE XE (vanilla engine add-on)

Sources: [MGE XE releases](https://github.com/Greatness7/MGE-XE/releases), [Nexus page](https://www.nexusmods.com/morrowind/mods/41102).

- **Distant land is generated offline from the user's mod list** by a wizard. Objects below a
  minimum static size are left out. Guides suggested a low minimum size when statics cast shadows,
  because the shadows come from that distant data.
- **v0.20:**
  - Three triangle budgets per object by distance tier.
  - Horizon culling for statics and terrain, on by default.
  - Each cell's distant geometry merged into one mesh, so a cell takes "one or two draw calls rather
    than one per object".
  - Incremental rebuilds (only dirty cells).
  - Native per-pixel lighting, with an opt-in limit of 32 lights per object.
  - Real depth-buffer capture instead of redrawing the scene.
  - Uploads split into frame-budgeted chunks.
- **v0.21:** distant statics stream in and out instead of all loading at startup, using 23 % less
  VRAM and disk.
- **Shadows** are drawn from the simplified distant-land data, which is why they also show under
  water.

### For us

- A per-cell merged mesh with tiered triangle budgets and incremental bakes is the same answer
  OpenMW and Godotwind reached. Our far field already does this. The step that is missing is the
  same treatment for the active grid.
- Horizon (terrain) culling is a cheap occlusion win for statics behind hills.
- Use simplified proxies as shadow casters at mid range.

---

## 6. Summary: rules to try, in likely order of payoff for our numbers

1. **Merge immovable statics inside the active grid at bake time**, per cell or chunk, by material,
   keeping interactive objects separate. OpenMW's "active grid paging" is its single biggest fix for
   CPU-bound towns, and it goes straight at our 46-52 k entities.
2. **Shadow casters by class and apparent size:** a short shadow range (60-120 m), actors and clutter
   only in the near cascades, casters drawn from merged chunks, far cascades cached or updated at a
   lower rate. OpenMW ships every caster class off by default; Godotwind cut shadow draws by 36-40 %
   with the size rule.
3. **Actor gates:** processing range about 100 m with a fade, no pose or skinning when no view needs
   the actor, update-rate LOD (30/15/5 Hz), actors left out of far cascades.
4. **Apparent-size culling everywhere:** a 2 px rule on screen, plus the unit-free 0.01 size/distance
   rule for paged content.
5. **Texture atlases or arrays for the worst town meshes**, so merged chunks really collapse into
   few draws (Project Atlas).
6. **Check the walking cost separately:** streaming work must be preemptible and prebaked
   (Godotwind's main lesson). Our walk (25 ms) is far worse than standing still (14-19 ms).
