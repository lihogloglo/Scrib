//! superfargoth patch: NGX calls replaced by plugin ABI calls (`abi.rs`); public API unchanged.

use crate::{
    DlssSdk,
    abi::{self, OptimalSettings, RayReconstructionCreate, RayReconstructionEvaluate},
    types::*,
};
use std::{
    iter, ptr,
    sync::{Arc, Mutex},
};
use wgpu::{
    Adapter, CommandBuffer, CommandEncoder, CommandEncoderDescriptor, Device, Queue, Texture,
    TextureTransition, TextureUses, TextureView, hal::api::Vulkan,
};

/// Camera-specific object for using DLSS Ray Reconstruction.
pub struct DlssRayReconstruction {
    upscaled_resolution: [u32; 2],
    render_resolution: [u32; 2],
    device: Device,
    sdk: Arc<Mutex<DlssSdk>>,
    feature: abi::Feature,
}

impl DlssRayReconstruction {
    /// Create a new [`DlssRayReconstruction`] object.
    ///
    /// This is an expensive operation. The resulting object should be cached, and only recreated when settings change.
    ///
    /// This should only be called if [`crate::FeatureSupport::ray_reconstruction_supported`] is true.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        upscaled_resolution: [u32; 2],
        perf_quality_mode: DlssPerfQualityMode,
        feature_flags: DlssFeatureFlags,
        roughness_mode: DlssRayReconstructionRoughnessMode,
        depth_mode: DlssRayReconstructionDepthMode,
        sdk: Arc<Mutex<DlssSdk>>,
        device: &Device,
        queue: &Queue,
    ) -> Result<Self, DlssError> {
        // Not supported by ray reconstruction
        if feature_flags.contains(DlssFeatureFlags::AutoExposure) {
            return Err(DlssError::UnsupportedParameter);
        }

        let locked_sdk = sdk.lock().unwrap();
        let api = locked_sdk.plugin.api;

        let quality = perf_quality_mode.as_quality(upscaled_resolution);

        let mut settings = OptimalSettings::default();
        // SAFETY: live SDK context, valid out pointer.
        check(unsafe {
            (api.optimal_settings)(
                locked_sdk.sdk,
                abi::FEATURE_RAY_RECONSTRUCTION,
                upscaled_resolution[0],
                upscaled_resolution[1],
                quality,
                &mut settings,
            )
        })?;
        let mut optimal_render_resolution = settings.optimal;
        if perf_quality_mode == DlssPerfQualityMode::Dlaa {
            optimal_render_resolution = upscaled_resolution;
        }

        let create_params = RayReconstructionCreate {
            render_size: optimal_render_resolution,
            target_size: upscaled_resolution,
            quality,
            flags: feature_flags.as_flags(),
            output_subrects: u32::from(feature_flags.contains(DlssFeatureFlags::OutputSubrect)),
            roughness_packed: u32::from(matches!(
                roughness_mode,
                DlssRayReconstructionRoughnessMode::Packed
            )),
            depth_hardware: u32::from(matches!(depth_mode, DlssRayReconstructionDepthMode::Hardware)),
        };

        let mut command_encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("dlss_ray_reconstruction_context_creation"),
        });

        let mut feature = ptr::null_mut();
        // SAFETY: as upstream: live device; the raw command buffer is recording inside `as_hal_mut`.
        unsafe {
            let hal_device = device.as_hal::<Vulkan>().unwrap();
            command_encoder.as_hal_mut::<Vulkan, _, _>(|command_encoder| {
                check((api.create_ray_reconstruction)(
                    locked_sdk.sdk,
                    hal_device.raw_device().handle(),
                    command_encoder.unwrap().raw_handle(),
                    &create_params,
                    &mut feature,
                ))
            })?
        }

        queue.submit([command_encoder.finish()]);

        Ok(Self {
            upscaled_resolution,
            render_resolution: optimal_render_resolution,
            device: device.clone(),
            sdk: Arc::clone(&sdk),
            feature,
        })
    }

    /// Encode rendering commands for DLSS Ray Reconstruction.
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
        render_parameters: DlssRayReconstructionRenderParameters,
        command_encoder: &mut CommandEncoder,
        adapter: &Adapter,
    ) -> Result<CommandBuffer, DlssError> {
        render_parameters.validate()?;

        let sdk = self.sdk.lock().unwrap();

        let partial_texture_size = render_parameters
            .partial_texture_size
            .unwrap_or(self.render_resolution);

        // The plugin reads these through raw pointers during the evaluate call. The bindings
        // must stay alive until the evaluate call below.
        let res = |view: &TextureView| texture_to_resource(view, adapter);
        let diffuse_albedo = res(render_parameters.diffuse_albedo);
        let specular_albedo = res(render_parameters.specular_albedo);
        let normals = res(render_parameters.normals);
        let roughness = render_parameters.roughness.map(res);
        let color = res(render_parameters.color);
        let dlss_output = res(render_parameters.dlss_output);
        let depth = res(render_parameters.depth);
        let motion_vectors = res(render_parameters.motion_vectors);
        let (transparency_layer, transparency_layer_opacity) =
            match render_parameters.transparency_overlay {
                Some(DlssRayReconstructionTransparencyOverlay::Premultiplied(layer)) => {
                    (Some(res(layer)), None)
                }
                Some(DlssRayReconstructionTransparencyOverlay::Separate { color, opacity }) => {
                    (Some(res(color)), Some(res(opacity)))
                }
                None => (None, None),
            };
        let color_before_transparency = render_parameters.color_before_transparency.map(res);
        let depth_of_field_guide = render_parameters.depth_of_field_guide.map(res);
        let screen_space_subsurface_scattering_guide = render_parameters
            .screen_space_subsurface_scattering_guide
            .map(res);
        let responsivity_mask = render_parameters.responsivity_mask.map(res);
        let alpha = render_parameters.alpha.map(res);
        let dlss_output_alpha = render_parameters.dlss_output_alpha.map(res);
        let mut specular_motion_vectors = None;
        let mut specular_hit_distance = None;
        let mut world_to_view = None;
        let mut view_to_clip = None;
        match render_parameters.specular_guide {
            DlssRayReconstructionSpecularGuide::SpecularMotionVectors(motion_vectors) => {
                specular_motion_vectors = Some(res(motion_vectors));
            }
            DlssRayReconstructionSpecularGuide::SpecularHitDistance {
                texture_view,
                world_to_view_rows_array,
                view_to_clip_rows_array,
            } => {
                specular_hit_distance = Some(res(texture_view));
                world_to_view = Some(world_to_view_rows_array);
                view_to_clip = Some(view_to_clip_rows_array);
            }
        }

        let eval_params = RayReconstructionEvaluate {
            diffuse_albedo: &diffuse_albedo,
            specular_albedo: &specular_albedo,
            normals: &normals,
            roughness: opt_ptr(roughness.as_ref()),
            color: &color,
            alpha: opt_ptr(alpha.as_ref()),
            output: &dlss_output,
            output_alpha: opt_ptr(dlss_output_alpha.as_ref()),
            depth: &depth,
            motion_vectors: &motion_vectors,
            responsivity_mask: opt_ptr(responsivity_mask.as_ref()),
            color_before_transparency: opt_ptr(color_before_transparency.as_ref()),
            screen_space_subsurface_scattering_guide: opt_ptr(
                screen_space_subsurface_scattering_guide.as_ref(),
            ),
            depth_of_field_guide: opt_ptr(depth_of_field_guide.as_ref()),
            specular_hit_distance: opt_ptr(specular_hit_distance.as_ref()),
            specular_motion_vectors: opt_ptr(specular_motion_vectors.as_ref()),
            transparency_layer: opt_ptr(transparency_layer.as_ref()),
            transparency_layer_opacity: opt_ptr(transparency_layer_opacity.as_ref()),
            world_to_view: world_to_view.as_ref().map_or(ptr::null(), ptr::from_ref),
            view_to_clip: view_to_clip.as_ref().map_or(ptr::null(), ptr::from_ref),
            jitter: render_parameters.jitter_offset,
            render_subrect: partial_texture_size,
            reset: u32::from(render_parameters.reset),
            motion_vector_scale: render_parameters.motion_vector_scale.unwrap_or([1.0, 1.0]),
        };

        command_encoder.transition_resources(iter::empty(), render_parameters.barrier_list());

        let mut dlss_command_encoder =
            self.device
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("dlss_ray_reconstruction"),
                });
        // SAFETY: as upstream; all resources referenced by `eval_params` are alive.
        unsafe {
            dlss_command_encoder.as_hal_mut::<Vulkan, _, _>(|command_encoder| {
                check((sdk.plugin.api.evaluate_ray_reconstruction)(
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
        let phase_count = ((8.0 * ratio * ratio) as u32).max(32);
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
        self.render_resolution
    }
}

impl Drop for DlssRayReconstruction {
    fn drop(&mut self) {
        // SAFETY: as upstream: wait for idle, then release the feature.
        unsafe {
            let hal_device = self.device.as_hal::<Vulkan>().unwrap();
            hal_device
                .raw_device()
                .device_wait_idle()
                .expect("Failed to wait for idle device when destroying DlssRayReconstruction");

            let api = self.sdk.lock().unwrap().plugin.api;
            check((api.release_feature)(self.feature))
                .expect("Failed to destroy DlssRayReconstruction feature");
        }
    }
}

unsafe impl Send for DlssRayReconstruction {}
unsafe impl Sync for DlssRayReconstruction {}

/// How roughness will be provided to [`DlssRayReconstruction`].
pub enum DlssRayReconstructionRoughnessMode {
    /// Roughness is provided as a standalone texture in [`DlssRayReconstructionRenderParameters::roughness`].
    Unpacked,
    /// Roughness is packed into the alpha channel of the normal texture in [`DlssRayReconstructionRenderParameters::normals`].
    Packed,
}

/// How depth will be provided to [`DlssRayReconstruction`].
pub enum DlssRayReconstructionDepthMode {
    /// Depth will be linear in view-space.
    Linear,
    /// Depth is a hardware depth buffer.
    Hardware,
}

/// Inputs and output resources needed for rendering [`DlssRayReconstruction`].
pub struct DlssRayReconstructionRenderParameters<'a> {
    /// Diffuse albedo.
    pub diffuse_albedo: &'a TextureView,
    /// Specular albedo.
    ///
    /// See section 3.4.2 of `$DLSS_SDK/doc/DLSS-RR Integration Guide.pdf` for how to calculate this texture.
    pub specular_albedo: &'a TextureView,
    /// Normals.
    ///
    /// Can be view-space or world-space.
    ///
    /// Must have linear material roughness in the alpha channel when using [`DlssRayReconstructionRoughnessMode::Packed`].
    pub normals: &'a TextureView,
    /// Linear material roughness.
    ///
    /// Must be provided when using [`DlssRayReconstructionRoughnessMode::Unpacked`].
    pub roughness: Option<&'a TextureView>,
    /// Main color view of your camera.
    pub color: &'a TextureView,
    /// Depth buffer.
    ///
    /// See [`DlssRayReconstructionDepthMode`] for format.
    pub depth: &'a TextureView,
    /// Motion vectors.
    pub motion_vectors: &'a TextureView,
    /// Specular material guide.
    pub specular_guide: DlssRayReconstructionSpecularGuide<'a>,
    /// Optional particles or other transparent effects, which are upscaled but not denoised.
    ///
    /// See section 3.4.10 of `$DLSS_SDK/doc/DLSS-RR Integration Guide.pdf`.
    pub transparency_overlay: Option<DlssRayReconstructionTransparencyOverlay<'a>>,
    /// Optional snapshot of [`Self::color`] before transparent effects are rendered on top of it.
    ///
    /// See section 3.4.11 of `$DLSS_SDK/doc/DLSS-RR Integration Guide.pdf`.
    pub color_before_transparency: Option<&'a TextureView>,
    /// Screen-space subsurface scattering guide.
    ///
    /// See section 3.4.12 of `$DLSS_SDK/doc/DLSS-RR Integration Guide.pdf` for how to calculate this texture
    pub screen_space_subsurface_scattering_guide: Option<&'a TextureView>,
    /// Optional depth of field guide.
    ///
    /// See section 3.4.13 of `$DLSS_SDK/doc/DLSS-RR Integration Guide.pdf` for how to calculate this texture.
    pub depth_of_field_guide: Option<&'a TextureView>,
    /// Optional per-pixel hint to make DLSS more or less responsive.
    ///
    /// See section 3.4.14 of `$DLSS_SDK/doc/DLSS-RR Integration Guide.pdf` for how to calculate this texture.
    pub responsivity_mask: Option<&'a TextureView>,
    /// Optional alpha texture to upscale, instead of the alpha channel of [`Self::color`].
    ///
    /// Requires [`DlssFeatureFlags::AlphaUpscaling`].
    pub alpha: Option<&'a TextureView>,
    /// Optional texture DLSS outputs alpha to, instead of the alpha channel of [`Self::dlss_output`].
    ///
    /// Requires [`DlssFeatureFlags::AlphaUpscaling`].
    pub dlss_output_alpha: Option<&'a TextureView>,
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

/// Guide buffer for specular material handling.
pub enum DlssRayReconstructionSpecularGuide<'a> {
    /// Motion vectors for objects reflected in specular material pixels.
    SpecularMotionVectors(&'a TextureView),
    /// World-space distance between primary vertex and hit point from tracing specular material pixels.
    SpecularHitDistance {
        /// Specular hit distance texture.
        texture_view: &'a TextureView,
        /// World-space to view-space camera matrix, as rows array.
        world_to_view_rows_array: [f32; 16],
        /// View-space to clip-space camera matrix, as rows array.
        view_to_clip_rows_array: [f32; 16],
    },
}

/// Transparent effects rendered separately from [`DlssRayReconstructionRenderParameters::color`].
pub enum DlssRayReconstructionTransparencyOverlay<'a> {
    /// RGB premultiplied by alpha, with alpha as the blending factor.
    Premultiplied(&'a TextureView),
    /// Color and per-channel opacity as two separate textures.
    Separate {
        /// Transparency color.
        color: &'a TextureView,
        /// Per-channel opacity.
        opacity: &'a TextureView,
    },
}

impl<'a> DlssRayReconstructionRenderParameters<'a> {
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

        fn storage_barrier(texture_view: &TextureView) -> TextureTransition<&Texture> {
            TextureTransition {
                texture: texture_view.texture(),
                selector: None,
                state: TextureUses::STORAGE_READ_WRITE,
            }
        }

        [
            Some(resource_barrier(self.diffuse_albedo)),
            Some(resource_barrier(self.specular_albedo)),
            Some(resource_barrier(self.normals)),
            self.roughness.map(resource_barrier),
            Some(resource_barrier(self.color)),
            Some(resource_barrier(self.depth)),
            Some(resource_barrier(self.motion_vectors)),
            match &self.specular_guide {
                DlssRayReconstructionSpecularGuide::SpecularMotionVectors(
                    specular_motion_vectors,
                ) => Some(resource_barrier(specular_motion_vectors)),
                DlssRayReconstructionSpecularGuide::SpecularHitDistance {
                    texture_view: specular_hit_distance,
                    ..
                } => Some(resource_barrier(specular_hit_distance)),
            },
            match &self.transparency_overlay {
                Some(DlssRayReconstructionTransparencyOverlay::Premultiplied(layer)) => {
                    Some(resource_barrier(layer))
                }
                Some(DlssRayReconstructionTransparencyOverlay::Separate { color, .. }) => {
                    Some(resource_barrier(color))
                }
                None => None,
            },
            match &self.transparency_overlay {
                Some(DlssRayReconstructionTransparencyOverlay::Separate { opacity, .. }) => {
                    Some(resource_barrier(opacity))
                }
                _ => None,
            },
            self.color_before_transparency.map(resource_barrier),
            self.screen_space_subsurface_scattering_guide
                .map(resource_barrier),
            self.depth_of_field_guide.map(resource_barrier),
            self.responsivity_mask.map(resource_barrier),
            self.alpha.map(resource_barrier),
            Some(storage_barrier(self.dlss_output)),
            self.dlss_output_alpha.map(storage_barrier),
        ]
        .into_iter()
        .flatten()
    }
}
