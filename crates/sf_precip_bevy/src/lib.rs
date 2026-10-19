//! Precipitation (weather.md "Precipitation"; as built: "WE3 as built
//! (part 2b)").
//!
//! Rain streaks and snow/ash flakes in three nested camera-centered boxes
//! (8, 24, 64 m). Each box is one draw of a fixed mesh of quads; a drop's
//! position is closed-form in the vertex shader (`p = wrap(seed · size +
//! offset, camera box)`), the offset being the velocity (fall + wind)
//! integrated on the CPU and wrapped. No simulation, no spawn/despawn, no
//! per-drop CPU work, so no particle crate (`bevy_hanabi` is not on Bevy
//! 0.20 yet; weather.md "Crates").
//!
//! The host (the weather plugin) writes [`Precip`]: up to two kinds at once
//! ([`PrecipSlot`], they crossfade by intensity), the wind, the brightness.
//! The draw follows whatever camera renders it (`view.world_position`).
//!
//! Not built: the occlusion map (roofs and trees don't block yet: hosts
//! switch [`Precip::enabled`] off indoors), splashes, textures (procedural
//! shapes). The streaks fog themselves from [`TranslucentFog`] (render-order.md
//! rule 1, RO-3): dimmed by the transmittance to their distance plus the
//! in-scatter in front of them.

use bevy_app::{App, Plugin, Update};
use bevy_asset::{Asset, Assets, Handle, RenderAssetUsages, embedded_asset};
use bevy_camera::visibility::{NoFrustumCulling, Visibility};
use bevy_ecs::prelude::*;
use bevy_light::NotShadowCaster;
use bevy_material::AlphaMode;
use bevy_math::{Vec2, Vec3, Vec4};
use bevy_mesh::{Indices, Mesh, Mesh3d, MeshVertexAttribute, MeshVertexBufferLayoutRef, PrimitiveTopology, VertexFormat};
use bevy_pbr::{Material, MaterialPipeline, MaterialPipelineKey, MaterialPlugin, MeshMaterial3d};
use bevy_reflect::TypePath;
use sf_render_order_bevy::{DrawBand, RenderOrderPlugin, TranslucentFog};
use bevy_render::render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderType, SpecializedMeshPipelineError};
use bevy_shader::ShaderRef;
use bevy_time::Time;

mod occlusion;
pub use occlusion::{NoPrecipOcclusion, PrecipOcclusion, PrecipOcclusionConfig, PrecipOcclusionStats};

#[cfg(test)]
mod tests;

pub const PRECIP_SHADER: &str = "embedded://sf_precip_bevy/shaders/precip.wesl";

/// Per drop vertex: (rank, size t, phase, 0).
pub const ATTRIBUTE_DROP: MeshVertexAttribute = MeshVertexAttribute::new("Precip_Drop", 988_540_930, VertexFormat::Float32x4);

/// The nested boxes: (edge m, drops at intensity 1).
pub const LAYERS: [(f32, u32); 3] = [(8.0, 3000), (24.0, 6000), (64.0, 8000)];

/// Kinds drawn at once (a crossfade needs two).
pub const SLOTS: usize = 2;

/// Streak length, seconds of travel (a motion-blur look).
pub const STREAK_S: f32 = 0.035;

/// How a kind draws.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PrecipShape {
    /// Stretched along the velocity (rain).
    #[default]
    Streak,
    /// Soft discs that flutter (snow, ash, dust).
    Flake,
}

/// One precipitation kind being drawn.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PrecipSlot {
    pub shape: PrecipShape,
    /// Fraction of the drops shown, `[0, 1]`.
    pub intensity: f32,
    /// m/s.
    pub fall_speed: f32,
    /// Drop width range, meters.
    pub size: [f32; 2],
    /// Linear rgb and opacity, before [`Precip::brightness`].
    pub color: Vec4,
    /// Flutter amplitude, meters (flakes).
    pub flutter: f32,
    /// Share of the wind the drops move with, `[0, 1]`.
    pub wind_follow: f32,
}

impl Default for PrecipSlot {
    fn default() -> Self {
        Self {
            shape: PrecipShape::Streak,
            intensity: 0.0,
            fall_speed: 7.0,
            size: [0.002, 0.004],
            color: Vec4::new(0.55, 0.6, 0.65, 0.3),
            flutter: 0.0,
            wind_follow: 1.0,
        }
    }
}

/// What falls (written by the host every frame or on change).
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct Precip {
    /// Off indoors (no occlusion map yet) and for `--off precip`.
    pub enabled: bool,
    pub slots: [Option<PrecipSlot>; SLOTS],
    /// Horizontal wind, m/s (x, z).
    pub wind: Vec2,
    /// Light on the drops (sky ambient, day/night; lightning adds).
    pub brightness: f32,
}

impl Default for Precip {
    fn default() -> Self {
        Self {
            enabled: true,
            slots: [None; SLOTS],
            wind: Vec2::ZERO,
            brightness: 1.0,
        }
    }
}

/// `precip.wesl`'s `PrecipParams`.
#[derive(Clone, Copy, Debug, Default, PartialEq, ShaderType)]
pub struct PrecipParams {
    pub volume: Vec4,
    pub offset: Vec4,
    pub velocity: Vec4,
    pub look: Vec4,
    pub color: Vec4,
}

/// One nested box of one kind.
#[derive(Asset, TypePath, AsBindGroup, Clone, Debug)]
pub struct PrecipMaterial {
    #[uniform(0)]
    pub params: PrecipParams,
    /// [`TranslucentFog::params`] (render-order.md rule 1; kept current by the plugin).
    #[uniform(1)]
    pub fog_params: Vec4,
    /// [`TranslucentFog::volume`].
    #[texture(2, dimension = "3d")]
    #[sampler(3)]
    pub fog_volume: Handle<bevy_image::Image>,
    /// [`PrecipOcclusion::clip_from_world`].
    #[uniform(4)]
    pub occ_clip: bevy_math::Mat4,
    /// [`PrecipOcclusion::params`].
    #[uniform(5)]
    pub occ_params: Vec4,
    /// [`PrecipOcclusion::image`] (top-down depth).
    #[texture(6, sample_type = "depth")]
    pub occ_map: Handle<bevy_image::Image>,
}

impl Material for PrecipMaterial {
    fn vertex_shader() -> ShaderRef {
        PRECIP_SHADER.into()
    }
    fn fragment_shader() -> ShaderRef {
        PRECIP_SHADER.into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
    fn enable_prepass() -> bool {
        false
    }
    fn enable_shadows() -> bool {
        false
    }
    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.buffers = vec![layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
            ATTRIBUTE_DROP.at_shader_location(2),
        ])?];
        Ok(())
    }
}

/// One drawn box: its kind slot and layer, the wrapped integrated offset.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct PrecipLayer {
    pub slot: usize,
    pub layer: usize,
    pub offset: Vec3,
}

/// Drops shown this frame (all boxes and kinds).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct PrecipStats {
    pub drops: u32,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrecipSystems;

pub struct PrecipPlugin;

impl Plugin for PrecipPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "shaders/precip.wesl");
        // Precipitation draws after every other translucent (water.md "Render order").
        if !app.is_plugin_added::<RenderOrderPlugin>() {
            app.add_plugins(RenderOrderPlugin);
        }
        occlusion::build(app);
        app.add_plugins(MaterialPlugin::<PrecipMaterial>::default())
            .init_resource::<Precip>()
            .init_resource::<PrecipStats>()
            .add_systems(bevy_app::Startup, spawn_layers.after(occlusion::setup))
            .add_systems(Update, update_layers.in_set(PrecipSystems));
    }
}

/// A small integer hash (PCG-ish), to [0, 1).
fn unit(mut x: u32) -> f32 {
    x = x.wrapping_mul(747_796_405).wrapping_add(2_891_336_453);
    x = ((x >> ((x >> 28) + 4)) ^ x).wrapping_mul(277_803_737);
    x = (x >> 22) ^ x;
    #[expect(clippy::cast_precision_loss, reason = "24 bits of hash")]
    let f = (x >> 8) as f32 / 16_777_216.0;
    f
}

/// The drops of one box: four vertices each (seed, corner, rank/size/phase).
pub fn drop_mesh(count: u32, salt: u32) -> Mesh {
    const CORNERS: [[f32; 2]; 4] = [[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]];
    let n = count as usize;
    let (mut seeds, mut corners, mut drops) = (Vec::with_capacity(n * 4), Vec::with_capacity(n * 4), Vec::with_capacity(n * 4));
    let mut indices = Vec::with_capacity(n * 6);
    for i in 0..count {
        let k = i.wrapping_mul(8).wrapping_add(salt.wrapping_mul(0x9E37_79B9));
        let seed = [unit(k), unit(k + 1), unit(k + 2)];
        // Rank spread evenly (i / count, shuffled by the hash) so the shown
        // fraction tracks the intensity closely.
        #[expect(clippy::cast_precision_loss, reason = "drop counts are small")]
        let rank = (i as f32 + unit(k + 3)) / count as f32;
        let drop = [rank, unit(k + 4), unit(k + 5), 0.0];
        for c in CORNERS {
            seeds.push(seed);
            corners.push(c);
            drops.push(drop);
        }
        let b = i * 4;
        indices.extend([b, b + 1, b + 2, b, b + 2, b + 3]);
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, seeds);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, corners);
    mesh.insert_attribute(ATTRIBUTE_DROP, drops);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// The velocity of `slot`'s drops in `wind` (m/s).
pub fn velocity(slot: &PrecipSlot, wind: Vec2) -> Vec3 {
    let w = wind * slot.wind_follow;
    Vec3::new(w.x, -slot.fall_speed, w.y)
}

/// `offset` moved by `v` over `dt`, wrapped to `[0, size)`.
pub fn advance(offset: Vec3, v: Vec3, dt: f32, size: f32) -> Vec3 {
    (offset + v * dt).rem_euclid(Vec3::splat(size))
}

/// The shader parameters of `layer` for `slot`.
pub fn layer_params(slot: &PrecipSlot, layer: usize, precip: &Precip, offset: Vec3, time: f32) -> PrecipParams {
    let (size, _) = LAYERS[layer];
    let inner = if layer == 0 { 0.0 } else { 0.5 * LAYERS[layer - 1].0 };
    let flake = slot.shape == PrecipShape::Flake;
    let v = velocity(slot, precip.wind);
    // Far boxes: thinner, more transparent (the fog makes the far curtain).
    let far = [1.0, 0.8, 0.55][layer];
    PrecipParams {
        volume: Vec4::new(size, inner, STREAK_S, if flake { 1.0 } else { 0.0 }),
        offset: offset.extend(time),
        velocity: v.extend(slot.flutter),
        look: Vec4::new(slot.size[0], slot.size[1], slot.intensity.clamp(0.0, 1.0), if flake { 1.5 } else { 1.0 }),
        color: (slot.color.truncate() * precip.brightness.max(0.0)).extend(slot.color.w * far),
    }
}

#[allow(clippy::needless_pass_by_value)]
fn spawn_layers(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<PrecipMaterial>>, fog: Res<TranslucentFog>, occ: Res<PrecipOcclusion>) {
    for (layer, (_, count)) in LAYERS.iter().enumerate() {
        #[expect(clippy::cast_possible_truncation, reason = "three layers")]
        let mesh = meshes.add(drop_mesh(*count, layer as u32 + 1));
        for slot in 0..SLOTS {
            let material = materials.add(PrecipMaterial { params: PrecipParams::default(), fog_params: fog.params, fog_volume: fog.volume.clone(), occ_clip: occ.clip_from_world, occ_params: occ.params, occ_map: occ.image.clone() });
            commands.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material),
                PrecipLayer { slot, layer, offset: Vec3::ZERO },
                Visibility::Hidden,
                NoFrustumCulling,
                NotShadowCaster,
                // Far box first, near box on top.
                DrawBand::Overlay { order: -i16::try_from(layer).unwrap_or(i16::MAX) },
                Name::new(format!("precip/{slot}/{layer}")),
            ));
        }
    }
}

/// Integrates each box's offset and writes its parameters; hides boxes of
/// empty slots.
#[allow(clippy::needless_pass_by_value)]
pub fn update_layers(
    time: Res<Time>,
    precip: Res<Precip>,
    mut stats: ResMut<PrecipStats>,
    mut layers: Query<(&mut PrecipLayer, &MeshMaterial3d<PrecipMaterial>, &mut Visibility)>,
    mut materials: ResMut<Assets<PrecipMaterial>>,
    fog: Res<TranslucentFog>,
    occ: Res<PrecipOcclusion>,
) {
    let dt = time.delta_secs().min(0.25);
    // Wrapped so the flutter phase keeps f32 precision.
    #[expect(clippy::cast_possible_truncation, reason = "wrapped to an hour")]
    let t = (time.elapsed_secs_f64() % 3600.0) as f32;
    let mut drops = 0.0;
    for (mut l, handle, mut vis) in &mut layers {
        let slot = precip.slots[l.slot].filter(|s| precip.enabled && s.intensity > 0.0);
        let want = if slot.is_some() { Visibility::Inherited } else { Visibility::Hidden };
        if *vis != want {
            *vis = want;
        }
        let Some(slot) = slot else { continue };
        let (size, count) = LAYERS[l.layer];
        l.offset = advance(l.offset, velocity(&slot, precip.wind), dt, size);
        #[expect(clippy::cast_precision_loss, reason = "drop counts are small")]
        let shown = count as f32 * slot.intensity.clamp(0.0, 1.0);
        drops += shown;
        let params = layer_params(&slot, l.layer, &precip, l.offset, t);
        let id: &Handle<PrecipMaterial> = &handle.0;
        if materials.get(id).is_some_and(|m| m.params != params || m.fog_params != fog.params || m.fog_volume != fog.volume || m.occ_clip != occ.clip_from_world || m.occ_params != occ.params || m.occ_map != occ.image)
            && let Some(mut m) = materials.get_mut(id)
        {
            m.params = params;
            m.fog_params = fog.params;
            m.fog_volume.clone_from(&fog.volume);
            m.occ_clip = occ.clip_from_world;
            m.occ_params = occ.params;
            m.occ_map.clone_from(&occ.image);
        }
    }
    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "a drop count")]
    let drops = drops as u32;
    stats.drops = drops;
}
