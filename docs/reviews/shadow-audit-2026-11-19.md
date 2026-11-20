Here is the audit of our shadow pipeline, stock Bevy 0.20, and Thomas Portal's Clockwork renderer (verified live
  against GitLab/GitHub APIs: gitlab.com/OursCodeur/openmw branch clockwork-alpha is still at c5e5a9a6 from 2026-10-
  19; Thomas Portal's November activity has been upstream OpenMW MRs, not new clockwork-alpha pushes).
  ──────
  Executive Verdict: Are we doing things correctly?

  • Sun / directional shadows: Yes. Our sun pipeline (PSSM cascades + F3 far-cascade hold + Clockwork-adapted world-
  unit PCSS + terrain far shadow map + contact shadows + sun/moon contrast gate) is modern, fast, and aligned with
  industry practice.
  • Local / point-light shadows: No — we are fighting Bevy's architecture. The character flickering, the "only close
  lamps cast shadows indoors" look, and the pile-up of workarounds all come from one root problem: Bevy treats every
  point-light cube face as an independent ECS root view + render schedule + command encoder, cleared and redrawn whole
  every frame.
  • Fact-check on Clockwork's "unlimited shadows" claim: Clockwork does not have unlimited shadows. It has a VRAM-
  scaled cube cap of 8 to 61 lights (2 cubes per GiB of VRAM: 47 on an RTX 4090, 16 on an 8 GB RTX 4060 Laptop) plus 6
  actors × 2 lights = 12 dedicated actor shadow slots. It feels unlimited in Morrowind because interiors have 20–30
  lights and Balmora has ~53, and it runs fast because of 4 structural design choices we don't have yet.
  ──────
  1. Bevy 0.20's "Natural" (Stock) Shadow Architecture

  Checked directly in stock Bevy 0.20 (bevy_pbr-0.20.0, bevy_light-0.20.0, bevy_render-0.20.0, summarized in
  docs/wiki/bevy/shadows-and-lights.md):

  1. Every shadowed PointLight spawns 6 full camera-like root views: prepare_lights attaches RootNonCameraView(Core3d)
  to every cube face (~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_pbr-0.20.0/src/render/light.rs:2239).
  Stock Bevy runs the entire Core3d schedule per face (upstream issue #25576, still open).
  2. Zero caching, cleared every frame: Depth targets come from TextureCache as one cube-array of count × 6 layers
  (~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_pbr-0.20.0/src/render/light.rs:1490), hard-capped at
  max_texture_array_layers / 6 (42 cubes on standard 256-layer wgpu). Every face clears and redraws every caster every
  frame.
  3. One global resolution, no atlas, no budget, no fade: Every point light shares PointLightShadowMap::size (default
  1024²). Turning shadows off even leaks the 6 face views unless worked around (Bevy #26059, our
  drop_stale_light_views).
  4. Main-thread CPU visibility scan per light: bevy_light::check_point_light_mesh_visibility tests every CPU-culled
  mesh against all 6 faces of every shadowed light every frame.
  5. Sun cascades: Max 4 cascades (MAX_CASCADES_PER_LIGHT = 4), sphere-fitted + texel-snapped, rendered inside the
  camera's per_view_shadow_pass (much cheaper than point lights because they aren't root views). Native ContactShadows
  exists; experimental PCSS is in cascade texels (penumbra jumps between cascades).

  Bottom line: Stock Bevy assumes 1–3 shadowed point lights in a small scene. At Morrowind density (20–30 overlapping
  lamps indoors, ~300 in Balmora), stock Bevy is completely unusable without heavy patching.
  ──────
  2. Our Current Implementation — And Why It Flickers / Costs Too Much

  Checked against live code (see docs/wiki/engine/shadows.md,
  bevyrrowind/crates/mw_environment_bevy/src/render.rs:26-31, superfargoth/crates/sf_sky_bevy/src/shadow_budget.rs,
  and vendored superfargoth/vendor/bevy_pbr/src/render/light.rs):

  What is actually running by default right now

  • Budget: High = 8 lamps at 512² (Medium 6, Low 0, Ultra 12 at 1024²), ranked 360° around the camera with hysteresis
  and 35–45 m distance fade (superfargoth/crates/sf_sky_bevy/src/shadow_budget.rs:335-373).
  • Caching: No static/dynamic caster split is active by default.
      • The legacy cache (sf_shadow_cache_bevy) has been disabled since 2026-10-21 (POINT_SHADOW_CACHE = false at
      bevyrrowind/crates/mw_environment_bevy/src/render.rs:31).
      • What runs is our own vendored fork of closed Bevy PRs #26149 + #26161: slim Core3dShadows schedule +
      skip_unchanged_shadow_views (whole-face hash skip).
      • The new CW-4 static/mover lamp cache (superfargoth/crates/sf_sky_bevy/src/lamp_cache.rs +
      superfargoth/vendor/bevy_pbr/src/render/point_shadow_cache.rs) is built, off by default, and regressed render
      thread time by +0.94 ms in the 2026-11-17 A/B.
  • Active mitigations (all on by default): day contrast gate (CW-3a), per-face frustum culling (CW-3b, -0.46 ms night
  walk), 1-tap 2×2 hardware PCF + early-out (Reset B2), NoFrustumCulling pinning for off-screen holders, and lamp
  contact shadows on the 8 budgeted lamps.

  Root causes of the exact problems you're seeing

  1. Why characters flicker when passing by lamps (SH-21a):
      • Because #26161 has no static/dynamic split, any moving NPC or player within 16 m dirties every lamp face it
      touches, forcing a full redraw of all statics + movers in that face.
      • To avoid tanking the framerate, we cap mover redraws at MAX_MOVER_LIGHT_REFRESHES_PER_FRAME = 2 lights (+1
      LRU) (superfargoth/vendor/bevy_pbr/src/render/light.rs:1353,
      superfargoth/vendor/bevy_pbr/src/render/light.rs:1880).
      • Walking past 4–8 lamps means each lamp's cubemap updates on a different frame (every 2–4 frames). The
      character is shaded against stale silhouettes of its own previous pose from multiple out-of-phase cubes → harsh
      flickering/strobing.
      • Carried torches redraw all 6 faces whole every frame (uncapped,
      superfargoth/vendor/bevy_pbr/src/render/light.rs:1863-1866), eating render budget.
  2. Why shadows only appear close to lamps in crowded interiors:
      • Indoors, it isn't even the 35–45 m fade — it's the 8-slot cap against 20–30 lamps in one interior. importance
      ranks heavily by 1/d² (and 1/d⁴ outside range), so the 8 nearest lamps win and every other lamp in the room is
      completely unshadowed and leaks through walls/pillars until you walk near it.
      • Unshadowed lamps also get no contact shadows (PointShadowBudget::contact_shadows only enables them on the 8
      holders).
  3. Why our CW-4 static/mover lamp cache was slower (+0.94 ms render, +2.8 ms p99) in the 2026-11-17 A/B:
      • Looking directly at the benchmark gauges (bevyrrowind/benchmarks/runs/pt_night/summary.md, base-1.md,
      lampcache-1.md):


   Metric (balmora_walk_night, 1440p DLSS Q) │                   base │              facecull │      lampcache (CW-4)
  ───────────────────────────────────────────┼────────────────────────┼───────────────────────┼───────────────────────
   Frame median / p99                        │       10.69 / 16.38 ms │      10.23 / 15.58 ms │      10.83 / 19.15 ms
   Render busy / Render graph                │         9.18 / 5.25 ms │        8.66 / 4.81 ms │       10.12 / 6.20 ms
   Total views (views/all/count, median)     │                     51 │                    51 │                     5
   Stutters/min (run #1)                     │            29.25 / min │                     — │           12.80 / min
   Drawn lamp faces / frame (median)         │                      0 │                     0 │            9 (max 14)
   encode//sum (compute/aux encoders)        │                4.22 ms │                     — │    7.57 ms (+3.35 ms)
   encode/point_shadow_cache_copies/sum      │                      0 │                     0 │              0.042 ms
   auxiliary_views GPU / CPU                 │         0.26 / 0.24 ms │                     — │        0.50 / 0.46 ms
   preprocess bind groups created/frame      │                     13 │                    11 │                    21

  • The smoking gun: The cache worked (views dropped 51 → 5, stutters dropped by 56%, and the depth copy itself cost
  only 0.042 ms!).
  • What hurt it was lifting the 2-light cap so walkers wouldn't flicker: lamp_cache/movers was 95 median (max 263),
  causing 9 mover faces to draw EVERY frame. Because each drawn face in Bevy is still a separate
  RootNonCameraView(Core3dShadows) with its own GPU preprocess compute encoder, render-pass encoder, and queue.submit
  command buffer (plus bind-group churn as views spawn/despawn), drawing 9 tiny mover passes added +3.35 thread-ms in
  encode//sum and +0.96 ms to the render graph.
  ──────
  3. How Clockwork Renderer Actually Does It (clockwork-alpha @ c5e5a9a6)

  Read directly from C:/cargo-target/refs/openmw-clockwork/
  (C:/cargo-target/refs/openmw-clockwork/docs/source/reference/modding/settings/shadows.rst,
  C:/cargo-target/refs/openmw-clockwork/components/sceneutil/worldshadows.cpp,
  C:/cargo-target/refs/openmw-clockwork/components/sceneutil/localshadows.cpp,
  C:/cargo-target/refs/openmw-clockwork/apps/openmw/mwrender/clockwork/frame/framedriver.cpp,
  C:/cargo-target/refs/openmw-clockwork/files/shaders/compatibility/gpuscene_worldsoft.comp):

  The 4 structural tricks behind Clockwork's speed and stability

  1. It decouples static room shadows from character shadows completely (the #1 reason characters never flicker):
      • System A — WorldShadowManager (static world cubes, 8–61 lights):
          • Holds 2 cubes per GiB of dedicated VRAM (16 cubes on an 8 GB RTX 4060, 47 on a 24 GB 4090, cap 61 + 2
          handover spares, C:/cargo-target/refs/openmw-clockwork/components/sceneutil/worldshadows.cpp:72-126).
          • Actors are NOT drawn into world shadow cubes. Static cubes are captured once (2 faces/frame budget) and
          only redrawn when a static caster moves > 1 texel
          (C:/cargo-target/refs/openmw-clockwork/components/sceneutil/worldshadows.cpp:583-594).
          • Moving world props (banners, silt striders) draw over a cached static copy at 30 Hz, strictly capped at ≤
          6–8 face draws per frame total (sViews = 8,
          C:/cargo-target/refs/openmw-clockwork/components/sceneutil/worldshadows.hpp:139).
          • Moving lights (player torch) get a separate 1024² fine atlas (1–4 slots), and at most 1 moving cube draws
          per frame (queueMoving, C:/cargo-target/refs/openmw-clockwork/components/sceneutil/worldshadows.cpp:917).
          • Empty faces are flagged in a bitmask (mOpen) and never drawn and never sampled
          (C:/cargo-target/refs/openmw-clockwork/files/shaders/lib/light/worldshadows.glsl:124). Lights buried inside
          geometry are skipped (C:/cargo-target/refs/openmw-clockwork/components/sceneutil/worldshadows.cpp:321-384).
      • System B — LocalShadowManager (actors ONLY, 100% per-frame, zero flicker):
          • A single 2D shadow atlas (localShadowDepth, 512² tiles, max 18 slots = 6 actors × 2 strongest lights +
          handover spares, C:/cargo-target/refs/openmw-clockwork/components/sceneutil/localshadows.hpp:43-58).
          • Each slot aims a tight perspective frustum strictly at ONE actor's bounding box from ONE light, drawing
          only that actor every frame — NOT a 6-face room cubemap!
          • Because the light already has a world shadow cube, Clockwork skips the actor's static blocker pass
          (C:/cargo-target/refs/openmw-clockwork/docs/source/reference/modding/settings/shadows.rst:355).
          • Actors beyond the 6-slot cap fall back to 3-capsule ambient occlusion shadows (capsules.glsl, 0.04 ms for
          400 actors).
          • Result: Actors have crisp 512² silhouettes, update at full framerate with zero flicker, and walking past
          10 lamps never triggers a single cubemap redraw.

  2. Zero per-lamp CPU/view overhead (single Vulkan command buffer, fixed 8-view pool):
      • Holding 47 cached cubes does not create 282 ECS views or 282 command encoders.
      • WorldShadowManager reuses a fixed pool of 8 face cameras total (sViews = 8,
      C:/cargo-target/refs/openmw-clockwork/components/sceneutil/worldshadows.cpp:1222-1241). A clean cached cube is
      literally just a 36-byte Record in an SSBO — 0 views, 0 CPU traversal, 0 encoders.
      • All queued faces (≤ 8/frame) are recorded into one Vulkan command buffer with one barrier before/after
      (C:/cargo-target/refs/openmw-clockwork/apps/openmw/mwrender/clockwork/frame/framedriver.cpp:3872-4010), driven
      by GPU indirect culling (GpuScene).
  3. Half-res screen-space shadow filtering instead of full-res forward shader sampling:
      • Rather than doing multi-tap PCF/PCSS inside the full-res forward PBR pass (which forced us down to 1-tap 2×2
      PCF because full-res sampling cost ~0.5 ms per shadowed lamp), Clockwork runs a half-res compute pass
      (C:/cargo-target/refs/openmw-clockwork/files/shaders/compatibility/gpuscene_worldsoft.comp) that evaluates area-
      light PCSS for the top 4 shadowed lights per pixel + bilateral blur, and packs them into a screen-space mask
      read by the forward pass (C:/cargo-target/refs/openmw-clockwork/files/shaders/lib/light/worldshadows.glsl:43).
      • Actor shadow slots similarly evaluate in a dedicated compute pass
      (C:/cargo-target/refs/openmw-clockwork/files/shaders/compatibility/gpuscene_slotshade.comp) before main shading.
  4. Better light selection & visual polish:
      • Ranks by reach² / (d + reach)² * sqrt(luminance)
      (C:/cargo-target/refs/openmw-clockwork/components/sceneutil/worldshadows.cpp:359) — bright braziers beat dim
      candles.
      • Keeps 2 spare cubes so outgoing and incoming lights crossfade simultaneously over 0.75 s
      (C:/cargo-target/refs/openmw-clockwork/components/sceneutil/worldshadows.cpp:1345-1373) instead of waiting for a
      slot.
      • Adds light-fixture self-shadowing (brazier bowls/sconces darken underneath) and local shadow strength (deepens
      the dominant light's shadow so overlapping fill lights don't wash it out).


  │ Caveat on Clockwork's published numbers: Its quoted GPU costs (0.06–0.24 ms for 8 cubes, +0.13 ms for 47 cubes,
  │ C:/cargo-target/refs/openmw-clockwork/docs/source/reference/modding/settings/shadows.rst:351-354) are at 1080p
  with
  │ 256 px faces on an RTX 4090, on a classic Blinn-Phong MSAA renderer (no PBR, no TAA/DLSS). On an RTX 4060 Laptop
  at
  │ 1440p with 512 px faces, expect ~3–4× that GPU time — still very cheap (~0.5–1.0 ms GPU) and near-zero CPU.
  ──────
  4. Side-by-Side Comparison

   Dimension                  │ Stock Bevy 0.20            │ Scrib Today (Default)      │ Clockwork (c5e5a9a6)
  ────────────────────────────┼────────────────────────────┼────────────────────────────┼─────────────────────────────
   Sun shadows                │ 4 CSM, texel PCSS, contact │ 3–4 PSSM CSM, F3 hold,     │ 3 CSM, world-unit PCSS, far
                              │ shadows                    │ world-unit PCSS, contact,  │ terrain horizon + 4096²
                              │                            │ far terrain map, contrast  │ sliced far object map
                              │ cap 42)                    │ 1024² (Ultra)              │ 4090), +1–4 moving at 1024²
   Static lamp caching        │ None (redraws every frame) │ Whole-face skip only       │ True static cache; 0
                              │                            │ (#26161 fork); CW-4        │ views/CPU when clean; empty
                              │                            │ static/mover cache built   │ faces skipped
                              │                            │ but off                    │
   Actor shadows from lamps   │ Redrawn inside 6-face      │ Redraws whole room cube,   │ Dedicated 2D actor atlas (6
                              │ cubes                      │ capped at 2 lights/frame → │ actors × 2 lights), drawn
                              │                            │ flickers                   │ every frame, zero flicker
   CPU cost of N cached lamps │ O(6N) full Core3d views +  │ O(6N) views + CPU caster   │ O(1) (fixed pool of ≤ 8
                              │ scans                      │ scans + hashes             │ face cameras total/frame;
                              │                            │                            │ single Vulkan cmd buffer)
   Main-pass lamp filter      │ Full-res                   │ Full-res 1-tap 2×2 PCF +   │ Half-res compute soft
                              │ Gaussian/Temporal/PCSS     │ early-out + full-res       │ shadow pass + bilateral
                              │                            │ contact shadows on 8 lamps │ blur + dedicated slot
                              │                            │                            │ compute pass
  ──────
  5. Audit Conclusions & Recommended Direction

  We are not doing anything "wrong" in our sun pipeline or our culling/gating patches (CW-3a, CW-3b, F3, Reset B2,
  bind_group_walk_once are all sound).

  Where our architecture is hitting a wall is trying to make Bevy's 1-view-per-cube-face model handle both (a) 20+
  static room lamps and (b) every-frame character shadows simultaneously. Every patch we stack on top of #26161 fights
  that same wall.

  If you want Clockwork-like results (many shadowed interior lamps, zero character flicker, low CPU cost) inside Bevy
  0.20 without rewriting the whole renderer:

  1. Use CW-4's already-built zero-view mechanism for static-only lamp cubes, and raise the interior/lamp cube budget
  from 8 → 16–24 (up to Bevy's 42-cube limit).
      • pt_night already proved CW-4 drops clean lamp views to 0 (views/all/count: 51 → 5).
      • Once static cubes never redraw for walking characters (LampTier::StaticOnly already exists in
      sf_sky::lamp_cache), holding 16–24 static cubes at 512² (96–144 MiB VRAM; or 256² for smaller/secondary lamps)
      costs nearly 0 CPU after initial capture and covers almost every Morrowind interior room — ending the "shadows
      only appear when close" artifact.
  2. Decouple character/mover shadows from room cubemaps (fixes flickering and the CW-4 +0.94 ms regression):
      • Best architectural match (Clockwork's LocalShadowManager): A dedicated single-encoder 2D shadow atlas for the
      nearest 6 actors × 2 lights, drawing only those actor meshes every frame. Zero flicker, sharper actor shadows
      than a 512² room cube, and 1 command encoder total instead of 9–12 Core3dShadows root views.
      • Alternative (fix CW-4 in place): Before abandoning mover overlays, check why lamp_cache/movers was 95 median
      on Balmora walk (likely non-character entities or too-far movers inflating mover faces), throttle non-actor
      world movers to 30 Hz capped at ≤ 4–6 faces/frame (like Clockwork), and collapse drawn mover faces into one
      shared encoder / pass (CW-13) instead of N RootNonCameraView schedules.
  3. Low-effort high-impact quality ports from Clockwork:
      • Include sqrt(luminance) in shadow_importance so bright braziers/fires outrank dim candles.
      • Skip sampling & drawing empty cube faces via an open_faces bitmask.
      • Add 1–2 spare cube slots so lamp handovers crossfade simultaneously instead of waiting for eviction.
      • Consider screen-space contact/Bend shadows (CW-16) for unshadowed lights beyond the cube budget instead of
      paying contact shadows only on the lamps that already have cube maps.


---

## 6. Follow-Up Audit: Upstream Bevy Status & Industry Landscape (2026-11-19)

### Live check on `bevyengine/bevy` (GitHub API, 2026-11-19)

No upstream Bevy PR solves raster local-light shadows at scale:

- **Shadow atlas / per-light resolution:** None (no open PR, no RFC).
- **Virtual Shadow Maps (VSM):** None.
- **Shadow map caching (static/dynamic split):** None. The only two PRs in that direction (`#26149` slim shadow schedule, `#26161` skip unchanged shadow maps) were self-closed unmerged on 2026-10-30 (we already maintain them as an own fork in `superfargoth/vendor/`).
- **Per-face `Core3d` overhead (`#25576`):** Still open with no active PR.
- **Hidden point-light view leak (`#26059`):** Open PR `#26480` (2026-11-10) fixes it (already worked around by our `drop_stale_light_views`).
- **Upstream direction:** PR `#25913` (merged 2026-10-05) added `PointLight`/`SpotLight`/`RectLight` support to **Bevy Solari** (hardware ray tracing). Upstream's answer to many shadowed lights is RT, not a raster atlas — which **cannot** be our primary path because Solari does not support transparent/cutout parity or skinned meshes cleanly yet, requires high-end RT hardware, and cannot hit our raster 120 fps / 8.3 ms 4060 Laptop target.

### Unbiased industry comparison (raster & hybrid)

Clockwork's `LocalShadowManager` (per-actor 2D frusta + physics raycasts + floor lattices + separate world mover overlays + ambient capsules) is **not** state of the art — it is an OSG-era workaround that maintains three overlapping local-shadow systems with brittle edge cases (stairs, multi-story leaks, physics props/doors vs actors).

| Approach | Who ships it | Pros | Cons / Fit for Bevy 0.20 |
|---|---|---|---|
| **1. Hardware RT / ReSTIR** | UE5 MegaLights, Cyberpunk Overdrive, Bevy Solari | Unlimited shadowed lights, zero flicker, zero maps. | High-end RT hardware only; missing skinned/transparent parity in Bevy; cannot replace raster. |
| **2. Virtual Shadow Maps (VSM)** | UE5 | Uniform screen-space texel density, page-level static/dynamic cache. | Needs GPU page-marking + meshlet/cluster culling; Bevy meshlets lack skinning/alpha/terrain parity. XL multi-month renderer rewrite. |
| **3. Unified Cached 2D Shadow Atlas + Single-Pass Draw** *(recommended)* | **Doom 2016/Eternal (id Tech 6/7), Unity HDRP, Godot 4, UE4/5 Cached Whole-Scene Shadows, Cyberpunk 2077** | **One unified system** for static rooms, characters, doors, and physics props. Variable tile sizes (512²/256²/128²) fit **40–60+ shadowed lamps** in ~80 MiB. Static depth cached; active movers blit + draw in **one single render pass / encoder** (zero flicker, no `RootNonCameraView` tax). | Replaces Bevy's cube-array shadow pass + sampling in vendored `bevy_pbr`. Reuses our existing CW-4 `sf_sky::lamp_cache` tile state machine cleanly. |
| **4. Clockwork Hybrid** | OpenMW Clockwork | Fits Morrowind's small-cast interiors well. | 3 separate shadow systems, capped at 6 actors × 2 lights, brittle stair/floor/blocker heuristics, doesn't cover physics/doors cleanly. |
| **5. Screen-Space Shadows (Bend SSS)** *(complement)* | Days Gone (SIGGRAPH 2023), Crimson Desert, UE5 | 1 compute pass, zero views, works on any light. | Only knows on-screen depth (cannot stop light leaking through walls from off-screen/occluded lamps). Best as a complement to #3. |

---

## 7. Deep Dive: Unified Cached 2D Shadow Atlas for Bevy 0.20

**Recommendation:** Yes — build a **Unified Cached 2D Shadow Atlas with Single-Pass Batched Rendering** (the Doom 2016 / Unity HDRP / UE Cached Whole-Scene Shadows architecture), directly evolving our existing CW-4 cache (`sf_sky::lamp_cache`) and `CW-13` roadmap item.

### 7.1 Why a 2D Shadow Atlas beats Bevy's Cube Array

Bevy's `texture_depth_cube_array` has three hard structural bottlenecks:
1. **Fixed resolution for every lamp:** Every shadowed lamp pays `6 × 512² × 4 B = 6 MiB` (or 24 MiB at 1024²), whether it's a hero brazier 2 m away or a tiny candle 25 m across the room.
2. **Hard layer cap:** Standard wgpu allows 256 array layers → hard ceiling of `256 / 6 = 42` cubes.
3. **One ECS root view + schedule + encoder per face:** Even with CW-4 caching static faces, drawing 9 active mover faces in `pt_night` spun up 9 separate `RootNonCameraView(Core3dShadows)` schedules and added **+3.35 thread-ms** in `encode//sum` (+0.94 ms render busy).

A **2D Shadow Atlas** breaks all three ceilings at once:
- **Variable resolution per light (power-of-two tiles):** 512² for hero/close/carried lights, 256² for mid-room lamps, 128² for far/small candles (16× cheaper in VRAM & fill than 512²).
- **No 42-cube array-layer cap:** One 2D texture holds hundreds of tiles.
- **Spot lights share the exact same atlas** (1 tile instead of 6), unifying punctual shadows.
- **Empty / wall-facing cube faces take zero tiles:** A wall sconce typically needs only 3–4 tiles instead of 6 (saving ~25–35% of atlas space).
- **Single render pass / single encoder:** All tile updates in a frame draw into viewport/scissor rects of the atlas inside **1–2 render passes total**, eliminating per-face `Core3dShadows` schedules and `wgpu` encoder/submit multiplication.

### 7.2 Projection Choice: Why Unfolded 6-Face Cube (Not Dual-Paraboloid or Tetrahedron)

| Projection | Tiles / point light | Verdict for Morrowind / Scrib |
|---|---:|---|
| **Unfolded 6-face cube** (Doom 2016/Eternal, HDRP, Godot 4 default) | 6 (3–5 after empty-face skip) | **Best.** Zero geometric distortion on Morrowind's long, low-poly wall/beam triangles. Works natively with Bevy's perspective frustum culling and per-face culling (CW-3b). Only requires a small **guard band** (2–4 texels widened FOV per face) so 2×2 PCF never bleeds into adjacent atlas tiles. |
| **Tetrahedron** (4 triangular faces) | 4 | **Skip.** Saves 33% faces, but triangular tiles waste square atlas area or complicate packing, have uneven texel density, and need custom non-standard frustum/seam math. |
| **Dual-Paraboloid** (2 hemispheres) | 2 | **Reject.** Non-linear vertex projection bends long straight triangles into curves unless geometry is heavily tessellated — causes severe shadow detachment on low-poly Morrowind corridors, walls, and furniture (Godot 4 defaults away from it for this reason). |

### 7.3 Two-Atlas Static/Dynamic Architecture & VRAM Math

Use **two 2D `Depth32Float` textures** (or a 2-layer texture, though two separate 2D textures let the Live Atlas be much smaller and avoid any WebGPU same-texture read/write subresource hazard):

1. **Static Atlas** (persistent, buddy/quadtree-allocated):
   - Holds cached static geometry depth per face (`512²`, `256²`, or `128²`).
   - Captures are amortized (e.g., 2–4 faces/frame on admission; instant during door-fade blackouts).
   - **Clean faces (no movers — ~85–95% of faces) point directly into the Static Atlas.** Zero live tile, zero blit, zero draw, zero CPU cost!
2. **Live (Mover) Atlas** (small pooled atlas, only for faces that *currently* have a visible mover inside them):
   - Only 10–25% of faces ever have a character/door/physics object inside them in a given frame.
   - Each frame, **one single `RenderPass`** over the Live Atlas handles every active mover face:
     1. Set `viewport` + `scissor_rect` to the face's Live tile.
     2. Draw a 1-triangle **depth-blit** (`DepthCompare::Always`, writes `frag_depth`) copying that face's static rect from `Static Atlas → Live Atlas` (sidesteps WebGPU's restriction against `copy_texture_to_texture` sub-rect depth copies, and stays inside the exact same render pass — ~0.02 ms GPU for a dozen tiles!).
     3. Immediately draw **only the movers** in that face's frustum into the same `viewport` + `scissor_rect` (`DepthCompare::GreaterEqual` reverse-Z).
     4. Point that face's GPU tile record to the Live Atlas rect!
   - *Alternative considered — Dual-Sample without blit:* Keep movers in a cleared Dynamic Atlas and sample `min(pcf(static), pcf(dynamic))` in the main shader. Avoids the blit (~0.02 ms) and allows higher-res dynamic tiles, **but** doubles fragment-shader shadow fetches on mover faces and double-darkens soft PCF penumbras where a character's shadow overlaps a static wall shadow. **In-pass depth blit gives exact hardware PCF and keeps 1 fetch per light in the main pass.**

**VRAM & capacity math (`Depth32Float`, 4 bytes/texel):**

| Preset | Static Atlas | Live Atlas | Total VRAM | Realistic Simultaneous Shadowed Lamps |
|---|---|---|---|---|
| **Medium** | 2048² (16 MiB) | 1024² (4 MiB) | **20 MiB** | ~16–24 lamps (e.g. 2 @ 512² + 8 @ 256² + 16 @ 128²) |
| **High** | **4096² (64 MiB)** | **2048² (16 MiB)** | **80 MiB** | **~45–60 lamps** (e.g. 4 @ 512² + 16 @ 256² + 40 @ 128²) — *vs 48 MiB for only 8 lamps today!* |
| **Ultra** | 8192² (256 MiB) | 4096×2048 (32 MiB) | **288 MiB** | **100+ lamps** at 512²/256² (entire Balmora or any mega-interior at once) |

*(Note: `Depth16Unorm` halves all VRAM numbers above — 40 MiB on High — and with Bevy's reverse-Z `near / z` over typical 0.1–20 m lamp ranges has very good precision, worth an A/B after `Depth32Float` works.)*

### 7.4 Atlas Tile Allocation (2D Quadtree / Buddy Allocator)

Standard pattern from Doom 2016, Frostbite, and HDRP:
- Treat the `4096²` Static Atlas as a quadtree of power-of-two tiles (`512² → 256² → 128²`, optionally `64²`).
- **Allocation:** Request a tile of size `S` based on light importance (distance + range + `sqrt(luminance)` + priority). Split the smallest available larger block if needed.
- **Free & coalesce:** When a light is evicted or despawns, merge empty sibling quadrants automatically — **zero external fragmentation**.
- **Anti-churn / resolution hysteresis:** Don't immediately resize a lamp's cached tiles when the player takes one step; apply strong hysteresis (e.g., 1.5× distance band or only upgrade/downgrade when a lamp enters a new tier for > 1 s) so static tiles stay cached and don't trigger recapture storms while walking!
- **Handover spares:** Reserve a small fraction of atlas space so an incoming light captures and fades in *while* an evicted light fades out (simultaneous crossfade, no pop).

### 7.5 How It Plugs Into Bevy 0.20 (Without Fighting ECS Views)

This is the crucial part — **fixing the CPU cliff we hit in `pt_night`**:

1. **Main world (already mostly built in `sf_sky::lamp_cache` + `sf_sky_bevy::lamp_cache`):**
   - `sf_sky::lamp_cache` already speaks in abstract tiles, capture queues, mover tracking, and light-relative 1 mm invalidation.
   - Add the Bevy-free **2D Buddy Allocator** + resolution tier picker (512/256/128) + empty-face omission (`open_faces` mask) + `sqrt(luminance)` ranking.
   - Output a flat per-frame `ShadowAtlasPlan`:
     - `tiles: Vec<GpuShadowAtlasTile>` (for every shadowed light: `6` entries for point, `1` for spot — `uv_offset: Vec2`, `uv_scale: f32`, `atlas_and_flags: u32`: 0 = empty/unshadowed, 1 = Static Atlas, 2 = Live Atlas).
     - `static_jobs: Vec<AtlasDrawJob>` (0–4 static face captures this frame, with caster entity lists).
     - `mover_jobs: Vec<AtlasDrawJob>` (all visible active mover faces this frame, with their `static_rect`, `live_rect`, and mover entity lists).
2. **Render world (vendored `bevy_pbr`, replacing per-face `RootNonCameraView`):**
   - **Zero `RootNonCameraView` entities for atlas lamps.** Clean cached lamps have zero views, zero phases, zero encoders.
   - **Batched GPU preprocessing & drawing:** Instead of running N `Core3dShadows` schedules:
     - Upload a small dynamic uniform buffer of `ViewUniform`s (1 per active `AtlasDrawJob`, typically 2–16 jobs/frame, reused from a fixed pool!).
     - Draw **all `static_jobs`** in **ONE `RenderPass` on the Static Atlas** (clear each job's rect via a 1-triangle depth-clear quad, then draw its static casters with `set_viewport`/`set_scissor_rect`).
     - Draw **all `mover_jobs`** in **ONE `RenderPass` on the Live Atlas** (1-triangle depth-blit from Static Atlas → Live Atlas per job, then draw its mover casters with `set_viewport`/`set_scissor_rect`).
     - **Result:** Total shadow encoding for *all* local lights in the entire frame = **1 command encoder, ≤ 2 render passes, 1 `wgpu` bind-group walk per pass** — regardless of whether 4 or 16 mover faces update every frame!
   - **Reuse Bevy's existing shadow depth pipelines (`Shadow` / `DrawDepthOnlyPrepass` / skinned & alpha-mask shadow shaders):** Only the view uniform offset (dynamic offset into the atlas job view buffer) and viewport/scissor change per job!
3. **Fragment shader (`superfargoth/vendor/bevy_pbr/src/render/shadows.wesl`):**
   - Replace `texture_depth_cube_array` sampling in `fetch_point_shadow` with:
     1. Compute `frag_ls` and pick the major axis → `face_index ∈ 0..6` and face-local `uv ∈ [-1, 1]²` (~8 ALU ops).
     2. Look up `tile = shadow_atlas_tiles[light_atlas_slot * 6 + face_index]`.
     3. If `tile.flags == EMPTY`, return `1.0` immediately (zero texture fetch!).
     4. Scale `uv` by `inv_guard_tan` (e.g. `(size - 2*guard) / size`), remap to `[0, 1]`, then `atlas_uv = tile.uv_offset + uv * tile.uv_scale`.
     5. Sample `textureSampleCompareLevel` from `static_shadow_atlas` or `live_shadow_atlas` (hardware 2×2 PCF, or multi-tap rotated PCF / PCSS with texel-clamped clamping inside `[tile.uv_offset + half_texel, tile.uv_offset + tile.uv_scale - half_texel]` so filter taps never leak across tiles!).
   - Spot lights reuse the exact same `shadow_atlas_tiles` lookup (1 tile, no face selection).

### 7.6 Key Technical Risks & Gotchas (Check Before Coding)

1. **Seam bleeding between adjacent atlas tiles:** Hardware cubemaps filter seamlessly across cube edges; 2D atlases do **not**. Fix: widen each face's projection FOV by a 2–4 texel guard band (`tan_half = size / (size - 2 * guard)`) and clamp filter tap UVs to the tile's inner rect. Proven in Doom 2016, HDRP, and Clockwork.
2. **Normal bias scaling per tile resolution:** In Bevy's `shadows.wesl:35`, `shadow_normal_bias` is pre-multiplied on the CPU by `2.0 / map_size`. With variable tile sizes (512 / 256 / 128), `fetch_point_shadow` must scale normal bias by `1.0 / tile_size_px` (derivable from `tile.uv_scale * atlas_size`) so 128² far tiles don't get shadow acne and 512² near tiles don't peter-pan!
3. **Clearing/blitting sub-rects of `Depth32Float` in WebGPU/wgpu:** WebGPU's `LoadOp::Clear` clears the *entire* texture, and `copy_texture_to_texture` cannot copy sub-rects of depth textures. Fix: open the atlas render pass with `LoadOp::Load`, and for each dirty tile emit a 3-vertex fullscreen-triangle depth shader with `set_viewport` + `set_scissor_rect` (writing `0.0` for reverse-Z clear on Static Atlas, or sampling `Static Atlas` for blit on Live Atlas). Standard WebGPU/Vulkan practice; zero extra passes.
4. **Why `lamp_cache/movers` was 95 median in `pt_night`:** Before building the atlas, audit what those 95 "movers" actually were in `sf_sky_bevy::lamp_cache`! If non-character static/animated props (flickering candle flames, wind-swayed plants, far NPCs) are being classified as movers, fixing that classification will cut mover draw load dramatically for both CW-4 and the future atlas.
5. **Main-pass GPU cost of 40–60 shadowed lights:** Even with 60 cached lamps in the atlas, a given pixel only evaluates lights in its **froxel cluster** (typically 1–6 overlapping lamps), and Reset B2's early-out skips lights with 0 contribution + empty faces return `1.0` without a texture fetch. Still, keep **lamp contact shadows** restricted to the closest 4–8 hero lamps (not all 60!), because contact shadows raymarch screen depth per light (~0.3–0.8 ms/lamp).

### 7.7 Recommended Phased Rollout

- **Phase 0 (Diagnosis, ~0.5 day):** Inspect the `lamp_cache/movers = 95` count on `balmora_walk_night` and check what entities are dirtying faces; test CW-4 with `LampTier::StaticOnly` on a `census` interior + `balmora_walk_night` to confirm the zero-view CPU ceiling.
- **Phase 1 (Single-Encoder Batched Point Shadow Pass — kills the CPU bottleneck):** Replace per-face `RootNonCameraView(Core3dShadows)` with one batched render-world pass (1–2 encoders total) over Bevy's existing shadow pipelines, verifying that 12+ mover faces/frame now run cleanly without the +0.94 ms render regression and **eliminate SH-21a character flickering**.
- **Phase 2 (Unified Cached 2D Shadow Atlas — breaks the 8-lamp ceiling):** Introduce the `4096²` Static + `2048²` Live 2D atlases, 2D buddy allocator, in-pass depth clear/blit shader, guard-band `shadows.wesl` sampling, and per-tile resolution tiers (512²/256²/128²). Raise the High budget from **8 → 40–60 lamps** (covering entire Morrowind interiors at once, ending the "only close lamps cast shadows" trick).
- **Phase 3 (Quality polish & fallback):** Add `sqrt(luminance)` ranking, 2-spare simultaneous handover crossfades, empty-face tile omission, and optionally Bend screen-space shadows (CW-16) for unbudgeted lights beyond the atlas.
