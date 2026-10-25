use crate::{
    abi::{self, Extensions},
    types::{DlssError, check},
    plugin::{self, App},
};
use ash::{Entry, vk::PhysicalDevice};
use std::{ffi::CStr, ptr, slice};
use uuid::Uuid;
use wgpu::{
    Adapter, Device, DeviceDescriptor, Instance, InstanceDescriptor, Limits, Queue,
    RequestDeviceError,
    hal::{
        DeviceError, InstanceError,
        api::Vulkan,
        vulkan::{CreateDeviceCallbackArgs, CreateInstanceCallbackArgs},
    },
};

/// Creates a wgpu [`Instance`] with the extensions required for DLSS.
///
/// If the current system does not support a given feature, it will set the corresponding variable in `feature_support` to false.
pub fn create_instance(
    project_id: Uuid,
    instance_descriptor: &InstanceDescriptor,
    feature_support: &mut FeatureSupport,
) -> Result<Instance, InitializationError> {
    unsafe {
        let mut result = Ok(());
        let raw_instance = wgpu::hal::vulkan::Instance::init_with_callback(
            &wgpu::hal::InstanceDescriptor {
                name: "wgpu",
                flags: instance_descriptor.flags,
                memory_budget_thresholds: instance_descriptor.memory_budget_thresholds,
                backend_options: instance_descriptor.backend_options.clone(),
                telemetry: None,
                display: None,
            },
            Some(Box::new(|mut args| {
                result = register_instance_extensions(project_id, &mut args, feature_support);
            })),
        )?;
        result?;

        Ok(Instance::from_hal::<Vulkan>(raw_instance))
    }
}

/// Call this inside of [`wgpu::hal::vulkan::Instance::init_with_callback`] to register wgpu instance extensions
/// necessary for DLSS.
///
/// superfargoth patch: without the plugin, marks everything unsupported and adds nothing.
pub fn register_instance_extensions(
    project_id: Uuid,
    args: &mut CreateInstanceCallbackArgs,
    feature_support: &mut FeatureSupport,
) -> Result<(), RegisterInstanceExtensionsError> {
    if plugin::get().is_none() {
        *feature_support = FeatureSupport::none();
        return Ok(());
    }
    let mut result = Ok(());
    match required_instance_extensions(project_id, abi::FEATURE_SUPER_RESOLUTION, args.entry) {
        Ok((extensions, true)) => args.extensions.extend(extensions),
        Ok((_, false)) => feature_support.super_resolution_supported = false,
        Err(err) => result = Err(err),
    };
    match required_instance_extensions(project_id, abi::FEATURE_RAY_RECONSTRUCTION, args.entry) {
        Ok((extensions, true)) => args.extensions.extend(extensions),
        Ok((_, false)) => feature_support.ray_reconstruction_supported = false,
        Err(err) => result = Err(err),
    };
    result
}

/// Creates a wgpu [`Device`] and [`Queue`] with the extensions required for DLSS.
///
/// If the current system does not support a given feature, it will set the corresponding variable in `feature_support` to false.
///
/// The provided [`Adapter`] must be using the Vulkan backend.
pub fn request_device(
    project_id: Uuid,
    adapter: &Adapter,
    device_descriptor: &DeviceDescriptor,
    feature_support: &mut FeatureSupport,
    limits: Option<Limits>,
) -> Result<(Device, Queue), InitializationError> {
    unsafe {
        let raw_adapter = adapter
            .as_hal::<Vulkan>()
            .ok_or(InitializationError::UnsupportedBackend)?;
        let mut result = Ok(());
        let open_device = raw_adapter.open_with_callback(
            device_descriptor.required_features,
            &limits.unwrap_or(adapter.limits()),
            &device_descriptor.memory_hints,
            Some(Box::new(|mut args| {
                result = register_device_extensions(
                    project_id,
                    &mut args,
                    &raw_adapter,
                    feature_support,
                );
            })),
        )?;
        result?;

        Ok(adapter.create_device_from_hal::<Vulkan>(open_device, device_descriptor)?)
    }
}

/// Call this inside of [`wgpu::hal::vulkan::Adapter::open_with_callback`] to register wgpu device extensions
/// necessary for DLSS.
///
/// superfargoth patch: without the plugin, marks everything unsupported and adds nothing.
pub fn register_device_extensions(
    project_id: Uuid,
    args: &mut CreateDeviceCallbackArgs,
    raw_adapter: &wgpu::hal::vulkan::Adapter,
    feature_support: &mut FeatureSupport,
) -> Result<(), RegisterInstanceExtensionsError> {
    if plugin::get().is_none() {
        *feature_support = FeatureSupport::none();
        return Ok(());
    }
    let raw_instance = raw_adapter.shared_instance().raw_instance();
    let raw_physical_device = raw_adapter.raw_physical_device();
    let mut result = Ok(());

    match required_device_extensions(
        project_id,
        abi::FEATURE_SUPER_RESOLUTION,
        raw_adapter,
        raw_instance.handle(),
        raw_physical_device,
    ) {
        Ok((extensions, true)) => args.extensions.extend(extensions),
        Ok((_, false)) => feature_support.super_resolution_supported = false,
        Err(err) => result = Err(err),
    };

    match required_device_extensions(
        project_id,
        abi::FEATURE_RAY_RECONSTRUCTION,
        raw_adapter,
        raw_instance.handle(),
        raw_physical_device,
    ) {
        Ok((extensions, true)) => args.extensions.extend(extensions),
        Ok((_, false)) => feature_support.ray_reconstruction_supported = false,
        Err(err) => result = Err(err),
    };
    result
}

/// Extension names from the plugin's [`Extensions`].
///
/// # Safety
/// `extensions` must come from a successful plugin call: the array is owned by the vendor runtime
/// and lives for the whole process (upstream relies on the same, hence `'static`).
unsafe fn extension_names(extensions: Extensions) -> impl Iterator<Item = &'static CStr> + Clone {
    let properties: &'static [ash::vk::ExtensionProperties] = if extensions.count == 0 {
        &[]
    } else {
        unsafe { slice::from_raw_parts(extensions.properties, extensions.count as usize) }
    };
    properties
        .iter()
        .map(|extension| unsafe { CStr::from_ptr(extension.extension_name.as_ptr()) })
}

fn required_instance_extensions(
    project_id: Uuid,
    feature: u32,
    entry: &Entry,
) -> Result<(impl Iterator<Item = &'static CStr>, bool), RegisterInstanceExtensionsError> {
    let plugin = crate::types::plugin()?;
    let app = App::new(project_id);
    let info = app.info(plugin);
    let mut extensions = Extensions { count: 0, properties: ptr::null() };
    // SAFETY: valid pointers for the duration of the call; see `extension_names`.
    let required_extensions = unsafe {
        check((plugin.api.instance_extensions)(&info, feature, &mut extensions))?;
        extension_names(extensions)
    };

    // Check that the required extensions are supported
    let supported_extensions = unsafe { entry.enumerate_instance_extension_properties(None)? };
    let extensions_supported = required_extensions.clone().all(|required_extension| {
        supported_extensions
            .iter()
            .any(|extension| extension.extension_name_as_c_str() == Ok(required_extension))
    });

    Ok((required_extensions, extensions_supported))
}

fn required_device_extensions(
    project_id: Uuid,
    feature: u32,
    raw_adapter: &wgpu::hal::vulkan::Adapter,
    raw_instance: ash::vk::Instance,
    raw_physical_device: PhysicalDevice,
) -> Result<(impl Iterator<Item = &'static CStr>, bool), RegisterInstanceExtensionsError> {
    let plugin = crate::types::plugin()?;
    let app = App::new(project_id);
    let info = app.info(plugin);
    let mut extensions = Extensions { count: 0, properties: ptr::null() };
    // SAFETY: valid pointers and live Vulkan handles for the duration of the call; see
    // `extension_names`.
    let required_extensions = unsafe {
        check((plugin.api.device_extensions)(
            &info,
            feature,
            raw_instance,
            raw_physical_device,
            &mut extensions,
        ))?;
        extension_names(extensions)
    };

    // Check that the required extensions are supported
    let extensions_supported = required_extensions.clone().all(|required_extension| {
        raw_adapter
            .physical_device_capabilities()
            .supports_extension(required_extension)
    });

    Ok((required_extensions, extensions_supported))
}

/// Which DLSS features are supported on the current system.
pub struct FeatureSupport {
    /// DLSS Super Resolution (DLSS) is supported.
    pub super_resolution_supported: bool,
    /// DLSS Ray Reconstruction (DLSS-RR) is supported.
    pub ray_reconstruction_supported: bool,
}

impl FeatureSupport {
    /// superfargoth patch: nothing supported (no plugin).
    fn none() -> Self {
        Self {
            super_resolution_supported: false,
            ray_reconstruction_supported: false,
        }
    }
}

impl Default for FeatureSupport {
    fn default() -> Self {
        Self {
            super_resolution_supported: true,
            ray_reconstruction_supported: true,
        }
    }
}

/// Error returned by [`request_device`].
#[derive(thiserror::Error, Debug)]
pub enum InitializationError {
    #[error(transparent)]
    InstanceError(#[from] InstanceError),
    #[error(transparent)]
    RequestDeviceError(#[from] RequestDeviceError),
    #[error(transparent)]
    DeviceError(#[from] DeviceError),
    #[error(transparent)]
    VulkanError(#[from] ash::vk::Result),
    #[error(transparent)]
    DlssError(#[from] DlssError),
    #[error("Provided adapter is not using the Vulkan backend")]
    UnsupportedBackend,
}

/// Error returned by [`register_instance_extensions`].
#[derive(thiserror::Error, Debug)]
pub enum RegisterInstanceExtensionsError {
    #[error(transparent)]
    VulkanError(#[from] ash::vk::Result),
    #[error(transparent)]
    DlssError(#[from] DlssError),
}

impl From<RegisterInstanceExtensionsError> for InitializationError {
    fn from(value: RegisterInstanceExtensionsError) -> Self {
        match value {
            RegisterInstanceExtensionsError::VulkanError(err) => {
                InitializationError::VulkanError(err)
            }
            RegisterInstanceExtensionsError::DlssError(dlss_error) => {
                InitializationError::DlssError(dlss_error)
            }
        }
    }
}
