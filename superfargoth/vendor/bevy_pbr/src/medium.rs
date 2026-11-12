use bevy_color::ColorToComponents;
use bevy_light::atmosphere::{ScatteringMedium, ScatteringTerm};

use bevy_app::{App, Plugin};
use bevy_asset::AssetId;
use bevy_ecs::{
    component::Component,
    resource::Resource,
    system::{Commands, Res, SystemParamItem},
};
use bevy_math::{ops, Vec3, Vec4};
use bevy_render::{
    render_asset::{PrepareAssetError, RenderAsset, RenderAssetPlugin},
    render_resource::{
        Extent3d, FilterMode, Sampler, SamplerDescriptor, Texture, TextureDataOrder,
        TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
        TextureViewDescriptor,
    },
    renderer::{RenderDevice, RenderQueue},
    RenderApp, RenderStartup,
};
use smallvec::SmallVec;
use tracing::warn;

#[doc(hidden)]
pub struct ScatteringMediumPlugin;

impl Plugin for ScatteringMediumPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenderAssetPlugin::<GpuScatteringMedium>::default());

        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_systems(RenderStartup, init_scattering_medium_sampler);
        }
    }
}

/// superfargoth patch (own, P1: live medium scales): the most terms a medium carries on the
/// GPU. Each term's coefficients are uniform data (`GpuAtmosphere::medium_scattering` and
/// `medium_absorption`, WGSL `array<vec4<f32>, 8>`); terms past this are ignored with a warning.
pub const MAX_MEDIUM_TERMS: usize = 8;

/// superfargoth patch (own, P1: live medium scales): multipliers on one
/// [`ScatteringTerm`]'s coefficients, per RGB channel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MediumTermScale {
    /// Multiplies [`ScatteringTerm::scattering`].
    pub scattering: Vec3,
    /// Multiplies [`ScatteringTerm::absorption`].
    pub absorption: Vec3,
}

impl Default for MediumTermScale {
    fn default() -> Self {
        Self {
            scattering: Vec3::ONE,
            absorption: Vec3::ONE,
        }
    }
}

/// superfargoth patch (own, P1: live medium scales): live per-term multipliers on the
/// [`ScatteringMedium`] of the `Atmosphere` on the same entity, in term order (absent: all
/// one, the stock look). As UE `SkyAtmosphere`'s scattering and absorption scales, changing
/// them re-bakes nothing on the CPU: the medium LUTs hold each term's unit falloff and phase,
/// the coefficients times these scales are atmosphere uniform data, and the GPU LUT passes
/// pick the change up (transmittance and multiscattering once, `AtmosphereStaticLutCache`).
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct ScatteringMediumScales(pub [MediumTermScale; MAX_MEDIUM_TERMS]);

/// The GPU representation of a [`ScatteringMedium`].
pub struct GpuScatteringMedium {
    /// The terms of the scattering medium.
    pub terms: SmallVec<[ScatteringTerm; 1]>,
    /// The resolution at which to sample the falloff distribution of each
    /// scattering term.
    pub falloff_resolution: u32,
    /// The resolution at which to sample the phase function of each
    /// scattering term.
    pub phase_resolution: u32,
    /// superfargoth patch (own, P1: live medium scales): the `density_lut`, a 2D
    /// `falloff_resolution x ceil(terms / 4)` LUT of each term's unit falloff (channel `k` of
    /// row `j` is term `4j + k`) against the atmosphere's "falloff parameter", a linear value
    /// which is 1.0 at the planet's surface and 0.0 at the edge of space. `medium_lut.wesl` (P1b) multiplies
    /// in each term's absorption and scattering from the atmosphere uniform on the GPU (stock Bevy baked
    /// the coefficient sums here, so any coefficient change re-baked it on the CPU).
    pub density_lut: Texture,
    /// The default [`TextureView`] of the `density_lut`
    pub density_lut_view: TextureView,
    /// superfargoth patch (own, P1: live medium scales): the `scattering_lut`, a 2D
    /// `phase_resolution x terms` LUT of each term's phase function (row `t` is term `t`, zero
    /// for a term without one), the U axis a nonlinear mapping of `neg_LdotV`, the dot product
    /// of the light direction and the incoming view vector. Stock Bevy baked scattering x
    /// phase x falloff summed over the terms into a `falloff x phase` LUT.
    pub scattering_lut: Texture,
    /// The default [`TextureView`] of the `scattering_lut`
    pub scattering_lut_view: TextureView,
}

impl RenderAsset for GpuScatteringMedium {
    type SourceAsset = ScatteringMedium;

    type Param = (Res<'static, RenderDevice>, Res<'static, RenderQueue>);

    fn prepare_asset(
        source_asset: Self::SourceAsset,
        _asset_id: AssetId<Self::SourceAsset>,
        (render_device, render_queue): &mut SystemParamItem<Self::Param>,
        _previous_asset: Option<&Self>,
    ) -> Result<Self, PrepareAssetError<Self::SourceAsset>> {
        // superfargoth patch (own, P1: live medium scales): per-term unit LUTs.
        let data = MediumLutData::bake(&source_asset);
        let (falloff_resolution, phase_resolution) =
            (data.falloff_resolution, data.phase_resolution);
        let (density, scattering) = (data.density, data.phase);
        let (density_rows, phase_rows) = (data.density_rows, data.phase_rows);

        let density_lut = render_device.create_texture_with_data(
            render_queue,
            &TextureDescriptor {
                label: source_asset
                    .label
                    .as_deref()
                    .map(|label| format!("{}_density_lut", label))
                    .as_deref()
                    .or(Some("scattering_medium_density_lut")),
                size: Extent3d {
                    width: falloff_resolution,
                    height: density_rows,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba32Float,
                usage: TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            TextureDataOrder::LayerMajor,
            bytemuck::cast_slice(density.as_slice()),
        );

        let density_lut_view = density_lut.create_view(&TextureViewDescriptor {
            label: source_asset
                .label
                .as_deref()
                .map(|label| format!("{}_density_lut_view", label))
                .as_deref()
                .or(Some("scattering_medium_density_lut_view")),
            ..Default::default()
        });

        let scattering_lut = render_device.create_texture_with_data(
            render_queue,
            &TextureDescriptor {
                label: source_asset
                    .label
                    .as_deref()
                    .map(|label| format!("{}_scattering_lut", label))
                    .as_deref()
                    .or(Some("scattering_medium_scattering_lut")),
                size: Extent3d {
                    width: phase_resolution,
                    height: phase_rows,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba32Float,
                usage: TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            TextureDataOrder::LayerMajor,
            bytemuck::cast_slice(scattering.as_slice()),
        );

        let scattering_lut_view = scattering_lut.create_view(&TextureViewDescriptor {
            label: source_asset
                .label
                .as_deref()
                .map(|label| format!("{}_scattering_lut", label))
                .as_deref()
                .or(Some("scattering_medium_scattering_lut_view")),
            ..Default::default()
        });

        Ok(Self {
            terms: source_asset.terms,
            falloff_resolution,
            phase_resolution,
            density_lut,
            density_lut_view,
            scattering_lut,
            scattering_lut_view,
        })
    }
}

/// superfargoth patch (own, P1: live medium scales): the CPU data of a medium's two LUTs
/// (layouts on [`GpuScatteringMedium::density_lut`] and [`GpuScatteringMedium::scattering_lut`]),
/// `terms x (falloff + phase)` samples; stock Bevy sampled `falloff x phase x terms`.
pub struct MediumLutData {
    pub falloff_resolution: u32,
    pub phase_resolution: u32,
    /// Rows of the density LUT: four terms per texel.
    pub density_rows: u32,
    /// Rows of the phase LUT: one per term.
    pub phase_rows: u32,
    /// `falloff_resolution x density_rows` texels, row-major.
    pub density: Vec<Vec4>,
    /// `phase_resolution x phase_rows` texels, row-major.
    pub phase: Vec<Vec4>,
}

impl MediumLutData {
    pub fn bake(medium: &ScatteringMedium) -> Self {
        // Define the power curve parameter for the nonlinear phase mapping
        const PHASE_MAPPING_N: f32 = 0.5;
        let inv_n = 1.0 / PHASE_MAPPING_N;

        if medium.terms.len() > MAX_MEDIUM_TERMS {
            warn!(
                "ScatteringMedium {:?}: {} terms, the atmosphere uses the first {MAX_MEDIUM_TERMS}",
                medium.label,
                medium.terms.len()
            );
        }
        let terms = &medium.terms[..medium.terms.len().min(MAX_MEDIUM_TERMS)];
        let falloff_resolution = medium.falloff_resolution.max(1);
        let phase_resolution = medium.phase_resolution.max(1);
        let density_rows = terms.len().div_ceil(4).max(1) as u32;
        let phase_rows = terms.len().max(1) as u32;

        let mut density = Vec::with_capacity((falloff_resolution * density_rows) as usize);
        for row in 0..density_rows as usize {
            density.extend((0..falloff_resolution).map(|i| {
                let falloff = (i as f32 + 0.5) / falloff_resolution as f32;
                let mut texel = [0.0; 4];
                for (k, term) in terms.iter().skip(4 * row).take(4).enumerate() {
                    texel[k] = term.falloff.sample(falloff);
                }
                Vec4::from_array(texel)
            }));
        }

        let mut phase = Vec::with_capacity((phase_resolution * phase_rows) as usize);
        for t in 0..phase_rows as usize {
            phase.extend((0..phase_resolution).map(|j| {
                let u = (j as f32 + 0.5) / phase_resolution as f32;

                // Nonlinear phase mapping to mitigate banding in low-resolution LUTs.
                // Phase functions peak at forward/backward scattering. Linearly
                // sampling these regions causes banding. The power curve (n < 1)
                // allocates more texels near the peaks.
                let s = 2.0 * u - 1.0;
                let neg_l_dot_v = s.signum() * (1.0 - ops::powf(1.0 - s.abs(), inv_n));

                terms
                    .get(t)
                    .and_then(|term| term.phase.sample(neg_l_dot_v))
                    .map_or(Vec4::ZERO, |p| p.to_vec3().extend(0.0))
            }));
        }

        Self {
            falloff_resolution,
            phase_resolution,
            density_rows,
            phase_rows,
            density,
            phase,
        }
    }
}

/// The default sampler for all scattering media LUTs.
///
/// Just a bilinear clamp-to-edge sampler, nothing fancy.
#[derive(Resource)]
pub struct ScatteringMediumSampler(Sampler);

impl ScatteringMediumSampler {
    pub fn sampler(&self) -> &Sampler {
        &self.0
    }
}

fn init_scattering_medium_sampler(mut commands: Commands, render_device: Res<RenderDevice>) {
    let sampler = render_device.create_sampler(&SamplerDescriptor {
        label: Some("scattering_medium_sampler"),
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Linear,
        ..Default::default()
    });

    commands.insert_resource(ScatteringMediumSampler(sampler));
}
