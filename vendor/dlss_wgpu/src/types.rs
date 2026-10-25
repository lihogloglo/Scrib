//! superfargoth patch: public types (upstream module `nvsdk_ngx`, the bindgen bindings) reduced to
//! their mapping onto the plugin ABI (`abi.rs`). No NVIDIA headers or constants: the plugin maps the
//! ABI values to NGX ones.

use crate::abi::{self, Resource, Status};
use ash::vk::{
    ImageAspectFlags, ImageSubresourceRange, REMAINING_ARRAY_LAYERS, REMAINING_MIP_LEVELS,
};
use wgpu::{Adapter, TextureUsages, TextureView, wgc::api::Vulkan};

/// How much DLSS should upscale by.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum DlssPerfQualityMode {
    /// Let DLSS decide.
    #[default]
    Auto,
    /// Anti-aliasing only, no upscaling.
    Dlaa,
    Quality,
    Balanced,
    Performance,
    UltraPerformance,
}

impl DlssPerfQualityMode {
    /// superfargoth patch: ABI `QUALITY_*` value (upstream: `NVSDK_NGX_PerfQuality_Value`).
    pub(crate) fn as_quality(&self, upscaled_resolution: [u32; 2]) -> u32 {
        match self {
            Self::Auto => {
                let mega_pixels =
                    (upscaled_resolution[0] * upscaled_resolution[1]) as f32 / 1_000_000.0;

                if mega_pixels < 2.03 {
                    abi::QUALITY_DLAA
                } else if mega_pixels < 3.68 {
                    abi::QUALITY_QUALITY
                } else if mega_pixels < 8.29 {
                    abi::QUALITY_PERFORMANCE
                } else {
                    abi::QUALITY_ULTRA_PERFORMANCE
                }
            }
            Self::Dlaa => abi::QUALITY_DLAA,
            Self::Quality => abi::QUALITY_QUALITY,
            Self::Balanced => abi::QUALITY_BALANCED,
            Self::Performance => abi::QUALITY_PERFORMANCE,
            Self::UltraPerformance => abi::QUALITY_ULTRA_PERFORMANCE,
        }
    }
}

bitflags::bitflags! {
    /// Flags for creating a DLSS context.
    ///
    /// superfargoth patch: bits are the ABI `CREATE_*` values (same numbers as NGX's flags).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DlssFeatureFlags: u32 {
        /// Use an HDR texture for the color input instead of an SDR texture.
        const HighDynamicRange = abi::CREATE_HDR;
        /// Motion vector values are at the render resolution.
        ///
        /// Not setting this flag indicates that the motion vectors are at the upscaled resolution and have already been dilated,
        /// which can lead to higher quality.
        ///
        /// Most renderers will want to use this flag.
        const LowResolutionMotionVectors = abi::CREATE_MV_LOW_RES;
        /// Motion vector values contain jitter.
        const JitteredMotionVectors = abi::CREATE_MV_JITTERED;
        /// Camera is using a reverse depth buffer.
        const InvertedDepth = abi::CREATE_DEPTH_INVERTED;
        /// Have DLSS apply auto-exposure.
        ///
        /// Not supported by Ray Reconstruction.
        const AutoExposure = abi::CREATE_AUTO_EXPOSURE;
        /// Use a 4 channel RGBA texture for the color input instead of a 3 channel RGB texture.
        const AlphaUpscaling = abi::CREATE_ALPHA_UPSCALING;
        /// Allow DLSS to write to a subrect of the output.
        const OutputSubrect = 256; // Not an NGX flag
    }
}

impl DlssFeatureFlags {
    pub(crate) fn as_flags(&self) -> u32 {
        let mut flags = *self;
        flags.remove(DlssFeatureFlags::OutputSubrect);
        flags.bits()
    }
}

/// Errors thrown by DLSS.
#[derive(thiserror::Error, Debug)]
pub enum DlssError {
    #[error(
        "The NGX SDK or a specific feature is not supported by the current system, hardware, and/or graphics API."
    )]
    FeatureNotSupported,
    #[error(
        "An error occurred within the underlying platform, which includes the graphics API in use, the operating system, or other system libraries and dependencies that are not part of the NGX SDK, such as NvAPI. Consult the NGX logs and the graphics API's validation layers for detailed information."
    )]
    PlatformError,
    #[error(
        "The NGX feature could not be created because a feature with identical parameters already exists, and the feature does not support multiple identical instances."
    )]
    FeatureAlreadyExists,
    #[error("A feature associated with the provided handle could not be found.")]
    FeatureNotFound,
    #[error(
        "One or more provided parameters had an incorrect value or type, or a required parameter was not provided."
    )]
    InvalidParameters,
    #[error(
        "The feature requires a scratch buffer, but none was provided or the provided buffer is too small. Query the SDK's scratch buffer size to determine the necessary size."
    )]
    ScratchBufferTooSmall,
    #[error(
        "A function that requires the NGX SDK to be initialized was called before the SDK was properly initialized."
    )]
    NotInitialized,
    #[error("One or more input buffers supplied to the feature had an unsupported format.")]
    UnsupportedInputFormat,
    #[error(
        "The feature requires read/write access to output buffers, but one or more provided buffers did not have the correct access flags (UAV in D3D11/D3D12)."
    )]
    RWFlagMissing,
    #[error("A required input parameter was not provided.")]
    MissingInput,
    #[error(
        "The requested feature could not be initialized, likely because the library for that feature could not be found."
    )]
    UnableToInitializeFeature,
    #[error(
        "A function was used which requires a newer version of the NVIDIA Display Driver or feature library than is currently installed."
    )]
    OutOfDate,
    #[error("An operation could not be completed because the system lacked sufficient GPU memory.")]
    OutOfGPUMemory,
    #[error("One or more buffers provided to the feature had an unsupported format.")]
    UnsupportedFormat,
    #[error(
        "The SDK does not have the necessary write permissions for the path specified in InApplicationDataPath."
    )]
    UnableToWriteToAppDataPath,
    #[error(
        "A parameter supplied to the feature is either unsupported by the current version or has an unsupported value."
    )]
    UnsupportedParameter,
    #[error(
        "NVIDIA has restricted the use of this feature in the current application. Contact NVIDIA for further information."
    )]
    Denied,
    #[error(
        "The requested feature or functionality has not been implemented in the current version of the NGX SDK, display driver, or feature library."
    )]
    NotImplemented,
    #[error("The NGX SDK returned an unknown error code {0:#x}")]
    Unknown(u32),
}

/// superfargoth patch: maps a plugin [`Status`] (upstream: `check_ngx_result`).
pub fn check(status: Status) -> Result<(), DlssError> {
    match status.code {
        abi::STATUS_SUCCESS => Ok(()),
        abi::STATUS_FEATURE_NOT_SUPPORTED => Err(DlssError::FeatureNotSupported),
        abi::STATUS_PLATFORM_ERROR => Err(DlssError::PlatformError),
        abi::STATUS_FEATURE_ALREADY_EXISTS => Err(DlssError::FeatureAlreadyExists),
        abi::STATUS_FEATURE_NOT_FOUND => Err(DlssError::FeatureNotFound),
        abi::STATUS_INVALID_PARAMETER => Err(DlssError::InvalidParameters),
        abi::STATUS_SCRATCH_BUFFER_TOO_SMALL => Err(DlssError::ScratchBufferTooSmall),
        abi::STATUS_NOT_INITIALIZED => Err(DlssError::NotInitialized),
        abi::STATUS_UNSUPPORTED_INPUT_FORMAT => Err(DlssError::UnsupportedInputFormat),
        abi::STATUS_RW_FLAG_MISSING => Err(DlssError::RWFlagMissing),
        abi::STATUS_MISSING_INPUT => Err(DlssError::MissingInput),
        abi::STATUS_UNABLE_TO_INITIALIZE_FEATURE => Err(DlssError::UnableToInitializeFeature),
        abi::STATUS_OUT_OF_DATE => Err(DlssError::OutOfDate),
        abi::STATUS_OUT_OF_GPU_MEMORY => Err(DlssError::OutOfGPUMemory),
        abi::STATUS_UNSUPPORTED_FORMAT => Err(DlssError::UnsupportedFormat),
        abi::STATUS_UNABLE_TO_WRITE_TO_APP_DATA_PATH => {
            Err(DlssError::UnableToWriteToAppDataPath)
        }
        abi::STATUS_UNSUPPORTED_PARAMETER => Err(DlssError::UnsupportedParameter),
        abi::STATUS_DENIED => Err(DlssError::Denied),
        abi::STATUS_NOT_IMPLEMENTED => Err(DlssError::NotImplemented),
        _ => Err(DlssError::Unknown(status.raw)),
    }
}

/// superfargoth patch: the plugin, or [`DlssError::NotInitialized`] (only reachable if a caller
/// ignores [`crate::FeatureSupport`], which is all-false without a plugin).
pub(crate) fn plugin() -> Result<&'static crate::plugin::Plugin, DlssError> {
    crate::plugin::get().ok_or(DlssError::NotInitialized)
}

/// superfargoth patch: raw Vulkan handles of a view (upstream built an `NVSDK_NGX_Resource_VK`
/// here; the plugin does that now).
pub fn texture_to_resource(texture_view: &TextureView, adapter: &Adapter) -> Resource {
    // SAFETY: as upstream: raw handle extraction from wgpu's Vulkan backend (the only backend this
    // crate supports); the handles are only used while `texture_view` is borrowed.
    unsafe {
        let raw_view = texture_view.as_hal::<Vulkan>().unwrap().raw_handle();
        let texture = texture_view.texture();

        Resource {
            image_view: raw_view,
            image: texture.as_hal::<Vulkan>().unwrap().raw_handle(),
            subresource_range: ImageSubresourceRange {
                aspect_mask: if texture.format().has_color_aspect() {
                    ImageAspectFlags::COLOR
                } else {
                    ImageAspectFlags::DEPTH
                },
                base_mip_level: 0,
                level_count: REMAINING_MIP_LEVELS,
                base_array_layer: 0,
                layer_count: REMAINING_ARRAY_LAYERS,
            },
            format: adapter
                .as_hal::<Vulkan>()
                .unwrap()
                .texture_format_as_raw(texture.format()),
            width: texture.width(),
            height: texture.height(),
            read_write: u32::from(texture.usage().contains(TextureUsages::STORAGE_BINDING)),
        }
    }
}

/// Pointer to an optional resource (null if absent), for ABI structs.
pub(crate) fn opt_ptr(resource: Option<&Resource>) -> *const Resource {
    resource.map_or(std::ptr::null(), std::ptr::from_ref)
}

pub fn halton_sequence(mut index: u32, base: u32) -> f32 {
    let mut f = 1.0;
    let mut result = 0.0;
    while index > 0 {
        f /= base as f32;
        result += f * (index % base) as f32;
        index = (index as f32 / base as f32).floor() as u32;
    }
    result
}
