//! Object-terrain blending (terrain.md "Object-terrain blending"): placed
//! objects marked by the host blend their albedo, roughness, AO and normal
//! into the terrain's own surface near their base, over a per-model band.
//!
//! The material is Bevy's `StandardMaterial` extended with the terrain's
//! bindings (100..112, shared with [`TerrainMaterial`]) and the band (113):
//! [`GroundBlendMaterial`], fragment `shaders/ground_blend.wesl`. The
//! prepass and shadow passes stay `StandardMaterial`'s.
//!
//! Hosts mark the parts to blend with their own component `M` and add
//! [`GroundBlendPlugin::<M>`]: each frame the plugin swaps marked parts of
//! the terrain's worldspace to a [`GroundBlendMaterial`] (one per source
//! material and band, cached), and drops the marker from parts of other
//! worldspaces (interiors share coordinates with the terrain, so they must
//! never sample it) and from blended (transparent) materials.

use std::collections::HashMap;
use std::marker::PhantomData;

use bevy_app::{App, Plugin, PostUpdate};
use bevy_asset::{Asset, AssetId, Assets, Handle, embedded_asset};
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_material::{AlphaMode, OpaqueRendererMethod};
use bevy_math::Vec4;
use bevy_pbr::{ExtendedMaterial, MaterialExtension, MaterialPlugin, MeshMaterial3d, StandardMaterial};
use bevy_reflect::TypePath;
use bevy_render::render_resource::AsBindGroup;
use bevy_render::storage::ShaderBuffer;
use bevy_shader::ShaderRef;
use sf_world::WorldspaceId;

use crate::{TerrainMaterial, TerrainState, TerrainSystems};

pub const GROUND_BLEND_SHADER: &str = "embedded://sf_terrain_bevy/shaders/ground_blend.wesl";

/// How strongly the terrain layers' height maps break up the blend's edge
/// (`ground_blend.wesl`: 0 = a plain ramp over the band).
pub const EDGE_BREAKUP: f32 = 0.5;

/// A `StandardMaterial` that blends into the terrain near the ground.
pub type GroundBlendMaterial = ExtendedMaterial<StandardMaterial, GroundBlendExt>;

/// The terrain's bindings (same handles as its [`TerrainMaterial`]) plus the
/// band. Not bindless: blended parts draw in their own batches.
#[derive(Asset, TypePath, AsBindGroup, Clone, Debug, Default)]
pub struct GroundBlendExt {
    #[storage(100, read_only)]
    pub params: Handle<ShaderBuffer>,
    #[texture(101, dimension = "2d_array", sample_type = "float", filterable = false)]
    pub heights: Handle<Image>,
    #[texture(102, dimension = "2d_array", sample_type = "u_int")]
    pub layer_ids: Handle<Image>,
    #[texture(103, dimension = "2d_array")]
    pub layer_weights: Handle<Image>,
    #[texture(104, dimension = "2d_array")]
    #[sampler(105)]
    pub macro_albedo: Handle<Image>,
    #[texture(106, dimension = "2d_array")]
    #[sampler(107)]
    pub albedo_height: Handle<Image>,
    #[texture(108, dimension = "2d_array")]
    pub normal_ao_roughness: Handle<Image>,
    #[texture(109, dimension = "2d_array")]
    #[sampler(110)]
    pub normals: Handle<Image>,
    #[texture(111)]
    #[sampler(112)]
    pub far_map: Handle<Image>,
    /// x: band height (m above the terrain where the blend ends), y: edge
    /// break-up ([`EDGE_BREAKUP`]).
    #[uniform(113)]
    pub band: Vec4,
}

impl GroundBlendExt {
    /// The terrain's bindings with a band of `band` meters.
    pub fn new(terrain: &TerrainMaterial, band: f32) -> Self {
        Self {
            params: terrain.params.clone(),
            heights: terrain.heights.clone(),
            layer_ids: terrain.layer_ids.clone(),
            layer_weights: terrain.layer_weights.clone(),
            macro_albedo: terrain.macro_albedo.clone(),
            albedo_height: terrain.albedo_height.clone(),
            normal_ao_roughness: terrain.normal_ao_roughness.clone(),
            normals: terrain.normals.clone(),
            far_map: terrain.far_map.clone(),
            band: Vec4::new(band, EDGE_BREAKUP, 0.0, 0.0),
        }
    }
}

impl MaterialExtension for GroundBlendExt {
    fn fragment_shader() -> ShaderRef {
        GROUND_BLEND_SHADER.into()
    }
}

/// Swaps the host's marked parts (component `M`) to [`GroundBlendMaterial`].
/// `read` returns a marked part's worldspace and band (meters).
pub struct GroundBlendPlugin<M: Component> {
    pub read: fn(&M) -> (&WorldspaceId, f32),
}

impl<M: Component> Plugin for GroundBlendPlugin<M> {
    fn build(&self, app: &mut App) {
        // Once per app, whatever the number of hosts (shader libraries:
        // `TerrainPlugin`). Without this plugin the material doesn't exist.
        if !app.is_plugin_added::<MaterialPlugin<GroundBlendMaterial>>() {
            embedded_asset!(app, "shaders/ground_blend.wesl");
            app.add_plugins(MaterialPlugin::<GroundBlendMaterial>::default()).init_resource::<GroundBlendCache>();
        }
        app.insert_resource(GroundBlendRead::<M>(self.read, PhantomData))
            .add_systems(PostUpdate, swap_materials::<M>.after(TerrainSystems::Update));
    }
}

#[derive(Resource)]
struct GroundBlendRead<M: Component>(fn(&M) -> (&WorldspaceId, f32), PhantomData<M>);

/// Blended materials by (source material, band bits), all reading the
/// terrain material `terrain`'s bindings.
#[derive(Resource, Default)]
pub struct GroundBlendCache {
    terrain: Option<AssetId<TerrainMaterial>>,
    materials: HashMap<(AssetId<StandardMaterial>, u32), Handle<GroundBlendMaterial>>,
    frames: u32,
    /// Parts swapped, and markers dropped (other worldspace; transparent).
    pub swapped: u64,
    pub other_worldspace: u64,
    pub transparent: u64,
}

impl GroundBlendCache {
    /// Number of blended materials alive.
    pub fn len(&self) -> usize {
        self.materials.len()
    }

    pub fn is_empty(&self) -> bool {
        self.materials.is_empty()
    }
}

/// Frames between sweeps of cache entries whose source material is gone
/// (its model unloaded).
const PRUNE_FRAMES: u32 = 600;

#[allow(clippy::too_many_arguments)]
fn swap_materials<M: Component>(
    mut commands: Commands,
    read: Res<GroundBlendRead<M>>,
    state: Option<Res<TerrainState>>,
    terrain: Res<Assets<TerrainMaterial>>,
    standard: Res<Assets<StandardMaterial>>,
    mut blended: ResMut<Assets<GroundBlendMaterial>>,
    mut cache: ResMut<GroundBlendCache>,
    marked: Query<(Entity, &M, &MeshMaterial3d<StandardMaterial>)>,
) {
    // Parts wait (as plain StandardMaterial) until a terrain exists.
    let Some(state) = state else { return };
    let Some(source) = terrain.get(&state.material) else { return };
    let cache = &mut *cache;
    if cache.terrain != Some(state.material.id()) {
        // A new terrain (worldspace or bake changed): rebind every cached
        // material in place, so blended parts follow without a re-swap.
        cache.terrain = Some(state.material.id());
        for ((_, bits), handle) in &cache.materials {
            if let Some(mut m) = blended.get_mut(handle) {
                m.extension = GroundBlendExt::new(source, f32::from_bits(*bits));
            }
        }
    }
    cache.frames += 1;
    if cache.frames >= PRUNE_FRAMES {
        cache.frames = 0;
        cache.materials.retain(|(id, _), _| standard.contains(*id));
    }
    for (entity, marker, material) in &marked {
        let (ws, band) = (read.0)(marker);
        let mut e = commands.entity(entity);
        if *ws != state.ws || band <= 0.0 {
            if cache.other_worldspace == 0 {
                log::info!("ground blend: first part outside the terrain's worldspace ({ws:?} vs {:?})", state.ws);
            }
            cache.other_worldspace += 1;
            e.remove::<M>();
            continue;
        }
        let key = (material.id(), band.to_bits());
        let handle = if let Some(h) = cache.materials.get(&key) {
            h.clone()
        } else {
            let Some(base) = standard.get(material.id()) else { continue };
            if !matches!(base.alpha_mode, AlphaMode::Opaque | AlphaMode::Mask(_)) {
                cache.transparent += 1;
                e.remove::<M>();
                continue;
            }
            let mut base = base.clone();
            // `ground_blend.wesl` is a forward fragment.
            base.opaque_render_method = OpaqueRendererMethod::Forward;
            let h = blended.add(GroundBlendMaterial { base, extension: GroundBlendExt::new(source, band) });
            cache.materials.insert(key, h.clone());
            h
        };
        if cache.swapped == 0 {
            log::info!("ground blend: first part swapped (band {band} m)");
        }
        cache.swapped += 1;
        e.remove::<(M, MeshMaterial3d<StandardMaterial>)>().insert(MeshMaterial3d(handle));
    }
}
