# Bevy 0.20: shadow maps, light views, clustering, probes

Facts only, from the stock 0.20.0 source (`~/.cargo/registry/src/*/<crate>-0.20.0/`); paths are
`<crate>/src/...:line`. Patch rows from `superfargoth/vendor/README.md`. Main-world caster lists per light:
[render-and-visibility.md](render-and-visibility.md) section 3.

## 1. Cascaded (directional) shadows

- `CascadeShadowConfig { bounds: Vec<f32>, overlap_proportion, minimum_distance }`
  (`bevy_light/src/cascade.rs:26`), built from `CascadeShadowConfigBuilder` (:60; defaults :136, cascade count
  depends on target, comments cite Unity/UE5/Godot distances). `build_directional_light_cascades` (:196) recomputes
  each cascade's frustum per view from camera projection and light direction.
- `DirectionalLightShadowMap { size }` default 2048 (`bevy_light/src/directional_light.rs:198-204`).
  `MAX_CASCADES_PER_LIGHT = 4`; `MAX_DIRECTIONAL_LIGHTS` = 10 (storage buffers) or 1
  (`bevy_pbr/src/render/light.rs:230-242`).
- Each cascade is one shadow view with `LightEntity::Directional { cascade_index }` (`light.rs:1037`). The depth
  texture is one array texture per view (`light.rs:1532`); shadow-map counts are capped by
  `max_texture_array_layers` (:1253-1267).
- Filtering: `ShadowFilteringMethod` = `Hardware2x2`, `Gaussian`, `Temporal` (`bevy_light/src/lib.rs:291`).
  Patches (vendor README): own world-unit PCSS (H2) and sun PCF receiver-plane fix (RO-11).

## 2. Point and spot shadows

- `PointLightShadowMap { size }` default 1024 (`bevy_light/src/point_light.rs:184-189`). Point lights get six
  shadow views (`LightEntity::Point { face_index }`, `light.rs:1044`; `RetainedViewEntity::new(light, None, face)`
  :1641-1643); a spot light gets one (`LightEntity::Spot`).
- `prepare_lights` (`light.rs:1051`): sorts clusterable lights with shadow-enabled first (:1271-1291), takes
  `point_light_shadow_maps_count` and `spot_light_shadow_maps_count` (:1229-1267), allocates one cube-array depth
  texture of `count * 6` layers (:1484-1490) and one for spots, through `TextureCache` (:1484). Lights past the
  count or the texture-array-layer limit get no shadow. A light with `shadow_maps_enabled == false` despawns
  its view entities (:1593-1598).
- Visible casters per face come from `CubemapVisibleEntities` (`light.rs:532`), extracted in `extract_lights`
  (`light.rs:333`); `queue_shadows` (`light.rs:2670`) bins them per `ShadowBatchSetKey` (:2819) into the view's shadow
  phase; shadow passes run through the same retained-bin machinery as camera views (section 5 of
  [render-and-visibility.md](render-and-visibility.md)).
- Early shadow-view cull: `check_point_light_mesh_visibility` skips lights not visible from any view
  (`bevy_light/src/lib.rs:517`).

## 3. What is retained across frames (stock)

- Shadow depth targets come from `TextureCache`, which keeps a texture while used within 3 frames
  (`bevy_render/src/texture/texture_cache.rs:91-99`); the contents are not reused as a cache: each shadow pass
  begins with `depth_attachment.get_attachment(StoreOp::Store)` (`light.rs:3050`) and clears on first use, so
  every shadow view with a phase is redrawn every frame.
- Retained per view: bins and `RetainedViewEntity` keyed caches (specialization, entity-to-bin), not the map.

## 4. Vendored per-face shadow cache patch

- Own fork of #26149 + #26161 (`superfargoth/vendor/README.md:11,40-55`): `Core3dShadows` runs a slim schedule
  (GPU mesh preprocessing + shadow passes) per point/spot shadow view instead of the whole `Core3d` camera schedule;
  `SkipRootViewRendering` hashes, per point/spot light, its visible casters, view/projection and queued entity count,
  and a view with unchanged inputs is skipped and keeps last frame's map. Directional lights are not covered.
  `BinnedRenderPhase::entity_count` added to `bevy_render`.
- On top (README rows): shadow early-out and lamp filter (B2), `ShadowMoverRefresh` (SH-20, redraws mover faces within
  16 m), CPU-listed shadow views (`CpuListedPointShadowCasters`), `AmbientLightSpecular` (N1). Behaviour of
  these in Scrib: [engine/shadows.md](../engine/shadows.md).

## 5. Clustering

- Clusters are a froxel grid per view; `ClusterConfig` default `FixedZ { total: 4096, z_slices: 24 }`, first slice
  depth 5.0, far Z = max clusterable range (`bevy_light/src/cluster/mod.rs:107,297-304,289-294`). Other variants:
  `None`, `Single`, `XYZ`.
- GPU clustering is used when the adapter supports compute and storage buffers, else CPU clustering
  (`bevy_pbr/src/cluster/mod.rs:79-100`); GPU path rasterizes per-object Z slices into count/index lists
  (`cluster/gpu.rs:1-40`, `extract_clusters_for_gpu_clustering` :1559). CPU path:
  `extract_clusters_for_cpu_clustering` (`cluster/mod.rs:394`), `prepare_clusters_for_cpu_clustering` (:478).
  With uniform buffers only `MAX_UNIFORM_BUFFER_CLUSTERABLE_OBJECTS = 204` fit (:31).
- Main-world ordering sets: `UpdateBounds`, `AssignLightsToClusters`, `UpdateDirectionalLightCascades`,
  `UpdateLightFrusta` (`bevy_light/src/lib.rs`, `SimulationLightSystems`).

## 6. Probes and environment maps

- `LightProbe` (`bevy_light/src/probe.rs:75`) marks a box volume; `EnvironmentMapLight` (:107, diffuse +
  specular cubemaps), `GeneratedEnvironmentMapLight` (:269), `IrradianceVolume` (:338). A view uses at most
  `MAX_VIEW_LIGHT_PROBES = 8` reflection probes and 8 irradiance volumes
  (`bevy_pbr/src/light_probe/mod.rs:52,125-129`). Shaders: `light_probe/*.wesl`.
- Patch (README): DDGI probe sampling (GI) replaces irradiance-volume/env-map diffuse inside a `GiProbeVolumeView`;
  atmosphere static LUT cache.

Scrib uses this in [shadows](../engine/shadows.md) and [lights](../engine/lights.md).
