# Visual checks: rendering

Open visual checks: far field, ghostfence, DLSS, render order, synthetic scenes (FF, GF, DL, RO). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

## Far field (superfargoth `far-field.md`, "Measured on Morrowind")

Screenshots: `D:\Gamedev\Bevyrrowind\shots\farfield\` (`balmora_a*`,
`redmtn_view*`, `summit_vivec*`, `ebonheart_night*`, `handoff_forest*`).

### FF-2: Far-field levels
- Kind: game location. Run: `.\bevyrrowind scene balmora`, press F5.
- Look for: L1 green from ~400 m, L2 blue from ~2 km, L3 purple beyond; no
  untinted gap and no patch that flickers between levels while flying.
- A/B: F5 on/off.
- Status: awaiting owner (render-opt 2026-10-28: `ImpostorSet` now holds per-level `[Handle<ImpostorMaterial>; 3]` sharing GPU texture arrays plus an emissive debug boost so F5 tints L1 green, L2 blue, and L3 purple cleanly through aerial haze; check `.\bevyrrowind scene summit-vivec --far-tint`).
- Source: farfield-lod.
- Owner 2026-10-30 (Shift+F6; F5 is quicksave now): every distant asset is green up to the horizon; no blue or purple
  seen, so L2/L3 still look never selected (hard-problems.md second tier). Status: rejected.

## Ghostfence (superfargoth `weather/distant-shadows.md`, bug B)

### GF-1: Ghostfence appears complete, never late
- Kind: game location. Run: `.\bevyrrowind scene ghostfence-far` and fly
  toward Ghostgate (the camera starts ~1.9 km south-west, 120 m up, facing it).
- Look for: the fence segments appear at their ring (~520-650 m) already
  loaded: no frames where the pylons stand without the field after a cell
  group reveals (B3 preload). Not fixed yet: the field is still invisible
  past the ring and appears there without a fade. Its segments are
  activators made only of blended parts, so they have no far copy (weather/distant-shadows.md
  B5: blended far proxies + scriptless activators).
- A/B: F5 (pylons tint green at L1; fence segments stay untinted). Compare
  `play\` before this session's exe if kept: there the field showed up a few
  frames after its cell.
- Also (owner 2026-10-19, round 6, from the summit at night): round light spots sit inside the fence "wall": they are the far-distance light sprites of the fence's blue lights, drawn while the field itself has no far copy. Not vanilla-looking.
- Status: known bug, seen by owner 2026-10-19 (round 2): the fence field disappears entirely at a distance and pops back in close. Cause known: blended-only activators have no far copy (weather/distant-shadows.md B5); Phase 1 or later.
- Source: weather-spec / farfield / nifpop.
- Owner 2026-11-15 (repeated): Ghostgate fence panels still pop in or vanish suddenly with distance. Owner suspects the emissive material has no LOD; the entry above says why: the segments are blended-only activators with no far copy (B5), so they appear at the ring with no fade.

## DLSS (superfargoth `sky-lighting.md`; `.sessions/dlss.md`)

Screenshots: `D:\Gamedev\Bevyrrowind\shots\dlss\`.

### DL-2: Super Resolution at 1440p
- Kind: game location. Run: `.\bevyrrowind scene dlss-balmora-1440`
- Look for: as sharp as native (F4 to off) when still and when flying;
  frame time drops ~12 to ~9 ms.
- A/B: F4.
- Status: deferred 2026-10-19 (phase0): recheck with a solari,dlss build when the RT role is decided (Phase 1).
- Source: dlss.

### DL-3: Ray-traced Census Office with Ray Reconstruction
- Kind: game location. Run: `.\bevyrrowind scene census-rt` (raster
  baseline: `.\bevyrrowind scene census`).
- Look for: clean, stable bounce lighting without crawling noise; brightness
  close to raster.
- A/B: `.\bevyrrowind scene census-rt --off denoiser` (noisy); F4 changes the
  RR quality.
- Status: confirmed by owner 2026-10-19 (round 14, solari,dlss build): the Census Office looks nice. Feels very smooth (owner). Gaps: no characters (`--rt` still opens the single-cell viewer, which spawns no actors, and Solari 0.20 traces static BLASes only: skinned meshes need a compute skin cache). Phase 1.10.
- Source: dlss.

### DL-4: Rug-edge black bands under RT
- Kind: game location. Run: `.\bevyrrowind scene census-rt`, look at the floor where rugs end.
- Look for: black bands along rug edges, with and without the denoiser.
  Believed to be content (rug geometry), not DLSS.
- A/B: `--off denoiser`.
- Status: known bug, owner 2026-10-19 (round 14): black bands at the edges of rugs, and also a table, a bench and a chair. Cause not investigated. Suspects: objects touching or interpenetrating the floor (contact gaps fully occluded, ray-origin offset too small), double-sided or flipped faces, then bad normals. First test: same camera in raster; if the bands are RT-only, it's the RT scene, not the meshes. Phase 1.10.
- Source: dlss.

### DL-5: Interior `--rt` without Ray Reconstruction quits (wgpu validation)
- Kind: game location. Run: `view "Seyda Neen, Census and Excise Office" --rt --off denoiser` (or `--rt` with no
  DLSS plugin package, which falls back to TAA).
- Look for: the app quits at the first frame: `Device::create_bind_group 'sky_motion'`: the view depth texture has
  `COPY_SRC | RENDER_ATTACHMENT`, no `TEXTURE_BINDING`. With RR on it runs (the RR path makes depth bindable).
- Status: awaiting owner (render-opt 2026-10-28: guarded `sf_render_order_bevy/src/sky_motion.rs` `prepare` on `TextureUsages::TEXTURE_BINDING` so `view "Seyda Neen, Census and Excise Office" --rt --off denoiser` runs without wgpu validation panic).
- Source: dlss-plugin-2.

## Synthetic scenes (framework examples)

Run from `D:\Gamedev\Bevyrrowind\superfargoth`; they bake their data on
first run (cached afterwards). Each isolates one system on generated
content, so a bug seen here isn't Morrowind content.

## Haze and dust (superfargoth `weather/distant-shadows.md` "Aerial perspective and haze" N1/N2; ROADMAP M6 F1; `.sessions/m6-f1.md`)

The `--clouds` weather now also sets the atmosphere's aerosol haze and an absorbing dust/ash term (table in
`weather-adapter.md` "Haze and dust"); `--haze <x>` and `--dust <x>` fix them for A/B. The C key's genera use the
fair-weather look. Near fog, ground fog banks and god rays are F2/F3, not this check.

### RO-A: Translucents cast no shadows; distant lights and rain fade into fog (render-order RO-1/2/3, rofix1)
Status: **step 1 confirmed by owner 2026-10-19** (no flame/glow shadow blobs in Balmora); steps 2-4 not checked. Agent check: dev build renders, shaders compile, rain at night visible (`view "Seyda Neen"`, hour 22, rain); fading not judged.
1. `view "Balmora" --hour 11 --weather clear`: torch flames, glow cards and glass cast no shadow on sunlit walls/ground; trees (cutout leaves) and solid props still do. Compare `--off transfog` (old: flames/glows cast near-solid shadows).
2. `view "Seyda Neen" --hour 22 --weather foggy`, fly ~1-3 km out: the far town's light points dim with distance and vanish into the fog instead of staying full-bright.
3. `view "Seyda Neen" --hour 15 --weather rain` (then `foggy` + rain via C): far rain streaks fade into the fog colour; near streaks unchanged.
4. Ghostfence (`view "Ghostgate" --hour 12`): the field casts no shadow.

### RO-B: Fire/torch smoke not drawn in front of the sky, or not hiding objects behind it (owner report)
- Kind: game location. Run: `.\bevyrrowind scene hi-balmora-night` and `.\bevyrrowind scene particles-census-fire`;
  look at torch/brazier smoke with the sky (and then a wall or NPC) behind it.
- Seen by owner 2026-10-21 (bw-owner10, AU-2 and PA-4): smoke sometimes goes missing in front of the sky and/or does
  not hide the objects behind it properly. Render order (translucent sort vs sky/clouds/fog passes), Phase 1. Not the
  GPU particle path itself (PA-4 confirmed). ROADMAP 1.12 calls it "RO-8" (render-order.md RO-8 is a different, fixed item).
- Ruled out (smokeorder 2026-10-21, by code): sky, stars, cloud/fog composite all draw before the transparent pass.
  Open: H1 particles sort by emitter origin (no bounds, `NoFrustumCulling`), so smoke metres up sorts as the flame
  (RO-10); H2 particles always lit (`particles.wesl` ignores unlit): dark smoke on dark sky; H3 TAA, no smoke MVs.
- Status: awaiting owner (render-opt 2026-10-28: added lit smoke shading via `p.misc.z` in `particles.wesl` so dark smoke catches ambient/froxel/point light against dark skies, replaced emitter-origin `NoFrustumCulling` sort with dynamic conservative plume `Aabb` in `mw_assets/src/particles.rs`, and swept `RO-6` DLSS `main_pass_viewport` UVs across clouds/water/precip/sky_motion).
- Owner 2026-10-30 (`scene hi-balmora-night`, dev copy): torch smoke now draws over everything, but its blending looks
  wrong, like the Ghostfence (GF-1): it seems not to show over darker things. Hard to describe; looks like a blend-mode
  issue (additive vs alpha?). Status: partial.
- Owner 2026-11-15 (`view "Sadrith Mora" --hour 21.25 --weather foggy`, owner shots `Documents/ShareX/Screenshots/2026-11/bevyrrowind_hUnUMC5zsg.jpg`, `bevyrrowind_oQU4iWEVPc.png`): looking through torch smoke, distant mushrooms/rocks that the fog hides elsewhere show unfogged, at full contrast, inside the plume. Hypothesis (not tested): the fog composite (render-order.md step 8, fogs by depth) sees the smoke's near depth at those pixels (a particle depth write or prepass), so the background is fogged as if it were at the smoke's distance; industry: blended particles never write depth and fog themselves (render-order rule 1). Status: known bug.
- Diagnosis + fix 2026-11-15 (bugfix-11-15): no depth or blend bug (smoke is NIF 0xED alpha blend, no depth write, no prepass). Cause:
  smoke was lit as a PBR surface (no Morrowind light saturation), so near its torch it was a dark veil: invisible over dark walls, and
  in lit froxel fog it replaced the bright fog veil (the "unfogged hole"). Now lit as a volume (no N·L, Bevy point/spot/sun lights,
  sun keeps atmosphere transmittance) with per-lamp saturation to texture x colour; NIF colour keys converted sRGB->linear
  (`mw_assets` particles.rs/.wesl). Look for: `scene hi-balmora-night`, grey textured smoke over dark walls, no white blow-out at
  the flame, flames about as bright as before; `view "Sadrith Mora" --hour 21.25 --weather foggy`, plume sits in the fog, no clear
  hole. Risk: smoke far from lights darker than before. Status: awaiting owner.
- Owner 2026-11-15: smoke disappeared; carried-torch handles very bright (RO-23). Fix 2026-11-16 (smoke-2): the sRGB key  conversion was wrong (game-wide NIF vertex colours stay raw) and the per-lamp cap kept the plume at 0.08x its texture; now  Morrowind's clamp on the sum of lights, "fully lit" in exposed units: beside a torch the plume shows its plain texture  (1.0 at 0.5-1.5 m, 0.6 at 3 m) at any exposure (old path: 10.7x at 0.5 m, up to 121x at night). Same look checks as above.  Status: confirmed by owner 2026-11-16 (`scene hi-balmora-night`).

### RO-13: F7 wireframe drawn away from the model (owner report)
- Kind: synthetic. Any anim-lab level, dev build, F7 (wireframe over skinned meshes; `mw_devtools_bevy/src/live_ab.rs`).
- Seen 2026-10-30: the wireframe seems to exist but shows at a different place than the character it belongs to.
- Suspect (not tested): the wireframe copy isn't placed in the same big_space grid cell / transform as the skinned mesh.
- Status: known bug: not diagnosed (owner pass 2026-10-30).
- Owner 2026-10-30 (Balmora, release copy): in the game F7 shows the wireframe on the terrain only, none on characters.

### RO-16: Terrain "veil", as if seen through resin (owner report 2026-11-05; was ROADMAP 1.12 "RO-13")
- Kind: game location, dev build. Run: `powershell -File tools/run_dev.ps1 scene ro16-terrain-veil` (Balmora street, 12:00, clear), then again with `--hour 23`. Seen: strongest at grazing angles, ground takes the sky/env colour, night = smooth blue-grey "clay" sheet, silhouettes lighter, rocks/trees less hit.
- Console (backquote), one at a time, each followed by the same line again to restore: `flip terrain_env_specular` (env-map specular only), `flip terrain_specular` (every specular lobe), `flip ambient` (the sky's flat 4-nit blue night ambient, all objects), `flip terrain_indirect` (env map + ambient + GI on the terrain), `render.bloom 0` (back: `render.bloom 0.1`), `flip fog`, `flip surface`, `flip terrain_tint` (raw layers). F1 shows their state.
- If it's the culprit: env_specular fixes noon grazing/shade; terrain_specular also fixes night; ambient (night only) makes the ground darker and its own colour again, no blue; indirect = env or ambient (ask which of the two above); bloom/fog lift the whole image, not only the terrain; surface/tint should change nothing on dry ground.
- Owner 2026-11-05, night: `ambient` lifts it (ground dark, no blue); `terrain_specular` removes the blue reflection and keeps the ground visible; `terrain_env_specular` no visible change; the rest irrelevant. Cause at night: the flat blue ambient (its specular lobe most). Noon not checked yet. Status: night diagnosed, fix pending. Toggles are RO-16 scaffolds (`TerrainDebug` bits 8/16/32/64, `Switch::Terrain*`, console `ambient`): remove with the fix.
- Fix 1 (terrain reflectance 0.3, was 0.5; console `terrain.reflectance`): look for the blue night film mostly gone, sheen in sun and rain kept; try 0.2-0.4 live. Status: awaiting owner.
- Fix 2 (night ambient from weather, `flip weather_ambient`): look for no blue cast at night; Clear vs Overcast vs Ash tint; brightness unchanged. Weather via C (next weather), `--hour 23`. Vanilla Clear nights stay slightly blue (Morrowind's own colour); Overcast/Rain greyer. Status: awaiting owner.
- N1 (flat ambient diffuse only, 2026-11-06; `flip ambient_specular`, on = old): `--hour 23`, look for terrain, walls and roofs keeping their dark readable colour with no blue sheen at grazing angles (silhouettes no longer lighter); lamps' highlights unchanged; noon identical. Also an interior (`view "Seyda Neen, Census and Excise Office"`): no sheen, not darker overall. Status: awaiting owner.
- N2 (night sky glow, 2026-11-06; `flip night_sky_glow`, off = black sky): `--hour 23`, look for a dark sky brighter toward the horizon (not a twilight; vanilla Clear "Sky Night Color" is near grey: say if it should take the bluer ambient hue), thin clouds darker than the sky not distracting, stars still visible, overcast = dim grey-blue cloud not black, wet/smooth surfaces reflecting that sky faintly; C cycles weather (Blight: reddish sky). Noon identical. Status: awaiting owner.
- N3 (the moons light the world, 2026-11-06; `flip moonlight`, off = the old weak moon): `--hour 23` (half Masser), `--day 167 --hour 21.5` (both full, low), `--day 179 --hour 23` (new: floor only). Look for walls and ground facing the moon visibly lighter and bluer (`Sun Night Color`) than the shadowed side, soft moon shadows near the camera (2 cascades to 120 m), no part darker than with `flip moonlight` off; overcast = faint. Risk: the moon also lights the sky (a blue-grey moonlit sky, brighter than N2 alone): say if it reads as dusk. Status: awaiting owner.
- Owner 2026-11-06 (`--hour 23`, clear): ground better (fix 1 / N1 OK), soft moonlight OK (N3); the night is too lit ("we can see very
  well"): wants values closer to OpenMW (knobs `--night-ambient` 4 nits, `--exposure-floor` 3.5; 1 nit was "too dark" 2026-10-19).
  Secunda (white moon) not visible: hidden behind Masser or not drawn, not diagnosed. Status: night brightness + Secunda open.
- Audit 2026-11-13 (tex-audit, code only, not A/B tested): new suspect for the day+night veil: the ground gets no normal or ARM map at all (`mw_world/src/terrain.rs:239-240` sets `normal: None, arm: None`; T5 not built), so every layer is a flat normal at roughness 0.9 with one even specular sheen, and layers are cut to 256 px (`mw_bake/src/world.rs:39` `LAYER_ARRAY_SIZE`, about 35 px/m; packs ship 2K). Lever: T5 + T6 (terrain normals, BC7/BC5 arrays at a per-preset size).

### TX-lab: Texture lab, normal / ORM maps one asset at a time (tx-lab, 2026-11-13)
- Kind: synthetic scene + global A/B. Needs a current bake with the packs mounted (`texture.cook: layer mounted, ... K normal
  companions, L ORM companions`, K, L > 0). Run: `scene level-texture-lab` (07:30 clear; cameras `wide`, `models`, `swatch-1..6`,
  `model-1..4` via `--camera`); own picks with `--lab-textures tx_a,tx_b --lab-models x/ex_a.nif`.
- Keys: N normal maps, M ORM maps (= console `flip normalmaps` / `flip orm`, also usable in Balmora; `--off normalmaps,orm`
  starts with them off), Z turntable, X lamp orbit, I info panel, `[` `]` sun.
- Look for: on each swatch, relief that moves with its orbiting lamp and the low sun and vanishes with N (none = no data:
  read the panel); M: metal swatch darker with coloured highlights, stone/wood matte. Panel per swatch: normal / ORM file,
  format (BC5 / BC1-BC7), mips, "bound", tangents yes. Models the same while turning.
- Log (no picture): `texture-lab: 6 swatches, 4 models; normals K/6, orm L/6`, then `texture-lab swatch N: ...` lines.
- Status: owner 2026-11-13 (full re-bake, default picks): normal maps work but TOO FAINT ("had to turn N on to see"; wants stronger); log: 0 sRGB normals, 0 without tangents, swatches 1+3 no companions, models 3-4 (`terrain_rock_wg_01/05.nif`) do not exist in the install (lab default bug). Source: tx-lab (`.sessions/tx-lab.md`).
- Owner 2026-11-15 (after tx-nstrength + tx-labpicks, log normals 6/6, orm 6/6, depth 6/6): N "definitely works well", M works. Follow-up: the X orbit lamp only shows at night (the 09:30 sun swamps it) and is too bright at night; lab tuning only. Status: confirmed with follow-up.
- Owner 2026-11-15, new bug, not diagnosed: as the X lamp orbits, something pops off abruptly, possibly the lamp shadow; owner unsure it happens when the lamp passes behind an object (the lab lamp has no visible marker, so its position cannot be seen: a small gizmo would help). Candidates: lamp shadow budget / cache (SH-20/SH-21 family), the double-sided normal flip guard (tx-nstrength), light culling. Logged on SH-21 too.

### TX-5: Parallax occlusion on statics from `_nh` height (textures T4d, 2026-11-13)
- Kind: synthetic scene + game location, A/B. Needs a re-bake with the packs mounted: `texture.cook: ... N normal
  companions (D with depth)` and at runtime `texture.cook: layer mounted, ... D depth companions`, D > 0.
- Runs: `scene level-texture-lab --lab-textures tx_cobblestone_01,tx_bm_cobblestone_01,tx_doorbricks_01,tx_dwe_brick00,tx_wg_cobblestones_01,tx_v_floor_01`
  (all ship `_nh`; cameras `swatch-1..6`), then `view "Balmora" --hour 8` (Hlaalu atlas `_nh`). Default High (Relief);
  also `--quality medium` (Occlusion) once.
- Look for: cobbles and bricks with real depth at grazing angles (stones stand out, mortar sinks), no swimming or
  crawling when the camera moves, no smearing or stretched texels at silhouettes and edges, nothing floating off
  the surface. A/B: key L in the lab, `flip parallax` anywhere (`--off parallax` starts off).
- Log (no picture): lab panel `depth: textures/..._nh_depth.ktx2 ... bound Relief { max_steps: 5 } 16 layers,
  scale 0.04`; `materials.live:` depth count > 0.
- Status: owner 2026-11-13 (lab default picks, L toggled): ARTIFACTS: stepped layers (Bevy layer count = mix(max 16, 1, |V.z|): ~8 slices at 45°, depth scale 0.04); fix = more layers per preset (industry 32-64 at grazing) + check scale, then re-check. Source: tx-pom (`.sessions/tx-pom.md`).
- Owner 2026-11-13: texture SWIMMING on some meshes when moving around them (e.g. big mushrooms): not yet diagnosed (parallax on curved meshes? missing tangents?); A/B `flip parallax` / `flip normalmaps` on one. Owner also wants parallax as a player toggle.
- Owner 2026-11-15 (CK-1 run, `balmora-sunset-shadows`, after the tx-pomfix re-bake): texture swimming still on some rocks, not everywhere; owner suspects POM. A/B `--off parallax` in the TX-5 re-check.
- Owner 2026-11-15, lab half (default picks, depth 6/6, L toggled): parallax works (after tx-pomfix). Balmora half next.
- Owner 2026-11-15, Balmora 08:00: depth on walls and paving works. Swimming on some landscape rocks is POM (gone with `flip parallax`), only at grazing angles; "okay for now, fix later" (likely curved rock meshes: POM assumes a flat surface; silhouette fade / slope clamp of tx-pomfix not enough there). Status: confirmed with follow-up.

### SS-matclass: Static material classes look unchanged (static-scene S2, 2026-11-08)
- Kind: game location, A/B. Runs: `view "Balmora" --hour 14` and `view "Seyda Neen" --hour 14`, each default and
  restarted with `--off matclass` (old materials, one sampler per image). Default should look identical.
- Look for any surface that differs between the two: missing or wrong textures (sampler sharing), clamped vs tiled
  edges on signs and banners, cutout edges (fences, hedges, tree cards) thicker or thinner, two-sided leaves and
  banners losing their back face, leaf light-through (RO-15) still on, decals or glowing windows unchanged.
- Gauges in a perf run (`statics/material_pipelines`, `statics/material_bind_groups`, `statics/bindless`): perf.md
  "Static material classes (S2)". If anything changed, run with `--off matclass` and report the surface.
- Status: awaiting owner.

### RO-17: Tree leaves reflect as one flat panel (owner report 2026-11-05)
- Kind: game location. Seen in `scene ro16-terrain-veil` (Balmora, 23:00 clear): a whole leaf card reflects as one panel in the light's colour, the leaf texture lost, no micro detail.
- Suspects (not diagnosed): flat cards with no normal map catch one uniform specular (statics reflectance 0.3, roughness 0.85, `mw_assets/src/lib.rs:453`); the flat-ambient specular lobe (same as RO-16 night); foliage transmission (RO-15) lighting the card evenly (A/B `--off foliagetransmission`).
- Status: parked (owner 2026-11-06: later).

### SC-foliage: Cutout candidates drawn from the static table (static-scene S1b, 2026-11-09)
- Why: an authored blend marked `nif.blend = "cutout_candidate"` draws alpha-tested at load when its texture's
  alpha has holes, and blended when it does not. The bake now reaches the same verdict from the texture bake's
  per-texture alpha class (`texture.alpha_class` → `mw_bake::scene::runtime_alpha`), so foliage, fences and
  grates enter the GPU table while glass and gauze stay sorted entities. A misclassified texture would show up
  as one surface drawn with the wrong edges.
- Kind: game location, A/B. Needs the S3b `--scene` path and a current `bevyrrowind bake` (start log:
  `alpha classes: N cutout candidates`). Runs: `view "Seyda Neen" --hour 14` and `view "Balmora" --hour 14`,
  each default and restarted with `--scene`.
- Look for: no missing vegetation, hedges, fences, grates or tree cards with `--scene` on; and no candidate
  surface with hard cutout edges where it used to fade softly (glass, gauze, cobwebs, smoke cards) — name the
  surface, which names the misclassified texture (`textures::alpha_is_cutout` thresholds).
- Status: owner 2026-11-11 (Balmora): no missing cutouts seen. Source: static-scene scene-fixes, defects 1 + 5.

### SC-scene: Statics drawn by the static scene (static-scene S3b C2, 2026-11-11)
- Why: `--scene` draws exterior statics from the baked GPU table (one batch per material/slab, slots retained) instead
  of entities + precombine. First time it runs in the game: anything wrong shows as missing, doubled or misplaced statics.
- Kind: game location, A/B. Needs a current `bevyrrowind bake` (log: `static scene: N cell tables`, `N chains`).
  Runs: `view "Balmora" --hour 14`, default then restarted with `--scene`; then walk/fly ≥ 300 m in one direction with `--scene`.
- Look for: every building, rock, fence and tree present with `--scene` (none missing, none flickering or z-fighting
  as a double draw); after crossing ~256 m nothing jumps or drifts (origin shift, O1); doors, containers, lights and
  scripted objects still entities (interactable); cutouts as SC-foliage. Log: no `sf_scene_bevy` errors, no `is full` warning.
- Status: owner 2026-11-11: "seems perfect": nothing missing or popping; a subtle flicker now and then, maybe shadows or fog
  (unattributed: compare in sc-noscene); felt faster, no FPS counter in the overlay to tell. Log: 883 `batch ... @exterior is
  full (4096 slots)` warnings (the overflow spawns as entities, so the batch is undersized for Balmora, and the warn-once
  doesn't hold). Logged, not fixed.
- Owner 2026-11-11, later in the same run: **foliage floating in the sky** when flying up; grass/foliage only, appearing and
  disappearing as a batch every ~200 m. Check sc-noscene (A/B) to tell if `--scene` causes it. Frame rate from the log (physics
  line, frames per 10 s): ~49-62, mostly 60 (vsync cap?); the stairs scene ran ~121 in the same build.
- A/B (sc-noscene, owner 2026-11-11): "exactly the same things", the short shadow flickers included, so the flicker and
  the floating foliage are not from `--scene` (log them as their own bugs). Frames per 10 s off: 90, 72, 69 (35 s run,
  different spots) vs on ~49-62: no faster on this evidence; the batch overflow above may explain it. Perf suite pending.
- Perf A/B 2026-11-11 (profiling exe, 1440p, DLSS Quality loaded, 3 runs each; benchmarks/runs/scene_off, scene_on):
  balmora_walk median 11.4-12.3 ms off vs 14.4-14.9 ms on (+~2.7 ms); balmora 9.3 ms off vs 11.2-11.8 ms on (+~2.1 ms).
  GPU busy 77-96% off vs 63-88% on (more CPU-bound). The on runs log 81 `batch ... is full (4096 slots)` warnings
  (overflow spawns as entities, so both paths run at once). Audit 2026-11-11 (traced, incl. 32k slots, no overflow:
  still +1.8 ms): per-frame O(ranges) CPU in the GS port + per-frame `extract_chain_slabs`; capacity is secondary.
  `superfargoth/docs/research/static-scene-perf-audit-2026-11-11.md`.

### SC-lamps: Lamp shadows of statics drawn by the static scene (static-scene fix 5, 2026-11-12)
- Why: the point-light shadow cache only knew `Mesh3d` casters; scene batches are now static casters (capture after the
  batch visibility, `ShadowCastersChanged` from cell stream-in/out and promotion recaptures nearby lamps).
- Kind: game location, A/B. Run: `scene sc-scene-night` (Balmora 22:00, `--scene`); A/B: `scene sc-noscene` then `[`/`]` to night.
- Look for: lamps cast shadows of buildings and walls, steady (no flicker or popping); walk into a new cell: its statics cast
  lamp shadows within ~2 s; shadows of cells left behind clear. Same shadows as without `--scene`.
- Status: awaiting owner.

### RO-18: Grass and foliage floating in the sky (owner report 2026-11-11)
- Seen in `scene sc-scene` and `scene sc-noscene` (Balmora 14:00), so not the static scene: fly up high and grass/foliage
  hangs in the air; it disappears and reappears as a batch every ~200 m of travel (grass or cell streaming, unknown).
- 2026-11-11 later (scene-audit): not reproduced. A scripted climb to 700 m and the owner's own flight (to 595 m, coast and
  open water, BRP capture of every new mesh) showed nothing floating; vertical origin shifts move all statics correctly.
  Intermittent. Capture tool: `node bevyrrowind/tools/brp_capture_meshes.mjs <out.jsonl> [port]`; rerun when it shows again.
- Status: known bug, not diagnosed. Next step: a diagnosis slice (which system spawns it: groundcover, statics, streaming).

### RO-19: Very short shadow flickers here and there (owner report 2026-11-11)
- Balmora 14:00, same in sc-scene and sc-noscene: brief, subtle flickers, the owner thinks shadows (maybe fog).
- Status: known bug, not diagnosed (see the shadows checks SH-* for related cases).

### RO-22: Near/far handoff band with GPU-culled statics (perf-push P3, 2026-11-12)
- Why: near statics are no longer culled by distance on the CPU (`NoCpuCulling`); their visibility ranges (LOD levels,
  the dithered end band where the far field takes over) are now honoured only by the GPU preprocess, in every view.
- Kind: game location. Run: `scene handoff-forest` (Seyda Neen trees at ~400 m) and `scene lamps-handoff` (fly back with
  S, Shift 400-600 m); F5 shows the LOD tint. A/B with `--off cpuculltag`.
- Look for: no gap (ground showing where a building or tree should be) and no double (near and far copy both solid) in
  the handoff band; LOD switches dither as before; sun shadows of near statics end where the statics end.
- Status: awaiting owner.

### TX-6: Statics look unchanged with tangents everywhere + shared flat normal map (fix B2, 2026-11-15)
- Why: perf fix B2 (texture-cost doc): every static now bakes tangents (b2a) and statics without a normal map bind one
  shared 1x1 flat map (b2b, `matclass::FLAT_NORMAL_MAP`), so a class has one pipeline. The look must not change.
- Kind: game location, A/B by relaunch (no live toggle). Needs the 2026-11-15 re-bake (mesh rule v3).
- Runs: `view "Balmora" --hour 8`, then the same with `--off flatnormal`; also `scene hi-balmora-night` once.
- Look for: walls, roofs, rocks, plain wood and cloth statics shade the same in both runs (no facets, no dark or
  speckled patches, no seams on smooth curved meshes, nothing lit from the wrong side, no black meshes); normal-mapped
  statics unchanged. Most likely spot if wrong: meshes whose source had no normals (smooth-normals fallback).
- Status: confirmed 2026-11-15 (owner: default and `--off flatnormal` "looked exactly the same"). Source: orch-11-15b (b2a-tangents, b2b-flatnormal).

### RO-23: Carried-torch handles unnaturally bright at night (owner report 2026-11-15)
- Kind: game location. `scene hi-balmora-night`: look at torches carried by guards/NPCs (the handle and head, not the flame).
- Seen by owner 2026-11-15: "the handle of torches seemed very very lit, very unnaturally bright". Predates the RO-B work.
- Cause (bugfix-11-15, by code): `mw_assets::self_lit` (carried light models, `mw_actors_bevy` held.rs) mapped Morrowind's
  "fully lit" to nits calibrated at SCENE_EV100 7 and exposed them; night exteriors sit at the EV 3.5 floor (`lights.rs:81`),
  ~11x brighter (and near black at day EVs). Fix 2026-11-16: emissive in exposed units (emissive 1 x texture, exposure
  weight 0) = the plain texture at any exposure; torch smoke's saturation uses the same exposed "fully lit".
- Look for: at night the handle shows its plain wood/metal texture, not glowing; by day not black. Status: confirmed by owner 2026-11-16 (`scene hi-balmora-night`): handles no longer over-bright.

### PP-1: F3 perf panel shows honest live numbers (CW-17, 2026-11-16)
- Kind: game location, release build (`cargo build --release -p bevyrrowind`, no `dev`). Any exterior, e.g.
  `view "Balmora" --hour 12` or `scene hi-balmora-night`; flags none (start with env `BEVYRROWIND_PANEL_GPU_TIMERS=1` for GPU times).
- Press F3: compact panel top-right (frame ms + fps, p99, max; main, render, GPU; bound by). F3 again: full (main
  stages, top GPU passes, views by kind, lamp faces culled, lights, encoders/submits, entities, preset, non-default
  tunables). F3 again: off. Console `flip perfpanel` does the same as compact/off.
- Look for: digits refresh ~4x a second and stay readable; frame ms matches the top-left fps; "bound by" matches the
  feel (GPU when the view is heavy, main when the town crowd is busy); flipping a CW switch or a tunable (console)
  moves the numbers within ~1 s and shows the tunable in the list with its layer; no hitch on open/close.
- Status: confirmed 2026-11-16 (Balmora 22:00, release): works. Owner follow-up: "extremely rudimentary", wants visualizations (graphs, bars), CW-17b. Source: cw17-panel.

### PP-2: Graphics presets switch live, Extremely low to Ultra (tunables T5b, 2026-11-16)
- Kind: game location, release build. `view "Balmora" --hour 22` (night: lamps), then again `--hour 12` (day: sun
  cascades, GTAO, bloom); start level `--quality high` (default), `--quality extremely-low` for a cold start at the floor.
- Press F3 (full panel), then Shift+F4 repeatedly (key removed 2026-11-16, T5d: now Esc > Options > Graphics, PP-4, or
  console `preset <level>`): extremely_low, low, medium, high, ultra, wrapping. The overlay note says
  what changed and what "applies on restart" (texture cap, far handoff, probe GI); F3's `preset:` line follows.
- Look for: each step changes the look the same frame with no crash or long hitch (a short pipeline hitch on GTAO/contact/
  filter/upscaler changes is expected); Extremely low: no lamp shadows, short one-cascade sun shadow, no GTAO or bloom,
  softer (DLSS Performance builds) or aliased (MSAA) image, nearer LOD swaps; Ultra: 12 lamp shadows, 4 cascades to 280 m.
  Console `set shadow.lamp.count 2` turns the panel to `custom (...; shadows custom)`; `reset shadow.lamp.count` back.
  Compare the panel's frame/main/render/GPU per level (CPU vs GPU bound). Textures change only after a restart.
- Status: confirmed 2026-11-16 (Balmora 22:00, release): cycling works. Owner follow-up: presets belong in the options menu, no shortcut (T5d). Day half not run. Source: tunables-t5b.

### PP-3: F3 perf panel graphs: frame-time graph, thread bars, pass bars (CW-17b, 2026-11-16)
- Kind: game location, release build (`cargo build --release -p bevyrrowind`). `view "Balmora" --hour 22`; flags none
  (env `BEVYRROWIND_PANEL_GPU_TIMERS=1` for the GPU line, bars and passes). Fullscreen 1440p, then once `--resolution 1280x720`.
- F3 compact (top right, dark translucent): frame line (mean, med, p99, max), "bound by" in its colour, the frame-time
  graph (last 600 frames: columns coloured main orange / render blue / GPU green / none grey, main/render/GPU lines on
  top, dashed 8.3 ms white and 11 ms red target lines, 25 ms scale, magenta mark on spikes above it), legend + "last N s",
  main / render / GPU bars against the frame (the bound one in full colour, "< bound"). F3 again: full adds GPU passes and
  main stages as sorted bars, the counts, submit-ms and views sparklines, preset and tunables. F3 again: off.
- Look for: the graph scrolls smoothly, a hitch (open the console, turn fast) stands out as a tall column; the column
  colour and "bound by" agree; bars move with the numbers; text readable at 1440p and 720p, nothing cut off the
  screen; no hitch opening/closing; `set diag.panel.scale 1.5` grows it live. Say what to add or drop.
- Status: confirmed 2026-11-17 (Balmora 22:00, release): "visualisation seemed nice"; less informative than the F3 panel of the Clockwork renderer (owner reference; compare when the panel is next touched). Source: cw17b-panel-viz.

### PP-4: Graphics page in the options menu, live behind the pause menu (tunables T5d, 2026-11-16)
- Kind: game location, release build. `view "Balmora" --hour 22` (no `--quality`: it would win over the saved levels).
- Esc > Options > E twice (or click the tab) to **Graphics**: Graphics Preset, then View Distance, Shadows, Lighting, Post-processing,
  Textures, Effects, Water, Sky, each Extremely Low / Low / Medium / High / Ultra (starts at High).
- Change Graphics Preset (left/right): the town behind the menu changes at once (lamp shadows, sun cascade reach, GTAO,
  bloom, LOD swaps), every group row follows; then change one group (e.g. Shadows): it changes alone and the overall
  reads **Custom**; steps never land on Custom. Graphics Preset, View Distance, Lighting and Textures show "Applies after
  restart" in the right pane (texture size, far land reach, probe GI wait for a restart). The overlay note says what changed.
- Look for: no crash or long hitch per step, labels readable, rows match F3's `preset:` line; quit and start again with
  the same command: the picked levels come back (saved in `<user data>/machine.toml`).
- Status: blocked 2026-11-17: the page itself could not be judged, the options menu is unusable (earlier windows stay visible and overlap new ones, items move on hover). Owner: menu overhaul is a big later session (ROADMAP 2.2b), not now. Source: tunables-t5d.

### PP-5: CW gates on by default? Owner play session (ROADMAP 1.25, 2026-11-17)
- Kind: normal play, release build, F3 compact open. Start: `bash tools/owner_launch.sh --release view "Balmora"
  --tunables tunables/cw-gates-on.toml` (all gates on; the file lists them). Play as usual: walk Balmora by day, wait to
  night (`[`/`]` hour keys), go into a house and the Census Office (Seyda Neen), back out, cross a crowd, run fast.
- A/B any gate live in the console: `set <name> off`, then `reset <name>` (back on). Watch the F3 graph and main/render/GPU.
- Look for, per gate (anything that looks wrong = that gate stays off):
  - `shadow.lamp.day_gate`: lamp shadows switch on at dusk without a pop; none missing at night or indoors.
  - `shadow.lamp.face_cull`: no lamp shadow vanishing or appearing as you turn; shadows behind you come back when you look.
  - `shadow.sun.gate`: sun/moon shadows fade smoothly in overcast and at night; never cut while the sun is clearly visible.
  - `select.gate`: no terrain holes, late LOD swaps or far-field gaps after a fast run, a turn on the spot or a door.
  - `render.skin.skip_unchanged`: no frozen, stretched or T-posed characters, near or far, in the crowd.
  - `mw.matctrl.gate`: animated surfaces (water in fountains, lava, flame/sign textures) keep moving when you look back.
  - `stream.interior.disable_exterior` (DR-5): indoors nothing changes on screen and no outdoor sound; going back out is as
    fast as before; F3 entities drop indoors and main ms falls.
- Verdict per gate: keep (on by default), maybe, reject; plus a feel for the panel's numbers with all on vs all off.
- Measured 2026-11-17 (perf-tests, interleaved, ledger perf): **faster** `select.gate` (walk -0.44 ms) and
  `shadow.lamp.face_cull` (night walk -0.46): a "keep" turns these on by default. Inconclusive: skin skip, matctrl gate,
  day gate, sun gate (night), disable_exterior (bench starts indoors). Lamp cache: render +0.94 ms on the night walk.
- Status: confirmed 2026-11-17 (owner played `--scene` + every gate on, `tunables/perf-play-2026-11-17.toml`, profiling exe, DLSS auto = Performance: "very smoothly and very fast", 110 fps in Balmora many times): all on by default, lamp cache stays off. Source: orch-presets, perf-tests.
