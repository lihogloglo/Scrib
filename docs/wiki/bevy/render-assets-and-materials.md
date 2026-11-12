# Bevy 0.20: render assets, images, materials, bindless, mesh slabs

Facts only, from the stock 0.20.0 source (`~/.cargo/registry/src/*/<crate>-0.20.0/`); paths are
`<crate>/src/...:line`. Vendored copies differ in line numbers; patches noted per section. Extraction and
visibility: [render-and-visibility.md](render-and-visibility.md).

## 1. RenderAsset: main world to GpuImage / RenderMesh

- Trait `RenderAsset` (`bevy_render/src/render_asset.rs:47`): `asset_usage` (:58), `byte_len` (:69),
  `prepare_asset(source, id, param, previous_asset: Option<&Self>)` (:76), `unload_asset` (:89), `take_gpu_data` (:100).
  `RenderAssetPlugin` (:135) adds `extract_render_asset` to `ExtractSchedule` and `prepare_assets` to
  `RenderSystems::PrepareAssets`.
- AssetEvent flow: `Assets<T>` queues events (`bevy_asset/src/assets.rs:376-394`), `asset_events` writes them as
  messages in `PostUpdate` (`bevy_asset/src/lib.rs:679`). `extract_render_asset` (`render_asset.rs:293`) reads them
  from the main world through a cached `SystemState` (:255): `Added`/`Modified` -> re-extract (`Modified` also
  into `modified`); `Unused` (last strong handle dropped) -> `removed`; `Removed` is ignored (:325).
- Extract copies the asset (`asset.clone()`) when usage has `RENDER_WORLD` (:349). If usage is exactly
  `RENDER_WORLD`, `take_gpu_data` moves the CPU data out of the main-world asset instead (:350). `GpuImage` takes
  `image.data` and refuses a second extract once `had_data` was true and data is gone
  (`bevy_render/src/texture/gpu_image.rs:39-56`).
- `prepare_assets` (`render_asset.rs:393`): first retries last frame's queue (`PrepareNextFrameAssets`), then
  drains `removed` (`unload_asset`), then prepares `extracted`. On update it removes the old entry first (:452), so
  users see no asset until the new one is ready. `RetryNextUpdate` re-queues (:439).
- Upload budget: `RenderAssetBytesPerFrame` / `Limiter` (:509-574): once `bpf.exhausted()`, assets with a
  `byte_len` wait for the next frame (:432), but at least one sized asset is written per frame.
- Device recovery: `RenderStartup` drains `RenderAssets<A>` into `RenderAssetsToReExtract` (:279).

## 2. GpuImage update path

- `GpuImage::prepare_asset` (`gpu_image.rs:63`). If `previous_asset` has an equal `texture_descriptor` and either
  the new image has no data or the old texture has `COPY_DST`, the texture is reused and data goes through
  `queue.write_texture` (:70-90). Otherwise a new texture (`create_texture_with_data`, :104).
- The texture view is reused only if descriptor and `COPY_DST` match and the view descriptor is unchanged
  (:149-157); a reused view keeps downstream bind groups valid.
- A sampler is created per image: `ImageSampler::Default` clones the default, `Descriptor` calls `create_sampler`
  every prepare (:164-169). Patch (vendor README row "shared image samplers"): opt-in `SharedImageSamplers`
  dedups equal descriptors (`texture/gpu_image.rs:21`).

## 3. Materials: erased asset, bind groups

- `MeshMaterial3d<M>` is an `ErasedRenderAsset` (`bevy_pbr/src/material.rs:1617`) added by
  `ErasedRenderAssetPlugin` (:450); its output is `PreparedMaterial { binding, properties }` (:1540). `prepare_asset`
  (:1647) calls `render_material_bindings.prepare_material` (:1675), then builds `MaterialProperties`
  (draw functions, alpha mode -> phase, `bindless`, shaders).
- Rebuild trigger: a material is prepared again only on an `AssetEvent` for the material asset (via the erased
  extract, `bevy_render/src/erased_render_asset.rs:135`). An image change does not re-prepare materials
  by itself; the bind group keeps the old `TextureView` unless `GpuImage` reused its view (section 2) or the
  material is modified. A material whose image is not yet in `RenderAssets<GpuImage>` returns `RetryNextUpdate`
  (`bevy_render/src/render_resource/bind_group.rs:112,648`) and is retried every frame.
- `prepare_material` (`bevy_render/src/material_bind_groups.rs:2762`): `build_bind_group` into a reused
  `BindGroupBuilder`; for an existing id it tries `try_update_data` (data-only fast path, :2796); if any GPU
  resource changed it frees and reallocates the slot (:2805-2810). `CreateBindGroupDirectly` builds a whole
  bind group per material (:2821).
- Bind groups themselves are created in `PrepareBindGroups`: `prepare_material_bind_groups` then
  `write_material_bind_group_buffers` (:528-546, 2697).

## 4. Bindless

- A material is bindless when `M::bindless_slot_count()` is `Some`, `bindless_supported(device)`, and the resolved
  count > 1 (`material_bind_groups.rs:2198`). `StandardMaterial` declares `#[bindless(index_table(range(0..31)))]`
  (`bevy_pbr/src/pbr_material.rs:25`); `ExtendedMaterial` takes the smaller limit of base and extension
  (`bevy_render/src/combined_bind_group.rs:18-40`). Non-bindless: one bind group per material
  (`MaterialBindGroupNonBindlessAllocator`, :2367).
- Bindless layout: slabs (`MaterialBindlessSlab`, :121) each hold binding arrays of textures/samplers/buffers, an
  index table per material slot, and data buffers (:94, 169, 186). A slab's single bind group serves up to
  `slab_capacity` resources; a material goes in the first slab that has the resources or free slots (:1071,
  `check_allocation` dedups equal resources). Slot ids are 16-bit, so a slab spills past 0xFFFF (:1105).
- Any allocate or free sets `bind_group = None` (:1126, :1444); `prepare_bind_group` (:1640) rebuilds the whole slab's bind
  group on the next `PrepareBindGroups` when it is `None`. Changed shader buffers also invalidate (:1727).
- Patch: #26321 (`ExtendedMaterial` bindless fallback), vendor README.

## 5. Mesh allocator (slabs)

- `MeshAllocator` (`bevy_render/src/mesh/allocator.rs:48`) over a `SlabAllocator` (`slab_allocator.rs`). Element
  classes: Metadata, Vertex, Index, MorphTarget (:156-166). Settings: min slab 1 MiB, max 512 MiB, large
  256 MiB, growth 1.5 (`slab_allocator.rs:183-194`). Vertex slabs are per vertex layout; without `BASE_VERTEX`
  each vertex array gets its own slab (`allocator.rs:459-475`).
- `allocate_and_free_meshes` (:249), in `PrepareAssets` before `prepare_assets::<RenderMesh>` (:205-209): free every
  removed/modified/added mesh (:629-650), then allocate all extracted meshes and `commit` (:420, :497). Growth
  reallocates the slab with a GPU copy; keys displaced by growth are listed for the frame
  (`meshes_displaced_by_slab_growth`, :298) so caches of those buffers rebuild.
- Lookups: `mesh_vertex_slice` / `mesh_index_slice` (:309, :320).

Scrib uses this in [streaming](../engine/streaming.md) and [statics and LOD](../engine/statics-and-lod.md).
