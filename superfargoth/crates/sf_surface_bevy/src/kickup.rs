//! Kick-up, slice W7 (surface-weather.md W7): puffs of snow and ash where a foot plants, from the
//! host's [`TrailStamp`] stream (the same foot-downs that print the trail ring).
//!
//! - **Main world** ([`update_kickup`]): each stamp within [`RANGE_M`] of the camera, over cover
//!   whose look is drawn (`SurfaceUniform::cover[c].x > 0`) and thick enough
//!   ([`sf_surface::kickup::kind_at`] on [`sf_surface::sample`] of the world surface), starts a
//!   burst in the fixed ring ([`BurstRing`]) with a strength from the cover, the pace
//!   ([`StepClock`]) and the foot's size. The live bursts (render position, age, facing,
//!   strength, kind, seed), the kind table and the ground wind go to the material's uniform.
//! - **Draw** ([`KickupMaterial`]): one entity, one draw of a fixed mesh of
//!   [`BURSTS`] × [`SPRITES`] quads ([`burst_mesh`]); `kickup.wesl` flies each sprite in closed
//!   form (the twin of `sf_surface::kickup::sprite_at`), lights it as the surfaces are lit (sun
//!   and moon with a shadow tap, the ambient floor, the sky's environment map, × exposure), fades
//!   it against the depth prepass and fogs it ([`TranslucentFog`]). Hidden while no burst lives.
//!
//! Presentation only (design rule "net-ready"): bursts never feed gameplay. Not built: puddle
//! splashes (the puddle level is a per-pixel function of the terrain's basins, no CPU twin at a
//! point yet), point lights on the puffs.

use bevy_app::{App, PostUpdate, Startup};
use bevy_asset::{Asset, Assets, Handle, RenderAssetUsages, embedded_asset};
use bevy_camera::visibility::{NoFrustumCulling, Visibility};
use bevy_ecs::prelude::*;
use bevy_light::NotShadowCaster;
use bevy_material::AlphaMode;
use bevy_math::{DVec2, Vec2, Vec3, Vec4};
use bevy_mesh::{Indices, Mesh, Mesh3d, MeshVertexAttribute, MeshVertexBufferLayoutRef, PrimitiveTopology, VertexFormat};
use bevy_pbr::{Material, MaterialPipeline, MaterialPipelineKey, MaterialPlugin, MeshMaterial3d};
use bevy_reflect::TypePath;
use bevy_render::RenderApp;
use bevy_render::render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderType, SpecializedMeshPipelineError};
use bevy_shader::ShaderRef;
use bevy_time::Time;
use bevy_transform::components::GlobalTransform;
use sf_render_order_bevy::{DrawBand, RenderOrderPlugin, TranslucentFog};
use sf_surface::kickup::{BURSTS, Burst, BurstRing, KickKind, KickLook, RANGE_M, SPRITES, StepClock, kind_at, strength};
use sf_weather_bevy::{CurrentWeather, WorldSurface, WorldWeatherMap};

use crate::{SurfaceOrigin, SurfaceSettings, SurfaceSystems, SurfaceUniform, SurfaceWeatherCamera, TrailStamp};

#[cfg(test)]
#[path = "kickup_tests.rs"]
mod tests;

pub const KICKUP_SHADER: &str = "embedded://sf_surface_bevy/kickup.wesl";

/// Per sprite vertex: (burst slot, sprite index, 0, 0).
pub const ATTRIBUTE_KICK: MeshVertexAttribute = MeshVertexAttribute::new("Kickup_Sprite", 1_310_872_407, VertexFormat::Float32x4);

/// Share of the weather's wind (10 m reference) the air at the feet moves with.
pub const GROUND_WIND: f32 = 0.5;

/// `--off kickup` (the game's A/B); on by default.
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct KickupSettings {
    pub on: bool,
}

impl Default for KickupSettings {
    fn default() -> Self {
        Self { on: true }
    }
}

/// Counters for status tools and tests.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct KickupStats {
    /// Bursts started.
    pub spawned: u64,
    /// Bursts alive after this frame's ageing.
    pub live: u32,
}

/// The burst pool and the pace of the feet (main world).
#[derive(Resource, Default)]
pub struct KickupState {
    pub ring: BurstRing,
    pub steps: StepClock,
}

/// `kickup.wesl`'s `KickKind`.
#[derive(Clone, Copy, Debug, Default, PartialEq, ShaderType)]
pub struct KickKindGpu {
    pub motion: Vec4,
    pub launch: Vec4,
    pub size: Vec4,
    pub color: Vec4,
}

impl From<&KickLook> for KickKindGpu {
    fn from(l: &KickLook) -> Self {
        Self {
            motion: Vec4::new(l.life_s, l.drag, l.gravity, l.wind_follow),
            launch: Vec4::new(l.speed, l.up, l.forward_bias, l.delay_s),
            size: Vec4::new(l.size[0], l.size[1], 0.0, 0.0),
            color: l.albedo.extend(l.opacity),
        }
    }
}

/// `kickup.wesl`'s `KickBurst`.
#[derive(Clone, Copy, Debug, PartialEq, ShaderType)]
pub struct KickBurstGpu {
    pub a: Vec4,
    pub b: Vec4,
    pub c: Vec4,
}

/// An empty slot (age < 0).
pub const EMPTY_BURST: KickBurstGpu = KickBurstGpu { a: Vec4::new(0.0, 0.0, 0.0, -1.0), b: Vec4::ZERO, c: Vec4::ZERO };

/// `kickup.wesl`'s `KickupParams`.
#[derive(Clone, Copy, Debug, PartialEq, ShaderType)]
pub struct KickupParams {
    pub kinds: [KickKindGpu; 2],
    pub wind: Vec4,
    pub bursts: [KickBurstGpu; BURSTS],
}

impl Default for KickupParams {
    fn default() -> Self {
        Self { kinds: KickKind::ALL.map(|k| KickKindGpu::from(&KickLook::of(k))), wind: Vec4::ZERO, bursts: [EMPTY_BURST; BURSTS] }
    }
}

/// The one kick-up draw.
#[derive(Asset, TypePath, AsBindGroup, Clone, Debug)]
pub struct KickupMaterial {
    #[uniform(0)]
    pub params: KickupParams,
    /// [`TranslucentFog::params`] (render-order.md rule 1; kept current by the plugin).
    #[uniform(1)]
    pub fog_params: Vec4,
    /// [`TranslucentFog::volume`].
    #[texture(2, dimension = "3d")]
    #[sampler(3)]
    pub fog_volume: Handle<bevy_image::Image>,
}

impl Material for KickupMaterial {
    fn vertex_shader() -> ShaderRef {
        KICKUP_SHADER.into()
    }
    fn fragment_shader() -> ShaderRef {
        KICKUP_SHADER.into()
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
            ATTRIBUTE_KICK.at_shader_location(2),
        ])?];
        Ok(())
    }
}

/// Marks the kick-up draw.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct KickupDraw;

/// A small integer hash (PCG-ish), to [0, 1).
pub fn unit(mut x: u32) -> f32 {
    x = x.wrapping_mul(747_796_405).wrapping_add(2_891_336_453);
    x = ((x >> ((x >> 28) + 4)) ^ x).wrapping_mul(277_803_737);
    x = (x >> 22) ^ x;
    #[expect(clippy::cast_precision_loss, reason = "24 bits of hash")]
    let f = (x >> 8) as f32 / 16_777_216.0;
    f
}

/// The sprites of every burst slot: four vertices each (seeds, corner, slot/index).
pub fn burst_mesh(bursts: u32, sprites: u32) -> Mesh {
    const CORNERS: [[f32; 2]; 4] = [[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]];
    let n = (bursts * sprites) as usize;
    let (mut seeds, mut corners, mut kicks) = (Vec::with_capacity(n * 4), Vec::with_capacity(n * 4), Vec::with_capacity(n * 4));
    let mut indices = Vec::with_capacity(n * 6);
    for b in 0..bursts {
        for s in 0..sprites {
            let i = b * sprites + s;
            // Sprite seeds: the same within a slot's sprite set as in every other slot (the burst's
            // seed moves them), spread so a burst's sprites cover the azimuth evenly.
            let k = s.wrapping_mul(8).wrapping_add(0x5EED);
            #[expect(clippy::cast_precision_loss, reason = "sprite counts are small")]
            let az = (s as f32 + 0.5 * unit(k)) / sprites as f32;
            let seed = [az, unit(k + 1), unit(k + 2)];
            #[expect(clippy::cast_precision_loss, reason = "slot and sprite indices are small")]
            let kick = [b as f32, s as f32, 0.0, 0.0];
            for c in CORNERS {
                seeds.push(seed);
                corners.push(c);
                kicks.push(kick);
            }
            let v = i * 4;
            indices.extend([v, v + 1, v + 2, v, v + 2, v + 3]);
        }
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, seeds);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, corners);
    mesh.insert_attribute(ATTRIBUTE_KICK, kicks);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// The live bursts as the shader reads them (render space around `origin`, ages at `now_s`).
pub fn pack_bursts(ring: &BurstRing, origin: bevy_math::DVec3, now_s: f64) -> [KickBurstGpu; BURSTS] {
    let mut out = [EMPTY_BURST; BURSTS];
    for (slot, b) in ring.live().filter(|(slot, _)| *slot < BURSTS) {
        let p = (b.at - origin).as_vec3();
        #[expect(clippy::cast_precision_loss, reason = "the kind index")]
        let kind = b.kind.index() as f32;
        out[slot] = KickBurstGpu { a: p.extend(b.age_s(now_s).max(0.0)), b: Vec4::new(b.forward.x, b.forward.y, b.strength, kind), c: Vec4::new(b.seed, 0.0, 0.0, 0.0) };
    }
    out
}

pub(crate) fn build(app: &mut App) {
    app.init_resource::<KickupSettings>()
        .init_resource::<KickupStats>()
        .init_resource::<KickupState>()
        .add_systems(PostUpdate, update_kickup.in_set(SurfaceSystems).after(crate::cover::update_cover));
    if app.get_sub_app(RenderApp).is_none() {
        return;
    }
    embedded_asset!(app, "kickup.wesl");
    // Puffs draw in the overlay band with the precipitation (water.md "Render order").
    if !app.is_plugin_added::<RenderOrderPlugin>() {
        app.add_plugins(RenderOrderPlugin);
    }
    app.add_plugins(MaterialPlugin::<KickupMaterial>::default()).add_systems(Startup, spawn_draw);
}

#[allow(clippy::needless_pass_by_value)]
fn spawn_draw(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<KickupMaterial>>, fog: Res<TranslucentFog>) {
    #[expect(clippy::cast_possible_truncation, reason = "64 bursts")]
    let mesh = meshes.add(burst_mesh(BURSTS as u32, SPRITES));
    let material = materials.add(KickupMaterial { params: KickupParams::default(), fog_params: fog.params, fog_volume: fog.volume.clone() });
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        KickupDraw,
        Visibility::Hidden,
        NoFrustumCulling,
        NotShadowCaster,
        // Before the precipitation boxes (orders -2..0): drops fall in front of the puffs.
        DrawBand::Overlay { order: -3 },
        Name::new("surface/kickup"),
    ));
}

/// Starts bursts from this frame's stamps, ages the ring and writes the draw (see the module
/// docs).
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
pub(crate) fn update_kickup(
    time: Option<Res<Time>>,
    kick: Res<KickupSettings>,
    settings: Res<SurfaceSettings>,
    origin: Res<SurfaceOrigin>,
    uniform: Res<SurfaceUniform>,
    cameras: Query<&GlobalTransform, With<SurfaceWeatherCamera>>,
    surface: Option<Res<WorldSurface>>,
    map: Option<Res<WorldWeatherMap>>,
    weather: Option<Res<CurrentWeather>>,
    mut stamps: MessageReader<TrailStamp>,
    mut state: ResMut<KickupState>,
    mut counters: ResMut<KickupStats>,
    mut draws: Query<(&MeshMaterial3d<KickupMaterial>, &mut Visibility), With<KickupDraw>>,
    materials: Option<ResMut<Assets<KickupMaterial>>>,
    fog: Option<Res<TranslucentFog>>,
) {
    let now = time.map_or(0.0, |t| t.elapsed_secs_f64());
    let camera = cameras.iter().next().map(GlobalTransform::translation);
    let on = kick.on && settings.layer && settings.cover;
    let looks = [KickLook::SNOW, KickLook::ASH];
    match camera.filter(|_| on) {
        None => {
            stamps.clear();
            state.ring.clear();
            state.steps.clear();
        }
        Some(camera) => {
            let looked = [uniform.cover[0].x > 0.0, uniform.cover[1].x > 0.0];
            let cover = surface.as_deref().zip(map.as_deref()).and_then(|(s, m)| s.grid().map(|g| (s, m, g)));
            for s in stamps.read() {
                let rel = origin.0.delta_f32(s.at);
                if rel.distance(camera) > RANGE_M {
                    continue;
                }
                let at = s.at.to_f64();
                let pace = state.steps.step(at, now);
                let Some((surface, map, grid)) = cover else { continue };
                let here = sf_surface::sample(&surface.state, map.map.raster(), grid.worldspace(), DVec2::new(at.x, at.z));
                let snow = if looked[0] { here.snow } else { 0.0 };
                let ash = if looked[1] { here.ash } else { 0.0 };
                let Some((kind, amount)) = kind_at(snow, ash) else { continue };
                let strength = strength(amount, pace, s.half_m.x);
                if strength <= 0.0 {
                    continue;
                }
                // W6b raises the cover above the physics ground the stamp sits on: throw the puff
                // from the uncut cover surface (the print's lip), or the depth test hides it in the
                // snow. Cell amounts only, full coverage (an upper bound): the shader's slope,
                // shelter and coverage terms aren't on the CPU.
                let raise = sf_surface::trail::cover_raise(snow, 1.0, uniform.cover[0].y, [0.0, 0.0], 0.0)
                    + sf_surface::trail::cover_raise(ash, 1.0, uniform.cover[1].y, [0.0, 0.0], 0.0);
                let at = at + bevy_math::DVec3::Y * f64::from(raise);
                #[expect(clippy::cast_possible_truncation, reason = "a hash seed")]
                let seed = unit(state.ring.spawned() as u32 ^ 0xB5E7);
                let forward = s.forward.try_normalize().unwrap_or(Vec2::ZERO);
                state.ring.spawn(Burst { at, forward, kind, strength, born_s: now, seed });
                counters.spawned += 1;
            }
        }
    }
    let live = state.ring.expire(now, &looks);
    #[expect(clippy::cast_possible_truncation, reason = "at most BURSTS")]
    let live_u32 = live as u32;
    counters.live = live_u32;
    let Some(mut materials) = materials else { return };
    for (handle, mut vis) in &mut draws {
        let want = if live > 0 { Visibility::Inherited } else { Visibility::Hidden };
        if *vis != want {
            *vis = want;
        }
        if live == 0 {
            continue;
        }
        let wind = weather.as_deref().filter(|w| w.valid).map_or(Vec2::ZERO, |w| w.frame.wind().velocity) * GROUND_WIND;
        let params = KickupParams {
            kinds: looks.map(|l| KickKindGpu::from(&l)),
            wind: Vec3::new(wind.x, 0.0, wind.y).extend(0.0),
            bursts: pack_bursts(&state.ring, origin.0.to_f64(), now),
        };
        if let Some(mut m) = materials.get_mut(&handle.0) {
            m.params = params;
            if let Some(fog) = fog.as_deref()
                && (m.fog_params != fog.params || m.fog_volume != fog.volume)
            {
                m.fog_params = fog.params;
                m.fog_volume.clone_from(&fog.volume);
            }
        }
    }
}
