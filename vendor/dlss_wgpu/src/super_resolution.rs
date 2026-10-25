//! superfargoth patch: NGX calls replaced by plugin ABI calls (`abi.rs`); public API unchanged.

use crate::{
    DlssSdk,
    abi::{self, OptimalSettings, SuperResolutionCreate, SuperResolutionEvaluate},
    types::*,
};
use std::{
    iter,
    ops::RangeInclusive,
    ptr,
    sync::{Arc, Mutex},
};
use wgpu::{
    Adapter, CommandBuffer, CommandEncoder, CommandEncoderDescriptor, Device, Queue, Texture,
    TextureTransition, TextureUses, TextureView, hal::api::Vulkan,
};

/// Camera-specific object for using DLSS Super Resolution.
pub struct DlssSuperResolution {
    upscaled_resolution: [u32; 2],
    min_render_resolution: [u32; 2],
    max_render_resolution: [u32; 2],
    device: Device,
    sdk: Arc<Mutex<DlssSdk>>,
    feature: abi::Feature,
}

impl DlssSuperResolution {
    /// Create a new [`DlssSuperResolution`] object.
    ///
    /// This is an expensive operation. The resulting object should be cached, and only recreated when settings change.
    ///
    /// This should only be called if [`crate::FeatureSupport::super_resolution_supported`] is true.
    pub fn new(
        upscaled_resolution: [u32; 2],
        perf_quality_mode: DlssPerfQualityMode,
        feature_flags: DlssFeatureFlags,
        sdk: Arc<Mutex<DlssSdk>>,
        device: &Device,
        queue: &Queue,
    ) -> Result<Self, DlssError> {
        let locked_sdk = sdk.lock().unwrap();
        let api = locked_sdk.plugin.api;

        let quality = perf_quality_mode.as_quality(upscaled_resolution);

        let mut settings = OptimalSettings::default();
        // SAFETY: live SDK context, valid out pointer.
        check(unsafe {
            (api.optimal_settings)(
                locked_sdk.sdk,
                abi::FEATURE_SUPER_RESOLUTION,
                upscaled_resolution[0],
                upscaled_resolution[1],
                quality,
                &mut settings,
            )
        })?;
        let OptimalSettings {
            optimal: mut optimal_render_resolution,
            max: mut max_render_resolution,
            min: mut min_render_resolution,
        } = settings;
        if perf_quality_mode == DlssPerfQualityMode::Dlaa {
            optimal_render_resolution = upscaled_resolution;
            min_render_resolution = upscaled_resolution;
            max_render_resolution = upscaled_resolution;
        }

        let create_params = SuperResolutionCreate {
            render_size: optimal_render_resolution,
            target_size: upscaled_resolution,
            quality,
            flags: feature_flags.as_flags(),
            output_subrects: u32::from(feature_flags.contains(DlssFeatureFlags::OutputSubrect)),
        };

        let mut command_encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("dlss_super_resolution_context_creation"),
        });

        let mut feature = ptr::null_mut();
        // SAFETY: as upstream: the raw command buffer is recording inside `as_hal_mut`.
        unsafe {
            command_encoder.as_hal_mut::<Vulkan, _, _>(|command_encoder| {
                check((api.create_super_resolution)(
                    locked_sdk.sdk,
                    command_encoder.unwrap().raw_handle(),
                    &create_params,
                    &mut feature,
                ))
            })?
        }

        queue.submit([command_encoder.finish()]);

        Ok(Self {
            upscaled_resolution,
            min_render_resolution,
            max_render_resolution,
            device: device.clone(),
            sdk: Arc::clone(&sdk),
            feature,
        })
    }

    /// Encode rendering commands for DLSS Super Resolution.
    ///
    /// The resulting command buffer should be submitted to a [`Queue`] in the same submit as the finished `command_encoder`, ordered immediately afterwards.
    /// ```compile_fail
    /// let mut my_command_encoder = device.create_command_encoder(descriptor);
    /// let dlss_command_buffer = dlss.render(render_parameters, &mut my_command_encoder, adapter).unwrap();
    /// queue.submit([my_command_encoder.finish(), dlss_command_buffer]);
    /// ```
    ///
    /// Failing to follow these rules is undefined behavior.
    pub fn render(
        &mut self,
        render_parameters: DlssSuperResolutionRenderParameters,
        command_encoder: &mut CommandEncoder,
        adapter: &Adapter,
    ) -> Result<CommandBuffer, DlssError> {
        render_parameters.validate()?;

        let sdk = self.sdk.lock().unwrap();

        let partial_texture_size = render_parameters
            .partial_texture_size
            .unwrap_or(self.max_render_resolution);

        // The plugin reads these through raw pointers during the evaluate call. The bindings
        // must stay alive until the evaluate call below.
        let (exposure, exposure_scale, pre_exposure) = match &render_parameters.exposure {
            DlssSuperResolutionExposure::Manual {
                exposure,
                exposure_scale,
                pre_exposure,
            } => (
                Some(texture_to_resource(exposure, adapter)),
                exposure_scale.unwrap_or(1.0),
                pre_exposure.unwrap_or(0.0),
            ),
            DlssSuperResolutionExposure::Automatic => (None, 0.0, 0.0),
        };
        let color = texture_to_resource(render_parameters.color, adapter);
        let dlss_output = texture_to_resource(render_parameters.dlss_output, adapter);
        let depth = texture_to_resource(render_parameters.depth, adapter);
        let motion_vectors = texture_to_resource(render_parameters.motion_vectors, adapter);
        let bias = render_parameters
            .bias
            .map(|bias| texture_to_resource(bias, adapter));

        let eval_params = SuperResolutionEvaluate {
            color: &color,
            output: &dlss_output,
            depth: &depth,
            motion_vectors: &motion_vectors,
            exposure: opt_ptr(exposure.as_ref()),
            bias: opt_ptr(bias.as_ref()),
            jitter: render_parameters.jitter_offset,
            render_subrect: partial_texture_size,
            reset: u32::from(render_parameters.reset),
            motion_vector_scale: render_parameters.motion_vector_scale.unwrap_or([1.0, 1.0]),
            pre_exposure,
            exposure_scale,
        };

        command_encoder.transition_resources(iter::empty(), render_parameters.barrier_list());

        let mut dlss_command_encoder =
            self.device
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("dlss_super_resolution"),
                });
        // SAFETY: as upstream; all resources referenced by `eval_params` are alive.
        unsafe {
            dlss_command_encoder.as_hal_mut::<Vulkan, _, _>(|command_encoder| {
                check((sdk.plugin.api.evaluate_super_resolution)(
                    sdk.sdk,
                    command_encoder.unwrap().raw_handle(),
                    self.feature,
                    &eval_params,
                ))
            })?;
        }
        Ok(dlss_command_encoder.finish())
    }

    /// Suggested subpixel camera jitter for a given frame.
    pub fn suggested_jitter(&self, frame_number: u32, render_resolution: [u32; 2]) -> [f32; 2] {
        let ratio = self.upscaled_resolution[0] as f32 / render_resolution[0] as f32;
        let phase_count = (8.0 * ratio * ratio) as u32;
        let i = frame_number % phase_count;

        [halton_sequence(i, 2) - 0.5, halton_sequence(i, 3) - 0.5]
    }

    /// Suggested mip bias to apply when sampling textures.
    pub fn suggested_mip_bias(&self, render_resolution: [u32; 2]) -> f32 {
        (render_resolution[0] as f32 / self.upscaled_resolution[0] as f32).log2() - 1.0
    }

    /// The upscaled resolution DLSS will output at.
    pub fn upscaled_resolution(&self) -> [u32; 2] {
        self.upscaled_resolution
    }

    /// The resolution the camera should render at, pre-upscaling.
    pub fn render_resolution(&self) -> [u32; 2] {
        self.min_render_resolution
    }

    /// Like [`Self::render_resolution`], but returns a range of values for use with dynamic resolution scaling.
    pub fn render_resolution_range(&self) -> RangeInclusive<[u32; 2]> {
        self.min_render_resolution..=self.max_render_resolution
    }
}

impl Drop for DlssSuperResolution {
    fn drop(&mut self) {
        // SAFETY: as upstream: wait for idle, then release the feature.
        unsafe {
            let hal_device = self.device.as_hal::<Vulkan>().unwrap();
            hal_device
                .raw_device()
                .device_wait_idle()
                .expect("Failed to wait for idle device when destroying DlssSuperResolution");

            let api = self.sdk.lock().unwrap().plugin.api;
            check((api.release_feature)(self.feature))
                .expect("Failed to destroy DlssSuperResolution feature");
        }
    }
}

unsafe impl Send for DlssSuperResolution {}
unsafe impl Sync for DlssSuperResolution {}

/// Inputs and output resources needed for rendering [`DlssSuperResolution`].
pub struct DlssSuperResolutionRenderParameters<'a> {
    /// Main color view of your camera.
    pub color: &'a TextureView,
    /// Depth buffer.
    pub depth: &'a TextureView,
    /// Motion vectors.
    pub motion_vectors: &'a TextureView,
    /// Camera exposure settings.
    pub exposure: DlssSuperResolutionExposure<'a>,
    /// Optional per-pixel bias to make DLSS more reactive.
    pub bias: Option<&'a TextureView>,
    /// The texture DLSS outputs to.
    pub dlss_output: &'a TextureView,
    /// Whether DLSS should reset temporal history, useful for camera cuts.
    pub reset: bool,
    /// Subpixel jitter that was applied to your camera.
    pub jitter_offset: [f32; 2],
    /// Optionally use only a specific subrect of the input textures, rather than the whole textures.
    // TODO: Allow configuring partial texture origins
    pub partial_texture_size: Option<[u32; 2]>,
    /// Optional scaling factor to apply to the values contained within [`Self::motion_vectors`].
    pub motion_vector_scale: Option<[f32; 2]>,
}

/// Camera exposure as input for [`DlssSuperResolution`]..
pub enum DlssSuperResolutionExposure<'a> {
    /// Exposure controlled by the application.
    Manual {
        exposure: &'a TextureView,
        exposure_scale: Option<f32>,
        pre_exposure: Option<f32>,
    },
    /// Auto-exposure handled by DLSS.
    Automatic,
}

impl<'a> DlssSuperResolutionRenderParameters<'a> {
    fn validate(&self) -> Result<(), DlssError> {
        // TODO
        Ok(())
    }

    fn barrier_list(&self) -> impl Iterator<Item = TextureTransition<&'a Texture>> {
        fn resource_barrier(texture_view: &TextureView) -> TextureTransition<&Texture> {
            TextureTransition {
                texture: texture_view.texture(),
                selector: None,
                state: TextureUses::RESOURCE,
            }
        }

        [
            Some(resource_barrier(self.color)),
            Some(resource_barrier(self.depth)),
            Some(resource_barrier(self.motion_vectors)),
            match &self.exposure {
                DlssSuperResolutionExposure::Manual { exposure, .. } => {
                    Some(resource_barrier(exposure))
                }
                DlssSuperResolutionExposure::Automatic => None,
            },
            self.bias.map(resource_barrier),
            Some(TextureTransition {
                texture: self.dlss_output.texture(),
                selector: None,
                state: TextureUses::STORAGE_READ_WRITE,
            }),
        ]
        .into_iter()
        .flatten()
    }
}
