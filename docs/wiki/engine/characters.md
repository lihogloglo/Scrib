# Characters

## What it does
An NPC or the player is one `Character` component. Off-thread tasks resolve its recipe and outfit, merge the outfit
into one skinned mesh per material, and the main thread spawns a rig, an `Animator` and the skinned meshes sharing
one joint palette. Identical looks share one cache entry. First person is a layer switch on the same meshes, not a
second body.

## How it works
- **Spawn flow** (task off-thread, upload on main): `superfargoth/crates/sf_character_bevy/src/lib.rs:1-52` (module
  doc), `upload` `:697`, rig `spawn_rig` `:988`. The body is a child with the race scale; meshes get
  `DynamicSkinnedMeshBounds` (`:899`, `:956`). An outfit swap keeps old meshes until the new ones are settled
  (rig and animator survive a swap on one skeleton).
- **Content seam**: the adapter implements `CharacterContent` (recipe, slots, cooked parts, view role).
  Morrowind: `bevyrrowind/crates/mw_actors_bevy/src/actors.rs:31` (`MwCharacters`), `:136` (`view_role`);
  records and slots in `bevyrrowind/crates/mw_actors/src/source.rs:88,126`, `slots.rs` (27 part slots).
- **Assembly** (Bevy-free): `superfargoth/crates/sf_character/src/assemble.rs:206` (`assemble`). Parts merge per
  material and `AssemblyPart::group`; `Shape` morphs are baked into vertices; a submesh with `Expression` morphs
  (blink, mouth) stays its own mesh on the same palette (`assemble.rs:1-19`). Optional body smoothing:
  `sf_character/src/smooth.rs` (off by default, `SmoothBodies` `lib.rs:184`).
- **Cache**: `OutfitCache` keyed by skeleton, outfit key, LOD, shape weights (`lib.rs:242,335`); entries nobody
  uses are evicted past `capacity`.
- **Skinning**: stock Bevy keeps one joint block per skinned mesh. Our vendored `bevy_pbr` patch **P5**
  (`superfargoth/vendor/bevy_pbr/src/render/skin.rs:145-423`, `SkinJointBlocks` `:197`) keys a block by
  (inverse-bindposes asset, joint entities) and refcounts it, so all meshes of one NPC extract and upload one
  palette (`extract_skins` `:534`, `prepare_skins` `:432`). Previous-frame matrices (motion vectors) stay valid at
  the same offset.
- **Roles** (first person): `sf_character/src/slots.rs:33` `ViewRole` (Body, Head, Arms, Held). A root with
  `RoleLayers` (`sf_character_bevy/src/roles.rs:78`) gets its outfit split per role; each part's `RenderLayers`
  follow the root rule (worldspace layer `W`, shadow-only `S`, both or neither). First person puts the head on
  `S` (hidden, still casting). A `ViewCopy` child of arm and held parts shares mesh, material and joints and is
  drawn by `ViewModelPlugin` (`view_model.rs`, FOV and depth slice in `sf_camera/src/view_model.rs`); it is
  `NotShadowCaster`, `NoRayTracing`. `sync_role_layers`, `follow_view_copies`: `lib.rs:473`.
- **Held items**: rigid props on a socket joint (`held.rs`); lights carry `[W, S]` via `ShadowLayers`.
- **Cloth and morph**: `cloth.rs` writes cloth joints in `AnimSystems::Apply` (`lib.rs:466`); `morph.rs` drives
  expressions after Apply (`:471`).

## Bevy features used
Skinned meshes and `SkinnedMesh`, `DynamicSkinnedMeshBounds`, `RenderLayers`, bindless `ExtendedMaterial`
(view model). Patched: skin joint blocks (P5). See [render and visibility](../bevy/render-and-visibility.md).

## Switches and counters
`--off actors` / `npcs` (`bevyrrowind/crates/mw_game_core/src/perf.rs:154`); `CharacterStats` (`lib.rs:360`);
console `flip viewmodel` (view model on and off). Owner scene: CH-5 (shared joint blocks), FP-1/FP-1b/FP-2.

## Known limits
- P5 expected 0.5-0.9 ms on the Balmora walk; needs a perf run ([perf ledger](../../ledger/perf.md)).
- First-person arms and view-model look unsigned (FP-1, FP-1b, FP-2 awaiting owner; no entry yet, see
  [animation ledger](../../ledger/animation.md)).
- Morph-bearing heads cost one extra draw and palette copy per character.

## Specs
[character.md](../../../superfargoth/docs/specs/character.md), [first-person.md](../../../superfargoth/docs/specs/first-person.md),
[character-shapes.md](../../../superfargoth/docs/specs/character-shapes.md)
