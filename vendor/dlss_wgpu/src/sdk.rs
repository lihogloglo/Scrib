use crate::{
    abi::{self, InitParams},
    types::{DlssError, check, plugin},
    plugin::{App, Plugin},
};
use std::{
    ptr,
    sync::{Arc, Mutex},
};
use uuid::Uuid;
use wgpu::{Device, hal::api::Vulkan};

/// Application-wide DLSS object.
///
/// superfargoth patch: holds the plugin's opaque SDK context instead of `NVSDK_NGX_Parameter`.
pub struct DlssSdk {
    pub(crate) sdk: abi::Sdk,
    pub(crate) plugin: &'static Plugin,
    pub(crate) device: Device,
}

impl DlssSdk {
    /// Creates the DLSS SDK.
    ///
    /// This should be done once per application.
    pub fn new(project_id: Uuid, device: Device) -> Result<Arc<Mutex<Self>>, DlssError> {
        let plugin = plugin()?;
        let app = App::new(project_id);
        let info = app.info(plugin);

        let mut sdk = ptr::null_mut();
        // SAFETY: as upstream: raw Vulkan handles from wgpu's Vulkan device, valid while `device`
        // lives (we keep it); the plugin initializes NGX on them (and starts NGX's update check).
        unsafe {
            let hal_device = device
                .as_hal::<Vulkan>()
                .ok_or(DlssError::PlatformError)?;
            let shared_instance = hal_device.shared_instance();
            let raw_instance = shared_instance.raw_instance();
            let params = InitParams {
                instance: raw_instance.handle(),
                physical_device: hal_device.raw_physical_device(),
                device: hal_device.raw_device().handle(),
                get_instance_proc_addr: shared_instance.entry().static_fn().get_instance_proc_addr,
                get_device_proc_addr: raw_instance.fp_v1_0().get_device_proc_addr,
            };
            check((plugin.api.init)(&info, &params, &mut sdk))?;
        }

        Ok(Arc::new(Mutex::new(Self { sdk, plugin, device })))
    }

    /// Returns the number of bytes of VRAM allocated by DLSS.
    pub fn get_vram_allocated_bytes(&mut self) -> Result<u64, DlssError> {
        let mut vram_allocated_bytes = 0;
        // SAFETY: `self.sdk` is a live context from `init`.
        check(unsafe { (self.plugin.api.vram_allocated)(self.sdk, &mut vram_allocated_bytes) })?;
        Ok(vram_allocated_bytes)
    }
}

impl Drop for DlssSdk {
    fn drop(&mut self) {
        // SAFETY: as upstream: wait for idle, then destroy the context on the same device.
        unsafe {
            let hal_device = self.device.as_hal::<Vulkan>().unwrap();
            hal_device
                .raw_device()
                .device_wait_idle()
                .expect("Failed to wait for idle device when destroying DlssSdk");

            check((self.plugin.api.shutdown)(self.sdk, hal_device.raw_device().handle()))
                .expect("Failed to destroy DlssSdk");
        }
    }
}

// SAFETY: as upstream (the NGX context is only used under the `Mutex` callers wrap it in).
unsafe impl Send for DlssSdk {}
unsafe impl Sync for DlssSdk {}
