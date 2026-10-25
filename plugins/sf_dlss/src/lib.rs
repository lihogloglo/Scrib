//! `sf_dlss`: the NVIDIA NGX (DLSS) side of superfargoth's `dlss_wgpu` facade.
//!
//! The game links only the facade (`superfargoth/vendor/dlss_wgpu`), which holds no NVIDIA code;
//! this DLL links NVIDIA's `nvsdk_ngx` static library and is shipped separately, with NVIDIA's
//! `nvngx_dlss.dll` / `nvngx_dlssd.dll` and licence, in a `dlss/` folder next to the game exe.
//! The two talk through the versioned C ABI in `abi.rs` (one file, shared by both sides).
//!
//! The NGX calls are a port of crates.io `dlss_wgpu` 6.0.0 (MIT OR Apache-2.0,
//! <https://github.com/bevyengine/dlss_wgpu>), with the wgpu side left in the facade. No wgpu or
//! Bevy here, so Bevy upgrades don't require rebuilding the plugin.
//!
//! Every export is `extern "C"`; a panic would abort, so nothing here unwraps.

#![allow(unsafe_op_in_unsafe_fn)] // https://github.com/rust-lang/rust-bindgen/issues/3243
#![allow(clippy::missing_safety_doc)]
#![allow(non_upper_case_globals)] // NGX constant names in match patterns
// Pedantic (Cargo.toml `[lints]`) except where the NGX FFI makes it noise:
#![allow(
    clippy::borrow_as_ptr,
    clippy::unreadable_literal,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::wildcard_imports,
    clippy::too_many_lines,
    reason = "NGX FFI: eval structs take `&mut x` as raw pointers, NGX ids/GUIDs and C int casts are \
              verbatim from the SDK, bindgen output is glob-imported, eval fills one long C struct"
)]

#[path = "../../../vendor/dlss_wgpu/src/abi.rs"]
mod abi;

#[allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused, clippy::all)]
mod ngx {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    pub type PFN_vkGetDeviceProcAddr = ash::vk::PFN_vkGetDeviceProcAddr;
    pub type PFN_vkGetInstanceProcAddr = ash::vk::PFN_vkGetInstanceProcAddr;
    pub type VkBuffer = ash::vk::Buffer;
    pub type VkCommandBuffer = ash::vk::CommandBuffer;
    pub type VkDevice = ash::vk::Device;
    pub type VkExtensionProperties = ash::vk::ExtensionProperties;
    pub type VkFormat = ash::vk::Format;
    pub type VkImage = ash::vk::Image;
    pub type VkImageSubresourceRange = ash::vk::ImageSubresourceRange;
    pub type VkImageView = ash::vk::ImageView;
    pub type VkInstance = ash::vk::Instance;
    pub type VkPhysicalDevice = ash::vk::PhysicalDevice;
}

use abi::{AppInfo, Status};
use ash::vk;
use ngx::*;
use std::{
    ffi::{CStr, CString, OsStr, OsString},
    path::PathBuf,
    ptr, thread,
};

// ---------------------------------------------------------------------------------------------
// Exports

#[unsafe(no_mangle)]
pub extern "C" fn sf_dlss_abi_version() -> u32 {
    abi::ABI_VERSION
}

#[unsafe(no_mangle)]
pub extern "C" fn sf_dlss_api_v1() -> *const abi::ApiV1 {
    &API
}

static API: abi::ApiV1 = abi::ApiV1 {
    instance_extensions,
    device_extensions,
    init,
    shutdown,
    vram_allocated,
    optimal_settings,
    create_super_resolution,
    create_ray_reconstruction,
    evaluate_super_resolution,
    evaluate_ray_reconstruction,
    release_feature,
};

// ---------------------------------------------------------------------------------------------
// Helpers

fn status(result: NVSDK_NGX_Result) -> Status {
    let code = match result {
        NVSDK_NGX_Result_NVSDK_NGX_Result_Success => abi::STATUS_SUCCESS,
        NVSDK_NGX_Result_NVSDK_NGX_Result_FAIL_FeatureNotSupported => {
            abi::STATUS_FEATURE_NOT_SUPPORTED
        }
        NVSDK_NGX_Result_NVSDK_NGX_Result_FAIL_PlatformError => abi::STATUS_PLATFORM_ERROR,
        NVSDK_NGX_Result_NVSDK_NGX_Result_FAIL_FeatureAlreadyExists => {
            abi::STATUS_FEATURE_ALREADY_EXISTS
        }
        NVSDK_NGX_Result_NVSDK_NGX_Result_FAIL_FeatureNotFound => abi::STATUS_FEATURE_NOT_FOUND,
        NVSDK_NGX_Result_NVSDK_NGX_Result_FAIL_InvalidParameter => abi::STATUS_INVALID_PARAMETER,
        NVSDK_NGX_Result_NVSDK_NGX_Result_FAIL_ScratchBufferTooSmall => {
            abi::STATUS_SCRATCH_BUFFER_TOO_SMALL
        }
        NVSDK_NGX_Result_NVSDK_NGX_Result_FAIL_NotInitialized => abi::STATUS_NOT_INITIALIZED,
        NVSDK_NGX_Result_NVSDK_NGX_Result_FAIL_UnsupportedInputFormat => {
            abi::STATUS_UNSUPPORTED_INPUT_FORMAT
        }
        NVSDK_NGX_Result_NVSDK_NGX_Result_FAIL_RWFlagMissing => abi::STATUS_RW_FLAG_MISSING,
        NVSDK_NGX_Result_NVSDK_NGX_Result_FAIL_MissingInput => abi::STATUS_MISSING_INPUT,
        NVSDK_NGX_Result_NVSDK_NGX_Result_FAIL_UnableToInitializeFeature => {
            abi::STATUS_UNABLE_TO_INITIALIZE_FEATURE
        }
        NVSDK_NGX_Result_NVSDK_NGX_Result_FAIL_OutOfDate => abi::STATUS_OUT_OF_DATE,
        NVSDK_NGX_Result_NVSDK_NGX_Result_FAIL_OutOfGPUMemory => abi::STATUS_OUT_OF_GPU_MEMORY,
        NVSDK_NGX_Result_NVSDK_NGX_Result_FAIL_UnsupportedFormat => abi::STATUS_UNSUPPORTED_FORMAT,
        NVSDK_NGX_Result_NVSDK_NGX_Result_FAIL_UnableToWriteToAppDataPath => {
            abi::STATUS_UNABLE_TO_WRITE_TO_APP_DATA_PATH
        }
        NVSDK_NGX_Result_NVSDK_NGX_Result_FAIL_UnsupportedParameter => {
            abi::STATUS_UNSUPPORTED_PARAMETER
        }
        NVSDK_NGX_Result_NVSDK_NGX_Result_FAIL_Denied => abi::STATUS_DENIED,
        NVSDK_NGX_Result_NVSDK_NGX_Result_FAIL_NotImplemented => abi::STATUS_NOT_IMPLEMENTED,
        _ => abi::STATUS_UNKNOWN,
    };
    Status { code, raw: result as u32 }
}

const INVALID: Status = Status { code: abi::STATUS_INVALID_PARAMETER, raw: 0 };

/// Early-returns a non-success [`Status`].
macro_rules! ngx {
    ($call:expr) => {{
        let s = status($call);
        if s.code != abi::STATUS_SUCCESS {
            return s;
        }
    }};
}

fn feature_id(feature: u32) -> Option<NVSDK_NGX_Feature> {
    match feature {
        abi::FEATURE_SUPER_RESOLUTION => Some(NVSDK_NGX_Feature_NVSDK_NGX_Feature_SuperSampling),
        abi::FEATURE_RAY_RECONSTRUCTION => {
            Some(NVSDK_NGX_Feature_NVSDK_NGX_Feature_RayReconstruction)
        }
        _ => None,
    }
}

fn perf_quality(quality: u32) -> Option<NVSDK_NGX_PerfQuality_Value> {
    Some(match quality {
        abi::QUALITY_PERFORMANCE => NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_MaxPerf,
        abi::QUALITY_BALANCED => NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_Balanced,
        abi::QUALITY_QUALITY => NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_MaxQuality,
        abi::QUALITY_ULTRA_PERFORMANCE => {
            NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_UltraPerformance
        }
        abi::QUALITY_DLAA => NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_DLAA,
        _ => return None,
    })
}

fn create_flags(flags: u32) -> NVSDK_NGX_DLSS_Feature_Flags {
    let map = [
        (abi::CREATE_HDR, NVSDK_NGX_DLSS_Feature_Flags_NVSDK_NGX_DLSS_Feature_Flags_IsHDR),
        (abi::CREATE_MV_LOW_RES, NVSDK_NGX_DLSS_Feature_Flags_NVSDK_NGX_DLSS_Feature_Flags_MVLowRes),
        (abi::CREATE_MV_JITTERED, NVSDK_NGX_DLSS_Feature_Flags_NVSDK_NGX_DLSS_Feature_Flags_MVJittered),
        (
            abi::CREATE_DEPTH_INVERTED,
            NVSDK_NGX_DLSS_Feature_Flags_NVSDK_NGX_DLSS_Feature_Flags_DepthInverted,
        ),
        (
            abi::CREATE_AUTO_EXPOSURE,
            NVSDK_NGX_DLSS_Feature_Flags_NVSDK_NGX_DLSS_Feature_Flags_AutoExposure,
        ),
        (
            abi::CREATE_ALPHA_UPSCALING,
            NVSDK_NGX_DLSS_Feature_Flags_NVSDK_NGX_DLSS_Feature_Flags_AlphaUpscaling,
        ),
    ];
    map.iter()
        .filter(|(ours, _)| flags & ours != 0)
        .fold(0, |acc, (_, theirs)| acc | theirs)
}

/// Owned copy of an [`AppInfo`] (the caller's pointers only live for the call).
#[derive(Clone)]
struct App {
    project_id: CString,
    engine_version: CString,
    plugin_dir: Option<PathBuf>,
    debug_overlay: bool,
}

impl App {
    unsafe fn from_abi(app: *const AppInfo) -> Option<Self> {
        let app = app.as_ref()?;
        if app.project_id.is_null() || app.engine_version.is_null() {
            return None;
        }
        Some(Self {
            project_id: CStr::from_ptr(app.project_id).to_owned(),
            engine_version: CStr::from_ptr(app.engine_version).to_owned(),
            plugin_dir: (!app.plugin_dir.is_null())
                .then(|| PathBuf::from(CStr::from_ptr(app.plugin_dir).to_string_lossy().into_owned())),
            debug_overlay: app.flags & abi::APP_DEBUG_OVERLAY != 0,
        })
    }

    /// Folders NGX searches for `nvngx_dlss(d).dll`: the plugin's own folder, the exe's folder,
    /// and `$DLSS_SDK`'s runtime folder (dev builds).
    fn library_paths(&self) -> Vec<Vec<wchar_t>> {
        let mut paths = Vec::new();
        if let Some(dir) = &self.plugin_dir {
            paths.push(os_str_to_wchar(dir.as_os_str()));
        }
        if let Some(exe_dir) = std::env::current_exe().ok().and_then(|e| e.parent().map(PathBuf::from)) {
            paths.push(os_str_to_wchar(exe_dir.as_os_str()));
        }
        let platform = env!("DLSS_SDK_PLATFORM");
        let profile = if self.debug_overlay { "dev" } else { "rel" };
        if let Ok(sdk) = std::env::var("DLSS_SDK") {
            paths.push(os_str_to_wchar(&OsString::from(format!("{sdk}/lib/{platform}/{profile}"))));
        }
        paths
    }
}

/// Port of upstream `feature_info::with_feature_info`.
fn with_feature_info<F, T>(app: &App, feature_id: NVSDK_NGX_Feature, callback: F) -> T
where
    F: FnOnce(&NVSDK_NGX_FeatureDiscoveryInfo) -> T,
{
    let data_path = os_str_to_wchar(std::env::temp_dir().as_os_str());

    let shared_library_paths = app.library_paths();
    let shared_library_path_pointers = shared_library_paths
        .iter()
        .map(Vec::as_ptr)
        .collect::<Vec<_>>();

    let feature_info_common = NVSDK_NGX_FeatureCommonInfo {
        PathListInfo: NVSDK_NGX_PathListInfo {
            Path: shared_library_path_pointers.as_ptr(),
            Length: shared_library_paths.len() as u32,
        },
        InternalData: ptr::null_mut(),
        LoggingInfo: NVSDK_NGX_LoggingInfo {
            LoggingCallback: None,
            MinimumLoggingLevel: NVSDK_NGX_Logging_Level_NVSDK_NGX_LOGGING_LEVEL_OFF,
            DisableOtherLoggingSinks: false,
        },
    };

    let feature_info = NVSDK_NGX_FeatureDiscoveryInfo {
        SDKVersion: NVSDK_NGX_Version_NVSDK_NGX_Version_API,
        FeatureID: feature_id,
        Identifier: NVSDK_NGX_Application_Identifier {
            IdentifierType: NVSDK_NGX_Application_Identifier_Type_NVSDK_NGX_Application_Identifier_Type_Project_Id,
            v: NVSDK_NGX_Application_Identifier_v {
                ProjectDesc: NVSDK_NGX_ProjectIdDescription {
                    ProjectId: app.project_id.as_ptr(),
                    EngineType: NVSDK_NGX_EngineType_NVSDK_NGX_ENGINE_TYPE_CUSTOM,
                    EngineVersion: app.engine_version.as_ptr(),
                },
            },
        },
        ApplicationDataPath: data_path.as_ptr(),
        FeatureInfo: &feature_info_common,
    };

    callback(&feature_info)
}

#[cfg(target_os = "windows")]
fn os_str_to_wchar(s: &OsStr) -> Vec<wchar_t> {
    use std::os::windows::ffi::OsStrExt;

    s.encode_wide().chain([0]).map(|c| c as wchar_t).collect()
}

#[cfg(not(target_os = "windows"))]
fn os_str_to_wchar(s: &OsStr) -> Vec<wchar_t> {
    s.to_str()
        .unwrap_or("")
        .chars()
        .chain([0 as char])
        .map(|c| c as wchar_t)
        .collect()
}

/// `NVSDK_NGX_Resource_VK` for an ABI resource.
unsafe fn resource(r: &abi::Resource) -> NVSDK_NGX_Resource_VK {
    NVSDK_NGX_Create_ImageView_Resource_VK(
        r.image_view,
        r.image,
        r.subresource_range,
        r.format,
        r.width,
        r.height,
        r.read_write != 0,
    )
}

unsafe fn opt_resource(r: *const abi::Resource) -> Option<NVSDK_NGX_Resource_VK> {
    r.as_ref().map(|r| resource(r))
}

fn opt_mut<T>(value: &mut Option<T>) -> *mut T {
    value.as_mut().map_or(ptr::null_mut(), ptr::from_mut)
}

const ORIGIN: NVSDK_NGX_Coordinates = NVSDK_NGX_Coordinates { X: 0, Y: 0 };

// ---------------------------------------------------------------------------------------------
// ABI functions

unsafe extern "C" fn instance_extensions(
    app: *const AppInfo,
    feature: u32,
    out: *mut abi::Extensions,
) -> Status {
    let (Some(app), Some(feature), Some(out)) = (App::from_abi(app), feature_id(feature), out.as_mut())
    else {
        return INVALID;
    };
    with_feature_info(&app, feature, |feature_info| {
        let mut properties = ptr::null_mut();
        let mut count = 0;
        ngx!(NVSDK_NGX_VULKAN_GetFeatureInstanceExtensionRequirements(
            feature_info,
            &mut count,
            &mut properties,
        ));
        *out = abi::Extensions { count, properties };
        Status::OK
    })
}

unsafe extern "C" fn device_extensions(
    app: *const AppInfo,
    feature: u32,
    instance: vk::Instance,
    physical_device: vk::PhysicalDevice,
    out: *mut abi::Extensions,
) -> Status {
    let (Some(app), Some(feature), Some(out)) = (App::from_abi(app), feature_id(feature), out.as_mut())
    else {
        return INVALID;
    };
    with_feature_info(&app, feature, |feature_info| {
        let mut properties = ptr::null_mut();
        let mut count = 0;
        ngx!(NVSDK_NGX_VULKAN_GetFeatureDeviceExtensionRequirements(
            instance,
            physical_device,
            feature_info,
            &mut count,
            &mut properties,
        ));
        *out = abi::Extensions { count, properties };
        Status::OK
    })
}

/// Port of upstream `sdk::check_for_updates`.
fn check_for_updates(app: App) {
    thread::spawn(move || {
        for feature in [
            NVSDK_NGX_Feature_NVSDK_NGX_Feature_SuperSampling,
            NVSDK_NGX_Feature_NVSDK_NGX_Feature_RayReconstruction,
        ] {
            with_feature_info(&app, feature, |feature_info| unsafe {
                NVSDK_NGX_UpdateFeature(&feature_info.Identifier, feature_info.FeatureID);
            });
        }
    });
}

unsafe extern "C" fn init(
    app: *const AppInfo,
    params: *const abi::InitParams,
    out: *mut abi::Sdk,
) -> Status {
    let (Some(app), Some(p), Some(out)) = (App::from_abi(app), params.as_ref(), out.as_mut()) else {
        return INVALID;
    };
    check_for_updates(app.clone());

    with_feature_info(&app, Default::default(), |feature_info| {
        ngx!(NVSDK_NGX_VULKAN_Init_with_ProjectID(
            feature_info.Identifier.v.ProjectDesc.ProjectId,
            NVSDK_NGX_EngineType_NVSDK_NGX_ENGINE_TYPE_CUSTOM,
            feature_info.Identifier.v.ProjectDesc.EngineVersion,
            feature_info.ApplicationDataPath,
            p.instance,
            p.physical_device,
            p.device,
            p.get_instance_proc_addr,
            p.get_device_proc_addr,
            feature_info.FeatureInfo,
            NVSDK_NGX_Version_NVSDK_NGX_Version_API,
        ));
        Status::OK
    });

    let mut parameters = ptr::null_mut();
    ngx!(NVSDK_NGX_VULKAN_GetCapabilityParameters(&mut parameters));

    let mut dlss_supported = 0;
    let result = status(NVSDK_NGX_Parameter_GetI(
        parameters,
        NVSDK_NGX_Parameter_SuperSampling_Available.as_ptr().cast(),
        &mut dlss_supported,
    ));
    if result.code != abi::STATUS_SUCCESS {
        NVSDK_NGX_VULKAN_DestroyParameters(parameters);
        return result;
    }
    if dlss_supported == 0 {
        NVSDK_NGX_VULKAN_DestroyParameters(parameters);
        return Status { code: abi::STATUS_FEATURE_NOT_SUPPORTED, raw: 0 };
    }
    *out = parameters.cast();
    Status::OK
}

unsafe extern "C" fn shutdown(sdk: abi::Sdk, device: vk::Device) -> Status {
    ngx!(NVSDK_NGX_VULKAN_DestroyParameters(sdk.cast()));
    status(NVSDK_NGX_VULKAN_Shutdown1(device))
}

unsafe extern "C" fn vram_allocated(sdk: abi::Sdk, out: *mut u64) -> Status {
    let Some(out) = out.as_mut() else { return INVALID };
    let mut bytes = 0;
    ngx!(NGX_DLSS_GET_STATS(sdk.cast(), &mut bytes));
    *out = bytes;
    Status::OK
}

unsafe extern "C" fn optimal_settings(
    sdk: abi::Sdk,
    feature: u32,
    width: u32,
    height: u32,
    quality: u32,
    out: *mut abi::OptimalSettings,
) -> Status {
    let (Some(quality), Some(out)) = (perf_quality(quality), out.as_mut()) else {
        return INVALID;
    };
    let get = match feature {
        abi::FEATURE_SUPER_RESOLUTION => NGX_DLSS_GET_OPTIMAL_SETTINGS,
        abi::FEATURE_RAY_RECONSTRUCTION => NGX_DLSSD_GET_OPTIMAL_SETTINGS,
        _ => return INVALID,
    };
    let mut deprecated_sharpness = 0.0f32;
    status(get(
        sdk.cast(),
        width,
        height,
        quality,
        &mut out.optimal[0],
        &mut out.optimal[1],
        &mut out.max[0],
        &mut out.max[1],
        &mut out.min[0],
        &mut out.min[1],
        &mut deprecated_sharpness,
    ))
}

unsafe extern "C" fn create_super_resolution(
    sdk: abi::Sdk,
    command_buffer: vk::CommandBuffer,
    params: *const abi::SuperResolutionCreate,
    out: *mut abi::Feature,
) -> Status {
    let (Some(p), Some(out)) = (params.as_ref(), out.as_mut()) else { return INVALID };
    let Some(quality) = perf_quality(p.quality) else { return INVALID };
    let mut create_params = NVSDK_NGX_DLSS_Create_Params {
        Feature: NVSDK_NGX_Feature_Create_Params {
            InWidth: p.render_size[0],
            InHeight: p.render_size[1],
            InTargetWidth: p.target_size[0],
            InTargetHeight: p.target_size[1],
            InPerfQualityValue: quality,
        },
        InFeatureCreateFlags: create_flags(p.flags),
        InEnableOutputSubrects: p.output_subrects != 0,
    };
    let mut feature = ptr::null_mut();
    ngx!(NGX_VULKAN_CREATE_DLSS_EXT(
        command_buffer,
        1,
        1,
        &mut feature,
        sdk.cast(),
        &mut create_params,
    ));
    *out = feature.cast();
    Status::OK
}

unsafe extern "C" fn create_ray_reconstruction(
    sdk: abi::Sdk,
    device: vk::Device,
    command_buffer: vk::CommandBuffer,
    params: *const abi::RayReconstructionCreate,
    out: *mut abi::Feature,
) -> Status {
    let (Some(p), Some(out)) = (params.as_ref(), out.as_mut()) else { return INVALID };
    let Some(quality) = perf_quality(p.quality) else { return INVALID };
    let mut create_params = NVSDK_NGX_DLSSD_Create_Params {
        InDenoiseMode: NVSDK_NGX_DLSS_Denoise_Mode_NVSDK_NGX_DLSS_Denoise_Mode_DLUnified,
        InRoughnessMode: if p.roughness_packed != 0 {
            NVSDK_NGX_DLSS_Roughness_Mode_NVSDK_NGX_DLSS_Roughness_Mode_Packed
        } else {
            NVSDK_NGX_DLSS_Roughness_Mode_NVSDK_NGX_DLSS_Roughness_Mode_Unpacked
        },
        InUseHWDepth: if p.depth_hardware != 0 {
            NVSDK_NGX_DLSS_Depth_Type_NVSDK_NGX_DLSS_Depth_Type_HW
        } else {
            NVSDK_NGX_DLSS_Depth_Type_NVSDK_NGX_DLSS_Depth_Type_Linear
        },
        InWidth: p.render_size[0],
        InHeight: p.render_size[1],
        InTargetWidth: p.target_size[0],
        InTargetHeight: p.target_size[1],
        InPerfQualityValue: quality,
        InFeatureCreateFlags: create_flags(p.flags),
        InEnableOutputSubrects: p.output_subrects != 0,
    };
    let mut feature = ptr::null_mut();
    ngx!(NGX_VULKAN_CREATE_DLSSD_EXT1(
        device,
        command_buffer,
        1,
        1,
        &mut feature,
        sdk.cast(),
        &mut create_params,
    ));
    *out = feature.cast();
    Status::OK
}

unsafe extern "C" fn evaluate_super_resolution(
    sdk: abi::Sdk,
    command_buffer: vk::CommandBuffer,
    feature: abi::Feature,
    params: *const abi::SuperResolutionEvaluate,
) -> Status {
    let Some(p) = params.as_ref() else { return INVALID };
    let (Some(color), Some(output), Some(depth), Some(motion_vectors)) = (
        opt_resource(p.color),
        opt_resource(p.output),
        opt_resource(p.depth),
        opt_resource(p.motion_vectors),
    ) else {
        return INVALID;
    };
    // NGX reads these through raw pointers during EvaluateFeature; they live until the end.
    let (mut color, mut output, mut depth, mut motion_vectors) = (color, output, depth, motion_vectors);
    let mut exposure = opt_resource(p.exposure);
    let mut bias = opt_resource(p.bias);

    let mut eval_params = NVSDK_NGX_VK_DLSS_Eval_Params {
        Feature: NVSDK_NGX_VK_Feature_Eval_Params {
            pInColor: &mut color,
            pInOutput: &mut output,
            InSharpness: 0.0,
        },
        pInDepth: &mut depth,
        pInMotionVectors: &mut motion_vectors,
        InJitterOffsetX: p.jitter[0],
        InJitterOffsetY: p.jitter[1],
        InRenderSubrectDimensions: NVSDK_NGX_Dimensions {
            Width: p.render_subrect[0],
            Height: p.render_subrect[1],
        },
        InReset: p.reset as _,
        InMVScaleX: p.motion_vector_scale[0],
        InMVScaleY: p.motion_vector_scale[1],
        pInTransparencyMask: ptr::null_mut(),
        pInExposureTexture: opt_mut(&mut exposure),
        pInBiasCurrentColorMask: opt_mut(&mut bias),
        InColorSubrectBase: ORIGIN,
        InDepthSubrectBase: ORIGIN,
        InMVSubrectBase: ORIGIN,
        InTranslucencySubrectBase: ORIGIN,
        InBiasCurrentColorSubrectBase: ORIGIN,
        InOutputSubrectBase: ORIGIN,
        InPreExposure: p.pre_exposure,
        InExposureScale: p.exposure_scale,
        InIndicatorInvertXAxis: 0,
        InIndicatorInvertYAxis: 0,
        GBufferSurface: NVSDK_NGX_VK_GBuffer {
            pInAttrib: [ptr::null_mut(); 17],
        },
        InToneMapperType: NVSDK_NGX_ToneMapperType_NVSDK_NGX_TONEMAPPER_STRING,
        pInMotionVectors3D: ptr::null_mut(),
        pInIsParticleMask: ptr::null_mut(),
        pInAnimatedTextureMask: ptr::null_mut(),
        pInDepthHighRes: ptr::null_mut(),
        pInPositionViewSpace: ptr::null_mut(),
        InFrameTimeDeltaInMsec: 0.0,
        pInRayTracingHitDistance: ptr::null_mut(),
        pInMotionVectorsReflections: ptr::null_mut(),
    };

    status(NGX_VULKAN_EVALUATE_DLSS_EXT(
        command_buffer,
        feature.cast(),
        sdk.cast(),
        &mut eval_params,
    ))
}

unsafe extern "C" fn evaluate_ray_reconstruction(
    sdk: abi::Sdk,
    command_buffer: vk::CommandBuffer,
    feature: abi::Feature,
    params: *const abi::RayReconstructionEvaluate,
) -> Status {
    let Some(p) = params.as_ref() else { return INVALID };
    let required = (
        opt_resource(p.diffuse_albedo),
        opt_resource(p.specular_albedo),
        opt_resource(p.normals),
        opt_resource(p.color),
        opt_resource(p.output),
        opt_resource(p.depth),
        opt_resource(p.motion_vectors),
    );
    let (
        Some(mut diffuse_albedo),
        Some(mut specular_albedo),
        Some(mut normals),
        Some(mut color),
        Some(mut output),
        Some(mut depth),
        Some(mut motion_vectors),
    ) = required
    else {
        return INVALID;
    };
    // NGX reads these through raw pointers during EvaluateFeature; they live until the end.
    let mut roughness = opt_resource(p.roughness);
    let mut alpha = opt_resource(p.alpha);
    let mut output_alpha = opt_resource(p.output_alpha);
    let mut responsivity_mask = opt_resource(p.responsivity_mask);
    let mut color_before_transparency = opt_resource(p.color_before_transparency);
    let mut sss_guide = opt_resource(p.screen_space_subsurface_scattering_guide);
    let mut dof_guide = opt_resource(p.depth_of_field_guide);
    let mut specular_hit_distance = opt_resource(p.specular_hit_distance);
    let mut specular_motion_vectors = opt_resource(p.specular_motion_vectors);
    let mut transparency_layer = opt_resource(p.transparency_layer);
    let mut transparency_layer_opacity = opt_resource(p.transparency_layer_opacity);

    let mut eval_params = NVSDK_NGX_VK_DLSSD_Eval_Params {
        pInResponsivityMask: opt_mut(&mut responsivity_mask),
        pInDiffuseAlbedo: &mut diffuse_albedo,
        pInSpecularAlbedo: &mut specular_albedo,
        pInNormals: &mut normals,
        pInRoughness: opt_mut(&mut roughness),
        pInColor: &mut color,
        pInAlpha: opt_mut(&mut alpha),
        pInOutput: &mut output,
        pInOutputAlpha: opt_mut(&mut output_alpha),
        pInDepth: &mut depth,
        pInMotionVectors: &mut motion_vectors,
        InJitterOffsetX: p.jitter[0],
        InJitterOffsetY: p.jitter[1],
        InRenderSubrectDimensions: NVSDK_NGX_Dimensions {
            Width: p.render_subrect[0],
            Height: p.render_subrect[1],
        },
        InReset: p.reset as _,
        InMVScaleX: p.motion_vector_scale[0],
        InMVScaleY: p.motion_vector_scale[1],
        pInTransparencyMask: ptr::null_mut(),
        pInExposureTexture: ptr::null_mut(),
        pInBiasCurrentColorMask: ptr::null_mut(),
        InAlphaSubrectBase: ORIGIN,
        InOutputAlphaSubrectBase: ORIGIN,
        InDiffuseAlbedoSubrectBase: ORIGIN,
        InSpecularAlbedoSubrectBase: ORIGIN,
        InNormalsSubrectBase: ORIGIN,
        InRoughnessSubrectBase: ORIGIN,
        InColorSubrectBase: ORIGIN,
        InDepthSubrectBase: ORIGIN,
        InMVSubrectBase: ORIGIN,
        InTranslucencySubrectBase: ORIGIN,
        InBiasCurrentColorSubrectBase: ORIGIN,
        InOutputSubrectBase: ORIGIN,
        InPreExposure: 0.0,
        InExposureScale: 0.0,
        InIndicatorInvertXAxis: 0,
        InIndicatorInvertYAxis: 0,
        InResponsivityMaskSubrectBase: ORIGIN,
        pInReflectedAlbedo: ptr::null_mut(),
        pInColorBeforeParticles: ptr::null_mut(),
        pInColorAfterParticles: ptr::null_mut(),
        pInColorBeforeTransparency: opt_mut(&mut color_before_transparency),
        pInColorAfterTransparency: ptr::null_mut(),
        pInColorBeforeFog: ptr::null_mut(),
        pInColorAfterFog: ptr::null_mut(),
        pInScreenSpaceSubsurfaceScatteringGuide: opt_mut(&mut sss_guide),
        pInColorBeforeScreenSpaceSubsurfaceScattering: ptr::null_mut(),
        pInColorAfterScreenSpaceSubsurfaceScattering: ptr::null_mut(),
        pInScreenSpaceRefractionGuide: ptr::null_mut(),
        pInColorBeforeScreenSpaceRefraction: ptr::null_mut(),
        pInColorAfterScreenSpaceRefraction: ptr::null_mut(),
        pInDepthOfFieldGuide: opt_mut(&mut dof_guide),
        pInColorBeforeDepthOfField: ptr::null_mut(),
        pInColorAfterDepthOfField: ptr::null_mut(),
        pInDiffuseHitDistance: ptr::null_mut(),
        pInSpecularHitDistance: opt_mut(&mut specular_hit_distance),
        pInDiffuseRayDirection: ptr::null_mut(),
        pInSpecularRayDirection: ptr::null_mut(),
        pInDiffuseRayDirectionHitDistance: ptr::null_mut(),
        pInSpecularRayDirectionHitDistance: ptr::null_mut(),
        InReflectedAlbedoSubrectBase: ORIGIN,
        InColorBeforeParticlesSubrectBase: ORIGIN,
        InColorAfterParticlesSubrectBase: ORIGIN,
        InColorBeforeTransparencySubrectBase: ORIGIN,
        InColorAfterTransparencySubrectBase: ORIGIN,
        InColorBeforeFogSubrectBase: ORIGIN,
        InColorAfterFogSubrectBase: ORIGIN,
        InScreenSpaceSubsurfaceScatteringGuideSubrectBase: ORIGIN,
        InColorBeforeScreenSpaceSubsurfaceScatteringSubrectBase: ORIGIN,
        InColorAfterScreenSpaceSubsurfaceScatteringSubrectBase: ORIGIN,
        InScreenSpaceRefractionGuideSubrectBase: ORIGIN,
        InColorBeforeScreenSpaceRefractionSubrectBase: ORIGIN,
        InColorAfterScreenSpaceRefractionSubrectBase: ORIGIN,
        InDepthOfFieldGuideSubrectBase: ORIGIN,
        InColorBeforeDepthOfFieldSubrectBase: ORIGIN,
        InColorAfterDepthOfFieldSubrectBase: ORIGIN,
        InDiffuseHitDistanceSubrectBase: ORIGIN,
        InSpecularHitDistanceSubrectBase: ORIGIN,
        InDiffuseRayDirectionSubrectBase: ORIGIN,
        InSpecularRayDirectionSubrectBase: ORIGIN,
        InDiffuseRayDirectionHitDistanceSubrectBase: ORIGIN,
        InSpecularRayDirectionHitDistanceSubrectBase: ORIGIN,
        pInWorldToViewMatrix: p.world_to_view.cast_mut().cast(),
        pInViewToClipMatrix: p.view_to_clip.cast_mut().cast(),
        GBufferSurface: NVSDK_NGX_VK_GBuffer {
            pInAttrib: [ptr::null_mut(); 17],
        },
        InToneMapperType: NVSDK_NGX_ToneMapperType_NVSDK_NGX_TONEMAPPER_STRING,
        pInMotionVectors3D: ptr::null_mut(),
        pInIsParticleMask: ptr::null_mut(),
        pInAnimatedTextureMask: ptr::null_mut(),
        pInDepthHighRes: ptr::null_mut(),
        pInPositionViewSpace: ptr::null_mut(),
        InFrameTimeDeltaInMsec: 0.0,
        pInRayTracingHitDistance: ptr::null_mut(),
        pInMotionVectorsReflections: opt_mut(&mut specular_motion_vectors),
        pInTransparencyLayer: opt_mut(&mut transparency_layer),
        InTransparencyLayerSubrectBase: ORIGIN,
        pInTransparencyLayerOpacity: opt_mut(&mut transparency_layer_opacity),
        InTransparencyLayerOpacitySubrectBase: ORIGIN,
        pInTransparencyLayerMvecs: ptr::null_mut(),
        InTransparencyLayerMvecsSubrectBase: ORIGIN,
        pInDisocclusionMask: ptr::null_mut(),
        InDisocclusionMaskSubrectBase: ORIGIN,
    };

    status(NGX_VULKAN_EVALUATE_DLSSD_EXT(
        command_buffer,
        feature.cast(),
        sdk.cast(),
        &mut eval_params,
    ))
}

unsafe extern "C" fn release_feature(feature: abi::Feature) -> Status {
    status(NVSDK_NGX_VULKAN_ReleaseFeature(feature.cast()))
}
