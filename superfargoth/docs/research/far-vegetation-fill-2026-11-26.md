# Far vegetation fill cost: industry options and Bevy 0.20 behaviour (2026-11-26)

Summary. Shipped engines do not shade distant vegetation into the full-resolution shadow cascades. The standard moves are: turn "cast shadow" off on the far LODs and impostors (UE per-LOD setting), hand far shadows to a different system (UE distance field shadows or a far cascade, Horizon Zero Dawn's heightfield distance cascade), or use a separate cheap shadow caster mesh (HZD keeps shadow casters apart from visual meshes; UE "foliage shadow impostor"). Overdraw of alpha-tested foliage is fixed by trimming card hulls (Persson), by a masked-only depth prepass followed by a depth-tested main pass (UE early-Z masking), and by real geometry instead of cards near the camera. Hashed alpha and alpha to coverage fix quality, not fill. In Bevy 0.20 an `AlphaMode::Mask` material runs its fragment shader in the depth prepass, in every shadow cascade and again in the main pass, because the main pass compares `GreaterEqual` and still discards. There is no per-cascade caster mask, and `unlit` does not stop shadow casting. So options (a) `NotShadowCaster` on cards and (d) opaque leaves are plain Bevy features; (b) needs a custom material; (c) is the only one that restores early Z in the main pass.

## Part 1: industry

| Option | Standard technique | Who ships it | Source | What it buys |
|---|---|---|---|---|
| (a) cards cast no shadow | Per-LOD "cast shadow" off (far LODs and billboards); far shadows come from distance field shadows or a far cascade instead | Unreal (static mesh LOD setting; Distance Field Shadows beyond Dynamic Shadow Distance; Far Shadow Cascades for landscape) | Epic, "Using Distance Field Shadows in Unreal Engine", https://dev.epicgames.com/documentation/unreal-engine/using-distance-field-shadows-in-unreal-engine ; per-LOD advice is forum-level only (e.g. https://forums.unrealengine.com/t/foliage-have-no-shadow-in-distance/490977), no official page found | Removes the whole far-tree cost from every cascade. Costs: far trees lose their shadows unless a second system supplies them. UE forum notes impostors can self-shadow wrongly under distance field shadows (unverified, forum report) |
| (a) alt: shadow proxy / far-shadow system | Distant shadows from a heightfield cascade, and shadow casters kept separate from visual meshes, one fixed shadow LOD per cascade | Guerrilla, Horizon Zero Dawn | G. Sanders, "Between Tech and Art: The Vegetation of Horizon Zero Dawn", GDC 2018, https://www.gdcvault.com/play/1025530 . Details (cascades 0 and 1 use shadow maps for vegetation, distance cascade is heightfield based, shadow casters separate, ~43:09 and ~46:47) come from an AI-generated summary, https://videohighlight.com/v/wavnKZNSYqU : unverified against the talk | Shows a shipped open-world answer: vegetation casts only in near cascades, far shadow is not per-tree |
| (a) alt: shadow impostor | Static low-cost shadow-only impostor mesh replaces the real foliage as caster | Unreal (Nanite foliage shadow imposter, research paper) | "Unreal Engine Nanite foliage shadow imposter", SPIE 12784, 2023, https://nanolithography.spiedigitallibrary.org/conference-proceedings-of-spie/12784/127842E/Unreal-engine-nanite-foliage-shadow-imposter/10.1117/12.2692451.full (abstract only read) | Keeps shadows with a much cheaper caster |
| (b) cheap fragment shader | Impostor shader samples baked atlas frames; blends the 3 nearest views (mobile falls back to 1 frame) | Unreal ImpostorBaker (Epic plugin) | Epic, "Impostor Baker Plugin in Unreal Engine", https://dev.epicgames.com/documentation/en-us/unreal-engine/impostor-baker-plugin-in-unreal-engine : "pixel shader is more expensive for imposters" than billboards, but one 8-triangle card replaces 8 cards (72 triangles) and billboards have worse masked overdraw. Original write-up R. Brucks, "Octahedral Impostors", shaderbits.com, 2018 (cited by others; not fetched, unverified) | Epic's own numbers: none published. A single-frame lookup is the cheap end; our flat card is already cheaper than a 3-frame blend per pixel. No published ms figures found |
| (c) opaque or trimmed geometry | Depth prepass that evaluates the mask, then base pass with depth test; trim cards to the visible shape | Unreal: r.EarlyZPassOnlyMaterialMasking ("Mask material only in early Z-pass", needs r.EarlyZPass=2); Avalanche tool that fits a tight polygon around visible texels (4 verts keep ~60 percent of the quad area, 8 keep ~52 percent, per slides) | UE cvar: community cvar wiki https://indxzero.github.io/ue544cvarwiki/articles/r.earlyzpassonlymaterialmasking (not an Epic page; known flicker bugs, UE-65185). Persson, "Graphics Gems for Games: Findings from Avalanche Studios", https://www.humus.name/Articles/Persson_GraphicsGemsForGames.pdf (shown for particles, same maths for foliage cards) | Early Z skips shading of hidden pixels; trimming cuts the pixels rasterised at all (about 40 to 48 percent fewer for a 4 to 8 vertex hull) |
| (d) opaque leaves / no shadow on trees | Near LODs have real leaf geometry; far LODs and billboards cast no shadow; Horizon splits per-cascade shadow LOD | Guerrilla (see above); Unreal per-LOD cast shadow | as above | Removes alpha test from the shadow pass for those objects |
| Hashed alpha | Per-pixel stable noise threshold | Not tied to a shipped title in what I found | Wyman and McGuire, "Hashed Alpha Testing", I3D 2017, https://research.nvidia.com/labs/rtr/publication/wyman2017hashed | Quality only: keeps distant geometry from vanishing and avoids flicker. It still runs the alpha fetch and discard, so no fill saving. The cost was not stated in what I read |
| Alpha to coverage | Alpha becomes MSAA coverage mask | Common with MSAA | see Bevy (c) below | Quality only; needs MSAA, so not our DLSS path |
| GTA V / RDR2 | 4 cascade shadow maps (RDR2 per a third-party analysis); vegetation tagged by material id | Rockstar | Courrèges, "GTA V Graphics Study", 2015, https://www.adriancourreges.com/blog/2015/11/02/gta-v-graphics-study/ (cascade and tree sections not read, unverified) | No finding on far-tree shadows |

Not verified at all: Far Cry 5 and Ghost Recon Wildlands vegetation talks, SpeedTree docs, any published millisecond cost for an octahedral impostor shader.

## Part 2: Bevy 0.20 source (vendored, `superfargoth/vendor/`)

- a. Main pass depth: `depth_compare: GreaterEqual` (reverse Z) in `bevy_pbr/src/render/mesh.rs:4117`; depth write is on for Opaque and Mask (`mesh.rs:3893-3903`, `4116`). It is not `Equal`, so prepass depth rejects farther fragments but equal and nearer ones are shaded. The depth texture is cleared at first use (the prepass) and loaded afterwards (`bevy_render/src/texture/texture_attachment.rs:286-292`; main node `bevy_core_pipeline/src/core_3d/main_opaque_pass_3d_node.rs:65`). A Mask material still discards in the main pass: `MAY_DISCARD` is set (`bevy_pbr/src/material.rs:809`) and `alpha_discard` runs (`render/pbr_functions.wesl:100-121`). So overdraw behind the prepass depth is culled early, but every fragment that survives is shaded again with the alpha fetch.
- b. Yes. A prepass pipeline with `MAY_DISCARD` keeps a fragment shader (`prepass/mod.rs:408-416, 649`), which samples alpha and discards (`render/pbr_prepass_functions.wesl:20-88`, called from `pbr_prepass.wesl:55`). Shadow passes go through the same prepass pipeline. The cheaper variant is only for non-discarding materials: depth-only opaque pipelines have no fragment shader at all (`is_depth_only_opaque_prepass`, `prepass/mod.rs:413`; `material.rs:235`). There is no cheaper discard variant.
- c. No hashed or stochastic alpha test found in `bevy_pbr` (grep of shaders and `material.rs`; absence not exhaustively proven). `AlphaMode::AlphaToCoverage` (`bevy_material-0.20.0/src/alpha.rs:47`) needs MSAA; with `Msaa::Off` it falls back to plain Mask discard (`material.rs:810-812`; `pbr_functions.wesl:109-114`).
- d. Yes. `unlit` only sets a material flag (`pbr_material.rs:1122`); shadow casting depends on `NotShadowCaster` and the material's shadow settings, not on `unlit`. Mask discard still runs in the shadow pass for an unlit material.
- e. Only all or nothing. `NotShadowCaster` (`bevy_light-0.20.0/src/lib.rs:264`) is filtered in the visibility systems and the instance batching (`gpu_instance_batch/mod.rs:1054`); I found no per-cascade mask. Casters are tested against every cascade frustum (project wiki `docs/wiki/bevy/render-and-visibility.md:40`). A per-cascade cut would need a patch in the vendored `bevy_pbr`. `CascadeShadowConfig` bounds only set cascade ranges.

## Part 3: can a material give a masked prepass and a depth-equal, non-discarding main pass? (bigworld-5, 2026-11-26)

Code-read only (vendored `bevy_pbr` 0.20, paths under `superfargoth/vendor/bevy_pbr/src/`); nothing built, nothing
measured. **Answer: yes, through `Material::specialize`; no patch to the vendored `bevy_pbr`.**

- **The hook sees the finished descriptor of both pipelines.** `Material::specialize(pipeline, descriptor, layout, key)`
  (`material.rs:301`) is stored as `user_specialize` and called last, after `MeshPipeline::specialize`, on the main
  pass descriptor (`material.rs:687-689`) and on the prepass descriptor (`prepass/mod.rs:400-407`), which shadow
  views share. `StandardMaterial::specialize` itself edits `descriptor.primitive.cull_mode` and
  `descriptor.depth_stencil.bias` there (`pbr_material.rs:1627, 1642`), so changing the depth-stencil state in the
  hook is the path Bevy uses, not a reach into private state (rule 4 does not apply).
- **Main pass.** Stock state is `depth_write_enabled: true`, `depth_compare: GreaterEqual` for opaque and mask
  (`render/mesh.rs:3897-3903, 4114-4117`). The hook can set `depth_compare = Equal` and `depth_write_enabled =
  false`. `MAY_DISCARD` in the main pass is only a shader def (`render/mesh.rs:3993`): a material with its own
  fragment shader discards only if that shader does, so "non-discarding" is our shader's choice. `AlphaMode::Mask`
  still routes the item to the alpha-mask phases (`material.rs:1909`; prepass `prepass/mod.rs:1284-1291`).
- **Prepass.** A `Mask` material with its own `prepass_fragment_shader` keeps a fragment stage
  (`prepass/mod.rs:647-652`), so the mask-only prepass is a shader we write (alpha fetch, discard, motion vector).
  On the DLSS-path camera the motion-vector target needs a fragment stage for every material anyway.
- **Telling the pipelines apart.** The key cannot: prepass keys and shadow keys both carry `DEPTH_PREPASS`
  (`prepass/mod.rs:1003`, `precache.rs:239`), as does the main pass key of a view that has a depth prepass
  (`render/mesh.rs:445-447`). The descriptor can: prepass and shadow descriptors carry the `PREPASS_PIPELINE` def
  and the label `prepass_pipeline` (`prepass/mod.rs:447, 724`); the main one is `opaque_mesh_pipeline`
  (`render/mesh.rs:3897`). Rule for the hook: switch to `Equal` only when the descriptor is not a prepass one and
  `key.mesh_key` has `DEPTH_PREPASS`. A view without a depth prepass keeps the stock state, and the fragment shader
  keeps its discard under `@if(!DEPTH_PREPASS)` (the def reaches the main shader, `render/mesh.rs:3910-3912`).
- **Conditions for the equal test to hold.**
  1. Both vertex stages give bit-identical clip positions: the same expressions and `@invariant` on the position
     output. Bevy's own shaders have no `@invariant` (grep of `bevy_pbr`); ours already do it for this reason
     (`sf_lod_bevy/src/shaders/impostor.wesl:24`, `impostor_prepass.wesl:5-6, 17`; terrain the same), so naga and
     the WESL front end accept it.
  2. The prepass writes the rasterised depth. A per-pixel `frag_depth` (impostor depth offset) breaks it: the main
     pass would have to write the same value, which turns early Z off and shades every masked-out fragment again.
     Today's `ImpostorMaterial` writes `frag_depth` only for emulated unclipped depth in shadow views
     (`impostor_prepass.wesl:73, 92`), so its camera prepass qualifies. BW-4's "depth" (spec "Slices") must stay
     out of `frag_depth` on the camera path, or the equal test is lost.
  3. The mesh is in the prepass of every view where the main pass tests `Equal`: `enable_prepass()` true (the only
     skip, `prepass/mod.rs:1204`) and forward rendering.
  4. Fade by dither discards in the prepass only; the main pass then fails the equal test on those pixels by
     itself.
- **Not traced:** with `OcclusionCulling`, whether the main pass can draw an instance that neither the early nor
  the late prepass drew. With `Equal` such an instance would be missing for that frame (stock draws it). Check in
  the first slice that builds this (a counter, not a picture).
- **Existing code this applies to.** `ImpostorMaterial` (`sf_lod_bevy/src/materials.rs:60-97`) already overrides
  `specialize` (vertex layout only) and still discards in the main pass (`impostor.wesl:64`) against
  `GreaterEqual`: it is the stock double alpha test. The change there, or in BW-4's slot impostor material, is a
  few lines in the hook and one `@if` around the discard.
- **Upstream:** one search of bevyengine/bevy issues and PRs found none for an equal-depth main pass after a depth
  prepass; not exhaustive.
- **Cost expectation, unchanged and unmeasured:** BW-2b's `stand/cards.opaque` main span (0.16 ms against 2.73 ms
  masked) is the proxy for the main pass; the masked prepass stays near 1.0 ms. A bench arm prices it (queue
  slice (c), on meta's go).
