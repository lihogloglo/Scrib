//! superfargoth patch: the C ABI between this facade and the `sf_dlss` plugin DLL.
//!
//! This file is the single definition of the ABI. The plugin (`superfargoth/plugins/sf_dlss`)
//! includes it with `#[path = "../../../vendor/dlss_wgpu/src/abi.rs"]`, so both sides always
//! agree on the layout. Bump [`ABI_VERSION`] on any change to a type or function below.
//!
//! Everything here is our own vocabulary (status codes, flags, plain-data structs) plus public
//! Vulkan handle types from `ash`; no NVIDIA headers, types or constants. The plugin maps these
//! to NGX values explicitly.
//!
//! Exported plugin symbols:
//! - `sf_dlss_abi_version() -> u32`, must return [`ABI_VERSION`];
//! - `sf_dlss_api_v1() -> *const ApiV1`, a static function table.

#![allow(dead_code)] // Each side uses its half.

use ash::vk;
use std::ffi::{c_char, c_void};

/// Version of every type and function in this file.
pub const ABI_VERSION: u32 = 1;

/// Symbol: `unsafe extern "C" fn() -> u32`.
pub const SYMBOL_ABI_VERSION: &[u8] = b"sf_dlss_abi_version\0";
/// Symbol: `unsafe extern "C" fn() -> *const ApiV1`.
pub const SYMBOL_API_V1: &[u8] = b"sf_dlss_api_v1\0";

/// Plugin file name inside the plugin folder.
#[cfg(target_os = "windows")]
pub const PLUGIN_FILE: &str = "sf_dlss.dll";
/// Plugin file name inside the plugin folder.
#[cfg(not(target_os = "windows"))]
pub const PLUGIN_FILE: &str = "libsf_dlss.so";

/// Result of every call. `code` is one of the `STATUS_*` values; for [`STATUS_UNKNOWN`],
/// `raw` holds the vendor's original result code (for logging only).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Status {
    pub code: u32,
    pub raw: u32,
}

impl Status {
    pub const OK: Self = Self { code: STATUS_SUCCESS, raw: 0 };
}

pub const STATUS_SUCCESS: u32 = 0;
pub const STATUS_FEATURE_NOT_SUPPORTED: u32 = 1;
pub const STATUS_PLATFORM_ERROR: u32 = 2;
pub const STATUS_FEATURE_ALREADY_EXISTS: u32 = 3;
pub const STATUS_FEATURE_NOT_FOUND: u32 = 4;
pub const STATUS_INVALID_PARAMETER: u32 = 5;
pub const STATUS_SCRATCH_BUFFER_TOO_SMALL: u32 = 6;
pub const STATUS_NOT_INITIALIZED: u32 = 7;
pub const STATUS_UNSUPPORTED_INPUT_FORMAT: u32 = 8;
pub const STATUS_RW_FLAG_MISSING: u32 = 9;
pub const STATUS_MISSING_INPUT: u32 = 10;
pub const STATUS_UNABLE_TO_INITIALIZE_FEATURE: u32 = 11;
pub const STATUS_OUT_OF_DATE: u32 = 12;
pub const STATUS_OUT_OF_GPU_MEMORY: u32 = 13;
pub const STATUS_UNSUPPORTED_FORMAT: u32 = 14;
pub const STATUS_UNABLE_TO_WRITE_TO_APP_DATA_PATH: u32 = 15;
pub const STATUS_UNSUPPORTED_PARAMETER: u32 = 16;
pub const STATUS_DENIED: u32 = 17;
pub const STATUS_NOT_IMPLEMENTED: u32 = 18;
pub const STATUS_UNKNOWN: u32 = u32::MAX;

/// Feature selector.
pub const FEATURE_SUPER_RESOLUTION: u32 = 0;
pub const FEATURE_RAY_RECONSTRUCTION: u32 = 1;

/// Performance/quality preset.
pub const QUALITY_PERFORMANCE: u32 = 0;
pub const QUALITY_BALANCED: u32 = 1;
pub const QUALITY_QUALITY: u32 = 2;
pub const QUALITY_ULTRA_PERFORMANCE: u32 = 3;
pub const QUALITY_DLAA: u32 = 5;

/// Feature creation flags (bit values of the public `DlssFeatureFlags`).
pub const CREATE_HDR: u32 = 1 << 0;
pub const CREATE_MV_LOW_RES: u32 = 1 << 1;
pub const CREATE_MV_JITTERED: u32 = 1 << 2;
pub const CREATE_DEPTH_INVERTED: u32 = 1 << 3;
pub const CREATE_AUTO_EXPOSURE: u32 = 1 << 6;
pub const CREATE_ALPHA_UPSCALING: u32 = 1 << 7;

/// [`AppInfo::flags`]: use the SDK's development runtime (on-screen debug overlay).
pub const APP_DEBUG_OVERLAY: u32 = 1 << 0;

/// Application identity, passed to every call that needs it.
#[repr(C)]
pub struct AppInfo {
    /// Project id (UUID string), NUL-terminated UTF-8.
    pub project_id: *const c_char,
    /// Engine version string, NUL-terminated UTF-8.
    pub engine_version: *const c_char,
    /// Folder the plugin was loaded from (where the vendor runtime DLLs sit), NUL-terminated
    /// UTF-8. The plugin also searches the executable's folder.
    pub plugin_dir: *const c_char,
    /// `APP_*` bits.
    pub flags: u32,
}

/// Required Vulkan extensions. The array is owned by the vendor runtime and stays valid for
/// the life of the process.
#[repr(C)]
pub struct Extensions {
    pub count: u32,
    pub properties: *const vk::ExtensionProperties,
}

/// Vulkan objects for SDK initialization.
#[repr(C)]
pub struct InitParams {
    pub instance: vk::Instance,
    pub physical_device: vk::PhysicalDevice,
    pub device: vk::Device,
    pub get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
    pub get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr,
}

/// Render resolutions for a target resolution and quality preset.
#[repr(C)]
#[derive(Default)]
pub struct OptimalSettings {
    pub optimal: [u32; 2],
    pub max: [u32; 2],
    pub min: [u32; 2],
}

/// A Vulkan image view as feature input or output.
#[repr(C)]
pub struct Resource {
    pub image_view: vk::ImageView,
    pub image: vk::Image,
    pub subresource_range: vk::ImageSubresourceRange,
    pub format: vk::Format,
    pub width: u32,
    pub height: u32,
    /// Non-zero if the image has storage usage (written by the feature).
    pub read_write: u32,
}

/// Super Resolution creation parameters.
#[repr(C)]
pub struct SuperResolutionCreate {
    pub render_size: [u32; 2],
    pub target_size: [u32; 2],
    pub quality: u32,
    pub flags: u32,
    pub output_subrects: u32,
}

/// Ray Reconstruction creation parameters.
#[repr(C)]
pub struct RayReconstructionCreate {
    pub render_size: [u32; 2],
    pub target_size: [u32; 2],
    pub quality: u32,
    pub flags: u32,
    pub output_subrects: u32,
    /// Non-zero: roughness packed in the normals' alpha.
    pub roughness_packed: u32,
    /// Non-zero: hardware depth; zero: linear depth.
    pub depth_hardware: u32,
}

/// Super Resolution evaluation inputs. Nullable pointers are optional inputs.
#[repr(C)]
pub struct SuperResolutionEvaluate {
    pub color: *const Resource,
    pub output: *const Resource,
    pub depth: *const Resource,
    pub motion_vectors: *const Resource,
    pub exposure: *const Resource,
    pub bias: *const Resource,
    pub jitter: [f32; 2],
    pub render_subrect: [u32; 2],
    pub reset: u32,
    pub motion_vector_scale: [f32; 2],
    pub pre_exposure: f32,
    pub exposure_scale: f32,
}

/// Ray Reconstruction evaluation inputs. Nullable pointers are optional inputs.
#[repr(C)]
pub struct RayReconstructionEvaluate {
    pub diffuse_albedo: *const Resource,
    pub specular_albedo: *const Resource,
    pub normals: *const Resource,
    pub roughness: *const Resource,
    pub color: *const Resource,
    pub alpha: *const Resource,
    pub output: *const Resource,
    pub output_alpha: *const Resource,
    pub depth: *const Resource,
    pub motion_vectors: *const Resource,
    pub responsivity_mask: *const Resource,
    pub color_before_transparency: *const Resource,
    pub screen_space_subsurface_scattering_guide: *const Resource,
    pub depth_of_field_guide: *const Resource,
    pub specular_hit_distance: *const Resource,
    pub specular_motion_vectors: *const Resource,
    pub transparency_layer: *const Resource,
    pub transparency_layer_opacity: *const Resource,
    /// Row-major 4x4, set together with `specular_hit_distance`.
    pub world_to_view: *const [f32; 16],
    pub view_to_clip: *const [f32; 16],
    pub jitter: [f32; 2],
    pub render_subrect: [u32; 2],
    pub reset: u32,
    pub motion_vector_scale: [f32; 2],
}

/// Opaque SDK context (one per device).
pub type Sdk = *mut c_void;
/// Opaque feature instance (one per camera).
pub type Feature = *mut c_void;

/// Function table returned by `sf_dlss_api_v1`.
#[repr(C)]
pub struct ApiV1 {
    /// Instance extensions `feature` needs. Callable before any Vulkan instance exists.
    pub instance_extensions:
        unsafe extern "C" fn(app: *const AppInfo, feature: u32, out: *mut Extensions) -> Status,
    /// Device extensions `feature` needs on `physical_device`.
    pub device_extensions: unsafe extern "C" fn(
        app: *const AppInfo,
        feature: u32,
        instance: vk::Instance,
        physical_device: vk::PhysicalDevice,
        out: *mut Extensions,
    ) -> Status,
    /// Initializes the SDK on a device and checks Super Resolution availability
    /// ([`STATUS_FEATURE_NOT_SUPPORTED`] if absent). Also starts the vendor's background
    /// runtime update check.
    pub init:
        unsafe extern "C" fn(app: *const AppInfo, params: *const InitParams, out: *mut Sdk) -> Status,
    /// Destroys `sdk` and shuts the SDK down on `device`. The caller waits for device idle.
    pub shutdown: unsafe extern "C" fn(sdk: Sdk, device: vk::Device) -> Status,
    /// VRAM allocated by the vendor runtime.
    pub vram_allocated: unsafe extern "C" fn(sdk: Sdk, out: *mut u64) -> Status,
    /// Render resolutions for `feature` at `width` x `height` and `quality`.
    pub optimal_settings: unsafe extern "C" fn(
        sdk: Sdk,
        feature: u32,
        width: u32,
        height: u32,
        quality: u32,
        out: *mut OptimalSettings,
    ) -> Status,
    /// Records Super Resolution creation into `command_buffer`.
    pub create_super_resolution: unsafe extern "C" fn(
        sdk: Sdk,
        command_buffer: vk::CommandBuffer,
        params: *const SuperResolutionCreate,
        out: *mut Feature,
    ) -> Status,
    /// Records Ray Reconstruction creation into `command_buffer`.
    pub create_ray_reconstruction: unsafe extern "C" fn(
        sdk: Sdk,
        device: vk::Device,
        command_buffer: vk::CommandBuffer,
        params: *const RayReconstructionCreate,
        out: *mut Feature,
    ) -> Status,
    /// Records a Super Resolution evaluation into `command_buffer`.
    pub evaluate_super_resolution: unsafe extern "C" fn(
        sdk: Sdk,
        command_buffer: vk::CommandBuffer,
        feature: Feature,
        params: *const SuperResolutionEvaluate,
    ) -> Status,
    /// Records a Ray Reconstruction evaluation into `command_buffer`.
    pub evaluate_ray_reconstruction: unsafe extern "C" fn(
        sdk: Sdk,
        command_buffer: vk::CommandBuffer,
        feature: Feature,
        params: *const RayReconstructionEvaluate,
    ) -> Status,
    /// Releases a feature. The caller waits for device idle.
    pub release_feature: unsafe extern "C" fn(feature: Feature) -> Status,
}
